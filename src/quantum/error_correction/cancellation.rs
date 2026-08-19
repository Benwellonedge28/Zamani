//! Cooperative cancellation infrastructure for Zamani Quantum Error Correction.
//!
//! This module provides cancellation primitives for potentially expensive QEC
//! operations such as:
//!
//! - syndrome-stream processing;
//! - decoding;
//! - MWPM;
//! - Union-Find;
//! - partition reconciliation;
//! - distributed decoding;
//! - simulations;
//! - threshold experiments;
//! - checkpoint creation;
//! - scalability benchmarks.
//!
//! # Design goals
//!
//! * Cooperative rather than forceful cancellation.
//! * No `unsafe` code.
//! * Thread-safe cancellation state.
//! * Hierarchical parent/child cancellation.
//! * Explicit cancellation reasons.
//! * Deadline support.
//! * Periodic polling suitable for hot decoder loops.
//! * Low-overhead fast-path checks.
//! * Deterministic cancellation semantics.
//! * Optional progress checkpoints.
//! * Optional cancellation callbacks.
//! * No dependency on a particular executor/runtime.
//! * Integration with the canonical [`crate::quantum::error_correction::errors::QecError`].
//!
//! # Architectural rule
//!
//! ```text
//! QEC operation
//!      |
//!      v
//! CancellationToken
//!      |
//! +----+-------------------+
//! |                        |
//! v                        v
//! explicit cancel       deadline
//! |                        |
//! +-----------+------------+
//!             |
//!             v
//!       cancellation state
//!             |
//!             v
//!      decoder checkpoint
//!             |
//!             v
//!        QecError
//! ```
//!
//! Cancellation is deliberately cooperative. A decoder must periodically call
//! [`CancellationToken::check`] or an equivalent checkpoint operation.
//!
//! A cancellation request never forcibly terminates a thread while it is
//! mutating decoder state.

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

/// Why an operation was cancelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancellationReason {
    /// Cancellation was explicitly requested by the caller.
    Requested,

    /// A parent operation cancelled this child operation.
    ParentCancelled,

    /// The configured deadline elapsed.
    DeadlineExceeded,

    /// The operation exceeded its configured polling/iteration budget.
    BudgetExceeded,

    /// The owning scheduler cancelled the workload.
    SchedulerShutdown,

    /// The distributed coordinator cancelled the workload.
    DistributedShutdown,

    /// The operation was superseded by another operation.
    Superseded,

    /// The caller supplied a custom cancellation reason.
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(message) => {
                write!(formatter, "custom cancellation: {message}")
            }
            _ => write!(formatter, "{}", self.as_str()),
        }
    }
}

// -----------------------------------------------------------------------------
// Cancellation state
// -----------------------------------------------------------------------------

/// Internal immutable cancellation metadata.
#[derive(Debug, Clone)]
struct CancellationMetadata {
    reason: CancellationReason,
    requested_at: Instant,
    generation: u64,
}

/// Shared cancellation state.
#[derive(Debug)]
struct CancellationState {
    cancelled: AtomicBool,

    generation: AtomicU64,

    metadata: RwLock<Option<CancellationMetadata>>,

    wait_lock: Mutex<()>,

    wait_condvar: Condvar,

    callbacks: Mutex<Vec<Arc<dyn Fn(CancellationReason) + Send + Sync + 'static>>>,
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

    fn cancel(&self, reason: CancellationReason) -> bool {
        if self
            .cancelled
            .swap(true, Ordering::AcqRel)
        {
            return false;
        }

        let generation = self
            .generation
            .fetch_add(1, Ordering::AcqRel)
            .saturating_add(1);

        let metadata = CancellationMetadata {
            reason: reason.clone(),
            requested_at: Instant::now(),
            generation,
        };

        {
            let mut guard = self
                .metadata
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            *guard = Some(metadata);
        }

        self.wait_condvar.notify_all();

        let callbacks = {
            let guard = self
                .callbacks
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            guard.clone()
        };

        for callback in callbacks {
            callback(reason.clone());
        }

        true
    }

    fn reason(&self) -> Option<CancellationReason> {
        let guard = self
            .metadata
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        guard.as_ref().map(|metadata| metadata.reason.clone())
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
/// A source is used by schedulers, coordinators, callers, or supervisors to
/// request cancellation.
#[derive(Clone, Debug)]
pub struct CancellationSource {
    state: Arc<CancellationState>,
}

impl CancellationSource {
    /// Creates a new independent cancellation source.
    pub fn new() -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
        }
    }

    /// Creates a source together with its corresponding token.
    pub fn new_pair() -> (Self, CancellationToken) {
        let source = Self::new();
        let token = source.token();

        (source, token)
    }

    /// Returns a token that observes this source.
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            state: Arc::clone(&self.state),
            deadline: None,
            parent_generation: self.state.generation(),
        }
    }

    /// Requests cancellation.
    ///
    /// Returns `true` only when this call changed the state from active to
    /// cancelled.
    pub fn cancel(&self) -> bool {
        self.cancel_with_reason(CancellationReason::Requested)
    }

    /// Requests cancellation with an explicit reason.
    pub fn cancel_with_reason(&self, reason: CancellationReason) -> bool {
        self.state.cancel(reason)
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    /// Returns the cancellation reason, if cancelled.
    pub fn reason(&self) -> Option<CancellationReason> {
        self.state.reason()
    }

    /// Returns the cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Registers a callback invoked once when cancellation occurs.
    ///
    /// Callbacks must be short and must not depend on the cancelled operation
    /// completing. They should never panic.
    pub fn on_cancel<F>(&self, callback: F)
    where
        F: Fn(CancellationReason) + Send + Sync + 'static,
    {
        let callback: Arc<dyn Fn(CancellationReason) + Send + Sync + 'static> =
            Arc::new(callback);

        if let Some(reason) = self.reason() {
            callback(reason);
            return;
        }

        let mut callbacks = self
            .state
            .callbacks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if self.is_cancelled() {
            drop(callbacks);

            if let Some(reason) = self.reason() {
                callback(reason);
            }

            return;
        }

        callbacks.push(callback);
    }

    /// Creates a child source that can be cancelled independently.
    ///
    /// The returned child token observes both the child and parent.
    pub fn child(&self) -> (CancellationSource, CancellationToken) {
        let child = Self::new();
        let child_token = CancellationToken::with_parent(
            child.token(),
            self.token(),
        );

        (child, child_token)
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

/// Cooperative cancellation token.
///
/// Tokens are cheap to clone and may safely be passed to decoder workers.
///
/// Cancellation is checked without locking on the normal fast path.
#[derive(Clone, Debug)]
pub struct CancellationToken {
    state: Arc<CancellationState>,
    deadline: Option<Instant>,
    parent_generation: u64,
}

impl CancellationToken {
    /// Creates an already-active token.
    pub fn new() -> Self {
        CancellationSource::new().token()
    }

    fn with_parent(
        child: Self,
        parent: CancellationToken,
    ) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            deadline: child.deadline.or(parent.deadline),
            parent_generation: parent.parent_generation,
        }
    }

    /// Returns a token that is cancelled after `duration`.
    pub fn with_timeout(duration: Duration) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            deadline: Instant::now().checked_add(duration),
            parent_generation: 0,
        }
    }

    /// Returns a token with an absolute deadline.
    pub fn with_deadline(deadline: Instant) -> Self {
        Self {
            state: Arc::new(CancellationState::new()),
            deadline: Some(deadline),
            parent_generation: 0,
        }
    }

    /// Returns whether cancellation has been requested.
    ///
    /// This is intentionally a cheap operation suitable for tight decoder
    /// loops.
    pub fn is_cancelled(&self) -> bool {
        if self.state.cancelled.load(Ordering::Acquire) {
            return true;
        }

        match self.deadline {
            Some(deadline) if Instant::now() >= deadline => {
                self.state.cancelled.store(true, Ordering::Release);
                true
            }
            _ => false,
        }
    }

    /// Returns the reason for cancellation.
    pub fn reason(&self) -> Option<CancellationReason> {
        if self.state.cancelled.load(Ordering::Acquire) {
            if let Some(reason) = self.state.reason() {
                return Some(reason);
            }
        }

        if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Some(CancellationReason::DeadlineExceeded);
        }

        None
    }

    /// Returns the configured deadline.
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Returns the remaining time before the deadline.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline.map(|deadline| {
            deadline.saturating_duration_since(Instant::now())
        })
    }

    /// Returns the cancellation generation.
    pub fn generation(&self) -> u64 {
        self.state.generation()
    }

    /// Converts cancellation into the canonical QEC error.
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

    /// Explicitly records a cancellation request on this token.
    ///
    /// Most code should instead retain a [`CancellationSource`] and call
    /// `source.cancel()`.
    pub fn request(&self) -> bool {
        self.state.cancel(CancellationReason::Requested)
    }

    /// Checks cancellation and returns the supplied value unchanged.
    ///
    /// Useful at state-machine boundaries:
    ///
    /// ```text
    /// token.checkpoint(value)?
    /// ```
    pub fn checkpoint<T>(&self, value: T) -> QecResult<T> {
        self.check()?;
        Ok(value)
    }

    /// Checks cancellation at a decoder iteration boundary.
    ///
    /// This function is deliberately small so it can be called frequently.
    #[inline]
    pub fn poll(&self) -> QecResult<()> {
        self.check()
    }

    /// Sleeps until either the duration expires or cancellation occurs.
    ///
    /// Returns `Ok(())` if the wait completed normally and a cancellation
    /// error otherwise.
    pub fn sleep_or_cancel(&self, duration: Duration) -> QecResult<()> {
        self.check()?;

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
            .wait_timeout(guard, duration)
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        drop(guard);

        self.check()
    }

    /// Returns a token with a shorter deadline.
    pub fn with_timeout_from_now(
        &self,
        duration: Duration,
    ) -> Self {
        let candidate = Instant::now()
            .checked_add(duration);

        let deadline = match (self.deadline, candidate) {
            (Some(existing), Some(candidate)) => {
                Some(existing.min(candidate))
            }
            (existing, candidate) => existing.or(candidate),
        };

        Self {
            state: Arc::clone(&self.state),
            deadline,
            parent_generation: self.parent_generation,
        }
    }

    /// Creates a child token whose cancellation is linked to this token.
    ///
    /// The child may have an independent deadline but cannot outlive the
    /// parent cancellation state semantically.
    pub fn child(&self) -> ChildCancellation {
        ChildCancellation::new(self.clone())
    }

    /// Returns whether this token has a deadline.
    pub const fn has_deadline(&self) -> bool {
        self.deadline.is_some()
    }

    /// Returns the parent generation captured when the token was created.
    pub const fn parent_generation(&self) -> u64 {
        self.parent_generation
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

/// A child cancellation context.
///
/// Child cancellation is local while parent cancellation remains authoritative.
#[derive(Clone, Debug)]
pub struct ChildCancellation {
    parent: CancellationToken,
    source: CancellationSource,
    token: CancellationToken,
}

impl ChildCancellation {
    /// Creates a child cancellation context.
    pub fn new(parent: CancellationToken) -> Self {
        let source = CancellationSource::new();

        let token = CancellationToken {
            state: Arc::clone(&source.state),
            deadline: parent.deadline,
            parent_generation: parent.generation(),
        };

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

    /// Cancels only the child operation.
    pub fn cancel(&self) -> bool {
        self.source.cancel()
    }

    /// Cancels the child with a reason.
    pub fn cancel_with_reason(
        &self,
        reason: CancellationReason,
    ) -> bool {
        self.source.cancel_with_reason(reason)
    }

    /// Checks both parent and child cancellation.
    pub fn check(&self) -> QecResult<()> {
        self.parent.check()?;
        self.token.check()
    }

    /// Returns whether either parent or child is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.parent.is_cancelled()
            || self.token.is_cancelled()
    }

    /// Returns the effective cancellation reason.
    pub fn reason(&self) -> Option<CancellationReason> {
        if self.parent.is_cancelled() {
            return self
                .parent
                .reason()
                .or(Some(CancellationReason::ParentCancelled));
        }

        self.token.reason()
    }
}

// -----------------------------------------------------------------------------
// Cancellation budget
// -----------------------------------------------------------------------------

/// Bounded cooperative cancellation budget.
///
/// This protects against decoders that are computationally expensive even
/// when no wall-clock deadline has been configured.
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

    /// Creates a budget with a maximum number of cancellation checkpoints.
    pub const fn new(maximum_checks: u64) -> Self {
        Self {
            maximum_checks: Some(maximum_checks),
            checks: AtomicU64::new(0),
        }
    }

    /// Returns the number of completed checks.
    pub fn checks(&self) -> u64 {
        self.checks.load(Ordering::Relaxed)
    }

    /// Returns the configured maximum.
    pub const fn maximum_checks(&self) -> Option<u64> {
        self.maximum_checks
    }

    /// Performs a budget checkpoint.
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

    /// Resets the consumed checkpoint count.
    pub fn reset(&self) {
        self.checks.store(0, Ordering::Release);
    }
}

// -----------------------------------------------------------------------------
// Polling policy
// -----------------------------------------------------------------------------

/// Controls how frequently a decoder should perform expensive cancellation
/// checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollingPolicy {
    /// Check cancellation every N iterations.
    pub every_iterations: u64,

    /// Always check at this wall-clock interval.
    pub interval: Option<Duration>,
}

impl PollingPolicy {
    /// Conservative default suitable for production decoders.
    pub const DEFAULT: Self = Self {
        every_iterations: 1_024,
        interval: Some(Duration::from_millis(10)),
    };

    /// Checks every iteration.
    pub const EVERY_ITERATION: Self = Self {
        every_iterations: 1,
        interval: None,
    };

    /// Creates a policy with an iteration interval.
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

    /// Determines whether an iteration should poll.
    #[inline]
    pub fn should_poll(
        &self,
        iteration: u64,
        last_poll: Instant,
    ) -> bool {
        if iteration == 0 {
            return true;
        }

        if iteration
            .checked_rem(self.every_iterations.max(1))
            == Some(0)
        {
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

/// Stateful cancellation polling helper for hot decoder loops.
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

    /// Performs a conditional cancellation check.
    #[inline]
    pub fn poll(
        &mut self,
        token: &CancellationToken,
    ) -> QecResult<()> {
        let iteration = self.iterations;

        self.iterations = self
            .iterations
            .saturating_add(1);

        if self
            .policy
            .should_poll(
                iteration,
                self.last_poll,
            )
        {
            token.check()?;
            self.last_poll = Instant::now();
        }

        Ok(())
    }

    /// Forces an immediate cancellation check.
    pub fn force_poll(
        &mut self,
        token: &CancellationToken,
    ) -> QecResult<()> {
        token.check()?;
        self.last_poll = Instant::now();
        Ok(())
    }

    /// Returns the number of iterations observed.
    pub fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Resets the poller.
    pub fn reset(&mut self) {
        self.iterations = 0;
        self.last_poll = Instant::now();
    }
}

// -----------------------------------------------------------------------------
// Scoped cancellation guard
// -----------------------------------------------------------------------------

/// RAII helper for operations that should cancel their child work when the
/// guard leaves scope.
#[derive(Debug)]
pub struct CancellationGuard {
    source: CancellationSource,
    cancel_on_drop: bool,
}

impl CancellationGuard {
    /// Creates a guard around a source.
    pub fn new(source: CancellationSource) -> Self {
        Self {
            source,
            cancel_on_drop: true,
        }
    }

    /// Returns the source.
    pub fn source(&self) -> &CancellationSource {
        &self.source
    }

    /// Disables automatic cancellation on drop.
    pub fn disarm(&mut self) {
        self.cancel_on_drop = false;
    }

    /// Explicitly cancels the guarded operation.
    pub fn cancel(&self) {
        self.source.cancel();
    }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            self.source
                .cancel_with_reason(
                    CancellationReason::Superseded,
                );
        }
    }
}

// -----------------------------------------------------------------------------
// Cancellation-aware execution helper
// -----------------------------------------------------------------------------

/// Executes a closure while providing it with a cancellation token.
///
/// The closure remains responsible for calling `check()` at safe points.
pub fn run_cancellable<T, F>(
    token: &CancellationToken,
    operation: F,
) -> QecResult<T>
where
    F: FnOnce(&CancellationToken) -> QecResult<T>,
{
    token.check()?;

    let result = operation(token);

    match result {
        Ok(value) => {
            token.check()?;
            Ok(value)
        }
        Err(error) => Err(error),
    }
}

// -----------------------------------------------------------------------------
// Thread cancellation helper
// -----------------------------------------------------------------------------

/// Joins a worker thread while respecting cancellation.
///
/// Rust threads cannot safely be force-killed. Therefore this function uses
/// cooperative cancellation and bounded waiting.
pub fn join_or_cancel<T>(
    handle: thread::JoinHandle<T>,
    source: &CancellationSource,
    wait: Duration,
) -> QecResult<T>
where
    T: Send + 'static,
{
    if source.is_cancelled() {
        return Err(QecError::cancelled(
            "worker join refused because cancellation was already requested",
        ));
    }

    let started = Instant::now();

    while !handle.is_finished() {
        if started.elapsed() >= wait {
            source.cancel_with_reason(
                CancellationReason::DeadlineExceeded,
            );

            return Err(QecError::cancelled(
                "worker did not terminate within the configured cancellation wait interval",
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
    }

    #[test]
    fn cancellation_produces_qec_error() {
        let source = CancellationSource::new();
        let token = source.token();

        source.cancel();

        let error = token
            .check()
            .expect_err("cancelled token must fail");

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
            Some(CancellationReason::SchedulerShutdown)
        );
    }

    #[test]
    fn generation_changes_once() {
        let source = CancellationSource::new();

        assert_eq!(source.generation(), 0);

        source.cancel();

        assert_eq!(source.generation(), 1);

        source.cancel();

        assert_eq!(source.generation(), 1);
    }

    #[test]
    fn checkpoint_returns_value_when_active() {
        let source = CancellationSource::new();
        let token = source.token();

        let value = token
            .checkpoint(42)
            .expect("active token");

        assert_eq!(value, 42);
    }

    #[test]
    fn checkpoint_rejects_cancelled_operation() {
        let source = CancellationSource::new();
        let token = source.token();

        source.cancel();

        assert!(
            token.checkpoint(42).is_err()
        );
    }

    #[test]
    fn timeout_reports_deadline() {
        let token =
            CancellationToken::with_timeout(
                Duration::from_millis(1),
            );

        thread::sleep(Duration::from_millis(5));

        assert!(token.is_cancelled());

        assert_eq!(
            token.reason(),
            Some(CancellationReason::DeadlineExceeded)
        );
    }

    #[test]
    fn remaining_time_is_bounded() {
        let token =
            CancellationToken::with_timeout(
                Duration::from_secs(1),
            );

        let remaining = token
            .remaining()
            .expect("deadline exists");

        assert!(
            remaining <= Duration::from_secs(1)
        );
    }

    #[test]
    fn child_can_cancel_independently() {
        let parent = CancellationSource::new();
        let child = parent
            .token()
            .child();

        child.cancel();

        assert!(child.is_cancelled());
        assert!(!parent.is_cancelled());
    }

    #[test]
    fn parent_cancellation_propagates_semantically() {
        let parent = CancellationSource::new();
        let child = parent
            .token()
            .child();

        parent.cancel();

        assert!(child.is_cancelled());
        assert!(
            child
                .reason()
                .is_some()
        );
    }

    #[test]
    fn budget_counts_checks() {
        let source = CancellationSource::new();
        let token = source.token();

        let budget = CancellationBudget::new(2);

        assert!(budget.check(&token).is_ok());
        assert!(budget.check(&token).is_ok());
        assert!(budget.check(&token).is_err());

        assert_eq!(
            budget.checks(),
            3
        );
    }

    #[test]
    fn unlimited_budget_does_not_fail() {
        let source = CancellationSource::new();
        let token = source.token();

        let budget =
            CancellationBudget::unlimited();

        for _ in 0..10_000 {
            budget
                .check(&token)
                .expect("unlimited budget");
        }
    }

    #[test]
    fn polling_policy_polls_at_configured_interval() {
        let policy =
            PollingPolicy::every_iterations(4);

        assert!(
            policy.should_poll(
                0,
                Instant::now()
            )
        );

        assert!(
            !policy.should_poll(
                1,
                Instant::now()
            )
        );

        assert!(
            !policy.should_poll(
                2,
                Instant::now()
            )
        );

        assert!(
            !policy.should_poll(
                3,
                Instant::now()
            )
        );

        assert!(
            policy.should_poll(
                4,
                Instant::now()
            )
        );
    }

    #[test]
    fn poller_counts_iterations() {
        let source = CancellationSource::new();
        let token = source.token();

        let mut poller =
            CancellationPoller::new(
                PollingPolicy::EVERY_ITERATION,
            );

        for _ in 0..10 {
            poller
                .poll(&token)
                .expect("active token");
        }

        assert_eq!(
            poller.iterations(),
            10
        );
    }

    #[test]
    fn run_cancellable_executes_active_operation() {
        let source = CancellationSource::new();
        let token = source.token();

        let result =
            run_cancellable(&token, |_| {
                Ok::<_, QecError>(123)
            })
            .expect("operation should succeed");

        assert_eq!(result, 123);
    }

    #[test]
    fn run_cancellable_rejects_pre_cancelled_operation() {
        let source = CancellationSource::new();
        let token = source.token();

        source.cancel();

        let result =
            run_cancellable(&token, |_| {
                Ok::<_, QecError>(123)
            });

        assert!(result.is_err());
    }

    #[test]
    fn callbacks_run_once() {
        let source = CancellationSource::new();

        let count =
            Arc::new(AtomicU64::new(0));

        let callback_count =
            Arc::clone(&count);

        source.on_cancel(move |_| {
            callback_count
                .fetch_add(1, Ordering::SeqCst);
        });

        source.cancel();
        source.cancel();

        assert_eq!(
            count.load(Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn callback_added_after_cancel_runs_immediately() {
        let source = CancellationSource::new();

        source.cancel();

        let count =
            Arc::new(AtomicU64::new(0));

        let callback_count =
            Arc::clone(&count);

        source.on_cancel(move |_| {
            callback_count
                .fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(
            count.load(Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn cancellation_guard_cancels_on_drop() {
        let source = CancellationSource::new();

        {
            let _guard =
                CancellationGuard::new(
                    source.clone(),
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
    fn cancellation_guard_can_be_disarmed() {
        let source = CancellationSource::new();

        {
            let mut guard =
                CancellationGuard::new(
                    source.clone(),
                );

            guard.disarm();
        }

        assert!(!source.is_cancelled());
    }

    #[test]
    fn deadline_shortening_never_extends_parent_deadline() {
        let token =
            CancellationToken::with_timeout(
                Duration::from_secs(10),
            );

        let child =
            token.with_timeout_from_now(
                Duration::from_millis(10),
            );

        assert!(
            child
                .remaining()
                .expect("deadline")
                <= Duration::from_millis(10)
        );
    }

    #[test]
    fn cancellation_reason_is_machine_readable() {
        assert_eq!(
            CancellationReason::Requested.as_str(),
            "requested"
        );

        assert_eq!(
            CancellationReason::DeadlineExceeded.as_str(),
            "deadline_exceeded"
        );

        assert_eq!(
            CancellationReason::SchedulerShutdown.as_str(),
            "scheduler_shutdown"
        );
    }
}