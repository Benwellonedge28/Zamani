
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Cancellation
//!
//! Path: src/zur/execution/cancellation.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the ZUR execution-layer cancellation coordination API.
//!
//! This module coordinates cooperative cancellation across:
//!
//! - Execution operations.
//! - Execution sessions.
//! - Nested operations.
//! - Runtime-managed tasks.
//! - Asynchronous computations.
//! - JIT operations.
//! - Domain execution adapters.
//! - Hybrid execution.
//! - Distributed execution.
//! - External execution providers.
//!
//! It reuses the standard library's canonical CancellationToken.
//!
//! It does not implement another cancellation-token state machine.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - ExecutionCancellationController.
//! - Cancellation controller composition.
//! - Parent-linked execution cancellation.
//! - Child cancellation scopes.
//! - Operation-context integration.
//! - Execution cancellation request orchestration.
//! - Cancellation request reports.
//!
//! EXISTING AUTHORITIES
//!
//! - zur/error.rs:
//!   ZurError and ZurResult.
//!
//! - zur/ids.rs:
//!   ExecutionId and SessionId.
//!
//! - zur/contracts/execution.rs:
//!   ExecutionEngine, ExecutionHandle and CancellationDisposition.
//!
//! - zur/execution/session.rs:
//!   ExecutionSession and session cancellation.
//!
//! - stdlib/context.rs:
//!   OperationContext and CancellationSignal.
//!
//! - stdlib/concurrency/cancellation.rs:
//!   CancellationToken and CancellationFuture.
//!
//! - distributed/execution/cancellation.rs:
//!   Distributed cancellation and deadline semantics.
//!
//! OWNERSHIP BOUNDARIES
//!
//! This module does not:
//!
//! - Define source-language cancellation semantics.
//! - Parse cancellation grammar.
//! - Define cancellation requirements or hints.
//! - Define execution lifecycle states.
//! - Define ExecutionId or SessionId.
//! - Implement task scheduling.
//! - Terminate operating-system processes.
//! - Forcefully interrupt arbitrary machine instructions.
//! - Interrupt physical quantum operations.
//! - Interrupt arbitrary FPGA or hardware operations.
//! - Implement distributed cancellation transport.
//! - Implement distributed consensus.
//! - Implement checkpointing.
//! - Implement rollback or recovery.
//! - Allocate or release physical resources.
//! - Replace provider cancellation acknowledgements.
//!
//! These responsibilities remain with their existing authorities.
//!
//! CANCELLATION GUARANTEES
//!
//! A cancellation request is not proof that execution stopped.
//!
//! Local cooperative cancellation and authoritative execution
//! cancellation are separate observable events.
//!
//! Only CancellationDisposition::Confirmed establishes an
//! explicit cancellation confirmation from the execution authority.
//!
//! SCALABILITY
//!
//! No artificial maximum is imposed on:
//!
//! - Cancellation controllers.
//! - Child cancellation scopes.
//! - Execution operations.
//! - Sessions.
//! - Tasks.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Distributed participants.
//!
//! Actual allocations remain subject to available resources,
//! address-space constraints and implementation capabilities.
//!
//! Literal infinite physical computation is not promised.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw pointers.
//! No static mutable state.
//! No global cancellation registry.
//! No arbitrary hardware ceilings.
//! No fabricated cancellation confirmation.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use std::sync::Arc;

use crate::stdlib::concurrency::cancellation::{
    CancellationFuture,
    CancellationToken,
};

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
};

use crate::zur::contracts::execution::{
    CancellationDisposition,
    ExecutionEngine,
    ExecutionHandle,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.execution.cancellation";

// ============================================================================
// EXECUTION CANCELLATION CONTROLLER
// ============================================================================

/// Thread-safe cooperative cancellation controller.
///
/// This controller coordinates cancellation for one logical
/// execution scope.
///
/// Cloning a controller shares its cancellation state.
///
/// Independently created controllers do not share local state.
///
/// A controller may optionally observe cancellation from a parent
/// cancellation source.
///
/// Parent cancellation propagates downward.
///
/// Child cancellation never propagates upward.
///
/// The controller delegates token implementation and asynchronous
/// waiter notification to the standard-library cancellation system.
#[derive(Clone)]
pub struct ExecutionCancellationController {
    token: CancellationToken,
}

impl ExecutionCancellationController {

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates an independent execution cancellation controller.
    ///
    /// The controller starts in the uncancelled state.
    ///
    /// It does not depend on a runtime singleton, execution session,
    /// operating system or particular computational domain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    /// Creates a controller linked to an existing cancellation signal.
    ///
    /// Cancellation requested by the parent is observable through
    /// this controller.
    ///
    /// Cancelling this controller does not cancel the parent.
    ///
    /// The parent is retained through a shared reference.
    #[must_use]
    pub fn linked_to(
        parent: Arc<dyn CancellationSignal>,
    ) -> Self {
        Self {
            token: CancellationToken::linked_to(parent),
        }
    }

    /// Creates a controller linked to an operation context.
    ///
    /// This is the preferred integration boundary when an execution
    /// operation must inherit runtime or session cancellation.
    ///
    /// Parent cancellation remains observable.
    ///
    /// Local cancellation remains independently requestable.
    #[must_use]
    pub fn linked_to_context(
        parent: &OperationContext,
    ) -> Self {
        Self::linked_to(
            parent.cancellation_signal(),
        )
    }

    /// Creates a child cancellation controller.
    ///
    /// The child:
    ///
    /// - Observes cancellation of its parent.
    /// - Has independent local cancellation state.
    /// - Can be cancelled without cancelling its parent.
    /// - Preserves its own cancellation reason.
    ///
    /// This operation does not create a new execution identity
    /// or execution session.
    #[must_use]
    pub fn child(&self) -> Self {
        Self {
            token: self.token.child_token(),
        }
    }

    // ------------------------------------------------------------------------
    // Cancellation requests
    // ------------------------------------------------------------------------

    /// Requests cooperative cancellation.
    ///
    /// The first successful local cancellation request establishes
    /// the local cancellation reason.
    ///
    /// Subsequent requests do not overwrite that reason.
    ///
    /// Returns true when this call establishes local cancellation.
    ///
    /// Returns false when cancellation was already requested locally
    /// or inherited from a parent.
    ///
    /// The reason is descriptive information only.
    ///
    /// Do not place credentials, secrets or private keys in it.
    ///
    /// This operation does not terminate the associated computation.
    pub fn request(
        &self,
        reason: impl Into<String>,
    ) -> bool {
        self.token.cancel_with_reason(reason)
    }

    /// Requests cancellation using the standard default reason.
    ///
    /// Returns true only when this call establishes local cancellation.
    pub fn request_default(&self) -> bool {
        self.request("execution cancellation requested")
    }

    // ------------------------------------------------------------------------
    // Cancellation observation
    // ------------------------------------------------------------------------

    /// Returns whether cancellation has been requested locally
    /// or inherited from a parent.
    #[must_use]
    pub fn is_requested(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Returns whether this controller has a parent cancellation source.
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.token.has_parent()
    }

    /// Returns the local cancellation reason or inherited parent reason.
    ///
    /// A local reason takes precedence over a parent reason.
    #[must_use]
    pub fn reason(&self) -> Option<String> {
        self.token.reason()
    }

    /// Returns the underlying canonical cancellation token.
    ///
    /// This permits integration with standard-library tasks,
    /// asynchronous operations and other existing cancellation-aware
    /// facilities.
    ///
    /// The returned reference does not permit replacing the token.
    #[must_use]
    pub fn token(&self) -> &CancellationToken {
        &self.token
    }

    /// Returns a shared read-only cancellation signal.
    ///
    /// The signal can be supplied to OperationContext.
    ///
    /// The caller cannot cancel through this read-only interface.
    #[must_use]
    pub fn signal(&self) -> Arc<dyn CancellationSignal> {
        Arc::new(self.token.clone())
    }

    /// Returns a future that resolves when cancellation is observed.
    ///
    /// This future uses the standard library's existing cancellation
    /// notification mechanism.
    ///
    /// It registers and unregisters asynchronous waiters according
    /// to the canonical CancellationFuture implementation.
    ///
    /// Dropping the future does not cancel the controller.
    #[must_use]
    pub fn cancelled(&self) -> CancellationFuture {
        self.token.cancelled()
    }

    /// Checks whether cancellation has been requested.
    ///
    /// This method checks cancellation only.
    ///
    /// It does not independently check deadlines, permissions,
    /// capabilities, feasibility or resource budgets.
    ///
    /// # Errors
    ///
    /// Returns ZurError with ErrorKind::Cancelled when cancellation
    /// has been requested.
    pub fn check(&self) -> ZurResult<()> {
        if !self.is_requested() {
            return Ok(());
        }

        let reason = self.reason().unwrap_or_else(|| {
            "execution cancellation requested".to_owned()
        });

        Err(
            ZurError::cancelled(reason)
                .with_component(COMPONENT)
                .with_operation("check"),
        )
    }

    // ------------------------------------------------------------------------
    // Operation-context integration
    // ------------------------------------------------------------------------

    /// Creates a child operation context linked to this controller.
    ///
    /// The supplied parent context is used to inherit:
    ///
    /// - Component metadata.
    /// - Logical scope metadata.
    /// - Existing typed extensions.
    ///
    /// This controller must itself have been created with
    /// linked_to_context(parent) when parent cancellation must
    /// propagate into the resulting context.
    ///
    /// The new operation name is descriptive metadata.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if the operation name
    /// is invalid.
    pub fn operation_context(
        &self,
        parent: &OperationContext,
        operation: impl Into<String>,
    ) -> ZurResult<OperationContext> {

        let builder = parent
            .child_builder()
            .with_operation(operation)
            .map_err(|error| {
                ZurError::invalid_input(
                    error.to_string(),
                )
                .with_component(COMPONENT)
                .with_operation("operation_context")
            })?;

        Ok(
            builder
                .with_cancellation(self.signal())
                .build()
        )
    }

    // ------------------------------------------------------------------------
    // Execution-engine integration
    // ------------------------------------------------------------------------

    /// Requests cancellation through the canonical execution engine.
    ///
    /// This method performs two distinct operations:
    ///
    /// 1. Requests local cooperative cancellation.
    /// 2. Requests cancellation from the authoritative execution engine.
    ///
    /// The execution engine remains responsible for establishing
    /// whether execution actually stopped.
    ///
    /// `control_context` must be an operation context suitable for
    /// carrying the cancellation request itself. It should not be
    /// the already-cancelled operation context when that context
    /// would cause the engine to abandon the cancellation request.
    ///
    /// A failed engine request does not undo the local cancellation.
    ///
    /// Repeated calls may issue repeated engine requests. The engine
    /// is responsible for handling already-terminal executions.
    ///
    /// This method does not independently modify ExecutionState.
    ///
    /// # Errors
    ///
    /// Returns the structured error reported by the execution engine.
    ///
    /// # Important
    ///
    /// A successful return does not necessarily mean cancellation
    /// was confirmed.
    ///
    /// Inspect CancellationRequestReport::disposition().
    pub async fn cancel_execution<'a>(
        &'a self,
        engine: &'a dyn ExecutionEngine,
        handle: &'a ExecutionHandle,
        control_context: &'a OperationContext,
        reason: impl Into<String>,
    ) -> ZurResult<CancellationRequestReport> {

        let local_request_established = self.request(reason);

        let disposition = engine
            .cancel(handle, control_context)
            .await?;

        Ok(
            CancellationRequestReport {
                local_request_established,
                disposition,
            }
        )
    }
}

// ============================================================================
// DEFAULT
// ============================================================================

impl Default for ExecutionCancellationController {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for ExecutionCancellationController {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutionCancellationController")
            .field("requested", &self.is_requested())
            .field("has_parent", &self.has_parent())
            .field("reason", &self.reason())
            .finish()
    }
}

// ============================================================================
// EXECUTION CANCELLATION REQUEST REPORT
// ============================================================================

/// Report describing local and authoritative cancellation outcomes.
///
/// This report deliberately preserves the distinction between
/// a local cancellation request and the execution authority's
/// reported cancellation disposition.
///
/// It does not invent execution completion or provider acknowledgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancellationRequestReport {

    /// Whether this call established a new local cancellation request.
    ///
    /// False means cancellation was already requested locally
    /// or inherited from a parent.
    local_request_established: bool,

    /// Cancellation disposition reported by the execution authority.
    disposition: CancellationDisposition,
}

impl CancellationRequestReport {

    /// Returns whether this call established local cancellation.
    #[must_use]
    pub const fn local_request_established(
        &self,
    ) -> bool {
        self.local_request_established
    }

    /// Returns the authoritative execution cancellation disposition.
    #[must_use]
    pub const fn disposition(
        &self,
    ) -> CancellationDisposition {
        self.disposition
    }

    /// Returns whether the execution authority explicitly confirmed
    /// cancellation.
    ///
    /// Only CancellationDisposition::Confirmed returns true.
    #[must_use]
    pub const fn is_confirmed(
        &self,
    ) -> bool {
        self.disposition.is_confirmed()
    }

    /// Returns whether the request was accepted but confirmation
    /// remains outstanding.
    #[must_use]
    pub const fn is_pending(
        &self,
    ) -> bool {
        matches!(
            self.disposition,
            CancellationDisposition::Requested
        )
    }

    /// Returns whether the execution authority rejected the request.
    #[must_use]
    pub const fn is_rejected(
        &self,
    ) -> bool {
        matches!(
            self.disposition,
            CancellationDisposition::Rejected
        )
    }

    /// Returns whether the execution authority could not establish
    /// the cancellation outcome.
    #[must_use]
    pub const fn is_unknown(
        &self,
    ) -> bool {
        matches!(
            self.disposition,
            CancellationDisposition::Unknown
        )
    }

    /// Returns whether the execution was already terminal.
    #[must_use]
    pub const fn was_already_terminal(
        &self,
    ) -> bool {
        matches!(
            self.disposition,
            CancellationDisposition::AlreadyTerminal
        )
    }
}

// ============================================================================
// CANCELLATION OBSERVATION FUTURE
// ============================================================================

/// Future that resolves when the controller observes cancellation.
///
/// This is a convenience wrapper around the canonical
/// standard-library CancellationFuture.
///
/// It introduces no independent waiter registry or notification state.
#[must_use = "cancellation observation futures must be polled"]
pub struct ExecutionCancellationFuture {
    inner: CancellationFuture,
}

impl ExecutionCancellationFuture {

    /// Creates an observation future for a cancellation controller.
    #[must_use]
    pub fn new(
        controller: &ExecutionCancellationController,
    ) -> Self {
        Self {
            inner: controller.cancelled(),
        }
    }
}

impl Future for ExecutionCancellationFuture {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(context)
    }
}

impl fmt::Debug for ExecutionCancellationFuture {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutionCancellationFuture")
            .finish_non_exhaustive()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use core::task::Wake;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use std::task::Waker;

    // ------------------------------------------------------------------------
    // Test wake implementation
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn new_controller_starts_uncancelled() {
        let controller =
            ExecutionCancellationController::new();

        assert!(!controller.is_requested());
        assert!(!controller.has_parent());
        assert_eq!(controller.reason(), None);
    }

    #[test]
    fn default_controller_is_independent() {
        let first =
            ExecutionCancellationController::default();

        let second =
            ExecutionCancellationController::default();

        assert!(first.request("first"));

        assert!(first.is_requested());
        assert!(!second.is_requested());
    }

    // ------------------------------------------------------------------------
    // Cancellation behavior
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_request_is_observable() {
        let controller =
            ExecutionCancellationController::new();

        assert!(controller.request("stop execution"));

        assert!(controller.is_requested());

        assert_eq!(
            controller.reason().as_deref(),
            Some("stop execution"),
        );
    }

    #[test]
    fn repeated_requests_preserve_first_reason() {
        let controller =
            ExecutionCancellationController::new();

        assert!(controller.request("first reason"));

        assert!(!controller.request("second reason"));

        assert_eq!(
            controller.reason().as_deref(),
            Some("first reason"),
        );
    }

    #[test]
    fn check_succeeds_before_cancellation() {
        let controller =
            ExecutionCancellationController::new();

        assert!(controller.check().is_ok());
    }

    #[test]
    fn check_returns_structured_cancellation_error() {
        let controller =
            ExecutionCancellationController::new();

        controller.request("test cancellation");

        let error = controller
            .check()
            .expect_err("cancellation must be observable");

        assert_eq!(
            error.kind(),
            crate::zur::error::ErrorKind::Cancelled,
        );
    }

    // ------------------------------------------------------------------------
    // Cloning
    // ------------------------------------------------------------------------

    #[test]
    fn cloned_controllers_share_state() {
        let original =
            ExecutionCancellationController::new();

        let cloned = original.clone();

        assert!(original.request("shared cancellation"));

        assert!(cloned.is_requested());

        assert_eq!(
            cloned.reason().as_deref(),
            Some("shared cancellation"),
        );
    }

    // ------------------------------------------------------------------------
    // Parent and child propagation
    // ------------------------------------------------------------------------

    #[test]
    fn linked_controller_observes_parent_cancellation() {
        let parent =
            ExecutionCancellationController::new();

        let child =
            ExecutionCancellationController::linked_to(
                parent.signal(),
            );

        assert!(parent.request("parent cancelled"));

        assert!(child.is_requested());

        assert_eq!(
            child.reason().as_deref(),
            Some("parent cancelled"),
        );
    }

    #[test]
    fn child_cancellation_does_not_cancel_parent() {
        let parent =
            ExecutionCancellationController::new();

        let child = parent.child();

        assert!(child.request("child cancelled"));

        assert!(child.is_requested());

        assert!(!parent.is_requested());
    }

    #[test]
    fn independent_controllers_do_not_share_state() {
        let first =
            ExecutionCancellationController::new();

        let second =
            ExecutionCancellationController::new();

        assert!(first.request("first"));

        assert!(!second.is_requested());
    }

    #[test]
    fn nested_child_observes_ancestor_cancellation() {
        let root =
            ExecutionCancellationController::new();

        let child = root.child();

        let grandchild = child.child();

        assert!(root.request("root cancelled"));

        assert!(child.is_requested());

        assert!(grandchild.is_requested());
    }

    // ------------------------------------------------------------------------
    // Operation-context integration
    // ------------------------------------------------------------------------

    #[test]
    fn operation_context_observes_local_cancellation() {
        let parent = OperationContext::default();

        let controller =
            ExecutionCancellationController::linked_to_context(
                &parent,
            );

        let operation = controller
            .operation_context(
                &parent,
                "test.operation",
            )
            .expect("valid operation context");

        assert!(!operation.is_cancelled());

        assert!(controller.request("operation stopped"));

        assert!(operation.is_cancelled());
    }

    #[test]
    fn operation_context_inherits_parent_cancellation() {
        let parent_controller =
            ExecutionCancellationController::new();

        let parent = OperationContext::builder()
            .with_cancellation(
                parent_controller.signal(),
            )
            .build();

        let child_controller =
            ExecutionCancellationController::linked_to_context(
                &parent,
            );

        let operation = child_controller
            .operation_context(
                &parent,
                "nested.operation",
            )
            .expect("valid operation context");

        assert!(parent_controller.request("runtime stopping"));

        assert!(child_controller.is_requested());

        assert!(operation.is_cancelled());
    }

    // ------------------------------------------------------------------------
    // Asynchronous observation
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_future_waits_until_requested() {
        let controller =
            ExecutionCancellationController::new();

        let mut future = Box::pin(controller.cancelled());

        let wake = Arc::new(CountingWake::new());

        let waker = Waker::from(Arc::clone(&wake));

        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Pending,
        ));

        assert!(controller.request("wake waiter"));

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Ready(()),
        ));

        assert!(wake.count() > 0);
    }

    #[test]
    fn cancellation_future_is_immediately_ready_after_request() {
        let controller =
            ExecutionCancellationController::new();

        assert!(controller.request("already cancelled"));

        let mut future = Box::pin(controller.cancelled());

        let wake = Arc::new(CountingWake::new());

        let waker = Waker::from(wake);

        let mut context = Context::from_waker(&waker);

        assert!(matches!(
            future.as_mut().poll(&mut context),
            Poll::Ready(()),
        ));
    }

    #[test]
    fn dropped_cancellation_future_does_not_cancel_controller() {
        let controller =
            ExecutionCancellationController::new();

        let future = controller.cancelled();

        drop(future);

        assert!(!controller.is_requested());

        assert!(controller.request("later cancellation"));

        assert!(controller.is_requested());
    }

    // ------------------------------------------------------------------------
    // Report semantics
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_disposition_confirmation_is_not_assumed() {
        assert!(
            !CancellationDisposition::Requested.is_confirmed()
        );

        assert!(
            !CancellationDisposition::Unknown.is_confirmed()
        );

        assert!(
            !CancellationDisposition::Rejected.is_confirmed()
        );

        assert!(
            CancellationDisposition::Confirmed.is_confirmed()
        );
    }

    // ------------------------------------------------------------------------
    // Thread safety
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_is_visible_across_threads() {
        let controller =
            ExecutionCancellationController::new();

        let worker_controller = controller.clone();

        let worker = std::thread::spawn(move || {
            worker_controller.request("worker cancellation");
        });

        worker.join().expect("worker should finish");

        assert!(controller.is_requested());

        assert_eq!(
            controller.reason().as_deref(),
            Some("worker cancellation"),
        );
    }
}
