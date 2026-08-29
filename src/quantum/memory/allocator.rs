//! Zamani Quantum Memory — Provider-Neutral Memory Allocator
//!
//! Production-grade memory allocation boundary for `quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - provider-neutral memory allocation;
//! - host-memory allocation;
//! - registration of device/GPU/accelerator providers;
//! - registration of distributed-memory providers;
//! - registration of backend-native memory providers;
//! - allocation identity;
//! - allocation accounting;
//! - resource-limit enforcement;
//! - temporary/persistent/state allocation classes;
//! - reservation-before-provider-allocation semantics;
//! - provider-independent allocation handles;
//! - deterministic allocation metadata;
//! - explicit release semantics;
//! - allocation statistics;
//! - provider capability discovery.
//!
//! It deliberately does NOT own:
//!
//! - quantum-state representations;
//! - state-vector mathematics;
//! - density matrices;
//! - stabilizer/tableau mathematics;
//! - tensor-network algorithms;
//! - GPU kernels;
//! - CUDA/HIP/Metal/Vulkan implementations;
//! - distributed communication protocols;
//! - backend authentication;
//! - QPU APIs;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - quantum IR semantics.
//!
//! Those responsibilities belong to their respective subsystems.
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
//!                     quantum::memory
//!                              │
//!                   MemoryAllocator
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!           Host          Device/GPU       Distributed
//!             │                │                │
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              │
//!                              ▼
//!                     Backend-native memory
//! ```
//!
//! The allocator is deliberately below state representations.
//!
//! ```text
//! allocator
//!     ▲
//!     │
//! state_vector
//! density_matrix
//! stabilizer
//! sparse
//! tensor_network
//! backend_state
//! ```
//!
//! Therefore this file must not import any of those modules.
//!
//! # Provider-neutral hardware rule
//!
//! The allocator does NOT assume CUDA, HIP, Metal, Vulkan, IBM, Google,
//! Quantinuum, IonQ, Rigetti, Pasqal, D-Wave, neutral-atom hardware,
//! photonic hardware, superconducting hardware, trapped-ion hardware, or any
//! other vendor or technology.
//!
//! Hardware-specific modules register an implementation of `MemoryProvider`.
//!
//! This means the same allocator contract can support:
//!
//! - CPU RAM;
//! - pinned host memory;
//! - NVIDIA CUDA memory;
//! - AMD/HIP memory;
//! - Apple Metal memory;
//! - Vulkan/SYCL/other accelerator memory;
//! - unified memory;
//! - NUMA-specific memory;
//! - MPI/UCX/RDMA-backed distributed memory;
//! - QPU/provider-native state handles;
//! - remote simulator memory;
//! - future accelerator technologies.
//!
//! The generic allocator never needs to be edited when a new provider is
//! introduced.
//!
//! # No unsafe
//!
//! This module contains no `unsafe` code.
//!
//! Provider implementations must also expose safe Rust APIs. A provider may
//! internally use an FFI implementation in another crate/module, but unsafe
//! implementation details must never leak into this public allocator API.
//!
//! # Resource-safety rule
//!
//! Allocation follows this order:
//!
//! ```text
//! validate request
//!       │
//!       ▼
//! build MemoryRequirement
//!       │
//!       ▼
//! check MemoryLimits
//!       │
//!       ▼
//! reserve accounting
//!       │
//!       ▼
//! provider.allocate()
//!       │
//!       ├── failure ──► rollback accounting
//!       │
//!       ▼
//! publish allocation
//! ```
//!
//! There is deliberately no:
//!
//! ```text
//! allocate first
//! check limits later
//! ```
//!
//! That would permit transient resource exhaustion and violate the memory
//! subsystem's safety contract.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features.
//!
//! # Integration contract
//!
//! Later memory modules must consume this file as follows:
//!
//! - `pool.rs` uses `MemoryAllocator` for backing allocations;
//! - `reservation.rs` uses `MemoryRequirement` and allocator reservations;
//! - `budget.rs` may impose stricter runtime budgets above this allocator;
//! - `state.rs` uses `AllocationClass::State`;
//! - `state_vector.rs` uses host/device/state allocation classes;
//! - `density_matrix.rs` uses state allocations and limits;
//! - `stabilizer.rs` uses ordinary host/device allocations;
//! - `sparse.rs` uses host/device allocations;
//! - `tensor_network.rs` uses state and temporary allocations;
//! - `backend_state.rs` uses `BackendNative` allocations;
//! - `gpu.rs` registers GPU/device providers;
//! - `distributed.rs` registers distributed providers;
//! - `migration.rs` allocates destination memory before migration;
//! - `compaction.rs` allocates replacement blocks through this boundary;
//! - `diagnostics.rs` consumes allocation statistics;
//! - `telemetry.rs` consumes allocation statistics;
//! - `snapshot.rs` and `checkpoint.rs` use persistent allocations where
//!   applicable.
//!
//! The allocator must therefore remain representation-neutral.
//!
//! # Important ownership rule
//!
//! `MemoryAllocation` owns the provider allocation. Dropping the allocation
//! releases the provider resource and updates allocator accounting.
//!
//! Explicit `release()` is available when deterministic early release is
//! required.
//!
//! # Important accounting rule
//!
//! Accounting is reservation-based rather than "whatever the OS happened to
//! allocate". This makes the allocator deterministic and allows resource
//! limits to be enforced before provider calls.
//!
//! # Important provider rule
//!
//! A provider is responsible for the actual storage represented by its
//! `ProviderAllocation` implementation.
//!
//! The core allocator does not inspect provider internals or raw addresses.
//!
//! # Schema stability
//!
//! The public types in this file are intended to become the stable allocation
//! boundary for the rest of `quantum::memory`.
//!
//! New provider technologies should implement `MemoryProvider`; they should
//! not require changes to `MemoryAllocator` itself.
+
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex, MutexGuard,
};

use super::errors::MemoryError;
use super::limits::{MemoryLimitViolation, MemoryLimits, MemoryRequirement};
use super::types::{AllocationId, ByteCount, MemoryId};

// =============================================================================
// Schema
// =============================================================================

/// Stable allocator schema identifier.
pub const MEMORY_ALLOCATOR_SCHEMA_ID: &str =
    "zamani.quantum.memory.allocator";

/// Semantic version of the allocator contract.
pub const MEMORY_ALLOCATOR_SCHEMA_VERSION: u16 = 1;

/// Initial allocation identity.
///
/// Zero is reserved as an invalid/unissued allocation identity.
pub const FIRST_ALLOCATION_ID: u64 = 1;

/// Initial memory-domain identity.
///
/// Zero is reserved as an invalid/unissued memory identity.
pub const FIRST_MEMORY_ID: u64 = 1;

// =============================================================================
// Storage location
// =============================================================================

/// Provider-neutral physical/logical location of an allocation.
///
/// This type deliberately describes *where* storage lives without exposing
/// vendor-specific handles or pointers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemoryLocation {
    /// Ordinary process-host memory.
    Host,

    /// Host memory intended for accelerator/device transfer.
    PinnedHost,

    /// Device/accelerator memory identified by a provider-local device ID.
    Device {
        /// Provider-local device identifier.
        device_id: u32,
    },

    /// Unified/shared host-device memory.
    Unified {
        /// Provider-local device identifier.
        device_id: u32,
    },

    /// Distributed memory owned by a logical distributed domain.
    Distributed {
        /// Logical distributed-memory domain identifier.
        domain_id: u32,
    },

    /// Memory whose representation is owned by a backend/QPU provider.
    ///
    /// The string is an opaque provider namespace, not a credential and not a
    /// network endpoint.
    BackendNative {
        /// Provider/backend namespace.
        provider: String,
    },
}

impl MemoryLocation {
    /// Returns the stable storage-location category.
    pub const fn kind(&self) -> MemoryLocationKind {
        match self {
            Self::Host => MemoryLocationKind::Host,
            Self::PinnedHost => MemoryLocationKind::PinnedHost,
            Self::Device { .. } => MemoryLocationKind::Device,
            Self::Unified { .. } => MemoryLocationKind::Unified,
            Self::Distributed { .. } => MemoryLocationKind::Distributed,
            Self::BackendNative { .. } => MemoryLocationKind::BackendNative,
        }
    }

    /// Returns whether this is host-resident memory.
    pub const fn is_host(&self) -> bool {
        matches!(self, Self::Host | Self::PinnedHost)
    }

    /// Returns whether this is accelerator/device memory.
    pub const fn is_device(&self) -> bool {
        matches!(self, Self::Device { .. } | Self::Unified { .. })
    }

    /// Returns whether this is distributed memory.
    pub const fn is_distributed(&self) -> bool {
        matches!(self, Self::Distributed { .. })
    }

    /// Returns whether this is backend-native memory.
    pub const fn is_backend_native(&self) -> bool {
        matches!(self, Self::BackendNative { .. })
    }
}

impl fmt::Display for MemoryLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Host => formatter.write_str("host"),
            Self::PinnedHost => formatter.write_str("pinned-host"),
            Self::Device { device_id } => {
                write!(formatter, "device:{device_id}")
            }
            Self::Unified { device_id } => {
                write!(formatter, "unified:{device_id}")
            }
            Self::Distributed { domain_id } => {
                write!(formatter, "distributed:{domain_id}")
            }
            Self::BackendNative { provider } => {
                write!(formatter, "backend-native:{provider}")
            }
        }
    }
}

/// Broad storage-location category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemoryLocationKind {
    /// Host RAM.
    Host,

    /// Pinned host RAM.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified host/device memory.
    Unified,

    /// Distributed memory.
    Distributed,

    /// Provider-owned backend memory.
    BackendNative,
}

impl MemoryLocationKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::BackendNative => "backend_native",
        }
    }
}

impl fmt::Display for MemoryLocationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Allocation class
// =============================================================================

/// Semantic lifetime/resource class of an allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AllocationClass {
    /// Short-lived scratch/temporary storage.
    Temporary,

    /// Storage expected to outlive one operation.
    Persistent,

    /// Storage containing quantum-state data.
    State,

    /// Storage used to preserve a checkpoint/snapshot.
    Checkpoint,

    /// Storage used for metadata/diagnostic data.
    Metadata,
}

impl AllocationClass {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Temporary => "temporary",
            Self::Persistent => "persistent",
            Self::State => "state",
            Self::Checkpoint => "checkpoint",
            Self::Metadata => "metadata",
        }
    }

    /// Returns whether this allocation should count as state memory.
    pub const fn is_state(self) -> bool {
        matches!(self, Self::State)
    }

    /// Returns whether this allocation is intended to be short-lived.
    pub const fn is_temporary(self) -> bool {
        matches!(self, Self::Temporary)
    }
}

impl fmt::Display for AllocationClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provider allocation
// =============================================================================

/// Opaque allocation owned by a `MemoryProvider`.
///
/// Implementations may contain ordinary host buffers, accelerator handles,
/// distributed-memory registrations, or backend-native resources.
///
/// No raw pointer is required by this interface.
pub trait ProviderAllocation: Send + Sync + 'static {
    /// Number of bytes represented by this allocation.
    fn byte_len(&self) -> u64;

    /// Storage location represented by the provider allocation.
    fn location(&self) -> MemoryLocation;

    /// Optional provider-defined stable resource label.
    ///
    /// The label must never contain credentials, raw pointers, API tokens, or
    /// other secrets.
    fn resource_label(&self) -> Option<&str> {
        None
    }
}

// =============================================================================
// Provider
// =============================================================================

/// Provider-neutral allocation implementation.
///
/// A provider owns the actual allocation mechanism.
///
/// Examples of implementations include:
///
/// - host allocator;
/// - CUDA provider;
/// - HIP provider;
/// - Metal provider;
/// - Vulkan/SYCL provider;
/// - RDMA/MPI/UCX provider;
/// - remote simulator provider;
/// - QPU-native provider.
///
/// Provider-specific modules implement this trait without changing
/// `MemoryAllocator`.
pub trait MemoryProvider: Send + Sync + 'static {
    /// Stable provider identifier.
    fn provider_id(&self) -> &str;

    /// Storage location implemented by this provider.
    fn location(&self) -> MemoryLocation;

    /// Returns whether this provider can satisfy the supplied byte count.
    ///
    /// This is advisory. `allocate()` remains authoritative.
    fn can_allocate(&self, bytes: u64) -> bool;

    /// Allocates provider-owned memory.
    ///
    /// The allocator has already performed policy/accounting checks before
    /// calling this method.
    fn allocate(&self, bytes: u64) -> Result<Box<dyn ProviderAllocation>, MemoryError>;

    /// Provider-level availability description.
    ///
    /// This is deliberately textual and must never expose secrets.
    fn availability(&self) -> ProviderAvailability {
        ProviderAvailability::Available
    }
}

/// Provider availability state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ProviderAvailability {
    /// Provider can accept allocations.
    Available,

    /// Provider exists but is currently resource-constrained.
    Constrained,

    /// Provider exists but is unavailable.
    Unavailable,
}

impl ProviderAvailability {
    /// Returns true if allocation may be attempted.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available | Self::Constrained)
    }
}

// =============================================================================
// Host provider
// =============================================================================

/// Safe standard-library host-memory provider.
///
/// This is the only built-in concrete provider in this module.
///
/// Accelerator and QPU implementations belong in their owning modules.
#[derive(Debug, Default)]
pub struct HostMemoryProvider;

impl HostMemoryProvider {
    /// Creates a host-memory provider.
    pub const fn new() -> Self {
        Self
    }
}

/// Provider-owned host allocation.
///
/// `Vec<u8>` is intentionally kept private. No raw pointer is exposed.
pub struct HostAllocation {
    bytes: Box<[u8]>,
}

impl fmt::Debug for HostAllocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostAllocation")
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

impl ProviderAllocation for HostAllocation {
    fn byte_len(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn location(&self) -> MemoryLocation {
        MemoryLocation::Host
    }

    fn resource_label(&self) -> Option<&str> {
        Some("host")
    }
}

impl MemoryProvider for HostMemoryProvider {
    fn provider_id(&self) -> &str {
        "zamani.host"
    }

    fn location(&self) -> MemoryLocation {
        MemoryLocation::Host
    }

    fn can_allocate(&self, bytes: u64) -> bool {
        usize::try_from(bytes)
            .map(|value| value <= isize::MAX as usize)
            .unwrap_or(false)
    }

    fn allocate(
        &self,
        bytes: u64,
    ) -> Result<Box<dyn ProviderAllocation>, MemoryError> {
        let length = usize::try_from(bytes).map_err(|_| {
            MemoryError::AllocationFailed {
                requested_bytes: bytes,
                available_bytes: 0,
            }
        })?;

        // Rust's Vec allocation is safe. No raw pointer is exposed.
        let bytes = vec![0u8; length].into_boxed_slice();

        Ok(Box::new(HostAllocation { bytes }))
    }
}

// =============================================================================
// Allocation request
// =============================================================================

/// Complete allocation request.
///
/// This is the object that crosses the planner → allocator boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationRequest {
    /// Requested byte count.
    pub bytes: ByteCount,

    /// Storage location.
    pub location: MemoryLocation,

    /// Semantic allocation class.
    pub class: AllocationClass,

    /// Optional state-element count.
    ///
    /// Used by state representations for additional policy enforcement.
    pub state_elements: u64,

    /// Optional logical-qubit count.
    ///
    /// Used by quantum-state allocations and diagnostics.
    pub qubits: u64,

    /// Human-readable safe label.
    ///
    /// This must not contain secrets.
    pub label: Option<String>,
}

impl AllocationRequest {
    /// Creates an ordinary allocation request.
    pub fn new(
        bytes: ByteCount,
        location: MemoryLocation,
        class: AllocationClass,
    ) -> Self {
        Self {
            bytes,
            location,
            class,
            state_elements: 0,
            qubits: 0,
            label: None,
        }
    }

    /// Adds a state-element requirement.
    #[must_use]
    pub const fn with_state_elements(mut self, elements: u64) -> Self {
        self.state_elements = elements;
        self
    }

    /// Adds a qubit requirement.
    #[must_use]
    pub const fn with_qubits(mut self, qubits: u64) -> Self {
        self.qubits = qubits;
        self
    }

    /// Adds a safe diagnostic label.
    ///
    /// The allocator does not interpret this string as a provider identifier.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the requested bytes.
    pub const fn byte_count(&self) -> u64 {
        self.bytes.get()
    }
}

// =============================================================================
// Accounting
// =============================================================================

/// Runtime allocation counters.
///
/// This structure contains only accounting state. Hard policy remains owned by
/// `MemoryLimits`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AllocationAccounting {
    /// Number of live allocations.
    pub allocations: u64,

    /// Host bytes currently reserved.
    pub host_bytes: u64,

    /// Temporary host bytes.
    pub temporary_host_bytes: u64,

    /// Persistent host bytes.
    pub persistent_host_bytes: u64,

    /// Pinned host bytes.
    pub pinned_host_bytes: u64,

    /// Device bytes.
    pub device_bytes: u64,

    /// Temporary device bytes.
    pub temporary_device_bytes: u64,

    /// Distributed bytes.
    pub distributed_bytes: u64,

    /// State bytes.
    pub state_bytes: u64,

    /// Temporary state bytes.
    pub temporary_state_bytes: u64,

    /// State elements.
    pub state_elements: u64,

    /// Peak host bytes.
    pub peak_host_bytes: u64,

    /// Peak device bytes.
    pub peak_device_bytes: u64,

    /// Peak distributed bytes.
    pub peak_distributed_bytes: u64,

    /// Peak total bytes.
    pub peak_total_bytes: u64,
}

impl AllocationAccounting {
    /// Current total bytes across all tracked storage domains.
    pub const fn total_bytes(&self) -> u64 {
        self.host_bytes
            .saturating_add(self.device_bytes)
            .saturating_add(self.distributed_bytes)
    }

    fn reserve(
        &mut self,
        request: &AllocationRequest,
    ) -> Result<(), MemoryError> {
        let bytes = request.byte_count();

        self.allocations = self
            .allocations
            .checked_add(1)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "allocation-count increment".to_owned(),
            })?;

        match request.location {
            MemoryLocation::Host => {
                self.host_bytes = self.host_bytes.checked_add(bytes).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "host-byte accounting".to_owned(),
                    }
                })?;

                match request.class {
                    AllocationClass::Temporary => {
                        self.temporary_host_bytes = self
                            .temporary_host_bytes
                            .checked_add(bytes)
                            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                                operation: "temporary-host-byte accounting"
                                    .to_owned(),
                            })?;
                    }
                    AllocationClass::Persistent
                    | AllocationClass::State
                    | AllocationClass::Checkpoint
                    | AllocationClass::Metadata => {
                        self.persistent_host_bytes = self
                            .persistent_host_bytes
                            .checked_add(bytes)
                            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                                operation: "persistent-host-byte accounting"
                                    .to_owned(),
                            })?;
                    }
                }
            }

            MemoryLocation::PinnedHost => {
                self.host_bytes = self.host_bytes.checked_add(bytes).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "pinned-host byte accounting".to_owned(),
                    }
                })?;

                self.pinned_host_bytes = self
                    .pinned_host_bytes
                    .checked_add(bytes)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "pinned-host byte accounting".to_owned(),
                    })?;
            }

            MemoryLocation::Device { .. }
            | MemoryLocation::Unified { .. } => {
                self.device_bytes = self.device_bytes.checked_add(bytes).ok_or_else(|| {
                    MemoryError::ArithmeticOverflow {
                        operation: "device-byte accounting".to_owned(),
                    }
                })?;

                if request.class.is_temporary() {
                    self.temporary_device_bytes = self
                        .temporary_device_bytes
                        .checked_add(bytes)
                        .ok_or_else(|| MemoryError::ArithmeticOverflow {
                            operation: "temporary-device-byte accounting".to_owned(),
                        })?;
                }
            }

            MemoryLocation::Distributed { .. } => {
                self.distributed_bytes = self
                    .distributed_bytes
                    .checked_add(bytes)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "distributed-byte accounting".to_owned(),
                    })?;
            }

            MemoryLocation::BackendNative { .. } => {
                // Backend-native resources are provider-owned. Their provider
                // decides the physical storage domain. The allocator still
                // counts the logical bytes against allocation count and state
                // accounting when applicable.
            }
        }

        if request.class.is_state() {
            self.state_bytes = self
                .state_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "state-byte accounting".to_owned(),
                })?;

            self.state_elements = self
                .state_elements
                .checked_add(request.state_elements)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "state-element accounting".to_owned(),
                })?;
        }

        self.update_peaks();

        Ok(())
    }

    fn release(
        &mut self,
        request: &AllocationRequest,
    ) -> Result<(), MemoryError> {
        let bytes = request.byte_count();

        self.allocations = self
            .allocations
            .checked_sub(1)
            .ok_or_else(|| MemoryError::InvariantViolation {
                reason: "allocation accounting underflow".to_owned(),
            })?;

        match request.location {
            MemoryLocation::Host => {
                self.host_bytes = self.host_bytes.checked_sub(bytes).ok_or_else(|| {
                    MemoryError::InvariantViolation {
                        reason: "host-byte accounting underflow".to_owned(),
                    }
                })?;

                match request.class {
                    AllocationClass::Temporary => {
                        self.temporary_host_bytes = self
                            .temporary_host_bytes
                            .checked_sub(bytes)
                            .ok_or_else(|| MemoryError::InvariantViolation {
                                reason: "temporary-host accounting underflow"
                                    .to_owned(),
                            })?;
                    }
                    AllocationClass::Persistent
                    | AllocationClass::State
                    | AllocationClass::Checkpoint
                    | AllocationClass::Metadata => {
                        self.persistent_host_bytes = self
                            .persistent_host_bytes
                            .checked_sub(bytes)
                            .ok_or_else(|| MemoryError::InvariantViolation {
                                reason: "persistent-host accounting underflow"
                                    .to_owned(),
                            })?;
                    }
                }
            }

            MemoryLocation::PinnedHost => {
                self.host_bytes = self.host_bytes.checked_sub(bytes).ok_or_else(|| {
                    MemoryError::InvariantViolation {
                        reason: "pinned-host host-byte accounting underflow"
                            .to_owned(),
                    }
                })?;

                self.pinned_host_bytes = self
                    .pinned_host_bytes
                    .checked_sub(bytes)
                    .ok_or_else(|| MemoryError::InvariantViolation {
                        reason: "pinned-host accounting underflow".to_owned(),
                    })?;
            }

            MemoryLocation::Device { .. }
            | MemoryLocation::Unified { .. } => {
                self.device_bytes = self.device_bytes.checked_sub(bytes).ok_or_else(|| {
                    MemoryError::InvariantViolation {
                        reason: "device-byte accounting underflow".to_owned(),
                    }
                })?;

                if request.class.is_temporary() {
                    self.temporary_device_bytes = self
                        .temporary_device_bytes
                        .checked_sub(bytes)
                        .ok_or_else(|| MemoryError::InvariantViolation {
                            reason: "temporary-device accounting underflow"
                                .to_owned(),
                        })?;
                }
            }

            MemoryLocation::Distributed { .. } => {
                self.distributed_bytes = self
                    .distributed_bytes
                    .checked_sub(bytes)
                    .ok_or_else(|| MemoryError::InvariantViolation {
                        reason: "distributed-byte accounting underflow".to_owned(),
                    })?;
            }

            MemoryLocation::BackendNative { .. } => {}
        }

        if request.class.is_state() {
            self.state_bytes = self.state_bytes.checked_sub(bytes).ok_or_else(|| {
                MemoryError::InvariantViolation {
                    reason: "state-byte accounting underflow".to_owned(),
                }
            })?;

            self.state_elements = self
                .state_elements
                .checked_sub(request.state_elements)
                .ok_or_else(|| MemoryError::InvariantViolation {
                    reason: "state-element accounting underflow".to_owned(),
                })?;
        }

        Ok(())
    }

    fn update_peaks(&mut self) {
        let total = self.total_bytes();

        if self.host_bytes > self.peak_host_bytes {
            self.peak_host_bytes = self.host_bytes;
        }

        if self.device_bytes > self.peak_device_bytes {
            self.peak_device_bytes = self.device_bytes;
        }

        if self.distributed_bytes > self.peak_distributed_bytes {
            self.peak_distributed_bytes = self.distributed_bytes;
        }

        if total > self.peak_total_bytes {
            self.peak_total_bytes = total;
        }
    }
}

// =============================================================================
// Allocation record
// =============================================================================

struct AllocationRecord {
    id: AllocationId,
    memory_id: MemoryId,
    request: AllocationRequest,
    provider_id: String,
    provider_allocation: Option<Box<dyn ProviderAllocation>>,
    accounting: Arc<AllocatorInner>,
}

impl fmt::Debug for AllocationRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AllocationRecord")
            .field("id", &self.id)
            .field("memory_id", &self.memory_id)
            .field("request", &self.request)
            .field("provider_id", &self.provider_id)
            .finish()
    }
}

impl Drop for AllocationRecord {
    fn drop(&mut self) {
        let _provider_allocation = self.provider_allocation.take();

        if let Ok(mut state) = self.accounting.state.lock() {
            // Drop must not panic. If accounting is already inconsistent,
            // preserve process safety rather than attempting recovery through
            // another panic.
            let _ = state.release(&self.request);
        }
    }
}

// =============================================================================
// Public allocation handle
// =============================================================================

/// Owned memory allocation handle.
///
/// Dropping this handle releases the provider allocation and updates allocator
/// accounting.
///
/// Cloning is intentionally not implemented. Quantum memory ownership should
/// remain explicit.
pub struct MemoryAllocation {
    record: Option<AllocationRecord>,
}

impl fmt::Debug for MemoryAllocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryAllocation")
            .field("id", &self.id())
            .field("memory_id", &self.memory_id())
            .field("request", &self.request())
            .field("provider_id", &self.provider_id())
            .finish()
    }
}

impl MemoryAllocation {
    /// Returns the allocation identity.
    pub fn id(&self) -> AllocationId {
        self.record
            .as_ref()
            .map(|record| record.id)
            .unwrap_or_else(|| AllocationId::new(0))
    }

    /// Returns the memory-domain identity.
    pub fn memory_id(&self) -> MemoryId {
        self.record
            .as_ref()
            .map(|record| record.memory_id)
            .unwrap_or_else(|| MemoryId::new(0))
    }

    /// Returns the allocation request.
    pub fn request(&self) -> &AllocationRequest {
        match self.record.as_ref() {
            Some(record) => &record.request,
            None => {
                // This branch cannot safely return a reference to a temporary.
                // A released allocation therefore uses a dedicated panic-free
                // API below instead of exposing a fabricated request.
                //
                // The branch is unreachable for valid ownership, but Rust
                // requires a reference. The allocator never exposes a released
                // handle through normal construction.
                std::process::abort()
            }
        }
    }

    /// Returns the provider identifier.
    pub fn provider_id(&self) -> &str {
        match self.record.as_ref() {
            Some(record) => &record.provider_id,
            None => "released",
        }
    }

    /// Returns the provider allocation.
    ///
    /// The provider allocation remains opaque and cannot expose raw addresses
    /// through this API.
    pub fn provider_allocation(&self) -> Option<&dyn ProviderAllocation> {
        self.record
            .as_ref()
            .and_then(|record| record.provider_allocation.as_deref())
    }

    /// Returns the requested byte count.
    pub fn byte_len(&self) -> u64 {
        self.record
            .as_ref()
            .map(|record| record.request.byte_count())
            .unwrap_or(0)
    }

    /// Returns whether the handle still owns an allocation.
    pub fn is_live(&self) -> bool {
        self.record.is_some()
    }

    /// Explicitly releases the allocation.
    ///
    /// The provider allocation is dropped immediately.
    pub fn release(mut self) {
        self.record.take();
    }
}

// =============================================================================
// Allocator state
// =============================================================================

struct AllocatorInner {
    memory_id: MemoryId,
    limits: MemoryLimits,
    state: Mutex<AllocationAccounting>,
}

// =============================================================================
// Allocator
// =============================================================================

/// Thread-safe provider-neutral quantum memory allocator.
///
/// `MemoryAllocator` is cheap to clone. Clones share the same memory domain,
/// limits, provider registry and accounting state.
#[derive(Clone)]
pub struct MemoryAllocator {
    inner: Arc<AllocatorInner>,
    next_allocation_id: Arc<AtomicU64>,
    providers: Arc<Mutex<BTreeMap<MemoryLocation, Arc<dyn MemoryProvider>>>>,
}

impl fmt::Debug for MemoryAllocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let provider_count = self
            .providers
            .lock()
            .map(|providers| providers.len())
            .unwrap_or(0);

        formatter
            .debug_struct("MemoryAllocator")
            .field("memory_id", &self.inner.memory_id)
            .field("limits", &self.inner.limits)
            .field("provider_count", &provider_count)
            .finish()
    }
}

impl MemoryAllocator {
    /// Creates an allocator with the supplied memory limits and no providers.
    ///
    /// The host provider can be added with `with_host_provider()`.
    pub fn new(
        memory_id: MemoryId,
        limits: MemoryLimits,
    ) -> Result<Self, MemoryError> {
        limits.validate().map_err(|error| MemoryError::InvalidArgument {
            reason: error.to_string(),
        })?;

        if memory_id.get() == 0 {
            return Err(MemoryError::InvalidIdentifier {
                kind: "memory-id".to_owned(),
                identifier: "0".to_owned(),
                context: None,
            });
        }

        Ok(Self {
            inner: Arc::new(AllocatorInner {
                memory_id,
                limits,
                state: Mutex::new(AllocationAccounting::default()),
            }),
            next_allocation_id: Arc::new(AtomicU64::new(FIRST_ALLOCATION_ID)),
            providers: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Creates an allocator using the production memory policy.
    pub fn production(memory_id: MemoryId) -> Result<Self, MemoryError> {
        Self::new(memory_id, MemoryLimits::production())
    }

    /// Returns this allocator's memory-domain identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.inner.memory_id
    }

    /// Returns the immutable memory policy.
    pub const fn limits(&self) -> MemoryLimits {
        self.inner.limits
    }

    /// Registers or replaces a provider for a storage location.
    ///
    /// Replacement is permitted only when no live allocation currently uses
    /// that location. This prevents a provider implementation from being
    /// replaced underneath a live allocation.
    pub fn register_provider(
        &self,
        provider: Arc<dyn MemoryProvider>,
    ) -> Result<(), MemoryError> {
        let location = provider.location();

        if provider.provider_id().is_empty() {
            return Err(MemoryError::InvalidArgument {
                reason: "memory provider identifier cannot be empty".to_owned(),
            });
        }

        if let MemoryLocation::BackendNative { provider: name } = &location {
            if name.is_empty() {
                return Err(MemoryError::InvalidArgument {
                    reason: "backend-native provider namespace cannot be empty"
                        .to_owned(),
                });
            }
        }

        let mut providers = lock_state(&self.providers)?;

        if providers.contains_key(&location) {
            let state = lock_state(&self.inner.state)?;

            if state.allocations > 0 {
                // A location-specific replacement requires proving there is no
                // live allocation on that location. The current accounting
                // structure intentionally does not keep per-location counts,
                // so replacement is conservatively denied while any
                // allocation exists.
                return Err(MemoryError::ConcurrencyConflict {
                    reason: format!(
                        "cannot replace memory provider `{location}` while \
                         allocations are live"
                    ),
                });
            }
        }

        providers.insert(location, provider);

        Ok(())
    }

    /// Registers the standard host provider.
    pub fn with_host_provider(self) -> Result<Self, MemoryError> {
        self.register_provider(Arc::new(HostMemoryProvider::new()))?;
        Ok(self)
    }

    /// Returns whether a provider exists for the requested location.
    pub fn has_provider(
        &self,
        location: &MemoryLocation,
    ) -> Result<bool, MemoryError> {
        let providers = lock_state(&self.providers)?;
        Ok(providers.contains_key(location))
    }

    /// Returns the number of registered providers.
    pub fn provider_count(&self) -> Result<usize, MemoryError> {
        let providers = lock_state(&self.providers)?;
        Ok(providers.len())
    }

    /// Allocates memory after checking all applicable resource limits.
    pub fn allocate(
        &self,
        request: AllocationRequest,
    ) -> Result<MemoryAllocation, MemoryError> {
        validate_request(&request)?;

        let provider = {
            let providers = lock_state(&self.providers)?;

            match providers.get(&request.location) {
                Some(provider) => Arc::clone(provider),
                None => {
                    return Err(MemoryError::UnsupportedStorageLocation {
                        location: request.location.to_string(),
                    });
                }
            }
        };

        if !provider.can_allocate(request.byte_count()) {
            return Err(MemoryError::AllocationFailed {
                requested_bytes: request.byte_count(),
                available_bytes: 0,
            });
        }

        let mut state = lock_state(&self.inner.state)?;

        let requirement = build_requirement(&state, &request)?;

        self.inner
            .limits
            .check_requirement(requirement)
            .map_err(limit_violation_to_error)?;

        state.reserve(&request)?;
        drop(state);

        let provider_allocation =
            match provider.allocate(request.byte_count()) {
                Ok(allocation) => allocation,
                Err(error) => {
                    let mut rollback = lock_state(&self.inner.state)?;

                    let rollback_result = rollback.release(&request);

                    if let Err(rollback_error) = rollback_result {
                        return Err(MemoryError::InvariantViolation {
                            reason: format!(
                                "provider allocation failed with `{error}`, \
                                 and accounting rollback failed with \
                                 `{rollback_error}`"
                            ),
                        });
                    }

                    return Err(error);
                }
            };

        if provider_allocation.byte_len() != request.byte_count() {
            let actual = provider_allocation.byte_len();

            drop(provider_allocation);

            let mut rollback = lock_state(&self.inner.state)?;
            let rollback_result = rollback.release(&request);

            if let Err(rollback_error) = rollback_result {
                return Err(MemoryError::InvariantViolation {
                    reason: format!(
                        "provider returned {actual} bytes for a request of \
                         {} bytes and accounting rollback failed: \
                         {rollback_error}",
                        request.byte_count()
                    ),
                });
            }

            return Err(MemoryError::AllocationFailed {
                requested_bytes: request.byte_count(),
                available_bytes: actual,
            });
        }

        if provider_allocation.location() != request.location {
            let actual_location = provider_allocation.location();

            drop(provider_allocation);

            let mut rollback = lock_state(&self.inner.state)?;
            let rollback_result = rollback.release(&request);

            if let Err(rollback_error) = rollback_result {
                return Err(MemoryError::InvariantViolation {
                    reason: format!(
                        "provider returned location `{actual_location}` for \
                         request `{}` and accounting rollback failed: \
                         {rollback_error}",
                        request.location
                    ),
                });
            }

            return Err(MemoryError::BackendRejected {
                reason: format!(
                    "provider `{}` returned storage location `{actual_location}` \
                     instead of requested `{}`",
                    provider.provider_id(),
                    request.location
                ),
            });
        }

        let raw_id = self.next_allocation_id.fetch_add(1, Ordering::Relaxed);

        if raw_id == 0 {
            let mut rollback = lock_state(&self.inner.state)?;
            let _ = rollback.release(&request);

            drop(provider_allocation);

            return Err(MemoryError::InvariantViolation {
                reason: "allocation identity counter wrapped to zero".to_owned(),
            });
        }

        let allocation_id = AllocationId::new(raw_id);

        Ok(MemoryAllocation {
            record: Some(AllocationRecord {
                id: allocation_id,
                memory_id: self.inner.memory_id,
                request,
                provider_id: provider.provider_id().to_owned(),
                provider_allocation: Some(provider_allocation),
                accounting: Arc::clone(&self.inner),
            }),
        })
    }

    /// Allocates a raw byte buffer in host memory.
    ///
    /// This convenience method is intentionally limited to host memory.
    /// Device/QPU allocations must use an explicitly registered provider.
    pub fn allocate_host(
        &self,
        bytes: ByteCount,
        class: AllocationClass,
    ) -> Result<MemoryAllocation, MemoryError> {
        self.allocate(AllocationRequest::new(
            bytes,
            MemoryLocation::Host,
            class,
        ))
    }

    /// Allocates quantum-state memory.
    pub fn allocate_state(
        &self,
        bytes: ByteCount,
        location: MemoryLocation,
        qubits: u64,
        state_elements: u64,
    ) -> Result<MemoryAllocation, MemoryError> {
        self.allocate(
            AllocationRequest::new(bytes, location, AllocationClass::State)
                .with_qubits(qubits)
                .with_state_elements(state_elements),
        )
    }

    /// Returns a snapshot of current allocation accounting.
    pub fn accounting(&self) -> Result<AllocationAccounting, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(*state)
    }

    /// Returns the current number of live allocations.
    pub fn allocation_count(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(state.allocations)
    }

    /// Returns current total tracked bytes.
    pub fn allocated_bytes(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(state.total_bytes())
    }

    /// Returns the peak total tracked bytes.
    pub fn peak_allocated_bytes(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(state.peak_total_bytes)
    }

    /// Returns the remaining host bytes under the configured hard host limit.
    pub fn remaining_host_bytes(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(self
            .inner
            .limits
            .max_host_bytes()
            .saturating_sub(state.host_bytes))
    }

    /// Returns the remaining device bytes under the configured device limit.
    pub fn remaining_device_bytes(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(self
            .inner
            .limits
            .max_device_bytes()
            .saturating_sub(state.device_bytes))
    }

    /// Returns the remaining distributed bytes under the configured
    /// distributed-memory limit.
    pub fn remaining_distributed_bytes(&self) -> Result<u64, MemoryError> {
        let state = lock_state(&self.inner.state)?;
        Ok(self
            .inner
            .limits
            .max_distributed_bytes()
            .saturating_sub(state.distributed_bytes))
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn validate_request(request: &AllocationRequest) -> Result<(), MemoryError> {
    if request.bytes.get() == 0 {
        return Err(MemoryError::InvalidArgument {
            reason: "zero-byte allocations are not permitted".to_owned(),
        });
    }

    if request.location.is_backend_native() {
        if let MemoryLocation::BackendNative { provider } = &request.location {
            if provider.is_empty() {
                return Err(MemoryError::InvalidArgument {
                    reason: "backend-native provider namespace cannot be empty"
                        .to_owned(),
                });
            }
        }
    }

    if request.class.is_state()
        && request.state_elements == 0
        && request.qubits != 0
    {
        return Err(MemoryError::InvalidArgument {
            reason: "state allocations with a non-zero qubit count must also \
                     declare state_elements"
                .to_owned(),
        });
    }

    Ok(())
}

fn build_requirement(
    accounting: &AllocationAccounting,
    request: &AllocationRequest,
) -> Result<MemoryRequirement, MemoryError> {
    let bytes = request.byte_count();

    let next_allocations = accounting
        .allocations
        .checked_add(1)
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "allocation-count requirement".to_owned(),
        })?;

    let next_state_elements = accounting
        .state_elements
        .checked_add(request.state_elements)
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "state-element requirement".to_owned(),
        })?;

    let next_state_bytes = accounting
        .state_bytes
        .checked_add(if request.class.is_state() {
            bytes
        } else {
            0
        })
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "state-byte requirement".to_owned(),
        })?;

    let mut requirement = MemoryRequirement::empty()
        .with_allocations(next_allocations)
        .with_qubits(request.qubits)
        .with_state_bytes(next_state_bytes)
        .with_state_elements(next_state_elements);

    match request.location {
        MemoryLocation::Host => {
            let host = accounting
                .host_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "host-byte requirement".to_owned(),
                })?;

            requirement = requirement.with_host_bytes(host);

            match request.class {
                AllocationClass::Temporary => {
                    let temporary = accounting
                        .temporary_host_bytes
                        .checked_add(bytes)
                        .ok_or_else(|| MemoryError::ArithmeticOverflow {
                            operation: "temporary-host requirement".to_owned(),
                        })?;

                    requirement =
                        requirement.with_temporary_host_bytes(temporary);
                }
                _ => {
                    let persistent = accounting
                        .persistent_host_bytes
                        .checked_add(bytes)
                        .ok_or_else(|| MemoryError::ArithmeticOverflow {
                            operation: "persistent-host requirement".to_owned(),
                        })?;

                    requirement =
                        requirement.with_persistent_host_bytes(persistent);
                }
            }
        }

        MemoryLocation::PinnedHost => {
            let host = accounting
                .host_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "pinned-host requirement".to_owned(),
                })?;

            let pinned = accounting
                .pinned_host_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "pinned-host requirement".to_owned(),
                })?;

            requirement = requirement
                .with_host_bytes(host)
                .with_pinned_host_bytes(pinned);
        }

        MemoryLocation::Device { .. }
        | MemoryLocation::Unified { .. } => {
            let device = accounting
                .device_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "device-byte requirement".to_owned(),
                })?;

            requirement = requirement.with_device_bytes(device);

            if request.class.is_temporary() {
                let temporary = accounting
                    .temporary_device_bytes
                    .checked_add(bytes)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation: "temporary-device requirement".to_owned(),
                    })?;

                requirement =
                    requirement.with_temporary_device_bytes(temporary);
            }
        }

        MemoryLocation::Distributed { .. } => {
            let distributed = accounting
                .distributed_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "distributed-byte requirement".to_owned(),
                })?;

            requirement = requirement.with_distributed_bytes(distributed);
        }

        MemoryLocation::BackendNative { .. } => {
            // Provider-owned memory has no universal physical memory domain.
            // The provider's own availability/capacity policy is authoritative.
            //
            // Allocation count and state limits are still enforced.
        }
    }

    if request.class.is_state() {
        let temporary_state = if request.class.is_temporary() {
            accounting
                .temporary_state_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "temporary-state requirement".to_owned(),
                })?
        } else {
            accounting.temporary_state_bytes
        };

        requirement = requirement.with_temporary_state_bytes(temporary_state);
    }

    Ok(requirement)
}

fn limit_violation_to_error(
    violation: MemoryLimitViolation,
) -> MemoryError {
    MemoryError::MemoryLimitExceeded {
        limit: violation.kind().to_string(),
        requested_bytes: violation.requested(),
        maximum_bytes: violation.maximum(),
    }
}

fn lock_state<T>(
    mutex: &Mutex<T>,
) -> Result<MutexGuard<'_, T>, MemoryError> {
    mutex.lock().map_err(|_| MemoryError::ConcurrencyConflict {
        reason: "memory allocator state lock is poisoned".to_owned(),
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::memory::limits::MemoryLimits;

    fn allocator() -> MemoryAllocator {
        MemoryAllocator::production(MemoryId::new(FIRST_MEMORY_ID))
            .expect("production limits must be valid")
            .with_host_provider()
            .expect("host provider registration must succeed")
    }

    #[test]
    fn host_provider_allocates_without_unsafe() {
        let allocator = allocator();

        let allocation = allocator
            .allocate_host(ByteCount::new(4096), AllocationClass::Temporary)
            .expect("host allocation should succeed");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 4096);
        assert_eq!(allocation.provider_id(), "zamani.host");
        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            4096
        );
    }

    #[test]
    fn dropping_allocation_releases_accounting() {
        let allocator = allocator();

        {
            let _allocation = allocator
                .allocate_host(ByteCount::new(1024), AllocationClass::Temporary)
                .expect("allocation should succeed");

            assert_eq!(
                allocator.allocated_bytes().expect("accounting"),
                1024
            );
        }

        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            0
        );
        assert_eq!(
            allocator.allocation_count().expect("accounting"),
            0
        );
    }

    #[test]
    fn explicit_release_is_supported() {
        let allocator = allocator();

        let allocation = allocator
            .allocate_host(ByteCount::new(1024), AllocationClass::Persistent)
            .expect("allocation should succeed");

        allocation.release();

        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            0
        );
    }

    #[test]
    fn zero_byte_allocations_are_rejected() {
        let allocator = allocator();

        let result = allocator.allocate_host(
            ByteCount::ZERO,
            AllocationClass::Temporary,
        );

        assert!(matches!(
            result,
            Err(MemoryError::InvalidArgument { .. })
        ));
    }

    #[test]
    fn unregistered_location_is_rejected() {
        let allocator = allocator();

        let result = allocator.allocate(AllocationRequest::new(
            ByteCount::new(1024),
            MemoryLocation::Device { device_id: 0 },
            AllocationClass::Temporary,
        ));

        assert!(matches!(
            result,
            Err(MemoryError::UnsupportedStorageLocation { .. })
        ));
    }

    #[test]
    fn allocation_limit_is_checked_before_provider_allocation() {
        let limits = MemoryLimits::deny_all();

        let allocator =
            MemoryAllocator::new(MemoryId::new(FIRST_MEMORY_ID), limits)
                .expect("deny-all limits must be valid")
                .with_host_provider()
                .expect("provider registration");

        let result = allocator.allocate_host(
            ByteCount::new(1),
            AllocationClass::Temporary,
        );

        assert!(matches!(
            result,
            Err(MemoryError::MemoryLimitExceeded { .. })
        ));

        assert_eq!(
            allocator.allocated_bytes().expect("accounting"),
            0
        );
    }

    #[test]
    fn state_allocation_tracks_state_bytes_and_elements() {
        let allocator = allocator();

        let allocation = allocator
            .allocate_state(
                ByteCount::new(16 * 1024),
                MemoryLocation::Host,
                10,
                1024,
            )
            .expect("state allocation should succeed");

        let accounting = allocator.accounting().expect("accounting");

        assert_eq!(accounting.state_bytes, 16 * 1024);
        assert_eq!(accounting.state_elements, 1024);

        drop(allocation);

        let accounting = allocator.accounting().expect("accounting");

        assert_eq!(accounting.state_bytes, 0);
        assert_eq!(accounting.state_elements, 0);
    }

    #[test]
    fn allocation_identity_is_non_zero() {
        let allocator = allocator();

        let allocation = allocator
            .allocate_host(ByteCount::new(1), AllocationClass::Temporary)
            .expect("allocation should succeed");

        assert_ne!(allocation.id().get(), 0);
    }

    #[test]
    fn provider_replacement_is_safe() {
        let allocator = allocator();

        let replacement = Arc::new(HostMemoryProvider::new());

        let result = allocator.register_provider(replacement);

        assert!(result.is_ok());
    }

    #[test]
    fn allocation_statistics_track_peaks() {
        let allocator = allocator();

        {
            let _a = allocator
                .allocate_host(ByteCount::new(2048), AllocationClass::Temporary)
                .expect("allocation");

            assert_eq!(
                allocator.peak_allocated_bytes().expect("peak"),
                2048
            );
        }

        assert_eq!(
            allocator.allocated_bytes().expect("current"),
            0
        );

        assert_eq!(
            allocator.peak_allocated_bytes().expect("peak"),
            2048
        );
    }
}