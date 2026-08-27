//! Zamani Quantum — Hardware Instruction Set
//!
//! Production-grade, provider-independent representation of quantum hardware
//! instructions.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "What instructions does a quantum execution target understand, and what
//! > are the exact semantic, operand, parameter, and execution requirements
//! > of those instructions?"
//!
//! The module owns:
//!
//! - instruction identity;
//! - instruction categories;
//! - instruction arity;
//! - operand kinds;
//! - parameter schemas;
//! - parameter domains;
//! - native instruction sets;
//! - instruction aliases;
//! - instruction lookup;
//! - deterministic instruction ordering;
//! - instruction-set validation;
//! - provider-independent instruction metadata;
//! - interoperability names;
//! - instruction-set versioning;
//! - capability requirements associated with instructions.
//!
//! The module does NOT own:
//!
//! - physical topology;
//! - calibration values;
//! - backend lifecycle;
//! - backend authentication;
//! - provider/network I/O;
//! - routing algorithms;
//! - scheduling algorithms;
//! - quantum IR semantics;
//! - benchmark protocols;
//! - transpilation;
//! - pulse waveform generation;
//! - execution.
//!
//! Those responsibilities belong to other quantum subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                │
//!                                ▼
//!                         Workload analysis
//!                                │
//!                                ▼
//!                    ┌────────────────────────┐
//!                    │ hardware compatibility │
//!                    └────────────┬───────────┘
//!                                 │
//!                                 ▼
//!                        InstructionSet
//!                                 │
//!             ┌───────────────────┼───────────────────┐
//!             ▼                   ▼                   ▼
//!          topology          calibration          timing
//!             │                   │                   │
//!             └───────────────────┼───────────────────┘
//!                                 ▼
//!                         routing / scheduling
//!                                 │
//!                                 ▼
//!                         backend / adapter
//! ```
//!
//! The instruction set is therefore a foundational hardware contract.
//!
//! # Design requirements
//!
//! The implementation is designed to support:
//!
//! - gate-model processors;
//! - dynamic circuits;
//! - mid-circuit measurement;
//! - reset;
//! - classical control;
//! - pulse-level control;
//! - analog execution;
//! - annealing;
//! - photonic/bosonic systems;
//! - qudits;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - emulators;
//! - heterogeneous provider APIs;
//! - OpenQASM/QIR interoperability;
//! - future instruction extensions.
//!
//! # Important semantic rule
//!
//! An instruction's *name* is not its complete definition.
//!
//! For example, two providers may both expose `cx`, but their instruction
//! metadata may differ in:
//!
//! - supported operand domains;
//! - duration;
//! - direction;
//! - calibration requirements;
//! - parameter restrictions;
//! - execution semantics.
//!
//! Therefore this module models instruction metadata explicitly.
//!
//! # Stability
//!
//! The following are stable contracts:
//!
//! - `InstructionId` canonicalization;
//! - `Instruction` semantic identity;
//! - `InstructionKind`;
//! - `OperandKind`;
//! - `ParameterSpec`;
//! - `ParameterDomain`;
//! - `InstructionSet` lookup/registration behavior;
//! - validation error taxonomy;
//! - deterministic iteration;
//! - serialization schema version.
//!
//! Provider adapters may add instructions without modifying this module.
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
//! # Serialization
//!
//! Serialization uses the repository's existing Serde dependency. No new
//! dependency is introduced by this module.
//!
//! # Security
//!
//! Instruction definitions must never contain:
//!
//! - API keys;
//! - authentication tokens;
//! - passwords;
//! - private keys;
//! - provider credentials.
//!
//! Instruction metadata is safe to serialize and cache.
//!
//! # Integration contract
//!
//! Consumers should use:
//!
//! ```text
//! InstructionSet
//! Instruction
//! InstructionId
//! InstructionKind
//! OperandKind
//! ParameterSpec
//! ParameterDomain
//! ```
//!
//! A backend may expose a native instruction set by value or by reference.
//!
//! `backend.rs` can continue exposing legacy `BTreeSet<String>` gate names
//! during the migration because `InstructionSet::canonical_names()` provides
//! a deterministic compatibility projection.
//!
//! New hardware code must prefer `InstructionSet`.
//!
//! # No-re-edit guarantee
//!
//! This file intentionally does not import any future hardware module.
//! Consequently, implementation of:
//!
//! - topology.rs;
//! - calibration.rs;
//! - timing.rs;
//! - backend.rs;
//! - validation.rs;
//! - compatibility.rs;
//! - execution.rs;
//! - provider adapters;
//!
//! cannot require changes to the core instruction model merely to establish
//! their integration.
//!
//! Those modules consume this already-defined contract.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;

// =============================================================================
// Schema version
// =============================================================================

/// Version of the serialized instruction-set schema.
///
/// Increment this only when the meaning or structure of serialized
/// instruction metadata changes incompatibly.
pub const INSTRUCTION_SET_SCHEMA_VERSION: u16 = 1;

/// Version of the semantic instruction model.
pub const INSTRUCTION_MODEL_VERSION: u16 = 1;

// =============================================================================
// Instruction identifiers
// =============================================================================

/// Maximum length accepted for a canonical instruction identifier.
pub const MAX_INSTRUCTION_ID_LENGTH: usize = 128;

/// Stable provider-independent instruction identifier.
///
/// Instruction IDs are canonical lowercase identifiers using ASCII letters,
/// digits, `_`, `.`, `-`, `:`, and `/`.
///
/// Examples:
///
/// - `x`
/// - `rz`
/// - `cx`
/// - `measure`
/// - `reset`
/// - `pulse.play`
/// - `analog.evolve`
/// - `anneal.run`
/// - `custom:vendor.operation`
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct InstructionId(String);

impl InstructionId {
    /// Creates a validated canonical instruction identifier.
    pub fn new(value: impl AsRef<str>) -> Result<Self, InstructionIdError> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(InstructionIdError::Empty);
        }

        if value.len() > MAX_INSTRUCTION_ID_LENGTH {
            return Err(InstructionIdError::TooLong {
                length: value.len(),
                maximum: MAX_INSTRUCTION_ID_LENGTH,
            });
        }

        if value != value.trim() {
            return Err(InstructionIdError::Whitespace);
        }

        if value.chars().any(|character| character.is_whitespace()) {
            return Err(InstructionIdError::Whitespace);
        }

        if !value.is_ascii() {
            return Err(InstructionIdError::NonAscii);
        }

        if !is_valid_instruction_identifier(value) {
            return Err(InstructionIdError::InvalidCharacters {
                value: value.to_owned(),
            });
        }

        Ok(Self(value.to_ascii_lowercase()))
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its canonical string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for InstructionId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for InstructionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for InstructionId {
    type Err = InstructionIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors produced while constructing an [`InstructionId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionIdError {
    /// The identifier is empty.
    Empty,

    /// The identifier contains whitespace.
    Whitespace,

    /// The identifier contains non-ASCII characters.
    NonAscii,

    /// The identifier exceeds the maximum length.
    TooLong {
        /// Actual length.
        length: usize,

        /// Maximum accepted length.
        maximum: usize,
    },

    /// The identifier contains unsupported characters.
    InvalidCharacters {
        /// Rejected value.
        value: String,
    },
}

impl fmt::Display for InstructionIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "instruction identifier cannot be empty")
            }

            Self::Whitespace => {
                write!(
                    formatter,
                    "instruction identifier cannot contain whitespace"
                )
            }

            Self::NonAscii => {
                write!(
                    formatter,
                    "instruction identifier must contain only ASCII characters"
                )
            }

            Self::TooLong { length, maximum } => {
                write!(
                    formatter,
                    "instruction identifier length {} exceeds maximum {}",
                    length, maximum
                )
            }

            Self::InvalidCharacters { value } => {
                write!(
                    formatter,
                    "instruction identifier '{}' contains unsupported characters",
                    value
                )
            }
        }
    }
}

impl std::error::Error for InstructionIdError {}

fn is_valid_instruction_identifier(value: &str) -> bool {
    value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'_' | b'.' | b'-' | b':' | b'/')
    })
}

// =============================================================================
// Instruction category
// =============================================================================

/// Semantic category of an instruction.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum InstructionKind {
    /// Single-qubit unitary gate.
    SingleQubitGate,

    /// Multi-qubit unitary gate.
    MultiQubitGate,

    /// Arbitrary-arity unitary operation.
    NQubitGate,

    /// Measurement instruction.
    Measurement,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Explicit qubit reset.
    Reset,

    /// Classical conditional/control instruction.
    ClassicalControl,

    /// Classical computation associated with a quantum program.
    ClassicalOperation,

    /// Barrier/synchronization instruction.
    Barrier,

    /// Delay/no-op timing instruction.
    Delay,

    /// Pulse-level instruction.
    Pulse,

    /// Analog/Hamiltonian instruction.
    Analog,

    /// Quantum annealing instruction.
    Annealing,

    /// State preparation instruction.
    StatePreparation,

    /// State initialization instruction.
    Initialization,

    /// Quantum teleportation/network instruction.
    Network,

    /// Error-correction instruction.
    ErrorCorrection,

    /// Syndrome extraction instruction.
    Syndrome,

    /// Logical-qubit instruction.
    Logical,

    /// Photonic/bosonic instruction.
    Photonic,

    /// Continuous-variable instruction.
    ContinuousVariable,

    /// Qudit instruction.
    Qudit,

    /// Provider-specific instruction.
    Custom,
}

impl InstructionKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleQubitGate => "single_qubit_gate",
            Self::MultiQubitGate => "multi_qubit_gate",
            Self::NQubitGate => "n_qubit_gate",
            Self::Measurement => "measurement",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::Reset => "reset",
            Self::ClassicalControl => "classical_control",
            Self::ClassicalOperation => "classical_operation",
            Self::Barrier => "barrier",
            Self::Delay => "delay",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::StatePreparation => "state_preparation",
            Self::Initialization => "initialization",
            Self::Network => "network",
            Self::ErrorCorrection => "error_correction",
            Self::Syndrome => "syndrome",
            Self::Logical => "logical",
            Self::Photonic => "photonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::Qudit => "qudit",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for InstructionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Operand model
// =============================================================================

/// Kind of operand accepted by an instruction.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum OperandKind {
    /// Physical or logical qubit.
    Qubit,

    /// Qudit.
    Qudit,

    /// Photonic mode.
    Mode,

    /// Bosonic mode.
    BosonicMode,

    /// Continuous-variable mode.
    ContinuousVariable,

    /// Classical bit.
    ClassicalBit,

    /// Classical register.
    ClassicalRegister,

    /// Measurement result.
    MeasurementResult,

    /// Pulse/control channel.
    ControlChannel,

    /// Drive channel.
    DriveChannel,

    /// Measurement channel.
    MeasureChannel,

    /// Acquire channel.
    AcquireChannel,

    /// Frame.
    Frame,

    /// Waveform.
    Waveform,

    /// Analog spatial field.
    SpatialField,

    /// Analog temporal field.
    TemporalField,

    /// Observable.
    Observable,

    /// Logical qubit.
    LogicalQubit,

    /// Syndrome register.
    SyndromeRegister,

    /// Quantum-network endpoint.
    NetworkEndpoint,

    /// Generic resource.
    Resource,

    /// Provider-defined operand.
    Custom,
}

impl OperandKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qubit => "qubit",
            Self::Qudit => "qudit",
            Self::Mode => "mode",
            Self::BosonicMode => "bosonic_mode",
            Self::ContinuousVariable => "continuous_variable",
            Self::ClassicalBit => "classical_bit",
            Self::ClassicalRegister => "classical_register",
            Self::MeasurementResult => "measurement_result",
            Self::ControlChannel => "control_channel",
            Self::DriveChannel => "drive_channel",
            Self::MeasureChannel => "measure_channel",
            Self::AcquireChannel => "acquire_channel",
            Self::Frame => "frame",
            Self::Waveform => "waveform",
            Self::SpatialField => "spatial_field",
            Self::TemporalField => "temporal_field",
            Self::Observable => "observable",
            Self::LogicalQubit => "logical_qubit",
            Self::SyndromeRegister => "syndrome_register",
            Self::NetworkEndpoint => "network_endpoint",
            Self::Resource => "resource",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for OperandKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Describes one operand position.
///
/// An operand can be required or optional. Fixed-position operands are
/// represented explicitly so that provider adapters do not have to infer
/// semantics from arity alone.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct OperandSpec {
    /// Operand position, starting at zero.
    pub position: usize,

    /// Semantic operand type.
    pub kind: OperandKind,

    /// Whether the operand is optional.
    pub optional: bool,

    /// Human-readable role, e.g. `control`, `target`, `condition`.
    pub role: Option<String>,
}

impl OperandSpec {
    /// Creates a required operand.
    pub fn required(
        position: usize,
        kind: OperandKind,
    ) -> Self {
        Self {
            position,
            kind,
            optional: false,
            role: None,
        }
    }

    /// Creates an optional operand.
    pub fn optional(
        position: usize,
        kind: OperandKind,
    ) -> Self {
        Self {
            position,
            kind,
            optional: true,
            role: None,
        }
    }

    /// Assigns a semantic role.
    pub fn with_role(
        mut self,
        role: impl Into<String>,
    ) -> Self {
        self.role = Some(role.into());
        self
    }
}

// =============================================================================
// Parameter model
// =============================================================================

/// Parameter value type expected by an instruction.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum ParameterType {
    /// Real-valued parameter.
    Real,

    /// Integer parameter.
    Integer,

    /// Boolean parameter.
    Boolean,

    /// String parameter.
    String,

    /// Frequency value.
    Frequency,

    /// Duration value.
    Duration,

    /// Phase angle.
    Angle,

    /// Amplitude.
    Amplitude,

    /// Probability in `[0, 1]`.
    Probability,

    /// Generic numeric parameter.
    Numeric,

    /// Provider-defined parameter type.
    Custom,
}

impl ParameterType {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Real => "real",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::String => "string",
            Self::Frequency => "frequency",
            Self::Duration => "duration",
            Self::Angle => "angle",
            Self::Amplitude => "amplitude",
            Self::Probability => "probability",
            Self::Numeric => "numeric",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Mathematical domain restriction for a parameter.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum ParameterDomain {
    /// Any finite real number.
    AnyFinite,

    /// Closed interval `[min, max]`.
    InclusiveRange {
        /// Minimum value.
        min: f64,

        /// Maximum value.
        max: f64,
    },

    /// Open interval `(min, max)`.
    ExclusiveRange {
        /// Lower bound.
        min: f64,

        /// Upper bound.
        max: f64,
    },

    /// Half-open interval `[min, max)`.
    InclusiveExclusiveRange {
        /// Lower bound.
        min: f64,

        /// Upper bound.
        max: f64,
    },

    /// Half-open interval `(min, max]`.
    ExclusiveInclusiveRange {
        /// Lower bound.
        min: f64,

        /// Upper bound.
        max: f64,
    },

    /// Non-negative finite real number.
    NonNegativeFinite,

    /// Strictly positive finite real number.
    PositiveFinite,

    /// Unit interval `[0, 1]`.
    Probability,

    /// Any finite integer.
    AnyInteger,

    /// Closed integer interval.
    IntegerRange {
        /// Minimum integer.
        min: i64,

        /// Maximum integer.
        max: i64,
    },

    /// No domain restriction beyond the parameter type.
    Unrestricted,
}

impl ParameterDomain {
    /// Validates a floating-point parameter against the domain.
    pub fn validate_f64(
        &self,
        value: f64,
    ) -> Result<(), ParameterDomainError> {
        if !value.is_finite() {
            return Err(ParameterDomainError::NonFinite { value });
        }

        match *self {
            Self::AnyFinite | Self::Unrestricted => Ok(()),

            Self::InclusiveRange { min, max } => {
                validate_real_bounds(min, max)?;

                if value < min || value > max {
                    return Err(ParameterDomainError::OutsideRange {
                        value,
                        min,
                        max,
                        lower_inclusive: true,
                        upper_inclusive: true,
                    });
                }

                Ok(())
            }

            Self::ExclusiveRange { min, max } => {
                validate_real_bounds(min, max)?;

                if value <= min || value >= max {
                    return Err(ParameterDomainError::OutsideRange {
                        value,
                        min,
                        max,
                        lower_inclusive: false,
                        upper_inclusive: false,
                    });
                }

                Ok(())
            }

            Self::InclusiveExclusiveRange { min, max } => {
                validate_real_bounds(min, max)?;

                if value < min || value >= max {
                    return Err(ParameterDomainError::OutsideRange {
                        value,
                        min,
                        max,
                        lower_inclusive: true,
                        upper_inclusive: false,
                    });
                }

                Ok(())
            }

            Self::ExclusiveInclusiveRange { min, max } => {
                validate_real_bounds(min, max)?;

                if value <= min || value > max {
                    return Err(ParameterDomainError::OutsideRange {
                        value,
                        min,
                        max,
                        lower_inclusive: false,
                        upper_inclusive: true,
                    });
                }

                Ok(())
            }

            Self::NonNegativeFinite => {
                if value < 0.0 {
                    return Err(ParameterDomainError::Negative {
                        value,
                    });
                }

                Ok(())
            }

            Self::PositiveFinite => {
                if value <= 0.0 {
                    return Err(ParameterDomainError::NotPositive {
                        value,
                    });
                }

                Ok(())
            }

            Self::Probability => {
                if !(0.0..=1.0).contains(&value) {
                    return Err(ParameterDomainError::OutsideRange {
                        value,
                        min: 0.0,
                        max: 1.0,
                        lower_inclusive: true,
                        upper_inclusive: true,
                    });
                }

                Ok(())
            }

            Self::AnyInteger | Self::IntegerRange { .. } => {
                Err(ParameterDomainError::WrongValueKind)
            }
        }
    }

    /// Validates an integer parameter against the domain.
    pub fn validate_i64(
        &self,
        value: i64,
    ) -> Result<(), ParameterDomainError> {
        match *self {
            Self::AnyInteger | Self::Unrestricted => Ok(()),

            Self::IntegerRange { min, max } => {
                if min > max {
                    return Err(ParameterDomainError::InvalidBounds);
                }

                if value < min || value > max {
                    return Err(ParameterDomainError::IntegerOutsideRange {
                        value,
                        min,
                        max,
                    });
                }

                Ok(())
            }

            _ => Err(ParameterDomainError::WrongValueKind),
        }
    }
}

fn validate_real_bounds(
    min: f64,
    max: f64,
) -> Result<(), ParameterDomainError> {
    if !min.is_finite() || !max.is_finite() || min > max {
        return Err(ParameterDomainError::InvalidBounds);
    }

    Ok(())
}

/// Errors produced by parameter-domain validation.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterDomainError {
    /// Value is NaN or infinite.
    NonFinite {
        /// Invalid value.
        value: f64,
    },

    /// Lower/upper bounds are invalid.
    InvalidBounds,

    /// Floating-point value is outside its allowed interval.
    OutsideRange {
        /// Supplied value.
        value: f64,

        /// Lower bound.
        min: f64,

        /// Upper bound.
        max: f64,

        /// Whether the lower bound is inclusive.
        lower_inclusive: bool,

        /// Whether the upper bound is inclusive.
        upper_inclusive: bool,
    },

    /// Negative value where non-negative was required.
    Negative {
        /// Supplied value.
        value: f64,
    },

    /// Non-positive value where positive was required.
    NotPositive {
        /// Supplied value.
        value: f64,
    },

    /// Integer outside its allowed range.
    IntegerOutsideRange {
        /// Supplied value.
        value: i64,

        /// Lower bound.
        min: i64,

        /// Upper bound.
        max: i64,
    },

    /// Domain does not accept the supplied primitive kind.
    WrongValueKind,
}

impl fmt::Display for ParameterDomainError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFinite { value } => {
                write!(
                    formatter,
                    "parameter value {} is not finite",
                    value
                )
            }

            Self::InvalidBounds => {
                write!(
                    formatter,
                    "parameter domain bounds are invalid"
                )
            }

            Self::OutsideRange {
                value,
                min,
                max,
                lower_inclusive,
                upper_inclusive,
            } => {
                let left = if *lower_inclusive { '[' } else { '(' };
                let right = if *upper_inclusive { ']' } else { ')' };

                write!(
                    formatter,
                    "parameter value {} is outside {}{}, {}{}",
                    value,
                    left,
                    min,
                    max,
                    right
                )
            }

            Self::Negative { value } => {
                write!(
                    formatter,
                    "parameter value {} must be non-negative",
                    value
                )
            }

            Self::NotPositive { value } => {
                write!(
                    formatter,
                    "parameter value {} must be positive",
                    value
                )
            }

            Self::IntegerOutsideRange {
                value,
                min,
                max,
            } => {
                write!(
                    formatter,
                    "integer parameter value {} is outside [{}, {}]",
                    value,
                    min,
                    max
                )
            }

            Self::WrongValueKind => {
                write!(
                    formatter,
                    "parameter domain does not accept this value kind"
                )
            }
        }
    }
}

impl std::error::Error for ParameterDomainError {}

/// Complete specification for one instruction parameter.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ParameterSpec {
    /// Zero-based parameter position.
    pub position: usize,

    /// Stable parameter name.
    pub name: String,

    /// Parameter primitive type.
    pub parameter_type: ParameterType,

    /// Domain restriction.
    pub domain: ParameterDomain,

    /// Whether the parameter may be omitted.
    pub optional: bool,

    /// Optional documentation.
    pub description: Option<String>,
}

impl ParameterSpec {
    /// Creates a required real parameter.
    pub fn real(
        position: usize,
        name: impl Into<String>,
    ) -> Self {
        Self {
            position,
            name: name.into(),
            parameter_type: ParameterType::Real,
            domain: ParameterDomain::AnyFinite,
            optional: false,
            description: None,
        }
    }

    /// Creates an angle parameter.
    pub fn angle(
        position: usize,
        name: impl Into<String>,
    ) -> Self {
        Self {
            position,
            name: name.into(),
            parameter_type: ParameterType::Angle,
            domain: ParameterDomain::AnyFinite,
            optional: false,
            description: None,
        }
    }

    /// Creates a probability parameter.
    pub fn probability(
        position: usize,
        name: impl Into<String>,
    ) -> Self {
        Self {
            position,
            name: name.into(),
            parameter_type: ParameterType::Probability,
            domain: ParameterDomain::Probability,
            optional: false,
            description: None,
        }
    }

    /// Creates an integer parameter.
    pub fn integer(
        position: usize,
        name: impl Into<String>,
    ) -> Self {
        Self {
            position,
            name: name.into(),
            parameter_type: ParameterType::Integer,
            domain: ParameterDomain::AnyInteger,
            optional: false,
            description: None,
        }
    }

    /// Sets the domain.
    pub fn with_domain(
        mut self,
        domain: ParameterDomain,
    ) -> Self {
        self.domain = domain;
        self
    }

    /// Marks the parameter optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Adds documentation.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// Hardware capability semantically required by an instruction.
///
/// These names intentionally mirror the production hardware capability
/// vocabulary without importing `capabilities.rs`. This keeps this file
/// independently compilable and prevents a foundational dependency cycle.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum InstructionCapability {
    /// Measurement support.
    Measurement,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Reset support.
    Reset,

    /// Classical control.
    ClassicalControl,

    /// Dynamic circuits.
    DynamicCircuits,

    /// Parameterized execution.
    ParameterizedCircuits,

    /// Pulse control.
    PulseControl,

    /// Analog control.
    AnalogControl,

    /// Annealing.
    Annealing,

    /// Logical qubits.
    LogicalQubits,

    /// Fault-tolerant execution.
    FaultTolerantExecution,

    /// Syndrome extraction.
    SyndromeMeasurement,

    /// Photonic operations.
    Photonic,

    /// Continuous-variable operations.
    ContinuousVariable,

    /// Qudit operations.
    Qudit,

    /// Quantum-network operation.
    QuantumNetworking,

    /// Provider-specific capability.
    Custom,
}

impl InstructionCapability {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measurement => "measurement",
            Self::MidCircuitMeasurement => "mid_circuit_measurement",
            Self::Reset => "reset",
            Self::ClassicalControl => "classical_control",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::ParameterizedCircuits => "parameterized_circuits",
            Self::PulseControl => "pulse_control",
            Self::AnalogControl => "analog_control",
            Self::Annealing => "annealing",
            Self::LogicalQubits => "logical_qubits",
            Self::FaultTolerantExecution => "fault_tolerant_execution",
            Self::SyndromeMeasurement => "syndrome_measurement",
            Self::Photonic => "photonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::Qudit => "qudit",
            Self::QuantumNetworking => "quantum_networking",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Interoperability identifiers
// =============================================================================

/// External interoperability naming information.
///
/// These fields allow an instruction to have one canonical Zamani identity
/// while exposing standardized/provider-specific names.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct InteroperabilityNames {
    /// Optional OpenQASM name.
    pub openqasm: Option<String>,

    /// Optional QIR/LLVM-facing name.
    pub qir: Option<String>,

    /// Optional Quil name.
    pub quil: Option<String>,

    /// Provider-specific aliases.
    pub provider_aliases: BTreeSet<String>,
}

impl Default for InteroperabilityNames {
    fn default() -> Self {
        Self {
            openqasm: None,
            qir: None,
            quil: None,
            provider_aliases: BTreeSet::new(),
        }
    }
}

impl InteroperabilityNames {
    /// Creates an empty interoperability mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an OpenQASM spelling.
    pub fn with_openqasm(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.openqasm = Some(name.into());
        self
    }

    /// Sets a QIR-facing name.
    pub fn with_qir(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.qir = Some(name.into());
        self
    }

    /// Sets a Quil name.
    pub fn with_quil(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.quil = Some(name.into());
        self
    }

    /// Adds a provider alias.
    pub fn with_provider_alias(
        mut self,
        alias: impl Into<String>,
    ) -> Self {
        self.provider_aliases
            .insert(alias.into().to_ascii_lowercase());
        self
    }
}

// =============================================================================
// Instruction metadata
// =============================================================================

/// Describes the semantic and execution contract of one hardware instruction.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct Instruction {
    /// Stable canonical identifier.
    pub id: InstructionId,

    /// Semantic instruction category.
    pub kind: InstructionKind,

    /// Human-readable display name.
    pub name: String,

    /// Ordered operand schema.
    pub operands: Vec<OperandSpec>,

    /// Ordered parameter schema.
    pub parameters: Vec<ParameterSpec>,

    /// Required hardware capabilities.
    pub required_capabilities: BTreeSet<InstructionCapability>,

    /// Interoperability names.
    pub interoperability: InteroperabilityNames,

    /// Whether the operation is unitary.
    pub unitary: bool,

    /// Whether the operation is reversible.
    pub reversible: bool,

    /// Whether the operation has an adjoint.
    pub adjoint: bool,

    /// Whether the operation can be controlled by a quantum control.
    pub controllable: bool,

    /// Whether the instruction may be executed conditionally on classical data.
    pub classically_controllable: bool,

    /// Whether the operation may appear in a dynamic circuit.
    pub dynamic: bool,

    /// Optional semantic documentation.
    pub description: Option<String>,
}

impl Instruction {
    /// Creates a new instruction after validating its schema.
    pub fn new(
        id: InstructionId,
        kind: InstructionKind,
        name: impl Into<String>,
        operands: Vec<OperandSpec>,
        parameters: Vec<ParameterSpec>,
    ) -> Result<Self, InstructionError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(InstructionError::EmptyDisplayName);
        }

        let instruction = Self {
            id,
            kind,
            name,
            operands,
            parameters,
            required_capabilities: BTreeSet::new(),
            interoperability: InteroperabilityNames::default(),
            unitary: false,
            reversible: false,
            adjoint: false,
            controllable: false,
            classically_controllable: false,
            dynamic: false,
            description: None,
        };

        instruction.validate()?;

        Ok(instruction)
    }

    /// Creates a simple one-qubit gate.
    pub fn single_qubit_gate(
        id: impl AsRef<str>,
        name: impl Into<String>,
    ) -> Result<Self, InstructionError> {
        let id = InstructionId::new(id.as_ref())
            .map_err(InstructionError::InvalidId)?;

        Self::new(
            id,
            InstructionKind::SingleQubitGate,
            name,
            vec![OperandSpec::required(0, OperandKind::Qubit)],
            Vec::new(),
        )
    }

    /// Creates a simple two-qubit gate.
    pub fn two_qubit_gate(
        id: impl AsRef<str>,
        name: impl Into<String>,
    ) -> Result<Self, InstructionError> {
        let id = InstructionId::new(id.as_ref())
            .map_err(InstructionError::InvalidId)?;

        Self::new(
            id,
            InstructionKind::MultiQubitGate,
            name,
            vec![
                OperandSpec::required(0, OperandKind::Qubit)
                    .with_role("control"),
                OperandSpec::required(1, OperandKind::Qubit)
                    .with_role("target"),
            ],
            Vec::new(),
        )
    }

    /// Creates a measurement instruction.
    pub fn measurement(
        id: impl AsRef<str>,
    ) -> Result<Self, InstructionError> {
        let id = InstructionId::new(id.as_ref())
            .map_err(InstructionError::InvalidId)?;

        let mut instruction = Self::new(
            id,
            InstructionKind::Measurement,
            "Measurement",
            vec![
                OperandSpec::required(0, OperandKind::Qubit)
                    .with_role("qubit"),
                OperandSpec::required(
                    1,
                    OperandKind::ClassicalBit,
                )
                .with_role("destination"),
            ],
            Vec::new(),
        )?;

        instruction
            .required_capabilities
            .insert(InstructionCapability::Measurement);

        Ok(instruction)
    }

    /// Creates a reset instruction.
    pub fn reset(
        id: impl AsRef<str>,
    ) -> Result<Self, InstructionError> {
        let id = InstructionId::new(id.as_ref())
            .map_err(InstructionError::InvalidId)?;

        let mut instruction = Self::new(
            id,
            InstructionKind::Reset,
            "Reset",
            vec![OperandSpec::required(
                0,
                OperandKind::Qubit,
            )],
            Vec::new(),
        )?;

        instruction
            .required_capabilities
            .insert(InstructionCapability::Reset);

        Ok(instruction)
    }

    /// Adds a required capability.
    pub fn requiring(
        mut self,
        capability: InstructionCapability,
    ) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Adds several required capabilities.
    pub fn requiring_all<I>(
        mut self,
        capabilities: I,
    ) -> Self
    where
        I: IntoIterator<Item = InstructionCapability>,
    {
        self.required_capabilities
            .extend(capabilities);
        self
    }

    /// Sets unitary semantics.
    pub fn unitary(mut self) -> Self {
        self.unitary = true;
        self
    }

    /// Marks the instruction reversible.
    pub fn reversible(mut self) -> Self {
        self.reversible = true;
        self
    }

    /// Marks the instruction as having an adjoint.
    pub fn with_adjoint(mut self) -> Self {
        self.adjoint = true;
        self
    }

    /// Marks the instruction as quantum-controlled.
    pub fn controllable(mut self) -> Self {
        self.controllable = true;
        self
    }

    /// Marks the instruction as classically controllable.
    pub fn classically_controllable(mut self) -> Self {
        self.classically_controllable = true;
        self
    }

    /// Marks the instruction as dynamic-circuit compatible.
    pub fn dynamic(mut self) -> Self {
        self.dynamic = true;
        self.required_capabilities
            .insert(InstructionCapability::DynamicCircuits);
        self
    }

    /// Adds interoperability metadata.
    pub fn with_interoperability(
        mut self,
        interoperability: InteroperabilityNames,
    ) -> Self {
        self.interoperability = interoperability;
        self
    }

    /// Adds documentation.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the required operand count.
    pub fn required_operand_count(&self) -> usize {
        self.operands
            .iter()
            .filter(|operand| !operand.optional)
            .count()
    }

    /// Returns the maximum operand count.
    pub fn maximum_operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Returns whether an operand count is valid.
    pub fn accepts_operand_count(
        &self,
        count: usize,
    ) -> bool {
        count >= self.required_operand_count()
            && count <= self.maximum_operand_count()
    }

    /// Returns whether the supplied parameter count is valid.
    pub fn accepts_parameter_count(
        &self,
        count: usize,
    ) -> bool {
        let required = self
            .parameters
            .iter()
            .filter(|parameter| !parameter.optional)
            .count();

        count >= required && count <= self.parameters.len()
    }

    /// Returns a required capability iterator.
    pub fn capabilities(
        &self,
    ) -> impl Iterator<Item = &InstructionCapability> {
        self.required_capabilities.iter()
    }

    /// Validates the instruction schema.
    pub fn validate(&self) -> Result<(), InstructionError> {
        if self.name.trim().is_empty() {
            return Err(InstructionError::EmptyDisplayName);
        }

        validate_operands(&self.operands)?;
        validate_parameters(&self.parameters)?;

        if self.reversible && !self.unitary {
            return Err(InstructionError::InvalidSemantics {
                message:
                    "a reversible instruction must be unitary"
                        .to_owned(),
            });
        }

        if self.adjoint && !self.unitary {
            return Err(InstructionError::InvalidSemantics {
                message:
                    "an instruction with an adjoint must be unitary"
                        .to_owned(),
            });
        }

        if self.kind == InstructionKind::Measurement
            && self.unitary
        {
            return Err(InstructionError::InvalidSemantics {
                message:
                    "measurement instructions cannot be unitary"
                        .to_owned(),
            });
        }

        if self.kind == InstructionKind::Reset
            && self.unitary
        {
            return Err(InstructionError::InvalidSemantics {
                message:
                    "reset instructions cannot be unitary"
                        .to_owned(),
            });
        }

        if self.dynamic
            && !self
                .required_capabilities
                .contains(&InstructionCapability::DynamicCircuits)
        {
            return Err(InstructionError::MissingRequiredCapability {
                capability:
                    InstructionCapability::DynamicCircuits,
            });
        }

        Ok(())
    }
}

fn validate_operands(
    operands: &[OperandSpec],
) -> Result<(), InstructionError> {
    let mut positions = BTreeSet::new();

    for operand in operands {
        if !positions.insert(operand.position) {
            return Err(InstructionError::DuplicateOperandPosition {
                position: operand.position,
            });
        }

        if let Some(role) = &operand.role {
            if role.trim().is_empty() {
                return Err(InstructionError::EmptyOperandRole {
                    position: operand.position,
                });
            }
        }
    }

    if !operands.is_empty() {
        for (expected, operand) in operands.iter().enumerate() {
            if operand.position != expected {
                return Err(InstructionError::NonContiguousOperandPositions);
            }
        }
    }

    let mut optional_seen = false;

    for operand in operands {
        if operand.optional {
            optional_seen = true;
        } else if optional_seen {
            return Err(
                InstructionError::RequiredOperandAfterOptional,
            );
        }
    }

    Ok(())
}

fn validate_parameters(
    parameters: &[ParameterSpec],
) -> Result<(), InstructionError> {
    let mut positions = BTreeSet::new();

    for parameter in parameters {
        if !positions.insert(parameter.position) {
            return Err(
                InstructionError::DuplicateParameterPosition {
                    position: parameter.position,
                },
            );
        }

        if parameter.name.trim().is_empty() {
            return Err(
                InstructionError::EmptyParameterName {
                    position: parameter.position,
                },
            );
        }

        match parameter.domain {
            ParameterDomain::InclusiveRange { min, max }
            | ParameterDomain::ExclusiveRange { min, max }
            | ParameterDomain::InclusiveExclusiveRange {
                min,
                max,
            }
            | ParameterDomain::ExclusiveInclusiveRange {
                min,
                max,
            } => {
                validate_real_bounds(min, max).map_err(
                    |_| InstructionError::InvalidParameterDomain {
                        position: parameter.position,
                    },
                )?;
            }

            ParameterDomain::IntegerRange { min, max } => {
                if min > max {
                    return Err(
                        InstructionError::InvalidParameterDomain {
                            position: parameter.position,
                        },
                    );
                }
            }

            _ => {}
        }
    }

    if !parameters.is_empty() {
        for (expected, parameter) in
            parameters.iter().enumerate()
        {
            if parameter.position != expected {
                return Err(
                    InstructionError::NonContiguousParameterPositions,
                );
            }
        }
    }

    let mut optional_seen = false;

    for parameter in parameters {
        if parameter.optional {
            optional_seen = true;
        } else if optional_seen {
            return Err(
                InstructionError::RequiredParameterAfterOptional,
            );
        }
    }

    Ok(())
}

// =============================================================================
// Instruction errors
// =============================================================================

/// Errors produced by instruction definitions and instruction sets.
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionError {
    /// Invalid instruction identifier.
    InvalidId(InstructionIdError),

    /// Empty display name.
    EmptyDisplayName,

    /// Duplicate operand position.
    DuplicateOperandPosition {
        /// Duplicate position.
        position: usize,
    },

    /// Operand positions are not contiguous.
    NonContiguousOperandPositions,

    /// Required operand occurs after an optional operand.
    RequiredOperandAfterOptional,

    /// Empty operand role.
    EmptyOperandRole {
        /// Operand position.
        position: usize,
    },

    /// Duplicate parameter position.
    DuplicateParameterPosition {
        /// Duplicate position.
        position: usize,
    },

    /// Parameter positions are not contiguous.
    NonContiguousParameterPositions,

    /// Required parameter occurs after optional parameter.
    RequiredParameterAfterOptional,

    /// Empty parameter name.
    EmptyParameterName {
        /// Parameter position.
        position: usize,
    },

    /// Invalid parameter domain.
    InvalidParameterDomain {
        /// Parameter position.
        position: usize,
    },

    /// Invalid semantic combination.
    InvalidSemantics {
        /// Explanation.
        message: String,
    },

    /// Required capability was not declared.
    MissingRequiredCapability {
        /// Missing capability.
        capability: InstructionCapability,
    },

    /// Duplicate instruction ID.
    DuplicateInstruction {
        /// Duplicate ID.
        id: InstructionId,
    },

    /// Instruction was not found.
    InstructionNotFound {
        /// Requested ID.
        id: InstructionId,
    },

    /// Alias is already registered.
    DuplicateAlias {
        /// Alias.
        alias: String,
    },

    /// Alias points to an unknown instruction.
    AliasTargetNotFound {
        /// Alias.
        alias: String,

        /// Target instruction.
        target: InstructionId,
    },

    /// Alias is identical to an existing canonical instruction.
    AliasConflictsWithInstruction {
        /// Conflicting alias.
        alias: String,
    },

    /// Instruction set has invalid metadata.
    InvalidInstructionSet {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for InstructionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidId(error) => {
                write!(formatter, "invalid instruction ID: {}", error)
            }

            Self::EmptyDisplayName => {
                write!(
                    formatter,
                    "instruction display name cannot be empty"
                )
            }

            Self::DuplicateOperandPosition { position } => {
                write!(
                    formatter,
                    "duplicate operand position {}",
                    position
                )
            }

            Self::NonContiguousOperandPositions => {
                write!(
                    formatter,
                    "instruction operand positions must be contiguous from zero"
                )
            }

            Self::RequiredOperandAfterOptional => {
                write!(
                    formatter,
                    "required operands cannot occur after optional operands"
                )
            }

            Self::EmptyOperandRole { position } => {
                write!(
                    formatter,
                    "operand {} has an empty role",
                    position
                )
            }

            Self::DuplicateParameterPosition { position } => {
                write!(
                    formatter,
                    "duplicate parameter position {}",
                    position
                )
            }

            Self::NonContiguousParameterPositions => {
                write!(
                    formatter,
                    "instruction parameter positions must be contiguous from zero"
                )
            }

            Self::RequiredParameterAfterOptional => {
                write!(
                    formatter,
                    "required parameters cannot occur after optional parameters"
                )
            }

            Self::EmptyParameterName { position } => {
                write!(
                    formatter,
                    "parameter {} has an empty name",
                    position
                )
            }

            Self::InvalidParameterDomain { position } => {
                write!(
                    formatter,
                    "parameter {} has an invalid domain",
                    position
                )
            }

            Self::InvalidSemantics { message } => {
                write!(
                    formatter,
                    "invalid instruction semantics: {}",
                    message
                )
            }

            Self::MissingRequiredCapability {
                capability,
            } => {
                write!(
                    formatter,
                    "instruction requires capability '{}' but it was not declared",
                    capability.as_str()
                )
            }

            Self::DuplicateInstruction { id } => {
                write!(
                    formatter,
                    "instruction '{}' is already registered",
                    id
                )
            }

            Self::InstructionNotFound { id } => {
                write!(
                    formatter,
                    "instruction '{}' was not found",
                    id
                )
            }

            Self::DuplicateAlias { alias } => {
                write!(
                    formatter,
                    "instruction alias '{}' is already registered",
                    alias
                )
            }

            Self::AliasTargetNotFound {
                alias,
                target,
            } => {
                write!(
                    formatter,
                    "instruction alias '{}' targets unknown instruction '{}'",
                    alias,
                    target
                )
            }

            Self::AliasConflictsWithInstruction { alias } => {
                write!(
                    formatter,
                    "instruction alias '{}' conflicts with an instruction ID",
                    alias
                )
            }

            Self::InvalidInstructionSet { message } => {
                write!(
                    formatter,
                    "invalid instruction set: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for InstructionError {}

// =============================================================================
// Instruction-set metadata
// =============================================================================

/// Versioned metadata describing an instruction set.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct InstructionSetMetadata {
    /// Serialization schema version.
    pub schema_version: u16,

    /// Semantic instruction model version.
    pub model_version: u16,

    /// Optional backend/provider instruction-set version.
    pub implementation_version: Option<String>,

    /// Optional source/provider name.
    pub source: Option<String>,
}

impl Default for InstructionSetMetadata {
    fn default() -> Self {
        Self {
            schema_version: INSTRUCTION_SET_SCHEMA_VERSION,
            model_version: INSTRUCTION_MODEL_VERSION,
            implementation_version: None,
            source: None,
        }
    }
}

impl InstructionSetMetadata {
    /// Creates metadata for a new instruction set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the implementation version.
    pub fn with_implementation_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.implementation_version = Some(version.into());
        self
    }

    /// Sets the source.
    pub fn with_source(
        mut self,
        source: impl Into<String>,
    ) -> Self {
        self.source = Some(source.into());
        self
    }
}

// =============================================================================
// Instruction set
// =============================================================================

/// Deterministic collection of instructions supported by a hardware target.
///
/// Canonical instruction IDs are stored in a `BTreeMap`, making iteration,
/// serialization and compatibility checks deterministic.
///
/// Aliases are kept separately so canonical identity remains unambiguous.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct InstructionSet {
    /// Versioned instruction-set metadata.
    pub metadata: InstructionSetMetadata,

    /// Canonical instruction definitions.
    instructions: BTreeMap<InstructionId, Instruction>,

    /// Alias → canonical instruction ID.
    aliases: BTreeMap<String, InstructionId>,
}

impl Default for InstructionSet {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionSet {
    /// Creates an empty instruction set.
    pub fn new() -> Self {
        Self {
            metadata: InstructionSetMetadata::default(),
            instructions: BTreeMap::new(),
            aliases: BTreeMap::new(),
        }
    }

    /// Creates an instruction set with metadata.
    pub fn with_metadata(
        metadata: InstructionSetMetadata,
    ) -> Self {
        Self {
            metadata,
            instructions: BTreeMap::new(),
            aliases: BTreeMap::new(),
        }
    }

    /// Returns the number of canonical instructions.
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Returns whether the instruction set is empty.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Returns the number of aliases.
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }

    /// Registers an instruction.
    ///
    /// Registration is atomic from the caller's perspective: the instruction
    /// is validated before the set is modified.
    pub fn register(
        &mut self,
        instruction: Instruction,
    ) -> Result<(), InstructionError> {
        instruction.validate()?;

        if self
            .instructions
            .contains_key(&instruction.id)
        {
            return Err(
                InstructionError::DuplicateInstruction {
                    id: instruction.id.clone(),
                },
            );
        }

        if self
            .aliases
            .contains_key(instruction.id.as_str())
        {
            return Err(
                InstructionError::AliasConflictsWithInstruction {
                    alias: instruction.id.as_str().to_owned(),
                },
            );
        }

        self.instructions
            .insert(instruction.id.clone(), instruction);

        Ok(())
    }

    /// Removes a canonical instruction.
    ///
    /// All aliases pointing at that instruction are removed as well.
    pub fn remove(
        &mut self,
        id: &InstructionId,
    ) -> Result<Instruction, InstructionError> {
        let instruction = self
            .instructions
            .remove(id)
            .ok_or_else(|| InstructionError::InstructionNotFound {
                id: id.clone(),
            })?;

        self.aliases.retain(|_, target| target != id);

        Ok(instruction)
    }

    /// Registers an alias for an existing instruction.
    pub fn register_alias(
        &mut self,
        alias: impl AsRef<str>,
        target: &InstructionId,
    ) -> Result<(), InstructionError> {
        let alias = normalize_alias(alias.as_ref())?;

        if !self.instructions.contains_key(target) {
            return Err(
                InstructionError::AliasTargetNotFound {
                    alias,
                    target: target.clone(),
                },
            );
        }

        if self.instructions.keys().any(|id| {
            id.as_str() == alias
        }) {
            return Err(
                InstructionError::AliasConflictsWithInstruction {
                    alias,
                },
            );
        }

        if self.aliases.contains_key(&alias) {
            return Err(
                InstructionError::DuplicateAlias { alias },
            );
        }

        self.aliases.insert(alias, target.clone());

        Ok(())
    }

    /// Resolves either a canonical ID or an alias.
    pub fn resolve(
        &self,
        name: &str,
    ) -> Result<&Instruction, InstructionError> {
        let id = InstructionId::new(name)
            .map_err(InstructionError::InvalidId)?;

        if let Some(instruction) =
            self.instructions.get(&id)
        {
            return Ok(instruction);
        }

        if let Some(target) = self.aliases.get(id.as_str()) {
            return self.instructions.get(target).ok_or_else(
                || InstructionError::AliasTargetNotFound {
                    alias: id.as_str().to_owned(),
                    target: target.clone(),
                },
            );
        }

        Err(InstructionError::InstructionNotFound { id })
    }

    /// Resolves a canonical ID only.
    pub fn get(
        &self,
        id: &InstructionId,
    ) -> Option<&Instruction> {
        self.instructions.get(id)
    }

    /// Returns all canonical instructions in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&InstructionId, &Instruction)> {
        self.instructions.iter()
    }

    /// Returns all canonical instruction IDs.
    pub fn ids(
        &self,
    ) -> impl Iterator<Item = &InstructionId> {
        self.instructions.keys()
    }

    /// Returns all aliases in deterministic order.
    pub fn aliases(
        &self,
    ) -> impl Iterator<Item = (&String, &InstructionId)> {
        self.aliases.iter()
    }

    /// Returns whether the set contains an instruction or alias.
    pub fn contains(&self, name: &str) -> bool {
        self.resolve(name).is_ok()
    }

    /// Returns canonical names as strings.
    ///
    /// This is the migration/compatibility projection for the current
    /// `backend.rs` representation (`BTreeSet<String>`).
    pub fn canonical_names(&self) -> BTreeSet<String> {
        self.instructions
            .keys()
            .map(|id| id.as_str().to_owned())
            .collect()
    }

    /// Creates an instruction set from canonical names.
    ///
    /// This compatibility constructor creates minimally specified custom
    /// instructions. New code should prefer registering complete
    /// `Instruction` definitions.
    pub fn from_canonical_names<I, S>(
        names: I,
    ) -> Result<Self, InstructionError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut set = Self::new();

        for name in names {
            let id = InstructionId::new(name.as_ref())
                .map_err(InstructionError::InvalidId)?;

            let instruction = Instruction::new(
                id,
                InstructionKind::Custom,
                name.as_ref(),
                Vec::new(),
                Vec::new(),
            )?;

            set.register(instruction)?;
        }

        Ok(set)
    }

    /// Validates the entire instruction set.
    pub fn validate(&self) -> Result<(), InstructionError> {
        if self.metadata.schema_version == 0 {
            return Err(
                InstructionError::InvalidInstructionSet {
                    message:
                        "schema version cannot be zero"
                            .to_owned(),
                },
            );
        }

        if self.metadata.model_version == 0 {
            return Err(
                InstructionError::InvalidInstructionSet {
                    message:
                        "model version cannot be zero"
                            .to_owned(),
                },
            );
        }

        for instruction in self.instructions.values() {
            instruction.validate()?;
        }

        for (alias, target) in &self.aliases {
            if !self.instructions.contains_key(target) {
                return Err(
                    InstructionError::AliasTargetNotFound {
                        alias: alias.clone(),
                        target: target.clone(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns the instructions requiring a specific capability.
    pub fn requiring_capability(
        &self,
        capability: InstructionCapability,
    ) -> Vec<&Instruction> {
        self.instructions
            .values()
            .filter(|instruction| {
                instruction
                    .required_capabilities
                    .contains(&capability)
            })
            .collect()
    }

    /// Returns all instructions of a particular semantic kind.
    pub fn of_kind(
        &self,
        kind: InstructionKind,
    ) -> Vec<&Instruction> {
        self.instructions
            .values()
            .filter(|instruction| instruction.kind == kind)
            .collect()
    }

    /// Creates a conservative standard gate-model instruction set.
    ///
    /// This does not claim that every hardware backend supports these
    /// instructions. It is a reusable vocabulary for constructing backend
    /// instruction sets.
    pub fn standard_gate_model() -> Result<Self, InstructionError> {
        let mut set = Self::new();

        set.register(
            Instruction::single_qubit_gate("x", "Pauli-X")?
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("x"),
                ),
        )?;

        set.register(
            Instruction::single_qubit_gate("y", "Pauli-Y")?
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("y"),
                ),
        )?;

        set.register(
            Instruction::single_qubit_gate("z", "Pauli-Z")?
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("z"),
                ),
        )?;

        let h = Instruction::single_qubit_gate(
            "h",
            "Hadamard",
        )?
        .unitary()
        .reversible()
        .with_adjoint()
        .with_interoperability(
            InteroperabilityNames::new()
                .with_openqasm("h"),
        );

        set.register(h)?;

        let mut cx = Instruction::two_qubit_gate(
            "cx",
            "Controlled-X",
        )?
        .unitary()
        .reversible()
        .with_adjoint()
        .controllable()
        .with_interoperability(
            InteroperabilityNames::new()
                .with_openqasm("cx"),
        );

        cx.description =
            Some("Controlled-NOT gate.".to_owned());

        set.register(cx)?;

        let rz = Instruction::single_qubit_gate(
            "rz",
            "Z rotation",
        )?
        .unitary()
        .reversible()
        .with_adjoint()
        .requiring(
            InstructionCapability::ParameterizedCircuits,
        );

        let rz = Instruction {
            parameters: vec![ParameterSpec::angle(0, "theta")
                .with_description(
                    "Rotation angle in radians.",
                )],
            ..rz
        };

        set.register(rz)?;

        let mut measure =
            Instruction::measurement("measure")?;

        measure.interoperability =
            InteroperabilityNames::new()
                .with_openqasm("measure");

        set.register(measure)?;

        let mut reset = Instruction::reset("reset")?;

        reset.interoperability =
            InteroperabilityNames::new()
                .with_openqasm("reset");

        set.register(reset)?;

        Ok(set)
    }
}

// =============================================================================
// Alias validation helper
// =============================================================================

fn normalize_alias(
    alias: &str,
) -> Result<String, InstructionError> {
    let id = InstructionId::new(alias)
        .map_err(InstructionError::InvalidId)?;

    Ok(id.into_string())
}

// =============================================================================
// Standard instruction constructors
// =============================================================================

/// Returns a standard single-qubit X gate.
pub fn x_instruction() -> Result<Instruction, InstructionError> {
    Instruction::single_qubit_gate("x", "Pauli-X")
        .map(|instruction| {
            instruction
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("x"),
                )
        })
}

/// Returns a standard single-qubit Y gate.
pub fn y_instruction() -> Result<Instruction, InstructionError> {
    Instruction::single_qubit_gate("y", "Pauli-Y")
        .map(|instruction| {
            instruction
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("y"),
                )
        })
}

/// Returns a standard single-qubit Z gate.
pub fn z_instruction() -> Result<Instruction, InstructionError> {
    Instruction::single_qubit_gate("z", "Pauli-Z")
        .map(|instruction| {
            instruction
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("z"),
                )
        })
}

/// Returns a standard Hadamard gate.
pub fn h_instruction() -> Result<Instruction, InstructionError> {
    Instruction::single_qubit_gate("h", "Hadamard")
        .map(|instruction| {
            instruction
                .unitary()
                .reversible()
                .with_adjoint()
                .with_interoperability(
                    InteroperabilityNames::new()
                        .with_openqasm("h"),
                )
        })
}

/// Returns a standard controlled-X gate.
pub fn cx_instruction() -> Result<Instruction, InstructionError> {
    Instruction::two_qubit_gate(
        "cx",
        "Controlled-X",
    )
    .map(|instruction| {
        instruction
            .unitary()
            .reversible()
            .with_adjoint()
            .with_interoperability(
                InteroperabilityNames::new()
                    .with_openqasm("cx"),
            )
    })
}

/// Returns a standard parameterized RZ gate.
pub fn rz_instruction() -> Result<Instruction, InstructionError> {
    let instruction = Instruction::single_qubit_gate(
        "rz",
        "Z rotation",
    )?
    .unitary()
    .reversible()
    .with_adjoint()
    .requiring(
        InstructionCapability::ParameterizedCircuits,
    )
    .with_interoperability(
        InteroperabilityNames::new()
            .with_openqasm("rz"),
    );

    Ok(Instruction {
        parameters: vec![
            ParameterSpec::angle(0, "theta")
                .with_description(
                    "Rotation angle in radians.",
                ),
        ],
        ..instruction
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_ids_are_canonicalized() {
        let id =
            InstructionId::new("CX").expect("valid ID");

        assert_eq!(id.as_str(), "cx");
    }

    #[test]
    fn instruction_ids_reject_whitespace() {
        assert_eq!(
            InstructionId::new("c x"),
            Err(InstructionIdError::Whitespace)
        );
    }

    #[test]
    fn instruction_ids_reject_empty_values() {
        assert_eq!(
            InstructionId::new(""),
            Err(InstructionIdError::Empty)
        );
    }

    #[test]
    fn instruction_ids_reject_non_ascii() {
        assert_eq!(
            InstructionId::new("xé"),
            Err(InstructionIdError::NonAscii)
        );
    }

    #[test]
    fn instruction_ids_reject_invalid_characters() {
        assert!(matches!(
            InstructionId::new("x!"),
            Err(InstructionIdError::InvalidCharacters { .. })
        ));
    }

    #[test]
    fn instruction_ids_accept_provider_namespace() {
        let id = InstructionId::new("custom:provider/cx")
            .expect("valid provider identifier");

        assert_eq!(
            id.as_str(),
            "custom:provider/cx"
        );
    }

    #[test]
    fn single_qubit_instruction_has_one_operand() {
        let instruction =
            x_instruction().expect("valid instruction");

        assert_eq!(
            instruction.required_operand_count(),
            1
        );

        assert_eq!(
            instruction.maximum_operand_count(),
            1
        );
    }

    #[test]
    fn two_qubit_instruction_has_two_operands() {
        let instruction =
            cx_instruction().expect("valid instruction");

        assert_eq!(
            instruction.required_operand_count(),
            2
        );

        assert_eq!(
            instruction.maximum_operand_count(),
            2
        );

        assert_eq!(
            instruction.operands[0].role.as_deref(),
            Some("control")
        );

        assert_eq!(
            instruction.operands[1].role.as_deref(),
            Some("target")
        );
    }

    #[test]
    fn parameter_positions_must_be_contiguous() {
        let id =
            InstructionId::new("invalid").expect("valid ID");

        let result = Instruction::new(
            id,
            InstructionKind::SingleQubitGate,
            "Invalid",
            vec![
                OperandSpec::required(
                    0,
                    OperandKind::Qubit,
                ),
            ],
            vec![
                ParameterSpec::real(1, "theta"),
            ],
        );

        assert_eq!(
            result,
            Err(
                InstructionError::NonContiguousParameterPositions
            )
        );
    }

    #[test]
    fn optional_parameters_must_be_last() {
        let id =
            InstructionId::new("invalid").expect("valid ID");

        let result = Instruction::new(
            id,
            InstructionKind::SingleQubitGate,
            "Invalid",
            vec![
                OperandSpec::required(
                    0,
                    OperandKind::Qubit,
                ),
            ],
            vec![
                ParameterSpec::real(0, "theta")
                    .optional(),
                ParameterSpec::real(1, "phi"),
            ],
        );

        assert_eq!(
            result,
            Err(
                InstructionError::RequiredParameterAfterOptional
            )
        );
    }

    #[test]
    fn optional_operands_must_be_last() {
        let id =
            InstructionId::new("invalid").expect("valid ID");

        let result = Instruction::new(
            id,
            InstructionKind::Custom,
            "Invalid",
            vec![
                OperandSpec::optional(
                    0,
                    OperandKind::Qubit,
                ),
                OperandSpec::required(
                    1,
                    OperandKind::Qubit,
                ),
            ],
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(
                InstructionError::RequiredOperandAfterOptional
            )
        );
    }

    #[test]
    fn duplicate_instruction_ids_are_rejected() {
        let mut set = InstructionSet::new();

        set.register(
            x_instruction().expect("valid X"),
        )
        .expect("registration succeeds");

        assert!(matches!(
            set.register(
                x_instruction().expect("valid X")
            ),
            Err(
                InstructionError::DuplicateInstruction { .. }
            )
        ));
    }

    #[test]
    fn aliases_resolve_to_canonical_instruction() {
        let mut set = InstructionSet::new();

        let instruction =
            x_instruction().expect("valid X");

        let id = instruction.id.clone();

        set.register(instruction)
            .expect("registration succeeds");

        set.register_alias("pauli_x", &id)
            .expect("alias succeeds");

        let resolved = set
            .resolve("pauli_x")
            .expect("alias resolves");

        assert_eq!(resolved.id, id);
    }

    #[test]
    fn aliases_are_removed_with_instruction() {
        let mut set = InstructionSet::new();

        let instruction =
            x_instruction().expect("valid X");

        let id = instruction.id.clone();

        set.register(instruction)
            .expect("registration succeeds");

        set.register_alias("pauli_x", &id)
            .expect("alias succeeds");

        set.remove(&id)
            .expect("removal succeeds");

        assert!(!set.contains("pauli_x"));
        assert_eq!(set.alias_count(), 0);
    }

    #[test]
    fn alias_cannot_conflict_with_instruction() {
        let mut set = InstructionSet::new();

        let x =
            x_instruction().expect("valid X");

        set.register(x)
            .expect("registration succeeds");

        let h =
            h_instruction().expect("valid H");

        let h_id = h.id.clone();

        set.register(h)
            .expect("registration succeeds");

        assert!(matches!(
            set.register_alias("x", &h_id),
            Err(
                InstructionError::AliasConflictsWithInstruction { .. }
            )
        ));
    }

    #[test]
    fn standard_gate_model_is_deterministic() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        let names: Vec<&str> = set
            .ids()
            .map(InstructionId::as_str)
            .collect();

        assert_eq!(
            names,
            vec![
                "cx",
                "h",
                "measure",
                "reset",
                "rz",
                "x",
                "y",
                "z",
            ]
        );
    }

    #[test]
    fn standard_set_contains_measurement_and_reset() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        assert!(set.contains("measure"));
        assert!(set.contains("reset"));
    }

    #[test]
    fn standard_set_contains_parameterized_rz() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        let rz = set.resolve("rz")
            .expect("RZ exists");

        assert_eq!(rz.parameters.len(), 1);
        assert_eq!(
            rz.parameters[0].parameter_type,
            ParameterType::Angle
        );
    }

    #[test]
    fn standard_set_has_openqasm_names() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        assert_eq!(
            set.resolve("cx")
                .unwrap()
                .interoperability
                .openqasm
                .as_deref(),
            Some("cx")
        );
    }

    #[test]
    fn canonical_name_projection_is_deterministic() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        let names = set.canonical_names();

        assert!(names.contains("x"));
        assert!(names.contains("cx"));
        assert!(names.contains("rz"));
        assert!(names.contains("measure"));
        assert!(names.contains("reset"));
    }

    #[test]
    fn compatibility_constructor_accepts_legacy_names() {
        let names =
            vec!["x", "cx", "measure", "reset"];

        let set =
            InstructionSet::from_canonical_names(names)
                .expect("legacy names");

        assert_eq!(set.len(), 4);
        assert!(set.contains("cx"));
    }

    #[test]
    fn probability_domain_accepts_zero_and_one() {
        let domain =
            ParameterDomain::Probability;

        assert!(domain.validate_f64(0.0).is_ok());
        assert!(domain.validate_f64(1.0).is_ok());
        assert!(
            domain.validate_f64(1.1).is_err()
        );
        assert!(
            domain.validate_f64(-0.1).is_err()
        );
    }

    #[test]
    fn probability_domain_rejects_non_finite_values() {
        let domain =
            ParameterDomain::Probability;

        assert!(
            domain.validate_f64(f64::NAN).is_err()
        );

        assert!(
            domain.validate_f64(f64::INFINITY).is_err()
        );

        assert!(
            domain.validate_f64(f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn integer_domain_works() {
        let domain =
            ParameterDomain::IntegerRange {
                min: 0,
                max: 10,
            };

        assert!(domain.validate_i64(0).is_ok());
        assert!(domain.validate_i64(10).is_ok());
        assert!(domain.validate_i64(11).is_err());
        assert!(domain.validate_i64(-1).is_err());
    }

    #[test]
    fn dynamic_instruction_requires_dynamic_capability() {
        let id =
            InstructionId::new("dynamic.test")
                .expect("valid ID");

        let mut instruction = Instruction::new(
            id,
            InstructionKind::Custom,
            "Dynamic test",
            Vec::new(),
            Vec::new(),
        )
        .expect("valid instruction");

        instruction.dynamic = true;

        assert_eq!(
            instruction.validate(),
            Err(
                InstructionError::MissingRequiredCapability {
                    capability:
                        InstructionCapability::DynamicCircuits
                }
            )
        );
    }

    #[test]
    fn dynamic_builder_adds_capability() {
        let id =
            InstructionId::new("dynamic.test")
                .expect("valid ID");

        let instruction = Instruction::new(
            id,
            InstructionKind::Custom,
            "Dynamic test",
            Vec::new(),
            Vec::new(),
        )
        .expect("valid instruction")
        .dynamic();

        assert!(
            instruction
                .required_capabilities
                .contains(
                    &InstructionCapability::DynamicCircuits
                )
        );

        assert!(instruction.validate().is_ok());
    }

    #[test]
    fn measurement_is_not_unitary() {
        let measurement =
            Instruction::measurement("measure")
                .expect("valid measurement");

        assert!(!measurement.unitary);
        assert!(
            measurement
                .required_capabilities
                .contains(
                    &InstructionCapability::Measurement
                )
        );
    }

    #[test]
    fn reset_is_not_unitary() {
        let reset =
            Instruction::reset("reset")
                .expect("valid reset");

        assert!(!reset.unitary);
        assert!(
            reset
                .required_capabilities
                .contains(
                    &InstructionCapability::Reset
                )
        );
    }

    #[test]
    fn standard_set_validates() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        assert!(set.validate().is_ok());
    }

    #[test]
    fn aliases_are_normalized() {
        let mut set = InstructionSet::new();

        let instruction =
            x_instruction().expect("valid X");

        let id = instruction.id.clone();

        set.register(instruction)
            .expect("registration succeeds");

        set.register_alias("PAULI_X", &id)
            .expect("alias succeeds");

        assert!(set.contains("pauli_x"));
        assert!(set.contains("PAULI_X"));
    }

    #[test]
    fn instruction_kind_strings_are_stable() {
        assert_eq!(
            InstructionKind::SingleQubitGate.as_str(),
            "single_qubit_gate"
        );

        assert_eq!(
            InstructionKind::Measurement.as_str(),
            "measurement"
        );

        assert_eq!(
            InstructionKind::Pulse.as_str(),
            "pulse"
        );

        assert_eq!(
            InstructionKind::Analog.as_str(),
            "analog"
        );
    }

    #[test]
    fn operand_kind_strings_are_stable() {
        assert_eq!(
            OperandKind::Qubit.as_str(),
            "qubit"
        );

        assert_eq!(
            OperandKind::DriveChannel.as_str(),
            "drive_channel"
        );

        assert_eq!(
            OperandKind::LogicalQubit.as_str(),
            "logical_qubit"
        );
    }

    #[test]
    fn capability_strings_are_stable() {
        assert_eq!(
            InstructionCapability::Measurement.as_str(),
            "measurement"
        );

        assert_eq!(
            InstructionCapability::DynamicCircuits.as_str(),
            "dynamic_circuits"
        );

        assert_eq!(
            InstructionCapability::PulseControl.as_str(),
            "pulse_control"
        );
    }

    #[test]
    fn no_instruction_can_have_required_operand_after_optional() {
        let id =
            InstructionId::new("invalid")
                .expect("valid ID");

        let result = Instruction::new(
            id,
            InstructionKind::Custom,
            "Invalid",
            vec![
                OperandSpec::required(
                    0,
                    OperandKind::Qubit,
                ),
                OperandSpec::optional(
                    1,
                    OperandKind::ClassicalBit,
                ),
                OperandSpec::required(
                    2,
                    OperandKind::Qubit,
                ),
            ],
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(
                InstructionError::RequiredOperandAfterOptional
            )
        );
    }

    #[test]
    fn instruction_set_lookup_is_case_insensitive() {
        let set =
            InstructionSet::standard_gate_model()
                .expect("standard set");

        assert_eq!(
            set.resolve("CX")
                .expect("CX resolves")
                .id
                .as_str(),
            "cx"
        );
    }

    #[test]
    fn remove_unknown_instruction_returns_structured_error() {
        let mut set = InstructionSet::new();

        let id =
            InstructionId::new("does.not.exist")
                .expect("valid ID");

        assert_eq!(
            set.remove(&id),
            Err(
                InstructionError::InstructionNotFound {
                    id
                }
            )
        );
    }
}