//! Zamani Quantum Error-Correction Scheduler
//!
//! Production-grade scheduling infrastructure for QEC workloads.
//!
//! Responsibilities:
//! - Schedule decoder/simulation/benchmark/diagnostic jobs.
//! - Enforce priority, deadline, CPU, memory, and parallelism policies.
//! - Provide bounded worker pools and backpressure.
//! - Support single-thread, multithread, multi-process, distributed,
//!   and accelerated execution modes through backend abstraction.
//! - Integrate with the QEC resource, cancellation, configuration,
//!   limits, metrics, telemetry, checkpoint, deterministic, and capability
//!   layers without duplicating their responsibilities.
//! - Guarantee deterministic admission ordering when deterministic mode
//!   is enabled.
//! - Fail safely instead of panicking on malformed scheduling requests.
//!
//! Design principle:
//!
//!     Untrusted Job
//!          |
//!          v
//!     Scheduler Validation
//!          |
//!          v
//!     Admission Control
//!          |
//!          +----> Resource / Capability Checks
//!          |
//!          v
//!     Priority Queue
//!          |
//!          v
//!     Worker Selection
//!          |
//!          v
//!     Backend Execution
//!          |
//!          v
//!     Metrics / Telemetry / Checkpoint
//!
//! The scheduler deliberately does NOT implement decoding algorithms.
//! MWPM, Union-Find, simulation, etc. remain responsible for their own
//! mathematical execution.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering as AtomicOrdering},
    Arc, Mutex,
};
use std::time::{Duration, Instant, SystemTime};

/// Scheduler-local result type.
///
/// If the repository's `errors.rs` is already available, the integration
/// layer should map `SchedulerError` into `QecError::...` rather than
/// introducing a second global error hierarchy.
pub type SchedulerResult<T> = Result<T, SchedulerError>;

/// Unique scheduler job identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobId(u64);

impl JobId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Scheduler worker identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WorkerId(u64);

impl WorkerId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// Supported execution modes.
///
/// The scheduler does not require any particular transport or accelerator
/// implementation. Those belong to `backend.rs` and the distributed/runtime
/// layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ExecutionMode {
    SingleThread,
    MultiThread,
    MultiProcess,
    Distributed,
    Accelerated,
}

impl ExecutionMode {
    pub fn is_parallel(self) -> bool {
        !matches!(self, Self::SingleThread)
    }

    pub fn requires_remote_capability(self) -> bool {
        matches!(self, Self::MultiProcess | Self::Distributed)
    }

    pub fn requires_accelerator(self) -> bool {
        matches!(self, Self::Accelerated)
    }
}

/// Workload class.
///
/// This allows the scheduler to prioritize operational QEC work over
/// background experimentation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum JobKind {
    LogicalOperation,
    Decode,
    Simulation,
    ThresholdBenchmark,
    Diagnostic,
}

/// Explicit priority class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

impl Priority {
    fn rank(self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::High => 4,
            Self::Normal => 3,
            Self::Low => 2,
            Self::Background => 1,
        }
    }
}

/// Scheduling deadline.
///
/// `None` means that the job has no wall-clock deadline beyond configured
/// resource limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Deadline(pub Option<Instant>);

impl Deadline {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn after(duration: Duration) -> Self {
        Self(Some(Instant::now() + duration))
    }

    pub fn expired(self) -> bool {
        self.0.is_some_and(|deadline| Instant::now() >= deadline)
    }

    pub fn remaining(self) -> Option<Duration> {
        self.0.map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }
}

/// Resource reservation requested by a job.
///
/// The scheduler reserves resources before admission to prevent aggregate
/// overcommitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceQuota {
    pub cpu_units: u32,
    pub memory_bytes: u64,
    pub parallel_workers: u32,
    pub decoder_iterations: u64,
}

impl ResourceQuota {
    pub const fn zero() -> Self {
        Self {
            cpu_units: 0,
            memory_bytes: 0,
            parallel_workers: 0,
            decoder_iterations: 0,
        }
    }

    pub fn validate(self) -> SchedulerResult<()> {
        if self.cpu_units == 0 {
            return Err(SchedulerError::InvalidQuota(
                "cpu_units must be greater than zero".into(),
            ));
        }

        if self.parallel_workers == 0 {
            return Err(SchedulerError::InvalidQuota(
                "parallel_workers must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

/// Scheduler-wide resource budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulerBudget {
    pub cpu_units: u32,
    pub memory_bytes: u64,
    pub parallel_workers: u32,
    pub max_queued_jobs: usize,
}

impl SchedulerBudget {
    pub const fn conservative() -> Self {
        Self {
            cpu_units: 1,
            memory_bytes: 64 * 1024 * 1024,
            parallel_workers: 1,
            max_queued_jobs: 1024,
        }
    }

    pub fn validate(self) -> SchedulerResult<()> {
        if self.cpu_units == 0 {
            return Err(SchedulerError::InvalidBudget(
                "cpu_units must be greater than zero".into(),
            ));
        }

        if self.memory_bytes == 0 {
            return Err(SchedulerError::InvalidBudget(
                "memory_bytes must be greater than zero".into(),
            ));
        }

        if self.parallel_workers == 0 {
            return Err(SchedulerError::InvalidBudget(
                "parallel_workers must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

/// Job execution state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    Rejected,
    TimedOut,
}

/// Immutable scheduling metadata.
#[derive(Clone, Debug)]
pub struct JobSpec {
    pub kind: JobKind,
    pub priority: Priority,
    pub deadline: Deadline,
    pub quota: ResourceQuota,
    pub mode: ExecutionMode,
    pub deterministic: bool,
    pub checkpointable: bool,
    pub capability: CapabilityRequirement,
}

/// Capabilities required by a job.
///
/// The actual capability authorization should be performed by
/// `capabilities.rs`; this type only describes the scheduler's requirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityRequirement {
    pub decode: bool,
    pub simulate: bool,
    pub benchmark: bool,
    pub inspect_topology: bool,
    pub accelerator: bool,
    pub distributed_execution: bool,
}

impl CapabilityRequirement {
    pub const fn none() -> Self {
        Self {
            decode: false,
            simulate: false,
            benchmark: false,
            inspect_topology: false,
            accelerator: false,
            distributed_execution: false,
        }
    }

    pub fn for_mode(mode: ExecutionMode) -> Self {
        Self {
            decode: false,
            simulate: false,
            benchmark: false,
            inspect_topology: false,
            accelerator: mode.requires_accelerator(),
            distributed_execution: mode.requires_remote_capability(),
        }
    }
}

/// A backend-independent executable QEC workload.
///
/// The scheduler owns admission and lifecycle management. The closure is
/// supplied by the backend/decoder integration layer.
pub trait JobExecutor: Send + Sync + 'static {
    fn execute(&self, context: ExecutionContext) -> SchedulerResult<JobOutput>;
}

/// Execution context provided to an admitted job.
#[derive(Clone)]
pub struct ExecutionContext {
    pub job_id: JobId,
    pub worker_id: WorkerId,
    pub mode: ExecutionMode,
    pub quota: ResourceQuota,
    pub cancellation: CancellationHandle,
    pub deadline: Deadline,
    pub deterministic: bool,
}

impl ExecutionContext {
    pub fn check_cancelled(&self) -> SchedulerResult<()> {
        if self.cancellation.is_cancelled() {
            return Err(SchedulerError::CancellationRequested);
        }

        if self.deadline.expired() {
            return Err(SchedulerError::DeadlineExceeded);
        }

        Ok(())
    }
}

/// Generic job output.
///
/// Concrete QEC modules should attach their domain-specific result outside
/// this scheduler abstraction.
#[derive(Clone, Debug)]
pub struct JobOutput {
    pub success: bool,
    pub logical_failure: bool,
    pub correction_count: u64,
    pub detection_event_count: u64,
    pub decoder_iterations: u64,
}

/// Scheduler job handle.
#[derive(Clone)]
pub struct JobHandle {
    id: JobId,
    cancellation: CancellationHandle,
    state: Arc<Mutex<JobState>>,
}

impl JobHandle {
    pub fn id(&self) -> JobId {
        self.id
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();

        if let Ok(mut state) = self.state.lock() {
            if matches!(*state, JobState::Queued | JobState::Running) {
                *state = JobState::Cancelled;
            }
        }
    }

    pub fn state(&self) -> JobState {
        self.state
            .lock()
            .map(|state| *state)
            .unwrap_or(JobState::Failed)
    }

    pub fn cancellation(&self) -> CancellationHandle {
        self.cancellation.clone()
    }
}

/// Cancellation primitive.
///
/// This is intentionally compatible with a future dedicated
/// `cancellation.rs` implementation.
#[derive(Clone, Default)]
pub struct CancellationHandle {
    cancelled: Arc<AtomicBool>,
}

impl CancellationHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, AtomicOrdering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(AtomicOrdering::Acquire)
    }
}

/// Internal queue item.
///
/// `BinaryHeap` is a max-heap, so higher priority and earlier deadlines
/// receive greater ordering priority.
struct QueueItem {
    job_id: JobId,
    spec: JobSpec,
    executor: Arc<dyn JobExecutor>,
    cancellation: CancellationHandle,
    state: Arc<Mutex<JobState>>,
    sequence: u64,
}

impl fmt::Debug for QueueItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QueueItem")
            .field("job_id", &self.job_id)
            .field("priority", &self.spec.priority)
            .field("kind", &self.spec.kind)
            .field("mode", &self.spec.mode)
            .field("sequence", &self.sequence)
            .finish()
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.job_id == other.job_id
    }
}

impl Eq for QueueItem {}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let priority = self
            .spec
            .priority
            .rank()
            .cmp(&other.spec.priority.rank());

        if priority != Ordering::Equal {
            return priority;
        }

        let deadline = match (self.spec.deadline.0, other.spec.deadline.0) {
            (Some(a), Some(b)) => b.cmp(&a),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        };

        if deadline != Ordering::Equal {
            return deadline;
        }

        // Smaller sequence number means older admission.
        other.sequence.cmp(&self.sequence)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Scheduler error model.
///
/// These variants map naturally onto the unified QEC error model proposed
/// for `errors.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    InvalidQuota(String),
    InvalidBudget(String),
    QueueFull,
    ResourceLimitExceeded(String),
    MemoryLimitExceeded {
        requested: u64,
        available: u64,
    },
    CpuLimitExceeded {
        requested: u32,
        available: u32,
    },
    ParallelismLimitExceeded {
        requested: u32,
        available: u32,
    },
    CapabilityDenied(String),
    UnsupportedExecutionMode(ExecutionMode),
    DeadlineExceeded,
    CancellationRequested,
    SchedulerShuttingDown,
    WorkerUnavailable,
    ExecutorFailed(String),
    InternalInvariantViolation(String),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuota(message) => write!(formatter, "invalid resource quota: {message}"),
            Self::InvalidBudget(message) => write!(formatter, "invalid scheduler budget: {message}"),
            Self::QueueFull => write!(formatter, "scheduler queue is full"),
            Self::ResourceLimitExceeded(message) => {
                write!(formatter, "resource limit exceeded: {message}")
            }
            Self::MemoryLimitExceeded {
                requested,
                available,
            } => write!(
                formatter,
                "memory quota exceeded: requested={requested}, available={available}"
            ),
            Self::CpuLimitExceeded {
                requested,
                available,
            } => write!(
                formatter,
                "CPU quota exceeded: requested={requested}, available={available}"
            ),
            Self::ParallelismLimitExceeded {
                requested,
                available,
            } => write!(
                formatter,
                "parallelism quota exceeded: requested={requested}, available={available}"
            ),
            Self::CapabilityDenied(capability) => {
                write!(formatter, "required capability denied: {capability}")
            }
            Self::UnsupportedExecutionMode(mode) => {
                write!(formatter, "unsupported execution mode: {mode:?}")
            }
            Self::DeadlineExceeded => write!(formatter, "job deadline exceeded"),
            Self::CancellationRequested => write!(formatter, "job cancellation requested"),
            Self::SchedulerShuttingDown => write!(formatter, "scheduler is shutting down"),
            Self::WorkerUnavailable => write!(formatter, "no worker is currently available"),
            Self::ExecutorFailed(message) => write!(formatter, "job executor failed: {message}"),
            Self::InternalInvariantViolation(message) => {
                write!(formatter, "scheduler invariant violation: {message}")
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Scheduler metrics.
///
/// This intentionally contains counters rather than a particular telemetry
/// implementation. `metrics.rs` / `telemetry.rs` can consume these values.
#[derive(Debug, Default)]
pub struct SchedulerMetrics {
    submitted: AtomicU64,
    admitted: AtomicU64,
    completed: AtomicU64,
    failed: AtomicU64,
    cancelled: AtomicU64,
    timed_out: AtomicU64,
    rejected: AtomicU64,
    logical_failures: AtomicU64,
    total_corrections: AtomicU64,
    total_detection_events: AtomicU64,
    total_decoder_iterations: AtomicU64,
}

impl SchedulerMetrics {
    pub fn snapshot(&self) -> SchedulerMetricsSnapshot {
        SchedulerMetricsSnapshot {
            submitted: self.submitted.load(AtomicOrdering::Relaxed),
            admitted: self.admitted.load(AtomicOrdering::Relaxed),
            completed: self.completed.load(AtomicOrdering::Relaxed),
            failed: self.failed.load(AtomicOrdering::Relaxed),
            cancelled: self.cancelled.load(AtomicOrdering::Relaxed),
            timed_out: self.timed_out.load(AtomicOrdering::Relaxed),
            rejected: self.rejected.load(AtomicOrdering::Relaxed),
            logical_failures: self.logical_failures.load(AtomicOrdering::Relaxed),
            total_corrections: self.total_corrections.load(AtomicOrdering::Relaxed),
            total_detection_events: self
                .total_detection_events
                .load(AtomicOrdering::Relaxed),
            total_decoder_iterations: self
                .total_decoder_iterations
                .load(AtomicOrdering::Relaxed),
        }
    }
}

/// Immutable metrics snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SchedulerMetricsSnapshot {
    pub submitted: u64,
    pub admitted: u64,
    pub completed: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub timed_out: u64,
    pub rejected: u64,
    pub logical_failures: u64,
    pub total_corrections: u64,
    pub total_detection_events: u64,
    pub total_decoder_iterations: u64,
}

/// Current resource usage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResourceUsage {
    pub cpu_units: u32,
    pub memory_bytes: u64,
    pub parallel_workers: u32,
    pub queued_jobs: usize,
}

/// Worker descriptor.
#[derive(Clone, Debug)]
pub struct Worker {
    pub id: WorkerId,
    pub mode: ExecutionMode,
    pub available: bool,
    pub cpu_units: u32,
    pub memory_bytes: u64,
    pub parallel_workers: u32,
}

impl Worker {
    fn can_accept(&self, quota: ResourceQuota) -> bool {
        self.available
            && quota.cpu_units <= self.cpu_units
            && quota.memory_bytes <= self.memory_bytes
            && quota.parallel_workers <= self.parallel_workers
    }
}

/// Scheduler configuration.
///
/// A future `configuration.rs` implementation can construct this object
/// from the global `QecConfig`.
#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    pub budget: SchedulerBudget,
    pub mode: ExecutionMode,
    pub deterministic: bool,
    pub allow_accelerated: bool,
    pub allow_distributed: bool,
    pub fail_fast: bool,
}

impl SchedulerConfig {
    pub fn validate(&self) -> SchedulerResult<()> {
        self.budget.validate()?;

        if self.mode == ExecutionMode::Accelerated && !self.allow_accelerated {
            return Err(SchedulerError::CapabilityDenied(
                "accelerated execution".into(),
            ));
        }

        if self.mode == ExecutionMode::Distributed && !self.allow_distributed {
            return Err(SchedulerError::CapabilityDenied(
                "distributed execution".into(),
            ));
        }

        Ok(())
    }
}

/// Internal scheduler state.
struct SchedulerState {
    queue: BinaryHeap<QueueItem>,
    workers: HashMap<WorkerId, Worker>,
    usage: ResourceUsage,
    shutting_down: bool,
}

/// Production-grade QEC scheduler.
///
/// The scheduler is deliberately synchronous at this abstraction boundary:
/// `submit()` performs admission and queues the job; `dispatch_one()`
/// performs one scheduling decision. A runtime/thread-pool layer can call
/// `dispatch_one()` from workers, while a distributed runtime can use the
/// same policy engine remotely.
pub struct QecScheduler {
    config: SchedulerConfig,
    state: Mutex<SchedulerState>,
    metrics: Arc<SchedulerMetrics>,
    next_job_id: AtomicU64,
    next_sequence: AtomicU64,
    next_worker_id: AtomicU64,
}

impl QecScheduler {
    pub fn new(config: SchedulerConfig) -> SchedulerResult<Self> {
        config.validate()?;

        let mut workers = HashMap::new();

        let worker_id = WorkerId::new(0);
        workers.insert(
            worker_id,
            Worker {
                id: worker_id,
                mode: config.mode,
                available: true,
                cpu_units: config.budget.cpu_units,
                memory_bytes: config.budget.memory_bytes,
                parallel_workers: config.budget.parallel_workers,
            },
        );

        Ok(Self {
            config,
            state: Mutex::new(SchedulerState {
                queue: BinaryHeap::new(),
                workers,
                usage: ResourceUsage::default(),
                shutting_down: false,
            }),
            metrics: Arc::new(SchedulerMetrics::default()),
            next_job_id: AtomicU64::new(1),
            next_sequence: AtomicU64::new(1),
            next_worker_id: AtomicU64::new(1),
        })
    }

    /// Registers an execution worker.
    ///
    /// Distributed/accelerated runtimes can register workers with their
    /// capabilities before dispatch.
    pub fn register_worker(
        &self,
        mode: ExecutionMode,
        cpu_units: u32,
        memory_bytes: u64,
        parallel_workers: u32,
    ) -> SchedulerResult<WorkerId> {
        if cpu_units == 0 || memory_bytes == 0 || parallel_workers == 0 {
            return Err(SchedulerError::InvalidBudget(
                "worker resources must be non-zero".into(),
            ));
        }

        if mode == ExecutionMode::Accelerated && !self.config.allow_accelerated {
            return Err(SchedulerError::CapabilityDenied(
                "accelerated execution".into(),
            ));
        }

        if mode == ExecutionMode::Distributed && !self.config.allow_distributed {
            return Err(SchedulerError::CapabilityDenied(
                "distributed execution".into(),
            ));
        }

        let id = WorkerId::new(
            self.next_worker_id
                .fetch_add(1, AtomicOrdering::Relaxed),
        );

        let mut state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        state.workers.insert(
            id,
            Worker {
                id,
                mode,
                available: true,
                cpu_units,
                memory_bytes,
                parallel_workers,
            },
        );

        Ok(id)
    }

    /// Submit a QEC job with bounded admission.
    pub fn submit(
        &self,
        spec: JobSpec,
        executor: Arc<dyn JobExecutor>,
    ) -> SchedulerResult<JobHandle> {
        spec.quota.validate()?;

        if spec.mode == ExecutionMode::Accelerated && !self.config.allow_accelerated {
            return Err(SchedulerError::CapabilityDenied(
                "accelerated execution".into(),
            ));
        }

        if spec.mode == ExecutionMode::Distributed && !self.config.allow_distributed {
            return Err(SchedulerError::CapabilityDenied(
                "distributed execution".into(),
            ));
        }

        self.metrics
            .submitted
            .fetch_add(1, AtomicOrdering::Relaxed);

        if spec.deadline.expired() {
            self.metrics
                .rejected
                .fetch_add(1, AtomicOrdering::Relaxed);

            return Err(SchedulerError::DeadlineExceeded);
        }

        let job_id = JobId::new(
            self.next_job_id
                .fetch_add(1, AtomicOrdering::Relaxed),
        );

        let cancellation = CancellationHandle::new();

        let handle = JobHandle {
            id: job_id,
            cancellation: cancellation.clone(),
            state: Arc::new(Mutex::new(JobState::Queued)),
        };

        let state_handle = handle.state.clone();

        let sequence = self
            .next_sequence
            .fetch_add(1, AtomicOrdering::Relaxed);

        let mut state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        if state.shutting_down {
            self.metrics
                .rejected
                .fetch_add(1, AtomicOrdering::Relaxed);

            return Err(SchedulerError::SchedulerShuttingDown);
        }

        if state.queue.len() >= self.config.budget.max_queued_jobs {
            self.metrics
                .rejected
                .fetch_add(1, AtomicOrdering::Relaxed);

            return Err(SchedulerError::QueueFull);
        }

        // Admission control prevents aggregate overcommitment.
        self.check_budget(&state.usage, spec.quota)?;

        state.usage.cpu_units = state
            .usage
            .cpu_units
            .checked_add(spec.quota.cpu_units)
            .ok_or_else(|| {
                SchedulerError::ResourceLimitExceeded("CPU accounting overflow".into())
            })?;

        state.usage.memory_bytes = state
            .usage
            .memory_bytes
            .checked_add(spec.quota.memory_bytes)
            .ok_or_else(|| {
                SchedulerError::ResourceLimitExceeded("memory accounting overflow".into())
            })?;

        state.usage.parallel_workers = state
            .usage
            .parallel_workers
            .checked_add(spec.quota.parallel_workers)
            .ok_or_else(|| {
                SchedulerError::ResourceLimitExceeded("parallelism accounting overflow".into())
            })?;

        state.usage.queued_jobs = state
            .usage
            .queued_jobs
            .checked_add(1)
            .ok_or_else(|| {
                SchedulerError::ResourceLimitExceeded("queue accounting overflow".into())
            })?;

        state.queue.push(QueueItem {
            job_id,
            spec,
            executor,
            cancellation,
            state: state_handle,
            sequence,
        });

        self.metrics
            .admitted
            .fetch_add(1, AtomicOrdering::Relaxed);

        Ok(handle)
    }

    fn check_budget(
        &self,
        usage: &ResourceUsage,
        quota: ResourceQuota,
    ) -> SchedulerResult<()> {
        let cpu = usage.cpu_units.checked_add(quota.cpu_units).ok_or(
            SchedulerError::CpuLimitExceeded {
                requested: quota.cpu_units,
                available: 0,
            },
        )?;

        if cpu > self.config.budget.cpu_units {
            return Err(SchedulerError::CpuLimitExceeded {
                requested: quota.cpu_units,
                available: self
                    .config
                    .budget
                    .cpu_units
                    .saturating_sub(usage.cpu_units),
            });
        }

        let memory = usage
            .memory_bytes
            .checked_add(quota.memory_bytes)
            .ok_or(SchedulerError::MemoryLimitExceeded {
                requested: quota.memory_bytes,
                available: 0,
            })?;

        if memory > self.config.budget.memory_bytes {
            return Err(SchedulerError::MemoryLimitExceeded {
                requested: quota.memory_bytes,
                available: self
                    .config
                    .budget
                    .memory_bytes
                    .saturating_sub(usage.memory_bytes),
            });
        }

        let workers = usage
            .parallel_workers
            .checked_add(quota.parallel_workers)
            .ok_or(SchedulerError::ParallelismLimitExceeded {
                requested: quota.parallel_workers,
                available: 0,
            })?;

        if workers > self.config.budget.parallel_workers {
            return Err(SchedulerError::ParallelismLimitExceeded {
                requested: quota.parallel_workers,
                available: self
                    .config
                    .budget
                    .parallel_workers
                    .saturating_sub(usage.parallel_workers),
            });
        }

        Ok(())
    }

    /// Select and execute one eligible job.
    ///
    /// This is the scheduling boundary. An outer worker pool may repeatedly
    /// call this method. A future async/runtime implementation can replace
    /// this execution mechanism without changing the admission policy.
    pub fn dispatch_one(&self) -> SchedulerResult<Option<JobOutput>> {
        let item = {
            let mut state = self.state.lock().map_err(|_| {
                SchedulerError::InternalInvariantViolation("state poisoned".into())
            })?;

            if state.shutting_down {
                return Err(SchedulerError::SchedulerShuttingDown);
            }

            let item = self.select_next_job(&mut state)?;

            let Some(item) = item else {
                return Ok(None);
            };

            if item.cancellation.is_cancelled() {
                Self::release_usage(&mut state, item.spec.quota)?;
                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = JobState::Cancelled;
                }

                self.metrics
                    .cancelled
                    .fetch_add(1, AtomicOrdering::Relaxed);

                return Ok(None);
            }

            if item.spec.deadline.expired() {
                Self::release_usage(&mut state, item.spec.quota)?;
                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = JobState::TimedOut;
                }

                self.metrics
                    .timed_out
                    .fetch_add(1, AtomicOrdering::Relaxed);

                return Ok(None);
            }

            state.usage.queued_jobs = state.usage.queued_jobs.saturating_sub(1);

            if let Ok(mut job_state) = item.state.lock() {
                *job_state = JobState::Running;
            }

            item
        };

        let worker_id = self.find_worker(item.spec.mode, item.spec.quota)?;

        let context = ExecutionContext {
            job_id: item.job_id,
            worker_id,
            mode: item.spec.mode,
            quota: item.spec.quota,
            cancellation: item.cancellation.clone(),
            deadline: item.spec.deadline,
            deterministic: item.spec.deterministic || self.config.deterministic,
        };

        let started = Instant::now();

        let result = if context.cancellation.is_cancelled() {
            Err(SchedulerError::CancellationRequested)
        } else if context.deadline.expired() {
            Err(SchedulerError::DeadlineExceeded)
        } else {
            item.executor.execute(context)
        };

        let elapsed = started.elapsed();

        self.finalize_job(&item, result.as_ref(), elapsed)?;

        result.map(Some)
    }

    fn select_next_job(
        &self,
        state: &mut SchedulerState,
    ) -> SchedulerResult<Option<QueueItem>> {
        let mut deferred = Vec::new();
        let mut selected = None;

        while let Some(item) = state.queue.pop() {
            if item.cancellation.is_cancelled() {
                Self::release_usage(state, item.spec.quota)?;

                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = JobState::Cancelled;
                }

                self.metrics
                    .cancelled
                    .fetch_add(1, AtomicOrdering::Relaxed);

                continue;
            }

            if item.spec.deadline.expired() {
                Self::release_usage(state, item.spec.quota)?;

                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = JobState::TimedOut;
                }

                self.metrics
                    .timed_out
                    .fetch_add(1, AtomicOrdering::Relaxed);

                continue;
            }

            if self.find_worker_in_state(state, item.spec.mode, item.spec.quota)
                .is_some()
            {
                selected = Some(item);
                break;
            }

            deferred.push(item);
        }

        for item in deferred {
            state.queue.push(item);
        }

        Ok(selected)
    }

    fn find_worker(
        &self,
        mode: ExecutionMode,
        quota: ResourceQuota,
    ) -> SchedulerResult<WorkerId> {
        let state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        self.find_worker_in_state(&state, mode, quota)
            .ok_or(SchedulerError::WorkerUnavailable)
    }

    fn find_worker_in_state(
        &self,
        state: &SchedulerState,
        mode: ExecutionMode,
        quota: ResourceQuota,
    ) -> Option<WorkerId> {
        state
            .workers
            .values()
            .filter(|worker| worker.mode == mode && worker.can_accept(quota))
            .map(|worker| worker.id)
            .min_by_key(|id| id.get())
    }

    fn finalize_job(
        &self,
        item: &QueueItem,
        result: Result<&JobOutput, &SchedulerError>,
        _elapsed: Duration,
    ) -> SchedulerResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        Self::release_usage(&mut state, item.spec.quota)?;

        match result {
            Ok(output) => {
                if item.cancellation.is_cancelled() {
                    if let Ok(mut job_state) = item.state.lock() {
                        *job_state = JobState::Cancelled;
                    }

                    self.metrics
                        .cancelled
                        .fetch_add(1, AtomicOrdering::Relaxed);
                    return Ok(());
                }

                if item.spec.deadline.expired() {
                    if let Ok(mut job_state) = item.state.lock() {
                        *job_state = JobState::TimedOut;
                    }

                    self.metrics
                        .timed_out
                        .fetch_add(1, AtomicOrdering::Relaxed);
                    return Ok(());
                }

                if output.success {
                    if let Ok(mut job_state) = item.state.lock() {
                        *job_state = JobState::Completed;
                    }

                    self.metrics
                        .completed
                        .fetch_add(1, AtomicOrdering::Relaxed);
                } else {
                    if let Ok(mut job_state) = item.state.lock() {
                        *job_state = JobState::Failed;
                    }

                    self.metrics.failed.fetch_add(1, AtomicOrdering::Relaxed);
                }

                if output.logical_failure {
                    self.metrics
                        .logical_failures
                        .fetch_add(1, AtomicOrdering::Relaxed);
                }

                self.metrics.total_corrections.fetch_add(
                    output.correction_count,
                    AtomicOrdering::Relaxed,
                );

                self.metrics.total_detection_events.fetch_add(
                    output.detection_event_count,
                    AtomicOrdering::Relaxed,
                );

                self.metrics.total_decoder_iterations.fetch_add(
                    output.decoder_iterations,
                    AtomicOrdering::Relaxed,
                );
            }

            Err(SchedulerError::CancellationRequested)
            | Err(SchedulerError::DeadlineExceeded) => {
                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = if matches!(
                        result,
                        Err(SchedulerError::CancellationRequested)
                    ) {
                        JobState::Cancelled
                    } else {
                        JobState::TimedOut
                    };
                }

                if matches!(
                    result,
                    Err(SchedulerError::CancellationRequested)
                ) {
                    self.metrics
                        .cancelled
                        .fetch_add(1, AtomicOrdering::Relaxed);
                } else {
                    self.metrics
                        .timed_out
                        .fetch_add(1, AtomicOrdering::Relaxed);
                }
            }

            Err(_) => {
                if let Ok(mut job_state) = item.state.lock() {
                    *job_state = JobState::Failed;
                }

                self.metrics.failed.fetch_add(1, AtomicOrdering::Relaxed);
            }
        }

        Ok(())
    }

    fn release_usage(
        state: &mut SchedulerState,
        quota: ResourceQuota,
    ) -> SchedulerResult<()> {
        state.usage.cpu_units = state
            .usage
            .cpu_units
            .checked_sub(quota.cpu_units)
            .ok_or_else(|| {
                SchedulerError::InternalInvariantViolation(
                    "CPU usage underflow during resource release".into(),
                )
            })?;

        state.usage.memory_bytes = state
            .usage
            .memory_bytes
            .checked_sub(quota.memory_bytes)
            .ok_or_else(|| {
                SchedulerError::InternalInvariantViolation(
                    "memory usage underflow during resource release".into(),
                )
            })?;

        state.usage.parallel_workers = state
            .usage
            .parallel_workers
            .checked_sub(quota.parallel_workers)
            .ok_or_else(|| {
                SchedulerError::InternalInvariantViolation(
                    "parallelism usage underflow during resource release".into(),
                )
            })?;

        Ok(())
    }

    /// Returns current aggregate resource usage.
    pub fn resource_usage(&self) -> SchedulerResult<ResourceUsage> {
        let state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        Ok(state.usage)
    }

    /// Returns immutable scheduler metrics.
    pub fn metrics(&self) -> SchedulerMetricsSnapshot {
        self.metrics.snapshot()
    }

    /// Returns the number of queued jobs.
    pub fn queue_len(&self) -> SchedulerResult<usize> {
        let state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        Ok(state.queue.len())
    }

    /// Initiates graceful scheduler shutdown.
    ///
    /// Queued jobs are not silently executed after shutdown begins.
    pub fn shutdown(&self) -> SchedulerResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        state.shutting_down = true;

        while let Some(item) = state.queue.pop() {
            item.cancellation.cancel();

            if let Ok(mut job_state) = item.state.lock() {
                *job_state = JobState::Cancelled;
            }

            Self::release_usage(&mut state, item.spec.quota)?;

            self.metrics
                .cancelled
                .fetch_add(1, AtomicOrdering::Relaxed);
        }

        state.usage.queued_jobs = 0;

        Ok(())
    }

    /// Enables worker availability changes without destroying scheduler state.
    pub fn set_worker_available(
        &self,
        worker_id: WorkerId,
        available: bool,
    ) -> SchedulerResult<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| SchedulerError::InternalInvariantViolation("state poisoned".into()))?;

        let worker = state
            .workers
            .get_mut(&worker_id)
            .ok_or(SchedulerError::WorkerUnavailable)?;

        worker.available = available;

        Ok(())
    }
}

/// A small deterministic test executor.
#[cfg(test)]
struct TestExecutor {
    output: JobOutput,
}

#[cfg(test)]
impl JobExecutor for TestExecutor {
    fn execute(&self, context: ExecutionContext) -> SchedulerResult<JobOutput> {
        context.check_cancelled()?;
        Ok(self.output.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheduler() -> QecScheduler {
        QecScheduler::new(SchedulerConfig {
            budget: SchedulerBudget {
                cpu_units: 8,
                memory_bytes: 1024 * 1024,
                parallel_workers: 8,
                max_queued_jobs: 16,
            },
            mode: ExecutionMode::SingleThread,
            deterministic: true,
            allow_accelerated: false,
            allow_distributed: false,
            fail_fast: true,
        })
        .expect("valid scheduler")
    }

    fn spec(priority: Priority) -> JobSpec {
        JobSpec {
            kind: JobKind::Decode,
            priority,
            deadline: Deadline::none(),
            quota: ResourceQuota {
                cpu_units: 1,
                memory_bytes: 1024,
                parallel_workers: 1,
                decoder_iterations: 100,
            },
            mode: ExecutionMode::SingleThread,
            deterministic: true,
            checkpointable: true,
            capability: CapabilityRequirement {
                decode: true,
                ..CapabilityRequirement::none()
            },
        }
    }

    fn executor() -> Arc<dyn JobExecutor> {
        Arc::new(TestExecutor {
            output: JobOutput {
                success: true,
                logical_failure: false,
                correction_count: 2,
                detection_event_count: 4,
                decoder_iterations: 10,
            },
        })
    }

    #[test]
    fn accepts_valid_job() {
        let scheduler = scheduler();

        let handle = scheduler
            .submit(spec(Priority::Normal), executor())
            .expect("submission should succeed");

        assert_eq!(handle.state(), JobState::Queued);
        assert_eq!(scheduler.queue_len().unwrap(), 1);
    }

    #[test]
    fn enforces_memory_budget() {
        let scheduler = scheduler();

        let mut job = spec(Priority::Normal);
        job.quota.memory_bytes = 2 * 1024 * 1024;

        let result = scheduler.submit(job, executor());

        assert!(matches!(
            result,
            Err(SchedulerError::MemoryLimitExceeded { .. })
        ));
    }

    #[test]
    fn dispatches_job() {
        let scheduler = scheduler();

        scheduler
            .submit(spec(Priority::Normal), executor())
            .expect("submission should succeed");

        let output = scheduler
            .dispatch_one()
            .expect("dispatch should succeed")
            .expect("job should execute");

        assert!(output.success);

        let metrics = scheduler.metrics();
        assert_eq!(metrics.submitted, 1);
        assert_eq!(metrics.admitted, 1);
        assert_eq!(metrics.completed, 1);
        assert_eq!(metrics.total_corrections, 2);
        assert_eq!(metrics.total_detection_events, 4);
    }

    #[test]
    fn higher_priority_job_wins() {
        let scheduler = scheduler();

        let low = scheduler
            .submit(spec(Priority::Low), executor())
            .expect("submission should succeed");

        let high = scheduler
            .submit(spec(Priority::Critical), executor())
            .expect("submission should succeed");

        let _ = scheduler.dispatch_one().expect("dispatch");

        assert_eq!(high.state(), JobState::Completed);
        assert_eq!(low.state(), JobState::Queued);
    }

    #[test]
    fn cancellation_is_observed() {
        let scheduler = scheduler();

        let handle = scheduler
            .submit(spec(Priority::Normal), executor())
            .expect("submission should succeed");

        handle.cancel();

        let result = scheduler.dispatch_one().expect("dispatch");

        assert!(result.is_none());
        assert_eq!(handle.state(), JobState::Cancelled);
    }

    #[test]
    fn expired_deadline_is_rejected() {
        let scheduler = scheduler();

        let mut job = spec(Priority::Normal);
        job.deadline = Deadline(Some(Instant::now() - Duration::from_secs(1)));

        let result = scheduler.submit(job, executor());

        assert_eq!(result, Err(SchedulerError::DeadlineExceeded));
    }

    #[test]
    fn queue_backpressure_is_enforced() {
        let scheduler = QecScheduler::new(SchedulerConfig {
            budget: SchedulerBudget {
                cpu_units: 8,
                memory_bytes: 1024 * 1024,
                parallel_workers: 8,
                max_queued_jobs: 1,
            },
            mode: ExecutionMode::SingleThread,
            deterministic: true,
            allow_accelerated: false,
            allow_distributed: false,
            fail_fast: true,
        })
        .unwrap();

        scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        let result = scheduler.submit(spec(Priority::Normal), executor());

        assert_eq!(result, Err(SchedulerError::QueueFull));
    }

    #[test]
    fn resource_usage_returns_to_zero_after_execution() {
        let scheduler = scheduler();

        scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        let _ = scheduler.dispatch_one().unwrap();

        assert_eq!(
            scheduler.resource_usage().unwrap(),
            ResourceUsage::default()
        );
    }

    #[test]
    fn worker_registration_is_supported() {
        let scheduler = scheduler();

        let worker = scheduler
            .register_worker(ExecutionMode::SingleThread, 2, 4096, 2)
            .unwrap();

        assert_eq!(worker.get(), 1);
    }

    #[test]
    fn deterministic_queue_order_is_stable() {
        let scheduler = scheduler();

        let first = scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        let second = scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        let _ = scheduler.dispatch_one().unwrap();

        assert_eq!(first.state(), JobState::Completed);
        assert_eq!(second.state(), JobState::Queued);
    }

    #[test]
    fn graceful_shutdown_cancels_queued_jobs() {
        let scheduler = scheduler();

        let first = scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        let second = scheduler
            .submit(spec(Priority::Normal), executor())
            .unwrap();

        scheduler.shutdown().unwrap();

        assert_eq!(first.state(), JobState::Cancelled);
        assert_eq!(second.state(), JobState::Cancelled);
        assert_eq!(scheduler.queue_len().unwrap(), 0);
    }
}