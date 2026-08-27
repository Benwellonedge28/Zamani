//! Zamani Quantum — Hardware Validation Engine
//!
//! Production-grade, provider-neutral validation for quantum workloads
//! against hardware backend capabilities, limits, topology, and execution
//! requirements.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "Can this quantum workload legally and safely be accepted by this
//! > hardware backend as currently described?"
//!
//! It validates:
//!
//! - backend metadata;
//! - backend operational state;
//! - backend topology invariants;
//! - qubit/resource requirements;
//! - circuit depth;
//! - operation count;
//! - shot count;
//! - gate support;
//! - parameterized-gate requirements;
//! - arbitrary rotation requirements;
//! - measurement;
//! - reset;
//! - mid-circuit measurement;
//! - classical control;
//! - dynamic circuits;
//! - physical qubit indices;
//! - native two-qubit connectivity;
//! - backend/topology consistency;
//! - malformed workload declarations;
//! - contradictory workload requirements.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - execute quantum programs;
//! - communicate with QPUs;
//! - perform network I/O;
//! - authenticate providers;
//! - store credentials;
//! - perform transpilation;
//! - decompose gates;
//! - perform routing;
//! - perform scheduling;
//! - modify topology;
//! - modify calibration;
//! - select a provider;
//! - perform benchmark analysis;
//! - depend on `quantum::benchmarking`;
//! - depend on provider-specific SDKs;
//! - depend on Danga.
//!
//! Validation may report that transpilation/routing is required, but it does
//! not perform those transformations.
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
//! optimization / analysis
//!      |
//!      v
//! routing / scheduling
//!      |
//!      v
//! hardware::validation
//!      |
//!      +---- capabilities
//!      +---- limits
//!      +---- topology
//!      +---- execution requirements
//!      |
//!      v
//! backend / provider adapter
//! ```
//!
//! # Dependency direction
//!
//! ```text
//! validation.rs
//!     |
//!     +---- backend.rs
//!     |
//!     +---- topology.rs
//!     |
//!     +---- no dependency on providers
//!     +---- no dependency on benchmarking
//!     +---- no dependency on runtime
//! ```
//!
//! `backend.rs` owns the backend data model. This module owns the reusable
//! validation policy over that data model.
//!
//! # Production design
//!
//! Validation is deliberately separated into:
//!
//! 1. structural validation;
//! 2. resource validation;
//! 3. capability validation;
//! 4. topology validation;
//! 5. semantic consistency validation.
//!
//! Validation produces a complete deterministic report rather than relying
//! solely on a first-error `Result`.
//!
//! Consumers that only need pass/fail can use `validate()`.
//!
//! Consumers such as compilers, schedulers, Danga, benchmarking, IDEs, and
//! diagnostics systems should use `validate_with_report()`.
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
//! No external crates are required.
//!
//! # Integration contract
//!
//! This file is independently complete once the following existing types are
//! available:
//!
//! - `super::backend::BackendCapabilities`;
//! - `super::backend::BackendError`;
//! - `super::backend::BackendLimits`;
//! - `super::backend::BackendMetadata`;
//! - `super::backend::BackendStatus`;
//! - `super::backend::CircuitRequirements`;
//! - `super::backend::QuantumBackend`;
//! - `super::topology::HardwareTopology`;
//!
//! The authoritative topology implementation remains in `topology.rs`.
//!
//! The existing backend API remains source-compatible because this module
//! adds validation functionality rather than changing the backend data model.
//!
//! # Future integration
//!
//! `QuantumBackend::validate()` should eventually delegate to:
//!
//! ```text
//! validation::validate(self, requirements)
//! ```
//!
//! That delegation is intentionally not performed inside this file. This
//! prevents validation.rs from creating a circular ownership relationship with
//! the backend aggregate.
//!
//! Provider adapters should validate before submission.
//!
//! Benchmarking should validate before execution.
//!
//! Routing should consume validation diagnostics when routing is required.
//!
//! Scheduling should consume the validated workload/backend pair.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------
//!
//! The schema identifiers make diagnostics and serialized provenance stable
//! across Rust implementation changes.

/// Stable schema version for hardware validation.
pub const VALIDATION_SCHEMA_VERSION: u16 = 1;

/// Stable schema identifier for hardware validation.
pub const VALIDATION_SCHEMA_ID: &str = "zamani.quantum.hardware.validation";

use std::collections::BTreeSet;
use std::fmt;

use super::backend::{
    BackendCapabilities,
    BackendError,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    QuantumBackend,
};
use super::topology::HardwareTopology;

// =============================================================================
// Severity
// =============================================================================

/// Severity of a hardware validation diagnostic.
///
/// Ordering is intentionally stable:
///
/// `Info < Warning < Error < Fatal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationSeverity {
    /// Informational diagnostic.
    Info,

    /// The workload may still be executable, but the caller should be aware
    /// of a limitation or condition.
    Warning,

    /// The workload cannot currently be accepted as described.
    Error,

    /// The backend/workload state violates a fundamental invariant and must
    /// not proceed.
    Fatal,
}

impl ValidationSeverity {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }

    /// Returns whether this severity prevents execution.
    pub const fn blocks_execution(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }
}

// =============================================================================
// Diagnostic codes
// =============================================================================

/// Stable machine-readable validation diagnostic code.
///
/// These codes are deliberately independent of human-readable error messages
/// so tooling can safely match diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValidationCode {
    // Backend identity/state.
    BackendIdEmpty,
    BackendNameEmpty,
    BackendProviderEmpty,
    BackendVersionEmpty,
    BackendUnavailable,

    // Backend structure.
    BackendTopologyInvalid,
    BackendTopologyResourceMismatch,
    BackendTopologyEmpty,

    // Workload structure.
    ZeroQubits,
    ZeroShots,
    ZeroOperationsWithNonZeroDepth,
    DepthWithoutOperations,
    OperationsWithoutResources,
    GateListOperationMismatch,

    // Resource limits.
    QubitLimitExceeded,
    CircuitDepthExceeded,
    OperationLimitExceeded,
    ShotLimitExceeded,

    // Capability requirements.
    MeasurementUnsupported,
    ResetUnsupported,
    MidCircuitMeasurementUnsupported,
    ClassicalControlUnsupported,
    DynamicCircuitUnsupported,
    ArbitraryRotationUnsupported,
    ParameterizedGateUnsupported,
    GateUnsupported,

    // Qubit/connection validity.
    InvalidQubit,
    InvalidConnection,
    SelfConnection,
    UnsupportedConnection,

    // Semantic consistency.
    DynamicCircuitRequiresMidCircuitMeasurement,
    DynamicCircuitRequiresClassicalControl,
    MidCircuitMeasurementRequiresMeasurement,
    ClassicalControlRequiresDynamicCircuit,
    DuplicateGateDeclaration,
    EmptyGateName,

    // Topology/routing information.
    RoutingRequired,
    DirectionalRoutingRequired,

    // Non-blocking information.
    NativeGate,
    BackendHasUnusedCapacity,
}

impl ValidationCode {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendIdEmpty => "backend.id.empty",
            Self::BackendNameEmpty => "backend.name.empty",
            Self::BackendProviderEmpty => "backend.provider.empty",
            Self::BackendVersionEmpty => "backend.version.empty",
            Self::BackendUnavailable => "backend.unavailable",

            Self::BackendTopologyInvalid => "backend.topology.invalid",
            Self::BackendTopologyResourceMismatch => {
                "backend.topology.resource_mismatch"
            }
            Self::BackendTopologyEmpty => "backend.topology.empty",

            Self::ZeroQubits => "workload.qubits.zero",
            Self::ZeroShots => "workload.shots.zero",
            Self::ZeroOperationsWithNonZeroDepth => {
                "workload.operations.zero_with_depth"
            }
            Self::DepthWithoutOperations => {
                "workload.depth.without_operations"
            }
            Self::OperationsWithoutResources => {
                "workload.operations.without_resources"
            }
            Self::GateListOperationMismatch => {
                "workload.gates.operation_count_mismatch"
            }

            Self::QubitLimitExceeded => "resource.qubits.limit_exceeded",
            Self::CircuitDepthExceeded => "resource.depth.limit_exceeded",
            Self::OperationLimitExceeded => {
                "resource.operations.limit_exceeded"
            }
            Self::ShotLimitExceeded => "resource.shots.limit_exceeded",

            Self::MeasurementUnsupported => {
                "capability.measurement.unsupported"
            }
            Self::ResetUnsupported => "capability.reset.unsupported",
            Self::MidCircuitMeasurementUnsupported => {
                "capability.mid_circuit_measurement.unsupported"
            }
            Self::ClassicalControlUnsupported => {
                "capability.classical_control.unsupported"
            }
            Self::DynamicCircuitUnsupported => {
                "capability.dynamic_circuit.unsupported"
            }
            Self::ArbitraryRotationUnsupported => {
                "capability.arbitrary_rotation.unsupported"
            }
            Self::ParameterizedGateUnsupported => {
                "capability.parameterized_gate.unsupported"
            }
            Self::GateUnsupported => "capability.gate.unsupported",

            Self::InvalidQubit => "topology.qubit.invalid",
            Self::InvalidConnection => "topology.connection.invalid",
            Self::SelfConnection => "topology.connection.self",
            Self::UnsupportedConnection => {
                "topology.connection.unsupported"
            }

            Self::DynamicCircuitRequiresMidCircuitMeasurement => {
                "semantic.dynamic_circuit.requires_mid_circuit_measurement"
            }
            Self::DynamicCircuitRequiresClassicalControl => {
                "semantic.dynamic_circuit.requires_classical_control"
            }
            Self::MidCircuitMeasurementRequiresMeasurement => {
                "semantic.mid_circuit_measurement.requires_measurement"
            }
            Self::ClassicalControlRequiresDynamicCircuit => {
                "semantic.classical_control.requires_dynamic_circuit"
            }
            Self::DuplicateGateDeclaration => {
                "semantic.gate.duplicate_declaration"
            }
            Self::EmptyGateName => "semantic.gate.empty_name",

            Self::RoutingRequired => "routing.required",
            Self::DirectionalRoutingRequired => "routing.directional_required",

            Self::NativeGate => "capability.gate.native",
            Self::BackendHasUnusedCapacity => "backend.capacity.unused",
        }
    }
}

// =============================================================================
// Validation diagnostic
// =============================================================================

/// One deterministic validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    /// Stable diagnostic code.
    pub code: ValidationCode,

    /// Diagnostic severity.
    pub severity: ValidationSeverity,

    /// Human-readable message.
    pub message: String,

    /// Optional workload/backend field associated with the diagnostic.
    pub field: Option<String>,

    /// Optional numeric value associated with the diagnostic.
    pub value: Option<usize>,

    /// Optional expected/maximum numeric value.
    pub expected: Option<usize>,

    /// Optional backend identifier.
    pub backend_id: Option<String>,

    /// Optional qubit/resource identifier.
    pub qubit: Option<usize>,

    /// Optional second qubit/resource identifier.
    pub target: Option<usize>,
}

impl ValidationDiagnostic {
    /// Creates a diagnostic with only the essential fields.
    pub fn new(
        code: ValidationCode,
        severity: ValidationSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            field: None,
            value: None,
            expected: None,
            backend_id: None,
            qubit: None,
            target: None,
        }
    }

    /// Associates a field name.
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Associates an observed value.
    pub fn with_value(mut self, value: usize) -> Self {
        self.value = Some(value);
        self
    }

    /// Associates an expected value.
    pub fn with_expected(mut self, expected: usize) -> Self {
        self.expected = Some(expected);
        self
    }

    /// Associates a backend.
    pub fn with_backend(mut self, backend_id: impl Into<String>) -> Self {
        self.backend_id = Some(backend_id.into());
        self
    }

    /// Associates a qubit/resource.
    pub fn with_qubit(mut self, qubit: usize) -> Self {
        self.qubit = Some(qubit);
        self
    }

    /// Associates a target resource.
    pub fn with_target(mut self, target: usize) -> Self {
        self.target = Some(target);
        self
    }

    /// Returns whether the diagnostic blocks execution.
    pub const fn blocks_execution(&self) -> bool {
        self.severity.blocks_execution()
    }
}

impl fmt::Display for ValidationDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}: {}",
            self.severity.as_str(),
            self.code.as_str(),
            self.message
        )
    }
}

// =============================================================================
// Validation report
// =============================================================================

/// Complete deterministic validation report.
///
/// The report deliberately contains every diagnostic discovered during a
/// validation pass rather than stopping at the first error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// Validation schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: &'static str,

    /// Backend identifier, when available.
    pub backend_id: Option<String>,

    /// All diagnostics in deterministic validation order.
    pub diagnostics: Vec<ValidationDiagnostic>,
}

impl ValidationReport {
    /// Creates an empty report.
    pub fn new(backend_id: Option<String>) -> Self {
        Self {
            schema_version: VALIDATION_SCHEMA_VERSION,
            schema_id: VALIDATION_SCHEMA_ID,
            backend_id,
            diagnostics: Vec::new(),
        }
    }

    /// Adds a diagnostic.
    pub fn push(&mut self, diagnostic: ValidationDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns whether there are no diagnostics at all.
    pub fn is_clean(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns whether the workload is executable.
    pub fn is_valid(&self) -> bool {
        !self.has_blocking_errors()
    }

    /// Returns whether at least one error/fatal diagnostic exists.
    pub fn has_blocking_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(ValidationDiagnostic::blocks_execution)
    }

    /// Returns whether at least one warning exists.
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.severity == ValidationSeverity::Warning
            })
    }

    /// Returns the number of diagnostics at the requested severity.
    pub fn count_severity(&self, severity: ValidationSeverity) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == severity)
            .count()
    }

    /// Returns all errors and fatal diagnostics.
    pub fn errors(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics.iter().filter(|diagnostic| {
            diagnostic.severity.blocks_execution()
        })
    }

    /// Returns all warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationDiagnostic> {
        self.diagnostics.iter().filter(|diagnostic| {
            diagnostic.severity == ValidationSeverity::Warning
        })
    }

    /// Returns a stable list of diagnostic codes.
    pub fn codes(&self) -> Vec<ValidationCode> {
        self.diagnostics.iter().map(|item| item.code).collect()
    }

    /// Converts a failed report to the existing backend error vocabulary.
    ///
    /// The first blocking diagnostic is selected deterministically.
    ///
    /// This method exists specifically for compatibility with the current
    /// `QuantumBackend::validate() -> Result<(), BackendError>` API.
    pub fn first_backend_error(&self) -> Option<BackendError> {
        self.diagnostics
            .iter()
            .find(|diagnostic| diagnostic.blocks_execution())
            .and_then(diagnostic_to_backend_error)
    }

    /// Returns `Ok(())` when valid, otherwise returns the first compatible
    /// backend error.
    pub fn into_result(self) -> Result<(), BackendError> {
        match self.first_backend_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.diagnostics.is_empty() {
            return write!(formatter, "hardware validation passed");
        }

        for (index, diagnostic) in self.diagnostics.iter().enumerate() {
            if index != 0 {
                formatter.write_str("\n")?;
            }

            write!(formatter, "{}", diagnostic)?;
        }

        Ok(())
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Validation policy controlling non-fatal diagnostics.
///
/// The default policy is intentionally conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationPolicy {
    /// Report unused backend capacity as informational diagnostics.
    pub report_unused_capacity: bool,

    /// Report gates that are directly native to the backend.
    pub report_native_gates: bool,

    /// Report when physical connectivity exists but the requested direction
    /// does not.
    pub report_directional_routing: bool,

    /// Treat an unknown/unavailable backend as a hard failure.
    pub require_available_backend: bool,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            report_unused_capacity: false,
            report_native_gates: false,
            report_directional_routing: true,
            require_available_backend: true,
        }
    }
}

impl ValidationPolicy {
    /// Strict production policy.
    pub const fn strict() -> Self {
        Self {
            report_unused_capacity: false,
            report_native_gates: false,
            report_directional_routing: true,
            require_available_backend: true,
        }
    }

    /// Compiler-analysis policy.
    ///
    /// This policy still blocks unsafe execution but provides additional
    /// information useful to compilers and IDEs.
    pub const fn analysis() -> Self {
        Self {
            report_unused_capacity: true,
            report_native_gates: true,
            report_directional_routing: true,
            require_available_backend: true,
        }
    }

    /// Returns the default production policy.
    pub const fn production() -> Self {
        Self::strict()
    }
}

// =============================================================================
// Public validation API
// =============================================================================

/// Validates a workload against a backend using the strict production policy.
///
/// This is the compatibility-oriented entry point.
///
/// It returns the existing `BackendError` vocabulary so current callers can
/// migrate without changing their error handling immediately.
pub fn validate(
    backend: &QuantumBackend,
    requirements: &CircuitRequirements,
) -> Result<(), BackendError> {
    validate_with_policy(backend, requirements, ValidationPolicy::strict())
        .into_result()
}

/// Validates a workload and returns the complete diagnostic report.
pub fn validate_with_report(
    backend: &QuantumBackend,
    requirements: &CircuitRequirements,
) -> ValidationReport {
    validate_with_policy(
        backend,
        requirements,
        ValidationPolicy::strict(),
    )
}

/// Validates using an explicit policy.
pub fn validate_with_policy(
    backend: &QuantumBackend,
    requirements: &CircuitRequirements,
    policy: ValidationPolicy,
) -> ValidationReport {
    let backend_id = Some(backend.metadata.id.clone());
    let mut report = ValidationReport::new(backend_id);

    validate_backend_metadata(
        &backend.metadata,
        &mut report,
    );

    validate_backend_status(
        &backend.metadata.status,
        &backend.metadata.id,
        policy,
        &mut report,
    );

    validate_backend_topology(
        &backend.topology,
        backend.limits,
        &mut report,
    );

    validate_workload_structure(
        requirements,
        &mut report,
    );

    validate_resource_limits(
        backend.limits,
        requirements,
        &mut report,
    );

    validate_capabilities(
        &backend.capabilities,
        requirements,
        &mut report,
    );

    validate_gate_set(
        &backend.capabilities,
        requirements,
        policy,
        &mut report,
    );

    validate_qubit_references(
        &backend.topology,
        requirements,
        &mut report,
    );

    validate_connections(
        &backend.topology,
        requirements,
        policy,
        &mut report,
    );

    validate_semantic_consistency(
        requirements,
        &mut report,
    );

    report
}

/// Validates backend metadata independently.
///
/// This function is useful for backend discovery/registration before a
/// topology or workload exists.
pub fn validate_metadata(
    metadata: &BackendMetadata,
) -> ValidationReport {
    let mut report =
        ValidationReport::new(Some(metadata.id.clone()));

    validate_backend_metadata(metadata, &mut report);

    report
}

/// Validates topology independently.
///
/// This is deliberately separate from workload validation so provider
/// discovery can reject malformed topology snapshots before they are exposed
/// to compilers or schedulers.
pub fn validate_topology(
    topology: &HardwareTopology,
) -> ValidationReport {
    let mut report = ValidationReport::new(None);

    validate_topology_invariants(topology, &mut report);

    report
}

/// Validates a workload independently of a backend.
///
/// This catches malformed requirements before backend selection.
pub fn validate_workload(
    requirements: &CircuitRequirements,
) -> ValidationReport {
    let mut report = ValidationReport::new(None);

    validate_workload_structure(
        requirements,
        &mut report,
    );

    validate_semantic_consistency(
        requirements,
        &mut report,
    );

    report
}

// =============================================================================
// Backend metadata validation
// =============================================================================

fn validate_backend_metadata(
    metadata: &BackendMetadata,
    report: &mut ValidationReport,
) {
    let backend_id = metadata.id.clone();

    if metadata.id.trim().is_empty() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendIdEmpty,
                ValidationSeverity::Fatal,
                "backend identifier cannot be empty or whitespace",
            )
            .with_field("metadata.id")
            .with_backend(backend_id.clone()),
        );
    }

    if metadata.name.trim().is_empty() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendNameEmpty,
                ValidationSeverity::Error,
                "backend name cannot be empty or whitespace",
            )
            .with_field("metadata.name")
            .with_backend(backend_id.clone()),
        );
    }

    if metadata.provider.trim().is_empty() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendProviderEmpty,
                ValidationSeverity::Error,
                "backend provider cannot be empty or whitespace",
            )
            .with_field("metadata.provider")
            .with_backend(backend_id.clone()),
        );
    }

    if metadata.version.trim().is_empty() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendVersionEmpty,
                ValidationSeverity::Error,
                "backend version cannot be empty or whitespace",
            )
            .with_field("metadata.version")
            .with_backend(backend_id),
        );
    }
}

fn validate_backend_status(
    status: &BackendStatus,
    backend_id: &str,
    policy: ValidationPolicy,
    report: &mut ValidationReport,
) {
    if !policy.require_available_backend {
        return;
    }

    if !status.is_usable() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendUnavailable,
                ValidationSeverity::Error,
                format!(
                    "backend '{}' is not available for execution ({:?})",
                    backend_id, status
                ),
            )
            .with_field("metadata.status")
            .with_backend(backend_id.to_owned()),
        );
    }
}

// =============================================================================
// Backend topology validation
// =============================================================================

fn validate_backend_topology(
    topology: &HardwareTopology,
    limits: BackendLimits,
    report: &mut ValidationReport,
) {
    validate_topology_invariants(topology, report);

    let topology_count = topology.qubit_count();

    if topology_count == 0 {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendTopologyEmpty,
                ValidationSeverity::Fatal,
                "backend topology contains no quantum resources",
            )
            .with_field("topology.qubit_count"),
        );

        return;
    }

    if limits.max_qubits != 0
        && topology_count > limits.max_qubits
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendTopologyResourceMismatch,
                ValidationSeverity::Error,
                format!(
                    "backend topology exposes {} resources but backend \
                     limit declares a maximum of {}",
                    topology_count,
                    limits.max_qubits
                ),
            )
            .with_field("limits.max_qubits")
            .with_value(topology_count)
            .with_expected(limits.max_qubits),
        );
    }
}

fn validate_topology_invariants(
    topology: &HardwareTopology,
    report: &mut ValidationReport,
) {
    if topology.qubit_count() == 0 {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendTopologyEmpty,
                ValidationSeverity::Fatal,
                "hardware topology must contain at least one resource",
            )
            .with_field("topology.qubit_count"),
        );

        return;
    }

    if let Err(error) = topology.validate() {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::BackendTopologyInvalid,
                ValidationSeverity::Fatal,
                format!("hardware topology invariant violation: {}", error),
            )
            .with_field("topology"),
        );
    }

    // Verify all resources are represented consistently by the public
    // topology API. This catches malformed implementations even if a future
    // topology implementation changes its internal representation.
    for resource in topology.resources() {
        if let Err(error) = topology.neighbours(resource) {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::BackendTopologyInvalid,
                    ValidationSeverity::Fatal,
                    format!(
                        "cannot inspect outgoing topology for resource {}: {}",
                        resource, error
                    ),
                )
                .with_qubit(resource),
            );
        }

        if let Err(error) = topology.physical_neighbours(resource) {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::BackendTopologyInvalid,
                    ValidationSeverity::Fatal,
                    format!(
                        "cannot inspect physical topology for resource {}: {}",
                        resource, error
                    ),
                )
                .with_qubit(resource),
            );
        }
    }

    // Check every coupling against the public topology semantics.
    for coupling in topology.couplings() {
        if !topology.contains(coupling.source)
            || !topology.contains(coupling.target)
        {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::BackendTopologyInvalid,
                    ValidationSeverity::Fatal,
                    format!(
                        "coupling {} -> {} references a resource outside \
                         the topology",
                        coupling.source, coupling.target
                    ),
                )
                .with_qubit(coupling.source)
                .with_target(coupling.target),
            );

            continue;
        }

        if coupling.source == coupling.target {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::BackendTopologyInvalid,
                    ValidationSeverity::Fatal,
                    format!(
                        "topology contains a self-coupling on resource {}",
                        coupling.source
                    ),
                )
                .with_qubit(coupling.source),
            );
        }

        match topology.coupling(
            coupling.source,
            coupling.target,
        ) {
            Ok(Some(_)) => {}
            Ok(None) => {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::BackendTopologyInvalid,
                        ValidationSeverity::Fatal,
                        format!(
                            "coupling {} -> {} is stored but cannot be \
                             retrieved through the topology API",
                            coupling.source, coupling.target
                        ),
                    )
                    .with_qubit(coupling.source)
                    .with_target(coupling.target),
                );
            }
            Err(error) => {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::BackendTopologyInvalid,
                        ValidationSeverity::Fatal,
                        format!(
                            "topology coupling query failed for {} -> {}: {}",
                            coupling.source,
                            coupling.target,
                            error
                        ),
                    )
                    .with_qubit(coupling.source)
                    .with_target(coupling.target),
                );
            }
        }
    }
}

// =============================================================================
// Workload structural validation
// =============================================================================

fn validate_workload_structure(
    requirements: &CircuitRequirements,
    report: &mut ValidationReport,
) {
    if requirements.qubit_count == 0 {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ZeroQubits,
                ValidationSeverity::Error,
                "quantum workload must contain at least one qubit",
            )
            .with_field("qubit_count")
            .with_value(0),
        );
    }

    if requirements.shots == 0 {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ZeroShots,
                ValidationSeverity::Error,
                "execution shot count must be greater than zero",
            )
            .with_field("shots")
            .with_value(0),
        );
    }

    if requirements.operation_count == 0
        && requirements.circuit_depth > 0
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ZeroOperationsWithNonZeroDepth,
                ValidationSeverity::Error,
                format!(
                    "circuit depth is {} but operation count is zero",
                    requirements.circuit_depth
                ),
            )
            .with_field("operation_count")
            .with_value(requirements.operation_count),
        );
    }

    if requirements.operation_count > 0
        && requirements.qubit_count == 0
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::OperationsWithoutResources,
                ValidationSeverity::Error,
                "workload contains operations but declares zero qubits",
            )
            .with_field("qubit_count")
            .with_value(requirements.qubit_count),
        );
    }

    if requirements.circuit_depth == 0
        && requirements.operation_count > 0
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::DepthWithoutOperations,
                ValidationSeverity::Warning,
                "workload declares operations but zero circuit depth",
            )
            .with_field("circuit_depth")
            .with_value(requirements.circuit_depth),
        );
    }

    if requirements.gates.len() > requirements.operation_count {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::GateListOperationMismatch,
                ValidationSeverity::Error,
                format!(
                    "workload lists {} gate names but declares only {} \
                     operations",
                    requirements.gates.len(),
                    requirements.operation_count
                ),
            )
            .with_field("gates")
            .with_value(requirements.gates.len())
            .with_expected(requirements.operation_count),
        );
    }

    // A gate list may intentionally contain only unique gate kinds rather
    // than one item per operation, so a shorter list is informational rather
    // than automatically invalid.
}

// =============================================================================
// Resource limit validation
// =============================================================================

fn validate_resource_limits(
    limits: BackendLimits,
    requirements: &CircuitRequirements,
    report: &mut ValidationReport,
) {
    if limits.max_qubits != 0
        && requirements.qubit_count > limits.max_qubits
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::QubitLimitExceeded,
                ValidationSeverity::Error,
                format!(
                    "workload requires {} qubits but backend supports at \
                     most {}",
                    requirements.qubit_count,
                    limits.max_qubits
                ),
            )
            .with_field("qubit_count")
            .with_value(requirements.qubit_count)
            .with_expected(limits.max_qubits),
        );
    }

    if limits.max_circuit_depth != 0
        && requirements.circuit_depth > limits.max_circuit_depth
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::CircuitDepthExceeded,
                ValidationSeverity::Error,
                format!(
                    "workload depth {} exceeds backend maximum {}",
                    requirements.circuit_depth,
                    limits.max_circuit_depth
                ),
            )
            .with_field("circuit_depth")
            .with_value(requirements.circuit_depth)
            .with_expected(limits.max_circuit_depth),
        );
    }

    if limits.max_operations != 0
        && requirements.operation_count > limits.max_operations
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::OperationLimitExceeded,
                ValidationSeverity::Error,
                format!(
                    "workload contains {} operations but backend allows \
                     at most {}",
                    requirements.operation_count,
                    limits.max_operations
                ),
            )
            .with_field("operation_count")
            .with_value(requirements.operation_count)
            .with_expected(limits.max_operations),
        );
    }

    if limits.max_shots != 0
        && requirements.shots > limits.max_shots
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ShotLimitExceeded,
                ValidationSeverity::Error,
                format!(
                    "workload requests {} shots but backend allows at most \
                     {}",
                    requirements.shots,
                    limits.max_shots
                ),
            )
            .with_field("shots")
            .with_value(requirements.shots)
            .with_expected(limits.max_shots),
        );
    }
}

// =============================================================================
// Capability validation
// =============================================================================

fn validate_capabilities(
    capabilities: &BackendCapabilities,
    requirements: &CircuitRequirements,
    report: &mut ValidationReport,
) {
    if requirements.requires_measurement
        && !capabilities.measurement
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::MeasurementUnsupported,
                ValidationSeverity::Error,
                "workload requires measurement but backend does not support \
                 measurement",
            )
            .with_field("capabilities.measurement"),
        );
    }

    if requirements.requires_reset
        && !capabilities.reset
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ResetUnsupported,
                ValidationSeverity::Error,
                "workload requires qubit reset but backend does not support \
                 reset",
            )
            .with_field("capabilities.reset"),
        );
    }

    if requirements.requires_mid_circuit_measurement
        && !capabilities.mid_circuit_measurement
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::MidCircuitMeasurementUnsupported,
                ValidationSeverity::Error,
                "workload requires mid-circuit measurement but backend does \
                 not support it",
            )
            .with_field("capabilities.mid_circuit_measurement"),
        );
    }

    if requirements.requires_classical_control
        && !capabilities.classical_control
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ClassicalControlUnsupported,
                ValidationSeverity::Error,
                "workload requires classical control but backend does not \
                 support classical control",
            )
            .with_field("capabilities.classical_control"),
        );
    }

    if requirements.requires_dynamic_circuits
        && !capabilities.dynamic_circuits
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::DynamicCircuitUnsupported,
                ValidationSeverity::Error,
                "workload requires dynamic circuits but backend does not \
                 support dynamic circuits",
            )
            .with_field("capabilities.dynamic_circuits"),
        );
    }
}

// =============================================================================
// Gate validation
// =============================================================================

fn validate_gate_set(
    capabilities: &BackendCapabilities,
    requirements: &CircuitRequirements,
    policy: ValidationPolicy,
    report: &mut ValidationReport,
) {
    let mut seen = BTreeSet::new();

    for gate in &requirements.gates {
        let normalized = normalize_gate_name(gate);

        if normalized.is_empty() {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::EmptyGateName,
                    ValidationSeverity::Error,
                    "workload contains an empty gate name",
                )
                .with_field("gates"),
            );

            continue;
        }

        if !seen.insert(normalized.clone()) {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::DuplicateGateDeclaration,
                    ValidationSeverity::Info,
                    format!(
                        "gate '{}' appears more than once in the declared \
                         gate-kind list",
                        normalized
                    ),
                )
                .with_field("gates"),
            );
        }

        if capabilities.supports_gate(&normalized) {
            if policy.report_native_gates {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::NativeGate,
                        ValidationSeverity::Info,
                        format!(
                            "gate '{}' is natively supported by the backend",
                            normalized
                        ),
                    )
                    .with_field("gates"),
                );
            }

            continue;
        }

        if is_single_qubit_rotation(&normalized) {
            if capabilities.arbitrary_single_qubit_rotations {
                if policy.report_native_gates {
                    report.push(
                        ValidationDiagnostic::new(
                            ValidationCode::NativeGate,
                            ValidationSeverity::Info,
                            format!(
                                "rotation gate '{}' is accepted through \
                                 arbitrary single-qubit rotation support",
                                normalized
                            ),
                        )
                        .with_field("gates"),
                    );
                }

                continue;
            }

            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::ArbitraryRotationUnsupported,
                    ValidationSeverity::Error,
                    format!(
                        "gate '{}' requires arbitrary single-qubit rotation \
                         support, which the backend does not advertise",
                        normalized
                    ),
                )
                .with_field("gates"),
            );

            continue;
        }

        if is_parameterized_gate(&normalized) {
            if capabilities.parameterized_gates {
                if policy.report_native_gates {
                    report.push(
                        ValidationDiagnostic::new(
                            ValidationCode::NativeGate,
                            ValidationSeverity::Info,
                            format!(
                                "parameterized gate '{}' is accepted by \
                                 backend parameterized-gate support",
                                normalized
                            ),
                        )
                        .with_field("gates"),
                    );
                }

                continue;
            }

            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::ParameterizedGateUnsupported,
                    ValidationSeverity::Error,
                    format!(
                        "parameterized gate '{}' is not supported by the \
                         backend",
                        normalized
                    ),
                )
                .with_field("gates"),
            );

            continue;
        }

        report.push(
            ValidationDiagnostic::new(
                ValidationCode::GateUnsupported,
                ValidationSeverity::Error,
                format!(
                    "backend does not support gate '{}'",
                    normalized
                ),
            )
            .with_field("gates"),
        );
    }
}

// =============================================================================
// Qubit reference validation
// =============================================================================

fn validate_qubit_references(
    topology: &HardwareTopology,
    requirements: &CircuitRequirements,
    report: &mut ValidationReport,
) {
    let topology_count = topology.qubit_count();

    if topology_count == 0 {
        return;
    }

    if requirements.qubit_count > topology_count {
        // Resource-limit diagnostics already report the primary failure.
        // Do not duplicate it for every possible qubit.
        return;
    }

    for &(source, target) in &requirements.two_qubit_edges {
        if source >= requirements.qubit_count {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::InvalidQubit,
                    ValidationSeverity::Error,
                    format!(
                        "source qubit {} is outside workload range 0..{}",
                        source,
                        requirements.qubit_count.saturating_sub(1)
                    ),
                )
                .with_field("two_qubit_edges")
                .with_qubit(source),
            );
        }

        if target >= requirements.qubit_count {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::InvalidQubit,
                    ValidationSeverity::Error,
                    format!(
                        "target qubit {} is outside workload range 0..{}",
                        target,
                        requirements.qubit_count.saturating_sub(1)
                    ),
                )
                .with_field("two_qubit_edges")
                .with_qubit(target),
            );
        }

        if source == target {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::SelfConnection,
                    ValidationSeverity::Error,
                    format!(
                        "two-qubit operation cannot connect qubit {} to \
                         itself",
                        source
                    ),
                )
                .with_field("two_qubit_edges")
                .with_qubit(source)
                .with_target(target),
            );
        }
    }
}

// =============================================================================
// Connectivity validation
// =============================================================================

fn validate_connections(
    topology: &HardwareTopology,
    requirements: &CircuitRequirements,
    policy: ValidationPolicy,
    report: &mut ValidationReport,
) {
    for &(source, target) in &requirements.two_qubit_edges {
        if source >= requirements.qubit_count
            || target >= requirements.qubit_count
            || source == target
        {
            continue;
        }

        let native = match topology.is_connected(source, target) {
            Ok(value) => value,
            Err(error) => {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::InvalidConnection,
                        ValidationSeverity::Error,
                        format!(
                            "cannot validate connection {} -> {}: {}",
                            source, target, error
                        ),
                    )
                    .with_field("two_qubit_edges")
                    .with_qubit(source)
                    .with_target(target),
                );

                continue;
            }
        };

        if native {
            continue;
        }

        let physical = match topology.is_physically_adjacent(
            source,
            target,
        ) {
            Ok(value) => value,
            Err(error) => {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::InvalidConnection,
                        ValidationSeverity::Error,
                        format!(
                            "cannot inspect physical connection {} <-> {}: {}",
                            source, target, error
                        ),
                    )
                    .with_field("two_qubit_edges")
                    .with_qubit(source)
                    .with_target(target),
                );

                continue;
            }
        };

        if physical {
            if policy.report_directional_routing {
                report.push(
                    ValidationDiagnostic::new(
                        ValidationCode::DirectionalRoutingRequired,
                        ValidationSeverity::Warning,
                        format!(
                            "physical coupling exists between {} and {}, \
                             but the requested native direction {} -> {} is \
                             not directly supported",
                            source, target, source, target
                        ),
                    )
                    .with_field("two_qubit_edges")
                    .with_qubit(source)
                    .with_target(target),
                );
            }

            continue;
        }

        let path_exists = topology
            .shortest_path_with_mode(
                source,
                target,
                super::topology::PathMode::Undirected,
            )
            .is_ok();

        if path_exists {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::RoutingRequired,
                    ValidationSeverity::Warning,
                    format!(
                        "qubits {} and {} are not directly coupled; \
                         routing is required before execution",
                        source, target
                    ),
                )
                .with_field("two_qubit_edges")
                .with_qubit(source)
                .with_target(target),
            );
        } else {
            report.push(
                ValidationDiagnostic::new(
                    ValidationCode::UnsupportedConnection,
                    ValidationSeverity::Error,
                    format!(
                        "no physical connection path exists between qubits \
                         {} and {}",
                        source, target
                    ),
                )
                .with_field("two_qubit_edges")
                .with_qubit(source)
                .with_target(target),
            );
        }
    }
}

// =============================================================================
// Semantic consistency validation
// =============================================================================

fn validate_semantic_consistency(
    requirements: &CircuitRequirements,
    report: &mut ValidationReport,
) {
    if requirements.requires_dynamic_circuits
        && !requirements.requires_mid_circuit_measurement
        && !requirements.requires_classical_control
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::DynamicCircuitRequiresClassicalControl,
                ValidationSeverity::Warning,
                "workload declares dynamic-circuit execution but does not \
                 declare mid-circuit measurement or classical control",
            )
            .with_field("requires_dynamic_circuits"),
        );
    }

    if requirements.requires_mid_circuit_measurement
        && !requirements.requires_measurement
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::MidCircuitMeasurementRequiresMeasurement,
                ValidationSeverity::Warning,
                "mid-circuit measurement is requested without declaring \
                 general measurement support as a workload requirement",
            )
            .with_field("requires_mid_circuit_measurement"),
        );
    }

    if requirements.requires_classical_control
        && !requirements.requires_dynamic_circuits
    {
        report.push(
            ValidationDiagnostic::new(
                ValidationCode::ClassicalControlRequiresDynamicCircuit,
                ValidationSeverity::Warning,
                "classical control is declared without declaring dynamic \
                 circuit execution",
            )
            .with_field("requires_classical_control"),
        );
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Canonical gate-name normalization.
///
/// This intentionally mirrors the existing backend normalization contract:
///
/// - leading/trailing whitespace is removed;
/// - ASCII case is normalized to uppercase.
///
/// Unicode case folding is deliberately not performed because quantum gate
/// identifiers are protocol identifiers, not natural-language strings.
pub fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_uppercase()
}

/// Returns whether the gate is one of the currently recognized arbitrary
/// single-qubit rotation families.
pub fn is_single_qubit_rotation(gate: &str) -> bool {
    matches!(
        gate,
        "RX" | "RY" | "RZ" | "U" | "U1" | "U2" | "U3"
    )
}

/// Returns whether the gate belongs to the currently recognized parameterized
/// gate families.
///
/// The distinction is intentionally conservative. A provider adapter may
/// support additional parameterized gates through its native gate set.
pub fn is_parameterized_gate(gate: &str) -> bool {
    matches!(
        gate,
        "RX" | "RY" | "RZ" | "U" | "U1" | "U2" | "U3"
    )
}

/// Converts a validation diagnostic into the repository's existing backend
/// error vocabulary.
///
/// This is intentionally lossy because `BackendError` predates the complete
/// diagnostic model. New code should prefer `ValidationReport`.
fn diagnostic_to_backend_error(
    diagnostic: &ValidationDiagnostic,
) -> Option<BackendError> {
    match diagnostic.code {
        ValidationCode::BackendUnavailable => {
            // BackendStatus is not retained in the diagnostic model, so this
            // compatibility conversion uses Unavailable as the conservative
            // fallback. Callers needing exact status should consume the
            // ValidationReport directly.
            Some(BackendError::BackendUnavailable {
                backend_id: diagnostic
                    .backend_id
                    .clone()
                    .unwrap_or_default(),
                status: BackendStatus::Unavailable,
            })
        }

        ValidationCode::BackendIdEmpty
        | ValidationCode::BackendNameEmpty
        | ValidationCode::BackendProviderEmpty
        | ValidationCode::BackendVersionEmpty => {
            Some(BackendError::InvalidBackendId)
        }

        ValidationCode::BackendTopologyEmpty => {
            Some(BackendError::EmptyTopology)
        }

        ValidationCode::BackendTopologyInvalid => {
            Some(BackendError::InvalidTopology(
                diagnostic.message.clone(),
            ))
        }

        ValidationCode::BackendTopologyResourceMismatch
        | ValidationCode::QubitLimitExceeded => {
            Some(BackendError::QubitLimitExceeded {
                requested: diagnostic.value.unwrap_or(0),
                maximum: diagnostic.expected.unwrap_or(0),
            })
        }

        ValidationCode::CircuitDepthExceeded => {
            Some(BackendError::CircuitDepthExceeded {
                requested: diagnostic.value.unwrap_or(0),
                maximum: diagnostic.expected.unwrap_or(0),
            })
        }

        ValidationCode::OperationLimitExceeded => {
            Some(BackendError::OperationLimitExceeded {
                requested: diagnostic.value.unwrap_or(0),
                maximum: diagnostic.expected.unwrap_or(0),
            })
        }

        ValidationCode::ShotLimitExceeded
        | ValidationCode::ZeroShots => {
            Some(BackendError::ShotLimitExceeded {
                requested: diagnostic.value.unwrap_or(0),
                maximum: diagnostic.expected.unwrap_or(0),
            })
        }

        ValidationCode::ZeroQubits => {
            Some(BackendError::ZeroQubits)
        }

        ValidationCode::MeasurementUnsupported => {
            Some(BackendError::MeasurementUnsupported)
        }

        ValidationCode::ResetUnsupported => {
            Some(BackendError::ResetUnsupported)
        }

        ValidationCode::MidCircuitMeasurementUnsupported => {
            Some(
                BackendError::MidCircuitMeasurementUnsupported,
            )
        }

        ValidationCode::ClassicalControlUnsupported => {
            Some(BackendError::ClassicalControlUnsupported)
        }

        ValidationCode::DynamicCircuitUnsupported => {
            Some(BackendError::DynamicCircuitUnsupported)
        }

        ValidationCode::ArbitraryRotationUnsupported
        | ValidationCode::ParameterizedGateUnsupported
        | ValidationCode::GateUnsupported => {
            Some(BackendError::UnsupportedGate {
                gate: diagnostic
                    .field
                    .clone()
                    .unwrap_or_else(|| "unknown".to_owned()),
            })
        }

        ValidationCode::InvalidQubit => {
            Some(BackendError::InvalidQubit {
                qubit: diagnostic.qubit.unwrap_or(0),
                qubit_count: diagnostic.expected.unwrap_or(0),
            })
        }

        ValidationCode::InvalidConnection
        | ValidationCode::SelfConnection
        | ValidationCode::UnsupportedConnection => {
            Some(BackendError::UnsupportedConnection {
                control: diagnostic.qubit.unwrap_or(0),
                target: diagnostic.target.unwrap_or(0),
            })
        }

        // These diagnostics are informational/warnings or describe malformed
        // workload structure not represented by a dedicated BackendError.
        ValidationCode::ZeroOperationsWithNonZeroDepth
        | ValidationCode::DepthWithoutOperations
        | ValidationCode::GateListOperationMismatch
        | ValidationCode::DynamicCircuitRequiresMidCircuitMeasurement
        | ValidationCode::DynamicCircuitRequiresClassicalControl
        | ValidationCode::MidCircuitMeasurementRequiresMeasurement
        | ValidationCode::ClassicalControlRequiresDynamicCircuit
        | ValidationCode::DuplicateGateDeclaration
        | ValidationCode::EmptyGateName
        | ValidationCode::RoutingRequired
        | ValidationCode::DirectionalRoutingRequired
        | ValidationCode::NativeGate
        | ValidationCode::BackendHasUnusedCapacity
        | ValidationCode::OperationsWithoutResources => {
            None
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::hardware::backend::{
        BackendCapabilities,
        BackendKind,
        BackendLimits,
        BackendMetadata,
        BackendStatus,
        CircuitRequirements,
        QuantumBackend,
    };
    use crate::quantum::hardware::topology::HardwareTopology;

    fn backend() -> QuantumBackend {
        let metadata = BackendMetadata::new(
            "validation-test",
            "Validation Test Backend",
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
            HardwareTopology::linear(4).expect(
                "linear topology with four resources must be valid",
            ),
        )
        .expect("test backend must be valid")
    }

    fn valid_requirements() -> CircuitRequirements {
        CircuitRequirements {
            qubit_count: 2,
            circuit_depth: 2,
            operation_count: 3,
            shots: 100,
            gates: vec![
                "H".to_owned(),
                "CX".to_owned(),
                "MEASURE".to_owned(),
            ],
            two_qubit_edges: vec![(0, 1)],
            requires_measurement: true,
            requires_reset: false,
            requires_mid_circuit_measurement: false,
            requires_classical_control: false,
            requires_dynamic_circuits: false,
        }
    }

    #[test]
    fn valid_workload_passes() {
        let report =
            validate_with_report(&backend(), &valid_requirements());

        assert!(report.is_valid());
        assert!(!report.has_blocking_errors());
    }

    #[test]
    fn zero_qubits_is_rejected() {
        let mut requirements = valid_requirements();
        requirements.qubit_count = 0;

        let report =
            validate_with_report(&backend(), &requirements);

        assert!(!report.is_valid());
        assert!(report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == ValidationCode::ZeroQubits
            }));
    }

    #[test]
    fn zero_shots_is_rejected() {
        let mut requirements = valid_requirements();
        requirements.shots = 0;

        let report =
            validate_with_report(&backend(), &requirements);

        assert!(!report.is_valid());
        assert!(report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == ValidationCode::ZeroShots
            }));
    }

    #[test]
    fn unsupported_measurement_is_rejected() {
        let mut backend = backend();
        backend.capabilities.measurement = false;

        let requirements = valid_requirements();

        let report =
            validate_with_report(&backend, &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::MeasurementUnsupported
        }));
    }

    #[test]
    fn unsupported_gate_is_rejected() {
        let mut requirements = valid_requirements();
        requirements.gates.push("NON_NATIVE_GATE".to_owned());
        requirements.operation_count = 4;

        let report =
            validate_with_report(&backend(), &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == ValidationCode::GateUnsupported
        }));
    }

    #[test]
    fn arbitrary_rotation_requires_capability() {
        let mut requirements = valid_requirements();
        requirements.gates.push("RX".to_owned());
        requirements.operation_count = 4;

        let report =
            validate_with_report(&backend(), &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::ArbitraryRotationUnsupported
        }));
    }

    #[test]
    fn arbitrary_rotation_capability_allows_rotation() {
        let mut backend = backend();
        backend.capabilities.arbitrary_single_qubit_rotations = true;

        let mut requirements = valid_requirements();
        requirements.gates.push("RX".to_owned());
        requirements.operation_count = 4;

        let report =
            validate_with_report(&backend, &requirements);

        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::ArbitraryRotationUnsupported
        }));
    }

    #[test]
    fn parameterized_gate_capability_allows_parameterized_gate() {
        let mut backend = backend();
        backend.capabilities.parameterized_gates = true;

        let mut requirements = valid_requirements();
        requirements.gates.push("RZ".to_owned());
        requirements.operation_count = 4;

        let report =
            validate_with_report(&backend, &requirements);

        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::ParameterizedGateUnsupported
        }));
    }

    #[test]
    fn directed_connection_is_valid_in_native_direction() {
        let mut backend = backend();

        backend.topology =
            HardwareTopology::from_couplings(
                2,
                [
                    super::super::topology::Coupling::directed(
                        0, 1,
                    ),
                ],
            )
            .expect("directed topology must be valid");

        let mut requirements = valid_requirements();
        requirements.qubit_count = 2;
        requirements.two_qubit_edges = vec![(0, 1)];

        let report =
            validate_with_report(&backend, &requirements);

        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::UnsupportedConnection
        }));
    }

    #[test]
    fn reverse_directed_connection_reports_routing() {
        let mut backend = backend();

        backend.topology =
            HardwareTopology::from_couplings(
                2,
                [
                    super::super::topology::Coupling::directed(
                        0, 1,
                    ),
                ],
            )
            .expect("directed topology must be valid");

        let mut requirements = valid_requirements();
        requirements.qubit_count = 2;
        requirements.two_qubit_edges = vec![(1, 0)];

        let report =
            validate_with_report(&backend, &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::DirectionalRoutingRequired
        }));
    }

    #[test]
    fn disconnected_resources_are_rejected() {
        let backend = backend();

        let mut requirements = valid_requirements();
        requirements.qubit_count = 4;
        requirements.two_qubit_edges = vec![(0, 3)];

        let report =
            validate_with_report(&backend, &requirements);

        // The linear topology is connected, therefore routing is required
        // rather than outright rejection.
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == ValidationCode::RoutingRequired
        }));
    }

    #[test]
    fn truly_disconnected_topology_rejects_connection() {
        let mut backend = backend();

        backend.topology =
            HardwareTopology::new(4).expect(
                "four isolated resources must be constructible",
            );

        let mut requirements = valid_requirements();
        requirements.qubit_count = 4;
        requirements.two_qubit_edges = vec![(0, 3)];

        let report =
            validate_with_report(&backend, &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::UnsupportedConnection
        }));
    }

    #[test]
    fn topology_validation_is_independent() {
        let topology =
            HardwareTopology::linear(4).expect(
                "linear topology must be valid",
            );

        let report = validate_topology(&topology);

        assert!(report.is_valid());
    }

    #[test]
    fn workload_validation_is_independent() {
        let requirements = valid_requirements();

        let report = validate_workload(&requirements);

        assert!(report.is_valid());
    }

    #[test]
    fn unavailable_backend_is_rejected() {
        let mut backend = backend();
        backend.metadata.status = BackendStatus::Maintenance;

        let report =
            validate_with_report(&backend, &valid_requirements());

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == ValidationCode::BackendUnavailable
        }));
    }

    #[test]
    fn resource_limit_is_rejected() {
        let mut backend = backend();

        backend.limits =
            BackendLimits::unlimited().with_max_qubits(1);

        let requirements = valid_requirements();

        let report =
            validate_with_report(&backend, &requirements);

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == ValidationCode::QubitLimitExceeded
        }));
    }

    #[test]
    fn normalization_is_deterministic() {
        assert_eq!(
            normalize_gate_name("  cx "),
            "CX"
        );

        assert_eq!(
            normalize_gate_name("rZ"),
            "RZ"
        );
    }

    #[test]
    fn strict_validation_returns_backend_compatible_error() {
        let mut requirements = valid_requirements();
        requirements.qubit_count = 0;

        let error =
            validate(&backend(), &requirements)
                .expect_err("zero-qubit workload must fail");

        assert_eq!(error, BackendError::ZeroQubits);
    }

    #[test]
    fn report_collects_multiple_failures() {
        let mut backend = backend();
        backend.capabilities.measurement = false;
        backend.limits =
            BackendLimits::unlimited().with_max_qubits(1);

        let mut requirements = valid_requirements();
        requirements.qubit_count = 2;
        requirements.shots = 0;
        requirements.gates.push(
            "UNKNOWN_GATE".to_owned(),
        );

        let report =
            validate_with_report(&backend, &requirements);

        assert!(report.diagnostics.len() >= 3);
        assert!(report.has_blocking_errors());
    }

    #[test]
    fn production_policy_is_conservative() {
        let policy = ValidationPolicy::production();

        assert!(policy.require_available_backend);
        assert!(policy.report_directional_routing);
        assert!(!policy.report_native_gates);
    }

    #[test]
    fn validation_schema_is_stable() {
        assert_eq!(
            VALIDATION_SCHEMA_ID,
            "zamani.quantum.hardware.validation"
        );

        assert_eq!(VALIDATION_SCHEMA_VERSION, 1);
    }
}