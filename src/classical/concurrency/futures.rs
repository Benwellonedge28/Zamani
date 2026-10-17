
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency: executor-independent future composition.
//!
//! File: src/classical/concurrency/futures.rs
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! # Contract
//!
//! Purpose:
//! - Provide boxed future types for Send and local-only computations.
//! - Join dynamically sized collections of homogeneous futures.
//! - Preserve result ordering independently of completion ordering.
//! - Provide genuinely fail-fast composition for fallible futures.
//!
//! Authority:
//! - grammar/concurrency/futures.g4 defines the parser integration boundary.
//! - grammar/concurrency/async.g4 and grammar/concurrency/await.g4 own
//!   their respective source-language syntax.
//! - grammar/spec/concurrency.md defines the language-level concurrency
//!   contract.
//! - src/classical/concurrency/task.rs owns task identity and cancellation.
//! - The frontend AST and canonical semantic IR remain authoritative for
//!   source representation and compiler semantics.
//!
//! Owns:
//! - BoxFuture and LocalBoxFuture.
//! - Boxing helpers.
//! - JoinAll and TryJoinAll.
//! - join_all and try_join_all.
//!
//! Does not own:
//! - Source syntax, AST, semantic type checking, or canonical IR.
//! - Task IDs, task lifecycle, or cancellation-token ownership.
//! - An executor, scheduler, worker pool, event loop, or I/O reactor.
//! - Timers, network transports, device queues, or distributed execution.
//! - Resource discovery, placement, or hardware-specific lowering.
//!
//! Execution:
//! These combinators progress only when their parent future is polled.
//! They do not spawn tasks or create parallelism. A future that performs
//! blocking work can block the executor thread that polls it.
//!
//! Ordering:
//! Inputs are polled in input order on each parent poll. Results are
//! returned in input order, not completion order. For fallible joins,
//! the first Err observed in this polling order is returned immediately.
//!
//! Cancellation:
//! Dropping a combinator drops its owned, unfinished child futures.
//! This cannot guarantee that work already submitted to an external system
//! has stopped. Cancellation-token integration and wake-up policy belong
//! to the executor/runtime contract.
//!
//! Scalability:
//! No artificial language-level limit is imposed on future count.
//! Storage uses host collections and therefore remains bounded by actual
//! address space, allocator behavior, and available resources.
//!
//! Allocation:
//! Rust allocation failure may abort or panic according to the allocator
//! and build configuration. This module does not claim to convert every
//! out-of-memory condition into a recoverable error.
//!
//! Compatibility:
//! `asynchronization.rs` should re-export this module's public API rather
//! than maintain a second implementation of future composition.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ---------------------------------------------------------------------------
// Future aliases and boxing
// ---------------------------------------------------------------------------

/// A pinned future that may be transferred between threads.
///
/// `Send` does not imply that this future is spawned or polled on another
/// thread. That decision belongs to an executor.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A pinned future for an executor that does not require `Send`.
///
/// Use this when a future owns local state that cannot safely be transferred
/// between threads.
pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Box a future whose captured state is safe to transfer between threads.
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

// ---------------------------------------------------------------------------
// Join all
// ---------------------------------------------------------------------------

/// A future that resolves after all child futures resolve.
///
/// Each child is pinned in its own heap allocation. Moving this combinator
/// therefore moves the pin pointers, not the child future values.
///
/// Children are polled in input order whenever this combinator is polled.
/// Completed children are not polled again. Outputs retain input order.
///
/// This type does not spawn child futures. It relies on the enclosing
/// executor to poll it again when a child wakes its task.
pub struct JoinAll<F: Future> {
    futures: Vec<Option<Pin<Box<F>>>>,
    outputs: Vec<Option<F::Output>>,
    remaining: usize,
    completed: bool,
}

// SAFETY: no unsafe code is involved. The inner `F` values remain pinned
// inside their individual Boxes. The outer structure does not promise that
// output values are pinned; moving the outer structure moves only ordinary
// state and pin pointers.
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

        let count = futures.len();
        let outputs = std::iter::repeat_with(|| None)
            .take(count)
            .collect();

        Self {
            futures,
            outputs,
            remaining: count,
            completed: false,
        }
    }

    /// Number of futures supplied at construction.
    pub fn len(&self) -> usize {
        self.futures.len()
    }

    /// Whether no futures were supplied.
    pub fn is_empty(&self) -> bool {
        self.futures.is_empty()
    }

    /// Number of child futures that have not completed.
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

        // Polling after completion is outside the Future contract. Avoid
        // returning fabricated or duplicate output if a caller does so.
        if this.completed {
            return Poll::Pending;
        }

        for index in 0..this.futures.len() {
            let result = match this.futures[index].as_mut() {
                Some(future) => Some(future.as_mut().poll(context)),
                None => None,
            };

            if let Some(Poll::Ready(output)) = result {
                // The child is complete; never poll it again.
                this.futures[index] = None;
                this.outputs[index] = Some(output);
                this.remaining -= 1;
            }
        }

        if this.remaining != 0 {
            return Poll::Pending;
        }

        let outputs = this
            .outputs
            .iter_mut()
            .map(|slot| {
                slot.take()
                    .expect("completed JoinAll must contain every output")
            })
            .collect();

        this.completed = true;
        Poll::Ready(outputs)
    }
}

/// Join every future in an iterable collection.
///
/// The collection may be dynamically sized. There is no arbitrary future-count
/// limit; allocation and host resources still constrain actual execution.
///
/// Outputs retain input order. The returned future owns the child futures.
/// Dropping it drops unfinished children but does not guarantee cancellation
/// of external work that those children may already have submitted.
///
/// An empty collection completes immediately with an empty vector.
pub fn join_all<I>(futures: I) -> JoinAll<I::Item>
where
    I: IntoIterator,
    I::Item: Future,
{
    JoinAll::from_iter(futures)
}

// ---------------------------------------------------------------------------
// Fallible join
// ---------------------------------------------------------------------------

/// A future that joins fallible child futures and stops on the first
/// observed error.
///
/// Successful results are returned in input order if all children succeed.
/// When an error is observed, unfinished children and previously collected
/// successful outputs are dropped immediately.
///
/// "First error" means the first `Err` observed by polling children in input
/// order. It does not mean the error that occurred first in wall-clock time.
pub struct TryJoinAll<F, T, E>
where
    F: Future<Output = Result<T, E>>,
{
    futures: Vec<Option<Pin<Box<F>>>>,
    outputs: Vec<Option<T>>,
    remaining: usize,
    completed: bool,
}

impl<F, T, E> Unpin for TryJoinAll<F, T, E>
where
    F: Future<Output = Result<T, E>>,
{
}

impl<F, T, E> TryJoinAll<F, T, E>
where
    F: Future<Output = Result<T, E>>,
{
    fn from_iter<I>(futures: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        let futures: Vec<Option<Pin<Box<F>>>> = futures
            .into_iter()
            .map(|future| Some(Box::pin(future)))
            .collect();

        let count = futures.len();
        let outputs = std::iter::repeat_with(|| None)
            .take(count)
            .collect();

        Self {
            futures,
            outputs,
            remaining: count,
            completed: false,
        }
    }

    /// Number of futures supplied at construction.
    pub fn len(&self) -> usize {
        self.futures.len()
    }

    /// Whether no futures were supplied.
    pub fn is_empty(&self) -> bool {
        self.futures.is_empty()
    }

    /// Number of child futures that have not completed.
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    fn discard_children(&mut self) {
        for future in &mut self.futures {
            *future = None;
        }
    }
}

impl<F, T, E> Future for TryJoinAll<F, T, E>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<Vec<T>, E>;

    fn poll(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Polling after completion is outside the Future contract.
        if this.completed {
            return Poll::Pending;
        }

        for index in 0..this.futures.len() {
            let result = match this.futures[index].as_mut() {
                Some(future) => Some(future.as_mut().poll(context)),
                None => None,
            };

            match result {
                Some(Poll::Ready(Ok(value))) => {
                    this.futures[index] = None;
                    this.outputs[index] = Some(value);
                    this.remaining -= 1;
                }

                Some(Poll::Ready(Err(error))) => {
                    // Release all owned child futures and successful outputs
                    // before reporting the failure to the caller.
                    this.discard_children();
                    this.outputs.clear();
                    this.remaining = 0;
                    this.completed = true;

                    return Poll::Ready(Err(error));
                }

                Some(Poll::Pending) | None => {}
            }
        }

        if this.remaining != 0 {
            return Poll::Pending;
        }

        let outputs = this
            .outputs
            .iter_mut()
            .map(|slot| {
                slot.take()
                    .expect("successful TryJoinAll must contain every output")
            })
            .collect();

        this.completed = true;
        Poll::Ready(Ok(outputs))
    }
}

/// Join fallible futures, returning the first error observed in polling order.
///
/// Unlike implementing this by joining `Result` values and collecting them
/// afterwards, this combinator returns immediately when a child produces an
/// error. It drops the remaining child futures and accumulated outputs.
///
/// Successful values retain input order. An empty collection returns
/// `Ok(Vec::new())`.
pub fn try_join_all<I, F, T, E>(futures: I) -> TryJoinAll<F, T, E>
where
    I: IntoIterator<Item = F>,
    F: Future<Output = Result<T, E>>,
{
    TryJoinAll::from_iter(futures)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::{pending, poll_fn, ready};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::task::{Wake, Waker};

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn with_context<R>(f: impl FnOnce(&mut Context<'_>) -> R) -> R {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        f(&mut context)
    }

    #[test]
    fn boxed_send_future_returns_value() {
        let mut future = boxed(async { 42_u64 });

        with_context(|context| {
            assert_eq!(future.as_mut().poll(context), Poll::Ready(42));
        });
    }

    #[test]
    fn boxed_local_future_returns_value() {
        let mut future = boxed_local(async { "zamani" });

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready("zamani")
            );
        });
    }

    #[test]
    fn empty_join_completes_with_empty_vector() {
        let mut future = Box::pin(join_all(std::iter::empty::<
            std::future::Ready<u8>,
        >()));

        with_context(|context| {
            assert_eq!(future.as_mut().poll(context), Poll::Ready(vec![]));
        });
    }

    #[test]
    fn join_preserves_input_order() {
        let futures = [ready(10), ready(20), ready(30)];
        let mut future = Box::pin(join_all(futures));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(vec![10, 20, 30])
            );
        });
    }

    #[test]
    fn join_supports_dynamic_input_size() {
        let count = 257_usize;
        let futures = (0..count).map(ready);
        let mut future = Box::pin(join_all(futures));

        with_context(|context| {
            let Poll::Ready(values) = future.as_mut().poll(context) else {
                panic!("ready futures should complete in one poll");
            };

            assert_eq!(values.len(), count);
            assert_eq!(values.first(), Some(&0));
            assert_eq!(values.last(), Some(&(count - 1)));
        });
    }

    #[test]
    fn join_tracks_remaining_futures() {
        let futures = [ready(1), pending::<u8>()];
        let mut future = Box::pin(join_all(futures));

        assert_eq!(future.as_ref().get_ref().len(), 2);
        assert_eq!(future.as_ref().get_ref().remaining(), 2);

        with_context(|context| {
            assert_eq!(future.as_mut().poll(context), Poll::Pending);
        });

        assert_eq!(future.as_ref().get_ref().remaining(), 1);
    }

    #[test]
    fn successful_fallible_join_preserves_input_order() {
        let futures = [
            ready(Ok::<_, &'static str>(3)),
            ready(Ok(5)),
            ready(Ok(8)),
        ];
        let mut future = Box::pin(try_join_all(futures));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(Ok(vec![3, 5, 8]))
            );
        });
    }

    #[test]
    fn fallible_join_returns_first_error_observed() {
        let futures = [
            ready(Ok::<_, &'static str>(3)),
            ready(Err("expected failure")),
            ready(Ok(8)),
        ];
        let mut future = Box::pin(try_join_all(futures));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(Err("expected failure"))
            );
        });
    }

    #[test]
    fn fallible_join_does_not_poll_children_after_observed_error() {
        let later_poll_count = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&later_poll_count);

        let later = poll_fn(move |_| {
            observed.fetch_add(1, Ordering::SeqCst);
            Poll::Ready(Ok::<_, &'static str>(2_u8))
        });

        let futures = [ready(Err::<u8, _>("stop")), later];
        let mut future = Box::pin(try_join_all(futures));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(Err("stop"))
            );
        });

        assert_eq!(later_poll_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn fallible_join_empty_input_succeeds() {
        let mut future = Box::pin(try_join_all(
            std::iter::empty::<std::future::Ready<Result<u8, &'static str>>>(),
        ));

        with_context(|context| {
            assert_eq!(
                future.as_mut().poll(context),
                Poll::Ready(Ok(Vec::new()))
            );
        });
    }

    #[test]
    fn pending_child_keeps_join_pending() {
        let futures = [ready(3), pending::<u8>()];
        let mut future = Box::pin(join_all(futures));

        with_context(|context| {
            assert_eq!(future.as_mut().poll(context), Poll::Pending);
        });
    }

    #[test]
    fn dropping_join_drops_owned_child_future() {
        struct DropProbe(Arc<AtomicUsize>);

        impl Future for DropProbe {
            type Output = ();

            fn poll(
                self: Pin<&mut Self>,
                _context: &mut Context<'_>,
            ) -> Poll<Self::Output> {
                Poll::Pending
            }
        }

        impl Drop for DropProbe {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let dropped = Arc::new(AtomicUsize::new(0));
        let probe = DropProbe(Arc::clone(&dropped));
        let joined = join_all([probe]);

        drop(joined);

        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }
}
