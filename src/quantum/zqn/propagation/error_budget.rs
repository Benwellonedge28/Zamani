//! Zamani Quantum Noise (ZQN) — Error Budget
//!
//! Path:
//!     src/quantum/zqn/propagation/error_budget.rs
//!
//! # Purpose
//!
//! This module defines the target-independent semantic model for an error
//! budget: a quantitative contract describing how much error a computation,
//! operation, resource, subsystem, or result may consume before violating an
//! explicitly requested tolerance.
//!
//! An error budget is deliberately different from an error estimate:
//!
//! ```text
//! Error budget
//!     = allowed / required tolerance
//!
//! Error estimate
//!     = predicted / observed error
//!
//! Error budget evaluation
//!     = comparison between the two
//! ```
//!
//! This distinction is essential for scientific correctness.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - error-budget identity metadata;
//! - budget dimensions;
//! - tolerance semantics;
//! - error-budget entries;
//! - budget allocation;
//! - budget consumption;
//! - deterministic budget evaluation;
//! - remaining-budget calculations;
//! - exhaustion/violation classification;
//! - explicit aggregation semantics;
//! - composition of independently defined budgets;
//! - validation of local budget invariants;
//! - portable serialization representation;
//! - stable error-budget semantics.
//!
//! This file does NOT own:
//!
//! - quantum channels;
//! - quantum states;
//! - fidelity definitions;
//! - uncertainty distributions;
//! - numerical propagation algorithms;
//! - noise models;
//! - fault generation;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulation;
//! - benchmarking methodology;
//! - runtime resource accounting;
//! - canonical qubit identity.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      │ semantic computation
//!      ▼
//! ZQN noise / calibration / characterization
//!      │
//!      ▼
//! propagation::error_budget
//!      │
//!      ├── allowed error
//!      ├── allocation
//!      ├── consumption
//!      └── compliance
//!      │
//!      ├───────────────┬────────────────┐
//!      ▼               ▼                ▼
//! fidelity       uncertainty       accumulation
//!      │               │                │
//!      └───────────────┼────────────────┘
//!                      ▼
//!             routing / scheduling / QEC
//!                      │
//!                      ▼
//!                    target
//! ```
//!
//! # Canonical quantum-resource identity
//!
//! An error budget may eventually be associated with logical or physical
//! resources, but this module does not create a second qubit identity system.
//!
//! When a higher-level API needs to associate a budget with a concrete quantum
//! resource, it must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This low-level budget representation therefore does not need to depend on
//! `quantum::ir::qubit` merely to perform arithmetic.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_BUDGET_ENTRIES
//! MAX_ERROR
//! MAX_DEPTH
//! ```
//!
//! An error budget can describe any finite workload representable by the
//! surrounding execution/storage infrastructure.
//!
//! "Infinity" in Zamani means that ZQN imposes no artificial architectural
//! ceiling. It does not mean that a finite machine can materialize infinite
//! state or infinite metadata.
//!
//! Concrete resource restrictions belong to `ZqnLimits`, runtime policy,
//! target capabilities, allocator availability, and other explicit execution
//! policies.
//!
//! # Mathematical semantics
//!
//! A budget is represented as a collection of independent named dimensions.
//!
//! For each dimension `d`:
//!
//! ```text
//! allowed(d) >= 0
//! consumed(d) >= 0
//! remaining(d) = allowed(d) - consumed(d)
//! ```
//!
//! Compliance is:
//!
//! ```text
//! consumed(d) <= allowed(d)
//! ```
//!
//! for every enforced dimension.
//!
//! A dimension may optionally be marked as informational rather than enforced.
//! Informational dimensions are retained for analysis/provenance but do not
//! cause budget violation.
//!
//! # Important numerical rule
//!
//! This module does not silently convert invalid numerical values.
//!
//! It rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative error quantities;
//! - negative tolerances.
//!
//! It also does not silently clamp values.
//!
//! For example:
//!
//! ```text
//! NaN      -> error
//! -0.1     -> error
//! 1.2      -> valid for a generic non-probability error metric
//! ```
//!
//! An error quantity is NOT automatically assumed to be a probability.
//!
//! Probability-specific validation belongs to the probability subsystem.
//!
//! # Aggregation semantics
//!
//! Error accumulation is not universally additive.
//!
//! Therefore this module does not secretly assume:
//!
//! ```text
//! total = sum(errors)
//! ```
//!
//! Instead it explicitly supports aggregation policies:
//!
//! - `Sum`;
//! - `Maximum`;
//! - `RootSumSquare`;
//! - `Custom`.
//!
//! `Custom` records the fact that a downstream propagator owns the actual
//! mathematical aggregation. This module does not pretend to implement an
//! arbitrary mathematical function.
//!
//! `RootSumSquare` is useful when independent contributions are conventionally
//! combined in quadrature, but it must only be selected when that assumption is
//! scientifically justified by the caller.
//!
//! # Budget allocation
//!
//! A parent budget can be divided into named child allocations.
//!
//! Example:
//!
//! ```text
//! total error budget
//! ├── preparation
//! ├── gates
//! ├── idle
//! ├── measurement
//! └── transport
//! ```
//!
//! Allocation is a policy operation. It does not assert that the physical
//! system actually produces errors according to that allocation.
//!
//! # Composition
//!
//! Two budgets can be composed explicitly.
//!
//! Composition never silently chooses a physical interpretation.
//!
//! The caller must specify the desired composition policy.
//!
//! # Determinism
//!
//! All operations in this file are deterministic.
//!
//! There is:
//!
//! - no RNG;
//! - no global mutable state;
//! - no wall-clock dependency;
//! - no process identity dependency;
//! - no thread identity dependency;
//! - no hash-map iteration dependency.
//!
//! Ordered collections use `BTreeMap`/`BTreeSet` where ordering is semantically
//! useful for deterministic serialization and evaluation.
//!
//! # Parallel execution
//!
//! The semantic result of evaluating a fixed budget must not depend on whether
//! independent entries were evaluated sequentially or concurrently.
//!
//! This module therefore exposes pure value-based aggregation primitives.
//!
//! Parallel orchestration belongs to higher-level propagation/execution layers.
//!
//! # Resource safety
//!
//! This module never performs implicit unbounded work.
//!
//! The caller owns the collection supplied to the budget. For large workloads,
//! callers should aggregate contributions incrementally instead of constructing
//! a gigantic collection solely for budget evaluation.
//!
//! Checked arithmetic is used wherever integer accounting is performed.
//!
//! Floating-point arithmetic is validated for finiteness at API boundaries.
//!
//! # Serialization
//!
//! The semantic structure is `serde`-serializable, but this file does not own
//! the repository-wide wire schema.
//!
//! `propagation`/`io` may wrap this representation in a versioned schema.
//!
//! Serialized ordering must remain deterministic.
//!
//! # Versioning
//!
//! `ERROR_BUDGET_SCHEMA_VERSION` identifies the semantic representation.
//!
//! Adding a new aggregation variant is a compatibility-sensitive operation.
//! Existing variants must not silently change meaning.
//!
//! # Integration with existing ZQN IDs
//!
//! The repository already reserves `ErrorBudgetId` in:
//!
//! ```text
//! quantum::zqn::core::ids
//! ```
//!
//! This module intentionally does not assume a constructor or representation
//! for that ID. Higher-level ZQN integration can attach the canonical
//! `ErrorBudgetId` without coupling the arithmetic core to ID allocation.
//!
//! # Integration with fault-tolerant IR
//!
//! The Quantum IR fault-tolerant dialect already exposes logical error-budget
//! requirements. This file provides the richer physical/propagation-side
//! representation.
//!
//! Conceptually:
//!
//! ```text
//! quantum::ir::dialect::fault_tolerant::LogicalErrorBudget
//!                     │
//!                     ▼
//!               integration layer
//!                     │
//!                     ▼
//! zqn::propagation::error_budget::ErrorBudget
//! ```
//!
//! The conversion belongs at the integration boundary rather than making this
//! file depend on the fault-tolerant dialect.
//!
//! # Integration with future propagation files
//!
//! `uncertainty.rs` may consume `BudgetRequirement`.
//!
//! `fidelity.rs` may consume `ErrorBudget` and compare fidelity-derived error
//! measures against a budget.
//!
//! `bounds.rs` may evaluate conservative upper bounds against the budget.
//!
//! `sensitivity.rs` may determine which budget dimension is most influential.
//!
//! `accumulation.rs` may generate `BudgetConsumption` values and use the
//! aggregation primitives here.
//!
//! None of those modules needs to modify this file merely because it is added.
//!
//! # Integration with routing
//!
//! `integration::routing` can query remaining budget or construct a budget
//! requirement for candidate placements.
//!
//! ZQN does not decide routing.
//!
//! # Integration with scheduling
//!
//! `integration::scheduling` can use a budget to determine whether additional
//! idle time or operation duration remains acceptable.
//!
//! ZQN does not decide scheduling.
//!
//! # Integration with QEC
//!
//! `integration::qec` can map logical error-budget requirements into physical
//! budget constraints and compare predicted logical error against a target.
//!
//! ZQN does not implement a decoder or QEC code.
//!
//! # Integration with benchmarking
//!
//! Benchmarking can use this module to express acceptance thresholds and
//! evaluate measured error against those thresholds.
//!
//! This module does not own benchmark methodology.
//!
//! # Integration with hardware
//!
//! Hardware adapters may provide measured error estimates and calibration data.
//!
//! They do not need to know how an error budget is internally represented.
//!
//! # Integration with runtime
//!
//! Runtime policy may impose resource limits around budget evaluation.
//!
//! An error budget is a semantic tolerance, not a memory/CPU limit.
//!
//! # Security
//!
//! External budget data must be treated as untrusted input.
//!
//! Deserialization and higher-level ingestion must reject:
//!
//! - non-finite values;
//! - invalid aggregation policies;
//! - empty identifiers where identifiers are required;
//! - negative tolerances;
//! - inconsistent entry state;
//! - resource-exhausting payloads according to the active `ZqnLimits`.
//!
//! This file does not allocate based on untrusted counts without an owning
//! collection/API deciding to do so.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Current semantic schema version for [`ErrorBudget`].
pub const ERROR_BUDGET_SCHEMA_VERSION: u32 = 1;

/// Result type for this module.
pub type ErrorBudgetResult<T> = Result<T, ErrorBudgetError>;

/// A finite non-negative error quantity.
///
/// This is intentionally NOT a probability. Values may represent quantities
/// such as infidelity, distance, timing error, energy error, or another
/// explicitly defined metric.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ErrorQuantity(f64);

impl ErrorQuantity {
    /// Creates a validated error quantity.
    pub fn new(value: f64) -> ErrorBudgetResult<Self> {
        validate_finite_non_negative("error quantity", value)?;
        Ok(Self(value))
    }

    /// Returns zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns whether this quantity is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Adds two quantities using checked floating-point validation.
    pub fn checked_add(self, other: Self) -> ErrorBudgetResult<Self> {
        let value = self.0 + other.0;

        if !value.is_finite() {
            return Err(ErrorBudgetError::NumericalOverflow {
                operation: "error quantity addition",
            });
        }

        Self::new(value)
    }

    /// Multiplies a quantity by a finite non-negative scalar.
    pub fn checked_mul(self, scalar: f64) -> ErrorBudgetResult<Self> {
        validate_finite_non_negative("scalar", scalar)?;

        let value = self.0 * scalar;

        if !value.is_finite() {
            return Err(ErrorBudgetError::NumericalOverflow {
                operation: "error quantity multiplication",
            });
        }

        Self::new(value)
    }
}

impl Default for ErrorQuantity {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for ErrorQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.17}", self.0)
    }
}

/// A non-negative error tolerance.
///
/// A tolerance is semantically an allowed amount of error.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ErrorTolerance(f64);

impl ErrorTolerance {
    /// Creates a validated tolerance.
    pub fn new(value: f64) -> ErrorBudgetResult<Self> {
        validate_finite_non_negative("error tolerance", value)?;
        Ok(Self(value))
    }

    /// Returns zero tolerance.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns whether this tolerance is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Default for ErrorTolerance {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for ErrorTolerance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.17}", self.0)
    }
}

/// Determines how multiple error contributions are aggregated.
///
/// This enum intentionally does not claim that one aggregation rule is
/// universally physically correct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Aggregation {
    /// Add independent contributions.
    Sum,

    /// Use the largest contribution.
    Maximum,

    /// Combine contributions using root-sum-square.
    RootSumSquare,

    /// Aggregation is owned by a higher-level algorithm.
    Custom,
}

impl Default for Aggregation {
    fn default() -> Self {
        Self::Sum
    }
}

impl fmt::Display for Aggregation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Sum => "sum",
            Self::Maximum => "maximum",
            Self::RootSumSquare => "root_sum_square",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

/// Whether a budget dimension is enforced or informational.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BudgetEnforcement {
    /// Exceeding the tolerance makes the budget non-compliant.
    Enforced,

    /// The dimension is recorded for analysis but does not cause failure.
    Informational,
}

impl Default for BudgetEnforcement {
    fn default() -> Self {
        Self::Enforced
    }
}

/// A named error-budget dimension.
///
/// Dimension names are deliberately strings rather than a closed enum so that
/// new physical metrics can be introduced without changing this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BudgetDimension(String);

impl BudgetDimension {
    /// Creates a dimension name.
    pub fn new(value: impl Into<String>) -> ErrorBudgetResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ErrorBudgetError::EmptyDimension);
        }

        if value.trim() != value {
            return Err(ErrorBudgetError::InvalidDimension {
                reason: "leading or trailing whitespace",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(ErrorBudgetError::InvalidDimension {
                reason: "control character",
            });
        }

        Ok(Self(value))
    }

    /// Returns the dimension name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BudgetDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A single budget allocation.
///
/// An allocation says how much of the parent budget is intentionally reserved
/// for a named concern.
///
/// Allocation itself does not claim that the physical error will obey that
/// division.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetAllocation {
    dimension: BudgetDimension,
    tolerance: ErrorTolerance,
    enforcement: BudgetEnforcement,
}

impl BudgetAllocation {
    /// Creates an enforced allocation.
    pub fn enforced(
        dimension: BudgetDimension,
        tolerance: ErrorTolerance,
    ) -> Self {
        Self {
            dimension,
            tolerance,
            enforcement: BudgetEnforcement::Enforced,
        }
    }

    /// Creates an informational allocation.
    pub fn informational(
        dimension: BudgetDimension,
        tolerance: ErrorTolerance,
    ) -> Self {
        Self {
            dimension,
            tolerance,
            enforcement: BudgetEnforcement::Informational,
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub fn dimension(&self) -> &BudgetDimension {
        &self.dimension
    }

    /// Returns the tolerance.
    #[must_use]
    pub const fn tolerance(&self) -> ErrorTolerance {
        self.tolerance
    }

    /// Returns the enforcement mode.
    #[must_use]
    pub const fn enforcement(&self) -> BudgetEnforcement {
        self.enforcement
    }
}

/// A measured/predicted consumption of a budget dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetConsumption {
    dimension: BudgetDimension,
    consumed: ErrorQuantity,
}

impl BudgetConsumption {
    /// Creates a consumption record.
    pub fn new(
        dimension: BudgetDimension,
        consumed: ErrorQuantity,
    ) -> Self {
        Self {
            dimension,
            consumed,
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub fn dimension(&self) -> &BudgetDimension {
        &self.dimension
    }

    /// Returns the consumed amount.
    #[must_use]
    pub const fn consumed(&self) -> ErrorQuantity {
        self.consumed
    }
}

/// Result of evaluating one budget dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetStatus {
    /// Consumption is strictly below the tolerance.
    WithinBudget,

    /// Consumption exactly equals the tolerance.
    AtLimit,

    /// Consumption exceeds the tolerance.
    Exceeded,

    /// Dimension is informational and therefore never causes compliance
    /// failure.
    Informational,
}

impl BudgetStatus {
    /// Returns true when this status satisfies an enforced budget.
    #[must_use]
    pub const fn is_compliant(self) -> bool {
        match self {
            Self::WithinBudget
            | Self::AtLimit
            | Self::Informational => true,

            Self::Exceeded => false,
        }
    }
}

impl fmt::Display for BudgetStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::WithinBudget => "within_budget",
            Self::AtLimit => "at_limit",
            Self::Exceeded => "exceeded",
            Self::Informational => "informational",
        };

        formatter.write_str(value)
    }
}

/// Evaluation of one budget dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetEvaluation {
    dimension: BudgetDimension,
    tolerance: ErrorTolerance,
    consumed: ErrorQuantity,
    remaining: ErrorQuantity,
    status: BudgetStatus,
}

impl BudgetEvaluation {
    /// Returns the evaluated dimension.
    #[must_use]
    pub fn dimension(&self) -> &BudgetDimension {
        &self.dimension
    }

    /// Returns the allowed tolerance.
    #[must_use]
    pub const fn tolerance(&self) -> ErrorTolerance {
        self.tolerance
    }

    /// Returns consumed error.
    #[must_use]
    pub const fn consumed(&self) -> ErrorQuantity {
        self.consumed
    }

    /// Returns remaining budget.
    ///
    /// When a budget is exceeded, this returns zero rather than a negative
    /// quantity. The violation itself remains available through `status`.
    #[must_use]
    pub const fn remaining(&self) -> ErrorQuantity {
        self.remaining
    }

    /// Returns the evaluation status.
    #[must_use]
    pub const fn status(&self) -> BudgetStatus {
        self.status
    }

    /// Returns whether the dimension is compliant.
    #[must_use]
    pub const fn is_compliant(&self) -> bool {
        self.status.is_compliant()
    }
}

/// Complete result of evaluating an [`ErrorBudget`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBudgetEvaluation {
    budget: ErrorBudgetIdValue,
    dimensions: BTreeMap<BudgetDimension, BudgetEvaluation>,
    compliant: bool,
}

impl ErrorBudgetEvaluation {
    /// Returns the stable budget value.
    #[must_use]
    pub const fn budget(&self) -> ErrorBudgetIdValue {
        self.budget
    }

    /// Returns all dimension evaluations in deterministic order.
    #[must_use]
    pub fn dimensions(&self) -> &BTreeMap<BudgetDimension, BudgetEvaluation> {
        &self.dimensions
    }

    /// Returns whether all enforced dimensions are compliant.
    #[must_use]
    pub const fn is_compliant(&self) -> bool {
        self.compliant
    }

    /// Returns the number of evaluated dimensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns whether there are no dimensions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dimensions.is_empty()
    }
}

/// Portable value representation for a budget identity.
///
/// This deliberately avoids depending on the implementation details of
/// `core::ids::ErrorBudgetId`.
///
/// Integration layers can convert this value to the canonical ZQN ID.
pub type ErrorBudgetIdValue = u64;

/// Identity metadata for an error budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ErrorBudgetIdentity {
    value: ErrorBudgetIdValue,
}

impl ErrorBudgetIdentity {
    /// Creates an explicit identity value.
    ///
    /// This does not allocate or establish global uniqueness.
    #[must_use]
    pub const fn new(value: ErrorBudgetIdValue) -> Self {
        Self { value }
    }

    /// Returns the value.
    #[must_use]
    pub const fn value(self) -> ErrorBudgetIdValue {
        self.value
    }
}

impl Default for ErrorBudgetIdentity {
    fn default() -> Self {
        Self::new(0)
    }
}

/// An immutable semantic description of an error budget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBudget {
    schema_version: u32,
    identity: ErrorBudgetIdentity,
    aggregation: Aggregation,
    allocations: BTreeMap<BudgetDimension, BudgetAllocation>,
}

impl ErrorBudget {
    /// Creates an empty error budget with explicit identity.
    ///
    /// An empty budget is valid and evaluates as compliant. This is useful for
    /// incrementally constructing policy objects.
    #[must_use]
    pub fn new(identity: ErrorBudgetIdentity) -> Self {
        Self {
            schema_version: ERROR_BUDGET_SCHEMA_VERSION,
            identity,
            aggregation: Aggregation::Sum,
            allocations: BTreeMap::new(),
        }
    }

    /// Creates an error budget with a selected aggregation rule.
    #[must_use]
    pub fn with_aggregation(
        identity: ErrorBudgetIdentity,
        aggregation: Aggregation,
    ) -> Self {
        Self {
            schema_version: ERROR_BUDGET_SCHEMA_VERSION,
            identity,
            aggregation,
            allocations: BTreeMap::new(),
        }
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the budget identity.
    #[must_use]
    pub const fn identity(&self) -> ErrorBudgetIdentity {
        self.identity
    }

    /// Returns the selected aggregation rule.
    #[must_use]
    pub const fn aggregation(&self) -> Aggregation {
        self.aggregation
    }

    /// Returns all allocations in deterministic order.
    #[must_use]
    pub fn allocations(&self) -> &BTreeMap<BudgetDimension, BudgetAllocation> {
        &self.allocations
    }

    /// Returns the number of dimensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.allocations.len()
    }

    /// Returns whether the budget contains no dimensions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.allocations.is_empty()
    }

    /// Changes the aggregation rule.
    ///
    /// This does not change existing allocations.
    pub fn set_aggregation(&mut self, aggregation: Aggregation) {
        self.aggregation = aggregation;
    }

    /// Adds an allocation.
    ///
    /// Duplicate dimensions are rejected instead of silently replacing an
    /// existing policy.
    pub fn add_allocation(
        &mut self,
        allocation: BudgetAllocation,
    ) -> ErrorBudgetResult<()> {
        let dimension = allocation.dimension.clone();

        if self.allocations.contains_key(&dimension) {
            return Err(ErrorBudgetError::DuplicateDimension { dimension });
        }

        self.allocations.insert(dimension, allocation);
        Ok(())
    }

    /// Adds an enforced allocation directly.
    pub fn allocate(
        &mut self,
        dimension: BudgetDimension,
        tolerance: ErrorTolerance,
    ) -> ErrorBudgetResult<()> {
        self.add_allocation(BudgetAllocation::enforced(
            dimension,
            tolerance,
        ))
    }

    /// Adds an informational dimension directly.
    pub fn observe(
        &mut self,
        dimension: BudgetDimension,
        tolerance: ErrorTolerance,
    ) -> ErrorBudgetResult<()> {
        self.add_allocation(BudgetAllocation::informational(
            dimension,
            tolerance,
        ))
    }

    /// Looks up an allocation.
    #[must_use]
    pub fn allocation(
        &self,
        dimension: &BudgetDimension,
    ) -> Option<&BudgetAllocation> {
        self.allocations.get(dimension)
    }

    /// Validates all local invariants.
    pub fn validate(&self) -> ErrorBudgetResult<()> {
        if self.schema_version != ERROR_BUDGET_SCHEMA_VERSION {
            return Err(ErrorBudgetError::UnsupportedSchemaVersion {
                expected: ERROR_BUDGET_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        for (key, allocation) in &self.allocations {
            if key != allocation.dimension() {
                return Err(ErrorBudgetError::InconsistentDimensionKey);
            }

            validate_finite_non_negative(
                "allocation tolerance",
                allocation.tolerance().value(),
            )?;
        }

        Ok(())
    }

    /// Evaluates consumption values against the budget.
    ///
    /// Unknown dimensions are rejected rather than silently ignored. This is
    /// important because silently ignoring an unexpected metric can turn an
    /// apparently compliant computation into a false compliance result.
    pub fn evaluate<I>(
        &self,
        consumptions: I,
    ) -> ErrorBudgetResult<ErrorBudgetEvaluation>
    where
        I: IntoIterator<Item = BudgetConsumption>,
    {
        self.validate()?;

        let mut aggregated = BTreeMap::<BudgetDimension, ErrorQuantity>::new();

        for consumption in consumptions {
            if !self.allocations.contains_key(consumption.dimension()) {
                return Err(ErrorBudgetError::UnknownDimension {
                    dimension: consumption.dimension.clone(),
                });
            }

            let current = aggregated
                .get(consumption.dimension())
                .copied()
                .unwrap_or_default();

            let next = current.checked_add(consumption.consumed)?;
            aggregated.insert(consumption.dimension.clone(), next);
        }

        let mut dimensions = BTreeMap::new();
        let mut compliant = true;

        for (dimension, allocation) in &self.allocations {
            let consumed = aggregated
                .get(dimension)
                .copied()
                .unwrap_or_default();

            let tolerance = allocation.tolerance();

            let status = match allocation.enforcement() {
                BudgetEnforcement::Informational => {
                    BudgetStatus::Informational
                }

                BudgetEnforcement::Enforced if consumed.value() < tolerance.value() => {
                    BudgetStatus::WithinBudget
                }

                BudgetEnforcement::Enforced
                    if consumed.value() == tolerance.value() =>
                {
                    BudgetStatus::AtLimit
                }

                BudgetEnforcement::Enforced => BudgetStatus::Exceeded,
            };

            if !status.is_compliant() {
                compliant = false;
            }

            let remaining_value = if consumed.value() >= tolerance.value() {
                0.0
            } else {
                tolerance.value() - consumed.value()
            };

            let remaining = ErrorQuantity::new(remaining_value)?;

            dimensions.insert(
                dimension.clone(),
                BudgetEvaluation {
                    dimension: dimension.clone(),
                    tolerance,
                    consumed,
                    remaining,
                    status,
                },
            );
        }

        Ok(ErrorBudgetEvaluation {
            budget: self.identity.value(),
            dimensions,
            compliant,
        })
    }

    /// Evaluates a single dimension.
    pub fn evaluate_dimension(
        &self,
        dimension: &BudgetDimension,
        consumed: ErrorQuantity,
    ) -> ErrorBudgetResult<BudgetEvaluation> {
        let allocation = self
            .allocation(dimension)
            .ok_or_else(|| ErrorBudgetError::UnknownDimension {
                dimension: dimension.clone(),
            })?;

        let status = match allocation.enforcement() {
            BudgetEnforcement::Informational => BudgetStatus::Informational,

            BudgetEnforcement::Enforced
                if consumed.value() < allocation.tolerance().value() =>
            {
                BudgetStatus::WithinBudget
            }

            BudgetEnforcement::Enforced
                if consumed.value() == allocation.tolerance().value() =>
            {
                BudgetStatus::AtLimit
            }

            BudgetEnforcement::Enforced => BudgetStatus::Exceeded,
        };

        let remaining_value = if consumed.value() >= allocation.tolerance().value() {
            0.0
        } else {
            allocation.tolerance().value() - consumed.value()
        };

        Ok(BudgetEvaluation {
            dimension: dimension.clone(),
            tolerance: allocation.tolerance(),
            consumed,
            remaining: ErrorQuantity::new(remaining_value)?,
            status,
        })
    }

    /// Returns the total tolerance across all dimensions.
    ///
    /// This operation is only meaningful when dimensions share a common error
    /// metric. It is therefore explicit rather than being used implicitly by
    /// `evaluate`.
    pub fn total_tolerance(&self) -> ErrorBudgetResult<ErrorTolerance> {
        let mut total = 0.0;

        for allocation in self.allocations.values() {
            total += allocation.tolerance().value();

            if !total.is_finite() {
                return Err(ErrorBudgetError::NumericalOverflow {
                    operation: "total tolerance",
                });
            }
        }

        ErrorTolerance::new(total)
    }

    /// Composes this budget with another budget using an explicit policy.
    ///
    /// Dimensions must not collide because silently combining two independently
    /// defined tolerances is ambiguous.
    pub fn compose(
        &self,
        other: &Self,
        policy: BudgetComposition,
    ) -> ErrorBudgetResult<Self> {
        self.validate()?;
        other.validate()?;

        let mut result = Self::with_aggregation(
            self.identity,
            self.aggregation,
        );

        for allocation in self.allocations.values() {
            result.add_allocation(allocation.clone())?;
        }

        for allocation in other.allocations.values() {
            if let Some(existing) = result.allocations.get_mut(allocation.dimension()) {
                let tolerance = compose_tolerance(
                    existing.tolerance(),
                    allocation.tolerance(),
                    policy,
                )?;

                *existing = BudgetAllocation {
                    dimension: existing.dimension.clone(),
                    tolerance,
                    enforcement: compose_enforcement(
                        existing.enforcement(),
                        allocation.enforcement(),
                    ),
                };
            } else {
                result.add_allocation(allocation.clone())?;
            }
        }

        Ok(result)
    }

    /// Creates a child allocation from a parent dimension.
    ///
    /// The child tolerance must not exceed the parent tolerance.
    pub fn child_allocation(
        &self,
        parent: &BudgetDimension,
        child: BudgetDimension,
        tolerance: ErrorTolerance,
    ) -> ErrorBudgetResult<BudgetAllocation> {
        let parent_allocation = self
            .allocation(parent)
            .ok_or_else(|| ErrorBudgetError::UnknownDimension {
                dimension: parent.clone(),
            })?;

        if tolerance.value() > parent_allocation.tolerance().value() {
            return Err(ErrorBudgetError::ChildBudgetExceedsParent {
                parent: parent.clone(),
                parent_tolerance: parent_allocation.tolerance(),
                child: child.clone(),
                child_tolerance: tolerance,
            });
        }

        Ok(BudgetAllocation {
            dimension: child,
            tolerance,
            enforcement: parent_allocation.enforcement(),
        })
    }
}

/// Defines how two budget tolerances are composed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BudgetComposition {
    /// Add the tolerances.
    Add,

    /// Take the stricter/smaller tolerance.
    Minimum,

    /// Take the looser/larger tolerance.
    Maximum,

    /// Require exact equality.
    RequireEqual,
}

fn compose_tolerance(
    left: ErrorTolerance,
    right: ErrorTolerance,
    policy: BudgetComposition,
) -> ErrorBudgetResult<ErrorTolerance> {
    let value = match policy {
        BudgetComposition::Add => left.value() + right.value(),

        BudgetComposition::Minimum => {
            left.value().min(right.value())
        }

        BudgetComposition::Maximum => {
            left.value().max(right.value())
        }

        BudgetComposition::RequireEqual => {
            if left.value() != right.value() {
                return Err(ErrorBudgetError::IncompatibleTolerance {
                    left,
                    right,
                });
            }

            left.value()
        }
    };

    ErrorTolerance::new(value)
}

fn compose_enforcement(
    left: BudgetEnforcement,
    right: BudgetEnforcement,
) -> BudgetEnforcement {
    match (left, right) {
        (BudgetEnforcement::Enforced, _)
        | (_, BudgetEnforcement::Enforced) => BudgetEnforcement::Enforced,

        (
            BudgetEnforcement::Informational,
            BudgetEnforcement::Informational,
        ) => BudgetEnforcement::Informational,
    }
}

/// Aggregates a finite iterator of error quantities.
///
/// This is the reusable numerical primitive intended for
/// `propagation::accumulation`.
pub fn aggregate<I>(
    values: I,
    aggregation: Aggregation,
) -> ErrorBudgetResult<ErrorQuantity>
where
    I: IntoIterator<Item = ErrorQuantity>,
{
    match aggregation {
        Aggregation::Sum => aggregate_sum(values),

        Aggregation::Maximum => aggregate_maximum(values),

        Aggregation::RootSumSquare => aggregate_root_sum_square(values),

        Aggregation::Custom => Err(ErrorBudgetError::CustomAggregationRequired),
    }
}

fn aggregate_sum<I>(
    values: I,
) -> ErrorBudgetResult<ErrorQuantity>
where
    I: IntoIterator<Item = ErrorQuantity>,
{
    let mut total = ErrorQuantity::zero();

    for value in values {
        total = total.checked_add(value)?;
    }

    Ok(total)
}

fn aggregate_maximum<I>(
    values: I,
) -> ErrorBudgetResult<ErrorQuantity>
where
    I: IntoIterator<Item = ErrorQuantity>,
{
    let mut maximum = ErrorQuantity::zero();

    for value in values {
        if value.value() > maximum.value() {
            maximum = value;
        }
    }

    Ok(maximum)
}

fn aggregate_root_sum_square<I>(
    values: I,
) -> ErrorBudgetResult<ErrorQuantity>
where
    I: IntoIterator<Item = ErrorQuantity>,
{
    let mut sum = 0.0;

    for value in values {
        let squared = value.value() * value.value();

        if !squared.is_finite() {
            return Err(ErrorBudgetError::NumericalOverflow {
                operation: "root-sum-square squaring",
            });
        }

        sum += squared;

        if !sum.is_finite() {
            return Err(ErrorBudgetError::NumericalOverflow {
                operation: "root-sum-square accumulation",
            });
        }
    }

    let result = sum.sqrt();

    ErrorQuantity::new(result)
}

fn validate_finite_non_negative(
    field: &'static str,
    value: f64,
) -> ErrorBudgetResult<()> {
    if !value.is_finite() {
        return Err(ErrorBudgetError::NonFiniteValue { field });
    }

    if value < 0.0 {
        return Err(ErrorBudgetError::NegativeValue { field });
    }

    Ok(())
}

/// Errors specific to error-budget construction and evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorBudgetError {
    /// A required dimension name is empty.
    EmptyDimension,

    /// A dimension contains invalid syntax/content.
    InvalidDimension {
        /// Why the dimension was rejected.
        reason: &'static str,
    },

    /// A dimension occurs more than once.
    DuplicateDimension {
        /// Duplicated dimension.
        dimension: BudgetDimension,
    },

    /// A consumption references a dimension not present in the budget.
    UnknownDimension {
        /// Unknown dimension.
        dimension: BudgetDimension,
    },

    /// A floating-point input is NaN or infinite.
    NonFiniteValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A non-negative quantity was negative.
    NegativeValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// Floating-point arithmetic produced a non-finite value.
    NumericalOverflow {
        /// Arithmetic operation that overflowed.
        operation: &'static str,
    },

    /// The budget's serialized schema is unsupported.
    UnsupportedSchemaVersion {
        /// Current supported schema.
        expected: u32,

        /// Supplied schema.
        actual: u32,
    },

    /// A map key and the contained allocation disagree.
    InconsistentDimensionKey,

    /// A child allocation exceeds its parent.
    ChildBudgetExceedsParent {
        /// Parent dimension.
        parent: BudgetDimension,

        /// Parent tolerance.
        parent_tolerance: ErrorTolerance,

        /// Child dimension.
        child: BudgetDimension,

        /// Child tolerance.
        child_tolerance: ErrorTolerance,
    },

    /// Two composition inputs require incompatible tolerances.
    IncompatibleTolerance {
        /// Left tolerance.
        left: ErrorTolerance,

        /// Right tolerance.
        right: ErrorTolerance,
    },

    /// A custom aggregation requires a higher-level implementation.
    CustomAggregationRequired,
}

impl fmt::Display for ErrorBudgetError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyDimension => {
                formatter.write_str("error-budget dimension cannot be empty")
            }

            Self::InvalidDimension { reason } => {
                write!(
                    formatter,
                    "invalid error-budget dimension: {reason}"
                )
            }

            Self::DuplicateDimension { dimension } => {
                write!(
                    formatter,
                    "duplicate error-budget dimension `{dimension}`"
                )
            }

            Self::UnknownDimension { dimension } => {
                write!(
                    formatter,
                    "error-budget consumption references unknown \
                     dimension `{dimension}`"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "`{field}` must be finite"
                )
            }

            Self::NegativeValue { field } => {
                write!(
                    formatter,
                    "`{field}` must be non-negative"
                )
            }

            Self::NumericalOverflow { operation } => {
                write!(
                    formatter,
                    "numerical overflow during {operation}"
                )
            }

            Self::UnsupportedSchemaVersion { expected, actual } => {
                write!(
                    formatter,
                    "unsupported error-budget schema version {actual}; \
                     expected {expected}"
                )
            }

            Self::InconsistentDimensionKey => {
                formatter.write_str(
                    "error-budget dimension map key is inconsistent \
                     with its allocation",
                )
            }

            Self::ChildBudgetExceedsParent {
                parent,
                parent_tolerance,
                child,
                child_tolerance,
            } => {
                write!(
                    formatter,
                    "child budget `{child}` ({child_tolerance}) exceeds \
                     parent budget `{parent}` ({parent_tolerance})"
                )
            }

            Self::IncompatibleTolerance { left, right } => {
                write!(
                    formatter,
                    "incompatible error-budget tolerances: \
                     {left} and {right}"
                )
            }

            Self::CustomAggregationRequired => {
                formatter.write_str(
                    "custom aggregation requires a higher-level \
                     propagation implementation",
                )
            }
        }
    }
}

impl std::error::Error for ErrorBudgetError {}

/// Converts a budget status into a simple boolean compliance result.
#[must_use]
pub const fn is_within_budget(
    tolerance: ErrorTolerance,
    consumed: ErrorQuantity,
) -> bool {
    consumed.value() <= tolerance.value()
}

/// Calculates remaining budget without producing a negative quantity.
///
/// The caller is expected to have validated both inputs through their
/// constructors.
pub fn remaining_budget(
    tolerance: ErrorTolerance,
    consumed: ErrorQuantity,
) -> ErrorBudgetResult<ErrorQuantity> {
    let value = if consumed.value() >= tolerance.value() {
        0.0
    } else {
        tolerance.value() - consumed.value()
    };

    ErrorQuantity::new(value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dimension(name: &str) -> BudgetDimension {
        BudgetDimension::new(name).expect("valid dimension")
    }

    fn quantity(value: f64) -> ErrorQuantity {
        ErrorQuantity::new(value).expect("valid quantity")
    }

    fn tolerance(value: f64) -> ErrorTolerance {
        ErrorTolerance::new(value).expect("valid tolerance")
    }

    #[test]
    fn zero_values_are_valid() {
        assert_eq!(ErrorQuantity::zero().value(), 0.0);
        assert_eq!(ErrorTolerance::zero().value(), 0.0);
    }

    #[test]
    fn negative_values_are_rejected() {
        assert!(ErrorQuantity::new(-1.0).is_err());
        assert!(ErrorTolerance::new(-1.0).is_err());
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert!(ErrorQuantity::new(f64::NAN).is_err());
        assert!(ErrorQuantity::new(f64::INFINITY).is_err());
        assert!(ErrorQuantity::new(f64::NEG_INFINITY).is_err());

        assert!(ErrorTolerance::new(f64::NAN).is_err());
        assert!(ErrorTolerance::new(f64::INFINITY).is_err());
    }

    #[test]
    fn dimensions_are_validated() {
        assert!(BudgetDimension::new("").is_err());
        assert!(BudgetDimension::new(" gates").is_err());
        assert!(BudgetDimension::new("gates ").is_err());
        assert!(BudgetDimension::new("gate\nerror").is_err());
        assert!(BudgetDimension::new("gates").is_ok());
    }

    #[test]
    fn duplicate_dimensions_are_rejected() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(0.1))
            .expect("first allocation");

        assert!(
            budget
                .allocate(dimension("gate"), tolerance(0.2))
                .is_err()
        );
    }

    #[test]
    fn budget_is_compliant_below_limit() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(1.0))
            .expect("allocation");

        let evaluation = budget
            .evaluate([BudgetConsumption::new(
                dimension("gate"),
                quantity(0.25),
            )])
            .expect("evaluation");

        assert!(evaluation.is_compliant());

        let result = evaluation
            .dimensions()
            .get(&dimension("gate"))
            .expect("dimension");

        assert_eq!(result.status(), BudgetStatus::WithinBudget);
        assert_eq!(result.remaining().value(), 0.75);
    }

    #[test]
    fn exact_limit_is_compliant() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(1.0))
            .expect("allocation");

        let evaluation = budget
            .evaluate([BudgetConsumption::new(
                dimension("gate"),
                quantity(1.0),
            )])
            .expect("evaluation");

        assert!(evaluation.is_compliant());

        assert_eq!(
            evaluation
                .dimensions()
                .get(&dimension("gate"))
                .expect("dimension")
                .status(),
            BudgetStatus::AtLimit
        );
    }

    #[test]
    fn exceeding_limit_is_non_compliant() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(1.0))
            .expect("allocation");

        let evaluation = budget
            .evaluate([BudgetConsumption::new(
                dimension("gate"),
                quantity(1.1),
            )])
            .expect("evaluation");

        assert!(!evaluation.is_compliant());

        let result = evaluation
            .dimensions()
            .get(&dimension("gate"))
            .expect("dimension");

        assert_eq!(result.status(), BudgetStatus::Exceeded);
        assert_eq!(result.remaining().value(), 0.0);
    }

    #[test]
    fn informational_dimension_never_breaks_compliance() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .observe(dimension("diagnostic"), tolerance(1.0))
            .expect("allocation");

        let evaluation = budget
            .evaluate([BudgetConsumption::new(
                dimension("diagnostic"),
                quantity(100.0),
            )])
            .expect("evaluation");

        assert!(evaluation.is_compliant());

        assert_eq!(
            evaluation
                .dimensions()
                .get(&dimension("diagnostic"))
                .expect("dimension")
                .status(),
            BudgetStatus::Informational
        );
    }

    #[test]
    fn unknown_dimensions_are_rejected() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(1.0))
            .expect("allocation");

        let result = budget.evaluate([BudgetConsumption::new(
            dimension("measurement"),
            quantity(0.1),
        )]);

        assert!(matches!(
            result,
            Err(ErrorBudgetError::UnknownDimension { .. })
        ));
    }

    #[test]
    fn repeated_consumptions_are_aggregated() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("gate"), tolerance(1.0))
            .expect("allocation");

        let evaluation = budget
            .evaluate([
                BudgetConsumption::new(
                    dimension("gate"),
                    quantity(0.4),
                ),
                BudgetConsumption::new(
                    dimension("gate"),
                    quantity(0.3),
                ),
            ])
            .expect("evaluation");

        let result = evaluation
            .dimensions()
            .get(&dimension("gate"))
            .expect("dimension");

        assert_eq!(result.consumed().value(), 0.7);
        assert_eq!(result.remaining().value(), 0.3);
    }

    #[test]
    fn sum_aggregation_is_correct() {
        let result = aggregate(
            [
                quantity(0.2),
                quantity(0.3),
                quantity(0.5),
            ],
            Aggregation::Sum,
        )
        .expect("aggregation");

        assert!((result.value() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn maximum_aggregation_is_correct() {
        let result = aggregate(
            [
                quantity(0.2),
                quantity(0.8),
                quantity(0.5),
            ],
            Aggregation::Maximum,
        )
        .expect("aggregation");

        assert_eq!(result.value(), 0.8);
    }

    #[test]
    fn root_sum_square_is_correct() {
        let result = aggregate(
            [
                quantity(3.0),
                quantity(4.0),
            ],
            Aggregation::RootSumSquare,
        )
        .expect("aggregation");

        assert!((result.value() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn custom_aggregation_is_explicitly_rejected() {
        let result =
            aggregate([quantity(1.0)], Aggregation::Custom);

        assert!(matches!(
            result,
            Err(ErrorBudgetError::CustomAggregationRequired)
        ));
    }

    #[test]
    fn remaining_budget_never_goes_negative() {
        let remaining =
            remaining_budget(tolerance(1.0), quantity(2.0))
                .expect("remaining");

        assert_eq!(remaining.value(), 0.0);
    }

    #[test]
    fn composition_adds_tolerances_when_requested() {
        let mut left =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        left.allocate(dimension("gate"), tolerance(0.2))
            .expect("left");

        let mut right =
            ErrorBudget::new(ErrorBudgetIdentity::new(2));

        right
            .allocate(dimension("gate"), tolerance(0.3))
            .expect("right");

        let composed = left
            .compose(&right, BudgetComposition::Add)
            .expect("composition");

        assert_eq!(
            composed
                .allocation(&dimension("gate"))
                .expect("allocation")
                .tolerance()
                .value(),
            0.5
        );
    }

    #[test]
    fn composition_can_select_stricter_tolerance() {
        let mut left =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        left.allocate(dimension("gate"), tolerance(0.2))
            .expect("left");

        let mut right =
            ErrorBudget::new(ErrorBudgetIdentity::new(2));

        right
            .allocate(dimension("gate"), tolerance(0.3))
            .expect("right");

        let composed = left
            .compose(&right, BudgetComposition::Minimum)
            .expect("composition");

        assert_eq!(
            composed
                .allocation(&dimension("gate"))
                .expect("allocation")
                .tolerance()
                .value(),
            0.2
        );
    }

    #[test]
    fn equal_tolerance_policy_rejects_conflicts() {
        let mut left =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        left.allocate(dimension("gate"), tolerance(0.2))
            .expect("left");

        let mut right =
            ErrorBudget::new(ErrorBudgetIdentity::new(2));

        right
            .allocate(dimension("gate"), tolerance(0.3))
            .expect("right");

        let result =
            left.compose(&right, BudgetComposition::RequireEqual);

        assert!(matches!(
            result,
            Err(ErrorBudgetError::IncompatibleTolerance { .. })
        ));
    }

    #[test]
    fn child_budget_cannot_exceed_parent() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        budget
            .allocate(dimension("total"), tolerance(1.0))
            .expect("parent");

        assert!(
            budget
                .child_allocation(
                    &dimension("total"),
                    dimension("gate"),
                    tolerance(0.5),
                )
                .is_ok()
        );

        assert!(
            budget
                .child_allocation(
                    &dimension("total"),
                    dimension("gate"),
                    tolerance(1.1),
                )
                .is_err()
        );
    }

    #[test]
    fn empty_budget_is_compliant() {
        let budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(1));

        let evaluation =
            budget.evaluate(std::iter::empty())
                .expect("evaluation");

        assert!(evaluation.is_compliant());
        assert!(evaluation.is_empty());
    }

    #[test]
    fn validation_is_deterministic() {
        let mut first =
            ErrorBudget::new(ErrorBudgetIdentity::new(7));

        first
            .allocate(dimension("z"), tolerance(0.3))
            .expect("allocation");

        first
            .allocate(dimension("a"), tolerance(0.1))
            .expect("allocation");

        let mut second =
            ErrorBudget::new(ErrorBudgetIdentity::new(7));

        second
            .allocate(dimension("a"), tolerance(0.1))
            .expect("allocation");

        second
            .allocate(dimension("z"), tolerance(0.3))
            .expect("allocation");

        assert_eq!(first, second);
        assert!(first.validate().is_ok());
    }

    #[test]
    fn aggregation_is_order_independent_for_normal_values() {
        let first = aggregate(
            [
                quantity(0.1),
                quantity(0.2),
                quantity(0.3),
            ],
            Aggregation::Sum,
        )
        .expect("first");

        let second = aggregate(
            [
                quantity(0.3),
                quantity(0.1),
                quantity(0.2),
            ],
            Aggregation::Sum,
        )
        .expect("second");

        assert_eq!(first.value(), second.value());
    }

    #[test]
    fn identity_does_not_imply_existence() {
        let identity = ErrorBudgetIdentity::new(u64::MAX);

        assert_eq!(identity.value(), u64::MAX);
    }

    #[test]
    fn no_architectural_machine_size_limit_exists() {
        let identity = ErrorBudgetIdentity::new(u64::MAX);

        let mut budget = ErrorBudget::new(identity);

        budget
            .allocate(
                dimension("arbitrary_metric"),
                tolerance(1.0),
            )
            .expect("allocation");

        assert_eq!(budget.identity().value(), u64::MAX);
    }

    #[test]
    fn serde_round_trip_preserves_semantics() {
        let mut budget =
            ErrorBudget::new(ErrorBudgetIdentity::new(42));

        budget
            .allocate(dimension("gate"), tolerance(0.01))
            .expect("allocation");

        let encoded =
            serde_json::to_string(&budget)
                .expect("serialize");

        let decoded: ErrorBudget =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(budget, decoded);
    }
}