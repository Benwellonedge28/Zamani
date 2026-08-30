//! Zamani Quantum Optimization — Target Constraints
//!
//! Production-grade, backend-independent target constraint model for the
//! Zamani quantum optimizer.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                  quantum::optimization
//!                              │
//!                              ▼
//!                 optimization::targets
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!         gate_set.rs    constraints.rs    target.rs
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                           planner
//!                              │
//!                              ▼
//!                      optimization passes
//! ```
//!
//! This module answers:
//!
//! > What resource, semantic, structural, control-flow, parameter,
//! > approximation, and execution-shape restrictions must an optimized
//! > circuit satisfy for a particular optimization target?
//!
//! # Ownership
//!
//! This module owns target-wide constraints.
//!
//! It does NOT own:
//!
//! - the canonical quantum IR;
//! - gate definitions;
//! - decomposition rules;
//! - circuit routing;
//! - physical topology;
//! - pulse scheduling;
//! - hardware calibration;
//! - QPU communication;
//! - provider credentials;
//! - execution;
//! - benchmarking orchestration;
//! - quantum error-correction algorithms;
//! - optimization passes.
//!
//! The canonical quantum representation remains:
//!
//! `crate::quantum::ir`
//!
//! Gate vocabulary remains owned by:
//!
//! `crate::quantum::optimization::targets::gate_set`
//!
//! Complete target composition remains owned by:
//!
//! `crate::quantum::optimization::targets::target`
//!
//! # Important architectural distinction
//!
//! A target constraint is not necessarily a physical hardware constraint.
//!
//! For example:
//!
//! - a logical fault-tolerant target may constrain T-count;
//! - a simulator target may have no qubit limit;
//! - a compiler profile may impose a maximum optimization depth;
//! - a hardware target may constrain circuit duration;
//! - a dynamic-circuit target may permit mid-circuit measurement;
//! - a restricted QIR target profile may forbid loops;
//! - an abstract target may leave every resource bound unbounded.
//!
//! Physical topology remains outside this file.
//!
//! In particular, this file must NOT contain:
//!
//! - coupling graphs;
//! - shortest paths;
//! - SWAP routing;
//! - physical qubit IDs;
//! - calibration tables;
//! - pulse schedules.
//!
//! Those belong to routing/hardware subsystems.
//!
//! # Unbounded scalability
//!
//! Zamani must not artificially impose a finite circuit-size ceiling merely
//! because a target has no declared capacity.
//!
//! Therefore resource quantities use:
//!
//! `Option<u128>`
//!
//! where:
//!
//! `Some(value)` = explicitly bounded.
//!
//! `None` = unbounded by this target contract.
//!
//! This does not claim that a physical computer has infinite resources.
//! Actual resource availability is determined by the execution environment.
//!
//! The optimizer must therefore treat `None` as:
//!
//! > no constraint known at this layer
//!
//! rather than:
//!
//! > infinite resources guaranteed.
//!
//! # Hard versus soft constraints
//!
//! Production optimization needs to distinguish constraints that make a
//! circuit invalid from constraints that merely influence optimization.
//!
//! Examples:
//!
//! Hard:
//!
//! - target does not permit mid-circuit measurement;
//! - target permits at most three operands;
//! - target permits at most N logical qubits;
//! - target does not support dynamic control flow.
//!
//! Soft:
//!
//! - preferred maximum depth;
//! - preferred maximum two-qubit count;
//! - preferred duration;
//! - optimization budget;
//! - target-specific cost threshold.
//!
//! This module therefore models both through `ConstraintStrength`.
//!
//! # Determinism
//!
//! Collections use ordered containers.
//!
//! There is no global mutable state.
//!
//! No randomness is used.
//!
//! # Numerical safety
//!
//! Floating-point values are accepted only when finite and non-negative where
//! the semantic meaning requires a magnitude, duration, probability, error,
//! tolerance, or cost.
//!
//! NaN and infinity are rejected.
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
//! `target.rs` must consume `TargetConstraints` from this module instead of
//! declaring a second `TargetConstraints` type.
//!
//! `gate_set.rs` should use the operation-level portions of this contract
//! where appropriate, while retaining ownership of operation definitions.
//!
//! `profiles.rs` should construct predefined constraint sets.
//!
//! `planner.rs` should inspect these constraints when selecting optimization
//! passes.
//!
//! `pipeline.rs` should enforce optimization-resource constraints through the
//! optimizer's own `OptimizationLimits`, not by confusing compiler budgets
//! with target capabilities.
//!
//! `verification/*` should use the semantic constraints when determining
//! whether an optimized result is target-valid.
//!
//! `serialization/*` should serialize this structure directly.
//!
//! `provenance.rs` should record the final constraint snapshot used for an
//! optimization run.
//!
//! # External architectural alignment
//!
//! Modern quantum target systems distinguish supported operations, resource
//! limits, control-flow capabilities, timing, and device validity. QIR target
//! profiles distinguish control-flow capabilities, while Cirq device
//! specifications distinguish gate arity, supported targets, durations, and
//! device validity. Zamani keeps those concepts explicit while ensuring that
//! physical topology remains outside the optimizer target contract.
//!
//! # Security and robustness
//!
//! This file:
//!
//! - contains no unsafe code;
//! - performs no I/O;
//! - performs no network access;
//! - performs no allocation proportional to circuit size;
//! - has no global mutable state;
//! - does not trust floating-point NaN/infinity;
//! - does not silently reinterpret invalid constraints;
//! - validates cross-field invariants;
//! - supports unbounded target-level resource fields with `Option<u128>`;
//! - provides deterministic serialization;
//! - provides stable machine-readable identifiers.

// -----------------------------------------------------------------------------
// Crate-level safety policy
// -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for target constraints.
pub const TARGET_CONSTRAINTS_SCHEMA_ID: &str =
    "zamani.quantum.optimization.target_constraints";

/// Semantic version of the serialized constraint contract.
pub const TARGET_CONSTRAINTS_SCHEMA_VERSION: u32 = 1;

/// Maximum length of a constraint identifier.
pub const MAX_CONSTRAINT_ID_LENGTH: usize = 512;

/// Maximum length of a metadata key.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum length of a metadata value.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of custom constraints attached to one target.
pub const MAX_CUSTOM_CONSTRAINTS: usize = 16_384;

/// Maximum number of metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 16_384;

/// Maximum number of explicitly listed supported control-flow operations.
pub const MAX_CONTROL_FLOW_OPERATIONS: usize = 4096;

// =============================================================================
// Result and error
// =============================================================================

/// Result type for target-constraint operations.
pub type ConstraintResult<T> = Result<T, ConstraintError>;

/// Errors produced while constructing or validating target constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,
    },

    /// An identifier exceeds its maximum representation length.
    IdentifierTooLong {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Maximum allowed length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// An identifier contains unsupported characters.
    InvalidIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Supplied value.
        value: String,
    },

    /// A numerical value is invalid.
    InvalidNumericValue {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A resource bound is invalid.
    InvalidResourceBound {
        /// Name of the invalid resource.
        field: &'static str,
    },

    /// A range is malformed.
    InvalidRange {
        /// Name of the affected field.
        field: &'static str,
    },

    /// A constraint is internally inconsistent.
    InvalidConstraint {
        /// Human-readable explanation.
        message: String,
    },

    /// Too many custom constraints were supplied.
    TooManyCustomConstraints {
        /// Maximum allowed count.
        maximum: usize,

        /// Actual count.
        actual: usize,
    },

    /// Too much metadata was supplied.
    TooMuchMetadata {
        /// Maximum allowed count.
        maximum: usize,

        /// Actual count.
        actual: usize,
    },

    /// A metadata key is invalid.
    InvalidMetadataKey {
        /// Invalid key.
        key: String,
    },

    /// A metadata value is invalid.
    InvalidMetadataValue {
        /// Invalid key.
        key: String,
    },

    /// Too many control-flow operation identifiers were supplied.
    TooManyControlFlowOperations {
        /// Maximum allowed count.
        maximum: usize,

        /// Actual count.
        actual: usize,
    },
}

impl fmt::Display for ConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "constraint field `{field}` must not be empty")
            }

            Self::IdentifierTooLong {
                field,
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "constraint field `{field}` exceeds maximum length {maximum}: \
                     actual {actual}"
                )
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "constraint field `{field}` contains invalid identifier `{value}`"
                )
            }

            Self::InvalidNumericValue { field } => {
                write!(
                    formatter,
                    "constraint numeric field `{field}` must be finite and non-negative"
                )
            }

            Self::InvalidResourceBound { field } => {
                write!(
                    formatter,
                    "constraint resource bound `{field}` is invalid"
                )
            }

            Self::InvalidRange { field } => {
                write!(
                    formatter,
                    "constraint range `{field}` is invalid"
                )
            }

            Self::InvalidConstraint { message } => {
                write!(formatter, "invalid target constraint: {message}")
            }

            Self::TooManyCustomConstraints { maximum, actual } => {
                write!(
                    formatter,
                    "too many custom target constraints: maximum {maximum}, actual {actual}"
                )
            }

            Self::TooMuchMetadata { maximum, actual } => {
                write!(
                    formatter,
                    "too much target-constraint metadata: maximum {maximum}, actual {actual}"
                )
            }

            Self::InvalidMetadataKey { key } => {
                write!(formatter, "invalid target-constraint metadata key `{key}`")
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "invalid target-constraint metadata value for `{key}`"
                )
            }

            Self::TooManyControlFlowOperations { maximum, actual } => {
                write!(
                    formatter,
                    "too many control-flow operation identifiers: \
                     maximum {maximum}, actual {actual}"
                )
            }
        }
    }
}

impl std::error::Error for ConstraintError {}

// =============================================================================
// Constraint identifier
// =============================================================================

/// Stable identifier for a custom target constraint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConstraintId(String);

impl ConstraintId {
    /// Creates and validates a constraint identifier.
    pub fn new(value: impl Into<String>) -> ConstraintResult<Self> {
        let value = value.into();

        validate_identifier(
            "constraint",
            &value,
            MAX_CONSTRAINT_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ConstraintId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ConstraintId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Constraint strength
// =============================================================================

/// Whether a target constraint is mandatory or advisory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintStrength {
    /// Violating the constraint makes the target invalid.
    Hard,

    /// Violating the constraint is allowed but should influence optimization.
    Soft,
}

impl Default for ConstraintStrength {
    fn default() -> Self {
        Self::Hard
    }
}

// =============================================================================
// Constraint scope
// =============================================================================

/// Scope at which a constraint is evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintScope {
    /// Applies to the complete circuit.
    Circuit,

    /// Applies to one operation.
    Operation,

    /// Applies to one logical qubit.
    Qubit,

    /// Applies to one classical register/value domain.
    Classical,

    /// Applies to a region/block.
    Region,

    /// Applies to the target as a whole.
    Target,
}

impl Default for ConstraintScope {
    fn default() -> Self {
        Self::Circuit
    }
}

// =============================================================================
// Resource bounds
// =============================================================================

/// An optional non-negative resource capacity.
///
/// `None` means the target imposes no bound at this abstraction layer.
///
/// `Some(value)` means the target explicitly limits the resource.
///
/// The type is intentionally `u128` so that the optimizer does not impose an
/// unnecessary practical upper bound on circuit/resource counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceBound {
    /// Maximum permitted value.
    pub maximum: Option<u128>,
}

impl ResourceBound {
    /// Creates an unbounded resource.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { maximum: None }
    }

    /// Creates a bounded resource.
    #[must_use]
    pub const fn bounded(maximum: u128) -> Self {
        Self {
            maximum: Some(maximum),
        }
    }

    /// Returns whether this resource is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.maximum.is_none()
    }

    /// Returns whether a value satisfies the bound.
    #[must_use]
    pub const fn accepts(self, value: u128) -> bool {
        match self.maximum {
            Some(maximum) => value <= maximum,
            None => true,
        }
    }

    /// Returns the maximum when bounded.
    #[must_use]
    pub const fn maximum(self) -> Option<u128> {
        self.maximum
    }

    /// Validates the bound.
    ///
    /// All `u128` values are valid. The method exists so that resource bounds
    /// have a stable validation contract and can later gain representation
    /// metadata without changing callers.
    pub const fn validate(self) -> ConstraintResult<()> {
        Ok(())
    }
}

impl Default for ResourceBound {
    fn default() -> Self {
        Self::unbounded()
    }
}

// =============================================================================
// Floating-point range
// =============================================================================

/// Inclusive range over finite floating-point values.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NumericRange {
    /// Inclusive lower bound.
    pub minimum: f64,

    /// Inclusive upper bound.
    pub maximum: f64,
}

impl NumericRange {
    /// Creates a validated numeric range.
    pub fn new(minimum: f64, maximum: f64) -> ConstraintResult<Self> {
        let range = Self { minimum, maximum };
        range.validate()?;
        Ok(range)
    }

    /// Returns whether a value is accepted.
    #[must_use]
    pub fn accepts(&self, value: f64) -> bool {
        value.is_finite()
            && self.minimum.is_finite()
            && self.maximum.is_finite()
            && self.minimum <= value
            && value <= self.maximum
    }

    /// Validates the range.
    pub fn validate(&self) -> ConstraintResult<()> {
        if !self.minimum.is_finite() || !self.maximum.is_finite() {
            return Err(ConstraintError::InvalidNumericValue {
                field: "numeric_range",
            });
        }

        if self.minimum > self.maximum {
            return Err(ConstraintError::InvalidRange {
                field: "numeric_range",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Resource constraints
// =============================================================================

/// Target-level resource constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Maximum logical qubits.
    pub logical_qubits: ResourceBound,

    /// Maximum ancilla qubits.
    pub ancillas: ResourceBound,

    /// Maximum classical bits/register elements.
    pub classical_bits: ResourceBound,

    /// Maximum total operations.
    pub operations: ResourceBound,

    /// Maximum circuit depth.
    pub depth: ResourceBound,

    /// Maximum two-qubit depth.
    pub two_qubit_depth: ResourceBound,

    /// Maximum one-qubit operation count.
    pub single_qubit_operations: ResourceBound,

    /// Maximum two-qubit operation count.
    pub two_qubit_operations: ResourceBound,

    /// Maximum multi-qubit operation count.
    pub multi_qubit_operations: ResourceBound,

    /// Maximum measurement count.
    pub measurements: ResourceBound,

    /// Maximum reset count.
    pub resets: ResourceBound,

    /// Maximum barrier count.
    pub barriers: ResourceBound,

    /// Maximum delay operation count.
    pub delays: ResourceBound,

    /// Maximum control-flow operation count.
    pub control_flow_operations: ResourceBound,

    /// Maximum classical-control operation count.
    pub classical_control_operations: ResourceBound,

    /// Maximum T count.
    pub t_count: ResourceBound,

    /// Maximum T depth.
    pub t_depth: ResourceBound,

    /// Maximum non-Clifford operation count.
    pub non_clifford_operations: ResourceBound,

    /// Maximum logical execution duration, expressed in target-defined units.
    pub duration: Option<f64>,
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            logical_qubits: ResourceBound::unbounded(),
            ancillas: ResourceBound::unbounded(),
            classical_bits: ResourceBound::unbounded(),
            operations: ResourceBound::unbounded(),
            depth: ResourceBound::unbounded(),
            two_qubit_depth: ResourceBound::unbounded(),
            single_qubit_operations: ResourceBound::unbounded(),
            two_qubit_operations: ResourceBound::unbounded(),
            multi_qubit_operations: ResourceBound::unbounded(),
            measurements: ResourceBound::unbounded(),
            resets: ResourceBound::unbounded(),
            barriers: ResourceBound::unbounded(),
            delays: ResourceBound::unbounded(),
            control_flow_operations: ResourceBound::unbounded(),
            classical_control_operations: ResourceBound::unbounded(),
            t_count: ResourceBound::unbounded(),
            t_depth: ResourceBound::unbounded(),
            non_clifford_operations: ResourceBound::unbounded(),
            duration: None,
        }
    }
}

impl ResourceConstraints {
    /// Validates the complete resource-constraint set.
    pub fn validate(&self) -> ConstraintResult<()> {
        self.logical_qubits.validate()?;
        self.ancillas.validate()?;
        self.classical_bits.validate()?;
        self.operations.validate()?;
        self.depth.validate()?;
        self.two_qubit_depth.validate()?;
        self.single_qubit_operations.validate()?;
        self.two_qubit_operations.validate()?;
        self.multi_qubit_operations.validate()?;
        self.measurements.validate()?;
        self.resets.validate()?;
        self.barriers.validate()?;
        self.delays.validate()?;
        self.control_flow_operations.validate()?;
        self.classical_control_operations.validate()?;
        self.t_count.validate()?;
        self.t_depth.validate()?;
        self.non_clifford_operations.validate()?;

        if let Some(duration) = self.duration {
            validate_non_negative_finite(duration, "duration")?;
        }

        Ok(())
    }

    /// Returns whether the supplied logical-qubit count is accepted.
    #[must_use]
    pub const fn accepts_logical_qubits(&self, value: u128) -> bool {
        self.logical_qubits.accepts(value)
    }

    /// Returns whether the supplied operation count is accepted.
    #[must_use]
    pub const fn accepts_operations(&self, value: u128) -> bool {
        self.operations.accepts(value)
    }

    /// Returns whether the supplied depth is accepted.
    #[must_use]
    pub const fn accepts_depth(&self, value: u128) -> bool {
        self.depth.accepts(value)
    }

    /// Returns whether the supplied T count is accepted.
    #[must_use]
    pub const fn accepts_t_count(&self, value: u128) -> bool {
        self.t_count.accepts(value)
    }

    /// Returns whether the supplied T depth is accepted.
    #[must_use]
    pub const fn accepts_t_depth(&self, value: u128) -> bool {
        self.t_depth.accepts(value)
    }

    /// Returns whether the supplied duration is accepted.
    #[must_use]
    pub fn accepts_duration(&self, value: f64) -> bool {
        match self.duration {
            Some(maximum) => {
                value.is_finite() && value >= 0.0 && value <= maximum
            }
            None => value.is_finite() && value >= 0.0,
        }
    }
}

// =============================================================================
// Operation-shape constraints
// =============================================================================

/// Constraints on operation structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationConstraints {
    /// Maximum quantum operands accepted by any operation.
    pub max_arity: Option<usize>,

    /// Maximum classical parameters accepted by one operation.
    pub max_parameters_per_operation: Option<usize>,

    /// Maximum number of controls.
    pub max_controls: Option<usize>,

    /// Maximum number of targets.
    pub max_targets: Option<usize>,

    /// Whether variable/unbounded arity operations are allowed.
    pub variable_arity: bool,

    /// Whether operations with zero quantum operands are allowed.
    pub zero_qubit_operations: bool,

    /// Whether operations with one quantum operand are allowed.
    pub one_qubit_operations: bool,

    /// Whether two-qubit operations are allowed.
    pub two_qubit_operations: bool,

    /// Whether three-qubit operations are allowed.
    pub three_qubit_operations: bool,

    /// Whether operations with more than three operands are allowed.
    pub multi_qubit_operations: bool,
}

impl Default for OperationConstraints {
    fn default() -> Self {
        Self {
            max_arity: None,
            max_parameters_per_operation: None,
            max_controls: None,
            max_targets: None,
            variable_arity: true,
            zero_qubit_operations: false,
            one_qubit_operations: true,
            two_qubit_operations: true,
            three_qubit_operations: true,
            multi_qubit_operations: true,
        }
    }
}

impl OperationConstraints {
    /// Validates the operation-shape contract.
    pub fn validate(&self) -> ConstraintResult<()> {
        if let Some(max_arity) = self.max_arity {
            if max_arity == 0 && !self.zero_qubit_operations {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "max_arity=0 requires zero_qubit_operations=true".to_owned(),
                });
            }

            if max_arity < 1 && self.one_qubit_operations {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "one_qubit_operations cannot be enabled when max_arity < 1"
                            .to_owned(),
                });
            }

            if max_arity < 2 && self.two_qubit_operations {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "two_qubit_operations cannot be enabled when max_arity < 2"
                            .to_owned(),
                });
            }

            if max_arity < 3 && self.three_qubit_operations {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "three_qubit_operations cannot be enabled when max_arity < 3"
                            .to_owned(),
                });
            }

            if max_arity < 4 && self.multi_qubit_operations {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "multi_qubit_operations cannot be enabled when max_arity < 4"
                            .to_owned(),
                });
            }
        }

        if let (Some(max_controls), Some(max_arity)) =
            (self.max_controls, self.max_arity)
        {
            if max_controls > max_arity {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "max_controls cannot exceed max_arity".to_owned(),
                });
            }
        }

        if let (Some(max_targets), Some(max_arity)) =
            (self.max_targets, self.max_arity)
        {
            if max_targets > max_arity {
                return Err(ConstraintError::InvalidConstraint {
                    message:
                        "max_targets cannot exceed max_arity".to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Returns whether an operation arity is accepted.
    #[must_use]
    pub fn accepts_arity(&self, arity: usize) -> bool {
        if let Some(maximum) = self.max_arity {
            if arity > maximum {
                return false;
            }
        }

        match arity {
            0 => self.zero_qubit_operations,
            1 => self.one_qubit_operations,
            2 => self.two_qubit_operations,
            3 => self.three_qubit_operations,
            _ => self.multi_qubit_operations,
        }
    }

    /// Returns whether a parameter count is accepted.
    #[must_use]
    pub fn accepts_parameter_count(&self, count: usize) -> bool {
        match self.max_parameters_per_operation {
            Some(maximum) => count <= maximum,
            None => true,
        }
    }

    /// Returns whether a control/target shape is accepted.
    #[must_use]
    pub fn accepts_control_target_shape(
        &self,
        controls: usize,
        targets: usize,
    ) -> bool {
        if let Some(maximum) = self.max_controls {
            if controls > maximum {
                return false;
            }
        }

        if let Some(maximum) = self.max_targets {
            if targets > maximum {
                return false;
            }
        }

        controls
            .checked_add(targets)
            .map(|arity| self.accepts_arity(arity))
            .unwrap_or(false)
    }
}

// =============================================================================
// Parameter constraints
// =============================================================================

/// Constraints governing symbolic and concrete operation parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterConstraints {
    /// Whether symbolic parameters are permitted.
    pub symbolic_parameters: bool,

    /// Whether concrete parameters must be finite.
    pub finite_parameters: bool,

    /// Whether arbitrary finite parameter values are accepted.
    pub arbitrary_parameters: bool,

    /// Optional global parameter range.
    pub parameter_range: Option<NumericRange>,

    /// Maximum number of symbolic parameters in a circuit.
    pub max_symbolic_parameters: ResourceBound,

    /// Maximum number of distinct symbolic parameters.
    pub max_distinct_symbols: ResourceBound,

    /// Maximum expression depth permitted for one parameter expression.
    pub max_expression_depth: ResourceBound,

    /// Maximum number of nodes in one symbolic expression.
    pub max_expression_nodes: ResourceBound,

    /// Whether symbolic expressions may remain unresolved after optimization.
    pub unresolved_symbols_allowed: bool,
}

impl Default for ParameterConstraints {
    fn default() -> Self {
        Self {
            symbolic_parameters: true,
            finite_parameters: true,
            arbitrary_parameters: true,
            parameter_range: None,
            max_symbolic_parameters: ResourceBound::unbounded(),
            max_distinct_symbols: ResourceBound::unbounded(),
            max_expression_depth: ResourceBound::unbounded(),
            max_expression_nodes: ResourceBound::unbounded(),
            unresolved_symbols_allowed: true,
        }
    }
}

impl ParameterConstraints {
    /// Validates parameter constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        if let Some(range) = &self.parameter_range {
            range.validate()?;
        }

        self.max_symbolic_parameters.validate()?;
        self.max_distinct_symbols.validate()?;
        self.max_expression_depth.validate()?;
        self.max_expression_nodes.validate()?;

        if !self.symbolic_parameters
            && (self.unresolved_symbols_allowed
                || self.max_symbolic_parameters.maximum().is_some())
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "symbolic parameter limits cannot be active when symbolic_parameters=false"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Returns whether a concrete parameter is accepted.
    #[must_use]
    pub fn accepts_value(&self, value: f64) -> bool {
        if self.finite_parameters && !value.is_finite() {
            return false;
        }

        if !self.arbitrary_parameters {
            return self
                .parameter_range
                .as_ref()
                .map(|range| range.accepts(value))
                .unwrap_or(false);
        }

        match &self.parameter_range {
            Some(range) => range.accepts(value),
            None => value.is_finite() || !self.finite_parameters,
        }
    }

    /// Returns whether a symbolic-parameter count is accepted.
    #[must_use]
    pub const fn accepts_symbolic_parameter_count(
        &self,
        count: u128,
    ) -> bool {
        self.max_symbolic_parameters.accepts(count)
    }

    /// Returns whether a distinct-symbol count is accepted.
    #[must_use]
    pub const fn accepts_distinct_symbol_count(
        &self,
        count: u128,
    ) -> bool {
        self.max_distinct_symbols.accepts(count)
    }
}

// =============================================================================
// Timing constraints
// =============================================================================

/// Timing constraints known at the optimization-target layer.
///
/// Units are target-defined. The target should document its time unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingConstraints {
    /// Whether timing information is meaningful for this target.
    pub timing_is_semantic: bool,

    /// Optional maximum total circuit duration.
    pub max_circuit_duration: Option<f64>,

    /// Optional preferred maximum circuit duration.
    pub preferred_circuit_duration: Option<f64>,

    /// Optional maximum operation duration.
    pub max_operation_duration: Option<f64>,

    /// Optional maximum delay duration.
    pub max_delay_duration: Option<f64>,

    /// Whether parallel operations are permitted.
    pub parallel_operations: bool,

    /// Whether overlapping operations on disjoint qubits are allowed.
    pub overlapping_disjoint_operations: bool,

    /// Whether zero-duration operations are legal.
    pub zero_duration_operations: bool,

    /// Whether duration must be known before target validation.
    pub duration_required: bool,
}

impl Default for TimingConstraints {
    fn default() -> Self {
        Self {
            timing_is_semantic: false,
            max_circuit_duration: None,
            preferred_circuit_duration: None,
            max_operation_duration: None,
            max_delay_duration: None,
            parallel_operations: true,
            overlapping_disjoint_operations: true,
            zero_duration_operations: true,
            duration_required: false,
        }
    }
}

impl TimingConstraints {
    /// Validates timing constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        validate_optional_non_negative_finite(
            self.max_circuit_duration,
            "max_circuit_duration",
        )?;

        validate_optional_non_negative_finite(
            self.preferred_circuit_duration,
            "preferred_circuit_duration",
        )?;

        validate_optional_non_negative_finite(
            self.max_operation_duration,
            "max_operation_duration",
        )?;

        validate_optional_non_negative_finite(
            self.max_delay_duration,
            "max_delay_duration",
        )?;

        if let (Some(preferred), Some(maximum)) = (
            self.preferred_circuit_duration,
            self.max_circuit_duration,
        ) {
            if preferred > maximum {
                return Err(ConstraintError::InvalidRange {
                    field: "preferred_circuit_duration",
                });
            }
        }

        Ok(())
    }

    /// Returns whether a circuit duration satisfies the hard duration limit.
    #[must_use]
    pub fn accepts_circuit_duration(&self, duration: f64) -> bool {
        if !duration.is_finite() || duration < 0.0 {
            return false;
        }

        match self.max_circuit_duration {
            Some(maximum) => duration <= maximum,
            None => true,
        }
    }

    /// Returns whether an operation duration satisfies the target.
    #[must_use]
    pub fn accepts_operation_duration(&self, duration: f64) -> bool {
        if !duration.is_finite() || duration < 0.0 {
            return false;
        }

        if !self.zero_duration_operations && duration == 0.0 {
            return false;
        }

        match self.max_operation_duration {
            Some(maximum) => duration <= maximum,
            None => true,
        }
    }

    /// Returns whether parallel execution is permitted.
    #[must_use]
    pub const fn allows_parallel_operations(&self) -> bool {
        self.parallel_operations
    }
}

// =============================================================================
// Measurement and dynamic-circuit constraints
// =============================================================================

/// Constraints governing measurement and dynamic execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicCircuitConstraints {
    /// Whether measurement is permitted before circuit termination.
    pub mid_circuit_measurement: bool,

    /// Whether measurement results can control later operations.
    pub measurement_based_control: bool,

    /// Whether classical branching is permitted.
    pub conditional_branches: bool,

    /// Whether switch/case-style classical branching is permitted.
    pub switch_case: bool,

    /// Whether loops are permitted.
    pub loops: bool,

    /// Whether loops may have data-dependent termination.
    pub dynamic_loops: bool,

    /// Whether reset is permitted.
    pub reset: bool,

    /// Whether measurement can be followed by quantum operations on the same
    /// qubit.
    pub post_measurement_quantum_operations: bool,

    /// Whether measurement results can be used by multiple later operations.
    pub measurement_result_reuse: bool,

    /// Whether terminal measurement is required.
    pub terminal_measurement_only: bool,

    /// Whether a circuit may contain no measurement at all.
    pub measurement_optional: bool,

    /// Stable identifiers for target-supported control-flow constructs.
    pub supported_control_flow_operations: Vec<String>,
}

impl Default for DynamicCircuitConstraints {
    fn default() -> Self {
        Self {
            mid_circuit_measurement: false,
            measurement_based_control: false,
            conditional_branches: false,
            switch_case: false,
            loops: false,
            dynamic_loops: false,
            reset: true,
            post_measurement_quantum_operations: false,
            measurement_result_reuse: false,
            terminal_measurement_only: false,
            measurement_optional: true,
            supported_control_flow_operations: Vec::new(),
        }
    }
}

impl DynamicCircuitConstraints {
    /// Adds a supported control-flow operation identifier.
    pub fn add_supported_control_flow(
        &mut self,
        operation: impl Into<String>,
    ) -> ConstraintResult<()> {
        let operation = operation.into();

        validate_identifier(
            "control_flow_operation",
            &operation,
            MAX_CONSTRAINT_ID_LENGTH,
        )?;

        if !self
            .supported_control_flow_operations
            .iter()
            .any(|existing| existing == &operation)
        {
            if self.supported_control_flow_operations.len()
                >= MAX_CONTROL_FLOW_OPERATIONS
            {
                return Err(
                    ConstraintError::TooManyControlFlowOperations {
                        maximum: MAX_CONTROL_FLOW_OPERATIONS,
                        actual: self
                            .supported_control_flow_operations
                            .len()
                            + 1,
                    },
                );
            }

            self.supported_control_flow_operations.push(operation);
            self.supported_control_flow_operations.sort();
        }

        Ok(())
    }

    /// Validates dynamic-circuit constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.measurement_based_control
            && !self.mid_circuit_measurement
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "measurement_based_control requires mid_circuit_measurement"
                        .to_owned(),
            });
        }

        if self.dynamic_loops && !self.loops {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "dynamic_loops requires loops=true".to_owned(),
            });
        }

        if self.post_measurement_quantum_operations
            && !self.mid_circuit_measurement
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "post_measurement_quantum_operations requires mid_circuit_measurement"
                        .to_owned(),
            });
        }

        if self.measurement_result_reuse
            && !self.measurement_based_control
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "measurement_result_reuse requires measurement_based_control"
                        .to_owned(),
            });
        }

        if self.terminal_measurement_only
            && self.post_measurement_quantum_operations
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "terminal_measurement_only conflicts with \
                     post_measurement_quantum_operations"
                        .to_owned(),
            });
        }

        if self.supported_control_flow_operations.len()
            > MAX_CONTROL_FLOW_OPERATIONS
        {
            return Err(
                ConstraintError::TooManyControlFlowOperations {
                    maximum: MAX_CONTROL_FLOW_OPERATIONS,
                    actual: self.supported_control_flow_operations.len(),
                },
            );
        }

        for operation in &self.supported_control_flow_operations {
            validate_identifier(
                "control_flow_operation",
                operation,
                MAX_CONSTRAINT_ID_LENGTH,
            )?;
        }

        Ok(())
    }

    /// Returns whether a requested control-flow capability is supported.
    #[must_use]
    pub fn supports_control_flow(&self, operation: &str) -> bool {
        match operation {
            "if" | "if_else" | "conditional" => {
                self.conditional_branches
            }

            "switch" | "switch_case" => self.switch_case,

            "while" | "loop" => self.loops,

            "dynamic_loop" => self.dynamic_loops,

            other => self
                .supported_control_flow_operations
                .iter()
                .any(|candidate| candidate == other),
        }
    }
}

// =============================================================================
// Measurement constraints
// =============================================================================

/// Measurement-specific target constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasurementConstraints {
    /// Whether measurement is supported at all.
    pub supported: bool,

    /// Whether mid-circuit measurement is supported.
    pub mid_circuit: bool,

    /// Whether terminal measurement is supported.
    pub terminal: bool,

    /// Whether measurement results may be reused.
    pub result_reuse: bool,

    /// Whether measurement ordering is semantically observable.
    pub ordering_semantic: bool,

    /// Whether measurement basis changes are supported downstream.
    pub arbitrary_basis: bool,

    /// Whether destructive measurement is supported.
    pub destructive: bool,

    /// Whether repeated measurement of one qubit is supported.
    pub repeated_measurement: bool,
}

impl Default for MeasurementConstraints {
    fn default() -> Self {
        Self {
            supported: true,
            mid_circuit: false,
            terminal: true,
            result_reuse: false,
            ordering_semantic: true,
            arbitrary_basis: true,
            destructive: false,
            repeated_measurement: false,
        }
    }
}

impl MeasurementConstraints {
    /// Validates measurement constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.mid_circuit && !self.supported {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "mid_circuit measurement requires measurement support"
                        .to_owned(),
            });
        }

        if self.terminal && !self.supported {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "terminal measurement requires measurement support"
                        .to_owned(),
            });
        }

        if self.result_reuse && !self.mid_circuit {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "measurement result reuse requires mid-circuit measurement"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Reset constraints
// =============================================================================

/// Reset-specific target constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResetConstraints {
    /// Whether reset is supported.
    pub supported: bool,

    /// Whether reset may occur after measurement.
    pub after_measurement: bool,

    /// Whether reset may occur in the middle of a circuit.
    pub mid_circuit: bool,

    /// Whether reset may be optimized away when provably redundant.
    pub redundant_reset_elimination: bool,
}

impl Default for ResetConstraints {
    fn default() -> Self {
        Self {
            supported: true,
            after_measurement: true,
            mid_circuit: true,
            redundant_reset_elimination: true,
        }
    }
}

impl ResetConstraints {
    /// Validates reset constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.after_measurement && !self.supported {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "after_measurement reset requires reset support".to_owned(),
            });
        }

        if self.mid_circuit && !self.supported {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "mid_circuit reset requires reset support".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Barrier and ordering constraints
// =============================================================================

/// Ordering and barrier semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderingConstraints {
    /// Whether barriers are semantically meaningful.
    pub barriers_semantic: bool,

    /// Whether barriers prevent gate motion across them.
    pub barriers_block_reordering: bool,

    /// Whether operation order is semantically observable for otherwise
    /// commuting operations.
    pub operation_order_semantic: bool,

    /// Whether compiler transformations may reorder independent operations.
    pub independent_reordering_allowed: bool,

    /// Whether commutation-based transformations are allowed.
    pub commutation_optimization_allowed: bool,

    /// Whether measurement boundaries prevent operation motion.
    pub measurement_boundaries: bool,

    /// Whether reset boundaries prevent operation motion.
    pub reset_boundaries: bool,
}

impl Default for OrderingConstraints {
    fn default() -> Self {
        Self {
            barriers_semantic: true,
            barriers_block_reordering: true,
            operation_order_semantic: false,
            independent_reordering_allowed: true,
            commutation_optimization_allowed: true,
            measurement_boundaries: true,
            reset_boundaries: true,
        }
    }
}

impl OrderingConstraints {
    /// Validates ordering constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.operation_order_semantic
            && self.independent_reordering_allowed
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "operation_order_semantic conflicts with \
                     independent_reordering_allowed"
                        .to_owned(),
            });
        }

        if !self.independent_reordering_allowed
            && self.commutation_optimization_allowed
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "commutation optimization requires independent \
                     reordering to be allowed"
                        .to_owned(),
            });
        }

        if !self.barriers_semantic
            && self.barriers_block_reordering
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "barriers cannot block reordering when they are not \
                     semantically meaningful"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Global-phase constraints
// =============================================================================

/// Global-phase semantics relevant to optimization equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalPhaseConstraints {
    /// Whether downstream execution ignores global phase.
    pub global_phase_ignored: bool,

    /// Whether optimization may introduce a global phase.
    pub optimization_may_change_global_phase: bool,

    /// Whether exact unitary equivalence is required.
    pub exact_unitary_required: bool,
}

impl Default for GlobalPhaseConstraints {
    fn default() -> Self {
        Self {
            global_phase_ignored: true,
            optimization_may_change_global_phase: true,
            exact_unitary_required: false,
        }
    }
}

impl GlobalPhaseConstraints {
    /// Validates global-phase semantics.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.exact_unitary_required
            && self.optimization_may_change_global_phase
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "exact_unitary_required conflicts with \
                     optimization_may_change_global_phase"
                        .to_owned(),
            });
        }

        if !self.global_phase_ignored
            && self.optimization_may_change_global_phase
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "global phase cannot be changed when it is semantically \
                     observable"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Approximation constraints
// =============================================================================

/// Constraints for approximate optimization and synthesis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApproximationConstraints {
    /// Whether approximate transformations are allowed.
    pub allowed: bool,

    /// Maximum accepted absolute approximation tolerance.
    pub absolute_tolerance: Option<f64>,

    /// Maximum accepted relative approximation tolerance.
    pub relative_tolerance: Option<f64>,

    /// Whether approximation error must be certified.
    pub require_certificate: bool,

    /// Whether approximation is permitted during ordinary optimization.
    pub optimization_allowed: bool,

    /// Whether approximation is permitted during synthesis.
    pub synthesis_allowed: bool,

    /// Whether approximation is permitted for fault-tolerant synthesis.
    pub fault_tolerant_allowed: bool,
}

impl Default for ApproximationConstraints {
    fn default() -> Self {
        Self {
            allowed: false,
            absolute_tolerance: Some(0.0),
            relative_tolerance: Some(0.0),
            require_certificate: true,
            optimization_allowed: false,
            synthesis_allowed: false,
            fault_tolerant_allowed: false,
        }
    }
}

impl ApproximationConstraints {
    /// Validates approximation policy.
    pub fn validate(&self) -> ConstraintResult<()> {
        validate_optional_non_negative_finite(
            self.absolute_tolerance,
            "absolute_tolerance",
        )?;

        validate_optional_non_negative_finite(
            self.relative_tolerance,
            "relative_tolerance",
        )?;

        if !self.allowed
            && (self.optimization_allowed
                || self.synthesis_allowed
                || self.fault_tolerant_allowed)
        {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "approximate sub-policies cannot be enabled when \
                     approximation is disabled"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Returns whether an approximation error is accepted.
    #[must_use]
    pub fn accepts_error(&self, error: f64) -> bool {
        if !error.is_finite() || error < 0.0 {
            return false;
        }

        if !self.allowed {
            return error == 0.0;
        }

        match self.absolute_tolerance {
            Some(maximum) => error <= maximum,
            None => true,
        }
    }
}

// =============================================================================
// Fault-tolerant constraints
// =============================================================================

/// Fault-tolerant resource constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultTolerantConstraints {
    /// Whether fault-tolerant optimization is applicable.
    pub enabled: bool,

    /// Maximum T count.
    pub max_t_count: ResourceBound,

    /// Maximum T depth.
    pub max_t_depth: ResourceBound,

    /// Maximum magic-state demand.
    pub max_magic_states: ResourceBound,

    /// Maximum non-Clifford operation count.
    pub max_non_clifford_operations: ResourceBound,

    /// Whether Clifford+T structure must be preserved.
    pub clifford_t_required: bool,

    /// Whether logical operations may remain at the end of optimization.
    pub logical_operations_allowed: bool,
}

impl Default for FaultTolerantConstraints {
    fn default() -> Self {
        Self {
            enabled: false,
            max_t_count: ResourceBound::unbounded(),
            max_t_depth: ResourceBound::unbounded(),
            max_magic_states: ResourceBound::unbounded(),
            max_non_clifford_operations: ResourceBound::unbounded(),
            clifford_t_required: false,
            logical_operations_allowed: false,
        }
    }
}

impl FaultTolerantConstraints {
    /// Validates fault-tolerant constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        self.max_t_count.validate()?;
        self.max_t_depth.validate()?;
        self.max_magic_states.validate()?;
        self.max_non_clifford_operations.validate()?;

        if self.clifford_t_required && !self.enabled {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "clifford_t_required requires fault-tolerant optimization"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Control constraints
// =============================================================================

/// Classical-control and control-flow constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlConstraints {
    /// Whether classically controlled quantum operations are allowed.
    pub classical_control: bool,

    /// Whether controls may depend on measurement results.
    pub measurement_control: bool,

    /// Whether nested classical control is allowed.
    pub nested_control: bool,

    /// Maximum control nesting depth.
    pub max_control_depth: ResourceBound,

    /// Maximum number of classical control dependencies.
    pub max_control_dependencies: ResourceBound,

    /// Whether quantum-controlled operations are allowed.
    pub quantum_control: bool,

    /// Maximum number of quantum controls.
    pub max_quantum_controls: Option<usize>,
}

impl Default for ControlConstraints {
    fn default() -> Self {
        Self {
            classical_control: false,
            measurement_control: false,
            nested_control: false,
            max_control_depth: ResourceBound::unbounded(),
            max_control_dependencies: ResourceBound::unbounded(),
            quantum_control: true,
            max_quantum_controls: None,
        }
    }
}

impl ControlConstraints {
    /// Validates control constraints.
    pub fn validate(&self) -> ConstraintResult<()> {
        self.max_control_depth.validate()?;
        self.max_control_dependencies.validate()?;

        if self.measurement_control && !self.classical_control {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "measurement_control requires classical_control".to_owned(),
            });
        }

        if self.max_quantum_controls == Some(0) && self.quantum_control {
            return Err(ConstraintError::InvalidConstraint {
                message:
                    "quantum_control cannot be enabled with \
                     max_quantum_controls=0"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Constraint policy
// =============================================================================

/// Policy used when a constraint is encountered during optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintPolicy {
    /// Reject a circuit that violates the constraint.
    Reject,

    /// Allow the circuit to remain temporarily invalid while a later
    /// transformation may repair it.
    Defer,

    /// Treat the constraint as an optimization preference.
    Prefer,

    /// Record the violation but do not reject the result.
    Warn,
}

impl Default for ConstraintPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

// =============================================================================
// Custom constraints
// =============================================================================

/// A target-specific extensibility point.
///
/// Custom constraints are intentionally declarative. This file cannot execute
/// arbitrary user callbacks because doing so would make target validation
/// non-deterministic and would prevent stable serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomConstraint {
    /// Stable custom constraint identifier.
    pub id: ConstraintId,

    /// Human-readable description.
    pub description: String,

    /// Scope of the constraint.
    pub scope: ConstraintScope,

    /// Whether the constraint is hard or soft.
    pub strength: ConstraintStrength,

    /// Policy to apply when the constraint is violated.
    pub policy: ConstraintPolicy,

    /// Optional numeric threshold.
    pub threshold: Option<f64>,

    /// Optional string value interpreted by the owning subsystem.
    pub value: Option<String>,

    /// Arbitrary deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl CustomConstraint {
    /// Creates a custom constraint.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
    ) -> ConstraintResult<Self> {
        Ok(Self {
            id: ConstraintId::new(id)?,
            description: description.into(),
            scope: ConstraintScope::Target,
            strength: ConstraintStrength::Hard,
            policy: ConstraintPolicy::Reject,
            threshold: None,
            value: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Sets the constraint scope.
    #[must_use]
    pub const fn with_scope(
        mut self,
        scope: ConstraintScope,
    ) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the constraint strength.
    #[must_use]
    pub const fn with_strength(
        mut self,
        strength: ConstraintStrength,
    ) -> Self {
        self.strength = strength;
        self
    }

    /// Sets the violation policy.
    #[must_use]
    pub const fn with_policy(
        mut self,
        policy: ConstraintPolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Sets an optional threshold.
    pub fn with_threshold(
        mut self,
        threshold: f64,
    ) -> ConstraintResult<Self> {
        validate_non_negative_finite(threshold, "custom_constraint_threshold")?;
        self.threshold = Some(threshold);
        Ok(self)
    }

    /// Sets an optional custom value.
    #[must_use]
    pub fn with_value(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Adds metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ConstraintResult<()> {
        validate_metadata_key(&key.into())?;
        let key = key.into();
        let value = value.into();

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(ConstraintError::InvalidMetadataValue { key });
        }

        self.metadata.insert(key, value);

        if self.metadata.len() > MAX_METADATA_ENTRIES {
            return Err(ConstraintError::TooMuchMetadata {
                maximum: MAX_METADATA_ENTRIES,
                actual: self.metadata.len(),
            });
        }

        Ok(())
    }

    /// Validates the custom constraint.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.description.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(ConstraintError::InvalidMetadataValue {
                key: self.id.as_str().to_owned(),
            });
        }

        if let Some(threshold) = self.threshold {
            validate_non_negative_finite(
                threshold,
                "custom_constraint_threshold",
            )?;
        }

        for (key, value) in &self.metadata {
            validate_metadata_key(key)?;

            if value.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(ConstraintError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Complete target constraints
// =============================================================================

/// Complete target-wide constraint contract.
///
/// This is the canonical constraint object that `targets::target` should
/// embed.
///
/// It deliberately contains no physical topology.
///
/// An absent resource limit means "unbounded at this target layer".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetConstraints {
    /// Stable schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u32,

    /// Hard/soft resource limits.
    pub resources: ResourceConstraints,

    /// Operation-shape limits.
    pub operations: OperationConstraints,

    /// Parameter constraints.
    pub parameters: ParameterConstraints,

    /// Timing constraints.
    pub timing: TimingConstraints,

    /// Dynamic-circuit constraints.
    pub dynamic_circuits: DynamicCircuitConstraints,

    /// Measurement constraints.
    pub measurement: MeasurementConstraints,

    /// Reset constraints.
    pub reset: ResetConstraints,

    /// Ordering and barrier constraints.
    pub ordering: OrderingConstraints,

    /// Global-phase semantics.
    pub global_phase: GlobalPhaseConstraints,

    /// Approximation policy.
    pub approximation: ApproximationConstraints,

    /// Fault-tolerant resource constraints.
    pub fault_tolerance: FaultTolerantConstraints,

    /// Classical and quantum control constraints.
    pub control: ControlConstraints,

    /// Default policy for hard/soft constraint handling.
    pub default_policy: ConstraintPolicy,

    /// Custom target-specific constraints.
    pub custom: BTreeMap<ConstraintId, CustomConstraint>,

    /// Deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl Default for TargetConstraints {
    fn default() -> Self {
        Self::unrestricted()
    }
}

impl TargetConstraints {
    /// Creates a target with no target-level resource ceiling.
    ///
    /// This is the correct default for generic/unbounded targets.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self {
            schema_id: TARGET_CONSTRAINTS_SCHEMA_ID.to_owned(),
            schema_version: TARGET_CONSTRAINTS_SCHEMA_VERSION,
            resources: ResourceConstraints::default(),
            operations: OperationConstraints::default(),
            parameters: ParameterConstraints::default(),
            timing: TimingConstraints::default(),
            dynamic_circuits: DynamicCircuitConstraints::default(),
            measurement: MeasurementConstraints::default(),
            reset: ResetConstraints::default(),
            ordering: OrderingConstraints::default(),
            global_phase: GlobalPhaseConstraints::default(),
            approximation: ApproximationConstraints::default(),
            fault_tolerance: FaultTolerantConstraints::default(),
            control: ControlConstraints::default(),
            default_policy: ConstraintPolicy::Reject,
            custom: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates conservative target constraints.
    ///
    /// This is useful for a target where unsupported features should be
    /// rejected unless explicitly enabled.
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            schema_id: TARGET_CONSTRAINTS_SCHEMA_ID.to_owned(),
            schema_version: TARGET_CONSTRAINTS_SCHEMA_VERSION,
            resources: ResourceConstraints::default(),
            operations: OperationConstraints {
                max_arity: Some(3),
                max_parameters_per_operation: Some(64),
                max_controls: Some(2),
                max_targets: Some(3),
                variable_arity: false,
                zero_qubit_operations: false,
                one_qubit_operations: true,
                two_qubit_operations: true,
                three_qubit_operations: true,
                multi_qubit_operations: false,
            },
            parameters: ParameterConstraints {
                symbolic_parameters: false,
                finite_parameters: true,
                arbitrary_parameters: true,
                parameter_range: None,
                max_symbolic_parameters: ResourceBound::bounded(0),
                max_distinct_symbols: ResourceBound::bounded(0),
                max_expression_depth: ResourceBound::bounded(0),
                max_expression_nodes: ResourceBound::bounded(0),
                unresolved_symbols_allowed: false,
            },
            timing: TimingConstraints::default(),
            dynamic_circuits: DynamicCircuitConstraints::default(),
            measurement: MeasurementConstraints::default(),
            reset: ResetConstraints::default(),
            ordering: OrderingConstraints::default(),
            global_phase: GlobalPhaseConstraints::default(),
            approximation: ApproximationConstraints::default(),
            fault_tolerance: FaultTolerantConstraints::default(),
            control: ControlConstraints::default(),
            default_policy: ConstraintPolicy::Reject,
            custom: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a logical fault-tolerant constraint profile.
    #[must_use]
    pub fn fault_tolerant() -> Self {
        let mut constraints = Self::unrestricted();

        constraints.fault_tolerance.enabled = true;
        constraints.fault_tolerance.clifford_t_required = true;
        constraints.fault_tolerance.logical_operations_allowed = true;

        constraints
    }

    /// Returns whether the target is effectively unbounded with respect to
    /// target-level circuit size.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.resources.logical_qubits.is_unbounded()
            && self.resources.ancillas.is_unbounded()
            && self.resources.operations.is_unbounded()
            && self.resources.depth.is_unbounded()
            && self.resources.two_qubit_depth.is_unbounded()
            && self.resources.two_qubit_operations.is_unbounded()
    }

    /// Adds a custom constraint.
    pub fn add_custom(
        &mut self,
        constraint: CustomConstraint,
    ) -> ConstraintResult<()> {
        constraint.validate()?;

        let id = constraint.id.clone();

        if !self.custom.contains_key(&id)
            && self.custom.len() >= MAX_CUSTOM_CONSTRAINTS
        {
            return Err(
                ConstraintError::TooManyCustomConstraints {
                    maximum: MAX_CUSTOM_CONSTRAINTS,
                    actual: self.custom.len() + 1,
                },
            );
        }

        self.custom.insert(id, constraint);
        Ok(())
    }

    /// Adds target metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ConstraintResult<()> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(ConstraintError::InvalidMetadataValue { key });
        }

        self.metadata.insert(key, value);

        if self.metadata.len() > MAX_METADATA_ENTRIES {
            return Err(ConstraintError::TooMuchMetadata {
                maximum: MAX_METADATA_ENTRIES,
                actual: self.metadata.len(),
            });
        }

        Ok(())
    }

    /// Validates the complete constraint contract.
    pub fn validate(&self) -> ConstraintResult<()> {
        if self.schema_id != TARGET_CONSTRAINTS_SCHEMA_ID {
            return Err(ConstraintError::InvalidConstraint {
                message: format!(
                    "unexpected schema id `{}`",
                    self.schema_id
                ),
            });
        }

        if self.schema_version != TARGET_CONSTRAINTS_SCHEMA_VERSION {
            return Err(ConstraintError::InvalidConstraint {
                message: format!(
                    "unsupported schema version {}",
                    self.schema_version
                ),
            });
        }

        self.resources.validate()?;
        self.operations.validate()?;
        self.parameters.validate()?;
        self.timing.validate()?;
        self.dynamic_circuits.validate()?;
        self.measurement.validate()?;
        self.reset.validate()?;
        self.ordering.validate()?;
        self.global_phase.validate()?;
        self.approximation.validate()?;
        self.fault_tolerance.validate()?;
        self.control.validate()?;

        for constraint in self.custom.values() {
            constraint.validate()?;
        }

        if self.custom.len() > MAX_CUSTOM_CONSTRAINTS {
            return Err(
                ConstraintError::TooManyCustomConstraints {
                    maximum: MAX_CUSTOM_CONSTRAINTS,
                    actual: self.custom.len(),
                },
            );
        }

        for (key, value) in &self.metadata {
            validate_metadata_key(key)?;

            if value.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(ConstraintError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        if self.metadata.len() > MAX_METADATA_ENTRIES {
            return Err(ConstraintError::TooMuchMetadata {
                maximum: MAX_METADATA_ENTRIES,
                actual: self.metadata.len(),
            });
        }

        Ok(())
    }

    /// Returns whether a logical-qubit count is accepted.
    #[must_use]
    pub const fn accepts_logical_qubits(&self, count: u128) -> bool {
        self.resources.logical_qubits.accepts(count)
    }

    /// Returns whether an ancilla count is accepted.
    #[must_use]
    pub const fn accepts_ancillas(&self, count: u128) -> bool {
        self.resources.ancillas.accepts(count)
    }

    /// Returns whether an operation count is accepted.
    #[must_use]
    pub const fn accepts_operations(&self, count: u128) -> bool {
        self.resources.operations.accepts(count)
    }

    /// Returns whether a depth value is accepted.
    #[must_use]
    pub const fn accepts_depth(&self, depth: u128) -> bool {
        self.resources.depth.accepts(depth)
    }

    /// Returns whether an operation arity is accepted.
    #[must_use]
    pub fn accepts_operation_arity(&self, arity: usize) -> bool {
        self.operations.accepts_arity(arity)
    }

    /// Returns whether a parameter count is accepted.
    #[must_use]
    pub fn accepts_parameter_count(&self, count: usize) -> bool {
        self.operations.accepts_parameter_count(count)
    }

    /// Returns whether symbolic parameters are allowed.
    #[must_use]
    pub const fn allows_symbolic_parameters(&self) -> bool {
        self.parameters.symbolic_parameters
    }

    /// Returns whether mid-circuit measurement is allowed.
    #[must_use]
    pub const fn allows_mid_circuit_measurement(&self) -> bool {
        self.dynamic_circuits.mid_circuit_measurement
            && self.measurement.mid_circuit
    }

    /// Returns whether classical measurement control is allowed.
    #[must_use]
    pub const fn allows_measurement_control(&self) -> bool {
        self.dynamic_circuits.measurement_based_control
            && self.control.measurement_control
    }

    /// Returns whether loops are allowed.
    #[must_use]
    pub const fn allows_loops(&self) -> bool {
        self.dynamic_circuits.loops
    }

    /// Returns whether reset is allowed.
    #[must_use]
    pub const fn allows_reset(&self) -> bool {
        self.reset.supported
    }

    /// Returns whether barriers are semantically meaningful.
    #[must_use]
    pub const fn barriers_are_semantic(&self) -> bool {
        self.ordering.barriers_semantic
    }

    /// Returns whether global phase can be ignored.
    #[must_use]
    pub const fn ignores_global_phase(&self) -> bool {
        self.global_phase.global_phase_ignored
    }

    /// Returns whether approximate optimization is allowed.
    #[must_use]
    pub const fn allows_approximation(&self) -> bool {
        self.approximation.allowed
            && self.approximation.optimization_allowed
    }

    /// Returns whether fault-tolerant optimization is enabled.
    #[must_use]
    pub const fn fault_tolerant_enabled(&self) -> bool {
        self.fault_tolerance.enabled
    }

    /// Returns whether a circuit summary satisfies target resource bounds.
    #[must_use]
    pub fn accepts_resource_summary(
        &self,
        summary: &ResourceSummary,
    ) -> bool {
        self.resources.logical_qubits.accepts(summary.logical_qubits)
            && self.resources.ancillas.accepts(summary.ancillas)
            && self.resources.classical_bits.accepts(summary.classical_bits)
            && self.resources.operations.accepts(summary.operations)
            && self.resources.depth.accepts(summary.depth)
            && self
                .resources
                .two_qubit_depth
                .accepts(summary.two_qubit_depth)
            && self
                .resources
                .single_qubit_operations
                .accepts(summary.single_qubit_operations)
            && self
                .resources
                .two_qubit_operations
                .accepts(summary.two_qubit_operations)
            && self
                .resources
                .multi_qubit_operations
                .accepts(summary.multi_qubit_operations)
            && self.resources.measurements.accepts(summary.measurements)
            && self.resources.resets.accepts(summary.resets)
            && self.resources.barriers.accepts(summary.barriers)
            && self.resources.delays.accepts(summary.delays)
            && self
                .resources
                .control_flow_operations
                .accepts(summary.control_flow_operations)
            && self
                .resources
                .classical_control_operations
                .accepts(summary.classical_control_operations)
            && self.resources.t_count.accepts(summary.t_count)
            && self.resources.t_depth.accepts(summary.t_depth)
            && self
                .resources
                .non_clifford_operations
                .accepts(summary.non_clifford_operations)
            && match self.resources.duration {
                Some(maximum) => summary.duration.is_finite()
                    && summary.duration >= 0.0
                    && summary.duration <= maximum,
                None => true,
            }
    }
}

// =============================================================================
// Resource summary
// =============================================================================

/// Backend-independent summary used to test a circuit against target
/// constraints.
///
/// This is deliberately not a circuit representation and must not be used as
/// one.
///
/// Analysis modules should populate this structure from canonical IR.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResourceSummary {
    /// Number of logical qubits.
    pub logical_qubits: u128,

    /// Number of ancillas.
    pub ancillas: u128,

    /// Number of classical bits.
    pub classical_bits: u128,

    /// Total operation count.
    pub operations: u128,

    /// Logical circuit depth.
    pub depth: u128,

    /// Two-qubit depth.
    pub two_qubit_depth: u128,

    /// Single-qubit operation count.
    pub single_qubit_operations: u128,

    /// Two-qubit operation count.
    pub two_qubit_operations: u128,

    /// Multi-qubit operation count.
    pub multi_qubit_operations: u128,

    /// Measurement count.
    pub measurements: u128,

    /// Reset count.
    pub resets: u128,

    /// Barrier count.
    pub barriers: u128,

    /// Delay count.
    pub delays: u128,

    /// Control-flow operation count.
    pub control_flow_operations: u128,

    /// Classically controlled operation count.
    pub classical_control_operations: u128,

    /// T count.
    pub t_count: u128,

    /// T depth.
    pub t_depth: u128,

    /// Non-Clifford operation count.
    pub non_clifford_operations: u128,

    /// Estimated duration in target-defined units.
    pub duration: f64,
}

impl Default for ResourceSummary {
    fn default() -> Self {
        Self {
            logical_qubits: 0,
            ancillas: 0,
            classical_bits: 0,
            operations: 0,
            depth: 0,
            two_qubit_depth: 0,
            single_qubit_operations: 0,
            two_qubit_operations: 0,
            multi_qubit_operations: 0,
            measurements: 0,
            resets: 0,
            barriers: 0,
            delays: 0,
            control_flow_operations: 0,
            classical_control_operations: 0,
            t_count: 0,
            t_depth: 0,
            non_clifford_operations: 0,
            duration: 0.0,
        }
    }
}

impl ResourceSummary {
    /// Creates an empty summary.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            logical_qubits: 0,
            ancillas: 0,
            classical_bits: 0,
            operations: 0,
            depth: 0,
            two_qubit_depth: 0,
            single_qubit_operations: 0,
            two_qubit_operations: 0,
            multi_qubit_operations: 0,
            measurements: 0,
            resets: 0,
            barriers: 0,
            delays: 0,
            control_flow_operations: 0,
            classical_control_operations: 0,
            t_count: 0,
            t_depth: 0,
            non_clifford_operations: 0,
            duration: 0.0,
        }
    }

    /// Validates the summary.
    pub fn validate(&self) -> ConstraintResult<()> {
        if !self.duration.is_finite() || self.duration < 0.0 {
            return Err(ConstraintError::InvalidNumericValue {
                field: "resource_summary.duration",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for complete target constraints.
#[derive(Debug, Clone)]
pub struct TargetConstraintsBuilder {
    constraints: TargetConstraints,
}

impl Default for TargetConstraintsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetConstraintsBuilder {
    /// Creates an unrestricted target-constraint builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            constraints: TargetConstraints::unrestricted(),
        }
    }

    /// Creates a conservative target-constraint builder.
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            constraints: TargetConstraints::conservative(),
        }
    }

    /// Creates a fault-tolerant target-constraint builder.
    #[must_use]
    pub fn fault_tolerant() -> Self {
        Self {
            constraints: TargetConstraints::fault_tolerant(),
        }
    }

    /// Sets the resource constraints.
    #[must_use]
    pub fn resources(
        mut self,
        resources: ResourceConstraints,
    ) -> Self {
        self.constraints.resources = resources;
        self
    }

    /// Sets operation constraints.
    #[must_use]
    pub fn operations(
        mut self,
        operations: OperationConstraints,
    ) -> Self {
        self.constraints.operations = operations;
        self
    }

    /// Sets parameter constraints.
    #[must_use]
    pub fn parameters(
        mut self,
        parameters: ParameterConstraints,
    ) -> Self {
        self.constraints.parameters = parameters;
        self
    }

    /// Sets timing constraints.
    #[must_use]
    pub fn timing(
        mut self,
        timing: TimingConstraints,
    ) -> Self {
        self.constraints.timing = timing;
        self
    }

    /// Sets dynamic-circuit constraints.
    #[must_use]
    pub fn dynamic_circuits(
        mut self,
        constraints: DynamicCircuitConstraints,
    ) -> Self {
        self.constraints.dynamic_circuits = constraints;
        self
    }

    /// Sets measurement constraints.
    #[must_use]
    pub fn measurement(
        mut self,
        measurement: MeasurementConstraints,
    ) -> Self {
        self.constraints.measurement = measurement;
        self
    }

    /// Sets reset constraints.
    #[must_use]
    pub fn reset(
        mut self,
        reset: ResetConstraints,
    ) -> Self {
        self.constraints.reset = reset;
        self
    }

    /// Sets ordering constraints.
    #[must_use]
    pub fn ordering(
        mut self,
        ordering: OrderingConstraints,
    ) -> Self {
        self.constraints.ordering = ordering;
        self
    }

    /// Sets global-phase constraints.
    #[must_use]
    pub fn global_phase(
        mut self,
        global_phase: GlobalPhaseConstraints,
    ) -> Self {
        self.constraints.global_phase = global_phase;
        self
    }

    /// Sets approximation constraints.
    #[must_use]
    pub fn approximation(
        mut self,
        approximation: ApproximationConstraints,
    ) -> Self {
        self.constraints.approximation = approximation;
        self
    }

    /// Sets fault-tolerant constraints.
    #[must_use]
    pub fn fault_tolerance(
        mut self,
        fault_tolerance: FaultTolerantConstraints,
    ) -> Self {
        self.constraints.fault_tolerance = fault_tolerance;
        self
    }

    /// Sets control constraints.
    #[must_use]
    pub fn control(
        mut self,
        control: ControlConstraints,
    ) -> Self {
        self.constraints.control = control;
        self
    }

    /// Sets the default policy.
    #[must_use]
    pub const fn default_policy(
        mut self,
        policy: ConstraintPolicy,
    ) -> Self {
        self.constraints.default_policy = policy;
        self
    }

    /// Adds a custom constraint.
    pub fn add_custom(
        mut self,
        constraint: CustomConstraint,
    ) -> ConstraintResult<Self> {
        self.constraints.add_custom(constraint)?;
        Ok(self)
    }

    /// Adds metadata.
    pub fn add_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ConstraintResult<Self> {
        self.constraints.add_metadata(key, value)?;
        Ok(self)
    }

    /// Validates and returns the final constraint object.
    pub fn build(self) -> ConstraintResult<TargetConstraints> {
        self.constraints.validate()?;
        Ok(self.constraints)
    }
}

// =============================================================================
// Utility functions
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> ConstraintResult<()> {
    if value.is_empty() {
        return Err(ConstraintError::EmptyIdentifier { field });
    }

    if value.len() > maximum {
        return Err(ConstraintError::IdentifierTooLong {
            field,
            maximum,
            actual: value.len(),
        });
    }

    if !is_valid_identifier(value) {
        return Err(ConstraintError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn is_valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return false;
    };

    if !(first.is_ascii_alphanumeric() || first == '_' || first == '.') {
        return false;
    }

    characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | '.' | '-' | ':')
    })
}

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> ConstraintResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(ConstraintError::InvalidNumericValue { field });
    }

    Ok(())
}

fn validate_optional_non_negative_finite(
    value: Option<f64>,
    field: &'static str,
) -> ConstraintResult<()> {
    if let Some(value) = value {
        validate_non_negative_finite(value, field)?;
    }

    Ok(())
}

fn validate_metadata_key(key: &str) -> ConstraintResult<()> {
    validate_identifier(
        "metadata_key",
        key,
        MAX_METADATA_KEY_LENGTH,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrestricted_constraints_are_valid() {
        let constraints = TargetConstraints::unrestricted();

        assert!(constraints.validate().is_ok());
        assert!(constraints.is_unbounded());
        assert!(constraints.accepts_logical_qubits(u128::MAX));
        assert!(constraints.accepts_operations(u128::MAX));
        assert!(constraints.accepts_depth(u128::MAX));
    }

    #[test]
    fn resource_bound_supports_unbounded_values() {
        let bound = ResourceBound::unbounded();

        assert!(bound.accepts(0));
        assert!(bound.accepts(u128::MAX));
        assert!(bound.is_unbounded());
    }

    #[test]
    fn bounded_resource_rejects_values_above_limit() {
        let bound = ResourceBound::bounded(10);

        assert!(bound.accepts(0));
        assert!(bound.accepts(10));
        assert!(!bound.accepts(11));
    }

    #[test]
    fn numeric_range_rejects_invalid_range() {
        let result = NumericRange::new(2.0, 1.0);

        assert!(matches!(
            result,
            Err(ConstraintError::InvalidRange {
                field: "numeric_range"
            })
        ));
    }

    #[test]
    fn numeric_range_accepts_boundaries() {
        let range = NumericRange::new(0.0, 1.0)
            .expect("valid range");

        assert!(range.accepts(0.0));
        assert!(range.accepts(0.5));
        assert!(range.accepts(1.0));
        assert!(!range.accepts(-0.1));
        assert!(!range.accepts(1.1));
    }

    #[test]
    fn operation_constraints_validate_arity() {
        let constraints = OperationConstraints {
            max_arity: Some(2),
            max_parameters_per_operation: Some(32),
            max_controls: Some(1),
            max_targets: Some(2),
            variable_arity: false,
            zero_qubit_operations: false,
            one_qubit_operations: true,
            two_qubit_operations: true,
            three_qubit_operations: false,
            multi_qubit_operations: false,
        };

        assert!(constraints.validate().is_ok());
        assert!(constraints.accepts_arity(1));
        assert!(constraints.accepts_arity(2));
        assert!(!constraints.accepts_arity(3));
    }

    #[test]
    fn symbolic_parameters_can_be_unbounded() {
        let constraints = ParameterConstraints::default();

        assert!(constraints.symbolic_parameters);
        assert!(constraints.accepts_symbolic_parameter_count(u128::MAX));
        assert!(constraints.accepts_distinct_symbol_count(u128::MAX));
    }

    #[test]
    fn timing_rejects_negative_duration() {
        let timing = TimingConstraints {
            max_circuit_duration: Some(-1.0),
            ..TimingConstraints::default()
        };

        assert!(timing.validate().is_err());
    }

    #[test]
    fn dynamic_control_dependencies_are_validated() {
        let constraints = DynamicCircuitConstraints {
            measurement_based_control: true,
            mid_circuit_measurement: false,
            ..DynamicCircuitConstraints::default()
        };

        assert!(constraints.validate().is_err());
    }

    #[test]
    fn terminal_measurement_conflicts_with_post_measurement_execution() {
        let constraints = DynamicCircuitConstraints {
            terminal_measurement_only: true,
            post_measurement_quantum_operations: true,
            mid_circuit_measurement: true,
            ..DynamicCircuitConstraints::default()
        };

        assert!(constraints.validate().is_err());
    }

    #[test]
    fn global_phase_rules_are_consistent() {
        let constraints = GlobalPhaseConstraints {
            global_phase_ignored: false,
            optimization_may_change_global_phase: true,
            exact_unitary_required: false,
        };

        assert!(constraints.validate().is_err());
    }

    #[test]
    fn approximation_is_disabled_by_default() {
        let constraints = ApproximationConstraints::default();

        assert!(!constraints.allowed);
        assert!(!constraints.optimization_allowed);
        assert!(constraints.accepts_error(0.0));
        assert!(!constraints.accepts_error(0.0001));
    }

    #[test]
    fn fault_tolerant_profile_is_valid() {
        let constraints = TargetConstraints::fault_tolerant();

        assert!(constraints.validate().is_ok());
        assert!(constraints.fault_tolerant_enabled());
    }

    #[test]
    fn custom_constraint_can_be_added() {
        let constraint = CustomConstraint::new(
            "custom.example",
            "Example target-specific constraint",
        )
        .expect("valid custom constraint");

        let mut constraints = TargetConstraints::unrestricted();

        constraints
            .add_custom(constraint)
            .expect("custom constraint should be accepted");

        assert_eq!(constraints.custom.len(), 1);
    }

    #[test]
    fn resource_summary_can_be_checked() {
        let mut constraints = TargetConstraints::unrestricted();

        constraints.resources.logical_qubits =
            ResourceBound::bounded(8);

        constraints.resources.operations =
            ResourceBound::bounded(100);

        let mut summary = ResourceSummary::empty();
        summary.logical_qubits = 8;
        summary.operations = 100;

        assert!(constraints.accepts_resource_summary(&summary));

        summary.logical_qubits = 9;

        assert!(!constraints.accepts_resource_summary(&summary));
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut constraints = TargetConstraints::unrestricted();

        constraints
            .add_metadata("target.kind", "generic")
            .expect("valid metadata");

        constraints
            .add_metadata("target.version", "1")
            .expect("valid metadata");

        assert_eq!(
            constraints.metadata.get("target.kind"),
            Some(&"generic".to_owned())
        );
    }

    #[test]
    fn builder_produces_valid_constraints() {
        let constraints = TargetConstraintsBuilder::new()
            .add_metadata("profile", "generic")
            .expect("metadata")
            .build()
            .expect("valid constraints");

        assert_eq!(
            constraints.schema_id,
            TARGET_CONSTRAINTS_SCHEMA_ID
        );
        assert_eq!(
            constraints.schema_version,
            TARGET_CONSTRAINTS_SCHEMA_VERSION
        );
    }

    #[test]
    fn invalid_identifiers_are_rejected() {
        assert!(ConstraintId::new("").is_err());
        assert!(ConstraintId::new("invalid value").is_err());
        assert!(ConstraintId::new("valid.namespace:constraint").is_ok());
    }

    #[test]
    fn resource_summary_rejects_invalid_duration() {
        let summary = ResourceSummary {
            duration: f64::NAN,
            ..ResourceSummary::default()
        };

        assert!(summary.validate().is_err());
    }

    #[test]
    fn unrestricted_target_accepts_max_u128_resource_counts() {
        let constraints = TargetConstraints::unrestricted();

        let summary = ResourceSummary {
            logical_qubits: u128::MAX,
            ancillas: u128::MAX,
            classical_bits: u128::MAX,
            operations: u128::MAX,
            depth: u128::MAX,
            two_qubit_depth: u128::MAX,
            single_qubit_operations: u128::MAX,
            two_qubit_operations: u128::MAX,
            multi_qubit_operations: u128::MAX,
            measurements: u128::MAX,
            resets: u128::MAX,
            barriers: u128::MAX,
            delays: u128::MAX,
            control_flow_operations: u128::MAX,
            classical_control_operations: u128::MAX,
            t_count: u128::MAX,
            t_depth: u128::MAX,
            non_clifford_operations: u128::MAX,
            duration: 0.0,
        };

        assert!(constraints.accepts_resource_summary(&summary));
    }
}