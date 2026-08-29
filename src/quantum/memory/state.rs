//! Zamani Quantum Memory — Canonical Quantum State Contract
//!
//! `state.rs` is the representation-neutral state boundary for
//! `quantum::memory`.
//!
//! # Architectural responsibility
//!
//! This module defines:
//!
//! - the canonical `QuantumState` trait;
//! - state metadata;
//! - state capabilities;
//! - state lifecycle and coherence status;
//! - state operation descriptors;
//! - provider-neutral operation semantics;
//! - state execution domains;
//! - state observations that are representation-independent;
//! - validation helpers for state operations;
//! - deterministic state requirements;
//! - state-provider contracts;
//! - capability negotiation;
//! - safe state forking;
//! - state-level integration boundaries.
//!
//! This module does NOT implement:
//!
//! - state-vector mathematics;
//! - density matrices;
//! - stabilizer/tableau mathematics;
//! - sparse-state algorithms;
//! - tensor-network algorithms;
//! - allocation;
//! - GPU memory;
//! - distributed communication;
//! - measurement sampling;
//! - routing;
//! - scheduling;
//! - compiler parsing;
//! - gate optimization;
//! - QEC decoding;
//! - vendor-specific QPU APIs.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                    hardware-independent
//!                       quantum semantics
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!      optimization         routing            scheduling
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                       execution layer
//!                              │
//!                              ▼
//!                  quantum::memory::state
//!                              │
//!        ┌─────────────┬───────┼────────┬──────────────┐
//!        ▼             ▼       ▼        ▼              ▼
//!   StateVector   Density   Stabilizer Sparse     TensorNetwork
//!        │             │       │        │              │
//!        └─────────────┴───────┼────────┴──────────────┘
//!                              ▼
//!                  provider/backend boundary
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!           CPU              GPU              QPU
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                local/emulator       remote/backend
//! ```
//!
//! # Critical design rule
//!
//! A `QuantumState` is NOT synonymous with a simulator state.
//!
//! A real QPU may expose:
//!
//! - no amplitudes;
//! - no probability vector;
//! - no direct state cloning;
//! - no arbitrary unitary execution;
//! - no direct normalization;
//! - no state-vector serialization.
//!
//! Therefore optional functionality is expressed through capabilities.
//! Implementations MUST NOT fabricate simulator information merely to satisfy
//! this trait.
//!
//! # Canonical identity rule
//!
//! This module uses the canonical identities owned by `quantum::ir`:
//!
//! - `quantum::ir::QubitId`;
//! - `quantum::ir::PhysicalQubitId`;
//! - `quantum::ir::ClassicalBitId`;
//! - `quantum::ir::OperationId`.
//!
//! `memory::state` MUST NOT define replacement quantum identity types.
//!
//! # Error boundary
//!
//! All fallible operations use:
//!
//! ```text
//! Result<T, MemoryError>
//! ```
//!
//! from `quantum::memory::errors`.
//!
//! The state layer does not invent a second state-specific error hierarchy.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `unsafe` is explicitly denied.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly-only features are required.
//!
//! # Determinism
//!
//! This module never creates a hidden global RNG.
//!
//! Measurement randomness, if required, belongs to the measurement/execution
//! implementation and must be explicitly injected or controlled by the
//! execution layer.
//!
//! # Vendor neutrality
//!
//! Nothing in this module refers to IBM, Quantinuum, IonQ, Rigetti, IQM,
//! Pasqal, D-Wave, Google, AWS, NVIDIA, CUDA, ROCm, Metal, QIR, OpenQASM,
//! Quil, Cirq, Qiskit, Braket, or another vendor-specific API.
//!
//! Hardware adapters translate their native representations into this
//! provider-neutral contract.
//!
//! # Integration contract
//!
//! Earlier foundational memory modules:
//!
//! ```text
//! errors.rs
//! types.rs
//! numeric.rs
//! complex.rs
//! representation.rs
//! limits.rs
//! layout.rs
//! indexing.rs
//! ```
//!
//! Later modules consume this contract:
//!
//! ```text
//! state_vector.rs
//! density_matrix.rs
//! stabilizer.rs
//! sparse.rs
//! tensor_network.rs
//! backend_state.rs
//! view.rs
//! permutation.rs
//! measurement.rs
//! reset.rs
//! snapshot.rs
//! checkpoint.rs
//! migration.rs
//! diagnostics.rs
//! telemetry.rs
//! ```
//!
//! No later module should require changing this file merely because a new
//! state representation or hardware provider is added.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::time::Duration;

use super::errors::MemoryError;
use super::types::{
    AmplitudeCount,
    ByteCount,
    ClassicalBitCount,
    MemoryId,
    QubitCount,
    StateId,
};

use crate::quantum::ir::{
    ClassicalBitId,
    OperationId,
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Result aliases
// =============================================================================

/// Canonical result type for state operations.
pub type StateResult<T> = Result<T, MemoryError>;

/// Borrowed state object suitable for provider-neutral consumers.
pub type StateRef<'a> = &'a dyn QuantumState;

/// Mutable borrowed state object suitable for execution.
pub type StateMut<'a> = &'a mut dyn QuantumState;

// =============================================================================
// State representation identity
// =============================================================================

/// Provider-neutral name for the representation implemented by a state.
///
/// This is deliberately a string rather than an enum owned by this file.
///
/// `representation.rs` owns the canonical representation taxonomy. This
/// lightweight value lets `state.rs` remain independently extensible and
/// prevents the state trait from having to be modified whenever a new
/// representation is introduced.
///
/// Examples:
///
/// - `"state_vector"`
/// - `"density_matrix"`
/// - `"stabilizer"`
/// - `"sparse"`
/// - `"mps"`
/// - `"backend_native"`
/// - `"photonic"`
/// - `"continuous_variable"`
/// - `"annealing"`
/// - `"custom"`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StateRepresentationName(String);

impl StateRepresentationName {
    /// Maximum permitted representation-name length.
    pub const MAX_LENGTH: usize = 256;

    /// Creates a representation name.
    ///
    /// Names are intentionally conservative:
    ///
    /// - non-empty;
    /// - no leading/trailing whitespace;
    /// - no control characters;
    /// - bounded length.
    pub fn new(value: impl Into<String>) -> StateResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.len() > Self::MAX_LENGTH
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(Self::invalid_name_error());
        }

        Ok(Self(value))
    }

    /// Returns the representation name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn invalid_name_error() -> MemoryError {
        // The canonical error taxonomy owns the actual error construction.
        //
        // This function is intentionally unreachable in normal builds until
        // the foundational errors contract exposes its structured constructor.
        //
        // Implementations should construct the appropriate
        // MemoryError::InvalidArgument/InvalidIdentifier variant through the
        // canonical errors API.
        //
        // The state module itself therefore does not duplicate that taxonomy.
        MemoryError::invalid_argument(
            "state representation name must be non-empty, bounded, \
             trimmed, and free of control characters",
        )
    }
}

impl fmt::Display for StateRepresentationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Storage location
// =============================================================================

/// Provider-neutral storage location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateStorageLocation {
    /// Ordinary host memory.
    Host,

    /// Page-locked/pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared host-device memory.
    Unified,

    /// Memory distributed across multiple execution nodes.
    Distributed,

    /// State is owned by a remote execution service/QPU.
    Remote,

    /// Storage location is controlled by an external provider.
    External,

    /// Representation has no directly addressable storage exposed here.
    Opaque,
}

impl StateStorageLocation {
    /// Returns whether this location is directly host-readable.
    pub const fn is_host_readable(self) -> bool {
        matches!(self, Self::Host | Self::PinnedHost | Self::Unified)
    }

    /// Returns whether this location represents device memory.
    pub const fn is_device(self) -> bool {
        matches!(self, Self::Device | Self::Unified)
    }

    /// Returns whether this location can span execution nodes.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether the state is externally owned.
    pub const fn is_external(self) -> bool {
        matches!(self, Self::Remote | Self::External | Self::Opaque)
    }
}

impl fmt::Display for StateStorageLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::Remote => "remote",
            Self::External => "external",
            Self::Opaque => "opaque",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Execution domain
// =============================================================================

/// Execution domain in which the state exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateExecutionDomain {
    /// Local classical simulation.
    LocalSimulator,

    /// Local hardware emulator.
    LocalEmulator,

    /// Remote simulator.
    RemoteSimulator,

    /// Physical quantum processor.
    Qpu,

    /// Quantum emulator backed by a physical-device model.
    HardwareEmulator,

    /// Hybrid classical/quantum execution.
    Hybrid,

    /// Multi-node distributed simulation or execution.
    Distributed,

    /// Provider-defined execution domain.
    Custom,
}

impl StateExecutionDomain {
    /// Returns whether this domain represents actual quantum hardware.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether this domain is simulator-like.
    pub const fn is_simulator(self) -> bool {
        matches!(
            self,
            Self::LocalSimulator
                | Self::LocalEmulator
                | Self::RemoteSimulator
                | Self::HardwareEmulator
        )
    }

    /// Returns whether this domain can involve both classical and quantum
    /// resources.
    pub const fn is_hybrid(self) -> bool {
        matches!(self, Self::Hybrid)
    }

    /// Returns whether the domain can span multiple execution nodes.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }
}

impl fmt::Display for StateExecutionDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::LocalSimulator => "local_simulator",
            Self::LocalEmulator => "local_emulator",
            Self::RemoteSimulator => "remote_simulator",
            Self::Qpu => "qpu",
            Self::HardwareEmulator => "hardware_emulator",
            Self::Hybrid => "hybrid",
            Self::Distributed => "distributed",
            Self::Custom => "custom",
        };

        f.write_str(name)
    }
}

// =============================================================================
// State lifecycle
// =============================================================================

/// Lifecycle of a state resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateLifecycle {
    /// Resource has been allocated but not yet initialized.
    Allocated,

    /// State is initialized and available.
    Ready,

    /// State is currently executing an operation.
    Executing,

    /// State is temporarily unavailable but can be resumed.
    Suspended,

    /// State has been released and must not be accessed.
    Released,

    /// State encountered a terminal failure.
    Failed,
}

impl StateLifecycle {
    /// Returns whether operations may normally be submitted.
    pub const fn accepts_operations(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether the state is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Released | Self::Failed)
    }

    /// Returns whether the state can potentially be resumed.
    pub const fn is_resumable(self) -> bool {
        matches!(self, Self::Suspended)
    }
}

impl fmt::Display for StateLifecycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Allocated => "allocated",
            Self::Ready => "ready",
            Self::Executing => "executing",
            Self::Suspended => "suspended",
            Self::Released => "released",
            Self::Failed => "failed",
        };

        f.write_str(name)
    }
}

// =============================================================================
// State consistency
// =============================================================================

/// Coherence/consistency status of a quantum state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateConsistency {
    /// State is internally consistent.
    Consistent,

    /// Host copy contains modifications not yet synchronized elsewhere.
    HostDirty,

    /// Device copy contains modifications not yet synchronized elsewhere.
    DeviceDirty,

    /// Multiple locations contain synchronized copies.
    Synchronized,

    /// State is distributed and requires collective consistency management.
    Distributed,

    /// Provider controls consistency and does not expose its internal state.
    ProviderManaged,

    /// Consistency cannot currently be established.
    Unknown,
}

impl StateConsistency {
    /// Returns whether a caller may treat the state as internally consistent.
    pub const fn is_consistent(self) -> bool {
        matches!(
            self,
            Self::Consistent
                | Self::Synchronized
                | Self::Distributed
                | Self::ProviderManaged
        )
    }

    /// Returns whether synchronization may be required.
    pub const fn requires_synchronization(self) -> bool {
        matches!(self, Self::HostDirty | Self::DeviceDirty)
    }
}

// =============================================================================
// Capability bit set
// =============================================================================

/// Capabilities exposed by a quantum-state implementation.
///
/// This is deliberately a bit set instead of an enum because a state may
/// support several independent capabilities simultaneously.
///
/// Examples:
///
/// - a state vector supports amplitude access and arbitrary unitary
///   simulation;
/// - a QPU state may support gate execution and measurement but not amplitude
///   access;
/// - a stabilizer state supports Clifford execution but may reject arbitrary
///   unitaries;
/// - a remote backend may support snapshots without exposing amplitudes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StateCapabilities(u64);

impl StateCapabilities {
    /// No capabilities.
    pub const NONE: Self = Self(0);

    /// Can execute unitary operations.
    pub const UNITARY: Self = Self(1 << 0);

    /// Can execute general quantum channels/noise operations.
    pub const CHANNEL: Self = Self(1 << 1);

    /// Can execute measurement operations.
    pub const MEASUREMENT: Self = Self(1 << 2);

    /// Can execute reset operations.
    pub const RESET: Self = Self(1 << 3);

    /// Can expose individual amplitudes.
    pub const AMPLITUDE_ACCESS: Self = Self(1 << 4);

    /// Can expose computational-basis probabilities.
    pub const PROBABILITY_ACCESS: Self = Self(1 << 5);

    /// Can expose expectation values.
    pub const EXPECTATION_VALUE: Self = Self(1 << 6);

    /// Can form tensor products without leaving the representation.
    pub const TENSOR_PRODUCT: Self = Self(1 << 7);

    /// Can compute partial traces/reduced states.
    pub const PARTIAL_TRACE: Self = Self(1 << 8);

    /// Can clone/fork the complete quantum state.
    pub const FORK: Self = Self(1 << 9);

    /// Can serialize the current state.
    pub const SERIALIZE: Self = Self(1 << 10);

    /// Can restore state from a compatible snapshot.
    pub const RESTORE: Self = Self(1 << 11);

    /// Can synchronize between memory locations.
    pub const SYNCHRONIZE: Self = Self(1 << 12);

    /// Can migrate to another representation/location.
    pub const MIGRATE: Self = Self(1 << 13);

    /// Can execute operations concurrently where provider permits.
    pub const CONCURRENT_EXECUTION: Self = Self(1 << 14);

    /// Can execute controlled operations.
    pub const CONTROLLED_OPERATION: Self = Self(1 << 15);

    /// Can execute parameterized operations.
    pub const PARAMETERIZED_OPERATION: Self = Self(1 << 16);

    /// Can execute dynamic/mid-circuit operations.
    pub const DYNAMIC_CIRCUIT: Self = Self(1 << 17);

    /// Can retain state across multiple operations/jobs.
    pub const PERSISTENT: Self = Self(1 << 18);

    /// State is externally owned by a backend.
    pub const BACKEND_NATIVE: Self = Self(1 << 19);

    /// State supports distributed execution.
    pub const DISTRIBUTED: Self = Self(1 << 20);

    /// State supports deterministic replay when given the same execution
    /// inputs and RNG state.
    pub const DETERMINISTIC_REPLAY: Self = Self(1 << 21);

    /// State supports exact state-vector-like amplitude semantics.
    pub const PURE_STATE: Self = Self(1 << 22);

    /// State supports mixed-state semantics.
    pub const MIXED_STATE: Self = Self(1 << 23);

    /// State supports stabilizer/Clifford semantics.
    pub const STABILIZER: Self = Self(1 << 24);

    /// State supports sparse-state semantics.
    pub const SPARSE: Self = Self(1 << 25);

    /// State supports tensor-network semantics.
    pub const TENSOR_NETWORK: Self = Self(1 << 26);

    /// State supports custom/provider-defined operations.
    pub const CUSTOM_OPERATIONS: Self = Self(1 << 27);

    /// Returns the raw bit representation.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Creates a capability set from raw bits.
    ///
    /// Unknown bits are preserved so future capabilities can be represented
    /// without changing the storage layout.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns whether all requested capabilities are present.
    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Returns whether any requested capability is present.
    pub const fn intersects(self, requested: Self) -> bool {
        self.0 & requested.0 != 0
    }

    /// Adds capabilities.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Removes capabilities.
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Adds one capability.
    pub const fn with(self, capability: Self) -> Self {
        self.union(capability)
    }

    /// Removes one capability.
    pub const fn without(self, capability: Self) -> Self {
        self.difference(capability)
    }

    /// Returns whether no capability is present.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for StateCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for StateCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for StateCapabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

// =============================================================================
// Operation kinds
// =============================================================================

/// Provider-neutral state operation category.
///
/// This category intentionally does not encode individual gate names.
///
/// Gate semantics remain owned by `quantum::ir`, while backend-specific
/// instructions remain owned by hardware adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateOperationKind {
    /// Initialize a state/register.
    Initialize,

    /// Apply a unitary transformation.
    Unitary,

    /// Apply a general quantum channel/noise operation.
    Channel,

    /// Measure one or more qubits.
    Measure,

    /// Reset one or more qubits.
    Reset,

    /// Query a computational-basis probability.
    Probability,

    /// Query an expectation value.
    ExpectationValue,

    /// Construct a tensor product.
    TensorProduct,

    /// Reduce/trace out part of a state.
    PartialTrace,

    /// Synchronize host/device/distributed copies.
    Synchronize,

    /// Create a checkpoint/snapshot boundary.
    Snapshot,

    /// Restore a previously captured state.
    Restore,

    /// Migrate state representation or storage location.
    Migrate,

    /// Backend/provider-defined operation.
    Custom,
}

impl StateOperationKind {
    /// Returns the capabilities normally required by this operation.
    pub const fn required_capabilities(self) -> StateCapabilities {
        match self {
            Self::Initialize => StateCapabilities::NONE,
            Self::Unitary => StateCapabilities::UNITARY,
            Self::Channel => StateCapabilities::CHANNEL,
            Self::Measure => StateCapabilities::MEASUREMENT,
            Self::Reset => StateCapabilities::RESET,
            Self::Probability => StateCapabilities::PROBABILITY_ACCESS,
            Self::ExpectationValue => StateCapabilities::EXPECTATION_VALUE,
            Self::TensorProduct => StateCapabilities::TENSOR_PRODUCT,
            Self::PartialTrace => StateCapabilities::PARTIAL_TRACE,
            Self::Synchronize => StateCapabilities::SYNCHRONIZE,
            Self::Snapshot => StateCapabilities::SERIALIZE,
            Self::Restore => StateCapabilities::RESTORE,
            Self::Migrate => StateCapabilities::MIGRATE,
            Self::Custom => StateCapabilities::CUSTOM_OPERATIONS,
        }
    }

    /// Returns whether this operation can change quantum state.
    pub const fn is_mutating(self) -> bool {
        matches!(
            self,
            Self::Initialize
                | Self::Unitary
                | Self::Channel
                | Self::Measure
                | Self::Reset
                | Self::TensorProduct
                | Self::Restore
                | Self::Migrate
                | Self::Custom
        )
    }

    /// Returns whether the operation can require classical output.
    pub const fn may_produce_classical_output(self) -> bool {
        matches!(self, Self::Measure | Self::Probability | Self::Custom)
    }
}

// =============================================================================
// Operation semantics
// =============================================================================

/// Semantic flags associated with a state operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StateOperationSemantics(u32);

impl StateOperationSemantics {
    /// Operation has no special semantic flags.
    pub const NONE: Self = Self(0);

    /// Operation is mathematically unitary.
    pub const UNITARY: Self = Self(1 << 0);

    /// Operation is reversible.
    pub const REVERSIBLE: Self = Self(1 << 1);

    /// Operation may be probabilistic.
    pub const PROBABILISTIC: Self = Self(1 << 2);

    /// Operation may collapse quantum state.
    pub const COLLAPSING: Self = Self(1 << 3);

    /// Operation requires classical feedback.
    pub const CLASSICAL_FEEDBACK: Self = Self(1 << 4);

    /// Operation may introduce noise.
    pub const NOISY: Self = Self(1 << 5);

    /// Operation can be executed without modifying quantum state.
    pub const READ_ONLY: Self = Self(1 << 6);

    /// Operation is backend/provider-specific.
    pub const PROVIDER_DEFINED: Self = Self(1 << 7);

    /// Returns raw flags.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Creates flags from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Tests whether all flags are present.
    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Adds flags.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOr for StateOperationSemantics {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

// =============================================================================
// State operation descriptor
// =============================================================================

/// Provider-neutral operation descriptor consumed by a `QuantumState`.
///
/// Implementations are normally adapters over canonical `quantum::ir`
/// operations.
///
/// The state layer deliberately receives an abstraction instead of directly
/// depending on one particular gate enum. This keeps memory usable by:
///
/// - simulators;
/// - hardware emulators;
/// - QPUs;
/// - pulse-level adapters;
/// - custom quantum execution systems.
///
/// An adapter can therefore translate the canonical IR into this contract
/// without making the memory layer depend on the adapter.
pub trait StateOperation: Send + Sync {
    /// Operation identity when one exists.
    ///
    /// `None` is permitted for ephemeral operations created directly by a
    /// simulator or provider.
    fn operation_id(&self) -> Option<OperationId> {
        None
    }

    /// Operation category.
    fn kind(&self) -> StateOperationKind;

    /// Stable provider-neutral operation name.
    ///
    /// Examples include `"h"`, `"cx"`, `"measure"`, `"reset"`, or a
    /// canonical custom operation name.
    fn name(&self) -> &str;

    /// Logical qubits consumed by the operation.
    fn logical_qubits(&self) -> &[QubitId];

    /// Physical qubits, if routing/hardware lowering has already selected them.
    ///
    /// A simulator may return an empty slice.
    fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &[]
    }

    /// Classical destinations consumed or produced by the operation.
    fn classical_bits(&self) -> &[ClassicalBitId] {
        &[]
    }

    /// Real-valued operation parameters.
    ///
    /// Quantum IR parameter evaluation must occur before the execution layer
    /// consumes an operation requiring concrete numeric values.
    fn parameters(&self) -> &[f64] {
        &[]
    }

    /// Operation semantic flags.
    fn semantics(&self) -> StateOperationSemantics {
        StateOperationSemantics::NONE
    }

    /// Returns whether this operation has a valid provider-neutral descriptor.
    fn validate_descriptor(&self) -> StateResult<()> {
        validate_operation_descriptor(self)
    }
}

// =============================================================================
// State metadata
// =============================================================================

/// Immutable metadata describing a quantum-state resource.
///
/// This is deliberately descriptive rather than owning the actual state
/// storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateMetadata {
    /// Unique memory-state identity.
    pub state_id: StateId,

    /// Optional memory resource owning the state.
    pub memory_id: Option<MemoryId>,

    /// Number of logical qubits represented.
    pub qubit_count: QubitCount,

    /// Number of classical bits associated with the state/execution context.
    pub classical_bit_count: ClassicalBitCount,

    /// Number of amplitudes when the representation has a meaningful
    /// amplitude-count concept.
    pub amplitude_count: Option<AmplitudeCount>,

    /// Current estimated/declared state memory consumption.
    pub memory_bytes: ByteCount,

    /// Representation identifier.
    pub representation: StateRepresentationName,

    /// Storage location.
    pub storage_location: StateStorageLocation,

    /// Execution domain.
    pub execution_domain: StateExecutionDomain,

    /// Provider/backend name when known.
    pub provider_name: Option<String>,

    /// Physical-device identifier/name when known.
    ///
    /// This is intentionally descriptive. Hardware topology remains owned by
    /// `quantum::hardware`.
    pub device_name: Option<String>,
}

impl StateMetadata {
    /// Creates metadata after validating bounded provider/device names.
    pub fn new(
        state_id: StateId,
        memory_id: Option<MemoryId>,
        qubit_count: QubitCount,
        classical_bit_count: ClassicalBitCount,
        amplitude_count: Option<AmplitudeCount>,
        memory_bytes: ByteCount,
        representation: StateRepresentationName,
        storage_location: StateStorageLocation,
        execution_domain: StateExecutionDomain,
        provider_name: Option<String>,
        device_name: Option<String>,
    ) -> StateResult<Self> {
        validate_optional_name(provider_name.as_deref(), "provider name")?;
        validate_optional_name(device_name.as_deref(), "device name")?;

        Ok(Self {
            state_id,
            memory_id,
            qubit_count,
            classical_bit_count,
            amplitude_count,
            memory_bytes,
            representation,
            storage_location,
            execution_domain,
            provider_name,
            device_name,
        })
    }

    /// Returns whether this state represents an actual QPU execution resource.
    pub const fn is_qpu(&self) -> bool {
        self.execution_domain.is_qpu()
    }

    /// Returns whether the representation is externally owned.
    pub const fn is_external(&self) -> bool {
        self.storage_location.is_external()
    }
}

// =============================================================================
// State requirements
// =============================================================================

/// Requirements a caller can use when selecting a state provider.
///
/// This is consumed by the future representation-selection/runtime layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateRequirements {
    /// Minimum number of logical qubits required.
    pub qubits: QubitCount,

    /// Required classical bits.
    pub classical_bits: ClassicalBitCount,

    /// Minimum capabilities.
    pub capabilities: StateCapabilities,

    /// Preferred execution domain, when any.
    pub execution_domain: Option<StateExecutionDomain>,

    /// Preferred storage location, when any.
    pub storage_location: Option<StateStorageLocation>,

    /// Optional required representation name.
    pub representation: Option<StateRepresentationName>,

    /// Maximum permitted memory consumption.
    pub maximum_memory: Option<ByteCount>,

    /// Whether a provider must support deterministic replay.
    pub deterministic_replay: bool,

    /// Whether provider-native/custom operations are acceptable.
    pub allow_custom_operations: bool,
}

impl Default for StateRequirements {
    fn default() -> Self {
        Self {
            qubits: QubitCount::ZERO,
            classical_bits: ClassicalBitCount::ZERO,
            capabilities: StateCapabilities::NONE,
            execution_domain: None,
            storage_location: None,
            representation: None,
            maximum_memory: None,
            deterministic_replay: false,
            allow_custom_operations: false,
        }
    }
}

impl StateRequirements {
    /// Creates requirements for a qubit count.
    pub fn for_qubits(qubits: QubitCount) -> Self {
        Self {
            qubits,
            ..Self::default()
        }
    }

    /// Adds required capabilities.
    pub const fn with_capabilities(mut self, capabilities: StateCapabilities) -> Self {
        self.capabilities = self.capabilities.union(capabilities);
        self
    }

    /// Requires deterministic replay.
    pub const fn require_deterministic_replay(mut self) -> Self {
        self.deterministic_replay = true;
        self
    }

    /// Allows provider-specific custom operations.
    pub const fn allow_custom_operations(mut self) -> Self {
        self.allow_custom_operations = true;
        self
    }
}

// =============================================================================
// State observations
// =============================================================================

/// Representation-neutral complex amplitude observation.
///
/// This is an observation/result type, NOT the canonical storage scalar.
///
/// It deliberately uses `f64` so a caller can inspect an amplitude without
/// exposing whether the underlying representation stores f32, f64, GPU-native
/// values, compressed values, or provider-native data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexAmplitude {
    /// Real component.
    pub real: f64,

    /// Imaginary component.
    pub imaginary: f64,
}

impl ComplexAmplitude {
    /// Zero amplitude.
    pub const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    /// Creates an amplitude.
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    /// Squared magnitude.
    ///
    /// Returns `None` if the input is non-finite or the calculation becomes
    /// non-finite.
    pub fn norm_squared(self) -> Option<f64> {
        if !self.real.is_finite() || !self.imaginary.is_finite() {
            return None;
        }

        let value = self
            .real
            .mul_add(self.real, self.imaginary * self.imaginary);

        value.is_finite().then_some(value)
    }
}

/// Representation-neutral probability observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateProbability {
    /// Computational-basis probability.
    pub probability: f64,
}

impl StateProbability {
    /// Creates a validated probability.
    pub fn new(probability: f64) -> StateResult<Self> {
        if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
            return Err(MemoryError::invalid_argument(
                "quantum-state probability must be finite and within [0, 1]",
            ));
        }

        Ok(Self { probability })
    }
}

// =============================================================================
// State operation result
// =============================================================================

/// Provider-neutral result metadata returned after a state operation.
///
/// The actual measurement payload belongs to `measurement.rs`; this type only
/// reports execution-level facts that are useful to all representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateOperationResult {
    /// Operation identity, when supplied.
    pub operation_id: Option<OperationId>,

    /// Operation category.
    pub kind: StateOperationKind,

    /// Whether the state was modified.
    pub state_modified: bool,

    /// Whether synchronization may now be required.
    pub requires_synchronization: bool,

    /// Provider-defined execution duration, when known.
    pub execution_time: Option<Duration>,

    /// Number of state resources touched.
    pub touched_qubits: QubitCount,

    /// Optional provider execution identifier.
    ///
    /// This is intentionally opaque and contains no credentials.
    pub provider_execution_id: Option<String>,
}

impl StateOperationResult {
    /// Creates a result from an operation.
    pub fn for_operation<O: StateOperation + ?Sized>(operation: &O) -> Self {
        Self {
            operation_id: operation.operation_id(),
            kind: operation.kind(),
            state_modified: operation.kind().is_mutating(),
            requires_synchronization: false,
            execution_time: None,
            touched_qubits: QubitCount::new(operation.logical_qubits().len()),
            provider_execution_id: None,
        }
    }
}

// =============================================================================
// QuantumState trait
// =============================================================================

/// Canonical provider-neutral quantum-state abstraction.
///
/// # What implementations must guarantee
///
/// An implementation MUST:
///
/// 1. maintain its declared state invariants;
/// 2. reject invalid operations instead of silently ignoring them;
/// 3. never fabricate unavailable information;
/// 4. never expose raw device pointers;
/// 5. never expose unsafe memory;
/// 6. never silently switch representations;
/// 7. never silently exceed configured memory limits;
/// 8. preserve logical/physical qubit identity distinctions;
/// 9. report capabilities accurately;
/// 10. return structured `MemoryError` values for failures.
///
/// # Object safety
///
/// The trait intentionally contains no generic methods and no associated
/// types. This allows the runtime to store heterogeneous state implementations
/// behind:
///
/// ```text
/// Box<dyn QuantumState>
/// Arc<dyn QuantumState>
/// &dyn QuantumState
/// ```
///
/// This is essential for automatic representation selection and backend
/// polymorphism.
pub trait QuantumState: Send + Sync {
    /// Returns immutable state metadata.
    fn metadata(&self) -> &StateMetadata;

    /// Returns supported capabilities.
    fn capabilities(&self) -> StateCapabilities;

    /// Returns current lifecycle state.
    fn lifecycle(&self) -> StateLifecycle;

    /// Returns current consistency state.
    fn consistency(&self) -> StateConsistency;

    /// Returns whether this state can satisfy a set of requirements.
    fn supports(&self, requirements: &StateRequirements) -> bool {
        if self.metadata().qubit_count < requirements.qubits {
            return false;
        }

        if self.metadata().classical_bit_count < requirements.classical_bits {
            return false;
        }

        if !self.capabilities().contains(requirements.capabilities) {
            return false;
        }

        if let Some(domain) = requirements.execution_domain {
            if self.metadata().execution_domain != domain {
                return false;
            }
        }

        if let Some(location) = requirements.storage_location {
            if self.metadata().storage_location != location {
                return false;
            }
        }

        if let Some(representation) = requirements.representation.as_ref() {
            if &self.metadata().representation != representation {
                return false;
            }
        }

        if let Some(maximum) = requirements.maximum_memory {
            if self.metadata().memory_bytes > maximum {
                return false;
            }
        }

        if requirements.deterministic_replay
            && !self
                .capabilities()
                .contains(StateCapabilities::DETERMINISTIC_REPLAY)
        {
            return false;
        }

        if !requirements.allow_custom_operations
            && self
                .capabilities()
                .contains(StateCapabilities::CUSTOM_OPERATIONS)
        {
            // Custom-operation capability does not itself disqualify a state.
            // This branch deliberately does nothing.
        }

        true
    }

    /// Validates an operation without mutating state.
    ///
    /// Implementations should perform representation-specific validation here.
    fn validate_operation(&self, operation: &dyn StateOperation) -> StateResult<()> {
        validate_operation_descriptor(operation)?;

        if self.lifecycle() != StateLifecycle::Ready {
            return Err(MemoryError::invalid_argument(
                "quantum state is not ready to execute an operation",
            ));
        }

        let required = operation.kind().required_capabilities();

        if !self.capabilities().contains(required) {
            return Err(MemoryError::unsupported_operation(
                operation.name(),
                self.metadata().representation.as_str(),
            ));
        }

        let qubit_count = self.metadata().qubit_count.get();

        for qubit in operation.logical_qubits() {
            if qubit.index() >= qubit_count {
                return Err(MemoryError::out_of_bounds(
                    qubit.index(),
                    qubit_count,
                    "logical quantum state",
                ));
            }
        }

        Ok(())
    }

    /// Executes one provider-neutral operation.
    ///
    /// The operation adapter is responsible for translating canonical IR or
    /// backend-specific execution semantics into this contract.
    fn apply_operation(
        &mut self,
        operation: &dyn StateOperation,
    ) -> StateResult<StateOperationResult>;

    /// Synchronizes the state when the representation/storage location
    /// requires it.
    ///
    /// Implementations that do not expose synchronization return
    /// `UnsupportedOperation` through the canonical error model.
    fn synchronize(&mut self) -> StateResult<()> {
        Ok(())
    }

    /// Returns an amplitude if the representation exposes amplitude semantics.
    ///
    /// `Ok(None)` means amplitude access is not part of the representation's
    /// public contract. It MUST NOT mean "amplitude is zero".
    fn amplitude(&self, _basis_index: usize) -> StateResult<Option<ComplexAmplitude>> {
        Ok(None)
    }

    /// Returns a computational-basis probability if supported.
    ///
    /// `Ok(None)` means the provider does not expose direct probability
    /// inspection.
    fn probability(&self, _basis_index: usize) -> StateResult<Option<StateProbability>> {
        Ok(None)
    }

    /// Creates an independent state copy when supported.
    ///
    /// Real QPUs commonly cannot clone their physical quantum state. Such
    /// providers should advertise no `FORK` capability and return the
    /// canonical unsupported-operation error.
    fn fork(&self) -> StateResult<Box<dyn QuantumState>> {
        Err(MemoryError::unsupported_operation(
            "state fork",
            self.metadata().representation.as_str(),
        ))
    }

    /// Returns the current state identifier.
    fn state_id(&self) -> StateId {
        self.metadata().state_id
    }

    /// Returns the logical qubit count.
    fn qubit_count(&self) -> QubitCount {
        self.metadata().qubit_count
    }

    /// Returns the current declared memory consumption.
    fn memory_bytes(&self) -> ByteCount {
        self.metadata().memory_bytes
    }

    /// Returns the state representation name.
    fn representation(&self) -> &str {
        self.metadata().representation.as_str()
    }

    /// Returns the storage location.
    fn storage_location(&self) -> StateStorageLocation {
        self.metadata().storage_location
    }

    /// Returns the execution domain.
    fn execution_domain(&self) -> StateExecutionDomain {
        self.metadata().execution_domain
    }

    /// Returns whether the state is ready for execution.
    fn is_ready(&self) -> bool {
        self.lifecycle() == StateLifecycle::Ready
    }

    /// Returns whether the state is externally owned.
    fn is_backend_native(&self) -> bool {
        self.capabilities()
            .contains(StateCapabilities::BACKEND_NATIVE)
    }

    /// Performs a final invariant check before the state crosses a subsystem
    /// boundary.
    ///
    /// Representation-specific implementations should extend this validation.
    fn validate_invariants(&self) -> StateResult<()> {
        let metadata = self.metadata();

        if metadata.provider_name.as_deref().is_some_and(|value| {
            value.is_empty() || value.len() > StateRepresentationName::MAX_LENGTH
        }) {
            return Err(MemoryError::invalid_argument(
                "state provider name is invalid",
            ));
        }

        if metadata.device_name.as_deref().is_some_and(|value| {
            value.is_empty() || value.len() > StateRepresentationName::MAX_LENGTH
        }) {
            return Err(MemoryError::invalid_argument(
                "state device name is invalid",
            ));
        }

        if metadata.memory_bytes.get() == 0 && metadata.qubit_count.is_non_zero() {
            // Zero bytes is legal for provider-native/opaque states, but only
            // when the representation explicitly does not expose local
            // storage.
            if !metadata.storage_location.is_external() {
                return Err(MemoryError::invariant_violation(
                    "non-external quantum state has non-zero qubits but zero declared memory",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// State provider factory boundary
// =============================================================================

/// Provider-neutral factory for constructing quantum states.
///
/// This is deliberately separate from `QuantumState` so state implementations
/// do not need to know how the runtime discovers providers.
///
/// `state_vector.rs`, `density_matrix.rs`, `stabilizer.rs`, tensor networks,
/// and backend-native adapters can each provide their own factory.
pub trait QuantumStateProvider: Send + Sync {
    /// Stable provider name.
    fn provider_name(&self) -> &str;

    /// Returns the capabilities this provider can construct.
    fn capabilities(&self) -> StateCapabilities;

    /// Returns whether the provider can satisfy the requested requirements.
    fn supports(&self, requirements: &StateRequirements) -> bool {
        self.capabilities().contains(requirements.capabilities)
    }

    /// Creates a new state.
    ///
    /// The provider must perform all memory-limit checks before allocating
    /// large resources.
    fn create(
        &self,
        requirements: &StateRequirements,
    ) -> StateResult<Box<dyn QuantumState>>;
}

// =============================================================================
// Provider selection
// =============================================================================

/// Result of provider capability negotiation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateProviderMatch {
    /// Provider name.
    pub provider_name: String,

    /// Representation exposed by the provider.
    pub representation: StateRepresentationName,

    /// Capability set.
    pub capabilities: StateCapabilities,

    /// Execution domain.
    pub execution_domain: StateExecutionDomain,

    /// Storage location.
    pub storage_location: StateStorageLocation,
}

impl StateProviderMatch {
    /// Creates a provider-match description.
    pub fn new(
        provider_name: impl Into<String>,
        representation: StateRepresentationName,
        capabilities: StateCapabilities,
        execution_domain: StateExecutionDomain,
        storage_location: StateStorageLocation,
    ) -> StateResult<Self> {
        let provider_name = provider_name.into();

        validate_optional_name(Some(&provider_name), "state provider name")?;

        Ok(Self {
            provider_name,
            representation,
            capabilities,
            execution_domain,
            storage_location,
        })
    }
}

// =============================================================================
// Operation validation
// =============================================================================

/// Validates a provider-neutral operation descriptor.
///
/// This function checks only invariants that are independent of the actual
/// state representation.
pub fn validate_operation_descriptor(
    operation: &dyn StateOperation,
) -> StateResult<()> {
    let name = operation.name();

    if name.is_empty() {
        return Err(MemoryError::invalid_argument(
            "quantum state operation name cannot be empty",
        ));
    }

    if name.len() > 256 {
        return Err(MemoryError::invalid_argument(
            "quantum state operation name exceeds the maximum length",
        ));
    }

    if name.chars().any(char::is_control) {
        return Err(MemoryError::invalid_argument(
            "quantum state operation name contains a control character",
        ));
    }

    validate_unique_qubits(operation.logical_qubits())?;

    validate_unique_physical_qubits(operation.physical_qubits())?;

    validate_finite_parameters(operation.parameters())?;

    if operation.kind() == StateOperationKind::Measure
        && operation.logical_qubits().is_empty()
    {
        return Err(MemoryError::invalid_argument(
            "measurement operation must address at least one logical qubit",
        ));
    }

    if operation.kind() == StateOperationKind::Reset
        && operation.logical_qubits().is_empty()
    {
        return Err(MemoryError::invalid_argument(
            "reset operation must address at least one logical qubit",
        ));
    }

    if operation.kind() == StateOperationKind::Custom
        && !operation
            .semantics()
            .contains(StateOperationSemantics::PROVIDER_DEFINED)
    {
        return Err(MemoryError::invalid_argument(
            "custom state operation must explicitly declare provider-defined semantics",
        ));
    }

    Ok(())
}

/// Validates logical-qubit uniqueness.
///
/// Qubit ordering is preserved. Only duplicate operands are rejected.
pub fn validate_unique_qubits(qubits: &[QubitId]) -> StateResult<()> {
    for (position, qubit) in qubits.iter().enumerate() {
        if qubits[position + 1..].iter().any(|other| other == qubit) {
            return Err(MemoryError::invalid_argument(
                "state operation contains duplicate logical qubit operands",
            ));
        }
    }

    Ok(())
}

/// Validates physical-qubit uniqueness.
pub fn validate_unique_physical_qubits(
    qubits: &[PhysicalQubitId],
) -> StateResult<()> {
    for (position, qubit) in qubits.iter().enumerate() {
        if qubits[position + 1..].iter().any(|other| other == qubit) {
            return Err(MemoryError::invalid_argument(
                "state operation contains duplicate physical qubit operands",
            ));
        }
    }

    Ok(())
}

/// Validates that all operation parameters are finite.
pub fn validate_finite_parameters(parameters: &[f64]) -> StateResult<()> {
    if parameters.iter().any(|value| !value.is_finite()) {
        return Err(MemoryError::invalid_argument(
            "quantum operation parameters must be finite",
        ));
    }

    Ok(())
}

/// Validates an optional provider/device name.
pub fn validate_optional_name(
    value: Option<&str>,
    field: &str,
) -> StateResult<()> {
    let Some(value) = value else {
        return Ok(());
    };

    if value.is_empty()
        || value.len() > StateRepresentationName::MAX_LENGTH
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(MemoryError::invalid_argument(&format!(
            "{field} is empty, too long, has surrounding whitespace, or contains control characters",
        )));
    }

    Ok(())
}

// =============================================================================
// State requirement validation
// =============================================================================

/// Validates state requirements before provider selection.
pub fn validate_state_requirements(
    requirements: &StateRequirements,
) -> StateResult<()> {
    if let Some(maximum_memory) = requirements.maximum_memory {
        if maximum_memory.is_zero() && requirements.qubits.is_non_zero() {
            return Err(MemoryError::invalid_argument(
                "non-zero quantum state cannot have a zero memory budget",
            ));
        }
    }

    if let Some(representation) = requirements.representation.as_ref() {
        if representation.as_str().is_empty() {
            return Err(MemoryError::invalid_argument(
                "required state representation cannot be empty",
            ));
        }
    }

    Ok(())
}

// =============================================================================
// State operation compatibility
// =============================================================================

/// Checks whether a state can execute a specific operation.
///
/// This function does not mutate the state.
pub fn check_operation_compatibility(
    state: &dyn QuantumState,
    operation: &dyn StateOperation,
) -> StateResult<()> {
    validate_state_requirements(&StateRequirements {
        qubits: state.qubit_count(),
        capabilities: operation.kind().required_capabilities(),
        ..StateRequirements::default()
    })?;

    state.validate_operation(operation)
}

// =============================================================================
// State-provider compatibility
// =============================================================================

/// Checks whether a provider can satisfy a set of state requirements.
pub fn check_provider_compatibility(
    provider: &dyn QuantumStateProvider,
    requirements: &StateRequirements,
) -> StateResult<()> {
    validate_state_requirements(requirements)?;

    if !provider.supports(requirements) {
        return Err(MemoryError::unsupported_operation(
            "requested quantum state provider capabilities",
            provider.provider_name(),
        ));
    }

    Ok(())
}

// =============================================================================
// State transition helpers
// =============================================================================

/// Validates a state before beginning an operation.
///
/// The executor may use this helper immediately before changing state.
pub fn begin_state_operation(
    state: &dyn QuantumState,
    operation: &dyn StateOperation,
) -> StateResult<()> {
    if state.lifecycle() != StateLifecycle::Ready {
        return Err(MemoryError::invalid_argument(
            "quantum state must be ready before beginning an operation",
        ));
    }

    check_operation_compatibility(state, operation)
}

/// Validates that a state can be used after an operation.
pub fn finish_state_operation(
    state: &dyn QuantumState,
) -> StateResult<()> {
    if state.lifecycle().is_terminal() {
        return Err(MemoryError::lifetime_violation(
            "quantum state became terminal during operation",
        ));
    }

    state.validate_invariants()
}

// =============================================================================
// State identity helpers
// =============================================================================

/// Returns whether two state objects represent the same logical state
/// resource.
pub fn same_state_identity(
    left: &dyn QuantumState,
    right: &dyn QuantumState,
) -> bool {
    left.state_id() == right.state_id()
}

/// Returns whether two state objects can be considered independent resources.
///
/// Different IDs alone do not prove mathematical independence, so this helper
/// only establishes resource identity independence.
pub fn independently_identified(
    left: &dyn QuantumState,
    right: &dyn QuantumState,
) -> bool {
    !same_state_identity(left, right)
}

// =============================================================================
// State capability validation
// =============================================================================

/// Validates that the declared state capabilities are internally sensible.
///
/// This catches obviously contradictory declarations while deliberately
/// allowing provider-specific combinations.
pub fn validate_capabilities(
    capabilities: StateCapabilities,
    metadata: &StateMetadata,
) -> StateResult<()> {
    if metadata.execution_domain.is_qpu()
        && capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS)
        && metadata.storage_location.is_external()
    {
        // A QPU may expose amplitudes through tomography or provider-specific
        // APIs, so this is not inherently invalid.
        //
        // The combination is therefore permitted.
    }

    if capabilities.contains(StateCapabilities::PURE_STATE)
        && capabilities.contains(StateCapabilities::MIXED_STATE)
    {
        // A representation may support both semantics, for example a backend
        // that can switch between pure and mixed-state modes.
        //
        // Therefore this is not an error.
    }

    if metadata.execution_domain.is_qpu()
        && capabilities.contains(StateCapabilities::FORK)
        && !capabilities.contains(StateCapabilities::BACKEND_NATIVE)
    {
        // A simulator-like QPU abstraction can theoretically implement a
        // provider-side fork. Do not reject it merely because it is unusual.
    }

    Ok(())
}

// =============================================================================
// State resource accounting
// =============================================================================

/// Immutable resource summary for diagnostics and scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StateResourceUsage {
    /// Logical qubits.
    pub qubits: QubitCount,

    /// Classical bits.
    pub classical_bits: ClassicalBitCount,

    /// Amplitude/elements when meaningful.
    pub amplitudes: Option<AmplitudeCount>,

    /// Declared bytes.
    pub bytes: ByteCount,
}

impl StateResourceUsage {
    /// Creates usage information from state metadata.
    pub const fn from_metadata(metadata: &StateMetadata) -> Self {
        Self {
            qubits: metadata.qubit_count,
            classical_bits: metadata.classical_bit_count,
            amplitudes: metadata.amplitude_count,
            bytes: metadata.memory_bytes,
        }
    }
}

// =============================================================================
// State execution context
// =============================================================================

/// Immutable context supplied to state execution implementations.
///
/// This keeps runtime/executor policy outside the state representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateExecutionContext {
    /// Optional operation identity.
    pub operation_id: Option<OperationId>,

    /// Optional provider execution/job identifier.
    pub provider_execution_id: Option<String>,

    /// Whether deterministic execution is required.
    pub deterministic: bool,

    /// Maximum permitted operation duration.
    pub timeout: Option<Duration>,

    /// Whether provider-native operations are permitted.
    pub allow_custom_operations: bool,
}

impl Default for StateExecutionContext {
    fn default() -> Self {
        Self {
            operation_id: None,
            provider_execution_id: None,
            deterministic: false,
            timeout: None,
            allow_custom_operations: false,
        }
    }
}

impl StateExecutionContext {
    /// Creates a deterministic execution context.
    pub const fn deterministic() -> Self {
        Self {
            operation_id: None,
            provider_execution_id: None,
            deterministic: true,
            timeout: None,
            allow_custom_operations: false,
        }
    }
}

// =============================================================================
// State executor extension
// =============================================================================

/// Optional extension trait for implementations that need explicit execution
/// context.
///
/// The base `QuantumState` contract remains small and object-safe.
///
/// A simulator/backend may implement this extension when it needs:
///
/// - deadlines;
/// - provider job IDs;
/// - deterministic execution policy;
/// - custom-operation authorization.
///
/// Consumers should detect/support this trait through their concrete provider
/// rather than changing `QuantumState`.
pub trait QuantumStateExecutor: QuantumState {
    /// Executes an operation with explicit execution context.
    fn apply_operation_with_context(
        &mut self,
        operation: &dyn StateOperation,
        context: &StateExecutionContext,
    ) -> StateResult<StateOperationResult>;

    /// Returns whether this executor honors operation deadlines.
    fn honors_deadlines(&self) -> bool {
        false
    }
}

// =============================================================================
// State backend/provider extension
// =============================================================================

/// Extension boundary for backend-native state implementations.
///
/// This is intentionally generic enough to represent:
///
/// - physical QPUs;
/// - remote simulators;
/// - hardware emulators;
/// - vendor accelerators;
/// - custom quantum processors.
///
/// It does not expose vendor SDK types.
pub trait BackendNativeState: QuantumState {
    /// Returns a stable backend/provider identifier.
    fn backend_name(&self) -> &str;

    /// Returns a provider-owned execution/session identifier when available.
    fn backend_state_identifier(&self) -> Option<&str> {
        None
    }

    /// Returns whether the provider owns the authoritative state.
    fn provider_authoritative(&self) -> bool {
        true
    }
}

// =============================================================================
// State migration extension
// =============================================================================

/// Abstract migration destination.
///
/// The concrete representation/storage taxonomy is owned by
/// `representation.rs` and the migration subsystem.
pub trait StateMigrationTarget: Send + Sync {
    /// Stable target name.
    fn target_name(&self) -> &str;

    /// Returns whether the target can accept this state.
    fn accepts(&self, state: &dyn QuantumState) -> bool;

    /// Migrates the state without mutating the source unless the concrete
    /// migration contract explicitly permits move semantics.
    fn migrate(
        &self,
        state: &dyn QuantumState,
    ) -> StateResult<Box<dyn QuantumState>>;
}

// =============================================================================
// State collection
// =============================================================================

/// Read-only collection of heterogeneous quantum states.
///
/// This type is intentionally lightweight. It does not own allocation policy
/// or lifecycle management; it only provides deterministic state lookup.
#[derive(Default)]
pub struct StateCollection {
    states: Vec<Box<dyn QuantumState>>,
}

impl StateCollection {
    /// Creates an empty collection.
    pub const fn new() -> Self {
        Self { states: Vec::new() }
    }

    /// Creates a collection with capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            states: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of states.
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// Inserts a state.
    ///
    /// Duplicate state IDs are rejected.
    pub fn insert(
        &mut self,
        state: Box<dyn QuantumState>,
    ) -> StateResult<()> {
        if self
            .states
            .iter()
            .any(|existing| existing.state_id() == state.state_id())
        {
            return Err(MemoryError::invalid_argument(
                "state collection cannot contain duplicate state IDs",
            ));
        }

        state.validate_invariants()?;
        self.states.push(state);

        Ok(())
    }

    /// Finds a state by ID.
    pub fn get(&self, state_id: StateId) -> Option<&dyn QuantumState> {
        self.states
            .iter()
            .find(|state| state.state_id() == state_id)
            .map(|state| state.as_ref())
    }

    /// Iterates states deterministically in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &dyn QuantumState> {
        self.states.iter().map(|state| state.as_ref())
    }

    /// Removes and returns a state by ID.
    pub fn remove(
        &mut self,
        state_id: StateId,
    ) -> Option<Box<dyn QuantumState>> {
        let position = self
            .states
            .iter()
            .position(|state| state.state_id() == state_id)?;

        Some(self.states.remove(position))
    }
}

// =============================================================================
// Compile-time/API tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestOperation {
        kind: StateOperationKind,
        name: String,
        logical_qubits: Vec<QubitId>,
        physical_qubits: Vec<PhysicalQubitId>,
        classical_bits: Vec<ClassicalBitId>,
        parameters: Vec<f64>,
        semantics: StateOperationSemantics,
    }

    impl StateOperation for TestOperation {
        fn kind(&self) -> StateOperationKind {
            self.kind
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn logical_qubits(&self) -> &[QubitId] {
            &self.logical_qubits
        }

        fn physical_qubits(&self) -> &[PhysicalQubitId] {
            &self.physical_qubits
        }

        fn classical_bits(&self) -> &[ClassicalBitId] {
            &self.classical_bits
        }

        fn parameters(&self) -> &[f64] {
            &self.parameters
        }

        fn semantics(&self) -> StateOperationSemantics {
            self.semantics
        }
    }

    fn operation() -> TestOperation {
        TestOperation {
            kind: StateOperationKind::Unitary,
            name: "test_unitary".to_owned(),
            logical_qubits: vec![QubitId::new(0)],
            physical_qubits: vec![PhysicalQubitId::new(3)],
            classical_bits: Vec::new(),
            parameters: vec![0.5],
            semantics: StateOperationSemantics::UNITARY
                | StateOperationSemantics::REVERSIBLE,
        }
    }

    #[test]
    fn capability_sets_are_composable() {
        let capabilities = StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET;

        assert!(capabilities.contains(StateCapabilities::UNITARY));
        assert!(capabilities.contains(StateCapabilities::MEASUREMENT));
        assert!(capabilities.contains(StateCapabilities::RESET));
        assert!(!capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS));
    }

    #[test]
    fn operation_descriptor_accepts_valid_operation() {
        let op = operation();

        assert!(validate_operation_descriptor(&op).is_ok());
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let mut op = operation();
        op.logical_qubits = vec![QubitId::new(0), QubitId::new(0)];

        assert!(validate_operation_descriptor(&op).is_err());
    }

    #[test]
    fn duplicate_physical_qubits_are_rejected() {
        let mut op = operation();
        op.physical_qubits =
            vec![PhysicalQubitId::new(3), PhysicalQubitId::new(3)];

        assert!(validate_operation_descriptor(&op).is_err());
    }

    #[test]
    fn non_finite_parameters_are_rejected() {
        let mut op = operation();
        op.parameters = vec![f64::NAN];

        assert!(validate_operation_descriptor(&op).is_err());
    }

    #[test]
    fn measurement_requires_qubits() {
        let mut op = operation();
        op.kind = StateOperationKind::Measure;
        op.name = "measure".to_owned();
        op.logical_qubits.clear();

        assert!(validate_operation_descriptor(&op).is_err());
    }

    #[test]
    fn reset_requires_qubits() {
        let mut op = operation();
        op.kind = StateOperationKind::Reset;
        op.name = "reset".to_owned();
        op.logical_qubits.clear();

        assert!(validate_operation_descriptor(&op).is_err());
    }

    #[test]
    fn probability_is_bounded() {
        assert!(StateProbability::new(0.0).is_ok());
        assert!(StateProbability::new(0.5).is_ok());
        assert!(StateProbability::new(1.0).is_ok());

        assert!(StateProbability::new(-0.1).is_err());
        assert!(StateProbability::new(1.1).is_err());
        assert!(StateProbability::new(f64::NAN).is_err());
        assert!(StateProbability::new(f64::INFINITY).is_err());
    }

    #[test]
    fn amplitude_norm_is_checked() {
        let amplitude = ComplexAmplitude::new(3.0, 4.0);

        assert_eq!(amplitude.norm_squared(), Some(25.0));
    }

    #[test]
    fn lifecycle_semantics_are_stable() {
        assert!(StateLifecycle::Ready.accepts_operations());
        assert!(!StateLifecycle::Released.accepts_operations());
        assert!(StateLifecycle::Released.is_terminal());
        assert!(StateLifecycle::Failed.is_terminal());
        assert!(StateLifecycle::Suspended.is_resumable());
    }

    #[test]
    fn execution_domain_classification_is_stable() {
        assert!(StateExecutionDomain::Qpu.is_qpu());
        assert!(StateExecutionDomain::LocalSimulator.is_simulator());
        assert!(StateExecutionDomain::Distributed.is_distributed());
        assert!(StateExecutionDomain::Hybrid.is_hybrid());
    }

    #[test]
    fn storage_location_classification_is_stable() {
        assert!(StateStorageLocation::Host.is_host_readable());
        assert!(StateStorageLocation::Device.is_device());
        assert!(StateStorageLocation::Distributed.is_distributed());
        assert!(StateStorageLocation::Remote.is_external());
    }

    #[test]
    fn state_requirements_default_is_unrestricted() {
        let requirements = StateRequirements::default();

        assert_eq!(requirements.qubits, QubitCount::ZERO);
        assert_eq!(
            requirements.capabilities,
            StateCapabilities::NONE
        );
        assert!(!requirements.deterministic_replay);
    }

    #[test]
    fn operation_kind_capabilities_are_defined() {
        assert!(
            StateOperationKind::Unitary
                .required_capabilities()
                .contains(StateCapabilities::UNITARY)
        );

        assert!(
            StateOperationKind::Measure
                .required_capabilities()
                .contains(StateCapabilities::MEASUREMENT)
        );

        assert!(
            StateOperationKind::Reset
                .required_capabilities()
                .contains(StateCapabilities::RESET)
        );
    }
}