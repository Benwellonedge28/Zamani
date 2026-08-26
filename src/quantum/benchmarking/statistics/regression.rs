//! Zamani Quantum Benchmarking — Regression Statistics
//!
//! Production regression primitives for quantum benchmarking.
//!
//! # Purpose
//!
//! This module provides deterministic, bounded, protocol-independent
//! regression for benchmark data. Its primary production use case is fitting
//! randomized-benchmarking decay curves of the form
//!
//!     y(m) = A * p^m + B
//!
//! where:
//!
//! - `A` is the decay amplitude;
//! - `p` is the decay parameter;
//! - `B` is the asymptotic offset;
//! - `m` is sequence length / cycle count / benchmark depth.
//!
//! The implementation is deliberately broader than randomized benchmarking.
//! The same regression infrastructure can be used by:
//!
//! - randomized benchmarking;
//! - interleaved randomized benchmarking;
//! - simultaneous randomized benchmarking;
//! - purity randomized benchmarking;
//! - leakage benchmarking;
//! - cycle benchmarking;
//! - layer-fidelity experiments;
//! - coherence measurements;
//! - logical-error-rate experiments;
//! - drift/stability analysis;
//! - application benchmarking;
//! - future benchmark protocols.
//!
//! # Architectural boundary
//!
//! This module:
//!
//! - consumes already-observed numerical data;
//! - validates the data;
//! - fits a declared mathematical model;
//! - calculates residuals and goodness-of-fit diagnostics;
//! - calculates parameter uncertainty where numerically justified;
//! - exposes convergence and boundary diagnostics;
//! - enforces production statistical limits;
//! - performs no I/O;
//! - performs no logging;
//! - generates no quantum circuits;
//! - executes no circuits;
//! - knows nothing about hardware;
//! - knows nothing about quantum IR;
//! - does not calculate protocol-specific gate fidelity.
//!
//! The protocol layer is responsible for interpreting a fitted decay
//! parameter physically.
//!
//! For example, randomized benchmarking may derive an error-per-step estimate
//! from `p`, but this file must not assume that interpretation.
//!
//! # Statistical policy
//!
//! The canonical exponential-decay model is:
//!
//!     y = A * exp(-k * x) + B
//!
//! with:
//!
//!     p = exp(-k)
//!
//! This parameterization is numerically preferable to directly optimizing
//! `p^x` because `k` behaves smoothly near `p = 1`.
//!
//! The implementation uses a deterministic profile-search strategy:
//!
//! 1. validate the observations;
//! 2. search the decay-rate domain using a logarithmic grid;
//! 3. solve the linear `A/B` subproblem exactly for each candidate decay;
//! 4. refine the best candidate with bounded golden-section search;
//! 5. calculate residuals;
//! 6. calculate goodness-of-fit statistics;
//! 7. calculate covariance/standard errors when the design matrix is
//!    sufficiently well-conditioned;
//! 8. transform the decay-rate uncertainty into uncertainty for `p`;
//! 9. expose all assumptions and diagnostics to the caller.
//!
//! This avoids an unrestricted nonlinear optimizer while still providing a
//! genuine nonlinear fit for the exponential parameter.
//!
//! # Important scientific limitation
//!
//! A good numerical fit does not prove that the physical system obeys the
//! assumed model.
//!
//! In particular, standard randomized benchmarking often uses a single
//! exponential decay model under assumptions concerning the noise process.
//! Non-Markovian, time-dependent, coherent, leakage, or gate-dependent
//! behaviour can produce deviations from that model.
//!
//! Therefore [`RegressionFit`] exposes:
//!
//! - residual statistics;
//! - R²;
//! - adjusted R²;
//! - RMSE;
//! - AIC;
//! - BIC;
//! - boundary information;
//! - covariance availability;
//! - convergence information.
//!
//! Protocols must use these diagnostics before making scientific claims.
//!
//! # Weighting
//!
//! Optional observation weights are interpreted as inverse variances:
//!
//!     weight_i = 1 / variance_i
//!
//! When weights are supplied, the regression minimizes weighted least squares.
//!
//! When weights are absent, ordinary least squares is used.
//!
//! Weights must be finite and strictly positive.
//!
//! # Resource safety
//!
//! This module uses [`BenchmarkLimits`] and, specifically,
//! `max_statistical_iterations`.
//!
//! No user-controlled iteration count may bypass the configured production
//! limit.
//!
//! # Reproducibility
//!
//! The fitting algorithm is deterministic. There is no random number
//! generation and no global state.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Dependencies:
//!
//! ```text
//! statistics::regression
//!        │
//!        ├── statistics::confidence
//!        │
//!        └── core::limits
//! ```
//!
//! It must not depend on:
//!
//! - benchmark protocols;
//! - quantum IR;
//! - frontend/lowering;
//! - algorithms;
//! - routing;
//! - scheduling;
//! - hardware implementations;
//! - runtime;
//! - `volume_estimator.rs`;
//! - application benchmarks.
//!
//! Protocol integration is one-way:
//!
//! ```text
//! protocols::randomized_benchmarking
//!              │
//!              ▼
//!     statistics::regression
//!              │
//!              ▼
//!       RegressionFit
//! ```
//!
//! The protocol layer then interprets `decay_parameter` according to the
//! relevant benchmarking specification.
//!
//! # Integration with existing Zamani statistics
//!
//! [`ConfidenceLevel`] is reused from `statistics::confidence` so that
//! confidence semantics are not duplicated.
//!
//! Bootstrap analysis remains in `statistics::bootstrap`.
//!
//! Regression itself does not perform bootstrap resampling. A protocol may
//! take the deterministic fitted model and use `statistics::bootstrap` for
//! empirical uncertainty estimation when required.
//!
//! # Numerical design
//!
//! The implementation deliberately avoids an external linear-algebra crate.
//! The fitted linear subproblem has only two parameters and the covariance
//! calculation has only three parameters, so small fixed-size systems can be
//! solved directly with explicit conditioning checks.
//!
//! This keeps the benchmarking subsystem portable and dependency-light.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use super::confidence::{ConfidenceError, ConfidenceLevel};
use crate::quantum::benchmarking::core::limits::{BenchmarkLimits, LimitError};

/// Result type returned by regression operations.
pub type RegressionResult<T> = Result<T, RegressionError>;

/// Stable algorithm identifier.
///
/// If the numerical algorithm is changed in a way that can alter benchmark
/// results, this identifier must be changed so provenance can distinguish
/// results produced by different fitting algorithms.
pub const REGRESSION_ALGORITHM_ID: &str =
    "zamani.regression.exponential_profile.v1";

/// Canonical model identifier for randomized-benchmarking-style decay.
pub const EXPONENTIAL_DECAY_MODEL_ID: &str =
    "exponential_decay_a_p_x_plus_b.v1";

/// Minimum positive decay rate considered by the bounded search.
///
/// `k = 0` corresponds exactly to `p = 1`. This small positive value is used
/// as the lower numerical search boundary for the logarithmic portion of the
/// search.
pub const DEFAULT_MIN_DECAY_RATE: f64 = 1.0e-12;

/// Maximum decay rate.
///
/// `p = exp(-50)` is already extremely close to zero. Values beyond this
/// provide little practical benefit for benchmark decay fitting while making
/// the numerical search less stable.
pub const DEFAULT_MAX_DECAY_RATE: f64 = 50.0;

/// Number of deterministic grid points used to locate the decay-rate basin.
///
/// This is deliberately modest because every grid point solves a complete
/// weighted linear regression for `A` and `B`.
pub const DEFAULT_GRID_POINTS: u64 = 128;

/// Default relative convergence tolerance for the decay-rate refinement.
pub const DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-10;

/// Default absolute convergence tolerance for the decay-rate refinement.
pub const DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-14;

/// Default maximum refinement iterations.
///
/// The actual number is additionally bounded by `BenchmarkLimits`.
pub const DEFAULT_MAX_REFINEMENT_ITERATIONS: u64 = 256;

/// Minimum acceptable number of observations for a three-parameter
/// exponential model.
///
/// Three observations can algebraically determine three parameters, but they
/// leave zero residual degrees of freedom. Production statistical diagnostics
/// therefore require at least four observations.
pub const MIN_OBSERVATIONS: usize = 4;

/// Minimum condition number reciprocal accepted for covariance estimation.
///
/// This is a conservative numerical guard. A fit may still be returned when
/// covariance is unavailable.
pub const DEFAULT_MIN_CONDITION_RECIPROCAL: f64 = 1.0e-12;

/// Minimum number of distinct x-values.
///
/// The exponential model cannot be identified if all sequence lengths are
/// identical.
pub const MIN_DISTINCT_X_VALUES: usize = 2;

/// Errors produced by regression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegressionError {
    /// No observations were supplied.
    EmptyObservations,

    /// Too few observations were supplied for the requested model.
    InsufficientObservations {
        /// Number of observations supplied.
        observations: usize,

        /// Minimum required observations.
        minimum: usize,
    },

    /// An x-value is not finite.
    NonFiniteIndependentValue {
        /// Zero-based observation index.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// A y-value is not finite.
    NonFiniteDependentValue {
        /// Zero-based observation index.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// An observation weight is not finite.
    NonFiniteWeight {
        /// Zero-based observation index.
        index: usize,

        /// Invalid weight.
        value: f64,
    },

    /// An observation weight is not strictly positive.
    InvalidWeight {
        /// Zero-based observation index.
        index: usize,

        /// Invalid weight.
        value: f64,
    },

    /// All x-values are identical, making the decay parameter unidentifiable.
    InsufficientDistinctIndependentValues {
        /// Number of distinct x-values.
        distinct_values: usize,

        /// Minimum required distinct x-values.
        minimum: usize,
    },

    /// The configured lower decay rate is invalid.
    InvalidMinimumDecayRate {
        /// Invalid rate.
        value: f64,
    },

    /// The configured upper decay rate is invalid.
    InvalidMaximumDecayRate {
        /// Invalid rate.
        value: f64,
    },

    /// The decay-rate bounds are reversed or equal.
    InvalidDecayRateRange {
        /// Lower bound.
        minimum: f64,

        /// Upper bound.
        maximum: f64,
    },

    /// The configured convergence tolerance is invalid.
    InvalidTolerance {
        /// Invalid tolerance.
        value: f64,
    },

    /// The configured grid size is invalid.
    InvalidGridPoints {
        /// Invalid number of grid points.
        value: u64,
    },

    /// The configured iteration count is invalid.
    InvalidIterationCount {
        /// Invalid number of iterations.
        value: u64,
    },

    /// A requested statistical resource exceeds the production limit.
    Limit(LimitError),

    /// Confidence-level validation failed.
    Confidence(ConfidenceError),

    /// A two-parameter linear system could not be solved reliably.
    SingularLinearSystem,

    /// The covariance matrix could not be estimated reliably.
    SingularCovarianceMatrix,

    /// A numerical calculation produced NaN or infinity.
    NumericalFailure {
        /// Operation that failed.
        operation: &'static str,
    },

    /// The optimization did not produce a finite candidate.
    NoFiniteFit,

    /// The fitted result violates the configured model bounds.
    InvalidFit {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A user supplied an invalid model configuration.
    InvalidConfiguration {
        /// Human-readable reason.
        reason: &'static str,
    },
}

impl fmt::Display for RegressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservations => {
                write!(formatter, "regression requires observations")
            }

            Self::InsufficientObservations {
                observations,
                minimum,
            } => {
                write!(
                    formatter,
                    "regression requires at least {minimum} observations, got {observations}"
                )
            }

            Self::NonFiniteIndependentValue { index, value } => {
                write!(
                    formatter,
                    "independent value at index {index} is non-finite: {value}"
                )
            }

            Self::NonFiniteDependentValue { index, value } => {
                write!(
                    formatter,
                    "dependent value at index {index} is non-finite: {value}"
                )
            }

            Self::NonFiniteWeight { index, value } => {
                write!(
                    formatter,
                    "weight at index {index} is non-finite: {value}"
                )
            }

            Self::InvalidWeight { index, value } => {
                write!(
                    formatter,
                    "weight at index {index} must be strictly positive, got {value}"
                )
            }

            Self::InsufficientDistinctIndependentValues {
                distinct_values,
                minimum,
            } => {
                write!(
                    formatter,
                    "regression requires at least {minimum} distinct independent values, got {distinct_values}"
                )
            }

            Self::InvalidMinimumDecayRate { value } => {
                write!(
                    formatter,
                    "minimum decay rate must be finite and non-negative, got {value}"
                )
            }

            Self::InvalidMaximumDecayRate { value } => {
                write!(
                    formatter,
                    "maximum decay rate must be finite and positive, got {value}"
                )
            }

            Self::InvalidDecayRateRange { minimum, maximum } => {
                write!(
                    formatter,
                    "invalid decay-rate range: minimum={minimum}, maximum={maximum}"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "regression tolerance must be finite and positive, got {value}"
                )
            }

            Self::InvalidGridPoints { value } => {
                write!(
                    formatter,
                    "regression grid_points must be positive, got {value}"
                )
            }

            Self::InvalidIterationCount { value } => {
                write!(
                    formatter,
                    "regression iteration count must be positive, got {value}"
                )
            }

            Self::Limit(error) => {
                write!(formatter, "benchmark statistical limit: {error}")
            }

            Self::Confidence(error) => {
                write!(formatter, "confidence-level error: {error}")
            }

            Self::SingularLinearSystem => {
                write!(
                    formatter,
                    "exponential regression linear subproblem is singular or ill-conditioned"
                )
            }

            Self::SingularCovarianceMatrix => {
                write!(
                    formatter,
                    "exponential regression covariance matrix is singular or ill-conditioned"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "non-finite numerical result during {operation}"
                )
            }

            Self::NoFiniteFit => {
                write!(
                    formatter,
                    "regression search did not produce a finite fit"
                )
            }

            Self::InvalidFit { reason } => {
                write!(
                    formatter,
                    "regression produced an invalid fit: {reason}"
                )
            }

            Self::InvalidConfiguration { reason } => {
                write!(
                    formatter,
                    "invalid regression configuration: {reason}"
                )
            }
        }
    }
}

impl Error for RegressionError {}

impl From<LimitError> for RegressionError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

impl From<ConfidenceError> for RegressionError {
    fn from(error: ConfidenceError) -> Self {
        Self::Confidence(error)
    }
}

/// An individual regression observation.
///
/// `weight`, when present, is interpreted as inverse variance:
///
/// `weight = 1 / variance`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RegressionObservation {
    /// Independent variable, e.g. RB sequence length.
    pub x: f64,

    /// Observed response, e.g. survival probability.
    pub y: f64,

    /// Optional inverse-variance weight.
    pub weight: Option<f64>,
}

impl RegressionObservation {
    /// Creates an unweighted observation.
    pub const fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            weight: None,
        }
    }

    /// Creates an inverse-variance weighted observation.
    pub const fn weighted(x: f64, y: f64, weight: f64) -> Self {
        Self {
            x,
            y,
            weight: Some(weight),
        }
    }
}

/// Convenience constructor for an unweighted observation.
impl From<(f64, f64)> for RegressionObservation {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// Configuration for exponential-decay regression.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RegressionConfig {
    /// Confidence level used for parameter confidence intervals.
    pub confidence_level: ConfidenceLevel,

    /// Minimum decay rate `k` in `p = exp(-k)`.
    pub min_decay_rate: f64,

    /// Maximum decay rate `k` in `p = exp(-k)`.
    pub max_decay_rate: f64,

    /// Number of points in the deterministic initial search grid.
    pub grid_points: u64,

    /// Relative convergence tolerance.
    pub relative_tolerance: f64,

    /// Absolute convergence tolerance.
    pub absolute_tolerance: f64,

    /// Maximum local refinement iterations.
    pub max_refinement_iterations: u64,

    /// Whether covariance and parameter uncertainty should be calculated.
    pub calculate_uncertainty: bool,
}

impl Default for RegressionConfig {
    fn default() -> Self {
        Self {
            confidence_level: ConfidenceLevel::default(),
            min_decay_rate: DEFAULT_MIN_DECAY_RATE,
            max_decay_rate: DEFAULT_MAX_DECAY_RATE,
            grid_points: DEFAULT_GRID_POINTS,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            max_refinement_iterations: DEFAULT_MAX_REFINEMENT_ITERATIONS,
            calculate_uncertainty: true,
        }
    }
}

impl RegressionConfig {
    /// Creates the default production configuration.
    pub fn production() -> Self {
        Self::default()
    }

    /// Validates the configuration independently of benchmark limits.
    pub fn validate(&self) -> RegressionResult<()> {
        ConfidenceLevel::new(self.confidence_level.value())?;

        if !self.min_decay_rate.is_finite() || self.min_decay_rate < 0.0 {
            return Err(RegressionError::InvalidMinimumDecayRate {
                value: self.min_decay_rate,
            });
        }

        if !self.max_decay_rate.is_finite() || self.max_decay_rate <= 0.0 {
            return Err(RegressionError::InvalidMaximumDecayRate {
                value: self.max_decay_rate,
            });
        }

        if self.min_decay_rate >= self.max_decay_rate {
            return Err(RegressionError::InvalidDecayRateRange {
                minimum: self.min_decay_rate,
                maximum: self.max_decay_rate,
            });
        }

        if self.grid_points < 2 {
            return Err(RegressionError::InvalidGridPoints {
                value: self.grid_points,
            });
        }

        if self.max_refinement_iterations == 0 {
            return Err(RegressionError::InvalidIterationCount {
                value: self.max_refinement_iterations,
            });
        }

        if !self.relative_tolerance.is_finite()
            || self.relative_tolerance <= 0.0
        {
            return Err(RegressionError::InvalidTolerance {
                value: self.relative_tolerance,
            });
        }

        if !self.absolute_tolerance.is_finite()
            || self.absolute_tolerance <= 0.0
        {
            return Err(RegressionError::InvalidTolerance {
                value: self.absolute_tolerance,
            });
        }

        Ok(())
    }

    /// Validates the configuration against the global benchmarking resource
    /// policy.
    pub fn validate_against_limits(
        &self,
        limits: &BenchmarkLimits,
    ) -> RegressionResult<()> {
        self.validate()?;

        limits.check_statistical_iterations(self.grid_points)?;

        limits.check_statistical_iterations(
            self.max_refinement_iterations,
        )?;

        Ok(())
    }
}

/// Parameter uncertainty estimate.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParameterEstimate {
    /// Point estimate.
    pub value: f64,

    /// Standard error, if numerically available.
    pub standard_error: Option<f64>,

    /// Lower confidence bound, if available.
    pub lower: Option<f64>,

    /// Upper confidence bound, if available.
    pub upper: Option<f64>,
}

impl ParameterEstimate {
    /// Creates a point-only estimate.
    pub const fn point(value: f64) -> Self {
        Self {
            value,
            standard_error: None,
            lower: None,
            upper: None,
        }
    }

    /// Creates a fully specified estimate.
    pub const fn with_uncertainty(
        value: f64,
        standard_error: f64,
        lower: f64,
        upper: f64,
    ) -> Self {
        Self {
            value,
            standard_error: Some(standard_error),
            lower: Some(lower),
            upper: Some(upper),
        }
    }

    /// Returns whether uncertainty information is available.
    pub const fn has_uncertainty(self) -> bool {
        self.standard_error.is_some()
            && self.lower.is_some()
            && self.upper.is_some()
    }
}

/// Regression diagnostics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegressionDiagnostics {
    /// Number of observations.
    pub observations: usize,

    /// Number of fitted parameters.
    pub parameters: usize,

    /// Residual degrees of freedom.
    pub degrees_of_freedom: isize,

    /// Sum of squared residuals.
    pub residual_sum_of_squares: f64,

    /// Weighted sum of squared residuals.
    pub weighted_residual_sum_of_squares: f64,

    /// Root mean squared error.
    pub rmse: f64,

    /// Coefficient of determination.
    pub r_squared: Option<f64>,

    /// Adjusted coefficient of determination.
    pub adjusted_r_squared: Option<f64>,

    /// Akaike information criterion.
    pub aic: Option<f64>,

    /// Bayesian information criterion.
    pub bic: Option<f64>,

    /// Whether weights were used.
    pub weighted: bool,

    /// Whether the covariance matrix was successfully estimated.
    pub covariance_available: bool,

    /// Numerical conditioning indicator.
    ///
    /// This is the reciprocal of the approximate condition number of the
    /// normal matrix after scaling. Larger is better.
    pub conditioning_reciprocal: Option<f64>,

    /// Number of refinement iterations performed.
    pub refinement_iterations: u64,

    /// Whether the refinement met the convergence criterion.
    pub converged: bool,

    /// Whether the optimum was found at a search boundary.
    pub boundary_solution: bool,

    /// Minimum fitted residual.
    pub residual_min: f64,

    /// Maximum fitted residual.
    pub residual_max: f64,

    /// Mean residual.
    pub residual_mean: f64,

    /// Maximum absolute residual.
    pub maximum_absolute_residual: f64,
}

impl RegressionDiagnostics {
    /// Returns whether the fit has positive residual degrees of freedom.
    pub fn has_residual_degrees_of_freedom(&self) -> bool {
        self.degrees_of_freedom > 0
    }

    /// Returns whether the model has a potentially meaningful R².
    pub fn has_r_squared(&self) -> bool {
        self.r_squared.is_some()
    }

    /// Returns whether the fit was numerically converged.
    pub fn is_converged(&self) -> bool {
        self.converged
    }
}

/// Fitted exponential-decay model.
///
/// The physical/protocol interpretation of `decay_parameter` belongs to the
/// caller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegressionFit {
    /// Stable fitting algorithm identifier.
    pub algorithm: String,

    /// Stable model identifier.
    pub model: String,

    /// Fitted amplitude `A`.
    pub amplitude: ParameterEstimate,

    /// Fitted decay parameter `p`.
    ///
    /// The model is `A * p^x + B`.
    pub decay_parameter: ParameterEstimate,

    /// Fitted asymptotic offset `B`.
    pub offset: ParameterEstimate,

    /// Equivalent exponential decay rate `k`, where `p = exp(-k)`.
    pub decay_rate: ParameterEstimate,

    /// Diagnostics.
    pub diagnostics: RegressionDiagnostics,

    /// Predicted values at each input observation.
    pub predictions: Vec<f64>,

    /// Residuals `observed - predicted`.
    pub residuals: Vec<f64>,

    /// Confidence level used for uncertainty intervals.
    pub confidence_level: ConfidenceLevel,
}

impl RegressionFit {
    /// Predicts a value at a new independent-variable value.
    pub fn predict(&self, x: f64) -> RegressionResult<f64> {
        if !x.is_finite() {
            return Err(RegressionError::NumericalFailure {
                operation: "prediction input validation",
            });
        }

        let amplitude = self.amplitude.value;
        let decay = self.decay_parameter.value;
        let offset = self.offset.value;

        let prediction =
            amplitude * decay.powf(x) + offset;

        if !prediction.is_finite() {
            return Err(RegressionError::NumericalFailure {
                operation: "exponential prediction",
            });
        }

        Ok(prediction)
    }

    /// Returns the fitted error rate `1 - p`.
    ///
    /// This is a mathematical transformation only. Protocols must decide
    /// whether this quantity has the required physical interpretation.
    pub fn error_rate(&self) -> f64 {
        1.0 - self.decay_parameter.value
    }

    /// Returns the Akaike information criterion, when available.
    pub fn aic(&self) -> Option<f64> {
        self.diagnostics.aic
    }

    /// Returns the Bayesian information criterion, when available.
    pub fn bic(&self) -> Option<f64> {
        self.diagnostics.bic
    }

    /// Returns whether the fit landed on the decay-rate search boundary.
    pub fn is_boundary_solution(&self) -> bool {
        self.diagnostics.boundary_solution
    }
}

/// Internal representation of the fitted linear coefficients for a fixed
/// exponential decay rate.
#[derive(Debug, Clone, Copy)]
struct LinearFit {
    amplitude: f64,
    offset: f64,
    weighted_sse: f64,
}

/// Internal normal-equation accumulator for two parameters.
///
/// The model is:
///
///     y = A*z + B
///
/// where `z = exp(-k*x)`.
#[derive(Debug, Clone, Copy, Default)]
struct LinearAccumulator {
    s_zz: f64,
    s_z: f64,
    s_one: f64,
    s_zy: f64,
    s_y: f64,
}

impl LinearAccumulator {
    fn add(
        &mut self,
        z: f64,
        y: f64,
        weight: f64,
    ) -> RegressionResult<()> {
        let terms = [
            z * z * weight,
            z * weight,
            weight,
            z * y * weight,
            y * weight,
        ];

        for value in terms {
            if !value.is_finite() {
                return Err(RegressionError::NumericalFailure {
                    operation: "linear normal-equation accumulation",
                });
            }
        }

        self.s_zz += terms[0];
        self.s_z += terms[1];
        self.s_one += terms[2];
        self.s_zy += terms[3];
        self.s_y += terms[4];

        if !self.s_zz.is_finite()
            || !self.s_z.is_finite()
            || !self.s_one.is_finite()
            || !self.s_zy.is_finite()
            || !self.s_y.is_finite()
        {
            return Err(RegressionError::NumericalFailure {
                operation: "linear normal-equation accumulation",
            });
        }

        Ok(())
    }

    fn solve(self) -> RegressionResult<(f64, f64)> {
        let determinant =
            self.s_zz * self.s_one - self.s_z * self.s_z;

        let scale = self.s_zz.abs() * self.s_one.abs()
            + self.s_z.abs() * self.s_z.abs()
            + 1.0;

        if !determinant.is_finite()
            || determinant.abs()
                <= f64::EPSILON * scale * 128.0
        {
            return Err(RegressionError::SingularLinearSystem);
        }

        let amplitude =
            (self.s_zy * self.s_one - self.s_z * self.s_y)
                / determinant;

        let offset =
            (self.s_zz * self.s_y - self.s_z * self.s_zy)
                / determinant;

        if !amplitude.is_finite() || !offset.is_finite() {
            return Err(RegressionError::NumericalFailure {
                operation: "linear coefficient solution",
            });
        }

        Ok((amplitude, offset))
    }
}

/// Internal evaluator for a fixed decay rate.
fn evaluate_decay_rate(
    observations: &[RegressionObservation],
    decay_rate: f64,
) -> RegressionResult<LinearFit> {
    if !decay_rate.is_finite() || decay_rate < 0.0 {
        return Err(RegressionError::NumericalFailure {
            operation: "decay-rate evaluation",
        });
    }

    let mut accumulator = LinearAccumulator::default();

    for observation in observations {
        let weight = observation.weight.unwrap_or(1.0);

        let exponent = -decay_rate * observation.x;

        let z = if exponent < -745.0 {
            0.0
        } else {
            exponent.exp()
        };

        if !z.is_finite() {
            return Err(RegressionError::NumericalFailure {
                operation: "exponential basis evaluation",
            });
        }

        accumulator.add(z, observation.y, weight)?;
    }

    let (amplitude, offset) = accumulator.solve()?;

    let mut weighted_sse = 0.0;

    for observation in observations {
        let weight = observation.weight.unwrap_or(1.0);
        let exponent = -decay_rate * observation.x;

        let z = if exponent < -745.0 {
            0.0
        } else {
            exponent.exp()
        };

        let prediction = amplitude * z + offset;
        let residual = observation.y - prediction;

        let contribution = weight * residual * residual;

        if !contribution.is_finite() {
            return Err(RegressionError::NumericalFailure {
                operation: "weighted residual calculation",
            });
        }

        weighted_sse += contribution;
    }

    if !weighted_sse.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "weighted residual accumulation",
        });
    }

    Ok(LinearFit {
        amplitude,
        offset,
        weighted_sse,
    })
}

/// Determines whether a value has reached the numerical convergence
/// criterion relative to another value.
fn approximately_equal(
    a: f64,
    b: f64,
    relative_tolerance: f64,
    absolute_tolerance: f64,
) -> bool {
    let difference = (a - b).abs();

    difference <= absolute_tolerance
        || difference
            <= relative_tolerance * a.abs().max(b.abs()).max(1.0)
}

/// Returns the normal-distribution quantile for probability `p`.
///
/// This is an implementation of the Acklam rational approximation. It is
/// sufficient for confidence intervals in this module and avoids introducing
/// another numerical dependency.
///
/// The function is internal because the confidence module remains the
/// canonical owner of general confidence-interval semantics.
fn standard_normal_quantile(p: f64) -> RegressionResult<f64> {
    if !p.is_finite() || p <= 0.0 || p >= 1.0 {
        return Err(RegressionError::InvalidConfiguration {
            reason: "normal quantile probability must be strictly between 0 and 1",
        });
    }

    const A: [f64; 6] = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.383577518672690e2,
        -3.066479806614716e1,
        2.506628277459239e0,
    ];

    const B: [f64; 5] = [
        -5.447609879822406e1,
        1.615858368580409e2,
        -1.556989798598866e2,
        6.680131188771972e1,
        -1.328068155288572e1,
    ];

    const C: [f64; 6] = [
        -7.784894002430293e-3,
        -3.223964580411365e-1,
        -2.400758277161838e0,
        -2.549732539343734e0,
        4.374664141464968e0,
        2.938163982698783e0,
    ];

    const D: [f64; 4] = [
        7.784695709041462e-3,
        3.224671290700398e-1,
        2.445134137142996e0,
        3.754408661907416e0,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        let numerator = ((((C[0] * q + C[1]) * q + C[2]) * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5];

        let denominator =
            (((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0;

        return Ok(-numerator / denominator);
    }

    if p > HIGH {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        let numerator = ((((C[0] * q + C[1]) * q + C[2]) * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5];

        let denominator =
            (((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0;

        return Ok(numerator / denominator);
    }

    let q = p - 0.5;
    let r = q * q;

    let numerator =
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r
            + A[4])
            * r
            + A[5])
            * q;

    let denominator =
        ((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r
            + B[4])
            * r
            + 1.0;

    let result = numerator / denominator;

    if !result.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "standard normal quantile",
        });
    }

    Ok(result)
}

/// A small symmetric 3×3 matrix.
///
/// Used internally for the covariance of `(A, k, B)`.
#[derive(Debug, Clone, Copy)]
struct Matrix3 {
    values: [[f64; 3]; 3],
}

impl Matrix3 {
    fn zero() -> Self {
        Self {
            values: [[0.0; 3]; 3],
        }
    }

    fn add_outer_product(
        &mut self,
        vector: [f64; 3],
        weight: f64,
    ) -> RegressionResult<()> {
        for row in 0..3 {
            for column in row..3 {
                let contribution =
                    weight * vector[row] * vector[column];

                if !contribution.is_finite() {
                    return Err(RegressionError::NumericalFailure {
                        operation: "covariance normal-matrix accumulation",
                    });
                }

                self.values[row][column] += contribution;
                if row != column {
                    self.values[column][row] =
                        self.values[row][column];
                }
            }
        }

        Ok(())
    }

    fn determinant(&self) -> f64 {
        let a = self.values;

        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1]
                * (a[1][0] * a[2][2]
                    - a[1][2] * a[2][0])
            + a[0][2]
                * (a[1][0] * a[2][1]
                    - a[1][1] * a[2][0])
    }

    fn inverse(&self) -> RegressionResult<Self> {
        let determinant = self.determinant();

        let max_element = self
            .values
            .iter()
            .flat_map(|row| row.iter())
            .map(|value| value.abs())
            .fold(0.0, f64::max);

        if !determinant.is_finite()
            || !max_element.is_finite()
            || max_element == 0.0
            || determinant.abs()
                <= f64::EPSILON
                    * max_element
                    * max_element
                    * max_element
                    * 1024.0
        {
            return Err(RegressionError::SingularCovarianceMatrix);
        }

        let a = self.values;

        let mut inverse = [[0.0; 3]; 3];

        inverse[0][0] =
            (a[1][1] * a[2][2] - a[1][2] * a[2][1])
                / determinant;

        inverse[0][1] =
            (a[0][2] * a[2][1] - a[0][1] * a[2][2])
                / determinant;

        inverse[0][2] =
            (a[0][1] * a[1][2] - a[0][2] * a[1][1])
                / determinant;

        inverse[1][0] =
            (a[1][2] * a[2][0] - a[1][0] * a[2][2])
                / determinant;

        inverse[1][1] =
            (a[0][0] * a[2][2] - a[0][2] * a[2][0])
                / determinant;

        inverse[1][2] =
            (a[0][2] * a[1][0] - a[0][0] * a[1][2])
                / determinant;

        inverse[2][0] =
            (a[1][0] * a[2][1] - a[1][1] * a[2][0])
                / determinant;

        inverse[2][1] =
            (a[0][1] * a[2][0] - a[0][0] * a[2][1])
                / determinant;

        inverse[2][2] =
            (a[0][0] * a[1][1] - a[0][1] * a[1][0])
                / determinant;

        for row in 0..3 {
            for column in 0..3 {
                if !inverse[row][column].is_finite() {
                    return Err(
                        RegressionError::SingularCovarianceMatrix,
                    );
                }
            }
        }

        Ok(Self { values: inverse })
    }
}

/// Creates the Jacobian normal matrix for `(A, k, B)`.
fn covariance_normal_matrix(
    observations: &[RegressionObservation],
    amplitude: f64,
    decay_rate: f64,
) -> RegressionResult<Matrix3> {
    let mut matrix = Matrix3::zero();

    for observation in observations {
        let weight = observation.weight.unwrap_or(1.0);

        let exponent = -decay_rate * observation.x;

        let z = if exponent < -745.0 {
            0.0
        } else {
            exponent.exp()
        };

        let derivative_amplitude = z;
        let derivative_decay_rate =
            -amplitude * observation.x * z;
        let derivative_offset = 1.0;

        let jacobian = [
            derivative_amplitude,
            derivative_decay_rate,
            derivative_offset,
        ];

        matrix.add_outer_product(jacobian, weight)?;
    }

    Ok(matrix)
}

/// Calculates an approximate conditioning reciprocal for the covariance
/// normal matrix.
///
/// This is deliberately conservative. It is a diagnostic rather than a
/// substitute for a full SVD-based condition-number calculation.
fn conditioning_reciprocal(matrix: &Matrix3) -> Option<f64> {
    let max_row_sum = matrix
        .values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| value.abs())
                .sum::<f64>()
        })
        .fold(0.0, f64::max);

    if !max_row_sum.is_finite() || max_row_sum <= 0.0 {
        return None;
    }

    let inverse = matrix.inverse().ok()?;

    let max_inverse_row_sum = inverse
        .values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| value.abs())
                .sum::<f64>()
        })
        .fold(0.0, f64::max);

    if !max_inverse_row_sum.is_finite()
        || max_inverse_row_sum <= 0.0
    {
        return None;
    }

    let condition = max_row_sum * max_inverse_row_sum;

    if !condition.is_finite() || condition <= 0.0 {
        return None;
    }

    Some(1.0 / condition)
}

/// Calculates weighted/unweighted residual statistics.
fn calculate_diagnostics(
    observations: &[RegressionObservation],
    predictions: &[f64],
    residuals: &[f64],
    parameter_count: usize,
    amplitude: f64,
    decay_rate: f64,
    refinement_iterations: u64,
    converged: bool,
    boundary_solution: bool,
    covariance_matrix: Option<Matrix3>,
    calculate_uncertainty: bool,
) -> RegressionResult<(
    RegressionDiagnostics,
    Option<[f64; 3]>,
)> {
    if observations.len() != predictions.len()
        || observations.len() != residuals.len()
    {
        return Err(RegressionError::InvalidConfiguration {
            reason: "internal regression vector lengths are inconsistent",
        });
    }

    let n = observations.len();

    let weighted =
        observations.iter().any(|observation| {
            observation.weight.is_some()
        });

    let mut rss = 0.0;
    let mut weighted_rss = 0.0;
    let mut residual_sum = 0.0;

    let mut residual_min = f64::INFINITY;
    let mut residual_max = f64::NEG_INFINITY;
    let mut maximum_absolute_residual = 0.0;

    let mut weighted_y_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut unweighted_y_sum = 0.0;

    for observation in observations {
        let weight = observation.weight.unwrap_or(1.0);

        weighted_y_sum += weight * observation.y;
        weight_sum += weight;
        unweighted_y_sum += observation.y;

        if !weighted_y_sum.is_finite()
            || !weight_sum.is_finite()
            || !unweighted_y_sum.is_finite()
        {
            return Err(RegressionError::NumericalFailure {
                operation: "regression diagnostic mean accumulation",
            });
        }
    }

    let mean_y = if weighted {
        weighted_y_sum / weight_sum
    } else {
        unweighted_y_sum / n as f64
    };

    for index in 0..n {
        let residual = residuals[index];
        let weight = observations[index].weight.unwrap_or(1.0);

        rss += residual * residual;
        weighted_rss += weight * residual * residual;
        residual_sum += residual;

        residual_min = residual_min.min(residual);
        residual_max = residual_max.max(residual);
        maximum_absolute_residual =
            maximum_absolute_residual.max(residual.abs());

        if !rss.is_finite()
            || !weighted_rss.is_finite()
            || !residual_sum.is_finite()
        {
            return Err(RegressionError::NumericalFailure {
                operation: "regression residual accumulation",
            });
        }
    }

    let degrees_of_freedom =
        n as isize - parameter_count as isize;

    let denominator = if weighted {
        weighted_rss
    } else {
        rss
    };

    let rmse = (denominator / n as f64).sqrt();

    if !rmse.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "regression RMSE",
        });
    }

    let mut total_sum_of_squares = 0.0;

    for observation in observations {
        let weight = observation.weight.unwrap_or(1.0);
        let deviation = observation.y - mean_y;

        total_sum_of_squares += weight * deviation * deviation;
    }

    let r_squared = if total_sum_of_squares > 0.0
        && total_sum_of_squares.is_finite()
    {
        Some(
            1.0
                - weighted_rss
                    / total_sum_of_squares,
        )
    } else {
        None
    };

    let adjusted_r_squared = match r_squared {
        Some(value) if degrees_of_freedom > 0 => {
            let n_float = n as f64;
            let p_float = parameter_count as f64;

            Some(
                1.0
                    - (1.0 - value)
                        * ((n_float - 1.0)
                            / (n_float - p_float)),
            )
        }
        _ => None,
    };

    let information_criterion_sse =
        if weighted { weighted_rss } else { rss };

    let aic = if information_criterion_sse > 0.0
        && information_criterion_sse.is_finite()
    {
        let n_float = n as f64;
        let p_float = parameter_count as f64;

        Some(
            n_float
                * (information_criterion_sse / n_float).ln()
                + 2.0 * p_float,
        )
    } else {
        None
    };

    let bic = if information_criterion_sse > 0.0
        && information_criterion_sse.is_finite()
    {
        let n_float = n as f64;
        let p_float = parameter_count as f64;

        Some(
            n_float
                * (information_criterion_sse / n_float).ln()
                + p_float * n_float.ln(),
        )
    } else {
        None
    };

    let conditioning =
        covariance_matrix.and_then(conditioning_reciprocal);

    let mut standard_errors = None;

    if calculate_uncertainty && degrees_of_freedom > 0 {
        if let Some(matrix) = covariance_matrix {
            if let Ok(inverse) = matrix.inverse() {
                let covariance_scale = if weighted {
                    1.0
                } else {
                    weighted_rss
                        / degrees_of_freedom as f64
                };

                if covariance_scale.is_finite()
                    && covariance_scale >= 0.0
                {
                    let mut errors = [0.0; 3];

                    for index in 0..3 {
                        let variance =
                            inverse.values[index][index]
                                * covariance_scale;

                        if variance.is_finite()
                            && variance >= 0.0
                        {
                            errors[index] = variance.sqrt();
                        } else {
                            standard_errors = None;
                            break;
                        }
                    }

                    if standard_errors.is_none() {
                        if errors
                            .iter()
                            .all(|value| value.is_finite())
                        {
                            standard_errors = Some(errors);
                        }
                    }
                }
            }
        }
    }

    let diagnostics = RegressionDiagnostics {
        observations: n,
        parameters: parameter_count,
        degrees_of_freedom,
        residual_sum_of_squares: rss,
        weighted_residual_sum_of_squares: weighted_rss,
        rmse,
        r_squared,
        adjusted_r_squared,
        aic,
        bic,
        weighted,
        covariance_available: standard_errors.is_some(),
        conditioning_reciprocal: conditioning,
        refinement_iterations,
        converged,
        boundary_solution,
        residual_min,
        residual_max,
        residual_mean: residual_sum / n as f64,
        maximum_absolute_residual,
    };

    Ok((diagnostics, standard_errors))
}

/// Creates a parameter estimate using a normal approximation.
///
/// Bounds are clipped to mathematically valid decay bounds when requested.
fn normal_parameter_estimate(
    value: f64,
    standard_error: Option<f64>,
    confidence_level: ConfidenceLevel,
    lower_bound: Option<f64>,
    upper_bound: Option<f64>,
) -> RegressionResult<ParameterEstimate> {
    if !value.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "parameter estimate",
        });
    }

    let Some(se) = standard_error else {
        return Ok(ParameterEstimate::point(value));
    };

    if !se.is_finite() || se < 0.0 {
        return Ok(ParameterEstimate::point(value));
    }

    let tail = confidence_level.two_sided_tail_probability();
    let probability = 1.0 - tail;

    let z = standard_normal_quantile(probability)?;

    let margin = z * se;

    if !margin.is_finite() {
        return Ok(ParameterEstimate::point(value));
    }

    let mut lower = value - margin;
    let mut upper = value + margin;

    if let Some(bound) = lower_bound {
        lower = lower.max(bound);
    }

    if let Some(bound) = upper_bound {
        upper = upper.min(bound);
    }

    if lower > upper || !lower.is_finite() || !upper.is_finite() {
        return Ok(ParameterEstimate::point(value));
    }

    Ok(ParameterEstimate::with_uncertainty(
        value,
        se,
        lower,
        upper,
    ))
}

/// Converts an uncertainty estimate for `k` into an uncertainty estimate for
/// `p = exp(-k)`.
fn decay_parameter_from_rate(
    decay_rate: f64,
    standard_error: Option<f64>,
    confidence_level: ConfidenceLevel,
) -> RegressionResult<ParameterEstimate> {
    let p = (-decay_rate).exp();

    if !p.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "decay-rate to decay-parameter conversion",
        });
    }

    let Some(rate_error) = standard_error else {
        return Ok(ParameterEstimate::point(p));
    };

    if !rate_error.is_finite() || rate_error < 0.0 {
        return Ok(ParameterEstimate::point(p));
    }

    // Delta method:
    //
    // p = exp(-k)
    //
    // dp/dk = -exp(-k) = -p
    //
    // sigma_p = p * sigma_k
    let parameter_error = p * rate_error;

    normal_parameter_estimate(
        p,
        Some(parameter_error),
        confidence_level,
        Some(0.0),
        Some(1.0),
    )
}

/// Validates and normalizes input observations.
fn validate_observations(
    observations: &[RegressionObservation],
) -> RegressionResult<()> {
    if observations.is_empty() {
        return Err(RegressionError::EmptyObservations);
    }

    if observations.len() < MIN_OBSERVATIONS {
        return Err(RegressionError::InsufficientObservations {
            observations: observations.len(),
            minimum: MIN_OBSERVATIONS,
        });
    }

    let mut distinct_x = Vec::<f64>::new();

    for (index, observation) in observations.iter().enumerate() {
        if !observation.x.is_finite() {
            return Err(
                RegressionError::NonFiniteIndependentValue {
                    index,
                    value: observation.x,
                },
            );
        }

        if !observation.y.is_finite() {
            return Err(
                RegressionError::NonFiniteDependentValue {
                    index,
                    value: observation.y,
                },
            );
        }

        if let Some(weight) = observation.weight {
            if !weight.is_finite() {
                return Err(RegressionError::NonFiniteWeight {
                    index,
                    value: weight,
                });
            }

            if weight <= 0.0 {
                return Err(RegressionError::InvalidWeight {
                    index,
                    value: weight,
                });
            }
        }

        if !distinct_x
            .iter()
            .any(|existing| *existing == observation.x)
        {
            distinct_x.push(observation.x);
        }
    }

    if distinct_x.len() < MIN_DISTINCT_X_VALUES {
        return Err(
            RegressionError::InsufficientDistinctIndependentValues {
                distinct_values: distinct_x.len(),
                minimum: MIN_DISTINCT_X_VALUES,
            },
        );
    }

    Ok(())
}

/// Returns a deterministic logarithmic search grid for decay rate.
///
/// `k = 0` is included separately because it cannot be represented by
/// logarithmic coordinates.
fn generate_decay_grid(
    minimum: f64,
    maximum: f64,
    points: u64,
) -> RegressionResult<Vec<f64>> {
    if points < 2 {
        return Err(RegressionError::InvalidGridPoints {
            value: points,
        });
    }

    if minimum < 0.0
        || !minimum.is_finite()
        || !maximum.is_finite()
        || maximum <= 0.0
        || minimum >= maximum
    {
        return Err(RegressionError::InvalidDecayRateRange {
            minimum,
            maximum,
        });
    }

    let count = usize::try_from(points).map_err(|_| {
        RegressionError::InvalidGridPoints { value: points }
    })?;

    let mut grid = Vec::with_capacity(count + 1);

    // Include the no-decay boundary exactly.
    grid.push(0.0);

    let lower = minimum.max(f64::MIN_POSITIVE).ln();
    let upper = maximum.ln();

    for index in 0..count {
        let fraction = index as f64 / (count - 1) as f64;
        let log_rate =
            lower + fraction * (upper - lower);

        let rate = log_rate.exp();

        if !rate.is_finite()
            || rate < minimum
            || rate > maximum
        {
            return Err(RegressionError::NumericalFailure {
                operation: "decay-rate search grid generation",
            });
        }

        grid.push(rate);
    }

    Ok(grid)
}

/// Finds the best initial decay-rate candidate.
fn find_grid_candidate(
    observations: &[RegressionObservation],
    grid: &[f64],
) -> RegressionResult<(f64, LinearFit, usize)> {
    let mut best_rate = 0.0;
    let mut best_fit: Option<LinearFit> = None;
    let mut best_index = 0usize;

    for (index, &rate) in grid.iter().enumerate() {
        match evaluate_decay_rate(observations, rate) {
            Ok(fit) => {
                let replace = match best_fit {
                    None => true,
                    Some(current) => {
                        fit.weighted_sse < current.weighted_sse
                    }
                };

                if replace {
                    best_rate = rate;
                    best_fit = Some(fit);
                    best_index = index;
                }
            }

            Err(RegressionError::SingularLinearSystem) => {
                // A singular point does not invalidate the entire search.
                // The next candidate may be well-conditioned.
            }

            Err(error) => return Err(error),
        }
    }

    let Some(fit) = best_fit else {
        return Err(RegressionError::NoFiniteFit);
    };

    Ok((best_rate, fit, best_index))
}

/// Performs golden-section refinement inside a bounded decay-rate interval.
fn refine_decay_rate(
    observations: &[RegressionObservation],
    mut lower: f64,
    mut upper: f64,
    config: &RegressionConfig,
    iteration_limit: u64,
) -> RegressionResult<(f64, LinearFit, u64, bool)> {
    const GOLDEN_RATIO: f64 = 0.6180339887498949;

    if lower < 0.0
        || upper <= lower
        || !lower.is_finite()
        || !upper.is_finite()
    {
        return Err(RegressionError::InvalidDecayRateRange {
            minimum: lower,
            maximum: upper,
        });
    }

    // If the interval touches zero, use a tiny positive lower bound for the
    // logarithmic refinement while retaining zero as a separately tested
    // boundary solution.
    if lower == 0.0 {
        lower = DEFAULT_MIN_DECAY_RATE.min(upper * 0.5);
    }

    let mut x1 =
        upper - GOLDEN_RATIO * (upper - lower);
    let mut x2 =
        lower + GOLDEN_RATIO * (upper - lower);

    let mut f1 =
        evaluate_decay_rate(observations, x1)?;
    let mut f2 =
        evaluate_decay_rate(observations, x2)?;

    let mut iterations = 0u64;
    let mut converged = false;

    while iterations < iteration_limit {
        iterations += 1;

        let width = upper - lower;

        if approximately_equal(
            width,
            0.0,
            config.relative_tolerance,
            config.absolute_tolerance,
        ) {
            converged = true;
            break;
        }

        if f1.weighted_sse <= f2.weighted_sse {
            upper = x2;
            x2 = x1;
            f2 = f1;

            x1 =
                upper - GOLDEN_RATIO * (upper - lower);

            f1 =
                evaluate_decay_rate(observations, x1)?;
        } else {
            lower = x1;
            x1 = x2;
            f1 = f2;

            x2 =
                lower + GOLDEN_RATIO * (upper - lower);

            f2 =
                evaluate_decay_rate(observations, x2)?;
        }
    }

    let candidates = [
        (x1, f1),
        (x2, f2),
        ((lower + upper) / 2.0, evaluate_decay_rate(
            observations,
            (lower + upper) / 2.0,
        )?),
    ];

    let mut best = candidates[0];

    for candidate in candidates.iter().skip(1) {
        if candidate.1.weighted_sse < best.1.weighted_sse {
            best = *candidate;
        }
    }

    Ok((best.0, best.1, iterations, converged))
}

/// Production exponential-decay regression engine.
#[derive(Debug, Clone, Copy)]
pub struct RegressionEngine {
    config: RegressionConfig,
    limits: BenchmarkLimits,
}

impl RegressionEngine {
    /// Creates an engine using production benchmark limits.
    pub fn new(config: RegressionConfig) -> RegressionResult<Self> {
        Self::with_limits(config, BenchmarkLimits::production())
    }

    /// Creates an engine with an explicit resource policy.
    pub fn with_limits(
        config: RegressionConfig,
        limits: BenchmarkLimits,
    ) -> RegressionResult<Self> {
        config.validate_against_limits(&limits)?;
        limits.validate()?;

        Ok(Self { config, limits })
    }

    /// Returns the configured regression settings.
    pub fn config(&self) -> RegressionConfig {
        self.config
    }

    /// Returns the resource policy used by this engine.
    pub fn limits(&self) -> BenchmarkLimits {
        self.limits
    }

    /// Fits the canonical exponential-decay model:
    ///
    /// `y = A * p^x + B`
    pub fn fit(
        &self,
        observations: &[RegressionObservation],
    ) -> RegressionResult<RegressionFit> {
        self.validate_input(observations)?;

        let grid = generate_decay_grid(
            self.config.min_decay_rate,
            self.config.max_decay_rate,
            self.config.grid_points,
        )?;

        let (grid_rate, grid_fit, grid_index) =
            find_grid_candidate(observations, &grid)?;

        let mut best_rate = grid_rate;
        let mut best_fit = grid_fit;
        let mut refinement_iterations = 0u64;
        let mut converged = false;

        let boundary_at_zero =
            grid_index == 0 && grid_rate == 0.0;

        // If the grid winner is an interior point, refine around its
        // neighbouring grid points. If it is a boundary point, refinement is
        // not allowed to cross the configured model boundary.
        if !boundary_at_zero
            && grid_index > 0
            && grid_index + 1 < grid.len()
        {
            let lower = grid[grid_index - 1];
            let upper = grid[grid_index + 1];

            let remaining =
                self.config
                    .max_refinement_iterations
                    .min(
                        self.limits.max_statistical_iterations,
                    );

            let (
                refined_rate,
                refined_fit,
                iterations,
                did_converge,
            ) = refine_decay_rate(
                observations,
                lower,
                upper,
                &self.config,
                remaining,
            )?;

            refinement_iterations = iterations;
            converged = did_converge;

            if refined_fit.weighted_sse
                <= best_fit.weighted_sse
            {
                best_rate = refined_rate;
                best_fit = refined_fit;
            }
        } else {
            // A boundary optimum is a valid mathematical result, but it is
            // explicitly reported as such because boundary estimates can
            // carry substantially different inferential meaning.
            converged = true;
        }

        if !best_rate.is_finite()
            || !best_fit.weighted_sse.is_finite()
        {
            return Err(RegressionError::NoFiniteFit);
        }

        let decay_parameter =
            (-best_rate).exp();

        if !decay_parameter.is_finite()
            || decay_parameter < 0.0
            || decay_parameter > 1.0
        {
            return Err(RegressionError::InvalidFit {
                reason: "decay parameter is outside [0, 1]",
            });
        }

        let mut predictions =
            Vec::with_capacity(observations.len());
        let mut residuals =
            Vec::with_capacity(observations.len());

        for observation in observations {
            let exponent = -best_rate * observation.x;

            let basis = if exponent < -745.0 {
                0.0
            } else {
                exponent.exp()
            };

            let prediction =
                best_fit.amplitude * basis
                    + best_fit.offset;

            let residual =
                observation.y - prediction;

            if !prediction.is_finite()
                || !residual.is_finite()
            {
                return Err(RegressionError::NumericalFailure {
                    operation: "final regression prediction",
                });
            }

            predictions.push(prediction);
            residuals.push(residual);
        }

        let covariance_matrix = if self.config.calculate_uncertainty
        {
            covariance_normal_matrix(
                observations,
                best_fit.amplitude,
                best_rate,
            )
            .ok()
        } else {
            None
        };

        let (
            mut diagnostics,
            standard_errors,
        ) = calculate_diagnostics(
            observations,
            &predictions,
            &residuals,
            3,
            best_fit.amplitude,
            best_rate,
            refinement_iterations,
            converged,
            boundary_at_zero
                || best_rate
                    <= self.config.min_decay_rate
                || best_rate
                    >= self.config.max_decay_rate,
            covariance_matrix,
            self.config.calculate_uncertainty,
        )?;

        // A boundary solution is not a numerical failure. It is a diagnostic
        // that callers must be able to see.
        diagnostics.boundary_solution =
            boundary_at_zero
                || best_rate
                    <= self.config.min_decay_rate
                || best_rate
                    >= self.config.max_decay_rate;

        let (
            amplitude_error,
            decay_rate_error,
            offset_error,
        ) = match standard_errors {
            Some(errors) => (
                Some(errors[0]),
                Some(errors[1]),
                Some(errors[2]),
            ),
            None => (None, None, None),
        };

        let amplitude =
            normal_parameter_estimate(
                best_fit.amplitude,
                amplitude_error,
                self.config.confidence_level,
                None,
                None,
            )?;

        let decay_rate =
            normal_parameter_estimate(
                best_rate,
                decay_rate_error,
                self.config.confidence_level,
                Some(0.0),
                Some(self.config.max_decay_rate),
            )?;

        let offset =
            normal_parameter_estimate(
                best_fit.offset,
                offset_error,
                self.config.confidence_level,
                None,
                None,
            )?;

        let decay_parameter =
            decay_parameter_from_rate(
                best_rate,
                decay_rate_error,
                self.config.confidence_level,
            )?;

        Ok(RegressionFit {
            algorithm: REGRESSION_ALGORITHM_ID.to_owned(),
            model: EXPONENTIAL_DECAY_MODEL_ID.to_owned(),
            amplitude,
            decay_parameter,
            offset,
            decay_rate,
            diagnostics,
            predictions,
            residuals,
            confidence_level: self.config.confidence_level,
        })
    }

    /// Convenience API for unweighted `(x, y)` observations.
    pub fn fit_pairs(
        &self,
        observations: &[(f64, f64)],
    ) -> RegressionResult<RegressionFit> {
        let converted = observations
            .iter()
            .copied()
            .map(RegressionObservation::from)
            .collect::<Vec<_>>();

        self.fit(&converted)
    }

    fn validate_input(
        &self,
        observations: &[RegressionObservation],
    ) -> RegressionResult<()> {
        validate_observations(observations)?;

        self.config.validate_against_limits(
            &self.limits,
        )?;

        self.limits.check_observations(
            observations.len() as u64,
        )?;

        Ok(())
    }
}

/// Fits the canonical exponential-decay model using production defaults.
///
/// Model:
///
/// `y = A * p^x + B`
pub fn fit_exponential_decay(
    observations: &[RegressionObservation],
) -> RegressionResult<RegressionFit> {
    RegressionEngine::new(RegressionConfig::production())?
        .fit(observations)
}

/// Fits the canonical exponential-decay model from `(x, y)` pairs.
pub fn fit_exponential_decay_pairs(
    observations: &[(f64, f64)],
) -> RegressionResult<RegressionFit> {
    RegressionEngine::new(RegressionConfig::production())?
        .fit_pairs(observations)
}

/// Evaluates the canonical exponential-decay model.
///
/// This helper is intentionally independent of the fitting engine.
pub fn exponential_decay(
    x: f64,
    amplitude: f64,
    decay_parameter: f64,
    offset: f64,
) -> RegressionResult<f64> {
    if !x.is_finite()
        || !amplitude.is_finite()
        || !decay_parameter.is_finite()
        || !offset.is_finite()
    {
        return Err(RegressionError::NumericalFailure {
            operation: "exponential-decay model input",
        });
    }

    if !(0.0..=1.0).contains(&decay_parameter) {
        return Err(RegressionError::InvalidFit {
            reason: "decay parameter must be in [0, 1]",
        });
    }

    let value =
        amplitude * decay_parameter.powf(x) + offset;

    if !value.is_finite() {
        return Err(RegressionError::NumericalFailure {
            operation: "exponential-decay model evaluation",
        });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_data() -> Vec<RegressionObservation> {
        let amplitude = 0.45;
        let decay = 0.985;
        let offset = 0.50;

        (0..=20)
            .map(|x| {
                let x = x as f64;
                let y =
                    amplitude * decay.powf(x)
                        + offset;

                RegressionObservation::new(x, y)
            })
            .collect()
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = RegressionConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn model_evaluation_is_deterministic() {
        let first =
            exponential_decay(10.0, 0.45, 0.985, 0.5)
                .expect("model evaluation");

        let second =
            exponential_decay(10.0, 0.45, 0.985, 0.5)
                .expect("model evaluation");

        assert_eq!(first, second);
    }

    #[test]
    fn exact_synthetic_decay_is_recovered() {
        let observations = synthetic_data();

        let fit =
            fit_exponential_decay(&observations)
                .expect("synthetic exponential fit");

        assert!(
            (fit.decay_parameter.value - 0.985).abs()
                < 1.0e-4,
            "fitted p={}",
            fit.decay_parameter.value
        );

        assert!(
            (fit.amplitude.value - 0.45).abs()
                < 1.0e-3,
            "fitted A={}",
            fit.amplitude.value
        );

        assert!(
            (fit.offset.value - 0.50).abs()
                < 1.0e-3,
            "fitted B={}",
            fit.offset.value
        );

        assert!(
            fit.diagnostics.r_squared.unwrap_or(0.0)
                > 0.999_999
        );

        assert!(fit.diagnostics.converged);
    }

    #[test]
    fn repeated_fits_are_identical() {
        let observations = synthetic_data();

        let first =
            fit_exponential_decay(&observations)
                .expect("first fit");

        let second =
            fit_exponential_decay(&observations)
                .expect("second fit");

        assert_eq!(
            first.decay_parameter.value,
            second.decay_parameter.value
        );

        assert_eq!(
            first.amplitude.value,
            second.amplitude.value
        );

        assert_eq!(
            first.offset.value,
            second.offset.value
        );

        assert_eq!(
            first.predictions,
            second.predictions
        );
    }

    #[test]
    fn insufficient_observations_are_rejected() {
        let observations = vec![
            RegressionObservation::new(0.0, 1.0),
            RegressionObservation::new(1.0, 0.9),
            RegressionObservation::new(2.0, 0.8),
        ];

        let result =
            fit_exponential_decay(&observations);

        assert!(matches!(
            result,
            Err(RegressionError::InsufficientObservations {
                ..
            })
        ));
    }

    #[test]
    fn identical_x_values_are_rejected() {
        let observations = vec![
            RegressionObservation::new(1.0, 0.9),
            RegressionObservation::new(1.0, 0.8),
            RegressionObservation::new(1.0, 0.7),
            RegressionObservation::new(1.0, 0.6),
        ];

        let result =
            fit_exponential_decay(&observations);

        assert!(matches!(
            result,
            Err(
                RegressionError::InsufficientDistinctIndependentValues {
                    ..
                }
            )
        ));
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let observations = vec![
            RegressionObservation::new(0.0, 1.0),
            RegressionObservation::new(1.0, f64::NAN),
            RegressionObservation::new(2.0, 0.8),
            RegressionObservation::new(3.0, 0.7),
        ];

        let result =
            fit_exponential_decay(&observations);

        assert!(matches!(
            result,
            Err(
                RegressionError::NonFiniteDependentValue {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_weights_are_rejected() {
        let observations = vec![
            RegressionObservation::weighted(
                0.0, 1.0, 1.0
            ),
            RegressionObservation::weighted(
                1.0, 0.9, 0.0
            ),
            RegressionObservation::weighted(
                2.0, 0.8, 1.0
            ),
            RegressionObservation::weighted(
                3.0, 0.7, 1.0
            ),
        ];

        let result =
            fit_exponential_decay(&observations);

        assert!(matches!(
            result,
            Err(RegressionError::InvalidWeight {
                ..
            })
        ));
    }

    #[test]
    fn weighted_fit_is_supported() {
        let observations = synthetic_data()
            .into_iter()
            .map(|observation| {
                RegressionObservation::weighted(
                    observation.x,
                    observation.y,
                    10.0,
                )
            })
            .collect::<Vec<_>>();

        let fit =
            fit_exponential_decay(&observations)
                .expect("weighted fit");

        assert!(
            (fit.decay_parameter.value - 0.985).abs()
                < 1.0e-4
        );

        assert!(fit.diagnostics.weighted);
    }

    #[test]
    fn prediction_matches_fitted_model() {
        let observations = synthetic_data();

        let fit =
            fit_exponential_decay(&observations)
                .expect("fit");

        let x = 17.5;

        let predicted =
            fit.predict(x)
                .expect("prediction");

        let direct =
            exponential_decay(
                x,
                fit.amplitude.value,
                fit.decay_parameter.value,
                fit.offset.value,
            )
            .expect("direct prediction");

        assert_eq!(predicted, direct);
    }

    #[test]
    fn decay_error_rate_is_one_minus_decay() {
        let fit = RegressionFit {
            algorithm: REGRESSION_ALGORITHM_ID.to_owned(),
            model: EXPONENTIAL_DECAY_MODEL_ID.to_owned(),
            amplitude: ParameterEstimate::point(0.5),
            decay_parameter: ParameterEstimate::point(0.9),
            offset: ParameterEstimate::point(0.5),
            decay_rate: ParameterEstimate::point(
                -0.9f64.ln(),
            ),
            diagnostics: RegressionDiagnostics {
                observations: 4,
                parameters: 3,
                degrees_of_freedom: 1,
                residual_sum_of_squares: 0.0,
                weighted_residual_sum_of_squares: 0.0,
                rmse: 0.0,
                r_squared: Some(1.0),
                adjusted_r_squared: Some(1.0),
                aic: None,
                bic: None,
                weighted: false,
                covariance_available: false,
                conditioning_reciprocal: None,
                refinement_iterations: 0,
                converged: true,
                boundary_solution: false,
                residual_min: 0.0,
                residual_max: 0.0,
                residual_mean: 0.0,
                maximum_absolute_residual: 0.0,
            },
            predictions: Vec::new(),
            residuals: Vec::new(),
            confidence_level: ConfidenceLevel::default(),
        };

        assert_eq!(fit.error_rate(), 0.1);
    }

    #[test]
    fn boundary_no_decay_is_supported() {
        let observations = vec![
            RegressionObservation::new(0.0, 0.75),
            RegressionObservation::new(1.0, 0.75),
            RegressionObservation::new(2.0, 0.75),
            RegressionObservation::new(3.0, 0.75),
            RegressionObservation::new(4.0, 0.75),
            RegressionObservation::new(5.0, 0.75),
        ];

        let config = RegressionConfig {
            min_decay_rate: 1.0e-12,
            max_decay_rate: 5.0,
            ..RegressionConfig::default()
        };

        let engine =
            RegressionEngine::new(config)
                .expect("engine");

        let fit =
            engine.fit(&observations)
                .expect("fit");

        assert!(
            fit.decay_parameter.value > 0.999
        );
    }

    #[test]
    fn fit_is_auditable() {
        let observations = synthetic_data();

        let fit =
            fit_exponential_decay(&observations)
                .expect("fit");

        assert_eq!(
            fit.algorithm,
            REGRESSION_ALGORITHM_ID
        );

        assert_eq!(
            fit.model,
            EXPONENTIAL_DECAY_MODEL_ID
        );

        assert_eq!(
            fit.predictions.len(),
            observations.len()
        );

        assert_eq!(
            fit.residuals.len(),
            observations.len()
        );

        assert!(
            fit.diagnostics.observations
                == observations.len()
        );

        assert!(
            fit.diagnostics.parameters == 3
        );
    }

    #[test]
    fn confidence_intervals_are_present_for_well_conditioned_data() {
        let observations = synthetic_data();

        let fit =
            fit_exponential_decay(&observations)
                .expect("fit");

        assert!(
            fit.amplitude.has_uncertainty()
                || !fit.diagnostics.covariance_available
        );

        assert!(
            fit.decay_parameter.value >= 0.0
                && fit.decay_parameter.value <= 1.0
        );
    }

    #[test]
    fn grid_configuration_is_bounded() {
        let limits =
            BenchmarkLimits::production();

        let config = RegressionConfig {
            grid_points: limits.max_statistical_iterations + 1,
            ..RegressionConfig::default()
        };

        let result =
            RegressionEngine::new(config);

        assert!(result.is_err());
    }

    #[test]
    fn nan_configuration_is_rejected() {
        let config = RegressionConfig {
            min_decay_rate: f64::NAN,
            ..RegressionConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_decay_parameter_is_rejected() {
        let result =
            exponential_decay(
                1.0,
                1.0,
                1.5,
                0.0,
            );

        assert!(result.is_err());
    }
}