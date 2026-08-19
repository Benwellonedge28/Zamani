//! Zamani Quantum — Hardware Backend Abstraction
//!
//! Production-oriented abstraction for quantum execution backends.
//!
//! Responsibilities:
//! - Describe simulator, emulator, QPU, and custom backends.
//! - Validate circuits against backend capabilities and resource limits.
//! - Validate gate support and connectivity through `HardwareTopology`.
//! - Keep hardware execution separate from compilation/transpilation.
//! - Provide deterministic backend metadata for scheduling and routing.
//!
//! This module intentionally does NOT perform network or device I/O.
//! Concrete execution providers should implement `QuantumBackend`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::topology::HardwareTopology;

// -----------------------------------------------------------------------------
// Backend identity
// -----------------------------------------------------------------------------

/// Kind of quantum execution backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendKind {
    /// Pure software simulator.
    Simulator,

    /// Software emulator modeling a specific hardware architecture.
    Emulator,

    /// Physical quantum processing unit.
    Qpu,

    /// Repository/application-specific backend.
    Custom,
}

/// Operational state of a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendStatus {
    Available,
    Busy,
    Maintenance,
    Offline,
    Unavailable,
}

impl BackendStatus {
    pub fn is_usable(self) -> bool {
        matches!(self, Self::Available)
    }
}

// -----------------------------------------------------------------------------
// Backend capabilities
// -----------------------------------------------------------------------------

/// Capabilities exposed by a backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// Whether the backend supports measurement operations.
    pub measurement: bool,

    /// Whether qubits can be explicitly reset.
    pub reset: bool,

    /// Whether measurement can occur before the circuit terminates.
    pub mid_circuit_measurement: bool,

    /// Whether classical conditions can control subsequent operations.
    pub classical_control: bool,

    /// Whether arbitrary single-qubit rotations are supported.
    pub arbitrary_single_qubit_rotations: bool,

    /// Whether parameterized gates can be submitted without prior binding.
    pub parameterized_gates: bool,

    /// Whether the backend supports dynamic circuits.
    pub dynamic_circuits: bool,

    /// Native gate set.
    pub native_gates: BTreeSet<String>,
}

impl Default for BackendCapabilities {
    fn default() -> Self {
        Self {
            measurement: true,
            reset: true,
            mid_circuit_measurement: false,
            classical_control: false,
            arbitrary_single_qubit_rotations: false,
            parameterized_gates: false,
            dynamic_circuits: false,
            native_gates: BTreeSet::new(),
        }
    }
}

impl BackendCapabilities {
    /// Create a conservative capability profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a native gate.
    pub fn with_gate(mut self, gate: impl Into<String>) -> Self {
        self.native_gates.insert(normalize_gate_name(&gate.into()));
        self
    }

    /// Register several native gates.
    pub fn with_gates<I, S>(mut self, gates: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for gate in gates {
            self.native_gates
                .insert(normalize_gate_name(&gate.into()));
        }

        self
    }

    pub fn supports_gate(&self, gate: &str) -> bool {
        self.native_gates.contains(&normalize_gate_name(gate))
    }
}

// -----------------------------------------------------------------------------
// Resource limits
// -----------------------------------------------------------------------------

/// Hard backend resource limits.
///
/// Zero means "unlimited/not specified" for optional limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendLimits {
    pub max_qubits: usize,
    pub max_circuit_depth: usize,
    pub max_operations: usize,
    pub max_shots: usize,
}

impl Default for BackendLimits {
    fn default() -> Self {
        Self {
            max_qubits: 0,
            max_circuit_depth: 0,
            max_operations: 0,
            max_shots: 0,
        }
    }
}

impl BackendLimits {
    pub fn unlimited() -> Self {
        Self::default()
    }

    pub fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    pub fn with_max_depth(mut self, value: usize) -> Self {
        self.max_circuit_depth = value;
        self
    }

    pub fn with_max_operations(mut self, value: usize) -> Self {
        self.max_operations = value;
        self
    }

    pub fn with_max_shots(mut self, value: usize) -> Self {
        self.max_shots = value;
        self
    }

    pub fn allows_qubits(&self, count: usize) -> bool {
        self.max_qubits == 0 || count <= self.max_qubits
    }

    pub fn allows_depth(&self, depth: usize) -> bool {
        self.max_circuit_depth == 0 || depth <= self.max_circuit_depth
    }

    pub fn allows_operations(&self, operations: usize) -> bool {
        self.max_operations == 0 || operations <= self.max_operations
    }

    pub fn allows_shots(&self, shots: usize) -> bool {
        self.max_shots == 0 || shots <= self.max_shots
    }
}

// -----------------------------------------------------------------------------
// Backend metadata
// -----------------------------------------------------------------------------

/// Stable metadata describing a backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendMetadata {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub version: String,
    pub kind: BackendKind,
    pub status: BackendStatus,
    pub properties: BTreeMap<String, String>,
}

impl BackendMetadata {
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
            properties: BTreeMap::new(),
        }
    }

    pub fn set_status(&mut self, status: BackendStatus) {
        self.status = status;
    }

    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.properties.insert(key.into(), value.into());
    }
}

// -----------------------------------------------------------------------------
// Circuit description used for validation
// -----------------------------------------------------------------------------

/// Lightweight circuit requirements consumed by the backend validator.
///
/// The actual quantum circuit representation remains in
/// `crate::quantum::ir::circuit`.
///
/// This keeps the backend layer independent from the exact circuit
/// implementation while still allowing the compiler pipeline to validate
/// resource requirements before execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitRequirements {
    pub qubit_count: usize,
    pub circuit_depth: usize,
    pub operation_count: usize,
    pub shots: usize,

    /// Gates used by the circuit.
    pub gates: Vec<String>,

    /// Qubit pairs requiring a native two-qubit interaction.
    pub two_qubit_edges: Vec<(usize, usize)>,

    pub requires_measurement: bool,
    pub requires_reset: bool,
    pub requires_mid_circuit_measurement: bool,
    pub requires_classical_control: bool,
    pub requires_dynamic_circuits: bool,
}

impl Default for CircuitRequirements {
    fn default() -> Self {
        Self {
            qubit_count: 0,
            circuit_depth: 0,
            operation_count: 0,
            shots: 1,
            gates: Vec::new(),
            two_qubit_edges: Vec::new(),
            requires_measurement: false,
            requires_reset: false,
            requires_mid_circuit_measurement: false,
            requires_classical_control: false,
            requires_dynamic_circuits: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Backend errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    BackendUnavailable {
        backend_id: String,
        status: BackendStatus,
    },

    InvalidBackendId,

    ZeroQubits,

    QubitLimitExceeded {
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

    UnsupportedGate {
        gate: String,
    },

    MeasurementUnsupported,

    ResetUnsupported,

    MidCircuitMeasurementUnsupported,

    ClassicalControlUnsupported,

    DynamicCircuitUnsupported,

    InvalidQubit {
        qubit: usize,
        qubit_count: usize,
    },

    UnsupportedConnection {
        control: usize,
        target: usize,
    },

    EmptyTopology,

    InvalidTopology(String),

    ExecutionUnavailable(String),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable {
                backend_id,
                status,
            } => write!(
                f,
                "backend '{}' is not available ({:?})",
                backend_id, status
            ),

            Self::InvalidBackendId => {
                write!(f, "backend ID cannot be empty")
            }

            Self::ZeroQubits => {
                write!(f, "circuit must contain at least one qubit")
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "circuit requires {} qubits but backend supports at most {}",
                requested, maximum
            ),

            Self::CircuitDepthExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "circuit depth {} exceeds backend limit {}",
                requested, maximum
            ),

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "operation count {} exceeds backend limit {}",
                requested, maximum
            ),

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "shot count {} exceeds backend limit {}",
                requested, maximum
            ),

            Self::UnsupportedGate { gate } => {
                write!(f, "backend does not support gate '{}'", gate)
            }

            Self::MeasurementUnsupported => {
                write!(f, "backend does not support measurement")
            }

            Self::ResetUnsupported => {
                write!(f, "backend does not support qubit reset")
            }

            Self::MidCircuitMeasurementUnsupported => {
                write!(f, "backend does not support mid-circuit measurement")
            }

            Self::ClassicalControlUnsupported => {
                write!(f, "backend does not support classical control")
            }

            Self::DynamicCircuitUnsupported => {
                write!(f, "backend does not support dynamic circuits")
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => write!(
                f,
                "qubit {} is outside circuit range 0..{}",
                qubit,
                qubit_count.saturating_sub(1)
            ),

            Self::UnsupportedConnection { control, target } => {
                write!(
                    f,
                    "backend topology does not support connection {} -> {}",
                    control, target
                )
            }

            Self::EmptyTopology => {
                write!(f, "backend topology contains no qubits")
            }

            Self::InvalidTopology(message) => {
                write!(f, "invalid backend topology: {}", message)
            }

            Self::ExecutionUnavailable(message) => {
                write!(f, "execution unavailable: {}", message)
            }
        }
    }
}

impl std::error::Error for BackendError {}

// -----------------------------------------------------------------------------
// Execution request/result
// -----------------------------------------------------------------------------

/// Backend execution request.
///
/// Concrete providers can convert this into their native API request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    pub circuit: CircuitRequirements,
}

impl ExecutionRequest {
    pub fn new(circuit: CircuitRequirements) -> Self {
        Self { circuit }
    }
}

/// Generic execution result.
///
/// Backend-specific information should be stored in `metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub backend_id: String,
    pub shots: usize,
    pub counts: BTreeMap<String, usize>,
    pub metadata: BTreeMap<String, String>,
}

impl ExecutionResult {
    pub fn empty(backend_id: impl Into<String>, shots: usize) -> Self {
        Self {
            backend_id: backend_id.into(),
            shots,
            counts: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// Quantum backend
// -----------------------------------------------------------------------------

/// Production-facing quantum backend descriptor.
///
/// Execution itself is deliberately represented as a separate method that
/// currently returns `ExecutionUnavailable`. Concrete simulator/QPU adapters
/// should wrap this type and provide actual execution.
#[derive(Debug, Clone)]
pub struct QuantumBackend {
    pub metadata: BackendMetadata,
    pub capabilities: BackendCapabilities,
    pub limits: BackendLimits,
    pub topology: HardwareTopology,
}

impl QuantumBackend {
    /// Create a backend.
    pub fn new(
        metadata: BackendMetadata,
        capabilities: BackendCapabilities,
        limits: BackendLimits,
        topology: HardwareTopology,
    ) -> Result<Self, BackendError> {
        validate_backend_metadata(&metadata)?;

        Ok(Self {
            metadata,
            capabilities,
            limits,
            topology,
        })
    }

    /// Returns the stable backend identifier.
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    /// Returns backend kind.
    pub fn kind(&self) -> BackendKind {
        self.metadata.kind
    }

    /// Returns whether execution is currently permitted.
    pub fn is_available(&self) -> bool {
        self.metadata.status.is_usable()
    }

    /// Change backend operational status.
    pub fn set_status(&mut self, status: BackendStatus) {
        self.metadata.status = status;
    }

    /// Returns the number of physical qubits described by the topology.
    pub fn qubit_count(&self) -> usize {
        topology_qubit_count(&self.topology)
    }

    /// Validate a circuit before handing it to an execution adapter.
    pub fn validate(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        if !self.is_available() {
            return Err(BackendError::BackendUnavailable {
                backend_id: self.metadata.id.clone(),
                status: self.metadata.status,
            });
        }

        if requirements.qubit_count == 0 {
            return Err(BackendError::ZeroQubits);
        }

        if !self.limits.allows_qubits(requirements.qubit_count) {
            return Err(BackendError::QubitLimitExceeded {
                requested: requirements.qubit_count,
                maximum: self.limits.max_qubits,
            });
        }

        if !self.limits.allows_depth(requirements.circuit_depth) {
            return Err(BackendError::CircuitDepthExceeded {
                requested: requirements.circuit_depth,
                maximum: self.limits.max_circuit_depth,
            });
        }

        if !self
            .limits
            .allows_operations(requirements.operation_count)
        {
            return Err(BackendError::OperationLimitExceeded {
                requested: requirements.operation_count,
                maximum: self.limits.max_operations,
            });
        }

        if !self.limits.allows_shots(requirements.shots) {
            return Err(BackendError::ShotLimitExceeded {
                requested: requirements.shots,
                maximum: self.limits.max_shots,
            });
        }

        self.validate_features(requirements)?;
        self.validate_gates(requirements)?;
        self.validate_qubits(requirements)?;
        self.validate_connections(requirements)?;

        Ok(())
    }

    fn validate_features(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        if requirements.requires_measurement
            && !self.capabilities.measurement
        {
            return Err(BackendError::MeasurementUnsupported);
        }

        if requirements.requires_reset && !self.capabilities.reset {
            return Err(BackendError::ResetUnsupported);
        }

        if requirements.requires_mid_circuit_measurement
            && !self.capabilities.mid_circuit_measurement
        {
            return Err(BackendError::MidCircuitMeasurementUnsupported);
        }

        if requirements.requires_classical_control
            && !self.capabilities.classical_control
        {
            return Err(BackendError::ClassicalControlUnsupported);
        }

        if requirements.requires_dynamic_circuits
            && !self.capabilities.dynamic_circuits
        {
            return Err(BackendError::DynamicCircuitUnsupported);
        }

        Ok(())
    }

    fn validate_gates(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        for gate in &requirements.gates {
            let normalized = normalize_gate_name(gate);

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

            return Err(BackendError::UnsupportedGate {
                gate: gate.clone(),
            });
        }

        Ok(())
    }

    fn validate_qubits(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        let topology_count = self.qubit_count();

        if topology_count == 0 {
            return Err(BackendError::EmptyTopology);
        }

        if requirements.qubit_count > topology_count {
            return Err(BackendError::QubitLimitExceeded {
                requested: requirements.qubit_count,
                maximum: topology_count,
            });
        }

        for &(control, target) in &requirements.two_qubit_edges {
            if control >= requirements.qubit_count {
                return Err(BackendError::InvalidQubit {
                    qubit: control,
                    qubit_count: requirements.qubit_count,
                });
            }

            if target >= requirements.qubit_count {
                return Err(BackendError::InvalidQubit {
                    qubit: target,
                    qubit_count: requirements.qubit_count,
                });
            }
        }

        Ok(())
    }

    fn validate_connections(
        &self,
        requirements: &CircuitRequirements,
    ) -> Result<(), BackendError> {
        for &(control, target) in &requirements.two_qubit_edges {
            if !topology_supports_connection(
                &self.topology,
                control,
                target,
            ) {
                return Err(BackendError::UnsupportedConnection {
                    control,
                    target,
                });
            }
        }

        Ok(())
    }

    /// Validate an execution request.
    pub fn validate_request(
        &self,
        request: &ExecutionRequest,
    ) -> Result<(), BackendError> {
        self.validate(&request.circuit)
    }

    /// Execution boundary.
    ///
    /// The hardware abstraction intentionally refuses to pretend that a
    /// physical execution provider exists. Concrete simulator/QPU adapters
    /// should implement actual execution outside this module.
    pub fn execute(
        &self,
        request: &ExecutionRequest,
    ) -> Result<ExecutionResult, BackendError> {
        self.validate_request(request)?;

        Err(BackendError::ExecutionUnavailable(format!(
            "backend '{}' has no execution adapter",
            self.metadata.id
        )))
    }

    /// Returns native gates in deterministic order.
    pub fn native_gates(&self) -> Vec<String> {
        self.capabilities
            .native_gates
            .iter()
            .cloned()
            .collect()
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn validate_backend_metadata(
    metadata: &BackendMetadata,
) -> Result<(), BackendError> {
    if metadata.id.trim().is_empty() {
        return Err(BackendError::InvalidBackendId);
    }

    Ok(())
}

/// Normalize gate names so backend validation is case-insensitive and
/// whitespace-independent.
fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_uppercase()
}

fn is_single_qubit_rotation(gate: &str) -> bool {
    matches!(
        gate,
        "RX" | "RY" | "RZ" | "U" | "U1" | "U2" | "U3"
    )
}

fn is_parameterized_gate(gate: &str) -> bool {
    matches!(
        gate,
        "RX" | "RY" | "RZ" | "U" | "U1" | "U2" | "U3"
    )
}

/// Extract the number of physical qubits from the repository topology.
///
/// The topology module is intentionally kept authoritative. This helper is
/// isolated so any future topology API change only affects this integration
/// point.
fn topology_qubit_count(topology: &HardwareTopology) -> usize {
    topology.qubits.len()
}

/// Determine whether a physical connection exists.
///
/// `HardwareTopology` remains the source of truth for connectivity.
fn topology_supports_connection(
    topology: &HardwareTopology,
    control: usize,
    target: usize,
) -> bool {
    if control == target {
        return false;
    }

    topology
        .connections
        .iter()
        .any(|connection| {
            (connection.control == control
                && connection.target == target)
                || (connection.control == target
                    && connection.target == control)
        })
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn topology() -> HardwareTopology {
        HardwareTopology::new(4)
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
                "H", "X", "Y", "Z", "S", "T", "CX", "CZ", "SWAP", "MEASURE",
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

        assert!(validate_backend_metadata(&metadata).is_err());
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

        let gates = capabilities.native_gates.iter()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            gates,
            vec![
                "CX".to_string(),
                "H".to_string(),
                "X".to_string(),
                "Z".to_string()
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

        assert!(backend.validate(&requirements).is_ok());
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
            backend.validate(&requirements),
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
        .unwrap();

        let requirements = CircuitRequirements {
            qubit_count: 3,
            ..Default::default()
        };

        assert!(matches!(
            backend.validate(&requirements),
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
            backend.validate(&requirements),
            Err(BackendError::BackendUnavailable { .. })
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

        let result = backend.validate(&requirements);

        assert!(matches!(
            result,
            Err(BackendError::UnsupportedConnection {
                control: 0,
                target: 3
            })
        ));
    }

    #[test]
    fn backend_execution_is_explicitly_unimplemented() {
        let backend = backend();

        let request = ExecutionRequest::new(CircuitRequirements {
            qubit_count: 1,
            gates: vec!["H".into()],
            ..Default::default()
        });

        let result = backend.execute(&request);

        assert!(matches!(
            result,
            Err(BackendError::ExecutionUnavailable(_))
        ));
    }

    #[test]
    fn status_available_is_usable() {
        assert!(BackendStatus::Available.is_usable());
        assert!(!BackendStatus::Offline.is_usable());
        assert!(!BackendStatus::Maintenance.is_usable());
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
        .unwrap();

        let requirements = CircuitRequirements {
            qubit_count: 1,
            gates: vec!["RX".into()],
            ..Default::default()
        };

        assert!(backend.validate(&requirements).is_ok());
    }
}