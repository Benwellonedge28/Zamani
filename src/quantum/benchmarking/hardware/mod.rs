//! Zamani Quantum Benchmarking — Hardware Integration Boundary
//!
//! Production hardware-facing boundary for the quantum benchmarking system.
//!
//! This module is deliberately metadata/capability oriented.
//!
//! It DOES:
//! - describe quantum technologies;
//! - describe execution models;
//! - describe benchmark-required capabilities;
//! - describe benchmark hardware profiles;
//! - validate benchmark requirements before execution;
//! - integrate backend metadata, topology and calibration provenance;
//! - provide deterministic hardware compatibility decisions;
//! - keep vendor-specific execution out of benchmark protocols.
//!
//! It DOES NOT:
//! - execute hardware jobs;
//! - perform network I/O;
//! - store credentials;
//! - own calibration acquisition;
//! - own the canonical quantum IR;
//! - own routing;
//! - own scheduling;
//! - duplicate the authoritative topology graph;
//! - duplicate the authoritative calibration snapshot.
//!
//! Authoritative hardware modules:
//!
//! `crate::quantum::hardware::backend`
//!     Backend identity, capabilities, limits and execution boundary.
//!
//! `crate::quantum::hardware::topology`
//!     Physical connectivity and routing topology.
//!
//! `crate::quantum::hardware::calibration`
//!     Calibration snapshots and calibration measurements.
//!
//! Benchmarking consumes those abstractions; it does not replace them.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir
//!      |
//!      +--> optimization
//!      +--> routing
//!      +--> scheduling
//!      +--> error_correction
//!      |
//!      v
//! quantum::hardware
//!      |
//!      v
//! quantum::benchmarking::hardware
//!      |
//!      +--> benchmarking::execution
//!      +--> benchmarking::protocols
//!      +--> benchmarking::metrics
//!      +--> benchmarking::analysis
//! ```
//!
//! The dependency must never be reversed.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//! No nightly features are required.

use std::collections::BTreeSet;
use std::fmt;

// =============================================================================
// Authoritative hardware-layer re-exports
// =============================================================================

/// Backend abstraction owned by `quantum::hardware`.
pub use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendError,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

/// Calibration primitives owned by `quantum::hardware`.
pub use crate::quantum::hardware::calibration::{
    CalibrationSnapshot,
    CalibrationTimestamp,
    GateCalibration,
    QubitCalibration,
    ReadoutCalibration,
};

/// Topology primitives owned by `quantum::hardware`.
pub use crate::quantum::hardware::topology::{
    Connectivity,
    Coupling,
    HardwareTopology,
    QubitId,
    TopologyError,
};

// =============================================================================
// Quantum technology
// =============================================================================

/// Physical or computational technology represented by a benchmark target.
///
/// The benchmark framework must not assume that every quantum computer is a
/// gate-model qubit device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumTechnology {
    /// Superconducting gate-model processors.
    Superconducting,

    /// Trapped-ion processors.
    TrappedIon,

    /// Neutral-atom processors.
    NeutralAtom,

    /// Photonic quantum processors.
    Photonic,

    /// Semiconductor/spin qubits.
    Spin,

    /// Topological quantum computing technology.
    Topological,

    /// Quantum annealing systems.
    Annealing,

    /// Analog quantum systems.
    Analog,

    /// Other digital gate-model hardware.
    GateModelOther,

    /// General-purpose software simulator.
    Simulator,

    /// Hardware-specific software emulator.
    Emulator,

    /// Distributed/networked quantum system.
    Distributed,

    /// Research/custom technology.
    Other,
}

impl QuantumTechnology {
    /// Returns whether the technology normally exposes a circuit/gate model.
    pub const fn is_gate_model(self) -> bool {
        matches!(
            self,
            Self::Superconducting
                | Self::TrappedIon
                | Self::NeutralAtom
                | Self::Photonic
                | Self::Spin
                | Self::Topological
                | Self::GateModelOther
                | Self::Simulator
                | Self::Emulator
        )
    }

    /// Returns whether the technology represents physical hardware rather
    /// than software-only simulation/emulation.
    pub const fn is_physical(self) -> bool {
        !matches!(self, Self::Simulator | Self::Emulator)
    }
}

// =============================================================================
// Execution model
// =============================================================================

/// Primary execution model exposed by a benchmark target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumExecutionModel {
    /// Conventional digital quantum circuits.
    GateModel,

    /// Continuous/analog quantum evolution.
    Analog,

    /// Quantum annealing/adiabatic optimization.
    Annealing,

    /// Sampling-first execution.
    Sampling,

    /// Logical/fault-tolerant quantum execution.
    Logical,

    /// Distributed quantum execution.
    Distributed,
}

impl QuantumExecutionModel {
    /// Returns whether a canonical circuit is required.
    pub const fn requires_circuit(self) -> bool {
        matches!(self, Self::GateModel | Self::Logical)
    }
}

// =============================================================================
// Benchmark capability vocabulary
// =============================================================================

/// Fine-grained capabilities that a benchmark may require.
///
/// This is deliberately separate from `BackendCapabilities`.
///
/// `BackendCapabilities` describes the general backend contract.
/// `HardwareCapability` describes the vocabulary used by benchmarking
/// protocols to negotiate compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareCapability {
    GateExecution,
    Measurement,
    Reset,
    MidCircuitMeasurement,
    ClassicalControl,
    DynamicCircuits,
    ParameterizedGates,
    ArbitrarySingleQubitRotations,
    TwoQubitGates,
    ParallelOperations,
    ConditionalOperations,
    StateVectorAccess,
    DensityMatrixAccess,
    AmplitudeAccess,
    ExpectationValues,
    AnalogControl,
    Annealing,
    LogicalQubits,
    ErrorCorrection,
    SyndromeExtraction,
    DecoderExecution,
    PulseControl,
    TimingMetadata,
    CalibrationMetadata,
    TopologyMetadata,
    DeterministicSeeding,
}

impl HardwareCapability {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateExecution => "gate_execution",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::ClassicalControl => "classical_control",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::ParameterizedGates => "parameterized_gates",
            Self::ArbitrarySingleQubitRotations => {
                "arbitrary_single_qubit_rotations"
            }
            Self::TwoQubitGates => "two_qubit_gates",
            Self::ParallelOperations => "parallel_operations",
            Self::ConditionalOperations => "conditional_operations",
            Self::StateVectorAccess => "state_vector_access",
            Self::DensityMatrixAccess => "density_matrix_access",
            Self::AmplitudeAccess => "amplitude_access",
            Self::ExpectationValues => "expectation_values",
            Self::AnalogControl => "analog_control",
            Self::Annealing => "annealing",
            Self::LogicalQubits => "logical_qubits",
            Self::ErrorCorrection => "error_correction",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::DecoderExecution => "decoder_execution",
            Self::PulseControl => "pulse_control",
            Self::TimingMetadata => "timing_metadata",
            Self::CalibrationMetadata => "calibration_metadata",
            Self::TopologyMetadata => "topology_metadata",
            Self::DeterministicSeeding => "deterministic_seeding",
        }
    }
}

/// Deterministic set of benchmark hardware capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilitySet {
    capabilities: BTreeSet<HardwareCapability>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a capability set from an iterator.
    pub fn from_iter<I>(capabilities: I) -> Self
    where
        I: IntoIterator<Item = HardwareCapability>,
    {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }

    /// Adds one capability.
    pub fn insert(&mut self, capability: HardwareCapability) -> bool {
        self.capabilities.insert(capability)
    }

    /// Builder-style capability insertion.
    pub fn with(mut self, capability: HardwareCapability) -> Self {
        self.insert(capability);
        self
    }

    /// Returns whether a capability exists.
    pub fn contains(&self, capability: HardwareCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns the number of capabilities.
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Returns capabilities in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = HardwareCapability> + '_ {
        self.capabilities.iter().copied()
    }

    /// Returns capabilities as a deterministic vector.
    pub fn to_vec(&self) -> Vec<HardwareCapability> {
        self.iter().collect()
    }
}

// =============================================================================
// Benchmark hardware requirements
// =============================================================================

/// Hardware requirements declared by a benchmark protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareRequirements {
    /// Required execution model.
    pub execution_model: Option<QuantumExecutionModel>,

    /// Required physical technology.
    pub technology: Option<QuantumTechnology>,

    /// Required capabilities.
    pub capabilities: CapabilitySet,

    /// Minimum physical qubit count.
    pub min_physical_qubits: Option<usize>,

    /// Minimum logical qubit count.
    pub min_logical_qubits: Option<usize>,

    /// Required circuit depth.
    ///
    /// Despite the historical `max_*` naming, this represents the maximum
    /// depth the requested benchmark workload needs to execute.
    pub max_circuit_depth: Option<usize>,

    /// Required operation capacity.
    pub max_operations: Option<usize>,

    /// Required shots per execution request.
    pub max_shots: Option<usize>,

    /// Required native gates.
    pub required_gates: BTreeSet<String>,

    /// Required physical connections.
    ///
    /// Connections are stored canonically as `(min, max)`. Exact directed
    /// validation is performed against `HardwareTopology`.
    pub required_connections: BTreeSet<(usize, usize)>,
}

impl Default for HardwareRequirements {
    fn default() -> Self {
        Self {
            execution_model: None,
            technology: None,
            capabilities: CapabilitySet::new(),
            min_physical_qubits: None,
            min_logical_qubits: None,
            max_circuit_depth: None,
            max_operations: None,
            max_shots: None,
            required_gates: BTreeSet::new(),
            required_connections: BTreeSet::new(),
        }
    }
}

impl HardwareRequirements {
    /// Creates unconstrained requirements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requires an execution model.
    pub fn with_execution_model(
        mut self,
        model: QuantumExecutionModel,
    ) -> Self {
        self.execution_model = Some(model);
        self
    }

    /// Requires a technology.
    pub fn with_technology(
        mut self,
        technology: QuantumTechnology,
    ) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Requires one capability.
    pub fn require_capability(
        mut self,
        capability: HardwareCapability,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Requires minimum physical capacity.
    pub fn with_min_physical_qubits(
        mut self,
        value: usize,
    ) -> Self {
        self.min_physical_qubits = Some(value);
        self
    }

    /// Requires minimum logical capacity.
    pub fn with_min_logical_qubits(
        mut self,
        value: usize,
    ) -> Self {
        self.min_logical_qubits = Some(value);
        self
    }

    /// Declares the maximum circuit depth needed by the workload.
    pub fn with_max_circuit_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_circuit_depth = Some(value);
        self
    }

    /// Declares the maximum operation count needed.
    pub fn with_max_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operations = Some(value);
        self
    }

    /// Declares the maximum shots needed per request.
    pub fn with_max_shots(
        mut self,
        value: usize,
    ) -> Self {
        self.max_shots = Some(value);
        self
    }

    /// Requires a native gate.
    pub fn require_gate(
        mut self,
        gate: impl Into<String>,
    ) -> Self {
        self.required_gates
            .insert(normalize_gate_name(&gate.into()));
        self
    }

    /// Requires a connection.
    pub fn require_connection(
        mut self,
        source: usize,
        target: usize,
    ) -> Self {
        self.required_connections
            .insert(canonical_edge(source, target));
        self
    }
}

// =============================================================================
// Compatibility diagnostics
// =============================================================================

/// A single missing or incompatible hardware requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityGap {
    /// Backend is not currently usable.
    BackendUnavailable {
        status: BackendStatus,
    },

    /// Execution model mismatch.
    ExecutionModel {
        required: QuantumExecutionModel,
        actual: QuantumExecutionModel,
    },

    /// Technology mismatch.
    Technology {
        required: QuantumTechnology,
        actual: QuantumTechnology,
    },

    /// Missing capability.
    Capability(HardwareCapability),

    /// Insufficient physical qubits.
    PhysicalQubitCount {
        required: usize,
        available: usize,
    },

    /// Insufficient logical qubits.
    LogicalQubitCount {
        required: usize,
        available: usize,
    },

    /// Insufficient circuit depth capacity.
    CircuitDepth {
        required: usize,
        supported: usize,
    },

    /// Insufficient operation capacity.
    Operations {
        required: usize,
        supported: usize,
    },

    /// Insufficient shot capacity.
    Shots {
        required: usize,
        supported: usize,
    },

    /// Missing native gate.
    Gate {
        gate: String,
    },

    /// Missing physical connection.
    Connection {
        source: usize,
        target: usize,
    },
}

impl fmt::Display for CapabilityGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable { status } => {
                write!(f, "backend is unavailable ({status:?})")
            }

            Self::ExecutionModel { required, actual } => {
                write!(
                    f,
                    "requires execution model {required:?}, \
                     backend exposes {actual:?}"
                )
            }

            Self::Technology { required, actual } => {
                write!(
                    f,
                    "requires technology {required:?}, \
                     backend exposes {actual:?}"
                )
            }

            Self::Capability(capability) => {
                write!(
                    f,
                    "missing capability `{}`",
                    capability.as_str()
                )
            }

            Self::PhysicalQubitCount {
                required,
                available,
            } => {
                write!(
                    f,
                    "requires {required} physical qubits, \
                     only {available} available"
                )
            }

            Self::LogicalQubitCount {
                required,
                available,
            } => {
                write!(
                    f,
                    "requires {required} logical qubits, \
                     only {available} available"
                )
            }

            Self::CircuitDepth {
                required,
                supported,
            } => {
                write!(
                    f,
                    "requires circuit depth {required}, \
                     backend supports {supported}"
                )
            }

            Self::Operations {
                required,
                supported,
            } => {
                write!(
                    f,
                    "requires {required} operations, \
                     backend supports {supported}"
                )
            }

            Self::Shots {
                required,
                supported,
            } => {
                write!(
                    f,
                    "requires {required} shots, \
                     backend supports {supported}"
                )
            }

            Self::Gate { gate } => {
                write!(
                    f,
                    "required native gate `{gate}` is unavailable"
                )
            }

            Self::Connection { source, target } => {
                write!(
                    f,
                    "required hardware connection \
                     {source} -> {target} is unavailable"
                )
            }
        }
    }
}

/// Result of benchmark/backend compatibility validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareValidation {
    /// True only when no compatibility gap exists.
    pub supported: bool,

    /// Deterministically ordered compatibility gaps.
    pub gaps: Vec<CapabilityGap>,
}

impl HardwareValidation {
    /// Creates a successful validation result.
    pub fn supported() -> Self {
        Self {
            supported: true,
            gaps: Vec::new(),
        }
    }

    /// Creates a validation result from gaps.
    pub fn unsupported(gaps: Vec<CapabilityGap>) -> Self {
        Self {
            supported: gaps.is_empty(),
            gaps,
        }
    }

    /// Returns whether the requirements are fully satisfied.
    pub fn is_supported(&self) -> bool {
        self.supported && self.gaps.is_empty()
    }
}

/// Typed compatibility error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareCompatibilityError {
    /// Backend against which validation failed.
    pub backend_id: String,

    /// All missing/incompatible requirements.
    pub gaps: Vec<CapabilityGap>,
}

impl fmt::Display for HardwareCompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "benchmark is incompatible with backend `{}`",
            self.backend_id
        )?;

        if !self.gaps.is_empty() {
            write!(f, ": ")?;

            for (index, gap) in self.gaps.iter().enumerate() {
                if index != 0 {
                    write!(f, "; ")?;
                }

                write!(f, "{gap}")?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for HardwareCompatibilityError {}

// =============================================================================
// Benchmark hardware profile
// =============================================================================

/// Immutable hardware description consumed by benchmark protocols.
///
/// This is a benchmark-facing view of a backend. It deliberately contains
/// no credentials, network clients, job handles, mutable execution state or
/// duplicated topology graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkHardwareProfile {
    /// Stable backend identifier.
    pub backend_id: String,

    /// Human-readable backend name.
    pub backend_name: String,

    /// Provider/organization identifier.
    pub provider: String,

    /// Provider/backend software or firmware version.
    pub version: String,

    /// Generic backend kind.
    pub kind: BackendKind,

    /// Physical quantum technology.
    pub technology: QuantumTechnology,

    /// Primary execution model.
    pub execution_model: QuantumExecutionModel,

    /// Backend status captured at profile creation.
    pub status: BackendStatus,

    /// Physical qubit capacity.
    pub physical_qubits: usize,

    /// Logical qubit capacity, when exposed.
    pub logical_qubits: usize,

    /// Maximum circuit depth supported, when bounded.
    pub max_circuit_depth: Option<usize>,

    /// Maximum operation count supported, when bounded.
    pub max_operations: Option<usize>,

    /// Maximum shots supported per request, when bounded.
    pub max_shots: Option<usize>,

    /// Benchmark capability set.
    pub capabilities: CapabilitySet,

    /// Native gate set.
    pub native_gates: BTreeSet<String>,

    /// Stable topology fingerprint.
    pub topology_fingerprint: Option<String>,

    /// Stable calibration fingerprint.
    pub calibration_fingerprint: Option<String>,

    /// Calibration timestamp.
    pub calibration_timestamp: Option<CalibrationTimestamp>,

    /// Immutable backend-specific properties.
    pub properties: Vec<(String, String)>,
}

impl BenchmarkHardwareProfile {
    /// Creates a profile with explicit metadata.
    pub fn new(
        backend_id: impl Into<String>,
        backend_name: impl Into<String>,
        provider: impl Into<String>,
        version: impl Into<String>,
        kind: BackendKind,
        technology: QuantumTechnology,
        execution_model: QuantumExecutionModel,
        physical_qubits: usize,
    ) -> Result<Self, HardwareProfileError> {
        let backend_id = backend_id.into();

        if backend_id.trim().is_empty() {
            return Err(HardwareProfileError::InvalidBackendId);
        }

        if physical_qubits == 0 {
            return Err(HardwareProfileError::ZeroPhysicalQubits);
        }

        Ok(Self {
            backend_id,
            backend_name: backend_name.into(),
            provider: provider.into(),
            version: version.into(),
            kind,
            technology,
            execution_model,
            status: BackendStatus::Available,
            physical_qubits,
            logical_qubits: 0,
            max_circuit_depth: None,
            max_operations: None,
            max_shots: None,
            capabilities: CapabilitySet::new(),
            native_gates: BTreeSet::new(),
            topology_fingerprint: None,
            calibration_fingerprint: None,
            calibration_timestamp: None,
            properties: Vec::new(),
        })
    }

    /// Builds a benchmark profile from the authoritative backend abstraction.
    ///
    /// Technology and execution model are supplied explicitly because the
    /// generic backend abstraction intentionally does not guess them.
    pub fn from_backend(
        backend: &QuantumBackend,
        technology: QuantumTechnology,
        execution_model: QuantumExecutionModel,
    ) -> Result<Self, HardwareProfileError> {
        let physical_qubits = backend.qubit_count();

        let mut profile = Self::new(
            backend.metadata.id.clone(),
            backend.metadata.name.clone(),
            backend.metadata.provider.clone(),
            backend.metadata.version.clone(),
            backend.metadata.kind,
            technology,
            execution_model,
            physical_qubits,
        )?;

        profile.status = backend.metadata.status;

        if backend.limits.max_circuit_depth != 0 {
            profile.max_circuit_depth =
                Some(backend.limits.max_circuit_depth);
        }

        if backend.limits.max_operations != 0 {
            profile.max_operations =
                Some(backend.limits.max_operations);
        }

        if backend.limits.max_shots != 0 {
            profile.max_shots =
                Some(backend.limits.max_shots);
        }

        for gate in backend.native_gates() {
            profile
                .native_gates
                .insert(normalize_gate_name(&gate));
        }

        if !profile.native_gates.is_empty() {
            profile
                .capabilities
                .insert(HardwareCapability::GateExecution);
        }

        if backend.capabilities.measurement {
            profile
                .capabilities
                .insert(HardwareCapability::Measurement);
        }

        if backend.capabilities.reset {
            profile
                .capabilities
                .insert(HardwareCapability::Reset);
        }

        if backend.capabilities.mid_circuit_measurement {
            profile
                .capabilities
                .insert(HardwareCapability::MidCircuitMeasurement);
        }

        if backend.capabilities.classical_control {
            profile
                .capabilities
                .insert(HardwareCapability::ClassicalControl);
        }

        if backend.capabilities.dynamic_circuits {
            profile
                .capabilities
                .insert(HardwareCapability::DynamicCircuits);
        }

        if backend.capabilities.parameterized_gates {
            profile
                .capabilities
                .insert(HardwareCapability::ParameterizedGates);
        }

        if backend.capabilities.arbitrary_single_qubit_rotations {
            profile
                .capabilities
                .insert(
                    HardwareCapability::ArbitrarySingleQubitRotations,
                );
        }

        profile
            .capabilities
            .insert(HardwareCapability::TopologyMetadata);

        for (key, value) in &backend.metadata.properties {
            profile
                .properties
                .push((key.clone(), value.clone()));
        }

        profile.properties.sort();

        Ok(profile)
    }

    /// Adds one capability.
    pub fn with_capability(
        mut self,
        capability: HardwareCapability,
    ) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds one native gate.
    pub fn with_native_gate(
        mut self,
        gate: impl Into<String>,
    ) -> Self {
        self.native_gates
            .insert(normalize_gate_name(&gate.into()));
        self
    }

    /// Adds multiple native gates.
    pub fn with_native_gates<I, S>(
        mut self,
        gates: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for gate in gates {
            self.native_gates
                .insert(normalize_gate_name(&gate.into()));
        }

        if !self.native_gates.is_empty() {
            self.capabilities
                .insert(HardwareCapability::GateExecution);
        }

        self
    }

    /// Sets logical qubit capacity.
    pub fn with_logical_qubits(
        mut self,
        value: usize,
    ) -> Self {
        self.logical_qubits = value;

        if value > 0 {
            self.capabilities
                .insert(HardwareCapability::LogicalQubits);
        }

        self
    }

    /// Sets maximum circuit depth.
    pub fn with_max_circuit_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_circuit_depth = Some(value);
        self
    }

    /// Sets maximum operation capacity.
    pub fn with_max_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operations = Some(value);
        self
    }

    /// Sets maximum shots per request.
    pub fn with_max_shots(
        mut self,
        value: usize,
    ) -> Self {
        self.max_shots = Some(value);
        self
    }

    /// Sets topology fingerprint.
    pub fn with_topology_fingerprint(
        mut self,
        fingerprint: impl Into<String>,
    ) -> Self {
        self.topology_fingerprint = Some(fingerprint.into());
        self
    }

    /// Attaches calibration provenance.
    pub fn with_calibration(
        mut self,
        fingerprint: impl Into<String>,
        timestamp: CalibrationTimestamp,
    ) -> Self {
        self.calibration_fingerprint = Some(fingerprint.into());
        self.calibration_timestamp = Some(timestamp);

        self.capabilities
            .insert(HardwareCapability::CalibrationMetadata);

        self
    }

    /// Sets backend status.
    pub fn with_status(
        mut self,
        status: BackendStatus,
    ) -> Self {
        self.status = status;
        self
    }

    /// Adds immutable metadata.
    pub fn with_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.properties.push((key.into(), value.into()));
        self.properties.sort();
        self
    }

    /// Performs capability-only validation.
    ///
    /// Topology edge validation is intentionally separate because
    /// `HardwareTopology` remains the single source of truth.
    pub fn check(
        &self,
        requirements: &HardwareRequirements,
    ) -> HardwareValidation {
        let mut gaps = Vec::new();

        if !self.status.is_usable() {
            gaps.push(CapabilityGap::BackendUnavailable {
                status: self.status,
            });
        }

        if let Some(required) = requirements.execution_model {
            if required != self.execution_model {
                gaps.push(CapabilityGap::ExecutionModel {
                    required,
                    actual: self.execution_model,
                });
            }
        }

        if let Some(required) = requirements.technology {
            if required != self.technology {
                gaps.push(CapabilityGap::Technology {
                    required,
                    actual: self.technology,
                });
            }
        }

        for capability in requirements.capabilities.iter() {
            if !self.capabilities.contains(capability) {
                gaps.push(CapabilityGap::Capability(capability));
            }
        }

        if let Some(required) =
            requirements.min_physical_qubits
        {
            if self.physical_qubits < required {
                gaps.push(
                    CapabilityGap::PhysicalQubitCount {
                        required,
                        available: self.physical_qubits,
                    },
                );
            }
        }

        if let Some(required) =
            requirements.min_logical_qubits
        {
            if self.logical_qubits < required {
                gaps.push(
                    CapabilityGap::LogicalQubitCount {
                        required,
                        available: self.logical_qubits,
                    },
                );
            }
        }

        if let Some(required) =
            requirements.max_circuit_depth
        {
            if let Some(supported) =
                self.max_circuit_depth
            {
                if required > supported {
                    gaps.push(
                        CapabilityGap::CircuitDepth {
                            required,
                            supported,
                        },
                    );
                }
            }
        }

        if let Some(required) =
            requirements.max_operations
        {
            if let Some(supported) =
                self.max_operations
            {
                if required > supported {
                    gaps.push(
                        CapabilityGap::Operations {
                            required,
                            supported,
                        },
                    );
                }
            }
        }

        if let Some(required) =
            requirements.max_shots
        {
            if let Some(supported) =
                self.max_shots
            {
                if required > supported {
                    gaps.push(CapabilityGap::Shots {
                        required,
                        supported,
                    });
                }
            }
        }

        for gate in &requirements.required_gates {
            if !self.native_gates.contains(gate) {
                gaps.push(CapabilityGap::Gate {
                    gate: gate.clone(),
                });
            }
        }

        sort_capability_gaps(&mut gaps);

        HardwareValidation::unsupported(gaps)
    }

    /// Performs capability validation and returns a typed error on failure.
    pub fn require(
        &self,
        requirements: &HardwareRequirements,
    ) -> Result<(), HardwareCompatibilityError> {
        let validation = self.check(requirements);

        if validation.is_supported() {
            return Ok(());
        }

        Err(HardwareCompatibilityError {
            backend_id: self.backend_id.clone(),
            gaps: validation.gaps,
        })
    }

    /// Returns native gates in deterministic order.
    pub fn native_gate_list(&self) -> Vec<String> {
        self.native_gates.iter().cloned().collect()
    }
}

// =============================================================================
// Profile construction errors
// =============================================================================

/// Errors produced while constructing a benchmark hardware profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareProfileError {
    InvalidBackendId,
    ZeroPhysicalQubits,
}

impl fmt::Display for HardwareProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBackendId => {
                write!(
                    f,
                    "benchmark hardware backend ID cannot be empty"
                )
            }

            Self::ZeroPhysicalQubits => {
                write!(
                    f,
                    "benchmark hardware profile must expose \
                     at least one physical qubit"
                )
            }
        }
    }
}

impl std::error::Error for HardwareProfileError {}

// =============================================================================
// Topology integration
// =============================================================================

/// Validates benchmark-required connections against the authoritative
/// `HardwareTopology`.
///
/// No topology state is copied into `BenchmarkHardwareProfile`.
pub fn validate_required_connections(
    topology: &HardwareTopology,
    requirements: &HardwareRequirements,
) -> HardwareValidation {
    let mut gaps = Vec::new();

    for &(source, target) in &requirements.required_connections {
        match topology.is_connected(source, target) {
            Ok(true) => {}

            Ok(false) | Err(_) => {
                gaps.push(CapabilityGap::Connection {
                    source,
                    target,
                });
            }
        }
    }

    sort_capability_gaps(&mut gaps);

    HardwareValidation::unsupported(gaps)
}

/// Performs both profile and topology validation.
pub fn validate_hardware_requirements(
    profile: &BenchmarkHardwareProfile,
    topology: &HardwareTopology,
    requirements: &HardwareRequirements,
) -> HardwareValidation {
    let mut gaps = profile.check(requirements).gaps;

    gaps.extend(
        validate_required_connections(
            topology,
            requirements,
        )
        .gaps,
    );

    sort_capability_gaps(&mut gaps);

    HardwareValidation::unsupported(gaps)
}

/// Performs validation and returns a typed error if incompatible.
pub fn require_hardware_requirements(
    profile: &BenchmarkHardwareProfile,
    topology: &HardwareTopology,
    requirements: &HardwareRequirements,
) -> Result<(), HardwareCompatibilityError> {
    let validation =
        validate_hardware_requirements(
            profile,
            topology,
            requirements,
        );

    if validation.is_supported() {
        return Ok(());
    }

    Err(HardwareCompatibilityError {
        backend_id: profile.backend_id.clone(),
        gaps: validation.gaps,
    })
}

// =============================================================================
// Calibration integration
// =============================================================================

/// Attaches calibration provenance without copying the complete calibration
/// dataset into the benchmark profile.
pub fn attach_calibration_provenance(
    profile: &mut BenchmarkHardwareProfile,
    fingerprint: impl Into<String>,
    timestamp: CalibrationTimestamp,
) {
    profile.calibration_fingerprint =
        Some(fingerprint.into());

    profile.calibration_timestamp =
        Some(timestamp);

    profile
        .capabilities
        .insert(HardwareCapability::CalibrationMetadata);
}

// =============================================================================
// Canonicalization
// =============================================================================

fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_uppercase()
}

fn canonical_edge(
    source: usize,
    target: usize,
) -> (usize, usize) {
    if source <= target {
        (source, target)
    } else {
        (target, source)
    }
}

fn sort_capability_gaps(
    gaps: &mut Vec<CapabilityGap>,
) {
    gaps.sort_by(|a, b| {
        capability_gap_key(a)
            .cmp(&capability_gap_key(b))
    });
}

fn capability_gap_key(
    gap: &CapabilityGap,
) -> String {
    match gap {
        CapabilityGap::BackendUnavailable { status } => {
            format!("00:{status:?}")
        }

        CapabilityGap::ExecutionModel {
            required,
            actual,
        } => {
            format!("01:{required:?}:{actual:?}")
        }

        CapabilityGap::Technology {
            required,
            actual,
        } => {
            format!("02:{required:?}:{actual:?}")
        }

        CapabilityGap::Capability(capability) => {
            format!("03:{}", capability.as_str())
        }

        CapabilityGap::PhysicalQubitCount {
            required,
            available,
        } => {
            format!("04:{required}:{available}")
        }

        CapabilityGap::LogicalQubitCount {
            required,
            available,
        } => {
            format!("05:{required}:{available}")
        }

        CapabilityGap::CircuitDepth {
            required,
            supported,
        } => {
            format!("06:{required}:{supported}")
        }

        CapabilityGap::Operations {
            required,
            supported,
        } => {
            format!("07:{required}:{supported}")
        }

        CapabilityGap::Shots {
            required,
            supported,
        } => {
            format!("08:{required}:{supported}")
        }

        CapabilityGap::Gate { gate } => {
            format!("09:{gate}")
        }

        CapabilityGap::Connection {
            source,
            target,
        } => {
            format!("10:{source}:{target}")
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn simulator_backend() -> QuantumBackend {
        let metadata = BackendMetadata::new(
            "test-simulator",
            "Test Simulator",
            "Zamani",
            "1.0.0",
            BackendKind::Simulator,
        );

        let capabilities =
            BackendCapabilities::new()
                .with_gates([
                    "H",
                    "X",
                    "Y",
                    "Z",
                    "CX",
                    "CZ",
                    "SWAP",
                ]);

        let topology =
            HardwareTopology::linear(4)
                .expect("linear topology");

        QuantumBackend::new(
            metadata,
            capabilities,
            BackendLimits::unlimited(),
            topology,
        )
        .expect("backend should be valid")
    }

    #[test]
    fn capability_set_is_deterministic() {
        let set = CapabilitySet::new()
            .with(HardwareCapability::Measurement)
            .with(HardwareCapability::GateExecution)
            .with(HardwareCapability::Reset);

        assert_eq!(set.len(), 3);

        assert!(
            set.contains(
                HardwareCapability::Measurement
            )
        );

        assert_eq!(set.to_vec().len(), 3);
    }

    #[test]
    fn requirements_normalize_gate_names() {
        let requirements =
            HardwareRequirements::new()
                .require_gate(" cx ")
                .require_gate("H");

        assert!(
            requirements
                .required_gates
                .contains("CX")
        );

        assert!(
            requirements
                .required_gates
                .contains("H")
        );
    }

    #[test]
    fn profile_can_be_built_from_backend() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .expect("profile should be valid");

        assert_eq!(
            profile.backend_id,
            "test-simulator"
        );

        assert_eq!(
            profile.physical_qubits,
            4
        );

        assert!(
            profile.native_gates.contains("CX")
        );

        assert!(
            profile.capabilities.contains(
                HardwareCapability::Measurement
            )
        );
    }

    #[test]
    fn compatible_profile_passes_requirements() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .unwrap()
            .with_capability(
                HardwareCapability::DeterministicSeeding,
            );

        let requirements =
            HardwareRequirements::new()
                .with_execution_model(
                    QuantumExecutionModel::GateModel,
                )
                .with_technology(
                    QuantumTechnology::Simulator,
                )
                .with_min_physical_qubits(4)
                .require_capability(
                    HardwareCapability::Measurement,
                )
                .require_gate("cx");

        assert!(
            profile
                .require(&requirements)
                .is_ok()
        );
    }

    #[test]
    fn unavailable_backend_is_rejected() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .unwrap()
            .with_status(
                BackendStatus::Offline,
            );

        let requirements =
            HardwareRequirements::new();

        let validation =
            profile.check(&requirements);

        assert!(!validation.is_supported());

        assert!(validation.gaps.iter().any(
            |gap| matches!(
                gap,
                CapabilityGap::BackendUnavailable {
                    status: BackendStatus::Offline
                }
            )
        ));
    }

    #[test]
    fn missing_capability_is_reported() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .unwrap();

        let requirements =
            HardwareRequirements::new()
                .require_capability(
                    HardwareCapability::PulseControl,
                );

        let validation =
            profile.check(&requirements);

        assert!(
            !validation.is_supported()
        );

        assert!(validation.gaps.iter().any(
            |gap| matches!(
                gap,
                CapabilityGap::Capability(
                    HardwareCapability::PulseControl
                )
            )
        ));
    }

    #[test]
    fn topology_is_validated_by_authoritative_topology() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .unwrap();

        let requirements =
            HardwareRequirements::new()
                .require_connection(0, 1)
                .require_connection(0, 3);

        let validation =
            validate_hardware_requirements(
                &profile,
                &backend.topology,
                &requirements,
            );

        assert!(
            !validation.is_supported()
        );

        assert!(
            validation.gaps.iter().any(
                |gap| matches!(
                    gap,
                    CapabilityGap::Connection {
                        source: 0,
                        target: 3
                    }
                )
            )
        );
    }

    #[test]
    fn calibration_provenance_is_explicit() {
        let mut profile =
            BenchmarkHardwareProfile::new(
                "backend",
                "Backend",
                "Provider",
                "1",
                BackendKind::Qpu,
                QuantumTechnology::Superconducting,
                QuantumExecutionModel::GateModel,
                5,
            )
            .unwrap();

        let timestamp =
            CalibrationTimestamp::from_unix_nanos(42);

        attach_calibration_provenance(
            &mut profile,
            "sha256:abc",
            timestamp,
        );

        assert_eq!(
            profile.calibration_fingerprint
                .as_deref(),
            Some("sha256:abc")
        );

        assert_eq!(
            profile.calibration_timestamp,
            Some(timestamp)
        );

        assert!(
            profile.capabilities.contains(
                HardwareCapability::CalibrationMetadata
            )
        );
    }

    #[test]
    fn technology_model_helpers_are_stable() {
        assert!(
            QuantumTechnology::Superconducting
                .is_gate_model()
        );

        assert!(
            QuantumTechnology::Superconducting
                .is_physical()
        );

        assert!(
            QuantumTechnology::Simulator
                .is_gate_model()
        );

        assert!(
            !QuantumTechnology::Simulator
                .is_physical()
        );

        assert!(
            !QuantumExecutionModel::Analog
                .requires_circuit()
        );

        assert!(
            QuantumExecutionModel::GateModel
                .requires_circuit()
        );
    }

    #[test]
    fn native_gate_list_is_deterministic() {
        let backend = simulator_backend();

        let profile =
            BenchmarkHardwareProfile::from_backend(
                &backend,
                QuantumTechnology::Simulator,
                QuantumExecutionModel::GateModel,
            )
            .unwrap();

        let gates =
            profile.native_gate_list();

        let mut sorted = gates.clone();
        sorted.sort();

        assert_eq!(gates, sorted);
    }
}