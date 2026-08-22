//! Zamani Quantum Error Correction — Resource Estimation.
//!
//! This module provides deterministic, overflow-safe, allocation-free
//! estimation of resources required by QEC workloads before execution.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - resource requirement estimation;
//! - deterministic workload sizing;
//! - surface-code resource estimates;
//! - decoder workload estimates;
//! - graph-size estimates;
//! - streaming estimates;
//! - partition estimates;
//! - distributed-worker estimates;
//! - QPU shot/circuit estimates;
//! - mathematical-verification estimates;
//! - memory estimates;
//! - conservative admission/preflight against [`QecLimits`];
//! - conversion of resource-policy failures into [`QecError`];
//! - estimate schema identity.
//!
//! This module does NOT own:
//!
//! - resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - decoder execution (`decoder.rs`);
//! - decoder results (`decoder_result.rs`);
//! - cancellation state (`cancellation.rs`);
//! - capability authorization (`capabilities.rs`);
//! - QPU execution (`qpu_adapter.rs`);
//! - surface-code topology construction (`surface_code.rs`);
//! - decoding algorithms;
//! - logical-equivalence mathematics.
//!
//! # Architectural separation
//!
//! ```text
//!                         Workload
//!                            │
//!                            ▼
//!                    resource_estimator.rs
//!                            │
//!                            ▼
//!                    ResourceEstimate
//!                            │
//!                ┌───────────┴───────────┐
//!                ▼                       ▼
//!           QecLimits              ResourceManager
//!           admission              runtime accounting
//!                │                       │
//!                ▼                       ▼
//!             allowed                 observed
//!                │
//!                ▼
//!          execution layer
//! ```
//!
//! The distinction is fundamental:
//!
//! ```text
//! resource_estimator.rs = what will probably be required
//! limits.rs             = what is permitted
//! memory.rs             = what may be allocated
//! resources.rs          = what was actually consumed
//! decoder_result.rs     = immutable usage snapshot
//! ```
//!
//! # No second policy
//!
//! This module MUST NOT introduce production-wide resource ceilings.
//!
//! It may contain:
//!
//! - mathematical estimation constants;
//! - model coefficients;
//! - conservative expansion factors;
//! - workload-specific derived quantities.
//!
//! Those values describe an estimate, not authorization.
//!
//! Authorization is always performed by [`QecLimits`].
//!
//! # Determinism
//!
//! For identical inputs, this module produces identical estimates.
//!
//! It does not:
//!
//! - inspect wall-clock time;
//! - inspect process state;
//! - allocate memory for estimation;
//! - use randomness;
//! - inspect runtime resource counters;
//! - perform I/O.
//!
//! # Arithmetic safety
//!
//! Every derived quantity is calculated with checked arithmetic.
//!
//! No estimate may silently:
//!
//! - wrap;
//! - saturate;
//! - truncate;
//! - convert an overflow into an apparently valid estimate.
//!
//! # Admission semantics
//!
//! An estimate is intentionally conservative.
//!
//! A workload is admissible only when all relevant estimated resources fit
//! within the supplied [`QecLimits`].
//!
//! Passing estimation does NOT guarantee execution success because runtime
//! consumption may depend on actual input, implementation details, hardware,
//! scheduling, and dynamic execution.
//!
//! Conversely, failing estimation is a deterministic reason to reject a
//! workload before expensive allocation or execution.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are used.
//!
//! `unsafe` is forbidden.
//!
//! # Integration
//!
//! `surface_code.rs` should call the surface-code estimation functions before
//! allocation.
//!
//! `decoding_graph.rs` should use graph estimates before graph allocation.
//!
//! `decoder.rs` should use decoder-work estimates before expensive execution.
//!
//! `streaming.rs` should use streaming estimates before creating buffers.
//!
//! `partition.rs` should use partition estimates before admission.
//!
//! `distributed.rs` should use distributed estimates before worker admission.
//!
//! `qpu_adapter.rs` should use QPU estimates before submission.
//!
//! `verification.rs` should use verification estimates before exact work.
//!
//! `scheduler.rs` may use the aggregate estimate for admission.
//!
//! `resources.rs` remains the source of actual runtime usage.
//!
//! `decoder_result.rs` records runtime usage; it does not call this module to
//! discover what actually happened.
//!
//! # Completion contract
//!
//! Once this file is complete, later QEC modules must not modify it merely to
//! add their own resource policy. They should use the generic estimation
//! primitives and domain-specific methods already provided here.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::{LimitError, LimitKind, QecLimits};

/// Stable schema version for resource estimates.
///
/// Increment this when the semantic meaning of the public estimate changes.
pub const RESOURCE_ESTIMATE_SCHEMA_VERSION: u32 = 1;

/// Number of bytes used as the conservative per-qubit logical-state estimate.
///
/// This is an estimation coefficient, not a memory-policy limit.
pub const DEFAULT_BYTES_PER_QUBIT: u64 = 16;

/// Conservative per-stabilizer metadata estimate in bytes.
pub const DEFAULT_BYTES_PER_STABILIZER: u64 = 32;

/// Conservative per-syndrome-event metadata estimate in bytes.
pub const DEFAULT_BYTES_PER_SYNDROME_EVENT: u64 = 32;

/// Conservative per-graph-node metadata estimate in bytes.
pub const DEFAULT_BYTES_PER_GRAPH_NODE: u64 = 48;

/// Conservative per-graph-edge metadata estimate in bytes.
pub const DEFAULT_BYTES_PER_GRAPH_EDGE: u64 = 64;

/// Conservative per-worker coordination estimate in bytes.
pub const DEFAULT_BYTES_PER_WORKER: u64 = 4096;

/// Conservative decoder workspace coefficient per event.
pub const DEFAULT_BYTES_PER_DECODER_EVENT: u64 = 64;

/// Conservative streaming bookkeeping coefficient per event.
pub const DEFAULT_BYTES_PER_STREAM_EVENT: u64 = 32;

/// Conservative partition bookkeeping coefficient.
pub const DEFAULT_BYTES_PER_PARTITION: u64 = 1024;

/// Conservative distributed-job bookkeeping coefficient.
pub const DEFAULT_BYTES_PER_DISTRIBUTED_JOB: u64 = 4096;

/// Conservative QPU measurement bookkeeping coefficient.
pub const DEFAULT_BYTES_PER_QPU_SHOT: u64 = 16;

/// Conservative verification bookkeeping coefficient.
pub const DEFAULT_BYTES_PER_VERIFICATION_OPERATION: u64 = 16;

/// Additional graph-node multiplier used for temporal decoding graphs.
///
/// This describes an estimate model, not a production resource limit.
pub const DEFAULT_GRAPH_NODES_PER_DETECTION_EVENT: u64 = 1;

/// Conservative graph-edge multiplier per graph node.
///
/// Actual decoders may construct fewer or more edges; this value is therefore
/// intentionally an admission estimate rather than a correctness assertion.
pub const DEFAULT_GRAPH_EDGES_PER_NODE: u64 = 4;

/// Conservative decoder-iteration estimate per syndrome event.
pub const DEFAULT_ITERATIONS_PER_SYNDROME_EVENT: u64 = 1;

/// Conservative distributed jobs per partition.
pub const DEFAULT_JOBS_PER_PARTITION: u64 = 1;

/// Conservative QPU circuits per requested circuit.
pub const DEFAULT_CIRCUIT_MULTIPLIER: u64 = 1;

/// Conservative verification operation multiplier.
///
/// Verification algorithms may use more sophisticated exact operation counts.
pub const DEFAULT_VERIFICATION_OPERATION_MULTIPLIER: u64 = 1;

/* ========================================================================== */
/* Estimation errors                                                          */
/* ========================================================================== */

/// Error returned while calculating a resource estimate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstimateError {
    /// A derived quantity overflowed.
    ArithmeticOverflow {
        resource: LimitKind,
        operation: &'static str,
    },

    /// An estimate cannot be represented by the public estimate type.
    RepresentationOverflow {
        field: &'static str,
    },

    /// A workload parameter is mathematically invalid.
    InvalidParameter {
        parameter: &'static str,
        value: u128,
        message: String,
    },

    /// The requested workload violates canonical QEC policy.
    LimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },
}

impl fmt::Display for EstimateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow {
                resource,
                operation,
            } => write!(
                formatter,
                "arithmetic overflow while estimating {resource} \
                 during {operation}"
            ),

            Self::RepresentationOverflow { field } => write!(
                formatter,
                "resource estimate field {field} cannot represent \
                 the calculated value"
            ),

            Self::InvalidParameter {
                parameter,
                value,
                message,
            } => write!(
                formatter,
                "invalid resource-estimation parameter {parameter}={value}: \
                 {message}"
            ),

            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                formatter,
                "estimated {resource} exceeds configured limit: \
                 requested {requested}, maximum {maximum}"
            ),
        }
    }
}

impl std::error::Error for EstimateError {}

impl From<EstimateError> for QecError {
    fn from(error: EstimateError) -> Self {
        match error {
            EstimateError::ArithmeticOverflow {
                resource,
                operation,
            } => QecError::NumericalFailure {
                operation: super::errors::NumericalOperation::Overflow,
                message: format!(
                    "resource estimation overflow for {resource} \
                     during {operation}"
                ),
            },

            EstimateError::RepresentationOverflow { field } => {
                QecError::NumericalFailure {
                    operation:
                        super::errors::NumericalOperation::Overflow,
                    message: format!(
                        "resource-estimate representation overflow \
                         for {field}"
                    ),
                }
            }

            EstimateError::InvalidParameter {
                parameter,
                value,
                message,
            } => QecError::InvalidInput {
                message: format!(
                    "invalid resource-estimation parameter \
                     {parameter}={value}: {message}"
                ),
            },

            EstimateError::LimitExceeded {
                resource,
                requested,
                maximum,
            } => QecError::ResourceLimitExceeded {
                resource: resource_kind_from_limit(resource),
                requested,
                current: 0,
                limit: maximum,
                message: format!(
                    "resource estimate exceeds {resource} policy"
                ),
            },
        }
    }
}

impl From<LimitError> for EstimateError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::Exceeded {
                resource,
                requested,
                maximum,
            } => Self::LimitExceeded {
                resource,
                requested,
                maximum,
            },

            LimitError::ArithmeticOverflow { resource } => {
                Self::ArithmeticOverflow {
                    resource,
                    operation: "canonical limit calculation",
                }
            }

            LimitError::InvalidLimit { resource, value } => {
                Self::InvalidParameter {
                    parameter: resource.as_str(),
                    value,
                    message: "configured limit must be greater than zero"
                        .to_owned(),
                }
            }

            LimitError::InconsistentLimits {
                resource,
                related_resource,
                reason,
            } => Self::InvalidParameter {
                parameter: resource.as_str(),
                value: 0,
                message: format!(
                    "inconsistent with {related_resource}: {reason}"
                ),
            },

            LimitError::UnsupportedSchema { found, expected } => {
                Self::InvalidParameter {
                    parameter: "limits_schema_version",
                    value: found as u128,
                    message: format!(
                        "unsupported schema; expected {expected}"
                    ),
                }
            }
        }
    }
}

fn resource_kind_from_limit(resource: LimitKind) -> ResourceKind {
    match resource {
        LimitKind::CodeDistance => ResourceKind::CodeDistance,
        LimitKind::Qubits => ResourceKind::Qubits,
        LimitKind::Stabilizers => ResourceKind::Stabilizers,
        LimitKind::SyndromeEvents => ResourceKind::SyndromeEvents,
        LimitKind::MeasurementRounds => {
            ResourceKind::MeasurementRounds
        }
        LimitKind::GraphNodes => ResourceKind::GraphNodes,
        LimitKind::GraphEdges => ResourceKind::GraphEdges,
        LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
        LimitKind::DecoderTimeNs => ResourceKind::DecoderTimeNs,
        LimitKind::Parallelism => ResourceKind::Parallelism,
        LimitKind::CheckpointSizeBytes => {
            ResourceKind::CheckpointSizeBytes
        }
        LimitKind::Partitions => ResourceKind::Partitions,
        LimitKind::StreamBufferEvents => {
            ResourceKind::StreamBufferEvents
        }
        LimitKind::DecoderIterations => {
            ResourceKind::DecoderIterations
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
        LimitKind::QpuCircuits => ResourceKind::QpuCircuits,
        LimitKind::VerificationOperations => {
            ResourceKind::VerificationOperations
        }
    }
}

/* ========================================================================== */
/* Safe arithmetic                                                            */
/* ========================================================================== */

fn checked_add(
    resource: LimitKind,
    left: u64,
    right: u64,
) -> Result<u64, EstimateError> {
    left.checked_add(right).ok_or(
        EstimateError::ArithmeticOverflow {
            resource,
            operation: "addition",
        },
    )
}

fn checked_mul(
    resource: LimitKind,
    left: u64,
    right: u64,
) -> Result<u64, EstimateError> {
    left.checked_mul(right).ok_or(
        EstimateError::ArithmeticOverflow {
            resource,
            operation: "multiplication",
        },
    )
}

fn checked_square(
    resource: LimitKind,
    value: u64,
) -> Result<u64, EstimateError> {
    checked_mul(resource, value, value)
}

fn checked_usize(
    field: &'static str,
    value: u64,
) -> Result<usize, EstimateError> {
    usize::try_from(value).map_err(|_| {
        EstimateError::RepresentationOverflow { field }
    })
}

fn checked_u64(
    field: &'static str,
    value: usize,
) -> Result<u64, EstimateError> {
    u64::try_from(value).map_err(|_| {
        EstimateError::RepresentationOverflow { field }
    })
}

fn require_positive(
    parameter: &'static str,
    value: u128,
) -> Result<(), EstimateError> {
    if value == 0 {
        return Err(EstimateError::InvalidParameter {
            parameter,
            value,
            message: "value must be greater than zero".to_owned(),
        });
    }

    Ok(())
}

/* ========================================================================== */
/* Workload dimensions                                                        */
/* ========================================================================== */

/// Primary workload dimensions supplied to the estimator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkloadDimensions {
    /// Code distance.
    pub distance: usize,

    /// Number of physical qubits.
    pub qubits: usize,

    /// Number of stabilizer generators.
    pub stabilizers: usize,

    /// Number of measurement rounds.
    pub rounds: usize,

    /// Number of syndrome/detection events.
    pub syndrome_events: usize,

    /// Requested decoder iterations.
    ///
    /// `None` means the estimator should derive a conservative value.
    pub decoder_iterations: Option<usize>,

    /// Number of parallel workers.
    pub workers: usize,

    /// Number of stream-buffer events.
    pub stream_buffer_events: usize,

    /// Number of partitions.
    pub partitions: usize,

    /// QPU shots.
    pub qpu_shots: u64,

    /// QPU circuits.
    pub qpu_circuits: u64,

    /// Mathematical-verification operations.
    pub verification_operations: u64,
}

impl WorkloadDimensions {
    /// Creates an empty workload with explicit code dimensions.
    pub fn new(
        distance: usize,
        qubits: usize,
        stabilizers: usize,
    ) -> Result<Self, EstimateError> {
        require_positive("distance", distance as u128)?;
        require_positive("qubits", qubits as u128)?;
        require_positive("stabilizers", stabilizers as u128)?;

        Ok(Self {
            distance,
            qubits,
            stabilizers,
            rounds: 1,
            syndrome_events: 0,
            decoder_iterations: None,
            workers: 1,
            stream_buffer_events: 1,
            partitions: 1,
            qpu_shots: 0,
            qpu_circuits: 0,
            verification_operations: 0,
        })
    }

    /// Sets measurement rounds.
    #[must_use]
    pub const fn with_rounds(
        mut self,
        rounds: usize,
    ) -> Self {
        self.rounds = rounds;
        self
    }

    /// Sets syndrome-event count.
    #[must_use]
    pub const fn with_syndrome_events(
        mut self,
        events: usize,
    ) -> Self {
        self.syndrome_events = events;
        self
    }

    /// Sets an explicit decoder-iteration budget.
    #[must_use]
    pub const fn with_decoder_iterations(
        mut self,
        iterations: usize,
    ) -> Self {
        self.decoder_iterations = Some(iterations);
        self
    }

    /// Sets worker count.
    #[must_use]
    pub const fn with_workers(
        mut self,
        workers: usize,
    ) -> Self {
        self.workers = workers;
        self
    }

    /// Sets stream-buffer capacity.
    #[must_use]
    pub const fn with_stream_buffer_events(
        mut self,
        events: usize,
    ) -> Self {
        self.stream_buffer_events = events;
        self
    }

    /// Sets partition count.
    #[must_use]
    pub const fn with_partitions(
        mut self,
        partitions: usize,
    ) -> Self {
        self.partitions = partitions;
        self
    }

    /// Sets QPU shot/circuit counts.
    #[must_use]
    pub const fn with_qpu(
        mut self,
        shots: u64,
        circuits: u64,
    ) -> Self {
        self.qpu_shots = shots;
        self.qpu_circuits = circuits;
        self
    }

    /// Sets verification workload.
    #[must_use]
    pub const fn with_verification_operations(
        mut self,
        operations: u64,
    ) -> Self {
        self.verification_operations = operations;
        self
    }
}

/* ========================================================================== */
/* Estimate model                                                             */
/* ========================================================================== */

/// Complete deterministic resource estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceEstimate {
    /// Estimate schema version.
    pub schema_version: u32,

    /// Physical qubits required.
    pub qubits: u64,

    /// Stabilizer generators required.
    pub stabilizers: u64,

    /// Syndrome events processed.
    pub syndrome_events: u64,

    /// Measurement rounds.
    pub measurement_rounds: u64,

    /// Estimated decoding-graph nodes.
    pub graph_nodes: u64,

    /// Estimated decoding-graph edges.
    pub graph_edges: u64,

    /// Estimated peak memory in bytes.
    pub peak_memory_bytes: u64,

    /// Estimated decoder iterations.
    pub decoder_iterations: u64,

    /// Estimated worker count.
    pub workers: u64,

    /// Estimated stream-buffer capacity.
    pub stream_buffer_events: u64,

    /// Estimated partition count.
    pub partitions: u64,

    /// Estimated QPU shots.
    pub qpu_shots: u64,

    /// Estimated QPU circuits.
    pub qpu_circuits: u64,

    /// Estimated mathematical-verification operations.
    pub verification_operations: u64,
}

impl ResourceEstimate {
    /// Creates an all-zero estimate.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            schema_version: RESOURCE_ESTIMATE_SCHEMA_VERSION,
            qubits: 0,
            stabilizers: 0,
            syndrome_events: 0,
            measurement_rounds: 0,
            graph_nodes: 0,
            graph_edges: 0,
            peak_memory_bytes: 0,
            decoder_iterations: 0,
            workers: 0,
            stream_buffer_events: 0,
            partitions: 0,
            qpu_shots: 0,
            qpu_circuits: 0,
            verification_operations: 0,
        }
    }

    /// Returns the largest estimated resource dimension.
    ///
    /// This is informational only; admission must check every dimension.
    #[must_use]
    pub fn dominant_resource(&self) -> Option<LimitKind> {
        let dimensions = [
            (
                LimitKind::Qubits,
                self.qubits as u128,
            ),
            (
                LimitKind::Stabilizers,
                self.stabilizers as u128,
            ),
            (
                LimitKind::SyndromeEvents,
                self.syndrome_events as u128,
            ),
            (
                LimitKind::GraphNodes,
                self.graph_nodes as u128,
            ),
            (
                LimitKind::GraphEdges,
                self.graph_edges as u128,
            ),
            (
                LimitKind::MemoryBytes,
                self.peak_memory_bytes as u128,
            ),
            (
                LimitKind::DecoderIterations,
                self.decoder_iterations as u128,
            ),
            (
                LimitKind::Parallelism,
                self.workers as u128,
            ),
            (
                LimitKind::StreamBufferEvents,
                self.stream_buffer_events as u128,
            ),
            (
                LimitKind::Partitions,
                self.partitions as u128,
            ),
            (
                LimitKind::QpuShots,
                self.qpu_shots as u128,
            ),
            (
                LimitKind::QpuCircuits,
                self.qpu_circuits as u128,
            ),
            (
                LimitKind::VerificationOperations,
                self.verification_operations as u128,
            ),
        ];

        dimensions
            .iter()
            .copied()
            .max_by_key(|(_, value)| *value)
            .and_then(|(resource, value)| {
                if value == 0 {
                    None
                } else {
                    Some(resource)
                }
            })
    }

    /// Returns whether this estimate contains no requested workload.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.qubits == 0
            && self.stabilizers == 0
            && self.syndrome_events == 0
            && self.measurement_rounds == 0
            && self.graph_nodes == 0
            && self.graph_edges == 0
            && self.peak_memory_bytes == 0
            && self.decoder_iterations == 0
            && self.workers == 0
            && self.stream_buffer_events == 0
            && self.partitions == 0
            && self.qpu_shots == 0
            && self.qpu_circuits == 0
            && self.verification_operations == 0
    }

    /// Returns a conservative aggregate memory estimate.
    ///
    /// This is the same value stored in `peak_memory_bytes`.
    #[must_use]
    pub const fn memory_bytes(&self) -> u64 {
        self.peak_memory_bytes
    }

    /// Performs complete admission against canonical QEC limits.
    ///
    /// Every relevant dimension is checked independently.
    pub fn admit(
        &self,
        limits: &QecLimits,
    ) -> Result<(), EstimateError> {
        limits.validate()?;

        limits.validate_code_size(
            checked_usize("qubits", self.qubits)?,
            checked_usize("qubits", self.qubits)?,
            checked_usize("stabilizers", self.stabilizers)?,
        )?;

        limits.validate_syndrome(
            checked_usize(
                "syndrome_events",
                self.syndrome_events,
            )?,
            checked_usize(
                "measurement_rounds",
                self.measurement_rounds,
            )?,
        )?;

        limits.validate_graph(
            checked_usize("graph_nodes", self.graph_nodes)?,
            checked_usize("graph_edges", self.graph_edges)?,
        )?;

        limits.validate_memory(self.peak_memory_bytes)?;

        limits.validate_decoder_work(
            checked_usize(
                "decoder_iterations",
                self.decoder_iterations,
            )?,
            0,
        )?;

        limits.validate_parallelism(
            checked_usize("workers", self.workers)?,
        )?;

        limits.validate_stream(
            checked_usize(
                "stream_buffer_events",
                self.stream_buffer_events,
            )?,
        )?;

        limits.validate_partition(
            checked_usize("partitions", self.partitions)?,
            checked_usize("qubits", self.qubits)?,
        )?;

        limits.validate_qpu(
            self.qpu_shots,
            self.qpu_circuits,
        )?;

        limits.validate_verification(
            self.verification_operations,
        )?;

        Ok(())
    }

    /// Performs admission and converts the result to canonical `QecError`.
    pub fn require_admitted(
        &self,
        limits: &QecLimits,
    ) -> QecResult<()> {
        self.admit(limits).map_err(QecError::from)
    }
}

/* ========================================================================== */
/* Estimation configuration                                                   */
/* ========================================================================== */

/// Estimation coefficients.
///
/// These coefficients describe how the estimator models implementation
/// overhead. They are not policy limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceEstimateModel {
    /// Bytes per physical qubit.
    pub bytes_per_qubit: u64,

    /// Bytes per stabilizer.
    pub bytes_per_stabilizer: u64,

    /// Bytes per syndrome event.
    pub bytes_per_syndrome_event: u64,

    /// Bytes per graph node.
    pub bytes_per_graph_node: u64,

    /// Bytes per graph edge.
    pub bytes_per_graph_edge: u64,

    /// Bytes per worker.
    pub bytes_per_worker: u64,

    /// Bytes of decoder workspace per syndrome event.
    pub bytes_per_decoder_event: u64,

    /// Bytes of stream bookkeeping per buffered event.
    pub bytes_per_stream_event: u64,

    /// Bytes of partition bookkeeping.
    pub bytes_per_partition: u64,

    /// Bytes of distributed-job bookkeeping.
    pub bytes_per_distributed_job: u64,

    /// Bytes of QPU bookkeeping per shot.
    pub bytes_per_qpu_shot: u64,

    /// Bytes of verification bookkeeping per operation.
    pub bytes_per_verification_operation: u64,

    /// Graph nodes generated per detection event.
    pub graph_nodes_per_event: u64,

    /// Graph edges generated per graph node.
    pub graph_edges_per_node: u64,

    /// Decoder iterations estimated per syndrome event.
    pub iterations_per_syndrome_event: u64,

    /// Distributed jobs estimated per partition.
    pub jobs_per_partition: u64,
}

impl Default for ResourceEstimateModel {
    fn default() -> Self {
        Self {
            bytes_per_qubit: DEFAULT_BYTES_PER_QUBIT,
            bytes_per_stabilizer: DEFAULT_BYTES_PER_STABILIZER,
            bytes_per_syndrome_event:
                DEFAULT_BYTES_PER_SYNDROME_EVENT,
            bytes_per_graph_node:
                DEFAULT_BYTES_PER_GRAPH_NODE,
            bytes_per_graph_edge:
                DEFAULT_BYTES_PER_GRAPH_EDGE,
            bytes_per_worker: DEFAULT_BYTES_PER_WORKER,
            bytes_per_decoder_event:
                DEFAULT_BYTES_PER_DECODER_EVENT,
            bytes_per_stream_event:
                DEFAULT_BYTES_PER_STREAM_EVENT,
            bytes_per_partition:
                DEFAULT_BYTES_PER_PARTITION,
            bytes_per_distributed_job:
                DEFAULT_BYTES_PER_DISTRIBUTED_JOB,
            bytes_per_qpu_shot:
                DEFAULT_BYTES_PER_QPU_SHOT,
            bytes_per_verification_operation:
                DEFAULT_BYTES_PER_VERIFICATION_OPERATION,
            graph_nodes_per_event:
                DEFAULT_GRAPH_NODES_PER_DETECTION_EVENT,
            graph_edges_per_node:
                DEFAULT_GRAPH_EDGES_PER_NODE,
            iterations_per_syndrome_event:
                DEFAULT_ITERATIONS_PER_SYNDROME_EVENT,
            jobs_per_partition:
                DEFAULT_JOBS_PER_PARTITION,
        }
    }
}

/* ========================================================================== */
/* Surface-code geometry                                                     */
/* ========================================================================== */

/// Surface-code geometry model understood by the estimator.
///
/// The estimator deliberately distinguishes geometry from the actual
/// implementation in `surface_code.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceCodeModel {
    /// Rotated planar surface-code estimate.
    ///
    /// The estimator uses:
    ///
    /// - data qubits = d²;
    /// - stabilizers = d² - 1.
    ///
    /// These are estimation assumptions, not topology construction logic.
    RotatedPlanar,

    /// Caller supplies explicit qubit/stabilizer counts.
    Custom {
        qubits: usize,
        stabilizers: usize,
    },
}

impl SurfaceCodeModel {
    /// Estimates physical qubits and stabilizers.
    pub fn estimate(
        self,
        distance: usize,
    ) -> Result<(usize, usize), EstimateError> {
        require_positive("distance", distance as u128)?;

        match self {
            Self::RotatedPlanar => {
                let d = checked_u64(
                    "distance",
                    distance,
                )?;

                let qubits = checked_square(
                    LimitKind::Qubits,
                    d,
                )?;

                let stabilizers = qubits.checked_sub(1).ok_or(
                    EstimateError::ArithmeticOverflow {
                        resource: LimitKind::Stabilizers,
                        operation: "rotated-planar stabilizer count",
                    },
                )?;

                Ok((
                    checked_usize("qubits", qubits)?,
                    checked_usize(
                        "stabilizers",
                        stabilizers,
                    )?,
                ))
            }

            Self::Custom {
                qubits,
                stabilizers,
            } => {
                require_positive(
                    "custom.qubits",
                    qubits as u128,
                )?;

                require_positive(
                    "custom.stabilizers",
                    stabilizers as u128,
                )?;

                Ok((qubits, stabilizers))
            }
        }
    }
}

/* ========================================================================== */
/* Estimator                                                                  */
/* ========================================================================== */

/// Deterministic resource estimator.
///
/// The estimator is stateless and therefore safe to share between execution
/// layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceEstimator {
    model: ResourceEstimateModel,
}

impl Default for ResourceEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceEstimator {
    /// Creates an estimator using canonical conservative coefficients.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            model: ResourceEstimateModel {
                bytes_per_qubit: DEFAULT_BYTES_PER_QUBIT,
                bytes_per_stabilizer:
                    DEFAULT_BYTES_PER_STABILIZER,
                bytes_per_syndrome_event:
                    DEFAULT_BYTES_PER_SYNDROME_EVENT,
                bytes_per_graph_node:
                    DEFAULT_BYTES_PER_GRAPH_NODE,
                bytes_per_graph_edge:
                    DEFAULT_BYTES_PER_GRAPH_EDGE,
                bytes_per_worker:
                    DEFAULT_BYTES_PER_WORKER,
                bytes_per_decoder_event:
                    DEFAULT_BYTES_PER_DECODER_EVENT,
                bytes_per_stream_event:
                    DEFAULT_BYTES_PER_STREAM_EVENT,
                bytes_per_partition:
                    DEFAULT_BYTES_PER_PARTITION,
                bytes_per_distributed_job:
                    DEFAULT_BYTES_PER_DISTRIBUTED_JOB,
                bytes_per_qpu_shot:
                    DEFAULT_BYTES_PER_QPU_SHOT,
                bytes_per_verification_operation:
                    DEFAULT_BYTES_PER_VERIFICATION_OPERATION,
                graph_nodes_per_event:
                    DEFAULT_GRAPH_NODES_PER_DETECTION_EVENT,
                graph_edges_per_node:
                    DEFAULT_GRAPH_EDGES_PER_NODE,
                iterations_per_syndrome_event:
                    DEFAULT_ITERATIONS_PER_SYNDROME_EVENT,
                jobs_per_partition:
                    DEFAULT_JOBS_PER_PARTITION,
            },
        }
    }

    /// Creates an estimator with explicit coefficients.
    #[must_use]
    pub const fn with_model(
        model: ResourceEstimateModel,
    ) -> Self {
        Self { model }
    }

    /// Returns the estimator's coefficient model.
    #[must_use]
    pub const fn model(&self) -> &ResourceEstimateModel {
        &self.model
    }

    /// Estimates a complete workload.
    pub fn estimate(
        &self,
        workload: WorkloadDimensions,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "distance",
            workload.distance as u128,
        )?;

        require_positive(
            "qubits",
            workload.qubits as u128,
        )?;

        require_positive(
            "stabilizers",
            workload.stabilizers as u128,
        )?;

        require_positive(
            "rounds",
            workload.rounds as u128,
        )?;

        require_positive(
            "workers",
            workload.workers as u128,
        )?;

        require_positive(
            "stream_buffer_events",
            workload.stream_buffer_events as u128,
        )?;

        require_positive(
            "partitions",
            workload.partitions as u128,
        )?;

        let qubits = checked_u64(
            "qubits",
            workload.qubits,
        )?;

        let stabilizers = checked_u64(
            "stabilizers",
            workload.stabilizers,
        )?;

        let rounds = checked_u64(
            "measurement_rounds",
            workload.rounds,
        )?;

        let events = checked_u64(
            "syndrome_events",
            workload.syndrome_events,
        )?;

        let workers = checked_u64(
            "workers",
            workload.workers,
        )?;

        let stream_buffer = checked_u64(
            "stream_buffer_events",
            workload.stream_buffer_events,
        )?;

        let partitions = checked_u64(
            "partitions",
            workload.partitions,
        )?;

        let graph_nodes = checked_mul(
            LimitKind::GraphNodes,
            events,
            self.model.graph_nodes_per_event,
        )?;

        let graph_edges = checked_mul(
            LimitKind::GraphEdges,
            graph_nodes,
            self.model.graph_edges_per_node,
        )?;

        let derived_iterations = checked_mul(
            LimitKind::DecoderIterations,
            events,
            self.model.iterations_per_syndrome_event,
        )?;

        let decoder_iterations = match workload.decoder_iterations {
            Some(iterations) => {
                checked_u64(
                    "decoder_iterations",
                    iterations,
                )?
            }
            None => derived_iterations,
        };

        let memory = self.estimate_memory_bytes(
            qubits,
            stabilizers,
            events,
            graph_nodes,
            graph_edges,
            workers,
            stream_buffer,
            partitions,
            workload.qpu_shots,
            workload.verification_operations,
        )?;

        Ok(ResourceEstimate {
            schema_version:
                RESOURCE_ESTIMATE_SCHEMA_VERSION,
            qubits,
            stabilizers,
            syndrome_events: events,
            measurement_rounds: rounds,
            graph_nodes,
            graph_edges,
            peak_memory_bytes: memory,
            decoder_iterations,
            workers,
            stream_buffer_events: stream_buffer,
            partitions,
            qpu_shots: workload.qpu_shots,
            qpu_circuits: workload.qpu_circuits,
            verification_operations:
                workload.verification_operations,
        })
    }

    /// Estimates a rotated-planar surface-code workload.
    pub fn estimate_surface_code(
        &self,
        model: SurfaceCodeModel,
        distance: usize,
        rounds: usize,
        syndrome_events: usize,
    ) -> Result<ResourceEstimate, EstimateError> {
        let (qubits, stabilizers) =
            model.estimate(distance)?;

        WorkloadDimensions::new(
            distance,
            qubits,
            stabilizers,
        )?
        .with_rounds(rounds)
        .with_syndrome_events(syndrome_events)
        .with_decoder_iterations(
            self.estimate_decoder_iterations(
                syndrome_events,
            )?,
        )
        .pipe(|workload| self.estimate(workload))
    }

    /// Estimates decoder-only workload.
    ///
    /// This is the preferred entry point for `decoder.rs` when the code
    /// topology is already known elsewhere.
    pub fn estimate_decoder(
        &self,
        syndrome_events: usize,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "syndrome_events",
            syndrome_events as u128,
        )?;

        let events = checked_u64(
            "syndrome_events",
            syndrome_events,
        )?;

        let iterations =
            self.estimate_decoder_iterations(
                syndrome_events,
            )?;

        let graph_nodes = checked_mul(
            LimitKind::GraphNodes,
            events,
            self.model.graph_nodes_per_event,
        )?;

        let graph_edges = checked_mul(
            LimitKind::GraphEdges,
            graph_nodes,
            self.model.graph_edges_per_node,
        )?;

        let memory = self.estimate_memory_bytes(
            0,
            0,
            events,
            graph_nodes,
            graph_edges,
            1,
            events,
            1,
            0,
            0,
        )?;

        Ok(ResourceEstimate {
            schema_version:
                RESOURCE_ESTIMATE_SCHEMA_VERSION,
            qubits: 0,
            stabilizers: 0,
            syndrome_events: events,
            measurement_rounds: 1,
            graph_nodes,
            graph_edges,
            peak_memory_bytes: memory,
            decoder_iterations: iterations,
            workers: 1,
            stream_buffer_events: events,
            partitions: 1,
            qpu_shots: 0,
            qpu_circuits: 0,
            verification_operations: 0,
        })
    }

    /// Estimates a bounded streaming workload.
    pub fn estimate_streaming(
        &self,
        syndrome_events: usize,
        buffer_events: usize,
        workers: usize,
    ) -> Result<ResourceEstimate, EstimateError> {
        let mut estimate =
            self.estimate_decoder(syndrome_events)?;

        require_positive(
            "buffer_events",
            buffer_events as u128,
        )?;

        require_positive(
            "workers",
            workers as u128,
        )?;

        estimate.stream_buffer_events =
            checked_u64(
                "stream_buffer_events",
                buffer_events,
            )?;

        estimate.workers =
            checked_u64("workers", workers)?;

        estimate.peak_memory_bytes =
            self.estimate_memory_bytes(
                estimate.qubits,
                estimate.stabilizers,
                estimate.syndrome_events,
                estimate.graph_nodes,
                estimate.graph_edges,
                estimate.workers,
                estimate.stream_buffer_events,
                estimate.partitions,
                estimate.qpu_shots,
                estimate.verification_operations,
            )?;

        Ok(estimate)
    }

    /// Estimates a partitioned workload.
    pub fn estimate_partitioned(
        &self,
        base: WorkloadDimensions,
        partitions: usize,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "partitions",
            partitions as u128,
        )?;

        let estimate = self.estimate(
            base.with_partitions(partitions),
        )?;

        Ok(estimate)
    }

    /// Estimates a distributed workload.
    pub fn estimate_distributed(
        &self,
        base: WorkloadDimensions,
        partitions: usize,
        workers: usize,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "partitions",
            partitions as u128,
        )?;

        require_positive(
            "workers",
            workers as u128,
        )?;

        let mut estimate = self.estimate(
            base
                .with_partitions(partitions)
                .with_workers(workers),
        )?;

        let jobs = checked_mul(
            LimitKind::Partitions,
            checked_u64(
                "partitions",
                partitions,
            )?,
            self.model.jobs_per_partition,
        )?;

        let distributed_memory =
            checked_mul(
                LimitKind::MemoryBytes,
                jobs,
                self.model.bytes_per_distributed_job,
            )?;

        estimate.peak_memory_bytes =
            checked_add(
                LimitKind::MemoryBytes,
                estimate.peak_memory_bytes,
                distributed_memory,
            )?;

        Ok(estimate)
    }

    /// Estimates a QPU workload.
    ///
    /// The estimator does not submit anything to hardware.
    pub fn estimate_qpu(
        &self,
        base: WorkloadDimensions,
        shots: u64,
        circuits: u64,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "shots",
            shots as u128,
        )?;

        require_positive(
            "circuits",
            circuits as u128,
        )?;

        let mut estimate =
            self.estimate(base.with_qpu(shots, circuits))?;

        let qpu_memory =
            checked_mul(
                LimitKind::MemoryBytes,
                shots,
                self.model.bytes_per_qpu_shot,
            )?;

        estimate.peak_memory_bytes =
            checked_add(
                LimitKind::MemoryBytes,
                estimate.peak_memory_bytes,
                qpu_memory,
            )?;

        Ok(estimate)
    }

    /// Estimates mathematical verification workload.
    pub fn estimate_verification(
        &self,
        base: WorkloadDimensions,
        operations: u64,
    ) -> Result<ResourceEstimate, EstimateError> {
        require_positive(
            "verification_operations",
            operations as u128,
        )?;

        let mut estimate = self.estimate(
            base.with_verification_operations(
                operations,
            ),
        )?;

        let verification_memory =
            checked_mul(
                LimitKind::MemoryBytes,
                operations,
                self.model.bytes_per_verification_operation,
            )?;

        estimate.peak_memory_bytes =
            checked_add(
                LimitKind::MemoryBytes,
                estimate.peak_memory_bytes,
                verification_memory,
            )?;

        Ok(estimate)
    }

    /// Estimates decoder iterations using the canonical model.
    pub fn estimate_decoder_iterations(
        &self,
        syndrome_events: usize,
    ) -> Result<u64, EstimateError> {
        require_positive(
            "syndrome_events",
            syndrome_events as u128,
        )?;

        let events = checked_u64(
            "syndrome_events",
            syndrome_events,
        )?;

        checked_mul(
            LimitKind::DecoderIterations,
            events,
            self.model.iterations_per_syndrome_event,
        )
    }

    /// Estimates peak memory without allocating it.
    fn estimate_memory_bytes(
        &self,
        qubits: u64,
        stabilizers: u64,
        syndrome_events: u64,
        graph_nodes: u64,
        graph_edges: u64,
        workers: u64,
        stream_buffer_events: u64,
        partitions: u64,
        qpu_shots: u64,
        verification_operations: u64,
    ) -> Result<u64, EstimateError> {
        let mut total = 0_u64;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                qubits,
                self.model.bytes_per_qubit,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                stabilizers,
                self.model.bytes_per_stabilizer,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                syndrome_events,
                self.model.bytes_per_syndrome_event,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                graph_nodes,
                self.model.bytes_per_graph_node,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                graph_edges,
                self.model.bytes_per_graph_edge,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                workers,
                self.model.bytes_per_worker,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                stream_buffer_events,
                self.model.bytes_per_stream_event,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                partitions,
                self.model.bytes_per_partition,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                qpu_shots,
                self.model.bytes_per_qpu_shot,
            )?,
        )?;

        total = checked_add(
            LimitKind::MemoryBytes,
            total,
            checked_mul(
                LimitKind::MemoryBytes,
                verification_operations,
                self.model.bytes_per_verification_operation,
            )?,
        )?;

        Ok(total)
    }
}

/* ========================================================================== */
/* Small integration helpers                                                  */
/* ========================================================================== */

/// Estimates rotated-planar surface-code resources using the canonical model.
///
/// This is intentionally a convenience function for callers that do not need
/// to construct a `ResourceEstimator` themselves.
pub fn estimate_surface_code(
    distance: usize,
    rounds: usize,
    syndrome_events: usize,
) -> Result<ResourceEstimate, EstimateError> {
    ResourceEstimator::new().estimate_surface_code(
        SurfaceCodeModel::RotatedPlanar,
        distance,
        rounds,
        syndrome_events,
    )
}

/// Estimates decoder resources from syndrome-event count.
pub fn estimate_decoder(
    syndrome_events: usize,
) -> Result<ResourceEstimate, EstimateError> {
    ResourceEstimator::new()
        .estimate_decoder(syndrome_events)
}

/// Estimates a workload and immediately checks canonical limits.
pub fn estimate_and_admit(
    workload: WorkloadDimensions,
    limits: &QecLimits,
) -> QecResult<ResourceEstimate> {
    let estimate =
        ResourceEstimator::new()
            .estimate(workload)
            .map_err(QecError::from)?;

    estimate.require_admitted(limits)?;

    Ok(estimate)
}

/// Estimates a rotated surface-code workload and immediately checks limits.
pub fn estimate_surface_code_and_admit(
    distance: usize,
    rounds: usize,
    syndrome_events: usize,
    limits: &QecLimits,
) -> QecResult<ResourceEstimate> {
    let estimate =
        ResourceEstimator::new()
            .estimate_surface_code(
                SurfaceCodeModel::RotatedPlanar,
                distance,
                rounds,
                syndrome_events,
            )
            .map_err(QecError::from)?;

    estimate.require_admitted(limits)?;

    Ok(estimate)
}

/* ========================================================================== */
/* Internal pipe helper                                                       */
/* ========================================================================== */

trait Pipe: Sized {
    fn pipe<T, F>(self, function: F) -> T
    where
        F: FnOnce(Self) -> T;
}

impl<T> Pipe for T {
    fn pipe<U, F>(self, function: F) -> U
    where
        F: FnOnce(Self) -> U,
    {
        function(self)
    }
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_is_deterministic() {
        let first = ResourceEstimator::new().model();
        let second = ResourceEstimator::new().model();

        assert_eq!(first, second);
    }

    #[test]
    fn rotated_surface_code_uses_checked_geometry() {
        let (qubits, stabilizers) =
            SurfaceCodeModel::RotatedPlanar
                .estimate(3)
                .expect("distance 3 must estimate");

        assert_eq!(qubits, 9);
        assert_eq!(stabilizers, 8);
    }

    #[test]
    fn zero_distance_is_rejected() {
        let result =
            SurfaceCodeModel::RotatedPlanar
                .estimate(0);

        assert!(matches!(
            result,
            Err(EstimateError::InvalidParameter {
                parameter: "distance",
                ..
            })
        ));
    }

    #[test]
    fn decoder_estimate_is_deterministic() {
        let estimator = ResourceEstimator::new();

        let first = estimator
            .estimate_decoder(100)
            .expect("estimate must succeed");

        let second = estimator
            .estimate_decoder(100)
            .expect("estimate must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn graph_estimate_is_derived_from_events() {
        let estimate = ResourceEstimator::new()
            .estimate_decoder(10)
            .expect("estimate must succeed");

        assert_eq!(estimate.graph_nodes, 10);
        assert_eq!(estimate.graph_edges, 40);
    }

    #[test]
    fn decoder_iterations_are_derived() {
        let estimate = ResourceEstimator::new()
            .estimate_decoder(25)
            .expect("estimate must succeed");

        assert_eq!(
            estimate.decoder_iterations,
            25
        );
    }

    #[test]
    fn memory_estimate_is_nonzero_for_decoder_work() {
        let estimate = ResourceEstimator::new()
            .estimate_decoder(10)
            .expect("estimate must succeed");

        assert!(estimate.peak_memory_bytes > 0);
    }

    #[test]
    fn explicit_decoder_iterations_override_default() {
        let workload =
            WorkloadDimensions::new(3, 9, 8)
                .expect("valid workload")
                .with_rounds(3)
                .with_syndrome_events(20)
                .with_decoder_iterations(100);

        let estimate =
            ResourceEstimator::new()
                .estimate(workload)
                .expect("estimate must succeed");

        assert_eq!(
            estimate.decoder_iterations,
            100
        );
    }

    #[test]
    fn stream_estimate_tracks_buffer() {
        let estimate =
            ResourceEstimator::new()
                .estimate_streaming(
                    100,
                    25,
                    4,
                )
                .expect("estimate must succeed");

        assert_eq!(
            estimate.stream_buffer_events,
            25
        );

        assert_eq!(estimate.workers, 4);
    }

    #[test]
    fn partition_estimate_tracks_partitions() {
        let workload =
            WorkloadDimensions::new(5, 25, 24)
                .expect("valid workload")
                .with_rounds(5)
                .with_syndrome_events(100);

        let estimate =
            ResourceEstimator::new()
                .estimate_partitioned(
                    workload,
                    10,
                )
                .expect("estimate must succeed");

        assert_eq!(estimate.partitions, 10);
    }

    #[test]
    fn qpu_estimate_tracks_shots_and_circuits() {
        let workload =
            WorkloadDimensions::new(3, 9, 8)
                .expect("valid workload")
                .with_rounds(3)
                .with_syndrome_events(20);

        let estimate =
            ResourceEstimator::new()
                .estimate_qpu(
                    workload,
                    1000,
                    5,
                )
                .expect("estimate must succeed");

        assert_eq!(estimate.qpu_shots, 1000);
        assert_eq!(estimate.qpu_circuits, 5);
    }

    #[test]
    fn verification_estimate_tracks_operations() {
        let workload =
            WorkloadDimensions::new(3, 9, 8)
                .expect("valid workload")
                .with_rounds(3)
                .with_syndrome_events(20);

        let estimate =
            ResourceEstimator::new()
                .estimate_verification(
                    workload,
                    500,
                )
                .expect("estimate must succeed");

        assert_eq!(
            estimate.verification_operations,
            500
        );
    }

    #[test]
    fn admission_uses_canonical_limits() {
        let limits = QecLimits::new();

        let workload =
            WorkloadDimensions::new(
                3,
                9,
                8,
            )
            .expect("valid workload")
            .with_rounds(3)
            .with_syndrome_events(20);

        let estimate =
            ResourceEstimator::new()
                .estimate(workload)
                .expect("estimate must succeed");

        assert!(estimate.admit(&limits).is_ok());
    }

    #[test]
    fn excessive_qubit_estimate_is_rejected() {
        let mut limits = QecLimits::new();
        limits.max_qubits = 10;

        let workload =
            WorkloadDimensions::new(
                5,
                25,
                24,
            )
            .expect("valid workload");

        let estimate =
            ResourceEstimator::new()
                .estimate(workload)
                .expect("estimate must succeed");

        assert!(estimate.admit(&limits).is_err());
    }

    #[test]
    fn excessive_graph_estimate_is_rejected() {
        let mut limits = QecLimits::new();
        limits.max_graph_nodes = 10;

        let estimate =
            ResourceEstimator::new()
                .estimate_decoder(100)
                .expect("estimate must succeed");

        assert!(estimate.admit(&limits).is_err());
    }

    #[test]
    fn excessive_memory_estimate_is_rejected() {
        let mut limits = QecLimits::new();
        limits.max_memory_bytes = 1;

        let estimate =
            ResourceEstimator::new()
                .estimate_decoder(100)
                .expect("estimate must succeed");

        assert!(matches!(
            estimate.admit(&limits),
            Err(EstimateError::LimitExceeded {
                resource: LimitKind::MemoryBytes,
                ..
            })
        ));
    }

    #[test]
    fn overflow_is_rejected() {
        let model = ResourceEstimateModel {
            bytes_per_qubit: u64::MAX,
            ..ResourceEstimateModel::default()
        };

        let estimator =
            ResourceEstimator::with_model(model);

        let workload =
            WorkloadDimensions::new(
                2,
                2,
                1,
            )
            .expect("valid workload");

        let result =
            estimator.estimate(workload);

        assert!(matches!(
            result,
            Err(EstimateError::ArithmeticOverflow {
                resource: LimitKind::MemoryBytes,
                ..
            })
        ));
    }

    #[test]
    fn empty_estimate_is_empty() {
        assert!(
            ResourceEstimate::empty().is_empty()
        );
    }

    #[test]
    fn dominant_resource_is_deterministic() {
        let estimate = ResourceEstimate {
            schema_version:
                RESOURCE_ESTIMATE_SCHEMA_VERSION,
            qubits: 10,
            stabilizers: 5,
            syndrome_events: 20,
            measurement_rounds: 2,
            graph_nodes: 20,
            graph_edges: 80,
            peak_memory_bytes: 1000,
            decoder_iterations: 20,
            workers: 2,
            stream_buffer_events: 10,
            partitions: 1,
            qpu_shots: 0,
            qpu_circuits: 0,
            verification_operations: 0,
        };

        assert_eq!(
            estimate.dominant_resource(),
            Some(LimitKind::MemoryBytes)
        );
    }
}