//! Zamani Quantum IR — Fault-Tolerant Dialect
//!
//! Path:
//!     src/quantum/ir/dialect/fault_tolerant.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent semantic vocabulary
//! for fault-tolerant quantum computation inside the Zamani Quantum IR.
//!
//! The dialect describes:
//!
//! - logical qubits;
//! - encoded logical resources;
//! - quantum error-correcting code identities;
//! - code parameters;
//! - logical operations;
//! - logical measurements;
//! - logical initialization;
//! - logical reset;
//! - syndrome extraction intent;
//! - stabilizer/check intent;
//! - decoding requirements;
//! - logical error-budget requirements;
//! - fault-tolerance levels;
//! - magic-state/resource requirements;
//! - distillation requirements;
//! - encoded-state preparation;
//! - logical resource accounting;
//! - fault-tolerant execution requirements;
//! - extensible future fault-tolerance operations.
//!
//! It deliberately does NOT implement:
//!
//! - a particular QEC code;
//! - a decoder;
//! - a syndrome-extraction circuit generator;
//! - physical-qubit allocation;
//! - physical topology;
//! - routing;
//! - lattice surgery implementation;
//! - magic-state factory implementation;
//! - physical gate synthesis;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - hardware execution;
//! - simulator state;
//! - probability simulation;
//! - backend APIs.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Architectural principle
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! canonical Quantum IR
//!      │
//!      ▼
//! fault_tolerant dialect
//!      │
//!      ├── declares logical intent
//!      ├── declares code requirements
//!      ├── declares error budgets
//!      ├── declares FT resources
//!      └── declares logical operations
//!      │
//!      ▼
//! QEC compiler / optimizer
//!      │
//!      ├── code selection
//!      ├── encoding
//!      ├── syndrome extraction
//!      ├── decoding
//!      ├── lattice construction
//!      └── resource estimation
//!      │
//!      ▼
//! routing / scheduling / hardware
//!      │
//!      ▼
//! backend
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program must be written once and remain independent of:
//!
//! - the number of physical qubits;
//! - the number of logical qubits;
//! - code distance;
//! - code family;
//! - physical topology;
//! - hardware vendor;
//! - physical gate set;
//! - decoder implementation;
//! - number of syndrome rounds;
//! - magic-state factory design;
//! - processor size.
//!
//! Therefore this module contains no architectural constants such as:
//!
//! ```text
//! MAX_LOGICAL_QUBITS
//! MAX_CODE_DISTANCE
//! MAX_SYNDROME_ROUNDS
//! MAX_MAGIC_STATES
//! MAX_PHYSICAL_QUBITS
//! ```
//!
//! Any concrete limits belong to explicit resource/security policies.
//!
//! A value such as `7`, `17`, `31`, `100`, or `4096` may be a valid program
//! parameter, but none of those values is a Zamani architectural limit.
//!
//! # Logical identity
//!
//! Logical qubits use the canonical identity:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module MUST NOT define another logical-qubit ID type.
//!
//! Physical identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Physical IDs are intentionally not required by the semantic fault-tolerant
//! dialect.
//!
//! # Relationship with model::logical
//!
//! `model::logical` provides the broader target-independent logical quantum
//! computation model.
//!
//! This module specializes that semantic space with fault-tolerance concepts.
//!
//! Conceptually:
//!
//! ```text
//! model::logical
//!       │
//!       └── logical computation
//!              │
//!              ▼
//! dialect::fault_tolerant
//!              │
//!              ├── encoding requirements
//!              ├── code requirements
//!              ├── logical FT operations
//!              ├── syndrome intent
//!              └── error budgets
//! ```
//!
//! This module does not replace `model::logical` and does not duplicate its
//! logical-qubit identity system.
//!
//! # Extensibility
//!
//! Fault-tolerant quantum computing is not a closed list of code families.
//!
//! New constructions must therefore be representable without modifying this
//! file merely because a new QEC code, logical gate technique, decoder,
//! distillation protocol, or encoded architecture appears.
//!
//! Standard constructions may use well-known namespace/name pairs such as:
//!
//! ```text
//! qec.surface
//! qec.color
//! qec.steane
//! qec.shor
//! qec.bacon_shor
//! qec.repetition
//! qec.css
//! qec.stabilizer
//! qec.subsystem
//! qec.gkp
//! ```
//!
//! These names are identifiers/conventions, not a closed enum.
//!
//! # Error budgets
//!
//! A fault-tolerant program may express an acceptable logical error target
//! without selecting the code or physical implementation.
//!
//! For example:
//!
//! ```text
//! target logical failure probability <= p
//! ```
//!
//! The compiler may then select a code distance, number of rounds, decoder,
//! factory architecture, or physical resources capable of satisfying that
//! requirement.
//!
//! # Determinism
//!
//! Semantic collections that do not have intrinsic ordering use `BTreeMap` and
//! `BTreeSet`.
//!
//! Ordered operation sequences remain owned by the surrounding program/region
//! representation.
//!
//! # Serialization
//!
//! This file does not define repository-wide serialization.
//!
//! The serialization subsystem must encode all public semantic information in
//! deterministic order.
//!
//! Unknown code parameters, operation attributes, and extension data must not
//! be silently discarded.
//!
//! # Hashing
//!
//! Cryptographic hashing belongs to the canonical IR hashing subsystem.
//!
//! This module supplies deterministic equality and ordering where appropriate,
//! but does not implement a cryptographic hash function.
//!
//! # Validation
//!
//! Constructors validate local invariants.
//!
//! Whole-program validation remains the responsibility of the repository-wide
//! validation subsystem.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Related IR:
//!
//! ```text
//! quantum::ir::model::logical
//! quantum::ir::operation
//! quantum::ir::resource
//! quantum::ir::capability
//! quantum::ir::attribute
//! quantum::ir::extension
//! quantum::ir::validation
//! quantum::ir::serialization
//! quantum::ir::hash
//! ```
//!
//! Downstream:
//!
//! ```text
//! quantum::optimization
//! quantum::qec
//! quantum::routing
//! quantum::scheduling
//! quantum::hardware
//! quantum::simulator
//! quantum::backend
//! ```
//!
//! This module must not depend on those downstream implementation systems.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type for fault-tolerant dialect operations.
pub type FaultTolerantResult<T> = Result<T, FaultTolerantError>;

// =============================================================================
// Error
// =============================================================================

/// Errors produced by local fault-tolerant dialect validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaultTolerantError {
    /// A required namespace/name was empty.
    EmptyIdentifier {
        /// Semantic field containing the invalid identifier.
        field: &'static str,
    },

    /// An identifier contained invalid surrounding whitespace.
    InvalidIdentifier {
        /// Semantic field containing the invalid identifier.
        field: &'static str,
    },

    /// A parameter name was duplicated.
    DuplicateParameter {
        /// Duplicated parameter name.
        name: String,
    },

    /// A code requirement contained an invalid value.
    InvalidCodeRequirement {
        /// Stable description of the invalid requirement.
        message: &'static str,
    },

    /// A logical resource was duplicated.
    DuplicateLogicalQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// An operation had duplicate targets.
    DuplicateTarget {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// An operation requires targets but none were provided.
    MissingTargets {
        /// Operation name.
        operation: String,
    },

    /// An operation's arity was invalid.
    InvalidArity {
        /// Operation name.
        operation: String,

        /// Minimum target count.
        minimum: usize,

        /// Maximum target count, if bounded.
        maximum: Option<usize>,

        /// Actual target count.
        actual: usize,
    },

    /// A logical qubit was not declared by the surrounding logical resource
    /// set when local validation required declaration.
    UnknownLogicalQubit {
        /// Referenced logical qubit.
        qubit: QubitId,
    },

    /// A requested numerical value was invalid.
    InvalidNumericValue {
        /// Semantic field.
        field: &'static str,
    },

    /// A probability was outside `[0, 1]`.
    InvalidProbability {
        /// Semantic field.
        field: &'static str,
    },

    /// A count must be greater than zero.
    ZeroCount {
        /// Semantic field.
        field: &'static str,
    },

    /// A logical error target was incompatible with the selected policy.
    InvalidErrorBudget {
        /// Stable description.
        message: &'static str,
    },

    /// A logical operation references a disabled logical resource.
    DisabledLogicalQubit {
        /// Disabled logical qubit.
        qubit: QubitId,
    },

    /// A requirement contradicts another requirement.
    ContradictoryRequirement {
        /// Stable description.
        message: &'static str,
    },
}

impl fmt::Display for FaultTolerantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(formatter, "{field} contains invalid surrounding whitespace")
            }

            Self::DuplicateParameter { name } => {
                write!(formatter, "duplicate fault-tolerant parameter `{name}`")
            }

            Self::InvalidCodeRequirement { message } => {
                write!(formatter, "invalid QEC code requirement: {message}")
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(formatter, "logical qubit {qubit} is duplicated")
            }

            Self::DuplicateTarget { qubit } => {
                write!(formatter, "logical target {qubit} is duplicated")
            }

            Self::MissingTargets { operation } => {
                write!(formatter, "fault-tolerant operation `{operation}` requires targets")
            }

            Self::InvalidArity {
                operation,
                minimum,
                maximum,
                actual,
            } => match maximum {
                Some(maximum) if minimum == maximum => {
                    write!(
                        formatter,
                        "fault-tolerant operation `{operation}` requires exactly \
                         {minimum} target(s), got {actual}"
                    )
                }
                Some(maximum) => {
                    write!(
                        formatter,
                        "fault-tolerant operation `{operation}` requires \
                         {minimum}..={maximum} target(s), got {actual}"
                    )
                }
                None => {
                    write!(
                        formatter,
                        "fault-tolerant operation `{operation}` requires at least \
                         {minimum} target(s), got {actual}"
                    )
                }
            },

            Self::UnknownLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "fault-tolerant operation references undeclared logical qubit {qubit}"
                )
            }

            Self::InvalidNumericValue { field } => {
                write!(formatter, "invalid numerical value for `{field}`")
            }

            Self::InvalidProbability { field } => {
                write!(
                    formatter,
                    "probability `{field}` must be finite and within [0, 1]"
                )
            }

            Self::ZeroCount { field } => {
                write!(formatter, "`{field}` must be greater than zero")
            }

            Self::InvalidErrorBudget { message } => {
                write!(formatter, "invalid fault-tolerant error budget: {message}")
            }

            Self::DisabledLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "fault-tolerant operation references disabled logical qubit {qubit}"
                )
            }

            Self::ContradictoryRequirement { message } => {
                write!(
                    formatter,
                    "contradictory fault-tolerant requirement: {message}"
                )
            }
        }
    }
}

impl std::error::Error for FaultTolerantError {}

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> FaultTolerantResult<()> {
    if value.is_empty() {
        return Err(FaultTolerantError::EmptyIdentifier { field });
    }

    if value.trim() != value {
        return Err(FaultTolerantError::InvalidIdentifier { field });
    }

    Ok(())
}

// =============================================================================
// Extensible qualified name
// =============================================================================

/// Extensible namespace/name identifier.
///
/// This is used instead of closed enums for QEC codes, logical operations,
/// decoders, distillation protocols, and other evolving fault-tolerant
/// concepts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QualifiedName {
    namespace: String,
    name: String,
}

impl QualifiedName {
    /// Creates a qualified name.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier("namespace", &namespace)?;
        validate_identifier("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns `namespace.name`.
    #[must_use]
    pub fn as_qualified_string(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Parameter values
// =============================================================================

/// A symbolic or concrete fault-tolerant parameter value.
///
/// Parameters remain symbolic until a downstream compilation stage decides
/// that numerical resolution is required.
#[derive(Debug, Clone, PartialEq)]
pub enum FaultTolerantValue {
    /// Exact integer value.
    Integer(i128),

    /// Exact unsigned value.
    Unsigned(u128),

    /// Finite floating-point value.
    Float(f64),

    /// Symbolic expression/name.
    Symbol(String),

    /// Boolean value.
    Boolean(bool),

    /// Textual value for extensible semantic parameters.
    String(String),
}

impl FaultTolerantValue {
    /// Creates a symbolic value.
    pub fn symbol(value: impl Into<String>) -> FaultTolerantResult<Self> {
        let value = value.into();
        validate_identifier("symbol", &value)?;
        Ok(Self::Symbol(value))
    }

    /// Returns whether this value is a finite floating-point value.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Float(value) => value.is_finite(),
            _ => true,
        }
    }
}

// =============================================================================
// Code parameter
// =============================================================================

/// A named parameter of an error-correcting code.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeParameter {
    name: String,
    value: FaultTolerantValue,
}

impl CodeParameter {
    /// Creates a code parameter.
    pub fn new(
        name: impl Into<String>,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<Self> {
        let name = name.into();
        validate_identifier("parameter name", &name)?;

        if !value.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "parameter",
            });
        }

        Ok(Self { name, value })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parameter value.
    #[must_use]
    pub fn value(&self) -> &FaultTolerantValue {
        &self.value
    }
}

// =============================================================================
// QEC code
// =============================================================================

/// Extensible quantum error-correcting code descriptor.
///
/// The descriptor is declarative and does not implement the code.
#[derive(Debug, Clone, PartialEq)]
pub struct QecCode {
    identity: QualifiedName,
    parameters: BTreeMap<String, FaultTolerantValue>,
}

impl QecCode {
    /// Creates a code descriptor with no parameters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        Ok(Self {
            identity: QualifiedName::new(namespace, name)?,
            parameters: BTreeMap::new(),
        })
    }

    /// Creates a code descriptor from a parameter sequence.
    pub fn with_parameters(
        namespace: impl Into<String>,
        name: impl Into<String>,
        parameters: impl IntoIterator<Item = CodeParameter>,
    ) -> FaultTolerantResult<Self> {
        let mut code = Self::new(namespace, name)?;

        for parameter in parameters {
            code.set_parameter(
                parameter.name().to_owned(),
                parameter.value().clone(),
            )?;
        }

        Ok(code)
    }

    /// Returns the code identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.identity
    }

    /// Returns the code namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.identity.namespace()
    }

    /// Returns the code name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.identity.name()
    }

    /// Returns all code parameters in deterministic order.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, FaultTolerantValue> {
        &self.parameters
    }

    /// Sets a code parameter.
    pub fn set_parameter(
        &mut self,
        name: String,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        validate_identifier("parameter name", &name)?;

        if !value.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "code parameter",
            });
        }

        if self.parameters.contains_key(&name) {
            return Err(FaultTolerantError::DuplicateParameter { name });
        }

        self.parameters.insert(name, value);
        Ok(())
    }

    /// Returns a code parameter.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&FaultTolerantValue> {
        self.parameters.get(name)
    }
}

// =============================================================================
// Code-selection policy
// =============================================================================

/// Policy describing how the compiler may select an error-correcting code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeSelectionPolicy {
    /// The program requires the explicitly specified code.
    Exact(QualifiedName),

    /// Any code satisfying the declared requirements is acceptable.
    AnyCompatible,

    /// The implementation may choose any compatible code from the requested
    /// namespace.
    Namespace(String),

    /// The code family is implementation-defined but must satisfy all
    /// explicitly declared requirements.
    Automatic,
}

impl CodeSelectionPolicy {
    /// Creates an exact code policy.
    pub fn exact(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        Ok(Self::Exact(QualifiedName::new(namespace, name)?))
    }

    /// Creates a namespace policy.
    pub fn namespace(
        namespace: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        let namespace = namespace.into();
        validate_identifier("code namespace", &namespace)?;
        Ok(Self::Namespace(namespace))
    }
}

// =============================================================================
// Fault-tolerance level
// =============================================================================

/// Semantic fault-tolerance assurance level.
///
/// These levels are requirements/annotations, not numerical guarantees by
/// themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultToleranceLevel {
    /// No fault-tolerance guarantee is requested.
    None,

    /// Error detection is required.
    Detection,

    /// Error correction is required.
    Correction,

    /// Logical computation must remain protected during execution.
    FaultTolerant,

    /// Strong fault-tolerant execution requirements are requested.
    FaultTolerantProtected,

    /// The program requires a fault-tolerant architecture capable of
    /// maintaining a declared logical error budget.
    ErrorBudgetBounded,
}

impl FaultToleranceLevel {
    /// Returns whether the level requires active fault-tolerant correction.
    #[must_use]
    pub const fn requires_correction(self) -> bool {
        matches!(
            self,
            Self::Correction
                | Self::FaultTolerant
                | Self::FaultTolerantProtected
                | Self::ErrorBudgetBounded
        )
    }
}

// =============================================================================
// Logical error budget
// =============================================================================

/// A declarative logical-error budget.
///
/// All probability fields are optional so a program can specify only the
/// constraint it actually cares about.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalErrorBudget {
    per_operation: Option<f64>,
    per_logical_qubit: Option<f64>,
    total_failure: Option<f64>,
    per_circuit: Option<f64>,
}

impl LogicalErrorBudget {
    /// Creates an empty error budget.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            per_operation: None,
            per_logical_qubit: None,
            total_failure: None,
            per_circuit: None,
        }
    }

    fn validate_probability(
        field: &'static str,
        value: f64,
    ) -> FaultTolerantResult<()> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(FaultTolerantError::InvalidProbability { field });
        }

        Ok(())
    }

    /// Sets the maximum logical error probability per operation.
    pub fn set_per_operation(
        &mut self,
        value: f64,
    ) -> FaultTolerantResult<()> {
        Self::validate_probability("per_operation", value)?;
        self.per_operation = Some(value);
        Ok(())
    }

    /// Sets the maximum logical error probability per logical qubit.
    pub fn set_per_logical_qubit(
        &mut self,
        value: f64,
    ) -> FaultTolerantResult<()> {
        Self::validate_probability("per_logical_qubit", value)?;
        self.per_logical_qubit = Some(value);
        Ok(())
    }

    /// Sets the maximum total logical failure probability.
    pub fn set_total_failure(
        &mut self,
        value: f64,
    ) -> FaultTolerantResult<()> {
        Self::validate_probability("total_failure", value)?;
        self.total_failure = Some(value);
        Ok(())
    }

    /// Sets the maximum circuit failure probability.
    pub fn set_per_circuit(
        &mut self,
        value: f64,
    ) -> FaultTolerantResult<()> {
        Self::validate_probability("per_circuit", value)?;
        self.per_circuit = Some(value);
        Ok(())
    }

    /// Returns the per-operation target.
    #[must_use]
    pub const fn per_operation(&self) -> Option<f64> {
        self.per_operation
    }

    /// Returns the per-logical-qubit target.
    #[must_use]
    pub const fn per_logical_qubit(&self) -> Option<f64> {
        self.per_logical_qubit
    }

    /// Returns the total-failure target.
    #[must_use]
    pub const fn total_failure(&self) -> Option<f64> {
        self.total_failure
    }

    /// Returns the per-circuit target.
    #[must_use]
    pub const fn per_circuit(&self) -> Option<f64> {
        self.per_circuit
    }

    /// Returns whether any error-budget constraint exists.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.per_operation.is_none()
            && self.per_logical_qubit.is_none()
            && self.total_failure.is_none()
            && self.per_circuit.is_none()
    }
}

impl Default for LogicalErrorBudget {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Logical resource
// =============================================================================

/// Status of a logical qubit in the fault-tolerant semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalResourceState {
    /// Logical resource is usable.
    Available,

    /// Logical resource is encoded.
    Encoded,

    /// Logical resource is measured.
    Measured,

    /// Logical resource has been reset.
    Reset,

    /// Logical resource is unavailable.
    Disabled,
}

impl Default for LogicalResourceState {
    fn default() -> Self {
        Self::Available
    }
}

impl LogicalResourceState {
    /// Returns whether ordinary logical operations may reference the resource.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

/// Fault-tolerant logical resource.
///
/// The identity remains the canonical `QubitId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalResource {
    qubit: QubitId,
    state: LogicalResourceState,
}

impl LogicalResource {
    /// Creates an unencoded logical resource.
    #[must_use]
    pub const fn new(qubit: QubitId) -> Self {
        Self {
            qubit,
            state: LogicalResourceState::Available,
        }
    }

    /// Returns the canonical logical qubit identity.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the semantic state.
    #[must_use]
    pub const fn state(&self) -> LogicalResourceState {
        self.state
    }

    /// Marks the logical resource as encoded.
    pub const fn mark_encoded(&mut self) {
        self.state = LogicalResourceState::Encoded;
    }

    /// Marks the logical resource as measured.
    pub const fn mark_measured(&mut self) {
        self.state = LogicalResourceState::Measured;
    }

    /// Marks the logical resource as reset.
    pub const fn mark_reset(&mut self) {
        self.state = LogicalResourceState::Reset;
    }

    /// Marks the logical resource as available.
    pub const fn mark_available(&mut self) {
        self.state = LogicalResourceState::Available;
    }

    /// Marks the logical resource as disabled.
    pub const fn mark_disabled(&mut self) {
        self.state = LogicalResourceState::Disabled;
    }
}

// =============================================================================
// Logical resource set
// =============================================================================

/// Deterministic collection of logical fault-tolerant resources.
///
/// This structure does not allocate physical qubits and does not imply a
/// hardware size.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicalResourceSet {
    resources: BTreeMap<QubitId, LogicalResourceState>,
}

impl LogicalResourceSet {
    /// Creates an empty logical resource set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            resources: BTreeMap::new(),
        }
    }

    /// Adds a logical resource.
    pub fn insert(
        &mut self,
        resource: LogicalResource,
    ) -> FaultTolerantResult<()> {
        let qubit = resource.qubit();

        if self.resources.contains_key(&qubit) {
            return Err(FaultTolerantError::DuplicateLogicalQubit { qubit });
        }

        self.resources.insert(qubit, resource.state());
        Ok(())
    }

    /// Adds a logical qubit in the default available state.
    pub fn insert_qubit(
        &mut self,
        qubit: QubitId,
    ) -> FaultTolerantResult<()> {
        self.insert(LogicalResource::new(qubit))
    }

    /// Returns whether a logical qubit exists.
    #[must_use]
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.resources.contains_key(&qubit)
    }

    /// Returns the resource state.
    #[must_use]
    pub fn state(
        &self,
        qubit: QubitId,
    ) -> Option<LogicalResourceState> {
        self.resources.get(&qubit).copied()
    }

    /// Returns the number of logical resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether no logical resources exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns resources in deterministic logical-ID order.
    #[must_use]
    pub fn resources(&self) -> &BTreeMap<QubitId, LogicalResourceState> {
        &self.resources
    }

    /// Validates that every referenced target is declared and usable.
    pub fn validate_targets(
        &self,
        targets: &[QubitId],
    ) -> FaultTolerantResult<()> {
        let mut seen = BTreeSet::new();

        for &qubit in targets {
            if !seen.insert(qubit) {
                return Err(FaultTolerantError::DuplicateTarget { qubit });
            }

            match self.resources.get(&qubit) {
                None => {
                    return Err(
                        FaultTolerantError::UnknownLogicalQubit { qubit }
                    );
                }

                Some(state) if !state.is_usable() => {
                    return Err(
                        FaultTolerantError::DisabledLogicalQubit { qubit }
                    );
                }

                Some(_) => {}
            }
        }

        Ok(())
    }
}

// =============================================================================
// Logical operation arity
// =============================================================================

/// Target-arity constraint for a fault-tolerant operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arity {
    /// Exactly this many targets.
    Exact(usize),

    /// At least this many targets.
    AtLeast(usize),

    /// Between minimum and maximum inclusive.
    Range {
        /// Minimum target count.
        minimum: usize,

        /// Maximum target count.
        maximum: usize,
    },

    /// Any target count, including zero.
    Any,
}

impl Arity {
    /// Validates a target count.
    #[must_use]
    pub fn accepts(self, actual: usize) -> bool {
        match self {
            Self::Exact(expected) => actual == expected,
            Self::AtLeast(minimum) => actual >= minimum,
            Self::Range { minimum, maximum } => {
                actual >= minimum && actual <= maximum
            }
            Self::Any => true,
        }
    }

    /// Returns the minimum required target count.
    #[must_use]
    pub const fn minimum(self) -> usize {
        match self {
            Self::Exact(value) => value,
            Self::AtLeast(value) => value,
            Self::Range { minimum, .. } => minimum,
            Self::Any => 0,
        }
    }

    /// Returns the maximum target count where bounded.
    #[must_use]
    pub const fn maximum(self) -> Option<usize> {
        match self {
            Self::Exact(value) => Some(value),
            Self::AtLeast(_) | Self::Any => None,
            Self::Range { maximum, .. } => Some(maximum),
        }
    }
}

// =============================================================================
// Logical operation kind
// =============================================================================

/// Extensible fault-tolerant operation identity.
///
/// Standard names are conventions rather than a closed architectural set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicalOperationKind(QualifiedName);

impl LogicalOperationKind {
    /// Creates an operation kind.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        Ok(Self(QualifiedName::new(namespace, name)?))
    }

    /// Returns the qualified operation identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.0
    }
}

impl fmt::Display for LogicalOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Logical operation
// =============================================================================

/// Declarative logical fault-tolerant operation.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalOperation {
    kind: LogicalOperationKind,
    targets: Vec<QubitId>,
    parameters: BTreeMap<String, FaultTolerantValue>,
    arity: Arity,
}

impl LogicalOperation {
    /// Creates an operation.
    pub fn new(
        kind: LogicalOperationKind,
        targets: impl Into<Vec<QubitId>>,
        arity: Arity,
    ) -> FaultTolerantResult<Self> {
        let targets = targets.into();

        if !arity.accepts(targets.len()) {
            return Err(FaultTolerantError::InvalidArity {
                operation: kind.to_string(),
                minimum: arity.minimum(),
                maximum: arity.maximum(),
                actual: targets.len(),
            });
        }

        if arity.minimum() > 0 && targets.is_empty() {
            return Err(FaultTolerantError::MissingTargets {
                operation: kind.to_string(),
            });
        }

        let mut seen = BTreeSet::new();

        for &target in &targets {
            if !seen.insert(target) {
                return Err(FaultTolerantError::DuplicateTarget {
                    qubit: target,
                });
            }
        }

        Ok(Self {
            kind,
            targets,
            parameters: BTreeMap::new(),
            arity,
        })
    }

    /// Returns the operation kind.
    #[must_use]
    pub fn kind(&self) -> &LogicalOperationKind {
        &self.kind
    }

    /// Returns ordered logical targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns operation parameters.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &BTreeMap<String, FaultTolerantValue> {
        &self.parameters
    }

    /// Returns the declared arity constraint.
    #[must_use]
    pub const fn arity(&self) -> Arity {
        self.arity
    }

    /// Adds a parameter.
    pub fn set_parameter(
        &mut self,
        name: String,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        validate_identifier("operation parameter name", &name)?;

        if !value.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "operation parameter",
            });
        }

        if self.parameters.contains_key(&name) {
            return Err(FaultTolerantError::DuplicateParameter { name });
        }

        self.parameters.insert(name, value);
        Ok(())
    }

    /// Validates operation targets against a logical resource set.
    pub fn validate_targets(
        &self,
        resources: &LogicalResourceSet,
    ) -> FaultTolerantResult<()> {
        resources.validate_targets(&self.targets)
    }
}

// =============================================================================
// Syndrome type
// =============================================================================

/// Semantic kind of syndrome/check information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyndromeKind {
    /// Generic syndrome information.
    Generic,

    /// Stabilizer measurement result.
    Stabilizer,

    /// Gauge measurement result.
    Gauge,

    /// Parity-check result.
    Parity,

    /// Subsystem-code gauge information.
    Subsystem,

    /// Extensible check mechanism.
    Custom,
}

// =============================================================================
// Syndrome requirement
// =============================================================================

/// Declarative syndrome-extraction requirement.
#[derive(Debug, Clone, PartialEq)]
pub struct SyndromeRequirement {
    kind: SyndromeKind,
    rounds: Option<FaultTolerantValue>,
    checks: Option<FaultTolerantValue>,
    adaptive: bool,
    preserve_logical_state: bool,
}

impl SyndromeRequirement {
    /// Creates a generic syndrome requirement.
    #[must_use]
    pub const fn new(kind: SyndromeKind) -> Self {
        Self {
            kind,
            rounds: None,
            checks: None,
            adaptive: false,
            preserve_logical_state: true,
        }
    }

    /// Returns syndrome kind.
    #[must_use]
    pub const fn kind(&self) -> SyndromeKind {
        self.kind
    }

    /// Sets a symbolic or concrete syndrome-round requirement.
    pub fn set_rounds(
        &mut self,
        rounds: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        if !rounds.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "syndrome rounds",
            });
        }

        self.rounds = Some(rounds);
        Ok(())
    }

    /// Sets a symbolic or concrete check requirement.
    pub fn set_checks(
        &mut self,
        checks: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        if !checks.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "syndrome checks",
            });
        }

        self.checks = Some(checks);
        Ok(())
    }

    /// Sets whether syndrome extraction may be adaptive.
    pub const fn set_adaptive(&mut self, adaptive: bool) {
        self.adaptive = adaptive;
    }

    /// Sets whether logical state preservation is required.
    pub const fn set_preserve_logical_state(&mut self, preserve: bool) {
        self.preserve_logical_state = preserve;
    }

    /// Returns the requested syndrome rounds.
    #[must_use]
    pub fn rounds(&self) -> Option<&FaultTolerantValue> {
        self.rounds.as_ref()
    }

    /// Returns the requested checks.
    #[must_use]
    pub fn checks(&self) -> Option<&FaultTolerantValue> {
        self.checks.as_ref()
    }

    /// Returns whether adaptive extraction is permitted/required.
    #[must_use]
    pub const fn adaptive(&self) -> bool {
        self.adaptive
    }

    /// Returns whether logical state preservation is required.
    #[must_use]
    pub const fn preserve_logical_state(&self) -> bool {
        self.preserve_logical_state
    }
}

// =============================================================================
// Decoder requirement
// =============================================================================

/// Declarative decoder requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderRequirement {
    /// Any decoder satisfying the program requirements.
    AnyCompatible,

    /// A particular decoder identity is required.
    Exact(QualifiedName),

    /// Decoder must belong to a namespace.
    Namespace(String),
}

impl DecoderRequirement {
    /// Creates an exact decoder requirement.
    pub fn exact(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        Ok(Self::Exact(QualifiedName::new(namespace, name)?))
    }

    /// Creates a decoder namespace requirement.
    pub fn namespace(
        namespace: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        let namespace = namespace.into();
        validate_identifier("decoder namespace", &namespace)?;
        Ok(Self::Namespace(namespace))
    }
}

// =============================================================================
// Magic-state resource
// =============================================================================

/// Type of non-Clifford resource used by a fault-tolerant architecture.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MagicResourceKind(QualifiedName);

impl MagicResourceKind {
    /// Creates a magic-resource kind.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> FaultTolerantResult<Self> {
        Ok(Self(QualifiedName::new(namespace, name)?))
    }

    /// Returns the resource identity.
    #[must_use]
    pub fn identity(&self) -> &QualifiedName {
        &self.0
    }
}

impl fmt::Display for MagicResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Magic-state requirement
// =============================================================================

/// Declarative magic-state resource requirement.
#[derive(Debug, Clone, PartialEq)]
pub struct MagicStateRequirement {
    kind: MagicResourceKind,
    amount: FaultTolerantValue,
    minimum_fidelity: Option<f64>,
}

impl MagicStateRequirement {
    /// Creates a magic-state requirement.
    pub fn new(
        kind: MagicResourceKind,
        amount: FaultTolerantValue,
    ) -> FaultTolerantResult<Self> {
        if !amount.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "magic-state amount",
            });
        }

        Ok(Self {
            kind,
            amount,
            minimum_fidelity: None,
        })
    }

    /// Returns the magic-state kind.
    #[must_use]
    pub fn kind(&self) -> &MagicResourceKind {
        &self.kind
    }

    /// Returns the requested amount.
    #[must_use]
    pub fn amount(&self) -> &FaultTolerantValue {
        &self.amount
    }

    /// Sets the minimum acceptable fidelity.
    pub fn set_minimum_fidelity(
        &mut self,
        fidelity: f64,
    ) -> FaultTolerantResult<()> {
        if !fidelity.is_finite() || !(0.0..=1.0).contains(&fidelity) {
            return Err(FaultTolerantError::InvalidProbability {
                field: "minimum fidelity",
            });
        }

        self.minimum_fidelity = Some(fidelity);
        Ok(())
    }

    /// Returns the minimum fidelity requirement.
    #[must_use]
    pub const fn minimum_fidelity(&self) -> Option<f64> {
        self.minimum_fidelity
    }
}

// =============================================================================
// Distillation requirement
// =============================================================================

/// Declarative magic-state distillation requirement.
#[derive(Debug, Clone, PartialEq)]
pub struct DistillationRequirement {
    protocol: QualifiedName,
    input_fidelity: Option<f64>,
    output_fidelity: Option<f64>,
    output_count: FaultTolerantValue,
}

impl DistillationRequirement {
    /// Creates a distillation requirement.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        output_count: FaultTolerantValue,
    ) -> FaultTolerantResult<Self> {
        if !output_count.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "distillation output count",
            });
        }

        Ok(Self {
            protocol: QualifiedName::new(namespace, name)?,
            input_fidelity: None,
            output_fidelity: None,
            output_count,
        })
    }

    /// Sets minimum input fidelity.
    pub fn set_input_fidelity(
        &mut self,
        fidelity: f64,
    ) -> FaultTolerantResult<()> {
        if !fidelity.is_finite() || !(0.0..=1.0).contains(&fidelity) {
            return Err(FaultTolerantError::InvalidProbability {
                field: "distillation input fidelity",
            });
        }

        self.input_fidelity = Some(fidelity);
        Ok(())
    }

    /// Sets minimum output fidelity.
    pub fn set_output_fidelity(
        &mut self,
        fidelity: f64,
    ) -> FaultTolerantResult<()> {
        if !fidelity.is_finite() || !(0.0..=1.0).contains(&fidelity) {
            return Err(FaultTolerantError::InvalidProbability {
                field: "distillation output fidelity",
            });
        }

        self.output_fidelity = Some(fidelity);
        Ok(())
    }

    /// Returns the distillation protocol.
    #[must_use]
    pub fn protocol(&self) -> &QualifiedName {
        &self.protocol
    }

    /// Returns the requested output count.
    #[must_use]
    pub fn output_count(&self) -> &FaultTolerantValue {
        &self.output_count
    }

    /// Returns input fidelity.
    #[must_use]
    pub const fn input_fidelity(&self) -> Option<f64> {
        self.input_fidelity
    }

    /// Returns output fidelity.
    #[must_use]
    pub const fn output_fidelity(&self) -> Option<f64> {
        self.output_fidelity
    }
}

// =============================================================================
// Logical measurement
// =============================================================================

/// Logical measurement basis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalMeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// X basis.
    X,

    /// Y basis.
    Y,

    /// Named observable.
    Observable(QualifiedName),

    /// Extensible basis.
    Custom(QualifiedName),
}

/// Declarative logical measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalMeasurement {
    targets: Vec<QubitId>,
    basis: LogicalMeasurementBasis,
    destructive: bool,
}

impl LogicalMeasurement {
    /// Creates a logical measurement.
    pub fn new(
        targets: impl Into<Vec<QubitId>>,
        basis: LogicalMeasurementBasis,
        destructive: bool,
    ) -> FaultTolerantResult<Self> {
        let targets = targets.into();

        if targets.is_empty() {
            return Err(FaultTolerantError::MissingTargets {
                operation: "logical.measure".to_owned(),
            });
        }

        let mut seen = BTreeSet::new();

        for &qubit in &targets {
            if !seen.insert(qubit) {
                return Err(FaultTolerantError::DuplicateTarget {
                    qubit,
                });
            }
        }

        Ok(Self {
            targets,
            basis,
            destructive,
        })
    }

    /// Returns measurement targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns measurement basis.
    #[must_use]
    pub fn basis(&self) -> &LogicalMeasurementBasis {
        &self.basis
    }

    /// Returns whether measurement is destructive.
    #[must_use]
    pub const fn destructive(&self) -> bool {
        self.destructive
    }
}

// =============================================================================
// Encoded-state preparation
// =============================================================================

/// Kind of logical/encoded state preparation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PreparationKind {
    /// Logical zero state.
    LogicalZero,

    /// Logical one state.
    LogicalOne,

    /// Named logical state.
    Named(QualifiedName),

    /// Custom encoded-state preparation.
    Custom(QualifiedName),
}

/// Declarative encoded-state preparation requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedPreparation {
    targets: Vec<QubitId>,
    kind: PreparationKind,
}

impl EncodedPreparation {
    /// Creates encoded-state preparation.
    pub fn new(
        targets: impl Into<Vec<QubitId>>,
        kind: PreparationKind,
    ) -> FaultTolerantResult<Self> {
        let targets = targets.into();

        if targets.is_empty() {
            return Err(FaultTolerantError::MissingTargets {
                operation: "logical.prepare".to_owned(),
            });
        }

        let mut seen = BTreeSet::new();

        for &qubit in &targets {
            if !seen.insert(qubit) {
                return Err(FaultTolerantError::DuplicateTarget {
                    qubit,
                });
            }
        }

        Ok(Self { targets, kind })
    }

    /// Returns target logical qubits.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns preparation kind.
    #[must_use]
    pub fn kind(&self) -> &PreparationKind {
        &self.kind
    }
}

// =============================================================================
// Logical resource estimate
// =============================================================================

/// Declarative fault-tolerant resource requirement.
///
/// All values may remain symbolic.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResourceRequirement {
    logical_qubits: Option<FaultTolerantValue>,
    physical_qubits: Option<FaultTolerantValue>,
    syndrome_measurements: Option<FaultTolerantValue>,
    correction_operations: Option<FaultTolerantValue>,
    magic_states: Option<FaultTolerantValue>,
    circuit_depth: Option<FaultTolerantValue>,
}

impl ResourceRequirement {
    /// Creates an empty requirement.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_qubits: None,
            physical_qubits: None,
            syndrome_measurements: None,
            correction_operations: None,
            magic_states: None,
            circuit_depth: None,
        }
    }

    fn set_value(
        slot: &mut Option<FaultTolerantValue>,
        field: &'static str,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        if !value.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue { field });
        }

        *slot = Some(value);
        Ok(())
    }

    /// Sets logical-qubit requirement.
    pub fn set_logical_qubits(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.logical_qubits,
            "logical qubits",
            value,
        )
    }

    /// Sets physical-qubit requirement.
    pub fn set_physical_qubits(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.physical_qubits,
            "physical qubits",
            value,
        )
    }

    /// Sets syndrome-measurement requirement.
    pub fn set_syndrome_measurements(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.syndrome_measurements,
            "syndrome measurements",
            value,
        )
    }

    /// Sets correction-operation requirement.
    pub fn set_correction_operations(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.correction_operations,
            "correction operations",
            value,
        )
    }

    /// Sets magic-state requirement.
    pub fn set_magic_states(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.magic_states,
            "magic states",
            value,
        )
    }

    /// Sets logical circuit-depth requirement.
    pub fn set_circuit_depth(
        &mut self,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        Self::set_value(
            &mut self.circuit_depth,
            "circuit depth",
            value,
        )
    }

    /// Returns logical-qubit requirement.
    #[must_use]
    pub fn logical_qubits(&self) -> Option<&FaultTolerantValue> {
        self.logical_qubits.as_ref()
    }

    /// Returns physical-qubit requirement.
    #[must_use]
    pub fn physical_qubits(&self) -> Option<&FaultTolerantValue> {
        self.physical_qubits.as_ref()
    }

    /// Returns syndrome-measurement requirement.
    #[must_use]
    pub fn syndrome_measurements(
        &self,
    ) -> Option<&FaultTolerantValue> {
        self.syndrome_measurements.as_ref()
    }

    /// Returns correction-operation requirement.
    #[must_use]
    pub fn correction_operations(
        &self,
    ) -> Option<&FaultTolerantValue> {
        self.correction_operations.as_ref()
    }

    /// Returns magic-state requirement.
    #[must_use]
    pub fn magic_states(&self) -> Option<&FaultTolerantValue> {
        self.magic_states.as_ref()
    }

    /// Returns circuit-depth requirement.
    #[must_use]
    pub fn circuit_depth(&self) -> Option<&FaultTolerantValue> {
        self.circuit_depth.as_ref()
    }
}

// =============================================================================
// Fault-tolerant operation
// =============================================================================

/// Top-level fault-tolerant dialect operation.
///
/// The operation is deliberately extensible. Standard operations are represented
/// by semantic kinds rather than a closed enum.
#[derive(Debug, Clone, PartialEq)]
pub enum FaultTolerantOperation {
    /// Encodes logical resources according to a selected/required code.
    Encode {
        /// Target logical qubits.
        targets: Vec<QubitId>,

        /// Code requirement.
        code: QecCode,
    },

    /// Decodes/releases logical resources.
    Decode {
        /// Target logical qubits.
        targets: Vec<QubitId>,
    },

    /// Performs a logical operation.
    LogicalGate(LogicalOperation),

    /// Performs logical measurement.
    Measure(LogicalMeasurement),

    /// Prepares an encoded state.
    Prepare(EncodedPreparation),

    /// Performs logical reset.
    Reset {
        /// Target logical qubits.
        targets: Vec<QubitId>,
    },

    /// Declares syndrome extraction intent.
    Syndrome {
        /// Target logical resources.
        targets: Vec<QubitId>,

        /// Syndrome requirements.
        requirement: SyndromeRequirement,
    },

    /// Requests a correction/recovery phase.
    Correct {
        /// Target logical resources.
        targets: Vec<QubitId>,

        /// Optional decoder requirement.
        decoder: Option<DecoderRequirement>,
    },

    /// Requests magic-state resource production/use.
    MagicState(MagicStateRequirement),

    /// Requests magic-state distillation.
    Distill(DistillationRequirement),

    /// Extensible custom fault-tolerant operation.
    Custom {
        /// Operation identity.
        kind: LogicalOperationKind,

        /// Logical targets.
        targets: Vec<QubitId>,

        /// Parameters.
        parameters: BTreeMap<String, FaultTolerantValue>,
    },
}

impl FaultTolerantOperation {
    /// Returns logical targets where the operation has explicit targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        match self {
            Self::Encode { targets, .. }
            | Self::Decode { targets }
            | Self::Reset { targets }
            | Self::Syndrome { targets, .. }
            | Self::Correct { targets, .. } => targets,

            Self::LogicalGate(operation) => operation.targets(),

            Self::Measure(measurement) => measurement.targets(),

            Self::Prepare(preparation) => preparation.targets(),

            Self::MagicState(_)
            | Self::Distill(_) => &[],

            Self::Custom { targets, .. } => targets,
        }
    }

    /// Validates that explicit targets are unique.
    pub fn validate_targets(&self) -> FaultTolerantResult<()> {
        let mut seen = BTreeSet::new();

        for &target in self.targets() {
            if !seen.insert(target) {
                return Err(FaultTolerantError::DuplicateTarget {
                    qubit: target,
                });
            }
        }

        Ok(())
    }

    /// Validates targets against a logical resource set.
    pub fn validate_against(
        &self,
        resources: &LogicalResourceSet,
    ) -> FaultTolerantResult<()> {
        self.validate_targets()?;
        resources.validate_targets(self.targets())
    }
}

// =============================================================================
// Fault-tolerant program requirements
// =============================================================================

/// Complete declarative fault-tolerance requirement set.
///
/// This object is intentionally independent from physical machine topology.
#[derive(Debug, Clone, PartialEq)]
pub struct FaultTolerantRequirements {
    level: FaultToleranceLevel,
    code_policy: CodeSelectionPolicy,
    error_budget: LogicalErrorBudget,
    decoder: Option<DecoderRequirement>,
    syndrome: Option<SyndromeRequirement>,
    resources: ResourceRequirement,
    logical_qubits: LogicalResourceSet,
    magic_states: Vec<MagicStateRequirement>,
    distillation: Vec<DistillationRequirement>,
    attributes: BTreeMap<String, FaultTolerantValue>,
}

impl FaultTolerantRequirements {
    /// Creates a default fault-tolerance requirement set.
    pub fn new(
        level: FaultToleranceLevel,
        code_policy: CodeSelectionPolicy,
    ) -> Self {
        Self {
            level,
            code_policy,
            error_budget: LogicalErrorBudget::new(),
            decoder: None,
            syndrome: None,
            resources: ResourceRequirement::new(),
            logical_qubits: LogicalResourceSet::new(),
            magic_states: Vec::new(),
            distillation: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Returns fault-tolerance level.
    #[must_use]
    pub const fn level(&self) -> FaultToleranceLevel {
        self.level
    }

    /// Returns code-selection policy.
    #[must_use]
    pub fn code_policy(&self) -> &CodeSelectionPolicy {
        &self.code_policy
    }

    /// Returns logical error budget.
    #[must_use]
    pub fn error_budget(&self) -> &LogicalErrorBudget {
        &self.error_budget
    }

    /// Returns mutable logical error budget.
    #[must_use]
    pub fn error_budget_mut(&mut self) -> &mut LogicalErrorBudget {
        &mut self.error_budget
    }

    /// Sets a decoder requirement.
    pub fn set_decoder(
        &mut self,
        decoder: DecoderRequirement,
    ) {
        self.decoder = Some(decoder);
    }

    /// Returns decoder requirement.
    #[must_use]
    pub fn decoder(&self) -> Option<&DecoderRequirement> {
        self.decoder.as_ref()
    }

    /// Sets syndrome requirements.
    pub fn set_syndrome(
        &mut self,
        syndrome: SyndromeRequirement,
    ) {
        self.syndrome = Some(syndrome);
    }

    /// Returns syndrome requirements.
    #[must_use]
    pub fn syndrome(&self) -> Option<&SyndromeRequirement> {
        self.syndrome.as_ref()
    }

    /// Returns resource requirements.
    #[must_use]
    pub fn resources(&self) -> &ResourceRequirement {
        &self.resources
    }

    /// Returns mutable resource requirements.
    #[must_use]
    pub fn resources_mut(&mut self) -> &mut ResourceRequirement {
        &mut self.resources
    }

    /// Returns logical resources.
    #[must_use]
    pub fn logical_qubits(&self) -> &LogicalResourceSet {
        &self.logical_qubits
    }

    /// Returns mutable logical resources.
    #[must_use]
    pub fn logical_qubits_mut(&mut self) -> &mut LogicalResourceSet {
        &mut self.logical_qubits
    }

    /// Adds a magic-state requirement.
    pub fn add_magic_state(
        &mut self,
        requirement: MagicStateRequirement,
    ) {
        self.magic_states.push(requirement);
    }

    /// Returns magic-state requirements.
    #[must_use]
    pub fn magic_states(&self) -> &[MagicStateRequirement] {
        &self.magic_states
    }

    /// Adds a distillation requirement.
    pub fn add_distillation(
        &mut self,
        requirement: DistillationRequirement,
    ) {
        self.distillation.push(requirement);
    }

    /// Returns distillation requirements.
    #[must_use]
    pub fn distillation(&self) -> &[DistillationRequirement] {
        &self.distillation
    }

    /// Sets an extensible semantic attribute.
    pub fn set_attribute(
        &mut self,
        name: String,
        value: FaultTolerantValue,
    ) -> FaultTolerantResult<()> {
        validate_identifier("fault-tolerant attribute", &name)?;

        if !value.is_finite() {
            return Err(FaultTolerantError::InvalidNumericValue {
                field: "fault-tolerant attribute",
            });
        }

        self.attributes.insert(name, value);
        Ok(())
    }

    /// Returns attributes in deterministic order.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<String, FaultTolerantValue> {
        &self.attributes
    }

    /// Performs local consistency validation.
    pub fn validate(&self) -> FaultTolerantResult<()> {
        if self.level == FaultToleranceLevel::ErrorBudgetBounded
            && self.error_budget.is_empty()
        {
            return Err(FaultTolerantError::ContradictoryRequirement {
                message:
                    "ErrorBudgetBounded requires at least one logical error budget",
            });
        }

        if self.level.requires_correction()
            && matches!(
                self.code_policy,
                CodeSelectionPolicy::Exact(_)
            )
            && self.logical_qubits.is_empty()
        {
            // This is intentionally NOT an error: logical resources may be
            // declared by the surrounding program/region. The requirement
            // object is valid independently.
        }

        Ok(())
    }
}

// =============================================================================
// Fault-tolerant dialect
// =============================================================================

/// Complete fault-tolerant dialect fragment.
///
/// This is the primary type downstream IR consumers should use when they need
/// a self-contained fault-tolerant semantic declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct FaultTolerantDialect {
    requirements: FaultTolerantRequirements,
    operations: Vec<FaultTolerantOperation>,
}

impl FaultTolerantDialect {
    /// Creates an empty fault-tolerant dialect instance.
    #[must_use]
    pub fn new(
        level: FaultToleranceLevel,
        code_policy: CodeSelectionPolicy,
    ) -> Self {
        Self {
            requirements: FaultTolerantRequirements::new(
                level,
                code_policy,
            ),
            operations: Vec::new(),
        }
    }

    /// Returns fault-tolerance requirements.
    #[must_use]
    pub fn requirements(&self) -> &FaultTolerantRequirements {
        &self.requirements
    }

    /// Returns mutable fault-tolerance requirements.
    #[must_use]
    pub fn requirements_mut(
        &mut self,
    ) -> &mut FaultTolerantRequirements {
        &mut self.requirements
    }

    /// Returns ordered dialect operations.
    #[must_use]
    pub fn operations(&self) -> &[FaultTolerantOperation] {
        &self.operations
    }

    /// Adds an operation after local validation.
    pub fn push(
        &mut self,
        operation: FaultTolerantOperation,
    ) -> FaultTolerantResult<()> {
        operation.validate_targets()?;
        self.operations.push(operation);
        Ok(())
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether no operations exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Validates the complete local dialect.
    pub fn validate(&self) -> FaultTolerantResult<()> {
        self.requirements.validate()?;

        for operation in &self.operations {
            operation.validate_against(
                self.requirements.logical_qubits(),
            )?;
        }

        Ok(())
    }

    /// Returns whether the dialect explicitly requests fault tolerance.
    #[must_use]
    pub const fn is_fault_tolerant(&self) -> bool {
        !matches!(
            self.requirements.level(),
            FaultToleranceLevel::None
        )
    }
}

// =============================================================================
// Standard semantic constructors
// =============================================================================

/// Creates the standard logical-X operation identity.
pub fn logical_x(
    target: QubitId,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new("logical", "x")?,
        vec![target],
        Arity::Exact(1),
    )
}

/// Creates the standard logical-Y operation identity.
pub fn logical_y(
    target: QubitId,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new("logical", "y")?,
        vec![target],
        Arity::Exact(1),
    )
}

/// Creates the standard logical-Z operation identity.
pub fn logical_z(
    target: QubitId,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new("logical", "z")?,
        vec![target],
        Arity::Exact(1),
    )
}

/// Creates the standard logical-H operation identity.
pub fn logical_h(
    target: QubitId,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new("logical", "h")?,
        vec![target],
        Arity::Exact(1),
    )
}

/// Creates the standard logical-CX operation identity.
pub fn logical_cx(
    control: QubitId,
    target: QubitId,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new("logical", "cx")?,
        vec![control, target],
        Arity::Exact(2),
    )
}

/// Creates an arbitrary extensible logical operation.
pub fn custom_logical_operation(
    namespace: impl Into<String>,
    name: impl Into<String>,
    targets: impl Into<Vec<QubitId>>,
    arity: Arity,
) -> FaultTolerantResult<LogicalOperation> {
    LogicalOperation::new(
        LogicalOperationKind::new(namespace, name)?,
        targets,
        arity,
    )
}

// =============================================================================
// Standard code constructors
// =============================================================================

/// Creates an extensible surface-code descriptor.
///
/// The constructor intentionally does not select a distance. Distance remains
/// a program/compiler parameter.
pub fn surface_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "surface")
}

/// Creates an extensible color-code descriptor.
pub fn color_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "color")
}

/// Creates an extensible Steane-code descriptor.
pub fn steane_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "steane")
}

/// Creates an extensible Shor-code descriptor.
pub fn shor_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "shor")
}

/// Creates an extensible Bacon-Shor-code descriptor.
pub fn bacon_shor_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "bacon_shor")
}

/// Creates an extensible repetition-code descriptor.
pub fn repetition_code() -> FaultTolerantResult<QecCode> {
    QecCode::new("qec", "repetition")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualified_names_are_deterministic() {
        let name =
            QualifiedName::new("qec", "surface").expect("valid name");

        assert_eq!(name.namespace(), "qec");
        assert_eq!(name.name(), "surface");
        assert_eq!(name.to_string(), "qec.surface");
    }

    #[test]
    fn canonical_qubit_identity_is_used() {
        let qubit = QubitId::new(42);
        let resource = LogicalResource::new(qubit);

        assert_eq!(resource.qubit(), qubit);
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let qubit = QubitId::new(1);
        let mut resources = LogicalResourceSet::new();

        resources
            .insert_qubit(qubit)
            .expect("first insertion succeeds");

        let error = resources
            .insert_qubit(qubit)
            .expect_err("duplicate must fail");

        assert_eq!(
            error,
            FaultTolerantError::DuplicateLogicalQubit {
                qubit
            }
        );
    }

    #[test]
    fn duplicate_operation_targets_are_rejected() {
        let qubit = QubitId::new(3);

        let error = logical_operation_for_test(vec![qubit, qubit]);

        assert!(matches!(
            error,
            Err(FaultTolerantError::DuplicateTarget { .. })
        ));
    }

    fn logical_operation_for_test(
        targets: Vec<QubitId>,
    ) -> FaultTolerantResult<LogicalOperation> {
        LogicalOperation::new(
            LogicalOperationKind::new("test", "operation")?,
            targets,
            Arity::AtLeast(1),
        )
    }

    #[test]
    fn arbitrary_operation_arity_is_supported() {
        let targets = vec![
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
            QubitId::new(3),
            QubitId::new(4),
        ];

        let operation = LogicalOperation::new(
            LogicalOperationKind::new("future", "global")?,
            targets.clone(),
            Arity::AtLeast(1),
        )
        .expect("valid arbitrary-arity operation");

        assert_eq!(operation.targets(), targets.as_slice());
    }

    #[test]
    fn error_budget_rejects_invalid_probability() {
        let mut budget = LogicalErrorBudget::new();

        assert!(
            budget.set_total_failure(-0.1).is_err()
        );

        assert!(
            budget.set_total_failure(1.1).is_err()
        );

        assert!(
            budget.set_total_failure(f64::NAN).is_err()
        );

        assert!(
            budget.set_total_failure(1.0).is_ok()
        );
    }

    #[test]
    fn symbolic_parameters_are_supported() {
        let value =
            FaultTolerantValue::symbol("distance")
                .expect("valid symbol");

        let parameter =
            CodeParameter::new("distance", value)
                .expect("valid parameter");

        assert_eq!(parameter.name(), "distance");
    }

    #[test]
    fn code_is_extensible() {
        let code = QecCode::new(
            "future_qec",
            "next_generation_code",
        )
        .expect("future code must be representable");

        assert_eq!(code.namespace(), "future_qec");
        assert_eq!(code.name(), "next_generation_code");
    }

    #[test]
    fn logical_operation_can_be_parameterized() {
        let mut operation = logical_x(QubitId::new(7))
            .expect("logical X must construct");

        operation
            .set_parameter(
                "implementation_variant".to_owned(),
                FaultTolerantValue::String(
                    "future_variant".to_owned(),
                ),
            )
            .expect("parameter should be accepted");

        assert!(operation.parameters().contains_key(
            "implementation_variant"
        ));
    }

    #[test]
    fn resource_validation_rejects_unknown_qubit() {
        let mut resources = LogicalResourceSet::new();

        resources
            .insert_qubit(QubitId::new(0))
            .expect("resource insertion");

        let operation =
            logical_x(QubitId::new(1))
                .expect("operation construction");

        assert!(
            operation.validate_targets(&resources).is_err()
        );
    }

    #[test]
    fn dialect_accepts_extensible_operation() {
        let qubit = QubitId::new(0);

        let mut dialect = FaultTolerantDialect::new(
            FaultToleranceLevel::FaultTolerant,
            CodeSelectionPolicy::AnyCompatible,
        );

        dialect
            .requirements_mut()
            .logical_qubits_mut()
            .insert_qubit(qubit)
            .expect("logical resource");

        dialect
            .push(FaultTolerantOperation::LogicalGate(
                logical_x(qubit).expect("logical X"),
            ))
            .expect("operation insertion");

        dialect
            .validate()
            .expect("dialect must validate");

        assert_eq!(dialect.operation_count(), 1);
    }

    #[test]
    fn bounded_error_budget_requires_constraint() {
        let dialect = FaultTolerantDialect::new(
            FaultToleranceLevel::ErrorBudgetBounded,
            CodeSelectionPolicy::Automatic,
        );

        assert!(dialect.validate().is_err());
    }

    #[test]
    fn encoded_preparation_requires_target() {
        let error = EncodedPreparation::new(
            Vec::<QubitId>::new(),
            PreparationKind::LogicalZero,
        )
        .expect_err("empty target list must fail");

        assert!(matches!(
            error,
            FaultTolerantError::MissingTargets { .. }
        ));
    }

    #[test]
    fn syndrome_rounds_can_remain_symbolic() {
        let mut syndrome =
            SyndromeRequirement::new(SyndromeKind::Stabilizer);

        syndrome
            .set_rounds(
                FaultTolerantValue::symbol("rounds")
                    .expect("symbol"),
            )
            .expect("symbolic rounds");

        assert!(syndrome.rounds().is_some());
    }

    #[test]
    fn magic_state_fidelity_is_validated() {
        let kind =
            MagicResourceKind::new("magic", "t_state")
                .expect("resource kind");

        let mut requirement =
            MagicStateRequirement::new(
                kind,
                FaultTolerantValue::Unsigned(1),
            )
            .expect("requirement");

        assert!(
            requirement
                .set_minimum_fidelity(0.999)
                .is_ok()
        );

        assert!(
            requirement
                .set_minimum_fidelity(2.0)
                .is_err()
        );
    }

    #[test]
    fn no_fixed_machine_size_exists() {
        let mut resources = LogicalResourceSet::new();

        for index in 0..1024usize {
            resources
                .insert_qubit(QubitId::new(index))
                .expect("unique logical resource");
        }

        assert_eq!(resources.len(), 1024);
    }

    #[test]
    fn standard_code_constructors_are_extensible_descriptors() {
        assert_eq!(
            surface_code()
                .expect("surface code")
                .identity()
                .to_string(),
            "qec.surface"
        );

        assert_eq!(
            steane_code()
                .expect("Steane code")
                .identity()
                .to_string(),
            "qec.steane"
        );
    }
}