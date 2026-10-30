
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Distributed execution cancellation and deadline control.
//!
//! Path: src/distributed/execution/cancellation.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibility
//!
//! This module owns cooperative cancellation signals, cancellation tokens,
//! parent-linked cancellation, and monotonic execution deadlines.
//!
//! It does not:
//! - terminate operating-system processes;
//! - interrupt arbitrary machine instructions;
//! - forcibly stop a remote worker;
//! - implement transport cancellation messages;
//! - decide whether a partially executed operation is safe to retry;
//! - persist cancellation state across process failure;
//! - mark an execution lifecycle as Cancelled without provider confirmation;
//! - implement task scheduling, placement, or recovery.
//!
//! Those operations belong to the existing runtime, transport, lifecycle,
//! and resilience authorities.
//!
//! # Integration
//!
//! - execution/executor.rs consumes ExecutionControl and CancellationSignal.
//! - execution/lifecycle.rs records cancellation only after confirmation
//!   from the responsible execution provider.
//! - execution/results.rs preserves partial results when cancellation occurs.
//! - transport implementations translate cancellation into their own
//!   protocol-level cancellation mechanism.
//! - runtime owns token lifetimes and distributes cancellation requests.
//! - resilience/recovery.rs reconciles operations whose outcomes are unknown.
//!
//! # Guarantees and limitations
//!
//! Cancellation is cooperative. A provider must check the supplied control
//! during long-running operations and propagate it into nested operations.
//! A cancellation request does not prove that an operation stopped.
//!
//! Deadlines use std::time::Instant and are local monotonic-clock deadlines.
//! They must not be serialized as timestamps or compared between machines.
//! A remote runtime must translate the remaining duration using its own
//! clock/deadline propagation contract.
//!
//! This module has no arbitrary universal limits. Actual cancellation
//! latency depends on how frequently providers check the control and on
//! whether their underlying operations support interruption.

use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedResult,
};

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Cancellation signal
// -----------------------------------------------------------------------------

/// Read-only cooperative cancellation interface.
///
/// Implementations must be thread-safe. Execution providers may inspect
/// cancellation from different threads or execution contexts.
///
/// The default reason implementation avoids requiring every external
/// cancellation source to store a textual reason.
pub trait CancellationSignal: Send + Sync {
    /// Returns whether cancellation has been requested.
    fn is_cancelled(&self) -> bool;

    /// Returns the cancellation reason, if the source provides one.
    ///
    /// The reason is diagnostic information, not a machine-readable
    /// classification. Callers must not parse it for control flow.
    fn cancellation_reason(&self) -> Option<String> {
        None
    }
}

/// Cancellation source that never requests cancellation.
///
/// This is useful when an execution has a deadline but no external
/// cancellation source.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeverCancelled;

impl CancellationSignal for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// -----------------------------------------------------------------------------
// Token state
// -----------------------------------------------------------------------------

/// Shared state behind a cancellation token.
#[derive(Debug, Default)]
struct CancellationState {
    requested: AtomicBool,
    reason: Mutex<Option<String>>,
}

/// A cloneable cancellation token.
///
/// Every clone observes the same cancellation state. The first successful
/// cancellation request establishes the reason. Later requests are
/// idempotent and do not overwrite it.
///
/// Tokens are local in-process coordination objects. They are not wire
/// messages, durable records, or distributed consensus objects.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
    parent: Option<Arc<dyn CancellationSignal>>,
}

impl CancellationToken {
    /// Creates a new cancellation token that has not been cancelled.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a child token linked to a parent cancellation signal.
    ///
    /// The child can be cancelled independently. It also observes
    /// cancellation requested by its parent.
    ///
    /// Cancelling the child does not cancel the parent.
    #[must_use]
    pub fn linked_to(parent: Arc<dyn CancellationSignal>) -> Self {
        Self {
            state: Arc::new(CancellationState::default()),
            parent: Some(parent),
        }
    }

    /// Requests cancellation.
    ///
    /// The first successful request wins. Returns `true` if this call
    /// established cancellation and `false` if cancellation had already
    /// been requested locally or inherited from the parent.
    ///
    /// An empty reason is permitted. Reasons are diagnostic text and
    /// should not contain credentials, secrets, or sensitive payloads.
    pub fn cancel(&self, reason: impl Into<String>) -> bool {
        if self.is_cancelled() {
            return false;
        }

        // Recovering a poisoned mutex is intentional here. Cancellation
        // must remain requestable even if unrelated diagnostic code
        // previously panicked while holding the reason lock.
        let mut stored_reason = self
            .state
            .reason
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if self.state.requested.load(Ordering::Acquire) {
            return false;
        }

        *stored_reason = Some(reason.into());

        // Publish the reason before publishing the cancellation flag.
        self.state.requested.store(true, Ordering::Release);
        true
    }

    /// Returns whether cancellation has been requested locally or by
    /// the linked parent.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.requested.load(Ordering::Acquire)
            || self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.is_cancelled())
    }

    /// Returns the local cancellation reason or the linked parent's reason.
    ///
    /// A child-local reason takes precedence over its parent's reason.
    #[must_use]
    pub fn reason(&self) -> Option<String> {
        if self.state.requested.load(Ordering::Acquire) {
            let reason = self
                .state
                .reason
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            return reason.clone();
        }

        self.parent
            .as_ref()
            .and_then(|parent| parent.cancellation_reason())
    }

    /// Returns whether this token has a parent cancellation source.
    #[must_use]
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Creates an independent token.
    ///
    /// This is deliberately not a clone: a clone shares cancellation state,
    /// while a new token starts uncancelled and has no parent.
    #[must_use]
    pub fn independent(&self) -> Self {
        Self::new()
    }
}

impl CancellationSignal for CancellationToken {
    fn is_cancelled(&self) -> bool {
        CancellationToken::is_cancelled(self)
    }

    fn cancellation_reason(&self) -> Option<String> {
        self.reason()
    }
}

// -----------------------------------------------------------------------------
// Execution control
// -----------------------------------------------------------------------------

/// Cooperative cancellation and deadline control for an execution.
///
/// A control value can be cloned and shared with providers, nested tasks,
/// and runtime operations. Clones share the cancellation source and
/// preserve the same local monotonic deadline.
///
/// The deadline is not a distributed wall-clock timestamp. A remote
/// provider must receive a translated remaining duration through the
/// transport/runtime boundary.
#[derive(Clone)]
pub struct ExecutionControl {
    cancellation: Arc<dyn CancellationSignal>,
    deadline: Option<Instant>,
}

impl fmt::Debug for ExecutionControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionControl")
            .field("cancelled", &self.is_cancelled())
            .field("deadline", &self.deadline)
            .field("remaining", &self.remaining())
            .finish()
    }
}

impl ExecutionControl {
    /// Creates control with no deadline and no external cancellation.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancellation: Arc::new(NeverCancelled),
            deadline: None,
        }
    }

    /// Creates control using a cancellation token.
    #[must_use]
    pub fn with_token(token: CancellationToken) -> Self {
        Self {
            cancellation: Arc::new(token),
            deadline: None,
        }
    }

    /// Creates control using an arbitrary cancellation signal.
    ///
    /// This permits integration with existing runtime cancellation
    /// authorities without introducing another global state mechanism.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: Arc<dyn CancellationSignal>,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    /// Sets an absolute local monotonic-clock deadline.
    ///
    /// The supplied instant must originate from the same process clock.
    #[must_use]
    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets a timeout relative to the current monotonic clock.
    ///
    /// A zero duration means the deadline is immediate.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput if the duration cannot be represented by
    /// the platform's monotonic clock.
    pub fn with_timeout(self, timeout: Duration) -> DistributedResult<Self> {
        let deadline = Instant::now()
            .checked_add(timeout)
            .ok_or_else(|| {
                DistributedError::invalid_input(
                    "execution timeout exceeds the supported monotonic clock range",
                )
            })?;

        Ok(self.with_deadline(deadline))
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Returns the cancellation reason, if available.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<String> {
        self.cancellation.cancellation_reason()
    }

    /// Returns the configured local monotonic-clock deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Returns the remaining duration until the deadline.
    ///
    /// Returns `None` when no deadline is configured. A zero duration
    /// indicates that the deadline has expired.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }

    /// Returns whether the deadline has expired.
    #[must_use]
    pub fn is_deadline_expired(&self) -> bool {
        self.deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
    }

    /// Checks cancellation and deadline conditions.
    ///
    /// Cancellation takes precedence when both conditions are observed
    /// during the same check.
    ///
    /// # Errors
    ///
    /// Returns a typed cancellation or deadline error. A deadline error
    /// reports NotSent because this check itself performs no dispatch.
    /// The caller must preserve separate delivery information if an
    /// operation was already dispatched.
    pub fn check(&self) -> DistributedResult<()> {
        if self.is_cancelled() {
            let reason = self
                .cancellation_reason()
                .unwrap_or_else(|| "distributed execution cancellation requested".to_owned());

            return Err(DistributedError::Cancelled {
                operation: None,
                reason,
            });
        }

        if self.is_deadline_expired() {
            return Err(DistributedError::DeadlineExceeded {
                operation: "distributed execution".to_owned(),
                delivery: DeliveryState::NotSent,
            });
        }

        Ok(())
    }

    /// Checks cancellation and deadline conditions for a named operation.
    ///
    /// This is useful when a provider is checking a nested operation.
    ///
    /// # Errors
    ///
    /// Returns a typed cancellation or deadline error containing the
    /// supplied operation identifier.
    pub fn check_operation(
        &self,
        operation: impl Into<String>,
    ) -> DistributedResult<()> {
        let operation = operation.into();

        if self.is_cancelled() {
            let reason = self
                .cancellation_reason()
                .unwrap_or_else(|| "distributed execution cancellation requested".to_owned());

            return Err(DistributedError::Cancelled {
                operation: Some(operation),
                reason,
            });
        }

        if self.is_deadline_expired() {
            return Err(DistributedError::DeadlineExceeded {
                operation,
                delivery: DeliveryState::NotSent,
            });
        }

        Ok(())
    }

    /// Returns the control's cancellation signal as a shared trait object.
    ///
    /// Providers may clone this handle to pass cancellation observation
    /// into nested operations without gaining the ability to cancel the
    /// caller's token unless they were separately given a token.
    #[must_use]
    pub fn cancellation_signal(&self) -> Arc<dyn CancellationSignal> {
        Arc::clone(&self.cancellation)
    }

    /// Returns a new control with the same cancellation source and a
    /// different local deadline.
    #[must_use]
    pub fn with_replaced_deadline(&self, deadline: Option<Instant>) -> Self {
        Self {
            cancellation: Arc::clone(&self.cancellation),
            deadline,
        }
    }
}

impl Default for ExecutionControl {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn token_starts_uncancelled() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());
        assert_eq!(token.reason(), None);
    }

    #[test]
    fn clones_share_cancellation_state() {
        let token = CancellationToken::new();
        let clone = token.clone();

        assert!(token.cancel("requested by caller"));
        assert!(clone.is_cancelled());
        assert_eq!(
            clone.reason().as_deref(),
            Some("requested by caller")
        );
    }

    #[test]
    fn cancellation_is_idempotent_and_first_reason_wins() {
        let token = CancellationToken::new();

        assert!(token.cancel("first"));
        assert!(!token.cancel("second"));

        assert_eq!(token.reason().as_deref(), Some("first"));
    }

    #[test]
    fn child_observes_parent_cancellation() {
        let parent = CancellationToken::new();
        let parent_signal: Arc<dyn CancellationSignal> =
            Arc::new(parent.clone());

        let child = CancellationToken::linked_to(parent_signal);

        assert!(!child.is_cancelled());

        assert!(parent.cancel("parent shutdown"));

        assert!(child.is_cancelled());
        assert_eq!(
            child.reason().as_deref(),
            Some("parent shutdown")
        );
    }

    #[test]
    fn child_can_be_cancelled_without_cancelling_parent() {
        let parent = CancellationToken::new();
        let parent_signal: Arc<dyn CancellationSignal> =
            Arc::new(parent.clone());

        let child = CancellationToken::linked_to(parent_signal);

        assert!(child.cancel("child stopped"));
        assert!(child.is_cancelled());
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn independent_token_does_not_share_cancellation() {
        let token = CancellationToken::new();
        let independent = token.independent();

        assert!(token.cancel("stop original"));
        assert!(!independent.is_cancelled());
    }

    #[test]
    fn control_reports_cancellation_as_typed_error() {
        let token = CancellationToken::new();
        let control = ExecutionControl::with_token(token.clone());

        assert!(control.check().is_ok());

        token.cancel("test cancellation");

        let error = control.check().expect_err("must be cancelled");

        assert_eq!(
            error.kind(),
            crate::distributed::error::DistributedErrorKind::Cancelled
        );
    }

    #[test]
    fn control_reports_expired_deadline() {
        let control = ExecutionControl::new()
            .with_deadline(Instant::now() - Duration::from_millis(1));

        let error = control.check().expect_err("deadline must expire");

        assert_eq!(
            error.kind(),
            crate::distributed::error::DistributedErrorKind::DeadlineExceeded
        );
    }

    #[test]
    fn zero_timeout_is_immediately_expired() {
        let control = ExecutionControl::new()
            .with_timeout(Duration::ZERO)
            .expect("zero timeout is representable");

        assert!(control.is_deadline_expired());
        assert!(control.check().is_err());
    }

    #[test]
    fn control_can_be_cloned_and_shared() {
        let token = CancellationToken::new();
        let control = ExecutionControl::with_token(token.clone());
        let cloned = control.clone();

        token.cancel("shared");

        assert!(control.is_cancelled());
        assert!(cloned.is_cancelled());
    }

    #[test]
    fn concurrent_cancellation_has_one_winner() {
        let token = CancellationToken::new();
        let winners = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..16)
            .map(|index| {
                let token = token.clone();
                let winners = Arc::clone(&winners);

                thread::spawn(move || {
                    if token.cancel(format!("request-{index}")) {
                        winners.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("cancellation thread must finish");
        }

        assert_eq!(winners.load(Ordering::Relaxed), 1);
        assert!(token.is_cancelled());
        assert!(token.reason().is_some());
    }

    #[test]
    fn parent_cancellation_reason_is_inherited() {
        let parent = CancellationToken::new();
        let parent_signal: Arc<dyn CancellationSignal> =
            Arc::new(parent.clone());

        let child = CancellationToken::linked_to(parent_signal);

        parent.cancel("runtime shutdown");

        assert_eq!(
            child.reason().as_deref(),
            Some("runtime shutdown")
        );
    }

    #[test]
    fn operation_check_preserves_operation_name() {
        let token = CancellationToken::new();
        let control = ExecutionControl::with_token(token.clone());

        token.cancel("operator requested cancellation");

        let error = control
            .check_operation("distributed.task.execute")
            .expect_err("operation must be cancelled");

        match error {
            DistributedError::Cancelled { operation, reason } => {
                assert_eq!(
                    operation.as_deref(),
                    Some("distributed.task.execute")
                );
                assert_eq!(reason, "operator requested cancellation");
            }
            other => panic!("unexpected error: {other}"),
        }
    }
}
