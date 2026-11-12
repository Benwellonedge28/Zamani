
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Local Scheduler
//!
//! Path: src/zur/scheduling/local.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides an explicitly configured, process-local FIFO scheduler
//! for ZUR-managed execution jobs.
//!
//! The scheduler accepts jobs through the canonical TaskDispatcher
//! interface and executes them using explicitly created local workers.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - LocalScheduler.
//! - LocalSchedulerConfig.
//! - Local worker lifecycle.
//! - Local FIFO job dispatch.
//! - Optional queue-capacity enforcement.
//! - Graceful scheduler shutdown.
//! - Worker-level panic containment.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Task identity.
//! - Task lifecycle.
//! - Task completion.
//! - Task cancellation.
//! - Task execution semantics.
//! - Compiler scheduling.
//! - Distributed scheduling.
//! - Quantum scheduling.
//! - Hybrid execution planning.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Backend selection.
//! - Domain-specific computation.
//! - A second executor implementation.
//!
//! EXISTING AUTHORITIES
//!
//! - zur/concurrency/task.rs:
//!   Canonical Task and task lifecycle.
//!
//! - zur/concurrency/executor.rs:
//!   TaskDispatcher and TaskExecutor.
//!
//! - zur/execution/cancellation.rs:
//!   Execution cancellation.
//!
//! - zur/policy.rs:
//!   Operational execution and resource-exhaustion policies.
//!
//! - zur/error.rs:
//!   Canonical ZurError and ZurResult.
//!
//! - zur/core/context.rs:
//!   RuntimeContext.
//!
//! GRAMMAR AUTHORITY
//!
//! - grammar/spec/concurrency.md
//! - grammar/spec/determinism.md
//! - grammar/spec/resources.md
//! - grammar/concurrency/tasks.g4
//! - grammar/concurrency/parallel.g4
//! - grammar/concurrency/task-parallel.g4
//!
//! The grammar and semantic layer describe logical concurrency.
//! This module implements one possible local execution strategy.
//!
//! SCALABILITY
//!
//! There is no artificial maximum for:
//!
//! - Jobs.
//! - Tasks.
//! - Worker threads.
//! - Scheduler instances.
//! - Scheduling operations.
//!
//! Worker count is explicitly configured or obtained from the
//! operating system's available_parallelism information.
//!
//! Queue capacity is optionally configured by the caller.
//!
//! An unbounded queue is constrained by actual allocator and
//! address-space resources.
//!
//! A bounded queue rejects new work when its configured capacity
//! is exhausted. It never silently discards accepted work.
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
//! No global scheduler.
//! No implicit worker creation.
//!
//! EXECUTION GUARANTEES
//!
//! - A successful dispatch means the queue accepted the job.
//! - A failed dispatch means the job was not accepted.
//! - Accepted jobs are processed at most once.
//! - Accepted queued jobs are drained during graceful shutdown.
//! - Jobs execute outside the receiver synchronization lock.
//! - Job panics do not intentionally terminate the worker pool.
//! - Concurrent producers are ordered by their successful enqueue
//!   operations, not by source-level task identity.
//! - Completion ordering is not guaranteed.
//!
//! A successful dispatch does not imply task completion.

use std::num::NonZeroUsize;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{
    self,
    Receiver,
    Sender,
    SyncSender,
    TrySendError,
};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use crate::zur::concurrency::executor::{
    DispatchJob,
    TaskDispatcher,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.scheduling.local";

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration for a local scheduler.
///
/// Worker count must be explicitly supplied.
///
/// Queue capacity is optional:
///
/// - `None`: use an unbounded queue.
/// - `Some(capacity)`: enforce the supplied queue budget.
///
/// No hardware-specific worker count or queue limit is assumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSchedulerConfig {
    worker_count: NonZeroUsize,
    queue_capacity: Option<NonZeroUsize>,
    thread_name_prefix: Option<String>,
}

impl LocalSchedulerConfig {
    /// Creates a scheduler configuration.
    ///
    /// The worker count must be nonzero.
    ///
    /// No workers are created by this constructor.
    #[must_use]
    pub fn new(worker_count: NonZeroUsize) -> Self {
        Self {
            worker_count,
            queue_capacity: None,
            thread_name_prefix: None,
        }
    }

    /// Creates a configuration using the operating system's
    /// available parallelism information.
    ///
    /// This is an explicit convenience operation.
    ///
    /// It does not silently substitute an arbitrary worker count
    /// when the operating system cannot provide the information.
    pub fn from_available_parallelism() -> ZurResult<Self> {
        let worker_count = thread::available_parallelism()
            .map_err(|error| {
                ZurError::resource_unavailable(
                    "operating system could not determine available parallelism",
                )
                .with_source(error)
                .with_component(COMPONENT)
                .with_operation("from_available_parallelism")
            })?;

        Ok(Self::new(worker_count))
    }

    /// Sets an optional maximum number of queued jobs.
    ///
    /// This is an operational queue budget, not a language-level
    /// task or concurrency ceiling.
    ///
    /// When omitted, the scheduler uses an unbounded queue.
    #[must_use]
    pub fn with_queue_capacity(
        mut self,
        capacity: Option<NonZeroUsize>,
    ) -> Self {
        self.queue_capacity = capacity;
        self
    }

    /// Sets a prefix for worker thread names.
    ///
    /// Actual worker names are derived from this prefix and
    /// their scheduler-local creation indices.
    ///
    /// An empty prefix or one containing a null character is rejected.
    pub fn with_thread_name_prefix(
        mut self,
        prefix: impl Into<String>,
    ) -> ZurResult<Self> {
        let prefix = prefix.into();

        if prefix.trim().is_empty() || prefix.contains('\0') {
            return Err(
                ZurError::configuration(
                    "worker thread name prefix must be nonempty and contain no null character",
                )
                .with_component(COMPONENT)
                .with_operation("with_thread_name_prefix"),
            );
        }

        self.thread_name_prefix = Some(prefix);

        Ok(self)
    }

    /// Returns the configured worker count.
    #[must_use]
    pub const fn worker_count(&self) -> NonZeroUsize {
        self.worker_count
    }

    /// Returns the optional queue budget.
    #[must_use]
    pub const fn queue_capacity(&self) -> Option<NonZeroUsize> {
        self.queue_capacity
    }

    /// Returns the optional worker-name prefix.
    #[must_use]
    pub fn thread_name_prefix(&self) -> Option<&str> {
        self.thread_name_prefix.as_deref()
    }
}

// ============================================================================
// INTERNAL QUEUE
// ============================================================================

/// Internal sender abstraction.
///
/// Both implementations preserve FIFO message delivery.
///
/// Bounded queues use nonblocking submission so dispatch never
/// unexpectedly blocks a caller waiting for queue capacity.
enum JobSender {
    Unbounded(Sender<DispatchJob>),
    Bounded(SyncSender<DispatchJob>),
}

impl JobSender {
    /// Attempts to transfer ownership of one job.
    ///
    /// On failure, ownership is returned internally and the
    /// rejected job is dropped before returning the error.
    fn submit(&self, job: DispatchJob) -> ZurResult<()> {
        match self {
            Self::Unbounded(sender) => {
                sender.send(job).map_err(|error| {
                    drop(error.0);

                    ZurError::service_unavailable(
                        "local scheduler workers are unavailable",
                    )
                    .with_component(COMPONENT)
                    .with_operation("dispatch")
                })
            }

            Self::Bounded(sender) => {
                match sender.try_send(job) {
                    Ok(()) => Ok(()),

                    Err(TrySendError::Full(rejected)) => {
                        drop(rejected);

                        Err(
                            ZurError::resource_unavailable(
                                "local scheduler queue budget is exhausted",
                            )
                            .with_component(COMPONENT)
                            .with_operation("dispatch"),
                        )
                    }

                    Err(TrySendError::Disconnected(rejected)) => {
                        drop(rejected);

                        Err(
                            ZurError::service_unavailable(
                                "local scheduler workers are unavailable",
                            )
                            .with_component(COMPONENT)
                            .with_operation("dispatch"),
                        )
                    }
                }
            }
        }
    }
}

// ============================================================================
// SCHEDULER STATE
// ============================================================================

/// Mutable scheduler lifecycle state.
struct SchedulerState {
    /// Present while new work can be accepted.
    sender: Option<JobSender>,

    /// Worker handles retained for graceful shutdown.
    workers: Option<Vec<JoinHandle<()>>>,
}

/// Acquires a mutex while recovering an internally poisoned lock.
///
/// Scheduler state contains only operational coordination data.
/// Recovering the lock does not establish task completion or
/// claim that an execution operation succeeded.
fn lock_unpoisoned<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

// ============================================================================
// LOCAL SCHEDULER
// ============================================================================

/// Process-local FIFO scheduler.
///
/// The scheduler implements the canonical TaskDispatcher contract.
///
/// It can therefore be supplied directly to the existing
/// TaskExecutor.
///
/// # Execution
///
/// Each configured worker repeatedly receives one accepted job
/// and executes it outside the queue synchronization lock.
///
/// Multiple workers may execute jobs concurrently.
///
/// A single worker executes accepted jobs sequentially.
///
/// # Queue behavior
///
/// An unbounded queue accepts work subject to available memory.
///
/// A bounded queue rejects work immediately when its configured
/// capacity is exhausted.
///
/// The scheduler never silently discards a successfully accepted job.
///
/// # Shutdown
///
/// Call [`LocalScheduler::shutdown`] to close submission and wait
/// for worker termination.
///
/// Dropping the scheduler closes submission and detaches worker
/// handles. Workers drain already accepted jobs before exiting.
///
/// # Thread safety
///
/// The scheduler is safe to share between threads.
pub struct LocalScheduler {
    state: Mutex<SchedulerState>,

    /// Serializes explicit shutdown operations.
    shutdown_serial: Mutex<()>,

    worker_count: NonZeroUsize,

    /// Number of job closures that panicked outside their own
    /// execution-level panic handling.
    panicked_jobs: Arc<AtomicUsize>,
}

impl LocalScheduler {
    /// Constructs a local scheduler.
    ///
    /// Worker creation is explicit and occurs during this call.
    ///
    /// If any worker cannot be created, previously created workers
    /// are shut down and joined before the constructor returns an error.
    pub fn new(config: LocalSchedulerConfig) -> ZurResult<Self> {
        let (sender, receiver) = match config.queue_capacity {
            Some(capacity) => {
                let (sender, receiver) =
                    mpsc::sync_channel::<DispatchJob>(capacity.get());

                (JobSender::Bounded(sender), receiver)
            }

            None => {
                let (sender, receiver) =
                    mpsc::channel::<DispatchJob>();

                (JobSender::Unbounded(sender), receiver)
            }
        };

        let receiver = Arc::new(Mutex::new(receiver));

        let panicked_jobs = Arc::new(AtomicUsize::new(0));

        let mut workers = Vec::new();

        for worker_index in 0..config.worker_count.get() {
            let worker_receiver = Arc::clone(&receiver);

            let worker_panic_counter =
                Arc::clone(&panicked_jobs);

            let mut builder = thread::Builder::new();

            if let Some(prefix) = config.thread_name_prefix.as_ref() {
                builder = builder.name(
                    format!("{prefix}-{worker_index}"),
                );
            }

            let spawn_result = builder.spawn(move || {
                worker_loop(
                    worker_receiver,
                    worker_panic_counter,
                );
            });

            match spawn_result {
                Ok(worker) => {
                    workers.push(worker);
                }

                Err(error) => {
                    // Closing every sender disconnects the channel.
                    // Existing workers then leave their receive loops.
                    drop(sender);
                    drop(receiver);

                    for worker in workers {
                        let _ = worker.join();
                    }

                    return Err(
                        ZurError::resource_unavailable(
                            "failed to create a local scheduler worker",
                        )
                        .with_source(error)
                        .with_component(COMPONENT)
                        .with_operation("new"),
                    );
                }
            }
        }

        Ok(Self {
            state: Mutex::new(SchedulerState {
                sender: Some(sender),
                workers: Some(workers),
            }),
            shutdown_serial: Mutex::new(()),
            worker_count: config.worker_count,
            panicked_jobs,
        })
    }

    /// Constructs a scheduler using the operating system's
    /// available parallelism information.
    ///
    /// This operation explicitly creates the resulting workers.
    pub fn using_available_parallelism() -> ZurResult<Self> {
        Self::new(
            LocalSchedulerConfig::from_available_parallelism()?,
        )
    }

    /// Returns the configured worker count.
    ///
    /// This is configuration information, not proof that the
    /// operating system will continuously provide that capacity.
    #[must_use]
    pub const fn worker_count(&self) -> NonZeroUsize {
        self.worker_count
    }

    /// Returns the number of worker-job panics observed.
    ///
    /// Panics are contained so that one failing job does not
    /// intentionally terminate the entire worker pool.
    ///
    /// This counter is observational. It does not replace
    /// task-level failure reporting.
    #[must_use]
    pub fn panicked_jobs(&self) -> usize {
        self.panicked_jobs.load(Ordering::Relaxed)
    }

    /// Returns whether the scheduler has stopped accepting work.
    #[must_use]
    pub fn is_shutdown(&self) -> bool {
        lock_unpoisoned(&self.state)
            .sender
            .is_none()
    }

    /// Stops accepting new jobs and waits for workers to finish.
    ///
    /// Previously accepted jobs are drained before workers exit.
    ///
    /// This method is idempotent.
    ///
    /// If called from one of its own worker threads, that worker's
    /// handle is detached rather than joined against itself.
    ///
    /// # Errors
    ///
    /// Returns an error if a worker thread terminates unexpectedly
    /// with an uncaught panic.
    pub fn shutdown(&self) -> ZurResult<()> {
        let _shutdown_guard =
            lock_unpoisoned(&self.shutdown_serial);

        let (sender, workers) = {
            let mut state = lock_unpoisoned(&self.state);

            (
                state.sender.take(),
                state.workers.take().unwrap_or_default(),
            )
        };

        // Closing the final sender causes workers to drain the
        // accepted queue and exit when it becomes empty.
        drop(sender);

        let current_thread = thread::current().id();

        let mut worker_panicked = false;

        for worker in workers {
            if worker.thread().id() == current_thread {
                // Joining the current worker would deadlock.
                // Dropping its JoinHandle safely detaches it.
                drop(worker);
                continue;
            }

            if worker.join().is_err() {
                worker_panicked = true;
            }
        }

        if worker_panicked {
            return Err(
                ZurError::internal(
                    "one or more local scheduler workers terminated unexpectedly",
                )
                .with_component(COMPONENT)
                .with_operation("shutdown"),
            );
        }

        Ok(())
    }
}

// ============================================================================
// TASK DISPATCHER INTEGRATION
// ============================================================================

impl TaskDispatcher for LocalScheduler {
    /// Accepts an owned execution job into the local FIFO queue.
    ///
    /// A successful result means the job has been accepted.
    ///
    /// It does not mean that the job has started or completed.
    ///
    /// On failure, the job is not retained by the scheduler.
    fn dispatch(&self, job: DispatchJob) -> ZurResult<()> {
        let state = lock_unpoisoned(&self.state);

        let sender = state.sender.as_ref().ok_or_else(|| {
            ZurError::service_unavailable(
                "local scheduler has been shut down",
            )
            .with_component(COMPONENT)
            .with_operation("dispatch")
        })?;

        sender.submit(job)
    }
}

// ============================================================================
// WORKER LOOP
// ============================================================================

/// Executes jobs received from the shared local queue.
///
/// The receiver lock is held only while receiving a job.
///
/// Actual job execution occurs outside the lock, allowing
/// independently configured workers to execute concurrently.
fn worker_loop(
    receiver: Arc<Mutex<Receiver<DispatchJob>>>,
    panicked_jobs: Arc<AtomicUsize>,
) {
    loop {
        let received = {
            let receiver_guard = lock_unpoisoned(&receiver);

            receiver_guard.recv()
        };

        let job = match received {
            Ok(job) => job,
            Err(_) => return,
        };

        // The task executor owns execution-level failure reporting.
        // This boundary protects the worker loop from an unexpected
        // panic escaping an accepted type-erased dispatch job.
        //
        // panic=abort cannot be recovered from.
        if catch_unwind(AssertUnwindSafe(job)).is_err() {
            // Saturation is confined to an observational diagnostic
            // counter. It never limits the number of accepted jobs.
            let _ = panicked_jobs.fetch_update(
                Ordering::Relaxed,
                Ordering::Relaxed,
                |current| Some(current.saturating_add(1)),
            );
        }
    }
}

// ============================================================================
// DROP
// ============================================================================

impl Drop for LocalScheduler {
    /// Closes submission and detaches worker handles.
    ///
    /// Explicit shutdown should be preferred when the caller needs
    /// confirmation that all workers have terminated.
    ///
    /// Dropping the scheduler does not silently discard queued jobs:
    /// the workers retain their receivers and drain accepted jobs
    /// before observing channel disconnection.
    fn drop(&mut self) {
        let state = match self.state.get_mut() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };

        let sender = state.sender.take();

        // Close the queue before detaching worker handles.
        drop(sender);

        let workers = state.workers.take();

        // Dropping JoinHandles detaches the threads.
        // The workers exit after draining accepted jobs.
        drop(workers);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    fn one_worker() -> NonZeroUsize {
        NonZeroUsize::new(1).expect("one is nonzero")
    }

    fn one_queue_slot() -> NonZeroUsize {
        NonZeroUsize::new(1).expect("one is nonzero")
    }

    fn receive_completion(
        receiver: &Receiver<()>,
    ) {
        receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("worker should complete the job");
    }

    #[test]
    fn scheduler_executes_accepted_jobs() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        let (sender, receiver) = mpsc::channel();

        scheduler
            .dispatch(Box::new(move || {
                sender.send(()).expect("receiver exists");
            }))
            .expect("job should be accepted");

        receive_completion(&receiver);

        scheduler
            .shutdown()
            .expect("shutdown should succeed");
    }

    #[test]
    fn scheduler_runs_jobs_on_worker_threads() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        let caller = thread::current().id();

        let (sender, receiver) = mpsc::channel();

        scheduler
            .dispatch(Box::new(move || {
                sender
                    .send(thread::current().id())
                    .expect("receiver exists");
            }))
            .expect("job should be accepted");

        let worker_id = receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("worker should execute job");

        assert_ne!(caller, worker_id);

        scheduler.shutdown().expect("shutdown should succeed");
    }

    #[test]
    fn scheduler_drains_accepted_jobs_on_shutdown() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        let completed = Arc::new(AtomicUsize::new(0));

        for _ in 0..32 {
            let completed = Arc::clone(&completed);

            scheduler
                .dispatch(Box::new(move || {
                    completed.fetch_add(1, Ordering::Relaxed);
                }))
                .expect("job should be accepted");
        }

        scheduler
            .shutdown()
            .expect("shutdown should drain accepted jobs");

        assert_eq!(
            completed.load(Ordering::Relaxed),
            32,
        );
    }

    #[test]
    fn scheduler_rejects_work_after_shutdown() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        scheduler.shutdown().expect("shutdown should succeed");

        assert!(scheduler.is_shutdown());

        let result = scheduler.dispatch(Box::new(|| {}));

        assert!(result.is_err());
    }

    #[test]
    fn shutdown_is_idempotent() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        scheduler.shutdown().expect("first shutdown");

        scheduler.shutdown().expect("second shutdown");
    }

    #[test]
    fn bounded_queue_rejects_excess_work() {
        let config = LocalSchedulerConfig::new(one_worker())
            .with_queue_capacity(Some(one_queue_slot()));

        let scheduler = LocalScheduler::new(config)
            .expect("scheduler should initialize");

        let (started_sender, started_receiver) =
            mpsc::channel();

        let (release_sender, release_receiver) =
            mpsc::channel();

        // Keep the only worker occupied.
        scheduler
            .dispatch(Box::new(move || {
                started_sender
                    .send(())
                    .expect("started receiver exists");

                release_receiver
                    .recv()
                    .expect("release sender exists");
            }))
            .expect("first job should be accepted");

        receive_completion(&started_receiver);

        // The single queue slot can now accept one additional job.
        scheduler
            .dispatch(Box::new(|| {}))
            .expect("second job should be accepted");

        // The worker is occupied and the queue is full.
        let result = scheduler.dispatch(Box::new(|| {}));

        assert!(result.is_err());

        release_sender
            .send(())
            .expect("worker release should succeed");

        scheduler
            .shutdown()
            .expect("accepted jobs should drain");
    }

    #[test]
    fn worker_survives_a_panicking_job() {
        let scheduler = LocalScheduler::new(
            LocalSchedulerConfig::new(one_worker()),
        )
        .expect("scheduler should initialize");

        scheduler
            .dispatch(Box::new(|| {
                panic!("intentional worker-job panic");
            }))
            .expect("job should be accepted");

        let (sender, receiver) = mpsc::channel();

        scheduler
            .dispatch(Box::new(move || {
                sender.send(()).expect("receiver exists");
            }))
            .expect("subsequent job should be accepted");

        receive_completion(&receiver);

        assert_eq!(scheduler.panicked_jobs(), 1);

        scheduler
            .shutdown()
            .expect("worker should shut down");
    }

    #[test]
    fn available_parallelism_configuration_is_explicit() {
        let configuration =
            LocalSchedulerConfig::from_available_parallelism();

        assert!(configuration.is_ok());
    }

    #[test]
    fn scheduler_configuration_preserves_queue_budget() {
        let configuration = LocalSchedulerConfig::new(one_worker())
            .with_queue_capacity(Some(one_queue_slot()));

        assert_eq!(
            configuration.queue_capacity(),
            Some(one_queue_slot()),
        );
    }

    #[test]
    fn thread_name_prefix_rejects_invalid_input() {
        let configuration = LocalSchedulerConfig::new(one_worker());

        assert!(
            configuration
                .with_thread_name_prefix("")
                .is_err()
        );

        assert!(
            configuration
                .with_thread_name_prefix("invalid\0prefix")
                .is_err()
        );
    }
}
