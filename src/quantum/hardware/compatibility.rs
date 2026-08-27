//! Zamani Quantum Hardware — Compatibility Analysis
//!
//! Production-grade, provider-neutral compatibility checking between a
//! quantum workload and a concrete hardware backend.
//!
//! # Responsibility
//!
//! This module answers one question before execution:
//!
//! > Can this workload execute on this backend as represented, and if not,
//! > exactly what must change before it can execute?
//!
//! It owns:
//!
//! - compatibility requirements;
//! - capability negotiation against the current `QuantumBackend` contract;
//! - deterministic diagnostics;
//! - severity classification;
//! - transformation requirements;
//! - calibration-policy hooks without owning calibration state;
//! - stable compatibility decisions suitable for compilers, schedulers,
//!   provider adapters, benchmarking, registries, and Danga.
//!
//! It does NOT own:
//!
//! - Quantum IR semantics;
//! - circuit optimization;
//! - transpilation/routing algorithms;
//! - scheduling;
//! - calibration acquisition;
//! - provider I/O;
//! - credentials/authentication;
//! - job submission;
//! - benchmark statistics;
//! - provider-specific APIs.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum IR
//!      |
//!      v
//! workload requirements
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! QuantumBackend       CalibrationView
//!      |                    |
//!      +---------+----------+
//!                v
//!        compatibility.rs
//!                |
//!       +--------+---------+
//!       |                  |
//!       v                  v
//! Compatible         TransformationPlan
//!       |                  |
//!       +--------+---------+
//!                v
//!      routing / scheduling
//!                |
//!                v
//!          provider adapter
//! ```
//!
//! Compatibility is deliberately downstream of the canonical Quantum IR and
//! upstream of routing, scheduling, execution, and provider adapters.
//!
//! Benchmarking consumes this module; this module never consumes benchmarking.
//!
//! # Integration
//!
//! The current hardware backend provides:
//!
//! - `BackendCapabilities`;
//! - `BackendLimits`;
//! - `BackendMetadata`;
//! - `BackendStatus`;
//! - `CircuitRequirements`;
//! - `QuantumBackend`;
//! - `HardwareTopology`.
//!
//! This module consumes those public contracts without modifying them.
//!
//! When `quantum::hardware::mod.rs` becomes authoritative, add:
//!
//! ```text
//! pub mod compatibility;
//! ```
//!
//! No implementation changes in this file are required.
//!
//! `CalibrationView` allows `calibration.rs` to integrate later by implementing
//! one small trait. This prevents compatibility from owning calibration state
//! or becoming coupled to its storage representation.
//!
//! # Determinism
//!
//! Compatibility analysis is deterministic:
//!
//! - requirements are validated before evaluation;
//! - diagnostics are sorted by stable severity/code/subject/message ordering;
//! - no `HashMap` iteration is exposed;
//! - no wall clock is read;
//! - no provider network calls are performed;
//! - calibration freshness is supplied as immutable evidence;
//! - identical inputs produce identical reports.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! No unsafe code is permitted.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

use super::backend::{
    BackendCapabilities,
    BackendKind,
    CircuitRequirements,
    QuantumBackend,
};
use super::topology::HardwareTopology;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier.
pub const COMPATIBILITY_SCHEMA_ID: &str =
    "zamani.quantum.hardware.compatibility";

/// Semantic schema version.
pub const COMPATIBILITY_SCHEMA_VERSION: u16 = 1;

/// Maximum number of individual workload requirements accepted.
pub const DEFAULT_MAX_REQUIREMENTS: usize = 4096;

/// Maximum number of diagnostics retained in a report.
pub const DEFAULT_MAX_DIAGNOSTICS: usize = 4096;

// =============================================================================
// Compatibility status
// =============================================================================

/// Final compatibility classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompatibilityStatus {
    /// The workload can execute without transformation or warnings.
    Compatible,

    /// The workload can execute, but there are non-fatal warnings.
    CompatibleWithWarnings,

    /// The workload can execute after a declared compiler transformation.
    RequiresTransformation,

    /// No safe supported execution path is currently known.
    Incompatible,
}

impl CompatibilityStatus {
    /// Returns true when direct execution is permitted.
    pub const fn is_direct(self) -> bool {
        matches!(
            self,
            Self::Compatible | Self::CompatibleWithWarnings
        )
    }

    /// Returns true when a transformation plan is required.
    pub const fn is_transformable(self) -> bool {
        matches!(self, Self::RequiresTransformation)
    }

    /// Returns true when the workload cannot safely execute.
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::CompatibleWithWarnings => "compatible_with_warnings",
            Self::RequiresTransformation => "requires_transformation",
            Self::Incompatible => "incompatible",
        }
    }
}

impl fmt::Display for CompatibilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Diagnostic severity
// =============================================================================

/// Severity of a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// Informational evidence.
    Info,

    /// Does not prevent execution.
    Warning,

    /// Requires a transformation.
    Error,

    /// Execution must not proceed.
    Fatal,
}

impl DiagnosticSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }

    const fn rank(self) -> u8 {
        match self {
            Self::Fatal => 0,
            Self::Error => 1,
            Self::Warning => 2,
            Self::Info => 3,
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Diagnostic codes
// =============================================================================

/// Stable machine-readable compatibility code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompatibilityCode {
    BackendUnavailable,
    BackendKindMismatch,
    ZeroQubits,
    QubitLimitExceeded,
    TopologyCapacityExceeded,
    CircuitDepthExceeded,
    OperationLimitExceeded,
    ShotLimitExceeded,
    MeasurementUnsupported,
    ResetUnsupported,
    MidCircuitMeasurementUnsupported,
    ClassicalControlUnsupported,
    DynamicCircuitUnsupported,
    UnsupportedGate,
    ParameterizedGateUnsupported,
    ArbitraryRotationUnsupported,
    UnsupportedConnection,
    InvalidLogicalQubit,
    InvalidPhysicalQubit,
    PulseExecutionUnsupported,
    AnalogExecutionUnsupported,
    AnnealingExecutionUnsupported,
    LogicalExecutionUnsupported,
    StateVectorAccessUnsupported,
    DensityMatrixAccessUnsupported,
    ExpectationValueUnsupported,
    SeedUnsupported,
    CalibrationRequired,
    CalibrationUnavailable,
    CalibrationStale,
    CalibrationUnknown,
    ExperimentalCapability,
    TransformationRequired,
    RoutingRequired,
    GateDecompositionRequired,
    ParameterBindingRequired,
    ShotsDefaulted,
    EmptyWorkload,
    RequirementLimitExceeded,
    InvalidRequirement,
    TopologyInvalid,
}

impl CompatibilityCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendUnavailable => "backend_unavailable",
            Self::BackendKindMismatch => "backend_kind_mismatch",
            Self::ZeroQubits => "zero_qubits",
            Self::QubitLimitExceeded => "qubit_limit_exceeded",
            Self::TopologyCapacityExceeded => "topology_capacity_exceeded",
            Self::CircuitDepthExceeded => "circuit_depth_exceeded",
            Self::OperationLimitExceeded => "operation_limit_exceeded",
            Self::ShotLimitExceeded => "shot_limit_exceeded",
            Self::MeasurementUnsupported => "measurement_unsupported",
            Self::ResetUnsupported => "reset_unsupported",
            Self::MidCircuitMeasurementUnsupported =>
                "mid_circuit_measurement_unsupported",
            Self::ClassicalControlUnsupported =>
                "classical_control_unsupported",
            Self::DynamicCircuitUnsupported =>
                "dynamic_circuit_unsupported",
            Self::UnsupportedGate => "unsupported_gate",
            Self::ParameterizedGateUnsupported =>
                "parameterized_gate_unsupported",
            Self::ArbitraryRotationUnsupported =>
                "arbitrary_rotation_unsupported",
            Self::UnsupportedConnection => "unsupported_connection",
            Self::InvalidLogicalQubit => "invalid_logical_qubit",
            Self::InvalidPhysicalQubit => "invalid_physical_qubit",
            Self::PulseExecutionUnsupported =>
                "pulse_execution_unsupported",
            Self::AnalogExecutionUnsupported =>
                "analog_execution_unsupported",
            Self::AnnealingExecutionUnsupported =>
                "annealing_execution_unsupported",
            Self::LogicalExecutionUnsupported =>
                "logical_execution_unsupported",
            Self::StateVectorAccessUnsupported =>
                "state_vector_access_unsupported",
            Self::DensityMatrixAccessUnsupported =>
                "density_matrix_access_unsupported",
            Self::ExpectationValueUnsupported =>
                "expectation_value_unsupported",
            Self::SeedUnsupported => "seed_unsupported",
            Self::CalibrationRequired => "calibration_required",
            Self::CalibrationUnavailable => "calibration_unavailable",
            Self::CalibrationStale => "calibration_stale",
            Self::CalibrationUnknown => "calibration_unknown",
            Self::ExperimentalCapability =>
                "experimental_capability",
            Self::TransformationRequired =>
                "transformation_required",
            Self::RoutingRequired => "routing_required",
            Self::GateDecompositionRequired =>
                "gate_decomposition_required",
            Self::ParameterBindingRequired =>
                "parameter_binding_required",
            Self::ShotsDefaulted => "shots_defaulted",
            Self::EmptyWorkload => "empty_workload",
            Self::RequirementLimitExceeded =>
                "requirement_limit_exceeded",
            Self::InvalidRequirement => "invalid_requirement",
            Self::TopologyInvalid => "topology_invalid",
        }
    }
}

impl fmt::Display for CompatibilityCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Transformation requirements
// =============================================================================

/// Transformation required from a downstream compiler stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationRequirement {
    /// Decompose unsupported operations.
    GateDecomposition,

    /// Bind parameters before provider submission.
    ParameterBinding,

    /// Map logical interactions onto physical topology.
    Routing,

    /// Apply hardware timing constraints.
    Scheduling,

    /// Lower to a provider/native representation.
    NativeLowering,
}

impl TransformationRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateDecomposition => "gate_decomposition",
            Self::ParameterBinding => "parameter_binding",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::NativeLowering => "native_lowering",
        }
    }
}

impl fmt::Display for TransformationRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Workload kind
// =============================================================================

/// Kind of workload presented to hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WorkloadKind {
    Circuit,
    DynamicCircuit,
    Pulse,
    Analog,
    Annealing,
    Logical,
    Generic,
}

impl WorkloadKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circuit => "circuit",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Generic => "generic",
        }
    }
}

// =============================================================================
// Execution model
// =============================================================================

/// Execution model required by the workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionModel {
    GateModel,
    DynamicCircuit,
    Pulse,
    Analog,
    Annealing,
    Logical,
    Simulator,
    Emulator,
    Custom,
}

impl ExecutionModel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Calibration integration contract
// =============================================================================

/// Immutable calibration freshness evidence.
///
/// The compatibility layer does not read the system clock. The calibration
/// subsystem supplies this evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationFreshness {
    /// Age of the relevant calibration in nanoseconds.
    pub age_ns: u64,

    /// Maximum age accepted by the workload policy.
    pub maximum_age_ns: u64,

    /// Whether the calibration owner explicitly marked the evidence usable.
    pub usable: bool,
}

impl CalibrationFreshness {
    pub const fn fresh(
        age_ns: u64,
        maximum_age_ns: u64,
    ) -> Self {
        Self {
            age_ns,
            maximum_age_ns,
            usable: true,
        }
    }

    pub const fn stale(
        age_ns: u64,
        maximum_age_ns: u64,
    ) -> Self {
        Self {
            age_ns,
            maximum_age_ns,
            usable: false,
        }
    }

    pub const fn is_fresh(self) -> bool {
        self.usable && self.age_ns <= self.maximum_age_ns
    }
}

/// Read-only calibration contract.
///
/// `calibration.rs` can implement this trait later without modifying
/// `compatibility.rs`.
pub trait CalibrationView {
    fn freshness(&self) -> Option<CalibrationFreshness>;
}

// =============================================================================
// Requirements
// =============================================================================

/// Provider-neutral compatibility requirements.
///
/// This intentionally extends the current `CircuitRequirements` so future
/// pulse, analog, annealing, logical, simulator, emulator, and distributed
/// hardware models can be integrated without redesigning compatibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityRequirements {
    pub workload_kind: WorkloadKind,
    pub execution_model: ExecutionModel,

    pub qubit_count: usize,
    pub circuit_depth: usize,
    pub operation_count: usize,

    /// Zero means unspecified and must be defaulted by execution policy.
    pub shots: usize,

    /// Normalized gate/instruction identifiers.
    pub instructions: Vec<String>,

    /// Logical interaction pairs.
    pub two_resource_edges: Vec<(usize, usize)>,

    pub requires_measurement: bool,
    pub requires_reset: bool,
    pub requires_mid_circuit_measurement: bool,
    pub requires_classical_control: bool,
    pub requires_dynamic_circuits: bool,

    pub requires_arbitrary_rotations: bool,
    pub requires_parameterized_submission: bool,

    pub requires_state_vector: bool,
    pub requires_density_matrix: bool,
    pub requires_expectation_values: bool,

    pub requires_seed: bool,
    pub requires_calibration: bool,

    pub allow_routing: bool,
    pub allow_gate_decomposition: bool,
    pub allow_parameter_binding: bool,

    pub max_diagnostics: usize,
}

impl Default for CompatibilityRequirements {
    fn default() -> Self {
        Self {
            workload_kind: WorkloadKind::Circuit,
            execution_model: ExecutionModel::GateModel,
            qubit_count: 0,
            circuit_depth: 0,
            operation_count: 0,
            shots: 0,
            instructions: Vec::new(),
            two_resource_edges: Vec::new(),
            requires_measurement: false,
            requires_reset: false,
            requires_mid_circuit_measurement: false,
            requires_classical_control: false,
            requires_dynamic_circuits: false,
            requires_arbitrary_rotations: false,
            requires_parameterized_submission: false,
            requires_state_vector: false,
            requires_density_matrix: false,
            requires_expectation_values: false,
            requires_seed: false,
            requires_calibration: false,
            allow_routing: true,
            allow_gate_decomposition: true,
            allow_parameter_binding: true,
            max_diagnostics: DEFAULT_MAX_DIAGNOSTICS,
        }
    }
}

impl CompatibilityRequirements {
    /// Converts the repository's current circuit requirement contract into the
    /// production compatibility contract.
    pub fn from_circuit(
        circuit: &CircuitRequirements,
    ) -> Self {
        Self {
            workload_kind: if circuit.requires_dynamic_circuits {
                WorkloadKind::DynamicCircuit
            } else {
                WorkloadKind::Circuit
            },

            execution_model: if circuit.requires_dynamic_circuits {
                ExecutionModel::DynamicCircuit
            } else {
                ExecutionModel::GateModel
            },

            qubit_count: circuit.qubit_count,
            circuit_depth: circuit.circuit_depth,
            operation_count: circuit.operation_count,
            shots: circuit.shots,
            instructions: circuit.gates.clone(),
            two_resource_edges: circuit.two_qubit_edges.clone(),

            requires_measurement: circuit.requires_measurement,
            requires_reset: circuit.requires_reset,
            requires_mid_circuit_measurement:
                circuit.requires_mid_circuit_measurement,
            requires_classical_control:
                circuit.requires_classical_control,
            requires_dynamic_circuits:
                circuit.requires_dynamic_circuits,

            ..Self::default()
        }
    }

    /// Whether the request contains no executable requirements.
    pub fn is_empty(&self) -> bool {
        self.qubit_count == 0
            && self.operation_count == 0
            && self.instructions.is_empty()
            && self.two_resource_edges.is_empty()
            && !self.requires_measurement
            && !self.requires_reset
            && !self.requires_state_vector
            && !self.requires_density_matrix
            && !self.requires_expectation_values
    }

    /// Validate requirement-internal invariants.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if self.max_diagnostics == 0 {
            return Err(
                CompatibilityError::InvalidRequirement(
                    "max_diagnostics must be greater than zero"
                        .to_owned(),
                ),
            );
        }

        for &(source, target) in &self.two_resource_edges {
            if source == target {
                return Err(
                    CompatibilityError::InvalidRequirement(
                        format!(
                            "self-interaction ({source}, {target}) is invalid"
                        ),
                    ),
                );
            }

            if source >= self.qubit_count
                || target >= self.qubit_count
            {
                return Err(
                    CompatibilityError::InvalidRequirement(
                        format!(
                            "interaction ({source}, {target}) references a resource outside 0..{}",
                            self.qubit_count.saturating_sub(1)
                        ),
                    ),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Diagnostic
// =============================================================================

/// One deterministic compatibility finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityDiagnostic {
    pub severity: DiagnosticSeverity,
    pub code: CompatibilityCode,
    pub subject: String,
    pub message: String,
    pub remediation: Option<String>,
    pub transformation: Option<TransformationRequirement>,
}

impl CompatibilityDiagnostic {
    fn new(
        severity: DiagnosticSeverity,
        code: CompatibilityCode,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            subject: subject.into(),
            message: message.into(),
            remediation: None,
            transformation: None,
        }
    }

    fn with_remediation(
        mut self,
        remediation: impl Into<String>,
    ) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    fn with_transformation(
        mut self,
        transformation: TransformationRequirement,
    ) -> Self {
        self.transformation = Some(transformation);
        self
    }
}

impl Ord for CompatibilityDiagnostic {
    fn cmp(&self, other: &Self) -> Ordering {
        self.severity
            .rank()
            .cmp(&other.severity.rank())
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.subject.cmp(&other.subject))
            .then_with(|| self.message.cmp(&other.message))
    }
}

impl PartialOrd for CompatibilityDiagnostic {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Analysis errors
// =============================================================================

/// Errors indicating that compatibility analysis itself could not be
/// completed.
///
/// A normal hardware incompatibility is NOT represented as an error; it is
/// represented by `CompatibilityStatus::Incompatible`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    InvalidRequirement(String),

    RequirementLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    DiagnosticLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    Topology(String),
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequirement(message) => {
                write!(
                    f,
                    "invalid compatibility requirement: {message}"
                )
            }

            Self::RequirementLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "compatibility request contains {requested} \
                     requirements; maximum is {maximum}"
                )
            }

            Self::DiagnosticLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "compatibility analysis produced {requested} \
                     diagnostics; maximum is {maximum}"
                )
            }

            Self::Topology(message) => {
                write!(
                    f,
                    "compatibility topology error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CompatibilityError {}

// =============================================================================
// Transformation plan
// =============================================================================

/// Deterministic downstream transformation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationPlan {
    requirements: Vec<TransformationRequirement>,
}

impl TransformationPlan {
    fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    fn insert(
        &mut self,
        requirement: TransformationRequirement,
    ) {
        if !self.requirements.contains(&requirement) {
            self.requirements.push(requirement);
            self.requirements.sort_unstable();
        }
    }

    /// Required transformations in deterministic order.
    pub fn requirements(
        &self,
    ) -> &[TransformationRequirement] {
        &self.requirements
    }

    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    pub fn contains(
        &self,
        requirement: TransformationRequirement,
    ) -> bool {
        self.requirements.contains(&requirement)
    }
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Complete compatibility result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub schema_id: &'static str,
    pub schema_version: u16,

    pub backend_id: String,
    pub backend_kind: BackendKind,
    pub workload_kind: WorkloadKind,

    pub status: CompatibilityStatus,

    pub diagnostics: Vec<CompatibilityDiagnostic>,

    pub transformation_plan: TransformationPlan,
}

impl CompatibilityReport {
    pub fn is_compatible(&self) -> bool {
        matches!(
            self.status,
            CompatibilityStatus::Compatible
                | CompatibilityStatus::CompatibleWithWarnings
        )
    }

    pub fn is_directly_executable(&self) -> bool {
        self.status.is_direct()
    }

    pub fn requires_transformation(&self) -> bool {
        self.status.is_transformable()
    }

    pub fn is_incompatible(&self) -> bool {
        self.status.is_incompatible()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.severity,
                DiagnosticSeverity::Error
                    | DiagnosticSeverity::Fatal
            )
        })
    }

    pub fn has_fatal_errors(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Fatal
        })
    }

    pub fn diagnostics_by_code(
        &self,
        code: CompatibilityCode,
    ) -> impl Iterator<Item = &CompatibilityDiagnostic> {
        self.diagnostics
            .iter()
            .filter(move |diagnostic| diagnostic.code == code)
    }
}

// =============================================================================
// Policy
// =============================================================================

/// Policy controlling compatibility analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityPolicy {
    pub reject_empty_workloads: bool,
    pub allow_routing: bool,
    pub allow_gate_decomposition: bool,
    pub allow_parameter_binding: bool,

    /// If true, missing calibration evidence is fatal whenever calibration is
    /// required.
    pub unknown_calibration_is_fatal: bool,

    pub warn_on_experimental_capabilities: bool,

    pub max_requirements: usize,
    pub max_diagnostics: usize,
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self {
            reject_empty_workloads: true,
            allow_routing: true,
            allow_gate_decomposition: true,
            allow_parameter_binding: true,
            unknown_calibration_is_fatal: true,
            warn_on_experimental_capabilities: true,
            max_requirements: DEFAULT_MAX_REQUIREMENTS,
            max_diagnostics: DEFAULT_MAX_DIAGNOSTICS,
        }
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Stateless deterministic compatibility analyzer.
#[derive(Debug, Clone, Copy)]
pub struct CompatibilityAnalyzer {
    policy: CompatibilityPolicy,
}

impl CompatibilityAnalyzer {
    /// Creates the production-safe default analyzer.
    pub const fn new() -> Self {
        Self {
            policy: CompatibilityPolicy {
                reject_empty_workloads: true,
                allow_routing: true,
                allow_gate_decomposition: true,
                allow_parameter_binding: true,
                unknown_calibration_is_fatal: true,
                warn_on_experimental_capabilities: true,
                max_requirements: DEFAULT_MAX_REQUIREMENTS,
                max_diagnostics: DEFAULT_MAX_DIAGNOSTICS,
            },
        }
    }

    pub const fn with_policy(
        policy: CompatibilityPolicy,
    ) -> Self {
        Self { policy }
    }

    pub const fn policy(
        self,
    ) -> CompatibilityPolicy {
        self.policy
    }

    /// Analyze a workload without calibration evidence.
    pub fn analyze(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
    ) -> Result<CompatibilityReport, CompatibilityError> {
        self.analyze_with_calibration::<NoCalibration>(
            backend,
            requirements,
            None,
        )
    }

    /// Analyze a workload with optional calibration evidence.
    pub fn analyze_with_calibration<C: CalibrationView>(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        calibration: Option<&C>,
    ) -> Result<CompatibilityReport, CompatibilityError> {
        requirements.validate()?;

        let requirement_count = requirements
            .instructions
            .len()
            .saturating_add(
                requirements.two_resource_edges.len(),
            );

        if requirement_count > self.policy.max_requirements {
            return Err(
                CompatibilityError::RequirementLimitExceeded {
                    requested: requirement_count,
                    maximum: self.policy.max_requirements,
                },
            );
        }

        let mut diagnostics = Vec::new();
        let mut plan = TransformationPlan::new();

        self.check_backend_state(
            backend,
            &mut diagnostics,
        );

        self.check_execution_model(
            backend,
            requirements,
            &mut diagnostics,
        );

        self.check_resource_limits(
            backend,
            requirements,
            &mut diagnostics,
        );

        self.check_features(
            backend,
            requirements,
            &mut diagnostics,
        );

        self.check_instructions(
            backend,
            requirements,
            &mut diagnostics,
            &mut plan,
        );

        self.check_connectivity(
            backend,
            requirements,
            &mut diagnostics,
            &mut plan,
        )?;

        self.check_optional_capabilities(
            backend,
            requirements,
            &mut diagnostics,
        );

        self.check_calibration(
            requirements,
            calibration,
            &mut diagnostics,
        );

        self.check_empty_workload(
            requirements,
            &mut diagnostics,
        );

        diagnostics.sort_unstable();

        let max_diagnostics = requirements
            .max_diagnostics
            .min(self.policy.max_diagnostics);

        if diagnostics.len() > max_diagnostics {
            return Err(
                CompatibilityError::DiagnosticLimitExceeded {
                    requested: diagnostics.len(),
                    maximum: max_diagnostics,
                },
            );
        }

        let status = classify(
            &diagnostics,
            &plan,
        );

        Ok(CompatibilityReport {
            schema_id: COMPATIBILITY_SCHEMA_ID,
            schema_version: COMPATIBILITY_SCHEMA_VERSION,
            backend_id: backend.id().to_owned(),
            backend_kind: backend.kind(),
            workload_kind: requirements.workload_kind,
            status,
            diagnostics,
            transformation_plan: plan,
        })
    }

    fn check_backend_state(
        &self,
        backend: &QuantumBackend,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        if backend.metadata.id.trim().is_empty() {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::BackendUnavailable,
                    "backend.id",
                    "backend has no stable identifier",
                ),
            );
        }

        if !backend.is_available() {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::BackendUnavailable,
                    "backend.status",
                    format!(
                        "backend '{}' is not available ({:?})",
                        backend.id(),
                        backend.metadata.status
                    ),
                )
                .with_remediation(
                    "wait for the backend to become available or \
                     select another backend",
                ),
            );
        }
    }

    fn check_execution_model(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        let compatible = match requirements.execution_model {
            ExecutionModel::Simulator => {
                backend.kind() == BackendKind::Simulator
            }

            ExecutionModel::Emulator => {
                backend.kind() == BackendKind::Emulator
            }

            ExecutionModel::GateModel => true,

            ExecutionModel::DynamicCircuit => {
                backend.capabilities.dynamic_circuits
            }

            // These capabilities are intentionally not guessed from the
            // current backend contract.
            ExecutionModel::Pulse
            | ExecutionModel::Analog
            | ExecutionModel::Annealing
            | ExecutionModel::Logical => false,

            ExecutionModel::Custom => {
                backend.kind() == BackendKind::Custom
            }
        };

        if compatible {
            return;
        }

        let (
            code,
            subject,
            message,
        ) = match requirements.execution_model {
            ExecutionModel::Pulse => (
                CompatibilityCode::PulseExecutionUnsupported,
                "execution_model",
                "pulse execution is not represented by the \
                 current backend capability contract",
            ),

            ExecutionModel::Analog => (
                CompatibilityCode::AnalogExecutionUnsupported,
                "execution_model",
                "analog execution is not represented by the \
                 current backend capability contract",
            ),

            ExecutionModel::Annealing => (
                CompatibilityCode::AnnealingExecutionUnsupported,
                "execution_model",
                "annealing execution is not represented by the \
                 current backend capability contract",
            ),

            ExecutionModel::Logical => (
                CompatibilityCode::LogicalExecutionUnsupported,
                "execution_model",
                "logical execution is not represented by the \
                 current backend capability contract",
            ),

            _ => (
                CompatibilityCode::BackendKindMismatch,
                "execution_model",
                "backend kind and requested execution model \
                 are incompatible",
            ),
        };

        diagnostics.push(
            CompatibilityDiagnostic::new(
                DiagnosticSeverity::Fatal,
                code,
                subject,
                message,
            )
            .with_remediation(
                "select a backend exposing the required execution \
                 model or lower the workload to a supported model",
            ),
        );
    }

    fn check_resource_limits(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        if requirements.qubit_count == 0 {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::ZeroQubits,
                    "qubit_count",
                    "workload requires zero quantum resources",
                )
                .with_remediation(
                    "construct a non-empty quantum workload",
                ),
            );
        }

        if backend.limits.max_qubits != 0
            && requirements.qubit_count
                > backend.limits.max_qubits
        {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::QubitLimitExceeded,
                    "qubit_count",
                    format!(
                        "workload requires {} qubits but backend \
                         limit is {}",
                        requirements.qubit_count,
                        backend.limits.max_qubits
                    ),
                )
                .with_remediation(
                    "select a backend with sufficient resources",
                ),
            );
        }

        let topology_count =
            backend.topology.qubit_count();

        if requirements.qubit_count > topology_count {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::TopologyCapacityExceeded,
                    "topology",
                    format!(
                        "workload requires {} resources but topology \
                         exposes {}",
                        requirements.qubit_count,
                        topology_count
                    ),
                )
                .with_remediation(
                    "select a backend with a topology containing \
                     enough physical resources",
                ),
            );
        }

        if backend.limits.max_circuit_depth != 0
            && requirements.circuit_depth
                > backend.limits.max_circuit_depth
        {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::CircuitDepthExceeded,
                    "circuit_depth",
                    format!(
                        "workload depth {} exceeds backend limit {}",
                        requirements.circuit_depth,
                        backend.limits.max_circuit_depth
                    ),
                )
                .with_remediation(
                    "optimize or decompose the circuit, or select \
                     a backend with a larger depth limit",
                ),
            );
        }

        if backend.limits.max_operations != 0
            && requirements.operation_count
                > backend.limits.max_operations
        {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::OperationLimitExceeded,
                    "operation_count",
                    format!(
                        "workload contains {} operations but backend \
                         limit is {}",
                        requirements.operation_count,
                        backend.limits.max_operations
                    ),
                )
                .with_remediation(
                    "reduce operations or select a backend with \
                     a larger operation limit",
                ),
            );
        }

        if requirements.shots == 0 {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Info,
                    CompatibilityCode::ShotsDefaulted,
                    "shots",
                    "workload did not specify a shot count",
                )
                .with_remediation(
                    "execution must apply an explicit shot default \
                     before submission",
                ),
            );
        } else if backend.limits.max_shots != 0
            && requirements.shots > backend.limits.max_shots
        {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::ShotLimitExceeded,
                    "shots",
                    format!(
                        "workload requests {} shots but backend \
                         limit is {}",
                        requirements.shots,
                        backend.limits.max_shots
                    ),
                )
                .with_remediation(
                    "reduce shots or select another backend",
                ),
            );
        }
    }

    fn check_features(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        check_feature(
            requirements.requires_measurement,
            backend.capabilities.measurement,
            CompatibilityCode::MeasurementUnsupported,
            "measurement",
            "workload requires measurement but backend \
             does not expose it",
            "select a backend with measurement support",
            diagnostics,
        );

        check_feature(
            requirements.requires_reset,
            backend.capabilities.reset,
            CompatibilityCode::ResetUnsupported,
            "reset",
            "workload requires reset but backend does not \
             expose it",
            "select a backend with reset support",
            diagnostics,
        );

        check_feature(
            requirements.requires_mid_circuit_measurement,
            backend.capabilities.mid_circuit_measurement,
            CompatibilityCode::MidCircuitMeasurementUnsupported,
            "mid_circuit_measurement",
            "workload requires mid-circuit measurement but \
             backend does not expose it",
            "select a backend supporting mid-circuit measurement",
            diagnostics,
        );

        check_feature(
            requirements.requires_classical_control,
            backend.capabilities.classical_control,
            CompatibilityCode::ClassicalControlUnsupported,
            "classical_control",
            "workload requires classical control but backend \
             does not expose it",
            "select a backend supporting classical feed-forward",
            diagnostics,
        );

        check_feature(
            requirements.requires_dynamic_circuits,
            backend.capabilities.dynamic_circuits,
            CompatibilityCode::DynamicCircuitUnsupported,
            "dynamic_circuits",
            "workload requires dynamic circuits but backend \
             does not expose them",
            "select a dynamic-circuit backend",
            diagnostics,
        );
    }

    fn check_instructions(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
        plan: &mut TransformationPlan,
    ) {
        let mut seen = BTreeSet::new();

        for instruction in &requirements.instructions {
            let normalized =
                normalize_instruction(instruction);

            if normalized.is_empty() {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::InvalidRequirement,
                        "instruction",
                        "workload contains an empty instruction identifier",
                    ),
                );
                continue;
            }

            if !seen.insert(normalized.clone()) {
                continue;
            }

            if backend
                .capabilities
                .supports_gate(&normalized)
            {
                continue;
            }

            if is_rotation_instruction(&normalized) {
                if backend
                    .capabilities
                    .arbitrary_single_qubit_rotations
                {
                    continue;
                }

                if self.policy.allow_gate_decomposition
                    && requirements.allow_gate_decomposition
                {
                    plan.insert(
                        TransformationRequirement::GateDecomposition,
                    );

                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Error,
                            CompatibilityCode::ArbitraryRotationUnsupported,
                            format!("instruction:{normalized}"),
                            format!(
                                "instruction `{normalized}` requires \
                                 arbitrary single-qubit rotation support"
                            ),
                        )
                        .with_remediation(
                            "decompose the rotation into native gates",
                        )
                        .with_transformation(
                            TransformationRequirement::GateDecomposition,
                        ),
                    );
                } else {
                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Fatal,
                            CompatibilityCode::ArbitraryRotationUnsupported,
                            format!("instruction:{normalized}"),
                            format!(
                                "instruction `{normalized}` requires \
                                 arbitrary rotation support"
                            ),
                        )
                        .with_remediation(
                            "select a backend supporting arbitrary \
                             rotations",
                        ),
                    );
                }

                continue;
            }

            if looks_parameterized(&normalized) {
                if backend.capabilities.parameterized_gates {
                    continue;
                }

                if self.policy.allow_parameter_binding
                    && requirements.allow_parameter_binding
                {
                    plan.insert(
                        TransformationRequirement::ParameterBinding,
                    );

                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Error,
                            CompatibilityCode::ParameterizedGateUnsupported,
                            format!("instruction:{normalized}"),
                            format!(
                                "instruction `{normalized}` is \
                                 parameterized but backend does not \
                                 support unbound parameters"
                            ),
                        )
                        .with_remediation(
                            "bind all parameters before submission",
                        )
                        .with_transformation(
                            TransformationRequirement::ParameterBinding,
                        ),
                    );
                } else {
                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Fatal,
                            CompatibilityCode::ParameterizedGateUnsupported,
                            format!("instruction:{normalized}"),
                            format!(
                                "instruction `{normalized}` is \
                                 parameterized and backend does not \
                                 support unbound parameters"
                            ),
                        )
                        .with_remediation(
                            "bind parameters or select a backend \
                             supporting parameterized gates",
                        ),
                    );
                }

                continue;
            }

            if self.policy.allow_gate_decomposition
                && requirements.allow_gate_decomposition
            {
                plan.insert(
                    TransformationRequirement::GateDecomposition,
                );

                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Error,
                        CompatibilityCode::UnsupportedGate,
                        format!("instruction:{normalized}"),
                        format!(
                            "instruction `{normalized}` is not in the \
                             backend native instruction set"
                        ),
                    )
                    .with_remediation(
                        "decompose the instruction into native \
                         instructions",
                    )
                    .with_transformation(
                        TransformationRequirement::GateDecomposition,
                    ),
                );
            } else {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::UnsupportedGate,
                        format!("instruction:{normalized}"),
                        format!(
                            "instruction `{normalized}` is not supported \
                             by the backend"
                        ),
                    )
                    .with_remediation(
                        "select a backend supporting the instruction \
                         or enable a valid decomposition pass",
                    ),
                );
            }
        }

        if requirements.requires_parameterized_submission
            && !backend.capabilities.parameterized_gates
            && self.policy.allow_parameter_binding
            && requirements.allow_parameter_binding
        {
            plan.insert(
                TransformationRequirement::ParameterBinding,
            );
        }
    }

    fn check_connectivity(
        &self,
        backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
        plan: &mut TransformationPlan,
    ) -> Result<(), CompatibilityError> {
        if requirements.two_resource_edges.is_empty() {
            return Ok(());
        }

        if backend.topology.qubit_count() == 0 {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::TopologyInvalid,
                    "topology",
                    "backend topology exposes no quantum resources",
                ),
            );

            return Ok(());
        }

        for &(source, target) in
            &requirements.two_resource_edges
        {
            if source >= requirements.qubit_count
                || target >= requirements.qubit_count
            {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::InvalidLogicalQubit,
                        format!("interaction:{source}->{target}"),
                        format!(
                            "interaction references a logical resource \
                             outside workload range 0..{}",
                            requirements
                                .qubit_count
                                .saturating_sub(1)
                        ),
                    ),
                );

                continue;
            }

            let connected = backend
                .topology
                .is_connected(source, target)
                .map_err(|error| {
                    CompatibilityError::Topology(
                        error.to_string()
                    )
                })?;

            if connected {
                continue;
            }

            let physically_adjacent = backend
                .topology
                .is_physically_adjacent(
                    source,
                    target,
                )
                .map_err(|error| {
                    CompatibilityError::Topology(
                        error.to_string()
                    )
                })?;

            if physically_adjacent {
                plan.insert(
                    TransformationRequirement::Routing,
                );

                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Error,
                        CompatibilityCode::UnsupportedConnection,
                        format!("interaction:{source}->{target}"),
                        format!(
                            "resources {source} and {target} are physically \
                             adjacent but the requested native direction \
                             is unavailable"
                        ),
                    )
                    .with_remediation(
                        "use a direction-aware decomposition or route \
                         through supported couplings",
                    )
                    .with_transformation(
                        TransformationRequirement::Routing,
                    ),
                );

                continue;
            }

            let routable = backend
                .topology
                .shortest_path(source, target)
                .is_ok();

            if routable {
                if self.policy.allow_routing
                    && requirements.allow_routing
                {
                    plan.insert(
                        TransformationRequirement::Routing,
                    );

                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Error,
                            CompatibilityCode::RoutingRequired,
                            format!("interaction:{source}->{target}"),
                            format!(
                                "resources {source} and {target} are not \
                                 directly connected; routing is required"
                            ),
                        )
                        .with_remediation(
                            "run hardware-aware routing before scheduling",
                        )
                        .with_transformation(
                            TransformationRequirement::Routing,
                        ),
                    );
                } else {
                    diagnostics.push(
                        CompatibilityDiagnostic::new(
                            DiagnosticSeverity::Fatal,
                            CompatibilityCode::UnsupportedConnection,
                            format!("interaction:{source}->{target}"),
                            format!(
                                "resources {source} and {target} are not \
                                 directly connected and routing is disabled"
                            ),
                        )
                        .with_remediation(
                            "enable hardware-aware routing or select a \
                             backend with the required connectivity",
                        ),
                    );
                }
            } else {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::UnsupportedConnection,
                        format!("interaction:{source}->{target}"),
                        format!(
                            "no native hardware path exists between \
                             resources {source} and {target}"
                        ),
                    )
                    .with_remediation(
                        "select a connected backend or remap logical \
                         resources to a connected physical subgraph",
                    ),
                );
            }
        }

        Ok(())
    }

    fn check_optional_capabilities(
        &self,
        _backend: &QuantumBackend,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        // These are intentionally fatal until the expanded capabilities
        // contract exists. Compatibility must never guess that a provider
        // supports an operation it has not advertised.

        if requirements.requires_state_vector {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::StateVectorAccessUnsupported,
                    "state_vector",
                    "the current backend capability contract does not \
                     expose state-vector access",
                )
                .with_remediation(
                    "select a simulator/backend with explicit \
                     state-vector support",
                ),
            );
        }

        if requirements.requires_density_matrix {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::DensityMatrixAccessUnsupported,
                    "density_matrix",
                    "the current backend capability contract does not \
                     expose density-matrix access",
                )
                .with_remediation(
                    "select a simulator/backend with explicit \
                     density-matrix support",
                ),
            );
        }

        if requirements.requires_expectation_values {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::ExpectationValueUnsupported,
                    "expectation_values",
                    "the current backend capability contract does not \
                     expose expectation-value execution",
                )
                .with_remediation(
                    "select a backend exposing observable execution",
                ),
            );
        }

        if requirements.requires_seed {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Warning,
                    CompatibilityCode::SeedUnsupported,
                    "seed",
                    "the current backend capability contract does not \
                     advertise deterministic seed support",
                )
                .with_remediation(
                    "provider adapters must explicitly establish seed \
                     semantics before relying on deterministic sampling",
                ),
            );
        }
    }

    fn check_calibration<C: CalibrationView>(
        &self,
        requirements: &CompatibilityRequirements,
        calibration: Option<&C>,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        if !requirements.requires_calibration {
            return;
        }

        let freshness =
            calibration.and_then(CalibrationView::freshness);

        match freshness {
            Some(evidence) if evidence.is_fresh() => {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Info,
                        CompatibilityCode::CalibrationRequired,
                        "calibration",
                        "required calibration evidence is fresh \
                         and usable",
                    ),
                );
            }

            Some(evidence) => {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::CalibrationStale,
                        "calibration",
                        format!(
                            "required calibration is stale or unusable: \
                             age={} ns; maximum={} ns",
                            evidence.age_ns,
                            evidence.maximum_age_ns
                        ),
                    )
                    .with_remediation(
                        "refresh calibration and re-run compatibility \
                         analysis before submission",
                    ),
                );
            }

            None if self.policy.unknown_calibration_is_fatal => {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Fatal,
                        CompatibilityCode::CalibrationUnavailable,
                        "calibration",
                        "required calibration evidence was not supplied",
                    )
                    .with_remediation(
                        "obtain a calibration snapshot and provide \
                         freshness evidence before execution",
                    ),
                );
            }

            None => {
                diagnostics.push(
                    CompatibilityDiagnostic::new(
                        DiagnosticSeverity::Warning,
                        CompatibilityCode::CalibrationUnknown,
                        "calibration",
                        "calibration was required but freshness could \
                         not be established",
                    )
                    .with_remediation(
                        "obtain explicit calibration freshness evidence \
                         before physical execution",
                    ),
                );
            }
        }
    }

    fn check_empty_workload(
        &self,
        requirements: &CompatibilityRequirements,
        diagnostics: &mut Vec<CompatibilityDiagnostic>,
    ) {
        if self.policy.reject_empty_workloads
            && requirements.is_empty()
        {
            diagnostics.push(
                CompatibilityDiagnostic::new(
                    DiagnosticSeverity::Fatal,
                    CompatibilityCode::EmptyWorkload,
                    "workload",
                    "workload contains no executable quantum \
                     requirements",
                )
                .with_remediation(
                    "construct a valid quantum workload",
                ),
            );
        }
    }
}

impl Default for CompatibilityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// No-calibration marker
// =============================================================================

struct NoCalibration;

impl CalibrationView for NoCalibration {
    fn freshness(&self) -> Option<CalibrationFreshness> {
        None
    }
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Check the current repository `CircuitRequirements` contract.
pub fn check_circuit_compatibility(
    backend: &QuantumBackend,
    requirements: &CircuitRequirements,
) -> Result<CompatibilityReport, CompatibilityError> {
    CompatibilityAnalyzer::new().analyze(
        backend,
        &CompatibilityRequirements::from_circuit(
            requirements,
        ),
    )
}

/// Check generalized hardware compatibility.
pub fn check_compatibility(
    backend: &QuantumBackend,
    requirements: &CompatibilityRequirements,
) -> Result<CompatibilityReport, CompatibilityError> {
    CompatibilityAnalyzer::new().analyze(
        backend,
        requirements,
    )
}

/// Map the current backend kind to its default execution model.
pub const fn execution_model_for_backend_kind(
    kind: BackendKind,
) -> ExecutionModel {
    match kind {
        BackendKind::Simulator =>
            ExecutionModel::Simulator,

        BackendKind::Emulator =>
            ExecutionModel::Emulator,

        BackendKind::Qpu =>
            ExecutionModel::GateModel,

        BackendKind::Custom =>
            ExecutionModel::Custom,
    }
}

/// Stable human-readable report summary.
pub fn summarize(
    report: &CompatibilityReport,
) -> String {
    format!(
        "backend={} workload={} status={} diagnostics={} transformations={}",
        report.backend_id,
        report.workload_kind.as_str(),
        report.status,
        report.diagnostics.len(),
        report
            .transformation_plan
            .requirements()
            .len(),
    )
}

// =============================================================================
// Internal helpers
// =============================================================================

fn classify(
    diagnostics: &[CompatibilityDiagnostic],
    plan: &TransformationPlan,
) -> CompatibilityStatus {
    if diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Fatal
    }) {
        return CompatibilityStatus::Incompatible;
    }

    if !plan.is_empty()
        || diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
        })
    {
        return CompatibilityStatus::RequiresTransformation;
    }

    if diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
    }) {
        return CompatibilityStatus::CompatibleWithWarnings;
    }

    CompatibilityStatus::Compatible
}

fn check_feature(
    required: bool,
    supported: bool,
    code: CompatibilityCode,
    subject: &str,
    message: &str,
    remediation: &str,
    diagnostics: &mut Vec<CompatibilityDiagnostic>,
) {
    if !required || supported {
        return;
    }

    diagnostics.push(
        CompatibilityDiagnostic::new(
            DiagnosticSeverity::Fatal,
            code,
            subject,
            message,
        )
        .with_remediation(remediation),
    );
}

fn normalize_instruction(
    instruction: &str,
) -> String {
    instruction
        .trim()
        .to_ascii_lowercase()
}

fn is_rotation_instruction(
    instruction: &str,
) -> bool {
    matches!(
        instruction,
        "rx"
            | "ry"
            | "rz"
            | "u"
            | "u1"
            | "u2"
            | "u3"
    ) || instruction.starts_with("rx(")
        || instruction.starts_with("ry(")
        || instruction.starts_with("rz(")
        || instruction.starts_with("u(")
}

fn looks_parameterized(
    instruction: &str,
) -> bool {
    instruction.contains('(')
        && instruction.ends_with(')')
        && instruction != "measure()"
        && instruction != "reset()"
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
    };

    use crate::quantum::hardware::topology::{
        Coupling,
        HardwareTopology,
    };

    fn backend() -> QuantumBackend {
        let metadata = BackendMetadata::new(
            "local/test",
            "Test backend",
            "Zamani",
            "1.0.0",
            BackendKind::Qpu,
        );

        let capabilities =
            BackendCapabilities::new()
                .with_gates([
                    "x",
                    "h",
                    "cx",
                    "measure",
                    "reset",
                ]);

        let limits =
            BackendLimits::unlimited()
                .with_max_qubits(4)
                .with_max_shots(10_000);

        QuantumBackend::new(
            metadata,
            capabilities,
            limits,
            HardwareTopology::linear(4)
                .expect("valid topology"),
        )
        .expect("valid backend")
    }

    #[test]
    fn circuit_requirements_convert_without_loss() {
        let circuit = CircuitRequirements {
            qubit_count: 2,
            circuit_depth: 3,
            operation_count: 4,
            shots: 100,
            gates: vec![
                "h".to_owned(),
                "cx".to_owned(),
            ],
            two_qubit_edges: vec![(0, 1)],
            requires_measurement: true,
            requires_reset: false,
            requires_mid_circuit_measurement: false,
            requires_classical_control: false,
            requires_dynamic_circuits: false,
        };

        let requirements =
            CompatibilityRequirements::from_circuit(
                &circuit,
            );

        assert_eq!(
            requirements.qubit_count,
            2
        );
        assert_eq!(
            requirements.circuit_depth,
            3
        );
        assert_eq!(
            requirements.operation_count,
            4
        );
        assert_eq!(
            requirements.shots,
            100
        );
        assert_eq!(
            requirements.instructions,
            circuit.gates
        );
        assert_eq!(
            requirements.two_resource_edges,
            circuit.two_qubit_edges
        );
    }

    #[test]
    fn supported_circuit_is_compatible() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 2,
                operation_count: 2,
                shots: 100,
                instructions: vec![
                    "h".to_owned(),
                    "cx".to_owned(),
                ],
                two_resource_edges:
                    vec![(0, 1)],
                requires_measurement: true,
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::Compatible
        );

        assert!(
            report
                .transformation_plan
                .is_empty()
        );

        assert!(!report.has_errors());
    }

    #[test]
    fn unsupported_gate_can_require_decomposition() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 1,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["t".to_owned()],
                allow_gate_decomposition:
                    true,
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::RequiresTransformation
        );

        assert!(
            report
                .transformation_plan
                .contains(
                    TransformationRequirement::GateDecomposition
                )
        );
    }

    #[test]
    fn unsupported_gate_is_incompatible_when_decomposition_disabled() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 1,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["t".to_owned()],
                allow_gate_decomposition:
                    false,
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(report.has_fatal_errors());
    }

    #[test]
    fn disconnected_resources_require_routing() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 4,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["cx".to_owned()],
                two_resource_edges:
                    vec![(0, 3)],
                allow_routing: true,
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::RequiresTransformation
        );

        assert!(
            report
                .transformation_plan
                .contains(
                    TransformationRequirement::Routing
                )
        );
    }

    #[test]
    fn reverse_direction_is_not_silently_accepted() {
        let topology =
            HardwareTopology::from_couplings(
                2,
                [Coupling::directed(0, 1)],
            )
            .expect("valid topology");

        let metadata =
            BackendMetadata::new(
                "local/directed",
                "Directed backend",
                "Zamani",
                "1.0.0",
                BackendKind::Qpu,
            );

        let capabilities =
            BackendCapabilities::new()
                .with_gate("cx");

        let backend =
            QuantumBackend::new(
                metadata,
                capabilities,
                BackendLimits::unlimited(),
                topology,
            )
            .expect("valid backend");

        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 2,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["cx".to_owned()],
                two_resource_edges:
                    vec![(1, 0)],
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend,
                &requirements,
            )
            .expect("analysis must succeed");

        assert!(matches!(
            report.status,
            CompatibilityStatus::RequiresTransformation
                | CompatibilityStatus::Incompatible
        ));

        assert!(
            report.diagnostics.iter().any(
                |diagnostic| {
                    diagnostic.code
                        == CompatibilityCode::UnsupportedConnection
                        || diagnostic.code
                            == CompatibilityCode::RoutingRequired
                }
            )
        );
    }

    #[test]
    fn stale_calibration_is_fatal() {
        struct TestCalibration;

        impl CalibrationView
            for TestCalibration
        {
            fn freshness(
                &self,
            ) -> Option<CalibrationFreshness> {
                Some(
                    CalibrationFreshness::stale(
                        100,
                        10,
                    ),
                )
            }
        }

        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 1,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["x".to_owned()],
                requires_calibration:
                    true,
                ..CompatibilityRequirements::default()
            };

        let analyzer =
            CompatibilityAnalyzer::new();

        let calibration =
            TestCalibration;

        let report = analyzer
            .analyze_with_calibration(
                &backend(),
                &requirements,
                Some(&calibration),
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(
            report.diagnostics.iter().any(
                |diagnostic| {
                    diagnostic.code
                        == CompatibilityCode::CalibrationStale
                }
            )
        );
    }

    #[test]
    fn missing_required_calibration_is_fatal() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 1,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["x".to_owned()],
                requires_calibration:
                    true,
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        assert_eq!(
            report.status,
            CompatibilityStatus::Incompatible
        );

        assert!(
            report.diagnostics.iter().any(
                |diagnostic| {
                    diagnostic.code
                        == CompatibilityCode::CalibrationUnavailable
                }
            )
        );
    }

    #[test]
    fn diagnostics_are_deterministically_sorted() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 8,
                circuit_depth: 10,
                operation_count: 100,
                shots: 100_000,
                instructions: vec![
                    "unknown_b".to_owned(),
                    "unknown_a".to_owned(),
                ],
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        let mut sorted =
            report.diagnostics.clone();

        sorted.sort_unstable();

        assert_eq!(
            report.diagnostics,
            sorted
        );
    }

    #[test]
    fn status_strings_are_stable() {
        assert_eq!(
            CompatibilityStatus::Compatible.as_str(),
            "compatible"
        );

        assert_eq!(
            DiagnosticSeverity::Fatal.as_str(),
            "fatal"
        );

        assert_eq!(
            CompatibilityCode::UnsupportedGate.as_str(),
            "unsupported_gate"
        );
    }

    #[test]
    fn backend_kind_mapping_is_stable() {
        assert_eq!(
            execution_model_for_backend_kind(
                BackendKind::Simulator
            ),
            ExecutionModel::Simulator
        );

        assert_eq!(
            execution_model_for_backend_kind(
                BackendKind::Emulator
            ),
            ExecutionModel::Emulator
        );

        assert_eq!(
            execution_model_for_backend_kind(
                BackendKind::Qpu
            ),
            ExecutionModel::GateModel
        );

        assert_eq!(
            execution_model_for_backend_kind(
                BackendKind::Custom
            ),
            ExecutionModel::Custom
        );
    }

    #[test]
    fn report_summary_is_stable() {
        let requirements =
            CompatibilityRequirements {
                workload_kind:
                    WorkloadKind::Circuit,
                execution_model:
                    ExecutionModel::GateModel,
                qubit_count: 1,
                operation_count: 1,
                shots: 1,
                instructions:
                    vec!["x".to_owned()],
                ..CompatibilityRequirements::default()
            };

        let report =
            check_compatibility(
                &backend(),
                &requirements,
            )
            .expect("analysis must succeed");

        let summary =
            summarize(&report);

        assert!(
            summary.contains("backend=local/test")
        );

        assert!(
            summary.contains("workload=circuit")
        );

        assert!(
            summary.contains("status=compatible")
        );
    }
}