//! Zamani Quantum IR — Canonical Error Model
//!
//! This module defines the stable, hardware-independent error vocabulary for
//! the Quantum Intermediate Representation (IR).
//!
//! Design goals:
//! - preserve structured failure information;
//! - avoid converting errors into opaque strings;
//! - remain independent from the other IR modules;
//! - support deterministic diagnostics;
//! - support resource/limit enforcement;
//! - provide a stable top-level `IrError` for compiler integration;
//! - remain compatible with Rust 1.97.1.
//!
//! IMPORTANT ARCHITECTURAL BOUNDARY
//! --------------------------------
//! `errors.rs` intentionally does NOT import:
//! - gate.rs
//! - circuit.rs
//! - measurement.rs
//! - qubits.rs
//! - validation.rs
//! - limits.rs
//! - identity.rs
//!
//! Those modules may later implement `From<TheirError> for IrError`.
//! Keeping the dependency direction this way prevents the canonical error
//! vocabulary from becoming coupled to individual IR implementations.
//!
//! The IR error model is hardware-independent. Backend, routing, scheduling,
//! QPU, calibration, and device-specific errors must be translated into their
//! respective higher-level compiler/backend error domains rather than added
//! here.

use std::error::Error;
use std::fmt;

// -----------------------------------------------------------------------------
// Result alias
// -----------------------------------------------------------------------------

/// Canonical result type for Quantum IR operations.
///
/// Individual IR modules may continue to expose specialized error types for
/// local construction/validation details. Public IR boundaries should convert
/// those errors into `IrError` where appropriate.
pub type IrResult<T> = Result<T, IrError>;

// -----------------------------------------------------------------------------
// Error category
// -----------------------------------------------------------------------------

/// Stable high-level category for a Quantum IR failure.
///
/// This category is intentionally independent from the concrete error payload.
/// Consumers can classify failures without parsing diagnostic strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrErrorKind {
    /// Resource or configured IR limit was exceeded.
    Limits,

    /// A logical or classical identifier is invalid.
    Identifier,

    /// Qubit-related structural or semantic failure.
    Qubit,

    /// Measurement-related structural or semantic failure.
    Measurement,

    /// Gate-related structural or semantic failure.
    Gate,

    /// Circuit container or mutation failure.
    Circuit,

    /// Parameter validation or parameter-shape failure.
    Parameter,

    /// Whole-IR validation failure.
    Validation,

    /// The IR contains an invalid structural combination.
    InvalidStructure,

    /// Serialization/deserialization failure.
    Serialization,

    /// IR schema/version incompatibility.
    Version,

    /// A requested operation is not supported by this IR contract.
    Unsupported,

    /// An internal invariant was violated.
    Invariant,
}

impl fmt::Display for IrErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Limits => "limits",
            Self::Identifier => "identifier",
            Self::Qubit => "qubit",
            Self::Measurement => "measurement",
            Self::Gate => "gate",
            Self::Circuit => "circuit",
            Self::Parameter => "parameter",
            Self::Validation => "validation",
            Self::InvalidStructure => "invalid_structure",
            Self::Serialization => "serialization",
            Self::Version => "version",
            Self::Unsupported => "unsupported",
            Self::Invariant => "invariant",
        };

        write!(f, "{name}")
    }
}

// -----------------------------------------------------------------------------
// Limit errors
// -----------------------------------------------------------------------------

/// Resource-limit violation.
///
/// This is deliberately self-contained so `errors.rs` does not need to depend
/// on `limits.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrLimitError {
    /// Stable name of the violated limit.
    pub limit: &'static str,

    /// Observed/requested value.
    pub actual: usize,

    /// Maximum permitted value.
    pub maximum: usize,
}

impl IrLimitError {
    /// Creates a limit violation.
    pub const fn new(
        limit: &'static str,
        actual: usize,
        maximum: usize,
    ) -> Self {
        Self {
            limit,
            actual,
            maximum,
        }
    }
}

impl fmt::Display for IrLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IR limit `{}` exceeded: actual={}, maximum={}",
            self.limit,
            self.actual,
            self.maximum
        )
    }
}

impl Error for IrLimitError {}

// -----------------------------------------------------------------------------
// Identifier errors
// -----------------------------------------------------------------------------

/// Invalid identifier information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrIdentifierError {
    /// Logical qubit index is outside the circuit namespace.
    QubitOutOfRange {
        index: usize,
        count: usize,
    },

    /// Classical bit index is outside the circuit namespace.
    ClassicalBitOutOfRange {
        index: usize,
        count: usize,
    },

    /// An operation identifier is invalid for the current operation list.
    OperationOutOfRange {
        index: usize,
        count: usize,
    },

    /// An identifier was duplicated where uniqueness is required.
    Duplicate {
        kind: &'static str,
        index: usize,
    },

    /// Identifier namespace is invalid.
    InvalidNamespace {
        kind: &'static str,
    },
}

impl fmt::Display for IrIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitOutOfRange { index, count } => write!(
                f,
                "logical qubit index {index} is outside range 0..{count}"
            ),

            Self::ClassicalBitOutOfRange { index, count } => write!(
                f,
                "classical bit index {index} is outside range 0..{count}"
            ),

            Self::OperationOutOfRange { index, count } => write!(
                f,
                "operation index {index} is outside operation count {count}"
            ),

            Self::Duplicate { kind, index } => {
                write!(f, "{kind} identifier {index} is duplicated")
            }

            Self::InvalidNamespace { kind } => {
                write!(f, "invalid {kind} identifier namespace")
            }
        }
    }
}

impl Error for IrIdentifierError {}

// -----------------------------------------------------------------------------
// Qubit errors
// -----------------------------------------------------------------------------

/// Canonical qubit-level error payload.
///
/// `qubits.rs` can translate its local `QubitError` into this representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrQubitError {
    /// Invalid logical qubit index.
    OutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// Duplicate logical qubit in one operation.
    Duplicate {
        qubit: usize,
    },

    /// Attempted operation on a disabled qubit.
    Disabled {
        qubit: usize,
    },

    /// Requested allocation could not be satisfied.
    NoAvailableQubit,

    /// Invalid number of qubits requested.
    InvalidCount {
        count: usize,
    },

    /// Logical/physical identity misuse.
    InvalidIdentity {
        message: &'static str,
    },
}

impl fmt::Display for IrQubitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "qubit q{qubit} is outside range 0..{num_qubits}"
            ),

            Self::Duplicate { qubit } => {
                write!(f, "qubit q{qubit} appears more than once")
            }

            Self::Disabled { qubit } => {
                write!(f, "qubit q{qubit} is disabled")
            }

            Self::NoAvailableQubit => {
                write!(f, "no available logical qubit")
            }

            Self::InvalidCount { count } => {
                write!(f, "invalid logical qubit count: {count}")
            }

            Self::InvalidIdentity { message } => {
                write!(f, "invalid qubit identity: {message}")
            }
        }
    }
}

impl Error for IrQubitError {}

// -----------------------------------------------------------------------------
// Measurement errors
// -----------------------------------------------------------------------------

/// Canonical measurement-level error payload.
///
/// `measurement.rs` can translate its local `MeasurementError` into this
/// representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrMeasurementError {
    /// A measurement has no valid quantum source.
    MissingQubit,

    /// A measurement has no valid classical destination.
    MissingClassicalTarget,

    /// Measured qubit is outside the circuit namespace.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// Classical destination is outside the classical namespace.
    ClassicalBitOutOfRange {
        bit: usize,
        num_classical_bits: usize,
    },

    /// The same qubit is measured more than once in a group.
    DuplicateQubit {
        qubit: usize,
    },

    /// Multiple measurements target the same classical destination.
    DuplicateClassicalTarget {
        bit: usize,
    },

    /// Measurement semantics are invalid.
    InvalidConfiguration {
        reason: &'static str,
    },
}

impl fmt::Display for IrMeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingQubit => {
                write!(f, "measurement requires a logical qubit")
            }

            Self::MissingClassicalTarget => {
                write!(f, "measurement requires a classical destination")
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "measurement qubit q{qubit} is outside range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                f,
                "measurement classical bit c{bit} is outside range 0..{num_classical_bits}"
            ),

            Self::DuplicateQubit { qubit } => write!(
                f,
                "qubit q{qubit} is measured more than once in the same group"
            ),

            Self::DuplicateClassicalTarget { bit } => write!(
                f,
                "classical bit c{bit} is used by more than one measurement"
            ),

            Self::InvalidConfiguration { reason } => {
                write!(f, "invalid measurement configuration: {reason}")
            }
        }
    }
}

impl Error for IrMeasurementError {}

// -----------------------------------------------------------------------------
// Gate errors
// -----------------------------------------------------------------------------

/// Canonical gate-level error payload.
///
/// `gate.rs` can translate its local `GateError` into this representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrGateError {
    /// Gate received the wrong number of logical qubits.
    InvalidQubitCount {
        gate: &'static str,
        expected: usize,
        actual: usize,
    },

    /// A logical qubit occurs more than once.
    DuplicateQubit {
        qubit: usize,
    },

    /// A required gate parameter was not supplied.
    MissingParameter {
        gate: &'static str,
    },

    /// A parameter was supplied to a non-parameterized gate.
    UnexpectedParameter {
        gate: &'static str,
    },

    /// A gate parameter is invalid.
    InvalidParameter,

    /// A classical target was supplied where it is not valid.
    InvalidClassicalTarget {
        gate: &'static str,
    },

    /// Measurement gate has no classical target.
    MissingClassicalTarget,

    /// Barrier configuration is invalid.
    InvalidBarrier,

    /// Gate structure violates the IR contract.
    InvalidStructure {
        gate: &'static str,
        reason: &'static str,
    },
}

impl fmt::Display for IrGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount {
                gate,
                expected,
                actual,
            } => write!(
                f,
                "gate `{gate}` expects {expected} qubits but received {actual}"
            ),

            Self::DuplicateQubit { qubit } => {
                write!(f, "qubit q{qubit} appears more than once")
            }

            Self::MissingParameter { gate } => {
                write!(f, "gate `{gate}` requires a parameter")
            }

            Self::UnexpectedParameter { gate } => {
                write!(f, "gate `{gate}` does not accept parameters")
            }

            Self::InvalidParameter => {
                write!(f, "gate parameter must be finite")
            }

            Self::InvalidClassicalTarget { gate } => {
                write!(f, "gate `{gate}` cannot target a classical bit")
            }

            Self::MissingClassicalTarget => {
                write!(f, "measurement requires a classical target")
            }

            Self::InvalidBarrier => {
                write!(f, "invalid barrier")
            }

            Self::InvalidStructure { gate, reason } => {
                write!(
                    f,
                    "invalid structure for gate `{gate}`: {reason}"
                )
            }
        }
    }
}

impl Error for IrGateError {}

// -----------------------------------------------------------------------------
// Parameter errors
// -----------------------------------------------------------------------------

/// Parameter-level failure.
///
/// This remains independent from `parameter.rs` so the parameter module can
/// later define its own richer parameter representation without changing the
/// canonical error model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrParameterError {
    /// A floating-point parameter was NaN or infinite.
    NonFinite,

    /// The number of parameters does not match the operation contract.
    InvalidArity {
        expected: usize,
        actual: usize,
    },

    /// Parameter expression is structurally invalid.
    InvalidExpression,

    /// Parameter symbol is invalid.
    InvalidSymbol,

    /// Parameter binding is missing.
    UnboundSymbol {
        name: String,
    },
}

impl fmt::Display for IrParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => {
                write!(f, "parameter must be finite")
            }

            Self::InvalidArity { expected, actual } => write!(
                f,
                "invalid parameter count: expected {expected}, received {actual}"
            ),

            Self::InvalidExpression => {
                write!(f, "invalid parameter expression")
            }

            Self::InvalidSymbol => {
                write!(f, "invalid parameter symbol")
            }

            Self::UnboundSymbol { name } => {
                write!(f, "parameter symbol `{name}` is unbound")
            }
        }
    }
}

impl Error for IrParameterError {}

// -----------------------------------------------------------------------------
// Circuit errors
// -----------------------------------------------------------------------------

/// Canonical circuit-level error payload.
///
/// `circuit.rs` can translate its local `CircuitError` into this representation
/// without requiring this file to depend on `circuit.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrCircuitError {
    /// A qubit is outside the circuit's logical namespace.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// A classical bit is outside the circuit's classical namespace.
    ClassicalBitOutOfRange {
        bit: usize,
        num_classical_bits: usize,
    },

    /// An operation has no operands where operands are required.
    MissingOperands,

    /// An operation index is invalid.
    OperationOutOfRange {
        index: usize,
        len: usize,
    },

    /// Circuit metadata violates its configured constraints.
    InvalidMetadata,

    /// Circuit structure is invalid.
    InvalidStructure {
        reason: &'static str,
    },

    /// An operation could not be inserted without violating invariants.
    MutationRejected {
        reason: &'static str,
    },
}

impl fmt::Display for IrCircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "qubit q{qubit} is outside circuit range 0..{num_qubits}"
            ),

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => write!(
                f,
                "classical bit c{bit} is outside circuit range 0..{num_classical_bits}"
            ),

            Self::MissingOperands => {
                write!(f, "operation has no operands")
            }

            Self::OperationOutOfRange { index, len } => write!(
                f,
                "operation index {index} is outside circuit length {len}"
            ),

            Self::InvalidMetadata => {
                write!(f, "circuit metadata violates IR constraints")
            }

            Self::InvalidStructure { reason } => {
                write!(f, "invalid circuit structure: {reason}")
            }

            Self::MutationRejected { reason } => {
                write!(f, "circuit mutation rejected: {reason}")
            }
        }
    }
}

impl Error for IrCircuitError {}

// -----------------------------------------------------------------------------
// Validation errors
// -----------------------------------------------------------------------------

/// Whole-IR validation failure.
///
/// This represents failures discovered when validating an already-existing IR
/// object, including IR loaded from an external source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrValidationError {
    /// Validation found a structural violation.
    Structural {
        operation: Option<usize>,
        reason: &'static str,
    },

    /// Validation found a resource violation.
    Resource {
        resource: &'static str,
        actual: usize,
        maximum: usize,
    },

    /// Validation found a semantic violation.
    Semantic {
        operation: Option<usize>,
        reason: &'static str,
    },

    /// Validation encountered an invalid identifier.
    Identifier {
        kind: &'static str,
        index: usize,
    },

    /// Validation configuration itself is invalid.
    InvalidConfiguration {
        reason: &'static str,
    },
}

impl fmt::Display for IrValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Structural {
                operation,
                reason,
            } => match operation {
                Some(index) => write!(
                    f,
                    "structural validation failure at operation {index}: {reason}"
                ),
                None => write!(
                    f,
                    "structural validation failure: {reason}"
                ),
            },

            Self::Resource {
                resource,
                actual,
                maximum,
            } => write!(
                f,
                "validation resource limit `{resource}` exceeded: actual={actual}, maximum={maximum}"
            ),

            Self::Semantic {
                operation,
                reason,
            } => match operation {
                Some(index) => write!(
                    f,
                    "semantic validation failure at operation {index}: {reason}"
                ),
                None => write!(
                    f,
                    "semantic validation failure: {reason}"
                ),
            },

            Self::Identifier { kind, index } => {
                write!(
                    f,
                    "invalid {kind} identifier {index}"
                )
            }

            Self::InvalidConfiguration { reason } => {
                write!(
                    f,
                    "invalid validation configuration: {reason}"
                )
            }
        }
    }
}

impl Error for IrValidationError {}

// -----------------------------------------------------------------------------
// Invalid-structure errors
// -----------------------------------------------------------------------------

/// Generic structural error that does not belong to one particular IR type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStructureError {
    /// An operation is internally inconsistent.
    InconsistentOperation {
        reason: &'static str,
    },

    /// A circuit is internally inconsistent.
    InconsistentCircuit {
        reason: &'static str,
    },

    /// Two IR components disagree about a shared invariant.
    InconsistentInvariant {
        invariant: &'static str,
    },

    /// A required field is missing.
    MissingField {
        field: &'static str,
    },
}

impl fmt::Display for IrStructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InconsistentOperation { reason } => {
                write!(f, "inconsistent operation: {reason}")
            }

            Self::InconsistentCircuit { reason } => {
                write!(f, "inconsistent circuit: {reason}")
            }

            Self::InconsistentInvariant { invariant } => {
                write!(
                    f,
                    "IR invariant violated: {invariant}"
                )
            }

            Self::MissingField { field } => {
                write!(f, "required IR field `{field}` is missing")
            }
        }
    }
}

impl Error for IrStructureError {}

// -----------------------------------------------------------------------------
// Serialization errors
// -----------------------------------------------------------------------------

/// Serialization/deserialization failure.
///
/// The IR core does not prescribe a serialization format. JSON, binary IR,
/// replay formats, OpenQASM, or other encodings can translate their errors
/// into this representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrSerializationError {
    /// Input could not be decoded.
    Decode {
        format: &'static str,
        reason: String,
    },

    /// IR could not be encoded.
    Encode {
        format: &'static str,
        reason: String,
    },

    /// Required serialized field is missing.
    MissingField {
        format: &'static str,
        field: &'static str,
    },

    /// Serialized field has an invalid value.
    InvalidField {
        format: &'static str,
        field: &'static str,
        reason: &'static str,
    },

    /// Serialized representation is too large.
    TooLarge {
        format: &'static str,
        bytes: usize,
        maximum: usize,
    },
}

impl fmt::Display for IrSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode { format, reason } => {
                write!(
                    f,
                    "failed to decode {format} IR: {reason}"
                )
            }

            Self::Encode { format, reason } => {
                write!(
                    f,
                    "failed to encode {format} IR: {reason}"
                )
            }

            Self::MissingField { format, field } => {
                write!(
                    f,
                    "{format} IR is missing required field `{field}`"
                )
            }

            Self::InvalidField {
                format,
                field,
                reason,
            } => {
                write!(
                    f,
                    "{format} IR field `{field}` is invalid: {reason}"
                )
            }

            Self::TooLarge {
                format,
                bytes,
                maximum,
            } => {
                write!(
                    f,
                    "{format} IR is too large: {bytes} bytes, maximum {maximum}"
                )
            }
        }
    }
}

impl Error for IrSerializationError {}

// -----------------------------------------------------------------------------
// Version errors
// -----------------------------------------------------------------------------

/// IR schema/version compatibility failure.
///
/// Version handling remains independent from `identity.rs`. The identity module
/// can later provide strongly typed versions and convert them into this stable
/// diagnostic representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrVersionError {
    /// Serialized or incoming IR uses an unsupported version.
    Unsupported {
        found: String,
        supported: &'static str,
    },

    /// A required IR version is older/newer than permitted.
    Incompatible {
        required: String,
        found: String,
    },

    /// Version string cannot be parsed.
    Invalid {
        value: String,
    },
}

impl fmt::Display for IrVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { found, supported } => write!(
                f,
                "unsupported IR version `{found}`; supported versions: {supported}"
            ),

            Self::Incompatible { required, found } => write!(
                f,
                "incompatible IR version: required `{required}`, found `{found}`"
            ),

            Self::Invalid { value } => {
                write!(f, "invalid IR version `{value}`")
            }
        }
    }
}

impl Error for IrVersionError {}

// -----------------------------------------------------------------------------
// Unsupported operation
// -----------------------------------------------------------------------------

/// An operation that is outside the current IR contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrUnsupportedError {
    /// Stable operation/category name.
    pub operation: &'static str,
}

impl IrUnsupportedError {
    /// Creates an unsupported-operation error.
    pub const fn new(
        operation: &'static str,
    ) -> Self {
        Self { operation }
    }
}

impl fmt::Display for IrUnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unsupported Quantum IR operation: {}",
            self.operation
        )
    }
}

impl Error for IrUnsupportedError {}

// -----------------------------------------------------------------------------
// Invariant errors
// -----------------------------------------------------------------------------

/// Internal invariant violation.
///
/// This variant is intended for impossible states detected inside trusted
/// implementation code. It must not be used as a substitute for normal
/// validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrInvariantError {
    /// Name of the violated invariant.
    pub invariant: &'static str,
}

impl IrInvariantError {
    /// Creates an invariant error.
    pub const fn new(
        invariant: &'static str,
    ) -> Self {
        Self { invariant }
    }
}

impl fmt::Display for IrInvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "internal IR invariant violated: {}",
            self.invariant
        )
    }
}

impl Error for IrInvariantError {}

// -----------------------------------------------------------------------------
// Canonical IR error
// -----------------------------------------------------------------------------

/// Canonical top-level error returned by the Quantum IR.
///
/// This is the error type that should cross the public IR boundary.
///
/// Individual modules may maintain specialized errors for local ergonomics,
/// but they should convert those errors into this type when crossing module or
/// subsystem boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrError {
    /// Resource limit violation.
    Limits(IrLimitError),

    /// Identifier failure.
    Identifier(IrIdentifierError),

    /// Qubit failure.
    Qubit(IrQubitError),

    /// Measurement failure.
    Measurement(IrMeasurementError),

    /// Gate failure.
    Gate(IrGateError),

    /// Circuit failure.
    Circuit(IrCircuitError),

    /// Parameter failure.
    Parameter(IrParameterError),

    /// Whole-IR validation failure.
    Validation(IrValidationError),

    /// Generic structural failure.
    InvalidStructure(IrStructureError),

    /// Serialization/deserialization failure.
    Serialization(IrSerializationError),

    /// Version/schema compatibility failure.
    Version(IrVersionError),

    /// Unsupported operation.
    Unsupported(IrUnsupportedError),

    /// Internal invariant violation.
    Invariant(IrInvariantError),
}

impl IrError {
    /// Returns the stable high-level error category.
    pub const fn kind(&self) -> IrErrorKind {
        match self {
            Self::Limits(_) => IrErrorKind::Limits,
            Self::Identifier(_) => IrErrorKind::Identifier,
            Self::Qubit(_) => IrErrorKind::Qubit,
            Self::Measurement(_) => IrErrorKind::Measurement,
            Self::Gate(_) => IrErrorKind::Gate,
            Self::Circuit(_) => IrErrorKind::Circuit,
            Self::Parameter(_) => IrErrorKind::Parameter,
            Self::Validation(_) => IrErrorKind::Validation,
            Self::InvalidStructure(_) => {
                IrErrorKind::InvalidStructure
            }
            Self::Serialization(_) => {
                IrErrorKind::Serialization
            }
            Self::Version(_) => IrErrorKind::Version,
            Self::Unsupported(_) => IrErrorKind::Unsupported,
            Self::Invariant(_) => IrErrorKind::Invariant,
        }
    }

    /// Returns true when this is a resource-limit failure.
    pub const fn is_limit_error(&self) -> bool {
        matches!(self, Self::Limits(_))
    }

    /// Returns true when this is a validation failure.
    pub const fn is_validation_error(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Returns true when this is a serialization failure.
    pub const fn is_serialization_error(&self) -> bool {
        matches!(self, Self::Serialization(_))
    }

    /// Returns true when this is a version compatibility failure.
    pub const fn is_version_error(&self) -> bool {
        matches!(self, Self::Version(_))
    }

    /// Returns true when this represents an internal invariant violation.
    pub const fn is_invariant_error(&self) -> bool {
        matches!(self, Self::Invariant(_))
    }
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limits(error) => {
                write!(f, "IR limits: {error}")
            }

            Self::Identifier(error) => {
                write!(f, "IR identifier: {error}")
            }

            Self::Qubit(error) => {
                write!(f, "IR qubit: {error}")
            }

            Self::Measurement(error) => {
                write!(f, "IR measurement: {error}")
            }

            Self::Gate(error) => {
                write!(f, "IR gate: {error}")
            }

            Self::Circuit(error) => {
                write!(f, "IR circuit: {error}")
            }

            Self::Parameter(error) => {
                write!(f, "IR parameter: {error}")
            }

            Self::Validation(error) => {
                write!(f, "IR validation: {error}")
            }

            Self::InvalidStructure(error) => {
                write!(f, "IR structure: {error}")
            }

            Self::Serialization(error) => {
                write!(f, "IR serialization: {error}")
            }

            Self::Version(error) => {
                write!(f, "IR version: {error}")
            }

            Self::Unsupported(error) => {
                write!(f, "IR unsupported: {error}")
            }

            Self::Invariant(error) => {
                write!(f, "IR invariant: {error}")
            }
        }
    }
}

impl Error for IrError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Limits(error) => Some(error),
            Self::Identifier(error) => Some(error),
            Self::Qubit(error) => Some(error),
            Self::Measurement(error) => Some(error),
            Self::Gate(error) => Some(error),
            Self::Circuit(error) => Some(error),
            Self::Parameter(error) => Some(error),
            Self::Validation(error) => Some(error),
            Self::InvalidStructure(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::Version(error) => Some(error),
            Self::Unsupported(error) => Some(error),
            Self::Invariant(error) => Some(error),
        }
    }
}

// -----------------------------------------------------------------------------
// Constructors
// -----------------------------------------------------------------------------

impl From<IrLimitError> for IrError {
    fn from(error: IrLimitError) -> Self {
        Self::Limits(error)
    }
}

impl From<IrIdentifierError> for IrError {
    fn from(error: IrIdentifierError) -> Self {
        Self::Identifier(error)
    }
}

impl From<IrQubitError> for IrError {
    fn from(error: IrQubitError) -> Self {
        Self::Qubit(error)
    }
}

impl From<IrMeasurementError> for IrError {
    fn from(error: IrMeasurementError) -> Self {
        Self::Measurement(error)
    }
}

impl From<IrGateError> for IrError {
    fn from(error: IrGateError) -> Self {
        Self::Gate(error)
    }
}

impl From<IrCircuitError> for IrError {
    fn from(error: IrCircuitError) -> Self {
        Self::Circuit(error)
    }
}

impl From<IrParameterError> for IrError {
    fn from(error: IrParameterError) -> Self {
        Self::Parameter(error)
    }
}

impl From<IrValidationError> for IrError {
    fn from(error: IrValidationError) -> Self {
        Self::Validation(error)
    }
}

impl From<IrStructureError> for IrError {
    fn from(error: IrStructureError) -> Self {
        Self::InvalidStructure(error)
    }
}

impl From<IrSerializationError> for IrError {
    fn from(error: IrSerializationError) -> Self {
        Self::Serialization(error)
    }
}

impl From<IrVersionError> for IrError {
    fn from(error: IrVersionError) -> Self {
        Self::Version(error)
    }
}

impl From<IrUnsupportedError> for IrError {
    fn from(error: IrUnsupportedError) -> Self {
        Self::Unsupported(error)
    }
}

impl From<IrInvariantError> for IrError {
    fn from(error: IrInvariantError) -> Self {
        Self::Invariant(error)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_error_is_structured() {
        let error = IrError::from(
            IrLimitError::new(
                "max_qubits",
                17,
                16,
            ),
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Limits
        );

        assert!(error.is_limit_error());

        assert_eq!(
            error.to_string(),
            "IR limits: IR limit `max_qubits` exceeded: actual=17, maximum=16"
        );
    }

    #[test]
    fn gate_error_has_stable_category() {
        let error = IrError::from(
            IrGateError::InvalidQubitCount {
                gate: "cx",
                expected: 2,
                actual: 1,
            },
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Gate
        );

        assert!(!error.is_validation_error());
    }

    #[test]
    fn parameter_non_finite_is_structured() {
        let error = IrError::from(
            IrParameterError::NonFinite,
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Parameter
        );

        assert_eq!(
            error.to_string(),
            "IR parameter: parameter must be finite"
        );
    }

    #[test]
    fn validation_error_is_classifiable() {
        let error = IrError::from(
            IrValidationError::Structural {
                operation: Some(3),
                reason: "invalid operand",
            },
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Validation
        );

        assert!(error.is_validation_error());
    }

    #[test]
    fn version_error_is_classifiable() {
        let error = IrError::from(
            IrVersionError::Unsupported {
                found: "2.0".to_owned(),
                supported: "1.x",
            },
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Version
        );

        assert!(error.is_version_error());
    }

    #[test]
    fn serialization_error_is_classifiable() {
        let error = IrError::from(
            IrSerializationError::MissingField {
                format: "binary",
                field: "version",
            },
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Serialization
        );

        assert!(error.is_serialization_error());
    }

    #[test]
    fn invariant_error_is_classifiable() {
        let error = IrError::from(
            IrInvariantError::new(
                "validated circuit contains an invalid operation",
            ),
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Invariant
        );

        assert!(error.is_invariant_error());
    }

    #[test]
    fn source_is_available() {
        let error = IrError::from(
            IrGateError::InvalidParameter,
        );

        assert!(error.source().is_some());
    }

    #[test]
    fn identifier_errors_preserve_values() {
        let error = IrError::from(
            IrIdentifierError::QubitOutOfRange {
                index: 8,
                count: 8,
            },
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Identifier
        );

        assert_eq!(
            error.to_string(),
            "IR identifier: logical qubit index 8 is outside range 0..8"
        );
    }

    #[test]
    fn unsupported_errors_are_structured() {
        let error = IrError::from(
            IrUnsupportedError::new(
                "dynamic hardware pulse",
            ),
        );

        assert_eq!(
            error.kind(),
            IrErrorKind::Unsupported
        );
    }
}