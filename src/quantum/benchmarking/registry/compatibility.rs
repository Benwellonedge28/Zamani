//! Zamani Quantum Benchmarking — Backend Compatibility
//!
//! Production-grade compatibility negotiation between a benchmark and a
//! quantum execution backend.
//!
//! # Responsibilities
//!
//! This module answers one question:
//!
//!     "Can benchmark X be executed correctly on backend Y under the
//!      requested experiment constraints?"
//!
//! It does NOT:
//!
//! - execute circuits;
//! - generate circuits;
//! - compile or transpile circuits;
//! - perform statistical analysis;
//! - estimate benchmark metrics;
//! - mutate backend state;
//! - perform network/device I/O;
//! - silently downgrade an experiment;
//! - assume that every quantum technology is gate-model based.
//!
//! Those responsibilities belong to the owning benchmarking/runtime layers.
//!
//! # Architectural boundary
//!
//! ```text
//! Benchmark Registry
//!        │
//!        ▼
//! compatibility.rs
//!        │
//!        ▼
//! quantum::hardware::backend
//!        │
//!        ├── BackendKind
//!        ├── BackendStatus
//!        ├── BackendCapabilities
//!        ├── BackendLimits
//!        └── HardwareTopology
//! ```
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! benchmarking → hardware
//! ```
//!
//! and never:
//!
//! ```text
//! hardware → benchmarking
//! ```
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//!
//! # Design principles
//!
//! 1. Compatibility is explicit.
//! 2. Unsupported capability is never silently ignored.
//! 3. Resource limits are checked before execution.
//! 4. Backend operational status is checked separately from technical
//!    capability.
//! 5. Technology-specific requirements are represented explicitly.
//! 6. Compatibility results are deterministic.
//! 7. A compatibility report explains both success and failure.
//! 8. Warnings never masquerade as hard compatibility.
//! 9. The module is usable before the rest of the benchmarking tree exists.
//! 10. The public types are stable enough for `registry.rs` and future
//!    protocol implementations to consume without changing this file.
//!
//! # Integration contract
//!
//! `registry.rs` should:
//!
//! 1. resolve a benchmark descriptor;
//! 2. obtain its `BenchmarkRequirements`;
//! 3. call `check_compatibility()`;
//! 4. reject execution on `Incompatible`;
//! 5. permit execution on `Compatible`;
//! 6. surface `CompatibleWithWarnings` to the caller;
//! 7. preserve the complete `CompatibilityReport` in benchmark provenance.
//!
//! Protocol implementations should construct requirements through the
//! constructors/builders in this file instead of implementing their own
//! backend checks.
//!
//! Future `core/*` types may wrap these types, but this module must remain
//! independent of them to preserve the dependency direction and avoid
//! cyclic module dependencies.

// =============================================================================
// Imports
// =============================================================================

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendKind,
    BackendStatus,
    QuantumBackend,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable compatibility schema version.
///
/// Increment this when the serialized/semantic meaning of compatibility
/// requirements changes incompatibly.
pub const COMPATIBILITY_SCHEMA_VERSION: u32 = 1;

/// Stable namespace for benchmark compatibility identities.
pub const COMPATIBILITY_NAMESPACE: &str = "zamani.quantum.benchmarking.compatibility";

/// Minimum supported benchmark API generation.
pub const MIN_SUPPORTED_BENCHMARK_API_VERSION: u32 = 1;

/// Maximum benchmark API generation understood by this compatibility layer.
pub const MAX_SUPPORTED_BENCHMARK_API_VERSION: u32 = 1;

// =============================================================================
// Benchmark technology
// =============================================================================

/// Quantum technology or execution model expected by a benchmark.
///
/// `Any` means that the benchmark is intentionally technology-neutral.
///
/// The compatibility layer must not infer a technology merely from a
/// backend's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumTechnology {
    /// Any execution technology capable of satisfying the requirements.
    Any,

    /// Gate-model superconducting systems.
    Superconducting,

    /// Trapped-ion systems.
    TrappedIon,

    /// Neutral-atom systems.
    NeutralAtom,

    /// Photonic quantum systems.
    Photonic,

    /// Semiconductor/spin quantum systems.
    Spin,

    /// Topological quantum systems.
    Topological,

    /// Quantum annealing systems.
    Annealing,

    /// Analog quantum simulation systems.
    Analog,

    /// Pure classical simulation.
    ClassicalSimulator,

    /// Hardware-specific or otherwise unclassified technology.
    Custom,
}

impl QuantumTechnology {
    /// Returns whether this requirement is compatible with a backend
    /// technology.
    ///
    /// `Any` is universally compatible.
    #[inline]
    pub fn matches(self, backend: Self) -> bool {
        self == Self::Any || backend == Self::Any || self == backend
    }
}

impl fmt::Display for QuantumTechnology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Any => "any",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::Spin => "spin",
            Self::Topological => "topological",
            Self::Annealing => "annealing",
            Self::Analog => "analog",
            Self::ClassicalSimulator => "classical_simulator",
            Self::Custom => "custom",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Benchmark execution model
// =============================================================================

/// Fundamental execution model expected by a benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionModel {
    /// Standard gate-based circuit execution.
    GateModel,

    /// Analog quantum evolution.
    Analog,

    /// Quantum annealing / optimization hardware.
    Annealing,

    /// Sampling-oriented execution where the benchmark does not require a
    /// conventional circuit abstraction.
    Sampling,

    /// Logical-qubit / fault-tolerant execution.
    Logical,

    /// Purely classical simulator execution.
    Simulator,

    /// Hybrid quantum/classical execution.
    Hybrid,
}

impl fmt::Display for ExecutionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::GateModel => "gate_model",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::Logical => "logical",
            Self::Simulator => "simulator",
            Self::Hybrid => "hybrid",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Benchmark family
// =============================================================================

/// High-level benchmark family.
///
/// This is deliberately broader than individual protocol names so that the
/// compatibility layer can reason about entire classes of benchmarks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BenchmarkFamily {
    /// Quantum Volume and related random-circuit volume benchmarks.
    QuantumVolume,

    /// Randomized benchmarking family.
    RandomizedBenchmarking,

    /// Cross-entropy/random-circuit-sampling family.
    CrossEntropy,

    /// Cycle/layer benchmarking.
    CycleBenchmarking,

    /// SPAM/readout characterization.
    Readout,

    /// Gate/process characterization.
    GateCharacterization,

    /// Coherence characterization.
    Coherence,

    /// Crosstalk characterization.
    Crosstalk,

    /// Long-term stability/drift.
    Drift,

    /// Tomographic characterization.
    Tomography,

    /// Application workload.
    Application,

    /// Volumetric benchmark.
    Volumetric,

    /// Error-correction benchmark.
    ErrorCorrection,

    /// Logical/fault-tolerant benchmark.
    FaultTolerant,

    /// User-defined benchmark.
    Custom,
}

impl fmt::Display for BenchmarkFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::QuantumVolume => "quantum_volume",
            Self::RandomizedBenchmarking => "randomized_benchmarking",
            Self::CrossEntropy => "cross_entropy",
            Self::CycleBenchmarking => "cycle_benchmarking",
            Self::Readout => "readout",
            Self::GateCharacterization => "gate_characterization",
            Self::Coherence => "coherence",
            Self::Crosstalk => "crosstalk",
            Self::Drift => "drift",
            Self::Tomography => "tomography",
            Self::Application => "application",
            Self::Volumetric => "volumetric",
            Self::ErrorCorrection => "error_correction",
            Self::FaultTolerant => "fault_tolerant",
            Self::Custom => "custom",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// Individual backend capability requirements.
///
/// These are deliberately independent from a specific protocol so the same
/// compatibility machinery can serve future benchmark families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityRequirement {
    /// Backend must support measurement.
    Measurement,

    /// Backend must support explicit reset.
    Reset,

    /// Backend must support mid-circuit measurement.
    MidCircuitMeasurement,

    /// Backend must support classical control.
    ClassicalControl,

    /// Backend must support arbitrary single-qubit rotations.
    ArbitrarySingleQubitRotations,

    /// Backend must accept parameterized gates.
    ParameterizedGates,

    /// Backend must support dynamic circuits.
    DynamicCircuits,

    /// Backend must have a non-empty native gate set.
    NativeGateSet,

    /// Backend must expose a non-empty physical topology.
    Topology,

    /// Backend must expose at least one physical qubit.
    Qubits,

    /// Backend must support actual execution rather than merely being
    /// described by the backend abstraction.
    Execution,
}

impl fmt::Display for CapabilityRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::ClassicalControl => "classical_control",
            Self::ArbitrarySingleQubitRotations => {
                "arbitrary_single_qubit_rotations"
            }
            Self::ParameterizedGates => "parameterized_gates",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::NativeGateSet => "native_gate_set",
            Self::Topology => "topology",
            Self::Qubits => "qubits",
            Self::Execution => "execution",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Resource requirements
// =============================================================================

/// Resource requirements imposed by one benchmark execution.
///
/// Zero for a resource means "no explicit requirement".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRequirements {
    /// Minimum number of physical qubits required.
    pub min_qubits: usize,

    /// Maximum number of physical qubits the benchmark intends to use.
    ///
    /// Zero means no explicit maximum.
    pub max_qubits: usize,

    /// Maximum circuit depth required by the benchmark.
    ///
    /// Zero means no explicit maximum.
    pub max_circuit_depth: usize,

    /// Maximum operation count required by the benchmark.
    ///
    /// Zero means no explicit maximum.
    pub max_operations: usize,

    /// Maximum number of shots required by the benchmark.
    ///
    /// Zero means no explicit maximum.
    pub max_shots: usize,

    /// Requested number of shots for one execution point.
    pub requested_shots: usize,

    /// Requested number of qubits for one execution point.
    pub requested_qubits: usize,

    /// Requested circuit depth for one execution point.
    pub requested_depth: usize,

    /// Requested operation count for one execution point.
    pub requested_operations: usize,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            min_qubits: 1,
            max_qubits: 0,
            max_circuit_depth: 0,
            max_operations: 0,
            max_shots: 0,
            requested_shots: 1,
            requested_qubits: 1,
            requested_depth: 0,
            requested_operations: 0,
        }
    }
}

impl ResourceRequirements {
    /// Create conservative one-qubit requirements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum qubit requirement.
    pub fn with_min_qubits(mut self, value: usize) -> Self {
        self.min_qubits = value;
        self
    }

    /// Set requested qubit count.
    pub fn with_requested_qubits(mut self, value: usize) -> Self {
        self.requested_qubits = value;
        self
    }

    /// Set requested shots.
    pub fn with_requested_shots(mut self, value: usize) -> Self {
        self.requested_shots = value;
        self
    }

    /// Set requested depth.
    pub fn with_requested_depth(mut self, value: usize) -> Self {
        self.requested_depth = value;
        self
    }

    /// Set requested operation count.
    pub fn with_requested_operations(mut self, value: usize) -> Self {
        self.requested_operations = value;
        self
    }

    /// Set maximum supported benchmark width.
    pub fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    /// Set maximum circuit depth.
    pub fn with_max_depth(mut self, value: usize) -> Self {
        self.max_circuit_depth = value;
        self
    }

    /// Set maximum operations.
    pub fn with_max_operations(mut self, value: usize) -> Self {
        self.max_operations = value;
        self
    }

    /// Set maximum shots.
    pub fn with_max_shots(mut self, value: usize) -> Self {
        self.max_shots = value;
        self
    }

    fn validate(&self) -> Result<(), CompatibilityError> {
        if self.min_qubits == 0 {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "minimum qubit requirement must be greater than zero",
            });
        }

        if self.requested_qubits == 0 {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "requested qubit count must be greater than zero",
            });
        }

        if self.requested_shots == 0 {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "requested shot count must be greater than zero",
            });
        }

        if self.max_qubits != 0 && self.min_qubits > self.max_qubits {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "minimum qubit count exceeds maximum qubit count",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Gate requirements
// =============================================================================

/// Required native gates.
///
/// Gate names are normalized once at construction time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRequirements {
    gates: BTreeSet<String>,
}

impl Default for GateRequirements {
    fn default() -> Self {
        Self {
            gates: BTreeSet::new(),
        }
    }
}

impl GateRequirements {
    /// Create an empty gate requirement set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add one required gate.
    pub fn with_gate(mut self, gate: impl Into<String>) -> Self {
        let gate = normalize_gate_name(&gate.into());

        if !gate.is_empty() {
            self.gates.insert(gate);
        }

        self
    }

    /// Add several required gates.
    pub fn with_gates<I, S>(mut self, gates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for gate in gates {
            let gate = normalize_gate_name(&gate.into());

            if !gate.is_empty() {
                self.gates.insert(gate);
            }
        }

        self
    }

    /// Return required native gates.
    pub fn gates(&self) -> impl Iterator<Item = &str> {
        self.gates.iter().map(String::as_str)
    }

    /// Return the number of required gates.
    pub fn len(&self) -> usize {
        self.gates.len()
    }

    /// Whether there are no required gates.
    pub fn is_empty(&self) -> bool {
        self.gates.is_empty()
    }
}

// =============================================================================
// Topology requirements
// =============================================================================

/// Topology constraints for a benchmark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRequirements {
    /// Whether the backend must expose topology information.
    pub required: bool,

    /// Whether all required two-qubit interactions must be directly
    /// connected.
    ///
    /// If false, a routing layer may potentially make the benchmark
    /// executable.
    pub requires_direct_connectivity: bool,

    /// Required two-qubit pairs.
    pub required_edges: Vec<(usize, usize)>,

    /// Whether a connected topology is required.
    pub requires_connected_topology: bool,
}

impl Default for TopologyRequirements {
    fn default() -> Self {
        Self {
            required: false,
            requires_direct_connectivity: false,
            required_edges: Vec::new(),
            requires_connected_topology: false,
        }
    }
}

impl TopologyRequirements {
    /// Require topology metadata.
    pub fn required() -> Self {
        Self {
            required: true,
            ..Self::default()
        }
    }

    /// Require direct connectivity.
    pub fn direct(mut self) -> Self {
        self.required = true;
        self.requires_direct_connectivity = true;
        self
    }

    /// Require a connected topology.
    pub fn connected(mut self) -> Self {
        self.required = true;
        self.requires_connected_topology = true;
        self
    }

    /// Add a required physical edge.
    pub fn with_edge(mut self, source: usize, target: usize) -> Self {
        self.required = true;
        self.required_edges.push((source, target));
        self
    }
}

// =============================================================================
// Benchmark requirements
// =============================================================================

/// Complete compatibility contract for a benchmark.
///
/// This is the primary type consumed by `registry.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkRequirements {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark family.
    pub family: BenchmarkFamily,

    /// Benchmark API version.
    pub api_version: u32,

    /// Required execution model.
    pub execution_model: ExecutionModel,

    /// Required quantum technology.
    pub technology: QuantumTechnology,

    /// Required backend capabilities.
    pub capabilities: BTreeSet<CapabilityRequirement>,

    /// Resource requirements.
    pub resources: ResourceRequirements,

    /// Native gate requirements.
    pub gates: GateRequirements,

    /// Topology requirements.
    pub topology: TopologyRequirements,

    /// Whether the benchmark can execute on simulators.
    pub simulator_allowed: bool,

    /// Whether the benchmark can execute on emulators.
    pub emulator_allowed: bool,

    /// Whether the benchmark requires a physical QPU.
    pub qpu_required: bool,

    /// Whether logical/fault-tolerant execution is required.
    pub logical_backend_required: bool,

    /// Optional required backend kinds.
    pub allowed_backend_kinds: BTreeSet<BackendKind>,
}

impl BenchmarkRequirements {
    /// Create a new technology-neutral benchmark requirement set.
    pub fn new(
        benchmark_id: impl Into<String>,
        family: BenchmarkFamily,
        execution_model: ExecutionModel,
    ) -> Result<Self, CompatibilityError> {
        let benchmark_id = normalize_identifier(&benchmark_id.into());

        if benchmark_id.is_empty() {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "benchmark ID cannot be empty",
            });
        }

        Ok(Self {
            benchmark_id,
            family,
            api_version: 1,
            execution_model,
            technology: QuantumTechnology::Any,
            capabilities: BTreeSet::new(),
            resources: ResourceRequirements::default(),
            gates: GateRequirements::default(),
            topology: TopologyRequirements::default(),
            simulator_allowed: true,
            emulator_allowed: true,
            qpu_required: false,
            logical_backend_required: false,
            allowed_backend_kinds: BTreeSet::new(),
        })
    }

    /// Require a backend capability.
    pub fn require_capability(
        mut self,
        capability: CapabilityRequirement,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Require a technology.
    pub fn require_technology(
        mut self,
        technology: QuantumTechnology,
    ) -> Self {
        self.technology = technology;
        self
    }

    /// Restrict execution to a backend kind.
    pub fn allow_backend_kind(
        mut self,
        kind: BackendKind,
    ) -> Self {
        self.allowed_backend_kinds.insert(kind);
        self
    }

    /// Require a physical QPU.
    pub fn require_qpu(mut self) -> Self {
        self.qpu_required = true;
        self.simulator_allowed = false;
        self.emulator_allowed = false;
        self.allowed_backend_kinds.clear();
        self.allowed_backend_kinds.insert(BackendKind::Qpu);
        self
    }

    /// Require a logical backend.
    pub fn require_logical_backend(mut self) -> Self {
        self.logical_backend_required = true;
        self
    }

    /// Disallow simulator execution.
    pub fn disallow_simulator(mut self) -> Self {
        self.simulator_allowed = false;
        self
    }

    /// Disallow emulator execution.
    pub fn disallow_emulator(mut self) -> Self {
        self.emulator_allowed = false;
        self
    }

    /// Replace resource requirements.
    pub fn with_resources(
        mut self,
        resources: ResourceRequirements,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Replace native gate requirements.
    pub fn with_gates(
        mut self,
        gates: GateRequirements,
    ) -> Self {
        self.gates = gates;
        self
    }

    /// Replace topology requirements.
    pub fn with_topology(
        mut self,
        topology: TopologyRequirements,
    ) -> Self {
        self.topology = topology;
        self
    }

    /// Validate the requirement object itself.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if self.benchmark_id.is_empty() {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "benchmark ID cannot be empty",
            });
        }

        if self.api_version < MIN_SUPPORTED_BENCHMARK_API_VERSION
            || self.api_version > MAX_SUPPORTED_BENCHMARK_API_VERSION
        {
            return Err(CompatibilityError::UnsupportedBenchmarkApiVersion {
                requested: self.api_version,
                minimum: MIN_SUPPORTED_BENCHMARK_API_VERSION,
                maximum: MAX_SUPPORTED_BENCHMARK_API_VERSION,
            });
        }

        self.resources.validate()?;

        if self.qpu_required
            && self
                .allowed_backend_kinds
                .iter()
                .any(|kind| *kind != BackendKind::Qpu)
        {
            return Err(CompatibilityError::InvalidRequirements {
                reason: "a QPU-required benchmark cannot allow non-QPU-only backend kinds",
            });
        }

        if self.logical_backend_required
            && self.execution_model != ExecutionModel::Logical
        {
            return Err(CompatibilityError::InvalidRequirements {
                reason:
                    "logical backend requirement requires logical execution model",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Compatibility finding
// =============================================================================

/// Severity of a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FindingSeverity {
    /// Informational observation.
    Info,

    /// The benchmark is executable, but the caller should be aware of a
    /// limitation or condition.
    Warning,

    /// The benchmark cannot be executed correctly.
    Error,
}

impl fmt::Display for FindingSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => f.write_str("info"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}

/// Machine-readable compatibility reason.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompatibilityReason {
    BackendUnavailable,
    UnsupportedBackendKind,
    UnsupportedTechnology,
    UnsupportedExecutionModel,
    MissingCapability,
    UnsupportedGate,
    InsufficientQubits,
    CircuitDepthLimitExceeded,
    OperationLimitExceeded,
    ShotLimitExceeded,
    MissingTopology,
    DisconnectedTopology,
    MissingDirectConnection,
    SimulatorNotAllowed,
    EmulatorNotAllowed,
    QpuRequired,
    LogicalBackendRequired,
    InvalidBenchmarkRequirements,
    UnsupportedBenchmarkApiVersion,
}

impl fmt::Display for CompatibilityReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::BackendUnavailable => "backend_unavailable",
            Self::UnsupportedBackendKind => "unsupported_backend_kind",
            Self::UnsupportedTechnology => "unsupported_technology",
            Self::UnsupportedExecutionModel => {
                "unsupported_execution_model"
            }
            Self::MissingCapability => "missing_capability",
            Self::UnsupportedGate => "unsupported_gate",
            Self::InsufficientQubits => "insufficient_qubits",
            Self::CircuitDepthLimitExceeded => {
                "circuit_depth_limit_exceeded"
            }
            Self::OperationLimitExceeded => "operation_limit_exceeded",
            Self::ShotLimitExceeded => "shot_limit_exceeded",
            Self::MissingTopology => "missing_topology",
            Self::DisconnectedTopology => "disconnected_topology",
            Self::MissingDirectConnection => {
                "missing_direct_connection"
            }
            Self::SimulatorNotAllowed => "simulator_not_allowed",
            Self::EmulatorNotAllowed => "emulator_not_allowed",
            Self::QpuRequired => "qpu_required",
            Self::LogicalBackendRequired => {
                "logical_backend_required"
            }
            Self::InvalidBenchmarkRequirements => {
                "invalid_benchmark_requirements"
            }
            Self::UnsupportedBenchmarkApiVersion => {
                "unsupported_benchmark_api_version"
            }
        };

        f.write_str(value)
    }
}

/// One compatibility diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityFinding {
    /// Finding severity.
    pub severity: FindingSeverity,

    /// Machine-readable reason.
    pub reason: CompatibilityReason,

    /// Stable human-readable message.
    pub message: String,

    /// Optional capability involved.
    pub capability: Option<CapabilityRequirement>,

    /// Optional gate involved.
    pub gate: Option<String>,
}

impl CompatibilityFinding {
    fn error(
        reason: CompatibilityReason,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: FindingSeverity::Error,
            reason,
            message: message.into(),
            capability: None,
            gate: None,
        }
    }

    fn warning(
        reason: CompatibilityReason,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: FindingSeverity::Warning,
            reason,
            message: message.into(),
            capability: None,
            gate: None,
        }
    }

    fn info(
        reason: CompatibilityReason,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: FindingSeverity::Info,
            reason,
            message: message.into(),
            capability: None,
            gate: None,
        }
    }

    fn with_capability(
        mut self,
        capability: CapabilityRequirement,
    ) -> Self {
        self.capability = Some(capability);
        self
    }

    fn with_gate(
        mut self,
        gate: impl Into<String>,
    ) -> Self {
        self.gate = Some(gate.into());
        self
    }
}

// =============================================================================
// Compatibility status
// =============================================================================

/// Final compatibility state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityStatus {
    /// Fully compatible with no warnings.
    Compatible,

    /// Executable, but with warnings that must be retained by callers.
    CompatibleWithWarnings,

    /// Cannot be executed correctly.
    Incompatible,
}

impl CompatibilityStatus {
    /// Whether the benchmark can be executed.
    #[inline]
    pub fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Compatible | Self::CompatibleWithWarnings
        )
    }

    /// Whether execution is permitted without warnings.
    #[inline]
    pub fn is_strictly_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

impl fmt::Display for CompatibilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible => f.write_str("compatible"),
            Self::CompatibleWithWarnings => {
                f.write_str("compatible_with_warnings")
            }
            Self::Incompatible => f.write_str("incompatible"),
        }
    }
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Complete compatibility decision.
///
/// This is intentionally immutable after construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    /// Compatibility schema version.
    pub schema_version: u32,

    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Backend identifier.
    pub backend_id: String,

    /// Benchmark family.
    pub family: BenchmarkFamily,

    /// Required execution model.
    pub execution_model: ExecutionModel,

    /// Required technology.
    pub required_technology: QuantumTechnology,

    /// Result status.
    pub status: CompatibilityStatus,

    /// All compatibility findings.
    pub findings: Vec<CompatibilityFinding>,
}

impl CompatibilityReport {
    /// Whether this report permits execution.
    #[inline]
    pub fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Whether this report contains errors.
    #[inline]
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }

    /// Whether this report contains warnings.
    #[inline]
    pub fn has_warnings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Warning)
    }

    /// Number of errors.
    pub fn error_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.severity == FindingSeverity::Error
            })
            .count()
    }

    /// Number of warnings.
    pub fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.severity == FindingSeverity::Warning
            })
            .count()
    }

    /// Return only error findings.
    pub fn errors(&self) -> impl Iterator<Item = &CompatibilityFinding> {
        self.findings.iter().filter(|finding| {
            finding.severity == FindingSeverity::Error
        })
    }

    /// Return only warning findings.
    pub fn warnings(
        &self,
    ) -> impl Iterator<Item = &CompatibilityFinding> {
        self.findings.iter().filter(|finding| {
            finding.severity == FindingSeverity::Warning
        })
    }
}

// =============================================================================
// Compatibility errors
// =============================================================================

/// Errors indicating that compatibility itself could not be evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    /// Benchmark requirements are malformed.
    InvalidRequirements {
        reason: &'static str,
    },

    /// Benchmark API version is not understood.
    UnsupportedBenchmarkApiVersion {
        requested: u32,
        minimum: u32,
        maximum: u32,
    },
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirements { reason } => {
                write!(f, "invalid benchmark compatibility requirements: {reason}")
            }

            Self::UnsupportedBenchmarkApiVersion {
                requested,
                minimum,
                maximum,
            } => write!(
                f,
                "unsupported benchmark API version {}; supported range is {}..={}",
                requested, minimum, maximum
            ),
        }
    }
}

impl std::error::Error for CompatibilityError {}

// =============================================================================
// Public compatibility functions
// =============================================================================

/// Check whether a benchmark is compatible with a backend.
///
/// This is the primary API that `registry.rs` should call.
///
/// The function is pure with respect to the backend: it does not mutate
/// backend state or perform execution.
pub fn check_compatibility(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
) -> Result<CompatibilityReport, CompatibilityError> {
    requirements.validate()?;

    let mut findings = Vec::new();

    check_backend_status(requirements, backend, &mut findings);
    check_backend_kind(requirements, backend, &mut findings);
    check_execution_model(requirements, backend, &mut findings);
    check_backend_technology(requirements, backend, &mut findings);
    check_backend_capabilities(requirements, backend, &mut findings);
    check_backend_kind_policy(requirements, backend, &mut findings);
    check_resources(requirements, backend, &mut findings);
    check_gates(requirements, backend, &mut findings);
    check_topology(requirements, backend, &mut findings);

    let status = determine_status(&findings);

    Ok(CompatibilityReport {
        schema_version: COMPATIBILITY_SCHEMA_VERSION,
        benchmark_id: requirements.benchmark_id.clone(),
        backend_id: backend.id().to_owned(),
        family: requirements.family,
        execution_model: requirements.execution_model,
        required_technology: requirements.technology,
        status,
        findings,
    })
}

/// Strict compatibility check.
///
/// Returns `Ok(())` only when the benchmark is fully compatible without
/// warnings.
///
/// This is useful for CI and hardware submission paths where warnings should
/// block execution rather than being accepted implicitly.
pub fn require_strict_compatibility(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
) -> Result<(), CompatibilityError> {
    let report = check_compatibility(requirements, backend)?;

    if report.status == CompatibilityStatus::Compatible {
        Ok(())
    } else if report.status == CompatibilityStatus::CompatibleWithWarnings {
        Err(CompatibilityError::InvalidRequirements {
            reason: "benchmark is compatible only with warnings under strict compatibility",
        })
    } else {
        Err(CompatibilityError::InvalidRequirements {
            reason: "benchmark is incompatible with backend",
        })
    }
}

/// Convenience predicate for registry discovery.
///
/// This never returns an error because malformed requirements are treated as
/// incompatible for discovery purposes.
pub fn is_compatible(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
) -> bool {
    check_compatibility(requirements, backend)
        .map(|report| report.is_compatible())
        .unwrap_or(false)
}

// =============================================================================
// Individual checks
// =============================================================================

fn check_backend_status(
    _requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    if backend.is_available() {
        findings.push(CompatibilityFinding::info(
            CompatibilityReason::BackendUnavailable,
            "backend is operationally available",
        ));
        return;
    }

    findings.push(CompatibilityFinding::error(
        CompatibilityReason::BackendUnavailable,
        format!(
            "backend '{}' is not available ({:?})",
            backend.id(),
            backend.metadata.status
        ),
    ));
}

fn check_backend_kind(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    match requirements.execution_model {
        ExecutionModel::Simulator => {
            if backend.kind() != BackendKind::Simulator {
                findings.push(CompatibilityFinding::warning(
                    CompatibilityReason::UnsupportedExecutionModel,
                    format!(
                        "benchmark requests simulator execution but backend kind is {:?}",
                        backend.kind()
                    ),
                ));
            }
        }

        ExecutionModel::GateModel
        | ExecutionModel::Hybrid
        | ExecutionModel::Sampling
        | ExecutionModel::Analog
        | ExecutionModel::Annealing
        | ExecutionModel::Logical => {}
    }
}

fn check_execution_model(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    match requirements.execution_model {
        ExecutionModel::GateModel => {
            if matches!(
                backend.kind(),
                BackendKind::Qpu
                    | BackendKind::Simulator
                    | BackendKind::Emulator
                    | BackendKind::Custom
            ) {
                findings.push(CompatibilityFinding::info(
                    CompatibilityReason::UnsupportedExecutionModel,
                    "backend kind is eligible for gate-model compatibility; protocol-specific capabilities are checked separately",
                ));
            } else {
                findings.push(CompatibilityFinding::error(
                    CompatibilityReason::UnsupportedExecutionModel,
                    format!(
                        "backend kind {:?} cannot satisfy gate-model execution",
                        backend.kind()
                    ),
                ));
            }
        }

        ExecutionModel::Simulator => {
            if backend.kind() != BackendKind::Simulator {
                findings.push(CompatibilityFinding::error(
                    CompatibilityReason::UnsupportedExecutionModel,
                    "simulator execution was explicitly required",
                ));
            }
        }

        ExecutionModel::Logical => {
            if !matches!(
                backend.kind(),
                BackendKind::Qpu | BackendKind::Custom
            ) {
                findings.push(CompatibilityFinding::error(
                    CompatibilityReason::LogicalBackendRequired,
                    "logical execution requires a logical-capable QPU or custom backend",
                ));
            }
        }

        // The backend abstraction currently does not encode separate analog
        // or annealing kinds. Do not falsely claim compatibility; these
        // models require a future capability/metadata extension.
        ExecutionModel::Analog => {
            findings.push(CompatibilityFinding::warning(
                CompatibilityReason::UnsupportedExecutionModel,
                "analog execution requires an explicit backend capability; the current hardware backend contract does not encode it",
            ));
        }

        ExecutionModel::Annealing => {
            findings.push(CompatibilityFinding::warning(
                CompatibilityReason::UnsupportedExecutionModel,
                "annealing execution requires an explicit backend capability; the current hardware backend contract does not encode it",
            ));
        }

        ExecutionModel::Sampling | ExecutionModel::Hybrid => {
            // Sampling and hybrid execution can be implemented by several
            // backend kinds. Concrete protocol capabilities remain the
            // authoritative check.
        }
    }
}

fn check_backend_technology(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    if requirements.technology == QuantumTechnology::Any {
        return;
    }

    let actual = infer_backend_technology(backend);

    match actual {
        Some(actual) if requirements.technology.matches(actual) => {
            findings.push(CompatibilityFinding::info(
                CompatibilityReason::UnsupportedTechnology,
                format!(
                    "backend technology '{}' satisfies required technology '{}'",
                    actual, requirements.technology
                ),
            ));
        }

        Some(actual) => {
            findings.push(CompatibilityFinding::error(
                CompatibilityReason::UnsupportedTechnology,
                format!(
                    "benchmark requires technology '{}' but backend reports '{}'",
                    requirements.technology, actual
                ),
            ));
        }

        None => {
            findings.push(CompatibilityFinding::warning(
                CompatibilityReason::UnsupportedTechnology,
                format!(
                    "benchmark requires technology '{}', but backend metadata does not declare a recognized technology",
                    requirements.technology
                ),
            ));
        }
    }
}

fn check_backend_capabilities(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    for requirement in &requirements.capabilities {
        if capability_supported(&backend.capabilities, *requirement) {
            continue;
        }

        findings.push(
            CompatibilityFinding::error(
                CompatibilityReason::MissingCapability,
                format!(
                    "backend '{}' does not provide required capability '{}'",
                    backend.id(),
                    requirement
                ),
            )
            .with_capability(*requirement),
        );
    }
}

fn check_backend_kind_policy(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    if requirements.qpu_required && backend.kind() != BackendKind::Qpu {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::QpuRequired,
            "benchmark requires a physical QPU",
        ));
    }

    if requirements.logical_backend_required
        && !matches!(
            backend.kind(),
            BackendKind::Qpu | BackendKind::Custom
        )
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::LogicalBackendRequired,
            "benchmark requires logical backend execution",
        ));
    }

    match backend.kind() {
        BackendKind::Simulator if !requirements.simulator_allowed => {
            findings.push(CompatibilityFinding::error(
                CompatibilityReason::SimulatorNotAllowed,
                "this benchmark explicitly disallows simulator execution",
            ));
        }

        BackendKind::Emulator if !requirements.emulator_allowed => {
            findings.push(CompatibilityFinding::error(
                CompatibilityReason::EmulatorNotAllowed,
                "this benchmark explicitly disallows emulator execution",
            ));
        }

        _ => {}
    }

    if !requirements.allowed_backend_kinds.is_empty()
        && !requirements
            .allowed_backend_kinds
            .contains(&backend.kind())
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::UnsupportedBackendKind,
            format!(
                "backend kind {:?} is not in the benchmark's allowed backend kinds",
                backend.kind()
            ),
        ));
    }
}

fn check_resources(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    let resources = &requirements.resources;

    let available_qubits = backend.qubit_count();

    if available_qubits < resources.min_qubits {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::InsufficientQubits,
            format!(
                "benchmark requires at least {} qubits but backend exposes {}",
                resources.min_qubits, available_qubits
            ),
        ));
    }

    if resources.requested_qubits > 0
        && available_qubits < resources.requested_qubits
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::InsufficientQubits,
            format!(
                "benchmark requests {} qubits but backend exposes {}",
                resources.requested_qubits, available_qubits
            ),
        ));
    }

    let limits = &backend.limits;

    if limits.max_qubits != 0
        && resources.requested_qubits > limits.max_qubits
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::InsufficientQubits,
            format!(
                "benchmark requests {} qubits but backend limit is {}",
                resources.requested_qubits, limits.max_qubits
            ),
        ));
    }

    if resources.max_qubits != 0
        && limits.max_qubits != 0
        && resources.max_qubits > limits.max_qubits
    {
        findings.push(CompatibilityFinding::warning(
            CompatibilityReason::InsufficientQubits,
            format!(
                "benchmark's declared maximum width {} exceeds backend maximum {}",
                resources.max_qubits, limits.max_qubits
            ),
        ));
    }

    if limits.max_circuit_depth != 0
        && resources.requested_depth > limits.max_circuit_depth
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::CircuitDepthLimitExceeded,
            format!(
                "benchmark requests circuit depth {} but backend limit is {}",
                resources.requested_depth,
                limits.max_circuit_depth
            ),
        ));
    }

    if resources.max_circuit_depth != 0
        && limits.max_circuit_depth != 0
        && resources.max_circuit_depth > limits.max_circuit_depth
    {
        findings.push(CompatibilityFinding::warning(
            CompatibilityReason::CircuitDepthLimitExceeded,
            format!(
                "benchmark's declared maximum depth {} exceeds backend limit {}",
                resources.max_circuit_depth,
                limits.max_circuit_depth
            ),
        ));
    }

    if limits.max_operations != 0
        && resources.requested_operations > limits.max_operations
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::OperationLimitExceeded,
            format!(
                "benchmark requests {} operations but backend limit is {}",
                resources.requested_operations,
                limits.max_operations
            ),
        ));
    }

    if resources.max_operations != 0
        && limits.max_operations != 0
        && resources.max_operations > limits.max_operations
    {
        findings.push(CompatibilityFinding::warning(
            CompatibilityReason::OperationLimitExceeded,
            format!(
                "benchmark's declared maximum operation count {} exceeds backend limit {}",
                resources.max_operations,
                limits.max_operations
            ),
        ));
    }

    if limits.max_shots != 0
        && resources.requested_shots > limits.max_shots
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::ShotLimitExceeded,
            format!(
                "benchmark requests {} shots but backend limit is {}",
                resources.requested_shots, limits.max_shots
            ),
        ));
    }

    if resources.max_shots != 0
        && limits.max_shots != 0
        && resources.max_shots > limits.max_shots
    {
        findings.push(CompatibilityFinding::warning(
            CompatibilityReason::ShotLimitExceeded,
            format!(
                "benchmark's declared maximum shot count {} exceeds backend limit {}",
                resources.max_shots, limits.max_shots
            ),
        ));
    }
}

fn check_gates(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    for gate in requirements.gates.gates() {
        if backend.capabilities.supports_gate(gate) {
            continue;
        }

        findings.push(
            CompatibilityFinding::error(
                CompatibilityReason::UnsupportedGate,
                format!(
                    "backend '{}' does not advertise required native gate '{}'",
                    backend.id(),
                    gate
                ),
            )
            .with_gate(gate),
        );
    }
}

fn check_topology(
    requirements: &BenchmarkRequirements,
    backend: &QuantumBackend,
    findings: &mut Vec<CompatibilityFinding>,
) {
    let topology = &backend.topology;

    if requirements.topology.required
        && topology.qubit_count() == 0
    {
        findings.push(CompatibilityFinding::error(
            CompatibilityReason::MissingTopology,
            "benchmark requires topology information but backend exposes no qubits",
        ));

        return;
    }

    if requirements.topology.requires_connected_topology
        && !topology.is_fully_connected()
    {
        findings.push(CompatibilityFinding::warning(
            CompatibilityReason::DisconnectedTopology,
            "benchmark requests a connected topology but backend topology is not fully connected",
        ));
    }

    for &(source, target) in &requirements.topology.required_edges {
        if source >= topology.qubit_count()
            || target >= topology.qubit_count()
        {
            findings.push(CompatibilityFinding::error(
                CompatibilityReason::MissingDirectConnection,
                format!(
                    "required topology edge {} -> {} references a qubit outside the backend topology",
                    source, target
                ),
            ));

            continue;
        }

        match topology.is_connected(source, target) {
            Ok(true) => {}

            Ok(false) => {
                if requirements
                    .topology
                    .requires_direct_connectivity
                {
                    findings.push(CompatibilityFinding::error(
                        CompatibilityReason::MissingDirectConnection,
                        format!(
                            "backend does not provide required direct connection {} -> {}",
                            source, target
                        ),
                    ));
                } else {
                    findings.push(CompatibilityFinding::warning(
                        CompatibilityReason::MissingDirectConnection,
                        format!(
                            "backend does not directly connect {} -> {}; routing may be required",
                            source, target
                        ),
                    ));
                }
            }

            Err(_) => {
                findings.push(CompatibilityFinding::error(
                    CompatibilityReason::MissingDirectConnection,
                    format!(
                        "backend topology could not validate required connection {} -> {}",
                        source, target
                    ),
                ));
            }
        }
    }
}

// =============================================================================
// Capability mapping
// =============================================================================

fn capability_supported(
    capabilities: &BackendCapabilities,
    requirement: CapabilityRequirement,
) -> bool {
    match requirement {
        CapabilityRequirement::Measurement => {
            capabilities.measurement
        }

        CapabilityRequirement::Reset => capabilities.reset,

        CapabilityRequirement::MidCircuitMeasurement => {
            capabilities.mid_circuit_measurement
        }

        CapabilityRequirement::ClassicalControl => {
            capabilities.classical_control
        }

        CapabilityRequirement::ArbitrarySingleQubitRotations => {
            capabilities.arbitrary_single_qubit_rotations
        }

        CapabilityRequirement::ParameterizedGates => {
            capabilities.parameterized_gates
        }

        CapabilityRequirement::DynamicCircuits => {
            capabilities.dynamic_circuits
        }

        CapabilityRequirement::NativeGateSet => {
            !capabilities.native_gates.is_empty()
        }

        CapabilityRequirement::Topology => true,

        CapabilityRequirement::Qubits => true,

        // The current QuantumBackend is a descriptor and deliberately does
        // not itself implement execution. Therefore this requirement cannot
        // be truthfully inferred from the current backend abstraction.
        CapabilityRequirement::Execution => false,
    }
}

// =============================================================================
// Technology inference
// =============================================================================

/// Infer backend technology from stable backend metadata.
///
/// The current hardware backend abstraction stores arbitrary metadata
/// properties rather than a dedicated technology enum. This function therefore
/// uses a deliberately conservative metadata lookup.
///
/// Recognized keys, in priority order:
///
/// - `technology`
/// - `quantum_technology`
/// - `platform`
///
/// Unknown values return `None`; compatibility then reports a warning instead
/// of falsely claiming a technology match.
fn infer_backend_technology(
    backend: &QuantumBackend,
) -> Option<QuantumTechnology> {
    let properties = &backend.metadata.properties;

    for key in ["technology", "quantum_technology", "platform"] {
        if let Some(value) = properties.get(key) {
            if let Some(technology) = parse_technology(value) {
                return Some(technology);
            }
        }
    }

    match backend.kind() {
        BackendKind::Simulator => {
            Some(QuantumTechnology::ClassicalSimulator)
        }

        BackendKind::Emulator => {
            // An emulator is not necessarily a simulator of one specific
            // technology. Do not guess.
            None
        }

        BackendKind::Qpu | BackendKind::Custom => None,
    }
}

fn parse_technology(value: &str) -> Option<QuantumTechnology> {
    match normalize_identifier(value).as_str() {
        "any" => Some(QuantumTechnology::Any),

        "superconducting"
        | "superconducting_qubit"
        | "superconducting_qubits" => {
            Some(QuantumTechnology::Superconducting)
        }

        "trapped_ion" | "trapped_ions" | "ion_trap" => {
            Some(QuantumTechnology::TrappedIon)
        }

        "neutral_atom" | "neutral_atoms" => {
            Some(QuantumTechnology::NeutralAtom)
        }

        "photonic" | "photon" | "photons" => {
            Some(QuantumTechnology::Photonic)
        }

        "spin" | "spin_qubit" | "spin_qubits" => {
            Some(QuantumTechnology::Spin)
        }

        "topological" | "topological_qubit" => {
            Some(QuantumTechnology::Topological)
        }

        "annealing" | "quantum_annealing" => {
            Some(QuantumTechnology::Annealing)
        }

        "analog" | "analog_quantum" => {
            Some(QuantumTechnology::Analog)
        }

        "classical_simulator"
        | "simulator"
        | "statevector_simulator"
        | "density_matrix_simulator" => {
            Some(QuantumTechnology::ClassicalSimulator)
        }

        "custom" => Some(QuantumTechnology::Custom),

        _ => None,
    }
}

// =============================================================================
// Status calculation
// =============================================================================

fn determine_status(
    findings: &[CompatibilityFinding],
) -> CompatibilityStatus {
    if findings.iter().any(|finding| {
        finding.severity == FindingSeverity::Error
    }) {
        return CompatibilityStatus::Incompatible;
    }

    if findings.iter().any(|finding| {
        finding.severity == FindingSeverity::Warning
    }) {
        return CompatibilityStatus::CompatibleWithWarnings;
    }

    CompatibilityStatus::Compatible
}

// =============================================================================
// Normalization helpers
// =============================================================================

fn normalize_identifier(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn normalize_gate_name(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::hardware::backend::{
        BackendCapabilities,
        BackendLimits,
        BackendMetadata,
        BackendKind,
    };
    use crate::quantum::hardware::topology::HardwareTopology;

    fn simulator_backend() -> QuantumBackend {
        let metadata = BackendMetadata::new(
            "test.simulator",
            "Test Simulator",
            "Zamani",
            "1.0.0",
            BackendKind::Simulator,
        );

        let capabilities = BackendCapabilities::new()
            .with_gates([
                "x",
                "y",
                "z",
                "h",
                "s",
                "t",
                "cx",
                "measure",
            ]);

        let limits = BackendLimits::unlimited();

        QuantumBackend::new(
            metadata,
            capabilities,
            limits,
            HardwareTopology::fully_connected(32).unwrap(),
        )
        .unwrap()
    }

    fn qpu_backend() -> QuantumBackend {
        let mut metadata = BackendMetadata::new(
            "test.qpu",
            "Test QPU",
            "Zamani",
            "1.0.0",
            BackendKind::Qpu,
        );

        metadata.insert_property(
            "technology",
            "superconducting",
        );

        let capabilities = BackendCapabilities::new()
            .with_gates([
                "x",
                "y",
                "z",
                "h",
                "sx",
                "cx",
                "measure",
            ]);

        let limits = BackendLimits::default()
            .with_max_qubits(10)
            .with_max_depth(100)
            .with_max_operations(10_000)
            .with_max_shots(100_000);

        QuantumBackend::new(
            metadata,
            capabilities,
            limits,
            HardwareTopology::linear(10).unwrap(),
        )
        .unwrap()
    }

    fn basic_qv_requirements() -> BenchmarkRequirements {
        BenchmarkRequirements::new(
            "quantum_volume",
            BenchmarkFamily::QuantumVolume,
            ExecutionModel::GateModel,
        )
        .unwrap()
        .require_capability(CapabilityRequirement::Measurement)
        .require_capability(CapabilityRequirement::NativeGateSet)
        .with_resources(
            ResourceRequirements::new()
                .with_min_qubits(2)
                .with_requested_qubits(4)
                .with_requested_shots(1_000)
                .with_requested_depth(4),
        )
        .with_gates(
            GateRequirements::new()
                .with_gates(["h", "cx"]),
        )
        .with_topology(
            TopologyRequirements::required(),
        )
    }

    #[test]
    fn compatible_simulator_is_accepted() {
        let backend = simulator_backend();
        let requirements = basic_qv_requirements();

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert!(report.is_compatible());
        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );
        assert!(!report.has_errors());
    }

    #[test]
    fn insufficient_qubits_are_rejected() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements()
                .with_resources(
                    ResourceRequirements::new()
                        .with_min_qubits(11)
                        .with_requested_qubits(11),
                );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(report.errors().any(|finding| {
            finding.reason
                == CompatibilityReason::InsufficientQubits
        }));
    }

    #[test]
    fn unsupported_gate_is_rejected() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements().with_gates(
                GateRequirements::new()
                    .with_gate("toffoli"),
            );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(report.errors().any(|finding| {
            finding.reason
                == CompatibilityReason::UnsupportedGate
        }));
    }

    #[test]
    fn directed_topology_warning_is_not_silently_ignored() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements().with_topology(
                TopologyRequirements::required()
                    .with_edge(0, 9),
            );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        // Linear topology does not directly connect 0 and 9.
        assert!(report.has_warnings());
        assert!(report.is_compatible());
    }

    #[test]
    fn direct_topology_requirement_rejects_missing_edge() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements().with_topology(
                TopologyRequirements::required()
                    .direct()
                    .with_edge(0, 9),
            );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(report.errors().any(|finding| {
            finding.reason
                == CompatibilityReason::MissingDirectConnection
        }));
    }

    #[test]
    fn qpu_requirement_rejects_simulator() {
        let backend = simulator_backend();

        let requirements =
            basic_qv_requirements().require_qpu();

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(report.errors().any(|finding| {
            finding.reason
                == CompatibilityReason::QpuRequired
        }));
    }

    #[test]
    fn simulator_only_requirement_rejects_qpu() {
        let backend = qpu_backend();

        let requirements =
            BenchmarkRequirements::new(
                "simulator_benchmark",
                BenchmarkFamily::Custom,
                ExecutionModel::Simulator,
            )
            .unwrap();

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn technology_is_checked_from_backend_metadata() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements()
                .require_technology(
                    QuantumTechnology::Superconducting,
                );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert!(report.is_compatible());
    }

    #[test]
    fn incompatible_technology_is_rejected() {
        let backend = qpu_backend();

        let requirements =
            basic_qv_requirements()
                .require_technology(
                    QuantumTechnology::TrappedIon,
                );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn technology_is_not_guessed_for_unknown_qpu() {
        let metadata = BackendMetadata::new(
            "unknown.qpu",
            "Unknown QPU",
            "Zamani",
            "1.0.0",
            BackendKind::Qpu,
        );

        let backend = QuantumBackend::new(
            metadata,
            BackendCapabilities::new()
                .with_gates(["h", "cx"]),
            BackendLimits::unlimited(),
            HardwareTopology::fully_connected(8).unwrap(),
        )
        .unwrap();

        let requirements =
            basic_qv_requirements()
                .require_technology(
                    QuantumTechnology::Superconducting,
                );

        let report =
            check_compatibility(&requirements, &backend)
                .unwrap();

        assert_eq!(
            report.status,
            CompatibilityStatus::CompatibleWithWarnings
        );

        assert!(report.warnings().any(|finding| {
            finding.reason
                == CompatibilityReason::UnsupportedTechnology
        }));
    }

    #[test]
    fn malformed_requirements_are_rejected_before_backend_checks() {
        let mut requirements =
            basic_qv_requirements();

        requirements.api_version =
            MAX_SUPPORTED_BENCHMARK_API_VERSION + 1;

        let backend = simulator_backend();

        let result =
            check_compatibility(&requirements, &backend);

        assert!(matches!(
            result,
            Err(
                CompatibilityError::UnsupportedBenchmarkApiVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_gate_names_are_ignored() {
        let requirements =
            GateRequirements::new()
                .with_gate("")
                .with_gate("   ");

        assert!(requirements.is_empty());
    }

    #[test]
    fn gate_names_are_normalized() {
        let requirements =
            GateRequirements::new()
                .with_gate(" CX ");

        assert_eq!(
            requirements.gates().collect::<Vec<_>>(),
            vec!["cx"]
        );
    }

    #[test]
    fn strict_compatibility_accepts_clean_backend() {
        let backend = simulator_backend();
        let requirements = basic_qv_requirements();

        assert!(
            require_strict_compatibility(
                &requirements,
                &backend
            )
            .is_ok()
        );
    }

    #[test]
    fn convenience_predicate_is_false_for_incompatible_backend() {
        let backend = simulator_backend();

        let requirements =
            basic_qv_requirements()
                .with_resources(
                    ResourceRequirements::new()
                        .with_min_qubits(1_000),
                );

        assert!(
            !is_compatible(&requirements, &backend)
        );
    }

    #[test]
    fn status_has_expected_semantics() {
        assert!(
            CompatibilityStatus::Compatible
                .is_compatible()
        );

        assert!(
            CompatibilityStatus::CompatibleWithWarnings
                .is_compatible()
        );

        assert!(
            !CompatibilityStatus::Incompatible
                .is_compatible()
        );

        assert!(
            CompatibilityStatus::Compatible
                .is_strictly_compatible()
        );

        assert!(
            !CompatibilityStatus::CompatibleWithWarnings
                .is_strictly_compatible()
        );
    }
}