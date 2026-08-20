//! Cooperative cancellation infrastructure for Zamani Quantum Error Correction.
//!
//! Cancellation is part of the QEC execution contract. Expensive operations
//! must bind a [`CancellationToken`] and poll it at safe points. Cancellation
//! is cooperative: no thread is force-killed while it may be mutating QEC
//! state.
//!
//! The implementation is deliberately runtime-agnostic and is suitable for
//! decoders, streaming, simulation, checkpointing, partition reconciliation,
//! distributed coordination and QPU submission/polling.

use core::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Condvar, Mutex, RwLock,
};
use std::thread;
use std::time::{Duration, Instant};

use super::errors::{QecError, QecResult};

// -----------------------------------------------------------------------------
// Cancellation reason
// -----------------------------------------------------------------------------

/// Why an operation stopped cooperatively.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancellationReason {
    /// Cancellation was explicitly requested by the caller.
    Requested,

    /// A parent operation cancelled this child operation.
    ParentCancelled,

    /// The configured deadline elapsed.
    DeadlineExceeded,

    /// A cancellation checkpoint budget was exceeded.
    BudgetExceeded,

    /// The owning scheduler shut down the workload.
    SchedulerShutdown,

    /// A distributed coordinator shut down the workload.
    DistributedShutdown,

    /// The operation was superseded by another operation.
    Superseded,

    /// Application-specific cancellation reason.
    Custom(String),
}

impl CancellationReason {
    /// Stable machine-readable identifier.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::ParentCancelled => "parent_cancelled",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::BudgetExceeded => "budget_exceeded",
            Self::SchedulerShutdown => "scheduler_shutdown",
            Self::DistributedShutdown => "distributed_shutdown",
            Self::Superseded => "superseded",
            Self::Custom(_) => "custom",
        }
    }
}

impl fmt::Display for CancellationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(message) => {
                write!(f, "custom cancellation: {message}")
            }
            other => f.write_str(other.as_str()),
        }
    }
}

// -----------------------------------------------------------------------------
// Shared cancellation state
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct CancellationMetadata {
    reason: CancellationReason,
    requested_at: Instant,
    generation: u64,
}

struct CancellationState {
    cancelled: AtomicBool,
    generation: AtomicU64,

    metadata: RwLock<Option<CancellationMetadata>>,

    wait_lock: Mutex<()>,
    wait_condvar: Condvar,

    callbacks:
        Mutex<Vec<Arc<dyn Fn(CancellationReason) + Send + Sync + 'static>>>,
}

impl fmt::Debug for CancellationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CancellationState")
            .field(
                "cancelled",
                &self.cancelled.load(Ordering::Acquire),
            )
            .field(
                "generation",
                &self.generation.load(Ordering::Acquire),
            )
            .field(
                "metadata",
                &self
                    .metadata
                    .read()
                    .ok()
                    .and_then(|guard| guard.clone()),
            )
            .field(
                "callback_count",
                &self
                    .callbacks
                    .lock()
                    .map(|guard| guard.len())
                    .unwrap_or(0),
            )
            .finish()
    }
}

impl CancellationState {
    fn new() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            generation: AtomicU64::new(0),
            metadata: RwLock::new(None),
            wait_lock: Mutex::new(()),
            wait_condvar: Condvar::new(),
            callbacks: Mutex::new(Vec::new()),
        }
    }

    /// Transition active -> cancelled exactly once.
    fn cancel(&self, reason: CancellationReason) -> bool {
        if self.cancelled.swap(true, Ordering::AcqRel) {
            return false;
        }

        let generation = self
            .generation
            .fetch_add(1, Ordering::AcqRel)
            .saturating_add(1);

        {
            let mut metadata = self
                .metadata
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            *metadata = Some(CancellationMetadata {
                reason: reason.clone(),
                requested_at: Instant::now(),
                generation,
            });
        }

        /*
         * Wake waiters before callbacks. Cancellation state is already
         * committed at this point, so callbacks cannot prevent propagation.
         */
        self.wait_condvar.notify_all();

        /*
         * Drain callbacks under the callback mutex.

         * This closes the registration race:
         *
         *   cancel()        on_cancel()
         *      |                |
         *      |                |
         *      +--- callback lock
         *
         * A callback is therefore either drained by cancel(), or observed as
         * already cancelled and invoked immediately by on_cancel().
         */
        let callbacks = {
            let mut guard = self
                .callbacks
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            std::mem::take(&mut *guard)
        };

        for callback in callbacks {
            let callback_reason = reason.clone();

            /*
             * A callback must never prevent cancellation from completing.
             * Cancellation remains committed even if application callback
             * code panics.
             */
            let _ = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    callback(callback_reason);
                }),
            );
        }

        true
    }

    fn reason(&self) -> Option<CancellationReason> {
        self.metadata
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            .map(|metadata| metadata.reason.clone())
    }

    fn metadata(&self) -> Option<CancellationMetadata> {
        self.metadata
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }
}

// -----------------------------------------------------------------------------
// Cancellation source
// -----------------------------------------------------------------------------

/// Owner-side cancellation controller.
///
/// Sources belong to callers, schedulers, supervisors, or distributed
/// coordinators. Workers normally receive only a [`CancellationToken`].
#[derive(Clone)]
pub struct CancellationSource {
    state: Arc<CancellationState>,
}

impl fmt::Debug for CancellationSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CancellationSource")
            .field("state", &self.state)
            .finish()
    }
}

impl CancellationSource {
    /// Creates an independent cancellation source.
    pub fn new() -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
        }
    }

    /// Creates a source and its corresponding token.
    pub fn new_pair() -> (Self, CancellationToken) {
        let source = Self::new();
        let token = source.token();

        (source, token)
    }

    /// Returns a token observing this source.
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            state: Arc::clone(&self.state),
            parent: None,
            deadline: None,
        }
    }

    /// Requests normal cancellation.
    pub fn cancel(&self) -> bool {
        self.cancel_with_reason(CancellationReason::Requested)
    }

    /// Requests cancellation with an explicit reason.
    pub fn cancel_with_reason(
        &self,
        reason: CancellationReason,
    ) -> bool {
        self.state.cancel(reason)
    }

    /// Returns whether this source is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    /// Returns the source's cancellation reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        self.state.reason()
    }

    /// Returns the cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Registers a callback.

    /// The callback executes at most once.
    ///
    /// If cancellation already happened, the callback is invoked immediately.
    pub fn on_cancel<F>(&self, callback: F)
    where
        F: Fn(CancellationReason) + Send + Sync + 'static,
    {
        let callback:
            Arc<dyn Fn(CancellationReason) + Send + Sync + 'static> =
            Arc::new(callback);

        let immediate_reason = {
            let mut callbacks = self
                .state
                .callbacks
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if self.state.cancelled.load(Ordering::Acquire) {
                self.state.reason()
            } else {
                callbacks.push(callback.clone());
                None
            }
        };

        if let Some(reason) = immediate_reason {
            let _ = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    callback(reason);
                }),
            );
        }
    }

    /// Creates an independently cancellable child operation.
    ///
    /// Parent cancellation is always visible to the returned child token.
    pub fn child(&self) -> (CancellationSource, CancellationToken) {
        let child = Self::new();

        let token = child
            .token()
            .with_parent(self.token());

        (child, token)
    }
}

impl Default for CancellationSource {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Cancellation token
// -----------------------------------------------------------------------------

/// Cheap, cloneable, thread-safe cooperative cancellation token.
///
/// Every expensive QEC operation should receive one of these.
#[derive(Clone)]
pub struct CancellationToken {
    state: Arc<CancellationState>,

    /*
     * Parent state is intentionally separate from the local state.
     *
     * This gives the child:
     *
     *   parent cancellation -> child observes it
     *
     * without allowing:
     *
     *   child cancellation -> parent cancellation.
     */
    parent: Option<Arc<CancellationState>>,

    deadline: Option<Instant>,
}

impl fmt::Debug for CancellationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CancellationToken")
            .field("cancelled", &self.is_cancelled())
            .field("reason", &self.reason())
            .field("deadline", &self.deadline)
            .field("has_parent", &self.parent.is_some())
            .finish()
    }
}

impl CancellationToken {
    /// Creates an active token.
    pub fn new() -> Self {
        CancellationSource::new().token()
    }

    /// Links this token to a parent token.
    ///
    /// Existing deadlines are never extended.
    fn with_parent(mut self, parent: Self) -> Self {
        self.deadline = match (self.deadline, parent.deadline) {
            (Some(child), Some(parent)) => {
                Some(child.min(parent))
            }
            (child, parent) => child.or(parent),
        };

        self.parent = Some(parent.state);

        self
    }

    /// Creates a token with a relative deadline.
    pub fn with_timeout(duration: Duration) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            parent: None,
            deadline: Instant::now().checked_add(duration),
        }
    }

    /// Creates a token with an absolute deadline.
    pub fn with_deadline(deadline: Instant) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            parent: None,
            deadline: Some(deadline),
        }
    }

    /// Returns true when this token, its parent, or its deadline is cancelled.
    ///
    /// Deadline cancellation is materialized into local cancellation state so
    /// that callbacks, waiters, generations, and diagnostics observe the same
    /// state transition.
    pub fn is_cancelled(&self) -> bool {
        if self.state.cancelled.load(Ordering::Acquire) {
            return true;
        }

        if let Some(parent) = &self.parent {
            if parent.cancelled.load(Ordering::Acquire) {
                return true;
            }
        }

        if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            let _ = self
                .state
                .cancel(CancellationReason::DeadlineExceeded);

            return true;
        }

        false
    }

    /// Returns the effective cancellation reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        /*
         * Parent cancellation takes precedence because the child operation
         * itself did not request the shutdown.
         */
        if let Some(parent) = &self.parent {
            if parent.cancelled.load(Ordering::Acquire) {
                return parent
                    .reason()
                    .or(Some(CancellationReason::ParentCancelled));
            }
        }

        if self.is_cancelled() {
            return self.state.reason();
        }

        None
    }

    /// Returns the configured effective deadline.
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Returns remaining time until the effective deadline.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline.map(|deadline| {
            deadline.saturating_duration_since(Instant::now())
        })
    }

    /// Returns this token's local cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Canonical cancellation boundary for QEC algorithms.
    #[inline]
    pub fn check(&self) -> QecResult<()> {
        if !self.is_cancelled() {
            return Ok(());
        }

        let reason = self
            .reason()
            .unwrap_or(CancellationReason::Requested);

        Err(QecError::cancelled(format!(
            "QEC operation cancelled: {reason}"
        )))
    }

    /// Requests cancellation of this token's local operation.
    ///
    /// Prefer retaining a [`CancellationSource`] when ownership of
    /// cancellation should remain separate from workers.
    pub fn request(&self) -> bool {
        self.state.cancel(CancellationReason::Requested)
    }

    /// Checks cancellation and returns the supplied value unchanged.
    pub fn checkpoint<T>(&self, value: T) -> QecResult<T> {
        self.check()?;
        Ok(value)
    }

    /// Cheap polling entry point for decoder loops.
    #[inline]
    pub fn poll(&self) -> QecResult<()> {
        self.check()
    }

    /// Sleeps until cancellation or the requested duration.
    ///
    /// If a deadline occurs first, the effective wait is shortened.
    pub fn sleep_or_cancel(
        &self,
        duration: Duration,
    ) -> QecResult<()> {
        self.check()?;

        let effective_duration = match self.remaining() {
            Some(remaining) => remaining.min(duration),
            None => duration,
        };

        let guard = self
            .state
            .wait_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if self.is_cancelled() {
            drop(guard);
            return self.check();
        }

        let (guard, _) = self
            .state
            .wait_condvar
            .wait_timeout(guard, effective_duration)
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        drop(guard);

        self.check()
    }

    /// Adds a deadline without extending an existing one.
    pub fn with_timeout_from_now(
        &self,
        duration: Duration,
    ) -> Self {
        let candidate = Instant::now().checked_add(duration);

        let deadline = match (self.deadline, candidate) {
            (Some(existing), Some(candidate)) => {
                Some(existing.min(candidate))
            }
            (existing, candidate) => existing.or(candidate),
        };

        Self {
            state: Arc::clone(&self.state),
            parent: self.parent.clone(),
            deadline,
        }
    }

    /// Creates an independently cancellable child context.
    pub fn child(&self) -> ChildCancellation {
        ChildCancellation::new(self.clone())
    }

    /// Returns whether this token has a deadline.
    pub const fn has_deadline(&self) -> bool {
        self.deadline.is_some()
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Child cancellation
// -----------------------------------------------------------------------------

/// Independently cancellable child context.
///
/// Parent cancellation remains authoritative, while child cancellation is
/// local to the child operation.
#[derive(Clone, Debug)]
pub struct ChildCancellation {
    parent: CancellationToken,
    source: CancellationSource,
    token: CancellationToken,
}

impl ChildCancellation {
    /// Creates a child context.
    pub fn new(parent: CancellationToken) -> Self {
        let source = CancellationSource::new();

        let token = source
            .token()
            .with_parent(parent.clone());

        Self {
            parent,
            source,
            token,
        }
    }

    /// Returns the child token.
    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Cancels only the child.
    pub fn cancel(&self) -> bool {
        self.source.cancel()
    }

    /// Cancels only the child with an explicit reason.
    pub fn cancel_with_reason(
        &self,
        reason: CancellationReason,
    ) -> bool {
        self.source.cancel_with_reason(reason)
    }

    /// Checks parent and child state.
    pub fn check(&self) -> QecResult<()> {
        self.parent.check()?;
        self.token.check()
    }

    /// Returns whether either parent or child is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.parent.is_cancelled()
            || self.token.is_cancelled()
    }

    /// Returns the effective reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        if self.parent.is_cancelled() {
            self.parent
                .reason()
                .or(Some(CancellationReason::ParentCancelled))
        } else {
            self.token.reason()
        }
    }
}

// -----------------------------------------------------------------------------
// Cancellation budget
// -----------------------------------------------------------------------------

/// Bounds cancellation-checkpoint consumption.
///
/// This is intentionally separate from QEC algorithmic resource limits:
/// `QecLimits::decoder_iterations` should govern work, while this budget
/// governs cancellation polling overhead.
#[derive(Debug)]
pub struct CancellationBudget {
    maximum_checks: Option<u64>,
    checks: AtomicU64,
}

impl CancellationBudget {
    /// Creates an unlimited budget.
    pub const fn unlimited() -> Self {
        Self {
            maximum_checks: None,
            checks: AtomicU64::new(0),
        }
    }

    /// Creates a bounded checkpoint budget.
    pub const fn new(maximum_checks: u64) -> Self {
        Self {
            maximum_checks: Some(maximum_checks),
            checks: AtomicU64::new(0),
        }
    }

    /// Number of checkpoints consumed.
    pub fn checks(&self) -> u64 {
        self.checks.load(Ordering::Relaxed)
    }

    /// Configured checkpoint limit.
    pub const fn maximum_checks(&self) -> Option<u64> {
        self.maximum_checks
    }

    /// Performs a budget-aware cancellation check.
    pub fn check(
        &self,
        token: &CancellationToken,
    ) -> QecResult<()> {
        token.check()?;

        let current = self
            .checks
            .fetch_add(1, Ordering::AcqRel)
            .saturating_add(1);

        if let Some(limit) = self.maximum_checks {
            if current > limit {
                return Err(QecError::cancelled(format!(
                    "QEC cancellation checkpoint budget exceeded: \
                     {current} > {limit}"
                )));
            }
        }

        Ok(())
    }

    /// Resets consumed checkpoint count.
    pub fn reset(&self) {
        self.checks.store(0, Ordering::Release);
    }
}

// -----------------------------------------------------------------------------
// Polling policy
// -----------------------------------------------------------------------------

/// Controls cancellation polling frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollingPolicy {
    /// Poll every N iterations.
    pub every_iterations: u64,

    /// Also poll when this wall-clock interval expires.
    pub interval: Option<Duration>,
}

impl PollingPolicy {
    /// Production-oriented default.
    pub const DEFAULT: Self = Self {
        every_iterations: 1_024,
        interval: Some(Duration::from_millis(10)),
    };

    /// Poll every iteration.
    pub const EVERY_ITERATION: Self = Self {
        every_iterations: 1,
        interval: None,
    };

    /// Creates an iteration-based policy.
    pub const fn every_iterations(
        every_iterations: u64,
    ) -> Self {
        Self {
            every_iterations: if every_iterations == 0 {
                1
            } else {
                every_iterations
            },
            interval: None,
        }
    }

    /// Determines whether polling should occur.
    #[inline]
    pub fn should_poll(
        &self,
        iteration: u64,
        last_poll: Instant,
    ) -> bool {
        if iteration == 0 {
            return true;
        }

        if iteration % self.every_iterations.max(1) == 0 {
            return true;
        }

        self.interval
            .is_some_and(|interval| {
                last_poll.elapsed() >= interval
            })
    }
}

impl Default for PollingPolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// -----------------------------------------------------------------------------
// Polling controller
// -----------------------------------------------------------------------------

/// Stateful polling helper for hot decoder loops.
#[derive(Debug)]
pub struct CancellationPoller {
    policy: PollingPolicy,
    last_poll: Instant,
    iterations: u64,
}

impl CancellationPoller {
    /// Creates a poller.
    pub fn new(policy: PollingPolicy) -> Self {
        Self {
            policy,
            last_poll: Instant::now(),
            iterations: 0,
        }
    }

    /// Conditionally checks cancellation.
    #[inline]
    pub fn poll(
        &mut self,
        token: &CancellationToken,
    ) -> QecResult<()> {
        let iteration = self.iterations;

        self.iterations = self
            .iterations
            .saturating_add(1);

        if self.policy.should_poll(
            iteration,
            self.last_poll,
        ) {
            token.check()?;
            self.last_poll = Instant::now();
        }

        Ok(())
    }

    /// Forces an immediate check.
    pub fn force_poll(
        &mut self,
        token: &CancellationToken,
    ) -> QecResult<()> {
        token.check()?;
        self.last_poll = Instant::now();
        Ok(())
    }

    /// Number of iterations observed.
    pub fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Resets the polling controller.
    pub fn reset(&mut self) {
        self.iterations = 0;
        self.last_poll = Instant::now();
    }
}

// -----------------------------------------------------------------------------
// Scoped cancellation guard
// -----------------------------------------------------------------------------

/// Cancels its child source when dropped unless disarmed.
#[derive(Debug)]
pub struct CancellationGuard {
    source: CancellationSource,
    cancel_on_drop: bool,
}

impl CancellationGuard {
    pub fn new(source: CancellationSource) -> Self {
        Self {
            source,
            cancel_on_drop: true,
        }
    }

    pub fn source(&self) -> &CancellationSource {
        &self.source
    }

    /// Prevents automatic cancellation on drop.
    pub fn disarm(&mut self) {
        self.cancel_on_drop = false;
    }

    /// Explicitly cancels the guarded operation.
    pub fn cancel(&self) {
        let _ = self.source.cancel_with_reason(
            CancellationReason::Superseded,
        );
    }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            let _ = self.source.cancel_with_reason(
                CancellationReason::Superseded,
            );
        }
    }
}

// -----------------------------------------------------------------------------
// Cancellable execution helper
// -----------------------------------------------------------------------------

/// Executes an operation inside a cancellation boundary.
///
/// The operation receives the token and is responsible for polling it at
/// internal safe points. A final check prevents returning a successful result
/// after cancellation occurred during the final part of the operation.
pub fn run_cancellable<T, F>(
    token: &CancellationToken,
    operation: F,
) -> QecResult<T>
where
    F: FnOnce(&CancellationToken) -> QecResult<T>,
{
    token.check()?;

    let result = operation(token)?;

    token.check()?;

    Ok(result)
}

// -----------------------------------------------------------------------------
// Thread lifecycle helper
// -----------------------------------------------------------------------------

/// Joins a worker while respecting cooperative cancellation.
///
/// Rust does not provide safe forceful thread termination. If the worker does
/// not terminate within `wait`, cancellation is requested and the function
/// returns a cancellation error.
///
/// The worker itself must honor the supplied cancellation token.
pub fn join_or_cancel<T>(
    handle: thread::JoinHandle<T>,
    source: &CancellationSource,
    wait: Duration,
) -> QecResult<T>
where
    T: Send + 'static,
{
    if source.is_cancelled() {
        if handle.is_finished() {
            return handle.join().map_err(|_| {
                QecError::invariant(
                    "worker_thread_join",
                    "QEC worker thread terminated with a panic",
                )
            });
        }

        return Err(QecError::cancelled(
            "worker join refused because cancellation \
             was already requested",
        ));
    }

    let started = Instant::now();

    while !handle.is_finished() {
        if started.elapsed() >= wait {
            let _ = source.cancel_with_reason(
                CancellationReason::DeadlineExceeded,
            );

            return Err(QecError::cancelled(
                "worker did not terminate within the \
                 configured cancellation wait interval",
            ));
        }

        thread::sleep(Duration::from_millis(1));
    }

    handle.join().map_err(|_| {
        QecError::invariant(
            "worker_thread_join",
            "QEC worker thread terminated with a panic",
        )
    })
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    #[test]
    fn source_starts_active() {
        let source = CancellationSource::new();

        assert!(!source.is_cancelled());
        assert!(source.reason().is_none());
    }

    #[test]
    fn cancellation_is_idempotent() {
        let source = CancellationSource::new();

        assert!(source.cancel());
        assert!(!source.cancel());

        assert!(source.is_cancelled());
        assert_eq!(source.generation(), 1);
    }

    #[test]
    fn cancellation_produces_qec_error() {
        let (source, token) =
            CancellationSource::new_pair();

        source.cancel();

        let error = token
            .check()
            .expect_err(
                "cancelled token must fail",
            );

        assert!(error.is_cancellation());
        assert_eq!(
            error.code(),
            "QEC-CANCEL-001"
        );
    }

    #[test]
    fn custom_reason_is_preserved() {
        let source = CancellationSource::new();

        source.cancel_with_reason(
            CancellationReason::SchedulerShutdown,
        );

        assert_eq!(
            source.reason(),
            Some(
                CancellationReason::SchedulerShutdown
            )
        );
    }

    #[test]
    fn deadline_materializes_cancellation_state() {
        let token =
            CancellationToken::with_timeout(
                Duration::from_millis(1),
            );

        thread::sleep(Duration::from_millis(5));

        assert!(token.is_cancelled());

        assert_eq!(
            token.reason(),
            Some(
                CancellationReason::DeadlineExceeded
            )
        );

        assert!(token.generation() >= 1);
    }

    #[test]
    fn parent_cancellation_is_observed_by_child() {
        let parent =
            CancellationSource::new();

        let child =
            parent.token().child();

        parent.cancel_with_reason(
            CancellationReason::SchedulerShutdown,
        );

        assert!(child.is_cancelled());

        assert_eq!(
            child.reason(),
            Some(
                CancellationReason::SchedulerShutdown
            )
        );
    }

    #[test]
    fn child_cancellation_does_not_cancel_parent() {
        let parent =
            CancellationSource::new();

        let child =
            parent.token().child();

        assert!(child.cancel());

        assert!(child.is_cancelled());
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn timeout_never_extends_parent_deadline() {
        let token =
            CancellationToken::with_timeout(
                Duration::from_secs(1),
            );

        let shortened =
            token.with_timeout_from_now(
                Duration::from_millis(1),
            );

        assert!(
            shortened
                .remaining()
                .expect("deadline exists")
                <= Duration::from_millis(1)
        );
    }

    #[test]
    fn callback_runs_once() {
        let source =
            CancellationSource::new();

        let calls =
            Arc::new(AtomicUsize::new(0));

        let calls_clone =
            Arc::clone(&calls);

        source.on_cancel(move |_| {
            calls_clone.fetch_add(
                1,
                Ordering::SeqCst,
            );
        });

        assert!(source.cancel());
        assert!(!source.cancel());

        assert_eq!(
            calls.load(Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn callback_registered_after_cancel_runs_immediately() {
        let source =
            CancellationSource::new();

        source.cancel();

        let calls =
            Arc::new(AtomicUsize::new(0));

        let calls_clone =
            Arc::clone(&calls);

        source.on_cancel(move |_| {
            calls_clone.fetch_add(
                1,
                Ordering::SeqCst,
            );
        });

        assert_eq!(
            calls.load(Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn budget_enforces_limit() {
        let token =
            CancellationToken::new();

        let budget =
            CancellationBudget::new(1);

        assert!(
            budget.check(&token).is_ok()
        );

        assert!(
            budget.check(&token).is_err()
        );
    }

    #[test]
    fn poller_polls_first_iteration() {
        let token =
            CancellationToken::new();

        let mut poller =
            CancellationPoller::new(
                PollingPolicy::every_iterations(
                    100
                )
            );

        assert!(
            poller.poll(&token).is_ok()
        );

        assert_eq!(
            poller.iterations(),
            1
        );
    }

    #[test]
    fn guard_cancels_on_drop() {
        let source =
            CancellationSource::new();

        {
            let _guard =
                CancellationGuard::new(
                    source.clone()
                );
        }

        assert!(source.is_cancelled());

        assert_eq!(
            source.reason(),
            Some(
                CancellationReason::Superseded
            )
        );
    }

    #[test]
    fn guard_can_be_disarmed() {
        let source =
            CancellationSource::new();

        {
            let mut guard =
                CancellationGuard::new(
                    source.clone()
                );

            guard.disarm();
        }

        assert!(!source.is_cancelled());
    }

    #[test]
    fn run_cancellable_checks_before_and_after() {
        let source =
            CancellationSource::new();

        let token =
            source.token();

        let result =
            run_cancellable(
                &token,
                |_| Ok::<_, QecError>(42),
            );

        assert_eq!(
            result.expect("operation succeeds"),
            42
        );
    }

    #[test]
    fn cancellation_metadata_is_stable() {
        let source =
            CancellationSource::new();

        source.cancel_with_reason(
            CancellationReason::Custom(
                "test".into()
            )
        );

        let metadata =
            source
                .state
                .metadata()
                .expect("metadata");

        assert_eq!(
            metadata.generation,
            1
        );

        assert!(
            metadata.requested_at.elapsed()
                < Duration::from_secs(1)
        );
    }
}