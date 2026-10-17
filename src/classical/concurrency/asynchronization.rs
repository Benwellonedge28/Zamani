
#![forbid(unsafe_code)]

//! Zamani asynchronous computation and future composition.
//!
//! File: `src/classical/concurrency/asynchronization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! # Contract
//!
//! **Purpose**
//!
//! Provide safe, executor-independent primitives for composing asynchronous
//! computations. This module supports boxed futures, joining heterogeneous
//! execution plans through a common future type, and fallible future joining.
//!
//! **Authority**
//!
//! - `grammar/expressions/async.g4` owns async expression syntax.
//! - `grammar/functions/async.g4` owns async function declarations and the
//!   canonical `awaitExpression` grammar rule.
//! - `grammar/concurrency/async.g4` owns the concurrency-domain composition
//!   boundary.
//! - `src/classical/concurrency/task.rs` owns task identity, task lifecycle,
//!   and cooperative task cancellation.
//! - The existing frontend AST owns parsed source structure.
//! - Semantic analysis owns awaitability, effects, ownership, and legality.
//! - The canonical IR owns the representation consumed by lowering.
//! - The executor and runtime own actual asynchronous execution.
//!
//! **Does not own**
//!
//! This module does not implement a scheduler, executor, event loop, timer
//! service, operating-system I/O reactor, distributed transport, task identity
//! allocator, or hardware backend. It does not define a second AST or IR.
//!
//! **POCO-REAF**
//!
//! A future is a computation that may complete later. It does not intrinsically
//! identify a thread, core, worker, machine, GPU, FPGA, QPU, or network node.
//! This module imposes no fixed language-level maximum on the number of futures
//! composed. Actual memory, address-space, and runtime resource limits remain
//! real and must be reported by the owning layer.
//!
//! **Scheduling and cancellation**
//!
//! Joining futures polls them when the enclosing future is polled. It does not
//! create threads or guarantee parallel execution. Cancellation remains
//! cooperative and is integrated through the task/executor contract.
//!
//! **Error handling**
//!
//! `try_join_all` returns the first error observed in polling order. Once that
//! error is returned, remaining futures are dropped. Future destruction is not
//! a guarantee that external work already submitted to a provider has stopped.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// =============================================================================
// Future aliases
// =============================================================================

/// A boxed, pinned future that may be sent between threads.
///
/// This alias is suitable for APIs that require `Send`. It does not imply that
/// a future will be scheduled on another thread.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A boxed, pinned future that is not required to implement `Send`.
///
/// Use this for local executors or computations whose captured state is not
/// safe to transfer between threads.
pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Box a future for use behind a `Send` future interface.
pub fn boxed<'a, F>(future: F) -> BoxFuture<'a, F::Output>
where
    F: Future + Send + 'a,
{
    Box::pin(future)
}

/// Box a future without requiring `Send`.
pub fn boxed_local<'a, F>(future: F) -> LocalBoxFuture<'a, F::Output>
where
    F: Future + 'a,
{
    Box::pin(future)
}

// =============================================================================
// Join all
// =============================================================================

/// A future that completes after every input future has completed.
///
/// Futures are polled in their input order on each poll of this join future.
/// A completed future is removed from the active set and is never polled again.
///
/// This type does not spawn work. Parallelism, if any, depends on the executor
/// and on the futures supplied to it.
///
/// The inner futures are individually pinned on the heap. Moving this outer
/// structure therefore moves only pin pointers, not the pinned future values.
pub struct JoinAll<F: Future> {
    futures: Vec<Option<Pin<Box<F>>>>,
    outputs: Vec<Option<F::Output>>,
    remaining: usize,
}

// SAFETY NOTE:
// No unsafe code is used or needed here. Every F remains pinned inside its own
// Box. Moving JoinAll moves the Box pointers and completed output values only;
// it never moves a pinned F out of its allocation.
impl<F: Future> Unpin for JoinAll<F> {}

impl<F: Future> JoinAll<F> {
    fn from_iter<I>(futures: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        let futures: Vec<Option<Pin<Box<F>>>> = futures
            .into_iter()
            .map(|future| Some(Box::pin(future)))
            .collect();

        let remaining = futures.len();
        let outputs = std::iter::repeat_with(|| None)
            .take(remaining)
            .collect();

        Self {
            futures,
            outputs,
            remaining,
        }
    }

    /// Number of input futures.
    pub fn len(&self) -> usize {
        self.futures.len()
    }

    /// Whether the join contains no input futures.
    pub fn is_empty(&self) -> bool {
        self.futures.is_empty()
    }

    /// Number of futures that have not completed yet.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.remaining == 0 {
            // The empty case and repeated polling after completion both return
            // an empty vector only for an empty join. A completed non-empty
            // join is consumed by its executor as a completed future.
            //
            // Polling a completed Future again is not required to return the
            // original output. Returning an empty vector here avoids retaining
            // a second copy of potentially large outputs.
            return Poll::Ready(Vec::new());
        }

        let Self {
            futures,
            outputs,
            remaining,
        } = this;

        for (index, slot) in futures.iter_mut().enumerate() {
            // Poll only active futures. Compute the Poll value before taking
            // the slot so the pin borrow ends before the slot is modified.
            let result = match slot.as_mut() {
                Some(future) => Some(future.as_mut().poll(context)),
                None => None,
            };

            if let Some(Poll::Ready(output)) = result {
                *slot = None;
                outputs[index] = Some(output);
                *remaining -= 1;
            }
        }

        if *remaining != 0 {
            return Poll::Pending;
        }

        let completed = outputs
            .iter_mut()
            .map(|output| {
                output
                    .take()
                    .expect("completed JoinAll must contain every output")
            })
            .collect();

        Poll::Ready(completed)
    }
}

/// Join all futures and return their outputs in input order.
///
/// The returned future owns its input futures. Dropping it drops futures that
/// have not completed. This does not guarantee cancellation of work that a
/// future has already submitted to an external runtime or device.
///
/// An empty input completes immediately with an empty vector.
///
/// The vector and each boxed future consume finite memory. No artificial
/// architectural maximum is imposed by this API.
pub fn join_all<I>(futures: I) -> JoinAll<I::Item>
where
    I: IntoIterator,
    I::Item: Future,
{
    JoinAll::from_iter(futures)
}

// =============================================================================
// Fallible join
// =============================================================================

/// Join fallible futures, stopping at the first observed error.
///
/// Results retain input order when all futures succeed. If an error is
/// observed, this future returns it and drops the remaining join state.
///
/// This function does not promise that the first error in wall-clock time is
/// selected: futures are polled in input order, and multiple futures may
/// already have completed before an error is observed.
pub async fn try_join_all<I, F, T, E>(
    futures: I,
) -> Result<Vec<T>, E>
where
    I: IntoIterator<Item = F>,
    F: Future<Output = Result<T, E>>,
{
    join_all(futures).await.into_iter().collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::task::{Wake, Waker};

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn context() -> Context<'static> {
        // The leaked Waker is test-only and provides a static context lifetime.
        // It is safe Rust and does not use unsafe code.
        let waker = Box::leak(Box::new(Waker::from(Arc::new(NoopWake))));
        Context::from_waker(waker)
    }

    #[test]
    fn boxed_future_returns_its_value() {
        let future = boxed(async { 42_u64 });
        let mut context = context();
        let mut future = future;

        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));
    }

    #[test]
    fn boxed_local_future_returns_its_value() {
        let mut future = boxed_local(async { "zamani" });
        let mut context = context();

        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready("zamani")
        );
    }

    #[test]
    fn empty_join_completes_immediately() {
        let mut future = Box::pin(join_all(
            std::iter::empty::<std::future::Ready<u8>>(),
        ));
        let mut context = context();

        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(vec![]));
    }

    #[test]
    fn join_preserves_input_order() {
        let futures = [
            std::future::ready(10),
            std::future::ready(20),
            std::future::ready(30),
        ];

        let mut future = Box::pin(join_all(futures));
        let mut context = context();

        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready(vec![10, 20, 30])
        );
    }

    #[test]
    fn join_tracks_length_and_remaining_count() {
        let futures = [std::future::ready(1), std::future::ready(2)];
        let mut future = Box::pin(join_all(futures));

        assert_eq!(future.as_ref().get_ref().len(), 2);
        assert_eq!(future.as_ref().get_ref().remaining(), 2);
        assert!(!future.as_ref().get_ref().is_empty());

        let mut context = context();
        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready(vec![1, 2])
        );
    }

    #[test]
    fn join_all_accepts_a_dynamically_sized_collection() {
        let count = 257_usize;
        let futures = (0..count).map(std::future::ready);

        let mut future = Box::pin(join_all(futures));
        let mut context = context();

        let Poll::Ready(values) = future.as_mut().poll(&mut context) else {
            panic!("ready futures should complete in one poll");
        };

        assert_eq!(values.len(), count);
        assert_eq!(values.first(), Some(&0));
        assert_eq!(values.last(), Some(&(count - 1)));
    }

    #[test]
    fn fallible_join_returns_all_successful_values_in_order() {
        let futures = [
            std::future::ready(Ok::<_, &'static str>(3)),
            std::future::ready(Ok::<_, &'static str>(5)),
            std::future::ready(Ok::<_, &'static str>(8)),
        ];

        let mut future = Box::pin(try_join_all(futures));
        let mut context = context();

        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready(Ok(vec![3, 5, 8]))
        );
    }

    #[test]
    fn fallible_join_propagates_an_error() {
        let futures = [
            std::future::ready(Ok::<_, &'static str>(3)),
            std::future::ready(Err("expected failure")),
            std::future::ready(Ok(8)),
        ];

        let mut future = Box::pin(try_join_all(futures));
        let mut context = context();

        assert_eq!(
            future.as_mut().poll(&mut context),
            Poll::Ready(Err("expected failure"))
        );
    }

    #[test]
    fn join_remains_pending_when_a_future_is_pending() {
        let futures = [
            std::future::ready(3),
            std::future::pending::<u8>(),
        ];

        let mut future = Box::pin(join_all(futures));
        let mut context = context();

        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(future.as_ref().get_ref().remaining(), 1);
    }
}
