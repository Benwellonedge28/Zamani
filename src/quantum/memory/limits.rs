//! Zamani Quantum Memory — Resource Limits
//!
//! This module defines the immutable, representation-independent resource
//! policy for `quantum::memory`.
//!
//! # Architectural position
//!
//! `quantum::memory::limits` is a foundational policy module.
//!
//! It must not depend on:
//!
//! - `quantum::memory::types`;
//! - `quantum::memory::representation`;
//! - `quantum::memory::allocator`;
//! - `quantum::memory::state`;
//! - `quantum::memory::state_vector`;
//! - `quantum::memory::density_matrix`;
//! - `quantum::memory::tensor_network`;
//! - `quantum::memory::gpu`;
//! - `quantum::memory::distributed`;
//! - `quantum::ir`;
//! - `quantum::hardware`;
//! - runtime;
//! - benchmarking.
//!
//! Later modules consume this module's contracts.
//!
//! # Purpose
//!
//! Quantum memory has fundamentally different scaling characteristics from
//! ordinary application memory. In particular:
//!
//! ```text
//! state vector:
//!     amplitudes = 2^n
//!
//! density matrix:
//!     complex elements = 4^n
//!
//! ```
//!
//! Therefore memory safety cannot be implemented by merely checking whether a
//! requested `Vec` allocation succeeds. The request must be estimated and
//! validated *before* allocation.
//!
//! This module provides:
//!
//! - immutable hard resource limits;
//! - deterministic configuration validation;
//! - checked quantum-state memory estimation;
//! - representation-independent memory requirements;
//! - resource-limit violation reporting;
//! - checked powers and multiplication;
//! - host/device/distributed memory limits;
//! - snapshot/checkpoint limits;
//! - tensor-network limits;
//! - allocation-count limits;
//! - temporary/persistent memory limits;
//! - conversion-safe byte estimates.
//!
//! # Important distinction
//!
//! `MemoryLimits` describes what a memory subsystem is *allowed* to consume.
//!
//! It does not track current consumption.
//!
//! Accounting belongs to later modules:
//!
//! ```text
//! limits.rs
//!     │
//!     ├── allocator.rs
//!     ├── budget.rs
//!     ├── reservation.rs
//!     ├── pool.rs
//!     └── state implementations
//! ```
//!
//! This separation is intentional.
//!
//! # No unsafe
//!
//! This module contains no `unsafe` code and exposes no raw pointers.
//!
//! # No hidden unlimited mode
//!
//! There is deliberately no `usize::MAX` or `u64::MAX` sentinel meaning
//! "unlimited". An explicit configured limit is always required.
//!
//! # Integer model
//!
//! Public limits use `u64` rather than `usize` so that:
//!
//! - policies are platform-independent;
//! - serialized policies have stable widths;
//! - 32-bit and 64-bit targets use the same policy representation;
//! - overflow checks are explicit;
//! - later allocators can safely convert to `usize` only after validation.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - no nightly features;
//! - no unsafe.
//!
//! # Integration contract
//!
//! Later modules must follow these rules:
//!
//! 1. Never calculate `2^n` using an unchecked shift.
//! 2. Never calculate dense state-vector bytes using unchecked multiplication.
//! 3. Never allocate before checking a `MemoryRequirement`.
//! 4. Never introduce a second memory-limit policy.
//! 5. Never use `u64::MAX` as an unlimited sentinel.
//! 6. Never silently clamp an invalid request.
//! 7. Never silently downgrade a representation because its requested memory
//!    exceeds policy.
//! 8. Explicitly choose whether an allocation is temporary or persistent.
//! 9. GPU/device and distributed allocations must be checked independently.
//! 10. Snapshot and checkpoint sizes must be checked before serialization.
//! 11. Representation implementations may impose stricter limits, but may not
//!     bypass these limits.
//! 12. This module does not perform allocation, I/O, logging, telemetry, or
//!     benchmarking.

use core::fmt;

// =============================================================================
// Production defaults
// =============================================================================

/// Default maximum number of logical/physical qubits represented by the
/// memory subsystem.
///
/// This does not imply that a dense state vector of this size can be
/// allocated. Dense representations are additionally bounded by byte and
/// amplitude limits.
pub const DEFAULT_MAX_QUBITS: u64 = 4096;

/// Default maximum number of classical bits associated with quantum memory.
pub const DEFAULT_MAX_CLASSICAL_BITS: u64 = 1_000_000;

/// Default maximum number of independently tracked allocations.
pub const DEFAULT_MAX_ALLOCATIONS: u64 = 1_000_000;

/// Default maximum total host memory managed by one memory domain.
///
/// 64 GiB.
pub const DEFAULT_MAX_HOST_BYTES: u64 = 64 * 1024 * 1024 * 1024;

/// Default maximum temporary host memory.
///
/// 16 GiB.
pub const DEFAULT_MAX_TEMPORARY_HOST_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// Default maximum persistent host memory.
///
/// 64 GiB.
pub const DEFAULT_MAX_PERSISTENT_HOST_BYTES: u64 = 64 * 1024 * 1024 * 1024;

/// Default maximum pinned-host memory.
///
/// 16 GiB.
pub const DEFAULT_MAX_PINNED_HOST_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// Default maximum device/GPU memory managed by one memory domain.
///
/// 64 GiB.
pub const DEFAULT_MAX_DEVICE_BYTES: u64 = 64 * 1024 * 1024 * 1024;

/// Default maximum temporary device/GPU memory.
///
/// 16 GiB.
pub const DEFAULT_MAX_TEMPORARY_DEVICE_BYTES: u64 =
    16 * 1024 * 1024 * 1024;

/// Default maximum distributed memory represented by one memory domain.
///
/// 1 TiB.
pub const DEFAULT_MAX_DISTRIBUTED_BYTES: u64 =
    1024 * 1024 * 1024 * 1024;

/// Default maximum bytes in one quantum-state allocation.
///
/// 64 GiB.
pub const DEFAULT_MAX_STATE_BYTES: u64 =
    64 * 1024 * 1024 * 1024;

/// Default maximum bytes in one temporary state allocation.
///
/// 16 GiB.
pub const DEFAULT_MAX_TEMPORARY_STATE_BYTES: u64 =
    16 * 1024 * 1024 * 1024;

/// Default maximum number of state amplitudes/elements.
pub const DEFAULT_MAX_STATE_ELEMENTS: u64 = 1u64 << 36;

/// Default maximum snapshot size.
///
/// 64 GiB.
pub const DEFAULT_MAX_SNAPSHOT_BYTES: u64 =
    64 * 1024 * 1024 * 1024;

/// Default maximum checkpoint size.
///
/// 64 GiB.
pub const DEFAULT_MAX_CHECKPOINT_BYTES: u64 =
    64 * 1024 * 1024 * 1024;

/// Default maximum tensor rank.
pub const DEFAULT_MAX_TENSOR_RANK: u64 = 64;

/// Default maximum tensor dimension.
///
/// This is a per-dimension bound, not the total tensor element count.
pub const DEFAULT_MAX_TENSOR_DIMENSION: u64 = 1_000_000;

/// Default maximum tensor-network bond dimension.
pub const DEFAULT_MAX_BOND_DIMENSION: u64 = 65_536;

/// Default maximum number of tensors in one tensor network.
pub const DEFAULT_MAX_TENSORS: u64 = 1_000_000;

/// Default maximum number of distributed partitions.
pub const DEFAULT_MAX_DISTRIBUTED_PARTITIONS: u64 = 1_000_000;

/// Default maximum bytes in one distributed partition.
pub const DEFAULT_MAX_DISTRIBUTED_PARTITION_BYTES: u64 =
    64 * 1024 * 1024 * 1024;

/// Default maximum number of qubits per dense state-vector allocation.
///
/// This is intentionally separate from `max_qubits`. A stabilizer
/// representation may support thousands of qubits while a dense state vector
/// cannot reasonably do so.
pub const DEFAULT_MAX_STATE_VECTOR_QUBITS: u64 = 32;

/// Default maximum qubits for density-matrix allocation.
///
/// Density matrices scale as 4^n complex elements and therefore require a much
/// smaller practical bound.
pub const DEFAULT_MAX_DENSITY_MATRIX_QUBITS: u64 = 16;

/// Default maximum qubits for one tensor-network state before representation
/// specific bond-dimension constraints are applied.
pub const DEFAULT_MAX_TENSOR_NETWORK_QUBITS: u64 = 4096;

/// Default maximum number of measurement/classical-result entries retained by
/// one memory object.
pub const DEFAULT_MAX_MEASUREMENT_RESULTS: u64 = 1_000_000;

/// Default maximum serialized metadata bytes.
pub const DEFAULT_MAX_METADATA_BYTES: u64 = 16 * 1024 * 1024;

/// Default maximum deterministic work units permitted for memory planning.
///
/// This is not a wall-clock timeout.
pub const DEFAULT_MAX_PLANNING_WORK: u64 = 100_000_000;

// =============================================================================
// Limit identity
// =============================================================================

/// Stable identity for an independently enforceable memory limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MemoryLimitKind {
    /// Maximum logical/physical qubit count.
    Qubits,

    /// Maximum classical-bit count.
    ClassicalBits,

    /// Maximum allocation count.
    Allocations,

    /// Maximum host bytes.
    HostBytes,

    /// Maximum temporary host bytes.
    TemporaryHostBytes,

    /// Maximum persistent host bytes.
    PersistentHostBytes,

    /// Maximum pinned host bytes.
    PinnedHostBytes,

    /// Maximum device/GPU bytes.
    DeviceBytes,

    /// Maximum temporary device/GPU bytes.
    TemporaryDeviceBytes,

    /// Maximum distributed bytes.
    DistributedBytes,

    /// Maximum bytes in one state allocation.
    StateBytes,

    /// Maximum temporary state bytes.
    TemporaryStateBytes,

    /// Maximum number of state elements.
    StateElements,

    /// Maximum snapshot size.
    SnapshotBytes,

    /// Maximum checkpoint size.
    CheckpointBytes,

    /// Maximum tensor rank.
    TensorRank,

    /// Maximum dimension of one tensor axis.
    TensorDimension,

    /// Maximum tensor-network bond dimension.
    BondDimension,

    /// Maximum number of tensors.
    Tensors,

    /// Maximum number of distributed partitions.
    DistributedPartitions,

    /// Maximum bytes per distributed partition.
    DistributedPartitionBytes,

    /// Maximum state-vector qubit count.
    StateVectorQubits,

    /// Maximum density-matrix qubit count.
    DensityMatrixQubits,

    /// Maximum tensor-network qubit count.
    TensorNetworkQubits,

    /// Maximum measurement-result count.
    MeasurementResults,

    /// Maximum metadata bytes.
    MetadataBytes,

    /// Maximum planning work.
    PlanningWork,
}

impl fmt::Display for MemoryLimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Qubits => "qubits",
            Self::ClassicalBits => "classical-bits",
            Self::Allocations => "allocations",
            Self::HostBytes => "host-bytes",
            Self::TemporaryHostBytes => "temporary-host-bytes",
            Self::PersistentHostBytes => "persistent-host-bytes",
            Self::PinnedHostBytes => "pinned-host-bytes",
            Self::DeviceBytes => "device-bytes",
            Self::TemporaryDeviceBytes => "temporary-device-bytes",
            Self::DistributedBytes => "distributed-bytes",
            Self::StateBytes => "state-bytes",
            Self::TemporaryStateBytes => "temporary-state-bytes",
            Self::StateElements => "state-elements",
            Self::SnapshotBytes => "snapshot-bytes",
            Self::CheckpointBytes => "checkpoint-bytes",
            Self::TensorRank => "tensor-rank",
            Self::TensorDimension => "tensor-dimension",
            Self::BondDimension => "bond-dimension",
            Self::Tensors => "tensors",
            Self::DistributedPartitions => "distributed-partitions",
            Self::DistributedPartitionBytes => "distributed-partition-bytes",
            Self::StateVectorQubits => "state-vector-qubits",
            Self::DensityMatrixQubits => "density-matrix-qubits",
            Self::TensorNetworkQubits => "tensor-network-qubits",
            Self::MeasurementResults => "measurement-results",
            Self::MetadataBytes => "metadata-bytes",
            Self::PlanningWork => "planning-work",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Configuration errors
// =============================================================================

/// Error produced by an invalid `MemoryLimits` configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MemoryLimitConfigError {
    /// A required non-zero limit was configured as zero.
    ZeroRequiredLimit {
        /// Stable field name.
        field: &'static str,
    },

    /// Temporary host memory exceeds total host memory.
    TemporaryHostExceedsHost {
        temporary: u64,
        host: u64,
    },

    /// Persistent host memory exceeds total host memory.
    PersistentHostExceedsHost {
        persistent: u64,
        host: u64,
    },

    /// Temporary device memory exceeds total device memory.
    TemporaryDeviceExceedsDevice {
        temporary: u64,
        device: u64,
    },

    /// Temporary state memory exceeds total state memory.
    TemporaryStateExceedsState {
        temporary: u64,
        state: u64,
    },

    /// State memory exceeds host memory.
    StateExceedsHost {
        state: u64,
        host: u64,
    },

    /// State-vector qubit policy exceeds the global qubit policy.
    StateVectorQubitsExceedGlobal {
        state_vector: u64,
        qubits: u64,
    },

    /// Density-matrix qubit policy exceeds the global qubit policy.
    DensityMatrixQubitsExceedGlobal {
        density_matrix: u64,
        qubits: u64,
    },

    /// Tensor-network qubit policy exceeds the global qubit policy.
    TensorNetworkQubitsExceedGlobal {
        tensor_network: u64,
        qubits: u64,
    },

    /// Distributed partition bytes exceeds distributed capacity.
    PartitionExceedsDistributed {
        partition: u64,
        distributed: u64,
    },

    /// A tensor dimension exceeds the total tensor element safety boundary.
    TensorDimensionExceedsElementLimit {
        dimension: u64,
        max_elements: u64,
    },
}

impl fmt::Display for MemoryLimitConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroRequiredLimit { field } => {
                write!(f, "memory limit `{field}` must be greater than zero")
            }

            Self::TemporaryHostExceedsHost {
                temporary,
                host,
            } => {
                write!(
                    f,
                    "temporary host memory limit {temporary} exceeds \
                     host memory limit {host}"
                )
            }

            Self::PersistentHostExceedsHost {
                persistent,
                host,
            } => {
                write!(
                    f,
                    "persistent host memory limit {persistent} exceeds \
                     host memory limit {host}"
                )
            }

            Self::TemporaryDeviceExceedsDevice {
                temporary,
                device,
            } => {
                write!(
                    f,
                    "temporary device memory limit {temporary} exceeds \
                     device memory limit {device}"
                )
            }

            Self::TemporaryStateExceedsState {
                temporary,
                state,
            } => {
                write!(
                    f,
                    "temporary state memory limit {temporary} exceeds \
                     state memory limit {state}"
                )
            }

            Self::StateExceedsHost { state, host } => {
                write!(
                    f,
                    "state memory limit {state} exceeds host memory limit {host}"
                )
            }

            Self::StateVectorQubitsExceedGlobal {
                state_vector,
                qubits,
            } => {
                write!(
                    f,
                    "state-vector qubit limit {state_vector} exceeds \
                     global qubit limit {qubits}"
                )
            }

            Self::DensityMatrixQubitsExceedGlobal {
                density_matrix,
                qubits,
            } => {
                write!(
                    f,
                    "density-matrix qubit limit {density_matrix} exceeds \
                     global qubit limit {qubits}"
                )
            }

            Self::TensorNetworkQubitsExceedGlobal {
                tensor_network,
                qubits,
            } => {
                write!(
                    f,
                    "tensor-network qubit limit {tensor_network} exceeds \
                     global qubit limit {qubits}"
                )
            }

            Self::PartitionExceedsDistributed {
                partition,
                distributed,
            } => {
                write!(
                    f,
                    "distributed partition limit {partition} exceeds \
                     distributed memory limit {distributed}"
                )
            }

            Self::TensorDimensionExceedsElementLimit {
                dimension,
                max_elements,
            } => {
                write!(
                    f,
                    "tensor dimension {dimension} exceeds configured \
                     element safety limit {max_elements}"
                )
            }
        }
    }
}

impl std::error::Error for MemoryLimitConfigError {}

// =============================================================================
// Runtime violations
// =============================================================================

/// A checked memory resource-limit violation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MemoryLimitViolation {
    kind: MemoryLimitKind,
    requested: u64,
    maximum: u64,
}

impl MemoryLimitViolation {
    /// Creates a new resource-limit violation.
    #[must_use]
    pub const fn new(
        kind: MemoryLimitKind,
        requested: u64,
        maximum: u64,
    ) -> Self {
        Self {
            kind,
            requested,
            maximum,
        }
    }

    /// Returns the violated limit.
    #[must_use]
    pub const fn kind(self) -> MemoryLimitKind {
        self.kind
    }

    /// Returns the requested amount.
    #[must_use]
    pub const fn requested(self) -> u64 {
        self.requested
    }

    /// Returns the configured maximum.
    #[must_use]
    pub const fn maximum(self) -> u64 {
        self.maximum
    }
}

impl fmt::Display for MemoryLimitViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "quantum memory limit `{}` exceeded: requested {}, maximum {}",
            self.kind, self.requested, self.maximum
        )
    }
}

impl std::error::Error for MemoryLimitViolation {}

// =============================================================================
// Estimation errors
// =============================================================================

/// Error produced while calculating a memory requirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MemoryEstimateError {
    /// A checked power operation overflowed `u64`.
    ExponentOverflow {
        /// Base of the power operation.
        base: u64,

        /// Exponent of the power operation.
        exponent: u64,
    },

    /// Element-count multiplication overflowed `u64`.
    ElementCountOverflow,

    /// Byte-count multiplication overflowed `u64`.
    ByteCountOverflow,

    /// A byte count cannot be represented by the target platform's `usize`.
    PlatformSizeOverflow {
        /// Number of bytes that must be represented.
        bytes: u64,
    },

    /// Requested tensor rank is invalid.
    InvalidTensorRank,

    /// Requested tensor dimension is invalid.
    InvalidTensorDimension,

    /// Requested bond dimension is invalid.
    InvalidBondDimension,
}

impl fmt::Display for MemoryEstimateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExponentOverflow { base, exponent } => {
                write!(
                    f,
                    "memory estimate overflow: {base}^{exponent} \
                     cannot be represented"
                )
            }

            Self::ElementCountOverflow => {
                f.write_str("memory element-count calculation overflowed")
            }

            Self::ByteCountOverflow => {
                f.write_str("memory byte-count calculation overflowed")
            }

            Self::PlatformSizeOverflow { bytes } => {
                write!(
                    f,
                    "memory requirement of {bytes} bytes cannot be \
                     represented by the target platform"
                )
            }

            Self::InvalidTensorRank => {
                f.write_str("tensor rank must be greater than zero")
            }

            Self::InvalidTensorDimension => {
                f.write_str("tensor dimension must be greater than zero")
            }

            Self::InvalidBondDimension => {
                f.write_str("tensor-network bond dimension must be greater \
                             than zero")
            }
        }
    }
}

impl std::error::Error for MemoryEstimateError {}

// =============================================================================
// Memory estimate
// =============================================================================

/// Result of a checked memory-size calculation.
///
/// This is deliberately independent of any concrete state representation.
///
/// `elements` means the number of scalar/complex elements required by the
/// representation, not the number of bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MemoryEstimate {
    /// Number of logical qubits involved.
    qubits: u64,

    /// Number of mathematical state elements.
    elements: u64,

    /// Bytes required when each element occupies `bytes_per_element`.
    bytes: u64,

    /// Bytes per state element.
    bytes_per_element: u64,
}

impl MemoryEstimate {
    /// Creates a validated memory estimate.
    pub const fn new(
        qubits: u64,
        elements: u64,
        bytes_per_element: u64,
    ) -> Result<Self, MemoryEstimateError> {
        match elements.checked_mul(bytes_per_element) {
            Some(bytes) => Ok(Self {
                qubits,
                elements,
                bytes,
                bytes_per_element,
            }),
            None => Err(MemoryEstimateError::ByteCountOverflow),
        }
    }

    /// Number of logical qubits.
    #[must_use]
    pub const fn qubits(self) -> u64 {
        self.qubits
    }

    /// Number of state elements.
    #[must_use]
    pub const fn elements(self) -> u64 {
        self.elements
    }

    /// Required bytes.
    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.bytes
    }

    /// Bytes per element.
    #[must_use]
    pub const fn bytes_per_element(self) -> u64 {
        self.bytes_per_element
    }

    /// Returns whether the byte count can be represented as `usize`.
    #[must_use]
    pub fn fits_platform_usize(self) -> bool {
        usize::try_from(self.bytes).is_ok()
    }

    /// Converts the byte count to `usize` after an explicit checked test.
    pub fn bytes_as_usize(self) -> Result<usize, MemoryEstimateError> {
        usize::try_from(self.bytes)
            .map_err(|_| MemoryEstimateError::PlatformSizeOverflow {
                bytes: self.bytes,
            })
    }
}

// =============================================================================
// Generic memory requirement
// =============================================================================

/// Representation-independent memory requirement.
///
/// Later allocation and reservation modules use this as the boundary object
/// between planning and allocation.
///
/// A requirement may describe:
//!
//! - logical qubits;
//! - classical bits;
//! - host memory;
//! - device memory;
//! - distributed memory;
//! - temporary memory;
//! - persistent memory;
//! - state elements;
//! - allocations;
//! - tensor-network resources;
//! - snapshots/checkpoints.
///
/// Zero means "not requested" for an individual category.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct MemoryRequirement {
    qubits: u64,
    classical_bits: u64,
    allocations: u64,

    host_bytes: u64,
    temporary_host_bytes: u64,
    persistent_host_bytes: u64,
    pinned_host_bytes: u64,

    device_bytes: u64,
    temporary_device_bytes: u64,

    distributed_bytes: u64,
    distributed_partitions: u64,
    distributed_partition_bytes: u64,

    state_bytes: u64,
    temporary_state_bytes: u64,
    state_elements: u64,

    snapshot_bytes: u64,
    checkpoint_bytes: u64,

    tensor_rank: u64,
    tensor_dimension: u64,
    bond_dimension: u64,
    tensors: u64,

    measurement_results: u64,
    metadata_bytes: u64,
    planning_work: u64,
}

impl MemoryRequirement {
    /// Creates an empty requirement.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            qubits: 0,
            classical_bits: 0,
            allocations: 0,
            host_bytes: 0,
            temporary_host_bytes: 0,
            persistent_host_bytes: 0,
            pinned_host_bytes: 0,
            device_bytes: 0,
            temporary_device_bytes: 0,
            distributed_bytes: 0,
            distributed_partitions: 0,
            distributed_partition_bytes: 0,
            state_bytes: 0,
            temporary_state_bytes: 0,
            state_elements: 0,
            snapshot_bytes: 0,
            checkpoint_bytes: 0,
            tensor_rank: 0,
            tensor_dimension: 0,
            bond_dimension: 0,
            tensors: 0,
            measurement_results: 0,
            metadata_bytes: 0,
            planning_work: 0,
        }
    }

    /// Sets the logical qubit requirement.
    #[must_use]
    pub const fn with_qubits(mut self, value: u64) -> Self {
        self.qubits = value;
        self
    }

    /// Sets the classical-bit requirement.
    #[must_use]
    pub const fn with_classical_bits(mut self, value: u64) -> Self {
        self.classical_bits = value;
        self
    }

    /// Sets the allocation-count requirement.
    #[must_use]
    pub const fn with_allocations(mut self, value: u64) -> Self {
        self.allocations = value;
        self
    }

    /// Sets host-memory requirement.
    #[must_use]
    pub const fn with_host_bytes(mut self, value: u64) -> Self {
        self.host_bytes = value;
        self
    }

    /// Sets temporary host-memory requirement.
    #[must_use]
    pub const fn with_temporary_host_bytes(mut self, value: u64) -> Self {
        self.temporary_host_bytes = value;
        self
    }

    /// Sets persistent host-memory requirement.
    #[must_use]
    pub const fn with_persistent_host_bytes(mut self, value: u64) -> Self {
        self.persistent_host_bytes = value;
        self
    }

    /// Sets pinned host-memory requirement.
    #[must_use]
    pub const fn with_pinned_host_bytes(mut self, value: u64) -> Self {
        self.pinned_host_bytes = value;
        self
    }

    /// Sets device-memory requirement.
    #[must_use]
    pub const fn with_device_bytes(mut self, value: u64) -> Self {
        self.device_bytes = value;
        self
    }

    /// Sets temporary device-memory requirement.
    #[must_use]
    pub const fn with_temporary_device_bytes(mut self, value: u64) -> Self {
        self.temporary_device_bytes = value;
        self
    }

    /// Sets distributed-memory requirement.
    #[must_use]
    pub const fn with_distributed_bytes(mut self, value: u64) -> Self {
        self.distributed_bytes = value;
        self
    }

    /// Sets distributed partition count.
    #[must_use]
    pub const fn with_distributed_partitions(mut self, value: u64) -> Self {
        self.distributed_partitions = value;
        self
    }

    /// Sets distributed partition size.
    #[must_use]
    pub const fn with_distributed_partition_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.distributed_partition_bytes = value;
        self
    }

    /// Sets state-memory requirement.
    #[must_use]
    pub const fn with_state_bytes(mut self, value: u64) -> Self {
        self.state_bytes = value;
        self
    }

    /// Sets temporary state-memory requirement.
    #[must_use]
    pub const fn with_temporary_state_bytes(mut self, value: u64) -> Self {
        self.temporary_state_bytes = value;
        self
    }

    /// Sets state-element requirement.
    #[must_use]
    pub const fn with_state_elements(mut self, value: u64) -> Self {
        self.state_elements = value;
        self
    }

    /// Sets snapshot size.
    #[must_use]
    pub const fn with_snapshot_bytes(mut self, value: u64) -> Self {
        self.snapshot_bytes = value;
        self
    }

    /// Sets checkpoint size.
    #[must_use]
    pub const fn with_checkpoint_bytes(mut self, value: u64) -> Self {
        self.checkpoint_bytes = value;
        self
    }

    /// Sets tensor rank.
    #[must_use]
    pub const fn with_tensor_rank(mut self, value: u64) -> Self {
        self.tensor_rank = value;
        self
    }

    /// Sets tensor dimension.
    #[must_use]
    pub const fn with_tensor_dimension(mut self, value: u64) -> Self {
        self.tensor_dimension = value;
        self
    }

    /// Sets tensor-network bond dimension.
    #[must_use]
    pub const fn with_bond_dimension(mut self, value: u64) -> Self {
        self.bond_dimension = value;
        self
    }

    /// Sets tensor count.
    #[must_use]
    pub const fn with_tensors(mut self, value: u64) -> Self {
        self.tensors = value;
        self
    }

    /// Sets measurement-result requirement.
    #[must_use]
    pub const fn with_measurement_results(mut self, value: u64) -> Self {
        self.measurement_results = value;
        self
    }

    /// Sets metadata size.
    #[must_use]
    pub const fn with_metadata_bytes(mut self, value: u64) -> Self {
        self.metadata_bytes = value;
        self
    }

    /// Sets planning work.
    #[must_use]
    pub const fn with_planning_work(mut self, value: u64) -> Self {
        self.planning_work = value;
        self
    }

    /// Returns logical qubits.
    #[must_use]
    pub const fn qubits(self) -> u64 {
        self.qubits
    }

    /// Returns classical bits.
    #[must_use]
    pub const fn classical_bits(self) -> u64 {
        self.classical_bits
    }

    /// Returns allocation count.
    #[must_use]
    pub const fn allocations(self) -> u64 {
        self.allocations
    }

    /// Returns host bytes.
    #[must_use]
    pub const fn host_bytes(self) -> u64 {
        self.host_bytes
    }

    /// Returns temporary host bytes.
    #[must_use]
    pub const fn temporary_host_bytes(self) -> u64 {
        self.temporary_host_bytes
    }

    /// Returns persistent host bytes.
    #[must_use]
    pub const fn persistent_host_bytes(self) -> u64 {
        self.persistent_host_bytes
    }

    /// Returns pinned host bytes.
    #[must_use]
    pub const fn pinned_host_bytes(self) -> u64 {
        self.pinned_host_bytes
    }

    /// Returns device bytes.
    #[must_use]
    pub const fn device_bytes(self) -> u64 {
        self.device_bytes
    }

    /// Returns temporary device bytes.
    #[must_use]
    pub const fn temporary_device_bytes(self) -> u64 {
        self.temporary_device_bytes
    }

    /// Returns distributed bytes.
    #[must_use]
    pub const fn distributed_bytes(self) -> u64 {
        self.distributed_bytes
    }

    /// Returns distributed partitions.
    #[must_use]
    pub const fn distributed_partitions(self) -> u64 {
        self.distributed_partitions
    }

    /// Returns distributed partition bytes.
    #[must_use]
    pub const fn distributed_partition_bytes(self) -> u64 {
        self.distributed_partition_bytes
    }

    /// Returns state bytes.
    #[must_use]
    pub const fn state_bytes(self) -> u64 {
        self.state_bytes
    }

    /// Returns temporary state bytes.
    #[must_use]
    pub const fn temporary_state_bytes(self) -> u64 {
        self.temporary_state_bytes
    }

    /// Returns state elements.
    #[must_use]
    pub const fn state_elements(self) -> u64 {
        self.state_elements
    }

    /// Returns snapshot bytes.
    #[must_use]
    pub const fn snapshot_bytes(self) -> u64 {
        self.snapshot_bytes
    }

    /// Returns checkpoint bytes.
    #[must_use]
    pub const fn checkpoint_bytes(self) -> u64 {
        self.checkpoint_bytes
    }

    /// Returns tensor rank.
    #[must_use]
    pub const fn tensor_rank(self) -> u64 {
        self.tensor_rank
    }

    /// Returns tensor dimension.
    #[must_use]
    pub const fn tensor_dimension(self) -> u64 {
        self.tensor_dimension
    }

    /// Returns bond dimension.
    #[must_use]
    pub const fn bond_dimension(self) -> u64 {
        self.bond_dimension
    }

    /// Returns tensor count.
    #[must_use]
    pub const fn tensors(self) -> u64 {
        self.tensors
    }

    /// Returns measurement results.
    #[must_use]
    pub const fn measurement_results(self) -> u64 {
        self.measurement_results
    }

    /// Returns metadata bytes.
    #[must_use]
    pub const fn metadata_bytes(self) -> u64 {
        self.metadata_bytes
    }

    /// Returns planning work.
    #[must_use]
    pub const fn planning_work(self) -> u64 {
        self.planning_work
    }

    /// Adds two requirements using checked arithmetic.
    ///
    /// This is used by reservation planning and composite state construction.
    pub const fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, MemoryEstimateError> {
        macro_rules! add {
            ($left:expr, $right:expr) => {
                match $left.checked_add($right) {
                    Some(value) => value,
                    None => {
                        return Err(MemoryEstimateError::ElementCountOverflow)
                    }
                }
            };
        }

        Ok(Self {
            qubits: add!(self.qubits, other.qubits),
            classical_bits: add!(self.classical_bits, other.classical_bits),
            allocations: add!(self.allocations, other.allocations),

            host_bytes: add!(self.host_bytes, other.host_bytes),
            temporary_host_bytes: add!(
                self.temporary_host_bytes,
                other.temporary_host_bytes
            ),
            persistent_host_bytes: add!(
                self.persistent_host_bytes,
                other.persistent_host_bytes
            ),
            pinned_host_bytes: add!(
                self.pinned_host_bytes,
                other.pinned_host_bytes
            ),

            device_bytes: add!(self.device_bytes, other.device_bytes),
            temporary_device_bytes: add!(
                self.temporary_device_bytes,
                other.temporary_device_bytes
            ),

            distributed_bytes: add!(
                self.distributed_bytes,
                other.distributed_bytes
            ),
            distributed_partitions: add!(
                self.distributed_partitions,
                other.distributed_partitions
            ),
            distributed_partition_bytes: add!(
                self.distributed_partition_bytes,
                other.distributed_partition_bytes
            ),

            state_bytes: add!(self.state_bytes, other.state_bytes),
            temporary_state_bytes: add!(
                self.temporary_state_bytes,
                other.temporary_state_bytes
            ),
            state_elements: add!(
                self.state_elements,
                other.state_elements
            ),

            snapshot_bytes: add!(
                self.snapshot_bytes,
                other.snapshot_bytes
            ),
            checkpoint_bytes: add!(
                self.checkpoint_bytes,
                other.checkpoint_bytes
            ),

            tensor_rank: add!(self.tensor_rank, other.tensor_rank),
            tensor_dimension: add!(
                self.tensor_dimension,
                other.tensor_dimension
            ),
            bond_dimension: add!(
                self.bond_dimension,
                other.bond_dimension
            ),
            tensors: add!(self.tensors, other.tensors),

            measurement_results: add!(
                self.measurement_results,
                other.measurement_results
            ),
            metadata_bytes: add!(
                self.metadata_bytes,
                other.metadata_bytes
            ),
            planning_work: add!(
                self.planning_work,
                other.planning_work
            ),
        })
    }
}

// =============================================================================
// Memory limits
// =============================================================================

/// Immutable hard resource limits for Zamani quantum memory.
///
/// This structure is intentionally `Copy` and contains no runtime state.
/// Current allocation accounting belongs to `allocator.rs`, `budget.rs`, and
/// `reservation.rs`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MemoryLimits {
    max_qubits: u64,
    max_classical_bits: u64,
    max_allocations: u64,

    max_host_bytes: u64,
    max_temporary_host_bytes: u64,
    max_persistent_host_bytes: u64,
    max_pinned_host_bytes: u64,

    max_device_bytes: u64,
    max_temporary_device_bytes: u64,

    max_distributed_bytes: u64,
    max_distributed_partitions: u64,
    max_distributed_partition_bytes: u64,

    max_state_bytes: u64,
    max_temporary_state_bytes: u64,
    max_state_elements: u64,

    max_snapshot_bytes: u64,
    max_checkpoint_bytes: u64,

    max_tensor_rank: u64,
    max_tensor_dimension: u64,
    max_bond_dimension: u64,
    max_tensors: u64,

    max_state_vector_qubits: u64,
    max_density_matrix_qubits: u64,
    max_tensor_network_qubits: u64,

    max_measurement_results: u64,
    max_metadata_bytes: u64,
    max_planning_work: u64,
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl MemoryLimits {
    /// Creates the production memory policy.
    ///
    /// The policy is deliberately finite. Dense state-vector and
    /// density-matrix operations are further bounded by their representation
    /// specific qubit limits and byte limits.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_classical_bits: DEFAULT_MAX_CLASSICAL_BITS,
            max_allocations: DEFAULT_MAX_ALLOCATIONS,

            max_host_bytes: DEFAULT_MAX_HOST_BYTES,
            max_temporary_host_bytes: DEFAULT_MAX_TEMPORARY_HOST_BYTES,
            max_persistent_host_bytes:
                DEFAULT_MAX_PERSISTENT_HOST_BYTES,
            max_pinned_host_bytes: DEFAULT_MAX_PINNED_HOST_BYTES,

            max_device_bytes: DEFAULT_MAX_DEVICE_BYTES,
            max_temporary_device_bytes:
                DEFAULT_MAX_TEMPORARY_DEVICE_BYTES,

            max_distributed_bytes: DEFAULT_MAX_DISTRIBUTED_BYTES,
            max_distributed_partitions:
                DEFAULT_MAX_DISTRIBUTED_PARTITIONS,
            max_distributed_partition_bytes:
                DEFAULT_MAX_DISTRIBUTED_PARTITION_BYTES,

            max_state_bytes: DEFAULT_MAX_STATE_BYTES,
            max_temporary_state_bytes:
                DEFAULT_MAX_TEMPORARY_STATE_BYTES,
            max_state_elements: DEFAULT_MAX_STATE_ELEMENTS,

            max_snapshot_bytes: DEFAULT_MAX_SNAPSHOT_BYTES,
            max_checkpoint_bytes: DEFAULT_MAX_CHECKPOINT_BYTES,

            max_tensor_rank: DEFAULT_MAX_TENSOR_RANK,
            max_tensor_dimension: DEFAULT_MAX_TENSOR_DIMENSION,
            max_bond_dimension: DEFAULT_MAX_BOND_DIMENSION,
            max_tensors: DEFAULT_MAX_TENSORS,

            max_state_vector_qubits:
                DEFAULT_MAX_STATE_VECTOR_QUBITS,
            max_density_matrix_qubits:
                DEFAULT_MAX_DENSITY_MATRIX_QUBITS,
            max_tensor_network_qubits:
                DEFAULT_MAX_TENSOR_NETWORK_QUBITS,

            max_measurement_results:
                DEFAULT_MAX_MEASUREMENT_RESULTS,
            max_metadata_bytes: DEFAULT_MAX_METADATA_BYTES,
            max_planning_work: DEFAULT_MAX_PLANNING_WORK,
        }
    }

    /// Creates a deliberately restrictive policy suitable for untrusted or
    /// embedded environments.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            max_qubits: 256,
            max_classical_bits: 65_536,
            max_allocations: 100_000,

            max_host_bytes: 4 * 1024 * 1024 * 1024,
            max_temporary_host_bytes:
                1024 * 1024 * 1024,
            max_persistent_host_bytes:
                4 * 1024 * 1024 * 1024,
            max_pinned_host_bytes:
                512 * 1024 * 1024,

            max_device_bytes:
                4 * 1024 * 1024 * 1024,
            max_temporary_device_bytes:
                1024 * 1024 * 1024,

            max_distributed_bytes:
                64 * 1024 * 1024 * 1024,
            max_distributed_partitions: 4096,
            max_distributed_partition_bytes:
                4 * 1024 * 1024 * 1024,

            max_state_bytes:
                4 * 1024 * 1024 * 1024,
            max_temporary_state_bytes:
                1024 * 1024 * 1024,
            max_state_elements: 1u64 << 32,

            max_snapshot_bytes:
                4 * 1024 * 1024 * 1024,
            max_checkpoint_bytes:
                4 * 1024 * 1024 * 1024,

            max_tensor_rank: 32,
            max_tensor_dimension: 65_536,
            max_bond_dimension: 4096,
            max_tensors: 100_000,

            max_state_vector_qubits: 28,
            max_density_matrix_qubits: 14,
            max_tensor_network_qubits: 256,

            max_measurement_results: 100_000,
            max_metadata_bytes: 1024 * 1024,
            max_planning_work: 10_000_000,
        }
    }

    /// Creates a deny-by-default policy.
    ///
    /// Zero resource limits are legal. They mean that the corresponding
    /// resource is prohibited.
    ///
    /// This policy is useful for security testing and for constructing an
    /// explicitly permissioned policy through builder methods.
    #[must_use]
    pub const fn deny_all() -> Self {
        Self {
            max_qubits: 0,
            max_classical_bits: 0,
            max_allocations: 0,

            max_host_bytes: 0,
            max_temporary_host_bytes: 0,
            max_persistent_host_bytes: 0,
            max_pinned_host_bytes: 0,

            max_device_bytes: 0,
            max_temporary_device_bytes: 0,

            max_distributed_bytes: 0,
            max_distributed_partitions: 0,
            max_distributed_partition_bytes: 0,

            max_state_bytes: 0,
            max_temporary_state_bytes: 0,
            max_state_elements: 0,

            max_snapshot_bytes: 0,
            max_checkpoint_bytes: 0,

            max_tensor_rank: 0,
            max_tensor_dimension: 0,
            max_bond_dimension: 0,
            max_tensors: 0,

            max_state_vector_qubits: 0,
            max_density_matrix_qubits: 0,
            max_tensor_network_qubits: 0,

            max_measurement_results: 0,
            max_metadata_bytes: 0,
            max_planning_work: 0,
        }
    }

    // -------------------------------------------------------------------------
    // Configuration
    // -------------------------------------------------------------------------

    /// Validates the internal consistency of this policy.
    ///
    /// Zero values are allowed. They mean that a resource is disabled.
    pub const fn validate(&self) -> Result<(), MemoryLimitConfigError> {
        if self.max_temporary_host_bytes > self.max_host_bytes {
            return Err(
                MemoryLimitConfigError::TemporaryHostExceedsHost {
                    temporary: self.max_temporary_host_bytes,
                    host: self.max_host_bytes,
                },
            );
        }

        if self.max_persistent_host_bytes > self.max_host_bytes {
            return Err(
                MemoryLimitConfigError::PersistentHostExceedsHost {
                    persistent: self.max_persistent_host_bytes,
                    host: self.max_host_bytes,
                },
            );
        }

        if self.max_temporary_device_bytes > self.max_device_bytes {
            return Err(
                MemoryLimitConfigError::TemporaryDeviceExceedsDevice {
                    temporary: self.max_temporary_device_bytes,
                    device: self.max_device_bytes,
                },
            );
        }

        if self.max_temporary_state_bytes > self.max_state_bytes {
            return Err(
                MemoryLimitConfigError::TemporaryStateExceedsState {
                    temporary: self.max_temporary_state_bytes,
                    state: self.max_state_bytes,
                },
            );
        }

        if self.max_state_bytes > self.max_host_bytes {
            return Err(MemoryLimitConfigError::StateExceedsHost {
                state: self.max_state_bytes,
                host: self.max_host_bytes,
            });
        }

        if self.max_state_vector_qubits > self.max_qubits {
            return Err(
                MemoryLimitConfigError::StateVectorQubitsExceedGlobal {
                    state_vector: self.max_state_vector_qubits,
                    qubits: self.max_qubits,
                },
            );
        }

        if self.max_density_matrix_qubits > self.max_qubits {
            return Err(
                MemoryLimitConfigError::DensityMatrixQubitsExceedGlobal {
                    density_matrix: self.max_density_matrix_qubits,
                    qubits: self.max_qubits,
                },
            );
        }

        if self.max_tensor_network_qubits > self.max_qubits {
            return Err(
                MemoryLimitConfigError::TensorNetworkQubitsExceedGlobal {
                    tensor_network: self.max_tensor_network_qubits,
                    qubits: self.max_qubits,
                },
            );
        }

        if self.max_distributed_partition_bytes
            > self.max_distributed_bytes
        {
            return Err(
                MemoryLimitConfigError::PartitionExceedsDistributed {
                    partition: self.max_distributed_partition_bytes,
                    distributed: self.max_distributed_bytes,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Maximum logical/physical qubits.
    #[must_use]
    pub const fn max_qubits(&self) -> u64 {
        self.max_qubits
    }

    /// Maximum classical bits.
    #[must_use]
    pub const fn max_classical_bits(&self) -> u64 {
        self.max_classical_bits
    }

    /// Maximum tracked allocations.
    #[must_use]
    pub const fn max_allocations(&self) -> u64 {
        self.max_allocations
    }

    /// Maximum host bytes.
    #[must_use]
    pub const fn max_host_bytes(&self) -> u64 {
        self.max_host_bytes
    }

    /// Maximum temporary host bytes.
    #[must_use]
    pub const fn max_temporary_host_bytes(&self) -> u64 {
        self.max_temporary_host_bytes
    }

    /// Maximum persistent host bytes.
    #[must_use]
    pub const fn max_persistent_host_bytes(&self) -> u64 {
        self.max_persistent_host_bytes
    }

    /// Maximum pinned host bytes.
    #[must_use]
    pub const fn max_pinned_host_bytes(&self) -> u64 {
        self.max_pinned_host_bytes
    }

    /// Maximum device bytes.
    #[must_use]
    pub const fn max_device_bytes(&self) -> u64 {
        self.max_device_bytes
    }

    /// Maximum temporary device bytes.
    #[must_use]
    pub const fn max_temporary_device_bytes(&self) -> u64 {
        self.max_temporary_device_bytes
    }

    /// Maximum distributed bytes.
    #[must_use]
    pub const fn max_distributed_bytes(&self) -> u64 {
        self.max_distributed_bytes
    }

    /// Maximum distributed partitions.
    #[must_use]
    pub const fn max_distributed_partitions(&self) -> u64 {
        self.max_distributed_partitions
    }

    /// Maximum distributed partition bytes.
    #[must_use]
    pub const fn max_distributed_partition_bytes(&self) -> u64 {
        self.max_distributed_partition_bytes
    }

    /// Maximum bytes for one quantum state.
    #[must_use]
    pub const fn max_state_bytes(&self) -> u64 {
        self.max_state_bytes
    }

    /// Maximum temporary state bytes.
    #[must_use]
    pub const fn max_temporary_state_bytes(&self) -> u64 {
        self.max_temporary_state_bytes
    }

    /// Maximum state elements.
    #[must_use]
    pub const fn max_state_elements(&self) -> u64 {
        self.max_state_elements
    }

    /// Maximum snapshot bytes.
    #[must_use]
    pub const fn max_snapshot_bytes(&self) -> u64 {
        self.max_snapshot_bytes
    }

    /// Maximum checkpoint bytes.
    #[must_use]
    pub const fn max_checkpoint_bytes(&self) -> u64 {
        self.max_checkpoint_bytes
    }

    /// Maximum tensor rank.
    #[must_use]
    pub const fn max_tensor_rank(&self) -> u64 {
        self.max_tensor_rank
    }

    /// Maximum tensor dimension.
    #[must_use]
    pub const fn max_tensor_dimension(&self) -> u64 {
        self.max_tensor_dimension
    }

    /// Maximum tensor-network bond dimension.
    #[must_use]
    pub const fn max_bond_dimension(&self) -> u64 {
        self.max_bond_dimension
    }

    /// Maximum number of tensors.
    #[must_use]
    pub const fn max_tensors(&self) -> u64 {
        self.max_tensors
    }

    /// Maximum state-vector qubit count.
    #[must_use]
    pub const fn max_state_vector_qubits(&self) -> u64 {
        self.max_state_vector_qubits
    }

    /// Maximum density-matrix qubit count.
    #[must_use]
    pub const fn max_density_matrix_qubits(&self) -> u64 {
        self.max_density_matrix_qubits
    }

    /// Maximum tensor-network qubit count.
    #[must_use]
    pub const fn max_tensor_network_qubits(&self) -> u64 {
        self.max_tensor_network_qubits
    }

    /// Maximum measurement results.
    #[must_use]
    pub const fn max_measurement_results(&self) -> u64 {
        self.max_measurement_results
    }

    /// Maximum metadata bytes.
    #[must_use]
    pub const fn max_metadata_bytes(&self) -> u64 {
        self.max_metadata_bytes
    }

    /// Maximum planning work.
    #[must_use]
    pub const fn max_planning_work(&self) -> u64 {
        self.max_planning_work
    }

    // -------------------------------------------------------------------------
    // Basic checks
    // -------------------------------------------------------------------------

    /// Checks whether a requested value fits a limit.
    pub const fn check(
        kind: MemoryLimitKind,
        requested: u64,
        maximum: u64,
    ) -> Result<(), MemoryLimitViolation> {
        if requested <= maximum {
            Ok(())
        } else {
            Err(MemoryLimitViolation::new(
                kind, requested, maximum,
            ))
        }
    }

    /// Checks logical/physical qubits.
    pub const fn check_qubits(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::Qubits,
            requested,
            self.max_qubits,
        )
    }

    /// Checks classical bits.
    pub const fn check_classical_bits(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::ClassicalBits,
            requested,
            self.max_classical_bits,
        )
    }

    /// Checks allocation count.
    pub const fn check_allocations(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::Allocations,
            requested,
            self.max_allocations,
        )
    }

    /// Checks host memory.
    pub const fn check_host_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::HostBytes,
            requested,
            self.max_host_bytes,
        )
    }

    /// Checks temporary host memory.
    pub const fn check_temporary_host_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TemporaryHostBytes,
            requested,
            self.max_temporary_host_bytes,
        )
    }

    /// Checks persistent host memory.
    pub const fn check_persistent_host_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::PersistentHostBytes,
            requested,
            self.max_persistent_host_bytes,
        )
    }

    /// Checks pinned host memory.
    pub const fn check_pinned_host_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::PinnedHostBytes,
            requested,
            self.max_pinned_host_bytes,
        )
    }

    /// Checks device memory.
    pub const fn check_device_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::DeviceBytes,
            requested,
            self.max_device_bytes,
        )
    }

    /// Checks temporary device memory.
    pub const fn check_temporary_device_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TemporaryDeviceBytes,
            requested,
            self.max_temporary_device_bytes,
        )
    }

    /// Checks distributed memory.
    pub const fn check_distributed_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::DistributedBytes,
            requested,
            self.max_distributed_bytes,
        )
    }

    /// Checks distributed partition count.
    pub const fn check_distributed_partitions(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::DistributedPartitions,
            requested,
            self.max_distributed_partitions,
        )
    }

    /// Checks distributed partition size.
    pub const fn check_distributed_partition_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::DistributedPartitionBytes,
            requested,
            self.max_distributed_partition_bytes,
        )
    }

    /// Checks one state allocation.
    pub const fn check_state_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::StateBytes,
            requested,
            self.max_state_bytes,
        )
    }

    /// Checks temporary state memory.
    pub const fn check_temporary_state_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TemporaryStateBytes,
            requested,
            self.max_temporary_state_bytes,
        )
    }

    /// Checks state element count.
    pub const fn check_state_elements(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::StateElements,
            requested,
            self.max_state_elements,
        )
    }

    /// Checks snapshot size.
    pub const fn check_snapshot_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::SnapshotBytes,
            requested,
            self.max_snapshot_bytes,
        )
    }

    /// Checks checkpoint size.
    pub const fn check_checkpoint_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::CheckpointBytes,
            requested,
            self.max_checkpoint_bytes,
        )
    }

    /// Checks tensor rank.
    pub const fn check_tensor_rank(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TensorRank,
            requested,
            self.max_tensor_rank,
        )
    }

    /// Checks tensor dimension.
    pub const fn check_tensor_dimension(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TensorDimension,
            requested,
            self.max_tensor_dimension,
        )
    }

    /// Checks bond dimension.
    pub const fn check_bond_dimension(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::BondDimension,
            requested,
            self.max_bond_dimension,
        )
    }

    /// Checks tensor count.
    pub const fn check_tensors(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::Tensors,
            requested,
            self.max_tensors,
        )
    }

    /// Checks state-vector qubit count.
    pub const fn check_state_vector_qubits(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::StateVectorQubits,
            requested,
            self.max_state_vector_qubits,
        )
    }

    /// Checks density-matrix qubit count.
    pub const fn check_density_matrix_qubits(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::DensityMatrixQubits,
            requested,
            self.max_density_matrix_qubits,
        )
    }

    /// Checks tensor-network qubit count.
    pub const fn check_tensor_network_qubits(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::TensorNetworkQubits,
            requested,
            self.max_tensor_network_qubits,
        )
    }

    /// Checks measurement-result count.
    pub const fn check_measurement_results(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::MeasurementResults,
            requested,
            self.max_measurement_results,
        )
    }

    /// Checks metadata bytes.
    pub const fn check_metadata_bytes(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::MetadataBytes,
            requested,
            self.max_metadata_bytes,
        )
    }

    /// Checks deterministic planning work.
    pub const fn check_planning_work(
        &self,
        requested: u64,
    ) -> Result<(), MemoryLimitViolation> {
        Self::check(
            MemoryLimitKind::PlanningWork,
            requested,
            self.max_planning_work,
        )
    }

    // -------------------------------------------------------------------------
    // Requirement validation
    // -------------------------------------------------------------------------

    /// Validates an entire resource requirement against this policy.
    ///
    /// The first violation is returned. No allocation or mutation occurs.
    pub const fn check_requirement(
        &self,
        requirement: MemoryRequirement,
    ) -> Result<(), MemoryLimitViolation> {
        self.check_qubits(requirement.qubits())?;
        self.check_classical_bits(requirement.classical_bits())?;
        self.check_allocations(requirement.allocations())?;

        self.check_host_bytes(requirement.host_bytes())?;
        self.check_temporary_host_bytes(
            requirement.temporary_host_bytes(),
        )?;
        self.check_persistent_host_bytes(
            requirement.persistent_host_bytes(),
        )?;
        self.check_pinned_host_bytes(
            requirement.pinned_host_bytes(),
        )?;

        self.check_device_bytes(requirement.device_bytes())?;
        self.check_temporary_device_bytes(
            requirement.temporary_device_bytes(),
        )?;

        self.check_distributed_bytes(
            requirement.distributed_bytes(),
        )?;
        self.check_distributed_partitions(
            requirement.distributed_partitions(),
        )?;
        self.check_distributed_partition_bytes(
            requirement.distributed_partition_bytes(),
        )?;

        self.check_state_bytes(requirement.state_bytes())?;
        self.check_temporary_state_bytes(
            requirement.temporary_state_bytes(),
        )?;
        self.check_state_elements(
            requirement.state_elements(),
        )?;

        self.check_snapshot_bytes(requirement.snapshot_bytes())?;
        self.check_checkpoint_bytes(
            requirement.checkpoint_bytes(),
        )?;

        self.check_tensor_rank(requirement.tensor_rank())?;
        self.check_tensor_dimension(
            requirement.tensor_dimension(),
        )?;
        self.check_bond_dimension(requirement.bond_dimension())?;
        self.check_tensors(requirement.tensors())?;

        self.check_measurement_results(
            requirement.measurement_results(),
        )?;
        self.check_metadata_bytes(
            requirement.metadata_bytes(),
        )?;
        self.check_planning_work(requirement.planning_work())?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Power helpers
    // -------------------------------------------------------------------------

    /// Computes `base^exponent` with checked arithmetic.
    pub const fn checked_pow_u64(
        base: u64,
        exponent: u64,
    ) -> Result<u64, MemoryEstimateError> {
        let mut result = 1u64;
        let mut power = base;
        let mut remaining = exponent;

        while remaining > 0 {
            if remaining & 1 == 1 {
                result = match result.checked_mul(power) {
                    Some(value) => value,
                    None => {
                        return Err(
                            MemoryEstimateError::ExponentOverflow {
                                base,
                                exponent,
                            },
                        )
                    }
                };
            }

            remaining >>= 1;

            if remaining > 0 {
                power = match power.checked_mul(power) {
                    Some(value) => value,
                    None => {
                        return Err(
                            MemoryEstimateError::ExponentOverflow {
                                base,
                                exponent,
                            },
                        )
                    }
                };
            }
        }

        Ok(result)
    }

    /// Calculates the number of basis states for `qubits`.
    ///
    /// Equivalent to `2^qubits`, but always checked.
    pub const fn basis_state_count(
        qubits: u64,
    ) -> Result<u64, MemoryEstimateError> {
        Self::checked_pow_u64(2, qubits)
    }

    /// Calculates the number of elements in a dense density matrix.
    ///
    /// Equivalent to `4^qubits`.
    pub const fn density_matrix_element_count(
        qubits: u64,
    ) -> Result<u64, MemoryEstimateError> {
        Self::checked_pow_u64(4, qubits)
    }

    // -------------------------------------------------------------------------
    // State-vector estimation
    // -------------------------------------------------------------------------

    /// Estimates dense state-vector memory.
    ///
    /// `bytes_per_element` must include the complete representation of one
    /// complex amplitude. For example, a complex f64 amplitude is normally
    /// represented using 16 bytes.
    pub const fn estimate_state_vector(
        &self,
        qubits: u64,
        bytes_per_element: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.check_state_vector_qubits(qubits)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        let elements = Self::basis_state_count(qubits)?;

        self.check_state_elements(elements)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        let estimate = MemoryEstimate::new(
            qubits,
            elements,
            bytes_per_element,
        )?;

        self.check_state_bytes(estimate.bytes())
            .map_err(|_| MemoryEstimateError::ByteCountOverflow)?;

        Ok(estimate)
    }

    /// Estimates a dense state vector using complex f64 storage.
    ///
    /// A complex f64 amplitude occupies 16 bytes.
    pub const fn estimate_state_vector_complex_f64(
        &self,
        qubits: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.estimate_state_vector(qubits, 16)
    }

    /// Estimates a dense state vector using complex f32 storage.
    ///
    /// A complex f32 amplitude occupies 8 bytes.
    pub const fn estimate_state_vector_complex_f32(
        &self,
        qubits: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.estimate_state_vector(qubits, 8)
    }

    // -------------------------------------------------------------------------
    // Density-matrix estimation
    // -------------------------------------------------------------------------

    /// Estimates dense density-matrix memory.
    ///
    /// A density matrix contains `4^n` complex elements.
    pub const fn estimate_density_matrix(
        &self,
        qubits: u64,
        bytes_per_element: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.check_density_matrix_qubits(qubits)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        let elements =
            Self::density_matrix_element_count(qubits)?;

        self.check_state_elements(elements)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        let estimate = MemoryEstimate::new(
            qubits,
            elements,
            bytes_per_element,
        )?;

        self.check_state_bytes(estimate.bytes())
            .map_err(|_| MemoryEstimateError::ByteCountOverflow)?;

        Ok(estimate)
    }

    /// Estimates a density matrix using complex f64 storage.
    pub const fn estimate_density_matrix_complex_f64(
        &self,
        qubits: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.estimate_density_matrix(qubits, 16)
    }

    /// Estimates a density matrix using complex f32 storage.
    pub const fn estimate_density_matrix_complex_f32(
        &self,
        qubits: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.estimate_density_matrix(qubits, 8)
    }

    // -------------------------------------------------------------------------
    // Tensor estimation
    // -------------------------------------------------------------------------

    /// Calculates tensor element count as:
    ///
    /// `dimension^rank`
    ///
    /// This is intentionally checked because tensor operations can otherwise
    /// create enormous implicit allocations.
    pub const fn tensor_element_count(
        rank: u64,
        dimension: u64,
    ) -> Result<u64, MemoryEstimateError> {
        if rank == 0 {
            return Err(MemoryEstimateError::InvalidTensorRank);
        }

        if dimension == 0 {
            return Err(MemoryEstimateError::InvalidTensorDimension);
        }

        Self::checked_pow_u64(dimension, rank)
    }

    /// Estimates the memory required by a dense tensor.
    pub const fn estimate_tensor(
        &self,
        rank: u64,
        dimension: u64,
        bytes_per_element: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.check_tensor_rank(rank)
            .map_err(|_| MemoryEstimateError::InvalidTensorRank)?;

        self.check_tensor_dimension(dimension)
            .map_err(|_| {
                MemoryEstimateError::InvalidTensorDimension
            })?;

        let elements =
            Self::tensor_element_count(rank, dimension)?;

        self.check_state_elements(elements)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        MemoryEstimate::new(0, elements, bytes_per_element)
    }

    // -------------------------------------------------------------------------
    // Tensor-network estimation
    // -------------------------------------------------------------------------

    /// Estimates a conservative upper bound for a tensor-network state.
    ///
    /// The estimate is intentionally conservative:
    ///
    /// ```text
    /// tensor count × physical dimension × bond dimension²
    /// ```
    ///
    /// This is not a claim about the exact storage layout of every tensor
    /// network. It is a safety estimate used before allocation.
    pub const fn estimate_tensor_network(
        &self,
        qubits: u64,
        physical_dimension: u64,
        bond_dimension: u64,
        bytes_per_element: u64,
    ) -> Result<MemoryEstimate, MemoryEstimateError> {
        self.check_tensor_network_qubits(qubits)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        self.check_tensor_dimension(physical_dimension)
            .map_err(|_| {
                MemoryEstimateError::InvalidTensorDimension
            })?;

        self.check_bond_dimension(bond_dimension)
            .map_err(|_| {
                MemoryEstimateError::InvalidBondDimension
            })?;

        if bond_dimension == 0 {
            return Err(MemoryEstimateError::InvalidBondDimension);
        }

        // physical_dimension × bond_dimension²
        let bond_squared = bond_dimension
            .checked_mul(bond_dimension)
            .ok_or(MemoryEstimateError::ElementCountOverflow)?;

        let elements_per_tensor = physical_dimension
            .checked_mul(bond_squared)
            .ok_or(MemoryEstimateError::ElementCountOverflow)?;

        let elements = qubits
            .checked_mul(elements_per_tensor)
            .ok_or(MemoryEstimateError::ElementCountOverflow)?;

        self.check_state_elements(elements)
            .map_err(|_| MemoryEstimateError::ElementCountOverflow)?;

        MemoryEstimate::new(
            qubits,
            elements,
            bytes_per_element,
        )
    }

    // -------------------------------------------------------------------------
    // Distributed estimation
    // -------------------------------------------------------------------------

    /// Estimates the number of bytes in one distributed partition.
    ///
    /// This is intentionally separate from global distributed memory because
    /// distributed execution must validate both:
    ///
    /// - total distributed capacity;
    /// - per-partition capacity.
    pub const fn check_distributed_partition(
        &self,
        partition_bytes: u64,
    ) -> Result<(), MemoryLimitViolation> {
        self.check_distributed_partition_bytes(partition_bytes)
    }

    /// Checks a complete distributed memory requirement.
    pub const fn check_distributed_allocation(
        &self,
        partitions: u64,
        partition_bytes: u64,
        total_bytes: u64,
    ) -> Result<(), MemoryLimitViolation> {
        self.check_distributed_partitions(partitions)?;
        self.check_distributed_partition_bytes(partition_bytes)?;
        self.check_distributed_bytes(total_bytes)
    }

    // -------------------------------------------------------------------------
    // Snapshot/checkpoint planning
    // -------------------------------------------------------------------------

    /// Checks a snapshot before serialization.
    pub const fn check_snapshot(
        &self,
        bytes: u64,
    ) -> Result<(), MemoryLimitViolation> {
        self.check_snapshot_bytes(bytes)
    }

    /// Checks a checkpoint before serialization.
    pub const fn check_checkpoint(
        &self,
        bytes: u64,
    ) -> Result<(), MemoryLimitViolation> {
        self.check_checkpoint_bytes(bytes)
    }

    // -------------------------------------------------------------------------
    // Platform conversion
    // -------------------------------------------------------------------------

    /// Checks whether a byte requirement can be represented as a platform
    /// `usize`.
    pub fn check_platform_bytes(
        bytes: u64,
    ) -> Result<usize, MemoryEstimateError> {
        usize::try_from(bytes)
            .map_err(|_| MemoryEstimateError::PlatformSizeOverflow {
                bytes,
            })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_internally_valid() {
        assert!(MemoryLimits::production().validate().is_ok());
    }

    #[test]
    fn strict_policy_is_internally_valid() {
        assert!(MemoryLimits::strict().validate().is_ok());
    }

    #[test]
    fn deny_all_policy_is_internally_valid() {
        assert!(MemoryLimits::deny_all().validate().is_ok());
    }

    #[test]
    fn basis_state_count_is_checked() {
        assert_eq!(
            MemoryLimits::basis_state_count(0).unwrap(),
            1
        );

        assert_eq!(
            MemoryLimits::basis_state_count(1).unwrap(),
            2
        );

        assert_eq!(
            MemoryLimits::basis_state_count(10).unwrap(),
            1024
        );
    }

    #[test]
    fn density_matrix_count_is_checked() {
        assert_eq!(
            MemoryLimits::density_matrix_element_count(0).unwrap(),
            1
        );

        assert_eq!(
            MemoryLimits::density_matrix_element_count(1).unwrap(),
            4
        );

        assert_eq!(
            MemoryLimits::density_matrix_element_count(2).unwrap(),
            16
        );
    }

    #[test]
    fn state_vector_complex_f64_estimate_is_correct() {
        let limits = MemoryLimits::production();

        let estimate =
            limits.estimate_state_vector_complex_f64(10).unwrap();

        assert_eq!(estimate.qubits(), 10);
        assert_eq!(estimate.elements(), 1024);
        assert_eq!(estimate.bytes_per_element(), 16);
        assert_eq!(estimate.bytes(), 16_384);
    }

    #[test]
    fn state_vector_complex_f32_estimate_is_correct() {
        let limits = MemoryLimits::production();

        let estimate =
            limits.estimate_state_vector_complex_f32(10).unwrap();

        assert_eq!(estimate.elements(), 1024);
        assert_eq!(estimate.bytes_per_element(), 8);
        assert_eq!(estimate.bytes(), 8192);
    }

    #[test]
    fn density_matrix_complex_f64_estimate_is_correct() {
        let limits = MemoryLimits::production();

        let estimate =
            limits.estimate_density_matrix_complex_f64(4).unwrap();

        assert_eq!(estimate.elements(), 256);
        assert_eq!(estimate.bytes_per_element(), 16);
        assert_eq!(estimate.bytes(), 4096);
    }

    #[test]
    fn state_vector_memory_is_exponential() {
        let limits = MemoryLimits::production();

        let small =
            limits.estimate_state_vector_complex_f64(10).unwrap();

        let larger =
            limits.estimate_state_vector_complex_f64(20).unwrap();

        assert_eq!(
            larger.bytes(),
            small.bytes() * 1024
        );
    }

    #[test]
    fn density_matrix_memory_is_four_to_the_n() {
        let limits = MemoryLimits::production();

        let small =
            limits.estimate_density_matrix_complex_f64(4).unwrap();

        let larger =
            limits.estimate_density_matrix_complex_f64(5).unwrap();

        assert_eq!(
            larger.bytes(),
            small.bytes() * 4
        );
    }

    #[test]
    fn state_vector_limit_is_enforced() {
        let limits = MemoryLimits::strict();

        let result =
            limits.estimate_state_vector_complex_f64(29);

        assert!(result.is_err());
    }

    #[test]
    fn density_matrix_limit_is_enforced() {
        let limits = MemoryLimits::strict();

        let result =
            limits.estimate_density_matrix_complex_f64(15);

        assert!(result.is_err());
    }

    #[test]
    fn huge_power_is_rejected() {
        let result = MemoryLimits::basis_state_count(64);

        assert!(result.is_err());
    }

    #[test]
    fn requirement_fits_when_all_fields_are_within_limits() {
        let limits = MemoryLimits::production();

        let requirement = MemoryRequirement::empty()
            .with_qubits(100)
            .with_classical_bits(100)
            .with_allocations(10)
            .with_host_bytes(1024 * 1024)
            .with_state_bytes(1024 * 1024);

        assert!(limits.check_requirement(requirement).is_ok());
    }

    #[test]
    fn requirement_rejects_excessive_state_memory() {
        let limits = MemoryLimits::strict();

        let requirement = MemoryRequirement::empty()
            .with_state_bytes(limits.max_state_bytes() + 1);

        let result = limits.check_requirement(requirement);

        assert_eq!(
            result.unwrap_err().kind(),
            MemoryLimitKind::StateBytes
        );
    }

    #[test]
    fn requirement_addition_is_checked() {
        let left = MemoryRequirement::empty()
            .with_host_bytes(100);

        let right = MemoryRequirement::empty()
            .with_host_bytes(200);

        let combined = left.checked_add(right).unwrap();

        assert_eq!(combined.host_bytes(), 300);
    }

    #[test]
    fn requirement_addition_detects_overflow() {
        let left = MemoryRequirement::empty()
            .with_host_bytes(u64::MAX);

        let right = MemoryRequirement::empty()
            .with_host_bytes(1);

        assert!(left.checked_add(right).is_err());
    }

    #[test]
    fn distributed_partition_is_checked_independently() {
        let limits = MemoryLimits::strict();

        assert!(
            limits
                .check_distributed_partition(
                    limits.max_distributed_partition_bytes()
                )
                .is_ok()
        );

        assert!(
            limits
                .check_distributed_partition(
                    limits.max_distributed_partition_bytes() + 1
                )
                .is_err()
        );
    }

    #[test]
    fn snapshot_limit_is_checked_before_serialization() {
        let limits = MemoryLimits::strict();

        assert!(
            limits
                .check_snapshot(limits.max_snapshot_bytes())
                .is_ok()
        );

        assert!(
            limits
                .check_snapshot(limits.max_snapshot_bytes() + 1)
                .is_err()
        );
    }

    #[test]
    fn checkpoint_limit_is_checked_before_serialization() {
        let limits = MemoryLimits::strict();

        assert!(
            limits
                .check_checkpoint(limits.max_checkpoint_bytes())
                .is_ok()
        );

        assert!(
            limits
                .check_checkpoint(limits.max_checkpoint_bytes() + 1)
                .is_err()
        );
    }

    #[test]
    fn tensor_element_count_is_checked() {
        assert_eq!(
            MemoryLimits::tensor_element_count(2, 4).unwrap(),
            16
        );

        assert_eq!(
            MemoryLimits::tensor_element_count(3, 2).unwrap(),
            8
        );
    }

    #[test]
    fn tensor_zero_rank_is_rejected() {
        assert_eq!(
            MemoryLimits::tensor_element_count(0, 2),
            Err(MemoryEstimateError::InvalidTensorRank)
        );
    }

    #[test]
    fn tensor_zero_dimension_is_rejected() {
        assert_eq!(
            MemoryLimits::tensor_element_count(2, 0),
            Err(MemoryEstimateError::InvalidTensorDimension)
        );
    }

    #[test]
    fn tensor_network_estimate_is_checked() {
        let limits = MemoryLimits::strict();

        let estimate = limits
            .estimate_tensor_network(
                10,
                2,
                4,
                16,
            )
            .unwrap();

        assert_eq!(estimate.qubits(), 10);

        // 10 tensors × 2 physical dimension × 4² bond dimension.
        assert_eq!(estimate.elements(), 320);

        assert_eq!(estimate.bytes(), 320 * 16);
    }

    #[test]
    fn platform_conversion_is_checked() {
        let value = MemoryLimits::check_platform_bytes(1024);

        assert_eq!(value.unwrap(), 1024usize);
    }

    #[test]
    fn violation_contains_machine_readable_identity() {
        let limits = MemoryLimits::strict();

        let violation =
            limits.check_qubits(limits.max_qubits() + 1).unwrap_err();

        assert_eq!(violation.kind(), MemoryLimitKind::Qubits);
        assert_eq!(
            violation.requested(),
            limits.max_qubits() + 1
        );
        assert_eq!(
            violation.maximum(),
            limits.max_qubits()
        );
    }

    #[test]
    fn zero_limits_are_valid() {
        let limits = MemoryLimits::deny_all();

        assert!(limits.check_qubits(0).is_ok());
        assert!(limits.check_qubits(1).is_err());

        assert!(limits.check_state_bytes(0).is_ok());
        assert!(limits.check_state_bytes(1).is_err());
    }

    #[test]
    fn estimate_knows_platform_capacity() {
        let limits = MemoryLimits::production();

        let estimate =
            limits.estimate_state_vector_complex_f64(10).unwrap();

        assert!(estimate.fits_platform_usize());
        assert_eq!(
            estimate.bytes_as_usize().unwrap(),
            16_384usize
        );
    }
}