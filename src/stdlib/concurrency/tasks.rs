
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Logical Tasks
//!
//! Provides target-independent task abstractions for asynchronous,
//! concurrent, parallel, hybrid and distributed computation.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Logical task handles.
//! - Task lifecycle observation.
//! - Task completion observation.
//! - Cooperative task cancellation requests.
//! - Runtime executor submission contracts.
//! - Task execution metadata.
//! - Task future integration.
//!
//! This module does NOT own:
//!
//! - Physical threads.
//! - Worker pools.
//! - CPU scheduling.
//! - GPU scheduling.
//! - QPU scheduling.
//! - FPGA scheduling.
//! - Distributed placement.
//! - Network routing.
//! - Hardware allocation.
//! - Runtime execution engines.
//! - Resource allocation.
//! - Quantum execution.
//! - HDL execution.
//! - Runtime shutdown.
//!
//! Those responsibilities belong to the runtime, HAL,
//! distributed execution and hardware provider authorities.
//!
//! # Grammar authorities
//!
//! - grammar/concurrency/tasks.g4
//! - grammar/concurrency/task-parallel.g4
//! - grammar/concurrency/concurrency.g4
//! - grammar/concurrency/spawn.g4
//! - grammar/concurrency/async.g4
//! - grammar/concurrency/await.g4
//! - grammar/expressions/async.g4
//! - grammar/statements/concurrency.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! These grammars define source-language intent.
//!
//! This module does not parse or redefine their syntax.
//!
//! # Scalability
//!
//! No application-defined maximum is imposed on:
//!
//! - Tasks.
//! - Concurrent tasks.
//! - Nested tasks.
//! - Task handles.
//! - Logical execution domains.
//! - Task results.
//! - Providers.
//!
//! Actual limits are determined by available memory, executor
//! capacity, runtime configuration and platform resources.
//!
//! # Important
//!
//! A task represents logical computation, not a physical
//! execution resource.
//!
//! A runtime may execute logical tasks sequentially,
//! concurrently, in batches, on accelerators or remotely,
//! provided program semantics are preserved.

use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

use std::any::Any;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::runtime::concurrency::TaskId as RuntimeTaskId;

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
};

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifier assigned by the authoritative runtime.
///
/// This is an alias of the existing runtime identifier rather
/// than a second independently generated identifier system.
pub type TaskId = RuntimeTaskId;

// ============================================================================
// TASK STATUS
// ============================================================================

/// Observable logical task lifecycle.
///
/// These states describe the standard-library task handle.
/// They do not imply a particular physical execution mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TaskStatus {
    /// Task construction has started.
    Created,

    /// The runtime has accepted the task.
    Submitted,

    /// The executor is currently polling the task.
    Running,

    /// The task has completed successfully.
    Completed,

    /// The task has failed.
    Failed,

    /// Cancellation has been requested.
    CancellationRequested,

    /// The task has acknowledged cancellation.
    Cancelled,
}

impl TaskStatus {
    /// Returns whether the status represents a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled
        )
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

// ============================================================================
// TASK FUTURE
// ============================================================================

/// A boxed future submitted to the runtime.
///
/// The runtime is responsible for polling and executing it.
pub type BoxTaskFuture =
    Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// ============================================================================
// EXECUTOR CONTRACT
// ============================================================================

/// Runtime-owned task submission interface.
///
/// Implementations must:
///
/// - Accept a future for execution.
/// - Execute accepted futures according to runtime policy.
/// - Return a runtime-owned task identifier.
/// - Never claim successful acceptance when submission failed.
/// - Report submission failures through structured errors.
/// - Respect the future's Send and lifetime requirements.
///
/// This interface does not prescribe a scheduling algorithm.
///
/// An implementation may use cooperative execution,
/// threads, accelerators, distributed execution or another
/// compatible execution mechanism.
pub trait TaskExecutor: Send + Sync {
    /// Submit a logical task to the runtime.
    ///
    /// The executor takes ownership of the future.
    ///
    /// An error means that the future was not accepted.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the runtime cannot
    /// accept the task.
    fn submit(&self, future: BoxTaskFuture) -> Result<TaskId>;
}

// ============================================================================
// CANCELLATION
// ============================================================================

/// Thread-safe cooperative cancellation signal.
///
/// Cancellation requests do not forcibly terminate a task.
/// The executor must poll the task for cancellation to be
/// acknowledged.
struct TaskCancellation {
    cancelled: AtomicBool,
    waiter: Mutex<Option<Waker>>,
}

impl TaskCancellation {
    /// Creates a new cancellation signal.
    fn new() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            waiter: Mutex::new(None),
        }
    }

    /// Requests cancellation and wakes the executor.
    fn cancel(&self) -> bool {
        let previous = self.cancelled.swap(
            true,
            Ordering::AcqRel,
        );

        if !previous {
            let waiter = self
                .waiter
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take();

            if let Some(waker) = waiter {
                waker.wake();
            }
        }

        !previous
    }

    /// Registers an executor waker.
    fn register(&self, waker: &Waker) {
        let mut waiter = self
            .waiter
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        match waiter.as_ref() {
            Some(existing) if existing.will_wake(waker) => {}
            _ => {
                *waiter = Some(waker.clone());
            }
        }

        // Avoid losing a cancellation request that races
        // with waker registration.
        if self.is_cancelled() {
            if let Some(registered) = waiter.take() {
                registered.wake();
            }
        }
    }
}

impl CancellationSignal for TaskCancellation {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn cancellation_reason(&self) -> Option<String> {
        self.is_cancelled()
            .then(|| "task cancellation requested".to_owned())
    }
}

// ============================================================================
// COMPLETION STATE
// ============================================================================

/// Internal completion information.
///
/// The result is consumed exactly once by the task handle.
struct CompletionState<T> {
    status: TaskStatus,
    result: Option<Result<T>>,
    waiter: Option<Waker>,
}

/// Thread-safe shared completion state.
struct Completion<T> {
    state: Mutex<CompletionState<T>>,
}

impl<T> Completion<T> {
    /// Creates a completion state.
    fn new() -> Self {
        Self {
            state: Mutex::new(CompletionState {
                status: TaskStatus::Created,
                result: None,
                waiter: None,
            }),
        }
    }

    /// Updates the lifecycle status without overwriting
    /// a terminal state.
    fn transition(&self, status: TaskStatus) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if !state.status.is_terminal() {
            state.status = status;
        }
    }

    /// Marks a successfully submitted task.
    ///
    /// The executor may already have started polling it,
    /// so this must never move Running back to Submitted.
    fn mark_submitted(&self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if state.status == TaskStatus::Created {
            state.status = TaskStatus::Submitted;
        }
    }

    /// Completes the task.
    fn finish(&self, status: TaskStatus, result: Result<T>) {
        let waiter = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            if state.status.is_terminal() {
                return;
            }

            state.status = status;
            state.result = Some(result);

            state.waiter.take()
        };

        if let Some(waker) = waiter {
            waker.wake();
        }
    }

    /// Reads the current status.
    fn status(&self) -> TaskStatus {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .status
    }

    /// Polls for task completion.
    fn poll_result(
        &self,
        context: &mut Context<'_>,
    ) -> Poll<Result<T>> {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(result) = state.result.take() {
            return Poll::Ready(result);
        }

        if state.status.is_terminal() {
            return Poll::Ready(Err(
                Error::new(
                    ErrorKind::Internal,
                    "task result was already consumed",
                )
                .in_component("stdlib.concurrency.tasks")
                .during_operation("poll_result"),
            ));
        }

        match state.waiter.as_ref() {
            Some(existing) if existing.will_wake(context.waker()) => {}
            _ => {
                state.waiter = Some(context.waker().clone());
            }
        }

        Poll::Pending
    }
}

// ============================================================================
// TASK HANDLE
// ============================================================================

/// Handle to a submitted logical task.
///
/// The handle is thread-safe when its result type is Send.
///
/// Completion is observed asynchronously through Future.
///
/// The handle does not execute or schedule the task itself.
pub struct TaskHandle<T> {
    id: TaskId,
    completion: Arc<Completion<T>>,
    cancellation: Arc<TaskCancellation>,
}

impl<T> TaskHandle<T> {
    /// Returns the runtime-owned task identifier.
    #[must_use]
    pub const fn id(&self) -> TaskId {
        self.id
    }

    /// Returns the observable task lifecycle status.
    #[must_use]
    pub fn status(&self) -> TaskStatus {
        self.completion.status()
    }

    /// Requests cooperative cancellation.
    ///
    /// Returns false when cancellation was already requested
    /// or the task has reached a terminal state.
    ///
    /// This operation does not claim that physical execution
    /// has already stopped.
    pub fn cancel(&self) -> bool {
        let mut state = self
            .completion
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if state.status.is_terminal()
            || self.cancellation.is_cancelled()
        {
            return false;
        }

        self.cancellation.cancel();

        state.status = TaskStatus::CancellationRequested;

        true
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Awaits task completion and consumes the result.
    ///
    /// # Errors
    ///
    /// Returns the task's execution or cancellation error.
    pub async fn join(self) -> Result<T> {
        self.await
    }
}

impl<T> Future for TaskHandle<T> {
    type Output = Result<T>;

    fn poll(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        self.completion.poll_result(context)
    }
}

impl<T> fmt::Debug for TaskHandle<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskHandle")
            .field("id", &self.id)
            .field("status", &self.status())
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

// ============================================================================
// TASK EXECUTION FUTURE
// ============================================================================

/// Future wrapper responsible for observing cancellation,
/// capturing task panics and recording completion.
///
/// Scheduling and polling remain runtime responsibilities.
struct TaskRunner<F, T> {
    future: Pin<Box<F>>,
    completion: Arc<Completion<T>>,
    cancellation: Arc<TaskCancellation>,
    context: OperationContext,
}

impl<F, T> Future for TaskRunner<F, T>
where
    F: Future<Output = T>,
{
    type Output = ();

    fn poll(
        self: Pin<&mut Self>,
        task_context: &mut Context<'_>,
    ) -> Poll<()> {
        let this = self.get_mut();

        if this.cancellation.is_cancelled()
            || this.context.is_cancelled()
        {
            this.completion.finish(
                TaskStatus::Cancelled,
                Err(
                    Error::new(
                        ErrorKind::Cancelled,
                        "task was cancelled before completion",
                    )
                    .in_component("stdlib.concurrency.tasks")
                    .during_operation("execute"),
                ),
            );

            return Poll::Ready(());
        }

        this.completion.transition(TaskStatus::Running);

        // Contain panics originating from user future polling.
        // This does not protect against process-aborting panics.
        let result = catch_unwind(AssertUnwindSafe(|| {
            this.future.as_mut().poll(task_context)
        }));

        match result {
            Ok(Poll::Pending) => {
                this.cancellation.register(task_context.waker());

                if this.cancellation.is_cancelled() {
                    task_context.waker().wake_by_ref();
                }

                Poll::Pending
            }

            Ok(Poll::Ready(value)) => {
                if this.cancellation.is_cancelled() {
                    this.completion.finish(
                        TaskStatus::Cancelled,
                        Err(
                            Error::new(
                                ErrorKind::Cancelled,
                                "task was cancelled during execution",
                            )
                            .in_component("stdlib.concurrency.tasks")
                            .during_operation("execute"),
                        ),
                    );
                } else {
                    this.completion.finish(
                        TaskStatus::Completed,
                        Ok(value),
                    );
                }

                Poll::Ready(())
            }

            Err(_) => {
                this.completion.finish(
                    TaskStatus::Failed,
                    Err(
                        Error::new(
                            ErrorKind::Internal,
                            "task future panicked during execution",
                        )
                        .in_component("stdlib.concurrency.tasks")
                        .during_operation("execute"),
                    ),
                );

                Poll::Ready(())
            }
        }
    }
}

// ============================================================================
// TASK SPAWNING
// ============================================================================

/// Spawns a logical asynchronous task.
///
/// The supplied executor remains the sole authority for
/// scheduling and actual execution.
///
/// The task future must be Send and have a static lifetime.
///
/// # Arguments
///
/// - executor: Runtime-owned task executor.
/// - context: Parent operation context.
/// - future: Asynchronous computation.
///
/// # Errors
///
/// Returns an error if the runtime rejects submission.
///
/// # Scalability
///
/// This function introduces no application-defined task-count
/// or hardware-count ceiling.
pub fn spawn<F, T>(
    executor: &dyn TaskExecutor,
    context: OperationContext,
    future: F,
) -> Result<TaskHandle<T>>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    spawn_with(executor, context, move |_| future)
}

/// Spawns a task whose computation receives its own
/// cancellation-aware operation context.
///
/// The supplied context inherits the parent context's
/// metadata and extensions while using the task's
/// cooperative cancellation signal.
///
/// This is the preferred API when task code performs
/// nested standard-library operations.
///
/// # Errors
///
/// Returns an error when the runtime rejects submission.
pub fn spawn_with<F, T, Factory>(
    executor: &dyn TaskExecutor,
    parent_context: OperationContext,
    factory: Factory,
) -> Result<TaskHandle<T>>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
    Factory: FnOnce(OperationContext) -> F,
{
    let cancellation = Arc::new(TaskCancellation::new());

    let task_context = parent_context
        .child_builder()
        .with_cancellation(cancellation.clone())
        .build();

    let completion = Arc::new(Completion::<T>::new());

    let future = factory(task_context.clone());

    let runner = TaskRunner {
        future: Box::pin(future),
        completion: Arc::clone(&completion),
        cancellation: Arc::clone(&cancellation),
        context: task_context,
    };

    // The executor must guarantee that returning an error
    // means the future was not accepted.
    let id = executor.submit(Box::pin(runner))?;

    completion.mark_submitted();

    Ok(TaskHandle {
        id,
        completion,
        cancellation,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::AtomicU64;
    use std::task::Wake;
    use std::thread;

    use crate::stdlib::context::OperationContext;

    struct TestExecutor {
        next_id: AtomicU64,
    }

    impl TestExecutor {
        fn new() -> Self {
            Self {
                next_id: AtomicU64::new(1),
            }
        }
    }

    struct ThreadWake {
        thread: thread::Thread,
    }

    impl Wake for ThreadWake {
        fn wake(self: Arc<Self>) {
            self.thread.unpark();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.thread.unpark();
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = std::task::Waker::from(
            Arc::new(ThreadWake {
                thread: thread::current(),
            }),
        );

        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => thread::park(),
            }
        }
    }

    impl TaskExecutor for TestExecutor {
        fn submit(&self, future: BoxTaskFuture) -> Result<TaskId> {
            let id = self.next_id.fetch_add(
                1,
                Ordering::Relaxed,
            );

            thread::Builder::new()
                .spawn(move || {
                    block_on(future);
                })
                .map_err(|error| {
                    Error::new(
                        ErrorKind::ResourceUnavailable,
                        format!("test executor could not start task: {error}"),
                    )
                    .in_component("stdlib.concurrency.tasks.tests")
                    .during_operation("submit")
                })?;

            Ok(id)
        }
    }

    #[test]
    fn task_completes_successfully() {
        let executor = TestExecutor::new();

        let handle = spawn(
            &executor,
            OperationContext::default(),
            async {
                42_u64
            },
        )
        .expect("task submission");

        assert_eq!(block_on(handle), Ok(42));
    }

    #[test]
    fn task_receives_cancellation_aware_context() {
        let executor = TestExecutor::new();

        let handle = spawn_with(
            &executor,
            OperationContext::default(),
            |context| async move {
                context.operation().unwrap_or("task")
            },
        )
        .expect("task submission");

        assert_eq!(
            block_on(handle).expect("task result"),
            "task"
        );
    }

    #[test]
    fn task_reports_panics_as_structured_errors() {
        let executor = TestExecutor::new();

        let handle = spawn(
            &executor,
            OperationContext::default(),
            async {
                panic!("intentional task panic");
                #[allow(unreachable_code)]
                42_u64
            },
        )
        .expect("task submission");

        let error = block_on(handle)
            .expect_err("panic must become an error");

        assert_eq!(error.kind(), ErrorKind::Internal);
    }

    #[test]
    fn task_identifier_is_runtime_owned() {
        let executor = TestExecutor::new();

        let handle = spawn(
            &executor,
            OperationContext::default(),
            async {
                7_u64
            },
        )
        .expect("task submission");

        assert!(handle.id() > 0);

        assert_eq!(block_on(handle), Ok(7));
    }

    #[test]
    fn task_status_has_terminal_classification() {
        assert!(!TaskStatus::Created.is_terminal());
        assert!(!TaskStatus::Submitted.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());

        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn task_cancellation_signal_is_idempotent() {
        let cancellation = TaskCancellation::new();

        assert!(!cancellation.is_cancelled());
        assert!(cancellation.cancel());
        assert!(!cancellation.cancel());
        assert!(cancellation.is_cancelled());
    }

    #[test]
    fn task_status_display_is_available() {
        assert_eq!(
            TaskStatus::Completed.to_string(),
            "Completed"
        );
    }

    #[test]
    fn completion_state_does_not_regress() {
        let completion = Completion::<u64>::new();

        completion.transition(TaskStatus::Running);
        completion.mark_submitted();

        assert_eq!(
            completion.status(),
            TaskStatus::Running
        );
    }

    #[test]
    fn completion_is_terminal_after_success() {
        let completion = Completion::<u64>::new();

        completion.finish(
            TaskStatus::Completed,
            Ok(42),
        );

        completion.transition(TaskStatus::Running);

        assert_eq!(
            completion.status(),
            TaskStatus::Completed
        );
    }
}
