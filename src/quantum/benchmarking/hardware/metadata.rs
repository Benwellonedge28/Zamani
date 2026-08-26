//! Zamani Quantum Benchmarking — Hardware Metadata
//!
//! Production-grade, backend-independent metadata captured for a quantum
//! benchmarking experiment.
//!
//! # Responsibility
//!
//! This module owns the benchmarking representation of the execution target.
//! It does NOT own:
//!
//! - backend execution;
//! - network communication;
//! - credentials or authentication;
//! - calibration acquisition;
//! - topology construction;
//! - circuit compilation;
//! - routing;
//! - scheduling;
//! - benchmark protocol mathematics;
//! - statistical analysis;
//! - result reporting.
//!
//! Those responsibilities remain with their owning subsystems.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::hardware::backend
//!              │
//!              ├──────────────┐
//!              ▼              ▼
//! quantum::hardware::      quantum::hardware::
//! calibration              topology
//!              │              │
//!              └──────┬───────┘
//!                     ▼
//! benchmarking::hardware::metadata
//!                     │
//!                     ▼
//!              benchmark provenance
//!                     │
//!                     ▼
//!               BenchmarkResult
//! ```
//!
//! # Why this module exists
//!
//! `quantum::hardware::backend::BackendMetadata` is the backend subsystem's
//! backend descriptor. Benchmarking needs a stronger concept:
//!
//! a reproducible snapshot of the execution environment used for a benchmark.
//!
//! A benchmark result must therefore be able to answer:
//!
//! - What backend was used?
//! - Which provider supplied it?
//! - What backend version was reported?
//! - Was it a simulator, emulator, QPU, or custom backend?
//! - Was it available when the experiment was captured?
//! - How many physical qubits were exposed?
//! - What capability profile was active?
//! - What backend limits were active?
//! - What topology was active?
//! - Which calibration snapshot was active?
//! - When was the metadata captured?
//! - Can the metadata be deterministically fingerprinted?
//!
//! # Security boundary
//!
//! This module deliberately does not store:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private credentials;
//! - authorization headers;
//! - raw provider secrets.
//!
//! Provider-specific properties are allowed, but callers are responsible for
//! ensuring that secrets are never inserted into `BackendMetadata.properties`.
//!
//! # Reproducibility
//!
//! All maps are represented by `BTreeMap` and capability gate sets are sorted
//! before fingerprinting. This makes the serialized logical representation
//! deterministic.
//!
//! The fingerprint implemented here is a deterministic non-cryptographic
//! fingerprint. It MUST NOT be treated as a cryptographic integrity hash.
//!
//! A future cryptographic provenance layer may wrap this metadata in a
//! cryptographic digest without requiring this module to change its semantic
//! model.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is intentionally self-contained with respect to the benchmarking
//! tree. It depends only on the already-existing quantum hardware abstractions:
//!
//! - `quantum::hardware::backend`
//! - `quantum::hardware::calibration`
//! - `quantum::hardware::topology`
//!
//! Future modules consume this file rather than modifying it:
//!
//! ```text
//! execution/
//!      │
//!      ▼
//! hardware::metadata
//!      │
//!      ▼
//! core::provenance
//!      │
//!      ▼
//! core::result
//! ```
//!
//! In particular:
//!
//! - `hardware/capabilities.rs` can consume capability information here;
//! - `hardware/timing.rs` can attach timing metadata to benchmark results;
//! - `core/provenance.rs` can record the metadata fingerprint;
//! - `analysis/baseline.rs` can compare fingerprints;
//! - `reporting/json.rs` can serialize an equivalent representation;
//! - benchmark protocols do not need to know how metadata is collected.
//!
//! This file therefore should not need to be re-edited merely because those
//! later modules are implemented.

use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    QuantumBackend,
};

use crate::quantum::hardware::calibration::CalibrationSnapshot;

use crate::quantum::hardware::topology::{
    Connectivity,
    Coupling,
    HardwareTopology,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable schema identifier for this benchmarking metadata contract.
pub const HARDWARE_METADATA_SCHEMA_ID: &str =
    "zamani.quantum.benchmarking.hardware.metadata";

/// Version of the hardware metadata schema.
///
/// Increment this when the semantic structure of `HardwareMetadata` changes
/// incompatibly.
pub const HARDWARE_METADATA_SCHEMA_VERSION: u32 = 1;

/// Maximum provider/backend identifier length accepted by this module.
pub const MAX_IDENTIFIER_LENGTH: usize = 512;

/// Maximum name/provider/version length accepted by this module.
pub const MAX_TEXT_LENGTH: usize = 1024;

/// Maximum number of provider properties retained.
pub const MAX_PROPERTIES: usize = 256;

/// Maximum provider property key length.
pub const MAX_PROPERTY_KEY_LENGTH: usize = 256;

/// Maximum provider property value length.
pub const MAX_PROPERTY_VALUE_LENGTH: usize = 4096;

/// Maximum number of native gates retained.
pub const MAX_NATIVE_GATES: usize = 1024;

/// Maximum number of topology couplings represented in metadata.
pub const MAX_TOPOLOGY_COUPLINGS: usize = 1_000_000;

/// Maximum number of calibration qubits represented in metadata.
pub const MAX_CALIBRATION_QUBITS: usize = 1_000_000;

/// Maximum number of calibration gates represented in metadata.
pub const MAX_CALIBRATION_GATES: usize = 1_000_000;

// ============================================================================
// Capture timestamp
// ============================================================================

/// Wall-clock timestamp at which benchmark hardware metadata was captured.
///
/// The value is stored as Unix nanoseconds so that it can be:
///
/// - copied into provenance;
/// - compared;
/// - serialized by future reporting layers;
/// - hashed deterministically.
///
/// This timestamp is descriptive metadata. It is not used for measuring
/// execution latency. Benchmark timing belongs to the execution/timing layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataTimestamp {
    unix_ns: u128,
}

impl MetadataTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(unix_ns: u128) -> Self {
        Self { unix_ns }
    }

    /// Returns the timestamp as Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u128 {
        self.unix_ns
    }

    /// Captures the current system clock.
    ///
    /// A clock failure produces zero rather than panicking. The resulting
    /// metadata remains structurally valid but callers can detect the zero
    /// timestamp through `is_epoch()`.
    pub fn now() -> Self {
        let unix_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        Self { unix_ns }
    }

    /// Returns whether the timestamp is exactly Unix epoch.
    pub const fn is_epoch(self) -> bool {
        self.unix_ns == 0
    }
}

// ============================================================================
// Hardware technology
// ============================================================================

/// Technology class of the benchmark execution target.
///
/// This deliberately extends the existing `BackendKind` rather than replacing
/// it. `BackendKind` describes the operational backend category while this
/// enum describes the underlying quantum technology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareTechnology {
    /// Technology is not disclosed or not known.
    Unknown,

    /// Classical software simulator.
    ClassicalSimulator,

    /// Software emulator of a physical quantum architecture.
    Emulator,

    /// Superconducting qubits.
    Superconducting,

    /// Trapped-ion quantum computers.
    TrappedIon,

    /// Neutral-atom systems.
    NeutralAtom,

    /// Photonic quantum systems.
    Photonic,

    /// Semiconductor/spin qubits.
    Spin,

    /// Quantum dots or related semiconductor architectures.
    QuantumDot,

    /// Topological quantum-computing systems.
    Topological,

    /// Quantum annealing systems.
    Annealing,

    /// Analog quantum systems.
    Analog,

    /// Other quantum technology.
    Other,
}

impl HardwareTechnology {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::ClassicalSimulator => "classical_simulator",
            Self::Emulator => "emulator",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::Spin => "spin",
            Self::QuantumDot => "quantum_dot",
            Self::Topological => "topological",
            Self::Annealing => "annealing",
            Self::Analog => "analog",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for HardwareTechnology {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Metadata errors
// ============================================================================

/// Errors produced while constructing or validating benchmark hardware
/// metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareMetadataError {
    /// A required textual field was empty.
    EmptyField {
        field: &'static str,
    },

    /// A textual field exceeded its configured maximum.
    FieldTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// A property key was invalid.
    InvalidPropertyKey {
        key: String,
    },

    /// A property value was too large.
    PropertyValueTooLong {
        key: String,
        length: usize,
        maximum: usize,
    },

    /// Too many properties were supplied.
    TooManyProperties {
        count: usize,
        maximum: usize,
    },

    /// Too many native gates were supplied.
    TooManyNativeGates {
        count: usize,
        maximum: usize,
    },

    /// The topology contains more couplings than the metadata safety limit.
    TopologyTooLarge {
        count: usize,
        maximum: usize,
    },

    /// Calibration data contains more qubits than allowed.
    CalibrationTooLarge {
        qubits: usize,
        maximum: usize,
    },

    /// Calibration data contains more gate entries than allowed.
    CalibrationGatesTooLarge {
        gates: usize,
        maximum: usize,
    },

    /// Backend and calibration identifiers do not refer to the same target.
    BackendCalibrationMismatch {
        backend_id: String,
        calibration_backend_id: String,
    },

    /// Backend and topology dimensions disagree.
    BackendTopologyMismatch {
        backend_qubits: usize,
        topology_qubits: usize,
    },

    /// Calibration references a qubit not present in the topology.
    CalibrationQubitOutsideTopology {
        qubit: usize,
        topology_qubits: usize,
    },

    /// A topology coupling references an invalid qubit.
    TopologyQubitOutOfRange {
        qubit: usize,
        topology_qubits: usize,
    },

    /// A topology coupling is malformed.
    InvalidCoupling {
        source: usize,
        target: usize,
    },

    /// The backend limit is smaller than the physical topology.
    BackendLimitBelowTopology {
        maximum: usize,
        topology_qubits: usize,
    },

    /// A zero-shot backend limit is allowed by the backend subsystem as
    /// "unspecified", but this error is retained for future strict validation
    /// policies.
    InvalidBackendLimit {
        field: &'static str,
        value: usize,
    },

    /// Metadata was created from a backend with an invalid identity.
    InvalidBackendIdentity,

    /// A fingerprint could not be produced because the canonical
    /// representation was invalid.
    FingerprintFailure,

    /// General consistency error.
    InconsistentMetadata {
        message: String,
    },
}

impl fmt::Display for HardwareMetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "hardware metadata field `{field}` cannot be empty")
            }

            Self::FieldTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "hardware metadata field `{field}` has length {}, maximum is {}",
                    length, maximum
                )
            }

            Self::InvalidPropertyKey { key } => {
                write!(
                    formatter,
                    "invalid hardware metadata property key `{key}`"
                )
            }

            Self::PropertyValueTooLong {
                key,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "hardware metadata property `{key}` value has length {}, maximum is {}",
                    length, maximum
                )
            }

            Self::TooManyProperties { count, maximum } => {
                write!(
                    formatter,
                    "hardware metadata contains {} properties, maximum is {}",
                    count, maximum
                )
            }

            Self::TooManyNativeGates { count, maximum } => {
                write!(
                    formatter,
                    "hardware metadata contains {} native gates, maximum is {}",
                    count, maximum
                )
            }

            Self::TopologyTooLarge { count, maximum } => {
                write!(
                    formatter,
                    "hardware topology contains {} couplings, maximum metadata limit is {}",
                    count, maximum
                )
            }

            Self::CalibrationTooLarge { qubits, maximum } => {
                write!(
                    formatter,
                    "calibration snapshot contains {} qubits, maximum metadata limit is {}",
                    qubits, maximum
                )
            }

            Self::CalibrationGatesTooLarge { gates, maximum } => {
                write!(
                    formatter,
                    "calibration snapshot contains {} gates, maximum metadata limit is {}",
                    gates, maximum
                )
            }

            Self::BackendCalibrationMismatch {
                backend_id,
                calibration_backend_id,
            } => {
                write!(
                    formatter,
                    "backend ID `{backend_id}` does not match calibration backend ID `{calibration_backend_id}`"
                )
            }

            Self::BackendTopologyMismatch {
                backend_qubits,
                topology_qubits,
            } => {
                write!(
                    formatter,
                    "backend reports {} qubits but topology contains {}",
                    backend_qubits, topology_qubits
                )
            }

            Self::CalibrationQubitOutsideTopology {
                qubit,
                topology_qubits,
            } => {
                write!(
                    formatter,
                    "calibration references qubit {} outside topology containing {} qubits",
                    qubit, topology_qubits
                )
            }

            Self::TopologyQubitOutOfRange {
                qubit,
                topology_qubits,
            } => {
                write!(
                    formatter,
                    "topology references qubit {} outside range 0..{}",
                    qubit,
                    topology_qubits.saturating_sub(1)
                )
            }

            Self::InvalidCoupling { source, target } => {
                write!(
                    formatter,
                    "invalid topology coupling {} -> {}",
                    source, target
                )
            }

            Self::BackendLimitBelowTopology {
                maximum,
                topology_qubits,
            } => {
                write!(
                    formatter,
                    "backend maximum qubit limit {} is below topology size {}",
                    maximum, topology_qubits
                )
            }

            Self::InvalidBackendLimit { field, value } => {
                write!(
                    formatter,
                    "invalid backend limit `{field}` with value {}",
                    value
                )
            }

            Self::InvalidBackendIdentity => {
                write!(formatter, "backend identity is invalid")
            }

            Self::FingerprintFailure => {
                write!(formatter, "unable to construct metadata fingerprint")
            }

            Self::InconsistentMetadata { message } => {
                write!(formatter, "inconsistent hardware metadata: {message}")
            }
        }
    }
}

impl std::error::Error for HardwareMetadataError {}

// ============================================================================
// Topology metadata
// ============================================================================

/// Immutable benchmark representation of a hardware topology.
///
/// This is intentionally a summary plus canonical coupling information. It
/// does not replace `HardwareTopology`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyMetadata {
    qubit_count: usize,
    coupling_count: usize,
    couplings: Vec<TopologyCouplingMetadata>,
    fully_connected: bool,
    maximum_degree: usize,
}

impl TopologyMetadata {
    /// Capture topology metadata.
    pub fn from_topology(
        topology: &HardwareTopology,
    ) -> Result<Self, HardwareMetadataError> {
        let qubit_count = topology.qubit_count();
        let coupling_count = topology.coupling_count();

        if qubit_count == 0 {
            return Err(HardwareMetadataError::InconsistentMetadata {
                message: "topology contains zero qubits".to_string(),
            });
        }

        if coupling_count > MAX_TOPOLOGY_COUPLINGS {
            return Err(HardwareMetadataError::TopologyTooLarge {
                count: coupling_count,
                maximum: MAX_TOPOLOGY_COUPLINGS,
            });
        }

        let mut couplings = Vec::with_capacity(coupling_count);

        for coupling in topology.couplings() {
            validate_coupling(coupling, qubit_count)?;

            couplings.push(TopologyCouplingMetadata::from_coupling(
                *coupling,
            ));
        }

        couplings.sort();

        Ok(Self {
            qubit_count,
            coupling_count,
            couplings,
            fully_connected: topology.is_fully_connected(),
            maximum_degree: topology.maximum_degree(),
        })
    }

    /// Number of physical qubits.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Number of coupling edges.
    pub const fn coupling_count(&self) -> usize {
        self.coupling_count
    }

    /// Returns the canonical coupling list.
    pub fn couplings(&self) -> &[TopologyCouplingMetadata] {
        &self.couplings
    }

    /// Whether every pair of qubits is mutually reachable.
    pub const fn is_fully_connected(&self) -> bool {
        self.fully_connected
    }

    /// Maximum graph degree.
    pub const fn maximum_degree(&self) -> usize {
        self.maximum_degree
    }
}

/// Canonical coupling representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyCouplingMetadata {
    /// Source physical qubit.
    pub source: usize,

    /// Target physical qubit.
    pub target: usize,

    /// Whether the native coupling is bidirectional.
    pub bidirectional: bool,
}

impl TopologyCouplingMetadata {
    fn from_coupling(coupling: Coupling) -> Self {
        Self {
            source: coupling.source,
            target: coupling.target,
            bidirectional: matches!(
                coupling.connectivity,
                Connectivity::Bidirectional
            ),
        }
    }
}

// ============================================================================
// Capability metadata
// ============================================================================

/// Benchmark-owned immutable representation of backend capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityMetadata {
    pub measurement: bool,
    pub reset: bool,
    pub mid_circuit_measurement: bool,
    pub classical_control: bool,
    pub arbitrary_single_qubit_rotations: bool,
    pub parameterized_gates: bool,
    pub dynamic_circuits: bool,
    pub native_gates: Vec<String>,
}

impl CapabilityMetadata {
    /// Captures capabilities from the canonical hardware backend abstraction.
    pub fn from_capabilities(
        capabilities: &BackendCapabilities,
    ) -> Result<Self, HardwareMetadataError> {
        if capabilities.native_gates.len() > MAX_NATIVE_GATES {
            return Err(HardwareMetadataError::TooManyNativeGates {
                count: capabilities.native_gates.len(),
                maximum: MAX_NATIVE_GATES,
            });
        }

        let mut native_gates = capabilities
            .native_gates
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        native_gates.sort();

        for gate in &native_gates {
            validate_text("native_gate", gate, MAX_TEXT_LENGTH)?;
        }

        Ok(Self {
            measurement: capabilities.measurement,
            reset: capabilities.reset,
            mid_circuit_measurement: capabilities.mid_circuit_measurement,
            classical_control: capabilities.classical_control,
            arbitrary_single_qubit_rotations: capabilities
                .arbitrary_single_qubit_rotations,
            parameterized_gates: capabilities.parameterized_gates,
            dynamic_circuits: capabilities.dynamic_circuits,
            native_gates,
        })
    }

    /// Returns whether a capability is present.
    pub fn supports_gate(&self, gate: &str) -> bool {
        self.native_gates
            .binary_search(&normalize_gate_name(gate))
            .is_ok()
    }
}

// ============================================================================
// Limit metadata
// ============================================================================

/// Benchmark-owned representation of backend resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimitMetadata {
    pub max_qubits: usize,
    pub max_circuit_depth: usize,
    pub max_operations: usize,
    pub max_shots: usize,
}

impl From<BackendLimits> for LimitMetadata {
    fn from(limits: BackendLimits) -> Self {
        Self {
            max_qubits: limits.max_qubits,
            max_circuit_depth: limits.max_circuit_depth,
            max_operations: limits.max_operations,
            max_shots: limits.max_shots,
        }
    }
}

// ============================================================================
// Calibration metadata
// ============================================================================

/// Summary of the calibration state attached to a benchmark.
///
/// Benchmarking must identify the calibration snapshot without duplicating
/// the full calibration model. The complete calibration object remains owned
/// by `quantum::hardware::calibration`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationMetadata {
    backend_id: String,
    timestamp_unix_ns: u128,
    qubit_count: usize,
    gate_count: usize,
    fingerprint: String,
}

impl CalibrationMetadata {
    /// Captures calibration identity.
    pub fn from_snapshot(
        snapshot: &CalibrationSnapshot,
    ) -> Result<Self, HardwareMetadataError> {
        validate_text(
            "calibration.backend_id",
            &snapshot.backend_id,
            MAX_IDENTIFIER_LENGTH,
        )?;

        if snapshot.qubits.len() > MAX_CALIBRATION_QUBITS {
            return Err(HardwareMetadataError::CalibrationTooLarge {
                qubits: snapshot.qubits.len(),
                maximum: MAX_CALIBRATION_QUBITS,
            });
        }

        if snapshot.gates.len() > MAX_CALIBRATION_GATES {
            return Err(HardwareMetadataError::CalibrationGatesTooLarge {
                gates: snapshot.gates.len(),
                maximum: MAX_CALIBRATION_GATES,
            });
        }

        snapshot
            .validate()
            .map_err(|error| HardwareMetadataError::InconsistentMetadata {
                message: error.to_string(),
            })?;

        let fingerprint = calibration_fingerprint(snapshot);

        Ok(Self {
            backend_id: snapshot.backend_id.clone(),
            timestamp_unix_ns: snapshot.timestamp.as_unix_nanos(),
            qubit_count: snapshot.qubits.len(),
            gate_count: snapshot.gates.len(),
            fingerprint,
        })
    }

    /// Backend ID associated with this calibration.
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Calibration timestamp.
    pub const fn timestamp_unix_nanos(&self) -> u128 {
        self.timestamp_unix_ns
    }

    /// Number of calibrated qubits.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Number of calibrated gate entries.
    pub const fn gate_count(&self) -> usize {
        self.gate_count
    }

    /// Deterministic calibration fingerprint.
    ///
    /// This is a provenance fingerprint, not a cryptographic hash.
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

// ============================================================================
// Hardware metadata
// ============================================================================

/// Complete hardware metadata snapshot used by benchmarking.
///
/// This is the principal public type of this module.
///
/// The structure intentionally contains a snapshot rather than a reference to
/// a live backend. A benchmark result must remain meaningful after the backend
/// object has gone out of scope or changed state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareMetadata {
    schema_id: String,
    schema_version: u32,

    backend_id: String,
    backend_name: String,
    provider: String,
    backend_version: String,

    backend_kind: BackendKind,
    backend_status: BackendStatus,
    technology: HardwareTechnology,

    physical_qubits: usize,

    capabilities: CapabilityMetadata,
    limits: LimitMetadata,
    topology: TopologyMetadata,

    calibration: Option<CalibrationMetadata>,

    provider_properties: BTreeMap<String, String>,

    captured_at: MetadataTimestamp,
}

impl HardwareMetadata {
    /// Capture metadata from a complete `QuantumBackend`.
    ///
    /// This is the preferred constructor because it obtains all canonical
    /// backend-owned information from one source.
    pub fn from_backend(
        backend: &QuantumBackend,
    ) -> Result<Self, HardwareMetadataError> {
        Self::from_backend_parts(
            &backend.metadata,
            &backend.capabilities,
            backend.limits,
            &backend.topology,
            None,
            MetadataTimestamp::now(),
        )
    }

    /// Capture metadata from a backend and an explicit calibration snapshot.
    ///
    /// The calibration backend ID must match the backend ID.
    pub fn from_backend_with_calibration(
        backend: &QuantumBackend,
        calibration: &CalibrationSnapshot,
    ) -> Result<Self, HardwareMetadataError> {
        Self::from_backend_parts(
            &backend.metadata,
            &backend.capabilities,
            backend.limits,
            &backend.topology,
            Some(calibration),
            MetadataTimestamp::now(),
        )
    }

    /// Capture metadata from explicit backend components.
    ///
    /// This constructor exists so tests and future adapters can build metadata
    /// without constructing a live execution object.
    pub fn from_backend_parts(
        backend: &BackendMetadata,
        capabilities: &BackendCapabilities,
        limits: BackendLimits,
        topology: &HardwareTopology,
        calibration: Option<&CalibrationSnapshot>,
        captured_at: MetadataTimestamp,
    ) -> Result<Self, HardwareMetadataError> {
        validate_backend_metadata(backend)?;

        let topology_metadata =
            TopologyMetadata::from_topology(topology)?;

        let physical_qubits = topology_metadata.qubit_count();

        if limits.max_qubits != 0
            && limits.max_qubits < physical_qubits
        {
            return Err(
                HardwareMetadataError::BackendLimitBelowTopology {
                    maximum: limits.max_qubits,
                    topology_qubits: physical_qubits,
                },
            );
        }

        let capability_metadata =
            CapabilityMetadata::from_capabilities(capabilities)?;

        let calibration_metadata = match calibration {
            Some(snapshot) => {
                if snapshot.backend_id != backend.id {
                    return Err(
                        HardwareMetadataError::BackendCalibrationMismatch {
                            backend_id: backend.id.clone(),
                            calibration_backend_id: snapshot
                                .backend_id
                                .clone(),
                        },
                    );
                }

                for qubit in snapshot.qubits.keys() {
                    if *qubit >= physical_qubits {
                        return Err(
                            HardwareMetadataError::CalibrationQubitOutsideTopology {
                                qubit: *qubit,
                                topology_qubits: physical_qubits,
                            },
                        );
                    }
                }

                Some(CalibrationMetadata::from_snapshot(snapshot)?)
            }
            None => None,
        };

        validate_provider_properties(&backend.properties)?;

        Ok(Self {
            schema_id: HARDWARE_METADATA_SCHEMA_ID.to_string(),
            schema_version: HARDWARE_METADATA_SCHEMA_VERSION,

            backend_id: backend.id.clone(),
            backend_name: backend.name.clone(),
            provider: backend.provider.clone(),
            backend_version: backend.version.clone(),

            backend_kind: backend.kind,
            backend_status: backend.status,

            technology: infer_technology(
                backend.kind,
                &backend.properties,
            ),

            physical_qubits,

            capabilities: capability_metadata,
            limits: limits.into(),
            topology: topology_metadata,

            calibration: calibration_metadata,

            provider_properties: backend.properties.clone(),

            captured_at,
        })
    }

    /// Create metadata with an explicitly selected technology classification.
    ///
    /// This is useful for provider adapters where technology is known from a
    /// trusted source but cannot be inferred from generic backend properties.
    pub fn with_technology(
        mut self,
        technology: HardwareTechnology,
    ) -> Self {
        self.technology = technology;
        self
    }

    // ------------------------------------------------------------------------
    // Schema
    // ------------------------------------------------------------------------

    /// Metadata schema identifier.
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Metadata schema version.
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    // ------------------------------------------------------------------------
    // Backend identity
    // ------------------------------------------------------------------------

    /// Stable backend identifier.
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Human-readable backend name.
    pub fn backend_name(&self) -> &str {
        &self.backend_name
    }

    /// Backend/provider name.
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Backend implementation/version reported by the provider.
    pub fn backend_version(&self) -> &str {
        &self.backend_version
    }

    /// Backend operational category.
    pub const fn backend_kind(&self) -> BackendKind {
        self.backend_kind
    }

    /// Backend status at metadata capture time.
    pub const fn backend_status(&self) -> BackendStatus {
        self.backend_status
    }

    /// Underlying technology classification.
    pub const fn technology(&self) -> HardwareTechnology {
        self.technology
    }

    /// Physical qubit count.
    pub const fn physical_qubits(&self) -> usize {
        self.physical_qubits
    }

    // ------------------------------------------------------------------------
    // Capabilities and limits
    // ------------------------------------------------------------------------

    /// Backend capabilities.
    pub const fn capabilities(&self) -> &CapabilityMetadata {
        &self.capabilities
    }

    /// Backend resource limits.
    pub const fn limits(&self) -> LimitMetadata {
        self.limits
    }

    /// Topology metadata.
    pub const fn topology(&self) -> &TopologyMetadata {
        &self.topology
    }

    /// Calibration metadata, when a calibration snapshot was supplied.
    pub fn calibration(&self) -> Option<&CalibrationMetadata> {
        self.calibration.as_ref()
    }

    /// Provider-specific metadata.
    ///
    /// Credentials and secrets must never be inserted here.
    pub fn provider_properties(&self) -> &BTreeMap<String, String> {
        &self.provider_properties
    }

    /// Metadata capture timestamp.
    pub const fn captured_at(&self) -> MetadataTimestamp {
        self.captured_at
    }

    // ------------------------------------------------------------------------
    // Capability helpers
    // ------------------------------------------------------------------------

    /// Whether measurement is supported.
    pub const fn supports_measurement(&self) -> bool {
        self.capabilities.measurement
    }

    /// Whether reset is supported.
    pub const fn supports_reset(&self) -> bool {
        self.capabilities.reset
    }

    /// Whether mid-circuit measurement is supported.
    pub const fn supports_mid_circuit_measurement(&self) -> bool {
        self.capabilities.mid_circuit_measurement
    }

    /// Whether dynamic circuits are supported.
    pub const fn supports_dynamic_circuits(&self) -> bool {
        self.capabilities.dynamic_circuits
    }

    /// Whether the backend reports a native gate.
    pub fn supports_gate(&self, gate: &str) -> bool {
        self.capabilities.supports_gate(gate)
    }

    // ------------------------------------------------------------------------
    // Calibration helpers
    // ------------------------------------------------------------------------

    /// Returns whether calibration metadata was attached.
    pub const fn has_calibration(&self) -> bool {
        self.calibration.is_some()
    }

    /// Returns the calibration fingerprint, if present.
    pub fn calibration_fingerprint(&self) -> Option<&str> {
        self.calibration
            .as_ref()
            .map(CalibrationMetadata::fingerprint)
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validate the complete metadata snapshot.
    ///
    /// This should be called before placing metadata into a benchmark result
    /// or provenance record.
    pub fn validate(&self) -> Result<(), HardwareMetadataError> {
        if self.schema_id != HARDWARE_METADATA_SCHEMA_ID {
            return Err(HardwareMetadataError::InconsistentMetadata {
                message: format!(
                    "unexpected schema ID `{}`",
                    self.schema_id
                ),
            });
        }

        if self.schema_version != HARDWARE_METADATA_SCHEMA_VERSION {
            return Err(HardwareMetadataError::InconsistentMetadata {
                message: format!(
                    "unsupported metadata schema version {}",
                    self.schema_version
                ),
            });
        }

        validate_text(
            "backend_id",
            &self.backend_id,
            MAX_IDENTIFIER_LENGTH,
        )?;

        validate_text(
            "backend_name",
            &self.backend_name,
            MAX_TEXT_LENGTH,
        )?;

        validate_text(
            "provider",
            &self.provider,
            MAX_TEXT_LENGTH,
        )?;

        validate_text(
            "backend_version",
            &self.backend_version,
            MAX_TEXT_LENGTH,
        )?;

        if self.physical_qubits == 0 {
            return Err(HardwareMetadataError::InconsistentMetadata {
                message: "physical qubit count cannot be zero".to_string(),
            });
        }

        if self.topology.qubit_count() != self.physical_qubits {
            return Err(
                HardwareMetadataError::BackendTopologyMismatch {
                    backend_qubits: self.physical_qubits,
                    topology_qubits: self.topology.qubit_count(),
                },
            );
        }

        if self.limits.max_qubits != 0
            && self.limits.max_qubits < self.physical_qubits
        {
            return Err(
                HardwareMetadataError::BackendLimitBelowTopology {
                    maximum: self.limits.max_qubits,
                    topology_qubits: self.physical_qubits,
                },
            );
        }

        validate_provider_properties(&self.provider_properties)?;

        if let Some(calibration) = &self.calibration {
            if calibration.backend_id != self.backend_id {
                return Err(
                    HardwareMetadataError::BackendCalibrationMismatch {
                        backend_id: self.backend_id.clone(),
                        calibration_backend_id: calibration.backend_id.clone(),
                    },
                );
            }

            if calibration.qubit_count > self.physical_qubits {
                return Err(HardwareMetadataError::InconsistentMetadata {
                    message: format!(
                        "calibration contains {} qubits but hardware exposes {}",
                        calibration.qubit_count,
                        self.physical_qubits
                    ),
                });
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Fingerprinting
    // ------------------------------------------------------------------------

    /// Returns a deterministic fingerprint of the complete metadata snapshot.
    ///
    /// The fingerprint changes when captured-at time, backend identity,
    /// capabilities, topology, calibration identity, limits, or provider
    /// properties change.
    ///
    /// This is a non-cryptographic fingerprint and MUST NOT be used as a
    /// security hash.
    pub fn fingerprint(
        &self,
    ) -> Result<String, HardwareMetadataError> {
        self.validate()?;

        let canonical = self.canonical_representation();

        Ok(fnv1a_128_hex(canonical.as_bytes()))
    }

    /// Returns a deterministic fingerprint that excludes the volatile capture
    /// timestamp.
    ///
    /// This is useful when determining whether two metadata snapshots describe
    /// the same backend configuration even when they were captured at different
    /// times.
    pub fn configuration_fingerprint(
        &self,
    ) -> Result<String, HardwareMetadataError> {
        self.validate()?;

        let canonical = self.configuration_canonical_representation();

        Ok(fnv1a_128_hex(canonical.as_bytes()))
    }

    /// Builds the canonical representation used by fingerprinting.
    ///
    /// This representation is an internal stable contract. It deliberately
    /// avoids `Debug` formatting because enum/debug formatting is not an ideal
    /// interchange representation.
    fn canonical_representation(&self) -> String {
        let mut value = self.configuration_canonical_representation();

        value.push_str("|captured_at=");
        value.push_str(
            &self.captured_at.as_unix_nanos().to_string(),
        );

        value
    }

    /// Canonical representation excluding volatile capture time.
    fn configuration_canonical_representation(&self) -> String {
        let mut value = String::new();

        push_field(&mut value, "schema_id", &self.schema_id);
        push_field(
            &mut value,
            "schema_version",
            &self.schema_version.to_string(),
        );

        push_field(&mut value, "backend_id", &self.backend_id);
        push_field(&mut value, "backend_name", &self.backend_name);
        push_field(&mut value, "provider", &self.provider);
        push_field(
            &mut value,
            "backend_version",
            &self.backend_version,
        );

        push_field(
            &mut value,
            "backend_kind",
            backend_kind_name(self.backend_kind),
        );

        push_field(
            &mut value,
            "backend_status",
            backend_status_name(self.backend_status),
        );

        push_field(
            &mut value,
            "technology",
            self.technology.as_str(),
        );

        push_field(
            &mut value,
            "physical_qubits",
            &self.physical_qubits.to_string(),
        );

        // Capabilities.
        push_field(
            &mut value,
            "capability.measurement",
            bool_name(self.capabilities.measurement),
        );

        push_field(
            &mut value,
            "capability.reset",
            bool_name(self.capabilities.reset),
        );

        push_field(
            &mut value,
            "capability.mid_circuit_measurement",
            bool_name(
                self.capabilities.mid_circuit_measurement,
            ),
        );

        push_field(
            &mut value,
            "capability.classical_control",
            bool_name(self.capabilities.classical_control),
        );

        push_field(
            &mut value,
            "capability.arbitrary_single_qubit_rotations",
            bool_name(
                self.capabilities
                    .arbitrary_single_qubit_rotations,
            ),
        );

        push_field(
            &mut value,
            "capability.parameterized_gates",
            bool_name(self.capabilities.parameterized_gates),
        );

        push_field(
            &mut value,
            "capability.dynamic_circuits",
            bool_name(self.capabilities.dynamic_circuits),
        );

        for (index, gate) in
            self.capabilities.native_gates.iter().enumerate()
        {
            push_field(
                &mut value,
                &format!("native_gate.{index}"),
                gate,
            );
        }

        // Limits.
        push_field(
            &mut value,
            "limit.max_qubits",
            &self.limits.max_qubits.to_string(),
        );

        push_field(
            &mut value,
            "limit.max_circuit_depth",
            &self.limits.max_circuit_depth.to_string(),
        );

        push_field(
            &mut value,
            "limit.max_operations",
            &self.limits.max_operations.to_string(),
        );

        push_field(
            &mut value,
            "limit.max_shots",
            &self.limits.max_shots.to_string(),
        );

        // Topology.
        push_field(
            &mut value,
            "topology.qubit_count",
            &self.topology.qubit_count().to_string(),
        );

        push_field(
            &mut value,
            "topology.coupling_count",
            &self.topology.coupling_count().to_string(),
        );

        push_field(
            &mut value,
            "topology.fully_connected",
            bool_name(self.topology.is_fully_connected()),
        );

        push_field(
            &mut value,
            "topology.maximum_degree",
            &self.topology.maximum_degree().to_string(),
        );

        for (index, coupling) in
            self.topology.couplings().iter().enumerate()
        {
            push_field(
                &mut value,
                &format!("topology.coupling.{index}.source"),
                &coupling.source.to_string(),
            );

            push_field(
                &mut value,
                &format!("topology.coupling.{index}.target"),
                &coupling.target.to_string(),
            );

            push_field(
                &mut value,
                &format!("topology.coupling.{index}.bidirectional"),
                bool_name(coupling.bidirectional),
            );
        }

        // Calibration.
        match &self.calibration {
            Some(calibration) => {
                push_field(
                    &mut value,
                    "calibration.present",
                    "true",
                );

                push_field(
                    &mut value,
                    "calibration.backend_id",
                    &calibration.backend_id,
                );

                push_field(
                    &mut value,
                    "calibration.timestamp_unix_ns",
                    &calibration
                        .timestamp_unix_ns
                        .to_string(),
                );

                push_field(
                    &mut value,
                    "calibration.qubit_count",
                    &calibration.qubit_count.to_string(),
                );

                push_field(
                    &mut value,
                    "calibration.gate_count",
                    &calibration.gate_count.to_string(),
                );

                push_field(
                    &mut value,
                    "calibration.fingerprint",
                    &calibration.fingerprint,
                );
            }

            None => {
                push_field(
                    &mut value,
                    "calibration.present",
                    "false",
                );
            }
        }

        // Provider properties are already ordered by BTreeMap.
        for (key, property) in
            &self.provider_properties
        {
            push_field(
                &mut value,
                &format!("provider_property.{key}"),
                property,
            );
        }

        value
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for benchmark hardware metadata.
///
/// The builder is useful for provider adapters that cannot construct a
/// `QuantumBackend` because they are outside the core backend execution layer.
///
/// Normal Zamani execution should prefer `HardwareMetadata::from_backend`.
#[derive(Debug, Clone)]
pub struct HardwareMetadataBuilder {
    backend: BackendMetadata,
    capabilities: BackendCapabilities,
    limits: BackendLimits,
    topology: HardwareTopology,
    calibration: Option<CalibrationSnapshot>,
    technology: Option<HardwareTechnology>,
    captured_at: MetadataTimestamp,
}

impl HardwareMetadataBuilder {
    /// Creates a builder.
    pub fn new(
        backend: BackendMetadata,
        capabilities: BackendCapabilities,
        limits: BackendLimits,
        topology: HardwareTopology,
    ) -> Self {
        Self {
            backend,
            capabilities,
            limits,
            topology,
            calibration: None,
            technology: None,
            captured_at: MetadataTimestamp::now(),
        }
    }

    /// Attaches a calibration snapshot.
    pub fn with_calibration(
        mut self,
        calibration: CalibrationSnapshot,
    ) -> Self {
        self.calibration = Some(calibration);
        self
    }

    /// Overrides automatic technology inference.
    pub fn with_technology(
        mut self,
        technology: HardwareTechnology,
    ) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Uses an explicit capture timestamp.
    pub fn with_capture_timestamp(
        mut self,
        timestamp: MetadataTimestamp,
    ) -> Self {
        self.captured_at = timestamp;
        self
    }

    /// Builds validated metadata.
    pub fn build(
        self,
    ) -> Result<HardwareMetadata, HardwareMetadataError> {
        let mut metadata = HardwareMetadata::from_backend_parts(
            &self.backend,
            &self.capabilities,
            self.limits,
            &self.topology,
            self.calibration.as_ref(),
            self.captured_at,
        )?;

        if let Some(technology) = self.technology {
            metadata.technology = technology;
        }

        metadata.validate()?;

        Ok(metadata)
    }
}

// ============================================================================
// Conversion helpers
// ============================================================================

impl TryFrom<&QuantumBackend> for HardwareMetadata {
    type Error = HardwareMetadataError;

    fn try_from(
        backend: &QuantumBackend,
    ) -> Result<Self, Self::Error> {
        Self::from_backend(backend)
    }
}

impl TryFrom<&BackendMetadata> for HardwareMetadata {
    type Error = HardwareMetadataError;

    /// This conversion cannot infer topology/capabilities and therefore is
    /// intentionally not implemented as a blanket conversion.
    ///
    /// The trait implementation is omitted deliberately. Callers must use
    /// `HardwareMetadataBuilder` or `from_backend_parts`.
}

// ============================================================================
// Internal validation
// ============================================================================

fn validate_backend_metadata(
    metadata: &BackendMetadata,
) -> Result<(), HardwareMetadataError> {
    validate_text(
        "backend_id",
        &metadata.id,
        MAX_IDENTIFIER_LENGTH,
    )?;

    validate_text(
        "backend_name",
        &metadata.name,
        MAX_TEXT_LENGTH,
    )?;

    validate_text(
        "provider",
        &metadata.provider,
        MAX_TEXT_LENGTH,
    )?;

    validate_text(
        "backend_version",
        &metadata.version,
        MAX_TEXT_LENGTH,
    )?;

    if metadata.id.trim().is_empty() {
        return Err(HardwareMetadataError::InvalidBackendIdentity);
    }

    validate_provider_properties(&metadata.properties)
}

fn validate_provider_properties(
    properties: &BTreeMap<String, String>,
) -> Result<(), HardwareMetadataError> {
    if properties.len() > MAX_PROPERTIES {
        return Err(HardwareMetadataError::TooManyProperties {
            count: properties.len(),
            maximum: MAX_PROPERTIES,
        });
    }

    for (key, value) in properties {
        if key.trim().is_empty() {
            return Err(HardwareMetadataError::InvalidPropertyKey {
                key: key.clone(),
            });
        }

        if key.len() > MAX_PROPERTY_KEY_LENGTH {
            return Err(HardwareMetadataError::FieldTooLong {
                field: "provider_property_key",
                length: key.len(),
                maximum: MAX_PROPERTY_KEY_LENGTH,
            });
        }

        if value.len() > MAX_PROPERTY_VALUE_LENGTH {
            return Err(
                HardwareMetadataError::PropertyValueTooLong {
                    key: key.clone(),
                    length: value.len(),
                    maximum: MAX_PROPERTY_VALUE_LENGTH,
                },
            );
        }

        // Refuse embedded control characters. This protects later JSON,
        // Markdown, CSV and log/report integrations from malformed metadata.
        if key.chars().any(char::is_control)
            || value.chars().any(char::is_control)
        {
            return Err(
                HardwareMetadataError::InconsistentMetadata {
                    message: format!(
                        "provider property `{key}` contains control characters"
                    ),
                },
            );
        }
    }

    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HardwareMetadataError> {
    if value.trim().is_empty() {
        return Err(HardwareMetadataError::EmptyField { field });
    }

    if value.len() > maximum {
        return Err(HardwareMetadataError::FieldTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HardwareMetadataError::InconsistentMetadata {
            message: format!(
                "field `{field}` contains control characters"
            ),
        });
    }

    Ok(())
}

fn validate_coupling(
    coupling: &Coupling,
    qubit_count: usize,
) -> Result<(), HardwareMetadataError> {
    if coupling.source >= qubit_count {
        return Err(
            HardwareMetadataError::TopologyQubitOutOfRange {
                qubit: coupling.source,
                topology_qubits: qubit_count,
            },
        );
    }

    if coupling.target >= qubit_count {
        return Err(
            HardwareMetadataError::TopologyQubitOutOfRange {
                qubit: coupling.target,
                topology_qubits: qubit_count,
            },
        );
    }

    if coupling.source == coupling.target {
        return Err(HardwareMetadataError::InvalidCoupling {
            source: coupling.source,
            target: coupling.target,
        });
    }

    Ok(())
}

// ============================================================================
// Technology inference
// ============================================================================

/// Infer technology from the existing backend descriptor.
///
/// Providers can override the result with `with_technology()` when their
/// adapter has authoritative technology information.
fn infer_technology(
    kind: BackendKind,
    properties: &BTreeMap<String, String>,
) -> HardwareTechnology {
    let technology_value = properties
        .get("technology")
        .or_else(|| properties.get("technology_type"))
        .or_else(|| properties.get("architecture"));

    if let Some(value) = technology_value {
        if let Some(technology) =
            parse_technology(value)
        {
            return technology;
        }
    }

    match kind {
        BackendKind::Simulator => {
            HardwareTechnology::ClassicalSimulator
        }

        BackendKind::Emulator => {
            HardwareTechnology::Emulator
        }

        BackendKind::Qpu | BackendKind::Custom => {
            HardwareTechnology::Unknown
        }
    }
}

fn parse_technology(value: &str) -> Option<HardwareTechnology> {
    let normalized = value
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_")
        .replace(' ', "_");

    match normalized.as_str() {
        "simulator" | "classical_simulator" => {
            Some(HardwareTechnology::ClassicalSimulator)
        }

        "emulator" => {
            Some(HardwareTechnology::Emulator)
        }

        "superconducting"
        | "superconducting_qubit"
        | "superconducting_qubits" => {
            Some(HardwareTechnology::Superconducting)
        }

        "trapped_ion"
        | "trapped_ions"
        | "ion_trap"
        | "ion_traps" => {
            Some(HardwareTechnology::TrappedIon)
        }

        "neutral_atom"
        | "neutral_atoms" => {
            Some(HardwareTechnology::NeutralAtom)
        }

        "photonic"
        | "photonics" => {
            Some(HardwareTechnology::Photonic)
        }

        "spin"
        | "spin_qubit"
        | "spin_qubits" => {
            Some(HardwareTechnology::Spin)
        }

        "quantum_dot"
        | "quantum_dots" => {
            Some(HardwareTechnology::QuantumDot)
        }

        "topological"
        | "topological_qubit"
        | "topological_qubits" => {
            Some(HardwareTechnology::Topological)
        }

        "annealing"
        | "quantum_annealing"
        | "quantum_annealer" => {
            Some(HardwareTechnology::Annealing)
        }

        "analog"
        | "analog_quantum" => {
            Some(HardwareTechnology::Analog)
        }

        "other" => {
            Some(HardwareTechnology::Other)
        }

        "unknown" => {
            Some(HardwareTechnology::Unknown)
        }

        _ => None,
    }
}

// ============================================================================
// Canonical names
// ============================================================================

fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_lowercase()
}

fn backend_kind_name(kind: BackendKind) -> &'static str {
    match kind {
        BackendKind::Simulator => "simulator",
        BackendKind::Emulator => "emulator",
        BackendKind::Qpu => "qpu",
        BackendKind::Custom => "custom",
    }
}

fn backend_status_name(status: BackendStatus) -> &'static str {
    match status {
        BackendStatus::Available => "available",
        BackendStatus::Busy => "busy",
        BackendStatus::Maintenance => "maintenance",
        BackendStatus::Offline => "offline",
        BackendStatus::Unavailable => "unavailable",
    }
}

fn bool_name(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn push_field(
    output: &mut String,
    key: &str,
    value: &str,
) {
    output.push_str(key.len().to_string().as_str());
    output.push(':');
    output.push_str(key);
    output.push('=');
    output.push_str(value.len().to_string().as_str());
    output.push(':');
    output.push_str(value);
    output.push('|');
}

// ============================================================================
// Calibration fingerprinting
// ============================================================================

/// Create a deterministic fingerprint for a calibration snapshot.
///
/// Calibration maps are already `BTreeMap`s, so iteration order is stable.
fn calibration_fingerprint(
    snapshot: &CalibrationSnapshot,
) -> String {
    let mut canonical = String::new();

    push_field(
        &mut canonical,
        "backend_id",
        &snapshot.backend_id,
    );

    push_field(
        &mut canonical,
        "timestamp",
        &snapshot.timestamp
            .as_unix_nanos()
            .to_string(),
    );

    for (qubit_id, calibration) in
        &snapshot.qubits
    {
        push_field(
            &mut canonical,
            "qubit.id",
            &qubit_id.to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.t1_ns",
            &calibration.t1_ns.to_bits().to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.t2_ns",
            &calibration.t2_ns.to_bits().to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.reset_error",
            &calibration.reset_error.to_bits().to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.frequency_hz",
            &calibration.frequency_hz.to_bits().to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.readout.p01",
            &calibration
                .readout
                .p01
                .to_bits()
                .to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.readout.p10",
            &calibration
                .readout
                .p10
                .to_bits()
                .to_string(),
        );

        push_field(
            &mut canonical,
            "qubit.readout.shots",
            &calibration
                .readout
                .shots
                .to_string(),
        );
    }

    for (key, gate) in &snapshot.gates {
        push_field(
            &mut canonical,
            "gate.key",
            key,
        );

        push_field(
            &mut canonical,
            "gate.name",
            &gate.gate,
        );

        for (index, qubit) in
            gate.qubits.iter().enumerate()
        {
            push_field(
                &mut canonical,
                &format!("gate.qubit.{index}"),
                &qubit.to_string(),
            );
        }

        push_field(
            &mut canonical,
            "gate.duration_ns",
            &gate.duration_ns.to_string(),
        );

        push_field(
            &mut canonical,
            "gate.error_rate",
            &gate.error_rate.to_bits().to_string(),
        );

        push_field(
            &mut canonical,
            "gate.shots",
            &gate.shots.to_string(),
        );

        push_field(
            &mut canonical,
            "gate.operational",
            bool_name(gate.operational),
        );
    }

    for (key, value) in &snapshot.metadata {
        push_field(
            &mut canonical,
            &format!("metadata.{key}"),
            value,
        );
    }

    fnv1a_128_hex(canonical.as_bytes())
}

// ============================================================================
// Deterministic fingerprint
// ============================================================================

/// FNV-1a 128-bit non-cryptographic hash.
///
/// This implementation is intentionally local so the metadata module does not
/// require a new dependency merely for deterministic provenance fingerprints.
///
/// It is NOT suitable for:
///
/// - authentication;
//! - tamper-proofing;
//! - signatures;
//! - cryptographic commitments.
//!
//! A future cryptographic provenance layer must use a cryptographic digest.
fn fnv1a_128_hex(bytes: &[u8]) -> String {
    const OFFSET: u128 =
        0x6c62272e07bb014262b821756295c58d;

    const PRIME: u128 =
        0x00000000010000000000000000000013;

    let mut hash = OFFSET;

    for byte in bytes {
        hash ^= *byte as u128;
        hash = hash.wrapping_mul(PRIME);
    }

    format!("{hash:032x}")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::hardware::backend::{
        BackendCapabilities,
        BackendKind,
        BackendLimits,
        BackendMetadata,
    };

    use crate::quantum::hardware::calibration::{
        CalibrationSnapshot,
        GateCalibration,
        QubitCalibration,
    };

    #[test]
    fn timestamp_round_trip() {
        let timestamp =
            MetadataTimestamp::from_unix_nanos(123_456_789);

        assert_eq!(
            timestamp.as_unix_nanos(),
            123_456_789
        );

        assert!(!timestamp.is_epoch());
    }

    #[test]
    fn topology_metadata_is_deterministic() {
        let topology =
            HardwareTopology::linear(4).unwrap();

        let first =
            TopologyMetadata::from_topology(&topology)
                .unwrap();

        let second =
            TopologyMetadata::from_topology(&topology)
                .unwrap();

        assert_eq!(first, second);
        assert_eq!(first.qubit_count(), 4);
        assert_eq!(first.coupling_count(), 3);
    }

    #[test]
    fn capabilities_are_sorted() {
        let capabilities =
            BackendCapabilities::new()
                .with_gates([
                    "CX",
                    "H",
                    "x",
                    "rz",
                ]);

        let metadata =
            CapabilityMetadata::from_capabilities(
                &capabilities,
            )
            .unwrap();

        assert_eq!(
            metadata.native_gates,
            vec![
                "cx".to_string(),
                "h".to_string(),
                "rz".to_string(),
                "x".to_string(),
            ]
        );
    }

    #[test]
    fn technology_is_inferred_from_property() {
        let mut backend =
            BackendMetadata::new(
                "qpu-1",
                "Test QPU",
                "test-provider",
                "1.0",
                BackendKind::Qpu,
            );

        backend.insert_property(
            "technology",
            "superconducting",
        );

        let topology =
            HardwareTopology::linear(5).unwrap();

        let metadata =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(100),
            )
            .unwrap();

        assert_eq!(
            metadata.technology(),
            HardwareTechnology::Superconducting
        );
    }

    #[test]
    fn simulator_technology_is_inferred() {
        let backend =
            BackendMetadata::new(
                "sim",
                "Simulator",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            );

        let topology =
            HardwareTopology::new(2).unwrap();

        let metadata =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(100),
            )
            .unwrap();

        assert_eq!(
            metadata.technology(),
            HardwareTechnology::ClassicalSimulator
        );
    }

    #[test]
    fn backend_topology_mismatch_is_rejected() {
        let backend =
            BackendMetadata::new(
                "qpu",
                "QPU",
                "provider",
                "1",
                BackendKind::Qpu,
            );

        let topology =
            HardwareTopology::new(3).unwrap();

        let limits =
            BackendLimits::unlimited()
                .with_max_qubits(2);

        let result =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                limits,
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(1),
            );

        assert!(matches!(
            result,
            Err(
                HardwareMetadataError::BackendLimitBelowTopology {
                    maximum: 2,
                    topology_qubits: 3
                }
            )
        ));
    }

    #[test]
    fn calibration_backend_mismatch_is_rejected() {
        let backend =
            BackendMetadata::new(
                "backend-a",
                "A",
                "provider",
                "1",
                BackendKind::Qpu,
            );

        let topology =
            HardwareTopology::new(2).unwrap();

        let calibration =
            CalibrationSnapshot::new("backend-b");

        let result =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                Some(&calibration),
                MetadataTimestamp::from_unix_nanos(1),
            );

        assert!(matches!(
            result,
            Err(
                HardwareMetadataError::BackendCalibrationMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn calibration_is_fingerprinted() {
        let mut calibration =
            CalibrationSnapshot::with_timestamp(
                "qpu",
                crate::quantum::hardware::calibration::CalibrationTimestamp::from_unix_nanos(
                    42
                ),
            );

        let qubit =
            QubitCalibration::new(0)
                .unwrap()
                .with_t1_ns(1000.0)
                .unwrap()
                .with_t2_ns(800.0)
                .unwrap();

        calibration.insert_qubit(qubit);

        let gate =
            GateCalibration::new(
                "cx",
                vec![0, 1],
            )
            .unwrap()
            .with_duration_ns(20)
            .unwrap()
            .with_error_rate(0.001)
            .unwrap()
            .with_shots(1000);

        calibration.insert_gate(gate);

        let metadata =
            CalibrationMetadata::from_snapshot(
                &calibration,
            )
            .unwrap();

        assert_eq!(
            metadata.backend_id(),
            "qpu"
        );

        assert_eq!(
            metadata.timestamp_unix_nanos(),
            42
        );

        assert!(!metadata.fingerprint().is_empty());
    }

    #[test]
    fn metadata_fingerprint_is_deterministic() {
        let backend =
            BackendMetadata::new(
                "sim",
                "Simulator",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            );

        let topology =
            HardwareTopology::linear(3).unwrap();

        let metadata =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(123),
            )
            .unwrap();

        let first =
            metadata.fingerprint().unwrap();

        let second =
            metadata.fingerprint().unwrap();

        assert_eq!(first, second);
        assert_eq!(first.len(), 32);
    }

    #[test]
    fn configuration_fingerprint_ignores_capture_time() {
        let backend =
            BackendMetadata::new(
                "sim",
                "Simulator",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            );

        let topology =
            HardwareTopology::linear(3).unwrap();

        let first =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(100),
            )
            .unwrap();

        let second =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(200),
            )
            .unwrap();

        assert_ne!(
            first.fingerprint().unwrap(),
            second.fingerprint().unwrap()
        );

        assert_eq!(
            first.configuration_fingerprint().unwrap(),
            second.configuration_fingerprint().unwrap()
        );
    }

    #[test]
    fn provider_properties_are_rejected_when_oversized() {
        let backend =
            BackendMetadata::new(
                "backend",
                "Backend",
                "Provider",
                "1",
                BackendKind::Qpu,
            );

        let mut properties =
            backend.properties.clone();

        properties.insert(
            "x".to_string(),
            "a".repeat(
                MAX_PROPERTY_VALUE_LENGTH + 1
            ),
        );

        let mut invalid = backend;
        invalid.properties = properties;

        let result =
            validate_backend_metadata(&invalid);

        assert!(matches!(
            result,
            Err(
                HardwareMetadataError::PropertyValueTooLong {
                    ..
                }
            )
        ));
    }

    #[test]
    fn builder_produces_valid_metadata() {
        let backend =
            BackendMetadata::new(
                "sim",
                "Zamani Simulator",
                "Zamani",
                "1.0",
                BackendKind::Simulator,
            );

        let capabilities =
            BackendCapabilities::default()
                .with_gates(["x", "h"]);

        let topology =
            HardwareTopology::linear(4).unwrap();

        let metadata =
            HardwareMetadataBuilder::new(
                backend,
                capabilities,
                BackendLimits::unlimited(),
                topology,
            )
            .with_technology(
                HardwareTechnology::ClassicalSimulator,
            )
            .with_capture_timestamp(
                MetadataTimestamp::from_unix_nanos(999),
            )
            .build()
            .unwrap();

        assert_eq!(
            metadata.backend_id(),
            "sim"
        );

        assert_eq!(
            metadata.physical_qubits(),
            4
        );

        assert_eq!(
            metadata.technology(),
            HardwareTechnology::ClassicalSimulator
        );

        metadata.validate().unwrap();
    }

    #[test]
    fn no_credentials_are_modeled() {
        // This test documents an intentional security boundary:
        // authentication material is not part of HardwareMetadata.
        //
        // Provider-specific descriptive properties remain available, but
        // credentials must never be represented by this module.
        let backend =
            BackendMetadata::new(
                "backend",
                "Backend",
                "Provider",
                "1",
                BackendKind::Qpu,
            );

        let topology =
            HardwareTopology::new(1).unwrap();

        let metadata =
            HardwareMetadata::from_backend_parts(
                &backend,
                &BackendCapabilities::default(),
                BackendLimits::unlimited(),
                &topology,
                None,
                MetadataTimestamp::from_unix_nanos(1),
            )
            .unwrap();

        assert!(metadata.provider_properties().is_empty());
    }
}