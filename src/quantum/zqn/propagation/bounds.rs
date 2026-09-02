//! Zamani Quantum Noise (ZQN) — Conservative Error Bounds
//!
//! Path:
//!     src/quantum/zqn/propagation/bounds.rs
//!
//! # Purpose
//!
//! This module owns deterministic, conservative bounds for quantities produced
//! or consumed by the ZQN propagation subsystem.
//!
//! The central distinction is:
//!
//! ```text
//! uncertainty.rs
//!     probabilistic/statistical or standard uncertainty propagation
//!
//! bounds.rs
//!     deterministic conservative bounds
//!
//! error_budget.rs
//!     allowed error and budget compliance
//!
//! fidelity.rs
//!     fidelity/error metrics
//! ```
//!
//! A bound is not a confidence interval.
//!
//! A bound does not assert a probability distribution.
//!
//! A bound does not infer statistical confidence.
//!
//! A conservative bound states that, under the explicitly declared model and
//! assumptions, the true quantity is guaranteed to lie within the represented
//! domain.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - finite non-negative error bounds;
//! - explicit unbounded error bounds;
//! - deterministic bound intervals;
//! - conservative interval arithmetic;
//! - scalar affine bound propagation;
//! - scalar error-factor propagation;
//! - additive bound accumulation;
//! - maximum bound accumulation;
//! - root-sum-square bound accumulation;
//! - budget comparison helpers;
//! - bound classification;
//! - numerical validation;
//! - explicit resource-policy checks;
//! - deterministic behavior;
//! - machine-size-independent semantics.
//!
//! This file does NOT own:
//!
//! - quantum channels;
//! - quantum states;
//! - fidelity definitions;
//! - probability distributions;
//! - statistical estimation;
//! - uncertainty distributions;
//! - calibration storage;
//! - characterization experiments;
//! - noise models;
//! - fault generation;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - runtime orchestration;
//! - benchmark methodology;
//! - canonical Quantum IR;
//! - canonical qubit identity;
//! - serialization wire formats;
//! - automatic differentiation;
//! - symbolic algebra;
//! - vendor numerical libraries.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! physical/noise semantics
//!     │
//!     ├───────────────┬────────────────┐
//!     ▼               ▼                ▼
//! uncertainty      fidelity        characterization
//!     │               │                │
//!     └───────────────┼────────────────┘
//!                     ▼
//!          propagation::bounds
//!                     │
//!          ┌──────────┼───────────┐
//!          ▼          ▼           ▼
//!      error_budget  QEC       routing/scheduling
//!                     │
//!                     ▼
//!                   target
//! ```
//!
//! # Canonical quantum identity
//!
//! Bound arithmetic is representation-independent and therefore does not need
//! to import `quantum::ir::qubit`.
//!
//! When a higher-level object associates a bound with a quantum resource, that
//! higher-level integration layer MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file intentionally does not define another `QubitId` or
//! `PhysicalQubitId`.
//!
//! This is consistent with the repository's canonical quantum identity rule.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_BOUND_ENTRIES
//! MAX_CIRCUIT_DEPTH
//! MAX_ERROR
//! ```
//!
//! Bounds operate on dimensions and collections supplied by the caller.
//!
//! Therefore the same code can process:
//!
//! ```text
//! one resource
//! thousands of resources
//! millions of resources
//! distributed resources
//! logical resources
//! physical resources
//! future resource types
//! ```
//!
//! subject only to the resources and explicit execution policy available to
//! the caller.
//!
//! "Infinity" at the architectural level means no artificial machine-size
//! ceiling. It does NOT mean that an implementation can materialize infinite
//! memory or perform infinite computation.
//!
//! # Explicit mathematical unboundedness
//!
//! A mathematically unbounded result MUST NOT be represented as:
//!
//! ```text
//! f64::INFINITY
//! ```
//!
//! because this module rejects non-finite floating-point values as invalid
//! numerical input.
//!
//! Instead this module represents unboundedness explicitly:
//!
//! ```text
//! BoundValue::Unbounded
//! ```
//!
//! This distinction is essential:
//!
//! ```text
//! NaN
//!     invalid numerical input
//!
//! +∞ encoded in f64
//!     invalid numerical input
//!
//! BoundValue::Unbounded
//!     valid mathematical result
//! ```
//!
//! # Conservative semantics
//!
//! A finite error bound is represented as:
//!
//! ```text
//! 0 <= lower <= upper
//! ```
//!
//! An unbounded upper result is represented as:
//!
//! ```text
//! 0 <= lower <= Unbounded
//! ```
//!
//! The lower bound is always finite because an error magnitude is
//! non-negative and the useful conservative lower bound is representable.
//!
//! # Important rule
//!
//! This module never silently narrows a bound.
//!
//! It may widen a result when required by conservative arithmetic.
//!
//! It never silently converts:
//!
//! ```text
//! invalid -> zero
//! invalid -> finite maximum
//! unbounded -> arbitrary finite value
//! negative -> absolute value
//! NaN -> zero
//! ```
//!
//! # Interval semantics
//!
//! For deterministic signed quantities, this module also provides
//! [`ValueInterval`].
//!
//! A value interval may contain negative values:
//!
//! ```text
//! [lower, upper]
//! ```
//!
//! with:
//!
//! ```text
//! lower <= upper
//! ```
//!
//! Its endpoints must be finite.
//!
//! If an operation would mathematically become unbounded, the operation
//! returns [`ValueIntervalError::UnboundedResult`] rather than fabricating a
//! finite endpoint.
//!
//! # Error-bound semantics
//!
//! Error bounds are non-negative magnitudes.
//!
//! For two finite bounds:
//!
//! ```text
//! a = [a_l, a_u]
//! b = [b_l, b_u]
//! ```
//!
//! Addition:
//!
//! ```text
//! [a_l + b_l, a_u + b_u]
//! ```
//!
//! Conservative worst-case accumulation:
//!
//! ```text
//! upper = a_u + b_u
//! ```
//!
//! Maximum:
//!
//! ```text
//! upper = max(a_u, b_u)
//! ```
//!
//! Root-sum-square:
//!
//! ```text
//! upper = sqrt(a_u² + b_u²)
//! ```
//!
//! Root-sum-square is only conservative when its statistical/physical
//! assumptions are valid. This module therefore never silently changes an
//! explicitly selected aggregation policy.
//!
//! # Relationship with error_budget.rs
//!
//! `error_budget.rs` owns:
//!
//! ```text
//! allowed tolerance
//! consumed error
//! compliance
//! allocation
//! ```
//!
//! This file owns:
//!
//! ```text
//! conservative predicted/derived bound
//! ```
//!
//! Integration therefore looks like:
//!
//! ```text
//! ConservativeErrorBound
//!         │
//!         ▼
//! ErrorQuantity
//!         │
//!         ▼
//! ErrorBudget::evaluate(...)
//! ```
//!
//! The conversion is exposed through [`ConservativeErrorBound::as_error_quantity`]
//! for finite bounds.
//!
//! An unbounded bound cannot be converted to a finite `ErrorQuantity` and must
//! be handled explicitly by the caller.
//!
//! # Relationship with uncertainty.rs
//!
//! `uncertainty.rs` already distinguishes standard uncertainty from
//! deterministic intervals.
//!
//! A deterministic interval can be transformed into a conservative error
//! magnitude by explicitly choosing a reference value.
//!
//! This file does not reinterpret a standard deviation as a deterministic
//! bound.
//!
//! In particular, this module never performs:
//!
//! ```text
//! bound = k * standard_deviation
//! ```
//!
//! without the caller explicitly supplying the factor and the physical/
//! statistical justification.
//!
//! # Determinism
//!
//! All operations are deterministic.
//!
//! There is:
//!
//! - no RNG;
//! - no global mutable state;
//! - no system-clock dependency;
//! - no process identity dependency;
//! - no thread identity dependency;
//! - no unordered semantic reduction;
//! - no hidden parallelism.
//!
//! Identical inputs and identical policy produce the same operation sequence.
//!
//! # Parallel execution
//!
//! The aggregation functions accept iterators.
//!
//! This permits callers to stream large collections rather than forcing a
//! materialized `Vec`.
//!
//! A caller requiring bit-for-bit deterministic floating-point results across
//! parallel execution must use a deterministic reduction order.
//!
//! This module does not claim that arbitrary floating-point reassociation is
//! bit-for-bit invariant.
//!
//! # Resource safety
//!
//! No artificial semantic limit is imposed.
//!
//! Explicit resource limits are provided through [`BoundLimits`].
//!
//! `None` means this module imposes no additional limit.
//!
//! Limits are operational/security policy, not machine-size semantics.
//!
//! # Numerical safety
//!
//! All finite floating-point inputs must satisfy:
//!
//! ```text
//! is_finite()
//! ```
//!
//! Error magnitudes must additionally satisfy:
//!
//! ```text
//! value >= 0
//! ```
//!
//! Arithmetic that produces a non-finite result is reported as numerical
//! overflow rather than silently becoming unbounded.
//!
//! Mathematical unboundedness is represented explicitly by
//! [`BoundValue::Unbounded`].
//!
//! # Security
//!
//! Bounds can be derived from untrusted calibration, model, or serialized
//! inputs.
//!
//! Therefore this file:
//!
//! - validates every public floating-point input;
//! - checks dimension arithmetic;
//! - checks configured resource policies;
//! - never invokes external processes;
//! - never performs I/O;
//! - never allocates based on an untrusted count unless the caller explicitly
//!   requests an allocation-producing operation;
//! - contains no `unsafe` code.
//!
//! # Serialization
//!
//! This file defines semantic values only.
//!
//! Repository-wide serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! In particular, [`BoundValue::Unbounded`] should be represented by an
//! explicit schema variant rather than serialized as JSON `Infinity` or
//! another non-standard floating-point value.
//!
//! # Versioning
//!
//! `BOUNDS_SCHEMA_ID` and `BOUNDS_SCHEMA_VERSION` identify the semantic
//! contract of this module.
//!
//! The external wire schema remains owned by `zqn::io`.
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

use super::error_budget::{
    ErrorBudget,
    ErrorBudgetError,
    ErrorQuantity,
    BudgetConsumption,
    BudgetDimension,
};

// =============================================================================
// Public schema
// =============================================================================

/// Stable semantic identifier for this module.
pub const BOUNDS_SCHEMA_ID: &str =
    "zamani.quantum.zqn.propagation.bounds";

/// Semantic version of the public bounds contract.
pub const BOUNDS_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance used by comparison helpers.
pub const DEFAULT_BOUND_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Errors returned by conservative-bound operations.
#[derive(Clone, Debug, PartialEq)]
pub enum BoundError {
    /// A required floating-point value was NaN or infinite.
    NonFinite {
        /// Semantic name of the invalid value.
        field: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// A non-negative quantity was negative.
    NegativeValue {
        /// Semantic name of the invalid value.
        field: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// An interval has lower > upper.
    InvalidInterval {
        /// Lower endpoint.
        lower: f64,

        /// Upper endpoint.
        upper: f64,
    },

    /// Arithmetic produced a non-finite result.
    NumericalOverflow {
        /// Operation that failed.
        operation: &'static str,
    },

    /// A dimension calculation overflowed.
    SizeOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// An explicitly configured resource limit was exceeded.
    ResourceLimitExceeded {
        /// Requested amount.
        requested: u128,

        /// Configured maximum.
        maximum: u128,

        /// Resource category.
        resource: &'static str,
    },

    /// A finite result was required but the mathematical result is unbounded.
    UnboundedResult {
        /// Operation producing the unbounded result.
        operation: &'static str,
    },

    /// A numerical tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// An aggregation requires a mathematical assumption not provided by the
    /// caller.
    InvalidAggregationAssumption {
        /// Aggregation name.
        aggregation: &'static str,
    },

    /// The requested operation is not representable by this scalar contract.
    UnsupportedOperation {
        /// Operation name.
        operation: &'static str,
    },

    /// A finite bound cannot be produced from an unbounded value.
    RequiresFiniteBound {
        /// Context requiring a finite bound.
        context: &'static str,
    },

    /// A bound could not be converted to an error-budget quantity because the
    /// requested semantics are incompatible.
    BudgetConversion {
        /// Context.
        context: &'static str,
    },
}

impl fmt::Display for BoundError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(
                    formatter,
                    "non-finite bound value in `{field}`: {value}"
                )
            }

            Self::NegativeValue { field, value } => {
                write!(
                    formatter,
                    "negative bound value in `{field}`: {value}"
                )
            }

            Self::InvalidInterval { lower, upper } => {
                write!(
                    formatter,
                    "invalid interval [{lower}, {upper}]: lower exceeds upper"
                )
            }

            Self::NumericalOverflow { operation } => {
                write!(
                    formatter,
                    "numerical overflow during {operation}"
                )
            }

            Self::SizeOverflow { operation } => {
                write!(
                    formatter,
                    "size arithmetic overflow during {operation}"
                )
            }

            Self::ResourceLimitExceeded {
                requested,
                maximum,
                resource,
            } => {
                write!(
                    formatter,
                    "resource limit exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::UnboundedResult { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` produces an unbounded result"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "invalid bound tolerance: {value}"
                )
            }

            Self::InvalidAggregationAssumption { aggregation } => {
                write!(
                    formatter,
                    "aggregation `{aggregation}` requires an \
                     explicit mathematical/physical assumption"
                )
            }

            Self::UnsupportedOperation { operation } => {
                write!(
                    formatter,
                    "unsupported bound operation: {operation}"
                )
            }

            Self::RequiresFiniteBound { context } => {
                write!(
                    formatter,
                    "finite bound required for {context}"
                )
            }

            Self::BudgetConversion { context } => {
                write!(
                    formatter,
                    "bound-to-budget conversion failed for {context}"
                )
            }
        }
    }
}

impl std::error::Error for BoundError {}

/// Result type used by this module.
pub type BoundResult<T> = Result<T, BoundError>;

// =============================================================================
// Explicit bounded/unbounded value
// =============================================================================

/// A non-negative upper/lower magnitude that may be mathematically unbounded.
///
/// `Unbounded` is intentionally distinct from `f64::INFINITY`.
///
/// This permits the module to reject invalid floating-point infinities while
/// still representing legitimate mathematical unboundedness.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BoundValue {
    /// A finite non-negative value.
    Finite(f64),

    /// No finite upper bound exists under the supplied assumptions.
    Unbounded,
}

impl BoundValue {
    /// Creates a finite bound value.
    pub fn finite(value: f64) -> BoundResult<Self> {
        validate_non_negative("bound", value)?;
        Ok(Self::Finite(value))
    }

    /// Creates an explicit unbounded value.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Returns whether this value is finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        matches!(self, Self::Finite(_))
    }

    /// Returns whether this value is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns the finite value, or an error when unbounded.
    pub fn finite_value(self) -> BoundResult<f64> {
        match self {
            Self::Finite(value) => Ok(value),
            Self::Unbounded => Err(BoundError::RequiresFiniteBound {
                context: "finite bound value",
            }),
        }
    }

    /// Returns zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self::Finite(0.0)
    }

    /// Returns the larger of two bounds.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => {
                Self::Unbounded
            }

            (Self::Finite(left), Self::Finite(right)) => {
                Self::Finite(left.max(right))
            }
        }
    }

    /// Returns the smaller of two bounds.
    ///
    /// This operation is mathematically valid, but callers must ensure that
    /// taking a minimum is semantically justified for their bound model.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unbounded, finite)
            | (finite, Self::Unbounded) => finite,

            (Self::Finite(left), Self::Finite(right)) => {
                Self::Finite(left.min(right))
            }
        }
    }

    /// Adds two non-negative bounds.
    pub fn checked_add(self, other: Self) -> BoundResult<Self> {
        match (self, other) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => {
                Ok(Self::Unbounded)
            }

            (Self::Finite(left), Self::Finite(right)) => {
                let value = left + right;

                if !value.is_finite() {
                    return Err(BoundError::NumericalOverflow {
                        operation: "finite bound addition",
                    });
                }

                Ok(Self::Finite(value))
            }
        }
    }

    /// Multiplies two non-negative bounds.
    pub fn checked_mul(self, other: Self) -> BoundResult<Self> {
        match (self, other) {
            (Self::Finite(left), Self::Finite(right)) => {
                let value = left * right;

                if !value.is_finite() {
                    return Err(BoundError::NumericalOverflow {
                        operation: "finite bound multiplication",
                    });
                }

                Ok(Self::Finite(value))
            }

            (Self::Finite(left), Self::Unbounded)
                if left == 0.0 =>
            {
                // 0 * an unbounded non-negative quantity is mathematically
                // zero. The caller's model has supplied an exact zero factor.
                Ok(Self::Finite(0.0))
            }

            (Self::Unbounded, Self::Finite(right))
                if right == 0.0 =>
            {
                Ok(Self::Finite(0.0))
            }

            (Self::Unbounded, _) | (_, Self::Unbounded) => {
                Ok(Self::Unbounded)
            }
        }
    }

    /// Multiplies a bound by a finite non-negative scalar.
    pub fn checked_mul_scalar(self, scalar: f64) -> BoundResult<Self> {
        validate_non_negative("scalar", scalar)?;

        match self {
            Self::Finite(value) => {
                let result = value * scalar;

                if !result.is_finite() {
                    return Err(BoundError::NumericalOverflow {
                        operation: "bound scalar multiplication",
                    });
                }

                Ok(Self::Finite(result))
            }

            Self::Unbounded if scalar == 0.0 => {
                Ok(Self::Finite(0.0))
            }

            Self::Unbounded => Ok(Self::Unbounded),
        }
    }

    /// Squares a bound.
    pub fn checked_square(self) -> BoundResult<Self> {
        self.checked_mul(self)
    }

    /// Computes root-sum-square with another bound.
    pub fn root_sum_square(self, other: Self) -> BoundResult<Self> {
        match (self, other) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => {
                Ok(Self::Unbounded)
            }

            (Self::Finite(left), Self::Finite(right)) => {
                let value = left.mul_add(left, right * right);

                if !value.is_finite() {
                    return Err(BoundError::NumericalOverflow {
                        operation: "root-sum-square accumulation",
                    });
                }

                let result = value.sqrt();

                if !result.is_finite() {
                    return Err(BoundError::NumericalOverflow {
                        operation: "root-sum-square square root",
                    });
                }

                Ok(Self::Finite(result))
            }
        }
    }
}

impl Default for BoundValue {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for BoundValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(value) => write!(formatter, "{value:.17}"),
            Self::Unbounded => formatter.write_str("unbounded"),
        }
    }
}

// =============================================================================
// Conservative error bound
// =============================================================================

/// A deterministic conservative bound on a non-negative error quantity.
///
/// The lower endpoint is always finite.
///
/// The upper endpoint may be finite or explicitly unbounded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConservativeErrorBound {
    lower: f64,
    upper: BoundValue,
}

impl ConservativeErrorBound {
    /// Creates a finite conservative error bound.
    pub fn finite(lower: f64, upper: f64) -> BoundResult<Self> {
        validate_non_negative("lower bound", lower)?;
        validate_non_negative("upper bound", upper)?;

        if lower > upper {
            return Err(BoundError::InvalidInterval { lower, upper });
        }

        Ok(Self {
            lower,
            upper: BoundValue::Finite(upper),
        })
    }

    /// Creates a bound with an explicitly unbounded upper endpoint.
    pub fn unbounded(lower: f64) -> BoundResult<Self> {
        validate_non_negative("lower bound", lower)?;

        Ok(Self {
            lower,
            upper: BoundValue::Unbounded,
        })
    }

    /// Creates an exact error quantity represented as a bound.
    pub fn exact(value: f64) -> BoundResult<Self> {
        Self::finite(value, value)
    }

    /// Creates the zero bound.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            lower: 0.0,
            upper: BoundValue::Finite(0.0),
        }
    }

    /// Returns the lower endpoint.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.lower
    }

    /// Returns the upper endpoint.
    #[must_use]
    pub const fn upper(self) -> BoundValue {
        self.upper
    }

    /// Returns whether the upper endpoint is finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.upper.is_finite()
    }

    /// Returns whether the upper endpoint is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.upper.is_unbounded()
    }

    /// Returns the finite upper endpoint.
    pub fn finite_upper(self) -> BoundResult<f64> {
        self.upper.finite_value()
    }

    /// Returns the width when the upper endpoint is finite.
    pub fn width(self) -> BoundResult<f64> {
        let upper = self.finite_upper()?;
        let width = upper - self.lower;

        if !width.is_finite() {
            return Err(BoundError::NumericalOverflow {
                operation: "bound width",
            });
        }

        Ok(width)
    }

    /// Returns whether a finite value is contained by this bound.
    #[must_use]
    pub fn contains(self, value: f64) -> bool {
        if !value.is_finite() || value < 0.0 {
            return false;
        }

        if value < self.lower {
            return false;
        }

        match self.upper {
            BoundValue::Finite(upper) => value <= upper,
            BoundValue::Unbounded => true,
        }
    }

    /// Returns the finite upper bound as the canonical ZQN error quantity.
    ///
    /// This is the bridge into `propagation::error_budget`.
    pub fn as_error_quantity(self) -> BoundResult<ErrorQuantity> {
        ErrorQuantity::new(self.finite_upper().map_err(|_| {
            BoundError::RequiresFiniteBound {
                context: "error-budget conversion",
            }
        })?)
        .map_err(|_| BoundError::BudgetConversion {
            context: "finite conservative error bound",
        })
    }

    /// Converts this bound into a budget consumption record.
    ///
    /// Unbounded bounds are rejected because `ErrorQuantity` intentionally
    /// represents finite numerical quantities.
    pub fn as_budget_consumption(
        self,
        dimension: BudgetDimension,
    ) -> BoundResult<BudgetConsumption> {
        let quantity = self.as_error_quantity()?;

        Ok(BudgetConsumption::new(
            dimension,
            quantity,
        ))
    }

    /// Evaluates this bound against an existing error budget dimension.
    ///
    /// This does not mutate the budget.
    pub fn evaluate_against_budget(
        self,
        budget: &ErrorBudget,
        dimension: &BudgetDimension,
    ) -> Result<super::error_budget::BudgetEvaluation, BoundError> {
        let quantity = self.as_error_quantity()?;

        budget
            .evaluate_dimension(dimension, quantity)
            .map_err(|error| {
                BoundError::BudgetConversion {
                    context: budget_error_context(&error),
                }
            })
    }

    /// Adds two conservative error bounds.
    ///
    /// This is a worst-case additive bound.
    pub fn checked_add(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let lower = checked_add_finite(
            self.lower,
            other.lower,
            "lower-bound addition",
        )?;

        let upper = self.upper.checked_add(other.upper)?;

        Self::from_parts(lower, upper)
    }

    /// Takes the conservative maximum of two bounds.
    ///
    /// This is appropriate when the mathematical quantity is known to be the
    /// maximum of two error contributions.
    pub fn maximum(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let lower = self.lower.max(other.lower);
        let upper = self.upper.max(other.upper);

        Self::from_parts(lower, upper)
    }

    /// Combines two bounds using root-sum-square.
    ///
    /// This operation is available because it is a standard conservative
    /// representation under appropriate independence/orthogonality
    /// assumptions. The caller remains responsible for establishing those
    /// assumptions.
    pub fn root_sum_square(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let lower = root_sum_square_finite(
            self.lower,
            other.lower,
        )?;

        let upper = self.upper.root_sum_square(other.upper)?;

        Self::from_parts(lower, upper)
    }

    /// Multiplies the bound by a finite non-negative factor.
    pub fn checked_mul_scalar(
        self,
        factor: f64,
    ) -> BoundResult<Self> {
        validate_non_negative("factor", factor)?;

        let lower = checked_mul_finite(
            self.lower,
            factor,
            "lower-bound scalar multiplication",
        )?;

        let upper = self.upper.checked_mul_scalar(factor)?;

        Self::from_parts(lower, upper)
    }

    /// Adds an exact non-negative error contribution.
    pub fn add_exact_error(
        self,
        error: f64,
    ) -> BoundResult<Self> {
        validate_non_negative("error contribution", error)?;

        let lower = checked_add_finite(
            self.lower,
            error,
            "lower-bound error accumulation",
        )?;

        let upper = self.upper.checked_add(
            BoundValue::Finite(error),
        )?;

        Self::from_parts(lower, upper)
    }

    fn from_parts(
        lower: f64,
        upper: BoundValue,
    ) -> BoundResult<Self> {
        validate_non_negative("lower bound", lower)?;

        match upper {
            BoundValue::Finite(value) => {
                validate_non_negative("upper bound", value)?;

                if lower > value {
                    return Err(BoundError::InvalidInterval {
                        lower,
                        upper: value,
                    });
                }
            }

            BoundValue::Unbounded => {}
        }

        Ok(Self { lower, upper })
    }
}

// =============================================================================
// Signed deterministic interval
// =============================================================================

/// A deterministic closed interval for a signed scalar quantity.
///
/// Unlike [`ConservativeErrorBound`], this type is not restricted to
/// non-negative values.
///
/// It is useful for conservative propagation of intermediate quantities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValueInterval {
    lower: f64,
    upper: f64,
}

impl ValueInterval {
    /// Creates a validated interval.
    pub fn new(lower: f64, upper: f64) -> BoundResult<Self> {
        validate_finite("interval lower", lower)?;
        validate_finite("interval upper", upper)?;

        if lower > upper {
            return Err(BoundError::InvalidInterval { lower, upper });
        }

        Ok(Self { lower, upper })
    }

    /// Creates an exact interval.
    pub fn exact(value: f64) -> BoundResult<Self> {
        Self::new(value, value)
    }

    /// Returns the lower endpoint.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.lower
    }

    /// Returns the upper endpoint.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.upper
    }

    /// Returns the midpoint.
    #[must_use]
    pub fn midpoint(self) -> f64 {
        self.lower + (self.upper - self.lower) * 0.5
    }

    /// Returns the interval width.
    pub fn width(self) -> BoundResult<f64> {
        let width = self.upper - self.lower;

        if !width.is_finite() {
            return Err(BoundError::NumericalOverflow {
                operation: "interval width",
            });
        }

        Ok(width)
    }

    /// Returns whether a value lies within the interval.
    #[must_use]
    pub fn contains(self, value: f64) -> bool {
        value.is_finite()
            && value >= self.lower
            && value <= self.upper
    }

    /// Conservative interval addition.
    pub fn checked_add(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let lower = checked_add_finite(
            self.lower,
            other.lower,
            "interval addition lower endpoint",
        )?;

        let upper = checked_add_finite(
            self.upper,
            other.upper,
            "interval addition upper endpoint",
        )?;

        Self::new(lower, upper)
    }

    /// Conservative interval subtraction.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let lower = checked_sub_finite(
            self.lower,
            other.upper,
            "interval subtraction lower endpoint",
        )?;

        let upper = checked_sub_finite(
            self.upper,
            other.lower,
            "interval subtraction upper endpoint",
        )?;

        Self::new(lower, upper)
    }

    /// Conservative interval multiplication.
    ///
    /// All four endpoint products are considered because neither interval is
    /// assumed to have a fixed sign.
    pub fn checked_mul(
        self,
        other: Self,
    ) -> BoundResult<Self> {
        let products = [
            checked_mul_finite(
                self.lower,
                other.lower,
                "interval multiplication",
            )?,
            checked_mul_finite(
                self.lower,
                other.upper,
                "interval multiplication",
            )?,
            checked_mul_finite(
                self.upper,
                other.lower,
                "interval multiplication",
            )?,
            checked_mul_finite(
                self.upper,
                other.upper,
                "interval multiplication",
            )?,
        ];

        let mut lower = products[0];
        let mut upper = products[0];

        for value in products.iter().copied().skip(1) {
            lower = lower.min(value);
            upper = upper.max(value);
        }

        Self::new(lower, upper)
    }

    /// Conservative interval division.
    ///
    /// Division by an interval containing zero is rejected because the
    /// mathematical result is unbounded and cannot be represented by this
    /// finite interval type.
    pub fn checked_div(
        self,
        denominator: Self,
    ) -> BoundResult<Self> {
        if denominator.contains(0.0) {
            return Err(BoundError::UnboundedResult {
                operation: "interval division by an interval containing zero",
            });
        }

        let reciprocal_lower = 1.0 / denominator.upper;
        let reciprocal_upper = 1.0 / denominator.lower;

        validate_finite(
            "interval reciprocal lower",
            reciprocal_lower,
        )?;

        validate_finite(
            "interval reciprocal upper",
            reciprocal_upper,
        )?;

        let reciprocal = if reciprocal_lower <= reciprocal_upper {
            Self::new(reciprocal_lower, reciprocal_upper)?
        } else {
            Self::new(reciprocal_upper, reciprocal_lower)?
        };

        self.checked_mul(reciprocal)
    }

    /// Conservative affine transformation:
    ///
    /// ```text
    /// y = a*x + b
    /// ```
    pub fn affine(
        self,
        a: f64,
        b: f64,
    ) -> BoundResult<Self> {
        validate_finite("affine coefficient", a)?;
        validate_finite("affine offset", b)?;

        let scaled = if a >= 0.0 {
            Self::new(
                checked_mul_finite(
                    a,
                    self.lower,
                    "affine lower scaling",
                )?,
                checked_mul_finite(
                    a,
                    self.upper,
                    "affine upper scaling",
                )?,
            )?
        } else {
            Self::new(
                checked_mul_finite(
                    a,
                    self.upper,
                    "affine lower scaling",
                )?,
                checked_mul_finite(
                    a,
                    self.lower,
                    "affine upper scaling",
                )?,
            )?
        };

        scaled.checked_add(Self::exact(b)?)
    }

    /// Converts a signed interval into a conservative absolute-magnitude
    /// error bound.
    ///
    /// The resulting upper bound is:
    ///
    /// ```text
    /// max(abs(lower), abs(upper))
    /// ```
    pub fn absolute_error_bound(self) -> BoundResult<ConservativeErrorBound> {
        let lower_abs = self.lower.abs();
        let upper_abs = self.upper.abs();

        validate_finite(
            "absolute lower magnitude",
            lower_abs,
        )?;

        validate_finite(
            "absolute upper magnitude",
            upper_abs,
        )?;

        let maximum = lower_abs.max(upper_abs);

        ConservativeErrorBound::finite(
            0.0,
            maximum,
        )
    }
}

// =============================================================================
// Aggregation
// =============================================================================

/// Explicit aggregation strategy for conservative bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoundAggregation {
    /// Worst-case additive accumulation.
    Sum,

    /// Worst-case maximum.
    Maximum,

    /// Root-sum-square accumulation.
    ///
    /// The caller must establish the assumptions under which RSS is justified.
    RootSumSquare,
}

impl Default for BoundAggregation {
    fn default() -> Self {
        Self::Sum
    }
}

impl fmt::Display for BoundAggregation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => formatter.write_str("sum"),
            Self::Maximum => formatter.write_str("maximum"),
            Self::RootSumSquare => {
                formatter.write_str("root_sum_square")
            }
        }
    }
}

/// Aggregates conservative error bounds from a streaming iterator.
///
/// This function does not materialize the iterator and therefore does not
/// impose a collection-size requirement.
pub fn aggregate<I>(
    values: I,
    aggregation: BoundAggregation,
) -> BoundResult<ConservativeErrorBound>
where
    I: IntoIterator<Item = ConservativeErrorBound>,
{
    let mut result = ConservativeErrorBound::zero();

    match aggregation {
        BoundAggregation::Sum => {
            for value in values {
                result = result.checked_add(value)?;
            }
        }

        BoundAggregation::Maximum => {
            for value in values {
                result = result.maximum(value)?;
            }
        }

        BoundAggregation::RootSumSquare => {
            for value in values {
                result = result.root_sum_square(value)?;
            }
        }
    }

    Ok(result)
}

// =============================================================================
// Weighted accumulation
// =============================================================================

/// A conservative contribution with an explicit non-negative factor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedBound {
    bound: ConservativeErrorBound,
    factor: f64,
}

impl WeightedBound {
    /// Creates a weighted contribution.
    pub fn new(
        bound: ConservativeErrorBound,
        factor: f64,
    ) -> BoundResult<Self> {
        validate_non_negative("weight factor", factor)?;

        Ok(Self { bound, factor })
    }

    /// Returns the underlying bound.
    #[must_use]
    pub const fn bound(self) -> ConservativeErrorBound {
        self.bound
    }

    /// Returns the factor.
    #[must_use]
    pub const fn factor(self) -> f64 {
        self.factor
    }

    /// Materializes the weighted conservative bound.
    pub fn materialize(self) -> BoundResult<ConservativeErrorBound> {
        self.bound.checked_mul_scalar(self.factor)
    }
}

/// Aggregates weighted bounds without materializing an intermediate collection.
pub fn aggregate_weighted<I>(
    values: I,
    aggregation: BoundAggregation,
) -> BoundResult<ConservativeErrorBound>
where
    I: IntoIterator<Item = WeightedBound>,
{
    let materialized = values.into_iter().map(WeightedBound::materialize);

    let mut result = ConservativeErrorBound::zero();

    match aggregation {
        BoundAggregation::Sum => {
            for value in materialized {
                result = result.checked_add(value?)?;
            }
        }

        BoundAggregation::Maximum => {
            for value in materialized {
                result = result.maximum(value?)?;
            }
        }

        BoundAggregation::RootSumSquare => {
            for value in materialized {
                result = result.root_sum_square(value?)?;
            }
        }
    }

    Ok(result)
}

// =============================================================================
// Resource limits
// =============================================================================

/// Explicit operational policy for bound calculations.
///
/// These limits are not quantum-machine-size limits.
///
/// `None` means that this module imposes no additional restriction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundLimits {
    /// Maximum number of input bounds accepted by a caller-facing operation
    /// when the operation explicitly checks collection cardinality.
    pub max_input_bounds: Option<u128>,

    /// Maximum number of scalar operations permitted by an operation that
    /// exposes operation-count accounting.
    pub max_scalar_operations: Option<u128>,
}

impl BoundLimits {
    /// Creates an unrestricted policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_input_bounds: None,
            max_scalar_operations: None,
        }
    }

    /// Validates an input-bound count.
    pub fn check_input_bounds(
        &self,
        count: usize,
    ) -> BoundResult<()> {
        if let Some(maximum) = self.max_input_bounds {
            let requested = count as u128;

            if requested > maximum {
                return Err(BoundError::ResourceLimitExceeded {
                    requested,
                    maximum,
                    resource: "input_bounds",
                });
            }
        }

        Ok(())
    }

    /// Validates a scalar-operation count.
    pub fn check_scalar_operations(
        &self,
        count: u128,
    ) -> BoundResult<()> {
        if let Some(maximum) = self.max_scalar_operations {
            if count > maximum {
                return Err(BoundError::ResourceLimitExceeded {
                    requested: count,
                    maximum,
                    resource: "scalar_operations",
                });
            }
        }

        Ok(())
    }
}

impl Default for BoundLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Budget integration
// =============================================================================

/// Evaluates a finite conservative bound against an error budget.
///
/// This is the preferred integration boundary for `error_budget.rs`.
pub fn evaluate_against_budget(
    bound: ConservativeErrorBound,
    budget: &ErrorBudget,
    dimension: &BudgetDimension,
) -> BoundResult<super::error_budget::BudgetEvaluation> {
    bound.evaluate_against_budget(budget, dimension)
}

/// Creates a budget consumption from a finite conservative bound.
pub fn budget_consumption(
    bound: ConservativeErrorBound,
    dimension: BudgetDimension,
) -> BoundResult<BudgetConsumption> {
    bound.as_budget_consumption(dimension)
}

// =============================================================================
// Error-bound transformations
// =============================================================================

/// Converts a signed deterministic interval into an absolute conservative
/// error bound around zero.
pub fn absolute_bound(
    interval: ValueInterval,
) -> BoundResult<ConservativeErrorBound> {
    interval.absolute_error_bound()
}

/// Computes the conservative error between two scalar intervals.
///
/// Given:
///
/// ```text
/// expected ∈ [e_l, e_u]
/// observed ∈ [o_l, o_u]
/// ```
///
/// the maximum absolute difference is:
///
/// ```text
/// max(|e_l - o_u|, |e_u - o_l|)
/// ```
pub fn difference_bound(
    expected: ValueInterval,
    observed: ValueInterval,
) -> BoundResult<ConservativeErrorBound> {
    let first = checked_sub_finite(
        expected.lower(),
        observed.upper(),
        "difference lower candidate",
    )?;

    let second = checked_sub_finite(
        expected.upper(),
        observed.lower(),
        "difference upper candidate",
    )?;

    let first_abs = first.abs();
    let second_abs = second.abs();

    validate_finite(
        "difference bound first magnitude",
        first_abs,
    )?;

    validate_finite(
        "difference bound second magnitude",
        second_abs,
    )?;

    ConservativeErrorBound::finite(
        0.0,
        first_abs.max(second_abs),
    )
}

/// Computes the conservative absolute error introduced by a finite affine
/// perturbation.
///
/// If:
///
/// ```text
/// x ∈ [x_l, x_u]
/// y = a*x + b
/// ```
///
/// and the nominal/reference value is `reference`, the result is the maximum
/// absolute deviation of `y` from `reference`.
pub fn affine_deviation_bound(
    interval: ValueInterval,
    a: f64,
    b: f64,
    reference: f64,
) -> BoundResult<ConservativeErrorBound> {
    validate_finite("reference", reference)?;

    let transformed = interval.affine(a, b)?;

    difference_bound(
        transformed,
        ValueInterval::exact(reference)?,
    )
}

/// Computes the conservative absolute error resulting from a perturbation
/// interval around a nominal value.
///
/// If:
///
/// ```text
/// perturbation ∈ [p_l, p_u]
/// ```
///
/// the absolute error bound is:
///
/// ```text
/// max(|p_l|, |p_u|)
/// ```
pub fn perturbation_bound(
    perturbation: ValueInterval,
) -> BoundResult<ConservativeErrorBound> {
    perturbation.absolute_error_bound()
}

// =============================================================================
// Bound propagation through products
// =============================================================================

/// Conservative error bound for a product when the nominal product and
/// component errors are known.
///
/// This helper uses first-order worst-case differential propagation:
///
/// ```text
/// δ(ab) ≈ |b|δa + |a|δb + δaδb
/// ```
///
/// The quadratic term is retained, making this a conservative algebraic
/// enclosure when the supplied component errors are deterministic magnitude
/// bounds.
///
/// The nominal values may be signed.
pub fn product_error_bound(
    left_nominal: f64,
    left_error: ConservativeErrorBound,
    right_nominal: f64,
    right_error: ConservativeErrorBound,
) -> BoundResult<ConservativeErrorBound> {
    validate_finite("left nominal", left_nominal)?;
    validate_finite("right nominal", right_nominal)?;

    let left_factor = left_error
        .checked_mul_scalar(left_nominal.abs())?;

    let right_factor = right_error
        .checked_mul_scalar(right_nominal.abs())?;

    let cross = left_error.checked_mul_scalar(1.0)?;

    let cross = match (
        cross.upper(),
        right_error.upper(),
    ) {
        (BoundValue::Finite(left), BoundValue::Finite(right)) => {
            let product = left * right;

            if !product.is_finite() {
                return Err(BoundError::NumericalOverflow {
                    operation: "product error quadratic term",
                });
            }

            ConservativeErrorBound::exact(product)?
        }

        (BoundValue::Unbounded, _)
        | (_, BoundValue::Unbounded) => {
            ConservativeErrorBound::unbounded(0.0)?
        }
    };

    left_factor
        .checked_add(right_factor)?
        .checked_add(cross)
}

/// Conservative error bound for an affine transformation of an uncertain
/// scalar:
///
/// ```text
/// y = a*x + b
/// ```
pub fn affine_error_bound(
    nominal: f64,
    error: ConservativeErrorBound,
    a: f64,
) -> BoundResult<ConservativeErrorBound> {
    validate_finite("nominal", nominal)?;
    validate_finite("affine coefficient", a)?;

    error.checked_mul_scalar(a.abs())
}

// =============================================================================
// Composition helpers
// =============================================================================

/// Composes two conservative bounds sequentially under worst-case additive
/// semantics.
pub fn compose_additive(
    first: ConservativeErrorBound,
    second: ConservativeErrorBound,
) -> BoundResult<ConservativeErrorBound> {
    first.checked_add(second)
}

/// Composes two conservative bounds under explicit maximum semantics.
pub fn compose_maximum(
    first: ConservativeErrorBound,
    second: ConservativeErrorBound,
) -> BoundResult<ConservativeErrorBound> {
    first.maximum(second)
}

/// Composes two conservative bounds under explicit RSS semantics.
///
/// The caller is responsible for ensuring the RSS assumptions are valid.
pub fn compose_root_sum_square(
    first: ConservativeErrorBound,
    second: ConservativeErrorBound,
) -> BoundResult<ConservativeErrorBound> {
    first.root_sum_square(second)
}

// =============================================================================
// Utility validation
// =============================================================================

fn validate_finite(
    field: &'static str,
    value: f64,
) -> BoundResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(BoundError::NonFinite { field, value })
    }
}

fn validate_non_negative(
    field: &'static str,
    value: f64,
) -> BoundResult<()> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(BoundError::NegativeValue {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_tolerance(value: f64) -> BoundResult<()> {
    validate_finite("tolerance", value)?;

    if value <= 0.0 {
        return Err(BoundError::InvalidTolerance { value });
    }

    Ok(())
}

fn checked_add_finite(
    left: f64,
    right: f64,
    operation: &'static str,
) -> BoundResult<f64> {
    validate_finite("left operand", left)?;
    validate_finite("right operand", right)?;

    let result = left + right;

    if !result.is_finite() {
        return Err(BoundError::NumericalOverflow {
            operation,
        });
    }

    Ok(result)
}

fn checked_sub_finite(
    left: f64,
    right: f64,
    operation: &'static str,
) -> BoundResult<f64> {
    validate_finite("left operand", left)?;
    validate_finite("right operand", right)?;

    let result = left - right;

    if !result.is_finite() {
        return Err(BoundError::NumericalOverflow {
            operation,
        });
    }

    Ok(result)
}

fn checked_mul_finite(
    left: f64,
    right: f64,
    operation: &'static str,
) -> BoundResult<f64> {
    validate_finite("left operand", left)?;
    validate_finite("right operand", right)?;

    let result = left * right;

    if !result.is_finite() {
        return Err(BoundError::NumericalOverflow {
            operation,
        });
    }

    Ok(result)
}

fn root_sum_square_finite(
    left: f64,
    right: f64,
) -> BoundResult<f64> {
    validate_non_negative("RSS left value", left)?;
    validate_non_negative("RSS right value", right)?;

    let squared = left.mul_add(left, right * right);

    if !squared.is_finite() {
        return Err(BoundError::NumericalOverflow {
            operation: "finite root-sum-square",
        });
    }

    let result = squared.sqrt();

    if !result.is_finite() {
        return Err(BoundError::NumericalOverflow {
            operation: "finite root-sum-square square root",
        });
    }

    Ok(result)
}

fn budget_error_context(
    error: &ErrorBudgetError,
) -> &'static str {
    match error {
        ErrorBudgetError::EmptyDimension => {
            "empty budget dimension"
        }

        ErrorBudgetError::InvalidDimension { .. } => {
            "invalid budget dimension"
        }

        ErrorBudgetError::DuplicateDimension { .. } => {
            "duplicate budget dimension"
        }

        ErrorBudgetError::UnknownDimension { .. } => {
            "unknown budget dimension"
        }

        ErrorBudgetError::NonFiniteValue { .. } => {
            "non-finite budget value"
        }

        ErrorBudgetError::NegativeValue { .. } => {
            "negative budget value"
        }

        ErrorBudgetError::NumericalOverflow { .. } => {
            "budget numerical overflow"
        }

        ErrorBudgetError::UnsupportedSchemaVersion { .. } => {
            "unsupported budget schema"
        }

        ErrorBudgetError::InconsistentDimensionKey => {
            "inconsistent budget dimension key"
        }

        ErrorBudgetError::ChildBudgetExceedsParent { .. } => {
            "child budget exceeds parent"
        }

        ErrorBudgetError::IncompatibleTolerance { .. } => {
            "incompatible budget tolerance"
        }

        ErrorBudgetError::CustomAggregationRequired => {
            "custom budget aggregation"
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn finite(value: f64) -> ConservativeErrorBound {
        ConservativeErrorBound::exact(value)
            .expect("valid finite bound")
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            BOUNDS_SCHEMA_ID,
            "zamani.quantum.zqn.propagation.bounds"
        );

        assert_eq!(BOUNDS_SCHEMA_VERSION, 1);
    }

    #[test]
    fn unsafe_code_is_forbidden_by_module_contract() {
        // This test intentionally has no unsafe operation.
        //
        // The compile-time guarantee is provided by:
        //
        //     #![forbid(unsafe_code)]
        //
        // This test documents the contract.
        assert_eq!(BOUNDS_SCHEMA_VERSION, 1);
    }

    #[test]
    fn finite_bound_validates_order() {
        assert!(ConservativeErrorBound::finite(0.1, 0.2).is_ok());
        assert!(ConservativeErrorBound::finite(0.2, 0.1).is_err());
    }

    #[test]
    fn negative_bound_is_rejected() {
        assert!(ConservativeErrorBound::finite(-0.1, 0.2).is_err());
        assert!(ConservativeErrorBound::finite(0.1, -0.2).is_err());
    }

    #[test]
    fn non_finite_bound_is_rejected() {
        assert!(
            ConservativeErrorBound::finite(
                f64::NAN,
                1.0
            )
            .is_err()
        );

        assert!(
            ConservativeErrorBound::finite(
                0.0,
                f64::INFINITY
            )
            .is_err()
        );

        assert!(
            ConservativeErrorBound::finite(
                0.0,
                f64::NEG_INFINITY
            )
            .is_err()
        );
    }

    #[test]
    fn explicit_unbounded_is_valid() {
        let bound =
            ConservativeErrorBound::unbounded(0.0)
                .expect("valid unbounded bound");

        assert!(bound.is_unbounded());
        assert!(!bound.is_finite());
        assert!(bound.finite_upper().is_err());
    }

    #[test]
    fn unbounded_is_not_f64_infinity() {
        let bound =
            ConservativeErrorBound::unbounded(0.0)
                .expect("valid bound");

        assert!(matches!(
            bound.upper(),
            BoundValue::Unbounded
        ));
    }

    #[test]
    fn exact_bound_contains_exact_value() {
        let bound = finite(0.5);

        assert!(bound.contains(0.5));
        assert!(!bound.contains(0.4));
        assert!(!bound.contains(0.6));
    }

    #[test]
    fn finite_addition_is_conservative() {
        let first =
            ConservativeErrorBound::finite(0.1, 0.2)
                .expect("valid bound");

        let second =
            ConservativeErrorBound::finite(0.2, 0.3)
                .expect("valid bound");

        let result =
            first.checked_add(second)
                .expect("valid addition");

        assert_eq!(result.lower(), 0.3);
        assert_eq!(
            result.finite_upper().expect("finite"),
            0.5
        );
    }

    #[test]
    fn unbounded_addition_remains_unbounded() {
        let first = finite(0.1);

        let second =
            ConservativeErrorBound::unbounded(0.2)
                .expect("valid bound");

        let result =
            first.checked_add(second)
                .expect("valid addition");

        assert!(result.is_unbounded());
        assert_eq!(result.lower(), 0.3);
    }

    #[test]
    fn maximum_is_conservative() {
        let first =
            ConservativeErrorBound::finite(0.1, 0.4)
                .expect("valid bound");

        let second =
            ConservativeErrorBound::finite(0.2, 0.3)
                .expect("valid bound");

        let result =
            first.maximum(second)
                .expect("valid maximum");

        assert_eq!(result.lower(), 0.2);
        assert_eq!(
            result.finite_upper().expect("finite"),
            0.4
        );
    }

    #[test]
    fn rss_is_correct_for_simple_values() {
        let first = finite(3.0);
        let second = finite(4.0);

        let result =
            first.root_sum_square(second)
                .expect("valid RSS");

        assert_eq!(
            result.finite_upper().expect("finite"),
            5.0
        );
    }

    #[test]
    fn rss_with_unbounded_input_is_unbounded() {
        let first = finite(1.0);

        let second =
            ConservativeErrorBound::unbounded(0.0)
                .expect("valid bound");

        let result =
            first.root_sum_square(second)
                .expect("valid RSS");

        assert!(result.is_unbounded());
    }

    #[test]
    fn zero_factor_can_collapse_unbounded_bound() {
        let bound =
            ConservativeErrorBound::unbounded(0.0)
                .expect("valid bound");

        let result =
            bound.checked_mul_scalar(0.0)
                .expect("zero scaling");

        assert_eq!(
            result.finite_upper().expect("finite"),
            0.0
        );
    }

    #[test]
    fn affine_interval_handles_negative_slope() {
        let interval =
            ValueInterval::new(1.0, 3.0)
                .expect("valid interval");

        let result =
            interval.affine(-2.0, 4.0)
                .expect("valid affine operation");

        assert_eq!(result.lower(), -2.0);
        assert_eq!(result.upper(), 2.0);
    }

    #[test]
    fn interval_addition_is_conservative() {
        let first =
            ValueInterval::new(1.0, 2.0)
                .expect("valid interval");

        let second =
            ValueInterval::new(3.0, 5.0)
                .expect("valid interval");

        let result =
            first.checked_add(second)
                .expect("valid addition");

        assert_eq!(result.lower(), 4.0);
        assert_eq!(result.upper(), 7.0);
    }

    #[test]
    fn interval_subtraction_is_conservative() {
        let first =
            ValueInterval::new(1.0, 2.0)
                .expect("valid interval");

        let second =
            ValueInterval::new(3.0, 5.0)
                .expect("valid interval");

        let result =
            first.checked_sub(second)
                .expect("valid subtraction");

        assert_eq!(result.lower(), -4.0);
        assert_eq!(result.upper(), -1.0);
    }

    #[test]
    fn interval_multiplication_checks_all_endpoints() {
        let first =
            ValueInterval::new(-2.0, 3.0)
                .expect("valid interval");

        let second =
            ValueInterval::new(-4.0, 5.0)
                .expect("valid interval");

        let result =
            first.checked_mul(second)
                .expect("valid multiplication");

        assert_eq!(result.lower(), -12.0);
        assert_eq!(result.upper(), 15.0);
    }

    #[test]
    fn interval_division_rejects_zero() {
        let numerator =
            ValueInterval::new(1.0, 2.0)
                .expect("valid interval");

        let denominator =
            ValueInterval::new(-1.0, 1.0)
                .expect("valid interval");

        assert!(matches!(
            numerator.checked_div(denominator),
            Err(BoundError::UnboundedResult { .. })
        ));
    }

    #[test]
    fn absolute_bound_is_correct() {
        let interval =
            ValueInterval::new(-3.0, 2.0)
                .expect("valid interval");

        let bound =
            interval.absolute_error_bound()
                .expect("valid absolute bound");

        assert_eq!(
            bound.finite_upper().expect("finite"),
            3.0
        );
    }

    #[test]
    fn difference_bound_is_correct() {
        let expected =
            ValueInterval::new(1.0, 2.0)
                .expect("valid interval");

        let observed =
            ValueInterval::new(3.0, 5.0)
                .expect("valid interval");

        let bound =
            difference_bound(expected, observed)
                .expect("valid difference bound");

        assert_eq!(
            bound.finite_upper().expect("finite"),
            4.0
        );
    }

    #[test]
    fn affine_deviation_is_correct() {
        let input =
            ValueInterval::new(1.0, 2.0)
                .expect("valid interval");

        let bound =
            affine_deviation_bound(
                input,
                2.0,
                1.0,
                4.0,
            )
            .expect("valid deviation");

        // y ∈ [3, 5], reference = 4.
        // Maximum absolute deviation = 1.
        assert_eq!(
            bound.finite_upper().expect("finite"),
            1.0
        );
    }

    #[test]
    fn perturbation_bound_is_correct() {
        let perturbation =
            ValueInterval::new(-0.2, 0.4)
                .expect("valid interval");

        let bound =
            perturbation_bound(perturbation)
                .expect("valid bound");

        assert_eq!(
            bound.finite_upper().expect("finite"),
            0.4
        );
    }

    #[test]
    fn product_error_bound_is_conservative() {
        let left = finite(0.1);
        let right = finite(0.2);

        let result =
            product_error_bound(
                2.0,
                left,
                3.0,
                right,
            )
            .expect("valid product bound");

        // |3|*0.1 + |2|*0.2 + 0.1*0.2
        // = 0.3 + 0.4 + 0.02
        assert!(
            (result.finite_upper().expect("finite") - 0.72).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn affine_error_bound_scales_magnitude() {
        let error = finite(0.2);

        let result =
            affine_error_bound(
                10.0,
                error,
                -3.0,
            )
            .expect("valid affine error");

        assert_eq!(
            result.finite_upper().expect("finite"),
            0.6
        );
    }

    #[test]
    fn aggregate_sum_is_streaming() {
        let values = [
            finite(0.1),
            finite(0.2),
            finite(0.3),
        ];

        let result =
            aggregate(
                values.into_iter(),
                BoundAggregation::Sum,
            )
            .expect("valid aggregation");

        assert!(
            (result.finite_upper().expect("finite") - 0.6).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn aggregate_maximum_is_correct() {
        let values = [
            finite(0.1),
            finite(0.7),
            finite(0.3),
        ];

        let result =
            aggregate(
                values.into_iter(),
                BoundAggregation::Maximum,
            )
            .expect("valid aggregation");

        assert_eq!(
            result.finite_upper().expect("finite"),
            0.7
        );
    }

    #[test]
    fn aggregate_rss_is_correct() {
        let values = [
            finite(3.0),
            finite(4.0),
        ];

        let result =
            aggregate(
                values.into_iter(),
                BoundAggregation::RootSumSquare,
            )
            .expect("valid aggregation");

        assert_eq!(
            result.finite_upper().expect("finite"),
            5.0
        );
    }

    #[test]
    fn weighted_aggregation_is_correct() {
        let values = [
            WeightedBound::new(finite(2.0), 3.0)
                .expect("valid weight"),
            WeightedBound::new(finite(1.0), 4.0)
                .expect("valid weight"),
        ];

        let result =
            aggregate_weighted(
                values.into_iter(),
                BoundAggregation::Sum,
            )
            .expect("valid aggregation");

        assert_eq!(
            result.finite_upper().expect("finite"),
            10.0
        );
    }

    #[test]
    fn negative_weight_is_rejected() {
        assert!(
            WeightedBound::new(
                finite(1.0),
                -1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn resource_limit_is_operational_only() {
        let limits = BoundLimits {
            max_input_bounds: Some(2),
            max_scalar_operations: None,
        };

        assert!(limits.check_input_bounds(2).is_ok());

        assert!(matches!(
            limits.check_input_bounds(3),
            Err(BoundError::ResourceLimitExceeded {
                resource: "input_bounds",
                ..
            })
        ));
    }

    #[test]
    fn unlimited_policy_has_no_artificial_size_constant() {
        let limits = BoundLimits::unlimited();

        assert!(limits.check_input_bounds(usize::MAX).is_ok());

        assert!(
            limits
                .check_scalar_operations(u128::MAX)
                .is_ok()
        );
    }

    #[test]
    fn finite_bound_can_become_budget_consumption() {
        let bound = finite(0.25);

        let dimension =
            BudgetDimension::new("gate_error")
                .expect("valid dimension");

        let consumption =
            budget_consumption(bound, dimension)
                .expect("valid conversion");

        assert_eq!(
            consumption.consumed().value(),
            0.25
        );
    }

    #[test]
    fn unbounded_bound_cannot_become_finite_budget_quantity() {
        let bound =
            ConservativeErrorBound::unbounded(0.0)
                .expect("valid bound");

        let dimension =
            BudgetDimension::new("gate_error")
                .expect("valid dimension");

        assert!(
            bound
                .as_budget_consumption(dimension)
                .is_err()
        );
    }

    #[test]
    fn bound_can_be_evaluated_against_error_budget() {
        let mut budget =
            ErrorBudget::new(
                super::super::error_budget::ErrorBudgetIdentity::new(1)
            );

        let dimension =
            BudgetDimension::new("gate_error")
                .expect("valid dimension");

        budget
            .allocate(
                dimension.clone(),
                super::super::error_budget::ErrorTolerance::new(1.0)
                    .expect("valid tolerance"),
            )
            .expect("valid allocation");

        let bound = finite(0.25);

        let evaluation =
            evaluate_against_budget(
                bound,
                &budget,
                &dimension,
            )
            .expect("valid evaluation");

        assert!(evaluation.is_compliant());
        assert_eq!(
            evaluation
                .consumed()
                .value(),
            0.25
        );
    }

    #[test]
    fn zero_bound_is_valid() {
        let bound = ConservativeErrorBound::zero();

        assert_eq!(bound.lower(), 0.0);
        assert_eq!(
            bound.finite_upper().expect("finite"),
            0.0
        );
        assert!(bound.contains(0.0));
    }

    #[test]
    fn exact_policy_values_remain_finite() {
        let bound = finite(1.0);

        let result =
            bound.checked_mul_scalar(2.0)
                .expect("valid multiplication");

        assert_eq!(
            result.finite_upper().expect("finite"),
            2.0
        );
    }

    #[test]
    fn numerical_overflow_is_not_silently_unbounded() {
        let result =
            ConservativeErrorBound::exact(f64::MAX)
                .expect("valid maximum");

        let multiplication =
            result.checked_mul_scalar(2.0);

        assert!(matches!(
            multiplication,
            Err(BoundError::NumericalOverflow { .. })
        ));
    }

    #[test]
    fn infinity_is_rejected_as_input() {
        assert!(
            BoundValue::finite(f64::INFINITY)
                .is_err()
        );

        assert!(
            BoundValue::finite(f64::NEG_INFINITY)
                .is_err()
        );
    }

    #[test]
    fn mathematical_unboundedness_remains_representable() {
        let value = BoundValue::unbounded();

        assert!(value.is_unbounded());
        assert!(value.finite_value().is_err());
    }

    #[test]
    fn bound_contains_all_finite_non_negative_values_when_unbounded() {
        let bound =
            ConservativeErrorBound::unbounded(1.0)
                .expect("valid bound");

        assert!(bound.contains(1.0));
        assert!(bound.contains(1000.0));
        assert!(bound.contains(f64::MAX));
        assert!(!bound.contains(0.999));
        assert!(!bound.contains(-1.0));
        assert!(!bound.contains(f64::NAN));
    }

    #[test]
    fn deterministic_results_are_repeatable() {
        let first = aggregate(
            [
                finite(0.1),
                finite(0.2),
                finite(0.3),
            ],
            BoundAggregation::Sum,
        )
        .expect("valid result");

        let second = aggregate(
            [
                finite(0.1),
                finite(0.2),
                finite(0.3),
            ],
            BoundAggregation::Sum,
        )
        .expect("valid result");

        assert_eq!(first, second);
    }

    #[test]
    fn no_qubit_identity_is_created_by_bounds() {
        // Bounds are intentionally resource-independent.
        //
        // Resource association belongs to an integration layer using:
        //
        // crate::quantum::ir::qubit::QubitId
        // crate::quantum::ir::qubit::PhysicalQubitId
        //
        // This test documents that this module's scalar contract does not
        // require a second identity system.
        let bound = finite(0.1);

        assert_eq!(
            bound.finite_upper().expect("finite"),
            0.1
        );
    }
}