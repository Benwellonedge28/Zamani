//! Runtime resource accounting for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! `limits.rs` owns the canonical declarative QEC resource policy.
//! `resources.rs` owns runtime accounting, admission, reservations and
//! operation-local quotas.
//! `memory.rs` owns actual memory-allocation enforcement.
//! `cancellation.rs` owns cooperative cancellation.
//! `errors.rs` owns the canonical public QEC error boundary.
//!
//! Dependency direction:
//!
//! ```text
//!                     QecLimits
//!                         │
//!                         ▼
//!                 ResourceManager
//!                    │     │
//!          ┌─────────┘     └──────────┐
//!          ▼                          ▼
//!   ResourceScope              ResourceSnapshot
//!          │
//!     ┌────┼──────────────┐
//!     ▼    ▼              ▼
//!  Memory Workers      Counters
//!
//! CancellationToken ───────────────► admission checks
//!
//! memory.rs ───────────────────────► actual allocation enforcement
//!
//! errors.rs ◄────────────────────── all public failures
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - runtime resource accounting;
//! - resource admission;
//! - allocation-free preflight;
//! - atomic counter reservation;
//! - RAII memory reservations;
//! - RAII worker reservations;
//! - operation-local quotas;
//! - resource snapshots;
//! - runtime wall-time enforcement;
//! - resource request estimation;
//! - compatibility adapters for legacy callers.
//!
//! This module does NOT own:
//!
//! - global resource policy;
//! - actual allocator management;
//! - decoder algorithms;
//! - QPU execution;
//! - scheduling policy;
//! - authorization;
//! - telemetry transport;
//! - checkpoint serialization.
//!
//! # Important distinction
//!
//! ```text
//! limits.rs
//!     = what the execution is allowed to consume
//!
//! resources.rs
//!     = what the execution has consumed / reserved
//!
//! memory.rs
//!     = actual memory allocation enforcement
//!
//! cancellation.rs
//!     = whether execution should stop
//! ```
//!
//! # Resource semantics
//!
//! There are two kinds of runtime quantities:
//!
//! 1. **Live reservations**
//!    - memory;
//!    - worker slots.
//!
//!    These increase when acquired and decrease when released.
//!
//! 2. **Monotonic consumption counters**
//!    - syndrome events;
//!    - graph nodes;
//!    - graph edges;
//!    - decoder iterations;
//!    - QPU shots;
//!    - verification operations;
//!    - etc.
//!
//! Consumption counters represent cumulative work within the manager's
//! lifetime and therefore do not decrease when an operation finishes.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No unstable language features are used.

use core::fmt;
use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use super::{
    cancellation::CancellationToken,
    errors::{
        NumericalOperation,
        QecError,
        QecResult,
        ResourceKind as QecResourceKind,
    },
    limits::{LimitError, LimitKind, QecLimits},
};

/* ========================================================================== */
/* Compatibility constants                                                    */
/* ========================================================================== */

/// Application-level "no additional finite ceiling" sentinel.
///
/// This never means physically infinite memory or compute.
pub const UNLIMITED_U64: u64 = u64::MAX;

/// Application-level "no additional finite worker ceiling" sentinel.
///
/// This never means physically infinite parallelism.
pub const UNLIMITED_USIZE: usize = usize::MAX;

/* ========================================================================== */
/* Runtime resource kinds                                                      */
/* ========================================================================== */

/// Runtime dimensions tracked by [`ResourceManager`].
///
/// This is deliberately distinct from [`LimitKind`].
///
/// `LimitKind` describes declarative policy.
/// `ResourceKind` describes runtime accounting.
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
    /// Stable machine-readable identifier.
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

    /// Converts the runtime dimension to the canonical QEC error dimension.
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
            Self::StabilizerWeight => QecResourceKind::StabilizerWeight,
            Self::LogicalOperatorWeight => QecResourceKind::LogicalWeight,
            Self::QubitsPerPartition => QecResourceKind::Custom,
            Self::QpuShots => QecResourceKind::QpuShots,
            Self::QpuCircuits => QecResourceKind::QpuCircuits,
            Self::VerificationOperations => QecResourceKind::Operations,
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* ========================================================================== */
/* Legacy compatibility policy                                                */
/* ========================================================================== */

/// Legacy compatibility adapter.
///
/// New code must use [`QecLimits`].
///
/// This type intentionally remains only as a migration boundary so existing
/// callers do not need to be rewritten simultaneously.
#[deprecated(
    note = "use limits::QecLimits and ResourceManager::new instead"
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
        Self::from_qec_limits(&QecLimits::default())
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
        self.validate()?;

        let mut limits = QecLimits::default();

        limits.max_memory_bytes = self.max_memory_bytes;
        limits.max_syndrome_events =
            checked_u64_to_usize(
                self.max_syndrome_events,
                ResourceKind::SyndromeEvents,
            )?;
        limits.max_graph_nodes =
            checked_u64_to_usize(
                self.max_graph_nodes,
                ResourceKind::GraphNodes,
            )?;
        limits.max_graph_edges =
            checked_u64_to_usize(
                self.max_graph_edges,
                ResourceKind::GraphEdges,
            )?;
        limits.max_decoder_iterations =
            checked_u64_to_usize(
                self.max_decoder_iterations,
                ResourceKind::DecoderIterations,
            )?;
        limits.max_parallelism = self.max_parallelism;

        if let Some(duration) = self.max_wall_time {
            limits.max_decoder_time_ns =
                duration_to_u64_nanos(duration)?;
        }

        limits.validate().map_err(ResourceError::from)
    }

    pub fn from_qec_limits(limits: &QecLimits) -> Self {
        Self {
            max_memory_bytes: limits.max_memory_bytes,
            max_syndrome_events:
                limits.max_syndrome_events as u64,
            max_graph_nodes:
                limits.max_graph_nodes as u64,
            max_graph_edges:
                limits.max_graph_edges as u64,
            max_decoder_iterations:
                limits.max_decoder_iterations as u64,
            max_parallelism: limits.max_parallelism,
            max_wall_time: Some(
                Duration::from_nanos(
                    limits.max_decoder_time_ns,
                ),
            ),
        }
    }
}

/* ========================================================================== */
/* Operation quota                                                             */
/* ========================================================================== */

/// Optional operation-local limits.
///
/// Every configured quota is a stricter ceiling for one operation.
///
/// A quota can never increase the corresponding global [`QecLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

impl ResourceQuota {
    pub fn validate(&self) -> Result<(), ResourceError> {
        macro_rules! require_nonzero {
            ($value:expr, $reason:expr) => {
                if let Some(value) = $value {
                    if value == 0 {
                        return Err(ResourceError::InvalidLimit {
                            reason: $reason,
                        });
                    }
                }
            };
        }

        require_nonzero!(
            self.max_memory_bytes,
            "operation memory quota must be greater than zero"
        );
        require_nonzero!(
            self.max_syndrome_events,
            "operation syndrome quota must be greater than zero"
        );
        require_nonzero!(
            self.max_graph_nodes,
            "operation graph-node quota must be greater than zero"
        );
        require_nonzero!(
            self.max_graph_edges,
            "operation graph-edge quota must be greater than zero"
        );
        require_nonzero!(
            self.max_decoder_iterations,
            "operation decoder-iteration quota must be greater than zero"
        );
        require_nonzero!(
            self.max_parallelism,
            "operation parallelism quota must be greater than zero"
        );
        require_nonzero!(
            self.max_code_distance,
            "operation code-distance quota must be greater than zero"
        );
        require_nonzero!(
            self.max_qubits,
            "operation qubit quota must be greater than zero"
        );
        require_nonzero!(
            self.max_stabilizers,
            "operation stabilizer quota must be greater than zero"
        );
        require_nonzero!(
            self.max_rounds,
            "operation round quota must be greater than zero"
        );
        require_nonzero!(
            self.max_checkpoint_size_bytes,
            "operation checkpoint quota must be greater than zero"
        );
        require_nonzero!(
            self.max_partitions,
            "operation partition quota must be greater than zero"
        );
        require_nonzero!(
            self.max_stream_buffer_events,
            "operation stream-buffer quota must be greater than zero"
        );
        require_nonzero!(
            self.max_stabilizer_weight,
            "operation stabilizer-weight quota must be greater than zero"
        );
        require_nonzero!(
            self.max_logical_operator_weight,
            "operation logical-weight quota must be greater than zero"
        );
        require_nonzero!(
            self.max_qubits_per_partition,
            "operation partition-qubit quota must be greater than zero"
        );
        require_nonzero!(
            self.max_qpu_shots,
            "operation QPU-shot quota must be greater than zero"
        );
        require_nonzero!(
            self.max_qpu_circuits,
            "operation QPU-circuit quota must be greater than zero"
        );
        require_nonzero!(
            self.max_verification_operations,
            "operation verification quota must be greater than zero"
        );

        Ok(())
    }
}

/* ========================================================================== */
/* Runtime snapshot                                                            */
/* ========================================================================== */

/// Immutable runtime resource state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSnapshot {
    /// Currently reserved memory.
    pub allocated_bytes: u64,

    /// Highest simultaneous memory reservation.
    pub peak_bytes: u64,

    /// Cumulative syndrome/detection events.
    pub syndrome_events: u64,

    /// Cumulative graph nodes admitted.
    pub graph_nodes: u64,

    /// Cumulative graph edges admitted.
    pub graph_edges: u64,

    /// Cumulative decoder iterations.
    pub decoder_iterations: u64,

    /// Currently reserved worker slots.
    pub parallel_workers: usize,

    /// Highest code distance observed.
    pub code_distance: usize,

    /// Cumulative qubits admitted.
    pub qubits: usize,

    /// Cumulative stabilizers admitted.
    pub stabilizers: usize,

    /// Cumulative measurement rounds admitted.
    pub measurement_rounds: usize,

    /// Cumulative checkpoint bytes.
    pub checkpoint_bytes: u64,

    /// Cumulative partitions admitted.
    pub partitions: usize,

    /// Cumulative stream-buffer events.
    pub stream_buffer_events: usize,

    /// Cumulative QPU shots.
    pub qpu_shots: u64,

    /// Cumulative QPU circuits.
    pub qpu_circuits: u64,

    /// Cumulative mathematical-verification operations.
    pub verification_operations: u64,

    /// Wall-clock time since manager creation.
    pub wall_time: Duration,

    /// Backend/decoder-reported compute time.
    pub compute_time: Duration,
}

impl ResourceSnapshot {
    pub fn is_idle(&self) -> bool {
        self.allocated_bytes == 0
            && self.parallel_workers == 0
    }
}

/* ========================================================================== */
/* Resource errors                                                             */
/* ========================================================================== */

/// Runtime resource-management error.
///
/// Policy-definition errors originate in `limits.rs`.
/// Runtime enforcement errors originate here.
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

    AccountingInvariantViolation {
        resource: ResourceKind,
        attempted_release: u64,
        current: u64,
    },
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
                    "{resource} limit exceeded: \
                     requested={requested}, current={current}, limit={limit}"
                )
            }

            Self::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism limit exceeded: \
                     requested={requested}, current={current}, limit={limit}"
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
                    "{resource} operation quota exceeded: \
                     requested={requested}, current={current}, quota={limit}"
                )
            }

            Self::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => {
                write!(
                    f,
                    "parallelism operation quota exceeded: \
                     requested={requested}, current={current}, quota={limit}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "resource accounting arithmetic overflow for {resource}"
                )
            }

            Self::WallTimeLimitExceeded {
                elapsed,
                limit,
            } => {
                write!(
                    f,
                    "QEC wall-time limit exceeded: \
                     elapsed={elapsed:?}, limit={limit:?}"
                )
            }

            Self::Cancelled => {
                f.write_str("QEC resource operation cancelled")
            }

            Self::AccountingInvariantViolation {
                resource,
                attempted_release,
                current,
            } => {
                write!(
                    f,
                    "resource accounting invariant violation for {resource}: \
                     attempted release={attempted_release}, current={current}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceError {}

/* ========================================================================== */
/* Limit error conversion                                                      */
/* ========================================================================== */

impl From<LimitError> for ResourceError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::InvalidLimit { resource, value } => {
                Self::PolicyInvalid {
                    resource,
                    message: format!(
                        "configured value {value} must be greater than zero"
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
                    message: "policy arithmetic overflow".to_owned(),
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

            LimitError::UnsupportedSchema {
                found,
                expected,
            } => {
                Self::PolicyInvalid {
                    resource: LimitKind::MemoryBytes,
                    message: format!(
                        "unsupported limits schema {found}; expected {expected}"
                    ),
                }
            }
        }
    }
}

/* ========================================================================== */
/* Atomic counters                                                             */
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
}

/* ========================================================================== */
/* Resource manager                                                            */
/* ========================================================================== */

/// Thread-safe runtime resource manager.
///
/// The manager is suitable for sharing across decoder workers through `Arc`.
#[derive(Debug)]
pub struct ResourceManager {
    limits: QecLimits,
    counters: ResourceCounters,
    cancellation: CancellationToken,
    started: Instant,
}

impl ResourceManager {
    /// Creates a manager from canonical QEC limits.
    pub fn new(limits: QecLimits) -> Result<Self, ResourceError> {
        Self::with_cancellation(limits, CancellationToken::new())
    }

    /// Creates a manager using an externally owned cancellation token.
    ///
    /// This is the preferred integration point for:
    ///
    /// - scheduler.rs;
    /// - decoder.rs;
    /// - streaming.rs;
    /// - partition.rs;
    /// - distributed.rs;
    /// - checkpoint.rs;
    /// - QPU execution.
    pub fn with_cancellation(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> Result<Self, ResourceError> {
        limits.validate()?;

        Ok(Self {
            limits,
            counters: ResourceCounters::default(),
            cancellation,
            started: Instant::now(),
        })
    }

    /// Explicit canonical constructor alias.
    pub fn from_qec_limits(
        limits: QecLimits,
    ) -> Result<Self, ResourceError> {
        Self::new(limits)
    }

    /// Legacy constructor.
    #[allow(deprecated)]
    pub fn from_resource_limits(
        limits: ResourceLimits,
    ) -> Result<Self, ResourceError> {
        Self::new(limits.into_qec_limits()?)
    }

    /// Returns the canonical immutable policy.
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the cancellation token used by this manager.
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Creates a shareable manager.
    pub fn shared(
        limits: QecLimits,
    ) -> Result<Arc<Self>, ResourceError> {
        Ok(Arc::new(Self::new(limits)?))
    }

    /// Requests cancellation through the canonical cancellation subsystem.
    pub fn cancel(&self) {
        self.cancellation.request();
    }

    /// Returns whether the workload is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Checks cancellation and the global wall-clock budget.
    pub fn check(&self) -> Result<(), ResourceError> {
        self.cancellation
            .check()
            .map_err(|_| ResourceError::Cancelled)?;

        let elapsed = self.started.elapsed();
        let limit =
            Duration::from_nanos(self.limits.max_decoder_time_ns);

        if elapsed > limit {
            return Err(ResourceError::WallTimeLimitExceeded {
                elapsed,
                limit,
            });
        }

        Ok(())
    }

    /// Canonical QEC-error form of [`Self::check`].
    pub fn check_qec(&self) -> QecResult<()> {
        self.check().map_err(Into::into)
    }

    /* ---------------------------------------------------------------------- */
    /* Snapshot                                                                 */
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
    /* Allocation-free preflight                                               */
    /* ---------------------------------------------------------------------- */

    /// Validates a complete workload request without changing runtime state.
    pub fn preflight(
        &self,
        request: &ResourceRequest,
    ) -> Result<(), ResourceError> {
        self.check()?;
        request.validate()?;

        self.check_request_limit(
            ResourceKind::CodeDistance,
            request.code_distance,
            self.limits.max_code_distance as u64,
        )?;

        self.check_request_limit(
            ResourceKind::Qubits,
            request.qubits,
            self.limits.max_qubits as u64,
        )?;

        self.check_request_limit(
            ResourceKind::Stabilizers,
            request.stabilizers,
            self.limits.max_stabilizers as u64,
        )?;

        self.check_request_limit(
            ResourceKind::SyndromeEvents,
            request.syndrome_events,
            self.limits.max_syndrome_events as u64,
        )?;

        self.check_request_limit(
            ResourceKind::MeasurementRounds,
            request.measurement_rounds,
            self.limits.max_rounds as u64,
        )?;

        self.check_request_limit(
            ResourceKind::GraphNodes,
            request.graph_nodes,
            self.limits.max_graph_nodes as u64,
        )?;

        self.check_request_limit(
            ResourceKind::GraphEdges,
            request.graph_edges,
            self.limits.max_graph_edges as u64,
        )?;

        self.check_request_limit(
            ResourceKind::MemoryBytes,
            request.memory_bytes,
            self.limits.max_memory_bytes,
        )?;

        self.check_request_limit(
            ResourceKind::DecoderIterations,
            request.decoder_iterations,
            self.limits.max_decoder_iterations as u64,
        )?;

        self.check_request_limit(
            ResourceKind::ParallelWorkers,
            usize_to_u64(
                request.parallel_workers,
                ResourceKind::ParallelWorkers,
            )?,
            self.limits.max_parallelism as u64,
        )?;

        self.check_request_limit(
            ResourceKind::CheckpointSizeBytes,
            request.checkpoint_size_bytes,
            self.limits.max_checkpoint_size_bytes,
        )?;

        self.check_request_limit(
            ResourceKind::Partitions,
            usize_to_u64(
                request.partitions,
                ResourceKind::Partitions,
            )?,
            self.limits.max_partitions as u64,
        )?;

        self.check_request_limit(
            ResourceKind::StreamBufferEvents,
            usize_to_u64(
                request.stream_buffer_events,
                ResourceKind::StreamBufferEvents,
            )?,
            self.limits.max_stream_buffer_events as u64,
        )?;

        self.check_request_limit(
            ResourceKind::StabilizerWeight,
            usize_to_u64(
                request.stabilizer_weight,
                ResourceKind::StabilizerWeight,
            )?,
            self.limits.max_stabilizer_weight as u64,
        )?;

        self.check_request_limit(
            ResourceKind::LogicalOperatorWeight,
            usize_to_u64(
                request.logical_operator_weight,
                ResourceKind::LogicalOperatorWeight,
            )?,
            self.limits.max_logical_operator_weight as u64,
        )?;

        self.check_request_limit(
            ResourceKind::QubitsPerPartition,
            usize_to_u64(
                request.qubits_per_partition,
                ResourceKind::QubitsPerPartition,
            )?,
            self.limits.max_qubits_per_partition as u64,
        )?;

        self.check_request_limit(
            ResourceKind::QpuShots,
            request.qpu_shots,
            self.limits.max_qpu_shots,
        )?;

        self.check_request_limit(
            ResourceKind::QpuCircuits,
            request.qpu_circuits,
            self.limits.max_qpu_circuits,
        )?;

        self.check_request_limit(
            ResourceKind::VerificationOperations,
            request.verification_operations,
            self.limits.max_verification_operations,
        )?;

        Ok(())
    }

    pub fn available_memory(&self) -> u64 {
        self.limits
            .max_memory_bytes
            .saturating_sub(
                self.counters
                    .allocated_bytes
                    .load(Ordering::Acquire),
            )
    }

    /* ---------------------------------------------------------------------- */
    /* Memory reservations                                                     */
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

        self.try_add_u64(
            ResourceKind::MemoryBytes,
            &self.counters.allocated_bytes,
            bytes,
            self.limits.max_memory_bytes,
            quota,
        )?;

        self.update_peak();

        Ok(())
    }

    pub fn release_memory(
        &self,
        bytes: u64,
    ) -> Result<(), ResourceError> {
        release_u64_checked(
            ResourceKind::MemoryBytes,
            &self.counters.allocated_bytes,
            bytes,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Cumulative counters                                                      */
    /* ---------------------------------------------------------------------- */

    pub fn record_syndrome_events(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::SyndromeEvents,
            &self.counters.syndrome_events,
            count,
            self.limits.max_syndrome_events as u64,
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
    /* Cumulative usize resources                                               */
    /* ---------------------------------------------------------------------- */

    pub fn record_code_distance(
        &self,
        distance: usize,
    ) -> Result<(), ResourceError> {
        let distance_u64 =
            usize_to_u64(distance, ResourceKind::CodeDistance)?;

        self.check_request_limit(
            ResourceKind::CodeDistance,
            distance_u64,
            self.limits.max_code_distance as u64,
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
    /* Worker reservations                                                      */
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

            if let Some(quota_limit) = quota {
                if next > quota_limit {
                    return Err(
                        ResourceError::ParallelismQuotaExceeded {
                            requested: workers,
                            current,
                            limit: quota_limit,
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

    pub fn release_workers(
        &self,
        workers: usize,
    ) -> Result<(), ResourceError> {
        release_usize_checked(
            ResourceKind::ParallelWorkers,
            &self.counters.parallel_workers,
            workers,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Compute time                                                             */
    /* ---------------------------------------------------------------------- */

    pub fn record_compute_time(
        &self,
        duration: Duration,
    ) -> Result<(), ResourceError> {
        let nanos = duration_to_u64_nanos(duration)?;

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
    /* Internal counter helpers                                                */
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

        self.try_add_u64(
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
                    requested: usize_to_u64(
                        requested,
                        resource,
                    )?,
                    current: usize_to_u64(
                        current,
                        resource,
                    )?,
                    limit: usize_to_u64(
                        global_limit,
                        resource,
                    )?,
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

    fn try_add_u64(
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

            if let Some(quota_limit) = quota {
                if next > quota_limit {
                    return Err(ResourceError::QuotaExceeded {
                        resource,
                        requested,
                        current,
                        limit: quota_limit,
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

    fn check_request_limit(
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
/* Resource request                                                            */
/* ========================================================================== */

/// Allocation-free estimate of resources required by an operation.
///
/// Zero means "not requested" for dimensions where zero is a valid omission.
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
        if self.parallel_workers == usize::MAX {
            return Err(ResourceError::InvalidLimit {
                reason: "parallel worker request cannot use usize::MAX",
            });
        }

        Ok(())
    }

    /// Builds a conservative surface-code resource estimate.
    ///
    /// No allocation is performed.
    pub fn surface_code(
        distance: usize,
        stabilizer_weight: usize,
    ) -> Result<Self, ResourceError> {
        let distance_u64 =
            usize_to_u64(
                distance,
                ResourceKind::CodeDistance,
            )?;

        let d2 = distance.checked_mul(distance).ok_or(
            ResourceError::ArithmeticOverflow {
                resource: ResourceKind::Qubits,
            },
        )?;

        let stabilizers = d2.checked_sub(1).ok_or(
            ResourceError::ArithmeticOverflow {
                resource: ResourceKind::Stabilizers,
            },
        )?;

        let d2_u64 =
            usize_to_u64(
                d2,
                ResourceKind::Qubits,
            )?;

        let stabilizers_u64 =
            usize_to_u64(
                stabilizers,
                ResourceKind::Stabilizers,
            )?;

        let weight_u64 =
            usize_to_u64(
                stabilizer_weight,
                ResourceKind::StabilizerWeight,
            )?;

        let node_bytes =
            usize_to_u64(
                std::mem::size_of::<usize>(),
                ResourceKind::MemoryBytes,
            )?;

        let memory_bytes = d2_u64
            .checked_mul(node_bytes)
            .and_then(|value| value.checked_mul(8))
            .ok_or(
                ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::MemoryBytes,
                },
            )?;

        Ok(Self {
            code_distance: distance_u64,
            qubits: d2_u64,
            stabilizers: stabilizers_u64,
            stabilizer_weight:
                checked_u64_to_usize(
                    weight_u64,
                    ResourceKind::StabilizerWeight,
                )?,
            memory_bytes,
            ..Self::default()
        })
    }

    /// Builds a conservative graph resource estimate.
    ///
    /// No allocation is performed.
    pub fn graph(
        nodes: usize,
        edges: usize,
    ) -> Result<Self, ResourceError> {
        let nodes_u64 =
            usize_to_u64(nodes, ResourceKind::GraphNodes)?;

        let edges_u64 =
            usize_to_u64(edges, ResourceKind::GraphEdges)?;

        let element_bytes =
            usize_to_u64(
                std::mem::size_of::<usize>(),
                ResourceKind::MemoryBytes,
            )?;

        let node_memory =
            nodes_u64
                .checked_mul(element_bytes)
                .ok_or(
                    ResourceError::ArithmeticOverflow {
                        resource: ResourceKind::MemoryBytes,
                    },
                )?;

        let edge_memory =
            edges_u64
                .checked_mul(element_bytes)
                .ok_or(
                    ResourceError::ArithmeticOverflow {
                        resource: ResourceKind::MemoryBytes,
                    },
                )?;

        let memory_bytes =
            node_memory.checked_add(edge_memory).ok_or(
                ResourceError::ArithmeticOverflow {
                    resource: ResourceKind::MemoryBytes,
                },
            )?;

        Ok(Self {
            graph_nodes: nodes_u64,
            graph_edges: edges_u64,
            memory_bytes,
            ..Self::default()
        })
    }
}

/* ========================================================================== */
/* RAII memory reservation                                                     */
/* ========================================================================== */

/// RAII reservation for live memory.
pub struct MemoryReservation<'a> {
    manager: &'a ResourceManager,
    bytes: u64,
    active: bool,
}

impl<'a> MemoryReservation<'a> {
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Explicitly releases the reservation.
    ///
    /// Consumes the guard so a second release cannot occur.
    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        let result =
            self.manager.release_memory(self.bytes);

        self.active = false;

        result.is_ok()
    }
}

impl Drop for MemoryReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.manager.release_memory(self.bytes);
            self.active = false;
        }
    }
}

/* ========================================================================== */
/* RAII worker reservation                                                     */
/* ========================================================================== */

/// RAII reservation for live worker slots.
pub struct WorkerReservation<'a> {
    manager: &'a ResourceManager,
    workers: usize,
    active: bool,
}

impl<'a> WorkerReservation<'a> {
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// Explicitly releases the worker reservation.
    pub fn release(
        mut self,
    ) -> bool {
        if !self.active {
            return false;
        }

        let result =
            self.manager.release_workers(self.workers);

        self.active = false;

        result.is_ok()
    }
}

impl Drop for WorkerReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.manager.release_workers(self.workers);
            self.active = false;
        }
    }
}

/* ========================================================================== */
/* Operation scope                                                             */
/* ========================================================================== */

/// A bounded logical operation.
///
/// All reservations and cumulative accounting made through this scope are
/// constrained by both:
///
/// ```text
/// global QecLimits
///        AND
/// operation ResourceQuota
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

    pub fn cancellation_token(&self) -> CancellationToken {
        self.manager.cancellation_token()
    }

    pub fn reserve_memory(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'a>, ResourceError> {
        self.check()?;

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
        self.record_u64(
            ResourceKind::SyndromeEvents,
            count,
            self.manager.limits.max_syndrome_events as u64,
            self.quota.max_syndrome_events,
        )
    }

    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::GraphNodes,
            count,
            self.manager.limits.max_graph_nodes as u64,
            self.quota.max_graph_nodes.map(|v| v as u64),
        )
    }

    pub fn record_graph_edges(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::GraphEdges,
            count,
            self.manager.limits.max_graph_edges as u64,
            self.quota.max_graph_edges.map(|v| v as u64),
        )
    }

    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::DecoderIterations,
            count,
            self.manager
                .limits
                .max_decoder_iterations as u64,
            self.quota.max_decoder_iterations,
        )
    }

    pub fn record_qpu_shots(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::QpuShots,
            count,
            self.manager.limits.max_qpu_shots,
            self.quota.max_qpu_shots,
        )
    }

    pub fn record_qpu_circuits(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::QpuCircuits,
            count,
            self.manager.limits.max_qpu_circuits,
            self.quota.max_qpu_circuits,
        )
    }

    pub fn record_verification_operations(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::VerificationOperations,
            count,
            self.manager
                .limits
                .max_verification_operations,
            self.quota.max_verification_operations,
        )
    }

    pub fn record_qubits(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Qubits,
            count,
            self.manager.limits.max_qubits,
            self.quota.max_qubits,
        )
    }

    pub fn record_stabilizers(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Stabilizers,
            count,
            self.manager.limits.max_stabilizers,
            self.quota.max_stabilizers,
        )
    }

    pub fn record_measurement_rounds(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::MeasurementRounds,
            count,
            self.manager.limits.max_rounds,
            self.quota.max_rounds,
        )
    }

    pub fn record_partitions(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::Partitions,
            count,
            self.manager.limits.max_partitions,
            self.quota.max_partitions,
        )
    }

    pub fn record_stream_buffer_events(
        &self,
        count: usize,
    ) -> Result<(), ResourceError> {
        self.record_usize(
            ResourceKind::StreamBufferEvents,
            count,
            self.manager
                .limits
                .max_stream_buffer_events,
            self.quota.max_stream_buffer_events,
        )
    }

    pub fn record_checkpoint_bytes(
        &self,
        count: u64,
    ) -> Result<(), ResourceError> {
        self.record_u64(
            ResourceKind::CheckpointSizeBytes,
            count,
            self.manager
                .limits
                .max_checkpoint_size_bytes,
            self.quota.max_checkpoint_size_bytes,
        )
    }

    pub fn acquire_workers(
        &self,
        workers: usize,
    ) -> Result<WorkerReservation<'a>, ResourceError> {
        self.check()?;

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

    /// Performs global and operation-local preflight.
    ///
    /// No runtime counters are modified.
    pub fn preflight(
        &self,
        request: &ResourceRequest,
    ) -> Result<(), ResourceError> {
        self.check()?;
        self.manager.preflight(request)?;

        macro_rules! quota_u64 {
            ($field:ident, $kind:expr) => {
                if let Some(limit) = self.quota.$field {
                    if request.$field > limit {
                        return Err(
                            ResourceError::QuotaExceeded {
                                resource: $kind,
                                requested: request.$field,
                                current: 0,
                                limit,
                            },
                        );
                    }
                }
            };
        }

        macro_rules! quota_usize {
            ($field:ident, $kind:expr) => {
                if let Some(limit) = self.quota.$field {
                    let requested = usize_to_u64(
                        request.$field,
                        $kind,
                    )?;

                    let limit_u64 = usize_to_u64(
                        limit,
                        $kind,
                    )?;

                    if requested > limit_u64 {
                        return Err(
                            ResourceError::QuotaExceeded {
                                resource: $kind,
                                requested,
                                current: 0,
                                limit: limit_u64,
                            },
                        );
                    }
                }
            };
        }

        quota_u64!(
            max_memory_bytes,
            ResourceKind::MemoryBytes
        );
        quota_u64!(
            max_syndrome_events,
            ResourceKind::SyndromeEvents
        );
        quota_u64!(
            max_graph_nodes,
            ResourceKind::GraphNodes
        );
        quota_u64!(
            max_graph_edges,
            ResourceKind::GraphEdges
        );
        quota_u64!(
            max_decoder_iterations,
            ResourceKind::DecoderIterations
        );
        quota_u64!(
            max_checkpoint_size_bytes,
            ResourceKind::CheckpointSizeBytes
        );
        quota_u64!(
            max_qpu_shots,
            ResourceKind::QpuShots
        );
        quota_u64!(
            max_qpu_circuits,
            ResourceKind::QpuCircuits
        );
        quota_u64!(
            max_verification_operations,
            ResourceKind::VerificationOperations
        );

        quota_usize!(
            max_parallelism,
            ResourceKind::ParallelWorkers
        );
        quota_usize!(
            max_code_distance,
            ResourceKind::CodeDistance
        );
        quota_usize!(
            max_qubits,
            ResourceKind::Qubits
        );
        quota_usize!(
            max_stabilizers,
            ResourceKind::Stabilizers
        );
        quota_usize!(
            max_rounds,
            ResourceKind::MeasurementRounds
        );
        quota_usize!(
            max_partitions,
            ResourceKind::Partitions
        );
        quota_usize!(
            max_stream_buffer_events,
            ResourceKind::StreamBufferEvents
        );
        quota_usize!(
            max_stabilizer_weight,
            ResourceKind::StabilizerWeight
        );
        quota_usize!(
            max_logical_operator_weight,
            ResourceKind::LogicalOperatorWeight
        );
        quota_usize!(
            max_qubits_per_partition,
            ResourceKind::QubitsPerPartition
        );

        Ok(())
    }

    pub fn snapshot(&self) -> ResourceSnapshot {
        self.manager.snapshot()
    }

    fn record_u64(
        &self,
        resource: ResourceKind,
        count: u64,
        global_limit: u64,
        quota: Option<u64>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        self.manager.try_add_u64(
            resource,
            counter_for_u64(
                resource,
                &self.manager.counters,
            )?,
            count,
            global_limit,
            quota,
        )
    }

    fn record_usize(
        &self,
        resource: ResourceKind,
        count: usize,
        global_limit: usize,
        quota: Option<usize>,
    ) -> Result<(), ResourceError> {
        self.check()?;

        let requested =
            usize_to_u64(count, resource)?;

        let quota_u64 =
            quota.map(|value| {
                usize_to_u64(value, resource)
            });

        let quota_u64 =
            match quota_u64 {
                Some(Ok(value)) => Some(value),
                Some(Err(error)) => return Err(error),
                None => None,
            };

        self.manager.try_add_usize(
            resource,
            counter_for_usize(
                resource,
                &self.manager.counters,
            )?,
            count,
            global_limit,
            quota,
        )?;

        let _ = requested;

        let _ = quota_u64;

        Ok(())
    }
}

/* ========================================================================== */
/* Canonical QEC error conversion                                              */
/* ========================================================================== */

impl From<ResourceError> for QecError {
    fn from(error: ResourceError) -> Self {
        match error {
            ResourceError::InvalidLimit { reason } => {
                QecError::InvalidInput {
                    message: reason.to_owned(),
                }
            }

            ResourceError::PolicyInvalid {
                resource,
                message,
            } => {
                QecError::ResourceLimitExceeded {
                    resource: resource_to_qec_kind(resource),
                    requested: 0,
                    current: 0,
                    limit: 0,
                    message,
                }
            }

            ResourceError::LimitExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                if resource == ResourceKind::MemoryBytes {
                    QecError::MemoryLimitExceeded {
                        requested_bytes: requested,
                        current_bytes: current,
                        limit_bytes: limit,
                        message:
                            "QEC memory resource limit exceeded"
                                .to_owned(),
                    }
                } else {
                    QecError::ResourceLimitExceeded {
                        resource: resource.to_qec_kind(),
                        requested: requested as u128,
                        current: current as u128,
                        limit: limit as u128,
                        message:
                            "QEC resource limit exceeded"
                                .to_owned(),
                    }
                }
            }

            ResourceError::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => {
                QecError::ResourceLimitExceeded {
                    resource: QecResourceKind::Parallelism,
                    requested: requested as u128,
                    current: current as u128,
                    limit: limit as u128,
                    message:
                        "QEC parallelism limit exceeded"
                            .to_owned(),
                }
            }

            ResourceError::QuotaExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                QecError::ResourceLimitExceeded {
                    resource: resource.to_qec_kind(),
                    requested: requested as u128,
                    current: current as u128,
                    limit: limit as u128,
                    message:
                        "QEC operation quota exceeded"
                            .to_owned(),
                }
            }

            ResourceError::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => {
                QecError::ResourceLimitExceeded {
                    resource: QecResourceKind::Parallelism,
                    requested: requested as u128,
                    current: current as u128,
                    limit: limit as u128,
                    message:
                        "QEC operation parallelism quota exceeded"
                            .to_owned(),
                }
            }

            ResourceError::ArithmeticOverflow { resource } => {
                QecError::NumericalFailure {
                    operation: NumericalOperation::IntegerConversion,
                    message: format!(
                        "resource accounting overflow for {resource}"
                    ),
                }
            }

            ResourceError::WallTimeLimitExceeded {
                elapsed,
                limit,
            } => {
                QecError::TimeLimitExceeded {
                    elapsed_nanos:
                        duration_to_u64_nanos_saturating(elapsed),
                    limit_nanos:
                        duration_to_u64_nanos_saturating(limit),
                    message:
                        "QEC wall-time limit exceeded"
                            .to_owned(),
                }
            }

            ResourceError::Cancelled => {
                QecError::CancellationRequested {
                    message:
                        "QEC resource operation cancelled"
                            .to_owned(),
                }
            }

            ResourceError::AccountingInvariantViolation {
                resource,
                attempted_release,
                current,
            } => {
                QecError::InternalInvariantViolation {
                    invariant: format!(
                        "{resource} release cannot exceed current reservation"
                    ),
                    message: format!(
                        "attempted release={attempted_release}, \
                         current={current}"
                    ),
                }
            }
        }
    }
}

/* ========================================================================== */
/* Shared constructors                                                         */
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

/* ========================================================================== */
/* Helper functions                                                            */
/* ========================================================================== */

fn usize_to_u64(
    value: usize,
    resource: ResourceKind,
) -> Result<u64, ResourceError> {
    u64::try_from(value).map_err(|_| {
        ResourceError::ArithmeticOverflow { resource }
    })
}

fn checked_u64_to_usize(
    value: u64,
    resource: ResourceKind,
) -> Result<usize, ResourceError> {
    usize::try_from(value).map_err(|_| {
        ResourceError::ArithmeticOverflow { resource }
    })
}

fn duration_to_u64_nanos(
    duration: Duration,
) -> Result<u64, ResourceError> {
    u64::try_from(duration.as_nanos()).map_err(|_| {
        ResourceError::ArithmeticOverflow {
            resource: ResourceKind::MemoryBytes,
        }
    })
}

fn duration_to_u64_nanos_saturating(
    duration: Duration,
) -> u64 {
    u64::try_from(duration.as_nanos())
        .unwrap_or(u64::MAX)
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

fn release_u64_checked(
    resource: ResourceKind,
    counter: &AtomicU64,
    amount: u64,
) -> Result<(), ResourceError> {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        if amount > current {
            return Err(
                ResourceError::AccountingInvariantViolation {
                    resource,
                    attempted_release: amount,
                    current,
                },
            );
        }

        let next = current - amount;

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

fn release_usize_checked(
    resource: ResourceKind,
    counter: &AtomicUsize,
    amount: usize,
) -> Result<(), ResourceError> {
    let mut current =
        counter.load(Ordering::Acquire);

    loop {
        if amount > current {
            return Err(
                ResourceError::AccountingInvariantViolation {
                    resource,
                    attempted_release:
                        usize_to_u64(amount, resource)?,
                    current:
                        usize_to_u64(current, resource)?,
                },
            );
        }

        let next = current - amount;

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

fn resource_to_qec_kind(
    resource: LimitKind,
) -> QecResourceKind {
    match resource {
        LimitKind::CodeDistance =>
            QecResourceKind::CodeDistance,

        LimitKind::Qubits =>
            QecResourceKind::Qubits,

        LimitKind::Stabilizers =>
            QecResourceKind::Stabilizers,

        LimitKind::SyndromeEvents =>
            QecResourceKind::SyndromeEvents,

        LimitKind::MeasurementRounds =>
            QecResourceKind::MeasurementRounds,

        LimitKind::GraphNodes =>
            QecResourceKind::GraphNodes,

        LimitKind::GraphEdges =>
            QecResourceKind::GraphEdges,

        LimitKind::MemoryBytes =>
            QecResourceKind::MemoryBytes,

        LimitKind::DecoderTimeNs =>
            QecResourceKind::Time,

        LimitKind::Parallelism =>
            QecResourceKind::Parallelism,

        LimitKind::CheckpointSizeBytes =>
            QecResourceKind::CheckpointSize,

        LimitKind::Partitions =>
            QecResourceKind::Partitions,

        LimitKind::StreamBufferEvents =>
            QecResourceKind::StreamBuffer,

        LimitKind::DecoderIterations =>
            QecResourceKind::DecoderIterations,

        LimitKind::StabilizerWeight =>
            QecResourceKind::StabilizerWeight,

        LimitKind::LogicalOperatorWeight =>
            QecResourceKind::LogicalWeight,

        LimitKind::QubitsPerPartition =>
            QecResourceKind::Custom,

        LimitKind::QpuShots =>
            QecResourceKind::QpuShots,

        LimitKind::QpuCircuits =>
            QecResourceKind::QpuCircuits,

        LimitKind::VerificationOperations =>
            QecResourceKind::Operations,
    }
}

fn counter_for_u64<'a>(
    resource: ResourceKind,
    counters: &'a ResourceCounters,
) -> Result<&'a AtomicU64, ResourceError> {
    match resource {
        ResourceKind::SyndromeEvents =>
            Ok(&counters.syndrome_events),

        ResourceKind::GraphNodes =>
            Ok(&counters.graph_nodes),

        ResourceKind::GraphEdges =>
            Ok(&counters.graph_edges),

        ResourceKind::DecoderIterations =>
            Ok(&counters.decoder_iterations),

        ResourceKind::CheckpointSizeBytes =>
            Ok(&counters.checkpoint_bytes),

        ResourceKind::QpuShots =>
            Ok(&counters.qpu_shots),

        ResourceKind::QpuCircuits =>
            Ok(&counters.qpu_circuits),

        ResourceKind::VerificationOperations =>
            Ok(&counters.verification_operations),

        ResourceKind::MemoryBytes =>
            Ok(&counters.allocated_bytes),

        _ => Err(
            ResourceError::InvalidLimit {
                reason:
                    "resource dimension is not a u64 cumulative counter",
            },
        ),
    }
}

fn counter_for_usize<'a>(
    resource: ResourceKind,
    counters: &'a ResourceCounters,
) -> Result<&'a AtomicUsize, ResourceError> {
    match resource {
        ResourceKind::Qubits =>
            Ok(&counters.qubits),

        ResourceKind::Stabilizers =>
            Ok(&counters.stabilizers),

        ResourceKind::MeasurementRounds =>
            Ok(&counters.measurement_rounds),

        ResourceKind::Partitions =>
            Ok(&counters.partitions),

        ResourceKind::StreamBufferEvents =>
            Ok(&counters.stream_buffer_events),

        _ => Err(
            ResourceError::InvalidLimit {
                reason:
                    "resource dimension is not a usize cumulative counter",
            },
        ),
    }
}

impl ResourceManager {
    fn try_add_usize(
        &self,
        resource: ResourceKind,
        counter: &AtomicUsize,
        requested: usize,
        global_limit: usize,
        quota: Option<usize>,
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
                    requested:
                        usize_to_u64(
                            requested,
                            resource,
                        )?,
                    current:
                        usize_to_u64(
                            current,
                            resource,
                        )?,
                    limit:
                        usize_to_u64(
                            global_limit,
                            resource,
                        )?,
                });
            }

            if let Some(quota_limit) = quota {
                if next > quota_limit {
                    return Err(
                        ResourceError::QuotaExceeded {
                            resource,
                            requested:
                                usize_to_u64(
                                    requested,
                                    resource,
                                )?,
                            current:
                                usize_to_u64(
                                    current,
                                    resource,
                                )?,
                            limit:
                                usize_to_u64(
                                    quota_limit,
                                    resource,
                                )?,
                        },
                    );
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
}

/* ========================================================================== */
/* Unit tests                                                                  */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        let mut limits = QecLimits::default();

        limits.max_memory_bytes = 1_024;
        limits.max_parallelism = 2;
        limits.max_qpu_shots = 10;
        limits.max_verification_operations = 10;

        limits
    }

    #[test]
    fn canonical_limits_create_manager() {
        let manager =
            ResourceManager::new(limits())
                .expect("limits must be valid");

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn memory_reservation_is_raii() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        {
            let reservation =
                manager.reserve_memory(512)
                    .unwrap();

            assert_eq!(
                reservation.bytes(),
                512
            );

            assert_eq!(
                manager.snapshot().allocated_bytes,
                512
            );
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
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        manager
            .reserve_memory(1_024)
            .unwrap();

        let result =
            manager.reserve_memory(1);

        assert!(matches!(
            result,
            Err(ResourceError::LimitExceeded {
                resource:
                    ResourceKind::MemoryBytes,
                ..
            })
        ));
    }

    #[test]
    fn worker_reservation_is_raii() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        {
            let reservation =
                manager.acquire_workers(2)
                    .unwrap();

            assert_eq!(
                reservation.workers(),
                2
            );

            assert_eq!(
                manager.snapshot().parallel_workers,
                2
            );
        }

        assert_eq!(
            manager.snapshot().parallel_workers,
            0
        );
    }

    #[test]
    fn cancellation_is_delegated_to_canonical_token() {
        let token =
            CancellationToken::new();

        let manager =
            ResourceManager::with_cancellation(
                limits(),
                token.clone(),
            )
            .unwrap();

        token.request();

        assert!(matches!(
            manager.check(),
            Err(ResourceError::Cancelled)
        ));
    }

    #[test]
    fn operation_quota_cannot_expand_global_limit() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        let quota = ResourceQuota {
            max_memory_bytes: Some(10_000),
            ..ResourceQuota::default()
        };

        let scope =
            manager.scope("test", quota)
                .unwrap();

        assert!(
            scope.reserve_memory(1_025).is_err()
        );
    }

    #[test]
    fn operation_quota_can_tighten_global_limit() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        let quota = ResourceQuota {
            max_memory_bytes: Some(100),
            ..ResourceQuota::default()
        };

        let scope =
            manager.scope("test", quota)
                .unwrap();

        scope.reserve_memory(100)
            .unwrap();

        assert!(
            scope.reserve_memory(1).is_err()
        );
    }

    #[test]
    fn preflight_performs_no_allocation() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        let request = ResourceRequest {
            qubits: 10,
            memory_bytes: 512,
            ..ResourceRequest::default()
        };

        manager
            .preflight(&request)
            .unwrap();

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn qpu_limits_are_enforced() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        manager
            .record_qpu_shots(10)
            .unwrap();

        assert!(
            manager.record_qpu_shots(1).is_err()
        );
    }

    #[test]
    fn verification_limits_are_enforced() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

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

        assert_eq!(
            request.code_distance,
            5
        );

        assert_eq!(
            request.qubits,
            25
        );

        assert_eq!(
            request.stabilizer_weight,
            4
        );
    }

    #[test]
    fn graph_request_uses_checked_arithmetic() {
        let request =
            ResourceRequest::graph(10, 20)
                .unwrap();

        assert_eq!(
            request.graph_nodes,
            10
        );

        assert_eq!(
            request.graph_edges,
            20
        );

        assert!(
            request.memory_bytes > 0
        );
    }

    #[test]
    fn release_cannot_underflow_accounting() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        let result =
            manager.release_memory(1);

        assert!(matches!(
            result,
            Err(
                ResourceError::
                    AccountingInvariantViolation {
                        resource:
                            ResourceKind::MemoryBytes,
                        ..
                    }
            )
        ));
    }

    #[test]
    fn operation_quota_preflight_checks_all_dimensions() {
        let manager =
            ResourceManager::new(limits())
                .unwrap();

        let quota = ResourceQuota {
            max_qubits: Some(4),
            max_graph_nodes: Some(4),
            max_qpu_shots: Some(4),
            ..ResourceQuota::default()
        };

        let scope =
            manager.scope("test", quota)
                .unwrap();

        let request = ResourceRequest {
            qubits: 5,
            ..ResourceRequest::default()
        };

        assert!(matches!(
            scope.preflight(&request),
            Err(ResourceError::QuotaExceeded {
                resource:
                    ResourceKind::Qubits,
                ..
            })
        ));
    }
}