//! Zamani Quantum Memory — Abstract Memory Addresses
//!
//! This module defines the representation-independent address model used by
//! `quantum::memory`.
//!
//! # Architectural role
//!
//! `address.rs` does **not** expose raw CPU pointers, GPU pointers, physical
//! addresses, MMIO addresses, or vendor-specific device handles.
//!
//! Instead, it represents an address as a safe, opaque reference to a managed
//! memory resource plus an offset and address-space classification.
//!
//! The fundamental model is:
//!
//! ```text
//!                    Quantum Memory Address
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          │                 │                 │
//!        Host             Device          Distributed
//!          │                 │                 │
//!      allocation        allocation       partition
//!          │                 │                 │
//!          └─────────────────┼─────────────────┘
//!                            │
//!                       Remote/Backend
//!                            │
//!                      opaque handle
//! ```
//!
//! # Why addresses are abstract
//!
//! Zamani must be capable of executing quantum workloads against:
//!
//! - ordinary host RAM;
//! - pinned host memory;
//! - NUMA host memory;
//! - CPU simulator memory;
//! - GPU/device memory;
//! - unified/managed accelerator memory;
//! - distributed simulator memory;
//! - remote simulator memory;
//! - remote QPU services;
//! - hardware-specific memory/session objects;
//! - future accelerator architectures.
//!
//! A raw pointer cannot represent all of these safely or portably.
//!
//! In particular, a remote QPU normally does not expose a process-visible
//! address for its quantum state. The useful identity may instead be a backend
//! session, job, state, register, or other opaque backend object.
//!
//! Therefore this module deliberately represents **logical memory locations**,
//! not machine pointers.
//!
//! # Safety
//!
//! This module:
//!
//! - uses no `unsafe`;
//! - exposes no raw pointers;
//! - performs no pointer arithmetic;
//! - performs checked offset arithmetic;
//! - does not dereference addresses;
//! - does not claim that an address is physically valid;
//! - does not allocate memory;
//! - does not free memory;
//! - does not perform I/O.
//!
//! # Address validity
//!
//! An `Address` is a *descriptive capability/reference*, not proof that the
//! underlying storage is currently accessible.
//!
//! Actual validity is established by the owning allocator/backend.
//!
//! This distinction is important because memory may be:
//!
//! - allocated but not mapped;
//! - resident on another NUMA node;
//! - resident on a GPU;
//! - temporarily migrating;
//! - remotely owned;
//! - released;
//! - invalidated by an allocator;
//! - owned by an external QPU service.
//!
//! `address.rs` therefore does not maintain allocator state.
//!
//! # Identity model
//!
//! The canonical identity types are defined by `memory::types`:
//!
//! ```text
//! MemoryId
//! AllocationId
//! BackendMemoryId
//! ByteCount
//! ```
//!
//! This module consumes those types and does not redefine them.
//!
//! Logical/physical qubit identity remains owned by `quantum::ir`.
//!
//! # Integration contract
//!
//! Later memory modules must use this address model instead of introducing:
//!
//! - `*const T`;
//! - `*mut T`;
//! - raw `usize` addresses;
//! - raw CUDA pointers;
//! - raw Metal/Vulkan handles;
//! - vendor-specific device addresses;
//! - process virtual addresses.
//!
//! Concrete allocators may internally use implementation-specific mechanisms,
//! but those mechanisms must remain behind their safe public abstraction.
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
//!
//! # Dependency boundary
//!
//! This module may depend on:
//!
//! ```text
//! quantum::memory::types
//! ```
//!
//! It must not depend on:
//!
//! ```text
//! allocator
//! pool
//! reservation
//! budget
//! state
//! state_vector
//! density_matrix
//! stabilizer
//! sparse
//! tensor_network
//! gpu
//! distributed
//! routing
//! scheduling
//! hardware vendors
//! benchmarking
//! runtime
//! ```
//!
//! This keeps the address contract foundational and prevents circular
//! dependencies.
//!
//! # Design principle
//!
//! The address layer answers:
//!
//! > "Which managed memory resource and offset are we referring to?"
//!
//! It does not answer:
//!
//! > "How do I access that memory?"
//!
//! Access belongs to `allocator.rs`, `cpu.rs`, `gpu.rs`, `distributed.rs`,
//! backend adapters, or other appropriate modules.
//!
//! # Remote QPU principle
//!
//! A remote QPU is represented through an opaque backend identity rather than
//! pretending that Zamani can address the QPU's internal physical memory.
//!
//! Thus the same address abstraction can describe:
//!
//! ```text
//! local CPU state
//! local GPU state
//! distributed simulator partition
//! remote simulator object
//! remote QPU memory/session object
//! ```
//!
//! without vendor lock-in.
//!
//! # Determinism
//!
//! Address values are deterministic data structures. This module does not
//! generate identities. Identity allocation belongs to allocator/backend
//! implementations.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::types::{AllocationId, BackendMemoryId, ByteCount, MemoryId};

// =============================================================================
// Address-space classification
// =============================================================================

/// The broad memory/address space in which an address is interpreted.
///
/// This is intentionally generic and vendor-neutral.
///
/// It describes *where an address belongs*, not how the address is accessed.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum AddressSpace {
    /// Ordinary process-visible host memory.
    Host,

    /// Host memory allocated for accelerator/device transfer.
    ///
    /// The exact pinning mechanism belongs to the allocator/backend.
    PinnedHost,

    /// Memory associated with a NUMA node.
    ///
    /// NUMA topology and placement policies belong elsewhere.
    Numa,

    /// Accelerator/device memory.
    ///
    /// This intentionally does not mean CUDA specifically.
    Device,

    /// Unified or managed memory visible through more than one execution
    /// domain.
    Unified,

    /// Memory owned by a distributed execution domain.
    Distributed,

    /// Memory owned by a remote simulator/service.
    RemoteSimulator,

    /// Memory/session/state object owned by a remote quantum backend or QPU.
    RemoteQpu,

    /// Opaque backend-managed memory whose exact execution environment is not
    /// exposed by the generic memory layer.
    BackendOpaque,
}

impl AddressSpace {
    /// Returns `true` when the address normally refers to process-local memory.
    ///
    /// This does not guarantee direct CPU dereferenceability.
    pub const fn is_host_local(self) -> bool {
        matches!(
            self,
            Self::Host | Self::PinnedHost | Self::Numa | Self::Unified
        )
    }

    /// Returns `true` when the address belongs to an accelerator/device
    /// memory domain.
    pub const fn is_device(self) -> bool {
        matches!(self, Self::Device | Self::Unified)
    }

    /// Returns `true` when the address belongs to distributed memory.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns `true` when the address is remotely owned.
    pub const fn is_remote(self) -> bool {
        matches!(
            self,
            Self::RemoteSimulator | Self::RemoteQpu | Self::BackendOpaque
        )
    }

    /// Returns whether this address space may represent a QPU-owned object.
    pub const fn supports_remote_qpu(self) -> bool {
        matches!(self, Self::RemoteQpu | Self::BackendOpaque)
    }
}

impl fmt::Display for AddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned-host",
            Self::Numa => "numa",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::RemoteSimulator => "remote-simulator",
            Self::RemoteQpu => "remote-qpu",
            Self::BackendOpaque => "backend-opaque",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Address validation errors
// =============================================================================

/// Errors produced while constructing or transforming abstract addresses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AddressError {
    /// The supplied alignment was zero.
    ZeroAlignment,

    /// The supplied alignment was not a power of two.
    NonPowerOfTwoAlignment {
        /// The invalid alignment.
        alignment: u64,
    },

    /// The offset was not aligned as requested.
    MisalignedOffset {
        /// Requested offset.
        offset: u64,

        /// Required alignment.
        alignment: u64,
    },

    /// Address offset arithmetic overflowed.
    OffsetOverflow {
        /// Existing offset.
        current: u64,

        /// Requested additional offset.
        additional: u64,
    },

    /// The requested range extends beyond the allocation.
    OutOfBounds {
        /// Start offset.
        offset: u64,

        /// Requested length.
        length: u64,

        /// Allocation size.
        allocation_size: u64,
    },

    /// The supplied address has no allocation/resource identity.
    MissingResourceIdentity,

    /// A local allocation address was created without an allocation identity.
    MissingAllocationIdentity,

    /// A remote address was created without a backend memory identity.
    MissingBackendIdentity,

    /// An address was used with a resource whose identity does not match.
    ResourceMismatch {
        /// Address resource identity.
        address_resource: u64,

        /// Expected resource identity.
        expected_resource: u64,
    },

    /// An address was used with an allocation whose identity does not match.
    AllocationMismatch {
        /// Address allocation identity.
        address_allocation: u64,

        /// Expected allocation identity.
        expected_allocation: u64,
    },

    /// The requested address operation is not meaningful for the address
    /// space.
    UnsupportedOperation {
        /// Address space involved.
        space: AddressSpace,
    },

    /// A range end overflowed.
    RangeOverflow {
        /// Start offset.
        offset: u64,

        /// Range length.
        length: u64,
    },

    /// An address range has an invalid ordering.
    InvalidRange {
        /// Start.
        start: u64,

        /// End.
        end: u64,
    },
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroAlignment => {
                f.write_str("address alignment must be greater than zero")
            }

            Self::NonPowerOfTwoAlignment { alignment } => {
                write!(
                    f,
                    "address alignment {alignment} is not a power of two"
                )
            }

            Self::MisalignedOffset {
                offset,
                alignment,
            } => {
                write!(
                    f,
                    "address offset {offset} is not aligned to {alignment} bytes"
                )
            }

            Self::OffsetOverflow {
                current,
                additional,
            } => {
                write!(
                    f,
                    "address offset overflow: {current} + {additional}"
                )
            }

            Self::OutOfBounds {
                offset,
                length,
                allocation_size,
            } => {
                write!(
                    f,
                    "address range [{offset}, {}) exceeds allocation size {allocation_size}",
                    offset.saturating_add(*length)
                )
            }

            Self::MissingResourceIdentity => {
                f.write_str("address is missing its memory resource identity")
            }

            Self::MissingAllocationIdentity => {
                f.write_str("local address is missing its allocation identity")
            }

            Self::MissingBackendIdentity => {
                f.write_str("remote/backend address is missing its backend identity")
            }

            Self::ResourceMismatch {
                address_resource,
                expected_resource,
            } => {
                write!(
                    f,
                    "memory resource mismatch: address={address_resource}, expected={expected_resource}"
                )
            }

            Self::AllocationMismatch {
                address_allocation,
                expected_allocation,
            } => {
                write!(
                    f,
                    "allocation mismatch: address={address_allocation}, expected={expected_allocation}"
                )
            }

            Self::UnsupportedOperation { space } => {
                write!(
                    f,
                    "operation is not supported for address space {space}"
                )
            }

            Self::RangeOverflow { offset, length } => {
                write!(
                    f,
                    "address range overflow: {offset} + {length}"
                )
            }

            Self::InvalidRange { start, end } => {
                write!(
                    f,
                    "invalid address range: start {start} > end {end}"
                )
            }
        }
    }
}

impl std::error::Error for AddressError {}

// =============================================================================
// Address identity
// =============================================================================

/// Stable identity of an abstract memory address.
///
/// This is not a machine pointer.
///
/// It identifies:
///
/// ```text
/// memory resource
///       │
///       └── allocation/backend object
///                    │
///                    └── byte offset
/// ```
///
/// The structure is intentionally copyable because it is metadata rather than
/// ownership.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Address {
    /// Address-space classification.
    space: AddressSpace,

    /// Managed memory resource containing the address.
    memory: MemoryId,

    /// Allocation identity for local managed allocations, when applicable.
    allocation: Option<AllocationId>,

    /// Backend-owned identity for remote/backend-managed memory, when
    /// applicable.
    backend: Option<BackendMemoryId>,

    /// Byte offset relative to the identified resource/allocation.
    offset: u64,
}

impl Address {
    /// Creates a local allocation-backed address.
    ///
    /// This constructor is appropriate for:
    ///
    /// - host memory;
    /// - pinned host memory;
    /// - NUMA memory;
    /// - device memory;
    /// - unified memory;
    /// - distributed memory.
    ///
    /// The actual storage remains owned by the allocator.
    pub const fn local(
        space: AddressSpace,
        memory: MemoryId,
        allocation: AllocationId,
        offset: u64,
    ) -> Self {
        Self {
            space,
            memory,
            allocation: Some(allocation),
            backend: None,
            offset,
        }
    }

    /// Creates a backend-owned address.
    ///
    /// This is appropriate when the underlying system is represented by an
    /// opaque backend identity rather than a process-local allocation.
    ///
    /// Typical uses include:
    ///
    /// - remote simulators;
    /// - remote QPUs;
    /// - provider-managed state/session objects;
    /// - vendor-neutral backend plugins.
    pub const fn backend(
        space: AddressSpace,
        memory: MemoryId,
        backend: BackendMemoryId,
        offset: u64,
    ) -> Self {
        Self {
            space,
            memory,
            allocation: None,
            backend: Some(backend),
            offset,
        }
    }

    /// Returns the address space.
    pub const fn space(self) -> AddressSpace {
        self.space
    }

    /// Returns the owning memory resource identity.
    pub const fn memory(self) -> MemoryId {
        self.memory
    }

    /// Returns the local allocation identity, if present.
    pub const fn allocation(self) -> Option<AllocationId> {
        self.allocation
    }

    /// Returns the backend identity, if present.
    pub const fn backend_memory(self) -> Option<BackendMemoryId> {
        self.backend
    }

    /// Returns the byte offset.
    pub const fn offset(self) -> u64 {
        self.offset
    }

    /// Returns whether this address is local-allocation-backed.
    pub const fn is_local(self) -> bool {
        self.allocation.is_some()
    }

    /// Returns whether this address is backend-owned.
    pub const fn is_backend_owned(self) -> bool {
        self.backend.is_some()
    }

    /// Returns whether this address belongs to a remote address space.
    pub const fn is_remote(self) -> bool {
        self.space.is_remote()
    }

    /// Creates another address by adding a byte offset.
    ///
    /// No pointer arithmetic is performed.
    pub fn checked_add(self, additional: u64) -> Result<Self, AddressError> {
        let offset = self.offset.checked_add(additional).ok_or(
            AddressError::OffsetOverflow {
                current: self.offset,
                additional,
            },
        )?;

        Ok(Self {
            offset,
            ..self
        })
    }

    /// Creates another address by subtracting a byte offset.
    pub fn checked_sub(self, amount: u64) -> Result<Self, AddressError> {
        let offset = self
            .offset
            .checked_sub(amount)
            .ok_or(AddressError::RangeOverflow {
                offset: self.offset,
                length: amount,
            })?;

        Ok(Self {
            offset,
            ..self
        })
    }

    /// Returns the address range `[offset, offset + length)`.
    pub fn checked_range(self, length: u64) -> Result<AddressRange, AddressError> {
        let end = self.offset.checked_add(length).ok_or(
            AddressError::RangeOverflow {
                offset: self.offset,
                length,
            },
        )?;

        Ok(AddressRange {
            start: self.offset,
            end,
        })
    }

    /// Checks whether a range beginning at this address fits within an
    /// allocation of the specified size.
    pub fn validate_range(
        self,
        length: u64,
        allocation_size: ByteCount,
    ) -> Result<AddressRange, AddressError> {
        let range = self.checked_range(length)?;

        if range.end > allocation_size.get() {
            return Err(AddressError::OutOfBounds {
                offset: self.offset,
                length,
                allocation_size: allocation_size.get(),
            });
        }

        Ok(range)
    }

    /// Validates that the address has the expected memory resource identity.
    pub fn validate_memory(self, expected: MemoryId) -> Result<(), AddressError> {
        if self.memory != expected {
            return Err(AddressError::ResourceMismatch {
                address_resource: self.memory.value(),
                expected_resource: expected.value(),
            });
        }

        Ok(())
    }

    /// Validates that the address refers to the expected local allocation.
    pub fn validate_allocation(
        self,
        expected: AllocationId,
    ) -> Result<(), AddressError> {
        let actual = self
            .allocation
            .ok_or(AddressError::MissingAllocationIdentity)?;

        if actual != expected {
            return Err(AddressError::AllocationMismatch {
                address_allocation: actual.value(),
                expected_allocation: expected.value(),
            });
        }

        Ok(())
    }

    /// Validates that the address refers to the expected backend memory
    /// object.
    pub fn validate_backend(
        self,
        expected: BackendMemoryId,
    ) -> Result<(), AddressError> {
        let actual = self
            .backend
            .ok_or(AddressError::MissingBackendIdentity)?;

        if actual != expected {
            return Err(AddressError::ResourceMismatch {
                address_resource: actual.value(),
                expected_resource: expected.value(),
            });
        }

        Ok(())
    }

    /// Validates the address against its structural invariants.
    ///
    /// This does not validate that the allocator/backend currently has live
    /// storage at the address.
    pub fn validate(self) -> Result<(), AddressError> {
        if self.memory.value() == 0 {
            return Err(AddressError::MissingResourceIdentity);
        }

        match self.space {
            AddressSpace::RemoteSimulator
            | AddressSpace::RemoteQpu
            | AddressSpace::BackendOpaque => {
                if self.backend.is_none() {
                    return Err(AddressError::MissingBackendIdentity);
                }
            }

            _ => {
                if self.allocation.is_none() {
                    return Err(AddressError::MissingAllocationIdentity);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}+{}",
            self.space,
            self.memory.value(),
            self.offset
        )?;

        if let Some(allocation) = self.allocation {
            write!(f, "/{}", allocation.value())?;
        }

        if let Some(backend) = self.backend {
            write!(f, "/backend:{}", backend.value())?;
        }

        Ok(())
    }
}

// =============================================================================
// Address ranges
// =============================================================================

/// A half-open byte range `[start, end)`.
///
/// The end is exclusive.
///
/// This is useful for:
///
/// - allocator validation;
/// - buffer views;
/// - DMA transfers;
/// - GPU copies;
/// - distributed transfers;
/// - serialization;
/// - state/tensor buffers.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct AddressRange {
    start: u64,
    end: u64,
}

impl AddressRange {
    /// Creates a checked address range.
    pub const fn new(start: u64, end: u64) -> Result<Self, AddressError> {
        if start > end {
            return Err(AddressError::InvalidRange { start, end });
        }

        Ok(Self { start, end })
    }

    /// Returns the inclusive start.
    pub const fn start(self) -> u64 {
        self.start
    }

    /// Returns the exclusive end.
    pub const fn end(self) -> u64 {
        self.end
    }

    /// Returns the length in bytes.
    pub const fn len(self) -> u64 {
        self.end - self.start
    }

    /// Returns whether the range is empty.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether this range contains the supplied byte offset.
    pub const fn contains(self, offset: u64) -> bool {
        offset >= self.start && offset < self.end
    }

    /// Returns whether this range completely contains another range.
    pub const fn contains_range(self, other: Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    /// Returns whether two ranges overlap.
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns the intersection of two ranges.
    pub const fn intersection(self, other: Self) -> Option<Self> {
        let start = if self.start > other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end < other.end {
            self.end
        } else {
            other.end
        };

        if start >= end {
            None
        } else {
            Some(Self { start, end })
        }
    }

    /// Extends this range by `additional` bytes.
    pub const fn checked_extend(
        self,
        additional: u64,
    ) -> Result<Self, AddressError> {
        let end = match self.end.checked_add(additional) {
            Some(value) => value,
            None => {
                return Err(AddressError::RangeOverflow {
                    offset: self.end,
                    length: additional,
                })
            }
        };

        Ok(Self {
            start: self.start,
            end,
        })
    }
}

// =============================================================================
// Alignment
// =============================================================================

/// A validated memory alignment.
///
/// Alignment is represented as a positive power of two.
///
/// This is deliberately an abstract alignment rather than a Rust layout or
/// pointer type, allowing it to be used for CPU, GPU, accelerator, DMA and
/// distributed buffers.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Alignment(u64);

impl Alignment {
    /// One-byte alignment.
    pub const ONE: Self = Self(1);

    /// Creates an alignment after validation.
    pub const fn new(value: u64) -> Result<Self, AddressError> {
        if value == 0 {
            return Err(AddressError::ZeroAlignment);
        }

        if !value.is_power_of_two() {
            return Err(AddressError::NonPowerOfTwoAlignment { alignment: value });
        }

        Ok(Self(value))
    }

    /// Returns the alignment in bytes.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns whether an offset satisfies this alignment.
    pub const fn is_aligned(self, offset: u64) -> bool {
        offset & (self.0 - 1) == 0
    }

    /// Validates an offset against this alignment.
    pub const fn validate_offset(
        self,
        offset: u64,
    ) -> Result<(), AddressError> {
        if self.is_aligned(offset) {
            Ok(())
        } else {
            Err(AddressError::MisalignedOffset {
                offset,
                alignment: self.0,
            })
        }
    }

    /// Rounds `value` up to the next aligned value.
    pub const fn align_up(self, value: u64) -> Result<u64, AddressError> {
        let mask = self.0 - 1;

        let adjusted = match value.checked_add(mask) {
            Some(value) => value,
            None => {
                return Err(AddressError::RangeOverflow {
                    offset: value,
                    length: mask,
                })
            }
        };

        Ok(adjusted & !mask)
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self::ONE
    }
}

// =============================================================================
// Address descriptor
// =============================================================================

/// Complete metadata describing an addressable memory region.
///
/// This type is useful when an allocator needs to communicate:
///
/// - where the resource belongs;
/// - which allocation owns it;
/// - its size;
/// - its alignment;
/// - the address-space classification.
///
/// It still does not expose a machine pointer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AddressDescriptor {
    /// Base address of the region.
    base: Address,

    /// Size of the region.
    size: ByteCount,

    /// Required alignment.
    alignment: Alignment,
}

impl AddressDescriptor {
    /// Creates a descriptor for a local allocation.
    pub fn local(
        space: AddressSpace,
        memory: MemoryId,
        allocation: AllocationId,
        size: ByteCount,
        alignment: Alignment,
    ) -> Result<Self, AddressError> {
        let base = Address::local(space, memory, allocation, 0);

        base.validate()?;
        alignment.validate_offset(0)?;

        Ok(Self {
            base,
            size,
            alignment,
        })
    }

    /// Creates a descriptor for backend-owned memory.
    pub fn backend(
        space: AddressSpace,
        memory: MemoryId,
        backend: BackendMemoryId,
        size: ByteCount,
        alignment: Alignment,
    ) -> Result<Self, AddressError> {
        let base = Address::backend(space, memory, backend, 0);

        base.validate()?;
        alignment.validate_offset(0)?;

        Ok(Self {
            base,
            size,
            alignment,
        })
    }

    /// Returns the base address.
    pub const fn base(self) -> Address {
        self.base
    }

    /// Returns the size.
    pub const fn size(self) -> ByteCount {
        self.size
    }

    /// Returns the alignment.
    pub const fn alignment(self) -> Alignment {
        self.alignment
    }

    /// Returns the address immediately at `offset` bytes into this region.
    ///
    /// The resulting address must remain inside the descriptor.
    pub fn address_at(self, offset: u64) -> Result<Address, AddressError> {
        self.alignment.validate_offset(offset)?;

        if offset > self.size.get() {
            return Err(AddressError::OutOfBounds {
                offset,
                length: 0,
                allocation_size: self.size.get(),
            });
        }

        self.base.checked_add(offset)
    }

    /// Returns an address range beginning at `offset`.
    pub fn range_at(
        self,
        offset: u64,
        length: u64,
    ) -> Result<(Address, AddressRange), AddressError> {
        self.alignment.validate_offset(offset)?;

        let address = self.base.checked_add(offset)?;
        let range = address.validate_range(length, self.size)?;

        Ok((address, range))
    }

    /// Validates that the complete descriptor is internally consistent.
    pub fn validate(self) -> Result<(), AddressError> {
        self.base.validate()?;
        self.alignment.validate_offset(self.base.offset())?;

        if self.base.offset() > self.size.get() {
            return Err(AddressError::OutOfBounds {
                offset: self.base.offset(),
                length: 0,
                allocation_size: self.size.get(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Backend address reference
// =============================================================================

/// An explicit reference to backend-owned memory.
///
/// This type is useful for remote QPUs and remote execution systems where
/// Zamani must never pretend that the provider exposes a normal CPU/GPU
/// pointer.
///
/// The `BackendMemoryId` identifies the backend-managed object while `offset`
/// allows backends that expose logical byte ranges to represent subregions.
///
/// Backends that do not expose offsets should use offset `0`.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct BackendAddress {
    /// Generic address-space classification.
    space: AddressSpace,

    /// Zamani memory-resource identity.
    memory: MemoryId,

    /// Opaque backend memory identity.
    backend: BackendMemoryId,

    /// Logical byte offset.
    offset: u64,
}

impl BackendAddress {
    /// Creates a backend address.
    ///
    /// The address space must be one of:
    ///
    /// - `RemoteSimulator`;
    /// - `RemoteQpu`;
    /// - `BackendOpaque`.
    pub fn new(
        space: AddressSpace,
        memory: MemoryId,
        backend: BackendMemoryId,
        offset: u64,
    ) -> Result<Self, AddressError> {
        if !space.is_remote() {
            return Err(AddressError::UnsupportedOperation { space });
        }

        if memory.value() == 0 {
            return Err(AddressError::MissingResourceIdentity);
        }

        Ok(Self {
            space,
            memory,
            backend,
            offset,
        })
    }

    /// Returns the address space.
    pub const fn space(self) -> AddressSpace {
        self.space
    }

    /// Returns the Zamani memory resource.
    pub const fn memory(self) -> MemoryId {
        self.memory
    }

    /// Returns the backend-owned identity.
    pub const fn backend(self) -> BackendMemoryId {
        self.backend
    }

    /// Returns the logical offset.
    pub const fn offset(self) -> u64 {
        self.offset
    }

    /// Adds a logical byte offset.
    pub fn checked_add(self, amount: u64) -> Result<Self, AddressError> {
        let offset = self.offset.checked_add(amount).ok_or(
            AddressError::OffsetOverflow {
                current: self.offset,
                additional: amount,
            },
        )?;

        Ok(Self { offset, ..self })
    }

    /// Converts this backend reference into the generic `Address`.
    pub const fn as_address(self) -> Address {
        Address::backend(
            self.space,
            self.memory,
            self.backend,
            self.offset,
        )
    }
}

// =============================================================================
// Distributed address
// =============================================================================

/// Logical identity of a distributed memory partition.
///
/// This does not contain a network address or transport-specific handle.
///
/// Communication belongs to `distributed.rs`.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct DistributedAddress {
    /// Global memory resource.
    memory: MemoryId,

    /// Allocation containing the distributed region.
    allocation: AllocationId,

    /// Logical partition identifier.
    partition: u64,

    /// Offset inside the partition.
    offset: u64,
}

impl DistributedAddress {
    /// Creates a distributed address.
    pub const fn new(
        memory: MemoryId,
        allocation: AllocationId,
        partition: u64,
        offset: u64,
    ) -> Self {
        Self {
            memory,
            allocation,
            partition,
            offset,
        }
    }

    /// Returns the memory resource.
    pub const fn memory(self) -> MemoryId {
        self.memory
    }

    /// Returns the allocation.
    pub const fn allocation(self) -> AllocationId {
        self.allocation
    }

    /// Returns the partition identity.
    pub const fn partition(self) -> u64 {
        self.partition
    }

    /// Returns the partition-local offset.
    pub const fn offset(self) -> u64 {
        self.offset
    }

    /// Converts this distributed address into the generic address model.
    ///
    /// The partition identity remains available separately because a generic
    /// byte offset alone cannot identify which distributed partition owns the
    /// data.
    pub const fn as_address(self) -> Address {
        Address::local(
            AddressSpace::Distributed,
            self.memory,
            self.allocation,
            self.offset,
        )
    }

    /// Adds a partition-local byte offset.
    pub fn checked_add(self, amount: u64) -> Result<Self, AddressError> {
        let offset = self.offset.checked_add(amount).ok_or(
            AddressError::OffsetOverflow {
                current: self.offset,
                additional: amount,
            },
        )?;

        Ok(Self { offset, ..self })
    }
}

// =============================================================================
// Address identity key
// =============================================================================

/// Stable comparison key for determining whether two addresses can refer to
/// the same underlying memory domain.
///
/// This is intentionally separate from `Address` so callers can perform
/// identity comparisons without depending on offsets.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum AddressIdentity {
    /// Local allocation identity.
    Local {
        /// Memory resource.
        memory: MemoryId,

        /// Allocation.
        allocation: AllocationId,
    },

    /// Backend-owned identity.
    Backend {
        /// Memory resource.
        memory: MemoryId,

        /// Backend memory object.
        backend: BackendMemoryId,
    },
}

impl AddressIdentity {
    /// Returns the identity of an address.
    pub const fn of(address: Address) -> Option<Self> {
        if let Some(allocation) = address.allocation {
            return Some(Self::Local {
                memory: address.memory,
                allocation,
            });
        }

        if let Some(backend) = address.backend {
            return Some(Self::Backend {
                memory: address.memory,
                backend,
            });
        }

        None
    }
}

impl Address {
    /// Returns the stable underlying-resource identity.
    pub const fn identity(self) -> Option<AddressIdentity> {
        AddressIdentity::of(self)
    }

    /// Returns whether two addresses belong to the same underlying allocation
    /// or backend memory object.
    pub const fn same_resource(self, other: Self) -> bool {
        match (self.identity(), other.identity()) {
            (
                Some(AddressIdentity::Local {
                    memory: memory_a,
                    allocation: allocation_a,
                }),
                Some(AddressIdentity::Local {
                    memory: memory_b,
                    allocation: allocation_b,
                }),
            ) => memory_a.value() == memory_b.value()
                && allocation_a.value() == allocation_b.value(),

            (
                Some(AddressIdentity::Backend {
                    memory: memory_a,
                    backend: backend_a,
                }),
                Some(AddressIdentity::Backend {
                    memory: memory_b,
                    backend: backend_b,
                }),
            ) => {
                memory_a.value() == memory_b.value()
                    && backend_a.value() == backend_b.value()
            }

            _ => false,
        }
    }

    /// Returns the byte distance between two addresses belonging to the same
    /// resource.
    ///
    /// The result is signed so that addresses can be compared in either
    /// direction without wrapping.
    pub fn checked_distance(self, other: Self) -> Result<i128, AddressError> {
        if !self.same_resource(other) {
            return Err(AddressError::ResourceMismatch {
                address_resource: self.memory.value(),
                expected_resource: other.memory.value(),
            });
        }

        Ok(i128::from(self.offset) - i128::from(other.offset))
    }
}

// =============================================================================
// Address capability
// =============================================================================

/// Describes what operations a backend/allocator may permit for an address.
///
/// This is metadata only. It does not grant access by itself.
///
/// A backend may impose stricter permissions.
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
pub struct AddressCapabilities {
    /// CPU-side reads are permitted by the owning backend.
    cpu_read: bool,

    /// CPU-side writes are permitted by the owning backend.
    cpu_write: bool,

    /// Device-side reads are permitted.
    device_read: bool,

    /// Device-side writes are permitted.
    device_write: bool,

    /// DMA-style transfer is permitted.
    transfer: bool,

    /// Remote/backend access is permitted.
    remote_access: bool,
}

impl AddressCapabilities {
    /// No capabilities.
    pub const NONE: Self = Self {
        cpu_read: false,
        cpu_write: false,
        device_read: false,
        device_write: false,
        transfer: false,
        remote_access: false,
    };

    /// Capabilities for ordinary mutable host memory.
    pub const HOST_READ_WRITE: Self = Self {
        cpu_read: true,
        cpu_write: true,
        device_read: false,
        device_write: false,
        transfer: true,
        remote_access: false,
    };

    /// Capabilities for read/write accelerator memory.
    pub const DEVICE_READ_WRITE: Self = Self {
        cpu_read: false,
        cpu_write: false,
        device_read: true,
        device_write: true,
        transfer: true,
        remote_access: false,
    };

    /// Capabilities for unified memory.
    pub const UNIFIED_READ_WRITE: Self = Self {
        cpu_read: true,
        cpu_write: true,
        device_read: true,
        device_write: true,
        transfer: true,
        remote_access: false,
    };

    /// Capabilities for an opaque remote backend.
    pub const REMOTE: Self = Self {
        cpu_read: false,
        cpu_write: false,
        device_read: false,
        device_write: false,
        transfer: false,
        remote_access: true,
    };

    /// Returns whether CPU reads are permitted.
    pub const fn cpu_read(self) -> bool {
        self.cpu_read
    }

    /// Returns whether CPU writes are permitted.
    pub const fn cpu_write(self) -> bool {
        self.cpu_write
    }

    /// Returns whether device reads are permitted.
    pub const fn device_read(self) -> bool {
        self.device_read
    }

    /// Returns whether device writes are permitted.
    pub const fn device_write(self) -> bool {
        self.device_write
    }

    /// Returns whether transfers are permitted.
    pub const fn transfer(self) -> bool {
        self.transfer
    }

    /// Returns whether remote access is permitted.
    pub const fn remote_access(self) -> bool {
        self.remote_access
    }

    /// Returns whether no operation is permitted.
    pub const fn is_empty(self) -> bool {
        !self.cpu_read
            && !self.cpu_write
            && !self.device_read
            && !self.device_write
            && !self.transfer
            && !self.remote_access
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn memory() -> MemoryId {
        MemoryId::new(1)
    }

    fn allocation() -> AllocationId {
        AllocationId::new(10)
    }

    fn backend() -> BackendMemoryId {
        BackendMemoryId::new(20)
    }

    #[test]
    fn local_address_preserves_identity() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 128);

        assert_eq!(address.space(), AddressSpace::Host);
        assert_eq!(address.memory(), memory());
        assert_eq!(address.allocation(), Some(allocation()));
        assert_eq!(address.backend_memory(), None);
        assert_eq!(address.offset(), 128);
        assert!(address.is_local());
        assert!(!address.is_backend_owned());
        assert!(address.validate().is_ok());
    }

    #[test]
    fn backend_address_preserves_identity() {
        let address =
            Address::backend(AddressSpace::RemoteQpu, memory(), backend(), 0);

        assert_eq!(address.space(), AddressSpace::RemoteQpu);
        assert_eq!(address.memory(), memory());
        assert_eq!(address.backend_memory(), Some(backend()));
        assert_eq!(address.allocation(), None);
        assert!(address.is_backend_owned());
        assert!(address.is_remote());
        assert!(address.validate().is_ok());
    }

    #[test]
    fn address_addition_is_checked() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 100);

        let next = address.checked_add(50).expect("valid addition");

        assert_eq!(next.offset(), 150);
        assert_eq!(next.memory(), address.memory());
        assert_eq!(next.allocation(), address.allocation());
    }

    #[test]
    fn address_addition_rejects_overflow() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), u64::MAX);

        assert!(matches!(
            address.checked_add(1),
            Err(AddressError::OffsetOverflow { .. })
        ));
    }

    #[test]
    fn address_subtraction_is_checked() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 100);

        let previous = address.checked_sub(40).expect("valid subtraction");

        assert_eq!(previous.offset(), 60);
    }

    #[test]
    fn address_subtraction_rejects_underflow() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 10);

        assert!(address.checked_sub(11).is_err());
    }

    #[test]
    fn address_range_is_half_open() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 10);

        let range = address
            .checked_range(20)
            .expect("range should be valid");

        assert_eq!(range.start(), 10);
        assert_eq!(range.end(), 30);
        assert_eq!(range.len(), 20);
        assert!(range.contains(10));
        assert!(range.contains(29));
        assert!(!range.contains(30));
    }

    #[test]
    fn range_overlap_works() {
        let a = AddressRange::new(10, 30).expect("valid");
        let b = AddressRange::new(20, 40).expect("valid");

        assert!(a.overlaps(b));

        let intersection = a.intersection(b).expect("intersection");

        assert_eq!(intersection.start(), 20);
        assert_eq!(intersection.end(), 30);
    }

    #[test]
    fn range_non_overlap_works() {
        let a = AddressRange::new(0, 10).expect("valid");
        let b = AddressRange::new(10, 20).expect("valid");

        assert!(!a.overlaps(b));
        assert!(a.intersection(b).is_none());
    }

    #[test]
    fn range_rejects_invalid_order() {
        assert!(matches!(
            AddressRange::new(20, 10),
            Err(AddressError::InvalidRange { .. })
        ));
    }

    #[test]
    fn range_validation_rejects_out_of_bounds() {
        let address =
            Address::local(AddressSpace::Host, memory(), allocation(), 80);

        let size = ByteCount::new(100);

        assert!(address.validate_range(20, size).is_ok());

        assert!(matches!(
            address.validate_range(21, size),
            Err(AddressError::OutOfBounds { .. })
        ));
    }

    #[test]
    fn alignment_accepts_power_of_two() {
        let alignment = Alignment::new(64).expect("valid alignment");

        assert!(alignment.is_aligned(0));
        assert!(alignment.is_aligned(64));
        assert!(alignment.is_aligned(128));
        assert!(!alignment.is_aligned(65));
    }

    #[test]
    fn alignment_rejects_zero() {
        assert!(matches!(
            Alignment::new(0),
            Err(AddressError::ZeroAlignment)
        ));
    }

    #[test]
    fn alignment_rejects_non_power_of_two() {
        assert!(matches!(
            Alignment::new(24),
            Err(AddressError::NonPowerOfTwoAlignment { .. })
        ));
    }

    #[test]
    fn alignment_rounds_up() {
        let alignment = Alignment::new(64).expect("valid");

        assert_eq!(alignment.align_up(0).expect("valid"), 0);
        assert_eq!(alignment.align_up(1).expect("valid"), 64);
        assert_eq!(alignment.align_up(63).expect("valid"), 64);
        assert_eq!(alignment.align_up(64).expect("valid"), 64);
        assert_eq!(alignment.align_up(65).expect("valid"), 128);
    }

    #[test]
    fn descriptor_validates_base_and_alignment() {
        let alignment = Alignment::new(64).expect("valid");

        let descriptor = AddressDescriptor::local(
            AddressSpace::Host,
            memory(),
            allocation(),
            ByteCount::new(1024),
            alignment,
        )
        .expect("descriptor");

        assert!(descriptor.validate().is_ok());

        let address = descriptor.address_at(128).expect("address");

        assert_eq!(address.offset(), 128);
    }

    #[test]
    fn descriptor_rejects_misaligned_offset() {
        let alignment = Alignment::new(64).expect("valid");

        let descriptor = AddressDescriptor::local(
            AddressSpace::Host,
            memory(),
            allocation(),
            ByteCount::new(1024),
            alignment,
        )
        .expect("descriptor");

        assert!(matches!(
            descriptor.address_at(65),
            Err(AddressError::MisalignedOffset { .. })
        ));
    }

    #[test]
    fn descriptor_rejects_out_of_bounds_address() {
        let alignment = Alignment::new(64).expect("valid");

        let descriptor = AddressDescriptor::local(
            AddressSpace::Host,
            memory(),
            allocation(),
            ByteCount::new(1024),
            alignment,
        )
        .expect("descriptor");

        assert!(descriptor.address_at(1025).is_err());
    }

    #[test]
    fn backend_address_requires_remote_space() {
        assert!(BackendAddress::new(
            AddressSpace::Host,
            memory(),
            backend(),
            0
        )
        .is_err());

        assert!(BackendAddress::new(
            AddressSpace::RemoteQpu,
            memory(),
            backend(),
            0
        )
        .is_ok());
    }

    #[test]
    fn backend_address_converts_to_generic_address() {
        let backend_address = BackendAddress::new(
            AddressSpace::RemoteQpu,
            memory(),
            backend(),
            128,
        )
        .expect("backend address");

        let address = backend_address.as_address();

        assert_eq!(address.space(), AddressSpace::RemoteQpu);
        assert_eq!(address.memory(), memory());
        assert_eq!(address.backend_memory(), Some(backend()));
        assert_eq!(address.offset(), 128);
        assert!(address.validate().is_ok());
    }

    #[test]
    fn distributed_address_preserves_partition() {
        let distributed =
            DistributedAddress::new(memory(), allocation(), 7, 1024);

        assert_eq!(distributed.memory(), memory());
        assert_eq!(distributed.allocation(), allocation());
        assert_eq!(distributed.partition(), 7);
        assert_eq!(distributed.offset(), 1024);
        assert_eq!(
            distributed.as_address().space(),
            AddressSpace::Distributed
        );
    }

    #[test]
    fn distributed_address_offset_is_checked() {
        let distributed =
            DistributedAddress::new(memory(), allocation(), 7, u64::MAX);

        assert!(distributed.checked_add(1).is_err());
    }

    #[test]
    fn address_identity_is_stable() {
        let a =
            Address::local(AddressSpace::Host, memory(), allocation(), 0);

        let b =
            Address::local(AddressSpace::Host, memory(), allocation(), 4096);

        assert_eq!(a.identity(), b.identity());
        assert!(a.same_resource(b));
    }

    #[test]
    fn different_allocations_are_not_same_resource() {
        let a =
            Address::local(AddressSpace::Host, memory(), allocation(), 0);

        let b = Address::local(
            AddressSpace::Host,
            memory(),
            AllocationId::new(11),
            0,
        );

        assert!(!a.same_resource(b));
    }

    #[test]
    fn local_and_backend_resources_are_not_same_resource() {
        let local =
            Address::local(AddressSpace::Host, memory(), allocation(), 0);

        let remote =
            Address::backend(AddressSpace::RemoteQpu, memory(), backend(), 0);

        assert!(!local.same_resource(remote));
    }

    #[test]
    fn checked_distance_is_signed() {
        let a =
            Address::local(AddressSpace::Host, memory(), allocation(), 100);

        let b =
            Address::local(AddressSpace::Host, memory(), allocation(), 40);

        assert_eq!(a.checked_distance(b).expect("distance"), 60);
        assert_eq!(b.checked_distance(a).expect("distance"), -60);
    }

    #[test]
    fn checked_distance_rejects_different_resources() {
        let a =
            Address::local(AddressSpace::Host, memory(), allocation(), 100);

        let b = Address::local(
            AddressSpace::Host,
            memory(),
            AllocationId::new(11),
            40,
        );

        assert!(a.checked_distance(b).is_err());
    }

    #[test]
    fn capabilities_have_expected_semantics() {
        assert!(AddressCapabilities::HOST_READ_WRITE.cpu_read());
        assert!(AddressCapabilities::HOST_READ_WRITE.cpu_write());
        assert!(!AddressCapabilities::HOST_READ_WRITE.device_read());

        assert!(AddressCapabilities::DEVICE_READ_WRITE.device_read());
        assert!(AddressCapabilities::DEVICE_READ_WRITE.device_write());
        assert!(!AddressCapabilities::DEVICE_READ_WRITE.cpu_read());

        assert!(AddressCapabilities::UNIFIED_READ_WRITE.cpu_read());
        assert!(AddressCapabilities::UNIFIED_READ_WRITE.device_read());

        assert!(AddressCapabilities::REMOTE.remote_access());
    }

    #[test]
    fn remote_spaces_are_classified_correctly() {
        assert!(AddressSpace::RemoteQpu.is_remote());
        assert!(AddressSpace::RemoteSimulator.is_remote());
        assert!(AddressSpace::BackendOpaque.is_remote());

        assert!(!AddressSpace::Host.is_remote());
        assert!(!AddressSpace::Device.is_remote());
    }

    #[test]
    fn local_validation_requires_allocation() {
        let address = Address {
            space: AddressSpace::Host,
            memory: memory(),
            allocation: None,
            backend: None,
            offset: 0,
        };

        assert!(matches!(
            address.validate(),
            Err(AddressError::MissingAllocationIdentity)
        ));
    }

    #[test]
    fn remote_validation_requires_backend_identity() {
        let address = Address {
            space: AddressSpace::RemoteQpu,
            memory: memory(),
            allocation: None,
            backend: None,
            offset: 0,
        };

        assert!(matches!(
            address.validate(),
            Err(AddressError::MissingBackendIdentity)
        ));
    }

    #[test]
    fn zero_memory_identity_is_rejected() {
        let address =
            Address::local(AddressSpace::Host, MemoryId::new(0), allocation(), 0);

        assert!(matches!(
            address.validate(),
            Err(AddressError::MissingResourceIdentity)
        ));
    }
}