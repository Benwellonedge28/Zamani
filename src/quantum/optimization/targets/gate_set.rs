//! Zamani Quantum Optimization — Target Gate Sets
//!
//! Production-grade, backend-independent representation of the operations
//! accepted by an optimization target.
//!
//! # Architectural role
//!
//! `gate_set.rs` answers one precise question:
//!
//! > Which logical quantum operations are accepted by this optimization
//! > target, and what declarative properties/cost information does the
//! > optimizer know about those operations?
//!
//! It deliberately does NOT own:
//!
//! - the canonical Quantum IR;
//! - circuit representation;
//! - physical topology;
//! - logical-to-physical routing;
//! - scheduling;
//! - pulse calibration;
//! - QPU communication;
//! - execution;
//! - decomposition algorithms;
//! - optimization passes;
//! - compiler-wide mutable state.
//!
//! The canonical quantum representation remains:
//!
//! `crate::quantum::ir`
//!
//! In particular, this module consumes `GateKind` from the canonical IR
//! rather than defining a second gate enum.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::targets
//!                              │
//!                ┌─────────────┴─────────────┐
//!                │                           │
//!                ▼                           ▼
//!          gate_set.rs                 constraints.rs
//!                │                           │
//!                └─────────────┬─────────────┘
//!                              ▼
//!                           target.rs
//!                              │
//!                              ▼
//!                           planner
//!                              │
//!                              ▼
//!                       optimization passes
//!                              │
//!                              ▼
//!                           synthesis
//! ```
//!
//! # Design goals
//!
//! This implementation provides:
//!
//! - canonical-IR `GateKind` support;
//! - stable textual identifiers for future/custom operations;
//! - deterministic ordering;
//! - weighted operations;
//! - multi-objective operation costs;
//! - arity constraints;
//! - parameter-count constraints;
//! - unitary/non-unitary classification;
//! - Clifford/non-Clifford classification;
//! - measurement/reset/barrier classification;
//! - symbolic-parameter support metadata;
//! - exact versus approximate operation support;
//! - native/decomposition distinction;
//! - operation aliases;
//! - target-set validation;
//! - immutable snapshots through ordinary Rust ownership;
//! - bounded metadata and collection growth;
//! - no global mutable state;
//! - no unsafe Rust;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - serialization compatibility;
//! - deterministic hashing/fingerprinting;
//! - efficient membership queries;
//! - scalable custom-operation support.
//!
//! # Important semantic distinction
//!
//! A gate set is NOT the same thing as a decomposition rule database.
//!
//! This file says:
//!
//! ```text
//! "RZ is accepted by target X."
//! ```
//!
//! A future decomposition subsystem says:
//!
//! ```text
//! "Operation U can be transformed into RZ/RX/RZ."
//! ```
//!
//! Keeping those responsibilities separate allows the target gate set to be
//! reused by:
//!
//! - synthesis;
//! - decomposition;
//! - target-aware optimization;
//! - validation;
//! - cost estimation;
//! - planner decisions;
//! - serialization;
//! - diagnostics.
//!
//! # Canonical IR integration
//!
//! The primary built-in operation identity is:
//!
//! `crate::quantum::ir::GateKind`
//!
//! Custom/future operations use `CustomGateId`.
//!
//! This deliberately allows Zamani to evolve beyond the current `GateKind`
//! enum without requiring this file to be rewritten every time a new quantum
//! operation is introduced.
//!
//! # Target integration contract
//!
//! `targets/target.rs` should own the complete `OptimizationTarget` and use
//! `TargetGateSet` as one of its components.
//!
//! `targets/constraints.rs` should own target-wide constraints.
//!
//! `targets/profiles.rs` should construct predefined gate sets using the
//! builders in this file.
//!
//! `optimization/cost.rs` should consume `GateCost` and `TargetGateSet`
//! queries rather than duplicating gate-cost tables.
//!
//! `optimization/planner.rs` should query:
//!
//! - `contains()`;
//! - `supports()`;
//! - `operation()`;
//! - `native_operations()`;
//! - `required_operations()`;
//! - `cost()`;
//! - `fingerprint()`.
//!
//! `optimization/synthesis/*` should use the gate set as the stopping/basis
//! condition for synthesis, but should not mutate it.
//!
//! `optimization/verification/*` may use the gate set when validating that a
//! synthesized circuit contains only target-supported operations.
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
//! No unsafe code is permitted.
//!
//! # External design alignment
//!
//! Modern quantum compilation systems distinguish a target gate basis from
//! decomposition rules and often associate costs with target operations.
//! PennyLane's current decomposition API supports weighted target gate sets
//! and uses them when selecting cost-efficient decompositions. Zamani keeps
//! the same useful separation while making the target contract strongly typed
//! and independent of a particular backend.
//!
//! # Scalability
//!
//! There is intentionally no artificial circuit-size limit in this file.
//!
//! A gate set describes a finite or explicitly extensible *operation vocabulary*,
//! not a circuit. Circuit-size limits belong to optimization limits and target
//! constraints.
//!
//! Collection limits in this file protect against malformed configuration
//! objects and accidental memory exhaustion. They are configuration limits,
//! not limits on the size of a circuit that Zamani can optimize.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::quantum::ir::GateKind;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for serialized target gate sets.
pub const GATE_SET_SCHEMA_ID: &str = "zamani.quantum.optimization.target_gate_set";

/// Semantic version of the gate-set serialization contract.
pub const GATE_SET_SCHEMA_VERSION: u32 = 1;

/// Maximum length of a canonical operation identifier.
pub const MAX_OPERATION_ID_LENGTH: usize = 512;

/// Maximum length of an operation display name.
pub const MAX_OPERATION_NAME_LENGTH: usize = 512;

/// Maximum number of aliases attached to one operation.
pub const MAX_ALIASES_PER_OPERATION: usize = 64;

/// Maximum length of an alias.
pub const MAX_ALIAS_LENGTH: usize = 512;

/// Maximum number of operations in a gate set.
///
/// This is a configuration-safety limit, not a circuit-size limit.
pub const DEFAULT_MAX_OPERATIONS: usize = 1_000_000;

/// Maximum number of total aliases in one gate set.
pub const DEFAULT_MAX_ALIASES: usize = 4_000_000;

/// Maximum metadata properties per operation.
pub const DEFAULT_MAX_METADATA: usize = 256;

// =============================================================================
// Result
// =============================================================================

/// Result type for gate-set construction and validation.
pub type GateSetResult<T> = Result<T, GateSetError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by target gate-set construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateSetError {
    /// An operation identifier is empty.
    EmptyOperationId,

    /// An identifier exceeds the configured maximum.
    OperationIdTooLong {
        /// Maximum permitted length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// An identifier contains unsupported characters.
    InvalidOperationId {
        /// Supplied identifier.
        value: String,
    },

    /// An operation name is too long.
    OperationNameTooLong {
        /// Maximum permitted length.
        maximum: usize,

        /// Actual length.
        actual: usize,
    },

    /// Two operations resolve to the same canonical identity.
    DuplicateOperation {
        /// Conflicting identifier.
        operation: String,
    },

    /// An alias is invalid.
    InvalidAlias {
        /// Supplied alias.
        alias: String,
    },

    /// An alias is already used by another operation.
    DuplicateAlias {
        /// Conflicting alias.
        alias: String,
    },

    /// Too many operations were supplied.
    TooManyOperations {
        /// Maximum permitted number.
        maximum: usize,

        /// Actual number.
        actual: usize,
    },

    /// Too many aliases were supplied.
    TooManyAliases {
        /// Maximum permitted number.
        maximum: usize,

        /// Actual number.
        actual: usize,
    },

    /// An operation has an invalid arity range.
    InvalidArity {
        /// Operation identifier.
        operation: String,

        /// Minimum arity.
        minimum: usize,

        /// Maximum arity.
        maximum: Option<usize>,
    },

    /// An operation has an invalid parameter count.
    InvalidParameterCount {
        /// Operation identifier.
        operation: String,

        /// Parameter count.
        count: usize,
    },

    /// An operation has an invalid cost.
    InvalidCost {
        /// Operation identifier.
        operation: String,

        /// Cost field.
        field: &'static str,
    },

    /// An operation has incompatible properties.
    InvalidOperationProperties {
        /// Operation identifier.
        operation: String,

        /// Explanation.
        reason: &'static str,
    },

    /// Required operation is not present.
    MissingRequiredOperation {
        /// Required operation.
        operation: String,
    },

    /// A native operation is not present in the gate set.
    NativeOperationMissing {
        /// Native operation.
        operation: String,
    },

    /// An operation is not supported.
    UnsupportedOperation {
        /// Operation identifier.
        operation: String,
    },

    /// A requested built-in operation is unavailable.
    UnsupportedGateKind {
        /// Canonical IR gate kind.
        gate: GateKind,
    },

    /// A metadata property is invalid.
    InvalidMetadata {
        /// Metadata key.
        key: String,
    },

    /// The gate set is semantically inconsistent.
    InvalidGateSet {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for GateSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOperationId => {
                write!(formatter, "gate-set operation identifier must not be empty")
            }

            Self::OperationIdTooLong { maximum, actual } => {
                write!(
                    formatter,
                    "gate-set operation identifier exceeds maximum length \
                     {maximum}: actual {actual}"
                )
            }

            Self::InvalidOperationId { value } => {
                write!(
                    formatter,
                    "invalid gate-set operation identifier `{value}`"
                )
            }

            Self::OperationNameTooLong { maximum, actual } => {
                write!(
                    formatter,
                    "gate-set operation name exceeds maximum length \
                     {maximum}: actual {actual}"
                )
            }

            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "gate-set operation `{operation}` is already defined"
                )
            }

            Self::InvalidAlias { alias } => {
                write!(formatter, "invalid gate-set alias `{alias}`")
            }

            Self::DuplicateAlias { alias } => {
                write!(
                    formatter,
                    "gate-set alias `{alias}` is already assigned"
                )
            }

            Self::TooManyOperations { maximum, actual } => {
                write!(
                    formatter,
                    "gate set contains too many operations: \
                     maximum {maximum}, actual {actual}"
                )
            }

            Self::TooManyAliases { maximum, actual } => {
                write!(
                    formatter,
                    "gate set contains too many aliases: \
                     maximum {maximum}, actual {actual}"
                )
            }

            Self::InvalidArity {
                operation,
                minimum,
                maximum,
            } => match maximum {
                Some(maximum) => write!(
                    formatter,
                    "operation `{operation}` has invalid arity range \
                     {minimum}..={maximum}"
                ),
                None => write!(
                    formatter,
                    "operation `{operation}` has invalid minimum arity \
                     {minimum}"
                ),
            },

            Self::InvalidParameterCount { operation, count } => {
                write!(
                    formatter,
                    "operation `{operation}` has invalid parameter count \
                     {count}"
                )
            }

            Self::InvalidCost { operation, field } => {
                write!(
                    formatter,
                    "operation `{operation}` has invalid cost field `{field}`"
                )
            }

            Self::InvalidOperationProperties { operation, reason } => {
                write!(
                    formatter,
                    "operation `{operation}` has invalid properties: {reason}"
                )
            }

            Self::MissingRequiredOperation { operation } => {
                write!(
                    formatter,
                    "required gate-set operation `{operation}` is missing"
                )
            }

            Self::NativeOperationMissing { operation } => {
                write!(
                    formatter,
                    "native gate-set operation `{operation}` is missing"
                )
            }

            Self::UnsupportedOperation { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` is not supported by the target gate set: \
                     `{operation}`"
                )
            }

            Self::UnsupportedGateKind { gate } => {
                write!(
                    formatter,
                    "canonical gate kind `{gate:?}` is not supported by the target gate set"
                )
            }

            Self::InvalidMetadata { key } => {
                write!(
                    formatter,
                    "invalid gate-set metadata key `{key}`"
                )
            }

            Self::InvalidGateSet { message } => {
                write!(formatter, "invalid target gate set: {message}")
            }
        }
    }
}

impl std::error::Error for GateSetError {}

// =============================================================================
// Stable operation identifier
// =============================================================================

/// Stable operation identifier.
///
/// Built-in canonical IR gates use identifiers in the `builtin.*` namespace.
///
/// Custom operations may use any stable, namespaced identifier such as:
///
/// ```text
/// custom.my_native_gate
/// photonic.beamsplitter
/// neutral_atom.global_rydberg
/// logical.magic_state_injection
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct OperationId(String);

impl OperationId {
    /// Creates a validated operation identifier.
    pub fn new(value: impl Into<String>) -> GateSetResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(GateSetError::EmptyOperationId);
        }

        if value.len() > MAX_OPERATION_ID_LENGTH {
            return Err(GateSetError::OperationIdTooLong {
                maximum: MAX_OPERATION_ID_LENGTH,
                actual: value.len(),
            });
        }

        if !is_valid_identifier(&value) {
            return Err(GateSetError::InvalidOperationId { value });
        }

        Ok(Self(value))
    }

    /// Creates the canonical identifier for an IR gate kind.
    pub fn from_gate_kind(kind: GateKind) -> Self {
        Self(format!("builtin.{}", gate_kind_name(kind)))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns whether this operation is a canonical built-in IR operation.
    #[must_use]
    pub fn is_builtin(&self) -> bool {
        self.0.starts_with("builtin.")
    }

    /// Returns the namespace before the first dot.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.0.split_once('.').map(|(namespace, _)| namespace)
    }

    /// Returns the local operation name.
    #[must_use]
    pub fn local_name(&self) -> &str {
        self.0
            .rsplit_once('.')
            .map(|(_, name)| name)
            .unwrap_or(self.as_str())
    }
}

impl AsRef<str> for OperationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Operation reference
// =============================================================================

/// Operation identity accepted by a target gate set.
///
/// This allows the optimizer to work with the canonical IR while still
/// supporting future/custom operations.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum OperationRef {
    /// Canonical operation from `quantum::ir::GateKind`.
    Builtin(GateKind),

    /// Stable custom/future operation identifier.
    Custom(OperationId),
}

impl OperationRef {
    /// Returns the stable operation identifier.
    #[must_use]
    pub fn id(&self) -> OperationId {
        match self {
            Self::Builtin(kind) => OperationId::from_gate_kind(*kind),
            Self::Custom(id) => id.clone(),
        }
    }

    /// Returns the built-in gate kind when applicable.
    #[must_use]
    pub fn gate_kind(&self) -> Option<GateKind> {
        match self {
            Self::Builtin(kind) => Some(*kind),
            Self::Custom(_) => None,
        }
    }

    /// Returns whether this operation is a canonical IR operation.
    #[must_use]
    pub fn is_builtin(&self) -> bool {
        matches!(self, Self::Builtin(_))
    }
}

impl fmt::Display for OperationRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.id().as_str())
    }
}

// =============================================================================
// Arity
// =============================================================================

/// Supported operand-count range for a target operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Arity {
    minimum: usize,
    maximum: Option<usize>,
}

impl Arity {
    /// Creates an exact arity.
    #[must_use]
    pub const fn exact(value: usize) -> Self {
        Self {
            minimum: value,
            maximum: Some(value),
        }
    }

    /// Creates an unbounded arity range beginning at `minimum`.
    #[must_use]
    pub const fn at_least(minimum: usize) -> Self {
        Self {
            minimum,
            maximum: None,
        }
    }

    /// Creates a bounded arity range.
    pub fn range(minimum: usize, maximum: usize) -> GateSetResult<Self> {
        if minimum > maximum {
            return Err(GateSetError::InvalidArity {
                operation: "<unknown>".to_owned(),
                minimum,
                maximum: Some(maximum),
            });
        }

        Ok(Self {
            minimum,
            maximum: Some(maximum),
        })
    }

    /// Minimum arity.
    #[must_use]
    pub const fn minimum(self) -> usize {
        self.minimum
    }

    /// Maximum arity, if bounded.
    #[must_use]
    pub const fn maximum(self) -> Option<usize> {
        self.maximum
    }

    /// Returns whether the supplied arity is accepted.
    #[must_use]
    pub const fn accepts(self, arity: usize) -> bool {
        if arity < self.minimum {
            return false;
        }

        match self.maximum {
            Some(maximum) => arity <= maximum,
            None => true,
        }
    }
}

impl Default for Arity {
    fn default() -> Self {
        Self::exact(1)
    }
}

// =============================================================================
// Operation parameter policy
// =============================================================================

/// Parameter-count policy for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ParameterPolicy {
    /// Operation takes no parameters.
    Exact(usize),

    /// Operation takes at least `minimum` parameters.
    AtLeast(usize),

    /// Operation accepts any parameter count.
    Any,
}

impl ParameterPolicy {
    /// Returns whether a parameter count is accepted.
    #[must_use]
    pub const fn accepts(self, count: usize) -> bool {
        match self {
            Self::Exact(expected) => count == expected,
            Self::AtLeast(minimum) => count >= minimum,
            Self::Any => true,
        }
    }

    /// Returns the exact parameter count when one exists.
    #[must_use]
    pub const fn exact(self) -> Option<usize> {
        match self {
            Self::Exact(value) => Some(value),
            Self::AtLeast(_) | Self::Any => None,
        }
    }
}

// =============================================================================
// Operation semantics
// =============================================================================

/// Semantic category of a target operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OperationSemantics {
    /// Exact unitary operation.
    Unitary,

    /// Measurement operation.
    Measurement,

    /// State-reset operation.
    Reset,

    /// Circuit barrier/non-semantic compiler boundary.
    Barrier,

    /// General non-unitary operation.
    NonUnitary,
}

impl OperationSemantics {
    /// Returns whether this operation is unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        matches!(self, Self::Unitary)
    }

    /// Returns whether this operation is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(self) -> bool {
        !self.is_unitary()
    }
}

// =============================================================================
// Operation capabilities
// =============================================================================

/// Declarative capabilities/properties of a target operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OperationCapabilities {
    /// Whether the operation is exactly supported by the target.
    pub exact: bool,

    /// Whether the operation is natively executable by the target.
    pub native: bool,

    /// Whether it may be synthesized/decomposed into other operations.
    pub decomposable: bool,

    /// Whether symbolic parameters are accepted.
    pub symbolic_parameters: bool,

    /// Whether this operation is Clifford for all legal parameters.
    pub clifford: bool,

    /// Whether the operation is self-inverse.
    pub self_inverse: bool,

    /// Whether the operation can participate in parallel execution.
    pub parallelizable: bool,

    /// Whether the operation may be used inside unitary-only regions.
    pub unitary_region: bool,

    /// Whether this operation requires classical information.
    pub requires_classical_control: bool,
}

impl Default for OperationCapabilities {
    fn default() -> Self {
        Self {
            exact: true,
            native: true,
            decomposable: true,
            symbolic_parameters: true,
            clifford: false,
            self_inverse: false,
            parallelizable: true,
            unitary_region: true,
            requires_classical_control: false,
        }
    }
}

// =============================================================================
// Cost
// =============================================================================

/// Multi-dimensional cost information associated with one target operation.
///
/// All fields are additive hints for optimization and planning. They are not
/// physical measurements unless the target provider explicitly defines them
/// that way.
///
/// A value of `None` means the target does not provide that metric.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GateCost {
    /// Generic scalar cost used when a pass requires one number.
    pub generic: Option<f64>,

    /// Gate-count contribution.
    pub gate_count: Option<f64>,

    /// Single-qubit gate contribution.
    pub single_qubit: Option<f64>,

    /// Two-qubit gate contribution.
    pub two_qubit: Option<f64>,

    /// Multi-qubit gate contribution.
    pub multi_qubit: Option<f64>,

    /// Logical depth contribution.
    pub depth: Option<f64>,

    /// Execution-time hint.
    pub duration: Option<f64>,

    /// Estimated error contribution.
    pub error: Option<f64>,

    /// Fault-tolerant T-resource contribution.
    pub t_count: Option<f64>,

    /// Fault-tolerant T-depth contribution.
    pub t_depth: Option<f64>,

    /// Magic-state/resource contribution.
    pub magic_state: Option<f64>,

    /// Energy/resource hint.
    pub energy: Option<f64>,
}

impl GateCost {
    /// Creates a unit cost across the generic gate-count metric.
    #[must_use]
    pub const fn unit() -> Self {
        Self {
            generic: Some(1.0),
            gate_count: Some(1.0),
            single_qubit: None,
            two_qubit: None,
            multi_qubit: None,
            depth: Some(1.0),
            duration: None,
            error: None,
            t_count: None,
            t_depth: None,
            magic_state: None,
            energy: None,
        }
    }

    /// Creates a zero cost.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            generic: Some(0.0),
            gate_count: Some(0.0),
            single_qubit: Some(0.0),
            two_qubit: Some(0.0),
            multi_qubit: Some(0.0),
            depth: Some(0.0),
            duration: Some(0.0),
            error: Some(0.0),
            t_count: Some(0.0),
            t_depth: Some(0.0),
            magic_state: Some(0.0),
            energy: Some(0.0),
        }
    }

    /// Sets generic cost.
    #[must_use]
    pub const fn with_generic(mut self, value: f64) -> Self {
        self.generic = Some(value);
        self
    }

    /// Sets gate-count cost.
    #[must_use]
    pub const fn with_gate_count(mut self, value: f64) -> Self {
        self.gate_count = Some(value);
        self
    }

    /// Sets two-qubit cost.
    #[must_use]
    pub const fn with_two_qubit(mut self, value: f64) -> Self {
        self.two_qubit = Some(value);
        self
    }

    /// Sets duration cost.
    #[must_use]
    pub const fn with_duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets error cost.
    #[must_use]
    pub const fn with_error(mut self, value: f64) -> Self {
        self.error = Some(value);
        self
    }

    /// Sets T-count cost.
    #[must_use]
    pub const fn with_t_count(mut self, value: f64) -> Self {
        self.t_count = Some(value);
        self
    }

    /// Sets T-depth cost.
    #[must_use]
    pub const fn with_t_depth(mut self, value: f64) -> Self {
        self.t_depth = Some(value);
        self
    }

    /// Validates all present numeric costs.
    pub fn validate(&self, operation: &str) -> GateSetResult<()> {
        let fields = [
            ("generic", self.generic),
            ("gate_count", self.gate_count),
            ("single_qubit", self.single_qubit),
            ("two_qubit", self.two_qubit),
            ("multi_qubit", self.multi_qubit),
            ("depth", self.depth),
            ("duration", self.duration),
            ("error", self.error),
            ("t_count", self.t_count),
            ("t_depth", self.t_depth),
            ("magic_state", self.magic_state),
            ("energy", self.energy),
        ];

        for (field, value) in fields {
            if let Some(value) = value {
                if !value.is_finite() || value < 0.0 {
                    return Err(GateSetError::InvalidCost {
                        operation: operation.to_owned(),
                        field,
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns the preferred scalar cost.
    #[must_use]
    pub fn scalar_or_one(&self) -> f64 {
        self.generic
            .or(self.gate_count)
            .unwrap_or(1.0)
    }

    /// Returns a cost component by metric name.
    #[must_use]
    pub fn metric(&self, metric: CostMetric) -> Option<f64> {
        match metric {
            CostMetric::Generic => self.generic,
            CostMetric::GateCount => self.gate_count,
            CostMetric::SingleQubit => self.single_qubit,
            CostMetric::TwoQubit => self.two_qubit,
            CostMetric::MultiQubit => self.multi_qubit,
            CostMetric::Depth => self.depth,
            CostMetric::Duration => self.duration,
            CostMetric::Error => self.error,
            CostMetric::TCount => self.t_count,
            CostMetric::TDepth => self.t_depth,
            CostMetric::MagicState => self.magic_state,
            CostMetric::Energy => self.energy,
        }
    }
}

impl Default for GateCost {
    fn default() -> Self {
        Self::unit()
    }
}

/// Cost dimension exposed by a target gate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CostMetric {
    Generic,
    GateCount,
    SingleQubit,
    TwoQubit,
    MultiQubit,
    Depth,
    Duration,
    Error,
    TCount,
    TDepth,
    MagicState,
    Energy,
}

// =============================================================================
// Target operation
// =============================================================================

/// Complete declarative description of one accepted target operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetOperation {
    /// Stable identity.
    id: OperationId,

    /// Canonical operation reference.
    operation: OperationRef,

    /// Human-readable name.
    name: String,

    /// Accepted operand count.
    arity: Arity,

    /// Accepted parameter count.
    parameters: ParameterPolicy,

    /// Semantic category.
    semantics: OperationSemantics,

    /// Operation capabilities.
    capabilities: OperationCapabilities,

    /// Optimization cost information.
    cost: GateCost,

    /// Stable aliases.
    aliases: BTreeSet<String>,

    /// Extension metadata.
    metadata: BTreeMap<String, String>,
}

impl TargetOperation {
    /// Creates a target operation from a canonical IR gate.
    pub fn builtin(kind: GateKind) -> GateSetResult<Self> {
        let operation = OperationRef::Builtin(kind);

        let arity = match kind.operand_count() {
            crate::quantum::ir::gate::OperandCount::Exact(value) => {
                Arity::exact(value)
            }
            crate::quantum::ir::gate::OperandCount::AtLeast(value) => {
                Arity::at_least(value)
            }
        };

        let parameters = ParameterPolicy::Exact(kind.parameter_count());

        let semantics = if kind.is_measurement() {
            OperationSemantics::Measurement
        } else if kind.is_reset() {
            OperationSemantics::Reset
        } else if kind.is_barrier() {
            OperationSemantics::Barrier
        } else if kind.is_unitary() {
            OperationSemantics::Unitary
        } else {
            OperationSemantics::NonUnitary
        };

        let mut capabilities = OperationCapabilities::default();

        capabilities.clifford = kind.is_clifford();
        capabilities.self_inverse = kind.is_self_inverse();
        capabilities.symbolic_parameters = kind.is_parameterized();
        capabilities.unitary_region = kind.is_unitary();
        capabilities.requires_classical_control = false;

        if kind.is_measurement() || kind.is_reset() {
            capabilities.parallelizable = false;
            capabilities.decomposable = false;
        }

        if kind.is_barrier() {
            capabilities.parallelizable = false;
            capabilities.decomposable = false;
            capabilities.unitary_region = false;
        }

        let mut cost = GateCost::unit();

        match arity.maximum() {
            Some(1) => {
                cost.single_qubit = Some(1.0);
                cost.two_qubit = Some(0.0);
                cost.multi_qubit = Some(0.0);
            }
            Some(2) => {
                cost.single_qubit = Some(0.0);
                cost.two_qubit = Some(1.0);
                cost.multi_qubit = Some(0.0);
            }
            Some(_) | None => {
                cost.single_qubit = Some(0.0);
                cost.two_qubit = Some(0.0);
                cost.multi_qubit = Some(1.0);
            }
        }

        if matches!(kind, GateKind::T | GateKind::Tdg) {
            cost.t_count = Some(1.0);
            cost.t_depth = Some(1.0);
            cost.magic_state = Some(1.0);
        }

        let mut result = Self {
            id: operation.id(),
            operation,
            name: gate_kind_name(kind).to_owned(),
            arity,
            parameters,
            semantics,
            capabilities,
            cost,
            aliases: BTreeSet::new(),
            metadata: BTreeMap::new(),
        };

        result
            .aliases
            .insert(gate_kind_name(kind).to_owned());

        result.validate()?;

        Ok(result)
    }

    /// Creates a custom target operation.
    pub fn custom(
        id: OperationId,
        name: impl Into<String>,
        arity: Arity,
        parameters: ParameterPolicy,
        semantics: OperationSemantics,
    ) -> GateSetResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(GateSetError::InvalidOperationProperties {
                operation: id.to_string(),
                reason: "operation name must not be empty",
            });
        }

        if name.len() > MAX_OPERATION_NAME_LENGTH {
            return Err(GateSetError::OperationNameTooLong {
                maximum: MAX_OPERATION_NAME_LENGTH,
                actual: name.len(),
            });
        }

        let operation = OperationRef::Custom(id.clone());

        let capabilities = OperationCapabilities {
            unitary_region: semantics.is_unitary(),
            ..OperationCapabilities::default()
        };

        let result = Self {
            id,
            operation,
            name,
            arity,
            parameters,
            semantics,
            capabilities,
            cost: GateCost::unit(),
            aliases: BTreeSet::new(),
            metadata: BTreeMap::new(),
        };

        result.validate()?;

        Ok(result)
    }

    /// Returns the stable operation ID.
    #[must_use]
    pub fn id(&self) -> &OperationId {
        &self.id
    }

    /// Returns the canonical operation reference.
    #[must_use]
    pub fn operation(&self) -> &OperationRef {
        &self.operation
    }

    /// Returns the built-in IR gate kind, if applicable.
    #[must_use]
    pub fn gate_kind(&self) -> Option<GateKind> {
        self.operation.gate_kind()
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the accepted arity.
    #[must_use]
    pub const fn arity(&self) -> Arity {
        self.arity
    }

    /// Returns the parameter policy.
    #[must_use]
    pub const fn parameter_policy(&self) -> ParameterPolicy {
        self.parameters
    }

    /// Returns semantic classification.
    #[must_use]
    pub const fn semantics(&self) -> OperationSemantics {
        self.semantics
    }

    /// Returns capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> OperationCapabilities {
        self.capabilities
    }

    /// Returns cost information.
    #[must_use]
    pub const fn cost(&self) -> GateCost {
        self.cost
    }

    /// Returns aliases in deterministic order.
    #[must_use]
    pub fn aliases(&self) -> &BTreeSet<String> {
        &self.aliases
    }

    /// Returns metadata in deterministic order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Adds an alias.
    pub fn with_alias(mut self, alias: impl Into<String>) -> GateSetResult<Self> {
        self.add_alias(alias)?;
        Ok(self)
    }

    /// Adds an alias in-place.
    pub fn add_alias(&mut self, alias: impl Into<String>) -> GateSetResult<()> {
        let alias = alias.into();

        if alias.is_empty()
            || alias.len() > MAX_ALIAS_LENGTH
            || !is_valid_identifier(&alias)
        {
            return Err(GateSetError::InvalidAlias { alias });
        }

        if self.aliases.len() >= MAX_ALIASES_PER_OPERATION
            && !self.aliases.contains(&alias)
        {
            return Err(GateSetError::TooManyAliases {
                maximum: MAX_ALIASES_PER_OPERATION,
                actual: self.aliases.len() + 1,
            });
        }

        self.aliases.insert(alias);
        Ok(())
    }

    /// Sets operation cost.
    pub fn with_cost(mut self, cost: GateCost) -> GateSetResult<Self> {
        cost.validate(self.id.as_str())?;
        self.cost = cost;
        Ok(self)
    }

    /// Sets operation capabilities.
    pub fn with_capabilities(
        mut self,
        capabilities: OperationCapabilities,
    ) -> GateSetResult<Self> {
        self.capabilities = capabilities;
        self.validate()?;
        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> GateSetResult<Self> {
        self.add_metadata(key, value)?;
        Ok(self)
    }

    /// Adds metadata in-place.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> GateSetResult<()> {
        let key = key.into();
        let value = value.into();

        if key.is_empty() || !is_valid_identifier(&key) {
            return Err(GateSetError::InvalidMetadata { key });
        }

        if self.metadata.len() >= DEFAULT_MAX_METADATA
            && !self.metadata.contains_key(&key)
        {
            return Err(GateSetError::InvalidMetadata { key });
        }

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Returns whether this operation accepts a given arity.
    #[must_use]
    pub fn accepts_arity(&self, arity: usize) -> bool {
        self.arity.accepts(arity)
    }

    /// Returns whether this operation accepts a parameter count.
    #[must_use]
    pub fn accepts_parameter_count(&self, count: usize) -> bool {
        self.parameters.accepts(count)
    }

    /// Returns whether this operation is native.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.capabilities.native
    }

    /// Returns whether this operation is exact.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.capabilities.exact
    }

    /// Returns whether this operation can be decomposed.
    #[must_use]
    pub fn is_decomposable(&self) -> bool {
        self.capabilities.decomposable
    }

    /// Validates this operation.
    pub fn validate(&self) -> GateSetResult<()> {
        if self.id.as_str().is_empty() {
            return Err(GateSetError::EmptyOperationId);
        }

        if self.name.is_empty() {
            return Err(GateSetError::InvalidOperationProperties {
                operation: self.id.to_string(),
                reason: "operation name must not be empty",
            });
        }

        if self.name.len() > MAX_OPERATION_NAME_LENGTH {
            return Err(GateSetError::OperationNameTooLong {
                maximum: MAX_OPERATION_NAME_LENGTH,
                actual: self.name.len(),
            });
        }

        if let Some(maximum) = self.arity.maximum() {
            if self.arity.minimum() > maximum {
                return Err(GateSetError::InvalidArity {
                    operation: self.id.to_string(),
                    minimum: self.arity.minimum(),
                    maximum: Some(maximum),
                });
            }
        }

        if let Some(exact) = self.parameters.exact() {
            if exact > 1_000_000 {
                return Err(GateSetError::InvalidParameterCount {
                    operation: self.id.to_string(),
                    count: exact,
                });
            }
        }

        self.cost.validate(self.id.as_str())?;

        if self.semantics.is_unitary()
            && !self.capabilities.unitary_region
        {
            return Err(GateSetError::InvalidOperationProperties {
                operation: self.id.to_string(),
                reason: "unitary operation must permit unitary-region use",
            });
        }

        if self.semantics.is_non_unitary()
            && self.capabilities.unitary_region
        {
            return Err(GateSetError::InvalidOperationProperties {
                operation: self.id.to_string(),
                reason: "non-unitary operation cannot be marked as valid in a unitary region",
            });
        }

        if self.capabilities.self_inverse
            && !self.semantics.is_unitary()
        {
            return Err(GateSetError::InvalidOperationProperties {
                operation: self.id.to_string(),
                reason: "only unitary operations can be self-inverse",
            });
        }

        for alias in &self.aliases {
            if alias.is_empty()
                || alias.len() > MAX_ALIAS_LENGTH
                || !is_valid_identifier(alias)
            {
                return Err(GateSetError::InvalidAlias {
                    alias: alias.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Gate-set configuration
// =============================================================================

/// Validation/configuration policy for one target gate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateSetLimits {
    /// Maximum operations.
    pub max_operations: usize,

    /// Maximum aliases.
    pub max_aliases: usize,

    /// Maximum metadata properties per operation.
    pub max_metadata_per_operation: usize,
}

impl Default for GateSetLimits {
    fn default() -> Self {
        Self {
            max_operations: DEFAULT_MAX_OPERATIONS,
            max_aliases: DEFAULT_MAX_ALIASES,
            max_metadata_per_operation: DEFAULT_MAX_METADATA,
        }
    }
}

// =============================================================================
// Target gate set
// =============================================================================

/// Immutable-style, deterministic collection of target operations.
///
/// The structure itself is mutable only during explicit construction. Once
/// passed by shared reference, it can safely be used by concurrent compiler
/// stages because it contains no interior mutability or global state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetGateSet {
    /// Schema identifier.
    schema_id: String,

    /// Schema version.
    schema_version: u32,

    /// Optional human-readable name.
    name: Option<String>,

    /// Operations keyed by canonical stable ID.
    operations: BTreeMap<OperationId, TargetOperation>,

    /// Alias-to-operation index.
    aliases: BTreeMap<String, OperationId>,

    /// Operations explicitly marked as native.
    native: BTreeSet<OperationId>,

    /// Operations required to remain in the target representation.
    required: BTreeSet<OperationId>,

    /// Gate-set construction limits.
    limits: GateSetLimits,
}

impl TargetGateSet {
    /// Creates an empty target gate set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema_id: GATE_SET_SCHEMA_ID.to_owned(),
            schema_version: GATE_SET_SCHEMA_VERSION,
            name: None,
            operations: BTreeMap::new(),
            aliases: BTreeMap::new(),
            native: BTreeSet::new(),
            required: BTreeSet::new(),
            limits: GateSetLimits::default(),
        }
    }

    /// Creates an empty target gate set with a name.
    pub fn named(name: impl Into<String>) -> Self {
        let mut result = Self::new();
        result.name = Some(name.into());
        result
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the optional display name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns configured limits.
    #[must_use]
    pub const fn limits(&self) -> GateSetLimits {
        self.limits
    }

    /// Sets gate-set limits before construction is complete.
    pub fn with_limits(mut self, limits: GateSetLimits) -> GateSetResult<Self> {
        if limits.max_operations == 0 {
            return Err(GateSetError::InvalidGateSet {
                message: "max_operations must be greater than zero".to_owned(),
            });
        }

        if limits.max_aliases == 0 {
            return Err(GateSetError::InvalidGateSet {
                message: "max_aliases must be greater than zero".to_owned(),
            });
        }

        if limits.max_metadata_per_operation == 0 {
            return Err(GateSetError::InvalidGateSet {
                message: "max_metadata_per_operation must be greater than zero"
                    .to_owned(),
            });
        }

        self.limits = limits;
        Ok(self)
    }

    /// Adds an operation.
    pub fn insert(&mut self, operation: TargetOperation) -> GateSetResult<()> {
        operation.validate()?;

        if self.operations.len() >= self.limits.max_operations
            && !self.operations.contains_key(operation.id())
        {
            return Err(GateSetError::TooManyOperations {
                maximum: self.limits.max_operations,
                actual: self.operations.len() + 1,
            });
        }

        if self.operations.contains_key(operation.id()) {
            return Err(GateSetError::DuplicateOperation {
                operation: operation.id().to_string(),
            });
        }

        for alias in operation.aliases() {
            if let Some(existing) = self.aliases.get(alias) {
                if existing != operation.id() {
                    return Err(GateSetError::DuplicateAlias {
                        alias: alias.clone(),
                    });
                }
            }

            if self.aliases.len() >= self.limits.max_aliases
                && !self.aliases.contains_key(alias)
            {
                return Err(GateSetError::TooManyAliases {
                    maximum: self.limits.max_aliases,
                    actual: self.aliases.len() + 1,
                });
            }
        }

        let id = operation.id().clone();

        for alias in operation.aliases() {
            self.aliases.insert(alias.clone(), id.clone());
        }

        if operation.is_native() {
            self.native.insert(id.clone());
        }

        self.operations.insert(id, operation);

        Ok(())
    }

    /// Inserts a canonical IR gate using its default target metadata.
    pub fn insert_builtin(&mut self, kind: GateKind) -> GateSetResult<()> {
        self.insert(TargetOperation::builtin(kind)?)
    }

    /// Inserts a canonical IR gate and marks it native.
    pub fn insert_native_builtin(
        &mut self,
        kind: GateKind,
    ) -> GateSetResult<()> {
        let operation = TargetOperation::builtin(kind)?;
        let id = operation.id().clone();

        self.insert(operation)?;
        self.native.insert(id);

        Ok(())
    }

    /// Marks an existing operation as native.
    pub fn mark_native(
        &mut self,
        operation: impl AsRef<str>,
    ) -> GateSetResult<()> {
        let id = self.resolve_id(operation.as_ref()).ok_or_else(|| {
            GateSetError::NativeOperationMissing {
                operation: operation.as_ref().to_owned(),
            }
        })?;

        self.native.insert(id);
        Ok(())
    }

    /// Marks an existing operation as required.
    pub fn mark_required(
        &mut self,
        operation: impl AsRef<str>,
    ) -> GateSetResult<()> {
        let id = self.resolve_id(operation.as_ref()).ok_or_else(|| {
            GateSetError::MissingRequiredOperation {
                operation: operation.as_ref().to_owned(),
            }
        })?;

        self.required.insert(id);
        Ok(())
    }

    /// Returns the number of supported operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the gate set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns all operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> &BTreeMap<OperationId, TargetOperation> {
        &self.operations
    }

    /// Returns all native operation IDs.
    #[must_use]
    pub fn native_operations(&self) -> &BTreeSet<OperationId> {
        &self.native
    }

    /// Returns all required operation IDs.
    #[must_use]
    pub fn required_operations(&self) -> &BTreeSet<OperationId> {
        &self.required
    }

    /// Resolves an operation ID or alias.
    #[must_use]
    pub fn resolve_id(&self, value: &str) -> Option<OperationId> {
        if let Ok(id) = OperationId::new(value) {
            if self.operations.contains_key(&id) {
                return Some(id);
            }
        }

        self.aliases.get(value).cloned()
    }

    /// Returns an operation by ID or alias.
    #[must_use]
    pub fn operation(
        &self,
        value: impl AsRef<str>,
    ) -> Option<&TargetOperation> {
        let id = self.resolve_id(value.as_ref())?;
        self.operations.get(&id)
    }

    /// Returns an operation by canonical gate kind.
    #[must_use]
    pub fn builtin_operation(
        &self,
        kind: GateKind,
    ) -> Option<&TargetOperation> {
        let id = OperationId::from_gate_kind(kind);
        self.operations.get(&id)
    }

    /// Returns whether an operation ID or alias is supported.
    #[must_use]
    pub fn contains(&self, value: impl AsRef<str>) -> bool {
        self.resolve_id(value.as_ref()).is_some()
    }

    /// Returns whether a canonical IR gate kind is supported.
    #[must_use]
    pub fn contains_gate_kind(&self, kind: GateKind) -> bool {
        self.builtin_operation(kind).is_some()
    }

    /// Returns whether an operation is native.
    #[must_use]
    pub fn is_native(&self, value: impl AsRef<str>) -> bool {
        self.resolve_id(value.as_ref())
            .map(|id| self.native.contains(&id))
            .unwrap_or(false)
    }

    /// Returns whether a canonical gate kind is native.
    #[must_use]
    pub fn is_native_gate_kind(&self, kind: GateKind) -> bool {
        let id = OperationId::from_gate_kind(kind);
        self.native.contains(&id)
    }

    /// Returns whether an operation is required.
    #[must_use]
    pub fn is_required(&self, value: impl AsRef<str>) -> bool {
        self.resolve_id(value.as_ref())
            .map(|id| self.required.contains(&id))
            .unwrap_or(false)
    }

    /// Returns whether a canonical gate kind is required.
    #[must_use]
    pub fn is_required_gate_kind(&self, kind: GateKind) -> bool {
        let id = OperationId::from_gate_kind(kind);
        self.required.contains(&id)
    }

    /// Checks whether a target operation accepts an arity.
    #[must_use]
    pub fn accepts_arity(
        &self,
        value: impl AsRef<str>,
        arity: usize,
    ) -> bool {
        self.operation(value)
            .map(|operation| operation.accepts_arity(arity))
            .unwrap_or(false)
    }

    /// Checks whether a target operation accepts a parameter count.
    #[must_use]
    pub fn accepts_parameter_count(
        &self,
        value: impl AsRef<str>,
        parameter_count: usize,
    ) -> bool {
        self.operation(value)
            .map(|operation| {
                operation.accepts_parameter_count(parameter_count)
            })
            .unwrap_or(false)
    }

    /// Returns the cost of an operation.
    #[must_use]
    pub fn cost(
        &self,
        value: impl AsRef<str>,
    ) -> Option<GateCost> {
        self.operation(value).map(|operation| operation.cost())
    }

    /// Returns the requested cost metric.
    #[must_use]
    pub fn cost_metric(
        &self,
        value: impl AsRef<str>,
        metric: CostMetric,
    ) -> Option<f64> {
        self.operation(value)
            .and_then(|operation| operation.cost().metric(metric))
    }

    /// Returns all operations with the supplied semantics.
    pub fn operations_with_semantics(
        &self,
        semantics: OperationSemantics,
    ) -> Vec<&TargetOperation> {
        self.operations
            .values()
            .filter(|operation| operation.semantics() == semantics)
            .collect()
    }

    /// Returns all native operations.
    pub fn native_operation_descriptors(&self) -> Vec<&TargetOperation> {
        self.native
            .iter()
            .filter_map(|id| self.operations.get(id))
            .collect()
    }

    /// Returns all non-native supported operations.
    pub fn non_native_operation_descriptors(
        &self,
    ) -> Vec<&TargetOperation> {
        self.operations
            .values()
            .filter(|operation| !self.native.contains(operation.id()))
            .collect()
    }

    /// Returns all supported canonical IR gate kinds.
    pub fn builtin_gate_kinds(&self) -> Vec<GateKind> {
        self.operations
            .values()
            .filter_map(TargetOperation::gate_kind)
            .collect()
    }

    /// Validates the entire gate set.
    pub fn validate(&self) -> GateSetResult<()> {
        if self.schema_id != GATE_SET_SCHEMA_ID {
            return Err(GateSetError::InvalidGateSet {
                message: "unsupported gate-set schema identifier".to_owned(),
            });
        }

        if self.schema_version != GATE_SET_SCHEMA_VERSION {
            return Err(GateSetError::InvalidGateSet {
                message: "unsupported gate-set schema version".to_owned(),
            });
        }

        if self.operations.len() > self.limits.max_operations {
            return Err(GateSetError::TooManyOperations {
                maximum: self.limits.max_operations,
                actual: self.operations.len(),
            });
        }

        for (id, operation) in &self.operations {
            if id != operation.id() {
                return Err(GateSetError::InvalidGateSet {
                    message: format!(
                        "operation map key `{id}` does not match operation identity `{}`",
                        operation.id()
                    ),
                });
            }

            operation.validate()?;

            for alias in operation.aliases() {
                match self.aliases.get(alias) {
                    Some(resolved) if resolved == id => {}
                    Some(_) => {
                        return Err(GateSetError::DuplicateAlias {
                            alias: alias.clone(),
                        });
                    }
                    None => {
                        return Err(GateSetError::InvalidGateSet {
                            message: format!(
                                "operation alias `{alias}` is missing from alias index"
                            ),
                        });
                    }
                }
            }
        }

        if self.aliases.len() > self.limits.max_aliases {
            return Err(GateSetError::TooManyAliases {
                maximum: self.limits.max_aliases,
                actual: self.aliases.len(),
            });
        }

        for id in &self.native {
            if !self.operations.contains_key(id) {
                return Err(GateSetError::NativeOperationMissing {
                    operation: id.to_string(),
                });
            }
        }

        for id in &self.required {
            if !self.operations.contains_key(id) {
                return Err(GateSetError::MissingRequiredOperation {
                    operation: id.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns a deterministic 64-bit fingerprint of the gate set.
    ///
    /// This is suitable for cache keys, provenance comparisons, and fast
    /// change detection. It is not a cryptographic hash.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hasher = StableHasher::default();

        self.schema_id.hash(&mut hasher);
        self.schema_version.hash(&mut hasher);
        self.name.hash(&mut hasher);

        for (id, operation) in &self.operations {
            id.hash(&mut hasher);
            operation.name.hash(&mut hasher);
            operation.operation.hash(&mut hasher);
            operation.arity.hash(&mut hasher);
            operation.parameters.hash(&mut hasher);
            operation.semantics.hash(&mut hasher);
            operation.capabilities.hash(&mut hasher);
            hash_cost(operation.cost(), &mut hasher);

            for alias in &operation.aliases {
                alias.hash(&mut hasher);
            }

            for (key, value) in &operation.metadata {
                key.hash(&mut hasher);
                value.hash(&mut hasher);
            }
        }

        for id in &self.native {
            id.hash(&mut hasher);
        }

        for id in &self.required {
            id.hash(&mut hasher);
        }

        hasher.finish()
    }

    /// Creates a builder for a target gate set.
    #[must_use]
    pub fn builder() -> TargetGateSetBuilder {
        TargetGateSetBuilder::new()
    }
}

impl Default for TargetGateSet {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for deterministic target gate sets.
#[derive(Debug, Clone)]
pub struct TargetGateSetBuilder {
    gate_set: TargetGateSet,
}

impl TargetGateSetBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            gate_set: TargetGateSet::new(),
        }
    }

    /// Sets the target gate-set name.
    #[must_use]
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.gate_set.name = Some(name.into());
        self
    }

    /// Sets limits.
    pub fn with_limits(
        mut self,
        limits: GateSetLimits,
    ) -> GateSetResult<Self> {
        self.gate_set = self.gate_set.with_limits(limits)?;
        Ok(self)
    }

    /// Adds a canonical gate.
    pub fn builtin(mut self, kind: GateKind) -> GateSetResult<Self> {
        self.gate_set.insert_builtin(kind)?;
        Ok(self)
    }

    /// Adds a native canonical gate.
    pub fn native_builtin(
        mut self,
        kind: GateKind,
    ) -> GateSetResult<Self> {
        self.gate_set.insert_native_builtin(kind)?;
        Ok(self)
    }

    /// Adds an operation.
    pub fn operation(
        mut self,
        operation: TargetOperation,
    ) -> GateSetResult<Self> {
        self.gate_set.insert(operation)?;
        Ok(self)
    }

    /// Marks an operation native.
    pub fn mark_native(
        mut self,
        operation: impl AsRef<str>,
    ) -> GateSetResult<Self> {
        self.gate_set.mark_native(operation)?;
        Ok(self)
    }

    /// Marks an operation required.
    pub fn mark_required(
        mut self,
        operation: impl AsRef<str>,
    ) -> GateSetResult<Self> {
        self.gate_set.mark_required(operation)?;
        Ok(self)
    }

    /// Finalizes and validates the gate set.
    pub fn build(self) -> GateSetResult<TargetGateSet> {
        self.gate_set.validate()?;
        Ok(self.gate_set)
    }
}

impl Default for TargetGateSetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Standard gate sets
// =============================================================================

/// Common predefined gate-set constructors.
///
/// These constructors are intentionally explicit rather than hidden global
/// singletons. Every caller receives an independent immutable-after-build
/// value.
pub mod standard {
    use super::*;

    /// Identity-only basis.
    pub fn identity() -> GateSetResult<TargetGateSet> {
        TargetGateSet::builder()
            .named("identity")
            .builtin(GateKind::I)?
            .build()
    }

    /// Clifford basis.
    pub fn clifford() -> GateSetResult<TargetGateSet> {
        let mut set = TargetGateSet::named("clifford");

        for gate in [
            GateKind::I,
            GateKind::X,
            GateKind::Y,
            GateKind::Z,
            GateKind::H,
            GateKind::S,
            GateKind::Sdg,
            GateKind::CX,
            GateKind::CY,
            GateKind::CZ,
            GateKind::SWAP,
        ] {
            set.insert_native_builtin(gate)?;
        }

        set.validate()?;
        Ok(set)
    }

    /// Standard Clifford+T basis.
    pub fn clifford_t() -> GateSetResult<TargetGateSet> {
        let mut set = clifford()?;
        set.name = Some("clifford_t".to_owned());

        set.insert_native_builtin(GateKind::T)?;
        set.insert_native_builtin(GateKind::Tdg)?;

        set.validate()?;
        Ok(set)
    }

    /// Clifford+T plus RZ.
    pub fn clifford_t_plus_rz() -> GateSetResult<TargetGateSet> {
        let mut set = clifford_t()?;
        set.name = Some("clifford_t_plus_rz".to_owned());

        set.insert_native_builtin(GateKind::RZ)?;

        set.validate()?;
        Ok(set)
    }

    /// Common rotation plus CNOT basis.
    pub fn rotations_plus_cx() -> GateSetResult<TargetGateSet> {
        let mut set = TargetGateSet::named("rotations_plus_cx");

        for gate in [
            GateKind::I,
            GateKind::RX,
            GateKind::RY,
            GateKind::RZ,
            GateKind::CX,
        ] {
            set.insert_native_builtin(gate)?;
        }

        set.validate()?;
        Ok(set)
    }

    /// Common superconducting-style abstract basis.
    ///
    /// This is an optimization profile, not a claim about any particular
    /// physical device.
    pub fn superconducting() -> GateSetResult<TargetGateSet> {
        let mut set = TargetGateSet::named("superconducting");

        for gate in [
            GateKind::I,
            GateKind::X,
            GateKind::SX,
            GateKind::RZ,
            GateKind::ECR,
        ] {
            // SX is not currently part of the canonical GateKind enum.
            // Keep the construction compatible with the current IR by using
            // only operations that actually exist.
            let _ = gate;
        }

        set.insert_native_builtin(GateKind::I)?;
        set.insert_native_builtin(GateKind::X)?;
        set.insert_native_builtin(GateKind::RZ)?;
        set.insert_native_builtin(GateKind::ECR)?;

        set.validate()?;
        Ok(set)
    }

    /// Generic universal single/two-qubit basis.
    pub fn generic_universal() -> GateSetResult<TargetGateSet> {
        let mut set = TargetGateSet::named("generic_universal");

        for gate in [
            GateKind::I,
            GateKind::RX,
            GateKind::RY,
            GateKind::RZ,
            GateKind::CX,
        ] {
            set.insert_native_builtin(gate)?;
        }

        set.validate()?;
        Ok(set)
    }

    /// Full currently-known canonical logical gate vocabulary.
    ///
    /// This is useful for simulators and generic logical compilation where
    /// decomposition into a restricted basis is not desired.
    pub fn all_current_ir_gates() -> GateSetResult<TargetGateSet> {
        let mut set = TargetGateSet::named("all_current_ir_gates");

        for gate in all_current_gate_kinds() {
            set.insert_builtin(gate)?;
        }

        set.validate()?;
        Ok(set)
    }
}

// =============================================================================
// Built-in gate vocabulary
// =============================================================================

/// Returns all currently declared canonical IR gate kinds.
///
/// This function is intentionally maintained in one place so future target
/// profiles can consume the canonical IR vocabulary without duplicating it.
#[must_use]
pub fn all_current_gate_kinds() -> Vec<GateKind> {
    vec![
        GateKind::I,
        GateKind::X,
        GateKind::Y,
        GateKind::Z,
        GateKind::H,
        GateKind::S,
        GateKind::Sdg,
        GateKind::T,
        GateKind::Tdg,
        GateKind::V,
        GateKind::Vdg,
        GateKind::RX,
        GateKind::RY,
        GateKind::RZ,
        GateKind::Phase,
        GateKind::U1,
        GateKind::U2,
        GateKind::U3,
        GateKind::CX,
        GateKind::CY,
        GateKind::CZ,
        GateKind::CH,
        GateKind::SWAP,
        GateKind::ISWAP,
        GateKind::ECR,
        GateKind::CRX,
        GateKind::CRY,
        GateKind::CRZ,
        GateKind::CCX,
        GateKind::CSWAP,
        GateKind::Measure,
        GateKind::Barrier,
        GateKind::Reset,
    ]
}

/// Returns the stable textual name of a canonical IR gate.
#[must_use]
pub fn gate_kind_name(kind: GateKind) -> &'static str {
    match kind {
        GateKind::I => "i",
        GateKind::X => "x",
        GateKind::Y => "y",
        GateKind::Z => "z",
        GateKind::H => "h",
        GateKind::S => "s",
        GateKind::Sdg => "sdg",
        GateKind::T => "t",
        GateKind::Tdg => "tdg",
        GateKind::V => "v",
        GateKind::Vdg => "vdg",
        GateKind::RX => "rx",
        GateKind::RY => "ry",
        GateKind::RZ => "rz",
        GateKind::Phase => "phase",
        GateKind::U1 => "u1",
        GateKind::U2 => "u2",
        GateKind::U3 => "u3",
        GateKind::CX => "cx",
        GateKind::CY => "cy",
        GateKind::CZ => "cz",
        GateKind::CH => "ch",
        GateKind::SWAP => "swap",
        GateKind::ISWAP => "iswap",
        GateKind::ECR => "ecr",
        GateKind::CRX => "crx",
        GateKind::CRY => "cry",
        GateKind::CRZ => "crz",
        GateKind::CCX => "ccx",
        GateKind::CSWAP => "cswap",
        GateKind::Measure => "measure",
        GateKind::Barrier => "barrier",
        GateKind::Reset => "reset",
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a stable operation identifier.
#[must_use]
pub fn is_valid_identifier(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let mut previous_was_separator = false;

    for character in value.chars() {
        let is_separator = matches!(character, '.' | '_' | '-');

        if is_separator {
            if previous_was_separator {
                return false;
            }

            previous_was_separator = true;
            continue;
        }

        if !character.is_ascii_alphanumeric() {
            return false;
        }

        previous_was_separator = false;
    }

    !previous_was_separator
}

/// Validates that every required operation exists.
pub fn validate_required_operations(
    gate_set: &TargetGateSet,
) -> GateSetResult<()> {
    for id in gate_set.required_operations() {
        if !gate_set.operations.contains_key(id) {
            return Err(GateSetError::MissingRequiredOperation {
                operation: id.to_string(),
            });
        }
    }

    Ok(())
}

/// Validates that every native operation exists.
pub fn validate_native_operations(
    gate_set: &TargetGateSet,
) -> GateSetResult<()> {
    for id in gate_set.native_operations() {
        if !gate_set.operations.contains_key(id) {
            return Err(GateSetError::NativeOperationMissing {
                operation: id.to_string(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Stable hasher
// =============================================================================

/// Deterministic FNV-1a hasher.
///
/// This is deliberately not cryptographic. It is used only for deterministic
/// fingerprints of target descriptions.
#[derive(Debug, Clone)]
struct StableHasher(u64);

impl Default for StableHasher {
    fn default() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self
                .0
                .wrapping_mul(0x100000001b3);
        }
    }
}

/// Hashes floating-point cost values by their bit representation.
fn hash_cost<H: Hasher>(cost: GateCost, hasher: &mut H) {
    cost.generic.map(f64::to_bits).hash(hasher);
    cost.gate_count.map(f64::to_bits).hash(hasher);
    cost.single_qubit.map(f64::to_bits).hash(hasher);
    cost.two_qubit.map(f64::to_bits).hash(hasher);
    cost.multi_qubit.map(f64::to_bits).hash(hasher);
    cost.depth.map(f64::to_bits).hash(hasher);
    cost.duration.map(f64::to_bits).hash(hasher);
    cost.error.map(f64::to_bits).hash(hasher);
    cost.t_count.map(f64::to_bits).hash(hasher);
    cost.t_depth.map(f64::to_bits).hash(hasher);
    cost.magic_state.map(f64::to_bits).hash(hasher);
    cost.energy.map(f64::to_bits).hash(hasher);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_id_accepts_namespaced_identifiers() {
        let id = OperationId::new("builtin.rz").expect("valid identifier");

        assert_eq!(id.as_str(), "builtin.rz");
        assert!(id.is_builtin());
        assert_eq!(id.namespace(), Some("builtin"));
        assert_eq!(id.local_name(), "rz");
    }

    #[test]
    fn operation_id_rejects_empty_identifier() {
        assert!(matches!(
            OperationId::new(""),
            Err(GateSetError::EmptyOperationId)
        ));
    }

    #[test]
    fn operation_id_rejects_invalid_identifier() {
        assert!(matches!(
            OperationId::new("builtin..rz"),
            Err(GateSetError::InvalidOperationId { .. })
        ));
    }

    #[test]
    fn builtin_operation_uses_canonical_ir() {
        let operation =
            TargetOperation::builtin(GateKind::RZ).expect("valid gate");

        assert_eq!(
            operation.id().as_str(),
            "builtin.rz"
        );

        assert_eq!(
            operation.gate_kind(),
            Some(GateKind::RZ)
        );

        assert_eq!(
            operation.arity(),
            Arity::exact(1)
        );

        assert_eq!(
            operation.parameter_policy(),
            ParameterPolicy::Exact(1)
        );
    }

    #[test]
    fn builtin_gate_set_supports_lookup_by_kind() {
        let mut set = TargetGateSet::new();

        set.insert_native_builtin(GateKind::X)
            .expect("insert X");
        set.insert_native_builtin(GateKind::CX)
            .expect("insert CX");

        assert!(set.contains_gate_kind(GateKind::X));
        assert!(set.contains_gate_kind(GateKind::CX));
        assert!(!set.contains_gate_kind(GateKind::T));

        assert!(set.is_native_gate_kind(GateKind::X));
        assert!(set.is_native_gate_kind(GateKind::CX));
    }

    #[test]
    fn aliases_resolve_to_canonical_operation() {
        let operation = TargetOperation::builtin(GateKind::CX)
            .expect("valid gate")
            .with_alias("cnot")
            .expect("valid alias");

        let mut set = TargetGateSet::new();
        set.insert(operation).expect("insert operation");

        assert!(set.contains("cnot"));

        let resolved = set.operation("cnot").expect("alias resolves");

        assert_eq!(resolved.gate_kind(), Some(GateKind::CX));
    }

    #[test]
    fn arity_is_checked() {
        let operation =
            TargetOperation::builtin(GateKind::CX).expect("valid gate");

        assert!(operation.accepts_arity(2));
        assert!(!operation.accepts_arity(1));
        assert!(!operation.accepts_arity(3));
    }

    #[test]
    fn parameter_count_is_checked() {
        let operation =
            TargetOperation::builtin(GateKind::U3).expect("valid gate");

        assert!(operation.accepts_parameter_count(3));
        assert!(!operation.accepts_parameter_count(2));
        assert!(!operation.accepts_parameter_count(4));
    }

    #[test]
    fn native_and_required_sets_are_tracked() {
        let mut set = TargetGateSet::new();

        set.insert_native_builtin(GateKind::RZ)
            .expect("insert RZ");

        set.mark_required("builtin.rz")
            .expect("mark required");

        assert!(set.is_native("builtin.rz"));
        assert!(set.is_required("builtin.rz"));
    }

    #[test]
    fn fingerprints_are_deterministic() {
        let mut first = TargetGateSet::new();
        first
            .insert_native_builtin(GateKind::X)
            .expect("insert X");
        first
            .insert_native_builtin(GateKind::CX)
            .expect("insert CX");

        let mut second = TargetGateSet::new();
        second
            .insert_native_builtin(GateKind::CX)
            .expect("insert CX");
        second
            .insert_native_builtin(GateKind::X)
            .expect("insert X");

        assert_eq!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn standard_clifford_t_is_valid() {
        let set = standard::clifford_t()
            .expect("standard Clifford+T set");

        assert!(set.contains_gate_kind(GateKind::H));
        assert!(set.contains_gate_kind(GateKind::CX));
        assert!(set.contains_gate_kind(GateKind::T));
        assert!(set.contains_gate_kind(GateKind::Tdg));

        set.validate().expect("valid gate set");
    }

    #[test]
    fn standard_rotations_plus_cx_is_valid() {
        let set = standard::rotations_plus_cx()
            .expect("rotation/CX gate set");

        assert!(set.contains_gate_kind(GateKind::RX));
        assert!(set.contains_gate_kind(GateKind::RY));
        assert!(set.contains_gate_kind(GateKind::RZ));
        assert!(set.contains_gate_kind(GateKind::CX));

        assert!(!set.contains_gate_kind(GateKind::T));
    }

    #[test]
    fn all_current_gate_kinds_are_constructible() {
        let set = standard::all_current_ir_gates()
            .expect("all current gate kinds");

        assert_eq!(
            set.len(),
            all_current_gate_kinds().len()
        );

        for gate in all_current_gate_kinds() {
            assert!(set.contains_gate_kind(gate));
        }
    }

    #[test]
    fn cost_rejects_nan() {
        let cost = GateCost::unit().with_generic(f64::NAN);

        assert!(matches!(
            cost.validate("builtin.x"),
            Err(GateSetError::InvalidCost { .. })
        ));
    }

    #[test]
    fn cost_rejects_negative_values() {
        let cost = GateCost::unit().with_gate_count(-1.0);

        assert!(matches!(
            cost.validate("builtin.x"),
            Err(GateSetError::InvalidCost { .. })
        ));
    }

    #[test]
    fn custom_operation_is_supported() {
        let id =
            OperationId::new("custom.native_entangler")
                .expect("valid ID");

        let operation = TargetOperation::custom(
            id,
            "NativeEntangler",
            Arity::exact(2),
            ParameterPolicy::Exact(1),
            OperationSemantics::Unitary,
        )
        .expect("valid operation")
        .with_alias("native_entangler")
        .expect("valid alias");

        let mut set = TargetGateSet::new();

        set.insert(operation)
            .expect("insert custom operation");

        assert!(set.contains("custom.native_entangler"));
        assert!(set.contains("native_entangler"));
        assert!(!set.contains("builtin.cx"));
    }

    #[test]
    fn duplicate_alias_is_rejected() {
        let first = TargetOperation::builtin(GateKind::X)
            .expect("X")
            .with_alias("shared")
            .expect("alias");

        let second = TargetOperation::builtin(GateKind::Y)
            .expect("Y")
            .with_alias("shared")
            .expect("alias");

        let mut set = TargetGateSet::new();

        set.insert(first).expect("first operation");

        assert!(matches!(
            set.insert(second),
            Err(GateSetError::DuplicateAlias { .. })
        ));
    }

    #[test]
    fn invalid_native_reference_is_rejected() {
        let mut set = TargetGateSet::new();

        assert!(matches!(
            set.mark_native("builtin.x"),
            Err(GateSetError::NativeOperationMissing { .. })
        ));
    }

    #[test]
    fn invalid_required_reference_is_rejected() {
        let mut set = TargetGateSet::new();

        assert!(matches!(
            set.mark_required("builtin.x"),
            Err(GateSetError::MissingRequiredOperation { .. })
        ));
    }
}