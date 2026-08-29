//! Zamani Quantum Memory — Lifetime and Ownership Management
//!
//! Production-grade, representation-independent lifetime semantics for
//! quantum memory resources.
//!
//! # Architectural role
//!
//! This module defines the lifecycle of quantum-memory resources independently
//! from:
//!
//! - quantum-state representation;
//! - state-vector implementation;
//! - density matrices;
//! - stabilizer/tableau storage;
//! - tensor networks;
//! - GPU memory;
//! - distributed memory;
//! - hardware-specific APIs;
//! - routing algorithms;
//! - scheduling algorithms;
//! - QEC decoders;
//! - compiler IR construction.
//!
//! The module therefore works for:
//!
//! - classical simulators;
//! - state-vector simulators;
//! - density-matrix simulators;
//! - stabilizer simulators;
//! - tensor-network simulators;
//! - CPU execution;
//! - GPU execution;
//! - distributed execution;
//! - local QPUs;
//! - remote/cloud QPUs;
//! - dynamic circuits;
//! - mid-circuit measurement;
//! - measurement/reset/reuse workflows;
//! - quantum-error-correction workloads;
//! - hardware with very different physical qubit technologies.
//!
//! # Critical semantic distinction
//!
//! A quantum-memory allocation has a lifetime that is different from the
//! semantic state of the quantum information stored in it.
//!
//! In particular:
//!
//! ```text
//! allocation lifetime
//!
//! Allocated ──► Active ──► Released
//!                    │
//!                    ├──► Measured
//!                    ├──► Reset
//!                    └──► Active
//! ```
//!
//! Measurement does NOT inherently release memory.
//!
//! Reset does NOT allocate a new memory resource.
//!
//! A measured qubit may remain allocated and may participate in a subsequent
//! operation when the execution model permits it.
//!
//! This distinction is essential for dynamic circuits and QEC.
//!
//! # Ownership model
//!
//! A `MemoryLease` represents exclusive ownership of a quantum-memory
//! resource's lifetime metadata.
//!
//! Cloning a lease is deliberately not supported.
//!
//! `Copy` is deliberately not implemented.
//!
//! IDs can be copied; ownership cannot.
//!
//! # Canonical identities
//!
//! This module uses the canonical identities from `quantum::ir`:
//!
//! - `crate::quantum::ir::QubitId`;
//! - `crate::quantum::ir::PhysicalQubitId`;
//! - `crate::quantum::ir::ClassicalBitId`.
//!
//! It never defines replacement qubit IDs.
//!
//! Memory-specific allocation identity comes from `memory::types`.
//!
//! # Hardware neutrality
//!
//! No vendor-specific assumptions are made here.
//!
//! A QPU adapter may associate a lifetime record with:
//!
//! - IBM hardware;
//! - Quantinuum hardware;
//! - IonQ hardware;
//! - Rigetti hardware;
//! - IQM hardware;
//! - neutral-atom hardware;
//! - superconducting hardware;
//! - trapped-ion hardware;
//! - photonic hardware;
//! - annealing hardware;
//! - simulators;
//! - future hardware types.
//!
//! Such adapters belong outside this module.
//!
//! # Safety
//!
//! This module uses no `unsafe` code.
//!
//! No raw pointers are exposed.
//!
//! No global mutable state is used.
//!
//! No implicit allocation is performed by state-transition methods.
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
//! Later memory modules should consume this module rather than creating their
//! own lifetime state machine.
//!
//! In particular:
//!
//! - `allocator.rs` owns physical allocation/deallocation;
//! - `budget.rs` owns resource accounting;
//! - `qubit.rs` owns logical-qubit namespace bookkeeping;
//! - `state.rs` owns quantum-state representation;
//! - `measurement.rs` owns measurement mechanics;
//! - `reset.rs` owns reset mechanics;
//! - `migration.rs` owns representation/location migration;
//! - `coherence.rs` owns host/device/distributed coherence;
//! - `hardware/*` owns backend-specific execution;
//! - this module owns lifecycle legality and ownership semantics.
//!
//! No module should reinterpret `Released` as merely "not currently used".
//! Released is terminal for a lease generation. Reuse requires a new lease
//! generation or a new allocation record.

use std::fmt;

use crate::quantum::ir::{ClassicalBitId, PhysicalQubitId, QubitId};

use super::types::{AllocationId, MemoryId};

// =============================================================================
// Lifecycle state
// =============================================================================

/// Lifecycle state of a quantum-memory allocation.
///
/// This state describes the lifetime of the memory resource, not the complete
/// quantum state stored in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LifetimeState {
    /// Resource exists and has been allocated but has not entered execution.
    Allocated,

    /// Resource is actively owned and usable.
    Active,

    /// Resource is temporarily unavailable for normal operations because an
    /// execution subsystem has explicitly placed it in a transition state.
    ///
    /// `Quiescent` does not mean released.
    Quiescent,

    /// Resource has been released.
    ///
    /// This is terminal for the current allocation generation.
    Released,

    /// Resource has been invalidated because its ownership/lifecycle contract
    /// can no longer be trusted.
    ///
    /// Invalidation is stronger than release and normally indicates a
    /// detected consistency, backend, or integrity failure.
    Invalid,
}

impl LifetimeState {
    /// Returns whether this lifetime state permits normal quantum operations.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Allocated | Self::Active)
    }

    /// Returns whether this state represents an allocated resource.
    pub const fn is_allocated(self) -> bool {
        matches!(
            self,
            Self::Allocated | Self::Active | Self::Quiescent
        )
    }

    /// Returns whether this state is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Released | Self::Invalid)
    }

    /// Returns whether the resource has been released.
    pub const fn is_released(self) -> bool {
        matches!(self, Self::Released)
    }

    /// Returns whether the resource has been invalidated.
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns whether the resource is temporarily quiescent.
    pub const fn is_quiescent(self) -> bool {
        matches!(self, Self::Quiescent)
    }
}

impl fmt::Display for LifetimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Allocated => "allocated",
            Self::Active => "active",
            Self::Quiescent => "quiescent",
            Self::Released => "released",
            Self::Invalid => "invalid",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Quantum semantic state
// =============================================================================

/// Semantic state of the quantum information currently associated with a
/// memory allocation.
///
/// This is deliberately separate from [`LifetimeState`].
///
/// A measured qubit can remain allocated.
/// A reset qubit can return to an active state.
/// A released allocation cannot have a quantum semantic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumMemoryState {
    /// Quantum information is available for normal operations.
    Available,

    /// The resource has been measured.
    ///
    /// The allocation remains valid.
    Measured,

    /// A reset operation has established the backend/simulator's reset
    /// semantics.
    Reset,

    /// The quantum state is currently involved in an explicitly tracked
    /// operation/transition.
    InTransition,

    /// The semantic state is unknown because execution failed or the backend
    /// could not provide sufficient state information.
    Unknown,
}

impl QuantumMemoryState {
    /// Returns whether normal quantum operations may proceed according to
    /// memory-level semantics.
    ///
    /// Backend and circuit-level validation may impose additional rules.
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Available | Self::Measured | Self::Reset
        )
    }

    /// Returns whether this state represents a measurement event.
    pub const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }

    /// Returns whether this state represents a reset state.
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the state is unknown.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl Default for QuantumMemoryState {
    fn default() -> Self {
        Self::Available
    }
}

impl fmt::Display for QuantumMemoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Available => "available",
            Self::Measured => "measured",
            Self::Reset => "reset",
            Self::InTransition => "in-transition",
            Self::Unknown => "unknown",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Ownership
// =============================================================================

/// Ownership mode for a quantum-memory resource.
///
/// This is intentionally a memory-level concept and does not attempt to model
/// the entire language ownership/type system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ownership {
    /// The current execution context has exclusive ownership.
    Exclusive,

    /// The resource is controlled by an external backend/execution provider.
    ///
    /// Local code may hold metadata about the resource but must not assume it
    /// can directly release or mutate the backend resource.
    BackendManaged,

    /// The resource is jointly represented across execution domains.
    ///
    /// This is useful for distributed simulation and distributed execution.
    Distributed,
}

impl Ownership {
    /// Returns whether local code may directly perform lifecycle operations.
    pub const fn permits_local_lifecycle_control(self) -> bool {
        matches!(self, Self::Exclusive)
    }

    /// Returns whether the resource is externally controlled.
    pub const fn is_backend_managed(self) -> bool {
        matches!(self, Self::BackendManaged)
    }

    /// Returns whether the resource is distributed.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }
}

// =============================================================================
// Reuse policy
// =============================================================================

/// Policy governing what happens after measurement or reset.
///
/// This is separate from lifetime because measurement does not inherently
/// terminate a memory allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReusePolicy {
    /// The allocation may continue to be used after measurement.
    ///
    /// This is appropriate for dynamic circuits and many QEC workflows.
    AllowAfterMeasurement,

    /// Measurement is permitted, but subsequent reuse requires an explicit
    /// reset operation.
    RequireResetAfterMeasurement,

    /// Measurement consumes the logical allocation from the perspective of
    /// this memory owner.
    ///
    /// The physical resource may still exist under a backend owner.
    ReleaseAfterMeasurement,
}

impl Default for ReusePolicy {
    fn default() -> Self {
        Self::AllowAfterMeasurement
    }
}

// =============================================================================
// Release reason
// =============================================================================

/// Reason why a memory allocation was released.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReleaseReason {
    /// Normal program-controlled release.
    Explicit,

    /// Allocation lifetime ended because an execution scope completed.
    ScopeEnd,

    /// Backend completed the job and relinquished ownership.
    BackendCompleted,

    /// Resource was reclaimed by a memory manager.
    Reclaimed,

    /// Resource was migrated to another allocation/location and the old
    /// allocation was retired.
    Migrated,

    /// Resource was released after a measurement according to an explicit
    /// policy.
    Measurement,

    /// Resource became unusable because of an execution failure.
    ExecutionFailure,
}

impl fmt::Display for ReleaseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Explicit => "explicit",
            Self::ScopeEnd => "scope-end",
            Self::BackendCompleted => "backend-completed",
            Self::Reclaimed => "reclaimed",
            Self::Migrated => "migrated",
            Self::Measurement => "measurement",
            Self::ExecutionFailure => "execution-failure",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Invalidation reason
// =============================================================================

/// Reason why a memory allocation was invalidated.
///
/// Invalid state is intentionally distinct from normal release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InvalidationReason {
    /// Ownership information is no longer trustworthy.
    OwnershipViolation,

    /// The backend reported that the resource is no longer valid.
    BackendInvalidated,

    /// A memory integrity check failed.
    IntegrityFailure,

    /// A synchronization/coherence failure makes the current state
    /// untrustworthy.
    CoherenceFailure,

    /// A distributed execution participant failed in a way that prevents
    /// continued use.
    DistributedFailure,

    /// The execution system detected an impossible lifecycle transition.
    LifecycleViolation,

    /// State metadata and physical storage disagree.
    StateMismatch,
}

impl fmt::Display for InvalidationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::OwnershipViolation => "ownership-violation",
            Self::BackendInvalidated => "backend-invalidated",
            Self::IntegrityFailure => "integrity-failure",
            Self::CoherenceFailure => "coherence-failure",
            Self::DistributedFailure => "distributed-failure",
            Self::LifecycleViolation => "lifecycle-violation",
            Self::Mismatch => "state-mismatch",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Lifecycle event
// =============================================================================

/// Event recorded against a lifetime record.
///
/// Events are immutable values. The caller decides whether and where to retain
/// them; this module does not maintain an unbounded event log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifetimeEvent {
    /// Allocation entered existence.
    Allocated,

    /// Allocation became active.
    Activated,

    /// Allocation entered quiescence.
    Quiesced,

    /// Allocation resumed active use.
    Resumed,

    /// Quantum memory was measured.
    Measured,

    /// Quantum memory was reset.
    Reset,

    /// Allocation was explicitly released.
    Released(ReleaseReason),

    /// Allocation was invalidated.
    Invalidated(InvalidationReason),
}

// =============================================================================
// Errors
// =============================================================================

/// Errors returned by lifecycle operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifetimeError {
    /// The allocation ID does not match the lifetime record.
    AllocationMismatch {
        expected: AllocationId,
        actual: AllocationId,
    },

    /// The memory resource ID does not match the lifetime record.
    MemoryMismatch {
        expected: MemoryId,
        actual: MemoryId,
    },

    /// An operation requires a logical qubit but the record has none.
    MissingLogicalQubit,

    /// The requested logical qubit does not match the record.
    LogicalQubitMismatch {
        expected: QubitId,
        actual: QubitId,
    },

    /// A physical qubit is associated with the record but does not match the
    /// requested physical qubit.
    PhysicalQubitMismatch {
        expected: PhysicalQubitId,
        actual: PhysicalQubitId,
    },

    /// A classical measurement destination does not match the record.
    ClassicalBitMismatch {
        expected: ClassicalBitId,
        actual: ClassicalBitId,
    },

    /// Operation was attempted after release.
    Released {
        allocation: AllocationId,
    },

    /// Operation was attempted after invalidation.
    Invalid {
        allocation: AllocationId,
    },

    /// Operation is not legal in the current lifetime state.
    InvalidTransition {
        from: LifetimeState,
        operation: &'static str,
    },

    /// Operation is not legal for the current semantic state.
    InvalidQuantumTransition {
        from: QuantumMemoryState,
        operation: &'static str,
    },

    /// The current ownership mode prevents the requested operation.
    OwnershipDenied {
        ownership: Ownership,
        operation: &'static str,
    },

    /// Measurement reuse is forbidden by policy.
    ResetRequiredAfterMeasurement {
        allocation: AllocationId,
    },

    /// A release reason was supplied where the current ownership model does
    /// not permit release.
    ReleaseDenied {
        ownership: Ownership,
    },

    /// The lifetime generation does not match the expected generation.
    GenerationMismatch {
        expected: u64,
        actual: u64,
    },

    /// An attempt was made to create an invalid generation.
    GenerationOverflow,

    /// The supplied lease has already been consumed/released.
    LeaseConsumed,

    /// The resource was already terminal.
    AlreadyTerminal {
        state: LifetimeState,
    },

    /// The requested state operation cannot be performed because the state is
    /// unknown.
    UnknownQuantumState,

    /// A release operation requires explicit ownership.
    ExclusiveOwnershipRequired,

    /// An externally managed backend allocation must be released by its
    /// backend owner rather than this local lease.
    BackendManagedRelease,

    /// A distributed resource cannot be treated as a single local allocation
    /// without a distributed coordinator.
    DistributedCoordinationRequired,
}

impl fmt::Display for LifetimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationMismatch { expected, actual } => write!(
                f,
                "allocation mismatch: expected {expected}, got {actual}"
            ),

            Self::MemoryMismatch { expected, actual } => {
                write!(f, "memory mismatch: expected {expected}, got {actual}")
            }

            Self::MissingLogicalQubit => {
                f.write_str("lifetime record has no logical qubit association")
            }

            Self::LogicalQubitMismatch { expected, actual } => write!(
                f,
                "logical qubit mismatch: expected {expected}, got {actual}"
            ),

            Self::PhysicalQubitMismatch { expected, actual } => write!(
                f,
                "physical qubit mismatch: expected {expected}, got {actual}"
            ),

            Self::ClassicalBitMismatch { expected, actual } => write!(
                f,
                "classical measurement bit mismatch: expected {expected}, got {actual}"
            ),

            Self::Released { allocation } => {
                write!(f, "allocation {allocation} has been released")
            }

            Self::Invalid { allocation } => {
                write!(f, "allocation {allocation} has been invalidated")
            }

            Self::InvalidTransition { from, operation } => write!(
                f,
                "cannot perform {operation} while lifetime is {from}"
            ),

            Self::InvalidQuantumTransition { from, operation } => write!(
                f,
                "cannot perform {operation} while quantum memory state is {from}"
            ),

            Self::OwnershipDenied {
                ownership,
                operation,
            } => write!(
                f,
                "ownership mode {ownership:?} does not permit {operation}"
            ),

            Self::ResetRequiredAfterMeasurement { allocation } => write!(
                f,
                "allocation {allocation} requires reset before reuse"
            ),

            Self::ReleaseDenied { ownership } => write!(
                f,
                "release is not permitted under ownership mode {ownership:?}"
            ),

            Self::GenerationMismatch { expected, actual } => write!(
                f,
                "lifetime generation mismatch: expected {expected}, got {actual}"
            ),

            Self::GenerationOverflow => {
                f.write_str("lifetime generation counter overflow")
            }

            Self::LeaseConsumed => {
                f.write_str("memory lease has already been consumed")
            }

            Self::AlreadyTerminal { state } => {
                write!(f, "allocation is already terminal in state {state}")
            }

            Self::UnknownQuantumState => {
                f.write_str("quantum memory state is unknown")
            }

            Self::ExclusiveOwnershipRequired => {
                f.write_str("exclusive ownership is required for this operation")
            }

            Self::BackendManagedRelease => {
                f.write_str(
                    "backend-managed allocation must be released by its backend owner",
                )
            }

            Self::DistributedCoordinationRequired => {
                f.write_str(
                    "distributed memory operation requires distributed coordination",
                )
            }
        }
    }
}

impl std::error::Error for LifetimeError {}

/// Result type used by this module.
pub type LifetimeResult<T> = Result<T, LifetimeError>;

// =============================================================================
// Lifetime identity
// =============================================================================

/// Stable generation number for an allocation lifetime.
///
/// A generation prevents an old lease from being accidentally reused after a
/// memory resource has been recycled.
///
/// Generation `0` is the initial generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LifetimeGeneration(u64);

impl LifetimeGeneration {
    /// Initial generation.
    pub const INITIAL: Self = Self(0);

    /// Creates a generation.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric generation.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next generation.
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl fmt::Display for LifetimeGeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "generation {}", self.0)
    }
}

// =============================================================================
// Association
// =============================================================================

/// Quantum-resource association tracked by a lifetime record.
///
/// A memory allocation may be associated with a logical qubit, a physical
/// qubit, both, or neither.
///
/// The absence of an association is valid for representation-level memory
/// such as a state buffer, tensor workspace, or backend-native resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumAssociation {
    logical_qubit: Option<QubitId>,
    physical_qubit: Option<PhysicalQubitId>,
    measurement_bit: Option<ClassicalBitId>,
}

impl QuantumAssociation {
    /// Creates an unassociated memory resource.
    pub const fn unassociated() -> Self {
        Self {
            logical_qubit: None,
            physical_qubit: None,
            measurement_bit: None,
        }
    }

    /// Creates an association with a logical qubit.
    pub const fn logical(logical: QubitId) -> Self {
        Self {
            logical_qubit: Some(logical),
            physical_qubit: None,
            measurement_bit: None,
        }
    }

    /// Creates an association with a physical qubit.
    pub const fn physical(physical: PhysicalQubitId) -> Self {
        Self {
            logical_qubit: None,
            physical_qubit: Some(physical),
            measurement_bit: None,
        }
    }

    /// Creates a complete logical/physical association.
    pub const fn mapped(
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Self {
        Self {
            logical_qubit: Some(logical),
            physical_qubit: Some(physical),
            measurement_bit: None,
        }
    }

    /// Returns the logical-qubit association.
    pub const fn logical_qubit(self) -> Option<QubitId> {
        self.logical_qubit
    }

    /// Returns the physical-qubit association.
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        self.physical_qubit
    }

    /// Returns the measurement destination.
    pub const fn measurement_bit(self) -> Option<ClassicalBitId> {
        self.measurement_bit
    }

    /// Associates a classical measurement destination.
    pub const fn with_measurement_bit(
        self,
        bit: ClassicalBitId,
    ) -> Self {
        Self {
            logical_qubit: self.logical_qubit,
            physical_qubit: self.physical_qubit,
            measurement_bit: Some(bit),
        }
    }
}

impl Default for QuantumAssociation {
    fn default() -> Self {
        Self::unassociated()
    }
}

// =============================================================================
// Lifetime record
// =============================================================================

/// Complete lifecycle metadata for one memory allocation generation.
///
/// This type contains no actual quantum state.
///
/// It is therefore safe to use with:
///
/// - state vectors;
/// - density matrices;
/// - stabilizer tables;
/// - tensor networks;
/// - sparse states;
/// - GPU buffers;
/// - distributed buffers;
/// - backend-native state handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifetimeRecord {
    allocation_id: AllocationId,
    memory_id: MemoryId,
    generation: LifetimeGeneration,
    lifetime: LifetimeState,
    quantum_state: QuantumMemoryState,
    ownership: Ownership,
    reuse_policy: ReusePolicy,
    association: QuantumAssociation,
    measurement_count: u64,
}

impl LifetimeRecord {
    /// Creates a new exclusive memory lifetime record.
    ///
    /// The resource starts in `Allocated` state.
    pub const fn new(
        allocation_id: AllocationId,
        memory_id: MemoryId,
        association: QuantumAssociation,
    ) -> Self {
        Self {
            allocation_id,
            memory_id,
            generation: LifetimeGeneration::INITIAL,
            lifetime: LifetimeState::Allocated,
            quantum_state: QuantumMemoryState::Available,
            ownership: Ownership::Exclusive,
            reuse_policy: ReusePolicy::AllowAfterMeasurement,
            association,
            measurement_count: 0,
        }
    }

    /// Creates a backend-managed lifetime record.
    pub const fn backend_managed(
        allocation_id: AllocationId,
        memory_id: MemoryId,
        association: QuantumAssociation,
    ) -> Self {
        Self {
            allocation_id,
            memory_id,
            generation: LifetimeGeneration::INITIAL,
            lifetime: LifetimeState::Allocated,
            quantum_state: QuantumMemoryState::Available,
            ownership: Ownership::BackendManaged,
            reuse_policy: ReusePolicy::AllowAfterMeasurement,
            association,
            measurement_count: 0,
        }
    }

    /// Creates a distributed lifetime record.
    pub const fn distributed(
        allocation_id: AllocationId,
        memory_id: MemoryId,
        association: QuantumAssociation,
    ) -> Self {
        Self {
            allocation_id,
            memory_id,
            generation: LifetimeGeneration::INITIAL,
            lifetime: LifetimeState::Allocated,
            quantum_state: QuantumMemoryState::Available,
            ownership: Ownership::Distributed,
            reuse_policy: ReusePolicy::AllowAfterMeasurement,
            association,
            measurement_count: 0,
        }
    }

    /// Returns the allocation identity.
    pub const fn allocation_id(&self) -> AllocationId {
        self.allocation_id
    }

    /// Returns the memory identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the lifetime generation.
    pub const fn generation(&self) -> LifetimeGeneration {
        self.generation
    }

    /// Returns the allocation lifetime state.
    pub const fn lifetime(&self) -> LifetimeState {
        self.lifetime
    }

    /// Returns the quantum semantic state.
    pub const fn quantum_state(&self) -> QuantumMemoryState {
        self.quantum_state
    }

    /// Returns the ownership mode.
    pub const fn ownership(&self) -> Ownership {
        self.ownership
    }

    /// Returns the reuse policy.
    pub const fn reuse_policy(&self) -> ReusePolicy {
        self.reuse_policy
    }

    /// Returns the resource association.
    pub const fn association(&self) -> QuantumAssociation {
        self.association
    }

    /// Returns the logical qubit, if one is associated.
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        self.association.logical_qubit()
    }

    /// Returns the physical qubit, if one is associated.
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        self.association.physical_qubit()
    }

    /// Returns the measurement bit, if one is associated.
    pub const fn measurement_bit(&self) -> Option<ClassicalBitId> {
        self.association.measurement_bit()
    }

    /// Returns how many measurements have been recorded.
    pub const fn measurement_count(&self) -> u64 {
        self.measurement_count
    }

    /// Returns whether this resource can be used.
    pub const fn is_usable(&self) -> bool {
        self.lifetime.is_usable()
    }

    /// Returns whether this resource is terminal.
    pub const fn is_terminal(&self) -> bool {
        self.lifetime.is_terminal()
    }

    /// Changes the reuse policy.
    pub const fn with_reuse_policy(
        mut self,
        policy: ReusePolicy,
    ) -> Self {
        self.reuse_policy = policy;
        self
    }

    /// Changes the resource association.
    ///
    /// Association changes are metadata-only. They do not perform routing,
    /// physical allocation, or state migration.
    pub const fn with_association(
        mut self,
        association: QuantumAssociation,
    ) -> Self {
        self.association = association;
        self
    }

    /// Activates an allocated resource.
    pub fn activate(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_exclusive_lifecycle("activate")?;

        match self.lifetime {
            LifetimeState::Allocated => {
                self.lifetime = LifetimeState::Active;
                Ok(LifetimeEvent::Activated)
            }

            LifetimeState::Active => Ok(LifetimeEvent::Activated),

            state => Err(LifetimeError::InvalidTransition {
                from: state,
                operation: "activate",
            }),
        }
    }

    /// Places the resource into an explicit quiescent state.
    ///
    /// This is useful while a migration, synchronization, backend transition,
    /// or checkpoint is being coordinated.
    pub fn quiesce(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_exclusive_lifecycle("quiesce")?;

        match self.lifetime {
            LifetimeState::Allocated | LifetimeState::Active => {
                self.lifetime = LifetimeState::Quiescent;
                Ok(LifetimeEvent::Quiesced)
            }

            state => Err(LifetimeError::InvalidTransition {
                from: state,
                operation: "quiesce",
            }),
        }
    }

    /// Resumes an explicitly quiescent resource.
    pub fn resume(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_exclusive_lifecycle("resume")?;

        match self.lifetime {
            LifetimeState::Quiescent => {
                self.lifetime = LifetimeState::Active;
                Ok(LifetimeEvent::Resumed)
            }

            state => Err(LifetimeError::InvalidTransition {
                from: state,
                operation: "resume",
            }),
        }
    }

    /// Records a measurement event.
    ///
    /// Measurement does not release the allocation.
    pub fn record_measurement(
        &mut self,
    ) -> LifetimeResult<LifetimeEvent> {
        self.ensure_operational("measure")?;

        if self.quantum_state == QuantumMemoryState::Unknown {
            return Err(LifetimeError::UnknownQuantumState);
        }

        self.quantum_state = QuantumMemoryState::Measured;

        self.measurement_count = self
            .measurement_count
            .checked_add(1)
            .ok_or(LifetimeError::GenerationOverflow)?;

        match self.reuse_policy {
            ReusePolicy::ReleaseAfterMeasurement => {
                self.release(ReleaseReason::Measurement)
            }

            _ => Ok(LifetimeEvent::Measured),
        }
    }

    /// Records the beginning of a quantum-memory transition.
    ///
    /// This does not change the allocation lifetime.
    pub fn begin_transition(&mut self) -> LifetimeResult<()> {
        self.ensure_operational("begin transition")?;

        self.quantum_state = QuantumMemoryState::InTransition;

        Ok(())
    }

    /// Records the completion of a quantum-memory transition.
    pub fn end_transition(
        &mut self,
    ) -> LifetimeResult<()> {
        self.ensure_operational("end transition")?;

        self.quantum_state = QuantumMemoryState::Available;

        Ok(())
    }

    /// Records reset semantics.
    ///
    /// Reset returns the semantic state to an operational state without
    /// creating a new allocation.
    pub fn record_reset(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_operational("reset")?;

        self.quantum_state = QuantumMemoryState::Reset;

        Ok(LifetimeEvent::Reset)
    }

    /// Marks the semantic state available after reset or explicit
    /// reinitialization.
    pub fn mark_available(&mut self) -> LifetimeResult<()> {
        self.ensure_operational("mark available")?;

        self.quantum_state = QuantumMemoryState::Available;

        Ok(())
    }

    /// Marks the semantic state as unknown.
    ///
    /// This is intentionally irreversible through the ordinary state APIs.
    /// Recovery requires an explicit backend/state restoration operation.
    pub fn mark_unknown(&mut self) -> LifetimeResult<()> {
        if self.lifetime.is_terminal() {
            return Err(self.terminal_error());
        }

        self.quantum_state = QuantumMemoryState::Unknown;

        Ok(())
    }

    /// Releases this allocation.
    ///
    /// Backend-managed resources cannot be released by a local owner.
    /// Distributed resources require the distributed coordinator.
    pub fn release(
        &mut self,
        reason: ReleaseReason,
    ) -> LifetimeResult<LifetimeEvent> {
        match self.lifetime {
            LifetimeState::Released => {
                return Err(LifetimeError::AlreadyTerminal {
                    state: LifetimeState::Released,
                });
            }

            LifetimeState::Invalid => {
                return Err(LifetimeError::AlreadyTerminal {
                    state: LifetimeState::Invalid,
                });
            }

            _ => {}
        }

        match self.ownership {
            Ownership::Exclusive => {}

            Ownership::BackendManaged => {
                return Err(LifetimeError::BackendManagedRelease);
            }

            Ownership::Distributed => {
                return Err(LifetimeError::DistributedCoordinationRequired);
            }
        }

        self.lifetime = LifetimeState::Released;

        Ok(LifetimeEvent::Released(reason))
    }

    /// Releases a backend-managed resource after the backend has explicitly
    /// confirmed ownership termination.
    ///
    /// This is intentionally separate from `release()` so local code cannot
    /// accidentally release a resource it does not own.
    pub fn backend_release_confirmed(
        &mut self,
        reason: ReleaseReason,
    ) -> LifetimeResult<LifetimeEvent> {
        if self.ownership != Ownership::BackendManaged {
            return Err(LifetimeError::OwnershipDenied {
                ownership: self.ownership,
                operation: "backend-confirmed release",
            });
        }

        match self.lifetime {
            LifetimeState::Released => {
                return Err(LifetimeError::AlreadyTerminal {
                    state: LifetimeState::Released,
                });
            }

            LifetimeState::Invalid => {
                return Err(LifetimeError::AlreadyTerminal {
                    state: LifetimeState::Invalid,
                });
            }

            _ => {}
        }

        self.lifetime = LifetimeState::Released;

        Ok(LifetimeEvent::Released(reason))
    }

    /// Invalidates the resource.
    ///
    /// Invalid is stronger than Released because it indicates that state
    /// correctness can no longer be assumed.
    pub fn invalidate(
        &mut self,
        reason: InvalidationReason,
    ) -> LifetimeResult<LifetimeEvent> {
        if self.lifetime == LifetimeState::Invalid {
            return Err(LifetimeError::AlreadyTerminal {
                state: LifetimeState::Invalid,
            });
        }

        self.lifetime = LifetimeState::Invalid;
        self.quantum_state = QuantumMemoryState::Unknown;

        Ok(LifetimeEvent::Invalidated(reason))
    }

    /// Validates the allocation identity.
    pub fn validate_allocation(
        &self,
        allocation: AllocationId,
    ) -> LifetimeResult<()> {
        if self.allocation_id != allocation {
            return Err(LifetimeError::AllocationMismatch {
                expected: self.allocation_id,
                actual: allocation,
            });
        }

        Ok(())
    }

    /// Validates the memory-resource identity.
    pub fn validate_memory(
        &self,
        memory: MemoryId,
    ) -> LifetimeResult<()> {
        if self.memory_id != memory {
            return Err(LifetimeError::MemoryMismatch {
                expected: self.memory_id,
                actual: memory,
            });
        }

        Ok(())
    }

    /// Validates the lifetime generation.
    pub fn validate_generation(
        &self,
        generation: LifetimeGeneration,
    ) -> LifetimeResult<()> {
        if self.generation != generation {
            return Err(LifetimeError::GenerationMismatch {
                expected: self.generation.get(),
                actual: generation.get(),
            });
        }

        Ok(())
    }

    /// Validates that a logical qubit belongs to this record.
    pub fn validate_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> LifetimeResult<()> {
        match self.logical_qubit() {
            Some(expected) if expected == qubit => Ok(()),

            Some(expected) => Err(LifetimeError::LogicalQubitMismatch {
                expected,
                actual: qubit,
            }),

            None => Err(LifetimeError::MissingLogicalQubit),
        }
    }

    /// Validates the physical-qubit association.
    pub fn validate_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> LifetimeResult<()> {
        match self.physical_qubit() {
            Some(expected) if expected == qubit => Ok(()),

            Some(expected) => Err(LifetimeError::PhysicalQubitMismatch {
                expected,
                actual: qubit,
            }),

            None => Err(LifetimeError::PhysicalQubitMismatch {
                expected: qubit,
                actual: qubit,
            }),
        }
    }

    /// Validates the measurement destination.
    pub fn validate_measurement_bit(
        &self,
        bit: ClassicalBitId,
    ) -> LifetimeResult<()> {
        match self.measurement_bit() {
            Some(expected) if expected == bit => Ok(()),

            Some(expected) => Err(LifetimeError::ClassicalBitMismatch {
                expected,
                actual: bit,
            }),

            None => Err(LifetimeError::ClassicalBitMismatch {
                expected: bit,
                actual: bit,
            }),
        }
    }

    /// Advances the allocation to a new generation.
    ///
    /// The current generation must already be terminal.
    ///
    /// This method changes only lifecycle identity. It does not allocate
    /// physical memory.
    pub fn next_generation(
        &self,
    ) -> LifetimeResult<LifetimeGeneration> {
        if !self.lifetime.is_terminal() {
            return Err(LifetimeError::InvalidTransition {
                from: self.lifetime,
                operation: "advance generation",
            });
        }

        self.generation
            .checked_next()
            .ok_or(LifetimeError::GenerationOverflow)
    }

    fn ensure_operational(
        &self,
        operation: &'static str,
    ) -> LifetimeResult<()> {
        match self.lifetime {
            LifetimeState::Released => {
                Err(LifetimeError::Released {
                    allocation: self.allocation_id,
                })
            }

            LifetimeState::Invalid => {
                Err(LifetimeError::Invalid {
                    allocation: self.allocation_id,
                })
            }

            LifetimeState::Quiescent => {
                Err(LifetimeError::InvalidTransition {
                    from: LifetimeState::Quiescent,
                    operation,
                })
            }

            LifetimeState::Allocated | LifetimeState::Active => {
                if self.quantum_state == QuantumMemoryState::Unknown {
                    Err(LifetimeError::UnknownQuantumState)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn ensure_exclusive_lifecycle(
        &self,
        operation: &'static str,
    ) -> LifetimeResult<()> {
        match self.ownership {
            Ownership::Exclusive => Ok(()),

            Ownership::BackendManaged => {
                Err(LifetimeError::OwnershipDenied {
                    ownership: self.ownership,
                    operation,
                })
            }

            Ownership::Distributed => {
                Err(LifetimeError::DistributedCoordinationRequired)
            }
        }
    }

    fn terminal_error(&self) -> LifetimeError {
        match self.lifetime {
            LifetimeState::Released => LifetimeError::Released {
                allocation: self.allocation_id,
            },

            LifetimeState::Invalid => LifetimeError::Invalid {
                allocation: self.allocation_id,
            },

            _ => LifetimeError::InvalidTransition {
                from: self.lifetime,
                operation: "terminal operation",
            },
        }
    }
}

// =============================================================================
// Memory lease
// =============================================================================

/// Exclusive ownership token for a memory lifetime.
///
/// The lease intentionally does not implement `Clone` or `Copy`.
///
/// Dropping a `MemoryLease` does **not** silently release the resource because
/// automatic release would make backend-managed and distributed semantics
/// unsafe and would hide lifecycle bugs.
///
/// Call [`MemoryLease::release`] explicitly.
#[derive(Debug, PartialEq, Eq)]
pub struct MemoryLease {
    record: LifetimeRecord,
    consumed: bool,
}

impl MemoryLease {
    /// Creates a new exclusive lease.
    pub fn new(record: LifetimeRecord) -> LifetimeResult<Self> {
        if record.ownership() != Ownership::Exclusive {
            return Err(LifetimeError::OwnershipDenied {
                ownership: record.ownership(),
                operation: "create exclusive memory lease",
            });
        }

        Ok(Self {
            record,
            consumed: false,
        })
    }

    /// Returns an immutable view of the lifetime record.
    pub const fn record(&self) -> &LifetimeRecord {
        &self.record
    }

    /// Returns the allocation identity.
    pub const fn allocation_id(&self) -> AllocationId {
        self.record.allocation_id()
    }

    /// Returns the memory identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.record.memory_id()
    }

    /// Returns the current generation.
    pub const fn generation(&self) -> LifetimeGeneration {
        self.record.generation()
    }

    /// Returns the current lifetime state.
    pub const fn lifetime(&self) -> LifetimeState {
        self.record.lifetime()
    }

    /// Returns the current semantic state.
    pub const fn quantum_state(&self) -> QuantumMemoryState {
        self.record.quantum_state()
    }

    /// Returns whether the lease has been explicitly consumed.
    pub const fn is_consumed(&self) -> bool {
        self.consumed
    }

    /// Activates the resource.
    pub fn activate(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_not_consumed()?;
        self.record.activate()
    }

    /// Quiesces the resource.
    pub fn quiesce(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_not_consumed()?;
        self.record.quiesce()
    }

    /// Resumes a quiescent resource.
    pub fn resume(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_not_consumed()?;
        self.record.resume()
    }

    /// Records measurement.
    pub fn record_measurement(
        &mut self,
    ) -> LifetimeResult<LifetimeEvent> {
        self.ensure_not_consumed()?;
        self.record.record_measurement()
    }

    /// Records reset.
    pub fn record_reset(&mut self) -> LifetimeResult<LifetimeEvent> {
        self.ensure_not_consumed()?;
        self.record.record_reset()
    }

    /// Marks the quantum state available.
    pub fn mark_available(&mut self) -> LifetimeResult<()> {
        self.ensure_not_consumed()?;
        self.record.mark_available()
    }

    /// Begins a state transition.
    pub fn begin_transition(&mut self) -> LifetimeResult<()> {
        self.ensure_not_consumed()?;
        self.record.begin_transition()
    }

    /// Ends a state transition.
    pub fn end_transition(&mut self) -> LifetimeResult<()> {
        self.ensure_not_consumed()?;
        self.record.end_transition()
    }

    /// Marks the state unknown.
    pub fn mark_unknown(&mut self) -> LifetimeResult<()> {
        self.ensure_not_consumed()?;
        self.record.mark_unknown()
    }

    /// Explicitly releases the owned allocation.
    ///
    /// Consuming the lease prevents accidental reuse of an old lifetime token.
    pub fn release(
        mut self,
        reason: ReleaseReason,
    ) -> LifetimeResult<LifetimeRecord> {
        self.ensure_not_consumed()?;

        self.record.release(reason)?;
        self.consumed = true;

        Ok(self.record)
    }

    fn ensure_not_consumed(&self) -> LifetimeResult<()> {
        if self.consumed {
            Err(LifetimeError::LeaseConsumed)
        } else {
            Ok(())
        }
    }
}

// =============================================================================
// Lifetime validator
// =============================================================================

/// Stateless lifecycle validator.
///
/// This type exists so higher-level modules can validate transitions without
/// taking ownership of a `LifetimeRecord`.
///
/// It intentionally contains no global policy and no mutable state.
#[derive(Debug, Clone, Copy, Default)]
pub struct LifetimeValidator;

impl LifetimeValidator {
    /// Creates a validator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates whether a resource can be used.
    pub const fn can_use(
        &self,
        lifetime: LifetimeState,
        quantum_state: QuantumMemoryState,
    ) -> bool {
        lifetime.is_usable() && quantum_state.is_operational()
    }

    /// Validates whether measurement is possible.
    pub const fn can_measure(
        &self,
        lifetime: LifetimeState,
        quantum_state: QuantumMemoryState,
    ) -> bool {
        lifetime.is_usable()
            && !matches!(quantum_state, QuantumMemoryState::Unknown)
    }

    /// Validates whether reset is possible.
    pub const fn can_reset(
        &self,
        lifetime: LifetimeState,
        quantum_state: QuantumMemoryState,
    ) -> bool {
        lifetime.is_usable()
            && !matches!(
                quantum_state,
                QuantumMemoryState::Unknown
                    | QuantumMemoryState::InTransition
            )
    }

    /// Validates whether a resource may be released under its ownership mode.
    pub const fn can_release(
        &self,
        lifetime: LifetimeState,
        ownership: Ownership,
    ) -> bool {
        lifetime.is_allocated()
            && matches!(ownership, Ownership::Exclusive)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn allocation() -> AllocationId {
        AllocationId::new(1)
    }

    fn memory() -> MemoryId {
        MemoryId::new(2)
    }

    fn logical() -> QubitId {
        QubitId::new(3)
    }

    fn physical() -> PhysicalQubitId {
        PhysicalQubitId::new(4)
    }

    fn record() -> LifetimeRecord {
        LifetimeRecord::new(
            allocation(),
            memory(),
            QuantumAssociation::mapped(logical(), physical()),
        )
    }

    #[test]
    fn new_record_is_allocated_and_available() {
        let record = record();

        assert_eq!(record.lifetime(), LifetimeState::Allocated);
        assert_eq!(
            record.quantum_state(),
            QuantumMemoryState::Available
        );
        assert_eq!(record.ownership(), Ownership::Exclusive);
        assert_eq!(
            record.generation(),
            LifetimeGeneration::INITIAL
        );
        assert_eq!(record.measurement_count(), 0);
    }

    #[test]
    fn activation_transitions_to_active() {
        let mut record = record();

        let event = record.activate().expect("activation should succeed");

        assert_eq!(event, LifetimeEvent::Activated);
        assert_eq!(record.lifetime(), LifetimeState::Active);
    }

    #[test]
    fn measurement_does_not_release_memory_by_default() {
        let mut record = record();

        record.activate().expect("activation should succeed");

        let event = record
            .record_measurement()
            .expect("measurement should succeed");

        assert_eq!(event, LifetimeEvent::Measured);
        assert_eq!(record.lifetime(), LifetimeState::Active);
        assert_eq!(
            record.quantum_state(),
            QuantumMemoryState::Measured
        );
        assert_eq!(record.measurement_count(), 1);
    }

    #[test]
    fn measurement_can_be_reused_after_reset() {
        let mut record = record();

        record.activate().expect("activation should succeed");
        record
            .record_measurement()
            .expect("measurement should succeed");

        record
            .record_reset()
            .expect("reset should succeed");

        assert_eq!(
            record.quantum_state(),
            QuantumMemoryState::Reset
        );

        record
            .mark_available()
            .expect("availability should succeed");

        assert_eq!(
            record.quantum_state(),
            QuantumMemoryState::Available
        );
        assert_eq!(record.lifetime(), LifetimeState::Active);
    }

    #[test]
    fn measurement_release_policy_releases() {
        let mut record = record().with_reuse_policy(
            ReusePolicy::ReleaseAfterMeasurement,
        );

        record.activate().expect("activation should succeed");

        let event = record
            .record_measurement()
            .expect("measurement should release");

        assert_eq!(
            event,
            LifetimeEvent::Released(ReleaseReason::Measurement)
        );
        assert_eq!(record.lifetime(), LifetimeState::Released);
    }

    #[test]
    fn release_is_terminal() {
        let mut record = record();

        record
            .release(ReleaseReason::Explicit)
            .expect("release should succeed");

        assert_eq!(record.lifetime(), LifetimeState::Released);

        let result = record.activate();

        assert!(matches!(
            result,
            Err(LifetimeError::Released { .. })
        ));
    }

    #[test]
    fn invalidation_makes_quantum_state_unknown() {
        let mut record = record();

        record
            .invalidate(InvalidationReason::IntegrityFailure)
            .expect("invalidation should succeed");

        assert_eq!(record.lifetime(), LifetimeState::Invalid);
        assert_eq!(
            record.quantum_state(),
            QuantumMemoryState::Unknown
        );
    }

    #[test]
    fn backend_managed_resources_cannot_be_locally_released() {
        let mut record = LifetimeRecord::backend_managed(
            allocation(),
            memory(),
            QuantumAssociation::mapped(logical(), physical()),
        );

        let result = record.release(ReleaseReason::Explicit);

        assert!(matches!(
            result,
            Err(LifetimeError::BackendManagedRelease)
        ));
    }

    #[test]
    fn_backend_managed_release_requires_confirmation() {
        let mut record = LifetimeRecord::backend_managed(
            allocation(),
            memory(),
            QuantumAssociation::mapped(logical(), physical()),
        );

        record
            .backend_release_confirmed(ReleaseReason::BackendCompleted)
            .expect("backend confirmation should release");

        assert_eq!(record.lifetime(), LifetimeState::Released);
    }

    #[test]
    fn distributed_resources_require_coordination() {
        let mut record = LifetimeRecord::distributed(
            allocation(),
            memory(),
            QuantumAssociation::mapped(logical(), physical()),
        );

        let result = record.release(ReleaseReason::Explicit);

        assert!(matches!(
            result,
            Err(LifetimeError::DistributedCoordinationRequired)
        ));
    }

    #[test]
    fn quiescent_resources_cannot_be_used() {
        let mut record = record();

        record.quiesce().expect("quiescence should succeed");

        assert!(!record.is_usable());

        let result = record.record_reset();

        assert!(matches!(
            result,
            Err(LifetimeError::InvalidTransition {
                from: LifetimeState::Quiescent,
                ..
            })
        ));
    }

    #[test]
    fn resume_restores_use() {
        let mut record = record();

        record.quiesce().expect("quiescence should succeed");
        record.resume().expect("resume should succeed");

        assert_eq!(record.lifetime(), LifetimeState::Active);
        assert!(record.is_usable());
    }

    #[test]
    fn association_validation_works() {
        let record = record();

        assert!(
            record.validate_logical_qubit(logical()).is_ok()
        );

        assert!(
            record.validate_physical_qubit(physical()).is_ok()
        );

        assert!(
            record
                .validate_logical_qubit(QubitId::new(99))
                .is_err()
        );
    }

    #[test]
    fn measurement_count_is_monotonic() {
        let mut record = record();

        record
            .record_measurement()
            .expect("measurement should succeed");

        record
            .record_reset()
            .expect("reset should succeed");

        record
            .record_measurement()
            .expect("second measurement should succeed");

        assert_eq!(record.measurement_count(), 2);
    }

    #[test]
    fn generation_increments_only_after_terminal_state() {
        let mut record = record();

        assert!(matches!(
            record.next_generation(),
            Err(LifetimeError::InvalidTransition { .. })
        ));

        record
            .release(ReleaseReason::Explicit)
            .expect("release should succeed");

        let generation = record
            .next_generation()
            .expect("next generation should succeed");

        assert_eq!(generation.get(), 1);
    }

    #[test]
    fn lease_is_explicitly_released() {
        let mut lease = MemoryLease::new(record())
            .expect("exclusive record should create a lease");

        lease
            .activate()
            .expect("activation should succeed");

        let record = lease
            .release(ReleaseReason::Explicit)
            .expect("lease release should succeed");

        assert_eq!(record.lifetime(), LifetimeState::Released);
    }

    #[test]
    fn backend_record_cannot_become_exclusive_lease() {
        let record = LifetimeRecord::backend_managed(
            allocation(),
            memory(),
            QuantumAssociation::unassociated(),
        );

        let result = MemoryLease::new(record);

        assert!(matches!(
            result,
            Err(LifetimeError::OwnershipDenied { .. })
        ));
    }

    #[test]
    fn validator_matches_lifecycle_semantics() {
        let validator = LifetimeValidator::new();

        assert!(validator.can_use(
            LifetimeState::Active,
            QuantumMemoryState::Available
        ));

        assert!(validator.can_use(
            LifetimeState::Active,
            QuantumMemoryState::Measured
        ));

        assert!(!validator.can_use(
            LifetimeState::Released,
            QuantumMemoryState::Available
        ));

        assert!(!validator.can_use(
            LifetimeState::Active,
            QuantumMemoryState::Unknown
        ));
    }
}