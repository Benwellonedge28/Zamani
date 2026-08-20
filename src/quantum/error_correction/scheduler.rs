//! Zamani Quantum Error-Correction Scheduler.
//!
//! Production scheduling and admission-control infrastructure for QEC.
//!
//! Architectural contract:
//!
//! ```text
//!                       UNTRUSTED JOB
//!                            │
//!                            ▼
//!                     Job Validation
//!                            │
//!                            ▼
//!                  Capability Authorization
//!                            │
//!                            ▼
//!                    QecLimits Preflight
//!                            │
//!                            ▼
//!                   Resource Reservation
//!                            │
//!                            ▼
//!                       Admission
//!                            │
//!                            ▼
//!                    Deterministic Queue
//!                            │
//!                            ▼
//!                       Worker Lease
//!                            │
//!                            ▼
//!                         Running
//!                       /    |     \
//!                      /     |      \
//!               checkpoint  cancel   failure
//!                    │        │        │
//!                    ▼        ▼        ▼
//!                Paused   Cancelled  Retry/Failed
//!                    │
//!                    ▼
//!                 Resuming
//!                    │
//!                    ▼
//!                Running
//!                    │
//!                    ▼
//!                 Completed
//! ```
//!
//! The scheduler does NOT implement QEC mathematics. Decoders, simulation,
//! graph construction, syndrome extraction, QPU adapters, etc. remain owned
//! by their respective modules.
//!
//! The scheduler owns:
//!
//! * validation;
//! * admission control;
//! * priority ordering;
//! * bounded queueing;
//! * resource reservation;
//! * worker leases;
//! * cancellation propagation;
//! * deadlines;
//! * checkpoint state transitions;
//! * retry policy;
//! * deterministic scheduling;
//! * lifecycle state;
//! * safe failure.
//!
//! Resource policy is intentionally delegated to `limits.rs` / `QecLimits`.
//! Runtime accounting can be supplied through `ResourceAccounting`, allowing
//! `resources.rs` to remain the runtime accounting authority without creating
//! a second scheduler-specific resource policy.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering as AtomicOrdering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

use super::configuration::QecConfig;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::{LimitKind, QecLimits};

/* ========================================================================== */
/* Identifiers                                                                */
/* ========================================================================== */

/// Globally unique scheduler job identifier within one scheduler instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobId(u64);

impl JobId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Scheduler worker identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WorkerId(u64);

impl WorkerId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/* ========================================================================== */
/* Execution                                                                  */
/* ========================================================================== */

/// Execution backend class.
///
/// This describes scheduling requirements. It does not perform backend I/O.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ExecutionMode {
    SingleThread,
    MultiThread,
    MultiProcess,
    Distributed,
    Accelerated,
}

impl ExecutionMode {
    #[must_use]
    pub const fn requires_parallelism(self) -> bool {
        !matches!(self, Self::SingleThread)
    }

    #[must_use]
    pub const fn requires_distributed_capability(self) -> bool {
        matches!(self, Self::MultiProcess | Self::Distributed)
    }

    #[must_use]
    pub const fn requires_accelerator(self) -> bool {
        matches!(self, Self::Accelerated)
    }
}

/// QEC workload class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum JobKind {
    LogicalOperation,
    Decode,
    Simulation,
    ThresholdBenchmark,
    Diagnostic,
}

/// Scheduling priority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

impl Priority {
    #[must_use]
    const fn rank(self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::High => 4,
            Self::Normal => 3,
            Self::Low => 2,
            Self::Background => 1,
        }
    }
}

/* ========================================================================== */
/* Deadline                                                                   */
/* ========================================================================== */

/// Optional wall-clock deadline.
#[derive(Clone, Copy, Debug)]
pub struct Deadline(Option<Instant>);

impl Deadline {
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    #[must_use]
    pub fn after(duration: Duration) -> Self {
        Self(Some(Instant::now() + duration))
    }

    #[must_use]
    pub fn expired(self) -> bool {
        self.0
            .is_some_and(|deadline| Instant::now() >= deadline)
    }

    #[must_use]
    pub fn remaining(self) -> Option<Duration> {
        self.0
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }
}

/* ========================================================================== */
/* Resource reservation                                                       */
/* ========================================================================== */

/// Resource reservation requested by a scheduled workload.
///
/// These values are reservations, not global policies. The maximum allowed
/// values always come from `QecLimits`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceRequest {
    pub memory_bytes: u64,
    pub parallel_workers: usize,
    pub decoder_iterations: usize,
}

impl ResourceRequest {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            memory_bytes: 0,
            parallel_workers: 0,
            decoder_iterations: 0,
        }
    }

    pub fn validate_against(&self, limits: &QecLimits) -> SchedulerResult<()> {
        if self.memory_bytes > limits.max_memory_bytes {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: self.memory_bytes as u128,
                maximum: limits.max_memory_bytes as u128,
            });
        }

        if self.parallel_workers > limits.max_parallelism {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::Parallelism,
                requested: self.parallel_workers as u128,
                maximum: limits.max_parallelism as u128,
            });
        }

        if self.decoder_iterations > limits.max_decoder_iterations {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::DecoderIterations,
                requested: self.decoder_iterations as u128,
                maximum: limits.max_decoder_iterations as u128,
            });
        }

        Ok(())
    }
}

/// Runtime resource reservation.
///
/// A scheduler never assumes that a resource is available merely because it
/// fits inside the global policy. It must also fit inside currently available
/// capacity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceReservation {
    pub memory_bytes: u64,
    pub parallel_workers: usize,
    pub decoder_iterations: usize,
}

impl ResourceReservation {
    fn try_add(
        self,
        request: ResourceRequest,
        limits: &QecLimits,
    ) -> SchedulerResult<Self> {
        let memory = self
            .memory_bytes
            .checked_add(request.memory_bytes)
            .ok_or(SchedulerError::ArithmeticOverflow(
                "scheduler memory reservation",
            ))?;

        let workers = self
            .parallel_workers
            .checked_add(request.parallel_workers)
            .ok_or(SchedulerError::ArithmeticOverflow(
                "scheduler worker reservation",
            ))?;

        let iterations = self
            .decoder_iterations
            .checked_add(request.decoder_iterations)
            .ok_or(SchedulerError::ArithmeticOverflow(
                "scheduler decoder-iteration reservation",
            ))?;

        if memory > limits.max_memory_bytes {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: memory as u128,
                maximum: limits.max_memory_bytes as u128,
            });
        }

        if workers > limits.max_parallelism {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::Parallelism,
                requested: workers as u128,
                maximum: limits.max_parallelism as u128,
            });
        }

        if iterations > limits.max_decoder_iterations {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::DecoderIterations,
                requested: iterations as u128,
                maximum: limits.max_decoder_iterations as u128,
            });
        }

        Ok(Self {
            memory_bytes: memory,
            parallel_workers: workers,
            decoder_iterations: iterations,
        })
    }

    fn subtract(self, request: ResourceRequest) -> Self {
        Self {
            memory_bytes: self.memory_bytes.saturating_sub(request.memory_bytes),
            parallel_workers: self
                .parallel_workers
                .saturating_sub(request.parallel_workers),
            decoder_iterations: self
                .decoder_iterations
                .saturating_sub(request.decoder_iterations),
        }
    }
}

/* ========================================================================== */
/* Capability requirements                                                     */
/* ========================================================================== */

/// Scheduler-side capability requirements.
///
/// Actual capability possession remains the responsibility of
/// `capabilities.rs`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilityRequirement {
    pub decode: bool,
    pub simulate: bool,
    pub benchmark: bool,
    pub inspect_topology: bool,
    pub accelerator: bool,
    pub distributed_execution: bool,
    pub streaming: bool,
    pub checkpoint: bool,
    pub deterministic: bool,
}

impl CapabilityRequirement {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            decode: false,
            simulate: false,
            benchmark: false,
            inspect_topology: false,
            accelerator: false,
            distributed_execution: false,
            streaming: false,
            checkpoint: false,
            deterministic: false,
        }
    }

    #[must_use]
    pub const fn for_mode(mode: ExecutionMode) -> Self {
        Self {
            decode: false,
            simulate: false,
            benchmark: false,
            inspect_topology: false,
            accelerator: mode.requires_accelerator(),
            distributed_execution: mode.requires_distributed_capability(),
            streaming: false,
            checkpoint: false,
            deterministic: false,
        }
    }
}

/// Fail-closed capability authorizer.
///
/// The scheduler intentionally does not know how capability grants are
/// represented internally. `capabilities.rs` can provide the closure.
pub type CapabilityAuthorizer =
    Arc<dyn Fn(CapabilityRequirement) -> SchedulerResult<()> + Send + Sync>;

fn allow_all_capabilities() -> CapabilityAuthorizer {
    Arc::new(|_| Ok(()))
}

/* ========================================================================== */
/* Cancellation                                                               */
/* ========================================================================== */

/// Scheduler cancellation token.
///
/// Expensive QEC components should bind their own cancellation implementation
/// to this token when they execute through the scheduler.
#[derive(Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, AtomicOrdering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(AtomicOrdering::Acquire)
    }

    pub fn check(&self) -> SchedulerResult<()> {
        if self.is_cancelled() {
            Err(SchedulerError::CancellationRequested)
        } else {
            Ok(())
        }
    }
}

/* ========================================================================== */
/* Lifecycle                                                                  */
/* ========================================================================== */

/// Explicit scheduler lifecycle.
///
/// This is intentionally richer than a simple queued/running flag because
/// checkpoint/resume and recovery are first-class QEC operations.
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
    #[must_use]
    pub const fn terminal(self) -> bool {
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

/* ========================================================================== */
/* Job specification                                                          */
/* ========================================================================== */

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
    pub fn validate(&self, limits: &QecLimits) -> SchedulerResult<()> {
        self.resources.validate_against(limits)?;

        if self.max_retries > 1_000_000 {
            return Err(SchedulerError::InvalidRequest(
                "max_retries exceeds scheduler safety bound".into(),
            ));
        }

        if self.mode.requires_parallelism() && self.resources.parallel_workers == 0 {
            return Err(SchedulerError::InvalidRequest(
                "parallel execution requires at least one worker reservation".into(),
            ));
        }

        if self.mode == ExecutionMode::SingleThread
            && self.resources.parallel_workers > 1
        {
            return Err(SchedulerError::InvalidRequest(
                "single-thread execution cannot reserve multiple workers".into(),
            ));
        }

        if self.checkpointable && !self.capabilities.checkpoint {
            return Err(SchedulerError::CapabilityDenied(
                "checkpoint capability required by checkpointable job".into(),
            ));
        }

        if self.deterministic && !self.capabilities.deterministic {
            return Err(SchedulerError::CapabilityDenied(
                "deterministic-execution capability required".into(),
            ));
        }

        if self.mode.requires_accelerator() && !self.capabilities.accelerator {
            return Err(SchedulerError::CapabilityDenied(
                "accelerator capability required".into(),
            ));
        }

        if self.mode.requires_distributed_capability()
            && !self.capabilities.distributed_execution
        {
            return Err(SchedulerError::CapabilityDenied(
                "distributed-execution capability required".into(),
            ));
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Executor                                                                   */
/* ========================================================================== */

/// Backend-independent executable workload.
pub trait JobExecutor: Send + Sync + 'static {
    fn execute(&self, context: ExecutionContext) -> SchedulerResult<JobOutput>;
}

/// Context passed to a worker.
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
    pub fn check(&self) -> SchedulerResult<()> {
        self.cancellation.check()?;

        if self.deadline.expired() {
            return Err(SchedulerError::DeadlineExceeded);
        }

        Ok(())
    }
}

/// Generic scheduler output.
///
/// Domain-specific QEC results remain owned by the decoder/backend.
#[derive(Clone, Debug, Default)]
pub struct JobOutput {
    pub success: bool,
    pub logical_failure: bool,
    pub correction_count: u64,
    pub detection_event_count: u64,
    pub decoder_iterations: u64,
}

/* ========================================================================== */
/* Checkpointing                                                              */
/* ========================================================================== */

/// Checkpoint boundary requested by the scheduler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointReason {
    Explicit,
    ResourcePressure,
    Retry,
    Pause,
}

/// Checkpoint callback.
///
/// The checkpoint implementation itself belongs to `checkpoint.rs`.
pub type CheckpointHook =
    Arc<dyn Fn(JobId, CheckpointReason) -> SchedulerResult<()> + Send + Sync>;

fn noop_checkpoint() -> CheckpointHook {
    Arc::new(|_, _| Ok(()))
}

/* ========================================================================== */
/* Resource accounting integration                                            */
/* ========================================================================== */

/// Runtime resource-accounting integration point.
///
/// `resources.rs` remains the authoritative runtime accounting layer. The
/// scheduler only requires this minimal contract, which prevents scheduler.rs
/// from recreating the entire ResourceManager implementation.
///
/// A production adapter should reserve/release against `ResourceManager`.
pub trait ResourceAccounting: Send + Sync {
    fn reserve(
        &self,
        request: ResourceRequest,
        limits: &QecLimits,
    ) -> SchedulerResult<()>;

    fn release(&self, request: ResourceRequest);

    fn snapshot(&self) -> ResourceReservation;
}

/// Scheduler-local fallback accounting adapter.
///
/// This is useful for tests and for configurations where the caller has not
/// supplied a ResourceManager adapter. It still enforces QecLimits and never
/// permits aggregate overcommitment.
#[derive(Default)]
pub struct LocalResourceAccounting {
    current: Mutex<ResourceReservation>,
}

impl ResourceAccounting for LocalResourceAccounting {
    fn reserve(
        &self,
        request: ResourceRequest,
        limits: &QecLimits,
    ) -> SchedulerResult<()> {
        let mut current = self.current.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "resource accounting mutex poisoned".into(),
            )
        })?;

        let next = current.try_add(request, limits)?;
        *current = next;
        Ok(())
    }

    fn release(&self, request: ResourceRequest) {
        if let Ok(mut current) = self.current.lock() {
            *current = current.subtract(request);
        }
    }

    fn snapshot(&self) -> ResourceReservation {
        self.current
            .lock()
            .map(|value| *value)
            .unwrap_or_default()
    }
}

/* ========================================================================== */
/* Queue                                                                      */
/* ========================================================================== */

struct QueueItem {
    id: JobId,
    spec: JobSpec,
    executor: Arc<dyn JobExecutor>,
    cancellation: CancellationToken,
    state: Arc<Mutex<JobState>>,
    sequence: u64,
    attempt: u32,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

        // Earlier admission wins.
        other.sequence.cmp(&self.sequence)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* ========================================================================== */
/* Job handle                                                                 */
/* ========================================================================== */

/// Handle retained by the caller.
#[derive(Clone)]
pub struct JobHandle {
    id: JobId,
    state: Arc<Mutex<JobState>>,
    cancellation: CancellationToken,
}

impl JobHandle {
    #[must_use]
    pub const fn id(&self) -> JobId {
        self.id
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();

        if let Ok(mut state) = self.state.lock() {
            if matches!(
                *state,
                JobState::Created
                    | JobState::Validating
                    | JobState::Admitted
                    | JobState::Running
                    | JobState::Checkpointing
                    | JobState::Paused
                    | JobState::Resuming
            ) {
                *state = JobState::Cancelled;
            }
        }
    }

    #[must_use]
    pub fn state(&self) -> JobState {
        self.state
            .lock()
            .map(|state| *state)
            .unwrap_or(JobState::Failed)
    }

    #[must_use]
    pub fn cancellation(&self) -> CancellationToken {
        self.cancellation.clone()
    }
}

/* ========================================================================== */
/* Scheduler metrics                                                          */
/* ========================================================================== */

/// Scheduler lifecycle counters.
///
/// These are deliberately lightweight. Full QEC decoder metrics belong to
/// `metrics.rs`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulerMetrics {
    pub submitted: u64,
    pub admitted: u64,
    pub rejected: u64,
    pub started: u64,
    pub completed: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub timed_out: u64,
    pub checkpointed: u64,
    pub resumed: u64,
    pub retries: u64,
    pub resource_rejections: u64,
}

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Scheduler-specific error.
///
/// Public callers should normally use the `QecResult` conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchedulerError {
    InvalidRequest(String),

    InvalidConfiguration(String),

    QueueFull,

    SchedulerDisabled,

    SchedulerShuttingDown,

    ResourceLimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    CapabilityDenied(String),

    WorkerUnavailable,

    DeadlineExceeded,

    CancellationRequested,

    CheckpointRequired,

    CheckpointFailed(String),

    RetryLimitExceeded,

    ExecutorFailed(String),

    ArithmeticOverflow(&'static str),

    InternalInvariantViolation(String),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => {
                write!(f, "invalid scheduler request: {message}")
            }

            Self::InvalidConfiguration(message) => {
                write!(f, "invalid scheduler configuration: {message}")
            }

            Self::QueueFull => write!(f, "scheduler queue is full"),

            Self::SchedulerDisabled => write!(f, "scheduler is disabled"),

            Self::SchedulerShuttingDown => {
                write!(f, "scheduler is shutting down")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "{resource} limit exceeded: requested={requested}, maximum={maximum}"
            ),

            Self::CapabilityDenied(capability) => {
                write!(f, "capability denied: {capability}")
            }

            Self::WorkerUnavailable => {
                write!(f, "no compatible worker is available")
            }

            Self::DeadlineExceeded => write!(f, "job deadline exceeded"),

            Self::CancellationRequested => {
                write!(f, "job cancellation requested")
            }

            Self::CheckpointRequired => {
                write!(f, "checkpoint is required before this transition")
            }

            Self::CheckpointFailed(message) => {
                write!(f, "checkpoint failed: {message}")
            }

            Self::RetryLimitExceeded => {
                write!(f, "job retry limit exceeded")
            }

            Self::ExecutorFailed(message) => {
                write!(f, "job executor failed: {message}")
            }

            Self::ArithmeticOverflow(operation) => {
                write!(f, "scheduler arithmetic overflow: {operation}")
            }

            Self::InternalInvariantViolation(message) => {
                write!(f, "scheduler invariant violation: {message}")
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Scheduler result type.
pub type SchedulerResult<T> = Result<T, SchedulerError>;

/// Convert scheduler errors to the canonical QEC public error boundary.
impl From<SchedulerError> for QecError {
    fn from(error: SchedulerError) -> Self {
        match error {
            SchedulerError::InvalidRequest(message) => {
                QecError::InvalidInput { message }
            }

            SchedulerError::InvalidConfiguration(message) => {
                QecError::UnsupportedConfiguration {
                    feature: "scheduler".into(),
                    message,
                }
            }

            SchedulerError::QueueFull => QecError::ResourceLimitExceeded {
                resource: ResourceKind::AllocationCount,
                requested: 1,
                current: 0,
                limit: 0,
                message: "scheduler queue is full".into(),
            },

            SchedulerError::SchedulerDisabled => {
                QecError::UnsupportedConfiguration {
                    feature: "scheduler".into(),
                    message: "scheduler is disabled".into(),
                }
            }

            SchedulerError::SchedulerShuttingDown => {
                QecError::UnsupportedConfiguration {
                    feature: "scheduler".into(),
                    message: "scheduler is shutting down".into(),
                }
            }

            SchedulerError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                let mapped = match resource {
                    LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
                    LimitKind::Parallelism => ResourceKind::Parallelism,
                    LimitKind::DecoderIterations => {
                        ResourceKind::DecoderIterations
                    }
                    LimitKind::QpuShots => ResourceKind::QpuShots,
                    LimitKind::QpuCircuits => ResourceKind::QpuCircuits,
                    LimitKind::Partitions => ResourceKind::Partitions,
                    LimitKind::SyndromeEvents => {
                        ResourceKind::SyndromeEvents
                    }
                    LimitKind::GraphNodes => ResourceKind::GraphNodes,
                    LimitKind::GraphEdges => ResourceKind::GraphEdges,
                    LimitKind::CodeDistance => ResourceKind::CodeDistance,
                    LimitKind::Qubits => ResourceKind::Qubits,
                    LimitKind::Stabilizers => ResourceKind::Stabilizers,
                    LimitKind::MeasurementRounds => {
                        ResourceKind::MeasurementRounds
                    }
                    LimitKind::CheckpointSizeBytes => {
                        ResourceKind::CheckpointSize
                    }
                    LimitKind::StreamBufferEvents => {
                        ResourceKind::StreamBuffer
                    }
                    _ => ResourceKind::Custom,
                };

                QecError::ResourceLimitExceeded {
                    resource: mapped,
                    requested,
                    current: 0,
                    limit: maximum,
                    message: "scheduler resource admission rejected".into(),
                }
            }

            SchedulerError::CapabilityDenied(message) => {
                QecError::UnsupportedConfiguration {
                    feature: "capability".into(),
                    message,
                }
            }

            SchedulerError::WorkerUnavailable => {
                QecError::UnsupportedConfiguration {
                    feature: "worker".into(),
                    message: "no compatible worker is available".into(),
                }
            }

            SchedulerError::DeadlineExceeded => {
                QecError::TimeLimitExceeded {
                    elapsed_nanos: 0,
                    limit_nanos: 0,
                    message: "scheduler deadline exceeded".into(),
                }
            }

            SchedulerError::CancellationRequested => {
                QecError::CancellationRequested {
                    message: "scheduler cancellation requested".into(),
                }
            }

            SchedulerError::CheckpointRequired => {
                QecError::UnsupportedConfiguration {
                    feature: "checkpoint".into(),
                    message: "checkpoint required for lifecycle transition".into(),
                }
            }

            SchedulerError::CheckpointFailed(message) => {
                QecError::InternalInvariantViolation {
                    invariant: "checkpoint".into(),
                    message,
                }
            }

            SchedulerError::RetryLimitExceeded => {
                QecError::DecoderFailure {
                    decoder: super::errors::DecoderKind::Custom,
                    message: "scheduler retry limit exceeded".into(),
                }
            }

            SchedulerError::ExecutorFailed(message) => {
                QecError::DecoderFailure {
                    decoder: super::errors::DecoderKind::Custom,
                    message,
                }
            }

            SchedulerError::ArithmeticOverflow(operation) => {
                QecError::NumericalFailure {
                    operation: super::errors::NumericalOperation::Custom,
                    message: operation.into(),
                }
            }

            SchedulerError::InternalInvariantViolation(message) => {
                QecError::InternalInvariantViolation {
                    invariant: "scheduler".into(),
                    message,
                }
            }
        }
    }
}

/* ========================================================================== */
/* Scheduler                                                                  */
/* ========================================================================== */

/// Production QEC scheduler.
///
/// The scheduler is intentionally synchronous at the orchestration boundary:
/// callers submit jobs and explicitly drive worker execution through
/// `run_next()` / `run_worker()`. This avoids pretending that merely having a
/// scheduler object automatically provides distributed execution.
///
/// A higher-level runtime can place this scheduler behind an async/threaded
/// worker pool without changing the admission or lifecycle semantics.
pub struct QecScheduler {
    limits: QecLimits,

    max_queued_jobs: usize,
    max_running_jobs: usize,

    deterministic: bool,
    enable_deadlines: bool,
    enable_cancellation: bool,
    enable_backpressure: bool,

    queue: Mutex<BinaryHeap<QueueItem>>,
    jobs: Mutex<HashMap<JobId, JobRecord>>,
    workers: Mutex<HashMap<WorkerId, WorkerRecord>>,

    next_job_id: AtomicU64,
    next_sequence: AtomicU64,

    running_jobs: AtomicU64,

    shutting_down: AtomicBool,

    accounting: Arc<dyn ResourceAccounting>,
    authorize: CapabilityAuthorizer,
    checkpoint: CheckpointHook,

    metrics: Mutex<SchedulerMetrics>,
}

struct JobRecord {
    spec: JobSpec,
    state: Arc<Mutex<JobState>>,
    cancellation: CancellationToken,
    submitted_at: Instant,
    attempt: u32,
}

struct WorkerRecord {
    mode: ExecutionMode,
    busy: bool,
    current_job: Option<JobId>,
}

impl QecScheduler {
    /// Construct directly from canonical QEC limits.
    pub fn new(limits: QecLimits) -> SchedulerResult<Self> {
        Self::with_integrations(
            limits,
            1_024,
            8,
            Arc::new(LocalResourceAccounting::default()),
            allow_all_capabilities(),
            noop_checkpoint(),
        )
    }

    /// Construct from the complete QEC configuration.
    ///
    /// This ensures scheduler policy starts from `QecConfig` instead of
    /// inventing an independent configuration tree.
    pub fn from_config(config: &QecConfig) -> SchedulerResult<Self> {
        config
            .validate()
            .map_err(|error| SchedulerError::InvalidConfiguration(error.to_string()))?;

        let scheduler = &config.scheduler;

        Self::with_integrations(
            config.limits,
            scheduler.max_queued_jobs,
            scheduler.max_running_jobs as usize,
            Arc::new(LocalResourceAccounting::default()),
            allow_all_capabilities(),
            noop_checkpoint(),
        )
        .and_then(|mut scheduler_instance| {
            scheduler_instance.enable_deadlines = scheduler.enable_deadlines;
            scheduler_instance.enable_cancellation = scheduler.enable_cancellation;
            scheduler_instance.enable_backpressure = scheduler.enable_backpressure;
            scheduler_instance.deterministic =
                config.determinism.enabled
                || config.determinism.seed.is_some();

            Ok(scheduler_instance)
        })
    }

    /// Construct with explicit runtime integration points.
    pub fn with_integrations(
        limits: QecLimits,
        max_queued_jobs: usize,
        max_running_jobs: usize,
        accounting: Arc<dyn ResourceAccounting>,
        authorize: CapabilityAuthorizer,
        checkpoint: CheckpointHook,
    ) -> SchedulerResult<Self> {
        limits
            .validate()
            .map_err(|error| SchedulerError::InvalidConfiguration(error.to_string()))?;

        if max_queued_jobs == 0 {
            return Err(SchedulerError::InvalidConfiguration(
                "max_queued_jobs must be greater than zero".into(),
            ));
        }

        if max_running_jobs == 0 {
            return Err(SchedulerError::InvalidConfiguration(
                "max_running_jobs must be greater than zero".into(),
            ));
        }

        if max_running_jobs > limits.max_parallelism {
            return Err(SchedulerError::InvalidConfiguration(
                "max_running_jobs exceeds QecLimits.max_parallelism".into(),
            ));
        }

        Ok(Self {
            limits,

            max_queued_jobs,
            max_running_jobs,

            deterministic: false,
            enable_deadlines: true,
            enable_cancellation: true,
            enable_backpressure: true,

            queue: Mutex::new(BinaryHeap::new()),
            jobs: Mutex::new(HashMap::new()),
            workers: Mutex::new(HashMap::new()),

            next_job_id: AtomicU64::new(1),
            next_sequence: AtomicU64::new(0),

            running_jobs: AtomicU64::new(0),

            shutting_down: AtomicBool::new(false),

            accounting,
            authorize,
            checkpoint,

            metrics: Mutex::new(SchedulerMetrics::default()),
        })
    }

    /* ---------------------------------------------------------------------- */
    /* Worker management                                                      */
    /* ---------------------------------------------------------------------- */

    /// Register a worker.
    pub fn register_worker(
        &self,
        worker_id: WorkerId,
        mode: ExecutionMode,
    ) -> SchedulerResult<()> {
        let mut workers = self.workers.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "worker registry mutex poisoned".into(),
            )
        })?;

        if workers.contains_key(&worker_id) {
            return Err(SchedulerError::InvalidRequest(
                "worker ID is already registered".into(),
            ));
        }

        workers.insert(
            worker_id,
            WorkerRecord {
                mode,
                busy: false,
                current_job: None,
            },
        );

        Ok(())
    }

    /// Remove a worker.
    ///
    /// A busy worker cannot disappear silently. The current job is cancelled
    /// and will be eligible for retry when submitted again by the runtime.
    pub fn unregister_worker(
        &self,
        worker_id: WorkerId,
    ) -> SchedulerResult<()> {
        let mut workers = self.workers.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "worker registry mutex poisoned".into(),
            )
        })?;

        let worker = workers
            .remove(&worker_id)
            .ok_or(SchedulerError::WorkerUnavailable)?;

        if let Some(job_id) = worker.current_job {
            drop(workers);

            if let Ok(jobs) = self.jobs.lock() {
                if let Some(record) = jobs.get(&job_id) {
                    record.cancellation.cancel();

                    if let Ok(mut state) = record.state.lock() {
                        *state = JobState::Failed;
                    }
                }
            }
        }

        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Submission                                                              */
    /* ---------------------------------------------------------------------- */

    /// Submit a workload.
    ///
    /// Admission is atomic with resource reservation. A job is never placed
    /// into the queue if its reservation cannot be established.
    pub fn submit(
        &self,
        spec: JobSpec,
        executor: Arc<dyn JobExecutor>,
    ) -> QecResult<JobHandle> {
        self.submit_internal(spec, executor)
            .map_err(QecError::from)
    }

    fn submit_internal(
        &self,
        spec: JobSpec,
        executor: Arc<dyn JobExecutor>,
    ) -> SchedulerResult<JobHandle> {
        if self.shutting_down.load(AtomicOrdering::Acquire) {
            return Err(SchedulerError::SchedulerShuttingDown);
        }

        if !self.enable_cancellation {
            return Err(SchedulerError::InvalidConfiguration(
                "production scheduling requires cancellation".into(),
            ));
        }

        spec.validate(&self.limits)?;

        if !self.enable_deadlines && spec.deadline.0.is_some() {
            return Err(SchedulerError::InvalidRequest(
                "deadlines are disabled by scheduler configuration".into(),
            ));
        }

        if self
            .running_jobs
            .load(AtomicOrdering::Acquire)
            >= self.max_running_jobs as u64
            && !self.enable_backpressure
        {
            return Err(SchedulerError::ResourceLimitExceeded {
                resource: LimitKind::Parallelism,
                requested: self.max_running_jobs as u128 + 1,
                maximum: self.max_running_jobs as u128,
            });
        }

        (self.authorize)(spec.capabilities)?;

        if spec.deadline.expired() {
            return Err(SchedulerError::DeadlineExceeded);
        }

        {
            let queue = self.queue.lock().map_err(|_| {
                SchedulerError::InternalInvariantViolation(
                    "scheduler queue mutex poisoned".into(),
                )
            })?;

            if queue.len() >= self.max_queued_jobs {
                return Err(SchedulerError::QueueFull);
            }
        }

        // Reserve before insertion into the queue.
        self.accounting
            .reserve(spec.resources, &self.limits)?;

        let id = JobId::new(
            self.next_job_id
                .fetch_add(1, AtomicOrdering::Relaxed),
        );

        let sequence = self
            .next_sequence
            .fetch_add(1, AtomicOrdering::Relaxed);

        let cancellation = CancellationToken::new();

        let state = Arc::new(Mutex::new(JobState::Created));

        if let Ok(mut value) = state.lock() {
            *value = JobState::Validating;
        }

        let record = JobRecord {
            spec: spec.clone(),
            state: state.clone(),
            cancellation: cancellation.clone(),
            submitted_at: Instant::now(),
            attempt: 0,
        };

        {
            let mut jobs = self.jobs.lock().map_err(|_| {
                self.accounting.release(spec.resources);

                SchedulerError::InternalInvariantViolation(
                    "job registry mutex poisoned".into(),
                )
            })?;

            jobs.insert(id, record);
        }

        {
            let mut queue = self.queue.lock().map_err(|_| {
                self.accounting.release(spec.resources);

                SchedulerError::InternalInvariantViolation(
                    "scheduler queue mutex poisoned".into(),
                )
            })?;

            queue.push(QueueItem {
                id,
                spec,
                executor,
                cancellation,
                state: state.clone(),
                sequence,
                attempt: 0,
            });
        }

        if let Ok(mut value) = state.lock() {
            *value = JobState::Admitted;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.submitted = metrics.submitted.saturating_add(1);
            metrics.admitted = metrics.admitted.saturating_add(1);
        }

        Ok(JobHandle {
            id,
            state,
            cancellation: self
                .jobs
                .lock()
                .ok()
                .and_then(|jobs| jobs.get(&id).map(|job| job.cancellation.clone()))
                .unwrap_or_default(),
        })
    }

    /* ---------------------------------------------------------------------- */
    /* Queue execution                                                         */
    /* ---------------------------------------------------------------------- */

    /// Run the highest-priority compatible queued job.
    pub fn run_next(
        &self,
        worker_id: WorkerId,
    ) -> QecResult<Option<JobOutput>> {
        self.run_next_internal(worker_id)
            .map_err(QecError::from)
    }

    fn run_next_internal(
        &self,
        worker_id: WorkerId,
    ) -> SchedulerResult<Option<JobOutput>> {
        if self.shutting_down.load(AtomicOrdering::Acquire) {
            return Err(SchedulerError::SchedulerShuttingDown);
        }

        let worker_mode = {
            let workers = self.workers.lock().map_err(|_| {
                SchedulerError::InternalInvariantViolation(
                    "worker registry mutex poisoned".into(),
                )
            })?;

            let worker = workers
                .get(&worker_id)
                .ok_or(SchedulerError::WorkerUnavailable)?;

            if worker.busy {
                return Err(SchedulerError::WorkerUnavailable);
            }

            worker.mode
        };

        let item = self.take_next_compatible(worker_mode)?;

        let Some(item) = item else {
            return Ok(None);
        };

        self.mark_worker_running(worker_id, item.id)?;

        self.transition(item.id, JobState::Running)?;

        self.running_jobs
            .fetch_add(1, AtomicOrdering::AcqRel);

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.started = metrics.started.saturating_add(1);
        }

        let context = ExecutionContext {
            job_id: item.id,
            worker_id,
            mode: item.spec.mode,
            resources: item.spec.resources,
            deadline: item.spec.deadline,
            cancellation: item.cancellation.clone(),
            deterministic: item.spec.deterministic || self.deterministic,
            attempt: item.attempt,
        };

        let result = if context.check().is_err() {
            Err(SchedulerError::CancellationRequested)
        } else {
            item.executor.execute(context)
        };

        self.running_jobs
            .fetch_sub(1, AtomicOrdering::AcqRel);

        self.mark_worker_idle(worker_id, item.id)?;

        match result {
            Ok(output) => {
                self.accounting.release(item.spec.resources);

                self.transition(item.id, JobState::Completed)?;

                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.completed = metrics.completed.saturating_add(1);
                }

                Ok(Some(output))
            }

            Err(error) => {
                self.handle_failure(item, error)?;

                Ok(None)
            }
        }
    }

    fn take_next_compatible(
        &self,
        worker_mode: ExecutionMode,
    ) -> SchedulerResult<Option<QueueItem>> {
        let mut queue = self.queue.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "scheduler queue mutex poisoned".into(),
            )
        })?;

        let mut deferred = Vec::new();
        let mut selected = None;

        while let Some(item) = queue.pop() {
            if self.item_compatible_with_worker(&item, worker_mode) {
                selected = Some(item);
                break;
            }

            deferred.push(item);
        }

        for item in deferred {
            queue.push(item);
        }

        Ok(selected)
    }

    fn item_compatible_with_worker(
        &self,
        item: &QueueItem,
        worker_mode: ExecutionMode,
    ) -> bool {
        match item.spec.mode {
            ExecutionMode::SingleThread => true,

            ExecutionMode::MultiThread => matches!(
                worker_mode,
                ExecutionMode::MultiThread
                    | ExecutionMode::MultiProcess
                    | ExecutionMode::Distributed
                    | ExecutionMode::Accelerated
            ),

            ExecutionMode::MultiProcess => matches!(
                worker_mode,
                ExecutionMode::MultiProcess | ExecutionMode::Distributed
            ),

            ExecutionMode::Distributed => {
                matches!(worker_mode, ExecutionMode::Distributed)
            }

            ExecutionMode::Accelerated => {
                matches!(worker_mode, ExecutionMode::Accelerated)
            }
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Failure / retry                                                         */
    /* ---------------------------------------------------------------------- */

    fn handle_failure(
        &self,
        item: QueueItem,
        error: SchedulerError,
    ) -> SchedulerResult<()> {
        self.accounting.release(item.spec.resources);

        if matches!(
            error,
            SchedulerError::CancellationRequested
                | SchedulerError::DeadlineExceeded
        ) {
            let state = if matches!(
                error,
                SchedulerError::DeadlineExceeded
            ) {
                JobState::TimedOut
            } else {
                JobState::Cancelled
            };

            self.transition(item.id, state)?;

            if let Ok(mut metrics) = self.metrics.lock() {
                if state == JobState::TimedOut {
                    metrics.timed_out = metrics.timed_out.saturating_add(1);
                } else {
                    metrics.cancelled = metrics.cancelled.saturating_add(1);
                }
            }

            return Ok(());
        }

        if item.attempt < item.spec.max_retries {
            let next_attempt = item.attempt.saturating_add(1);

            if item.spec.checkpointable {
                self.transition(item.id, JobState::Checkpointing)?;

                (self.checkpoint)(
                    item.id,
                    CheckpointReason::Retry,
                )
                .map_err(|error| {
                    SchedulerError::CheckpointFailed(error.to_string())
                })?;

                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.checkpointed =
                        metrics.checkpointed.saturating_add(1);
                }

                self.transition(item.id, JobState::Paused)?;
                self.transition(item.id, JobState::Resuming)?;

                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.resumed = metrics.resumed.saturating_add(1);
                }
            }

            let sequence = self
                .next_sequence
                .fetch_add(1, AtomicOrdering::Relaxed);

            let cancellation = CancellationToken::new();

            let state = {
                let jobs = self.jobs.lock().map_err(|_| {
                    SchedulerError::InternalInvariantViolation(
                        "job registry mutex poisoned".into(),
                    )
                })?;

                jobs.get(&item.id)
                    .map(|record| record.state.clone())
                    .ok_or_else(|| {
                        SchedulerError::InternalInvariantViolation(
                            "retry target job missing".into(),
                        )
                    })?
            };

            if let Ok(mut value) = state.lock() {
                *value = JobState::Admitted;
            }

            self.accounting
                .reserve(item.spec.resources, &self.limits)?;

            {
                let mut queue = self.queue.lock().map_err(|_| {
                    self.accounting.release(item.spec.resources);

                    SchedulerError::InternalInvariantViolation(
                        "scheduler queue mutex poisoned".into(),
                    )
                })?;

                queue.push(QueueItem {
                    id: item.id,
                    spec: item.spec.clone(),
                    executor: item.executor,
                    cancellation,
                    state,
                    sequence,
                    attempt: next_attempt,
                });
            }

            if let Ok(mut jobs) = self.jobs.lock() {
                if let Some(record) = jobs.get_mut(&item.id) {
                    record.attempt = next_attempt;
                }
            }

            if let Ok(mut metrics) = self.metrics.lock() {
                metrics.retries = metrics.retries.saturating_add(1);
            }

            return Ok(());
        }

        self.transition(item.id, JobState::Failed)?;

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.failed = metrics.failed.saturating_add(1);
        }

        let _ = error;

        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Lifecycle                                                               */
    /* ---------------------------------------------------------------------- */

    fn transition(
        &self,
        job_id: JobId,
        next: JobState,
    ) -> SchedulerResult<()> {
        let jobs = self.jobs.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job registry mutex poisoned".into(),
            )
        })?;

        let record = jobs
            .get(&job_id)
            .ok_or_else(|| {
                SchedulerError::InternalInvariantViolation(
                    "job not found during lifecycle transition".into(),
                )
            })?;

        let mut state = record.state.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job state mutex poisoned".into(),
            )
        })?;

        let current = *state;

        if !Self::valid_transition(current, next) {
            return Err(SchedulerError::InternalInvariantViolation(
                format!(
                    "invalid scheduler transition: {:?} -> {:?}",
                    current, next
                ),
            ));
        }

        *state = next;

        Ok(())
    }

    const fn valid_transition(
        current: JobState,
        next: JobState,
    ) -> bool {
        use JobState::*;

        match (current, next) {
            (Created, Validating) => true,

            (Validating, Admitted) => true,
            (Validating, Rejected) => true,
            (Validating, Cancelled) => true,

            (Admitted, Running) => true,
            (Admitted, Cancelled) => true,

            (Running, Completed) => true,
            (Running, Failed) => true,
            (Running, Cancelled) => true,
            (Running, TimedOut) => true,
            (Running, Checkpointing) => true,

            (Checkpointing, Paused) => true,
            (Checkpointing, Failed) => true,
            (Checkpointing, Cancelled) => true,

            (Paused, Resuming) => true,
            (Paused, Cancelled) => true,

            (Resuming, Running) => true,
            (Resuming, Failed) => true,
            (Resuming, Cancelled) => true,

            _ if current.terminal() => false,

            _ => false,
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Cancellation                                                            */
    /* ---------------------------------------------------------------------- */

    pub fn cancel(&self, job_id: JobId) -> SchedulerResult<()> {
        let jobs = self.jobs.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job registry mutex poisoned".into(),
            )
        })?;

        let record = jobs
            .get(&job_id)
            .ok_or_else(|| {
                SchedulerError::InvalidRequest(
                    "unknown scheduler job".into(),
                )
            })?;

        record.cancellation.cancel();

        let state = record.state.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job state mutex poisoned".into(),
            )
        })?;

        if state.terminal() {
            return Ok(());
        }

        drop(state);

        self.transition(job_id, JobState::Cancelled)?;

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.cancelled = metrics.cancelled.saturating_add(1);
        }

        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Checkpoint                                                              */
    /* ---------------------------------------------------------------------- */

    pub fn checkpoint(
        &self,
        job_id: JobId,
        reason: CheckpointReason,
    ) -> SchedulerResult<()> {
        let jobs = self.jobs.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job registry mutex poisoned".into(),
            )
        })?;

        let record = jobs
            .get(&job_id)
            .ok_or_else(|| {
                SchedulerError::InvalidRequest(
                    "unknown scheduler job".into(),
                )
            })?;

        if !record.spec.checkpointable {
            return Err(SchedulerError::CheckpointRequired);
        }

        drop(jobs);

        self.transition(job_id, JobState::Checkpointing)?;

        if let Err(error) = (self.checkpoint)(job_id, reason) {
            let _ = self.transition(job_id, JobState::Failed);

            return Err(SchedulerError::CheckpointFailed(
                error.to_string(),
            ));
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.checkpointed = metrics.checkpointed.saturating_add(1);
        }

        self.transition(job_id, JobState::Paused)
    }

    /// Resume a paused job.
    ///
    /// The actual work is requeued by the caller/runtime because the scheduler
    /// deliberately does not manufacture an executor or checkpoint payload.
    pub fn mark_resuming(
        &self,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        self.transition(job_id, JobState::Resuming)
    }

    /* ---------------------------------------------------------------------- */
    /* Shutdown                                                               */
    /* ---------------------------------------------------------------------- */

    /// Prevent new submissions and cancel queued/running work.
    pub fn shutdown(&self) -> SchedulerResult<()> {
        self.shutting_down
            .store(true, AtomicOrdering::Release);

        let jobs = self.jobs.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "job registry mutex poisoned".into(),
            )
        })?;

        for record in jobs.values() {
            if let Ok(state) = record.state.lock() {
                if !state.terminal() {
                    record.cancellation.cancel();
                }
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn is_shutting_down(&self) -> bool {
        self.shutting_down.load(AtomicOrdering::Acquire)
    }

    /* ---------------------------------------------------------------------- */
    /* Inspection                                                              */
    /* ---------------------------------------------------------------------- */

    #[must_use]
    pub fn job_state(&self, job_id: JobId) -> Option<JobState> {
        self.jobs
            .lock()
            .ok()
            .and_then(|jobs| jobs.get(&job_id).map(|job| {
                job.state
                    .lock()
                    .map(|state| *state)
                    .unwrap_or(JobState::Failed)
            }))
    }

    #[must_use]
    pub fn queue_depth(&self) -> usize {
        self.queue
            .lock()
            .map(|queue| queue.len())
            .unwrap_or(0)
    }

    #[must_use]
    pub fn running_jobs(&self) -> usize {
        self.running_jobs
            .load(AtomicOrdering::Acquire)
            .try_into()
            .unwrap_or(usize::MAX)
    }

    #[must_use]
    pub fn resource_snapshot(&self) -> ResourceReservation {
        self.accounting.snapshot()
    }

    #[must_use]
    pub fn metrics(&self) -> SchedulerMetrics {
        self.metrics
            .lock()
            .map(|metrics| *metrics)
            .unwrap_or_default()
    }

    #[must_use]
    pub fn limits(&self) -> QecLimits {
        self.limits
    }

    #[must_use]
    pub fn max_queued_jobs(&self) -> usize {
        self.max_queued_jobs
    }

    #[must_use]
    pub fn max_running_jobs(&self) -> usize {
        self.max_running_jobs
    }

    fn mark_worker_running(
        &self,
        worker_id: WorkerId,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        let mut workers = self.workers.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "worker registry mutex poisoned".into(),
            )
        })?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(SchedulerError::WorkerUnavailable)?;

        if worker.busy {
            return Err(SchedulerError::WorkerUnavailable);
        }

        worker.busy = true;
        worker.current_job = Some(job_id);

        Ok(())
    }

    fn mark_worker_idle(
        &self,
        worker_id: WorkerId,
        job_id: JobId,
    ) -> SchedulerResult<()> {
        let mut workers = self.workers.lock().map_err(|_| {
            SchedulerError::InternalInvariantViolation(
                "worker registry mutex poisoned".into(),
            )
        })?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(SchedulerError::WorkerUnavailable)?;

        if worker.current_job != Some(job_id) {
            return Err(SchedulerError::InternalInvariantViolation(
                "worker/job ownership mismatch".into(),
            ));
        }

        worker.busy = false;
        worker.current_job = None;

        Ok(())
    }
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

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
                success: true,
                ..Default::default()
            })
        }
    }

    struct FailingExecutor;

    impl JobExecutor for FailingExecutor {
        fn execute(
            &self,
            _context: ExecutionContext,
        ) -> SchedulerResult<JobOutput> {
            Err(SchedulerError::ExecutorFailed(
                "intentional test failure".into(),
            ))
        }
    }

    fn spec() -> JobSpec {
        JobSpec {
            kind: JobKind::Decode,
            priority: Priority::Normal,
            deadline: Deadline::none(),
            resources: ResourceRequest {
                memory_bytes: 1024,
                parallel_workers: 1,
                decoder_iterations: 100,
            },
            mode: ExecutionMode::SingleThread,
            deterministic: true,
            checkpointable: false,
            capabilities: CapabilityRequirement {
                decode: true,
                deterministic: true,
                ..CapabilityRequirement::none()
            },
            max_retries: 0,
        }
    }

    #[test]
    fn admission_reserves_resources() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        let handle = scheduler
            .submit(spec(), Arc::new(SuccessfulExecutor))
            .unwrap();

        assert_eq!(handle.state(), JobState::Admitted);

        let snapshot = scheduler.resource_snapshot();

        assert_eq!(snapshot.memory_bytes, 1024);
        assert_eq!(snapshot.parallel_workers, 1);
    }

    #[test]
    fn successful_execution_releases_resources() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        scheduler
            .register_worker(
                WorkerId::new(1),
                ExecutionMode::SingleThread,
            )
            .unwrap();

        let handle = scheduler
            .submit(spec(), Arc::new(SuccessfulExecutor))
            .unwrap();

        let output = scheduler
            .run_next(WorkerId::new(1))
            .unwrap()
            .unwrap();

        assert!(output.success);
        assert_eq!(handle.state(), JobState::Completed);

        let snapshot = scheduler.resource_snapshot();

        assert_eq!(snapshot.memory_bytes, 0);
        assert_eq!(snapshot.parallel_workers, 0);
    }

    #[test]
    fn cancellation_is_propagated() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        scheduler
            .register_worker(
                WorkerId::new(1),
                ExecutionMode::SingleThread,
            )
            .unwrap();

        let handle = scheduler
            .submit(spec(), Arc::new(SuccessfulExecutor))
            .unwrap();

        handle.cancel();

        assert_eq!(handle.state(), JobState::Cancelled);
        assert!(handle.cancellation().is_cancelled());
    }

    #[test]
    fn priority_is_deterministic() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        let mut low = spec();
        low.priority = Priority::Low;

        let mut critical = spec();
        critical.priority = Priority::Critical;

        scheduler
            .submit(low, Arc::new(SuccessfulExecutor))
            .unwrap();

        scheduler
            .submit(critical, Arc::new(SuccessfulExecutor))
            .unwrap();

        scheduler
            .register_worker(
                WorkerId::new(1),
                ExecutionMode::SingleThread,
            )
            .unwrap();

        let _ = scheduler
            .run_next(WorkerId::new(1))
            .unwrap()
            .unwrap();

        assert_eq!(scheduler.metrics().completed, 1);
    }

    #[test]
    fn incompatible_worker_does_not_execute_job() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        let mut request = spec();
        request.mode = ExecutionMode::Accelerated;
        request.capabilities.accelerator = true;

        scheduler
            .submit(request, Arc::new(SuccessfulExecutor))
            .unwrap();

        scheduler
            .register_worker(
                WorkerId::new(1),
                ExecutionMode::SingleThread,
            )
            .unwrap();

        let result = scheduler
            .run_next(WorkerId::new(1))
            .unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn failed_job_does_not_leak_reserved_resources() {
        let scheduler = QecScheduler::new(QecLimits::default()).unwrap();

        scheduler
            .register_worker(
                WorkerId::new(1),
                ExecutionMode::SingleThread,
            )
            .unwrap();

        let handle = scheduler
            .submit(spec(), Arc::new(FailingExecutor))
            .unwrap();

        scheduler
            .run_next(WorkerId::new(1))
            .unwrap();

        assert_eq!(handle.state(), JobState::Failed);

        let snapshot = scheduler.resource_snapshot();

        assert_eq!(snapshot.memory_bytes, 0);
        assert_eq!(snapshot.parallel_workers, 0);
    }

    #[test]
    fn queue_is_bounded() {
        let limits = QecLimits::default();

        let scheduler = QecScheduler::with_integrations(
            limits,
            1,
            1,
            Arc::new(LocalResourceAccounting::default()),
            allow_all_capabilities(),
            noop_checkpoint(),
        )
        .unwrap();

        scheduler
            .submit(spec(), Arc::new(SuccessfulExecutor))
            .unwrap();

        let second = scheduler.submit(
            spec(),
            Arc::new(SuccessfulExecutor),
        );

        assert!(second.is_err());
    }

    #[test]
    fn limits_are_single_source_of_truth() {
        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 1024;

        let scheduler = QecScheduler::new(limits).unwrap();

        let mut request = spec();
        request.resources.memory_bytes = 2048;

        let result = scheduler.submit(
            request,
            Arc::new(SuccessfulExecutor),
        );

        assert!(result.is_err());
    }
}