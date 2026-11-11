
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Concurrency
//!
//! Path: `src/stdlib/concurrency/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the public composition boundary for Zamani's
//! portable concurrency standard library.
//!
//! It provides a unified API for:
//!
//! - Classical synchronization.
//! - Logical task creation and observation.
//! - Typed message passing.
//! - Cooperative cancellation.
//! - Asynchronous computation.
//! - Future composition.
//! - Runtime-owned task submission.
//!
//! The same logical concurrency APIs must be usable by programs
//! targeting different execution environments, provided the
//! required semantics and capabilities are supported.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Public concurrency module composition.
//! - Stable concurrency API exports.
//! - Documentation of concurrency API boundaries.
//! - Integration between the standard-library concurrency modules.
//!
//! It does NOT own:
//!
//! - Physical thread creation.
//! - Worker-pool implementation.
//! - Task scheduling.
//! - CPU, GPU, FPGA or QPU selection.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Distributed task placement.
//! - Network routing.
//! - Quantum scheduling.
//! - HDL synthesis or physical execution.
//! - Runtime lifecycle.
//! - Compiler intermediate representations.
//! - Language parsing.
//! - Semantic analysis.
//! - Concurrency optimization.
//! - Hardware synchronization.
//!
//! Those responsibilities belong to the authoritative
//! compiler, semantic, IR, runtime, distributed, quantum,
//! HDL and HAL subsystems.
//!
//! # Module ownership
//!
//! ## synchronization
//!
//! Provides portable classical synchronization primitives,
//! including mutexes, read/write locks, condition variables,
//! barriers, one-time initialization and atomic operations.
//!
//! It does not provide a scheduler or distributed lock service.
//!
//! ## tasks
//!
//! Provides logical task handles, task lifecycle observation,
//! cancellation requests and runtime executor submission
//! contracts.
//!
//! A logical task is not necessarily a physical thread.
//!
//! ## channels
//!
//! Provides typed in-process message channels, including
//! bounded and unbounded communication, backpressure,
//! blocking operations and non-blocking operations.
//!
//! Distributed communication remains the responsibility
//! of the distributed and networking subsystems.
//!
//! ## cancellation
//!
//! Provides cooperative cancellation tokens, child tokens,
//! cancellation reasons and asynchronous cancellation
//! observation.
//!
//! Cancellation does not forcibly terminate physical work.
//!
//! ## asynchronous
//!
//! Provides Rust Future interoperability, boxed futures,
//! asynchronous composition and integration with the
//! runtime-owned task submission API.
//!
//! It does not implement an independent executor.
//!
//! # POCO-REAF
//!
//! Concurrency APIs describe logical computation and its
//! required synchronization and communication semantics.
//!
//! They must not assume a particular:
//!
//! - Processor architecture.
//! - CPU count.
//! - Thread count.
//! - GPU count.
//! - FPGA count.
//! - QPU count.
//! - Accelerator count.
//! - Memory capacity.
//! - Register width.
//! - Distributed node count.
//! - Computational domain.
//! - Hardware vendor.
//! - Execution topology.
//!
//! No application-defined universal maximum is imposed on
//! tasks, channels, participants, synchronization objects,
//! futures, devices or execution domains.
//!
//! Actual limits are determined by available resources,
//! platform capabilities, memory, runtime configuration
//! and explicitly configured operational budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Requirements and execution
//!
//! A concurrency operation may express:
//!
//! - Requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Feasibility.
//!
//! These concepts retain their respective standard-library
//! and semantic authorities.
//!
//! This module does not independently determine whether
//! a requested operation is physically feasible.
//!
//! # Grammar integration
//!
//! Relevant language grammar authorities include:
//!
//! - grammar/concurrency/concurrency.g4
//! - grammar/concurrency/tasks.g4
//! - grammar/concurrency/task-parallel.g4
//! - grammar/concurrency/spawn.g4
//! - grammar/concurrency/async.g4
//! - grammar/concurrency/await.g4
//! - grammar/concurrency/synchronization.g4
//! - grammar/concurrency/channels.g4
//! - grammar/concurrency/cancellation.g4
//! - grammar/expressions/async.g4
//! - grammar/statements/concurrency.g4
//! - grammar/execution/execution-context.g4
//!
//! Normative specifications:
//!
//! - grammar/spec/concurrency.md
//! - grammar/spec/determinism.md
//!
//! Grammar defines source-language syntax and intent.
//! This module provides implementation APIs.
//!
//! Neither replaces the other.
//!
//! # Integration
//!
//! Operation context:
//!     crate::stdlib::context
//!
//! Structured errors:
//!     crate::stdlib::error
//!
//! Runtime task identity:
//!     crate::runtime::concurrency
//!
//! Canonical future composition:
//!     crate::classical::concurrency::futures
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Quantum computation:
//!     crate::quantum
//!
//! Hardware description:
//!     crate::hdl
//!
//! Universal intermediate representation:
//!     crate::zuir
//!
//! Physical execution:
//!     crate::runtime and HAL providers
//!
//! # Initialization
//!
//! This module deliberately has no global initialization
//! function.
//!
//! Its public types and synchronization primitives are
//! constructed explicitly by their callers.
//!
//! Runtime-owned services must be initialized by the runtime.
//!
//! Importing this module must not initialize every concurrency
//! subsystem or allocate physical execution resources.
//!
//! # Safety
//!
//! This module and its implementation modules prohibit unsafe
//! Rust.
//!
//! # Compatibility
//!
//! The existing legacy modules:
//!
//! - crate::stdlib::sync
//! - crate::stdlib::concurrent
//!
//! remain separate migration facades until their callers
//! have been migrated and compatibility has been verified.
//!
//! They must not become competing implementation authorities.

// ============================================================================
// AUTHORITATIVE IMPLEMENTATION MODULES
// ============================================================================

/// Portable classical synchronization primitives.
///
/// Provides mutexes, read/write locks, condition variables,
/// barriers, one-time initialization and atomic operations.
///
/// Does not provide task scheduling or distributed locks.
pub mod synchronization;

/// Logical tasks and runtime executor submission contracts.
///
/// The runtime retains ownership of task scheduling and
/// physical execution.
pub mod tasks;

/// Typed, in-process message channels.
///
/// Provides bounded and unbounded channels, backpressure,
/// explicit closure and message-delivery semantics.
pub mod channels;

/// Cooperative cancellation infrastructure.
///
/// Provides cancellation tokens, parent-child cancellation
/// relationships and asynchronous cancellation observation.
pub mod cancellation;

/// Asynchronous computation and Future interoperability.
///
/// Composes the canonical future implementation with
/// runtime-owned task submission.
pub mod asynchronous;

// ============================================================================
// SYNCHRONIZATION PUBLIC API
// ============================================================================

/// Common synchronization primitives.
///
/// These are portable local synchronization facilities.
/// Their availability and implementation characteristics
/// remain subject to the target Rust platform.
pub use synchronization::{
    Barrier,
    BarrierWaitResult,
    Condvar,
    InvalidBarrierParticipants,
    Mutex,
    MutexGuard,
    Once,
    OnceLock,
    PoisonError,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    TryLockError,
    WaitTimeoutResult,
};

/// Target-supported atomic operations.
///
/// Atomic availability and lock-free guarantees depend on
/// the compilation target.
pub use synchronization::atomic;

// ============================================================================
// LOGICAL TASK PUBLIC API
// ============================================================================

/// Runtime-owned logical task identity.
pub use tasks::TaskId;

/// Logical task lifecycle states.
pub use tasks::TaskStatus;

/// Runtime task submission interface.
pub use tasks::TaskExecutor;

/// Boxed future accepted by a task executor.
pub use tasks::BoxTaskFuture;

/// Logical task completion and observation handle.
pub use tasks::TaskHandle;

/// Submits a logical task using the canonical task API.
pub use tasks::spawn;

/// Submits a logical task using the caller-supplied
/// construction context supported by the task API.
pub use tasks::spawn_with;

// ============================================================================
// CHANNEL PUBLIC API
// ============================================================================

/// Channel capacity configuration.
pub use channels::ChannelCapacity;

/// Channel construction configuration.
pub use channels::ChannelConfig;

/// Channel sender.
pub use channels::Sender;

/// Channel receiver.
pub use channels::Receiver;

/// Error returned when sending fails because a channel
/// is disconnected or otherwise unavailable.
pub use channels::SendError;

/// Classification of channel send failures.
pub use channels::SendFailure;

/// Error returned by non-blocking channel sends.
pub use channels::TrySendError;

/// Error returned by timed channel sends.
pub use channels::SendTimeoutError;

/// Classification of timed send failures.
pub use channels::SendTimeoutKind;

/// Error returned by non-blocking channel receives.
pub use channels::TryRecvError;

/// Creates a new unbounded channel.
///
/// The channel remains subject to available memory.
pub use channels::channel;

/// Creates a new bounded channel.
///
/// The capacity is supplied by the caller and is not a
/// universal language-level concurrency limit.
pub use channels::bounded_channel;

/// Creates a channel using an explicit configuration.
pub use channels::channel_with_config;

// ============================================================================
// CANCELLATION PUBLIC API
// ============================================================================

/// Cooperative cancellation token.
pub use cancellation::CancellationToken;

/// Future that resolves when cancellation is requested.
pub use cancellation::CancellationFuture;

// ============================================================================
// ASYNCHRONOUS PUBLIC API
// ============================================================================

/// Canonical Rust asynchronous Future interoperability.
pub use asynchronous::AsyncFuture;

/// Asynchronous polling result.
pub use asynchronous::Poll;

/// Asynchronous task execution context.
pub use asynchronous::Context;

/// Asynchronous task waker.
pub use asynchronous::Waker;

/// Boxed asynchronous future.
pub use asynchronous::BoxFuture;

/// Boxed local asynchronous future.
pub use asynchronous::LocalBoxFuture;

/// Fallible asynchronous computation.
pub use asynchronous::AsyncResult;

/// Fallible local asynchronous computation.
pub use asynchronous::LocalAsyncResult;

/// Asynchronous computation returning a value.
pub use asynchronous::AsyncValue;

/// Local asynchronous computation returning a value.
pub use asynchronous::LocalAsyncValue;

/// Descriptive asynchronous future properties.
pub use asynchronous::FutureProperties;

/// Creates a boxed asynchronous future.
pub use asynchronous::boxed;

/// Creates a boxed local asynchronous future.
pub use asynchronous::boxed_local;

/// Creates an immediately ready future.
pub use asynchronous::ready;

/// Creates a pending future.
pub use asynchronous::pending;

/// Creates a future from a polling function.
pub use asynchronous::poll_fn;

// ============================================================================
// API BOUNDARY TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;

    #[test]
    fn synchronization_api_is_available() {
        let lock = Mutex::new(42_u64);

        assert_eq!(
            *lock.lock().expect("mutex should be available"),
            42
        );
    }

    #[test]
    fn barrier_api_is_available() {
        let barrier = Barrier::new(
            NonZeroUsize::new(1)
                .expect("one is non-zero"),
        );

        assert_eq!(
            barrier.participant_count().get(),
            1
        );

        assert!(barrier.wait().is_leader());
    }

    #[test]
    fn task_lifecycle_is_available() {
        assert!(TaskStatus::Created.is_terminal() == false);
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn channel_api_is_available() {
        let (sender, receiver) = channel::<u64>();

        sender.send(42)
            .expect("sending should succeed");

        assert_eq!(
            receiver.recv().expect("receiving should succeed"),
            42
        );
    }

    #[test]
    fn bounded_channel_api_is_available() {
        let config = ChannelConfig::bounded(8)
            .expect("positive channel capacity is valid");

        assert!(config.capacity().is_bounded());
    }

    #[test]
    fn cancellation_api_is_available() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());

        assert!(token.cancel());

        assert!(token.is_cancelled());
    }

    #[test]
    fn asynchronous_future_api_is_available() {
        let mut future = Box::pin(ready(42_u64));

        let waker = std::task::Waker::noop();

        let mut context = std::task::Context::from_waker(waker);

        assert_eq!(
            std::future::Future::poll(
                future.as_mut(),
                &mut context
            ),
            Poll::Ready(42)
        );
    }
}
