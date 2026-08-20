//! Cooperative cancellation infrastructure for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! `cancellation.rs` owns cancellation state and propagation only.
//!
//! It does NOT own:
//! - QEC workload limits;
//! - memory allocation;
//! - resource accounting;
//! - scheduling policy;
//! - decoder policy;
//! - authorization;
//! - telemetry transport;
//! - checkpoint serialization.
//!
//! The dependency direction is:
//!
//! ```text
//! errors.rs
//!     │
//!     ▼
//! cancellation.rs
//!     │
//!     ├── decoder.rs
//!     ├── streaming.rs
//!     ├── partition.rs
//!     ├── distributed.rs
//!     ├── scheduler.rs
//!     ├── checkpoint.rs
//!     ├── simulation.rs
//!     └── QPU execution
//! ```
//!
//! `limits.rs` remains independent:
//!
//! ```text
//! limits.rs       = what work is permitted
//! resources.rs    = what work is being consumed
//! memory.rs       = allocation enforcement
//! cancellation.rs = whether work should stop
//! ```
//!
//! # Cancellation guarantees
//!
//! 1. Cancellation is cooperative.
//! 2. Cancellation is monotonic: active -> cancelled.
//! 3. Repeated cancellation is idempotent.
//! 4. Parent cancellation propagates to descendants.
//! 5. Child cancellation never cancels its parent.
//! 6. Deadlines never extend an existing deadline.
//! 7. Cancellation checks are deterministic.
//! 8. Cancellation callbacks execute at most once.
//! 9. Callback panics cannot prevent cancellation propagation.
//! 10. Cancellation errors use the canonical `QecError` boundary.
//! 11. Cancellation does not silently become success.
//! 12. Worker threads are never forcefully terminated.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and uses only stable standard
//! library facilities.

use core::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Condvar, Mutex, RwLock,
};
use std::thread;
use std::time::{Duration, Instant};

use super::errors::{QecError, QecResult};

// ============================================================================
// Cancellation reason
// ============================================================================

/// Reason why an operation became cancelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancellationReason {
    /// Explicit cancellation requested by the owner.
    Requested,

    /// Cancellation propagated from a parent operation.
    ParentCancelled,

    /// The operation deadline expired.
    DeadlineExceeded,

    /// The cancellation polling budget was exhausted.
    BudgetExceeded,

    /// The scheduler shut down the workload.
    SchedulerShutdown,

    /// A distributed coordinator shut down the workload.
    DistributedShutdown,

    /// The operation was superseded.
    Superseded,

    /// Application-defined cancellation.
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
            Self::Custom(message) => write!(f, "custom cancellation: {message}"),
            other => f.write_str(other.as_str()),
        }
    }
}

// ============================================================================
// Cancellation metadata
// ============================================================================

/// Immutable metadata describing the cancellation transition.
#[derive(Debug, Clone)]
pub struct CancellationMetadata {
    reason: CancellationReason,
    requested_at: Instant,
    generation: u64,
}

impl CancellationMetadata {
    /// Returns the cancellation reason.
    pub fn reason(&self) -> &CancellationReason {
        &self.reason
    }

    /// Returns when cancellation was committed.
    pub fn requested_at(&self) -> Instant {
        self.requested_at
    }

    /// Returns the cancellation generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

// ============================================================================
// Shared state
// ============================================================================

type CancellationCallback =
    Arc<dyn Fn(CancellationReason) + Send + Sync + 'static>;

struct CancellationState {
    cancelled: AtomicBool,
    generation: AtomicU64,

    metadata: RwLock<Option<CancellationMetadata>>,

    wait_lock: Mutex<()>,
    wait_condvar: Condvar,

    callbacks: Mutex<Vec<CancellationCallback>>,
}

impl fmt::Debug for CancellationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metadata = self
            .metadata
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();

        let callback_count = self
            .callbacks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len();

        f.debug_struct("CancellationState")
            .field(
                "cancelled",
                &self.cancelled.load(Ordering::Acquire),
            )
            .field(
                "generation",
                &self.generation.load(Ordering::Acquire),
            )
            .field("metadata", &metadata)
            .field("callback_count", &callback_count)
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

    /// Commits cancellation exactly once.
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

        // State is committed before callbacks execute.
        self.wait_condvar.notify_all();

        let callbacks = {
            let mut guard = self
                .callbacks
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            std::mem::take(&mut *guard)
        };

        for callback in callbacks {
            let callback_reason = reason.clone();

            let _ = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    callback(callback_reason);
                }),
            );
        }

        true
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn metadata(&self) -> Option<CancellationMetadata> {
        self.metadata
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn reason(&self) -> Option<CancellationReason> {
        self.metadata().map(|metadata| metadata.reason)
    }

    fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    fn register_callback(&self, callback: CancellationCallback) {
        let immediate_reason = {
            let mut callbacks = self
                .callbacks
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if self.is_cancelled() {
                self.reason()
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
}

// ============================================================================
// Cancellation source
// ============================================================================

/// Owner-side cancellation controller.
///
/// A source is normally retained by the caller, scheduler, supervisor, or
/// coordinator. Workers should normally receive only a token.
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
    /// Creates an active cancellation source.
    pub fn new() -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
        }
    }

    /// Creates a source and token together.
    pub fn new_pair() -> (Self, CancellationToken) {
        let source = Self::new();
        let token = source.token();

        (source, token)
    }

    /// Creates an observing token.
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            state: Arc::clone(&self.state),
            parent: None,
            deadline: None,
        }
    }

    /// Requests ordinary cancellation.
    pub fn cancel(&self) -> bool {
        self.cancel_with_reason(CancellationReason::Requested)
    }

    /// Requests cancellation with a specific reason.
    pub fn cancel_with_reason(&self, reason: CancellationReason) -> bool {
        self.state.cancel(reason)
    }

    /// Returns whether cancellation has been committed.
    pub fn is_cancelled(&self) -> bool {
        self.state.is_cancelled()
    }

    /// Returns the source cancellation reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        self.state.reason()
    }

    /// Returns complete cancellation metadata.
    pub fn metadata(&self) -> Option<CancellationMetadata> {
        self.state.metadata()
    }

    /// Returns the cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Registers a callback executed at most once.
    pub fn on_cancel<F>(&self, callback: F)
    where
        F: Fn(CancellationReason) + Send + Sync + 'static,
    {
        self.state.register_callback(Arc::new(callback));
    }

    /// Creates an independently cancellable child operation.
    pub fn child(&self) -> ChildCancellation {
        ChildCancellation::new(self.token())
    }
}

impl Default for CancellationSource {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Cancellation token
// ============================================================================

/// Cloneable cooperative cancellation token.
///
/// A token observes local cancellation, parent cancellation, and deadlines.
/// Expensive QEC operations should receive a token and poll it at safe
/// boundaries.
#[derive(Clone)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
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
    /// Creates a new active token.
    pub fn new() -> Self {
        CancellationSource::new().token()
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

    /// Creates a child token observing the supplied parent.
    fn with_parent(mut self, parent: CancellationToken) -> Self {
        self.deadline = match (self.deadline, parent.deadline) {
            (Some(child), Some(parent)) => Some(child.min(parent)),
            (child, parent) => child.or(parent),
        };

        self.parent = Some(parent.state);

        self
    }

    /// Returns whether this token is cancelled.
    pub fn is_cancelled(&self) -> bool {
        if self.state.is_cancelled() {
            return true;
        }

        if self
            .parent
            .as_ref()
            .is_some_and(|parent| parent.is_cancelled())
        {
            return true;
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
        if let Some(parent) = &self.parent {
            if parent.is_cancelled() {
                return Some(CancellationReason::ParentCancelled);
            }
        }

        if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
            && !self.state.is_cancelled()
        {
            let _ = self
                .state
                .cancel(CancellationReason::DeadlineExceeded);
        }

        self.state.reason()
    }

    /// Returns local cancellation metadata.
    pub fn metadata(&self) -> Option<CancellationMetadata> {
        self.state.metadata()
    }

    /// Returns this token's local cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Returns the effective deadline.
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Returns whether a deadline is configured.
    pub const fn has_deadline(&self) -> bool {
        self.deadline.is_some()
    }

    /// Returns remaining time until the effective deadline.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }

    /// Canonical cancellation boundary.
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

    /// Alias for `check`, suitable for hot loops.
    #[inline]
    pub fn poll(&self) -> QecResult<()> {
        self.check()
    }

    /// Checks cancellation and returns the supplied value.
    pub fn checkpoint<T>(&self, value: T) -> QecResult<T> {
        self.check()?;
        Ok(value)
    }

    /// Requests cancellation of the local token state.
    ///
    /// This exists for backwards compatibility and for locally owned tokens.
    /// Worker code should normally not call it; ownership should remain with
    /// `CancellationSource`.
    pub fn request(&self) -> bool {
        self.state.cancel(CancellationReason::Requested)
    }

    /// Creates a child cancellation context.
    pub fn child(&self) -> ChildCancellation {
        ChildCancellation::new(self.clone())
    }

    /// Creates a token with a deadline that is no later than the current one.
    pub fn with_timeout_from_now(&self, duration: Duration) -> Self {
        let candidate = Instant::now().checked_add(duration);

        let deadline = match (self.deadline, candidate) {
            (Some(existing), Some(candidate)) => Some(existing.min(candidate)),
            (existing, candidate) => existing.or(candidate),
        };

        Self {
            state: Arc::clone(&self.state),
            parent: self.parent.clone(),
            deadline,
        }
    }

    /// Sleeps until cancellation, deadline, or the requested duration.
    pub fn sleep_or_cancel(&self, duration: Duration) -> QecResult<()> {
        self.check()?;

        let effective_duration = self
            .remaining()
            .map(|remaining| remaining.min(duration))
            .unwrap_or(duration);

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
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Child cancellation
// ============================================================================

/// Independently cancellable child operation.
///
/// Parent cancellation is authoritative. Child cancellation is local.
#[derive(Clone, Debug)]
pub struct ChildCancellation {
    parent: CancellationToken,
    source: CancellationSource,
    token: CancellationToken,
}

impl ChildCancellation {
    /// Creates a child operation.
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

    /// Cancels only this child.
    pub fn cancel(&self) -> bool {
        self.source.cancel()
    }

    /// Cancels only this child with a reason.
    pub fn cancel_with_reason(&self, reason: CancellationReason) -> bool {
        self.source.cancel_with_reason(reason)
    }

    /// Returns whether parent or child is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.parent.is_cancelled() || self.token.is_cancelled()
    }

    /// Checks parent and child cancellation.
    pub fn check(&self) -> QecResult<()> {
        self.parent.check()?;
        self.token.check()
    }

    /// Returns the effective reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        if self.parent.is_cancelled() {
            Some(CancellationReason::ParentCancelled)
        } else {
            self.token.reason()
        }
    }

    /// Registers a child-local callback.
    pub fn on_cancel<F>(&self, callback: F)
    where
        F: Fn(CancellationReason) + Send + Sync + 'static,
    {
        self.source.on_cancel(callback);
    }
}

// ============================================================================
// Cancellation budget
// ============================================================================

/// Bounds the number of cancellation checkpoints.
///
/// This is intentionally independent from `QecLimits`.
///
/// ```text
/// QecLimits
///     = maximum permitted workload
///
/// CancellationBudget
///     = maximum cancellation polling/checkpoint overhead
/// ```
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

    /// Creates a bounded budget.
    pub const fn new(maximum_checks: u64) -> Self {
        Self {
            maximum_checks: Some(maximum_checks),
            checks: AtomicU64::new(0),
        }
    }

    /// Returns the configured maximum.
    pub const fn maximum_checks(&self) -> Option<u64> {
        self.maximum_checks
    }

    /// Returns consumed checks.
    pub fn checks(&self) -> u64 {
        self.checks.load(Ordering::Acquire)
    }

    /// Resets the budget.
    pub fn reset(&self) {
        self.checks.store(0, Ordering::Release);
    }

    /// Checks cancellation and consumes one budget unit.
    pub fn check(&self, token: &CancellationToken) -> QecResult<()> {
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
}

// ============================================================================
// Polling policy
// ============================================================================

/// Controls cancellation polling frequency in hot loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollingPolicy {
    /// Poll at least every N iterations.
    pub every_iterations: u64,

    /// Poll when this wall-clock interval expires.
    pub interval: Option<Duration>,
}

impl PollingPolicy {
    /// Production default.
    pub const DEFAULT: Self = Self {
        every_iterations: 1_024,
        interval: Some(Duration::from_millis(10)),
    };

    /// Maximum responsiveness.
    pub const EVERY_ITERATION: Self = Self {
        every_iterations: 1,
        interval: None,
    };

    /// Creates an iteration-based policy.
    pub const fn every_iterations(every_iterations: u64) -> Self {
        Self {
            every_iterations: if every_iterations == 0 {
                1
            } else {
                every_iterations
            },
            interval: None,
        }
    }

    /// Determines whether a poll is due.
    pub fn should_poll(&self, iteration: u64, last_poll: Instant) -> bool {
        if iteration == 0 {
            return true;
        }

        if iteration % self.every_iterations.max(1) == 0 {
            return true;
        }

        self.interval
            .is_some_and(|interval| last_poll.elapsed() >= interval)
    }
}

impl Default for PollingPolicy {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// ============================================================================
// Cancellation poller
// ============================================================================

/// Stateful polling helper for expensive loops.
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
    pub fn poll(&mut self, token: &CancellationToken) -> QecResult<()> {
        let iteration = self.iterations;

        self.iterations = self
            .iterations
            .saturating_add(1);

        if self.policy.should_poll(iteration, self.last_poll) {
            token.check()?;
            self.last_poll = Instant::now();
        }

        Ok(())
    }

    /// Forces an immediate check.
    pub fn force_poll(&mut self, token: &CancellationToken) -> QecResult<()> {
        token.check()?;
        self.last_poll = Instant::now();
        Ok(())
    }

    /// Returns observed iterations.
    pub fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Returns the polling policy.
    pub fn policy(&self) -> PollingPolicy {
        self.policy
    }

    /// Resets the poller.
    pub fn reset(&mut self) {
        self.iterations = 0;
        self.last_poll = Instant::now();
    }
}

// ============================================================================
// Scoped cancellation guard
// ============================================================================

/// Cancels a source when dropped unless disarmed.
///
/// Useful for scheduler jobs, temporary decoder workers, and speculative
/// execution.
#[derive(Debug)]
pub struct CancellationGuard {
    source: CancellationSource,
    cancel_on_drop: bool,
    reason: CancellationReason,
}

impl CancellationGuard {
    /// Creates a guard that cancels on drop.
    pub fn new(source: CancellationSource) -> Self {
        Self {
            source,
            cancel_on_drop: true,
            reason: CancellationReason::Superseded,
        }
    }

    /// Creates a guard with an explicit drop reason.
    pub fn with_reason(
        source: CancellationSource,
        reason: CancellationReason,
    ) -> Self {
        Self {
            source,
            cancel_on_drop: true,
            reason,
        }
    }

    /// Returns the guarded source.
    pub fn source(&self) -> &CancellationSource {
        &self.source
    }

    /// Prevents cancellation on drop.
    pub fn disarm(&mut self) {
        self.cancel_on_drop = false;
    }

    /// Explicitly cancels the guarded operation.
    pub fn cancel(&self) -> bool {
        self.source
            .cancel_with_reason(self.reason.clone())
    }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            let _ = self
                .source
                .cancel_with_reason(self.reason.clone());
        }
    }
}

// ============================================================================
// Cancellable execution helper
// ============================================================================

/// Runs an operation inside a cancellation boundary.
///
/// A cancellation check occurs both before and after the operation. This
/// prevents an operation that completed after cancellation from being reported
/// as successful.
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

// ============================================================================
// Worker lifecycle
// ============================================================================

/// Joins a worker while respecting a bounded cooperative wait.
///
/// Rust intentionally does not provide safe forceful thread termination.
///
/// If the worker does not finish before `wait`:
///
/// 1. cancellation is requested;
/// 2. the worker is allowed to observe it;
/// 3. a cancellation error is returned.
///
/// The worker must therefore use the supplied cancellation token.
pub fn join_or_cancel<T>(
    handle: thread::JoinHandle<T>,
    source: &CancellationSource,
    wait: Duration,
) -> QecResult<T>
where
    T: Send + 'static,
{
    if handle.is_finished() {
        return handle.join().map_err(|_| {
            QecError::invariant(
                "worker_thread_join",
                "QEC worker thread terminated with a panic",
            )
        });
    }

    if source.is_cancelled() {
        return Err(QecError::cancelled(
            "worker join aborted because cancellation was already requested",
        ));
    }

    let started = Instant::now();

    while !handle.is_finished() {
        if started.elapsed() >= wait {
            let _ = source.cancel_with_reason(
                CancellationReason::DeadlineExceeded,
            );

            return Err(QecError::cancelled(
                "worker did not terminate within the configured \
                 cancellation wait interval",
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn source_starts_active() {
        let source = CancellationSource::new();

        assert!(!source.is_cancelled());
        assert!(source.reason().is_none());
        assert_eq!(source.generation(), 0);
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
        let (source, token) = CancellationSource::new_pair();

        source.cancel();

        let error = token
            .check()
            .expect_err("cancelled token must fail");

        assert!(error.is_cancellation());
        assert_eq!(error.code(), "QEC-CANCEL-001");
    }

    #[test]
    fn custom_reason_is_preserved() {
        let source = CancellationSource::new();

        source.cancel_with_reason(
            CancellationReason::SchedulerShutdown,
        );

        assert_eq!(
            source.reason(),
            Some(CancellationReason::SchedulerShutdown)
        );
    }

    #[test]
    fn cancellation_metadata_is_recorded() {
        let source = CancellationSource::new();

        source.cancel();

        let metadata = source
            .metadata()
            .expect("metadata must exist after cancellation");

        assert_eq!(metadata.generation(), 1);
        assert_eq!(
            metadata.reason(),
            &CancellationReason::Requested
        );
    }

    #[test]
    fn deadline_materializes_local_cancellation() {
        let token = CancellationToken::with_timeout(
            Duration::from_millis(1),
        );

        thread::sleep(Duration::from_millis(5));

        assert!(token.is_cancelled());
        assert_eq!(
            token.reason(),
            Some(CancellationReason::DeadlineExceeded)
        );
        assert!(token.generation() >= 1);
    }

    #[test]
    fn parent_cancellation_reaches_child() {
        let parent = CancellationSource::new();
        let child = parent.token().child();

        parent.cancel_with_reason(
            CancellationReason::SchedulerShutdown,
        );

        assert!(child.is_cancelled());
        assert_eq!(
            child.reason(),
            Some(CancellationReason::ParentCancelled)
        );

        assert!(child.check().is_err());
    }

    #[test]
    fn child_cancellation_does_not_cancel_parent() {
        let parent = CancellationSource::new();
        let child = parent.token().child();

        assert!(child.cancel());

        assert!(child.is_cancelled());
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn deadline_never_extends() {
        let token = CancellationToken::with_timeout(
            Duration::from_secs(1),
        );

        let shortened = token.with_timeout_from_now(
            Duration::from_millis(1),
        );

        assert!(
            shortened
                .remaining()
                .expect("deadline must exist")
                <= Duration::from_millis(1)
        );
    }

    #[test]
    fn callback_runs_once() {
        let source = CancellationSource::new();

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        source.on_cancel(move |_| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(source.cancel());
        assert!(!source.cancel());

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn callback_registered_after_cancellation_runs_immediately() {
        let source = CancellationSource::new();

        source.cancel();

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        source.on_cancel(move |_| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn callback_panic_does_not_break_cancellation() {
        let source = CancellationSource::new();

        source.on_cancel(|_| {
            panic!("callback failure must not break cancellation");
        });

        assert!(source.cancel());
        assert!(source.is_cancelled());
    }

    #[test]
    fn cancellation_budget_enforces_limit() {
        let source = CancellationSource::new();
        let token = source.token();

        let budget = CancellationBudget::new(2);

        assert!(budget.check(&token).is_ok());
        assert!(budget.check(&token).is_ok());

        let result = budget.check(&token);

        assert!(result.is_err());
        assert_eq!(budget.checks(), 3);
    }

    #[test]
    fn cancellation_budget_can_reset() {
        let source = CancellationSource::new();
        let token = source.token();

        let budget = CancellationBudget::new(1);

        assert!(budget.check(&token).is_ok());
        assert!(budget.check(&token).is_err());

        budget.reset();

        assert_eq!(budget.checks(), 0);
        assert!(budget.check(&token).is_ok());
    }

    #[test]
    fn polling_policy_never_uses_zero_interval() {
        let policy = PollingPolicy::every_iterations(0);

        assert_eq!(policy.every_iterations, 1);
    }

    #[test]
    fn poller_counts_iterations() {
        let source = CancellationSource::new();
        let token = source.token();

        let mut poller =
            CancellationPoller::new(PollingPolicy::EVERY_ITERATION);

        assert!(poller.poll(&token).is_ok());
        assert!(poller.poll(&token).is_ok());

        assert_eq!(poller.iterations(), 2);
    }

    #[test]
    fn run_cancellable_returns_success_when_active() {
        let source = CancellationSource::new();
        let token = source.token();

        let value = run_cancellable(&token, |_| Ok(42))
            .expect("operation should succeed");

        assert_eq!(value, 42);
    }

    #[test]
    fn run_cancellable_rejects_pre_cancelled_operation() {
        let source = CancellationSource::new();
        let token = source.token();

        source.cancel();

        let result = run_cancellable(&token, |_| Ok(42));

        assert!(result.is_err());
    }

    #[test]
    fn guard_cancels_on_drop() {
        let source = CancellationSource::new();

        {
            let _guard = CancellationGuard::new(source.clone());
        }

        assert!(source.is_cancelled());
        assert_eq!(
            source.reason(),
            Some(CancellationReason::Superseded)
        );
    }

    #[test]
    fn guard_can_be_disarmed() {
        let source = CancellationSource::new();

        {
            let mut guard = CancellationGuard::new(source.clone());
            guard.disarm();
        }

        assert!(!source.is_cancelled());
    }

    #[test]
    fn child_callback_is_local() {
        let parent = CancellationSource::new();
        let child = parent.token().child();

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        child.on_cancel(move |_| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
        });

        child.cancel();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn sleep_or_cancel_returns_after_requested_duration() {
        let source = CancellationSource::new();
        let token = source.token();

        let started = Instant::now();

        token
            .sleep_or_cancel(Duration::from_millis(1))
            .expect("sleep should complete");

        assert!(started.elapsed() >= Duration::from_micros(500));
        assert!(!token.is_cancelled());
    }
}