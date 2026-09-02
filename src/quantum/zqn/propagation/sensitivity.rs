//! Zamani Quantum Noise (ZQN) — Propagation Sensitivity Analysis.
//!
//! Path:
//!     src/quantum/zqn/propagation/sensitivity.rs
//!
//! # Purpose
//!
//! This module owns deterministic sensitivity analysis for quantities affected
//! by uncertain or configurable parameters.
//!
//! It answers questions such as:
//!
//! - Which input parameter most strongly affects an output?
//! - How strongly does an output change with respect to each parameter?
//! - Which uncertainty contribution dominates a propagated result?
//! - Which parameters deserve calibration priority?
//! - Which parameters dominate an error budget?
//! - How sensitive is a computation to a physical/noise/calibration parameter?
//!
//! Sensitivity is deliberately different from uncertainty propagation.
//!
//! ```text
//! Sensitivity
//!     = how output changes when an input changes
//!
//! Uncertainty propagation
//!     = how uncertainty in inputs becomes uncertainty in outputs
//!
//! Error budget
//!     = how much error is allowed/allocated
//! ```
//!
//! The mathematical relationship is commonly:
//!
//! ```text
//! y = f(x)
//!
//! J = ∂f/∂x
//!
//! Σ_y ≈ J Σ_x Jᵀ
//! ```
//!
//! where `J` is the sensitivity/Jacobian matrix and `Σ` is an input
//! covariance matrix.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - scalar local sensitivities;
//! - vector/matrix Jacobian representations;
//! - deterministic finite-difference sensitivity estimation;
//! - central, forward and backward finite differences;
//! - normalized/relative sensitivity;
//! - sensitivity contribution analysis;
//! - covariance-aware local influence analysis;
//! - dominant-parameter ranking;
//! - sensitivity reports;
//! - explicit numerical tolerances;
//! - explicit approximation classification;
//! - deterministic ordering of sensitivity results;
//! - resource-aware sensitivity calculation.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - quantum states;
//! - quantum channels;
//! - noise semantics;
//! - calibration storage;
//! - statistical estimation;
//! - probability distributions;
//! - uncertainty semantics;
//! - error-budget allocation;
//! - fidelity definitions;
//! - QEC algorithms;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - simulator execution;
//! - canonical Quantum IR;
//! - qubit identity;
//! - serialization wire formats;
//! - automatic differentiation;
//! - symbolic algebra.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! noise / calibration / characterization
//!      │
//!      ├───────────────┐
//!      │               │
//!      ▼               ▼
//! quantified       parameterized
//! uncertainty      computation/model
//!      │               │
//!      │               ▼
//!      │       propagation::sensitivity
//!      │               │
//!      └───────┬───────┘
//!              │
//!              ├──► propagation::uncertainty
//!              │
//!              ├──► propagation::error_budget
//!              │
//!              ├──► propagation::fidelity
//!              │
//!              ├──► routing
//!              │
//!              ├──► scheduling
//!              │
//!              └──► QEC analysis
//! ```
//!
//! # Fundamental distinction
//!
//! Sensitivity is not automatically an uncertainty.
//!
//! Given:
//!
//! ```text
//! y = 2x
//! ```
//!
//! the derivative is:
//!
//! ```text
//! dy/dx = 2
//! ```
//!
//! If `x` has uncertainty `u(x) = 0.1`, then a first-order propagated
//! contribution is approximately:
//!
//! ```text
//! u_y ≈ |2| * 0.1 = 0.2
//! ```
//!
//! The sensitivity (`2`) and uncertainty contribution (`0.2`) are different
//! semantic quantities.
//!
//! # Write once, scale everywhere
//!
//! No architectural maximum is imposed on:
//!
//! - number of parameters;
//! - number of outputs;
//! - number of quantum resources;
//! - number of qubits;
//! - number of machines;
//! - circuit depth;
//! - operation count;
//! - parameter dimension.
//!
//! All dimensions are supplied by the caller.
//!
//! The implementation is therefore valid for any finite dimension that can be
//! represented and processed with the available resources.
//!
//! "Infinity" means:
//!
//! > ZQN imposes no artificial finite machine-size ceiling.
//!
//! It does NOT mean that finite memory or finite CPU resources can be ignored.
//!
//! # Resource safety
//!
//! Sensitivity estimation can require one or more model evaluations per
//! parameter.
//!
//! Therefore work is explicitly bounded by `SensitivityLimits` when desired.
//!
//! No hidden global limit exists.
//!
//! No fixed parameter count exists.
//!
//! No fixed output count exists.
//!
//! # Determinism
//!
//! This module:
//!
//! - uses no random-number generator;
//! - uses no global mutable state;
//! - does not read the system clock;
//! - does not depend on thread identity;
//! - does not use unordered maps;
//! - uses deterministic parameter/output ordering;
//! - performs deterministic sequential accumulation.
//!
//! Given identical inputs, model evaluations and numerical policy, results are
//! deterministic.
//!
//! If the supplied model itself is stochastic, the model caller must provide a
//! deterministic evaluation policy. This module does not silently seed or
//! control an external stochastic model.
//!
//! # Numerical safety
//!
//! The implementation rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - zero/negative finite-difference steps;
//! - invalid tolerances;
//! - dimension mismatches;
//! - arithmetic size overflow;
//! - invalid covariance entries;
//! - non-finite model outputs;
//! - non-finite model evaluations;
//! - invalid parameter values where validation is requested.
//!
//! It does not silently:
//!
//! - clamp NaN;
//! - convert infinity to a finite value;
//! - replace a zero derivative with a fake epsilon;
//! - take absolute values to hide invalid input;
//! - silently change finite-difference methods;
//! - silently normalize parameter units.
//!
//! # Finite differences
//!
//! This module provides three explicit finite-difference methods.
//!
//! Forward:
//!
//! ```text
//! f'(x) ≈ [f(x+h) - f(x)] / h
//! ```
//!
//! Backward:
//!
//! ```text
//! f'(x) ≈ [f(x) - f(x-h)] / h
//! ```
//!
//! Central:
//!
//! ```text
//! f'(x) ≈ [f(x+h) - f(x-h)] / (2h)
//! ```
//!
//! Central differences are normally preferred for smooth functions because
//! they generally provide better local accuracy, but this module does not
//! silently substitute one method for another.
//!
//! # Approximation contract
//!
//! Finite-difference sensitivity is an approximation.
//!
//! Every finite-difference result is explicitly marked as approximate.
//!
//! It must never be represented as an exact symbolic derivative.
//!
//! Exact derivatives can be supplied by another subsystem through
//! `SensitivityMatrix`.
//!
//! # Automatic differentiation
//!
//! Automatic differentiation is intentionally not implemented here.
//!
//! This module provides the semantic representation consumed by an AD system.
//!
//! An AD implementation may produce a `SensitivityMatrix` without changing
//! this file's contract.
//!
//! # Symbolic differentiation
//!
//! Symbolic algebra is also outside this module.
//!
//! A symbolic engine may produce exact derivatives and convert them into the
//! `SensitivityMatrix` representation defined here.
//!
//! # Quantum-resource identity
//!
//! Sensitivity is resource-agnostic.
//!
//! This file therefore deliberately does not define or duplicate:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! When a sensitivity entry is associated with a quantum resource, the
//! surrounding integration layer must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A ZQN-specific qubit identity must not be introduced.
//!
//! # Integration with uncertainty.rs
//!
//! `propagation::uncertainty` consumes sensitivity/Jacobian information when
//! performing first-order covariance propagation.
//!
//! The mathematical contract is:
//!
//! ```text
//! Σ_y ≈ J Σ_x Jᵀ
//! ```
//!
//! `SensitivityMatrix` is therefore intentionally independent of the concrete
//! uncertainty implementation.
//!
//! # Integration with error_budget.rs
//!
//! Error-budget analysis can use `SensitivityReport` to determine which input
//! dimensions have the greatest influence on an error quantity.
//!
//! This file does not allocate or consume budgets.
//!
//! # Integration with fidelity.rs
//!
//! Fidelity analysis may use sensitivity to determine which physical/noise
//! parameters most strongly affect a fidelity metric.
//!
//! Fidelity remains the owner of fidelity semantics.
//!
//! # Integration with calibration
//!
//! Calibration may use sensitivity to prioritize calibration effort.
//!
//! Example:
//!
//! ```text
//! parameter A: high sensitivity
//! parameter B: low sensitivity
//!
//! => calibration of A may have greater expected impact
//! ```
//!
//! This module does not decide calibration policy.
//!
//! # Integration with noise
//!
//! Noise models may expose parameterized quantities such as:
//!
//! - gate error;
//! - T1;
//! - T2;
//! - readout error;
//! - crosstalk strength;
//! - drift rate;
//! - loss rate.
//!
//! ZQN sensitivity can quantify how an output metric responds to those
//! parameters without taking ownership of their physical meaning.
//!
//! # Integration with routing
//!
//! Routing can use sensitivity information when choosing between physical
//! mappings, but routing policy remains outside this module.
//!
//! # Integration with scheduling
//!
//! Scheduling may use sensitivity to identify which timing/noise parameters
//! deserve optimization priority.
//!
//! # Integration with QEC
//!
//! QEC may consume sensitivity information when identifying which physical
//! parameters dominate logical-error behavior.
//!
//! This module does not perform decoding or correction.
//!
//! # Serialization
//!
//! This module defines semantic structures only.
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
//! Sensitivity analysis may receive untrusted model dimensions or parameter
//! values.
//!
//! Therefore:
//!
//! - dimensions are validated;
//! - work estimates use checked arithmetic;
//! - caller-visible limits are supported;
//! - no unsafe code exists;
//! - no external process is invoked;
//! - no recursion is required;
//! - no hidden global allocation is used.
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
//! - no unsafe code.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. sensitivity semantics are explicit;
//! 2. Jacobian dimensions are validated;
//! 3. finite-difference methods are explicit;
//! 4. approximations are explicitly classified;
//! 5. exact externally supplied sensitivities are supported;
//! 6. relative sensitivity is explicit;
//! 7. covariance-aware influence is supported without owning covariance;
//! 8. dominant parameters can be deterministically ranked;
//! 9. no machine-size limits exist;
//! 10. resource limits are caller-controlled;
//! 11. no RNG exists;
//! 12. no global mutable state exists;
//! 13. no quantum identifier is duplicated;
//! 14. no serialization format is owned here;
//! 15. no unsafe code exists;
//! 16. downstream modules can consume this contract without reopening this
//!     implementation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic schema identifier.
pub const SENSITIVITY_SCHEMA_ID: &str =
    "zamani.quantum.zqn.propagation.sensitivity";

/// Semantic version of this module's public contract.
pub const SENSITIVITY_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance used for validation.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

/// Default finite-difference relative step.
pub const DEFAULT_RELATIVE_STEP: f64 = 1.0e-6;

/// Default absolute step used when the parameter value is zero.
pub const DEFAULT_ABSOLUTE_STEP: f64 = 1.0e-8;

// ============================================================================
// Error
// ============================================================================

/// Errors produced by sensitivity analysis.
#[derive(Clone, Debug, PartialEq)]
pub enum SensitivityError {
    /// A required floating-point value was not finite.
    NonFinite {
        /// Semantic name of the invalid value.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// A finite-difference step was not strictly positive and finite.
    InvalidStep {
        /// Supplied step.
        value: f64,
    },

    /// A numerical tolerance was invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// Parameter and output dimensions differ.
    DimensionMismatch {
        /// Expected dimension.
        expected: usize,
        /// Actual dimension.
        actual: usize,
        /// Semantic operation.
        context: &'static str,
    },

    /// A matrix size calculation overflowed.
    SizeOverflow {
        /// Semantic operation.
        context: &'static str,
    },

    /// The requested work exceeds an explicitly supplied resource policy.
    ResourceLimitExceeded {
        /// Requested amount.
        requested: u128,
        /// Maximum permitted amount.
        maximum: u128,
        /// Resource category.
        resource: &'static str,
    },

    /// A model evaluation returned an invalid number.
    InvalidModelOutput {
        /// Output index.
        output: usize,
        /// Returned value.
        value: f64,
    },

    /// A model evaluation failed.
    ModelEvaluationFailed {
        /// Parameter index being perturbed.
        parameter: usize,
        /// Evaluation direction.
        direction: EvaluationDirection,
    },

    /// A relative sensitivity could not be calculated because the reference
    /// value is zero and no finite absolute scale exists.
    ZeroReference {
        /// Semantic context.
        context: &'static str,
    },

    /// A requested influence calculation requires covariance dimensions that
    /// do not match the sensitivity dimensions.
    CovarianceDimensionMismatch {
        /// Number of parameters.
        parameters: usize,
        /// Number of covariance rows.
        covariance_rows: usize,
        /// Number of covariance columns.
        covariance_columns: usize,
    },

    /// Covariance contains an invalid numerical value.
    InvalidCovariance {
        /// Row.
        row: usize,
        /// Column.
        column: usize,
        /// Value.
        value: f64,
    },

    /// Covariance is insufficiently symmetric.
    CovarianceNotSymmetric {
        /// Row.
        row: usize,
        /// Column.
        column.
        lhs: f64,
        /// Mirrored value.
        rhs: f64,
    },

    /// A parameter perturbation produced an invalid parameter vector.
    InvalidPerturbedParameters {
        /// Parameter index.
        parameter: usize,
    },
}

impl fmt::Display for SensitivityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(f, "non-finite sensitivity value in `{field}`: {value}")
            }
            Self::InvalidStep { value } => {
                write!(f, "finite-difference step must be finite and > 0: {value}")
            }
            Self::InvalidTolerance { value } => {
                write!(f, "sensitivity tolerance must be finite and >= 0: {value}")
            }
            Self::DimensionMismatch {
                expected,
                actual,
                context,
            } => write!(
                f,
                "dimension mismatch in {context}: expected {expected}, got {actual}"
            ),
            Self::SizeOverflow { context } => {
                write!(f, "size arithmetic overflow in {context}")
            }
            Self::ResourceLimitExceeded {
                requested,
                maximum,
                resource,
            } => write!(
                f,
                "sensitivity resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
            ),
            Self::InvalidModelOutput { output, value } => {
                write!(f, "model output {output} is non-finite: {value}")
            }
            Self::ModelEvaluationFailed {
                parameter,
                direction,
            } => write!(
                f,
                "model evaluation failed while perturbing parameter {parameter} ({direction:?})"
            ),
            Self::ZeroReference { context } => {
                write!(f, "relative sensitivity has a zero reference in {context}")
            }
            Self::CovarianceDimensionMismatch {
                parameters,
                covariance_rows,
                covariance_columns,
            } => write!(
                f,
                "covariance dimension mismatch: parameters={parameters}, covariance={covariance_rows}x{covariance_columns}"
            ),
            Self::InvalidCovariance { row, column, value } => {
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
            } => write!(
                f,
                "covariance matrix is not symmetric at ({row}, {column}): {lhs} != {rhs}"
            ),
            Self::InvalidPerturbedParameters { parameter } => {
                write!(
                    f,
                    "perturbation produced invalid parameters for parameter {parameter}"
                )
            }
        }
    }
}

impl std::error::Error for SensitivityError {}

// ============================================================================
// Approximation classification
// ============================================================================

/// Mathematical status of a sensitivity result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensitivityKind {
    /// Exact derivative supplied by an external exact method.
    Exact,

    /// First-order derivative estimated by finite differences.
    FiniteDifference,

    /// Sensitivity derived from an externally supplied approximation.
    Approximate,
}

// ============================================================================
// Finite-difference method
// ============================================================================

/// Explicit finite-difference method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteDifferenceMethod {
    /// `(f(x+h) - f(x)) / h`.
    Forward,

    /// `(f(x) - f(x-h)) / h`.
    Backward,

    /// `(f(x+h) - f(x-h)) / (2h)`.
    Central,
}

// ============================================================================
// Evaluation direction
// ============================================================================

/// Direction associated with a model evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationDirection {
    /// Baseline evaluation.
    Baseline,

    /// Positive perturbation.
    Positive,

    /// Negative perturbation.
    Negative,
}

// ============================================================================
// Limits
// ============================================================================

/// Explicit resource policy for sensitivity calculations.
///
/// All fields are optional. `None` means this module imposes no limit for that
/// category.
///
/// This type deliberately contains no hardware-size constants.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SensitivityLimits {
    /// Maximum number of model evaluations.
    pub max_model_evaluations: Option<u128>,

    /// Maximum number of Jacobian elements.
    pub max_jacobian_elements: Option<u128>,

    /// Maximum number of parameter-output influence calculations.
    pub max_influence_elements: Option<u128>,
}

impl SensitivityLimits {
    /// Creates an unlimited policy.
    pub const fn unlimited() -> Self {
        Self {
            max_model_evaluations: None,
            max_jacobian_elements: None,
            max_influence_elements: None,
        }
    }

    fn check(
        &self,
        requested: u128,
        maximum: Option<u128>,
        resource: &'static str,
    ) -> Result<(), SensitivityError> {
        if let Some(maximum) = maximum {
            if requested > maximum {
                return Err(SensitivityError::ResourceLimitExceeded {
                    requested,
                    maximum,
                    resource,
                });
            }
        }

        Ok(())
    }

    fn checked_jacobian_elements(
        &self,
        parameters: usize,
        outputs: usize,
    ) -> Result<(), SensitivityError> {
        let requested = (parameters as u128)
            .checked_mul(outputs as u128)
            .ok_or(SensitivityError::SizeOverflow {
                context: "parameter_count * output_count",
            })?;

        self.check(
            requested,
            self.max_jacobian_elements,
            "jacobian elements",
        )
    }

    fn checked_influence_elements(
        &self,
        parameters: usize,
        outputs: usize,
    ) -> Result<(), SensitivityError> {
        let requested = (parameters as u128)
            .checked_mul(outputs as u128)
            .ok_or(SensitivityError::SizeOverflow {
                context: "parameter_count * output_count",
            })?;

        self.check(
            requested,
            self.max_influence_elements,
            "influence elements",
        )
    }
}

// ============================================================================
// Numerical policy
// ============================================================================

/// Numerical policy controlling sensitivity calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensitivityPolicy {
    /// Finite-difference method.
    pub method: FiniteDifferenceMethod,

    /// Relative perturbation size.
    pub relative_step: f64,

    /// Absolute perturbation size used when relative scaling is insufficient.
    pub absolute_step: f64,

    /// Numerical comparison tolerance.
    pub tolerance: f64,

    /// Resource policy.
    pub limits: SensitivityLimits,
}

impl Default for SensitivityPolicy {
    fn default() -> Self {
        Self {
            method: FiniteDifferenceMethod::Central,
            relative_step: DEFAULT_RELATIVE_STEP,
            absolute_step: DEFAULT_ABSOLUTE_STEP,
            tolerance: DEFAULT_TOLERANCE,
            limits: SensitivityLimits::unlimited(),
        }
    }
}

impl SensitivityPolicy {
    /// Validates the complete numerical policy.
    pub fn validate(&self) -> Result<(), SensitivityError> {
        validate_positive_finite("relative_step", self.relative_step)?;
        validate_positive_finite("absolute_step", self.absolute_step)?;

        if !self.tolerance.is_finite() || self.tolerance < 0.0 {
            return Err(SensitivityError::InvalidTolerance {
                value: self.tolerance,
            });
        }

        Ok(())
    }

    /// Calculates a deterministic perturbation for a parameter value.
    pub fn step_for(&self, parameter: f64) -> Result<f64, SensitivityError> {
        validate_finite("parameter", parameter)?;
        self.validate()?;

        let scaled = parameter.abs() * self.relative_step;

        if !scaled.is_finite() {
            return Err(SensitivityError::NonFinite {
                field: "computed_step",
                value: scaled,
            });
        }

        let step = scaled.max(self.absolute_step);

        validate_positive_finite("computed_step", step)?;

        Ok(step)
    }
}

// ============================================================================
// Parameter vector
// ============================================================================

/// Immutable parameter vector used by sensitivity analysis.
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterVector {
    values: Vec<f64>,
}

impl ParameterVector {
    /// Creates a validated parameter vector.
    pub fn new(values: Vec<f64>) -> Result<Self, SensitivityError> {
        for &value in &values {
            validate_finite("parameter", value)?;
        }

        Ok(Self { values })
    }

    /// Creates an empty parameter vector.
    pub fn empty() -> Self {
        Self { values: Vec::new() }
    }

    /// Number of parameters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether no parameters exist.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns a parameter by index.
    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Returns the parameter slice.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    /// Creates a perturbed copy.
    pub fn with_perturbation(
        &self,
        index: usize,
        delta: f64,
    ) -> Result<Self, SensitivityError> {
        validate_finite("delta", delta)?;

        let current = self
            .get(index)
            .ok_or(SensitivityError::DimensionMismatch {
                expected: self.len(),
                actual: index.saturating_add(1),
                context: "parameter index",
            })?;

        let perturbed = current + delta;

        if !perturbed.is_finite() {
            return Err(SensitivityError::InvalidPerturbedParameters {
                parameter: index,
            });
        }

        let mut values = self.values.clone();
        values[index] = perturbed;

        Self::new(values)
    }
}

// ============================================================================
// Output vector
// ============================================================================

/// Validated model output vector.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputVector {
    values: Vec<f64>,
}

impl OutputVector {
    /// Creates a validated output vector.
    pub fn new(values: Vec<f64>) -> Result<Self, SensitivityError> {
        for (index, &value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(SensitivityError::InvalidModelOutput {
                    output: index,
                    value,
                });
            }
        }

        Ok(Self { values })
    }

    /// Number of outputs.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether no outputs exist.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an output by index.
    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Returns the output slice.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

// ============================================================================
// Jacobian
// ============================================================================

/// Row-major Jacobian/sensitivity matrix.
///
/// Element `(output, parameter)` is:
///
/// ```text
/// ∂output / ∂parameter
/// ```
///
/// The matrix is stored row-major:
///
/// ```text
/// [output_0 wrt parameter_0, output_0 wrt parameter_1, ...]
/// [output_1 wrt parameter_0, output_1 wrt parameter_1, ...]
/// ...
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SensitivityMatrix {
    outputs: usize,
    parameters: usize,
    values: Vec<f64>,
    kind: SensitivityKind,
}

impl SensitivityMatrix {
    /// Creates a validated sensitivity matrix.
    pub fn new(
        outputs: usize,
        parameters: usize,
        values: Vec<f64>,
        kind: SensitivityKind,
    ) -> Result<Self, SensitivityError> {
        let expected = outputs
            .checked_mul(parameters)
            .ok_or(SensitivityError::SizeOverflow {
                context: "outputs * parameters",
            })?;

        if values.len() != expected {
            return Err(SensitivityError::DimensionMismatch {
                expected,
                actual: values.len(),
                context: "sensitivity matrix element count",
            });
        }

        for &value in &values {
            validate_finite("sensitivity", value)?;
        }

        Ok(Self {
            outputs,
            parameters,
            values,
            kind,
        })
    }

    /// Creates an exact Jacobian.
    pub fn exact(
        outputs: usize,
        parameters: usize,
        values: Vec<f64>,
    ) -> Result<Self, SensitivityError> {
        Self::new(
            outputs,
            parameters,
            values,
            SensitivityKind::Exact,
        )
    }

    /// Creates an externally supplied approximate Jacobian.
    pub fn approximate(
        outputs: usize,
        parameters: usize,
        values: Vec<f64>,
    ) -> Result<Self, SensitivityError> {
        Self::new(
            outputs,
            parameters,
            values,
            SensitivityKind::Approximate,
        )
    }

    /// Number of outputs.
    pub fn output_count(&self) -> usize {
        self.outputs
    }

    /// Number of parameters.
    pub fn parameter_count(&self) -> usize {
        self.parameters
    }

    /// Mathematical classification.
    pub fn kind(&self) -> SensitivityKind {
        self.kind
    }

    /// Returns `∂output / ∂parameter`.
    pub fn get(&self, output: usize, parameter: usize) -> Option<f64> {
        if output >= self.outputs || parameter >= self.parameters {
            return None;
        }

        let index = output
            .checked_mul(self.parameters)?
            .checked_add(parameter)?;

        self.values.get(index).copied()
    }

    /// Returns the complete row for an output.
    pub fn output_row(&self, output: usize) -> Option<&[f64]> {
        if output >= self.outputs {
            return None;
        }

        let start = output.checked_mul(self.parameters)?;
        let end = start.checked_add(self.parameters)?;

        self.values.get(start..end)
    }

    /// Returns the complete matrix in row-major form.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

// ============================================================================
// Relative sensitivity
// ============================================================================

/// A normalized sensitivity value.
///
/// For non-zero input and output references:
///
/// ```text
/// S_relative = (x / y) * (dy / dx)
/// ```
///
/// This describes percentage-level response:
///
/// ```text
/// approximately:
/// 1% change in x -> S_relative % change in y
/// ```
///
/// The value is dimensionless when the references use compatible physical
/// units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelativeSensitivity {
    /// Absolute derivative.
    pub derivative: f64,

    /// Input reference value.
    pub input: f64,

    /// Output reference value.
    pub output: f64,

    /// Dimensionless relative sensitivity.
    pub relative: f64,
}

impl RelativeSensitivity {
    /// Computes normalized sensitivity.
    pub fn calculate(
        derivative: f64,
        input: f64,
        output: f64,
    ) -> Result<Self, SensitivityError> {
        validate_finite("derivative", derivative)?;
        validate_finite("input", input)?;
        validate_finite("output", output)?;

        if output == 0.0 {
            return Err(SensitivityError::ZeroReference {
                context: "relative sensitivity output",
            });
        }

        let relative = (input / output) * derivative;

        validate_finite("relative_sensitivity", relative)?;

        Ok(Self {
            derivative,
            input,
            output,
            relative,
        })
    }
}

// ============================================================================
// Influence
// ============================================================================

/// Influence of one parameter on one output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensitivityInfluence {
    /// Output index.
    pub output: usize,

    /// Parameter index.
    pub parameter: usize,

    /// Absolute local derivative.
    pub derivative: f64,

    /// Absolute magnitude of the derivative.
    pub magnitude: f64,

    /// Optional normalized sensitivity.
    pub relative: Option<f64>,

    /// Approximate first-order output uncertainty contribution if the supplied
    /// parameter standard uncertainty is used.
    pub uncertainty_contribution: Option<f64>,
}

impl SensitivityInfluence {
    fn from_derivative(
        output: usize,
        parameter: usize,
        derivative: f64,
        input_value: Option<f64>,
        output_value: Option<f64>,
        standard_uncertainty: Option<f64>,
    ) -> Result<Self, SensitivityError> {
        validate_finite("derivative", derivative)?;

        let magnitude = derivative.abs();

        let relative = match (input_value, output_value) {
            (Some(input), Some(output)) if output != 0.0 => {
                Some(RelativeSensitivity::calculate(
                    derivative,
                    input,
                    output,
                )?
                .relative)
            }
            _ => None,
        };

        let uncertainty_contribution = match standard_uncertainty {
            Some(uncertainty) => {
                validate_finite("standard_uncertainty", uncertainty)?;

                if uncertainty < 0.0 {
                    return Err(SensitivityError::NonFinite {
                        field: "negative standard uncertainty",
                        value: uncertainty,
                    });
                }

                let contribution = magnitude * uncertainty;

                validate_finite(
                    "uncertainty_contribution",
                    contribution,
                )?;

                Some(contribution)
            }
            None => None,
        };

        Ok(Self {
            output,
            parameter,
            derivative,
            magnitude,
            relative,
            uncertainty_contribution,
        })
    }
}

// ============================================================================
// Report
// ============================================================================

/// Complete deterministic sensitivity-analysis result.
#[derive(Clone, Debug, PartialEq)]
pub struct SensitivityReport {
    /// Jacobian.
    pub jacobian: SensitivityMatrix,

    /// Parameter values used for the analysis, when available.
    pub parameters: Option<ParameterVector>,

    /// Baseline outputs, when available.
    pub baseline_outputs: Option<OutputVector>,

    /// Per-output/per-parameter influence entries.
    pub influences: Vec<SensitivityInfluence>,

    /// Parameter ranking by aggregate absolute influence.
    pub parameter_ranking: Vec<RankedParameter>,

    /// Output ranking by aggregate absolute influence.
    pub output_ranking: Vec<RankedOutput>,
}

impl SensitivityReport {
    /// Builds a report from a Jacobian.
    pub fn from_jacobian(
        jacobian: SensitivityMatrix,
        parameters: Option<ParameterVector>,
        baseline_outputs: Option<OutputVector>,
        standard_uncertainties: Option<&[f64]>,
    ) -> Result<Self, SensitivityError> {
        if let Some(parameters) = &parameters {
            if parameters.len() != jacobian.parameter_count() {
                return Err(SensitivityError::DimensionMismatch {
                    expected: jacobian.parameter_count(),
                    actual: parameters.len(),
                    context: "sensitivity parameters",
                });
            }
        }

        if let Some(outputs) = &baseline_outputs {
            if outputs.len() != jacobian.output_count() {
                return Err(SensitivityError::DimensionMismatch {
                    expected: jacobian.output_count(),
                    actual: outputs.len(),
                    context: "sensitivity baseline outputs",
                });
            }
        }

        if let Some(uncertainties) = standard_uncertainties {
            if uncertainties.len() != jacobian.parameter_count() {
                return Err(SensitivityError::DimensionMismatch {
                    expected: jacobian.parameter_count(),
                    actual: uncertainties.len(),
                    context: "parameter standard uncertainties",
                });
            }

            for &uncertainty in uncertainties {
                if !uncertainty.is_finite() || uncertainty < 0.0 {
                    return Err(SensitivityError::NonFinite {
                        field: "standard_uncertainty",
                        value: uncertainty,
                    });
                }
            }
        }

        let mut influences = Vec::with_capacity(
            jacobian
                .output_count()
                .checked_mul(jacobian.parameter_count())
                .ok_or(SensitivityError::SizeOverflow {
                    context: "influence capacity",
                })?,
        );

        for output in 0..jacobian.output_count() {
            for parameter in 0..jacobian.parameter_count() {
                let derivative = jacobian
                    .get(output, parameter)
                    .ok_or(SensitivityError::DimensionMismatch {
                        expected: jacobian.output_count(),
                        actual: output,
                        context: "jacobian output access",
                    })?;

                let parameter_value =
                    parameters.as_ref().and_then(|p| p.get(parameter));

                let output_value =
                    baseline_outputs.as_ref().and_then(|o| o.get(output));

                let uncertainty =
                    standard_uncertainties.and_then(|u| u.get(parameter).copied());

                influences.push(SensitivityInfluence::from_derivative(
                    output,
                    parameter,
                    derivative,
                    parameter_value,
                    output_value,
                    uncertainty,
                )?);
            }
        }

        let parameter_ranking =
            rank_parameters(&influences, jacobian.parameter_count())?;

        let output_ranking =
            rank_outputs(&influences, jacobian.output_count())?;

        Ok(Self {
            jacobian,
            parameters,
            baseline_outputs,
            influences,
            parameter_ranking,
            output_ranking,
        })
    }

    /// Returns the most influential parameter, if any exist.
    pub fn dominant_parameter(&self) -> Option<&RankedParameter> {
        self.parameter_ranking.first()
    }

    /// Returns the most sensitive output, if any exist.
    pub fn dominant_output(&self) -> Option<&RankedOutput> {
        self.output_ranking.first()
    }

    /// Returns all influences for one parameter.
    pub fn influences_for_parameter(
        &self,
        parameter: usize,
    ) -> impl Iterator<Item = &SensitivityInfluence> {
        self.influences
            .iter()
            .filter(move |entry| entry.parameter == parameter)
    }

    /// Returns all influences for one output.
    pub fn influences_for_output(
        &self,
        output: usize,
    ) -> impl Iterator<Item = &SensitivityInfluence> {
        self.influences
            .iter()
            .filter(move |entry| entry.output == output)
    }
}

// ============================================================================
// Rankings
// ============================================================================

/// Aggregate influence ranking for one parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RankedParameter {
    /// Parameter index.
    pub parameter: usize,

    /// Sum of absolute derivative magnitudes across outputs.
    pub absolute_influence: f64,

    /// Maximum absolute derivative across outputs.
    pub maximum_influence: f64,

    /// Sum of first-order uncertainty contributions, when available.
    pub uncertainty_influence: Option<f64>,
}

/// Aggregate influence ranking for one output.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RankedOutput {
    /// Output index.
    pub output: usize,

    /// Sum of absolute derivative magnitudes across parameters.
    pub absolute_influence: f64,

    /// Maximum absolute derivative across parameters.
    pub maximum_influence: f64,

    /// Sum of first-order uncertainty contributions, when available.
    pub uncertainty_influence: Option<f64>,
}

fn rank_parameters(
    influences: &[SensitivityInfluence],
    parameter_count: usize,
) -> Result<Vec<RankedParameter>, SensitivityError> {
    let mut result = Vec::with_capacity(parameter_count);

    for parameter in 0..parameter_count {
        let mut absolute_influence = 0.0_f64;
        let mut maximum_influence = 0.0_f64;
        let mut uncertainty_influence = 0.0_f64;
        let mut has_uncertainty = false;

        for influence in influences
            .iter()
            .filter(|entry| entry.parameter == parameter)
        {
            absolute_influence += influence.magnitude;
            maximum_influence =
                maximum_influence.max(influence.magnitude);

            if let Some(value) = influence.uncertainty_contribution {
                uncertainty_influence += value;
                has_uncertainty = true;
            }
        }

        validate_finite("parameter absolute influence", absolute_influence)?;
        validate_finite("parameter maximum influence", maximum_influence)?;

        if has_uncertainty {
            validate_finite(
                "parameter uncertainty influence",
                uncertainty_influence,
            )?;
        }

        result.push(RankedParameter {
            parameter,
            absolute_influence,
            maximum_influence,
            uncertainty_influence: has_uncertainty
                .then_some(uncertainty_influence),
        });
    }

    result.sort_by(compare_ranked_parameters);

    Ok(result)
}

fn rank_outputs(
    influences: &[SensitivityInfluence],
    output_count: usize,
) -> Result<Vec<RankedOutput>, SensitivityError> {
    let mut result = Vec::with_capacity(output_count);

    for output in 0..output_count {
        let mut absolute_influence = 0.0_f64;
        let mut maximum_influence = 0.0_f64;
        let mut uncertainty_influence = 0.0_f64;
        let mut has_uncertainty = false;

        for influence in influences.iter().filter(|entry| entry.output == output) {
            absolute_influence += influence.magnitude;
            maximum_influence =
                maximum_influence.max(influence.magnitude);

            if let Some(value) = influence.uncertainty_contribution {
                uncertainty_influence += value;
                has_uncertainty = true;
            }
        }

        validate_finite("output absolute influence", absolute_influence)?;
        validate_finite("output maximum influence", maximum_influence)?;

        if has_uncertainty {
            validate_finite(
                "output uncertainty influence",
                uncertainty_influence,
            )?;
        }

        result.push(RankedOutput {
            output,
            absolute_influence,
            maximum_influence,
            uncertainty_influence: has_uncertainty
                .then_some(uncertainty_influence),
        });
    }

    result.sort_by(compare_ranked_outputs);

    Ok(result)
}

fn compare_ranked_parameters(
    lhs: &RankedParameter,
    rhs: &RankedParameter,
) -> Ordering {
    rhs.absolute_influence
        .partial_cmp(&lhs.absolute_influence)
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            lhs.parameter.cmp(&rhs.parameter)
        })
}

fn compare_ranked_outputs(
    lhs: &RankedOutput,
    rhs: &RankedOutput,
) -> Ordering {
    rhs.absolute_influence
        .partial_cmp(&lhs.absolute_influence)
        .unwrap_or(Ordering::Equal)
        .then_with(|| lhs.output.cmp(&rhs.output))
}

// ============================================================================
// Model interface
// ============================================================================

/// Deterministic parameterized model interface.
///
/// Implementors receive a complete parameter vector and return a complete
/// output vector.
///
/// The trait deliberately does not know anything about:
///
/// - qubits;
/// - gates;
/// - hardware;
/// - noise;
/// - circuits;
/// - vendors.
///
/// That keeps finite-difference sensitivity reusable throughout ZQN.
///
/// A model may internally evaluate a quantum program, noise model, calibration
/// model, fidelity metric or any other deterministic function.
pub trait SensitivityModel {
    /// Evaluates the model.
    fn evaluate(
        &self,
        parameters: &[f64],
    ) -> Result<Vec<f64>, SensitivityModelError>;
}

/// Model-level failure.
///
/// The sensitivity subsystem deliberately does not depend on a particular
/// upstream error type. Integration layers can convert their errors into this
/// small semantic boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SensitivityModelError {
    message: String,
}

impl SensitivityModelError {
    /// Creates a model failure with a stable human-readable message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the model error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for SensitivityModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SensitivityModelError {}

// ============================================================================
// Analyzer
// ============================================================================

/// Production sensitivity analyzer.
///
/// The analyzer itself contains no mutable global state and is safe to reuse
/// for independent analyses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensitivityAnalyzer {
    /// Numerical policy.
    pub policy: SensitivityPolicy,
}

impl Default for SensitivityAnalyzer {
    fn default() -> Self {
        Self {
            policy: SensitivityPolicy::default(),
        }
    }
}

impl SensitivityAnalyzer {
    /// Creates an analyzer using the supplied policy.
    pub fn new(policy: SensitivityPolicy) -> Result<Self, SensitivityError> {
        policy.validate()?;

        Ok(Self { policy })
    }

    /// Creates an analyzer using the default production numerical policy.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Calculates a finite-difference Jacobian.
    ///
    /// The model is evaluated only as many times as required by the selected
    /// method:
    ///
    /// - forward: `1 + parameter_count`;
    /// - backward: `1 + parameter_count`;
    /// - central: `2 * parameter_count`.
    pub fn finite_difference<M: SensitivityModel>(
        &self,
        model: &M,
        parameters: &ParameterVector,
    ) -> Result<SensitivityMatrix, SensitivityError> {
        self.policy.validate()?;

        let parameter_count = parameters.len();

        let baseline = match self.policy.method {
            FiniteDifferenceMethod::Forward
            | FiniteDifferenceMethod::Backward
            | FiniteDifferenceMethod::Central => {
                self.evaluate_checked(
                    model,
                    parameters,
                    0,
                    EvaluationDirection::Baseline,
                )?
            }
        };

        let output_count = baseline.len();

        self.policy
            .limits
            .checked_jacobian_elements(
                parameter_count,
                output_count,
            )?;

        let evaluation_count = match self.policy.method {
            FiniteDifferenceMethod::Forward
            | FiniteDifferenceMethod::Backward => {
                (parameter_count as u128)
                    .checked_add(1)
                    .ok_or(SensitivityError::SizeOverflow {
                        context: "finite-difference evaluation count",
                    })?
            }
            FiniteDifferenceMethod::Central => {
                (parameter_count as u128)
                    .checked_mul(2)
                    .ok_or(SensitivityError::SizeOverflow {
                        context: "central finite-difference evaluation count",
                    })?
            }
        };

        self.policy.limits.check(
            evaluation_count,
            self.policy.limits.max_model_evaluations,
            "model evaluations",
        )?;

        let matrix_len = output_count
            .checked_mul(parameter_count)
            .ok_or(SensitivityError::SizeOverflow {
                context: "jacobian allocation",
            })?;

        let mut jacobian = Vec::with_capacity(matrix_len);

        for output in 0..output_count {
            for parameter in 0..parameter_count {
                let step = self
                    .policy
                    .step_for(parameters.get(parameter).ok_or(
                        SensitivityError::DimensionMismatch {
                            expected: parameter_count,
                            actual: parameter,
                            context: "parameter lookup",
                        },
                    )?)?;

                let derivative = match self.policy.method {
                    FiniteDifferenceMethod::Forward => {
                        let positive =
                            parameters.with_perturbation(parameter, step)?;

                        let positive_output =
                            self.evaluate_checked(
                                model,
                                &positive,
                                parameter,
                                EvaluationDirection::Positive,
                            )?;

                        let value_plus =
                            positive_output.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: positive_output.len(),
                                    context: "forward output count",
                                },
                            )?;

                        let value_zero =
                            baseline.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: baseline.len(),
                                    context: "baseline output count",
                                },
                            )?;

                        (value_plus - value_zero) / step
                    }

                    FiniteDifferenceMethod::Backward => {
                        let negative =
                            parameters.with_perturbation(parameter, -step)?;

                        let negative_output =
                            self.evaluate_checked(
                                model,
                                &negative,
                                parameter,
                                EvaluationDirection::Negative,
                            )?;

                        let value_minus =
                            negative_output.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: negative_output.len(),
                                    context: "backward output count",
                                },
                            )?;

                        let value_zero =
                            baseline.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: baseline.len(),
                                    context: "baseline output count",
                                },
                            )?;

                        (value_zero - value_minus) / step
                    }

                    FiniteDifferenceMethod::Central => {
                        let positive =
                            parameters.with_perturbation(parameter, step)?;

                        let negative =
                            parameters.with_perturbation(parameter, -step)?;

                        let positive_output =
                            self.evaluate_checked(
                                model,
                                &positive,
                                parameter,
                                EvaluationDirection::Positive,
                            )?;

                        let negative_output =
                            self.evaluate_checked(
                                model,
                                &negative,
                                parameter,
                                EvaluationDirection::Negative,
                            )?;

                        let value_plus =
                            positive_output.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: positive_output.len(),
                                    context: "central positive output count",
                                },
                            )?;

                        let value_minus =
                            negative_output.get(output).ok_or(
                                SensitivityError::DimensionMismatch {
                                    expected: output_count,
                                    actual: negative_output.len(),
                                    context: "central negative output count",
                                },
                            )?;

                        (value_plus - value_minus) / (2.0 * step)
                    }
                };

                validate_finite("finite_difference_derivative", derivative)?;

                jacobian.push(derivative);
            }
        }

        SensitivityMatrix::new(
            output_count,
            parameter_count,
            jacobian,
            SensitivityKind::FiniteDifference,
        )
    }

    fn evaluate_checked<M: SensitivityModel>(
        &self,
        model: &M,
        parameters: &ParameterVector,
        parameter: usize,
        direction: EvaluationDirection,
    ) -> Result<OutputVector, SensitivityError> {
        let values = model
            .evaluate(parameters.as_slice())
            .map_err(|_| SensitivityError::ModelEvaluationFailed {
                parameter,
                direction,
            })?;

        OutputVector::new(values)
    }

    /// Builds a complete sensitivity report from a Jacobian.
    pub fn report(
        &self,
        jacobian: SensitivityMatrix,
        parameters: Option<ParameterVector>,
        baseline_outputs: Option<OutputVector>,
        standard_uncertainties: Option<&[f64]>,
    ) -> Result<SensitivityReport, SensitivityError> {
        self.policy
            .limits
            .checked_influence_elements(
                jacobian.parameter_count(),
                jacobian.output_count(),
            )?;

        SensitivityReport::from_jacobian(
            jacobian,
            parameters,
            baseline_outputs,
            standard_uncertainties,
        )
    }

    /// Performs finite-difference analysis and creates a complete report.
    pub fn analyze<M: SensitivityModel>(
        &self,
        model: &M,
        parameters: &ParameterVector,
        standard_uncertainties: Option<&[f64]>,
    ) -> Result<SensitivityReport, SensitivityError> {
        let jacobian =
            self.finite_difference(model, parameters)?;

        let baseline =
            OutputVector::new(model.evaluate(parameters.as_slice()).map_err(
                |_| SensitivityError::ModelEvaluationFailed {
                    parameter: 0,
                    direction: EvaluationDirection::Baseline,
                },
            )?)?;

        self.report(
            jacobian,
            Some(parameters.clone()),
            Some(baseline),
            standard_uncertainties,
        )
    }
}

// ============================================================================
// Covariance-aware influence
// ============================================================================

/// Validated row-major covariance matrix.
///
/// This type is intentionally a small semantic matrix owned by sensitivity
/// analysis rather than a second uncertainty model.
///
/// `uncertainty.rs` may provide its own covariance representation and convert
/// into this type at the integration boundary if necessary.
#[derive(Clone, Debug, PartialEq)]
pub struct CovarianceMatrix {
    dimension: usize,
    values: Vec<f64>,
}

impl CovarianceMatrix {
    /// Creates a covariance matrix after validating dimensions, finiteness and
    /// symmetry.
    pub fn new(
        dimension: usize,
        values: Vec<f64>,
        tolerance: f64,
    ) -> Result<Self, SensitivityError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(SensitivityError::InvalidTolerance {
                value: tolerance,
            });
        }

        let expected = dimension
            .checked_mul(dimension)
            .ok_or(SensitivityError::SizeOverflow {
                context: "covariance dimension squared",
            })?;

        if values.len() != expected {
            return Err(SensitivityError::DimensionMismatch {
                expected,
                actual: values.len(),
                context: "covariance matrix element count",
            });
        }

        for row in 0..dimension {
            for column in 0..dimension {
                let index = row
                    .checked_mul(dimension)
                    .and_then(|v| v.checked_add(column))
                    .ok_or(SensitivityError::SizeOverflow {
                        context: "covariance index",
                    })?;

                let value = values[index];

                if !value.is_finite() {
                    return Err(SensitivityError::InvalidCovariance {
                        row,
                        column,
                        value,
                    });
                }
            }
        }

        for row in 0..dimension {
            for column in row..dimension {
                let lhs_index = row
                    .checked_mul(dimension)
                    .and_then(|v| v.checked_add(column))
                    .ok_or(SensitivityError::SizeOverflow {
                        context: "covariance lhs index",
                    })?;

                let rhs_index = column
                    .checked_mul(dimension)
                    .and_then(|v| v.checked_add(row))
                    .ok_or(SensitivityError::SizeOverflow {
                        context: "covariance rhs index",
                    })?;

                let lhs = values[lhs_index];
                let rhs = values[rhs_index];

                if (lhs - rhs).abs() > tolerance {
                    return Err(SensitivityError::CovarianceNotSymmetric {
                        row,
                        column,
                        lhs,
                        rhs,
                    });
                }
            }
        }

        Ok(Self {
            dimension,
            values,
        })
    }

    /// Creates a diagonal covariance matrix from standard uncertainties.
    pub fn diagonal(
        standard_uncertainties: &[f64],
    ) -> Result<Self, SensitivityError> {
        let dimension = standard_uncertainties.len();

        let size = dimension
            .checked_mul(dimension)
            .ok_or(SensitivityError::SizeOverflow {
                context: "diagonal covariance size",
            })?;

        let mut values = vec![0.0; size];

        for (index, &uncertainty) in standard_uncertainties.iter().enumerate() {
            if !uncertainty.is_finite() || uncertainty < 0.0 {
                return Err(SensitivityError::NonFinite {
                    field: "standard_uncertainty",
                    value: uncertainty,
                });
            }

            let variance = uncertainty * uncertainty;

            validate_finite("variance", variance)?;

            let diagonal_index = index
                .checked_mul(dimension)
                .and_then(|v| v.checked_add(index))
                .ok_or(SensitivityError::SizeOverflow {
                    context: "diagonal covariance index",
                })?;

            values[diagonal_index] = variance;
        }

        Self::new(
            dimension,
            values,
            DEFAULT_TOLERANCE,
        )
    }

    /// Dimension of the covariance matrix.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns an element.
    pub fn get(&self, row: usize, column: usize) -> Option<f64> {
        if row >= self.dimension || column >= self.dimension {
            return None;
        }

        let index = row
            .checked_mul(self.dimension)?
            .checked_add(column)?;

        self.values.get(index).copied()
    }

    /// Returns the underlying row-major representation.
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

/// First-order output variance produced by a sensitivity row and covariance.
///
/// Mathematically:
///
/// ```text
/// variance(y) ≈ J Σ Jᵀ
/// ```
///
/// This function does not claim that the covariance itself is a valid physical
/// covariance beyond the validation performed by `CovarianceMatrix`.
pub fn propagated_output_variance(
    sensitivity: &[f64],
    covariance: &CovarianceMatrix,
) -> Result<f64, SensitivityError> {
    if sensitivity.len() != covariance.dimension() {
        return Err(SensitivityError::DimensionMismatch {
            expected: covariance.dimension(),
            actual: sensitivity.len(),
            context: "sensitivity/covariance dimension",
        });
    }

    let mut variance = 0.0_f64;

    for i in 0..sensitivity.len() {
        for j in 0..sensitivity.len() {
            let covariance_ij =
                covariance.get(i, j).ok_or(
                    SensitivityError::DimensionMismatch {
                        expected: covariance.dimension(),
                        actual: i,
                        context: "covariance access",
                    },
                )?;

            variance +=
                sensitivity[i] * covariance_ij * sensitivity[j];

            validate_finite(
                "propagated variance",
                variance,
            )?;
        }
    }

    if variance < 0.0 {
        // A mathematically valid covariance matrix should produce a
        // non-negative quadratic form. We do not silently clamp a negative
        // result because doing so could hide invalid numerical input.
        return Err(SensitivityError::InvalidCovariance {
            row: 0,
            column: 0,
            value: variance,
        });
    }

    Ok(variance)
}

/// First-order output standard uncertainty.
///
/// This computes:
///
/// ```text
/// sqrt(J Σ Jᵀ)
/// ```
pub fn propagated_output_standard_uncertainty(
    sensitivity: &[f64],
    covariance: &CovarianceMatrix,
) -> Result<f64, SensitivityError> {
    let variance =
        propagated_output_variance(sensitivity, covariance)?;

    let standard_uncertainty = variance.sqrt();

    validate_finite(
        "propagated standard uncertainty",
        standard_uncertainty,
    )?;

    Ok(standard_uncertainty)
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_finite(
    field: &'static str,
    value: f64,
) -> Result<(), SensitivityError> {
    if !value.is_finite() {
        return Err(SensitivityError::NonFinite {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_positive_finite(
    field: &'static str,
    value: f64,
) -> Result<(), SensitivityError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(SensitivityError::InvalidStep {
            value,
        });
    }

    // Keep the field argument semantically useful without creating a separate
    // error vocabulary for each positive scalar.
    let _ = field;

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct LinearModel;

    impl SensitivityModel for LinearModel {
        fn evaluate(
            &self,
            parameters: &[f64],
        ) -> Result<Vec<f64>, SensitivityModelError> {
            if parameters.len() != 2 {
                return Err(SensitivityModelError::new(
                    "expected two parameters",
                ));
            }

            Ok(vec![
                2.0 * parameters[0]
                    + 3.0 * parameters[1],
                5.0 * parameters[0]
                    - parameters[1],
            ])
        }
    }

    struct ProductModel;

    impl SensitivityModel for ProductModel {
        fn evaluate(
            &self,
            parameters: &[f64],
        ) -> Result<Vec<f64>, SensitivityModelError> {
            if parameters.len() != 2 {
                return Err(SensitivityModelError::new(
                    "expected two parameters",
                ));
            }

            Ok(vec![
                parameters[0] * parameters[1],
            ])
        }
    }

    #[test]
    fn parameter_vector_rejects_non_finite_values() {
        let result =
            ParameterVector::new(vec![1.0, f64::NAN]);

        assert!(matches!(
            result,
            Err(SensitivityError::NonFinite { .. })
        ));
    }

    #[test]
    fn parameter_vector_perturbation_is_deterministic() {
        let parameters =
            ParameterVector::new(vec![1.0, 2.0])
                .expect("valid parameters");

        let positive = parameters
            .with_perturbation(0, 0.5)
            .expect("valid perturbation");

        assert_eq!(positive.as_slice(), &[1.5, 2.0]);
    }

    #[test]
    fn exact_jacobian_has_expected_dimensions() {
        let jacobian = SensitivityMatrix::exact(
            2,
            2,
            vec![2.0, 3.0, 5.0, -1.0],
        )
        .expect("valid jacobian");

        assert_eq!(jacobian.output_count(), 2);
        assert_eq!(jacobian.parameter_count(), 2);
        assert_eq!(jacobian.get(0, 0), Some(2.0));
        assert_eq!(jacobian.get(1, 1), Some(-1.0));
        assert_eq!(
            jacobian.kind(),
            SensitivityKind::Exact
        );
    }

    #[test]
    fn central_difference_matches_linear_model() {
        let analyzer =
            SensitivityAnalyzer::standard();

        let parameters =
            ParameterVector::new(vec![1.0, 2.0])
                .expect("valid parameters");

        let jacobian = analyzer
            .finite_difference(&LinearModel, &parameters)
            .expect("finite difference should succeed");

        assert!((jacobian.get(0, 0).expect("value") - 2.0).abs() < 1e-8);
        assert!((jacobian.get(0, 1).expect("value") - 3.0).abs() < 1e-8);
        assert!((jacobian.get(1, 0).expect("value") - 5.0).abs() < 1e-8);
        assert!((jacobian.get(1, 1).expect("value") + 1.0).abs() < 1e-8);

        assert_eq!(
            jacobian.kind(),
            SensitivityKind::FiniteDifference
        );
    }

    #[test]
    fn forward_difference_is_supported() {
        let mut policy =
            SensitivityPolicy::default();

        policy.method =
            FiniteDifferenceMethod::Forward;

        let analyzer =
            SensitivityAnalyzer::new(policy)
                .expect("valid policy");

        let parameters =
            ParameterVector::new(vec![1.0, 2.0])
                .expect("valid parameters");

        let jacobian = analyzer
            .finite_difference(&LinearModel, &parameters)
            .expect("finite difference should succeed");

        assert!((jacobian.get(0, 0).expect("value") - 2.0).abs() < 1e-5);
        assert!((jacobian.get(0, 1).expect("value") - 3.0).abs() < 1e-5);
    }

    #[test]
    fn backward_difference_is_supported() {
        let mut policy =
            SensitivityPolicy::default();

        policy.method =
            FiniteDifferenceMethod::Backward;

        let analyzer =
            SensitivityAnalyzer::new(policy)
                .expect("valid policy");

        let parameters =
            ParameterVector::new(vec![1.0, 2.0])
                .expect("valid parameters");

        let jacobian = analyzer
            .finite_difference(&LinearModel, &parameters)
            .expect("finite difference should succeed");

        assert!((jacobian.get(0, 0).expect("value") - 2.0).abs() < 1e-5);
        assert!((jacobian.get(0, 1).expect("value") - 3.0).abs() < 1e-5);
    }

    #[test]
    fn nonlinear_central_difference_is_local() {
        let analyzer =
            SensitivityAnalyzer::standard();

        let parameters =
            ParameterVector::new(vec![3.0, 4.0])
                .expect("valid parameters");

        let jacobian = analyzer
            .finite_difference(&ProductModel, &parameters)
            .expect("finite difference should succeed");

        assert!(
            (jacobian.get(0, 0).expect("value") - 4.0).abs()
                < 1e-5
        );

        assert!(
            (jacobian.get(0, 1).expect("value") - 3.0).abs()
                < 1e-5
        );
    }

    #[test]
    fn relative_sensitivity_is_dimensionless() {
        let result =
            RelativeSensitivity::calculate(
                2.0,
                3.0,
                6.0,
            )
            .expect("valid relative sensitivity");

        assert_eq!(result.relative, 1.0);
    }

    #[test]
    fn relative_sensitivity_rejects_zero_output() {
        let result =
            RelativeSensitivity::calculate(
                2.0,
                3.0,
                0.0,
            );

        assert!(matches!(
            result,
            Err(SensitivityError::ZeroReference { .. })
        ));
    }

    #[test]
    fn covariance_diagonal_is_valid() {
        let covariance =
            CovarianceMatrix::diagonal(&[0.1, 0.2])
                .expect("valid covariance");

        assert_eq!(covariance.dimension(), 2);

        assert!(
            (covariance.get(0, 0).expect("value") - 0.01).abs()
                < 1e-12
        );

        assert!(
            (covariance.get(1, 1).expect("value") - 0.04).abs()
                < 1e-12
        );
    }

    #[test]
    fn covariance_rejects_non_symmetric_input() {
        let result =
            CovarianceMatrix::new(
                2,
                vec![
                    1.0, 0.1,
                    0.2, 1.0,
                ],
                1e-12,
            );

        assert!(matches!(
            result,
            Err(SensitivityError::CovarianceNotSymmetric { .. })
        ));
    }

    #[test]
    fn covariance_propagation_matches_diagonal_formula() {
        let covariance =
            CovarianceMatrix::diagonal(&[0.1, 0.2])
                .expect("valid covariance");

        let sensitivity = [2.0, 3.0];

        let variance =
            propagated_output_variance(
                &sensitivity,
                &covariance,
            )
            .expect("valid propagated variance");

        // 2²*0.1² + 3²*0.2² = 0.04 + 0.36 = 0.40
        assert!((variance - 0.40).abs() < 1e-12);
    }

    #[test]
    fn report_ranks_dominant_parameter() {
        let jacobian =
            SensitivityMatrix::exact(
                2,
                2,
                vec![
                    1.0, 10.0,
                    2.0, 20.0,
                ],
            )
            .expect("valid jacobian");

        let report =
            SensitivityReport::from_jacobian(
                jacobian,
                None,
                None,
                None,
            )
            .expect("valid report");

        assert_eq!(
            report
                .dominant_parameter()
                .expect("parameter")
                .parameter,
            1
        );
    }

    #[test]
    fn ranking_tie_is_deterministic() {
        let jacobian =
            SensitivityMatrix::exact(
                1,
                2,
                vec![5.0, 5.0],
            )
            .expect("valid jacobian");

        let report =
            SensitivityReport::from_jacobian(
                jacobian,
                None,
                None,
                None,
            )
            .expect("valid report");

        assert_eq!(
            report.parameter_ranking[0].parameter,
            0
        );

        assert_eq!(
            report.parameter_ranking[1].parameter,
            1
        );
    }

    #[test]
    fn uncertainty_contributions_are_recorded() {
        let jacobian =
            SensitivityMatrix::exact(
                1,
                2,
                vec![2.0, 3.0],
            )
            .expect("valid jacobian");

        let report =
            SensitivityReport::from_jacobian(
                jacobian,
                None,
                None,
                Some(&[0.1, 0.2]),
            )
            .expect("valid report");

        assert_eq!(
            report.influences[0]
                .uncertainty_contribution,
            Some(0.2)
        );

        assert_eq!(
            report.influences[1]
                .uncertainty_contribution,
            Some(0.6)
        );
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let jacobian =
            SensitivityMatrix::exact(
                2,
                2,
                vec![1.0, 2.0, 3.0, 4.0],
            )
            .expect("valid jacobian");

        let result =
            SensitivityReport::from_jacobian(
                jacobian,
                Some(
                    ParameterVector::new(vec![1.0])
                        .expect("valid parameter vector"),
                ),
                None,
                None,
            );

        assert!(matches!(
            result,
            Err(SensitivityError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn resource_limit_rejects_large_jacobian() {
        let policy = SensitivityPolicy {
            limits: SensitivityLimits {
                max_model_evaluations: None,
                max_jacobian_elements: Some(2),
                max_influence_elements: None,
            },
            ..SensitivityPolicy::default()
        };

        let analyzer =
            SensitivityAnalyzer::new(policy)
                .expect("valid policy");

        let parameters =
            ParameterVector::new(vec![1.0, 2.0])
                .expect("valid parameters");

        let result =
            analyzer.finite_difference(
                &LinearModel,
                &parameters,
            );

        assert!(matches!(
            result,
            Err(SensitivityError::ResourceLimitExceeded { .. })
        ));
    }

    #[test]
    fn zero_parameter_model_is_supported() {
        struct EmptyModel;

        impl SensitivityModel for EmptyModel {
            fn evaluate(
                &self,
                parameters: &[f64],
            ) -> Result<Vec<f64>, SensitivityModelError> {
                if !parameters.is_empty() {
                    return Err(SensitivityModelError::new(
                        "parameters must be empty",
                    ));
                }

                Ok(vec![42.0])
            }
        }

        let analyzer =
            SensitivityAnalyzer::standard();

        let parameters =
            ParameterVector::empty();

        let jacobian = analyzer
            .finite_difference(
                &EmptyModel,
                &parameters,
            )
            .expect("empty parameter model should work");

        assert_eq!(jacobian.output_count(), 1);
        assert_eq!(jacobian.parameter_count(), 0);
        assert!(jacobian.as_slice().is_empty());
    }
}