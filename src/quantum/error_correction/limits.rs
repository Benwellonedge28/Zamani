//! Canonical declarative resource policy for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! `QecLimits` is the single source of truth for declarative QEC admission
//! and preflight limits.
//!
//! ```text
//!                         QecConfig
//!                            |
//!                            v
//!                         QecLimits
//!                            |
//!          +-----------------+-----------------+
//!          |                 |                 |
//!          v                 v                 v
//!      Validation       ResourceManager    Algorithms
//!          |                 |                 |
//!          +-----------------+-----------------+
//!                            |
//!                            v
//!                    Runtime accounting
//! ```
//!
//! The responsibilities are deliberately separated:
//!
//! - `QecLimits` = what an execution is allowed to request.
//! - `resources.rs` = what an execution has actually consumed.
//! - `memory.rs` = allocation/reservation enforcement.
//! - `configuration.rs` = validated composition of the complete QEC policy.
//!
//! This module does NOT:
//!
//! - allocate memory;
//! - execute decoders;
//! - access a QPU;
//! - perform network I/O;
//! - spawn workers;
//! - maintain runtime counters;
//! - implement decoder-specific algorithms.
//!
//! # Design guarantees
//!
//! 1. Every production resource ceiling has one canonical field.
//! 2. Derived resource calculations use checked arithmetic.
//! 3. Resource checks occur before allocation or execution.
//! 4. Zero limits are rejected.
//! 5. Invalid policy relationships are rejected.
//! 6. Large workloads are bounded by explicit policy.
//! 7. Runtime accounting remains in `resources.rs`.
//! 8. Resource policy is independent of high-level QEC algorithms.
//! 9. The API is deterministic and allocation-free.
//! 10. Rust 1.97.1 compatible.
//!
//! # Integration contract
//!
//! `configuration.rs` should validate and embed `QecLimits`.
//!
//! `resources.rs` should use the scalar fields and `LimitKind` rather than
//! introducing another production policy.
//!
//! `memory.rs` should use `validate_memory()` before reservations.
//!
//! `surface_code.rs` should use `validate_code_size()` before construction.
//!
//! `decoding_graph.rs` should use `validate_graph()` before allocation.
//!
//! `syndrome.rs` should use `validate_syndrome()`.
//!
//! `decoder.rs`, `mwpm.rs`, and `union_find.rs` should use
//! `validate_decoder_work()`.
//!
//! `partition.rs` should use `validate_partition()`.
//!
//! `streaming.rs` should use `validate_stream()`.
//!
//! `checkpoint.rs` should use `validate_checkpoint()`.
//!
//! QPU integration should use `validate_qpu()`.
//!
//! Mathematical verification should use `validate_verification()`.
//!
//! No later module should create a second independent production limit
//! structure.

use core::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for the canonical `QecLimits` representation.
///
/// Increment this when the serialized meaning or structure of `QecLimits`
/// changes incompatibly.
pub const QEC_LIMITS_SCHEMA_VERSION: u32 = 3;

/* ========================================================================== */
/* Conservative defaults                                                      */
/* ========================================================================== */

/// Default maximum surface-code distance.
pub const DEFAULT_MAX_CODE_DISTANCE: usize = 10_001;

/// Default maximum physical qubit count.
pub const DEFAULT_MAX_QUBITS: usize = 100_000_000;

/// Default maximum stabilizer count.
pub const DEFAULT_MAX_STABILIZERS: usize = 100_000_000;

/// Default maximum retained/processed syndrome events.
pub const DEFAULT_MAX_SYNDROME_EVENTS: usize = 100_000_000;

/// Default maximum measurement rounds.
pub const DEFAULT_MAX_ROUNDS: usize = 10_000_000;

/// Default maximum decoding-graph nodes.
pub const DEFAULT_MAX_GRAPH_NODES: usize = 500_000_000;

/// Default maximum decoding-graph edges.
pub const DEFAULT_MAX_GRAPH_EDGES: usize = 2_000_000_000;

/// Default maximum memory budget: 64 GiB.
pub const DEFAULT_MAX_MEMORY_BYTES: u64 = 64 * 1024 * 1024 * 1024;

/// Default maximum decoder execution time: 24 hours.
pub const DEFAULT_MAX_DECODER_TIME_NS: u64 =
    24 * 60 * 60 * 1_000_000_000;

/// Default maximum parallel workers.
pub const DEFAULT_MAX_PARALLELISM: usize = 1024;

/// Default maximum checkpoint size: 16 GiB.
pub const DEFAULT_MAX_CHECKPOINT_SIZE_BYTES: u64 =
    16 * 1024 * 1024 * 1024;

/// Default maximum number of partitions.
pub const DEFAULT_MAX_PARTITIONS: usize = 1_000_000;

/// Default maximum stream-buffer events.
pub const DEFAULT_MAX_STREAM_BUFFER_EVENTS: usize = 10_000_000;

/// Default maximum decoder iterations.
pub const DEFAULT_MAX_DECODER_ITERATIONS: usize = 10_000_000;

/// Default maximum stabilizer weight.
pub const DEFAULT_MAX_STABILIZER_WEIGHT: usize = 64;

/// Default maximum logical-operator weight.
pub const DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT: usize = 100_000_000;

/// Default maximum qubits in one partition.
pub const DEFAULT_MAX_QUBITS_PER_PARTITION: usize = 10_000_000;

/// Default maximum QPU shots in one operation.
pub const DEFAULT_MAX_QPU_SHOTS: u64 = 1_000_000_000;

/// Default maximum QPU circuits in one operation.
pub const DEFAULT_MAX_QPU_CIRCUITS: u64 = 1_000_000;

/// Default maximum mathematical-verification operations.
pub const DEFAULT_MAX_VERIFICATION_OPERATIONS: u64 = 100_000_000;

/* ========================================================================== */
/* Limit kinds                                                                */
/* ========================================================================== */

/// Every declarative resource dimension controlled by `QecLimits`.
///
/// This enum intentionally describes policy dimensions rather than runtime
/// counters. Runtime resource kinds belong to `resources.rs`.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
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

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Error returned by resource-policy validation or preflight.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LimitError {
    /// A configured limit is zero and therefore unusable.
    InvalidLimit {
        resource: LimitKind,
        value: u128,
    },

    /// A requested amount exceeds the configured policy.
    Exceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    /// A checked derived-resource calculation overflowed.
    ArithmeticOverflow {
        resource: LimitKind,
    },

    /// Two policy dimensions contradict one another.
    InconsistentLimits {
        resource: LimitKind,
        related_resource: LimitKind,
        reason: &'static str,
    },

    /// The serialized policy uses an unsupported schema.
    UnsupportedSchema {
        found: u32,
        expected: u32,
    },
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit { resource, value } => {
                write!(
                    f,
                    "invalid QEC limit for {resource}: \
                     {value} must be greater than zero"
                )
            }

            Self::Exceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "{resource} limit exceeded: \
                     requested {requested}, maximum {maximum}"
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

            Self::UnsupportedSchema { found, expected } => {
                write!(
                    f,
                    "unsupported QEC limits schema {found}; \
                     expected {expected}"
                )
            }
        }
    }
}

impl std::error::Error for LimitError {}

/* ========================================================================== */
/* Canonical policy                                                           */
/* ========================================================================== */

/// Canonical declarative resource policy for one QEC execution context.
///
/// This is the single source of truth for production resource ceilings.
///
/// It contains policy only. Runtime usage belongs to `resources.rs`.
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
    /// Schema version of this policy.
    pub schema_version: u32,

    /// Maximum supported code distance.
    pub max_code_distance: usize,

    /// Maximum physical qubits.
    pub max_qubits: usize,

    /// Maximum stabilizer generators.
    pub max_stabilizers: usize,

    /// Maximum syndrome/detection events.
    pub max_syndrome_events: usize,

    /// Maximum measurement rounds.
    pub max_rounds: usize,

    /// Maximum decoding-graph nodes.
    pub max_graph_nodes: usize,

    /// Maximum decoding-graph edges.
    pub max_graph_edges: usize,

    /// Maximum memory budget in bytes.
    pub max_memory_bytes: u64,

    /// Maximum decoder execution time in nanoseconds.
    pub max_decoder_time_ns: u64,

    /// Maximum parallel workers.
    pub max_parallelism: usize,

    /// Maximum checkpoint size in bytes.
    pub max_checkpoint_size_bytes: u64,

    /// Maximum number of partitions.
    pub max_partitions: usize,

    /// Maximum events in one stream buffer.
    pub max_stream_buffer_events: usize,

    /// Maximum decoder iterations.
    pub max_decoder_iterations: usize,

    /// Maximum weight of one stabilizer.
    pub max_stabilizer_weight: usize,

    /// Maximum logical-operator weight.
    pub max_logical_operator_weight: usize,

    /// Maximum qubits in one partition.
    pub max_qubits_per_partition: usize,

    /// Maximum QPU shots in one operation.
    pub max_qpu_shots: u64,

    /// Maximum QPU circuits in one operation.
    pub max_qpu_circuits: u64,

    /// Maximum operations for exact mathematical verification.
    pub max_verification_operations: u64,
}

impl Default for QecLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl QecLimits {
    /// Creates the canonical conservative production policy.
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

    /* ---------------------------------------------------------------------- */
    /* Policy validation                                                      */
    /* ---------------------------------------------------------------------- */

    /// Validates the policy itself.
    ///
    /// `configuration.rs` should call this before accepting a `QecLimits`
    /// instance into a validated `QecConfig`.
    pub fn validate(&self) -> Result<(), LimitError> {
        if self.schema_version != QEC_LIMITS_SCHEMA_VERSION {
            return Err(LimitError::UnsupportedSchema {
                found: self.schema_version,
                expected: QEC_LIMITS_SCHEMA_VERSION,
            });
        }

        self.require_nonzero(
            LimitKind::CodeDistance,
            self.max_code_distance as u128,
        )?;

        self.require_nonzero(
            LimitKind::Qubits,
            self.max_qubits as u128,
        )?;

        self.require_nonzero(
            LimitKind::Stabilizers,
            self.max_stabilizers as u128,
        )?;

        self.require_nonzero(
            LimitKind::SyndromeEvents,
            self.max_syndrome_events as u128,
        )?;

        self.require_nonzero(
            LimitKind::MeasurementRounds,
            self.max_rounds as u128,
        )?;

        self.require_nonzero(
            LimitKind::GraphNodes,
            self.max_graph_nodes as u128,
        )?;

        self.require_nonzero(
            LimitKind::GraphEdges,
            self.max_graph_edges as u128,
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
            self.max_parallelism as u128,
        )?;

        self.require_nonzero(
            LimitKind::CheckpointSizeBytes,
            self.max_checkpoint_size_bytes as u128,
        )?;

        self.require_nonzero(
            LimitKind::Partitions,
            self.max_partitions as u128,
        )?;

        self.require_nonzero(
            LimitKind::StreamBufferEvents,
            self.max_stream_buffer_events as u128,
        )?;

        self.require_nonzero(
            LimitKind::DecoderIterations,
            self.max_decoder_iterations as u128,
        )?;

        self.require_nonzero(
            LimitKind::StabilizerWeight,
            self.max_stabilizer_weight as u128,
        )?;

        self.require_nonzero(
            LimitKind::LogicalOperatorWeight,
            self.max_logical_operator_weight as u128,
        )?;

        self.require_nonzero(
            LimitKind::QubitsPerPartition,
            self.max_qubits_per_partition as u128,
        )?;

        self.require_nonzero(
            LimitKind::QpuShots,
            self.max_qpu_shots as u128,
        )?;

        self.require_nonzero(
            LimitKind::QpuCircuits,
            self.max_qpu_circuits as u128,
        )?;

        self.require_nonzero(
            LimitKind::VerificationOperations,
            self.max_verification_operations as u128,
        )?;

        /*
         * These invariants are intentionally conservative and describe the
         * canonical stabilizer-code execution model.
         */

        if self.max_stabilizers > self.max_qubits {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::Stabilizers,
                related_resource: LimitKind::Qubits,
                reason:
                    "stabilizers cannot exceed qubits under the canonical \
                     stabilizer-code policy",
            });
        }

        if self.max_qubits_per_partition > self.max_qubits {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::QubitsPerPartition,
                related_resource: LimitKind::Qubits,
                reason:
                    "per-partition qubits cannot exceed total allowed \
                     qubits",
            });
        }

        if self.max_stream_buffer_events > self.max_syndrome_events {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::StreamBufferEvents,
                related_resource: LimitKind::SyndromeEvents,
                reason:
                    "stream-buffer capacity cannot exceed the total \
                     syndrome-event policy",
            });
        }

        if self.max_logical_operator_weight > self.max_qubits {
            return Err(LimitError::InconsistentLimits {
                resource: LimitKind::LogicalOperatorWeight,
                related_resource: LimitKind::Qubits,
                reason:
                    "logical-operator weight cannot exceed total allowed \
                     qubits",
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
            Err(LimitError::InvalidLimit { resource, value })
        } else {
            Ok(())
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Generic checking                                                       */
    /* ---------------------------------------------------------------------- */

    /// Checks an arbitrary resource request.
    ///
    /// This is the preferred primitive for new QEC modules when no
    /// domain-specific helper below is appropriate.
    pub fn check(
        &self,
        resource: LimitKind,
        requested: u128,
    ) -> Result<(), LimitError> {
        let maximum = self.maximum(resource);

        if requested > maximum {
            Err(LimitError::Exceeded {
                resource,
                requested,
                maximum,
            })
        } else {
            Ok(())
        }
    }

    /// Returns a configured maximum as `u128`.
    ///
    /// `u128` prevents intermediate narrowing when callers perform
    /// cross-platform comparisons.
    #[must_use]
    pub const fn maximum(&self, resource: LimitKind) -> u128 {
        match resource {
            LimitKind::CodeDistance => self.max_code_distance as u128,
            LimitKind::Qubits => self.max_qubits as u128,
            LimitKind::Stabilizers => self.max_stabilizers as u128,
            LimitKind::SyndromeEvents => {
                self.max_syndrome_events as u128
            }
            LimitKind::MeasurementRounds => self.max_rounds as u128,
            LimitKind::GraphNodes => self.max_graph_nodes as u128,
            LimitKind::GraphEdges => self.max_graph_edges as u128,
            LimitKind::MemoryBytes => self.max_memory_bytes as u128,
            LimitKind::DecoderTimeNs => {
                self.max_decoder_time_ns as u128
            }
            LimitKind::Parallelism => self.max_parallelism as u128,
            LimitKind::CheckpointSizeBytes => {
                self.max_checkpoint_size_bytes as u128
            }
            LimitKind::Partitions => self.max_partitions as u128,
            LimitKind::StreamBufferEvents => {
                self.max_stream_buffer_events as u128
            }
            LimitKind::DecoderIterations => {
                self.max_decoder_iterations as u128
            }
            LimitKind::StabilizerWeight => {
                self.max_stabilizer_weight as u128
            }
            LimitKind::LogicalOperatorWeight => {
                self.max_logical_operator_weight as u128
            }
            LimitKind::QubitsPerPartition => {
                self.max_qubits_per_partition as u128
            }
            LimitKind::QpuShots => self.max_qpu_shots as u128,
            LimitKind::QpuCircuits => self.max_qpu_circuits as u128,
            LimitKind::VerificationOperations => {
                self.max_verification_operations as u128
            }
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Domain-specific preflight                                             */
    /* ---------------------------------------------------------------------- */

    /// Validates code distance and primary code resources.
    pub fn validate_code_size(
        &self,
        distance: usize,
        qubits: usize,
        stabilizers: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::CodeDistance,
            distance as u128,
        )?;

        self.check(
            LimitKind::Qubits,
            qubits as u128,
        )?;

        self.check(
            LimitKind::Stabilizers,
            stabilizers as u128,
        )
    }

    /// Validates a memory reservation before allocation.
    pub fn validate_memory(
        &self,
        bytes: u64,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::MemoryBytes,
            bytes as u128,
        )
    }

    /// Validates decoding-graph size before graph allocation.
    pub fn validate_graph(
        &self,
        nodes: usize,
        edges: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::GraphNodes,
            nodes as u128,
        )?;

        self.check(
            LimitKind::GraphEdges,
            edges as u128,
        )
    }

    /// Validates syndrome-event and round counts.
    pub fn validate_syndrome(
        &self,
        events: usize,
        rounds: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::SyndromeEvents,
            events as u128,
        )?;

        self.check(
            LimitKind::MeasurementRounds,
            rounds as u128,
        )
    }

    /// Validates decoder iteration and time budgets.
    pub fn validate_decoder_work(
        &self,
        iterations: usize,
        time_ns: u64,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::DecoderIterations,
            iterations as u128,
        )?;

        self.check(
            LimitKind::DecoderTimeNs,
            time_ns as u128,
        )
    }

    /// Validates partition count and partition size.
    pub fn validate_partition(
        &self,
        partitions: usize,
        qubits_per_partition: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::Partitions,
            partitions as u128,
        )?;

        self.check(
            LimitKind::QubitsPerPartition,
            qubits_per_partition as u128,
        )
    }

    /// Validates bounded streaming capacity.
    pub fn validate_stream(
        &self,
        buffer_events: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::StreamBufferEvents,
            buffer_events as u128,
        )
    }

    /// Validates checkpoint size before persistence.
    pub fn validate_checkpoint(
        &self,
        bytes: u64,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::CheckpointSizeBytes,
            bytes as u128,
        )
    }

    /// Validates QPU shot/circuit budgets.
    pub fn validate_qpu(
        &self,
        shots: u64,
        circuits: u64,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::QpuShots,
            shots as u128,
        )?;

        self.check(
            LimitKind::QpuCircuits,
            circuits as u128,
        )
    }

    /// Validates exact mathematical verification work.
    pub fn validate_verification(
        &self,
        operations: u64,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::VerificationOperations,
            operations as u128,
        )
    }

    /// Validates stabilizer weight.
    pub fn validate_stabilizer(
        &self,
        weight: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::StabilizerWeight,
            weight as u128,
        )
    }

    /// Validates logical-operator weight.
    pub fn validate_logical_operator(
        &self,
        weight: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::LogicalOperatorWeight,
            weight as u128,
        )
    }

    /// Validates parallel worker count.
    pub fn validate_parallelism(
        &self,
        workers: usize,
    ) -> Result<(), LimitError> {
        self.check(
            LimitKind::Parallelism,
            workers as u128,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Checked derived-resource calculations                                  */
    /* ---------------------------------------------------------------------- */

    /// Performs checked multiplication and then applies the policy.
    pub fn checked_product(
        &self,
        resource: LimitKind,
        a: u64,
        b: u64,
    ) -> Result<u64, LimitError> {
        let value = a.checked_mul(b).ok_or(
            LimitError::ArithmeticOverflow { resource },
        )?;

        self.check(resource, value as u128)?;

        Ok(value)
    }

    /// Performs checked addition and then applies the policy.
    pub fn checked_sum(
        &self,
        resource: LimitKind,
        a: u64,
        b: u64,
    ) -> Result<u64, LimitError> {
        let value = a.checked_add(b).ok_or(
            LimitError::ArithmeticOverflow { resource },
        )?;

        self.check(resource, value as u128)?;

        Ok(value)
    }

    /// Calculates `count * bytes_per_item` safely.
    pub fn checked_bytes(
        &self,
        count: u64,
        bytes_per_item: u64,
    ) -> Result<u64, LimitError> {
        self.checked_product(
            LimitKind::MemoryBytes,
            count,
            bytes_per_item,
        )
    }

    /// Calculates the number of edges in a complete undirected graph.
    ///
    /// This is an estimator only. It performs no allocation.
    pub fn checked_complete_graph_edges(
        &self,
        nodes: u64,
    ) -> Result<u64, LimitError> {
        if nodes < 2 {
            return Ok(0);
        }

        let n_minus_one = nodes.checked_sub(1).ok_or(
            LimitError::ArithmeticOverflow {
                resource: LimitKind::GraphEdges,
            },
        )?;

        let product = nodes.checked_mul(n_minus_one).ok_or(
            LimitError::ArithmeticOverflow {
                resource: LimitKind::GraphEdges,
            },
        )?;

        let edges = product / 2;

        self.check(
            LimitKind::GraphEdges,
            edges as u128,
        )?;

        Ok(edges)
    }

    /// Calculates `ceil(a / b)` safely.
    pub fn checked_ceil_div(
        &self,
        resource: LimitKind,
        a: u64,
        b: u64,
    ) -> Result<u64, LimitError> {
        if b == 0 {
            return Err(LimitError::ArithmeticOverflow { resource });
        }

        let adjusted = a.checked_add(b - 1).ok_or(
            LimitError::ArithmeticOverflow { resource },
        )?;

        let value = adjusted / b;

        self.check(resource, value as u128)?;

        Ok(value)
    }

    /* ---------------------------------------------------------------------- */
    /* Explicit configuration helpers                                         */
    /* ---------------------------------------------------------------------- */

    #[must_use]
    pub const fn with_max_memory_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.max_memory_bytes = value;
        self
    }

    #[must_use]
    pub const fn with_max_qubits(
        mut self,
        value: usize,
    ) -> Self {
        self.max_qubits = value;
        self
    }

    #[must_use]
    pub const fn with_max_code_distance(
        mut self,
        value: usize,
    ) -> Self {
        self.max_code_distance = value;
        self
    }

    #[must_use]
    pub const fn with_max_syndrome_events(
        mut self,
        value: usize,
    ) -> Self {
        self.max_syndrome_events = value;
        self
    }

    #[must_use]
    pub const fn with_max_graph_nodes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_graph_nodes = value;
        self
    }

    #[must_use]
    pub const fn with_max_graph_edges(
        mut self,
        value: usize,
    ) -> Self {
        self.max_graph_edges = value;
        self
    }

    #[must_use]
    pub const fn with_max_decoder_iterations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_decoder_iterations = value;
        self
    }

    #[must_use]
    pub const fn with_max_parallelism(
        mut self,
        value: usize,
    ) -> Self {
        self.max_parallelism = value;
        self
    }

    #[must_use]
    pub const fn with_max_partitions(
        mut self,
        value: usize,
    ) -> Self {
        self.max_partitions = value;
        self
    }

    #[must_use]
    pub const fn with_max_stream_buffer_events(
        mut self,
        value: usize,
    ) -> Self {
        self.max_stream_buffer_events = value;
        self
    }

    #[must_use]
    pub const fn with_max_qpu_shots(
        mut self,
        value: u64,
    ) -> Self {
        self.max_qpu_shots = value;
        self
    }

    #[must_use]
    pub const fn with_max_qpu_circuits(
        mut self,
        value: u64,
    ) -> Self {
        self.max_qpu_circuits = value;
        self
    }

    #[must_use]
    pub const fn with_max_verification_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.max_verification_operations = value;
        self
    }
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid() {
        assert!(QecLimits::default().validate().is_ok());
    }

    #[test]
    fn zero_limit_is_rejected() {
        let limits = QecLimits {
            max_qubits: 0,
            ..QecLimits::default()
        };

        assert!(matches!(
            limits.validate(),
            Err(LimitError::InvalidLimit {
                resource: LimitKind::Qubits,
                ..
            })
        ));
    }

    #[test]
    fn scalar_limit_is_enforced() {
        let limits = QecLimits::default().with_max_qubits(10);

        assert!(
            limits
                .check(LimitKind::Qubits, 10)
                .is_ok()
        );

        assert!(
            limits
                .check(LimitKind::Qubits, 11)
                .is_err()
        );
    }

    #[test]
    fn code_size_is_checked_before_construction() {
        let limits = QecLimits::default()
            .with_max_code_distance(5)
            .with_max_qubits(100);

        assert!(
            limits
                .validate_code_size(5, 100, 100)
                .is_ok()
        );

        assert!(
            limits
                .validate_code_size(6, 100, 100)
                .is_err()
        );
    }

    #[test]
    fn memory_is_checked_before_allocation() {
        let limits = QecLimits::default()
            .with_max_memory_bytes(100);

        assert!(
            limits
                .checked_bytes(10, 10)
                .is_ok()
        );

        assert!(
            limits
                .checked_bytes(11, 10)
                .is_err()
        );
    }

    #[test]
    fn complete_graph_estimate_is_checked() {
        let limits = QecLimits::default()
            .with_max_graph_edges(3);

        assert_eq!(
            limits
                .checked_complete_graph_edges(3)
                .unwrap(),
            3
        );

        assert!(
            limits
                .checked_complete_graph_edges(4)
                .is_err()
        );
    }

    #[test]
    fn overflow_is_reported() {
        let limits = QecLimits::default();

        assert!(matches!(
            limits.checked_product(
                LimitKind::MemoryBytes,
                u64::MAX,
                2,
            ),
            Err(LimitError::ArithmeticOverflow {
                resource: LimitKind::MemoryBytes,
            })
        ));
    }

    #[test]
    fn partition_invariant_is_enforced() {
        let limits = QecLimits {
            max_qubits: 100,
            max_qubits_per_partition: 101,
            ..QecLimits::default()
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn stream_invariant_is_enforced() {
        let limits = QecLimits {
            max_syndrome_events: 100,
            max_stream_buffer_events: 101,
            ..QecLimits::default()
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn logical_weight_invariant_is_enforced() {
        let limits = QecLimits {
            max_qubits: 100,
            max_logical_operator_weight: 101,
            ..QecLimits::default()
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn qpu_limits_are_independent() {
        let limits = QecLimits::default()
            .with_max_qpu_shots(100)
            .with_max_qpu_circuits(2);

        assert!(
            limits
                .validate_qpu(100, 2)
                .is_ok()
        );

        assert!(
            limits
                .validate_qpu(101, 2)
                .is_err()
        );

        assert!(
            limits
                .validate_qpu(100, 3)
                .is_err()
        );
    }

    #[test]
    fn verification_budget_is_independent() {
        let limits = QecLimits::default()
            .with_max_verification_operations(100);

        assert!(
            limits
                .validate_verification(100)
                .is_ok()
        );

        assert!(
            limits
                .validate_verification(101)
                .is_err()
        );
    }
}