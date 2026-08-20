//! Runtime resource accounting for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! `limits.rs` owns the declarative QEC resource policy.
//! `resources.rs` owns runtime accounting and reservations.
//!
//! ```text
//!                         QecConfig
//!                            │
//!                            ▼
//!                         QecLimits
//!                            │
//!                 ┌──────────┴──────────┐
//!                 │                     │
//!                 ▼                     ▼
//!             Preflight            ResourceManager
//!                                       │
//!                           ┌───────────┼───────────┐
//!                           │           │           │
//!                           ▼           ▼           ▼
//!                         Memory     Counters    Cancellation
//!                           │           │           │
//!                           └───────────┼───────────┘
//!                                       ▼
//!                              ResourceSnapshot
//! ```
//!
//! The important architectural distinction is:
//!
//! - [`QecLimits`] = what the execution is allowed to request.
//! - [`ResourceManager`] = what the execution has actually consumed.
//! - [`ResourceSnapshot`] = immutable runtime accounting state.
//! - [`ResourceScope`] = a bounded logical operation.
//!
//! `resources.rs` must not invent a second independent production resource
//! policy. `QecLimits` is the canonical policy.
//!
//! # Safety properties
//!
//! - Resource checks occur before reservations.
//! - Reservations are atomic.
//! - Memory and workers use RAII guards.
//! - Counter arithmetic is checked.
//! - Resource exhaustion returns structured errors.
//! - Cancellation is checked by expensive operations.
//! - Wall-clock limits are enforced.
//! - Operation quotas cannot bypass global limits.
//! - Preflight estimation performs no allocation.
//! - Resource-limited work cannot silently appear successful.
//! - Runtime counters can be shared safely between decoder workers.
//!
//! The subsystem does not promise literally infinite memory, runtime, or
//! parallelism. Arbitrarily large workloads are supported only when the
//! configured resource policy and physical execution environment permit them.

use core::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    AtomicUsize,
    Ordering,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::errors::{
    QecError,
    QecResult,
    ResourceKind as QecResourceKind,
};
use super::limits::{
    LimitError,
    LimitKind,
    QecLimits,
};

/// Compatibility sentinel.
///
/// This does not mean physical resources are infinite. It means the
/// application-level policy does not impose an additional finite ceiling.
pub const UNLIMITED_U64: u64 = u64::MAX;

/// Compatibility sentinel for parallelism.
pub const UNLIMITED_USIZE: usize = usize::MAX;

/* ========================================================================== */
/* Runtime resource kinds                                                     */
/* ========================================================================== */

/// Runtime dimensions tracked by [`ResourceManager`].
///
/// This is intentionally separate from `limits::LimitKind`:
///
/// - `LimitKind` describes policy dimensions.
/// - `ResourceKind` describes runtime counters/reservations.
///
/// Both are mapped explicitly rather than relying on enum layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    MemoryBytes,
    SyndromeEvents,
    GraphNodes,
    GraphEdges,
    DecoderIterations,
    ParallelWorkers,
    CodeDistance,
    Qubits,
    Stabilizers,
    MeasurementRounds,
    CheckpointSizeBytes,
    Partitions,
    StreamBufferEvents,
    StabilizerWeight,
    LogicalOperatorWeight,
    QubitsPerPartition,
    QpuShots,
    QpuCircuits,
    VerificationOperations,
}

impl ResourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MemoryBytes => "memory_bytes",
            Self::SyndromeEvents => "syndrome_events",
            Self::GraphNodes => "graph_nodes",
            Self::GraphEdges => "graph_edges",
            Self::DecoderIterations => "decoder_iterations",
            Self::ParallelWorkers => "parallel_workers",
            Self::CodeDistance => "code_distance",
            Self::Qubits => "qubits",
            Self::Stabilizers => "stabilizers",
            Self::MeasurementRounds => "measurement_rounds",
            Self::CheckpointSizeBytes => "checkpoint_size_bytes",
            Self::Partitions => "partitions",
            Self::StreamBufferEvents => "stream_buffer_events",
            Self::StabilizerWeight => "stabilizer_weight",
            Self::LogicalOperatorWeight => "logical_operator_weight",
            Self::QubitsPerPartition => "qubits_per_partition",
            Self::QpuShots => "qpu_shots",
            Self::QpuCircuits => "qpu_circuits",
            Self::VerificationOperations => "verification_operations",
        }
    }

    pub const fn to_qec_kind(self) -> QecResourceKind {
        match self {
            Self::MemoryBytes => QecResourceKind::MemoryBytes,
            Self::SyndromeEvents => QecResourceKind::SyndromeEvents,
            Self::GraphNodes => QecResourceKind::GraphNodes,
            Self::GraphEdges => QecResourceKind::GraphEdges,
            Self::DecoderIterations => QecResourceKind::DecoderIterations,
            Self::ParallelWorkers => QecResourceKind::Parallelism,
            Self::CodeDistance => QecResourceKind::CodeDistance,
            Self::Qubits => QecResourceKind::Qubits,
            Self::Stabilizers => QecResourceKind::Stabilizers,
            Self::MeasurementRounds => QecResourceKind::MeasurementRounds,
            Self::CheckpointSizeBytes => QecResourceKind::CheckpointSize,
            Self::Partitions => QecResourceKind::Partitions,
            Self::StreamBufferEvents => QecResourceKind::StreamBuffer,
            Self::StabilizerWeight => QecResourceKind::Custom,
            Self::LogicalOperatorWeight => QecResourceKind::Custom,
            Self::QubitsPerPartition => QecResourceKind::Custom,
            Self::QpuShots => QecResourceKind::QpuShots,
            Self::QpuCircuits => QecResourceKind::QpuCircuits,
            Self::VerificationOperations => QecResourceKind::Custom,
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* ========================================================================== */
/* Compatibility resource policy                                             */
/* ========================================================================== */

/// Compatibility adapter for older callers.
///
/// `QecLimits` is now canonical. New code should use:
///
/// ```text
/// QecLimits
///     ↓
/// ResourceManager::from_qec_limits
/// ```
///
/// This type remains available so existing lower-level code can migrate
/// without requiring an all-at-once repository rewrite.
///
/// New production code should not introduce additional `ResourceLimits`.
#[deprecated(
    note = "use limits::QecLimits; ResourceManager::from_qec_limits() is the canonical API"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    pub max_memory_bytes: u64,
    pub max_syndrome_events: u64,
    pub max_graph_nodes: u64,
    pub max_graph_edges: u64,
    pub max_decoder_iterations: u64,
    pub max_parallelism: usize,
    pub max_wall_time: Option<Duration>,
}

#[allow(deprecated)]
impl Default for ResourceLimits {
    fn default() -> Self {
        let limits = QecLimits::default();

        Self::from(&limits)
    }
}

#[allow(deprecated)]
impl ResourceLimits {
    pub const fn unlimited() -> Self {
        Self {
            max_memory_bytes: UNLIMITED_U64,
            max_syndrome_events: UNLIMITED_U64,
            max_graph_nodes: UNLIMITED_U64,
            max_graph_edges: UNLIMITED_U64,
            max_decoder_iterations: UNLIMITED_U64,
            max_parallelism: UNLIMITED_USIZE,
            max_wall_time: None,
        }
    }

    pub fn validate(&self) -> Result<(), ResourceError> {
        if self.max_memory_bytes == 0
            || self.max_syndrome_events == 0
            || self.max_graph_nodes == 0
            || self.max_graph_edges == 0
            || self.max_decoder_iterations == 0
            || self.max_parallelism == 0
        {
            return Err(ResourceError::InvalidLimit {
                reason: "finite resource limits must be greater than zero",
            });
        }

        Ok(())
    }

    pub fn into_qec_limits(self) -> Result<QecLimits, ResourceError> {
        let mut limits = QecLimits::default();

        limits.max_memory_bytes = self.max_memory_bytes;
        limits.max_syndrome_events = to_usize(
            self.max_syndrome_events,
            LimitKind::SyndromeEvents,
        )?;
        limits.max_graph_nodes =
            to_usize(self.max_graph_nodes, LimitKind::GraphNodes)?;
        limits.max_graph_edges =
            to_usize(self.max_graph_edges, LimitKind::GraphEdges)?;
        limits.max_decoder_iterations = to_usize(
            self.max_decoder_iterations,
            LimitKind::DecoderIterations,
        )?;
        limits.max_parallelism = self.max_parallelism;

        /*
         * The canonical QecLimits has a nanosecond decoder-time limit rather
         * than an Option<Duration>. `None` is represented here by the
         * canonical maximum.
         */
        if let Some(duration) = self.max_wall_time {
            limits.max_decoder_time_ns =
                u64::try_from(duration.as_nanos()).map_err(|_| {
                    ResourceError::ArithmeticOverflow {
                        resource: ResourceKind::MemoryBytes,
                    }
                })?;
        }

        limits.validate().map_err(ResourceError::from)
    }

    pub fn from_qec_limits(limits: &QecLimits) -> Self {
        Self {
            max_memory_bytes: limits.max_memory_bytes,
            max_syndrome_events: limits.max_syndrome_events as u64,
            max_graph_nodes: limits.max_graph_nodes as u64,
            max_graph_edges: limits.max_graph_edges as u64,
            max_decoder_iterations:
                limits.max_decoder_iterations as u64,
            max_parallelism: limits.max_parallelism,
            max_wall_time: Some(Duration::from_nanos(
                limits.max_decoder_time_ns,
            )),
        }
    }
}

/* ========================================================================== */
/* Per-operation quotas                                                       */
/* ========================================================================== */

/// Optional stricter quota for one logical operation.
///
/// A quota can only tighten the global `QecLimits`. It can never expand it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceQuota {
    pub max_memory_bytes: Option<u64>,
    pub max_syndrome_events: Option<u64>,
    pub max_graph_nodes: Option<u64>,
    pub max_graph_edges: Option<u64>,
    pub max_decoder_iterations: Option<u64>,
    pub max_parallelism: Option<usize>,
    pub max_wall_time: Option<Duration>,

    pub max_code_distance: Option<usize>,
    pub max_qubits: Option<usize>,
    pub max_stabilizers: Option<usize>,
    pub max_rounds: Option<usize>,
    pub max_checkpoint_size_bytes: Option<u64>,
    pub max_partitions: Option<usize>,
    pub max_stream_buffer_events: Option<usize>,
    pub max_stabilizer_weight: Option<usize>,
    pub max_logical_operator_weight: Option<usize>,
    pub max_qubits_per_partition: Option<usize>,
    pub max_qpu_shots: Option<u64>,
    pub max_qpu_circuits: Option<u64>,
    pub max_verification_operations: Option<u64>,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_syndrome_events: None,
            max_graph_nodes: None,
            max_graph_edges: None,
            max_decoder_iterations: None,
            max_parallelism: None,
            max_wall_time: None,
            max_code_distance: None,
            max_qubits: None,
            max_stabilizers: None,
            max_rounds: None,
            max_checkpoint_size_bytes: None,
            max_partitions: None,
            max_stream_buffer_events: None,
            max_stabilizer_weight: None,
            max_logical_operator_weight: None,
            max_qubits_per_partition: None,
            max_qpu_shots: None,
            max_qpu_circuits: None,
            max_verification_operations: None,
        }
    }
}

impl ResourceQuota {
    pub fn validate(&self) -> Result<(), ResourceError> {
        let numeric_limits = [
            self.max_memory_bytes,
            self.max_syndrome_events,
            self.max_graph_nodes,
            self.max_graph_edges,
            self.max_decoder_iterations,
            self.max_code_distance.map(|v| v as u64),
            self.max_qubits.map(|v| v as u64),
            self.max_stabilizers.map(|v| v as u64),
            self.max_rounds.map(|v| v as u64),
            self.max_checkpoint_size_bytes,
            self.max_partitions.map(|v| v as u64),
            self.max_stream_buffer_events.map(|v| v as u64),
            self.max_stabilizer_weight.map(|v| v as u64),
            self.max_logical_operator_weight.map(|v| v as u64),
            self.max_qubits_per_partition.map(|v| v as u64),
            self.max_qpu_shots,
            self.max_qpu_circuits,
            self.max_verification_operations,
        ];

        if numeric_limits.iter().flatten().any(|v| *v == 0) {
            return Err(ResourceError::InvalidLimit {
                reason: "finite operation quotas must be greater than zero",
            });
        }

        if self.max_parallelism == Some(0) {
            return Err(ResourceError::InvalidLimit {
                reason: "operation parallelism quota must be greater than zero",
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Runtime snapshot                                                           */
/* ========================================================================== */

/// Immutable runtime resource snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSnapshot {
    pub allocated_bytes: u64,
    pub peak_bytes: u64,

    pub syndrome_events: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub decoder_iterations: u64,

    pub parallel_workers: usize,

    pub code_distance: usize,
    pub qubits: usize,
    pub stabilizers: usize,
    pub measurement_rounds: usize,

    pub checkpoint_bytes: u64,
    pub partitions: usize,
    pub stream_buffer_events: usize,

    pub qpu_shots: u64,
    pub qpu_circuits: u64,

    pub verification_operations: u64,

    /// Wall-clock time since manager creation.
    pub wall_time: Duration,

    /// Backend-reported compute time.
    pub compute_time: Duration,
}

impl ResourceSnapshot {
    pub fn is_idle(&self) -> bool {
        self.allocated_bytes == 0
            && self.parallel_workers == 0
    }
}

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Runtime resource-accounting error.
///
/// Policy-definition errors originate from `limits.rs` and are converted
/// into this runtime type at the resource-management boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    InvalidLimit {
        reason: &'static str,
    },

    PolicyInvalid {
        resource: LimitKind,
        message: String,
    },

    LimitExceeded {
        resource: ResourceKind,
        requested: u64,
        current: u64,
        limit: u64,
    },

    ParallelismLimitExceeded {
        requested: usize,
        current: usize,
        limit: usize,
    },

    QuotaExceeded {
        resource: ResourceKind,
        requested: u64,
        current: u64,
        limit: u64,
    },

    ParallelismQuotaExceeded {
        requested: usize,
        current: usize,
        limit: usize,
    },

    ArithmeticOverflow {
        resource: ResourceKind,
    },

    WallTimeLimitExceeded {
        elapsed: Duration,
        limit: Duration,
    },

    Cancelled,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit { reason } => {
                write!(f, "invalid resource policy: {reason}")
            }

            Self::PolicyInvalid {
                resource,
                message,
            } => {
                write!(
                    f,
                    "invalid QEC resource policy for {resource}: {message}"
                )
            }

            Self::LimitExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "{resource} limit exceeded: requested {requested}, \
                     current {current}, limit {limit}"
                )
            }

            Self::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism limit exceeded: requested {requested}, \
                     current {current}, limit {limit}"
                )
            }

            Self::QuotaExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "{resource} operation quota exceeded: requested \
                     {requested}, current {current}, quota {limit}"
                )
            }

            Self::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism operation quota exceeded: requested \
                     {requested}, current {current}, quota {limit}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "resource counter overflow for {resource}"
                )
            }

            Self::WallTimeLimitExceeded {
                elapsed,
                limit,
            } => {
                write!(
                    f,
                    "wall-time limit exceeded: elapsed {elapsed:?}, \
                     limit {limit:?}"
                )
            }

            Self::Cancelled => {
                f.write_str("resource operation cancelled")
            }
        }
    }
}

impl std::error::Error for ResourceError {}

impl From<LimitError> for ResourceError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::InvalidLimit { resource, value } => {
                Self::PolicyInvalid {
                    resource,
                    message: format!(
                        "value {value} must be greater than zero"
                    ),
                }
            }

            LimitError::Exceeded {
                resource,
                requested,
                maximum,
            } => {
                Self::PolicyInvalid {
                    resource,
                    message: format!(
                        "requested {requested} exceeds maximum {maximum}"
                    ),
                }
            }

            LimitError::ArithmeticOverflow { resource } => {
                Self::PolicyInvalid {
                    resource,
                    message: "policy arithmetic overflow".to_string(),
                }
            }

            LimitError::InconsistentLimits {
                resource,
                related_resource,
                reason,
            } => {
                Self::PolicyInvalid {
                    resource,
                    message: format!(
                        "inconsistent with {related_resource}: {reason}"
                    ),
                }
            }
        }
    }
}

/* ========================================================================== */
/* Atomic runtime counters                                                    */
/* ========================================================================== */

#[derive(Debug, Default)]
struct ResourceCounters {
    allocated_bytes: AtomicU64,
    peak_bytes: AtomicU64,

    syndrome_events: AtomicU64,
    graph_nodes: AtomicU64,
    graph_edges: AtomicU64,
    decoder_iterations: AtomicU64,

    parallel_workers: AtomicUsize,

    code_distance: AtomicUsize,
    qubits: AtomicUsize,
    stabilizers: AtomicUsize,
    measurement_rounds: AtomicUsize,

    checkpoint_bytes: AtomicU64,
    partitions: AtomicUsize,
    stream_buffer_events: AtomicUsize,

    qpu_shots: AtomicU64,
    qpu_circuits: AtomicU64,

    verification_operations: AtomicU64,

    compute_time_nanos: AtomicU64,

    cancelled: AtomicBool,
}

/* ========================================================================== */
/* Resource manager                                                           */
/* ========================================================================== */

/// Thread-safe runtime resource manager.
///
/// The manager is intentionally independent from any particular decoder.
/// MWPM, Union-Find, surface-code construction, streaming, partitioning,
/// simulation, verification and QPU adapters can all use the same manager.
#[derive(Debug)]
pub struct ResourceManager {
    limits: QecLimits,
    counters: ResourceCounters,
    started: Instant,
}

impl ResourceManager {
    /// Creates a manager from the canonical QEC resource policy.
    pub fn from_qec_limits(
        limits: QecLimits,
    ) -> Result<Self, ResourceError> {
        limits.validate()?;

        Ok(Self {
            limits,
            counters: ResourceCounters::default(),
            started: Instant::now(),
        })
    }

    /// Canonical constructor.
    pub fn new(
        limits: QecLimits,
    ) -> Result<Self, ResourceError> {
        Self::from_qec_limits(limits)
    }

    /// Compatibility constructor for older callers.
    #[allow(deprecated)]
    pub fn from_resource_limits(
        limits: ResourceLimits,
    ) -> Result<Self, ResourceError> {
        Self::from_qec_limits(limits.into_qec_limits()?)
    }

    /// Returns the canonical immutable policy.
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Creates a shared resource manager.
    pub fn shared(
        limits: QecLimits,
    ) -> Result<Arc<Self>, ResourceError> {
        Ok(Arc::new(Self::new(limits)?))
    }

    /* ---------------------------------------------------------------------- */
    /* Cancellation                                                            */
    /* ---------------------------------------------------------------------- */

    /// Requests cancellation of the current manager workload.
    pub fn cancel(&self) {
        self.counters
            .cancelled
            .store(true, Ordering::Release);
    }

    /// Clears the cancellation state.
    ///
    /// This should only be used between logical operations.
    pub fn reset_cancellation(&self) {
        self.counters
            .cancelled
            .store(false, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.counters
            .cancelled
            .load(Ordering::Acquire)
    }

    /// Checks cancellation and the global execution deadline.
    pub fn check(&self) -> Result<(), ResourceError> {
        if self.is_cancelled() {
            return Err(ResourceError::Cancelled);
        }

        let elapsed = self.started.elapsed();
        let limit = Duration::from_nanos(
            self.limits.max_decoder_time_ns,
        );

        if elapsed > limit {
            return Err(ResourceError::WallTimeLimitExceeded {
                elapsed,
                limit,
            });
        }

        Ok(())
    }

    /// Same check exposed as the canonical QEC error type.
    pub fn check_qec(&self) -> QecResult<()> {
        self.check().map_err(Into::into)
    }

    /* ---------------------------------------------------------------------- */
    /* Snapshots                                                               */
    /* ---------------------------------------------------------------------- */

    pub fn snapshot(&self) -> ResourceSnapshot {
        ResourceSnapshot {
            allocated_bytes: self
                .counters
                .allocated_bytes
                .load(Ordering::Acquire),

            peak_bytes: self
                .counters
                .peak_bytes
                .load(Ordering::Acquire),

            syndrome_events: self
                .counters
                .syndrome_events
                .load(Ordering::Acquire),

            graph_nodes: self
                .counters
                .graph_nodes
                .load(Ordering::Acquire),

            graph_edges: self
                .counters
                .graph_edges
                .load(Ordering::Acquire),

            decoder_iterations: self
                .counters
                .decoder_iterations
                .load(Ordering::Acquire),

            parallel_workers: self
                .counters
                .parallel_workers
                .load(Ordering::Acquire),

            code_distance: self
                .counters
                .code_distance
                .load(Ordering::Acquire),

            qubits: self
                .counters
                .qubits
                .load(Ordering::Acquire),

            stabilizers: self
                .counters
                .stabilizers
                .load(Ordering::Acquire),

            measurement_rounds: self
                .counters
                .measurement_rounds
                .load(Ordering::Acquire),

            checkpoint_bytes: self
                .counters
                .checkpoint_bytes
                .load(Ordering::Acquire),

            partitions: self
                .counters
                .partitions
                .load(Ordering::Acquire),

            stream_buffer_events: self
                .counters
                .stream_buffer_events
                .load(Ordering::Acquire),

            qpu_shots: self
                .counters
                .qpu_shots
                .load(Ordering::Acquire),

            qpu_circuits: self
                .counters
                .qpu_circuits
                .load(Ordering::Acquire),

            verification_operations: self
                .counters
                .verification_operations
                .load(Ordering::Acquire),

            wall_time: self.started.elapsed(),

            compute_time: Duration::from_nanos(
                self.counters
                    .compute_time_nanos
                    .load(Ordering::Acquire),
            ),
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Preflight                                                               */
    /* ---------------------------------------------------------------------- */

    /// Checks a complete workload request before allocation/construction.
    ///
    /// This is the API that surface-code construction, simulation, QPU
    /// adapters and exact verification should call before doing expensive
    /// work.
    pub fn preflight(
        &self,
        request: &ResourceRequest,
    ) -> Result<(), ResourceError> {
        self.check()?;
        request.validate()?;

        self.check_policy_value(
            ResourceKind::CodeDistance,
            request.code_distance,
            self.limits.max_code_distance,
        )?;

        self.check_policy_value(
            ResourceKind::Qubits,
            request.qubits,
            self.limits.max_qubits,
        )?;

        self.check_policy_value(
            ResourceKind::Stabilizers,
            request.stabilizers,
            self.limits.max_stabilizers,
        )?;

        self.check_policy_value(
            ResourceKind::SyndromeEvents,
            request.syndrome_events,
            self.limits.max_syndrome_events,
        )?;

        self.check_policy_value(
            ResourceKind::MeasurementRounds,
            request.measurement_rounds,
            self.limits.max_rounds,
        )?;

        self.check_policy_value(
            ResourceKind::GraphNodes,
            request.graph_nodes,
            self.limits.max_graph_nodes,
        )?;

        self.check_policy_value(
            ResourceKind::GraphEdges,
            request.graph_edges,
            self.limits.max_graph_edges,
        )?;

        self.check_policy_value(
            ResourceKind::MemoryBytes,
            request.memory_bytes,
            self.limits.max_memory_bytes,
        )?;

        self.check_policy_value(
            ResourceKind::DecoderIterations,
            request.decoder_iterations,
            self.limits.max_decoder_iterations as u64,
        )?;

        self.check_policy_value(
            ResourceKind::ParallelWorkers,
            request.parallel_workers as u64,
            self.limits.max_parallelism as u64,
        )?;

        self.check_policy_value(
            ResourceKind::CheckpointSizeBytes,
            request.checkpoint_size_bytes,
            self.limits.max_checkpoint_size_bytes,
        )?;

        self.check_policy_value(
            ResourceKind::Partitions,
            request.partitions as u64,
            self.limits.max_partitions as u64,
        )?;

        self.check_policy_value(
            ResourceKind::StreamBufferEvents,
            request.stream_buffer_events as u64,
            self.limits.max_stream_buffer_events as u64,
        )?;

        self.check_policy_value(
            ResourceKind::StabilizerWeight,
            request.stabilizer_weight as u64,
            self.limits.max_stabilizer_weight as u64,
        )?;

        self.check_policy_value(
            ResourceKind::LogicalOperatorWeight,
            request.logical_operator_weight as u64,
            self.limits.max_logical_operator_weight as u64,
        )?;

        self.check_policy_value(
            ResourceKind::QubitsPerPartition,
            request.qubits_per_partition as u64,
            self.limits.max_qubits_per_partition as u64,
        )?;

        self.check_policy_value(
            ResourceKind::QpuShots,
            request.qpu_shots,
            self.limits.max_qpu_shots,
        )?;

        self.check_policy_value(
            ResourceKind::QpuCircuits,
            request.qpu_circuits,
            self.limits.max_qpu_circuits,
        )?;

        self.check_policy_value(
            ResourceKind::VerificationOperations,
            request.verification_operations,
            self.limits.max_verification_operations,
        )?;

        Ok(())
    }

    /// Returns the amount of memory that may be safely reserved after
    /// accounting for the currently allocated memory.
    pub fn available_memory(&self) -> u64 {
        let current = self
            .counters
            .allocated_bytes
            .load(Ordering::Acquire);

        self.limits
            .max_memory_bytes
            .saturating_sub(current)
    }

    /* ---------------------------------------------------------------------- */
    /* Memory                                                                  */
    /* ---------------------------------------------------------------------- */

    pub fn reserve_memory(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'_>, ResourceError> {
        self.reserve_memory_with_quota(bytes, None)?;

        Ok(MemoryReservation {
            manager: self,
            bytes,
            active: true,
        })
    }

    pub fn reserve_memory_with_quota(
        &self,
        bytes: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            ResourceKind::MemoryBytes,
            &self.counters.allocated_bytes,
            bytes,
            self.limits.max_memory_bytes,
            quota,
        )?;

        self.update_peak();

        Ok(())
    }

    pub fn release_memory(&self, bytes: u64) {
        saturating_sub_u64(
            &self.counters.allocated_bytes,
            bytes,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Generic bounded counters                                                */
    /* ---------------------------------------------------------------------- */

    pub fn record_syndrome_events(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::SyndromeEvents,
            &self.counters.syndrome_events,
            count,
            self.limits.max_syndrome_events,
            None,
        )
    }

    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::GraphNodes,
            &self.counters.graph_nodes,
            count,
            self.limits.max_graph_nodes as u64,
            None,
        )
    }

    pub fn record_graph_edges(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::GraphEdges,
            &self.counters.graph_edges,
            count,
            self.limits.max_graph_edges as u64,
            None,
        )
    }

    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::DecoderIterations,
            &self.counters.decoder_iterations,
            count,
            self.limits.max_decoder_iterations as u64,
            None,
        )
    }

    pub fn record_qpu_shots(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::QpuShots,
            &self.counters.qpu_shots,
            count,
            self.limits.max_qpu_shots,
            None,
        )
    }

    pub fn record_qpu_circuits(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::QpuCircuits,
            &self.counters.qpu_circuits,
            count,
            self.limits.max_qpu_circuits,
            None,
        )
    }

    pub fn record_verification_operations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::VerificationOperations,
            &self.counters.verification_operations,
            count,
            self.limits.max_verification_operations,
            None,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Size/count resources                                                    */
    /* ---------------------------------------------------------------------- */

    pub fn record_code_distance(
        &self,
        distance: usize,
    ) -> Result<(), ResourceError> {
        self.check_policy_value(
            ResourceKind::CodeDistance,
            distance as u64,
            self.limits.max_code_distance,
        )?;

        store_max_usize(
            &self.counters.code_distance,
            distance,
        );

        Ok(())
    }

    pub fn record_qubits(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Qubits,
            &self.counters.qubits,
            count,
            self.limits.max_qubits,
        )
    }

    pub fn record_stabilizers(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Stabilizers,
            &self.counters.stabilizers,
            count,
            self.limits.max_stabilizers,
        )
    }

    pub fn record_measurement_rounds(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::MeasurementRounds,
            &self.counters.measurement_rounds,
            count,
            self.limits.max_rounds,
        )
    }

    pub fn record_checkpoint_bytes(
        &self,
        bytes: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::CheckpointSizeBytes,
            &self.counters.checkpoint_bytes,
            bytes,
            self.limits.max_checkpoint_size_bytes,
            None,
        )
    }

    pub fn record_partitions(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Partitions,
            &self.counters.partitions,
            count,
            self.limits.max_partitions,
        )
    }

    pub fn record_stream_buffer_events(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::StreamBufferEvents,
            &self.counters.stream_buffer_events,
            count,
            self.limits.max_stream_buffer_events,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Workers                                                                 */
    /* ---------------------------------------------------------------------- */

    pub fn acquire_workers(
        &self,
        workers: usize,
    ) -> Result<WorkerReservation<'_>, ResourceError> {
        self.acquire_workers_with_quota(workers, None)?;

        Ok(WorkerReservation {
            manager: self,
            workers,
            active: true,
        })
    }

    pub fn acquire_workers_with_quota(
        &self,
        workers: usize,
        quota: Option<usize>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        if workers == 0 {
            return Ok(());
        }

        let mut current = self
            .counters
            .parallel_workers
            .load(Ordering::Acquire);

        loop {
            let next = current.checked_add(workers).ok_or(
                ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::ParallelWorkers,
                },
            )?;

            if next > self.limits.max_parallelism {
                return Err(
                    ResourceError::ParallelismLimitExceeded {
                        requested: workers,
                        current,
                        limit: self.limits.max_parallelism,
                    },
                );
            }

            if let Some(limit) = quota {
                if next > limit {
                    return Err(
                        ResourceError::ParallelismQuotaExceeded {
                            requested: workers,
                            current,
                            limit,
                        },
                    );
                }
            }

            match self
                .counters
                .parallel_workers
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => return Ok(()),
                Err(actual) => current = actual,
            }
        }
    }

    pub fn release_workers(&self, workers: usize) {
        saturating_sub_usize(
            &self.counters.parallel_workers,
            workers,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Compute time                                                            */
    /* ---------------------------------------------------------------------- */

    pub fn record_compute_time(
        &self,
        duration: Duration,
    ) -> Result<(), ResourceError> {
        let nanos = u64::try_from(duration.as_nanos())
            .map_err(|_| ResourceError::ArithmeticOverflow {
                resource: ResourceKind::DecoderIterations,
            })?;

        let mut current = self
            .counters
            .compute_time_nanos
            .load(Ordering::Acquire);

        loop {
            let next = current.checked_add(nanos).ok_or(
                ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::DecoderIterations,
                },
            )?;

            match self
                .counters
                .compute_time_nanos
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => return Ok(()),
                Err(actual) => current = actual,
            }
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Operation scopes                                                        */
    /* ---------------------------------------------------------------------- */

    pub fn scope<'a>(
        &'a self,
        name: impl Into<String>,
        quota: ResourceQuota,
    ) -> Result<ResourceScope<'a>, ResourceError> {
        quota.validate()?;
        self.check()?;

        Ok(ResourceScope {
            manager: self,
            name: name.into(),
            quota,
            started: Instant::now(),
        })
    }

    /* ---------------------------------------------------------------------- */
    /* Internal accounting helpers                                             */
    /* ---------------------------------------------------------------------- */

    fn record_u64(
        &self,
        resource: ResourceKind,
        counter: &AtomicU64,
        requested: u64,
        global_limit: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.try_add(
            resource,
            counter,
            requested,
            global_limit,
            quota,
        )
    }

    fn record_usize(
        &self,
        resource: ResourceKind,
        counter: &AtomicUsize,
        requested: usize,
        global_limit: usize,
    ) -> Result<(), ResourceError> {
        self.check()?;

        let mut current =
            counter.load(Ordering::Acquire);

        loop {
            let next = current.checked_add(requested).ok_or(
                ResourceError::ArithmeticOverflow { resource },
            )?;

            if next > global_limit {
                return Err(ResourceError::LimitExceeded {
                    resource,
                    requested: requested as u64,
                    current: current as u64,
                    limit: global_limit as u64,
                });
            }

            match counter.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),
                Err(actual) => current = actual,
            }
        }
    }

    fn try_add(
        &self,
        resource: ResourceKind,
        counter: &AtomicU64,
        requested: u64,
        global_limit: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        let mut current =
            counter.load(Ordering::Acquire);

        loop {
            let next = current.checked_add(requested).ok_or(
                ResourceError::ArithmeticOverflow { resource },
            )?;

            if next > global_limit {
                return Err(ResourceError::LimitExceeded {
                    resource,
                    requested,
                    current,
                    limit: global_limit,
                });
            }

            if let Some(limit) = quota {
                if next > limit {
                    return Err(ResourceError::QuotaExceeded {
                        resource,
                        requested,
                        current,
                        limit,
                    });
                }
            }

            match counter.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),
                Err(actual) => current = actual,
            }
        }
    }

    fn check_policy_value(
        &self,
        resource: ResourceKind,
        requested: u64,
        maximum: u64,
    ) -> Result<(), ResourceError> {
        if requested > maximum {
            return Err(ResourceError::LimitExceeded {
                resource,
                requested,
                current: 0,
                limit: maximum,
            });
        }

        Ok(())
    }

    fn update_peak(&self) {
        let current = self
            .counters
            .allocated_bytes
            .load(Ordering::Acquire);

        let mut peak = self
            .counters
            .peak_bytes
            .load(Ordering::Acquire);

        while current > peak {
            match self
                .counters
                .peak_bytes
                .compare_exchange_weak(
                    peak,
                    current,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => break,
                Err(actual) => peak = actual,
            }
        }
    }
}

/* ========================================================================== */
/* Resource request / preflight model                                         */
/* ========================================================================== */

/// Declarative estimate of resources required by an operation.
///
/// This object is intentionally allocation-free and can therefore be used
/// before constructing a surface code, graph, syndrome buffer, checkpoint,
/// QPU workload or exact verification search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceRequest {
    pub code_distance: u64,
    pub qubits: u64,
    pub stabilizers: u64,
    pub syndrome_events: u64,
    pub measurement_rounds: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub memory_bytes: u64,
    pub decoder_iterations: u64,
    pub parallel_workers: usize,
    pub checkpoint_size_bytes: u64,
    pub partitions: usize,
    pub stream_buffer_events: usize,
    pub stabilizer_weight: usize,
    pub logical_operator_weight: usize,
    pub qubits_per_partition: usize,
    pub qpu_shots: u64,
    pub qpu_circuits: u64,
    pub verification_operations: u64,
}

impl ResourceRequest {
    pub fn validate(&self) -> Result<(), ResourceError> {
        /*
         * Zero means "not requested" rather than an invalid workload estimate.
         * This allows callers to provide only the dimensions they know.
         */
        if self.parallel_workers == usize::MAX {
            return Err(ResourceError::InvalidLimit {
                reason: "parallel worker request cannot use usize::MAX",
            });
        }

        Ok(())
    }

    /// Computes a surface-code request without allocating.
    ///
    /// This is deliberately conservative.
    pub fn surface_code(
        distance: usize,
        stabilizer_weight: usize,
    ) -> Result<Self, ResourceError> {
        let distance_u64 =
            u64::try_from(distance).map_err(|_| {
                ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::CodeDistance,
                }
            })?;

        let d2 = distance
            .checked_mul(distance)
            .ok_or(ResourceError::ArithmeticOverflow {
                resource: ResourceKind::Qubits,
            })?;

        let stabilizers = d2
            .checked_sub(1)
            .ok_or(ResourceError::ArithmeticOverflow {
                resource: ResourceKind::Stabilizers,
            })?;

        let memory_bytes = d2
            .checked_mul(
                std::mem::size_of::<usize>(),
            )
            .and_then(|v| v.checked_mul(8))
            .ok_or(ResourceError::ArithmeticOverflow {
                resource: ResourceKind::MemoryBytes,
            })?;

        Ok(Self {
            code_distance: distance_u64,
            qubits: d2 as u64,
            stabilizers: stabilizers as u64,
            stabilizer_weight: stabilizer_weight as usize,
            memory_bytes: memory_bytes as u64,
            ..Self::default()
        })
    }

    /// Computes a graph request without allocating.
    pub fn graph(
        nodes: usize,
        edges: usize,
    ) -> Result<Self, ResourceError> {
        let memory_nodes = (nodes as u64)
            .checked_mul(
                std::mem::size_of::<usize>() as u64,
            )
            .ok_or(ResourceError::ArithmeticOverflow {
                resource: ResourceKind::MemoryBytes,
            })?;

        let memory_edges = (edges as u64)
            .checked_mul(
                std::mem::size_of::<usize>() as u64,
            )
            .ok_or(ResourceError::ArithmeticOverflow {
                resource: ResourceKind::MemoryBytes,
            })?;

        let memory_bytes =
            memory_nodes
                .checked_add(memory_edges)
                .ok_or(ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::MemoryBytes,
                })?;

        Ok(Self {
            graph_nodes: nodes as u64,
            graph_edges: edges as u64,
            memory_bytes,
            ..Self::default()
        })
    }
}

/* ========================================================================== */
/* RAII reservations                                                          */
/* ========================================================================== */

/// RAII memory reservation.
pub struct MemoryReservation<'a> {
    manager: &'a ResourceManager,
    bytes: u64,
    active: bool,
}

impl<'a> MemoryReservation<'a> {
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        self.manager.release_memory(self.bytes);
        self.active = false;

        true
    }
}

impl Drop for MemoryReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            self.manager.release_memory(self.bytes);
            self.active = false;
        }
    }
}

/// RAII worker reservation.
pub struct WorkerReservation<'a> {
    manager: &'a ResourceManager,
    workers: usize,
    active: bool,
}

impl<'a> WorkerReservation<'a> {
    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        self.manager.release_workers(self.workers);
        self.active = false;

        true
    }
}

impl Drop for WorkerReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            self.manager.release_workers(self.workers);
            self.active = false;
        }
    }
}

/* ========================================================================== */
/* Operation scope                                                            */
/* ========================================================================== */

/// A bounded logical QEC operation.
///
/// All resource reservations made through this scope are constrained by both:
///
/// ```text
/// QecLimits
///     AND
/// ResourceQuota
/// ```
pub struct ResourceScope<'a> {
    manager: &'a ResourceManager,
    name: String,
    quota: ResourceQuota,
    started: Instant,
}

impl<'a> ResourceScope<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quota(&self) -> ResourceQuota {
        self.quota
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    pub fn check(&self) -> Result<(), ResourceError> {
        self.manager.check()?;

        if let Some(limit) = self.quota.max_wall_time {
            let elapsed = self.elapsed();

            if elapsed > limit {
                return Err(
                    ResourceError::WallTimeLimitExceeded {
                        elapsed,
                        limit,
                    },
                );
            }
        }

        Ok(())
    }

    pub fn check_qec(&self) -> QecResult<()> {
        self.check().map_err(Into::into)
    }

    pub fn reserve_memory(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'a>, ResourceError> {
        self.manager.reserve_memory_with_quota(
            bytes,
            self.quota.max_memory_bytes,
        )?;

        Ok(MemoryReservation {
            manager: self.manager,
            bytes,
            active: true,
        })
    }

    pub fn record_syndrome_events(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::SyndromeEvents,
            &self.manager.counters.syndrome_events,
            count,
            self.manager.limits.max_syndrome_events as u64,
            self.quota.max_syndrome_events,
        )
    }

    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::GraphNodes,
            &self.manager.counters.graph_nodes,
            count,
            self.manager.limits.max_graph_nodes as u64,
            self.quota.max_graph_nodes.map(|v| v as u64),
        )
    }

    pub fn record_graph_edges(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::GraphEdges,
            &self.manager.counters.graph_edges,
            count,
            self.manager.limits.max_graph_edges as u64,
            self.quota.max_graph_edges.map(|v| v as u64),
        )
    }

    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::DecoderIterations,
            &self.manager.counters.decoder_iterations,
            count,
            self.manager.limits.max_decoder_iterations as u64,
            self.quota.max_decoder_iterations,
        )
    }

    pub fn record_qpu_shots(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::QpuShots,
            &self.manager.counters.qpu_shots,
            count,
            self.manager.limits.max_qpu_shots,
            self.quota.max_qpu_shots,
        )
    }

    pub fn record_qpu_circuits(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::QpuCircuits,
            &self.manager.counters.qpu_circuits,
            count,
            self.manager.limits.max_qpu_circuits,
            self.quota.max_qpu_circuits,
        )
    }

    pub fn record_verification_operations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.manager.check()?;

        self.manager.try_add(
            ResourceKind::VerificationOperations,
            &self.manager.counters.verification_operations,
            count,
            self.manager.limits.max_verification_operations,
            self.quota.max_verification_operations,
        )
    }

    pub fn acquire_workers(
        &self,
        workers: usize,
    ) -> Result<WorkerReservation<'a>, ResourceError> {
        self.manager.acquire_workers_with_quota(
            workers,
            self.quota.max_parallelism,
        )?;

        Ok(WorkerReservation {
            manager: self.manager,
            workers,
            active: true,
        })
    }

    pub fn preflight(
        &self,
        request: &ResourceRequest,
    ) -> Result<(), ResourceError> {
        self.check()?;
        self.manager.preflight(request)?;

        /*
         * Apply the operation quota as a second, stricter boundary.
         */
        if let Some(limit) = self.quota.max_memory_bytes {
            if request.memory_bytes > limit {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::MemoryBytes,
                    requested: request.memory_bytes,
                    current: 0,
                    limit,
                });
            }
        }

        if let Some(limit) = self.quota.max_qubits {
            if request.qubits > limit as u64 {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::Qubits,
                    requested: request.qubits,
                    current: 0,
                    limit: limit as u64,
                });
            }
        }

        if let Some(limit) = self.quota.max_graph_nodes {
            if request.graph_nodes > limit as u64 {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::GraphNodes,
                    requested: request.graph_nodes,
                    current: 0,
                    limit: limit as u64,
                });
            }
        }

        if let Some(limit) = self.quota.max_graph_edges {
            if request.graph_edges > limit as u64 {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::GraphEdges,
                    requested: request.graph_edges,
                    current: 0,
                    limit: limit as u64,
                });
            }
        }

        if let Some(limit) = self.quota.max_qpu_shots {
            if request.qpu_shots > limit {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::QpuShots,
                    requested: request.qpu_shots,
                    current: 0,
                    limit,
                });
            }
        }

        if let Some(limit) = self.quota.max_verification_operations {
            if request.verification_operations > limit {
                return Err(ResourceError::QuotaExceeded {
                    resource: ResourceKind::VerificationOperations,
                    requested: request.verification_operations,
                    current: 0,
                    limit,
                });
            }
        }

        Ok(())
    }

    pub fn snapshot(&self) -> ResourceSnapshot {
        self.manager.snapshot()
    }
}

/* ========================================================================== */
/* Canonical error conversion                                                 */
/* ========================================================================== */

impl From<ResourceError> for QecError {
    fn from(error: ResourceError) -> Self {
        match error {
            ResourceError::InvalidLimit { reason } => {
                QecError::invalid_input(reason)
            }

            ResourceError::PolicyInvalid {
                resource,
                message,
            } => {
                QecError::resource_limit(
                    resource.to_qec_kind(),
                    0,
                    0,
                    0,
                    message,
                )
            }

            ResourceError::LimitExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                if resource == ResourceKind::MemoryBytes {
                    QecError::memory_limit(
                        requested,
                        current,
                        limit,
                        format!(
                            "{resource} resource limit exceeded"
                        ),
                    )
                } else {
                    QecError::resource_limit(
                        resource.to_qec_kind(),
                        requested as u128,
                        current as u128,
                        limit as u128,
                        format!(
                            "{resource} resource limit exceeded"
                        ),
                    )
                }
            }

            ResourceError::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => {
                QecError::resource_limit(
                    QecResourceKind::Parallelism,
                    requested as u128,
                    current as u128,
                    limit as u128,
                    "parallelism resource limit exceeded",
                )
            }

            ResourceError::QuotaExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                QecError::resource_limit(
                    resource.to_qec_kind(),
                    requested as u128,
                    current as u128,
                    limit as u128,
                    "operation resource quota exceeded",
                )
            }

            ResourceError::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => {
                QecError::resource_limit(
                    QecResourceKind::Parallelism,
                    requested as u128,
                    current as u128,
                    limit as u128,
                    "operation parallelism quota exceeded",
                )
            }

            ResourceError::ArithmeticOverflow { resource } => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    format!(
                        "resource accounting overflow for {resource}"
                    ),
                )
            }

            ResourceError::WallTimeLimitExceeded {
                elapsed,
                limit,
            } => {
                let elapsed_nanos =
                    u64::try_from(elapsed.as_nanos())
                        .unwrap_or(u64::MAX);

                let limit_nanos =
                    u64::try_from(limit.as_nanos())
                        .unwrap_or(u64::MAX);

                QecError::time_limit(
                    elapsed_nanos,
                    limit_nanos,
                    "QEC wall-time limit exceeded",
                )
            }

            ResourceError::Cancelled => {
                QecError::cancelled(
                    "QEC resource operation cancelled",
                )
            }
        }
    }
}

/* ========================================================================== */
/* Shared helpers                                                             */
/* ========================================================================== */

/// Creates a shareable canonical resource manager.
pub fn shared(
    limits: QecLimits,
) -> Result<Arc<ResourceManager>, ResourceError> {
    ResourceManager::shared(limits)
}

/// Compatibility shared constructor.
#[allow(deprecated)]
pub fn shared_legacy(
    limits: ResourceLimits,
) -> Result<Arc<ResourceManager>, ResourceError> {
    ResourceManager::from_resource_limits(limits)
}

fn to_usize(
    value: u64,
    resource: LimitKind,
) -> Result<usize, ResourceError> {
    usize::try_from(value).map_err(|_| {
        ResourceError::ArithmeticOverflow {
            resource: match resource {
                LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
                LimitKind::SyndromeEvents => {
                    ResourceKind::SyndromeEvents
                }
                LimitKind::GraphNodes => ResourceKind::GraphNodes,
                LimitKind::GraphEdges => ResourceKind::GraphEdges,
                LimitKind::DecoderIterations => {
                    ResourceKind::DecoderIterations
                }
                LimitKind::CodeDistance => {
                    ResourceKind::CodeDistance
                }
                LimitKind::Qubits => ResourceKind::Qubits,
                LimitKind::Stabilizers => {
                    ResourceKind::Stabilizers
                }
                LimitKind::MeasurementRounds => {
                    ResourceKind::MeasurementRounds
                }
                LimitKind::CheckpointSizeBytes => {
                    ResourceKind::CheckpointSizeBytes
                }
                LimitKind::Parallelism => {
                    ResourceKind::ParallelWorkers
                }
                LimitKind::Partitions => {
                    ResourceKind::Partitions
                }
                LimitKind::StreamBufferEvents => {
                    ResourceKind::StreamBufferEvents
                }
                LimitKind::StabilizerWeight => {
                    ResourceKind::StabilizerWeight
                }
                LimitKind::LogicalOperatorWeight => {
                    ResourceKind::LogicalOperatorWeight
                }
                LimitKind::QubitsPerPartition => {
                    ResourceKind::QubitsPerPartition
                }
                LimitKind::QpuShots => ResourceKind::QpuShots,
                LimitKind::QpuCircuits => {
                    ResourceKind::QpuCircuits
                }
                LimitKind::VerificationOperations => {
                    ResourceKind::VerificationOperations
                }
            },
        }
    })
}

fn store_max_usize(
    target: &AtomicUsize,
    value: usize,
) {
    let mut current =
        target.load(Ordering::Acquire);

    while value > current {
        match target.compare_exchange_weak(
            current,
            value,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => break,
            Err(actual) => current = actual,
        }
    }
}

fn saturating_sub_u64(
    counter: &AtomicU64,
    amount: u64,
) {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        let next = current.saturating_sub(amount);

        match counter.compare_exchange_weak(
            current,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return,
            Err(actual) => current = actual,
        }
    }
}

fn saturating_sub_usize(
    counter: &AtomicUsize,
    amount: usize,
) {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        let next = current.saturating_sub(amount);

        match counter.compare_exchange_weak(
            current,
            next,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return,
            Err(actual) => current = actual,
        }
    }
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_limits_create_manager() {
        let manager =
            ResourceManager::new(QecLimits::default())
                .expect("default QecLimits must be valid");

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn memory_reservation_is_raii() {
        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 1024;

        let manager =
            ResourceManager::new(limits).unwrap();

        {
            let reservation =
                manager.reserve_memory(512).unwrap();

            assert_eq!(
                manager.snapshot().allocated_bytes,
                512
            );

            assert_eq!(reservation.bytes(), 512);
        }

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );

        assert_eq!(
            manager.snapshot().peak_bytes,
            512
        );
    }

    #[test]
    fn memory_limit_is_enforced_atomically() {
        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 100;

        let manager =
            ResourceManager::new(limits).unwrap();

        manager.reserve_memory(100).unwrap();

        let result =
            manager.reserve_memory(1);

        assert!(matches!(
            result,
            Err(ResourceError::LimitExceeded {
                resource: ResourceKind::MemoryBytes,
                ..
            })
        ));
    }

    #[test]
    fn worker_reservation_is_raii() {
        let mut limits = QecLimits::default();
        limits.max_parallelism = 2;

        let manager =
            ResourceManager::new(limits).unwrap();

        {
            let workers =
                manager.acquire_workers(2).unwrap();

            assert_eq!(
                manager.snapshot().parallel_workers,
                2
            );

            assert_eq!(workers.workers(), 2);
        }

        assert_eq!(
            manager.snapshot().parallel_workers,
            0
        );
    }

    #[test]
    fn cancellation_is_enforced() {
        let manager =
            ResourceManager::new(QecLimits::default())
                .unwrap();

        manager.cancel();

        assert!(matches!(
            manager.check(),
            Err(ResourceError::Cancelled)
        ));
    }

    #[test]
    fn operation_quota_cannot_expand_global_limit() {
        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 100;

        let manager =
            ResourceManager::new(limits).unwrap();

        let quota = ResourceQuota {
            max_memory_bytes: Some(1_000),
            ..ResourceQuota::default()
        };

        let scope =
            manager.scope("test", quota).unwrap();

        assert!(scope.reserve_memory(101).is_err());
    }

    #[test]
    fn operation_quota_can_tighten_global_limit() {
        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 1_000;

        let manager =
            ResourceManager::new(limits).unwrap();

        let quota = ResourceQuota {
            max_memory_bytes: Some(100),
            ..ResourceQuota::default()
        };

        let scope =
            manager.scope("test", quota).unwrap();

        scope.reserve_memory(100).unwrap();

        assert!(scope.reserve_memory(1).is_err());
    }

    #[test]
    fn preflight_performs_no_allocation() {
        let mut limits = QecLimits::default();
        limits.max_qubits = 1_000;

        let manager =
            ResourceManager::new(limits).unwrap();

        let request = ResourceRequest {
            qubits: 1_000,
            memory_bytes: 512,
            ..ResourceRequest::default()
        };

        manager.preflight(&request).unwrap();

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn preflight_rejects_large_request_before_construction() {
        let mut limits = QecLimits::default();
        limits.max_qubits = 100;

        let manager =
            ResourceManager::new(limits).unwrap();

        let request = ResourceRequest {
            qubits: 101,
            ..ResourceRequest::default()
        };

        assert!(matches!(
            manager.preflight(&request),
            Err(ResourceError::LimitExceeded {
                resource: ResourceKind::Qubits,
                ..
            })
        ));
    }

    #[test]
    fn qpu_limits_are_runtime_enforced() {
        let mut limits = QecLimits::default();
        limits.max_qpu_shots = 10;

        let manager =
            ResourceManager::new(limits).unwrap();

        manager.record_qpu_shots(10).unwrap();

        assert!(
            manager.record_qpu_shots(1).is_err()
        );
    }

    #[test]
    fn verification_budget_is_runtime_enforced() {
        let mut limits = QecLimits::default();
        limits.max_verification_operations = 10;

        let manager =
            ResourceManager::new(limits).unwrap();

        manager
            .record_verification_operations(10)
            .unwrap();

        assert!(
            manager
                .record_verification_operations(1)
                .is_err()
        );
    }

    #[test]
    fn surface_code_request_uses_checked_arithmetic() {
        let request =
            ResourceRequest::surface_code(5, 4)
                .unwrap();

        assert_eq!(request.code_distance, 5);
        assert_eq!(request.qubits, 25);
        assert_eq!(request.stabilizer_weight, 4);
    }

    #[test]
    fn snapshot_is_zero_for_new_manager() {
        let manager =
            ResourceManager::new(QecLimits::default())
                .unwrap();

        let snapshot = manager.snapshot();

        assert_eq!(snapshot.allocated_bytes, 0);
        assert_eq!(snapshot.parallel_workers, 0);
        assert_eq!(snapshot.syndrome_events, 0);
        assert_eq!(snapshot.graph_nodes, 0);
        assert_eq!(snapshot.graph_edges, 0);
        assert_eq!(snapshot.qpu_shots, 0);
        assert_eq!(snapshot.verification_operations, 0);
    }
}