//! Canonical resource policy for the Zamani Quantum Error Correction
//! subsystem.
//!
//! # Architectural role
//!
//! `QecLimits` is the single declarative resource policy used by the QEC
//! execution pipeline.
//!
//! ```text
//!                    QecConfig
//!                        |
//!                        v
//!                    QecLimits
//!                        |
//!          +-------------+-------------+
//!          |             |             |
//!          v             v             v
//!     Preflight      ResourceManager  Algorithms
//!          |             |             |
//!          +-------------+-------------+
//!                        |
//!                        v
//!                 ResourceSnapshot
//! ```
//!
//! The important distinction is:
//!
//! - `QecLimits` = what an execution is allowed to request.
//! - `ResourceManager` = what an execution has actually consumed.
//! - `ResourceSnapshot` = what an execution has consumed so far.
//!
//! This module does not allocate memory, execute decoders, access QPUs,
//! perform network I/O, or spawn workers.
//!
//! # Safety goals
//!
//! * No resource check performs allocation.
//! * No derived-resource calculation uses unchecked arithmetic.
//! * Malformed requests return structured errors.
//! * Every resource dimension has one canonical policy field.
//! * Resource estimates can be performed before construction.
//! * Limits are finite by default.
//! * Larger workloads require explicit configuration.
//! * Resource-limited execution must never be reported as successful
//!   mathematical verification.
//!
//! `QecLimits` is therefore a policy boundary, not a promise that a workload
//! can physically execute at its configured maximum.

use core::fmt;

use serde::{Deserialize, Serialize};

/// Current schema version for the canonical QEC resource policy.
pub const QEC_LIMITS_SCHEMA_VERSION: u32 = 2;

/* -------------------------------------------------------------------------- */
/* Conservative defaults                                                     */
/* -------------------------------------------------------------------------- */

/// Default maximum surface-code distance.
pub const DEFAULT_MAX_CODE_DISTANCE: usize = 10_001;

/// Default maximum number of physical qubits.
pub const DEFAULT_MAX_QUBITS: usize = 100_000_000;

/// Default maximum number of stabilizers.
pub const DEFAULT_MAX_STABILIZERS: usize = 100_000_000;

/// Default maximum retained/processed syndrome events.
pub const DEFAULT_MAX_SYNDROME_EVENTS: usize = 100_000_000;

/// Default maximum number of measurement rounds.
pub const DEFAULT_MAX_ROUNDS: usize = 10_000_000;

/// Default maximum decoding-graph nodes.
pub const DEFAULT_MAX_GRAPH_NODES: usize = 500_000_000;

/// Default maximum decoding-graph edges.
pub const DEFAULT_MAX_GRAPH_EDGES: usize = 2_000_000_000;

/// Default maximum memory budget: 64 GiB.
pub const DEFAULT_MAX_MEMORY_BYTES: u64 =
    64 * 1024 * 1024 * 1024;

/// Default maximum decoder time: 24 hours.
pub const DEFAULT_MAX_DECODER_TIME_NS: u64 =
    24 * 60 * 60 * 1_000_000_000;

/// Default maximum parallel workers.
pub const DEFAULT_MAX_PARALLELISM: usize = 1024;

/// Default maximum checkpoint size: 16 GiB.
pub const DEFAULT_MAX_CHECKPOINT_SIZE_BYTES: u64 =
    16 * 1024 * 1024 * 1024;

/// Default maximum partition count.
pub const DEFAULT_MAX_PARTITIONS: usize = 1_000_000;

/// Default maximum stream-buffer size.
pub const DEFAULT_MAX_STREAM_BUFFER_EVENTS: usize =
    10_000_000;

/// Default maximum decoder iterations.
pub const DEFAULT_MAX_DECODER_ITERATIONS: usize =
    10_000_000;

/// Default maximum stabilizer weight.
pub const DEFAULT_MAX_STABILIZER_WEIGHT: usize = 64;

/// Default maximum logical-operator weight.
pub const DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT: usize =
    100_000_000;

/// Default maximum qubits per partition.
pub const DEFAULT_MAX_QUBITS_PER_PARTITION: usize =
    10_000_000;

/// Default maximum QPU shots.
pub const DEFAULT_MAX_QPU_SHOTS: u64 = 1_000_000_000;

/// Default maximum QPU circuits in one operation.
pub const DEFAULT_MAX_QPU_CIRCUITS: u64 = 1_000_000;

/// Default maximum exact-verification operations.
///
/// Exact mathematical verification is intentionally much more restrictive
/// than ordinary decoding because some algorithms scale exponentially.
pub const DEFAULT_MAX_VERIFICATION_OPERATIONS: u64 =
    100_000_000;

/* -------------------------------------------------------------------------- */
/* Resource kinds                                                             */
/* -------------------------------------------------------------------------- */

/// Canonical resource dimensions controlled by [`QecLimits`].
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum LimitKind {
    CodeDistance,
    Qubits,
    Stabilizers,
    SyndromeEvents,
    MeasurementRounds,
    GraphNodes,
    GraphEdges,
    MemoryBytes,
    DecoderTimeNs,
    Parallelism,
    CheckpointSizeBytes,
    Partitions,
    StreamBufferEvents,
    DecoderIterations,
    StabilizerWeight,
    LogicalOperatorWeight,
    QubitsPerPartition,
    QpuShots,
    QpuCircuits,
    VerificationOperations,
}

impl LimitKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDistance => "code_distance",
            Self::Qubits => "qubits",
            Self::Stabilizers => "stabilizers",
            Self::SyndromeEvents => "syndrome_events",
            Self::MeasurementRounds => "measurement_rounds",
            Self::GraphNodes => "graph_nodes",
            Self::GraphEdges => "graph_edges",
            Self::MemoryBytes => "memory_bytes",
            Self::DecoderTimeNs => "decoder_time_ns",
            Self::Parallelism => "parallelism",
            Self::CheckpointSizeBytes => "checkpoint_size_bytes",
            Self::Partitions => "partitions",
            Self::StreamBufferEvents => "stream_buffer_events",
            Self::DecoderIterations => "decoder_iterations",
            Self::StabilizerWeight => "stabilizer_weight",
            Self::LogicalOperatorWeight => "logical_operator_weight",
            Self::QubitsPerPartition => "qubits_per_partition",
            Self::QpuShots => "qpu_shots",
            Self::QpuCircuits => "qpu_circuits",
            Self::VerificationOperations => "verification_operations",
        }
    }
}

impl fmt::Display for LimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* -------------------------------------------------------------------------- */
/* Errors                                                                     */
/* -------------------------------------------------------------------------- */

/// Error returned when a configured resource policy or request is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LimitError {
    /// The policy itself contains an invalid zero value.
    InvalidLimit {
        resource: LimitKind,
        value: u128,
    },

    /// A requested resource exceeds its configured maximum.
    Exceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    /// A derived-resource calculation overflowed.
    ArithmeticOverflow {
        resource: LimitKind,
    },

    /// Two limits violate an internal policy invariant.
    InconsistentLimits {
        resource: LimitKind,
        related_resource: LimitKind,
        reason: &'static str,
    },
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit { resource, value } => {
                write!(
                    f,
                    "invalid QEC limit for {resource}: value {value} \
                     must be greater than zero"
                )
            }

            Self::Exceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "{resource} limit exceeded: requested {requested}, \
                     maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "arithmetic overflow while estimating {resource}"
                )
            }

            Self::InconsistentLimits {
                resource,
                related_resource,
                reason,
            } => {
                write!(
                    f,
                    "inconsistent QEC limits: {resource} and \
                     {related_resource}: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for LimitError {}

/* -------------------------------------------------------------------------- */
/* Canonical policy                                                           */
/* -------------------------------------------------------------------------- */

/// Canonical resource policy for one QEC execution context.
///
/// This is the **single source of truth for declarative QEC resource
/// limits**. Configuration, preflight checks, surface-code construction,
/// graph construction, decoders, streaming, partitioning, checkpointing and
/// mathematical verification should consume this policy rather than invent
/// local production ceilings.
///
/// The structure contains only scalar values and is therefore cheap to copy.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct QecLimits {
    /// Schema version of this resource policy.
    pub schema_version: u32,

    /// Maximum supported code distance.
    pub max_code_distance: usize,

    /// Maximum physical qubit count.
    pub max_qubits: usize,

    /// Maximum stabilizer count.
    pub max_stabilizers: usize,

    /// Maximum syndrome/detection-event count.
    pub max_syndrome_events: usize,

    /// Maximum number of measurement rounds.
    pub max_rounds: usize,

    /// Maximum decoding-graph node count.
    pub max_graph_nodes: usize,

    /// Maximum decoding-graph edge count.
    pub max_graph_edges: usize,

    /// Maximum memory budget in bytes.
    pub max_memory_bytes: u64,

    /// Maximum decoder execution time in nanoseconds.
    pub max_decoder_time_ns: u64,

    /// Maximum number of parallel workers.
    pub max_parallelism: usize,

    /// Maximum checkpoint size in bytes.
    pub max_checkpoint_size_bytes: u64,

    /// Maximum number of partitions.
    pub max_partitions: usize,

    /// Maximum events retained in one stream buffer.
    pub max_stream_buffer_events: usize,

    /// Maximum decoder iterations.
    pub max_decoder_iterations: usize,

    /// Maximum number of data qubits touched by one stabilizer.
    pub max_stabilizer_weight: usize,

    /// Maximum logical-operator weight.
    pub max_logical_operator_weight: usize,

    /// Maximum qubits assigned to one partition.
    pub max_qubits_per_partition: usize,

    /// Maximum QPU shots in one operation.
    pub max_qpu_shots: u64,

    /// Maximum QPU circuits submitted by one operation.
    pub max_qpu_circuits: u64,

    /// Maximum operations permitted by exact mathematical verification.
    pub max_verification_operations: u64,
}

impl Default for QecLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl QecLimits {
    /// Creates the conservative production policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            schema_version: QEC_LIMITS_SCHEMA_VERSION,

            max_code_distance: DEFAULT_MAX_CODE_DISTANCE,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_stabilizers: DEFAULT_MAX_STABILIZERS,
            max_syndrome_events: DEFAULT_MAX_SYNDROME_EVENTS,
            max_rounds: DEFAULT_MAX_ROUNDS,
            max_graph_nodes: DEFAULT_MAX_GRAPH_NODES,
            max_graph_edges: DEFAULT_MAX_GRAPH_EDGES,

            max_memory_bytes: DEFAULT_MAX_MEMORY_BYTES,
            max_decoder_time_ns: DEFAULT_MAX_DECODER_TIME_NS,
            max_parallelism: DEFAULT_MAX_PARALLELISM,

            max_checkpoint_size_bytes:
                DEFAULT_MAX_CHECKPOINT_SIZE_BYTES,

            max_partitions: DEFAULT_MAX_PARTITIONS,
            max_stream_buffer_events:
                DEFAULT_MAX_STREAM_BUFFER_EVENTS,

            max_decoder_iterations:
                DEFAULT_MAX_DECODER_ITERATIONS,

            max_stabilizer_weight:
                DEFAULT_MAX_STABILIZER_WEIGHT,

            max_logical_operator_weight:
                DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT,

            max_qubits_per_partition:
                DEFAULT_MAX_QUBITS_PER_PARTITION,

            max_qpu_shots: DEFAULT_MAX_QPU_SHOTS,
            max_qpu_circuits: DEFAULT_MAX_QPU_CIRCUITS,

            max_verification_operations:
                DEFAULT_MAX_VERIFICATION_OPERATIONS,
        }
    }

    /// Validates the policy itself.
    ///
    /// This must be called before the policy is accepted by `QecConfig`.
    pub fn validate(&self) -> Result<(), LimitError> {
        if self.schema_version != QEC_LIMITS_SCHEMA_VERSION {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::CodeDistance,
                related_resource: LimitKind::Qubits,
                reason: "unsupported QecLimits schema version",
            });
        }

        self.require_nonzero(
            LimitKind::CodeDistance,
            self.max_code_distance,
        )?;

        self.require_nonzero(
            LimitKind::Qubits,
            self.max_qubits,
        )?;

        self.require_nonzero(
            LimitKind::Stabilizers,
            self.max_stabilizers,
        )?;

        self.require_nonzero(
            LimitKind::SyndromeEvents,
            self.max_syndrome_events,
        )?;

        self.require_nonzero(
            LimitKind::MeasurementRounds,
            self.max_rounds,
        )?;

        self.require_nonzero(
            LimitKind::GraphNodes,
            self.max_graph_nodes,
        )?;

        self.require_nonzero(
            LimitKind::GraphEdges,
            self.max_graph_edges,
        )?;

        self.require_nonzero(
            LimitKind::MemoryBytes,
            self.max_memory_bytes as u128,
        )?;

        self.require_nonzero(
            LimitKind::DecoderTimeNs,
            self.max_decoder_time_ns as u128,
        )?;

        self.require_nonzero(
            LimitKind::Parallelism,
            self.max_parallelism,
        )?;

        self.require_nonzero(
            LimitKind::CheckpointSizeBytes,
            self.max_checkpoint_size_bytes as u128,
        )?;

        self.require_nonzero(
            LimitKind::Partitions,
            self.max_partitions,
        )?;

        self.require_nonzero(
            LimitKind::StreamBufferEvents,
            self.max_stream_buffer_events,
        )?;

        self.require_nonzero(
            LimitKind::DecoderIterations,
            self.max_decoder_iterations,
        )?;

        self.require_nonzero(
            LimitKind::StabilizerWeight,
            self.max_stabilizer_weight,
        )?;

        self.require_nonzero(
            LimitKind::LogicalOperatorWeight,
            self.max_logical_operator_weight,
        )?;

        self.require_nonzero(
            LimitKind::QubitsPerPartition,
            self.max_qubits_per_partition,
        )?;

        self.require_nonzero(
            LimitKind::QpuShots,
            self.max_qpu_shots,
        )?;

        self.require_nonzero(
            LimitKind::QpuCircuits,
            self.max_qpu_circuits,
        )?;

        self.require_nonzero(
            LimitKind::VerificationOperations,
            self.max_verification_operations,
        )?;

        /*
         * A partition cannot legitimately contain more qubits than the
         * entire workload is allowed to contain.
         */
        if self.max_qubits_per_partition > self.max_qubits {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::QubitsPerPartition,
                related_resource: LimitKind::Qubits,
                reason:
                    "per-partition qubits exceed total allowed qubits",
            });
        }

        /*
         * A stream buffer is itself a syndrome-event workload.
         */
        if self.max_stream_buffer_events > self.max_syndrome_events {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::StreamBufferEvents,
                related_resource: LimitKind::SyndromeEvents,
                reason:
                    "stream buffer exceeds total syndrome-event limit",
            });
        }

        /*
         * Checkpoints cannot exceed the complete memory policy.
         */
        if self.max_checkpoint_size_bytes
            > self.max_memory_bytes
        {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::CheckpointSizeBytes,
                related_resource: LimitKind::MemoryBytes,
                reason:
                    "checkpoint size exceeds memory budget",
            });
        }

        Ok(())
    }

    fn require_nonzero(
        &self,
        resource: LimitKind,
        value: u128,
    ) -> Result<(), LimitError> {
        if value == 0 {
            return Err(LimitError::InvalidLimit {
                resource,
                value,
            });
        }

        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Direct checks                                                          */
    /* ---------------------------------------------------------------------- */

    pub fn check_code_distance(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::CodeDistance,
            requested,
            self.max_code_distance,
        )
    }

    pub fn check_qubits(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::Qubits,
            requested,
            self.max_qubits,
        )
    }

    pub fn check_stabilizers(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::Stabilizers,
            requested,
            self.max_stabilizers,
        )
    }

    pub fn check_syndrome_events(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::SyndromeEvents,
            requested,
            self.max_syndrome_events,
        )
    }

    pub fn check_rounds(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::MeasurementRounds,
            requested,
            self.max_rounds,
        )
    }

    pub fn check_graph_nodes(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::GraphNodes,
            requested,
            self.max_graph_nodes,
        )
    }

    pub fn check_graph_edges(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::GraphEdges,
            requested,
            self.max_graph_edges,
        )
    }

    pub fn check_memory(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::MemoryBytes,
            requested,
            self.max_memory_bytes,
        )
    }

    pub fn check_decoder_time_ns(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::DecoderTimeNs,
            requested,
            self.max_decoder_time_ns,
        )
    }

    pub fn check_parallelism(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::Parallelism,
            requested,
            self.max_parallelism,
        )
    }

    pub fn check_checkpoint_size(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::CheckpointSizeBytes,
            requested,
            self.max_checkpoint_size_bytes,
        )
    }

    pub fn check_partitions(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::Partitions,
            requested,
            self.max_partitions,
        )
    }

    pub fn check_stream_buffer(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::StreamBufferEvents,
            requested,
            self.max_stream_buffer_events,
        )
    }

    pub fn check_decoder_iterations(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::DecoderIterations,
            requested,
            self.max_decoder_iterations,
        )
    }

    pub fn check_stabilizer_weight(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::StabilizerWeight,
            requested,
            self.max_stabilizer_weight,
        )
    }

    pub fn check_logical_operator_weight(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::LogicalOperatorWeight,
            requested,
            self.max_logical_operator_weight,
        )
    }

    pub fn check_qubits_per_partition(
        &self,
        requested: usize,
    ) -> Result<(), LimitError> {
        self.check_usize(
            LimitKind::QubitsPerPartition,
            requested,
            self.max_qubits_per_partition,
        )
    }

    pub fn check_qpu_shots(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::QpuShots,
            requested,
            self.max_qpu_shots,
        )
    }

    pub fn check_qpu_circuits(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::QpuCircuits,
            requested,
            self.max_qpu_circuits,
        )
    }

    pub fn check_verification_operations(
        &self,
        requested: u64,
    ) -> Result<(), LimitError> {
        self.check_u64(
            LimitKind::VerificationOperations,
            requested,
            self.max_verification_operations,
        )
    }

    fn check_usize(
        &self,
        resource: LimitKind,
        requested: usize,
        maximum: usize,
    ) -> Result<(), LimitError> {
        if requested > maximum {
            return Err(LimitError::Exceeded {
                resource,
                requested: requested as u128,
                maximum: maximum as u128,
            });
        }

        Ok(())
    }

    fn check_u64(
        &self,
        resource: LimitKind,
        requested: u64,
        maximum: u64,
    ) -> Result<(), LimitError> {
        if requested > maximum {
            return Err(LimitError::Exceeded {
                resource,
                requested: requested as u128,
                maximum: maximum as u128,
            });
        }

        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Overflow-safe derived resource calculations                            */
    /* ---------------------------------------------------------------------- */

    /// Checked `distance²` calculation for surface-code data-qubit count.
    pub fn estimate_surface_code_qubits(
        &self,
        distance: usize,
    ) -> Result<usize, LimitError> {
        self.check_code_distance(distance)?;

        let qubits = distance
            .checked_mul(distance)
            .ok_or(LimitError::ArithmeticOverflow {
                resource: LimitKind::Qubits,
            })?;

        self.check_qubits(qubits)?;

        Ok(qubits)
    }

    /// Estimates a square surface-code stabilizer count.
    ///
    /// The topology module remains responsible for the exact mathematical
    /// stabilizer count. This helper is deliberately conservative and is
    /// intended for preflight only.
    pub fn estimate_surface_code_stabilizers(
        &self,
        distance: usize,
    ) -> Result<usize, LimitError> {
        self.check_code_distance(distance)?;

        let d_minus_one = distance
            .checked_sub(1)
            .ok_or(LimitError::ArithmeticOverflow {
                resource: LimitKind::Stabilizers,
            })?;

        let stabilizers = d_minus_one
            .checked_mul(d_minus_one)
            .and_then(|value| value.checked_add(
                d_minus_one,
            ))
            .ok_or(LimitError::ArithmeticOverflow {
                resource: LimitKind::Stabilizers,
            })?;

        self.check_stabilizers(stabilizers)?;

        Ok(stabilizers)
    }

    /// Estimates a square lattice graph-node count.
    pub fn estimate_surface_code_graph_nodes(
        &self,
        distance: usize,
        rounds: usize,
    ) -> Result<usize, LimitError> {
        self.check_code_distance(distance)?;
        self.check_rounds(rounds)?;

        let stabilizers =
            self.estimate_surface_code_stabilizers(distance)?;

        let nodes = stabilizers
            .checked_mul(rounds)
            .ok_or(LimitError::ArithmeticOverflow {
                resource: LimitKind::GraphNodes,
            })?;

        self.check_graph_nodes(nodes)?;

        Ok(nodes)
    }

    /// Conservative graph-edge estimate.
    ///
    /// The exact graph builder owns topology-specific edge generation. This
    /// method exists solely to prevent graph construction from starting when
    /// the requested workload is already obviously impossible.
    pub fn estimate_surface_code_graph_edges(
        &self,
        distance: usize,
        rounds: usize,
    ) -> Result<usize, LimitError> {
        let nodes =
            self.estimate_surface_code_graph_nodes(
                distance,
                rounds,
            )?;

        let edges = nodes
            .checked_mul(4)
            .and_then(|value| value.checked_add(
                nodes,
            ))
            .ok_or(LimitError::ArithmeticOverflow {
                resource: LimitKind::GraphEdges,
            })?;

        self.check_graph_edges(edges)?;

        Ok(edges)
    }

    /// Checked multiplication for workload dimensions.
    pub fn checked_product(
        &self,
        resource: LimitKind,
        lhs: usize,
        rhs: usize,
    ) -> Result<usize, LimitError> {
        let value = lhs
            .checked_mul(rhs)
            .ok_or(LimitError::ArithmeticOverflow {
                resource,
            })?;

        Ok(value)
    }

    /// Checked addition for workload dimensions.
    pub fn checked_sum(
        &self,
        resource: LimitKind,
        lhs: usize,
        rhs: usize,
    ) -> Result<usize, LimitError> {
        lhs.checked_add(rhs).ok_or(
            LimitError::ArithmeticOverflow { resource },
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Unified preflight                                                      */
    /* ---------------------------------------------------------------------- */

    /// Performs a complete bounded preflight for a generic QEC workload.
    ///
    /// This function is intended to run before expensive construction.
    pub fn preflight(
        &self,
        request: &QecResourceRequest,
    ) -> Result<QecResourceEstimate, LimitError> {
        self.validate()?;

        self.check_code_distance(
            request.code_distance,
        )?;

        self.check_qubits(request.qubits)?;
        self.check_stabilizers(request.stabilizers)?;
        self.check_syndrome_events(
            request.syndrome_events,
        )?;
        self.check_rounds(request.rounds)?;
        self.check_graph_nodes(
            request.graph_nodes,
        )?;
        self.check_graph_edges(
            request.graph_edges,
        )?;
        self.check_memory(request.memory_bytes)?;
        self.check_decoder_time_ns(
            request.decoder_time_ns,
        )?;
        self.check_parallelism(
            request.parallelism,
        )?;
        self.check_checkpoint_size(
            request.checkpoint_size_bytes,
        )?;
        self.check_partitions(request.partitions)?;
        self.check_stream_buffer(
            request.stream_buffer_events,
        )?;
        self.check_decoder_iterations(
            request.decoder_iterations,
        )?;
        self.check_stabilizer_weight(
            request.stabilizer_weight,
        )?;
        self.check_logical_operator_weight(
            request.logical_operator_weight,
        )?;
        self.check_qubits_per_partition(
            request.qubits_per_partition,
        )?;
        self.check_qpu_shots(request.qpu_shots)?;
        self.check_qpu_circuits(
            request.qpu_circuits,
        )?;
        self.check_verification_operations(
            request.verification_operations,
        )?;

        Ok(QecResourceEstimate::from_request(
            *request,
        ))
    }
}

/* -------------------------------------------------------------------------- */
/* Preflight request and estimate                                             */
/* -------------------------------------------------------------------------- */

/// Resource request used by constructors and execution planners before
/// allocation.
///
/// This structure contains requested resources rather than observed usage.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct QecResourceRequest {
    pub code_distance: usize,
    pub qubits: usize,
    pub stabilizers: usize,
    pub syndrome_events: usize,
    pub rounds: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub memory_bytes: u64,
    pub decoder_time_ns: u64,
    pub parallelism: usize,
    pub checkpoint_size_bytes: u64,
    pub partitions: usize,
    pub stream_buffer_events: usize,
    pub decoder_iterations: usize,
    pub stabilizer_weight: usize,
    pub logical_operator_weight: usize,
    pub qubits_per_partition: usize,
    pub qpu_shots: u64,
    pub qpu_circuits: u64,
    pub verification_operations: u64,
}

/// Result of a successful resource preflight.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct QecResourceEstimate {
    pub requested: QecResourceRequest,
}

impl QecResourceEstimate {
    const fn from_request(
        requested: QecResourceRequest,
    ) -> Self {
        Self { requested }
    }

    #[must_use]
    pub const fn requested(
        &self,
    ) -> QecResourceRequest {
        self.requested
    }
}

/* -------------------------------------------------------------------------- */
/* Surface-code-specific preflight                                           */
/* -------------------------------------------------------------------------- */

/// Resource estimate for surface-code construction.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SurfaceCodeResourceEstimate {
    pub distance: usize,
    pub rounds: usize,
    pub data_qubits: usize,
    pub stabilizers: usize,
    pub graph_nodes: usize,
    pub conservative_graph_edges: usize,
}

impl QecLimits {
    /// Performs all resource checks necessary before constructing a
    /// surface-code workload.
    ///
    /// This is the API that `surface_code.rs` should use before allocating
    /// its qubit/stabilizer/topology vectors.
    pub fn preflight_surface_code(
        &self,
        distance: usize,
        rounds: usize,
    ) -> Result<SurfaceCodeResourceEstimate, LimitError> {
        self.validate()?;
        self.check_code_distance(distance)?;
        self.check_rounds(rounds)?;

        let data_qubits =
            self.estimate_surface_code_qubits(distance)?;

        let stabilizers =
            self.estimate_surface_code_stabilizers(distance)?;

        let graph_nodes =
            self.estimate_surface_code_graph_nodes(
                distance,
                rounds,
            )?;

        let conservative_graph_edges =
            self.estimate_surface_code_graph_edges(
                distance,
                rounds,
            )?;

        Ok(SurfaceCodeResourceEstimate {
            distance,
            rounds,
            data_qubits,
            stabilizers,
            graph_nodes,
            conservative_graph_edges,
        })
    }
}

/* -------------------------------------------------------------------------- */
/* Runtime-resource compatibility                                             */
/* -------------------------------------------------------------------------- */

/*
 * `resources.rs` currently has its own ResourceLimits structure because it
 * also carries a runtime wall-time Duration. The canonical declarative
 * policy remains QecLimits.
 *
 * This adapter intentionally does not introduce a second set of ceilings.
 * Runtime resource accounting receives the values already approved here.
 */

impl QecLimits {
    /// Returns the runtime memory ceiling.
    #[must_use]
    pub const fn memory_budget_bytes(
        &self,
    ) -> u64 {
        self.max_memory_bytes
    }

    /// Returns the runtime syndrome-event ceiling.
    #[must_use]
    pub const fn syndrome_event_budget(
        &self,
    ) -> u64 {
        self.max_syndrome_events as u64
    }

    /// Returns the runtime graph-node ceiling.
    #[must_use]
    pub const fn graph_node_budget(
        &self,
    ) -> u64 {
        self.max_graph_nodes as u64
    }

    /// Returns the runtime graph-edge ceiling.
    #[must_use]
    pub const fn graph_edge_budget(
        &self,
    ) -> u64 {
        self.max_graph_edges as u64
    }

    /// Returns the runtime decoder-iteration ceiling.
    #[must_use]
    pub const fn decoder_iteration_budget(
        &self,
    ) -> u64 {
        self.max_decoder_iterations as u64
    }

    /// Returns the runtime parallel-worker ceiling.
    #[must_use]
    pub const fn parallelism_budget(
        &self,
    ) -> usize {
        self.max_parallelism
    }

    /// Returns the configured decoder deadline.
    #[must_use]
    pub const fn decoder_deadline_ns(
        &self,
    ) -> u64 {
        self.max_decoder_time_ns
    }
}

/* -------------------------------------------------------------------------- */
/* Tests                                                                      */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_limits_are_valid() {
        QecLimits::default()
            .validate()
            .expect("default limits must be valid");
    }

    #[test]
    fn surface_code_preflight_rejects_before_allocation() {
        let mut limits = QecLimits::default();

        limits.max_code_distance = 3;
        limits.max_qubits = 8;

        let result =
            limits.preflight_surface_code(3, 3);

        assert!(result.is_ok());

        limits.max_qubits = 4;

        let result =
            limits.preflight_surface_code(3, 3);

        assert!(matches!(
            result,
            Err(LimitError::Exceeded {
                resource: LimitKind::Qubits,
                ..
            })
        ));
    }

    #[test]
    fn distance_square_overflow_is_rejected() {
        let limits = QecLimits::default();

        let result =
            limits.estimate_surface_code_qubits(
                usize::MAX,
            );

        assert!(result.is_err());
    }

    #[test]
    fn stream_buffer_cannot_exceed_event_budget() {
        let mut limits = QecLimits::default();

        limits.max_syndrome_events = 100;
        limits.max_stream_buffer_events = 101;

        assert!(matches!(
            limits.validate(),
            Err(LimitError::InconsistentLimits {
                resource:
                    LimitKind::StreamBufferEvents,
                related_resource:
                    LimitKind::SyndromeEvents,
                ..
            })
        ));
    }

    #[test]
    fn partition_cannot_exceed_total_qubits() {
        let mut limits = QecLimits::default();

        limits.max_qubits = 100;
        limits.max_qubits_per_partition = 101;

        assert!(matches!(
            limits.validate(),
            Err(LimitError::InconsistentLimits {
                resource:
                    LimitKind::QubitsPerPartition,
                related_resource:
                    LimitKind::Qubits,
                ..
            })
        ));
    }

    #[test]
    fn checkpoint_cannot_exceed_memory_budget() {
        let mut limits = QecLimits::default();

        limits.max_memory_bytes = 1024;
        limits.max_checkpoint_size_bytes = 2048;

        assert!(matches!(
            limits.validate(),
            Err(LimitError::InconsistentLimits {
                resource:
                    LimitKind::CheckpointSizeBytes,
                related_resource:
                    LimitKind::MemoryBytes,
                ..
            })
        ));
    }

    #[test]
    fn generic_preflight_checks_every_dimension() {
        let limits = QecLimits::default();

        let request = QecResourceRequest {
            code_distance: 3,
            qubits: 9,
            stabilizers: 4,
            syndrome_events: 10,
            rounds: 3,
            graph_nodes: 12,
            graph_edges: 20,
            memory_bytes: 4096,
            decoder_time_ns: 1_000,
            parallelism: 1,
            checkpoint_size_bytes: 1024,
            partitions: 1,
            stream_buffer_events: 10,
            decoder_iterations: 100,
            stabilizer_weight: 4,
            logical_operator_weight: 3,
            qubits_per_partition: 9,
            qpu_shots: 1,
            qpu_circuits: 1,
            verification_operations: 1,
        };

        let estimate =
            limits.preflight(&request)
                .expect("valid request must pass");

        assert_eq!(
            estimate.requested(),
            request
        );
    }

    #[test]
    fn direct_checks_are_deterministic() {
        let limits = QecLimits {
            max_code_distance: 5,
            ..QecLimits::default()
        };

        assert!(limits.check_code_distance(5).is_ok());

        assert!(matches!(
            limits.check_code_distance(6),
            Err(LimitError::Exceeded {
                resource: LimitKind::CodeDistance,
                requested: 6,
                maximum: 5,
            })
        ));
    }

    #[test]
    fn schema_is_explicit() {
        assert_eq!(
            QecLimits::default().schema_version,
            QEC_LIMITS_SCHEMA_VERSION
        );
    }
}