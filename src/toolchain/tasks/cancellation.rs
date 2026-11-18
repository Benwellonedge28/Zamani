
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task and Build Cancellation
//!
//! Path: `src/toolchain/tasks/cancellation.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a shared cancellation authority for one logical toolchain
//! operation, including build operations and the tasks participating in
//! those operations.
//!
//! The module provides:
//!
//! - A cancellation source that owns the right to request cancellation.
//! - Cloneable observation tokens for tasks and providers.
//! - First-request-wins cancellation reasons.
//! - Thread-safe cancellation observation.
//! - Direct propagation to the canonical backend process cancellation token.
//! - Integration with the task executor's cancellation interfaces.
//! - Explicit cancellation checks that preserve the canonical toolchain
//!   cancellation error classification.
//!
//! ## Ownership
//!
//! This module owns cancellation coordination only.
//!
//! It does not own:
//!
//! - Task definitions or task lifecycle states.
//! - Task dependency graphs or topological ordering.
//! - Task scheduling or worker pools.
//! - Operational resource budgets.
//! - Retry or recovery policy.
//! - Provider implementation.
//! - External process execution.
//! - Compiler cancellation implementation.
//! - Runtime scheduling or execution.
//! - Hardware allocation or discovery.
//!
//! ## Canonical integration authorities
//!
//! Task cancellation behavior:
//! `crate::toolchain::tasks::model::CancellationBehavior`
//!
//! Task executor cancellation interfaces:
//! `crate::toolchain::tasks::executor`
//!
//! External process cancellation:
//! `crate::backend::process::CancellationToken`
//!
//! Toolchain cancellation errors:
//! `crate::toolchain::errors::ToolchainError`
//!
//! Toolchain results:
//! `crate::toolchain::result::ToolchainResult`
//!
//! ## Cancellation semantics
//!
//! Requesting cancellation is not proof that an operation has stopped.
//!
//! The cancellation authority:
//!
//! - Publishes a cancellation request.
//! - Makes the request observable to all token clones.
//! - Preserves the first cancellation reason.
//! - Propagates the request to backend process execution.
//!
//! The task executor and providers remain responsible for observing the
//! request and recording the resulting task lifecycle state.
//!
//! In particular:
//!
//! - Cooperative tasks must check cancellation during execution.
//! - AtSafePoint tasks must check cancellation at their documented safe
//!   points.
//! - NonInterruptible tasks cannot be forcibly interrupted by this module.
//! - Compensatable tasks require provider-owned compensation behavior.
//!
//! This module never marks a task cancelled merely because a request was
//! issued. The executor records the actual outcome.
//!
//! ## Concurrency
//!
//! Cancellation tokens are safe to clone and share across threads.
//!
//! Cancellation is idempotent. The first successful cancellation request
//! establishes the reason. Subsequent requests do not overwrite it.
//!
//! Mutex poisoning is recovered without panicking. Cancellation observation
//! must remain available even if an unrelated observer panics while holding
//! the reason lock.
//!
//! ## Scalability
//!
//! No fixed limit is imposed on the number of tasks, providers, processes,
//! build stages, computing domains, devices or distributed participants
//! that may observe a shared token.
//!
//! Tokens share one allocation-backed state rather than copying cancellation
//! state for each observer.
//!
//! Actual memory, thread, process and operating-system limits still apply.
//! Literal infinite execution or storage is not promised.
//!
//! ## Security
//!
//! Cancellation reasons are diagnostic information. Callers must not place
//! credentials, private keys, access tokens or other secrets in them.
//!
//! Cancellation does not bypass provider authorization, process isolation,
//! resource budgets or other security controls.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::backend::process;
use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;

// ============================================================================
// SHARED CANCELLATION STATE
// ============================================================================

/// Shared state for all observers of one cancellation authority.
///
/// The backend token is stored alongside the task-level state so that
/// external processes receive the same cancellation request.
///
/// The reason mutex protects only the diagnostic reason. It is never held
/// while executing providers, processes or user callbacks.
#[derive(Debug)]
struct CancellationState {
    cancelled: AtomicBool,
    reason: Mutex<Option<Arc<str>>>,
    process_token: process::CancellationToken,
}

impl CancellationState {
    /// Creates a fresh, non-cancelled state.
    fn new() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            reason: Mutex::new(None),
            process_token: process::CancellationToken::new(),
        }
    }

    /// Acquires the reason lock while recovering from poisoning.
    ///
    /// A panic in code that previously held this lock must not permanently
    /// disable cancellation observation.
    fn reason_guard(&self) -> MutexGuard<'_, Option<Arc<str>>> {
        match self.reason.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// Returns whether cancellation has been requested through either
    /// the task-level authority or the shared backend process token.
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
            || self.process_token.is_cancelled()
    }

    /// Returns the first recorded cancellation reason.
    ///
    /// If the backend process token was cancelled independently, the
    /// reason is reported as an externally initiated cancellation.
    fn reason(&self) -> Option<String> {
        if let Some(reason) = self.reason_guard().as_ref() {
            return Some(reason.to_string());
        }

        if self.process_token.is_cancelled() {
            return Some(
                "Cancellation was requested through the backend process token"
                    .to_owned(),
            );
        }

        None
    }

    /// Requests cancellation.
    ///
    /// Returns `true` only when this call establishes the cancellation
    /// request. Returns `false` if cancellation was already requested.
    fn cancel(&self, reason: String) -> bool {
        let mut recorded_reason = self.reason_guard();

        // Cancellation may also have been initiated by a consumer that
        // received the backend process token directly.
        if self.cancelled.load(Ordering::Acquire)
            || self.process_token.is_cancelled()
        {
            return false;
        }

        let reason = if reason.trim().is_empty() {
            "Cancellation requested".to_owned()
        } else {
            reason
        };

        // Record the reason before publishing cancellation. An observer
        // that sees the published cancellation request can therefore
        // retrieve its reason.
        *recorded_reason = Some(Arc::<str>::from(reason));

        // Propagate to the canonical process cancellation authority.
        // ProcessRunner observes this token and attempts to terminate
        // its direct child process.
        self.process_token.cancel();

        // Release publication makes the recorded reason and preceding
        // state changes visible to observers using Acquire loads.
        self.cancelled.store(true, Ordering::Release);

        true
    }
}

// ============================================================================
// CANCELLATION SOURCE
// ============================================================================

/// Authority that may request cancellation of one logical operation.
///
/// A source is normally created by the build executor or the caller that
/// owns the overall task execution.
///
/// Clone the source only when another component is intentionally allowed
/// to request cancellation. Give ordinary task providers a
/// [`CancellationToken`] instead.
///
/// All clones share the same cancellation state.
#[derive(Clone, Debug)]
pub struct CancellationSource {
    state: Arc<CancellationState>,
}

impl Default for CancellationSource {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationSource {
    /// Creates a new cancellation authority.
    ///
    /// The new operation begins in the non-cancelled state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
        }
    }

    /// Returns a cloneable observation token.
    ///
    /// Every token returned by this source observes the same cancellation
    /// request and reason.
    #[must_use]
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            state: Arc::clone(&self.state),
        }
    }

    /// Requests cancellation with a diagnostic reason.
    ///
    /// The first cancellation request wins. Later calls do not overwrite
    /// the original reason.
    ///
    /// Returns `true` if this call established cancellation, or `false`
    /// if cancellation had already been requested.
    ///
    /// This method does not wait for tasks or processes to stop.
    pub fn cancel(&self, reason: impl Into<String>) -> bool {
        self.state.cancel(reason.into())
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.is_cancelled()
    }

    /// Returns the recorded cancellation reason, if available.
    #[must_use]
    pub fn reason(&self) -> Option<String> {
        self.state.reason()
    }
}

// ============================================================================
// CANCELLATION TOKEN
// ============================================================================

/// Cloneable observation token for a logical task or build operation.
///
/// Tokens are read-oriented. The cancellation source owns the authority
/// to request cancellation.
///
/// A token can be shared with multiple tasks, providers and external
/// process requests. Every clone observes the same cancellation state.
#[derive(Clone, Debug)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
}

impl CancellationToken {
    /// Returns whether cancellation has been requested.
    ///
    /// This operation is lock-free for the ordinary task-level
    /// cancellation path.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state.is_cancelled()
    }

    /// Returns the first recorded cancellation reason, if available.
    #[must_use]
    pub fn reason(&self) -> Option<String> {
        self.state.reason()
    }

    /// Checks cancellation and returns the canonical toolchain error
    /// when cancellation has been requested.
    ///
    /// Providers should call this at appropriate cooperative checkpoints.
    ///
    /// This check does not forcibly interrupt a non-interruptible operation.
    pub fn check_cancelled(&self) -> ToolchainResult<()> {
        if self.is_cancelled() {
            let reason = self
                .reason()
                .unwrap_or_else(|| "Cancellation requested".to_owned());

            return Err(
                ToolchainError::cancelled(reason)
                    .with_operation("tasks.cancellation"),
            );
        }

        Ok(())
    }

    /// Returns a clone of the canonical backend process cancellation token.
    ///
    /// Pass this token to `ProcessRequest::cancellation_token` so that
    /// `ProcessRunner` observes cancellation of the owning build or task
    /// operation.
    ///
    /// The returned token shares state with the backend process authority.
    /// If a consumer cancels it directly, this task token also observes
    /// cancellation.
    #[must_use]
    pub fn backend_process_token(&self) -> process::CancellationToken {
        self.state.process_token.clone()
    }
}

// ============================================================================
// TASK EXECUTOR INTEGRATION
// ============================================================================

// These implementations adapt the shared cancellation authority to the
// executor contracts that already exist in tasks/executor.rs.
//
// They deliberately do not define new executor traits or duplicate the
// executor's cancellation interfaces.

impl super::executor::CancellationProbe for CancellationToken {
    /// Returns whether cancellation has been requested.
    fn is_cancelled(&self) -> bool {
        CancellationToken::is_cancelled(self)
    }
}

impl super::executor::CancellationToken for CancellationToken {
    /// Returns the first recorded cancellation reason.
    fn cancellation_reason(&self) -> Option<String> {
        CancellationToken::reason(self)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn new_source_and_token_are_not_cancelled() {
        let source = CancellationSource::new();
        let token = source.token();

        assert!(!source.is_cancelled());
        assert!(!token.is_cancelled());
        assert_eq!(source.reason(), None);
        assert_eq!(token.reason(), None);
        assert!(token.check_cancelled().is_ok());
    }

    #[test]
    fn cancellation_is_visible_to_all_token_clones() {
        let source = CancellationSource::new();

        let first = source.token();
        let second = first.clone();
        let third = source.token();

        assert!(source.cancel("user requested cancellation"));

        assert!(first.is_cancelled());
        assert!(second.is_cancelled());
        assert!(third.is_cancelled());

        assert_eq!(
            first.reason().as_deref(),
            Some("user requested cancellation")
        );
    }

    #[test]
    fn first_cancellation_reason_is_preserved() {
        let source = CancellationSource::new();

        assert!(source.cancel("first reason"));
        assert!(!source.cancel("second reason"));

        assert_eq!(
            source.reason().as_deref(),
            Some("first reason")
        );
    }

    #[test]
    fn empty_reason_uses_a_non_empty_default() {
        let source = CancellationSource::new();

        assert!(source.cancel("   "));

        assert_eq!(
            source.reason().as_deref(),
            Some("Cancellation requested")
        );
    }

    #[test]
    fn cancellation_propagates_to_backend_process_token() {
        let source = CancellationSource::new();
        let task_token = source.token();
        let process_token = task_token.backend_process_token();

        assert!(!process_token.is_cancelled());

        assert!(source.cancel("stop external compiler"));

        assert!(process_token.is_cancelled());
        assert!(task_token.is_cancelled());
    }

    #[test]
    fn backend_process_cancellation_is_visible_to_task_token() {
        let source = CancellationSource::new();
        let task_token = source.token();
        let process_token = task_token.backend_process_token();

        process_token.cancel();

        assert!(task_token.is_cancelled());
        assert!(source.is_cancelled());

        assert_eq!(
            task_token.reason().as_deref(),
            Some(
                "Cancellation was requested through the backend process token"
            )
        );
    }

    #[test]
    fn check_cancelled_returns_canonical_cancelled_error() {
        let source = CancellationSource::new();
        let token = source.token();

        source.cancel("build cancelled");

        let error = token
            .check_cancelled()
            .expect_err("cancelled operation must return an error");

        assert_eq!(
            error.kind(),
            crate::toolchain::errors::ToolchainErrorKind::Cancelled
        );

        assert_eq!(
            error.operation(),
            Some("tasks.cancellation")
        );

        assert!(error.message().contains("build cancelled"));
    }

    #[test]
    fn concurrent_cancellation_is_idempotent() {
        let source = CancellationSource::new();
        let barrier = Arc::new(Barrier::new(3));

        let first_source = source.clone();
        let first_barrier = Arc::clone(&barrier);

        let first = thread::spawn(move || {
            first_barrier.wait();
            first_source.cancel("first concurrent request")
        });

        let second_source = source.clone();
        let second_barrier = Arc::clone(&barrier);

        let second = thread::spawn(move || {
            second_barrier.wait();
            second_source.cancel("second concurrent request")
        });

        barrier.wait();

        let first_won = first.join().expect("first thread should finish");
        let second_won = second.join().expect("second thread should finish");

        // Exactly one caller establishes the cancellation reason.
        assert_ne!(first_won, second_won);

        assert!(source.is_cancelled());

        let reason = source
            .reason()
            .expect("a successful cancellation has a reason");

        assert!(
            reason == "first concurrent request"
                || reason == "second concurrent request"
        );
    }

    #[test]
    fn executor_cancellation_interfaces_observe_shared_state() {
        use crate::toolchain::tasks::executor::{
            CancellationProbe,
            CancellationToken as ExecutorCancellationToken,
        };

        let source = CancellationSource::new();
        let token = source.token();

        let probe: &dyn CancellationProbe = &token;
        let executor_token: &dyn ExecutorCancellationToken = &token;

        assert!(!probe.is_cancelled());
        assert_eq!(executor_token.cancellation_reason(), None);

        source.cancel("executor cancellation");

        assert!(probe.is_cancelled());

        assert_eq!(
            executor_token.cancellation_reason().as_deref(),
            Some("executor cancellation")
        );
    }
}
