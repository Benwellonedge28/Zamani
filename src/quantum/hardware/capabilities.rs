//! Zamani Quantum Hardware — Capability Model
//!
//! Production-grade, provider-neutral capability model for quantum hardware,
//! simulators, emulators, logical/fault-tolerant systems, analog processors,
//! annealers, photonic/bosonic systems, and distributed quantum systems.
//!
//! # Responsibility
//!
//! This module defines:
//!
//! - atomic hardware capabilities;
//! - capability stability;
//! - capability categories;
//! - capability sets;
//! - capability requirements;
//! - capability matching;
//! - capability negotiation;
//! - capability diagnostics;
//! - capability profiles;
//! - stable serialization;
//! - deterministic ordering;
//! - provider-neutral capability metadata.
//!
//! # Non-responsibilities
//!
//! This module does NOT own:
//!
//! - backend identity;
//! - provider identity;
//! - hardware technology;
//! - topology;
//! - calibration;
//! - instruction semantics;
//! - timing;
//! - execution;
//! - jobs;
//! - queues;
//! - authentication;
//! - credentials;
//! - networking;
//! - provider APIs;
//! - routing algorithms;
//! - scheduling algorithms;
//! - benchmarking;
//! - quantum IR.
//!
//! Those concerns belong to other modules.
//!
//! # Architectural position
//!
//! ```text
//! identity.rs
//!     |
//! technology.rs
//!     |
//! capabilities.rs  <---- this module
//!     |
//!     +---- instruction_set.rs
//!     +---- timing.rs
//!     +---- topology.rs
//!     +---- calibration.rs
//!     |
//!     v
//! compatibility.rs
//!     |
//!     v
//! backend.rs
//! ```
//!
//! Capability data is authoritative for answering:
//!
//! > "Can this execution target perform this operation or satisfy this
//! > workload requirement?"
//!
//! It must not answer:
//!
//! > "How should the operation be routed?"
//!
//! or:
//!
//! > "How should the provider API be called?"
//!
//! # Critical architectural distinction
//!
//! A capability is not an instruction.
//!
//! For example:
//!
//! ```text
//! Capability:
//!     MidCircuitMeasurement
//!
//! Instruction:
//!     measure
//!
//! ```
//!
//! A backend can possess the capability while supporting several different
//! measurement instructions or provider-specific representations.
//!
//! Likewise, a capability is not a technology:
//!
//! ```text
//! technology:
//!     Superconducting
//!
//! capabilities:
//!     Measurement
//!     Reset
//!     MidCircuitMeasurement
//!     DynamicCircuits
//!     ParameterizedGates
//! ```
//!
//! # Capability states
//!
//! A capability is never represented merely by `bool`.
//!
//! Production hardware can expose capabilities that are:
//!
//! - stable;
//! - experimental;
//! - deprecated;
//! - unavailable;
//! - provider-defined;
//! - conditionally available.
//!
//! Therefore this module explicitly models capability status.
//!
//! # Experimental capabilities
//!
//! Experimental capabilities must never silently be treated as stable.
//! Higher-level compatibility code can choose a policy such as:
//!
//! ```text
//! StableOnly
//! StableAndExperimental
//! IncludeDeprecated
//! ```
//!
//! # Provider independence
//!
//! Provider adapters must map provider-specific capabilities into this model.
//! Provider-specific types must never leak into this module.
//!
//! Adding a new provider must therefore not require modification to this
//! module.
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
//! # Serialization
//!
//! Serde is used because it is already an established repository dependency.
//! Serialized identifiers are stable strings rather than Rust enum debug
//! representations.
//!
//! # Stability
//!
//! The string identifiers returned by `as_str()` are part of the persistent
//! hardware contract. They must not be changed casually because they can
//! appear in:
//!
//! - configuration;
//! - manifests;
//! - cache keys;
//! - benchmark records;
//! - execution provenance;
//! - telemetry;
//! - serialized backend descriptions.
//!
//! New capabilities should normally be added without renaming existing
//! identifiers.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// =============================================================================
// Capability status
// =============================================================================

/// Lifecycle/stability state of a hardware capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityStatus {
    /// Fully supported and considered part of the stable backend contract.
    Stable,

    /// Supported, but exposed as experimental by the backend/provider.
    Experimental,

    /// Previously supported but scheduled for removal or replacement.
    Deprecated,

    /// Known capability that is not currently available on this target.
    Unavailable,
}

impl CapabilityStatus {
    /// Stable serialized identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns whether this status represents currently usable support.
    pub const fn is_supported(self) -> bool {
        matches!(self, Self::Stable | Self::Experimental)
    }

    /// Returns whether the capability is stable.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns whether the capability is experimental.
    pub const fn is_experimental(self) -> bool {
        matches!(self, Self::Experimental)
    }

    /// Returns whether the capability is deprecated.
    pub const fn is_deprecated(self) -> bool {
        matches!(self, Self::Deprecated)
    }

    /// Returns whether the capability is unavailable.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }
}

impl Default for CapabilityStatus {
    fn default() -> Self {
        Self::Stable
    }
}

impl fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CapabilityStatus {
    type Err = CapabilityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalize_identifier(value)?.as_str() {
            "stable" => Ok(Self::Stable),
            "experimental" | "experimental_only" => Ok(Self::Experimental),
            "deprecated" => Ok(Self::Deprecated),
            "unavailable" | "unsupported" => Ok(Self::Unavailable),
            _ => Err(CapabilityParseError::UnknownStatus {
                value: value.trim().to_owned(),
            }),
        }
    }
}

impl Serialize for CapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Capability category
// =============================================================================

/// Broad category for a capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityCategory {
    /// Quantum measurement and classical readout.
    Measurement,

    /// Qubit preparation, reset, and reuse.
    StatePreparation,

    /// Gate-based quantum computation.
    GateModel,

    /// Classical control and dynamic execution.
    ClassicalControl,

    /// Parameterized or variational computation.
    Parameterization,

    /// Timing and synchronization.
    Timing,

    /// Pulse-level control.
    PulseControl,

    /// Analog quantum computation.
    Analog,

    /// Quantum annealing.
    Annealing,

    /// Photonic/bosonic/continuous-variable computation.
    Photonic,

    /// Fault-tolerant/logical quantum computation.
    FaultTolerance,

    /// Error correction and mitigation.
    ErrorCorrection,

    /// Parallel/concurrent execution.
    Concurrency,

    /// Workload/job-level execution features.
    Execution,

    /// Result and data-access features.
    Results,

    /// Calibration and device characterization.
    Calibration,

    /// Topology and connectivity information.
    Topology,

    /// Simulator/emulator-specific features.
    Simulation,

    /// Distributed/networked quantum computation.
    Distributed,

    /// Provider-defined extension.
    Custom,
}

impl CapabilityCategory {
    /// Stable serialized identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::StatePreparation => "state_preparation",
            Self::GateModel => "gate_model",
            Self::ClassicalControl => "classical_control",
            Self::Parameterization => "parameterization",
            Self::Timing => "timing",
            Self::PulseControl => "pulse_control",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Photonic => "photonic",
            Self::FaultTolerance => "fault_tolerance",
            Self::ErrorCorrection => "error_correction",
            Self::Concurrency => "concurrency",
            Self::Execution => "execution",
            Self::Results => "results",
            Self::Calibration => "calibration",
            Self::Topology => "topology",
            Self::Simulation => "simulation",
            Self::Distributed => "distributed",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for CapabilityCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for CapabilityCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

impl FromStr for CapabilityCategory {
    type Err = CapabilityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalize_identifier(value)?.as_str() {
            "measurement" => Ok(Self::Measurement),
            "state_preparation" | "state_preparation_and_reset" => {
                Ok(Self::StatePreparation)
            }
            "gate_model" | "gates" | "gate" => Ok(Self::GateModel),
            "classical_control" | "control_flow" => Ok(Self::ClassicalControl),
            "parameterization" | "parameters" => Ok(Self::Parameterization),
            "timing" | "scheduling" => Ok(Self::Timing),
            "pulse_control" | "pulse" => Ok(Self::PulseControl),
            "analog" | "analog_computing" => Ok(Self::Analog),
            "annealing" | "quantum_annealing" => Ok(Self::Annealing),
            "photonic" | "bosonic" | "continuous_variable" => {
                Ok(Self::Photonic)
            }
            "fault_tolerance" | "fault_tolerant" | "logical" => {
                Ok(Self::FaultTolerance)
            }
            "error_correction" | "error_mitigation" => {
                Ok(Self::ErrorCorrection)
            }
            "concurrency" | "parallelism" => Ok(Self::Concurrency),
            "execution" => Ok(Self::Execution),
            "results" | "result" | "data_access" => Ok(Self::Results),
            "calibration" => Ok(Self::Calibration),
            "topology" | "connectivity" => Ok(Self::Topology),
            "simulation" | "simulator" | "emulation" => {
                Ok(Self::Simulation)
            }
            "distributed" | "network" | "networked" => Ok(Self::Distributed),
            "custom" | "provider_defined" => Ok(Self::Custom),
            _ => Err(CapabilityParseError::UnknownCategory {
                value: value.trim().to_owned(),
            }),
        }
    }
}

// =============================================================================
// Capability enumeration
// =============================================================================

/// Atomic capability exposed by a quantum execution target.
///
/// This is intentionally provider-neutral.
///
/// A capability says what the target can do, not how the provider exposes it.
///
/// The vocabulary is deliberately broader than today's gate-model hardware so
/// that Zamani does not need a breaking redesign when new quantum architectures
/// are added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumCapability {
    // -------------------------------------------------------------------------
    // Measurement
    // -------------------------------------------------------------------------

    /// Terminal measurement.
    Measurement,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Measurement of individual quantum resources.
    SingleResourceMeasurement,

    /// Simultaneous measurement of multiple resources.
    ParallelMeasurement,

    /// Measurement basis selection.
    ConfigurableMeasurementBasis,

    /// Measurement of arbitrary observables.
    ObservableMeasurement,

    /// Expectation-value evaluation.
    ExpectationValues,

    /// Sampling from a quantum target.
    Sampling,

    /// Probability distribution output.
    ProbabilityDistributions,

    // -------------------------------------------------------------------------
    // State preparation / reset
    // -------------------------------------------------------------------------

    /// Explicit qubit/resource reset.
    Reset,

    /// Fast reset suitable for repeated circuits.
    FastReset,

    /// Mid-circuit reset.
    MidCircuitReset,

    /// Reuse of quantum resources during one workload.
    QubitReuse,

    /// Explicit state preparation.
    StatePreparation,

    /// Arbitrary state preparation where supported.
    ArbitraryStatePreparation,

    /// Leakage detection.
    LeakageDetection,

    /// Leakage reduction/removal.
    LeakageReduction,

    // -------------------------------------------------------------------------
    // Gate model
    // -------------------------------------------------------------------------

    /// Single-qubit gates.
    SingleQubitGates,

    /// Two-qubit gates.
    TwoQubitGates,

    /// Three-qubit gates.
    ThreeQubitGates,

    /// General multi-qubit gates.
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

    /// Reversible gate operations.
    ReversibleOperations,

    /// Non-unitary quantum operations.
    NonUnitaryOperations,

    // -------------------------------------------------------------------------
    // Classical control / dynamic circuits
    // -------------------------------------------------------------------------

    /// Classical control of quantum operations.
    ClassicalControl,

    /// Conditional quantum operations.
    ConditionalOperations,

    /// Dynamic circuits.
    DynamicCircuits,

    /// Classical feed-forward from measurement results.
    ClassicalFeedForward,

    /// Fast measurement-to-control feedback.
    FastFeedForward,

    /// Runtime branching.
    RuntimeBranching,

    /// Runtime loops.
    RuntimeLoops,

    /// Runtime classical expressions.
    RuntimeClassicalExpressions,

    // -------------------------------------------------------------------------
    // Parameterization / variational execution
    // -------------------------------------------------------------------------

    /// Parameterized workload submission.
    ParameterizedExecution,

    /// Runtime parameter binding.
    RuntimeParameterBinding,

    /// Batch parameter execution.
    ParameterBatchExecution,

    /// Parameter sweeps.
    ParameterSweeps,

    /// Variational execution.
    VariationalExecution,

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------

    /// Hardware-native timing information.
    TimingInformation,

    /// Explicit delays.
    Delays,

    /// Timing alignment constraints.
    TimingAlignment,

    /// Hardware cycle timing.
    CycleTiming,

    /// Synchronization barriers.
    Synchronization,

    /// Precise instruction durations.
    InstructionDurations,

    /// Hardware clock information.
    HardwareClock,

    // -------------------------------------------------------------------------
    // Pulse control
    // -------------------------------------------------------------------------

    /// Pulse-level execution.
    PulseLevelControl,

    /// Custom waveforms.
    CustomWaveforms,

    /// Drive-channel control.
    DriveChannels,

    /// Measure-channel control.
    MeasureChannels,

    /// Acquire-channel control.
    AcquireChannels,

    /// Control-channel access.
    ControlChannels,

    /// Pulse calibrations.
    PulseCalibrations,

    /// Hardware frames.
    Frames,

    /// Pulse schedules.
    PulseSchedules,

    // -------------------------------------------------------------------------
    // Analog quantum computing
    // -------------------------------------------------------------------------

    /// Analog quantum execution.
    AnalogExecution,

    /// Time-dependent Hamiltonians.
    TimeDependentHamiltonians,

    /// Spatially dependent Hamiltonians.
    SpatialHamiltonians,

    /// Analog control fields.
    AnalogControlFields,

    /// Analog observables.
    AnalogObservables,

    /// Analog program submission.
    AnalogProgramSubmission,

    // -------------------------------------------------------------------------
    // Annealing
    // -------------------------------------------------------------------------

    /// Quantum annealing execution.
    QuantumAnnealing,

    /// Ising-model execution.
    IsingModels,

    /// QUBO execution.
    Qubo,

    /// Custom annealing schedules.
    AnnealingSchedules,

    /// Reverse annealing.
    ReverseAnnealing,

    /// Annealing pauses.
    AnnealingPauses,

    /// Annealing gauges.
    AnnealingGauges,

    // -------------------------------------------------------------------------
    // Photonic / bosonic / CV
    // -------------------------------------------------------------------------

    /// Photonic mode computation.
    PhotonicModes,

    /// Bosonic operations.
    BosonicOperations,

    /// Continuous-variable operations.
    ContinuousVariableOperations,

    /// Fock-state operations.
    FockStateOperations,

    /// Gaussian operations.
    GaussianOperations,

    /// Non-Gaussian operations.
    NonGaussianOperations,

    /// Photon-number measurement.
    PhotonNumberMeasurement,

    /// Homodyne measurement.
    HomodyneMeasurement,

    /// Heterodyne measurement.
    HeterodyneMeasurement,

    // -------------------------------------------------------------------------
    // Error correction / fault tolerance
    // -------------------------------------------------------------------------

    /// Logical qubits.
    LogicalQubits,

    /// Logical gates.
    LogicalGates,

    /// Logical measurements.
    LogicalMeasurements,

    /// Error-correcting codes.
    ErrorCorrectionCodes,

    /// Syndrome extraction.
    SyndromeExtraction,

    /// Decoder execution.
    DecoderExecution,

    /// Fault-tolerant operations.
    FaultTolerantOperations,

    /// Transversal operations.
    TransversalOperations,

    /// Magic-state support.
    MagicStateSupport,

    /// Logical reset.
    LogicalReset,

    /// Logical error-rate reporting.
    LogicalErrorRates,

    // -------------------------------------------------------------------------
    // Error mitigation / characterization
    // -------------------------------------------------------------------------

    /// Noise-model exposure.
    NoiseModel,

    /// Readout-error characterization.
    ReadoutErrorCharacterization,

    /// Readout error mitigation.
    ReadoutErrorMitigation,

    /// Gate-error mitigation.
    GateErrorMitigation,

    /// Zero-noise extrapolation.
    ZeroNoiseExtrapolation,

    /// Probabilistic error cancellation.
    ProbabilisticErrorCancellation,

    /// Randomized compiling.
    RandomizedCompiling,

    /// Error-rate estimation.
    ErrorRateEstimation,

    // -------------------------------------------------------------------------
    // Concurrency / resource sharing
    // -------------------------------------------------------------------------

    /// Parallel quantum operations.
    ParallelOperations,

    /// Concurrent execution on independent resources.
    ConcurrentExecution,

    /// Multiple circuits in one submission.
    BatchExecution,

    /// Streaming execution/results.
    StreamingExecution,

    /// Circuit batching.
    CircuitBatching,

    /// Workload prioritization.
    JobPriorities,

    /// Queue information.
    QueueInformation,

    /// Reservation support.
    Reservations,

    // -------------------------------------------------------------------------
    // Execution lifecycle
    // -------------------------------------------------------------------------

    /// Asynchronous execution.
    AsynchronousExecution,

    /// Synchronous execution.
    SynchronousExecution,

    /// Job status polling.
    JobStatus,

    /// Job cancellation.
    JobCancellation,

    /// Job timeout.
    JobTimeout,

    /// Retryable execution.
    RetryableExecution,

    /// Execution metadata.
    ExecutionMetadata,

    /// Cost estimation.
    CostEstimation,

    // -------------------------------------------------------------------------
    // Results
    // -------------------------------------------------------------------------

    /// Bitstring/count results.
    Counts,

    /// Raw samples.
    RawSamples,

    /// State-vector access.
    StateVector,

    /// Density-matrix access.
    DensityMatrix,

    /// Amplitude access.
    Amplitudes,

    /// Wavefunction access.
    Wavefunction,

    /// Classical register output.
    ClassicalRegisters,

    /// Raw measurement records.
    RawMeasurementRecords,

    /// Analog result acquisition.
    AnalogResultAcquisition,

    /// Annealing sample results.
    AnnealingResults,

    /// Logical result reporting.
    LogicalResults,

    // -------------------------------------------------------------------------
    // Calibration
    // -------------------------------------------------------------------------

    /// Calibration data access.
    CalibrationData,

    /// Calibration snapshots.
    CalibrationSnapshots,

    /// Calibration versioning.
    CalibrationVersioning,

    /// Calibration provenance.
    CalibrationProvenance,

    /// Hardware characterization.
    HardwareCharacterization,

    // -------------------------------------------------------------------------
    // Topology
    // -------------------------------------------------------------------------

    /// Topology information.
    TopologyInformation,

    /// Connectivity information.
    ConnectivityInformation,

    /// Directed coupling information.
    DirectedCoupling,

    /// Modular hardware topology.
    ModularTopology,

    /// Multi-chip/inter-module connectivity.
    InterModuleConnectivity,

    // -------------------------------------------------------------------------
    // Simulation / emulation
    // -------------------------------------------------------------------------

    /// State-vector simulation.
    StateVectorSimulation,

    /// Stabilizer simulation.
    StabilizerSimulation,

    /// Tensor-network simulation.
    TensorNetworkSimulation,

    /// Density-matrix simulation.
    DensityMatrixSimulation,

    /// Trajectory simulation.
    TrajectorySimulation,

    /// Noisy simulation.
    NoisySimulation,

    /// Deterministic simulation.
    DeterministicSimulation,

    /// Hardware emulation.
    HardwareEmulation,

    /// Fault injection.
    FaultInjection,

    /// Configurable simulation seed.
    DeterministicSeeding,

    // -------------------------------------------------------------------------
    // Distributed quantum
    // -------------------------------------------------------------------------

    /// Distributed quantum execution.
    DistributedExecution,

    /// Remote quantum resources.
    RemoteQuantumResources,

    /// Quantum network links.
    QuantumNetworkLinks,

    /// Entanglement resource management.
    EntanglementResources,

    /// Remote gates.
    RemoteOperations,

    /// Quantum teleportation primitives.
    Teleportation,

    // -------------------------------------------------------------------------
    // Provider / custom
    // -------------------------------------------------------------------------

    /// Provider-specific extension capability.
    ProviderExtension,
}

impl QuantumCapability {
    /// Stable serialized identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::SingleResourceMeasurement => "single_resource_measurement",
            Self::ParallelMeasurement => "parallel_measurement",
            Self::ConfigurableMeasurementBasis => "configurable_measurement_basis",
            Self::ObservableMeasurement => "observable_measurement",
            Self::ExpectationValues => "expectation_values",
            Self::Sampling => "sampling",
            Self::ProbabilityDistributions => "probability_distributions",

            Self::Reset => "reset",
            Self::FastReset => "fast_reset",
            Self::MidCircuitReset => "mid_circuit_reset",
            Self::QubitReuse => "qubit_reuse",
            Self::StatePreparation => "state_preparation",
            Self::ArbitraryStatePreparation => "arbitrary_state_preparation",
            Self::LeakageDetection => "leakage_detection",
            Self::LeakageReduction => "leakage_reduction",

            Self::SingleQubitGates => "single_qubit_gates",
            Self::TwoQubitGates => "two_qubit_gates",
            Self::ThreeQubitGates => "three_qubit_gates",
            Self::MultiQubitGates => "multi_qubit_gates",
            Self::ArbitrarySingleQubitRotations => "arbitrary_single_qubit_rotations",
            Self::ParameterizedGates => "parameterized_gates",
            Self::NativeGateExecution => "native_gate_execution",
            Self::ControlledOperations => "controlled_operations",
            Self::AdjointOperations => "adjoint_operations",
            Self::ReversibleOperations => "reversible_operations",
            Self::NonUnitaryOperations => "non_unitary_operations",

            Self::ClassicalControl => "classical_control",
            Self::ConditionalOperations => "conditional_operations",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::ClassicalFeedForward => "classical_feed_forward",
            Self::FastFeedForward => "fast_feed_forward",
            Self::RuntimeBranching => "runtime_branching",
            Self::RuntimeLoops => "runtime_loops",
            Self::RuntimeClassicalExpressions => "runtime_classical_expressions",

            Self::ParameterizedExecution => "parameterized_execution",
            Self::RuntimeParameterBinding => "runtime_parameter_binding",
            Self::ParameterBatchExecution => "parameter_batch_execution",
            Self::ParameterSweeps => "parameter_sweeps",
            Self::VariationalExecution => "variational_execution",

            Self::TimingInformation => "timing_information",
            Self::Delays => "delays",
            Self::TimingAlignment => "timing_alignment",
            Self::CycleTiming => "cycle_timing",
            Self::Synchronization => "synchronization",
            Self::InstructionDurations => "instruction_durations",
            Self::HardwareClock => "hardware_clock",

            Self::PulseLevelControl => "pulse_level_control",
            Self::CustomWaveforms => "custom_waveforms",
            Self::DriveChannels => "drive_channels",
            Self::MeasureChannels => "measure_channels",
            Self::AcquireChannels => "acquire_channels",
            Self::ControlChannels => "control_channels",
            Self::PulseCalibrations => "pulse_calibrations",
            Self::Frames => "frames",
            Self::PulseSchedules => "pulse_schedules",

            Self::AnalogExecution => "analog_execution",
            Self::TimeDependentHamiltonians => "time_dependent_hamiltonians",
            Self::SpatialHamiltonians => "spatial_hamiltonians",
            Self::AnalogControlFields => "analog_control_fields",
            Self::AnalogObservables => "analog_observables",
            Self::AnalogProgramSubmission => "analog_program_submission",

            Self::QuantumAnnealing => "quantum_annealing",
            Self::IsingModels => "ising_models",
            Self::Qubo => "qubo",
            Self::AnnealingSchedules => "annealing_schedules",
            Self::ReverseAnnealing => "reverse_annealing",
            Self::AnnealingPauses => "annealing_pauses",
            Self::AnnealingGauges => "annealing_gauges",

            Self::PhotonicModes => "photonic_modes",
            Self::BosonicOperations => "bosonic_operations",
            Self::ContinuousVariableOperations => "continuous_variable_operations",
            Self::FockStateOperations => "fock_state_operations",
            Self::GaussianOperations => "gaussian_operations",
            Self::NonGaussianOperations => "non_gaussian_operations",
            Self::PhotonNumberMeasurement => "photon_number_measurement",
            Self::HomodyneMeasurement => "homodyne_measurement",
            Self::HeterodyneMeasurement => "heterodyne_measurement",

            Self::LogicalQubits => "logical_qubits",
            Self::LogicalGates => "logical_gates",
            Self::LogicalMeasurements => "logical_measurements",
            Self::ErrorCorrectionCodes => "error_correction_codes",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::DecoderExecution => "decoder_execution",
            Self::FaultTolerantOperations => "fault_tolerant_operations",
            Self::TransversalOperations => "transversal_operations",
            Self::MagicStateSupport => "magic_state_support",
            Self::LogicalReset => "logical_reset",
            Self::LogicalErrorRates => "logical_error_rates",

            Self::NoiseModel => "noise_model",
            Self::ReadoutErrorCharacterization => "readout_error_characterization",
            Self::ReadoutErrorMitigation => "readout_error_mitigation",
            Self::GateErrorMitigation => "gate_error_mitigation",
            Self::ZeroNoiseExtrapolation => "zero_noise_extrapolation",
            Self::ProbabilisticErrorCancellation => "probabilistic_error_cancellation",
            Self::RandomizedCompiling => "randomized_compiling",
            Self::ErrorRateEstimation => "error_rate_estimation",

            Self::ParallelOperations => "parallel_operations",
            Self::ConcurrentExecution => "concurrent_execution",
            Self::BatchExecution => "batch_execution",
            Self::StreamingExecution => "streaming_execution",
            Self::CircuitBatching => "circuit_batching",
            Self::JobPriorities => "job_priorities",
            Self::QueueInformation => "queue_information",
            Self::Reservations => "reservations",

            Self::AsynchronousExecution => "asynchronous_execution",
            Self::SynchronousExecution => "synchronous_execution",
            Self::JobStatus => "job_status",
            Self::JobCancellation => "job_cancellation",
            Self::JobTimeout => "job_timeout",
            Self::RetryableExecution => "retryable_execution",
            Self::ExecutionMetadata => "execution_metadata",
            Self::CostEstimation => "cost_estimation",

            Self::Counts => "counts",
            Self::RawSamples => "raw_samples",
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Amplitudes => "amplitudes",
            Self::Wavefunction => "wavefunction",
            Self::ClassicalRegisters => "classical_registers",
            Self::RawMeasurementRecords => "raw_measurement_records",
            Self::AnalogResultAcquisition => "analog_result_acquisition",
            Self::AnnealingResults => "annealing_results",
            Self::LogicalResults => "logical_results",

            Self::CalibrationData => "calibration_data",
            Self::CalibrationSnapshots => "calibration_snapshots",
            Self::CalibrationVersioning => "calibration_versioning",
            Self::CalibrationProvenance => "calibration_provenance",
            Self::HardwareCharacterization => "hardware_characterization",

            Self::TopologyInformation => "topology_information",
            Self::ConnectivityInformation => "connectivity_information",
            Self::DirectedCoupling => "directed_coupling",
            Self::ModularTopology => "modular_topology",
            Self::InterModuleConnectivity => "inter_module_connectivity",

            Self::StateVectorSimulation => "state_vector_simulation",
            Self::StabilizerSimulation => "stabilizer_simulation",
            Self::TensorNetworkSimulation => "tensor_network_simulation",
            Self::DensityMatrixSimulation => "density_matrix_simulation",
            Self::TrajectorySimulation => "trajectory_simulation",
            Self::NoisySimulation => "noisy_simulation",
            Self::DeterministicSimulation => "deterministic_simulation",
            Self::HardwareEmulation => "hardware_emulation",
            Self::FaultInjection => "fault_injection",
            Self::DeterministicSeeding => "deterministic_seeding",

            Self::DistributedExecution => "distributed_execution",
            Self::RemoteQuantumResources => "remote_quantum_resources",
            Self::QuantumNetworkLinks => "quantum_network_links",
            Self::EntanglementResources => "entanglement_resources",
            Self::RemoteOperations => "remote_operations",
            Self::Teleportation => "teleportation",

            Self::ProviderExtension => "provider_extension",
        }
    }

    /// Human-readable capability name.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Measurement => "Measurement",
            Self::MidCircuitMeasurement => "Mid-Circuit Measurement",
            Self::SingleResourceMeasurement => "Single-Resource Measurement",
            Self::ParallelMeasurement => "Parallel Measurement",
            Self::ConfigurableMeasurementBasis => "Configurable Measurement Basis",
            Self::ObservableMeasurement => "Observable Measurement",
            Self::ExpectationValues => "Expectation Values",
            Self::Sampling => "Sampling",
            Self::ProbabilityDistributions => "Probability Distributions",

            Self::Reset => "Reset",
            Self::FastReset => "Fast Reset",
            Self::MidCircuitReset => "Mid-Circuit Reset",
            Self::QubitReuse => "Qubit Reuse",
            Self::StatePreparation => "State Preparation",
            Self::ArbitraryStatePreparation => "Arbitrary State Preparation",
            Self::LeakageDetection => "Leakage Detection",
            Self::LeakageReduction => "Leakage Reduction",

            Self::SingleQubitGates => "Single-Qubit Gates",
            Self::TwoQubitGates => "Two-Qubit Gates",
            Self::ThreeQubitGates => "Three-Qubit Gates",
            Self::MultiQubitGates => "Multi-Qubit Gates",
            Self::ArbitrarySingleQubitRotations => "Arbitrary Single-Qubit Rotations",
            Self::ParameterizedGates => "Parameterized Gates",
            Self::NativeGateExecution => "Native Gate Execution",
            Self::ControlledOperations => "Controlled Operations",
            Self::AdjointOperations => "Adjoint Operations",
            Self::ReversibleOperations => "Reversible Operations",
            Self::NonUnitaryOperations => "Non-Unitary Operations",

            Self::ClassicalControl => "Classical Control",
            Self::ConditionalOperations => "Conditional Operations",
            Self::DynamicCircuits => "Dynamic Circuits",
            Self::ClassicalFeedForward => "Classical Feed-Forward",
            Self::FastFeedForward => "Fast Feed-Forward",
            Self::RuntimeBranching => "Runtime Branching",
            Self::RuntimeLoops => "Runtime Loops",
            Self::RuntimeClassicalExpressions => "Runtime Classical Expressions",

            Self::ParameterizedExecution => "Parameterized Execution",
            Self::RuntimeParameterBinding => "Runtime Parameter Binding",
            Self::ParameterBatchExecution => "Parameter Batch Execution",
            Self::ParameterSweeps => "Parameter Sweeps",
            Self::VariationalExecution => "Variational Execution",

            Self::TimingInformation => "Timing Information",
            Self::Delays => "Delays",
            Self::TimingAlignment => "Timing Alignment",
            Self::CycleTiming => "Cycle Timing",
            Self::Synchronization => "Synchronization",
            Self::InstructionDurations => "Instruction Durations",
            Self::HardwareClock => "Hardware Clock",

            Self::PulseLevelControl => "Pulse-Level Control",
            Self::CustomWaveforms => "Custom Waveforms",
            Self::DriveChannels => "Drive Channels",
            Self::MeasureChannels => "Measure Channels",
            Self::AcquireChannels => "Acquire Channels",
            Self::ControlChannels => "Control Channels",
            Self::PulseCalibrations => "Pulse Calibrations",
            Self::Frames => "Frames",
            Self::PulseSchedules => "Pulse Schedules",

            Self::AnalogExecution => "Analog Execution",
            Self::TimeDependentHamiltonians => "Time-Dependent Hamiltonians",
            Self::SpatialHamiltonians => "Spatial Hamiltonians",
            Self::AnalogControlFields => "Analog Control Fields",
            Self::AnalogObservables => "Analog Observables",
            Self::AnalogProgramSubmission => "Analog Program Submission",

            Self::QuantumAnnealing => "Quantum Annealing",
            Self::IsingModels => "Ising Models",
            Self::Qubo => "QUBO",
            Self::AnnealingSchedules => "Annealing Schedules",
            Self::ReverseAnnealing => "Reverse Annealing",
            Self::AnnealingPauses => "Annealing Pauses",
            Self::AnnealingGauges => "Annealing Gauges",

            Self::PhotonicModes => "Photonic Modes",
            Self::BosonicOperations => "Bosonic Operations",
            Self::ContinuousVariableOperations => "Continuous-Variable Operations",
            Self::FockStateOperations => "Fock-State Operations",
            Self::GaussianOperations => "Gaussian Operations",
            Self::NonGaussianOperations => "Non-Gaussian Operations",
            Self::PhotonNumberMeasurement => "Photon-Number Measurement",
            Self::HomodyneMeasurement => "Homodyne Measurement",
            Self::HeterodyneMeasurement => "Heterodyne Measurement",

            Self::LogicalQubits => "Logical Qubits",
            Self::LogicalGates => "Logical Gates",
            Self::LogicalMeasurements => "Logical Measurements",
            Self::ErrorCorrectionCodes => "Error-Correction Codes",
            Self::SyndromeExtraction => "Syndrome Extraction",
            Self::DecoderExecution => "Decoder Execution",
            Self::FaultTolerantOperations => "Fault-Tolerant Operations",
            Self::TransversalOperations => "Transversal Operations",
            Self::MagicStateSupport => "Magic-State Support",
            Self::LogicalReset => "Logical Reset",
            Self::LogicalErrorRates => "Logical Error Rates",

            Self::NoiseModel => "Noise Model",
            Self::ReadoutErrorCharacterization => "Readout-Error Characterization",
            Self::ReadoutErrorMitigation => "Readout-Error Mitigation",
            Self::GateErrorMitigation => "Gate-Error Mitigation",
            Self::ZeroNoiseExtrapolation => "Zero-Noise Extrapolation",
            Self::ProbabilisticErrorCancellation => "Probabilistic Error Cancellation",
            Self::RandomizedCompiling => "Randomized Compiling",
            Self::ErrorRateEstimation => "Error-Rate Estimation",

            Self::ParallelOperations => "Parallel Operations",
            Self::ConcurrentExecution => "Concurrent Execution",
            Self::BatchExecution => "Batch Execution",
            Self::StreamingExecution => "Streaming Execution",
            Self::CircuitBatching => "Circuit Batching",
            Self::JobPriorities => "Job Priorities",
            Self::QueueInformation => "Queue Information",
            Self::Reservations => "Reservations",

            Self::AsynchronousExecution => "Asynchronous Execution",
            Self::SynchronousExecution => "Synchronous Execution",
            Self::JobStatus => "Job Status",
            Self::JobCancellation => "Job Cancellation",
            Self::JobTimeout => "Job Timeout",
            Self::RetryableExecution => "Retryable Execution",
            Self::ExecutionMetadata => "Execution Metadata",
            Self::CostEstimation => "Cost Estimation",

            Self::Counts => "Counts",
            Self::RawSamples => "Raw Samples",
            Self::StateVector => "State Vector",
            Self::DensityMatrix => "Density Matrix",
            Self::Amplitudes => "Amplitudes",
            Self::Wavefunction => "Wavefunction",
            Self::ClassicalRegisters => "Classical Registers",
            Self::RawMeasurementRecords => "Raw Measurement Records",
            Self::AnalogResultAcquisition => "Analog Result Acquisition",
            Self::AnnealingResults => "Annealing Results",
            Self::LogicalResults => "Logical Results",

            Self::CalibrationData => "Calibration Data",
            Self::CalibrationSnapshots => "Calibration Snapshots",
            Self::CalibrationVersioning => "Calibration Versioning",
            Self::CalibrationProvenance => "Calibration Provenance",
            Self::HardwareCharacterization => "Hardware Characterization",

            Self::TopologyInformation => "Topology Information",
            Self::ConnectivityInformation => "Connectivity Information",
            Self::DirectedCoupling => "Directed Coupling",
            Self::ModularTopology => "Modular Topology",
            Self::InterModuleConnectivity => "Inter-Module Connectivity",

            Self::StateVectorSimulation => "State-Vector Simulation",
            Self::StabilizerSimulation => "Stabilizer Simulation",
            Self::TensorNetworkSimulation => "Tensor-Network Simulation",
            Self::DensityMatrixSimulation => "Density-Matrix Simulation",
            Self::TrajectorySimulation => "Trajectory Simulation",
            Self::NoisySimulation => "Noisy Simulation",
            Self::DeterministicSimulation => "Deterministic Simulation",
            Self::HardwareEmulation => "Hardware Emulation",
            Self::FaultInjection => "Fault Injection",
            Self::DeterministicSeeding => "Deterministic Seeding",

            Self::DistributedExecution => "Distributed Execution",
            Self::RemoteQuantumResources => "Remote Quantum Resources",
            Self::QuantumNetworkLinks => "Quantum Network Links",
            Self::EntanglementResources => "Entanglement Resources",
            Self::RemoteOperations => "Remote Operations",
            Self::Teleportation => "Teleportation",

            Self::ProviderExtension => "Provider Extension",
        }
    }

    /// Returns the capability category.
    pub const fn category(self) -> CapabilityCategory {
        match self {
            Self::Measurement
            | Self::MidCircuitMeasurement
            | Self::SingleResourceMeasurement
            | Self::ParallelMeasurement
            | Self::ConfigurableMeasurementBasis
            | Self::ObservableMeasurement
            | Self::ExpectationValues
            | Self::Sampling
            | Self::ProbabilityDistributions
            | Self::PhotonNumberMeasurement
            | Self::HomodyneMeasurement
            | Self::HeterodyneMeasurement => CapabilityCategory::Measurement,

            Self::Reset
            | Self::FastReset
            | Self::MidCircuitReset
            | Self::QubitReuse
            | Self::StatePreparation
            | Self::ArbitraryStatePreparation
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
            | Self::ReversibleOperations
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

            Self::TimingInformation
            | Self::Delays
            | Self::TimingAlignment
            | Self::CycleTiming
            | Self::Synchronization
            | Self::InstructionDurations
            | Self::HardwareClock => CapabilityCategory::Timing,

            Self::PulseLevelControl
            | Self::CustomWaveforms
            | Self::DriveChannels
            | Self::MeasureChannels
            | Self::AcquireChannels
            | Self::ControlChannels
            | Self::PulseCalibrations
            | Self::Frames
            | Self::PulseSchedules => CapabilityCategory::PulseControl,

            Self::AnalogExecution
            | Self::TimeDependentHamiltonians
            | Self::SpatialHamiltonians
            | Self::AnalogControlFields
            | Self::AnalogObservables
            | Self::AnalogProgramSubmission => CapabilityCategory::Analog,

            Self::QuantumAnnealing
            | Self::IsingModels
            | Self::Qubo
            | Self::AnnealingSchedules
            | Self::ReverseAnnealing
            | Self::AnnealingPauses
            | Self::AnnealingGauges => CapabilityCategory::Annealing,

            Self::PhotonicModes
            | Self::BosonicOperations
            | Self::ContinuousVariableOperations
            | Self::FockStateOperations
            | Self::GaussianOperations
            | Self::NonGaussianOperations
            | Self::PhotonNumberMeasurement
            | Self::HomodyneMeasurement
            | Self::HeterodyneMeasurement => CapabilityCategory::Photonic,

            Self::LogicalQubits
            | Self::LogicalGates
            | Self::LogicalMeasurements
            | Self::ErrorCorrectionCodes
            | Self::SyndromeExtraction
            | Self::DecoderExecution
            | Self::FaultTolerantOperations
            | Self::TransversalOperations
            | Self::MagicStateSupport
            | Self::LogicalReset
            | Self::LogicalErrorRates => CapabilityCategory::FaultTolerance,

            Self::NoiseModel
            | Self::ReadoutErrorCharacterization
            | Self::ReadoutErrorMitigation
            | Self::GateErrorMitigation
            | Self::ZeroNoiseExtrapolation
            | Self::ProbabilisticErrorCancellation
            | Self::RandomizedCompiling
            | Self::ErrorRateEstimation => CapabilityCategory::ErrorCorrection,

            Self::ParallelOperations
            | Self::ConcurrentExecution
            | Self::BatchExecution
            | Self::StreamingExecution
            | Self::CircuitBatching
            | Self::JobPriorities
            | Self::QueueInformation
            | Self::Reservations => CapabilityCategory::Concurrency,

            Self::AsynchronousExecution
            | Self::SynchronousExecution
            | Self::JobStatus
            | Self::JobCancellation
            | Self::JobTimeout
            | Self::RetryableExecution
            | Self::ExecutionMetadata
            | Self::CostEstimation => CapabilityCategory::Execution,

            Self::Counts
            | Self::RawSamples
            | Self::StateVector
            | Self::DensityMatrix
            | Self::Amplitudes
            | Self::Wavefunction
            | Self::ClassicalRegisters
            | Self::RawMeasurementRecords
            | Self::AnalogResultAcquisition
            | Self::AnnealingResults
            | Self::LogicalResults => CapabilityCategory::Results,

            Self::CalibrationData
            | Self::CalibrationSnapshots
            | Self::CalibrationVersioning
            | Self::CalibrationProvenance
            | Self::HardwareCharacterization => CapabilityCategory::Calibration,

            Self::TopologyInformation
            | Self::ConnectivityInformation
            | Self::DirectedCoupling
            | Self::ModularTopology
            | Self::InterModuleConnectivity => CapabilityCategory::Topology,

            Self::StateVectorSimulation
            | Self::StabilizerSimulation
            | Self::TensorNetworkSimulation
            | Self::DensityMatrixSimulation
            | Self::TrajectorySimulation
            | Self::NoisySimulation
            | Self::DeterministicSimulation
            | Self::HardwareEmulation
            | Self::FaultInjection
            | Self::DeterministicSeeding => CapabilityCategory::Simulation,

            Self::DistributedExecution
            | Self::RemoteQuantumResources
            | Self::QuantumNetworkLinks
            | Self::EntanglementResources
            | Self::RemoteOperations
            | Self::Teleportation => CapabilityCategory::Distributed,

            Self::ProviderExtension => CapabilityCategory::Custom,
        }
    }

    /// Returns whether this capability is inherently high-risk if treated as
    /// universally available.
    ///
    /// This is used by compatibility and execution policy layers to prevent
    /// accidental assumptions about powerful or provider-sensitive features.
    pub const fn requires_explicit_opt_in(self) -> bool {
        matches!(
            self,
            Self::PulseLevelControl
                | Self::CustomWaveforms
                | Self::AnalogExecution
                | Self::QuantumAnnealing
                | Self::LogicalQubits
                | Self::FaultTolerantOperations
                | Self::ProviderExtension
                | Self::DistributedExecution
                | Self::RemoteQuantumResources
                | Self::RuntimeBranching
                | Self::RuntimeLoops
                | Self::RawMeasurementRecords
                | Self::StateVector
                | Self::DensityMatrix
                | Self::Wavefunction
        )
    }
}

impl fmt::Display for QuantumCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
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

        Self::from_str(&value).map_err(serde::de::Error::custom)
    }
}

impl FromStr for QuantumCapability {
    type Err = CapabilityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_identifier(value)?;

        CAPABILITY_TABLE
            .iter()
            .find(|(identifier, _)| *identifier == normalized)
            .map(|(_, capability)| *capability)
            .ok_or_else(|| CapabilityParseError::UnknownCapability {
                value: value.trim().to_owned(),
            })
    }
}

// =============================================================================
// Capability parse errors
// =============================================================================

/// Parsing errors for capability values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityParseError {
    /// The supplied value was empty.
    Empty,

    /// The supplied value contains unsupported syntax.
    InvalidIdentifier {
        /// Original value.
        value: String,
    },

    /// Unknown capability identifier.
    UnknownCapability {
        /// Original value.
        value: String,
    },

    /// Unknown capability category.
    UnknownCategory {
        /// Original value.
        value: String,
    },

    /// Unknown capability status.
    UnknownStatus {
        /// Original value.
        value: String,
    },
}

impl fmt::Display for CapabilityParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "capability identifier cannot be empty"),

            Self::InvalidIdentifier { value } => {
                write!(f, "invalid capability identifier '{}'", value)
            }

            Self::UnknownCapability { value } => {
                write!(f, "unknown quantum capability '{}'", value)
            }

            Self::UnknownCategory { value } => {
                write!(f, "unknown capability category '{}'", value)
            }

            Self::UnknownStatus { value } => {
                write!(f, "unknown capability status '{}'", value)
            }
        }
    }
}

impl Error for CapabilityParseError {}

// =============================================================================
// Capability metadata
// =============================================================================

/// Detailed metadata for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    /// Atomic capability.
    pub capability: QuantumCapability,

    /// Lifecycle/stability state.
    #[serde(default)]
    pub status: CapabilityStatus,

    /// Optional provider-independent explanation.
    #[serde(default)]
    pub description: Option<String>,

    /// Optional stable provider/API identifier.
    ///
    /// This is metadata only. It must never contain secrets.
    #[serde(default)]
    pub external_identifier: Option<String>,
}

impl CapabilityDescriptor {
    /// Construct a stable capability descriptor.
    pub fn stable(capability: QuantumCapability) -> Self {
        Self {
            capability,
            status: CapabilityStatus::Stable,
            description: None,
            external_identifier: None,
        }
    }

    /// Construct an experimental capability descriptor.
    pub fn experimental(capability: QuantumCapability) -> Self {
        Self {
            capability,
            status: CapabilityStatus::Experimental,
            description: None,
            external_identifier: None,
        }
    }

    /// Set a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set an external non-secret identifier.
    pub fn with_external_identifier(
        mut self,
        identifier: impl Into<String>,
    ) -> Self {
        self.external_identifier = Some(identifier.into());
        self
    }

    /// Returns whether the descriptor currently represents supported
    /// capability.
    pub fn is_supported(&self) -> bool {
        self.status.is_supported()
    }
}

// =============================================================================
// Capability policy
// =============================================================================

/// Policy controlling which capability statuses are acceptable during
/// compatibility checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityPolicy {
    /// Only stable capabilities satisfy requirements.
    StableOnly,

    /// Stable and experimental capabilities satisfy requirements.
    StableAndExperimental,

    /// Stable, experimental, and deprecated capabilities satisfy requirements.
    ///
    /// This is intended for migration and compatibility analysis, not normal
    /// production execution.
    IncludeDeprecated,

    /// Any explicitly advertised capability status except unavailable.
    AllSupported,
}

impl Default for CapabilityPolicy {
    fn default() -> Self {
        Self::StableOnly
    }
}

impl CapabilityPolicy {
    /// Determine whether a capability status satisfies this policy.
    pub const fn accepts(self, status: CapabilityStatus) -> bool {
        match self {
            Self::StableOnly => matches!(status, CapabilityStatus::Stable),

            Self::StableAndExperimental => {
                matches!(
                    status,
                    CapabilityStatus::Stable | CapabilityStatus::Experimental
                )
            }

            Self::IncludeDeprecated => {
                matches!(
                    status,
                    CapabilityStatus::Stable
                        | CapabilityStatus::Experimental
                        | CapabilityStatus::Deprecated
                )
            }

            Self::AllSupported => status.is_supported(),
        }
    }
}

impl fmt::Display for CapabilityPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::StableOnly => "stable_only",
            Self::StableAndExperimental => "stable_and_experimental",
            Self::IncludeDeprecated => "include_deprecated",
            Self::AllSupported => "all_supported",
        };

        f.write_str(value)
    }
}

impl Serialize for CapabilityPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CapabilityPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match normalize_identifier(&value)
            .map_err(serde::de::Error::custom)?
            .as_str()
        {
            "stable_only" => Ok(Self::StableOnly),
            "stable_and_experimental" => Ok(Self::StableAndExperimental),
            "include_deprecated" => Ok(Self::IncludeDeprecated),
            "all_supported" => Ok(Self::AllSupported),
            _ => Err(serde::de::Error::custom(
                "unknown capability policy",
            )),
        }
    }
}

// =============================================================================
// Capability set
// =============================================================================

/// Deterministic collection of capabilities exposed by one execution target.
///
/// `BTreeMap` is deliberately used rather than `HashMap` so that:
//!
//! - serialization is deterministic;
//! - equality is deterministic;
//! - benchmark metadata is reproducible;
//! - logs are stable;
//! - registry queries are deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySet {
    /// Capability descriptors indexed by their atomic capability.
    #[serde(default)]
    capabilities: BTreeMap<QuantumCapability, CapabilityDescriptor>,
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitySet {
    /// Construct an empty capability set.
    pub fn new() -> Self {
        Self {
            capabilities: BTreeMap::new(),
        }
    }

    /// Construct a set containing stable capabilities.
    pub fn from_capabilities<I>(capabilities: I) -> Self
    where
        I: IntoIterator<Item = QuantumCapability>,
    {
        let mut set = Self::new();

        for capability in capabilities {
            set.insert(capability);
        }

        set
    }

    /// Insert or replace a stable capability.
    pub fn insert(&mut self, capability: QuantumCapability) {
        self.capabilities
            .insert(capability, CapabilityDescriptor::stable(capability));
    }

    /// Insert an explicit descriptor.
    pub fn insert_descriptor(&mut self, descriptor: CapabilityDescriptor) {
        self.capabilities
            .insert(descriptor.capability, descriptor);
    }

    /// Insert an experimental capability.
    pub fn insert_experimental(&mut self, capability: QuantumCapability) {
        self.capabilities.insert(
            capability,
            CapabilityDescriptor::experimental(capability),
        );
    }

    /// Remove a capability.
    pub fn remove(&mut self, capability: QuantumCapability) -> Option<CapabilityDescriptor> {
        self.capabilities.remove(&capability)
    }

    /// Returns whether a capability is present regardless of lifecycle status.
    pub fn contains(&self, capability: QuantumCapability) -> bool {
        self.capabilities.contains_key(&capability)
    }

    /// Returns whether a capability satisfies the supplied policy.
    pub fn supports(
        &self,
        capability: QuantumCapability,
        policy: CapabilityPolicy,
    ) -> bool {
        self.capabilities
            .get(&capability)
            .map(|descriptor| policy.accepts(descriptor.status))
            .unwrap_or(false)
    }

    /// Returns the descriptor for a capability.
    pub fn descriptor(
        &self,
        capability: QuantumCapability,
    ) -> Option<&CapabilityDescriptor> {
        self.capabilities.get(&capability)
    }

    /// Return the number of advertised capabilities.
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Return whether the set contains no capabilities.
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Iterate over descriptors in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&QuantumCapability, &CapabilityDescriptor)> {
        self.capabilities.iter()
    }

    /// Iterate over capabilities in deterministic order.
    pub fn capabilities(&self) -> impl Iterator<Item = QuantumCapability> + '_ {
        self.capabilities.keys().copied()
    }

    /// Return all capabilities in a deterministic set.
    pub fn to_set(&self) -> BTreeSet<QuantumCapability> {
        self.capabilities.keys().copied().collect()
    }

    /// Return all capabilities in one category.
    pub fn by_category(
        &self,
        category: CapabilityCategory,
    ) -> Vec<QuantumCapability> {
        self.capabilities
            .keys()
            .copied()
            .filter(|capability| capability.category() == category)
            .collect()
    }

    /// Return all capabilities accepted by a policy.
    pub fn supported(
        &self,
        policy: CapabilityPolicy,
    ) -> Vec<QuantumCapability> {
        self.capabilities
            .iter()
            .filter_map(|(capability, descriptor)| {
                policy
                    .accepts(descriptor.status)
                    .then_some(*capability)
            })
            .collect()
    }

    /// Return all experimental capabilities.
    pub fn experimental(&self) -> Vec<QuantumCapability> {
        self.capabilities
            .iter()
            .filter_map(|(capability, descriptor)| {
                descriptor.status.is_experimental().then_some(*capability)
            })
            .collect()
    }

    /// Return all deprecated capabilities.
    pub fn deprecated(&self) -> Vec<QuantumCapability> {
        self.capabilities
            .iter()
            .filter_map(|(capability, descriptor)| {
                descriptor.status.is_deprecated().then_some(*capability)
            })
            .collect()
    }

    /// Return all unavailable capabilities.
    pub fn unavailable(&self) -> Vec<QuantumCapability> {
        self.capabilities
            .iter()
            .filter_map(|(capability, descriptor)| {
                descriptor.status.is_unavailable().then_some(*capability)
            })
            .collect()
    }

    /// Require every capability in `requirements`.
    pub fn check(
        &self,
        requirements: &CapabilityRequirements,
        policy: CapabilityPolicy,
    ) -> CapabilityCheckResult {
        let mut missing = BTreeSet::new();
        let mut rejected_status = BTreeMap::new();

        for capability in requirements.required.iter().copied() {
            match self.capabilities.get(&capability) {
                Some(descriptor) if policy.accepts(descriptor.status) => {}

                Some(descriptor) => {
                    rejected_status.insert(capability, descriptor.status);
                }

                None => {
                    missing.insert(capability);
                }
            }
        }

        let mut warnings = Vec::new();

        for capability in requirements.preferred.iter().copied() {
            if !self.supports(capability, policy) {
                warnings.push(CapabilityWarning::PreferredUnavailable {
                    capability,
                });
            }
        }

        if requirements
            .required
            .iter()
            .any(|capability| capability.requires_explicit_opt_in())
        {
            for capability in requirements.required.iter().copied() {
                if capability.requires_explicit_opt_in() {
                    warnings.push(CapabilityWarning::ExplicitOptInRequired {
                        capability,
                    });
                }
            }
        }

        CapabilityCheckResult {
            compatible: missing.is_empty() && rejected_status.is_empty(),
            missing,
            rejected_status,
            warnings,
        }
    }

    /// Merge another capability set into this one.
    ///
    /// The incoming descriptor replaces the existing descriptor for the same
    /// capability. This is intentionally explicit rather than silently
    /// combining lifecycle states.
    pub fn merge(&mut self, other: &CapabilitySet) {
        for descriptor in other.capabilities.values() {
            self.insert_descriptor(descriptor.clone());
        }
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// Requirements imposed by a quantum workload.
///
/// This is intentionally separate from `CapabilitySet`:
///
/// ```text
/// CapabilitySet
///     = what hardware provides
///
/// CapabilityRequirements
///     = what workload requires
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Capabilities that must be available.
    #[serde(default)]
    pub required: BTreeSet<QuantumCapability>,

    /// Capabilities that are desirable but not mandatory.
    #[serde(default)]
    pub preferred: BTreeSet<QuantumCapability>,

    /// Policy under which the requirements are normally evaluated.
    #[serde(default)]
    pub policy: CapabilityPolicy,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityRequirements {
    /// Construct empty requirements using the production-safe
    /// `StableOnly` policy.
    pub fn new() -> Self {
        Self {
            required: BTreeSet::new(),
            preferred: BTreeSet::new(),
            policy: CapabilityPolicy::StableOnly,
        }
    }

    /// Require one capability.
    pub fn require(mut self, capability: QuantumCapability) -> Self {
        self.required.insert(capability);
        self
    }

    /// Require multiple capabilities.
    pub fn require_all<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = QuantumCapability>,
    {
        self.required.extend(capabilities);
        self
    }

    /// Prefer one capability.
    pub fn prefer(mut self, capability: QuantumCapability) -> Self {
        self.preferred.insert(capability);
        self
    }

    /// Prefer multiple capabilities.
    pub fn prefer_all<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = QuantumCapability>,
    {
        self.preferred.extend(capabilities);
        self
    }

    /// Set the capability acceptance policy.
    pub fn with_policy(mut self, policy: CapabilityPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Returns whether no requirements exist.
    pub fn is_empty(&self) -> bool {
        self.required.is_empty() && self.preferred.is_empty()
    }

    /// Returns whether a capability is mandatory.
    pub fn requires(&self, capability: QuantumCapability) -> bool {
        self.required.contains(&capability)
    }

    /// Returns whether a capability is preferred.
    pub fn prefers(&self, capability: QuantumCapability) -> bool {
        self.preferred.contains(&capability)
    }
}

// =============================================================================
// Capability diagnostics
// =============================================================================

/// Warning generated during capability negotiation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityWarning {
    /// A preferred capability is unavailable.
    PreferredUnavailable {
        capability: QuantumCapability,
    },

    /// A capability needs explicit execution opt-in.
    ExplicitOptInRequired {
        capability: QuantumCapability,
    },

    /// The backend exposes a deprecated capability.
    DeprecatedCapability {
        capability: QuantumCapability,
    },
}

impl fmt::Display for CapabilityWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreferredUnavailable { capability } => write!(
                f,
                "preferred capability '{}' is unavailable",
                capability
            ),

            Self::ExplicitOptInRequired { capability } => write!(
                f,
                "capability '{}' requires explicit opt-in",
                capability
            ),

            Self::DeprecatedCapability { capability } => write!(
                f,
                "capability '{}' is deprecated",
                capability
            ),
        }
    }
}

/// Result of capability negotiation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityCheckResult {
    /// True only when all mandatory requirements are satisfied.
    pub compatible: bool,

    /// Capabilities completely absent from the target.
    #[serde(default)]
    pub missing: BTreeSet<QuantumCapability>,

    /// Capabilities present but rejected by the selected lifecycle policy.
    #[serde(default)]
    pub rejected_status: BTreeMap<QuantumCapability, CapabilityStatus>,

    /// Non-fatal diagnostics.
    #[serde(default)]
    pub warnings: Vec<CapabilityWarning>,
}

impl CapabilityCheckResult {
    /// Returns whether there is any incompatibility.
    pub fn is_incompatible(&self) -> bool {
        !self.compatible
    }

    /// Returns whether warnings were generated.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Returns the total number of blocking requirements.
    pub fn blocking_count(&self) -> usize {
        self.missing.len() + self.rejected_status.len()
    }
}

// =============================================================================
// Capability profiles
// =============================================================================

/// Named, reusable capability profile.
///
/// Profiles provide canonical baseline configurations for common target types
/// without claiming that every backend of that type has identical capabilities.
///
/// Provider adapters can start from a profile and then add/remove capabilities
/// according to actual backend metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityProfile {
    /// Conservative gate-model target.
    GateModel,

    /// Dynamic-circuit gate-model target.
    DynamicCircuit,

    /// Pulse-capable target.
    Pulse,

    /// Analog target.
    Analog,

    /// Quantum annealer.
    Annealing,

    /// Photonic/bosonic target.
    Photonic,

    /// Logical/fault-tolerant target.
    Logical,

    /// State-vector simulator.
    StateVectorSimulator,

    /// Hardware emulator.
    HardwareEmulator,

    /// Distributed quantum target.
    Distributed,
}

impl CapabilityProfile {
    /// Construct the baseline capabilities for this profile.
    pub fn capabilities(self) -> CapabilitySet {
        match self {
            Self::GateModel => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Reset,
                QuantumCapability::SingleQubitGates,
                QuantumCapability::TwoQubitGates,
                QuantumCapability::NativeGateExecution,
                QuantumCapability::Sampling,
                QuantumCapability::Counts,
                QuantumCapability::SynchronousExecution,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
                QuantumCapability::ExecutionMetadata,
            ]),

            Self::DynamicCircuit => {
                let mut set = Self::GateModel.capabilities();

                for capability in [
                    QuantumCapability::MidCircuitMeasurement,
                    QuantumCapability::MidCircuitReset,
                    QuantumCapability::ClassicalControl,
                    QuantumCapability::ConditionalOperations,
                    QuantumCapability::DynamicCircuits,
                    QuantumCapability::ClassicalFeedForward,
                ] {
                    set.insert(capability);
                }

                set
            }

            Self::Pulse => {
                let mut set = Self::DynamicCircuit.capabilities();

                for capability in [
                    QuantumCapability::TimingInformation,
                    QuantumCapability::Delays,
                    QuantumCapability::TimingAlignment,
                    QuantumCapability::CycleTiming,
                    QuantumCapability::Synchronization,
                    QuantumCapability::InstructionDurations,
                    QuantumCapability::HardwareClock,
                    QuantumCapability::PulseLevelControl,
                    QuantumCapability::CustomWaveforms,
                    QuantumCapability::DriveChannels,
                    QuantumCapability::MeasureChannels,
                    QuantumCapability::AcquireChannels,
                    QuantumCapability::ControlChannels,
                    QuantumCapability::PulseCalibrations,
                    QuantumCapability::Frames,
                    QuantumCapability::PulseSchedules,
                ] {
                    set.insert(capability);
                }

                set
            }

            Self::Analog => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Sampling,
                QuantumCapability::ProbabilityDistributions,
                QuantumCapability::AnalogExecution,
                QuantumCapability::TimeDependentHamiltonians,
                QuantumCapability::SpatialHamiltonians,
                QuantumCapability::AnalogControlFields,
                QuantumCapability::AnalogObservables,
                QuantumCapability::AnalogProgramSubmission,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
                QuantumCapability::ExecutionMetadata,
            ]),

            Self::Annealing => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Sampling,
                QuantumCapability::ProbabilityDistributions,
                QuantumCapability::QuantumAnnealing,
                QuantumCapability::IsingModels,
                QuantumCapability::Qubo,
                QuantumCapability::AnnealingSchedules,
                QuantumCapability::AnnealingResults,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
                QuantumCapability::ExecutionMetadata,
            ]),

            Self::Photonic => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Sampling,
                QuantumCapability::ProbabilityDistributions,
                QuantumCapability::PhotonicModes,
                QuantumCapability::BosonicOperations,
                QuantumCapability::ContinuousVariableOperations,
                QuantumCapability::GaussianOperations,
                QuantumCapability::PhotonNumberMeasurement,
                QuantumCapability::Counts,
                QuantumCapability::RawSamples,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
            ]),

            Self::Logical => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::LogicalQubits,
                QuantumCapability::LogicalGates,
                QuantumCapability::LogicalMeasurements,
                QuantumCapability::ErrorCorrectionCodes,
                QuantumCapability::SyndromeExtraction,
                QuantumCapability::DecoderExecution,
                QuantumCapability::FaultTolerantOperations,
                QuantumCapability::LogicalReset,
                QuantumCapability::LogicalErrorRates,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
                QuantumCapability::ExecutionMetadata,
            ]),

            Self::StateVectorSimulator => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Reset,
                QuantumCapability::SingleQubitGates,
                QuantumCapability::TwoQubitGates,
                QuantumCapability::ThreeQubitGates,
                QuantumCapability::MultiQubitGates,
                QuantumCapability::ParameterizedGates,
                QuantumCapability::Sampling,
                QuantumCapability::Counts,
                QuantumCapability::ProbabilityDistributions,
                QuantumCapability::StateVector,
                QuantumCapability::Amplitudes,
                QuantumCapability::Wavefunction,
                QuantumCapability::StateVectorSimulation,
                QuantumCapability::DeterministicSimulation,
                QuantumCapability::DeterministicSeeding,
                QuantumCapability::SynchronousExecution,
                QuantumCapability::BatchExecution,
            ]),

            Self::HardwareEmulator => {
                let mut set = Self::GateModel.capabilities();

                for capability in [
                    QuantumCapability::NoiseModel,
                    QuantumCapability::ReadoutErrorCharacterization,
                    QuantumCapability::ErrorRateEstimation,
                    QuantumCapability::HardwareEmulation,
                    QuantumCapability::FaultInjection,
                    QuantumCapability::DeterministicSeeding,
                    QuantumCapability::CalibrationData,
                ] {
                    set.insert(capability);
                }

                set
            }

            Self::Distributed => CapabilitySet::from_capabilities([
                QuantumCapability::Measurement,
                QuantumCapability::Sampling,
                QuantumCapability::DistributedExecution,
                QuantumCapability::RemoteQuantumResources,
                QuantumCapability::QuantumNetworkLinks,
                QuantumCapability::EntanglementResources,
                QuantumCapability::RemoteOperations,
                QuantumCapability::Teleportation,
                QuantumCapability::AsynchronousExecution,
                QuantumCapability::JobStatus,
                QuantumCapability::ExecutionMetadata,
            ]),
        }
    }
}

// =============================================================================
// Capability utility constants
// =============================================================================

/// Complete authoritative capability table used by parsing.
///
/// Keeping this as one deterministic table makes parsing exhaustive and
/// prevents provider adapters from creating stringly-typed core capabilities.
static CAPABILITY_TABLE: &[(&str, QuantumCapability)] = &[
    ("measurement", QuantumCapability::Measurement),
    (
        "mid_circuit_measurement",
        QuantumCapability::MidCircuitMeasurement,
    ),
    (
        "single_resource_measurement",
        QuantumCapability::SingleResourceMeasurement,
    ),
    (
        "parallel_measurement",
        QuantumCapability::ParallelMeasurement,
    ),
    (
        "configurable_measurement_basis",
        QuantumCapability::ConfigurableMeasurementBasis,
    ),
    (
        "observable_measurement",
        QuantumCapability::ObservableMeasurement,
    ),
    (
        "expectation_values",
        QuantumCapability::ExpectationValues,
    ),
    ("sampling", QuantumCapability::Sampling),
    (
        "probability_distributions",
        QuantumCapability::ProbabilityDistributions,
    ),
    ("reset", QuantumCapability::Reset),
    ("fast_reset", QuantumCapability::FastReset),
    ("mid_circuit_reset", QuantumCapability::MidCircuitReset),
    ("qubit_reuse", QuantumCapability::QubitReuse),
    ("state_preparation", QuantumCapability::StatePreparation),
    (
        "arbitrary_state_preparation",
        QuantumCapability::ArbitraryStatePreparation,
    ),
    (
        "leakage_detection",
        QuantumCapability::LeakageDetection,
    ),
    (
        "leakage_reduction",
        QuantumCapability::LeakageReduction,
    ),
    (
        "single_qubit_gates",
        QuantumCapability::SingleQubitGates,
    ),
    ("two_qubit_gates", QuantumCapability::TwoQubitGates),
    ("three_qubit_gates", QuantumCapability::ThreeQubitGates),
    (
        "multi_qubit_gates",
        QuantumCapability::MultiQubitGates,
    ),
    (
        "arbitrary_single_qubit_rotations",
        QuantumCapability::ArbitrarySingleQubitRotations,
    ),
    (
        "parameterized_gates",
        QuantumCapability::ParameterizedGates,
    ),
    (
        "native_gate_execution",
        QuantumCapability::NativeGateExecution,
    ),
    (
        "controlled_operations",
        QuantumCapability::ControlledOperations,
    ),
    (
        "adjoint_operations",
        QuantumCapability::AdjointOperations,
    ),
    (
        "reversible_operations",
        QuantumCapability::ReversibleOperations,
    ),
    (
        "non_unitary_operations",
        QuantumCapability::NonUnitaryOperations,
    ),
    (
        "classical_control",
        QuantumCapability::ClassicalControl,
    ),
    (
        "conditional_operations",
        QuantumCapability::ConditionalOperations,
    ),
    (
        "dynamic_circuits",
        QuantumCapability::DynamicCircuits,
    ),
    (
        "classical_feed_forward",
        QuantumCapability::ClassicalFeedForward,
    ),
    (
        "fast_feed_forward",
        QuantumCapability::FastFeedForward,
    ),
    (
        "runtime_branching",
        QuantumCapability::RuntimeBranching,
    ),
    (
        "runtime_loops",
        QuantumCapability::RuntimeLoops,
    ),
    (
        "runtime_classical_expressions",
        QuantumCapability::RuntimeClassicalExpressions,
    ),
    (
        "parameterized_execution",
        QuantumCapability::ParameterizedExecution,
    ),
    (
        "runtime_parameter_binding",
        QuantumCapability::RuntimeParameterBinding,
    ),
    (
        "parameter_batch_execution",
        QuantumCapability::ParameterBatchExecution,
    ),
    ("parameter_sweeps", QuantumCapability::ParameterSweeps),
    (
        "variational_execution",
        QuantumCapability::VariationalExecution,
    ),
    (
        "timing_information",
        QuantumCapability::TimingInformation,
    ),
    ("delays", QuantumCapability::Delays),
    (
        "timing_alignment",
        QuantumCapability::TimingAlignment,
    ),
    ("cycle_timing", QuantumCapability::CycleTiming),
    ("synchronization", QuantumCapability::Synchronization),
    (
        "instruction_durations",
        QuantumCapability::InstructionDurations,
    ),
    ("hardware_clock", QuantumCapability::HardwareClock),
    (
        "pulse_level_control",
        QuantumCapability::PulseLevelControl,
    ),
    ("custom_waveforms", QuantumCapability::CustomWaveforms),
    ("drive_channels", QuantumCapability::DriveChannels),
    ("measure_channels", QuantumCapability::MeasureChannels),
    ("acquire_channels", QuantumCapability::AcquireChannels),
    ("control_channels", QuantumCapability::ControlChannels),
    (
        "pulse_calibrations",
        QuantumCapability::PulseCalibrations,
    ),
    ("frames", QuantumCapability::Frames),
    ("pulse_schedules", QuantumCapability::PulseSchedules),
    (
        "analog_execution",
        QuantumCapability::AnalogExecution,
    ),
    (
        "time_dependent_hamiltonians",
        QuantumCapability::TimeDependentHamiltonians,
    ),
    (
        "spatial_hamiltonians",
        QuantumCapability::SpatialHamiltonians,
    ),
    (
        "analog_control_fields",
        QuantumCapability::AnalogControlFields,
    ),
    (
        "analog_observables",
        QuantumCapability::AnalogObservables,
    ),
    (
        "analog_program_submission",
        QuantumCapability::AnalogProgramSubmission,
    ),
    (
        "quantum_annealing",
        QuantumCapability::QuantumAnnealing,
    ),
    ("ising_models", QuantumCapability::IsingModels),
    ("qubo", QuantumCapability::Qubo),
    (
        "annealing_schedules",
        QuantumCapability::AnnealingSchedules,
    ),
    (
        "reverse_annealing",
        QuantumCapability::ReverseAnnealing,
    ),
    (
        "annealing_pauses",
        QuantumCapability::AnnealingPauses,
    ),
    (
        "annealing_gauges",
        QuantumCapability::AnnealingGauges,
    ),
    (
        "photonic_modes",
        QuantumCapability::PhotonicModes,
    ),
    (
        "bosonic_operations",
        QuantumCapability::BosonicOperations,
    ),
    (
        "continuous_variable_operations",
        QuantumCapability::ContinuousVariableOperations,
    ),
    (
        "fock_state_operations",
        QuantumCapability::FockStateOperations,
    ),
    (
        "gaussian_operations",
        QuantumCapability::GaussianOperations,
    ),
    (
        "non_gaussian_operations",
        QuantumCapability::NonGaussianOperations,
    ),
    (
        "photon_number_measurement",
        QuantumCapability::PhotonNumberMeasurement,
    ),
    (
        "homodyne_measurement",
        QuantumCapability::HomodyneMeasurement,
    ),
    (
        "heterodyne_measurement",
        QuantumCapability::HeterodyneMeasurement,
    ),
    ("logical_qubits", QuantumCapability::LogicalQubits),
    ("logical_gates", QuantumCapability::LogicalGates),
    (
        "logical_measurements",
        QuantumCapability::LogicalMeasurements,
    ),
    (
        "error_correction_codes",
        QuantumCapability::ErrorCorrectionCodes,
    ),
    (
        "syndrome_extraction",
        QuantumCapability::SyndromeExtraction,
    ),
    (
        "decoder_execution",
        QuantumCapability::DecoderExecution,
    ),
    (
        "fault_tolerant_operations",
        QuantumCapability::FaultTolerantOperations,
    ),
    (
        "transversal_operations",
        QuantumCapability::TransversalOperations,
    ),
    (
        "magic_state_support",
        QuantumCapability::MagicStateSupport,
    ),
    ("logical_reset", QuantumCapability::LogicalReset),
    (
        "logical_error_rates",
        QuantumCapability::LogicalErrorRates,
    ),
    ("noise_model", QuantumCapability::NoiseModel),
    (
        "readout_error_characterization",
        QuantumCapability::ReadoutErrorCharacterization,
    ),
    (
        "readout_error_mitigation",
        QuantumCapability::ReadoutErrorMitigation,
    ),
    (
        "gate_error_mitigation",
        QuantumCapability::GateErrorMitigation,
    ),
    (
        "zero_noise_extrapolation",
        QuantumCapability::ZeroNoiseExtrapolation,
    ),
    (
        "probabilistic_error_cancellation",
        QuantumCapability::ProbabilisticErrorCancellation,
    ),
    (
        "randomized_compiling",
        QuantumCapability::RandomizedCompiling,
    ),
    (
        "error_rate_estimation",
        QuantumCapability::ErrorRateEstimation,
    ),
    (
        "parallel_operations",
        QuantumCapability::ParallelOperations,
    ),
    (
        "concurrent_execution",
        QuantumCapability::ConcurrentExecution,
    ),
    (
        "batch_execution",
        QuantumCapability::BatchExecution,
    ),
    (
        "streaming_execution",
        QuantumCapability::StreamingExecution,
    ),
    (
        "circuit_batching",
        QuantumCapability::CircuitBatching,
    ),
    ("job_priorities", QuantumCapability::JobPriorities),
    (
        "queue_information",
        QuantumCapability::QueueInformation,
    ),
    ("reservations", QuantumCapability::Reservations),
    (
        "asynchronous_execution",
        QuantumCapability::AsynchronousExecution,
    ),
    (
        "synchronous_execution",
        QuantumCapability::SynchronousExecution,
    ),
    ("job_status", QuantumCapability::JobStatus),
    (
        "job_cancellation",
        QuantumCapability::JobCancellation,
    ),
    ("job_timeout", QuantumCapability::JobTimeout),
    (
        "retryable_execution",
        QuantumCapability::RetryableExecution,
    ),
    (
        "execution_metadata",
        QuantumCapability::ExecutionMetadata,
    ),
    ("cost_estimation", QuantumCapability::CostEstimation),
    ("counts", QuantumCapability::Counts),
    ("raw_samples", QuantumCapability::RawSamples),
    ("state_vector", QuantumCapability::StateVector),
    ("density_matrix", QuantumCapability::DensityMatrix),
    ("amplitudes", QuantumCapability::Amplitudes),
    ("wavefunction", QuantumCapability::Wavefunction),
    (
        "classical_registers",
        QuantumCapability::ClassicalRegisters,
    ),
    (
        "raw_measurement_records",
        QuantumCapability::RawMeasurementRecords,
    ),
    (
        "analog_result_acquisition",
        QuantumCapability::AnalogResultAcquisition,
    ),
    (
        "annealing_results",
        QuantumCapability::AnnealingResults,
    ),
    (
        "logical_results",
        QuantumCapability::LogicalResults,
    ),
    (
        "calibration_data",
        QuantumCapability::CalibrationData,
    ),
    (
        "calibration_snapshots",
        QuantumCapability::CalibrationSnapshots,
    ),
    (
        "calibration_versioning",
        QuantumCapability::CalibrationVersioning,
    ),
    (
        "calibration_provenance",
        QuantumCapability::CalibrationProvenance,
    ),
    (
        "hardware_characterization",
        QuantumCapability::HardwareCharacterization,
    ),
    (
        "topology_information",
        QuantumCapability::TopologyInformation,
    ),
    (
        "connectivity_information",
        QuantumCapability::ConnectivityInformation,
    ),
    (
        "directed_coupling",
        QuantumCapability::DirectedCoupling,
    ),
    (
        "modular_topology",
        QuantumCapability::ModularTopology,
    ),
    (
        "inter_module_connectivity",
        QuantumCapability::InterModuleConnectivity,
    ),
    (
        "state_vector_simulation",
        QuantumCapability::StateVectorSimulation,
    ),
    (
        "stabilizer_simulation",
        QuantumCapability::StabilizerSimulation,
    ),
    (
        "tensor_network_simulation",
        QuantumCapability::TensorNetworkSimulation,
    ),
    (
        "density_matrix_simulation",
        QuantumCapability::DensityMatrixSimulation,
    ),
    (
        "trajectory_simulation",
        QuantumCapability::TrajectorySimulation,
    ),
    (
        "noisy_simulation",
        QuantumCapability::NoisySimulation,
    ),
    (
        "deterministic_simulation",
        QuantumCapability::DeterministicSimulation,
    ),
    (
        "hardware_emulation",
        QuantumCapability::HardwareEmulation,
    ),
    ("fault_injection", QuantumCapability::FaultInjection),
    (
        "deterministic_seeding",
        QuantumCapability::DeterministicSeeding,
    ),
    (
        "distributed_execution",
        QuantumCapability::DistributedExecution,
    ),
    (
        "remote_quantum_resources",
        QuantumCapability::RemoteQuantumResources,
    ),
    (
        "quantum_network_links",
        QuantumCapability::QuantumNetworkLinks,
    ),
    (
        "entanglement_resources",
        QuantumCapability::EntanglementResources,
    ),
    (
        "remote_operations",
        QuantumCapability::RemoteOperations,
    ),
    ("teleportation", QuantumCapability::Teleportation),
    (
        "provider_extension",
        QuantumCapability::ProviderExtension,
    ),
];

// =============================================================================
// Normalization
// =============================================================================

/// Normalize a capability identifier.
///
/// Supported normalization:
///
/// - surrounding whitespace is rejected rather than silently changed;
/// - ASCII uppercase is converted to lowercase;
/// - `-` is converted to `_`;
/// - spaces are converted to `_`;
/// - repeated `_` characters are collapsed.
///
/// This permits configuration/user input such as:
///
/// ```text
/// mid-circuit-measurement
/// Mid Circuit Measurement
/// MID_CIRCUIT_MEASUREMENT
/// ```
///
/// while keeping canonical serialization stable.
fn normalize_identifier(value: &str) -> Result<String, CapabilityParseError> {
    if value.is_empty() {
        return Err(CapabilityParseError::Empty);
    }

    if value.trim() != value {
        return Err(CapabilityParseError::InvalidIdentifier {
            value: value.to_owned(),
        });
    }

    let mut result = String::with_capacity(value.len());
    let mut previous_separator = false;

    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character.to_ascii_lowercase());
            previous_separator = false;
        } else if matches!(character, '-' | '_' | ' ') {
            if !previous_separator && !result.is_empty() {
                result.push('_');
                previous_separator = true;
            }
        } else {
            return Err(CapabilityParseError::InvalidIdentifier {
                value: value.to_owned(),
            });
        }
    }

    while result.ends_with('_') {
        result.pop();
    }

    if result.is_empty() {
        return Err(CapabilityParseError::Empty);
    }

    Ok(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_identifiers_are_stable() {
        assert_eq!(
            QuantumCapability::MidCircuitMeasurement.as_str(),
            "mid_circuit_measurement"
        );

        assert_eq!(
            QuantumCapability::DynamicCircuits.as_str(),
            "dynamic_circuits"
        );

        assert_eq!(
            QuantumCapability::PulseLevelControl.as_str(),
            "pulse_level_control"
        );
    }

    #[test]
    fn capability_categories_are_deterministic() {
        assert_eq!(
            QuantumCapability::Measurement.category(),
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
            QuantumCapability::LogicalQubits.category(),
            CapabilityCategory::FaultTolerance
        );
    }

    #[test]
    fn capability_parsing_accepts_common_forms() {
        assert_eq!(
            QuantumCapability::from_str("mid_circuit_measurement").unwrap(),
            QuantumCapability::MidCircuitMeasurement
        );

        assert_eq!(
            QuantumCapability::from_str("MID-CIRCUIT-MEASUREMENT").unwrap(),
            QuantumCapability::MidCircuitMeasurement
        );

        assert_eq!(
            QuantumCapability::from_str("Mid Circuit Measurement").unwrap(),
            QuantumCapability::MidCircuitMeasurement
        );
    }

    #[test]
    fn capability_parsing_rejects_unknown_values() {
        let result = QuantumCapability::from_str("not_a_real_capability");

        assert!(matches!(
            result,
            Err(CapabilityParseError::UnknownCapability { .. })
        ));
    }

    #[test]
    fn status_policy_is_conservative_by_default() {
        assert!(CapabilityPolicy::StableOnly.accepts(
            CapabilityStatus::Stable
        ));

        assert!(!CapabilityPolicy::StableOnly.accepts(
            CapabilityStatus::Experimental
        ));

        assert!(!CapabilityPolicy::StableOnly.accepts(
            CapabilityStatus::Deprecated
        ));

        assert!(!CapabilityPolicy::StableOnly.accepts(
            CapabilityStatus::Unavailable
        ));
    }

    #[test]
    fn experimental_policy_accepts_experimental_capabilities() {
        assert!(
            CapabilityPolicy::StableAndExperimental
                .accepts(CapabilityStatus::Experimental)
        );
    }

    #[test]
    fn capability_set_is_deterministic() {
        let set = CapabilitySet::from_capabilities([
            QuantumCapability::Measurement,
            QuantumCapability::Reset,
            QuantumCapability::TwoQubitGates,
        ]);

        let values: Vec<_> = set.capabilities().collect();

        assert_eq!(
            values,
            vec![
                QuantumCapability::Measurement,
                QuantumCapability::Reset,
                QuantumCapability::TwoQubitGates,
            ]
        );
    }

    #[test]
    fn capability_set_supports_stable_capabilities() {
        let set = CapabilitySet::from_capabilities([
            QuantumCapability::Measurement,
            QuantumCapability::Reset,
        ]);

        assert!(set.supports(
            QuantumCapability::Measurement,
            CapabilityPolicy::StableOnly
        ));

        assert!(!set.supports(
            QuantumCapability::DynamicCircuits,
            CapabilityPolicy::StableOnly
        ));
    }

    #[test]
    fn experimental_capabilities_do_not_satisfy_stable_only() {
        let mut set = CapabilitySet::new();

        set.insert_experimental(QuantumCapability::DynamicCircuits);

        assert!(!set.supports(
            QuantumCapability::DynamicCircuits,
            CapabilityPolicy::StableOnly
        ));

        assert!(set.supports(
            QuantumCapability::DynamicCircuits,
            CapabilityPolicy::StableAndExperimental
        ));
    }

    #[test]
    fn capability_requirements_detect_missing_capabilities() {
        let hardware = CapabilitySet::from_capabilities([
            QuantumCapability::Measurement,
            QuantumCapability::Reset,
        ]);

        let requirements = CapabilityRequirements::new()
            .require(QuantumCapability::Measurement)
            .require(QuantumCapability::DynamicCircuits);

        let result = hardware.check(
            &requirements,
            CapabilityPolicy::StableOnly,
        );

        assert!(!result.compatible);
        assert!(result
            .missing
            .contains(&QuantumCapability::DynamicCircuits));
        assert!(result
            .missing
            .contains(&QuantumCapability::DynamicCircuits));
    }

    #[test]
    fn preferred_capability_generates_warning_not_failure() {
        let hardware = CapabilitySet::from_capabilities([
            QuantumCapability::Measurement,
        ]);

        let requirements = CapabilityRequirements::new()
            .require(QuantumCapability::Measurement)
            .prefer(QuantumCapability::DynamicCircuits);

        let result = hardware.check(
            &requirements,
            CapabilityPolicy::StableOnly,
        );

        assert!(result.compatible);
        assert!(result.has_warnings());
        assert_eq!(result.blocking_count(), 0);
    }

    #[test]
    fn required_experimental_capability_is_rejected_by_stable_policy() {
        let mut hardware = CapabilitySet::new();

        hardware.insert_experimental(QuantumCapability::DynamicCircuits);

        let requirements = CapabilityRequirements::new()
            .require(QuantumCapability::DynamicCircuits);

        let result = hardware.check(
            &requirements,
            CapabilityPolicy::StableOnly,
        );

        assert!(!result.compatible);
        assert_eq!(
            result.rejected_status.get(
                &QuantumCapability::DynamicCircuits
            ),
            Some(&CapabilityStatus::Experimental)
        );
    }

    #[test]
    fn required_experimental_capability_can_be_accepted_explicitly() {
        let mut hardware = CapabilitySet::new();

        hardware.insert_experimental(QuantumCapability::DynamicCircuits);

        let requirements = CapabilityRequirements::new()
            .require(QuantumCapability::DynamicCircuits);

        let result = hardware.check(
            &requirements,
            CapabilityPolicy::StableAndExperimental,
        );

        assert!(result.compatible);
    }

    #[test]
    fn profiles_are_useful_baselines() {
        let gate_model = CapabilityProfile::GateModel.capabilities();

        assert!(gate_model.contains(QuantumCapability::Measurement));
        assert!(gate_model.contains(QuantumCapability::Reset));
        assert!(gate_model.contains(QuantumCapability::TwoQubitGates));

        assert!(
            !gate_model.contains(
                QuantumCapability::PulseLevelControl
            )
        );
    }

    #[test]
    fn dynamic_profile_extends_gate_model() {
        let dynamic = CapabilityProfile::DynamicCircuit.capabilities();

        assert!(dynamic.contains(
            QuantumCapability::MidCircuitMeasurement
        ));

        assert!(dynamic.contains(
            QuantumCapability::ClassicalFeedForward
        ));

        assert!(dynamic.contains(
            QuantumCapability::DynamicCircuits
        ));
    }

    #[test]
    fn pulse_profile_contains_timing_and_pulse_capabilities() {
        let pulse = CapabilityProfile::Pulse.capabilities();

        assert!(pulse.contains(
            QuantumCapability::TimingInformation
        ));

        assert!(pulse.contains(
            QuantumCapability::PulseLevelControl
        ));

        assert!(pulse.contains(
            QuantumCapability::CustomWaveforms
        ));
    }

    #[test]
    fn analog_profile_is_not_a_gate_model_profile() {
        let analog = CapabilityProfile::Analog.capabilities();

        assert!(analog.contains(
            QuantumCapability::AnalogExecution
        ));

        assert!(!analog.contains(
            QuantumCapability::TwoQubitGates
        ));
    }

    #[test]
    fn annealing_profile_contains_qubo_and_ising() {
        let annealing = CapabilityProfile::Annealing.capabilities();

        assert!(annealing.contains(
            QuantumCapability::QuantumAnnealing
        ));

        assert!(annealing.contains(
            QuantumCapability::Qubo
        ));

        assert!(annealing.contains(
            QuantumCapability::IsingModels
        ));
    }

    #[test]
    fn simulator_profile_exposes_state_vector() {
        let simulator =
            CapabilityProfile::StateVectorSimulator.capabilities();

        assert!(simulator.contains(
            QuantumCapability::StateVector
        ));

        assert!(simulator.contains(
            QuantumCapability::StateVectorSimulation
        ));

        assert!(simulator.contains(
            QuantumCapability::DeterministicSeeding
        ));
    }

    #[test]
    fn distributed_profile_contains_network_capabilities() {
        let distributed =
            CapabilityProfile::Distributed.capabilities();

        assert!(distributed.contains(
            QuantumCapability::DistributedExecution
        ));

        assert!(distributed.contains(
            QuantumCapability::EntanglementResources
        ));
    }

    #[test]
    fn serialization_uses_stable_strings() {
        let capability =
            serde_json::to_string(
                &QuantumCapability::MidCircuitMeasurement
            )
            .unwrap();

        assert_eq!(
            capability,
            "\"mid_circuit_measurement\""
        );

        let decoded: QuantumCapability =
            serde_json::from_str(&capability).unwrap();

        assert_eq!(
            decoded,
            QuantumCapability::MidCircuitMeasurement
        );
    }

    #[test]
    fn capability_set_serialization_is_deterministic() {
        let set = CapabilitySet::from_capabilities([
            QuantumCapability::Reset,
            QuantumCapability::Measurement,
            QuantumCapability::TwoQubitGates,
        ]);

        let first = serde_json::to_string(&set).unwrap();
        let second = serde_json::to_string(&set).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn capability_set_merge_is_explicit() {
        let mut first = CapabilitySet::from_capabilities([
            QuantumCapability::Measurement,
        ]);

        let second = CapabilitySet::from_capabilities([
            QuantumCapability::Reset,
        ]);

        first.merge(&second);

        assert!(first.contains(QuantumCapability::Measurement));
        assert!(first.contains(QuantumCapability::Reset));
    }

    #[test]
    fn explicit_opt_in_capabilities_are_marked() {
        assert!(
            QuantumCapability::PulseLevelControl
                .requires_explicit_opt_in()
        );

        assert!(
            QuantumCapability::AnalogExecution
                .requires_explicit_opt_in()
        );

        assert!(
            !QuantumCapability::Measurement
                .requires_explicit_opt_in()
        );
    }

    #[test]
    fn status_serialization_is_stable() {
        let value =
            serde_json::to_string(&CapabilityStatus::Experimental)
                .unwrap();

        assert_eq!(value, "\"experimental\"");
    }

    #[test]
    fn policy_serialization_is_stable() {
        let value =
            serde_json::to_string(
                &CapabilityPolicy::StableAndExperimental,
            )
            .unwrap();

        assert_eq!(value, "\"stable_and_experimental\"");
    }
}