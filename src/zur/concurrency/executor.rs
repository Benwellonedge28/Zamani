
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Concurrency Executor
//!
//! Path: src/zur/concurrency/executor.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the execution coordination boundary for ZUR-managed tasks.
//!
//! This module accepts an already-created Task, transfers its operation
//! to an explicitly supplied execution facility, coordinates cancellation,
//! and publishes the actual execution outcome through the canonical Task.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - TaskDispatcher.
//! - TaskExecutor.
//! - TaskExecutionContext.
//! - InlineTaskDispatcher.
//! - Task submission coordination.
//! - Execution panic containment.
//! - Execution outcome publication.
//!
//! This module does NOT own:
//!
//! - Task identity.
//! - Task lifecycle definitions.
//! - Task completion storage.
//! - Task scheduling policies.
//! - Work stealing.
//! - Worker-pool implementation.
//! - Thread creation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Compiler execution planning.
//! - Compiled artifact execution.
//! - Distributed execution.
//! - Domain-specific execution.
//! - Runtime-wide lifecycle.
//!
//! EXISTING AUTHORITIES
//!
//! - concurrency/task.rs:
//!   Task, TaskId, TaskStatus and TaskOutcome.
//!
//! - zur/ids.rs:
//!   RuntimeOperationId.
//!
//! - zur/error.rs:
//!   ZurError and ZurResult.
//!
//! - execution/cancellation.rs:
//!   ExecutionCancellationController.
//!
//! - contracts/execution.rs:
//!   ExecutionEngine for compiled Zamani artifacts.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw pointers.
//! No static mutable state.
//! No global task registry.
//! No implicit worker creation.
//! No fabricated completion.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Tasks.
//! - Workers.
//! - Concurrent operations.
//! - Execution facilities.
//! - Computational domains.
//! - Devices.
//! - Providers.
//!
//! Growth is constrained by actual resource availability,
//! allocator capacity and the supplied execution facility.
//!
//! Literal infinite physical computation is not promised.

use std::any::Any;
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use crate::zur::concurrency::task::{
    Task,
    TaskId,
    TaskOutcome,
    TaskStatus,
};

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::execution::cancellation::
    ExecutionCancellationController;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.concurrency.executor";

// ============================================================================
// EXECUTION JOB
// ============================================================================

/// An owned, executable task operation.
///
/// The operation is transferred to an execution facility.
///
/// It must be Send because an implementation may transfer it
/// between execution contexts.
///
/// The executor catches unwinding panics where the Rust build
/// uses panic=unwind.
///
/// A process-aborting panic cannot be recovered from.
pub type TaskOperation<T> = Box<
    dyn FnOnce(TaskExecutionContext) -> ZurResult<T>
        + Send
        + 'static,
>;

/// Type-erased execution job accepted by a task dispatcher.
///
/// The job owns everything required for its execution.
pub type DispatchJob = Box<
    dyn FnOnce() + Send + 'static,
>;

// ============================================================================
// TASK EXECUTION CONTEXT
// ============================================================================

/// Read-only operational context supplied to a running task.
///
/// The context exposes the task's identity and cancellation
/// observation without transferring ownership of task lifecycle.
///
/// It does not expose scheduling or hardware-selection APIs.
#[derive(Clone)]
pub struct TaskExecutionContext {
    task_id: TaskId,
    cancellation: ExecutionCancellationController,
}

impl TaskExecutionContext {
    /// Creates an execution context for an existing task.
    ///
    /// Internal construction ensures that the context uses
    /// the task's canonical cancellation controller.
    fn new<T>(task: &Task<T>) -> Self
    where
        T: Send + Sync + 'static,
    {
        Self {
            task_id: task.id().clone(),
            cancellation: task.cancellation_controller(),
        }
    }

    /// Returns the canonical task identity.
    #[must_use]
    pub fn task_id(&self) -> &TaskId {
        &self.task_id
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancellation_requested(&self) -> bool {
        self.cancellation.is_requested()
    }

    /// Returns the available cancellation reason.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<String> {
        self.cancellation.reason()
    }

    /// Checks cooperative cancellation.
    ///
    /// Call this at appropriate execution checkpoints.
    ///
    /// Cancellation does not forcibly interrupt arbitrary
    /// computation or external hardware operations.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        self.cancellation
            .check()
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("check_cancelled")
            })
    }

    /// Returns a clone of the task cancellation controller.
    ///
    /// This allows cancellation-aware operations to integrate
    /// with existing ZUR cancellation infrastructure.
    #[must_use]
    pub fn cancellation_controller(
        &self,
    ) -> ExecutionCancellationController {
        self.cancellation.clone()
    }
}

// ============================================================================
// EXECUTION FACILITY CONTRACT
// ============================================================================

/// Explicit task execution facility.
///
/// Implementations receive ownership of a task operation.
///
/// The dispatcher is an execution substrate, not a scheduler.
///
/// It must not silently change the task's semantic ordering,
/// synchronization or cancellation guarantees.
///
/// IMPORTANT CONTRACT:
///
/// - Ok means the job has been accepted exactly once.
/// - Err means the job was not accepted and will never execute.
/// - A rejected job must be dropped rather than retained.
/// - Accepted jobs must eventually execute or be explicitly
///   accounted for by the execution facility.
/// - A dispatcher must not execute the same job more than once.
///
/// An asynchronous worker-pool implementation can implement
/// this interface without changing TaskExecutor.
///
/// Resource exhaustion and unavailable execution facilities
/// must be reported through ZurError.
///
/// Implementations must not invent execution success.
pub trait TaskDispatcher: Send + Sync + 'static {
    /// Transfers a task job to the execution facility.
    ///
    /// This method must not claim acceptance unless ownership
    /// and exactly-once execution responsibility are established.
    ///
    /// An error guarantees that the job will not execute.
    fn dispatch(
        &self,
        job: DispatchJob,
    ) -> ZurResult<()>;
}

// ============================================================================
// INLINE EXECUTION FACILITY
// ============================================================================

/// Executes accepted operations immediately on the calling thread.
///
/// This is a useful production execution mode for:
///
/// - Small workloads.
/// - Embedded environments.
/// - Deterministic execution.
/// - Resource-constrained environments.
/// - Testing.
/// - Systems without an available worker pool.
///
/// It does not create threads.
///
/// It does not provide parallel execution.
///
/// It does not implement scheduling.
#[derive(Debug, Default, Clone, Copy)]
pub struct InlineTaskDispatcher;

impl TaskDispatcher for InlineTaskDispatcher {
    fn dispatch(
        &self,
        job: DispatchJob,
    ) -> ZurResult<()> {
        job();

        Ok(())
    }
}

// ============================================================================
// SUBMISSION REGISTRY
// ============================================================================

/// Tracks task submissions owned by one TaskExecutor.
///
/// This is an executor-local coordination mechanism.
///
/// It is not a global task registry.
///
/// Entries are removed when execution finishes or dispatch fails.
#[derive(Default)]
struct SubmissionRegistry {
    active: Mutex<BTreeSet<TaskId>>,
}

impl SubmissionRegistry {
    /// Acquires exclusive submission ownership for a task.
    fn acquire(
        self: &Arc<Self>,
        task_id: TaskId,
    ) -> ZurResult<SubmissionLease> {
        let mut active = self.active.lock().map_err(|_| {
            internal_error(
                "submission registry lock was poisoned",
                "acquire",
            )
        })?;

        if !active.insert(task_id.clone()) {
            return Err(
                ZurError::invalid_input(
                    "task already has an active submission \
                     in this executor",
                )
                .with_component(COMPONENT)
                .with_operation("acquire"),
            );
        }

        Ok(SubmissionLease {
            registry: Arc::clone(self),
            task_id,
        })
    }

    /// Removes a task submission.
    fn release(
        &self,
        task_id: &TaskId,
    ) {
        match self.active.lock() {
            Ok(mut active) => {
                active.remove(task_id);
            }

            // Recover only this internal coordination structure.
            // A poisoned lock must never leave a task permanently
            // registered merely because a previous observer panicked.
            Err(poisoned) => {
                poisoned.into_inner().remove(task_id);
            }
        }
    }
}

/// RAII ownership of one executor submission.
///
/// Dropping the lease releases the executor-local submission entry.
struct SubmissionLease {
    registry: Arc<SubmissionRegistry>,
    task_id: TaskId,
}

impl Drop for SubmissionLease {
    fn drop(&mut self) {
        self.registry.release(&self.task_id);
    }
}

// ============================================================================
// TASK EXECUTOR
// ============================================================================

/// Coordinates execution of ZUR-managed concurrent tasks.
///
/// The executor does not own the scheduler or worker pool.
///
/// It receives an explicitly supplied TaskDispatcher.
///
/// Each executor maintains its own submission coordination state.
///
/// For a given task, callers must designate one executor as the
/// submission authority. Cross-executor duplicate prevention
/// requires a higher-level task ownership contract.
pub struct TaskExecutor<D>
where
    D: TaskDispatcher + ?Sized,
{
    dispatcher: Arc<D>,
    submissions: Arc<SubmissionRegistry>,
}

impl<D> Clone for TaskExecutor<D>
where
    D: TaskDispatcher + ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            dispatcher: Arc::clone(&self.dispatcher),
            submissions: Arc::clone(&self.submissions),
        }
    }
}

impl<D> TaskExecutor<D>
where
    D: TaskDispatcher + ?Sized,
{
    /// Creates an executor using an explicit dispatcher.
    ///
    /// No workers, threads, devices or runtime services
    /// are created by this constructor.
    #[must_use]
    pub fn new(dispatcher: Arc<D>) -> Self {
        Self {
            dispatcher,
            submissions: Arc::new(
                SubmissionRegistry::default(),
            ),
        }
    }

    /// Returns a reference to the supplied execution facility.
    #[must_use]
    pub fn dispatcher(&self) -> &D {
        &self.dispatcher
    }

    /// Submits an existing task for execution.
    ///
    /// The task must be Created or Ready.
    ///
    /// Created tasks are transitioned to Ready before dispatch.
    ///
    /// Ready tasks may be retried following a previously rejected
    /// dispatch.
    ///
    /// The task's lifecycle is subsequently controlled by the
    /// execution authority through this executor.
    ///
    /// The caller retains its Task handle and can observe completion
    /// using Task::wait or Task::try_completion.
    ///
    /// A successful return means that the dispatcher accepted
    /// the operation. It does not necessarily mean execution
    /// has already completed.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The task is not eligible for execution.
    /// - Cancellation prevents submission.
    /// - Another submission is already active in this executor.
    /// - The dispatcher rejects the operation.
    /// - The submission registry is unavailable.
    pub fn submit<T, F>(
        &self,
        task: Task<T>,
        operation: F,
    ) -> ZurResult<()>
    where
        T: Send + Sync + 'static,
        F: FnOnce(TaskExecutionContext) -> ZurResult<T>
            + Send
            + 'static,
    {
        let snapshot = task.snapshot()?;

        match snapshot.status() {
            TaskStatus::Created => {
                task.mark_ready()?;
            }

            TaskStatus::Ready => {}

            TaskStatus::CancellationRequested => {
                // This executor has not accepted this submission.
                // If cancellation was already requested before
                // dispatch, no execution remains to acknowledge.
                task.acknowledge_cancelled()?;

                return Err(
                    ZurError::cancelled(
                        "task was cancelled before execution submission",
                    )
                    .with_component(COMPONENT)
                    .with_operation("submit"),
                );
            }

            TaskStatus::Running => {
                return Err(
                    ZurError::invalid_input(
                        "task is already running",
                    )
                    .with_component(COMPONENT)
                    .with_operation("submit"),
                );
            }

            TaskStatus::Completed
            | TaskStatus::Failed
            | TaskStatus::Cancelled => {
                return Err(
                    ZurError::invalid_input(
                        "terminal tasks cannot be submitted again",
                    )
                    .with_component(COMPONENT)
                    .with_operation("submit"),
                );
            }
        }

        // Acquire submission ownership before transferring work.
        let lease = self
            .submissions
            .acquire(task.id().clone())?;

        let execution_context =
            TaskExecutionContext::new(&task);

        let task_for_execution = task.clone();

        let job: DispatchJob = Box::new(move || {
            // The lease remains alive for the complete execution.
            let _submission_lease = lease;

            execute_task(
                task_for_execution,
                execution_context,
                operation,
            );
        });

        // The dispatcher contract guarantees that an Err means
        // the operation was not accepted and will never execute.
        //
        // If rejected, dropping the job releases its submission lease.
        self.dispatcher
            .dispatch(job)
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("dispatch")
            })
    }
}

// ============================================================================
// TASK EXECUTION
// ============================================================================

/// Executes one accepted task operation.
///
/// This function is called only by an execution facility.
///
/// It does not schedule or select the task.
fn execute_task<T, F>(
    task: Task<T>,
    context: TaskExecutionContext,
    operation: F,
)
where
    T: Send + Sync + 'static,
    F: FnOnce(TaskExecutionContext) -> ZurResult<T>
        + Send
        + 'static,
{
    // Cancellation may have been requested after dispatch
    // but before the execution facility started this operation.
    //
    // In that case the task never enters Running.
    if let Err(error) = task.mark_running() {
        let snapshot = task.snapshot();

        if let Ok(snapshot) = snapshot {
            if snapshot.status() == TaskStatus::CancellationRequested {
                let _ = task.acknowledge_cancelled();
            } else if !snapshot.is_terminal() {
                // The execution authority encountered an actual
                // lifecycle failure before beginning user code.
                //
                // Task lifecycle validation remains authoritative.
                let _ = error;
            }
        }

        return;
    }

    // Catch unwinding panics so that an operation panic does not
    // leave the task permanently marked as Running.
    //
    // panic=abort cannot be recovered from.
    let execution = catch_unwind(AssertUnwindSafe(|| {
        operation(context)
    }));

    match execution {
        Ok(Ok(value)) => {
            // Completion is published only after the operation
            // actually returned a successful value.
            //
            // Completion is allowed to win a cancellation race
            // when the operation genuinely finishes successfully.
            let _ = task.complete(value);
        }

        Ok(Err(error)) => {
            publish_operation_error(&task, error);
        }

        Err(_panic_payload) => {
            let error = internal_error(
                "task operation terminated with a panic",
                "execute_task",
            );

            let _ = task.fail(error);
        }
    }
}

// ============================================================================
// FAILURE AND CANCELLATION PUBLICATION
// ============================================================================

/// Publishes an actual operation failure or acknowledged cancellation.
fn publish_operation_error<T>(
    task: &Task<T>,
    error: ZurError,
)
where
    T: Send + Sync + 'static,
{
    let cancellation_requested =
        task.is_cancellation_requested();

    // An operation error is cancellation only when both:
    //
    // 1. Cancellation was actually requested.
    // 2. The operation reported the canonical cancellation kind.
    //
    // Other failures remain failures even when cancellation
    // happened concurrently.
    if cancellation_requested
        && error.kind() == ErrorKind::Cancelled
    {
        let _ = task.acknowledge_cancelled();
        return;
    }

    // Publish the actual failure.
    //
    // CancellationRequested -> Failed is valid because an
    // execution failure can win a cancellation race.
    let _ = task.fail(
        error
            .with_component(COMPONENT)
            .with_operation("execute_task"),
    );
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured internal executor error.
fn internal_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// CONVENIENCE CONSTRUCTION
// ============================================================================

impl TaskExecutor<InlineTaskDispatcher> {
    /// Creates an executor that runs operations on the calling thread.
    ///
    /// No threads or additional runtime services are created.
    #[must_use]
    pub fn inline() -> Self {
        Self::new(Arc::new(InlineTaskDispatcher))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::{
        IdNamespace,
        RuntimeOperationId,
    };

    fn task<T>() -> Task<T>
    where
        T: Send + Sync + 'static,
    {
        let namespace = IdNamespace::new(
            "zamani.test.concurrency",
        )
        .expect("valid namespace");

        let id = RuntimeOperationId::new(
            namespace,
            vec![1],
        )
        .expect("valid task identity");

        Task::new(id)
    }

    #[test]
    fn inline_executor_runs_operation() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        executor
            .submit(task.clone(), |_context| Ok(42))
            .expect("submission succeeds");

        let outcome = task.wait().expect("completion exists");

        assert!(outcome.is_success());

        assert_eq!(
            outcome.value().map(|value| **value),
            Some(42),
        );

        assert_eq!(
            task.snapshot().unwrap().status(),
            TaskStatus::Completed,
        );
    }

    #[test]
    fn operation_failure_is_published() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        executor
            .submit(task.clone(), |_context| {
                Err(
                    ZurError::invalid_input(
                        "test operation failure",
                    ),
                )
            })
            .expect("submission succeeds");

        let outcome = task.wait().expect("completion exists");

        assert!(outcome.is_failure());

        assert_eq!(
            task.snapshot().unwrap().status(),
            TaskStatus::Failed,
        );
    }

    #[test]
    fn task_cancellation_is_observable() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        let task_for_operation = task.clone();

        executor
            .submit(task.clone(), move |context| {
                task_for_operation
                    .request_cancel("test cancellation")?;

                context.check_cancelled()?;

                Ok(42)
            })
            .expect("dispatcher accepts operation");

        let outcome = task.wait().expect("completion exists");

        assert!(outcome.is_cancelled());

        assert_eq!(
            task.snapshot().unwrap().status(),
            TaskStatus::Cancelled,
        );
    }

    #[test]
    fn operation_panic_does_not_leave_task_running() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        executor
            .submit(task.clone(), |_context| -> ZurResult<usize> {
                panic!("intentional executor test panic");
            })
            .expect("submission succeeds");

        let outcome = task.wait().expect("completion exists");

        assert!(outcome.is_failure());

        assert_eq!(
            task.snapshot().unwrap().status(),
            TaskStatus::Failed,
        );
    }

    #[test]
    fn pre_cancelled_task_is_not_executed() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        task.request_cancel("cancel before submission")
            .expect("cancellation requested");

        let result = executor.submit(
            task.clone(),
            |_context| Ok(42),
        );

        assert!(result.is_err());

        assert_eq!(
            task.snapshot().unwrap().status(),
            TaskStatus::Cancelled,
        );
    }

    #[test]
    fn terminal_task_cannot_be_submitted_again() {
        let executor = TaskExecutor::inline();

        let task = task::<usize>();

        executor
            .submit(task.clone(), |_context| Ok(1))
            .expect("first execution succeeds");

        let second = executor.submit(
            task.clone(),
            |_context| Ok(2),
        );

        assert!(second.is_err());

        let outcome = task.wait().unwrap();

        assert_eq!(
            outcome.value().map(|value| **value),
            Some(1),
        );
    }

    #[test]
    fn cloned_executor_shares_submission_registry() {
        let executor = TaskExecutor::inline();

        let cloned = executor.clone();

        assert!(
            Arc::ptr_eq(
                &executor.submissions,
                &cloned.submissions,
            )
        );
    }
}
