//! Resource limits for the Quantum Error Correction subsystem.
//!
//! `QecLimits` defines explicit resource ceilings for QEC operations.
//! It exists to prevent accidental or adversarial requests from causing
//! unbounded allocation, excessive computation, or denial of service.
//!
//! The limits are intentionally independent of individual decoders.
//! Decoders, graph builders, syndrome processors, simulators, and future
//! distributed backends should consult the same policy.
//!
//! Design goals:
//! - deterministic validation;
//! - overflow-safe derived-resource calculations;
//! - zero allocation during limit checks;
//! - no panics for malformed arithmetic;
//! - configurable limits;
//! - conservative defaults;
//! - explicit opt-in for larger workloads.
//!
//! Important:
//! These limits are resource-safety boundaries, not mathematical limits on
//! the size of a QEC code. A larger code can be processed by increasing the
//! configured limits or by using streaming/partitioned/distributed execution.

use core::fmt;

/// Default maximum surface-code distance.
///
/// This is deliberately conservative. Applications processing larger codes
/// should explicitly configure a larger limit rather than accidentally
/// constructing an enormous workload.
pub const DEFAULT_MAX_CODE_DISTANCE: usize = 10_001;

/// Default maximum number of physical qubits.
pub const DEFAULT_MAX_QUBITS: usize = 100_000_000;

/// Default maximum number of stabilizers.
pub const DEFAULT_MAX_STABILIZERS: usize = 100_000_000;

/// Default maximum number of syndrome events retained by a single operation.
pub const DEFAULT_MAX_SYNDROME_EVENTS: usize = 100_000_000;

/// Default maximum number of measurement rounds.
pub const DEFAULT_MAX_ROUNDS: usize = 10_000_000;

/// Default maximum number of decoding-graph nodes.
pub const DEFAULT_MAX_GRAPH_NODES: usize = 500_000_000;

/// Default maximum number of decoding-graph edges.
pub const DEFAULT_MAX_GRAPH_EDGES: usize = 2_000_000_000;

/// Default maximum memory budget.
///
/// 64 GiB.
pub const DEFAULT_MAX_MEMORY_BYTES: u64 = 64 * 1024 * 1024 * 1024;

/// Default maximum decoder time in nanoseconds.
///
/// 24 hours.
pub const DEFAULT_MAX_DECODER_TIME_NS: u64 = 24 * 60 * 60 * 1_000_000_000;

/// Default maximum parallel workers.
pub const DEFAULT_MAX_PARALLELISM: usize = 1024;

/// Default maximum checkpoint size.
///
/// 16 GiB.
pub const DEFAULT_MAX_CHECKPOINT_SIZE_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// Default maximum partition count.
pub const DEFAULT_MAX_PARTITIONS: usize = 1_000_000;

/// Default maximum streaming buffer size.
pub const DEFAULT_MAX_STREAM_BUFFER_EVENTS: usize = 10_000_000;

/// Default maximum decoder iterations.
pub const DEFAULT_MAX_DECODER_ITERATIONS: usize = 10_000_000;

/// Default maximum stabilizer weight.
pub const DEFAULT_MAX_STABILIZER_WEIGHT: usize = 64;

/// Default maximum logical-operator weight.
pub const DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT: usize = 100_000_000;

/// Default maximum number of qubits per partition.
pub const DEFAULT_MAX_QUBITS_PER_PARTITION: usize = 10_000_000;

/// Error returned when a requested resource exceeds the configured policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LimitError {
    /// A requested code distance exceeds the configured maximum.
    CodeDistance {
        requested: usize,
        maximum: usize,
    },

    /// A requested qubit count exceeds the configured maximum.
    Qubits {
        requested: usize,
        maximum: usize,
    },

    /// A requested stabilizer count exceeds the configured maximum.
    Stabilizers {
        requested: usize,
        maximum: usize,
    },

    /// A requested syndrome-event count exceeds the configured maximum.
    SyndromeEvents {
        requested: usize,
        maximum: usize,
    },

    /// A requested number of measurement rounds exceeds the configured maximum.
    Rounds {
        requested: usize,
        maximum: usize,
    },

    /// A requested graph-node count exceeds the configured maximum.
    GraphNodes {
        requested: usize,
        maximum: usize,
    },

    /// A requested graph-edge count exceeds the configured maximum.
    GraphEdges {
        requested: usize,
        maximum: usize,
    },

    /// A requested memory allocation exceeds the configured maximum.
    MemoryBytes {
        requested: u64,
        maximum: u64,
    },

    /// A requested decoder timeout exceeds the configured maximum.
    DecoderTimeNs {
        requested: u64,
        maximum: u64,
    },

    /// A requested worker count exceeds the configured maximum.
    Parallelism {
        requested: usize,
        maximum: usize,
    },

    /// A requested checkpoint exceeds the configured maximum.
    CheckpointSizeBytes {
        requested: u64,
        maximum: u64,
    },

    /// A requested partition count exceeds the configured maximum.
    Partitions {
        requested: usize,
        maximum: usize,
    },

    /// A requested stream buffer exceeds the configured maximum.
    StreamBufferEvents {
        requested: usize,
        maximum: usize,
    },

    /// A requested decoder iteration count exceeds the configured maximum.
    DecoderIterations {
        requested: usize,
        maximum: usize,
    },

    /// A stabilizer is too large for the configured policy.
    StabilizerWeight {
        requested: usize,
        maximum: usize,
    },

    /// A logical operator is too large for the configured policy.
    LogicalOperatorWeight {
        requested: usize,
        maximum: usize,
    },

    /// A partition contains too many qubits.
    QubitsPerPartition {
        requested: usize,
        maximum: usize,
    },

    /// A derived resource calculation overflowed.
    ArithmeticOverflow {
        resource: &'static str,
    },
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeDistance { requested, maximum } => {
                write!(
                    f,
                    "code distance {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::Qubits { requested, maximum } => {
                write!(
                    f,
                    "qubit count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::Stabilizers { requested, maximum } => {
                write!(
                    f,
                    "stabilizer count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::SyndromeEvents { requested, maximum } => {
                write!(
                    f,
                    "syndrome-event count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::Rounds { requested, maximum } => {
                write!(
                    f,
                    "measurement-round count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::GraphNodes { requested, maximum } => {
                write!(
                    f,
                    "graph-node count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::GraphEdges { requested, maximum } => {
                write!(
                    f,
                    "graph-edge count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::MemoryBytes { requested, maximum } => {
                write!(
                    f,
                    "memory request {} bytes exceeds configured maximum {} bytes",
                    requested, maximum
                )
            }
            Self::DecoderTimeNs { requested, maximum } => {
                write!(
                    f,
                    "decoder time {} ns exceeds configured maximum {} ns",
                    requested, maximum
                )
            }
            Self::Parallelism { requested, maximum } => {
                write!(
                    f,
                    "parallelism {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::CheckpointSizeBytes { requested, maximum } => {
                write!(
                    f,
                    "checkpoint size {} bytes exceeds configured maximum {} bytes",
                    requested, maximum
                )
            }
            Self::Partitions { requested, maximum } => {
                write!(
                    f,
                    "partition count {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::StreamBufferEvents { requested, maximum } => {
                write!(
                    f,
                    "stream buffer size {} events exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::DecoderIterations { requested, maximum } => {
                write!(
                    f,
                    "decoder iterations {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::StabilizerWeight { requested, maximum } => {
                write!(
                    f,
                    "stabilizer weight {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::LogicalOperatorWeight { requested, maximum } => {
                write!(
                    f,
                    "logical-operator weight {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::QubitsPerPartition { requested, maximum } => {
                write!(
                    f,
                    "qubits per partition {} exceeds configured maximum {}",
                    requested, maximum
                )
            }
            Self::ArithmeticOverflow { resource } => {
                write!(f, "arithmetic overflow while calculating {}", resource)
            }
        }
    }
}

/// Resource policy for one QEC execution context.
///
/// The structure is intentionally composed entirely of scalar values so it
/// can be copied cheaply and safely between execution components.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QecLimits {
    /// Maximum supported code distance.
    pub max_code_distance: usize,

    /// Maximum number of physical qubits.
    pub max_qubits: usize,

    /// Maximum number of stabilizers.
    pub max_stabilizers: usize,

    /// Maximum number of syndrome/detection events.
    pub max_syndrome_events: usize,

    /// Maximum number of measurement rounds.
    pub max_rounds: usize,

    /// Maximum number of decoding graph nodes.
    pub max_graph_nodes: usize,

    /// Maximum number of decoding graph edges.
    pub max_graph_edges: usize,

    /// Maximum memory budget in bytes.
    pub max_memory_bytes: u64,

    /// Maximum permitted decoder time in nanoseconds.
    pub max_decoder_time_ns: u64,

    /// Maximum number of parallel workers.
    pub max_parallelism: usize,

    /// Maximum checkpoint size in bytes.
    pub max_checkpoint_size_bytes: u64,

    /// Maximum number of partitions.
    pub max_partitions: usize,

    /// Maximum buffered events in streaming execution.
    pub max_stream_buffer_events: usize,

    /// Maximum decoder iterations.
    pub max_decoder_iterations: usize,

    /// Maximum number of data qubits acted upon by one stabilizer.
    pub max_stabilizer_weight: usize,

    /// Maximum logical-operator weight.
    pub max_logical_operator_weight: usize,

    /// Maximum number of qubits assigned to one partition.
    pub max_qubits_per_partition: usize,
}

impl Default for QecLimits {
    fn default() -> Self {
        Self {
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
            max_checkpoint_size_bytes: DEFAULT_MAX_CHECKPOINT_SIZE_BYTES,
            max_partitions: DEFAULT_MAX_PARTITIONS,
            max_stream_buffer_events: DEFAULT_MAX_STREAM_BUFFER_EVENTS,
            max_decoder_iterations: DEFAULT_MAX_DECODER_ITERATIONS,
            max_stabilizer_weight: DEFAULT_MAX_STABILIZER_WEIGHT,
            max_logical_operator_weight: DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT,
            max_qubits_per_partition: DEFAULT_MAX_QUBITS_PER_PARTITION,
        }
    }
}

impl QecLimits {
    /// Creates the default production resource policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
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
            max_checkpoint_size_bytes: DEFAULT_MAX_CHECKPOINT_SIZE_BYTES,
            max_partitions: DEFAULT_MAX_PARTITIONS,
            max_stream_buffer_events: DEFAULT_MAX_STREAM_BUFFER_EVENTS,
            max_decoder_iterations: DEFAULT_MAX_DECODER_ITERATIONS,
            max_stabilizer_weight: DEFAULT_MAX_STABILIZER_WEIGHT,
            max_logical_operator_weight: DEFAULT_MAX_LOGICAL_OPERATOR_WEIGHT,
            max_qubits_per_partition: DEFAULT_MAX_QUBITS_PER_PARTITION,
        }
    }

    /// Validates the policy itself.
    ///
    /// A limit of zero is rejected for resources that must be able to support
    /// at least one item. Memory and time may also not be zero because a
    /// zero-resource execution policy cannot perform useful work.
    pub fn validate(&self) -> Result<(), LimitError> {
        if self.max_code_distance == 0 {
            return Err(LimitError::CodeDistance {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_qubits == 0 {
            return Err(LimitError::Qubits {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_stabilizers == 0 {
            return Err(LimitError::Stabilizers {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_syndrome_events == 0 {
            return Err(LimitError::SyndromeEvents {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_rounds == 0 {
            return Err(LimitError::Rounds {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_graph_nodes == 0 {
            return Err(LimitError::GraphNodes {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_graph_edges == 0 {
            return Err(LimitError::GraphEdges {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_memory_bytes == 0 {
            return Err(LimitError::MemoryBytes {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_decoder_time_ns == 0 {
            return Err(LimitError::DecoderTimeNs {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_parallelism == 0 {
            return Err(LimitError::Parallelism {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_checkpoint_size_bytes == 0 {
            return Err(LimitError::CheckpointSizeBytes {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_partitions == 0 {
            return Err(LimitError::Partitions {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_stream_buffer_events == 0 {
            return Err(LimitError::StreamBufferEvents {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_decoder_iterations == 0 {
            return Err(LimitError::DecoderIterations {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_stabilizer_weight == 0 {
            return Err(LimitError::StabilizerWeight {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_logical_operator_weight == 0 {
            return Err(LimitError::LogicalOperatorWeight {
                requested: 0,
                maximum: 0,
            });
        }

        if self.max_qubits_per_partition == 0 {
            return Err(LimitError::QubitsPerPartition {
                requested: 0,
                maximum: 0,
            });
        }

        Ok(())
    }

    /// Validates a requested code distance.
    pub fn check_code_distance(&self, distance: usize) -> Result<(), LimitError> {
        if distance > self.max_code_distance {
            Err(LimitError::CodeDistance {
                requested: distance,
                maximum: self.max_code_distance,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested qubit count.
    pub fn check_qubits(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_qubits {
            Err(LimitError::Qubits {
                requested: count,
                maximum: self.max_qubits,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested stabilizer count.
    pub fn check_stabilizers(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_stabilizers {
            Err(LimitError::Stabilizers {
                requested: count,
                maximum: self.max_stabilizers,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested syndrome-event count.
    pub fn check_syndrome_events(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_syndrome_events {
            Err(LimitError::SyndromeEvents {
                requested: count,
                maximum: self.max_syndrome_events,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested number of measurement rounds.
    pub fn check_rounds(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_rounds {
            Err(LimitError::Rounds {
                requested: count,
                maximum: self.max_rounds,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested graph-node count.
    pub fn check_graph_nodes(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_graph_nodes {
            Err(LimitError::GraphNodes {
                requested: count,
                maximum: self.max_graph_nodes,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested graph-edge count.
    pub fn check_graph_edges(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_graph_edges {
            Err(LimitError::GraphEdges {
                requested: count,
                maximum: self.max_graph_edges,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested memory allocation.
    pub fn check_memory_bytes(&self, bytes: u64) -> Result<(), LimitError> {
        if bytes > self.max_memory_bytes {
            Err(LimitError::MemoryBytes {
                requested: bytes,
                maximum: self.max_memory_bytes,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested decoder duration.
    pub fn check_decoder_time_ns(&self, nanoseconds: u64) -> Result<(), LimitError> {
        if nanoseconds > self.max_decoder_time_ns {
            Err(LimitError::DecoderTimeNs {
                requested: nanoseconds,
                maximum: self.max_decoder_time_ns,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested worker count.
    pub fn check_parallelism(&self, workers: usize) -> Result<(), LimitError> {
        if workers > self.max_parallelism {
            Err(LimitError::Parallelism {
                requested: workers,
                maximum: self.max_parallelism,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested checkpoint size.
    pub fn check_checkpoint_size_bytes(&self, bytes: u64) -> Result<(), LimitError> {
        if bytes > self.max_checkpoint_size_bytes {
            Err(LimitError::CheckpointSizeBytes {
                requested: bytes,
                maximum: self.max_checkpoint_size_bytes,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested partition count.
    pub fn check_partitions(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_partitions {
            Err(LimitError::Partitions {
                requested: count,
                maximum: self.max_partitions,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested stream-buffer size.
    pub fn check_stream_buffer_events(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_stream_buffer_events {
            Err(LimitError::StreamBufferEvents {
                requested: count,
                maximum: self.max_stream_buffer_events,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a requested decoder iteration count.
    pub fn check_decoder_iterations(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_decoder_iterations {
            Err(LimitError::DecoderIterations {
                requested: count,
                maximum: self.max_decoder_iterations,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a stabilizer's support size.
    pub fn check_stabilizer_weight(&self, weight: usize) -> Result<(), LimitError> {
        if weight > self.max_stabilizer_weight {
            Err(LimitError::StabilizerWeight {
                requested: weight,
                maximum: self.max_stabilizer_weight,
            })
        } else {
            Ok(())
        }
    }

    /// Validates a logical operator's support size.
    pub fn check_logical_operator_weight(&self, weight: usize) -> Result<(), LimitError> {
        if weight > self.max_logical_operator_weight {
            Err(LimitError::LogicalOperatorWeight {
                requested: weight,
                maximum: self.max_logical_operator_weight,
            })
        } else {
            Ok(())
        }
    }

    /// Validates the size of one partition.
    pub fn check_qubits_per_partition(&self, count: usize) -> Result<(), LimitError> {
        if count > self.max_qubits_per_partition {
            Err(LimitError::QubitsPerPartition {
                requested: count,
                maximum: self.max_qubits_per_partition,
            })
        } else {
            Ok(())
        }
    }

    /// Safely calculates `a * b` and checks the result against a resource
    /// limit.
    pub fn checked_product(
        &self,
        a: usize,
        b: usize,
        resource: &'static str,
    ) -> Result<usize, LimitError> {
        let value = a
            .checked_mul(b)
            .ok_or(LimitError::ArithmeticOverflow { resource })?;

        Ok(value)
    }

    /// Safely calculates `a + b` and checks the result against a resource
    /// limit.
    pub fn checked_sum(
        &self,
        a: usize,
        b: usize,
        resource: &'static str,
    ) -> Result<usize, LimitError> {
        a.checked_add(b)
            .ok_or(LimitError::ArithmeticOverflow { resource })
    }

    /// Calculates the square of a distance without overflowing `usize`.
    pub fn checked_distance_square(&self, distance: usize) -> Result<usize, LimitError> {
        self.checked_product(distance, distance, "distance squared")
    }

    /// Calculates a conservative upper bound for the number of data qubits
    /// in a square lattice.
    ///
    /// This helper intentionally performs only arithmetic. The exact mapping
    /// between distance and qubit count belongs to `surface_code.rs`.
    pub fn checked_square_lattice_qubits(
        &self,
        distance: usize,
    ) -> Result<usize, LimitError> {
        self.check_code_distance(distance)?;

        let count = self.checked_distance_square(distance)?;

        self.check_qubits(count)?;
        Ok(count)
    }

    /// Calculates a conservative upper bound for graph edges from a node
    /// count and maximum degree.
    pub fn checked_graph_edges_from_degree(
        &self,
        nodes: usize,
        max_degree: usize,
    ) -> Result<usize, LimitError> {
        self.check_graph_nodes(nodes)?;

        let directed_edges = self.checked_product(nodes, max_degree, "graph edge count")?;

        let edges = directed_edges
            .checked_add(1)
            .ok_or(LimitError::ArithmeticOverflow {
                resource: "graph edge count",
            })?
            / 2;

        self.check_graph_edges(edges)?;
        Ok(edges)
    }

    /// Returns whether the requested memory is permitted.
    #[must_use]
    pub const fn allows_memory(&self, bytes: u64) -> bool {
        bytes <= self.max_memory_bytes
    }

    /// Returns whether the requested graph size is permitted.
    #[must_use]
    pub const fn allows_graph(&self, nodes: usize, edges: usize) -> bool {
        nodes <= self.max_graph_nodes && edges <= self.max_graph_edges
    }

    /// Returns whether the requested code size is permitted.
    #[must_use]
    pub const fn allows_code(
        &self,
        distance: usize,
        qubits: usize,
        stabilizers: usize,
    ) -> bool {
        distance <= self.max_code_distance
            && qubits <= self.max_qubits
            && stabilizers <= self.max_stabilizers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid() {
        assert!(QecLimits::default().validate().is_ok());
    }

    #[test]
    fn rejects_code_distance_above_limit() {
        let limits = QecLimits {
            max_code_distance: 5,
            ..QecLimits::default()
        };

        assert_eq!(
            limits.check_code_distance(6),
            Err(LimitError::CodeDistance {
                requested: 6,
                maximum: 5
            })
        );
    }

    #[test]
    fn accepts_values_at_the_limit() {
        let limits = QecLimits::default();

        assert!(limits.check_qubits(limits.max_qubits).is_ok());
        assert!(limits.check_stabilizers(limits.max_stabilizers).is_ok());
        assert!(limits.check_graph_nodes(limits.max_graph_nodes).is_ok());
        assert!(limits.check_graph_edges(limits.max_graph_edges).is_ok());
    }

    #[test]
    fn rejects_values_above_the_limit() {
        let limits = QecLimits::default();

        assert!(limits.check_qubits(limits.max_qubits + 1).is_err());
        assert!(
            limits
                .check_stabilizers(limits.max_stabilizers + 1)
                .is_err()
        );
        assert!(
            limits
                .check_graph_nodes(limits.max_graph_nodes + 1)
                .is_err()
        );
        assert!(
            limits
                .check_graph_edges(limits.max_graph_edges + 1)
                .is_err()
        );
    }

    #[test]
    fn detects_multiplication_overflow() {
        let limits = QecLimits::default();

        let result = limits.checked_product(usize::MAX, 2, "test");

        assert_eq!(
            result,
            Err(LimitError::ArithmeticOverflow {
                resource: "test"
            })
        );
    }

    #[test]
    fn detects_addition_overflow() {
        let limits = QecLimits::default();

        let result = limits.checked_sum(usize::MAX, 1, "test");

        assert_eq!(
            result,
            Err(LimitError::ArithmeticOverflow {
                resource: "test"
            })
        );
    }

    #[test]
    fn distance_square_is_checked() {
        let limits = QecLimits::default();

        assert_eq!(limits.checked_distance_square(10), Ok(100));
    }

    #[test]
    fn memory_limit_is_enforced() {
        let limits = QecLimits {
            max_memory_bytes: 1024,
            ..QecLimits::default()
        };

        assert!(limits.check_memory_bytes(1024).is_ok());
        assert!(limits.check_memory_bytes(1025).is_err());
        assert!(limits.allows_memory(1024));
        assert!(!limits.allows_memory(1025));
    }

    #[test]
    fn graph_limits_are_checked_together() {
        let limits = QecLimits {
            max_graph_nodes: 100,
            max_graph_edges: 200,
            ..QecLimits::default()
        };

        assert!(limits.allows_graph(100, 200));
        assert!(!limits.allows_graph(101, 200));
        assert!(!limits.allows_graph(100, 201));
    }

    #[test]
    fn code_limits_are_checked_together() {
        let limits = QecLimits {
            max_code_distance: 9,
            max_qubits: 100,
            max_stabilizers: 100,
            ..QecLimits::default()
        };

        assert!(limits.allows_code(9, 100, 100));
        assert!(!limits.allows_code(10, 100, 100));
        assert!(!limits.allows_code(9, 101, 100));
        assert!(!limits.allows_code(9, 100, 101));
    }

    #[test]
    fn policy_can_be_used_without_allocations() {
        let limits = QecLimits::new();

        assert!(limits.check_code_distance(3).is_ok());
        assert!(limits.check_qubits(9).is_ok());
        assert!(limits.check_stabilizers(8).is_ok());
    }

    #[test]
    fn display_is_deterministic() {
        let error = LimitError::Qubits {
            requested: 101,
            maximum: 100,
        };

        assert_eq!(
            error.to_string(),
            "qubit count 101 exceeds configured maximum 100"
        );
    }
}