
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Cooperative Cancellation
//!
//! Path: `src/stdlib/concurrency/cancellation.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides portable, cooperative cancellation primitives for
//! standard-library operations and logical asynchronous computations.
//!
//! The implementation supports:
//!
//! - cloneable cancellation tokens;
//! - independently cancellable child tokens;
//! - parent-linked cancellation;
//! - cancellation reasons;
//! - idempotent cancellation requests;
//! - cancellation-aware error checking;
//! - asynchronous cancellation notification;
//! - wake-up propagation to registered asynchronous operations;
//! - integration with OperationContext;
//! - integration with runtime-owned task execution.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns the standard-library cancellation-token contract.
//!
//! It does NOT own:
//!
//! - task scheduling;
//! - task execution;
//! - thread creation;
//! - executor implementation;
//! - process termination;
//! - device interruption;
//! - QPU execution control;
//! - FPGA execution control;
//! - hardware allocation;
//! - distributed placement;
//! - network cancellation protocols;
//! - remote-worker termination;
//! - execution deadlines;
//! - recovery or retry policy;
//! - cancellation acknowledgement from physical providers.
//!
//! Those responsibilities remain with their respective runtime,
//! distributed-execution, HAL and provider authorities.
//!
//! ## CANCELLATION SEMANTICS
//!
//! Cancellation is cooperative.
//!
//! A cancellation token records a request. An operation must observe
//! that request and decide how to stop safely.
//!
//! A cancellation request does not imply that execution has stopped.
//!
//! Cancellation acknowledgement must be reported by the authority
//! responsible for the actual execution.
//!
//! ## PARENT-CHILD SEMANTICS
//!
//! A child token:
//!
//! - observes cancellation of its parent;
//! - may be cancelled independently;
//! - cannot cancel its parent;
//! - preserves its own reason when locally cancelled;
//! - otherwise observes its parent's reason.
//!
//! Parent relationships are directional.
//!
//! ## ASYNCHRONOUS SEMANTICS
//!
//! CancellationFuture resolves when cancellation is observed.
//!
//! It registers the current task waker with the token and its parent
//! cancellation source.
//!
//! Registration is race-safe: cancellation occurring before, during
//! or immediately after registration cannot be silently lost when
//! the signal implementation supports waker registration.
//!
//! Registered wakers are removed when the future completes or is
//! dropped.
//!
//! ## INTEGRATION
//!
//! - context.rs provides CancellationSignal.
//! - tasks.rs uses CancellationToken instead of a private token.
//! - asynchronous.rs may await CancellationFuture.
//! - runtime owns scheduling and cancellation acknowledgement.
//! - distributed execution retains its own deadline and transport
//!   semantics.
//!
//! ## SCALABILITY
//!
//! This module imposes no arbitrary maximum on:
//!
//! - cancellation tokens;
//! - child tokens;
//! - parent-child relationships;
//! - registered asynchronous waiters;
//! - tasks;
//! - execution domains;
//! - devices;
//! - processors;
//! - accelerators;
//! - qubits;
//! - distributed participants.
//!
//! Actual allocations remain subject to available memory and
//! platform representational limits.
//!
//! This is not a claim of literally infinite physical resources.
//!
//! ## SECURITY
//!
//! Cancellation reasons are diagnostic text.
//!
//! Do not include credentials, access tokens, private keys,
//! passwords or other sensitive material in cancellation reasons.
//!
//! Cancellation is not an authorization or security boundary.
//!
//! ## PANIC POLICY
//!
//! Ordinary cancellation operations do not panic when a mutex
//! has been poisoned.
//!
//! Poisoned synchronization state is recovered because cancellation
//! notification must remain available after unrelated panics.
//!
//! Waker implementations are external code. As with normal Future
//! implementations, a panic inside an external Waker is not swallowed.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant grammar authorities include:
//!
//! - grammar/concurrency/concurrency.g4
//! - grammar/concurrency/tasks.g4
//! - grammar/concurrency/async.g4
//! - grammar/expressions/async.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! These grammars describe source-language constructs and execution
//! intent. This file does not define or parse source-language syntax.
//!
//! The existing grammar deliberately does not introduce a CANCEL
//! keyword merely to expose this Rust implementation.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! 1. context.rs exposes the waker registration extension.
//! 2. concurrency/mod.rs exports this module.
//! 3. stdlib/mod.rs exports concurrency.
//! 4. tasks.rs uses CancellationToken.
//! 5. Parent cancellation wakes child cancellation futures.
//! 6. Dropped cancellation futures unregister their wakers.
//! 7. Cancellation reasons remain stable after the first request.
//! 8. Cancellation errors use stdlib/error.rs.
//! 9. Tests pass on the supported Rust baseline.
//! 10. Formatting, compilation and Clippy pass.

use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::stdlib::context::CancellationSignal;
use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// INTERNAL SYNCHRONIZATION
// ============================================================================

/// Locks a mutex while recovering its contents if it was poisoned.
///
/// Cancellation is a coordination mechanism. Recovering the protected
/// state is preferable to making cancellation unavailable because
/// unrelated code previously panicked while holding a lock.
fn lock_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

// ============================================================================
// SHARED CANCELLATION STATE
// ============================================================================

/// Shared state for one cancellation source.
#[derive(Default)]
struct CancellationState {
    /// Whether cancellation was requested locally.
    requested: AtomicBool,

    /// The first locally established cancellation reason.
    reason: Mutex<Option<String>>,

    /// Asynchronous operations waiting for cancellation.
    waiters: Mutex<Vec<Waker>>,
}

impl CancellationState {
    /// Creates an uncancelled state.
    fn new() -> Self {
        Self::default()
    }

    /// Registers a waiter without duplicating an equivalent waker.
    fn register_waker(&self, waker: &Waker) {
        let mut waiters = lock_recover(&self.waiters);

        if !waiters.iter().any(|existing| existing.will_wake(waker)) {
            waiters.push(waker.clone());
        }
    }

    /// Removes a previously registered waiter.
    fn unregister_waker(&self, waker: &Waker) {
        let mut waiters = lock_recover(&self.waiters);

        waiters.retain(|existing| !existing.will_wake(waker));
    }

    /// Wakes all registered waiters.
    ///
    /// Wakers are invoked after releasing the mutex to avoid executing
    /// external executor code while holding internal synchronization.
    fn wake_waiters(&self) {
        let waiters = {
            let mut registered = lock_recover(&self.waiters);
            core::mem::take(&mut *registered)
        };

        for waiter in waiters {
            waiter.wake();
        }
    }

    /// Returns the local cancellation reason.
    fn local_reason(&self) -> Option<String> {
        if !self.requested.load(Ordering::Acquire) {
            return None;
        }

        lock_recover(&self.reason).clone()
    }

    /// Establishes local cancellation exactly once.
    fn request(&self, reason: String) -> bool {
        let mut stored_reason = lock_recover(&self.reason);

        if self.requested.load(Ordering::Acquire) {
            return false;
        }

        *stored_reason = Some(reason);

        // Publish the reason before publishing the cancellation flag.
        self.requested.store(true, Ordering::Release);

        drop(stored_reason);

        self.wake_waiters();

        true
    }
}

// ============================================================================
// CANCELLATION TOKEN
// ============================================================================

/// Cloneable, thread-safe cooperative cancellation token.
///
/// Clones share the same local cancellation state.
///
/// A child token can observe an external parent cancellation signal
/// without acquiring the ability to cancel that parent.
///
/// Cancellation tokens are process-local coordination objects.
/// They are not distributed messages, durable records, consensus
/// objects or execution acknowledgements.
#[derive(Clone)]
pub struct CancellationToken {
    /// Shared local cancellation state.
    state: Arc<CancellationState>,

    /// Optional parent cancellation source.
    parent: Option<Arc<dyn CancellationSignal>>,
}

impl CancellationToken {
    /// Creates a new independent cancellation token.
    ///
    /// The token starts in the uncancelled state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            parent: None,
        }
    }

    /// Creates a child token linked to a parent cancellation signal.
    ///
    /// The child observes parent cancellation and can also be
    /// cancelled independently.
    ///
    /// Cancelling this child never cancels the parent.
    ///
    /// Parent implementations that support asynchronous notification
    /// should implement the waker registration methods on
    /// CancellationSignal.
    #[must_use]
    pub fn linked_to(parent: Arc<dyn CancellationSignal>) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            parent: Some(parent),
        }
    }

    /// Creates a child token linked to this token.
    ///
    /// The returned token has independent local cancellation state
    /// but observes cancellation requested through this token.
    #[must_use]
    pub fn child_token(&self) -> Self {
        Self::linked_to(Arc::new(self.clone()))
    }

    /// Requests cancellation using a general descriptive reason.
    ///
    /// Returns `true` only when this call establishes local cancellation.
    ///
    /// Returns `false` when cancellation was already requested locally
    /// or has already been observed through the parent.
    ///
    /// The first successful local request establishes the reason.
    /// Later requests never overwrite it.
    ///
    /// An empty reason is permitted.
    ///
    /// Reasons must not contain secrets or credentials.
    pub fn cancel(&self) -> bool {
        self.cancel_with_reason("cancellation requested")
    }

    /// Requests cancellation with a descriptive reason.
    ///
    /// The reason is diagnostic information, not a machine-readable
    /// control-flow value.
    ///
    /// The first successful local request wins.
    ///
    /// Returns `false` if cancellation has already been requested
    /// locally or inherited from the parent.
    pub fn cancel_with_reason(
        &self,
        reason: impl Into<String>,
    ) -> bool {
        if self.is_cancelled() {
            return false;
        }

        self.state.request(reason.into())
    }

    /// Returns whether cancellation has been requested locally
    /// or by the linked parent.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.requested.load(Ordering::Acquire)
            || self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.is_cancelled())
    }

    /// Returns the local cancellation reason or inherited parent reason.
    ///
    /// A local reason takes precedence over a parent reason.
    #[must_use]
    pub fn reason(&self) -> Option<String> {
        self.state.local_reason().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.cancellation_reason())
        })
    }

    /// Returns whether this token has a parent cancellation source.
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Returns a future that resolves when cancellation is observed.
    ///
    /// The future registers its waker with this token and its parent
    /// cancellation source.
    ///
    /// Dropping the future unregisters its waker.
    #[must_use]
    pub fn cancelled(&self) -> CancellationFuture {
        CancellationFuture {
            token: self.clone(),
            registered_waker: None,
        }
    }

    /// Checks cancellation and returns a structured standard-library error.
    ///
    /// This checks cancellation only. It does not check deadlines,
    /// permissions, capabilities, resource budgets or feasibility.
    ///
    /// # Errors
    ///
    /// Returns `ErrorKind::Cancelled` when cancellation has been requested.
    pub fn check_cancelled(&self) -> Result<()> {
        if self.is_cancelled() {
            return Err(
                Error::new(
                    ErrorKind::Cancelled,
                    "operation cancellation was requested",
                )
                .in_component("stdlib.concurrency.cancellation")
                .during_operation("check_cancelled"),
            );
        }

        Ok(())
    }

    /// Registers a waker with this token and its parent chain.
    fn register_waker_chain(&self, waker: &Waker) {
        self.state.register_waker(waker);

        if let Some(parent) = &self.parent {
            parent.register_waker(waker);
        }
    }

    /// Unregisters a waker from this token and its parent chain.
    fn unregister_waker_chain(&self, waker: &Waker) {
        self.state.unregister_waker(waker);

        if let Some(parent) = &self.parent {
            parent.unregister_waker(waker);
        }
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CancellationToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CancellationToken")
            .field("cancelled", &self.is_cancelled())
            .field("has_parent", &self.has_parent())
            .field("reason", &self.reason())
            .finish()
    }
}

impl CancellationSignal for CancellationToken {
    fn is_cancelled(&self) -> bool {
        CancellationToken::is_cancelled(self)
    }

    fn cancellation_reason(&self) -> Option<String> {
        self.reason()
    }

    fn register_waker(&self, waker: &Waker) {
        self.register_waker_chain(waker);
    }

    fn unregister_waker(&self, waker: &Waker) {
        self.unregister_waker_chain(waker);
    }
}

// ============================================================================
// ASYNCHRONOUS CANCELLATION FUTURE
// ============================================================================

/// Future that resolves when its cancellation token is cancelled.
///
/// The future does not execute, schedule or terminate an operation.
///
/// Its purpose is to notify an asynchronous operation that it should
/// observe and handle a cancellation request.
#[must_use = "cancellation futures do nothing unless polled"]
pub struct CancellationFuture {
    /// Shared cancellation token.
    token: CancellationToken,

    /// Waker currently registered by this future.
    registered_waker: Option<Waker>,
}

impl Future for CancellationFuture {
    type Output = ();

    fn poll(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.token.is_cancelled() {
            this.unregister_current_waker();
            return Poll::Ready(());
        }

        let current_waker = context.waker();

        let needs_registration = this
            .registered_waker
            .as_ref()
            .is_none_or(|existing| !existing.will_wake(current_waker));

        if needs_registration {
            this.unregister_current_waker();

            this.token.register_waker_chain(current_waker);

            this.registered_waker = Some(current_waker.clone());
        }

        // Recheck after registration to close the race between the
        // initial cancellation check and waker registration.
        if this.token.is_cancelled() {
            this.unregister_current_waker();
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl CancellationFuture {
    /// Unregisters the currently registered waker.
    fn unregister_current_waker(&mut self) {
        if let Some(waker) = self.registered_waker.take() {
            self.token.unregister_waker_chain(&waker);
        }
    }
}

impl Drop for CancellationFuture {
    fn drop(&mut self) {
        self.unregister_current_waker();
    }
}

impl fmt::Debug for CancellationFuture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CancellationFuture")
            .field("cancelled", &self.token.is_cancelled())
            .field("registered", &self.registered_waker.is_some())
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::task::Wake;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Waker;

    struct CountingWake {
        count: AtomicUsize,
    }

    impl CountingWake {
        fn new() -> Self {
            Self {
                count: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize {
            self.count.load(Ordering::Acquire)
        }
    }

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.count.fetch_add(1, Ordering::AcqRel);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.count.fetch_add(1, Ordering::AcqRel);
        }
    }

    fn test_waker() -> (Arc<CountingWake>, Waker) {
        let counter = Arc::new(CountingWake::new());
        let waker = Waker::from(Arc::clone(&counter));

        (counter, waker)
    }

    #[test]
    fn new_token_starts_uncancelled() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());
        assert_eq!(token.reason(), None);
        assert!(!token.has_parent());
    }

    #[test]
    fn clones_share_cancellation_state() {
        let token = CancellationToken::new();
        let clone = token.clone();

        assert!(token.cancel());

        assert!(clone.is_cancelled());
        assert!(token.is_cancelled());
    }

    #[test]
    fn cancellation_is_idempotent() {
        let token = CancellationToken::new();

        assert!(token.cancel());
        assert!(!token.cancel());

        assert!(token.is_cancelled());
    }

    #[test]
    fn first_reason_is_preserved() {
        let token = CancellationToken::new();

        assert!(token.cancel_with_reason("requested by caller"));
        assert!(!token.cancel_with_reason("replacement reason"));

        assert_eq!(
            token.reason().as_deref(),
            Some("requested by caller")
        );
    }

    #[test]
    fn child_observes_parent_cancellation() {
        let parent = CancellationToken::new();
        let child = parent.child_token();

        assert!(!child.is_cancelled());

        assert!(parent.cancel_with_reason("parent stopped"));

        assert!(child.is_cancelled());
        assert_eq!(
            child.reason().as_deref(),
            Some("parent stopped")
        );
    }

    #[test]
    fn child_cancellation_does_not_cancel_parent() {
        let parent = CancellationToken::new();
        let child = parent.child_token();

        assert!(child.cancel_with_reason("child stopped"));

        assert!(child.is_cancelled());
        assert!(!parent.is_cancelled());

        assert_eq!(
            child.reason().as_deref(),
            Some("child stopped")
        );
    }

    #[test]
    fn locally_cancelled_child_keeps_its_own_reason() {
        let parent = CancellationToken::new();
        let child = parent.child_token();

        assert!(child.cancel_with_reason("local reason"));
        assert!(parent.cancel_with_reason("parent reason"));

        assert_eq!(
            child.reason().as_deref(),
            Some("local reason")
        );
    }

    #[test]
    fn cancellation_check_returns_structured_error() {
        let token = CancellationToken::new();

        assert!(token.check_cancelled().is_ok());

        token.cancel();

        let error = token
            .check_cancelled()
            .expect_err("cancelled token must return an error");

        assert_eq!(error.kind(), ErrorKind::Cancelled);
    }

    #[test]
    fn cancellation_future_is_pending_before_cancellation() {
        let token = CancellationToken::new();
        let mut future = Box::pin(token.cancelled());

        let (_, waker) = test_waker();
        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Pending
        ));
    }

    #[test]
    fn cancellation_wakes_registered_future() {
        let token = CancellationToken::new();
        let mut future = Box::pin(token.cancelled());

        let (counter, waker) = test_waker();
        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Pending
        ));

        assert_eq!(counter.count(), 0);

        assert!(token.cancel());

        assert!(counter.count() > 0);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Ready(())
        ));
    }

    #[test]
    fn parent_cancellation_wakes_child_future() {
        let parent = CancellationToken::new();
        let child = parent.child_token();

        let mut future = Box::pin(child.cancelled());

        let (counter, waker) = test_waker();
        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Pending
        ));

        assert!(parent.cancel());

        assert!(counter.count() > 0);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Ready(())
        ));
    }

    #[test]
    fn already_cancelled_token_resolves_immediately() {
        let token = CancellationToken::new();

        assert!(token.cancel());

        let mut future = Box::pin(token.cancelled());

        let (_, waker) = test_waker();
        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Ready(())
        ));
    }

    #[test]
    fn dropping_future_unregisters_waker() {
        let token = CancellationToken::new();

        {
            let mut future = Box::pin(token.cancelled());

            let (_, waker) = test_waker();
            let mut context = Context::from_waker(&waker);

            assert!(matches!(
                future.as_mut().poll(&mut context),
                Poll::Pending
            ));

            assert_eq!(lock_recover(&token.state.waiters).len(), 1);
        }

        assert_eq!(lock_recover(&token.state.waiters).len(), 0);
    }

    #[test]
    fn cancellation_future_replaces_changed_waker() {
        let token = CancellationToken::new();
        let mut future = Box::pin(token.cancelled());

        let (_, first_waker) = test_waker();
        let (_, second_waker) = test_waker();

        let mut first_context = Context::from_waker(&first_waker);
        let mut second_context = Context::from_waker(&second_waker);

        assert!(matches!(
            future.as_mut().poll(&mut first_context),
            Poll::Pending
        ));

        assert!(matches!(
            future.as_mut().poll(&mut second_context),
            Poll::Pending
        ));

        assert_eq!(lock_recover(&token.state.waiters).len(), 1);
    }

    #[test]
    fn cancellation_signal_exposes_reason() {
        let token = CancellationToken::new();

        let signal: Arc<dyn CancellationSignal> = Arc::new(token.clone());

        token.cancel_with_reason("operation abandoned");

        assert!(signal.is_cancelled());
        assert_eq!(
            signal.cancellation_reason().as_deref(),
            Some("operation abandoned")
        );
    }

    #[test]
    fn independent_tokens_do_not_share_state() {
        let first = CancellationToken::new();
        let second = CancellationToken::new();

        assert!(first.cancel());

        assert!(first.is_cancelled());
        assert!(!second.is_cancelled());
    }
}
