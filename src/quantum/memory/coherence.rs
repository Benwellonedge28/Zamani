//! Zamani Quantum Memory — Coherence and Consistency Contract
//!
//! Production-grade, provider-neutral coherence model for quantum memory.
//!
//! # Responsibility
//!
//! This module owns the *coherence protocol* for quantum-memory resources.
//! It does not own allocation, state mathematics, synchronization transport,
//! GPU APIs, distributed networking, routing, scheduling, or QPU SDKs.
//!
//! The coherence layer answers questions such as:
//!
//! - Which memory location currently owns the authoritative state?
//! - Which copies are clean, dirty, stale, or invalid?
//! - Which generation/version does a copy represent?
//! - Is a read permitted?
//! - Is a write permitted?
//! - Can a host/device/distributed/backend copy be synchronized?
//! - Is a synchronization request stale?
//! - Has concurrent access produced a conflict?
//! - Can a remote QPU resource participate without pretending that it has
//!   addressable quantum RAM?
//!
//! It deliberately supports:
//!
//! - CPU memory;
//! - pinned host memory;
//! - GPU/device memory;
//! - unified memory;
//! - distributed simulation memory;
//! - remote simulators;
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom QPUs;
//! - photonic processors;
//! - spin/semiconductor devices;
//! - topological devices;
//! - logical/fault-tolerant QPUs;
//! - analog quantum processors;
//! - quantum annealers;
//! - networked quantum execution;
//! - future provider-defined execution targets.
//!
//! No vendor-specific API is required.
//!
//! # Architectural position
//!
//! ```text
//!                       quantum::ir
//!                            |
//!                            v
//!                    execution planning
//!                            |
//!             +--------------+--------------+
//!             |              |              |
//!             v              v              v
//!           CPU            GPU             QPU
//!             |              |              |
//!             +--------------+--------------+
//!                            |
//!                            v
//!                  quantum::memory::coherence
//!                            |
//!          +-----------------+-----------------+
//!          |                 |                 |
//!          v                 v                 v
//!      ownership          versions         permissions
//!          |                 |                 |
//!          +-----------------+-----------------+
//!                            |
//!                            v
//!                    synchronization.rs
//! ```
//!
//! `coherence.rs` describes *what must be true*.
//!
//! `synchronization.rs` is responsible for *making it true*.
//!
//! `allocator.rs`, `gpu.rs`, `distributed.rs`, and `backend_state.rs` own
//! resource-specific implementations.
//!
//! # Critical quantum rule
//!
//! A real QPU frequently does not expose a locally readable copy of the
//! quantum state. Therefore coherence MUST NOT imply that every location has
//! an amplitude buffer.
//!
//! For a QPU, coherence may instead describe:
//!
//! ```text
//! provider-owned execution state
//!        |
//!        +-- generation
//!        +-- execution epoch
//!        +-- logical/physical mapping
//!        +-- classical results
//!        +-- synchronization token
//!        +-- opaque backend resource identity
//! ```
//!
//! The model therefore separates:
//!
//! - memory location;
//! - state representation;
//! - state ownership;
//! - synchronization capability;
//! - observability.
//!
//! # State consistency integration
//!
//! `state.rs` already owns the representation-neutral `StateConsistency`
//! vocabulary. This module does NOT redefine it.
//!
//! Instead, this module provides the detailed coherence protocol and can map
//! its detailed state to `StateConsistency` where required.
//!
//! # Backend integration
//!
//! `backend_state.rs` owns provider-neutral opaque backend resources and its
//! synchronization token contract.
//!
//! This module does not depend on a provider SDK and does not dereference,
//! inspect, or fabricate provider state.
//!
//! # Safety
//!
//! No unsafe Rust is used.
//!
//! `unsafe_code` is denied explicitly.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Completion invariant
//!
//! Once this file is complete, later modules should integrate with its public
//! contracts rather than modifying its semantics merely because a new CPU,
//! GPU, QPU, simulator, distributed provider, or memory representation is
//! introduced.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::time::Duration;

use super::errors::MemoryError;
use super::state::StateConsistency;
use super::types::{ByteCount, MemoryId, StateId};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the coherence protocol.
pub const COHERENCE_SCHEMA_ID: &str = "zamani.quantum.memory.coherence";

/// Semantic version of the coherence contract.
pub const COHERENCE_SCHEMA_VERSION: u16 = 1;

/// Maximum number of locations tracked by one coherence domain.
pub const MAX_COHERENCE_LOCATIONS: usize = 1_000_000;

/// Maximum provider/device identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend/device/resource identifier length.
pub const MAX_RESOURCE_ID_LENGTH: usize = 1024;

/// Maximum custom coherence-domain name.
pub const MAX_DOMAIN_NAME_LENGTH: usize = 256;

// =============================================================================
// Result
// =============================================================================

/// Canonical result type for coherence operations.
pub type CoherenceResult<T> = Result<T, MemoryError>;

// =============================================================================
// Coherence location
// =============================================================================

/// Provider-neutral class of a memory location participating in coherence.
///
/// This is intentionally broader than local RAM because a quantum execution
/// system may have no directly addressable quantum memory on the QPU at all.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoherenceLocation {
    /// Ordinary CPU/host memory.
    Host,

    /// Page-locked/pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Memory whose ownership is shared by host and device.
    Unified,

    /// Memory partition owned by a distributed execution node.
    DistributedNode(u64),

    /// Provider-owned remote execution state.
    RemoteProvider,

    /// Opaque external state managed by an execution provider.
    External,

    /// State for which the underlying storage is intentionally opaque.
    Opaque,

    /// Future/provider-defined location.
    Custom(String),
}

impl CoherenceLocation {
    /// Creates a provider-defined location.
    pub fn custom(value: impl Into<String>) -> CoherenceResult<Self> {
        let value = value.into();

        validate_name(&value, MAX_DOMAIN_NAME_LENGTH, "custom coherence location")?;

        Ok(Self::Custom(value))
    }

    /// Returns true if this location is directly addressable by Zamani's
    /// local memory subsystem.
    pub const fn is_locally_addressable(&self) -> bool {
        matches!(
            self,
            Self::Host | Self::PinnedHost | Self::Device | Self::Unified
        )
    }

    /// Returns true if this location can represent an external provider.
    pub const fn is_external(&self) -> bool {
        matches!(
            self,
            Self::RemoteProvider | Self::External | Self::Opaque
        )
    }

    /// Returns true if this is distributed memory.
    pub const fn is_distributed(&self) -> bool {
        matches!(self, Self::DistributedNode(_))
    }

    /// Returns a stable identifier suitable for diagnostics and telemetry.
    pub fn as_str(&self) -> String {
        match self {
            Self::Host => "host".to_owned(),
            Self::PinnedHost => "pinned_host".to_owned(),
            Self::Device => "device".to_owned(),
            Self::Unified => "unified".to_owned(),
            Self::DistributedNode(node) => format!("distributed:{node}"),
            Self::RemoteProvider => "remote_provider".to_owned(),
            Self::External => "external".to_owned(),
            Self::Opaque => "opaque".to_owned(),
            Self::Custom(value) => value.clone(),
        }
    }
}

impl fmt::Display for CoherenceLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}

// =============================================================================
// Location identity
// =============================================================================

/// Stable identity for a coherence participant.
///
/// A location identity is distinct from a raw address. No pointer or physical
/// memory address is ever exposed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoherenceLocationId {
    domain: String,
    location: CoherenceLocation,
}

impl CoherenceLocationId {
    /// Creates a location identity.
    pub fn new(
        domain: impl Into<String>,
        location: CoherenceLocation,
    ) -> CoherenceResult<Self> {
        let domain = domain.into();

        validate_name(&domain, MAX_DOMAIN_NAME_LENGTH, "coherence domain")?;

        Ok(Self { domain, location })
    }

    /// Returns the coherence domain.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the location.
    pub fn location(&self) -> &CoherenceLocation {
        &self.location
    }
}

impl fmt::Display for CoherenceLocationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.domain, self.location)
    }
}

// =============================================================================
// Access mode
// =============================================================================

/// Access mode requested against a coherence participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoherenceAccess {
    /// Read-only access.
    Read,

    /// Exclusive mutation.
    Write,

    /// Read followed by mutation.
    ReadWrite,

    /// Observation only; does not imply local quantum-state visibility.
    Observe,

    /// Provider-controlled execution access.
    Execute,
}

impl CoherenceAccess {
    /// Returns true if the access can modify state.
    pub const fn is_write_capable(self) -> bool {
        matches!(self, Self::Write | Self::ReadWrite | Self::Execute)
    }

    /// Returns true if the access requires a readable local representation.
    pub const fn requires_local_read(self) -> bool {
        matches!(self, Self::Read | Self::ReadWrite)
    }

    /// Returns true if this is observation rather than state access.
    pub const fn is_observation(self) -> bool {
        matches!(self, Self::Observe)
    }
}

// =============================================================================
// Authority
// =============================================================================

/// Which participant is currently authoritative for a quantum-memory resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoherenceAuthority {
    /// Host memory is authoritative.
    Host,

    /// Device memory is authoritative.
    Device,

    /// A distributed partition set is authoritative.
    Distributed,

    /// An external/remote provider is authoritative.
    External,

    /// No single location is authoritative; the protocol manages a coherent
    /// replicated state.
    Shared,

    /// Authority is intentionally unavailable/opaque.
    Opaque,
}

impl CoherenceAuthority {
    /// Returns true if authority is external to Zamani's local memory.
    pub const fn is_external(self) -> bool {
        matches!(self, Self::External | Self::Opaque)
    }
}

impl fmt::Display for CoherenceAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Host => "host",
            Self::Device => "device",
            Self::Distributed => "distributed",
            Self::External => "external",
            Self::Shared => "shared",
            Self::Opaque => "opaque",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Copy state
// =============================================================================

/// Coherence status of one participating copy/resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoherenceCopyState {
    /// Copy exactly represents the current committed generation.
    Clean,

    /// Copy contains a newer local mutation that has not been propagated.
    Dirty,

    /// Copy is older than the authoritative generation.
    Stale,

    /// Copy is known to be unusable until explicitly refreshed.
    Invalid,

    /// Copy is currently being synchronized.
    Synchronizing,

    /// Copy has been detached from the coherence domain.
    Detached,

    /// Provider controls the state and does not expose copy semantics.
    ProviderManaged,
}

impl CoherenceCopyState {
    /// Returns true if the copy can safely be read under the current version.
    pub const fn is_readable(self) -> bool {
        matches!(
            self,
            Self::Clean | Self::Dirty | Self::ProviderManaged
        )
    }

    /// Returns true if synchronization is required before treating this copy
    /// as current.
    pub const fn requires_refresh(self) -> bool {
        matches!(self, Self::Stale | Self::Invalid)
    }

    /// Returns true if the copy is currently unavailable.
    pub const fn is_unavailable(self) -> bool {
        matches!(
            self,
            Self::Invalid | Self::Synchronizing | Self::Detached
        )
    }
}

impl fmt::Display for CoherenceCopyState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Clean => "clean",
            Self::Dirty => "dirty",
            Self::Stale => "stale",
            Self::Invalid => "invalid",
            Self::Synchronizing => "synchronizing",
            Self::Detached => "detached",
            Self::ProviderManaged => "provider_managed",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Version / generation
// =============================================================================

/// Monotonically increasing coherence generation.
///
/// Generation zero represents an uninitialized coherence domain. A committed
/// state begins at generation one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CoherenceGeneration(u64);

impl CoherenceGeneration {
    /// Uninitialized generation.
    pub const ZERO: Self = Self(0);

    /// First committed generation.
    pub const INITIAL: Self = Self(1);

    /// Creates a generation.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric generation.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns true for the uninitialized generation.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next generation.
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns true when `self` is newer than `other`.
    pub const fn is_newer_than(self, other: Self) -> bool {
        self.0 > other.0
    }

    /// Returns true when `self` is older than `other`.
    pub const fn is_older_than(self, other: Self) -> bool {
        self.0 < other.0
    }
}

impl fmt::Display for CoherenceGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Epoch
// =============================================================================

/// Execution epoch used to reject operations belonging to a previous
/// lifecycle/session.
///
/// Generations identify state changes; epochs identify lifecycle boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CoherenceEpoch(u64);

impl CoherenceEpoch {
    /// Initial epoch.
    pub const INITIAL: Self = Self(1);

    /// Creates an epoch.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric value.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next epoch.
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

// =============================================================================
// Coherence token
// =============================================================================

/// Immutable token representing the coherence view observed by an operation.
///
/// This token is deliberately opaque with respect to physical addresses.
/// It can safely be carried across:
///
/// - synchronization layers;
/// - execution queues;
/// - backend adapters;
/// - distributed transport;
/// - checkpoint metadata.
///
/// It cannot be used to access provider memory directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoherenceToken {
    state_id: StateId,
    memory_id: MemoryId,
    generation: CoherenceGeneration,
    epoch: CoherenceEpoch,
    location: CoherenceLocationId,
}

impl CoherenceToken {
    /// Creates a token.
    pub fn new(
        state_id: StateId,
        memory_id: MemoryId,
        generation: CoherenceGeneration,
        epoch: CoherenceEpoch,
        location: CoherenceLocationId,
    ) -> Self {
        Self {
            state_id,
            memory_id,
            generation,
            epoch,
            location,
        }
    }

    /// Returns the state identity.
    pub fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns the memory identity.
    pub fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the generation.
    pub fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns the lifecycle epoch.
    pub fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns the location.
    pub fn location(&self) -> &CoherenceLocationId {
        &self.location
    }
}

// =============================================================================
// Synchronization direction
// =============================================================================

/// Direction of a synchronization operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SynchronizationDirection {
    /// Source becomes authoritative at destination.
    SourceToDestination,

    /// Destination becomes authoritative at source.
    DestinationToSource,

    /// Both locations are reconciled under an explicit conflict policy.
    Bidirectional,

    /// Synchronization is performed by an external provider.
    ProviderManaged,
}

impl fmt::Display for SynchronizationDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::SourceToDestination => "source_to_destination",
            Self::DestinationToSource => "destination_to_source",
            Self::Bidirectional => "bidirectional",
            Self::ProviderManaged => "provider_managed",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Synchronization intent
// =============================================================================

/// Why synchronization is being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SynchronizationReason {
    /// Prepare a destination for reading.
    ReadPreparation,

    /// Commit a local mutation.
    WriteCommit,

    /// Transfer state before execution.
    ExecutionPreparation,

    /// Retrieve execution results.
    ExecutionCompletion,

    /// Create a consistent checkpoint.
    Checkpoint,

    /// Create a consistent snapshot.
    Snapshot,

    /// Migrate state between representations or devices.
    Migration,

    /// Explicit user/runtime request.
    Explicit,

    /// Recover from a stale or invalid copy.
    Recovery,

    /// Provider-controlled lifecycle operation.
    ProviderLifecycle,
}

// =============================================================================
// Conflict policy
// =============================================================================

/// Policy used when two participants disagree about the current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConflictPolicy {
    /// Reject the operation rather than guessing.
    Reject,

    /// Accept only when the source generation is strictly newer.
    NewerGenerationWins,

    /// Accept an explicitly designated authoritative location.
    AuthorityWins,

    /// Reconcile using a provider-defined operation.
    ProviderManaged,
}

impl Default for ConflictPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

// =============================================================================
// Coherence capability
// =============================================================================

/// Capabilities exposed by a coherence participant.
///
/// This capability model is intentionally orthogonal to quantum-state
/// representation. A QPU can therefore expose execution/coherence capabilities
/// without exposing amplitudes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CoherenceCapabilities {
    /// Participant can be read by Zamani.
    pub readable: bool,

    /// Participant can be mutated by Zamani.
    pub writable: bool,

    /// Participant supports explicit synchronization.
    pub synchronization: bool,

    /// Participant supports generation/version checks.
    pub versioned: bool,

    /// Participant supports snapshot-like capture.
    pub snapshot: bool,

    /// Participant supports checkpoint-like capture.
    pub checkpoint: bool,

    /// Participant supports migration.
    pub migration: bool,

    /// Participant is externally/provider managed.
    pub provider_managed: bool,

    /// Participant can participate in distributed coherence.
    pub distributed: bool,
}

impl CoherenceCapabilities {
    /// Capabilities for ordinary host memory.
    pub const fn host() -> Self {
        Self {
            readable: true,
            writable: true,
            synchronization: true,
            versioned: true,
            snapshot: true,
            checkpoint: true,
            migration: true,
            provider_managed: false,
            distributed: false,
        }
    }

    /// Capabilities for a typical GPU/device memory participant.
    pub const fn device() -> Self {
        Self {
            readable: true,
            writable: true,
            synchronization: true,
            versioned: true,
            snapshot: true,
            checkpoint: true,
            migration: true,
            provider_managed: false,
            distributed: false,
        }
    }

    /// Capabilities for a provider-owned QPU.
    ///
    /// Notice that `readable` is false: the QPU need not expose quantum
    /// amplitudes or local quantum memory.
    pub const fn provider_qpu() -> Self {
        Self {
            readable: false,
            writable: false,
            synchronization: true,
            versioned: true,
            snapshot: false,
            checkpoint: true,
            migration: false,
            provider_managed: true,
            distributed: false,
        }
    }

    /// Capabilities for distributed memory.
    pub const fn distributed() -> Self {
        Self {
            readable: true,
            writable: true,
            synchronization: true,
            versioned: true,
            snapshot: true,
            checkpoint: true,
            migration: true,
            provider_managed: false,
            distributed: true,
        }
    }

    /// Determines whether a requested access is permitted.
    pub const fn allows(self, access: CoherenceAccess) -> bool {
        match access {
            CoherenceAccess::Read => self.readable,
            CoherenceAccess::Write => self.writable,
            CoherenceAccess::ReadWrite => self.readable && self.writable,
            CoherenceAccess::Observe => true,
            CoherenceAccess::Execute => self.writable || self.provider_managed,
        }
    }
}

// =============================================================================
// Participant
// =============================================================================

/// One participant in a coherence domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoherenceParticipant {
    id: CoherenceLocationId,
    state: CoherenceCopyState,
    generation: CoherenceGeneration,
    capabilities: CoherenceCapabilities,
    reserved_bytes: ByteCount,
}

impl CoherenceParticipant {
    /// Creates a participant.
    pub fn new(
        id: CoherenceLocationId,
        state: CoherenceCopyState,
        generation: CoherenceGeneration,
        capabilities: CoherenceCapabilities,
        reserved_bytes: ByteCount,
    ) -> Self {
        Self {
            id,
            state,
            generation,
            capabilities,
            reserved_bytes,
        }
    }

    /// Returns the participant identity.
    pub fn id(&self) -> &CoherenceLocationId {
        &self.id
    }

    /// Returns the copy state.
    pub const fn state(&self) -> CoherenceCopyState {
        self.state
    }

    /// Returns the generation represented by this participant.
    pub const fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns its capabilities.
    pub const fn capabilities(&self) -> CoherenceCapabilities {
        self.capabilities
    }

    /// Returns reserved bytes.
    pub const fn reserved_bytes(&self) -> ByteCount {
        self.reserved_bytes
    }

    /// Returns whether this participant represents the current generation.
    pub const fn is_current(&self, generation: CoherenceGeneration) -> bool {
        self.generation == generation
    }
}

// =============================================================================
// Coherence transition
// =============================================================================

/// Validated state transition in the coherence protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoherenceTransition {
    /// Mark a clean/current copy dirty after a mutation.
    BeginWrite,

    /// Commit a dirty copy as the new authoritative generation.
    CommitWrite,

    /// Mark a copy stale because another participant became authoritative.
    MarkStale,

    /// Begin synchronization.
    BeginSynchronization,

    /// Complete synchronization successfully.
    CompleteSynchronization,

    /// Invalidate a participant.
    Invalidate,

    /// Detach a participant.
    Detach,

    /// Restore a participant to a known generation.
    Refresh,
}

// =============================================================================
// Coherence domain
// =============================================================================

/// Complete coherence state for one logical quantum-memory resource.
///
/// This object contains only metadata and protocol state. It never contains
/// quantum amplitudes or raw memory addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoherenceDomain {
    state_id: StateId,
    memory_id: MemoryId,
    epoch: CoherenceEpoch,
    generation: CoherenceGeneration,
    authority: CoherenceAuthority,
    consistency: StateConsistency,
    conflict_policy: ConflictPolicy,
    participants: Vec<CoherenceParticipant>,
}

impl CoherenceDomain {
    /// Creates a new coherence domain.
    ///
    /// The initial state is:
    ///
    /// - epoch = INITIAL;
    /// - generation = INITIAL;
    /// - no participants;
    /// - provider-independent consistency.
    pub fn new(
        state_id: StateId,
        memory_id: MemoryId,
        authority: CoherenceAuthority,
        conflict_policy: ConflictPolicy,
    ) -> Self {
        Self {
            state_id,
            memory_id,
            epoch: CoherenceEpoch::INITIAL,
            generation: CoherenceGeneration::INITIAL,
            authority,
            consistency: StateConsistency::Consistent,
            conflict_policy,
            participants: Vec::new(),
        }
    }

    /// Returns the state identity.
    pub fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns the memory identity.
    pub fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the lifecycle epoch.
    pub const fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns the current committed generation.
    pub const fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns the authority.
    pub const fn authority(&self) -> CoherenceAuthority {
        self.authority
    }

    /// Returns the detailed state consistency model used by `state.rs`.
    pub const fn consistency(&self) -> StateConsistency {
        self.consistency
    }

    /// Returns the conflict policy.
    pub const fn conflict_policy(&self) -> ConflictPolicy {
        self.conflict_policy
    }

    /// Returns all participants.
    pub fn participants(&self) -> &[CoherenceParticipant] {
        &self.participants
    }

    /// Returns a participant by identity.
    pub fn participant(
        &self,
        id: &CoherenceLocationId,
    ) -> Option<&CoherenceParticipant> {
        self.participants.iter().find(|participant| participant.id() == id)
    }

    /// Adds a participant.
    ///
    /// Duplicate location identities are rejected because two entries with
    /// the same identity would make coherence decisions ambiguous.
    pub fn add_participant(
        &mut self,
        participant: CoherenceParticipant,
    ) -> CoherenceResult<()> {
        if self.participants.iter().any(|existing| {
            existing.id() == participant.id()
        }) {
            return Err(MemoryError::invariant_violation(
                "duplicate coherence participant",
            ));
        }

        if self.participants.len() >= MAX_COHERENCE_LOCATIONS {
            return Err(MemoryError::memory_limit_exceeded(
                "coherence participant count",
            ));
        }

        self.participants.push(participant);
        self.recompute_consistency();

        Ok(())
    }

    /// Removes a participant after it has been detached.
    pub fn remove_participant(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<CoherenceParticipant> {
        let index = self
            .participants
            .iter()
            .position(|participant| participant.id() == id)
            .ok_or_else(|| {
                MemoryError::invalid_argument(
                    "coherence participant does not exist",
                )
            })?;

        let participant = self.participants.remove(index);

        self.recompute_consistency();

        Ok(participant)
    }

    /// Creates a coherence token for a participant.
    pub fn token(
        &self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<CoherenceToken> {
        let participant = self.participant(id).ok_or_else(|| {
            MemoryError::invalid_argument(
                "cannot create coherence token for unknown participant",
            )
        })?;

        Ok(CoherenceToken::new(
            self.state_id,
            self.memory_id,
            participant.generation(),
            self.epoch,
            id.clone(),
        ))
    }

    /// Validates that a token still represents this coherence domain.
    pub fn validate_token(
        &self,
        token: &CoherenceToken,
    ) -> CoherenceResult<()> {
        if token.state_id() != self.state_id
            || token.memory_id() != self.memory_id
        {
            return Err(MemoryError::concurrency_conflict(
                "coherence token belongs to another memory resource",
            ));
        }

        if token.epoch() != self.epoch {
            return Err(MemoryError::stale_generation(
                "coherence token belongs to an expired execution epoch",
            ));
        }

        let participant = self.participant(token.location()).ok_or_else(|| {
            MemoryError::concurrency_conflict(
                "coherence token participant is no longer attached",
            )
        })?;

        if participant.generation() != token.generation() {
            return Err(MemoryError::stale_generation(
                "coherence token generation is stale",
            ));
        }

        Ok(())
    }

    /// Validates an access request.
    pub fn validate_access(
        &self,
        id: &CoherenceLocationId,
        access: CoherenceAccess,
    ) -> CoherenceResult<()> {
        let participant = self.participant(id).ok_or_else(|| {
            MemoryError::invalid_argument(
                "coherence access requested for unknown participant",
            )
        })?;

        if !participant.capabilities().allows(access) {
            return Err(MemoryError::unsupported_operation(
                "coherence participant does not support requested access",
            ));
        }

        if participant.state().is_unavailable() {
            return Err(MemoryError::coherence_error(
                "coherence participant is unavailable",
            ));
        }

        if access.requires_local_read()
            && participant.state() == CoherenceCopyState::Stale
        {
            return Err(MemoryError::coherence_error(
                "read requires synchronization of stale memory",
            ));
        }

        Ok(())
    }

    /// Begins a write against a participant.
    ///
    /// This operation does not commit the write. It only marks the participant
    /// as dirty. The caller must later call `commit_write`.
    pub fn begin_write(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<()> {
        self.validate_access(id, CoherenceAccess::Write)?;

        let participant = self.participant_mut(id)?;

        participant.state = CoherenceCopyState::Dirty;
        self.consistency = StateConsistency::HostDirty;

        Ok(())
    }

    /// Commits a write and advances the global generation.
    ///
    /// The committed participant becomes authoritative unless the domain is
    /// explicitly provider-managed or opaque.
    pub fn commit_write(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<CoherenceGeneration> {
        let participant = self.participant_mut(id)?;

        if participant.state != CoherenceCopyState::Dirty {
            return Err(MemoryError::coherence_error(
                "only a dirty participant may commit a write",
            ));
        }

        let next_generation = self
            .generation
            .checked_next()
            .ok_or_else(|| {
                MemoryError::arithmetic_overflow(
                    "coherence generation overflow",
                )
            })?;

        self.generation = next_generation;

        participant.generation = next_generation;
        participant.state = CoherenceCopyState::Clean;

        for other in &mut self.participants {
            if other.id() != id
                && other.state() != CoherenceCopyState::Detached
            {
                other.state = if other.capabilities().provider_managed {
                    CoherenceCopyState::ProviderManaged
                } else {
                    CoherenceCopyState::Stale
                };
            }
        }

        self.authority = authority_for_location(
            self.participant(id)
                .map(|participant| participant.id().location())
                .ok_or_else(|| {
                    MemoryError::invariant_violation(
                        "committed coherence participant disappeared",
                    )
                })?,
        );

        self.recompute_consistency();

        Ok(next_generation)
    }

    /// Marks a participant as stale because another participant is newer.
    pub fn mark_stale(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<()> {
        let participant = self.participant_mut(id)?;

        if participant.state() == CoherenceCopyState::Detached {
            return Err(MemoryError::lifetime_violation(
                "detached coherence participant cannot be marked stale",
            ));
        }

        if participant.capabilities().provider_managed {
            participant.state = CoherenceCopyState::ProviderManaged;
        } else {
            participant.state = CoherenceCopyState::Stale;
        }

        self.recompute_consistency();

        Ok(())
    }

    /// Begins synchronization for a participant.
    pub fn begin_synchronization(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<()> {
        let participant = self.participant_mut(id)?;

        if !participant.capabilities().synchronization {
            return Err(MemoryError::unsupported_operation(
                "participant does not support synchronization",
            ));
        }

        if participant.state() == CoherenceCopyState::Detached {
            return Err(MemoryError::lifetime_violation(
                "detached participant cannot synchronize",
            ));
        }

        participant.state = CoherenceCopyState::Synchronizing;

        self.recompute_consistency();

        Ok(())
    }

    /// Completes synchronization against the current committed generation.
    pub fn complete_synchronization(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<CoherenceGeneration> {
        let participant = self.participant_mut(id)?;

        if participant.state() != CoherenceCopyState::Synchronizing {
            return Err(MemoryError::coherence_error(
                "participant is not synchronizing",
            ));
        }

        if !participant.capabilities().synchronization {
            return Err(MemoryError::unsupported_operation(
                "participant does not support synchronization",
            ));
        }

        participant.generation = self.generation;

        participant.state = if participant.capabilities().provider_managed {
            CoherenceCopyState::ProviderManaged
        } else {
            CoherenceCopyState::Clean
        };

        self.recompute_consistency();

        Ok(self.generation)
    }

    /// Invalidates a participant.
    pub fn invalidate(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<()> {
        let participant = self.participant_mut(id)?;

        participant.state = CoherenceCopyState::Invalid;

        self.recompute_consistency();

        Ok(())
    }

    /// Detaches a participant.
    pub fn detach(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<()> {
        let participant = self.participant_mut(id)?;

        if participant.state() == CoherenceCopyState::Dirty {
            return Err(MemoryError::coherence_error(
                "dirty participant cannot be detached before commit",
            ));
        }

        participant.state = CoherenceCopyState::Detached;

        self.recompute_consistency();

        Ok(())
    }

    /// Refreshes a participant to the current generation.
    pub fn refresh(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<CoherenceGeneration> {
        let participant = self.participant_mut(id)?;

        if !participant.capabilities().synchronization {
            return Err(MemoryError::unsupported_operation(
                "participant cannot be refreshed",
            ));
        }

        if participant.state() == CoherenceCopyState::Detached {
            return Err(MemoryError::lifetime_violation(
                "detached participant cannot be refreshed",
            ));
        }

        participant.generation = self.generation;
        participant.state = if participant.capabilities().provider_managed {
            CoherenceCopyState::ProviderManaged
        } else {
            CoherenceCopyState::Clean
        };

        self.recompute_consistency();

        Ok(self.generation)
    }

    /// Advances the execution epoch.
    ///
    /// Epoch advancement invalidates tokens from previous execution phases.
    pub fn advance_epoch(&mut self) -> CoherenceResult<CoherenceEpoch> {
        let next_epoch = self.epoch.checked_next().ok_or_else(|| {
            MemoryError::arithmetic_overflow(
                "coherence epoch overflow",
            )
        })?;

        self.epoch = next_epoch;

        for participant in &mut self.participants {
            if participant.state() != CoherenceCopyState::Detached {
                participant.state = if participant.capabilities().provider_managed
                {
                    CoherenceCopyState::ProviderManaged
                } else {
                    CoherenceCopyState::Stale
                };
            }
        }

        self.consistency = StateConsistency::Unknown;

        Ok(next_epoch)
    }

    /// Checks whether a participant is synchronized with the current
    /// generation.
    pub fn is_current(
        &self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<bool> {
        let participant = self.participant(id).ok_or_else(|| {
            MemoryError::invalid_argument(
                "unknown coherence participant",
            )
        })?;

        Ok(
            participant.generation() == self.generation
                && participant.state().is_readable(),
        )
    }

    /// Determines whether synchronization is needed for a requested read.
    pub fn needs_synchronization(
        &self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<bool> {
        let participant = self.participant(id).ok_or_else(|| {
            MemoryError::invalid_argument(
                "unknown coherence participant",
            )
        })?;

        Ok(
            participant.generation() != self.generation
                || participant.state().requires_refresh(),
        )
    }

    /// Validates a proposed synchronization.
    pub fn validate_synchronization(
        &self,
        source: &CoherenceLocationId,
        destination: &CoherenceLocationId,
        direction: SynchronizationDirection,
        reason: SynchronizationReason,
    ) -> CoherenceResult<()> {
        let source_participant = self.participant(source).ok_or_else(|| {
            MemoryError::invalid_argument(
                "unknown coherence synchronization source",
            )
        })?;

        let destination_participant =
            self.participant(destination).ok_or_else(|| {
                MemoryError::invalid_argument(
                    "unknown coherence synchronization destination",
                )
            })?;

        if source == destination {
            return Err(MemoryError::invalid_argument(
                "coherence synchronization source and destination must differ",
            ));
        }

        if !source_participant.capabilities().synchronization
            || !destination_participant
                .capabilities()
                .synchronization
        {
            return Err(MemoryError::unsupported_operation(
                "source and destination must support synchronization",
            ));
        }

        if source_participant.state() == CoherenceCopyState::Invalid
            || destination_participant.state() == CoherenceCopyState::Invalid
        {
            return Err(MemoryError::coherence_error(
                "invalid coherence participant cannot synchronize",
            ));
        }

        match direction {
            SynchronizationDirection::SourceToDestination => {
                if source_participant.state() == CoherenceCopyState::Stale {
                    return Err(MemoryError::coherence_error(
                        "stale source cannot become authoritative",
                    ));
                }
            }

            SynchronizationDirection::DestinationToSource => {
                if destination_participant.state() == CoherenceCopyState::Stale
                {
                    return Err(MemoryError::coherence_error(
                        "stale destination cannot become authoritative",
                    ));
                }
            }

            SynchronizationDirection::Bidirectional => {
                if source_participant.state() == CoherenceCopyState::Dirty
                    && destination_participant.state()
                        == CoherenceCopyState::Dirty
                {
                    if self.conflict_policy == ConflictPolicy::Reject {
                        return Err(MemoryError::concurrency_conflict(
                            "bidirectional synchronization has two dirty participants",
                        ));
                    }
                }
            }

            SynchronizationDirection::ProviderManaged => {
                if !source_participant.capabilities().provider_managed
                    && !destination_participant
                        .capabilities()
                        .provider_managed
                {
                    return Err(MemoryError::unsupported_operation(
                        "provider-managed synchronization requires a provider participant",
                    ));
                }
            }
        }

        match reason {
            SynchronizationReason::ReadPreparation
            | SynchronizationReason::ExecutionPreparation => {
                if destination_participant
                    .capabilities()
                    .provider_managed
                {
                    // A provider-managed destination does not imply that a
                    // local state copy exists. The synchronization layer must
                    // use the provider's native mechanism.
                }
            }

            SynchronizationReason::WriteCommit
            | SynchronizationReason::ExecutionCompletion
            | SynchronizationReason::Checkpoint
            | SynchronizationReason::Snapshot
            | SynchronizationReason::Migration
            | SynchronizationReason::Explicit
            | SynchronizationReason::Recovery
            | SynchronizationReason::ProviderLifecycle => {}
        }

        Ok(())
    }

    /// Returns the synchronization direction that would normally be used to
    /// refresh a destination from the authoritative state.
    pub fn refresh_direction(
        &self,
        destination: &CoherenceLocationId,
    ) -> CoherenceResult<SynchronizationDirection> {
        let participant = self.participant(destination).ok_or_else(|| {
            MemoryError::invalid_argument(
                "unknown coherence destination",
            )
        })?;

        match self.authority {
            CoherenceAuthority::Host
            | CoherenceAuthority::Device
            | CoherenceAuthority::Distributed => {
                let _ = participant;
                Ok(SynchronizationDirection::SourceToDestination)
            }

            CoherenceAuthority::External
            | CoherenceAuthority::Opaque => {
                Ok(SynchronizationDirection::ProviderManaged)
            }

            CoherenceAuthority::Shared => {
                Ok(SynchronizationDirection::Bidirectional)
            }
        }
    }

    /// Validates that a generation can be committed.
    pub fn validate_generation(
        &self,
        observed: CoherenceGeneration,
    ) -> CoherenceResult<()> {
        if observed != self.generation {
            return Err(MemoryError::stale_generation(
                "observed coherence generation does not match current generation",
            ));
        }

        Ok(())
    }

    /// Recomputes the representation-neutral consistency status used by
    /// `state.rs`.
    fn recompute_consistency(&mut self) {
        if self.participants.is_empty() {
            self.consistency = StateConsistency::Consistent;
            return;
        }

        if self
            .participants
            .iter()
            .any(|participant| participant.state() == CoherenceCopyState::Invalid)
        {
            self.consistency = StateConsistency::Unknown;
            return;
        }

        if self
            .participants
            .iter()
            .any(|participant| participant.state() == CoherenceCopyState::Synchronizing)
        {
            self.consistency = StateConsistency::Unknown;
            return;
        }

        if self
            .participants
            .iter()
            .any(|participant| participant.state() == CoherenceCopyState::Dirty)
        {
            self.consistency = StateConsistency::HostDirty;
            return;
        }

        let has_external = self
            .participants
            .iter()
            .any(|participant| participant.capabilities().provider_managed);

        if has_external {
            self.consistency = StateConsistency::ProviderManaged;
            return;
        }

        let has_distributed = self
            .participants
            .iter()
            .any(|participant| participant.id().location().is_distributed());

        if has_distributed {
            self.consistency = StateConsistency::Distributed;
            return;
        }

        let all_current = self.participants.iter().all(|participant| {
            participant.generation() == self.generation
                && matches!(
                    participant.state(),
                    CoherenceCopyState::Clean
                )
        });

        self.consistency = if all_current {
            if self.participants.len() > 1 {
                StateConsistency::Synchronized
            } else {
                StateConsistency::Consistent
            }
        } else {
            StateConsistency::Unknown
        };
    }

    fn participant_mut(
        &mut self,
        id: &CoherenceLocationId,
    ) -> CoherenceResult<&mut CoherenceParticipant> {
        self.participants
            .iter_mut()
            .find(|participant| participant.id() == id)
            .ok_or_else(|| {
                MemoryError::invalid_argument(
                    "unknown coherence participant",
                )
            })
    }
}

// =============================================================================
// Synchronization request
// =============================================================================

/// Immutable synchronization request.
///
/// `synchronization.rs` can consume this request and perform the actual data
/// movement, device synchronization, provider interaction, or distributed
/// communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationRequest {
    state_id: StateId,
    memory_id: MemoryId,
    source: CoherenceLocationId,
    destination: CoherenceLocationId,
    direction: SynchronizationDirection,
    reason: SynchronizationReason,
    observed_generation: CoherenceGeneration,
    epoch: CoherenceEpoch,
    timeout: Option<Duration>,
}

impl SynchronizationRequest {
    /// Creates a validated synchronization request.
    pub fn new(
        domain: &CoherenceDomain,
        source: CoherenceLocationId,
        destination: CoherenceLocationId,
        direction: SynchronizationDirection,
        reason: SynchronizationReason,
        timeout: Option<Duration>,
    ) -> CoherenceResult<Self> {
        domain.validate_synchronization(
            &source,
            &destination,
            direction,
            reason,
        )?;

        Ok(Self {
            state_id: domain.state_id(),
            memory_id: domain.memory_id(),
            source,
            destination,
            direction,
            reason,
            observed_generation: domain.generation(),
            epoch: domain.epoch(),
            timeout,
        })
    }

    /// Returns the state identity.
    pub fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns the memory identity.
    pub fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the source.
    pub fn source(&self) -> &CoherenceLocationId {
        &self.source
    }

    /// Returns the destination.
    pub fn destination(&self) -> &CoherenceLocationId {
        &self.destination
    }

    /// Returns the direction.
    pub const fn direction(&self) -> SynchronizationDirection {
        self.direction
    }

    /// Returns the reason.
    pub const fn reason(&self) -> SynchronizationReason {
        self.reason
    }

    /// Returns the generation observed when the request was created.
    pub const fn observed_generation(&self) -> CoherenceGeneration {
        self.observed_generation
    }

    /// Returns the epoch observed when the request was created.
    pub const fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns the optional timeout.
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Validates that the request is still current.
    pub fn validate(
        &self,
        domain: &CoherenceDomain,
    ) -> CoherenceResult<()> {
        if self.state_id != domain.state_id()
            || self.memory_id != domain.memory_id()
        {
            return Err(MemoryError::concurrency_conflict(
                "synchronization request targets another memory resource",
            ));
        }

        if self.epoch != domain.epoch() {
            return Err(MemoryError::stale_generation(
                "synchronization request belongs to an expired epoch",
            ));
        }

        domain.validate_generation(self.observed_generation)?;

        domain.validate_synchronization(
            &self.source,
            &self.destination,
            self.direction,
            self.reason,
        )
    }
}

// =============================================================================
// Synchronization completion
// =============================================================================

/// Result metadata produced after a synchronization implementation completes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationCompletion {
    state_id: StateId,
    memory_id: MemoryId,
    source: CoherenceLocationId,
    destination: CoherenceLocationId,
    generation: CoherenceGeneration,
    epoch: CoherenceEpoch,
    bytes_transferred: ByteCount,
}

impl SynchronizationCompletion {
    /// Creates completion metadata.
    pub fn new(
        request: &SynchronizationRequest,
        generation: CoherenceGeneration,
        bytes_transferred: ByteCount,
    ) -> Self {
        Self {
            state_id: request.state_id,
            memory_id: request.memory_id,
            source: request.source.clone(),
            destination: request.destination.clone(),
            generation,
            epoch: request.epoch,
            bytes_transferred,
        }
    }

    /// Returns the committed generation.
    pub const fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns the execution epoch.
    pub const fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns transferred bytes.
    pub const fn bytes_transferred(&self) -> ByteCount {
        self.bytes_transferred
    }

    /// Returns the source.
    pub fn source(&self) -> &CoherenceLocationId {
        &self.source
    }

    /// Returns the destination.
    pub fn destination(&self) -> &CoherenceLocationId {
        &self.destination
    }

    /// Returns state identity.
    pub const fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns memory identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.memory_id
    }
}

// =============================================================================
// Provider coherence contract
// =============================================================================

/// Provider-neutral capability contract for externally managed quantum
/// resources.
///
/// Hardware adapters can implement this trait without exposing their SDK
/// types to `quantum::memory`.
pub trait CoherenceProvider {
    /// Returns capabilities of the provider resource.
    fn capabilities(&self) -> CoherenceCapabilities;

    /// Returns the current generation known by the provider.
    ///
    /// A provider may return a monotonically increasing execution/version
    /// number without exposing quantum amplitudes.
    fn generation(&self) -> CoherenceResult<CoherenceGeneration>;

    /// Validates a provider-owned coherence token.
    fn validate_token(
        &self,
        token: &CoherenceToken,
    ) -> CoherenceResult<()>;

    /// Requests provider-managed synchronization.
    ///
    /// Implementations own all network/API interaction.
    fn synchronize(
        &mut self,
        request: &SynchronizationRequest,
    ) -> CoherenceResult<SynchronizationCompletion>;
}

// =============================================================================
// Helper functions
// =============================================================================

fn validate_name(
    value: &str,
    maximum: usize,
    description: &'static str,
) -> CoherenceResult<()> {
    if value.is_empty() {
        return Err(MemoryError::invalid_argument(description));
    }

    if value.len() > maximum {
        return Err(MemoryError::memory_limit_exceeded(description));
    }

    if value.chars().any(char::is_control) {
        return Err(MemoryError::invalid_argument(description));
    }

    Ok(())
}

fn authority_for_location(
    location: &CoherenceLocation,
) -> CoherenceAuthority {
    match location {
        CoherenceLocation::Host | CoherenceLocation::PinnedHost => {
            CoherenceAuthority::Host
        }

        CoherenceLocation::Device | CoherenceLocation::Unified => {
            CoherenceAuthority::Device
        }

        CoherenceLocation::DistributedNode(_) => {
            CoherenceAuthority::Distributed
        }

        CoherenceLocation::RemoteProvider
        | CoherenceLocation::External => CoherenceAuthority::External,

        CoherenceLocation::Opaque | CoherenceLocation::Custom(_) => {
            CoherenceAuthority::Opaque
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn memory_id() -> MemoryId {
        MemoryId::new(1)
    }

    fn state_id() -> StateId {
        StateId::new(1)
    }

    fn host() -> CoherenceLocationId {
        CoherenceLocationId::new(
            "test",
            CoherenceLocation::Host,
        )
        .expect("valid host location")
    }

    fn device() -> CoherenceLocationId {
        CoherenceLocationId::new(
            "test",
            CoherenceLocation::Device,
        )
        .expect("valid device location")
    }

    #[test]
    fn generation_advances_without_overflow() {
        let generation = CoherenceGeneration::INITIAL;

        assert_eq!(
            generation.checked_next(),
            Some(CoherenceGeneration::new(2))
        );
    }

    #[test]
    fn host_participant_can_be_added() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        let participant = CoherenceParticipant::new(
            host(),
            CoherenceCopyState::Clean,
            CoherenceGeneration::INITIAL,
            CoherenceCapabilities::host(),
            ByteCount::new(1024),
        );

        assert!(domain.add_participant(participant).is_ok());
        assert_eq!(domain.participants().len(), 1);
    }

    #[test]
    fn duplicate_participant_is_rejected() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        let first = CoherenceParticipant::new(
            host(),
            CoherenceCopyState::Clean,
            CoherenceGeneration::INITIAL,
            CoherenceCapabilities::host(),
            ByteCount::new(1024),
        );

        let second = CoherenceParticipant::new(
            host(),
            CoherenceCopyState::Clean,
            CoherenceGeneration::INITIAL,
            CoherenceCapabilities::host(),
            ByteCount::new(1024),
        );

        assert!(domain.add_participant(first).is_ok());
        assert!(domain.add_participant(second).is_err());
    }

    #[test]
    fn write_commit_advances_generation_and_stales_other_copy() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        let host_participant = CoherenceParticipant::new(
            host(),
            CoherenceCopyState::Clean,
            CoherenceGeneration::INITIAL,
            CoherenceCapabilities::host(),
            ByteCount::new(1024),
        );

        let device_participant = CoherenceParticipant::new(
            device(),
            CoherenceCopyState::Clean,
            CoherenceGeneration::INITIAL,
            CoherenceCapabilities::device(),
            ByteCount::new(1024),
        );

        domain
            .add_participant(host_participant)
            .expect("host participant");

        domain
            .add_participant(device_participant)
            .expect("device participant");

        domain
            .begin_write(&host())
            .expect("begin host write");

        let generation = domain
            .commit_write(&host())
            .expect("commit host write");

        assert_eq!(generation, CoherenceGeneration::new(2));

        assert_eq!(
            domain
                .participant(&host())
                .expect("host")
                .state(),
            CoherenceCopyState::Clean
        );

        assert_eq!(
            domain
                .participant(&device())
                .expect("device")
                .state(),
            CoherenceCopyState::Stale
        );
    }

    #[test]
    fn stale_read_requires_synchronization() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        domain
            .add_participant(CoherenceParticipant::new(
                host(),
                CoherenceCopyState::Clean,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::host(),
                ByteCount::new(1024),
            ))
            .expect("host");

        domain
            .add_participant(CoherenceParticipant::new(
                device(),
                CoherenceCopyState::Stale,
                CoherenceGeneration::ZERO,
                CoherenceCapabilities::device(),
                ByteCount::new(1024),
            ))
            .expect("device");

        assert!(domain
            .validate_access(&device(), CoherenceAccess::Read)
            .is_err());

        assert!(domain.needs_synchronization(&device()).expect("check"));
    }

    #[test]
    fn provider_qpu_does_not_require_local_quantum_readability() {
        let capabilities = CoherenceCapabilities::provider_qpu();

        assert!(!capabilities.readable);
        assert!(!capabilities.writable);
        assert!(capabilities.provider_managed);
        assert!(capabilities.synchronization);

        assert!(capabilities.allows(CoherenceAccess::Observe));
        assert!(capabilities.allows(CoherenceAccess::Execute));
        assert!(!capabilities.allows(CoherenceAccess::Read));
    }

    #[test]
    fn token_becomes_stale_after_epoch_change() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        domain
            .add_participant(CoherenceParticipant::new(
                host(),
                CoherenceCopyState::Clean,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::host(),
                ByteCount::new(1024),
            ))
            .expect("host");

        let token = domain.token(&host()).expect("token");

        domain.advance_epoch().expect("advance epoch");

        assert!(domain.validate_token(&token).is_err());
    }

    #[test]
    fn both_dirty_participants_are_rejected_under_default_policy() {
        let mut domain = CoherenceDomain::new(
            state_id(),
            memory_id(),
            CoherenceAuthority::Shared,
            ConflictPolicy::Reject,
        );

        domain
            .add_participant(CoherenceParticipant::new(
                host(),
                CoherenceCopyState::Dirty,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::host(),
                ByteCount::new(1024),
            ))
            .expect("host");

        domain
            .add_participant(CoherenceParticipant::new(
                device(),
                CoherenceCopyState::Dirty,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::device(),
                ByteCount::new(1024),
            ))
            .expect("device");

        assert!(domain
            .validate_synchronization(
                &host(),
                &device(),
                SynchronizationDirection::Bidirectional,
                SynchronizationReason::Explicit,
            )
            .is_err());
    }
}