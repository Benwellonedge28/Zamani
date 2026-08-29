//! Zamani Quantum Memory — Device / GPU Memory Abstraction
//!
//! Production-grade, provider-neutral device-memory contracts for the
//! `quantum::memory` subsystem.
//!
//! # Architectural responsibility
//!
//! This module owns the *contract* for accelerator/device memory. It does not
//! implement CUDA, HIP, Metal, Vulkan, ROCm, SYCL, WGPU, FPGA drivers, QPU
//! network protocols, or vendor SDKs.
//!
//! The purpose is to give every later backend a stable, safe Rust interface:
//!
//! ```text
//!                         quantum::memory::gpu
//!                                  |
//!              +-------------------+-------------------+
//!              |                   |                   |
//!              v                   v                   v
//!          CPU host           accelerator          remote quantum
//!                            device memory            device
//!              |                   |                   |
//!              v                   v                   v
//!           host RAM        GPU/FPGA/TPU/etc.       QPU session
//! ```
//!
//! # Critical distinction
//!
//! A QPU normally does NOT expose general-purpose writable quantum memory to
//! the host. Therefore this module distinguishes:
//!
//! - `Accelerator` — programmable local accelerator memory;
//! - `QuantumProcessor` — remote/physical quantum execution device;
//! - `RemoteDevice` — remote execution/storage service;
//! - `Simulator` — accelerator-backed simulator;
//! - `Emulator` — device emulation;
//! - `Custom` — future provider-defined devices.
//!
//! A QPU may expose *classical buffers*, result buffers, calibration data,
//! execution metadata, or provider-native handles, but `gpu.rs` must never
//! claim that such a buffer is physical qubit state memory unless the provider
//! explicitly declares that capability.
//!
//! # Safety contract
//!
//! This file contains no `unsafe` code.
//!
//! Raw pointers, CUDA device pointers, Vulkan handles, Metal objects, HIP
//! pointers, FFI resources, and provider-specific native objects MUST NOT be
//! exposed through this API.
//!
//! Providers must wrap those resources in safe, opaque Rust types before
//! implementing the traits here.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable only
//! - no nightly features
//! - no `unsafe`
//!
//! # Dependency direction
//!
//! ```text
//! limits.rs -----------+
//! errors.rs -----------+----> gpu.rs
//! representation.rs ---+          |
//! address.rs ----------+          v
//! allocator.rs ---------------- device provider
//! coherence.rs ----------------- device synchronization
//! synchronization.rs ----------- device events
//! migration.rs ----------------- host/device movement
//! state_vector.rs -------------- accelerator execution
//! density_matrix.rs ----------- accelerator execution
//! tensor_network.rs ------------ accelerator execution
//! hardware/ -------------------- physical QPU provider
//! ```
//!
//! `gpu.rs` MUST NOT depend on:
//!
//! - routing;
//! - scheduling;
//! - algorithms;
//! - benchmarking;
//! - compiler/frontend;
//! - a specific quantum vendor;
//! - a specific GPU vendor.
//!
//! # Design principles
//!
//! 1. Device memory is explicitly owned.
//! 2. Device buffers are opaque.
//! 3. Device addresses are never exposed.
//! 4. Every allocation is checked before creation.
//! 5. Copy sizes are checked.
//! 6. Device identity is explicit.
//! 7. Memory location is explicit.
//! 8. Coherence is explicit.
//! 9. Synchronization is explicit.
//! 10. Host/device copies never silently synchronize.
//! 11. No global device state.
//! 12. No hidden allocation.
//! 13. No implicit vendor selection.
//! 14. No vendor-specific enum variants.
//! 15. QPU memory semantics are not confused with accelerator memory.
//! 16. Capability negotiation is explicit.
//! 17. Providers can reject unsupported operations.
//! 18. Provider errors are converted into `MemoryError`.
//! 19. Device resources are represented by stable opaque IDs.
//! 20. Device buffers cannot be constructed from arbitrary addresses.
//!
//! # Integration contract
//!
//! Later modules consume this file as follows:
//!
//! ```text
//! allocator.rs
//!     -> DeviceMemoryProvider
//!
//! state_vector.rs
//!     -> DeviceBuffer / DeviceMemoryProvider
//!
//! density_matrix.rs
//!     -> DeviceBuffer / DeviceMemoryProvider
//!
//! tensor_network.rs
//!     -> DeviceBuffer / DeviceMemoryProvider
//!
//! coherence.rs
//!     -> DeviceCoherence
//!
//! synchronization.rs
//!     -> DeviceEvent / DeviceStream
//!
//! migration.rs
//!     -> copy_to_device / copy_from_device
//!
//! hardware adapters
//!     -> DeviceKind::QuantumProcessor
//!        and/or BackendDeviceProvider
//!
//! benchmarking.rs
//!     -> DeviceInfo / DeviceCapabilities / DeviceMemoryStats
//!
//! diagnostics.rs
//!     -> DeviceMemoryStats
//! ```
//!
//! This means later files do not need to modify the fundamental device-memory
//! contract merely to add a new hardware vendor.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::time::Duration;

use super::errors::MemoryError;

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the device-memory contract.
pub const GPU_MEMORY_SCHEMA_ID: &str = "zamani.quantum.memory.device";

/// Semantic version of the device-memory contract.
///
/// This version changes only when the public semantic contract changes.
pub const GPU_MEMORY_SCHEMA_VERSION: u16 = 1;

/// Maximum provider/device name accepted by this module.
pub const MAX_DEVICE_NAME_LENGTH: usize = 256;

/// Maximum provider name accepted by this module.
pub const MAX_PROVIDER_NAME_LENGTH: usize = 256;

/// Maximum capability name accepted by this module.
pub const MAX_CAPABILITY_NAME_LENGTH: usize = 256;

/// Maximum number of devices a provider may return in one enumeration call.
pub const MAX_ENUMERATED_DEVICES: usize = 65_536;

/// Maximum number of capabilities in one device description.
pub const MAX_DEVICE_CAPABILITIES: usize = 1_024;

// =============================================================================
// Opaque identifiers
// =============================================================================

/// Stable opaque identifier for a device.
///
/// This is intentionally not a pointer and must not be interpreted as a
/// physical address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeviceId(u64);

impl DeviceId {
    /// Creates a device identifier.
    ///
    /// `0` is reserved as invalid.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "device-{}", self.0)
    }
}

/// Stable opaque identifier for a provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeviceProviderId(u64);

impl DeviceProviderId {
    /// Creates a provider identifier.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DeviceProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provider-{}", self.0)
    }
}

/// Stable opaque identifier for a device-memory allocation.
///
/// This is a logical allocation identity, NOT a device address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeviceAllocationId(u64);

impl DeviceAllocationId {
    /// Creates an allocation identifier.
    ///
    /// `0` is reserved as invalid.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DeviceAllocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "device-allocation-{}", self.0)
    }
}

/// Stable opaque identifier for a device stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeviceStreamId(u64);

impl DeviceStreamId {
    /// Creates a stream identifier.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Stable opaque identifier for a device event/fence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeviceEventId(u64);

impl DeviceEventId {
    /// Creates an event identifier.
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Device taxonomy
// =============================================================================

/// Provider-neutral classification of a compute device.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeviceKind {
    /// General-purpose accelerator such as a GPU.
    Accelerator,

    /// Graphics/compute processor.
    Gpu,

    /// FPGA or reconfigurable accelerator.
    Fpga,

    /// Tensor/AI accelerator.
    TensorAccelerator,

    /// CPU-side accelerator exposed through a device provider.
    CpuAccelerator,

    /// Quantum processor / QPU.
    QuantumProcessor,

    /// Classical simulator representing a quantum device.
    QuantumSimulator,

    /// Hardware emulator.
    QuantumEmulator,

    /// Remote accelerator or execution device.
    Remote,

    /// Provider-defined future hardware.
    Custom,
}

impl DeviceKind {
    /// Returns whether this kind can expose ordinary device-addressable
    /// classical memory.
    pub const fn supports_addressable_memory(self) -> bool {
        match self {
            Self::QuantumProcessor => false,
            Self::QuantumSimulator
            | Self::QuantumEmulator
            | Self::Accelerator
            | Self::Gpu
            | Self::Fpga
            | Self::TensorAccelerator
            | Self::CpuAccelerator
            | Self::Remote
            | Self::Custom => true,
        }
    }

    /// Returns whether the device represents physical quantum execution.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::QuantumProcessor)
    }
}

/// Type of memory exposed by a device provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeviceMemoryKind {
    /// Ordinary device-local memory.
    DeviceLocal,

    /// Host memory pinned for device transfers.
    PinnedHost,

    /// Unified/shared host-device memory.
    Unified,

    /// Managed memory controlled by a provider/runtime.
    Managed,

    /// Device-local constant/read-only memory.
    Constant,

    /// Fast shared/workgroup memory.
    Shared,

    /// Remote memory owned by a provider.
    Remote,

    /// Classical result memory exposed by a QPU provider.
    QuantumClassicalResult,

    /// Provider-native opaque memory.
    BackendNative,
}

impl DeviceMemoryKind {
    /// Returns whether the memory can be directly addressed by the device
    /// provider.
    pub const fn is_device_addressable(self) -> bool {
        matches!(
            self,
            Self::DeviceLocal
                | Self::Unified
                | Self::Managed
                | Self::Constant
                | Self::Shared
                | Self::BackendNative
        )
    }

    /// Returns whether the memory is suitable for mutable state storage.
    pub const fn supports_mutation(self) -> bool {
        matches!(
            self,
            Self::DeviceLocal
                | Self::Unified
                | Self::Managed
                | Self::Remote
                | Self::BackendNative
        )
    }
}

// =============================================================================
// Memory usage semantics
// =============================================================================

/// Semantic purpose of a device allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeviceAllocationPurpose {
    /// Quantum state vector storage.
    StateVector,

    /// Density matrix storage.
    DensityMatrix,

    /// Stabilizer/tableau storage.
    Stabilizer,

    /// Sparse state storage.
    SparseState,

    /// Tensor-network storage.
    TensorNetwork,

    /// Temporary quantum kernel workspace.
    TemporaryWorkspace,

    /// Classical companion memory.
    ClassicalMemory,

    /// Measurement results.
    MeasurementResults,

    /// Circuit parameters.
    Parameters,

    /// QPU execution input.
    QuantumExecutionInput,

    /// QPU execution output.
    QuantumExecutionOutput,

    /// Backend/provider native data.
    BackendNative,

    /// General-purpose device allocation.
    General,
}

// =============================================================================
// Device capabilities
// =============================================================================

/// Device capability identifiers.
///
/// This enum is intentionally generic. Vendor-specific capabilities should be
/// expressed through `DeviceCapability::Custom` in provider metadata rather
/// than added to this core module.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeviceCapability {
    /// Device memory allocation.
    DeviceMemory,

    /// Host-to-device transfers.
    HostToDevice,

    /// Device-to-host transfers.
    DeviceToHost,

    /// Device-to-device transfers.
    DeviceToDevice,

    /// Peer-to-peer transfers.
    PeerToPeer,

    /// Asynchronous streams.
    AsyncStreams,

    /// Events/fences.
    Events,

    /// Device-side synchronization.
    Synchronization,

    /// Unified memory.
    UnifiedMemory,

    /// Pinned host memory.
    PinnedHostMemory,

    /// Native complex arithmetic.
    ComplexArithmetic,

    /// f32 arithmetic.
    F32,

    /// f64 arithmetic.
    F64,

    /// Fast tensor operations.
    TensorOperations,

    /// Double precision atomic operations.
    F64Atomics,

    /// Quantum state-vector execution.
    QuantumStateVector,

    /// Density-matrix execution.
    QuantumDensityMatrix,

    /// Stabilizer execution.
    QuantumStabilizer,

    /// Tensor-network execution.
    QuantumTensorNetwork,

    /// Physical QPU execution.
    QuantumProcessor,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Dynamic circuits.
    DynamicCircuits,

    /// Classical feed-forward.
    ClassicalFeedback,

    /// Remote execution.
    RemoteExecution,

    /// Provider-native memory.
    BackendNativeMemory,
}

impl fmt::Display for DeviceCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::DeviceMemory => "device-memory",
            Self::HostToDevice => "host-to-device",
            Self::DeviceToHost => "device-to-host",
            Self::DeviceToDevice => "device-to-device",
            Self::PeerToPeer => "peer-to-peer",
            Self::AsyncStreams => "async-streams",
            Self::Events => "events",
            Self::Synchronization => "synchronization",
            Self::UnifiedMemory => "unified-memory",
            Self::PinnedHostMemory => "pinned-host-memory",
            Self::ComplexArithmetic => "complex-arithmetic",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::TensorOperations => "tensor-operations",
            Self::F64Atomics => "f64-atomics",
            Self::QuantumStateVector => "quantum-state-vector",
            Self::QuantumDensityMatrix => "quantum-density-matrix",
            Self::QuantumStabilizer => "quantum-stabilizer",
            Self::QuantumTensorNetwork => "quantum-tensor-network",
            Self::QuantumProcessor => "quantum-processor",
            Self::MidCircuitMeasurement => "mid-circuit-measurement",
            Self::DynamicCircuits => "dynamic-circuits",
            Self::ClassicalFeedback => "classical-feedback",
            Self::RemoteExecution => "remote-execution",
            Self::BackendNativeMemory => "backend-native-memory",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Device description
// =============================================================================

/// Static information describing one device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceInfo {
    device_id: DeviceId,
    provider_id: DeviceProviderId,
    name: String,
    provider_name: String,
    kind: DeviceKind,
    total_memory_bytes: u64,
    available_memory_bytes: u64,
    capabilities: Vec<DeviceCapability>,
}

impl DeviceInfo {
    /// Creates a validated device description.
    pub fn new(
        device_id: DeviceId,
        provider_id: DeviceProviderId,
        name: impl Into<String>,
        provider_name: impl Into<String>,
        kind: DeviceKind,
        total_memory_bytes: u64,
        available_memory_bytes: u64,
        capabilities: Vec<DeviceCapability>,
    ) -> Result<Self, MemoryError> {
        let name = validate_name(name.into(), MAX_DEVICE_NAME_LENGTH, "device name")?;
        let provider_name = validate_name(
            provider_name.into(),
            MAX_PROVIDER_NAME_LENGTH,
            "provider name",
        )?;

        if total_memory_bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                argument: "total_memory_bytes".to_string(),
                context: None,
            });
        }

        if available_memory_bytes > total_memory_bytes {
            return Err(MemoryError::InvalidArgument {
                argument: "available_memory_bytes".to_string(),
                context: None,
            });
        }

        if capabilities.len() > MAX_DEVICE_CAPABILITIES {
            return Err(MemoryError::InvalidArgument {
                argument: "capabilities".to_string(),
                context: None,
            });
        }

        if kind.is_qpu() && kind.supports_addressable_memory() {
            return Err(MemoryError::InvariantViolation {
                reason: "quantum processor cannot simultaneously declare ordinary \
                         addressable device memory through the core GPU contract"
                    .to_string(),
            });
        }

        Ok(Self {
            device_id,
            provider_id,
            name,
            provider_name,
            kind,
            total_memory_bytes,
            available_memory_bytes,
            capabilities,
        })
    }

    /// Device identifier.
    pub const fn device_id(&self) -> DeviceId {
        self.device_id
    }

    /// Provider identifier.
    pub const fn provider_id(&self) -> DeviceProviderId {
        self.provider_id
    }

    /// Device name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Provider name.
    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    /// Device kind.
    pub const fn kind(&self) -> DeviceKind {
        self.kind
    }

    /// Total device memory.
    pub const fn total_memory_bytes(&self) -> u64 {
        self.total_memory_bytes
    }

    /// Currently available device memory reported by the provider.
    pub const fn available_memory_bytes(&self) -> u64 {
        self.available_memory_bytes
    }

    /// Device capabilities.
    pub fn capabilities(&self) -> &[DeviceCapability] {
        &self.capabilities
    }

    /// Tests whether a capability is available.
    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.iter().any(|item| *item == capability)
    }

    /// Returns whether this device can provide ordinary addressable memory.
    pub const fn supports_addressable_memory(&self) -> bool {
        self.kind.supports_addressable_memory()
    }

    /// Returns whether this device is a physical QPU.
    pub const fn is_qpu(&self) -> bool {
        self.kind.is_qpu()
    }
}

// =============================================================================
// Allocation descriptor
// =============================================================================

/// Immutable description of a requested device-memory allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceAllocationRequest {
    bytes: u64,
    alignment: u64,
    memory_kind: DeviceMemoryKind,
    purpose: DeviceAllocationPurpose,
    zeroed: bool,
}

impl DeviceAllocationRequest {
    /// Creates a validated allocation request.
    pub fn new(
        bytes: u64,
        alignment: u64,
        memory_kind: DeviceMemoryKind,
        purpose: DeviceAllocationPurpose,
        zeroed: bool,
    ) -> Result<Self, MemoryError> {
        if bytes == 0 {
            return Err(MemoryError::InvalidArgument {
                argument: "bytes".to_string(),
                context: None,
            });
        }

        if alignment == 0 || !alignment.is_power_of_two() {
            return Err(MemoryError::InvalidArgument {
                argument: "alignment".to_string(),
                context: None,
            });
        }

        if !memory_kind.supports_mutation() && zeroed {
            return Err(MemoryError::UnsupportedOperation {
                operation: "zero-read-only-device-memory".to_string(),
            });
        }

        Ok(Self {
            bytes,
            alignment,
            memory_kind,
            purpose,
            zeroed,
        })
    }

    /// Number of requested bytes.
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Requested alignment.
    pub const fn alignment(&self) -> u64 {
        self.alignment
    }

    /// Requested memory kind.
    pub const fn memory_kind(&self) -> DeviceMemoryKind {
        self.memory_kind
    }

    /// Intended allocation purpose.
    pub const fn purpose(&self) -> DeviceAllocationPurpose {
        self.purpose
    }

    /// Whether the allocation must be initialized to zero.
    pub const fn zeroed(&self) -> bool {
        self.zeroed
    }
}

// =============================================================================
// Device buffer
// =============================================================================

/// Safe opaque device-memory allocation.
///
/// This object intentionally contains no raw pointer and exposes no device
/// address. The provider retains ownership of the underlying native resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceBuffer {
    allocation_id: DeviceAllocationId,
    device_id: DeviceId,
    bytes: u64,
    memory_kind: DeviceMemoryKind,
    purpose: DeviceAllocationPurpose,
}

impl DeviceBuffer {
    /// Constructs a buffer descriptor returned by a trusted provider.
    ///
    /// Provider implementations should only call this after successfully
    /// allocating the corresponding native resource.
    pub fn from_allocation(
        allocation_id: DeviceAllocationId,
        device_id: DeviceId,
        request: DeviceAllocationRequest,
    ) -> Self {
        Self {
            allocation_id,
            device_id,
            bytes: request.bytes(),
            memory_kind: request.memory_kind(),
            purpose: request.purpose(),
        }
    }

    /// Allocation identity.
    pub const fn allocation_id(&self) -> DeviceAllocationId {
        self.allocation_id
    }

    /// Device identity.
    pub const fn device_id(&self) -> DeviceId {
        self.device_id
    }

    /// Number of bytes.
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Memory kind.
    pub const fn memory_kind(&self) -> DeviceMemoryKind {
        self.memory_kind
    }

    /// Allocation purpose.
    pub const fn purpose(&self) -> DeviceAllocationPurpose {
        self.purpose
    }
}

// =============================================================================
// Transfer descriptors
// =============================================================================

/// Direction of a host/device transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TransferDirection {
    /// Host -> device.
    HostToDevice,

    /// Device -> host.
    DeviceToHost,

    /// Device -> device.
    DeviceToDevice,

    /// Peer device -> peer device.
    PeerToPeer,
}

/// A validated byte range used by transfer operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceByteRange {
    offset: u64,
    length: u64,
}

impl DeviceByteRange {
    /// Creates a validated byte range.
    pub const fn new(offset: u64, length: u64) -> Option<Self> {
        if length == 0 {
            return None;
        }

        if offset.checked_add(length).is_none() {
            return None;
        }

        Some(Self { offset, length })
    }

    /// Offset in bytes.
    pub const fn offset(self) -> u64 {
        self.offset
    }

    /// Length in bytes.
    pub const fn length(self) -> u64 {
        self.length
    }

    /// End-exclusive offset.
    pub const fn end(self) -> u64 {
        self.offset + self.length
    }
}

/// Transfer request.
///
/// The request does not contain raw host pointers. Host data is supplied as a
/// Rust slice to the provider operation, preventing raw-address leakage through
/// this API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceTransferRequest {
    direction: TransferDirection,
    range: DeviceByteRange,
    stream: Option<DeviceStreamId>,
}

impl DeviceTransferRequest {
    /// Creates a validated transfer request.
    pub const fn new(
        direction: TransferDirection,
        range: DeviceByteRange,
        stream: Option<DeviceStreamId>,
    ) -> Self {
        Self {
            direction,
            range,
            stream,
        }
    }

    /// Transfer direction.
    pub const fn direction(self) -> TransferDirection {
        self.direction
    }

    /// Byte range.
    pub const fn range(self) -> DeviceByteRange {
        self.range
    }

    /// Optional stream.
    pub const fn stream(self) -> Option<DeviceStreamId> {
        self.stream
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Device synchronization mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeviceSyncMode {
    /// Wait until all operations on the selected device have completed.
    Device,

    /// Wait until operations on a particular stream have completed.
    Stream,

    /// Wait for a particular event.
    Event,
}

/// Result of a non-blocking device operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceOperation {
    /// Operation completed immediately.
    Complete,

    /// Operation has been submitted and can be tracked with this event.
    Submitted(DeviceEventId),
}

// =============================================================================
// Statistics
// =============================================================================

/// Runtime device-memory statistics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeviceMemoryStats {
    /// Total provider-reported memory.
    pub total_bytes: u64,

    /// Currently available provider-reported memory.
    pub available_bytes: u64,

    /// Bytes currently allocated through Zamani.
    pub allocated_bytes: u64,

    /// Peak bytes allocated through Zamani.
    pub peak_allocated_bytes: u64,

    /// Number of active allocations.
    pub allocation_count: u64,

    /// Number of successful host-to-device transfers.
    pub host_to_device_transfers: u64,

    /// Number of successful device-to-host transfers.
    pub device_to_host_transfers: u64,

    /// Number of successful device-to-device transfers.
    pub device_to_device_transfers: u64,

    /// Number of synchronization operations.
    pub synchronization_count: u64,
}

impl DeviceMemoryStats {
    /// Validates basic accounting invariants.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.available_bytes > self.total_bytes {
            return Err(MemoryError::InvariantViolation {
                reason: "device memory statistics report more available memory \
                         than total memory"
                    .to_string(),
            });
        }

        if self.allocated_bytes > self.total_bytes {
            return Err(MemoryError::InvariantViolation {
                reason: "device memory statistics report more allocated memory \
                         than total memory"
                    .to_string(),
            });
        }

        if self.peak_allocated_bytes < self.allocated_bytes {
            return Err(MemoryError::InvariantViolation {
                reason: "device peak allocation is lower than current allocation"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Provider trait
// =============================================================================

/// Provider-neutral device-memory interface.
///
/// This is the principal integration point for:
///
/// - CUDA;
/// - HIP/ROCm;
/// - Metal;
/// - Vulkan;
/// - WGPU;
/// - SYCL;
/// - FPGA runtimes;
/// - custom accelerators;
/// - quantum simulators;
/// - remote quantum providers.
///
/// Provider implementations MUST NOT expose raw pointers through this trait.
///
/// Implementations may internally use FFI or native handles, but those details
/// belong to the provider crate/adapter, not this core memory module.
pub trait DeviceMemoryProvider: Send + Sync {
    /// Returns provider metadata.
    fn provider_id(&self) -> DeviceProviderId;

    /// Enumerates devices exposed by the provider.
    fn devices(&self) -> Result<Vec<DeviceInfo>, MemoryError>;

    /// Returns one device description.
    fn device(&self, device_id: DeviceId) -> Result<DeviceInfo, MemoryError>;

    /// Allocates device memory.
    fn allocate(
        &self,
        device_id: DeviceId,
        request: DeviceAllocationRequest,
    ) -> Result<DeviceBuffer, MemoryError>;

    /// Releases device memory.
    ///
    /// Implementations must make this operation idempotent with respect to
    /// their own internal bookkeeping, or return a deterministic error for an
    /// already-released allocation.
    fn deallocate(
        &self,
        buffer: DeviceBuffer,
    ) -> Result<(), MemoryError>;

    /// Copies bytes from host memory into a device buffer.
    ///
    /// The supplied host slice is the only host memory access exposed by the
    /// core contract.
    fn copy_to_device(
        &self,
        destination: &DeviceBuffer,
        destination_offset: u64,
        source: &[u8],
        stream: Option<DeviceStreamId>,
    ) -> Result<DeviceOperation, MemoryError>;

    /// Copies bytes from a device buffer into host memory.
    ///
    /// The destination slice must have sufficient capacity for the requested
    /// range.
    fn copy_from_device(
        &self,
        source: &DeviceBuffer,
        source_offset: u64,
        destination: &mut [u8],
        stream: Option<DeviceStreamId>,
    ) -> Result<DeviceOperation, MemoryError>;

    /// Copies bytes between two device buffers.
    fn copy_device_to_device(
        &self,
        destination: &DeviceBuffer,
        destination_offset: u64,
        source: &DeviceBuffer,
        source_offset: u64,
        bytes: u64,
        stream: Option<DeviceStreamId>,
    ) -> Result<DeviceOperation, MemoryError>;

    /// Creates a device stream.
    fn create_stream(
        &self,
        device_id: DeviceId,
    ) -> Result<DeviceStreamId, MemoryError>;

    /// Destroys a device stream.
    fn destroy_stream(
        &self,
        device_id: DeviceId,
        stream: DeviceStreamId,
    ) -> Result<(), MemoryError>;

    /// Creates a synchronization event.
    fn create_event(
        &self,
        device_id: DeviceId,
    ) -> Result<DeviceEventId, MemoryError>;

    /// Destroys a synchronization event.
    fn destroy_event(
        &self,
        device_id: DeviceId,
        event: DeviceEventId,
    ) -> Result<(), MemoryError>;

    /// Records an event on a stream.
    fn record_event(
        &self,
        device_id: DeviceId,
        stream: DeviceStreamId,
        event: DeviceEventId,
    ) -> Result<(), MemoryError>;

    /// Waits for an event.
    fn wait_event(
        &self,
        device_id: DeviceId,
        event: DeviceEventId,
        timeout: Option<Duration>,
    ) -> Result<(), MemoryError>;

    /// Synchronizes a selected device scope.
    fn synchronize(
        &self,
        device_id: DeviceId,
        mode: DeviceSyncMode,
        stream: Option<DeviceStreamId>,
        event: Option<DeviceEventId>,
        timeout: Option<Duration>,
    ) -> Result<(), MemoryError>;

    /// Returns memory statistics.
    fn memory_stats(
        &self,
        device_id: DeviceId,
    ) -> Result<DeviceMemoryStats, MemoryError>;
}

// =============================================================================
// Provider-independent validation helpers
// =============================================================================

/// Validates a device-memory allocation against provider-reported capacity.
///
/// This is intentionally independent of `MemoryLimits`. `allocator.rs` can
/// first enforce the Zamani policy and then use this function to enforce the
/// actual device's currently available capacity.
///
/// This two-level validation is required:
///
/// ```text
/// MemoryLimits
///      ↓
/// Zamani policy
///      ↓
/// Device capacity
///      ↓
/// provider allocation
/// ```
pub fn validate_device_allocation(
    device: &DeviceInfo,
    request: DeviceAllocationRequest,
) -> Result<(), MemoryError> {
    if !device.supports_addressable_memory() {
        return Err(MemoryError::UnsupportedStorageLocation {
            location: format!("{:?}", device.kind()),
        });
    }

    if request.bytes() > device.available_memory_bytes() {
        return Err(MemoryError::AllocationFailed {
            requested_bytes: request.bytes(),
            available_bytes: device.available_memory_bytes(),
        });
    }

    if matches!(
        request.memory_kind(),
        DeviceMemoryKind::QuantumClassicalResult
    ) && !device.is_qpu()
    {
        return Err(MemoryError::InvalidArgument {
            argument: "memory_kind".to_string(),
            context: None,
        });
    }

    Ok(())
}

/// Validates that a device buffer range is inside the allocation.
pub fn validate_buffer_range(
    buffer: &DeviceBuffer,
    offset: u64,
    length: u64,
) -> Result<(), MemoryError> {
    let end = offset.checked_add(length).ok_or_else(|| {
        MemoryError::ArithmeticOverflow {
            operation: "device buffer range end".to_string(),
        }
    })?;

    if end > buffer.bytes() {
        return Err(MemoryError::OutOfBounds {
            index: end,
            length: buffer.bytes(),
            resource: "device-buffer".to_string(),
        });
    }

    Ok(())
}

/// Validates a host/device transfer.
///
/// This prevents providers from receiving a request larger than either the
/// device allocation or the supplied host slice.
pub fn validate_host_transfer(
    buffer: &DeviceBuffer,
    offset: u64,
    host_bytes: usize,
) -> Result<u64, MemoryError> {
    let host_bytes_u64 = u64::try_from(host_bytes).map_err(|error| {
        MemoryError::IntegerConversion { source: error }
    })?;

    validate_buffer_range(buffer, offset, host_bytes_u64)?;

    Ok(host_bytes_u64)
}

/// Validates a device-to-device copy.
pub fn validate_device_copy(
    destination: &DeviceBuffer,
    destination_offset: u64,
    source: &DeviceBuffer,
    source_offset: u64,
    bytes: u64,
) -> Result<(), MemoryError> {
    validate_buffer_range(destination, destination_offset, bytes)?;
    validate_buffer_range(source, source_offset, bytes)?;

    if destination.device_id() != source.device_id() {
        return Ok(());
    }

    if destination.allocation_id() == source.allocation_id() {
        let destination_end = destination_offset.checked_add(bytes).ok_or_else(|| {
            MemoryError::ArithmeticOverflow {
                operation: "destination copy range".to_string(),
            }
        })?;

        let source_end = source_offset.checked_add(bytes).ok_or_else(|| {
            MemoryError::ArithmeticOverflow {
                operation: "source copy range".to_string(),
            }
        })?;

        let overlaps =
            destination_offset < source_end && source_offset < destination_end;

        if overlaps && destination_offset != source_offset {
            return Err(MemoryError::AliasingViolation {
                reason: "overlapping device-to-device copy within the same \
                         allocation is not permitted by the core contract"
                    .to_string(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Device capability negotiation
// =============================================================================

/// A requested collection of device capabilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceCapabilityRequest {
    capabilities: Vec<DeviceCapability>,
}

impl DeviceCapabilityRequest {
    /// Creates an empty capability request.
    pub const fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    /// Adds a required capability.
    pub fn require(
        &mut self,
        capability: DeviceCapability,
    ) -> Result<(), MemoryError> {
        if !self.capabilities.contains(&capability) {
            if self.capabilities.len() >= MAX_DEVICE_CAPABILITIES {
                return Err(MemoryError::InvalidArgument {
                    argument: "capabilities".to_string(),
                    context: None,
                });
            }

            self.capabilities.push(capability);
        }

        Ok(())
    }

    /// Returns requested capabilities.
    pub fn capabilities(&self) -> &[DeviceCapability] {
        &self.capabilities
    }

    /// Validates the request against a device.
    pub fn validate(&self, device: &DeviceInfo) -> Result<(), MemoryError> {
        for capability in &self.capabilities {
            if !device.has_capability(*capability) {
                return Err(MemoryError::BackendCapabilityUnavailable {
                    backend: device.provider_name().to_string(),
                    capability: capability.to_string(),
                });
            }
        }

        Ok(())
    }
}

impl Default for DeviceCapabilityRequest {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// QPU boundary
// =============================================================================

/// Describes what memory semantics a physical quantum processor exposes.
///
/// This exists specifically to prevent GPU memory and QPU memory from being
/// conflated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuantumProcessorMemoryModel {
    /// Whether the provider exposes writable classical input memory.
    pub classical_input_memory: bool,

    /// Whether the provider exposes readable classical result memory.
    pub classical_result_memory: bool,

    /// Whether the provider exposes provider-native opaque memory.
    pub backend_native_memory: bool,

    /// Whether the provider exposes user-addressable physical quantum state
    /// memory.
    ///
    /// This should almost always be `false`.
    pub addressable_quantum_state_memory: bool,
}

impl QuantumProcessorMemoryModel {
    /// Conservative model for a normal remote QPU.
    pub const fn standard_qpu() -> Self {
        Self {
            classical_input_memory: true,
            classical_result_memory: true,
            backend_native_memory: true,
            addressable_quantum_state_memory: false,
        }
    }

    /// Validates the model.
    pub const fn validate(self) -> bool {
        if self.addressable_quantum_state_memory
            && !self.backend_native_memory
        {
            return false;
        }

        true
    }
}

/// Generic QPU memory/session provider.
///
/// Physical QPUs should normally implement this interface instead of pretending
/// to implement `DeviceMemoryProvider`.
///
/// It allows hardware adapters to expose:
///
/// - classical execution input;
/// - classical result buffers;
/// - provider-native opaque state/session resources;
/// - synchronization;
/// - execution lifecycle.
///
/// The actual quantum state remains owned by the physical device.
pub trait QuantumDeviceMemoryProvider: Send + Sync {
    /// Returns the device identity.
    fn device_id(&self) -> DeviceId;

    /// Returns the memory model exposed by the QPU.
    fn memory_model(&self) -> QuantumProcessorMemoryModel;

    /// Writes classical execution input.
    fn write_classical_input(
        &self,
        data: &[u8],
    ) -> Result<DeviceOperation, MemoryError>;

    /// Reads classical execution results.
    fn read_classical_results(
        &self,
        destination: &mut [u8],
    ) -> Result<DeviceOperation, MemoryError>;

    /// Creates an opaque provider-native execution resource.
    fn create_execution_resource(
        &self,
        purpose: DeviceAllocationPurpose,
    ) -> Result<DeviceAllocationId, MemoryError>;

    /// Releases an opaque provider-native execution resource.
    fn release_execution_resource(
        &self,
        allocation: DeviceAllocationId,
    ) -> Result<(), MemoryError>;

    /// Synchronizes with the physical device.
    fn synchronize(
        &self,
        timeout: Option<Duration>,
    ) -> Result<(), MemoryError>;
}

// =============================================================================
// Provider registry contract
// =============================================================================

/// Read-only snapshot of registered device providers.
///
/// The registry itself belongs in the higher-level allocator/runtime layer.
/// This type only defines the information needed to select a provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceProviderInfo {
    provider_id: DeviceProviderId,
    name: String,
    devices: Vec<DeviceId>,
}

impl DeviceProviderInfo {
    /// Creates provider metadata.
    pub fn new(
        provider_id: DeviceProviderId,
        name: impl Into<String>,
        devices: Vec<DeviceId>,
    ) -> Result<Self, MemoryError> {
        let name = validate_name(
            name.into(),
            MAX_PROVIDER_NAME_LENGTH,
            "provider name",
        )?;

        if devices.len() > MAX_ENUMERATED_DEVICES {
            return Err(MemoryError::InvalidArgument {
                argument: "devices".to_string(),
                context: None,
            });
        }

        Ok(Self {
            provider_id,
            name,
            devices,
        })
    }

    /// Provider identity.
    pub const fn provider_id(&self) -> DeviceProviderId {
        self.provider_id
    }

    /// Provider name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Devices exposed by this provider.
    pub fn devices(&self) -> &[DeviceId] {
        &self.devices
    }
}

// =============================================================================
// Selection policy
// =============================================================================

/// Policy used by higher layers to select a device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceSelectionPolicy {
    /// Require an exact device.
    Exact(DeviceId),

    /// Prefer a device with sufficient memory.
    SufficientMemory,

    /// Prefer the device with the most available memory.
    MostAvailableMemory,

    /// Prefer accelerator devices.
    Accelerator,

    /// Prefer a quantum simulator.
    QuantumSimulator,

    /// Prefer a physical QPU.
    QuantumProcessor,

    /// Provider/runtime decides.
    ProviderDefault,
}

/// Validates a device against a selection policy.
pub fn matches_selection_policy(
    device: &DeviceInfo,
    policy: DeviceSelectionPolicy,
) -> bool {
    match policy {
        DeviceSelectionPolicy::Exact(id) => device.device_id() == id,
        DeviceSelectionPolicy::SufficientMemory
        | DeviceSelectionPolicy::MostAvailableMemory => true,
        DeviceSelectionPolicy::Accelerator => matches!(
            device.kind(),
            DeviceKind::Accelerator
                | DeviceKind::Gpu
                | DeviceKind::Fpga
                | DeviceKind::TensorAccelerator
                | DeviceKind::CpuAccelerator
        ),
        DeviceSelectionPolicy::QuantumSimulator => {
            device.kind() == DeviceKind::QuantumSimulator
        }
        DeviceSelectionPolicy::QuantumProcessor => {
            device.kind() == DeviceKind::QuantumProcessor
        }
        DeviceSelectionPolicy::ProviderDefault => true,
    }
}

// =============================================================================
// Utility validation
// =============================================================================

fn validate_name(
    value: String,
    maximum: usize,
    field: &'static str,
) -> Result<String, MemoryError> {
    if value.is_empty() {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_string(),
            context: None,
        });
    }

    if value.len() > maximum {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_string(),
            context: None,
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_string(),
            context: None,
        });
    }

    Ok(value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn device() -> DeviceInfo {
        DeviceInfo::new(
            DeviceId::new(1).expect("valid device id"),
            DeviceProviderId::new(1).expect("valid provider id"),
            "test-device",
            "test-provider",
            DeviceKind::Gpu,
            16 * 1024 * 1024,
            8 * 1024 * 1024,
            vec![
                DeviceCapability::DeviceMemory,
                DeviceCapability::HostToDevice,
                DeviceCapability::DeviceToHost,
                DeviceCapability::DeviceToDevice,
                DeviceCapability::AsyncStreams,
                DeviceCapability::Events,
                DeviceCapability::Synchronization,
                DeviceCapability::F32,
                DeviceCapability::F64,
                DeviceCapability::QuantumStateVector,
            ],
        )
        .expect("valid device")
    }

    #[test]
    fn device_id_zero_is_invalid() {
        assert!(DeviceId::new(0).is_none());
        assert!(DeviceId::new(1).is_some());
    }

    #[test]
    fn allocation_request_rejects_zero_bytes() {
        let result = DeviceAllocationRequest::new(
            0,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn allocation_request_rejects_invalid_alignment() {
        let result = DeviceAllocationRequest::new(
            64,
            3,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn allocation_request_accepts_power_of_two_alignment() {
        let result = DeviceAllocationRequest::new(
            64,
            64,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::StateVector,
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn device_capacity_is_checked() {
        let info = device();

        let request = DeviceAllocationRequest::new(
            16 * 1024 * 1024,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        )
        .expect("valid request");

        assert!(validate_device_allocation(&info, request).is_err());
    }

    #[test]
    fn device_capacity_accepts_valid_request() {
        let info = device();

        let request = DeviceAllocationRequest::new(
            1024,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::StateVector,
            true,
        )
        .expect("valid request");

        assert!(validate_device_allocation(&info, request).is_ok());
    }

    #[test]
    fn buffer_range_is_checked() {
        let request = DeviceAllocationRequest::new(
            1024,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        )
        .expect("valid request");

        let buffer = DeviceBuffer::from_allocation(
            DeviceAllocationId::new(1).expect("valid allocation id"),
            DeviceId::new(1).expect("valid device id"),
            request,
        );

        assert!(validate_buffer_range(&buffer, 0, 1024).is_ok());
        assert!(validate_buffer_range(&buffer, 1, 1024).is_err());
    }

    #[test]
    fn host_transfer_is_checked() {
        let request = DeviceAllocationRequest::new(
            32,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        )
        .expect("valid request");

        let buffer = DeviceBuffer::from_allocation(
            DeviceAllocationId::new(1).expect("valid allocation id"),
            DeviceId::new(1).expect("valid device id"),
            request,
        );

        let source = [0u8; 16];

        assert_eq!(
            validate_host_transfer(&buffer, 0, source.len())
                .expect("valid transfer"),
            16
        );

        assert!(validate_host_transfer(&buffer, 20, source.len()).is_err());
    }

    #[test]
    fn overlapping_same_buffer_copy_is_rejected() {
        let request = DeviceAllocationRequest::new(
            128,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        )
        .expect("valid request");

        let buffer = DeviceBuffer::from_allocation(
            DeviceAllocationId::new(1).expect("valid allocation id"),
            DeviceId::new(1).expect("valid device id"),
            request,
        );

        assert!(validate_device_copy(&buffer, 0, &buffer, 8, 32).is_err());
    }

    #[test]
    fn identical_same_buffer_copy_is_allowed() {
        let request = DeviceAllocationRequest::new(
            128,
            8,
            DeviceMemoryKind::DeviceLocal,
            DeviceAllocationPurpose::General,
            false,
        )
        .expect("valid request");

        let buffer = DeviceBuffer::from_allocation(
            DeviceAllocationId::new(1).expect("valid allocation id"),
            DeviceId::new(1).expect("valid device id"),
            request,
        );

        assert!(validate_device_copy(&buffer, 0, &buffer, 0, 32).is_ok());
    }

    #[test]
    fn capability_request_validates() {
        let info = device();

        let mut request = DeviceCapabilityRequest::new();

        request
            .require(DeviceCapability::F64)
            .expect("capability");

        request
            .require(DeviceCapability::QuantumStateVector)
            .expect("capability");

        assert!(request.validate(&info).is_ok());
    }

    #[test]
    fn capability_request_rejects_missing_capability() {
        let info = device();

        let mut request = DeviceCapabilityRequest::new();

        request
            .require(DeviceCapability::QuantumDensityMatrix)
            .expect("capability");

        assert!(request.validate(&info).is_err());
    }

    #[test]
    fn qpu_memory_model_does_not_claim_physical_state_memory() {
        let model = QuantumProcessorMemoryModel::standard_qpu();

        assert!(!model.addressable_quantum_state_memory);
        assert!(model.classical_input_memory);
        assert!(model.classical_result_memory);
        assert!(model.backend_native_memory);
        assert!(model.validate());
    }

    #[test]
    fn qpu_is_not_an_addressable_gpu_memory_device() {
        assert!(!DeviceKind::QuantumProcessor.supports_addressable_memory());
        assert!(DeviceKind::QuantumProcessor.is_qpu());
        assert!(DeviceKind::Gpu.supports_addressable_memory());
        assert!(!DeviceKind::Gpu.is_qpu());
    }

    #[test]
    fn device_stats_validate() {
        let stats = DeviceMemoryStats {
            total_bytes: 100,
            available_bytes: 50,
            allocated_bytes: 50,
            peak_allocated_bytes: 75,
            allocation_count: 2,
            ..DeviceMemoryStats::default()
        };

        assert!(stats.validate().is_ok());
    }

    #[test]
    fn invalid_device_stats_are_rejected() {
        let stats = DeviceMemoryStats {
            total_bytes: 100,
            available_bytes: 101,
            allocated_bytes: 50,
            peak_allocated_bytes: 75,
            allocation_count: 2,
            ..DeviceMemoryStats::default()
        };

        assert!(stats.validate().is_err());
    }

    #[test]
    fn device_selection_policy_matches() {
        let info = device();

        assert!(matches_selection_policy(
            &info,
            DeviceSelectionPolicy::Accelerator
        ));

        assert!(matches_selection_policy(
            &info,
            DeviceSelectionPolicy::Exact(info.device_id())
        ));

        assert!(!matches_selection_policy(
            &info,
            DeviceSelectionPolicy::QuantumProcessor
        ));
    }
}