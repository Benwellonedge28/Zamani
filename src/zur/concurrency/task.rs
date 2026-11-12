
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Task Lifecycle
//!
//! Path: src/zur/concurrency/task.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Defines the canonical lifecycle and completion contract for
//! runtime-managed concurrent tasks.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Task.
//! - TaskId (an alias of the canonical RuntimeOperationId).
//! - TaskStatus.
//! - TaskSnapshot.
//! - TaskOutcome.
//! - Task lifecycle transitions.
//! - Task cancellation requests.
//! - Completion publication and observation.
//! - Task-local synchronization.
//!
//! This module does NOT own:
//!
//! - Thread creation.
//! - Thread pools.
//! - Work stealing.
//! - Task scheduling.
//! - Execution planning.
//! - Compiler or domain IR.
//! - Distributed TaskId.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Async executor implementation.
//! - Runtime-wide lifecycle.
//!
//! EXISTING AUTHORITIES
//!
//! - zur/ids.rs owns RuntimeOperationId.
//! - zur/ids.rs owns IdAllocator.
//! - zur/error.rs owns ZurError and ZurResult.
//! - zur/execution/cancellation.rs owns execution cancellation.
//! - zur/execution/session.rs owns session cancellation.
//! - zur/core/context.rs owns RuntimeContext.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw pointers.
//! No static mutable state.
//! No global task registry.
//! No implicit thread creation.
//! No fabricated completion.
//!
//! SCALABILITY
//!
//! No artificial task-count, worker-count, processor-count,
//! device-count or domain-count ceiling is imposed.
//!
//! Actual growth is constrained by available resources,
//! address-space limits and the underlying execution provider.
//!
//! Literal infinite physical computation is not promised.

use std::fmt;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::execution::cancellation::{
    ExecutionCancellationController,
};

use crate::zur::ids::RuntimeOperationId;

// ============================================================================
// IDENTITY
// ============================================================================

/// Canonical identity of a ZUR-managed task.
///
/// Task identities reuse RuntimeOperationId because a task is a
/// ZUR-owned operational action.
///
/// This avoids introducing a duplicate runtime identity system.
///
/// Callers should allocate task identities using a dedicated namespace,
/// for example `zamani.zur.task`.
///
/// Identity allocation belongs to the caller or runtime composition layer.
///
/// Identity existence does not imply execution, availability or completion.
pub type TaskId = RuntimeOperationId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.concurrency.task";

// ============================================================================
// TASK STATUS
// ============================================================================

/// Observable lifecycle state of a runtime-managed task.
///
/// CancellationRequested is deliberately distinct from Cancelled.
///
/// A cancellation request does not prove that the executor or an
/// external provider has actually stopped executing the task.
///
/// Cancelled is only published after the execution authority
/// explicitly acknowledges cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TaskStatus {
    /// Task has been created but has not been prepared for execution.
    Created,

    /// Task has been accepted for execution.
    ///
    /// This does not prove that a worker has started it.
    Ready,

    /// An execution authority has started the task.
    Running,

    /// Cooperative cancellation has been requested.
    ///
    /// Execution may still be running.
    CancellationRequested,

    /// Execution has completed successfully.
    Completed,

    /// Execution has terminated with a failure.
    Failed,

    /// Cancellation has been explicitly acknowledged by the executor.
    Cancelled,
}

impl TaskStatus {
    /// Returns whether the task has reached a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled
        )
    }

    /// Returns whether the task is eligible to start execution.
    #[must_use]
    pub const fn can_start(self) -> bool {
        matches!(self, Self::Created | Self::Ready)
    }

    /// Returns whether the task has entered an execution state.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Running | Self::CancellationRequested
        )
    }
}

// ============================================================================
// TASK OUTCOME
// ============================================================================

/// Final outcome of an execution.
///
/// An outcome is immutable once published.
///
/// Arc permits multiple observers to inspect the same outcome
/// without cloning a potentially non-cloneable result or error.
#[derive(Debug)]
pub enum TaskOutcome<T> {
    /// The task completed successfully.
    Succeeded(Arc<T>),

    /// The task terminated with an error.
    Failed(Arc<ZurError>),

    /// The executor explicitly acknowledged cancellation.
    Cancelled {
        /// Authoritative cancellation reason when available.
        reason: Option<String>,
    },
}

impl<T> TaskOutcome<T> {
    /// Returns whether the task completed successfully.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded(_))
    }

    /// Returns whether the task failed.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    /// Returns whether cancellation was acknowledged.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    /// Returns a reference to the successful result.
    #[must_use]
    pub fn value(&self) -> Option<&Arc<T>> {
        match self {
            Self::Succeeded(value) => Some(value),
            Self::Failed(_) | Self::Cancelled { .. } => None,
        }
    }

    /// Returns the failure when the task failed.
    #[must_use]
    pub fn error(&self) -> Option<&Arc<ZurError>> {
        match self {
            Self::Failed(error) => Some(error),
            Self::Succeeded(_) | Self::Cancelled { .. } => None,
        }
    }

    /// Returns the cancellation reason.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<&str> {
        match self {
            Self::Cancelled { reason } => reason.as_deref(),
            Self::Succeeded(_) | Self::Failed(_) => None,
        }
    }
}

// ============================================================================
// TASK SNAPSHOT
// ============================================================================

/// Immutable observation of a task's current state.
///
/// A snapshot does not own or control the task.
///
/// It is safe to retain independently of subsequent task transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskSnapshot {
    id: TaskId,
    status: TaskStatus,
    cancellation_requested: bool,
    cancellation_reason: Option<String>,
}

impl TaskSnapshot {
    /// Returns the task identity.
    #[must_use]
    pub fn id(&self) -> &TaskId {
        &self.id
    }

    /// Returns the observed lifecycle state.
    #[must_use]
    pub const fn status(&self) -> TaskStatus {
        self.status
    }

    /// Returns whether cancellation has been requested.
    ///
    /// This includes cancellation inherited from a parent scope.
    #[must_use]
    pub const fn cancellation_requested(&self) -> bool {
        self.cancellation_requested
    }

    /// Returns the available cancellation reason.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<&str> {
        self.cancellation_reason.as_deref()
    }

    /// Returns whether execution has reached a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Mutable state shared by all handles to one task.
struct TaskInner<T> {
    id: TaskId,

    state: Mutex<TaskState<T>>,

    changed: Condvar,

    cancellation: ExecutionCancellationController,
}

/// Internal task lifecycle and completion storage.
struct TaskState<T> {
    status: TaskStatus,

    completion: Option<Arc<TaskOutcome<T>>>,
}

// ============================================================================
// TASK HANDLE
// ============================================================================

/// Thread-safe handle to a runtime-managed task.
///
/// Cloning a Task clones its shared ownership handle.
///
/// It does not duplicate the task, create another execution,
/// start a worker or transfer scheduling responsibility.
///
/// The execution engine owns actual task execution.
///
/// T must be Send + Sync because completed results are shared
/// between potentially independent observers.
pub struct Task<T>
where
    T: Send + Sync + 'static,
{
    inner: Arc<TaskInner<T>>,
}

impl<T> Clone for Task<T>
where
    T: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl<T> Task<T>
where
    T: Send + Sync + 'static,
{
    /// Creates a new independent task.
    ///
    /// The identity must have been allocated by the canonical
    /// ZUR identity allocator.
    ///
    /// No thread or execution worker is created.
    #[must_use]
    pub fn new(id: TaskId) -> Self {
        Self::with_controller(
            id,
            ExecutionCancellationController::new(),
        )
    }

    /// Creates a task linked to a parent cancellation controller.
    ///
    /// Parent cancellation propagates to the task.
    ///
    /// Task-local cancellation does not propagate upward.
    ///
    /// A child controller is created rather than sharing the parent's
    /// local cancellation ownership.
    #[must_use]
    pub fn with_parent(
        id: TaskId,
        parent: &ExecutionCancellationController,
    ) -> Self {
        Self::with_controller(id, parent.child())
    }

    /// Creates a task using an explicitly supplied cancellation controller.
    ///
    /// This is useful when an execution session or another execution
    /// component already owns the appropriate cancellation scope.
    ///
    /// Cloning the controller shares its cancellation state.
    #[must_use]
    pub fn with_controller(
        id: TaskId,
        cancellation: ExecutionCancellationController,
    ) -> Self {
        Self {
            inner: Arc::new(TaskInner {
                id,
                state: Mutex::new(TaskState {
                    status: TaskStatus::Created,
                    completion: None,
                }),
                changed: Condvar::new(),
                cancellation,
            }),
        }
    }

    // ========================================================================
    // IDENTITY
    // ========================================================================

    /// Returns the canonical task identity.
    #[must_use]
    pub fn id(&self) -> &TaskId {
        &self.inner.id
    }

    // ========================================================================
    // CANCELLATION
    // ========================================================================

    /// Returns a clone of the task's cancellation controller.
    ///
    /// This allows the executor and cancellation-aware operations
    /// to observe and coordinate cancellation.
    #[must_use]
    pub fn cancellation_controller(
        &self,
    ) -> ExecutionCancellationController {
        self.inner.cancellation.clone()
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancellation_requested(&self) -> bool {
        self.inner.cancellation.is_requested()
    }

    /// Requests cooperative task cancellation.
    ///
    /// This operation does not claim that execution has stopped.
    ///
    /// Returns true when this call transitions the task into
    /// CancellationRequested.
    ///
    /// Returns false if cancellation was already requested or
    /// the task had already reached a terminal state.
    pub fn request_cancel(
        &self,
        reason: impl Into<String>,
    ) -> ZurResult<bool> {
        let mut state = self.lock_state("request_cancel")?;

        if state.status.is_terminal() {
            return Ok(false);
        }

        if state.status == TaskStatus::CancellationRequested {
            return Ok(false);
        }

        if !self.inner.cancellation.is_requested() {
            self.inner.cancellation.request(reason);
        }

        state.status = TaskStatus::CancellationRequested;

        self.inner.changed.notify_all();

        Ok(true)
    }

    /// Returns an error when cancellation has been requested.
    ///
    /// The execution implementation should call this at appropriate
    /// cooperative cancellation checkpoints.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        self.inner
            .cancellation
            .check()
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("check_cancelled")
            })
    }

    // ========================================================================
    // LIFECYCLE TRANSITIONS
    // ========================================================================

    /// Marks the task as ready for execution.
    ///
    /// This does not imply that a scheduler has assigned a worker.
    ///
    /// Valid transition:
    ///
    /// Created -> Ready
    pub fn mark_ready(&self) -> ZurResult<()> {
        let mut state = self.lock_state("mark_ready")?;

        self.ensure_status(
            state.status,
            TaskStatus::Created,
            "mark_ready",
        )?;

        self.check_not_cancelled("mark_ready")?;

        state.status = TaskStatus::Ready;

        self.inner.changed.notify_all();

        Ok(())
    }

    /// Marks the task as actively executing.
    ///
    /// This method must only be called by the execution authority
    /// after execution has actually started.
    ///
    /// Valid transitions:
    ///
    /// Created -> Running
    /// Ready -> Running
    pub fn mark_running(&self) -> ZurResult<()> {
        let mut state = self.lock_state("mark_running")?;

        if !state.status.can_start() {
            return Err(self.transition_error(
                state.status,
                TaskStatus::Running,
                "mark_running",
            ));
        }

        self.check_not_cancelled("mark_running")?;

        state.status = TaskStatus::Running;

        self.inner.changed.notify_all();

        Ok(())
    }

    /// Publishes successful task completion.
    ///
    /// This must only be called after the execution authority
    /// has actually established successful completion.
    ///
    /// Valid transitions:
    ///
    /// Running -> Completed
    /// CancellationRequested -> Completed
    ///
    /// The second transition is permitted because completion and
    /// cancellation can race. The actual execution outcome wins.
    ///
    /// Completion can only be published once.
    pub fn complete(&self, value: T) -> ZurResult<()> {
        let mut state = self.lock_state("complete")?;

        self.ensure_completion_transition(
            state.status,
            TaskStatus::Completed,
            "complete",
        )?;

        state.completion = Some(Arc::new(
            TaskOutcome::Succeeded(Arc::new(value)),
        ));

        state.status = TaskStatus::Completed;

        self.inner.changed.notify_all();

        Ok(())
    }

    /// Publishes task failure.
    ///
    /// The supplied error must describe an actual execution failure.
    ///
    /// Failure is distinct from cancellation and incomplete observation.
    pub fn fail(&self, error: ZurError) -> ZurResult<()> {
        let mut state = self.lock_state("fail")?;

        self.ensure_completion_transition(
            state.status,
            TaskStatus::Failed,
            "fail",
        )?;

        state.completion = Some(Arc::new(
            TaskOutcome::Failed(Arc::new(error)),
        ));

        state.status = TaskStatus::Failed;

        self.inner.changed.notify_all();

        Ok(())
    }

    /// Acknowledges that execution has actually stopped because
    /// cancellation was requested.
    ///
    /// Only the execution authority should call this method.
    ///
    /// A cancellation request alone is insufficient.
    ///
    /// Valid transition:
    ///
    /// CancellationRequested -> Cancelled
    pub fn acknowledge_cancelled(&self) -> ZurResult<()> {
        let mut state = self.lock_state("acknowledge_cancelled")?;

        self.ensure_status(
            state.status,
            TaskStatus::CancellationRequested,
            "acknowledge_cancelled",
        )?;

        if !self.inner.cancellation.is_requested() {
            return Err(ZurError::internal(
                "cancellation acknowledgement without a request",
            )
            .with_component(COMPONENT)
            .with_operation("acknowledge_cancelled"));
        }

        let reason = self.inner.cancellation.reason();

        state.completion = Some(Arc::new(
            TaskOutcome::Cancelled { reason },
        ));

        state.status = TaskStatus::Cancelled;

        self.inner.changed.notify_all();

        Ok(())
    }

    // ========================================================================
    // OBSERVATION
    // ========================================================================

    /// Returns an immutable snapshot of the current task state.
    ///
    /// The snapshot may become outdated immediately after returning.
    pub fn snapshot(&self) -> ZurResult<TaskSnapshot> {
        let state = self.lock_state("snapshot")?;

        let cancellation_requested =
            self.inner.cancellation.is_requested();

        let status = if cancellation_requested
            && !state.status.is_terminal()
        {
            TaskStatus::CancellationRequested
        } else {
            state.status
        };

        Ok(TaskSnapshot {
            id: self.inner.id.clone(),
            status,
            cancellation_requested,
            cancellation_reason: self.inner.cancellation.reason(),
        })
    }

    /// Returns the published completion, if one exists.
    ///
    /// The returned Arc permits multiple independent observers.
    #[must_use]
    pub fn try_completion(
        &self,
    ) -> Option<Arc<TaskOutcome<T>>> {
        let state = self
            .inner
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        state.completion.clone()
    }

    /// Waits until the task reaches a terminal state.
    ///
    /// This blocks the calling thread.
    ///
    /// It belongs in a blocking execution or synchronization context,
    /// not inside an async executor worker unless that executor
    /// explicitly permits blocking.
    pub fn wait(&self) -> ZurResult<Arc<TaskOutcome<T>>> {
        let mut state = self.lock_state("wait")?;

        loop {
            if let Some(completion) = &state.completion {
                return Ok(Arc::clone(completion));
            }

            state = self
                .inner
                .changed
                .wait(state)
                .map_err(|_| {
                    ZurError::internal(
                        "task state lock was poisoned while waiting",
                    )
                    .with_component(COMPONENT)
                    .with_operation("wait")
                })?;
        }
    }

    /// Waits for task completion up to the supplied duration.
    ///
    /// Returns:
    ///
    /// Some(outcome) when terminal completion is published.
    ///
    /// None when the timeout expires before completion.
    ///
    /// A timeout does not cancel the task.
    pub fn wait_timeout(
        &self,
        timeout: Duration,
    ) -> ZurResult<Option<Arc<TaskOutcome<T>>>> {
        let started = Instant::now();

        let mut state = self.lock_state("wait_timeout")?;

        loop {
            if let Some(completion) = &state.completion {
                return Ok(Some(Arc::clone(completion)));
            }

            let elapsed = started.elapsed();

            if elapsed >= timeout {
                return Ok(None);
            }

            let remaining = timeout.saturating_sub(elapsed);

            let (next_state, result) = self
                .inner
                .changed
                .wait_timeout(state, remaining)
                .map_err(|_| {
                    ZurError::internal(
                        "task state lock was poisoned during timed wait",
                    )
                    .with_component(COMPONENT)
                    .with_operation("wait_timeout")
                })?;

            state = next_state;

            if result.timed_out() && state.completion.is_none() {
                return Ok(None);
            }
        }
    }

    // ========================================================================
    // INTERNAL VALIDATION
    // ========================================================================

    fn lock_state(
        &self,
        operation: &'static str,
    ) -> ZurResult<MutexGuard<'_, TaskState<T>>> {
        self.inner.state.lock().map_err(|_| {
            ZurError::internal("task lifecycle state lock was poisoned")
                .with_component(COMPONENT)
                .with_operation(operation)
        })
    }

    fn check_not_cancelled(
        &self,
        operation: &'static str,
    ) -> ZurResult<()> {
        self.inner
            .cancellation
            .check()
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation(operation)
            })
    }

    fn ensure_status(
        &self,
        actual: TaskStatus,
        expected: TaskStatus,
        operation: &'static str,
    ) -> ZurResult<()> {
        if actual == expected {
            return Ok(());
        }

        Err(self.transition_error(
            actual,
            expected,
            operation,
        ))
    }

    fn ensure_completion_transition(
        &self,
        actual: TaskStatus,
        target: TaskStatus,
        operation: &'static str,
    ) -> ZurResult<()> {
        if matches!(
            actual,
            TaskStatus::Running
                | TaskStatus::CancellationRequested
        ) {
            return Ok(());
        }

        Err(self.transition_error(
            actual,
            target,
            operation,
        ))
    }

    fn transition_error(
        &self,
        actual: TaskStatus,
        expected: TaskStatus,
        operation: &'static str,
    ) -> ZurError {
        ZurError::invalid_input(format!(
            "invalid task lifecycle transition: \
             actual={actual:?}, requested={expected:?}"
        ))
        .with_component(COMPONENT)
        .with_operation(operation)
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl<T> fmt::Debug for Task<T>
where
    T: Send + Sync + 'static,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let state = self
            .inner
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        formatter
            .debug_struct("Task")
            .field("id", &self.inner.id)
            .field("status", &state.status)
            .field(
                "cancellation_requested",
                &self.inner.cancellation.is_requested(),
            )
            .field("completed", &state.completion.is_some())
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::{
        IdAllocator,
        IdNamespace,
    };

    fn create_id() -> TaskId {
        let namespace = IdNamespace::new("zamani.zur.task")
            .expect("valid namespace");

        let allocator =
            IdAllocator::<RuntimeOperationId>::new(namespace);

        allocator.try_next().expect("identity allocation")
    }

    #[test]
    fn task_starts_in_created_state() {
        let task = Task::<u64>::new(create_id());

        let snapshot = task.snapshot().expect("snapshot");

        assert_eq!(snapshot.status(), TaskStatus::Created);
        assert!(!snapshot.is_terminal());
    }

    #[test]
    fn task_can_become_ready_and_running() {
        let task = Task::<u64>::new(create_id());

        task.mark_ready().expect("ready");
        task.mark_running().expect("running");

        assert_eq!(
            task.snapshot().expect("snapshot").status(),
            TaskStatus::Running
        );
    }

    #[test]
    fn task_can_complete_successfully() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");
        task.complete(42).expect("complete");

        let completion = task.wait().expect("wait");

        assert!(completion.is_success());

        let value = completion
            .value()
            .expect("successful value");

        assert_eq!(**value, 42);

        assert_eq!(
            task.snapshot().expect("snapshot").status(),
            TaskStatus::Completed
        );
    }

    #[test]
    fn completion_is_published_only_once() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");
        task.complete(42).expect("complete");

        assert!(task.complete(43).is_err());

        let completion = task.wait().expect("wait");

        assert_eq!(
            **completion.value().expect("value"),
            42
        );
    }

    #[test]
    fn task_cancellation_is_not_completion() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");

        assert!(
            task.request_cancel("requested by caller")
                .expect("cancellation request")
        );

        let snapshot = task.snapshot().expect("snapshot");

        assert_eq!(
            snapshot.status(),
            TaskStatus::CancellationRequested
        );

        assert!(!snapshot.is_terminal());

        assert!(task.try_completion().is_none());
    }

    #[test]
    fn executor_must_acknowledge_cancellation() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");

        task.request_cancel("test cancellation")
            .expect("request");

        task.acknowledge_cancelled()
            .expect("acknowledgement");

        let completion = task.wait().expect("wait");

        assert!(completion.is_cancelled());

        assert_eq!(
            task.snapshot().expect("snapshot").status(),
            TaskStatus::Cancelled
        );
    }

    #[test]
    fn task_cannot_acknowledge_unrequested_cancellation() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");

        assert!(task.acknowledge_cancelled().is_err());
    }

    #[test]
    fn task_failure_is_distinct_from_cancellation() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");

        task.fail(
            ZurError::internal("test execution failure")
        )
        .expect("failure");

        let completion = task.wait().expect("wait");

        assert!(completion.is_failure());

        assert_eq!(
            task.snapshot().expect("snapshot").status(),
            TaskStatus::Failed
        );
    }

    #[test]
    fn task_handles_share_the_same_state() {
        let task = Task::<u64>::new(create_id());

        let observer = task.clone();

        task.mark_running().expect("running");
        task.complete(100).expect("complete");

        let completion = observer.wait().expect("wait");

        assert_eq!(
            **completion.value().expect("value"),
            100
        );
    }

    #[test]
    fn task_timeout_does_not_cancel_execution() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");

        let completion = task
            .wait_timeout(Duration::from_millis(1))
            .expect("timed wait");

        assert!(completion.is_none());

        assert_eq!(
            task.snapshot().expect("snapshot").status(),
            TaskStatus::Running
        );

        assert!(!task.is_cancellation_requested());
    }

    #[test]
    fn task_cancellation_does_not_affect_an_independent_task() {
        let first = Task::<u64>::new(create_id());
        let second = Task::<u64>::new(create_id());

        first.mark_running().expect("running");

        first.request_cancel("first task")
            .expect("cancel");

        assert!(first.is_cancellation_requested());

        assert!(!second.is_cancellation_requested());

        assert_eq!(
            second.snapshot().expect("snapshot").status(),
            TaskStatus::Created
        );
    }

    #[test]
    fn terminal_tasks_cannot_be_restarted() {
        let task = Task::<u64>::new(create_id());

        task.mark_running().expect("running");
        task.complete(1).expect("complete");

        assert!(task.mark_running().is_err());
        assert!(task.mark_ready().is_err());
    }
}
