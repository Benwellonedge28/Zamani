
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Channels
//!
//! Path: `src/zur/concurrency/channel.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the ZUR-facing API for typed local message channels.
//!
//! This module deliberately reuses the canonical standard-library
//! channel implementation rather than introducing another queue,
//! endpoint-lifecycle implementation, or buffering system.
//!
//! ## Ownership
//!
//! The standard-library channel implementation owns:
//!
//! - Local channel construction.
//! - Bounded and unbounded buffering.
//! - Sender and receiver endpoint behavior.
//! - Message queue ordering.
//! - Blocking and non-blocking operations.
//! - Timed operations.
//! - Channel closure.
//! - Endpoint disconnection.
//! - Local channel errors.
//!
//! This module owns:
//!
//! - The ZUR public module path for local channels.
//! - Explicit re-exports of the canonical channel API.
//! - Documentation of ZUR channel integration boundaries.
//!
//! ## Existing authorities
//!
//! - `stdlib/concurrency/channels.rs`:
//!   Canonical local channel implementation.
//!
//! - `stdlib/concurrency/mod.rs`:
//!   Standard-library concurrency composition.
//!
//! - `zur/concurrency/task.rs`:
//!   Task lifecycle and completion.
//!
//! - `zur/concurrency/synchronization.rs`:
//!   Local synchronization primitives.
//!
//! - `zur/execution/cancellation.rs`:
//!   Cooperative execution cancellation.
//!
//! - `distributed/` and `networking/`:
//!   Distributed and remote communication.
//!
//! - `grammar/concurrency/channels.g4`:
//!   Channel source syntax.
//!
//! - `grammar/spec/concurrency.md`:
//!   Normative channel and concurrency semantics.
//!
//! ## Ownership boundaries
//!
//! This module does not:
//!
//! - Define another channel queue.
//! - Define another sender or receiver lifecycle.
//! - Define another channel identity system.
//! - Own task execution.
//! - Own task scheduling.
//! - Create threads or workers.
//! - Implement an asynchronous executor.
//! - Implement distributed message transport.
//! - Implement network protocols.
//! - Allocate physical resources.
//! - Select CPU, GPU, QPU, FPGA or other hardware.
//! - Define source-language channel semantics.
//! - Define channel payload type checking.
//! - Replace compiler or semantic IR.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Local communication semantics
//!
//! The canonical implementation provides:
//!
//! - Multi-producer communication.
//! - Multi-consumer communication.
//! - FIFO queue insertion and removal.
//! - Competing-consumer receiver behavior.
//! - Explicit bounded buffering.
//! - Unbounded buffering subject to available resources.
//! - Blocking, non-blocking and timed operations.
//! - Explicit closure and endpoint disconnection.
//!
//! Cloned receivers compete for messages. They do not receive
//! broadcast copies of every message.
//!
//! The implementation does not promise fairness among blocked
//! senders or receivers.
//!
//! ## Scalability
//!
//! This facade introduces no application-defined limits on:
//!
//! - Channels.
//! - Senders.
//! - Receivers.
//! - Messages.
//! - Payload types.
//! - Queue capacity.
//! - Communication relationships.
//!
//! Bounded capacity is an explicit per-channel configuration.
//!
//! Unbounded buffering remains subject to actual memory,
//! allocator, address-space and platform limitations.
//!
//! Literal infinite physical storage is not promised.
//!
//! ## Cancellation
//!
//! This facade does not introduce another cancellation system.
//!
//! Blocking channel operations retain the behavior of the
//! canonical standard-library implementation.
//!
//! Execution cancellation remains owned by
//! `zur/execution/cancellation.rs`.
//!
//! A runtime or executor requiring cancellation-aware channel
//! waits must provide an explicit wake-up and cancellation adapter.
//!
//! ## Asynchronous integration
//!
//! This module does not implement another Future or executor.
//!
//! Asynchronous channel integration belongs in the existing
//! asynchronous/futures subsystem.
//!
//! ## Distributed integration
//!
//! These channels provide local in-process communication.
//!
//! They must not be treated as network channels or as proof that
//! endpoints share memory across processes or machines.
//!
//! Distributed communication must use the existing distributed
//! and networking authorities and their transport contracts.
//!
//! ## Compatibility
//!
//! Public channel types and functions are re-exported from
//! `stdlib/concurrency/channels.rs`.
//!
//! This ensures that ZUR consumers and standard-library consumers
//! use the same local channel implementation and semantics.

/// Buffering configuration for a local channel.
///
/// `Unbounded` permits queue growth subject to available resources.
/// `Bounded` applies backpressure when its configured queue capacity
/// is exhausted.
pub use crate::stdlib::concurrency::channels::ChannelCapacity;

/// Complete local channel configuration.
pub use crate::stdlib::concurrency::channels::ChannelConfig;

/// Cloneable local receiving endpoint.
///
/// Cloned receivers are competing consumers of the same queue.
pub use crate::stdlib::concurrency::channels::Receiver;

/// Reason a blocking send could not deliver its message.
pub use crate::stdlib::concurrency::channels::SendFailure;

/// Error returned by a blocking send.
///
/// The original unsent value can be recovered.
pub use crate::stdlib::concurrency::channels::SendError;

/// Error returned by a timed send.
///
/// The original unsent value can be recovered.
pub use crate::stdlib::concurrency::channels::SendTimeoutError;

/// Classification of a timed-send failure.
pub use crate::stdlib::concurrency::channels::SendTimeoutKind;

/// Cloneable local sending endpoint.
pub use crate::stdlib::concurrency::channels::Sender;

/// Error returned by a non-blocking receive.
pub use crate::stdlib::concurrency::channels::TryRecvError;

/// Error returned by a non-blocking send.
///
/// The original unsent value can be recovered.
pub use crate::stdlib::concurrency::channels::TrySendError;

/// Creates an unbounded multi-producer, multi-consumer channel.
///
/// The queue grows subject to available resources.
///
/// This is the canonical standard-library constructor.
pub use crate::stdlib::concurrency::channels::channel;

/// Creates a bounded multi-producer, multi-consumer channel.
///
/// The supplied capacity must be positive.
///
/// A full channel applies backpressure to blocking senders.
pub use crate::stdlib::concurrency::channels::bounded_channel;

/// Creates a local channel using an explicit configuration.
pub use crate::stdlib::concurrency::channels::channel_with_config;

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[test]
    fn zur_api_uses_canonical_unbounded_channel() {
        let (sender, receiver) = channel::<String>();

        sender
            .send("Zamani".to_owned())
            .expect("message should be sent");

        assert_eq!(
            receiver.recv().expect("message should be received"),
            "Zamani"
        );
    }

    #[test]
    fn zur_api_supports_bounded_backpressure() {
        let (sender, receiver) =
            bounded_channel::<usize>(1).expect("valid capacity");

        sender.send(10).expect("first message should fit");

        let error = sender
            .try_send(20)
            .expect_err("second message should encounter backpressure");

        assert!(error.is_full());
        assert_eq!(error.into_inner(), 20);

        assert_eq!(
            receiver.recv().expect("first message should be received"),
            10
        );

        sender
            .send(20)
            .expect("space should be available after receiving");

        assert_eq!(
            receiver.recv().expect("second message should be received"),
            20
        );
    }

    #[test]
    fn zur_api_preserves_fifo_order() {
        let (sender, receiver) = channel::<u64>();

        for value in 0..32 {
            sender.send(value).expect("send should succeed");
        }

        for expected in 0..32 {
            assert_eq!(
                receiver.recv().expect("receive should succeed"),
                expected
            );
        }
    }

    #[test]
    fn zur_api_supports_cloned_senders() {
        let (sender, receiver) = channel::<u64>();
        let another_sender = sender.clone();

        sender.send(1).expect("first sender");

        another_sender
            .send(2)
            .expect("cloned sender");

        assert_eq!(receiver.recv().expect("first receive"), 1);
        assert_eq!(receiver.recv().expect("second receive"), 2);
    }

    #[test]
    fn zur_api_supports_cloned_competing_receivers() {
        let (sender, receiver) = channel::<u64>();
        let another_receiver = receiver.clone();

        sender.send(1).expect("first message");
        sender.send(2).expect("second message");

        let first = receiver.recv().expect("first consumer");
        let second = another_receiver
            .recv()
            .expect("second consumer");

        assert_ne!(first, second);
        assert_eq!(first + second, 3);
    }

    #[test]
    fn zur_api_supports_timed_operations() {
        let (sender, receiver) = channel::<u64>();

        assert_eq!(
            receiver
                .recv_timeout(Duration::ZERO)
                .expect("zero-duration receive"),
            None
        );

        sender
            .send(42)
            .expect("message should be sent");

        assert_eq!(
            receiver
                .recv_timeout(Duration::ZERO)
                .expect("message should be available"),
            Some(42)
        );
    }

    #[test]
    fn zur_api_preserves_messages_when_closed() {
        let (sender, receiver) = channel::<u64>();

        sender.send(1).expect("first message");
        sender.send(2).expect("second message");

        sender.close();

        assert_eq!(receiver.recv().expect("drain first"), 1);
        assert_eq!(receiver.recv().expect("drain second"), 2);
        assert!(receiver.recv().is_err());
    }

    #[test]
    fn zur_api_exposes_channel_capacity() {
        let (unbounded_sender, unbounded_receiver) = channel::<u64>();

        assert_eq!(unbounded_sender.capacity(), None);
        assert_eq!(unbounded_receiver.capacity(), None);

        let (bounded_sender, bounded_receiver) =
            bounded_channel::<u64>(8).expect("valid capacity");

        assert_eq!(
            bounded_sender.capacity().map(|value| value.get()),
            Some(8)
        );

        assert_eq!(
            bounded_receiver.capacity().map(|value| value.get()),
            Some(8)
        );
    }
}
