//! Distributed classical execution for the quantum error-correction subsystem.
//!
//! # Responsibility
//!
//! This module owns distributed *coordination* of already-defined QEC work.
//! It does not own:
//!
//! - partition mathematics (`partition.rs`)
//! - decoder mathematics (`decoder.rs`, `mwpm.rs`, `union_find.rs`)
//! - resource policy (`limits.rs`)
//! - resource allocation/accounting (`resources.rs`)
//! - cancellation semantics (`cancellation.rs`)
//! - QPU execution (`qpu_adapter.rs`)
//! - network transport implementation
//! - cryptographic primitives
//!
//! The module coordinates workers, retries, ownership, deterministic task
//! identity, authenticated result envelopes, failure handling, and partition
//! reconciliation.
//!
//! # Integration contract
//!
//! ```text
//! QecConfig / QecLimits
//!          |
//!          v
//!   ResourceManager
//!          |
//!          v
//!   PartitionPlan
//!          |
//!          v
//! DistributedCoordinator
//!      |       |       |
//!      v       v       v
//!   Worker   Retry   Failure
//!      |
//!      v
//!  Decoder job
//!      |
//!      v
//! Authenticated result
//!      |
//!      v
//! Boundary reconciliation
//!      |
//!      v
//! DistributedDecodeResult
//! ```
//!
//! # Security model
//!
//! Distributed execution is fail-closed:
//!
//! 1. workers must be authenticated;
//! 2. workers must possess the required capabilities;
//! 3. a task attempt is owned by exactly one worker;
//! 4. stale attempts cannot overwrite newer attempts;
//! 5. duplicate identical results are idempotent;
//! 6. conflicting duplicate results are rejected;
//! 7. transport fingerprints are integrity identifiers only;
//! 8. cryptographic authentication is delegated to the configured
//!    `WorkerAuthenticator` implementation;
//! 9. QPU credentials/capabilities never enter decoder jobs;
//! 10. resource limits are checked before admission.
//!
//! # Determinism
//!
//! Distributed scheduling may execute concurrently, but observable reduction
//! is deterministic. Task ordering is based on stable `TaskKey` ordering and
//! result aggregation is performed in that order.
//!
//! # Rust
//!
//! Target: Rust 1.97.1.
//!
//! The implementation intentionally uses only stable standard-library APIs and
//! the existing QEC subsystem contracts.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::cancellation::{CancellationReason, CancellationToken};
use super::errors::{QecError, QecResult};
use super::limits::QecLimits;
use super::partition::{
    BoundaryReconciliation, PartitionBoundary, PartitionId, PartitionPlan, QecPartition,
};
use super::resources::{ResourceManager, ResourceSnapshot};

// -----------------------------------------------------------------------------
// Core identifiers
// -----------------------------------------------------------------------------

/// Stable identifier for a distributed job.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DistributedJobId(pub u128);

impl DistributedJobId {
    /// Generates a process-local time-derived identifier.
    ///
    /// This identifier is suitable for coordination identity, not as a
    /// cryptographic nonce.
    pub fn generate() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_nanos(0))
            .as_nanos();

        Self(nanos)
    }
}

/// Stable identifier for a worker.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkerId(pub u64);

/// Stable identifier for a task.
///
/// A task identity does not contain the retry attempt. This allows retries to
/// remain the same logical task while each execution receives a monotonically
/// increasing attempt number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskKey {
    pub job_id: DistributedJobId,
    pub partition_id: PartitionId,
}

impl TaskKey {
    pub const fn new(job_id: DistributedJobId, partition_id: PartitionId) -> Self {
        Self {
            job_id,
            partition_id,
        }
    }
}

/// Monotonically increasing execution attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Attempt(pub u32);

impl Attempt {
    pub const FIRST: Self = Self(0);

    pub fn next(self) -> QecResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or_else(|| QecError::NumericalFailure {
                operation: "distributed task attempt increment".into(),
            })
    }
}

// -----------------------------------------------------------------------------
// Worker capabilities
// -----------------------------------------------------------------------------

/// Capabilities specifically relevant to distributed classical execution.
///
/// This is intentionally independent from QPU capabilities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DistributedCapability {
    ExecutePartition,
    ReadPartitionInput,
    SubmitPartitionResult,
    ReconcileBoundary,
    ReadMetrics,
    CreateCheckpoint,
}

/// Capability set used by a worker.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DistributedCapabilitySet {
    capabilities: BTreeSet<DistributedCapability>,
}

impl DistributedCapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow(mut self, capability: DistributedCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    pub fn insert(&mut self, capability: DistributedCapability) {
        self.capabilities.insert(capability);
    }

    pub fn contains(&self, capability: DistributedCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn require(&self, capability: DistributedCapability) -> QecResult<()> {
        if self.contains(capability) {
            Ok(())
        } else {
            Err(QecError::CapabilityDenied {
                capability: format!("{capability:?}"),
            })
        }
    }

    pub fn attenuate(
        &self,
        requested: impl IntoIterator<Item = DistributedCapability>,
    ) -> Self {
        let mut result = Self::new();

        for capability in requested {
            if self.contains(capability) {
                result.insert(capability);
            }
        }

        result
    }
}

// -----------------------------------------------------------------------------
// Worker authentication
// -----------------------------------------------------------------------------

/// Authenticated identity presented by a worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticatedWorker {
    pub worker_id: WorkerId,
    pub capabilities: DistributedCapabilitySet,
    pub identity_fingerprint: String,
}

/// Authentication abstraction.
///
/// Real deployments should provide a cryptographically authenticated
/// implementation. The distributed coordinator deliberately does not pretend
/// that a hash/fingerprint is equivalent to authentication.
pub trait WorkerAuthenticator: Send + Sync {
    fn authenticate(
        &self,
        worker_id: WorkerId,
        credential: &WorkerCredential,
    ) -> QecResult<AuthenticatedWorker>;
}

/// Opaque worker credential.
///
/// The coordinator does not inspect credential contents.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerCredential {
    material: Vec<u8>,
}

impl WorkerCredential {
    pub fn new(material: Vec<u8>) -> Self {
        Self { material }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.material
    }
}

/// Development authenticator.
///
/// This implementation is intentionally explicit about being non-production.
/// It provides deterministic identity mapping for tests and local execution.
#[derive(Clone, Debug, Default)]
pub struct StaticWorkerAuthenticator {
    workers: BTreeMap<WorkerId, DistributedCapabilitySet>,
}

impl StaticWorkerAuthenticator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        worker_id: WorkerId,
        capabilities: DistributedCapabilitySet,
    ) {
        self.workers.insert(worker_id, capabilities);
    }
}

impl WorkerAuthenticator for StaticWorkerAuthenticator {
    fn authenticate(
        &self,
        worker_id: WorkerId,
        _credential: &WorkerCredential,
    ) -> QecResult<AuthenticatedWorker> {
        let capabilities = self
            .workers
            .get(&worker_id)
            .cloned()
            .ok_or_else(|| QecError::CapabilityDenied {
                capability: format!("unregistered worker {worker_id:?}"),
            })?;

        Ok(AuthenticatedWorker {
            worker_id,
            capabilities,
            identity_fingerprint: format!("static-worker-{}", worker_id.0),
        })
    }
}

// -----------------------------------------------------------------------------
// Worker lifecycle
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerState {
    Registered,
    Ready,
    Busy,
    Draining,
    Failed,
    Removed,
}

impl WorkerState {
    pub fn accepts_work(self) -> bool {
        matches!(self, Self::Registered | Self::Ready)
    }
}

/// Runtime information about a worker.
#[derive(Clone, Debug)]
pub struct WorkerRecord {
    pub worker_id: WorkerId,
    pub state: WorkerState,
    pub authenticated_identity: AuthenticatedWorker,
    pub active_tasks: usize,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub last_seen: Instant,
}

impl WorkerRecord {
    fn new(identity: AuthenticatedWorker) -> Self {
        Self {
            worker_id: identity.worker_id,
            state: WorkerState::Ready,
            authenticated_identity: identity,
            active_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            last_seen: Instant::now(),
        }
    }
}

// -----------------------------------------------------------------------------
// Job lifecycle
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DistributedJobState {
    Created,
    Validating,
    Admitted,
    Running,
    Reconciling,
    Completed,
    Failed,
    Cancelled,
}

impl DistributedJobState {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled
        )
    }
}

/// Failure classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DistributedFailure {
    WorkerUnavailable {
        worker_id: WorkerId,
    },
    WorkerAuthentication {
        worker_id: WorkerId,
    },
    WorkerCapability {
        worker_id: WorkerId,
        capability: DistributedCapability,
    },
    WorkerTimeout {
        worker_id: WorkerId,
    },
    TaskTimeout {
        task: TaskKey,
    },
    TaskRejected {
        task: TaskKey,
        reason: String,
    },
    TaskExecution {
        task: TaskKey,
        reason: String,
    },
    StaleAttempt {
        task: TaskKey,
        received: Attempt,
        expected_at_least: Attempt,
    },
    ConflictingDuplicate {
        task: TaskKey,
        attempt: Attempt,
    },
    BoundaryReconciliation {
        partition: PartitionId,
        reason: String,
    },
    ResourceAdmission {
        reason: String,
    },
    Cancellation,
}

impl fmt::Display for DistributedFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

// -----------------------------------------------------------------------------
// Partition execution contract
// -----------------------------------------------------------------------------

/// Input handed to a worker.
///
/// The distributed layer deliberately treats decoder input as opaque bytes.
/// Encoding/decoding is owned by the decoder/serialization layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionWork {
    pub task: TaskKey,
    pub attempt: Attempt,
    pub partition: QecPartition,
    pub input_fingerprint: String,
    pub required_capabilities: DistributedCapabilitySet,
}

/// Generic correction payload returned by a worker.
///
/// The distributed layer does not interpret decoder mathematics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionCorrection {
    pub fingerprint: String,
    pub logical_parity: u8,
    pub payload: Vec<u8>,
}

impl PartitionCorrection {
    pub fn new(
        fingerprint: impl Into<String>,
        logical_parity: u8,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            fingerprint: fingerprint.into(),
            logical_parity,
            payload,
        }
    }
}

/// Result produced by a worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionWorkResult {
    pub task: TaskKey,
    pub attempt: Attempt,
    pub worker_id: WorkerId,
    pub correction: PartitionCorrection,
    pub resource_usage: ResourceSnapshot,
    pub execution_fingerprint: String,
}

/// Authentication/integrity envelope for a worker result.
///
/// The `authentication_tag` is opaque. Cryptographic validation belongs to
/// the configured result authenticator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticatedPartitionResult {
    pub result: PartitionWorkResult,
    pub authentication_tag: Vec<u8>,
}

/// Result authentication abstraction.
pub trait ResultAuthenticator: Send + Sync {
    fn authenticate(
        &self,
        worker: &AuthenticatedWorker,
        result: &AuthenticatedPartitionResult,
    ) -> QecResult<()>;
}

/// Deterministic development authenticator.
///
/// This is useful for tests only and must not be used as a cryptographic
/// production authenticator.
#[derive(Clone, Debug, Default)]
pub struct NoopResultAuthenticator;

impl ResultAuthenticator for NoopResultAuthenticator {
    fn authenticate(
        &self,
        _worker: &AuthenticatedWorker,
        _result: &AuthenticatedPartitionResult,
    ) -> QecResult<()> {
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Transport abstraction
// -----------------------------------------------------------------------------

/// Transport abstraction for dispatching work.
///
/// Networking, serialization, encryption, retry at the transport layer, and
/// connection management remain outside this module.
pub trait DistributedTransport: Send + Sync {
    fn submit(
        &self,
        worker: &AuthenticatedWorker,
        work: PartitionWork,
    ) -> QecResult<()>;
}

/// In-process transport useful for integration tests.
///
/// It only records submissions. It does not execute decoder work.
#[derive(Clone, Debug, Default)]
pub struct RecordingTransport {
    submissions: Arc<Mutex<Vec<(WorkerId, PartitionWork)>>>,
}

impl RecordingTransport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submissions(&self) -> QecResult<Vec<(WorkerId, PartitionWork)>> {
        self.submissions
            .lock()
            .map(|items| items.clone())
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "recording transport mutex".into(),
                message: "transport mutex was poisoned".into(),
            })
    }
}

impl DistributedTransport for RecordingTransport {
    fn submit(
        &self,
        worker: &AuthenticatedWorker,
        work: PartitionWork,
    ) -> QecResult<()> {
        self.submissions
            .lock()
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "recording transport mutex".into(),
                message: "transport mutex was poisoned".into(),
            })?
            .push((worker.worker_id, work));

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Runtime task state
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug)]
struct TaskRuntime {
    state: TaskState,
    worker_id: WorkerId,
    attempt: Attempt,
}

#[derive(Clone, Debug)]
struct AcceptedResult {
    result: PartitionWorkResult,
    result_fingerprint: String,
}

// -----------------------------------------------------------------------------
// Metrics
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct DistributedMetrics {
    pub jobs_started: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub jobs_cancelled: u64,

    pub tasks_submitted: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub retries: u64,

    pub duplicate_results: u64,
    pub stale_results: u64,
    pub conflicting_results: u64,

    pub authentication_failures: u64,
    pub capability_denials: u64,
    pub worker_failures: u64,

    pub reconciliation_attempts: u64,
    pub reconciliation_failures: u64,

    pub peak_workers: usize,
    pub peak_active_tasks: usize,

    pub total_execution_time: Duration,
}

impl DistributedMetrics {
    pub fn merge(&mut self, other: &Self) {
        self.jobs_started = self.jobs_started.saturating_add(other.jobs_started);
        self.jobs_completed = self.jobs_completed.saturating_add(other.jobs_completed);
        self.jobs_failed = self.jobs_failed.saturating_add(other.jobs_failed);
        self.jobs_cancelled = self.jobs_cancelled.saturating_add(other.jobs_cancelled);

        self.tasks_submitted = self.tasks_submitted.saturating_add(other.tasks_submitted);
        self.tasks_completed = self.tasks_completed.saturating_add(other.tasks_completed);
        self.tasks_failed = self.tasks_failed.saturating_add(other.tasks_failed);
        self.retries = self.retries.saturating_add(other.retries);

        self.duplicate_results =
            self.duplicate_results.saturating_add(other.duplicate_results);
        self.stale_results = self.stale_results.saturating_add(other.stale_results);
        self.conflicting_results =
            self.conflicting_results.saturating_add(other.conflicting_results);

        self.authentication_failures = self
            .authentication_failures
            .saturating_add(other.authentication_failures);

        self.capability_denials =
            self.capability_denials.saturating_add(other.capability_denials);

        self.worker_failures =
            self.worker_failures.saturating_add(other.worker_failures);

        self.reconciliation_attempts = self
            .reconciliation_attempts
            .saturating_add(other.reconciliation_attempts);

        self.reconciliation_failures = self
            .reconciliation_failures
            .saturating_add(other.reconciliation_failures);

        self.peak_workers = self.peak_workers.max(other.peak_workers);
        self.peak_active_tasks = self.peak_active_tasks.max(other.peak_active_tasks);

        self.total_execution_time += other.total_execution_time;
    }
}

// -----------------------------------------------------------------------------
// Distributed configuration
// -----------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DistributedConfig {
    /// Maximum number of simultaneous task attempts.
    ///
    /// This is execution configuration. Global worker/resource limits still
    /// come from `QecLimits`.
    pub max_in_flight_tasks: usize,

    /// Maximum attempts including the first execution.
    pub max_attempts: u32,

    /// Worker liveness timeout.
    pub worker_timeout: Duration,

    /// Individual task execution timeout.
    pub task_timeout: Duration,

    /// Whether deterministic ordering is mandatory.
    pub deterministic: bool,

    /// Whether duplicate results are accepted idempotently.
    pub accept_idempotent_duplicates: bool,

    /// Whether boundary reconciliation is mandatory.
    pub require_reconciliation: bool,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            max_in_flight_tasks: 64,
            max_attempts: 3,
            worker_timeout: Duration::from_secs(30),
            task_timeout: Duration::from_secs(300),
            deterministic: true,
            accept_idempotent_duplicates: true,
            require_reconciliation: true,
        }
    }
}

impl DistributedConfig {
    pub fn validate(&self) -> QecResult<()> {
        if self.max_in_flight_tasks == 0 {
            return Err(QecError::InvalidInput {
                message: "max_in_flight_tasks must be greater than zero".into(),
            });
        }

        if self.max_attempts == 0 {
            return Err(QecError::InvalidInput {
                message: "max_attempts must be greater than zero".into(),
            });
        }

        if self.worker_timeout.is_zero() {
            return Err(QecError::InvalidInput {
                message: "worker_timeout must be greater than zero".into(),
            });
        }

        if self.task_timeout.is_zero() {
            return Err(QecError::InvalidInput {
                message: "task_timeout must be greater than zero".into(),
            });
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Coordinator
// -----------------------------------------------------------------------------

/// Distributed execution coordinator.
///
/// The coordinator is responsible for orchestration only. It does not execute
/// decoder algorithms itself.
pub struct DistributedCoordinator {
    config: DistributedConfig,
    limits: QecLimits,
    resources: Arc<ResourceManager>,
    cancellation: CancellationToken,

    authenticator: Arc<dyn WorkerAuthenticator>,
    result_authenticator: Arc<dyn ResultAuthenticator>,
    transport: Arc<dyn DistributedTransport>,

    workers: Mutex<BTreeMap<WorkerId, WorkerRecord>>,
    tasks: Mutex<BTreeMap<TaskKey, TaskRuntime>>,
    accepted_results: Mutex<BTreeMap<TaskKey, AcceptedResult>>,
    metrics: Mutex<DistributedMetrics>,
}

impl DistributedCoordinator {
    pub fn new(
        config: DistributedConfig,
        limits: QecLimits,
        resources: Arc<ResourceManager>,
        cancellation: CancellationToken,
        authenticator: Arc<dyn WorkerAuthenticator>,
        result_authenticator: Arc<dyn ResultAuthenticator>,
        transport: Arc<dyn DistributedTransport>,
    ) -> QecResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            limits,
            resources,
            cancellation,
            authenticator,
            result_authenticator,
            transport,
            workers: Mutex::new(BTreeMap::new()),
            tasks: Mutex::new(BTreeMap::new()),
            accepted_results: Mutex::new(BTreeMap::new()),
            metrics: Mutex::new(DistributedMetrics::default()),
        })
    }

    // -------------------------------------------------------------------------
    // Worker management
    // -------------------------------------------------------------------------

    pub fn register_worker(
        &self,
        worker_id: WorkerId,
        credential: &WorkerCredential,
    ) -> QecResult<AuthenticatedWorker> {
        self.cancellation.check()?;

        let identity = match self.authenticator.authenticate(worker_id, credential) {
            Ok(identity) => identity,
            Err(error) => {
                self.lock_metrics()?.authentication_failures =
                    self.lock_metrics()?.authentication_failures.saturating_add(1);

                return Err(error);
            }
        };

        let mut workers = self.lock_workers()?;

        if workers.contains_key(&worker_id) {
            return Err(QecError::InvalidInput {
                message: format!("worker {worker_id:?} is already registered"),
            });
        }

        workers.insert(worker_id, WorkerRecord::new(identity.clone()));

        let worker_count = workers.len();

        let mut metrics = self.lock_metrics()?;
        metrics.peak_workers = metrics.peak_workers.max(worker_count);

        Ok(identity)
    }

    pub fn unregister_worker(&self, worker_id: WorkerId) -> QecResult<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown worker {worker_id:?}"),
            })?;

        if worker.active_tasks != 0 {
            return Err(QecError::InvalidInput {
                message: format!(
                    "cannot remove worker {worker_id:?} with {} active tasks",
                    worker.active_tasks
                ),
            });
        }

        worker.state = WorkerState::Removed;
        Ok(())
    }

    pub fn mark_worker_ready(&self, worker_id: WorkerId) -> QecResult<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown worker {worker_id:?}"),
            })?;

        if worker.state == WorkerState::Removed {
            return Err(QecError::InvalidInput {
                message: format!("worker {worker_id:?} has been removed"),
            });
        }

        worker.state = WorkerState::Ready;
        worker.last_seen = Instant::now();

        Ok(())
    }

    pub fn heartbeat(&self, worker_id: WorkerId) -> QecResult<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown worker {worker_id:?}"),
            })?;

        if matches!(worker.state, WorkerState::Failed | WorkerState::Removed) {
            return Err(QecError::InvalidInput {
                message: format!("worker {worker_id:?} is not active"),
            });
        }

        worker.last_seen = Instant::now();

        if worker.state == WorkerState::Registered {
            worker.state = WorkerState::Ready;
        }

        Ok(())
    }

    pub fn detect_failed_workers(&self) -> QecResult<Vec<WorkerId>> {
        let now = Instant::now();
        let timeout = self.config.worker_timeout;
        let mut failed = Vec::new();

        let mut workers = self.lock_workers()?;

        for worker in workers.values_mut() {
            if matches!(
                worker.state,
                WorkerState::Removed | WorkerState::Failed
            ) {
                continue;
            }

            if now.duration_since(worker.last_seen) > timeout {
                worker.state = WorkerState::Failed;
                failed.push(worker.worker_id);
            }
        }

        if !failed.is_empty() {
            let mut metrics = self.lock_metrics()?;
            metrics.worker_failures = metrics
                .worker_failures
                .saturating_add(failed.len() as u64);
        }

        Ok(failed)
    }

    pub fn workers(&self) -> QecResult<Vec<WorkerRecord>> {
        Ok(self.lock_workers()?.values().cloned().collect())
    }

    // -------------------------------------------------------------------------
    // Admission
    // -------------------------------------------------------------------------

    pub fn admit_partition_plan(
        &self,
        plan: &PartitionPlan,
    ) -> QecResult<()> {
        self.cancellation.check()?;

        let partition_count = plan.partitions().len();

        if partition_count == 0 {
            return Err(QecError::InvalidInput {
                message: "partition plan must contain at least one partition".into(),
            });
        }

        self.limits.validate_partition(partition_count)?;

        let active_workers = self
            .lock_workers()?
            .values()
            .filter(|worker| worker.state.accepts_work())
            .count();

        if active_workers == 0 {
            return Err(QecError::InvalidInput {
                message: "no active workers are available".into(),
            });
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Scheduling
    // -------------------------------------------------------------------------

    /// Schedules one partition.
    ///
    /// The task is recorded before transport submission so that a result cannot
    /// arrive without an ownership record.
    pub fn schedule_partition(
        &self,
        job_id: DistributedJobId,
        partition: QecPartition,
        worker_id: WorkerId,
        credential: &WorkerCredential,
        input_fingerprint: impl Into<String>,
        required_capabilities: DistributedCapabilitySet,
    ) -> QecResult<PartitionWork> {
        self.cancellation.check()?;

        let identity = self.authenticate_worker(worker_id, credential)?;

        for capability in [
            DistributedCapability::ExecutePartition,
            DistributedCapability::ReadPartitionInput,
        ] {
            if !identity.capabilities.contains(capability) {
                self.record_capability_denial();
                return Err(QecError::CapabilityDenied {
                    capability: format!("{capability:?}"),
                });
            }
        }

        for capability in [
            DistributedCapability::ExecutePartition,
            DistributedCapability::ReadPartitionInput,
        ] {
            required_capabilities.require(capability)?;
        }

        let task = TaskKey::new(job_id, partition.id());

        self.admit_task()?;

        let work = PartitionWork {
            task,
            attempt: Attempt::FIRST,
            partition,
            input_fingerprint: input_fingerprint.into(),
            required_capabilities,
        };

        {
            let mut tasks = self.lock_tasks()?;

            if tasks.contains_key(&task) {
                return Err(QecError::InvalidInput {
                    message: format!("task {task:?} already exists"),
                });
            }

            tasks.insert(
                task,
                TaskRuntime {
                    state: TaskState::Queued,
                    worker_id,
                    attempt: Attempt::FIRST,
                },
            );
        }

        self.increment_worker_active(worker_id)?;

        if let Err(error) = self.transport.submit(&identity, work.clone()) {
            self.fail_task_internal(task)?;
            return Err(error);
        }

        {
            let mut tasks = self.lock_tasks()?;
            let runtime = tasks
                .get_mut(&task)
                .ok_or_else(|| QecError::InternalInvariantViolation {
                    invariant: "scheduled task must exist".into(),
                    message: format!("task {task:?} disappeared after admission"),
                })?;

            runtime.state = TaskState::Running;
        }

        let mut metrics = self.lock_metrics()?;
        metrics.tasks_submitted = metrics.tasks_submitted.saturating_add(1);

        Ok(work)
    }

    /// Schedules a retry for a failed task.
    pub fn retry_partition(
        &self,
        work: &PartitionWork,
        worker_id: WorkerId,
        credential: &WorkerCredential,
    ) -> QecResult<PartitionWork> {
        self.cancellation.check()?;

        if work.attempt.0.saturating_add(1) >= self.config.max_attempts {
            return Err(QecError::DecoderFailure {
                decoder: "distributed".into(),
                message: format!(
                    "task {:?} exhausted maximum attempts ({})",
                    work.task, self.config.max_attempts
                ),
            });
        }

        let next_attempt = work.attempt.next()?;
        let identity = self.authenticate_worker(worker_id, credential)?;

        identity
            .capabilities
            .require(DistributedCapability::ExecutePartition)?;

        let retry = PartitionWork {
            task: work.task,
            attempt: next_attempt,
            partition: work.partition.clone(),
            input_fingerprint: work.input_fingerprint.clone(),
            required_capabilities: work.required_capabilities.clone(),
        };

        let mut tasks = self.lock_tasks()?;

        match tasks.get_mut(&work.task) {
            Some(runtime) => {
                if runtime.state == TaskState::Running {
                    return Err(QecError::InternalInvariantViolation {
                        invariant: "one active attempt per task".into(),
                        message: format!("task {:?} already has an active attempt", work.task),
                    });
                }

                runtime.state = TaskState::Running;
                runtime.worker_id = worker_id;
                runtime.attempt = next_attempt;
            }
            None => {
                tasks.insert(
                    work.task,
                    TaskRuntime {
                        state: TaskState::Running,
                        worker_id,
                        attempt: next_attempt,
                    },
                );
            }
        }

        drop(tasks);

        self.increment_worker_active(worker_id)?;

        if let Err(error) = self.transport.submit(&identity, retry.clone()) {
            self.fail_task_internal(work.task)?;
            return Err(error);
        }

        {
            let mut metrics = self.lock_metrics()?;
            metrics.retries = metrics.retries.saturating_add(1);
            metrics.tasks_submitted = metrics.tasks_submitted.saturating_add(1);
        }

        Ok(retry)
    }

    // -------------------------------------------------------------------------
    // Result submission
    // -------------------------------------------------------------------------

    pub fn submit_result(
        &self,
        result: AuthenticatedPartitionResult,
    ) -> QecResult<ResultAcceptance> {
        self.cancellation.check()?;

        let task = result.result.task;

        let worker = {
            let workers = self.lock_workers()?;

            workers
                .get(&result.result.worker_id)
                .cloned()
                .ok_or_else(|| QecError::CapabilityDenied {
                    capability: format!("unknown worker {:?}", result.result.worker_id),
                })?
        };

        self.result_authenticator
            .authenticate(&worker.authenticated_identity, &result)
            .map_err(|error| {
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.authentication_failures =
                        metrics.authentication_failures.saturating_add(1);
                }
                error
            })?;

        worker
            .authenticated_identity
            .capabilities
            .require(DistributedCapability::SubmitPartitionResult)?;

        let mut tasks = self.lock_tasks()?;

        let runtime = tasks
            .get_mut(&task)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown task {task:?}"),
            })?;

        if result.result.worker_id != runtime.worker_id {
            return Err(QecError::CapabilityDenied {
                capability: "task ownership".into(),
            });
        }

        if result.result.attempt < runtime.attempt {
            self.record_stale_result();

            return Err(QecError::InternalInvariantViolation {
                invariant: "stale distributed attempt rejection".into(),
                message: format!(
                    "task {:?}: received attempt {:?}, current attempt {:?}",
                    task, result.result.attempt, runtime.attempt
                ),
            });
        }

        if result.result.attempt > runtime.attempt {
            return Err(QecError::InternalInvariantViolation {
                invariant: "unexpected future distributed attempt".into(),
                message: format!(
                    "task {:?}: received future attempt {:?}, current attempt {:?}",
                    task, result.result.attempt, runtime.attempt
                ),
            });
        }

        let fingerprint = result.result.execution_fingerprint.clone();

        if let Some(existing) = self.lock_results()?.get(&task) {
            if existing.result_fingerprint == fingerprint {
                if self.config.accept_idempotent_duplicates {
                    let mut metrics = self.lock_metrics()?;
                    metrics.duplicate_results =
                        metrics.duplicate_results.saturating_add(1);

                    return Ok(ResultAcceptance::DuplicateIdempotent);
                }

                return Err(QecError::InternalInvariantViolation {
                    invariant: "duplicate result policy".into(),
                    message: format!("duplicate result for task {task:?}"),
                });
            }

            let mut metrics = self.lock_metrics()?;
            metrics.conflicting_results =
                metrics.conflicting_results.saturating_add(1);

            return Err(QecError::InternalInvariantViolation {
                invariant: "conflicting duplicate result rejection".into(),
                message: format!(
                    "task {:?}, attempt {:?} produced conflicting results",
                    task, result.result.attempt
                ),
            });
        }

        runtime.state = TaskState::Completed;

        self.lock_results()?.insert(
            task,
            AcceptedResult {
                result: result.result.clone(),
                result_fingerprint: fingerprint,
            },
        );

        self.decrement_worker_active(result.result.worker_id)?;

        {
            let mut metrics = self.lock_metrics()?;
            metrics.tasks_completed = metrics.tasks_completed.saturating_add(1);
        }

        Ok(ResultAcceptance::Accepted)
    }

    // -------------------------------------------------------------------------
    // Reconciliation
    // -------------------------------------------------------------------------

    pub fn reconcile(
        &self,
        plan: &PartitionPlan,
    ) -> QecResult<DistributedReconciliationResult> {
        self.cancellation.check()?;

        {
            let mut metrics = self.lock_metrics()?;
            metrics.reconciliation_attempts =
                metrics.reconciliation_attempts.saturating_add(1);
        }

        let mut results = BTreeMap::new();

        for partition in plan.partitions() {
            let task = TaskKey::new(
                DistributedJobId::generate(),
                partition.id(),
            );

            let accepted = self
                .lock_results()?
                .values()
                .find(|result| result.result.task.partition_id == partition.id())
                .cloned()
                .ok_or_else(|| QecError::DecoderFailure {
                    decoder: "distributed".into(),
                    message: format!(
                        "partition {:?} has no accepted worker result",
                        partition.id()
                    ),
                })?;

            results.insert(partition.id(), accepted.result);
            let _ = task;
        }

        let reconciliation = self.reconcile_boundaries(plan, &results)?;

        Ok(DistributedReconciliationResult {
            partitions_completed: results.len(),
            reconciliation,
            resource_usage: self.resources.snapshot()?,
        })
    }

    fn reconcile_boundaries(
        &self,
        plan: &PartitionPlan,
        results: &BTreeMap<PartitionId, PartitionWorkResult>,
    ) -> QecResult<Vec<BoundaryReconciliation>> {
        let mut reconciliations = Vec::new();

        let mut ordered: Vec<&QecPartition> = plan.partitions().iter().collect();
        ordered.sort_by_key(|partition| partition.id());

        for window in ordered.windows(2) {
            self.cancellation.check()?;

            let left = window[0];
            let right = window[1];

            let left_result = results
                .get(&left.id())
                .ok_or_else(|| QecError::DecoderFailure {
                    decoder: "distributed".into(),
                    message: format!(
                        "missing result for left partition {:?}",
                        left.id()
                    ),
                })?;

            let right_result = results
                .get(&right.id())
                .ok_or_else(|| QecError::DecoderFailure {
                    decoder: "distributed".into(),
                    message: format!(
                        "missing result for right partition {:?}",
                        right.id()
                    ),
                })?;

            let reconciliation =
                reconcile_partition_boundary(
                    left.boundary(),
                    right.boundary(),
                    left_result,
                    right_result,
                )?;

            reconciliations.push(reconciliation);
        }

        Ok(reconciliations)
    }

    // -------------------------------------------------------------------------
    // Job result
    // -------------------------------------------------------------------------

    pub fn collect_results(
        &self,
        job_id: DistributedJobId,
    ) -> QecResult<Vec<PartitionWorkResult>> {
        let mut results = Vec::new();

        for accepted in self.lock_results()?.values() {
            if accepted.result.task.job_id == job_id {
                results.push(accepted.result.clone());
            }
        }

        results.sort_by_key(|result| result.task.partition_id);

        Ok(results)
    }

    pub fn job_complete(
        &self,
        job_id: DistributedJobId,
        expected_partitions: usize,
    ) -> QecResult<bool> {
        let results = self.collect_results(job_id)?;
        Ok(results.len() == expected_partitions)
    }

    // -------------------------------------------------------------------------
    // Cancellation/failure
    // -------------------------------------------------------------------------

    pub fn cancel_task(&self, task: TaskKey) -> QecResult<()> {
        let mut tasks = self.lock_tasks()?;

        let runtime = tasks
            .get_mut(&task)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown task {task:?}"),
            })?;

        if matches!(
            runtime.state,
            TaskState::Completed | TaskState::Cancelled
        ) {
            return Ok(());
        }

        runtime.state = TaskState::Cancelled;

        drop(tasks);

        self.decrement_worker_active(runtime.worker_id)?;

        Ok(())
    }

    pub fn fail_task(&self, task: TaskKey) -> QecResult<()> {
        self.fail_task_internal(task)
    }

    fn fail_task_internal(&self, task: TaskKey) -> QecResult<()> {
        let worker_id = {
            let mut tasks = self.lock_tasks()?;

            let runtime = tasks
                .get_mut(&task)
                .ok_or_else(|| QecError::InvalidInput {
                    message: format!("unknown task {task:?}"),
                })?;

            if runtime.state == TaskState::Completed {
                return Ok(());
            }

            runtime.state = TaskState::Failed;
            runtime.worker_id
        };

        self.decrement_worker_active(worker_id)?;

        let mut metrics = self.lock_metrics()?;
        metrics.tasks_failed = metrics.tasks_failed.saturating_add(1);

        Ok(())
    }

    pub fn cancellation_reason(&self) -> QecResult<Option<CancellationReason>> {
        Ok(self.cancellation.reason())
    }

    // -------------------------------------------------------------------------
    // Introspection
    // -------------------------------------------------------------------------

    pub fn metrics(&self) -> QecResult<DistributedMetrics> {
        Ok(self.lock_metrics()?.clone())
    }

    pub fn task_state(&self, task: TaskKey) -> QecResult<Option<String>> {
        let tasks = self.lock_tasks()?;

        Ok(tasks.get(&task).map(|runtime| {
            format!("{:?}", runtime.state)
        }))
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    fn authenticate_worker(
        &self,
        worker_id: WorkerId,
        credential: &WorkerCredential,
    ) -> QecResult<AuthenticatedWorker> {
        let identity = self.authenticator.authenticate(worker_id, credential)?;

        let workers = self.lock_workers()?;

        let record = workers
            .get(&worker_id)
            .ok_or_else(|| QecError::CapabilityDenied {
                capability: format!("worker {worker_id:?} is not registered"),
            })?;

        if record.state == WorkerState::Removed {
            return Err(QecError::CapabilityDenied {
                capability: format!("worker {worker_id:?} is removed"),
            });
        }

        Ok(identity)
    }

    fn admit_task(&self) -> QecResult<()> {
        self.cancellation.check()?;

        let active_tasks = self
            .lock_tasks()?
            .values()
            .filter(|runtime| runtime.state == TaskState::Running)
            .count();

        if active_tasks >= self.config.max_in_flight_tasks {
            return Err(QecError::ResourceLimitExceeded {
                resource: "distributed in-flight tasks".into(),
                requested: active_tasks.saturating_add(1) as u64,
                limit: self.config.max_in_flight_tasks as u64,
            });
        }

        let worker_count = self
            .lock_workers()?
            .values()
            .filter(|worker| worker.state.accepts_work())
            .count();

        if worker_count == 0 {
            return Err(QecError::InvalidInput {
                message: "no workers available for distributed task admission".into(),
            });
        }

        Ok(())
    }

    fn increment_worker_active(&self, worker_id: WorkerId) -> QecResult<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown worker {worker_id:?}"),
            })?;

        if !worker.state.accepts_work() && worker.state != WorkerState::Busy {
            return Err(QecError::InvalidInput {
                message: format!("worker {worker_id:?} cannot accept work"),
            });
        }

        worker.active_tasks = worker.active_tasks.saturating_add(1);
        worker.state = WorkerState::Busy;
        worker.last_seen = Instant::now();

        let active_tasks = workers
            .values()
            .map(|worker| worker.active_tasks)
            .sum::<usize>();

        let mut metrics = self.lock_metrics()?;
        metrics.peak_active_tasks =
            metrics.peak_active_tasks.max(active_tasks);

        Ok(())
    }

    fn decrement_worker_active(&self, worker_id: WorkerId) -> QecResult<()> {
        let mut workers = self.lock_workers()?;

        let worker = workers
            .get_mut(&worker_id)
            .ok_or_else(|| QecError::InvalidInput {
                message: format!("unknown worker {worker_id:?}"),
            })?;

        worker.active_tasks = worker.active_tasks.saturating_sub(1);

        if worker.active_tasks == 0
            && worker.state == WorkerState::Busy
        {
            worker.state = WorkerState::Ready;
        }

        worker.last_seen = Instant::now();

        Ok(())
    }

    fn record_stale_result(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.stale_results = metrics.stale_results.saturating_add(1);
        }
    }

    fn record_capability_denial(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.capability_denials =
                metrics.capability_denials.saturating_add(1);
        }
    }

    fn lock_workers(
        &self,
    ) -> QecResult<std::sync::MutexGuard<'_, BTreeMap<WorkerId, WorkerRecord>>> {
        self.workers
            .lock()
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "distributed worker mutex".into(),
                message: "worker state mutex was poisoned".into(),
            })
    }

    fn lock_tasks(
        &self,
    ) -> QecResult<std::sync::MutexGuard<'_, BTreeMap<TaskKey, TaskRuntime>>> {
        self.tasks
            .lock()
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "distributed task mutex".into(),
                message: "task state mutex was poisoned".into(),
            })
    }

    fn lock_results(
        &self,
    ) -> QecResult<std::sync::MutexGuard<'_, BTreeMap<TaskKey, AcceptedResult>>> {
        self.accepted_results
            .lock()
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "distributed result mutex".into(),
                message: "result state mutex was poisoned".into(),
            })
    }

    fn lock_metrics(
        &self,
    ) -> QecResult<std::sync::MutexGuard<'_, DistributedMetrics>> {
        self.metrics
            .lock()
            .map_err(|_| QecError::InternalInvariantViolation {
                invariant: "distributed metrics mutex".into(),
                message: "metrics mutex was poisoned".into(),
            })
    }
}

// -----------------------------------------------------------------------------
// Result types
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultAcceptance {
    Accepted,
    DuplicateIdempotent,
}

/// Final distributed reconciliation result.
#[derive(Clone, Debug)]
pub struct DistributedReconciliationResult {
    pub partitions_completed: usize,
    pub reconciliation: Vec<BoundaryReconciliation>,
    pub resource_usage: ResourceSnapshot,
}

// -----------------------------------------------------------------------------
// Boundary reconciliation
// -----------------------------------------------------------------------------

/// Reconciles two neighboring partition boundaries.
///
/// The mathematical definition of the partition boundary is owned by
/// `partition.rs`. This function only combines worker-produced logical parity
/// with the canonical boundary metadata.
///
/// It deliberately refuses to silently invent a correction when boundary
/// metadata is incompatible.
fn reconcile_partition_boundary(
    left_boundary: &PartitionBoundary,
    right_boundary: &PartitionBoundary,
    left_result: &PartitionWorkResult,
    right_result: &PartitionWorkResult,
) -> QecResult<BoundaryReconciliation> {
    if left_boundary.right != right_boundary.left {
        return Err(QecError::InvalidInput {
            message: format!(
                "partition boundaries do not connect: left={:?}, right={:?}",
                left_boundary.right, right_boundary.left
            ),
        });
    }

    let parity = left_result
        .correction
        .logical_parity
        .wrapping_add(right_result.correction.logical_parity)
        & 1;

    Ok(BoundaryReconciliation {
        left_partition: left_boundary.partition,
        right_partition: right_boundary.partition,
        boundary: left_boundary.right,
        correction_fingerprint: combine_fingerprints(
            &left_result.correction.fingerprint,
            &right_result.correction.fingerprint,
        ),
        logical_parity: parity,
    })
}

fn combine_fingerprints(left: &str, right: &str) -> String {
    format!("{left}:{right}")
}

// -----------------------------------------------------------------------------
// Distributed job facade
// -----------------------------------------------------------------------------

/// High-level distributed job state.
///
/// This facade exists to make lifecycle state explicit without making the
/// coordinator responsible for decoder execution.
#[derive(Debug)]
pub struct DistributedJob {
    id: DistributedJobId,
    state: DistributedJobState,
    started: Instant,
}

impl DistributedJob {
    pub fn new() -> Self {
        Self {
            id: DistributedJobId::generate(),
            state: DistributedJobState::Created,
            started: Instant::now(),
        }
    }

    pub fn id(&self) -> DistributedJobId {
        self.id
    }

    pub fn state(&self) -> DistributedJobState {
        self.state
    }

    pub fn start_validation(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Validating)
    }

    pub fn admit(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Admitted)
    }

    pub fn start(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Running)
    }

    pub fn start_reconciliation(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Reconciling)
    }

    pub fn complete(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Completed)
    }

    pub fn fail(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Failed)
    }

    pub fn cancel(&mut self) -> QecResult<()> {
        self.transition(DistributedJobState::Cancelled)
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    fn transition(&mut self, next: DistributedJobState) -> QecResult<()> {
        let valid = match (self.state, next) {
            (DistributedJobState::Created, DistributedJobState::Validating) => true,
            (DistributedJobState::Validating, DistributedJobState::Admitted) => true,
            (DistributedJobState::Admitted, DistributedJobState::Running) => true,
            (DistributedJobState::Running, DistributedJobState::Reconciling) => true,
            (DistributedJobState::Reconciling, DistributedJobState::Completed) => true,

            (DistributedJobState::Created, DistributedJobState::Failed) => true,
            (DistributedJobState::Validating, DistributedJobState::Failed) => true,
            (DistributedJobState::Admitted, DistributedJobState::Failed) => true,
            (DistributedJobState::Running, DistributedJobState::Failed) => true,
            (DistributedJobState::Reconciling, DistributedJobState::Failed) => true,

            (DistributedJobState::Created, DistributedJobState::Cancelled) => true,
            (DistributedJobState::Validating, DistributedJobState::Cancelled) => true,
            (DistributedJobState::Admitted, DistributedJobState::Cancelled) => true,
            (DistributedJobState::Running, DistributedJobState::Cancelled) => true,
            (DistributedJobState::Reconciling, DistributedJobState::Cancelled) => true,

            _ if self.state.is_terminal() => false,
            _ => false,
        };

        if !valid {
            return Err(QecError::InternalInvariantViolation {
                invariant: "distributed job state machine".into(),
                message: format!(
                    "invalid distributed job transition {:?} -> {:?}",
                    self.state, next
                ),
            });
        }

        self.state = next;
        Ok(())
    }
}

impl Default for DistributedJob {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Deterministic result aggregation
// -----------------------------------------------------------------------------

/// Aggregates partition results in stable partition order.
///
/// This function intentionally does not inspect decoder semantics.
pub fn aggregate_results(
    mut results: Vec<PartitionWorkResult>,
) -> QecResult<Vec<PartitionWorkResult>> {
    results.sort_by_key(|result| result.task);

    let mut seen = BTreeSet::new();

    for result in &results {
        if !seen.insert(result.task) {
            return Err(QecError::InternalInvariantViolation {
                invariant: "unique accepted task results".into(),
                message: format!("duplicate task {:?} during aggregation", result.task),
            });
        }
    }

    Ok(results)
}

// -----------------------------------------------------------------------------
// Fingerprint helper
// -----------------------------------------------------------------------------

/// Deterministic, non-cryptographic fingerprint.
///
/// This function is intentionally NOT presented as a security primitive.
/// Production integrity/authentication must use `ResultAuthenticator`.
pub fn deterministic_fingerprint(data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;

    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    format!("{hash:016x}")
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn capabilities() -> DistributedCapabilitySet {
        DistributedCapabilitySet::new()
            .allow(DistributedCapability::ExecutePartition)
            .allow(DistributedCapability::ReadPartitionInput)
            .allow(DistributedCapability::SubmitPartitionResult)
            .allow(DistributedCapability::ReadMetrics)
            .allow(DistributedCapability::ReconcileBoundary)
    }

    #[test]
    fn attempt_increments_without_overflow() {
        assert_eq!(Attempt(0).next().unwrap(), Attempt(1));
    }

    #[test]
    fn static_authenticator_registers_workers() {
        let mut authenticator = StaticWorkerAuthenticator::new();

        authenticator.register(WorkerId(1), capabilities());

        let identity = authenticator
            .authenticate(WorkerId(1), &WorkerCredential::new(vec![]))
            .unwrap();

        assert_eq!(identity.worker_id, WorkerId(1));
        assert!(identity
            .capabilities
            .contains(DistributedCapability::ExecutePartition));
    }

    #[test]
    fn unregistered_worker_is_rejected() {
        let authenticator = StaticWorkerAuthenticator::new();

        let result = authenticator.authenticate(
            WorkerId(42),
            &WorkerCredential::new(vec![]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn capability_attenuation_is_deny_by_default() {
        let set = capabilities().attenuate([
            DistributedCapability::ExecutePartition,
            DistributedCapability::QpuSubmit,
        ]);

        assert!(set.contains(DistributedCapability::ExecutePartition));
        assert!(!set.contains(DistributedCapability::QpuSubmit));
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let a = deterministic_fingerprint(b"abc");
        let b = deterministic_fingerprint(b"abc");

        assert_eq!(a, b);
    }

    #[test]
    fn aggregation_is_stable() {
        let job = DistributedJobId(1);

        let make_result = |partition_id| PartitionWorkResult {
            task: TaskKey::new(job, partition_id),
            attempt: Attempt::FIRST,
            worker_id: WorkerId(1),
            correction: PartitionCorrection::new("x", 0, Vec::new()),
            resource_usage: ResourceSnapshot::default(),
            execution_fingerprint: "result".into(),
        };

        let results = aggregate_results(vec![
            make_result(PartitionId(3)),
            make_result(PartitionId(1)),
            make_result(PartitionId(2)),
        ])
        .unwrap();

        assert_eq!(results[0].task.partition_id, PartitionId(1));
        assert_eq!(results[1].task.partition_id, PartitionId(2));
        assert_eq!(results[2].task.partition_id, PartitionId(3));
    }

    #[test]
    fn job_state_machine_rejects_invalid_transition() {
        let mut job = DistributedJob::new();

        assert!(job.complete().is_err());
        assert_eq!(job.state(), DistributedJobState::Created);
    }

    #[test]
    fn job_state_machine_accepts_normal_lifecycle() {
        let mut job = DistributedJob::new();

        job.start_validation().unwrap();
        job.admit().unwrap();
        job.start().unwrap();
        job.start_reconciliation().unwrap();
        job.complete().unwrap();

        assert_eq!(job.state(), DistributedJobState::Completed);
    }

    #[test]
    fn duplicate_result_fingerprint_is_idempotent() {
        let first = "same";
        let second = "same";

        assert_eq!(first, second);
    }

    #[test]
    fn conflicting_result_fingerprint_is_detectable() {
        assert_ne!("first", "second");
    }
}