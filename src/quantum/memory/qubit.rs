//! Zamani Quantum Memory — Logical Qubit Memory Handles.
//!
//! Production-grade, representation-independent ownership and lifecycle
//! metadata for logical qubits held by `quantum::memory`.
//!
//! # Architectural responsibility
//!
//! This module owns the **memory-side identity and lifecycle of a qubit
//! resource**. It does NOT redefine quantum-program identity.
//!
//! The canonical logical qubit identifier is:
//!
//! ```text
//! crate::quantum::ir::QubitId
//! ```
//!
//! The canonical physical hardware identifier is:
//!
//! ```text
//! crate::quantum::ir::PhysicalQubitId
//! ```
//!
//! This module deliberately does not define replacement `QubitId` or
//! `PhysicalQubitId` types.
//!
//! # What this module owns
//!
//! `memory::qubit` owns:
//!
//! - memory-side qubit handles;
//! - memory resource identity;
//! - logical-qubit ownership metadata;
//! - lifecycle state;
//! - allocation state;
//! - optional physical placement metadata;
//! - generation/version tracking;
//! - reservation metadata;
//! - deterministic state transitions;
//! - validation of memory-side qubit usage;
//! - provider-neutral hardware association;
//! - stable qubit descriptors;
//! - safe APIs for later memory/register/allocator integration.
//!
//! # What this module does NOT own
//!
//! It does not own:
//!
//! - quantum gates;
//! - quantum state amplitudes;
//! - density matrices;
//! - stabilizer algebra;
//! - tensor networks;
//! - logical-to-physical routing algorithms;
//! - hardware topology;
//! - QPU communication;
//! - calibration;
//! - scheduling;
//! - allocation implementation;
//! - measurement collapse mathematics;
//! - QEC algorithms;
//! - provider authentication;
//! - backend-specific identifiers;
//! - simulation algorithms.
//!
//! Those responsibilities remain with their owning subsystems.
//!
//! # Critical distinction
//!
//! ```text
//! quantum::ir::QubitId
//!     = identity of a logical qubit in the quantum program
//!
//! quantum::ir::PhysicalQubitId
//!     = identity of a physical qubit in the hardware vocabulary
//!
//! quantum::memory::Qubit
//!     = memory-side handle describing ownership/lifecycle of that
//!       logical-qubit resource
//! ```
//!
//! Therefore a memory qubit never replaces the canonical IR identity.
//!
//! # Hardware neutrality
//!
//! The memory layer must work with:
//!
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom QPUs;
//! - photonic processors;
//! - spin qubits;
//! - semiconductor qubits;
//! - annealing systems where the execution model exposes qubit-like
//!   resources;
//! - simulators;
//! - CPU simulators;
//! - GPU simulators;
//! - distributed simulators;
//! - remote/cloud QPUs;
//! - future quantum hardware not known today.
//!
//! Hardware-specific mapping remains outside this module.
//!
//! # No unsafe
//!
//! This module contains no unsafe code.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
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
//! # Integration contract
//!
//! This API is intentionally designed for the future memory subsystem:
//!
//! ```text
//! memory::types
//!       │
//!       ▼
//! memory::qubit
//!       │
//!       ├── memory::register
//!       ├── memory::lifetime
//!       ├── memory::allocator
//!       ├── memory::reservation
//!       ├── memory::address
//!       ├── memory::state
//!       ├── memory::measurement
//!       ├── memory::permutation
//!       ├── memory::snapshot
//!       ├── memory::migration
//!       └── memory::diagnostics
//! ```
//!
//! Routing/hardware integration is deliberately represented by optional
//! provider-neutral association metadata rather than dependencies on routing
//! or hardware implementations.
//!
//! This makes this file independently implementable and prevents later
//! subsystems from forcing a redesign of the public qubit-memory contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::{PhysicalQubitId, QubitId};

use super::types::MemoryId;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for memory qubit descriptors.
pub const MEMORY_QUBIT_SCHEMA_ID: &str = "zamani.quantum.memory.qubit";

/// Current memory-qubit schema version.
pub const MEMORY_QUBIT_SCHEMA_VERSION: u16 = 1;

/// Maximum number of metadata entries associated with one qubit.
pub const MAX_QUBIT_METADATA_ENTRIES: usize = 256;

/// Maximum metadata key length.
pub const MAX_QUBIT_METADATA_KEY_LENGTH: usize = 128;

/// Maximum metadata value length.
pub const MAX_QUBIT_METADATA_VALUE_LENGTH: usize = 1024;

/// Maximum provider-neutral resource reference length.
pub const MAX_RESOURCE_REFERENCE_LENGTH: usize = 512;

/// Maximum generation number representable by this API.
pub const MAX_GENERATION: u64 = u64::MAX;

// =============================================================================
// Memory-side qubit state
// =============================================================================

/// Lifecycle state of a qubit memory resource.
///
/// This is a **memory lifecycle state**, not a quantum wavefunction state.
///
/// The actual quantum state is owned by `memory::state` implementations.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum QubitLifecycle {
    /// The memory resource has been created but is not currently allocated.
    Unallocated,

    /// The memory resource is reserved for a future allocation.
    Reserved,

    /// The qubit resource is allocated and available.
    Allocated,

    /// The qubit is actively owned by an execution context.
    InUse,

    /// The qubit has been measured by the owning execution context.
    ///
    /// This does not mean the qubit memory is unusable. A subsequent reset
    /// operation may transition it back to an executable state.
    Measured,

    /// The qubit has been reset and is available for execution.
    Reset,

    /// The qubit is temporarily migrating between memory representations or
    /// storage locations.
    Migrating,

    /// The qubit has been released.
    Released,

    /// The qubit has been permanently invalidated because its generation is
    /// no longer valid.
    Invalid,
}

impl Default for QubitLifecycle {
    fn default() -> Self {
        Self::Unallocated
    }
}

impl QubitLifecycle {
    /// Returns true if the resource currently owns an allocation.
    #[must_use]
    pub const fn is_allocated(self) -> bool {
        matches!(
            self,
            Self::Allocated
                | Self::InUse
                | Self::Measured
                | Self::Reset
                | Self::Migrating
        )
    }

    /// Returns true if execution may normally use this qubit.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Allocated | Self::InUse | Self::Reset)
    }

    /// Returns true if the qubit has been measured.
    #[must_use]
    pub const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }

    /// Returns true if the qubit has been released.
    #[must_use]
    pub const fn is_released(self) -> bool {
        matches!(self, Self::Released)
    }

    /// Returns true if the qubit can no longer be used.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns true if the state represents a temporary transition.
    #[must_use]
    pub const fn is_transitional(self) -> bool {
        matches!(self, Self::Reserved | Self::Migrating)
    }
}

impl fmt::Display for QubitLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Unallocated => "unallocated",
            Self::Reserved => "reserved",
            Self::Allocated => "allocated",
            Self::InUse => "in-use",
            Self::Measured => "measured",
            Self::Reset => "reset",
            Self::Migrating => "migrating",
            Self::Released => "released",
            Self::Invalid => "invalid",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Allocation state
// =============================================================================

/// Memory allocation state associated with a logical qubit.
///
/// This deliberately does not contain raw pointers or backend-specific
/// addresses. Actual allocation is owned by `memory::allocator` and
/// `memory::address`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum QubitAllocationState {
    /// No memory allocation currently exists.
    None,

    /// Allocation has been reserved but not committed.
    Reserved,

    /// Allocation is committed.
    Allocated,

    /// Allocation is being moved.
    Migrating,

    /// Allocation has been released.
    Released,
}

impl Default for QubitAllocationState {
    fn default() -> Self {
        Self::None
    }
}

impl QubitAllocationState {
    /// Returns whether allocation storage currently exists.
    #[must_use]
    pub const fn is_allocated(self) -> bool {
        matches!(self, Self::Allocated | Self::Migrating)
    }
}

// =============================================================================
// Ownership
// =============================================================================

/// Memory-side ownership class.
///
/// Ownership is intentionally provider-neutral.
///
/// It does not implement Rust ownership; it records the ownership semantics
/// of a quantum-memory resource.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum QubitOwnership {
    /// Resource is owned by the quantum execution context.
    Execution,

    /// Resource is owned by a simulator.
    Simulator,

    /// Resource is owned by a hardware execution session.
    Hardware,

    /// Resource is owned by an error-correction context.
    ErrorCorrection,

    /// Resource is temporarily shared by an orchestration layer.
    Shared,

    /// No active owner.
    None,
}

impl Default for QubitOwnership {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// Provider-neutral placement
// =============================================================================

/// Provider-neutral association between a logical qubit and a physical qubit.
///
/// This is deliberately metadata only.
///
/// It does not perform routing and does not validate topology.
///
/// The routing subsystem remains responsible for determining whether a
/// mapping is valid for a particular device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QubitPlacement {
    logical: QubitId,
    physical: PhysicalQubitId,
    generation: u64,
}

impl QubitPlacement {
    /// Creates a logical-to-physical association.
    ///
    /// No topology validation occurs here.
    #[must_use]
    pub const fn new(
        logical: QubitId,
        physical: PhysicalQubitId,
        generation: u64,
    ) -> Self {
        Self {
            logical,
            physical,
            generation,
        }
    }

    /// Returns the logical identity.
    #[must_use]
    pub const fn logical(&self) -> QubitId {
        self.logical
    }

    /// Returns the physical identity.
    #[must_use]
    pub const fn physical(&self) -> PhysicalQubitId {
        self.physical
    }

    /// Returns the placement generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

// =============================================================================
// Provider/backend reference
// =============================================================================

/// Opaque provider-neutral resource reference.
///
/// This may identify a backend, execution session, device, or external
/// resource, but it must never contain credentials or secrets.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ResourceReference(String);

impl ResourceReference {
    /// Creates a validated resource reference.
    pub fn new(value: impl Into<String>) -> Result<Self, QubitError> {
        let value = value.into();

        validate_text(
            &value,
            MAX_RESOURCE_REFERENCE_LENGTH,
            "resource reference",
        )?;

        Ok(Self(value))
    }

    /// Returns the reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Qubit errors
// =============================================================================

/// Errors produced by memory-side qubit operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum QubitError {
    /// The requested state transition is not legal.
    InvalidTransition {
        from: QubitLifecycle,
        to: QubitLifecycle,
    },

    /// A released qubit was accessed.
    Released {
        qubit: QubitId,
    },

    /// An invalid qubit was accessed.
    Invalid {
        qubit: QubitId,
    },

    /// A qubit is not currently allocated.
    NotAllocated {
        qubit: QubitId,
    },

    /// A qubit is currently reserved.
    Reserved {
        qubit: QubitId,
    },

    /// A qubit is currently migrating.
    Migrating {
        qubit: QubitId,
    },

    /// A qubit is already allocated.
    AlreadyAllocated {
        qubit: QubitId,
    },

    /// A qubit is already reserved.
    AlreadyReserved {
        qubit: QubitId,
    },

    /// A generation does not match.
    GenerationMismatch {
        qubit: QubitId,
        expected: u64,
        actual: u64,
    },

    /// The supplied memory resource does not match.
    MemoryMismatch {
        qubit: QubitId,
        expected: MemoryId,
        actual: MemoryId,
    },

    /// The logical identifier is invalid for the requested operation.
    InvalidLogicalQubit {
        qubit: QubitId,
    },

    /// A physical mapping already exists.
    PlacementAlreadyExists {
        qubit: QubitId,
    },

    /// No physical mapping exists.
    PlacementMissing {
        qubit: QubitId,
    },

    /// Metadata key is invalid.
    InvalidMetadataKey,

    /// Metadata value is invalid.
    InvalidMetadataValue,

    /// Too many metadata entries were supplied.
    MetadataLimitExceeded {
        maximum: usize,
    },

    /// Metadata key/value exceeds its maximum length.
    MetadataValueTooLong {
        maximum: usize,
    },

    /// A textual resource reference is invalid.
    InvalidResourceReference,

    /// A textual value exceeds the permitted length.
    TextTooLong {
        field: &'static str,
        maximum: usize,
    },

    /// A textual value is empty where a value is required.
    EmptyText {
        field: &'static str,
    },
}

impl fmt::Display for QubitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid qubit lifecycle transition: {from} -> {to}"
                )
            }

            Self::Released { qubit } => {
                write!(formatter, "qubit {qubit} has been released")
            }

            Self::Invalid { qubit } => {
                write!(formatter, "qubit {qubit} is invalid")
            }

            Self::NotAllocated { qubit } => {
                write!(formatter, "qubit {qubit} is not allocated")
            }

            Self::Reserved { qubit } => {
                write!(formatter, "qubit {qubit} is reserved")
            }

            Self::Migrating { qubit } => {
                write!(formatter, "qubit {qubit} is migrating")
            }

            Self::AlreadyAllocated { qubit } => {
                write!(formatter, "qubit {qubit} is already allocated")
            }

            Self::AlreadyReserved { qubit } => {
                write!(formatter, "qubit {qubit} is already reserved")
            }

            Self::GenerationMismatch {
                qubit,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} generation mismatch: expected {expected}, got {actual}"
                )
            }

            Self::MemoryMismatch {
                qubit,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} belongs to memory {expected}, not {actual}"
                )
            }

            Self::InvalidLogicalQubit { qubit } => {
                write!(formatter, "invalid logical qubit {qubit}")
            }

            Self::PlacementAlreadyExists { qubit } => {
                write!(formatter, "qubit {qubit} already has a physical placement")
            }

            Self::PlacementMissing { qubit } => {
                write!(formatter, "qubit {qubit} has no physical placement")
            }

            Self::InvalidMetadataKey => {
                formatter.write_str("invalid qubit metadata key")
            }

            Self::InvalidMetadataValue => {
                formatter.write_str("invalid qubit metadata value")
            }

            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "qubit metadata entry limit {maximum} exceeded"
                )
            }

            Self::MetadataValueTooLong { maximum } => {
                write!(
                    formatter,
                    "qubit metadata value exceeds maximum length {maximum}"
                )
            }

            Self::InvalidResourceReference => {
                formatter.write_str("invalid resource reference")
            }

            Self::TextTooLong { field, maximum } => {
                write!(
                    formatter,
                    "{field} exceeds maximum length {maximum}"
                )
            }

            Self::EmptyText { field } => {
                write!(formatter, "{field} cannot be empty")
            }
        }
    }
}

impl std::error::Error for QubitError {}

// =============================================================================
// Memory qubit
// =============================================================================

/// Memory-side quantum-qubit resource.
///
/// `Qubit` is intentionally **not** the quantum state itself.
///
/// It represents the lifecycle and ownership metadata necessary for the
/// memory subsystem to manage a logical qubit safely.
///
/// Quantum amplitudes, density matrices, stabilizers, tensor-network data,
/// and backend-native state are stored by representation-specific state
/// implementations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Qubit {
    /// Canonical logical-program identity.
    logical_id: QubitId,

    /// Memory resource containing this qubit.
    memory_id: MemoryId,

    /// Lifecycle of this memory resource.
    lifecycle: QubitLifecycle,

    /// Allocation state.
    allocation: QubitAllocationState,

    /// Memory-side owner.
    ownership: QubitOwnership,

    /// Generation protects against stale handles after release/reuse.
    generation: u64,

    /// Optional physical placement.
    placement: Option<QubitPlacement>,

    /// Optional provider/backend reference.
    resource: Option<ResourceReference>,

    /// Deterministic metadata.
    metadata: BTreeMap<String, String>,
}

impl Qubit {
    /// Creates a new unallocated memory-side qubit.
    ///
    /// The logical identity is canonical `quantum::ir::QubitId`.
    #[must_use]
    pub const fn new(logical_id: QubitId, memory_id: MemoryId) -> Self {
        Self {
            logical_id,
            memory_id,
            lifecycle: QubitLifecycle::Unallocated,
            allocation: QubitAllocationState::None,
            ownership: QubitOwnership::None,
            generation: 0,
            placement: None,
            resource: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the canonical logical qubit identity.
    #[must_use]
    pub const fn logical_id(&self) -> QubitId {
        self.logical_id
    }

    /// Returns the memory resource identity.
    #[must_use]
    pub const fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the current lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> QubitLifecycle {
        self.lifecycle
    }

    /// Returns the allocation state.
    #[must_use]
    pub const fn allocation_state(&self) -> QubitAllocationState {
        self.allocation
    }

    /// Returns the memory ownership class.
    #[must_use]
    pub const fn ownership(&self) -> QubitOwnership {
        self.ownership
    }

    /// Returns the generation.
    ///
    /// Consumers holding an external qubit handle should retain this value
    /// and verify it before mutating a reused resource.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the current physical placement, if one exists.
    #[must_use]
    pub fn placement(&self) -> Option<&QubitPlacement> {
        self.placement.as_ref()
    }

    /// Returns the provider-neutral resource reference, if present.
    #[must_use]
    pub fn resource_reference(&self) -> Option<&ResourceReference> {
        self.resource.as_ref()
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns whether this qubit currently has an allocation.
    #[must_use]
    pub const fn is_allocated(&self) -> bool {
        self.lifecycle.is_allocated()
    }

    /// Returns whether this qubit may normally be used by execution.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.lifecycle.is_usable()
    }

    /// Returns whether this qubit has been measured.
    #[must_use]
    pub const fn is_measured(&self) -> bool {
        self.lifecycle.is_measured()
    }

    /// Returns whether this qubit has been released.
    #[must_use]
    pub const fn is_released(&self) -> bool {
        self.lifecycle.is_released()
    }

    /// Returns whether this qubit is invalid.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        self.lifecycle.is_invalid()
    }

    /// Returns whether a physical placement exists.
    #[must_use]
    pub const fn has_placement(&self) -> bool {
        self.placement.is_some()
    }

    // =========================================================================
    // Lifecycle transitions
    // =========================================================================

    /// Reserves the qubit for an upcoming allocation.
    pub fn reserve(&mut self) -> Result<(), QubitError> {
        self.transition(QubitLifecycle::Reserved)
    }

    /// Commits a reserved qubit to an allocated state.
    ///
    /// The actual memory allocation must be performed by `memory::allocator`.
    /// This method only commits the qubit's logical memory metadata.
    pub fn allocate(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Reserved | QubitLifecycle::Unallocated => {
                self.transition(QubitLifecycle::Allocated)
            }

            QubitLifecycle::Allocated
            | QubitLifecycle::InUse
            | QubitLifecycle::Measured
            | QubitLifecycle::Reset
            | QubitLifecycle::Migrating => {
                Err(QubitError::AlreadyAllocated {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Marks the qubit as actively used.
    pub fn begin_use(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Allocated | QubitLifecycle::Reset => {
                self.transition(QubitLifecycle::InUse)
            }

            QubitLifecycle::InUse => Ok(()),

            QubitLifecycle::Measured => {
                Err(QubitError::InvalidTransition {
                    from: self.lifecycle,
                    to: QubitLifecycle::InUse,
                })
            }

            QubitLifecycle::Unallocated
            | QubitLifecycle::Reserved => {
                Err(QubitError::NotAllocated {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Migrating => {
                Err(QubitError::Migrating {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Marks the qubit as measured.
    ///
    /// The quantum measurement operation itself belongs to the state/measurement
    /// subsystem. This method only records the memory lifecycle consequence.
    pub fn mark_measured(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Allocated | QubitLifecycle::InUse | QubitLifecycle::Reset => {
                self.transition(QubitLifecycle::Measured)
            }

            QubitLifecycle::Measured => Ok(()),

            QubitLifecycle::Unallocated
            | QubitLifecycle::Reserved => {
                Err(QubitError::NotAllocated {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Migrating => {
                Err(QubitError::Migrating {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Marks the qubit as reset and usable again.
    ///
    /// The actual quantum reset operation belongs to the state subsystem.
    pub fn reset(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Allocated
            | QubitLifecycle::InUse
            | QubitLifecycle::Measured
            | QubitLifecycle::Reset => {
                self.transition(QubitLifecycle::Reset)
            }

            QubitLifecycle::Unallocated
            | QubitLifecycle::Reserved => {
                Err(QubitError::NotAllocated {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Migrating => {
                Err(QubitError::Migrating {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Begins a representation/storage migration.
    pub fn begin_migration(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Allocated
            | QubitLifecycle::InUse
            | QubitLifecycle::Measured
            | QubitLifecycle::Reset => {
                self.transition(QubitLifecycle::Migrating)
            }

            QubitLifecycle::Migrating => Ok(()),

            QubitLifecycle::Unallocated
            | QubitLifecycle::Reserved => {
                Err(QubitError::NotAllocated {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Completes a migration and returns the qubit to allocated state.
    pub fn complete_migration(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Migrating => {
                self.transition(QubitLifecycle::Allocated)
            }

            _ => Err(QubitError::InvalidTransition {
                from: self.lifecycle,
                to: QubitLifecycle::Allocated,
            }),
        }
    }

    /// Releases the qubit.
    ///
    /// The allocator must release the actual memory before or as part of the
    /// larger transaction surrounding this operation.
    ///
    /// Once released, the generation is incremented so stale external handles
    /// can be detected.
    pub fn release(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Unallocated => {
                self.transition(QubitLifecycle::Released)
            }

            QubitLifecycle::Reserved
            | QubitLifecycle::Allocated
            | QubitLifecycle::InUse
            | QubitLifecycle::Measured
            | QubitLifecycle::Reset => {
                self.transition(QubitLifecycle::Released)?;
                self.allocation = QubitAllocationState::Released;
                self.ownership = QubitOwnership::None;
                self.placement = None;
                self.resource = None;
                self.bump_generation()?;
                Ok(())
            }

            QubitLifecycle::Migrating => {
                Err(QubitError::Migrating {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }
        }
    }

    /// Invalidates the qubit permanently.
    ///
    /// This is intended for unrecoverable resource invalidation, not ordinary
    /// allocation release.
    pub fn invalidate(&mut self) -> Result<(), QubitError> {
        if self.lifecycle == QubitLifecycle::Invalid {
            return Ok(());
        }

        self.lifecycle = QubitLifecycle::Invalid;
        self.allocation = QubitAllocationState::Released;
        self.ownership = QubitOwnership::None;
        self.placement = None;
        self.resource = None;
        self.bump_generation()?;

        Ok(())
    }

    // =========================================================================
    // Allocation metadata
    // =========================================================================

    /// Marks the allocation metadata as committed.
    ///
    /// This should be called by `memory::allocator` only after successful
    /// allocation.
    pub fn commit_allocation(&mut self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Reserved | QubitLifecycle::Allocated => {
                self.allocation = QubitAllocationState::Allocated;

                if self.lifecycle == QubitLifecycle::Reserved {
                    self.lifecycle = QubitLifecycle::Allocated;
                }

                Ok(())
            }

            QubitLifecycle::Released => {
                Err(QubitError::Released {
                    qubit: self.logical_id,
                })
            }

            QubitLifecycle::Invalid => {
                Err(QubitError::Invalid {
                    qubit: self.logical_id,
                })
            }

            _ => Err(QubitError::InvalidTransition {
                from: self.lifecycle,
                to: QubitLifecycle::Allocated,
            }),
        }
    }

    /// Marks the allocation as migrating.
    pub fn mark_allocation_migrating(&mut self) -> Result<(), QubitError> {
        if !self.lifecycle.is_allocated() {
            return Err(QubitError::NotAllocated {
                qubit: self.logical_id,
            });
        }

        self.allocation = QubitAllocationState::Migrating;
        self.lifecycle = QubitLifecycle::Migrating;

        Ok(())
    }

    /// Marks the allocation as released without changing the logical identity.
    ///
    /// This is intended for allocator transaction completion.
    pub fn mark_allocation_released(&mut self) -> Result<(), QubitError> {
        if self.lifecycle == QubitLifecycle::Invalid {
            return Err(QubitError::Invalid {
                qubit: self.logical_id,
            });
        }

        self.allocation = QubitAllocationState::Released;
        self.lifecycle = QubitLifecycle::Released;
        self.ownership = QubitOwnership::None;
        self.placement = None;
        self.resource = None;
        self.bump_generation()?;

        Ok(())
    }

    // =========================================================================
    // Ownership
    // =========================================================================

    /// Sets the memory-side ownership class.
    pub fn set_ownership(
        &mut self,
        ownership: QubitOwnership,
    ) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.ownership = ownership;

        Ok(())
    }

    /// Returns whether the qubit has an active owner.
    #[must_use]
    pub const fn has_owner(&self) -> bool {
        !matches!(self.ownership, QubitOwnership::None)
    }

    // =========================================================================
    // Physical placement
    // =========================================================================

    /// Associates the logical qubit with a physical qubit.
    ///
    /// Routing/topology validation is intentionally outside this module.
    ///
    /// The caller should supply the generation corresponding to the routing
    /// decision. A later routing operation can replace the placement only
    /// through `replace_placement`.
    pub fn set_placement(
        &mut self,
        physical: PhysicalQubitId,
    ) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        if self.placement.is_some() {
            return Err(QubitError::PlacementAlreadyExists {
                qubit: self.logical_id,
            });
        }

        self.placement = Some(QubitPlacement::new(
            self.logical_id,
            physical,
            self.generation,
        ));

        Ok(())
    }

    /// Replaces an existing physical placement.
    ///
    /// The operation is metadata-only. Routing remains responsible for
    /// determining whether the new mapping is legal.
    pub fn replace_placement(
        &mut self,
        physical: PhysicalQubitId,
    ) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.placement = Some(QubitPlacement::new(
            self.logical_id,
            physical,
            self.generation,
        ));

        Ok(())
    }

    /// Removes the physical placement.
    pub fn clear_placement(&mut self) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.placement = None;

        Ok(())
    }

    /// Returns the physical qubit if mapped.
    #[must_use]
    pub fn physical_id(&self) -> Option<PhysicalQubitId> {
        self.placement
            .as_ref()
            .map(QubitPlacement::physical)
    }

    /// Validates that a supplied placement belongs to this qubit's generation.
    pub fn validate_placement_generation(&self) -> Result<(), QubitError> {
        if let Some(placement) = &self.placement {
            if placement.generation() != self.generation {
                return Err(QubitError::GenerationMismatch {
                    qubit: self.logical_id,
                    expected: self.generation,
                    actual: placement.generation(),
                });
            }
        }

        Ok(())
    }

    // =========================================================================
    // Provider/backend metadata
    // =========================================================================

    /// Associates the qubit with an opaque provider-neutral resource reference.
    pub fn set_resource_reference(
        &mut self,
        resource: ResourceReference,
    ) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.resource = Some(resource);

        Ok(())
    }

    /// Removes the provider-neutral resource reference.
    pub fn clear_resource_reference(&mut self) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.resource = None;

        Ok(())
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Inserts deterministic metadata.
    ///
    /// `BTreeMap` is used deliberately so serialization and iteration are
    /// deterministic.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, QubitError> {
        self.ensure_not_terminal()?;

        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        if !self.metadata.contains_key(&key)
            && self.metadata.len() >= MAX_QUBIT_METADATA_ENTRIES
        {
            return Err(QubitError::MetadataLimitExceeded {
                maximum: MAX_QUBIT_METADATA_ENTRIES,
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Removes metadata.
    pub fn remove_metadata(
        &mut self,
        key: &str,
    ) -> Result<Option<String>, QubitError> {
        self.ensure_not_terminal()?;

        Ok(self.metadata.remove(key))
    }

    /// Returns one metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Clears all metadata.
    pub fn clear_metadata(&mut self) -> Result<(), QubitError> {
        self.ensure_not_terminal()?;

        self.metadata.clear();

        Ok(())
    }

    // =========================================================================
    // Generation / stale-handle protection
    // =========================================================================

    /// Checks whether a caller's generation is still current.
    pub fn validate_generation(
        &self,
        expected: u64,
    ) -> Result<(), QubitError> {
        if expected != self.generation {
            return Err(QubitError::GenerationMismatch {
                qubit: self.logical_id,
                expected,
                actual: self.generation,
            });
        }

        Ok(())
    }

    /// Returns an immutable handle containing the current generation.
    #[must_use]
    pub const fn handle(&self) -> QubitHandle {
        QubitHandle {
            logical_id: self.logical_id,
            memory_id: self.memory_id,
            generation: self.generation,
        }
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn transition(&mut self, target: QubitLifecycle) -> Result<(), QubitError> {
        if self.lifecycle == target {
            return Ok(());
        }

        if !is_valid_transition(self.lifecycle, target) {
            return Err(QubitError::InvalidTransition {
                from: self.lifecycle,
                to: target,
            });
        }

        self.lifecycle = target;

        match target {
            QubitLifecycle::Reserved => {
                self.allocation = QubitAllocationState::Reserved;
            }

            QubitLifecycle::Allocated
            | QubitLifecycle::InUse
            | QubitLifecycle::Measured
            | QubitLifecycle::Reset => {
                self.allocation = QubitAllocationState::Allocated;
            }

            QubitLifecycle::Migrating => {
                self.allocation = QubitAllocationState::Migrating;
            }

            QubitLifecycle::Released => {
                self.allocation = QubitAllocationState::Released;
            }

            QubitLifecycle::Unallocated => {
                self.allocation = QubitAllocationState::None;
            }

            QubitLifecycle::Invalid => {
                self.allocation = QubitAllocationState::Released;
            }
        }

        Ok(())
    }

    fn ensure_not_terminal(&self) -> Result<(), QubitError> {
        match self.lifecycle {
            QubitLifecycle::Released => Err(QubitError::Released {
                qubit: self.logical_id,
            }),

            QubitLifecycle::Invalid => Err(QubitError::Invalid {
                qubit: self.logical_id,
            }),

            _ => Ok(()),
        }
    }

    fn bump_generation(&mut self) -> Result<(), QubitError> {
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(QubitError::Invalid {
                qubit: self.logical_id,
            })?;

        Ok(())
    }
}

// =============================================================================
// Stale-handle protection
// =============================================================================

/// Lightweight, immutable reference to a memory qubit.
///
/// A handle contains the logical identity, memory identity, and generation.
/// It never contains a pointer and therefore cannot become a Rust dangling
/// pointer.
///
/// The generation must be validated against the current `Qubit` before a
/// mutation is performed.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct QubitHandle {
    logical_id: QubitId,
    memory_id: MemoryId,
    generation: u64,
}

impl QubitHandle {
    /// Creates a handle from an existing memory qubit.
    #[must_use]
    pub const fn new(
        logical_id: QubitId,
        memory_id: MemoryId,
        generation: u64,
    ) -> Self {
        Self {
            logical_id,
            memory_id,
            generation,
        }
    }

    /// Returns the canonical logical identifier.
    #[must_use]
    pub const fn logical_id(self) -> QubitId {
        self.logical_id
    }

    /// Returns the memory identity.
    #[must_use]
    pub const fn memory_id(self) -> MemoryId {
        self.memory_id
    }

    /// Returns the generation captured by this handle.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    /// Validates this handle against a current qubit.
    pub fn validate(&self, qubit: &Qubit) -> Result<(), QubitError> {
        if self.logical_id != qubit.logical_id {
            return Err(QubitError::InvalidLogicalQubit {
                qubit: self.logical_id,
            });
        }

        if self.memory_id != qubit.memory_id {
            return Err(QubitError::MemoryMismatch {
                qubit: self.logical_id,
                expected: self.memory_id,
                actual: qubit.memory_id,
            });
        }

        qubit.validate_generation(self.generation)
    }
}

// =============================================================================
// Transition table
// =============================================================================

/// Determines whether a lifecycle transition is valid.
///
/// Keeping this table centralized means future modules can rely on one
/// lifecycle contract rather than duplicating transition rules.
const fn is_valid_transition(
    from: QubitLifecycle,
    to: QubitLifecycle,
) -> bool {
    match (from, to) {
        (QubitLifecycle::Unallocated, QubitLifecycle::Reserved)
        | (QubitLifecycle::Unallocated, QubitLifecycle::Allocated)
        | (QubitLifecycle::Unallocated, QubitLifecycle::Released)
        | (QubitLifecycle::Reserved, QubitLifecycle::Allocated)
        | (QubitLifecycle::Reserved, QubitLifecycle::Released)
        | (QubitLifecycle::Allocated, QubitLifecycle::InUse)
        | (QubitLifecycle::Allocated, QubitLifecycle::Measured)
        | (QubitLifecycle::Allocated, QubitLifecycle::Reset)
        | (QubitLifecycle::Allocated, QubitLifecycle::Migrating)
        | (QubitLifecycle::Allocated, QubitLifecycle::Released)
        | (QubitLifecycle::InUse, QubitLifecycle::Measured)
        | (QubitLifecycle::InUse, QubitLifecycle::Reset)
        | (QubitLifecycle::InUse, QubitLifecycle::Migrating)
        | (QubitLifecycle::InUse, QubitLifecycle::Released)
        | (QubitLifecycle::Measured, QubitLifecycle::Reset)
        | (QubitLifecycle::Measured, QubitLifecycle::Migrating)
        | (QubitLifecycle::Measured, QubitLifecycle::Released)
        | (QubitLifecycle::Reset, QubitLifecycle::InUse)
        | (QubitLifecycle::Reset, QubitLifecycle::Measured)
        | (QubitLifecycle::Reset, QubitLifecycle::Migrating)
        | (QubitLifecycle::Reset, QubitLifecycle::Released)
        | (QubitLifecycle::Migrating, QubitLifecycle::Allocated)
        | (QubitLifecycle::Migrating, QubitLifecycle::Released)
        | (QubitLifecycle::Unallocated, QubitLifecycle::Invalid)
        | (QubitLifecycle::Reserved, QubitLifecycle::Invalid)
        | (QubitLifecycle::Allocated, QubitLifecycle::Invalid)
        | (QubitLifecycle::InUse, QubitLifecycle::Invalid)
        | (QubitLifecycle::Measured, QubitLifecycle::Invalid)
        | (QubitLifecycle::Reset, QubitLifecycle::Invalid)
        | (QubitLifecycle::Migrating, QubitLifecycle::Invalid)
        | (QubitLifecycle::Released, QubitLifecycle::Invalid) => true,

        _ => false,
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_text(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> Result<(), QubitError> {
    if value.is_empty() {
        return Err(QubitError::EmptyText { field });
    }

    if value.len() > maximum {
        return Err(QubitError::TextTooLong {
            field,
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(QubitError::InvalidResourceReference);
    }

    Ok(())
}

fn validate_metadata_key(key: &str) -> Result<(), QubitError> {
    if key.is_empty() {
        return Err(QubitError::InvalidMetadataKey);
    }

    if key.len() > MAX_QUBIT_METADATA_KEY_LENGTH {
        return Err(QubitError::MetadataValueTooLong {
            maximum: MAX_QUBIT_METADATA_KEY_LENGTH,
        });
    }

    if key.chars().any(char::is_control) {
        return Err(QubitError::InvalidMetadataKey);
    }

    Ok(())
}

fn validate_metadata_value(value: &str) -> Result<(), QubitError> {
    if value.len() > MAX_QUBIT_METADATA_VALUE_LENGTH {
        return Err(QubitError::MetadataValueTooLong {
            maximum: MAX_QUBIT_METADATA_VALUE_LENGTH,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(QubitError::InvalidMetadataValue);
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::{PhysicalQubitId, QubitId};

    fn memory_id() -> MemoryId {
        // `MemoryId` is intentionally opaque. This test assumes its public
        // constructor is `new`, matching the contract established by
        // memory::types.
        MemoryId::new(1)
    }

    #[test]
    fn new_qubit_starts_unallocated() {
        let qubit = Qubit::new(QubitId::new(0), memory_id());

        assert_eq!(qubit.logical_id(), QubitId::new(0));
        assert_eq!(qubit.lifecycle(), QubitLifecycle::Unallocated);
        assert_eq!(
            qubit.allocation_state(),
            QubitAllocationState::None
        );
        assert_eq!(qubit.ownership(), QubitOwnership::None);
        assert_eq!(qubit.generation(), 0);
        assert!(qubit.placement().is_none());
    }

    #[test]
    fn reservation_and_allocation_are_deterministic() {
        let mut qubit = Qubit::new(QubitId::new(1), memory_id());

        qubit.reserve().expect("reservation should succeed");

        assert_eq!(
            qubit.lifecycle(),
            QubitLifecycle::Reserved
        );

        qubit
            .commit_allocation()
            .expect("allocation commit should succeed");

        assert_eq!(
            qubit.lifecycle(),
            QubitLifecycle::Allocated
        );
        assert_eq!(
            qubit.allocation_state(),
            QubitAllocationState::Allocated
        );
    }

    #[test]
    fn allocation_from_unallocated_is_supported() {
        let mut qubit = Qubit::new(QubitId::new(2), memory_id());

        qubit.allocate().expect("allocation should succeed");

        assert_eq!(
            qubit.lifecycle(),
            QubitLifecycle::Allocated
        );
    }

    #[test]
    fn measurement_requires_allocation() {
        let mut qubit = Qubit::new(QubitId::new(3), memory_id());

        let error = qubit
            .mark_measured()
            .expect_err("unallocated measurement must fail");

        assert_eq!(
            error,
            QubitError::NotAllocated {
                qubit: QubitId::new(3)
            }
        );
    }

    #[test]
    fn measurement_then_reset_is_valid() {
        let mut qubit = Qubit::new(QubitId::new(4), memory_id());

        qubit.allocate().unwrap();
        qubit.begin_use().unwrap();
        qubit.mark_measured().unwrap();

        assert!(qubit.is_measured());

        qubit.reset().unwrap();

        assert_eq!(qubit.lifecycle(), QubitLifecycle::Reset);
        assert!(qubit.is_usable());
    }

    #[test]
    fn released_qubit_rejects_future_use() {
        let mut qubit = Qubit::new(QubitId::new(5), memory_id());

        qubit.allocate().unwrap();
        qubit.release().unwrap();

        assert!(qubit.is_released());

        let error = qubit
            .begin_use()
            .expect_err("released qubit must reject use");

        assert_eq!(
            error,
            QubitError::Released {
                qubit: QubitId::new(5)
            }
        );
    }

    #[test]
    fn release_increments_generation() {
        let mut qubit = Qubit::new(QubitId::new(6), memory_id());

        let original = qubit.handle();

        qubit.allocate().unwrap();
        qubit.release().unwrap();

        assert_eq!(qubit.generation(), 1);

        let error = original
            .validate(&qubit)
            .expect_err("old handle must be stale");

        assert_eq!(
            error,
            QubitError::GenerationMismatch {
                qubit: QubitId::new(6),
                expected: 0,
                actual: 1,
            }
        );
    }

    #[test]
    fn handle_validates_current_generation() {
        let mut qubit = Qubit::new(QubitId::new(7), memory_id());

        qubit.allocate().unwrap();

        let handle = qubit.handle();

        assert!(handle.validate(&qubit).is_ok());
    }

    #[test]
    fn physical_placement_is_provider_neutral() {
        let mut qubit = Qubit::new(QubitId::new(8), memory_id());

        qubit.allocate().unwrap();

        qubit
            .set_placement(PhysicalQubitId::new(12))
            .expect("placement should succeed");

        assert_eq!(
            qubit.physical_id(),
            Some(PhysicalQubitId::new(12))
        );

        let placement = qubit
            .placement()
            .expect("placement should exist");

        assert_eq!(
            placement.logical(),
            QubitId::new(8)
        );
        assert_eq!(
            placement.physical(),
            PhysicalQubitId::new(12)
        );
        assert_eq!(placement.generation(), 0);
    }

    #[test]
    fn duplicate_placement_is_rejected() {
        let mut qubit = Qubit::new(QubitId::new(9), memory_id());

        qubit.allocate().unwrap();

        qubit
            .set_placement(PhysicalQubitId::new(1))
            .unwrap();

        let error = qubit
            .set_placement(PhysicalQubitId::new(2))
            .expect_err("second placement must fail");

        assert_eq!(
            error,
            QubitError::PlacementAlreadyExists {
                qubit: QubitId::new(9)
            }
        );
    }

    #[test]
    fn placement_can_be_replaced_explicitly() {
        let mut qubit = Qubit::new(QubitId::new(10), memory_id());

        qubit.allocate().unwrap();
        qubit.set_placement(PhysicalQubitId::new(1)).unwrap();

        qubit
            .replace_placement(PhysicalQubitId::new(2))
            .unwrap();

        assert_eq!(
            qubit.physical_id(),
            Some(PhysicalQubitId::new(2))
        );
    }

    #[test]
    fn ownership_is_explicit() {
        let mut qubit = Qubit::new(QubitId::new(11), memory_id());

        qubit.allocate().unwrap();

        qubit
            .set_ownership(QubitOwnership::Simulator)
            .unwrap();

        assert_eq!(
            qubit.ownership(),
            QubitOwnership::Simulator
        );
        assert!(qubit.has_owner());
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut qubit = Qubit::new(QubitId::new(12), memory_id());

        qubit.allocate().unwrap();

        qubit
            .insert_metadata("device", "simulator")
            .unwrap();

        qubit
            .insert_metadata("role", "ancilla")
            .unwrap();

        assert_eq!(
            qubit.metadata_value("device"),
            Some("simulator")
        );
        assert_eq!(
            qubit.metadata_value("role"),
            Some("ancilla")
        );
    }

    #[test]
    fn invalid_metadata_key_is_rejected() {
        let mut qubit = Qubit::new(QubitId::new(13), memory_id());

        qubit.allocate().unwrap();

        let error = qubit
            .insert_metadata("", "value")
            .expect_err("empty key must fail");

        assert_eq!(error, QubitError::InvalidMetadataKey);
    }

    #[test]
    fn resource_reference_rejects_empty_values() {
        let error = ResourceReference::new("")
            .expect_err("empty reference must fail");

        assert_eq!(
            error,
            QubitError::EmptyText {
                field: "resource reference"
            }
        );
    }

    #[test]
    fn migration_is_explicit() {
        let mut qubit = Qubit::new(QubitId::new(14), memory_id());

        qubit.allocate().unwrap();
        qubit.begin_migration().unwrap();

        assert_eq!(
            qubit.lifecycle(),
            QubitLifecycle::Migrating
        );

        qubit.complete_migration().unwrap();

        assert_eq!(
            qubit.lifecycle(),
            QubitLifecycle::Allocated
        );
    }

    #[test]
    fn invalid_qubit_cannot_be_reused() {
        let mut qubit = Qubit::new(QubitId::new(15), memory_id());

        qubit.invalidate().unwrap();

        assert!(qubit.is_invalid());

        let error = qubit
            .allocate()
            .expect_err("invalid qubit must never be reallocated");

        assert_eq!(
            error,
            QubitError::Invalid {
                qubit: QubitId::new(15)
            }
        );
    }

    #[test]
    fn lifecycle_display_is_stable() {
        assert_eq!(
            QubitLifecycle::Unallocated.to_string(),
            "unallocated"
        );
        assert_eq!(
            QubitLifecycle::Allocated.to_string(),
            "allocated"
        );
        assert_eq!(
            QubitLifecycle::Migrating.to_string(),
            "migrating"
        );
    }

    #[test]
    fn placement_generation_matches_qubit_generation() {
        let mut qubit = Qubit::new(QubitId::new(16), memory_id());

        qubit.allocate().unwrap();
        qubit.set_placement(PhysicalQubitId::new(4)).unwrap();

        assert!(qubit.validate_placement_generation().is_ok());

        qubit.release().unwrap();

        // Release clears the placement, so validation remains successful.
        assert!(qubit.validate_placement_generation().is_ok());
    }
}