
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Asynchronous Computation
//!
//! File: `src/stdlib/concurrency/asynchronous.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only. No `unsafe`.
//!
//! ## Purpose
//!
//! Provide the stable standard-library API for asynchronous computation.
//!
//! This module composes the canonical future implementation from
//! `crate::classical::concurrency::futures` with the runtime-owned task
//! submission interface from `super::tasks`.
//!
//! It provides:
//!
//! - Standard Rust Future interoperability.
//! - Send and local-only boxed future types.
//! - Fallible asynchronous result aliases.
//! - Future boxing.
//! - Dynamic future composition.
//! - Ordered joining of futures.
//! - Fail-fast joining of fallible futures.
//! - Runtime-owned task submission.
//! - Runtime-owned task completion handles.
//! - Cooperative task cancellation observation.
//!
//! ## Architectural ownership
//!
//! This module owns the standard-library asynchronous API boundary.
//!
//! It does NOT own:
//!
//! - Source-language async or await syntax.
//! - The frontend AST.
//! - Semantic type checking or awaitability validation.
//! - Canonical intermediate representations.
//! - Task identity allocation.
//! - Runtime task scheduling.
//! - Runtime future polling.
//! - Worker pools or operating-system threads.
//! - Timers or event loops.
//! - I/O reactors.
//! - Network transports.
//! - Distributed placement.
//! - Hardware allocation.
//! - CPU, GPU, FPGA or QPU selection.
//! - Quantum execution.
//! - HDL execution.
//! - Runtime lifecycle.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Canonical authorities
//!
//! Future composition:
//! `crate::classical::concurrency::futures`
//!
//! Logical task submission and completion:
//! `crate::stdlib::concurrency::tasks`
//!
//! Operation context:
//! `crate::stdlib::context`
//!
//! Standard-library errors:
//! `crate::stdlib::error`
//!
//! Runtime execution:
//! `crate::runtime`
//!
//! Source syntax:
//! `grammar/expressions/async.g4`
//! `grammar/functions/async.g4`
//! `grammar/concurrency/async.g4`
//! `grammar/concurrency/await.g4`
//! `grammar/concurrency/futures.g4`
//! `grammar/concurrency/tasks.g4`
//!
//! Normative semantics:
//! `grammar/spec/concurrency.md`
//! `grammar/spec/determinism.md`
//!
//! ## Fundamental distinction
//!
//! A Future represents a computation whose result may become available
//! later. It does not identify a physical execution resource.
//!
//! A Future is not automatically:
//!
//! - a thread;
//! - a worker;
//! - a process;
//! - a CPU core;
//! - a GPU kernel;
//! - a QPU operation;
//! - a remote task;
//! - a distributed participant.
//!
//! Future composition does not itself create parallelism.
//!
//! Actual execution requires an executor supplied by the runtime.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no application-defined maximum on:
//!
//! - futures;
//! - asynchronous operations;
//! - logical tasks;
//! - concurrent tasks;
//! - nested asynchronous computations;
//! - execution providers;
//! - hardware devices;
//! - distributed participants;
//! - computational domains.
//!
//! Collections and future composition remain subject to actual memory,
//! address-space, allocator, executor and target-resource availability.
//!
//! These implementation limits are not universal Zamani language limits.
//!
//! No finite implementation promises literal infinite computation.
//!
//! ## Cancellation
//!
//! Dropping an owned future drops that future's local state.
//!
//! It does not guarantee that an operation already submitted to an external
//! provider, device, remote service or runtime has stopped.
//!
//! Runtime task cancellation is provided through the task API.
//!
//! Cancellation remains cooperative.
//!
//! ## Error behavior
//!
//! This module does not silently convert failed asynchronous operations
//! into successful results.
//!
//! Fallible future composition preserves the error type supplied by the
//! operation.
//!
//! Runtime task submission returns the structured error type defined by
//! the standard-library task implementation.
//!
//! ## Blocking behavior
//!
//! A future must not perform unbounded blocking work on an executor thread
//! unless the executor explicitly supports that execution model.
//!
//! Blocking operations should use an appropriate runtime or provider
//! interface. This module does not create a blocking thread pool.
//!
//! ## Compatibility
//!
//! The old `stdlib::concurrent` module must not retain a second independent
//! Future implementation.
//!
//! Existing callers should migrate to this module's API or to the canonical
//! classical future-composition API.
//!
//! Existing public symbols should be preserved through reviewed compatibility
//! facades where necessary.

use core::future::Future;

// ============================================================================
// STANDARD FUTURE INTEROPERABILITY
// ============================================================================

/// The standard Rust asynchronous computation contract.
///
/// This is the standard-library Future trait. Zamani does not introduce
/// another incompatible future trait.
///
/// The executor determines when and where a future is polled.
pub use core::future::Future as AsyncFuture;

/// Standard future polling state.
pub use core::task::Poll;

/// Standard future polling context.
pub use core::task::Context;

/// Standard future wake-up interface.
pub use core::task::Waker;

// ============================================================================
// CANONICAL FUTURE COMPOSITION
// ============================================================================

/// Canonical future-composition types and operations.
///
/// These exports are re-exports of the existing classical concurrency
/// implementation. They are not a second implementation.
///
/// `join_all` and `try_join_all` do not spawn tasks or guarantee parallel
/// execution. They compose futures that are polled by their enclosing
/// executor.
pub use crate::classical::concurrency::futures::{
    boxed,
    boxed_local,
    join_all,
    try_join_all,
    BoxFuture,
    JoinAll,
    LocalBoxFuture,
    TryJoinAll,
};

// ============================================================================
// FALLIBLE ASYNCHRONOUS RESULTS
// ============================================================================

/// A Send asynchronous computation returning a fallible result.
///
/// The error type remains supplied by the operation's owning subsystem.
///
/// This alias does not prescribe a particular error taxonomy.
pub type AsyncResult<'a, T, E> =
    BoxFuture<'a, core::result::Result<T, E>>;

/// A local asynchronous computation returning a fallible result.
///
/// This type does not require the future to be Send and is suitable only
/// for execution environments that support local, non-Send futures.
pub type LocalAsyncResult<'a, T, E> =
    LocalBoxFuture<'a, core::result::Result<T, E>>;

/// A Send asynchronous computation returning a value.
///
/// This is a descriptive alias for the canonical boxed future type.
pub type AsyncValue<'a, T> = BoxFuture<'a, T>;

/// A local asynchronous computation returning a value.
///
/// This type does not require Send.
pub type LocalAsyncValue<'a, T> = LocalBoxFuture<'a, T>;

// ============================================================================
// STANDARD FUTURE CONSTRUCTORS
// ============================================================================

/// Creates a future that never resolves.
///
/// This is useful when implementing event-driven operations that remain
/// pending until an external event or cancellation mechanism wakes them.
///
/// This function does not create an event source or executor.
pub use core::future::pending;

/// Creates a future that resolves immediately with the supplied value.
pub use core::future::ready;

/// Creates a future from a polling function.
///
/// The caller remains responsible for correct polling and wake-up behavior.
pub use core::future::poll_fn;

// ============================================================================
// RUNTIME-OWNED TASK EXECUTION INTEGRATION
// ============================================================================

/// Runtime-owned task submission and completion APIs.
///
/// The task implementation owns task identity, lifecycle observation,
/// cooperative cancellation, and executor submission contracts.
///
/// The runtime owns actual scheduling and execution.
///
/// This module only exposes the existing task implementation through
/// the asynchronous API boundary.
pub use super::tasks::{
    spawn,
    spawn_with,
    BoxTaskFuture,
    TaskExecutor,
    TaskHandle,
    TaskId,
    TaskStatus,
};

// ============================================================================
// ASYNCHRONOUS API CONTRACT
// ============================================================================

/// Describes the execution properties required by an asynchronous future.
///
/// This trait is deliberately descriptive. It does not choose an executor,
/// allocate resources, or determine physical placement.
///
/// Implementations must report stable properties of the future type.
pub trait FutureProperties {
    /// Whether the future can be transferred between threads.
    ///
    /// This describes the Rust type's Send capability. It does not require
    /// an executor to use multiple threads.
    fn is_send_capable(&self) -> bool;

    /// Whether the future has a statically known output type.
    ///
    /// Rust futures have a statically known associated output type.
    /// Dynamically erased futures retain that type through their trait
    /// object declaration.
    fn has_known_output_type(&self) -> bool {
        true
    }
}

/// Future properties for futures that implement Send.
impl<F> FutureProperties for F
where
    F: Future + Send,
{
    fn is_send_capable(&self) -> bool {
        true
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::task::Poll;
    use std::sync::Arc;
    use std::task::{Wake, Waker};

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn with_context<R>(operation: impl FnOnce(&mut Context<'_>) -> R) -> R {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        operation(&mut context)
    }

    #[test]
    fn canonical_boxed_future_is_available() {
        let mut future = boxed(async { 42_u64 });

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(42)
            );
        });
    }

    #[test]
    fn local_boxed_future_is_available() {
        let mut future = boxed_local(async { "Zamani" });

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready("Zamani")
            );
        });
    }

    #[test]
    fn canonical_join_preserves_input_order() {
        let futures = [
            ready(10_u64),
            ready(20_u64),
            ready(30_u64),
        ];

        let mut joined = Box::pin(join_all(futures));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(vec![10, 20, 30])
            );
        });
    }

    #[test]
    fn canonical_fallible_join_returns_errors() {
        let futures = [
            ready(Ok::<_, &'static str>(10_u64)),
            ready(Err("operation failed")),
        ];

        let mut joined = Box::pin(try_join_all(futures));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Err("operation failed"))
            );
        });
    }

    #[test]
    fn empty_join_completes_successfully() {
        let futures = std::iter::empty::<
            std::future::Ready<u64>
        >();

        let mut joined = Box::pin(join_all(futures));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Vec::new())
            );
        });
    }

    #[test]
    fn async_result_alias_accepts_fallible_future() {
        let future: AsyncResult<'static, u64, &'static str> =
            boxed(async {
                Ok(42_u64)
            });

        let _ = future;
    }

    #[test]
    fn local_async_result_alias_accepts_local_future() {
        let future: LocalAsyncResult<'static, u64, &'static str> =
            boxed_local(async {
                Ok(42_u64)
            });

        let _ = future;
    }

    #[test]
    fn future_properties_identifies_send_futures() {
        let future = async {
            42_u64
        };

        assert!(future.is_send_capable());
        assert!(future.has_known_output_type());
    }

    #[test]
    fn ready_constructor_is_available() {
        let mut future = Box::pin(ready(42_u64));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(42)
            );
        });
    }

    #[test]
    fn pending_constructor_remains_pending() {
        let mut future = Box::pin(pending::<u64>());

        with_context(|context| {
            assert!(matches!(
                future.as_mut().poll(context),
                Poll::Pending
            ));
        });
    }
}
