//! Distributed Quantum Error Correction infrastructure.
//!
//! This module provides the coordination layer for partitioned QEC workloads.
//!
//! Architectural contract:
//!
//! ```text
//! QecConfig / validated policy
//!          |
//!          v
//!   DistributedCoordinator
//!          |
//!   +------+-------+
//!   |              |
//!   v              v
//! Resource       Capability
//! preflight      authorization
//!   |              |
//!   +------+-------+
//!          |
//!          v
//!   Deterministic planner
//!          |
//!          v
//!      PartitionTask
//!          |
//!          v
//!   Authenticated worker
//!          |
//!          v
//!    WorkerResult
//!          |
//!   +------+-------+
//!   |              |
//!   v              v
//! Integrity      Idempotency
//! validation     validation
//!   |              |
//!   +------+-------+
//!          |
//!          v
//! Boundary reconciliation
//!          |
//!          v
//!   Global logical result
//! ```
//!
//! The distributed layer is deliberately independent of a particular decoder.
//! MWPM, Union-Find, sparse decoders, accelerators, or future decoders can use
//! the infrastructure through [`DistributedDecoder`].
//!
//! Distributed classical decoding and QPU execution are intentionally separate.
//! Possessing distributed-execution authority does not imply QPU submission
//! authority.
//!
//! "Infinite scalability" is not promised. Arbitrarily large workloads are
//! supported only subject to explicit resource limits, partitioning, bounded
//! queues, cancellation, checkpointing, retry policy, and graceful failure.
//!
//! # Safety properties
//!
//! - No unsafe code.
//! - Checked arithmetic at resource boundaries.
//! - Bounded task queues.
//! - Explicit worker lifecycle.
//! - Explicit worker authentication state.
//! - Capability separation.
//! - Idempotent task identity.
//! - Duplicate-result suppression.
//! - Deterministic ordering.
//! - Explicit partition ownership.
//! - Explicit boundary contracts.
//! - Fail-closed worker validation.
//! - Cooperative cancellation.
//! - Job deadlines.
//! - Retry limits.
//! - No implicit QPU authority.
//!
//! # Integration
//!
//! `DistributedLimits` remains in this module as a compatibility policy for
//! existing callers. In the fully integrated configuration path it should be
//! derived from the validated QEC configuration/resource policy rather than
//! becoming a second independent global policy system.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

/// Stable identifier for a distributed QEC job.
pub type JobId = u64;

/// Stable identifier for a partition.
pub type PartitionId = u64;

/// Stable identifier for a worker.
pub type WorkerId = u64;

/// Identifier for a syndrome/detection event.
pub type EventId = u64;

/// Identifier for a boundary event.
pub type BoundaryId = u64;

/// Stable identifier for a task attempt.
pub type TaskAttempt = u32;

/// Numeric graph weight.
pub type Weight = f64;

/// Result type used by the distributed subsystem.
pub type Result<T> = std::result::Result<T, DistributedError>;

/// Current distributed task/result contract version.
pub const DISTRIBUTED_FORMAT_VERSION: u32 = 2;

/// Errors produced by the distributed QEC infrastructure.
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedError {
    InvalidInput(String),
    InvalidPartition(String),
    InvalidWorker(String),
    InvalidBoundary(String),
    InvalidJob(String),

    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u64,
        limit: u64,
    },

    WorkerFailure {
        worker_id: WorkerId,
        message: String,
    },

    WorkerUnavailable(WorkerId),

    WorkerUnauthenticated(WorkerId),

    CapabilityDenied {
        worker_id: WorkerId,
        capability: WorkerCapability,
    },

    Cancelled,
    DeadlineExceeded,

    DeterminismViolation(String),

    BoundaryReconciliationFailed(String),

    Timeout,

    InvalidWorkerOutput(String),

    IntegrityFailure(String),

    DuplicateTaskResult {
        task_key: TaskKey,
    },

    StaleTaskResult {
        task_key: TaskKey,
        expected_attempt: TaskAttempt,
        received_attempt: TaskAttempt,
    },

    RetryExhausted {
        partition_id: PartitionId,
        attempts: TaskAttempt,
    },

    PartitionOwnershipViolation {
        partition_id: PartitionId,
        expected_worker: WorkerId,
        received_worker: WorkerId,
    },

    InvariantViolation(String),

    Unsupported(String),

    IncompatibleVersion {
        expected: u32,
        found: u32,
    },

    Synchronization(String),
}

impl fmt::Display for DistributedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => {
                write!(f, "invalid distributed input: {message}")
            }

            Self::InvalidPartition(message) => {
                write!(f, "invalid partition: {message}")
            }

            Self::InvalidWorker(message) => {
                write!(f, "invalid worker: {message}")
            }

            Self::InvalidBoundary(message) => {
                write!(f, "invalid boundary: {message}")
            }

            Self::InvalidJob(message) => {
                write!(f, "invalid job: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    f,
                    "resource limit exceeded for {resource}: requested={requested}, limit={limit}"
                )
            }

            Self::WorkerFailure {
                worker_id,
                message,
            } => {
                write!(f, "worker {worker_id} failed: {message}")
            }

            Self::WorkerUnavailable(worker_id) => {
                write!(f, "worker {worker_id} unavailable")
            }

            Self::WorkerUnauthenticated(worker_id) => {
                write!(f, "worker {worker_id} is not authenticated")
            }

            Self::CapabilityDenied {
                worker_id,
                capability,
            } => {
                write!(
                    f,
                    "worker {worker_id} lacks required capability {capability:?}"
                )
            }

            Self::Cancelled => {
                write!(f, "distributed operation cancelled")
            }

            Self::DeadlineExceeded => {
                write!(f, "distributed operation deadline exceeded")
            }

            Self::DeterminismViolation(message) => {
                write!(f, "determinism violation: {message}")
            }

            Self::BoundaryReconciliationFailed(message) => {
                write!(f, "boundary reconciliation failed: {message}")
            }

            Self::Timeout => {
                write!(f, "distributed operation timed out")
            }

            Self::InvalidWorkerOutput(message) => {
                write!(f, "invalid worker output: {message}")
            }

            Self::IntegrityFailure(message) => {
                write!(f, "distributed integrity failure: {message}")
            }

            Self::DuplicateTaskResult { task_key } => {
                write!(f, "duplicate task result: {task_key:?}")
            }

            Self::StaleTaskResult {
                task_key,
                expected_attempt,
                received_attempt,
            } => {
                write!(
                    f,
                    "stale result for task {task_key:?}: expected attempt {expected_attempt}, received {received_attempt}"
                )
            }

            Self::RetryExhausted {
                partition_id,
                attempts,
            } => {
                write!(
                    f,
                    "retry policy exhausted for partition {partition_id} after {attempts} attempts"
                )
            }

            Self::PartitionOwnershipViolation {
                partition_id,
                expected_worker,
                received_worker,
            } => {
                write!(
                    f,
                    "partition {partition_id} belongs to worker {expected_worker}, \
                     but result came from worker {received_worker}"
                )
            }

            Self::InvariantViolation(message) => {
                write!(f, "distributed invariant violation: {message}")
            }

            Self::Unsupported(message) => {
                write!(f, "unsupported distributed operation: {message}")
            }

            Self::IncompatibleVersion { expected, found } => {
                write!(
                    f,
                    "incompatible distributed format: expected {expected}, found {found}"
                )
            }

            Self::Synchronization(message) => {
                write!(f, "distributed synchronization failure: {message}")
            }
        }
    }
}

impl std::error::Error for DistributedError {}

/// Resources bounded by distributed execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceKind {
    Jobs,
    Partitions,
    Workers,
    Events,
    BoundaryEvents,
    Bytes,
    InFlightTasks,
    Retries,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Jobs => "jobs",
            Self::Partitions => "partitions",
            Self::Workers => "workers",
            Self::Events => "events",
            Self::BoundaryEvents => "boundary events",
            Self::Bytes => "bytes",
            Self::InFlightTasks => "in-flight tasks",
            Self::Retries => "retries",
        };

        f.write_str(value)
    }
}

/// Compatibility resource policy for distributed execution.
///
/// Prefer deriving this policy from the validated global QEC configuration
/// rather than treating it as a second independent global resource system.
#[derive(Debug, Clone)]
pub struct DistributedLimits {
    pub max_jobs: u64,
    pub max_partitions_per_job: u64,
    pub max_workers: u64,
    pub max_events_per_partition: u64,
    pub max_boundary_events_per_partition: u64,
    pub max_in_flight_tasks: u64,
    pub max_job_bytes: u64,
    pub max_worker_time: Duration,
    pub max_job_time: Duration,

    /// Maximum retries for one partition task.
    pub max_retries_per_partition: u32,
}

impl Default for DistributedLimits {
    fn default() -> Self {
        Self {
            max_jobs: 1_024,
            max_partitions_per_job: 1_000_000,
            max_workers: 4_096,
            max_events_per_partition: 10_000_000,
            max_boundary_events_per_partition: 1_000_000,
            max_in_flight_tasks: 16_384,
            max_job_bytes: 8 * 1024 * 1024 * 1024,
            max_worker_time: Duration::from_secs(3_600),
            max_job_time: Duration::from_secs(86_400),
            max_retries_per_partition: 3,
        }
    }
}

impl DistributedLimits {
    pub fn validate(&self) -> Result<()> {
        let positive = [
            (self.max_jobs, "max_jobs"),
            (
                self.max_partitions_per_job,
                "max_partitions_per_job",
            ),
            (self.max_workers, "max_workers"),
            (
                self.max_events_per_partition,
                "max_events_per_partition",
            ),
            (
                self.max_boundary_events_per_partition,
                "max_boundary_events_per_partition",
            ),
            (
                self.max_in_flight_tasks,
                "max_in_flight_tasks",
            ),
            (self.max_job_bytes, "max_job_bytes"),
        ];

        for (value, name) in positive {
            if value == 0 {
                return Err(DistributedError::InvalidInput(format!(
                    "{name} must be greater than zero"
                )));
            }
        }

        if self.max_worker_time.is_zero() {
            return Err(DistributedError::InvalidInput(
                "max_worker_time must be greater than zero".into(),
            ));
        }

        if self.max_job_time.is_zero() {
            return Err(DistributedError::InvalidInput(
                "max_job_time must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

/// Cooperative cancellation token.
#[derive(Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    pub fn check(&self) -> Result<()> {
        if self.is_cancelled() {
            Err(DistributedError::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// Deterministic execution policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminismConfig {
    pub enabled: bool,
    pub seed: u64,
    pub stable_partition_order: bool,
    pub stable_worker_assignment: bool,
}

impl Default for DeterminismConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            seed: 0,
            stable_partition_order: true,
            stable_worker_assignment: true,
        }
    }
}

impl DeterminismConfig {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Distributed execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Local,
    MultiThreaded,
    MultiProcess,
    Distributed,
    Accelerated,
}

/// Worker lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Starting,
    Ready,
    Busy,
    Draining,
    Failed,
    Offline,
}

/// Explicit worker capabilities.
///
/// QPU submission is intentionally NOT represented by
/// `DistributedExecution`. A distributed classical worker does not
/// automatically receive hardware authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerCapability {
    Decode,
    BoundaryReconciliation,
    Checkpoint,
    Streaming,
    Accelerator,
    QpuInspect,
    QpuSubmit,
    QpuReadResults,
}

impl fmt::Display for WorkerCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Decode => "decode",
            Self::BoundaryReconciliation => "boundary-reconciliation",
            Self::Checkpoint => "checkpoint",
            Self::Streaming => "streaming",
            Self::Accelerator => "accelerator",
            Self::QpuInspect => "qpu-inspect",
            Self::QpuSubmit => "qpu-submit",
            Self::QpuReadResults => "qpu-read-results",
        };

        f.write_str(value)
    }
}

/// Worker capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerCapabilities {
    pub cpu: bool,
    pub gpu: bool,
    pub accelerator: bool,
    pub distributed_boundary_reconciliation: bool,
    pub checkpointing: bool,
    pub streaming: bool,

    /// Explicit classical decoding authority.
    pub decoding: bool,

    /// Explicit QPU inspection authority.
    pub qpu_inspect: bool,

    /// Explicit QPU submission authority.
    pub qpu_submit: bool,

    /// Explicit QPU result-read authority.
    pub qpu_read_results: bool,
}

impl Default for WorkerCapabilities {
    fn default() -> Self {
        Self {
            cpu: true,
            gpu: false,
            accelerator: false,
            distributed_boundary_reconciliation: true,
            checkpointing: true,
            streaming: true,
            decoding: true,
            qpu_inspect: false,
            qpu_submit: false,
            qpu_read_results: false,
        }
    }
}

impl WorkerCapabilities {
    pub fn has(&self, capability: WorkerCapability) -> bool {
        match capability {
            WorkerCapability::Decode => self.decoding,
            WorkerCapability::BoundaryReconciliation => {
                self.distributed_boundary_reconciliation
            }
            WorkerCapability::Checkpoint => self.checkpointing,
            WorkerCapability::Streaming => self.streaming,
            WorkerCapability::Accelerator => self.accelerator,
            WorkerCapability::QpuInspect => self.qpu_inspect,
            WorkerCapability::QpuSubmit => self.qpu_submit,
            WorkerCapability::QpuReadResults => self.qpu_read_results,
        }
    }
}

/// Authentication state for a worker.
///
/// A worker cannot execute distributed work until it is authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationState {
    Unauthenticated,
    Authenticated,
    Revoked,
}

/// Worker registration.
#[derive(Debug, Clone)]
pub struct WorkerDescriptor {
    pub id: WorkerId,
    pub name: String,
    pub state: WorkerState,
    pub capabilities: WorkerCapabilities,
    pub max_concurrent_tasks: u32,
}

impl WorkerDescriptor {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(DistributedError::InvalidWorker(
                "worker name cannot be empty".into(),
            ));
        }

        if self.max_concurrent_tasks == 0 {
            return Err(DistributedError::InvalidWorker(
                "max_concurrent_tasks must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

/// Runtime worker record.
#[derive(Debug, Clone)]
struct WorkerRuntime {
    descriptor: WorkerDescriptor,
    authentication: AuthenticationState,
    active_tasks: u32,
}

/// Detection event.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionEvent {
    pub id: EventId,
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub time: u64,
    pub weight: Weight,
}

impl DetectionEvent {
    pub fn validate(&self) -> Result<()> {
        if !self.weight.is_finite() || self.weight < 0.0 {
            return Err(DistributedError::InvalidInput(format!(
                "event {} has invalid weight {}",
                self.id, self.weight
            )));
        }

        Ok(())
    }
}

/// Explicit partition-boundary event.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryEvent {
    pub id: BoundaryId,
    pub source_partition: PartitionId,
    pub coordinate: (i64, i64, i64),
    pub time: u64,
    pub weight: Weight,
}

impl BoundaryEvent {
    pub fn validate(&self) -> Result<()> {
        if !self.weight.is_finite() || self.weight < 0.0 {
            return Err(DistributedError::InvalidBoundary(format!(
                "boundary {} has invalid weight {}",
                self.id, self.weight
            )));
        }

        Ok(())
    }
}

/// Mathematical contract at a partition boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionBoundary {
    pub partition_id: PartitionId,
    pub neighbor_id: PartitionId,

    /// Stable IDs of incoming events.
    pub incoming_events: Vec<BoundaryId>,

    /// Stable IDs of outgoing events.
    pub outgoing_events: Vec<BoundaryId>,

    /// Virtual-boundary parity.
    pub virtual_boundary_parity: u8,

    /// Correction parity crossing this boundary.
    pub correction_parity: u8,

    /// Logical parity contribution crossing the boundary.
    pub logical_parity: u8,

    /// Reconciliation protocol version.
    pub reconciliation_version: u32,
}

impl PartitionBoundary {
    pub fn validate(&self) -> Result<()> {
        if self.partition_id == self.neighbor_id {
            return Err(DistributedError::InvalidBoundary(
                "partition boundary cannot connect a partition to itself".into(),
            ));
        }

        if self.virtual_boundary_parity > 1
            || self.correction_parity > 1
            || self.logical_parity > 1
        {
            return Err(DistributedError::InvalidBoundary(
                "boundary parity must be 0 or 1".into(),
            ));
        }

        if self.reconciliation_version != DISTRIBUTED_FORMAT_VERSION {
            return Err(DistributedError::IncompatibleVersion {
                expected: DISTRIBUTED_FORMAT_VERSION,
                found: self.reconciliation_version,
            });
        }

        Ok(())
    }
}

/// A QEC partition.
#[derive(Debug, Clone)]
pub struct QecPartition {
    pub id: PartitionId,

    /// Half-open coordinate ranges.
    pub bounds: [(i64, i64); 3],

    pub events: Vec<DetectionEvent>,

    pub boundary_events: Vec<BoundaryEvent>,

    pub neighbors: BTreeSet<PartitionId>,

    pub logical_region: Option<u64>,
}

impl QecPartition {
    pub fn validate(&self, limits: &DistributedLimits) -> Result<()> {
        for (axis, (min, max)) in self.bounds.iter().enumerate() {
            if min >= max {
                return Err(DistributedError::InvalidPartition(format!(
                    "partition {} has invalid axis {} bounds [{}, {})",
                    self.id, axis, min, max
                )));
            }
        }

        let event_count = u64::try_from(self.events.len())
            .unwrap_or(u64::MAX);

        if event_count > limits.max_events_per_partition {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Events,
                requested: event_count,
                limit: limits.max_events_per_partition,
            });
        }

        let boundary_count = u64::try_from(self.boundary_events.len())
            .unwrap_or(u64::MAX);

        if boundary_count > limits.max_boundary_events_per_partition {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::BoundaryEvents,
                requested: boundary_count,
                limit: limits.max_boundary_events_per_partition,
            });
        }

        let mut event_ids = BTreeSet::new();

        for event in &self.events {
            event.validate()?;

            if !self.contains(event.x, event.y, event.z) {
                return Err(DistributedError::InvalidPartition(format!(
                    "event {} is outside partition {}",
                    event.id, self.id
                )));
            }

            if !event_ids.insert(event.id) {
                return Err(DistributedError::InvalidPartition(format!(
                    "duplicate event id {} in partition {}",
                    event.id, self.id
                )));
            }
        }

        let mut boundary_ids = BTreeSet::new();

        for boundary in &self.boundary_events {
            boundary.validate()?;

            if boundary.source_partition != self.id {
                return Err(DistributedError::InvalidBoundary(format!(
                    "boundary {} belongs to partition {}, not {}",
                    boundary.id,
                    boundary.source_partition,
                    self.id
                )));
            }

            if !boundary_ids.insert(boundary.id) {
                return Err(DistributedError::InvalidBoundary(format!(
                    "duplicate boundary id {} in partition {}",
                    boundary.id, self.id
                )));
            }
        }

        if self.neighbors.contains(&self.id) {
            return Err(DistributedError::InvalidPartition(format!(
                "partition {} cannot be its own neighbor",
                self.id
            )));
        }

        Ok(())
    }

    pub fn contains(&self, x: i64, y: i64, z: i64) -> bool {
        let [xb, yb, zb] = self.bounds;

        x >= xb.0
            && x < xb.1
            && y >= yb.0
            && y < yb.1
            && z >= zb.0
            && z < zb.1
    }

    pub fn estimated_event_bytes(&self) -> u64 {
        let events = u64::try_from(self.events.len())
            .unwrap_or(u64::MAX);

        let boundaries = u64::try_from(self.boundary_events.len())
            .unwrap_or(u64::MAX);

        events
            .saturating_mul(48)
            .saturating_add(boundaries.saturating_mul(48))
    }
}

/// Complete distributed job.
#[derive(Debug, Clone)]
pub struct DistributedJob {
    pub id: JobId,
    pub partitions: Vec<QecPartition>,
    pub mode: ExecutionMode,
    pub determinism: DeterminismConfig,
    pub metadata: BTreeMap<String, String>,
}

impl DistributedJob {
    pub fn validate(&self, limits: &DistributedLimits) -> Result<()> {
        if self.id == 0 {
            return Err(DistributedError::InvalidJob(
                "job id must be non-zero".into(),
            ));
        }

        self.determinism.validate()?;

        if self.partitions.is_empty() {
            return Err(DistributedError::InvalidJob(
                "job must contain at least one partition".into(),
            ));
        }

        let count = u64::try_from(self.partitions.len())
            .unwrap_or(u64::MAX);

        if count > limits.max_partitions_per_job {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Partitions,
                requested: count,
                limit: limits.max_partitions_per_job,
            });
        }

        let mut partition_ids = BTreeSet::new();
        let mut estimated_bytes = 0u64;

        for partition in &self.partitions {
            partition.validate(limits)?;

            if !partition_ids.insert(partition.id) {
                return Err(DistributedError::InvalidJob(format!(
                    "duplicate partition id {}",
                    partition.id
                )));
            }

            estimated_bytes = estimated_bytes
                .checked_add(partition.estimated_event_bytes())
                .ok_or(DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Bytes,
                    requested: u64::MAX,
                    limit: limits.max_job_bytes,
                })?;
        }

        if estimated_bytes > limits.max_job_bytes {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Bytes,
                requested: estimated_bytes,
                limit: limits.max_job_bytes,
            });
        }

        self.validate_neighbors()?;

        Ok(())
    }

    fn validate_neighbors(&self) -> Result<()> {
        let ids: BTreeSet<_> =
            self.partitions.iter().map(|p| p.id).collect();

        let partitions: BTreeMap<_, _> =
            self.partitions.iter().map(|p| (p.id, p)).collect();

        for partition in &self.partitions {
            for neighbor in &partition.neighbors {
                if !ids.contains(neighbor) {
                    return Err(DistributedError::InvalidJob(format!(
                        "partition {} references unknown neighbor {}",
                        partition.id, neighbor
                    )));
                }

                let reverse = partitions
                    .get(neighbor)
                    .map(|p| p.neighbors.contains(&partition.id))
                    .unwrap_or(false);

                if !reverse {
                    return Err(DistributedError::InvalidJob(format!(
                        "neighbor relation {} -> {} is not symmetric",
                        partition.id, neighbor
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Stable task identity.
///
/// The same task key must never be treated as a new task merely because it
/// was retried or received from another worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskKey {
    pub job_id: JobId,
    pub partition_id: PartitionId,
}

impl fmt::Display for TaskKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.job_id, self.partition_id)
    }
}

/// Task lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Created,
    Queued,
    Assigned,
    Running,
    Completed,
    Retrying,
    Failed,
    Cancelled,
}

/// Unit of distributed work.
#[derive(Debug, Clone)]
pub struct PartitionTask {
    pub job_id: JobId,
    pub partition: QecPartition,
    pub determinism: DeterminismConfig,

    /// Attempt starts at one.
    pub attempt: TaskAttempt,
}

impl PartitionTask {
    pub fn key(&self) -> TaskKey {
        TaskKey {
            job_id: self.job_id,
            partition_id: self.partition.id,
        }
    }

    pub fn validate(&self, limits: &DistributedLimits) -> Result<()> {
        if self.job_id == 0 {
            return Err(DistributedError::InvalidJob(
                "task job id must be non-zero".into(),
            ));
        }

        if self.attempt == 0 {
            return Err(DistributedError::InvalidInput(
                "task attempt must start at one".into(),
            ));
        }

        self.partition.validate(limits)?;
        self.determinism.validate()?;

        Ok(())
    }
}

/// Worker execution context.
#[derive(Clone)]
pub struct WorkerContext {
    pub job_id: JobId,
    pub worker_id: WorkerId,
    pub cancellation: CancellationToken,
    pub started_at: Instant,
    pub deadline: Instant,
    pub determinism: DeterminismConfig,
    pub attempt: TaskAttempt,
}

impl WorkerContext {
    pub fn check(&self) -> Result<()> {
        self.cancellation.check()?;

        if Instant::now() > self.deadline {
            return Err(DistributedError::DeadlineExceeded);
        }

        Ok(())
    }
}

/// Partition correction produced by a decoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionCorrection {
    pub partition_id: PartitionId,

    /// Opaque correction operation represented as qubit/op pairs.
    pub corrections: Vec<(u64, u8)>,

    pub logical_parity: u8,

    pub resolved_boundaries: Vec<BoundaryId>,
}

impl PartitionCorrection {
    pub fn validate(&self) -> Result<()> {
        if self.logical_parity > 1 {
            return Err(DistributedError::InvalidWorkerOutput(format!(
                "partition {} returned invalid logical parity {}",
                self.partition_id, self.logical_parity
            )));
        }

        let mut qubits = BTreeSet::new();

        for (qubit, operation) in &self.corrections {
            if *operation > 3 {
                return Err(DistributedError::InvalidWorkerOutput(
                    format!(
                        "partition {} returned invalid correction operation {} for qubit {}",
                        self.partition_id,
                        operation,
                        qubit
                    ),
                ));
            }

            if !qubits.insert(*qubit) {
                return Err(DistributedError::InvalidWorkerOutput(
                    format!(
                        "partition {} returned duplicate correction for qubit {}",
                        self.partition_id,
                        qubit
                    ),
                ));
            }
        }

        let mut boundaries = BTreeSet::new();

        for boundary in &self.resolved_boundaries {
            if !boundaries.insert(*boundary) {
                return Err(DistributedError::InvalidWorkerOutput(
                    format!(
                        "partition {} returned duplicate boundary {}",
                        self.partition_id,
                        boundary
                    ),
                ));
            }
        }

        Ok(())
    }
}

/// Result envelope returned by a worker.
///
/// The envelope makes worker identity, task identity and attempt explicit.
/// This is required for idempotent retry handling.
#[derive(Debug, Clone)]
pub struct WorkerResult {
    pub task_key: TaskKey,
    pub worker_id: WorkerId,
    pub attempt: TaskAttempt,
    pub correction: PartitionCorrection,

    /// Deterministic integrity value over the result contract.
    pub integrity: u64,
}

impl WorkerResult {
    pub fn compute_integrity(&self) -> u64 {
        let mut hash = FNV_OFFSET;

        hash = hash_u64(hash, self.task_key.job_id);
        hash = hash_u64(hash, self.task_key.partition_id);
        hash = hash_u64(hash, self.worker_id);
        hash = hash_u64(hash, u64::from(self.attempt));

        hash = hash_u64(
            hash,
            u64::from(self.correction.logical_parity),
        );

        for (qubit, operation) in &self.correction.corrections {
            hash = hash_u64(hash, *qubit);
            hash = hash_u64(hash, u64::from(*operation));
        }

        for boundary in &self.correction.resolved_boundaries {
            hash = hash_u64(hash, *boundary);
        }

        hash
    }

    pub fn validate(&self) -> Result<()> {
        self.correction.validate()?;

        if self.attempt == 0 {
            return Err(DistributedError::InvalidWorkerOutput(
                "worker result has zero attempt".into(),
            ));
        }

        let expected = self.compute_integrity();

        if expected != self.integrity {
            return Err(DistributedError::IntegrityFailure(format!(
                "task {} integrity mismatch",
                self.task_key
            )));
        }

        Ok(())
    }
}

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn hash_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

/// Global distributed result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedResult {
    pub job_id: JobId,
    pub corrections: Vec<PartitionCorrection>,
    pub logical_parity: u8,
    pub reconciled_boundaries: Vec<BoundaryId>,
    pub failed_partitions: Vec<PartitionId>,
}

impl DistributedResult {
    pub fn validate(&self) -> Result<()> {
        if self.logical_parity > 1 {
            return Err(DistributedError::InvariantViolation(
                "global logical parity must be 0 or 1".into(),
            ));
        }

        let mut partitions = BTreeSet::new();

        for correction in &self.corrections {
            correction.validate()?;

            if !partitions.insert(correction.partition_id) {
                return Err(DistributedError::InvariantViolation(
                    format!(
                        "duplicate correction for partition {}",
                        correction.partition_id
                    ),
                ));
            }
        }

        let mut boundaries = BTreeSet::new();

        for boundary in &self.reconciled_boundaries {
            if !boundaries.insert(*boundary) {
                return Err(DistributedError::InvariantViolation(
                    format!(
                        "duplicate reconciled boundary {}",
                        boundary
                    ),
                ));
            }
        }

        Ok(())
    }
}

/// Distributed decoder worker interface.
///
/// Implementations may wrap MWPM, Union-Find, sparse decoders, GPU
/// decoders, accelerator backends, processes, or authenticated RPC workers.
pub trait DistributedDecoder: Send + Sync {
    fn decode(
        &self,
        task: PartitionTask,
        context: WorkerContext,
    ) -> Result<PartitionCorrection>;
}

/// Infrastructure-only decoder.
#[derive(Default)]
pub struct IdentityDecoder;

impl DistributedDecoder for IdentityDecoder {
    fn decode(
        &self,
        task: PartitionTask,
        context: WorkerContext,
    ) -> Result<PartitionCorrection> {
        context.check()?;

        Ok(PartitionCorrection {
            partition_id: task.partition.id,
            corrections: Vec::new(),
            logical_parity: 0,
            resolved_boundaries: task
                .partition
                .boundary_events
                .iter()
                .map(|boundary| boundary.id)
                .collect(),
        })
    }
}

/// Coordinator metrics.
#[derive(Debug, Clone, Default)]
pub struct DistributedMetrics {
    pub jobs_submitted: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub jobs_cancelled: u64,

    pub partitions_processed: u64,
    pub partitions_failed: u64,

    pub boundary_reconciliations: u64,
    pub boundary_conflicts: u64,

    pub worker_failures: u64,
    pub worker_retries: u64,

    pub duplicate_results: u64,
    pub integrity_failures: u64,

    pub peak_in_flight_tasks: u64,

    pub total_events_processed: u64,
    pub total_boundaries_processed: u64,

    pub total_wall_time_ns: u64,
}

impl DistributedMetrics {
    fn record_peak(&mut self, value: u64) {
        self.peak_in_flight_tasks =
            self.peak_in_flight_tasks.max(value);
    }
}

/// Internal task record.
#[derive(Debug, Clone)]
struct TaskRuntime {
    state: TaskState,
    assigned_worker: WorkerId,
    attempt: TaskAttempt,
}

/// Coordinator for distributed QEC.
pub struct DistributedCoordinator {
    limits: DistributedLimits,

    workers: Mutex<BTreeMap<WorkerId, WorkerRuntime>>,

    metrics: Mutex<DistributedMetrics>,

    active_jobs: Mutex<BTreeSet<JobId>>,

    tasks: Mutex<BTreeMap<TaskKey, TaskRuntime>>,

    completed_results: Mutex<BTreeSet<TaskKey>>,

    next_job_id: AtomicU64,

    next_worker_id: AtomicU64,
}

impl DistributedCoordinator {
    pub fn new(limits: DistributedLimits) -> Result<Self> {
        limits.validate()?;

        Ok(Self {
            limits,
            workers: Mutex::new(BTreeMap::new()),
            metrics: Mutex::new(DistributedMetrics::default()),
            active_jobs: Mutex::new(BTreeSet::new()),
            tasks: Mutex::new(BTreeMap::new()),
            completed_results: Mutex::new(BTreeSet::new()),
            next_job_id: AtomicU64::new(1),
            next_worker_id: AtomicU64::new(1),
        })
    }

    pub fn limits(&self) -> &DistributedLimits {
        &self.limits
    }

    /// Register a worker.
    ///
    /// Workers start unauthenticated and cannot execute work until
    /// [`Self::authenticate_worker`] is called.
    pub fn register_worker(
        &self,
        mut descriptor: WorkerDescriptor,
    ) -> Result<WorkerId> {
        descriptor.validate()?;

        let mut workers = self.lock_workers()?;

        if u64::try_from(workers.len()).unwrap_or(u64::MAX)
            >= self.limits.max_workers
        {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Workers,
                requested: u64::try_from(workers.len())
                    .unwrap_or(u64::MAX)
                    .saturating_add(1),
                limit: self.limits.max_workers,
            });
        }

        let id = if descriptor.id == 0 {
            self.next_worker_id.fetch_add(1, Ordering::Relaxed)
        } else {
            descriptor.id
        };

        if workers.contains_key(&id) {
            return Err(DistributedError::InvalidWorker(format!(
                "worker id {} is already registered",
                id
            )));
        }

        descriptor.id = id;
        descriptor.state = WorkerState::Starting;

        workers.insert(
            id,
            WorkerRuntime {
                descriptor,
                authentication: AuthenticationState::Unauthenticated,
                active_tasks: 0,
            },
        );

        Ok(id)
    }

    /// Authenticate a worker.
    ///
    /// The coordinator intentionally does not implement cryptography itself.
    /// Authentication must be established by the surrounding secure transport
    /// or connector and explicitly asserted here.
    pub fn authenticate_worker(
        &self,
        worker_id: WorkerId,
    ) -> Result<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(worker_id))?;

        if worker.authentication == AuthenticationState::Revoked {
            return Err(DistributedError::InvalidWorker(
                "revoked worker cannot be re-authenticated".into(),
            ));
        }

        worker.authentication = AuthenticationState::Authenticated;
        worker.descriptor.state = WorkerState::Ready;

        Ok(())
    }

    /// Revoke a worker.
    pub fn revoke_worker(
        &self,
        worker_id: WorkerId,
    ) -> Result<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(worker_id))?;

        worker.authentication = AuthenticationState::Revoked;
        worker.descriptor.state = WorkerState::Offline;

        Ok(())
    }

    pub fn unregister_worker(
        &self,
        worker_id: WorkerId,
    ) -> Result<()> {
        let mut workers = self.lock_workers()?;

        match workers.remove(&worker_id) {
            Some(_) => Ok(()),
            None => Err(DistributedError::WorkerUnavailable(worker_id)),
        }
    }

    pub fn workers(&self) -> Result<Vec<WorkerDescriptor>> {
        let workers = self.lock_workers()?;

        Ok(workers
            .values()
            .map(|worker| worker.descriptor.clone())
            .collect())
    }

    pub fn allocate_job_id(&self) -> Result<JobId> {
        let mut active = self.lock_active_jobs()?;

        if u64::try_from(active.len()).unwrap_or(u64::MAX)
            >= self.limits.max_jobs
        {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Jobs,
                requested: u64::try_from(active.len())
                    .unwrap_or(u64::MAX)
                    .saturating_add(1),
                limit: self.limits.max_jobs,
            });
        }

        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);

        if id == 0 {
            return Err(DistributedError::InvariantViolation(
                "job identifier wrapped to zero".into(),
            ));
        }

        active.insert(id);

        Ok(id)
    }

    pub fn release_job(&self, job_id: JobId) -> Result<()> {
        let mut active = self.lock_active_jobs()?;
        active.remove(&job_id);
        Ok(())
    }

    pub fn metrics(&self) -> Result<DistributedMetrics> {
        Ok(self.lock_metrics()?.clone())
    }

    /// Execute a distributed decoding job.
    ///
    /// The implementation is deliberately deterministic and bounded. It
    /// currently performs scheduling sequentially; the task/result contract is
    /// designed so a future scheduler can safely replace this with concurrent
    /// or remote execution.
    pub fn execute<D: DistributedDecoder>(
        &self,
        job: DistributedJob,
        decoder: &D,
        cancellation: CancellationToken,
    ) -> Result<DistributedResult> {
        job.validate(&self.limits)?;

        cancellation.check()?;

        let started = Instant::now();

        self.record_job_submitted()?;

        let _job_guard =
            ActiveJobGuard::new(self, job.id)?;

        let worker_id =
            self.select_worker(&job.determinism)?;

        self.require_worker_capability(
            worker_id,
            WorkerCapability::Decode,
        )?;

        let mut partitions = job.partitions;

        if job.determinism.enabled
            && job.determinism.stable_partition_order
        {
            partitions.sort_by_key(|partition| partition.id);
        }

        let mut corrections =
            Vec::with_capacity(partitions.len());

        let mut failed_partitions = Vec::new();

        for partition in partitions {
            cancellation.check()?;

            if started.elapsed() > self.limits.max_job_time {
                self.record_job_failure()?;
                return Err(DistributedError::DeadlineExceeded);
            }

            let correction = self.execute_partition_with_retry(
                job.id,
                partition,
                &job.determinism,
                worker_id,
                decoder,
                cancellation.clone(),
                started,
            );

            match correction {
                Ok(value) => {
                    corrections.push(value);
                }

                Err(error) => {
                    self.record_partition_failure(&error)?;

                    failed_partitions.push(
                        match &error {
                            DistributedError::WorkerFailure {
                                ..
                            } => {
                                corrections
                                    .last()
                                    .map(|c| c.partition_id)
                                    .unwrap_or(0)
                            }

                            _ => 0,
                        },
                    );

                    self.record_job_failure()?;

                    return Err(error);
                }
            }
        }

        cancellation.check()?;

        let reconciliation =
            reconcile_boundaries(
                job.id,
                &corrections,
                &job.determinism,
            )?;

        let result = DistributedResult {
            job_id: job.id,
            corrections,
            logical_parity: reconciliation.logical_parity,
            reconciled_boundaries:
                reconciliation.reconciled_boundaries,
            failed_partitions,
        };

        result.validate()?;

        self.record_reconciliation()?;

        let elapsed = started.elapsed();

        {
            let mut metrics = self.lock_metrics()?;

            metrics.jobs_completed =
                metrics.jobs_completed.saturating_add(1);

            metrics.total_wall_time_ns =
                metrics.total_wall_time_ns.saturating_add(
                    elapsed
                        .as_nanos()
                        .min(u128::from(u64::MAX))
                        as u64,
                );
        }

        Ok(result)
    }

    fn execute_partition_with_retry<D: DistributedDecoder>(
        &self,
        job_id: JobId,
        partition: QecPartition,
        determinism: &DeterminismConfig,
        worker_id: WorkerId,
        decoder: &D,
        cancellation: CancellationToken,
        job_started: Instant,
    ) -> Result<PartitionCorrection> {
        let key = TaskKey {
            job_id,
            partition_id: partition.id,
        };

        let mut attempt: TaskAttempt = 1;

        loop {
            cancellation.check()?;

            if job_started.elapsed() > self.limits.max_job_time {
                return Err(DistributedError::DeadlineExceeded);
            }

            if attempt > self.limits.max_retries_per_partition
                .saturating_add(1)
            {
                return Err(DistributedError::RetryExhausted {
                    partition_id: partition.id,
                    attempts: attempt.saturating_sub(1),
                });
            }

            let task = PartitionTask {
                job_id,
                partition: partition.clone(),
                determinism: determinism.clone(),
                attempt,
            };

            task.validate(&self.limits)?;

            self.begin_task(key, worker_id, attempt)?;

            let worker_start = Instant::now();

            self.mark_worker_busy(worker_id)?;

            let context = WorkerContext {
                job_id,
                worker_id,
                cancellation: cancellation.clone(),
                started_at: worker_start,
                deadline: worker_start
                    + self.limits.max_worker_time,
                determinism: determinism.clone(),
                attempt,
            };

            let decode_result =
                decoder.decode(task.clone(), context);

            self.mark_worker_ready(worker_id)?;

            match decode_result {
                Ok(correction) => {
                    let envelope = WorkerResult {
                        task_key: key,
                        worker_id,
                        attempt,
                        integrity: 0,
                        correction,
                    };

                    let envelope = WorkerResult {
                        integrity:
                            envelope.compute_integrity(),
                        ..envelope
                    };

                    match self.accept_worker_result(envelope) {
                        Ok(correction) => {
                            self.record_partition_success(
                                &task,
                            )?;

                            return Ok(correction);
                        }

                        Err(
                            DistributedError::DuplicateTaskResult {
                                ..
                            },
                        ) => {
                            return Err(
                                DistributedError::DuplicateTaskResult {
                                    task_key: key,
                                },
                            );
                        }

                        Err(error) => {
                            self.fail_task(key)?;

                            return Err(error);
                        }
                    }
                }

                Err(
                    DistributedError::WorkerFailure {
                        worker_id,
                        message,
                    },
                ) => {
                    self.fail_task(key)?;

                    self.record_worker_failure()?;

                    if attempt
                        <= self.limits.max_retries_per_partition
                    {
                        self.record_retry()?;
                        attempt = attempt.saturating_add(1);

                        if attempt == 0 {
                            return Err(
                                DistributedError::RetryExhausted {
                                    partition_id: partition.id,
                                    attempts: u32::MAX,
                                },
                            );
                        }

                        continue;
                    }

                    return Err(
                        DistributedError::WorkerFailure {
                            worker_id,
                            message,
                        },
                    );
                }

                Err(error) => {
                    self.fail_task(key)?;

                    return Err(error);
                }
            }
        }
    }

    fn accept_worker_result(
        &self,
        result: WorkerResult,
    ) -> Result<PartitionCorrection> {
        result.validate()?;

        {
            let completed = self
                .completed_results
                .lock()
                .map_err(|_| {
                    DistributedError::Synchronization(
                        "completed-result registry poisoned"
                            .into(),
                    )
                })?;

            if completed.contains(&result.task_key) {
                let mut metrics = self.lock_metrics()?;

                metrics.duplicate_results =
                    metrics.duplicate_results.saturating_add(1);

                return Err(
                    DistributedError::DuplicateTaskResult {
                        task_key: result.task_key,
                    },
                );
            }
        }

        let mut tasks = self.lock_tasks()?;

        let runtime = tasks
            .get_mut(&result.task_key)
            .ok_or_else(|| {
                DistributedError::InvalidWorkerOutput(format!(
                    "worker returned unknown task {}",
                    result.task_key
                ))
            })?;

        if runtime.assigned_worker != result.worker_id {
            return Err(
                DistributedError::PartitionOwnershipViolation {
                    partition_id: result.task_key.partition_id,
                    expected_worker: runtime.assigned_worker,
                    received_worker: result.worker_id,
                },
            );
        }

        if runtime.attempt != result.attempt {
            return Err(DistributedError::StaleTaskResult {
                task_key: result.task_key,
                expected_attempt: runtime.attempt,
                received_attempt: result.attempt,
            });
        }

        runtime.state = TaskState::Completed;

        drop(tasks);

        let mut completed =
            self.completed_results.lock().map_err(|_| {
                DistributedError::Synchronization(
                    "completed-result registry poisoned".into(),
                )
            })?;

        if !completed.insert(result.task_key) {
            return Err(
                DistributedError::DuplicateTaskResult {
                    task_key: result.task_key,
                },
            );
        }

        Ok(result.correction)
    }

    fn begin_task(
        &self,
        key: TaskKey,
        worker_id: WorkerId,
        attempt: TaskAttempt,
    ) -> Result<()> {
        let mut tasks = self.lock_tasks()?;

        if tasks.contains_key(&key) {
            return Err(DistributedError::InvariantViolation(
                format!("task {key} already exists"),
            ));
        }

        let current_in_flight =
            tasks
                .values()
                .filter(|task| {
                    matches!(
                        task.state,
                        TaskState::Queued
                            | TaskState::Assigned
                            | TaskState::Running
                            | TaskState::Retrying
                    )
                })
                .count();

        let requested =
            u64::try_from(current_in_flight)
                .unwrap_or(u64::MAX)
                .saturating_add(1);

        if requested > self.limits.max_in_flight_tasks {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::InFlightTasks,
                    requested,
                    limit: self.limits.max_in_flight_tasks,
                },
            );
        }

        tasks.insert(
            key,
            TaskRuntime {
                state: TaskState::Running,
                assigned_worker: worker_id,
                attempt,
            },
        );

        let mut metrics = self.lock_metrics()?;
        metrics.record_peak(requested);

        Ok(())
    }

    fn fail_task(&self, key: TaskKey) -> Result<()> {
        let mut tasks = self.lock_tasks()?;

        if let Some(task) = tasks.get_mut(&key) {
            task.state = TaskState::Failed;
        }

        Ok(())
    }

    fn select_worker(
        &self,
        determinism: &DeterminismConfig,
    ) -> Result<WorkerId> {
        let workers = self.lock_workers()?;

        let mut candidates: Vec<_> = workers
            .values()
            .filter(|worker| {
                worker.authentication
                    == AuthenticationState::Authenticated
                    && worker.descriptor.state
                        == WorkerState::Ready
                    && worker.active_tasks
                        < worker.descriptor.max_concurrent_tasks
                    && worker.descriptor.capabilities.decoding
            })
            .collect();

        if candidates.is_empty() {
            return Err(
                DistributedError::WorkerUnavailable(0),
            );
        }

        if determinism.enabled
            && determinism.stable_worker_assignment
        {
            candidates.sort_by_key(|worker| worker.descriptor.id);
        } else {
            candidates.sort_by_key(|worker| worker.active_tasks);
        }

        Ok(candidates[0].descriptor.id)
    }

    fn require_worker_capability(
        &self,
        worker_id: WorkerId,
        capability: WorkerCapability,
    ) -> Result<()> {
        let workers = self.lock_workers()?;

        let worker = workers
            .get(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(
                worker_id,
            ))?;

        if worker.authentication
            != AuthenticationState::Authenticated
        {
            return Err(
                DistributedError::WorkerUnauthenticated(
                    worker_id,
                ),
            );
        }

        if !worker.descriptor.capabilities.has(capability) {
            return Err(DistributedError::CapabilityDenied {
                worker_id,
                capability,
            });
        }

        Ok(())
    }

    fn mark_worker_busy(
        &self,
        worker_id: WorkerId,
    ) -> Result<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(
                worker_id,
            ))?;

        if worker.authentication
            != AuthenticationState::Authenticated
        {
            return Err(
                DistributedError::WorkerUnauthenticated(
                    worker_id,
                ),
            );
        }

        if matches!(
            worker.descriptor.state,
            WorkerState::Failed | WorkerState::Offline
        ) {
            return Err(
                DistributedError::WorkerUnavailable(
                    worker_id,
                ),
            );
        }

        let requested =
            u64::from(worker.active_tasks).saturating_add(1);

        if requested > self.limits.max_in_flight_tasks {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::InFlightTasks,
                    requested,
                    limit: self.limits.max_in_flight_tasks,
                },
            );
        }

        if worker.active_tasks
            >= worker.descriptor.max_concurrent_tasks
        {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::InFlightTasks,
                    requested,
                    limit: u64::from(
                        worker.descriptor.max_concurrent_tasks,
                    ),
                },
            );
        }

        worker.active_tasks += 1;
        worker.descriptor.state = WorkerState::Busy;

        Ok(())
    }

    fn mark_worker_ready(
        &self,
        worker_id: WorkerId,
    ) -> Result<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(
                worker_id,
            ))?;

        worker.active_tasks =
            worker.active_tasks.saturating_sub(1);

        if worker.active_tasks == 0
            && worker.authentication
                == AuthenticationState::Authenticated
            && !matches!(
                worker.descriptor.state,
                WorkerState::Failed | WorkerState::Offline
            )
        {
            worker.descriptor.state = WorkerState::Ready;
        }

        Ok(())
    }

    fn record_job_submitted(&self) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.jobs_submitted =
            metrics.jobs_submitted.saturating_add(1);

        Ok(())
    }

    fn record_job_failure(&self) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.jobs_failed =
            metrics.jobs_failed.saturating_add(1);

        Ok(())
    }

    fn record_partition_success(
        &self,
        task: &PartitionTask,
    ) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.partitions_processed =
            metrics.partitions_processed.saturating_add(1);

        metrics.total_events_processed =
            metrics.total_events_processed.saturating_add(
                u64::try_from(task.partition.events.len())
                    .unwrap_or(u64::MAX),
            );

        metrics.total_boundaries_processed =
            metrics.total_boundaries_processed.saturating_add(
                u64::try_from(
                    task.partition.boundary_events.len(),
                )
                .unwrap_or(u64::MAX),
            );

        Ok(())
    }

    fn record_partition_failure(
        &self,
        error: &DistributedError,
    ) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.partitions_failed =
            metrics.partitions_failed.saturating_add(1);

        if matches!(
            error,
            DistributedError::IntegrityFailure(_)
        ) {
            metrics.integrity_failures =
                metrics.integrity_failures.saturating_add(1);
        }

        Ok(())
    }

    fn record_worker_failure(&self) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.worker_failures =
            metrics.worker_failures.saturating_add(1);

        Ok(())
    }

    fn record_retry(&self) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.worker_retries =
            metrics.worker_retries.saturating_add(1);

        Ok(())
    }

    fn record_reconciliation(&self) -> Result<()> {
        let mut metrics = self.lock_metrics()?;

        metrics.boundary_reconciliations =
            metrics.boundary_reconciliations.saturating_add(1);

        Ok(())
    }

    fn lock_workers(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, BTreeMap<WorkerId, WorkerRuntime>>> {
        self.workers.lock().map_err(|_| {
            DistributedError::Synchronization(
                "worker registry poisoned".into(),
            )
        })
    }

    fn lock_metrics(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, DistributedMetrics>> {
        self.metrics.lock().map_err(|_| {
            DistributedError::Synchronization(
                "metrics registry poisoned".into(),
            )
        })
    }

    fn lock_active_jobs(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, BTreeSet<JobId>>> {
        self.active_jobs.lock().map_err(|_| {
            DistributedError::Synchronization(
                "active-job registry poisoned".into(),
            )
        })
    }

    fn lock_tasks(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, BTreeMap<TaskKey, TaskRuntime>>> {
        self.tasks.lock().map_err(|_| {
            DistributedError::Synchronization(
                "task registry poisoned".into(),
            )
        })
    }
}

/// RAII guard for active jobs.
struct ActiveJobGuard<'a> {
    coordinator: &'a DistributedCoordinator,
    job_id: JobId,
    armed: bool,
}

impl<'a> ActiveJobGuard<'a> {
    fn new(
        coordinator: &'a DistributedCoordinator,
        job_id: JobId,
    ) -> Result<Self> {
        let mut active =
            coordinator.lock_active_jobs()?;

        if u64::try_from(active.len()).unwrap_or(u64::MAX)
            >= coordinator.limits.max_jobs
        {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Jobs,
                    requested: u64::try_from(active.len())
                        .unwrap_or(u64::MAX)
                        .saturating_add(1),
                    limit: coordinator.limits.max_jobs,
                },
            );
        }

        if !active.insert(job_id) {
            return Err(DistributedError::InvalidJob(
                format!("job {} is already active", job_id),
            ));
        }

        Ok(Self {
            coordinator,
            job_id,
            armed: true,
        })
    }
}

impl Drop for ActiveJobGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self
                .coordinator
                .release_job(self.job_id);
        }
    }
}

/// Boundary reconciliation output.
#[derive(Debug, Clone)]
struct Reconciliation {
    logical_parity: u8,
    reconciled_boundaries: Vec<BoundaryId>,
}

/// Reconcile partition corrections.
///
/// Rules:
///
/// 1. Partition order is deterministic.
/// 2. Boundary IDs are canonicalized.
/// 3. A boundary cannot be resolved twice.
/// 4. Logical parity is XOR-reduced.
/// 5. No unresolved duplicate correction is silently accepted.
///
/// This is deliberately a classical reconciliation primitive. It does not
/// attempt to perform MWPM itself.
fn reconcile_boundaries(
    job_id: JobId,
    corrections: &[PartitionCorrection],
    determinism: &DeterminismConfig,
) -> Result<Reconciliation> {
    if job_id == 0 {
        return Err(
            DistributedError::BoundaryReconciliationFailed(
                "job id cannot be zero".into(),
            ),
        );
    }

    let mut ordered = corrections.to_vec();

    if determinism.enabled
        && determinism.stable_partition_order
    {
        ordered.sort_by_key(|correction| correction.partition_id);
    }

    let mut boundaries = BTreeSet::new();
    let mut logical_parity = 0u8;

    for correction in ordered {
        correction.validate()?;

        logical_parity ^= correction.logical_parity;

        for boundary in correction.resolved_boundaries {
            if !boundaries.insert(boundary) {
                return Err(
                    DistributedError::BoundaryReconciliationFailed(
                        format!(
                            "boundary {} resolved more than once",
                            boundary
                        ),
                    ),
                );
            }
        }
    }

    Ok(Reconciliation {
        logical_parity,
        reconciled_boundaries:
            boundaries.into_iter().collect(),
    })
}

/// Deterministic rectangular partition planner.
#[derive(Debug, Clone)]
pub struct PartitionPlanner {
    pub x_chunks: u64,
    pub y_chunks: u64,
    pub z_chunks: u64,
}

impl PartitionPlanner {
    pub fn new(
        x_chunks: u64,
        y_chunks: u64,
        z_chunks: u64,
    ) -> Result<Self> {
        if x_chunks == 0
            || y_chunks == 0
            || z_chunks == 0
        {
            return Err(DistributedError::InvalidInput(
                "partition dimensions must be greater than zero"
                    .into(),
            ));
        }

        Ok(Self {
            x_chunks,
            y_chunks,
            z_chunks,
        })
    }

    pub fn partition_count(&self) -> Result<u64> {
        self.x_chunks
            .checked_mul(self.y_chunks)
            .and_then(|value| {
                value.checked_mul(self.z_chunks)
            })
            .ok_or(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Partitions,
                    requested: u64::MAX,
                    limit: u64::MAX,
                },
            )
    }

    pub fn plan(
        &self,
        extent: [(i64, i64); 3],
        limits: &DistributedLimits,
    ) -> Result<Vec<QecPartition>> {
        let total = self.partition_count()?;

        if total > limits.max_partitions_per_job {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Partitions,
                    requested: total,
                    limit: limits.max_partitions_per_job,
                },
            );
        }

        for (axis, (min, max)) in extent.iter().enumerate() {
            if min >= max {
                return Err(DistributedError::InvalidInput(
                    format!(
                        "invalid lattice extent on axis {}: [{}, {})",
                        axis, min, max
                    ),
                ));
            }
        }

        let x_ranges =
            split_range(extent[0], self.x_chunks)?;

        let y_ranges =
            split_range(extent[1], self.y_chunks)?;

        let z_ranges =
            split_range(extent[2], self.z_chunks)?;

        let capacity =
            usize::try_from(total).map_err(|_| {
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Partitions,
                    requested: total,
                    limit: u64::try_from(usize::MAX)
                        .unwrap_or(u64::MAX),
                }
            })?;

        let mut partitions =
            Vec::with_capacity(capacity);

        let mut id = 1u64;

        for x in &x_ranges {
            for y in &y_ranges {
                for z in &z_ranges {
                    partitions.push(QecPartition {
                        id,
                        bounds: [*x, *y, *z],
                        events: Vec::new(),
                        boundary_events: Vec::new(),
                        neighbors: BTreeSet::new(),
                        logical_region: None,
                    });

                    id = id.checked_add(1).ok_or(
                        DistributedError::InvalidPartition(
                            "partition identifier overflow".into(),
                        ),
                    )?;
                }
            }
        }

        connect_face_neighbors(&mut partitions);

        Ok(partitions)
    }
}

fn split_range(
    range: (i64, i64),
    chunks: u64,
) -> Result<Vec<(i64, i64)>> {
    if chunks == 0 || range.0 >= range.1 {
        return Err(DistributedError::InvalidInput(
            "invalid range or chunk count".into(),
        ));
    }

    let length =
        i128::from(range.1) - i128::from(range.0);

    if i128::from(chunks) > length {
        return Err(DistributedError::InvalidInput(
            format!(
                "cannot create {} non-empty chunks from range [{}, {})",
                chunks, range.0, range.1
            ),
        ));
    }

    let base = length / i128::from(chunks);
    let remainder = length % i128::from(chunks);

    let capacity =
        usize::try_from(chunks).map_err(|_| {
            DistributedError::InvalidInput(
                "chunk count does not fit platform usize".into(),
            )
        })?;

    let mut result = Vec::with_capacity(capacity);

    let mut current = i128::from(range.0);

    for index in 0..i128::from(chunks) {
        let size =
            base + if index < remainder { 1 } else { 0 };

        let next = current + size;

        let start =
            i64::try_from(current).map_err(|_| {
                DistributedError::InvalidInput(
                    "range conversion overflow".into(),
                )
            })?;

        let end =
            i64::try_from(next).map_err(|_| {
                DistributedError::InvalidInput(
                    "range conversion overflow".into(),
                )
            })?;

        result.push((start, end));

        current = next;
    }

    Ok(result)
}

fn ranges_touch(
    a: (i64, i64),
    b: (i64, i64),
) -> bool {
    a.1 == b.0 || b.1 == a.0
}

fn overlap(
    a: (i64, i64),
    b: (i64, i64),
) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn face_neighbors(
    a: &QecPartition,
    b: &QecPartition,
) -> bool {
    let ax = a.bounds[0];
    let ay = a.bounds[1];
    let az = a.bounds[2];

    let bx = b.bounds[0];
    let by = b.bounds[1];
    let bz = b.bounds[2];

    (ranges_touch(ax, bx)
        && overlap(ay, by)
        && overlap(az, bz))
        || (ranges_touch(ay, by)
            && overlap(ax, bx)
            && overlap(az, bz))
        || (ranges_touch(az, bz)
            && overlap(ax, bx)
            && overlap(ay, by))
}

fn connect_face_neighbors(
    partitions: &mut [QecPartition],
) {
    for left in 0..partitions.len() {
        for right in (left + 1)..partitions.len() {
            if face_neighbors(
                &partitions[left],
                &partitions[right],
            ) {
                let left_id = partitions[left].id;
                let right_id = partitions[right].id;

                partitions[left]
                    .neighbors
                    .insert(right_id);

                partitions[right]
                    .neighbors
                    .insert(left_id);
            }
        }
    }
}

/// Bounded task queue.
///
/// This is deliberately non-blocking. A scheduler can translate
/// `ResourceLimitExceeded(InFlightTasks)` into backpressure.
#[derive(Debug)]
pub struct TaskQueue {
    queue: VecDeque<PartitionTask>,
    max_tasks: usize,
}

impl TaskQueue {
    pub fn new(max_tasks: usize) -> Result<Self> {
        if max_tasks == 0 {
            return Err(DistributedError::InvalidInput(
                "task queue capacity must be greater than zero"
                    .into(),
            ));
        }

        Ok(Self {
            queue: VecDeque::with_capacity(
                max_tasks.min(1024),
            ),
            max_tasks,
        })
    }

    pub fn push(
        &mut self,
        task: PartitionTask,
    ) -> Result<()> {
        if self.queue.len() >= self.max_tasks {
            return Err(
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::InFlightTasks,
                    requested: u64::try_from(
                        self.queue.len(),
                    )
                    .unwrap_or(u64::MAX)
                    .saturating_add(1),
                    limit: u64::try_from(
                        self.max_tasks,
                    )
                    .unwrap_or(u64::MAX),
                },
            );
        }

        self.queue.push_back(task);

        Ok(())
    }

    pub fn pop(&mut self) -> Option<PartitionTask> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Stable task ordering.
pub fn stable_task_order(
    tasks: &mut [PartitionTask],
) {
    tasks.sort_by_key(|task| {
        (task.job_id, task.partition.id, task.attempt)
    });
}

/// Estimate partition workload without constructing decoder state.
pub fn estimate_partition_workload(
    partition: &QecPartition,
) -> Result<u64> {
    let limits = DistributedLimits::default();

    partition.validate(&limits)?;

    let events =
        u64::try_from(partition.events.len())
            .unwrap_or(u64::MAX);

    let boundaries =
        u64::try_from(partition.boundary_events.len())
            .unwrap_or(u64::MAX);

    let event_cost =
        events.checked_mul(64).ok_or(
            DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Bytes,
                requested: u64::MAX,
                limit: u64::MAX,
            },
        )?;

    let boundary_cost =
        boundaries.checked_mul(96).ok_or(
            DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Bytes,
                requested: u64::MAX,
                limit: u64::MAX,
            },
        )?;

    event_cost.checked_add(boundary_cost).ok_or(
        DistributedError::ResourceLimitExceeded {
            resource: ResourceKind::Bytes,
            requested: u64::MAX,
            limit: u64::MAX,
        },
    )
}

/// Validate a distributed format version.
pub fn validate_format_version(
    found: u32,
) -> Result<()> {
    if found != DISTRIBUTED_FORMAT_VERSION {
        return Err(
            DistributedError::IncompatibleVersion {
                expected: DISTRIBUTED_FORMAT_VERSION,
                found,
            },
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> DistributedLimits {
        DistributedLimits {
            max_jobs: 10,
            max_partitions_per_job: 100,
            max_workers: 10,
            max_events_per_partition: 1_000,
            max_boundary_events_per_partition: 100,
            max_in_flight_tasks: 100,
            max_job_bytes: 1_000_000,
            max_worker_time: Duration::from_secs(10),
            max_job_time: Duration::from_secs(30),
            max_retries_per_partition: 2,
        }
    }

    fn worker() -> WorkerDescriptor {
        WorkerDescriptor {
            id: 1,
            name: "test-worker".into(),
            state: WorkerState::Ready,
            capabilities: WorkerCapabilities::default(),
            max_concurrent_tasks: 1,
        }
    }

    fn partition(
        id: PartitionId,
    ) -> QecPartition {
        QecPartition {
            id,
            bounds: [(0, 10), (0, 10), (0, 10)],
            events: vec![DetectionEvent {
                id,
                x: 1,
                y: 1,
                z: 1,
                time: 0,
                weight: 1.0,
            }],
            boundary_events: Vec::new(),
            neighbors: BTreeSet::new(),
            logical_region: None,
        }
    }

    #[test]
    fn limits_validate() {
        assert!(limits().validate().is_ok());
    }

    #[test]
    fn partition_validates() {
        assert!(
            partition(1)
                .validate(&limits())
                .is_ok()
        );
    }

    #[test]
    fn invalid_weight_is_rejected() {
        let mut value = partition(1);

        value.events[0].weight = f64::NAN;

        assert!(
            value.validate(&limits()).is_err()
        );
    }

    #[test]
    fn duplicate_event_ids_are_rejected() {
        let mut value = partition(1);

        value.events.push(
            value.events[0].clone(),
        );

        assert!(
            value.validate(&limits()).is_err()
        );
    }

    #[test]
    fn planner_creates_expected_partition_count() {
        let planner =
            PartitionPlanner::new(2, 2, 2)
                .unwrap();

        let partitions = planner
            .plan(
                [(0, 10), (0, 10), (0, 10)],
                &limits(),
            )
            .unwrap();

        assert_eq!(partitions.len(), 8);
    }

    #[test]
    fn planner_connects_face_neighbors() {
        let planner =
            PartitionPlanner::new(2, 1, 1)
                .unwrap();

        let partitions = planner
            .plan(
                [(0, 10), (0, 10), (0, 10)],
                &limits(),
            )
            .unwrap();

        assert_eq!(partitions.len(), 2);

        assert!(
            partitions[0]
                .neighbors
                .contains(&partitions[1].id)
        );

        assert!(
            partitions[1]
                .neighbors
                .contains(&partitions[0].id)
        );
    }

    #[test]
    fn planner_rejects_too_many_chunks() {
        let planner =
            PartitionPlanner::new(11, 1, 1)
                .unwrap();

        assert!(
            planner
                .plan(
                    [(0, 10), (0, 10), (0, 10)],
                    &limits()
                )
                .is_err()
        );
    }

    #[test]
    fn cancellation_is_cooperative() {
        let token =
            CancellationToken::new();

        assert!(token.check().is_ok());

        token.cancel();

        assert_eq!(
            token.check(),
            Err(DistributedError::Cancelled)
        );
    }

    #[test]
    fn task_queue_is_bounded() {
        let mut queue =
            TaskQueue::new(1).unwrap();

        let task = PartitionTask {
            job_id: 1,
            partition: partition(1),
            determinism:
                DeterminismConfig::default(),
            attempt: 1,
        };

        assert!(
            queue.push(task.clone()).is_ok()
        );

        assert!(
            queue.push(task).is_err()
        );
    }

    #[test]
    fn deterministic_task_order_is_stable() {
        let mut tasks = vec![
            PartitionTask {
                job_id: 1,
                partition: partition(3),
                determinism:
                    DeterminismConfig::default(),
                attempt: 1,
            },
            PartitionTask {
                job_id: 1,
                partition: partition(1),
                determinism:
                    DeterminismConfig::default(),
                attempt: 1,
            },
            PartitionTask {
                job_id: 1,
                partition: partition(2),
                determinism:
                    DeterminismConfig::default(),
                attempt: 1,
            },
        ];

        stable_task_order(&mut tasks);

        assert_eq!(
            tasks[0].partition.id,
            1
        );
        assert_eq!(
            tasks[1].partition.id,
            2
        );
        assert_eq!(
            tasks[2].partition.id,
            3
        );
    }

    #[test]
    fn worker_must_be_authenticated() {
        let coordinator =
            DistributedCoordinator::new(
                limits()
            )
            .unwrap();

        let id = coordinator
            .register_worker(worker())
            .unwrap();

        assert_eq!(id, 1);

        let job = DistributedJob {
            id: 1,
            partitions: vec![partition(1)],
            mode: ExecutionMode::Distributed,
            determinism:
                DeterminismConfig::default(),
            metadata: BTreeMap::new(),
        };

        assert!(
            coordinator
                .execute(
                    job,
                    &IdentityDecoder,
                    CancellationToken::new()
                )
                .is_err()
        );
    }

    #[test]
    fn identity_decoder_executes_after_authentication() {
        let coordinator =
            DistributedCoordinator::new(
                limits()
            )
            .unwrap();

        let id = coordinator
            .register_worker(worker())
            .unwrap();

        coordinator
            .authenticate_worker(id)
            .unwrap();

        let job = DistributedJob {
            id: 1,
            partitions: vec![partition(1)],
            mode: ExecutionMode::Distributed,
            determinism:
                DeterminismConfig::default(),
            metadata: BTreeMap::new(),
        };

        let result = coordinator
            .execute(
                job,
                &IdentityDecoder,
                CancellationToken::new(),
            )
            .unwrap();

        assert_eq!(result.job_id, 1);
        assert_eq!(result.corrections.len(), 1);
        assert_eq!(result.logical_parity, 0);
    }

    #[test]
    fn malformed_job_is_rejected() {
        let coordinator =
            DistributedCoordinator::new(
                limits()
            )
            .unwrap();

        let worker_id = coordinator
            .register_worker(worker())
            .unwrap();

        coordinator
            .authenticate_worker(worker_id)
            .unwrap();

        let job = DistributedJob {
            id: 1,
            partitions: Vec::new(),
            mode: ExecutionMode::Distributed,
            determinism:
                DeterminismConfig::default(),
            metadata: BTreeMap::new(),
        };

        assert!(
            coordinator
                .execute(
                    job,
                    &IdentityDecoder,
                    CancellationToken::new()
                )
                .is_err()
        );
    }

    #[test]
    fn worker_result_integrity_is_verified() {
        let correction =
            PartitionCorrection {
                partition_id: 1,
                corrections: vec![(5, 1)],
                logical_parity: 0,
                resolved_boundaries: vec![],
            };

        let mut result = WorkerResult {
            task_key: TaskKey {
                job_id: 1,
                partition_id: 1,
            },
            worker_id: 1,
            attempt: 1,
            correction,
            integrity: 0,
        };

        result.integrity =
            result.compute_integrity();

        assert!(result.validate().is_ok());

        result.integrity ^= 1;

        assert!(
            matches!(
                result.validate(),
                Err(
                    DistributedError::IntegrityFailure(_)
                )
            )
        );
    }

    #[test]
    fn partition_boundary_requires_current_version() {
        let boundary =
            PartitionBoundary {
                partition_id: 1,
                neighbor_id: 2,
                incoming_events: vec![],
                outgoing_events: vec![],
                virtual_boundary_parity: 0,
                correction_parity: 0,
                logical_parity: 0,
                reconciliation_version:
                    DISTRIBUTED_FORMAT_VERSION,
            };

        assert!(boundary.validate().is_ok());
    }

    #[test]
    fn format_version_is_checked() {
        assert!(
            validate_format_version(
                DISTRIBUTED_FORMAT_VERSION
            )
            .is_ok()
        );

        assert!(
            validate_format_version(
                DISTRIBUTED_FORMAT_VERSION + 1
            )
            .is_err()
        );
    }

    #[test]
    fn workload_estimation_is_nonzero() {
        let value =
            partition(1);

        assert!(
            estimate_partition_workload(
                &value
            )
            .unwrap()
                > 0
        );
    }
}