
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency: safe, portable message channels.
//!
//! File: `src/classical/concurrency/channels.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97, as declared by the repository.
//!
//! # File contract
//!
//! **Purpose:** Provide a safe local channel realization with bounded or
//! unbounded buffering, cloneable senders and receivers, blocking and
//! non-blocking operations, explicit closure, and structured failures.
//!
//! **Authority:** `grammar/spec/concurrency.md` and
//! `grammar/concurrency/channels.g4` define the language-level requirements.
//!
//! **Owns:** Local channel configuration, queue state, endpoint lifetimes,
//! send/receive behavior, closure behavior, and local channel errors.
//!
//! **Does not own:** Source syntax, AST, payload type checking, compiler IR,
//! task identity, task scheduling, async executors, distributed transports,
//! network protocols, hardware capabilities, or resource allocation policy.
//!
//! **Inputs:** A channel configuration and values of the channel's Rust type.
//!
//! **Outputs:** Send/receive outcomes and structured errors.
//!
//! **Dependencies:** Rust standard library only.
//!
//! **Upstream integration:** Semantic analysis must establish that the source
//! channel's payload type, direction, ownership, effects, and operation are
//! valid before lowering to an implementation.
//!
//! **Downstream integration:** A local runtime may use this implementation.
//! Distributed and heterogeneous runtimes must provide suitable adapters when
//! local shared memory is not available.
//!
//! **AST/IR contract:** This file defines neither an AST nor an IR. Source
//! channel operations must flow through Zamani's existing frontend and
//! canonical semantic/IR architecture.
//!
//! **Resource contract:** Bounded channels have caller-selected, nonzero
//! capacity. Unbounded channels grow subject to actual memory availability.
//! Allocation failure and host resource exhaustion cannot be turned into a
//! universal guarantee of infinite storage.
//!
//! **Concurrency contract:** Sending and receiving are coordinated by a
//! mutex and condition variables. This is a safe local implementation; it
//! does not promise lock-free behavior, fairness, or distributed semantics.
//!
//! **Cancellation contract:** Blocking calls do not forcibly observe
//! `task::CancellationToken`. Cancellation-aware waits require an executor or
//! runtime adapter with an explicit wake-up/cancellation protocol.
//!
//! **Freeze criteria:** API, close behavior, error behavior, endpoint lifetime,
//! and bounded/unbounded semantics must pass the tests and repository checks
//! described after this file.

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

/// Buffering policy for a local channel.
///
/// The policy is explicit rather than tied to a particular machine, transport,
/// or execution device. Unbounded buffering can exhaust available memory;
/// bounded buffering applies backpressure to blocking senders.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChannelConfig {
    /// Permit queue growth subject to available memory.
    Unbounded,

    /// Limit the number of queued messages.
    ///
    /// The capacity must be nonzero. This counts queued messages, not messages
    /// currently held by receivers.
    Bounded(NonZeroUsize),
}

impl ChannelConfig {
    /// Create an unbounded-buffer configuration.
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Create a bounded-buffer configuration with a nonzero capacity.
    pub const fn bounded(capacity: NonZeroUsize) -> Self {
        Self::Bounded(capacity)
    }

    /// Create a bounded-buffer configuration from a host-sized integer.
    ///
    /// A zero capacity is rejected. Rendezvous channels, where a send directly
    /// hands a value to a waiting receiver without queue capacity, are not
    /// implemented by this local queue implementation.
    pub fn bounded_capacity(
        capacity: usize,
    ) -> Result<Self, ChannelConfigError> {
        NonZeroUsize::new(capacity)
            .map(Self::Bounded)
            .ok_or(ChannelConfigError::ZeroCapacity)
    }

    /// Return the queue capacity, or `None` for unbounded buffering.
    pub const fn capacity(self) -> Option<NonZeroUsize> {
        match self {
            Self::Unbounded => None,
            Self::Bounded(capacity) => Some(capacity),
        }
    }
}

impl Default for ChannelConfig {
    /// The default is explicitly unbounded for ergonomic local construction.
    ///
    /// Production code handling potentially unbounded input should choose a
    /// bounded configuration when backpressure is required.
    fn default() -> Self {
        Self::Unbounded
    }
}

/// Errors raised while configuring a channel.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChannelConfigError {
    /// A bounded queue must have positive capacity.
    ZeroCapacity,
}

impl fmt::Display for ChannelConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapacity => {
                f.write_str("bounded channel capacity must be greater than zero")
            }
        }
    }
}

impl Error for ChannelConfigError {}

/// Error returned when a blocking send cannot deliver its value.
///
/// The original value is preserved so a caller may recover it, retry through
/// another channel, or apply an application-specific failure policy.
#[derive(Debug, Eq, PartialEq)]
pub enum SendError<T> {
    /// The channel has been explicitly closed.
    Closed(T),

    /// No receiver endpoints remain.
    NoReceivers(T),

    /// A lock was poisoned before the value could be enqueued.
    LockPoisoned(T),
}

impl<T> SendError<T> {
    /// Recover the unsent value.
    pub fn into_inner(self) -> T {
        match self {
            Self::Closed(value)
            | Self::NoReceivers(value)
            | Self::LockPoisoned(value) => value,
        }
    }

    /// Return a stable error classification.
    pub const fn kind(&self) -> SendErrorKind {
        match self {
            Self::Closed(_) => SendErrorKind::Closed,
            Self::NoReceivers(_) => SendErrorKind::NoReceivers,
            Self::LockPoisoned(_) => SendErrorKind::LockPoisoned,
        }
    }
}

/// Classification of a failed blocking send.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SendErrorKind {
    Closed,
    NoReceivers,
    LockPoisoned,
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed(_) => f.write_str("channel is closed"),
            Self::NoReceivers(_) => {
                f.write_str("channel has no remaining receivers")
            }
            Self::LockPoisoned(_) => {
                f.write_str("channel state lock is poisoned")
            }
        }
    }
}

impl<T: fmt::Debug> Error for SendError<T> {}

/// Error returned by a non-blocking send.
#[derive(Debug, Eq, PartialEq)]
pub enum TrySendError<T> {
    /// The bounded queue is currently full.
    Full(T),

    /// The channel has been explicitly closed.
    Closed(T),

    /// No receiver endpoints remain.
    NoReceivers(T),

    /// A lock was poisoned before the value could be enqueued.
    LockPoisoned(T),
}

impl<T> TrySendError<T> {
    /// Recover the unsent value.
    pub fn into_inner(self) -> T {
        match self {
            Self::Full(value)
            | Self::Closed(value)
            | Self::NoReceivers(value)
            | Self::LockPoisoned(value) => value,
        }
    }

    /// Return a stable error classification.
    pub const fn kind(&self) -> TrySendErrorKind {
        match self {
            Self::Full(_) => TrySendErrorKind::Full,
            Self::Closed(_) => TrySendErrorKind::Closed,
            Self::NoReceivers(_) => TrySendErrorKind::NoReceivers,
            Self::LockPoisoned(_) => TrySendErrorKind::LockPoisoned,
        }
    }
}

/// Classification of a failed non-blocking send.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TrySendErrorKind {
    Full,
    Closed,
    NoReceivers,
    LockPoisoned,
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full(_) => f.write_str("channel queue is full"),
            Self::Closed(_) => f.write_str("channel is closed"),
            Self::NoReceivers(_) => {
                f.write_str("channel has no remaining receivers")
            }
            Self::LockPoisoned(_) => {
                f.write_str("channel state lock is poisoned")
            }
        }
    }
}

impl<T: fmt::Debug> Error for TrySendError<T> {}

/// Outcome of a non-blocking receive when no value was returned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryRecvError {
    /// No message is available at this moment, but the channel may receive
    /// future messages.
    Empty,

    /// The channel is closed and its queue has been drained.
    Closed,

    /// The channel state lock was poisoned.
    LockPoisoned,
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("channel has no message available"),
            Self::Closed => f.write_str("channel is closed and drained"),
            Self::LockPoisoned => {
                f.write_str("channel state lock is poisoned")
            }
        }
    }
}

impl Error for TryRecvError {}

/// Error returned by a blocking receive.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RecvError {
    /// The channel is closed and all queued messages have been consumed.
    Closed,

    /// The channel state lock was poisoned.
    LockPoisoned,
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed => f.write_str("channel is closed and drained"),
            Self::LockPoisoned => {
                f.write_str("channel state lock is poisoned")
            }
        }
    }
}

impl Error for RecvError {}

/// Internal state shared by all endpoints.
///
/// All queue and lifecycle state is protected by `Mutex`. The condition
/// variables coordinate waiters; they do not own channel semantics.
struct ChannelState<T> {
    queue: VecDeque<T>,
    closed: bool,
    senders: usize,
    receivers: usize,
}

struct Shared<T> {
    config: ChannelConfig,
    state: Mutex<ChannelState<T>>,
    message_available: Condvar,
    queue_space_available: Condvar,
}

impl<T> Shared<T> {
    fn lock(&self) -> Result<MutexGuard<'_, ChannelState<T>>, ()> {
        self.state.lock().map_err(|_| ())
    }

    fn queue_is_full(&self, state: &ChannelState<T>) -> bool {
        match self.config.capacity() {
            Some(capacity) => state.queue.len() >= capacity.get(),
            None => false,
        }
    }

    /// Close the channel and wake all blocked senders and receivers.
    ///
    /// Recovering the poisoned mutex here is intentional: closure is a
    /// best-effort terminal operation and must still wake waiters. Ordinary
    /// send/receive operations continue to report poisoned state.
    fn close(&self) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };

        state.closed = true;
        drop(state);

        self.message_available.notify_all();
        self.queue_space_available.notify_all();
    }

    fn decrement_senders(&self) {
        let should_close = {
            let mut state = match self.state.lock() {
                Ok(state) => state,
                Err(poisoned) => poisoned.into_inner(),
            };

            state.senders = state.senders.saturating_sub(1);
            state.senders == 0
        };

        if should_close {
            self.close();
        }
    }

    fn decrement_receivers(&self) {
        let should_close = {
            let mut state = match self.state.lock() {
                Ok(state) => state,
                Err(poisoned) => poisoned.into_inner(),
            };

            state.receivers = state.receivers.saturating_sub(1);
            state.receivers == 0
        };

        if should_close {
            self.close();
        }
    }
}

/// Create a local channel with the requested buffering policy.
///
/// The returned endpoints share one logical channel. Cloning either endpoint
/// creates another endpoint of the same direction.
///
/// Messages are delivered in queue insertion order. When explicitly closed,
/// or when all senders have been dropped, receivers may drain queued messages
/// before receiving `RecvError::Closed`.
///
/// # Resource behavior
///
/// The constructor does not preallocate the full bounded capacity. An
/// unbounded channel can grow until the process or allocator runs out of
/// resources. Select bounded buffering where backpressure is required.
///
/// # Example
///
/// ```
/// use std::num::NonZeroUsize;
/// use zamani_compiler::classical::concurrency::channels::{
///     channel, ChannelConfig,
/// };
///
/// let (sender, receiver) = channel::<String>(
///     ChannelConfig::bounded(NonZeroUsize::new(8).expect("nonzero")),
/// );
///
/// sender.send("hello".to_owned()).expect("send");
/// assert_eq!(receiver.recv().expect("receive"), "hello");
/// ```
pub fn channel<T>(config: ChannelConfig) -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        config,
        state: Mutex::new(ChannelState {
            queue: VecDeque::new(),
            closed: false,
            senders: 1,
            receivers: 1,
        }),
        message_available: Condvar::new(),
        queue_space_available: Condvar::new(),
    });

    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver { shared },
    )
}

/// Cloneable sending endpoint.
///
/// A blocking send waits while a bounded queue is full. It returns the original
/// value if the channel closes, all receivers disappear, or the state lock is
/// poisoned.
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    /// Send one value, waiting for space when the queue is bounded and full.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut value = Some(value);

        let mut state = match self.shared.lock() {
            Ok(state) => state,
            Err(()) => {
                return Err(SendError::LockPoisoned(
                    value.expect("value is retained until enqueue"),
                ));
            }
        };

        loop {
            if state.closed {
                return Err(SendError::Closed(
                    value.expect("value is retained until enqueue"),
                ));
            }

            if state.receivers == 0 {
                return Err(SendError::NoReceivers(
                    value.expect("value is retained until enqueue"),
                ));
            }

            if !self.shared.queue_is_full(&state) {
                state.queue.push_back(
                    value.take().expect("value is enqueued only once"),
                );

                drop(state);
                self.shared.message_available.notify_one();
                return Ok(());
            }

            state = match self.shared.queue_space_available.wait(state) {
                Ok(state) => state,
                Err(_) => {
                    return Err(SendError::LockPoisoned(
                        value.expect("value is retained while waiting"),
                    ));
                }
            };
        }
    }

    /// Attempt to send immediately without waiting for queue space.
    pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
        let mut state = match self.shared.lock() {
            Ok(state) => state,
            Err(()) => return Err(TrySendError::LockPoisoned(value)),
        };

        if state.closed {
            return Err(TrySendError::Closed(value));
        }

        if state.receivers == 0 {
            return Err(TrySendError::NoReceivers(value));
        }

        if self.shared.queue_is_full(&state) {
            return Err(TrySendError::Full(value));
        }

        state.queue.push_back(value);
        drop(state);

        self.shared.message_available.notify_one();
        Ok(())
    }

    /// Close the entire logical channel and wake blocked operations.
    ///
    /// Queued messages remain available for receivers to drain.
    pub fn close(&self) {
        self.shared.close();
    }

    /// Report whether the channel has been closed.
    ///
    /// This is an observation, not a reservation: another endpoint may close
    /// the channel immediately after this method returns.
    pub fn is_closed(&self) -> bool {
        let state = match self.shared.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.closed
    }

    /// Return the configured queue capacity, or `None` if unbounded.
    pub const fn capacity(&self) -> Option<NonZeroUsize> {
        self.shared.config.capacity()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        {
            let mut state = match self.shared.state.lock() {
                Ok(state) => state,
                Err(poisoned) => poisoned.into_inner(),
            };
            state.senders = state.senders.saturating_add(1);
        }

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.decrement_senders();
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender")
            .field("capacity", &self.shared.config.capacity())
            .field("closed", &self.is_closed())
            .finish()
    }
}

/// Cloneable receiving endpoint.
///
/// Multiple receivers compete for messages from the same FIFO queue. Each
/// message is removed by at most one successful receiver.
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    /// Receive one value, waiting while the queue is empty and open.
    ///
    /// Explicit closure or dropping the final sender causes this method to
    /// return `RecvError::Closed` once all queued messages have been drained.
    pub fn recv(&self) -> Result<T, RecvError> {
        let mut state = self.shared.lock().map_err(|_| ())?;

        loop {
            if let Some(value) = state.queue.pop_front() {
                drop(state);
                self.shared.queue_space_available.notify_one();
                return Ok(value);
            }

            if state.closed || state.senders == 0 {
                return Err(RecvError::Closed);
            }

            state = match self.shared.message_available.wait(state) {
                Ok(state) => state,
                Err(_) => return Err(RecvError::LockPoisoned),
            };
        }
    }

    /// Attempt to receive immediately without waiting.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        let mut state = self
            .shared
            .lock()
            .map_err(|_| TryRecvError::LockPoisoned)?;

        if let Some(value) = state.queue.pop_front() {
            drop(state);
            self.shared.queue_space_available.notify_one();
            return Ok(value);
        }

        if state.closed || state.senders == 0 {
            Err(TryRecvError::Closed)
        } else {
            Err(TryRecvError::Empty)
        }
    }

    /// Close the entire logical channel and wake blocked operations.
    ///
    /// Queued messages remain available for receivers to drain.
    pub fn close(&self) {
        self.shared.close();
    }

    /// Report whether the channel has been closed.
    pub fn is_closed(&self) -> bool {
        let state = match self.shared.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.closed
    }

    /// Return the configured queue capacity, or `None` if unbounded.
    pub const fn capacity(&self) -> Option<NonZeroUsize> {
        self.shared.config.capacity()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        {
            let mut state = match self.shared.state.lock() {
                Ok(state) => state,
                Err(poisoned) => poisoned.into_inner(),
            };
            state.receivers = state.receivers.saturating_add(1);
        }

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.decrement_receivers();
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Receiver")
            .field("capacity", &self.shared.config.capacity())
            .field("closed", &self.is_closed())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    fn bounded(capacity: usize) -> ChannelConfig {
        ChannelConfig::bounded_capacity(capacity).expect("positive capacity")
    }

    #[test]
    fn bounded_capacity_rejects_zero() {
        assert_eq!(
            ChannelConfig::bounded_capacity(0),
            Err(ChannelConfigError::ZeroCapacity)
        );
    }

    #[test]
    fn bounded_channel_sends_and_receives_fifo() {
        let (sender, receiver) = channel::<i32>(bounded(3));

        sender.send(10).expect("send first");
        sender.send(20).expect("send second");
        sender.send(30).expect("send third");

        assert_eq!(receiver.recv(), Ok(10));
        assert_eq!(receiver.recv(), Ok(20));
        assert_eq!(receiver.recv(), Ok(30));
    }

    #[test]
    fn nonblocking_send_reports_full_and_preserves_value() {
        let (sender, receiver) = channel::<String>(bounded(1));

        sender.send("first".to_owned()).expect("first send");

        let error = sender
            .try_send("second".to_owned())
            .expect_err("bounded queue is full");

        assert_eq!(error.kind(), TrySendErrorKind::Full);
        assert_eq!(error.into_inner(), "second");
        assert_eq!(receiver.recv().as_deref(), Ok("first"));
    }

    #[test]
    fn unbounded_channel_accepts_multiple_messages() {
        let (sender, receiver) = channel::<usize>(ChannelConfig::unbounded());

        for value in 0..512 {
            sender.send(value).expect("unbounded send");
        }

        for expected in 0..512 {
            assert_eq!(receiver.recv(), Ok(expected));
        }
    }

    #[test]
    fn closing_channel_drains_messages_before_closed() {
        let (sender, receiver) = channel::<u8>(bounded(4));

        sender.send(7).expect("send");
        sender.close();

        assert!(sender.is_closed());
        assert_eq!(receiver.recv(), Ok(7));
        assert_eq!(receiver.recv(), Err(RecvError::Closed));
    }

    #[test]
    fn dropping_last_sender_closes_receiver_after_drain() {
        let (sender, receiver) = channel::<u8>(ChannelConfig::unbounded());

        sender.send(1).expect("send");
        drop(sender);

        assert_eq!(receiver.recv(), Ok(1));
        assert_eq!(receiver.recv(), Err(RecvError::Closed));
    }

    #[test]
    fn dropping_last_receiver_makes_send_fail_with_value() {
        let (sender, receiver) = channel::<String>(ChannelConfig::unbounded());
        drop(receiver);

        let error = sender
            .send("preserved".to_owned())
            .expect_err("there are no receivers");

        assert_eq!(error.kind(), SendErrorKind::NoReceivers);
        assert_eq!(error.into_inner(), "preserved");
    }

    #[test]
    fn cloned_senders_share_the_same_channel() {
        let (sender, receiver) = channel::<usize>(ChannelConfig::unbounded());
        let second_sender = sender.clone();

        sender.send(3).expect("first sender");
        second_sender.send(4).expect("second sender");

        assert_eq!(receiver.recv(), Ok(3));
        assert_eq!(receiver.recv(), Ok(4));
    }

    #[test]
    fn cloned_receivers_compete_for_messages() {
        let (sender, receiver_a) = channel::<usize>(ChannelConfig::unbounded());
        let receiver_b = receiver_a.clone();

        sender.send(11).expect("send");
        sender.send(12).expect("send");

        let values = [
            receiver_a.recv().expect("first receive"),
            receiver_b.recv().expect("second receive"),
        ];

        assert!(values.contains(&11));
        assert!(values.contains(&12));
        assert_ne!(values[0], values[1]);
    }

    #[test]
    fn bounded_send_waits_until_receiver_frees_space() {
        let (sender, receiver) = channel::<usize>(bounded(1));

        sender.send(1).expect("initial send");

        let worker = thread::spawn(move || {
            sender.send(2).expect("blocked send resumes");
        });

        assert_eq!(receiver.recv(), Ok(1));
        worker.join().expect("sender thread");
        assert_eq!(receiver.recv(), Ok(2));
    }

    #[test]
    fn blocked_receiver_wakes_when_sender_is_dropped() {
        let (sender, receiver) = channel::<usize>(ChannelConfig::unbounded());

        let worker = thread::spawn(move || receiver.recv());
        drop(sender);

        assert_eq!(
            worker.join().expect("receiver thread"),
            Err(RecvError::Closed)
        );
    }

    #[test]
    fn explicit_close_wakes_blocked_receiver() {
        let (sender, receiver) = channel::<usize>(ChannelConfig::unbounded());
        let closer = sender.clone();

        let worker = thread::spawn(move || receiver.recv());
        closer.close();

        assert_eq!(
            worker.join().expect("receiver thread"),
            Err(RecvError::Closed)
        );
    }

    #[test]
    fn try_receive_distinguishes_empty_from_closed() {
        let (sender, receiver) = channel::<usize>(ChannelConfig::unbounded());

        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
        sender.close();
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
    }

    #[test]
    fn multiple_producers_deliver_all_messages() {
        let (sender, receiver) = channel::<usize>(bounded(8));
        let worker_count = 4usize;
        let messages_per_worker = 100usize;
        let mut workers = Vec::new();

        for worker_id in 0..worker_count {
            let worker_sender = sender.clone();

            workers.push(thread::spawn(move || {
                for sequence in 0..messages_per_worker {
                    worker_sender
                        .send(worker_id * messages_per_worker + sequence)
                        .expect("send from worker");
                }
            }));
        }

        drop(sender);

        let mut received = Vec::new();
        while let Ok(value) = receiver.recv() {
            received.push(value);
        }

        for worker in workers {
            worker.join().expect("producer thread");
        }

        received.sort_unstable();

        let expected_count = worker_count * messages_per_worker;
        assert_eq!(received.len(), expected_count);
        assert_eq!(
            received,
            (0..expected_count).collect::<Vec<_>>()
        );
    }

    #[test]
    fn endpoint_lifetimes_are_shared_without_global_limits() {
        let (sender, receiver) = channel::<Arc<str>>(ChannelConfig::unbounded());
        let payload: Arc<str> = Arc::from("portable");

        sender.send(Arc::clone(&payload)).expect("send");
        let received = receiver.recv().expect("receive");

        assert!(Arc::ptr_eq(&payload, &received));
    }
}
