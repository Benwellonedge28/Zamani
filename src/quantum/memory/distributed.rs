//! Zamani Quantum Memory — Distributed Memory
//!
//! Production-grade, provider-neutral distributed quantum-memory contracts.
//!
//! # Mission
//!
//! `quantum::memory::distributed` defines the memory-domain abstraction needed
//! when a quantum state, quantum-memory object, simulation buffer, logical
//! state, or execution workspace is distributed across multiple ranks,
//! processes, nodes, accelerators, or remote execution resources.
//!
//! This module is deliberately a MEMORY abstraction.
//!
//! It does NOT implement:
//!
//! - MPI;
//! - UCX;
//! - RDMA;
//! - TCP;
//! - InfiniBand;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - QPU provider SDKs;
//! - IBM APIs;
//! - Google APIs;
//! - IonQ APIs;
//! - Quantinuum APIs;
//! - Rigetti APIs;
//! - D-Wave APIs;
//! - Pasqal APIs;
//! - hardware routing;
//! - quantum algorithms;
//! - gate semantics;
//! - circuit parsing;
//! - quantum IR;
//! - QEC decoding;
//! - benchmarking protocols.
//!
//! Those implementations belong to their respective provider, hardware,
//! routing, execution, algorithm, error-correction, or benchmarking layers.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    runtime / simulator
//!                              │
//!                              ▼
//!                       quantum::memory
//!                              │
//!                  ┌───────────┴───────────┐
//!                  │                       │
//!                  ▼                       ▼
//!             local memory          distributed memory
//!                                          │
//!                         ┌────────────────┼────────────────┐
//!                         │                │                │
//!                         ▼                ▼                ▼
//!                       node 0           node 1           node N
//!                         │                │                │
//!                    CPU/GPU/QPU       CPU/GPU/QPU      CPU/GPU/QPU
//! ```
//!
//! The distributed layer provides:
//!
//! - distributed memory-domain identity;
//! - rank/node identity;
//! - topology-independent partitioning;
//! - quantum-state shard ranges;
//! - global/local index translation;
//! - ownership validation;
//! - migration planning;
//! - communication contracts;
//! - collective-operation contracts;
//! - synchronization contracts;
//! - distributed allocation-provider integration;
//! - communication-buffer accounting;
//! - replication metadata;
//! - consistency epochs;
//! - deterministic partition plans;
//! - provider-neutral capabilities;
//! - fault/health state;
//! - checksums and integrity metadata;
//! - bounded resource policies.
//!
//! # Critical architectural rule
//!
//! Distributed memory is NOT synonymous with MPI.
//!
//! MPI is one possible transport/provider.
//!
//! Other possible implementations include:
//!
//! - shared-memory process groups;
//! - RDMA;
//! - UCX;
//! - TCP;
//! - QUIC-like reliable transports;
//! - accelerator peer-to-peer;
//! - CUDA IPC;
//! - HIP IPC;
//! - vendor-neutral device fabrics;
//! - cloud/distributed simulation;
//! - remote simulator backends;
//! - QPU provider-native staging;
//! - future quantum-network transports.
//!
//! The core contracts below deliberately do not depend on any of them.
//!
//! # Quantum-state distribution
//!
//! For a dense state vector with `n` qubits:
//!
//! ```text
//! amplitudes = 2^n
//! ```
//!
//! A distributed plan divides those amplitudes into disjoint shards.
//!
//! ```text
//! global state
//! ┌──────────────────────────────────────────────┐
//! │ shard 0 │ shard 1 │ shard 2 │ ... │ shard N │
//! └──────────────────────────────────────────────┘
//!      │          │          │             │
//!      ▼          ▼          ▼             ▼
//!    rank 0     rank 1     rank 2        rank N
//! ```
//!
//! A partition is represented by a global offset and element count. This
//! permits state-vector, density-matrix, sparse, tensor, and future
//! representations to reuse the same ownership model without embedding
//! representation-specific mathematics here.
//!
//! # Communication principle
//!
//! Distributed state operations must distinguish:
//!
//! 1. local work;
//! 2. peer communication;
//! 3. collective communication;
//! 4. synchronization;
//! 5. migration;
//! 6. replication.
//!
//! The memory layer provides contracts for all six but does not decide which
//! quantum gate requires communication.
//!
//! The state representation/execution layer decides that.
//!
//! # Memory safety
//!
//! This module uses no `unsafe` code.
//!
//! No raw pointers are exposed.
//!
//! No device pointers are represented.
//!
//! No FFI handles are required.
//!
//! Provider implementations may internally use safe wrappers around external
//! runtimes, but unsafe implementation details must not cross this API.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! This file is intentionally written against already-established foundation
//! contracts:
//!
//! ```text
//! quantum::memory::errors
//! quantum::memory::types
//! quantum::memory::allocator
//! ```
//!
//! It therefore does not require future `representation.rs`, `coherence.rs`,
//! `synchronization.rs`, or `migration.rs` to be modified later.
//!
//! Future modules consume this API; they do not redefine these distributed
//! concepts.
//!
//! # Allocator integration
//!
//! `allocator.rs` already defines:
//!
//! ```text
//! MemoryLocation::Distributed { domain_id }
//! MemoryProvider
//! ProviderAllocation
//! AllocationRequest
//! AllocationClass
//! MemoryAllocator
//! ```
//!
//! This module supplies `DistributedMemoryProvider`, which can be adapted to
//! that generic allocator boundary without making the allocator aware of
//! distributed transport semantics.
//!
//! # Hardware integration
//!
//! Hardware-specific providers should map their execution resources to:
//!
//! ```text
//! DistributedDomain
//!     ├── NodeDescriptor
//!     ├── DeviceDescriptor
//!     ├── RankDescriptor
//!     └── DistributedCapabilities
//! ```
//!
//! A QPU that does not expose a user-addressable quantum-state memory space
//! MUST NOT pretend that it does. Such a backend can use distributed memory
//! for:
//!
//! - classical staging;
//! - job metadata;
//! - measurement/result buffers;
//! - compilation artifacts;
//! - checkpoint metadata;
//! - provider-native opaque handles.
//!
//! Actual quantum state remains provider-owned.
//!
//! # File completion invariant
//!
//! This file is complete when:
//!
//! - no transport SDK is imported;
//! - no vendor is hard-coded;
//! - arbitrary rank counts are representable;
//! - partition arithmetic is checked;
//! - ownership is deterministic;
//! - global/local index conversion is checked;
//! - communication payloads are explicitly bounded;
//! - collective operations are represented explicitly;
//! - failures are returned as `MemoryError`;
//! - no raw address is exposed;
//! - no unsafe code exists;
//! - allocator integration is provider-neutral;
//! - CPU/GPU/QPU resources can be described;
//! - state-vector and non-state-vector representations can use the same
//!   partition model;
//! - tests cover partitioning, ownership, overflow, serialization-neutral
//!   metadata, communication validation, and concurrency-safe provider state.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use super::allocator::{
    AllocationClass, AllocationRequest, MemoryLocation, MemoryProvider,
    ProviderAllocation, ProviderAvailability,
};
use super::errors::MemoryError;
use super::types::{ByteCount, MemoryId};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const DISTRIBUTED_MEMORY_SCHEMA_ID: &str =
    "zamani.quantum.memory.distributed";

/// Distributed-memory API major version.
pub const DISTRIBUTED_MEMORY_API_MAJOR: u16 = 1;

/// Distributed-memory API minor version.
pub const DISTRIBUTED_MEMORY_API_MINOR: u16 = 0;

/// Distributed-memory API patch version.
pub const DISTRIBUTED_MEMORY_API_PATCH: u16 = 0;

/// Maximum provider/domain identifier length.
///
/// This is deliberately bounded so diagnostics and registry metadata cannot
/// accidentally retain unbounded strings.
pub const MAX_IDENTIFIER_LENGTH: usize = 256;

/// Maximum communication payload accepted by the generic contract.
///
/// This is not a universal hardware limit. A concrete provider may impose a
/// lower limit. The value exists to prevent accidental construction of
/// absurdly large metadata/message objects.
pub const MAX_MESSAGE_BYTES: u64 = u64::MAX / 4;

// =============================================================================
// Identifiers
// =============================================================================

/// Stable distributed-domain identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DistributedDomainId(u64);

impl DistributedDomainId {
    /// Invalid/unassigned domain identifier.
    pub const INVALID: Self = Self(0);

    /// Creates an identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns whether this identifier is valid.
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for DistributedDomainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "domain-{}", self.0)
    }
}

/// Distributed rank identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RankId(u32);

impl RankId {
    /// Creates a rank.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric rank.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for RankId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rank-{}", self.0)
    }
}

/// Physical/logical node identifier.
///
/// A node may contain multiple ranks and multiple accelerators.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeId(u64);

impl NodeId {
    /// Creates a node identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "node-{}", self.0)
    }
}

/// Distributed shard identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShardId(u64);

impl ShardId {
    /// Creates a shard identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shard-{}", self.0)
    }
}

// =============================================================================
// Global/local indices
// =============================================================================

/// Global element index inside a distributed memory object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GlobalIndex(u64);

impl GlobalIndex {
    /// Creates a global index.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the index.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Local element index inside a shard.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalIndex(u64);

impl LocalIndex {
    /// Creates a local index.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the index.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Distributed device/resource description
// =============================================================================

/// Broad execution-resource kind associated with a distributed rank.
///
/// This intentionally does not name vendors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DistributedResourceKind {
    /// CPU-only resource.
    Cpu,

    /// Generic accelerator.
    Accelerator,

    /// GPU-like accelerator.
    Gpu,

    /// Quantum processing unit.
    Qpu,

    /// Simulator.
    Simulator,

    /// Hardware emulator.
    Emulator,

    /// Mixed CPU/accelerator/QPU resource.
    Hybrid,

    /// Provider-owned remote execution resource.
    Remote,
}

impl DistributedResourceKind {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Accelerator => "accelerator",
            Self::Gpu => "gpu",
            Self::Qpu => "qpu",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Hybrid => "hybrid",
            Self::Remote => "remote",
        }
    }
}

impl fmt::Display for DistributedResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Describes a resource participating in a distributed domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedResource {
    /// Stable resource identifier within the domain.
    pub resource_id: String,

    /// Owning node.
    pub node_id: NodeId,

    /// Resource class.
    pub kind: DistributedResourceKind,

    /// Optional local-memory capacity.
    pub memory_bytes: Option<u64>,

    /// Whether this resource can hold state memory.
    pub state_memory: bool,

    /// Whether peer communication is supported.
    pub peer_communication: bool,
}

impl DistributedResource {
    /// Validates the resource descriptor.
    pub fn validate(&self) -> Result<(), MemoryError> {
        validate_identifier(&self.resource_id, "resource-id")?;

        if let Some(bytes) = self.memory_bytes {
            if bytes == 0 {
                return Err(MemoryError::InvalidArgument {
                    reason: "resource memory capacity cannot be zero when supplied"
                        .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Node/rank descriptors
// =============================================================================

/// Distributed rank descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RankDescriptor {
    /// Rank identity.
    pub rank: RankId,

    /// Owning node.
    pub node: NodeId,

    /// Resources assigned to the rank.
    pub resources: Vec<DistributedResource>,

    /// Whether the rank is currently healthy.
    pub healthy: bool,
}

impl RankDescriptor {
    /// Validates this descriptor.
    pub fn validate(&self, world_size: u32) -> Result<(), MemoryError> {
        if self.rank.get() >= world_size {
            return Err(MemoryError::OutOfBounds {
                index: self.rank.get() as u64,
                bound: world_size as u64,
                context: "rank descriptor".to_owned(),
            });
        }

        for resource in &self.resources {
            resource.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Capabilities
// =============================================================================

/// Provider-neutral distributed-memory capabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DistributedCapabilities {
    /// Point-to-point communication.
    pub point_to_point: bool,

    /// Broadcast collective.
    pub broadcast: bool,

    /// Scatter collective.
    pub scatter: bool,

    /// Gather collective.
    pub gather: bool,

    /// All-gather collective.
    pub all_gather: bool,

    /// Reduce collective.
    pub reduce: bool,

    /// All-reduce collective.
    pub all_reduce: bool,

    /// Barrier synchronization.
    pub barrier: bool,

    /// Peer-to-peer device communication.
    pub peer_to_peer: bool,

    /// Remote memory access.
    pub remote_memory_access: bool,

    /// Non-blocking communication.
    pub asynchronous_communication: bool,

    /// Fault detection.
    pub fault_detection: bool,

    /// Replication.
    pub replication: bool,
}

impl DistributedCapabilities {
    /// Conservative capability set for a single-rank implementation.
    pub const fn serial() -> Self {
        Self {
            point_to_point: false,
            broadcast: true,
            scatter: true,
            gather: true,
            all_gather: true,
            reduce: true,
            all_reduce: true,
            barrier: true,
            peer_to_peer: false,
            remote_memory_access: false,
            asynchronous_communication: false,
            fault_detection: false,
            replication: false,
        }
    }

    /// Validates whether a requested operation is supported.
    pub fn require(
        &self,
        operation: CollectiveOperation,
    ) -> Result<(), MemoryError> {
        let supported = match operation {
            CollectiveOperation::Barrier => self.barrier,
            CollectiveOperation::Broadcast => self.broadcast,
            CollectiveOperation::Scatter => self.scatter,
            CollectiveOperation::Gather => self.gather,
            CollectiveOperation::AllGather => self.all_gather,
            CollectiveOperation::Reduce => self.reduce,
            CollectiveOperation::AllReduce => self.all_reduce,
        };

        if supported {
            Ok(())
        } else {
            Err(MemoryError::UnsupportedOperation {
                operation: format!(
                    "distributed collective `{}`",
                    operation.as_str()
                ),
            })
        }
    }
}

// =============================================================================
// Domain
// =============================================================================

/// Immutable description of a distributed memory domain.
///
/// A domain is the universe of ranks participating in one distributed memory
/// operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedDomain {
    /// Domain identity.
    pub id: DistributedDomainId,

    /// Human-readable safe name.
    pub name: String,

    /// Number of ranks.
    pub world_size: u32,

    /// Rank descriptors indexed by rank.
    pub ranks: BTreeMap<RankId, RankDescriptor>,

    /// Domain capabilities.
    pub capabilities: DistributedCapabilities,
}

impl DistributedDomain {
    /// Creates a domain.
    pub fn new(
        id: DistributedDomainId,
        name: impl Into<String>,
        world_size: u32,
        capabilities: DistributedCapabilities,
    ) -> Result<Self, MemoryError> {
        if !id.is_valid() {
            return Err(MemoryError::InvalidIdentifier {
                kind: "distributed-domain-id".to_owned(),
                identifier: id.get().to_string(),
                context: None,
            });
        }

        if world_size == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed world size must be greater than zero"
                    .to_owned(),
            });
        }

        let name = name.into();
        validate_identifier(&name, "distributed-domain-name")?;

        Ok(Self {
            id,
            name,
            world_size,
            ranks: BTreeMap::new(),
            capabilities,
        })
    }

    /// Adds or replaces a rank descriptor.
    pub fn with_rank(
        mut self,
        descriptor: RankDescriptor,
    ) -> Result<Self, MemoryError> {
        descriptor.validate(self.world_size)?;

        let rank = descriptor.rank;

        if self.ranks.contains_key(&rank) {
            return Err(MemoryError::InvalidArgument {
                reason: format!("duplicate distributed rank {}", rank.get()),
            });
        }

        self.ranks.insert(rank, descriptor);

        Ok(self)
    }

    /// Validates that all ranks are represented exactly once.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.world_size == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed world size cannot be zero".to_owned(),
            });
        }

        if self.ranks.len() != self.world_size as usize {
            return Err(MemoryError::InvalidArgument {
                reason: format!(
                    "domain declares {} ranks but contains {} descriptors",
                    self.world_size,
                    self.ranks.len()
                ),
            });
        }

        for rank in 0..self.world_size {
            let id = RankId::new(rank);

            match self.ranks.get(&id) {
                Some(descriptor) => descriptor.validate(self.world_size)?,
                None => {
                    return Err(MemoryError::InvalidArgument {
                        reason: format!("missing descriptor for rank {rank}"),
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns a rank descriptor.
    pub fn rank(
        &self,
        rank: RankId,
    ) -> Result<&RankDescriptor, MemoryError> {
        self.ranks.get(&rank).ok_or_else(|| MemoryError::OutOfBounds {
            index: rank.get() as u64,
            bound: self.world_size as u64,
            context: "distributed rank".to_owned(),
        })
    }
}

// =============================================================================
// Partition
// =============================================================================

/// A contiguous global-memory shard.
///
/// The range is:
///
/// ```text
/// [offset, offset + length)
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryShard {
    /// Stable shard identifier.
    pub id: ShardId,

    /// Owning rank.
    pub owner: RankId,

    /// Global starting element.
    pub offset: GlobalIndex,

    /// Number of elements.
    pub length: u64,
}

impl MemoryShard {
    /// Creates a shard after validating the range.
    pub fn new(
        id: ShardId,
        owner: RankId,
        offset: GlobalIndex,
        length: u64,
    ) -> Result<Self, MemoryError> {
        offset
            .get()
            .checked_add(length)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed shard end".to_owned(),
            })?;

        Ok(Self {
            id,
            owner,
            offset,
            length,
        })
    }

    /// Exclusive global end.
    pub fn end(self) -> Result<u64, MemoryError> {
        self.offset
            .get()
            .checked_add(self.length)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed shard end".to_owned(),
            })
    }

    /// Returns whether a global index belongs to this shard.
    pub fn contains(self, index: GlobalIndex) -> bool {
        match self.end() {
            Ok(end) => index.get() >= self.offset.get() && index.get() < end,
            Err(_) => false,
        }
    }

    /// Converts a global index into a local shard index.
    pub fn local_index(
        self,
        index: GlobalIndex,
    ) -> Result<LocalIndex, MemoryError> {
        if !self.contains(index) {
            return Err(MemoryError::OutOfBounds {
                index: index.get(),
                bound: self.length,
                context: format!("global index outside {}", self.id),
            });
        }

        Ok(LocalIndex::new(index.get() - self.offset.get()))
    }

    /// Returns the memory footprint of this shard.
    pub fn byte_size(self, element_bytes: u64) -> Result<ByteCount, MemoryError> {
        let bytes = self
            .length
            .checked_mul(element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed shard byte size".to_owned(),
            })?;

        Ok(ByteCount::new(bytes))
    }
}

// =============================================================================
// Partition plan
// =============================================================================

/// Deterministic contiguous partition plan.
///
/// This is intentionally representation-neutral.
///
/// It can partition:
///
/// - state-vector amplitudes;
/// - density-matrix elements;
/// - sparse storage ranges;
/// - tensor storage;
/// - classical buffers;
/// - measurement buffers;
/// - checkpoint payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionPlan {
    /// Total number of globally addressable elements.
    pub total_elements: u64,

    /// Ordered shards.
    pub shards: Vec<MemoryShard>,

    /// Number of ranks.
    pub world_size: u32,
}

impl PartitionPlan {
    /// Builds a balanced contiguous partition.
    ///
    /// The first `remainder` ranks receive one extra element.
    pub fn balanced(
        total_elements: u64,
        world_size: u32,
    ) -> Result<Self, MemoryError> {
        if world_size == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "partition world size cannot be zero".to_owned(),
            });
        }

        if total_elements == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "cannot partition zero elements".to_owned(),
            });
        }

        let world = u64::from(world_size);

        let base = total_elements / world;
        let remainder = total_elements % world;

        let mut shards = Vec::with_capacity(world_size as usize);
        let mut offset = 0u64;

        for rank in 0..world_size {
            let extra = if u64::from(rank) < remainder {
                1
            } else {
                0
            };

            let length = base
                .checked_add(extra)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "partition shard length".to_owned(),
                })?;

            if length == 0 {
                return Err(MemoryError::InvalidArgument {
                    reason: format!(
                        "world size {world_size} exceeds total elements \
                         {total_elements}"
                    ),
                });
            }

            let shard = MemoryShard::new(
                ShardId::new(u64::from(rank)),
                RankId::new(rank),
                GlobalIndex::new(offset),
                length,
            )?;

            offset = offset
                .checked_add(length)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "partition offset".to_owned(),
                })?;

            shards.push(shard);
        }

        let plan = Self {
            total_elements,
            shards,
            world_size,
        };

        plan.validate()?;

        Ok(plan)
    }

    /// Builds a state-vector partition for `qubit_count` qubits.
    ///
    /// This performs only checked arithmetic. It does not allocate state
    /// memory.
    pub fn for_qubits(
        qubit_count: u32,
        world_size: u32,
    ) -> Result<Self, MemoryError> {
        if qubit_count >= 64 {
            return Err(MemoryError::ArithmeticOverflow {
                operation: format!(
                    "state-vector amplitude count 2^{qubit_count}"
                ),
            });
        }

        let total = 1u64
            .checked_shl(qubit_count)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "state-vector amplitude count".to_owned(),
            })?;

        Self::balanced(total, world_size)
    }

    /// Returns the shard owning a global index.
    pub fn owner_of(
        &self,
        index: GlobalIndex,
    ) -> Result<RankId, MemoryError> {
        self.shard_of(index).map(|shard| shard.owner)
    }

    /// Returns the shard owning a global index.
    pub fn shard_of(
        &self,
        index: GlobalIndex,
    ) -> Result<MemoryShard, MemoryError> {
        if index.get() >= self.total_elements {
            return Err(MemoryError::OutOfBounds {
                index: index.get(),
                bound: self.total_elements,
                context: "distributed global index".to_owned(),
            });
        }

        // Binary search is deterministic and avoids requiring power-of-two
        // rank counts.
        let mut low = 0usize;
        let mut high = self.shards.len();

        while low < high {
            let middle = low + (high - low) / 2;
            let shard = self.shards[middle];

            if index.get() < shard.offset.get() {
                high = middle;
            } else if index.get() >= shard.end()? {
                low = middle + 1;
            } else {
                return Ok(shard);
            }
        }

        Err(MemoryError::InvariantViolation {
            reason: format!(
                "partition plan contains no shard for global index {}",
                index.get()
            ),
        })
    }

    /// Converts a global index into an owning rank and local index.
    pub fn locate(
        &self,
        index: GlobalIndex,
    ) -> Result<(RankId, LocalIndex), MemoryError> {
        let shard = self.shard_of(index)?;
        let local = shard.local_index(index)?;
        Ok((shard.owner, local))
    }

    /// Returns the shard owned by a rank.
    pub fn shard_for_rank(
        &self,
        rank: RankId,
    ) -> Result<MemoryShard, MemoryError> {
        self.shards
            .iter()
            .copied()
            .find(|shard| shard.owner == rank)
            .ok_or_else(|| MemoryError::OutOfBounds {
                index: u64::from(rank.get()),
                bound: u64::from(self.world_size),
                context: "distributed rank partition".to_owned(),
            })
    }

    /// Validates the entire partition.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.world_size == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "partition world size cannot be zero".to_owned(),
            });
        }

        if self.shards.len() != self.world_size as usize {
            return Err(MemoryError::InvalidArgument {
                reason: format!(
                    "partition has {} shards for {} ranks",
                    self.shards.len(),
                    self.world_size
                ),
            });
        }

        let mut expected_offset = 0u64;

        for (position, shard) in self.shards.iter().copied().enumerate() {
            if shard.owner.get() >= self.world_size {
                return Err(MemoryError::InvalidArgument {
                    reason: format!(
                        "shard {} has invalid owner {}",
                        position,
                        shard.owner.get()
                    ),
                });
            }

            if shard.offset.get() != expected_offset {
                return Err(MemoryError::InvariantViolation {
                    reason: format!(
                        "partition gap/overlap at shard {}: expected offset {}, \
                         found {}",
                        position,
                        expected_offset,
                        shard.offset.get()
                    ),
                });
            }

            if shard.length == 0 {
                return Err(MemoryError::InvalidArgument {
                    reason: format!("shard {} has zero length", position),
                });
            }

            expected_offset = shard.end()?;
        }

        if expected_offset != self.total_elements {
            return Err(MemoryError::InvariantViolation {
                reason: format!(
                    "partition ends at {}, expected {}",
                    expected_offset, self.total_elements
                ),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Communication
// =============================================================================

/// Unique operation identifier inside a distributed execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationId(u64);

impl OperationId {
    /// Creates an operation identifier.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Point-to-point distributed-memory message.
///
/// The payload is opaque to this module. State representations determine its
/// encoding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedMessage {
    /// Operation identity.
    pub operation: OperationId,

    /// Source rank.
    pub source: RankId,

    /// Destination rank.
    pub destination: RankId,

    /// Optional shard associated with the message.
    pub shard: Option<ShardId>,

    /// Logical consistency epoch.
    pub epoch: u64,

    /// Payload.
    pub payload: Vec<u8>,
}

impl DistributedMessage {
    /// Creates a message.
    pub fn new(
        operation: OperationId,
        source: RankId,
        destination: RankId,
        shard: Option<ShardId>,
        epoch: u64,
        payload: Vec<u8>,
    ) -> Result<Self, MemoryError> {
        if u64::try_from(payload.len()).unwrap_or(u64::MAX) > MAX_MESSAGE_BYTES {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed message exceeds maximum payload size"
                    .to_owned(),
            });
        }

        if source == destination {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed point-to-point message cannot target its \
                         own source rank"
                    .to_owned(),
            });
        }

        Ok(Self {
            operation,
            source,
            destination,
            shard,
            epoch,
            payload,
        })
    }

    /// Returns payload size.
    pub fn byte_len(&self) -> ByteCount {
        ByteCount::new(self.payload.len() as u64)
    }
}

/// Abstract distributed transport.
///
/// Concrete implementations may use MPI, UCX, RDMA, shared memory, sockets,
/// accelerator fabrics, cloud transports, or a provider-specific mechanism.
///
/// No transport implementation is embedded in this module.
pub trait DistributedTransport: Send + Sync + 'static {
    /// Returns the local rank.
    fn rank(&self) -> RankId;

    /// Returns the total number of ranks.
    fn world_size(&self) -> u32;

    /// Returns transport capabilities.
    fn capabilities(&self) -> DistributedCapabilities;

    /// Sends one point-to-point message.
    fn send(&self, message: DistributedMessage) -> Result<(), MemoryError>;

    /// Receives the next matching message.
    ///
    /// The implementation may use provider-specific matching internally.
    fn receive(
        &self,
        operation: OperationId,
        source: Option<RankId>,
        epoch: u64,
    ) -> Result<DistributedMessage, MemoryError>;

    /// Synchronizes all ranks.
    fn barrier(&self, operation: OperationId) -> Result<(), MemoryError>;

    /// Broadcasts bytes from the root rank.
    fn broadcast(
        &self,
        operation: OperationId,
        root: RankId,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, MemoryError>;

    /// Reduces unsigned integer values.
    fn all_reduce_u64(
        &self,
        operation: OperationId,
        value: u64,
    ) -> Result<u64, MemoryError>;
}

// =============================================================================
// Collective operations
// =============================================================================

/// Supported provider-neutral collective operation kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CollectiveOperation {
    /// Synchronize ranks.
    Barrier,

    /// Broadcast from one rank.
    Broadcast,

    /// Scatter from root.
    Scatter,

    /// Gather to root.
    Gather,

    /// All-gather to all ranks.
    AllGather,

    /// Reduce to root.
    Reduce,

    /// All-reduce to all ranks.
    AllReduce,
}

impl CollectiveOperation {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Barrier => "barrier",
            Self::Broadcast => "broadcast",
            Self::Scatter => "scatter",
            Self::Gather => "gather",
            Self::AllGather => "all_gather",
            Self::Reduce => "reduce",
            Self::AllReduce => "all_reduce",
        }
    }
}

impl fmt::Display for CollectiveOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Transfer planning
// =============================================================================

/// Describes a required movement of a shard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transfer {
    /// Source rank.
    pub source: RankId,

    /// Destination rank.
    pub destination: RankId,

    /// Shard being transferred.
    pub shard: ShardId,

    /// Global offset.
    pub offset: GlobalIndex,

    /// Element count.
    pub elements: u64,

    /// Bytes per element.
    pub element_bytes: u64,
}

impl Transfer {
    /// Creates a transfer descriptor.
    pub fn new(
        source: RankId,
        destination: RankId,
        shard: ShardId,
        offset: GlobalIndex,
        elements: u64,
        element_bytes: u64,
    ) -> Result<Self, MemoryError> {
        if source == destination {
            return Err(MemoryError::InvalidArgument {
                reason: "transfer source and destination must differ".to_owned(),
            });
        }

        if elements == 0 || element_bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed transfer cannot contain zero elements or \
                         zero element size"
                    .to_owned(),
            });
        }

        elements
            .checked_mul(element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed transfer byte size".to_owned(),
            })?;

        Ok(Self {
            source,
            destination,
            shard,
            offset,
            elements,
            element_bytes,
        })
    }

    /// Number of bytes transferred.
    pub fn bytes(self) -> Result<ByteCount, MemoryError> {
        let bytes = self
            .elements
            .checked_mul(self.element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed transfer byte count".to_owned(),
            })?;

        Ok(ByteCount::new(bytes))
    }
}

/// Deterministic migration plan.
///
/// The plan contains no executable communication and is safe to inspect before
/// committing any memory movement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPlan {
    /// Source partition.
    pub source: PartitionPlan,

    /// Destination partition.
    pub destination: PartitionPlan,

    /// Planned transfers.
    pub transfers: Vec<Transfer>,
}

impl MigrationPlan {
    /// Builds a plan between two contiguous partitions of equal global size.
    ///
    /// This produces a conservative overlap-derived plan. It does not require
    /// the source and destination world sizes to be equal.
    pub fn build(
        source: PartitionPlan,
        destination: PartitionPlan,
        element_bytes: u64,
    ) -> Result<Self, MemoryError> {
        source.validate()?;
        destination.validate()?;

        if source.total_elements != destination.total_elements {
            return Err(MemoryError::StateDimensionMismatch {
                expected: source.total_elements as usize,
                actual: destination.total_elements as usize,
                context: "distributed migration total elements".to_owned(),
            });
        }

        if element_bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "migration element size cannot be zero".to_owned(),
            });
        }

        let mut transfers = Vec::new();

        for src in source.shards.iter().copied() {
            let src_end = src.end()?;

            for dst in destination.shards.iter().copied() {
                let dst_end = dst.end()?;

                let start = src.offset.get().max(dst.offset.get());
                let end = src_end.min(dst_end);

                if start < end && src.owner != dst.owner {
                    let elements = end - start;

                    transfers.push(Transfer::new(
                        src.owner,
                        dst.owner,
                        src.id,
                        GlobalIndex::new(start),
                        elements,
                        element_bytes,
                    )?);
                }
            }
        }

        let plan = Self {
            source,
            destination,
            transfers,
        };

        plan.validate()?;

        Ok(plan)
    }

    /// Validates transfer consistency.
    pub fn validate(&self) -> Result<(), MemoryError> {
        self.source.validate()?;
        self.destination.validate()?;

        if self.source.total_elements != self.destination.total_elements {
            return Err(MemoryError::InvariantViolation {
                reason: "migration partitions have different total sizes"
                    .to_owned(),
            });
        }

        for transfer in &self.transfers {
            let source_shard = self.source.shard_of(transfer.offset)?;
            let destination_shard =
                self.destination.shard_of(transfer.offset)?;

            if source_shard.owner != transfer.source {
                return Err(MemoryError::InvariantViolation {
                    reason: format!(
                        "transfer source {} does not own {}",
                        transfer.source, transfer.shard
                    ),
                });
            }

            if destination_shard.owner != transfer.destination {
                return Err(MemoryError::InvariantViolation {
                    reason: format!(
                        "transfer destination {} does not own global index {}",
                        transfer.destination,
                        transfer.offset.get()
                    ),
                });
            }
        }

        Ok(())
    }

    /// Total communication volume.
    pub fn total_bytes(&self) -> Result<ByteCount, MemoryError> {
        let mut total = 0u64;

        for transfer in &self.transfers {
            total = total
                .checked_add(transfer.bytes()?.get())
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "migration total bytes".to_owned(),
                })?;
        }

        Ok(ByteCount::new(total))
    }
}

// =============================================================================
// Replication
// =============================================================================

/// Replication policy for distributed memory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReplicationPolicy {
    /// No replication.
    None,

    /// One additional copy.
    Single,

    /// Requested replication factor.
    Factor(u16),
}

impl ReplicationPolicy {
    /// Returns the total number of copies implied by this policy.
    pub const fn copies(self) -> u32 {
        match self {
            Self::None => 1,
            Self::Single => 2,
            Self::Factor(value) => {
                if value == 0 {
                    1
                } else {
                    value as u32
                }
            }
        }
    }

    /// Validates the policy.
    pub const fn validate(self) -> bool {
        match self {
            Self::None | Self::Single => true,
            Self::Factor(value) => value >= 1,
        }
    }
}

/// Replica assignment for one shard.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShardReplicaSet {
    /// Primary owner.
    pub primary: RankId,

    /// Additional replicas.
    pub replicas: Vec<RankId>,
}

impl ShardReplicaSet {
    /// Validates replica uniqueness.
    pub fn validate(&self) -> Result<(), MemoryError> {
        for replica in &self.replicas {
            if *replica == self.primary {
                return Err(MemoryError::InvalidArgument {
                    reason: "primary rank cannot also be a replica".to_owned(),
                });
            }
        }

        for (index, left) in self.replicas.iter().enumerate() {
            for right in self.replicas.iter().skip(index + 1) {
                if left == right {
                    return Err(MemoryError::InvalidArgument {
                        reason: "duplicate distributed replica rank".to_owned(),
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Consistency
// =============================================================================

/// Distributed-memory consistency state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConsistencyState {
    /// All replicas are known to be synchronized.
    Synchronized,

    /// One or more replicas contain newer data.
    Dirty,

    /// The state cannot currently be proven consistent.
    Unknown,

    /// The domain has detected corruption or an unrecoverable divergence.
    Invalid,
}

/// Logical consistency epoch.
///
/// Epochs provide deterministic metadata without relying on wall-clock time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConsistencyEpoch(u64);

impl ConsistencyEpoch {
    /// Initial epoch.
    pub const ZERO: Self = Self(0);

    /// Creates an epoch.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the value.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Advances by one epoch.
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

// =============================================================================
// Distributed memory object
// =============================================================================

/// Metadata describing one distributed memory object.
///
/// This structure owns no bytes. It describes ownership and storage policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedMemoryDescriptor {
    /// Memory-domain identity.
    pub memory_id: MemoryId,

    /// Distributed-domain identity.
    pub domain_id: DistributedDomainId,

    /// Total logical elements.
    pub total_elements: u64,

    /// Bytes per element.
    pub element_bytes: u64,

    /// Partition plan.
    pub partition: PartitionPlan,

    /// Replication policy.
    pub replication: ReplicationPolicy,

    /// Consistency state.
    pub consistency: ConsistencyState,

    /// Current consistency epoch.
    pub epoch: ConsistencyEpoch,
}

impl DistributedMemoryDescriptor {
    /// Creates a descriptor.
    pub fn new(
        memory_id: MemoryId,
        domain_id: DistributedDomainId,
        total_elements: u64,
        element_bytes: u64,
        partition: PartitionPlan,
    ) -> Result<Self, MemoryError> {
        if memory_id.get() == 0 {
            return Err(MemoryError::InvalidIdentifier {
                kind: "memory-id".to_owned(),
                identifier: "0".to_owned(),
                context: None,
            });
        }

        if !domain_id.is_valid() {
            return Err(MemoryError::InvalidIdentifier {
                kind: "distributed-domain-id".to_owned(),
                identifier: "0".to_owned(),
                context: None,
            });
        }

        if total_elements == 0 || element_bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed memory dimensions must be non-zero"
                    .to_owned(),
            });
        }

        partition.validate()?;

        if partition.total_elements != total_elements {
            return Err(MemoryError::StateDimensionMismatch {
                expected: total_elements as usize,
                actual: partition.total_elements as usize,
                context: "distributed descriptor partition".to_owned(),
            });
        }

        total_elements
            .checked_mul(element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed memory total bytes".to_owned(),
            })?;

        Ok(Self {
            memory_id,
            domain_id,
            total_elements,
            element_bytes,
            partition,
            replication: ReplicationPolicy::None,
            consistency: ConsistencyState::Synchronized,
            epoch: ConsistencyEpoch::ZERO,
        })
    }

    /// Returns total logical bytes.
    pub fn total_bytes(&self) -> Result<ByteCount, MemoryError> {
        let bytes = self
            .total_elements
            .checked_mul(self.element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed memory total bytes".to_owned(),
            })?;

        Ok(ByteCount::new(bytes))
    }

    /// Locates an element.
    pub fn locate(
        &self,
        index: GlobalIndex,
    ) -> Result<(RankId, LocalIndex), MemoryError> {
        self.partition.locate(index)
    }

    /// Returns the shard for a rank.
    pub fn shard_for_rank(
        &self,
        rank: RankId,
    ) -> Result<MemoryShard, MemoryError> {
        self.partition.shard_for_rank(rank)
    }

    /// Returns the next consistency epoch.
    pub fn next_epoch(&self) -> Result<ConsistencyEpoch, MemoryError> {
        self.epoch.checked_next().ok_or_else(|| {
            MemoryError::ArithmeticOverflow {
                operation: "distributed consistency epoch".to_owned(),
            }
        })
    }
}

// =============================================================================
// Distributed provider allocation
// =============================================================================

/// Provider-owned distributed allocation metadata.
///
/// The actual storage remains owned by the concrete distributed provider.
pub struct DistributedProviderAllocation {
    bytes: u64,
    location: MemoryLocation,
    domain_id: DistributedDomainId,
    shard: Option<ShardId>,
    label: String,
}

impl fmt::Debug for DistributedProviderAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DistributedProviderAllocation")
            .field("bytes", &self.bytes)
            .field("location", &self.location)
            .field("domain_id", &self.domain_id)
            .field("shard", &self.shard)
            .field("label", &self.label)
            .finish()
    }
}

impl ProviderAllocation for DistributedProviderAllocation {
    fn byte_len(&self) -> u64 {
        self.bytes
    }

    fn location(&self) -> MemoryLocation {
        self.location.clone()
    }

    fn resource_label(&self) -> Option<&str> {
        Some(&self.label)
    }
}

// =============================================================================
// Distributed allocation provider
// =============================================================================

/// Provider contract for actual distributed storage.
///
/// This trait is intentionally narrower than `DistributedTransport`.
///
/// A provider answers:
///
/// - where storage lives;
/// - how much storage can be allocated;
/// - how storage is allocated;
/// - how storage is released.
///
/// Communication remains a separate concern.
pub trait DistributedMemoryProvider: Send + Sync + 'static {
    /// Stable provider identifier.
    fn provider_id(&self) -> &str;

    /// Domain represented by this provider.
    fn domain(&self) -> DistributedDomainId;

    /// Returns domain capabilities.
    fn capabilities(&self) -> DistributedCapabilities;

    /// Returns whether a requested allocation is possible.
    fn can_allocate(&self, bytes: u64) -> bool;

    /// Allocates distributed storage.
    fn allocate(
        &self,
        bytes: u64,
        shard: Option<ShardId>,
    ) -> Result<DistributedProviderAllocation, MemoryError>;

    /// Releases provider-owned distributed storage.
    fn release(
        &self,
        allocation: DistributedProviderAllocation,
    ) -> Result<(), MemoryError>;

    /// Returns provider availability.
    fn availability(&self) -> ProviderAvailability {
        ProviderAvailability::Available
    }
}

// =============================================================================
// Allocator adapter
// =============================================================================

/// Adapter allowing a `DistributedMemoryProvider` to participate in the
/// generic `MemoryAllocator` provider registry.
///
/// This is the integration point with `allocator.rs`.
pub struct DistributedAllocatorProvider<P> {
    provider: Arc<P>,
}

impl<P> DistributedAllocatorProvider<P>
where
    P: DistributedMemoryProvider,
{
    /// Creates an allocator adapter.
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Returns the underlying provider.
    pub fn provider(&self) -> &Arc<P> {
        &self.provider
    }
}

impl<P> fmt::Debug for DistributedAllocatorProvider<P>
where
    P: DistributedMemoryProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DistributedAllocatorProvider")
            .field("provider_id", &self.provider.provider_id())
            .field("domain", &self.provider.domain())
            .finish()
    }
}

impl<P> MemoryProvider for DistributedAllocatorProvider<P>
where
    P: DistributedMemoryProvider,
{
    fn provider_id(&self) -> &str {
        self.provider.provider_id()
    }

    fn location(&self) -> MemoryLocation {
        MemoryLocation::Distributed {
            domain_id: self.provider.domain().get() as u32,
        }
    }

    fn can_allocate(&self, bytes: u64) -> bool {
        self.provider.can_allocate(bytes)
    }

    fn allocate(
        &self,
        bytes: u64,
    ) -> Result<Box<dyn ProviderAllocation>, MemoryError> {
        let allocation = self.provider.allocate(bytes, None)?;

        Ok(Box::new(allocation))
    }

    fn availability(&self) -> ProviderAvailability {
        self.provider.availability()
    }
}

// =============================================================================
// Distributed allocation request
// =============================================================================

/// Complete distributed allocation request.
///
/// This is intentionally separate from `AllocationRequest` because distributed
/// allocation may carry a shard identity and distribution policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedAllocationRequest {
    /// Requested bytes for this shard/allocation.
    pub bytes: ByteCount,

    /// Owning distributed domain.
    pub domain: DistributedDomainId,

    /// Optional shard.
    pub shard: Option<ShardId>,

    /// Semantic allocation class.
    pub class: AllocationClass,

    /// Logical element count represented.
    pub elements: u64,

    /// Bytes per element.
    pub element_bytes: u64,

    /// Optional safe label.
    pub label: Option<String>,
}

impl DistributedAllocationRequest {
    /// Creates a request.
    pub fn new(
        bytes: ByteCount,
        domain: DistributedDomainId,
        class: AllocationClass,
        elements: u64,
        element_bytes: u64,
    ) -> Result<Self, MemoryError> {
        if !domain.is_valid() {
            return Err(MemoryError::InvalidIdentifier {
                kind: "distributed-domain-id".to_owned(),
                identifier: domain.get().to_string(),
                context: None,
            });
        }

        if bytes.is_zero() || elements == 0 || element_bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                reason: "distributed allocation dimensions must be non-zero"
                    .to_owned(),
            });
        }

        let expected = elements
            .checked_mul(element_bytes)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "distributed allocation expected bytes".to_owned(),
            })?;

        if expected != bytes.get() {
            return Err(MemoryError::StateDimensionMismatch {
                expected: expected as usize,
                actual: bytes.get() as usize,
                context: "distributed allocation byte count".to_owned(),
            });
        }

        Ok(Self {
            bytes,
            domain,
            shard: None,
            class,
            elements,
            element_bytes,
            label: None,
        })
    }

    /// Assigns a shard.
    #[must_use]
    pub fn with_shard(mut self, shard: ShardId) -> Self {
        self.shard = Some(shard);
        self
    }

    /// Assigns a safe diagnostic label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Converts to the generic allocator request.
    pub fn as_allocator_request(&self) -> AllocationRequest {
        let mut request = AllocationRequest::new(
            self.bytes,
            MemoryLocation::Distributed {
                domain_id: self.domain.get() as u32,
            },
            self.class,
        )
        .with_state_elements(self.elements)
        .with_label(
            self.label
                .clone()
                .unwrap_or_else(|| "distributed-memory".to_owned()),
        );

        if self.class.is_state() {
            request = request.with_qubits(0);
        }

        request
    }
}

// =============================================================================
// Domain registry
// =============================================================================

/// Thread-safe registry of distributed domains.
///
/// This is deliberately local metadata. It does not perform discovery or
/// network I/O.
#[derive(Clone, Default)]
pub struct DistributedDomainRegistry {
    domains: Arc<Mutex<BTreeMap<DistributedDomainId, DistributedDomain>>>,
}

impl fmt::Debug for DistributedDomainRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self
            .domains
            .lock()
            .map(|domains| domains.len())
            .unwrap_or(0);

        f.debug_struct("DistributedDomainRegistry")
            .field("domain_count", &count)
            .finish()
    }
}

impl DistributedDomainRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a domain.
    pub fn register(
        &self,
        domain: DistributedDomain,
    ) -> Result<(), MemoryError> {
        domain.validate()?;

        let mut domains =
            self.domains.lock().map_err(|_| MemoryError::ConcurrencyConflict {
                reason: "distributed domain registry lock is poisoned"
                    .to_owned(),
            })?;

        if domains.contains_key(&domain.id) {
            return Err(MemoryError::InvalidArgument {
                reason: format!("distributed domain {} already exists", domain.id),
            });
        }

        domains.insert(domain.id, domain);

        Ok(())
    }

    /// Returns a domain.
    pub fn get(
        &self,
        id: DistributedDomainId,
    ) -> Result<DistributedDomain, MemoryError> {
        let domains =
            self.domains.lock().map_err(|_| MemoryError::ConcurrencyConflict {
                reason: "distributed domain registry lock is poisoned"
                    .to_owned(),
            })?;

        domains.get(&id).cloned().ok_or_else(|| {
            MemoryError::InvalidIdentifier {
                kind: "distributed-domain-id".to_owned(),
                identifier: id.get().to_string(),
                context: Some("distributed domain registry".to_owned()),
            }
        })
    }

    /// Removes a domain.
    pub fn remove(
        &self,
        id: DistributedDomainId,
    ) -> Result<DistributedDomain, MemoryError> {
        let mut domains =
            self.domains.lock().map_err(|_| MemoryError::ConcurrencyConflict {
                reason: "distributed domain registry lock is poisoned"
                    .to_owned(),
            })?;

        domains.remove(&id).ok_or_else(|| {
            MemoryError::InvalidIdentifier {
                kind: "distributed-domain-id".to_owned(),
                identifier: id.get().to_string(),
                context: Some("distributed domain registry".to_owned()),
            }
        })
    }

    /// Returns the number of registered domains.
    pub fn len(&self) -> Result<usize, MemoryError> {
        let domains =
            self.domains.lock().map_err(|_| MemoryError::ConcurrencyConflict {
                reason: "distributed domain registry lock is poisoned"
                    .to_owned(),
            })?;

        Ok(domains.len())
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> Result<bool, MemoryError> {
        Ok(self.len()? == 0)
    }
}

// =============================================================================
// Utility validation
// =============================================================================

fn validate_identifier(
    value: &str,
    kind: &str,
) -> Result<(), MemoryError> {
    if value.is_empty() {
        return Err(MemoryError::InvalidArgument {
            reason: format!("{kind} cannot be empty"),
        });
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(MemoryError::InvalidArgument {
            reason: format!(
                "{kind} exceeds maximum length of {} bytes",
                MAX_IDENTIFIER_LENGTH
            ),
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(MemoryError::InvalidArgument {
            reason: format!("{kind} contains control characters"),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_partition_covers_every_element_exactly_once() {
        let plan =
            PartitionPlan::balanced(10, 3).expect("partition must succeed");

        assert_eq!(plan.shards.len(), 3);
        assert_eq!(plan.shards[0].length, 4);
        assert_eq!(plan.shards[1].length, 3);
        assert_eq!(plan.shards[2].length, 3);

        plan.validate().expect("partition must validate");

        for index in 0..10 {
            let (rank, local) = plan
                .locate(GlobalIndex::new(index))
                .expect("index must be owned");

            assert_eq!(
                rank.get(),
                match index {
                    0..=3 => 0,
                    4..=6 => 1,
                    _ => 2,
                }
            );

            assert!(local.get() < 4);
        }
    }

    #[test]
    fn state_vector_partition_is_checked() {
        let plan =
            PartitionPlan::for_qubits(5, 4).expect("partition must succeed");

        assert_eq!(plan.total_elements, 32);
        assert_eq!(plan.world_size, 4);

        plan.validate().expect("partition must validate");
    }

    #[test]
    fn arbitrary_world_sizes_are_supported() {
        let plan =
            PartitionPlan::for_qubits(6, 3).expect("partition must succeed");

        assert_eq!(plan.total_elements, 64);
        assert_eq!(plan.shards.len(), 3);
        plan.validate().expect("partition must validate");
    }

    #[test]
    fn invalid_global_index_is_rejected() {
        let plan =
            PartitionPlan::balanced(10, 2).expect("partition must succeed");

        let result = plan.owner_of(GlobalIndex::new(10));

        assert!(matches!(
            result,
            Err(MemoryError::OutOfBounds { .. })
        ));
    }

    #[test]
    fn shard_local_index_is_correct() {
        let shard = MemoryShard::new(
            ShardId::new(7),
            RankId::new(2),
            GlobalIndex::new(100),
            20,
        )
        .expect("shard must be valid");

        assert_eq!(
            shard
                .local_index(GlobalIndex::new(100))
                .expect("index must belong")
                .get(),
            0
        );

        assert_eq!(
            shard
                .local_index(GlobalIndex::new(119))
                .expect("index must belong")
                .get(),
            19
        );
    }

    #[test]
    fn shard_out_of_bounds_is_rejected() {
        let shard = MemoryShard::new(
            ShardId::new(0),
            RankId::new(0),
            GlobalIndex::new(10),
            5,
        )
        .expect("shard must be valid");

        assert!(shard.local_index(GlobalIndex::new(15)).is_err());
    }

    #[test]
    fn migration_plan_handles_different_rank_counts() {
        let source =
            PartitionPlan::balanced(16, 2).expect("source partition");
        let destination =
            PartitionPlan::balanced(16, 4).expect("destination partition");

        let plan =
            MigrationPlan::build(source, destination, 16)
                .expect("migration plan must succeed");

        plan.validate().expect("migration plan must validate");
        assert!(!plan.transfers.is_empty());
        assert!(plan.total_bytes().expect("bytes").get() > 0);
    }

    #[test]
    fn identical_partition_needs_no_transfer() {
        let source =
            PartitionPlan::balanced(16, 4).expect("source partition");
        let destination = source.clone();

        let plan =
            MigrationPlan::build(source, destination, 16)
                .expect("migration plan must succeed");

        assert!(plan.transfers.is_empty());
        assert_eq!(plan.total_bytes().expect("bytes").get(), 0);
    }

    #[test]
    fn distributed_descriptor_is_valid() {
        let partition =
            PartitionPlan::balanced(1024, 4).expect("partition");

        let descriptor = DistributedMemoryDescriptor::new(
            MemoryId::new(1),
            DistributedDomainId::new(7),
            1024,
            16,
            partition,
        )
        .expect("descriptor");

        assert_eq!(
            descriptor.total_bytes().expect("bytes").get(),
            16_384
        );

        assert_eq!(
            descriptor
                .locate(GlobalIndex::new(513))
                .expect("location")
                .0
                .get(),
            2
        );
    }

    #[test]
    fn replication_policy_is_deterministic() {
        assert_eq!(ReplicationPolicy::None.copies(), 1);
        assert_eq!(ReplicationPolicy::Single.copies(), 2);
        assert_eq!(ReplicationPolicy::Factor(4).copies(), 4);
        assert!(!ReplicationPolicy::Factor(0).validate());
    }

    #[test]
    fn replica_set_rejects_duplicates() {
        let set = ShardReplicaSet {
            primary: RankId::new(0),
            replicas: vec![RankId::new(1), RankId::new(1)],
        };

        assert!(set.validate().is_err());
    }

    #[test]
    fn consistency_epoch_is_checked() {
        let epoch = ConsistencyEpoch::new(41);
        assert_eq!(epoch.checked_next().expect("next").get(), 42);
    }

    #[test]
    fn message_rejects_self_send() {
        let result = DistributedMessage::new(
            OperationId::new(1),
            RankId::new(0),
            RankId::new(0),
            None,
            0,
            vec![1, 2, 3],
        );

        assert!(result.is_err());
    }

    #[test]
    fn transfer_rejects_zero_size() {
        let result = Transfer::new(
            RankId::new(0),
            RankId::new(1),
            ShardId::new(0),
            GlobalIndex::new(0),
            0,
            16,
        );

        assert!(result.is_err());
    }

    #[test]
    fn domain_requires_every_rank() {
        let domain = DistributedDomain::new(
            DistributedDomainId::new(1),
            "test-domain",
            2,
            DistributedCapabilities::serial(),
        )
        .expect("domain");

        assert!(domain.validate().is_err());
    }

    #[test]
    fn domain_registry_is_thread_safe_metadata() {
        let registry = DistributedDomainRegistry::new();

        let domain = DistributedDomain::new(
            DistributedDomainId::new(1),
            "test-domain",
            1,
            DistributedCapabilities::serial(),
        )
        .expect("domain")
        .with_rank(RankDescriptor {
            rank: RankId::new(0),
            node: NodeId::new(0),
            resources: Vec::new(),
            healthy: true,
        })
        .expect("rank");

        registry.register(domain).expect("registration");

        assert_eq!(registry.len().expect("length"), 1);
        assert!(registry.get(DistributedDomainId::new(1)).is_ok());
    }

    #[test]
    fn allocation_request_matches_byte_dimensions() {
        let request = DistributedAllocationRequest::new(
            ByteCount::new(1024),
            DistributedDomainId::new(1),
            AllocationClass::State,
            64,
            16,
        )
        .expect("request");

        assert_eq!(request.bytes.get(), 1024);
        assert_eq!(request.elements, 64);
        assert_eq!(request.element_bytes, 16);
    }

    #[test]
    fn overflow_in_state_partition_is_rejected() {
        let result = PartitionPlan::for_qubits(64, 2);

        assert!(result.is_err());
    }
}