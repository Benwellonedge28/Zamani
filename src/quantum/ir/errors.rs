//! Zamani Quantum IR — Canonical Error Model
//!
//! This module defines the stable, hardware-independent error vocabulary for
//! the Zamani Quantum Intermediate Representation (IR).
//!
//! # Architectural role
//!
//! `errors.rs` is a foundational IR module. It defines:
//!
//! - the canonical `IrError` type;
//! - stable error categories;
//! - stable machine-readable error codes;
//! - structured error payloads;
//! - diagnostic severity;
//! - resource-limit failures;
//! - identifier failures;
//! - semantic failures;
//! - validation failures;
//! - serialization/version failures;
//! - extension failures;
//! - generic operation failures;
//! - conversion helpers for downstream IR modules.
//!
//! The error model is intentionally independent of concrete IR implementations.
//!
//! # Dependency boundary
//!
//! This file MUST NOT depend on:
//!
//! - `circuit.rs`;
//! - `gate.rs`;
//! - `measurement.rs`;
//! - `validation.rs`;
//! - `analysis.rs`;
//! - `program.rs`;
//! - `operation.rs`;
//! - `pulse.rs`;
//! - `waveform.rs`;
//! - `channel.rs`;
//! - `frame.rs`;
//! - `schedule.rs`;
//! - `resource.rs`;
//! - `capability.rs`;
//! - `mapping.rs`;
//! - `hardware`;
//! - `routing`;
//! - `scheduling`;
//! - `optimization`;
//! - `frontend`;
//! - `backend`;
//! - `simulator`.
//!
//! Downstream modules may convert their local errors into `IrError`.
//!
//! `errors.rs` intentionally does not import `quantum::ir::qubit`.
//! Logical and physical qubit identity remain owned by `qubit.rs`, while this
//! foundational module provides generic error categories that do not require
//! coupling to any particular IR resource implementation.
//!
//! # Hardware independence
//!
//! The canonical IR describes quantum-program semantics.
//!
//! Hardware-specific failures such as:
//!
//! - unavailable QPU;
//! - broken physical qubit;
//! - unsupported DAC;
//! - unsupported pulse generator;
//! - calibration failure;
//! - topology failure;
//! - backend transport failure;
//! - device timeout;
//!
//! MUST be represented by the corresponding hardware/backend layers and may
//! then be translated into higher-level compiler diagnostics.
//!
//! They are not part of the canonical semantic IR error taxonomy.
//!
//! # Scalability
//!
//! This module imposes no architectural limit on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of operations;
//! - circuit depth;
//! - number of pulse operations;
//! - number of programs;
//! - number of resources.
//!
//! Resource limits are policy values supplied by `limits.rs` or another
//! explicit compilation/execution policy.
//!
//! In particular, no value such as `63`, `64`, `4096`, or `1_000_000` is used
//! as a quantum-machine-size boundary.
//!
//! # Error design
//!
//! Every canonical error provides:
//!
//! - a stable category;
//! - a stable machine-readable code;
//! - a human-readable message;
//! - optional structured location information;
//! - optional contextual information;
//! - optional causal error information.
//!
//! Error messages are diagnostics. Consumers must use `IrErrorKind` and
//! `IrErrorCode` for programmatic classification rather than parsing strings.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.
//! No `unsafe` code is used.

// =============================================================================
// Standard library
// =============================================================================

use std::error::Error;
use std::fmt;

// =============================================================================
// Result aliases
// =============================================================================

/// Canonical result type for Quantum IR APIs.
pub type IrResult<T> = Result<T, IrError>;

/// Result type used when a lower-level error must be wrapped as an IR error.
pub type IrDiagnosticResult<T> = Result<T, IrError>;

// =============================================================================
// Error severity
// =============================================================================

/// Severity of an IR diagnostic.
///
/// Errors represent conditions that prevent a valid operation from completing.
/// Warnings and informational diagnostics are intentionally represented
/// separately so the compiler can use one diagnostic vocabulary without
/// confusing recoverable conditions with hard failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorSeverity {
    /// Informational diagnostic.
    Info,

    /// Recoverable warning.
    Warning,

    /// Fatal error for the current operation/program stage.
    Error,
}

impl fmt::Display for IrErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

// =============================================================================
// Error categories
// =============================================================================

/// Stable high-level category for an IR diagnostic.
///
/// This enum is intentionally independent of individual implementation files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorKind {
    /// Configured IR resource policy was violated.
    Limits,

    /// An IR identifier is invalid or inconsistent.
    Identifier,

    /// Logical quantum-resource semantics are invalid.
    Qubit,

    /// Classical-resource semantics are invalid.
    Classical,

    /// Parameter semantics are invalid.
    Parameter,

    /// Gate semantics are invalid.
    Gate,

    /// Measurement semantics are invalid.
    Measurement,

    /// Control-flow semantics are invalid.
    ControlFlow,

    /// Timing semantics are invalid.
    Timing,

    /// Pulse semantics are invalid.
    Pulse,

    /// Waveform semantics are invalid.
    Waveform,

    /// Channel semantics are invalid.
    Channel,

    /// Frame semantics are invalid.
    Frame,

    /// Scheduling representation is invalid.
    Schedule,

    /// Abstract resource requirement is invalid.
    Resource,

    /// Capability requirement is invalid.
    Capability,

    /// Logical/physical mapping representation is invalid.
    Mapping,

    /// Program structure is invalid.
    Program,

    /// Region/block structure is invalid.
    Region,

    /// Circuit structure is invalid.
    Circuit,

    /// Generic operation structure is invalid.
    Operation,

    /// IR validation failed.
    Validation,

    /// Static IR analysis failed.
    Analysis,

    /// Serialization/deserialization failed.
    Serialization,

    /// IR schema/version incompatibility.
    Version,

    /// An extension is malformed or unsupported.
    Extension,

    /// A requested feature is not part of the IR contract.
    Unsupported,

    /// A required invariant was violated.
    Invariant,

    /// A generic internal compiler/IR failure occurred.
    Internal,
}

impl fmt::Display for IrErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Limits => "limits",
            Self::Identifier => "identifier",
            Self::Qubit => "qubit",
            Self::Classical => "classical",
            Self::Parameter => "parameter",
            Self::Gate => "gate",
            Self::Measurement => "measurement",
            Self::ControlFlow => "control_flow",
            Self::Timing => "timing",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Schedule => "schedule",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Mapping => "mapping",
            Self::Program => "program",
            Self::Region => "region",
            Self::Circuit => "circuit",
            Self::Operation => "operation",
            Self::Validation => "validation",
            Self::Analysis => "analysis",
            Self::Serialization => "serialization",
            Self::Version => "version",
            Self::Extension => "extension",
            Self::Unsupported => "unsupported",
            Self::Invariant => "invariant",
            Self::Internal => "internal",
        };

        write!(f, "{value}")
    }
}

// =============================================================================
// Stable error codes
// =============================================================================

/// Stable machine-readable IR error code.
///
/// Error codes are intentionally independent of human-readable messages.
/// Compiler tools, IDEs, diagnostics, testing systems and external tooling
/// should use these codes rather than matching message text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorCode {
    // -------------------------------------------------------------------------
    // Limits
    // -------------------------------------------------------------------------

    /// A configured resource limit was exceeded.
    LimitExceeded,

    /// A requested resource amount cannot be represented safely.
    ResourceOverflow,

    /// A size calculation overflowed.
    SizeOverflow,

    /// An allocation/request was rejected before allocation.
    AllocationRejected,

    // -------------------------------------------------------------------------
    // Identity
    // -------------------------------------------------------------------------

    /// An identifier is invalid.
    InvalidIdentifier,

    /// An identifier is duplicated.
    DuplicateIdentifier,

    /// An identifier is outside a declared namespace.
    IdentifierOutOfRange,

    /// Two identity domains were incorrectly mixed.
    IdentityDomainMismatch,

    // -------------------------------------------------------------------------
    // Structure
    // -------------------------------------------------------------------------

    /// Required data is missing.
    MissingData,

    /// Data was supplied where it is not valid.
    UnexpectedData,

    /// A structural invariant was violated.
    InvalidStructure,

    /// A required operand is missing.
    MissingOperand,

    /// An operand is invalid.
    InvalidOperand,

    /// Operand arity is invalid.
    InvalidArity,

    // -------------------------------------------------------------------------
    // Type/value
    // -------------------------------------------------------------------------

    /// A value has the wrong type.
    TypeMismatch,

    /// A value is outside its legal domain.
    InvalidValue,

    /// A numeric value is NaN or infinite where finite data is required.
    NonFiniteValue,

    /// A parameter expression is invalid.
    InvalidExpression,

    /// A symbolic value has no binding.
    UnboundParameter,

    // -------------------------------------------------------------------------
    // Quantum semantics
    // -------------------------------------------------------------------------

    /// A logical qubit is invalid.
    InvalidQubit,

    /// A classical resource is invalid.
    InvalidClassicalResource,

    /// A gate is invalid.
    InvalidGate,

    /// A measurement is invalid.
    InvalidMeasurement,

    /// A control-flow construct is invalid.
    InvalidControlFlow,

    /// A pulse is invalid.
    InvalidPulse,

    /// A waveform is invalid.
    InvalidWaveform,

    /// A channel is invalid.
    InvalidChannel,

    /// A frame is invalid.
    InvalidFrame,

    /// A timing relation is invalid.
    InvalidTiming,

    /// A schedule is invalid.
    InvalidSchedule,

    /// A resource requirement is invalid.
    InvalidResourceRequirement,

    /// A capability requirement is invalid.
    InvalidCapabilityRequirement,

    /// A mapping is invalid.
    InvalidMapping,

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Semantic validation failed.
    SemanticValidationFailed,

    /// Structural validation failed.
    StructuralValidationFailed,

    /// Type validation failed.
    TypeValidationFailed,

    /// Resource validation failed.
    ResourceValidationFailed,

    /// Timing validation failed.
    TimingValidationFailed,

    /// Capability validation failed.
    CapabilityValidationFailed,

    // -------------------------------------------------------------------------
    // Versioning
    // -------------------------------------------------------------------------

    /// The IR version is not supported.
    UnsupportedVersion,

    /// A future version was encountered.
    FutureVersion,

    /// A version is malformed.
    InvalidVersion,

    /// A required compatibility conversion is unavailable.
    CompatibilityConversionUnavailable,

    // -------------------------------------------------------------------------
    // Serialization
    // -------------------------------------------------------------------------

    /// Serialization failed.
    SerializationFailed,

    /// Deserialization failed.
    DeserializationFailed,

    /// Serialized data is malformed.
    MalformedData,

    /// Serialized data is incomplete.
    TruncatedData,

    /// Serialized data exceeds the configured policy.
    SerializedSizeExceeded,

    // -------------------------------------------------------------------------
    // Extensions
    // -------------------------------------------------------------------------

    /// An extension is invalid.
    InvalidExtension,

    /// An extension is unsupported.
    UnsupportedExtension,

    /// An extension violates an IR invariant.
    ExtensionInvariantViolation,

    // -------------------------------------------------------------------------
    // Capability/support
    // -------------------------------------------------------------------------

    /// The semantic IR does not define the requested feature.
    UnsupportedFeature,

    /// A required feature cannot be represented.
    UnrepresentableFeature,

    // -------------------------------------------------------------------------
    // Internal
    // -------------------------------------------------------------------------

    /// An internal invariant failed.
    InternalInvariant,

    /// An internal implementation failure occurred.
    InternalFailure,
}

impl fmt::Display for IrErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::LimitExceeded => "IR-LIMIT-001",
            Self::ResourceOverflow => "IR-LIMIT-002",
            Self::SizeOverflow => "IR-LIMIT-003",
            Self::AllocationRejected => "IR-LIMIT-004",

            Self::InvalidIdentifier => "IR-ID-001",
            Self::DuplicateIdentifier => "IR-ID-002",
            Self::IdentifierOutOfRange => "IR-ID-003",
            Self::IdentityDomainMismatch => "IR-ID-004",

            Self::MissingData => "IR-STRUCT-001",
            Self::UnexpectedData => "IR-STRUCT-002",
            Self::InvalidStructure => "IR-STRUCT-003",
            Self::MissingOperand => "IR-STRUCT-004",
            Self::InvalidOperand => "IR-STRUCT-005",
            Self::InvalidArity => "IR-STRUCT-006",

            Self::TypeMismatch => "IR-TYPE-001",
            Self::InvalidValue => "IR-TYPE-002",
            Self::NonFiniteValue => "IR-TYPE-003",
            Self::InvalidExpression => "IR-TYPE-004",
            Self::UnboundParameter => "IR-TYPE-005",

            Self::InvalidQubit => "IR-QUBIT-001",
            Self::InvalidClassicalResource => "IR-CLASSICAL-001",
            Self::InvalidGate => "IR-GATE-001",
            Self::InvalidMeasurement => "IR-MEASURE-001",
            Self::InvalidControlFlow => "IR-CFLOW-001",
            Self::InvalidPulse => "IR-PULSE-001",
            Self::InvalidWaveform => "IR-WAVEFORM-001",
            Self::InvalidChannel => "IR-CHANNEL-001",
            Self::InvalidFrame => "IR-FRAME-001",
            Self::InvalidTiming => "IR-TIME-001",
            Self::InvalidSchedule => "IR-SCHEDULE-001",
            Self::InvalidResourceRequirement => "IR-RESOURCE-001",
            Self::InvalidCapabilityRequirement => "IR-CAPABILITY-001",
            Self::InvalidMapping => "IR-MAPPING-001",

            Self::SemanticValidationFailed => "IR-VALIDATION-001",
            Self::StructuralValidationFailed => "IR-VALIDATION-002",
            Self::TypeValidationFailed => "IR-VALIDATION-003",
            Self::ResourceValidationFailed => "IR-VALIDATION-004",
            Self::TimingValidationFailed => "IR-VALIDATION-005",
            Self::CapabilityValidationFailed => "IR-VALIDATION-006",

            Self::UnsupportedVersion => "IR-VERSION-001",
            Self::FutureVersion => "IR-VERSION-002",
            Self::InvalidVersion => "IR-VERSION-003",
            Self::CompatibilityConversionUnavailable => "IR-VERSION-004",

            Self::SerializationFailed => "IR-SERIALIZE-001",
            Self::DeserializationFailed => "IR-SERIALIZE-002",
            Self::MalformedData => "IR-SERIALIZE-003",
            Self::TruncatedData => "IR-SERIALIZE-004",
            Self::SerializedSizeExceeded => "IR-SERIALIZE-005",

            Self::InvalidExtension => "IR-EXT-001",
            Self::UnsupportedExtension => "IR-EXT-002",
            Self::ExtensionInvariantViolation => "IR-EXT-003",

            Self::UnsupportedFeature => "IR-SUPPORT-001",
            Self::UnrepresentableFeature => "IR-SUPPORT-002",

            Self::InternalInvariant => "IR-INTERNAL-001",
            Self::InternalFailure => "IR-INTERNAL-002",
        };

        write!(f, "{value}")
    }
}

// =============================================================================
// Diagnostic location
// =============================================================================

/// Generic source/IR location information.
///
/// The location intentionally does not depend on any future IR identity type.
/// It can therefore be used by every module without changing this file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct IrLocation {
    /// Optional source file/module name.
    pub source: Option<String>,

    /// Optional one-based line.
    pub line: Option<u64>,

    /// Optional one-based column.
    pub column: Option<u64>,

    /// Optional byte offset.
    pub byte_offset: Option<u64>,

    /// Optional byte length.
    pub byte_length: Option<u64>,

    /// Optional IR object label.
    pub object: Option<String>,

    /// Optional operation/program path.
    pub path: Option<String>,
}

impl IrLocation {
    /// Creates an empty location.
    pub const fn new() -> Self {
        Self {
            source: None,
            line: None,
            column: None,
            byte_offset: None,
            byte_length: None,
            object: None,
            path: None,
        }
    }

    /// Sets a source name.
    pub fn with_source<S: Into<String>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Sets a one-based line.
    pub fn with_line(mut self, line: u64) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets a one-based column.
    pub fn with_column(mut self, column: u64) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets a byte offset.
    pub fn with_byte_offset(mut self, byte_offset: u64) -> Self {
        self.byte_offset = Some(byte_offset);
        self
    }

    /// Sets a byte length.
    pub fn with_byte_length(mut self, byte_length: u64) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Sets a generic IR object description.
    pub fn with_object<S: Into<String>>(mut self, object: S) -> Self {
        self.object = Some(object.into());
        self
    }

    /// Sets an IR/source path.
    pub fn with_path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Returns whether the location contains any information.
    pub fn is_empty(&self) -> bool {
        self.source.is_none()
            && self.line.is_none()
            && self.column.is_none()
            && self.byte_offset.is_none()
            && self.byte_length.is_none()
            && self.object.is_none()
            && self.path.is_none()
    }
}

impl fmt::Display for IrLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;

        if let Some(source) = &self.source {
            write!(f, "{source}")?;
            wrote = true;
        }

        if let Some(line) = self.line {
            if wrote {
                write!(f, ":")?;
            }

            write!(f, "{line}")?;
            wrote = true;

            if let Some(column) = self.column {
                write!(f, ":{column}")?;
            }
        }

        if let Some(object) = &self.object {
            if wrote {
                write!(f, " ")?;
            }

            write!(f, "[{object}]")?;
            wrote = true;
        }

        if let Some(path) = &self.path {
            if wrote {
                write!(f, " ")?;
            }

            write!(f, "{path}")?;
            wrote = true;
        }

        if !wrote {
            write!(f, "<unknown>")
        } else {
            Ok(())
        }
    }
}

// =============================================================================
// Structured limit error
// =============================================================================

/// Structured resource-limit violation.
///
/// The `limit` field is a stable policy name such as:
///
/// - `logical_qubits`;
/// - `classical_bits`;
/// - `operations`;
/// - `parameters`;
/// - `pulse_operations`;
/// - `waveform_samples`;
/// - `program_bytes`.
///
/// The name is deliberately a string rather than a dependency on `limits.rs`,
/// allowing the limits module to evolve independently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrLimitError {
    /// Stable logical name of the configured limit.
    pub limit: String,

    /// Observed/requested amount.
    pub actual: u64,

    /// Configured maximum.
    pub maximum: u64,
}

impl IrLimitError {
    /// Creates a structured limit violation.
    pub fn new<S: Into<String>>(
        limit: S,
        actual: u64,
        maximum: u64,
    ) -> Self {
        Self {
            limit: limit.into(),
            actual,
            maximum,
        }
    }
}

impl fmt::Display for IrLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IR limit `{}` exceeded: requested={}, maximum={}",
            self.limit,
            self.actual,
            self.maximum
        )
    }
}

impl Error for IrLimitError {}

// =============================================================================
// Identifier error
// =============================================================================

/// Structured identifier failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrIdentifierError {
    /// Identifier is outside the relevant namespace.
    OutOfRange {
        /// Identifier value.
        value: u64,

        /// Namespace size/capacity.
        count: u64,

        /// Logical name of the identifier domain.
        domain: &'static str,
    },

    /// Identifier is duplicated.
    Duplicate {
        /// Logical identifier domain.
        domain: &'static str,

        /// Numeric identifier value.
        value: u64,
    },

    /// Identity domains were incorrectly mixed.
    DomainMismatch {
        /// Expected domain.
        expected: &'static str,

        /// Actual domain.
        actual: &'static str,
    },

    /// Identifier namespace is invalid.
    InvalidNamespace {
        /// Identifier domain.
        domain: &'static str,
    },

    /// An identifier value is otherwise invalid.
    Invalid {
        /// Identifier domain.
        domain: &'static str,

        /// Numeric value.
        value: u64,
    },
}

impl fmt::Display for IrIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                value,
                count,
                domain,
            } => write!(
                f,
                "{domain} identifier {value} is outside namespace range 0..{count}"
            ),

            Self::Duplicate { domain, value } => {
                write!(f, "duplicate {domain} identifier {value}")
            }

            Self::DomainMismatch { expected, actual } => write!(
                f,
                "identifier domain mismatch: expected `{expected}`, found `{actual}`"
            ),

            Self::InvalidNamespace { domain } => {
                write!(f, "invalid identifier namespace `{domain}`")
            }

            Self::Invalid { domain, value } => {
                write!(f, "invalid {domain} identifier {value}")
            }
        }
    }
}

impl Error for IrIdentifierError {}

// =============================================================================
// Generic semantic error
// =============================================================================

/// Generic structured semantic failure.
///
/// This is used where a specialized error type would create unnecessary
/// coupling between foundational and higher-level IR modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrSemanticError {
    /// Semantic domain.
    pub domain: &'static str,

    /// Human-readable explanation.
    pub reason: String,
}

impl IrSemanticError {
    /// Creates a semantic error.
    pub fn new<S: Into<String>>(
        domain: &'static str,
        reason: S,
    ) -> Self {
        Self {
            domain,
            reason: reason.into(),
        }
    }
}

impl fmt::Display for IrSemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid {} semantics: {}",
            self.domain,
            self.reason
        )
    }
}

impl Error for IrSemanticError {}

// =============================================================================
// Version error
// =============================================================================

/// Structured IR version compatibility failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrVersionError {
    /// Version is unsupported.
    Unsupported {
        /// Major component.
        major: u16,

        /// Minor component.
        minor: u16,

        /// Patch component.
        patch: u16,
    },

    /// Future version was encountered.
    Future {
        /// Major component.
        major: u16,

        /// Minor component.
        minor: u16,

        /// Patch component.
        patch: u16,
    },

    /// Version data is invalid.
    Invalid {
        /// Major component.
        major: u16,

        /// Minor component.
        minor: u16,

        /// Patch component.
        patch: u16,
    },

    /// Required compatibility conversion is unavailable.
    ConversionUnavailable {
        /// Source version as text.
        from: String,

        /// Target version as text.
        to: String,
    },
}

impl fmt::Display for IrVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported {
                major,
                minor,
                patch,
            } => write!(
                f,
                "unsupported IR version {major}.{minor}.{patch}"
            ),

            Self::Future {
                major,
                minor,
                patch,
            } => write!(
                f,
                "future IR version {major}.{minor}.{patch} is not understood by this implementation"
            ),

            Self::Invalid {
                major,
                minor,
                patch,
            } => write!(
                f,
                "invalid IR version {major}.{minor}.{patch}"
            ),

            Self::ConversionUnavailable { from, to } => write!(
                f,
                "IR compatibility conversion from {from} to {to} is unavailable"
            ),
        }
    }
}

impl Error for IrVersionError {}

// =============================================================================
// Serialization error
// =============================================================================

/// Structured serialization/deserialization failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrSerializationError {
    /// Serialization failed.
    Failed {
        /// Explanation.
        reason: String,
    },

    /// Deserialization failed.
    InvalidData {
        /// Explanation.
        reason: String,
    },

    /// Input is truncated.
    Truncated,

    /// Input is malformed.
    Malformed {
        /// Explanation.
        reason: String,
    },

    /// Serialized data exceeds a configured limit.
    SizeExceeded {
        /// Actual size.
        actual: u64,

        /// Maximum permitted size.
        maximum: u64,
    },
}

impl fmt::Display for IrSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed { reason } => {
                write!(f, "IR serialization failed: {reason}")
            }

            Self::InvalidData { reason } => {
                write!(f, "invalid IR serialized data: {reason}")
            }

            Self::Truncated => {
                write!(f, "IR serialized data is truncated")
            }

            Self::Malformed { reason } => {
                write!(f, "malformed IR serialized data: {reason}")
            }

            Self::SizeExceeded { actual, maximum } => write!(
                f,
                "serialized IR size {actual} exceeds maximum {maximum}"
            ),
        }
    }
}

impl Error for IrSerializationError {}

// =============================================================================
// Extension error
// =============================================================================

/// Structured extension failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrExtensionError {
    /// Extension is malformed.
    Invalid {
        /// Extension namespace/name.
        name: String,

        /// Explanation.
        reason: String,
    },

    /// Extension is not supported.
    Unsupported {
        /// Extension namespace/name.
        name: String,
    },

    /// Extension violates an invariant.
    InvariantViolation {
        /// Extension namespace/name.
        name: String,

        /// Explanation.
        reason: String,
    },
}

impl fmt::Display for IrExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid { name, reason } => write!(
                f,
                "invalid IR extension `{name}`: {reason}"
            ),

            Self::Unsupported { name } => {
                write!(f, "unsupported IR extension `{name}`")
            }

            Self::InvariantViolation { name, reason } => write!(
                f,
                "IR extension `{name}` violates an invariant: {reason}"
            ),
        }
    }
}

impl Error for IrExtensionError {}

// =============================================================================
// Main canonical error
// =============================================================================

/// Canonical structured Quantum IR error.
///
/// `IrError` is intentionally stable and broad enough to serve every planned
/// IR module without requiring this file to import those modules.
///
/// The error contains both machine-readable classification and human-readable
/// diagnostic information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrError {
    /// High-level error category.
    kind: IrErrorKind,

    /// Stable machine-readable code.
    code: IrErrorCode,

    /// Error severity.
    severity: IrErrorSeverity,

    /// Human-readable message.
    message: String,

    /// Optional structured location.
    location: Option<IrLocation>,

    /// Optional nested cause represented as diagnostic text.
    ///
    /// This is intentionally not `Box<dyn Error>` because the canonical error
    /// must remain cloneable, comparable, deterministic, and serialization
    /// friendly.
    cause: Option<String>,
}

impl IrError {
    /// Creates a canonical IR error.
    pub fn new<S: Into<String>>(
        kind: IrErrorKind,
        code: IrErrorCode,
        message: S,
    ) -> Self {
        Self {
            kind,
            code,
            severity: IrErrorSeverity::Error,
            message: message.into(),
            location: None,
            cause: None,
        }
    }

    /// Creates a warning diagnostic.
    pub fn warning<S: Into<String>>(
        kind: IrErrorKind,
        code: IrErrorCode,
        message: S,
    ) -> Self {
        Self {
            kind,
            code,
            severity: IrErrorSeverity::Warning,
            message: message.into(),
            location: None,
            cause: None,
        }
    }

    /// Creates an informational diagnostic.
    pub fn info<S: Into<String>>(
        kind: IrErrorKind,
        code: IrErrorCode,
        message: S,
    ) -> Self {
        Self {
            kind,
            code,
            severity: IrErrorSeverity::Info,
            message: message.into(),
            location: None,
            cause: None,
        }
    }

    /// Returns the error category.
    pub const fn kind(&self) -> IrErrorKind {
        self.kind
    }

    /// Returns the stable error code.
    pub const fn code(&self) -> IrErrorCode {
        self.code
    }

    /// Returns the severity.
    pub const fn severity(&self) -> IrErrorSeverity {
        self.severity
    }

    /// Returns the human-readable message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional location.
    pub fn location(&self) -> Option<&IrLocation> {
        self.location.as_ref()
    }

    /// Returns the optional causal diagnostic text.
    pub fn cause_message(&self) -> Option<&str> {
        self.cause.as_deref()
    }

    /// Adds a location.
    pub fn with_location(
        mut self,
        location: IrLocation,
    ) -> Self {
        self.location = Some(location);
        self
    }

    /// Adds a causal diagnostic.
    pub fn with_cause<S: Into<String>>(
        mut self,
        cause: S,
    ) -> Self {
        self.cause = Some(cause.into());
        self
    }

    /// Returns whether this is an error-level diagnostic.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Error)
    }

    /// Returns whether this is a warning.
    pub const fn is_warning(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Warning)
    }

    /// Returns whether this is informational.
    pub const fn is_info(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Info)
    }

    /// Creates a limit error.
    pub fn limit(error: IrLimitError) -> Self {
        Self::new(
            IrErrorKind::Limits,
            IrErrorCode::LimitExceeded,
            error.to_string(),
        )
    }

    /// Creates an identifier error.
    pub fn identifier(error: IrIdentifierError) -> Self {
        let code = match &error {
            IrIdentifierError::OutOfRange { .. } => {
                IrErrorCode::IdentifierOutOfRange
            }

            IrIdentifierError::Duplicate { .. } => {
                IrErrorCode::DuplicateIdentifier
            }

            IrIdentifierError::DomainMismatch { .. } => {
                IrErrorCode::IdentityDomainMismatch
            }

            IrIdentifierError::InvalidNamespace { .. }
            | IrIdentifierError::Invalid { .. } => {
                IrErrorCode::InvalidIdentifier
            }
        };

        Self::new(
            IrErrorKind::Identifier,
            code,
            error.to_string(),
        )
    }

    /// Creates a semantic error.
    pub fn semantic(
        kind: IrErrorKind,
        code: IrErrorCode,
        error: IrSemanticError,
    ) -> Self {
        Self::new(kind, code, error.to_string())
    }

    /// Creates a version error.
    pub fn version(error: IrVersionError) -> Self {
        let code = match &error {
            IrVersionError::Unsupported { .. } => {
                IrErrorCode::UnsupportedVersion
            }

            IrVersionError::Future { .. } => {
                IrErrorCode::FutureVersion
            }

            IrVersionError::Invalid { .. } => {
                IrErrorCode::InvalidVersion
            }

            IrVersionError::ConversionUnavailable { .. } => {
                IrErrorCode::CompatibilityConversionUnavailable
            }
        };

        Self::new(
            IrErrorKind::Version,
            code,
            error.to_string(),
        )
    }

    /// Creates a serialization error.
    pub fn serialization(
        error: IrSerializationError,
    ) -> Self {
        let code = match &error {
            IrSerializationError::Failed { .. } => {
                IrErrorCode::SerializationFailed
            }

            IrSerializationError::InvalidData { .. } => {
                IrErrorCode::DeserializationFailed
            }

            IrSerializationError::Truncated => {
                IrErrorCode::TruncatedData
            }

            IrSerializationError::Malformed { .. } => {
                IrErrorCode::MalformedData
            }

            IrSerializationError::SizeExceeded { .. } => {
                IrErrorCode::SerializedSizeExceeded
            }
        };

        Self::new(
            IrErrorKind::Serialization,
            code,
            error.to_string(),
        )
    }

    /// Creates an extension error.
    pub fn extension(
        error: IrExtensionError,
    ) -> Self {
        let code = match &error {
            IrExtensionError::Invalid { .. } => {
                IrErrorCode::InvalidExtension
            }

            IrExtensionError::Unsupported { .. } => {
                IrErrorCode::UnsupportedExtension
            }

            IrExtensionError::InvariantViolation { .. } => {
                IrErrorCode::ExtensionInvariantViolation
            }
        };

        Self::new(
            IrErrorKind::Extension,
            code,
            error.to_string(),
        )
    }

    /// Creates an unsupported-feature error.
    pub fn unsupported<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::Unsupported,
            IrErrorCode::UnsupportedFeature,
            message,
        )
    }

    /// Creates an invariant error.
    pub fn invariant<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::Invariant,
            IrErrorCode::InternalInvariant,
            message,
        )
    }

    /// Creates an internal implementation error.
    pub fn internal<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::Internal,
            IrErrorCode::InternalFailure,
            message,
        )
    }

    /// Creates a missing-data error.
    pub fn missing_data<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::MissingData,
            message,
        )
    }

    /// Creates an invalid-structure error.
    pub fn invalid_structure<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::InvalidStructure,
            message,
        )
    }

    /// Creates a type-mismatch error.
    pub fn type_mismatch<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::TypeMismatch,
            message,
        )
    }

    /// Creates a non-finite numeric-value error.
    pub fn non_finite_value<S: Into<String>>(
        message: S,
    ) -> Self {
        Self::new(
            IrErrorKind::Parameter,
            IrErrorCode::NonFiniteValue,
            message,
        )
    }

    /// Converts this error into its stable diagnostic string.
    pub fn diagnostic(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {}] {}",
            self.severity,
            self.code,
            self.message
        )?;

        if let Some(location) = &self.location {
            write!(f, " @ {location}")?;
        }

        if let Some(cause) = &self.cause {
            write!(f, ": {cause}")?;
        }

        Ok(())
    }
}

impl Error for IrError {}

// =============================================================================
// Conversions
// =============================================================================

impl From<IrLimitError> for IrError {
    fn from(error: IrLimitError) -> Self {
        Self::limit(error)
    }
}

impl From<IrIdentifierError> for IrError {
    fn from(error: IrIdentifierError) -> Self {
        Self::identifier(error)
    }
}

impl From<IrVersionError> for IrError {
    fn from(error: IrVersionError) -> Self {
        Self::version(error)
    }
}

impl From<IrSerializationError> for IrError {
    fn from(error: IrSerializationError) -> Self {
        Self::serialization(error)
    }
}

impl From<IrExtensionError> for IrError {
    fn from(error: IrExtensionError) -> Self {
        Self::extension(error)
    }
}

// =============================================================================
// Generic module-error constructor
// =============================================================================

/// Constructs a canonical IR error for a particular semantic domain.
///
/// This function is useful for later modules such as `pulse.rs`,
/// `waveform.rs`, `measurement.rs`, `schedule.rs`, and `mapping.rs` without
/// requiring those modules to modify this file.
pub fn module_error<S: Into<String>>(
    kind: IrErrorKind,
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(kind, code, message)
}

// =============================================================================
// Common constructors for future modules
// =============================================================================

/// Creates a qubit-domain error without importing `qubit.rs`.
pub fn qubit_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Qubit,
        code,
        message,
    )
}

/// Creates a classical-domain error.
pub fn classical_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Classical,
        code,
        message,
    )
}

/// Creates a parameter-domain error.
pub fn parameter_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Parameter,
        code,
        message,
    )
}

/// Creates a gate-domain error.
pub fn gate_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Gate,
        code,
        message,
    )
}

/// Creates a measurement-domain error.
pub fn measurement_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Measurement,
        code,
        message,
    )
}

/// Creates a control-flow error.
pub fn control_flow_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::ControlFlow,
        code,
        message,
    )
}

/// Creates a timing-domain error.
pub fn timing_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Timing,
        code,
        message,
    )
}

/// Creates a pulse-domain error.
pub fn pulse_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Pulse,
        code,
        message,
    )
}

/// Creates a waveform-domain error.
pub fn waveform_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Waveform,
        code,
        message,
    )
}

/// Creates a channel-domain error.
pub fn channel_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Channel,
        code,
        message,
    )
}

/// Creates a frame-domain error.
pub fn frame_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Frame,
        code,
        message,
    )
}

/// Creates a schedule-domain error.
pub fn schedule_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Schedule,
        code,
        message,
    )
}

/// Creates a resource-domain error.
pub fn resource_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Resource,
        code,
        message,
    )
}

/// Creates a capability-domain error.
pub fn capability_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Capability,
        code,
        message,
    )
}

/// Creates a mapping-domain error.
pub fn mapping_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Mapping,
        code,
        message,
    )
}

/// Creates a program-domain error.
pub fn program_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Program,
        code,
        message,
    )
}

/// Creates a region-domain error.
pub fn region_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Region,
        code,
        message,
    )
}

/// Creates a circuit-domain error.
pub fn circuit_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Circuit,
        code,
        message,
    )
}

/// Creates an operation-domain error.
pub fn operation_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Operation,
        code,
        message,
    )
}

/// Creates a validation-domain error.
pub fn validation_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Validation,
        code,
        message,
    )
}

/// Creates an analysis-domain error.
pub fn analysis_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(
        IrErrorKind::Analysis,
        code,
        message,
    )
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_compiles() {
        let result: IrResult<()> = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn error_code_is_stable() {
        assert_eq!(
            IrErrorCode::LimitExceeded.to_string(),
            "IR-LIMIT-001"
        );

        assert_eq!(
            IrErrorCode::InvalidQubit.to_string(),
            "IR-QUBIT-001"
        );

        assert_eq!(
            IrErrorCode::InvalidPulse.to_string(),
            "IR-PULSE-001"
        );
    }

    #[test]
    fn location_is_deterministic() {
        let location = IrLocation::new()
            .with_source("example.zm")
            .with_line(10)
            .with_column(5);

        assert_eq!(
            location.to_string(),
            "example.zm:10:5"
        );
    }

    #[test]
    fn limit_error_is_structured() {
        let error = IrLimitError::new(
            "logical_qubits",
            100,
            10,
        );

        assert_eq!(
            error.limit,
            "logical_qubits"
        );

        assert_eq!(error.actual, 100);
        assert_eq!(error.maximum, 10);
    }

    #[test]
    fn canonical_limit_conversion_works() {
        let error = IrError::from(
            IrLimitError::new(
                "operations",
                101,
                100,
            )
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Limits
        );

        assert_eq!(
            error.code(),
            IrErrorCode::LimitExceeded
        );

        assert!(error.is_error());
    }

    #[test]
    fn identifier_conversion_works() {
        let error = IrError::from(
            IrIdentifierError::Duplicate {
                domain: "logical_qubit",
                value: 7,
            }
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Identifier
        );

        assert_eq!(
            error.code(),
            IrErrorCode::DuplicateIdentifier
        );
    }

    #[test]
    fn version_conversion_is_structured() {
        let error = IrError::from(
            IrVersionError::Future {
                major: 2,
                minor: 0,
                patch: 0,
            }
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Version
        );

        assert_eq!(
            error.code(),
            IrErrorCode::FutureVersion
        );
    }

    #[test]
    fn serialization_conversion_is_structured() {
        let error = IrError::from(
            IrSerializationError::Truncated
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Serialization
        );

        assert_eq!(
            error.code(),
            IrErrorCode::TruncatedData
        );
    }

    #[test]
    fn extension_conversion_is_structured() {
        let error = IrError::from(
            IrExtensionError::Unsupported {
                name: "example.vendor.operation".to_string(),
            }
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Extension
        );

        assert_eq!(
            error.code(),
            IrErrorCode::UnsupportedExtension
        );
    }

    #[test]
    fn pulse_constructor_is_independent() {
        let error = pulse_error(
            IrErrorCode::InvalidPulse,
            "pulse duration must be positive",
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Pulse
        );
    }

    #[test]
    fn qubit_constructor_does_not_depend_on_qubit_module() {
        let error = qubit_error(
            IrErrorCode::InvalidQubit,
            "invalid logical qubit reference",
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Qubit
        );
    }

    #[test]
    fn no_fixed_machine_size_is_encoded() {
        let error = IrLimitError::new(
            "logical_qubits",
            u64::MAX,
            u64::MAX - 1,
        );

        assert_eq!(
            error.actual,
            u64::MAX
        );
    }

    #[test]
    fn diagnostics_include_machine_code() {
        let error = IrError::new(
            IrErrorKind::Pulse,
            IrErrorCode::InvalidPulse,
            "pulse amplitude is outside the permitted semantic range",
        );

        let diagnostic = error.to_string();

        assert!(
            diagnostic.contains("IR-PULSE-001")
        );

        assert!(
            diagnostic.contains("pulse amplitude")
        );
    }

    #[test]
    fn location_can_be_attached_after_construction() {
        let error = IrError::new(
            IrErrorKind::Validation,
            IrErrorCode::SemanticValidationFailed,
            "invalid operation",
        )
        .with_location(
            IrLocation::new()
                .with_source("program.zm")
                .with_line(42),
        );

        assert!(
            error
                .location()
                .is_some()
        );

        assert!(
            error
                .to_string()
                .contains("program.zm:42")
        );
    }

    #[test]
    fn warnings_are_not_errors() {
        let diagnostic = IrError::warning(
            IrErrorKind::Unsupported,
            IrErrorCode::UnsupportedFeature,
            "optional feature unavailable",
        );

        assert!(diagnostic.is_warning());
        assert!(!diagnostic.is_error());
    }

    #[test]
    fn informational_diagnostics_are_supported() {
        let diagnostic = IrError::info(
            IrErrorKind::Analysis,
            IrErrorCode::UnsupportedFeature,
            "analysis information",
        );

        assert!(diagnostic.is_info());
    }
}