//! Zamani Quantum Memory — Representation Contract
//!
//! This module defines the canonical, provider-neutral representation
//! vocabulary for `quantum::memory`.
//!
//! # Architectural responsibility
//!
//! `representation.rs` answers one fundamental question:
//!
//! > What kind of quantum state/resource is being represented, where does it
//! > live, what numerical model does it use, and what operations can it
//! > legitimately expose?
//!
//! It deliberately does NOT implement:
//!
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer/tableau algorithms;
//! - sparse-state algorithms;
//! - tensor-network algorithms;
//! - QPU communication;
//! - GPU kernels;
//! - distributed communication;
//! - routing;
//! - scheduling;
//! - circuit optimization;
//! - compiler parsing;
//! - QEC decoding;
//! - benchmark protocols;
//! - vendor SDKs.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                       execution layer
//!                              |
//!                              v
//!                 quantum::memory::representation
//!                              |
//!       +----------------------+----------------------+
//!       |                      |                      |
//!       v                      v                      v
//! StateVector             DensityMatrix          Stabilizer
//!       |                      |                      |
//!       +----------------------+----------------------+
//!                              |
//!              +---------------+---------------+
//!              |               |               |
//!              v               v               v
//!          SparseState     TensorNetwork   BackendNative
//!              |               |               |
//!              +---------------+---------------+
//!                              |
//!                provider/hardware boundary
//!                              |
//!          +---------+---------+---------+---------+
//!          |         |         |         |         |
//!          v         v         v         v         v
//!       CPU/GPU   QPU       Photonic   Annealing  Distributed
//! ```
//!
//! # Critical rule
//!
//! A representation is NOT the same thing as a hardware vendor, backend,
//! simulator, or execution provider.
//!
//! For example:
//!
//! ```text
//! StateVector
//!     may execute on CPU
//!     may execute on GPU
//!     may execute distributed
//!     may be used by a simulator
//!
//! BackendNative
//!     may represent an IBM QPU
//!     may represent an Ion-trap QPU
//!     may represent a photonic processor
//!     may represent an annealer
//!     may represent another provider
//! ```
//!
//! This separation is what allows Zamani to support different QPUs without
//! modifying the memory representation contract.
//!
//! # QPU neutrality
//!
//! Not every quantum computer exposes a conventional addressable quantum
//! memory containing amplitudes.
//!
//! Some systems expose:
//!
//! - gate-model qubits;
//! - qudits;
//! - continuous-variable modes;
//! - photonic modes;
//! - analog Hamiltonian evolution;
//! - annealing variables;
//! - measurement-based computation;
//! - provider-native execution handles;
//! - remote quantum resources.
//!
//! Therefore this module explicitly supports both mathematical state
//! representations and opaque/provider-managed representations.
//!
//! # Dependency policy
//!
//! This module depends only on the Rust standard library and Serde, which is
//! already part of Zamani's quantum-memory foundation.
//!
//! It does not depend on:
//!
//! - state.rs;
//! - state_vector.rs;
//! - density_matrix.rs;
//! - stabilizer.rs;
//! - tensor_network.rs;
//! - gpu.rs;
//! - distributed.rs;
//! - hardware implementations;
//! - benchmarking.
//!
//! This makes the file independently completable.
//!
//! # Integration contract
//!
//! Later modules MUST use these canonical concepts instead of creating their
//! own competing enums for:
//!
//! - representation;
//! - representation family;
//! - precision;
//! - storage location;
//! - execution mode;
//! - representation capabilities;
//! - representation selection policy.
//!
//! If another module needs a new representation, it should extend the
//! representation taxonomy here rather than silently inventing a second one.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly-only features are used.
//!
//! # Safety
//!
//! This module is explicitly safe Rust.
//!
//! No `unsafe` code is permitted.
//!
//! # Determinism
//!
//! Representation descriptions and selection policies are deterministic.
//!
//! This module does not perform stochastic state evolution and therefore does
//! not own an RNG.
//!
//! # Serialization
//!
//! Representation metadata is serializable because snapshots, checkpoints,
//! migration records, telemetry, and execution manifests need to know exactly
//! how a state was represented.
//!
//! Persistence formats remain responsible for schema/version envelopes and
//! integrity verification.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the representation contract.
pub const REPRESENTATION_SCHEMA_ID: &str = "zamani.quantum.memory.representation";

/// Semantic version of the representation contract.
pub const REPRESENTATION_SCHEMA_VERSION: u16 = 1;

/// Maximum length of a custom representation identifier.
pub const MAX_CUSTOM_REPRESENTATION_NAME: usize = 256;

// =============================================================================
// Representation family
// =============================================================================

/// Broad mathematical/physical family to which a representation belongs.
///
/// The family is intentionally broader than the concrete representation
/// enum. This allows implementations to add concrete representations without
/// changing every consumer's conceptual model.
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
pub enum RepresentationFamily {
    /// Pure discrete-variable quantum states.
    PureState,

    /// Mixed quantum states.
    MixedState,

    /// Stabilizer/Clifford states.
    Stabilizer,

    /// Sparse mathematical states.
    Sparse,

    /// Tensor-network representations.
    TensorNetwork,

    /// Pauli-frame or error-correction-oriented state metadata.
    PauliFrame,

    /// Photonic discrete-variable representations.
    PhotonicDiscreteVariable,

    /// Continuous-variable quantum representations.
    ContinuousVariable,

    /// Qudit or higher-dimensional discrete-variable systems.
    Qudit,

    /// Analog/Hamiltonian evolution representations.
    Analog,

    /// Quantum annealing/adiabatic representations.
    Annealing,

    /// Measurement-based quantum computation representations.
    MeasurementBased,

    /// Provider-owned execution state.
    BackendNative,

    /// Hybrid quantum/classical execution representation.
    Hybrid,

    /// User/provider-defined representation.
    Custom,
}

impl RepresentationFamily {
    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PureState => "pure_state",
            Self::MixedState => "mixed_state",
            Self::Stabilizer => "stabilizer",
            Self::Sparse => "sparse",
            Self::TensorNetwork => "tensor_network",
            Self::PauliFrame => "pauli_frame",
            Self::PhotonicDiscreteVariable => "photonic_discrete_variable",
            Self::ContinuousVariable => "continuous_variable",
            Self::Qudit => "qudit",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::BackendNative => "backend_native",
            Self::Hybrid => "hybrid",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the family normally represents a conventional
    /// state-vector-like mathematical object.
    pub const fn is_conventional_state(self) -> bool {
        matches!(
            self,
            Self::PureState
                | Self::MixedState
                | Self::Stabilizer
                | Self::Sparse
                | Self::TensorNetwork
                | Self::PauliFrame
        )
    }

    /// Returns whether the family may be provider-managed rather than
    /// directly represented as host-readable amplitudes.
    pub const fn is_provider_managed(self) -> bool {
        matches!(
            self,
            Self::BackendNative
                | Self::PhotonicDiscreteVariable
                | Self::ContinuousVariable
                | Self::Analog
                | Self::Annealing
                | Self::MeasurementBased
                | Self::Hybrid
        )
    }
}

impl fmt::Display for RepresentationFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Concrete representation
// =============================================================================

/// Canonical provider-neutral quantum-state/resource representation.
///
/// This enum describes what the memory subsystem believes it is storing or
/// referring to. It does not identify a vendor or backend.
///
/// The variants intentionally cover both conventional simulators and quantum
/// hardware models that cannot expose amplitudes directly.
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
pub enum StateRepresentation {
    /// Dense pure state:
    ///
    /// `|psi>` represented by `2^n` amplitudes for n two-level qubits.
    StateVector,

    /// Dense mixed state:
    ///
    /// `rho`, normally requiring `4^n` complex matrix elements for n qubits.
    DensityMatrix,

    /// Clifford/stabilizer tableau representation.
    Stabilizer,

    /// Sparse basis-index-to-amplitude representation.
    SparseState,

    /// Matrix Product State.
    Mps,

    /// Matrix Product Operator.
    Mpo,

    /// Generic tensor-network representation.
    TensorNetwork,

    /// Pauli-frame representation.
    PauliFrame,

    /// Classical-shadow or measurement-shadow state record.
    ClassicalShadow,

    /// Photonic Fock-space representation.
    PhotonicFock,

    /// Photonic Gaussian-state representation.
    PhotonicGaussian,

    /// Continuous-variable phase-space representation.
    ContinuousVariable,

    /// Finite-dimensional qudit state-vector representation.
    QuditStateVector,

    /// Finite-dimensional qudit density-matrix representation.
    QuditDensityMatrix,

    /// Analog/Hamiltonian evolution resource.
    AnalogHamiltonian,

    /// Quantum-annealing/adiabatic resource.
    Annealing,

    /// Measurement-based quantum computation resource.
    MeasurementBased,

    /// Opaque provider-owned state.
    BackendNative,

    /// Opaque simulator/provider state not covered by the built-in models.
    External,

    /// User/provider-defined representation identified by metadata.
    Custom,
}

impl StateRepresentation {
    /// Returns the representation's broad family.
    pub const fn family(self) -> RepresentationFamily {
        match self {
            Self::StateVector => RepresentationFamily::PureState,
            Self::DensityMatrix => RepresentationFamily::MixedState,
            Self::Stabilizer => RepresentationFamily::Stabilizer,
            Self::SparseState => RepresentationFamily::Sparse,
            Self::Mps | Self::Mpo | Self::TensorNetwork => {
                RepresentationFamily::TensorNetwork
            }
            Self::PauliFrame => RepresentationFamily::PauliFrame,
            Self::ClassicalShadow => RepresentationFamily::Hybrid,
            Self::PhotonicFock | Self::PhotonicGaussian => {
                RepresentationFamily::PhotonicDiscreteVariable
            }
            Self::ContinuousVariable => RepresentationFamily::ContinuousVariable,
            Self::QuditStateVector | Self::QuditDensityMatrix => {
                RepresentationFamily::Qudit
            }
            Self::AnalogHamiltonian => RepresentationFamily::Analog,
            Self::Annealing => RepresentationFamily::Annealing,
            Self::MeasurementBased => RepresentationFamily::MeasurementBased,
            Self::BackendNative => RepresentationFamily::BackendNative,
            Self::External => RepresentationFamily::BackendNative,
            Self::Custom => RepresentationFamily::Custom,
        }
    }

    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Stabilizer => "stabilizer",
            Self::SparseState => "sparse_state",
            Self::Mps => "mps",
            Self::Mpo => "mpo",
            Self::TensorNetwork => "tensor_network",
            Self::PauliFrame => "pauli_frame",
            Self::ClassicalShadow => "classical_shadow",
            Self::PhotonicFock => "photonic_fock",
            Self::PhotonicGaussian => "photonic_gaussian",
            Self::ContinuousVariable => "continuous_variable",
            Self::QuditStateVector => "qudit_state_vector",
            Self::QuditDensityMatrix => "qudit_density_matrix",
            Self::AnalogHamiltonian => "analog_hamiltonian",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::BackendNative => "backend_native",
            Self::External => "external",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the representation exposes a conventional pure-state
    /// amplitude vector.
    pub const fn is_state_vector_like(self) -> bool {
        matches!(
            self,
            Self::StateVector | Self::QuditStateVector
        )
    }

    /// Returns whether the representation is a mixed-state representation.
    pub const fn is_density_matrix_like(self) -> bool {
        matches!(
            self,
            Self::DensityMatrix | Self::QuditDensityMatrix
        )
    }

    /// Returns whether the representation is stabilizer-oriented.
    pub const fn is_stabilizer_like(self) -> bool {
        matches!(self, Self::Stabilizer | Self::PauliFrame)
    }

    /// Returns whether the representation is tensor-network-based.
    pub const fn is_tensor_network(self) -> bool {
        matches!(
            self,
            Self::Mps | Self::Mpo | Self::TensorNetwork
        )
    }

    /// Returns whether the representation is provider-native/opaque.
    pub const fn is_backend_native(self) -> bool {
        matches!(
            self,
            Self::BackendNative | Self::External
        )
    }

    /// Returns whether the representation can naturally model non-qubit
    /// finite-dimensional systems.
    pub const fn supports_non_binary_systems(self) -> bool {
        matches!(
            self,
            Self::PhotonicFock
                | Self::PhotonicGaussian
                | Self::ContinuousVariable
                | Self::QuditStateVector
                | Self::QuditDensityMatrix
                | Self::AnalogHamiltonian
                | Self::Annealing
                | Self::BackendNative
                | Self::External
                | Self::Custom
        )
    }

    /// Returns whether the representation is intrinsically vendor-neutral.
    ///
    /// `BackendNative` remains vendor-neutral at this layer because it
    /// contains no provider-specific type.
    pub const fn is_provider_neutral(self) -> bool {
        true
    }
}

impl fmt::Display for StateRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Precision
// =============================================================================

/// Numerical precision/scalar model associated with a representation.
///
/// This enum is metadata. It does not itself implement arithmetic.
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
pub enum Precision {
    /// Boolean/classical-bit precision.
    Bool,

    /// 8-bit integer metadata.
    U8,

    /// 16-bit integer metadata.
    U16,

    /// 32-bit integer metadata.
    U32,

    /// 64-bit integer metadata.
    U64,

    /// 32-bit floating-point real scalar.
    F32,

    /// 64-bit floating-point real scalar.
    F64,

    /// 16-bit floating-point scalar.
    F16,

    /// Brain floating-point 16-bit scalar.
    BF16,

    /// Complex values whose real and imaginary components are F32.
    ComplexF32,

    /// Complex values whose real and imaginary components are F64.
    ComplexF64,

    /// Extended/provider-defined precision.
    Extended,

    /// Arbitrary precision.
    Arbitrary,

    /// Fixed-point/provider-defined numerical representation.
    FixedPoint,

    /// Representation is not meaningfully described by a classical scalar.
    ProviderDefined,
}

impl Precision {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::F16 => "f16",
            Self::BF16 => "bf16",
            Self::ComplexF32 => "complex_f32",
            Self::ComplexF64 => "complex_f64",
            Self::Extended => "extended",
            Self::Arbitrary => "arbitrary",
            Self::FixedPoint => "fixed_point",
            Self::ProviderDefined => "provider_defined",
        }
    }

    /// Returns the known byte size of a scalar where it is fixed by the
    /// representation contract.
    pub const fn byte_size(self) -> Option<u8> {
        match self {
            Self::Bool => Some(1),
            Self::U8 => Some(1),
            Self::U16 => Some(2),
            Self::U32 => Some(4),
            Self::U64 => Some(8),
            Self::F16 | Self::BF16 => Some(2),
            Self::F32 => Some(4),
            Self::F64 => Some(8),
            Self::ComplexF32 => Some(8),
            Self::ComplexF64 => Some(16),
            Self::Extended
            | Self::Arbitrary
            | Self::FixedPoint
            | Self::ProviderDefined => None,
        }
    }

    /// Returns whether the precision is floating-point based.
    pub const fn is_floating_point(self) -> bool {
        matches!(
            self,
            Self::F16
                | Self::BF16
                | Self::F32
                | Self::F64
                | Self::Extended
                | Self::Arbitrary
        )
    }

    /// Returns whether the precision can represent complex amplitudes.
    pub const fn supports_complex_amplitudes(self) -> bool {
        matches!(
            self,
            Self::ComplexF32
                | Self::ComplexF64
                | Self::Extended
                | Self::Arbitrary
                | Self::ProviderDefined
        )
    }

    /// Returns an approximate ordering for known floating-point precision.
    ///
    /// This is intended for policy comparisons, not numerical computation.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Bool | Self::U8 => 1,
            Self::U16 | Self::F16 | Self::BF16 => 2,
            Self::U32 | Self::F32 | Self::ComplexF32 => 3,
            Self::U64 | Self::F64 | Self::ComplexF64 => 4,
            Self::Extended => 5,
            Self::Arbitrary => 6,
            Self::FixedPoint => 3,
            Self::ProviderDefined => 0,
        }
    }
}

impl fmt::Display for Precision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Storage location
// =============================================================================

/// Where representation storage is owned or physically located.
///
/// This abstraction deliberately does not expose pointers, file descriptors,
/// device pointers, CUDA objects, Metal objects, MPI handles, or provider SDK
/// types.
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
pub enum StorageLocation {
    /// Ordinary host memory.
    Host,

    /// Pinned/page-locked host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared host-device memory.
    Unified,

    /// Memory distributed over multiple execution nodes.
    Distributed,

    /// Memory owned by a remote execution system.
    Remote,

    /// Storage controlled by an external provider.
    External,

    /// Representation exposes no conventional addressable storage.
    Opaque,
}

impl StorageLocation {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::Remote => "remote",
            Self::External => "external",
            Self::Opaque => "opaque",
        }
    }

    /// Returns whether ordinary host code may directly read the storage.
    pub const fn is_host_accessible(self) -> bool {
        matches!(
            self,
            Self::Host | Self::PinnedHost | Self::Unified
        )
    }

    /// Returns whether the location represents accelerator memory.
    pub const fn is_device(self) -> bool {
        matches!(self, Self::Device | Self::Unified)
    }

    /// Returns whether it can span multiple nodes.
    pub const fn is_distributed(self) -> bool {
        matches!(self, Self::Distributed)
    }

    /// Returns whether ownership is external to the memory core.
    pub const fn is_external(self) -> bool {
        matches!(
            self,
            Self::Remote | Self::External | Self::Opaque
        )
    }
}

impl fmt::Display for StorageLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Execution mode
// =============================================================================

/// Execution model associated with a representation.
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
pub enum ExecutionMode {
    /// Exact classical simulation.
    Simulation,

    /// Hardware emulation using a device model.
    Emulation,

    /// Physical quantum hardware execution.
    Qpu,

    /// Hybrid classical/quantum execution.
    Hybrid,

    /// Multi-node distributed execution.
    Distributed,

    /// Analog quantum evolution.
    Analog,

    /// Quantum annealing/adiabatic execution.
    Annealing,

    /// Measurement-based execution.
    MeasurementBased,

    /// Provider-defined execution mode.
    ProviderDefined,
}

impl ExecutionMode {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simulation => "simulation",
            Self::Emulation => "emulation",
            Self::Qpu => "qpu",
            Self::Hybrid => "hybrid",
            Self::Distributed => "distributed",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::ProviderDefined => "provider_defined",
        }
    }

    /// Returns whether this mode represents physical quantum hardware.
    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether this mode is classical simulation/emulation.
    pub const fn is_simulation(self) -> bool {
        matches!(self, Self::Simulation | Self::Emulation)
    }

    /// Returns whether the mode is hybrid.
    pub const fn is_hybrid(self) -> bool {
        matches!(self, Self::Hybrid)
    }
}

impl fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Representation capabilities
// =============================================================================

/// Individual capability provided by a representation.
///
/// These are semantic capabilities, not implementation details.
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
#[repr(u8)]
#[non_exhaustive]
pub enum RepresentationCapability {
    /// Direct amplitude/state-element access.
    AmplitudeAccess = 0,

    /// Direct probability access.
    ProbabilityAccess = 1,

    /// Exact normalization operation.
    Normalization = 2,

    /// Single-qubit unitary application.
    SingleQubitUnitary = 3,

    /// Two-qubit unitary application.
    TwoQubitUnitary = 4,

    /// Arbitrary multi-qubit unitary application.
    MultiQubitUnitary = 5,

    /// General quantum channel/noise operation.
    QuantumChannels = 6,

    /// Mid-circuit measurement.
    MidCircuitMeasurement = 7,

    /// State collapse.
    MeasurementCollapse = 8,

    /// Reset.
    Reset = 9,

    /// Partial trace.
    PartialTrace = 10,

    /// Tensor product.
    TensorProduct = 11,

    /// Expectation-value evaluation.
    ExpectationValues = 12,

    /// Exact state cloning.
    ExactClone = 13,

    /// Snapshot support.
    Snapshot = 14,

    /// Checkpoint support.
    Checkpoint = 15,

    /// Host-side serialization.
    Serialization = 16,

    /// GPU/device execution.
    DeviceExecution = 17,

    /// Distributed execution.
    DistributedExecution = 18,

    /// Dynamic qubit allocation.
    DynamicQubitAllocation = 19,

    /// Dynamic qubit release.
    DynamicQubitRelease = 20,

    /// Non-binary/qudit systems.
    NonBinarySystems = 21,

    /// Photonic modes.
    PhotonicModes = 22,

    /// Continuous-variable modes.
    ContinuousVariables = 23,

    /// Analog Hamiltonian evolution.
    AnalogEvolution = 24,

    /// Annealing/adiabatic evolution.
    Annealing = 25,

    /// Measurement-based quantum computing.
    MeasurementBased = 26,

    /// Provider-native execution.
    BackendNativeExecution = 27,

    /// Remote execution.
    RemoteExecution = 28,

    /// Classical side-channel/state metadata.
    ClassicalCompanionMemory = 29,

    /// Representation migration.
    Migration = 30,
}

impl RepresentationCapability {
    /// Returns the bit corresponding to this capability.
    pub const fn bit(self) -> u64 {
        1u64 << (self as u8)
    }

    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmplitudeAccess => "amplitude_access",
            Self::ProbabilityAccess => "probability_access",
            Self::Normalization => "normalization",
            Self::SingleQubitUnitary => "single_qubit_unitary",
            Self::TwoQubitUnitary => "two_qubit_unitary",
            Self::MultiQubitUnitary => "multi_qubit_unitary",
            Self::QuantumChannels => "quantum_channels",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::MeasurementCollapse => "measurement_collapse",
            Self::Reset => "reset",
            Self::PartialTrace => "partial_trace",
            Self::TensorProduct => "tensor_product",
            Self::ExpectationValues => "expectation_values",
            Self::ExactClone => "exact_clone",
            Self::Snapshot => "snapshot",
            Self::Checkpoint => "checkpoint",
            Self::Serialization => "serialization",
            Self::DeviceExecution => "device_execution",
            Self::DistributedExecution => "distributed_execution",
            Self::DynamicQubitAllocation => "dynamic_qubit_allocation",
            Self::DynamicQubitRelease => "dynamic_qubit_release",
            Self::NonBinarySystems => "non_binary_systems",
            Self::PhotonicModes => "photonic_modes",
            Self::ContinuousVariables => "continuous_variables",
            Self::AnalogEvolution => "analog_evolution",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::BackendNativeExecution => "backend_native_execution",
            Self::RemoteExecution => "remote_execution",
            Self::ClassicalCompanionMemory => "classical_companion_memory",
            Self::Migration => "migration",
        }
    }
}

impl fmt::Display for RepresentationCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Capability set
// =============================================================================

/// Compact deterministic set of representation capabilities.
///
/// A manual bit-set is used instead of adding another dependency merely for
/// flags.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct CapabilitySet(u64);

impl CapabilitySet {
    /// Empty capability set.
    pub const EMPTY: Self = Self(0);

    /// Creates a capability set from raw bits.
    ///
    /// This is intentionally public for stable serialization/deserialization
    /// and forward compatibility. Callers should normally use `insert`.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the raw capability bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Creates a set containing one capability.
    pub const fn single(capability: RepresentationCapability) -> Self {
        Self(capability.bit())
    }

    /// Inserts a capability.
    pub const fn with(self, capability: RepresentationCapability) -> Self {
        Self(self.0 | capability.bit())
    }

    /// Inserts a capability in place.
    pub fn insert(&mut self, capability: RepresentationCapability) {
        self.0 |= capability.bit();
    }

    /// Removes a capability.
    pub fn remove(&mut self, capability: RepresentationCapability) {
        self.0 &= !capability.bit();
    }

    /// Returns whether the capability exists.
    pub const fn contains(self, capability: RepresentationCapability) -> bool {
        self.0 & capability.bit() != 0
    }

    /// Returns whether all requested capabilities are present.
    pub const fn contains_all(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Returns whether at least one requested capability is present.
    pub const fn contains_any(self, requested: Self) -> bool {
        self.0 & requested.0 != 0
    }

    /// Union of two capability sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Intersection of two capability sets.
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Returns whether the set is empty.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the number of capabilities.
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }
}

// =============================================================================
// Resource scaling
// =============================================================================

/// Qualitative resource-scaling class of a representation.
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
pub enum ScalingClass {
    /// Approximately constant in the number of qubits for the representation
    /// metadata itself.
    Constant,

    /// Polynomial resource growth.
    Polynomial,

    /// Exponential resource growth.
    Exponential,

    /// Potentially exponential depending on entanglement/bond dimension.
    EntanglementDependent,

    /// Depends primarily on sparsity.
    SparsityDependent,

    /// Depends on physical system/model parameters.
    PhysicalModelDependent,

    /// Provider-controlled.
    ProviderDefined,
}

impl ScalingClass {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constant => "constant",
            Self::Polynomial => "polynomial",
            Self::Exponential => "exponential",
            Self::EntanglementDependent => "entanglement_dependent",
            Self::SparsityDependent => "sparsity_dependent",
            Self::PhysicalModelDependent => "physical_model_dependent",
            Self::ProviderDefined => "provider_defined",
        }
    }
}

impl fmt::Display for ScalingClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Representation identifier
// =============================================================================

/// Validated identifier for a custom representation.
///
/// Built-in representations use [`StateRepresentation`]. This type exists so
/// provider-specific or future Zamani representations can be named without
/// exposing vendor SDK types.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct CustomRepresentationName(String);

impl CustomRepresentationName {
    /// Creates a validated custom representation name.
    ///
    /// Rules:
    ///
    /// - non-empty;
    /// - bounded;
    /// - no leading/trailing whitespace;
    /// - no control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, RepresentationValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RepresentationValidationError::EmptyName);
        }

        if value.len() > MAX_CUSTOM_REPRESENTATION_NAME {
            return Err(RepresentationValidationError::NameTooLong {
                length: value.len(),
                maximum: MAX_CUSTOM_REPRESENTATION_NAME,
            });
        }

        if value.trim() != value {
            return Err(RepresentationValidationError::WhitespaceInName);
        }

        if value.chars().any(char::is_control) {
            return Err(RepresentationValidationError::ControlCharacterInName);
        }

        Ok(Self(value))
    }

    /// Returns the name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CustomRepresentationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Validation error
// =============================================================================

/// Errors specific to representation metadata construction.
///
/// This is deliberately independent from `MemoryError` so this foundational
/// file remains usable in isolation. Higher-level memory modules can map this
/// error into `MemoryError::InvalidArgument` or
/// `MemoryError::UnsupportedRepresentation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepresentationValidationError {
    /// A representation name is empty.
    EmptyName,

    /// A representation name exceeds the permitted length.
    NameTooLong {
        /// Actual length.
        length: usize,

        /// Maximum permitted length.
        maximum: usize,
    },

    /// A representation name contains leading/trailing whitespace.
    WhitespaceInName,

    /// A representation name contains a control character.
    ControlCharacterInName,

    /// A descriptor contains an impossible combination of fields.
    InvalidCombination {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// Required capability is absent.
    MissingCapability(RepresentationCapability),

    /// A custom representation is missing its custom identifier.
    MissingCustomName,

    /// A built-in representation was incorrectly paired with a custom name.
    UnexpectedCustomName,

    /// A maximum qubit count is zero when a nonzero limit is required.
    InvalidMaximumQubits,

    /// A dimension is invalid.
    InvalidDimension,

    /// A policy has no usable candidates.
    NoCandidate,
}

impl fmt::Display for RepresentationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                f.write_str("representation name cannot be empty")
            }
            Self::NameTooLong { length, maximum } => {
                write!(
                    f,
                    "representation name length {length} exceeds maximum {maximum}"
                )
            }
            Self::WhitespaceInName => {
                f.write_str(
                    "representation name cannot contain leading or trailing whitespace",
                )
            }
            Self::ControlCharacterInName => {
                f.write_str("representation name cannot contain control characters")
            }
            Self::InvalidCombination { reason } => {
                write!(f, "invalid representation metadata combination: {reason}")
            }
            Self::MissingCapability(capability) => {
                write!(f, "required capability is missing: {capability}")
            }
            Self::MissingCustomName => {
                f.write_str("custom representation requires a custom name")
            }
            Self::UnexpectedCustomName => {
                f.write_str("built-in representation cannot carry a custom name")
            }
            Self::InvalidMaximumQubits => {
                f.write_str("maximum qubit count must be non-zero")
            }
            Self::InvalidDimension => {
                f.write_str("system dimension must be at least two")
            }
            Self::NoCandidate => {
                f.write_str("representation policy has no usable candidate")
            }
        }
    }
}

impl std::error::Error for RepresentationValidationError {}

// =============================================================================
// Representation descriptor
// =============================================================================

/// Complete immutable description of a representation implementation.
///
/// A descriptor is metadata. It does not own the state itself.
///
/// This type is intended to be stored by:
///
/// - state providers;
/// - migration logic;
/// - diagnostics;
/// - snapshots;
/// - checkpoint manifests;
/// - backend adapters;
/// - simulators;
/// - representation registries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationDescriptor {
    /// Built-in representation kind.
    representation: StateRepresentation,

    /// Optional custom representation identifier.
    custom_name: Option<CustomRepresentationName>,

    /// Broad mathematical/physical family.
    family: RepresentationFamily,

    /// Numerical precision/scalar model.
    precision: Precision,

    /// Storage location.
    storage: StorageLocation,

    /// Execution mode.
    execution: ExecutionMode,

    /// Supported semantic operations.
    capabilities: CapabilitySet,

    /// Resource scaling behavior.
    scaling: ScalingClass,

    /// Whether the representation is deterministic for identical inputs.
    deterministic: bool,

    /// Whether the representation is mathematically exact up to its declared
    /// numerical precision.
    exact_within_precision: bool,

    /// Whether the state can be directly serialized by the memory layer.
    directly_serializable: bool,

    /// Whether direct host-side state inspection is supported.
    host_inspectable: bool,

    /// Optional maximum number of qubits supported by this implementation.
    maximum_qubits: Option<u64>,

    /// Optional system dimension for non-binary systems.
    ///
    /// For ordinary qubits this is `2`.
    ///
    /// For qudits this may be `d >= 2`.
    ///
    /// For continuous-variable/provider-native systems this is normally
    /// `None`.
    system_dimension: Option<u32>,
}

impl RepresentationDescriptor {
    /// Creates a descriptor.
    pub fn new(
        representation: StateRepresentation,
        precision: Precision,
        storage: StorageLocation,
        execution: ExecutionMode,
        capabilities: CapabilitySet,
        scaling: ScalingClass,
    ) -> Result<Self, RepresentationValidationError> {
        let descriptor = Self {
            representation,
            custom_name: None,
            family: representation.family(),
            precision,
            storage,
            execution,
            capabilities,
            scaling,
            deterministic: true,
            exact_within_precision: true,
            directly_serializable: false,
            host_inspectable: storage.is_host_accessible(),
            maximum_qubits: None,
            system_dimension: Some(2),
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Creates a descriptor for a custom representation.
    pub fn custom(
        name: CustomRepresentationName,
        family: RepresentationFamily,
        precision: Precision,
        storage: StorageLocation,
        execution: ExecutionMode,
        capabilities: CapabilitySet,
        scaling: ScalingClass,
    ) -> Result<Self, RepresentationValidationError> {
        let descriptor = Self {
            representation: StateRepresentation::Custom,
            custom_name: Some(name),
            family,
            precision,
            storage,
            execution,
            capabilities,
            scaling,
            deterministic: true,
            exact_within_precision: false,
            directly_serializable: false,
            host_inspectable: storage.is_host_accessible(),
            maximum_qubits: None,
            system_dimension: None,
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Returns the representation kind.
    pub const fn representation(&self) -> StateRepresentation {
        self.representation
    }

    /// Returns the custom representation name.
    pub fn custom_name(&self) -> Option<&CustomRepresentationName> {
        self.custom_name.as_ref()
    }

    /// Returns the family.
    pub const fn family(&self) -> RepresentationFamily {
        self.family
    }

    /// Returns the precision.
    pub const fn precision(&self) -> Precision {
        self.precision
    }

    /// Returns the storage location.
    pub const fn storage(&self) -> StorageLocation {
        self.storage
    }

    /// Returns the execution mode.
    pub const fn execution(&self) -> ExecutionMode {
        self.execution
    }

    /// Returns capabilities.
    pub const fn capabilities(&self) -> CapabilitySet {
        self.capabilities
    }

    /// Returns scaling class.
    pub const fn scaling(&self) -> ScalingClass {
        self.scaling
    }

    /// Returns whether the implementation is deterministic.
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns whether mathematical guarantees are exact within precision.
    pub const fn exact_within_precision(&self) -> bool {
        self.exact_within_precision
    }

    /// Returns whether the memory layer can serialize it directly.
    pub const fn directly_serializable(&self) -> bool {
        self.directly_serializable
    }

    /// Returns whether host-side inspection is supported.
    pub const fn host_inspectable(&self) -> bool {
        self.host_inspectable
    }

    /// Returns the maximum qubit count, if constrained.
    pub const fn maximum_qubits(&self) -> Option<u64> {
        self.maximum_qubits
    }

    /// Returns the finite system dimension, if applicable.
    pub const fn system_dimension(&self) -> Option<u32> {
        self.system_dimension
    }

    /// Sets deterministic behavior metadata.
    pub const fn with_deterministic(mut self, value: bool) -> Self {
        self.deterministic = value;
        self
    }

    /// Sets precision-exactness metadata.
    pub const fn with_exact_within_precision(mut self, value: bool) -> Self {
        self.exact_within_precision = value;
        self
    }

    /// Sets direct serialization support.
    pub const fn with_direct_serialization(mut self, value: bool) -> Self {
        self.directly_serializable = value;
        self
    }

    /// Sets host inspectability.
    pub const fn with_host_inspectable(mut self, value: bool) -> Self {
        self.host_inspectable = value;
        self
    }

    /// Sets an implementation maximum qubit count.
    pub const fn with_maximum_qubits(mut self, value: Option<u64>) -> Self {
        self.maximum_qubits = value;
        self
    }

    /// Sets a finite system dimension.
    pub const fn with_system_dimension(mut self, value: Option<u32>) -> Self {
        self.system_dimension = value;
        self
    }

    /// Validates the complete descriptor.
    pub fn validate(&self) -> Result<(), RepresentationValidationError> {
        if self.representation == StateRepresentation::Custom {
            if self.custom_name.is_none() {
                return Err(RepresentationValidationError::MissingCustomName);
            }
        } else if self.custom_name.is_some() {
            return Err(RepresentationValidationError::UnexpectedCustomName);
        }

        if let Some(maximum_qubits) = self.maximum_qubits {
            if maximum_qubits == 0 {
                return Err(
                    RepresentationValidationError::InvalidMaximumQubits
                );
            }
        }

        if let Some(dimension) = self.system_dimension {
            if dimension < 2 {
                return Err(RepresentationValidationError::InvalidDimension);
            }
        }

        // Dense state vectors need complex amplitudes.
        if self.representation.is_state_vector_like()
            && !self.precision.supports_complex_amplitudes()
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "state-vector representations require a complex-capable precision",
                },
            );
        }

        // Dense density matrices also require complex values.
        if self.representation.is_density_matrix_like()
            && !self.precision.supports_complex_amplitudes()
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "density-matrix representations require a complex-capable precision",
                },
            );
        }

        // A provider-native representation should not pretend it has direct
        // host amplitude access unless explicitly declared by the provider.
        if self.representation.is_backend_native()
            && self.capabilities.contains(
                RepresentationCapability::AmplitudeAccess,
            )
            && !self.host_inspectable
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "backend-native amplitude access requires host inspectability",
                },
            );
        }

        // Device storage is meaningful for representations that can execute
        // there. A descriptor may still use provider-managed storage, but
        // generic device storage must expose device execution.
        if self.storage.is_device()
            && !self.capabilities.contains(
                RepresentationCapability::DeviceExecution,
            )
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "device/unified storage requires device-execution capability",
                },
            );
        }

        // Distributed storage must expose distributed execution.
        if self.storage.is_distributed()
            && !self.capabilities.contains(
                RepresentationCapability::DistributedExecution,
            )
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "distributed storage requires distributed-execution capability",
                },
            );
        }

        // QPU execution is provider-managed unless the representation is
        // explicitly declared as backend-native or an externally controlled
        // representation.
        if self.execution.is_qpu()
            && !self.representation.is_backend_native()
            && self.storage.is_host_accessible()
            && self.capabilities.contains(
                RepresentationCapability::AmplitudeAccess,
            )
        {
            return Err(
                RepresentationValidationError::InvalidCombination {
                    reason:
                        "ordinary host amplitude state must not masquerade as a physical QPU resource",
                },
            );
        }

        Ok(())
    }

    /// Returns whether this descriptor satisfies all required capabilities.
    pub const fn supports_all(
        &self,
        required: CapabilitySet,
    ) -> bool {
        self.capabilities.contains_all(required)
    }

    /// Returns whether the representation supports a capability.
    pub const fn supports(
        &self,
        capability: RepresentationCapability,
    ) -> bool {
        self.capabilities.contains(capability)
    }

    /// Returns a stable representation identifier.
    pub fn identifier(&self) -> String {
        match &self.custom_name {
            Some(name) => name.as_str().to_owned(),
            None => self.representation.as_str().to_owned(),
        }
    }
}

// =============================================================================
// Representation requirements
// =============================================================================

/// Requirements used when selecting a representation.
///
/// This type is intentionally declarative. It does not choose a simulator or
/// hardware provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationRequirements {
    /// Required representation family, if any.
    family: Option<RepresentationFamily>,

    /// Required concrete representation, if any.
    representation: Option<StateRepresentation>,

    /// Required precision.
    minimum_precision: Option<Precision>,

    /// Required storage location.
    storage: Option<StorageLocation>,

    /// Required execution mode.
    execution: Option<ExecutionMode>,

    /// Required capabilities.
    capabilities: CapabilitySet,

    /// Required minimum number of qubits.
    minimum_qubits: Option<u64>,

    /// Optional maximum permitted memory footprint.
    maximum_bytes: Option<u64>,

    /// Whether provider-native execution is permitted.
    allow_backend_native: bool,

    /// Whether host-inspectable state is required.
    require_host_inspection: bool,

    /// Whether exact state cloning is required.
    require_exact_clone: bool,

    /// Whether direct serialization is required.
    require_serialization: bool,
}

impl Default for RepresentationRequirements {
    fn default() -> Self {
        Self {
            family: None,
            representation: None,
            minimum_precision: None,
            storage: None,
            execution: None,
            capabilities: CapabilitySet::EMPTY,
            minimum_qubits: None,
            maximum_bytes: None,
            allow_backend_native: true,
            require_host_inspection: false,
            require_exact_clone: false,
            require_serialization: false,
        }
    }
}

impl RepresentationRequirements {
    /// Creates empty requirements.
    pub const fn new() -> Self {
        Self {
            family: None,
            representation: None,
            minimum_precision: None,
            storage: None,
            execution: None,
            capabilities: CapabilitySet::EMPTY,
            minimum_qubits: None,
            maximum_bytes: None,
            allow_backend_native: true,
            require_host_inspection: false,
            require_exact_clone: false,
            require_serialization: false,
        }
    }

    /// Requires a representation family.
    pub const fn with_family(
        mut self,
        family: RepresentationFamily,
    ) -> Self {
        self.family = Some(family);
        self
    }

    /// Requires a concrete representation.
    pub const fn with_representation(
        mut self,
        representation: StateRepresentation,
    ) -> Self {
        self.representation = Some(representation);
        self
    }

    /// Requires a minimum precision.
    pub const fn with_minimum_precision(
        mut self,
        precision: Precision,
    ) -> Self {
        self.minimum_precision = Some(precision);
        self
    }

    /// Requires a storage location.
    pub const fn with_storage(
        mut self,
        storage: StorageLocation,
    ) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Requires an execution mode.
    pub const fn with_execution(
        mut self,
        execution: ExecutionMode,
    ) -> Self {
        self.execution = Some(execution);
        self
    }

    /// Requires capabilities.
    pub const fn with_capabilities(
        mut self,
        capabilities: CapabilitySet,
    ) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Requires a minimum qubit count.
    pub const fn with_minimum_qubits(
        mut self,
        qubits: u64,
    ) -> Self {
        self.minimum_qubits = Some(qubits);
        self
    }

    /// Sets the maximum permitted memory footprint.
    pub const fn with_maximum_bytes(
        mut self,
        bytes: u64,
    ) -> Self {
        self.maximum_bytes = Some(bytes);
        self
    }

    /// Controls whether backend-native representations are allowed.
    pub const fn with_backend_native(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_backend_native = allowed;
        self
    }

    /// Requires host inspection.
    pub const fn requiring_host_inspection(
        mut self,
        required: bool,
    ) -> Self {
        self.require_host_inspection = required;
        self
    }

    /// Requires exact cloning.
    pub const fn requiring_exact_clone(
        mut self,
        required: bool,
    ) -> Self {
        self.require_exact_clone = required;
        self
    }

    /// Requires serialization.
    pub const fn requiring_serialization(
        mut self,
        required: bool,
    ) -> Self {
        self.require_serialization = required;
        self
    }

    /// Returns whether a descriptor satisfies these requirements.
    pub fn matches(
        &self,
        descriptor: &RepresentationDescriptor,
    ) -> bool {
        if let Some(family) = self.family {
            if descriptor.family() != family {
                return false;
            }
        }

        if let Some(representation) = self.representation {
            if descriptor.representation() != representation {
                return false;
            }
        }

        if let Some(precision) = self.minimum_precision {
            if descriptor.precision().rank() < precision.rank() {
                return false;
            }
        }

        if let Some(storage) = self.storage {
            if descriptor.storage() != storage {
                return false;
            }
        }

        if let Some(execution) = self.execution {
            if descriptor.execution() != execution {
                return false;
            }
        }

        if !descriptor.supports_all(self.capabilities) {
            return false;
        }

        if let Some(minimum_qubits) = self.minimum_qubits {
            if let Some(maximum) = descriptor.maximum_qubits() {
                if maximum < minimum_qubits {
                    return false;
                }
            }
        }

        if self.require_host_inspection
            && !descriptor.host_inspectable()
        {
            return false;
        }

        if self.require_exact_clone
            && !descriptor.supports(
                RepresentationCapability::ExactClone,
            )
        {
            return false;
        }

        if self.require_serialization
            && !descriptor.directly_serializable()
        {
            return false;
        }

        if !self.allow_backend_native
            && descriptor.representation().is_backend_native()
        {
            return false;
        }

        true
    }

    /// Returns whether a candidate is compatible with the requested memory
    /// budget.
    ///
    /// `maximum_bytes` is an admission constraint. Actual memory estimation
    /// belongs to `limits.rs` and the concrete representation.
    pub const fn maximum_bytes(&self) -> Option<u64> {
        self.maximum_bytes
    }
}

// =============================================================================
// Representation policy
// =============================================================================

/// Policy controlling how the execution layer chooses a representation.
///
/// Memory does not decide which quantum algorithm should run. It only
/// provides a deterministic representation-selection contract.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum RepresentationPolicy {
    /// Require one exact representation.
    Forced(StateRepresentation),

    /// Prefer the backend-native representation.
    BackendNative,

    /// Automatically choose from available candidates.
    Automatic,

    /// Prefer a state-vector representation.
    PreferStateVector,

    /// Prefer a density matrix.
    PreferDensityMatrix,

    /// Prefer a stabilizer representation.
    PreferStabilizer,

    /// Prefer sparse representation.
    PreferSparse,

    /// Prefer tensor-network representation.
    PreferTensorNetwork,
}

impl Default for RepresentationPolicy {
    fn default() -> Self {
        Self::Automatic
    }
}

impl RepresentationPolicy {
    /// Returns whether the policy forces a single representation.
    pub const fn is_forced(self) -> bool {
        matches!(self, Self::Forced(_))
    }

    /// Returns the forced representation, if any.
    pub const fn forced(self) -> Option<StateRepresentation> {
        match self {
            Self::Forced(value) => Some(value),
            _ => None,
        }
    }
}

// =============================================================================
// Selection context
// =============================================================================

/// Runtime information used to select a representation.
///
/// This is deliberately independent from any specific backend.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct RepresentationSelectionContext {
    /// Requested workload requirements.
    requirements: RepresentationRequirements,

    /// Requested representation policy.
    policy: RepresentationPolicy,

    /// Number of qubits required by the workload.
    qubits: u64,

    /// Available host memory in bytes, when known.
    available_host_bytes: Option<u64>,

    /// Available device memory in bytes, when known.
    available_device_bytes: Option<u64>,

    /// Whether distributed execution is available.
    distributed_available: bool,

    /// Whether device acceleration is available.
    device_available: bool,

    /// Whether physical QPU execution is requested.
    qpu_requested: bool,
}

impl RepresentationSelectionContext {
    /// Creates a selection context.
    pub const fn new(
        requirements: RepresentationRequirements,
        policy: RepresentationPolicy,
        qubits: u64,
    ) -> Self {
        Self {
            requirements,
            policy,
            qubits,
            available_host_bytes: None,
            available_device_bytes: None,
            distributed_available: false,
            device_available: false,
            qpu_requested: false,
        }
    }

    /// Sets known host memory.
    pub const fn with_host_memory(
        mut self,
        bytes: Option<u64>,
    ) -> Self {
        self.available_host_bytes = bytes;
        self
    }

    /// Sets known device memory.
    pub const fn with_device_memory(
        mut self,
        bytes: Option<u64>,
    ) -> Self {
        self.available_device_bytes = bytes;
        self
    }

    /// Sets distributed availability.
    pub const fn with_distributed(
        mut self,
        available: bool,
    ) -> Self {
        self.distributed_available = available;
        self
    }

    /// Sets device availability.
    pub const fn with_device(
        mut self,
        available: bool,
    ) -> Self {
        self.device_available = available;
        self
    }

    /// Sets QPU requirement.
    pub const fn with_qpu(
        mut self,
        requested: bool,
    ) -> Self {
        self.qpu_requested = requested;
        self
    }

    /// Returns requirements.
    pub const fn requirements(
        &self,
    ) -> &RepresentationRequirements {
        &self.requirements
    }

    /// Returns policy.
    pub const fn policy(&self) -> RepresentationPolicy {
        self.policy
    }

    /// Returns qubit count.
    pub const fn qubits(&self) -> u64 {
        self.qubits
    }

    /// Returns known host memory.
    pub const fn available_host_bytes(&self) -> Option<u64> {
        self.available_host_bytes
    }

    /// Returns known device memory.
    pub const fn available_device_bytes(&self) -> Option<u64> {
        self.available_device_bytes
    }

    /// Returns whether distributed execution is available.
    pub const fn distributed_available(&self) -> bool {
        self.distributed_available
    }

    /// Returns whether device execution is available.
    pub const fn device_available(&self) -> bool {
        self.device_available
    }

    /// Returns whether QPU execution is requested.
    pub const fn qpu_requested(&self) -> bool {
        self.qpu_requested
    }
}

// =============================================================================
// Candidate scoring
// =============================================================================

/// Deterministic score used internally by representation selection.
///
/// Higher scores are preferred.
///
/// The score is intentionally not exposed as an implementation-specific
/// heuristic. It is stable enough for deterministic selection, while future
/// versions can evolve the policy without changing representation identity.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct RepresentationScore {
    /// Overall score.
    value: i64,

    /// Stable tie-breaker.
    representation_rank: u16,
}

impl RepresentationScore {
    /// Creates a score.
    pub const fn new(
        value: i64,
        representation_rank: u16,
    ) -> Self {
        Self {
            value,
            representation_rank,
        }
    }

    /// Returns the score value.
    pub const fn value(self) -> i64 {
        self.value
    }

    /// Returns the deterministic tie-break rank.
    pub const fn representation_rank(self) -> u16 {
        self.representation_rank
    }
}

// =============================================================================
// Representation candidate
// =============================================================================

/// A descriptor plus the deterministic score assigned to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationCandidate {
    descriptor: RepresentationDescriptor,
    score: RepresentationScore,
}

impl RepresentationCandidate {
    /// Creates a candidate.
    pub fn new(
        descriptor: RepresentationDescriptor,
        score: RepresentationScore,
    ) -> Self {
        Self {
            descriptor,
            score,
        }
    }

    /// Returns the descriptor.
    pub const fn descriptor(
        &self,
    ) -> &RepresentationDescriptor {
        &self.descriptor
    }

    /// Returns the score.
    pub const fn score(&self) -> RepresentationScore {
        self.score
    }
}

// =============================================================================
// Selection result
// =============================================================================

/// Result of representation selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationSelection {
    descriptor: RepresentationDescriptor,
    score: RepresentationScore,
}

impl RepresentationSelection {
    /// Creates a selection result.
    pub fn new(
        descriptor: RepresentationDescriptor,
        score: RepresentationScore,
    ) -> Self {
        Self {
            descriptor,
            score,
        }
    }

    /// Returns the selected descriptor.
    pub const fn descriptor(
        &self,
    ) -> &RepresentationDescriptor {
        &self.descriptor
    }

    /// Returns the selected representation.
    pub const fn representation(
        &self,
    ) -> StateRepresentation {
        self.descriptor.representation()
    }

    /// Returns the score.
    pub const fn score(
        &self,
    ) -> RepresentationScore {
        self.score
    }
}

// =============================================================================
// Built-in descriptors
// =============================================================================

/// Returns the canonical descriptor for dense state-vector simulation.
///
/// This function contains metadata only; it never allocates state memory.
pub fn state_vector_descriptor(
    precision: Precision,
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::AmplitudeAccess)
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::Normalization)
        .with(RepresentationCapability::SingleQubitUnitary)
        .with(RepresentationCapability::TwoQubitUnitary)
        .with(RepresentationCapability::MultiQubitUnitary)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::TensorProduct)
        .with(RepresentationCapability::ExpectationValues)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::StateVector,
        precision,
        storage,
        execution,
        capabilities,
        ScalingClass::Exponential,
    )
}

/// Returns the canonical descriptor for density-matrix simulation.
pub fn density_matrix_descriptor(
    precision: Precision,
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::AmplitudeAccess)
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::Normalization)
        .with(RepresentationCapability::SingleQubitUnitary)
        .with(RepresentationCapability::TwoQubitUnitary)
        .with(RepresentationCapability::MultiQubitUnitary)
        .with(RepresentationCapability::QuantumChannels)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::PartialTrace)
        .with(RepresentationCapability::TensorProduct)
        .with(RepresentationCapability::ExpectationValues)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::DensityMatrix,
        precision,
        storage,
        execution,
        capabilities,
        ScalingClass::Exponential,
    )
}

/// Returns the canonical descriptor for stabilizer simulation.
pub fn stabilizer_descriptor(
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::Stabilizer,
        Precision::ProviderDefined,
        storage,
        execution,
        capabilities,
        ScalingClass::Polynomial,
    )
}

/// Returns the canonical descriptor for sparse-state simulation.
pub fn sparse_descriptor(
    precision: Precision,
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::AmplitudeAccess)
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::Normalization)
        .with(RepresentationCapability::SingleQubitUnitary)
        .with(RepresentationCapability::TwoQubitUnitary)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::ExpectationValues)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::SparseState,
        precision,
        storage,
        execution,
        capabilities,
        ScalingClass::SparsityDependent,
    )
}

/// Returns the canonical descriptor for MPS/tensor-network simulation.
pub fn tensor_network_descriptor(
    precision: Precision,
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::AmplitudeAccess)
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::Normalization)
        .with(RepresentationCapability::SingleQubitUnitary)
        .with(RepresentationCapability::TwoQubitUnitary)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::TensorProduct)
        .with(RepresentationCapability::ExpectationValues)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::TensorNetwork,
        precision,
        storage,
        execution,
        capabilities,
        ScalingClass::EntanglementDependent,
    )
}

/// Returns the canonical descriptor for provider-native quantum hardware.
///
/// This descriptor deliberately exposes no amplitude access by default.
pub fn backend_native_descriptor(
    storage: StorageLocation,
    execution: ExecutionMode,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::BackendNativeExecution)
        .with(RepresentationCapability::RemoteExecution)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::BackendNative,
        Precision::ProviderDefined,
        storage,
        execution,
        capabilities,
        ScalingClass::ProviderDefined,
    )
}

/// Returns the canonical descriptor for qudit state-vector simulation.
pub fn qudit_state_vector_descriptor(
    precision: Precision,
    storage: StorageLocation,
    execution: ExecutionMode,
    dimension: u32,
) -> Result<RepresentationDescriptor, RepresentationValidationError> {
    let capabilities = CapabilitySet::EMPTY
        .with(RepresentationCapability::AmplitudeAccess)
        .with(RepresentationCapability::ProbabilityAccess)
        .with(RepresentationCapability::Normalization)
        .with(RepresentationCapability::SingleQubitUnitary)
        .with(RepresentationCapability::TwoQubitUnitary)
        .with(RepresentationCapability::MultiQubitUnitary)
        .with(RepresentationCapability::MidCircuitMeasurement)
        .with(RepresentationCapability::MeasurementCollapse)
        .with(RepresentationCapability::Reset)
        .with(RepresentationCapability::TensorProduct)
        .with(RepresentationCapability::ExpectationValues)
        .with(RepresentationCapability::ExactClone)
        .with(RepresentationCapability::Snapshot)
        .with(RepresentationCapability::Checkpoint)
        .with(RepresentationCapability::Serialization)
        .with(RepresentationCapability::NonBinarySystems)
        .with(RepresentationCapability::Migration)
        .with(RepresentationCapability::ClassicalCompanionMemory);

    RepresentationDescriptor::new(
        StateRepresentation::QuditStateVector,
        precision,
        storage,
        execution,
        capabilities,
        ScalingClass::Exponential,
    )?
    .with_system_dimension(Some(dimension))
    .validate()
    .map(|_| {
        RepresentationDescriptor::new(
            StateRepresentation::QuditStateVector,
            precision,
            storage,
            execution,
            capabilities,
            ScalingClass::Exponential,
        )
        .expect("validated descriptor construction must succeed")
        .with_system_dimension(Some(dimension))
    })
}

// =============================================================================
// Deterministic selection
// =============================================================================

/// Selects the best representation from a candidate set.
///
/// Selection is deterministic:
///
/// 1. incompatible candidates are discarded;
/// 2. the policy contributes a stable preference;
/// 3. capability matches contribute score;
/// 4. execution/storage compatibility contributes score;
/// 5. representation rank is the final deterministic tie-breaker.
///
/// No randomness, global state, backend I/O, or benchmarking is involved.
pub fn select_representation(
    context: &RepresentationSelectionContext,
    candidates: &[RepresentationDescriptor],
) -> Result<RepresentationSelection, RepresentationValidationError> {
    let mut best: Option<RepresentationSelection> = None;

    for descriptor in candidates {
        if !context.requirements().matches(descriptor) {
            continue;
        }

        if let Some(maximum_qubits) = descriptor.maximum_qubits() {
            if context.qubits() > maximum_qubits {
                continue;
            }
        }

        if context.qpu_requested()
            && !descriptor.execution().is_qpu()
            && descriptor.representation()
                != StateRepresentation::BackendNative
        {
            continue;
        }

        if !context.distributed_available()
            && descriptor.storage().is_distributed()
        {
            continue;
        }

        if !context.device_available()
            && descriptor.storage().is_device()
        {
            continue;
        }

        if let Some(maximum_bytes) =
            context.requirements().maximum_bytes()
        {
            if let Some(available) =
                context.available_host_bytes()
            {
                if maximum_bytes > available
                    && descriptor.storage().is_host_accessible()
                {
                    continue;
                }
            }

            if let Some(available) =
                context.available_device_bytes()
            {
                if maximum_bytes > available
                    && descriptor.storage().is_device()
                {
                    continue;
                }
            }
        }

        if let Some(forced) = context.policy().forced() {
            if descriptor.representation() != forced {
                continue;
            }
        }

        let score = score_candidate(context, descriptor);

        let selection =
            RepresentationSelection::new(descriptor.clone(), score);

        let replace = match &best {
            None => true,
            Some(current) => selection.score() > current.score(),
        };

        if replace {
            best = Some(selection);
        }
    }

    best.ok_or(RepresentationValidationError::NoCandidate)
}

/// Calculates the deterministic score for a candidate.
fn score_candidate(
    context: &RepresentationSelectionContext,
    descriptor: &RepresentationDescriptor,
) -> RepresentationScore {
    let mut score = 0i64;

    match context.policy() {
        RepresentationPolicy::Forced(_) => {
            score += 10_000;
        }

        RepresentationPolicy::BackendNative => {
            if descriptor.representation()
                == StateRepresentation::BackendNative
            {
                score += 10_000;
            }
        }

        RepresentationPolicy::PreferStateVector => {
            if descriptor.representation()
                == StateRepresentation::StateVector
            {
                score += 5_000;
            }
        }

        RepresentationPolicy::PreferDensityMatrix => {
            if descriptor.representation()
                == StateRepresentation::DensityMatrix
            {
                score += 5_000;
            }
        }

        RepresentationPolicy::PreferStabilizer => {
            if descriptor.representation()
                == StateRepresentation::Stabilizer
            {
                score += 5_000;
            }
        }

        RepresentationPolicy::PreferSparse => {
            if descriptor.representation()
                == StateRepresentation::SparseState
            {
                score += 5_000;
            }
        }

        RepresentationPolicy::PreferTensorNetwork => {
            if descriptor.representation()
                == StateRepresentation::TensorNetwork
                || descriptor.representation()
                    == StateRepresentation::Mps
            {
                score += 5_000;
            }
        }

        RepresentationPolicy::Automatic => {}
    }

    if context.qpu_requested()
        && descriptor.execution().is_qpu()
    {
        score += 2_000;
    }

    if context.device_available()
        && descriptor.storage().is_device()
    {
        score += 500;
    }

    if context.distributed_available()
        && descriptor.storage().is_distributed()
    {
        score += 400;
    }

    if descriptor.deterministic() {
        score += 100;
    }

    if descriptor.exact_within_precision() {
        score += 100;
    }

    score += i64::from(descriptor.capabilities().count());

    RepresentationScore::new(
        score,
        representation_rank(descriptor.representation()),
    )
}

/// Stable representation rank used only as a final tie-breaker.
///
/// The ordering is intentionally explicit rather than relying on enum
/// discriminants, so adding a new representation does not accidentally
/// reorder existing deterministic decisions.
const fn representation_rank(
    representation: StateRepresentation,
) -> u16 {
    match representation {
        StateRepresentation::StateVector => 10,
        StateRepresentation::DensityMatrix => 20,
        StateRepresentation::Stabilizer => 30,
        StateRepresentation::SparseState => 40,
        StateRepresentation::Mps => 50,
        StateRepresentation::Mpo => 60,
        StateRepresentation::TensorNetwork => 70,
        StateRepresentation::PauliFrame => 80,
        StateRepresentation::ClassicalShadow => 90,
        StateRepresentation::PhotonicFock => 100,
        StateRepresentation::PhotonicGaussian => 110,
        StateRepresentation::ContinuousVariable => 120,
        StateRepresentation::QuditStateVector => 130,
        StateRepresentation::QuditDensityMatrix => 140,
        StateRepresentation::AnalogHamiltonian => 150,
        StateRepresentation::Annealing => 160,
        StateRepresentation::MeasurementBased => 170,
        StateRepresentation::BackendNative => 180,
        StateRepresentation::External => 190,
        StateRepresentation::Custom => 200,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representation_names_are_stable() {
        assert_eq!(
            StateRepresentation::StateVector.as_str(),
            "state_vector"
        );

        assert_eq!(
            StateRepresentation::DensityMatrix.as_str(),
            "density_matrix"
        );

        assert_eq!(
            StateRepresentation::BackendNative.as_str(),
            "backend_native"
        );
    }

    #[test]
    fn representation_families_are_correct() {
        assert_eq!(
            StateRepresentation::StateVector.family(),
            RepresentationFamily::PureState
        );

        assert_eq!(
            StateRepresentation::DensityMatrix.family(),
            RepresentationFamily::MixedState
        );

        assert_eq!(
            StateRepresentation::Stabilizer.family(),
            RepresentationFamily::Stabilizer
        );

        assert_eq!(
            StateRepresentation::Mps.family(),
            RepresentationFamily::TensorNetwork
        );

        assert_eq!(
            StateRepresentation::PhotonicFock.family(),
            RepresentationFamily::PhotonicDiscreteVariable
        );
    }

    #[test]
    fn capability_set_is_deterministic() {
        let set = CapabilitySet::EMPTY
            .with(RepresentationCapability::AmplitudeAccess)
            .with(RepresentationCapability::ProbabilityAccess);

        assert!(set.contains(
            RepresentationCapability::AmplitudeAccess
        ));

        assert!(set.contains(
            RepresentationCapability::ProbabilityAccess
        ));

        assert!(!set.contains(
            RepresentationCapability::QuantumChannels
        ));

        assert_eq!(set.count(), 2);
    }

    #[test]
    fn custom_names_are_validated() {
        assert!(
            CustomRepresentationName::new("zamani.custom.state")
                .is_ok()
        );

        assert!(
            CustomRepresentationName::new("")
                .is_err()
        );

        assert!(
            CustomRepresentationName::new(" leading")
                .is_err()
        );
    }

    #[test]
    fn state_vector_descriptor_is_valid() {
        let descriptor = state_vector_descriptor(
            Precision::ComplexF64,
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("state-vector descriptor must be valid");

        assert_eq!(
            descriptor.representation(),
            StateRepresentation::StateVector
        );

        assert_eq!(
            descriptor.family(),
            RepresentationFamily::PureState
        );

        assert!(
            descriptor.supports(
                RepresentationCapability::AmplitudeAccess
            )
        );
    }

    #[test]
    fn density_matrix_descriptor_supports_channels() {
        let descriptor = density_matrix_descriptor(
            Precision::ComplexF64,
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("density-matrix descriptor must be valid");

        assert!(
            descriptor.supports(
                RepresentationCapability::QuantumChannels
            )
        );
    }

    #[test]
    fn backend_native_does_not_fake_amplitudes() {
        let descriptor = backend_native_descriptor(
            StorageLocation::Remote,
            ExecutionMode::Qpu,
        )
        .expect("backend-native descriptor must be valid");

        assert_eq!(
            descriptor.representation(),
            StateRepresentation::BackendNative
        );

        assert!(
            !descriptor.supports(
                RepresentationCapability::AmplitudeAccess
            )
        );

        assert!(
            descriptor.supports(
                RepresentationCapability::BackendNativeExecution
            )
        );
    }

    #[test]
    fn device_storage_requires_device_capability() {
        let capabilities = CapabilitySet::EMPTY;

        let result = RepresentationDescriptor::new(
            StateRepresentation::StateVector,
            Precision::ComplexF64,
            StorageLocation::Device,
            ExecutionMode::Simulation,
            capabilities,
            ScalingClass::Exponential,
        );

        assert!(result.is_err());
    }

    #[test]
    fn distributed_storage_requires_distributed_capability() {
        let capabilities = CapabilitySet::EMPTY
            .with(RepresentationCapability::AmplitudeAccess)
            .with(RepresentationCapability::ProbabilityAccess)
            .with(RepresentationCapability::Normalization)
            .with(RepresentationCapability::DeviceExecution);

        let result = RepresentationDescriptor::new(
            StateRepresentation::StateVector,
            Precision::ComplexF64,
            StorageLocation::Distributed,
            ExecutionMode::Distributed,
            capabilities,
            ScalingClass::Exponential,
        );

        assert!(result.is_err());
    }

    #[test]
    fn automatic_selection_is_deterministic() {
        let state_vector = state_vector_descriptor(
            Precision::ComplexF64,
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("valid descriptor");

        let stabilizer = stabilizer_descriptor(
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("valid descriptor");

        let requirements = RepresentationRequirements::new();

        let context = RepresentationSelectionContext::new(
            requirements,
            RepresentationPolicy::PreferStabilizer,
            100,
        );

        let candidates = vec![
            state_vector,
            stabilizer,
        ];

        let first = select_representation(
            &context,
            &candidates,
        )
        .expect("selection must succeed");

        let second = select_representation(
            &context,
            &candidates,
        )
        .expect("selection must succeed");

        assert_eq!(first, second);

        assert_eq!(
            first.representation(),
            StateRepresentation::Stabilizer
        );
    }

    #[test]
    fn forced_selection_rejects_other_representations() {
        let state_vector = state_vector_descriptor(
            Precision::ComplexF64,
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("valid descriptor");

        let stabilizer = stabilizer_descriptor(
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("valid descriptor");

        let context = RepresentationSelectionContext::new(
            RepresentationRequirements::new(),
            RepresentationPolicy::Forced(
                StateRepresentation::Stabilizer,
            ),
            10,
        );

        let candidates = vec![
            state_vector,
            stabilizer,
        ];

        let selection = select_representation(
            &context,
            &candidates,
        )
        .expect("forced stabilizer selection must succeed");

        assert_eq!(
            selection.representation(),
            StateRepresentation::Stabilizer
        );
    }

    #[test]
    fn qpu_selection_prefers_backend_native() {
        let simulator = state_vector_descriptor(
            Precision::ComplexF64,
            StorageLocation::Host,
            ExecutionMode::Simulation,
        )
        .expect("valid descriptor");

        let qpu = backend_native_descriptor(
            StorageLocation::Remote,
            ExecutionMode::Qpu,
        )
        .expect("valid descriptor");

        let context = RepresentationSelectionContext::new(
            RepresentationRequirements::new(),
            RepresentationPolicy::BackendNative,
            100,
        )
        .with_qpu(true);

        let candidates = vec![
            simulator,
            qpu,
        ];

        let selection = select_representation(
            &context,
            &candidates,
        )
        .expect("QPU selection must succeed");

        assert_eq!(
            selection.representation(),
            StateRepresentation::BackendNative
        );
    }

    #[test]
    fn precision_ranks_are_stable() {
        assert!(
            Precision::ComplexF64.rank()
                > Precision::ComplexF32.rank()
        );

        assert!(
            Precision::Arbitrary.rank()
                > Precision::F64.rank()
        );
    }

    #[test]
    fn non_binary_representations_are_marked_correctly() {
        assert!(
            StateRepresentation::QuditStateVector
                .supports_non_binary_systems()
        );

        assert!(
            StateRepresentation::PhotonicFock
                .supports_non_binary_systems()
        );

        assert!(
            StateRepresentation::ContinuousVariable
                .supports_non_binary_systems()
        );
    }
}