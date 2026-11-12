
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Asynchronous Operations
//!
//! Path: src/zur/concurrency/async.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides the ZUR-facing asynchronous computation API.
//!
//! This module integrates ZUR with the canonical, executor-independent
//! future-composition implementation in:
//!
//!     crate::classical::concurrency::futures
//!
//! It exposes:
//!
//! - Send asynchronous operations.
//! - Local-only asynchronous operations.
//! - Fallible asynchronous operations.
//! - Standard Rust Future interoperability.
//! - Future boxing.
//! - Dynamic future composition.
//! - Ordered future joining.
//! - Fail-fast fallible future joining.
//! - Standard future constructors.
//!
//! # Architectural ownership
//!
//! This module owns the ZUR asynchronous API facade.
//!
//! It does NOT own:
//!
//! - Source-language async syntax.
//! - Source-language await syntax.
//! - Frontend AST.
//! - Semantic validation.
//! - Canonical intermediate representations.
//! - Task identity.
//! - Task lifecycle.
//! - Task completion storage.
//! - Task cancellation ownership.
//! - Future polling infrastructure.
//! - Executors.
//! - Worker pools.
//! - Thread creation.
//! - Scheduling.
//! - Work stealing.
//! - Timers.
//! - I/O reactors.
//! - Network transports.
//! - Distributed execution.
//! - Hardware discovery.
//! - Physical resource allocation.
//!
//! Those responsibilities belong to their existing authoritative
//! repository subsystems.
//!
//! # Canonical authorities
//!
//! Future composition:
//!
//!     crate::classical::concurrency::futures
//!
//! Task lifecycle:
//!
//!     crate::zur::concurrency::task
//!
//! Task execution dispatch:
//!
//!     crate::zur::concurrency::executor
//!
//! Cancellation:
//!
//!     crate::zur::execution::cancellation
//!
//! Runtime errors:
//!
//!     crate::zur::error
//!
//! Runtime identities:
//!
//!     crate::zur::ids
//!
//! Source syntax:
//!
//!     grammar/functions/async.g4
//!     grammar/concurrency/async.g4
//!     grammar/concurrency/await.g4
//!     grammar/concurrency/futures.g4
//!     grammar/concurrency/tasks.g4
//!
//! Normative concurrency semantics:
//!
//!     grammar/spec/concurrency.md
//!
//! # Fundamental execution distinction
//!
//! A Future represents a computation whose result may become available
//! later.
//!
//! A Future does not identify:
//!
//! - A thread.
//! - A worker.
//! - A CPU.
//! - A GPU.
//! - An FPGA.
//! - A QPU.
//! - A distributed node.
//! - A hardware device.
//! - A physical execution resource.
//!
//! Future composition does not automatically create parallel execution.
//!
//! A future progresses when an execution authority polls it.
//!
//! That authority may be supplied by an appropriate runtime, executor,
//! platform integration or external execution provider.
//!
//! # Cancellation
//!
//! Dropping a future drops its owned unfinished future state.
//!
//! This does not guarantee that external work already submitted to a
//! device, remote service or execution provider has stopped.
//!
//! Cancellation of runtime-managed tasks remains owned by the canonical
//! ZUR task and cancellation implementations.
//!
//! This module does not introduce another cancellation-token system.
//!
//! # Error handling
//!
//! Fallible operations preserve their original error types.
//!
//! This module does not convert failures into successful results.
//!
//! Runtime-owned failures should use crate::zur::error::ZurError.
//!
//! # Scalability and POCO-REAF
//!
//! No arbitrary maximum is imposed on:
//!
//! - Asynchronous operations.
//! - Futures.
//! - Future collections.
//! - Nested asynchronous computations.
//! - Computational domains.
//! - Devices.
//! - Execution providers.
//! - Distributed participants.
//!
//! Dynamic collections grow according to available memory and allocator
//! capacity.
//!
//! Actual execution remains constrained by:
//!
//! - Available memory.
//! - Address-space capacity.
//! - Executor capacity.
//! - Provider capabilities.
//! - Operational budgets.
//! - Program requirements.
//! - Target compatibility.
//!
//! No finite implementation promises literal infinite computation.
//!
//! # Blocking operations
//!
//! Futures should not perform unbounded blocking operations on an executor
//! thread unless the supplied executor explicitly supports that behavior.
//!
//! This module does not silently create blocking threads or worker pools.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No raw-pointer ownership.
//! No global mutable state.
//! No implicit execution.
//! No fabricated completion.
//!
//! # Integration rule
//!
//! This module must remain a facade over the canonical classical future
//! implementation. It must not independently implement Future
//! combinators or executor infrastructure.

use core::future::Future;

// ============================================================================
// STANDARD FUTURE INTEROPERABILITY
// ============================================================================

/// The standard Rust Future trait.
///
/// Zamani uses the standard Future contract instead of introducing
/// a competing asynchronous computation trait.
///
/// The execution authority determines when and where a future is polled.
pub use core::future::Future as AsyncFuture;

/// Standard future polling context.
pub use core::task::Context;

/// Standard future polling result.
pub use core::task::Poll;

/// Standard future wake-up interface.
pub use core::task::Waker;

// ============================================================================
// CANONICAL FUTURE IMPLEMENTATION
// ============================================================================

/// Canonical future-composition types and operations.
///
/// Every item is re-exported from the existing classical concurrency
/// implementation.
///
/// No second implementation is introduced.
///
/// `join_all` and `try_join_all` compose futures but do not spawn
/// independent tasks or guarantee parallel execution.
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
// STANDARD FUTURE CONSTRUCTORS
// ============================================================================

/// Creates a future that remains pending indefinitely.
///
/// The future becomes ready only if its implementation is replaced
/// or otherwise controlled by an external composition mechanism.
///
/// This function does not create an event source, timer or executor.
pub use core::future::pending;

/// Creates a future that immediately resolves with a supplied value.
pub use core::future::ready;

/// Creates a future from a polling function.
///
/// The caller is responsible for implementing correct polling
/// and wake-up behavior.
pub use core::future::poll_fn;

// ============================================================================
// ASYNCHRONOUS OPERATION TYPES
// ============================================================================

/// A boxed asynchronous operation that can safely be transferred
/// between threads.
///
/// `Send` is a Rust ownership property. It does not imply that the
/// operation is spawned, executed concurrently or moved between workers.
///
/// The lifetime parameter describes how long the operation may borrow
/// external data.
pub type AsyncOperation<'a, T> = BoxFuture<'a, T>;

/// A boxed asynchronous operation that does not require `Send`.
///
/// This supports local execution environments and futures that own
/// thread-local or otherwise non-Send state.
///
/// Such operations must remain within an execution context that
/// supports their ownership requirements.
pub type LocalAsyncOperation<'a, T> = LocalBoxFuture<'a, T>;

// ============================================================================
// FALLIBLE ASYNCHRONOUS OPERATIONS
// ============================================================================

/// A Send asynchronous operation returning a fallible result.
///
/// The error type belongs to the operation's authoritative subsystem.
///
/// For ZUR-owned operational failures, use `crate::zur::error::ZurError`.
pub type AsyncResult<'a, T, E> =
    AsyncOperation<'a, Result<T, E>>;

/// A local asynchronous operation returning a fallible result.
///
/// This type does not require the future to implement Send.
pub type LocalAsyncResult<'a, T, E> =
    LocalAsyncOperation<'a, Result<T, E>>;

// ============================================================================
// VALUE-RETURNING OPERATIONS
// ============================================================================

/// A Send asynchronous operation returning a value.
///
/// This is an alternative descriptive name for AsyncOperation.
pub type AsyncValue<'a, T> = AsyncOperation<'a, T>;

/// A local asynchronous operation returning a value.
pub type LocalAsyncValue<'a, T> = LocalAsyncOperation<'a, T>;

// ============================================================================
// JOIN COMPOSITION CONTRACTS
// ============================================================================

/// Joins a dynamically sized collection of futures.
///
/// This is a public alias for the canonical JoinAll implementation.
///
/// Properties:
///
/// - The returned future owns its child futures.
/// - Results preserve input order.
/// - Completed child futures are not polled again.
/// - Empty input completes with an empty vector.
/// - Unfinished children are dropped when the join is dropped.
///
/// The parent execution authority remains responsible for polling
/// and wake-up handling.
pub type AsyncJoin<F> = JoinAll<F>;

/// Joins fallible asynchronous operations.
///
/// This is a public alias for the canonical TryJoinAll implementation.
///
/// Properties:
///
/// - Successful results preserve input order.
/// - The first error observed in polling order is returned immediately.
/// - Remaining child futures are dropped after an observed error.
/// - Empty input completes successfully with an empty vector.
///
/// The first observed error is not necessarily the error that occurred
/// first in wall-clock time.
pub type AsyncTryJoin<F, T, E> = TryJoinAll<F, T, E>;

// ============================================================================
// MODULE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::future::Future;
    use core::task::{Context, Poll, Wake, Waker};

    use std::rc::Rc;
    use std::sync::Arc;

    // ------------------------------------------------------------------------
    // Test wake implementation
    // ------------------------------------------------------------------------

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn with_context<R>(
        operation: impl FnOnce(&mut Context<'_>) -> R,
    ) -> R {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);

        operation(&mut context)
    }

    // ------------------------------------------------------------------------
    // Send asynchronous operations
    // ------------------------------------------------------------------------

    #[test]
    fn send_async_operation_accepts_standard_future() {
        let operation: AsyncOperation<'static, u64> =
            boxed(async {
                42_u64
            });

        let mut operation = operation;

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(42)
            );
        });
    }

    #[test]
    fn async_value_alias_uses_canonical_future() {
        let operation: AsyncValue<'static, u64> =
            boxed(async {
                100_u64
            });

        let _ = operation;
    }

    // ------------------------------------------------------------------------
    // Local asynchronous operations
    // ------------------------------------------------------------------------

    #[test]
    fn local_async_operation_accepts_non_send_state() {
        let value = Rc::new(42_u64);

        let operation: LocalAsyncOperation<'static, u64> =
            boxed_local(async move {
                *value
            });

        let mut operation = operation;

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(42)
            );
        });
    }

    #[test]
    fn local_async_value_alias_is_available() {
        let operation: LocalAsyncValue<'static, u64> =
            boxed_local(async {
                64_u64
            });

        let _ = operation;
    }

    // ------------------------------------------------------------------------
    // Fallible operations
    // ------------------------------------------------------------------------

    #[test]
    fn fallible_async_operation_preserves_success() {
        let operation: AsyncResult<
            'static,
            u64,
            &'static str,
        > = boxed(async {
            Ok(42_u64)
        });

        let mut operation = operation;

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(Ok(42))
            );
        });
    }

    #[test]
    fn fallible_async_operation_preserves_failure() {
        let operation: AsyncResult<
            'static,
            u64,
            &'static str,
        > = boxed(async {
            Err("execution failed")
        });

        let mut operation = operation;

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(Err("execution failed"))
            );
        });
    }

    #[test]
    fn local_fallible_operation_accepts_non_send_state() {
        let value = Rc::new(42_u64);

        let operation: LocalAsyncResult<
            'static,
            u64,
            &'static str,
        > = boxed_local(async move {
            Ok(*value)
        });

        let mut operation = operation;

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(Ok(42))
            );
        });
    }

    // ------------------------------------------------------------------------
    // Ordered joining
    // ------------------------------------------------------------------------

    #[test]
    fn async_join_preserves_input_order() {
        let operations = [
            ready(30_u64),
            ready(10_u64),
            ready(20_u64),
        ];

        let mut joined = Box::pin(join_all(operations));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(vec![30, 10, 20])
            );
        });
    }

    #[test]
    fn async_join_supports_dynamic_collections() {
        let count = 257_usize;

        let operations = (0..count).map(ready);

        let mut joined = Box::pin(join_all(operations));

        with_context(|context| {
            let Poll::Ready(values) =
                joined.as_mut().poll(context)
            else {
                panic!("ready operations should complete");
            };

            assert_eq!(values.len(), count);
            assert_eq!(values.first(), Some(&0));
            assert_eq!(values.last(), Some(&(count - 1)));
        });
    }

    #[test]
    fn empty_async_join_completes_successfully() {
        let operations =
            std::iter::empty::<std::future::Ready<u64>>();

        let mut joined = Box::pin(join_all(operations));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Vec::new())
            );
        });
    }

    // ------------------------------------------------------------------------
    // Fail-fast joining
    // ------------------------------------------------------------------------

    #[test]
    fn fallible_async_join_preserves_success_order() {
        let operations = [
            ready(Ok::<_, &'static str>(10_u64)),
            ready(Ok(20_u64)),
            ready(Ok(30_u64)),
        ];

        let mut joined = Box::pin(try_join_all(operations));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Ok(vec![10, 20, 30]))
            );
        });
    }

    #[test]
    fn fallible_async_join_propagates_errors() {
        let operations = [
            ready(Ok::<_, &'static str>(10_u64)),
            ready(Err("expected failure")),
            ready(Ok(30_u64)),
        ];

        let mut joined = Box::pin(try_join_all(operations));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Err("expected failure"))
            );
        });
    }

    #[test]
    fn fallible_async_join_handles_empty_input() {
        let operations = std::iter::empty::<
            std::future::Ready<Result<u64, &'static str>>
        >();

        let mut joined = Box::pin(try_join_all(operations));

        with_context(|context| {
            assert_eq!(
                joined.as_mut().poll(context),
                Poll::Ready(Ok(Vec::new()))
            );
        });
    }

    // ------------------------------------------------------------------------
    // Standard future constructors
    // ------------------------------------------------------------------------

    #[test]
    fn ready_future_constructor_is_available() {
        let mut operation = Box::pin(ready(42_u64));

        with_context(|context| {
            assert_eq!(
                operation.as_mut().poll(context),
                Poll::Ready(42)
            );
        });
    }

    #[test]
    fn pending_future_constructor_remains_pending() {
        let mut operation = Box::pin(pending::<u64>());

        with_context(|context| {
            assert!(matches!(
                operation.as_mut().poll(context),
                Poll::Pending
            ));
        });
    }

    // ------------------------------------------------------------------------
    // Public type compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn async_join_alias_matches_canonical_join() {
        use std::any::type_name;

        assert_eq!(
            type_name::<AsyncJoin<std::future::Ready<u64>>>(),
            type_name::<JoinAll<std::future::Ready<u64>>>()
        );
    }

    #[test]
    fn async_try_join_alias_matches_canonical_join() {
        use std::any::type_name;

        assert_eq!(
            type_name::<
                AsyncTryJoin<
                    std::future::Ready<Result<u64, &'static str>>,
                    u64,
                    &'static str,
                >
            >(),
            type_name::<
                TryJoinAll<
                    std::future::Ready<Result<u64, &'static str>>,
                    u64,
                    &'static str,
                >
            >()
        );
    }
}
