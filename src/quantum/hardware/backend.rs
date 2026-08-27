//! Zamani Quantum — Canonical Hardware Backend
//!
//! Production-grade, provider-independent hardware backend contract.
//!
//! # Responsibility
//!
//! This module owns the backend aggregate and the provider-neutral contract
//! describing a quantum execution target.
//!
//! It is responsible for:
//!
//! - stable backend identity;
//! - backend kind and operational status;
//! - hardware capability declaration;
//! - resource limits;
//! - workload requirements;
//! - backend metadata;
//! - deterministic capability matching;
//! - hardware-topology validation;
//! - pre-execution validation;
//! - execution request/result data contracts;
//! - backend-independent execution abstraction;
//! - compatibility with simulators, emulators, QPUs and future quantum
//!   execution technologies;
//! - deterministic public behaviour;
//! - provider-independent error classification.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - communicate with IBM;
//! - communicate with IonQ;
//! - communicate with AWS Braket;
//! - communicate with Rigetti;
//! - communicate with IQM;
//! - communicate with Quantinuum;
//! - communicate with QuEra;
//! - own provider credentials;
//! - perform authentication;
//! - perform HTTP/network I/O;
//! - perform transpilation;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration experiments;
//! - own calibration state;
//! - perform benchmarking mathematics;
//! - implement QEC algorithms;
//! - implement OpenQASM parsing;
//! - implement QIR;
//! - implement a quantum simulator;
//! - implement a hardware emulator.
//!
//! Provider adapters, execution engines and higher-level subsystems consume
//! this contract.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum IR
//!      |
//!      +------------------------------+
//!      |                              |
//!      v                              v
//! optimization                   error correction
//!      |                              |
//!      +---------------+--------------+
//!                      |
//!                      v
//!             compatibility analysis
//!                      |
//!             +--------+---------+
//!             |                  |
//!             v                  v
//!          routing           scheduling
//!             |                  |
//!             +--------+---------+
//!                      |
//!                      v
//!               Quantum Workload
//!                      |
//!                      v
//!              Hardware Backend
//!                      |
//!          +-----------+-----------+
//!          |           |           |
//!          v           v           v
//!        local       provider    simulator
//!       adapter      adapter     adapter
//!          |           |           |
//!          +-----------+-----------+
//!                      |
//!                      v
//!                  execution
//!
//! benchmarking consumes this boundary.
//! hardware does not depend on benchmarking.
//! ```
//!
//! # Integration contract
//!
//! This file is deliberately usable before the later hardware files are
//! introduced.
//!
//! It depends only on:
//!
//! - the Rust standard library;
//! - `hardware::topology`.
//!
//! Later modules may consume this file without changing its public contract:
//!
//! - `backend_trait.rs`
//! - `backend_config.rs`
//! - `backend_status.rs`
//! - `capabilities.rs`
//! - `compatibility.rs`
//! - `validation.rs`
//! - `execution.rs`
//! - `job.rs`
//! - `provider.rs`
//! - `provider_registry.rs`
//! - provider adapters;
//! - benchmarking.
//!
//! Those modules must adapt to this contract rather than requiring this file
//! to know provider-specific details.
//!
//! # Stability rule
//!
//! Public types in this file form the backend compatibility surface.
//!
//! New hardware technologies should normally extend:
//!
//! - `QuantumWorkloadKind`;
//! - `BackendCapabilities`;
//! - `BackendLimits`;
//! - `BackendMetadata`;
//! - provider adapters;
//!
//! rather than changing the semantics of existing fields.
//!
//! # Security
//!
//! Backend metadata MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authentication headers;
//! - session cookies;
//! - credentials.
//!
//! Credentials belong to the credentials/authentication subsystem.
//!
//! # Determinism
//!
//! Deterministic collections are used wherever externally observable ordering
//! matters. Native gates and metadata use `BTree*` collections.
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
//! # Topology integration
//!
//! `HardwareTopology` is authoritative for physical connectivity. This file
//! never accesses topology implementation fields directly.
//!
//! In particular, topology connectivity MUST be queried through:
//!
//! - `qubit_count()`;
//! - `is_connected()`;
//! - `is_physically_adjacent()`;
//! - `couplings()`;
//! - `contains()`.
//!
//! This preserves the topology module's encapsulation and allows topology
//! implementation changes without rewriting backend semantics.
//!
//! # External interoperability
//!
//! The model intentionally accommodates concepts exposed by current quantum
//! hardware ecosystems:
//!
//! - dynamic circuits;
//! - classical feed-forward;
//! - measurement/reset;
//! - instruction duration and error;
//! - calibration snapshots;
//! - queues;
//! - QPU/simulator distinction;
//! - pulse-level workloads;
//! - analog workloads;
//! - annealing workloads;
//! - logical/fault-tolerant workloads;
//! - heterogeneous future quantum resources.
//!
//! OpenQASM and QIR remain interoperability/compilation layers outside this
//! module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::topology::{HardwareTopology, TopologyError};

// =============================================================================
// Schema
// =============================================================================

/// Stable backend schema identifier.
pub const BACKEND_SCHEMA_ID: &str = "zamani.quantum.hardware.backend";

/// Backend semantic schema version.
///
/// Increment when serialized/public semantics change incompatibly.
pub const BACKEND_SCHEMA_VERSION: u16 = 2;

/// Maximum backend identifier length in bytes.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum backend name length in bytes.
pub const MAX_BACKEND_NAME_LENGTH: usize = 512;

/// Maximum provider identifier length in bytes.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend version length in bytes.
pub const MAX_BACKEND_VERSION_LENGTH: usize = 128;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of metadata properties.
pub const MAX_METADATA_PROPERTIES: usize = 4096;

/// Maximum number of native instructions.
pub const MAX_NATIVE_INSTRUCTIONS: usize = 1_000_000;

/// Maximum number of gates/instructions in one workload requirement set.
pub const MAX_REQUIRED_INSTRUCTIONS: usize = 1_000_000;

/// Maximum number of topology edges in one workload requirement set.
pub const MAX_REQUIRED_EDGES: usize = 10_000_000;

// =============================================================================
// Backend kind
// =============================================================================

/// High-level kind of execution target.
///
/// This is deliberately distinct from physical technology.
///
/// For example:
///
/// ```text
/// technology = superconducting
/// kind       = Qpu
/// workload   = GateCircuit
/// ```
///
/// or:
///
/// ```text
/// technology = neutral_atom
/// kind       = Qpu
/// workload   = Analog
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendKind {
    /// Classical software simulator implementing an abstract quantum model.
    Simulator,

    /// Software emulator approximating a particular hardware architecture.
    Emulator,

    /// Physical quantum processing unit.
    Qpu,

    /// Application- or repository-specific execution implementation.
    Custom,
}

impl BackendKind {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::Custom => "custom",
        }
    }

    /// Returns whether this kind represents physical quantum hardware.
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether this kind is software-only.
    pub const fn is_software(self) -> bool {
        matches!(self, Self::Simulator | Self::Emulator)
    }
}

impl fmt::Display for BackendKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Backend status
// =============================================================================

/// Operational state of a backend.
///
/// `Busy` is intentionally distinct from `Available`: a backend may be
/// operational while temporarily unable to accept another execution request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendStatus {
    /// State has not yet been established.
    Unknown,

    /// Backend is operational and may accept work.
    Available,

    /// Backend is operational but currently occupied.
    Busy,

    /// Backend is undergoing maintenance.
    Maintenance,

    /// Backend is degraded but may still accept restricted work.
    Degraded,

    /// Backend cannot currently be reached or used.
    Offline,

    /// Backend has been explicitly retired.
    Retired,

    /// Backend is unavailable for an unspecified reason.
    Unavailable,
}

impl BackendStatus {
    /// Returns true if a normal execution request may be submitted.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns true if the backend is known to be operational.
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::Busy
                | Self::Degraded
                | Self::Maintenance
        )
    }

    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Available => "available",
            Self::Busy => "busy",
            Self::Maintenance => "maintenance",
            Self::Degraded => "degraded",
            Self::Offline => "offline",
            Self::Retired => "retired",
            Self::Unavailable => "unavailable",
        }
    }
}

impl fmt::Display for BackendStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Capability model
// =============================================================================

/// Capabilities advertised by a backend.
///
/// This is intentionally broader than gate-model circuits. It provides the
/// stable compatibility surface for future pulse, analog, annealing and
/// logical/fault-tolerant workloads.
///
/// Capability state belongs to the backend descriptor. Capability evidence such
/// as calibration values belongs to `calibration.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// Backend can perform terminal measurement.
    pub measurement: bool,

    /// Backend can reset quantum resources.
    pub reset: bool,

    /// Backend can measure before program termination.
    pub mid_circuit_measurement: bool,

    /// Backend can perform classical feed-forward after measurement.
    pub classical_control: bool,

    /// Backend can execute dynamic circuits/control flow.
    pub dynamic_circuits: bool,

    /// Backend supports arbitrary parameterized one-resource rotations.
    pub arbitrary_single_qubit_rotations: bool,

    /// Backend accepts unbound parameterized gates.
    pub parameterized_gates: bool,

    /// Backend supports three-resource operations.
    pub three_qubit_operations: bool,

    /// Backend supports operations with more than three quantum operands.
    pub multi_qubit_operations: bool,

    /// Backend supports concurrent/parallel operations.
    pub parallel_operations: bool,

    /// Backend supports batched submissions.
    pub batch_execution: bool,

    /// Backend exposes streaming results.
    pub streaming_results: bool,

    /// Backend supports cancellation.
    pub cancellation: bool,

    /// Backend exposes queue information.
    pub queue_information: bool,

    /// Backend supports pulse-level execution.
    pub pulse_control: bool,

    /// Backend supports analog Hamiltonian/control workloads.
    pub analog_control: bool,

    /// Backend supports annealing/Ising/QUBO workloads.
    pub annealing: bool,

    /// Backend exposes logical qubits.
    pub logical_qubits: bool,

    /// Backend supports fault-tolerant/logical operations.
    pub fault_tolerance: bool,

    /// Backend supports syndrome extraction.
    pub syndrome_measurement: bool,

    /// Backend supports decoder execution as part of its execution model.
    pub decoder_execution: bool,

    /// Backend supports deterministic seeded execution.
    pub deterministic_seeding: bool,

    /// Backend supports state-vector results.
    pub state_vector_results: bool,

    /// Backend supports density-matrix results.
    pub density_matrix_results: bool,

    /// Backend supports expectation-value results.
    pub expectation_value_results: bool,

    /// Backend supports readout mitigation.
    pub readout_mitigation: bool,

    /// Backend supports error mitigation.
    pub error_mitigation: bool,

    /// Backend exposes calibration information.
    pub calibration_data: bool,

    /// Backend exposes instruction timing.
    pub timing_information: bool,

    /// Backend exposes topology information.
    pub topology_information: bool,

    /// Backend exposes native instructions.
    pub native_instruction_set: bool,

    /// Stable native instruction identifiers.
    pub native_gates: BTreeSet<String>,

    /// Experimental capabilities are kept separate from stable capabilities.
    ///
    /// A capability appearing here MUST NOT be treated as stable support.
    pub experimental_capabilities: BTreeSet<String>,
}

impl Default for BackendCapabilities {
    fn default() -> Self {
        Self {
            measurement: true,
            reset: true,
            mid_circuit_measurement: false,
            classical_control: false,
            dynamic_circuits: false,
            arbitrary_single_qubit_rotations: false,
            parameterized_gates: false,
            three_qubit_operations: false,
            multi_qubit_operations: false,
            parallel_operations: false,
            batch_execution: false,
            streaming_results: false,
            cancellation: false,
            queue_information: false,
            pulse_control: false,
            analog_control: false,
            annealing: false,
            logical_qubits: false,
            fault_tolerance: false,
            syndrome_measurement: false,
            decoder_execution: false,
            deterministic_seeding: false,
            state_vector_results: false,
            density_matrix_results: false,
            expectation_value_results: false,
            readout_mitigation: false,
            error_mitigation: false,
            calibration_data: false,
            timing_information: false,
            topology_information: true,
            native_instruction_set: false,
            native_gates: BTreeSet::new(),
            experimental_capabilities: BTreeSet::new(),
        }
    }
}

impl BackendCapabilities {
    /// Creates a conservative capability profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a stable native instruction.
    pub fn with_gate(mut self, gate: impl Into<String>) -> Self {
        let gate = normalize_instruction_name(&gate.into());

        if !gate.is_empty() {
            self.native_gates.insert(gate);
            self.native_instruction_set = true;
        }

        self
    }

    /// Adds multiple stable native instructions.
    pub fn with_gates<I, S>(mut self, gates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for gate in gates {
            let gate = normalize_instruction_name(&gate.into());

            if !gate.is_empty() {
                self.native_gates.insert(gate);
            }
        }

        self.native_instruction_set = !self.native_gates.is_empty();
        self
    }

    /// Adds an experimental capability.
    pub fn with_experimental_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Self {
        let capability = normalize_capability_name(&capability.into());

        if !capability.is_empty() {
            self.experimental_capabilities.insert(capability);
        }

        self
    }

    /// Returns whether a native instruction is supported.
    pub fn supports_gate(&self, gate: &str) -> bool {
        self.native_gates
            .contains(&normalize_instruction_name(gate))
    }

    /// Returns whether a stable capability exists by identifier.
    pub fn supports_capability(&self, capability: &str) -> bool {
        let capability = normalize_capability_name(capability);

        self.experimental_capabilities
            .contains(&capability)
            || match capability.as_str() {
                "measurement" => self.measurement,
                "reset" => self.reset,
                "mid_circuit_measurement" => self.mid_circuit_measurement,
                "classical_control" => self.classical_control,
                "dynamic_circuits" => self.dynamic_circuits,
                "arbitrary_single_qubit_rotations" => {
                    self.arbitrary_single_qubit_rotations
                }
                "parameterized_gates" => self.parameterized_gates,
                "three_qubit_operations" => self.three_qubit_operations,
                "multi_qubit_operations" => self.multi_qubit_operations,
                "parallel_operations" => self.parallel_operations,
                "batch_execution" => self.batch_execution,
                "streaming_results" => self.streaming_results,
                "cancellation" => self.cancellation,
                "queue_information" => self.queue_information,
                "pulse_control" => self.pulse_control,
                "analog_control" => self.analog_control,
                "annealing" => self.annealing,
                "logical_qubits" => self.logical_qubits,
                "fault_tolerance" => self.fault_tolerance,
                "syndrome_measurement" => self.syndrome_measurement,
                "decoder_execution" => self.decoder_execution,
                "deterministic_seeding" => self.deterministic_seeding,
                "state_vector_results" => self.state_vector_results,
                "density_matrix_results" => self.density_matrix_results,
                "expectation_value_results" => {
                    self.expectation_value_results
                }
                "readout_mitigation" => self.readout_mitigation,
                "error_mitigation" => self.error_mitigation,
                "calibration_data" => self.calibration_data,
                "timing_information" => self.timing_information,
                "topology_information" => self.topology_information,
                "native_instruction_set" => self.native_instruction_set,
                _ => false,
            }
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Hard backend resource limits.
///
/// A value of `0` means that the provider has not supplied a finite limit.
///
/// A zero limit is therefore NOT interpreted as "zero resources".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendLimits {
    /// Maximum physical resources usable by one workload.
    pub max_qubits: usize,

    /// Maximum logical qubits usable by one workload.
    pub max_logical_qubits: usize,

    /// Maximum circuit depth.
    pub max_circuit_depth: usize,

    /// Maximum operation count.
    pub max_operations: usize,

    /// Maximum number of shots.
    pub max_shots: usize,

    /// Maximum number of classical bits/register elements.
    pub max_classical_bits: usize,

    /// Maximum number of concurrent programs.
    pub max_concurrent_jobs: usize,

    /// Maximum submission batch size.
    pub max_batch_size: usize,
}

impl Default for BackendLimits {
    fn default() -> Self {
        Self {
            max_qubits: 0,
            max_logical_qubits: 0,
            max_circuit_depth: 0,
            max_operations: 0,
            max_shots: 0,
            max_classical_bits: 0,
            max_concurrent_jobs: 0,
            max_batch_size: 0,
        }
    }
}

impl BackendLimits {
    /// Creates an unspecified/unlimited limit profile.
    pub const fn unlimited() -> Self {
        Self {
            max_qubits: 0,
            max_logical_qubits: 0,
            max_circuit_depth: 0,
            max_operations: 0,
            max_shots: 0,
            max_classical_bits: 0,
            max_concurrent_jobs: 0,
            max_batch_size: 0,
        }
    }

    pub const fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    pub const fn with_max_logical_qubits(mut self, value: usize) -> Self {
        self.max_logical_qubits = value;
        self
    }

    pub const fn with_max_depth(mut self, value: usize) -> Self {
        self.max_circuit_depth = value;
        self
    }

    pub const fn with_max_operations(mut self, value: usize) -> Self {
        self.max_operations = value;
        self
    }

    pub const fn with_max_shots(mut self, value: usize) -> Self {
        self.max_shots = value;
        self
    }

    pub const fn with_max_classical_bits(mut self, value: usize) -> Self {
        self.max_classical_bits = value;
        self
    }

    pub const fn with_max_concurrent_jobs(mut self, value: usize) -> Self {
        self.max_concurrent_jobs = value;
        self
    }

    pub const fn with_max_batch_size(mut self, value: usize) -> Self {
        self.max_batch_size = value;
        self
    }

    pub const fn allows_qubits(&self, count: usize) -> bool {
        self.max_qubits == 0 || count <= self.max_qubits
    }

    pub const fn allows_logical_qubits(&self, count: usize) -> bool {
        self.max_logical_qubits == 0 || count <= self.max_logical_qubits
    }

    pub const fn allows_depth(&self, depth: usize) -> bool {
        self.max_circuit_depth == 0 || depth <= self.max_circuit_depth
    }

    pub const fn allows_operations(&self, operations: usize) -> bool {
        self.max_operations == 0 || operations <= self.max_operations
    }

    pub const fn allows_shots(&self, shots: usize) -> bool {
        self.max_shots == 0 || shots <= self.max_shots
    }

    pub const fn allows_classical_bits(&self, count: usize) -> bool {
        self.max_classical_bits == 0 || count <= self.max_classical_bits
    }

    pub const fn allows_concurrent_jobs(&self, count: usize) -> bool {
        self.max_concurrent_jobs == 0 || count <= self.max_concurrent_jobs
    }

    pub const fn allows_batch_size(&self, count: usize) -> bool {
        self.max_batch_size == 0 || count <= self.max_batch_size
    }
}

// =============================================================================
// Backend metadata
// =============================================================================

/// Stable provider-neutral backend metadata.
///
/// This structure deliberately contains no secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendMetadata {
    /// Stable canonical backend identifier.
    pub id: String,

    /// Human-readable backend name.
    pub name: String,

    /// Stable provider identifier.
    pub provider: String,

    /// Backend/API semantic version.
    pub version: String,

    /// Kind of execution target.
    pub kind: BackendKind,

    /// Current operational state.
    pub status: BackendStatus,

    /// Optional stable hardware revision.
    pub hardware_revision: Option<String>,

    /// Optional firmware version.
    pub firmware_version: Option<String>,

    /// Optional provider API version.
    pub api_version: Option<String>,

    /// Optional region/location.
    pub region: Option<String>,

    /// Arbitrary non-secret metadata.
    pub properties: BTreeMap<String, String>,
}

impl BackendMetadata {
    /// Creates validated-by-`QuantumBackend::new` metadata.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
        version: impl Into<String>,
        kind: BackendKind,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            provider: provider.into(),
            version: version.into(),
            kind,
            status: BackendStatus::Available,
            hardware_revision: None,
            firmware_version: None,
            api_version: None,
            region: None,
            properties: BTreeMap::new(),
        }
    }

    pub fn set_status(&mut self, status: BackendStatus) {
        self.status = status;
    }

    pub fn with_hardware_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.hardware_revision = Some(revision.into());
        self
    }

    pub fn with_firmware_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.firmware_version = Some(version.into());
        self
    }

    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Inserts non-secret metadata.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), BackendError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_field(&key, &value)?;

        if self.properties.len() >= MAX_METADATA_PROPERTIES
            && !self.properties.contains_key(&key)
        {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_METADATA_PROPERTIES,
            });
        }

        if looks_like_secret_key(&key) {
            return Err(BackendError::SecretLikeMetadata {
                key,
            });
        }

        self.properties.insert(key, value);
        Ok(())
    }
}

// =============================================================================
// Workload model
// =============================================================================

/// Kind of quantum workload submitted to a backend.
///
/// This prevents the backend layer from assuming every quantum processor is a
/// conventional gate-model QPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumWorkloadKind {
    /// Ordinary gate-model circuit.
    GateCircuit,

    /// Circuit with measurement-dependent classical control.
    DynamicCircuit,

    /// Direct pulse/control program.
    PulseProgram,

    /// Analog Hamiltonian/control program.
    AnalogProgram,

    /// Ising/QUBO/annealing workload.
    AnnealingProblem,

    /// Logical/fault-tolerant quantum workload.
    LogicalProgram,

    /// Generic sampling workload.
    Sampling,

    /// Provider-specific/custom workload.
    Custom,
}

impl QuantumWorkloadKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateCircuit => "gate_circuit",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::PulseProgram => "pulse_program",
            Self::AnalogProgram => "analog_program",
            Self::AnnealingProblem => "annealing_problem",
            Self::LogicalProgram => "logical_program",
            Self::Sampling => "sampling",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for QuantumWorkloadKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Circuit requirements
// =============================================================================

/// Hardware requirements for a gate-model workload.
///
/// This is deliberately a requirement description rather than the canonical
/// Zamani Quantum IR.
///
/// The actual circuit remains owned by `quantum::ir`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitRequirements {
    /// Number of physical qubits/resources required.
    pub qubit_count: usize,

    /// Number of logical qubits required.
    pub logical_qubit_count: usize,

    /// Circuit depth.
    pub circuit_depth: usize,

    /// Number of quantum operations.
    pub operation_count: usize,

    /// Number of classical bits required.
    pub classical_bit_count: usize,

    /// Number of execution shots.
    pub shots: usize,

    /// Instructions used by the workload.
    pub gates: Vec<String>,

    /// Physical/native two-resource interactions required.
    pub two_qubit_edges: Vec<(usize, usize)>,

    /// Workload requires terminal measurement.
    pub requires_measurement: bool,

    /// Workload requires reset.
    pub requires_reset: bool,

    /// Workload requires mid-circuit measurement.
    pub requires_mid_circuit_measurement: bool,

    /// Workload requires measurement-based classical feed-forward.
    pub requires_classical_control: bool,

    /// Workload is explicitly dynamic.
    pub requires_dynamic_circuits: bool,

    /// Workload requires pulse-level control.
    pub requires_pulse_control: bool,

    /// Workload requires analog control.
    pub requires_analog_control: bool,

    /// Workload requires annealing.
    pub requires_annealing: bool,

    /// Workload requires logical/fault-tolerant resources.
    pub requires_logical_qubits: bool,

    /// Workload requires fault-tolerant operations.
    pub requires_fault_tolerance: bool,

    /// Workload requires deterministic seeded execution.
    pub requires_deterministic_seed: bool,

    /// Workload requires state-vector output.
    pub requires_state_vector: bool,

    /// Workload requires density-matrix output.
    pub requires_density_matrix: bool,

    /// Workload requires expectation values.
    pub requires_expectation_values: bool,
}

impl Default for CircuitRequirements {
    fn default() -> Self {
        Self {
            qubit_count: 0,
            logical_qubit_count: 0,
            circuit_depth: 0,
            operation_count: 0,
            classical_bit_count: 0,
            shots: 1,
            gates: Vec::new(),
            two_qubit_edges: Vec::new(),
            requires_measurement: false,
            requires_reset: false,
            requires_mid_circuit_measurement: false,
            requires_classical_control: false,
            requires_dynamic_circuits: false,
            requires_pulse_control: false,
            requires_analog_control: false,
            requires_annealing: false,
            requires_logical_qubits: false,
            requires_fault_tolerance: false,
            requires_deterministic_seed: false,
            requires_state_vector: false,
            requires_density_matrix: false,
            requires_expectation_values: false,
        }
    }
}

impl CircuitRequirements {
    /// Returns the workload kind implied by the requirements.
    pub fn inferred_kind(&self) -> QuantumWorkloadKind {
        if self.requires_annealing {
            QuantumWorkloadKind::AnnealingProblem
        } else if self.requires_analog_control {
            QuantumWorkloadKind::AnalogProgram
        } else if self.requires_pulse_control {
            QuantumWorkloadKind::PulseProgram
        } else if self.requires_logical_qubits || self.requires_fault_tolerance {
            QuantumWorkloadKind::LogicalProgram
        } else if self.requires_dynamic_circuits
            || self.requires_mid_circuit_measurement
            || self.requires_classical_control
        {
            QuantumWorkloadKind::DynamicCircuit
        } else {
            QuantumWorkloadKind::GateCircuit
        }
    }
}

// =============================================================================
// General workload requirements
// =============================================================================

/// Provider-neutral execution requirements.
///
/// This is the abstraction future `execution.rs` and `compatibility.rs` use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadRequirements {
    /// Kind of quantum workload.
    pub kind: QuantumWorkloadKind,

    /// Gate/circuit-specific requirements.
    pub circuit: CircuitRequirements,

    /// Required stable backend capability identifiers.
    pub required_capabilities: BTreeSet<String>,

    /// Required native instructions.
    pub required_instructions: BTreeSet<String>,

    /// Whether topology must be available.
    pub requires_topology: bool,

    /// Whether calibration evidence must be available.
    pub requires_calibration: bool,

    /// Whether fresh calibration is mandatory.
    pub requires_fresh_calibration: bool,
}

impl Default for WorkloadRequirements {
    fn default() -> Self {
        Self {
            kind: QuantumWorkloadKind::GateCircuit,
            circuit: CircuitRequirements::default(),
            required_capabilities: BTreeSet::new(),
            required_instructions: BTreeSet::new(),
            requires_topology: false,
            requires_calibration: false,
            requires_fresh_calibration: false,
        }
    }
}

impl WorkloadRequirements {
    /// Creates requirements from a circuit requirement object.
    pub fn from_circuit(circuit: CircuitRequirements) -> Self {
        Self {
            kind: circuit.inferred_kind(),
            circuit,
            ..Self::default()
        }
    }

    /// Adds a required capability.
    pub fn require_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Self {
        let capability = normalize_capability_name(&capability.into());

        if !capability.is_empty() {
            self.required_capabilities.insert(capability);
        }

        self
    }

    /// Adds a required instruction.
    pub fn require_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Self {
        let instruction = normalize_instruction_name(&instruction.into());

        if !instruction.is_empty() {
            self.required_instructions.insert(instruction);
        }

        self
    }

    pub fn with_topology_requirement(mut self, required: bool) -> Self {
        self.requires_topology = required;
        self
    }

    pub fn with_calibration_requirement(mut self, required: bool) -> Self {
        self.requires_calibration = required;
        self
    }

    pub fn with_fresh_calibration_requirement(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_fresh_calibration = required;
        self.requires_calibration |= required;
        self
    }
}

// =============================================================================
// Execution request
// =============================================================================

/// Provider-neutral execution request.
///
/// It contains workload requirements but intentionally does not contain
/// provider credentials or provider-specific transport state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    /// Backend-independent workload requirements.
    pub workload: WorkloadRequirements,

    /// Optional deterministic execution seed.
    pub seed: Option<u64>,

    /// Requested priority.
    ///
    /// `0` is normal priority. Larger values are higher priority. Providers
    /// may map this into their own scheduling model.
    pub priority: u32,

    /// Whether provider-side asynchronous execution is acceptable.
    pub asynchronous: bool,

    /// Optional caller-defined request identifier.
    ///
    /// This must not contain secrets.
    pub request_id: Option<String>,

    /// Non-secret execution metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionRequest {
    pub fn new(circuit: CircuitRequirements) -> Self {
        Self {
            workload: WorkloadRequirements::from_circuit(circuit),
            seed: None,
            priority: 0,
            asynchronous: true,
            request_id: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn from_workload(workload: WorkloadRequirements) -> Self {
        Self {
            workload,
            seed: None,
            priority: 0,
            asynchronous: true,
            request_id: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn synchronous(mut self) -> Self {
        self.asynchronous = false;
        self
    }

    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let request_id = request_id.into();

        validate_identifier(
            "request_id",
            &request_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if looks_like_secret_value(&request_id) {
            return Err(BackendError::SecretLikeMetadata {
                key: "request_id".to_string(),
            });
        }

        self.request_id = Some(request_id);
        Ok(self)
    }

    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), BackendError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_field(&key, &value)?;

        if looks_like_secret_key(&key) {
            return Err(BackendError::SecretLikeMetadata {
                key,
            });
        }

        self.metadata.insert(key, value);
        Ok(())
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Generic normalized execution result.
///
/// Provider-specific information must be stored in metadata, never in the
/// core result semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    /// Backend that produced the result.
    pub backend_id: String,

    /// Number of shots represented by the result.
    pub shots: usize,

    /// Normalized classical bitstring counts.
    pub counts: BTreeMap<String, usize>,

    /// Optional normalized expectation values.
    pub expectation_values: BTreeMap<String, String>,

    /// Provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionResult {
    pub fn empty(
        backend_id: impl Into<String>,
        shots: usize,
    ) -> Result<Self, BackendError> {
        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        Ok(Self {
            backend_id,
            shots,
            counts: BTreeMap::new(),
            expectation_values: BTreeMap::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Returns the number of recorded samples in `counts`.
    pub fn counted_shots(&self) -> usize {
        self.counts
            .values()
            .copied()
            .fold(0usize, |total, count| total.saturating_add(count))
    }

    /// Returns whether normalized counts account for exactly all shots.
    pub fn counts_match_shots(&self) -> bool {
        self.counted_shots() == self.shots
    }

    /// Inserts a normalized bitstring count.
    pub fn insert_count(
        &mut self,
        bitstring: impl Into<String>,
        count: usize,
    ) -> Result<(), BackendError> {
        let bitstring = bitstring.into();

        if bitstring.is_empty()
            || !bitstring.bytes().all(|byte| byte == b'0' || byte == b'1')
        {
            return Err(BackendError::InvalidBitstring {
                bitstring,
            });
        }

        let current = self.counted_shots();

        let new_total = current
            .checked_add(count)
            .ok_or(BackendError::ResultCountOverflow)?;

        if new_total > self.shots {
            return Err(BackendError::ResultShotsExceeded {
                represented: new_total,
                shots: self.shots,
            });
        }

        self.counts.insert(bitstring, count);
        Ok(())
    }
}

// =============================================================================
// Validation outcome
// =============================================================================

/// Severity of a backend validation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationSeverity {
    /// Informational observation.
    Info,

    /// Workload is valid but the caller should be aware of a limitation.
    Warning,

    /// Workload cannot be submitted to this backend.
    Error,
}

impl ValidationSeverity {
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Error)
    }
}

/// Structured validation diagnostic.
///
/// The diagnostic is intentionally machine-readable so Danga, benchmarking,
/// IDE/LSP integrations and provider adapters can consume it without parsing
/// human-readable strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    pub code: &'static str,
    pub severity: ValidationSeverity,
    pub message: String,
    pub requirement: String,
    pub backend_id: String,
}

/// Complete validation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendValidationReport {
    pub backend_id: String,
    pub valid: bool,
    pub diagnostics: Vec<ValidationDiagnostic>,
}

impl BackendValidationReport {
    fn new(backend_id: &str) -> Self {
        Self {
            backend_id: backend_id.to_string(),
            valid: true,
            diagnostics: Vec::new(),
        }
    }

    fn error(
        &mut self,
        code: &'static str,
        message: String,
        requirement: String,
    ) {
        self.valid = false;

        self.diagnostics.push(ValidationDiagnostic {
            code,
            severity: ValidationSeverity::Error,
            message,
            requirement,
            backend_id: self.backend_id.clone(),
        });
    }

    fn warning(
        &mut self,
        code: &'static str,
        message: String,
        requirement: String,
    ) {
        self.diagnostics.push(ValidationDiagnostic {
            code,
            severity: ValidationSeverity::Warning,
            message,
            requirement,
            backend_id: self.backend_id.clone(),
        });
    }

    pub fn has_errors(&self) -> bool {
        !self.valid
    }

    pub fn errors(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity.is_blocking())
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics.iter().filter(|diagnostic| {
            diagnostic.severity == ValidationSeverity::Warning
        })
    }
}

// =============================================================================
// Backend errors
// =============================================================================

/// Stable provider-neutral backend error taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    BackendUnavailable {
        backend_id: String,
        status: BackendStatus,
    },

    InvalidBackendId,

    InvalidIdentifier {
        field: &'static str,
    },

    IdentifierTooLong {
        field: &'static str,
        maximum: usize,
    },

    InvalidMetadata {
        key: String,
    },

    MetadataLimitExceeded {
        maximum: usize,
    },

    SecretLikeMetadata {
        key: String,
    },

    InvalidShots,

    ZeroQubits,

    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    LogicalQubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    CircuitDepthExceeded {
        requested: usize,
        maximum: usize,
    },

    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    ClassicalBitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    UnsupportedWorkload {
        workload: QuantumWorkloadKind,
    },

    UnsupportedCapability {
        capability: String,
    },

    UnsupportedGate {
        gate: String,
    },

    MeasurementUnsupported,

    ResetUnsupported,

    MidCircuitMeasurementUnsupported,

    ClassicalControlUnsupported,

    DynamicCircuitUnsupported,

    PulseControlUnsupported,

    AnalogControlUnsupported,

    AnnealingUnsupported,

    LogicalQubitsUnsupported,

    FaultToleranceUnsupported,

    DeterministicSeedingUnsupported,

    StateVectorUnsupported,

    DensityMatrixUnsupported,

    ExpectationValuesUnsupported,

    InvalidQubit {
        qubit: usize,
        qubit_count: usize,
    },

    UnsupportedConnection {
        control: usize,
        target: usize,
    },

    TopologyUnavailable,

    InvalidTopology(String),

    RequiredInstructionLimitExceeded {
        maximum: usize,
    },

    RequiredTopologyEdgeLimitExceeded {
        maximum: usize,
    },

    NativeInstructionSetUnavailable,

    CalibrationUnavailable,

    FreshCalibrationRequired,

    ExecutionUnavailable(String),

    ExecutionRejected(String),

    ResultCountOverflow,

    ResultShotsExceeded {
        represented: usize,
        shots: usize,
    },

    InvalidBitstring {
        bitstring: String,
    },

    Topology(TopologyError),
}

impl fmt::Display for BackendError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::BackendUnavailable {
                backend_id,
                status,
            } => write!(
                formatter,
                "backend '{}' is not available ({})",
                backend_id, status
            ),

            Self::InvalidBackendId => {
                formatter.write_str("backend ID cannot be empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid {field} identifier")
            }

            Self::IdentifierTooLong {
                field,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} identifier exceeds maximum length {maximum}"
                )
            }

            Self::InvalidMetadata { key } => {
                write!(formatter, "invalid metadata key '{key}'")
            }

            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "metadata property limit of {maximum} exceeded"
                )
            }

            Self::SecretLikeMetadata { key } => {
                write!(
                    formatter,
                    "metadata field '{key}' appears to contain secret material"
                )
            }

            Self::InvalidShots => {
                formatter.write_str("shot count must be greater than zero")
            }

            Self::ZeroQubits => {
                formatter.write_str(
                    "circuit workload must contain at least one qubit",
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "workload requires {requested} qubits but backend supports at most {maximum}"
            ),

            Self::LogicalQubitLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "workload requires {requested} logical qubits but backend supports at most {maximum}"
            ),

            Self::CircuitDepthExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "circuit depth {requested} exceeds backend limit {maximum}"
            ),

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "operation count {requested} exceeds backend limit {maximum}"
            ),

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "shot count {requested} exceeds backend limit {maximum}"
            ),

            Self::ClassicalBitLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "classical-bit requirement {requested} exceeds backend limit {maximum}"
            ),

            Self::UnsupportedWorkload { workload } => {
                write!(formatter, "backend does not support workload '{workload}'")
            }

            Self::UnsupportedCapability { capability } => {
                write!(
                    formatter,
                    "backend does not support required capability '{capability}'"
                )
            }

            Self::UnsupportedGate { gate } => {
                write!(formatter, "backend does not support gate '{gate}'")
            }

            Self::MeasurementUnsupported => {
                formatter.write_str("backend does not support measurement")
            }

            Self::ResetUnsupported => {
                formatter.write_str("backend does not support qubit reset")
            }

            Self::MidCircuitMeasurementUnsupported => {
                formatter.write_str(
                    "backend does not support mid-circuit measurement",
                )
            }

            Self::ClassicalControlUnsupported => {
                formatter.write_str(
                    "backend does not support measurement-based classical control",
                )
            }

            Self::DynamicCircuitUnsupported => {
                formatter.write_str("backend does not support dynamic circuits")
            }

            Self::PulseControlUnsupported => {
                formatter.write_str("backend does not support pulse control")
            }

            Self::AnalogControlUnsupported => {
                formatter.write_str("backend does not support analog control")
            }

            Self::AnnealingUnsupported => {
                formatter.write_str("backend does not support annealing workloads")
            }

            Self::LogicalQubitsUnsupported => {
                formatter.write_str("backend does not expose logical qubits")
            }

            Self::FaultToleranceUnsupported => {
                formatter.write_str(
                    "backend does not support fault-tolerant operations",
                )
            }

            Self::DeterministicSeedingUnsupported => {
                formatter.write_str(
                    "backend does not support deterministic seeded execution",
                )
            }

            Self::StateVectorUnsupported => {
                formatter.write_str("backend does not support state-vector results")
            }

            Self::DensityMatrixUnsupported => {
                formatter.write_str(
                    "backend does not support density-matrix results",
                )
            }

            Self::ExpectationValuesUnsupported => {
                formatter.write_str(
                    "backend does not support expectation-value results",
                )
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => write!(
                formatter,
                "qubit {qubit} is outside workload range 0..{}",
                qubit_count.saturating_sub(1)
            ),

            Self::UnsupportedConnection {
                control,
                target,
            } => write!(
                formatter,
                "backend topology does not support native connection {control} -> {target}"
            ),

            Self::TopologyUnavailable => {
                formatter.write_str("backend topology information is unavailable")
            }

            Self::InvalidTopology(message) => {
                write!(formatter, "invalid backend topology: {message}")
            }

            Self::RequiredInstructionLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "required instruction count exceeds maximum {maximum}"
                )
            }

            Self::RequiredTopologyEdgeLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "required topology edge count exceeds maximum {maximum}"
                )
            }

            Self::NativeInstructionSetUnavailable => {
                formatter.write_str(
                    "backend does not expose a native instruction set",
                )
            }

            Self::CalibrationUnavailable => {
                formatter.write_str(
                    "required calibration information is unavailable",
                )
            }

            Self::FreshCalibrationRequired => {
                formatter.write_str(
                    "fresh calibration evidence is required before execution",
                )
            }

            Self::ExecutionUnavailable(message) => {
                write!(formatter, "execution unavailable: {message}")
            }

            Self::ExecutionRejected(message) => {
                write!(formatter, "execution rejected: {message}")
            }

            Self::ResultCountOverflow => {
                formatter.write_str("execution result shot count overflowed")
            }

            Self::ResultShotsExceeded {
                represented,
                shots,
            } => write!(
                formatter,
                "result represents {represented} shots but execution requested only {shots}"
            ),

            Self::InvalidBitstring { bitstring } => {
                write!(
                    formatter,
                    "invalid result bitstring '{bitstring}'"
                )
            }

            Self::Topology(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<TopologyError> for BackendError {
    fn from(error: TopologyError) -> Self {
        Self::Topology(error)
    }
}

// =============================================================================
// Quantum backend
// =============================================================================

/// Canonical provider-neutral quantum backend.
///
/// This is a backend descriptor and validation boundary.
///
/// Actual provider execution is supplied by adapters implementing the later
/// backend/provider execution traits. `QuantumBackend` itself deliberately does
/// not perform network/device I/O.
#[derive(Debug, Clone)]
pub struct QuantumBackend {
    /// Stable backend metadata.
    pub metadata: BackendMetadata,

    /// Advertised capabilities.
    pub capabilities: BackendCapabilities,

    /// Hard resource limits.
    pub limits: BackendLimits,

    /// Authoritative physical topology.
    pub topology: HardwareTopology,
}

impl QuantumBackend {
    /// Creates a validated backend descriptor.
    pub fn new(
        metadata: BackendMetadata,
        capabilities: BackendCapabilities,
        limits: BackendLimits,
        topology: HardwareTopology,
    ) -> Result<Self, BackendError> {
        validate_backend_metadata(&metadata)?;
        validate_capabilities(&capabilities)?;
        validate_topology(&topology)?;

        let backend = Self {
            metadata,
            capabilities,
            limits,
            topology,
        };

        backend.validate_internal_consistency()?;

        Ok(backend)
    }

    /// Stable backend identifier.
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    /// Provider identifier.
    pub fn provider(&self) -> &str {
        &self.metadata.provider
    }

    /// Backend kind.
    pub const fn kind(&self) -> BackendKind {
        self.metadata.kind
    }

    /// Backend operational status.
    pub const fn status(&self) -> BackendStatus {
        self.metadata.status
    }

    /// Returns true if ordinary execution is currently permitted.
    pub const fn is_available(&self) -> bool {
        self.metadata.status.is_usable()
    }

    /// Updates the operational status.
    ///
    /// Status mutation does not modify topology, capabilities or limits.
    pub fn set_status(&mut self, status: BackendStatus) {
        self.metadata.status = status;
    }

    /// Number of physical resources represented by topology.
    pub const fn qubit_count(&self) -> usize {
        self.topology.qubit_count()
    }

    /// Number of topology couplings.
    pub const fn coupling_count(&self) -> usize {
        self.topology.coupling_count()
    }

    /// Returns the deterministic native instruction list.
    pub fn native_gates(&self) -> Vec<String> {
        self.capabilities.native_gates.iter().cloned().collect()
    }

    /// Returns a reference to the authoritative topology.
    pub fn topology(&self) -> &HardwareTopology {
        &self.topology
    }

    /// Returns the complete backend descriptor as a cheap borrowed view.
    pub fn descriptor(&self) -> BackendDescriptor<'_> {
        BackendDescriptor { backend: self }
    }

    /// Performs a complete compatibility/validation analysis.
    pub fn validation_report(
        &self,
        requirements: &WorkloadRequirements,
    ) -> BackendValidationReport {
        let mut report = BackendValidationReport::new(self.id());

        self.validate_status_report(&mut report);
        self.validate_resource_report(requirements, &mut report);
        self.validate_workload_report(requirements, &mut report);
        self.validate_capability_report(requirements, &mut report);
        self.validate_instruction_report(requirements, &mut report);
        self.validate_topology_report(requirements, &mut report);

        report
    }

    /// Validates a workload and returns a structured error on failure.
    pub fn validate(
        &self,
        requirements: &WorkloadRequirements,
    ) -> Result<(), BackendError> {
        let report = self.validation_report(requirements);

        if report.valid {
            Ok(())
        } else {
            first_validation_error(&report)
        }
    }

    /// Backwards-compatible circuit validation entry point.
    pub fn validate_circuit(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        let workload = WorkloadRequirements::from_circuit(requirements.clone());

        self.validate(&workload)
    }

    /// Validates an execution request before it reaches an adapter.
    pub fn validate_request(
        &self,
        request: &ExecutionRequest,
    ) -> Result<(), BackendError> {
        self.validate(&request.workload)?;

        if request.seed.is_some()
            && !self.capabilities.deterministic_seeding
        {
            return Err(BackendError::DeterministicSeedingUnsupported);
        }

        Ok(())
    }

    /// Performs provider-independent preflight validation.
    ///
    /// This is the correct execution boundary for provider adapters:
    ///
    /// ```text
    /// adapter
    ///   |
    ///   v
    /// backend.validate_request()
    ///   |
    ///   +--> rejected before provider I/O
    ///   |
    ///   v
    /// provider submission
    /// ```
    ///
    /// This method intentionally does NOT submit work.
    pub fn preflight(
        &self,
        request: &ExecutionRequest,
    ) -> Result<(), BackendError> {
        self.validate_request(request)
    }

    /// Returns a deterministic summary of backend capabilities.
    pub fn capability_names(&self) -> Vec<String> {
        let mut capabilities = BTreeSet::new();

        macro_rules! add {
            ($field:ident, $name:literal) => {
                if self.capabilities.$field {
                    capabilities.insert($name.to_string());
                }
            };
        }

        add!(measurement, "measurement");
        add!(reset, "reset");
        add!(
            mid_circuit_measurement,
            "mid_circuit_measurement"
        );
        add!(classical_control, "classical_control");
        add!(dynamic_circuits, "dynamic_circuits");
        add!(
            arbitrary_single_qubit_rotations,
            "arbitrary_single_qubit_rotations"
        );
        add!(parameterized_gates, "parameterized_gates");
        add!(three_qubit_operations, "three_qubit_operations");
        add!(
            multi_qubit_operations,
            "multi_qubit_operations"
        );
        add!(parallel_operations, "parallel_operations");
        add!(batch_execution, "batch_execution");
        add!(streaming_results, "streaming_results");
        add!(cancellation, "cancellation");
        add!(queue_information, "queue_information");
        add!(pulse_control, "pulse_control");
        add!(analog_control, "analog_control");
        add!(annealing, "annealing");
        add!(logical_qubits, "logical_qubits");
        add!(fault_tolerance, "fault_tolerance");
        add!(syndrome_measurement, "syndrome_measurement");
        add!(decoder_execution, "decoder_execution");
        add!(deterministic_seeding, "deterministic_seeding");
        add!(state_vector_results, "state_vector_results");
        add!(density_matrix_results, "density_matrix_results");
        add!(
            expectation_value_results,
            "expectation_value_results"
        );
        add!(readout_mitigation, "readout_mitigation");
        add!(error_mitigation, "error_mitigation");
        add!(calibration_data, "calibration_data");
        add!(timing_information, "timing_information");
        add!(topology_information, "topology_information");
        add!(
            native_instruction_set,
            "native_instruction_set"
        );

        capabilities
            .into_iter()
            .collect()
    }

    fn validate_internal_consistency(&self) -> Result<(), BackendError> {
        if self.limits.max_qubits != 0
            && self.limits.max_qubits > self.qubit_count()
        {
            // This is not an error. A provider may advertise a topology
            // snapshot smaller than its theoretical maximum. Therefore no
            // rejection is performed here.
        }

        if self.capabilities.native_instruction_set
            && self.capabilities.native_gates.len()
                > MAX_NATIVE_INSTRUCTIONS
        {
            return Err(BackendError::RequiredInstructionLimitExceeded {
                maximum: MAX_NATIVE_INSTRUCTIONS,
            });
        }

        Ok(())
    }

    fn validate_status_report(
        &self,
        report: &mut BackendValidationReport,
    ) {
        match self.status() {
            BackendStatus::Available => {}

            BackendStatus::Degraded => report.warning(
                "BACKEND_DEGRADED",
                "backend is operational but currently degraded".to_string(),
                "backend_status=degraded".to_string(),
            ),

            BackendStatus::Busy => report.error(
                "BACKEND_BUSY",
                "backend is currently busy".to_string(),
                "backend_status=available".to_string(),
            ),

            BackendStatus::Unknown => report.error(
                "BACKEND_STATUS_UNKNOWN",
                "backend operational status is unknown".to_string(),
                "backend_status=known".to_string(),
            ),

            BackendStatus::Maintenance => report.error(
                "BACKEND_MAINTENANCE",
                "backend is under maintenance".to_string(),
                "backend_status=available".to_string(),
            ),

            BackendStatus::Offline => report.error(
                "BACKEND_OFFLINE",
                "backend is offline".to_string(),
                "backend_status=available".to_string(),
            ),

            BackendStatus::Retired => report.error(
                "BACKEND_RETIRED",
                "backend has been retired".to_string(),
                "backend_status=available".to_string(),
            ),

            BackendStatus::Unavailable => report.error(
                "BACKEND_UNAVAILABLE",
                "backend is unavailable".to_string(),
                "backend_status=available".to_string(),
            ),
        }
    }

    fn validate_resource_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;

        if circuit.qubit_count == 0 {
            report.error(
                "ZERO_QUBITS",
                "gate workload contains zero qubits".to_string(),
                "qubit_count>0".to_string(),
            );
        }

        if !self.limits.allows_qubits(circuit.qubit_count) {
            report.error(
                "QUBIT_LIMIT",
                format!(
                    "workload requires {} qubits but backend limit is {}",
                    circuit.qubit_count,
                    self.limits.max_qubits
                ),
                format!("qubit_count<={}", self.limits.max_qubits),
            );
        }

        if !self
            .limits
            .allows_logical_qubits(circuit.logical_qubit_count)
        {
            report.error(
                "LOGICAL_QUBIT_LIMIT",
                format!(
                    "workload requires {} logical qubits but backend limit is {}",
                    circuit.logical_qubit_count,
                    self.limits.max_logical_qubits
                ),
                format!(
                    "logical_qubit_count<={}",
                    self.limits.max_logical_qubits
                ),
            );
        }

        if !self.limits.allows_depth(circuit.circuit_depth) {
            report.error(
                "CIRCUIT_DEPTH_LIMIT",
                format!(
                    "circuit depth {} exceeds backend limit {}",
                    circuit.circuit_depth,
                    self.limits.max_circuit_depth
                ),
                format!("circuit_depth<={}", self.limits.max_circuit_depth),
            );
        }

        if !self
            .limits
            .allows_operations(circuit.operation_count)
        {
            report.error(
                "OPERATION_LIMIT",
                format!(
                    "operation count {} exceeds backend limit {}",
                    circuit.operation_count,
                    self.limits.max_operations
                ),
                format!(
                    "operation_count<={}",
                    self.limits.max_operations
                ),
            );
        }

        if circuit.shots == 0 {
            report.error(
                "INVALID_SHOTS",
                "shot count must be greater than zero".to_string(),
                "shots>0".to_string(),
            );
        } else if !self.limits.allows_shots(circuit.shots) {
            report.error(
                "SHOT_LIMIT",
                format!(
                    "shot count {} exceeds backend limit {}",
                    circuit.shots,
                    self.limits.max_shots
                ),
                format!("shots<={}", self.limits.max_shots),
            );
        }

        if !self
            .limits
            .allows_classical_bits(circuit.classical_bit_count)
        {
            report.error(
                "CLASSICAL_BIT_LIMIT",
                format!(
                    "classical-bit requirement {} exceeds backend limit {}",
                    circuit.classical_bit_count,
                    self.limits.max_classical_bits
                ),
                format!(
                    "classical_bit_count<={}",
                    self.limits.max_classical_bits
                ),
            );
        }
    }

    fn validate_workload_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let supported = match requirements.kind {
            QuantumWorkloadKind::GateCircuit
            | QuantumWorkloadKind::Sampling => true,

            QuantumWorkloadKind::DynamicCircuit => {
                self.capabilities.dynamic_circuits
            }

            QuantumWorkloadKind::PulseProgram => {
                self.capabilities.pulse_control
            }

            QuantumWorkloadKind::AnalogProgram => {
                self.capabilities.analog_control
            }

            QuantumWorkloadKind::AnnealingProblem => {
                self.capabilities.annealing
            }

            QuantumWorkloadKind::LogicalProgram => {
                self.capabilities.logical_qubits
            }

            QuantumWorkloadKind::Custom => true,
        };

        if !supported {
            report.error(
                "UNSUPPORTED_WORKLOAD",
                format!(
                    "backend does not support workload '{}'",
                    requirements.kind
                ),
                format!("workload={}", requirements.kind),
            );
        }
    }

    fn validate_capability_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;
        let capabilities = &self.capabilities;

        if circuit.requires_measurement && !capabilities.measurement {
            report.error(
                "MEASUREMENT_UNSUPPORTED",
                "measurement is required but unsupported".to_string(),
                "measurement=true".to_string(),
            );
        }

        if circuit.requires_reset && !capabilities.reset {
            report.error(
                "RESET_UNSUPPORTED",
                "reset is required but unsupported".to_string(),
                "reset=true".to_string(),
            );
        }

        if circuit.requires_mid_circuit_measurement
            && !capabilities.mid_circuit_measurement
        {
            report.error(
                "MID_CIRCUIT_MEASUREMENT_UNSUPPORTED",
                "mid-circuit measurement is required but unsupported"
                    .to_string(),
                "mid_circuit_measurement=true".to_string(),
            );
        }

        if circuit.requires_classical_control
            && !capabilities.classical_control
        {
            report.error(
                "CLASSICAL_CONTROL_UNSUPPORTED",
                "classical feed-forward is required but unsupported"
                    .to_string(),
                "classical_control=true".to_string(),
            );
        }

        if circuit.requires_dynamic_circuits
            && !capabilities.dynamic_circuits
        {
            report.error(
                "DYNAMIC_CIRCUIT_UNSUPPORTED",
                "dynamic circuits are required but unsupported".to_string(),
                "dynamic_circuits=true".to_string(),
            );
        }

        if circuit.requires_pulse_control && !capabilities.pulse_control {
            report.error(
                "PULSE_CONTROL_UNSUPPORTED",
                "pulse-level control is required but unsupported".to_string(),
                "pulse_control=true".to_string(),
            );
        }

        if circuit.requires_analog_control && !capabilities.analog_control {
            report.error(
                "ANALOG_CONTROL_UNSUPPORTED",
                "analog control is required but unsupported".to_string(),
                "analog_control=true".to_string(),
            );
        }

        if circuit.requires_annealing && !capabilities.annealing {
            report.error(
                "ANNEALING_UNSUPPORTED",
                "annealing is required but unsupported".to_string(),
                "annealing=true".to_string(),
            );
        }

        if circuit.requires_logical_qubits && !capabilities.logical_qubits {
            report.error(
                "LOGICAL_QUBITS_UNSUPPORTED",
                "logical qubits are required but unavailable".to_string(),
                "logical_qubits=true".to_string(),
            );
        }

        if circuit.requires_fault_tolerance
            && !capabilities.fault_tolerance
        {
            report.error(
                "FAULT_TOLERANCE_UNSUPPORTED",
                "fault-tolerant execution is required but unsupported"
                    .to_string(),
                "fault_tolerance=true".to_string(),
            );
        }

        if circuit.requires_deterministic_seed
            && !capabilities.deterministic_seeding
        {
            report.error(
                "DETERMINISTIC_SEED_UNSUPPORTED",
                "deterministic seeded execution is required but unsupported"
                    .to_string(),
                "deterministic_seeding=true".to_string(),
            );
        }

        if circuit.requires_state_vector
            && !capabilities.state_vector_results
        {
            report.error(
                "STATE_VECTOR_UNSUPPORTED",
                "state-vector results are required but unsupported"
                    .to_string(),
                "state_vector_results=true".to_string(),
            );
        }

        if circuit.requires_density_matrix
            && !capabilities.density_matrix_results
        {
            report.error(
                "DENSITY_MATRIX_UNSUPPORTED",
                "density-matrix results are required but unsupported"
                    .to_string(),
                "density_matrix_results=true".to_string(),
            );
        }

        if circuit.requires_expectation_values
            && !capabilities.expectation_value_results
        {
            report.error(
                "EXPECTATION_VALUES_UNSUPPORTED",
                "expectation-value results are required but unsupported"
                    .to_string(),
                "expectation_value_results=true".to_string(),
            );
        }

        for capability in &requirements.required_capabilities {
            if !capabilities.supports_capability(capability) {
                report.error(
                    "REQUIRED_CAPABILITY_UNSUPPORTED",
                    format!(
                        "required capability '{}' is unsupported",
                        capability
                    ),
                    format!("capability={capability}"),
                );
            }
        }

        if requirements.requires_topology
            && !capabilities.topology_information
        {
            report.error(
                "TOPOLOGY_INFORMATION_UNAVAILABLE",
                "workload requires topology information".to_string(),
                "topology_information=true".to_string(),
            );
        }

        if requirements.requires_calibration
            && !capabilities.calibration_data
        {
            report.error(
                "CALIBRATION_UNAVAILABLE",
                "workload requires calibration information".to_string(),
                "calibration_data=true".to_string(),
            );
        }

        if requirements.requires_fresh_calibration {
            if !capabilities.calibration_data {
                report.error(
                    "FRESH_CALIBRATION_UNAVAILABLE",
                    "fresh calibration is required but calibration data is unavailable"
                        .to_string(),
                    "calibration_data=true".to_string(),
                );
            } else {
                report.warning(
                    "CALIBRATION_FRESHNESS_DEFERRED",
                    "calibration freshness must be checked against the selected calibration snapshot before submission"
                        .to_string(),
                    "fresh_calibration=true".to_string(),
                );
            }
        }
    }

    fn validate_instruction_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;

        if requirements.required_instructions.len()
            > MAX_REQUIRED_INSTRUCTIONS
        {
            report.error(
                "REQUIRED_INSTRUCTION_LIMIT",
                format!(
                    "required instruction count exceeds {}",
                    MAX_REQUIRED_INSTRUCTIONS
                ),
                format!(
                    "required_instruction_count<={}",
                    MAX_REQUIRED_INSTRUCTIONS
                ),
            );

            return;
        }

        for instruction in &requirements.required_instructions {
            if !self.capabilities.native_instruction_set {
                report.error(
                    "NATIVE_INSTRUCTION_SET_UNAVAILABLE",
                    "workload requires native instruction matching but backend does not expose its native instruction set"
                        .to_string(),
                    format!("native_instruction={instruction}"),
                );

                continue;
            }

            if !self.capabilities.supports_gate(instruction) {
                report.error(
                    "REQUIRED_INSTRUCTION_UNSUPPORTED",
                    format!(
                        "backend does not expose required native instruction '{}'",
                        instruction
                    ),
                    format!("native_instruction={instruction}"),
                );
            }
        }

        for gate in &circuit.gates {
            let normalized = normalize_instruction_name(gate);

            if normalized.is_empty() {
                report.error(
                    "INVALID_INSTRUCTION",
                    "workload contains an empty instruction identifier"
                        .to_string(),
                    "instruction_name!=empty".to_string(),
                );

                continue;
            }

            if self.capabilities.supports_gate(&normalized) {
                continue;
            }

            if is_single_qubit_rotation(&normalized)
                && self.capabilities.arbitrary_single_qubit_rotations
            {
                continue;
            }

            if is_parameterized_gate(&normalized)
                && self.capabilities.parameterized_gates
            {
                continue;
            }

            report.error(
                "UNSUPPORTED_GATE",
                format!(
                    "backend does not support instruction '{}'",
                    gate
                ),
                format!("native_instruction={normalized}"),
            );
        }
    }

    fn validate_topology_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;

        if circuit.qubit_count == 0 {
            return;
        }

        if !self.capabilities.topology_information {
            if !circuit.two_qubit_edges.is_empty()
                || requirements.requires_topology
            {
                report.error(
                    "TOPOLOGY_REQUIRED",
                    "workload requires topology information but backend does not expose it"
                        .to_string(),
                    "topology_information=true".to_string(),
                );
            }

            return;
        }

        if self.topology.qubit_count() == 0 {
            report.error(
                "EMPTY_TOPOLOGY",
                "backend topology contains zero resources".to_string(),
                "topology.qubit_count>0".to_string(),
            );

            return;
        }

        if circuit.qubit_count > self.topology.qubit_count() {
            report.error(
                "TOPOLOGY_QUBIT_LIMIT",
                format!(
                    "workload requires {} qubits but topology contains {}",
                    circuit.qubit_count,
                    self.topology.qubit_count()
                ),
                format!(
                    "qubit_count<={}",
                    self.topology.qubit_count()
                ),
            );
        }

        if circuit.two_qubit_edges.len()
            > MAX_REQUIRED_TOPOLOGY_EDGES
        {
            report.error(
                "TOPOLOGY_EDGE_LIMIT",
                format!(
                    "workload requires more than {} topology edges",
                    MAX_REQUIRED_TOPOLOGY_EDGES
                ),
                format!(
                    "topology_edge_count<={}",
                    MAX_REQUIRED_TOPOLOGY_EDGES
                ),
            );

            return;
        }

        for &(control, target) in &circuit.two_qubit_edges {
            if control >= circuit.qubit_count {
                report.error(
                    "INVALID_CONTROL_QUBIT",
                    format!(
                        "control qubit {} is outside workload range",
                        control
                    ),
                    format!(
                        "control_qubit<{}",
                        circuit.qubit_count
                    ),
                );

                continue;
            }

            if target >= circuit.qubit_count {
                report.error(
                    "INVALID_TARGET_QUBIT",
                    format!(
                        "target qubit {} is outside workload range",
                        target
                    ),
                    format!(
                        "target_qubit<{}",
                        circuit.qubit_count
                    ),
                );

                continue;
            }

            if control == target {
                report.error(
                    "SELF_INTERACTION",
                    format!(
                        "two-qubit operation cannot target the same qubit {} twice",
                        control
                    ),
                    "control!=target".to_string(),
                );

                continue;
            }

            match self.topology.is_connected(control, target) {
                Ok(true) => {}

                Ok(false) => report.error(
                    "UNSUPPORTED_CONNECTION",
                    format!(
                        "backend has no native connection from {} to {}",
                        control,
                        target
                    ),
                    format!("native_connection={control}->{target}"),
                ),

                Err(error) => report.error(
                    "TOPOLOGY_QUERY_FAILED",
                    error.to_string(),
                    format!("native_connection={control}->{target}"),
                ),
            }
        }
    }
}

// =============================================================================
// Borrowed descriptor
// =============================================================================

/// Immutable borrowed backend descriptor.
///
/// This is useful for registries and discovery without cloning the entire
/// backend.
#[derive(Debug, Clone, Copy)]
pub struct BackendDescriptor<'a> {
    backend: &'a QuantumBackend,
}

impl<'a> BackendDescriptor<'a> {
    pub fn id(self) -> &'a str {
        self.backend.id()
    }

    pub fn provider(self) -> &'a str {
        self.backend.provider()
    }

    pub const fn kind(self) -> BackendKind {
        self.backend.kind()
    }

    pub const fn status(self) -> BackendStatus {
        self.backend.status()
    }

    pub const fn qubit_count(self) -> usize {
        self.backend.qubit_count()
    }

    pub const fn coupling_count(self) -> usize {
        self.backend.coupling_count()
    }

    pub fn capabilities(self) -> &'a BackendCapabilities {
        &self.backend.capabilities
    }

    pub fn limits(self) -> BackendLimits {
        self.backend.limits
    }

    pub fn topology(self) -> &'a HardwareTopology {
        &self.backend.topology
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_backend_metadata(
    metadata: &BackendMetadata,
) -> Result<(), BackendError> {
    validate_identifier(
        "backend_id",
        &metadata.id,
        MAX_BACKEND_ID_LENGTH,
    )?;

    validate_identifier(
        "backend_name",
        &metadata.name,
        MAX_BACKEND_NAME_LENGTH,
    )?;

    validate_identifier(
        "provider_id",
        &metadata.provider,
        MAX_PROVIDER_ID_LENGTH,
    )?;

    validate_identifier(
        "backend_version",
        &metadata.version,
        MAX_BACKEND_VERSION_LENGTH,
    )?;

    for (key, value) in &metadata.properties {
        validate_metadata_field(key, value)?;

        if looks_like_secret_key(key) {
            return Err(BackendError::SecretLikeMetadata {
                key: key.clone(),
            });
        }
    }

    if metadata.properties.len() > MAX_METADATA_PROPERTIES {
        return Err(BackendError::MetadataLimitExceeded {
            maximum: MAX_METADATA_PROPERTIES,
        });
    }

    Ok(())
}

fn validate_capabilities(
    capabilities: &BackendCapabilities,
) -> Result<(), BackendError> {
    if capabilities.native_gates.len() > MAX_NATIVE_INSTRUCTIONS {
        return Err(BackendError::RequiredInstructionLimitExceeded {
            maximum: MAX_NATIVE_INSTRUCTIONS,
        });
    }

    for gate in &capabilities.native_gates {
        if gate.trim().is_empty() {
            return Err(BackendError::InvalidIdentifier {
                field: "native_instruction",
            });
        }
    }

    Ok(())
}

fn validate_topology(
    topology: &HardwareTopology,
) -> Result<(), BackendError> {
    if topology.qubit_count() == 0 {
        return Err(BackendError::TopologyUnavailable);
    }

    topology
        .validate()
        .map_err(BackendError::Topology)?;

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(if field == "backend_id" {
            BackendError::InvalidBackendId
        } else {
            BackendError::InvalidIdentifier {
                field,
            }
        });
    }

    if trimmed.len() > maximum {
        return Err(BackendError::IdentifierTooLong {
            field,
            maximum,
        });
    }

    if trimmed.chars().any(char::is_control) {
        return Err(BackendError::InvalidIdentifier {
            field,
        });
    }

    Ok(())
}

fn validate_metadata_field(
    key: &str,
    value: &str,
) -> Result<(), BackendError> {
    if key.trim().is_empty()
        || key.len() > MAX_METADATA_KEY_LENGTH
        || key.chars().any(char::is_control)
    {
        return Err(BackendError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    Ok(())
}

/// Conservative secret-key detection.
///
/// This is deliberately only a safety guard. It is not a replacement for a
/// dedicated credential system.
fn looks_like_secret_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();

    const SECRET_MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "auth_token",
        "authorization",
        "password",
        "passwd",
        "private_key",
        "secret",
        "client_secret",
        "session_cookie",
        "cookie",
    ];

    SECRET_MARKERS.iter().any(|marker| key.contains(marker))
}

/// Conservative secret-value detection for caller-defined identifiers.
fn looks_like_secret_value(value: &str) -> bool {
    let value = value.trim();

    value.len() > 256
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'-' | b'_' | b'.' | b'/' | b'+' | b'='
                )
        })
}

fn normalize_instruction_name(name: &str) -> String {
    name.trim().to_ascii_uppercase()
}

fn normalize_capability_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('-', "_")
}

fn is_single_qubit_rotation(gate: &str) -> bool {
    matches!(
        gate,
        "RX"
            | "RY"
            | "RZ"
            | "U"
            | "U1"
            | "U2"
            | "U3"
            | "PHASE"
            | "P"
    )
}

fn is_parameterized_gate(gate: &str) -> bool {
    matches!(
        gate,
        "RX"
            | "RY"
            | "RZ"
            | "U"
            | "U1"
            | "U2"
            | "U3"
            | "PHASE"
            | "P"
    )
}

fn first_validation_error(
    report: &BackendValidationReport,
) -> Result<(), BackendError> {
    let diagnostic = match report.errors().next() {
        Some(diagnostic) => diagnostic,
        None => return Ok(()),
    };

    match diagnostic.code {
        "BACKEND_BUSY" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Busy,
        }),

        "BACKEND_STATUS_UNKNOWN" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Unknown,
        }),

        "BACKEND_MAINTENANCE" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Maintenance,
        }),

        "BACKEND_OFFLINE" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Offline,
        }),

        "BACKEND_RETIRED" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Retired,
        }),

        "BACKEND_UNAVAILABLE" => Err(BackendError::BackendUnavailable {
            backend_id: report.backend_id.clone(),
            status: BackendStatus::Unavailable,
        }),

        "ZERO_QUBITS" => Err(BackendError::ZeroQubits),

        "QUBIT_LIMIT" | "TOPOLOGY_QUBIT_LIMIT" => {
            Err(BackendError::QubitLimitExceeded {
                requested: 0,
                maximum: 0,
            })
        }

        "LOGICAL_QUBIT_LIMIT" => {
            Err(BackendError::LogicalQubitLimitExceeded {
                requested: 0,
                maximum: 0,
            })
        }

        "CIRCUIT_DEPTH_LIMIT" => {
            Err(BackendError::CircuitDepthExceeded {
                requested: 0,
                maximum: 0,
            })
        }

        "OPERATION_LIMIT" => {
            Err(BackendError::OperationLimitExceeded {
                requested: 0,
                maximum: 0,
            })
        }

        "SHOT_LIMIT" => Err(BackendError::ShotLimitExceeded {
            requested: 0,
            maximum: 0,
        }),

        "CLASSICAL_BIT_LIMIT" => {
            Err(BackendError::ClassicalBitLimitExceeded {
                requested: 0,
                maximum: 0,
            })
        }

        "MEASUREMENT_UNSUPPORTED" => {
            Err(BackendError::MeasurementUnsupported)
        }

        "RESET_UNSUPPORTED" => {
            Err(BackendError::ResetUnsupported)
        }

        "MID_CIRCUIT_MEASUREMENT_UNSUPPORTED" => {
            Err(BackendError::MidCircuitMeasurementUnsupported)
        }

        "CLASSICAL_CONTROL_UNSUPPORTED" => {
            Err(BackendError::ClassicalControlUnsupported)
        }

        "DYNAMIC_CIRCUIT_UNSUPPORTED" => {
            Err(BackendError::DynamicCircuitUnsupported)
        }

        "PULSE_CONTROL_UNSUPPORTED" => {
            Err(BackendError::PulseControlUnsupported)
        }

        "ANALOG_CONTROL_UNSUPPORTED" => {
            Err(BackendError::AnalogControlUnsupported)
        }

        "ANNEALING_UNSUPPORTED" => {
            Err(BackendError::AnnealingUnsupported)
        }

        "LOGICAL_QUBITS_UNSUPPORTED" => {
            Err(BackendError::LogicalQubitsUnsupported)
        }

        "FAULT_TOLERANCE_UNSUPPORTED" => {
            Err(BackendError::FaultToleranceUnsupported)
        }

        "DETERMINISTIC_SEED_UNSUPPORTED" => {
            Err(BackendError::DeterministicSeedingUnsupported)
        }

        "STATE_VECTOR_UNSUPPORTED" => {
            Err(BackendError::StateVectorUnsupported)
        }

        "DENSITY_MATRIX_UNSUPPORTED" => {
            Err(BackendError::DensityMatrixUnsupported)
        }

        "EXPECTATION_VALUES_UNSUPPORTED" => {
            Err(BackendError::ExpectationValuesUnsupported)
        }

        "UNSUPPORTED_GATE"
        | "REQUIRED_INSTRUCTION_UNSUPPORTED" => {
            Err(BackendError::UnsupportedGate {
                gate: diagnostic.requirement.clone(),
            })
        }

        "TOPOLOGY_INFORMATION_UNAVAILABLE"
        | "TOPOLOGY_REQUIRED" => {
            Err(BackendError::TopologyUnavailable)
        }

        "UNSUPPORTED_CONNECTION" => {
            Err(BackendError::UnsupportedConnection {
                control: 0,
                target: 0,
            })
        }

        "CALIBRATION_UNAVAILABLE"
        | "FRESH_CALIBRATION_UNAVAILABLE" => {
            Err(BackendError::CalibrationUnavailable)
        }

        _ => Err(BackendError::ExecutionRejected(
            diagnostic.message.clone(),
        )),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn topology() -> HardwareTopology {
        HardwareTopology::linear(4)
            .expect("four-resource linear topology must be valid")
    }

    fn backend() -> QuantumBackend {
        let metadata = BackendMetadata::new(
            "test-simulator",
            "Test Simulator",
            "Zamani",
            "1.0.0",
            BackendKind::Simulator,
        );

        let capabilities = BackendCapabilities::new()
            .with_gates([
                "H",
                "X",
                "Y",
                "Z",
                "S",
                "T",
                "CX",
                "CZ",
                "SWAP",
                "MEASURE",
            ]);

        QuantumBackend::new(
            metadata,
            capabilities,
            BackendLimits::unlimited(),
            topology(),
        )
        .expect("backend should be valid")
    }

    #[test]
    fn backend_metadata_requires_id() {
        let metadata = BackendMetadata::new(
            "",
            "Test",
            "Zamani",
            "1.0",
            BackendKind::Simulator,
        );

        assert!(matches!(
            QuantumBackend::new(
                metadata,
                BackendCapabilities::default(),
                BackendLimits::unlimited(),
                topology(),
            ),
            Err(BackendError::InvalidBackendId)
        ));
    }

    #[test]
    fn gate_names_are_normalized() {
        let capabilities =
            BackendCapabilities::new().with_gate(" cx ");

        assert!(capabilities.supports_gate("CX"));
        assert!(capabilities.supports_gate("cx"));
    }

    #[test]
    fn native_gate_list_is_deterministic() {
        let capabilities = BackendCapabilities::new()
            .with_gates(["Z", "X", "H", "CX"]);

        let gates = capabilities
            .native_gates
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            gates,
            vec![
                "CX".to_string(),
                "H".to_string(),
                "X".to_string(),
                "Z".to_string(),
            ]
        );
    }

    #[test]
    fn backend_accepts_supported_circuit() {
        let backend = backend();

        let requirements = CircuitRequirements {
            qubit_count: 2,
            circuit_depth: 2,
            operation_count: 3,
            shots: 10,
            gates: vec!["H".into(), "CX".into()],
            two_qubit_edges: vec![(0, 1)],
            requires_measurement: true,
            ..Default::default()
        };

        assert!(backend.validate_circuit(&requirements).is_ok());
    }

    #[test]
    fn backend_rejects_unsupported_gate() {
        let backend = backend();

        let requirements = CircuitRequirements {
            qubit_count: 1,
            gates: vec!["TOFFOLI".into()],
            ..Default::default()
        };

        assert!(matches!(
            backend.validate_circuit(&requirements),
            Err(BackendError::UnsupportedGate { .. })
        ));
    }

    #[test]
    fn backend_rejects_excessive_qubits() {
        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "limited",
                "Limited",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            ),
            BackendCapabilities::new().with_gate("H"),
            BackendLimits::unlimited().with_max_qubits(2),
            topology(),
        )
        .expect("backend should be valid");

        let requirements = CircuitRequirements {
            qubit_count: 3,
            ..Default::default()
        };

        assert!(matches!(
            backend.validate_circuit(&requirements),
            Err(BackendError::QubitLimitExceeded { .. })
        ));
    }

    #[test]
    fn backend_rejects_unavailable_backend() {
        let mut backend = backend();

        backend.set_status(BackendStatus::Maintenance);

        let requirements = CircuitRequirements {
            qubit_count: 1,
            ..Default::default()
        };

        assert!(matches!(
            backend.validate_circuit(&requirements),
            Err(BackendError::BackendUnavailable {
                status: BackendStatus::Maintenance,
                ..
            })
        ));
    }

    #[test]
    fn backend_rejects_unsupported_connection() {
        let backend = backend();

        let requirements = CircuitRequirements {
            qubit_count: 4,
            gates: vec!["CX".into()],
            two_qubit_edges: vec![(0, 3)],
            ..Default::default()
        };

        let result = backend.validate_circuit(&requirements);

        assert!(matches!(
            result,
            Err(BackendError::UnsupportedConnection {
                control: 0,
                target: 3
            })
        ));
    }

    #[test]
    fn topology_direction_is_respected() {
        let topology = HardwareTopology::from_couplings(
            2,
            [
                super::super::topology::Coupling::directed(0, 1),
            ],
        )
        .expect("directed topology should be valid");

        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "directed",
                "Directed",
                "Zamani",
                "1.0",
                BackendKind::Qpu,
            ),
            BackendCapabilities::new().with_gate("CX"),
            BackendLimits::unlimited(),
            topology,
        )
        .expect("backend should be valid");

        let forward = CircuitRequirements {
            qubit_count: 2,
            gates: vec!["CX".into()],
            two_qubit_edges: vec![(0, 1)],
            ..Default::default()
        };

        let reverse = CircuitRequirements {
            qubit_count: 2,
            gates: vec!["CX".into()],
            two_qubit_edges: vec![(1, 0)],
            ..Default::default()
        };

        assert!(backend.validate_circuit(&forward).is_ok());

        assert!(matches!(
            backend.validate_circuit(&reverse),
            Err(BackendError::UnsupportedConnection {
                control: 1,
                target: 0
            })
        ));
    }

    #[test]
    fn arbitrary_rotations_are_supported_when_enabled() {
        let metadata = BackendMetadata::new(
            "rotation-backend",
            "Rotation Backend",
            "Zamani",
            "1.0",
            BackendKind::Simulator,
        );

        let capabilities = BackendCapabilities {
            arbitrary_single_qubit_rotations: true,
            ..BackendCapabilities::default()
        };

        let backend = QuantumBackend::new(
            metadata,
            capabilities,
            BackendLimits::unlimited(),
            topology(),
        )
        .expect("backend should be valid");

        let requirements = CircuitRequirements {
            qubit_count: 1,
            gates: vec!["RX".into()],
            ..Default::default()
        };

        assert!(backend.validate_circuit(&requirements).is_ok());
    }

    #[test]
    fn dynamic_circuit_requires_dynamic_capability() {
        let backend = backend();

        let requirements = CircuitRequirements {
            qubit_count: 2,
            requires_dynamic_circuits: true,
            ..Default::default()
        };

        assert!(matches!(
            backend.validate_circuit(&requirements),
            Err(BackendError::DynamicCircuitUnsupported)
        ));
    }

    #[test]
    fn dynamic_circuit_is_accepted_when_supported() {
        let metadata = BackendMetadata::new(
            "dynamic",
            "Dynamic",
            "Zamani",
            "1.0",
            BackendKind::Qpu,
        );

        let capabilities = BackendCapabilities {
            dynamic_circuits: true,
            mid_circuit_measurement: true,
            classical_control: true,
            ..BackendCapabilities::default()
        };

        let backend = QuantumBackend::new(
            metadata,
            capabilities,
            BackendLimits::unlimited(),
            topology(),
        )
        .expect("backend should be valid");

        let requirements = CircuitRequirements {
            qubit_count: 2,
            requires_measurement: true,
            requires_mid_circuit_measurement: true,
            requires_classical_control: true,
            requires_dynamic_circuits: true,
            ..Default::default()
        };

        assert!(backend.validate_circuit(&requirements).is_ok());
    }

    #[test]
    fn analog_workload_requires_analog_capability() {
        let backend = backend();

        let requirements = WorkloadRequirements {
            kind: QuantumWorkloadKind::AnalogProgram,
            circuit: CircuitRequirements {
                qubit_count: 2,
                requires_analog_control: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(matches!(
            backend.validate(&requirements),
            Err(BackendError::UnsupportedWorkload {
                workload: QuantumWorkloadKind::AnalogProgram
            })
        ));
    }

    #[test]
    fn pulse_workload_requires_pulse_capability() {
        let backend = backend();

        let requirements = WorkloadRequirements {
            kind: QuantumWorkloadKind::PulseProgram,
            circuit: CircuitRequirements {
                qubit_count: 1,
                requires_pulse_control: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(matches!(
            backend.validate(&requirements),
            Err(BackendError::UnsupportedWorkload {
                workload: QuantumWorkloadKind::PulseProgram
            })
        ));
    }

    #[test]
    fn logical_workload_requires_logical_capability() {
        let backend = backend();

        let requirements = WorkloadRequirements {
            kind: QuantumWorkloadKind::LogicalProgram,
            circuit: CircuitRequirements {
                qubit_count: 5,
                logical_qubit_count: 1,
                requires_logical_qubits: true,
                ..Default::default()
            },
            ..Default::default()
        };

        assert!(matches!(
            backend.validate(&requirements),
            Err(BackendError::UnsupportedWorkload {
                workload: QuantumWorkloadKind::LogicalProgram
            })
        ));
    }

    #[test]
    fn metadata_rejects_secret_like_fields() {
        let mut metadata = BackendMetadata::new(
            "safe",
            "Safe",
            "Zamani",
            "1.0",
            BackendKind::Simulator,
        );

        assert!(matches!(
            metadata.insert_property(
                "api_key",
                "do-not-store-this"
            ),
            Err(BackendError::SecretLikeMetadata { .. })
        ));
    }

    #[test]
    fn result_counts_are_bounded_by_shots() {
        let mut result =
            ExecutionResult::empty("local", 10).expect("valid result");

        result
            .insert_count("000", 7)
            .expect("first count valid");

        result
            .insert_count("111", 3)
            .expect("second count valid");

        assert!(result.counts_match_shots());
        assert_eq!(result.counted_shots(), 10);
    }

    #[test]
    fn result_counts_cannot_exceed_shots() {
        let mut result =
            ExecutionResult::empty("local", 10).expect("valid result");

        result
            .insert_count("000", 11)
            .expect_err("count must not exceed shots");
    }

    #[test]
    fn request_validation_uses_backend_contract() {
        let backend = backend();

        let request = ExecutionRequest::new(CircuitRequirements {
            qubit_count: 2,
            gates: vec!["H", "CX"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            two_qubit_edges: vec![(0, 1)],
            ..Default::default()
        });

        assert!(backend.validate_request(&request).is_ok());
    }

    #[test]
    fn deterministic_seed_requires_capability() {
        let backend = backend();

        let request = ExecutionRequest::new(CircuitRequirements {
            qubit_count: 1,
            ..Default::default()
        })
        .with_seed(42);

        assert!(matches!(
            backend.validate_request(&request),
            Err(BackendError::DeterministicSeedingUnsupported)
        ));
    }

    #[test]
    fn deterministic_seed_is_accepted_when_supported() {
        let capabilities = BackendCapabilities {
            deterministic_seeding: true,
            ..BackendCapabilities::default()
        };

        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "seeded",
                "Seeded",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            ),
            capabilities,
            BackendLimits::unlimited(),
            topology(),
        )
        .expect("backend should be valid");

        let request = ExecutionRequest::new(CircuitRequirements {
            qubit_count: 1,
            ..Default::default()
        })
        .with_seed(42);

        assert!(backend.validate_request(&request).is_ok());
    }

    #[test]
    fn validation_report_is_machine_readable() {
        let backend = backend();

        let requirements = CircuitRequirements {
            qubit_count: 2,
            gates: vec!["UNKNOWN_GATE".into()],
            ..Default::default()
        };

        let report = backend.validation_report(
            &WorkloadRequirements::from_circuit(requirements),
        );

        assert!(!report.valid);
        assert!(report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "UNSUPPORTED_GATE"));
    }

    #[test]
    fn backend_descriptor_is_non_owning() {
        let backend = backend();
        let descriptor = backend.descriptor();

        assert_eq!(descriptor.id(), "test-simulator");
        assert_eq!(descriptor.provider(), "Zamani");
        assert_eq!(descriptor.kind(), BackendKind::Simulator);
        assert_eq!(descriptor.qubit_count(), 4);
        assert_eq!(descriptor.coupling_count(), 3);
    }

    #[test]
    fn degraded_backend_is_reported_as_warning() {
        let mut backend = backend();
        backend.set_status(BackendStatus::Degraded);

        let requirements = CircuitRequirements {
            qubit_count: 1,
            ..Default::default()
        };

        let report = backend.validation_report(
            &WorkloadRequirements::from_circuit(requirements),
        );

        assert!(report.valid);
        assert!(report
            .warnings()
            .any(|warning| warning.code == "BACKEND_DEGRADED"));
    }

    #[test]
    fn topology_api_is_used_without_private_field_access() {
        let topology =
            HardwareTopology::linear(3).expect("valid topology");

        assert_eq!(topology.qubit_count(), 3);
        assert_eq!(topology.coupling_count(), 2);
        assert!(
            topology
                .is_connected(0, 1)
                .expect("valid qubits")
        );
        assert!(
            topology
                .is_connected(1, 0)
                .expect("valid qubits")
        );
    }
}