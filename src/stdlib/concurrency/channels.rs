
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Concurrency Channels
//!
//! Path: `src/stdlib/concurrency/channels.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides typed, target-independent, in-process message channels.
//!
//! The implementation supports:
//!
//! - Multiple cloned senders.
//! - Multiple cloned receivers.
//! - FIFO message ordering.
//! - Bounded channels with backpressure.
//! - Unbounded channels.
//! - Blocking send and receive.
//! - Non-blocking send and receive.
//! - Timed send and receive.
//! - Explicit channel closure.
//! - Sender and receiver disconnection.
//! - Graceful draining of messages after closure.
//!
//! ## OWNERSHIP
//!
//! This module owns local channel communication semantics.
//!
//! It does not own:
//!
//! - task scheduling;
//! - worker creation;
//! - runtime execution;
//! - asynchronous executors;
//! - distributed transport;
//! - network discovery;
//! - hardware allocation;
//! - physical communication links;
//! - compiler IR;
//! - language parsing;
//! - semantic type checking;
//! - cancellation-token implementation;
//! - channel-select scheduling.
//!
//! Those responsibilities belong to their existing repository authorities.
//!
//! ## POCO-REAF
//!
//! This implementation does not impose universal limits on:
//!
//! - channels;
//! - senders;
//! - receivers;
//! - messages;
//! - payload sizes;
//! - queue capacity;
//! - communication relationships.
//!
//! Bounded capacity is an explicit property of a particular channel
//! configuration. It is not a language-level or hardware-level ceiling.
//!
//! Unbounded channels grow subject to available memory and platform
//! representational limits. They do not promise infinite physical memory.
//!
//! ## ORDERING
//!
//! Messages are inserted and removed in FIFO order.
//!
//! With multiple receivers, each message is delivered to one receiver.
//! This is competing-consumer behavior, not broadcast.
//!
//! The implementation does not promise fairness between waiting senders
//! or receivers. Fairness policy belongs to a higher-level contract.
//!
//! ## CLOSURE
//!
//! Explicit closure prevents new sends.
//!
//! Already queued messages remain available to receivers.
//!
//! Once the queue is drained, receive reports disconnection.
//!
//! Dropping the final sender also causes receivers to observe
//! disconnection after queued messages have been drained.
//!
//! Dropping the final receiver causes senders to fail.
//!
//! ## ASYNCHRONOUS AND DISTRIBUTED INTEGRATION
//!
//! This file provides blocking and non-blocking local operations.
//!
//! Async adapters belong in the authoritative futures/asynchronous
//! subsystem and must not create another executor here.
//!
//! Distributed implementations belong behind the repository's
//! networking/distributed/runtime provider contracts.
//!
//! ## ERRORS
//!
//! Receive operations use `crate::stdlib::error::Result<T>`.
//!
//! Send errors retain ownership of the unsent message so callers can
//! recover or retry it without losing the payload.

use std::collections::VecDeque;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Channel buffering configuration.
///
/// A bounded channel applies backpressure when its queue reaches capacity.
/// An unbounded channel grows subject to available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCapacity {
    /// The queue has no configured capacity limit.
    Unbounded,

    /// The queue can hold up to the configured number of messages.
    Bounded(NonZeroUsize),
}

impl ChannelCapacity {
    /// Creates a bounded capacity from a positive value.
    ///
    /// # Errors
    ///
    /// Returns an invalid-input error when `capacity` is zero.
    pub fn bounded(capacity: usize) -> Result<Self> {
        NonZeroUsize::new(capacity)
            .map(Self::Bounded)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidInput,
                    "bounded channel capacity must be greater than zero",
                )
                .in_component("stdlib.concurrency.channels")
                .during_operation("channel.configure")
            })
    }

    /// Returns the configured capacity.
    ///
    /// `None` means that the channel is unbounded.
    #[must_use]
    pub const fn limit(self) -> Option<NonZeroUsize> {
        match self {
            Self::Unbounded => None,
            Self::Bounded(capacity) => Some(capacity),
        }
    }

    /// Returns whether this configuration is bounded.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::Bounded(_))
    }
}

/// Configuration for creating a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelConfig {
    capacity: ChannelCapacity,
}

impl ChannelConfig {
    /// Creates an unbounded channel configuration.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            capacity: ChannelCapacity::Unbounded,
        }
    }

    /// Creates a bounded channel configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if `capacity` is zero.
    pub fn bounded(capacity: usize) -> Result<Self> {
        Ok(Self {
            capacity: ChannelCapacity::bounded(capacity)?,
        })
    }

    /// Returns the channel's buffering configuration.
    #[must_use]
    pub const fn capacity(self) -> ChannelCapacity {
        self.capacity
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self::unbounded()
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Mutable state shared by all endpoints of a channel.
struct ChannelState<T> {
    /// Messages awaiting reception.
    queue: VecDeque<T>,

    /// Whether explicit closure has been requested.
    closed: bool,

    /// Whether at least one sender endpoint remains alive.
    sender_alive: bool,

    /// Whether at least one receiver endpoint remains alive.
    receiver_alive: bool,
}

/// Shared channel storage and synchronization.
struct ChannelShared<T> {
    /// Channel state.
    state: Mutex<ChannelState<T>>,

    /// Wakes receivers after messages are sent or the channel closes.
    readable: Condvar,

    /// Wakes senders after messages are received or the channel closes.
    writable: Condvar,

    /// Buffering configuration.
    capacity: ChannelCapacity,
}

impl<T> ChannelShared<T> {
    /// Locks the channel state.
    ///
    /// Poison recovery is safe here because the implementation does not
    /// execute user callbacks while holding the state lock.
    fn lock(&self) -> MutexGuard<'_, ChannelState<T>> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Returns whether the queue has reached its configured capacity.
    fn is_full(&self, state: &ChannelState<T>) -> bool {
        match self.capacity {
            ChannelCapacity::Unbounded => false,
            ChannelCapacity::Bounded(capacity) => {
                state.queue.len() >= capacity.get()
            }
        }
    }

    /// Marks the channel closed and wakes all blocked operations.
    fn close(&self) {
        let mut state = self.lock();
        state.closed = true;
        drop(state);

        self.readable.notify_all();
        self.writable.notify_all();
    }
}

/// Shared lifetime token for sender clones.
///
/// All clones share this token. The final sender clone dropping the token
/// marks the sender side disconnected.
struct SenderLease<T> {
    shared: Arc<ChannelShared<T>>,
}

impl<T> Drop for SenderLease<T> {
    fn drop(&mut self) {
        let mut state = self.shared.lock();
        state.sender_alive = false;
        drop(state);

        self.shared.readable.notify_all();
        self.shared.writable.notify_all();
    }
}

/// Shared lifetime token for receiver clones.
///
/// The final receiver clone dropping the token marks the receiver side
/// disconnected.
struct ReceiverLease<T> {
    shared: Arc<ChannelShared<T>>,
}

impl<T> Drop for ReceiverLease<T> {
    fn drop(&mut self) {
        let mut state = self.shared.lock();
        state.receiver_alive = false;
        drop(state);

        self.shared.readable.notify_all();
        self.shared.writable.notify_all();
    }
}

// ============================================================================
// ENDPOINTS
// ============================================================================

/// Sending endpoint of a typed channel.
///
/// Cloning a sender creates another handle to the same sending side.
///
/// Messages sent through all sender clones enter the same FIFO queue.
pub struct Sender<T> {
    lease: Arc<SenderLease<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            lease: Arc::clone(&self.lease),
        }
    }
}

/// Receiving endpoint of a typed channel.
///
/// Cloning a receiver creates another competing consumer of the same queue.
///
/// Each message is received by one receiver, not broadcast to every receiver.
pub struct Receiver<T> {
    lease: Arc<ReceiverLease<T>>,
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            lease: Arc::clone(&self.lease),
        }
    }
}

// ============================================================================
// SEND FAILURE TYPES
// ============================================================================

/// Reason a send operation could not deliver its message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendFailure {
    /// The channel has been explicitly closed.
    Closed,

    /// No receiving endpoint remains.
    Disconnected,
}

impl fmt::Display for SendFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed => formatter.write_str("channel is closed"),
            Self::Disconnected => {
                formatter.write_str("channel has no receiving endpoint")
            }
        }
    }
}

/// Error returned by a blocking send operation.
///
/// The original message is retained and can be recovered.
pub struct SendError<T> {
    value: T,
    reason: SendFailure,
}

impl<T> SendError<T> {
    /// Returns the reason the message was not delivered.
    #[must_use]
    pub const fn reason(&self) -> SendFailure {
        self.reason
    }

    /// Recovers the message that was not delivered.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: fmt::Debug> fmt::Debug for SendError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SendError")
            .field("reason", &self.reason)
            .field("value", &self.value)
            .finish()
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reason.fmt(formatter)
    }
}

impl<T: fmt::Debug> std::error::Error for SendError<T> {}

/// Error returned by a non-blocking send operation.
#[derive(Debug)]
pub enum TrySendError<T> {
    /// The bounded channel has no available queue capacity.
    Full(T),

    /// The channel has been explicitly closed.
    Closed(T),

    /// No receiving endpoint remains.
    Disconnected(T),
}

impl<T> TrySendError<T> {
    /// Recovers the message that was not delivered.
    #[must_use]
    pub fn into_inner(self) -> T {
        match self {
            Self::Full(value)
            | Self::Closed(value)
            | Self::Disconnected(value) => value,
        }
    }

    /// Returns whether the channel was full.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        matches!(self, Self::Full(_))
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full(_) => formatter.write_str("channel is full"),
            Self::Closed(_) => formatter.write_str("channel is closed"),
            Self::Disconnected(_) => {
                formatter.write_str("channel has no receiving endpoint")
            }
        }
    }
}

impl<T: fmt::Debug> std::error::Error for TrySendError<T> {}

/// Reason a timed send operation could not deliver its message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendTimeoutKind {
    /// The configured wait duration elapsed.
    TimedOut,

    /// The channel was explicitly closed.
    Closed,

    /// No receiving endpoint remains.
    Disconnected,
}

/// Error returned by a timed send operation.
///
/// The original message is retained.
pub struct SendTimeoutError<T> {
    value: T,
    kind: SendTimeoutKind,
}

impl<T> SendTimeoutError<T> {
    /// Returns the timeout failure reason.
    #[must_use]
    pub const fn kind(&self) -> SendTimeoutKind {
        self.kind
    }

    /// Recovers the message that was not delivered.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: fmt::Debug> fmt::Debug for SendTimeoutError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SendTimeoutError")
            .field("kind", &self.kind)
            .field("value", &self.value)
            .finish()
    }
}

impl<T> fmt::Display for SendTimeoutError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            SendTimeoutKind::TimedOut => {
                formatter.write_str("channel send timed out")
            }
            SendTimeoutKind::Closed => {
                formatter.write_str("channel is closed")
            }
            SendTimeoutKind::Disconnected => {
                formatter.write_str("channel has no receiving endpoint")
            }
        }
    }
}

impl<T: fmt::Debug> std::error::Error for SendTimeoutError<T> {}

// ============================================================================
// RECEIVE STATUS
// ============================================================================

/// Result of a non-blocking receive attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryRecvError {
    /// No message is currently available, but the channel may receive more.
    Empty,

    /// The channel is closed or has no remaining sender, and its queue
    /// has been drained.
    Disconnected,
}

// ============================================================================
// CHANNEL CONSTRUCTION
// ============================================================================

/// Creates an unbounded, multi-producer, multi-consumer channel.
///
/// The queue grows subject to available resources.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::concurrency::channels::channel;
///
/// let (sender, receiver) = channel::<String>();
/// sender.send("hello".to_owned()).expect("send succeeds");
/// assert_eq!(receiver.recv().expect("receive succeeds"), "hello");
/// ```
#[must_use]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    channel_with_config(ChannelConfig::unbounded())
}

/// Creates a bounded, multi-producer, multi-consumer channel.
///
/// The capacity must be greater than zero.
///
/// A full channel applies backpressure to blocking senders.
///
/// # Errors
///
/// Returns an invalid-input error when `capacity` is zero.
pub fn bounded_channel<T>(
    capacity: usize,
) -> Result<(Sender<T>, Receiver<T>)> {
    let config = ChannelConfig::bounded(capacity)?;
    Ok(channel_with_config(config))
}

/// Creates a channel using an explicit configuration.
#[must_use]
pub fn channel_with_config<T>(
    config: ChannelConfig,
) -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(ChannelShared {
        state: Mutex::new(ChannelState {
            queue: VecDeque::new(),
            closed: false,
            sender_alive: true,
            receiver_alive: true,
        }),
        readable: Condvar::new(),
        writable: Condvar::new(),
        capacity: config.capacity(),
    });

    let sender = Sender {
        lease: Arc::new(SenderLease {
            shared: Arc::clone(&shared),
        }),
    };

    let receiver = Receiver {
        lease: Arc::new(ReceiverLease { shared }),
    };

    (sender, receiver)
}

// ============================================================================
// SENDER IMPLEMENTATION
// ============================================================================

impl<T> Sender<T> {
    fn shared(&self) -> &ChannelShared<T> {
        &self.lease.shared
    }

    /// Sends a message, waiting while a bounded channel is full.
    ///
    /// The operation returns when the message is queued or the channel
    /// becomes closed or disconnected.
    ///
    /// # Errors
    ///
    /// Returns the original message inside [`SendError`] if delivery fails.
    pub fn send(&self, value: T) -> std::result::Result<(), SendError<T>> {
        let shared = self.shared();
        let mut value = Some(value);
        let mut state = shared.lock();

        loop {
            if state.closed {
                return Err(SendError {
                    value: value.take().expect("message is retained"),
                    reason: SendFailure::Closed,
                });
            }

            if !state.receiver_alive {
                return Err(SendError {
                    value: value.take().expect("message is retained"),
                    reason: SendFailure::Disconnected,
                });
            }

            if !shared.is_full(&state) {
                state.queue.push_back(
                    value.take().expect("message is retained"),
                );
                drop(state);
                shared.readable.notify_one();
                return Ok(());
            }

            state = shared
                .writable
                .wait(state)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    /// Attempts to send a message without waiting.
    ///
    /// # Errors
    ///
    /// Returns the original message when the channel is full, closed
    /// or disconnected.
    pub fn try_send(
        &self,
        value: T,
    ) -> std::result::Result<(), TrySendError<T>> {
        let shared = self.shared();
        let mut state = shared.lock();

        if state.closed {
            return Err(TrySendError::Closed(value));
        }

        if !state.receiver_alive {
            return Err(TrySendError::Disconnected(value));
        }

        if shared.is_full(&state) {
            return Err(TrySendError::Full(value));
        }

        state.queue.push_back(value);
        drop(state);

        shared.readable.notify_one();
        Ok(())
    }

    /// Sends a message, waiting for capacity for at most `timeout`.
    ///
    /// A zero timeout performs an immediate non-blocking capacity check.
    ///
    /// # Errors
    ///
    /// Returns the original message if the operation times out,
    /// the channel closes or all receivers disappear.
    pub fn send_timeout(
        &self,
        value: T,
        timeout: Duration,
    ) -> std::result::Result<(), SendTimeoutError<T>> {
        let shared = self.shared();
        let started = Instant::now();
        let mut value = Some(value);
        let mut state = shared.lock();

        loop {
            if state.closed {
                return Err(SendTimeoutError {
                    value: value.take().expect("message is retained"),
                    kind: SendTimeoutKind::Closed,
                });
            }

            if !state.receiver_alive {
                return Err(SendTimeoutError {
                    value: value.take().expect("message is retained"),
                    kind: SendTimeoutKind::Disconnected,
                });
            }

            if !shared.is_full(&state) {
                state.queue.push_back(
                    value.take().expect("message is retained"),
                );
                drop(state);
                shared.readable.notify_one();
                return Ok(());
            }

            let remaining = timeout.saturating_sub(started.elapsed());

            if remaining.is_zero() {
                return Err(SendTimeoutError {
                    value: value.take().expect("message is retained"),
                    kind: SendTimeoutKind::TimedOut,
                });
            }

            let wait_result = shared.writable.wait_timeout(state, remaining);

            let (next_state, timeout_result) = match wait_result {
                Ok(result) => result,
                Err(poisoned) => poisoned.into_inner(),
            };

            state = next_state;

            if timeout_result.timed_out()
                && shared.is_full(&state)
                && !state.closed
                && state.receiver_alive
            {
                return Err(SendTimeoutError {
                    value: value.take().expect("message is retained"),
                    kind: SendTimeoutKind::TimedOut,
                });
            }
        }
    }

    /// Explicitly closes the channel for all endpoints.
    ///
    /// Messages already queued remain available to receivers.
    ///
    /// Repeated calls are harmless.
    pub fn close(&self) {
        self.shared().close();
    }

    /// Returns whether the channel has been closed or disconnected.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        let state = self.shared().lock();
        state.closed || !state.receiver_alive
    }

    /// Returns the current number of queued messages.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shared().lock().queue.len()
    }

    /// Returns whether the queue is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the configured queue capacity.
    ///
    /// `None` means unbounded.
    #[must_use]
    pub const fn capacity(&self) -> Option<NonZeroUsize> {
        self.shared().capacity.limit()
    }
}

// ============================================================================
// RECEIVER IMPLEMENTATION
// ============================================================================

impl<T> Receiver<T> {
    fn shared(&self) -> &ChannelShared<T> {
        &self.lease.shared
    }

    /// Receives the next message, waiting until one is available.
    ///
    /// Messages queued before closure are delivered before disconnection
    /// is reported.
    ///
    /// # Errors
    ///
    /// Returns a structured communication error after the channel is
    /// closed or disconnected and its queue is empty.
    pub fn recv(&self) -> Result<T> {
        let shared = self.shared();
        let mut state = shared.lock();

        loop {
            if let Some(value) = state.queue.pop_front() {
                drop(state);
                shared.writable.notify_one();
                return Ok(value);
            }

            if state.closed || !state.sender_alive {
                return Err(receive_disconnected_error());
            }

            state = shared
                .readable
                .wait(state)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    /// Attempts to receive a message without waiting.
    ///
    /// # Errors
    ///
    /// Returns [`TryRecvError::Empty`] when no message is currently
    /// available and the channel may still receive messages.
    ///
    /// Returns [`TryRecvError::Disconnected`] after closure or sender
    /// disconnection when the queue has been drained.
    pub fn try_recv(
        &self,
    ) -> std::result::Result<T, TryRecvError> {
        let shared = self.shared();
        let mut state = shared.lock();

        if let Some(value) = state.queue.pop_front() {
            drop(state);
            shared.writable.notify_one();
            return Ok(value);
        }

        if state.closed || !state.sender_alive {
            return Err(TryRecvError::Disconnected);
        }

        Err(TryRecvError::Empty)
    }

    /// Receives a message, waiting for at most `timeout`.
    ///
    /// Returns `Ok(None)` when the timeout elapses while the channel
    /// remains connected and no message is available.
    ///
    /// # Errors
    ///
    /// Returns a structured communication error when the channel
    /// disconnects and its queue is empty.
    pub fn recv_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Option<T>> {
        let shared = self.shared();
        let started = Instant::now();
        let mut state = shared.lock();

        loop {
            if let Some(value) = state.queue.pop_front() {
                drop(state);
                shared.writable.notify_one();
                return Ok(Some(value));
            }

            if state.closed || !state.sender_alive {
                return Err(receive_disconnected_error());
            }

            let remaining = timeout.saturating_sub(started.elapsed());

            if remaining.is_zero() {
                return Ok(None);
            }

            let wait_result = shared.readable.wait_timeout(state, remaining);

            let (next_state, timeout_result) = match wait_result {
                Ok(result) => result,
                Err(poisoned) => poisoned.into_inner(),
            };

            state = next_state;

            if timeout_result.timed_out()
                && state.queue.is_empty()
                && !state.closed
                && state.sender_alive
            {
                return Ok(None);
            }
        }
    }

    /// Explicitly closes the channel for all endpoints.
    ///
    /// Messages already queued remain available to receivers.
    pub fn close(&self) {
        self.shared().close();
    }

    /// Returns whether the channel has been closed or disconnected.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        let state = self.shared().lock();
        state.closed || !state.sender_alive
    }

    /// Returns the current number of queued messages.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shared().lock().queue.len()
    }

    /// Returns whether the queue is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the configured queue capacity.
    ///
    /// `None` means unbounded.
    #[must_use]
    pub const fn capacity(&self) -> Option<NonZeroUsize> {
        self.shared().capacity.limit()
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn receive_disconnected_error() -> Error {
    Error::new(
        ErrorKind::Communication,
        "channel is closed or disconnected and contains no remaining messages",
    )
    .in_component("stdlib.concurrency.channels")
    .during_operation("channel.receive")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn unbounded_channel_delivers_messages_in_fifo_order() {
        let (sender, receiver) = channel();

        sender.send(10).expect("send");
        sender.send(20).expect("send");
        sender.send(30).expect("send");

        assert_eq!(receiver.recv().expect("receive"), 10);
        assert_eq!(receiver.recv().expect("receive"), 20);
        assert_eq!(receiver.recv().expect("receive"), 30);
    }

    #[test]
    fn bounded_channel_applies_backpressure() {
        let (sender, receiver) =
            bounded_channel::<usize>(1).expect("valid capacity");

        sender.send(1).expect("first send");

        let error = sender.try_send(2).expect_err("queue is full");

        assert!(error.is_full());
        assert_eq!(error.into_inner(), 2);

        assert_eq!(receiver.recv().expect("receive"), 1);
        sender.send(2).expect("capacity is available");
        assert_eq!(receiver.recv().expect("receive"), 2);
    }

    #[test]
    fn zero_capacity_is_rejected() {
        let result = bounded_channel::<usize>(0);

        assert!(result.is_err());
    }

    #[test]
    fn closed_channel_drains_messages_before_disconnect() {
        let (sender, receiver) = channel();

        sender.send(1).expect("send");
        sender.send(2).expect("send");
        sender.close();

        assert_eq!(receiver.recv().expect("first queued message"), 1);
        assert_eq!(receiver.recv().expect("second queued message"), 2);
        assert!(receiver.recv().is_err());
    }

    #[test]
    fn dropping_last_sender_disconnects_receiver() {
        let (sender, receiver) = channel::<usize>();

        sender.send(42).expect("send");
        drop(sender);

        assert_eq!(receiver.recv().expect("queued message"), 42);
        assert!(receiver.recv().is_err());
    }

    #[test]
    fn dropping_last_receiver_disconnects_sender() {
        let (sender, receiver) = channel::<usize>();

        drop(receiver);

        let error = sender.send(42).expect_err("receiver is gone");

        assert_eq!(error.reason(), SendFailure::Disconnected);
        assert_eq!(error.into_inner(), 42);
    }

    #[test]
    fn sender_clones_share_the_same_channel() {
        let (sender, receiver) = channel();

        let second_sender = sender.clone();

        sender.send(1).expect("first sender");
        second_sender.send(2).expect("second sender");

        assert_eq!(receiver.recv().expect("receive"), 1);
        assert_eq!(receiver.recv().expect("receive"), 2);
    }

    #[test]
    fn receiver_clones_are_competing_consumers() {
        let (sender, receiver) = channel();

        let second_receiver = receiver.clone();

        sender.send(1).expect("send");
        sender.send(2).expect("send");

        let first = receiver.recv().expect("first consumer");
        let second = second_receiver.recv().expect("second consumer");

        assert_ne!(first, second);
        assert_eq!(first + second, 3);
    }

    #[test]
    fn nonblocking_receive_distinguishes_empty_from_disconnected() {
        let (sender, receiver) = channel::<usize>();

        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));

        drop(sender);

        assert_eq!(
            receiver.try_recv(),
            Err(TryRecvError::Disconnected)
        );
    }

    #[test]
    fn timed_receive_returns_none_when_no_message_arrives() {
        let (_sender, receiver) = channel::<usize>();

        assert_eq!(
            receiver.recv_timeout(Duration::ZERO).expect("timeout"),
            None
        );
    }

    #[test]
    fn timed_send_preserves_message_when_queue_is_full() {
        let (sender, _receiver) =
            bounded_channel::<usize>(1).expect("valid capacity");

        sender.send(1).expect("fill queue");

        let error = sender
            .send_timeout(2, Duration::ZERO)
            .expect_err("queue is full");

        assert_eq!(error.kind(), SendTimeoutKind::TimedOut);
        assert_eq!(error.into_inner(), 2);
    }

    #[test]
    fn close_wakes_a_blocked_receiver() {
        let (sender, receiver) = channel::<usize>();

        let handle = thread::spawn(move || receiver.recv());

        sender.close();

        assert!(handle.join().expect("thread joins").is_err());
    }

    #[test]
    fn close_wakes_a_blocked_sender() {
        let (sender, receiver) =
            bounded_channel::<usize>(1).expect("valid capacity");

        sender.send(1).expect("fill queue");

        let blocked_sender = sender.clone();

        let handle = thread::spawn(move || blocked_sender.send(2));

        sender.close();

        let error = handle
            .join()
            .expect("thread joins")
            .expect_err("closed channel rejects send");

        assert_eq!(error.reason(), SendFailure::Closed);

        // The message that was queued before closure remains available.
        assert_eq!(receiver.recv().expect("drain queue"), 1);
    }

    #[test]
    fn multiple_threads_can_send_to_one_receiver() {
        let (sender, receiver) =
            bounded_channel::<usize>(4).expect("valid capacity");

        let first_sender = sender.clone();
        let second_sender = sender.clone();

        let first = thread::spawn(move || {
            for value in 0..100 {
                first_sender.send(value).expect("first sender");
            }
        });

        let second = thread::spawn(move || {
            for value in 100..200 {
                second_sender.send(value).expect("second sender");
            }
        });

        let mut received = Vec::new();

        for _ in 0..200 {
            received.push(receiver.recv().expect("receive"));
        }

        first.join().expect("first thread");
        second.join().expect("second thread");

        received.sort_unstable();

        assert_eq!(received, (0..200).collect::<Vec<_>>());
    }

    #[test]
    fn channel_capacity_is_reported_correctly() {
        let (_unbounded_sender, unbounded_receiver) = channel::<usize>();

        assert_eq!(unbounded_receiver.capacity(), None);

        let (_bounded_sender, bounded_receiver) =
            bounded_channel::<usize>(17).expect("valid capacity");

        assert_eq!(
            bounded_receiver.capacity().map(NonZeroUsize::get),
            Some(17)
        );
    }

    #[test]
    fn repeated_close_is_safe() {
        let (sender, receiver) = channel::<usize>();

        sender.close();
        sender.close();
        receiver.close();

        assert!(receiver.recv().is_err());
    }
}
