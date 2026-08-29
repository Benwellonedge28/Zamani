//! Zamani Quantum Memory — Provider-Neutral Backend-Native State
//!
//! Production-grade abstraction for quantum state and quantum-memory resources
//! that are owned by an external execution target rather than by Zamani's
//! local state representations.
//!
//! # Responsibility
//!
//! This module represents opaque quantum state/memory resources that may live
//! on any supported execution target, including:
//!
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom QPUs;
//! - photonic processors;
//! - spin/semiconductor devices;
//! - topological devices;
//! - analog quantum processors;
//! - quantum annealers;
//! - logical/fault-tolerant QPUs;
//! - networked/distributed quantum processors;
//! - local hardware-oriented emulators;
//! - local simulators;
//! - remote simulators;
//! - future quantum execution technologies.
//!
//! The module deliberately does NOT implement provider SDKs, network I/O,
//! authentication, credentials, allocation on a particular vendor, or quantum
//! simulation mathematics. It defines the stable memory-side contract that
//! those implementations consume.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      v
//! compiler / algorithms / optimization / QEC
//!      |
//!      v
//! execution planning
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! local memory                   hardware execution
//!      |                               |
//!      |                         quantum::hardware
//!      |                               |
//!      +---------------+---------------+
//!                      |
//!                      v
//!              memory::backend_state
//!                      |
//!             opaque external resource
//! ```
//!
//! `backend_state.rs` is intentionally independent of `hardware::backend.rs`.
//! The memory subsystem must not acquire a dependency on a concrete hardware
//! implementation merely to describe an opaque external resource. Hardware
//! adapters can translate their own backend identifiers/capabilities into the
//! provider-neutral types defined here.
//!
//! # Critical distinction
//!
//! A `BackendStateHandle` is NOT a local state vector and is NOT a raw pointer.
//! It is a validated, opaque description of externally owned state.
//!
//! Zamani may use it to identify, synchronize, migrate, snapshot, or release a
//! resource through an adapter, but this module never dereferences or accesses
//! provider memory directly.
//!
//! This design permits a real QPU to expose no addressable quantum RAM at all.
//! For such devices, a backend state can represent a provider execution
//! context, logical-qubit allocation, quantum session, or other opaque state
//! resource without pretending that amplitudes are locally readable.
//!
//! # No unsafe
//!
//! Unsafe Rust is forbidden in this file. Provider adapters must expose safe
//! abstractions to this boundary as well.
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//! - no external dependencies
//! - no `unsafe`
//!
//! # File-completion invariant
//!
//! This file is designed to be completed before the other memory
//! implementation modules. It therefore owns all contracts required to
//! describe a backend-native resource without importing `representation.rs`,
//! `allocator.rs`, `hardware::*`, or provider SDK types.
//!
//! Later modules integrate with these contracts rather than modifying them:
//!
//! - `state.rs` can treat `BackendStateHandle` as the `BackendNative` state
//!   representation.
//! - `representation.rs` can map its representation enum to/from
//!   `BackendStateKind` without changing this file.
//! - `address.rs` can use `BackendMemoryAddress` as an opaque remote address.
//! - `migration.rs` can use `BackendStateTransfer` and
//!   `BackendStateSnapshot`.
//! - `coherence.rs` can use `BackendStateSyncToken`.
//! - `synchronization.rs` can use `BackendStateSyncToken` and
//!   `BackendStateTransfer`.
//! - `snapshot.rs` / `checkpoint.rs` can persist the safe descriptors,
//!   never provider secrets.
//! - `gpu.rs`, `distributed.rs`, and hardware adapters can implement
//!   `BackendStateProvider`.
//! - `diagnostics.rs` and `telemetry.rs` can consume the safe metadata types.
//! - `quantum::hardware::backend_trait` remains the execution lifecycle
//!   authority; this module does not duplicate job submission APIs.
//!
//! Adding a new QPU provider must not require changing this file.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for backend-native quantum memory.
pub const BACKEND_STATE_SCHEMA_ID: &str =
    "zamani.quantum.memory.backend_state";

/// Semantic schema version.
pub const BACKEND_STATE_SCHEMA_VERSION: u16 = 1;

/// Maximum length of provider/backend/device identifiers accepted here.
pub const MAX_IDENTIFIER_LENGTH: usize = 512;

/// Maximum length of an opaque resource identifier.
pub const MAX_RESOURCE_ID_LENGTH: usize = 1024;

/// Maximum length of a provider-neutral format or representation identifier.
pub const MAX_KIND_LENGTH: usize = 128;

/// Maximum number of metadata entries accepted on one resource.
pub const MAX_METADATA_ENTRIES: usize = 256;

/// Maximum key length for safe metadata.
pub const MAX_METADATA_KEY_LENGTH: usize = 128;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of logical qubit identifiers attached to one backend state
/// descriptor through this generic boundary.
pub const MAX_QUBIT_IDS: usize = 1_000_000;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced by backend-native quantum-memory contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendStateError {
    /// An identifier was empty, too long, or contained forbidden characters.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A resource identifier exceeded the protocol limit.
    ResourceIdTooLong,

    /// A representation/storage kind was empty or too long.
    InvalidKind,

    /// A metadata key/value was invalid.
    InvalidMetadata {
        field: &'static str,
    },

    /// Too many metadata entries were supplied.
    MetadataLimitExceeded {
        maximum: usize,
    },

    /// Too many logical qubit identifiers were supplied.
    QubitIdLimitExceeded {
        maximum: usize,
    },

    /// A resource has already reached a terminal lifecycle state.
    InvalidLifecycleTransition,

    /// An operation requires ownership semantics that are not satisfied.
    OwnershipViolation,

    /// A resource from another backend/device cannot be combined with this
    /// resource under the requested operation.
    BackendMismatch,

    /// A state resource cannot satisfy a requested operation.
    UnsupportedOperation,

    /// A state transfer is incompatible with the source/destination contract.
    IncompatibleTransfer,

    /// A requested byte count would overflow or is otherwise invalid.
    InvalidByteCount,

    /// A generation/version token did not match the current resource.
    StaleGeneration,

    /// A synchronization token is invalid for the selected resource.
    InvalidSynchronizationToken,

    /// The provider does not expose a usable identifier for this resource.
    MissingResourceIdentity,

    /// An operation requires provider interaction and therefore cannot be
    /// completed by this pure contract layer.
    ProviderOperationRequired,
}

impl fmt::Display for BackendStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid backend-state identifier: {field}")
            }
            Self::ResourceIdTooLong => {
                formatter.write_str("backend resource identifier is too long")
            }
            Self::InvalidKind => {
                formatter.write_str("invalid backend-state kind")
            }
            Self::InvalidMetadata { field } => {
                write!(formatter, "invalid backend-state metadata {field}")
            }
            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "backend-state metadata limit exceeded: maximum {maximum}"
                )
            }
            Self::QubitIdLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "backend-state qubit-id limit exceeded: maximum {maximum}"
                )
            }
            Self::InvalidLifecycleTransition => {
                formatter.write_str("invalid backend-state lifecycle transition")
            }
            Self::OwnershipViolation => {
                formatter.write_str("backend-state ownership violation")
            }
            Self::BackendMismatch => {
                formatter.write_str("backend-state backend/device mismatch")
            }
            Self::UnsupportedOperation => {
                formatter.write_str("unsupported backend-state operation")
            }
            Self::IncompatibleTransfer => {
                formatter.write_str("incompatible backend-state transfer")
            }
            Self::InvalidByteCount => {
                formatter.write_str("invalid backend-state byte count")
            }
            Self::StaleGeneration => {
                formatter.write_str("stale backend-state generation")
            }
            Self::InvalidSynchronizationToken => {
                formatter.write_str("invalid backend-state synchronization token")
            }
            Self::MissingResourceIdentity => {
                formatter.write_str("backend-state resource identity is missing")
            }
            Self::ProviderOperationRequired => {
                formatter.write_str("provider operation is required")
            }
        }
    }
}

impl std::error::Error for BackendStateError {}

/// Result alias for this module.
pub type BackendStateResult<T> = Result<T, BackendStateError>;

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
    maximum: usize,
) -> BackendStateResult<()> {
    if value.is_empty() || value.len() > maximum {
        return Err(BackendStateError::InvalidIdentifier { field });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(BackendStateError::InvalidIdentifier { field });
    }

    Ok(())
}

fn validate_kind(value: &str) -> BackendStateResult<()> {
    if value.is_empty() || value.len() > MAX_KIND_LENGTH {
        return Err(BackendStateError::InvalidKind);
    }

    if value
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(BackendStateError::InvalidKind);
    }

    Ok(())
}

fn checked_bytes(bytes: u64) -> BackendStateResult<u64> {
    if bytes == 0 {
        return Err(BackendStateError::InvalidByteCount);
    }

    Ok(bytes)
}

// =============================================================================
// Resource identity
// =============================================================================

/// Provider-neutral identity of the owner of an external quantum resource.
///
/// This is metadata only. It contains no credentials and no network secrets.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendResourceOwner {
    provider_id: String,
    backend_id: String,
    device_id: String,
}

impl BackendResourceOwner {
    /// Creates a validated owner identity.
    pub fn new(
        provider_id: impl Into<String>,
        backend_id: impl Into<String>,
        device_id: impl Into<String>,
    ) -> BackendStateResult<Self> {
        let provider_id = provider_id.into();
        let backend_id = backend_id.into();
        let device_id = device_id.into();

        validate_identifier(
            &provider_id,
            "provider_id",
            MAX_IDENTIFIER_LENGTH,
        )?;
        validate_identifier(
            &backend_id,
            "backend_id",
            MAX_IDENTIFIER_LENGTH,
        )?;
        validate_identifier(
            &device_id,
            "device_id",
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self {
            provider_id,
            backend_id,
            device_id,
        })
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }
}

impl fmt::Debug for BackendResourceOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackendResourceOwner")
            .field("provider_id", &self.provider_id)
            .field("backend_id", &self.backend_id)
            .field("device_id", &self.device_id)
            .finish()
    }
}

/// Opaque identity of an external backend-owned quantum resource.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendResourceId(String);

impl BackendResourceId {
    /// Creates a validated opaque resource identifier.
    pub fn new(value: impl Into<String>) -> BackendStateResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "resource_id",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BackendResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BackendResourceId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for BackendResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Optional provider-neutral allocation identity.
///
/// Some QPUs do not expose an allocation object. In that case this remains
/// `None`; the state resource identity is sufficient.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendAllocationId(String);

impl BackendAllocationId {
    pub fn new(value: impl Into<String>) -> BackendStateResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "allocation_id",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BackendAllocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BackendAllocationId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for BackendAllocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Backend state kind
// =============================================================================

/// Provider-neutral description of what an external resource represents.
///
/// `Custom(String)` is mandatory for forward compatibility. New quantum
/// execution technologies must not require modifying this core memory file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendStateKind {
    /// A provider-native pure-state resource.
    StateVector,

    /// A provider-native mixed-state resource.
    DensityMatrix,

    /// A provider-native stabilizer/tableau resource.
    Stabilizer,

    /// A provider-native sparse-state resource.
    Sparse,

    /// A provider-native tensor-network resource.
    TensorNetwork,

    /// Logical/fault-tolerant quantum memory.
    Logical,

    /// Physical-qubit allocation/session without an exposed mathematical
    /// state representation.
    PhysicalQubits,

    /// Analog/Hamiltonian execution context.
    Analog,

    /// Annealing/Ising/QUBO execution context.
    Annealing,

    /// Photonic or other continuous-variable/provider-native representation.
    Photonic,

    /// Opaque provider-defined state resource.
    Custom(String),
}

impl BackendStateKind {
    /// Returns a stable wire identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Stabilizer => "stabilizer",
            Self::Sparse => "sparse",
            Self::TensorNetwork => "tensor_network",
            Self::Logical => "logical",
            Self::PhysicalQubits => "physical_qubits",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Photonic => "photonic",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Constructs a kind from a stable wire identifier.
    pub fn from_str(value: impl Into<String>) -> BackendStateResult<Self> {
        let value = value.into();

        validate_kind(&value)?;

        Ok(match value.as_str() {
            "state_vector" => Self::StateVector,
            "density_matrix" => Self::DensityMatrix,
            "stabilizer" => Self::Stabilizer,
            "sparse" => Self::Sparse,
            "tensor_network" => Self::TensorNetwork,
            "logical" => Self::Logical,
            "physical_qubits" => Self::PhysicalQubits,
            "analog" => Self::Analog,
            "annealing" => Self::Annealing,
            "photonic" => Self::Photonic,
            _ => Self::Custom(value),
        })
    }

    /// Returns true when the resource is expected to represent a gate-model
    /// mathematical state rather than merely an execution context.
    pub const fn is_mathematical_state(&self) -> bool {
        matches!(
            self,
            Self::StateVector
                | Self::DensityMatrix
                | Self::Stabilizer
                | Self::Sparse
                | Self::TensorNetwork
        )
    }
}

// =============================================================================
// Storage location and ownership
// =============================================================================

/// Where the backend-native resource is physically/logically owned.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendStorageLocation {
    /// Local CPU memory owned by a simulator/emulator/host adapter.
    Host,

    /// Host memory pinned for accelerator transfers.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/managed host-device memory.
    Unified,

    /// Memory distributed across multiple execution nodes.
    Distributed,

    /// Quantum memory physically owned by a QPU.
    Qpu,

    /// Remote simulator/emulator or remote execution service.
    Remote,

    /// Provider-defined location.
    Custom(String),
}

impl BackendStorageLocation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::Qpu => "qpu",
            Self::Remote => "remote",
            Self::Custom(value) => value.as_str(),
        }
    }
}

/// Ownership semantics of a backend-native resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendResourceOwnership {
    /// Zamani owns the lifecycle and may request release through the provider.
    Owned,

    /// The resource is borrowed from an external owner and must not be
    /// released by generic memory code.
    Borrowed,

    /// The resource is shared; release occurs only after the provider-side
    /// reference contract permits it.
    Shared,

    /// The resource's lifecycle is controlled exclusively by a remote job or
    /// provider session.
    ProviderManaged,
}

// =============================================================================
// Lifecycle
// =============================================================================

/// Lifecycle of an external quantum memory/state resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendStateLifecycle {
    /// Resource descriptor exists but provider allocation has not been
    /// confirmed.
    Declared,

    /// Provider allocation/resource creation has been confirmed.
    Allocated,

    /// Resource is available for operations.
    Ready,

    /// A provider operation currently owns/locks the resource.
    Busy,

    /// State/resource contents have changed since the last synchronization
    /// point known to Zamani.
    Dirty,

    /// Resource is being released.
    Releasing,

    /// Resource has been released and must not be used again.
    Released,

    /// Resource became invalid because the provider lost or invalidated it.
    Invalid,
}

impl BackendStateLifecycle {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Released | Self::Invalid)
    }

    pub const fn is_usable(self) -> bool {
        matches!(
            self,
            Self::Allocated
                | Self::Ready
                | Self::Busy
                | Self::Dirty
        )
    }

    pub const fn can_release(self) -> bool {
        matches!(
            self,
            Self::Declared
                | Self::Allocated
                | Self::Ready
                | Self::Busy
                | Self::Dirty
        )
    }
}

// =============================================================================
// Version/generation identity
// =============================================================================

/// Monotonic resource generation.
///
/// Providers/adapters increment this value whenever an operation invalidates
/// a previously observed state view. `0` is the initial generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendStateGeneration(u64);

impl BackendStateGeneration {
    pub const INITIAL: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> BackendStateResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(BackendStateError::StaleGeneration)
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Safe provider-neutral metadata attached to a backend state.
///
/// Metadata is deliberately a sorted vector rather than a `HashMap` so the
/// descriptor remains deterministic and serializable without an external
/// serialization crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateMetadata {
    entries: Vec<(String, String)>,
}

impl BackendStateMetadata {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> BackendStateResult<()> {
        let key = key.into();
        let value = value.into();

        if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
            return Err(BackendStateError::InvalidMetadata {
                field: "key",
            });
        }

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(BackendStateError::InvalidMetadata {
                field: "value",
            });
        }

        if key.chars().any(|character| character.is_control())
            || value.chars().any(|character| character.is_control())
        {
            return Err(BackendStateError::InvalidMetadata {
                field: "value",
            });
        }

        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|(existing, _)| existing == &key)
        {
            existing.1 = value;
            return Ok(());
        }

        if self.entries.len() >= MAX_METADATA_ENTRIES {
            return Err(BackendStateError::MetadataLimitExceeded {
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        self.entries.push((key, value));
        self.entries
            .sort_by(|left, right| left.0.cmp(&right.0));

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .binary_search_by(|(existing, _)| existing.as_str().cmp(key))
            .ok()
            .map(|index| self.entries[index].1.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for BackendStateMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Opaque address
// =============================================================================

/// Opaque provider/device address.
///
/// This is deliberately a string token rather than a pointer or integer
/// address. It may represent a device buffer ID, remote object URI, distributed
/// partition identifier, QPU memory slot, or provider-native handle encoded by
/// the adapter. The token must never contain credentials.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendMemoryAddress(String);

impl BackendMemoryAddress {
    pub fn new(value: impl Into<String>) -> BackendStateResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "memory_address",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BackendMemoryAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BackendMemoryAddress")
            .field(&self.0)
            .finish()
    }
}

// =============================================================================
// Capabilities
// =============================================================================

/// Operations a backend-native resource may support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStateOperation {
    Read,
    Write,
    ApplyOperation,
    Measure,
    Reset,
    Snapshot,
    Checkpoint,
    Clone,
    Copy,
    Migrate,
    Synchronize,
    Release,
    PartialTrace,
    TensorProduct,
    HostTransfer,
    DeviceTransfer,
    DistributedTransfer,
    ProviderNative,
}

/// Provider-neutral capability declaration for one backend-native state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateCapabilities {
    operations: Vec<BackendStateOperation>,
    supports_zero_copy: bool,
    supports_concurrent_read: bool,
    supports_concurrent_write: bool,
    supports_persistence: bool,
    supports_generation_tracking: bool,
    supports_partial_access: bool,
}

impl BackendStateCapabilities {
    pub fn new(
        operations: impl Into<Vec<BackendStateOperation>>,
    ) -> Self {
        let mut operations = operations.into();

        operations.sort_by_key(|operation| *operation as u8);
        operations.dedup();

        Self {
            operations,
            supports_zero_copy: false,
            supports_concurrent_read: false,
            supports_concurrent_write: false,
            supports_persistence: false,
            supports_generation_tracking: true,
            supports_partial_access: false,
        }
    }

    pub fn supports(
        &self,
        operation: BackendStateOperation,
    ) -> bool {
        self.operations.binary_search(&operation).is_ok()
    }

    pub fn operations(&self) -> &[BackendStateOperation] {
        &self.operations
    }

    pub const fn supports_zero_copy(&self) -> bool {
        self.supports_zero_copy
    }

    pub const fn supports_concurrent_read(&self) -> bool {
        self.supports_concurrent_read
    }

    pub const fn supports_concurrent_write(&self) -> bool {
        self.supports_concurrent_write
    }

    pub const fn supports_persistence(&self) -> bool {
        self.supports_persistence
    }

    pub const fn supports_generation_tracking(&self) -> bool {
        self.supports_generation_tracking
    }

    pub const fn supports_partial_access(&self) -> bool {
        self.supports_partial_access
    }

    pub fn with_zero_copy(mut self, supported: bool) -> Self {
        self.supports_zero_copy = supported;
        self
    }

    pub fn with_concurrent_read(mut self, supported: bool) -> Self {
        self.supports_concurrent_read = supported;
        self
    }

    pub fn with_concurrent_write(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_concurrent_write = supported;
        self
    }

    pub fn with_persistence(mut self, supported: bool) -> Self {
        self.supports_persistence = supported;
        self
    }

    pub fn with_generation_tracking(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_generation_tracking = supported;
        self
    }

    pub fn with_partial_access(
        mut self,
        supported: bool,
    ) -> Self {
        self.supports_partial_access = supported;
        self
    }
}

// =============================================================================
// State descriptor
// =============================================================================

/// Complete provider-neutral descriptor for an external quantum state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateDescriptor {
    owner: BackendResourceOwner,
    resource_id: BackendResourceId,
    allocation_id: Option<BackendAllocationId>,
    kind: BackendStateKind,
    location: BackendStorageLocation,
    ownership: BackendResourceOwnership,
    lifecycle: BackendStateLifecycle,
    generation: BackendStateGeneration,
    qubit_count: u64,
    logical_qubit_ids: Vec<u64>,
    byte_size: Option<u64>,
    address: Option<BackendMemoryAddress>,
    capabilities: BackendStateCapabilities,
    metadata: BackendStateMetadata,
}

impl BackendStateDescriptor {
    /// Creates a validated descriptor.
    pub fn new(
        owner: BackendResourceOwner,
        resource_id: BackendResourceId,
        kind: BackendStateKind,
        location: BackendStorageLocation,
        ownership: BackendResourceOwnership,
        qubit_count: u64,
    ) -> BackendStateResult<Self> {
        if qubit_count == 0 {
            return Err(BackendStateError::InvalidByteCount);
        }

        Ok(Self {
            owner,
            resource_id,
            allocation_id: None,
            kind,
            location,
            ownership,
            lifecycle: BackendStateLifecycle::Declared,
            generation: BackendStateGeneration::INITIAL,
            qubit_count,
            logical_qubit_ids: Vec::new(),
            byte_size: None,
            address: None,
            capabilities: BackendStateCapabilities::new(Vec::new()),
            metadata: BackendStateMetadata::new(),
        })
    }

    pub fn owner(&self) -> &BackendResourceOwner {
        &self.owner
    }

    pub fn resource_id(&self) -> &BackendResourceId {
        &self.resource_id
    }

    pub fn allocation_id(&self) -> Option<&BackendAllocationId> {
        self.allocation_id.as_ref()
    }

    pub fn kind(&self) -> &BackendStateKind {
        &self.kind
    }

    pub fn location(&self) -> &BackendStorageLocation {
        &self.location
    }

    pub const fn ownership(&self) -> BackendResourceOwnership {
        self.ownership
    }

    pub const fn lifecycle(&self) -> BackendStateLifecycle {
        self.lifecycle
    }

    pub const fn generation(&self) -> BackendStateGeneration {
        self.generation
    }

    pub const fn qubit_count(&self) -> u64 {
        self.qubit_count
    }

    pub fn logical_qubit_ids(&self) -> &[u64] {
        &self.logical_qubit_ids
    }

    pub const fn byte_size(&self) -> Option<u64> {
        self.byte_size
    }

    pub fn address(&self) -> Option<&BackendMemoryAddress> {
        self.address.as_ref()
    }

    pub fn capabilities(&self) -> &BackendStateCapabilities {
        &self.capabilities
    }

    pub fn metadata(&self) -> &BackendStateMetadata {
        &self.metadata
    }

    /// Associates an allocation identity with this descriptor.
    pub fn with_allocation_id(
        mut self,
        allocation_id: BackendAllocationId,
    ) -> Self {
        self.allocation_id = Some(allocation_id);
        self
    }

    /// Associates an opaque address with this descriptor.
    pub fn with_address(
        mut self,
        address: BackendMemoryAddress,
    ) -> Self {
        self.address = Some(address);
        self
    }

    /// Associates a known byte size with this descriptor.
    pub fn with_byte_size(
        mut self,
        bytes: u64,
    ) -> BackendStateResult<Self> {
        self.byte_size = Some(checked_bytes(bytes)?);
        Ok(self)
    }

    /// Sets the provider-native capability declaration.
    pub fn with_capabilities(
        mut self,
        capabilities: BackendStateCapabilities,
    ) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Adds or replaces safe metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> BackendStateResult<Self> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Attaches logical qubit identifiers to the external resource.
    ///
    /// These are Zamani-side logical identities. They are not interpreted as
    /// provider physical addresses.
    pub fn with_logical_qubit_ids(
        mut self,
        ids: impl Into<Vec<u64>>,
    ) -> BackendStateResult<Self> {
        let ids = ids.into();

        if ids.len() > MAX_QUBIT_IDS
            || (ids.len() as u64) > self.qubit_count
        {
            return Err(BackendStateError::QubitIdLimitExceeded {
                maximum: self.qubit_count as usize,
            });
        }

        self.logical_qubit_ids = ids;
        Ok(self)
    }

    /// Advances the lifecycle without permitting impossible terminal-state
    /// resurrection.
    pub fn transition_to(
        &mut self,
        next: BackendStateLifecycle,
    ) -> BackendStateResult<()> {
        if self.lifecycle.is_terminal() {
            return Err(BackendStateError::InvalidLifecycleTransition);
        }

        let valid = match (self.lifecycle, next) {
            (
                BackendStateLifecycle::Declared,
                BackendStateLifecycle::Allocated,
            ) => true,

            (
                BackendStateLifecycle::Allocated,
                BackendStateLifecycle::Ready,
            ) => true,

            (
                BackendStateLifecycle::Ready,
                BackendStateLifecycle::Busy,
            ) => true,

            (
                BackendStateLifecycle::Ready,
                BackendStateLifecycle::Dirty,
            ) => true,

            (
                BackendStateLifecycle::Busy,
                BackendStateLifecycle::Ready,
            ) => true,

            (
                BackendStateLifecycle::Busy,
                BackendStateLifecycle::Dirty,
            ) => true,

            (
                BackendStateLifecycle::Dirty,
                BackendStateLifecycle::Ready,
            ) => true,

            (
                BackendStateLifecycle::Dirty,
                BackendStateLifecycle::Busy,
            ) => true,

            (_, BackendStateLifecycle::Releasing) => {
                self.lifecycle.can_release()
            }

            (
                BackendStateLifecycle::Releasing,
                BackendStateLifecycle::Released,
            ) => true,

            (_, BackendStateLifecycle::Invalid) => true,

            (current, requested) if current == requested => true,

            _ => false,
        };

        if !valid {
            return Err(BackendStateError::InvalidLifecycleTransition);
        }

        self.lifecycle = next;
        Ok(())
    }

    /// Marks the resource as changed and advances its generation.
    pub fn mark_dirty(&mut self) -> BackendStateResult<()> {
        self.generation = self.generation.next()?;
        self.lifecycle = BackendStateLifecycle::Dirty;
        Ok(())
    }

    /// Marks a resource as synchronized/ready using an explicitly supplied
    /// generation.
    pub fn mark_ready(
        &mut self,
        generation: BackendStateGeneration,
    ) {
        self.generation = generation;
        self.lifecycle = BackendStateLifecycle::Ready;
    }
}

// =============================================================================
// Stable opaque handle
// =============================================================================

/// Cloneable, immutable handle to a backend-native quantum state descriptor.
///
/// Cloning this handle never duplicates provider memory. It duplicates only
/// the safe descriptor. Actual ownership/reference counting remains the
/// responsibility of the provider adapter.
#[derive(Clone)]
pub struct BackendStateHandle {
    descriptor: Arc<BackendStateDescriptor>,
}

impl BackendStateHandle {
    pub fn new(descriptor: BackendStateDescriptor) -> Self {
        Self {
            descriptor: Arc::new(descriptor),
        }
    }

    pub fn descriptor(&self) -> &BackendStateDescriptor {
        self.descriptor.as_ref()
    }

    pub fn resource_id(&self) -> &BackendResourceId {
        self.descriptor.resource_id()
    }

    pub fn owner(&self) -> &BackendResourceOwner {
        self.descriptor.owner()
    }

    pub fn generation(&self) -> BackendStateGeneration {
        self.descriptor.generation()
    }

    pub fn lifecycle(&self) -> BackendStateLifecycle {
        self.descriptor.lifecycle()
    }

    pub fn kind(&self) -> &BackendStateKind {
        self.descriptor.kind()
    }

    pub fn qubit_count(&self) -> u64 {
        self.descriptor.qubit_count()
    }

    pub fn is_terminal(&self) -> bool {
        self.lifecycle().is_terminal()
    }

    /// Compares two handles for identity, not contents.
    pub fn same_resource(&self, other: &Self) -> bool {
        self.resource_id() == other.resource_id()
            && self.owner() == other.owner()
    }
}

impl fmt::Debug for BackendStateHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackendStateHandle")
            .field("owner", &self.owner())
            .field("resource_id", &self.resource_id())
            .field("kind", &self.kind())
            .field("location", &self.descriptor.location())
            .field("ownership", &self.descriptor.ownership())
            .field("lifecycle", &self.lifecycle())
            .field("generation", &self.generation())
            .field("qubit_count", &self.qubit_count())
            .finish()
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Direction of synchronization between Zamani and the backend resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStateSyncDirection {
    /// Backend state becomes authoritative.
    BackendToZamani,

    /// Zamani-owned state becomes authoritative for the adapter operation.
    ZamaniToBackend,

    /// Both sides must be reconciled by the provider adapter.
    Bidirectional,
}

/// Immutable synchronization token.
///
/// The token contains no pointer and no secret. It is a generation-bound
/// logical marker that adapters may validate before committing a transfer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackendStateSyncToken {
    resource_id: BackendResourceId,
    generation: BackendStateGeneration,
}

impl BackendStateSyncToken {
    pub fn new(handle: &BackendStateHandle) -> Self {
        Self {
            resource_id: handle.resource_id().clone(),
            generation: handle.generation(),
        }
    }

    pub fn resource_id(&self) -> &BackendResourceId {
        &self.resource_id
    }

    pub const fn generation(&self) -> BackendStateGeneration {
        self.generation
    }

    pub fn validate_against(
        &self,
        handle: &BackendStateHandle,
    ) -> BackendStateResult<()> {
        if self.resource_id != *handle.resource_id() {
            return Err(BackendStateError::BackendMismatch);
        }

        if self.generation != handle.generation() {
            return Err(BackendStateError::StaleGeneration);
        }

        Ok(())
    }
}

// =============================================================================
// Transfer contract
// =============================================================================

/// Transfer destination for backend-native state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendStateTransferTarget {
    /// Transfer into another opaque external resource.
    Backend(BackendStateHandle),

    /// Transfer into a host/device abstraction owned by another memory layer.
    MemoryAddress(BackendMemoryAddress),

    /// Provider-defined target that remains opaque to this module.
    Custom(String),
}

/// Immutable state-transfer description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateTransfer {
    source: BackendStateHandle,
    target: BackendStateTransferTarget,
    direction: BackendStateSyncDirection,
    expected_generation: BackendStateGeneration,
    byte_count: Option<u64>,
}

impl BackendStateTransfer {
    pub fn new(
        source: BackendStateHandle,
        target: BackendStateTransferTarget,
        direction: BackendStateSyncDirection,
    ) -> Self {
        let expected_generation = source.generation();

        Self {
            source,
            target,
            direction,
            expected_generation,
            byte_count: None,
        }
    }

    pub fn source(&self) -> &BackendStateHandle {
        &self.source
    }

    pub fn target(&self) -> &BackendStateTransferTarget {
        &self.target
    }

    pub const fn direction(&self) -> BackendStateSyncDirection {
        self.direction
    }

    pub const fn expected_generation(
        &self,
    ) -> BackendStateGeneration {
        self.expected_generation
    }

    pub const fn byte_count(&self) -> Option<u64> {
        self.byte_count
    }

    pub fn with_byte_count(
        mut self,
        bytes: u64,
    ) -> BackendStateResult<Self> {
        self.byte_count = Some(checked_bytes(bytes)?);
        Ok(self)
    }

    /// Validates invariants that can be checked without contacting a provider.
    pub fn validate(&self) -> BackendStateResult<()> {
        if self.source.is_terminal() {
            return Err(BackendStateError::InvalidLifecycleTransition);
        }

        if self.source.generation() != self.expected_generation {
            return Err(BackendStateError::StaleGeneration);
        }

        if let BackendStateTransferTarget::Backend(target) = &self.target {
            if self.direction == BackendStateSyncDirection::Bidirectional
                && !self.source.same_resource(target)
            {
                return Err(BackendStateError::IncompatibleTransfer);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Snapshot descriptor
// =============================================================================

/// Provider-neutral description of a backend-native snapshot.
///
/// The actual snapshot bytes/blob are owned by the adapter or persistence
/// layer. This object carries only safe identity/provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateSnapshot {
    resource_id: BackendResourceId,
    generation: BackendStateGeneration,
    kind: BackendStateKind,
    qubit_count: u64,
    byte_size: Option<u64>,
    snapshot_id: BackendResourceId,
    format: String,
    checksum: Option<String>,
}

impl BackendStateSnapshot {
    pub fn new(
        resource: &BackendStateHandle,
        snapshot_id: BackendResourceId,
        format: impl Into<String>,
    ) -> BackendStateResult<Self> {
        let format = format.into();

        validate_kind(&format)?;

        Ok(Self {
            resource_id: resource.resource_id().clone(),
            generation: resource.generation(),
            kind: resource.kind().clone(),
            qubit_count: resource.qubit_count(),
            byte_size: resource.descriptor().byte_size(),
            snapshot_id,
            format,
            checksum: None,
        })
    }

    pub fn resource_id(&self) -> &BackendResourceId {
        &self.resource_id
    }

    pub const fn generation(
        &self,
    ) -> BackendStateGeneration {
        self.generation
    }

    pub fn kind(&self) -> &BackendStateKind {
        &self.kind
    }

    pub const fn qubit_count(&self) -> u64 {
        self.qubit_count
    }

    pub const fn byte_size(&self) -> Option<u64> {
        self.byte_size
    }

    pub fn snapshot_id(&self) -> &BackendResourceId {
        &self.snapshot_id
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    /// Sets an integrity identifier such as a hexadecimal digest.
    ///
    /// This module does not implement hashing and therefore treats the
    /// checksum as opaque.
    pub fn with_checksum(
        mut self,
        checksum: impl Into<String>,
    ) -> BackendStateResult<Self> {
        let checksum = checksum.into();

        validate_identifier(
            &checksum,
            "checksum",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        self.checksum = Some(checksum);
        Ok(self)
    }
}

// =============================================================================
// Provider adapter contract
// =============================================================================

/// Safe provider-neutral interface for backend-native state resources.
///
/// This trait deliberately contains no async methods because Rust 1.97's
/// standard library does not provide a universal async-trait abstraction
/// without choosing an executor/runtime.
///
/// Providers can wrap these synchronous contract methods in their own async
/// adapter and expose the resulting job lifecycle through
/// `quantum::hardware::backend_trait`.
///
/// Implementations MUST:
///
/// - perform all provider-specific I/O below this boundary;
/// - never expose provider SDK types through public method signatures;
/// - never expose raw pointers;
/// - never store credentials in `BackendStateDescriptor`;
/// - validate resource identity and generation before destructive operations;
/// - preserve provider-native semantics instead of fabricating unsupported
///   operations;
/// - return `UnsupportedOperation` when the provider cannot perform an
///   operation rather than silently emulating it.
pub trait BackendStateProvider: Send + Sync {
    /// Creates/allocates a backend-native state resource.
    fn allocate(
        &self,
        request: &BackendStateAllocationRequest,
    ) -> BackendStateResult<BackendStateHandle>;

    /// Refreshes an immutable handle/descriptor from provider state.
    fn refresh(
        &self,
        handle: &BackendStateHandle,
    ) -> BackendStateResult<BackendStateHandle>;

    /// Releases a provider-owned resource.
    fn release(
        &self,
        handle: &BackendStateHandle,
    ) -> BackendStateResult<()>;

    /// Applies a provider-defined operation represented by an opaque
    /// operation identifier/payload.
    fn apply_operation(
        &self,
        handle: &BackendStateHandle,
        operation: &BackendStateOperationRequest,
    ) -> BackendStateResult<BackendStateHandle>;

    /// Synchronizes a resource according to a validated transfer contract.
    fn synchronize(
        &self,
        transfer: &BackendStateTransfer,
    ) -> BackendStateResult<BackendStateHandle>;

    /// Creates a provider-native snapshot descriptor.
    fn snapshot(
        &self,
        handle: &BackendStateHandle,
        request: &BackendStateSnapshotRequest,
    ) -> BackendStateResult<BackendStateSnapshot>;

    /// Returns a provider-native health/status observation.
    fn health(
        &self,
        handle: &BackendStateHandle,
    ) -> BackendStateResult<BackendStateHealth>;
}

/// Allocation request for an external quantum state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateAllocationRequest {
    pub owner: BackendResourceOwner,
    pub kind: BackendStateKind,
    pub location: BackendStorageLocation,
    pub ownership: BackendResourceOwnership,
    pub qubit_count: u64,
    pub logical_qubit_ids: Vec<u64>,
    pub requested_bytes: Option<u64>,
    pub metadata: BackendStateMetadata,
}

impl BackendStateAllocationRequest {
    pub fn new(
        owner: BackendResourceOwner,
        kind: BackendStateKind,
        location: BackendStorageLocation,
        ownership: BackendResourceOwnership,
        qubit_count: u64,
    ) -> BackendStateResult<Self> {
        if qubit_count == 0 {
            return Err(BackendStateError::InvalidByteCount);
        }

        Ok(Self {
            owner,
            kind,
            location,
            ownership,
            qubit_count,
            logical_qubit_ids: Vec::new(),
            requested_bytes: None,
            metadata: BackendStateMetadata::new(),
        })
    }

    pub fn with_logical_qubit_ids(
        mut self,
        ids: Vec<u64>,
    ) -> BackendStateResult<Self> {
        if ids.len() > MAX_QUBIT_IDS
            || (ids.len() as u64) > self.qubit_count
        {
            return Err(BackendStateError::QubitIdLimitExceeded {
                maximum: self.qubit_count as usize,
            });
        }

        self.logical_qubit_ids = ids;
        Ok(self)
    }

    pub fn with_requested_bytes(
        mut self,
        bytes: u64,
    ) -> BackendStateResult<Self> {
        self.requested_bytes = Some(checked_bytes(bytes)?);
        Ok(self)
    }

    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> BackendStateResult<Self> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }
}

/// Opaque operation request passed from an executor/adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateOperationRequest {
    operation_id: String,
    payload_format: String,
    payload: Vec<u8>,
}

impl BackendStateOperationRequest {
    pub fn new(
        operation_id: impl Into<String>,
        payload_format: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> BackendStateResult<Self> {
        let operation_id = operation_id.into();
        let payload_format = payload_format.into();
        let payload = payload.into();

        validate_identifier(
            &operation_id,
            "operation_id",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        validate_kind(&payload_format)?;

        Ok(Self {
            operation_id,
            payload_format,
            payload,
        })
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn payload_format(&self) -> &str {
        &self.payload_format
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Snapshot request independent of a particular persistence format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateSnapshotRequest {
    snapshot_id: BackendResourceId,
    format: String,
    include_provider_metadata: bool,
}

impl BackendStateSnapshotRequest {
    pub fn new(
        snapshot_id: BackendResourceId,
        format: impl Into<String>,
    ) -> BackendStateResult<Self> {
        let format = format.into();

        validate_kind(&format)?;

        Ok(Self {
            snapshot_id,
            format,
            include_provider_metadata: false,
        })
    }

    pub fn snapshot_id(&self) -> &BackendResourceId {
        &self.snapshot_id
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub const fn include_provider_metadata(&self) -> bool {
        self.include_provider_metadata
    }

    pub fn with_provider_metadata(
        mut self,
        include: bool,
    ) -> Self {
        self.include_provider_metadata = include;
        self
    }
}

// =============================================================================
// Health/status
// =============================================================================

/// Normalized health state for an external resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStateHealthStatus {
    Healthy,
    Degraded,
    Busy,
    Unavailable,
    Unknown,
}

/// Safe resource health observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendStateHealth {
    pub status: BackendStateHealthStatus,
    pub generation: BackendStateGeneration,
    pub last_operation: Option<String>,
    pub observed_latency: Option<Duration>,
}

impl BackendStateHealth {
    pub fn new(
        status: BackendStateHealthStatus,
        generation: BackendStateGeneration,
    ) -> Self {
        Self {
            status,
            generation,
            last_operation: None,
            observed_latency: None,
        }
    }

    pub fn with_last_operation(
        mut self,
        operation: impl Into<String>,
    ) -> BackendStateResult<Self> {
        let operation = operation.into();

        validate_identifier(
            &operation,
            "last_operation",
            MAX_RESOURCE_ID_LENGTH,
        )?;

        self.last_operation = Some(operation);
        Ok(self)
    }

    pub fn with_latency(
        mut self,
        latency: Duration,
    ) -> Self {
        self.observed_latency = Some(latency);
        self
    }
}

// =============================================================================
// Utility policies
// =============================================================================

/// Policy describing how an external state may be exposed to higher layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStateAccessPolicy {
    /// Only provider-native operations are allowed; no local byte access.
    OpaqueOnly,

    /// Adapter may expose immutable bytes when the provider supports it.
    ReadOnly,

    /// Adapter may expose mutable bytes when the provider supports it.
    ReadWrite,

    /// State is addressable only through a synchronized mirror.
    Mirrored,
}

/// Policy for releasing an external resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStateReleasePolicy {
    /// Generic memory layer may request release only for owned resources.
    OwnedOnly,

    /// Provider decides when release is legal.
    ProviderManaged,

    /// Resource must never be automatically released.
    Manual,
}

/// Safe, provider-neutral resource estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendStateResourceEstimate {
    pub qubit_count: u64,
    pub minimum_bytes: Option<u64>,
    pub peak_bytes: Option<u64>,
    pub supports_dense_access: bool,
}

impl BackendStateResourceEstimate {
    /// Estimates dense state-vector storage.
    ///
    /// `bytes_per_amplitude` should normally be 8 for complex f32 and 16 for
    /// complex f64, depending on the scalar abstraction selected by the
    /// surrounding memory layer.
    pub fn for_dense_state_vector(
        qubit_count: u64,
        bytes_per_amplitude: u64,
    ) -> BackendStateResult<Self> {
        if qubit_count >= 64 || bytes_per_amplitude == 0 {
            return Err(BackendStateError::InvalidByteCount);
        }

        let amplitudes = 1u64
            .checked_shl(qubit_count as u32)
            .ok_or(BackendStateError::InvalidByteCount)?;

        let bytes = amplitudes
            .checked_mul(bytes_per_amplitude)
            .ok_or(BackendStateError::InvalidByteCount)?;

        Ok(Self {
            qubit_count,
            minimum_bytes: Some(bytes),
            peak_bytes: Some(bytes),
            supports_dense_access: true,
        })
    }

    /// Estimates dense density-matrix storage.
    pub fn for_density_matrix(
        qubit_count: u64,
        bytes_per_element: u64,
    ) -> BackendStateResult<Self> {
        if qubit_count >= 32 || bytes_per_element == 0 {
            return Err(BackendStateError::InvalidByteCount);
        }

        let dimension = 1u64
            .checked_shl(qubit_count as u32)
            .ok_or(BackendStateError::InvalidByteCount)?;

        let elements = dimension
            .checked_mul(dimension)
            .ok_or(BackendStateError::InvalidByteCount)?;

        let bytes = elements
            .checked_mul(bytes_per_element)
            .ok_or(BackendStateError::InvalidByteCount)?;

        Ok(Self {
            qubit_count,
            minimum_bytes: Some(bytes),
            peak_bytes: Some(bytes),
            supports_dense_access: true,
        })
    }
}

// =============================================================================
// Public helper functions
// =============================================================================

/// Returns the stable backend-state schema identifier.
#[inline]
pub const fn schema_id() -> &'static str {
    BACKEND_STATE_SCHEMA_ID
}

/// Returns the backend-state schema version.
#[inline]
pub const fn schema_version() -> u16 {
    BACKEND_STATE_SCHEMA_VERSION
}

/// Validates a backend-state resource identity without creating a resource.
pub fn validate_resource_identity(
    provider_id: &str,
    backend_id: &str,
    device_id: &str,
    resource_id: &str,
) -> BackendStateResult<()> {
    validate_identifier(
        provider_id,
        "provider_id",
        MAX_IDENTIFIER_LENGTH,
    )?;

    validate_identifier(
        backend_id,
        "backend_id",
        MAX_IDENTIFIER_LENGTH,
    )?;

    validate_identifier(
        device_id,
        "device_id",
        MAX_IDENTIFIER_LENGTH,
    )?;

    validate_identifier(
        resource_id,
        "resource_id",
        MAX_RESOURCE_ID_LENGTH,
    )?;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn owner() -> BackendResourceOwner {
        BackendResourceOwner::new(
            "provider",
            "backend",
            "device",
        )
        .expect("valid owner")
    }

    fn resource() -> BackendStateHandle {
        let descriptor = BackendStateDescriptor::new(
            owner(),
            BackendResourceId::new("resource-1")
                .expect("valid id"),
            BackendStateKind::StateVector,
            BackendStorageLocation::Qpu,
            BackendResourceOwnership::ProviderManaged,
            5,
        )
        .expect("valid descriptor");

        BackendStateHandle::new(descriptor)
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert!(BackendResourceId::new("bad\nvalue").is_err());

        assert!(
            BackendResourceOwner::new(
                "provider",
                "backend",
                "\0device",
            )
            .is_err()
        );
    }

    #[test]
    fn custom_kinds_are_forward_compatible() {
        let kind = BackendStateKind::from_str(
            "future_continuous_variable",
        )
        .expect("valid kind");

        assert_eq!(
            kind.as_str(),
            "future_continuous_variable"
        );

        assert!(
            matches!(
                kind,
                BackendStateKind::Custom(_)
            )
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata = BackendStateMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata");

        metadata
            .insert("a", "first")
            .expect("metadata");

        metadata
            .insert("m", "middle")
            .expect("metadata");

        let keys: Vec<&str> =
            metadata.iter().map(|(key, _)| key).collect();

        assert_eq!(
            keys,
            vec!["a", "m", "z"]
        );
    }

    #[test]
    fn metadata_replaces_existing_key() {
        let mut metadata = BackendStateMetadata::new();

        metadata
            .insert("mode", "old")
            .expect("metadata");

        metadata
            .insert("mode", "new")
            .expect("metadata");

        assert_eq!(
            metadata.get("mode"),
            Some("new")
        );

        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn lifecycle_cannot_resurrect_terminal_resource() {
        let mut descriptor =
            resource().descriptor().clone();

        descriptor
            .transition_to(
                BackendStateLifecycle::Allocated,
            )
            .expect("declared -> allocated");

        descriptor
            .transition_to(
                BackendStateLifecycle::Ready,
            )
            .expect("allocated -> ready");

        descriptor
            .transition_to(
                BackendStateLifecycle::Releasing,
            )
            .expect("ready -> releasing");

        descriptor
            .transition_to(
                BackendStateLifecycle::Released,
            )
            .expect("releasing -> released");

        assert!(
            descriptor
                .transition_to(
                    BackendStateLifecycle::Ready
                )
                .is_err()
        );
    }

    #[test]
    fn generation_advances_on_dirty() {
        let mut descriptor =
            resource().descriptor().clone();

        assert_eq!(
            descriptor.generation(),
            BackendStateGeneration::INITIAL
        );

        descriptor
            .mark_dirty()
            .expect("mark dirty");

        assert_eq!(
            descriptor.generation().get(),
            1
        );

        assert_eq!(
            descriptor.lifecycle(),
            BackendStateLifecycle::Dirty
        );
    }

    #[test]
    fn synchronization_token_rejects_stale_generation() {
        let original = resource();

        let token =
            BackendStateSyncToken::new(&original);

        let mut changed =
            original.descriptor().clone();

        changed
            .mark_dirty()
            .expect("dirty");

        let changed =
            BackendStateHandle::new(changed);

        assert!(
            token
                .validate_against(&changed)
                .is_err()
        );
    }

    #[test]
    fn synchronization_token_accepts_same_resource_generation() {
        let handle = resource();

        let token =
            BackendStateSyncToken::new(&handle);

        assert!(
            token.validate_against(&handle).is_ok()
        );
    }

    #[test]
    fn dense_state_estimate_is_checked() {
        let estimate =
            BackendStateResourceEstimate::
                for_dense_state_vector(
                    10,
                    16,
                )
                .expect("valid estimate");

        assert_eq!(
            estimate.minimum_bytes,
            Some(16_384)
        );
    }

    #[test]
    fn density_matrix_estimate_is_checked() {
        let estimate =
            BackendStateResourceEstimate::
                for_density_matrix(
                    4,
                    16,
                )
                .expect("valid estimate");

        assert_eq!(
            estimate.minimum_bytes,
            Some(4_096)
        );
    }

    #[test]
    fn snapshot_preserves_provenance() {
        let handle = resource();

        let snapshot =
            BackendStateSnapshot::new(
                &handle,
                BackendResourceId::new(
                    "snapshot-1",
                )
                .expect("snapshot id"),
                "zamani-backend-state-v1",
            )
            .expect("snapshot");

        assert_eq!(
            snapshot.resource_id(),
            handle.resource_id()
        );

        assert_eq!(
            snapshot.generation(),
            handle.generation()
        );

        assert_eq!(
            snapshot.qubit_count(),
            5
        );
    }

    #[test]
    fn transfer_captures_generation() {
        let source = resource();

        let transfer =
            BackendStateTransfer::new(
                source.clone(),
                BackendStateTransferTarget::
                    Backend(source.clone()),
                BackendStateSyncDirection::
                    BackendToZamani,
            );

        assert_eq!(
            transfer.expected_generation(),
            source.generation()
        );

        assert!(
            transfer.validate().is_ok()
        );
    }

    #[test]
    fn handles_are_opaque_and_cloneable_without_copying_state() {
        let first = resource();
        let second = first.clone();

        assert!(
            first.same_resource(&second)
        );
    }

    #[test]
    fn physical_qubit_resource_is_supported_without_state_vector_assumption() {
        let descriptor =
            BackendStateDescriptor::new(
                owner(),
                BackendResourceId::new(
                    "qpu-allocation",
                )
                .expect("id"),
                BackendStateKind::PhysicalQubits,
                BackendStorageLocation::Qpu,
                BackendResourceOwnership::ProviderManaged,
                1024,
            )
            .expect("descriptor");

        assert!(
            !descriptor
                .kind()
                .is_mathematical_state()
        );

        assert_eq!(
            descriptor.qubit_count(),
            1024
        );
    }
}