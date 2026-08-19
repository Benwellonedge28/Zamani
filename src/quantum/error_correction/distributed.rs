//! Distributed Quantum Error Correction infrastructure.
//!
//! This module provides the coordination layer for partitioned QEC workloads.
//!
//! Design goals:
//! - Distributed execution is optional.
//! - The same abstraction can represent local, threaded, process, or remote workers.
//! - Untrusted work is never executed without validation at the coordinator boundary.
//! - Explicit resource limits prevent allocation/worker/graph amplification.
//! - Cancellation is cooperative and checked at safe boundaries.
//! - Deterministic execution is supported through stable ordering and explicit seeds.
//! - Partition boundary information is preserved for global reconciliation.
//! - Worker failures are isolated and represented as typed errors.
//! - No unsafe code is required.
//!
//! This module deliberately does not implement a particular decoder. MWPM,
//! Union-Find, or future decoders can use the distributed infrastructure
//! through `DistributedDecoder`.
//!
//! Important:
//! "Infinite scalability" is not promised. The system instead supports
//! arbitrarily large workloads subject to explicit resource limits,
//! partitioning, streaming, checkpointing, and graceful failure.

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

/// Numeric graph weight.
///
/// The distributed layer intentionally does not interpret the mathematical
/// meaning of the weight. The decoder owns that responsibility.
pub type Weight = f64;

/// Result type used throughout this module.
pub type Result<T> = std::result::Result<T, DistributedError>;

/// Errors produced by the distributed QEC infrastructure.
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedError {
    /// The supplied request is structurally invalid.
    InvalidInput(String),

    /// A partition is invalid or internally inconsistent.
    InvalidPartition(String),

    /// A worker description is invalid.
    InvalidWorker(String),

    /// A boundary description is invalid.
    InvalidBoundary(String),

    /// A job cannot be accepted under the current configuration.
    InvalidJob(String),

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u64,
        limit: u64,
    },

    /// A worker could not complete its assigned operation.
    WorkerFailure {
        worker_id: WorkerId,
        message: String,
    },

    /// A worker disappeared or became unavailable.
    WorkerUnavailable(WorkerId),

    /// A job or partition was cancelled.
    Cancelled,

    /// A deadline expired.
    DeadlineExceeded,

    /// Deterministic execution requirements could not be satisfied.
    DeterminismViolation(String),

    /// A boundary could not be reconciled.
    BoundaryReconciliationFailed(String),

    /// A distributed operation timed out.
    Timeout,

    /// A worker returned malformed output.
    InvalidWorkerOutput(String),

    /// A distributed execution invariant was violated.
    InvariantViolation(String),

    /// A requested feature is not implemented by the selected backend.
    Unsupported(String),

    /// Serialization/checkpoint format is incompatible.
    IncompatibleVersion {
        expected: u32,
        found: u32,
    },

    /// Internal synchronization failed.
    Synchronization(String),
}

impl fmt::Display for DistributedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "invalid distributed input: {msg}"),
            Self::InvalidPartition(msg) => write!(f, "invalid partition: {msg}"),
            Self::InvalidWorker(msg) => write!(f, "invalid worker: {msg}"),
            Self::InvalidBoundary(msg) => write!(f, "invalid boundary: {msg}"),
            Self::InvalidJob(msg) => write!(f, "invalid job: {msg}"),
            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => write!(
                f,
                "resource limit exceeded for {resource:?}: requested={requested}, limit={limit}"
            ),
            Self::WorkerFailure { worker_id, message } => {
                write!(f, "worker {worker_id} failed: {message}")
            }
            Self::WorkerUnavailable(id) => write!(f, "worker {id} unavailable"),
            Self::Cancelled => write!(f, "distributed operation cancelled"),
            Self::DeadlineExceeded => write!(f, "distributed operation deadline exceeded"),
            Self::DeterminismViolation(msg) => {
                write!(f, "determinism violation: {msg}")
            }
            Self::BoundaryReconciliationFailed(msg) => {
                write!(f, "boundary reconciliation failed: {msg}")
            }
            Self::Timeout => write!(f, "distributed operation timed out"),
            Self::InvalidWorkerOutput(msg) => {
                write!(f, "invalid worker output: {msg}")
            }
            Self::InvariantViolation(msg) => {
                write!(f, "distributed invariant violation: {msg}")
            }
            Self::Unsupported(msg) => write!(f, "unsupported distributed operation: {msg}"),
            Self::IncompatibleVersion { expected, found } => {
                write!(f, "incompatible version: expected {expected}, found {found}")
            }
            Self::Synchronization(msg) => {
                write!(f, "distributed synchronization failure: {msg}")
            }
        }
    }
}

impl std::error::Error for DistributedError {}

/// Resource classes controlled by distributed execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceKind {
    Jobs,
    Partitions,
    Workers,
    Events,
    BoundaryEvents,
    Bytes,
    InFlightTasks,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Jobs => "jobs",
            Self::Partitions => "partitions",
            Self::Workers => "workers",
            Self::Events => "events",
            Self::BoundaryEvents => "boundary events",
            Self::Bytes => "bytes",
            Self::InFlightTasks => "in-flight tasks",
        };

        f.write_str(name)
    }
}

/// Resource limits for distributed execution.
#[derive(Debug, Clone)]
pub struct DistributedLimits {
    /// Maximum simultaneously accepted jobs.
    pub max_jobs: u64,

    /// Maximum partitions in one job.
    pub max_partitions_per_job: u64,

    /// Maximum registered workers.
    pub max_workers: u64,

    /// Maximum syndrome/detection events in one partition.
    pub max_events_per_partition: u64,

    /// Maximum boundary events in one partition.
    pub max_boundary_events_per_partition: u64,

    /// Maximum in-flight tasks.
    pub max_in_flight_tasks: u64,

    /// Maximum estimated memory for a job.
    pub max_job_bytes: u64,

    /// Maximum worker execution time.
    pub max_worker_time: Duration,

    /// Maximum global execution time.
    pub max_job_time: Duration,
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
        }
    }
}

impl DistributedLimits {
    pub fn validate(&self) -> Result<()> {
        if self.max_jobs == 0 {
            return Err(DistributedError::InvalidInput(
                "max_jobs must be greater than zero".into(),
            ));
        }

        if self.max_partitions_per_job == 0 {
            return Err(DistributedError::InvalidInput(
                "max_partitions_per_job must be greater than zero".into(),
            ));
        }

        if self.max_workers == 0 {
            return Err(DistributedError::InvalidInput(
                "max_workers must be greater than zero".into(),
            ));
        }

        if self.max_events_per_partition == 0 {
            return Err(DistributedError::InvalidInput(
                "max_events_per_partition must be greater than zero".into(),
            ));
        }

        if self.max_boundary_events_per_partition == 0 {
            return Err(DistributedError::InvalidInput(
                "max_boundary_events_per_partition must be greater than zero".into(),
            ));
        }

        if self.max_in_flight_tasks == 0 {
            return Err(DistributedError::InvalidInput(
                "max_in_flight_tasks must be greater than zero".into(),
            ));
        }

        if self.max_job_bytes == 0 {
            return Err(DistributedError::InvalidInput(
                "max_job_bytes must be greater than zero".into(),
            ));
        }

        if self.max_worker_time.is_zero() || self.max_job_time.is_zero() {
            return Err(DistributedError::InvalidInput(
                "execution time limits must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

/// Cooperative cancellation token.
///
/// Workers must periodically check this token at safe points.
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

/// Deterministic execution configuration.
#[derive(Debug, Clone)]
pub struct DeterminismConfig {
    /// Whether deterministic execution is required.
    pub enabled: bool,

    /// Stable seed used by stochastic algorithms.
    pub seed: u64,

    /// Require stable partition ordering.
    pub stable_partition_order: bool,

    /// Require stable worker assignment.
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
        // Reserved for future deterministic-policy checks.
        Ok(())
    }
}

/// Execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    Local,
    MultiThreaded,
    MultiProcess,
    Distributed,
    Accelerated,
}

/// Worker lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Starting,
    Ready,
    Busy,
    Draining,
    Failed,
    Offline,
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
        }
    }
}

/// A worker registered with the coordinator.
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

/// A detection event belonging to a partition.
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

/// A boundary event exposed by a partition.
///
/// Boundary events are deliberately explicit. A partition must never
/// silently discard a syndrome event that may interact with another
/// partition.
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

/// A partition of the global QEC decoding problem.
#[derive(Debug, Clone)]
pub struct QecPartition {
    pub id: PartitionId,

    /// Half-open bounds `(min, max)` in x/y/z coordinates.
    pub bounds: [(i64, i64); 3],

    pub events: Vec<DetectionEvent>,

    pub boundary_events: Vec<BoundaryEvent>,

    /// Neighboring partitions.
    pub neighbors: BTreeSet<PartitionId>,

    /// Optional logical-region identifier.
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

        let event_count = self.events.len() as u64;
        if event_count > limits.max_events_per_partition {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Events,
                requested: event_count,
                limit: limits.max_events_per_partition,
            });
        }

        let boundary_count = self.boundary_events.len() as u64;
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
                    boundary.id, boundary.source_partition, self.id
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
        let [x_bounds, y_bounds, z_bounds] = self.bounds;

        x >= x_bounds.0
            && x < x_bounds.1
            && y >= y_bounds.0
            && y < y_bounds.1
            && z >= z_bounds.0
            && z < z_bounds.1
    }

    pub fn estimated_event_bytes(&self) -> u64 {
        let event_bytes = self.events.len() as u64 * 48;
        let boundary_bytes = self.boundary_events.len() as u64 * 48;

        event_bytes.saturating_add(boundary_bytes)
    }
}

/// A complete distributed decoding job.
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

        let partition_count = self.partitions.len() as u64;

        if partition_count > limits.max_partitions_per_job {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Partitions,
                requested: partition_count,
                limit: limits.max_partitions_per_job,
            });
        }

        let mut ids = BTreeSet::new();
        let mut estimated_bytes = 0u64;

        for partition in &self.partitions {
            partition.validate(limits)?;

            if !ids.insert(partition.id) {
                return Err(DistributedError::InvalidJob(format!(
                    "duplicate partition id {}",
                    partition.id
                )));
            }

            estimated_bytes =
                estimated_bytes.saturating_add(partition.estimated_event_bytes());
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
        let ids: BTreeSet<_> = self.partitions.iter().map(|p| p.id).collect();

        for partition in &self.partitions {
            for neighbor in &partition.neighbors {
                if !ids.contains(neighbor) {
                    return Err(DistributedError::InvalidJob(format!(
                        "partition {} references unknown neighbor {}",
                        partition.id, neighbor
                    )));
                }

                let reverse_exists = self
                    .partitions
                    .iter()
                    .find(|p| p.id == *neighbor)
                    .map(|p| p.neighbors.contains(&partition.id))
                    .unwrap_or(false);

                if !reverse_exists {
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

/// Correction produced by one partition.
///
/// The distributed layer treats corrections as opaque decoder output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionCorrection {
    pub partition_id: PartitionId,

    /// Stable qubit identifiers and their correction operation.
    ///
    /// The actual interpretation of the operation belongs to the decoder.
    pub corrections: Vec<(u64, u8)>,

    /// Logical parity contribution discovered locally.
    pub logical_parity: u8,

    /// Boundary events consumed by the local correction.
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

        let mut ids = BTreeSet::new();

        for (qubit, operation) in &self.corrections {
            if *operation > 3 {
                return Err(DistributedError::InvalidWorkerOutput(format!(
                    "partition {} returned invalid correction operation {} for qubit {}",
                    self.partition_id, operation, qubit
                )));
            }

            if !ids.insert(*qubit) {
                return Err(DistributedError::InvalidWorkerOutput(format!(
                    "partition {} returned duplicate correction for qubit {}",
                    self.partition_id, qubit
                )));
            }
        }

        Ok(())
    }
}

/// Global result after boundary reconciliation.
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

        let mut partition_ids = BTreeSet::new();

        for correction in &self.corrections {
            correction.validate()?;

            if !partition_ids.insert(correction.partition_id) {
                return Err(DistributedError::InvariantViolation(format!(
                    "duplicate correction for partition {}",
                    correction.partition_id
                )));
            }
        }

        Ok(())
    }
}

/// A unit of work sent to a worker.
#[derive(Debug, Clone)]
pub struct PartitionTask {
    pub job_id: JobId,
    pub partition: QecPartition,
    pub determinism: DeterminismConfig,
}

impl PartitionTask {
    pub fn validate(&self, limits: &DistributedLimits) -> Result<()> {
        if self.job_id == 0 {
            return Err(DistributedError::InvalidJob(
                "task job id must be non-zero".into(),
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

/// Trait implemented by actual QEC decoding workers.
///
/// A worker may wrap:
/// - MWPM
/// - Union-Find
/// - a GPU decoder
/// - a remote RPC worker
/// - a process
/// - a future accelerator
pub trait DistributedDecoder: Send + Sync {
    fn decode(
        &self,
        task: PartitionTask,
        context: WorkerContext,
    ) -> Result<PartitionCorrection>;
}

/// Default deterministic no-op decoder.
///
/// This is useful for infrastructure testing and integration tests.
/// Real Zamani decoders should replace it.
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
                .map(|b| b.id)
                .collect(),
        })
    }
}

/// Runtime worker registration state.
#[derive(Debug, Clone)]
struct WorkerRuntime {
    descriptor: WorkerDescriptor,
    active_tasks: u32,
}

/// Coordinator statistics.
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
    pub peak_in_flight_tasks: u64,
    pub total_events_processed: u64,
    pub total_boundaries_processed: u64,
    pub total_wall_time_ns: u64,
}

impl DistributedMetrics {
    fn record_in_flight_peak(&mut self, value: u64) {
        self.peak_in_flight_tasks = self.peak_in_flight_tasks.max(value);
    }
}

/// Coordinator for distributed QEC decoding.
pub struct DistributedCoordinator {
    limits: DistributedLimits,
    workers: Mutex<BTreeMap<WorkerId, WorkerRuntime>>,
    metrics: Mutex<DistributedMetrics>,
    next_job_id: AtomicU64,
    next_worker_id: AtomicU64,
    active_jobs: Mutex<BTreeSet<JobId>>,
}

impl DistributedCoordinator {
    pub fn new(limits: DistributedLimits) -> Result<Self> {
        limits.validate()?;

        Ok(Self {
            limits,
            workers: Mutex::new(BTreeMap::new()),
            metrics: Mutex::new(DistributedMetrics::default()),
            next_job_id: AtomicU64::new(1),
            next_worker_id: AtomicU64::new(1),
            active_jobs: Mutex::new(BTreeSet::new()),
        })
    }

    pub fn limits(&self) -> &DistributedLimits {
        &self.limits
    }

    pub fn register_worker(
        &self,
        mut descriptor: WorkerDescriptor,
    ) -> Result<WorkerId> {
        descriptor.validate()?;

        let mut workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        if workers.len() as u64 >= self.limits.max_workers {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Workers,
                requested: workers.len() as u64 + 1,
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
        descriptor.state = WorkerState::Ready;

        workers.insert(
            id,
            WorkerRuntime {
                descriptor,
                active_tasks: 0,
            },
        );

        Ok(id)
    }

    pub fn unregister_worker(&self, worker_id: WorkerId) -> Result<()> {
        let mut workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        match workers.remove(&worker_id) {
            Some(_) => Ok(()),
            None => Err(DistributedError::WorkerUnavailable(worker_id)),
        }
    }

    pub fn workers(&self) -> Result<Vec<WorkerDescriptor>> {
        let workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        Ok(workers.values().map(|w| w.descriptor.clone()).collect())
    }

    pub fn allocate_job_id(&self) -> Result<JobId> {
        let mut active = self
            .active_jobs
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "active job registry poisoned".into(),
            ))?;

        if active.len() as u64 >= self.limits.max_jobs {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Jobs,
                requested: active.len() as u64 + 1,
                limit: self.limits.max_jobs,
            });
        }

        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);
        active.insert(id);

        Ok(id)
    }

    pub fn release_job(&self, job_id: JobId) -> Result<()> {
        let mut active = self
            .active_jobs
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "active job registry poisoned".into(),
            ))?;

        active.remove(&job_id);
        Ok(())
    }

    pub fn metrics(&self) -> Result<DistributedMetrics> {
        let metrics = self
            .metrics
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "metrics registry poisoned".into(),
            ))?;

        Ok(metrics.clone())
    }

    /// Validate and execute a complete job.
    ///
    /// The current implementation executes workers sequentially for portability
    /// and deterministic behavior. The API intentionally separates scheduling
    /// from decoding so a future scheduler can execute tasks concurrently,
    /// remotely, or on accelerators without changing the job model.
    pub fn execute<D: DistributedDecoder>(
        &self,
        job: DistributedJob,
        decoder: &D,
        cancellation: CancellationToken,
    ) -> Result<DistributedResult> {
        job.validate(&self.limits)?;

        let started = Instant::now();

        if started.elapsed() > self.limits.max_job_time {
            return Err(DistributedError::DeadlineExceeded);
        }

        let mut metrics = self
            .metrics
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "metrics registry poisoned".into(),
            ))?;

        metrics.jobs_submitted = metrics.jobs_submitted.saturating_add(1);
        drop(metrics);

        let mut active_job_guard = ActiveJobGuard::new(self, job.id)?;

        let worker_id = self.select_worker(&job.determinism)?;

        let mut corrections = Vec::with_capacity(job.partitions.len());
        let mut failed_partitions = Vec::new();

        for partition in job.partitions {
            cancellation.check()?;

            if started.elapsed() > self.limits.max_job_time {
                return Err(DistributedError::DeadlineExceeded);
            }

            let task = PartitionTask {
                job_id: job.id,
                partition,
                determinism: job.determinism.clone(),
            };

            task.validate(&self.limits)?;

            let context = WorkerContext {
                job_id: job.id,
                worker_id,
                cancellation: cancellation.clone(),
                started_at: started,
                deadline: started + self.limits.max_worker_time,
                determinism: job.determinism.clone(),
            };

            self.mark_worker_busy(worker_id)?;

            let result = decoder.decode(task.clone(), context);

            self.mark_worker_ready(worker_id)?;

            match result {
                Ok(correction) => {
                    correction.validate()?;

                    corrections.push(correction);

                    let mut metrics = self
                        .metrics
                        .lock()
                        .map_err(|_| DistributedError::Synchronization(
                            "metrics registry poisoned".into(),
                        ))?;

                    metrics.partitions_processed =
                        metrics.partitions_processed.saturating_add(1);

                    metrics.total_events_processed = metrics
                        .total_events_processed
                        .saturating_add(task.partition.events.len() as u64);

                    metrics.total_boundaries_processed = metrics
                        .total_boundaries_processed
                        .saturating_add(task.partition.boundary_events.len() as u64);

                    drop(metrics);
                }

                Err(DistributedError::WorkerFailure {
                    worker_id,
                    message,
                }) => {
                    failed_partitions.push(task.partition.id);

                    let mut metrics = self
                        .metrics
                        .lock()
                        .map_err(|_| DistributedError::Synchronization(
                            "metrics registry poisoned".into(),
                        ))?;

                    metrics.partitions_failed =
                        metrics.partitions_failed.saturating_add(1);
                    metrics.worker_failures =
                        metrics.worker_failures.saturating_add(1);

                    drop(metrics);

                    // A production deployment may choose a retry policy here.
                    // We intentionally do not silently retry because retries can
                    // duplicate side effects unless the decoder is idempotent.
                    return Err(DistributedError::WorkerFailure {
                        worker_id,
                        message,
                    });
                }

                Err(error) => {
                    failed_partitions.push(task.partition.id);
                    return Err(error);
                }
            }
        }

        cancellation.check()?;

        let reconciliation = reconcile_boundaries(
            job.id,
            &corrections,
            &job.determinism,
        )?;

        let result = DistributedResult {
            job_id: job.id,
            corrections,
            logical_parity: reconciliation.logical_parity,
            reconciled_boundaries: reconciliation.reconciled_boundaries,
            failed_partitions,
        };

        result.validate()?;

        let elapsed = started.elapsed();

        let mut metrics = self
            .metrics
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "metrics registry poisoned".into(),
            ))?;

        metrics.jobs_completed = metrics.jobs_completed.saturating_add(1);
        metrics.total_wall_time_ns = metrics
            .total_wall_time_ns
            .saturating_add(elapsed.as_nanos().min(u64::MAX as u128) as u64);

        drop(metrics);

        active_job_guard.disarm();

        self.release_job(job.id)?;

        Ok(result)
    }

    fn select_worker(
        &self,
        determinism: &DeterminismConfig,
    ) -> Result<WorkerId> {
        let workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        let mut candidates: Vec<_> = workers
            .values()
            .filter(|worker| {
                worker.descriptor.state == WorkerState::Ready
                    && worker.active_tasks < worker.descriptor.max_concurrent_tasks
            })
            .collect();

        if candidates.is_empty() {
            return Err(DistributedError::WorkerUnavailable(0));
        }

        if determinism.enabled && determinism.stable_worker_assignment {
            candidates.sort_by_key(|worker| worker.descriptor.id);
        }

        Ok(candidates[0].descriptor.id)
    }

    fn mark_worker_busy(&self, worker_id: WorkerId) -> Result<()> {
        let mut workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(worker_id))?;

        if worker.descriptor.state == WorkerState::Failed
            || worker.descriptor.state == WorkerState::Offline
        {
            return Err(DistributedError::WorkerUnavailable(worker_id));
        }

        if worker.active_tasks >= worker.descriptor.max_concurrent_tasks {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::InFlightTasks,
                requested: worker.active_tasks as u64 + 1,
                limit: self.limits.max_in_flight_tasks,
            });
        }

        worker.active_tasks += 1;
        worker.descriptor.state = WorkerState::Busy;

        Ok(())
    }

    fn mark_worker_ready(&self, worker_id: WorkerId) -> Result<()> {
        let mut workers = self
            .workers
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "worker registry poisoned".into(),
            ))?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or(DistributedError::WorkerUnavailable(worker_id))?;

        worker.active_tasks = worker.active_tasks.saturating_sub(1);

        if worker.active_tasks == 0
            && worker.descriptor.state != WorkerState::Failed
            && worker.descriptor.state != WorkerState::Offline
        {
            worker.descriptor.state = WorkerState::Ready;
        }

        Ok(())
    }
}

/// Guard that ensures active jobs are released even when execution fails.
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
        let mut active = coordinator
            .active_jobs
            .lock()
            .map_err(|_| DistributedError::Synchronization(
                "active job registry poisoned".into(),
            ))?;

        if active.len() as u64 >= coordinator.limits.max_jobs {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Jobs,
                requested: active.len() as u64 + 1,
                limit: coordinator.limits.max_jobs,
            });
        }

        if !active.insert(job_id) {
            return Err(DistributedError::InvalidJob(format!(
                "job {} is already active",
                job_id
            )));
        }

        Ok(Self {
            coordinator,
            job_id,
            armed: true,
        })
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for ActiveJobGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.coordinator.release_job(self.job_id);
        }
    }
}

/// Boundary reconciliation result.
#[derive(Debug, Clone)]
struct Reconciliation {
    logical_parity: u8,
    reconciled_boundaries: Vec<BoundaryId>,
}

/// Reconcile partition-level corrections.
///
/// The reconciliation operation is intentionally deterministic:
/// - partitions are processed in ascending ID order;
/// - boundary IDs are processed in ascending order;
/// - duplicate boundary resolutions are rejected;
/// - logical parity is XOR-reduced.
///
/// A future distributed implementation can replace this with a true
/// cross-partition matching algorithm while preserving this contract.
fn reconcile_boundaries(
    job_id: JobId,
    corrections: &[PartitionCorrection],
    determinism: &DeterminismConfig,
) -> Result<Reconciliation> {
    if job_id == 0 {
        return Err(DistributedError::BoundaryReconciliationFailed(
            "job id cannot be zero".into(),
        ));
    }

    let mut ordered = corrections.to_vec();

    if determinism.enabled && determinism.stable_partition_order {
        ordered.sort_by_key(|correction| correction.partition_id);
    }

    let mut boundaries = BTreeSet::new();
    let mut logical_parity = 0u8;

    for correction in ordered {
        correction.validate()?;

        logical_parity ^= correction.logical_parity;

        for boundary in correction.resolved_boundaries {
            if !boundaries.insert(boundary) {
                return Err(DistributedError::BoundaryReconciliationFailed(
                    format!("boundary {} resolved more than once", boundary),
                ));
            }
        }
    }

    Ok(Reconciliation {
        logical_parity,
        reconciled_boundaries: boundaries.into_iter().collect(),
    })
}

/// A deterministic partition planner.
///
/// This creates rectangular partitions without allocating the entire QEC
/// lattice. It is therefore suitable as a planning primitive for very large
/// workloads.
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
        if x_chunks == 0 || y_chunks == 0 || z_chunks == 0 {
            return Err(DistributedError::InvalidInput(
                "partition dimensions must be greater than zero".into(),
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
            .and_then(|v| v.checked_mul(self.z_chunks))
            .ok_or_else(|| {
                DistributedError::ResourceLimitExceeded {
                    resource: ResourceKind::Partitions,
                    requested: u64::MAX,
                    limit: u64::MAX - 1,
                }
            })
    }

    /// Generate rectangular partition bounds for a lattice.
    ///
    /// `extent` contains the exclusive maximum in each dimension.
    pub fn plan(
        &self,
        extent: [(i64, i64); 3],
        limits: &DistributedLimits,
    ) -> Result<Vec<QecPartition>> {
        let total = self.partition_count()?;

        if total > limits.max_partitions_per_job {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Partitions,
                requested: total,
                limit: limits.max_partitions_per_job,
            });
        }

        for (axis, (min, max)) in extent.iter().enumerate() {
            if min >= max {
                return Err(DistributedError::InvalidInput(format!(
                    "invalid lattice extent on axis {}: [{}, {})",
                    axis, min, max
                )));
            }
        }

        let x_ranges = split_range(extent[0], self.x_chunks)?;
        let y_ranges = split_range(extent[1], self.y_chunks)?;
        let z_ranges = split_range(extent[2], self.z_chunks)?;

        let mut partitions = Vec::with_capacity(total as usize);

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

                    id = id.checked_add(1).ok_or_else(|| {
                        DistributedError::InvalidPartition(
                            "partition identifier overflow".into(),
                        )
                    })?;
                }
            }
        }

        connect_face_neighbors(&mut partitions);

        Ok(partitions)
    }
}

fn split_range(range: (i64, i64), chunks: u64) -> Result<Vec<(i64, i64)>> {
    if chunks == 0 || range.0 >= range.1 {
        return Err(DistributedError::InvalidInput(
            "invalid range or chunk count".into(),
        ));
    }

    let length = (range.1 as i128) - (range.0 as i128);
    let chunks_i128 = chunks as i128;

    if chunks_i128 > length {
        return Err(DistributedError::InvalidInput(format!(
            "cannot create {} non-empty chunks from range [{}, {})",
            chunks, range.0, range.1
        )));
    }

    let base = length / chunks_i128;
    let remainder = length % chunks_i128;

    let mut result = Vec::with_capacity(chunks as usize);
    let mut current = range.0 as i128;

    for index in 0..chunks_i128 {
        let size = base + if index < remainder { 1 } else { 0 };
        let next = current + size;

        let current_i64 = i64::try_from(current).map_err(|_| {
            DistributedError::InvalidInput("range conversion overflow".into())
        })?;

        let next_i64 = i64::try_from(next).map_err(|_| {
            DistributedError::InvalidInput("range conversion overflow".into())
        })?;

        result.push((current_i64, next_i64));
        current = next;
    }

    Ok(result)
}

fn ranges_touch(a: (i64, i64), b: (i64, i64)) -> bool {
    a.1 == b.0 || b.1 == a.0
}

fn overlap(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn face_neighbors(a: &QecPartition, b: &QecPartition) -> bool {
    let ax = a.bounds[0];
    let ay = a.bounds[1];
    let az = a.bounds[2];

    let bx = b.bounds[0];
    let by = b.bounds[1];
    let bz = b.bounds[2];

    (ranges_touch(ax, bx) && overlap(ay, by) && overlap(az, bz))
        || (ranges_touch(ay, by) && overlap(ax, bx) && overlap(az, bz))
        || (ranges_touch(az, bz) && overlap(ax, bx) && overlap(ay, by))
}

fn connect_face_neighbors(partitions: &mut [QecPartition]) {
    for left in 0..partitions.len() {
        for right in (left + 1)..partitions.len() {
            if face_neighbors(&partitions[left], &partitions[right]) {
                let left_id = partitions[left].id;
                let right_id = partitions[right].id;

                partitions[left].neighbors.insert(right_id);
                partitions[right].neighbors.insert(left_id);
            }
        }
    }
}

/// A bounded task queue.
///
/// This provides backpressure rather than allowing an unbounded number of
/// pending distributed tasks to accumulate.
#[derive(Debug)]
pub struct TaskQueue {
    queue: VecDeque<PartitionTask>,
    max_tasks: usize,
}

impl TaskQueue {
    pub fn new(max_tasks: usize) -> Result<Self> {
        if max_tasks == 0 {
            return Err(DistributedError::InvalidInput(
                "task queue capacity must be greater than zero".into(),
            ));
        }

        Ok(Self {
            queue: VecDeque::with_capacity(max_tasks.min(1024)),
            max_tasks,
        })
    }

    pub fn push(
        &mut self,
        task: PartitionTask,
    ) -> Result<()> {
        if self.queue.len() >= self.max_tasks {
            return Err(DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::InFlightTasks,
                requested: self.queue.len() as u64 + 1,
                limit: self.max_tasks as u64,
            });
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

/// Stable task ordering helper.
///
/// Stable ordering is important for deterministic distributed reductions.
pub fn stable_task_order(tasks: &mut [PartitionTask]) {
    tasks.sort_by_key(|task| (task.job_id, task.partition.id));
}

/// Calculate an approximate workload size without allocating decoder state.
pub fn estimate_partition_workload(
    partition: &QecPartition,
) -> Result<u64> {
    partition.validate(&DistributedLimits::default())?;

    let event_cost = (partition.events.len() as u64)
        .checked_mul(64)
        .ok_or_else(|| {
            DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Bytes,
                requested: u64::MAX,
                limit: u64::MAX,
            }
        })?;

    let boundary_cost = (partition.boundary_events.len() as u64)
        .checked_mul(96)
        .ok_or_else(|| {
            DistributedError::ResourceLimitExceeded {
                resource: ResourceKind::Bytes,
                requested: u64::MAX,
                limit: u64::MAX,
            }
        })?;

    event_cost
        .checked_add(boundary_cost)
        .ok_or_else(|| DistributedError::ResourceLimitExceeded {
            resource: ResourceKind::Bytes,
            requested: u64::MAX,
            limit: u64::MAX,
        })
}

/// Version of the distributed task/result contract.
pub const DISTRIBUTED_FORMAT_VERSION: u32 = 1;

/// Validate a distributed format version.
pub fn validate_format_version(found: u32) -> Result<()> {
    if found != DISTRIBUTED_FORMAT_VERSION {
        return Err(DistributedError::IncompatibleVersion {
            expected: DISTRIBUTED_FORMAT_VERSION,
            found,
        });
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
            max_events_per_partition: 1000,
            max_boundary_events_per_partition: 100,
            max_in_flight_tasks: 100,
            max_job_bytes: 1_000_000,
            max_worker_time: Duration::from_secs(10),
            max_job_time: Duration::from_secs(30),
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

    fn partition(id: PartitionId) -> QecPartition {
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
        assert!(partition(1).validate(&limits()).is_ok());
    }

    #[test]
    fn invalid_weight_is_rejected() {
        let mut p = partition(1);
        p.events[0].weight = f64::NAN;

        assert!(p.validate(&limits()).is_err());
    }

    #[test]
    fn duplicate_event_ids_are_rejected() {
        let mut p = partition(1);
        p.events.push(p.events[0].clone());

        assert!(p.validate(&limits()).is_err());
    }

    #[test]
    fn planner_creates_expected_number_of_partitions() {
        let planner = PartitionPlanner::new(2, 2, 2).unwrap();

        let partitions = planner
            .plan([(0, 10), (0, 10), (0, 10)], &limits())
            .unwrap();

        assert_eq!(partitions.len(), 8);
    }

    #[test]
    fn planner_connects_face_neighbors() {
        let planner = PartitionPlanner::new(2, 1, 1).unwrap();

        let partitions = planner
            .plan([(0, 10), (0, 10), (0, 10)], &limits())
            .unwrap();

        assert_eq!(partitions.len(), 2);
        assert!(partitions[0].neighbors.contains(&partitions[1].id));
        assert!(partitions[1].neighbors.contains(&partitions[0].id));
    }

    #[test]
    fn planner_rejects_too_many_chunks() {
        let planner = PartitionPlanner::new(11, 1, 1).unwrap();

        assert!(planner
            .plan([(0, 10), (0, 10), (0, 10)], &limits())
            .is_err());
    }

    #[test]
    fn cancellation_is_cooperative() {
        let token = CancellationToken::new();

        assert!(token.check().is_ok());

        token.cancel();

        assert_eq!(
            token.check(),
            Err(DistributedError::Cancelled)
        );
    }

    #[test]
    fn task_queue_is_bounded() {
        let mut queue = TaskQueue::new(1).unwrap();

        let task = PartitionTask {
            job_id: 1,
            partition: partition(1),
            determinism: DeterminismConfig::default(),
        };

        assert!(queue.push(task.clone()).is_ok());
        assert!(queue.push(task).is_err());
    }

    #[test]
    fn deterministic_task_order_is_stable() {
        let mut tasks = vec![
            PartitionTask {
                job_id: 1,
                partition: partition(3),
                determinism: DeterminismConfig::default(),
            },
            PartitionTask {
                job_id: 1,
                partition: partition(1),
                determinism: DeterminismConfig::default(),
            },
            PartitionTask {
                job_id: 1,
                partition: partition(2),
                determinism: DeterminismConfig::default(),
            },
        ];

        stable_task_order(&mut tasks);

        assert_eq!(tasks[0].partition.id, 1);
        assert_eq!(tasks[1].partition.id, 2);
        assert_eq!(tasks[2].partition.id, 3);
    }

    #[test]
    fn identity_decoder_executes() {
        let coordinator = DistributedCoordinator::new(limits()).unwrap();

        coordinator.register_worker(worker()).unwrap();

        let job = DistributedJob {
            id: 1,
            partitions: vec![partition(1)],
            mode: ExecutionMode::Distributed,
            determinism: DeterminismConfig::default(),
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
        let coordinator = DistributedCoordinator::new(limits()).unwrap();

        coordinator.register_worker(worker()).unwrap();

        let job = DistributedJob {
            id: 1,
            partitions: Vec::new(),
            mode: ExecutionMode::Distributed,
            determinism: DeterminismConfig::default(),
            metadata: BTreeMap::new(),
        };

        assert!(coordinator
            .execute(
                job,
                &IdentityDecoder,
                CancellationToken::new()
            )
            .is_err());
    }

    #[test]
    fn format_version_is_checked() {
        assert!(validate_format_version(DISTRIBUTED_FORMAT_VERSION).is_ok());
        assert!(validate_format_version(DISTRIBUTED_FORMAT_VERSION + 1).is_err());
    }

    #[test]
    fn workload_estimation_is_nonzero() {
        let p = partition(1);

        assert!(estimate_partition_workload(&p).unwrap() > 0);
    }
}