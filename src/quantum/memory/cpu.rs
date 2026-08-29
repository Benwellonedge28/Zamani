//! Zamani Quantum Memory — CPU Memory Subsystem
//!
//! Production CPU-side memory implementation for `quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns CPU/host-side memory concerns that are above the generic
//! provider-neutral allocator boundary:
//!
//! - safe host-memory buffers;
//! - fallible CPU allocation;
//! - CPU memory capacity/accounting metadata;
//! - CPU memory domains;
//! - CPU topology metadata;
//! - NUMA-domain abstraction;
//! - cache/topology hints;
//! - CPU memory transfer/staging buffers;
//! - typed CPU buffers for quantum-state implementations;
//! - deterministic memory estimation;
//! - CPU capability discovery;
//! - host-memory validation;
//! - CPU-friendly copy/fill operations;
//! - bounded allocation helpers;
//! - integration contracts for state-vector, density-matrix, sparse,
//!   stabilizer and tensor-network implementations.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                         │
//!                         ▼
//!                 runtime / simulator
//!                         │
//!                         ▼
//!                  quantum::memory
//!                         │
//!                  MemoryAllocator
//!                         │
//!            ┌────────────┼─────────────┐
//!            │            │             │
//!            ▼            ▼             ▼
//!          cpu.rs       gpu.rs      distributed.rs
//!            │
//!            ▼
//!       CPU host storage
//! ```
//!
//! `allocator.rs` remains the canonical provider-neutral allocation boundary.
//! This module must not create a competing allocator abstraction.
//!
//! # Provider-neutral hardware rule
//!
//! CPU memory is a universal host-side substrate, not a QPU implementation.
//!
//! Quantum hardware may be:
//!
//! - superconducting;
//! - trapped-ion;
//! - neutral-atom;
//! - photonic;
//! - spin-based;
//! - semiconductor;
//! - topological;
//! - annealing/optimization hardware;
//! - measurement-based;
//! - hybrid;
//! - future hardware not yet represented by Zamani.
//!
//! Those systems may use CPU memory for:
//!
//! - circuit construction;
//! - classical control;
//! - measurement results;
//! - calibration metadata;
//! - scheduling;
//! - backend requests;
//! - result processing;
//! - host/device staging;
//! - checkpointing;
//! - simulation.
//!
//! They must not require this module to know their vendor API.
//!
//! GPU/device and QPU-native memory belongs behind the generic interfaces in
//! `allocator.rs`, `gpu.rs`, `distributed.rs`, and `backend_state.rs`.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.
//!
//! The module uses only safe Rust containers and operations.
//!
//! In particular:
//!
//! - no raw pointers;
//! - no `std::alloc` calls;
//! - no FFI;
//! - no unchecked indexing in public APIs;
//! - no unchecked capacity multiplication;
//! - no `Vec::set_len`;
//! - no `MaybeUninit`;
//! - no architecture-specific intrinsics.
//!
//! Hardware-specific SIMD/FFI implementations belong in `simd.rs` or provider
//! crates and must preserve the safe public memory contract.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! `cpu.rs` is intentionally independent of higher-level state representations.
//!
//! Later modules consume it as follows:
//!
//! - `allocator.rs` remains the canonical provider-neutral allocation layer;
//! - `state_vector.rs` may use `CpuBuffer<Complex<f32>>` or
//!   `CpuBuffer<Complex<f64>>` through the appropriate scalar abstraction;
//! - `density_matrix.rs` may use `CpuBuffer<T>` for matrix storage;
//! - `stabilizer.rs` may use typed integer/bit buffers;
//! - `sparse.rs` may use CPU buffers for indices and amplitudes;
//! - `tensor.rs` may use CPU buffers for tensor elements;
//! - `tensor_network.rs` may use CPU buffers for tensors and temporary
//!   contraction storage;
//! - `gpu.rs` may use CPU staging buffers for host/device transfers;
//! - `distributed.rs` may use CPU buffers as serialization/network staging
//!   storage;
//! - `migration.rs` may use CPU buffers as a source/destination staging area;
//! - `snapshot.rs` and `checkpoint.rs` may use CPU buffers before persistence;
//! - `diagnostics.rs` may consume CPU capacity and allocation statistics;
//! - `telemetry.rs` may consume the exported CPU metrics;
//! - `cache.rs` may use CPU buffers for host-side cache entries.
//!
//! No higher-level quantum subsystem should need to know how CPU memory was
//! allocated.
//!
//! # Important distinction
//!
//! A CPU buffer is not a quantum state.
//!
//! ```text
//! CpuBuffer<T>
//!      │
//!      ├── storage
//!      │
//!      └── capacity
//!
//! StateVector / DensityMatrix / Stabilizer / TensorNetwork
//!      │
//!      └── quantum semantics
//! ```
//!
//! This separation prevents `cpu.rs` from becoming a simulator implementation.
//!
//! # Allocation rule
//!
//! All potentially large allocations use fallible allocation:
//!
//! ```text
//! validate element count
//!        │
//!        ▼
//! checked byte calculation
//!        │
//!        ▼
//! caller checks MemoryLimits
//!        │
//!        ▼
//! CpuBuffer::try_with_len()
//!        │
//!        ▼
//! try_reserve_exact()
//! ```
//!
//! `cpu.rs` deliberately does not bypass the centralized `MemoryLimits` policy.
//! The CPU buffer provides local safety; the allocator/limits subsystem remains
//! authoritative for global resource policy.
//!
//! # No hidden hardware assumptions
//!
//! This module does not assume:
//!
//! - x86;
//! - ARM;
//! - RISC-V;
//! - Apple Silicon;
//! - Linux;
//! - Windows;
//! - macOS;
//! - a particular NUMA API;
//! - a particular cache hierarchy.
//!
//! Runtime topology discovery may be added by platform-specific components.
//! The public types here remain portable.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::mem::size_of;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the CPU-memory contract.
pub const CPU_MEMORY_SCHEMA_ID: &str = "zamani.quantum.memory.cpu";

/// Semantic version of the CPU-memory contract.
pub const CPU_MEMORY_SCHEMA_VERSION: u16 = 1;

/// Maximum provider/domain name retained by this module.
pub const MAX_CPU_DOMAIN_NAME_LENGTH: usize = 256;

/// Maximum number of CPU domains represented by one topology description.
pub const MAX_CPU_DOMAINS: usize = 65_536;

/// Maximum number of CPU threads represented by one topology description.
pub const MAX_CPU_THREADS: usize = 1_000_000;

// =============================================================================
// CPU memory errors
// =============================================================================

/// Errors specific to safe CPU-memory operations.
///
/// Global quantum-memory errors remain owned by `memory::errors`.
/// This local error is deliberately dependency-light so that `cpu.rs` does not
/// create a dependency cycle or force the foundational error module to depend
/// on CPU implementation details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpuMemoryError {
    /// Requested element count is invalid.
    InvalidElementCount {
        /// Requested number of elements.
        elements: u64,
    },

    /// Requested byte count is invalid.
    InvalidByteCount {
        /// Requested number of bytes.
        bytes: u64,
    },

    /// Element-to-byte conversion overflowed.
    SizeOverflow {
        /// Number of elements.
        elements: u64,

        /// Size of each element.
        element_size: usize,
    },

    /// The requested allocation cannot be represented by this process.
    AddressSpaceExceeded {
        /// Requested bytes.
        bytes: u64,
    },

    /// The standard library rejected the allocation request.
    AllocationFailed {
        /// Requested bytes.
        bytes: u64,
    },

    /// The requested range is outside the buffer.
    OutOfBounds {
        /// Requested index.
        index: usize,

        /// Buffer length.
        len: usize,
    },

    /// A range is invalid.
    InvalidRange {
        /// Start of range.
        start: usize,

        /// End of range.
        end: usize,

        /// Buffer length.
        len: usize,
    },

    /// A source and destination region have incompatible sizes.
    LengthMismatch {
        /// Source length.
        source: usize,

        /// Destination length.
        destination: usize,
    },

    /// CPU-domain metadata is invalid.
    InvalidDomain {
        /// Invalid domain identifier.
        domain_id: u32,
    },

    /// CPU topology metadata is invalid.
    InvalidTopology {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A supplied name is too long.
    NameTooLong,

    /// A requested operation is unsupported by this portable CPU layer.
    Unsupported {
        /// Stable operation identifier.
        operation: &'static str,
    },
}

impl fmt::Display for CpuMemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidElementCount { elements } => {
                write!(formatter, "invalid CPU buffer element count: {elements}")
            }
            Self::InvalidByteCount { bytes } => {
                write!(formatter, "invalid CPU buffer byte count: {bytes}")
            }
            Self::SizeOverflow {
                elements,
                element_size,
            } => {
                write!(
                    formatter,
                    "CPU buffer size overflow: {elements} elements × \
                     {element_size} bytes"
                )
            }
            Self::AddressSpaceExceeded { bytes } => {
                write!(
                    formatter,
                    "CPU buffer of {bytes} bytes cannot be represented by \
                     this process"
                )
            }
            Self::AllocationFailed { bytes } => {
                write!(
                    formatter,
                    "CPU memory allocation failed for {bytes} bytes"
                )
            }
            Self::OutOfBounds { index, len } => {
                write!(
                    formatter,
                    "CPU buffer index {index} is out of bounds for length {len}"
                )
            }
            Self::InvalidRange { start, end, len } => {
                write!(
                    formatter,
                    "invalid CPU buffer range {start}..{end} for length {len}"
                )
            }
            Self::LengthMismatch {
                source,
                destination,
            } => {
                write!(
                    formatter,
                    "CPU buffer length mismatch: source={source}, \
                     destination={destination}"
                )
            }
            Self::InvalidDomain { domain_id } => {
                write!(formatter, "invalid CPU memory domain: {domain_id}")
            }
            Self::InvalidTopology { reason } => {
                write!(formatter, "invalid CPU topology: {reason}")
            }
            Self::NameTooLong => {
                formatter.write_str("CPU domain name exceeds the maximum length")
            }
            Self::Unsupported { operation } => {
                write!(formatter, "unsupported CPU-memory operation: {operation}")
            }
        }
    }
}

impl std::error::Error for CpuMemoryError {}

// =============================================================================
// CPU architecture-independent memory kinds
// =============================================================================

/// Broad CPU memory classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CpuMemoryKind {
    /// Ordinary pageable host memory.
    Pageable,

    /// Memory intended to be used as a staging area for device transfers.
    ///
    /// This does not claim that the operating system has actually pinned the
    /// pages. True pinning is provider/platform specific.
    Staging,

    /// CPU memory associated with a logical NUMA domain.
    NumaLocal,

    /// CPU memory whose NUMA placement is intentionally unspecified.
    NumaInterleaved,
}

impl CpuMemoryKind {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pageable => "pageable",
            Self::Staging => "staging",
            Self::NumaLocal => "numa_local",
            Self::NumaInterleaved => "numa_interleaved",
        }
    }
}

impl fmt::Display for CpuMemoryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// CPU memory domain
// =============================================================================

/// Stable logical identifier for a CPU/NUMA memory domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CpuDomainId(u32);

impl CpuDomainId {
    /// Invalid/unassigned domain identifier.
    pub const INVALID: Self = Self(0);

    /// Creates a domain identifier.
    ///
    /// Zero is reserved for `INVALID`.
    pub const fn new(value: u32) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the raw stable identifier.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for CpuDomainId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Logical CPU memory domain metadata.
///
/// This intentionally does not call an OS NUMA API. Platform-specific
/// discovery can populate this structure later.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuMemoryDomain {
    id: CpuDomainId,
    name: String,
    memory_bytes: Option<u64>,
    thread_count: Option<u32>,
}

impl CpuMemoryDomain {
    /// Creates a CPU memory domain.
    pub fn new(
        id: CpuDomainId,
        name: impl Into<String>,
    ) -> Result<Self, CpuMemoryError> {
        if id == CpuDomainId::INVALID {
            return Err(CpuMemoryError::InvalidDomain {
                domain_id: id.get(),
            });
        }

        let name = name.into();

        if name.len() > MAX_CPU_DOMAIN_NAME_LENGTH {
            return Err(CpuMemoryError::NameTooLong);
        }

        Ok(Self {
            id,
            name,
            memory_bytes: None,
            thread_count: None,
        })
    }

    /// Sets the discovered memory capacity of this domain.
    pub fn with_memory_bytes(mut self, bytes: u64) -> Result<Self, CpuMemoryError> {
        if bytes == 0 {
            return Err(CpuMemoryError::InvalidByteCount { bytes });
        }

        self.memory_bytes = Some(bytes);
        Ok(self)
    }

    /// Sets the discovered CPU-thread count of this domain.
    pub fn with_thread_count(
        mut self,
        threads: u32,
    ) -> Result<Self, CpuMemoryError> {
        if threads == 0 {
            return Err(CpuMemoryError::InvalidTopology {
                reason: "thread count must be greater than zero",
            });
        }

        self.thread_count = Some(threads);
        Ok(self)
    }

    /// Returns the domain identifier.
    pub const fn id(&self) -> CpuDomainId {
        self.id
    }

    /// Returns the domain name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns discovered memory capacity.
    pub const fn memory_bytes(&self) -> Option<u64> {
        self.memory_bytes
    }

    /// Returns discovered thread count.
    pub const fn thread_count(&self) -> Option<u32> {
        self.thread_count
    }
}

// =============================================================================
// CPU topology
// =============================================================================

/// Portable CPU topology description.
///
/// This is metadata only. It does not attempt to bind threads or alter OS
/// scheduling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuTopology {
    logical_cpus: u32,
    physical_cores: Option<u32>,
    numa_domains: Vec<CpuMemoryDomain>,
}

impl CpuTopology {
    /// Creates a topology with the specified logical CPU count.
    pub fn new(logical_cpus: u32) -> Result<Self, CpuMemoryError> {
        if logical_cpus == 0 {
            return Err(CpuMemoryError::InvalidTopology {
                reason: "logical CPU count must be greater than zero",
            });
        }

        Ok(Self {
            logical_cpus,
            physical_cores: None,
            numa_domains: Vec::new(),
        })
    }

    /// Sets the physical-core count.
    pub fn with_physical_cores(
        mut self,
        physical_cores: u32,
    ) -> Result<Self, CpuMemoryError> {
        if physical_cores == 0 || physical_cores > self.logical_cpus {
            return Err(CpuMemoryError::InvalidTopology {
                reason: "physical-core count must be between 1 and logical CPU count",
            });
        }

        self.physical_cores = Some(physical_cores);
        Ok(self)
    }

    /// Adds a NUMA/memory domain.
    pub fn with_domain(
        mut self,
        domain: CpuMemoryDomain,
    ) -> Result<Self, CpuMemoryError> {
        if self.numa_domains.len() >= MAX_CPU_DOMAINS {
            return Err(CpuMemoryError::InvalidTopology {
                reason: "maximum CPU domain count exceeded",
            });
        }

        if self
            .numa_domains
            .iter()
            .any(|existing| existing.id() == domain.id())
        {
            return Err(CpuMemoryError::InvalidTopology {
                reason: "duplicate CPU memory-domain identifier",
            });
        }

        self.numa_domains.push(domain);
        Ok(self)
    }

    /// Returns logical CPU count.
    pub const fn logical_cpus(&self) -> u32 {
        self.logical_cpus
    }

    /// Returns physical-core count if discovered.
    pub const fn physical_cores(&self) -> Option<u32> {
        self.physical_cores
    }

    /// Returns the number of logical memory domains.
    pub fn domain_count(&self) -> usize {
        self.numa_domains.len()
    }

    /// Returns all known memory domains.
    pub fn domains(&self) -> &[CpuMemoryDomain] {
        &self.numa_domains
    }
}

// =============================================================================
// CPU capabilities
// =============================================================================

/// Portable CPU-memory capability description.
///
/// SIMD capabilities are represented as optional descriptive strings rather
/// than compile-time architecture assumptions. `simd.rs` owns actual kernel
/// dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuCapabilities {
    logical_cpus: u32,
    physical_cores: Option<u32>,
    numa_domains: usize,
    supports_parallel_copy: bool,
    supports_parallel_fill: bool,
    simd_features: Vec<String>,
}

impl CpuCapabilities {
    /// Builds capabilities from topology metadata.
    pub fn from_topology(topology: &CpuTopology) -> Self {
        Self {
            logical_cpus: topology.logical_cpus(),
            physical_cores: topology.physical_cores(),
            numa_domains: topology.domain_count(),
            supports_parallel_copy: topology.logical_cpus() > 1,
            supports_parallel_fill: topology.logical_cpus() > 1,
            simd_features: Vec::new(),
        }
    }

    /// Adds a descriptive SIMD feature.
    ///
    /// Actual SIMD dispatch belongs to `simd.rs`.
    pub fn with_simd_feature(mut self, feature: impl Into<String>) -> Self {
        let feature = feature.into();

        if !feature.is_empty() && feature.len() <= MAX_CPU_DOMAIN_NAME_LENGTH {
            self.simd_features.push(feature);
        }

        self
    }

    /// Returns logical CPU count.
    pub const fn logical_cpus(&self) -> u32 {
        self.logical_cpus
    }

    /// Returns physical-core count.
    pub const fn physical_cores(&self) -> Option<u32> {
        self.physical_cores
    }

    /// Returns NUMA-domain count.
    pub const fn numa_domains(&self) -> usize {
        self.numa_domains
    }

    /// Returns whether parallel copy is available.
    pub const fn supports_parallel_copy(&self) -> bool {
        self.supports_parallel_copy
    }

    /// Returns whether parallel fill is available.
    pub const fn supports_parallel_fill(&self) -> bool {
        self.supports_parallel_fill
    }

    /// Returns advertised SIMD features.
    pub fn simd_features(&self) -> &[String] {
        &self.simd_features
    }
}

// =============================================================================
// CPU allocation requirements
// =============================================================================

/// Checked CPU-memory allocation requirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CpuMemoryRequirement {
    elements: u64,
    element_size: usize,
    bytes: u64,
}

impl CpuMemoryRequirement {
    /// Creates a checked requirement.
    pub fn new(
        elements: u64,
        element_size: usize,
    ) -> Result<Self, CpuMemoryError> {
        if elements == 0 {
            return Err(CpuMemoryError::InvalidElementCount { elements });
        }

        if element_size == 0 {
            return Err(CpuMemoryError::InvalidByteCount { bytes: 0 });
        }

        let element_size_u64 =
            u64::try_from(element_size).map_err(|_| CpuMemoryError::SizeOverflow {
                elements,
                element_size,
            })?;

        let bytes = elements
            .checked_mul(element_size_u64)
            .ok_or(CpuMemoryError::SizeOverflow {
                elements,
                element_size,
            })?;

        let max_usize = usize::MAX as u64;

        if bytes > max_usize {
            return Err(CpuMemoryError::AddressSpaceExceeded { bytes });
        }

        Ok(Self {
            elements,
            element_size,
            bytes,
        })
    }

    /// Returns the number of elements.
    pub const fn elements(self) -> u64 {
        self.elements
    }

    /// Returns the size of one element.
    pub const fn element_size(self) -> usize {
        self.element_size
    }

    /// Returns the required bytes.
    pub const fn bytes(self) -> u64 {
        self.bytes
    }
}

impl fmt::Display for CpuMemoryRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} elements × {} bytes = {} bytes",
            self.elements, self.element_size, self.bytes
        )
    }
}

// =============================================================================
// Typed CPU buffer
// =============================================================================

/// Safe typed CPU-memory buffer.
///
/// This is the fundamental host-side storage object consumed by quantum-state
/// representations.
///
/// The buffer owns its data and cannot expose raw pointers.
#[derive(Clone, Debug, PartialEq)]
pub struct CpuBuffer<T> {
    data: Vec<T>,
    kind: CpuMemoryKind,
    domain: Option<CpuDomainId>,
}

impl<T> CpuBuffer<T> {
    /// Creates an empty pageable CPU buffer.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            kind: CpuMemoryKind::Pageable,
            domain: None,
        }
    }

    /// Creates an empty CPU buffer with an explicit memory kind.
    pub fn with_kind(kind: CpuMemoryKind) -> Self {
        Self {
            data: Vec::new(),
            kind,
            domain: None,
        }
    }

    /// Creates an empty CPU buffer associated with a logical CPU domain.
    pub fn with_domain(
        kind: CpuMemoryKind,
        domain: CpuDomainId,
    ) -> Result<Self, CpuMemoryError> {
        if domain == CpuDomainId::INVALID {
            return Err(CpuMemoryError::InvalidDomain {
                domain_id: domain.get(),
            });
        }

        Ok(Self {
            data: Vec::new(),
            kind,
            domain: Some(domain),
        })
    }

    /// Allocates `len` zero/Default-initialized elements without using
    /// infallible capacity growth.
    ///
    /// `T: Default` is required because safe Rust must initialize the vector.
    pub fn try_with_len(
        len: usize,
        kind: CpuMemoryKind,
    ) -> Result<Self, CpuMemoryError>
    where
        T: Default + Clone,
    {
        if len == 0 {
            return Ok(Self::with_kind(kind));
        }

        let mut data = Vec::new();

        data.try_reserve_exact(len)
            .map_err(|_| CpuMemoryError::AllocationFailed {
                bytes: Self::checked_bytes_for_len(len)?,
            })?;

        data.resize(len, T::default());

        Ok(Self {
            data,
            kind,
            domain: None,
        })
    }

    /// Allocates a buffer containing clones of the supplied value.
    pub fn try_filled(
        len: usize,
        value: T,
        kind: CpuMemoryKind,
    ) -> Result<Self, CpuMemoryError>
    where
        T: Clone,
    {
        if len == 0 {
            return Ok(Self::with_kind(kind));
        }

        let mut data = Vec::new();

        data.try_reserve_exact(len)
            .map_err(|_| CpuMemoryError::AllocationFailed {
                bytes: Self::checked_bytes_for_len(len)?,
            })?;

        data.resize(len, value);

        Ok(Self {
            data,
            kind,
            domain: None,
        })
    }

    /// Creates a CPU buffer by taking ownership of an existing vector.
    pub fn from_vec(data: Vec<T>, kind: CpuMemoryKind) -> Self {
        Self {
            data,
            kind,
            domain: None,
        }
    }

    /// Creates a domain-associated buffer from an existing vector.
    pub fn from_vec_in_domain(
        data: Vec<T>,
        kind: CpuMemoryKind,
        domain: CpuDomainId,
    ) -> Result<Self, CpuMemoryError> {
        if domain == CpuDomainId::INVALID {
            return Err(CpuMemoryError::InvalidDomain {
                domain_id: domain.get(),
            });
        }

        Ok(Self {
            data,
            kind,
            domain: Some(domain),
        })
    }

    /// Calculates bytes for a number of elements.
    pub fn checked_bytes_for_len(len: usize) -> Result<u64, CpuMemoryError> {
        let elements = u64::try_from(len).map_err(|_| {
            CpuMemoryError::SizeOverflow {
                elements: u64::MAX,
                element_size: size_of::<T>(),
            }
        })?;

        CpuMemoryRequirement::new(elements, size_of::<T>())
            .map(|requirement| requirement.bytes())
    }

    /// Reserves additional elements using fallible allocation.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CpuMemoryError> {
        if additional == 0 {
            return Ok(());
        }

        let bytes = Self::checked_bytes_for_len(
            self.data
                .len()
                .checked_add(additional)
                .ok_or(CpuMemoryError::SizeOverflow {
                    elements: u64::MAX,
                    element_size: size_of::<T>(),
                })?,
        )?;

        self.data
            .try_reserve(additional)
            .map_err(|_| CpuMemoryError::AllocationFailed { bytes })
    }

    /// Returns the number of initialized elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns allocated capacity in elements.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns logical bytes occupied by initialized elements.
    pub fn byte_len(&self) -> u64 {
        self.data
            .len()
            .checked_mul(size_of::<T>())
            .and_then(|value| u64::try_from(value).ok())
            .unwrap_or(u64::MAX)
    }

    /// Returns capacity in bytes.
    pub fn capacity_bytes(&self) -> u64 {
        self.data
            .capacity()
            .checked_mul(size_of::<T>())
            .and_then(|value| u64::try_from(value).ok())
            .unwrap_or(u64::MAX)
    }

    /// Returns the CPU memory kind.
    pub const fn kind(&self) -> CpuMemoryKind {
        self.kind
    }

    /// Returns the logical CPU domain, if one was assigned.
    pub const fn domain(&self) -> Option<CpuDomainId> {
        self.domain
    }

    /// Returns an immutable slice.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Returns an element using checked indexing.
    pub fn get(&self, index: usize) -> Result<&T, CpuMemoryError> {
        self.data
            .get(index)
            .ok_or(CpuMemoryError::OutOfBounds {
                index,
                len: self.data.len(),
            })
    }

    /// Returns a mutable element using checked indexing.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, CpuMemoryError> {
        let len = self.data.len();

        self.data
            .get_mut(index)
            .ok_or(CpuMemoryError::OutOfBounds { index, len })
    }

    /// Returns a checked immutable range.
    pub fn range(
        &self,
        start: usize,
        end: usize,
    ) -> Result<&[T], CpuMemoryError> {
        if start > end || end > self.data.len() {
            return Err(CpuMemoryError::InvalidRange {
                start,
                end,
                len: self.data.len(),
            });
        }

        Ok(&self.data[start..end])
    }

    /// Returns a checked mutable range.
    pub fn range_mut(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<&mut [T], CpuMemoryError> {
        if start > end || end > self.data.len() {
            return Err(CpuMemoryError::InvalidRange {
                start,
                end,
                len: self.data.len(),
            });
        }

        Ok(&mut self.data[start..end])
    }

    /// Replaces all initialized elements with `value`.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
    }

    /// Copies from another CPU buffer.
    pub fn copy_from(
        &mut self,
        source: &CpuBuffer<T>,
    ) -> Result<(), CpuMemoryError>
    where
        T: Clone,
    {
        if self.len() != source.len() {
            return Err(CpuMemoryError::LengthMismatch {
                source: source.len(),
                destination: self.len(),
            });
        }

        self.data.clone_from_slice(&source.data);
        Ok(())
    }

    /// Copies from a slice.
    pub fn copy_from_slice(
        &mut self,
        source: &[T],
    ) -> Result<(), CpuMemoryError>
    where
        T: Clone,
    {
        if self.len() != source.len() {
            return Err(CpuMemoryError::LengthMismatch {
                source: source.len(),
                destination: self.len(),
            });
        }

        self.data.clone_from_slice(source);
        Ok(())
    }

    /// Returns an owned vector.
    ///
    /// This is an explicit ownership transfer. It does not expose pointers.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Shrinks capacity to the initialized length.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Clears the initialized elements while retaining capacity.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Default for CpuBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for CpuBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for CpuBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// =============================================================================
// Byte buffer
// =============================================================================

/// Specialized byte buffer for staging, serialization, snapshots and
/// host/device transfers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuByteBuffer {
    buffer: CpuBuffer<u8>,
}

impl CpuByteBuffer {
    /// Creates an empty pageable byte buffer.
    pub fn new() -> Self {
        Self {
            buffer: CpuBuffer::new(),
        }
    }

    /// Allocates an exact byte length using fallible allocation.
    pub fn try_with_len(
        len: usize,
        kind: CpuMemoryKind,
    ) -> Result<Self, CpuMemoryError> {
        CpuBuffer::<u8>::try_with_len(len, kind).map(|buffer| Self { buffer })
    }

    /// Creates a byte buffer from existing bytes.
    pub fn from_vec(data: Vec<u8>, kind: CpuMemoryKind) -> Self {
        Self {
            buffer: CpuBuffer::from_vec(data, kind),
        }
    }

    /// Returns the number of bytes.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns capacity.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Returns byte length.
    pub fn byte_len(&self) -> u64 {
        self.buffer.byte_len()
    }

    /// Returns memory kind.
    pub const fn kind(&self) -> CpuMemoryKind {
        self.buffer.kind()
    }

    /// Returns immutable bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Returns mutable bytes.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    /// Returns a checked byte.
    pub fn get(&self, index: usize) -> Result<&u8, CpuMemoryError> {
        self.buffer.get(index)
    }

    /// Returns a checked mutable byte.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut u8, CpuMemoryError> {
        self.buffer.get_mut(index)
    }

    /// Reserves additional bytes.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), CpuMemoryError> {
        self.buffer.try_reserve(additional)
    }

    /// Clears the initialized bytes.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Shrinks capacity to length.
    pub fn shrink_to_fit(&mut self) {
        self.buffer.shrink_to_fit();
    }

    /// Consumes the buffer and returns its bytes.
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer.into_vec()
    }
}

impl Default for CpuByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// CPU memory statistics
// =============================================================================

/// CPU-memory statistics.
///
/// This structure contains local object-level statistics. Global allocation
/// accounting remains the responsibility of `allocator.rs`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CpuMemoryStatistics {
    allocations: u64,
    releases: u64,
    current_bytes: u64,
    peak_bytes: u64,
    failed_allocations: u64,
}

impl CpuMemoryStatistics {
    /// Creates empty statistics.
    pub const fn new() -> Self {
        Self {
            allocations: 0,
            releases: 0,
            current_bytes: 0,
            peak_bytes: 0,
            failed_allocations: 0,
        }
    }

    /// Records a successful allocation.
    pub fn record_allocation(&mut self, bytes: u64) {
        self.allocations = self.allocations.saturating_add(1);
        self.current_bytes = self.current_bytes.saturating_add(bytes);

        if self.current_bytes > self.peak_bytes {
            self.peak_bytes = self.current_bytes;
        }
    }

    /// Records a release.
    pub fn record_release(&mut self, bytes: u64) {
        self.releases = self.releases.saturating_add(1);
        self.current_bytes = self.current_bytes.saturating_sub(bytes);
    }

    /// Records a failed allocation.
    pub fn record_failure(&mut self) {
        self.failed_allocations = self.failed_allocations.saturating_add(1);
    }

    /// Returns successful allocation count.
    pub const fn allocations(&self) -> u64 {
        self.allocations
    }

    /// Returns release count.
    pub const fn releases(&self) -> u64 {
        self.releases
    }

    /// Returns current bytes.
    pub const fn current_bytes(&self) -> u64 {
        self.current_bytes
    }

    /// Returns peak bytes.
    pub const fn peak_bytes(&self) -> u64 {
        self.peak_bytes
    }

    /// Returns failed allocation count.
    pub const fn failed_allocations(&self) -> u64 {
        self.failed_allocations
    }
}

// =============================================================================
// CPU memory manager
// =============================================================================

/// Safe CPU memory manager.
///
/// This object does not replace `MemoryAllocator`. It provides CPU-specific
/// convenience and capability management above the generic allocator.
#[derive(Clone, Debug)]
pub struct CpuMemoryManager {
    topology: Arc<CpuTopology>,
    capabilities: CpuCapabilities,
    statistics: CpuMemoryStatistics,
}

impl CpuMemoryManager {
    /// Creates a CPU memory manager from explicitly supplied topology.
    pub fn new(topology: CpuTopology) -> Self {
        let capabilities = CpuCapabilities::from_topology(&topology);

        Self {
            topology: Arc::new(topology),
            capabilities,
            statistics: CpuMemoryStatistics::new(),
        }
    }

    /// Creates a conservative topology for the current process.
    ///
    /// This intentionally avoids external CPU-topology dependencies.
    pub fn conservative() -> Self {
        let logical_cpus = std::thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1);

        let logical_cpus = logical_cpus.min(u32::MAX as usize) as u32;

        Self::new(
            CpuTopology::new(logical_cpus)
                .expect("available_parallelism must produce at least one CPU"),
        )
    }

    /// Returns topology metadata.
    pub fn topology(&self) -> &CpuTopology {
        &self.topology
    }

    /// Returns CPU capabilities.
    pub fn capabilities(&self) -> &CpuCapabilities {
        &self.capabilities
    }

    /// Returns local statistics.
    pub const fn statistics(&self) -> CpuMemoryStatistics {
        self.statistics
    }

    /// Calculates a checked state-memory requirement.
    ///
    /// Quantum representations should call this before requesting storage and
    /// then pass the resulting byte count through `MemoryLimits`/allocator
    /// policy.
    pub fn estimate<T>(
        &self,
        elements: u64,
    ) -> Result<CpuMemoryRequirement, CpuMemoryError> {
        CpuMemoryRequirement::new(elements, size_of::<T>())
    }

    /// Allocates a typed CPU buffer after local arithmetic validation.
    ///
    /// Global resource limits are intentionally checked by the caller through
    /// `MemoryLimits` and `MemoryAllocator`.
    pub fn try_allocate<T>(
        &mut self,
        elements: usize,
        kind: CpuMemoryKind,
    ) -> Result<CpuBuffer<T>, CpuMemoryError>
    where
        T: Default + Clone,
    {
        match CpuBuffer::<T>::try_with_len(elements, kind) {
            Ok(buffer) => {
                self.statistics.record_allocation(buffer.byte_len());
                Ok(buffer)
            }
            Err(error) => {
                self.statistics.record_failure();
                Err(error)
            }
        }
    }

    /// Allocates a CPU byte staging buffer.
    pub fn try_allocate_bytes(
        &mut self,
        bytes: usize,
        kind: CpuMemoryKind,
    ) -> Result<CpuByteBuffer, CpuMemoryError> {
        match CpuByteBuffer::try_with_len(bytes, kind) {
            Ok(buffer) => {
                self.statistics.record_allocation(buffer.byte_len());
                Ok(buffer)
            }
            Err(error) => {
                self.statistics.record_failure();
                Err(error)
            }
        }
    }
}

// =============================================================================
// Memory estimation helpers
// =============================================================================

/// Calculates a checked CPU-memory requirement.
///
/// This helper is useful to state representations before they invoke the
/// global `MemoryLimits` policy.
pub fn estimate_bytes<T>(
    elements: u64,
) -> Result<CpuMemoryRequirement, CpuMemoryError> {
    CpuMemoryRequirement::new(elements, size_of::<T>())
}

/// Calculates bytes for an arbitrary element size.
pub fn estimate_bytes_for_element_size(
    elements: u64,
    element_size: usize,
) -> Result<CpuMemoryRequirement, CpuMemoryError> {
    CpuMemoryRequirement::new(elements, element_size)
}

/// Calculates dense state-vector element count.
///
/// For `n` qubits:
///
/// `elements = 2^n`.
pub fn state_vector_elements(
    qubits: u32,
) -> Result<u64, CpuMemoryError> {
    if qubits >= 64 {
        return Err(CpuMemoryError::SizeOverflow {
            elements: u64::MAX,
            element_size: 1,
        });
    }

    1u64
        .checked_shl(qubits)
        .ok_or(CpuMemoryError::SizeOverflow {
            elements: u64::MAX,
            element_size: 1,
        })
}

/// Calculates dense state-vector storage.
///
/// This performs only mathematical estimation. It does not allocate.
pub fn estimate_state_vector_bytes<T>(
    qubits: u32,
) -> Result<CpuMemoryRequirement, CpuMemoryError> {
    let elements = state_vector_elements(qubits)?;
    estimate_bytes::<T>(elements)
}

/// Calculates dense density-matrix element count.
///
/// A density matrix has `4^n = 2^(2n)` complex elements.
pub fn density_matrix_elements(
    qubits: u32,
) -> Result<u64, CpuMemoryError> {
    let doubled = qubits
        .checked_mul(2)
        .ok_or(CpuMemoryError::SizeOverflow {
            elements: u64::MAX,
            element_size: 1,
        })?;

    if doubled >= 64 {
        return Err(CpuMemoryError::SizeOverflow {
            elements: u64::MAX,
            element_size: 1,
        });
    }

    1u64
        .checked_shl(doubled)
        .ok_or(CpuMemoryError::SizeOverflow {
            elements: u64::MAX,
            element_size: 1,
        })
}

/// Calculates dense density-matrix storage.
///
/// This performs only mathematical estimation. It does not allocate.
pub fn estimate_density_matrix_bytes<T>(
    qubits: u32,
) -> Result<CpuMemoryRequirement, CpuMemoryError> {
    let elements = density_matrix_elements(qubits)?;
    estimate_bytes::<T>(elements)
}

// =============================================================================
// CPU copy helpers
// =============================================================================

/// Copies a slice into a destination slice after checking lengths.
pub fn copy_exact<T>(
    destination: &mut [T],
    source: &[T],
) -> Result<(), CpuMemoryError>
where
    T: Clone,
{
    if destination.len() != source.len() {
        return Err(CpuMemoryError::LengthMismatch {
            source: source.len(),
            destination: destination.len(),
        });
    }

    destination.clone_from_slice(source);
    Ok(())
}

/// Fills a slice with a cloned value.
pub fn fill_exact<T>(
    destination: &mut [T],
    value: T,
) where
    T: Clone,
{
    destination.fill(value);
}

// =============================================================================
// CPU memory policy
// =============================================================================

/// Policy describing how CPU memory should be used.
///
/// This is deliberately a policy object, not a replacement for global
/// `MemoryLimits`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuMemoryPolicy {
    preferred_kind: CpuMemoryKind,
    allow_staging: bool,
    allow_numa_metadata: bool,
}

impl CpuMemoryPolicy {
    /// Conservative pageable-memory policy.
    pub const fn conservative() -> Self {
        Self {
            preferred_kind: CpuMemoryKind::Pageable,
            allow_staging: true,
            allow_numa_metadata: true,
        }
    }

    /// Creates a policy with an explicit preferred kind.
    pub const fn new(preferred_kind: CpuMemoryKind) -> Self {
        Self {
            preferred_kind,
            allow_staging: true,
            allow_numa_metadata: true,
        }
    }

    /// Returns preferred memory kind.
    pub const fn preferred_kind(&self) -> CpuMemoryKind {
        self.preferred_kind
    }

    /// Returns whether staging memory is permitted.
    pub const fn allow_staging(&self) -> bool {
        self.allow_staging
    }

    /// Returns whether NUMA metadata may be used.
    pub const fn allow_numa_metadata(&self) -> bool {
        self.allow_numa_metadata
    }

    /// Disables staging memory.
    pub const fn without_staging(mut self) -> Self {
        self.allow_staging = false;
        self
    }

    /// Disables NUMA metadata.
    pub const fn without_numa_metadata(mut self) -> Self {
        self.allow_numa_metadata = false;
        self
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_zero_is_invalid() {
        assert_eq!(CpuDomainId::new(0), None);
        assert_eq!(CpuDomainId::new(1).map(CpuDomainId::get), Some(1));
    }

    #[test]
    fn topology_rejects_zero_cpus() {
        assert!(CpuTopology::new(0).is_err());
    }

    #[test]
    fn topology_accepts_valid_cpu_count() {
        let topology = CpuTopology::new(4).expect("valid topology");

        assert_eq!(topology.logical_cpus(), 4);
        assert_eq!(topology.domain_count(), 0);
    }

    #[test]
    fn domain_metadata_is_safe() {
        let domain = CpuMemoryDomain::new(
            CpuDomainId::new(1).expect("valid domain"),
            "node0",
        )
        .expect("valid domain");

        let domain = domain
            .with_memory_bytes(1024)
            .expect("valid memory")
            .with_thread_count(8)
            .expect("valid threads");

        assert_eq!(domain.id().get(), 1);
        assert_eq!(domain.memory_bytes(), Some(1024));
        assert_eq!(domain.thread_count(), Some(8));
    }

    #[test]
    fn duplicate_domains_are_rejected() {
        let domain_a = CpuMemoryDomain::new(
            CpuDomainId::new(1).expect("valid domain"),
            "node0",
        )
        .expect("valid domain");

        let domain_b = CpuMemoryDomain::new(
            CpuDomainId::new(1).expect("valid domain"),
            "node0-duplicate",
        )
        .expect("valid domain");

        let topology = CpuTopology::new(4)
            .expect("valid topology")
            .with_domain(domain_a)
            .expect("first domain");

        assert!(topology.with_domain(domain_b).is_err());
    }

    #[test]
    fn requirement_calculates_bytes_safely() {
        let requirement =
            CpuMemoryRequirement::new(1024, 16).expect("valid requirement");

        assert_eq!(requirement.elements(), 1024);
        assert_eq!(requirement.element_size(), 16);
        assert_eq!(requirement.bytes(), 16_384);
    }

    #[test]
    fn requirement_rejects_zero_elements() {
        assert!(CpuMemoryRequirement::new(0, 16).is_err());
    }

    #[test]
    fn requirement_detects_overflow() {
        assert!(CpuMemoryRequirement::new(u64::MAX, 16).is_err());
    }

    #[test]
    fn state_vector_estimation_is_correct() {
        assert_eq!(state_vector_elements(0).expect("zero qubits"), 1);
        assert_eq!(state_vector_elements(10).expect("ten qubits"), 1024);
    }

    #[test]
    fn density_matrix_estimation_is_correct() {
        assert_eq!(
            density_matrix_elements(0).expect("zero qubits"),
            1
        );
        assert_eq!(
            density_matrix_elements(3).expect("three qubits"),
            64
        );
    }

    #[test]
    fn f64_state_vector_estimation_is_correct() {
        // 10 qubits = 1024 complex elements.
        // The caller determines whether T is a scalar or complex storage type.
        let requirement =
            estimate_bytes::<f64>(1024).expect("valid estimate");

        assert_eq!(requirement.bytes(), 1024 * 8);
    }

    #[test]
    fn buffer_allocates_without_unsafe() {
        let buffer =
            CpuBuffer::<u64>::try_with_len(16, CpuMemoryKind::Pageable)
                .expect("small allocation");

        assert_eq!(buffer.len(), 16);
        assert_eq!(buffer.byte_len(), 16 * 8);
    }

    #[test]
    fn buffer_is_initialized() {
        let buffer =
            CpuBuffer::<u64>::try_with_len(4, CpuMemoryKind::Pageable)
                .expect("small allocation");

        assert_eq!(buffer.as_slice(), &[0, 0, 0, 0]);
    }

    #[test]
    fn filled_buffer_is_initialized() {
        let buffer =
            CpuBuffer::<u64>::try_filled(4, 7, CpuMemoryKind::Pageable)
                .expect("small allocation");

        assert_eq!(buffer.as_slice(), &[7, 7, 7, 7]);
    }

    #[test]
    fn checked_access_rejects_invalid_index() {
        let buffer =
            CpuBuffer::<u64>::try_with_len(4, CpuMemoryKind::Pageable)
                .expect("small allocation");

        assert!(buffer.get(4).is_err());
    }

    #[test]
    fn checked_range_rejects_invalid_range() {
        let buffer =
            CpuBuffer::<u64>::try_with_len(4, CpuMemoryKind::Pageable)
                .expect("small allocation");

        assert!(buffer.range(3, 5).is_err());
        assert!(buffer.range(4, 3).is_err());
    }

    #[test]
    fn copy_requires_equal_lengths() {
        let source =
            CpuBuffer::<u64>::try_filled(4, 1, CpuMemoryKind::Pageable)
                .expect("source");

        let mut destination =
            CpuBuffer::<u64>::try_filled(3, 0, CpuMemoryKind::Pageable)
                .expect("destination");

        assert!(destination.copy_from(&source).is_err());
    }

    #[test]
    fn copy_preserves_values() {
        let source =
            CpuBuffer::<u64>::try_filled(4, 42, CpuMemoryKind::Pageable)
                .expect("source");

        let mut destination =
            CpuBuffer::<u64>::try_with_len(4, CpuMemoryKind::Pageable)
                .expect("destination");

        destination
            .copy_from(&source)
            .expect("equal length copy");

        assert_eq!(destination.as_slice(), &[42, 42, 42, 42]);
    }

    #[test]
    fn byte_buffer_works() {
        let mut buffer =
            CpuByteBuffer::try_with_len(128, CpuMemoryKind::Staging)
                .expect("small staging allocation");

        assert_eq!(buffer.len(), 128);

        buffer.as_mut_slice()[0] = 42;

        assert_eq!(
            *buffer.get(0).expect("valid byte"),
            42
        );
    }

    #[test]
    fn conservative_manager_is_constructible() {
        let manager = CpuMemoryManager::conservative();

        assert!(manager.topology().logical_cpus() >= 1);
    }

    #[test]
    fn manager_allocates() {
        let mut manager = CpuMemoryManager::conservative();

        let buffer = manager
            .try_allocate::<u64>(8, CpuMemoryKind::Pageable)
            .expect("small allocation");

        assert_eq!(buffer.len(), 8);
        assert_eq!(manager.statistics().allocations(), 1);
        assert_eq!(manager.statistics().current_bytes(), 64);
    }

    #[test]
    fn policy_is_provider_neutral() {
        let policy = CpuMemoryPolicy::new(CpuMemoryKind::Staging);

        assert_eq!(
            policy.preferred_kind(),
            CpuMemoryKind::Staging
        );
        assert!(policy.allow_staging());
    }

    #[test]
    fn exact_copy_works() {
        let source = [1u64, 2, 3, 4];
        let mut destination = [0u64; 4];

        copy_exact(&mut destination, &source)
            .expect("equal length copy");

        assert_eq!(destination, source);
    }

    #[test]
    fn exact_copy_rejects_mismatch() {
        let source = [1u64, 2, 3];
        let mut destination = [0u64; 4];

        assert!(copy_exact(&mut destination, &source).is_err());
    }
}