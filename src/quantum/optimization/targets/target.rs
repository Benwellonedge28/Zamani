//! Zamani Quantum Optimization — Optimization Target
//!
//! Production-grade, backend-independent description of the target against
//! which logical quantum-circuit optimization is performed.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                          │
//!                          ▼
//!                optimization::target
//!                          │
//!             ┌────────────┼────────────┐
//!             │            │            │
//!             ▼            ▼            ▼
//!         gate_set     constraints   cost hints
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                       planner
//!                          │
//!                          ▼
//!                     pass pipeline
//! ```
//!
//! This module answers:
//!
//! > What kind of circuit representation should the optimizer prefer, and
//! > what operations/properties are acceptable for that target?
//!
//! It deliberately does NOT own:
//!
//! - physical topology;
//! - logical-to-physical mapping;
//! - routing;
//! - pulse scheduling;
//! - backend communication;
//! - calibration acquisition;
//! - QPU execution;
//! - provider credentials;
//! - benchmark execution;
//! - quantum error-correction algorithms;
//! - optimization passes themselves;
//! - circuit semantics.
//!
//! Those responsibilities belong to their respective Zamani subsystems.
//!
//! # Canonical IR
//!
//! The canonical quantum representation remains:
//!
//! `crate::quantum::ir`
//!
//! This file never defines another `QuantumGate`, `QuantumOperation`, or
//! circuit representation.
//!
//! # Target versus hardware
//!
//! An optimization target is intentionally broader than a hardware device.
//!
//! A target may describe:
//!
//! - an abstract logical gate basis;
//! - a simulator;
//! - a fault-tolerant logical gate set;
//! - a superconducting-style native basis;
//! - a trapped-ion-style basis;
//! - a neutral-atom-style basis;
//! - a photonic basis;
//! - an analog/continuous-variable target;
//! - a custom Zamani quantum architecture.
//!
//! Physical topology remains owned by `quantum::hardware::topology` and
//! `quantum::routing`.
//!
//! Therefore a target can be used before a physical device is selected.
//!
//! # Design principles
//!
//! 1. **Immutable after construction.**
//!    The optimizer should observe one coherent target for an optimization run.
//!
//! 2. **Deterministic.**
//!    Collections use ordered containers and identifiers are canonicalized.
//!
//! 3. **Extensible.**
//!    Custom operations are represented by stable textual identifiers rather
//!    than forcing every future quantum operation into a Rust enum.
//!
//! 4. **No artificial circuit-size ceiling.**
//!    `Option<u128>` is used where a target may impose a resource capacity.
//!    `None` means that this target does not impose that particular target-level
//!    bound. It does not claim that physical resources are literally infinite.
//!
//! 5. **No hidden hardware state.**
//!    This file contains a declarative target snapshot only.
//!
//! 6. **No unsafe Rust.**
//!
//! 7. **No global mutable state.**
//!
//! 8. **No provider-specific dependencies.**
//!
//! 9. **Target selection is separate from target construction.**
//!    `TargetSelection` lives in `optimization::config`; this module resolves
//!    it into a concrete `OptimizationTarget`.
//!
//! 10. **Future files consume this contract.**
//!     `gate_set.rs`, `constraints.rs`, `profiles.rs`, `planner.rs`, `cost.rs`,
//!     `pipeline.rs`, and serialization should consume this file rather than
//!     changing its core meaning.
//!
//! # External architectural alignment
//!
//! Modern quantum compiler target models commonly associate supported
//! operations with implementation properties such as duration and error and
//! distinguish globally supported operations from constrained variants.
//! Zamani follows the useful part of that model while deliberately keeping
//! physical topology outside this optimization target.
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
//! No unsafe code is permitted.
//!
//! # Integration contract
//!
//! This file is intentionally usable before the rest of `targets/` exists.
//!
//! Later files should integrate as follows:
//!
//! ```text
//! targets/target.rs
//!       │
//!       ├── targets/gate_set.rs
//!       │      └── consumes TargetOperation
//!       │
//!       ├── targets/constraints.rs
//!       │      └── consumes TargetConstraints
//!       │
//!       ├── targets/profiles.rs
//!       │      └── constructs predefined targets
//!       │
//!       ├── optimization/config.rs
//!       │      └── TargetSelection → resolve()
//!       │
//!       ├── optimization/planner.rs
//!       │      └── selects passes according to target
//!       │
//!       ├── optimization/cost.rs
//!       │      └── consumes operation cost hints
//!       │
//!       └── optimization/pipeline.rs
//!              └── receives immutable target
//! ```
//!
//! The target must never need to be modified merely because a new optimization
//! pass is introduced.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::quantum::optimization::config::TargetSelection;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for optimization targets.
pub const TARGET_SCHEMA_ID: &str = "zamani.quantum.optimization.target";

/// Semantic version of the target contract.
///
/// Increment this only when the meaning of an existing serialized field or
/// invariant changes incompatibly.
pub const TARGET_SCHEMA_VERSION: u32 = 1;

/// Maximum target identifier length.
pub const MAX_TARGET_ID_LENGTH: usize = 512;

/// Maximum target display-name length.
pub const MAX_TARGET_NAME_LENGTH: usize = 512;

/// Maximum operation identifier length.
pub const MAX_OPERATION_ID_LENGTH: usize = 512;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of metadata properties.
pub const MAX_METADATA_PROPERTIES: usize = 16_384;

/// Maximum number of target operations accepted by one target descriptor.
///
/// This is a validation guard, not a circuit-size limit.
///
/// `OptimizationTarget::unbounded_operation_count()` can be used by callers
/// that intentionally construct a target programmatically and do not want
/// this validation policy to be used.
pub const MAX_TARGET_OPERATIONS: usize = 1_000_000;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type used by target construction and validation.
pub type TargetResult<T> = Result<T, TargetError>;

/// Errors produced by optimization-target construction or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    /// The target identifier is empty.
    EmptyIdentifier {
        /// Affected field.
        field: &'static str,
    },

    /// An identifier exceeds the allowed representation size.
    IdentifierTooLong {
        /// Affected field.
        field: &'static str,

        /// Maximum permitted length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// An identifier contains unsupported characters.
    InvalidIdentifier {
        /// Affected field.
        field: &'static str,

        /// Supplied identifier.
        value: String,
    },

    /// An operation identifier is duplicated.
    DuplicateOperation {
        /// Duplicated identifier.
        operation: String,
    },

    /// An operation has invalid arity.
    InvalidArity {
        /// Operation identifier.
        operation: String,

        /// Minimum accepted arity.
        minimum: usize,

        /// Maximum accepted arity, when bounded.
        maximum: Option<usize>,
    },

    /// A target declares an impossible parameter count.
    InvalidParameterCount {
        /// Operation identifier.
        operation: String,

        /// Parameter count.
        count: usize,
    },

    /// A target property contains an invalid numeric value.
    InvalidNumericValue {
        /// Field name.
        field: &'static str,
    },

    /// A target resource capacity is invalid.
    InvalidCapacity {
        /// Field name.
        field: &'static str,
    },

    /// An operation has incompatible properties.
    InvalidOperationProperties {
        /// Operation identifier.
        operation: String,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// Too many target operations were supplied.
    TooManyOperations {
        /// Maximum accepted count.
        maximum: usize,

        /// Actual count.
        actual: usize,
    },

    /// Too much metadata was supplied.
    TooMuchMetadata {
        /// Maximum accepted property count.
        maximum: usize,

        /// Actual count.
        actual: usize,
    },

    /// A metadata key or value is invalid.
    InvalidMetadata {
        /// Affected field.
        field: &'static str,
    },

    /// The selected target is not resolvable.
    UnknownTarget {
        /// Requested target.
        target: String,
    },

    /// The target is semantically inconsistent.
    InvalidTarget {
        /// Description of the inconsistency.
        message: String,
    },
}

impl fmt::Display for TargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "target field `{field}` must not be empty")
            }

            Self::IdentifierTooLong {
                field,
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "target field `{field}` exceeds maximum length {maximum}: actual {actual}"
                )
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "target field `{field}` contains invalid identifier `{value}`"
                )
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "target operation `{operation}` is already defined")
            }

            Self::InvalidArity {
                operation,
                minimum,
                maximum,
            } => match maximum {
                Some(maximum) => write!(
                    formatter,
                    "target operation `{operation}` has invalid arity range {minimum}..={maximum}"
                ),
                None => write!(
                    formatter,
                    "target operation `{operation}` has invalid minimum arity {minimum}"
                ),
            },

            Self::InvalidParameterCount { operation, count } => {
                write!(
                    formatter,
                    "target operation `{operation}` has invalid parameter count {count}"
                )
            }

            Self::InvalidNumericValue { field } => {
                write!(
                    formatter,
                    "target numeric field `{field}` must be finite and non-negative"
                )
            }

            Self::InvalidCapacity { field } => {
                write!(
                    formatter,
                    "target capacity field `{field}` is invalid"
                )
            }

            Self::InvalidOperationProperties { operation, reason } => {
                write!(
                    formatter,
                    "target operation `{operation}` has invalid properties: {reason}"
                )
            }

            Self::TooManyOperations { maximum, actual } => {
                write!(
                    formatter,
                    "target contains too many operations: maximum {maximum}, actual {actual}"
                )
            }

            Self::TooMuchMetadata { maximum, actual } => {
                write!(
                    formatter,
                    "target contains too many metadata properties: maximum {maximum}, actual {actual}"
                )
            }

            Self::InvalidMetadata { field } => {
                write!(formatter, "invalid target metadata field `{field}`")
            }

            Self::UnknownTarget { target } => {
                write!(formatter, "unknown optimization target `{target}`")
            }

            Self::InvalidTarget { message } => {
                write!(formatter, "invalid optimization target: {message}")
            }
        }
    }
}

impl std::error::Error for TargetError {}

// =============================================================================
// Target identity
// =============================================================================

/// Stable target identifier.
///
/// The identifier is intentionally independent of a Rust type name. It may
/// therefore be serialized, used in compiler configuration, stored in
/// provenance, or supplied by Danga without coupling those systems to Rust
/// implementation details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TargetId(String);

impl TargetId {
    /// Creates and validates a target identifier.
    pub fn new(value: impl Into<String>) -> TargetResult<Self> {
        let value = canonical_identifier(value.into(), "target")?;
        Ok(Self(value))
    }

    /// Returns the canonical target identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for TargetId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Target kind
// =============================================================================

/// Semantic category of an optimization target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    /// Abstract hardware-independent logical target.
    Generic,

    /// Classical quantum simulator target.
    Simulator,

    /// Hardware-emulating target.
    Emulator,

    /// Physical QPU-oriented target.
    Qpu,

    /// Logical fault-tolerant target.
    Logical,

    /// Analog quantum target.
    Analog,

    /// Continuous-variable/photonic-oriented target.
    ContinuousVariable,

    /// Quantum annealing target.
    Annealing,

    /// Measurement-based quantum-computing target.
    MeasurementBased,

    /// User-defined target.
    Custom,
}

impl TargetKind {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::Logical => "logical",
            Self::Analog => "analog",
            Self::ContinuousVariable => "continuous_variable",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the target represents a physical execution target.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Qpu)
    }

    /// Returns whether the target is hardware-independent.
    #[must_use]
    pub const fn is_abstract(self) -> bool {
        matches!(
            self,
            Self::Generic
                | Self::Logical
                | Self::Simulator
                | Self::Emulator
        )
    }
}

impl Default for TargetKind {
    fn default() -> Self {
        Self::Generic
    }
}

impl fmt::Display for TargetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Technology
// =============================================================================

/// Broad technology classification.
///
/// This is descriptive metadata. It does not replace hardware capabilities
/// or topology.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetTechnology {
    /// No technology was specified.
    Abstract,

    /// Superconducting-style gate-model technology.
    Superconducting,

    /// Trapped-ion technology.
    TrappedIon,

    /// Neutral-atom technology.
    NeutralAtom,

    /// Photonic technology.
    Photonic,

    /// Semiconductor/spin technology.
    Spin,

    /// Quantum-dot technology.
    QuantumDot,

    /// Topological technology.
    Topological,

    /// Continuous-variable optical technology.
    ContinuousVariable,

    /// Annealing technology.
    Annealing,

    /// Measurement-based technology.
    MeasurementBased,

    /// User-defined technology identifier.
    Custom(String),
}

impl Default for TargetTechnology {
    fn default() -> Self {
        Self::Abstract
    }
}

impl TargetTechnology {
    /// Returns a stable textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Abstract => "abstract",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::Photonic => "photonic",
            Self::Spin => "spin",
            Self::QuantumDot => "quantum_dot",
            Self::Topological => "topological",
            Self::ContinuousVariable => "continuous_variable",
            Self::Annealing => "annealing",
            Self::MeasurementBased => "measurement_based",
            Self::Custom(value) => value.as_str(),
        }
    }
}

// =============================================================================
// Operation arity
// =============================================================================

/// Arity range accepted by a target operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OperationArity {
    /// Minimum number of quantum operands.
    pub minimum: usize,

    /// Maximum number of quantum operands.
    ///
    /// `None` means variable/unbounded arity subject to the target's other
    /// constraints.
    pub maximum: Option<usize>,
}

impl OperationArity {
    /// Creates an exact arity.
    #[must_use]
    pub const fn exact(value: usize) -> Self {
        Self {
            minimum: value,
            maximum: Some(value),
        }
    }

    /// Creates an arbitrary arity beginning at `minimum`.
    #[must_use]
    pub const fn at_least(minimum: usize) -> Self {
        Self {
            minimum,
            maximum: None,
        }
    }

    /// Returns whether the supplied arity is accepted.
    #[must_use]
    pub const fn accepts(self, actual: usize) -> bool {
        if actual < self.minimum {
            return false;
        }

        match self.maximum {
            Some(maximum) => actual <= maximum,
            None => true,
        }
    }

    /// Validates the arity range.
    pub fn validate(self, operation: &str) -> TargetResult<()> {
        if let Some(maximum) = self.maximum {
            if maximum < self.minimum {
                return Err(TargetError::InvalidArity {
                    operation: operation.to_owned(),
                    minimum: self.minimum,
                    maximum: self.maximum,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Parameter domain
// =============================================================================

/// Domain accepted by a target operation's parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterDomain {
    /// Operation has no parameters.
    None,

    /// Every finite real parameter is accepted.
    AnyFinite,

    /// Parameter must be within a finite inclusive range.
    Range {
        /// Minimum accepted value.
        minimum: f64,

        /// Maximum accepted value.
        maximum: f64,
    },

    /// Parameter is restricted to an explicit finite set.
    Discrete(Vec<f64>),

    /// Parameter domain is implementation-defined.
    Custom(String),
}

impl Default for ParameterDomain {
    fn default() -> Self {
        Self::None
    }
}

impl ParameterDomain {
    /// Returns whether a concrete value is accepted.
    ///
    /// `Custom` cannot be evaluated by this file and therefore returns
    /// `false`. A target-specific extension must perform that validation.
    #[must_use]
    pub fn accepts(&self, value: f64) -> bool {
        if !value.is_finite() {
            return false;
        }

        match self {
            Self::None => false,

            Self::AnyFinite => true,

            Self::Range { minimum, maximum } => {
                minimum.is_finite()
                    && maximum.is_finite()
                    && minimum <= maximum
                    && value >= *minimum
                    && value <= *maximum
            }

            Self::Discrete(values) => values
                .iter()
                .any(|candidate| candidate.is_finite() && *candidate == value),

            Self::Custom(_) => false,
        }
    }

    /// Validates the domain.
    pub fn validate(&self) -> TargetResult<()> {
        match self {
            Self::None | Self::AnyFinite | Self::Custom(_) => Ok(()),

            Self::Range { minimum, maximum } => {
                if !minimum.is_finite() || !maximum.is_finite() || minimum > maximum {
                    return Err(TargetError::InvalidNumericValue {
                        field: "parameter_range",
                    });
                }

                Ok(())
            }

            Self::Discrete(values) => {
                if values.iter().any(|value| !value.is_finite()) {
                    return Err(TargetError::InvalidNumericValue {
                        field: "parameter_domain",
                    });
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Operation properties
// =============================================================================

/// Cost and semantic properties of one target-supported operation.
///
/// These are optimization hints, not authoritative hardware calibration.
///
/// Hardware calibration remains owned by the hardware subsystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    /// Estimated execution duration in arbitrary target-defined time units.
    ///
    /// The unit must be documented by the target.
    pub duration: Option<f64>,

    /// Estimated operation error probability/rate.
    pub error: Option<f64>,

    /// Estimated energy/resource consumption.
    pub energy: Option<f64>,

    /// Whether the operation is parameterized.
    pub parameterized: bool,

    /// Whether arbitrary parameter values are accepted.
    pub arbitrary_parameters: bool,

    /// Whether the operation is invertible.
    pub invertible: bool,

    /// Whether the operation is self-inverse.
    pub self_inverse: bool,

    /// Whether the operation is Clifford for all accepted parameters.
    pub clifford: bool,

    /// Whether the operation is natively supported or merely an optimizer
    /// preference.
    pub native: bool,

    /// Whether the operation can be used as a control operation.
    pub controllable: bool,

    /// Whether the operation may be used inside a classically controlled
    /// region.
    pub classically_controlled: bool,

    /// Maximum approximation error tolerated by this operation, if applicable.
    pub approximation_tolerance: Option<f64>,
}

impl Default for OperationProperties {
    fn default() -> Self {
        Self {
            duration: None,
            error: None,
            energy: None,
            parameterized: false,
            arbitrary_parameters: false,
            invertible: true,
            self_inverse: false,
            clifford: false,
            native: true,
            controllable: false,
            classically_controlled: false,
            approximation_tolerance: None,
        }
    }
}

impl OperationProperties {
    /// Creates conservative properties for an exact native operation.
    #[must_use]
    pub fn exact_native() -> Self {
        Self::default()
    }

    /// Validates numeric fields.
    pub fn validate(&self) -> TargetResult<()> {
        validate_optional_non_negative(self.duration, "operation_duration")?;
        validate_optional_non_negative(self.error, "operation_error")?;
        validate_optional_non_negative(self.energy, "operation_energy")?;
        validate_optional_non_negative(
            self.approximation_tolerance,
            "approximation_tolerance",
        )?;

        if let Some(error) = self.error {
            if error > 1.0 {
                return Err(TargetError::InvalidNumericValue {
                    field: "operation_error",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Operation descriptor
// =============================================================================

/// One operation accepted by an optimization target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetOperation {
    /// Stable operation identifier.
    pub id: String,

    /// Quantum operand arity.
    pub arity: OperationArity,

    /// Number of classical parameters.
    pub parameter_count: usize,

    /// Accepted parameter domain.
    pub parameter_domain: ParameterDomain,

    /// Operation implementation properties.
    pub properties: OperationProperties,

    /// Optional alias identifiers.
    ///
    /// Aliases are normalized to stable operation IDs during construction.
    pub aliases: BTreeSet<String>,
}

impl TargetOperation {
    /// Creates a validated target operation.
    pub fn new(
        id: impl Into<String>,
        arity: OperationArity,
        parameter_count: usize,
    ) -> TargetResult<Self> {
        let id = canonical_identifier(id.into(), "operation")?;

        let operation = Self {
            id,
            arity,
            parameter_count,
            parameter_domain: if parameter_count == 0 {
                ParameterDomain::None
            } else {
                ParameterDomain::AnyFinite
            },
            properties: OperationProperties::default(),
            aliases: BTreeSet::new(),
        };

        operation.validate()?;

        Ok(operation)
    }

    /// Adds an alias.
    pub fn with_alias(mut self, alias: impl Into<String>) -> TargetResult<Self> {
        let alias = canonical_identifier(alias.into(), "operation_alias")?;

        if alias == self.id {
            return Err(TargetError::InvalidOperationProperties {
                operation: self.id.clone(),
                reason: "operation alias must differ from the canonical identifier",
            });
        }

        self.aliases.insert(alias);
        Ok(self)
    }

    /// Sets the parameter domain.
    #[must_use]
    pub fn with_parameter_domain(mut self, domain: ParameterDomain) -> Self {
        self.parameter_domain = domain;
        self
    }

    /// Sets operation properties.
    #[must_use]
    pub fn with_properties(mut self, properties: OperationProperties) -> Self {
        self.properties = properties;
        self
    }

    /// Returns whether this operation accepts the supplied arity.
    #[must_use]
    pub const fn accepts_arity(&self, arity: usize) -> bool {
        self.arity.accepts(arity)
    }

    /// Returns whether the operation accepts the supplied concrete parameter.
    #[must_use]
    pub fn accepts_parameter(&self, value: f64) -> bool {
        if self.parameter_count == 0 {
            return false;
        }

        self.parameter_domain.accepts(value)
    }

    /// Validates the complete operation descriptor.
    pub fn validate(&self) -> TargetResult<()> {
        self.arity.validate(&self.id)?;

        if self.parameter_count == 0 && !matches!(self.parameter_domain, ParameterDomain::None)
        {
            return Err(TargetError::InvalidOperationProperties {
                operation: self.id.clone(),
                reason: "zero-parameter operation must use ParameterDomain::None",
            });
        }

        if self.parameter_count > 0 && matches!(self.parameter_domain, ParameterDomain::None) {
            return Err(TargetError::InvalidOperationProperties {
                operation: self.id.clone(),
                reason: "parameterized operation cannot use ParameterDomain::None",
            });
        }

        self.parameter_domain.validate()?;
        self.properties.validate()?;

        for alias in &self.aliases {
            validate_identifier("operation_alias", alias)?;

            if alias == &self.id {
                return Err(TargetError::InvalidOperationProperties {
                    operation: self.id.clone(),
                    reason: "operation alias duplicates canonical identifier",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Target capabilities
// =============================================================================

/// Capabilities that influence optimization legality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetCapabilities {
    /// Mid-circuit measurement is supported.
    pub mid_circuit_measurement: bool,

    /// Measurement-dependent classical control is supported.
    pub classical_control: bool,

    /// Dynamic circuits are supported.
    pub dynamic_circuits: bool,

    /// Reset is supported.
    pub reset: bool,

    /// Parallel operations are supported.
    pub parallel_operations: bool,

    /// Parameterized operations can remain symbolic until execution.
    pub parameterized_operations: bool,

    /// Three-qubit operations are natively accepted.
    pub three_qubit_operations: bool,

    /// Variable-width/multi-qubit operations are accepted.
    pub multi_qubit_operations: bool,

    /// Approximate synthesis/optimization is permitted.
    pub approximate_optimization: bool,

    /// Logical qubit operations are supported.
    pub logical_operations: bool,

    /// Fault-tolerant resource optimization is meaningful for this target.
    pub fault_tolerant: bool,

    /// Pulse-level lowering is available downstream.
    pub pulse_level: bool,

    /// Analog operations are supported downstream.
    pub analog_operations: bool,

    /// Measurement-based operations are supported downstream.
    pub measurement_based: bool,

    /// Annealing operations are supported downstream.
    pub annealing: bool,

    /// Custom capability identifiers.
    pub custom: BTreeSet<String>,
}

impl Default for TargetCapabilities {
    fn default() -> Self {
        Self {
            mid_circuit_measurement: false,
            classical_control: false,
            dynamic_circuits: false,
            reset: true,
            parallel_operations: true,
            parameterized_operations: true,
            three_qubit_operations: true,
            multi_qubit_operations: false,
            approximate_optimization: false,
            logical_operations: false,
            fault_tolerant: false,
            pulse_level: false,
            analog_operations: false,
            measurement_based: false,
            annealing: false,
            custom: BTreeSet::new(),
        }
    }
}

impl TargetCapabilities {
    /// Creates conservative capabilities.
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            mid_circuit_measurement: false,
            classical_control: false,
            dynamic_circuits: false,
            reset: false,
            parallel_operations: false,
            parameterized_operations: false,
            three_qubit_operations: false,
            multi_qubit_operations: false,
            approximate_optimization: false,
            logical_operations: false,
            fault_tolerant: false,
            pulse_level: false,
            analog_operations: false,
            measurement_based: false,
            annealing: false,
            custom: BTreeSet::new(),
        }
    }

    /// Registers a custom capability.
    pub fn add_custom(&mut self, capability: impl Into<String>) -> TargetResult<()> {
        let capability = canonical_identifier(capability.into(), "capability")?;
        self.custom.insert(capability);
        Ok(())
    }

    /// Returns whether a capability is available.
    #[must_use]
    pub fn supports(&self, capability: &str) -> bool {
        match capability {
            "mid_circuit_measurement" => self.mid_circuit_measurement,
            "classical_control" => self.classical_control,
            "dynamic_circuits" => self.dynamic_circuits,
            "reset" => self.reset,
            "parallel_operations" => self.parallel_operations,
            "parameterized_operations" => self.parameterized_operations,
            "three_qubit_operations" => self.three_qubit_operations,
            "multi_qubit_operations" => self.multi_qubit_operations,
            "approximate_optimization" => self.approximate_optimization,
            "logical_operations" => self.logical_operations,
            "fault_tolerant" => self.fault_tolerant,
            "pulse_level" => self.pulse_level,
            "analog_operations" => self.analog_operations,
            "measurement_based" => self.measurement_based,
            "annealing" => self.annealing,
            other => self.custom.contains(other),
        }
    }
}

// =============================================================================
// Target constraints
// =============================================================================

/// Optimization-relevant resource constraints.
///
/// These are target-level constraints only. Physical topology and detailed
/// timing constraints remain outside this structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConstraints {
    /// Maximum logical qubits accepted by the target.
    ///
    /// `None` means this target does not impose a target-level logical-qubit
    /// limit.
    pub max_logical_qubits: Option<u128>,

    /// Maximum ancillas accepted by the target.
    pub max_ancillas: Option<u128>,

    /// Maximum operation arity.
    pub max_operation_arity: Option<usize>,

    /// Maximum symbolic parameter count for one operation.
    pub max_parameters_per_operation: Option<usize>,

    /// Maximum circuit depth preferred/allowed by this target.
    pub max_depth: Option<u128>,

    /// Maximum two-qubit depth.
    pub max_two_qubit_depth: Option<u128>,

    /// Maximum total operation count.
    ///
    /// `None` means the target itself imposes no target-level count.
    pub max_operations: Option<u128>,

    /// Maximum tolerated approximation error for approximate optimization.
    pub approximation_tolerance: Option<f64>,

    /// Whether barriers are semantically meaningful to this target.
    pub barriers_semantic: bool,

    /// Whether global phase may be ignored by downstream execution.
    pub ignores_global_phase: bool,
}

impl Default for TargetConstraints {
    fn default() -> Self {
        Self {
            max_logical_qubits: None,
            max_ancillas: None,
            max_operation_arity: None,
            max_parameters_per_operation: None,
            max_depth: None,
            max_two_qubit_depth: None,
            max_operations: None,
            approximation_tolerance: Some(0.0),
            barriers_semantic: true,
            ignores_global_phase: true,
        }
    }
}

impl TargetConstraints {
    /// Validates the constraint set.
    pub fn validate(&self) -> TargetResult<()> {
        validate_optional_positive(self.max_logical_qubits, "max_logical_qubits")?;
        validate_optional_positive(self.max_ancillas, "max_ancillas")?;
        validate_optional_positive(
            self.max_operation_arity.map(|value| value as u128),
            "max_operation_arity",
        )?;
        validate_optional_positive(
            self.max_parameters_per_operation.map(|value| value as u128),
            "max_parameters_per_operation",
        )?;
        validate_optional_positive(self.max_depth, "max_depth")?;
        validate_optional_positive(self.max_two_qubit_depth, "max_two_qubit_depth")?;
        validate_optional_positive(self.max_operations, "max_operations")?;

        if let Some(tolerance) = self.approximation_tolerance {
            if !tolerance.is_finite() || tolerance < 0.0 {
                return Err(TargetError::InvalidNumericValue {
                    field: "approximation_tolerance",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Target optimization policy
// =============================================================================

/// Target-specific optimization preferences.
///
/// This is intentionally a set of hints rather than a replacement for
/// `optimization::cost::CostModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetOptimizationPolicy {
    /// Relative preference for reducing total operation count.
    pub gate_count_weight: f64,

    /// Relative preference for reducing two-qubit operation count.
    pub two_qubit_weight: f64,

    /// Relative preference for reducing depth.
    pub depth_weight: f64,

    /// Relative preference for reducing T count.
    pub t_count_weight: f64,

    /// Relative preference for reducing T depth.
    pub t_depth_weight: f64,

    /// Relative preference for reducing estimated error.
    pub error_weight: f64,

    /// Relative preference for reducing duration.
    pub duration_weight: f64,

    /// Relative preference for reducing width.
    pub width_weight: f64,

    /// Whether target-native operations should be preferred over decomposed
    /// equivalent operations when semantic cost is otherwise equal.
    pub prefer_native_operations: bool,

    /// Whether exact transformations should be preferred over approximate
    /// transformations when cost is otherwise equal.
    pub prefer_exact_transformations: bool,

    /// Whether additional ancillas may be introduced by optimization.
    pub allow_ancillas: bool,

    /// Whether approximate transformations are allowed.
    pub allow_approximation: bool,
}

impl Default for TargetOptimizationPolicy {
    fn default() -> Self {
        Self {
            gate_count_weight: 1.0,
            two_qubit_weight: 10.0,
            depth_weight: 1.0,
            t_count_weight: 1.0,
            t_depth_weight: 1.0,
            error_weight: 1.0,
            duration_weight: 1.0,
            width_weight: 0.1,
            prefer_native_operations: true,
            prefer_exact_transformations: true,
            allow_ancillas: false,
            allow_approximation: false,
        }
    }
}

impl TargetOptimizationPolicy {
    /// Validates optimization weights.
    pub fn validate(&self) -> TargetResult<()> {
        validate_non_negative(self.gate_count_weight, "gate_count_weight")?;
        validate_non_negative(self.two_qubit_weight, "two_qubit_weight")?;
        validate_non_negative(self.depth_weight, "depth_weight")?;
        validate_non_negative(self.t_count_weight, "t_count_weight")?;
        validate_non_negative(self.t_depth_weight, "t_depth_weight")?;
        validate_non_negative(self.error_weight, "error_weight")?;
        validate_non_negative(self.duration_weight, "duration_weight")?;
        validate_non_negative(self.width_weight, "width_weight")?;

        let all_zero = self.gate_count_weight == 0.0
            && self.two_qubit_weight == 0.0
            && self.depth_weight == 0.0
            && self.t_count_weight == 0.0
            && self.t_depth_weight == 0.0
            && self.error_weight == 0.0
            && self.duration_weight == 0.0
            && self.width_weight == 0.0;

        if all_zero {
            return Err(TargetError::InvalidTarget {
                message: "target optimization policy has no active cost dimensions".to_owned(),
            });
        }

        if self.allow_approximation && self.prefer_exact_transformations {
            // This is legal: exact transformations are simply preferred.
            // Do not reject the configuration.
        }

        Ok(())
    }
}

// =============================================================================
// Target metadata
// =============================================================================

/// Deterministic target metadata.
///
/// Metadata is descriptive and must never contain credentials, API tokens,
/// passwords, private keys, cookies, or other secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMetadata {
    /// Human-readable target description.
    pub description: Option<String>,

    /// Provider-neutral metadata.
    pub properties: BTreeMap<String, String>,
}

impl Default for TargetMetadata {
    fn default() -> Self {
        Self {
            description: None,
            properties: BTreeMap::new(),
        }
    }
}

impl TargetMetadata {
    /// Adds metadata after validating its representation.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> TargetResult<()> {
        if self.properties.len() >= MAX_METADATA_PROPERTIES {
            return Err(TargetError::TooMuchMetadata {
                maximum: MAX_METADATA_PROPERTIES,
                actual: self.properties.len() + 1,
            });
        }

        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        if contains_secret_marker(&key) || contains_secret_marker(&value) {
            return Err(TargetError::InvalidMetadata {
                field: "secret_like_metadata",
            });
        }

        self.properties.insert(key, value);

        Ok(())
    }

    /// Validates the complete metadata set.
    pub fn validate(&self) -> TargetResult<()> {
        if let Some(description) = &self.description {
            if description.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(TargetError::InvalidMetadata {
                    field: "description",
                });
            }

            if contains_secret_marker(description) {
                return Err(TargetError::InvalidMetadata {
                    field: "description",
                });
            }
        }

        if self.properties.len() > MAX_METADATA_PROPERTIES {
            return Err(TargetError::TooMuchMetadata {
                maximum: MAX_METADATA_PROPERTIES,
                actual: self.properties.len(),
            });
        }

        for (key, value) in &self.properties {
            validate_metadata_key(key)?;
            validate_metadata_value(value)?;

            if contains_secret_marker(key) || contains_secret_marker(value) {
                return Err(TargetError::InvalidMetadata {
                    field: "secret_like_metadata",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Optimization target
// =============================================================================

/// Complete immutable optimization target.
///
/// This is the central contract consumed by the optimizer planner, cost model,
/// decomposition system, and target-aware passes.
///
/// The structure is cheap to clone because the internal representation is
/// reference-counted. Cloning a target therefore does not duplicate large
/// operation registries.
///
/// The target remains immutable after construction.
#[derive(Debug, Clone)]
pub struct OptimizationTarget {
    inner: Arc<TargetDefinition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TargetDefinition {
    schema_id: String,
    schema_version: u32,
    id: TargetId,
    name: String,
    kind: TargetKind,
    technology: TargetTechnology,
    version: Option<String>,
    operations: BTreeMap<String, TargetOperation>,
    aliases: BTreeMap<String, String>,
    capabilities: TargetCapabilities,
    constraints: TargetConstraints,
    optimization: TargetOptimizationPolicy,
    metadata: TargetMetadata,
}

impl OptimizationTarget {
    /// Creates a new target builder.
    #[must_use]
    pub fn builder(id: impl Into<String>) -> TargetBuilder {
        TargetBuilder::new(id)
    }

    /// Creates a generic hardware-independent target.
    ///
    /// The generic target intentionally does not claim that every possible
    /// quantum operation is natively supported. It supplies the canonical
    /// logical operations represented by the current Zamani IR.
    pub fn generic() -> TargetResult<Self> {
        let mut builder = Self::builder("generic")
            .name("Generic Zamani Quantum Target")
            .kind(TargetKind::Generic)
            .technology(TargetTechnology::Abstract)
            .version("1")
            .capabilities(TargetCapabilities {
                mid_circuit_measurement: true,
                classical_control: true,
                dynamic_circuits: true,
                reset: true,
                parallel_operations: true,
                parameterized_operations: true,
                three_qubit_operations: true,
                multi_qubit_operations: true,
                approximate_optimization: false,
                logical_operations: true,
                fault_tolerant: true,
                pulse_level: false,
                analog_operations: false,
                measurement_based: true,
                annealing: false,
                custom: BTreeSet::new(),
            });

        add_current_zamani_ir_operations(&mut builder)?;

        builder.build()
    }

    /// Creates a conservative logical target.
    pub fn logical() -> TargetResult<Self> {
        let mut builder = Self::builder("logical")
            .name("Zamani Logical Quantum Target")
            .kind(TargetKind::Logical)
            .technology(TargetTechnology::Abstract)
            .version("1")
            .capabilities(TargetCapabilities {
                logical_operations: true,
                fault_tolerant: true,
                parameterized_operations: true,
                reset: true,
                parallel_operations: true,
                three_qubit_operations: true,
                multi_qubit_operations: true,
                ..TargetCapabilities::default()
            });

        add_current_zamani_ir_operations(&mut builder)?;

        builder.build()
    }

    /// Resolves a configuration-level target selection.
    ///
    /// `Auto` currently resolves to the generic logical optimization target.
    /// Named targets are resolved only when this module has a built-in
    /// definition for them. Future `targets/profiles.rs` or a registry can
    /// extend this without changing the target representation.
    pub fn resolve(selection: &TargetSelection) -> TargetResult<Self> {
        match selection {
            TargetSelection::Auto | TargetSelection::Generic => Self::generic(),

            TargetSelection::Named(name) => {
                let canonical = canonical_identifier(name.clone(), "target")?;

                match canonical.as_str() {
                    "generic" => Self::generic(),
                    "logical" => Self::logical(),
                    other => Err(TargetError::UnknownTarget {
                        target: other.to_owned(),
                    }),
                }
            }
        }
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.inner.schema_id
    }

    /// Returns the target schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        TARGET_SCHEMA_VERSION
    }

    /// Returns the stable target identifier.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        &self.inner.id
    }

    /// Returns the human-readable target name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Returns the target kind.
    #[must_use]
    pub const fn kind(&self) -> TargetKind {
        self.inner.kind
    }

    /// Returns the target technology.
    #[must_use]
    pub fn technology(&self) -> &TargetTechnology {
        &self.inner.technology
    }

    /// Returns the target semantic version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.inner.version.as_deref()
    }

    /// Returns all supported canonical operation identifiers.
    #[must_use]
    pub fn operation_ids(&self) -> impl Iterator<Item = &str> {
        self.inner.operations.keys().map(String::as_str)
    }

    /// Returns the number of supported canonical operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.inner.operations.len()
    }

    /// Returns an operation descriptor by canonical identifier or alias.
    #[must_use]
    pub fn operation(&self, operation: &str) -> Option<&TargetOperation> {
        let canonical = normalize_lookup_identifier(operation)?;

        if let Some(operation) = self.inner.operations.get(&canonical) {
            return Some(operation);
        }

        let canonical = self.inner.aliases.get(&canonical)?;
        self.inner.operations.get(canonical)
    }

    /// Returns true if the target supports the operation at all.
    #[must_use]
    pub fn supports_operation(&self, operation: &str) -> bool {
        self.operation(operation).is_some()
    }

    /// Returns whether the target supports an operation with a given arity.
    #[must_use]
    pub fn supports_arity(&self, operation: &str, arity: usize) -> bool {
        self.operation(operation)
            .map(|value| value.accepts_arity(arity))
            .unwrap_or(false)
    }

    /// Returns whether a target operation accepts a concrete parameter.
    #[must_use]
    pub fn supports_parameter(&self, operation: &str, value: f64) -> bool {
        self.operation(operation)
            .map(|descriptor| descriptor.accepts_parameter(value))
            .unwrap_or(false)
    }

    /// Returns whether the operation is target-native.
    #[must_use]
    pub fn is_native(&self, operation: &str) -> bool {
        self.operation(operation)
            .map(|descriptor| descriptor.properties.native)
            .unwrap_or(false)
    }

    /// Returns whether the operation is invertible according to the target.
    #[must_use]
    pub fn is_invertible(&self, operation: &str) -> bool {
        self.operation(operation)
            .map(|descriptor| descriptor.properties.invertible)
            .unwrap_or(false)
    }

    /// Returns whether the operation is self-inverse according to the target.
    #[must_use]
    pub fn is_self_inverse(&self, operation: &str) -> bool {
        self.operation(operation)
            .map(|descriptor| descriptor.properties.self_inverse)
            .unwrap_or(false)
    }

    /// Returns whether the operation is Clifford for all target-supported
    /// parameters.
    #[must_use]
    pub fn is_clifford(&self, operation: &str) -> bool {
        self.operation(operation)
            .map(|descriptor| descriptor.properties.clifford)
            .unwrap_or(false)
    }

    /// Returns operation properties.
    #[must_use]
    pub fn operation_properties(&self, operation: &str) -> Option<&OperationProperties> {
        self.operation(operation)
            .map(|descriptor| &descriptor.properties)
    }

    /// Returns the target capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &TargetCapabilities {
        &self.inner.capabilities
    }

    /// Returns target resource constraints.
    #[must_use]
    pub fn constraints(&self) -> &TargetConstraints {
        &self.inner.constraints
    }

    /// Returns target optimization policy.
    #[must_use]
    pub fn optimization_policy(&self) -> &TargetOptimizationPolicy {
        &self.inner.optimization
    }

    /// Returns target metadata.
    #[must_use]
    pub fn metadata(&self) -> &TargetMetadata {
        &self.inner.metadata
    }

    /// Returns whether the target has a target-level logical-qubit capacity.
    #[must_use]
    pub fn has_logical_qubit_limit(&self) -> bool {
        self.constraints().max_logical_qubits.is_some()
    }

    /// Returns whether the target accepts the requested logical-qubit count.
    #[must_use]
    pub fn supports_logical_qubits(&self, count: u128) -> bool {
        self.constraints()
            .max_logical_qubits
            .map(|limit| count <= limit)
            .unwrap_or(true)
    }

    /// Returns whether the target accepts the requested ancilla count.
    #[must_use]
    pub fn supports_ancillas(&self, count: u128) -> bool {
        self.constraints()
            .max_ancillas
            .map(|limit| count <= limit)
            .unwrap_or(true)
    }

    /// Returns whether the target accepts the requested operation count.
    #[must_use]
    pub fn supports_operation_count(&self, count: u128) -> bool {
        self.constraints()
            .max_operations
            .map(|limit| count <= limit)
            .unwrap_or(true)
    }

    /// Returns whether the target accepts the requested depth.
    #[must_use]
    pub fn supports_depth(&self, depth: u128) -> bool {
        self.constraints()
            .max_depth
            .map(|limit| depth <= limit)
            .unwrap_or(true)
    }

    /// Returns whether approximate optimization is allowed.
    #[must_use]
    pub fn allows_approximation(&self) -> bool {
        self.inner.optimization.allow_approximation
            && self.inner.capabilities.approximate_optimization
            && self
                .inner
                .constraints
                .approximation_tolerance
                .map(|value| value > 0.0)
                .unwrap_or(false)
    }

    /// Returns the configured approximation tolerance.
    #[must_use]
    pub fn approximation_tolerance(&self) -> f64 {
        self.inner
            .constraints
            .approximation_tolerance
            .unwrap_or(0.0)
    }

    /// Returns whether the target ignores global phase.
    #[must_use]
    pub const fn ignores_global_phase(&self) -> bool {
        self.inner.constraints.ignores_global_phase
    }

    /// Validates the target.
    pub fn validate(&self) -> TargetResult<()> {
        if self.inner.schema_id != TARGET_SCHEMA_ID {
            return Err(TargetError::InvalidTarget {
                message: "unexpected target schema identifier".to_owned(),
            });
        }

        if self.inner.schema_version != TARGET_SCHEMA_VERSION {
            return Err(TargetError::InvalidTarget {
                message: "unsupported target schema version".to_owned(),
            });
        }

        validate_identifier("target", self.id().as_str())?;

        if self.name().is_empty() {
            return Err(TargetError::EmptyIdentifier {
                field: "target_name",
            });
        }

        if self.name().len() > MAX_TARGET_NAME_LENGTH {
            return Err(TargetError::IdentifierTooLong {
                field: "target_name",
                maximum: MAX_TARGET_NAME_LENGTH,
                actual: self.name().len(),
            });
        }

        if let Some(version) = self.version() {
            if version.is_empty() {
                return Err(TargetError::EmptyIdentifier {
                    field: "target_version",
                });
            }
        }

        if self.inner.operations.len() > MAX_TARGET_OPERATIONS {
            return Err(TargetError::TooManyOperations {
                maximum: MAX_TARGET_OPERATIONS,
                actual: self.inner.operations.len(),
            });
        }

        for (id, operation) in &self.inner.operations {
            if id != &operation.id {
                return Err(TargetError::InvalidTarget {
                    message: "operation map key differs from operation identifier".to_owned(),
                });
            }

            operation.validate()?;

            if let Some(maximum_arity) = self.constraints().max_operation_arity {
                if operation.arity.minimum > maximum_arity {
                    return Err(TargetError::InvalidOperationProperties {
                        operation: operation.id.clone(),
                        reason: "operation exceeds target maximum arity",
                    });
                }

                if let Some(operation_maximum) = operation.arity.maximum {
                    if operation_maximum > maximum_arity {
                        return Err(TargetError::InvalidOperationProperties {
                            operation: operation.id.clone(),
                            reason: "operation maximum arity exceeds target maximum arity",
                        });
                    }
                }
            }

            if let Some(maximum_parameters) = self.constraints().max_parameters_per_operation {
                if operation.parameter_count > maximum_parameters {
                    return Err(TargetError::InvalidOperationProperties {
                        operation: operation.id.clone(),
                        reason: "operation exceeds target parameter-count limit",
                    });
                }
            }
        }

        for (alias, canonical) in &self.inner.aliases {
            if !self.inner.operations.contains_key(canonical) {
                return Err(TargetError::InvalidTarget {
                    message: format!(
                        "target alias `{alias}` points to unknown operation `{canonical}`"
                    ),
                });
            }
        }

        self.capabilities().custom.iter().try_for_each(|capability| {
            validate_identifier("capability", capability)
        })?;

        self.constraints().validate()?;
        self.optimization_policy().validate()?;
        self.metadata().validate()?;

        if !self.capabilities().approximate_optimization
            && self.optimization_policy().allow_approximation
        {
            return Err(TargetError::InvalidTarget {
                message: "approximate optimization is enabled without target capability"
                    .to_owned(),
            });
        }

        if !self.capabilities().fault_tolerance
            && self.optimization_policy().t_count_weight > 0.0
            && self.kind() != TargetKind::Logical
        {
            // T-count may still be useful for ordinary targets, so this is not
            // rejected. It is deliberately only a compatibility observation.
        }

        Ok(())
    }

    /// Creates a stable deterministic fingerprint of the target.
    ///
    /// This is intentionally a non-cryptographic compiler identity suitable
    /// for caches and diagnostics. Cryptographic provenance hashes belong to
    /// the provenance/serialization subsystem.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hash = Fnv1a64::new();

        hash.write_str(self.schema_id());
        hash.write_u32(self.schema_version());
        hash.write_str(self.id().as_str());
        hash.write_str(self.name());
        hash.write_str(self.kind().as_str());
        hash.write_str(self.technology().as_str());

        if let Some(version) = self.version() {
            hash.write_str(version);
        }

        for operation in self.operation_ids() {
            hash.write_str(operation);

            if let Some(descriptor) = self.operation(operation) {
                hash.write_usize(descriptor.arity.minimum);

                match descriptor.arity.maximum {
                    Some(value) => {
                        hash.write_u8(1);
                        hash.write_usize(value);
                    }
                    None => hash.write_u8(0),
                }

                hash.write_usize(descriptor.parameter_count);
                hash.write_u8(u8::from(descriptor.properties.native));
                hash.write_u8(u8::from(descriptor.properties.invertible));
                hash.write_u8(u8::from(descriptor.properties.self_inverse));
                hash.write_u8(u8::from(descriptor.properties.clifford));
            }
        }

        hash.finish()
    }
}

impl PartialEq for OptimizationTarget {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for OptimizationTarget {}

// =============================================================================
// Target builder
// =============================================================================

/// Builder for immutable optimization targets.
///
/// The builder is intentionally separate from `OptimizationTarget` so target
/// construction can be validated completely before the target becomes visible
/// to compiler passes.
#[derive(Debug, Clone)]
pub struct TargetBuilder {
    id: TargetId,
    name: String,
    kind: TargetKind,
    technology: TargetTechnology,
    version: Option<String>,
    operations: BTreeMap<String, TargetOperation>,
    aliases: BTreeMap<String, String>,
    capabilities: TargetCapabilities,
    constraints: TargetConstraints,
    optimization: TargetOptimizationPolicy,
    metadata: TargetMetadata,
}

impl TargetBuilder {
    /// Creates a builder.
    pub fn new(id: impl Into<String>) -> TargetResult<Self> {
        let id = TargetId::new(id)?;

        Ok(Self {
            id,
            name: String::new(),
            kind: TargetKind::Generic,
            technology: TargetTechnology::Abstract,
            version: None,
            operations: BTreeMap::new(),
            aliases: BTreeMap::new(),
            capabilities: TargetCapabilities::default(),
            constraints: TargetConstraints::default(),
            optimization: TargetOptimizationPolicy::default(),
            metadata: TargetMetadata::default(),
        })
    }

    /// Sets the human-readable target name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the target kind.
    #[must_use]
    pub fn kind(mut self, kind: TargetKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the target technology.
    #[must_use]
    pub fn technology(mut self, technology: TargetTechnology) -> Self {
        self.technology = technology;
        self
    }

    /// Sets the target version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets target capabilities.
    #[must_use]
    pub fn capabilities(mut self, capabilities: TargetCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Sets target constraints.
    #[must_use]
    pub fn constraints(mut self, constraints: TargetConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    /// Sets target optimization policy.
    #[must_use]
    pub fn optimization_policy(
        mut self,
        optimization: TargetOptimizationPolicy,
    ) -> Self {
        self.optimization = optimization;
        self
    }

    /// Sets metadata.
    #[must_use]
    pub fn metadata(mut self, metadata: TargetMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds a supported operation.
    pub fn operation(mut self, operation: TargetOperation) -> TargetResult<Self> {
        self.insert_operation(operation)?;
        Ok(self)
    }

    /// Adds several supported operations.
    pub fn operations<I>(mut self, operations: I) -> TargetResult<Self>
    where
        I: IntoIterator<Item = TargetOperation>,
    {
        for operation in operations {
            self.insert_operation(operation)?;
        }

        Ok(self)
    }

    /// Inserts one operation into the builder.
    pub fn insert_operation(&mut self, operation: TargetOperation) -> TargetResult<()> {
        operation.validate()?;

        if self.operations.len() >= MAX_TARGET_OPERATIONS
            && !self.operations.contains_key(&operation.id)
        {
            return Err(TargetError::TooManyOperations {
                maximum: MAX_TARGET_OPERATIONS,
                actual: self.operations.len() + 1,
            });
        }

        if self.operations.contains_key(&operation.id) {
            return Err(TargetError::DuplicateOperation {
                operation: operation.id.clone(),
            });
        }

        for alias in &operation.aliases {
            if self.operations.contains_key(alias) || self.aliases.contains_key(alias) {
                return Err(TargetError::DuplicateOperation {
                    operation: alias.clone(),
                });
            }

            self.aliases.insert(alias.clone(), operation.id.clone());
        }

        self.operations.insert(operation.id.clone(), operation);

        Ok(())
    }

    /// Adds a metadata property.
    pub fn metadata_property(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> TargetResult<Self> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Builds the immutable target.
    pub fn build(self) -> TargetResult<OptimizationTarget> {
        let definition = TargetDefinition {
            schema_id: TARGET_SCHEMA_ID.to_owned(),
            schema_version: TARGET_SCHEMA_VERSION,
            id: self.id,
            name: self.name,
            kind: self.kind,
            technology: self.technology,
            version: self.version,
            operations: self.operations,
            aliases: self.aliases,
            capabilities: self.capabilities,
            constraints: self.constraints,
            optimization: self.optimization,
            metadata: self.metadata,
        };

        let target = OptimizationTarget {
            inner: Arc::new(definition),
        };

        target.validate()?;

        Ok(target)
    }
}

// =============================================================================
// Generic target construction
// =============================================================================

/// Adds all operations currently represented by Zamani's canonical gate IR.
///
/// This function is deliberately centralized here so the generic target has
/// one authoritative list. Future additions to `quantum::ir::GateKind` should
/// be added here as part of the IR/target integration contract.
///
/// Custom future operations do not require changing this file: callers can
/// register them through `TargetBuilder::operation`.
fn add_current_zamani_ir_operations(builder: &mut TargetBuilder) -> TargetResult<()> {
    add_simple(builder, "i", 1, false, true, true, true)?;
    add_simple(builder, "x", 1, false, true, true, true)?;
    add_simple(builder, "y", 1, false, true, true, true)?;
    add_simple(builder, "z", 1, false, true, true, true)?;
    add_simple(builder, "h", 1, false, true, true, true)?;
    add_simple(builder, "s", 1, false, true, false, true)?;
    add_simple(builder, "sdg", 1, false, true, false, true)?;
    add_simple(builder, "t", 1, false, true, false, false)?;
    add_simple(builder, "tdg", 1, false, true, false, false)?;
    add_simple(builder, "v", 1, false, true, false, false)?;
    add_simple(builder, "vdg", 1, false, true, false, false)?;

    add_parameterized(builder, "rx", 1, 1, true, false)?;
    add_parameterized(builder, "ry", 1, 1, true, false)?;
    add_parameterized(builder, "rz", 1, 1, true, false)?;
    add_parameterized(builder, "phase", 1, 1, true, false)?;
    add_parameterized(builder, "u1", 1, 1, true, false)?;
    add_parameterized(builder, "u2", 1, 2, true, false)?;
    add_parameterized(builder, "u3", 1, 3, true, false)?;

    add_simple(builder, "cx", 2, false, true, true, true)?;
    add_simple(builder, "cy", 2, false, true, true, true)?;
    add_simple(builder, "cz", 2, false, true, true, true)?;
    add_simple(builder, "ch", 2, false, true, true, true)?;
    add_simple(builder, "swap", 2, false, true, true, true)?;
    add_simple(builder, "iswap", 2, false, true, false, false)?;
    add_simple(builder, "ecr", 2, false, true, false, false)?;

    add_parameterized(builder, "crx", 2, 1, true, false)?;
    add_parameterized(builder, "cry", 2, 1, true, false)?;
    add_parameterized(builder, "crz", 2, 1, true, false)?;

    add_simple(builder, "ccx", 3, false, true, true, true)?;
    add_simple(builder, "cswap", 3, false, true, true, true)?;

    add_simple(builder, "measure", 1, false, false, false, false)?;
    add_simple(builder, "reset", 1, false, false, false, false)?;
    add_variable(builder, "barrier", 1, false, false, false, false)?;

    Ok(())
}

fn add_simple(
    builder: &mut TargetBuilder,
    id: &str,
    arity: usize,
    parameterized: bool,
    invertible: bool,
    self_inverse: bool,
    clifford: bool,
) -> TargetResult<()> {
    let mut operation = TargetOperation::new(
        id,
        OperationArity::exact(arity),
        if parameterized { 1 } else { 0 },
    )?;

    operation.properties = OperationProperties {
        parameterized,
        arbitrary_parameters: parameterized,
        invertible,
        self_inverse,
        clifford,
        native: true,
        ..OperationProperties::default()
    };

    if parameterized {
        operation.parameter_domain = ParameterDomain::AnyFinite;
    }

    builder.insert_operation(operation)
}

fn add_parameterized(
    builder: &mut TargetBuilder,
    id: &str,
    arity: usize,
    parameter_count: usize,
    invertible: bool,
    clifford: bool,
) -> TargetResult<()> {
    let mut operation =
        TargetOperation::new(id, OperationArity::exact(arity), parameter_count)?;

    operation.parameter_domain = ParameterDomain::AnyFinite;

    operation.properties = OperationProperties {
        parameterized: true,
        arbitrary_parameters: true,
        invertible,
        self_inverse: false,
        clifford,
        native: true,
        ..OperationProperties::default()
    };

    builder.insert_operation(operation)
}

fn add_variable(
    builder: &mut TargetBuilder,
    id: &str,
    minimum_arity: usize,
    parameterized: bool,
    invertible: bool,
    self_inverse: bool,
    clifford: bool,
) -> TargetResult<()> {
    let mut operation = TargetOperation::new(
        id,
        OperationArity::at_least(minimum_arity),
        if parameterized { 1 } else { 0 },
    )?;

    operation.properties = OperationProperties {
        parameterized,
        arbitrary_parameters: parameterized,
        invertible,
        self_inverse,
        clifford,
        native: true,
        ..OperationProperties::default()
    };

    if parameterized {
        operation.parameter_domain = ParameterDomain::AnyFinite;
    }

    builder.insert_operation(operation)
}

// =============================================================================
// Target utility functions
// =============================================================================

/// Returns the canonical target corresponding to the configuration selector.
pub fn resolve_target(selection: &TargetSelection) -> TargetResult<OptimizationTarget> {
    OptimizationTarget::resolve(selection)
}

/// Returns the default production optimization target.
pub fn default_target() -> TargetResult<OptimizationTarget> {
    OptimizationTarget::generic()
}

/// Validates a target without consuming it.
pub fn validate_target(target: &OptimizationTarget) -> TargetResult<()> {
    target.validate()
}

/// Returns whether a target selection resolves successfully.
#[must_use]
pub fn target_selection_is_supported(selection: &TargetSelection) -> bool {
    OptimizationTarget::resolve(selection).is_ok()
}

// =============================================================================
// Internal deterministic hash
// =============================================================================

/// Small deterministic FNV-1a implementation used only for target fingerprints.
///
/// This is deliberately not a cryptographic hash.
#[derive(Debug, Clone, Copy)]
struct Fnv1a64(u64);

impl Fnv1a64 {
    fn new() -> Self {
        Self(0xcbf29ce484222325)
    }

    fn write_byte(&mut self, byte: u8) {
        self.0 ^= u64::from(byte);
        self.0 = self.0.wrapping_mul(0x100000001b3);
    }

    fn write_u8(&mut self, value: u8) {
        self.write_byte(value);
    }

    fn write_u32(&mut self, value: u32) {
        for byte in value.to_le_bytes() {
            self.write_byte(byte);
        }
    }

    fn write_usize(&mut self, value: usize) {
        for byte in value.to_le_bytes() {
            self.write_byte(byte);
        }
    }

    fn write_str(&mut self, value: &str) {
        for byte in value.as_bytes() {
            self.write_byte(*byte);
        }

        // Include a separator so ["ab", "c"] and ["a", "bc"] cannot
        // accidentally produce the same concatenation.
        self.write_byte(0xff);
    }

    fn finish(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn canonical_identifier(value: String, field: &'static str) -> TargetResult<String> {
    if value.is_empty() {
        return Err(TargetError::EmptyIdentifier { field });
    }

    if value.len()
        > match field {
            "target" => MAX_TARGET_ID_LENGTH,
            "operation" | "operation_alias" => MAX_OPERATION_ID_LENGTH,
            _ => MAX_TARGET_ID_LENGTH,
        }
    {
        return Err(TargetError::IdentifierTooLong {
            field,
            maximum: match field {
                "target" => MAX_TARGET_ID_LENGTH,
                "operation" | "operation_alias" => MAX_OPERATION_ID_LENGTH,
                _ => MAX_TARGET_ID_LENGTH,
            },
            actual: value.len(),
        });
    }

    if !is_valid_identifier(&value) {
        return Err(TargetError::InvalidIdentifier {
            field,
            value,
        });
    }

    Ok(value.to_ascii_lowercase())
}

fn validate_identifier(field: &'static str, value: &str) -> TargetResult<()> {
    let _ = canonical_identifier(value.to_owned(), field)?;
    Ok(())
}

fn normalize_lookup_identifier(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    let value = value.trim();

    if value.is_empty() {
        return None;
    }

    Some(value.to_ascii_lowercase())
}

fn is_valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();

    let first = match characters.next() {
        Some(value) => value,
        None => return false,
    };

    if !(first.is_ascii_alphanumeric() || first == '_') {
        return false;
    }

    characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '-' | '.' | ':' | '/')
    })
}

fn validate_non_negative(value: f64, field: &'static str) -> TargetResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(TargetError::InvalidNumericValue { field });
    }

    Ok(())
}

fn validate_optional_non_negative(
    value: Option<f64>,
    field: &'static str,
) -> TargetResult<()> {
    if let Some(value) = value {
        validate_non_negative(value, field)?;
    }

    Ok(())
}

fn validate_optional_positive(
    value: Option<u128>,
    field: &'static str,
) -> TargetResult<()> {
    if let Some(value) = value {
        if value == 0 {
            return Err(TargetError::InvalidCapacity { field });
        }
    }

    Ok(())
}

fn validate_metadata_key(value: &str) -> TargetResult<()> {
    if value.is_empty() || value.len() > MAX_METADATA_KEY_LENGTH {
        return Err(TargetError::InvalidMetadata {
            field: "metadata_key",
        });
    }

    Ok(())
}

fn validate_metadata_value(value: &str) -> TargetResult<()> {
    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(TargetError::InvalidMetadata {
            field: "metadata_value",
        });
    }

    Ok(())
}

fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    const MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "authorization",
        "password",
        "passwd",
        "private_key",
        "secret_key",
        "session_cookie",
        "bearer ",
        "-----begin ",
    ];

    MARKERS.iter().any(|marker| lower.contains(marker))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_identifier_is_canonicalized() {
        let target = TargetId::new("Generic.Target").expect("valid target id");

        assert_eq!(target.as_str(), "generic.target");
    }

    #[test]
    fn invalid_identifier_is_rejected() {
        let result = TargetId::new("target with spaces");

        assert!(matches!(
            result,
            Err(TargetError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn generic_target_is_valid() {
        let target = OptimizationTarget::generic().expect("generic target");

        target.validate().expect("target must validate");
        assert_eq!(target.id().as_str(), "generic");
        assert!(target.supports_operation("cx"));
        assert!(target.supports_operation("rz"));
        assert!(target.supports_operation("measure"));
    }

    #[test]
    fn logical_target_is_valid() {
        let target = OptimizationTarget::logical().expect("logical target");

        target.validate().expect("target must validate");
        assert_eq!(target.kind(), TargetKind::Logical);
        assert!(target.capabilities().logical_operations);
        assert!(target.capabilities().fault_tolerance);
    }

    #[test]
    fn aliases_resolve() {
        let operation = TargetOperation::new("rx", OperationArity::exact(1), 1)
            .expect("operation")
            .with_alias("RX")
            .expect("alias");

        let target = OptimizationTarget::builder("test")
            .name("Test")
            .operation(operation)
            .expect("insert operation")
            .build()
            .expect("build target");

        assert!(target.supports_operation("rx"));
        assert!(target.supports_operation("RX"));
    }

    #[test]
    fn operation_arity_is_enforced() {
        let target = OptimizationTarget::generic().expect("generic target");

        assert!(target.supports_arity("cx", 2));
        assert!(!target.supports_arity("cx", 1));
        assert!(!target.supports_arity("cx", 3));
    }

    #[test]
    fn parameterized_operations_accept_finite_values() {
        let target = OptimizationTarget::generic().expect("generic target");

        assert!(target.supports_parameter("rz", 0.0));
        assert!(target.supports_parameter("rz", std::f64::consts::PI));
        assert!(!target.supports_parameter("rz", f64::NAN));
        assert!(!target.supports_parameter("rz", f64::INFINITY));
    }

    #[test]
    fn non_parameterized_operations_reject_parameters() {
        let target = OptimizationTarget::generic().expect("generic target");

        assert!(!target.supports_parameter("x", 1.0));
    }

    #[test]
    fn native_operation_properties_are_available() {
        let target = OptimizationTarget::generic().expect("generic target");

        assert!(target.is_native("cx"));
        assert!(target.is_native("rz"));
        assert!(target.is_clifford("h"));
        assert!(target.is_self_inverse("h"));
    }

    #[test]
    fn target_has_no_artificial_logical_qubit_limit() {
        let target = OptimizationTarget::generic().expect("generic target");

        assert!(!target.has_logical_qubit_limit());
        assert!(target.supports_logical_qubits(u128::MAX));
    }

    #[test]
    fn explicit_logical_qubit_limit_is_enforced() {
        let constraints = TargetConstraints {
            max_logical_qubits: Some(128),
            ..TargetConstraints::default()
        };

        let target = OptimizationTarget::builder("limited")
            .name("Limited")
            .constraints(constraints)
            .build()
            .expect("build target");

        assert!(target.supports_logical_qubits(128));
        assert!(!target.supports_logical_qubits(129));
    }

    #[test]
    fn approximation_requires_capability_and_positive_tolerance() {
        let constraints = TargetConstraints {
            approximation_tolerance: Some(1.0e-8),
            ..TargetConstraints::default()
        };

        let policy = TargetOptimizationPolicy {
            allow_approximation: true,
            ..TargetOptimizationPolicy::default()
        };

        let result = OptimizationTarget::builder("approx")
            .name("Approx")
            .constraints(constraints)
            .optimization_policy(policy)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn metadata_rejects_secret_like_values() {
        let mut metadata = TargetMetadata::default();

        let result = metadata.insert("api_key", "not-a-real-key");

        assert!(matches!(
            result,
            Err(TargetError::InvalidMetadata { .. })
        ));
    }

    #[test]
    fn target_fingerprint_is_deterministic() {
        let first = OptimizationTarget::generic()
            .expect("generic target")
            .fingerprint();

        let second = OptimizationTarget::generic()
            .expect("generic target")
            .fingerprint();

        assert_eq!(first, second);
    }

    #[test]
    fn target_fingerprint_changes_when_target_changes() {
        let first = OptimizationTarget::generic()
            .expect("generic target")
            .fingerprint();

        let second = OptimizationTarget::builder("different")
            .name("Different")
            .build()
            .expect("different target")
            .fingerprint();

        assert_ne!(first, second);
    }

    #[test]
    fn target_selection_resolves_generic() {
        let target =
            OptimizationTarget::resolve(&TargetSelection::Generic).expect("generic selection");

        assert_eq!(target.id().as_str(), "generic");
    }

    #[test]
    fn target_selection_resolves_auto() {
        let target =
            OptimizationTarget::resolve(&TargetSelection::Auto).expect("auto selection");

        assert_eq!(target.id().as_str(), "generic");
    }

    #[test]
    fn unknown_named_target_is_rejected() {
        let selection = TargetSelection::Named("does_not_exist".to_owned());

        let result = OptimizationTarget::resolve(&selection);

        assert!(matches!(
            result,
            Err(TargetError::UnknownTarget { .. })
        ));
    }

    #[test]
    fn custom_operations_do_not_require_ir_enum_changes() {
        let operation = TargetOperation::new(
            "custom.pauli_product_rotation",
            OperationArity::at_least(1),
            1,
        )
        .expect("custom operation");

        let target = OptimizationTarget::builder("custom")
            .name("Custom Target")
            .kind(TargetKind::Custom)
            .technology(TargetTechnology::Custom(
                "future_quantum_architecture".to_owned(),
            ))
            .operation(operation)
            .expect("operation")
            .build()
            .expect("target");

        assert!(target.supports_operation("custom.pauli_product_rotation"));
    }

    #[test]
    fn variable_arity_operations_are_supported() {
        let operation =
            TargetOperation::new("multi_controlled", OperationArity::at_least(2), 0)
                .expect("operation");

        let target = OptimizationTarget::builder("variable")
            .name("Variable")
            .operation(operation)
            .expect("operation")
            .build()
            .expect("target");

        assert!(target.supports_arity("multi_controlled", 2));
        assert!(target.supports_arity("multi_controlled", 10));
    }

    #[test]
    fn conservative_capabilities_are_safe() {
        let capabilities = TargetCapabilities::conservative();

        assert!(!capabilities.mid_circuit_measurement);
        assert!(!capabilities.dynamic_circuits);
        assert!(!capabilities.approximate_optimization);
    }

    #[test]
    fn target_operations_are_deterministically_ordered() {
        let target = OptimizationTarget::generic().expect("generic target");

        let ids: Vec<&str> = target.operation_ids().collect();

        let mut sorted = ids.clone();
        sorted.sort_unstable();

        assert_eq!(ids, sorted);
    }
}