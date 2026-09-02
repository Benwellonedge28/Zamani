//! Zamani Quantum Noise (ZQN) — Propagation Uncertainty.
//!
//! # Purpose
//!
//! This module owns deterministic propagation of already-quantified
//! uncertainty through mathematical transformations.
//!
//! It answers:
//!
//! > "Given uncertain input quantities and a declared propagation model,
//! > what uncertainty should be assigned to the resulting quantity?"
//!
//! This module is deliberately different from:
//!
//! - `characterization::uncertainty`, which estimates uncertainty from
//!   observations;
//! - `propagation::error_budget`, which allocates and aggregates error
//!   budgets;
//! - `propagation::fidelity`, which owns fidelity/distance measures;
//! - `propagation::sensitivity`, which owns sensitivity analysis;
//! - `noise/*`, which owns physical noise semantics;
//! - `calibration/*`, which owns calibration state;
//! - `simulation/*`, which owns execution.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - scalar uncertainty representations;
//! - deterministic uncertainty intervals;
//! - standard uncertainty;
//! - covariance representation;
//! - covariance validation;
//! - linear uncertainty propagation;
//! - independent first-order propagation;
//! - covariance-aware first-order propagation;
//! - conservative interval propagation for supported operations;
//! - uncertainty propagation policies;
//! - propagation result contracts;
//! - numerical validation required by those operations;
//! - explicit approximation classification.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - raw experimental observations;
//! - statistical estimators;
//! - confidence-interval construction;
//! - Bayesian inference;
//! - probability distributions;
//! - quantum channels;
//! - quantum states;
//! - noise models;
//! - calibration storage;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulator execution;
//! - benchmark methodology;
//! - canonical Quantum IR;
//! - qubit identity;
//! - serialization wire formats;
//! - cryptographic hashing;
//! - automatic differentiation;
//! - symbolic algebra;
//! - vendor numerical libraries.
//!
//! # Architectural position
//!
//! ```text
//! characterization / calibration / measurement
//!                    |
//!                    v
//!          quantified uncertainty
//!                    |
//!                    v
//!       propagation::uncertainty
//!                    |
//!          +---------+----------+
//!          |                    |
//!          v                    v
//!   error_budget           sensitivity
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!              fidelity / QEC
//!                    |
//!                    v
//!          routing / scheduling
//!                    |
//!                    v
//!                 runtime
//! ```
//!
//! # Fundamental separation
//!
//! This module never infers statistical uncertainty from raw samples.
//!
//! For example:
//!
//! ```text
//! estimate = 0.0017
//! standard uncertainty = 0.0002
//! ```
//!
//! is an input to this module.
//!
//! This module can propagate that uncertainty through:
//!
//! ```text
//! f(x) = 2x + 1
//! ```
//!
//! but it does not determine whether `0.0002` came from:
//!
//! - standard error;
//! - calibration uncertainty;
//! - a confidence interval;
//! - a Bayesian posterior;
//! - a deterministic physical bound.
//!
//! The upstream subsystem owns that semantic decision.
//!
//! # Approximation contract
//!
//! First-order propagation is explicitly approximate.
//!
//! It is based on a local linearization:
//!
//! ```text
//! y = f(x)
//!
//! J = ∂f/∂x
//!
//! Σ_y ≈ J Σ_x Jᵀ
//! ```
//!
//! This module never presents that approximation as an exact result.
//!
//! Exact/conservative interval propagation is represented separately.
//!
//! # Scalability
//!
//! There is no semantic limit on:
//!
//! - number of uncertain parameters;
//! - number of outputs;
//! - number of covariance entries;
//! - number of quantum resources;
//! - number of qubits;
//! - number of machines;
//! - circuit depth;
//! - execution duration.
//!
//! Concrete implementations are naturally bounded by available memory and
//! computation time.
//!
//! No machine-size constant is encoded here.
//!
//! The caller can choose explicit resource limits through
//! `PropagationLimits`.
//!
//! # Important scaling property
//!
//! The mathematical formulas operate on dimensions supplied by the caller.
//!
//! This file does NOT contain assumptions such as:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 5 qubits
//! 20 qubits
//! 127 qubits
//! 1024 qubits
//! ```
//!
//! A parameter vector of length `N` is handled using the same semantics for
//! every finite `N` supported by the available resources.
//!
//! # Determinism
//!
//! This module:
//!
//! - uses no random numbers;
//! - uses no global mutable state;
//! - does not read the system clock;
//! - does not depend on thread identity;
//! - does not use unordered maps;
//! - does not perform hidden parallel reduction.
//!
//! Given identical inputs, policy, floating-point environment, and execution
//! order, calculations are deterministic.
//!
//! If a caller parallelizes large matrix operations and requires bit-for-bit
//! reproducibility, the caller must use a deterministic reduction strategy.
//!
//! # Numerical safety
//!
//! The module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative standard uncertainty;
//! - invalid intervals;
//! - invalid covariance matrices;
//! - invalid tolerances;
//! - dimension mismatches;
//! - arithmetic overflow where checked integer arithmetic is applicable.
//!
//! It does not silently:
//!
//! - convert NaN to zero;
//! - clamp negative uncertainty;
//! - take absolute values to hide invalid input;
//! - replace infinity with the largest finite value;
//! - silently switch propagation methods.
//!
//! # Quantum-resource identity
//!
//! Uncertainty propagation itself is resource-agnostic.
//!
//! Therefore this module deliberately does not import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! When a higher-level propagation result is attached to a quantum resource,
//! that surrounding layer must use the canonical IDs owned by
//! `quantum::ir::qubit`.
//!
//! A second ZQN-specific qubit identifier must not be introduced.
//!
//! # Integration with characterization
//!
//! `characterization::uncertainty` produces uncertainty quantities.
//!
//! Those quantities can be converted into `UncertainValue` or
//! `UncertaintyInterval` and passed to this module.
//!
//! This module does not consume raw characterization observations.
//!
//! # Integration with calibration
//!
//! Calibration parameters may expose:
//!
//! ```text
//! value
//! uncertainty
//! covariance
//! validity
//! provenance
//! ```
//!
//! Calibration remains the owner of calibration state.
//!
//! This module only propagates the numerical uncertainty supplied by it.
//!
//! # Integration with noise
//!
//! A noise model may have uncertain parameters such as:
//!
//! ```text
//! T1
//! T2
//! gate_error
//! readout_error
//! crosstalk_strength
//! drift_rate
//! ```
//!
//! The noise subsystem owns the physical meaning of those quantities.
//!
//! This module can propagate their uncertainty through derived quantities,
//! such as an estimated circuit error or fidelity bound.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may consume propagation results as costs or bounds.
//!
//! This module does not know routing or scheduling policy.
//!
//! # Integration with QEC
//!
//! QEC may consume propagated physical uncertainty when evaluating logical
//! error sensitivity or error-budget allocation.
//!
//! QEC remains responsible for decoding and correction.
//!
//! # Integration with error_budget.rs
//!
//! `error_budget.rs` should consume `PropagationResult` and combine it with
//! its own allocation/aggregation semantics.
//!
//! This module must not become an error-budget manager.
//!
//! # Integration with sensitivity.rs
//!
//! Sensitivity analysis may construct the Jacobian consumed here.
//!
//! This module does not calculate parameter sensitivity itself.
//!
//! # Integration with fidelity.rs
//!
//! Fidelity calculations can consume propagated bounds or standard
//! uncertainties, but fidelity metrics remain owned by `fidelity.rs`.
//!
//! # Serialization
//!
//! This module defines semantic data structures only.
//!
//! It does not define a wire format.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! # Security
//!
//! Propagation is potentially exposed to untrusted model/calibration input.
//!
//! Therefore:
//!
//! - dimensions are checked before arithmetic;
//! - matrix dimensions use checked multiplication;
//! - optional work limits are explicit;
//! - no recursive numerical algorithm is used;
//! - no hidden allocation based on attacker-controlled values occurs without
//!   a caller-visible operation;
//! - no external processes are invoked;
//! - no unsafe code exists.
//!
//! # Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. uncertainty representation is explicit;
//! 2. intervals are validated;
//! 3. covariance is validated;
//! 4. independent and correlated propagation are distinct;
//! 5. first-order approximation is explicitly labelled;
//! 6. interval propagation is explicitly conservative;
//! 7. dimensions are checked;
//! 8. no machine-size assumptions exist;
//! 9. no quantum-resource IDs are duplicated;
//! 10. no RNG exists;
//! 11. no global mutable state exists;
//! 12. numerical failures are explicit;
//! 13. resource policy is explicit;
//! 14. serialization remains owned by `zqn::io`;
//! 15. the module can be integrated into future propagation files without
//!     changing its mathematical contracts.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic schema identifier.
pub const UNCERTAINTY_SCHEMA_ID: &str =
    "zamani.quantum.zqn.propagation.uncertainty";

/// Semantic version of this module's public contract.
pub const UNCERTAINTY_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance used by validation operations.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

// ============================================================================
// Error
// ============================================================================

/// Failure returned by uncertainty propagation.
#[derive(Clone, Debug, PartialEq)]
pub enum PropagationUncertaintyError {
    /// A supplied floating-point value was not finite.
    NonFinite {
        /// Semantic name of the invalid value.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// Standard uncertainty cannot be negative.
    NegativeStandardUncertainty {
        /// Supplied uncertainty.
        value: f64,
    },

    /// An interval has invalid ordering.
    InvalidInterval {
        /// Lower bound.
        lower: f64,
        /// Upper bound.
        upper: f64,
    },

    /// Covariance matrix has an invalid dimension.
    CovarianceDimensionMismatch {
        /// Expected matrix dimension.
        expected: usize,
        /// Actual matrix dimension.
        actual: usize,
    },

    /// Covariance matrix is not square.
    CovarianceNotSquare {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        columns: usize,
    },

    /// Covariance matrix contains invalid values.
    InvalidCovariance {
        /// Row index.
        row: usize,
        /// Column index.
        column: usize,
        /// Invalid value.
        value: f64,
    },

    /// Covariance matrix is not sufficiently symmetric.
    CovarianceNotSymmetric {
        /// Row index.
        row: usize,
        /// Column index.
        column: usize,
        /// First value.
        lhs: f64,
        /// Mirrored value.
        rhs: f64,
    },

    /// Dimensions of a propagation operation do not agree.
    DimensionMismatch {
        /// Left/input dimension.
        left: usize,
        /// Right/output dimension.
        right: usize,
        /// Description of the mismatch.
        context: &'static str,
    },

    /// Matrix/vector size arithmetic overflowed.
    SizeOverflow {
        /// Description of the calculation.
        context: &'static str,
    },

    /// A resource policy rejected the requested work.
    ResourceLimitExceeded {
        /// Requested units.
        requested: u128,
        /// Configured maximum.
        maximum: u128,
        /// Resource category.
        resource: &'static str,
    },

    /// The supplied tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// A covariance operation would require an unsupported numerical
    /// representation.
    NumericalFailure {
        /// Description of the operation.
        operation: &'static str,
    },

    /// A requested propagation method cannot represent the supplied data.
    UnsupportedPropagation {
        /// Method.
        method: &'static str,
    },

    /// A requested approximation is not permitted by policy.
    ApproximationNotAllowed {
        /// Approximation method.
        method: &'static str,
    },
}

impl fmt::Display for PropagationUncertaintyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(f, "non-finite uncertainty value in `{field}`: {value}")
            }
            Self::NegativeStandardUncertainty { value } => {
                write!(f, "standard uncertainty cannot be negative: {value}")
            }
            Self::InvalidInterval { lower, upper } => {
                write!(
                    f,
                    "invalid uncertainty interval [{lower}, {upper}]"
                )
            }
            Self::CovarianceDimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "covariance dimension mismatch: expected {expected}, got {actual}"
                )
            }
            Self::CovarianceNotSquare { rows, columns } => {
                write!(
                    f,
                    "covariance matrix is not square: {rows}x{columns}"
                )
            }
            Self::InvalidCovariance {
                row,
                column,
                value,
            } => {
                write!(
                    f,
                    "invalid covariance entry ({row}, {column}) = {value}"
                )
            }
            Self::CovarianceNotSymmetric {
                row,
                column,
                lhs,
                rhs,
            } => {
                write!(
                    f,
                    "covariance matrix is not symmetric at ({row}, {column}): {lhs} != {rhs}"
                )
            }
            Self::DimensionMismatch {
                left,
                right,
                context,
            } => {
                write!(
                    f,
                    "dimension mismatch in {context}: {left} != {right}"
                )
            }
            Self::SizeOverflow { context } => {
                write!(f, "size arithmetic overflow in {context}")
            }
            Self::ResourceLimitExceeded {
                requested,
                maximum,
                resource,
            } => {
                write!(
                    f,
                    "resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
                )
            }
            Self::InvalidTolerance { value } => {
                write!(f, "invalid numerical tolerance: {value}")
            }
            Self::NumericalFailure { operation } => {
                write!(f, "numerical failure during {operation}")
            }
            Self::UnsupportedPropagation { method } => {
                write!(f, "unsupported uncertainty propagation method: {method}")
            }
            Self::ApproximationNotAllowed { method } => {
                write!(
                    f,
                    "approximate propagation method is not allowed: {method}"
                )
            }
        }
    }
}

impl std::error::Error for PropagationUncertaintyError {}

/// Result alias for this module.
pub type PropagationResult<T> =
    Result<T, PropagationUncertaintyError>;

// ============================================================================
// Policy
// ============================================================================

/// Controls which uncertainty propagation methods are permitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropagationPolicy {
    /// Only mathematically conservative/non-approximate operations are
    /// permitted.
    ExactOrConservative,

    /// First-order approximations are permitted.
    AllowFirstOrderApproximation,
}

impl Default for PropagationPolicy {
    fn default() -> Self {
        Self::AllowFirstOrderApproximation
    }
}

// ============================================================================
// Resource limits
// ============================================================================

/// Explicit resource policy for propagation calculations.
///
/// These are execution-policy limits, not semantic limits on quantum-system
/// size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropagationLimits {
    /// Optional maximum number of scalar output values.
    pub max_output_values: Option<u128>,

    /// Optional maximum number of scalar matrix elements that an operation
    /// may inspect or produce.
    pub max_matrix_elements: Option<u128>,
}

impl Default for PropagationLimits {
    fn default() -> Self {
        Self {
            max_output_values: None,
            max_matrix_elements: None,
        }
    }
}

impl PropagationLimits {
    /// Creates an unlimited policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_output_values: None,
            max_matrix_elements: None,
        }
    }

    fn check_output_values(&self, count: usize) -> PropagationResult<()> {
        if let Some(maximum) = self.max_output_values {
            let requested = count as u128;

            if requested > maximum {
                return Err(
                    PropagationUncertaintyError::ResourceLimitExceeded {
                        requested,
                        maximum,
                        resource: "output_values",
                    },
                );
            }
        }

        Ok(())
    }

    fn check_matrix_elements(
        &self,
        rows: usize,
        columns: usize,
    ) -> PropagationResult<()> {
        let count = rows
            .checked_mul(columns)
            .ok_or(PropagationUncertaintyError::SizeOverflow {
                context: "matrix element count",
            })?;

        if let Some(maximum) = self.max_matrix_elements {
            let requested = count as u128;

            if requested > maximum {
                return Err(
                    PropagationUncertaintyError::ResourceLimitExceeded {
                        requested,
                        maximum,
                        resource: "matrix_elements",
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Uncertainty kind
// ============================================================================

/// Semantic kind of an uncertainty quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncertaintyKind {
    /// Standard uncertainty suitable for covariance propagation.
    Standard,

    /// Deterministic lower/upper bound.
    Bounded,

    /// Both a standard uncertainty and deterministic bounds are available.
    StandardAndBounded,
}

// ============================================================================
// Scalar uncertain value
// ============================================================================

/// A scalar value with explicitly represented uncertainty.
///
/// The value is the nominal/point value. It is not itself an estimator.
///
/// A standard uncertainty is suitable for first-order covariance propagation.
///
/// Bounds are deterministic bounds and are not interpreted as confidence
/// intervals.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UncertainValue {
    value: f64,
    standard_uncertainty: Option<f64>,
    bounds: Option<UncertaintyInterval>,
}

impl UncertainValue {
    /// Creates a value without an attached uncertainty.
    pub fn exact(value: f64) -> PropagationResult<Self> {
        validate_finite("value", value)?;

        Ok(Self {
            value,
            standard_uncertainty: None,
            bounds: None,
        })
    }

    /// Creates a value with a standard uncertainty.
    pub fn standard(
        value: f64,
        standard_uncertainty: f64,
    ) -> PropagationResult<Self> {
        validate_finite("value", value)?;
        validate_standard_uncertainty(standard_uncertainty)?;

        Ok(Self {
            value,
            standard_uncertainty: Some(standard_uncertainty),
            bounds: None,
        })
    }

    /// Creates a value with deterministic bounds.
    pub fn bounded(
        value: f64,
        bounds: UncertaintyInterval,
    ) -> PropagationResult<Self> {
        validate_finite("value", value)?;

        if !bounds.contains(value) {
            return Err(PropagationUncertaintyError::InvalidInterval {
                lower: bounds.lower(),
                upper: bounds.upper(),
            });
        }

        Ok(Self {
            value,
            standard_uncertainty: None,
            bounds: Some(bounds),
        })
    }

    /// Creates a value with both standard uncertainty and deterministic
    /// bounds.
    pub fn standard_and_bounded(
        value: f64,
        standard_uncertainty: f64,
        bounds: UncertaintyInterval,
    ) -> PropagationResult<Self> {
        validate_finite("value", value)?;
        validate_standard_uncertainty(standard_uncertainty)?;

        if !bounds.contains(value) {
            return Err(PropagationUncertaintyError::InvalidInterval {
                lower: bounds.lower(),
                upper: bounds.upper(),
            });
        }

        Ok(Self {
            value,
            standard_uncertainty: Some(standard_uncertainty),
            bounds: Some(bounds),
        })
    }

    /// Returns the nominal value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the standard uncertainty, if available.
    #[must_use]
    pub const fn standard_uncertainty(&self) -> Option<f64> {
        self.standard_uncertainty
    }

    /// Returns deterministic bounds, if available.
    #[must_use]
    pub const fn bounds(&self) -> Option<UncertaintyInterval> {
        self.bounds
    }

    /// Returns the semantic uncertainty kind.
    #[must_use]
    pub const fn kind(&self) -> UncertaintyKind {
        match (self.standard_uncertainty, self.bounds) {
            (Some(_), Some(_)) => UncertaintyKind::StandardAndBounded,
            (Some(_), None) => UncertaintyKind::Standard,
            (None, Some(_)) => UncertaintyKind::Bounded,
            (None, None) => UncertaintyKind::Standard,
        }
    }

    /// Returns whether no uncertainty information is attached.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.standard_uncertainty.is_none() && self.bounds.is_none()
    }
}

// ============================================================================
// Deterministic interval
// ============================================================================

/// Closed deterministic interval.
///
/// This is a physical/mathematical bound, not a confidence interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UncertaintyInterval {
    lower: f64,
    upper: f64,
}

impl UncertaintyInterval {
    /// Creates a validated closed interval.
    pub fn new(lower: f64, upper: f64) -> PropagationResult<Self> {
        validate_finite("lower", lower)?;
        validate_finite("upper", upper)?;

        if lower > upper {
            return Err(PropagationUncertaintyError::InvalidInterval {
                lower,
                upper,
            });
        }

        Ok(Self { lower, upper })
    }

    /// Returns the lower bound.
    #[must_use]
    pub const fn lower(&self) -> f64 {
        self.lower
    }

    /// Returns the upper bound.
    #[must_use]
    pub const fn upper(&self) -> f64 {
        self.upper
    }

    /// Returns the midpoint.
    #[must_use]
    pub fn midpoint(&self) -> f64 {
        self.lower + (self.upper - self.lower) * 0.5
    }

    /// Returns the half-width.
    #[must_use]
    pub fn half_width(&self) -> f64 {
        (self.upper - self.lower) * 0.5
    }

    /// Returns whether the interval contains a value.
    #[must_use]
    pub fn contains(&self, value: f64) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }
}

// ============================================================================
// Covariance matrix
// ============================================================================

/// Dense covariance matrix in row-major order.
///
/// The matrix is represented as:
///
/// ```text
/// covariance[row * dimension + column]
/// ```
///
/// This type owns covariance semantics, not a particular matrix backend.
///
/// Large systems should use another backend/representation through a future
/// adapter rather than forcing dense covariance storage.
#[derive(Clone, Debug, PartialEq)]
pub struct CovarianceMatrix {
    dimension: usize,
    elements: Vec<f64>,
}

impl CovarianceMatrix {
    /// Creates a zero covariance matrix.
    pub fn zeros(
        dimension: usize,
        limits: PropagationLimits,
    ) -> PropagationResult<Self> {
        limits.check_matrix_elements(dimension, dimension)?;

        let count = dimension.checked_mul(dimension).ok_or(
            PropagationUncertaintyError::SizeOverflow {
                context: "covariance matrix allocation",
            },
        )?;

        Ok(Self {
            dimension,
            elements: vec![0.0; count],
        })
    }

    /// Creates a covariance matrix from row-major elements.
    pub fn from_elements(
        dimension: usize,
        elements: Vec<f64>,
        tolerance: f64,
        limits: PropagationLimits,
    ) -> PropagationResult<Self> {
        validate_tolerance(tolerance)?;
        limits.check_matrix_elements(dimension, dimension)?;

        let expected = dimension.checked_mul(dimension).ok_or(
            PropagationUncertaintyError::SizeOverflow {
                context: "covariance matrix dimension",
            },
        )?;

        if elements.len() != expected {
            return Err(
                PropagationUncertaintyError::CovarianceDimensionMismatch {
                    expected,
                    actual: elements.len(),
                },
            );
        }

        for row in 0..dimension {
            for column in 0..dimension {
                let index = row * dimension + column;
                let value = elements[index];

                if !value.is_finite() || value < 0.0 && row == column {
                    return Err(
                        PropagationUncertaintyError::InvalidCovariance {
                            row,
                            column,
                            value,
                        },
                    );
                }
            }
        }

        for row in 0..dimension {
            for column in 0..dimension {
                let lhs = elements[row * dimension + column];
                let rhs = elements[column * dimension + row];

                if (lhs - rhs).abs() > tolerance {
                    return Err(
                        PropagationUncertaintyError::CovarianceNotSymmetric {
                            row,
                            column,
                            lhs,
                            rhs,
                        },
                    );
                }
            }
        }

        Ok(Self {
            dimension,
            elements,
        })
    }

    /// Returns the covariance dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns all row-major elements.
    #[must_use]
    pub fn elements(&self) -> &[f64] {
        &self.elements
    }

    /// Returns one covariance element.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<f64> {
        if row >= self.dimension || column >= self.dimension {
            return None;
        }

        Some(self.elements[row * self.dimension + column])
    }

    /// Returns the variance of one parameter.
    #[must_use]
    pub fn variance(&self, index: usize) -> Option<f64> {
        self.get(index, index)
    }

    /// Returns a covariance entry.
    #[must_use]
    pub fn covariance(&self, row: usize, column: usize) -> Option<f64> {
        self.get(row, column)
    }

    /// Creates a diagonal covariance matrix from standard uncertainties.
    pub fn from_standard_uncertainties(
        uncertainties: &[f64],
        limits: PropagationLimits,
    ) -> PropagationResult<Self> {
        limits.check_matrix_elements(
            uncertainties.len(),
            uncertainties.len(),
        )?;

        let mut elements = vec![
            0.0;
            uncertainties
                .len()
                .checked_mul(uncertainties.len())
                .ok_or(
                    PropagationUncertaintyError::SizeOverflow {
                        context: "diagonal covariance allocation",
                    },
                )?
        ];

        for (index, uncertainty) in uncertainties.iter().copied().enumerate()
        {
            validate_standard_uncertainty(uncertainty)?;

            elements[index * uncertainties.len() + index] =
                uncertainty * uncertainty;

            if !elements[index * uncertainties.len() + index].is_finite() {
                return Err(PropagationUncertaintyError::NumericalFailure {
                    operation: "variance calculation",
                });
            }
        }

        Self::from_elements(
            uncertainties.len(),
            elements,
            DEFAULT_TOLERANCE,
            limits,
        )
    }
}

// ============================================================================
// Propagation result
// ============================================================================

/// Semantic approximation classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropagationAccuracy {
    /// The operation is represented as a deterministic conservative bound.
    Conservative,

    /// The result uses first-order local linearization.
    FirstOrderApproximation,
}

/// Result of scalar uncertainty propagation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropagatedUncertainty {
    /// Nominal output value.
    pub value: f64,

    /// Propagated standard uncertainty, if available.
    pub standard_uncertainty: Option<f64>,

    /// Propagated deterministic bounds, if available.
    pub bounds: Option<UncertaintyInterval>,

    /// How the uncertainty was obtained.
    pub accuracy: PropagationAccuracy,
}

impl PropagatedUncertainty {
    /// Returns the uncertainty kind represented by this result.
    #[must_use]
    pub const fn kind(&self) -> UncertaintyKind {
        match (self.standard_uncertainty, self.bounds) {
            (Some(_), Some(_)) => UncertaintyKind::StandardAndBounded,
            (Some(_), None) => UncertaintyKind::Standard,
            (None, Some(_)) => UncertaintyKind::Bounded,
            (None, None) => UncertaintyKind::Standard,
        }
    }
}

/// Result of vector-valued covariance propagation.
#[derive(Clone, Debug, PartialEq)]
pub struct PropagatedVector {
    /// Nominal output values.
    pub values: Vec<f64>,

    /// Output covariance matrix.
    pub covariance: CovarianceMatrix,

    /// Accuracy classification.
    pub accuracy: PropagationAccuracy,
}

// ============================================================================
// Jacobian
// ============================================================================

/// Dense row-major Jacobian.
///
/// For `m` outputs and `n` inputs:
///
/// ```text
/// J has m * n entries
/// J[row * inputs + column]
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Jacobian {
    outputs: usize,
    inputs: usize,
    elements: Vec<f64>,
}

impl Jacobian {
    /// Creates a Jacobian from row-major elements.
    pub fn new(
        outputs: usize,
        inputs: usize,
        elements: Vec<f64>,
        limits: PropagationLimits,
    ) -> PropagationResult<Self> {
        limits.check_matrix_elements(outputs, inputs)?;

        let expected = outputs.checked_mul(inputs).ok_or(
            PropagationUncertaintyError::SizeOverflow {
                context: "Jacobian dimensions",
            },
        )?;

        if elements.len() != expected {
            return Err(PropagationUncertaintyError::DimensionMismatch {
                left: elements.len(),
                right: expected,
                context: "Jacobian element count",
            });
        }

        for value in &elements {
            validate_finite("Jacobian element", *value)?;
        }

        Ok(Self {
            outputs,
            inputs,
            elements,
        })
    }

    /// Returns a zero Jacobian.
    pub fn zeros(
        outputs: usize,
        inputs: usize,
        limits: PropagationLimits,
    ) -> PropagationResult<Self> {
        let count = outputs.checked_mul(inputs).ok_or(
            PropagationUncertaintyError::SizeOverflow {
                context: "Jacobian allocation",
            },
        )?;

        Self::new(outputs, inputs, vec![0.0; count], limits)
    }

    /// Returns the number of outputs.
    #[must_use]
    pub const fn outputs(&self) -> usize {
        self.outputs
    }

    /// Returns the number of inputs.
    #[must_use]
    pub const fn inputs(&self) -> usize {
        self.inputs
    }

    /// Returns the row-major Jacobian elements.
    #[must_use]
    pub fn elements(&self) -> &[f64] {
        &self.elements
    }

    /// Returns one derivative.
    #[must_use]
    pub fn get(&self, output: usize, input: usize) -> Option<f64> {
        if output >= self.outputs || input >= self.inputs {
            return None;
        }

        Some(self.elements[output * self.inputs + input])
    }
}

// ============================================================================
// Scalar first-order propagation
// ============================================================================

/// Propagates independent scalar standard uncertainties using first-order
/// linearization.
///
/// For:
///
/// ```text
/// y = f(x1, ..., xn)
/// ```
///
/// and independent input uncertainties:
///
/// ```text
/// σ_y² ≈ Σᵢ (∂f/∂xᵢ)² σᵢ²
/// ```
///
/// This is explicitly an approximation.
pub fn propagate_independent(
    derivatives: &[f64],
    uncertainties: &[f64],
    output_value: f64,
    policy: PropagationPolicy,
) -> PropagationResult<PropagatedUncertainty> {
    if policy == PropagationPolicy::ExactOrConservative {
        return Err(
            PropagationUncertaintyError::ApproximationNotAllowed {
                method: "first-order independent propagation",
            },
        );
    }

    validate_finite("output_value", output_value)?;

    if derivatives.len() != uncertainties.len() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: derivatives.len(),
            right: uncertainties.len(),
            context: "independent scalar propagation",
        });
    }

    let mut variance = 0.0;

    for (derivative, uncertainty) in
        derivatives.iter().zip(uncertainties.iter())
    {
        validate_finite("derivative", *derivative)?;
        validate_standard_uncertainty(*uncertainty)?;

        let contribution =
            (*derivative * *uncertainty) * (*derivative * *uncertainty);

        if !contribution.is_finite() {
            return Err(PropagationUncertaintyError::NumericalFailure {
                operation: "independent variance propagation",
            });
        }

        variance += contribution;

        if !variance.is_finite() {
            return Err(PropagationUncertaintyError::NumericalFailure {
                operation: "variance accumulation",
            });
        }
    }

    let standard_uncertainty = variance.sqrt();

    if !standard_uncertainty.is_finite() {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "standard uncertainty calculation",
        });
    }

    Ok(PropagatedUncertainty {
        value: output_value,
        standard_uncertainty: Some(standard_uncertainty),
        bounds: None,
        accuracy: PropagationAccuracy::FirstOrderApproximation,
    })
}

// ============================================================================
// Scalar covariance-aware propagation
// ============================================================================

/// Propagates correlated scalar uncertainties using a first-order covariance
/// model.
///
/// For a scalar output:
///
/// ```text
/// σ_y² ≈ J Σ Jᵀ
/// ```
///
/// where `J` is the derivative row vector.
pub fn propagate_with_covariance(
    derivatives: &[f64],
    covariance: &CovarianceMatrix,
    output_value: f64,
    policy: PropagationPolicy,
) -> PropagationResult<PropagatedUncertainty> {
    if policy == PropagationPolicy::ExactOrConservative {
        return Err(
            PropagationUncertaintyError::ApproximationNotAllowed {
                method: "first-order covariance propagation",
            },
        );
    }

    validate_finite("output_value", output_value)?;

    if derivatives.len() != covariance.dimension() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: derivatives.len(),
            right: covariance.dimension(),
            context: "covariance-aware scalar propagation",
        });
    }

    let mut variance = 0.0;

    for row in 0..covariance.dimension() {
        let derivative_row = derivatives[row];

        validate_finite("derivative", derivative_row)?;

        for column in 0..covariance.dimension() {
            let covariance_value =
                covariance.covariance(row, column).ok_or(
                    PropagationUncertaintyError::DimensionMismatch {
                        left: covariance.dimension(),
                        right: covariance.dimension(),
                        context: "covariance access",
                    },
                )?;

            variance +=
                derivative_row * covariance_value * derivatives[column];

            if !variance.is_finite() {
                return Err(
                    PropagationUncertaintyError::NumericalFailure {
                        operation: "covariance variance accumulation",
                    },
                );
            }
        }
    }

    // A covariance matrix with numerical noise can produce a tiny negative
    // result. That does not justify silently accepting materially negative
    // variance. Only a tiny value within the configured floating-point
    // tolerance is treated as zero.
    if variance < 0.0 {
        if variance >= -DEFAULT_TOLERANCE {
            variance = 0.0;
        } else {
            return Err(
                PropagationUncertaintyError::NumericalFailure {
                    operation: "negative propagated variance",
                },
            );
        }
    }

    let standard_uncertainty = variance.sqrt();

    if !standard_uncertainty.is_finite() {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "covariance standard uncertainty calculation",
        });
    }

    Ok(PropagatedUncertainty {
        value: output_value,
        standard_uncertainty: Some(standard_uncertainty),
        bounds: None,
        accuracy: PropagationAccuracy::FirstOrderApproximation,
    })
}

// ============================================================================
// Vector covariance propagation
// ============================================================================

/// Propagates a vector of uncertain inputs through a Jacobian.
///
/// The operation is:
///
/// ```text
/// Σ_y ≈ J Σ_x Jᵀ
/// ```
///
/// This is the fundamental covariance propagation primitive for larger
/// quantum-noise calculations.
///
/// The function does not assume that the dimensions correspond to qubits.
/// They may represent arbitrary parameters, modes, resources, channels,
/// calibration values, or other quantities.
pub fn propagate_covariance(
    values: &[f64],
    covariance: &CovarianceMatrix,
    jacobian: &Jacobian,
    policy: PropagationPolicy,
    limits: PropagationLimits,
) -> PropagationResult<PropagatedVector> {
    if policy == PropagationPolicy::ExactOrConservative {
        return Err(
            PropagationUncertaintyError::ApproximationNotAllowed {
                method: "first-order covariance propagation",
            },
        );
    }

    if values.len() != covariance.dimension() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: values.len(),
            right: covariance.dimension(),
            context: "input values and covariance",
        });
    }

    if jacobian.inputs() != values.len() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: jacobian.inputs(),
            right: values.len(),
            context: "Jacobian inputs and values",
        });
    }

    limits.check_output_values(jacobian.outputs())?;
    limits.check_matrix_elements(
        jacobian.outputs(),
        jacobian.outputs(),
    )?;

    for value in values {
        validate_finite("input value", *value)?;
    }

    let outputs = jacobian.outputs();
    let inputs = jacobian.inputs();

    let mut output_values = Vec::with_capacity(outputs);

    // A Jacobian represents local derivatives, not the function itself.
    // Therefore callers supply the nominal output values separately.
    //
    // This API intentionally uses zero nominal outputs as a mathematical
    // placeholder only when no output values are supplied by the caller.
    // For physical use, prefer `propagate_covariance_with_outputs`.
    output_values.resize(outputs, 0.0);

    let mut output_covariance =
        CovarianceMatrix::zeros(outputs, limits)?;

    for output_row in 0..outputs {
        for output_column in 0..outputs {
            let mut value = 0.0;

            for input_row in 0..inputs {
                let j_left = jacobian
                    .get(output_row, input_row)
                    .ok_or(
                        PropagationUncertaintyError::DimensionMismatch {
                            left: output_row,
                            right: outputs,
                            context: "Jacobian row access",
                        },
                    )?;

                for input_column in 0..inputs {
                    let covariance_value =
                        covariance
                            .covariance(input_row, input_column)
                            .ok_or(
                                PropagationUncertaintyError::DimensionMismatch {
                                    left: input_row,
                                    right: inputs,
                                    context: "covariance access",
                                },
                            )?;

                    let j_right = jacobian
                        .get(output_column, input_column)
                        .ok_or(
                            PropagationUncertaintyError::DimensionMismatch {
                                left: output_column,
                                right: outputs,
                                context: "Jacobian column access",
                            },
                        )?;

                    value += j_left * covariance_value * j_right;

                    if !value.is_finite() {
                        return Err(
                            PropagationUncertaintyError::NumericalFailure {
                                operation: "vector covariance propagation",
                            },
                        );
                    }
                }
            }

            if value < 0.0 && output_row == output_column {
                if value >= -DEFAULT_TOLERANCE {
                    value = 0.0;
                } else {
                    return Err(
                        PropagationUncertaintyError::NumericalFailure {
                            operation: "negative output variance",
                        },
                    );
                }
            }

            let index = output_row * outputs + output_column;

            output_covariance.elements[index] = value;
        }
    }

    Ok(PropagatedVector {
        values: output_values,
        covariance: output_covariance,
        accuracy: PropagationAccuracy::FirstOrderApproximation,
    })
}

/// Propagates covariance while retaining caller-provided nominal output
/// values.
///
/// This is the preferred vector API when the caller already evaluated the
/// underlying function.
pub fn propagate_covariance_with_outputs(
    output_values: Vec<f64>,
    covariance: &CovarianceMatrix,
    jacobian: &Jacobian,
    policy: PropagationPolicy,
    limits: PropagationLimits,
) -> PropagationResult<PropagatedVector> {
    if policy == PropagationPolicy::ExactOrConservative {
        return Err(
            PropagationUncertaintyError::ApproximationNotAllowed {
                method: "first-order covariance propagation",
            },
        );
    }

    if output_values.len() != jacobian.outputs() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: output_values.len(),
            right: jacobian.outputs(),
            context: "output values and Jacobian",
        });
    }

    for value in &output_values {
        validate_finite("output value", *value)?;
    }

    let mut result =
        propagate_covariance(
            &[],
            covariance,
            jacobian,
            policy,
            limits,
        );

    match &mut result {
        Ok(propagated) => {
            propagated.values = output_values;
        }
        Err(
            PropagationUncertaintyError::DimensionMismatch {
                context: "input values and covariance",
                ..
            },
        ) => {
            // The underlying covariance propagation only needs the covariance
            // dimension for this operation. Re-run the matrix computation
            // through the dimension-safe helper below.
            result = propagate_covariance_matrix_only(
                output_values,
                covariance,
                jacobian,
                policy,
                limits,
            );
        }
        _ => {}
    }

    result
}

fn propagate_covariance_matrix_only(
    output_values: Vec<f64>,
    covariance: &CovarianceMatrix,
    jacobian: &Jacobian,
    policy: PropagationPolicy,
    limits: PropagationLimits,
) -> PropagationResult<PropagatedVector> {
    if policy == PropagationPolicy::ExactOrConservative {
        return Err(
            PropagationUncertaintyError::ApproximationNotAllowed {
                method: "first-order covariance propagation",
            },
        );
    }

    if jacobian.inputs() != covariance.dimension() {
        return Err(PropagationUncertaintyError::DimensionMismatch {
            left: jacobian.inputs(),
            right: covariance.dimension(),
            context: "Jacobian and covariance",
        });
    }

    limits.check_output_values(output_values.len())?;
    limits.check_matrix_elements(
        jacobian.outputs(),
        jacobian.outputs(),
    )?;

    let outputs = jacobian.outputs();
    let inputs = jacobian.inputs();

    let mut elements = vec![
        0.0;
        outputs.checked_mul(outputs).ok_or(
            PropagationUncertaintyError::SizeOverflow {
                context: "output covariance allocation",
            },
        )?
    ];

    for row in 0..outputs {
        for column in 0..outputs {
            let mut propagated = 0.0;

            for input_row in 0..inputs {
                let j_left = jacobian.get(row, input_row).ok_or(
                    PropagationUncertaintyError::DimensionMismatch {
                        left: row,
                        right: outputs,
                        context: "Jacobian access",
                    },
                )?;

                for input_column in 0..inputs {
                    let covariance_value =
                        covariance
                            .covariance(input_row, input_column)
                            .ok_or(
                                PropagationUncertaintyError::DimensionMismatch {
                                    left: input_row,
                                    right: inputs,
                                    context: "covariance access",
                                },
                            )?;

                    let j_right =
                        jacobian.get(column, input_column).ok_or(
                            PropagationUncertaintyError::DimensionMismatch {
                                left: column,
                                right: outputs,
                                context: "Jacobian access",
                            },
                        )?;

                    propagated +=
                        j_left * covariance_value * j_right;

                    if !propagated.is_finite() {
                        return Err(
                            PropagationUncertaintyError::NumericalFailure {
                                operation: "covariance matrix propagation",
                            },
                        );
                    }
                }
            }

            if row == column && propagated < 0.0 {
                if propagated >= -DEFAULT_TOLERANCE {
                    propagated = 0.0;
                } else {
                    return Err(
                        PropagationUncertaintyError::NumericalFailure {
                            operation: "negative propagated variance",
                        },
                    );
                }
            }

            elements[row * outputs + column] = propagated;
        }
    }

    let propagated_covariance = CovarianceMatrix::from_elements(
        outputs,
        elements,
        DEFAULT_TOLERANCE,
        limits,
    )?;

    Ok(PropagatedVector {
        values: output_values,
        covariance: propagated_covariance,
        accuracy: PropagationAccuracy::FirstOrderApproximation,
    })
}

// ============================================================================
// Conservative interval propagation
// ============================================================================

/// Propagates addition of two deterministic intervals.
pub fn add_intervals(
    lhs: UncertaintyInterval,
    rhs: UncertaintyInterval,
) -> PropagationResult<UncertaintyInterval> {
    let lower = lhs.lower + rhs.lower;
    let upper = lhs.upper + rhs.upper;

    if !lower.is_finite() || !upper.is_finite() {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "interval addition",
        });
    }

    UncertaintyInterval::new(lower, upper)
}

/// Propagates subtraction of two deterministic intervals.
pub fn subtract_intervals(
    lhs: UncertaintyInterval,
    rhs: UncertaintyInterval,
) -> PropagationResult<UncertaintyInterval> {
    let lower = lhs.lower - rhs.upper;
    let upper = lhs.upper - rhs.lower;

    if !lower.is_finite() || !upper.is_finite() {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "interval subtraction",
        });
    }

    UncertaintyInterval::new(lower, upper)
}

/// Propagates multiplication of two deterministic intervals.
///
/// All four endpoint products are evaluated because neither interval is
/// assumed to have a fixed sign.
pub fn multiply_intervals(
    lhs: UncertaintyInterval,
    rhs: UncertaintyInterval,
) -> PropagationResult<UncertaintyInterval> {
    let products = [
        lhs.lower * rhs.lower,
        lhs.lower * rhs.upper,
        lhs.upper * rhs.lower,
        lhs.upper * rhs.upper,
    ];

    for value in products {
        if !value.is_finite() {
            return Err(PropagationUncertaintyError::NumericalFailure {
                operation: "interval multiplication",
            });
        }
    }

    let mut lower = products[0];
    let mut upper = products[0];

    for value in products.iter().copied().skip(1) {
        lower = lower.min(value);
        upper = upper.max(value);
    }

    UncertaintyInterval::new(lower, upper)
}

/// Propagates division of deterministic intervals.
///
/// Division is rejected when the denominator interval contains zero because
/// the result is then unbounded and cannot be represented by a finite
/// interval.
pub fn divide_intervals(
    numerator: UncertaintyInterval,
    denominator: UncertaintyInterval,
) -> PropagationResult<UncertaintyInterval> {
    if denominator.contains(0.0) {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "division by an interval containing zero",
        });
    }

    let reciprocals = [
        1.0 / denominator.lower,
        1.0 / denominator.upper,
    ];

    if reciprocals.iter().any(|value| !value.is_finite()) {
        return Err(PropagationUncertaintyError::NumericalFailure {
            operation: "interval reciprocal",
        });
    }

    let reciprocal = UncertaintyInterval::new(
        reciprocals[0].min(reciprocals[1]),
        reciprocals[0].max(reciprocals[1]),
    )?;

    multiply_intervals(numerator, reciprocal)
}

/// Propagates an interval through an affine transformation:
///
/// ```text
/// y = a*x + b
/// ```
pub fn affine_interval(
    input: UncertaintyInterval,
    a: f64,
    b: f64,
) -> PropagationResult<UncertaintyInterval> {
    validate_finite("a", a)?;
    validate_finite("b", b)?;

    let scaled = if a >= 0.0 {
        UncertaintyInterval::new(
            a * input.lower,
            a * input.upper,
        )?
    } else {
        UncertaintyInterval::new(
            a * input.upper,
            a * input.lower,
        )?
    };

    let result = add_intervals(
        scaled,
        UncertaintyInterval::new(b, b)?,
    )?;

    Ok(result)
}

// ============================================================================
// Helper functions
// ============================================================================

fn validate_finite(
    field: &'static str,
    value: f64,
) -> PropagationResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PropagationUncertaintyError::NonFinite {
            field,
            value,
        })
    }
}

fn validate_standard_uncertainty(
    value: f64,
) -> PropagationResult<()> {
    validate_finite("standard_uncertainty", value)?;

    if value < 0.0 {
        return Err(
            PropagationUncertaintyError::NegativeStandardUncertainty {
                value,
            },
        );
    }

    Ok(())
}

fn validate_tolerance(value: f64) -> PropagationResult<()> {
    validate_finite("tolerance", value)?;

    if value <= 0.0 {
        return Err(PropagationUncertaintyError::InvalidTolerance {
            value,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_value_has_no_uncertainty() {
        let value = UncertainValue::exact(3.0).expect("valid value");

        assert_eq!(value.value(), 3.0);
        assert_eq!(value.standard_uncertainty(), None);
        assert_eq!(value.bounds(), None);
        assert!(value.is_exact());
    }

    #[test]
    fn standard_uncertainty_is_preserved() {
        let value =
            UncertainValue::standard(3.0, 0.2).expect("valid uncertainty");

        assert_eq!(value.value(), 3.0);
        assert_eq!(value.standard_uncertainty(), Some(0.2));
        assert_eq!(value.kind(), UncertaintyKind::Standard);
    }

    #[test]
    fn negative_standard_uncertainty_is_rejected() {
        let result = UncertainValue::standard(1.0, -0.1);

        assert!(matches!(
            result,
            Err(
                PropagationUncertaintyError::NegativeStandardUncertainty {
                    ..
                }
            )
        ));
    }

    #[test]
    fn non_finite_value_is_rejected() {
        let result = UncertainValue::exact(f64::NAN);

        assert!(matches!(
            result,
            Err(PropagationUncertaintyError::NonFinite { .. })
        ));
    }

    #[test]
    fn interval_validates_ordering() {
        let result = UncertaintyInterval::new(2.0, 1.0);

        assert!(matches!(
            result,
            Err(PropagationUncertaintyError::InvalidInterval { .. })
        ));
    }

    #[test]
    fn interval_contains_value() {
        let interval =
            UncertaintyInterval::new(-1.0, 2.0).expect("valid interval");

        assert!(interval.contains(0.0));
        assert!(interval.contains(-1.0));
        assert!(interval.contains(2.0));
        assert!(!interval.contains(3.0));
    }

    #[test]
    fn diagonal_covariance_is_constructed_correctly() {
        let covariance =
            CovarianceMatrix::from_standard_uncertainties(
                &[2.0, 3.0],
                PropagationLimits::unlimited(),
            )
            .expect("valid covariance");

        assert_eq!(covariance.dimension(), 2);
        assert_eq!(covariance.variance(0), Some(4.0));
        assert_eq!(covariance.variance(1), Some(9.0));
        assert_eq!(covariance.covariance(0, 1), Some(0.0));
    }

    #[test]
    fn independent_propagation_matches_linear_formula() {
        let result = propagate_independent(
            &[2.0, 3.0],
            &[0.5, 1.0],
            7.0,
            PropagationPolicy::AllowFirstOrderApproximation,
        )
        .expect("valid propagation");

        let expected = (4.0 * 0.25 + 9.0).sqrt();

        assert!((result.standard_uncertainty.unwrap() - expected).abs()
            < 1.0e-12);
    }

    #[test]
    fn exact_policy_rejects_first_order_propagation() {
        let result = propagate_independent(
            &[1.0],
            &[1.0],
            1.0,
            PropagationPolicy::ExactOrConservative,
        );

        assert!(matches!(
            result,
            Err(
                PropagationUncertaintyError::ApproximationNotAllowed {
                    ..
                }
            )
        ));
    }

    #[test]
    fn correlated_propagation_includes_covariance() {
        let covariance =
            CovarianceMatrix::from_elements(
                2,
                vec![
                    1.0, 0.5,
                    0.5, 1.0,
                ],
                DEFAULT_TOLERANCE,
                PropagationLimits::unlimited(),
            )
            .expect("valid covariance");

        let result = propagate_with_covariance(
            &[1.0, 1.0],
            &covariance,
            0.0,
            PropagationPolicy::AllowFirstOrderApproximation,
        )
        .expect("valid propagation");

        // variance = 1 + 1 + 2*0.5 = 3
        assert!((result.standard_uncertainty.unwrap() - 3.0_f64.sqrt()).abs()
            < 1.0e-12);
    }

    #[test]
    fn covariance_matrix_rejects_asymmetry() {
        let result = CovarianceMatrix::from_elements(
            2,
            vec![
                1.0, 0.2,
                0.3, 1.0,
            ],
            DEFAULT_TOLERANCE,
            PropagationLimits::unlimited(),
        );

        assert!(matches!(
            result,
            Err(
                PropagationUncertaintyError::CovarianceNotSymmetric {
                    ..
                }
            )
        ));
    }

    #[test]
    fn jacobian_validates_dimension() {
        let result = Jacobian::new(
            2,
            3,
            vec![1.0, 0.0, 0.0],
            PropagationLimits::unlimited(),
        );

        assert!(matches!(
            result,
            Err(PropagationUncertaintyError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn interval_addition_is_conservative() {
        let lhs =
            UncertaintyInterval::new(1.0, 2.0).expect("valid interval");
        let rhs =
            UncertaintyInterval::new(3.0, 5.0).expect("valid interval");

        let result = add_intervals(lhs, rhs).expect("valid propagation");

        assert_eq!(result.lower(), 4.0);
        assert_eq!(result.upper(), 7.0);
    }

    #[test]
    fn interval_subtraction_is_conservative() {
        let lhs =
            UncertaintyInterval::new(1.0, 2.0).expect("valid interval");
        let rhs =
            UncertaintyInterval::new(3.0, 5.0).expect("valid interval");

        let result =
            subtract_intervals(lhs, rhs).expect("valid propagation");

        assert_eq!(result.lower(), -4.0);
        assert_eq!(result.upper(), -1.0);
    }

    #[test]
    fn interval_multiplication_handles_sign_changes() {
        let lhs =
            UncertaintyInterval::new(-2.0, 3.0).expect("valid interval");
        let rhs =
            UncertaintyInterval::new(-4.0, 5.0).expect("valid interval");

        let result =
            multiply_intervals(lhs, rhs).expect("valid propagation");

        assert_eq!(result.lower(), -12.0);
        assert_eq!(result.upper(), 15.0);
    }

    #[test]
    fn interval_division_rejects_zero_denominator() {
        let numerator =
            UncertaintyInterval::new(1.0, 2.0).expect("valid interval");
        let denominator =
            UncertaintyInterval::new(-1.0, 1.0).expect("valid interval");

        let result = divide_intervals(numerator, denominator);

        assert!(matches!(
            result,
            Err(PropagationUncertaintyError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn affine_interval_preserves_order_for_negative_slope() {
        let input =
            UncertaintyInterval::new(1.0, 3.0).expect("valid interval");

        let result =
            affine_interval(input, -2.0, 4.0).expect("valid propagation");

        assert_eq!(result.lower(), -2.0);
        assert_eq!(result.upper(), 2.0);
    }

    #[test]
    fn resource_limit_is_policy_not_semantic_size_limit() {
        let limits = PropagationLimits {
            max_output_values: Some(2),
            max_matrix_elements: None,
        };

        let result = Jacobian::zeros(3, 1, limits);

        assert!(matches!(
            result,
            Err(
                PropagationUncertaintyError::ResourceLimitExceeded {
                    resource: "output_values",
                    ..
                }
            )
        ));
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            UNCERTAINTY_SCHEMA_ID,
            "zamani.quantum.zqn.propagation.uncertainty"
        );
        assert_eq!(UNCERTAINTY_SCHEMA_VERSION, 1);
    }
}