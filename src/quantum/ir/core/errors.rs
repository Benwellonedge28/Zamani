//! Zamani Quantum IR — Canonical Error and Diagnostic Model
//!
//! This module defines the foundational, target-independent error vocabulary
//! for the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `core::errors` is a Tier-0 IR module. It must remain independent from
//! higher-level quantum IR modules so that all other IR layers can depend on
//! it without creating circular dependencies.
//!
//! The error model is designed for:
//!
//! - gate-based quantum computation;
//! - dynamic circuits;
//! - classical control;
//! - symbolic parameters;
//! - pulse-level control;
//! - analog/Hamiltonian computation;
//! - annealing and QUBO;
//! - logical/fault-tolerant computation;
//! - distributed quantum computation;
//! - continuous-variable and photonic computation;
//! - vendor extensions;
//! - serialization and schema migration;
//! - compiler diagnostics;
//! - resource-policy enforcement.
//!
//! # Dependency boundary
//!
//! This file intentionally does NOT depend on:
//!
//! - `quantum::ir::qubit`;
//! - `gate`;
//! - `measurement`;
//! - `operation`;
//! - `program`;
//! - `pulse`;
//! - `waveform`;
//! - `channel`;
//! - `frame`;
//! - `schedule`;
//! - `resource`;
//! - `capability`;
//! - `mapping`;
//! - `validation`;
//! - `analysis`;
//! - frontend;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulator;
//! - QEC;
//! - backend execution.
//!
//! This is deliberate.
//!
//! `quantum::ir::qubit` owns quantum identity. This module only owns the
//! diagnostic vocabulary used to report failures involving those identities.
//!
//! For example, `qubit.rs` may call:
//!
//! ```text
//! qubit_error(
//!     IrErrorCode::InvalidQubit,
//!     "logical qubit q17 is not declared",
//! )
//! ```
//!
//! without making the foundational error layer depend on `qubit.rs`.
//!
//! # Universal scalability
//!
//! No semantic quantum-machine size is encoded here.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum register size;
//! - maximum circuit depth;
//! - maximum operation count;
//! - maximum topology size;
//! - vendor-specific machine size.
//!
//! Runtime/compiler resource limits are policy values supplied by higher-level
//! components and are represented here as structured policy violations.
//!
//! `u64` is used for portable counters in diagnostics. It is not a statement
//! that Zamani's semantic universe is limited to `u64`.
//!
//! # Determinism
//!
//! Canonical errors are:
//!
//! - cloneable;
//! - comparable;
//! - hashable where appropriate;
//! - deterministic to format;
//! - independent of memory addresses;
//! - independent of hash-map ordering;
//! - independent of process identity.
//!
//! Error messages are diagnostic text. Consumers must use `IrErrorKind` and
//! `IrErrorCode` for machine-readable classification.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! Requirements:
//!
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`;
//! - Rust 2021 compatible.

use std::error::Error;
use std::fmt;

// ============================================================================
// Result aliases
// ============================================================================

/// Canonical result type for APIs which can fail with a Zamani IR error.
pub type IrResult<T> = Result<T, IrError>;

/// Explicit diagnostic result alias.
///
/// This exists as a semantic alias for APIs whose primary purpose is
/// diagnostic-producing validation or analysis.
pub type IrDiagnosticResult<T> = Result<T, IrError>;

// ============================================================================
// Severity
// ============================================================================

/// Severity of an IR diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorSeverity {
    /// Informational diagnostic.
    Info,

    /// Recoverable warning.
    Warning,

    /// Error preventing the requested operation from completing correctly.
    Error,
}

impl fmt::Display for IrErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => f.write_str("info"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}

// ============================================================================
// Error categories
// ============================================================================

/// Stable high-level category for an IR diagnostic.
///
/// This category is intentionally broader than individual files. Individual
/// IR modules should not need to modify this file merely because a new
/// operation or quantum architecture is introduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorKind {
    /// Configured resource/security policy was violated.
    Limits,

    /// An IR identifier is invalid or inconsistent.
    Identifier,

    /// Quantum-resource semantics are invalid.
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

    /// Program-level structure is invalid.
    Program,

    /// Region/block structure is invalid.
    Region,

    /// Circuit structure is invalid.
    Circuit,

    /// Generic operation structure is invalid.
    Operation,

    /// Generic structural representation is invalid.
    ///
    /// This category exists because structural errors are not always owned by
    /// a specific semantic subsystem.
    InvalidStructure,

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

    /// Requested functionality is outside the current semantic contract.
    Unsupported,

    /// A required invariant was violated.
    Invariant,

    /// Internal compiler/IR implementation failure.
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
            Self::InvalidStructure => "invalid_structure",
            Self::Validation => "validation",
            Self::Analysis => "analysis",
            Self::Serialization => "serialization",
            Self::Version => "version",
            Self::Extension => "extension",
            Self::Unsupported => "unsupported",
            Self::Invariant => "invariant",
            Self::Internal => "internal",
        };

        f.write_str(value)
    }
}

// ============================================================================
// Stable machine-readable error codes
// ============================================================================

/// Stable machine-readable IR error code.
///
/// These values form a public diagnostic protocol. Human-readable error
/// messages may evolve without changing these codes.
///
/// Do not use error-message text for programmatic classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IrErrorCode {
    // ------------------------------------------------------------------------
    // Limits
    // ------------------------------------------------------------------------

    /// Configured limit was exceeded.
    LimitExceeded,

    /// A resource count overflowed its representation.
    ResourceOverflow,

    /// A size calculation overflowed.
    SizeOverflow,

    /// A request was rejected by an explicit allocation policy.
    AllocationRejected,

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    /// Identifier is invalid.
    InvalidIdentifier,

    /// Identifier is duplicated.
    DuplicateIdentifier,

    /// Identifier is outside a declared namespace.
    IdentifierOutOfRange,

    /// Two identity domains were incorrectly mixed.
    IdentityDomainMismatch,

    // ------------------------------------------------------------------------
    // Structure
    // ------------------------------------------------------------------------

    /// Required data is missing.
    MissingData,

    /// Data was supplied where it is not valid.
    UnexpectedData,

    /// Generic structural representation is invalid.
    InvalidStructure,

    /// Required operand is missing.
    MissingOperand,

    /// Operand is invalid.
    InvalidOperand,

    /// Operand arity is invalid.
    InvalidArity,

    // ------------------------------------------------------------------------
    // Type and values
    // ------------------------------------------------------------------------

    /// Value has the wrong type.
    TypeMismatch,

    /// Value is outside its semantic domain.
    InvalidValue,

    /// Numeric value is NaN/infinite where finite data is required.
    NonFiniteValue,

    /// Parameter expression is invalid.
    InvalidExpression,

    /// Symbolic parameter has no required binding.
    UnboundParameter,

    // ------------------------------------------------------------------------
    // Quantum/classical domains
    // ------------------------------------------------------------------------

    /// Quantum-bit identity/reference is invalid.
    InvalidQubit,

    /// Classical resource is invalid.
    InvalidClassicalResource,

    /// Gate is invalid.
    InvalidGate,

    /// Measurement is invalid.
    InvalidMeasurement,

    /// Control-flow construct is invalid.
    InvalidControlFlow,

    /// Pulse is invalid.
    InvalidPulse,

    /// Waveform is invalid.
    InvalidWaveform,

    /// Channel is invalid.
    InvalidChannel,

    /// Frame is invalid.
    InvalidFrame,

    /// Timing relation is invalid.
    InvalidTiming,

    /// Schedule is invalid.
    InvalidSchedule,

    /// Resource requirement is invalid.
    InvalidResourceRequirement,

    /// Capability requirement is invalid.
    InvalidCapabilityRequirement,

    /// Mapping is invalid.
    InvalidMapping,

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Versioning
    // ------------------------------------------------------------------------

    /// IR version is unsupported.
    UnsupportedVersion,

    /// Future IR version was encountered.
    FutureVersion,

    /// IR version data is malformed.
    InvalidVersion,

    /// Required migration/compatibility conversion is unavailable.
    CompatibilityConversionUnavailable,

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    /// Serialization failed.
    SerializationFailed,

    /// Deserialization failed.
    DeserializationFailed,

    /// Serialized data is malformed.
    MalformedData,

    /// Serialized data is truncated.
    TruncatedData,

    /// Serialized representation exceeds configured policy.
    SerializedSizeExceeded,

    // ------------------------------------------------------------------------
    // Extensions
    // ------------------------------------------------------------------------

    /// Extension is invalid.
    InvalidExtension,

    /// Extension is unsupported.
    UnsupportedExtension,

    /// Extension violates an invariant.
    ExtensionInvariantViolation,

    // ------------------------------------------------------------------------
    // Support
    // ------------------------------------------------------------------------

    /// Feature is not supported by the current semantic layer.
    UnsupportedFeature,

    /// Feature cannot be represented by the current IR.
    UnrepresentableFeature,

    // ------------------------------------------------------------------------
    // Internal
    // ------------------------------------------------------------------------

    /// Internal invariant failed.
    InternalInvariant,

    /// Internal implementation failure occurred.
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

        f.write_str(value)
    }
}

// ============================================================================
// Diagnostic location
// ============================================================================

/// Generic source/IR location.
///
/// This type deliberately does not reference any specific IR identity type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct IrLocation {
    /// Optional source/module/file name.
    pub source: Option<String>,

    /// Optional one-based source line.
    pub line: Option<u64>,

    /// Optional one-based source column.
    pub column: Option<u64>,

    /// Optional byte offset.
    pub byte_offset: Option<u64>,

    /// Optional byte length.
    pub byte_length: Option<u64>,

    /// Optional logical IR object description.
    pub object: Option<String>,

    /// Optional IR/source path.
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

    /// Sets the source name.
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

    /// Sets a logical object description.
    pub fn with_object<S: Into<String>>(mut self, object: S) -> Self {
        self.object = Some(object.into());
        self
    }

    /// Sets an IR/source path.
    pub fn with_path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Returns whether no location information is present.
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
            f.write_str(source)?;
            wrote = true;
        }

        if let Some(line) = self.line {
            if wrote {
                f.write_str(":")?;
            }

            write!(f, "{line}")?;
            wrote = true;

            if let Some(column) = self.column {
                write!(f, ":{column}")?;
            }
        }

        if let Some(byte_offset) = self.byte_offset {
            if wrote {
                f.write_str(" ")?;
            }

            write!(f, "byte={byte_offset}")?;
            wrote = true;

            if let Some(byte_length) = self.byte_length {
                write!(f, "..{}", byte_offset.saturating_add(byte_length))?;
            }
        }

        if let Some(object) = &self.object {
            if wrote {
                f.write_str(" ")?;
            }

            write!(f, "[{object}]")?;
            wrote = true;
        }

        if let Some(path) = &self.path {
            if wrote {
                f.write_str(" ")?;
            }

            f.write_str(path)?;
            wrote = true;
        }

        if !wrote {
            f.write_str("<unknown>")?;
        }

        Ok(())
    }
}

// ============================================================================
// Structured resource-limit error
// ============================================================================

/// Structured resource/security-policy violation.
///
/// The policy name is intentionally a string so `errors.rs` does not depend
/// on `limits.rs`. This keeps the dependency graph acyclic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrLimitError {
    /// Stable logical name of the configured policy.
    pub limit: String,

    /// Actual/requested amount.
    pub actual: u64,

    /// Configured maximum.
    pub maximum: u64,
}

impl IrLimitError {
    /// Creates a limit violation.
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

// ============================================================================
// Identifier errors
// ============================================================================

/// Structured identifier failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrIdentifierError {
    /// Identifier is outside the declared namespace.
    OutOfRange {
        /// Numeric identifier.
        value: u64,

        /// Namespace cardinality.
        count: u64,

        /// Logical identifier domain.
        domain: &'static str,
    },

    /// Identifier is duplicated.
    Duplicate {
        /// Logical identifier domain.
        domain: &'static str,

        /// Numeric identifier.
        value: u64,
    },

    /// Two identifier domains were incorrectly mixed.
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

    /// Identifier is otherwise invalid.
    Invalid {
        /// Identifier domain.
        domain: &'static str,

        /// Numeric identifier.
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

// ============================================================================
// Structured semantic error
// ============================================================================

/// Generic structured semantic failure.
///
/// Higher-level modules can use this without defining another foundational
/// error dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrSemanticError {
    /// Semantic domain.
    pub domain: &'static str,

    /// Human-readable reason.
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

// ============================================================================
// Version errors
// ============================================================================

/// Structured IR version compatibility failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrVersionError {
    /// Version is unsupported.
    Unsupported {
        major: u16,
        minor: u16,
        patch: u16,
    },

    /// A newer/future version was encountered.
    Future {
        major: u16,
        minor: u16,
        patch: u16,
    },

    /// Version data is invalid.
    Invalid {
        major: u16,
        minor: u16,
        patch: u16,
    },

    /// Required compatibility conversion is unavailable.
    ConversionUnavailable {
        from: String,
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

// ============================================================================
// Serialization errors
// ============================================================================

/// Structured serialization/deserialization failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrSerializationError {
    /// Serialization failed.
    Failed {
        reason: String,
    },

    /// Serialized input cannot be interpreted as an IR value.
    InvalidData {
        reason: String,
    },

    /// Input ended before the complete representation was read.
    Truncated,

    /// Input is syntactically/structurally malformed.
    Malformed {
        reason: String,
    },

    /// Input/output exceeded an explicit policy.
    SizeExceeded {
        actual: u64,
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
                f.write_str("IR serialized data is truncated")
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

// ============================================================================
// Extension errors
// ============================================================================

/// Structured extension failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrExtensionError {
    /// Extension is malformed.
    Invalid {
        name: String,
        reason: String,
    },

    /// Extension is not supported by the current implementation.
    Unsupported {
        name: String,
    },

    /// Extension violates a semantic invariant.
    InvariantViolation {
        name: String,
        reason: String,
    },
}

impl fmt::Display for IrExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid { name, reason } => {
                write!(f, "invalid IR extension `{name}`: {reason}")
            }

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

// ============================================================================
// Canonical IR error
// ============================================================================

/// Canonical structured Zamani Quantum IR error.
///
/// This is the common error type used by foundational and higher-level IR
/// modules.
///
/// The representation deliberately uses owned strings instead of boxed
/// arbitrary error objects. This gives the canonical IR error:
///
/// - deterministic formatting;
/// - `Clone`;
/// - `Eq`;
/// - stable serialization potential;
/// - no lifetime requirements;
/// - no dependency on third-party error frameworks.
///
/// Lower-level errors can be preserved through `cause`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrError {
    kind: IrErrorKind,
    code: IrErrorCode,
    severity: IrErrorSeverity,
    message: String,
    location: Option<IrLocation>,
    cause: Option<String>,
}

impl IrError {
    /// Creates an error-level diagnostic.
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

    /// Returns the machine-readable code.
    pub const fn code(&self) -> IrErrorCode {
        self.code
    }

    /// Returns the diagnostic severity.
    pub const fn severity(&self) -> IrErrorSeverity {
        self.severity
    }

    /// Returns the human-readable message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional diagnostic location.
    pub fn location(&self) -> Option<&IrLocation> {
        self.location.as_ref()
    }

    /// Returns the optional causal diagnostic.
    pub fn cause_message(&self) -> Option<&str> {
        self.cause.as_deref()
    }

    /// Attaches a location.
    pub fn with_location(mut self, location: IrLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Attaches a causal diagnostic.
    pub fn with_cause<S: Into<String>>(mut self, cause: S) -> Self {
        self.cause = Some(cause.into());
        self
    }

    /// Returns whether this diagnostic is an error.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Error)
    }

    /// Returns whether this diagnostic is a warning.
    pub const fn is_warning(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Warning)
    }

    /// Returns whether this diagnostic is informational.
    pub const fn is_info(&self) -> bool {
        matches!(self.severity, IrErrorSeverity::Info)
    }

    /// Creates a resource-limit error.
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

    /// Creates a semantic-domain error.
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
    pub fn serialization(error: IrSerializationError) -> Self {
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
    pub fn extension(error: IrExtensionError) -> Self {
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
    pub fn unsupported<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::Unsupported,
            IrErrorCode::UnsupportedFeature,
            message,
        )
    }

    /// Creates an invariant violation.
    pub fn invariant<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::Invariant,
            IrErrorCode::InternalInvariant,
            message,
        )
    }

    /// Creates an internal implementation error.
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::Internal,
            IrErrorCode::InternalFailure,
            message,
        )
    }

    /// Creates a missing-data error.
    pub fn missing_data<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::MissingData,
            message,
        )
    }

    /// Creates an invalid-structure error.
    pub fn invalid_structure<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::InvalidStructure,
            message,
        )
    }

    /// Creates a missing-operand error.
    pub fn missing_operand<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::MissingOperand,
            message,
        )
    }

    /// Creates an invalid-operand error.
    pub fn invalid_operand<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::InvalidOperand,
            message,
        )
    }

    /// Creates an arity error.
    pub fn invalid_arity<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::InvalidArity,
            message,
        )
    }

    /// Creates a type-mismatch error.
    pub fn type_mismatch<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::TypeMismatch,
            message,
        )
    }

    /// Creates an invalid-value error.
    pub fn invalid_value<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::InvalidStructure,
            IrErrorCode::InvalidValue,
            message,
        )
    }

    /// Creates a non-finite numeric-value error.
    pub fn non_finite_value<S: Into<String>>(message: S) -> Self {
        Self::new(
            IrErrorKind::Parameter,
            IrErrorCode::NonFiniteValue,
            message,
        )
    }

    /// Returns a stable diagnostic string.
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

// ============================================================================
// Conversions
// ============================================================================

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

impl From<IrSemanticError> for IrError {
    fn from(error: IrSemanticError) -> Self {
        Self::semantic(
            IrErrorKind::Validation,
            IrErrorCode::SemanticValidationFailed,
            error,
        )
    }
}

// ============================================================================
// Generic constructor
// ============================================================================

/// Constructs a canonical error for a semantic module.
pub fn module_error<S: Into<String>>(
    kind: IrErrorKind,
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(kind, code, message)
}

// ============================================================================
// Domain-specific constructors
// ============================================================================

/// Creates a qubit-domain error.
///
/// This function deliberately does not import `quantum::ir::qubit`.
/// `qubit.rs` owns the actual `QubitId` type and can use this constructor
/// without creating a foundational dependency cycle.
pub fn qubit_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Qubit, code, message)
}

/// Creates a classical-domain error.
pub fn classical_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Classical, code, message)
}

/// Creates a parameter-domain error.
pub fn parameter_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Parameter, code, message)
}

/// Creates a gate-domain error.
pub fn gate_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Gate, code, message)
}

/// Creates a measurement-domain error.
pub fn measurement_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Measurement, code, message)
}

/// Creates a control-flow error.
pub fn control_flow_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::ControlFlow, code, message)
}

/// Creates a timing-domain error.
pub fn timing_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Timing, code, message)
}

/// Creates a pulse-domain error.
pub fn pulse_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Pulse, code, message)
}

/// Creates a waveform-domain error.
pub fn waveform_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Waveform, code, message)
}

/// Creates a channel-domain error.
pub fn channel_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Channel, code, message)
}

/// Creates a frame-domain error.
pub fn frame_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Frame, code, message)
}

/// Creates a schedule-domain error.
pub fn schedule_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Schedule, code, message)
}

/// Creates a resource-domain error.
pub fn resource_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Resource, code, message)
}

/// Creates a capability-domain error.
pub fn capability_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Capability, code, message)
}

/// Creates a mapping-domain error.
pub fn mapping_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Mapping, code, message)
}

/// Creates a program-domain error.
pub fn program_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Program, code, message)
}

/// Creates a region-domain error.
pub fn region_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Region, code, message)
}

/// Creates a circuit-domain error.
pub fn circuit_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Circuit, code, message)
}

/// Creates an operation-domain error.
pub fn operation_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Operation, code, message)
}

/// Creates a validation-domain error.
pub fn validation_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Validation, code, message)
}

/// Creates an analysis-domain error.
pub fn analysis_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Analysis, code, message)
}

/// Creates a serialization-domain error directly.
pub fn serialization_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Serialization, code, message)
}

/// Creates a version-domain error directly.
pub fn version_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Version, code, message)
}

/// Creates an extension-domain error directly.
pub fn extension_error<S: Into<String>>(
    code: IrErrorCode,
    message: S,
) -> IrError {
    IrError::new(IrErrorKind::Extension, code, message)
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_compiles() {
        let result: IrResult<()> = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn diagnostic_result_alias_compiles() {
        let result: IrDiagnosticResult<()> = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn structural_kind_exists() {
        let error = IrError::invalid_structure("invalid block");
        assert_eq!(
            error.kind(),
            IrErrorKind::InvalidStructure
        );
        assert_eq!(
            error.code(),
            IrErrorCode::InvalidStructure
        );
    }

    #[test]
    fn stable_error_codes_are_deterministic() {
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

        assert_eq!(
            IrErrorCode::InvalidStructure.to_string(),
            "IR-STRUCT-003"
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
    fn location_supports_ir_object_paths() {
        let location = IrLocation::new()
            .with_object("operation")
            .with_path("module.main.region[0].block[2].op[17]");

        let text = location.to_string();

        assert!(text.contains("[operation]"));
        assert!(text.contains("module.main.region[0].block[2].op[17]"));
    }

    #[test]
    fn limit_error_is_structured() {
        let error = IrLimitError::new(
            "logical_qubits",
            100,
            10,
        );

        assert_eq!(error.limit, "logical_qubits");
        assert_eq!(error.actual, 100);
        assert_eq!(error.maximum, 10);
    }

    #[test]
    fn limit_error_converts_to_canonical_error() {
        let error = IrError::from(
            IrLimitError::new(
                "operations",
                101,
                100,
            )
        );

        assert_eq!(error.kind(), IrErrorKind::Limits);
        assert_eq!(
            error.code(),
            IrErrorCode::LimitExceeded
        );
        assert!(error.is_error());
    }

    #[test]
    fn identifier_conversion_is_structured() {
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
    fn identifier_domain_mismatch_is_distinct() {
        let error = IrError::from(
            IrIdentifierError::DomainMismatch {
                expected: "logical_qubit",
                actual: "physical_qubit",
            }
        );

        assert_eq!(
            error.code(),
            IrErrorCode::IdentityDomainMismatch
        );
    }

    #[test]
    fn semantic_error_conversion_is_structured() {
        let error = IrError::from(
            IrSemanticError::new(
                "pulse",
                "duration must be non-negative",
            )
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Validation
        );

        assert_eq!(
            error.code(),
            IrErrorCode::SemanticValidationFailed
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
                name: "vendor.example.operation".to_string(),
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
    fn qubit_constructor_is_hardware_independent() {
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
    fn pulse_constructor_is_independent() {
        let error = pulse_error(
            IrErrorCode::InvalidPulse,
            "pulse duration is invalid",
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Pulse
        );
    }

    #[test]
    fn operation_constructor_is_independent() {
        let error = operation_error(
            IrErrorCode::InvalidOperand,
            "operation operand is invalid",
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Operation
        );
    }

    #[test]
    fn validation_constructor_is_independent() {
        let error = validation_error(
            IrErrorCode::StructuralValidationFailed,
            "operation references an undeclared value",
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Validation
        );
    }

    #[test]
    fn warnings_are_not_errors() {
        let diagnostic = IrError::warning(
            IrErrorKind::Unsupported,
            IrErrorCode::UnsupportedFeature,
            "optional capability is unavailable",
        );

        assert!(diagnostic.is_warning());
        assert!(!diagnostic.is_error());
    }

    #[test]
    fn information_is_not_error() {
        let diagnostic = IrError::info(
            IrErrorKind::Analysis,
            IrErrorCode::UnsupportedFeature,
            "analysis information",
        );

        assert!(diagnostic.is_info());
        assert!(!diagnostic.is_error());
    }

    #[test]
    fn location_can_be_attached() {
        let error = IrError::new(
            IrErrorKind::Validation,
            IrErrorCode::SemanticValidationFailed,
            "invalid operation",
        )
        .with_location(
            IrLocation::new()
                .with_source("program.zm")
                .with_line(42)
                .with_column(8),
        );

        assert!(error.location().is_some());

        let diagnostic = error.to_string();

        assert!(diagnostic.contains("program.zm:42:8"));
    }

    #[test]
    fn cause_can_be_attached() {
        let error = IrError::internal(
            "serialization subsystem failed",
        )
        .with_cause("unexpected end of input");

        assert_eq!(
            error.cause_message(),
            Some("unexpected end of input")
        );

        assert!(
            error
                .to_string()
                .contains("unexpected end of input")
        );
    }

    #[test]
    fn diagnostic_contains_machine_code() {
        let error = IrError::new(
            IrErrorKind::Pulse,
            IrErrorCode::InvalidPulse,
            "pulse amplitude is invalid",
        );

        let diagnostic = error.diagnostic();

        assert!(diagnostic.contains("IR-PULSE-001"));
        assert!(diagnostic.contains("pulse amplitude"));
    }

    #[test]
    fn large_resource_values_are_supported() {
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
    fn no_machine_size_constant_exists() {
        let error = IrLimitError::new(
            "logical_qubits",
            1_000_000,
            2_000_000,
        );

        assert_eq!(
            error.limit,
            "logical_qubits"
        );
    }

    #[test]
    fn formatting_is_deterministic() {
        let first = IrError::new(
            IrErrorKind::Gate,
            IrErrorCode::InvalidGate,
            "invalid gate",
        );

        let second = IrError::new(
            IrErrorKind::Gate,
            IrErrorCode::InvalidGate,
            "invalid gate",
        );

        assert_eq!(
            first.to_string(),
            second.to_string()
        );
    }

    #[test]
    fn all_domain_constructors_preserve_domain() {
        assert_eq!(
            classical_error(
                IrErrorCode::InvalidClassicalResource,
                "invalid bit",
            )
            .kind(),
            IrErrorKind::Classical
        );

        assert_eq!(
            parameter_error(
                IrErrorCode::InvalidExpression,
                "invalid expression",
            )
            .kind(),
            IrErrorKind::Parameter
        );

        assert_eq!(
            gate_error(
                IrErrorCode::InvalidGate,
                "invalid gate",
            )
            .kind(),
            IrErrorKind::Gate
        );

        assert_eq!(
            measurement_error(
                IrErrorCode::InvalidMeasurement,
                "invalid measurement",
            )
            .kind(),
            IrErrorKind::Measurement
        );

        assert_eq!(
            control_flow_error(
                IrErrorCode::InvalidControlFlow,
                "invalid branch",
            )
            .kind(),
            IrErrorKind::ControlFlow
        );

        assert_eq!(
            timing_error(
                IrErrorCode::InvalidTiming,
                "invalid timing",
            )
            .kind(),
            IrErrorKind::Timing
        );
    }
}