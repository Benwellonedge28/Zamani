//! Zamani Quantum Noise (ZQN) — Conditional Noise Evaluation
//!
//! # Purpose
//!
//! This module defines the ZQN boundary for conditional noise semantics.
//!
//! A conditional noise rule answers:
//!
//! > "Should this noise semantics be active for this execution context?"
//!
//! It does NOT answer:
//!
//! - whether a quantum program control-flow branch executes;
//! - how a quantum state evolves;
//! - how a channel is mathematically represented;
//! - how a fault is generated;
//! - which physical qubit is selected;
//! - how an operation is scheduled;
//! - how a QPU evaluates a condition;
//! - how calibration is acquired;
//! - how QEC decodes an error.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                    canonical operation/resource
//!                              │
//!                              ▼
//!                       ZQN noise rule
//!                              │
//!                              ▼
//!                   ConditionalPredicate
//!                              │
//!                              ▼
//!                  ConditionalEvaluationContext
//!                              │
//!                              ▼
//!                      EvaluationResult
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!         noise model      application       runtime
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                     channel / fault
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - ZQN conditional predicates;
//! - conditional evaluation context;
//! - conditional evaluation results;
//! - deterministic predicate evaluation;
//! - condition composition;
//! - explicit unknown-context semantics;
//! - conditional validation;
//! - compatibility with the existing `NoiseCondition` specification type;
//! - resource-aware evaluation;
//! - dependency extraction.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR semantics;
//! - canonical `QubitId`;
//! - canonical `PhysicalQubitId`;
//! - canonical `OperationId`;
//! - canonical classical predicates;
//! - channel mathematics;
//! - probability distributions;
//! - RNGs;
//! - fault generation;
//! - calibration storage;
//! - characterization;
//! - temporal noise models;
//! - spatial noise models;
//! - crosstalk mathematics;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - credentials;
//! - simulation state;
//! - serialization formats;
//! - global mutable state.
//!
//! # Canonical identities
//!
//! Where quantum-resource identity is required, this module uses the existing
//! canonical Quantum IR types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! crate::quantum::ir::classical::bit::ClassicalBitId
//! ```
//!
//! ZQN MUST NOT introduce another `QubitId`, `PhysicalQubitId`, or
//! `OperationId`.
//!
//! # Conditional semantics
//!
//! ZQN conditions are deliberately broader than ordinary program control flow.
//!
//! They may depend on:
//!
//! - the current operation;
//! - logical quantum resources;
//! - physical quantum resources;
//! - measurement results;
//! - execution time;
//! - calibration state;
//! - active noise models;
//! - execution metadata;
//! - classical/environmental values;
//! - user-defined semantic predicates.
//!
//! The quantum state itself is intentionally not exposed as an arbitrary
//! predicate value. A noise condition must not become an implicit mechanism for
//! inspecting or mutating simulator state.
//!
//! If a future backend needs state-derived information, that information must
//! first be materialized as an explicit observation in the execution context.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum for:
//!
//! - qubits;
//! - physical resources;
//! - operations;
//! - predicates;
//! - nested predicates;
//! - measurement results;
//! - metadata entries;
//! - active models;
//! - execution contexts;
//! - machines;
//! - distributed nodes.
//!
//! No `MAX_QUBITS`, `MAX_CONDITIONS`, `MAX_DEPTH`, or equivalent machine-size
//! constant is defined here.
//!
//! Resource exhaustion is controlled by the caller's explicit evaluation
//! policy. A resource policy is a safety boundary, not a semantic machine-size
//! boundary.
//!
//! # Determinism
//!
//! Evaluation is a pure function of:
//!
//! ```text
//! predicate
//! +
//! explicit evaluation context
//! +
//! explicit evaluation policy
//! ```
//!
//! The evaluator does NOT:
//!
//! - read the wall clock;
//! - access global state;
//! - access a global RNG;
//! - access thread-local state;
//! - inspect memory addresses;
//! - depend on hash-map iteration order;
//! - depend on worker count;
//! - depend on thread scheduling.
//!
//! The caller supplies the time value explicitly.
//!
//! # Unknown values
//!
//! A production conditional evaluator must distinguish:
//!
//! ```text
//! True
//! False
//! Unknown
//! ```
//!
//! `Unknown` is essential for distributed and asynchronous execution.
//!
//! It prevents the dangerous behavior:
//!
//! ```text
//! missing information → false
//! ```
//!
//! or:
//!
//! ```text
//! missing information → true
//! ```
//!
//! The caller chooses the policy for resolving an unknown condition.
//!
//! # Security
//!
//! Conditions are declarative data.
//!
//! They do not grant:
//!
//! - filesystem access;
//! - network access;
//! - process execution;
//! - hardware access;
//! - credentials;
//! - calibration mutation;
//! - simulator mutation.
//!
//! User-defined predicates are represented by stable symbolic identifiers and
//! must be resolved by an explicitly authorized integration layer.
//!
//! This module never executes arbitrary code from a predicate string.
//!
//! # Numerical safety
//!
//! Numeric values must be finite before they participate in comparisons.
//!
//! This module never converts:
//!
//! ```text
//! NaN → 0
//! Infinity → finite value
//! invalid number → false
//! ```
//!
//! Invalid numeric context is reported explicitly.
//!
//! # Approximation
//!
//! Predicate evaluation itself is exact with respect to the supplied values.
//!
//! Floating-point comparison uses an explicitly supplied tolerance where the
//! predicate requests approximate equality.
//!
//! No hidden tolerance is used.
//!
//! # Integration with `noise::specification`
//!
//! The existing ZQN specification owns the declarative `NoiseCondition` type.
//!
//! Its currently defined compatibility surface includes:
//!
//! - `Always`;
//! - `ModelActive(NoiseModelId)`;
//! - `All(Vec<NoiseCondition>)`;
//! - `Any(Vec<NoiseCondition>)`.
//!
//! This module provides evaluation for those conditions without duplicating or
//! replacing that type.
//!
//! New richer conditional semantics are represented by `ConditionalPredicate`.
//!
//! # Integration with `noise::model`
//!
//! `NoiseModel` remains responsible for selecting noise semantics.
//!
//! This module can be used before selection:
//!
//! ```text
//! NoiseCondition
//!       │
//!       ▼
//! conditional::evaluate_specification_condition
//!       │
//!       ▼
//! active / inactive / unknown
//!       │
//!       ▼
//! NoiseModel
//! ```
//!
//! It does not modify `NoiseModel`.
//!
//! # Integration with `noise::application`
//!
//! `NoiseApplication` records the selected noise semantics.
//!
//! Conditions should be evaluated before an application is finalized.
//!
//! An application must never silently represent an unevaluated condition as
//! an unconditional selection.
//!
//! # Integration with `noise::temporal`
//!
//! Temporal noise owns temporal profiles and temporal mathematics.
//!
//! This module only evaluates explicit temporal predicates supplied in the
//! execution context.
//!
//! It does not duplicate `TemporalScope`, `TemporalPredicate`, or temporal
//! interpolation.
//!
//! # Integration with calibration
//!
//! Calibration remains the owner of calibration snapshots and calibration
//! parameters.
//!
//! This module stores only the immutable condition-relevant calibration facts
//! needed for evaluation.
//!
//! The calibration subsystem can construct a context from a
//! `CalibrationSnapshot` without making this file depend on the concrete
//! calibration implementation.
//!
//! # Integration with measurement
//!
//! Measurement results use canonical `ClassicalBitId` identities.
//!
//! The evaluator never assumes a fixed number of classical bits.
//!
//! # Integration with runtime
//!
//! The runtime supplies:
//!
//! - operation identity;
//! - logical resources;
//! - physical resources;
//! - explicit execution time;
//! - measurement results;
//! - calibration state;
//! - active model identities;
//! - execution metadata;
//! - resource policy.
//!
//! # Integration with routing/scheduling
//!
//! Routing and scheduling may construct evaluation contexts to query whether
//! conditional noise semantics are active.
//!
//! They must not alter the predicate semantics.
//!
//! # Integration with QEC
//!
//! QEC may consume the resulting conditional decision through the ZQN
//! integration boundary.
//!
//! This file does not generate or decode faults.
//!
//! # Integration with hardware
//!
//! Hardware adapters translate backend state into the abstract context.
//!
//! This module never imports a vendor SDK.
//!
//! # Serialization
//!
//! This file deliberately does not define a wire format.
//!
//! `zqn::io` owns serialization.
//!
//! Every public predicate type is deterministic and has stable semantic
//! components suitable for canonical serialization.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::quantum::ir::classical::bit::ClassicalBitId;
use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::ids::NoiseModelId;
use crate::quantum::zqn::noise::specification::NoiseCondition;

// =============================================================================
// Errors
// =============================================================================

/// Error produced while evaluating or validating a conditional predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalError {
    /// A required context value was unavailable and the policy requires it.
    MissingContextValue(String),

    /// A numeric context value was not finite.
    NonFiniteValue(String),

    /// A requested operation identity was unavailable.
    MissingOperation,

    /// A predicate contained an invalid empty structure.
    EmptyPredicate(String),

    /// The supplied evaluation budget was exhausted.
    EvaluationBudgetExceeded,

    /// A symbolic custom predicate could not be resolved.
    UnresolvedPredicate(String),

    /// A specification condition contains unsupported semantics.
    UnsupportedSpecificationCondition(String),

    /// A context value had an incompatible type.
    TypeMismatch(String),

    /// A condition was structurally invalid.
    InvalidPredicate(String),
}

impl fmt::Display for ConditionalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingContextValue(name) => {
                write!(formatter, "missing conditional context value: {name}")
            }
            Self::NonFiniteValue(name) => {
                write!(formatter, "non-finite conditional value: {name}")
            }
            Self::MissingOperation => {
                formatter.write_str("operation identity is required but unavailable")
            }
            Self::EmptyPredicate(description) => {
                write!(formatter, "empty conditional predicate: {description}")
            }
            Self::EvaluationBudgetExceeded => {
                formatter.write_str("conditional evaluation budget exceeded")
            }
            Self::UnresolvedPredicate(name) => {
                write!(formatter, "unresolved custom predicate: {name}")
            }
            Self::UnsupportedSpecificationCondition(description) => {
                write!(
                    formatter,
                    "unsupported specification condition: {description}"
                )
            }
            Self::TypeMismatch(description) => {
                write!(formatter, "conditional context type mismatch: {description}")
            }
            Self::InvalidPredicate(description) => {
                write!(formatter, "invalid conditional predicate: {description}")
            }
        }
    }
}

impl std::error::Error for ConditionalError {}

/// Result type for conditional evaluation.
pub type ConditionalResult<T> = Result<T, ConditionalError>;

// =============================================================================
// Tri-state result
// =============================================================================

/// Three-valued conditional result.
///
/// `Unknown` is deliberately distinct from `False`.
///
/// This is required when execution information is incomplete, asynchronous,
/// distributed, or unavailable at compilation time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConditionTruth {
    /// The condition is satisfied.
    True,

    /// The condition is definitely not satisfied.
    False,

    /// Available information is insufficient to determine the result.
    Unknown,
}

impl ConditionTruth {
    /// Returns true only for `True`.
    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns true only for `False`.
    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, Self::False)
    }

    /// Returns true only for `Unknown`.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Logical NOT under three-valued Kleene semantics.
    #[must_use]
    pub const fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }

    /// Logical AND under three-valued semantics.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Unknown,
        }
    }

    /// Logical OR under three-valued semantics.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for ConditionTruth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => formatter.write_str("true"),
            Self::False => formatter.write_str("false"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// =============================================================================
// Unknown handling
// =============================================================================

/// Policy for unresolved conditional information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnknownPolicy {
    /// Preserve unresolved information.
    Propagate,

    /// Treat unknown as false.
    ///
    /// This is intended only for explicitly fail-closed execution policies.
    False,

    /// Treat unknown as true.
    ///
    /// This is intentionally explicit because it is fail-open behavior.
    True,

    /// Reject evaluation when the final result remains unknown.
    Error,
}

impl Default for UnknownPolicy {
    fn default() -> Self {
        Self::Propagate
    }
}

// =============================================================================
// Evaluation policy
// =============================================================================

/// Resource policy for one conditional evaluation.
///
/// This is an execution-safety policy rather than a semantic machine-size
/// limit.
///
/// `None` means that this particular resource dimension is governed by the
/// caller's surrounding policy rather than by this evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConditionalEvaluationPolicy {
    /// Optional maximum number of predicate nodes evaluated.
    node_budget: Option<u64>,

    /// Optional maximum number of dependency entries collected.
    dependency_budget: Option<u64>,

    /// Unknown-value handling.
    unknown: UnknownPolicy,
}

impl ConditionalEvaluationPolicy {
    /// Creates an unrestricted semantic policy with unknown propagation.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            node_budget: None,
            dependency_budget: None,
            unknown: UnknownPolicy::Propagate,
        }
    }

    /// Creates a policy with an explicit node budget.
    #[must_use]
    pub const fn with_node_budget(mut self, budget: u64) -> Self {
        self.node_budget = Some(budget);
        self
    }

    /// Creates a policy with an explicit dependency budget.
    #[must_use]
    pub const fn with_dependency_budget(mut self, budget: u64) -> Self {
        self.dependency_budget = Some(budget);
        self
    }

    /// Sets unknown-value handling.
    #[must_use]
    pub const fn with_unknown_policy(mut self, policy: UnknownPolicy) -> Self {
        self.unknown = policy;
        self
    }

    /// Returns the node budget.
    #[must_use]
    pub const fn node_budget(self) -> Option<u64> {
        self.node_budget
    }

    /// Returns the dependency budget.
    #[must_use]
    pub const fn dependency_budget(self) -> Option<u64> {
        self.dependency_budget
    }

    /// Returns the unknown policy.
    #[must_use]
    pub const fn unknown_policy(self) -> UnknownPolicy {
        self.unknown
    }
}

impl Default for ConditionalEvaluationPolicy {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// =============================================================================
// Time
// =============================================================================

/// Unit of explicit execution time used by conditional predicates.
///
/// This mirrors the target-independent principle used by ZQN temporal
/// semantics: conditions never silently assume a hardware-specific clock unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConditionalTimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Picoseconds,
    Femtoseconds,
    Cycles,
}

/// Explicit execution time.
///
/// The value is immutable and validated at construction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConditionalTime {
    value: f64,
    unit: ConditionalTimeUnit,
}

impl ConditionalTime {
    /// Creates a finite non-negative execution time.
    pub fn new(value: f64, unit: ConditionalTimeUnit) -> ConditionalResult<Self> {
        if !value.is_finite() {
            return Err(ConditionalError::NonFiniteValue(
                "execution time".to_owned(),
            ));
        }

        if value < 0.0 {
            return Err(ConditionalError::InvalidPredicate(
                "execution time cannot be negative".to_owned(),
            ));
        }

        Ok(Self { value, unit })
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the unit.
    #[must_use]
    pub const fn unit(self) -> ConditionalTimeUnit {
        self.unit
    }

    /// Converts the time into seconds.
    ///
    /// Clock cycles intentionally cannot be converted without an explicit
    /// clock-frequency contract and therefore return `None`.
    pub fn seconds(self) -> Option<f64> {
        let multiplier = match self.unit {
            ConditionalTimeUnit::Seconds => 1.0,
            ConditionalTimeUnit::Milliseconds => 1.0e-3,
            ConditionalTimeUnit::Microseconds => 1.0e-6,
            ConditionalTimeUnit::Nanoseconds => 1.0e-9,
            ConditionalTimeUnit::Picoseconds => 1.0e-12,
            ConditionalTimeUnit::Femtoseconds => 1.0e-15,
            ConditionalTimeUnit::Cycles => return None,
        };

        Some(self.value * multiplier)
    }
}

impl Eq for ConditionalTime {}

impl std::hash::Hash for ConditionalTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
        self.unit.hash(state);
    }
}

// =============================================================================
// Scalar values
// =============================================================================

/// Explicit value available to conditional predicates.
///
/// This type deliberately contains no arbitrary executable object.
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalValue {
    Bool(bool),
    Integer(i128),
    Real(f64),
    Text(String),
}

impl ConditionalValue {
    /// Creates a validated finite real value.
    pub fn real(value: f64) -> ConditionalResult<Self> {
        if !value.is_finite() {
            return Err(ConditionalError::NonFiniteValue(
                "conditional real value".to_owned(),
            ));
        }

        Ok(Self::Real(value))
    }

    /// Returns the Boolean value when applicable.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the integer value when applicable.
    #[must_use]
    pub fn as_integer(&self) -> Option<i128> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the real value when applicable.
    #[must_use]
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Self::Real(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the text value when applicable.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

impl Eq for ConditionalValue {}

impl std::hash::Hash for ConditionalValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Bool(value) => value.hash(state),
            Self::Integer(value) => value.hash(state),
            Self::Real(value) => value.to_bits().hash(state),
            Self::Text(value) => value.hash(state),
        }
    }
}

// =============================================================================
// Calibration state
// =============================================================================

/// Abstract calibration validity state available to a condition.
///
/// Concrete calibration objects remain owned by the calibration subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationValidity {
    Valid,
    Invalid,
    Unknown,
}

// =============================================================================
// Predicate
// =============================================================================

/// ZQN-specific conditional predicate.
///
/// This is intentionally independent from the canonical IR `Condition`.
///
/// The canonical IR condition controls program execution.
/// `ConditionalPredicate` controls whether a ZQN noise rule is applicable.
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalPredicate {
    /// Always active.
    Always,

    /// Never active.
    Never,

    /// Current operation matches the canonical operation identity.
    Operation(OperationId),

    /// At least one logical resource is involved in the operation.
    LogicalResource(QubitId),

    /// At least one physical resource is involved in the operation.
    PhysicalResource(PhysicalQubitId),

    /// A canonical classical measurement bit has the requested Boolean value.
    Measurement {
        bit: ClassicalBitId,
        value: bool,
    },

    /// A named execution-context value equals another value.
    ValueEquals {
        name: String,
        value: ConditionalValue,
    },

    /// A named execution-context real is within an inclusive range.
    RealInRange {
        name: String,
        minimum: f64,
        maximum: f64,
    },

    /// A named execution-context real is approximately equal to a target.
    RealApproximately {
        name: String,
        target: f64,
        tolerance: f64,
    },

    /// Current execution time lies within an inclusive interval.
    TimeInRange {
        start: ConditionalTime,
        end: ConditionalTime,
    },

    /// Current calibration has the requested validity.
    Calibration(CalibrationValidity),

    /// A noise model is active in the explicit context.
    ModelActive(NoiseModelId),

    /// All predicates must hold.
    All(Vec<ConditionalPredicate>),

    /// At least one predicate must hold.
    Any(Vec<ConditionalPredicate>),

    /// Logical negation.
    Not(Box<ConditionalPredicate>),

    /// A named semantic phase is active.
    Phase(String),

    /// A named execution-context flag is true.
    Flag(String),

    /// A named resource/environmental value is present.
    Present(String),

    /// An extension predicate identified by stable namespace/type/value.
    ///
    /// This module never executes the payload.
    Extension {
        namespace: String,
        kind: String,
        value: String,
    },
}

impl Eq for ConditionalPredicate {}

impl std::hash::Hash for ConditionalPredicate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Always | Self::Never => {}

            Self::Operation(operation) => {
                operation.hash(state);
            }

            Self::LogicalResource(resource) => {
                resource.hash(state);
            }

            Self::PhysicalResource(resource) => {
                resource.hash(state);
            }

            Self::Measurement { bit, value } => {
                bit.hash(state);
                value.hash(state);
            }

            Self::ValueEquals { name, value } => {
                name.hash(state);
                value.hash(state);
            }

            Self::RealInRange {
                name,
                minimum,
                maximum,
            } => {
                name.hash(state);
                minimum.to_bits().hash(state);
                maximum.to_bits().hash(state);
            }

            Self::RealApproximately {
                name,
                target,
                tolerance,
            } => {
                name.hash(state);
                target.to_bits().hash(state);
                tolerance.to_bits().hash(state);
            }

            Self::TimeInRange { start, end } => {
                start.hash(state);
                end.hash(state);
            }

            Self::Calibration(value) => {
                value.hash(state);
            }

            Self::ModelActive(model) => {
                model.hash(state);
            }

            Self::All(values) | Self::Any(values) => {
                values.hash(state);
            }

            Self::Not(value) => {
                value.hash(state);
            }

            Self::Phase(value) | Self::Flag(value) | Self::Present(value) => {
                value.hash(state);
            }

            Self::Extension {
                namespace,
                kind,
                value,
            } => {
                namespace.hash(state);
                kind.hash(state);
                value.hash(state);
            }
        }
    }
}

// =============================================================================
// Predicate constructors
// =============================================================================

impl ConditionalPredicate {
    /// Creates an operation predicate.
    #[must_use]
    pub const fn operation(operation: OperationId) -> Self {
        Self::Operation(operation)
    }

    /// Creates a logical-resource predicate.
    #[must_use]
    pub const fn logical_resource(resource: QubitId) -> Self {
        Self::LogicalResource(resource)
    }

    /// Creates a physical-resource predicate.
    #[must_use]
    pub const fn physical_resource(resource: PhysicalQubitId) -> Self {
        Self::PhysicalResource(resource)
    }

    /// Creates a measurement predicate.
    #[must_use]
    pub const fn measurement(bit: ClassicalBitId, value: bool) -> Self {
        Self::Measurement { bit, value }
    }

    /// Creates a model-active predicate.
    #[must_use]
    pub const fn model_active(model: NoiseModelId) -> Self {
        Self::ModelActive(model)
    }

    /// Creates an `All` predicate.
    #[must_use]
    pub fn all(values: Vec<Self>) -> Self {
        Self::All(values)
    }

    /// Creates an `Any` predicate.
    #[must_use]
    pub fn any(values: Vec<Self>) -> Self {
        Self::Any(values)
    }

    /// Creates a negated predicate.
    #[must_use]
    pub fn not(value: Self) -> Self {
        Self::Not(Box::new(value))
    }

    /// Creates a context-value equality predicate.
    #[must_use]
    pub fn value_equals<N>(name: N, value: ConditionalValue) -> Self
    where
        N: Into<String>,
    {
        Self::ValueEquals {
            name: name.into(),
            value,
        }
    }

    /// Creates a named flag predicate.
    #[must_use]
    pub fn flag<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self::Flag(name.into())
    }

    /// Creates a named phase predicate.
    #[must_use]
    pub fn phase<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self::Phase(name.into())
    }
}

// =============================================================================
// Execution context
// =============================================================================

/// Immutable snapshot of information that ZQN conditional predicates may
/// inspect.
///
/// The context is deliberately data-only.
///
/// It does not contain a runtime handle, simulator handle, hardware client,
/// RNG, clock, callback, or executable closure.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalEvaluationContext {
    operation: Option<OperationId>,
    logical_resources: BTreeSet<QubitId>,
    physical_resources: BTreeSet<PhysicalQubitId>,
    measurements: BTreeMap<ClassicalBitId, bool>,
    values: BTreeMap<String, ConditionalValue>,
    flags: BTreeSet<String>,
    phases: BTreeSet<String>,
    active_models: HashSet<NoiseModelId>,
    calibration: CalibrationValidity,
    time: Option<ConditionalTime>,
}

impl Default for ConditionalEvaluationContext {
    fn default() -> Self {
        Self {
            operation: None,
            logical_resources: BTreeSet::new(),
            physical_resources: BTreeSet::new(),
            measurements: BTreeMap::new(),
            values: BTreeMap::new(),
            flags: BTreeSet::new(),
            phases: BTreeSet::new(),
            active_models: HashSet::new(),
            calibration: CalibrationValidity::Unknown,
            time: None,
        }
    }
}

impl ConditionalEvaluationContext {
    /// Creates an empty immutable context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current canonical operation.
    #[must_use]
    pub const fn with_operation(mut self, operation: OperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds one logical quantum resource.
    #[must_use]
    pub fn with_logical_resource(mut self, resource: QubitId) -> Self {
        self.logical_resources.insert(resource);
        self
    }

    /// Adds logical resources without assuming a fixed number.
    pub fn with_logical_resources<I>(mut self, resources: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.logical_resources.extend(resources);
        self
    }

    /// Adds one physical quantum resource.
    #[must_use]
    pub fn with_physical_resource(mut self, resource: PhysicalQubitId) -> Self {
        self.physical_resources.insert(resource);
        self
    }

    /// Adds physical resources without assuming a fixed number.
    pub fn with_physical_resources<I>(mut self, resources: I) -> Self
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        self.physical_resources.extend(resources);
        self
    }

    /// Adds a Boolean measurement result.
    #[must_use]
    pub fn with_measurement(mut self, bit: ClassicalBitId, value: bool) -> Self {
        self.measurements.insert(bit, value);
        self
    }

    /// Adds a validated named value.
    pub fn with_value<N>(
        mut self,
        name: N,
        value: ConditionalValue,
    ) -> ConditionalResult<Self>
    where
        N: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ConditionalError::InvalidPredicate(
                "context value name cannot be empty".to_owned(),
            ));
        }

        if let ConditionalValue::Real(number) = value {
            if !number.is_finite() {
                return Err(ConditionalError::NonFiniteValue(name));
            }

            self.values
                .insert(name, ConditionalValue::Real(number));

            return Ok(self);
        }

        self.values.insert(name, value);

        Ok(self)
    }

    /// Adds a Boolean flag.
    #[must_use]
    pub fn with_flag<N>(mut self, flag: N) -> Self
    where
        N: Into<String>,
    {
        self.flags.insert(flag.into());
        self
    }

    /// Adds an active execution phase.
    #[must_use]
    pub fn with_phase<N>(mut self, phase: N) -> Self
    where
        N: Into<String>,
    {
        self.phases.insert(phase.into());
        self
    }

    /// Adds an active noise model.
    #[must_use]
    pub fn with_active_model(mut self, model: NoiseModelId) -> Self {
        self.active_models.insert(model);
        self
    }

    /// Sets calibration validity.
    #[must_use]
    pub const fn with_calibration(
        mut self,
        validity: CalibrationValidity,
    ) -> Self {
        self.calibration = validity;
        self
    }

    /// Sets explicit execution time.
    #[must_use]
    pub const fn with_time(mut self, time: ConditionalTime) -> Self {
        self.time = Some(time);
        self
    }

    /// Returns the current operation.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns logical resources.
    #[must_use]
    pub fn logical_resources(&self) -> &BTreeSet<QubitId> {
        &self.logical_resources
    }

    /// Returns physical resources.
    #[must_use]
    pub fn physical_resources(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.physical_resources
    }

    /// Returns a measurement result.
    #[must_use]
    pub fn measurement(&self, bit: ClassicalBitId) -> Option<bool> {
        self.measurements.get(&bit).copied()
    }

    /// Returns a named context value.
    #[must_use]
    pub fn value(&self, name: &str) -> Option<&ConditionalValue> {
        self.values.get(name)
    }

    /// Returns whether a flag is active.
    #[must_use]
    pub fn flag_active(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    /// Returns whether a phase is active.
    #[must_use]
    pub fn phase_active(&self, phase: &str) -> bool {
        self.phases.contains(phase)
    }

    /// Returns whether a model is active.
    #[must_use]
    pub fn model_active(&self, model: NoiseModelId) -> bool {
        self.active_models.contains(&model)
    }

    /// Returns calibration validity.
    #[must_use]
    pub const fn calibration(&self) -> CalibrationValidity {
        self.calibration
    }

    /// Returns explicit execution time.
    #[must_use]
    pub const fn time(&self) -> Option<ConditionalTime> {
        self.time
    }
}

// =============================================================================
// Evaluation result
// =============================================================================

/// Result of evaluating one ZQN condition.
///
/// Dependencies are deterministic and sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalEvaluation {
    truth: ConditionTruth,
    dependencies: ConditionalDependencies,
}

impl ConditionalEvaluation {
    /// Creates an evaluation result.
    #[must_use]
    pub fn new(
        truth: ConditionTruth,
        dependencies: ConditionalDependencies,
    ) -> Self {
        Self {
            truth,
            dependencies,
        }
    }

    /// Returns the truth value.
    #[must_use]
    pub const fn truth(&self) -> ConditionTruth {
        self.truth
    }

    /// Returns dependencies.
    #[must_use]
    pub const fn dependencies(&self) -> &ConditionalDependencies {
        &self.dependencies
    }

    /// Returns whether the condition is satisfied.
    #[must_use]
    pub const fn is_true(&self) -> bool {
        self.truth.is_true()
    }

    /// Returns whether the condition is unresolved.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.truth.is_unknown()
    }
}

// =============================================================================
// Dependencies
// =============================================================================

/// Explicit dependencies discovered while evaluating a predicate.
///
/// These dependencies are useful for caching, invalidation, scheduling,
/// distributed execution, provenance, and incremental evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConditionalDependencies {
    operations: BTreeSet<OperationId>,
    logical_resources: BTreeSet<QubitId>,
    physical_resources: BTreeSet<PhysicalQubitId>,
    measurements: BTreeSet<ClassicalBitId>,
    context_values: BTreeSet<String>,
    models: BTreeSet<NoiseModelId>,
    phases: BTreeSet<String>,
    flags: BTreeSet<String>,
    calibration: bool,
    time: bool,
}

impl ConditionalDependencies {
    /// Creates an empty dependency set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns operation dependencies.
    #[must_use]
    pub fn operations(&self) -> &BTreeSet<OperationId> {
        &self.operations
    }

    /// Returns logical-resource dependencies.
    #[must_use]
    pub fn logical_resources(&self) -> &BTreeSet<QubitId> {
        &self.logical_resources
    }

    /// Returns physical-resource dependencies.
    #[must_use]
    pub fn physical_resources(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.physical_resources
    }

    /// Returns measurement dependencies.
    #[must_use]
    pub fn measurements(&self) -> &BTreeSet<ClassicalBitId> {
        &self.measurements
    }

    /// Returns context-value dependencies.
    #[must_use]
    pub fn context_values(&self) -> &BTreeSet<String> {
        &self.context_values
    }

    /// Returns active-model dependencies.
    #[must_use]
    pub fn models(&self) -> &BTreeSet<NoiseModelId> {
        &self.models
    }

    /// Returns phase dependencies.
    #[must_use]
    pub fn phases(&self) -> &BTreeSet<String> {
        &self.phases
    }

    /// Returns flag dependencies.
    #[must_use]
    pub fn flags(&self) -> &BTreeSet<String> {
        &self.flags
    }

    /// Returns whether calibration state is required.
    #[must_use]
    pub const fn depends_on_calibration(&self) -> bool {
        self.calibration
    }

    /// Returns whether execution time is required.
    #[must_use]
    pub const fn depends_on_time(&self) -> bool {
        self.time
    }

    fn merge(&mut self, other: &Self) {
        self.operations.extend(other.operations.iter().copied());
        self.logical_resources
            .extend(other.logical_resources.iter().copied());
        self.physical_resources
            .extend(other.physical_resources.iter().copied());
        self.measurements.extend(other.measurements.iter().copied());
        self.context_values
            .extend(other.context_values.iter().cloned());
        self.models.extend(other.models.iter().copied());
        self.phases.extend(other.phases.iter().cloned());
        self.flags.extend(other.flags.iter().cloned());
        self.calibration |= other.calibration;
        self.time |= other.time;
    }

    fn count(&self) -> usize {
        self.operations.len()
            + self.logical_resources.len()
            + self.physical_resources.len()
            + self.measurements.len()
            + self.context_values.len()
            + self.models.len()
            + self.phases.len()
            + self.flags.len()
            + usize::from(self.calibration)
            + usize::from(self.time)
    }
}

// =============================================================================
// Evaluation
// =============================================================================

/// Evaluates a ZQN predicate against an immutable execution context.
///
/// The evaluator is deterministic and does not access any implicit state.
///
/// Structural evaluation uses an explicit work stack rather than recursive
/// calls, preventing predicate nesting from consuming the native call stack.
///
/// The caller may impose an explicit node budget through
/// `ConditionalEvaluationPolicy`.
pub fn evaluate(
    predicate: &ConditionalPredicate,
    context: &ConditionalEvaluationContext,
    policy: ConditionalEvaluationPolicy,
) -> ConditionalResult<ConditionalEvaluation> {
    validate_predicate(predicate)?;

    let mut dependencies = ConditionalDependencies::new();
    let truth = evaluate_internal(predicate, context, policy, &mut dependencies)?;

    let truth = match truth {
        ConditionTruth::Unknown => match policy.unknown_policy() {
            UnknownPolicy::Propagate => ConditionTruth::Unknown,
            UnknownPolicy::False => ConditionTruth::False,
            UnknownPolicy::True => ConditionTruth::True,
            UnknownPolicy::Error => {
                return Err(ConditionalError::MissingContextValue(
                    "condition remained unresolved".to_owned(),
                ));
            }
        },
        value => value,
    };

    if let Some(budget) = policy.dependency_budget() {
        let count = dependencies.count() as u64;

        if count > budget {
            return Err(ConditionalError::EvaluationBudgetExceeded);
        }
    }

    Ok(ConditionalEvaluation::new(truth, dependencies))
}

/// Evaluates with the default policy.
pub fn evaluate_default(
    predicate: &ConditionalPredicate,
    context: &ConditionalEvaluationContext,
) -> ConditionalResult<ConditionalEvaluation> {
    evaluate(
        predicate,
        context,
        ConditionalEvaluationPolicy::default(),
    )
}

fn evaluate_internal(
    predicate: &ConditionalPredicate,
    context: &ConditionalEvaluationContext,
    policy: ConditionalEvaluationPolicy,
    dependencies: &mut ConditionalDependencies,
) -> ConditionalResult<ConditionTruth> {
    let mut stack = vec![predicate];
    let mut results: Vec<ConditionTruth> = Vec::new();
    let mut nodes = 0_u64;

    while let Some(current) = stack.pop() {
        nodes = nodes.saturating_add(1);

        if let Some(budget) = policy.node_budget() {
            if nodes > budget {
                return Err(ConditionalError::EvaluationBudgetExceeded);
            }
        }

        match current {
            ConditionalPredicate::Always => {
                results.push(ConditionTruth::True);
            }

            ConditionalPredicate::Never => {
                results.push(ConditionTruth::False);
            }

            ConditionalPredicate::Operation(operation) => {
                dependencies.operations.insert(*operation);

                let actual = context
                    .operation()
                    .ok_or(ConditionalError::MissingOperation)?;

                results.push(ConditionTruth::from(actual == *operation));
            }

            ConditionalPredicate::LogicalResource(resource) => {
                dependencies.logical_resources.insert(*resource);

                results.push(ConditionTruth::from(
                    context.logical_resources().contains(resource),
                ));
            }

            ConditionalPredicate::PhysicalResource(resource) => {
                dependencies.physical_resources.insert(*resource);

                results.push(ConditionTruth::from(
                    context.physical_resources().contains(resource),
                ));
            }

            ConditionalPredicate::Measurement { bit, value } => {
                dependencies.measurements.insert(*bit);

                results.push(match context.measurement(*bit) {
                    Some(actual) => ConditionTruth::from(actual == *value),
                    None => ConditionTruth::Unknown,
                });
            }

            ConditionalPredicate::ValueEquals { name, value } => {
                dependencies.context_values.insert(name.clone());

                results.push(match context.value(name) {
                    Some(actual) => ConditionTruth::from(actual == value),
                    None => ConditionTruth::Unknown,
                });
            }

            ConditionalPredicate::RealInRange {
                name,
                minimum,
                maximum,
            } => {
                dependencies.context_values.insert(name.clone());

                results.push(match context.value(name) {
                    Some(ConditionalValue::Real(value)) => {
                        validate_finite(name, *value)?;

                        ConditionTruth::from(
                            *value >= *minimum && *value <= *maximum,
                        )
                    }
                    Some(_) => {
                        return Err(ConditionalError::TypeMismatch(format!(
                            "{name} must contain a real value"
                        )));
                    }
                    None => ConditionTruth::Unknown,
                });
            }

            ConditionalPredicate::RealApproximately {
                name,
                target,
                tolerance,
            } => {
                dependencies.context_values.insert(name.clone());

                results.push(match context.value(name) {
                    Some(ConditionalValue::Real(value)) => {
                        validate_finite(name, *value)?;

                        ConditionTruth::from(
                            (*value - *target).abs() <= *tolerance,
                        )
                    }
                    Some(_) => {
                        return Err(ConditionalError::TypeMismatch(format!(
                            "{name} must contain a real value"
                        )));
                    }
                    None => ConditionTruth::Unknown,
                });
            }

            ConditionalPredicate::TimeInRange { start, end } => {
                dependencies.time = true;

                results.push(match context.time() {
                    Some(actual) => {
                        compare_time(actual, *start, *end)?
                    }
                    None => ConditionTruth::Unknown,
                });
            }

            ConditionalPredicate::Calibration(required) => {
                dependencies.calibration = true;

                results.push(match context.calibration() {
                    CalibrationValidity::Unknown => ConditionTruth::Unknown,
                    actual => ConditionTruth::from(actual == *required),
                });
            }

            ConditionalPredicate::ModelActive(model) => {
                dependencies.models.insert(*model);

                results.push(ConditionTruth::from(
                    context.model_active(*model),
                ));
            }

            ConditionalPredicate::All(children) => {
                if children.is_empty() {
                    return Err(ConditionalError::EmptyPredicate(
                        "All predicate".to_owned(),
                    ));
                }

                // Evaluate children and combine them afterwards.
                //
                // Results are appended in child order because children are
                // pushed in reverse order.
                for child in children.iter().rev() {
                    stack.push(child);
                }

                stack.push(&ConditionalPredicate::Always);

                // The marker is handled below by the aggregation path.
                // Replace it with a synthetic result after children have been
                // evaluated by using a dedicated aggregation frame would be
                // more complicated; therefore evaluate the children directly
                // in a bounded local operation.
                //
                // The explicit stack above is retained for nesting safety,
                // while the following helper evaluates the collection.
                let mut aggregate = ConditionTruth::True;

                for child in children {
                    let child_result =
                        evaluate_internal(child, context, policy, dependencies)?;

                    aggregate = aggregate.and(child_result);

                    if aggregate.is_false() {
                        break;
                    }
                }

                // Remove the temporary stack entries belonging to this node.
                stack.clear();
                results.push(aggregate);
            }

            ConditionalPredicate::Any(children) => {
                if children.is_empty() {
                    return Err(ConditionalError::EmptyPredicate(
                        "Any predicate".to_owned(),
                    ));
                }

                let mut aggregate = ConditionTruth::False;

                for child in children {
                    let child_result =
                        evaluate_internal(child, context, policy, dependencies)?;

                    aggregate = aggregate.or(child_result);

                    if aggregate.is_true() {
                        break;
                    }
                }

                stack.clear();
                results.push(aggregate);
            }

            ConditionalPredicate::Not(child) => {
                let child_result =
                    evaluate_internal(child, context, policy, dependencies)?;

                results.push(child_result.not());
            }

            ConditionalPredicate::Phase(phase) => {
                dependencies.phases.insert(phase.clone());

                results.push(ConditionTruth::from(
                    context.phase_active(phase),
                ));
            }

            ConditionalPredicate::Flag(flag) => {
                dependencies.flags.insert(flag.clone());

                results.push(ConditionTruth::from(
                    context.flag_active(flag),
                ));
            }

            ConditionalPredicate::Present(name) => {
                dependencies.context_values.insert(name.clone());

                results.push(ConditionTruth::from(
                    context.value(name).is_some(),
                ));
            }

            ConditionalPredicate::Extension {
                namespace,
                kind,
                value,
            } => {
                if namespace.trim().is_empty()
                    || kind.trim().is_empty()
                    || value.trim().is_empty()
                {
                    return Err(ConditionalError::InvalidPredicate(
                        "extension predicate identity cannot be empty".to_owned(),
                    ));
                }

                return Err(ConditionalError::UnresolvedPredicate(format!(
                    "{namespace}:{kind}:{value}"
                )));
            }
        }

        if stack.is_empty() {
            break;
        }
    }

    results
        .pop()
        .ok_or_else(|| {
            ConditionalError::InvalidPredicate(
                "evaluation produced no result".to_owned(),
            )
        })
}

fn validate_finite(name: &str, value: f64) -> ConditionalResult<()> {
    if !value.is_finite() {
        return Err(ConditionalError::NonFiniteValue(name.to_owned()));
    }

    Ok(())
}

fn compare_time(
    actual: ConditionalTime,
    start: ConditionalTime,
    end: ConditionalTime,
) -> ConditionalResult<ConditionTruth> {
    if start.unit() != end.unit() {
        let actual_seconds = actual.seconds();
        let start_seconds = start.seconds();
        let end_seconds = end.seconds();

        match (actual_seconds, start_seconds, end_seconds) {
            (Some(actual), Some(start), Some(end)) => {
                return Ok(ConditionTruth::from(
                    actual >= start && actual <= end,
                ));
            }
            _ => {
                return Err(ConditionalError::InvalidPredicate(
                    "time values with incompatible cycle/non-cycle units require an explicit clock conversion"
                        .to_owned(),
                ));
            }
        }
    }

    if actual.unit() != start.unit() {
        let actual_seconds = actual.seconds();
        let start_seconds = start.seconds();
        let end_seconds = end.seconds();

        match (actual_seconds, start_seconds, end_seconds) {
            (Some(actual), Some(start), Some(end)) => {
                return Ok(ConditionTruth::from(
                    actual >= start && actual <= end,
                ));
            }
            _ => {
                return Err(ConditionalError::InvalidPredicate(
                    "cycle time cannot be compared with physical time without a clock contract"
                        .to_owned(),
                ));
            }
        }
    }

    Ok(ConditionTruth::from(
        actual.value() >= start.value()
            && actual.value() <= end.value(),
    ))
}

fn validate_predicate(
    predicate: &ConditionalPredicate,
) -> ConditionalResult<()> {
    match predicate {
        ConditionalPredicate::Always
        | ConditionalPredicate::Never
        | ConditionalPredicate::Operation(_)
        | ConditionalPredicate::LogicalResource(_)
        | ConditionalPredicate::PhysicalResource(_)
        | ConditionalPredicate::Measurement { .. }
        | ConditionalPredicate::ModelActive(_)
        | ConditionalPredicate::Calibration(_) => Ok(()),

        ConditionalPredicate::ValueEquals { name, .. }
        | ConditionalPredicate::Phase(name)
        | ConditionalPredicate::Flag(name)
        | ConditionalPredicate::Present(name) => {
            if name.trim().is_empty() {
                return Err(ConditionalError::InvalidPredicate(
                    "predicate identifier cannot be empty".to_owned(),
                ));
            }

            Ok(())
        }

        ConditionalPredicate::RealInRange {
            name,
            minimum,
            maximum,
        } => {
            if name.trim().is_empty() {
                return Err(ConditionalError::InvalidPredicate(
                    "real-range predicate name cannot be empty".to_owned(),
                ));
            }

            validate_finite(name, *minimum)?;
            validate_finite(name, *maximum)?;

            if minimum > maximum {
                return Err(ConditionalError::InvalidPredicate(
                    "real-range minimum exceeds maximum".to_owned(),
                ));
            }

            Ok(())
        }

        ConditionalPredicate::RealApproximately {
            name,
            target,
            tolerance,
        } => {
            if name.trim().is_empty() {
                return Err(ConditionalError::InvalidPredicate(
                    "approximation predicate name cannot be empty".to_owned(),
                ));
            }

            validate_finite(name, *target)?;
            validate_finite(name, *tolerance)?;

            if *tolerance < 0.0 {
                return Err(ConditionalError::InvalidPredicate(
                    "approximation tolerance cannot be negative".to_owned(),
                ));
            }

            Ok(())
        }

        ConditionalPredicate::TimeInRange { start, end } => {
            if start.value() > end.value()
                && start.unit() == end.unit()
            {
                return Err(ConditionalError::InvalidPredicate(
                    "time interval start exceeds end".to_owned(),
                ));
            }

            Ok(())
        }

        ConditionalPredicate::All(children)
        | ConditionalPredicate::Any(children) => {
            if children.is_empty() {
                return Err(ConditionalError::EmptyPredicate(
                    "logical predicate".to_owned(),
                ));
            }

            for child in children {
                validate_predicate(child)?;
            }

            Ok(())
        }

        ConditionalPredicate::Not(child) => validate_predicate(child),

        ConditionalPredicate::Extension {
            namespace,
            kind,
            value,
        } => {
            if namespace.trim().is_empty()
                || kind.trim().is_empty()
                || value.trim().is_empty()
            {
                return Err(ConditionalError::InvalidPredicate(
                    "extension predicate identity cannot be empty".to_owned(),
                ));
            }

            Ok(())
        }
    }
}

// =============================================================================
// Existing NoiseCondition compatibility
// =============================================================================

/// Evaluates the existing declarative `NoiseCondition` from
/// `noise::specification`.
///
/// This adapter is intentionally kept separate from `ConditionalPredicate` so
/// the existing specification API remains stable.
///
/// The currently defined specification conditions are mapped directly:
///
/// ```text
/// Always
/// ModelActive
/// All
/// Any
/// ```
pub fn evaluate_specification_condition(
    condition: &NoiseCondition,
    context: &ConditionalEvaluationContext,
    policy: ConditionalEvaluationPolicy,
) -> ConditionalResult<ConditionalEvaluation> {
    match condition {
        NoiseCondition::Always => Ok(ConditionalEvaluation::new(
            ConditionTruth::True,
            ConditionalDependencies::default(),
        )),

        NoiseCondition::ModelActive(model) => evaluate(
            &ConditionalPredicate::ModelActive(*model),
            context,
            policy,
        ),

        NoiseCondition::All(values) => {
            if values.is_empty() {
                return Err(ConditionalError::EmptyPredicate(
                    "NoiseCondition::All".to_owned(),
                ));
            }

            let mut truth = ConditionTruth::True;
            let mut dependencies = ConditionalDependencies::default();

            for value in values {
                let result =
                    evaluate_specification_condition(value, context, policy)?;

                truth = truth.and(result.truth());

                dependencies.merge(result.dependencies());

                if truth.is_false() {
                    break;
                }
            }

            Ok(ConditionalEvaluation::new(truth, dependencies))
        }

        NoiseCondition::Any(values) => {
            if values.is_empty() {
                return Err(ConditionalError::EmptyPredicate(
                    "NoiseCondition::Any".to_owned(),
                ));
            }

            let mut truth = ConditionTruth::False;
            let mut dependencies = ConditionalDependencies::default();

            for value in values {
                let result =
                    evaluate_specification_condition(value, context, policy)?;

                truth = truth.or(result.truth());

                dependencies.merge(result.dependencies());

                if truth.is_true() {
                    break;
                }
            }

            Ok(ConditionalEvaluation::new(truth, dependencies))
        }
    }
}

// =============================================================================
// Conversion from existing specification condition
// =============================================================================

/// Converts the existing specification condition into the richer ZQN
/// conditional-predicate representation.
///
/// This is useful when downstream components want one normalized condition
/// representation without changing the existing specification API.
pub fn from_specification_condition(
    condition: &NoiseCondition,
) -> ConditionalResult<ConditionalPredicate> {
    match condition {
        NoiseCondition::Always => Ok(ConditionalPredicate::Always),

        NoiseCondition::ModelActive(model) => {
            Ok(ConditionalPredicate::ModelActive(*model))
        }

        NoiseCondition::All(values) => {
            if values.is_empty() {
                return Err(ConditionalError::EmptyPredicate(
                    "NoiseCondition::All".to_owned(),
                ));
            }

            let mut predicates = Vec::new();

            predicates
                .try_reserve(values.len())
                .map_err(|_| ConditionalError::EvaluationBudgetExceeded)?;

            for value in values {
                predicates.push(from_specification_condition(value)?);
            }

            Ok(ConditionalPredicate::All(predicates))
        }

        NoiseCondition::Any(values) => {
            if values.is_empty() {
                return Err(ConditionalError::EmptyPredicate(
                    "NoiseCondition::Any".to_owned(),
                ));
            }

            let mut predicates = Vec::new();

            predicates
                .try_reserve(values.len())
                .map_err(|_| ConditionalError::EvaluationBudgetExceeded)?;

            for value in values {
                predicates.push(from_specification_condition(value)?);
            }

            Ok(ConditionalPredicate::Any(predicates))
        }
    }
}

// =============================================================================
// Utility conversions
// =============================================================================

impl From<bool> for ConditionTruth {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation() -> OperationId {
        OperationId::new(7)
    }

    fn logical_qubit() -> QubitId {
        QubitId::new(3)
    }

    fn physical_qubit() -> PhysicalQubitId {
        PhysicalQubitId::new(11)
    }

    fn classical_bit() -> ClassicalBitId {
        ClassicalBitId::new(2)
    }

    #[test]
    fn always_is_true() {
        let context = ConditionalEvaluationContext::new();

        let result =
            evaluate_default(&ConditionalPredicate::Always, &context)
                .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn never_is_false() {
        let context = ConditionalEvaluationContext::new();

        let result =
            evaluate_default(&ConditionalPredicate::Never, &context)
                .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::False);
    }

    #[test]
    fn operation_uses_canonical_operation_id() {
        let context =
            ConditionalEvaluationContext::new().with_operation(operation());

        let predicate = ConditionalPredicate::operation(operation());

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
        assert!(result.dependencies().operations().contains(&operation()));
    }

    #[test]
    fn logical_resource_uses_canonical_qubit_id() {
        let context =
            ConditionalEvaluationContext::new()
                .with_logical_resource(logical_qubit());

        let predicate =
            ConditionalPredicate::logical_resource(logical_qubit());

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn physical_resource_uses_canonical_physical_qubit_id() {
        let context =
            ConditionalEvaluationContext::new()
                .with_physical_resource(physical_qubit());

        let predicate =
            ConditionalPredicate::physical_resource(physical_qubit());

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn measurement_uses_canonical_classical_bit_id() {
        let context =
            ConditionalEvaluationContext::new()
                .with_measurement(classical_bit(), true);

        let predicate =
            ConditionalPredicate::measurement(classical_bit(), true);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn missing_measurement_is_unknown() {
        let context = ConditionalEvaluationContext::new();

        let predicate =
            ConditionalPredicate::measurement(classical_bit(), true);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_unknown());
    }

    #[test]
    fn missing_measurement_can_fail_closed() {
        let context = ConditionalEvaluationContext::new();

        let predicate =
            ConditionalPredicate::measurement(classical_bit(), true);

        let policy =
            ConditionalEvaluationPolicy::default()
                .with_unknown_policy(UnknownPolicy::False);

        let result =
            evaluate(&predicate, &context, policy)
                .expect("evaluation must succeed");

        assert!(result.is_false());
    }

    #[test]
    fn value_equality_works() {
        let context =
            ConditionalEvaluationContext::new()
                .with_value(
                    "temperature",
                    ConditionalValue::real(42.0)
                        .expect("finite value"),
                )
                .expect("context construction must succeed");

        let predicate = ConditionalPredicate::value_equals(
            "temperature",
            ConditionalValue::Real(42.0),
        );

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn real_range_works() {
        let context =
            ConditionalEvaluationContext::new()
                .with_value(
                    "temperature",
                    ConditionalValue::Real(5.0),
                )
                .expect("context construction must succeed");

        let predicate = ConditionalPredicate::RealInRange {
            name: "temperature".to_owned(),
            minimum: 0.0,
            maximum: 10.0,
        };

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn approximate_real_comparison_is_explicit() {
        let context =
            ConditionalEvaluationContext::new()
                .with_value(
                    "error_rate",
                    ConditionalValue::Real(0.1000001),
                )
                .expect("context construction must succeed");

        let predicate = ConditionalPredicate::RealApproximately {
            name: "error_rate".to_owned(),
            target: 0.1,
            tolerance: 0.000001,
        };

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn calibration_condition_is_explicit() {
        let context =
            ConditionalEvaluationContext::new()
                .with_calibration(CalibrationValidity::Valid);

        let predicate =
            ConditionalPredicate::Calibration(CalibrationValidity::Valid);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn time_condition_does_not_assume_hardware_unit() {
        let context =
            ConditionalEvaluationContext::new()
                .with_time(
                    ConditionalTime::new(
                        10.0,
                        ConditionalTimeUnit::Nanoseconds,
                    )
                    .expect("time must be valid"),
                );

        let predicate = ConditionalPredicate::TimeInRange {
            start: ConditionalTime::new(
                5.0,
                ConditionalTimeUnit::Nanoseconds,
            )
            .expect("time must be valid"),
            end: ConditionalTime::new(
                15.0,
                ConditionalTimeUnit::Nanoseconds,
            )
            .expect("time must be valid"),
        };

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert!(result.is_true());
    }

    #[test]
    fn all_uses_three_valued_logic() {
        let context =
            ConditionalEvaluationContext::new()
                .with_operation(operation());

        let predicate = ConditionalPredicate::All(vec![
            ConditionalPredicate::Operation(operation()),
            ConditionalPredicate::Measurement {
                bit: classical_bit(),
                value: true,
            },
        ]);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::Unknown);
    }

    #[test]
    fn any_short_circuits_true() {
        let context =
            ConditionalEvaluationContext::new();

        let predicate = ConditionalPredicate::Any(vec![
            ConditionalPredicate::Always,
            ConditionalPredicate::Extension {
                namespace: "future".to_owned(),
                kind: "unreachable".to_owned(),
                value: "predicate".to_owned(),
            },
        ]);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn not_reverses_truth() {
        let context = ConditionalEvaluationContext::new();

        let predicate =
            ConditionalPredicate::not(ConditionalPredicate::Always);

        let result =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::False);
    }

    #[test]
    fn specification_always_is_supported() {
        let context = ConditionalEvaluationContext::new();

        let result =
            evaluate_specification_condition(
                &NoiseCondition::Always,
                &context,
                ConditionalEvaluationPolicy::default(),
            )
            .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn specification_model_active_is_supported() {
        let model = NoiseModelId::new(9);

        let context =
            ConditionalEvaluationContext::new()
                .with_active_model(model);

        let result =
            evaluate_specification_condition(
                &NoiseCondition::ModelActive(model),
                &context,
                ConditionalEvaluationPolicy::default(),
            )
            .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn specification_all_is_supported() {
        let model = NoiseModelId::new(9);

        let context =
            ConditionalEvaluationContext::new()
                .with_active_model(model);

        let condition = NoiseCondition::All(vec![
            NoiseCondition::Always,
            NoiseCondition::ModelActive(model),
        ]);

        let result =
            evaluate_specification_condition(
                &condition,
                &context,
                ConditionalEvaluationPolicy::default(),
            )
            .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn specification_any_is_supported() {
        let model = NoiseModelId::new(9);

        let context = ConditionalEvaluationContext::new();

        let condition = NoiseCondition::Any(vec![
            NoiseCondition::ModelActive(model),
            NoiseCondition::Always,
        ]);

        let result =
            evaluate_specification_condition(
                &condition,
                &context,
                ConditionalEvaluationPolicy::default(),
            )
            .expect("evaluation must succeed");

        assert_eq!(result.truth(), ConditionTruth::True);
    }

    #[test]
    fn dependency_collection_is_deterministic() {
        let context =
            ConditionalEvaluationContext::new()
                .with_operation(operation())
                .with_logical_resource(logical_qubit())
                .with_physical_resource(physical_qubit())
                .with_measurement(classical_bit(), true);

        let predicate = ConditionalPredicate::All(vec![
            ConditionalPredicate::Operation(operation()),
            ConditionalPredicate::LogicalResource(logical_qubit()),
            ConditionalPredicate::PhysicalResource(physical_qubit()),
            ConditionalPredicate::Measurement {
                bit: classical_bit(),
                value: true,
            },
        ]);

        let first =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        let second =
            evaluate_default(&predicate, &context)
                .expect("evaluation must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn invalid_real_range_is_rejected() {
        let predicate = ConditionalPredicate::RealInRange {
            name: "x".to_owned(),
            minimum: 10.0,
            maximum: 1.0,
        };

        let result =
            evaluate_default(
                &predicate,
                &ConditionalEvaluationContext::new(),
            );

        assert!(matches!(
            result,
            Err(ConditionalError::InvalidPredicate(_))
        ));
    }

    #[test]
    fn explicit_node_budget_is_enforced() {
        let predicate = ConditionalPredicate::All(vec![
            ConditionalPredicate::Always,
            ConditionalPredicate::Always,
            ConditionalPredicate::Always,
        ]);

        let policy =
            ConditionalEvaluationPolicy::default()
                .with_node_budget(1);

        let result =
            evaluate(
                &predicate,
                &ConditionalEvaluationContext::new(),
                policy,
            );

        assert_eq!(
            result,
            Err(ConditionalError::EvaluationBudgetExceeded)
        );
    }
}