//! Zamani Quantum Benchmarking — Backend Capability Model
//!
//! Production-grade, backend-independent capability negotiation for the
//! quantum benchmarking subsystem.
//!
//! # Responsibility
//!
//! This module answers one question:
//!
//!     "Can this benchmark workload be meaningfully executed against this
//!      target, and if not, exactly why not?"
//!
//! It deliberately does NOT:
//!
//! - execute circuits;
//! - submit jobs;
//! - perform network I/O;
//! - own hardware topology;
//! - own calibration data;
//! - own Quantum IR;
//! - perform routing or transpilation;
//! - implement benchmark protocols;
//! - impose protocol-specific statistical assumptions.
//!
//! Those responsibilities remain owned by their respective Zamani modules.
//!
//! # Architectural position
//!
//! ```text
//! Benchmark protocol
//!        │
//!        ▼
//! Benchmark capability requirements
//!        │
//!        ▼
//! this module
//!        │
//!        ├── capability compatibility
//!        ├── execution-model compatibility
//!        ├── technology compatibility
//!        ├── measurement compatibility
//!        ├── dynamic-circuit compatibility
//!        ├── parameterization compatibility
//!        ├── state-access compatibility
//!        ├── QEC/logical-qubit compatibility
//!        └── resource-envelope compatibility
//!        │
//!        ▼
//! execution layer / hardware backend
//! ```
//!
//! # Important architectural rule
//!
//! `benchmarking::hardware::capabilities` depends conceptually on the
//! hardware abstraction, but the existing hardware backend must NOT depend
//! on benchmarking. This preserves the dependency direction:
//!
//! ```text
//! quantum::ir
//!      ↓
//! quantum::hardware
//!      ↓
//! benchmarking
//! ```
//!
//! and prevents:
//!
//! ```text
//! hardware → benchmarking → hardware
//! ```
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! This module intentionally uses only the Rust standard library so it can be
//! completed independently before the rest of the benchmarking framework is
//! integrated.
//!
//! # Design goals
//!
//! - deterministic capability descriptions;
//! - explicit technology and execution model;
//! - explicit benchmark requirements;
//! - no hidden global state;
//! - no backend-specific vendor types;
//! - stable machine-readable compatibility errors;
//! - support for gate-model, analog, annealing, sampling, photonic and
//!   hybrid quantum systems;
//! - support for simulators and emulators;
//! - support for logical/QEC targets;
//! - forward-compatible custom capability identifiers;
//! - safe resource validation;
//! - no silent capability assumptions.

use std::collections::BTreeSet;
use std::fmt;

// =============================================================================
// Versioning
// =============================================================================

/// Version of the benchmark capability schema.
///
/// The version is intentionally separate from the Zamani compiler version.
/// Changing the meaning of a capability is a schema change and therefore
/// requires an explicit version.
pub const CAPABILITY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Backend technology
// =============================================================================

/// Physical or computational technology represented by a benchmark target.
///
/// A target may advertise more than one technology when appropriate, for
/// example a hardware emulator may expose both `Emulator` and a modeled
/// physical technology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendTechnology {
    /// Generic CPU quantum simulator.
    CpuSimulator,

    /// GPU-accelerated quantum simulator.
    GpuSimulator,

    /// State-vector simulator.
    StateVectorSimulator,

    /// Density-matrix simulator.
    DensityMatrixSimulator,

    /// Stabilizer/Clifford simulator.
    StabilizerSimulator,

    /// Tensor-network simulator.
    TensorNetworkSimulator,

    /// Generic quantum emulator.
    Emulator,

    /// Superconducting quantum processor.
    Superconducting,

    /// Trapped-ion processor.
    TrappedIon,

    /// Neutral-atom processor.
    NeutralAtom,

    /// Photonic quantum processor.
    Photonic,

    /// Semiconductor/spin quantum processor.
    Spin,

    /// Topological quantum processor.
    Topological,

    /// Quantum annealing system.
    Annealing,

    /// Analog quantum simulation system.
    Analog,

    /// Digital gate-model quantum system.
    GateModel,

    /// Hybrid quantum/classical target.
    Hybrid,

    /// Fault-tolerant logical-qubit target.
    LogicalQuantumSystem,

    /// User-defined technology.
    Custom,
}

impl BackendTechnology {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuSimulator => "cpu_simulator",
            Self::GpuSimulator => "gpu_simulator",
            Self::StateVectorSimulator => "statevector_simulator",
            Self::DensityMatrixSimulator => "density_matrix_simulator",
            Self::StabilizerSimulator => "stabilizer_simulator",
            Self::TensorNetworkSimulator => "tensor_network_simulator",
            Self::Emulator => "emulator",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::Spin => "spin",
            Self::Topological => "topological",
            Self::Annealing => "annealing",
            Self::Analog => "analog",
            Self::GateModel => "gate_model",
            Self::Hybrid => "hybrid",
            Self::LogicalQuantumSystem => "logical_quantum_system",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Execution model
// =============================================================================

/// High-level execution model.
///
/// This is intentionally broader than "gate model" because Zamani's
/// benchmarking framework is expected to cover all major quantum-computing
/// execution paradigms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionModel {
    /// Discrete gate/circuit execution.
    GateModel,

    /// Continuous/analog Hamiltonian evolution.
    Analog,

    /// Quantum annealing / adiabatic optimization.
    Annealing,

    /// Direct sampling interface.
    Sampling,

    /// Pulse-level execution.
    Pulse,

    /// Hybrid quantum/classical execution.
    Hybrid,

    /// Logical/fault-tolerant quantum execution.
    Logical,

    /// Simulator execution.
    Simulation,

    /// Emulator execution.
    Emulation,

    /// User-defined execution model.
    Custom,
}

impl ExecutionModel {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::Pulse => "pulse",
            Self::Hybrid => "hybrid",
            Self::Logical => "logical",
            Self::Simulation => "simulation",
            Self::Emulation => "emulation",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// State representation / verification capability
// =============================================================================

/// Information the target can expose to a benchmark.
///
/// This is especially important for XEB and simulation-based verification:
/// a backend that can only return samples cannot be treated as though it can
/// provide an exact state vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateAccess {
    /// No state information beyond samples.
    SamplesOnly,

    /// Exact state-vector access.
    StateVector,

    /// Exact density-matrix access.
    DensityMatrix,

    /// Stabilizer representation.
    Stabilizer,

    /// Tensor-network representation.
    TensorNetwork,

    /// Expectation values can be returned directly.
    ExpectationValues,

    /// Amplitudes can be queried.
    Amplitudes,

    /// User-defined state representation.
    Custom,
}

impl StateAccess {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SamplesOnly => "samples_only",
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Stabilizer => "stabilizer",
            Self::TensorNetwork => "tensor_network",
            Self::ExpectationValues => "expectation_values",
            Self::Amplitudes => "amplitudes",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Measurement capability
// =============================================================================

/// Supported measurement modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementMode {
    /// Computational-basis bitstring measurement.
    ComputationalBasis,

    /// General observable expectation measurement.
    ExpectationValue,

    /// Arbitrary observable measurement where supported.
    Observable,

    /// Mid-circuit measurement.
    MidCircuit,

    /// Analog measurement.
    Analog,

    /// Photon/counting measurement.
    Photonic,

    /// Syndrome measurement.
    Syndrome,

    /// User-defined measurement mode.
    Custom,
}

impl MeasurementMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComputationalBasis => "computational_basis",
            Self::ExpectationValue => "expectation_value",
            Self::Observable => "observable",
            Self::MidCircuit => "mid_circuit",
            Self::Analog => "analog",
            Self::Photonic => "photonic",
            Self::Syndrome => "syndrome",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Capability identifiers
// =============================================================================

/// Atomic benchmark capability.
///
/// A benchmark protocol declares the capabilities it requires. A target
/// advertises the capabilities it provides.
///
/// The enum contains the common standardized capabilities while
/// `Custom(String)` permits forward-compatible extension without changing
/// this file whenever a new experimental capability appears.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capability {
    // -------------------------------------------------------------------------
    // Circuit execution
    // -------------------------------------------------------------------------

    /// Gate/circuit execution.
    GateExecution,

    /// Parallel gate/layer execution.
    ParallelGateExecution,

    /// Parameterized circuit execution.
    ParameterizedCircuits,

    /// Circuit recompilation between executions.
    DynamicCompilation,

    /// Dynamic circuits.
    DynamicCircuits,

    /// Classical conditional control.
    ClassicalControl,

    /// Explicit qubit reset.
    Reset,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Measurement at circuit termination.
    Measurement,

    /// Shot-based sampling.
    Sampling,

    /// Batch execution.
    BatchExecution,

    /// Asynchronous execution.
    AsyncExecution,

    /// Job cancellation.
    JobCancellation,

    /// Partial-result retrieval.
    PartialResults,

    // -------------------------------------------------------------------------
    // State access
    // -------------------------------------------------------------------------

    /// Exact state-vector access.
    StateVector,

    /// Exact density-matrix access.
    DensityMatrix,

    /// Amplitude access.
    Amplitudes,

    /// Expectation-value access.
    ExpectationValues,

    /// Stabilizer access.
    StabilizerState,

    /// Tensor-network access.
    TensorNetworkState,

    // -------------------------------------------------------------------------
    // Physical control
    // -------------------------------------------------------------------------

    /// Pulse-level execution.
    PulseControl,

    /// Analog Hamiltonian control.
    AnalogControl,

    /// Annealing/adiabatic control.
    AnnealingControl,

    /// Custom observable support.
    ObservableMeasurement,

    // -------------------------------------------------------------------------
    // Quantum error correction
    // -------------------------------------------------------------------------

    /// Physical syndrome extraction.
    SyndromeMeasurement,

    /// Repeated syndrome cycles.
    RepeatedSyndromeCycles,

    /// Logical qubit execution.
    LogicalQubits,

    /// Logical gates.
    LogicalGates,

    /// Logical measurements.
    LogicalMeasurements,

    /// Decoder integration.
    DecoderIntegration,

    /// Fault-tolerant circuit execution.
    FaultTolerantExecution,

    // -------------------------------------------------------------------------
    // Verification / benchmarking
    // -------------------------------------------------------------------------

    /// Exact ideal-distribution evaluation.
    ExactIdealDistribution,

    /// Approximate ideal-distribution evaluation.
    ApproximateIdealDistribution,

    /// Deterministic seeded execution.
    DeterministicSeed,

    /// Backend-reported timing.
    TimingInformation,

    /// Calibration snapshot association.
    CalibrationMetadata,

    /// Backend topology metadata.
    TopologyMetadata,

    /// Native gate-set metadata.
    NativeGateMetadata,

    // -------------------------------------------------------------------------
    // Technology-specific
    // -------------------------------------------------------------------------

    /// Photon-number detection.
    PhotonNumberDetection,

    /// Bosonic/continuous-variable operations.
    ContinuousVariable,

    /// Fermionic-mode operations.
    FermionicModes,

    /// Qubit encoding.
    QubitEncoding,

    /// Qudit encoding.
    QuditEncoding,

    // -------------------------------------------------------------------------
    // Extension
    // -------------------------------------------------------------------------

    /// Forward-compatible user-defined capability.
    Custom(String),
}

impl Capability {
    /// Stable machine-readable capability identifier.
    pub fn as_str(&self) -> String {
        match self {
            Self::GateExecution => "gate_execution",
            Self::ParallelGateExecution => "parallel_gate_execution",
            Self::ParameterizedCircuits => "parameterized_circuits",
            Self::DynamicCompilation => "dynamic_compilation",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::ClassicalControl => "classical_control",
            Self::Reset => "reset",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::Measurement => "measurement",
            Self::Sampling => "sampling",
            Self::BatchExecution => "batch_execution",
            Self::AsyncExecution => "async_execution",
            Self::JobCancellation => "job_cancellation",
            Self::PartialResults => "partial_results",
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Amplitudes => "amplitudes",
            Self::ExpectationValues => "expectation_values",
            Self::StabilizerState => "stabilizer_state",
            Self::TensorNetworkState => "tensor_network_state",
            Self::PulseControl => "pulse_control",
            Self::AnalogControl => "analog_control",
            Self::AnnealingControl => "annealing_control",
            Self::ObservableMeasurement => "observable_measurement",
            Self::SyndromeMeasurement => "syndrome_measurement",
            Self::RepeatedSyndromeCycles => "repeated_syndrome_cycles",
            Self::LogicalQubits => "logical_qubits",
            Self::LogicalGates => "logical_gates",
            Self::LogicalMeasurements => "logical_measurements",
            Self::DecoderIntegration => "decoder_integration",
            Self::FaultTolerantExecution => "fault_tolerant_execution",
            Self::ExactIdealDistribution => "exact_ideal_distribution",
            Self::ApproximateIdealDistribution => "approximate_ideal_distribution",
            Self::DeterministicSeed => "deterministic_seed",
            Self::TimingInformation => "timing_information",
            Self::CalibrationMetadata => "calibration_metadata",
            Self::TopologyMetadata => "topology_metadata",
            Self::NativeGateMetadata => "native_gate_metadata",
            Self::PhotonNumberDetection => "photon_number_detection",
            Self::ContinuousVariable => "continuous_variable",
            Self::FermionicModes => "fermionic_modes",
            Self::QubitEncoding => "qubit_encoding",
            Self::QuditEncoding => "qudit_encoding",
            Self::Custom(value) => format!("custom:{value}"),
        }
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Benchmark-visible resource envelope.
///
/// A capability profile says what a backend can do; this structure says how
/// much of it the backend can do.
///
/// `None` means that the target does not advertise a finite limit.
///
/// Zero is therefore a valid finite limit and is never interpreted as
/// "unlimited".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum physical qubits/modes.
    pub max_qubits: Option<u64>,

    /// Maximum logical qubits.
    pub max_logical_qubits: Option<u64>,

    /// Maximum circuit depth.
    pub max_circuit_depth: Option<u64>,

    /// Maximum operation count per circuit.
    pub max_operations: Option<u64>,

    /// Maximum shots per circuit/job.
    pub max_shots: Option<u64>,

    /// Maximum circuits in one batch.
    pub max_circuits_per_batch: Option<u64>,

    /// Maximum simultaneous circuits/workloads.
    pub max_concurrent_jobs: Option<u64>,

    /// Maximum number of classical bits.
    pub max_classical_bits: Option<u64>,

    /// Maximum supported QEC syndrome rounds.
    pub max_syndrome_rounds: Option<u64>,
}

impl ResourceLimits {
    /// Creates an unconstrained resource envelope.
    pub const fn unlimited() -> Self {
        Self {
            max_qubits: None,
            max_logical_qubits: None,
            max_circuit_depth: None,
            max_operations: None,
            max_shots: None,
            max_circuits_per_batch: None,
            max_concurrent_jobs: None,
            max_classical_bits: None,
            max_syndrome_rounds: None,
        }
    }

    /// Tests an optional maximum.
    fn allows(limit: Option<u64>, requested: u64) -> bool {
        match limit {
            Some(maximum) => requested <= maximum,
            None => true,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Benchmark requirements
// =============================================================================

/// Requirements declared by a benchmark workload.
///
/// Protocol implementations should construct this object before execution
/// and ask a `CapabilityProfile` to validate it.
///
/// The structure deliberately contains no execution implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkRequirements {
    /// Required execution models.
    pub execution_models: BTreeSet<ExecutionModel>,

    /// Accepted technologies. Empty means technology-neutral.
    pub technologies: BTreeSet<BackendTechnology>,

    /// Required atomic capabilities.
    pub capabilities: BTreeSet<Capability>,

    /// Required measurement modes.
    pub measurement_modes: BTreeSet<MeasurementMode>,

    /// Required state-access modes.
    pub state_access: BTreeSet<StateAccess>,

    /// Minimum number of physical qubits/modes.
    pub min_qubits: Option<u64>,

    /// Minimum number of logical qubits.
    pub min_logical_qubits: Option<u64>,

    /// Maximum acceptable circuit depth.
    pub required_max_depth: Option<u64>,

    /// Minimum number of supported shots.
    pub min_shots: Option<u64>,

    /// Minimum number of circuits that can be submitted in a batch.
    pub min_batch_size: Option<u64>,

    /// Whether topology metadata is required.
    pub requires_topology: bool,

    /// Whether calibration metadata is required.
    pub requires_calibration: bool,

    /// Whether deterministic seeded generation/execution is required.
    pub requires_deterministic_seed: bool,

    /// Whether the benchmark needs timing data.
    pub requires_timing: bool,

    /// Optional human-readable benchmark name.
    pub benchmark_id: String,
}

impl BenchmarkRequirements {
    /// Creates technology-neutral requirements for a benchmark.
    pub fn new(benchmark_id: impl Into<String>) -> Self {
        Self {
            execution_models: BTreeSet::new(),
            technologies: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            measurement_modes: BTreeSet::new(),
            state_access: BTreeSet::new(),
            min_qubits: None,
            min_logical_qubits: None,
            required_max_depth: None,
            min_shots: None,
            min_batch_size: None,
            requires_topology: false,
            requires_calibration: false,
            requires_deterministic_seed: false,
            requires_timing: false,
            benchmark_id: benchmark_id.into(),
        }
    }

    /// Requires an execution model.
    pub fn require_execution_model(
        mut self,
        model: ExecutionModel,
    ) -> Self {
        self.execution_models.insert(model);
        self
    }

    /// Adds an accepted technology.
    pub fn accept_technology(
        mut self,
        technology: BackendTechnology,
    ) -> Self {
        self.technologies.insert(technology);
        self
    }

    /// Requires an atomic capability.
    pub fn require_capability(
        mut self,
        capability: Capability,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Requires a measurement mode.
    pub fn require_measurement(
        mut self,
        mode: MeasurementMode,
    ) -> Self {
        self.measurement_modes.insert(mode);
        self
    }

    /// Requires a state-access mode.
    pub fn require_state_access(
        mut self,
        access: StateAccess,
    ) -> Self {
        self.state_access.insert(access);
        self
    }

    /// Requires at least this many qubits.
    pub fn with_min_qubits(mut self, count: u64) -> Self {
        self.min_qubits = Some(count);
        self
    }

    /// Requires at least this many logical qubits.
    pub fn with_min_logical_qubits(mut self, count: u64) -> Self {
        self.min_logical_qubits = Some(count);
        self
    }

    /// Requires the target to support at least this circuit depth.
    pub fn with_required_max_depth(mut self, depth: u64) -> Self {
        self.required_max_depth = Some(depth);
        self
    }

    /// Requires at least this number of shots.
    pub fn with_min_shots(mut self, shots: u64) -> Self {
        self.min_shots = Some(shots);
        self
    }

    /// Requires at least this batch size.
    pub fn with_min_batch_size(mut self, size: u64) -> Self {
        self.min_batch_size = Some(size);
        self
    }

    /// Requires topology metadata.
    pub fn requiring_topology(mut self) -> Self {
        self.requires_topology = true;
        self
    }

    /// Requires calibration metadata.
    pub fn requiring_calibration(mut self) -> Self {
        self.requires_calibration = true;
        self
    }

    /// Requires deterministic seed support.
    pub fn requiring_deterministic_seed(mut self) -> Self {
        self.requires_deterministic_seed = true;
        self
    }

    /// Requires timing information.
    pub fn requiring_timing(mut self) -> Self {
        self.requires_timing = true;
        self
    }

    /// Returns true when no technology restriction was specified.
    pub fn is_technology_neutral(&self) -> bool {
        self.technologies.is_empty()
    }
}

impl Default for BenchmarkRequirements {
    fn default() -> Self {
        Self::new("unnamed")
    }
}

// =============================================================================
// Capability profile
// =============================================================================

/// Complete benchmark-facing description of a target backend.
///
/// This is intentionally a value object. A provider can create it from its
/// own backend implementation without this module knowing anything about
/// provider APIs.
///
/// The profile is safe to clone and can therefore be embedded in benchmark
/// provenance and experiment plans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityProfile {
    /// Capability schema version.
    pub schema_version: u16,

    /// Stable backend identifier.
    pub backend_id: String,

    /// Human-readable backend name.
    pub backend_name: String,

    /// Provider identifier.
    pub provider_id: String,

    /// Backend technology.
    pub technology: BackendTechnology,

    /// Supported execution models.
    pub execution_models: BTreeSet<ExecutionModel>,

    /// Supported atomic capabilities.
    pub capabilities: BTreeSet<Capability>,

    /// Supported measurement modes.
    pub measurement_modes: BTreeSet<MeasurementMode>,

    /// State information available to benchmarks.
    pub state_access: BTreeSet<StateAccess>,

    /// Benchmark-visible resource envelope.
    pub limits: ResourceLimits,

    /// Physical qubit count, when known.
    pub qubit_count: Option<u64>,

    /// Logical qubit count, when known.
    pub logical_qubit_count: Option<u64>,

    /// Whether topology metadata is available.
    pub has_topology: bool,

    /// Whether calibration metadata is available.
    pub has_calibration: bool,

    /// Whether the backend can produce stable timing information.
    pub has_timing: bool,

    /// Optional native gate identifiers.
    pub native_gates: BTreeSet<String>,

    /// Optional custom capability tags.
    pub custom_tags: BTreeSet<String>,
}

impl CapabilityProfile {
    /// Creates an empty profile for a backend.
    pub fn new(
        backend_id: impl Into<String>,
        backend_name: impl Into<String>,
        provider_id: impl Into<String>,
        technology: BackendTechnology,
    ) -> Self {
        Self {
            schema_version: CAPABILITY_SCHEMA_VERSION,
            backend_id: backend_id.into(),
            backend_name: backend_name.into(),
            provider_id: provider_id.into(),
            technology,
            execution_models: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            measurement_modes: BTreeSet::new(),
            state_access: BTreeSet::new(),
            limits: ResourceLimits::unlimited(),
            qubit_count: None,
            logical_qubit_count: None,
            has_topology: false,
            has_calibration: false,
            has_timing: false,
            native_gates: BTreeSet::new(),
            custom_tags: BTreeSet::new(),
        }
    }

    /// Adds an execution model.
    pub fn with_execution_model(
        mut self,
        model: ExecutionModel,
    ) -> Self {
        self.execution_models.insert(model);
        self
    }

    /// Adds a capability.
    pub fn with_capability(
        mut self,
        capability: Capability,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds multiple capabilities.
    pub fn with_capabilities<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        self.capabilities.extend(capabilities);
        self
    }

    /// Adds a measurement mode.
    pub fn with_measurement_mode(
        mut self,
        mode: MeasurementMode,
    ) -> Self {
        self.measurement_modes.insert(mode);
        self
    }

    /// Adds a state-access mode.
    pub fn with_state_access(
        mut self,
        access: StateAccess,
    ) -> Self {
        self.state_access.insert(access);
        self
    }

    /// Sets resource limits.
    pub fn with_limits(
        mut self,
        limits: ResourceLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Sets physical qubit count.
    pub fn with_qubit_count(
        mut self,
        count: u64,
    ) -> Self {
        self.qubit_count = Some(count);
        self
    }

    /// Sets logical qubit count.
    pub fn with_logical_qubit_count(
        mut self,
        count: u64,
    ) -> Self {
        self.logical_qubit_count = Some(count);
        self
    }

    /// Marks topology metadata as available.
    pub fn with_topology(
        mut self,
        available: bool,
    ) -> Self {
        self.has_topology = available;
        self
    }

    /// Marks calibration metadata as available.
    pub fn with_calibration(
        mut self,
        available: bool,
    ) -> Self {
        self.has_calibration = available;
        self
    }

    /// Marks timing information as available.
    pub fn with_timing(
        mut self,
        available: bool,
    ) -> Self {
        self.has_timing = available;
        self
    }

    /// Adds a native gate.
    pub fn with_native_gate(
        mut self,
        gate: impl Into<String>,
    ) -> Self {
        let normalized = normalize_gate_name(&gate.into());

        if !normalized.is_empty() {
            self.native_gates.insert(normalized);
        }

        self
    }

    /// Adds a custom tag.
    pub fn with_custom_tag(
        mut self,
        tag: impl Into<String>,
    ) -> Self {
        let tag = tag.into();

        if !tag.is_empty() {
            self.custom_tags.insert(tag);
        }

        self
    }

    /// Tests a single atomic capability.
    pub fn supports(
        &self,
        capability: &Capability,
    ) -> bool {
        self.capabilities.contains(capability)
    }

    /// Tests an execution model.
    pub fn supports_execution_model(
        &self,
        model: ExecutionModel,
    ) -> bool {
        self.execution_models.contains(&model)
    }

    /// Tests a measurement mode.
    pub fn supports_measurement(
        &self,
        mode: MeasurementMode,
    ) -> bool {
        self.measurement_modes.contains(&mode)
    }

    /// Tests a state-access mode.
    pub fn supports_state_access(
        &self,
        access: StateAccess,
    ) -> bool {
        self.state_access.contains(&access)
    }

    /// Tests native gate support.
    pub fn supports_native_gate(
        &self,
        gate: &str,
    ) -> bool {
        self.native_gates.contains(
            &normalize_gate_name(gate),
        )
    }

    /// Returns all missing capabilities for a benchmark.
    pub fn missing_capabilities(
        &self,
        requirements: &BenchmarkRequirements,
    ) -> Vec<Capability> {
        requirements
            .capabilities
            .iter()
            .filter(|capability| !self.supports(capability))
            .cloned()
            .collect()
    }

    /// Returns all missing execution models.
    pub fn missing_execution_models(
        &self,
        requirements: &BenchmarkRequirements,
    ) -> Vec<ExecutionModel> {
        requirements
            .execution_models
            .iter()
            .filter(|model| !self.supports_execution_model(**model))
            .copied()
            .collect()
    }

    /// Returns all missing measurement modes.
    pub fn missing_measurement_modes(
        &self,
        requirements: &BenchmarkRequirements,
    ) -> Vec<MeasurementMode> {
        requirements
            .measurement_modes
            .iter()
            .filter(|mode| !self.supports_measurement(**mode))
            .copied()
            .collect()
    }

    /// Returns all missing state-access modes.
    pub fn missing_state_access(
        &self,
        requirements: &BenchmarkRequirements,
    ) -> Vec<StateAccess> {
        requirements
            .state_access
            .iter()
            .filter(|access| !self.supports_state_access(**access))
            .copied()
            .collect()
    }

    /// Performs complete benchmark compatibility validation.
    ///
    /// No execution is attempted.
    pub fn validate(
        &self,
        requirements: &BenchmarkRequirements,
    ) -> Result<CompatibilityReport, CapabilityError> {
        validate_profile(self)?;

        let mut failures = Vec::new();

        // ---------------------------------------------------------------------
        // Execution model
        // ---------------------------------------------------------------------

        for model in &requirements.execution_models {
            if !self.supports_execution_model(*model) {
                failures.push(CompatibilityFailure::MissingExecutionModel {
                    required: *model,
                });
            }
        }

        // ---------------------------------------------------------------------
        // Technology
        // ---------------------------------------------------------------------

        if !requirements.technologies.is_empty()
            && !requirements.technologies.contains(&self.technology)
        {
            failures.push(
                CompatibilityFailure::UnsupportedTechnology {
                    required: requirements.technologies.clone(),
                    actual: self.technology,
                },
            );
        }

        // ---------------------------------------------------------------------
        // Atomic capabilities
        // ---------------------------------------------------------------------

        for capability in &requirements.capabilities {
            if !self.supports(capability) {
                failures.push(
                    CompatibilityFailure::MissingCapability {
                        capability: capability.clone(),
                    },
                );
            }
        }

        // ---------------------------------------------------------------------
        // Measurement
        // ---------------------------------------------------------------------

        for mode in &requirements.measurement_modes {
            if !self.supports_measurement(*mode) {
                failures.push(
                    CompatibilityFailure::MissingMeasurementMode {
                        mode: *mode,
                    },
                );
            }
        }

        // ---------------------------------------------------------------------
        // State access
        // ---------------------------------------------------------------------

        for access in &requirements.state_access {
            if !self.supports_state_access(*access) {
                failures.push(
                    CompatibilityFailure::MissingStateAccess {
                        access: *access,
                    },
                );
            }
        }

        // ---------------------------------------------------------------------
        // Physical qubits
        // ---------------------------------------------------------------------

        if let Some(required) = requirements.min_qubits {
            match self.qubit_count {
                Some(actual) if actual >= required => {}
                Some(actual) => {
                    failures.push(
                        CompatibilityFailure::InsufficientQubits {
                            required,
                            available: actual,
                        },
                    );
                }
                None => {
                    failures.push(
                        CompatibilityFailure::UnknownResource {
                            resource: ResourceKind::PhysicalQubits,
                        },
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Logical qubits
        // ---------------------------------------------------------------------

        if let Some(required) = requirements.min_logical_qubits {
            match self.logical_qubit_count {
                Some(actual) if actual >= required => {}
                Some(actual) => {
                    failures.push(
                        CompatibilityFailure::InsufficientLogicalQubits {
                            required,
                            available: actual,
                        },
                    );
                }
                None => {
                    failures.push(
                        CompatibilityFailure::UnknownResource {
                            resource: ResourceKind::LogicalQubits,
                        },
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Resource limits
        // ---------------------------------------------------------------------

        if let Some(required_depth) =
            requirements.required_max_depth
        {
            if !ResourceLimits::allows(
                self.limits.max_circuit_depth,
                required_depth,
            ) {
                failures.push(
                    CompatibilityFailure::CircuitDepthLimitExceeded {
                        required: required_depth,
                        maximum: self
                            .limits
                            .max_circuit_depth
                            .unwrap_or(0),
                    },
                );
            }
        }

        if let Some(required_shots) =
            requirements.min_shots
        {
            if !ResourceLimits::allows(
                self.limits.max_shots,
                required_shots,
            ) {
                failures.push(
                    CompatibilityFailure::ShotLimitTooSmall {
                        required: required_shots,
                        maximum: self
                            .limits
                            .max_shots
                            .unwrap_or(0),
                    },
                );
            }
        }

        if let Some(required_batch) =
            requirements.min_batch_size
        {
            if !ResourceLimits::allows(
                self.limits.max_circuits_per_batch,
                required_batch,
            ) {
                failures.push(
                    CompatibilityFailure::BatchLimitTooSmall {
                        required: required_batch,
                        maximum: self
                            .limits
                            .max_circuits_per_batch
                            .unwrap_or(0),
                    },
                );
            }
        }

        // ---------------------------------------------------------------------
        // Metadata
        // ---------------------------------------------------------------------

        if requirements.requires_topology
            && !self.has_topology
        {
            failures.push(
                CompatibilityFailure::MissingTopologyMetadata,
            );
        }

        if requirements.requires_calibration
            && !self.has_calibration
        {
            failures.push(
                CompatibilityFailure::MissingCalibrationMetadata,
            );
        }

        if requirements.requires_deterministic_seed
            && !self.supports(
                &Capability::DeterministicSeed,
            )
        {
            failures.push(
                CompatibilityFailure::MissingCapability {
                    capability: Capability::DeterministicSeed,
                },
            );
        }

        if requirements.requires_timing
            && !self.has_timing
        {
            failures.push(
                CompatibilityFailure::MissingTimingInformation,
            );
        }

        Ok(CompatibilityReport {
            benchmark_id: requirements.benchmark_id.clone(),
            backend_id: self.backend_id.clone(),
            compatible: failures.is_empty(),
            failures,
        })
    }
}

// =============================================================================
// Compatibility failure model
// =============================================================================

/// Resource categories used in compatibility diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind {
    PhysicalQubits,
    LogicalQubits,
    CircuitDepth,
    Operations,
    Shots,
    BatchSize,
    ConcurrentJobs,
    ClassicalBits,
    SyndromeRounds,
}

impl ResourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalQubits => "physical_qubits",
            Self::LogicalQubits => "logical_qubits",
            Self::CircuitDepth => "circuit_depth",
            Self::Operations => "operations",
            Self::Shots => "shots",
            Self::BatchSize => "batch_size",
            Self::ConcurrentJobs => "concurrent_jobs",
            Self::ClassicalBits => "classical_bits",
            Self::SyndromeRounds => "syndrome_rounds",
        }
    }
}

/// Exact reason a benchmark cannot be guaranteed compatible with a target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityFailure {
    MissingExecutionModel {
        required: ExecutionModel,
    },

    UnsupportedTechnology {
        required: BTreeSet<BackendTechnology>,
        actual: BackendTechnology,
    },

    MissingCapability {
        capability: Capability,
    },

    MissingMeasurementMode {
        mode: MeasurementMode,
    },

    MissingStateAccess {
        access: StateAccess,
    },

    InsufficientQubits {
        required: u64,
        available: u64,
    },

    InsufficientLogicalQubits {
        required: u64,
        available: u64,
    },

    UnknownResource {
        resource: ResourceKind,
    },

    CircuitDepthLimitExceeded {
        required: u64,
        maximum: u64,
    },

    ShotLimitTooSmall {
        required: u64,
        maximum: u64,
    },

    BatchLimitTooSmall {
        required: u64,
        maximum: u64,
    },

    MissingTopologyMetadata,

    MissingCalibrationMetadata,

    MissingTimingInformation,

    InvalidProfile {
        reason: String,
    },
}

impl CompatibilityFailure {
    /// Stable machine-readable identifier.
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingExecutionModel { .. } =>
                "missing_execution_model",

            Self::UnsupportedTechnology { .. } =>
                "unsupported_technology",

            Self::MissingCapability { .. } =>
                "missing_capability",

            Self::MissingMeasurementMode { .. } =>
                "missing_measurement_mode",

            Self::MissingStateAccess { .. } =>
                "missing_state_access",

            Self::InsufficientQubits { .. } =>
                "insufficient_qubits",

            Self::InsufficientLogicalQubits { .. } =>
                "insufficient_logical_qubits",

            Self::UnknownResource { .. } =>
                "unknown_resource",

            Self::CircuitDepthLimitExceeded { .. } =>
                "circuit_depth_limit_exceeded",

            Self::ShotLimitTooSmall { .. } =>
                "shot_limit_too_small",

            Self::BatchLimitTooSmall { .. } =>
                "batch_limit_too_small",

            Self::MissingTopologyMetadata =>
                "missing_topology_metadata",

            Self::MissingCalibrationMetadata =>
                "missing_calibration_metadata",

            Self::MissingTimingInformation =>
                "missing_timing_information",

            Self::InvalidProfile { .. } =>
                "invalid_profile",
        }
    }
}

impl fmt::Display for CompatibilityFailure {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingExecutionModel { required } => {
                write!(
                    formatter,
                    "required execution model `{}` is unsupported",
                    required.as_str()
                )
            }

            Self::UnsupportedTechnology {
                required,
                actual,
            } => {
                write!(
                    formatter,
                    "backend technology `{}` is not one of the required technologies: ",
                    actual.as_str()
                )?;

                let mut first = true;

                for technology in required {
                    if !first {
                        write!(formatter, ", ")?;
                    }

                    first = false;
                    write!(formatter, "{}", technology.as_str())?;
                }

                Ok(())
            }

            Self::MissingCapability { capability } => {
                write!(
                    formatter,
                    "required capability `{}` is unsupported",
                    capability.as_str()
                )
            }

            Self::MissingMeasurementMode { mode } => {
                write!(
                    formatter,
                    "required measurement mode `{}` is unsupported",
                    mode.as_str()
                )
            }

            Self::MissingStateAccess { access } => {
                write!(
                    formatter,
                    "required state access `{}` is unavailable",
                    access.as_str()
                )
            }

            Self::InsufficientQubits {
                required,
                available,
            } => {
                write!(
                    formatter,
                    "benchmark requires {} physical qubits but only {} are available",
                    required,
                    available
                )
            }

            Self::InsufficientLogicalQubits {
                required,
                available,
            } => {
                write!(
                    formatter,
                    "benchmark requires {} logical qubits but only {} are available",
                    required,
                    available
                )
            }

            Self::UnknownResource { resource } => {
                write!(
                    formatter,
                    "resource `{}` is required but the backend does not advertise its capacity",
                    resource.as_str()
                )
            }

            Self::CircuitDepthLimitExceeded {
                required,
                maximum,
            } => {
                write!(
                    formatter,
                    "benchmark requires circuit depth {} but backend advertises maximum depth {}",
                    required,
                    maximum
                )
            }

            Self::ShotLimitTooSmall {
                required,
                maximum,
            } => {
                write!(
                    formatter,
                    "benchmark requires {} shots but backend advertises maximum {} shots",
                    required,
                    maximum
                )
            }

            Self::BatchLimitTooSmall {
                required,
                maximum,
            } => {
                write!(
                    formatter,
                    "benchmark requires batch size {} but backend advertises maximum {} circuits",
                    required,
                    maximum
                )
            }

            Self::MissingTopologyMetadata => {
                write!(
                    formatter,
                    "benchmark requires topology metadata"
                )
            }

            Self::MissingCalibrationMetadata => {
                write!(
                    formatter,
                    "benchmark requires calibration metadata"
                )
            }

            Self::MissingTimingInformation => {
                write!(
                    formatter,
                    "benchmark requires backend timing information"
                )
            }

            Self::InvalidProfile { reason } => {
                write!(
                    formatter,
                    "invalid capability profile: {}",
                    reason
                )
            }
        }
    }
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Result of capability negotiation.
///
/// A report is returned even when the benchmark is incompatible so callers
/// can inspect every failure instead of receiving only the first error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub benchmark_id: String,
    pub backend_id: String,
    pub compatible: bool,
    pub failures: Vec<CompatibilityFailure>,
}

impl CompatibilityReport {
    /// Creates a successful report.
    pub fn compatible(
        benchmark_id: impl Into<String>,
        backend_id: impl Into<String>,
    ) -> Self {
        Self {
            benchmark_id: benchmark_id.into(),
            backend_id: backend_id.into(),
            compatible: true,
            failures: Vec::new(),
        }
    }

    /// Returns true when no incompatibility was found.
    pub fn is_compatible(&self) -> bool {
        self.compatible
    }

    /// Returns the number of compatibility failures.
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }

    /// Returns all stable failure codes.
    pub fn failure_codes(&self) -> Vec<&'static str> {
        self.failures
            .iter()
            .map(CompatibilityFailure::code)
            .collect()
    }

    /// Converts the report into a hard error when incompatible.
    pub fn into_result(
        self,
    ) -> Result<Self, CapabilityError> {
        if self.compatible {
            Ok(self)
        } else {
            Err(CapabilityError::Incompatible {
                report: self,
            })
        }
    }
}

// =============================================================================
// Capability errors
// =============================================================================

/// Errors originating from capability-profile construction or negotiation.
///
/// This type is local to the capability layer by design. The future
/// `benchmarking::core::errors` module can wrap it without requiring this
/// file to be rewritten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    InvalidProfile {
        reason: String,
    },

    EmptyBackendId,

    EmptyBackendName,

    EmptyProviderId,

    UnsupportedSchemaVersion {
        version: u16,
    },

    Incompatible {
        report: CompatibilityReport,
    },
}

impl fmt::Display for CapabilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidProfile { reason } => {
                write!(
                    formatter,
                    "invalid capability profile: {}",
                    reason
                )
            }

            Self::EmptyBackendId => {
                write!(
                    formatter,
                    "backend identifier cannot be empty"
                )
            }

            Self::EmptyBackendName => {
                write!(
                    formatter,
                    "backend name cannot be empty"
                )
            }

            Self::EmptyProviderId => {
                write!(
                    formatter,
                    "provider identifier cannot be empty"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported capability schema version {}",
                    version
                )
            }

            Self::Incompatible { report } => {
                write!(
                    formatter,
                    "benchmark `{}` is incompatible with backend `{}`",
                    report.benchmark_id,
                    report.backend_id
                )
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

// =============================================================================
// Profile validation
// =============================================================================

/// Validates the structural integrity of a capability profile.
///
/// This is deliberately separate from benchmark compatibility validation:
/// a malformed backend description is different from a valid backend that
/// simply cannot run a particular benchmark.
pub fn validate_profile(
    profile: &CapabilityProfile,
) -> Result<(), CapabilityError> {
    if profile.schema_version != CAPABILITY_SCHEMA_VERSION {
        return Err(
            CapabilityError::UnsupportedSchemaVersion {
                version: profile.schema_version,
            },
        );
    }

    if profile.backend_id.trim().is_empty() {
        return Err(CapabilityError::EmptyBackendId);
    }

    if profile.backend_name.trim().is_empty() {
        return Err(CapabilityError::EmptyBackendName);
    }

    if profile.provider_id.trim().is_empty() {
        return Err(CapabilityError::EmptyProviderId);
    }

    // Logical qubits cannot be advertised as available without the logical
    // execution capability.
    if profile.logical_qubit_count.is_some()
        && !profile.supports(
            &Capability::LogicalQubits,
        )
    {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "logical qubit count is advertised without LogicalQubits capability"
                        .to_string(),
            },
        );
    }

    // Logical operations require logical qubits.
    if profile.supports(&Capability::LogicalGates)
        && !profile.supports(&Capability::LogicalQubits)
    {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "LogicalGates requires LogicalQubits capability"
                        .to_string(),
            },
        );
    }

    // Logical measurements require logical qubits.
    if profile.supports(
        &Capability::LogicalMeasurements,
    ) && !profile.supports(
        &Capability::LogicalQubits,
    ) {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "LogicalMeasurements requires LogicalQubits capability"
                        .to_string(),
            },
        );
    }

    // Dynamic circuits necessarily require the underlying ability to execute
    // gates and perform mid-circuit control/measurement.
    if profile.supports(&Capability::DynamicCircuits)
        && !profile.supports(&Capability::GateExecution)
    {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "DynamicCircuits requires GateExecution capability"
                        .to_string(),
            },
        );
    }

    // Exact ideal distributions are verification capabilities, not ordinary
    // hardware capabilities. They should only be advertised by a target that
    // has some state/distribution access mechanism.
    if profile.supports(
        &Capability::ExactIdealDistribution,
    ) && profile.state_access.is_empty()
    {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "ExactIdealDistribution requires at least one state-access mode"
                        .to_string(),
            },
        );
    }

    // Measurement capability must correspond to at least one measurement mode.
    if profile.supports(&Capability::Measurement)
        && profile.measurement_modes.is_empty()
    {
        return Err(
            CapabilityError::InvalidProfile {
                reason:
                    "Measurement capability requires at least one measurement mode"
                        .to_string(),
            },
        );
    }

    // Topology/calibration/timing are metadata declarations and can safely
    // coexist with any execution model. No artificial restrictions are added.
    Ok(())
}

// =============================================================================
// Capability builders
// =============================================================================

/// Builder for a capability profile.
///
/// The builder exists primarily to make backend adapters explicit and
/// deterministic. It does not contain provider-specific logic.
#[derive(Debug, Clone)]
pub struct CapabilityProfileBuilder {
    profile: CapabilityProfile,
}

impl CapabilityProfileBuilder {
    /// Creates a builder.
    pub fn new(
        backend_id: impl Into<String>,
        backend_name: impl Into<String>,
        provider_id: impl Into<String>,
        technology: BackendTechnology,
    ) -> Self {
        Self {
            profile: CapabilityProfile::new(
                backend_id,
                backend_name,
                provider_id,
                technology,
            ),
        }
    }

    pub fn execution_model(
        mut self,
        model: ExecutionModel,
    ) -> Self {
        self.profile.execution_models.insert(model);
        self
    }

    pub fn capability(
        mut self,
        capability: Capability,
    ) -> Self {
        self.profile.capabilities.insert(capability);
        self
    }

    pub fn measurement_mode(
        mut self,
        mode: MeasurementMode,
    ) -> Self {
        self.profile.measurement_modes.insert(mode);
        self
    }

    pub fn state_access(
        mut self,
        access: StateAccess,
    ) -> Self {
        self.profile.state_access.insert(access);
        self
    }

    pub fn limits(
        mut self,
        limits: ResourceLimits,
    ) -> Self {
        self.profile.limits = limits;
        self
    }

    pub fn qubit_count(
        mut self,
        count: u64,
    ) -> Self {
        self.profile.qubit_count = Some(count);
        self
    }

    pub fn logical_qubit_count(
        mut self,
        count: u64,
    ) -> Self {
        self.profile.logical_qubit_count = Some(count);
        self
    }

    pub fn topology(
        mut self,
        available: bool,
    ) -> Self {
        self.profile.has_topology = available;
        self
    }

    pub fn calibration(
        mut self,
        available: bool,
    ) -> Self {
        self.profile.has_calibration = available;
        self
    }

    pub fn timing(
        mut self,
        available: bool,
    ) -> Self {
        self.profile.has_timing = available;
        self
    }

    pub fn native_gate(
        mut self,
        gate: impl Into<String>,
    ) -> Self {
        let normalized =
            normalize_gate_name(&gate.into());

        if !normalized.is_empty() {
            self.profile.native_gates.insert(normalized);
        }

        self
    }

    pub fn custom_tag(
        mut self,
        tag: impl Into<String>,
    ) -> Self {
        let tag = tag.into();

        if !tag.is_empty() {
            self.profile.custom_tags.insert(tag);
        }

        self
    }

    /// Validates and returns the finished profile.
    pub fn build(
        self,
    ) -> Result<CapabilityProfile, CapabilityError> {
        validate_profile(&self.profile)?;

        Ok(self.profile)
    }
}

// =============================================================================
// Standard profiles
// =============================================================================

/// Creates a conservative generic state-vector simulator profile.
pub fn state_vector_simulator_profile(
    backend_id: impl Into<String>,
    backend_name: impl Into<String>,
    provider_id: impl Into<String>,
    qubit_count: u64,
) -> Result<CapabilityProfile, CapabilityError> {
    CapabilityProfileBuilder::new(
        backend_id,
        backend_name,
        provider_id,
        BackendTechnology::StateVectorSimulator,
    )
    .execution_model(ExecutionModel::Simulation)
    .execution_model(ExecutionModel::GateModel)
    .capability(Capability::GateExecution)
    .capability(Capability::ParallelGateExecution)
    .capability(Capability::ParameterizedCircuits)
    .capability(Capability::DynamicCircuits)
    .capability(Capability::ClassicalControl)
    .capability(Capability::Reset)
    .capability(Capability::MidCircuitMeasurement)
    .capability(Capability::Measurement)
    .capability(Capability::Sampling)
    .capability(Capability::BatchExecution)
    .capability(Capability::DeterministicSeed)
    .capability(Capability::TimingInformation)
    .capability(Capability::TopologyMetadata)
    .capability(Capability::NativeGateMetadata)
    .capability(Capability::ExactIdealDistribution)
    .capability(Capability::StateVector)
    .capability(Capability::Amplitudes)
    .capability(Capability::ExpectationValues)
    .measurement_mode(MeasurementMode::ComputationalBasis)
    .measurement_mode(MeasurementMode::ExpectationValue)
    .measurement_mode(MeasurementMode::Observable)
    .measurement_mode(MeasurementMode::MidCircuit)
    .state_access(StateAccess::StateVector)
    .state_access(StateAccess::Amplitudes)
    .state_access(StateAccess::ExpectationValues)
    .qubit_count(qubit_count)
    .topology(true)
    .timing(true)
    .build()
}

/// Creates a conservative generic density-matrix simulator profile.
pub fn density_matrix_simulator_profile(
    backend_id: impl Into<String>,
    backend_name: impl Into<String>,
    provider_id: impl Into<String>,
    qubit_count: u64,
) -> Result<CapabilityProfile, CapabilityError> {
    CapabilityProfileBuilder::new(
        backend_id,
        backend_name,
        provider_id,
        BackendTechnology::DensityMatrixSimulator,
    )
    .execution_model(ExecutionModel::Simulation)
    .execution_model(ExecutionModel::GateModel)
    .capability(Capability::GateExecution)
    .capability(Capability::ParallelGateExecution)
    .capability(Capability::ParameterizedCircuits)
    .capability(Capability::Reset)
    .capability(Capability::MidCircuitMeasurement)
    .capability(Capability::Measurement)
    .capability(Capability::Sampling)
    .capability(Capability::BatchExecution)
    .capability(Capability::DeterministicSeed)
    .capability(Capability::TimingInformation)
    .capability(Capability::ExactIdealDistribution)
    .capability(Capability::DensityMatrix)
    .capability(Capability::ExpectationValues)
    .measurement_mode(MeasurementMode::ComputationalBasis)
    .measurement_mode(MeasurementMode::ExpectationValue)
    .measurement_mode(MeasurementMode::Observable)
    .measurement_mode(MeasurementMode::MidCircuit)
    .state_access(StateAccess::DensityMatrix)
    .state_access(StateAccess::ExpectationValues)
    .qubit_count(qubit_count)
    .timing(true)
    .build()
}

// =============================================================================
// Gate normalization
// =============================================================================

/// Normalizes a gate name for capability comparison.
///
/// This is intentionally conservative:
///
/// - surrounding whitespace is removed;
/// - ASCII case is normalized;
/// - internal spelling is otherwise preserved.
///
/// Gate aliases/decomposition semantics belong to the compiler/IR layer,
/// not this capability layer.
pub fn normalize_gate_name(
    gate: &str,
) -> String {
    gate.trim().to_ascii_lowercase()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn simulator() -> CapabilityProfile {
        state_vector_simulator_profile(
            "local-statevector",
            "Local State Vector",
            "zamani",
            32,
        )
        .unwrap()
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            CAPABILITY_SCHEMA_VERSION,
            1
        );
    }

    #[test]
    fn capability_names_are_stable() {
        assert_eq!(
            Capability::GateExecution.as_str(),
            "gate_execution"
        );

        assert_eq!(
            Capability::LogicalQubits.as_str(),
            "logical_qubits"
        );

        assert_eq!(
            Capability::Custom("foo".to_string()).as_str(),
            "custom:foo"
        );
    }

    #[test]
    fn technology_names_are_stable() {
        assert_eq!(
            BackendTechnology::Superconducting.as_str(),
            "superconducting"
        );

        assert_eq!(
            BackendTechnology::NeutralAtom.as_str(),
            "neutral_atom"
        );
    }

    #[test]
    fn execution_model_names_are_stable() {
        assert_eq!(
            ExecutionModel::GateModel.as_str(),
            "gate_model"
        );

        assert_eq!(
            ExecutionModel::Annealing.as_str(),
            "annealing"
        );
    }

    #[test]
    fn standard_statevector_profile_is_valid() {
        let profile = simulator();

        assert_eq!(
            profile.backend_id,
            "local-statevector"
        );

        assert_eq!(
            profile.qubit_count,
            Some(32)
        );

        assert!(
            profile.supports(
                &Capability::StateVector
            )
        );

        assert!(
            profile.supports_state_access(
                StateAccess::StateVector
            )
        );
    }

    #[test]
    fn gate_model_benchmark_can_be_validated() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "quantum_volume",
            )
            .require_execution_model(
                ExecutionModel::GateModel,
            )
            .require_capability(
                Capability::GateExecution,
            )
            .require_capability(
                Capability::Sampling,
            )
            .require_measurement(
                MeasurementMode::ComputationalBasis,
            )
            .with_min_qubits(8)
            .with_min_shots(1)
            .requiring_deterministic_seed();

        let report =
            profile.validate(&requirements).unwrap();

        assert!(report.is_compatible());
        assert_eq!(
            report.failure_count(),
            0
        );
    }

    #[test]
    fn incompatible_capability_is_reported() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "logical_error_rate",
            )
            .require_capability(
                Capability::LogicalQubits,
            )
            .with_min_logical_qubits(1);

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());

        assert!(
            report
                .failures
                .iter()
                .any(|failure| {
                    matches!(
                        failure,
                        CompatibilityFailure::MissingCapability {
                            capability: Capability::LogicalQubits
                        }
                    )
                })
        );
    }

    #[test]
    fn insufficient_physical_qubits_are_reported() {
        let profile =
            state_vector_simulator_profile(
                "sim",
                "Simulator",
                "zamani",
                4,
            )
            .unwrap();

        let requirements =
            BenchmarkRequirements::new(
                "qv",
            )
            .with_min_qubits(8);

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());

        assert_eq!(
            report.failures,
            vec![
                CompatibilityFailure::InsufficientQubits {
                    required: 8,
                    available: 4
                }
            ]
        );
    }

    #[test]
    fn unknown_resource_is_not_treated_as_unlimited() {
        let profile =
            CapabilityProfile::new(
                "unknown",
                "Unknown Backend",
                "provider",
                BackendTechnology::Custom,
            );

        let requirements =
            BenchmarkRequirements::new(
                "benchmark",
            )
            .with_min_qubits(1);

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());

        assert_eq!(
            report.failures,
            vec![
                CompatibilityFailure::UnknownResource {
                    resource: ResourceKind::PhysicalQubits
                }
            ]
        );
    }

    #[test]
    fn technology_filter_is_enforced() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "hardware_only",
            )
            .accept_technology(
                BackendTechnology::Superconducting,
            );

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());

        assert!(matches!(
            report.failures.first(),
            Some(
                CompatibilityFailure::UnsupportedTechnology {
                    actual: BackendTechnology::StateVectorSimulator,
                    ..
                }
            )
        ));
    }

    #[test]
    fn technology_empty_set_means_any_technology() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "technology_neutral",
            );

        let report =
            profile.validate(&requirements).unwrap();

        assert!(report.is_compatible());
    }

    #[test]
    fn measurement_requirement_is_enforced() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "analog_benchmark",
            )
            .require_measurement(
                MeasurementMode::Analog,
            );

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());
    }

    #[test]
    fn state_access_requirement_is_enforced() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "density_matrix_analysis",
            )
            .require_state_access(
                StateAccess::DensityMatrix,
            );

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());
    }

    #[test]
    fn logical_profile_invariants_are_enforced() {
        let profile =
            CapabilityProfileBuilder::new(
                "invalid",
                "Invalid",
                "zamani",
                BackendTechnology::LogicalQuantumSystem,
            )
            .logical_qubit_count(4)
            .build();

        assert!(matches!(
            profile,
            Err(
                CapabilityError::InvalidProfile { .. }
            )
        ));
    }

    #[test]
    fn logical_gate_requires_logical_qubits() {
        let profile =
            CapabilityProfileBuilder::new(
                "invalid",
                "Invalid",
                "zamani",
                BackendTechnology::LogicalQuantumSystem,
            )
            .capability(
                Capability::LogicalGates,
            )
            .build();

        assert!(profile.is_err());
    }

    #[test]
    fn dynamic_circuit_requires_gate_execution() {
        let profile =
            CapabilityProfileBuilder::new(
                "invalid",
                "Invalid",
                "zamani",
                BackendTechnology::Custom,
            )
            .capability(
                Capability::DynamicCircuits,
            )
            .build();

        assert!(profile.is_err());
    }

    #[test]
    fn measurement_requires_measurement_mode() {
        let profile =
            CapabilityProfileBuilder::new(
                "invalid",
                "Invalid",
                "zamani",
                BackendTechnology::Custom,
            )
            .capability(
                Capability::Measurement,
            )
            .build();

        assert!(profile.is_err());
    }

    #[test]
    fn native_gate_names_are_normalized() {
        let profile =
            CapabilityProfile::new(
                "backend",
                "Backend",
                "provider",
                BackendTechnology::GateModel,
            )
            .with_native_gate(" CX ");

        assert!(
            profile.supports_native_gate("cx")
        );

        assert!(
            profile.supports_native_gate(" CX ")
        );
    }

    #[test]
    fn custom_capabilities_are_supported() {
        let capability =
            Capability::Custom(
                "future_capability".to_string(),
            );

        let profile =
            CapabilityProfile::new(
                "backend",
                "Backend",
                "provider",
                BackendTechnology::Custom,
            )
            .with_capability(
                capability.clone(),
            );

        assert!(
            profile.supports(&capability)
        );
    }

    #[test]
    fn failure_codes_are_machine_readable() {
        let failure =
            CompatibilityFailure::MissingCapability {
                capability:
                    Capability::Sampling,
            };

        assert_eq!(
            failure.code(),
            "missing_capability"
        );
    }

    #[test]
    fn compatibility_report_can_be_converted_to_error() {
        let report =
            CompatibilityReport {
                benchmark_id:
                    "test".to_string(),
                backend_id:
                    "backend".to_string(),
                compatible: false,
                failures: vec![
                    CompatibilityFailure::MissingCapability {
                        capability:
                            Capability::Sampling,
                    }
                ],
            };

        assert!(
            report.into_result().is_err()
        );
    }

    #[test]
    fn standard_profile_supports_qv_basics() {
        let profile = simulator();

        let requirements =
            BenchmarkRequirements::new(
                "quantum_volume",
            )
            .require_execution_model(
                ExecutionModel::GateModel,
            )
            .require_capability(
                Capability::GateExecution,
            )
            .require_capability(
                Capability::Sampling,
            )
            .require_capability(
                Capability::DeterministicSeed,
            )
            .require_measurement(
                MeasurementMode::ComputationalBasis,
            )
            .with_min_qubits(4)
            .with_min_shots(100);

        assert!(
            profile
                .validate(&requirements)
                .unwrap()
                .is_compatible()
        );
    }

    #[test]
    fn resource_limits_are_not_implicitly_unlimited_when_set() {
        let limits = ResourceLimits {
            max_qubits: Some(10),
            max_logical_qubits: None,
            max_circuit_depth: Some(100),
            max_operations: Some(1_000),
            max_shots: Some(10_000),
            max_circuits_per_batch: Some(100),
            max_concurrent_jobs: Some(4),
            max_classical_bits: None,
            max_syndrome_rounds: None,
        };

        let profile =
            CapabilityProfile::new(
                "limited",
                "Limited",
                "provider",
                BackendTechnology::Custom,
            )
            .with_capability(
                Capability::GateExecution,
            )
            .with_measurement_mode(
                MeasurementMode::ComputationalBasis,
            )
            .with_qubit_count(10)
            .with_limits(limits);

        let requirements =
            BenchmarkRequirements::new(
                "large",
            )
            .with_min_qubits(11);

        let report =
            profile.validate(&requirements).unwrap();

        assert!(!report.is_compatible());
    }

    #[test]
    fn profile_is_deterministically_ordered() {
        let profile =
            CapabilityProfile::new(
                "backend",
                "Backend",
                "provider",
                BackendTechnology::GateModel,
            )
            .with_capability(
                Capability::Sampling,
            )
            .with_capability(
                Capability::GateExecution,
            )
            .with_native_gate("z")
            .with_native_gate("cx")
            .with_native_gate("x");

        let capabilities: Vec<String> =
            profile
                .capabilities
                .iter()
                .map(Capability::as_str)
                .collect();

        assert_eq!(
            capabilities,
            vec![
                "gate_execution".to_string(),
                "sampling".to_string(),
            ]
        );

        let gates: Vec<&String> =
            profile.native_gates.iter().collect();

        assert_eq!(
            gates.len(),
            3
        );
    }
}