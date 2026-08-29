//! Zamani Quantum Memory — QPU / Quantum Hardware Memory Contract
//!
//! Production-grade, provider-neutral resource and memory boundary for
//! physical quantum processing units and quantum hardware.
//!
//! # Architectural responsibility
//!
//! This module defines the memory/resource contract between
//! `quantum::memory` and physical quantum hardware.
//!
//! It deliberately does NOT:
//!
//! - execute quantum programs;
//! - submit network requests;
//! - authenticate with providers;
//! - store credentials;
//! - define provider SDK types;
//! - perform routing;
//! - perform transpilation;
//! - schedule jobs;
//! - implement QPU calibration;
//! - implement quantum gates;
//! - implement a simulator;
//! - own quantum IR;
//! - implement measurement mathematics;
//! - implement QEC algorithms;
//! - depend on a specific vendor;
//! - assume that physical quantum hardware has conventional RAM;
//! - expose raw pointers;
//! - require `unsafe`.
//!
//! # Important semantic distinction
//!
//! A physical QPU is NOT generally equivalent to a classical memory device.
//!
//! A QPU may expose:
//!
//! - physical qubits;
//! - logical qubits;
//! - qudits;
//! - photonic modes;
//! - atoms;
//! - oscillators;
//! - annealing variables;
//! - couplers;
//! - measurement channels;
//! - classical result buffers;
//! - parameter buffers;
//! - pulse/control resources;
//! - provider-native opaque resources.
//!
//! Therefore this module models a **quantum hardware resource boundary**,
//! rather than pretending that every QPU exposes byte-addressable quantum RAM.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Quantum IR
//!      |
//!      +------------------------------+
//!      |                              |
//!      v                              v
//! optimization                    QEC
//!      |                              |
//!      +--------------+---------------+
//!                     |
//!                     v
//!              routing / scheduling
//!                     |
//!                     v
//!              hardware execution
//!                     |
//!          +----------+----------+
//!          |                     |
//!          v                     v
//!    QuantumBackend       QpuMemoryProvider
//!          |                     |
//!          |                     v
//!          |              quantum resources
//!          |              classical buffers
//!          |              result buffers
//!          |              control buffers
//!          |              provider handles
//!          |                     |
//!          +----------+----------+
//!                     |
//!                     v
//!                    QPU
//! ```
//!
//! `quantum::hardware` owns execution semantics.
//!
//! `quantum::memory::qpu` owns the provider-neutral memory/resource contract.
//!
//! This dependency direction is intentional:
//!
//! ```text
//! memory::qpu  <---- hardware adapters
//! hardware     ----> memory::qpu
//! ```
//!
//! `memory::qpu` MUST NOT depend on `quantum::hardware`.
//!
//! This prevents a circular dependency and allows the memory subsystem to be
//! independently tested and used by:
//!
//! - real QPUs;
//! - simulators;
//! - emulators;
//! - hybrid systems;
//! - distributed quantum systems;
//! - future hardware paradigms.
//!
//! # Hardware coverage
//!
//! The model is intentionally extensible enough for:
//!
//! - superconducting qubits;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - Rydberg systems;
//! - photonic systems;
//! - silicon/spin qubits;
//! - semiconductor quantum dots;
//! - NV-center systems;
//! - topological systems;
//! - Majorana-based systems;
//! - cat-qubit systems;
//! - bosonic/continuous-variable systems;
//! - quantum annealers;
//! - analog Hamiltonian quantum processors;
//! - hybrid quantum systems;
//! - logical/fault-tolerant QPUs;
//! - future architectures not yet known.
//!
//! The technology enum is deliberately independent from the execution
//! paradigm. A new provider or technology MUST NOT require modification of
//! the core allocation contract merely to be recognized.
//!
//! # Rust compatibility
//!
//! Required compatibility:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external crates required;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! The public API never exposes:
//!
//! - raw pointers;
//! - raw device addresses;
//! - FFI handles;
//! - provider SDK objects;
//! - mutable global state.
//!
//! Provider adapters may internally translate these contracts into their
//! provider APIs, but that implementation belongs outside this file.
//!
//! # Integration contract
//!
//! `types.rs`
//!     May later provide canonical `QubitId`, `MemoryId`, `AllocationId` and
//!     related identifiers. Until that module is established, this file owns
//!     its own strongly typed hardware-resource identifiers so that it is
//!     independently complete.
//!
//! `representation.rs`
//!     May map simulator state representations to hardware result/resource
//!     classes. This file must remain usable without that mapping.
//!
//! `allocator.rs`
//!     Provides general host/device memory allocation. This file models QPU
//!     resource allocation and therefore must not duplicate the generic
//!     allocator.
//!
//! `address.rs`
//!     May later provide a common abstraction for device addresses. This file
//!     intentionally uses opaque identifiers rather than exposing addresses.
//!
//! `coherence.rs`
//!     May later consume `QpuMemoryRegion` and `QpuMemoryLocation` to model
//!     synchronization between host-side state and hardware-side resources.
//!
//! `measurement.rs`
//!     Consumes result-buffer descriptors and measurement-buffer metadata.
//!
//! `snapshot.rs` / `checkpoint.rs`
//!     May record QPU resource identity and allocation metadata, but MUST NOT
//!     serialize provider secrets or ephemeral opaque handles as portable
//!     state.
//!
//! `hardware/backend.rs`
//!     Owns `QuantumBackend` and backend capabilities. Hardware adapters may
//!     translate backend capabilities into `QpuCapabilities`.
//!
//! `hardware/backend_trait.rs`
//!     Owns actual program submission/execution. It may use this module to
//!     reserve or release hardware resources before execution.
//!
//! `hardware/adapters/*`
//!     IBM, IonQ, IQM, Quantinuum, QuEra, Rigetti, AWS Braket, generic and
//!     future adapters implement provider-specific translation outside this
//!     module.
//!
//! `routing`
//!     Uses resource topology and physical-resource identities but MUST NOT
//!     manipulate QPU allocations directly.
//!
//! `scheduling`
//!     May consume reservation requirements and availability information.
//!
//! `benchmarking`
//!     May consume diagnostics and capacity metadata. This module MUST NOT
//!     depend on benchmarking.
//!
//! `Danga`
//!     May expose these abstractions to package/build/execution workflows but
//!     must not introduce a second QPU memory API.
//!
//! # Stability rule
//!
//! Once this file is accepted, adding a new provider must NOT require changing
//! this contract. Provider-specific capabilities belong in:
//!
//! - capability identifiers;
//! - provider adapters;
//! - provider metadata;
//! - opaque provider handles.
//!
//! The core resource model should remain stable.
//!
//! # Design rule
//!
//! The most important rule in this file is:
//!
//! ```text
//! physical quantum resource != classical byte-addressable memory
//! ```
//!
//! QPU memory may therefore represent a reservation of physical quantum
//! resources while classical/result/control memory is represented using
//! explicit buffer kinds.
//!
//! This distinction allows the same API to support both conventional
//! gate-model QPUs and non-gate-model quantum hardware.
//!
//! # External hardware considerations
//!
//! Current quantum hardware platforms expose different resource models,
//! including device topology, native instructions, measurement behavior,
//! dynamic circuits, analog programs, and device-specific limits.
//! A provider-neutral memory contract therefore cannot assume one universal
//! gate set or memory layout.
//!
//! Amazon Braket, for example, exposes both gate-based QPUs and analog
//! Hamiltonian devices, and device properties include topology and native
//! operations. IBM likewise exposes dynamic-circuit capabilities and backend
//! targets. This module deliberately represents those differences as
//! capabilities/resource kinds instead of embedding provider-specific logic.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the QPU memory contract.
pub const QPU_MEMORY_SCHEMA_ID: &str = "zamani.quantum.memory.qpu";

/// Semantic version of the QPU memory contract.
pub const QPU_MEMORY_SCHEMA_VERSION: u16 = 1;

/// Maximum identifier length.
pub const MAX_IDENTIFIER_LENGTH: usize = 512;

/// Maximum provider/technology identifier length.
pub const MAX_PROVIDER_IDENTIFIER_LENGTH: usize = 512;

/// Maximum capability identifier length.
pub const MAX_CAPABILITY_IDENTIFIER_LENGTH: usize = 256;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum number of resource identifiers in one allocation request.
pub const MAX_RESOURCE_IDENTIFIERS: usize = 1_000_000;

/// Maximum number of allocation regions.
pub const MAX_MEMORY_REGIONS: usize = 65_536;

/// Maximum byte-buffer allocation request.
pub const MAX_BUFFER_BYTES: u64 = 1 << 40; // 1 TiB

/// Maximum quantum-resource count represented by one request.
pub const MAX_QUANTUM_RESOURCES: u64 = 1_000_000_000;

/// Maximum classical bits represented by one request.
pub const MAX_CLASSICAL_BITS: u64 = 1_000_000_000;

/// Maximum shots represented by one request.
pub const MAX_SHOTS: u64 = 1_000_000_000_000;

// =============================================================================
// Validation
// =============================================================================

fn validate_identifier(
    name: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), QpuMemoryError> {
    if value.is_empty() {
        return Err(QpuMemoryError::InvalidIdentifier {
            field: name,
            reason: "identifier must not be empty",
        });
    }

    if value.len() > maximum {
        return Err(QpuMemoryError::IdentifierTooLong {
            field: name,
            maximum,
            actual: value.len(),
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(QpuMemoryError::InvalidIdentifier {
            field: name,
            reason: "identifier contains a control character",
        });
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<(), QpuMemoryError> {
    if metadata.len() > MAX_METADATA_ENTRIES {
        return Err(QpuMemoryError::MetadataLimitExceeded {
            maximum: MAX_METADATA_ENTRIES,
        });
    }

    for (key, value) in metadata {
        if key.is_empty() {
            return Err(QpuMemoryError::InvalidMetadata {
                reason: "metadata key must not be empty",
            });
        }

        if key.len() > MAX_METADATA_KEY_LENGTH {
            return Err(QpuMemoryError::InvalidMetadata {
                reason: "metadata key exceeds maximum length",
            });
        }

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(QpuMemoryError::InvalidMetadata {
                reason: "metadata value exceeds maximum length",
            });
        }

        if key.chars().any(|character| character.is_control())
            || value.chars().any(|character| character.is_control())
        {
            return Err(QpuMemoryError::InvalidMetadata {
                reason: "metadata contains a control character",
            });
        }

        if looks_like_secret_key(key) {
            return Err(QpuMemoryError::SecretMaterialRejected);
        }
    }

    Ok(())
}

fn looks_like_secret_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();

    normalized.contains("password")
        || normalized.contains("passwd")
        || normalized.contains("secret")
        || normalized.contains("token")
        || normalized.contains("api_key")
        || normalized.contains("apikey")
        || normalized.contains("private_key")
        || normalized.contains("authorization")
        || normalized.contains("cookie")
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the provider-neutral QPU memory contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QpuMemoryError {
    /// A required identifier was invalid.
    InvalidIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Reason for rejection.
        reason: &'static str,
    },

    /// Identifier exceeded its maximum permitted size.
    IdentifierTooLong {
        /// Field containing the identifier.
        field: &'static str,

        /// Maximum permitted length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// Numeric value exceeded a contract limit.
    LimitExceeded {
        /// Resource being limited.
        resource: &'static str,

        /// Maximum permitted value.
        maximum: u64,

        /// Requested value.
        requested: u64,
    },

    /// Metadata exceeded a contract limit.
    MetadataLimitExceeded {
        /// Maximum number of metadata entries.
        maximum: usize,
    },

    /// Metadata was invalid.
    InvalidMetadata {
        /// Reason for rejection.
        reason: &'static str,
    },

    /// Potential credential/secret material was detected.
    SecretMaterialRejected,

    /// Resource identifiers exceeded the supported request size.
    TooManyResources {
        /// Maximum resource count.
        maximum: usize,
    },

    /// Too many regions were supplied.
    TooManyRegions {
        /// Maximum region count.
        maximum: usize,
    },

    /// A requested resource kind is unsupported.
    UnsupportedResourceKind {
        /// Resource kind that cannot be represented.
        kind: &'static str,
    },

    /// An allocation was not found.
    AllocationNotFound,

    /// A region was not found.
    RegionNotFound,

    /// A reservation is no longer valid.
    ReservationInvalid,

    /// A reservation is already released.
    AlreadyReleased,

    /// A provider cannot satisfy the request.
    InsufficientCapacity,

    /// The provider rejected an operation.
    ProviderRejected {
        /// Provider-neutral reason.
        reason: String,
    },

    /// The provider is currently unavailable.
    ProviderUnavailable,

    /// The provider returned an invalid handle.
    InvalidProviderHandle,

    /// A release operation failed.
    ReleaseFailed {
        /// Provider-neutral reason.
        reason: String,
    },

    /// Synchronization is unavailable.
    SynchronizationUnavailable,

    /// The requested operation requires a provider feature that is absent.
    CapabilityUnavailable {
        /// Stable capability identifier.
        capability: String,
    },

    /// A requested operation is incompatible with the resource.
    IncompatibleRequest {
        /// Provider-neutral reason.
        reason: String,
    },
}

impl fmt::Display for QpuMemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::IdentifierTooLong {
                field,
                maximum,
                actual,
            } => write!(
                formatter,
                "{field} exceeds maximum length {maximum}: {actual}"
            ),
            Self::LimitExceeded {
                resource,
                maximum,
                requested,
            } => write!(
                formatter,
                "{resource} limit exceeded: requested {requested}, maximum {maximum}"
            ),
            Self::MetadataLimitExceeded { maximum } => write!(
                formatter,
                "metadata entry limit exceeded: maximum {maximum}"
            ),
            Self::InvalidMetadata { reason } => {
                write!(formatter, "invalid metadata: {reason}")
            }
            Self::SecretMaterialRejected => {
                formatter.write_str("potential secret material was rejected")
            }
            Self::TooManyResources { maximum } => write!(
                formatter,
                "too many quantum resources: maximum {maximum}"
            ),
            Self::TooManyRegions { maximum } => {
                write!(formatter, "too many memory regions: maximum {maximum}")
            }
            Self::UnsupportedResourceKind { kind } => {
                write!(formatter, "unsupported resource kind: {kind}")
            }
            Self::AllocationNotFound => {
                formatter.write_str("QPU allocation was not found")
            }
            Self::RegionNotFound => {
                formatter.write_str("QPU memory region was not found")
            }
            Self::ReservationInvalid => {
                formatter.write_str("QPU reservation is invalid")
            }
            Self::AlreadyReleased => {
                formatter.write_str("QPU allocation was already released")
            }
            Self::InsufficientCapacity => {
                formatter.write_str("QPU has insufficient capacity")
            }
            Self::ProviderRejected { reason } => {
                write!(formatter, "provider rejected request: {reason}")
            }
            Self::ProviderUnavailable => {
                formatter.write_str("QPU provider is unavailable")
            }
            Self::InvalidProviderHandle => {
                formatter.write_str("provider returned an invalid handle")
            }
            Self::ReleaseFailed { reason } => {
                write!(formatter, "QPU release failed: {reason}")
            }
            Self::SynchronizationUnavailable => {
                formatter.write_str("QPU synchronization is unavailable")
            }
            Self::CapabilityUnavailable { capability } => {
                write!(formatter, "QPU capability unavailable: {capability}")
            }
            Self::IncompatibleRequest { reason } => {
                write!(formatter, "incompatible QPU memory request: {reason}")
            }
        }
    }
}

impl std::error::Error for QpuMemoryError {}

// =============================================================================
// Strong identifiers
// =============================================================================

/// Provider-neutral QPU device identifier.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QpuDeviceId(String);

impl QpuDeviceId {
    /// Creates a validated device identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, QpuMemoryError> {
        let value = value.into();

        validate_identifier(
            "qpu_device_id",
            &value,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for QpuDeviceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("QpuDeviceId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for QpuDeviceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Provider-neutral allocation identifier.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QpuAllocationId(String);

impl QpuAllocationId {
    /// Creates a validated allocation identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, QpuMemoryError> {
        let value = value.into();

        validate_identifier(
            "qpu_allocation_id",
            &value,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for QpuAllocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("QpuAllocationId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for QpuAllocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Provider-neutral hardware-resource identifier.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QpuResourceId(String);

impl QpuResourceId {
    /// Creates a validated resource identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, QpuMemoryError> {
        let value = value.into();

        validate_identifier(
            "qpu_resource_id",
            &value,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for QpuResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("QpuResourceId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for QpuResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Provider-neutral memory-region identifier.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QpuRegionId(String);

impl QpuRegionId {
    /// Creates a validated region identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, QpuMemoryError> {
        let value = value.into();

        validate_identifier(
            "qpu_region_id",
            &value,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for QpuRegionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("QpuRegionId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for QpuRegionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque provider-owned allocation handle.
///
/// This is intentionally represented as a string rather than a pointer or
/// provider SDK object. Provider adapters may encode an ARN, UUID, task
/// resource name, device handle, session identifier, or another opaque
/// provider reference.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QpuProviderHandle(String);

impl QpuProviderHandle {
    /// Creates a validated opaque provider handle.
    pub fn new(value: impl Into<String>) -> Result<Self, QpuMemoryError> {
        let value = value.into();

        validate_identifier(
            "provider_handle",
            &value,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the opaque handle.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for QpuProviderHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("QpuProviderHandle(<opaque>)")
    }
}

// =============================================================================
// Quantum technology
// =============================================================================

/// Physical quantum technology.
///
/// This enum is intentionally descriptive rather than prescriptive. New
/// hardware technologies should normally be represented by `Other` rather
/// than requiring a breaking change to this contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumTechnology {
    /// Superconducting circuit qubits.
    Superconducting,

    /// Trapped-ion qubits.
    TrappedIon,

    /// Neutral atoms.
    NeutralAtom,

    /// Rydberg/programmable atom arrays.
    RydbergAtom,

    /// Photonic quantum systems.
    Photonic,

    /// Silicon spin systems.
    SiliconSpin,

    /// Quantum-dot/semiconductor systems.
    QuantumDot,

    /// NV-center or related defect-center systems.
    DefectCenter,

    /// Topological quantum systems.
    Topological,

    /// Majorana-based systems.
    Majorana,

    /// Bosonic/cat-qubit systems.
    Bosonic,

    /// Continuous-variable quantum systems.
    ContinuousVariable,

    /// Annealing hardware.
    Annealing,

    /// Analog Hamiltonian quantum hardware.
    AnalogHamiltonian,

    /// Hybrid quantum hardware.
    Hybrid,

    /// Logical/fault-tolerant quantum hardware.
    FaultTolerant,

    /// Technology not yet represented by the standard variants.
    Other(String),
}

impl QuantumTechnology {
    /// Stable identifier for serialization and diagnostics.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::RydbergAtom => "rydberg_atom",
            Self::Photonic => "photonic",
            Self::SiliconSpin => "silicon_spin",
            Self::QuantumDot => "quantum_dot",
            Self::DefectCenter => "defect_center",
            Self::Topological => "topological",
            Self::Majorana => "majorana",
            Self::Bosonic => "bosonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::Annealing => "annealing",
            Self::AnalogHamiltonian => "analog_hamiltonian",
            Self::Hybrid => "hybrid",
            Self::FaultTolerant => "fault_tolerant",
            Self::Other(value) => value.as_str(),
        }
    }
}

// =============================================================================
// Execution paradigm
// =============================================================================

/// Quantum hardware execution paradigm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuParadigm {
    /// Discrete gate-based execution.
    GateBased,

    /// Analog Hamiltonian evolution.
    AnalogHamiltonian,

    /// Quantum annealing / optimization hardware.
    Annealing,

    /// Measurement-based quantum computing.
    MeasurementBased,

    /// Continuous-variable execution.
    ContinuousVariable,

    /// Pulse-native execution.
    Pulse,

    /// Hybrid gate/pulse execution.
    Hybrid,

    /// Logical/fault-tolerant execution.
    FaultTolerant,

    /// Hardware-defined paradigm outside the standard set.
    Other,
}

impl QpuParadigm {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateBased => "gate_based",
            Self::AnalogHamiltonian => "analog_hamiltonian",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::ContinuousVariable => "continuous_variable",
            Self::Pulse => "pulse",
            Self::Hybrid => "hybrid",
            Self::FaultTolerant => "fault_tolerant",
            Self::Other => "other",
        }
    }
}

// =============================================================================
// Quantum resource kind
// =============================================================================

/// Physical/logical resource represented by a QPU allocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumResourceKind {
    /// Physical qubit.
    PhysicalQubit,

    /// Logical qubit.
    LogicalQubit,

    /// Qudit.
    Qudit,

    /// Photonic mode.
    PhotonicMode,

    /// Atomic degree of freedom.
    AtomicMode,

    /// Bosonic/oscillator mode.
    BosonicMode,

    /// Annealing variable.
    AnnealingVariable,

    /// Coupler/interconnect resource.
    Coupler,

    /// Measurement channel.
    MeasurementChannel,

    /// Control channel.
    ControlChannel,

    /// Provider-defined quantum resource.
    Other(String),
}

impl QuantumResourceKind {
    /// Stable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::PhysicalQubit => "physical_qubit",
            Self::LogicalQubit => "logical_qubit",
            Self::Qudit => "qudit",
            Self::PhotonicMode => "photonic_mode",
            Self::AtomicMode => "atomic_mode",
            Self::BosonicMode => "bosonic_mode",
            Self::AnnealingVariable => "annealing_variable",
            Self::Coupler => "coupler",
            Self::MeasurementChannel => "measurement_channel",
            Self::ControlChannel => "control_channel",
            Self::Other(value) => value.as_str(),
        }
    }

    /// Returns whether this is a quantum state-bearing resource.
    pub fn is_quantum_state_resource(&self) -> bool {
        matches!(
            self,
            Self::PhysicalQubit
                | Self::LogicalQubit
                | Self::Qudit
                | Self::PhotonicMode
                | Self::AtomicMode
                | Self::BosonicMode
                | Self::AnnealingVariable
        )
    }
}

// =============================================================================
// Memory region kinds
// =============================================================================

/// Kind of memory/resource region associated with a QPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuMemoryKind {
    /// Reservation of quantum hardware resources.
    QuantumResource,

    /// Classical input/parameter buffer.
    ClassicalInput,

    /// Classical measurement/result buffer.
    MeasurementResult,

    /// Classical register used for dynamic control.
    ClassicalControl,

    /// Pulse/control waveform buffer.
    ControlWaveform,

    /// Analog Hamiltonian program data.
    AnalogProgram,

    /// Annealing/QUBO/Ising program data.
    AnnealingProgram,

    /// Provider-native opaque buffer.
    ProviderNative,

    /// Diagnostic/calibration metadata.
    Metadata,
}

impl QpuMemoryKind {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuantumResource => "quantum_resource",
            Self::ClassicalInput => "classical_input",
            Self::MeasurementResult => "measurement_result",
            Self::ClassicalControl => "classical_control",
            Self::ControlWaveform => "control_waveform",
            Self::AnalogProgram => "analog_program",
            Self::AnnealingProgram => "annealing_program",
            Self::ProviderNative => "provider_native",
            Self::Metadata => "metadata",
        }
    }

    /// Returns true when the region represents conventional bytes.
    pub const fn is_byte_buffer(self) -> bool {
        !matches!(self, Self::QuantumResource)
    }
}

// =============================================================================
// Memory location
// =============================================================================

/// Location/ownership model for a QPU memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuMemoryLocation {
    /// Resource is physically resident on the QPU.
    Device,

    /// Resource is hosted by provider infrastructure.
    Provider,

    /// Resource is mirrored on the Zamani host.
    Host,

    /// Resource exists in both host and provider/device domains.
    Shared,

    /// Resource is distributed across multiple hardware nodes.
    Distributed,

    /// Location is opaque/provider-defined.
    Remote,
}

impl QpuMemoryLocation {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Provider => "provider",
            Self::Host => "host",
            Self::Shared => "shared",
            Self::Distributed => "distributed",
            Self::Remote => "remote",
        }
    }
}

// =============================================================================
// Access semantics
// =============================================================================

/// Access semantics for a QPU memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuAccessMode {
    /// Read-only after allocation.
    ReadOnly,

    /// Host/provider may write.
    WriteOnly,

    /// Read and write.
    ReadWrite,

    /// Resource may be consumed by execution but is not directly readable.
    ExecuteOnly,

    /// Resource is write-once and then immutable.
    ImmutableAfterWrite,
}

impl QpuAccessMode {
    /// Returns whether the region may be read directly.
    pub const fn readable(self) -> bool {
        matches!(
            self,
            Self::ReadOnly | Self::ReadWrite | Self::ImmutableAfterWrite
        )
    }

    /// Returns whether the region may be written directly.
    pub const fn writable(self) -> bool {
        matches!(
            self,
            Self::WriteOnly | Self::ReadWrite | Self::ImmutableAfterWrite
        )
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Synchronization state for a QPU memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuSyncState {
    /// Host and device/provider state agree.
    Synchronized,

    /// Host-side representation is newer.
    HostDirty,

    /// Device/provider representation is newer.
    DeviceDirty,

    /// Resource has no meaningful synchronization state.
    NotApplicable,

    /// Provider cannot expose synchronization semantics.
    Opaque,
}

impl QpuSyncState {
    /// Returns whether the state is synchronized.
    pub const fn is_synchronized(self) -> bool {
        matches!(self, Self::Synchronized | Self::NotApplicable)
    }
}

// =============================================================================
// Capability identifiers
// =============================================================================

/// Stable provider-neutral QPU capability identifiers.
///
/// String-based extensions allow new hardware capabilities without changing
/// this source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuCapability {
    /// Physical qubits are addressable.
    PhysicalQubits,

    /// Logical qubits are addressable.
    LogicalQubits,

    /// Qudits are supported.
    Qudits,

    /// Photonic modes are supported.
    PhotonicModes,

    /// Dynamic circuits are supported.
    DynamicCircuits,

    /// Mid-circuit measurement is supported.
    MidCircuitMeasurement,

    /// Classical feed-forward is supported.
    ClassicalFeedForward,

    /// Reset is supported.
    Reset,

    /// Pulse programming is supported.
    PulseControl,

    /// Analog Hamiltonian programming is supported.
    AnalogHamiltonian,

    /// Annealing programming is supported.
    Annealing,

    /// Fault-tolerant/logical execution is supported.
    FaultTolerance,

    /// Provider supports device-side result storage.
    DeviceResultStorage,

    /// Provider supports host/device result transfer.
    ResultTransfer,

    /// Provider supports batch execution.
    BatchExecution,

    /// Provider supports concurrent jobs.
    ConcurrentJobs,

    /// Provider supports cancellation.
    Cancellation,

    /// Provider exposes topology.
    Topology,

    /// Provider exposes native instruction information.
    NativeInstructions,

    /// Provider exposes calibration data.
    Calibration,

    /// Provider exposes timing information.
    Timing,

    /// Provider exposes resource reservation.
    ResourceReservation,

    /// Provider supports reusable sessions.
    Sessions,

    /// Provider-specific capability.
    Other(String),
}

impl QpuCapability {
    /// Stable capability identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::PhysicalQubits => "physical_qubits",
            Self::LogicalQubits => "logical_qubits",
            Self::Qudits => "qudits",
            Self::PhotonicModes => "photonic_modes",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::ClassicalFeedForward => "classical_feed_forward",
            Self::Reset => "reset",
            Self::PulseControl => "pulse_control",
            Self::AnalogHamiltonian => "analog_hamiltonian",
            Self::Annealing => "annealing",
            Self::FaultTolerance => "fault_tolerance",
            Self::DeviceResultStorage => "device_result_storage",
            Self::ResultTransfer => "result_transfer",
            Self::BatchExecution => "batch_execution",
            Self::ConcurrentJobs => "concurrent_jobs",
            Self::Cancellation => "cancellation",
            Self::Topology => "topology",
            Self::NativeInstructions => "native_instructions",
            Self::Calibration => "calibration",
            Self::Timing => "timing",
            Self::ResourceReservation => "resource_reservation",
            Self::Sessions => "sessions",
            Self::Other(value) => value.as_str(),
        }
    }
}

// =============================================================================
// Device description
// =============================================================================

/// Immutable provider-neutral description of a QPU memory/resource domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuDeviceDescriptor {
    /// Stable Zamani-facing device identity.
    pub device_id: QpuDeviceId,

    /// Provider identity.
    pub provider_id: String,

    /// Human-readable device name.
    pub name: String,

    /// Physical quantum technology.
    pub technology: QuantumTechnology,

    /// Execution paradigms supported by this device.
    pub paradigms: BTreeSet<QpuParadigm>,

    /// Maximum physical quantum-resource count.
    pub physical_resource_count: u64,

    /// Maximum logical quantum-resource count, if known.
    pub logical_resource_count: Option<u64>,

    /// Maximum classical memory exposed through this contract, if known.
    pub classical_memory_bytes: Option<u64>,

    /// Maximum result memory exposed through this contract, if known.
    pub result_memory_bytes: Option<u64>,

    /// Provider capabilities.
    pub capabilities: BTreeSet<QpuCapability>,

    /// Stable native instruction names.
    pub native_instructions: BTreeSet<String>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuDeviceDescriptor {
    /// Constructs a validated descriptor.
    pub fn new(
        device_id: QpuDeviceId,
        provider_id: impl Into<String>,
        name: impl Into<String>,
        technology: QuantumTechnology,
        physical_resource_count: u64,
    ) -> Result<Self, QpuMemoryError> {
        if physical_resource_count > MAX_QUANTUM_RESOURCES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "physical quantum resources",
                maximum: MAX_QUANTUM_RESOURCES,
                requested: physical_resource_count,
            });
        }

        let provider_id = provider_id.into();
        let name = name.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_IDENTIFIER_LENGTH,
        )?;

        validate_identifier("device_name", &name, MAX_IDENTIFIER_LENGTH)?;

        Ok(Self {
            device_id,
            provider_id,
            name,
            technology,
            paradigms: BTreeSet::new(),
            physical_resource_count,
            logical_resource_count: None,
            classical_memory_bytes: None,
            result_memory_bytes: None,
            capabilities: BTreeSet::new(),
            native_instructions: BTreeSet::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Adds an execution paradigm.
    pub fn with_paradigm(mut self, paradigm: QpuParadigm) -> Self {
        self.paradigms.insert(paradigm);
        self
    }

    /// Adds a capability.
    pub fn with_capability(mut self, capability: QpuCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds a native instruction.
    pub fn with_native_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        let instruction = instruction.into();

        validate_identifier(
            "native_instruction",
            &instruction,
            MAX_CAPABILITY_IDENTIFIER_LENGTH,
        )?;

        self.native_instructions
            .insert(instruction.to_ascii_lowercase());

        Ok(self)
    }

    /// Adds provider-neutral metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        self.metadata.insert(key.into(), value.into());
        validate_metadata(&self.metadata)?;
        Ok(self)
    }

    /// Sets the logical-resource count.
    pub fn with_logical_resource_count(
        mut self,
        count: u64,
    ) -> Result<Self, QpuMemoryError> {
        if count > MAX_QUANTUM_RESOURCES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "logical quantum resources",
                maximum: MAX_QUANTUM_RESOURCES,
                requested: count,
            });
        }

        self.logical_resource_count = Some(count);
        Ok(self)
    }

    /// Sets the classical memory capacity.
    pub fn with_classical_memory_bytes(
        mut self,
        bytes: u64,
    ) -> Result<Self, QpuMemoryError> {
        if bytes > MAX_BUFFER_BYTES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "classical memory bytes",
                maximum: MAX_BUFFER_BYTES,
                requested: bytes,
            });
        }

        self.classical_memory_bytes = Some(bytes);
        Ok(self)
    }

    /// Sets the result memory capacity.
    pub fn with_result_memory_bytes(
        mut self,
        bytes: u64,
    ) -> Result<Self, QpuMemoryError> {
        if bytes > MAX_BUFFER_BYTES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "result memory bytes",
                maximum: MAX_BUFFER_BYTES,
                requested: bytes,
            });
        }

        self.result_memory_bytes = Some(bytes);
        Ok(self)
    }

    /// Returns whether a capability is advertised.
    pub fn supports(&self, capability: &QpuCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Returns whether a paradigm is advertised.
    pub fn supports_paradigm(&self, paradigm: QpuParadigm) -> bool {
        self.paradigms.contains(&paradigm)
    }
}

// =============================================================================
// Resource specification
// =============================================================================

/// A requested quantum resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuResourceSpec {
    /// Kind of resource.
    pub kind: QuantumResourceKind,

    /// Number of resources required.
    pub count: u64,

    /// Whether exact physical identities are required.
    pub exact_identity_required: bool,

    /// Requested provider resource IDs.
    pub preferred_ids: Vec<QpuResourceId>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuResourceSpec {
    /// Creates a resource specification.
    pub fn new(
        kind: QuantumResourceKind,
        count: u64,
    ) -> Result<Self, QpuMemoryError> {
        if count > MAX_QUANTUM_RESOURCES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "quantum resources",
                maximum: MAX_QUANTUM_RESOURCES,
                requested: count,
            });
        }

        Ok(Self {
            kind,
            count,
            exact_identity_required: false,
            preferred_ids: Vec::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Requires exact resource identities.
    pub fn require_exact_identity(mut self) -> Self {
        self.exact_identity_required = true;
        self
    }

    /// Adds a preferred physical/logical resource.
    pub fn prefer_resource(
        mut self,
        resource: QpuResourceId,
    ) -> Result<Self, QpuMemoryError> {
        if self.preferred_ids.len() >= MAX_RESOURCE_IDENTIFIERS {
            return Err(QpuMemoryError::TooManyResources {
                maximum: MAX_RESOURCE_IDENTIFIERS,
            });
        }

        self.preferred_ids.push(resource);
        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        self.metadata.insert(key.into(), value.into());
        validate_metadata(&self.metadata)?;
        Ok(self)
    }
}

// =============================================================================
// Buffer specification
// =============================================================================

/// Classical/control/result buffer request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuBufferSpec {
    /// Buffer kind.
    pub kind: QpuMemoryKind,

    /// Number of bytes requested.
    pub bytes: u64,

    /// Access mode.
    pub access: QpuAccessMode,

    /// Optional alignment requirement.
    pub alignment_bytes: Option<u64>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuBufferSpec {
    /// Creates a byte-buffer specification.
    pub fn new(
        kind: QpuMemoryKind,
        bytes: u64,
        access: QpuAccessMode,
    ) -> Result<Self, QpuMemoryError> {
        if kind == QpuMemoryKind::QuantumResource {
            return Err(QpuMemoryError::IncompatibleRequest {
                reason: "quantum resources must use QpuResourceSpec",
            });
        }

        if bytes > MAX_BUFFER_BYTES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "QPU buffer bytes",
                maximum: MAX_BUFFER_BYTES,
                requested: bytes,
            });
        }

        Ok(Self {
            kind,
            bytes,
            access,
            alignment_bytes: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Sets an alignment requirement.
    pub fn with_alignment(
        mut self,
        alignment_bytes: u64,
    ) -> Result<Self, QpuMemoryError> {
        if alignment_bytes == 0 || !alignment_bytes.is_power_of_two() {
            return Err(QpuMemoryError::IncompatibleRequest {
                reason: "buffer alignment must be a non-zero power of two",
            });
        }

        self.alignment_bytes = Some(alignment_bytes);
        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        self.metadata.insert(key.into(), value.into());
        validate_metadata(&self.metadata)?;
        Ok(self)
    }
}

// =============================================================================
// Allocation request
// =============================================================================

/// Complete provider-neutral QPU memory/resource allocation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuAllocationRequest {
    /// Requested quantum resources.
    pub quantum_resources: Vec<QpuResourceSpec>,

    /// Requested classical/control/result buffers.
    pub buffers: Vec<QpuBufferSpec>,

    /// Whether the reservation must be exclusive.
    pub exclusive: bool,

    /// Whether the allocation may be reused across multiple jobs.
    pub reusable: bool,

    /// Provider-neutral request metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuAllocationRequest {
    /// Creates an empty allocation request.
    pub fn new() -> Self {
        Self {
            quantum_resources: Vec::new(),
            buffers: Vec::new(),
            exclusive: true,
            reusable: false,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a quantum resource requirement.
    pub fn with_quantum_resources(
        mut self,
        resource: QpuResourceSpec,
    ) -> Result<Self, QpuMemoryError> {
        if self.quantum_resources.len() >= MAX_MEMORY_REGIONS {
            return Err(QpuMemoryError::TooManyRegions {
                maximum: MAX_MEMORY_REGIONS,
            });
        }

        self.quantum_resources.push(resource);
        Ok(self)
    }

    /// Adds a buffer requirement.
    pub fn with_buffer(
        mut self,
        buffer: QpuBufferSpec,
    ) -> Result<Self, QpuMemoryError> {
        if self.buffers.len() >= MAX_MEMORY_REGIONS {
            return Err(QpuMemoryError::TooManyRegions {
                maximum: MAX_MEMORY_REGIONS,
            });
        }

        self.buffers.push(buffer);
        Ok(self)
    }

    /// Requests shared rather than exclusive resources.
    pub fn shared(mut self) -> Self {
        self.exclusive = false;
        self
    }

    /// Requests a reusable reservation.
    pub fn reusable(mut self) -> Self {
        self.reusable = true;
        self
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        self.metadata.insert(key.into(), value.into());
        validate_metadata(&self.metadata)?;
        Ok(self)
    }

    /// Validates the complete request.
    pub fn validate(&self) -> Result<(), QpuMemoryError> {
        if self.quantum_resources.is_empty() && self.buffers.is_empty() {
            return Err(QpuMemoryError::IncompatibleRequest {
                reason: "allocation request contains no resources",
            });
        }

        validate_metadata(&self.metadata)?;

        for resource in &self.quantum_resources {
            validate_metadata(&resource.metadata)?;

            if resource.preferred_ids.len() > MAX_RESOURCE_IDENTIFIERS {
                return Err(QpuMemoryError::TooManyResources {
                    maximum: MAX_RESOURCE_IDENTIFIERS,
                });
            }
        }

        for buffer in &self.buffers {
            validate_metadata(&buffer.metadata)?;
        }

        Ok(())
    }

    /// Returns the total requested byte capacity.
    pub fn total_buffer_bytes(&self) -> Result<u64, QpuMemoryError> {
        self.buffers.iter().try_fold(0u64, |total, buffer| {
            total.checked_add(buffer.bytes).ok_or(
                QpuMemoryError::LimitExceeded {
                    resource: "total QPU buffer bytes",
                    maximum: MAX_BUFFER_BYTES,
                    requested: u64::MAX,
                },
            )
        })
    }

    /// Returns the total requested quantum resource count.
    pub fn total_quantum_resources(&self) -> Result<u64, QpuMemoryError> {
        self.quantum_resources
            .iter()
            .try_fold(0u64, |total, resource| {
                total.checked_add(resource.count).ok_or(
                    QpuMemoryError::LimitExceeded {
                        resource: "total quantum resources",
                        maximum: MAX_QUANTUM_RESOURCES,
                        requested: u64::MAX,
                    },
                )
            })
    }
}

impl Default for QpuAllocationRequest {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Allocated resource
// =============================================================================

/// A resource granted by a QPU memory provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuAllocatedResource {
    /// Resource identity.
    pub resource_id: QpuResourceId,

    /// Resource kind.
    pub kind: QuantumResourceKind,

    /// Provider location.
    pub location: QpuMemoryLocation,

    /// Provider-owned opaque handle, if one exists.
    pub provider_handle: Option<QpuProviderHandle>,
}

/// An allocated QPU memory region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuMemoryRegion {
    /// Region identifier.
    pub region_id: QpuRegionId,

    /// Region kind.
    pub kind: QpuMemoryKind,

    /// Storage location.
    pub location: QpuMemoryLocation,

    /// Access semantics.
    pub access: QpuAccessMode,

    /// Byte size, where applicable.
    pub bytes: Option<u64>,

    /// Allocated quantum resources, where applicable.
    pub quantum_resources: Vec<QpuAllocatedResource>,

    /// Synchronization state.
    pub synchronization: QpuSyncState,

    /// Provider-owned opaque handle.
    pub provider_handle: Option<QpuProviderHandle>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuMemoryRegion {
    /// Returns whether this is a quantum resource region.
    pub const fn is_quantum_resource(&self) -> bool {
        matches!(self.kind, QpuMemoryKind::QuantumResource)
    }

    /// Returns whether this is a byte buffer.
    pub const fn is_byte_buffer(&self) -> bool {
        self.kind.is_byte_buffer()
    }

    /// Returns the number of allocated quantum resources.
    pub fn quantum_resource_count(&self) -> usize {
        self.quantum_resources.len()
    }
}

// =============================================================================
// Allocation
// =============================================================================

/// Successful QPU memory/resource allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuAllocation {
    /// Allocation identity.
    pub id: QpuAllocationId,

    /// Device identity.
    pub device_id: QpuDeviceId,

    /// Allocated regions.
    pub regions: Vec<QpuMemoryRegion>,

    /// Whether this allocation is exclusive.
    pub exclusive: bool,

    /// Whether this allocation may be reused.
    pub reusable: bool,

    /// Provider-owned opaque allocation handle.
    pub provider_handle: Option<QpuProviderHandle>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl QpuAllocation {
    /// Finds a region by identifier.
    pub fn region(
        &self,
        region_id: &QpuRegionId,
    ) -> Result<&QpuMemoryRegion, QpuMemoryError> {
        self.regions
            .iter()
            .find(|region| &region.region_id == region_id)
            .ok_or(QpuMemoryError::RegionNotFound)
    }

    /// Returns all physical resource identifiers.
    pub fn quantum_resource_ids(&self) -> Vec<QpuResourceId> {
        self.regions
            .iter()
            .flat_map(|region| {
                region
                    .quantum_resources
                    .iter()
                    .map(|resource| resource.resource_id.clone())
            })
            .collect()
    }

    /// Returns the number of quantum resources.
    pub fn quantum_resource_count(&self) -> usize {
        self.regions
            .iter()
            .map(QpuMemoryRegion::quantum_resource_count)
            .sum()
    }
}

// =============================================================================
// Result buffer
// =============================================================================

/// Description of a measurement/result buffer.
///
/// This does not contain the actual result values. The actual result format is
/// owned by the execution/result subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuResultBuffer {
    /// Region containing results.
    pub region_id: QpuRegionId,

    /// Number of shots represented.
    pub shots: u64,

    /// Number of classical bits per result, when applicable.
    pub classical_bits_per_shot: Option<u64>,

    /// Provider-native result representation.
    pub representation: String,
}

impl QpuResultBuffer {
    /// Creates a result-buffer descriptor.
    pub fn new(
        region_id: QpuRegionId,
        shots: u64,
        representation: impl Into<String>,
    ) -> Result<Self, QpuMemoryError> {
        if shots > MAX_SHOTS {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "shots",
                maximum: MAX_SHOTS,
                requested: shots,
            });
        }

        let representation = representation.into();

        validate_identifier(
            "result_representation",
            &representation,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self {
            region_id,
            shots,
            classical_bits_per_shot: None,
            representation,
        })
    }

    /// Sets the number of classical bits per shot.
    pub fn with_classical_bits_per_shot(
        mut self,
        bits: u64,
    ) -> Result<Self, QpuMemoryError> {
        if bits > MAX_CLASSICAL_BITS {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "classical bits per shot",
                maximum: MAX_CLASSICAL_BITS,
                requested: bits,
            });
        }

        self.classical_bits_per_shot = Some(bits);
        Ok(self)
    }
}

// =============================================================================
// Resource availability
// =============================================================================

/// Provider-neutral availability report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuResourceAvailability {
    /// Device identity.
    pub device_id: QpuDeviceId,

    /// Total physical quantum resources.
    pub total_quantum_resources: u64,

    /// Currently available quantum resources.
    pub available_quantum_resources: u64,

    /// Total classical buffer capacity, when known.
    pub total_classical_bytes: Option<u64>,

    /// Currently available classical buffer capacity, when known.
    pub available_classical_bytes: Option<u64>,

    /// Total result capacity, when known.
    pub total_result_bytes: Option<u64>,

    /// Currently available result capacity, when known.
    pub available_result_bytes: Option<u64>,
}

impl QpuResourceAvailability {
    /// Returns whether the requested allocation fits the advertised capacity.
    pub fn can_satisfy(
        &self,
        request: &QpuAllocationRequest,
    ) -> Result<bool, QpuMemoryError> {
        request.validate()?;

        let quantum = request.total_quantum_resources()?;

        if quantum > self.available_quantum_resources {
            return Ok(false);
        }

        let bytes = request.total_buffer_bytes()?;

        if bytes == 0 {
            return Ok(true);
        }

        let available_classical = self
            .available_classical_bytes
            .unwrap_or(u64::MAX);

        let available_result = self.available_result_bytes.unwrap_or(u64::MAX);

        Ok(bytes <= available_classical.max(available_result))
    }
}

// =============================================================================
// Provider contract
// =============================================================================

/// Provider-neutral QPU memory/resource provider.
///
/// This trait is deliberately synchronous and contains no network or async
/// policy. Provider adapters can implement it using their own internal
/// transport strategy.
///
/// The trait is object-safe and can therefore be stored as:
///
/// ```text
/// Box<dyn QpuMemoryProvider>
/// Arc<dyn QpuMemoryProvider>
/// ```
///
/// The provider MUST NOT expose raw pointers or provider SDK types through
/// this trait.
pub trait QpuMemoryProvider: Send + Sync {
    /// Returns the immutable device descriptor.
    fn descriptor(&self) -> &QpuDeviceDescriptor;

    /// Returns current resource availability.
    ///
    /// This may be a cached provider report. The contract does not require a
    /// network request.
    fn availability(
        &self,
    ) -> Result<QpuResourceAvailability, QpuMemoryError>;

    /// Reserves/allocates QPU resources.
    ///
    /// Providers must validate the entire request before committing any
    /// externally visible partial allocation.
    fn allocate(
        &self,
        request: &QpuAllocationRequest,
    ) -> Result<QpuAllocation, QpuMemoryError>;

    /// Releases an allocation.
    ///
    /// Implementations MUST make release idempotent where the provider
    /// permits it. If the provider cannot make release idempotent, the
    /// implementation must return a deterministic error rather than silently
    /// treating a failed release as successful.
    fn release(
        &self,
        allocation: &QpuAllocation,
    ) -> Result<(), QpuMemoryError>;

    /// Synchronizes an allocation when the provider exposes synchronization
    /// semantics.
    fn synchronize(
        &self,
        allocation: &QpuAllocation,
    ) -> Result<(), QpuMemoryError> {
        let _ = allocation;

        Err(QpuMemoryError::SynchronizationUnavailable)
    }

    /// Returns whether the provider advertises a capability.
    fn supports(&self, capability: &QpuCapability) -> bool {
        self.descriptor().supports(capability)
    }
}

// =============================================================================
// In-memory contract validator
// =============================================================================

/// Stateless validator for QPU memory requests.
///
/// This does not allocate anything. It is useful for hardware validation,
/// routing preflight, scheduling, and tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct QpuMemoryValidator;

impl QpuMemoryValidator {
    /// Validates an allocation request against a device descriptor.
    pub fn validate(
        descriptor: &QpuDeviceDescriptor,
        request: &QpuAllocationRequest,
    ) -> Result<(), QpuMemoryError> {
        request.validate()?;

        let quantum = request.total_quantum_resources()?;

        if quantum > descriptor.physical_resource_count
            && request
                .quantum_resources
                .iter()
                .all(|resource| {
                    resource.kind != QuantumResourceKind::LogicalQubit
                })
        {
            return Err(QpuMemoryError::InsufficientCapacity);
        }

        if let Some(classical_capacity) = descriptor.classical_memory_bytes {
            let requested = request
                .buffers
                .iter()
                .filter(|buffer| {
                    matches!(
                        buffer.kind,
                        QpuMemoryKind::ClassicalInput
                            | QpuMemoryKind::ClassicalControl
                    )
                })
                .try_fold(0u64, |total, buffer| {
                    total.checked_add(buffer.bytes)
                })
                .ok_or(QpuMemoryError::LimitExceeded {
                    resource: "classical memory bytes",
                    maximum: classical_capacity,
                    requested: u64::MAX,
                })?;

            if requested > classical_capacity {
                return Err(QpuMemoryError::InsufficientCapacity);
            }
        }

        if let Some(result_capacity) = descriptor.result_memory_bytes {
            let requested = request
                .buffers
                .iter()
                .filter(|buffer| {
                    matches!(buffer.kind, QpuMemoryKind::MeasurementResult)
                })
                .try_fold(0u64, |total, buffer| {
                    total.checked_add(buffer.bytes)
                })
                .ok_or(QpuMemoryError::LimitExceeded {
                    resource: "result memory bytes",
                    maximum: result_capacity,
                    requested: u64::MAX,
                })?;

            if requested > result_capacity {
                return Err(QpuMemoryError::InsufficientCapacity);
            }
        }

        if request.exclusive
            && !descriptor.supports(&QpuCapability::ResourceReservation)
        {
            return Err(QpuMemoryError::CapabilityUnavailable {
                capability: QpuCapability::ResourceReservation
                    .as_str()
                    .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Capability negotiation
// =============================================================================

/// Requirement used when selecting a QPU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuMemoryRequirement {
    /// Required capabilities.
    pub required_capabilities: BTreeSet<QpuCapability>,

    /// Required paradigms.
    pub required_paradigms: BTreeSet<QpuParadigm>,

    /// Minimum quantum resources.
    pub minimum_quantum_resources: u64,

    /// Minimum classical memory.
    pub minimum_classical_bytes: Option<u64>,

    /// Minimum result memory.
    pub minimum_result_bytes: Option<u64>,
}

impl QpuMemoryRequirement {
    /// Creates an empty requirement.
    pub fn new() -> Self {
        Self {
            required_capabilities: BTreeSet::new(),
            required_paradigms: BTreeSet::new(),
            minimum_quantum_resources: 0,
            minimum_classical_bytes: None,
            minimum_result_bytes: None,
        }
    }

    /// Requires a capability.
    pub fn require_capability(
        mut self,
        capability: QpuCapability,
    ) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Requires an execution paradigm.
    pub fn require_paradigm(mut self, paradigm: QpuParadigm) -> Self {
        self.required_paradigms.insert(paradigm);
        self
    }

    /// Requires a minimum number of quantum resources.
    pub fn require_quantum_resources(
        mut self,
        count: u64,
    ) -> Result<Self, QpuMemoryError> {
        if count > MAX_QUANTUM_RESOURCES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "required quantum resources",
                maximum: MAX_QUANTUM_RESOURCES,
                requested: count,
            });
        }

        self.minimum_quantum_resources = count;
        Ok(self)
    }

    /// Requires classical memory.
    pub fn require_classical_bytes(
        mut self,
        bytes: u64,
    ) -> Result<Self, QpuMemoryError> {
        if bytes > MAX_BUFFER_BYTES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "required classical bytes",
                maximum: MAX_BUFFER_BYTES,
                requested: bytes,
            });
        }

        self.minimum_classical_bytes = Some(bytes);
        Ok(self)
    }

    /// Requires result memory.
    pub fn require_result_bytes(
        mut self,
        bytes: u64,
    ) -> Result<Self, QpuMemoryError> {
        if bytes > MAX_BUFFER_BYTES {
            return Err(QpuMemoryError::LimitExceeded {
                resource: "required result bytes",
                maximum: MAX_BUFFER_BYTES,
                requested: bytes,
            });
        }

        self.minimum_result_bytes = Some(bytes);
        Ok(self)
    }

    /// Tests whether a device satisfies this requirement.
    pub fn is_satisfied_by(
        &self,
        descriptor: &QpuDeviceDescriptor,
    ) -> bool {
        if self
            .required_capabilities
            .iter()
            .any(|capability| !descriptor.supports(capability))
        {
            return false;
        }

        if self
            .required_paradigms
            .iter()
            .any(|paradigm| !descriptor.supports_paradigm(*paradigm))
        {
            return false;
        }

        if descriptor.physical_resource_count
            < self.minimum_quantum_resources
        {
            return false;
        }

        if let Some(required) = self.minimum_classical_bytes {
            if descriptor
                .classical_memory_bytes
                .map_or(true, |available| available < required)
            {
                return false;
            }
        }

        if let Some(required) = self.minimum_result_bytes {
            if descriptor
                .result_memory_bytes
                .map_or(true, |available| available < required)
            {
                return false;
            }
        }

        true
    }
}

impl Default for QpuMemoryRequirement {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Reservation state
// =============================================================================

/// Lifecycle state of a QPU allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QpuAllocationState {
    /// Allocation exists and is usable.
    Active,

    /// Allocation is being synchronized.
    Synchronizing,

    /// Allocation is being released.
    Releasing,

    /// Allocation has been released.
    Released,

    /// Allocation became invalid because the provider lost the resource.
    Invalid,
}

impl QpuAllocationState {
    /// Returns whether resources may normally be used.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the allocation is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Released | Self::Invalid)
    }
}

// =============================================================================
// Resource mapping
// =============================================================================

/// Mapping between Zamani logical resource identity and QPU resource identity.
///
/// This is deliberately not a routing algorithm. It merely stores the result
/// of a mapping produced elsewhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QpuResourceMapping {
    /// Logical resource identifier.
    pub logical: String,

    /// Physical/provider resource identifier.
    pub physical: QpuResourceId,

    /// Resource kind.
    pub kind: QuantumResourceKind,
}

impl QpuResourceMapping {
    /// Creates a validated mapping.
    pub fn new(
        logical: impl Into<String>,
        physical: QpuResourceId,
        kind: QuantumResourceKind,
    ) -> Result<Self, QpuMemoryError> {
        let logical = logical.into();

        validate_identifier(
            "logical_resource",
            &logical,
            MAX_IDENTIFIER_LENGTH,
        )?;

        Ok(Self {
            logical,
            physical,
            kind,
        })
    }
}

/// A complete logical-to-physical resource mapping.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QpuResourceMap {
    mappings: BTreeMap<String, QpuResourceMapping>,
}

impl QpuResourceMap {
    /// Creates an empty mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a mapping.
    pub fn insert(
        &mut self,
        mapping: QpuResourceMapping,
    ) -> Option<QpuResourceMapping> {
        self.mappings
            .insert(mapping.logical.clone(), mapping)
    }

    /// Returns a mapping.
    pub fn get(&self, logical: &str) -> Option<&QpuResourceMapping> {
        self.mappings.get(logical)
    }

    /// Returns the number of mappings.
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether there are no mappings.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Returns all mappings in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &QpuResourceMapping)> {
        self.mappings.iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_id_rejects_empty_values() {
        let result = QpuDeviceId::new("");

        assert!(matches!(
            result,
            Err(QpuMemoryError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn provider_handle_debug_does_not_expose_value() {
        let handle =
            QpuProviderHandle::new("provider-secret-handle").unwrap();

        let debug = format!("{handle:?}");

        assert_eq!(debug, "QpuProviderHandle(<opaque>)");
        assert!(!debug.contains("provider-secret-handle"));
    }

    #[test]
    fn allocation_request_rejects_empty_request() {
        let request = QpuAllocationRequest::new();

        assert!(request.validate().is_err());
    }

    #[test]
    fn buffer_alignment_must_be_power_of_two() {
        let result = QpuBufferSpec::new(
            QpuMemoryKind::ClassicalInput,
            1024,
            QpuAccessMode::ReadWrite,
        )
        .unwrap()
        .with_alignment(3);

        assert!(result.is_err());
    }

    #[test]
    fn quantum_resource_request_counts_correctly() {
        let request = QpuAllocationRequest::new()
            .with_quantum_resources(
                QpuResourceSpec::new(
                    QuantumResourceKind::PhysicalQubit,
                    10,
                )
                .unwrap(),
            )
            .unwrap()
            .with_quantum_resources(
                QpuResourceSpec::new(
                    QuantumResourceKind::PhysicalQubit,
                    20,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            request.total_quantum_resources().unwrap(),
            30
        );
    }

    #[test]
    fn buffer_request_counts_correctly() {
        let request = QpuAllocationRequest::new()
            .with_buffer(
                QpuBufferSpec::new(
                    QpuMemoryKind::ClassicalInput,
                    1024,
                    QpuAccessMode::ReadOnly,
                )
                .unwrap(),
            )
            .unwrap()
            .with_buffer(
                QpuBufferSpec::new(
                    QpuMemoryKind::MeasurementResult,
                    2048,
                    QpuAccessMode::ReadWrite,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            request.total_buffer_bytes().unwrap(),
            3072
        );
    }

    #[test]
    fn result_buffer_rejects_excessive_shots() {
        let region = QpuRegionId::new("results").unwrap();

        let result =
            QpuResultBuffer::new(region, MAX_SHOTS + 1, "counts");

        assert!(result.is_err());
    }

    #[test]
    fn capability_negotiation_is_deterministic() {
        let device_id = QpuDeviceId::new("test-qpu").unwrap();

        let descriptor = QpuDeviceDescriptor::new(
            device_id,
            "test-provider",
            "Test QPU",
            QuantumTechnology::Superconducting,
            127,
        )
        .unwrap()
        .with_paradigm(QpuParadigm::GateBased)
        .with_capability(QpuCapability::DynamicCircuits)
        .with_capability(QpuCapability::PhysicalQubits);

        let requirement = QpuMemoryRequirement::new()
            .require_paradigm(QpuParadigm::GateBased)
            .require_capability(QpuCapability::DynamicCircuits)
            .require_quantum_resources(50)
            .unwrap();

        assert!(requirement.is_satisfied_by(&descriptor));
    }

    #[test]
    fn capability_negotiation_rejects_missing_feature() {
        let device_id = QpuDeviceId::new("test-qpu").unwrap();

        let descriptor = QpuDeviceDescriptor::new(
            device_id,
            "test-provider",
            "Test QPU",
            QuantumTechnology::Photonic,
            100,
        )
        .unwrap()
        .with_paradigm(QpuParadigm::GateBased);

        let requirement = QpuMemoryRequirement::new()
            .require_capability(QpuCapability::DynamicCircuits);

        assert!(!requirement.is_satisfied_by(&descriptor));
    }

    #[test]
    fn metadata_rejects_secret_like_keys() {
        let device_id = QpuDeviceId::new("test-qpu").unwrap();

        let result = QpuDeviceDescriptor::new(
            device_id,
            "provider",
            "device",
            QuantumTechnology::Other("future".to_owned()),
            1,
        )
        .unwrap()
        .with_metadata("api_key", "secret");

        assert!(matches!(
            result,
            Err(QpuMemoryError::SecretMaterialRejected)
        ));
    }

    #[test]
    fn resource_mapping_is_deterministic() {
        let physical = QpuResourceId::new("q17").unwrap();

        let mapping =
            QpuResourceMapping::new(
                "logical_q0",
                physical,
                QuantumResourceKind::PhysicalQubit,
            )
            .unwrap();

        let mut map = QpuResourceMap::new();

        assert!(map.insert(mapping).is_none());
        assert_eq!(map.len(), 1);
        assert!(map.get("logical_q0").is_some());
    }

    #[test]
    fn allocation_state_semantics_are_correct() {
        assert!(QpuAllocationState::Active.is_usable());
        assert!(!QpuAllocationState::Released.is_usable());
        assert!(QpuAllocationState::Released.is_terminal());
        assert!(QpuAllocationState::Invalid.is_terminal());
    }

    #[test]
    fn synchronization_state_semantics_are_correct() {
        assert!(QpuSyncState::Synchronized.is_synchronized());
        assert!(QpuSyncState::NotApplicable.is_synchronized());
        assert!(!QpuSyncState::HostDirty.is_synchronized());
        assert!(!QpuSyncState::DeviceDirty.is_synchronized());
    }

    #[test]
    fn access_modes_are_consistent() {
        assert!(QpuAccessMode::ReadOnly.readable());
        assert!(!QpuAccessMode::ReadOnly.writable());

        assert!(!QpuAccessMode::WriteOnly.readable());
        assert!(QpuAccessMode::WriteOnly.writable());

        assert!(QpuAccessMode::ReadWrite.readable());
        assert!(QpuAccessMode::ReadWrite.writable());
    }

    #[test]
    fn quantum_resource_is_not_a_byte_buffer() {
        assert!(
            !QpuMemoryKind::QuantumResource.is_byte_buffer()
        );

        assert!(
            QpuMemoryKind::MeasurementResult.is_byte_buffer()
        );
    }

    #[test]
    fn descriptor_accepts_future_technology() {
        let device_id = QpuDeviceId::new("future-qpu").unwrap();

        let descriptor = QpuDeviceDescriptor::new(
            device_id,
            "future-provider",
            "Future Quantum Processor",
            QuantumTechnology::Other(
                "future_architecture".to_owned(),
            ),
            100,
        )
        .unwrap();

        assert_eq!(
            descriptor.technology.as_str(),
            "future_architecture"
        );
    }

    #[test]
    fn resource_availability_rejects_excess_quantum_resources() {
        let device_id = QpuDeviceId::new("qpu").unwrap();

        let availability = QpuResourceAvailability {
            device_id,
            total_quantum_resources: 10,
            available_quantum_resources: 4,
            total_classical_bytes: None,
            available_classical_bytes: None,
            total_result_bytes: None,
            available_result_bytes: None,
        };

        let request = QpuAllocationRequest::new()
            .with_quantum_resources(
                QpuResourceSpec::new(
                    QuantumResourceKind::PhysicalQubit,
                    5,
                )
                .unwrap(),
            )
            .unwrap();

        assert!(!availability.can_satisfy(&request).unwrap());
    }

    #[test]
    fn validator_rejects_unsupported_exclusive_reservation() {
        let device_id = QpuDeviceId::new("qpu").unwrap();

        let descriptor = QpuDeviceDescriptor::new(
            device_id,
            "provider",
            "QPU",
            QuantumTechnology::TrappedIon,
            32,
        )
        .unwrap();

        let request = QpuAllocationRequest::new()
            .with_quantum_resources(
                QpuResourceSpec::new(
                    QuantumResourceKind::PhysicalQubit,
                    2,
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            QpuMemoryValidator::validate(&descriptor, &request);

        assert!(matches!(
            result,
            Err(QpuMemoryError::CapabilityUnavailable { .. })
        ));
    }
}