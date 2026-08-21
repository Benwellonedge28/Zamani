//! Zamani Quantum Error-Correction Scheduler.
//!
//! Production admission, lifecycle, deterministic ordering, cancellation,
//! deadline, worker-lease, retry, and resource-reservation orchestration.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - job admission;
//! - scheduler lifecycle state;
//! - deterministic priority ordering;
//! - bounded queueing;
//! - worker registration;
//! - worker leasing;
//! - retry policy;
//! - deadline enforcement;
//! - cancellation propagation;
//! - checkpoint lifecycle transitions;
//! - scheduler-level resource reservation;
//! - execution dispatch.
//!
//! This module does NOT own:
//!
//! - QEC mathematics;
//! - decoder algorithms;
//! - decoding graphs;
//! - syndrome extraction;
//! - surface-code topology;
//! - QPU I/O;
//! - distributed network transport;
//! - checkpoint serialization;
//! - cache implementation;
//! - canonical resource policy;
//! - capability authority;
//! - runtime resource accounting.
//!
//! # Integration architecture
//!
//! ```text
//!                    QecConfig / QecLimits
//!                             │
//!                             ▼
//!                       configuration
//!                             │
//!                             ▼
//!                         Scheduler
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          │                  │                  │
//!          ▼                  ▼                  ▼
//!   capabilities.rs    resources.rs      cancellation.rs
//!   authorization      accounting        cancellation
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             ▼
//!                         Admission
//!                             │
//!                             ▼
//!                   Deterministic Queue
//!                             │
//!                             ▼
//!                       Worker Lease
//!                             │
//!                             ▼
//!                         Executor
//!                       /    |     \
//!                      /     |      \
//!                success   cancel   failure
//!                   │        │        │
//!                   ▼        ▼        ▼
//!               Completed Cancelled Retry/Failed
//! ```
//!
//! The scheduler is executor-agnostic. A decoder, simulator, streaming
//! worker, partition worker, distributed coordinator, or QPU orchestration
//! layer supplies a `JobExecutor`.
//!
//! # Resource ownership
//!
//! `limits.rs` remains the single source of truth for declarative limits.
//!
//! `resources.rs` remains the runtime accounting authority.
//!
//! `memory.rs` remains the memory allocation authority.
//!
//! This scheduler only maintains the reservation required to make admission
//! atomic and deterministic. Production runtime accounting is connected by
//! implementing `ResourceAccounting`.
//!
//! # Capability ownership
//!
//! `CapabilityRequirement` describes what a job needs.
//!
//! It does NOT grant authority.
//!
//! Actual authorization is supplied through `CapabilityAuthorizer`, whose
//! implementation belongs to `capabilities.rs`.
//!
//! # Cancellation ownership
//!
//! Cancellation is delegated completely to `cancellation.rs`.
//!
//! The scheduler owns a `CancellationSource` per admitted job and passes only
//! the corresponding `CancellationToken` to executors.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! The implementation uses only stable standard-library APIs.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::cancellation::{
    CancellationReason,
    CancellationSource,
    CancellationToken,
};
use super::configuration::QecConfig;
use super::errors::{
    NumericalOperation,
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;

/// Canonical scheduler result type.
///
/// Scheduler errors are always represented by the QEC-wide `QecError`.
pub type SchedulerResult<T> = QecResult<T>;

/// Maximum retry count accepted by the scheduler.
///
/// This is an API-safety bound, not a QEC resource policy.
pub const MAX_RETRY_COUNT: u32 = 1_000_000;

/// Maximum queue depth accepted by one scheduler instance.
///
/// This is an admission-safety bound. Actual workload/resource ceilings
/// remain controlled by `QecLimits`.
pub const MAX_QUEUE_DEPTH: usize = 1_000_000;

// ============================================================================
// Identifiers
// ============================================================================

/// Scheduler-local job identifier.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct JobId(u64);

impl JobId {
    /// Creates an identifier from a raw value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Scheduler worker identifier.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct WorkerId(u64);

impl WorkerId {
    /// Creates an identifier from a raw value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

// ============================================================================
// Scheduling primitives
// ============================================================================

/// QEC workload category.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JobKind {
    LogicalOperation,
    Decode,
    Simulation,
    ThresholdBenchmark,
    Diagnostic,
    Streaming,
    Partition,
    Distributed,
    Qpu,
}

/// Scheduling priority.
///
/// Higher priority wins. Equal priorities are ordered by submission sequence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

impl Priority {
    /// Returns the stable priority rank.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::High => 4,
            Self::Normal => 3,
            Self::Low => 2,
            Self::Background => 1,
        }
    }
}

/// Execution requirement.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExecutionMode {
    SingleThread,
    MultiThread,
    MultiProcess,
    Distributed,
    Accelerated,
    Qpu,
}

impl ExecutionMode {
    /// Whether the execution requires more than one classical worker.
    #[must_use]
    pub const fn requires_parallelism(self) -> bool {
        matches!(
            self,
            Self::MultiThread
                | Self::MultiProcess
                | Self::Distributed
                | Self::Accelerated
        )
    }

    /// Whether distributed execution authority is required.
    #[must_use]
    pub const fn requires_distributed_capability(self) -> bool {
        matches!(
            self,
            Self::MultiProcess | Self::Distributed
        )
    }

    /// Whether accelerator authority is required.
    #[must_use]
    pub const fn requires_accelerator(self) -> bool {
        matches!(self, Self::Accelerated)
    }

    /// Whether QPU authority is required.
    #[must_use]
    pub const fn requires_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }
}

// ============================================================================
// Deadline
// ============================================================================

/// Optional absolute scheduler deadline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Deadline(Option<Instant>);

impl Deadline {
    /// Creates a job without a deadline.
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    /// Creates a deadline relative to the current instant.
    #[must_use]
    pub fn after(duration: Duration) -> Self {
        Self(Instant::now().checked_add(duration))
    }

    /// Returns whether the deadline has expired.
    #[must_use]
    pub fn expired(self) -> bool {
        self.0
            .map_or(false, |deadline| Instant::now() >= deadline)
    }

    /// Returns remaining time.
    #[must_use]
    pub fn remaining(self) -> Option<Duration> {
        self.0.map(|deadline| {
            deadline.saturating_duration_since(Instant::now())
        })
    }

    /// Returns the underlying absolute deadline.
    #[must_use]
    pub const fn instant(self) -> Option<Instant> {
        self.0
    }
}

// ============================================================================
// Resource contract
// ============================================================================

/// Resource reservation requested by one job.
///
/// These values are reservations, not another resource-policy system.
/// Every request is validated against `QecLimits` before admission.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceRequest {
    /// Maximum memory reserved for this workload.
    pub memory_bytes: u64,

    /// Number of classical workers reserved.
    pub parallel_workers: usize,

    /// Decoder iteration reservation.
    pub decoder_iterations: usize,

    /// Number of partitions reserved.
    pub partitions: usize,
}

impl ResourceRequest {
    /// Creates an empty resource request.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            memory_bytes: 0,
            parallel_workers: 0,
            decoder_iterations: 0,
            partitions: 0,
        }
    }

    /// Validates this request against the canonical QEC limits.
    pub fn validate_against(
        &self,
        limits: &QecLimits,
    ) -> SchedulerResult<()> {
        if self.memory_bytes > limits.max_memory_bytes {
            return Err(resource_error(
                ResourceKind::MemoryBytes,
                self.memory_bytes as u128,
                0,
                limits.max_memory_bytes as u128,
                "scheduler memory request exceeds QecLimits",
            ));
        }

        if self.parallel_workers > limits.max_parallelism {
            return Err(resource_error(
                ResourceKind::Parallelism,
                self.parallel_workers as u128,
                0,
                limits.max_parallelism as u128,
                "scheduler worker request exceeds QecLimits",
            ));
        }

        if self.decoder_iterations
            > limits.max_decoder_iterations
        {
            return Err(resource_error(
                ResourceKind::DecoderIterations,
                self.decoder_iterations as u128,
                0,
                limits.max_decoder_iterations as u128,
                "scheduler iteration reservation exceeds QecLimits",
            ));
        }

        if self.partitions > limits.max_partitions {
            return Err(resource_error(
                ResourceKind::Partitions,
                self.partitions as u128,
                0,
                limits.max_partitions as u128,
                "scheduler partition reservation exceeds QecLimits",
            ));
        }

        Ok(())
    }
}

/// Aggregate scheduler reservation.
///
/// This is deliberately distinct from runtime consumption.
///
/// `resources.rs` remains authoritative for actual resource usage.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReservationSnapshot {
    pub memory_bytes: u64,
    pub parallel_workers: usize,
    pub decoder_iterations: usize,
    pub partitions: usize,
}

impl ReservationSnapshot {
    fn try_add(
        self,
        request: ResourceRequest,
        limits: &QecLimits,
    ) -> SchedulerResult<Self> {
        let memory = self
            .memory_bytes
            .checked_add(request.memory_bytes)
            .ok_or_else(|| {
                numerical_error(
                    "scheduler memory reservation overflow",
                )
            })?;

        let workers = self
            .parallel_workers
            .checked_add(request.parallel_workers)
            .ok_or_else(|| {
                numerical_error(
                    "scheduler worker reservation overflow",
                )
            })?;

        let iterations = self
            .decoder_iterations
            .checked_add(request.decoder_iterations)
            .ok_or_else(|| {
                numerical_error(
                    "scheduler iteration reservation overflow",
                )
            })?;

        let partitions = self
            .partitions
            .checked_add(request.partitions)
            .ok_or_else(|| {
                numerical_error(
                    "scheduler partition reservation overflow",
                )
            })?;

        if memory > limits.max_memory_bytes {
            return Err(resource_error(
                ResourceKind::MemoryBytes,
                memory as u128,
                0,
                limits.max_memory_bytes as u128,
                "aggregate scheduler reservation exceeds memory limit",
            ));
        }

        if workers > limits.max_parallelism {
            return Err(resource_error(
                ResourceKind::Parallelism,
                workers as u128,
                0,
                limits.max_parallelism as u128,
                "aggregate scheduler reservation exceeds parallelism limit",
            ));
        }

        if iterations > limits.max_decoder_iterations {
            return Err(resource_error(
                ResourceKind::DecoderIterations,
                iterations as u128,
                0,
                limits.max_decoder_iterations as u128,
                "aggregate scheduler reservation exceeds decoder iteration limit",
            ));
        }

        if partitions > limits.max_partitions {
            return Err(resource_error(
                ResourceKind::Partitions,
                partitions as u128,
                0,
                limits.max_partitions as u128,
                "aggregate scheduler reservation exceeds partition limit",
            ));
        }

        Ok(Self {
            memory_bytes: memory,
            parallel_workers: workers,
            decoder_iterations: iterations,
            partitions,
        })
    }

    fn subtract(self, request: ResourceRequest) -> Self {
        Self {
            memory_bytes: self
                .memory_bytes
                .saturating_sub(request.memory_bytes),

            parallel_workers: self
                .parallel_workers
                .saturating_sub(request.parallel_workers),

            decoder_iterations: self
                .decoder_iterations
                .saturating_sub(request.decoder_iterations),

            partitions: self
                .partitions
                .saturating_sub(request.partitions),
        }
    }
}

/// Runtime-resource accounting adapter.
///
/// `resources.rs` should implement this contract rather than requiring the
/// scheduler to know its internal accounting representation.
///
/// Implementations must make `reserve` atomic and must release exactly once.
pub trait ResourceAccounting: Send + Sync {
    /// Atomically reserves resources for an admitted job.
    fn reserve(
        &self,
        job_id: JobId,
        request: ResourceRequest,
    ) -> SchedulerResult<()>;

    /// Releases a previous reservation exactly once.
    fn release(
        &self,
        job_id: JobId,
        request: ResourceRequest,
    ) -> SchedulerResult<()>;
}

/// No-op resource adapter for isolated tests.
///
/// Production execution should connect this to `resources.rs`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopResourceAccounting;

impl ResourceAccounting for NoopResourceAccounting {
    fn reserve(
        &self,
        _job_id: JobId,
        _request: ResourceRequest,
    ) -> SchedulerResult<()> {
        Ok(())
    }

    fn release(
        &self,
        _job_id: JobId,
        _request: ResourceRequest,
    ) -> SchedulerResult<()> {
        Ok(())
    }
}

// ============================================================================
// Capability contract
// ============================================================================

/// Capability requirements for a scheduled workload.
///
/// This structure describes requirements only. It does not grant authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilityRequirement {
    pub decode: bool,
    pub simulate: bool,
    pub benchmark: bool,
    pub distributed_execution: bool,
    pub accelerator: bool,
    pub streaming: bool,
    pub checkpoint: bool,
    pub deterministic: bool,
    pub qpu_submit: bool,
    pub qpu_read_results: bool,
}

impl CapabilityRequirement {
    /// Creates an empty capability requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            decode: false,
            simulate: false,
            benchmark: false,
            distributed_execution: false,
            accelerator: false,
            streaming: false,
            checkpoint: false,
            deterministic: false,
            qpu_submit: false,
            qpu_read_results: false,
        }
    }

    /// Creates requirements implied by the workload.
    #[must_use]
    pub const fn for_job(
        kind: JobKind,
        mode: ExecutionMode,
        deterministic: bool,
        checkpointable: bool,
    ) -> Self {
        Self {
            decode: matches!(
                kind,
                JobKind::Decode
                    | JobKind::LogicalOperation
                    | JobKind::Streaming
                    | JobKind::Partition
                    | JobKind::Distributed
                    | JobKind::Qpu
            ),

            simulate: matches!(
                kind,
                JobKind::Simulation
                    | JobKind::ThresholdBenchmark
            ),

            benchmark: matches!(
                kind,
                JobKind::ThresholdBenchmark
            ),

            distributed_execution:
                mode.requires_distributed_capability(),

            accelerator: mode.requires_accelerator(),

            streaming: matches!(
                kind,
                JobKind::Streaming
            ),

            checkpoint: checkpointable,

            deterministic,

            qpu_submit: mode.requires_qpu(),

            qpu_read_results: mode.requires_qpu(),
        }
    }
}

/// Capability authorization boundary.
///
/// `capabilities.rs` owns the actual authority implementation.
pub type CapabilityAuthorizer = Arc<
    dyn Fn(CapabilityRequirement) -> SchedulerResult<()>
        + Send
        + Sync,
>;

fn allow_all_capabilities() -> CapabilityAuthorizer {
    Arc::new(|_| Ok(()))
}

// ============================================================================
// Lifecycle
// ============================================================================

/// Complete scheduler lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobState {
    Created,
    Validating,
    Admitted,
    Running,
    Checkpointing,
    Paused,
    Resuming,
    Completed,
    Failed,
    Cancelled,
    Rejected,
    TimedOut,
}

impl JobState {
    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::Rejected
                | Self::TimedOut
        )
    }
}

// ============================================================================
// Job specification
// ============================================================================

/// Immutable scheduling request.
#[derive(Clone, Debug)]
pub struct JobSpec {
    pub kind: JobKind,
    pub priority: Priority,
    pub deadline: Deadline,
    pub resources: ResourceRequest,
    pub mode: ExecutionMode,
    pub deterministic: bool,
    pub checkpointable: bool,
    pub capabilities: CapabilityRequirement,
    pub max_retries: u32,
}

impl JobSpec {
    /// Validates the job specification before admission.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SchedulerResult<()> {
        self.resources.validate_against(limits)?;

        if self.max_retries > MAX_RETRY_COUNT {
            return Err(QecError::InvalidInput {
                message:
                    "max_retries exceeds scheduler safety bound"
                        .into(),
            });
        }

        if self.mode.requires_parallelism()
            && self.resources.parallel_workers == 0
        {
            return Err(QecError::InvalidInput {
                message:
                    "parallel execution requires at least one worker reservation"
                        .into(),
            });
        }

        if matches!(
            self.mode,
            ExecutionMode::SingleThread
        ) && self.resources.parallel_workers > 1
        {
            return Err(QecError::InvalidInput {
                message:
                    "single-thread execution cannot reserve multiple workers"
                        .into(),
            });
        }

        if self.checkpointable
            && !self.capabilities.checkpoint
        {
            return Err(capability_error(
                "checkpoint",
                "checkpointable job",
            ));
        }

        if self.deterministic
            && !self.capabilities.deterministic
        {
            return Err(capability_error(
                "deterministic_execution",
                "deterministic job",
            ));
        }

        if self.mode.requires_accelerator()
            && !self.capabilities.accelerator
        {
            return Err(capability_error(
                "accelerator",
                "accelerated job",
            ));
        }

        if self.mode.requires_distributed_capability()
            && !self.capabilities.distributed_execution
        {
            return Err(capability_error(
                "distributed_execution",
                "distributed job",
            ));
        }

        if self.mode.requires_qpu()
            && (!self.capabilities.qpu_submit
                || !self.capabilities.qpu_read_results)
        {
            return Err(capability_error(
                "qpu_submit/qpu_read_results",
                "QPU job",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Executor contract
// ============================================================================

/// Opaque result produced by a scheduled executor.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JobOutput {
    /// Optional domain-specific serialized result.
    pub payload: Vec<u8>,

    /// Optional stable execution fingerprint.
    pub fingerprint: Option<String>,
}

/// Execution context passed to a worker.
#[derive(Clone)]
pub struct ExecutionContext {
    pub job_id: JobId,
    pub worker_id: WorkerId,
    pub mode: ExecutionMode,
    pub resources: ResourceRequest,
    pub deadline: Deadline,
    pub cancellation: CancellationToken,
    pub deterministic: bool,
    pub attempt: u32,
}

impl ExecutionContext {
    /// Checks cancellation and deadline before expensive work.
    pub fn check(&self) -> SchedulerResult<()> {
        self.cancellation.check()?;

        if self.deadline.expired() {
            return Err(QecError::TimeLimitExceeded {
                elapsed_nanos: 0,
                limit_nanos: 0,
                message:
                    "scheduler job deadline expired"
                        .into(),
            });
        }

        Ok(())
    }
}

/// Backend-independent executable workload.
///
/// The scheduler does not interpret the executor's domain-specific result.
pub trait JobExecutor: Send + Sync + 'static {
    fn execute(
        &self,
        context: ExecutionContext,
    ) -> SchedulerResult<JobOutput>;
}

// ============================================================================
// Queue
// ============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QueueEntry {
    priority: Priority,
    sequence: u64,
    job_id: JobId,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .rank()
            .cmp(&other.priority.rank())
            .then_with(|| other.sequence.cmp(&self.sequence))
            .then_with(|| other.job_id.cmp(&self.job_id))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Internal job state
// ============================================================================

struct JobRecord {
    spec: JobSpec,
    executor: Arc<dyn JobExecutor>,
    state: JobState,
    cancellation: CancellationSource,
    attempts: u32,
    queue_sequence: u64,
    output: Option<JobOutput>,
    last_error: Option<QecError>,
    worker: Option<WorkerId>,
    submitted_at: Instant,
    started_at: Option<Instant>,
    completed_at: Option<Instant>,
    reservation_accounted: bool,
}

impl fmt::Debug for JobRecord {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("JobRecord")
            .field("state", &self.state)
            .field("attempts", &self.attempts)
            .field("queue_sequence", &self.queue_sequence)
            .field("output", &self.output)
            .field("last_error", &self.last_error)
            .field("worker", &self.worker)
            .field("submitted_at", &self.submitted_at)
            .field("started_at", &self.started_at)
            .field("completed_at", &self.completed_at)
            .finish()
    }
}

/// Immutable public job-status snapshot.
#[derive(Clone, Debug)]
pub struct JobStatus {
    pub job_id: JobId,
    pub state: JobState,
    pub attempts: u32,
    pub worker: Option<WorkerId>,
    pub output: Option<JobOutput>,
    pub last_error: Option<String>,
    pub submitted_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
}

// ============================================================================
// Scheduler internals
// ============================================================================

struct SchedulerInner {
    limits: QecLimits,
    queue: BinaryHeap<QueueEntry>,
    jobs: BTreeMap<JobId, JobRecord>,
    reservations: ReservationSnapshot,
    next_job_id: u64,
    next_sequence: u64,
    workers: BTreeMap<WorkerId, bool>,
}

// ============================================================================
// Scheduler
// ============================================================================

/// Production QEC scheduler.
///
/// Cloning a scheduler creates another handle to the same scheduler state.
///
/// Execution never occurs while the scheduler mutex is held. This is critical:
/// an executor may call back into status/cancellation/integration APIs without
/// deadlocking the scheduler.
#[derive(Clone)]
pub struct Scheduler {
    inner: Arc<Mutex<SchedulerInner>>,
    authorizer: CapabilityAuthorizer,
    accounting: Arc<dyn ResourceAccounting>,
}

impl fmt::Debug for Scheduler {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let inner =
            self.inner.lock().unwrap_or_else(
                |poisoned| poisoned.into_inner(),
            );

        f.debug_struct("Scheduler")
            .field("limits", &inner.limits)
            .field("queued_jobs", &inner.queue.len())
            .field("jobs", &inner.jobs.len())
            .field("reservations", &inner.reservations)
            .field("workers", &inner.workers.len())
            .finish()
    }
}

impl Scheduler {
    /// Creates a scheduler from canonical QEC limits.
    pub fn new(
        limits: QecLimits,
    ) -> SchedulerResult<Self> {
        limits
            .validate()
            .map_err(|error| {
                QecError::UnsupportedConfiguration {
                    feature: "qec_limits".into(),
                    message: error.to_string(),
                }
            })?;

        Ok(Self::with_components(
            limits,
            allow_all_capabilities(),
            Arc::new(NoopResourceAccounting),
        ))
    }

    /// Creates a scheduler from the canonical QEC configuration.
    pub fn from_config(
        config: &QecConfig,
    ) -> SchedulerResult<Self> {
        config
            .validate()
            .map_err(|error| {
                QecError::UnsupportedConfiguration {
                    feature:
                        "qec_scheduler_configuration"
                            .into(),
                    message: error.to_string(),
                }
            })?;

        Self::new(config.limits)
    }

    /// Creates a scheduler with external capability and resource authorities.
    ///
    /// `limits` must already have passed `QecLimits::validate()`.
    pub fn with_components(
        limits: QecLimits,
        authorizer: CapabilityAuthorizer,
        accounting: Arc<dyn ResourceAccounting>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                SchedulerInner {
                    limits,
                    queue: BinaryHeap::new(),
                    jobs: BTreeMap::new(),
                    reservations:
                        ReservationSnapshot::default(),
                    next_job_id: 0,
                    next_sequence: 0,
                    workers: BTreeMap::new(),
                },
            )),
            authorizer,
            accounting,
        }
    }

    // ------------------------------------------------------------------------
    // Worker management
    // ------------------------------------------------------------------------

    /// Registers an available worker.
    ///
    /// Registration is idempotent.
    pub fn register_worker(
        &self,
        worker_id: WorkerId,
    ) -> SchedulerResult<()> {
        let mut inner = lock_inner(&self.inner)?;
        inner.workers.insert(worker_id, true);
        Ok(())
    }

    /// Removes a worker from future scheduling.
    ///
    /// Existing work is not forcefully terminated.
    pub fn unregister_worker(
        &self,
        worker_id: WorkerId,
    ) -> SchedulerResult<()> {
        let mut inner = lock_inner(&self.inner)?;
        inner.workers.remove(&worker_id);
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Admission
    // ------------------------------------------------------------------------

    /// Validates, authorizes, reserves, and queues a job.
    ///
    /// Admission is atomic from the scheduler's perspective:
    ///
    /// ```text
    /// validate
    ///     ↓
    /// capability authorization
    ///     ↓
    /// deadline check
    ///     ↓
    /// aggregate reservation
    ///     ↓
    /// ResourceAccounting::reserve
    ///     ↓
    /// queue
    /// ```
    pub fn submit(
        &self,
        spec: JobSpec,
        executor: Arc<dyn JobExecutor>,
    ) -> SchedulerResult<JobId> {
        let mut inner = lock_inner(&self.inner)?;

        if inner.queue.len() >= MAX_QUEUE_DEPTH {
            return Err(resource_error(
                ResourceKind::Operations,
                (inner.queue.len() + 1) as u128,
                inner.queue.len() as u128,
                MAX_QUEUE_DEPTH as u128,
                "scheduler queue depth exceeded",
            ));
        }

        spec.validate(&inner.limits)?;

        (self.authorizer)(spec.capabilities)?;

        if spec.deadline.expired() {
            return Err(QecError::TimeLimitExceeded {
                elapsed_nanos: 0,
                limit_nanos: 0,
                message:
                    "job deadline already expired"
                        .into(),
            });
        }

        let new_reservation =
            inner
                .reservations
                .try_add(
                    spec.resources,
                    &inner.limits,
                )?;

        let job_id =
            JobId(inner.next_job_id);

        inner.next_job_id =
            inner
                .next_job_id
                .checked_add(1)
                .ok_or_else(|| {
                    numerical_error(
                        "scheduler job identifier overflow",
                    )
                })?;

        let sequence = inner.next_sequence;

        inner.next_sequence =
            inner
                .next_sequence
                .checked_add(1)
                .ok_or_else(|| {
                    numerical_error(
                        "scheduler sequence overflow",
                    )
                })?;

        /*
         * External resource accounting occurs before the scheduler commits
         * its own reservation.
         */
        self.accounting
            .reserve(job_id, spec.resources)?;

        let cancellation =
            CancellationSource::new();

        let job = JobRecord {
            spec: spec.clone(),
            executor,
            state: JobState::Admitted,
            cancellation,
            attempts: 0,
            queue_sequence: sequence,
            output: None,
            last_error: None,
            worker: None,
            submitted_at: Instant::now(),
            started_at: None,
            completed_at: None,
            reservation_accounted: true,
        };

        inner.reservations = new_reservation;

        inner.jobs.insert(job_id, job);

        inner.queue.push(QueueEntry {
            priority: spec.priority,
            sequence,
            job_id,
        });

        Ok(job_id)
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    /// Cancels a job cooperatively.
    pub fn cancel(
        &self,
        job_id: JobId,
        reason: CancellationReason,
    ) -> SchedulerResult<()> {
        let mut inner = lock_inner(&self.inner)?;

        let terminal = inner
            .jobs
            .get(&job_id)
            .ok_or_else(|| unknown_job(job_id))?
            .state
            .is_terminal();

        if terminal {
            return Ok(());
        }

        {
            let job = inner
                .jobs
                .get_mut(&job_id)
                .ok_or_else(|| unknown_job(job_id))?;

            job.cancellation
                .cancel_with_reason(reason);

            if matches!(
                job.state,
                JobState::Admitted
                    | JobState::Validating
                    | JobState::Paused
                    | JobState::Resuming
            ) {
                job.state = JobState::Cancelled;
                job.completed_at =
                    Some(Instant::now());
            }
        }

        let should_release = inner
            .jobs
            .get(&job_id)
            .map_or(false, |job| {
                job.state == JobState::Cancelled
                    && job.reservation_accounted
            });

        if should_release {
            release_job_resources(
                &self.accounting,
                &mut inner,
                job_id,
            )?;
        }

        Ok(())
    }

    /// Ordinary user-requested cancellation.
    pub fn cancel_requested(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        self.cancel(
            job_id,
            CancellationReason::Requested,
        )
    }

    // ------------------------------------------------------------------------
    // Execution
    // ------------------------------------------------------------------------

    /// Runs the next eligible job on a registered worker.
    ///
    /// The executor runs completely outside the scheduler mutex.
    pub fn run_next(
        &self,
        worker_id: WorkerId,
    ) -> SchedulerResult<Option<JobId>> {
        let selected = {
            let mut inner = lock_inner(&self.inner)?;

            if !inner
                .workers
                .get(&worker_id)
                .copied()
                .unwrap_or(false)
            {
                return Err(
                    QecError::BackendFailure {
                        backend:
                            "scheduler".into(),
                        message: format!(
                            "worker {:?} is not registered",
                            worker_id
                        ),
                    },
                );
            }

            let mut deferred =
                Vec::new();

            let selected_entry =
                loop {
                    let Some(entry) =
                        inner.queue.pop()
                    else {
                        break None;
                    };

                    let eligible =
                        inner
                            .jobs
                            .get(&entry.job_id)
                            .map_or(
                                false,
                                |job| {
                                    matches!(
                                        job.state,
                                        JobState::Admitted
                                            | JobState::Resuming
                                    )
                                },
                            );

                    if eligible {
                        break Some(entry);
                    }

                    deferred.push(entry);
                };

            for entry in deferred {
                inner.queue.push(entry);
            }

            let Some(entry) = selected_entry
            else {
                return Ok(None);
            };

            /*
             * Cancellation/deadline checks occur before taking a worker lease.
             */
            let pre_cancelled =
                inner
                    .jobs
                    .get(&entry.job_id)
                    .map_or(false, |job| {
                        job.cancellation.is_cancelled()
                    });

            let pre_expired =
                inner
                    .jobs
                    .get(&entry.job_id)
                    .map_or(false, |job| {
                        job.spec.deadline.expired()
                    });

            if pre_cancelled
                || pre_expired
            {
                let state =
                    if pre_expired {
                        JobState::TimedOut
                    } else {
                        JobState::Cancelled
                    };

                {
                    let job = inner
                        .jobs
                        .get_mut(&entry.job_id)
                        .ok_or_else(|| {
                            unknown_job(entry.job_id)
                        })?;

                    job.state = state;
                    job.completed_at =
                        Some(Instant::now());
                }

                release_job_resources(
                    &self.accounting,
                    &mut inner,
                    entry.job_id,
                )?;

                return Ok(Some(entry.job_id));
            }

            let (
                executor,
                context,
            ) = {
                let job = inner
                    .jobs
                    .get_mut(&entry.job_id)
                    .ok_or_else(|| {
                        unknown_job(entry.job_id)
                    })?;

                let attempt =
                    job.attempts;

                job.attempts =
                    job.attempts
                        .checked_add(1)
                        .ok_or_else(|| {
                            numerical_error(
                                "scheduler attempt overflow",
                            )
                        })?;

                job.state =
                    JobState::Running;

                job.worker =
                    Some(worker_id);

                if job.started_at.is_none()
                {
                    job.started_at =
                        Some(Instant::now());
                }

                let context =
                    ExecutionContext {
                        job_id: entry.job_id,
                        worker_id,
                        mode: job.spec.mode,
                        resources:
                            job.spec.resources,
                        deadline:
                            job.spec.deadline,
                        cancellation:
                            job.cancellation
                                .token(),
                        deterministic:
                            job.spec
                                .deterministic,
                        attempt,
                    };

                (
                    Arc::clone(&job.executor),
                    context,
                )
            };

            inner
                .workers
                .insert(worker_id, false);

            (
                entry.job_id,
                executor,
                context,
            )
        };

        let (
            job_id,
            executor,
            context,
        ) = selected;

        /*
         * The executor is completely outside the scheduler lock.
         */
        let execution_result =
            context
                .check()
                .and_then(|_| {
                    executor.execute(
                        context.clone(),
                    )
                });

        let mut inner =
            lock_inner(&self.inner)?;

        /*
         * Worker becomes available again regardless of execution result.
         */
        inner
            .workers
            .insert(worker_id, true);

        match execution_result {
            Ok(output) => {
                {
                    let job = inner
                        .jobs
                        .get_mut(&job_id)
                        .ok_or_else(|| {
                            unknown_job(job_id)
                        })?;

                    job.output =
                        Some(output);

                    job.last_error =
                        None;

                    job.worker = None;

                    job.state =
                        JobState::Completed;

                    job.completed_at =
                        Some(Instant::now());
                }

                release_job_resources(
                    &self.accounting,
                    &mut inner,
                    job_id,
                )?;
            }

            Err(error) => {
                let (
                    cancelled,
                    timed_out,
                    retry,
                    priority,
                ) = {
                    let job = inner
                        .jobs
                        .get_mut(&job_id)
                        .ok_or_else(|| {
                            unknown_job(job_id)
                        })?;

                    job.last_error =
                        Some(error.clone());

                    job.worker = None;

                    let timed_out =
                        matches!(
                            error,
                            QecError::TimeLimitExceeded {
                                ..
                            }
                        );

                    let cancelled =
                        matches!(
                            error,
                            QecError::CancellationRequested {
                                ..
                            }
                        ) || job
                            .cancellation
                            .is_cancelled();

                    let retry =
                        !cancelled
                            && !timed_out
                            && job.attempts
                                <= job
                                    .spec
                                    .max_retries;

                    (
                        cancelled,
                        timed_out,
                        retry,
                        job.spec.priority,
                    )
                };

                if cancelled
                    || timed_out
                {
                    {
                        let job = inner
                            .jobs
                            .get_mut(&job_id)
                            .ok_or_else(
                                || {
                                    unknown_job(
                                        job_id,
                                    )
                                },
                            )?;

                        job.state =
                            if timed_out {
                                JobState::TimedOut
                            } else {
                                JobState::Cancelled
                            };

                        job.completed_at =
                            Some(Instant::now());
                    }

                    release_job_resources(
                        &self.accounting,
                        &mut inner,
                        job_id,
                    )?;
                } else if retry {
                    let sequence =
                        inner
                            .next_sequence
                            .checked_add(1)
                            .ok_or_else(
                                || {
                                    numerical_error(
                                        "scheduler retry sequence overflow",
                                    )
                                },
                            )?;

                    inner.next_sequence =
                        sequence;

                    {
                        let job = inner
                            .jobs
                            .get_mut(&job_id)
                            .ok_or_else(
                                || {
                                    unknown_job(
                                        job_id,
                                    )
                                },
                            )?;

                        job.state =
                            JobState::Admitted;

                        job.queue_sequence =
                            sequence;
                    }

                    inner.queue.push(
                        QueueEntry {
                            priority,
                            sequence,
                            job_id,
                        },
                    );
                } else {
                    {
                        let job = inner
                            .jobs
                            .get_mut(&job_id)
                            .ok_or_else(
                                || {
                                    unknown_job(
                                        job_id,
                                    )
                                },
                            )?;

                        job.state =
                            JobState::Failed;

                        job.completed_at =
                            Some(Instant::now);
                    }

                    release_job_resources(
                        &self.accounting,
                        &mut inner,
                        job_id,
                    )?;
                }
            }
        }

        Ok(Some(job_id))
    }

    // ------------------------------------------------------------------------
    // Status
    // ------------------------------------------------------------------------

    /// Returns a stable public job-status snapshot.
    pub fn status(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<JobStatus> {
        let inner = lock_inner(&self.inner)?;

        let job = inner
            .jobs
            .get(&job_id)
            .ok_or_else(|| unknown_job(job_id))?;

        Ok(JobStatus {
            job_id,
            state: job.state,
            attempts: job.attempts,
            worker: job.worker,
            output: job.output.clone(),
            last_error: job
                .last_error
                .as_ref()
                .map(ToString::to_string),
            submitted_at: job.submitted_at,
            started_at: job.started_at,
            completed_at: job.completed_at,
        })
    }

    /// Returns the cancellation token for an admitted job.
    ///
    /// Executors should use this token rather than creating a second
    /// cancellation mechanism.
    pub fn cancellation_token(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<CancellationToken> {
        let inner = lock_inner(&self.inner)?;

        let job = inner
            .jobs
            .get(&job_id)
            .ok_or_else(|| unknown_job(job_id))?;

        Ok(job.cancellation.token())
    }

    // ------------------------------------------------------------------------
    // Checkpoint lifecycle
    // ------------------------------------------------------------------------

    /// Marks a running job as checkpointing.
    ///
    /// Serialization remains owned by `checkpoint.rs`.
    pub fn begin_checkpoint(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        let mut inner =
            lock_inner(&self.inner)?;

        let job = inner
            .jobs
            .get_mut(&job_id)
            .ok_or_else(|| unknown_job(job_id))?;

        if job.state != JobState::Running {
            return Err(invalid_transition(
                job.state,
                JobState::Checkpointing,
            ));
        }

        if !job.spec.checkpointable {
            return Err(capability_error(
                "checkpoint",
                "begin_checkpoint",
            ));
        }

        job.state =
            JobState::Checkpointing;

        Ok(())
    }

    /// Moves a successfully checkpointed job to paused state.
    pub fn pause_after_checkpoint(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        let mut inner =
            lock_inner(&self.inner)?;

        let job = inner
            .jobs
            .get_mut(&job_id)
            .ok_or_else(|| unknown_job(job_id))?;

        if job.state
            != JobState::Checkpointing
        {
            return Err(invalid_transition(
                job.state,
                JobState::Paused,
            ));
        }

        job.state =
            JobState::Paused;

        job.worker = None;

        Ok(())
    }

    /// Requeues a paused job.
    pub fn resume(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        let mut inner =
            lock_inner(&self.inner)?;

        let (
            priority,
            deadline_expired,
        ) = {
            let job = inner
                .jobs
                .get(&job_id)
                .ok_or_else(|| unknown_job(job_id))?;

            if job.state != JobState::Paused {
                return Err(invalid_transition(
                    job.state,
                    JobState::Resuming,
                ));
            }

            if job.cancellation.is_cancelled() {
                return Err(
                    QecError::CancellationRequested {
                        message:
                            "cannot resume a cancelled job"
                                .into(),
                    },
                );
            }

            (
                job.spec.priority,
                job.spec.deadline.expired(),
            )
        };

        if deadline_expired {
            {
                let job = inner
                    .jobs
                    .get_mut(&job_id)
                    .ok_or_else(|| {
                        unknown_job(job_id)
                    })?;

                job.state =
                    JobState::TimedOut;

                job.completed_at =
                    Some(Instant::now());
            }

            release_job_resources(
                &self.accounting,
                &mut inner,
                job_id,
            )?;

            return Ok(());
        }

        let sequence =
            inner.next_sequence;

        inner.next_sequence =
            sequence
                .checked_add(1)
                .ok_or_else(|| {
                    numerical_error(
                        "scheduler resume sequence overflow",
                    )
                })?;

        {
            let job = inner
                .jobs
                .get_mut(&job_id)
                .ok_or_else(|| unknown_job(job_id))?;

            job.state =
                JobState::Resuming;

            job.queue_sequence =
                sequence;
        }

        inner.queue.push(
            QueueEntry {
                priority,
                sequence,
                job_id,
            },
        );

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Introspection
    // ------------------------------------------------------------------------

    /// Returns the scheduler's current aggregate reservation.
    #[must_use]
    pub fn reservation_snapshot(
        &self,
    ) -> ReservationSnapshot {
        self.inner
            .lock()
            .unwrap_or_else(
                |poisoned| poisoned.into_inner(),
            )
            .reservations
    }

    /// Returns the number of tracked jobs.
    #[must_use]
    pub fn job_count(&self) -> usize {
        self.inner
            .lock()
            .unwrap_or_else(
                |poisoned| poisoned.into_inner(),
            )
            .jobs
            .len()
    }

    /// Returns the current queue depth.
    #[must_use]
    pub fn queue_depth(&self) -> usize {
        self.inner
            .lock()
            .unwrap_or_else(
                |poisoned| poisoned.into_inner(),
            )
            .queue
            .len()
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn lock_inner(
    inner: &Arc<Mutex<SchedulerInner>>,
) -> SchedulerResult<
    std::sync::MutexGuard<'_, SchedulerInner>,
> {
    inner.lock().map_err(|_| {
        QecError::InternalInvariantViolation {
            invariant:
                "scheduler mutex integrity".into(),
            message:
                "scheduler state mutex was poisoned"
                    .into(),
        }
    })
}

fn release_job_resources(
    accounting: &Arc<dyn ResourceAccounting>,
    inner: &mut SchedulerInner,
    job_id: JobId,
) -> SchedulerResult<()> {
    let request = {
        let job = inner
            .jobs
            .get(&job_id)
            .ok_or_else(|| unknown_job(job_id))?;

        if !job.reservation_accounted {
            return Ok(());
        }

        job.spec.resources
    };

    accounting.release(job_id, request)?;

    inner.reservations =
        inner.reservations.subtract(request);

    if let Some(job) =
        inner.jobs.get_mut(&job_id)
    {
        job.reservation_accounted = false;
    }

    Ok(())
}

fn unknown_job(job_id: JobId) -> QecError {
    QecError::InvalidInput {
        message: format!(
            "unknown scheduler job id {}",
            job_id.get()
        ),
    }
}

fn invalid_transition(
    from: JobState,
    to: JobState,
) -> QecError {
    QecError::InvalidInput {
        message: format!(
            "invalid scheduler state transition: {from:?} -> {to:?}"
        ),
    }
}

fn capability_error(
    capability: &str,
    operation: &str,
) -> QecError {
    QecError::CapabilityDenied {
        capability: capability.into(),
        operation: operation.into(),
        message: format!(
            "capability {capability} is required for {operation}"
        ),
    }
}

fn numerical_error(
    message: &str,
) -> QecError {
    QecError::NumericalFailure {
        operation:
            NumericalOperation::Accumulation,
        message: message.into(),
    }
}

fn resource_error(
    resource: ResourceKind,
    requested: u128,
    current: u128,
    limit: u128,
    message: &str,
) -> QecError {
    QecError::ResourceLimitExceeded {
        resource,
        requested,
        current,
        limit,
        message: message.into(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct SuccessfulExecutor;

    impl JobExecutor for SuccessfulExecutor {
        fn execute(
            &self,
            context: ExecutionContext,
        ) -> SchedulerResult<JobOutput> {
            context.check()?;

            Ok(JobOutput {
                payload: vec![1, 2, 3],
                fingerprint:
                    Some("successful-test".into()),
            })
        }
    }

    struct FailingExecutor;

    impl JobExecutor for FailingExecutor {
        fn execute(
            &self,
            _context: ExecutionContext,
        ) -> SchedulerResult<JobOutput> {
            Err(QecError::BackendFailure {
                backend: "scheduler-test".into(),
                message:
                    "intentional test failure"
                        .into(),
            })
        }
    }

    fn spec(
        priority: Priority,
    ) -> JobSpec {
        JobSpec {
            kind: JobKind::Decode,
            priority,
            deadline: Deadline::none(),
            resources: ResourceRequest {
                memory_bytes: 1024,
                parallel_workers: 1,
                decoder_iterations: 1,
                partitions: 1,
            },
            mode:
                ExecutionMode::SingleThread,
            deterministic: false,
            checkpointable: false,
            capabilities:
                CapabilityRequirement {
                    decode: true,
                    ..CapabilityRequirement::none()
                },
            max_retries: 0,
        }
    }

    #[test]
    fn higher_priority_runs_first() {
        let scheduler =
            Scheduler::new(
                QecLimits::default(),
            )
            .expect("valid limits");

        scheduler
            .register_worker(
                WorkerId::new(1),
            )
            .expect("worker");

        let low = scheduler
            .submit(
                spec(Priority::Low),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("low");

        let high = scheduler
            .submit(
                spec(Priority::High),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("high");

        let ran = scheduler
            .run_next(WorkerId::new(1))
            .expect("run")
            .expect("job");

        assert_eq!(ran, high);

        assert_eq!(
            scheduler
                .status(low)
                .expect("status")
                .state,
            JobState::Admitted
        );
    }

    #[test]
    fn successful_execution_releases_reservation() {
        let scheduler =
            Scheduler::new(
                QecLimits::default(),
            )
            .expect("valid limits");

        scheduler
            .register_worker(
                WorkerId::new(1),
            )
            .expect("worker");

        let id = scheduler
            .submit(
                spec(Priority::Normal),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("submit");

        assert_eq!(
            scheduler
                .reservation_snapshot()
                .memory_bytes,
            1024
        );

        scheduler
            .run_next(WorkerId::new(1))
            .expect("run");

        assert_eq!(
            scheduler.reservation_snapshot(),
            ReservationSnapshot::default()
        );

        assert_eq!(
            scheduler
                .status(id)
                .expect("status")
                .state,
            JobState::Completed
        );
    }

    #[test]
    fn failure_without_retry_is_terminal() {
        let scheduler =
            Scheduler::new(
                QecLimits::default(),
            )
            .expect("valid limits");

        scheduler
            .register_worker(
                WorkerId::new(1),
            )
            .expect("worker");

        let id = scheduler
            .submit(
                spec(Priority::Normal),
                Arc::new(
                    FailingExecutor,
                ),
            )
            .expect("submit");

        scheduler
            .run_next(WorkerId::new(1))
            .expect("run");

        assert_eq!(
            scheduler
                .status(id)
                .expect("status")
                .state,
            JobState::Failed
        );

        assert_eq!(
            scheduler.reservation_snapshot(),
            ReservationSnapshot::default()
        );
    }

    #[test]
    fn cancellation_is_cooperative() {
        let scheduler =
            Scheduler::new(
                QecLimits::default(),
            )
            .expect("valid limits");

        scheduler
            .register_worker(
                WorkerId::new(1),
            )
            .expect("worker");

        let id = scheduler
            .submit(
                spec(Priority::Normal),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("submit");

        scheduler
            .cancel_requested(id)
            .expect("cancel");

        assert_eq!(
            scheduler
                .status(id)
                .expect("status")
                .state,
            JobState::Cancelled
        );

        assert_eq!(
            scheduler.reservation_snapshot(),
            ReservationSnapshot::default()
        );
    }

    #[test]
    fn deterministic_fifo_for_equal_priority() {
        let scheduler =
            Scheduler::new(
                QecLimits::default(),
            )
            .expect("valid limits");

        scheduler
            .register_worker(
                WorkerId::new(1),
            )
            .expect("worker");

        let first = scheduler
            .submit(
                spec(Priority::Normal),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("first");

        let second = scheduler
            .submit(
                spec(Priority::Normal),
                Arc::new(
                    SuccessfulExecutor,
                ),
            )
            .expect("second");

        assert_eq!(
            scheduler
                .run_next(
                    WorkerId::new(1)
                )
                .expect("run")
                .expect("job"),
            first
        );

        assert_eq!(
            scheduler
                .run_next(
                    WorkerId::new(1)
                )
                .expect("run")
                .expect("job"),
            second
        );
    }
}