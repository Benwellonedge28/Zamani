//! Zamani Quantum Benchmarking — Workload Model
//!
//! Defines the backend-independent workloads that benchmarking operates on.
//!
//! # Architectural role
//!
//! `Workload` is the semantic description of *what is being benchmarked*.
//! It deliberately does not:
//!
//! - execute quantum programs;
//! - select a hardware backend;
//! - perform routing;
//! - schedule pulses;
//! - perform calibration;
//! - perform statistical analysis;
//! - calculate benchmark metrics;
//! - communicate with a QPU;
//! - parse Zamani source code;
//! - implement individual benchmark protocols.
//!
//! Those responsibilities belong to the corresponding benchmarking,
//! compiler, runtime, and hardware layers.
//!
//! The intended dependency direction is:
//!
//! ```text
//! Zamani source / benchmark declaration
//!                 │
//!                 ▼
//!       benchmark workload construction
//!                 │
//!                 ▼
//!     benchmarking::core::workload
//!                 │
//!        ┌────────┼───────────────┐
//!        ▼        ▼               ▼
//!   Quantum IR  Application      QEC
//!        │        │               │
//!        └────────┼───────────────┘
//!                 ▼
//!          Experiment model
//!                 │
//!                 ▼
//!             Execution
//!                 │
//!                 ▼
//!             Observation
//!                 │
//!                 ▼
//!              Analysis
//! ```
//!
//! # Design goals
//!
//! This module is designed to support the complete Zamani quantum benchmarking
//! architecture rather than only gate-model circuits.
//!
//! Supported workload families include:
//!
//! - gate-model circuit workloads;
//! - random-circuit workloads;
//! - deterministic circuit workloads;
//! - application workloads;
//! - hybrid quantum/classical workloads;
//! - quantum error-correction workloads;
//! - logical-qubit workloads;
//! - analog quantum workloads;
//! - quantum annealing workloads;
//! - sampling workloads;
//! - state-preparation workloads;
//! - custom Zamani workloads.
//!
//! This distinction is important because not every quantum technology exposes
//! the same execution abstraction. A benchmark must not assume that every
//! target has:
//!
//! - qubits in the gate-model sense;
//! - a gate set;
//! - a circuit depth;
//! - computational-basis measurements;
//! - shots;
//! - a digital schedule.
//!
//! # Relationship with Quantum IR
//!
//! The canonical logical circuit representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! Benchmarking consumes that representation. It does not define a second
//! quantum circuit representation.
//!
//! `CircuitWorkload` therefore stores an `Arc<QuantumCircuit>` rather than
//! duplicating gates, measurements, parameters, or IR semantics.
//!
//! # Relationship with future benchmarking modules
//!
//! Later modules integrate with this file as follows:
//!
//! - `core::experiment` consumes `Workload`.
//! - `core::circuit` may provide additional benchmark-specific circuit
//!   metadata around `CircuitWorkload`.
//! - `core::execution` converts a workload into execution requests.
//! - `core::observation` stores execution observations.
//! - `protocols::*` create and analyze protocol-specific workloads.
//! - `applications::*` construct application workloads.
//! - `qec::*` construct error-correction workloads.
//! - `generators::*` generate circuit/application instances.
//! - `core::result` records the workload identity and metadata.
//! - `core::provenance` records workload identity/fingerprints.
//!
//! No later module should need to redefine the fundamental workload taxonomy.
//!
//! # Security/resource model
//!
//! Workloads are untrusted at public boundaries. Consequently this module:
//!
//! - bounds identifiers;
//! - bounds textual metadata;
//! - rejects empty required identifiers;
//! - rejects zero-sized workloads where nonsensical;
//! - rejects non-finite numerical workload parameters;
//! - uses checked arithmetic for derived resource quantities;
//! - does not allocate from user-controlled unbounded sizes;
//! - does not execute workload code;
//! - does not perform network or filesystem operations.
//!
//! Resource limits that are global to the benchmark engine belong in
//! `core::limits`. This file only validates invariants intrinsic to a workload.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.
//!
//! This file intentionally does not require serde. Serialization belongs to
//! the reporting/result layer so that the semantic workload model remains
//! dependency-light and usable during compiler/runtime bootstrap.

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Stable limits
// =============================================================================

/// Maximum UTF-8 byte length of a workload identifier.
pub const MAX_WORKLOAD_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a workload display name.
pub const MAX_WORKLOAD_NAME_BYTES: usize = 256;

/// Maximum UTF-8 byte length of a workload description.
pub const MAX_WORKLOAD_DESCRIPTION_BYTES: usize = 4096;

/// Maximum UTF-8 byte length of a custom workload kind identifier.
pub const MAX_CUSTOM_WORKLOAD_KIND_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an application identifier.
pub const MAX_APPLICATION_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a QEC code identifier.
pub const MAX_QEC_CODE_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an analog model identifier.
pub const MAX_ANALOG_MODEL_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an annealing model identifier.
pub const MAX_ANNEALING_MODEL_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a sampling model identifier.
pub const MAX_SAMPLING_MODEL_ID_BYTES: usize = 128;

/// Maximum number of custom workload tags.
pub const MAX_WORKLOAD_TAGS: usize = 64;

/// Maximum UTF-8 byte length of one workload tag.
pub const MAX_WORKLOAD_TAG_BYTES: usize = 64;

/// Maximum number of application input parameters stored directly in a
/// workload descriptor.
pub const MAX_APPLICATION_PARAMETERS: usize = 256;

/// Maximum UTF-8 byte length of an application parameter name.
pub const MAX_APPLICATION_PARAMETER_NAME_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an application parameter value.
pub const MAX_APPLICATION_PARAMETER_VALUE_BYTES: usize = 512;

/// Maximum number of logical dimensions attached to a workload.
pub const MAX_WORKLOAD_DIMENSIONS: usize = 64;

/// Maximum UTF-8 byte length of a workload dimension name.
pub const MAX_WORKLOAD_DIMENSION_NAME_BYTES: usize = 128;

// =============================================================================
// Error vocabulary
// =============================================================================

/// Errors raised while constructing or validating a benchmark workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkloadError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Name of the offending field.
        field: &'static str,
    },

    /// An identifier exceeded its bounded size.
    IdentifierTooLong {
        /// Name of the offending field.
        field: &'static str,

        /// Actual UTF-8 byte length.
        length: usize,

        /// Maximum accepted UTF-8 byte length.
        maximum: usize,
    },

    /// An identifier contains an invalid character.
    InvalidIdentifierCharacter {
        /// Name of the offending field.
        field: &'static str,

        /// Zero-based byte position.
        position: usize,

        /// Offending byte.
        byte: u8,
    },

    /// An identifier does not begin with the required character.
    InvalidIdentifierStart {
        /// Name of the offending field.
        field: &'static str,
    },

    /// A required numerical quantity was zero.
    ZeroValue {
        /// Name of the offending field.
        field: &'static str,
    },

    /// A numerical value was not finite.
    NonFiniteValue {
        /// Name of the offending field.
        field: &'static str,
    },

    /// A numerical value was outside the allowed range.
    ValueOutOfRange {
        /// Name of the offending field.
        field: &'static str,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Name of the calculation.
        calculation: &'static str,
    },

    /// Too many tags were supplied.
    TooManyTags {
        /// Actual number of tags.
        count: usize,

        /// Maximum accepted count.
        maximum: usize,
    },

    /// A tag is too long.
    TagTooLong {
        /// UTF-8 byte length.
        length: usize,

        /// Maximum accepted length.
        maximum: usize,
    },

    /// Too many application parameters were supplied.
    TooManyApplicationParameters {
        /// Actual number of parameters.
        count: usize,

        /// Maximum accepted count.
        maximum: usize,
    },

    /// Too many workload dimensions were supplied.
    TooManyDimensions {
        /// Actual number of dimensions.
        count: usize,

        /// Maximum accepted count.
        maximum: usize,
    },

    /// A required application parameter name is invalid.
    InvalidApplicationParameterName,

    /// An application parameter value is too large.
    ApplicationParameterValueTooLong {
        /// UTF-8 byte length.
        length: usize,

        /// Maximum accepted length.
        maximum: usize,
    },

    /// The workload contains incompatible configuration.
    InvalidCombination {
        /// Static description of the invalid combination.
        message: &'static str,
    },
}

impl fmt::Display for WorkloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => write!(
                f,
                "{field} is {length} bytes; maximum is {maximum} bytes"
            ),

            Self::InvalidIdentifierCharacter {
                field,
                position,
                byte,
            } => write!(
                f,
                "invalid byte 0x{byte:02x} at position {position} in {field}"
            ),

            Self::InvalidIdentifierStart { field } => write!(
                f,
                "{field} must begin with a lowercase ASCII letter"
            ),

            Self::ZeroValue { field } => {
                write!(f, "{field} must be greater than zero")
            }

            Self::NonFiniteValue { field } => {
                write!(f, "{field} must be finite")
            }

            Self::ValueOutOfRange { field } => {
                write!(f, "{field} is outside its permitted range")
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(f, "arithmetic overflow while calculating {calculation}")
            }

            Self::TooManyTags { count, maximum } => write!(
                f,
                "workload contains {count} tags; maximum is {maximum}"
            ),

            Self::TagTooLong { length, maximum } => write!(
                f,
                "workload tag is {length} bytes; maximum is {maximum}"
            ),

            Self::TooManyApplicationParameters { count, maximum } => write!(
                f,
                "application workload contains {count} parameters; maximum is {maximum}"
            ),

            Self::TooManyDimensions { count, maximum } => write!(
                f,
                "workload contains {count} dimensions; maximum is {maximum}"
            ),

            Self::InvalidApplicationParameterName => {
                f.write_str("application parameter name is invalid")
            }

            Self::ApplicationParameterValueTooLong { length, maximum } => {
                write!(
                    f,
                    "application parameter value is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidCombination { message } => {
                f.write_str(message)
            }
        }
    }
}

impl std::error::Error for WorkloadError {}

// =============================================================================
// Workload identity
// =============================================================================

/// Stable identity of a benchmark workload.
///
/// The identity is deliberately independent from execution backend identity.
/// The same workload can therefore be executed against:
///
/// - a CPU simulator;
/// - a GPU simulator;
/// - a superconducting QPU;
/// - a trapped-ion QPU;
/// - a neutral-atom system;
/// - a photonic system;
/// - an annealer;
/// - an analog device;
/// - a logical-qubit backend.
///
/// Backend identity belongs to provenance/execution metadata.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkloadId(String);

impl WorkloadId {
    /// Creates a validated workload identifier.
    ///
    /// Identifiers use:
    ///
    /// `[a-z][a-z0-9_]*`
    pub fn new<S: Into<String>>(value: S) -> Result<Self, WorkloadError> {
        let value = value.into();

        validate_identifier(
            "workload_id",
            &value,
            MAX_WORKLOAD_ID_BYTES,
        )?;

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for WorkloadId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Workload kind
// =============================================================================

/// Top-level workload category.
///
/// This is intentionally broader than "circuit".
///
/// A benchmark framework that assumes every quantum computer executes digital
/// circuits cannot faithfully represent analog, annealing, or other quantum
/// execution models.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkloadKind {
    /// A normal logical quantum circuit represented by Quantum IR.
    Circuit,

    /// A deterministic circuit workload.
    DeterministicCircuit,

    /// A random-circuit workload.
    RandomCircuit,

    /// An application-level quantum workload.
    Application,

    /// A hybrid quantum/classical workload.
    Hybrid,

    /// A quantum error-correction workload.
    ErrorCorrection,

    /// A logical-qubit/fault-tolerant workload.
    Logical,

    /// An analog quantum workload.
    Analog,

    /// A quantum annealing workload.
    Annealing,

    /// A sampling workload.
    Sampling,

    /// A state-preparation workload.
    StatePreparation,

    /// A user-defined workload.
    Custom(String),
}

impl WorkloadKind {
    /// Returns the stable machine identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Circuit => "circuit",
            Self::DeterministicCircuit => "deterministic_circuit",
            Self::RandomCircuit => "random_circuit",
            Self::Application => "application",
            Self::Hybrid => "hybrid",
            Self::ErrorCorrection => "error_correction",
            Self::Logical => "logical",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::StatePreparation => "state_preparation",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether this kind represents a custom workload.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Creates a validated custom workload kind.
    pub fn custom<S: Into<String>>(value: S) -> Result<Self, WorkloadError> {
        let value = value.into();

        validate_identifier(
            "custom_workload_kind",
            &value,
            MAX_CUSTOM_WORKLOAD_KIND_BYTES,
        )?;

        Ok(Self::Custom(value))
    }

    /// Returns whether the workload is circuit-backed.
    ///
    /// This includes circuit, deterministic-circuit, random-circuit,
    /// application, hybrid, error-correction, logical and state-preparation
    /// workloads that may carry Quantum IR.
    #[must_use]
    pub fn may_contain_circuit(&self) -> bool {
        matches!(
            self,
            Self::Circuit
                | Self::DeterministicCircuit
                | Self::RandomCircuit
                | Self::Application
                | Self::Hybrid
                | Self::ErrorCorrection
                | Self::Logical
                | Self::StatePreparation
        )
    }
}

impl fmt::Display for WorkloadKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Execution model
// =============================================================================

/// Execution model required by a workload.
///
/// This allows capability negotiation before execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionModel {
    /// Digital gate-model execution.
    GateModel,

    /// Analog quantum evolution.
    Analog,

    /// Quantum annealing / adiabatic execution.
    Annealing,

    /// Sampling-oriented execution.
    Sampling,

    /// Hybrid quantum/classical execution.
    Hybrid,

    /// Logical/fault-tolerant execution.
    Logical,

    /// Classical simulation/emulation of a quantum workload.
    Simulation,

    /// Workload-specific execution model.
    Custom,
}

impl ExecutionModel {
    /// Returns a stable machine identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::Hybrid => "hybrid",
            Self::Logical => "logical",
            Self::Simulation => "simulation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ExecutionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

// =============================================================================
// Circuit workload
// =============================================================================

/// A Quantum IR-backed benchmark workload.
///
/// The canonical circuit is owned by Quantum IR. Benchmarking stores an
/// `Arc` so the same circuit can safely be shared by:
///
/// - execution;
/// - resource analysis;
/// - reporting;
/// - reproducibility/fingerprinting;
/// - multiple benchmark protocol stages.
///
/// This type does not duplicate circuit operations.
#[derive(Debug, Clone)]
pub struct CircuitWorkload {
    circuit: Arc<QuantumCircuit>,
    instance_id: WorkloadId,
}

impl CircuitWorkload {
    /// Creates a circuit workload.
    pub fn new(
        circuit: Arc<QuantumCircuit>,
        instance_id: WorkloadId,
    ) -> Result<Self, WorkloadError> {
        if instance_id.as_str().is_empty() {
            return Err(WorkloadError::EmptyIdentifier {
                field: "circuit workload instance_id",
            });
        }

        Ok(Self {
            circuit,
            instance_id,
        })
    }

    /// Creates a circuit workload from an owned Quantum IR circuit.
    pub fn from_circuit(
        circuit: QuantumCircuit,
        instance_id: WorkloadId,
    ) -> Result<Self, WorkloadError> {
        Self::new(Arc::new(circuit), instance_id)
    }

    /// Returns the canonical Quantum IR circuit.
    #[must_use]
    pub fn circuit(&self) -> &QuantumCircuit {
        self.circuit.as_ref()
    }

    /// Returns a shared reference to the canonical circuit.
    #[must_use]
    pub fn circuit_arc(&self) -> &Arc<QuantumCircuit> {
        &self.circuit
    }

    /// Returns the workload instance identity.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        &self.instance_id
    }

    /// Clones the shared circuit handle without cloning circuit contents.
    #[must_use]
    pub fn shared_circuit(&self) -> Arc<QuantumCircuit> {
        Arc::clone(&self.circuit)
    }
}

// =============================================================================
// Application workload
// =============================================================================

/// A bounded application parameter.
///
/// Values are represented as strings intentionally.
///
/// Benchmarking should not impose an application-language-specific numerical
/// type system on application parameters. The Zamani frontend/application
/// layer can encode structured values into canonical textual representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationParameter {
    name: String,
    value: String,
}

impl ApplicationParameter {
    /// Creates a validated application parameter.
    pub fn new<N: Into<String>, V: Into<String>>(
        name: N,
        value: V,
    ) -> Result<Self, WorkloadError> {
        let name = name.into();
        let value = value.into();

        validate_identifier(
            "application_parameter_name",
            &name,
            MAX_APPLICATION_PARAMETER_NAME_BYTES,
        )?;

        if value.len() > MAX_APPLICATION_PARAMETER_VALUE_BYTES {
            return Err(
                WorkloadError::ApplicationParameterValueTooLong {
                    length: value.len(),
                    maximum: MAX_APPLICATION_PARAMETER_VALUE_BYTES,
                },
            );
        }

        Ok(Self { name, value })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the encoded parameter value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// An application-level quantum workload.
///
/// This represents workloads such as:
///
/// - Grover;
/// - QFT;
/// - Shor;
/// - VQE;
/// - QAOA;
/// - MaxCut;
/// - Hamiltonian simulation;
/// - amplitude estimation;
/// - quantum Monte Carlo;
/// - user-defined Zamani algorithms.
///
/// The application identity is separate from the generated circuit. This is
/// necessary because the same application can produce different circuits after
/// optimization, routing, compilation, approximation, or hardware lowering.
#[derive(Debug, Clone)]
pub struct ApplicationWorkload {
    application_id: String,
    instance_id: WorkloadId,
    problem_size: usize,
    parameters: Vec<ApplicationParameter>,
    circuit: Option<CircuitWorkload>,
}

impl ApplicationWorkload {
    /// Creates an application workload without an attached circuit.
    ///
    /// This is valid for:
    ///
    /// - application-level resource estimation;
    /// - classical pre-processing;
    /// - analog applications;
    /// - workloads whose circuit is generated later;
    /// - workloads represented by a non-circuit execution model.
    pub fn new<A: Into<String>>(
        application_id: A,
        instance_id: WorkloadId,
        problem_size: usize,
    ) -> Result<Self, WorkloadError> {
        let application_id = application_id.into();

        validate_identifier(
            "application_id",
            &application_id,
            MAX_APPLICATION_ID_BYTES,
        )?;

        if problem_size == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "application problem_size",
            });
        }

        Ok(Self {
            application_id,
            instance_id,
            problem_size,
            parameters: Vec::new(),
            circuit: None,
        })
    }

    /// Attaches a generated Quantum IR circuit.
    #[must_use]
    pub fn with_circuit(mut self, circuit: CircuitWorkload) -> Self {
        self.circuit = Some(circuit);
        self
    }

    /// Adds an application parameter.
    pub fn add_parameter(
        &mut self,
        parameter: ApplicationParameter,
    ) -> Result<(), WorkloadError> {
        if self.parameters.len() >= MAX_APPLICATION_PARAMETERS {
            return Err(
                WorkloadError::TooManyApplicationParameters {
                    count: self.parameters.len() + 1,
                    maximum: MAX_APPLICATION_PARAMETERS,
                },
            );
        }

        self.parameters.push(parameter);
        Ok(())
    }

    /// Returns the stable application identifier.
    #[must_use]
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    /// Returns the application instance identity.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        &self.instance_id
    }

    /// Returns the problem size.
    #[must_use]
    pub fn problem_size(&self) -> usize {
        self.problem_size
    }

    /// Returns application parameters.
    #[must_use]
    pub fn parameters(&self) -> &[ApplicationParameter] {
        &self.parameters
    }

    /// Returns the generated circuit, if one has already been attached.
    #[must_use]
    pub fn circuit(&self) -> Option<&CircuitWorkload> {
        self.circuit.as_ref()
    }
}

// =============================================================================
// Hybrid workload
// =============================================================================

/// Configuration of the classical portion of a hybrid workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridExecutionProfile {
    /// Maximum number of classical optimization iterations.
    pub max_iterations: usize,

    /// Whether classical convergence is part of the benchmark success
    /// criterion.
    pub convergence_required: bool,
}

impl HybridExecutionProfile {
    /// Creates a validated hybrid execution profile.
    pub fn new(
        max_iterations: usize,
        convergence_required: bool,
    ) -> Result<Self, WorkloadError> {
        if max_iterations == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "hybrid max_iterations",
            });
        }

        Ok(Self {
            max_iterations,
            convergence_required,
        })
    }
}

/// A hybrid quantum/classical workload.
///
/// This is required for VQE, QAOA and similar workloads where quantum
/// execution cannot be represented adequately by a single circuit.
#[derive(Debug, Clone)]
pub struct HybridWorkload {
    application: ApplicationWorkload,
    profile: HybridExecutionProfile,
}

impl HybridWorkload {
    /// Creates a hybrid workload.
    pub fn new(
        application: ApplicationWorkload,
        profile: HybridExecutionProfile,
    ) -> Self {
        Self {
            application,
            profile,
        }
    }

    /// Returns the application component.
    #[must_use]
    pub fn application(&self) -> &ApplicationWorkload {
        &self.application
    }

    /// Returns the classical execution profile.
    #[must_use]
    pub fn profile(&self) -> HybridExecutionProfile {
        self.profile
    }
}

// =============================================================================
// Error-correction workload
// =============================================================================

/// Error-correction workload configuration.
///
/// This describes the benchmarked QEC experiment without embedding a
/// particular decoder implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCorrectionWorkload {
    code_id: String,
    instance_id: WorkloadId,
    distance: usize,
    rounds: usize,
    logical_qubits: usize,
    physical_qubits: usize,
}

impl ErrorCorrectionWorkload {
    /// Creates a QEC workload.
    ///
    /// `physical_qubits` is explicit rather than inferred because different
    /// codes and implementations can have different resource geometries.
    pub fn new<C: Into<String>>(
        code_id: C,
        instance_id: WorkloadId,
        distance: usize,
        rounds: usize,
        logical_qubits: usize,
        physical_qubits: usize,
    ) -> Result<Self, WorkloadError> {
        let code_id = code_id.into();

        validate_identifier(
            "qec_code_id",
            &code_id,
            MAX_QEC_CODE_ID_BYTES,
        )?;

        if distance == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "qec distance",
            });
        }

        if rounds == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "qec rounds",
            });
        }

        if logical_qubits == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "qec logical_qubits",
            });
        }

        if physical_qubits == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "qec physical_qubits",
            });
        }

        if physical_qubits < logical_qubits {
            return Err(WorkloadError::InvalidCombination {
                message: "qec physical_qubits cannot be smaller than logical_qubits",
            });
        }

        Ok(Self {
            code_id,
            instance_id,
            distance,
            rounds,
            logical_qubits,
            physical_qubits,
        })
    }

    /// Returns the error-correction code identifier.
    #[must_use]
    pub fn code_id(&self) -> &str {
        &self.code_id
    }

    /// Returns the workload instance identifier.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        &self.instance_id
    }

    /// Returns code distance.
    #[must_use]
    pub fn distance(&self) -> usize {
        self.distance
    }

    /// Returns syndrome-extraction rounds.
    #[must_use]
    pub fn rounds(&self) -> usize {
        self.rounds
    }

    /// Returns logical-qubit count.
    #[must_use]
    pub fn logical_qubits(&self) -> usize {
        self.logical_qubits
    }

    /// Returns physical-qubit count.
    #[must_use]
    pub fn physical_qubits(&self) -> usize {
        self.physical_qubits
    }

    /// Returns the total number of logical-round observations expected from
    /// this workload.
    pub fn logical_rounds(&self) -> Result<usize, WorkloadError> {
        self.logical_qubits
            .checked_mul(self.rounds)
            .ok_or(WorkloadError::ArithmeticOverflow {
                calculation: "logical qubits × QEC rounds",
            })
    }
}

// =============================================================================
// Logical workload
// =============================================================================

/// A fault-tolerant logical workload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalWorkload {
    instance_id: WorkloadId,
    logical_qubits: usize,
    logical_depth: usize,
    physical_qubits: usize,
}

impl LogicalWorkload {
    /// Creates a logical workload descriptor.
    pub fn new(
        instance_id: WorkloadId,
        logical_qubits: usize,
        logical_depth: usize,
        physical_qubits: usize,
    ) -> Result<Self, WorkloadError> {
        if logical_qubits == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "logical_qubits",
            });
        }

        if logical_depth == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "logical_depth",
            });
        }

        if physical_qubits == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "physical_qubits",
            });
        }

        if physical_qubits < logical_qubits {
            return Err(WorkloadError::InvalidCombination {
                message:
                    "logical workload physical_qubits cannot be smaller than logical_qubits",
            });
        }

        Ok(Self {
            instance_id,
            logical_qubits,
            logical_depth,
            physical_qubits,
        })
    }

    /// Returns logical-qubit count.
    #[must_use]
    pub fn logical_qubits(&self) -> usize {
        self.logical_qubits
    }

    /// Returns logical depth.
    #[must_use]
    pub fn logical_depth(&self) -> usize {
        self.logical_depth
    }

    /// Returns physical-qubit allocation.
    #[must_use]
    pub fn physical_qubits(&self) -> usize {
        self.physical_qubits
    }

    /// Returns the logical-to-physical qubit overhead.
    pub fn qubit_overhead(&self) -> Result<f64, WorkloadError> {
        if self.logical_qubits == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "logical_qubits",
            });
        }

        Ok(self.physical_qubits as f64 / self.logical_qubits as f64)
    }
}

// =============================================================================
// Analog workload
// =============================================================================

/// Analog quantum workload descriptor.
///
/// The benchmark does not assume a particular Hamiltonian, pulse language, or
/// physical implementation. The model identifier is supplied by the analog
/// backend/domain layer.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogWorkload {
    model_id: String,
    instance_id: WorkloadId,
    system_size: usize,
    evolution_time: f64,
    repetitions: usize,
}

impl AnalogWorkload {
    /// Creates an analog workload.
    pub fn new<M: Into<String>>(
        model_id: M,
        instance_id: WorkloadId,
        system_size: usize,
        evolution_time: f64,
        repetitions: usize,
    ) -> Result<Self, WorkloadError> {
        let model_id = model_id.into();

        validate_identifier(
            "analog_model_id",
            &model_id,
            MAX_ANALOG_MODEL_ID_BYTES,
        )?;

        if system_size == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "analog system_size",
            });
        }

        if !evolution_time.is_finite() || evolution_time <= 0.0 {
            return Err(WorkloadError::ValueOutOfRange {
                field: "analog evolution_time",
            });
        }

        if repetitions == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "analog repetitions",
            });
        }

        Ok(Self {
            model_id,
            instance_id,
            system_size,
            evolution_time,
            repetitions,
        })
    }

    /// Returns the analog model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    /// Returns system size.
    #[must_use]
    pub fn system_size(&self) -> usize {
        self.system_size
    }

    /// Returns evolution time.
    #[must_use]
    pub fn evolution_time(&self) -> f64 {
        self.evolution_time
    }

    /// Returns repetition count.
    #[must_use]
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

// =============================================================================
// Annealing workload
// =============================================================================

/// Quantum annealing workload descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnealingWorkload {
    model_id: String,
    instance_id: WorkloadId,
    variables: usize,
    anneal_time: f64,
    repetitions: usize,
}

impl AnnealingWorkload {
    /// Creates an annealing workload.
    pub fn new<M: Into<String>>(
        model_id: M,
        instance_id: WorkloadId,
        variables: usize,
        anneal_time: f64,
        repetitions: usize,
    ) -> Result<Self, WorkloadError> {
        let model_id = model_id.into();

        validate_identifier(
            "annealing_model_id",
            &model_id,
            MAX_ANNEALING_MODEL_ID_BYTES,
        )?;

        if variables == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "annealing variables",
            });
        }

        if !anneal_time.is_finite() || anneal_time <= 0.0 {
            return Err(WorkloadError::ValueOutOfRange {
                field: "annealing anneal_time",
            });
        }

        if repetitions == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "annealing repetitions",
            });
        }

        Ok(Self {
            model_id,
            instance_id,
            variables,
            anneal_time,
            repetitions,
        })
    }

    /// Returns the annealing model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    /// Returns the problem-variable count.
    #[must_use]
    pub fn variables(&self) -> usize {
        self.variables
    }

    /// Returns anneal time.
    #[must_use]
    pub fn anneal_time(&self) -> f64 {
        self.anneal_time
    }

    /// Returns repetition count.
    #[must_use]
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

// =============================================================================
// Sampling workload
// =============================================================================

/// Sampling workload descriptor.
///
/// This deliberately does not require a circuit. Some quantum sampling
/// systems expose sampling as the primary abstraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SamplingWorkload {
    model_id: String,
    instance_id: WorkloadId,
    sample_width: usize,
    samples: usize,
}

impl SamplingWorkload {
    /// Creates a sampling workload.
    pub fn new<M: Into<String>>(
        model_id: M,
        instance_id: WorkloadId,
        sample_width: usize,
        samples: usize,
    ) -> Result<Self, WorkloadError> {
        let model_id = model_id.into();

        validate_identifier(
            "sampling_model_id",
            &model_id,
            MAX_SAMPLING_MODEL_ID_BYTES,
        )?;

        if sample_width == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "sampling sample_width",
            });
        }

        if samples == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "sampling samples",
            });
        }

        Ok(Self {
            model_id,
            instance_id,
            sample_width,
            samples,
        })
    }

    /// Returns the sampling model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    /// Returns sample width.
    #[must_use]
    pub fn sample_width(&self) -> usize {
        self.sample_width
    }

    /// Returns requested sample count.
    #[must_use]
    pub fn samples(&self) -> usize {
        self.samples
    }
}

// =============================================================================
// State-preparation workload
// =============================================================================

/// State-preparation workload.
///
/// This is distinct from a generic circuit workload because the benchmark
/// objective is the prepared quantum state itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatePreparationWorkload {
    instance_id: WorkloadId,
    state_dimension: usize,
    repetitions: usize,
}

impl StatePreparationWorkload {
    /// Creates a state-preparation workload.
    pub fn new(
        instance_id: WorkloadId,
        state_dimension: usize,
        repetitions: usize,
    ) -> Result<Self, WorkloadError> {
        if state_dimension == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "state_preparation state_dimension",
            });
        }

        if repetitions == 0 {
            return Err(WorkloadError::ZeroValue {
                field: "state_preparation repetitions",
            });
        }

        Ok(Self {
            instance_id,
            state_dimension,
            repetitions,
        })
    }

    /// Returns state dimension.
    #[must_use]
    pub fn state_dimension(&self) -> usize {
        self.state_dimension
    }

    /// Returns repetitions.
    #[must_use]
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

// =============================================================================
// Custom workload
// =============================================================================

/// A bounded user-defined benchmark dimension.
///
/// This is intentionally small and structured. Arbitrary executable data must
/// not be embedded in the workload model.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkloadDimension {
    name: String,
    value: f64,
}

impl WorkloadDimension {
    /// Creates a validated workload dimension.
    pub fn new<N: Into<String>>(
        name: N,
        value: f64,
    ) -> Result<Self, WorkloadError> {
        let name = name.into();

        validate_identifier(
            "workload_dimension_name",
            &name,
            MAX_WORKLOAD_DIMENSION_NAME_BYTES,
        )?;

        if !value.is_finite() {
            return Err(WorkloadError::NonFiniteValue {
                field: "workload dimension value",
            });
        }

        Ok(Self { name, value })
    }

    /// Returns dimension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns dimension value.
    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }
}

/// User-defined workload.
///
/// Custom workloads allow the Zamani language to introduce new benchmark
/// domains without changing the core taxonomy every time a new research
/// workload appears.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomWorkload {
    kind: WorkloadKind,
    instance_id: WorkloadId,
    dimensions: Vec<WorkloadDimension>,
}

impl CustomWorkload {
    /// Creates a custom workload.
    pub fn new(
        kind: WorkloadKind,
        instance_id: WorkloadId,
    ) -> Result<Self, WorkloadError> {
        if !kind.is_custom() {
            return Err(WorkloadError::InvalidCombination {
                message:
                    "CustomWorkload requires WorkloadKind::Custom",
            });
        }

        Ok(Self {
            kind,
            instance_id,
            dimensions: Vec::new(),
        })
    }

    /// Adds a custom dimension.
    pub fn add_dimension(
        &mut self,
        dimension: WorkloadDimension,
    ) -> Result<(), WorkloadError> {
        if self.dimensions.len() >= MAX_WORKLOAD_DIMENSIONS {
            return Err(WorkloadError::TooManyDimensions {
                count: self.dimensions.len() + 1,
                maximum: MAX_WORKLOAD_DIMENSIONS,
            });
        }

        self.dimensions.push(dimension);
        Ok(())
    }

    /// Returns the custom workload kind.
    #[must_use]
    pub fn kind(&self) -> &WorkloadKind {
        &self.kind
    }

    /// Returns the instance identity.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        &self.instance_id
    }

    /// Returns custom dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[WorkloadDimension] {
        &self.dimensions
    }
}

// =============================================================================
// Top-level workload
// =============================================================================

/// The complete backend-independent workload envelope.
///
/// Every benchmark experiment should eventually operate on this type rather
/// than defining another workload representation.
///
/// The enum is deliberately explicit instead of using `Box<dyn Any>`:
///
/// - it makes supported workload classes discoverable;
/// - it prevents type confusion;
/// - it makes validation deterministic;
/// - it permits stable serialization later;
/// - it makes capability negotiation possible;
/// - it avoids runtime downcasting;
/// - it keeps the Zamani language boundary predictable.
#[derive(Debug, Clone)]
pub enum Workload {
    /// Quantum IR-backed circuit.
    Circuit(CircuitWorkload),

    /// Deterministic circuit.
    DeterministicCircuit(CircuitWorkload),

    /// Randomly generated circuit.
    RandomCircuit(CircuitWorkload),

    /// Application-level workload.
    Application(ApplicationWorkload),

    /// Hybrid quantum/classical workload.
    Hybrid(HybridWorkload),

    /// Quantum error-correction workload.
    ErrorCorrection(ErrorCorrectionWorkload),

    /// Logical/fault-tolerant workload.
    Logical(LogicalWorkload),

    /// Analog quantum workload.
    Analog(AnalogWorkload),

    /// Quantum annealing workload.
    Annealing(AnnealingWorkload),

    /// Sampling workload.
    Sampling(SamplingWorkload),

    /// State-preparation workload.
    StatePreparation(StatePreparationWorkload),

    /// User-defined workload.
    Custom(CustomWorkload),
}

impl Workload {
    /// Returns the workload kind.
    #[must_use]
    pub fn kind(&self) -> WorkloadKind {
        match self {
            Self::Circuit(_) => WorkloadKind::Circuit,
            Self::DeterministicCircuit(_) => {
                WorkloadKind::DeterministicCircuit
            }
            Self::RandomCircuit(_) => WorkloadKind::RandomCircuit,
            Self::Application(_) => WorkloadKind::Application,
            Self::Hybrid(_) => WorkloadKind::Hybrid,
            Self::ErrorCorrection(_) => WorkloadKind::ErrorCorrection,
            Self::Logical(_) => WorkloadKind::Logical,
            Self::Analog(_) => WorkloadKind::Analog,
            Self::Annealing(_) => WorkloadKind::Annealing,
            Self::Sampling(_) => WorkloadKind::Sampling,
            Self::StatePreparation(_) => WorkloadKind::StatePreparation,
            Self::Custom(value) => value.kind().clone(),
        }
    }

    /// Returns the stable workload kind identifier.
    #[must_use]
    pub fn kind_id(&self) -> &str {
        match self {
            Self::Circuit(_) => "circuit",
            Self::DeterministicCircuit(_) => "deterministic_circuit",
            Self::RandomCircuit(_) => "random_circuit",
            Self::Application(_) => "application",
            Self::Hybrid(_) => "hybrid",
            Self::ErrorCorrection(_) => "error_correction",
            Self::Logical(_) => "logical",
            Self::Analog(_) => "analog",
            Self::Annealing(_) => "annealing",
            Self::Sampling(_) => "sampling",
            Self::StatePreparation(_) => "state_preparation",
            Self::Custom(value) => value.kind().id(),
        }
    }

    /// Returns the execution model naturally associated with this workload.
    ///
    /// Backend-specific capability negotiation can refine this later.
    #[must_use]
    pub fn execution_model(&self) -> ExecutionModel {
        match self {
            Self::Circuit(_)
            | Self::DeterministicCircuit(_)
            | Self::RandomCircuit(_) => ExecutionModel::GateModel,

            Self::Application(application) => {
                if application.circuit().is_some() {
                    ExecutionModel::GateModel
                } else {
                    ExecutionModel::Hybrid
                }
            }

            Self::Hybrid(_) => ExecutionModel::Hybrid,

            Self::ErrorCorrection(_) | Self::Logical(_) => {
                ExecutionModel::Logical
            }

            Self::Analog(_) => ExecutionModel::Analog,

            Self::Annealing(_) => ExecutionModel::Annealing,

            Self::Sampling(_) => ExecutionModel::Sampling,

            Self::StatePreparation(_) => ExecutionModel::GateModel,

            Self::Custom(_) => ExecutionModel::Custom,
        }
    }

    /// Returns the primary workload instance identity.
    #[must_use]
    pub fn instance_id(&self) -> &WorkloadId {
        match self {
            Self::Circuit(value)
            | Self::DeterministicCircuit(value)
            | Self::RandomCircuit(value) => value.instance_id(),

            Self::Application(value) => value.instance_id(),

            Self::Hybrid(value) => value.application().instance_id(),

            Self::ErrorCorrection(value) => value.instance_id(),

            Self::Logical(value) => &value.instance_id,

            Self::Analog(value) => &value.instance_id,

            Self::Annealing(value) => &value.instance_id,

            Self::Sampling(value) => &value.instance_id,

            Self::StatePreparation(value) => &value.instance_id,

            Self::Custom(value) => value.instance_id(),
        }
    }

    /// Returns whether the workload carries a canonical Quantum IR circuit.
    #[must_use]
    pub fn has_circuit(&self) -> bool {
        matches!(
            self,
            Self::Circuit(_)
                | Self::DeterministicCircuit(_)
                | Self::RandomCircuit(_)
                | Self::StatePreparation(_)
        ) || matches!(
            self,
            Self::Application(value) if value.circuit().is_some()
        ) || matches!(
            self,
            Self::Hybrid(value)
                if value.application().circuit().is_some()
        )
    }

    /// Returns the canonical Quantum IR circuit if this workload has one.
    ///
    /// No circuit is synthesized here. Generators own circuit construction.
    #[must_use]
    pub fn circuit(&self) -> Option<&QuantumCircuit> {
        match self {
            Self::Circuit(value)
            | Self::DeterministicCircuit(value)
            | Self::RandomCircuit(value) => Some(value.circuit()),

            Self::Application(value) => {
                value.circuit().map(CircuitWorkload::circuit)
            }

            Self::Hybrid(value) => value
                .application()
                .circuit()
                .map(CircuitWorkload::circuit),

            Self::StatePreparation(_) => None,

            Self::ErrorCorrection(_)
            | Self::Logical(_)
            | Self::Analog(_)
            | Self::Annealing(_)
            | Self::Sampling(_)
            | Self::Custom(_) => None,
        }
    }

    /// Returns the shared canonical circuit handle if one exists.
    #[must_use]
    pub fn shared_circuit(&self) -> Option<Arc<QuantumCircuit>> {
        match self {
            Self::Circuit(value)
            | Self::DeterministicCircuit(value)
            | Self::RandomCircuit(value) => {
                Some(value.shared_circuit())
            }

            Self::Application(value) => {
                value.circuit().map(CircuitWorkload::shared_circuit)
            }

            Self::Hybrid(value) => value
                .application()
                .circuit()
                .map(CircuitWorkload::shared_circuit),

            Self::StatePreparation(_)
            | Self::ErrorCorrection(_)
            | Self::Logical(_)
            | Self::Analog(_)
            | Self::Annealing(_)
            | Self::Sampling(_)
            | Self::Custom(_) => None,
        }
    }

    /// Returns the number of logical qubits represented directly by the
    /// workload when that information is intrinsically known.
    ///
    /// `None` is deliberate. Analog and annealing systems must not be forced
    /// into a digital-qubit interpretation.
    #[must_use]
    pub fn logical_qubits(&self) -> Option<usize> {
        match self {
            Self::ErrorCorrection(value) => {
                Some(value.logical_qubits())
            }

            Self::Logical(value) => Some(value.logical_qubits()),

            Self::Application(value) => value
                .circuit()
                .and_then(|_| None),

            Self::Circuit(_)
            | Self::DeterministicCircuit(_)
            | Self::RandomCircuit(_)
            | Self::Hybrid(_)
            | Self::Analog(_)
            | Self::Annealing(_)
            | Self::Sampling(_)
            | Self::StatePreparation(_)
            | Self::Custom(_) => None,
        }
    }

    /// Returns the physical-qubit allocation if intrinsically known.
    #[must_use]
    pub fn physical_qubits(&self) -> Option<usize> {
        match self {
            Self::ErrorCorrection(value) => {
                Some(value.physical_qubits())
            }

            Self::Logical(value) => Some(value.physical_qubits()),

            Self::Circuit(_)
            | Self::DeterministicCircuit(_)
            | Self::RandomCircuit(_)
            | Self::Application(_)
            | Self::Hybrid(_)
            | Self::Analog(_)
            | Self::Annealing(_)
            | Self::Sampling(_)
            | Self::StatePreparation(_)
            | Self::Custom(_) => None,
        }
    }

    /// Performs workload-local validation.
    ///
    /// This does not replace Quantum IR validation, backend capability
    /// validation, or benchmark configuration validation.
    pub fn validate(&self) -> Result<(), WorkloadError> {
        match self {
            Self::Circuit(value)
            | Self::DeterministicCircuit(value)
            | Self::RandomCircuit(value) => {
                if value.instance_id().as_str().is_empty() {
                    return Err(WorkloadError::EmptyIdentifier {
                        field: "circuit instance_id",
                    });
                }
            }

            Self::Application(value) => {
                if value.problem_size() == 0 {
                    return Err(WorkloadError::ZeroValue {
                        field: "application problem_size",
                    });
                }
            }

            Self::Hybrid(value) => {
                value.application().validate()?;
            }

            Self::ErrorCorrection(value) => {
                if value.distance() == 0 {
                    return Err(WorkloadError::ZeroValue {
                        field: "qec distance",
                    });
                }
            }

            Self::Logical(value) => {
                if value.logical_qubits == 0 {
                    return Err(WorkloadError::ZeroValue {
                        field: "logical_qubits",
                    });
                }
            }

            Self::Analog(value) => {
                if !value.evolution_time().is_finite()
                    || value.evolution_time() <= 0.0
                {
                    return Err(WorkloadError::ValueOutOfRange {
                        field: "analog evolution_time",
                    });
                }
            }

            Self::Annealing(value) => {
                if !value.anneal_time().is_finite()
                    || value.anneal_time() <= 0.0
                {
                    return Err(WorkloadError::ValueOutOfRange {
                        field: "annealing anneal_time",
                    });
                }
            }

            Self::Sampling(value) => {
                if value.sample_width() == 0 {
                    return Err(WorkloadError::ZeroValue {
                        field: "sampling sample_width",
                    });
                }
            }

            Self::StatePreparation(value) => {
                if value.state_dimension() == 0 {
                    return Err(WorkloadError::ZeroValue {
                        field: "state_preparation state_dimension",
                    });
                }
            }

            Self::Custom(value) => {
                if !value.kind().is_custom() {
                    return Err(WorkloadError::InvalidCombination {
                        message:
                            "custom workload must use a custom workload kind",
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Workload builder
// =============================================================================

/// Builder for the common workload envelope.
///
/// The builder exists so the future Zamani language/runtime integration can
/// construct workloads incrementally without exposing mutable internals.
#[derive(Debug, Clone)]
pub struct WorkloadBuilder {
    id: WorkloadId,
    kind: WorkloadKind,
    name: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    dimensions: Vec<WorkloadDimension>,
}

impl WorkloadBuilder {
    /// Creates a workload builder.
    pub fn new(id: WorkloadId, kind: WorkloadKind) -> Self {
        Self {
            id,
            kind,
            name: None,
            description: None,
            tags: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    /// Sets a human-readable workload name.
    pub fn name<S: Into<String>>(
        mut self,
        name: S,
    ) -> Result<Self, WorkloadError> {
        let name = name.into();

        if name.is_empty() {
            return Err(WorkloadError::EmptyIdentifier {
                field: "workload name",
            });
        }

        if name.len() > MAX_WORKLOAD_NAME_BYTES {
            return Err(WorkloadError::IdentifierTooLong {
                field: "workload name",
                length: name.len(),
                maximum: MAX_WORKLOAD_NAME_BYTES,
            });
        }

        self.name = Some(name);
        Ok(self)
    }

    /// Sets a bounded workload description.
    pub fn description<S: Into<String>>(
        mut self,
        description: S,
    ) -> Result<Self, WorkloadError> {
        let description = description.into();

        if description.len() > MAX_WORKLOAD_DESCRIPTION_BYTES {
            return Err(WorkloadError::IdentifierTooLong {
                field: "workload description",
                length: description.len(),
                maximum: MAX_WORKLOAD_DESCRIPTION_BYTES,
            });
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Adds a bounded workload tag.
    pub fn tag<S: Into<String>>(
        mut self,
        tag: S,
    ) -> Result<Self, WorkloadError> {
        if self.tags.len() >= MAX_WORKLOAD_TAGS {
            return Err(WorkloadError::TooManyTags {
                count: self.tags.len() + 1,
                maximum: MAX_WORKLOAD_TAGS,
            });
        }

        let tag = tag.into();

        if tag.is_empty() {
            return Err(WorkloadError::EmptyIdentifier {
                field: "workload tag",
            });
        }

        if tag.len() > MAX_WORKLOAD_TAG_BYTES {
            return Err(WorkloadError::TagTooLong {
                length: tag.len(),
                maximum: MAX_WORKLOAD_TAG_BYTES,
            });
        }

        self.tags.push(tag);
        Ok(self)
    }

    /// Adds a workload dimension.
    pub fn dimension(
        mut self,
        dimension: WorkloadDimension,
    ) -> Result<Self, WorkloadError> {
        if self.dimensions.len() >= MAX_WORKLOAD_DIMENSIONS {
            return Err(WorkloadError::TooManyDimensions {
                count: self.dimensions.len() + 1,
                maximum: MAX_WORKLOAD_DIMENSIONS,
            });
        }

        self.dimensions.push(dimension);
        Ok(self)
    }

    /// Returns the stable workload identity.
    #[must_use]
    pub fn id(&self) -> &WorkloadId {
        &self.id
    }

    /// Returns the workload kind.
    #[must_use]
    pub fn kind(&self) -> &WorkloadKind {
        &self.kind
    }

    /// Returns the configured name.
    #[must_use]
    pub fn configured_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the configured description.
    #[must_use]
    pub fn configured_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns tags.
    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Returns dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[WorkloadDimension] {
        &self.dimensions
    }
}

// =============================================================================
// Common workload metadata
// =============================================================================

/// Immutable metadata associated with every benchmark workload.
///
/// This is kept separate from the workload payload so `core::experiment` and
/// `core::provenance` can consume it without knowing the workload's concrete
/// domain.
#[derive(Debug, Clone)]
pub struct WorkloadMetadata {
    id: WorkloadId,
    kind: WorkloadKind,
    name: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    dimensions: Vec<WorkloadDimension>,
}

impl WorkloadMetadata {
    /// Creates metadata directly.
    pub fn new(
        id: WorkloadId,
        kind: WorkloadKind,
    ) -> Self {
        Self {
            id,
            kind,
            name: None,
            description: None,
            tags: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    /// Creates metadata from a builder.
    #[must_use]
    pub fn from_builder(builder: WorkloadBuilder) -> Self {
        Self {
            id: builder.id,
            kind: builder.kind,
            name: builder.name,
            description: builder.description,
            tags: builder.tags,
            dimensions: builder.dimensions,
        }
    }

    /// Returns workload identity.
    #[must_use]
    pub fn id(&self) -> &WorkloadId {
        &self.id
    }

    /// Returns workload kind.
    #[must_use]
    pub fn kind(&self) -> &WorkloadKind {
        &self.kind
    }

    /// Returns stable kind identifier.
    #[must_use]
    pub fn kind_id(&self) -> &str {
        self.kind.id()
    }

    /// Returns human-readable name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns tags.
    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Returns workload dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[WorkloadDimension] {
        &self.dimensions
    }
}

// =============================================================================
// Workload descriptor
// =============================================================================

/// Complete benchmark workload descriptor.
///
/// This pairs the semantic workload with stable metadata.
///
/// `WorkloadDescriptor` is the type that future `core::experiment` should
/// normally consume.
#[derive(Debug, Clone)]
pub struct WorkloadDescriptor {
    metadata: WorkloadMetadata,
    workload: Workload,
}

impl WorkloadDescriptor {
    /// Creates and validates a workload descriptor.
    pub fn new(
        metadata: WorkloadMetadata,
        workload: Workload,
    ) -> Result<Self, WorkloadError> {
        if metadata.id().as_str() != workload.instance_id().as_str() {
            return Err(WorkloadError::InvalidCombination {
                message:
                    "workload metadata id must equal workload instance_id",
            });
        }

        if metadata.kind() != &workload.kind() {
            return Err(WorkloadError::InvalidCombination {
                message:
                    "workload metadata kind must equal workload kind",
            });
        }

        workload.validate()?;

        Ok(Self {
            metadata,
            workload,
        })
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &WorkloadMetadata {
        &self.metadata
    }

    /// Returns the semantic workload payload.
    #[must_use]
    pub fn workload(&self) -> &Workload {
        &self.workload
    }

    /// Returns the workload kind.
    #[must_use]
    pub fn kind(&self) -> &WorkloadKind {
        self.metadata.kind()
    }

    /// Returns the stable workload identifier.
    #[must_use]
    pub fn id(&self) -> &WorkloadId {
        self.metadata.id()
    }

    /// Returns the execution model.
    #[must_use]
    pub fn execution_model(&self) -> ExecutionModel {
        self.workload.execution_model()
    }

    /// Returns whether this descriptor contains a canonical Quantum IR
    /// circuit.
    #[must_use]
    pub fn has_circuit(&self) -> bool {
        self.workload.has_circuit()
    }

    /// Returns the canonical Quantum IR circuit, if present.
    #[must_use]
    pub fn circuit(&self) -> Option<&QuantumCircuit> {
        self.workload.circuit()
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Validates a canonical identifier.
///
/// Canonical identifiers use:
///
/// `[a-z][a-z0-9_]*`
fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), WorkloadError> {
    if value.is_empty() {
        return Err(WorkloadError::EmptyIdentifier { field });
    }

    if value.len() > maximum {
        return Err(WorkloadError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(WorkloadError::InvalidIdentifierStart { field });
    }

    for (position, byte) in bytes.iter().copied().enumerate().skip(1) {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_')
        {
            return Err(WorkloadError::InvalidIdentifierCharacter {
                field,
                position,
                byte,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn workload_id() -> WorkloadId {
        WorkloadId::new("example_workload").unwrap()
    }

    #[test]
    fn workload_id_accepts_canonical_identifier() {
        let id = WorkloadId::new("quantum_volume_20").unwrap();

        assert_eq!(id.as_str(), "quantum_volume_20");
    }

    #[test]
    fn workload_id_rejects_empty_identifier() {
        let result = WorkloadId::new("");

        assert!(matches!(
            result,
            Err(WorkloadError::EmptyIdentifier {
                field: "workload_id"
            })
        ));
    }

    #[test]
    fn workload_id_rejects_invalid_first_character() {
        let result = WorkloadId::new("1benchmark");

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidIdentifierStart {
                field: "workload_id"
            })
        ));
    }

    #[test]
    fn workload_id_rejects_invalid_character() {
        let result = WorkloadId::new("benchmark-name");

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidIdentifierCharacter {
                field: "workload_id",
                ..
            })
        ));
    }

    #[test]
    fn custom_workload_kind_is_supported() {
        let kind =
            WorkloadKind::custom("quantum_machine_learning").unwrap();

        assert!(kind.is_custom());
        assert_eq!(
            kind.id(),
            "quantum_machine_learning"
        );
    }

    #[test]
    fn custom_workload_kind_rejects_invalid_identifier() {
        assert!(
            WorkloadKind::custom("QuantumML").is_err()
        );
    }

    #[test]
    fn execution_models_are_stable() {
        assert_eq!(
            ExecutionModel::GateModel.id(),
            "gate_model"
        );
        assert_eq!(
            ExecutionModel::Analog.id(),
            "analog"
        );
        assert_eq!(
            ExecutionModel::Annealing.id(),
            "annealing"
        );
        assert_eq!(
            ExecutionModel::Logical.id(),
            "logical"
        );
    }

    #[test]
    fn application_parameter_is_bounded() {
        let parameter =
            ApplicationParameter::new("problem_size", "32").unwrap();

        assert_eq!(parameter.name(), "problem_size");
        assert_eq!(parameter.value(), "32");
    }

    #[test]
    fn application_parameter_rejects_invalid_name() {
        assert!(
            ApplicationParameter::new(
                "problem-size",
                "32"
            )
            .is_err()
        );
    }

    #[test]
    fn application_workload_supports_parameters() {
        let mut workload =
            ApplicationWorkload::new(
                "qaoa",
                workload_id(),
                20,
            )
            .unwrap();

        workload
            .add_parameter(
                ApplicationParameter::new(
                    "layers",
                    "4",
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(workload.application_id(), "qaoa");
        assert_eq!(workload.problem_size(), 20);
        assert_eq!(workload.parameters().len(), 1);
    }

    #[test]
    fn hybrid_profile_requires_positive_iterations() {
        assert!(
            HybridExecutionProfile::new(0, true).is_err()
        );

        assert!(
            HybridExecutionProfile::new(100, true).is_ok()
        );
    }

    #[test]
    fn qec_workload_rejects_impossible_qubit_counts() {
        let result = ErrorCorrectionWorkload::new(
            "surface_code",
            workload_id(),
            3,
            100,
            10,
            5,
        );

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidCombination { .. })
        ));
    }

    #[test]
    fn qec_workload_calculates_logical_rounds_checked() {
        let workload =
            ErrorCorrectionWorkload::new(
                "surface_code",
                workload_id(),
                3,
                100,
                10,
                100,
            )
            .unwrap();

        assert_eq!(
            workload.logical_rounds().unwrap(),
            1000
        );
    }

    #[test]
    fn logical_workload_calculates_overhead() {
        let workload =
            LogicalWorkload::new(
                workload_id(),
                10,
                100,
                1000,
            )
            .unwrap();

        assert_eq!(
            workload.qubit_overhead().unwrap(),
            100.0
        );
    }

    #[test]
    fn analog_workload_rejects_non_positive_evolution_time() {
        assert!(
            AnalogWorkload::new(
                "hamiltonian",
                workload_id(),
                20,
                0.0,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn annealing_workload_rejects_non_finite_time() {
        assert!(
            AnnealingWorkload::new(
                "ising",
                workload_id(),
                50,
                f64::NAN,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn sampling_workload_requires_samples() {
        assert!(
            SamplingWorkload::new(
                "random_sampling",
                workload_id(),
                32,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn state_preparation_requires_dimension() {
        assert!(
            StatePreparationWorkload::new(
                workload_id(),
                0,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn custom_workload_requires_custom_kind() {
        let result = CustomWorkload::new(
            WorkloadKind::Circuit,
            workload_id(),
        );

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidCombination { .. })
        ));
    }

    #[test]
    fn custom_workload_accepts_custom_dimensions() {
        let mut workload =
            CustomWorkload::new(
                WorkloadKind::custom("tensor_network")
                    .unwrap(),
                workload_id(),
            )
            .unwrap();

        workload
            .add_dimension(
                WorkloadDimension::new(
                    "bond_dimension",
                    128.0,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            workload.dimensions().len(),
            1
        );
        assert_eq!(
            workload.dimensions()[0].name(),
            "bond_dimension"
        );
    }

    #[test]
    fn workload_builder_is_bounded() {
        let builder =
            WorkloadBuilder::new(
                workload_id(),
                WorkloadKind::Application,
            )
            .name("QAOA benchmark")
            .unwrap()
            .description("Production benchmark")
            .unwrap()
            .tag("optimization")
            .unwrap()
            .dimension(
                WorkloadDimension::new(
                    "problem_size",
                    32.0,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            builder.configured_name(),
            Some("QAOA benchmark")
        );
        assert_eq!(
            builder.tags(),
            &["optimization".to_owned()]
        );
        assert_eq!(
            builder.dimensions().len(),
            1
        );
    }

    #[test]
    fn metadata_preserves_builder_information() {
        let builder =
            WorkloadBuilder::new(
                workload_id(),
                WorkloadKind::Application,
            )
            .name("QAOA")
            .unwrap()
            .tag("optimization")
            .unwrap();

        let metadata =
            WorkloadMetadata::from_builder(builder);

        assert_eq!(
            metadata.id().as_str(),
            "example_workload"
        );
        assert_eq!(
            metadata.kind_id(),
            "application"
        );
        assert_eq!(
            metadata.name(),
            Some("QAOA")
        );
        assert_eq!(
            metadata.tags(),
            &["optimization".to_owned()]
        );
    }

    #[test]
    fn descriptor_rejects_metadata_identity_mismatch() {
        let workload = Workload::Sampling(
            SamplingWorkload::new(
                "random_sampling",
                workload_id(),
                32,
                1000,
            )
            .unwrap(),
        );

        let metadata =
            WorkloadMetadata::new(
                WorkloadId::new("different_workload")
                    .unwrap(),
                WorkloadKind::Sampling,
            );

        let result =
            WorkloadDescriptor::new(
                metadata,
                workload,
            );

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidCombination { .. })
        ));
    }

    #[test]
    fn descriptor_rejects_kind_mismatch() {
        let workload = Workload::Sampling(
            SamplingWorkload::new(
                "random_sampling",
                workload_id(),
                32,
                1000,
            )
            .unwrap(),
        );

        let metadata =
            WorkloadMetadata::new(
                workload_id(),
                WorkloadKind::Circuit,
            );

        let result =
            WorkloadDescriptor::new(
                metadata,
                workload,
            );

        assert!(matches!(
            result,
            Err(WorkloadError::InvalidCombination { .. })
        ));
    }

    #[test]
    fn workload_kind_is_correct() {
        let workload = Workload::Sampling(
            SamplingWorkload::new(
                "random_sampling",
                workload_id(),
                32,
                1000,
            )
            .unwrap(),
        );

        assert_eq!(
            workload.kind(),
            WorkloadKind::Sampling
        );

        assert_eq!(
            workload.kind_id(),
            "sampling"
        );

        assert_eq!(
            workload.execution_model(),
            ExecutionModel::Sampling
        );
    }

    #[test]
    fn workload_validation_is_idempotent() {
        let workload = Workload::Annealing(
            AnnealingWorkload::new(
                "ising",
                workload_id(),
                100,
                20.0,
                1000,
            )
            .unwrap(),
        );

        assert!(workload.validate().is_ok());
        assert!(workload.validate().is_ok());
    }

    #[test]
    fn workload_does_not_require_a_circuit_for_analog_execution() {
        let workload = Workload::Analog(
            AnalogWorkload::new(
                "rydberg",
                workload_id(),
                100,
                10.0,
                1000,
            )
            .unwrap(),
        );

        assert!(!workload.has_circuit());
        assert!(workload.circuit().is_none());
        assert_eq!(
            workload.execution_model(),
            ExecutionModel::Analog
        );
    }
}