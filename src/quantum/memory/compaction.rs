//! Zamani Quantum Memory — Compaction
//!
//! Production-grade, representation-independent memory compaction contracts
//! and safe host-side compaction algorithms for `quantum::memory`.
//!
//! # Purpose
//!
//! This module provides the canonical lifecycle/compaction abstraction for
//! quantum memory without assuming a particular state representation,
//! allocator, processor, accelerator, distributed-memory system, or QPU.
//!
//! Compaction is deliberately separated from allocation and from individual
//! quantum-state representations.
//!
//! The module supports:
//!
//! - deterministic compaction planning;
//! - stable relocation maps;
//! - hole/fragmentation analysis;
//! - bounded compaction policies;
//! - atomic-plan semantics;
//! - rollback-safe planning;
//! - in-place compaction of safe owned host containers;
//! - sparse-slot compaction;
//! - segment/block compaction planning;
//! - device/GPU compaction contracts;
//! - distributed-memory compaction contracts;
//! - backend/QPU capability negotiation;
//! - preservation of logical quantum identity;
//! - preservation of representation semantics;
//! - memory-budget aware planning;
//! - movement accounting;
//! - fragmentation metrics;
//! - deterministic diagnostics;
//! - no-unsafe implementation.
//!
//! # Architectural boundary
//!
//! `compaction.rs` owns:
//!
//! - determining whether compaction is useful;
//! - describing how storage may be relocated;
//! - constructing deterministic relocation plans;
//! - validating those plans;
//! - calculating movement and reclaimed-capacity estimates;
//! - executing safe generic host-side compaction operations;
//! - defining provider-neutral contracts for device/distributed/backend
//!   implementations.
//!
//! It does NOT own:
//!
//! - quantum IR;
//! - qubit identity;
//! - routing;
//! - scheduling;
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer mathematics;
//! - tensor-network mathematics;
//! - GPU APIs;
//! - CUDA/HIP/Metal/Vulkan APIs;
//! - MPI/RDMA/UCX APIs;
//! - QPU communication;
//! - vendor authentication;
//! - backend calibration;
//! - measurement;
//! - quantum error-correction algorithms;
//! - benchmarking protocols.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Critical semantic rule
//!
//! Compaction MUST change storage placement only.
//!
//! It MUST NOT change:
//!
//! - logical qubit identity;
//! - physical qubit identity;
//! - logical-to-physical mapping;
//! - quantum amplitudes;
//! - probabilities;
//! - tensor values;
//! - stabilizer generators;
//! - classical measurement values;
//! - state representation;
//! - backend semantics.
//!
//! A successful compaction operation is therefore observationally equivalent
//! to the same memory object before compaction, except for storage location,
//! capacity, fragmentation, or allocation metadata.
//!
//! # QPU / hardware rule
//!
//! Real QPUs generally do not expose their internal quantum-state memory to
//! the software memory subsystem. Consequently this module MUST NOT pretend
//! that it can compact physical qubits or QPU-internal memory.
//!
//! For QPUs, compaction is normally one of:
//!
//! 1. a no-op because no exposed memory needs compaction;
//! 2. compaction of host-side buffers associated with a QPU job;
//! 3. compaction of serialized circuits/results/checkpoints;
//! 4. provider-defined memory management through the backend contract.
//!
//! A provider that cannot safely compact memory must report
//! `CompactionCapability::Unsupported` rather than performing a fabricated
//! operation.
//!
//! # Representation rule
//!
//! Different representations have different compaction semantics:
//!
//! ```text
//! StateVector       -> contiguous-buffer / shard compaction
//! DensityMatrix     -> contiguous-buffer / tile compaction
//! Stabilizer        -> tableau/storage compaction
//! SparseState       -> support-slot compaction
//! TensorNetwork     -> tensor/edge/storage compaction
//! BackendNative     -> provider-defined or unsupported
//! Distributed       -> shard/partition compaction
//! ```
//!
//! This module supplies the common contract, not the representation-specific
//! implementation.
//!
//! # No silent approximation
//!
//! Compaction is lossless.
//!
//! This module MUST NOT:
//!
//! - drop amplitudes;
//! - threshold amplitudes;
//! - change floating-point precision;
//! - truncate tensor-network bond dimensions;
//! - merge mathematically distinct entries;
//! - alter sparse-state probabilities.
//!
//! Approximation belongs to explicit algorithms such as sparse pruning or
//! tensor-network truncation, not compaction.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//!
//! # Integration contract
//!
//! This file is designed to be completed independently before
//! `migration.rs`, `diagnostics.rs`, `telemetry.rs`, or representation-specific
//! modules depend on it.
//!
//! The intended dependencies are:
//!
//! ```text
//! memory::types
//!       │
//!       ▼
//! memory::errors
//!       │
//!       ▼
//! memory::compaction
//!       │
//!       ├── sparse
//!       ├── state_vector
//!       ├── density_matrix
//!       ├── stabilizer
//!       ├── tensor_network
//!       ├── allocator
//!       ├── pool
//!       ├── migration
//!       ├── cpu
//!       ├── gpu
//!       ├── distributed
//!       ├── diagnostics
//!       └── telemetry
//! ```
//!
//! The module intentionally depends only on the foundational memory types
//! and error contract.
//!
//! # Safety
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! No raw pointers, device pointers, FFI handles, or unsafe containers are
//! exposed by this module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use super::errors::MemoryError;
use super::types::{ByteCount, QubitCount};

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the compaction contract.
pub const COMPACTION_SCHEMA_ID: &str = "zamani.quantum.memory.compaction";

/// Semantic version of the public compaction contract.
pub const COMPACTION_SCHEMA_VERSION: u16 = 1;

/// Maximum number of relocation operations accepted in one plan.
///
/// This is a safety bound against accidentally constructing enormous metadata
/// structures from untrusted configuration.
pub const MAX_RELOCATION_OPERATIONS: usize = 16_777_216;

/// Maximum number of segments accepted by the generic planner.
pub const MAX_SEGMENTS: usize = 16_777_216;

/// Maximum number of holes accepted by the generic planner.
pub const MAX_HOLES: usize = 16_777_216;

// =============================================================================
// Result
// =============================================================================

/// Result type used by compaction operations.
pub type CompactionResult<T> = Result<T, MemoryError>;

// =============================================================================
// Representation domain
// =============================================================================

/// Representation class for which compaction is being requested.
///
/// This enum deliberately mirrors the planned memory representation boundary
/// without importing representation-specific implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompactionRepresentation {
    /// Dense pure-state vector.
    StateVector,

    /// Density matrix.
    DensityMatrix,

    /// Stabilizer/tableau state.
    Stabilizer,

    /// Sparse pure state.
    Sparse,

    /// Tensor-network representation.
    TensorNetwork,

    /// Generic tensor/buffer storage.
    Tensor,

    /// Backend-owned representation.
    BackendNative,

    /// Host-side data associated with a QPU job.
    QpuHostBuffer,

    /// Distributed state shard.
    DistributedShard,

    /// Generic memory object.
    Generic,
}

impl fmt::Display for CompactionRepresentation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Stabilizer => "stabilizer",
            Self::Sparse => "sparse",
            Self::TensorNetwork => "tensor_network",
            Self::Tensor => "tensor",
            Self::BackendNative => "backend_native",
            Self::QpuHostBuffer => "qpu_host_buffer",
            Self::DistributedShard => "distributed_shard",
            Self::Generic => "generic",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Storage domain
// =============================================================================

/// Storage domain whose fragmentation is being compacted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompactionStorage {
    /// Normal host memory.
    Host,

    /// Pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared memory.
    Unified,

    /// Distributed memory.
    Distributed,

    /// Host-side memory owned by a backend/QPU integration.
    BackendHost,

    /// Remote/backend-owned storage.
    Remote,
}

impl fmt::Display for CompactionStorage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::BackendHost => "backend_host",
            Self::Remote => "remote",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Capability
// =============================================================================

/// Provider capability for compaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompactionCapability {
    /// Compaction is supported and may relocate objects.
    Supported,

    /// Compaction is supported only when no externally visible address is
    /// retained.
    SupportedWithRelocation,

    /// Compaction can be performed only by a provider-specific operation.
    ProviderDefined,

    /// The provider explicitly does not support compaction.
    Unsupported,
}

impl CompactionCapability {
    /// Returns whether this capability permits a generic compaction operation.
    pub const fn permits_generic_compaction(self) -> bool {
        matches!(
            self,
            Self::Supported | Self::SupportedWithRelocation
        )
    }

    /// Returns whether provider-specific code is required.
    pub const fn requires_provider(self) -> bool {
        matches!(self, Self::ProviderDefined)
    }
}

// =============================================================================
// Policy
// =============================================================================

/// Compaction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactionPolicy {
    /// Minimum number of bytes that must be reclaimable before compaction is
    /// considered useful.
    pub minimum_reclaim_bytes: u64,

    /// Minimum fragmentation percentage before compaction is considered.
    ///
    /// This is represented in basis points:
    ///
    /// `10000 == 100%`
    ///
    /// `100 == 1%`.
    pub minimum_fragmentation_basis_points: u16,

    /// Maximum bytes that may be moved by one compaction operation.
    ///
    /// `None` means no explicit movement limit.
    pub maximum_movement_bytes: Option<u64>,

    /// Maximum number of relocation operations.
    pub maximum_relocations: usize,

    /// Whether stable ordering must be preserved.
    ///
    /// Stable ordering is strongly recommended for deterministic quantum
    /// memory behavior.
    pub preserve_order: bool,

    /// Whether compaction may move externally referenced objects.
    pub allow_relocation: bool,

    /// Whether compaction may run when the predicted temporary workspace is
    /// non-zero.
    pub allow_temporary_workspace: bool,
}

impl Default for CompactionPolicy {
    fn default() -> Self {
        Self {
            minimum_reclaim_bytes: 4096,
            minimum_fragmentation_basis_points: 500,
            maximum_movement_bytes: None,
            maximum_relocations: MAX_RELOCATION_OPERATIONS,
            preserve_order: true,
            allow_relocation: true,
            allow_temporary_workspace: true,
        }
    }
}

impl CompactionPolicy {
    /// Policy that only performs compaction when explicitly requested.
    pub const fn conservative() -> Self {
        Self {
            minimum_reclaim_bytes: u64::MAX,
            minimum_fragmentation_basis_points: u16::MAX,
            maximum_movement_bytes: Some(0),
            maximum_relocations: 0,
            preserve_order: true,
            allow_relocation: false,
            allow_temporary_workspace: false,
        }
    }

    /// Aggressive but lossless compaction policy.
    pub const fn aggressive() -> Self {
        Self {
            minimum_reclaim_bytes: 1,
            minimum_fragmentation_basis_points: 1,
            maximum_movement_bytes: None,
            maximum_relocations: MAX_RELOCATION_OPERATIONS,
            preserve_order: true,
            allow_relocation: true,
            allow_temporary_workspace: true,
        }
    }

    /// Validates the policy.
    pub fn validate(&self) -> CompactionResult<()> {
        if self.maximum_relocations > MAX_RELOCATION_OPERATIONS {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "maximum relocation count {} exceeds safety limit {}",
                    self.maximum_relocations,
                    MAX_RELOCATION_OPERATIONS
                ),
            });
        }

        if self.minimum_fragmentation_basis_points > 10_000 {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "fragmentation threshold {} basis points exceeds 10000",
                    self.minimum_fragmentation_basis_points
                ),
            });
        }

        if !self.allow_relocation
            && self.maximum_movement_bytes.is_some_and(|bytes| bytes > 0)
        {
            return Err(MemoryError::CompactionError {
                reason: "movement budget is non-zero while relocation is disabled"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Memory extent
// =============================================================================

/// A contiguous memory extent.
///
/// `offset` and `length` are logical offsets in the relevant storage domain.
/// They are not raw pointers and must never be interpreted as addresses by
/// this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemoryExtent {
    /// Logical storage offset.
    pub offset: u64,

    /// Extent length in bytes.
    pub length: u64,
}

impl MemoryExtent {
    /// Creates an extent.
    pub const fn new(offset: u64, length: u64) -> Self {
        Self { offset, length }
    }

    /// Returns the exclusive end offset.
    pub fn end(&self) -> CompactionResult<u64> {
        self.offset.checked_add(self.length).ok_or_else(|| {
            MemoryError::ArithmeticOverflow {
                operation: "memory extent end".to_string(),
            }
        })
    }

    /// Returns whether the extent is empty.
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns whether two extents overlap.
    pub fn overlaps(&self, other: &Self) -> CompactionResult<bool> {
        if self.is_empty() || other.is_empty() {
            return Ok(false);
        }

        let self_end = self.end()?;
        let other_end = other.end()?;

        Ok(self.offset < other_end && other.offset < self_end)
    }
}

// =============================================================================
// Allocation extent
// =============================================================================

/// An occupied memory extent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OccupiedExtent {
    /// Stable object identifier supplied by the allocator/provider.
    pub object_id: u64,

    /// Current storage extent.
    pub extent: MemoryExtent,

    /// Whether the object is movable.
    pub movable: bool,

    /// Whether the object is externally address-sensitive.
    ///
    /// An address-sensitive object must not be relocated by the generic
    /// compactor.
    pub address_sensitive: bool,
}

impl OccupiedExtent {
    /// Creates an occupied extent.
    pub const fn new(
        object_id: u64,
        offset: u64,
        length: u64,
        movable: bool,
        address_sensitive: bool,
    ) -> Self {
        Self {
            object_id,
            extent: MemoryExtent::new(offset, length),
            movable,
            address_sensitive,
        }
    }

    /// Returns whether this object may be relocated generically.
    pub const fn is_generically_movable(&self) -> bool {
        self.movable && !self.address_sensitive
    }
}

// =============================================================================
// Relocation
// =============================================================================

/// A single lossless relocation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Relocation {
    /// Stable object identity.
    pub object_id: u64,

    /// Current logical storage extent.
    pub source: MemoryExtent,

    /// New logical storage extent.
    pub destination: MemoryExtent,
}

impl Relocation {
    /// Creates a relocation.
    pub const fn new(
        object_id: u64,
        source: MemoryExtent,
        destination: MemoryExtent,
    ) -> Self {
        Self {
            object_id,
            source,
            destination,
        }
    }

    /// Returns the number of bytes moved.
    pub const fn bytes_moved(&self) -> u64 {
        self.source.length
    }

    /// Returns whether the relocation actually changes the offset.
    pub const fn changes_location(&self) -> bool {
        self.source.offset != self.destination.offset
    }
}

// =============================================================================
// Fragmentation statistics
// =============================================================================

/// Deterministic fragmentation statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationStats {
    /// Total managed capacity.
    pub capacity_bytes: u64,

    /// Occupied bytes.
    pub occupied_bytes: u64,

    /// Free bytes.
    pub free_bytes: u64,

    /// Largest contiguous free extent.
    pub largest_free_extent_bytes: u64,

    /// Number of free extents.
    pub free_extent_count: usize,

    /// Number of occupied extents.
    pub occupied_extent_count: usize,

    /// Number of movable occupied extents.
    pub movable_extent_count: usize,

    /// Bytes that can theoretically be reclaimed by packing movable objects.
    pub reclaimable_bytes: u64,

    /// Fragmentation expressed in basis points.
    pub fragmentation_basis_points: u16,
}

impl FragmentationStats {
    /// Computes fragmentation from capacity and largest free extent.
    pub fn calculate(
        capacity_bytes: u64,
        occupied_bytes: u64,
        largest_free_extent_bytes: u64,
        free_extent_count: usize,
        occupied_extent_count: usize,
        movable_extent_count: usize,
    ) -> CompactionResult<Self> {
        if occupied_bytes > capacity_bytes {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "occupied bytes {} exceed capacity {}",
                    occupied_bytes, capacity_bytes
                ),
            });
        }

        let free_bytes = capacity_bytes - occupied_bytes;

        let fragmentation_basis_points = if free_bytes == 0 {
            0
        } else {
            let fragmented_free = free_bytes.saturating_sub(largest_free_extent_bytes);

            let scaled = (fragmented_free as u128)
                .checked_mul(10_000)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "fragmentation basis-point calculation".to_string(),
                })?
                / free_bytes as u128;

            u16::try_from(scaled).unwrap_or(10_000)
        };

        let reclaimable_bytes = free_bytes.saturating_sub(largest_free_extent_bytes);

        Ok(Self {
            capacity_bytes,
            occupied_bytes,
            free_bytes,
            largest_free_extent_bytes,
            free_extent_count,
            occupied_extent_count,
            movable_extent_count,
            reclaimable_bytes,
            fragmentation_basis_points,
        })
    }

    /// Returns whether fragmentation is non-zero.
    pub const fn is_fragmented(&self) -> bool {
        self.free_extent_count > 1
            && self.free_bytes > self.largest_free_extent_bytes
    }
}

// =============================================================================
// Compaction request
// =============================================================================

/// Immutable description of a compaction request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactionRequest {
    /// Representation being compacted.
    pub representation: CompactionRepresentation,

    /// Storage domain.
    pub storage: CompactionStorage,

    /// Logical qubit count associated with the object, when applicable.
    ///
    /// This is metadata only. Compaction never changes it.
    pub qubits: Option<QubitCount>,

    /// Managed capacity.
    pub capacity_bytes: u64,

    /// Whether external observers may hold storage-location references.
    pub externally_address_sensitive: bool,

    /// Provider capability.
    pub capability: CompactionCapability,

    /// Compaction policy.
    pub policy: CompactionPolicy,
}

impl CompactionRequest {
    /// Creates a generic host-memory request.
    pub fn new(
        representation: CompactionRepresentation,
        storage: CompactionStorage,
        capacity_bytes: u64,
    ) -> Self {
        Self {
            representation,
            storage,
            qubits: None,
            capacity_bytes,
            externally_address_sensitive: false,
            capability: CompactionCapability::SupportedWithRelocation,
            policy: CompactionPolicy::default(),
        }
    }

    /// Validates the request.
    pub fn validate(&self) -> CompactionResult<()> {
        self.policy.validate()?;

        if self.capacity_bytes == 0 {
            return Err(MemoryError::CompactionError {
                reason: "cannot compact a zero-capacity memory domain".to_string(),
            });
        }

        if self.externally_address_sensitive
            && self.capability.permits_generic_compaction()
        {
            return Err(MemoryError::CompactionError {
                reason: "externally address-sensitive memory cannot use generic relocation"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Compaction plan
// =============================================================================

/// Complete deterministic compaction plan.
///
/// A plan is immutable after construction. Providers should validate the plan
/// and then execute it transactionally where their memory model permits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionPlan {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Representation.
    pub representation: CompactionRepresentation,

    /// Storage domain.
    pub storage: CompactionStorage,

    /// Original capacity.
    pub original_capacity_bytes: u64,

    /// Occupied bytes before compaction.
    pub occupied_bytes: u64,

    /// Free bytes before compaction.
    pub free_bytes_before: u64,

    /// Free bytes that become one contiguous tail after compaction.
    pub contiguous_reclaimable_bytes: u64,

    /// Predicted movement.
    pub movement_bytes: u64,

    /// Temporary workspace requirement.
    pub temporary_workspace_bytes: u64,

    /// Relocation operations.
    relocations: Vec<Relocation>,

    /// Whether ordering is preserved.
    pub preserves_order: bool,

    /// Whether the plan changes logical quantum semantics.
    ///
    /// This must always be false.
    pub changes_quantum_semantics: bool,
}

impl CompactionPlan {
    /// Returns an empty/no-op plan.
    pub fn noop(request: &CompactionRequest) -> CompactionResult<Self> {
        request.validate()?;

        Ok(Self {
            schema_id: COMPACTION_SCHEMA_ID,
            schema_version: COMPACTION_SCHEMA_VERSION,
            representation: request.representation,
            storage: request.storage,
            original_capacity_bytes: request.capacity_bytes,
            occupied_bytes: 0,
            free_bytes_before: request.capacity_bytes,
            contiguous_reclaimable_bytes: 0,
            movement_bytes: 0,
            temporary_workspace_bytes: 0,
            relocations: Vec::new(),
            preserves_order: true,
            changes_quantum_semantics: false,
        })
    }

    /// Returns all relocation operations in deterministic order.
    pub fn relocations(&self) -> &[Relocation] {
        &self.relocations
    }

    /// Returns the number of relocation operations.
    pub fn relocation_count(&self) -> usize {
        self.relocations.len()
    }

    /// Returns whether the plan performs no movement.
    pub fn is_noop(&self) -> bool {
        self.relocations.is_empty()
    }

    /// Returns whether the plan is semantically safe.
    pub fn is_lossless(&self) -> bool {
        !self.changes_quantum_semantics
    }

    /// Returns the expected post-compaction free extent.
    pub fn expected_free_tail(&self) -> CompactionResult<MemoryExtent> {
        let occupied_end = self
            .occupied_bytes
            .checked_add(0)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "post-compaction occupied end".to_string(),
            })?;

        let length = self
            .original_capacity_bytes
            .checked_sub(occupied_end)
            .ok_or_else(|| MemoryError::CompactionError {
                reason: "occupied bytes exceed original capacity".to_string(),
            })?;

        Ok(MemoryExtent::new(occupied_end, length))
    }

    /// Validates the entire relocation plan.
    pub fn validate(&self) -> CompactionResult<()> {
        if self.schema_id != COMPACTION_SCHEMA_ID {
            return Err(MemoryError::CompactionError {
                reason: "unexpected compaction schema identifier".to_string(),
            });
        }

        if self.schema_version != COMPACTION_SCHEMA_VERSION {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "unsupported compaction schema version {}",
                    self.schema_version
                ),
            });
        }

        if self.changes_quantum_semantics {
            return Err(MemoryError::CompactionError {
                reason: "compaction plan claims to change quantum semantics".to_string(),
            });
        }

        if self.relocations.len() > MAX_RELOCATION_OPERATIONS {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "relocation count {} exceeds maximum {}",
                    self.relocations.len(),
                    MAX_RELOCATION_OPERATIONS
                ),
            });
        }

        let mut movement = 0u64;

        for relocation in &self.relocations {
            if relocation.source.length != relocation.destination.length {
                return Err(MemoryError::CompactionError {
                    reason: format!(
                        "relocation for object {} changes its byte length",
                        relocation.object_id
                    ),
                });
            }

            let source_end = relocation.source.end()?;
            let destination_end = relocation.destination.end()?;

            if source_end > self.original_capacity_bytes {
                return Err(MemoryError::CompactionError {
                    reason: format!(
                        "source extent for object {} exceeds capacity",
                        relocation.object_id
                    ),
                });
            }

            if destination_end > self.original_capacity_bytes {
                return Err(MemoryError::CompactionError {
                    reason: format!(
                        "destination extent for object {} exceeds capacity",
                        relocation.object_id
                    ),
                });
            }

            movement = movement
                .checked_add(relocation.bytes_moved())
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "compaction movement accounting".to_string(),
                })?;
        }

        if movement != self.movement_bytes {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "movement accounting mismatch: plan says {}, relocations require {}",
                    self.movement_bytes, movement
                ),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Compaction report
// =============================================================================

/// Result of executing or accepting a compaction operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionReport {
    /// Whether compaction was performed.
    pub performed: bool,

    /// Number of relocated objects.
    pub relocated_objects: usize,

    /// Bytes moved.
    pub movement_bytes: u64,

    /// Bytes of contiguous free capacity after compaction.
    pub reclaimed_bytes: u64,

    /// Temporary workspace used.
    pub temporary_workspace_bytes: u64,

    /// Whether the operation preserved ordering.
    pub preserved_order: bool,

    /// Whether quantum semantics were preserved.
    pub semantics_preserved: bool,
}

impl CompactionReport {
    /// Creates a report from a validated plan.
    pub fn from_plan(plan: &CompactionPlan, performed: bool) -> Self {
        Self {
            performed,
            relocated_objects: plan.relocation_count(),
            movement_bytes: plan.movement_bytes,
            reclaimed_bytes: plan.contiguous_reclaimable_bytes,
            temporary_workspace_bytes: plan.temporary_workspace_bytes,
            preserved_order: plan.preserves_order,
            semantics_preserved: !plan.changes_quantum_semantics,
        }
    }
}

// =============================================================================
// Planner
// =============================================================================

/// Builds a stable compaction plan from occupied extents.
///
/// The algorithm is deliberately representation-neutral:
///
/// 1. validate the occupied extents;
/// 2. sort them by current offset;
/// 3. keep non-movable/address-sensitive objects fixed;
/// 4. move movable objects toward the lowest legal location;
/// 5. preserve object order;
/// 6. never change object sizes;
/// 7. never change logical quantum identity;
/// 8. never overlap destinations;
/// 9. never exceed the supplied capacity.
///
/// This is a *planning* algorithm. It does not perform memory movement.
pub fn plan_compaction(
    request: &CompactionRequest,
    occupied: &[OccupiedExtent],
) -> CompactionResult<CompactionPlan> {
    request.validate()?;

    if occupied.len() > MAX_SEGMENTS {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupied extent count {} exceeds maximum {}",
                occupied.len(),
                MAX_SEGMENTS
            ),
        });
    }

    if occupied.is_empty() {
        return CompactionPlan::noop(request);
    }

    let mut objects = occupied.to_vec();

    objects.sort_by(|left, right| {
        left.extent
            .offset
            .cmp(&right.extent.offset)
            .then_with(|| left.object_id.cmp(&right.object_id))
    });

    validate_occupied_extents(request.capacity_bytes, &objects)?;

    let occupied_bytes = objects.iter().try_fold(0u64, |sum, object| {
        sum.checked_add(object.extent.length)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "occupied-byte accounting".to_string(),
            })
    })?;

    if occupied_bytes > request.capacity_bytes {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupied bytes {} exceed capacity {}",
                occupied_bytes, request.capacity_bytes
            ),
        });
    }

    let mut relocations = Vec::new();

    /*
     * `cursor` is the lowest address at which the next movable object may
     * safely be placed while preserving ordering.
     *
     * Fixed/address-sensitive objects create barriers. Movable objects may
     * move into holes before them, but never across a fixed object.
     */
    let mut cursor = 0u64;

    for object in &objects {
        if !object.is_generically_movable() || !request.policy.allow_relocation {
            let end = object.extent.end()?;

            if end > cursor {
                cursor = end;
            }

            continue;
        }

        let destination = MemoryExtent::new(cursor, object.extent.length);

        if destination.offset != object.extent.offset {
            relocations.push(Relocation::new(
                object.object_id,
                object.extent,
                destination,
            ));

            if relocations.len() > request.policy.maximum_relocations {
                return Err(MemoryError::CompactionError {
                    reason: format!(
                        "planned relocation count exceeds policy maximum {}",
                        request.policy.maximum_relocations
                    ),
                });
            }
        }

        cursor = cursor
            .checked_add(object.extent.length)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "compaction destination cursor".to_string(),
            })?;
    }

    let movement_bytes = relocations.iter().try_fold(0u64, |sum, relocation| {
        sum.checked_add(relocation.bytes_moved())
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "compaction movement accounting".to_string(),
            })
    })?;

    if let Some(maximum) = request.policy.maximum_movement_bytes {
        if movement_bytes > maximum {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "planned movement {} bytes exceeds policy maximum {} bytes",
                    movement_bytes, maximum
                ),
            });
        }
    }

    let free_bytes_before = request
        .capacity_bytes
        .checked_sub(occupied_bytes)
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "free-byte accounting".to_string(),
        })?;

    /*
     * For a stable packed plan, all movable objects are packed toward the
     * beginning of each movable region. The exact reclaimed tail is the
     * difference between total capacity and total occupied bytes.
     */
    let contiguous_reclaimable_bytes = free_bytes_before;

    if contiguous_reclaimable_bytes < request.policy.minimum_reclaim_bytes
        && !relocations.is_empty()
    {
        return Ok(CompactionPlan {
            schema_id: COMPACTION_SCHEMA_ID,
            schema_version: COMPACTION_SCHEMA_VERSION,
            representation: request.representation,
            storage: request.storage,
            original_capacity_bytes: request.capacity_bytes,
            occupied_bytes,
            free_bytes_before,
            contiguous_reclaimable_bytes,
            movement_bytes,
            temporary_workspace_bytes: 0,
            relocations,
            preserves_order: request.policy.preserve_order,
            changes_quantum_semantics: false,
        });
    }

    let plan = CompactionPlan {
        schema_id: COMPACTION_SCHEMA_ID,
        schema_version: COMPACTION_SCHEMA_VERSION,
        representation: request.representation,
        storage: request.storage,
        original_capacity_bytes: request.capacity_bytes,
        occupied_bytes,
        free_bytes_before,
        contiguous_reclaimable_bytes,
        movement_bytes,
        temporary_workspace_bytes: 0,
        relocations,
        preserves_order: request.policy.preserve_order,
        changes_quantum_semantics: false,
    };

    plan.validate()?;

    Ok(plan)
}

// =============================================================================
// Validation
// =============================================================================

/// Validates occupied extents.
///
/// Requirements:
///
/// - no zero/negative-length representation;
/// - no extent overflow;
/// - no overlap;
/// - no extent outside capacity;
/// - deterministic object identity;
/// - no duplicate object IDs.
pub fn validate_occupied_extents(
    capacity_bytes: u64,
    occupied: &[OccupiedExtent],
) -> CompactionResult<()> {
    if occupied.len() > MAX_SEGMENTS {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupied extent count {} exceeds maximum {}",
                occupied.len(),
                MAX_SEGMENTS
            ),
        });
    }

    let mut sorted = occupied.to_vec();

    sorted.sort_by(|left, right| {
        left.extent
            .offset
            .cmp(&right.extent.offset)
            .then_with(|| left.object_id.cmp(&right.object_id))
    });

    for pair in sorted.windows(2) {
        let previous = pair[0];
        let current = pair[1];

        if previous.object_id == current.object_id {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "duplicate allocation/object identifier {}",
                    current.object_id
                ),
            });
        }

        if previous.extent.overlaps(&current.extent)? {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "occupied extents for objects {} and {} overlap",
                    previous.object_id, current.object_id
                ),
            });
        }
    }

    for object in &sorted {
        if object.extent.length == 0 {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "object {} has a zero-length occupied extent",
                    object.object_id
                ),
            });
        }

        let end = object.extent.end()?;

        if end > capacity_bytes {
            return Err(MemoryError::CompactionError {
                reason: format!(
                    "object {} ends at byte {}, beyond capacity {}",
                    object.object_id, end, capacity_bytes
                ),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Free extent analysis
// =============================================================================

/// Finds free extents in deterministic ascending order.
pub fn find_free_extents(
    capacity_bytes: u64,
    occupied: &[OccupiedExtent],
) -> CompactionResult<Vec<MemoryExtent>> {
    validate_occupied_extents(capacity_bytes, occupied)?;

    if occupied.len() > MAX_HOLES {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupied extent count {} exceeds hole-analysis limit {}",
                occupied.len(),
                MAX_HOLES
            ),
        });
    }

    if occupied.is_empty() {
        return Ok(vec![MemoryExtent::new(0, capacity_bytes)]);
    }

    let mut objects = occupied.to_vec();

    objects.sort_by(|left, right| {
        left.extent
            .offset
            .cmp(&right.extent.offset)
            .then_with(|| left.object_id.cmp(&right.object_id))
    });

    let mut free = Vec::new();
    let mut cursor = 0u64;

    for object in objects {
        if object.extent.offset > cursor {
            free.push(MemoryExtent::new(
                cursor,
                object.extent.offset - cursor,
            ));
        }

        cursor = object.extent.end()?;
    }

    if cursor < capacity_bytes {
        free.push(MemoryExtent::new(
            cursor,
            capacity_bytes - cursor,
        ));
    }

    Ok(free)
}

/// Calculates deterministic fragmentation statistics.
pub fn fragmentation_stats(
    capacity_bytes: u64,
    occupied: &[OccupiedExtent],
) -> CompactionResult<FragmentationStats> {
    validate_occupied_extents(capacity_bytes, occupied)?;

    let free = find_free_extents(capacity_bytes, occupied)?;

    let occupied_bytes = occupied.iter().try_fold(0u64, |sum, object| {
        sum.checked_add(object.extent.length)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "fragmentation occupied-byte calculation".to_string(),
            })
    })?;

    let largest_free_extent_bytes = free
        .iter()
        .map(|extent| extent.length)
        .max()
        .unwrap_or(0);

    let movable_extent_count = occupied
        .iter()
        .filter(|object| object.is_generically_movable())
        .count();

    FragmentationStats::calculate(
        capacity_bytes,
        occupied_bytes,
        largest_free_extent_bytes,
        free.len(),
        occupied.len(),
        movable_extent_count,
    )
}

// =============================================================================
// Decision
// =============================================================================

/// Decides whether a compaction plan is worthwhile under the supplied policy.
pub fn should_compact(
    request: &CompactionRequest,
    stats: &FragmentationStats,
) -> CompactionResult<bool> {
    request.validate()?;

    if request.capability == CompactionCapability::Unsupported {
        return Ok(false);
    }

    if request.capability == CompactionCapability::ProviderDefined {
        return Ok(false);
    }

    if !request.policy.allow_relocation {
        return Ok(false);
    }

    if stats.reclaimable_bytes < request.policy.minimum_reclaim_bytes {
        return Ok(false);
    }

    if stats.fragmentation_basis_points
        < request.policy.minimum_fragmentation_basis_points
    {
        return Ok(false);
    }

    if stats.movable_extent_count == 0 {
        return Ok(false);
    }

    Ok(true)
}

// =============================================================================
// Safe Vec compaction
// =============================================================================

/// Compacts an owned vector of optional slots in place.
///
/// This operation is lossless with respect to retained elements:
///
/// ```text
/// [Some(A), None, Some(B), None, Some(C)]
///                       │
///                       ▼
/// [Some(A), Some(B), Some(C), None, None]
/// ```
///
/// The relative order of retained values is preserved.
///
/// No quantum semantics are known by this function; it merely moves owned
/// values. State representations must call this only when slot order is
/// representation-safe.
///
/// This implementation uses only safe Rust.
pub fn compact_optional_slots<T>(
    slots: &mut Vec<Option<T>>,
) -> CompactionResult<usize> {
    let original_len = slots.len();

    if original_len == 0 {
        return Ok(0);
    }

    let mut write_index = 0usize;

    for read_index in 0..original_len {
        if slots[read_index].is_some() {
            if read_index != write_index {
                let value = slots[read_index].take();

                if let Some(value) = value {
                    slots[write_index] = Some(value);
                }
            }

            write_index = write_index
                .checked_add(1)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "optional-slot compaction index".to_string(),
                })?;
        }
    }

    let reclaimed_slots = original_len.saturating_sub(write_index);

    for slot in slots.iter_mut().skip(write_index) {
        *slot = None;
    }

    Ok(reclaimed_slots)
}

/// Compacts an owned vector according to a caller-supplied occupancy bitmap.
///
/// `occupied[i] == true` means the corresponding value must be retained.
///
/// The length of `occupied` must equal the length of `values`.
///
/// This function is useful for storage implementations that maintain separate
/// occupancy metadata.
pub fn compact_by_bitmap<T>(
    values: &mut Vec<T>,
    occupied: &[bool],
) -> CompactionResult<usize> {
    if values.len() != occupied.len() {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupancy bitmap length {} does not match value length {}",
                occupied.len(),
                values.len()
            ),
        });
    }

    /*
     * Stable in-place compaction without unsafe pointer manipulation.
     *
     * `Vec::remove` would be O(n²), which is unsuitable for large quantum
     * memory structures. `retain` performs a linear scan and preserves order.
     *
     * The index is tracked separately so the bitmap can be consulted without
     * exposing any raw memory addresses.
     */
    let mut index = 0usize;

    values.retain(|_| {
        let keep = occupied[index];
        index += 1;
        keep
    });

    let retained = values.len();

    Ok(occupied.len().saturating_sub(retained))
}

// =============================================================================
// Byte-buffer compaction
// =============================================================================

/// Compacts fixed-size byte records while preserving record order.
///
/// This function is intentionally record-based rather than a raw overlapping
/// memory copier. It therefore remains entirely safe Rust and can be used by
/// representation-specific modules for metadata records, serialized state
/// blocks, or other host-owned storage.
///
/// `record_size` must be non-zero when records exist.
///
/// `keep[i] == true` retains record `i`.
pub fn compact_fixed_records(
    bytes: &mut Vec<u8>,
    record_size: usize,
    keep: &[bool],
) -> CompactionResult<usize> {
    if record_size == 0 {
        return Err(MemoryError::CompactionError {
            reason: "record size must be non-zero".to_string(),
        });
    }

    if bytes.len() % record_size != 0 {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "byte buffer length {} is not divisible by record size {}",
                bytes.len(),
                record_size
            ),
        });
    }

    let record_count = bytes.len() / record_size;

    if keep.len() != record_count {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "keep bitmap length {} does not match record count {}",
                keep.len(),
                record_count
            ),
        });
    }

    let mut output = Vec::with_capacity(bytes.len());
    let mut retained = 0usize;

    for index in 0..record_count {
        if keep[index] {
            let start = index
                .checked_mul(record_size)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "fixed-record source offset".to_string(),
                })?;

            let end = start
                .checked_add(record_size)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "fixed-record source end".to_string(),
                })?;

            output.extend_from_slice(&bytes[start..end]);

            retained = retained
                .checked_add(1)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "retained record count".to_string(),
                })?;
        }
    }

    let removed = record_count.saturating_sub(retained);

    *bytes = output;

    Ok(removed)
}

// =============================================================================
// Sparse support compaction
// =============================================================================

/// Describes a sparse support entry that can be compacted.
///
/// The index is the representation-local basis/tensor slot, not a logical
/// qubit identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SparseSupportSlot {
    /// Stable storage slot identifier.
    pub slot_id: u64,

    /// Representation-local logical index.
    pub logical_index: u64,

    /// Current storage slot.
    pub storage_slot: usize,

    /// Whether the slot is occupied.
    pub occupied: bool,
}

impl SparseSupportSlot {
    /// Creates a sparse support slot.
    pub const fn new(
        slot_id: u64,
        logical_index: u64,
        storage_slot: usize,
        occupied: bool,
    ) -> Self {
        Self {
            slot_id,
            logical_index,
            storage_slot,
            occupied,
        }
    }
}

/// Compacts sparse support metadata.
///
/// The returned mapping changes storage slots only. `logical_index` remains
/// unchanged.
///
/// This is the intended integration boundary for `sparse.rs`.
pub fn plan_sparse_support_compaction(
    slots: &[SparseSupportSlot],
) -> CompactionResult<Vec<(u64, usize, usize)>> {
    let mut active = slots
        .iter()
        .filter(|slot| slot.occupied)
        .copied()
        .collect::<Vec<_>>();

    if active.len() > MAX_RELOCATION_OPERATIONS {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "sparse support size {} exceeds relocation safety limit {}",
                active.len(),
                MAX_RELOCATION_OPERATIONS
            ),
        });
    }

    active.sort_by(|left, right| {
        left.storage_slot
            .cmp(&right.storage_slot)
            .then_with(|| left.slot_id.cmp(&right.slot_id))
    });

    let mut result = Vec::new();

    for (new_slot, slot) in active.iter().enumerate() {
        if slot.storage_slot != new_slot {
            result.push((slot.slot_id, slot.storage_slot, new_slot));
        }
    }

    Ok(result)
}

// =============================================================================
// Qubit/state semantic guard
// =============================================================================

/// Metadata used to verify that compaction has not changed quantum semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticInvariant {
    /// Logical qubit count before compaction.
    pub qubits: QubitCount,

    /// Number of logical quantum objects.
    pub logical_objects: u64,

    /// Whether the representation is expected to preserve normalization.
    pub normalization_required: bool,

    /// Whether the representation contains classical companion memory.
    pub has_classical_memory: bool,
}

impl SemanticInvariant {
    /// Creates a semantic invariant.
    pub const fn new(
        qubits: QubitCount,
        logical_objects: u64,
        normalization_required: bool,
        has_classical_memory: bool,
    ) -> Self {
        Self {
            qubits,
            logical_objects,
            normalization_required,
            has_classical_memory,
        }
    }

    /// Verifies that a post-compaction invariant is semantically identical.
    pub fn verify_unchanged(
        &self,
        after: &Self,
    ) -> CompactionResult<()> {
        if self != after {
            return Err(MemoryError::CompactionError {
                reason: "compaction changed quantum-memory semantic metadata"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Provider contract
// =============================================================================

/// Provider-neutral compaction interface.
///
/// CPU, GPU, distributed, and backend-specific memory providers can implement
/// this trait without exposing their implementation details to the rest of
/// Zamani.
///
/// The trait is deliberately synchronous. A GPU/distributed implementation may
/// internally use asynchronous mechanisms, but completion must be established
/// before `compact` returns successfully.
///
/// No raw pointers or vendor handles appear in this interface.
pub trait CompactionProvider {
    /// Provider-specific capability.
    fn capability(&self) -> CompactionCapability;

    /// Returns the storage domain.
    fn storage(&self) -> CompactionStorage;

    /// Returns the representation being stored.
    fn representation(&self) -> CompactionRepresentation;

    /// Executes a previously validated plan.
    ///
    /// The provider MUST:
    ///
    /// 1. validate that the plan belongs to its representation/storage domain;
    /// 2. validate that the underlying memory is still compatible;
    /// 3. prevent concurrent mutation during movement;
    /// 4. preserve all logical/physical identities;
    /// 5. complete all required synchronization;
    /// 6. publish the new storage locations atomically according to its
    ///    memory model;
    /// 7. report failure without pretending that compaction completed.
    fn compact(
        &mut self,
        plan: &CompactionPlan,
    ) -> CompactionResult<CompactionReport>;

    /// Returns whether compaction can relocate externally referenced storage.
    fn supports_relocation(&self) -> bool {
        self.capability().permits_generic_compaction()
    }
}

// =============================================================================
// Provider validation
// =============================================================================

/// Validates that a provider is compatible with a plan.
pub fn validate_provider_plan<P: CompactionProvider>(
    provider: &P,
    plan: &CompactionPlan,
) -> CompactionResult<()> {
    plan.validate()?;

    if provider.storage() != plan.storage {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "provider storage domain `{}` does not match plan storage `{}`",
                provider.storage(),
                plan.storage
            ),
        });
    }

    if provider.representation() != plan.representation {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "provider representation `{}` does not match plan representation `{}`",
                provider.representation(),
                plan.representation
            ),
        });
    }

    if !provider.supports_relocation() && !plan.is_noop() {
        return Err(MemoryError::CompactionError {
            reason: "provider does not support relocation but plan contains moves"
                .to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Provider execution helper
// =============================================================================

/// Validates and executes a compaction plan through a provider.
pub fn execute_compaction<P: CompactionProvider>(
    provider: &mut P,
    plan: &CompactionPlan,
) -> CompactionResult<CompactionReport> {
    validate_provider_plan(provider, plan)?;

    let report = provider.compact(plan)?;

    if !report.semantics_preserved {
        return Err(MemoryError::CompactionError {
            reason: "provider reported that compaction changed quantum semantics"
                .to_string(),
        });
    }

    if report.relocated_objects != plan.relocation_count() {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "provider relocated {} objects but plan contained {}",
                report.relocated_objects,
                plan.relocation_count()
            ),
        });
    }

    if report.movement_bytes != plan.movement_bytes {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "provider reported {} moved bytes but plan required {}",
                report.movement_bytes,
                plan.movement_bytes
            ),
        });
    }

    Ok(report)
}

// =============================================================================
// Host provider
// =============================================================================

/// Simple host-side provider for logical extent metadata.
///
/// This provider does not own the actual quantum state. It is useful for
/// allocator/pool integration where the actual storage implementation owns
/// the bytes separately.
///
/// It provides transactional plan acceptance and deterministic accounting.
#[derive(Debug, Clone)]
pub struct HostCompactionProvider {
    /// Representation.
    representation: CompactionRepresentation,

    /// Whether relocation is permitted.
    relocation_supported: bool,

    /// Storage domain.
    storage: CompactionStorage,
}

impl HostCompactionProvider {
    /// Creates a host compaction provider.
    pub const fn new(
        representation: CompactionRepresentation,
        storage: CompactionStorage,
    ) -> Self {
        Self {
            representation,
            relocation_supported: true,
            storage,
        }
    }

    /// Enables or disables generic relocation.
    pub const fn with_relocation(
        mut self,
        supported: bool,
    ) -> Self {
        self.relocation_supported = supported;
        self
    }
}

impl CompactionProvider for HostCompactionProvider {
    fn capability(&self) -> CompactionCapability {
        if self.relocation_supported {
            CompactionCapability::SupportedWithRelocation
        } else {
            CompactionCapability::Unsupported
        }
    }

    fn storage(&self) -> CompactionStorage {
        self.storage
    }

    fn representation(&self) -> CompactionRepresentation {
        self.representation
    }

    fn compact(
        &mut self,
        plan: &CompactionPlan,
    ) -> CompactionResult<CompactionReport> {
        validate_provider_plan(self, plan)?;

        Ok(CompactionReport::from_plan(plan, !plan.is_noop()))
    }
}

// =============================================================================
// Hardware/QPU policy helpers
// =============================================================================

/// Returns the safe generic compaction capability for a storage domain.
///
/// This does not probe hardware. Hardware discovery belongs to hardware
/// adapters.
pub const fn default_capability_for_storage(
    storage: CompactionStorage,
) -> CompactionCapability {
    match storage {
        CompactionStorage::Host
        | CompactionStorage::PinnedHost
        | CompactionStorage::Unified => {
            CompactionCapability::SupportedWithRelocation
        }

        CompactionStorage::Device => {
            /*
             * Device compaction must be implemented by the GPU/device
             * provider because generic host Rust cannot safely manipulate
             * device memory.
             */
            CompactionCapability::ProviderDefined
        }

        CompactionStorage::Distributed => {
            /*
             * Distributed compaction requires coordination with the
             * distributed memory provider.
             */
            CompactionCapability::ProviderDefined
        }

        CompactionStorage::BackendHost => {
            /*
             * Backend integrations decide whether their host-side buffers
             * may be relocated.
             */
            CompactionCapability::ProviderDefined
        }

        CompactionStorage::Remote => {
            /*
             * Remote/QPU-owned memory must never be assumed relocatable.
             */
            CompactionCapability::Unsupported
        }
    }
}

/// Returns whether compaction can be performed without changing a QPU's
/// physical quantum state.
///
/// For real QPU execution, this returns true only for host-side storage
/// associated with the job, never for opaque remote quantum state.
pub const fn qpu_compaction_is_host_only(
    storage: CompactionStorage,
) -> bool {
    matches!(
        storage,
        CompactionStorage::Host
            | CompactionStorage::PinnedHost
            | CompactionStorage::BackendHost
    )
}

// =============================================================================
// Byte accounting
// =============================================================================

/// Converts a `ByteCount` to the internal byte representation.
pub const fn byte_count_value(value: ByteCount) -> u64 {
    value.get()
}

/// Calculates the bytes saved by removing trailing unused capacity.
pub fn calculate_reclaimed_bytes(
    capacity: ByteCount,
    occupied: ByteCount,
) -> CompactionResult<ByteCount> {
    let capacity = capacity.get();
    let occupied = occupied.get();

    if occupied > capacity {
        return Err(MemoryError::CompactionError {
            reason: format!(
                "occupied capacity {} exceeds total capacity {}",
                occupied, capacity
            ),
        });
    }

    Ok(ByteCount::new(capacity - occupied))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn extent(
        object_id: u64,
        offset: u64,
        length: u64,
        movable: bool,
    ) -> OccupiedExtent {
        OccupiedExtent::new(
            object_id,
            offset,
            length,
            movable,
            false,
        )
    }

    #[test]
    fn extent_end_is_checked() {
        let extent = MemoryExtent::new(u64::MAX, 1);

        assert!(matches!(
            extent.end(),
            Err(MemoryError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn overlapping_extents_are_rejected() {
        let objects = [
            extent(1, 0, 100, true),
            extent(2, 50, 100, true),
        ];

        assert!(validate_occupied_extents(1024, &objects).is_err());
    }

    #[test]
    fn duplicate_object_ids_are_rejected() {
        let objects = [
            extent(1, 0, 100, true),
            extent(1, 100, 100, true),
        ];

        assert!(validate_occupied_extents(1024, &objects).is_err());
    }

    #[test]
    fn free_extents_are_deterministic() {
        let objects = [
            extent(2, 200, 100, true),
            extent(1, 0, 100, true),
        ];

        let free = find_free_extents(512, &objects).unwrap();

        assert_eq!(
            free,
            vec![
                MemoryExtent::new(100, 100),
                MemoryExtent::new(300, 212),
            ]
        );
    }

    #[test]
    fn stable_compaction_plan_moves_objects_into_holes() {
        let request = CompactionRequest {
            representation: CompactionRepresentation::Sparse,
            storage: CompactionStorage::Host,
            qubits: Some(QubitCount::new(8)),
            capacity_bytes: 400,
            externally_address_sensitive: false,
            capability: CompactionCapability::SupportedWithRelocation,
            policy: CompactionPolicy::aggressive(),
        };

        let objects = [
            extent(1, 0, 100, true),
            extent(2, 200, 100, true),
        ];

        let plan = plan_compaction(&request, &objects).unwrap();

        assert_eq!(plan.relocation_count(), 1);
        assert_eq!(plan.relocations()[0].object_id, 2);
        assert_eq!(plan.relocations()[0].source.offset, 200);
        assert_eq!(plan.relocations()[0].destination.offset, 100);
        assert_eq!(plan.movement_bytes, 100);
        assert!(plan.is_lossless());
    }

    #[test]
    fn address_sensitive_objects_are_not_relocated() {
        let request = CompactionRequest {
            representation: CompactionRepresentation::BackendNative,
            storage: CompactionStorage::BackendHost,
            qubits: None,
            capacity_bytes: 400,
            externally_address_sensitive: false,
            capability: CompactionCapability::ProviderDefined,
            policy: CompactionPolicy::aggressive(),
        };

        let objects = [
            OccupiedExtent::new(1, 0, 100, false, true),
            extent(2, 200, 100, true),
        ];

        let plan = plan_compaction(&request, &objects).unwrap();

        assert!(plan.relocations().is_empty());
    }

    #[test]
    fn optional_slot_compaction_preserves_order() {
        let mut slots = vec![
            Some("a"),
            None,
            Some("b"),
            None,
            Some("c"),
        ];

        let reclaimed = compact_optional_slots(&mut slots).unwrap();

        assert_eq!(reclaimed, 2);
        assert_eq!(slots, vec![Some("a"), Some("b"), Some("c"), None, None]);
    }

    #[test]
    fn bitmap_compaction_preserves_selected_values() {
        let mut values = vec![1, 2, 3, 4, 5];

        let removed =
            compact_by_bitmap(&mut values, &[true, false, true, false, true])
                .unwrap();

        assert_eq!(removed, 2);
        assert_eq!(values, vec![1, 3, 5]);
    }

    #[test]
    fn fixed_record_compaction_preserves_record_boundaries() {
        let mut bytes = vec![
            1, 2, //
            3, 4, //
            5, 6, //
            7, 8,
        ];

        let removed =
            compact_fixed_records(&mut bytes, 2, &[true, false, true, false])
                .unwrap();

        assert_eq!(removed, 2);
        assert_eq!(bytes, vec![1, 2, 5, 6]);
    }

    #[test]
    fn sparse_support_compaction_does_not_change_logical_index() {
        let slots = [
            SparseSupportSlot::new(10, 7, 0, true),
            SparseSupportSlot::new(20, 99, 4, true),
        ];

        let moves = plan_sparse_support_compaction(&slots).unwrap();

        assert_eq!(moves, vec![(20, 4, 1)]);
    }

    #[test]
    fn semantic_invariant_rejects_changes() {
        let before = SemanticInvariant::new(
            QubitCount::new(8),
            8,
            true,
            true,
        );

        let after = SemanticInvariant::new(
            QubitCount::new(9),
            8,
            true,
            true,
        );

        assert!(before.verify_unchanged(&after).is_err());
    }

    #[test]
    fn host_provider_executes_valid_plan() {
        let request = CompactionRequest {
            representation: CompactionRepresentation::Generic,
            storage: CompactionStorage::Host,
            qubits: None,
            capacity_bytes: 400,
            externally_address_sensitive: false,
            capability: CompactionCapability::SupportedWithRelocation,
            policy: CompactionPolicy::aggressive(),
        };

        let objects = [
            extent(1, 0, 100, true),
            extent(2, 200, 100, true),
        ];

        let plan = plan_compaction(&request, &objects).unwrap();

        let mut provider = HostCompactionProvider::new(
            CompactionRepresentation::Generic,
            CompactionStorage::Host,
        );

        let report = execute_compaction(&mut provider, &plan).unwrap();

        assert_eq!(report.relocated_objects, 1);
        assert_eq!(report.movement_bytes, 100);
        assert!(report.semantics_preserved);
    }

    #[test]
    fn unsupported_remote_storage_is_not_genericly_compactable() {
        assert_eq!(
            default_capability_for_storage(CompactionStorage::Remote),
            CompactionCapability::Unsupported
        );
    }

    #[test]
    fn gpu_compaction_requires_provider() {
        assert_eq!(
            default_capability_for_storage(CompactionStorage::Device),
            CompactionCapability::ProviderDefined
        );
    }

    #[test]
    fn distributed_compaction_requires_provider() {
        assert_eq!(
            default_capability_for_storage(CompactionStorage::Distributed),
            CompactionCapability::ProviderDefined
        );
    }

    #[test]
    fn qpu_compaction_is_host_only() {
        assert!(qpu_compaction_is_host_only(
            CompactionStorage::Host
        ));

        assert!(!qpu_compaction_is_host_only(
            CompactionStorage::Remote
        ));
    }

    #[test]
    fn reclaimed_bytes_are_checked() {
        let reclaimed = calculate_reclaimed_bytes(
            ByteCount::new(1024),
            ByteCount::new(768),
        )
        .unwrap();

        assert_eq!(reclaimed.get(), 256);
    }

    #[test]
    fn occupied_greater_than_capacity_is_rejected() {
        assert!(calculate_reclaimed_bytes(
            ByteCount::new(100),
            ByteCount::new(101)
        )
        .is_err());
    }

    #[test]
    fn noop_plan_is_lossless() {
        let request = CompactionRequest::new(
            CompactionRepresentation::StateVector,
            CompactionStorage::Host,
            1024,
        );

        let plan = CompactionPlan::noop(&request).unwrap();

        assert!(plan.is_noop());
        assert!(plan.is_lossless());
        assert!(plan.validate().is_ok());
    }
}