//! Zamani Quantum IR — Capability Requirements
//!
//! Production-grade, hardware-independent capability requirements for the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::capability` describes what a quantum program REQUIRES from
//! its eventual execution target.
//!
//! It deliberately does NOT describe what a particular QPU, simulator,
//! emulator, provider, or backend actually supports.
//!
//! The architectural split is:
//!
//! ```text
//! quantum::ir::capability
//!     |
//!     |  "What does this program require?"
//!     v
//! quantum::hardware::capabilities
//!     |
//!     |  "What can this target do?"
//!     v
//! quantum::hardware::compatibility
//!     |
//!     |  "Can this target satisfy the program?"
//!     v
//! routing / scheduling / optimization / backend
//! ```
//!
//! This separation is essential for Zamani's universal quantum-program
//! principle:
//!
//! > A quantum program is written once and can be compiled toward any
//! > sufficiently capable target, from a tiny machine to a very large finite
//! > machine, subject only to explicit resource and capability constraints.
//!
//! # No architectural machine-size limit
//!
//! This module intentionally contains no fixed maximum such as:
//!
//! ```text
//! 63
//! 4096
//! 1_000_000
//! ```
//!
//! Those numbers must never become implicit quantum-machine limits.
//!
//! A capability requirement can describe:
//!
//! - one logical qubit;
//! - many logical qubits;
//! - an explicitly bounded finite number;
//! - an unknown/runtime-determined number;
//! - a minimum number of qubits;
//! - a requirement concerning all qubits in a program.
//!
//! Concrete resource limits are handled by `quantum::ir::limits` and concrete
//! target capacity is handled by the hardware subsystem.
//!
//! # Qubit identity boundary
//!
//! When a capability requirement needs to refer to particular program qubits,
//! this module uses the canonical types from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! QubitRef
//! ```
//!
//! This module does NOT perform logical-to-physical placement.
//!
//! Routing remains responsible for deciding where logical qubits execute.
//!
//! # Capability vs instruction
//!
//! A capability is not an instruction.
//!
//! For example:
//!
//! ```text
//! requirement:
//!     MidCircuitMeasurement
//!
//! instruction:
//!     provider-specific measurement operation
//! ```
//!
//! The IR requirement therefore remains stable even if different hardware
//! backends use different instruction sets.
//!
//! # Capability vs resource
//!
//! Capability and resource requirements are related but distinct.
//!
//! ```text
//! capability:
//!     PulseControl
//!
//! resource:
//!     at least 8 control channels
//!
//! capability:
//!     DynamicCircuits
//!
//! resource:
//!     at least 100 classical bits
//! ```
//!
//! Resource quantities belong conceptually to `quantum::ir::resource`.
//! This file provides a compact capability vocabulary and the capability
//! requirement container that can be attached to a program.
//!
//! # Capability states
//!
//! A program requirement is not "stable" or "experimental". Stability belongs
//! to the target's capability declaration.
//!
//! Therefore this module does NOT duplicate the hardware-side
//! `CapabilityStatus` model.
//!
//! A compiler may later apply a policy such as:
//!
//! ```text
//! stable target capabilities only
//! stable + experimental target capabilities
//! ```
//!
//! # Unknown / extension capabilities
//!
//! Quantum hardware evolves faster than language specifications.
//!
//! The IR therefore supports provider-neutral custom capability identifiers.
//!
//! Unknown capability identifiers must remain namespaced and must not silently
//! become equivalent to a known capability.
//!
//! # Serialization
//!
//! Serde is used because it is already a repository dependency.
//!
//! Stable string identifiers are used for persistent capability names.
//!
//! The exact Rust enum debug representation is NOT part of the serialized
//! contract.
//!
//! # Security
//!
//! This module contains no unsafe code.
//!
//! Capability requirements supplied by untrusted source must still be checked
//! against `QuantumIrLimits` and the downstream hardware capability model.
//!
//! No capability requirement grants permission to access hardware.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No unsafe code.
//!
//! # Integration contract
//!
//! This file is designed to be frozen before hardware compatibility is
//! implemented.
//!
//! Downstream consumers should use:
//!
//! ```text
//! CapabilityRequirementSet
//! CapabilityRequirement
//! QuantumCapability
//! CapabilityKind
//! CustomCapability
//! ```
//!
//! Hardware compatibility should consume this API without requiring changes
//! to the fundamental capability vocabulary.
//!
//! New hardware-specific capabilities should normally be represented through
//! `CustomCapability` until a stable language-level capability is formally
//! added.
//!
//! The IR must never import `quantum::hardware` from this module.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::qubit::{PhysicalQubitId, QubitId, QubitRef};

// =============================================================================
// Stable capability identifier
// =============================================================================

/// A stable, namespaced capability identifier.
///
/// Built-in capabilities use the `zamani.*` namespace.
///
/// Custom capabilities should use a reverse-domain or project namespace such
/// as:
///
/// ```text
/// acme.quantum.fast_feed_forward
/// provider.example.special_measurement
/// research.foo.logical_magic_state
/// ```
///
/// Capability identifiers are semantic identifiers, not display labels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Namespace used by Zamani-defined capabilities.
    pub const ZAMANI_NAMESPACE: &'static str = "zamani";

    /// Creates a capability identifier after validation.
    pub fn new<S>(value: S) -> Result<Self, CapabilityParseError>
    where
        S: Into<String>,
    {
        let value = value.into();
        validate_capability_identifier(&value)?;
        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this is a Zamani-defined capability.
    #[must_use]
    pub fn is_zamani(&self) -> bool {
        self.0
            .split('.')
            .next()
            .map(|part| part == Self::ZAMANI_NAMESPACE)
            .unwrap_or(false)
    }

    /// Converts the identifier into an owned `String`.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for CapabilityId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CapabilityId {
    type Err = CapabilityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl Serialize for CapabilityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Built-in capability vocabulary
// =============================================================================

/// Canonical Zamani capability vocabulary.
///
/// This describes semantic capabilities rather than hardware instructions.
///
/// New capabilities should be added here only when they represent stable
/// language-level semantics. Provider-specific or experimental capabilities
/// should use [`CustomCapability`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumCapability {
    // -------------------------------------------------------------------------
    // Measurement
    // -------------------------------------------------------------------------

    /// Terminal quantum measurement.
    Measurement,

    /// Measurement while the program continues executing.
    MidCircuitMeasurement,

    /// Measurement of multiple resources in parallel.
    ParallelMeasurement,

    /// User-selectable measurement basis.
    ConfigurableMeasurementBasis,

    /// Observable measurement.
    ObservableMeasurement,

    /// Expectation-value evaluation.
    ExpectationValues,

    /// Sampling support.
    Sampling,

    /// Probability-distribution output.
    ProbabilityDistributions,

    // -------------------------------------------------------------------------
    // Preparation / reset
    // -------------------------------------------------------------------------

    /// Explicit quantum-state preparation.
    StatePreparation,

    /// Arbitrary state preparation.
    ArbitraryStatePreparation,

    /// Reset support.
    Reset,

    /// Mid-circuit reset.
    MidCircuitReset,

    /// Reuse of quantum resources during one program.
    QubitReuse,

    /// Leakage detection.
    LeakageDetection,

    /// Leakage reduction.
    LeakageReduction,

    // -------------------------------------------------------------------------
    // Gate model
    // -------------------------------------------------------------------------

    /// Single-qubit operations.
    SingleQubitGates,

    /// Two-qubit operations.
    TwoQubitGates,

    /// Three-qubit operations.
    ThreeQubitGates,

    /// General multi-qubit operations.
    MultiQubitGates,

    /// Arbitrary single-qubit rotations.
    ArbitrarySingleQubitRotations,

    /// Parameterized gates.
    ParameterizedGates,

    /// Native gate execution.
    NativeGateExecution,

    /// Controlled operations.
    ControlledOperations,

    /// Adjoint/inverse operations.
    AdjointOperations,

    /// Non-unitary quantum operations.
    NonUnitaryOperations,

    // -------------------------------------------------------------------------
    // Classical control
    // -------------------------------------------------------------------------

    /// Classical control of quantum execution.
    ClassicalControl,

    /// Conditional quantum operations.
    ConditionalOperations,

    /// Dynamic circuits.
    DynamicCircuits,

    /// Measurement-driven classical feed-forward.
    ClassicalFeedForward,

    /// Low-latency measurement-to-control feedback.
    FastFeedForward,

    /// Runtime branching.
    RuntimeBranching,

    /// Runtime loops.
    RuntimeLoops,

    /// Runtime classical expressions.
    RuntimeClassicalExpressions,

    // -------------------------------------------------------------------------
    // Parameterized execution
    // -------------------------------------------------------------------------

    /// Parameterized workload execution.
    ParameterizedExecution,

    /// Runtime parameter binding.
    RuntimeParameterBinding,

    /// Multiple parameter bindings in one execution.
    ParameterBatchExecution,

    /// Parameter sweeps.
    ParameterSweeps,

    /// Variational execution.
    VariationalExecution,

    // -------------------------------------------------------------------------
    // Pulse/control
    // -------------------------------------------------------------------------

    /// Pulse-level control.
    PulseControl,

    /// User-defined pulse envelopes.
    CustomPulseEnvelopes,

    /// Sampled waveform execution.
    SampledWaveforms,

    /// Symbolic waveform generation.
    SymbolicWaveforms,

    /// Arbitrary waveform generation.
    ArbitraryWaveforms,

    /// Independent control-channel selection.
    ControlChannels,

    /// Drive channels.
    DriveChannels,

    /// Measurement channels.
    MeasurementChannels,

    /// Acquisition channels.
    AcquisitionChannels,

    /// Flux/control channels.
    FluxChannels,

    /// Optical/laser channels.
    OpticalChannels,

    /// Frame-based phase/frequency control.
    FrameControl,

    /// Frequency control.
    FrequencyControl,

    /// Phase control.
    PhaseControl,

    /// Amplitude control.
    AmplitudeControl,

    /// Fine-grained timing of pulses.
    PulseTiming,

    // -------------------------------------------------------------------------
    // Analog / Hamiltonian / annealing
    // -------------------------------------------------------------------------

    /// Analog quantum evolution.
    AnalogEvolution,

    /// Hamiltonian-programmable evolution.
    HamiltonianEvolution,

    /// Time-dependent Hamiltonians.
    TimeDependentHamiltonians,

    /// Quantum annealing.
    QuantumAnnealing,

    /// QUBO workloads.
    Qubo,

    /// Ising-model workloads.
    IsingModel,

    /// Adiabatic evolution.
    AdiabaticEvolution,

    // -------------------------------------------------------------------------
    // Photonic / bosonic / continuous variable
    // -------------------------------------------------------------------------

    /// Photonic computation.
    PhotonicComputation,

    /// Bosonic computation.
    BosonicComputation,

    /// Continuous-variable computation.
    ContinuousVariableComputation,

    /// Optical interferometry.
    OpticalInterferometry,

    /// Photon-number measurement.
    PhotonNumberMeasurement,

    /// Mode-based quantum computation.
    ModeBasedComputation,

    // -------------------------------------------------------------------------
    // Fault tolerance / logical quantum computing
    // -------------------------------------------------------------------------

    /// Logical-qubit execution.
    LogicalQubits,

    /// Fault-tolerant execution.
    FaultTolerantExecution,

    /// Error-corrected operations.
    ErrorCorrectedOperations,

    /// Syndrome extraction.
    SyndromeExtraction,

    /// Logical measurement.
    LogicalMeasurement,

    /// Logical reset.
    LogicalReset,

    /// Magic-state resources.
    MagicStateResources,

    /// Encoded operations.
    EncodedOperations,

    /// Lattice/code-based operations.
    QuantumErrorCorrectionCodes,

    // -------------------------------------------------------------------------
    // Error correction / mitigation
    // -------------------------------------------------------------------------

    /// Quantum error correction.
    ErrorCorrection,

    /// Error mitigation.
    ErrorMitigation,

    /// Error-suppression features.
    ErrorSuppression,

    /// Noise-aware execution.
    NoiseAwareExecution,

    // -------------------------------------------------------------------------
    // Parallelism / concurrency
    // -------------------------------------------------------------------------

    /// Parallel quantum operations.
    ParallelOperations,

    /// Independent execution regions.
    ConcurrentRegions,

    /// Synchronization between quantum regions.
    QuantumSynchronization,

    /// Barrier/synchronization semantics.
    Barriers,

    // -------------------------------------------------------------------------
    // Distributed quantum computation
    // -------------------------------------------------------------------------

    /// Distributed quantum execution.
    DistributedQuantum,

    /// Multiple quantum processing units.
    MultiQpuExecution,

    /// Inter-QPU communication.
    QuantumNetworking,

    /// Entanglement distribution.
    EntanglementDistribution,

    /// Remote quantum operations.
    RemoteQuantumOperations,

    /// Distributed measurement/control.
    DistributedControl,

    // -------------------------------------------------------------------------
    // Runtime / execution
    // -------------------------------------------------------------------------

    /// Runtime execution.
    RuntimeExecution,

    /// Repeated-shot execution.
    RepeatedExecution,

    /// Streaming result acquisition.
    StreamingResults,

    /// Mid-execution result access.
    RuntimeResultAccess,

    /// Deterministic execution mode.
    DeterministicExecution,

    /// Checkpoint/resume support.
    CheckpointResume,

    // -------------------------------------------------------------------------
    // Simulation
    // -------------------------------------------------------------------------

    /// State-vector simulation.
    StateVectorSimulation,

    /// Density-matrix simulation.
    DensityMatrixSimulation,

    /// Stabilizer simulation.
    StabilizerSimulation,

    /// Tensor-network simulation.
    TensorNetworkSimulation,

    /// Clifford simulation.
    CliffordSimulation,

    /// Noisy simulation.
    NoisySimulation,

    /// Shot-based simulation.
    ShotSimulation,

    // -------------------------------------------------------------------------
    // Resource / scaling
    // -------------------------------------------------------------------------

    /// Large-scale logical-resource execution.
    LargeScaleExecution,

    /// Distributed-resource execution.
    DistributedResources,

    /// Runtime allocation of quantum resources.
    DynamicResourceAllocation,

    /// Resource reuse.
    ResourceReuse,
}

impl QuantumCapability {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "zamani.measurement",
            Self::MidCircuitMeasurement => "zamani.mid_circuit_measurement",
            Self::ParallelMeasurement => "zamani.parallel_measurement",
            Self::ConfigurableMeasurementBasis => {
                "zamani.configurable_measurement_basis"
            }
            Self::ObservableMeasurement => "zamani.observable_measurement",
            Self::ExpectationValues => "zamani.expectation_values",
            Self::Sampling => "zamani.sampling",
            Self::ProbabilityDistributions => "zamani.probability_distributions",

            Self::StatePreparation => "zamani.state_preparation",
            Self::ArbitraryStatePreparation => {
                "zamani.arbitrary_state_preparation"
            }
            Self::Reset => "zamani.reset",
            Self::MidCircuitReset => "zamani.mid_circuit_reset",
            Self::QubitReuse => "zamani.qubit_reuse",
            Self::LeakageDetection => "zamani.leakage_detection",
            Self::LeakageReduction => "zamani.leakage_reduction",

            Self::SingleQubitGates => "zamani.single_qubit_gates",
            Self::TwoQubitGates => "zamani.two_qubit_gates",
            Self::ThreeQubitGates => "zamani.three_qubit_gates",
            Self::MultiQubitGates => "zamani.multi_qubit_gates",
            Self::ArbitrarySingleQubitRotations => {
                "zamani.arbitrary_single_qubit_rotations"
            }
            Self::ParameterizedGates => "zamani.parameterized_gates",
            Self::NativeGateExecution => "zamani.native_gate_execution",
            Self::ControlledOperations => "zamani.controlled_operations",
            Self::AdjointOperations => "zamani.adjoint_operations",
            Self::NonUnitaryOperations => "zamani.non_unitary_operations",

            Self::ClassicalControl => "zamani.classical_control",
            Self::ConditionalOperations => "zamani.conditional_operations",
            Self::DynamicCircuits => "zamani.dynamic_circuits",
            Self::ClassicalFeedForward => "zamani.classical_feed_forward",
            Self::FastFeedForward => "zamani.fast_feed_forward",
            Self::RuntimeBranching => "zamani.runtime_branching",
            Self::RuntimeLoops => "zamani.runtime_loops",
            Self::RuntimeClassicalExpressions => {
                "zamani.runtime_classical_expressions"
            }

            Self::ParameterizedExecution => "zamani.parameterized_execution",
            Self::RuntimeParameterBinding => "zamani.runtime_parameter_binding",
            Self::ParameterBatchExecution => "zamani.parameter_batch_execution",
            Self::ParameterSweeps => "zamani.parameter_sweeps",
            Self::VariationalExecution => "zamani.variational_execution",

            Self::PulseControl => "zamani.pulse_control",
            Self::CustomPulseEnvelopes => "zamani.custom_pulse_envelopes",
            Self::SampledWaveforms => "zamani.sampled_waveforms",
            Self::SymbolicWaveforms => "zamani.symbolic_waveforms",
            Self::ArbitraryWaveforms => "zamani.arbitrary_waveforms",
            Self::ControlChannels => "zamani.control_channels",
            Self::DriveChannels => "zamani.drive_channels",
            Self::MeasurementChannels => "zamani.measurement_channels",
            Self::AcquisitionChannels => "zamani.acquisition_channels",
            Self::FluxChannels => "zamani.flux_channels",
            Self::OpticalChannels => "zamani.optical_channels",
            Self::FrameControl => "zamani.frame_control",
            Self::FrequencyControl => "zamani.frequency_control",
            Self::PhaseControl => "zamani.phase_control",
            Self::AmplitudeControl => "zamani.amplitude_control",
            Self::PulseTiming => "zamani.pulse_timing",

            Self::AnalogEvolution => "zamani.analog_evolution",
            Self::HamiltonianEvolution => "zamani.hamiltonian_evolution",
            Self::TimeDependentHamiltonians => {
                "zamani.time_dependent_hamiltonians"
            }
            Self::QuantumAnnealing => "zamani.quantum_annealing",
            Self::Qubo => "zamani.qubo",
            Self::IsingModel => "zamani.ising_model",
            Self::AdiabaticEvolution => "zamani.adiabatic_evolution",

            Self::PhotonicComputation => "zamani.photonic_computation",
            Self::BosonicComputation => "zamani.bosonic_computation",
            Self::ContinuousVariableComputation => {
                "zamani.continuous_variable_computation"
            }
            Self::OpticalInterferometry => "zamani.optical_interferometry",
            Self::PhotonNumberMeasurement => "zamani.photon_number_measurement",
            Self::ModeBasedComputation => "zamani.mode_based_computation",

            Self::LogicalQubits => "zamani.logical_qubits",
            Self::FaultTolerantExecution => "zamani.fault_tolerant_execution",
            Self::ErrorCorrectedOperations => {
                "zamani.error_corrected_operations"
            }
            Self::SyndromeExtraction => "zamani.syndrome_extraction",
            Self::LogicalMeasurement => "zamani.logical_measurement",
            Self::LogicalReset => "zamani.logical_reset",
            Self::MagicStateResources => "zamani.magic_state_resources",
            Self::EncodedOperations => "zamani.encoded_operations",
            Self::QuantumErrorCorrectionCodes => {
                "zamani.quantum_error_correction_codes"
            }

            Self::ErrorCorrection => "zamani.error_correction",
            Self::ErrorMitigation => "zamani.error_mitigation",
            Self::ErrorSuppression => "zamani.error_suppression",
            Self::NoiseAwareExecution => "zamani.noise_aware_execution",

            Self::ParallelOperations => "zamani.parallel_operations",
            Self::ConcurrentRegions => "zamani.concurrent_regions",
            Self::QuantumSynchronization => "zamani.quantum_synchronization",
            Self::Barriers => "zamani.barriers",

            Self::DistributedQuantum => "zamani.distributed_quantum",
            Self::MultiQpuExecution => "zamani.multi_qpu_execution",
            Self::QuantumNetworking => "zamani.quantum_networking",
            Self::EntanglementDistribution => {
                "zamani.entanglement_distribution"
            }
            Self::RemoteQuantumOperations => {
                "zamani.remote_quantum_operations"
            }
            Self::DistributedControl => "zamani.distributed_control",

            Self::RuntimeExecution => "zamani.runtime_execution",
            Self::RepeatedExecution => "zamani.repeated_execution",
            Self::StreamingResults => "zamani.streaming_results",
            Self::RuntimeResultAccess => "zamani.runtime_result_access",
            Self::DeterministicExecution => "zamani.deterministic_execution",
            Self::CheckpointResume => "zamani.checkpoint_resume",

            Self::StateVectorSimulation => "zamani.state_vector_simulation",
            Self::DensityMatrixSimulation => "zamani.density_matrix_simulation",
            Self::StabilizerSimulation => "zamani.stabilizer_simulation",
            Self::TensorNetworkSimulation => "zamani.tensor_network_simulation",
            Self::CliffordSimulation => "zamani.clifford_simulation",
            Self::NoisySimulation => "zamani.noisy_simulation",
            Self::ShotSimulation => "zamani.shot_simulation",

            Self::LargeScaleExecution => "zamani.large_scale_execution",
            Self::DistributedResources => "zamani.distributed_resources",
            Self::DynamicResourceAllocation => {
                "zamani.dynamic_resource_allocation"
            }
            Self::ResourceReuse => "zamani.resource_reuse",
        }
    }

    /// Returns the stable [`CapabilityId`].
    #[must_use]
    pub fn id(self) -> CapabilityId {
        // Built-in identifiers are compile-time constants and therefore
        // guaranteed by this implementation to satisfy the identifier
        // validation rules.
        CapabilityId(self.as_str().to_owned())
    }

    /// Returns the semantic category of the capability.
    #[must_use]
    pub const fn category(self) -> CapabilityCategory {
        match self {
            Self::Measurement
            | Self::MidCircuitMeasurement
            | Self::ParallelMeasurement
            | Self::ConfigurableMeasurementBasis
            | Self::ObservableMeasurement
            | Self::ExpectationValues
            | Self::Sampling
            | Self::ProbabilityDistributions
            | Self::PhotonNumberMeasurement
            | Self::LogicalMeasurement => CapabilityCategory::Measurement,

            Self::StatePreparation
            | Self::ArbitraryStatePreparation
            | Self::Reset
            | Self::MidCircuitReset
            | Self::QubitReuse
            | Self::LeakageDetection
            | Self::LeakageReduction
            | Self::LogicalReset => CapabilityCategory::StatePreparation,

            Self::SingleQubitGates
            | Self::TwoQubitGates
            | Self::ThreeQubitGates
            | Self::MultiQubitGates
            | Self::ArbitrarySingleQubitRotations
            | Self::ParameterizedGates
            | Self::NativeGateExecution
            | Self::ControlledOperations
            | Self::AdjointOperations
            | Self::NonUnitaryOperations => CapabilityCategory::GateModel,

            Self::ClassicalControl
            | Self::ConditionalOperations
            | Self::DynamicCircuits
            | Self::ClassicalFeedForward
            | Self::FastFeedForward
            | Self::RuntimeBranching
            | Self::RuntimeLoops
            | Self::RuntimeClassicalExpressions => CapabilityCategory::ClassicalControl,

            Self::ParameterizedExecution
            | Self::RuntimeParameterBinding
            | Self::ParameterBatchExecution
            | Self::ParameterSweeps
            | Self::VariationalExecution => CapabilityCategory::Parameterization,

            Self::PulseControl
            | Self::CustomPulseEnvelopes
            | Self::SampledWaveforms
            | Self::SymbolicWaveforms
            | Self::ArbitraryWaveforms
            | Self::ControlChannels
            | Self::DriveChannels
            | Self::MeasurementChannels
            | Self::AcquisitionChannels
            | Self::FluxChannels
            | Self::OpticalChannels
            | Self::FrameControl
            | Self::FrequencyControl
            | Self::PhaseControl
            | Self::AmplitudeControl
            | Self::PulseTiming => CapabilityCategory::PulseControl,

            Self::AnalogEvolution
            | Self::HamiltonianEvolution
            | Self::TimeDependentHamiltonians
            | Self::AdiabaticEvolution => CapabilityCategory::Analog,

            Self::QuantumAnnealing | Self::Qubo | Self::IsingModel => {
                CapabilityCategory::Annealing
            }

            Self::PhotonicComputation
            | Self::BosonicComputation
            | Self::ContinuousVariableComputation
            | Self::OpticalInterferometry
            | Self::PhotonNumberMeasurement
            | Self::ModeBasedComputation => CapabilityCategory::Photonic,

            Self::LogicalQubits
            | Self::FaultTolerantExecution
            | Self::ErrorCorrectedOperations
            | Self::SyndromeExtraction
            | Self::LogicalMeasurement
            | Self::LogicalReset
            | Self::MagicStateResources
            | Self::EncodedOperations
            | Self::QuantumErrorCorrectionCodes => CapabilityCategory::FaultTolerance,

            Self::ErrorCorrection
            | Self::ErrorMitigation
            | Self::ErrorSuppression
            | Self::NoiseAwareExecution => CapabilityCategory::ErrorCorrection,

            Self::ParallelOperations
            | Self::ConcurrentRegions
            | Self::QuantumSynchronization
            | Self::Barriers => CapabilityCategory::Concurrency,

            Self::DistributedQuantum
            | Self::MultiQpuExecution
            | Self::QuantumNetworking
            | Self::EntanglementDistribution
            | Self::RemoteQuantumOperations
            | Self::DistributedControl => CapabilityCategory::Distributed,

            Self::RuntimeExecution
            | Self::RepeatedExecution
            | Self::StreamingResults
            | Self::RuntimeResultAccess
            | Self::DeterministicExecution
            | Self::CheckpointResume => CapabilityCategory::Execution,

            Self::StateVectorSimulation
            | Self::DensityMatrixSimulation
            | Self::StabilizerSimulation
            | Self::TensorNetworkSimulation
            | Self::CliffordSimulation
            | Self::NoisySimulation
            | Self::ShotSimulation => CapabilityCategory::Simulation,

            Self::LargeScaleExecution
            | Self::DistributedResources
            | Self::DynamicResourceAllocation
            | Self::ResourceReuse => CapabilityCategory::Execution,
        }
    }
}

impl fmt::Display for QuantumCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<QuantumCapability> for CapabilityId {
    fn from(capability: QuantumCapability) -> Self {
        capability.id()
    }
}

impl Serialize for QuantumCapability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for QuantumCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        QuantumCapability::from_str(&value).map_err(serde::de::Error::custom)
    }
}

impl FromStr for QuantumCapability {
    type Err = CapabilityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        for capability in QuantumCapability::all() {
            if capability.as_str() == value.trim() {
                return Ok(*capability);
            }
        }

        Err(CapabilityParseError::UnknownCapability {
            value: value.trim().to_owned(),
        })
    }
}

impl QuantumCapability {
    /// Returns all built-in capabilities.
    ///
    /// The returned slice is static and does not allocate.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Measurement,
            Self::MidCircuitMeasurement,
            Self::ParallelMeasurement,
            Self::ConfigurableMeasurementBasis,
            Self::ObservableMeasurement,
            Self::ExpectationValues,
            Self::Sampling,
            Self::ProbabilityDistributions,
            Self::StatePreparation,
            Self::ArbitraryStatePreparation,
            Self::Reset,
            Self::MidCircuitReset,
            Self::QubitReuse,
            Self::LeakageDetection,
            Self::LeakageReduction,
            Self::SingleQubitGates,
            Self::TwoQubitGates,
            Self::ThreeQubitGates,
            Self::MultiQubitGates,
            Self::ArbitrarySingleQubitRotations,
            Self::ParameterizedGates,
            Self::NativeGateExecution,
            Self::ControlledOperations,
            Self::AdjointOperations,
            Self::NonUnitaryOperations,
            Self::ClassicalControl,
            Self::ConditionalOperations,
            Self::DynamicCircuits,
            Self::ClassicalFeedForward,
            Self::FastFeedForward,
            Self::RuntimeBranching,
            Self::RuntimeLoops,
            Self::RuntimeClassicalExpressions,
            Self::ParameterizedExecution,
            Self::RuntimeParameterBinding,
            Self::ParameterBatchExecution,
            Self::ParameterSweeps,
            Self::VariationalExecution,
            Self::PulseControl,
            Self::CustomPulseEnvelopes,
            Self::SampledWaveforms,
            Self::SymbolicWaveforms,
            Self::ArbitraryWaveforms,
            Self::ControlChannels,
            Self::DriveChannels,
            Self::MeasurementChannels,
            Self::AcquisitionChannels,
            Self::FluxChannels,
            Self::OpticalChannels,
            Self::FrameControl,
            Self::FrequencyControl,
            Self::PhaseControl,
            Self::AmplitudeControl,
            Self::PulseTiming,
            Self::AnalogEvolution,
            Self::HamiltonianEvolution,
            Self::TimeDependentHamiltonians,
            Self::QuantumAnnealing,
            Self::Qubo,
            Self::IsingModel,
            Self::AdiabaticEvolution,
            Self::PhotonicComputation,
            Self::BosonicComputation,
            Self::ContinuousVariableComputation,
            Self::OpticalInterferometry,
            Self::PhotonNumberMeasurement,
            Self::ModeBasedComputation,
            Self::LogicalQubits,
            Self::FaultTolerantExecution,
            Self::ErrorCorrectedOperations,
            Self::SyndromeExtraction,
            Self::LogicalMeasurement,
            Self::LogicalReset,
            Self::MagicStateResources,
            Self::EncodedOperations,
            Self::QuantumErrorCorrectionCodes,
            Self::ErrorCorrection,
            Self::ErrorMitigation,
            Self::ErrorSuppression,
            Self::NoiseAwareExecution,
            Self::ParallelOperations,
            Self::ConcurrentRegions,
            Self::QuantumSynchronization,
            Self::Barriers,
            Self::DistributedQuantum,
            Self::MultiQpuExecution,
            Self::QuantumNetworking,
            Self::EntanglementDistribution,
            Self::RemoteQuantumOperations,
            Self::DistributedControl,
            Self::RuntimeExecution,
            Self::RepeatedExecution,
            Self::StreamingResults,
            Self::RuntimeResultAccess,
            Self::DeterministicExecution,
            Self::CheckpointResume,
            Self::StateVectorSimulation,
            Self::DensityMatrixSimulation,
            Self::StabilizerSimulation,
            Self::TensorNetworkSimulation,
            Self::CliffordSimulation,
            Self::NoisySimulation,
            Self::ShotSimulation,
            Self::LargeScaleExecution,
            Self::DistributedResources,
            Self::DynamicResourceAllocation,
            Self::ResourceReuse,
        ]
    }
}

// =============================================================================
// Capability category
// =============================================================================

/// High-level semantic category of a capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityCategory {
    Measurement,
    StatePreparation,
    GateModel,
    ClassicalControl,
    Parameterization,
    PulseControl,
    Analog,
    Annealing,
    Photonic,
    FaultTolerance,
    ErrorCorrection,
    Concurrency,
    Execution,
    Distributed,
    Simulation,
    Custom,
}

impl CapabilityCategory {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::StatePreparation => "state_preparation",
            Self::GateModel => "gate_model",
            Self::ClassicalControl => "classical_control",
            Self::Parameterization => "parameterization",
            Self::PulseControl => "pulse_control",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Photonic => "photonic",
            Self::FaultTolerance => "fault_tolerance",
            Self::ErrorCorrection => "error_correction",
            Self::Concurrency => "concurrency",
            Self::Execution => "execution",
            Self::Distributed => "distributed",
            Self::Simulation => "simulation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for CapabilityCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Custom capability
// =============================================================================

/// Provider/project-defined capability.
///
/// Custom capabilities are intentionally not interpreted by the core IR.
///
/// This makes the IR extensible without requiring every future hardware
/// feature to become a breaking change in the language core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomCapability {
    id: CapabilityId,
    category: String,
}

impl CustomCapability {
    /// Creates a custom capability.
    ///
    /// The identifier must be namespaced.
    pub fn new<I, C>(
        id: I,
        category: C,
    ) -> Result<Self, CapabilityParseError>
    where
        I: Into<String>,
        C: Into<String>,
    {
        let id = CapabilityId::new(id.into())?;
        let category = category.into();

        validate_namespace_component(&category)?;

        Ok(Self { id, category })
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the category namespace.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }
}

// =============================================================================
// Capability kind
// =============================================================================

/// Either a Zamani-defined capability or an extension capability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityKind {
    /// Stable Zamani capability.
    BuiltIn(QuantumCapability),

    /// Extension/provider/project capability.
    Custom(CustomCapability),
}

impl CapabilityKind {
    /// Returns the stable capability identifier.
    #[must_use]
    pub fn id(&self) -> CapabilityId {
        match self {
            Self::BuiltIn(capability) => capability.id(),
            Self::Custom(capability) => capability.id().clone(),
        }
    }

    /// Returns the capability category.
    #[must_use]
    pub fn category(&self) -> CapabilityCategory {
        match self {
            Self::BuiltIn(capability) => capability.category(),
            Self::Custom(_) => CapabilityCategory::Custom,
        }
    }

    /// Returns the stable identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::BuiltIn(capability) => capability.as_str(),
            Self::Custom(capability) => capability.id().as_str(),
        }
    }

    /// Returns whether this is a built-in capability.
    #[must_use]
    pub fn is_builtin(&self) -> bool {
        matches!(self, Self::BuiltIn(_))
    }

    /// Returns whether this is an extension capability.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl From<QuantumCapability> for CapabilityKind {
    fn from(value: QuantumCapability) -> Self {
        Self::BuiltIn(value)
    }
}

impl From<CustomCapability> for CapabilityKind {
    fn from(value: CustomCapability) -> Self {
        Self::Custom(value)
    }
}

impl fmt::Display for CapabilityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Capability scope
// =============================================================================

/// Scope at which a capability is required.
///
/// Scope is important for large and distributed systems.
///
/// A capability can be required:
///
/// - globally;
/// - for one logical qubit;
/// - for one physical qubit;
/// - for a specific group of program qubits;
/// - for a specific program operation.
///
/// The IR does not decide how hardware satisfies the requirement.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityScope {
    /// Capability is required for the execution target as a whole.
    Global,

    /// Capability is required for one logical qubit.
    LogicalQubit(QubitId),

    /// Capability is required for one physical target reference.
    ///
    /// This is normally used only after a placement/mapping stage.
    PhysicalQubit(PhysicalQubitId),

    /// Capability is required for a specific logical/physical reference.
    Qubit(QubitRef),

    /// Capability applies to a group of logical qubits.
    LogicalQubitSet(BTreeSet<QubitId>),

    /// Capability applies to a specific operation identity.
    ///
    /// The ID is intentionally kept opaque here so this file does not need to
    /// depend on `operation.rs`.
    Operation(u64),

    /// Capability applies to a named program region or semantic scope.
    ///
    /// This is a stable integration escape hatch before region identifiers are
    /// centralized in `identity.rs`.
    Region(u64),
}

impl CapabilityScope {
    /// Global scope.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Logical-qubit scope.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Physical-qubit scope.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Returns whether this scope is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the logical qubit when directly scoped to one.
    #[must_use]
    pub const fn logical_qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            Self::Qubit(QubitRef::Logical(id)) => Some(*id),
            _ => None,
        }
    }
}

// =============================================================================
// Requirement strength
// =============================================================================

/// Strength of a capability requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequirementStrength {
    /// Capability is mandatory.
    Required,

    /// Capability is useful but the compiler may legally transform the
    /// program into an implementation that does not need it.
    Preferred,

    /// Capability is an optimization opportunity only.
    Optional,
}

impl Default for RequirementStrength {
    fn default() -> Self {
        Self::Required
    }
}

impl RequirementStrength {
    /// Returns whether the capability is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns whether the capability is merely preferred.
    #[must_use]
    pub const fn is_preferred(self) -> bool {
        matches!(self, Self::Preferred)
    }

    /// Returns whether the capability is optional.
    #[must_use]
    pub const fn is_optional(self) -> bool {
        matches!(self, Self::Optional)
    }
}

// =============================================================================
// Requirement mode
// =============================================================================

/// How a capability is required relative to a target or region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityRequirementMode {
    /// At least one target/resource must provide the capability.
    Any,

    /// Every relevant target/resource must provide the capability.
    All,

    /// A specific number of independent resources must provide it.
    Count,

    /// The capability must be available for the complete program.
    Program,

    /// The capability is required only when execution reaches a dynamic path.
    Runtime,
}

impl Default for CapabilityRequirementMode {
    fn default() -> Self {
        Self::Program
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// One semantic capability requirement.
///
/// This is the atomic unit stored by [`CapabilityRequirementSet`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityRequirement {
    capability: CapabilityKind,
    scope: CapabilityScope,
    strength: RequirementStrength,
    mode: CapabilityRequirementMode,
    minimum_count: Option<usize>,
}

impl CapabilityRequirement {
    /// Creates a mandatory program-wide requirement.
    #[must_use]
    pub fn required(capability: QuantumCapability) -> Self {
        Self {
            capability: CapabilityKind::BuiltIn(capability),
            scope: CapabilityScope::Global,
            strength: RequirementStrength::Required,
            mode: CapabilityRequirementMode::Program,
            minimum_count: None,
        }
    }

    /// Creates a requirement with explicit scope and policy.
    pub fn new(
        capability: CapabilityKind,
        scope: CapabilityScope,
        strength: RequirementStrength,
        mode: CapabilityRequirementMode,
        minimum_count: Option<usize>,
    ) -> Result<Self, CapabilityRequirementError> {
        if matches!(mode, CapabilityRequirementMode::Count)
            && minimum_count.unwrap_or(0) == 0
        {
            return Err(CapabilityRequirementError::InvalidMinimumCount);
        }

        if !matches!(mode, CapabilityRequirementMode::Count)
            && minimum_count.is_some()
        {
            return Err(
                CapabilityRequirementError::CountOnlyModeRequiresCount,
            );
        }

        Ok(Self {
            capability,
            scope,
            strength,
            mode,
            minimum_count,
        })
    }

    /// Returns the capability.
    #[must_use]
    pub fn capability(&self) -> &CapabilityKind {
        &self.capability
    }

    /// Returns the stable capability identifier.
    #[must_use]
    pub fn capability_id(&self) -> CapabilityId {
        self.capability.id()
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &CapabilityScope {
        &self.scope
    }

    /// Returns the requirement strength.
    #[must_use]
    pub const fn strength(&self) -> RequirementStrength {
        self.strength
    }

    /// Returns the requirement mode.
    #[must_use]
    pub const fn mode(&self) -> CapabilityRequirementMode {
        self.mode
    }

    /// Returns the required minimum count.
    #[must_use]
    pub const fn minimum_count(&self) -> Option<usize> {
        self.minimum_count
    }

    /// Returns whether this is a mandatory requirement.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.strength.is_required()
    }

    /// Returns whether this requirement is global.
    #[must_use]
    pub fn is_global(&self) -> bool {
        self.scope.is_global()
    }

    /// Returns whether this requirement is scoped to a logical qubit.
    #[must_use]
    pub fn is_logical_qubit_scoped(&self) -> bool {
        matches!(
            self.scope,
            CapabilityScope::LogicalQubit(_)
                | CapabilityScope::Qubit(QubitRef::Logical(_))
        )
    }

    /// Creates a required logical-qubit capability.
    #[must_use]
    pub fn required_for_qubit(
        capability: QuantumCapability,
        qubit: QubitId,
    ) -> Self {
        Self {
            capability: CapabilityKind::BuiltIn(capability),
            scope: CapabilityScope::LogicalQubit(qubit),
            strength: RequirementStrength::Required,
            mode: CapabilityRequirementMode::Program,
            minimum_count: None,
        }
    }

    /// Creates a capability-count requirement.
    pub fn required_count(
        capability: QuantumCapability,
        count: usize,
    ) -> Result<Self, CapabilityRequirementError> {
        if count == 0 {
            return Err(CapabilityRequirementError::InvalidMinimumCount);
        }

        Ok(Self {
            capability: CapabilityKind::BuiltIn(capability),
            scope: CapabilityScope::Global,
            strength: RequirementStrength::Required,
            mode: CapabilityRequirementMode::Count,
            minimum_count: Some(count),
        })
    }
}

// =============================================================================
// Requirement set
// =============================================================================

/// Complete capability contract for a Zamani quantum program.
///
/// The set is deterministic:
///
/// - capabilities are indexed by stable IDs;
/// - duplicate requirements are merged;
/// - iteration is lexicographically deterministic.
///
/// This makes the type suitable for:
///
/// - compilation;
/// - compatibility checking;
/// - caching;
/// - provenance;
/// - serialization;
/// - deterministic builds.
///
/// It intentionally does not know anything about a hardware provider.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilityRequirementSet {
    requirements: BTreeMap<CapabilityId, Vec<CapabilityRequirement>>,
}

impl CapabilityRequirementSet {
    /// Creates an empty capability requirement set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a requirement.
    ///
    /// Requirements are stored deterministically by capability ID.
    pub fn require(
        &mut self,
        requirement: CapabilityRequirement,
    ) -> Result<(), CapabilityRequirementError> {
        let id = requirement.capability_id();

        let entries = self.requirements.entry(id).or_default();

        if entries.iter().any(|existing| existing == &requirement) {
            return Ok(());
        }

        entries.push(requirement);
        entries.sort_by(|left, right| {
            capability_requirement_sort_key(left)
                .cmp(&capability_requirement_sort_key(right))
        });

        Ok(())
    }

    /// Adds a mandatory built-in capability.
    pub fn require_capability(
        &mut self,
        capability: QuantumCapability,
    ) -> Result<(), CapabilityRequirementError> {
        self.require(CapabilityRequirement::required(capability))
    }

    /// Adds a mandatory capability for one logical qubit.
    pub fn require_for_qubit(
        &mut self,
        capability: QuantumCapability,
        qubit: QubitId,
    ) -> Result<(), CapabilityRequirementError> {
        self.require(CapabilityRequirement::required_for_qubit(
            capability,
            qubit,
        ))
    }

    /// Adds a capability count requirement.
    pub fn require_count(
        &mut self,
        capability: QuantumCapability,
        count: usize,
    ) -> Result<(), CapabilityRequirementError> {
        self.require(CapabilityRequirement::required_count(
            capability,
            count,
        )?)
    }

    /// Returns whether no requirements exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns the number of distinct capability identifiers.
    #[must_use]
    pub fn capability_count(&self) -> usize {
        self.requirements.len()
    }

    /// Returns the total number of requirement records.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.requirements.values().map(Vec::len).sum()
    }

    /// Returns all requirements in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityRequirement> {
        self.requirements.values().flat_map(|items| items.iter())
    }

    /// Returns all requirements for a specific capability.
    pub fn get(
        &self,
        capability: &CapabilityId,
    ) -> Option<&[CapabilityRequirement]> {
        self.requirements.get(capability).map(Vec::as_slice)
    }

    /// Returns whether the set contains a requirement for the capability.
    #[must_use]
    pub fn contains(&self, capability: QuantumCapability) -> bool {
        self.requirements.contains_key(&capability.id())
    }

    /// Returns all capability identifiers.
    pub fn capability_ids(
        &self,
    ) -> impl Iterator<Item = &CapabilityId> {
        self.requirements.keys()
    }

    /// Returns all mandatory requirements.
    pub fn required(
        &self,
    ) -> impl Iterator<Item = &CapabilityRequirement> {
        self.iter().filter(|requirement| requirement.is_required())
    }

    /// Returns whether all mandatory requirements are present in another
    /// capability-ID set.
    ///
    /// This method deliberately performs only identifier-level checking.
    /// Hardware-specific quantity, topology, timing, calibration, and
    /// instruction checks belong to `quantum::hardware::compatibility`.
    #[must_use]
    pub fn required_ids_satisfied<'a, I>(&self, available: I) -> bool
    where
        I: IntoIterator<Item = &'a CapabilityId>,
    {
        let available: BTreeSet<CapabilityId> =
            available.into_iter().cloned().collect();

        self.required()
            .all(|requirement| available.contains(&requirement.capability_id()))
    }

    /// Returns the mandatory capability IDs that are absent.
    #[must_use]
    pub fn missing_required_ids<'a, I>(
        &self,
        available: I,
    ) -> Vec<CapabilityId>
    where
        I: IntoIterator<Item = &'a CapabilityId>,
    {
        let available: BTreeSet<CapabilityId> =
            available.into_iter().cloned().collect();

        let mut missing = BTreeSet::new();

        for requirement in self.required() {
            let id = requirement.capability_id();

            if !available.contains(&id) {
                missing.insert(id);
            }
        }

        missing.into_iter().collect()
    }

    /// Validates internal invariants.
    pub fn validate(&self) -> Result<(), CapabilityRequirementError> {
        for (id, requirements) in &self.requirements {
            if requirements.is_empty() {
                return Err(
                    CapabilityRequirementError::EmptyCapabilityBucket {
                        capability: id.clone(),
                    },
                );
            }

            for requirement in requirements {
                if &requirement.capability_id() != id {
                    return Err(
                        CapabilityRequirementError::CapabilityIndexMismatch {
                            expected: id.clone(),
                            actual: requirement.capability_id(),
                        },
                    );
                }

                if matches!(
                    requirement.mode(),
                    CapabilityRequirementMode::Count
                ) && requirement.minimum_count().unwrap_or(0) == 0
                {
                    return Err(
                        CapabilityRequirementError::InvalidMinimumCount,
                    );
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Capability requirement report
// =============================================================================

/// Result of identifier-level capability checking.
///
/// This is deliberately provider-neutral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityCheckReport {
    /// Required capability identifiers that are present.
    satisfied: Vec<CapabilityId>,

    /// Required capability identifiers that are absent.
    missing: Vec<CapabilityId>,
}

impl CapabilityCheckReport {
    /// Creates a report.
    #[must_use]
    pub fn new(
        satisfied: Vec<CapabilityId>,
        missing: Vec<CapabilityId>,
    ) -> Self {
        Self {
            satisfied,
            missing,
        }
    }

    /// Returns satisfied requirements.
    #[must_use]
    pub fn satisfied(&self) -> &[CapabilityId] {
        &self.satisfied
    }

    /// Returns missing requirements.
    #[must_use]
    pub fn missing(&self) -> &[CapabilityId] {
        &self.missing
    }

    /// Returns whether every mandatory capability was satisfied.
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.missing.is_empty()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors related to capability identifiers and requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityParseError {
    /// Identifier is empty.
    EmptyIdentifier,

    /// Identifier contains invalid characters.
    InvalidIdentifier {
        /// The invalid identifier.
        value: String,
    },

    /// Identifier contains an empty namespace component.
    EmptyNamespaceComponent {
        /// Original identifier.
        value: String,
    },

    /// Capability is unknown.
    UnknownCapability {
        /// Unknown identifier.
        value: String,
    },

    /// Capability category is unknown.
    UnknownCategory {
        /// Unknown category.
        value: String,
    },
    
    /// Capability status is unknown.
    UnknownStatus {
        /// Unknown status.
        value: String,
    },
}

impl fmt::Display for CapabilityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("capability identifier cannot be empty")
            }
            Self::InvalidIdentifier { value } => {
                write!(formatter, "invalid capability identifier `{value}`")
            }
            Self::EmptyNamespaceComponent { value } => {
                write!(
                    formatter,
                    "capability identifier contains an empty namespace \
                     component: `{value}`"
                )
            }
            Self::UnknownCapability { value } => {
                write!(formatter, "unknown built-in capability `{value}`")
            }
            Self::UnknownCategory { value } => {
                write!(formatter, "unknown capability category `{value}`")
            }
            Self::UnknownStatus { value } => {
                write!(formatter, "unknown capability status `{value}`")
            }
        }
    }
}

impl Error for CapabilityParseError {}

/// Errors produced while constructing capability requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityRequirementError {
    /// A count requirement contained zero.
    InvalidMinimumCount,

    /// A minimum count was supplied to a non-count mode.
    CountOnlyModeRequiresCount,

    /// A capability bucket contains no requirements.
    EmptyCapabilityBucket {
        /// Capability ID.
        capability: CapabilityId,
    },

    /// Internal index mismatch.
    CapabilityIndexMismatch {
        /// Expected index key.
        expected: CapabilityId,

        /// Actual requirement capability.
        actual: CapabilityId,
    },
}

impl fmt::Display for CapabilityRequirementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMinimumCount => {
                formatter.write_str(
                    "capability minimum count must be greater than zero",
                )
            }
            Self::CountOnlyModeRequiresCount => {
                formatter.write_str(
                    "minimum count is only valid with Count requirement mode",
                )
            }
            Self::EmptyCapabilityBucket { capability } => {
                write!(
                    formatter,
                    "capability requirement bucket `{capability}` is empty"
                )
            }
            Self::CapabilityIndexMismatch { expected, actual } => {
                write!(
                    formatter,
                    "capability requirement index mismatch: expected \
                     `{expected}`, found `{actual}`"
                )
            }
        }
    }
}

impl Error for CapabilityRequirementError {}

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_capability_identifier(
    value: &str,
) -> Result<(), CapabilityParseError> {
    if value.trim().is_empty() {
        return Err(CapabilityParseError::EmptyIdentifier);
    }

    if value != value.trim() {
        return Err(CapabilityParseError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    let mut components = value.split('.');

    if components.next().is_none() {
        return Err(CapabilityParseError::EmptyIdentifier);
    }

    for component in value.split('.') {
        if component.is_empty() {
            return Err(CapabilityParseError::EmptyNamespaceComponent {
                value: value.to_owned(),
            });
        }

        validate_namespace_component(component)?;
    }

    Ok(())
}

fn validate_namespace_component(
    value: &str,
) -> Result<(), CapabilityParseError> {
    if value.is_empty() {
        return Err(CapabilityParseError::EmptyNamespaceComponent {
            value: value.to_owned(),
        });
    }

    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return Err(CapabilityParseError::EmptyNamespaceComponent {
            value: value.to_owned(),
        });
    };

    if !(first.is_ascii_alphanumeric() || first == '_' || first == '-') {
        return Err(CapabilityParseError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    for character in characters {
        if !(character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-')
        {
            return Err(CapabilityParseError::InvalidIdentifier {
                value: value.to_owned(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Deterministic sorting
// =============================================================================

fn capability_requirement_sort_key(
    requirement: &CapabilityRequirement,
) -> (u8, u8, String) {
    let strength = match requirement.strength() {
        RequirementStrength::Required => 0,
        RequirementStrength::Preferred => 1,
        RequirementStrength::Optional => 2,
    };

    let mode = match requirement.mode() {
        CapabilityRequirementMode::Program => 0,
        CapabilityRequirementMode::All => 1,
        CapabilityRequirementMode::Any => 2,
        CapabilityRequirementMode::Count => 3,
        CapabilityRequirementMode::Runtime => 4,
    };

    (strength, mode, scope_sort_string(requirement.scope()))
}

fn scope_sort_string(scope: &CapabilityScope) -> String {
    match scope {
        CapabilityScope::Global => "global".to_owned(),

        CapabilityScope::LogicalQubit(qubit) => {
            format!("logical:{:?}", qubit.index())
        }

        CapabilityScope::PhysicalQubit(qubit) => {
            format!("physical:{:?}", qubit.index())
        }

        CapabilityScope::Qubit(qubit) => {
            format!("qubit:{qubit}")
        }

        CapabilityScope::LogicalQubitSet(qubits) => {
            let mut result = String::from("logical_set:");

            for qubit in qubits {
                result.push_str(&qubit.index().to_string());
                result.push(',');
            }

            result
        }

        CapabilityScope::Operation(operation) => {
            format!("operation:{operation}")
        }

        CapabilityScope::Region(region) => {
            format!("region:{region}")
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_ids_are_stable() {
        assert_eq!(
            QuantumCapability::PulseControl.as_str(),
            "zamani.pulse_control"
        );

        assert_eq!(
            QuantumCapability::MidCircuitMeasurement.as_str(),
            "zamani.mid_circuit_measurement"
        );
    }

    #[test]
    fn builtin_capability_round_trips() {
        for capability in QuantumCapability::all() {
            let parsed =
                QuantumCapability::from_str(capability.as_str()).unwrap();

            assert_eq!(*capability, parsed);
        }
    }

    #[test]
    fn capability_identifier_requires_valid_namespace() {
        assert!(CapabilityId::new("").is_err());
        assert!(CapabilityId::new("zamani..pulse").is_err());
        assert!(CapabilityId::new("zamani pulse").is_err());
        assert!(CapabilityId::new("zamani.pulse_control").is_ok());
        assert!(CapabilityId::new("acme.quantum.feature").is_ok());
    }

    #[test]
    fn custom_capability_is_provider_neutral() {
        let capability = CustomCapability::new(
            "acme.quantum.fast_feedback",
            "feedback",
        )
        .unwrap();

        assert_eq!(
            capability.id().as_str(),
            "acme.quantum.fast_feedback"
        );
        assert_eq!(capability.category(), "feedback");
    }

    #[test]
    fn required_capability_is_constructed() {
        let requirement =
            CapabilityRequirement::required(QuantumCapability::PulseControl);

        assert!(requirement.is_required());
        assert!(requirement.is_global());
        assert_eq!(
            requirement.capability_id().as_str(),
            "zamani.pulse_control"
        );
    }

    #[test]
    fn qubit_scoped_requirement_uses_canonical_ir_qubit() {
        let qubit = QubitId::new(1);

        let requirement = CapabilityRequirement::required_for_qubit(
            QuantumCapability::PulseControl,
            qubit,
        );

        assert!(requirement.is_logical_qubit_scoped());
        assert_eq!(
            requirement.scope().logical_qubit_id(),
            Some(qubit)
        );
    }

    #[test]
    fn requirement_count_rejects_zero() {
        let result =
            CapabilityRequirement::required_count(
                QuantumCapability::TwoQubitGates,
                0,
            );

        assert!(matches!(
            result,
            Err(CapabilityRequirementError::InvalidMinimumCount)
        ));
    }

    #[test]
    fn requirement_count_accepts_large_finite_values() {
        let result =
            CapabilityRequirement::required_count(
                QuantumCapability::TwoQubitGates,
                usize::MAX,
            );

        assert!(result.is_ok());
    }

    #[test]
    fn requirement_set_deduplicates() {
        let mut set = CapabilityRequirementSet::new();

        set.require_capability(QuantumCapability::PulseControl)
            .unwrap();

        set.require_capability(QuantumCapability::PulseControl)
            .unwrap();

        assert_eq!(set.capability_count(), 1);
        assert_eq!(set.requirement_count(), 1);
    }

    #[test]
    fn requirement_set_is_deterministic() {
        let mut set = CapabilityRequirementSet::new();

        set.require_capability(QuantumCapability::TwoQubitGates)
            .unwrap();

        set.require_capability(QuantumCapability::PulseControl)
            .unwrap();

        set.require_capability(QuantumCapability::Measurement)
            .unwrap();

        let ids: Vec<&str> =
            set.capability_ids().map(CapabilityId::as_str).collect();

        assert_eq!(
            ids,
            vec![
                "zamani.measurement",
                "zamani.pulse_control",
                "zamani.two_qubit_gates"
            ]
        );
    }

    #[test]
    fn required_ids_are_checked_without_hardware_dependency() {
        let mut requirements = CapabilityRequirementSet::new();

        requirements
            .require_capability(QuantumCapability::PulseControl)
            .unwrap();

        requirements
            .require_capability(QuantumCapability::MidCircuitMeasurement)
            .unwrap();

        let available = vec![
            QuantumCapability::PulseControl.id(),
            QuantumCapability::MidCircuitMeasurement.id(),
        ];

        assert!(requirements.required_ids_satisfied(&available));
    }

    #[test]
    fn missing_ids_are_reported_deterministically() {
        let mut requirements = CapabilityRequirementSet::new();

        requirements
            .require_capability(QuantumCapability::PulseControl)
            .unwrap();

        requirements
            .require_capability(QuantumCapability::DynamicCircuits)
            .unwrap();

        let available = vec![QuantumCapability::PulseControl.id()];

        let missing = requirements.missing_required_ids(&available);

        assert_eq!(
            missing,
            vec![QuantumCapability::DynamicCircuits.id()]
        );
    }

    #[test]
    fn optional_requirement_does_not_count_as_required() {
        let capability =
            CapabilityRequirement::new(
                QuantumCapability::PulseControl.into(),
                CapabilityScope::Global,
                RequirementStrength::Optional,
                CapabilityRequirementMode::Program,
                None,
            )
            .unwrap();

        let mut set = CapabilityRequirementSet::new();
        set.require(capability).unwrap();

        let available: Vec<CapabilityId> = Vec::new();

        assert!(set.required_ids_satisfied(&available));
    }

    #[test]
    fn count_mode_requires_count() {
        let result = CapabilityRequirement::new(
            QuantumCapability::PulseControl.into(),
            CapabilityScope::Global,
            RequirementStrength::Required,
            CapabilityRequirementMode::Count,
            None,
        );

        assert!(matches!(
            result,
            Err(CapabilityRequirementError::InvalidMinimumCount)
        ));
    }

    #[test]
    fn non_count_mode_rejects_count() {
        let result = CapabilityRequirement::new(
            QuantumCapability::PulseControl.into(),
            CapabilityScope::Global,
            RequirementStrength::Required,
            CapabilityRequirementMode::Program,
            Some(4),
        );

        assert!(matches!(
            result,
            Err(
                CapabilityRequirementError::CountOnlyModeRequiresCount
            )
        ));
    }

    #[test]
    fn set_validation_passes_for_valid_set() {
        let mut set = CapabilityRequirementSet::new();

        set.require_capability(QuantumCapability::PulseControl)
            .unwrap();

        set.require_for_qubit(
            QuantumCapability::Measurement,
            QubitId::new(42),
        )
        .unwrap();

        assert!(set.validate().is_ok());
    }

    #[test]
    fn categories_are_semantically_stable() {
        assert_eq!(
            QuantumCapability::PulseControl.category(),
            CapabilityCategory::PulseControl
        );

        assert_eq!(
            QuantumCapability::MidCircuitMeasurement.category(),
            CapabilityCategory::Measurement
        );

        assert_eq!(
            QuantumCapability::DynamicCircuits.category(),
            CapabilityCategory::ClassicalControl
        );

        assert_eq!(
            QuantumCapability::QuantumAnnealing.category(),
            CapabilityCategory::Annealing
        );

        assert_eq!(
            QuantumCapability::FaultTolerantExecution.category(),
            CapabilityCategory::FaultTolerance
        );
    }

    #[test]
    fn one_and_large_qubit_ids_have_identical_semantics() {
        let small = QubitId::new(0);
        let large = QubitId::new(usize::MAX);

        let small_requirement =
            CapabilityRequirement::required_for_qubit(
                QuantumCapability::PulseControl,
                small,
            );

        let large_requirement =
            CapabilityRequirement::required_for_qubit(
                QuantumCapability::PulseControl,
                large,
            );

        assert_eq!(
            small_requirement.capability_id(),
            large_requirement.capability_id()
        );

        assert_eq!(
            small_requirement.scope().logical_qubit_id(),
            Some(small)
        );

        assert_eq!(
            large_requirement.scope().logical_qubit_id(),
            Some(large)
        );
    }

    #[test]
    fn physical_qubit_scope_is_distinct_from_logical_scope() {
        let logical =
            CapabilityScope::LogicalQubit(QubitId::new(7));

        let physical =
            CapabilityScope::PhysicalQubit(PhysicalQubitId::new(7));

        assert_ne!(logical, physical);
    }

    #[test]
    fn no_fixed_machine_size_is_encoded() {
        let requirement =
            CapabilityRequirement::required_count(
                QuantumCapability::LogicalQubits,
                usize::MAX,
            )
            .unwrap();

        assert_eq!(
            requirement.minimum_count(),
            Some(usize::MAX)
        );
    }
}