//! Zamani Quantum — Canonical Hardware Backend
//!
//! Production-grade, provider-independent quantum hardware boundary.
//!
//! # Responsibility
//!
//! This module defines the canonical backend descriptor and the stable
//! provider-neutral contracts required by the rest of Zamani Quantum.
//!
//! It owns:
//!
//! - backend identity and metadata;
//! - backend kind;
//! - backend operational status;
//! - backend capabilities;
//! - backend resource limits;
//! - workload requirements;
//! - execution-request requirements;
//! - normalized execution results;
//! - deterministic validation;
//! - topology-aware validation;
//! - capability negotiation primitives;
//! - provider-neutral backend errors;
//! - immutable backend descriptors;
//! - security validation for backend metadata;
//! - compatibility contracts consumed by compatibility, validation,
//!   routing, scheduling, benchmarking, registries, Danga and adapters.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP/network communication;
//! - provider authentication;
//! - credentials;
//! - API tokens;
//! - provider SDKs;
//! - transpilation;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - calibration storage;
//! - benchmarking mathematics;
//! - QEC algorithms;
//! - OpenQASM parsing;
//! - QIR generation;
//! - simulation;
//! - emulation.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      +-------------------+
//!      |                   |
//!      v                   v
//! optimization       error correction
//!      |                   |
//!      +---------+---------+
//!                |
//!                v
//!       compatibility analysis
//!                |
//!          +-----+-----+
//!          |           |
//!          v           v
//!       routing     scheduling
//!          |           |
//!          +-----+-----+
//!                |
//!                v
//!       Workload Requirements
//!                |
//!                v
//!        QuantumBackend
//!                |
//!        +-------+-------+
//!        |       |       |
//!        v       v       v
//!      local  provider  simulator
//!     adapter adapter   adapter
//!        |       |       |
//!        +-------+-------+
//!                |
//!                v
//!           execution.rs
//!                |
//!                v
//!              Job
//!                |
//!                v
//!             Result
//!
//! benchmarking consumes this boundary.
//! hardware never depends on benchmarking.
//! ```
//!
//! # Integration contract
//!
//! This file intentionally depends only on the Rust standard library and the
//! authoritative `hardware::topology` module.
//!
//! Future hardware modules consume this contract:
//!
//! - `backend_trait.rs`
//! - `backend_config.rs`
//! - `backend_status.rs`
//! - `capabilities.rs`
//! - `compatibility.rs`
//! - `validation.rs`
//! - `execution.rs`
//! - `job.rs`
//! - `queue.rs`
//! - `result.rs`
//! - `provider.rs`
//! - `provider_registry.rs`
//! - `device_registry.rs`
//! - `discovery.rs`
//! - provider adapters;
//! - benchmarking;
//! - Danga.
//!
//! Those modules must consume this stable provider-neutral contract instead of
//! making this module provider-specific.
//!
//! # Stability rule
//!
//! The following types form the compatibility surface currently consumed by
//! other Zamani Quantum modules:
//!
//! - `BackendKind`;
//! - `BackendStatus`;
//! - `BackendCapabilities`;
//! - `BackendLimits`;
//! - `BackendMetadata`;
//! - `QuantumWorkloadKind`;
//! - `CircuitRequirements`;
//! - `WorkloadRequirements`;
//! - `ExecutionRequest`;
//! - `ExecutionResult`;
//! - `BackendError`;
//! - `QuantumBackend`;
//! - `BackendDescriptor`.
//!
//! New provider-specific behaviour must be implemented through adapters.
//!
//! # Security
//!
//! This module never stores credentials.
//!
//! Backend metadata rejects fields that appear to contain:
//!
//! - API keys;
//! - access tokens;
//! - authorization headers;
//! - passwords;
//! - private keys;
//! - secrets;
//! - session cookies.
//!
//! This is a defence-in-depth measure, not a credential-management system.
//!
//! # Determinism
//!
//! All externally observable collections use deterministic ordering:
//!
//! - `BTreeMap`;
//! - `BTreeSet`;
//! - sorted diagnostics;
//! - normalized instruction identifiers;
//! - canonical capability identifiers.
//!
//! No system clock, random source, network state or provider state is read by
//! validation.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Topology boundary
//!
//! `HardwareTopology` is authoritative for physical connectivity.
//!
//! This file never accesses topology internals directly. It uses the public
//! topology API:
//!
//! - `qubit_count()`;
//! - `coupling_count()`;
//! - `is_connected()`;
//! - `validate()`.
//!
//! Directional topology semantics therefore remain owned by `topology.rs`.
//!
//! # Important semantic distinction
//!
//! `BackendKind` answers:
//!
//! > What type of execution target is this?
//!
//! `QuantumWorkloadKind` answers:
//!
//! > What kind of quantum workload is being requested?
//!
//! `BackendCapabilities` answers:
//!
//! > What can this backend do?
//!
//! `BackendLimits` answers:
//!
//! > What is the maximum resource envelope?
//!
//! `HardwareTopology` answers:
//!
//! > What physical connectivity exists?
//!
//! These concepts must not be collapsed into one enum or structure.

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

/// Semantic version of the backend schema.
///
/// Increment this value only when the meaning of the public serialized/backend
/// contract changes incompatibly.
pub const BACKEND_SCHEMA_VERSION: u16 = 3;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum backend name length.
pub const MAX_BACKEND_NAME_LENGTH: usize = 512;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum backend version length.
pub const MAX_BACKEND_VERSION_LENGTH: usize = 128;

/// Maximum hardware revision length.
pub const MAX_HARDWARE_REVISION_LENGTH: usize = 128;

/// Maximum firmware version length.
pub const MAX_FIRMWARE_VERSION_LENGTH: usize = 128;

/// Maximum API version length.
pub const MAX_API_VERSION_LENGTH: usize = 128;

/// Maximum region identifier length.
pub const MAX_REGION_LENGTH: usize = 256;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum metadata property count.
pub const MAX_METADATA_PROPERTIES: usize = 4096;

/// Maximum stable native instruction count.
pub const MAX_NATIVE_INSTRUCTIONS: usize = 1_000_000;

/// Maximum required instruction count.
pub const MAX_REQUIRED_INSTRUCTIONS: usize = 1_000_000;

/// Maximum required topology edge count.
pub const MAX_REQUIRED_TOPOLOGY_EDGES: usize = 10_000_000;

/// Maximum request metadata properties.
pub const MAX_REQUEST_METADATA_PROPERTIES: usize = 4096;

/// Maximum request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

// =============================================================================
// Backend kind
// =============================================================================

/// High-level execution-target category.
///
/// This is intentionally separate from physical quantum technology.
///
/// For example:
///
/// ```text
/// technology = superconducting
/// backend_kind = Qpu
/// workload = GateCircuit
/// ```
///
/// or:
///
/// ```text
/// technology = neutral_atom
/// backend_kind = Qpu
/// workload = AnalogProgram
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendKind {
    /// Classical software simulator.
    Simulator,

    /// Software model approximating a particular hardware architecture.
    Emulator,

    /// Physical quantum processing unit.
    Qpu,

    /// Repository/provider-specific execution implementation.
    Custom,
}

impl BackendKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::Custom => "custom",
        }
    }

    /// Returns true for physical QPUs.
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns true for software execution targets.
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

/// Current operational state of a backend.
///
/// Status is intentionally independent from backend identity and capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendStatus {
    /// No authoritative status has been established.
    Unknown,

    /// Backend is operational and can normally accept work.
    Available,

    /// Backend is operational but currently occupied.
    Busy,

    /// Backend is undergoing maintenance.
    Maintenance,

    /// Backend is operational but degraded.
    Degraded,

    /// Backend is unreachable/offline.
    Offline,

    /// Backend has permanently retired.
    Retired,

    /// Backend is temporarily unavailable.
    Unavailable,
}

impl BackendStatus {
    /// Returns true if normal submission is permitted by status alone.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns true if the device is known to be operational.
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::Busy
                | Self::Maintenance
                | Self::Degraded
        )
    }

    /// Stable machine-readable identifier.
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

/// Provider-neutral capabilities advertised by a backend.
///
/// The structure intentionally covers more than conventional gate-model QPUs.
///
/// Stable and experimental capabilities are kept separate. An experimental
/// capability MUST NOT silently satisfy a stable capability requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// Terminal measurement.
    pub measurement: bool,

    /// Quantum reset.
    pub reset: bool,

    /// Mid-circuit measurement.
    pub mid_circuit_measurement: bool,

    /// Measurement-dependent classical feed-forward.
    pub classical_control: bool,

    /// Dynamic circuit execution.
    pub dynamic_circuits: bool,

    /// Arbitrary one-qubit rotations.
    pub arbitrary_single_qubit_rotations: bool,

    /// Unbound/parameterized gates.
    pub parameterized_gates: bool,

    /// Three-qubit native operations.
    pub three_qubit_operations: bool,

    /// Operations with more than three quantum operands.
    pub multi_qubit_operations: bool,

    /// Parallel execution.
    pub parallel_operations: bool,

    /// Batched execution.
    pub batch_execution: bool,

    /// Streaming result support.
    pub streaming_results: bool,

    /// Provider-side job cancellation.
    pub cancellation: bool,

    /// Queue information.
    pub queue_information: bool,

    /// Pulse-level execution.
    pub pulse_control: bool,

    /// Analog Hamiltonian/control execution.
    pub analog_control: bool,

    /// Quantum annealing/Ising/QUBO execution.
    pub annealing: bool,

    /// Logical-qubit execution.
    pub logical_qubits: bool,

    /// Fault-tolerant execution.
    pub fault_tolerance: bool,

    /// Syndrome measurement.
    pub syndrome_measurement: bool,

    /// Provider-side decoder execution.
    pub decoder_execution: bool,

    /// Deterministic seeded execution.
    pub deterministic_seeding: bool,

    /// State-vector result access.
    pub state_vector_results: bool,

    /// Density-matrix result access.
    pub density_matrix_results: bool,

    /// Expectation-value result access.
    pub expectation_value_results: bool,

    /// Readout-error mitigation.
    pub readout_mitigation: bool,

    /// General error mitigation.
    pub error_mitigation: bool,

    /// Calibration information is exposed.
    pub calibration_data: bool,

    /// Timing information is exposed.
    pub timing_information: bool,

    /// Physical topology is exposed.
    pub topology_information: bool,

    /// Native instruction set is exposed.
    pub native_instruction_set: bool,

    /// Stable native instruction identifiers.
    pub native_gates: BTreeSet<String>,

    /// Experimental provider capabilities.
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

    /// Adds one stable native instruction.
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

    /// Returns whether a named stable capability is supported.
    pub fn supports_capability(&self, capability: &str) -> bool {
        let capability = normalize_capability_name(capability);

        match capability.as_str() {
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
            "expectation_value_results" => self.expectation_value_results,
            "readout_mitigation" => self.readout_mitigation,
            "error_mitigation" => self.error_mitigation,
            "calibration_data" => self.calibration_data,
            "timing_information" => self.timing_information,
            "topology_information" => self.topology_information,
            "native_instruction_set" => self.native_instruction_set,
            _ => false,
        }
    }

    /// Returns true if the named capability is explicitly marked experimental.
    pub fn is_experimental(&self, capability: &str) -> bool {
        self.experimental_capabilities
            .contains(&normalize_capability_name(capability))
    }

    /// Returns all stable capability identifiers in deterministic order.
    pub fn stable_names(&self) -> Vec<String> {
        let mut names = BTreeSet::new();

        macro_rules! add {
            ($field:ident, $name:literal) => {
                if self.$field {
                    names.insert($name.to_string());
                }
            };
        }

        add!(measurement, "measurement");
        add!(reset, "reset");
        add!(mid_circuit_measurement, "mid_circuit_measurement");
        add!(classical_control, "classical_control");
        add!(dynamic_circuits, "dynamic_circuits");
        add!(
            arbitrary_single_qubit_rotations,
            "arbitrary_single_qubit_rotations"
        );
        add!(parameterized_gates, "parameterized_gates");
        add!(three_qubit_operations, "three_qubit_operations");
        add!(multi_qubit_operations, "multi_qubit_operations");
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
        add!(expectation_value_results, "expectation_value_results");
        add!(readout_mitigation, "readout_mitigation");
        add!(error_mitigation, "error_mitigation");
        add!(calibration_data, "calibration_data");
        add!(timing_information, "timing_information");
        add!(topology_information, "topology_information");
        add!(native_instruction_set, "native_instruction_set");

        names.into_iter().collect()
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Hard backend resource limits.
///
/// A value of `0` means that the provider has not supplied a finite limit.
///
/// Therefore `0` means "unspecified/unbounded" rather than "zero".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendLimits {
    /// Maximum physical quantum resources.
    pub max_qubits: usize,

    /// Maximum logical qubits.
    pub max_logical_qubits: usize,

    /// Maximum circuit depth.
    pub max_circuit_depth: usize,

    /// Maximum operation count.
    pub max_operations: usize,

    /// Maximum shots.
    pub max_shots: usize,

    /// Maximum classical bits/register elements.
    pub max_classical_bits: usize,

    /// Maximum concurrent jobs.
    pub max_concurrent_jobs: usize,

    /// Maximum provider submission batch size.
    pub max_batch_size: usize,
}

impl Default for BackendLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

impl BackendLimits {
    /// Creates an unspecified/unbounded limit profile.
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

/// Provider-neutral backend metadata.
///
/// No credential material is permitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendMetadata {
    /// Stable canonical backend identifier.
    pub id: String,

    /// Human-readable backend name.
    pub name: String,

    /// Stable provider identifier.
    pub provider: String,

    /// Backend semantic/API version.
    pub version: String,

    /// Execution-target kind.
    pub kind: BackendKind,

    /// Current operational status.
    pub status: BackendStatus,

    /// Physical hardware revision.
    pub hardware_revision: Option<String>,

    /// Firmware version.
    pub firmware_version: Option<String>,

    /// Provider API version.
    pub api_version: Option<String>,

    /// Provider region/location.
    pub region: Option<String>,

    /// Non-secret provider metadata.
    pub properties: BTreeMap<String, String>,
}

impl BackendMetadata {
    /// Creates metadata.
    ///
    /// Validation is performed by `QuantumBackend::new`.
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

    /// Changes only operational status.
    pub fn set_status(&mut self, status: BackendStatus) {
        self.status = status;
    }

    /// Adds hardware revision metadata.
    pub fn with_hardware_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.hardware_revision = Some(revision.into());
        self
    }

    /// Adds firmware version metadata.
    pub fn with_firmware_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.firmware_version = Some(version.into());
        self
    }

    /// Adds provider API version metadata.
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }

    /// Adds provider region metadata.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Inserts non-secret metadata with security and size validation.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), BackendError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_field(&key, &value)?;

        if looks_like_secret_key(&key) {
            return Err(BackendError::SecretLikeMetadata { key });
        }

        if self.properties.len() >= MAX_METADATA_PROPERTIES
            && !self.properties.contains_key(&key)
        {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_METADATA_PROPERTIES,
            });
        }

        self.properties.insert(key, value);
        Ok(())
    }
}

// =============================================================================
// Quantum workload kinds
// =============================================================================

/// Canonical provider-neutral quantum workload category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumWorkloadKind {
    /// Conventional gate-model circuit.
    GateCircuit,

    /// Measurement/classical-control dependent circuit.
    DynamicCircuit,

    /// Direct pulse/control program.
    PulseProgram,

    /// Analog Hamiltonian/control program.
    AnalogProgram,

    /// Quantum annealing/Ising/QUBO workload.
    AnnealingProblem,

    /// Logical/fault-tolerant workload.
    LogicalProgram,

    /// Sampling workload.
    Sampling,

    /// Provider-specific workload.
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

/// Hardware requirements extracted from a gate-model workload.
///
/// This is NOT the canonical Quantum IR.
///
/// The canonical program remains owned by `quantum::ir`.
///
/// This structure is a compact hardware-compatibility view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitRequirements {
    /// Number of physical resources required.
    pub qubit_count: usize,

    /// Number of logical qubits required.
    pub logical_qubit_count: usize,

    /// Circuit depth.
    pub circuit_depth: usize,

    /// Number of quantum operations.
    pub operation_count: usize,

    /// Number of classical bits/register elements.
    pub classical_bit_count: usize,

    /// Number of requested execution shots.
    pub shots: usize,

    /// Instructions used by the workload.
    pub gates: Vec<String>,

    /// Required physical/native two-resource interactions.
    pub two_qubit_edges: Vec<(usize, usize)>,

    /// Terminal measurement is required.
    pub requires_measurement: bool,

    /// Reset is required.
    pub requires_reset: bool,

    /// Mid-circuit measurement is required.
    pub requires_mid_circuit_measurement: bool,

    /// Classical feed-forward is required.
    pub requires_classical_control: bool,

    /// Dynamic circuit support is required.
    pub requires_dynamic_circuits: bool,

    /// Pulse control is required.
    pub requires_pulse_control: bool,

    /// Analog control is required.
    pub requires_analog_control: bool,

    /// Annealing support is required.
    pub requires_annealing: bool,

    /// Logical qubits are required.
    pub requires_logical_qubits: bool,

    /// Fault-tolerant execution is required.
    pub requires_fault_tolerance: bool,

    /// Deterministic seeded execution is required.
    pub requires_deterministic_seed: bool,

    /// State-vector output is required.
    pub requires_state_vector: bool,

    /// Density-matrix output is required.
    pub requires_density_matrix: bool,

    /// Expectation-value output is required.
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
    /// Infers the workload category from the declared requirements.
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

    /// Returns true if any advanced workload feature is requested.
    pub fn is_advanced(&self) -> bool {
        !matches!(self.inferred_kind(), QuantumWorkloadKind::GateCircuit)
    }
}

// =============================================================================
// General workload requirements
// =============================================================================

/// Provider-neutral workload compatibility requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadRequirements {
    /// Explicit workload category.
    pub kind: QuantumWorkloadKind,

    /// Gate/circuit compatibility requirements.
    pub circuit: CircuitRequirements,

    /// Required stable capability identifiers.
    pub required_capabilities: BTreeSet<String>,

    /// Required native instructions.
    pub required_instructions: BTreeSet<String>,

    /// Physical topology information is required.
    pub requires_topology: bool,

    /// Calibration information is required.
    pub requires_calibration: bool,

    /// Fresh calibration is mandatory.
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
    /// Creates workload requirements from circuit requirements.
    pub fn from_circuit(circuit: CircuitRequirements) -> Self {
        let kind = circuit.inferred_kind();

        Self {
            kind,
            circuit,
            ..Self::default()
        }
    }

    /// Adds one required capability.
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

    /// Adds one required native instruction.
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

    /// Sets topology requirement.
    pub fn with_topology_requirement(mut self, required: bool) -> Self {
        self.requires_topology = required;
        self
    }

    /// Sets calibration requirement.
    pub fn with_calibration_requirement(mut self, required: bool) -> Self {
        self.requires_calibration = required;

        if !required {
            self.requires_fresh_calibration = false;
        }

        self
    }

    /// Sets fresh-calibration requirement.
    pub fn with_fresh_calibration_requirement(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_fresh_calibration = required;

        if required {
            self.requires_calibration = true;
        }

        self
    }

    /// Validates internal requirement-set invariants.
    pub fn validate(&self) -> Result<(), BackendError> {
        validate_workload_requirements(self)
    }
}

// =============================================================================
// Execution request
// =============================================================================

/// Provider-neutral execution request.
///
/// The actual executable Quantum IR/program payload remains outside this
/// structure. This object contains the hardware-facing execution policy and
/// compatibility requirements.
///
/// Provider adapters must perform `QuantumBackend::preflight()` before provider
/// submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    /// Hardware compatibility requirements.
    pub workload: WorkloadRequirements,

    /// Optional deterministic execution seed.
    pub seed: Option<u64>,

    /// Caller/provider scheduling priority.
    pub priority: u32,

    /// Whether asynchronous provider execution is permitted.
    pub asynchronous: bool,

    /// Optional caller request identifier.
    pub request_id: Option<String>,

    /// Non-secret execution metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionRequest {
    /// Creates a request from circuit requirements.
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

    /// Creates a request from general workload requirements.
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

    /// Requests deterministic execution with the supplied seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets execution priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Requests synchronous provider behaviour.
    pub fn synchronous(mut self) -> Self {
        self.asynchronous = false;
        self
    }

    /// Sets a validated request identifier.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let request_id = request_id.into();

        validate_identifier(
            "request_id",
            &request_id,
            MAX_REQUEST_ID_LENGTH,
        )?;

        self.request_id = Some(request_id);
        Ok(self)
    }

    /// Adds validated non-secret metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), BackendError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_field(&key, &value)?;

        if looks_like_secret_key(&key) {
            return Err(BackendError::SecretLikeMetadata { key });
        }

        if self.metadata.len() >= MAX_REQUEST_METADATA_PROPERTIES
            && !self.metadata.contains_key(&key)
        {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_REQUEST_METADATA_PROPERTIES,
            });
        }

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Validates request-local invariants without requiring a backend.
    pub fn validate_structure(&self) -> Result<(), BackendError> {
        self.workload.validate()?;

        if let Some(request_id) = &self.request_id {
            validate_identifier(
                "request_id",
                request_id,
                MAX_REQUEST_ID_LENGTH,
            )?;
        }

        if self.metadata.len() > MAX_REQUEST_METADATA_PROPERTIES {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_REQUEST_METADATA_PROPERTIES,
            });
        }

        for (key, value) in &self.metadata {
            validate_metadata_field(key, value)?;

            if looks_like_secret_key(key) {
                return Err(BackendError::SecretLikeMetadata {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Normalized provider-neutral execution result.
///
/// Providers may expose richer results. Those are normalized here where a
/// stable representation exists and may otherwise remain in metadata.
///
/// `counts` may be partial while a provider streams/constructs a result.
/// `counts_match_shots()` is therefore the completeness check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    /// Backend that produced the result.
    pub backend_id: String,

    /// Number of requested/represented shots.
    pub shots: usize,

    /// Normalized classical bitstring counts.
    pub counts: BTreeMap<String, usize>,

    /// Normalized expectation values.
    ///
    /// Values remain strings because the backend boundary deliberately avoids
    /// prescribing one floating-point representation for every provider and
    /// future observable type.
    pub expectation_values: BTreeMap<String, String>,

    /// Non-secret provider-neutral metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionResult {
    /// Creates an empty normalized result.
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

    /// Returns the number of samples represented by normalized counts.
    pub fn counted_shots(&self) -> usize {
        self.counts
            .values()
            .copied()
            .fold(0usize, |total, count| {
                total.saturating_add(count)
            })
    }

    /// Returns true if counts exactly represent all requested shots.
    pub fn counts_match_shots(&self) -> bool {
        self.counted_shots() == self.shots
    }

    /// Returns true if no count currently exceeds the requested shot budget.
    pub fn counts_within_shots(&self) -> bool {
        self.counted_shots() <= self.shots
    }

    /// Inserts/replaces a normalized bitstring count.
    ///
    /// Replacing an existing key correctly subtracts the previous count before
    /// applying the new count. This fixes the common accounting bug where
    /// replacing a result entry falsely appears to exceed the shot count.
    pub fn insert_count(
        &mut self,
        bitstring: impl Into<String>,
        count: usize,
    ) -> Result<(), BackendError> {
        let bitstring = bitstring.into();

        validate_bitstring(&bitstring)?;

        let previous = self.counts.get(&bitstring).copied().unwrap_or(0);
        let current = self.counted_shots();

        let without_previous = current
            .checked_sub(previous)
            .ok_or(BackendError::ResultCountOverflow)?;

        let new_total = without_previous
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

    /// Inserts an expectation value after validating its identifier.
    pub fn insert_expectation_value(
        &mut self,
        observable: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), BackendError> {
        let observable = observable.into();
        let value = value.into();

        validate_identifier(
            "observable",
            &observable,
            MAX_METADATA_KEY_LENGTH,
        )?;

        if value.trim().is_empty()
            || value.len() > MAX_METADATA_VALUE_LENGTH
            || value.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidMetadata {
                key: observable,
            });
        }

        self.expectation_values.insert(observable, value);
        Ok(())
    }

    /// Validates the normalized result.
    pub fn validate(&self) -> Result<(), BackendError> {
        validate_identifier(
            "backend_id",
            &self.backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if self.shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        for bitstring in self.counts.keys() {
            validate_bitstring(bitstring)?;
        }

        if !self.counts_within_shots() {
            return Err(BackendError::ResultShotsExceeded {
                represented: self.counted_shots(),
                shots: self.shots,
            });
        }

        for (key, value) in &self.expectation_values {
            validate_identifier(
                "observable",
                key,
                MAX_METADATA_KEY_LENGTH,
            )?;

            if value.trim().is_empty()
                || value.len() > MAX_METADATA_VALUE_LENGTH
                || value.chars().any(char::is_control)
            {
                return Err(BackendError::InvalidMetadata {
                    key: key.clone(),
                });
            }
        }

        for (key, value) in &self.metadata {
            validate_metadata_field(key, value)?;

            if looks_like_secret_key(key) {
                return Err(BackendError::SecretLikeMetadata {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Severity of a backend validation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationSeverity {
    /// Informational diagnostic.
    Info,

    /// Non-blocking warning.
    Warning,

    /// Blocking validation error.
    Error,
}

impl ValidationSeverity {
    /// Returns true for blocking errors.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Error)
    }

    const fn rank(self) -> u8 {
        match self {
            Self::Error => 0,
            Self::Warning => 1,
            Self::Info => 2,
        }
    }
}

/// Machine-readable backend validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    /// Stable diagnostic code.
    pub code: &'static str,

    /// Severity.
    pub severity: ValidationSeverity,

    /// Human-readable diagnostic.
    pub message: String,

    /// Machine-readable requirement description.
    pub requirement: String,

    /// Backend identifier.
    pub backend_id: String,

    /// Original provider-neutral error, when applicable.
    ///
    /// Kept private so consumers do not depend on the internal diagnostic
    /// construction format. `BackendValidationReport::first_error()` exposes
    /// it safely.
    source_error: Option<BackendError>,
}

impl ValidationDiagnostic {
    fn sort_key(&self) -> (
        u8,
        &'static str,
        &str,
        &str,
        &str,
    ) {
        (
            self.severity.rank(),
            self.code,
            self.requirement.as_str(),
            self.message.as_str(),
            self.backend_id.as_str(),
        )
    }
}

/// Complete deterministic backend validation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendValidationReport {
    /// Backend identifier.
    pub backend_id: String,

    /// True when no blocking diagnostic exists.
    pub valid: bool,

    /// Deterministically ordered diagnostics.
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

    fn push(
        &mut self,
        code: &'static str,
        severity: ValidationSeverity,
        message: String,
        requirement: String,
        source_error: Option<BackendError>,
    ) {
        if severity.is_blocking() {
            self.valid = false;
        }

        self.diagnostics.push(ValidationDiagnostic {
            code,
            severity,
            message,
            requirement,
            backend_id: self.backend_id.clone(),
            source_error,
        });
    }

    fn error(
        &mut self,
        code: &'static str,
        message: String,
        requirement: String,
        source_error: BackendError,
    ) {
        self.push(
            code,
            ValidationSeverity::Error,
            message,
            requirement,
            Some(source_error),
        );
    }

    fn warning(
        &mut self,
        code: &'static str,
        message: String,
        requirement: String,
    ) {
        self.push(
            code,
            ValidationSeverity::Warning,
            message,
            requirement,
            None,
        );
    }

    fn finalize(&mut self) {
        self.diagnostics
            .sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    }

    /// Returns true when blocking diagnostics exist.
    pub fn has_errors(&self) -> bool {
        !self.valid
    }

    /// Returns blocking diagnostics.
    pub fn errors(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity.is_blocking())
    }

    /// Returns warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics.iter().filter(|diagnostic| {
            diagnostic.severity == ValidationSeverity::Warning
        })
    }

    /// Returns the first deterministic underlying backend error.
    pub fn first_error(&self) -> Option<BackendError> {
        self.errors()
            .find_map(|diagnostic| diagnostic.source_error.clone())
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

    InconsistentWorkloadKind {
        declared: QuantumWorkloadKind,
        inferred: QuantumWorkloadKind,
    },

    UnsupportedCapability {
        capability: String,
    },

    ExperimentalCapabilityNotAccepted {
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

    InvalidWorkload(String),

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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

            Self::IdentifierTooLong { field, maximum } => {
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
                    "metadata field '{key}' appears to contain secret material",
                )
            }

            Self::InvalidShots => {
                formatter.write_str("shot count must be greater than zero")
            }

            Self::ZeroQubits => {
                formatter.write_str(
                    "gate-model workload must contain at least one qubit",
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
                write!(
                    formatter,
                    "backend does not support workload '{workload}'"
                )
            }

            Self::InconsistentWorkloadKind { declared, inferred } => {
                write!(
                    formatter,
                    "workload declares kind '{declared}' but its requirements infer '{inferred}'"
                )
            }

            Self::UnsupportedCapability { capability } => {
                write!(
                    formatter,
                    "backend does not support required capability '{capability}'"
                )
            }

            Self::ExperimentalCapabilityNotAccepted { capability } => {
                write!(
                    formatter,
                    "required capability '{capability}' is experimental and cannot satisfy a stable requirement"
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
                formatter.write_str(
                    "backend does not support dynamic circuits",
                )
            }

            Self::PulseControlUnsupported => {
                formatter.write_str("backend does not support pulse control")
            }

            Self::AnalogControlUnsupported => {
                formatter.write_str("backend does not support analog control")
            }

            Self::AnnealingUnsupported => {
                formatter.write_str(
                    "backend does not support annealing workloads",
                )
            }

            Self::LogicalQubitsUnsupported => {
                formatter.write_str(
                    "backend does not expose logical qubits",
                )
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
                formatter.write_str(
                    "backend does not support state-vector results",
                )
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

            Self::UnsupportedConnection { control, target } => write!(
                formatter,
                "backend topology does not support native connection {control} -> {target}"
            ),

            Self::TopologyUnavailable => {
                formatter.write_str(
                    "backend topology information is unavailable",
                )
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

            Self::InvalidWorkload(message) => {
                write!(formatter, "invalid workload: {message}")
            }

            Self::ExecutionUnavailable(message) => {
                write!(formatter, "execution unavailable: {message}")
            }

            Self::ExecutionRejected(message) => {
                write!(formatter, "execution rejected: {message}")
            }

            Self::ResultCountOverflow => {
                formatter.write_str(
                    "execution result shot count overflowed",
                )
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
// Quantum backend aggregate
// =============================================================================

/// Canonical provider-neutral quantum backend descriptor.
///
/// `QuantumBackend` is intentionally a descriptor/validation aggregate.
///
/// It is NOT itself a network executor.
///
/// A provider adapter receives a validated backend and execution request and
/// performs the actual provider operation.
#[derive(Debug, Clone)]
pub struct QuantumBackend {
    /// Stable backend metadata.
    pub metadata: BackendMetadata,

    /// Backend capabilities.
    pub capabilities: BackendCapabilities,

    /// Backend resource limits.
    pub limits: BackendLimits,

    /// Authoritative physical topology.
    pub topology: HardwareTopology,
}

impl QuantumBackend {
    /// Constructs and fully validates a backend descriptor.
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

    /// Stable provider identifier.
    pub fn provider(&self) -> &str {
        &self.metadata.provider
    }

    /// Backend kind.
    pub const fn kind(&self) -> BackendKind {
        self.metadata.kind
    }

    /// Backend status.
    pub const fn status(&self) -> BackendStatus {
        self.metadata.status
    }

    /// Returns true when status permits ordinary execution.
    pub const fn is_available(&self) -> bool {
        self.metadata.status.is_usable()
    }

    /// Changes only operational status.
    pub fn set_status(&mut self, status: BackendStatus) {
        self.metadata.status = status;
    }

    /// Physical resource count represented by the topology.
    pub const fn qubit_count(&self) -> usize {
        self.topology.qubit_count()
    }

    /// Number of physical couplings.
    pub const fn coupling_count(&self) -> usize {
        self.topology.coupling_count()
    }

    /// Returns native instructions in deterministic order.
    pub fn native_gates(&self) -> Vec<String> {
        self.capabilities
            .native_gates
            .iter()
            .cloned()
            .collect()
    }

    /// Returns the authoritative topology.
    pub fn topology(&self) -> &HardwareTopology {
        &self.topology
    }

    /// Returns a borrowed immutable descriptor.
    pub fn descriptor(&self) -> BackendDescriptor<'_> {
        BackendDescriptor { backend: self }
    }

    /// Returns all stable capability identifiers.
    pub fn capability_names(&self) -> Vec<String> {
        self.capabilities.stable_names()
    }

    /// Produces a complete deterministic validation report.
    pub fn validation_report(
        &self,
        requirements: &WorkloadRequirements,
    ) -> BackendValidationReport {
        let mut report = BackendValidationReport::new(self.id());

        if let Err(error) = requirements.validate() {
            report.error(
                "INVALID_WORKLOAD",
                error.to_string(),
                "workload_invariants=true".to_string(),
                error,
            );

            report.finalize();
            return report;
        }

        self.validate_status_report(&mut report);
        self.validate_workload_kind_report(requirements, &mut report);
        self.validate_resource_report(requirements, &mut report);
        self.validate_capability_report(requirements, &mut report);
        self.validate_instruction_report(requirements, &mut report);
        self.validate_topology_report(requirements, &mut report);

        report.finalize();
        report
    }

    /// Validates a workload.
    ///
    /// The returned error preserves the exact requested and maximum values.
    pub fn validate(
        &self,
        requirements: &WorkloadRequirements,
    ) -> Result<(), BackendError> {
        let report = self.validation_report(requirements);

        if report.valid {
            Ok(())
        } else {
            Err(report.first_error().unwrap_or_else(|| {
                BackendError::ExecutionRejected(
                    "backend validation failed without a structured error"
                        .to_string(),
                )
            }))
        }
    }

    /// Backwards-compatible gate-model validation entry point.
    pub fn validate_circuit(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        let workload =
            WorkloadRequirements::from_circuit(requirements.clone());

        self.validate(&workload)
    }

    /// Validates a complete execution request before provider submission.
    pub fn validate_request(
        &self,
        request: &ExecutionRequest,
    ) -> Result<(), BackendError> {
        request.validate_structure()?;
        self.validate(&request.workload)?;

        if request.seed.is_some()
            && !self.capabilities.deterministic_seeding
        {
            return Err(BackendError::DeterministicSeedingUnsupported);
        }

        Ok(())
    }

    /// Provider-independent preflight boundary.
    ///
    /// Provider adapters MUST call this before provider I/O.
    pub fn preflight(
        &self,
        request: &ExecutionRequest,
    ) -> Result<(), BackendError> {
        self.validate_request(request)
    }

    fn validate_internal_consistency(&self) -> Result<(), BackendError> {
        if self.capabilities.native_gates.len()
            > MAX_NATIVE_INSTRUCTIONS
        {
            return Err(BackendError::RequiredInstructionLimitExceeded {
                maximum: MAX_NATIVE_INSTRUCTIONS,
            });
        }

        if self.capabilities.native_instruction_set
            && self.capabilities.native_gates.is_empty()
        {
            return Err(BackendError::NativeInstructionSetUnavailable);
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
                "backend is operational but currently degraded"
                    .to_string(),
                "backend_status=degraded".to_string(),
            ),

            BackendStatus::Busy => report.error(
                "BACKEND_BUSY",
                "backend is currently busy".to_string(),
                "backend_status=available".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Busy,
                },
            ),

            BackendStatus::Unknown => report.error(
                "BACKEND_STATUS_UNKNOWN",
                "backend operational status is unknown".to_string(),
                "backend_status=known".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Unknown,
                },
            ),

            BackendStatus::Maintenance => report.error(
                "BACKEND_MAINTENANCE",
                "backend is under maintenance".to_string(),
                "backend_status=available".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Maintenance,
                },
            ),

            BackendStatus::Offline => report.error(
                "BACKEND_OFFLINE",
                "backend is offline".to_string(),
                "backend_status=available".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Offline,
                },
            ),

            BackendStatus::Retired => report.error(
                "BACKEND_RETIRED",
                "backend has been retired".to_string(),
                "backend_status=available".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Retired,
                },
            ),

            BackendStatus::Unavailable => report.error(
                "BACKEND_UNAVAILABLE",
                "backend is unavailable".to_string(),
                "backend_status=available".to_string(),
                BackendError::BackendUnavailable {
                    backend_id: self.id().to_string(),
                    status: BackendStatus::Unavailable,
                },
            ),
        }
    }

    fn validate_workload_kind_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let inferred = requirements.circuit.inferred_kind();

        if requirements.kind != QuantumWorkloadKind::Custom
            && requirements.kind != inferred
        {
            report.error(
                "WORKLOAD_KIND_MISMATCH",
                format!(
                    "declared workload kind '{}' does not match inferred kind '{}'",
                    requirements.kind, inferred
                ),
                format!("kind={}", requirements.kind),
                BackendError::InconsistentWorkloadKind {
                    declared: requirements.kind,
                    inferred,
                },
            );
        }
    }

    fn validate_resource_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;

        let requires_qubits = matches!(
            requirements.kind,
            QuantumWorkloadKind::GateCircuit
                | QuantumWorkloadKind::DynamicCircuit
                | QuantumWorkloadKind::PulseProgram
                | QuantumWorkloadKind::LogicalProgram
                | QuantumWorkloadKind::Sampling
                | QuantumWorkloadKind::Custom
        );

        if requires_qubits && circuit.qubit_count == 0 {
            report.error(
                "ZERO_QUBITS",
                "workload requires at least one quantum resource"
                    .to_string(),
                "qubit_count>0".to_string(),
                BackendError::ZeroQubits,
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
                format!(
                    "qubit_count<={}",
                    self.limits.max_qubits
                ),
                BackendError::QubitLimitExceeded {
                    requested: circuit.qubit_count,
                    maximum: self.limits.max_qubits,
                },
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
                BackendError::LogicalQubitLimitExceeded {
                    requested: circuit.logical_qubit_count,
                    maximum: self.limits.max_logical_qubits,
                },
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
                format!(
                    "circuit_depth<={}",
                    self.limits.max_circuit_depth
                ),
                BackendError::CircuitDepthExceeded {
                    requested: circuit.circuit_depth,
                    maximum: self.limits.max_circuit_depth,
                },
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
                BackendError::OperationLimitExceeded {
                    requested: circuit.operation_count,
                    maximum: self.limits.max_operations,
                },
            );
        }

        if circuit.shots == 0 {
            report.error(
                "INVALID_SHOTS",
                "shot count must be greater than zero".to_string(),
                "shots>0".to_string(),
                BackendError::InvalidShots,
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
                BackendError::ShotLimitExceeded {
                    requested: circuit.shots,
                    maximum: self.limits.max_shots,
                },
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
                BackendError::ClassicalBitLimitExceeded {
                    requested: circuit.classical_bit_count,
                    maximum: self.limits.max_classical_bits,
                },
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

        let workload_supported = match requirements.kind {
            QuantumWorkloadKind::GateCircuit
            | QuantumWorkloadKind::Sampling => true,

            QuantumWorkloadKind::DynamicCircuit => {
                capabilities.dynamic_circuits
            }

            QuantumWorkloadKind::PulseProgram => {
                capabilities.pulse_control
            }

            QuantumWorkloadKind::AnalogProgram => {
                capabilities.analog_control
            }

            QuantumWorkloadKind::AnnealingProblem => {
                capabilities.annealing
            }

            QuantumWorkloadKind::LogicalProgram => {
                capabilities.logical_qubits
            }

            QuantumWorkloadKind::Custom => true,
        };

        if !workload_supported {
            report.error(
                "UNSUPPORTED_WORKLOAD",
                format!(
                    "backend does not support workload '{}'",
                    requirements.kind
                ),
                format!("workload={}", requirements.kind),
                BackendError::UnsupportedWorkload {
                    workload: requirements.kind,
                },
            );
        }

        if circuit.requires_measurement && !capabilities.measurement {
            report.error(
                "MEASUREMENT_UNSUPPORTED",
                "measurement is required but unsupported".to_string(),
                "measurement=true".to_string(),
                BackendError::MeasurementUnsupported,
            );
        }

        if circuit.requires_reset && !capabilities.reset {
            report.error(
                "RESET_UNSUPPORTED",
                "reset is required but unsupported".to_string(),
                "reset=true".to_string(),
                BackendError::ResetUnsupported,
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
                BackendError::MidCircuitMeasurementUnsupported,
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
                BackendError::ClassicalControlUnsupported,
            );
        }

        if circuit.requires_dynamic_circuits
            && !capabilities.dynamic_circuits
        {
            report.error(
                "DYNAMIC_CIRCUIT_UNSUPPORTED",
                "dynamic circuits are required but unsupported".to_string(),
                "dynamic_circuits=true".to_string(),
                BackendError::DynamicCircuitUnsupported,
            );
        }

        if circuit.requires_pulse_control && !capabilities.pulse_control {
            report.error(
                "PULSE_CONTROL_UNSUPPORTED",
                "pulse-level control is required but unsupported".to_string(),
                "pulse_control=true".to_string(),
                BackendError::PulseControlUnsupported,
            );
        }

        if circuit.requires_analog_control && !capabilities.analog_control {
            report.error(
                "ANALOG_CONTROL_UNSUPPORTED",
                "analog control is required but unsupported".to_string(),
                "analog_control=true".to_string(),
                BackendError::AnalogControlUnsupported,
            );
        }

        if circuit.requires_annealing && !capabilities.annealing {
            report.error(
                "ANNEALING_UNSUPPORTED",
                "annealing is required but unsupported".to_string(),
                "annealing=true".to_string(),
                BackendError::AnnealingUnsupported,
            );
        }

        if circuit.requires_logical_qubits
            && !capabilities.logical_qubits
        {
            report.error(
                "LOGICAL_QUBITS_UNSUPPORTED",
                "logical qubits are required but unavailable".to_string(),
                "logical_qubits=true".to_string(),
                BackendError::LogicalQubitsUnsupported,
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
                BackendError::FaultToleranceUnsupported,
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
                BackendError::DeterministicSeedingUnsupported,
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
                BackendError::StateVectorUnsupported,
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
                BackendError::DensityMatrixUnsupported,
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
                BackendError::ExpectationValuesUnsupported,
            );
        }

        for capability in &requirements.required_capabilities {
            if capabilities.supports_capability(capability) {
                continue;
            }

            if capabilities.is_experimental(capability) {
                report.error(
                    "EXPERIMENTAL_CAPABILITY",
                    format!(
                        "required capability '{}' is experimental and cannot satisfy a stable requirement",
                        capability
                    ),
                    format!("stable_capability={capability}"),
                    BackendError::ExperimentalCapabilityNotAccepted {
                        capability: capability.clone(),
                    },
                );
            } else {
                report.error(
                    "REQUIRED_CAPABILITY_UNSUPPORTED",
                    format!(
                        "required capability '{}' is unsupported",
                        capability
                    ),
                    format!("capability={capability}"),
                    BackendError::UnsupportedCapability {
                        capability: capability.clone(),
                    },
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
                BackendError::TopologyUnavailable,
            );
        }

        if requirements.requires_calibration
            && !capabilities.calibration_data
        {
            report.error(
                "CALIBRATION_UNAVAILABLE",
                "workload requires calibration information".to_string(),
                "calibration_data=true".to_string(),
                BackendError::CalibrationUnavailable,
            );
        }

        if requirements.requires_fresh_calibration {
            if !capabilities.calibration_data {
                report.error(
                    "FRESH_CALIBRATION_UNAVAILABLE",
                    "fresh calibration is required but calibration data is unavailable"
                        .to_string(),
                    "calibration_data=true".to_string(),
                    BackendError::CalibrationUnavailable,
                );
            } else {
                // The backend layer cannot determine freshness without the
                // calibration subsystem. It deliberately emits a warning
                // rather than pretending freshness has been verified.
                report.warning(
                    "CALIBRATION_FRESHNESS_DEFERRED",
                    "calibration freshness must be verified against the selected calibration snapshot before provider submission"
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
                BackendError::RequiredInstructionLimitExceeded {
                    maximum: MAX_REQUIRED_INSTRUCTIONS,
                },
            );

            return;
        }

        if !requirements.required_instructions.is_empty()
            && !self.capabilities.native_instruction_set
        {
            report.error(
                "NATIVE_INSTRUCTION_SET_UNAVAILABLE",
                "workload requires native instruction matching but backend does not expose its native instruction set"
                    .to_string(),
                "native_instruction_set=true".to_string(),
                BackendError::NativeInstructionSetUnavailable,
            );
        }

        for instruction in &requirements.required_instructions {
            if !self.capabilities.native_instruction_set {
                continue;
            }

            if !self.capabilities.supports_gate(instruction) {
                report.error(
                    "REQUIRED_INSTRUCTION_UNSUPPORTED",
                    format!(
                        "backend does not expose required native instruction '{}'",
                        instruction
                    ),
                    format!(
                        "native_instruction={instruction}"
                    ),
                    BackendError::UnsupportedGate {
                        gate: instruction.clone(),
                    },
                );
            }
        }

        for gate in &requirements.circuit.gates {
            let normalized = normalize_instruction_name(gate);

            if normalized.is_empty() {
                report.error(
                    "INVALID_INSTRUCTION",
                    "workload contains an empty instruction identifier"
                        .to_string(),
                    "instruction_name!=empty".to_string(),
                    BackendError::InvalidIdentifier {
                        field: "instruction",
                    },
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
                format!(
                    "native_instruction={normalized}"
                ),
                BackendError::UnsupportedGate {
                    gate: normalized,
                },
            );
        }
    }

    fn validate_topology_report(
        &self,
        requirements: &WorkloadRequirements,
        report: &mut BackendValidationReport,
    ) {
        let circuit = &requirements.circuit;

        if !self.capabilities.topology_information {
            if !circuit.two_qubit_edges.is_empty()
                || requirements.requires_topology
            {
                report.error(
                    "TOPOLOGY_REQUIRED",
                    "workload requires topology information but backend does not expose it"
                        .to_string(),
                    "topology_information=true".to_string(),
                    BackendError::TopologyUnavailable,
                );
            }

            return;
        }

        if circuit.qubit_count == 0 {
            return;
        }

        if self.topology.qubit_count() == 0 {
            report.error(
                "EMPTY_TOPOLOGY",
                "backend topology contains zero resources".to_string(),
                "topology.qubit_count>0".to_string(),
                BackendError::TopologyUnavailable,
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
                BackendError::QubitLimitExceeded {
                    requested: circuit.qubit_count,
                    maximum: self.topology.qubit_count(),
                },
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
                BackendError::RequiredTopologyEdgeLimitExceeded {
                    maximum: MAX_REQUIRED_TOPOLOGY_EDGES,
                },
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
                    BackendError::InvalidQubit {
                        qubit: control,
                        qubit_count: circuit.qubit_count,
                    },
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
                    BackendError::InvalidQubit {
                        qubit: target,
                        qubit_count: circuit.qubit_count,
                    },
                );

                continue;
            }

            if control == target {
                report.error(
                    "SELF_INTERACTION",
                    format!(
                        "two-qubit operation cannot target the same resource {} twice",
                        control
                    ),
                    "control!=target".to_string(),
                    BackendError::UnsupportedConnection {
                        control,
                        target,
                    },
                );

                continue;
            }

            match self.topology.is_connected(control, target) {
                Ok(true) => {}

                Ok(false) => report.error(
                    "UNSUPPORTED_CONNECTION",
                    format!(
                        "backend topology does not support native connection {} -> {}",
                        control, target
                    ),
                    format!(
                        "native_connection={control}->{target}"
                    ),
                    BackendError::UnsupportedConnection {
                        control,
                        target,
                    },
                ),

                Err(error) => report.error(
                    "TOPOLOGY_QUERY_FAILED",
                    error.to_string(),
                    format!(
                        "native_connection={control}->{target}"
                    ),
                    BackendError::Topology(error),
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
/// This allows registries and discovery systems to inspect a backend without
/// cloning its complete topology/capability structures.
#[derive(Debug, Clone, Copy)]
pub struct BackendDescriptor<'a> {
    backend: &'a QuantumBackend,
}

impl<'a> BackendDescriptor<'a> {
    /// Backend ID.
    pub fn id(self) -> &'a str {
        self.backend.id()
    }

    /// Provider ID.
    pub fn provider(self) -> &'a str {
        self.backend.provider()
    }

    /// Backend kind.
    pub const fn kind(self) -> BackendKind {
        self.backend.kind()
    }

    /// Backend status.
    pub const fn status(self) -> BackendStatus {
        self.backend.status()
    }

    /// Physical resource count.
    pub const fn qubit_count(self) -> usize {
        self.backend.qubit_count()
    }

    /// Coupling count.
    pub const fn coupling_count(self) -> usize {
        self.backend.coupling_count()
    }

    /// Backend capabilities.
    pub fn capabilities(self) -> &'a BackendCapabilities {
        &self.backend.capabilities
    }

    /// Backend limits.
    pub const fn limits(self) -> BackendLimits {
        self.backend.limits
    }

    /// Backend topology.
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

    if let Some(value) = &metadata.hardware_revision {
        validate_identifier(
            "hardware_revision",
            value,
            MAX_HARDWARE_REVISION_LENGTH,
        )?;
    }

    if let Some(value) = &metadata.firmware_version {
        validate_identifier(
            "firmware_version",
            value,
            MAX_FIRMWARE_VERSION_LENGTH,
        )?;
    }

    if let Some(value) = &metadata.api_version {
        validate_identifier(
            "api_version",
            value,
            MAX_API_VERSION_LENGTH,
        )?;
    }

    if let Some(value) = &metadata.region {
        validate_identifier(
            "region",
            value,
            MAX_REGION_LENGTH,
        )?;
    }

    if metadata.properties.len() > MAX_METADATA_PROPERTIES {
        return Err(BackendError::MetadataLimitExceeded {
            maximum: MAX_METADATA_PROPERTIES,
        });
    }

    for (key, value) in &metadata.properties {
        validate_metadata_field(key, value)?;

        if looks_like_secret_key(key) {
            return Err(BackendError::SecretLikeMetadata {
                key: key.clone(),
            });
        }
    }

    Ok(())
}

fn validate_capabilities(
    capabilities: &BackendCapabilities,
) -> Result<(), BackendError> {
    if capabilities.native_gates.len()
        > MAX_NATIVE_INSTRUCTIONS
    {
        return Err(BackendError::RequiredInstructionLimitExceeded {
            maximum: MAX_NATIVE_INSTRUCTIONS,
        });
    }

    for gate in &capabilities.native_gates {
        if gate.trim().is_empty()
            || gate.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidIdentifier {
                field: "native_instruction",
            });
        }
    }

    for capability in &capabilities.experimental_capabilities {
        if capability.trim().is_empty()
            || capability.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidIdentifier {
                field: "experimental_capability",
            });
        }
    }

    if capabilities.native_instruction_set
        && capabilities.native_gates.is_empty()
    {
        return Err(BackendError::NativeInstructionSetUnavailable);
    }

    Ok(())
}

fn validate_topology(
    topology: &HardwareTopology,
) -> Result<(), BackendError> {
    if topology.qubit_count() == 0 {
        return Err(BackendError::TopologyUnavailable);
    }

    topology.validate().map_err(BackendError::Topology)?;

    Ok(())
}

fn validate_workload_requirements(
    requirements: &WorkloadRequirements,
) -> Result<(), BackendError> {
    if requirements.required_capabilities.len()
        > MAX_REQUIRED_INSTRUCTIONS
    {
        return Err(BackendError::RequiredInstructionLimitExceeded {
            maximum: MAX_REQUIRED_INSTRUCTIONS,
        });
    }

    if requirements.required_instructions.len()
        > MAX_REQUIRED_INSTRUCTIONS
    {
        return Err(BackendError::RequiredInstructionLimitExceeded {
            maximum: MAX_REQUIRED_INSTRUCTIONS,
        });
    }

    if requirements.circuit.two_qubit_edges.len()
        > MAX_REQUIRED_TOPOLOGY_EDGES
    {
        return Err(
            BackendError::RequiredTopologyEdgeLimitExceeded {
                maximum: MAX_REQUIRED_TOPOLOGY_EDGES,
            },
        );
    }

    if requirements.circuit.shots == 0 {
        return Err(BackendError::InvalidShots);
    }

    for capability in &requirements.required_capabilities {
        if capability.trim().is_empty()
            || capability.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidIdentifier {
                field: "required_capability",
            });
        }
    }

    for instruction in &requirements.required_instructions {
        if instruction.trim().is_empty()
            || instruction.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidIdentifier {
                field: "required_instruction",
            });
        }
    }

    for gate in &requirements.circuit.gates {
        if gate.trim().is_empty()
            || gate.chars().any(char::is_control)
        {
            return Err(BackendError::InvalidIdentifier {
                field: "instruction",
            });
        }
    }

    for &(control, target) in &requirements.circuit.two_qubit_edges {
        if control == target {
            return Err(BackendError::UnsupportedConnection {
                control,
                target,
            });
        }
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        if field == "backend_id" {
            return Err(BackendError::InvalidBackendId);
        }

        return Err(BackendError::InvalidIdentifier { field });
    }

    if trimmed.len() > maximum {
        return Err(BackendError::IdentifierTooLong {
            field,
            maximum,
        });
    }

    if trimmed.chars().any(char::is_control) {
        return Err(BackendError::InvalidIdentifier { field });
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

/// Conservative secret-key detector.
///
/// This is deliberately a defence-in-depth mechanism and must not be treated
/// as a substitute for a real credential manager.
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
        "privatekey",
        "client_secret",
        "session_cookie",
        "cookie",
        "secret",
    ];

    SECRET_MARKERS
        .iter()
        .any(|marker| key.contains(marker))
}

fn normalize_instruction_name(name: &str) -> String {
    name.trim().to_ascii_uppercase()
}

fn normalize_capability_name(name: &str) -> String {
    name.trim()
        .to_ascii_lowercase()
        .replace('-', "_")
}

fn validate_bitstring(
    bitstring: &str,
) -> Result<(), BackendError> {
    if bitstring.is_empty()
        || !bitstring
            .bytes()
            .all(|byte| byte == b'0' || byte == b'1')
    {
        return Err(BackendError::InvalidBitstring {
            bitstring: bitstring.to_string(),
        });
    }

    Ok(())
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

        let capabilities = BackendCapabilities::new().with_gates([
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
    fn backend_rejects_excessive_qubits_with_exact_values() {
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
            Err(BackendError::QubitLimitExceeded {
                requested: 3,
                maximum: 2
            })
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

        assert!(matches!(
            backend.validate_circuit(&requirements),
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
        let capabilities = BackendCapabilities {
            arbitrary_single_qubit_rotations: true,
            ..BackendCapabilities::default()
        };

        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "rotation-backend",
                "Rotation Backend",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            ),
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
        let capabilities = BackendCapabilities {
            dynamic_circuits: true,
            mid_circuit_measurement: true,
            classical_control: true,
            ..BackendCapabilities::default()
        };

        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "dynamic",
                "Dynamic",
                "Zamani",
                "1.0",
                BackendKind::Qpu,
            ),
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
            Err(BackendError::InconsistentWorkloadKind { .. })
                | Err(BackendError::UnsupportedWorkload { .. })
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
            Err(BackendError::UnsupportedWorkload { .. })
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
            Err(BackendError::UnsupportedWorkload { .. })
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
            ExecutionResult::empty("local", 10)
                .expect("valid result");

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
    fn replacing_result_count_does_not_double_count() {
        let mut result =
            ExecutionResult::empty("local", 10)
                .expect("valid result");

        result
            .insert_count("000", 7)
            .expect("initial count valid");

        result
            .insert_count("000", 3)
            .expect("replacement valid");

        assert_eq!(result.counted_shots(), 3);
        assert!(!result.counts_match_shots());
    }

    #[test]
    fn result_counts_cannot_exceed_shots() {
        let mut result =
            ExecutionResult::empty("local", 10)
                .expect("valid result");

        assert!(matches!(
            result.insert_count("000", 11),
            Err(BackendError::ResultShotsExceeded {
                represented: 11,
                shots: 10
            })
        ));
    }

    #[test]
    fn request_validation_uses_backend_contract() {
        let backend = backend();

        let request = ExecutionRequest::new(
            CircuitRequirements {
                qubit_count: 2,
                gates: vec!["H", "CX"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                two_qubit_edges: vec![(0, 1)],
                ..Default::default()
            },
        );

        assert!(backend.validate_request(&request).is_ok());
    }

    #[test]
    fn deterministic_seed_requires_capability() {
        let backend = backend();

        let request = ExecutionRequest::new(
            CircuitRequirements {
                qubit_count: 1,
                ..Default::default()
            },
        )
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

        let request = ExecutionRequest::new(
            CircuitRequirements {
                qubit_count: 1,
                ..Default::default()
            },
        )
        .with_seed(42);

        assert!(backend.validate_request(&request).is_ok());
    }

    #[test]
    fn validation_report_preserves_exact_error() {
        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "limited",
                "Limited",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            ),
            BackendCapabilities::default(),
            BackendLimits::unlimited().with_max_qubits(2),
            topology(),
        )
        .expect("backend should be valid");

        let requirements = CircuitRequirements {
            qubit_count: 9,
            ..Default::default()
        };

        let report = backend.validation_report(
            &WorkloadRequirements::from_circuit(requirements),
        );

        assert!(matches!(
            report.first_error(),
            Some(BackendError::QubitLimitExceeded {
                requested: 9,
                maximum: 2
            })
        ));
    }

    #[test]
    fn backend_descriptor_is_non_owning() {
        let backend = backend();
        let descriptor = backend.descriptor();

        assert_eq!(descriptor.id(), "test-simulator");
        assert_eq!(descriptor.provider(), "Zamani");
        assert_eq!(
            descriptor.kind(),
            BackendKind::Simulator
        );
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
        assert!(
            report
                .warnings()
                .any(|warning| {
                    warning.code == "BACKEND_DEGRADED"
                })
        );
    }

    #[test]
    fn experimental_capability_does_not_satisfy_stable_requirement() {
        let capabilities = BackendCapabilities::default()
            .with_experimental_capability("future_feature");

        let backend = QuantumBackend::new(
            BackendMetadata::new(
                "experimental",
                "Experimental",
                "Zamani",
                "1.0",
                BackendKind::Qpu,
            ),
            capabilities,
            BackendLimits::unlimited(),
            topology(),
        )
        .expect("backend should be valid");

        let requirements = WorkloadRequirements::default()
            .require_capability("future_feature");

        assert!(matches!(
            backend.validate(&requirements),
            Err(BackendError::ExperimentalCapabilityNotAccepted {
                ..
            })
        ));
    }

    #[test]
    fn topology_api_remains_encapsulated() {
        let topology =
            HardwareTopology::linear(3)
                .expect("valid topology");

        assert_eq!(topology.qubit_count(), 3);
        assert_eq!(topology.coupling_count(), 2);

        assert!(
            topology
                .is_connected(0, 1)
                .expect("valid resources")
        );

        assert!(
            topology
                .is_connected(1, 0)
                .expect("valid resources")
        );
    }
}