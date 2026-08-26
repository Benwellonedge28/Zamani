//! Zamani Quantum Benchmarking — QEC Threshold Analysis
//!
//! Production threshold and pseudo-threshold analysis for quantum error
//! correction experiments.
//!
//! # Responsibility
//!
//! This module owns the mathematical interpretation of a QEC threshold
//! experiment.
//!
//! It owns:
//!
//! - physical-error/logical-error observations;
//! - validated threshold data points;
//! - logical-error confidence intervals;
//! - distance-dependent logical-error curves;
//! - pairwise curve-crossing threshold estimation;
//! - robust aggregation of multiple crossing estimates;
//! - finite-size-scaling threshold estimation;
//! - pseudo-threshold estimation;
//! - below-threshold / above-threshold / indeterminate classification;
//! - suppression-direction analysis;
//! - fit-quality diagnostics;
//! - threshold-estimate uncertainty diagnostics;
//! - deterministic threshold reports;
//! - bounded resource validation;
//! - stable schema/version identifiers.
//!
//! It does NOT own:
//!
//! - QEC circuit generation;
//! - surface-code construction;
//! - noise-model generation;
//! - syndrome extraction;
//! - decoding;
//! - QPU execution;
//! - simulation execution;
//! - hardware calibration;
//! - resource admission;
//! - benchmark scheduling;
//! - report serialization;
//! - global benchmark registry;
//! - Zamani-language parsing.
//!
//! Those responsibilities belong to the existing QEC, benchmarking execution,
//! hardware, frontend and reporting layers.
//!
//! # Scientific meaning
//!
//! A QEC threshold is not a universal property of a physical qubit.
//!
//! It is a property of a specified experimental system:
//
//! ```text
//! code family
//!     +
//! syndrome circuit
//!     +
//! noise model
//!     +
//! decoder
//!     +
//! logical observable
//!     +
//! error-rate definition
//!     +
//! distance family
//!     +
//! statistical protocol
//!     +
//! physical-error parameterization
//! ```
//!
//! Consequently, this module never exposes a threshold as an unconditional
//! hardware constant.
//!
//! Every estimate is associated with an explicit `ThresholdContext`.
//!
//! # Threshold versus pseudo-threshold
//!
//! These concepts are deliberately separate.
//!
//! ## Asymptotic / scaling threshold
//!
//! The threshold is the transition region in which increasing code distance
//! changes from improving logical performance to failing to improve it.
//!
//! A practical finite-data estimate is obtained from distance-dependent
//! logical-error curves and/or a finite-size-scaling model.
//!
//! ## Pseudo-threshold
//!
//! A pseudo-threshold is a finite-distance quantity. It is the physical error
//! rate at which the encoded logical error rate crosses a specified baseline,
//! commonly:
//!
//! ```text
//! p_L(d, p) = p
//! ```
//!
//! for a chosen distance `d`.
//!
//! It MUST NOT be reported as the asymptotic threshold.
//!
//! # Finite-size scaling
//!
//! Near a continuous threshold transition, a commonly used local scaling model
//! is:
//!
//! ```text
//! p_L(p, d) = A + B x + C x²
//!
//! x = (p - p_th) d^(1 / nu)
//! ```
//!
//! where:
//!
//! - `p` is physical error rate;
//! - `d` is code distance;
//! - `p_th` is the threshold;
//! - `nu` is a scaling exponent;
//! - `A`, `B`, `C` are local fit coefficients.
//!
//! This implementation performs a bounded deterministic grid search over
//! `p_th` and `nu`, solving `A`, `B`, and `C` analytically through a weighted
//! 3x3 least-squares system.
//!
//! This is intentionally a bounded numerical estimator rather than an
//! unconstrained optimizer. It makes resource usage predictable and keeps the
//! benchmark library dependency-light.
//!
//! # Crossing estimator
//!
//! The crossing estimator independently searches pairs of distance curves for
//! physical-error intervals in which the ordering of logical error rates
//! changes.
//!
//! Each crossing is linearly interpolated between measured points.
//!
//! Multiple pairwise crossings are aggregated using a deterministic median.
//!
//! The spread of pairwise crossings is reported separately from statistical
//! confidence intervals because crossing spread represents model/data
//! disagreement, not binomial sampling uncertainty.
//!
//! # Statistical uncertainty
//!
//! Each logical-error observation is represented by integer:
//!
//! ```text
//! logical_errors
//! total_trials
//! ```
//!
//! and receives a Wilson confidence interval.
//!
//! This module does not silently interpret a curve-crossing spread as a
//! confidence interval.
//!
//! # Pseudothreshold
//!
//! For each distance:
//!
//! ```text
//! p_L(p, d) - baseline(p) = 0
//! ```
//!
//! is solved by linear interpolation between adjacent measurements.
//!
//! The default baseline is:
//!
//! ```text
//! baseline(p) = p
//! ```
//!
//! A custom constant or affine baseline can also be specified.
//!
//! # Production invariants
//!
//! 1. No NaN or infinity enters a public scientific result.
//! 2. Probabilities are constrained to `[0, 1]`.
//! 3. Error counts cannot exceed trial counts.
//! 4. Code distances must be positive.
//! 5. Physical-error points must be strictly ordered within a curve.
//! 6. Duplicate physical-error points are rejected.
//! 7. No public analysis function panics on invalid input.
//! 8. No integer arithmetic wraps.
//! 9. Allocation is bounded by configuration.
//! 10. Threshold and pseudo-threshold remain distinct.
//! 11. Statistical confidence and model-fit uncertainty remain distinct.
//! 12. Non-monotonic or insufficient data is reported rather than silently
//!     converted into a threshold.
//! 13. No diagnostics are printed.
//! 14. No process-global mutable state is used.
//! 15. No hardware-specific assumptions are embedded.
//!
//! # Integration
//!
//! The intended production dependency graph is:
//!
//! ```text
//! quantum::error_correction::simulation
//!                  │
//!                  │ logical failures / trials
//!                  ▼
//! benchmarking::qec::threshold
//!                  │
//!       ┌──────────┼───────────┐
//!       ▼          ▼           ▼
//! crossing     scaling      pseudo-threshold
//! estimator    estimator       estimator
//!       │          │           │
//!       └──────────┼───────────┘
//!                  ▼
//!          ThresholdAnalysis
//!                  │
//!                  ▼
//!       benchmarking::core::result
//!                  │
//!                  ├── reporting
//!                  ├── analysis
//!                  └── regression
//! ```
//!
//! The QEC statistical subsystem already owns general confidence-interval
//! mathematics. This file intentionally contains the small amount of
//! threshold-specific mathematics it requires so that the file remains
//! independently testable and does not force threshold consumers to depend on
//! the entire statistical implementation.
//!
//! Future integration with `quantum::error_correction::statistical` can replace
//! the local Wilson implementation without changing the public threshold data
//! model.
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
//! # Determinism
//!
//! Threshold analysis is deterministic.
//!
//! There is:
//!
//! - no random number generator;
//! - no global state;
//! - no parallel floating-point reduction;
//! - no nondeterministic map iteration.
//!
//! If bootstrap uncertainty is added later, the bootstrap seed and algorithm
//! version must become part of the higher-level benchmark provenance rather
//! than hidden inside this module.
//!
//! # Versioning
//!
//! `THRESHOLD_SCHEMA_VERSION` describes this externally observable mathematical
//! contract. It is independent of the Zamani compiler version and QEC API
//! version.
//!
//! Rust compatibility: Rust 1.97 / 1.97.1.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

// ============================================================================
// Stable public identity
// ============================================================================

/// Stable benchmark identifier.
pub const THRESHOLD_BENCHMARK_ID: &str = "qec_threshold";

/// Stable mathematical-result schema version.
pub const THRESHOLD_SCHEMA_VERSION: u32 = 1;

/// Stable estimator implementation identifier.
pub const THRESHOLD_ESTIMATOR_VERSION: &str =
    "qec-threshold-1.0";

/// Default confidence level for Wilson intervals.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Minimum supported confidence level.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.50;

/// Maximum supported confidence level.
///
/// Exactly `1.0` is rejected because finite-sample confidence intervals
/// require an unbounded critical value at that limit.
pub const MAX_CONFIDENCE_LEVEL: f64 = 0.999_999_999_999;

/// Default minimum number of code distances required for scaling analysis.
pub const DEFAULT_MIN_DISTANCES: usize = 3;

/// Default minimum number of physical-error points per curve.
pub const DEFAULT_MIN_POINTS_PER_CURVE: usize = 3;

/// Default number of threshold grid points.
pub const DEFAULT_THRESHOLD_GRID_POINTS: usize = 101;

/// Default number of scaling-exponent grid points.
pub const DEFAULT_NU_GRID_POINTS: usize = 61;

/// Default lower bound for the finite-size-scaling exponent.
pub const DEFAULT_NU_MIN: f64 = 0.5;

/// Default upper bound for the finite-size-scaling exponent.
pub const DEFAULT_NU_MAX: f64 = 5.0;

/// Maximum permitted threshold-grid points.
pub const MAX_THRESHOLD_GRID_POINTS: usize = 10_001;

/// Maximum permitted scaling-exponent grid points.
pub const MAX_NU_GRID_POINTS: usize = 2_001;

/// Maximum permitted number of input curves.
pub const MAX_CURVES: usize = 1_024;

/// Maximum permitted number of observations in one curve.
pub const MAX_POINTS_PER_CURVE: usize = 100_000;

/// Maximum permitted total observations.
///
/// This is deliberately bounded because threshold fitting is an analysis
/// operation and must not become an accidental unbounded-allocation surface.
pub const MAX_TOTAL_POINTS: usize = 1_000_000;

/// Smallest meaningful physical-error value.
///
/// Zero is mathematically valid and therefore remains allowed.
pub const MIN_PHYSICAL_ERROR: f64 = 0.0;

/// Largest physical-error probability.
pub const MAX_PHYSICAL_ERROR: f64 = 1.0;

/// Numerical tolerance used for equality at probability boundaries.
const PROBABILITY_EPSILON: f64 = 1.0e-15;

/// Numerical tolerance for deciding whether a denominator is effectively zero.
const DENOMINATOR_EPSILON: f64 = 1.0e-15;

/// Numerical tolerance for ordering duplicate physical-error points.
const X_ORDER_EPSILON: f64 = 1.0e-14;

/// Smallest positive finite value used when constructing regression weights.
const MIN_VARIANCE: f64 = 1.0e-18;

// ============================================================================
// Errors
// ============================================================================

/// Error returned by threshold analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ThresholdError {
    /// The supplied configuration is invalid.
    InvalidConfiguration {
        field: &'static str,
        reason: &'static str,
    },

    /// A floating-point input is NaN or infinity.
    NonFinite {
        field: &'static str,
        value: f64,
    },

    /// A probability is outside `[0, 1]`.
    ProbabilityOutOfRange {
        field: &'static str,
        value: f64,
    },

    /// An error count exceeds the trial count.
    ErrorsExceedTrials {
        errors: u64,
        trials: u64,
    },

    /// A distance is invalid.
    InvalidDistance {
        distance: u64,
    },

    /// A curve has no observations.
    EmptyCurve,

    /// Too few observations exist for the requested analysis.
    InsufficientData {
        requirement: &'static str,
        observed: usize,
        required: usize,
    },

    /// More curves were supplied than the bounded analyzer permits.
    TooManyCurves {
        count: usize,
        maximum: usize,
    },

    /// More points were supplied than the bounded analyzer permits.
    TooManyPoints {
        count: usize,
        maximum: usize,
    },

    /// Total observation count exceeds the global bound.
    TotalPointLimitExceeded {
        count: usize,
        maximum: usize,
    },

    /// Physical-error points are not strictly increasing.
    UnsortedOrDuplicatePhysicalError {
        previous: f64,
        current: f64,
    },

    /// A finite-size scaling fit cannot be constructed.
    ScalingFitUnavailable,

    /// A regression matrix is singular or numerically unstable.
    SingularRegression,

    /// A calculated statistic is non-finite.
    NonFiniteStatistic {
        statistic: &'static str,
    },

    /// An interpolation denominator is effectively zero.
    DegenerateInterpolation {
        x1: f64,
        x2: f64,
    },

    /// A threshold could not be identified from the supplied data.
    ThresholdNotIdentifiable {
        reason: &'static str,
    },
}

impl fmt::Display for ThresholdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, reason } => {
                write!(
                    formatter,
                    "invalid threshold configuration `{field}`: {reason}"
                )
            }

            Self::NonFinite { field, value } => {
                write!(formatter, "{field} is non-finite: {value}")
            }

            Self::ProbabilityOutOfRange { field, value } => {
                write!(
                    formatter,
                    "{field} must be in [0, 1], got {value}"
                )
            }

            Self::ErrorsExceedTrials { errors, trials } => {
                write!(
                    formatter,
                    "logical errors {errors} exceed trials {trials}"
                )
            }

            Self::InvalidDistance { distance } => {
                write!(
                    formatter,
                    "code distance must be positive, got {distance}"
                )
            }

            Self::EmptyCurve => {
                write!(formatter, "threshold curve contains no observations")
            }

            Self::InsufficientData {
                requirement,
                observed,
                required,
            } => {
                write!(
                    formatter,
                    "insufficient threshold data for {requirement}: \
                     observed {observed}, required {required}"
                )
            }

            Self::TooManyCurves { count, maximum } => {
                write!(
                    formatter,
                    "too many threshold curves: {count}, maximum {maximum}"
                )
            }

            Self::TooManyPoints { count, maximum } => {
                write!(
                    formatter,
                    "too many points in threshold curve: {count}, maximum {maximum}"
                )
            }

            Self::TotalPointLimitExceeded { count, maximum } => {
                write!(
                    formatter,
                    "threshold analysis contains {count} total points, \
                     maximum {maximum}"
                )
            }

            Self::UnsortedOrDuplicatePhysicalError { previous, current } => {
                write!(
                    formatter,
                    "physical-error points must be strictly increasing: \
                     previous={previous}, current={current}"
                )
            }

            Self::ScalingFitUnavailable => {
                write!(
                    formatter,
                    "finite-size scaling fit is unavailable for the supplied data"
                )
            }

            Self::SingularRegression => {
                write!(
                    formatter,
                    "finite-size scaling regression is singular or \
                     numerically unstable"
                )
            }

            Self::NonFiniteStatistic { statistic } => {
                write!(
                    formatter,
                    "threshold statistic `{statistic}` is non-finite"
                )
            }

            Self::DegenerateInterpolation { x1, x2 } => {
                write!(
                    formatter,
                    "cannot interpolate between degenerate x values \
                     {x1} and {x2}"
                )
            }

            Self::ThresholdNotIdentifiable { reason } => {
                write!(
                    formatter,
                    "threshold is not identifiable: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ThresholdError {}

/// Result alias used throughout this module.
pub type ThresholdResult<T> = Result<T, ThresholdError>;

// ============================================================================
// Confidence intervals
// ============================================================================

/// Confidence interval construction method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfidenceIntervalMethod {
    /// Wilson score interval for a binomial proportion.
    Wilson,
}

impl ConfidenceIntervalMethod {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wilson => "wilson",
        }
    }
}

/// A binomial confidence interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Lower confidence bound.
    pub lower: f64,

    /// Point estimate.
    pub estimate: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Interval method.
    pub method: ConfidenceIntervalMethod,
}

impl ConfidenceInterval {
    /// Returns the interval width.
    #[must_use]
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the half-width.
    #[must_use]
    pub fn half_width(self) -> f64 {
        self.width() / 2.0
    }

    /// Returns whether the interval is valid.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.lower.is_finite()
            && self.estimate.is_finite()
            && self.upper.is_finite()
            && self.lower >= -PROBABILITY_EPSILON
            && self.upper <= 1.0 + PROBABILITY_EPSILON
            && self.lower <= self.estimate + PROBABILITY_EPSILON
            && self.estimate <= self.upper + PROBABILITY_EPSILON
            && self.lower <= self.upper + PROBABILITY_EPSILON
    }
}

// ============================================================================
// Threshold context
// ============================================================================

/// Experimental context identifying what a threshold actually describes.
///
/// This deliberately contains descriptive strings rather than backend-specific
/// types. Threshold analysis must remain independent of hardware providers and
/// QEC implementation types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThresholdContext {
    /// Code-family identifier.
    pub code_family: String,

    /// Syndrome-circuit identifier/version.
    pub syndrome_circuit: String,

    /// Noise-model identifier.
    pub noise_model: String,

    /// Decoder identifier/version.
    pub decoder: String,

    /// Logical observable being measured.
    pub logical_observable: String,

    /// Physical-error-rate definition.
    pub physical_error_definition: String,

    /// Logical-error-rate definition.
    pub logical_error_definition: String,
}

impl ThresholdContext {
    /// Creates a fully specified threshold context.
    pub fn new(
        code_family: impl Into<String>,
        syndrome_circuit: impl Into<String>,
        noise_model: impl Into<String>,
        decoder: impl Into<String>,
        logical_observable: impl Into<String>,
        physical_error_definition: impl Into<String>,
        logical_error_definition: impl Into<String>,
    ) -> ThresholdResult<Self> {
        let context = Self {
            code_family: code_family.into(),
            syndrome_circuit: syndrome_circuit.into(),
            noise_model: noise_model.into(),
            decoder: decoder.into(),
            logical_observable: logical_observable.into(),
            physical_error_definition: physical_error_definition.into(),
            logical_error_definition: logical_error_definition.into(),
        };

        context.validate()?;
        Ok(context)
    }

    /// Validates that all identifying fields are non-empty.
    pub fn validate(&self) -> ThresholdResult<()> {
        let fields = [
            ("code_family", self.code_family.as_str()),
            ("syndrome_circuit", self.syndrome_circuit.as_str()),
            ("noise_model", self.noise_model.as_str()),
            ("decoder", self.decoder.as_str()),
            ("logical_observable", self.logical_observable.as_str()),
            (
                "physical_error_definition",
                self.physical_error_definition.as_str(),
            ),
            (
                "logical_error_definition",
                self.logical_error_definition.as_str(),
            ),
        ];

        for (field, value) in fields {
            if value.trim().is_empty() {
                return Err(ThresholdError::InvalidConfiguration {
                    field,
                    reason: "must not be empty",
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Physical/logical observations
// ============================================================================

/// One measured threshold data point.
///
/// The raw integer counters are retained because they are more reproducible
/// than a probability alone and permit confidence intervals to be recomputed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThresholdPoint {
    /// Number of logical failures.
    pub logical_errors: u64,

    /// Number of logical trials.
    pub trials: u64,

    /// Code distance.
    pub distance: u64,
}

impl ThresholdPoint {
    /// Creates a validated point.
    pub fn new(
        distance: u64,
        logical_errors: u64,
        trials: u64,
    ) -> ThresholdResult<Self> {
        if distance == 0 {
            return Err(ThresholdError::InvalidDistance { distance });
        }

        if trials == 0 {
            return Err(ThresholdError::InsufficientData {
                requirement: "at least one trial",
                observed: 0,
                required: 1,
            });
        }

        if logical_errors > trials {
            return Err(ThresholdError::ErrorsExceedTrials {
                errors: logical_errors,
                trials,
            });
        }

        Ok(Self {
            logical_errors,
            trials,
            distance,
        })
    }

    /// Returns the empirical logical-error probability.
    #[must_use]
    pub fn logical_error_rate(&self) -> f64 {
        self.logical_errors as f64 / self.trials as f64
    }

    /// Returns the Wilson confidence interval.
    pub fn confidence_interval(
        &self,
        confidence_level: f64,
    ) -> ThresholdResult<ConfidenceInterval> {
        wilson_interval(
            self.logical_errors,
            self.trials,
            confidence_level,
        )
    }
}

/// One physical-error/logical-error observation.
///
/// `physical_error_rate` belongs here rather than in `ThresholdPoint` because
/// the same code distance can be evaluated at many physical error rates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdObservation {
    /// Physical error probability/rate represented by the x-axis.
    pub physical_error_rate: f64,

    /// Logical measurement at this physical error rate.
    pub point: ThresholdPoint,
}

impl ThresholdObservation {
    /// Creates a validated observation.
    pub fn new(
        physical_error_rate: f64,
        distance: u64,
        logical_errors: u64,
        trials: u64,
    ) -> ThresholdResult<Self> {
        validate_probability(
            physical_error_rate,
            "physical_error_rate",
        )?;

        Ok(Self {
            physical_error_rate,
            point: ThresholdPoint::new(
                distance,
                logical_errors,
                trials,
            )?,
        })
    }

    /// Returns the logical-error probability.
    #[must_use]
    pub fn logical_error_rate(&self) -> f64 {
        self.point.logical_error_rate()
    }

    /// Returns the confidence interval.
    pub fn confidence_interval(
        &self,
        confidence_level: f64,
    ) -> ThresholdResult<ConfidenceInterval> {
        self.point.confidence_interval(confidence_level)
    }
}

// ============================================================================
// Distance curve
// ============================================================================

/// Logical-error curve for one code distance.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdCurve {
    /// Code distance represented by this curve.
    pub distance: u64,

    /// Ordered physical-error observations.
    observations: Vec<ThresholdObservation>,
}

impl ThresholdCurve {
    /// Creates an empty curve for a distance.
    pub fn new(distance: u64) -> ThresholdResult<Self> {
        if distance == 0 {
            return Err(ThresholdError::InvalidDistance { distance });
        }

        Ok(Self {
            distance,
            observations: Vec::new(),
        })
    }

    /// Creates a curve from observations.
    ///
    /// Observations must be strictly ordered by increasing physical-error rate.
    pub fn from_observations(
        distance: u64,
        observations: Vec<ThresholdObservation>,
    ) -> ThresholdResult<Self> {
        if distance == 0 {
            return Err(ThresholdError::InvalidDistance { distance });
        }

        if observations.is_empty() {
            return Err(ThresholdError::EmptyCurve);
        }

        if observations.len() > MAX_POINTS_PER_CURVE {
            return Err(ThresholdError::TooManyPoints {
                count: observations.len(),
                maximum: MAX_POINTS_PER_CURVE,
            });
        }

        for observation in &observations {
            if observation.point.distance != distance {
                return Err(ThresholdError::InvalidConfiguration {
                    field: "distance",
                    reason: "all observations in a curve must have the same distance",
                });
            }
        }

        for pair in observations.windows(2) {
            let previous = pair[0].physical_error_rate;
            let current = pair[1].physical_error_rate;

            if current <= previous + X_ORDER_EPSILON {
                return Err(
                    ThresholdError::UnsortedOrDuplicatePhysicalError {
                        previous,
                        current,
                    },
                );
            }
        }

        Ok(Self {
            distance,
            observations,
        })
    }

    /// Appends one observation while preserving ordering invariants.
    pub fn push(
        &mut self,
        observation: ThresholdObservation,
    ) -> ThresholdResult<()> {
        if observation.point.distance != self.distance {
            return Err(ThresholdError::InvalidConfiguration {
                field: "distance",
                reason: "observation distance does not match curve distance",
            });
        }

        if self.observations.len() >= MAX_POINTS_PER_CURVE {
            return Err(ThresholdError::TooManyPoints {
                count: self.observations.len() + 1,
                maximum: MAX_POINTS_PER_CURVE,
            });
        }

        if let Some(last) = self.observations.last() {
            if observation.physical_error_rate
                <= last.physical_error_rate + X_ORDER_EPSILON
            {
                return Err(
                    ThresholdError::UnsortedOrDuplicatePhysicalError {
                        previous: last.physical_error_rate,
                        current: observation.physical_error_rate,
                    },
                );
            }
        }

        self.observations.push(observation);
        Ok(())
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the curve is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Returns the observations.
    #[must_use]
    pub fn observations(&self) -> &[ThresholdObservation] {
        &self.observations
    }

    /// Returns the physical-error domain.
    ///
    /// `None` is returned for an empty curve.
    #[must_use]
    pub fn physical_error_range(&self) -> Option<(f64, f64)> {
        let first = self.observations.first()?;
        let last = self.observations.last()?;

        Some((
            first.physical_error_rate,
            last.physical_error_rate,
        ))
    }

    /// Returns an interpolated logical error rate at `physical_error_rate`.
    ///
    /// Exact observations are returned exactly. Values outside the measured
    /// domain are rejected rather than extrapolated.
    pub fn interpolate(
        &self,
        physical_error_rate: f64,
    ) -> ThresholdResult<f64> {
        validate_probability(
            physical_error_rate,
            "physical_error_rate",
        )?;

        if self.observations.is_empty() {
            return Err(ThresholdError::EmptyCurve);
        }

        if let Some(first) = self.observations.first() {
            if approx_equal(
                physical_error_rate,
                first.physical_error_rate,
            ) {
                return Ok(first.logical_error_rate());
            }
        }

        if let Some(last) = self.observations.last() {
            if approx_equal(
                physical_error_rate,
                last.physical_error_rate,
            ) {
                return Ok(last.logical_error_rate());
            }
        }

        for pair in self.observations.windows(2) {
            let left = pair[0];
            let right = pair[1];

            if physical_error_rate >= left.physical_error_rate
                && physical_error_rate <= right.physical_error_rate
            {
                return interpolate_linear(
                    left.physical_error_rate,
                    left.logical_error_rate(),
                    right.physical_error_rate,
                    right.logical_error_rate(),
                    physical_error_rate,
                );
            }
        }

        Err(ThresholdError::ThresholdNotIdentifiable {
            reason: "requested interpolation point lies outside the measured \
                     physical-error domain",
        })
    }

    /// Finds all crossings against a supplied baseline function.
    pub fn crossings_against<F>(
        &self,
        baseline: F,
    ) -> ThresholdResult<Vec<f64>>
    where
        F: Fn(f64) -> f64,
    {
        if self.observations.len() < 2 {
            return Ok(Vec::new());
        }

        let mut crossings = Vec::new();

        for pair in self.observations.windows(2) {
            let left = pair[0];
            let right = pair[1];

            let left_difference =
                left.logical_error_rate()
                    - baseline(left.physical_error_rate);

            let right_difference =
                right.logical_error_rate()
                    - baseline(right.physical_error_rate);

            validate_finite(
                left_difference,
                "baseline_left_difference",
            )?;
            validate_finite(
                right_difference,
                "baseline_right_difference",
            )?;

            if approx_zero(left_difference) {
                crossings.push(left.physical_error_rate);
                continue;
            }

            if approx_zero(right_difference) {
                crossings.push(right.physical_error_rate);
                continue;
            }

            if left_difference.signum()
                != right_difference.signum()
            {
                let x = interpolate_linear(
                    left.physical_error_rate,
                    left_difference,
                    right.physical_error_rate,
                    right_difference,
                    0.0,
                )?;

                crossings.push(x);
            }
        }

        deduplicate_sorted(&mut crossings);

        Ok(crossings)
    }
}

// ============================================================================
// Baseline model
// ============================================================================

/// Baseline used for pseudo-threshold analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Baseline {
    /// Unencoded baseline `p`.
    PhysicalErrorRate,

    /// Constant baseline.
    Constant(f64),

    /// Affine baseline `a + b p`.
    Affine {
        /// Constant term.
        intercept: f64,

        /// Slope.
        slope: f64,
    },
}

impl Default for Baseline {
    fn default() -> Self {
        Self::PhysicalErrorRate
    }
}

impl Baseline {
    /// Validates the baseline.
    pub fn validate(self) -> ThresholdResult<()> {
        match self {
            Self::PhysicalErrorRate => Ok(()),

            Self::Constant(value) => {
                validate_probability(value, "baseline")?;
                Ok(())
            }

            Self::Affine { intercept, slope } => {
                validate_finite(intercept, "baseline_intercept")?;
                validate_finite(slope, "baseline_slope")?;

                Ok(())
            }
        }
    }

    /// Evaluates the baseline.
    pub fn evaluate(self, physical_error_rate: f64) -> f64 {
        match self {
            Self::PhysicalErrorRate => physical_error_rate,

            Self::Constant(value) => value,

            Self::Affine { intercept, slope } => {
                intercept + slope * physical_error_rate
            }
        }
    }
}

// ============================================================================
// Estimator selection
// ============================================================================

/// Threshold estimation method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdEstimatorMethod {
    /// Median of pairwise distance-curve crossings.
    PairwiseCrossing,

    /// Weighted finite-size scaling fit.
    FiniteSizeScaling,

    /// Run both methods and compare them.
    Combined,
}

impl Default for ThresholdEstimatorMethod {
    fn default() -> Self {
        Self::Combined
    }
}

impl ThresholdEstimatorMethod {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PairwiseCrossing => "pairwise_crossing",
            Self::FiniteSizeScaling => "finite_size_scaling",
            Self::Combined => "combined",
        }
    }
}

// ============================================================================
// Threshold configuration
// ============================================================================

/// Production threshold-analysis configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdConfig {
    /// Estimation method.
    pub estimator: ThresholdEstimatorMethod,

    /// Confidence level for per-point Wilson intervals.
    pub confidence_level: f64,

    /// Minimum number of distinct code distances required.
    pub minimum_distances: usize,

    /// Minimum number of physical-error observations required per curve.
    pub minimum_points_per_curve: usize,

    /// Minimum number of curves required for crossing analysis.
    pub minimum_crossing_curves: usize,

    /// Number of threshold grid points for finite-size scaling.
    pub threshold_grid_points: usize,

    /// Number of `nu` grid points for finite-size scaling.
    pub nu_grid_points: usize,

    /// Lower bound of the finite-size scaling `nu` search.
    pub nu_min: f64,

    /// Upper bound of the finite-size scaling `nu` search.
    pub nu_max: f64,

    /// Fraction of the measured physical-error range used by default for
    /// finite-size scaling.
    ///
    /// `1.0` means use the complete measured domain.
    ///
    /// Values below `1.0` focus the fit around the center of the observed
    /// transition region.
    pub scaling_window_fraction: f64,

    /// Whether the analyzer should require the fitted threshold to lie inside
    /// the measured physical-error domain.
    pub require_threshold_in_domain: bool,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            estimator: ThresholdEstimatorMethod::Combined,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            minimum_distances: DEFAULT_MIN_DISTANCES,
            minimum_points_per_curve:
                DEFAULT_MIN_POINTS_PER_CURVE,
            minimum_crossing_curves: 2,
            threshold_grid_points:
                DEFAULT_THRESHOLD_GRID_POINTS,
            nu_grid_points: DEFAULT_NU_GRID_POINTS,
            nu_min: DEFAULT_NU_MIN,
            nu_max: DEFAULT_NU_MAX,
            scaling_window_fraction: 1.0,
            require_threshold_in_domain: true,
        }
    }
}

impl ThresholdConfig {
    /// Validates configuration.
    pub fn validate(self) -> ThresholdResult<()> {
        validate_confidence_level(self.confidence_level)?;

        if self.minimum_distances < 2 {
            return Err(ThresholdError::InvalidConfiguration {
                field: "minimum_distances",
                reason: "must be at least 2",
            });
        }

        if self.minimum_points_per_curve < 2 {
            return Err(ThresholdError::InvalidConfiguration {
                field: "minimum_points_per_curve",
                reason: "must be at least 2",
            });
        }

        if self.minimum_crossing_curves < 2 {
            return Err(ThresholdError::InvalidConfiguration {
                field: "minimum_crossing_curves",
                reason: "must be at least 2",
            });
        }

        if self.threshold_grid_points < 3
            || self.threshold_grid_points > MAX_THRESHOLD_GRID_POINTS
        {
            return Err(ThresholdError::InvalidConfiguration {
                field: "threshold_grid_points",
                reason: "must be between 3 and MAX_THRESHOLD_GRID_POINTS",
            });
        }

        if self.nu_grid_points < 3
            || self.nu_grid_points > MAX_NU_GRID_POINTS
        {
            return Err(ThresholdError::InvalidConfiguration {
                field: "nu_grid_points",
                reason: "must be between 3 and MAX_NU_GRID_POINTS",
            });
        }

        validate_finite(self.nu_min, "nu_min")?;
        validate_finite(self.nu_max, "nu_max")?;

        if self.nu_min <= 0.0 {
            return Err(ThresholdError::InvalidConfiguration {
                field: "nu_min",
                reason: "must be greater than zero",
            });
        }

        if self.nu_max <= self.nu_min {
            return Err(ThresholdError::InvalidConfiguration {
                field: "nu_max",
                reason: "must be greater than nu_min",
            });
        }

        if !self.scaling_window_fraction.is_finite()
            || self.scaling_window_fraction <= 0.0
            || self.scaling_window_fraction > 1.0
        {
            return Err(ThresholdError::InvalidConfiguration {
                field: "scaling_window_fraction",
                reason: "must be finite and in (0, 1]",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Crossing result
// ============================================================================

/// One crossing between two code-distance curves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveCrossing {
    /// Smaller code distance.
    pub lower_distance: u64,

    /// Larger code distance.
    pub upper_distance: u64,

    /// Estimated physical-error crossing.
    pub physical_error_rate: f64,

    /// Absolute logical-error mismatch at the interpolated crossing.
    pub residual: f64,
}

impl CurveCrossing {
    /// Returns whether the crossing is finite and in the unit interval.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.physical_error_rate.is_finite()
            && self.physical_error_rate >= 0.0
            && self.physical_error_rate <= 1.0
            && self.residual.is_finite()
            && self.residual >= 0.0
    }
}

// ============================================================================
// Pairwise crossing summary
// ============================================================================

/// Result of pairwise curve-crossing analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossingAnalysis {
    /// All detected crossings.
    pub crossings: Vec<CurveCrossing>,

    /// Median crossing.
    pub median_threshold: Option<f64>,

    /// Minimum crossing.
    pub minimum_threshold: Option<f64>,

    /// Maximum crossing.
    pub maximum_threshold: Option<f64>,

    /// Median absolute deviation of crossings.
    pub median_absolute_deviation: Option<f64>,
}

impl CrossingAnalysis {
    /// Returns the number of detected crossings.
    #[must_use]
    pub fn crossing_count(&self) -> usize {
        self.crossings.len()
    }

    /// Returns the crossing spread.
    #[must_use]
    pub fn spread(&self) -> Option<f64> {
        match (
            self.minimum_threshold,
            self.maximum_threshold,
        ) {
            (Some(minimum), Some(maximum)) => Some(maximum - minimum),
            _ => None,
        }
    }

    /// Returns whether at least one crossing exists.
    #[must_use]
    pub fn is_identifiable(&self) -> bool {
        self.median_threshold.is_some()
    }
}

// ============================================================================
// Finite-size scaling result
// ============================================================================

/// Result of the finite-size scaling fit.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalingFit {
    /// Estimated threshold.
    pub threshold: f64,

    /// Estimated critical exponent.
    pub nu: f64,

    /// Constant fit coefficient.
    pub a: f64,

    /// Linear fit coefficient.
    pub b: f64,

    /// Quadratic fit coefficient.
    pub c: f64,

    /// Weighted residual sum of squares.
    pub weighted_sse: f64,

    /// Weighted root mean square error.
    pub weighted_rmse: f64,

    /// Coefficient of determination.
    pub r_squared: Option<f64>,

    /// Number of observations used.
    pub observations: usize,

    /// Number of fitted parameters.
    pub parameters: usize,

    /// Number of grid points searched for threshold.
    pub threshold_grid_points: usize,

    /// Number of grid points searched for `nu`.
    pub nu_grid_points: usize,
}

impl ScalingFit {
    /// Returns degrees of freedom.
    #[must_use]
    pub fn degrees_of_freedom(&self) -> usize {
        self.observations.saturating_sub(self.parameters)
    }

    /// Returns whether the fit is numerically valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.threshold.is_finite()
            && self.nu.is_finite()
            && self.nu > 0.0
            && self.a.is_finite()
            && self.b.is_finite()
            && self.c.is_finite()
            && self.weighted_sse.is_finite()
            && self.weighted_rmse.is_finite()
            && self.r_squared.map_or(true, f64::is_finite)
    }
}

// ============================================================================
// Pseudothreshold result
// ============================================================================

/// One finite-distance pseudo-threshold estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct PseudothresholdEstimate {
    /// Code distance.
    pub distance: u64,

    /// Physical error rate at which logical error crosses the baseline.
    pub physical_error_rate: f64,

    /// Baseline model.
    pub baseline: Baseline,

    /// Whether the crossing lies between measured observations.
    pub interpolated: bool,
}

impl PseudothresholdEstimate {
    /// Returns whether the estimate is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.physical_error_rate.is_finite()
            && self.physical_error_rate >= 0.0
            && self.physical_error_rate <= 1.0
    }
}

// ============================================================================
// Threshold classification
// ============================================================================

/// Classification of threshold evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdClassification {
    /// Evidence is consistent with error suppression as distance increases.
    BelowThreshold,

    /// Evidence is consistent with degradation/no suppression as distance
    /// increases.
    AboveThreshold,

    /// The observations do not support a reliable classification.
    Indeterminate,

    /// Curves are non-monotonic enough that a simple classification is unsafe.
    NonMonotonic,
}

impl ThresholdClassification {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BelowThreshold => "below_threshold",
            Self::AboveThreshold => "above_threshold",
            Self::Indeterminate => "indeterminate",
            Self::NonMonotonic => "non_monotonic",
        }
    }
}

// ============================================================================
// Threshold estimate
// ============================================================================

/// Canonical threshold estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdEstimate {
    /// Primary threshold estimate.
    pub threshold: f64,

    /// Estimation method used.
    pub method: ThresholdEstimatorMethod,

    /// Optional crossing-based estimate.
    pub crossing_estimate: Option<f64>,

    /// Optional finite-size scaling estimate.
    pub scaling_estimate: Option<f64>,

    /// Pairwise-crossing spread.
    ///
    /// This is not a confidence interval.
    pub crossing_spread: Option<f64>,

    /// Optional finite-size scaling fit.
    pub scaling_fit: Option<ScalingFit>,

    /// Confidence level attached to the underlying point intervals.
    pub confidence_level: f64,

    /// Minimum physical-error point used by the analysis.
    pub physical_error_min: f64,

    /// Maximum physical-error point used by the analysis.
    pub physical_error_max: f64,
}

impl ThresholdEstimate {
    /// Returns whether the threshold lies within the measured domain.
    #[must_use]
    pub fn is_in_domain(&self) -> bool {
        self.threshold >= self.physical_error_min
            && self.threshold <= self.physical_error_max
    }
}

// ============================================================================
// Full analysis report
// ============================================================================

/// Complete deterministic threshold-analysis result.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdAnalysis {
    /// Stable schema version.
    pub schema_version: u32,

    /// Stable benchmark identifier.
    pub benchmark_id: &'static str,

    /// Estimator implementation version.
    pub estimator_version: &'static str,

    /// Experimental context.
    pub context: ThresholdContext,

    /// Analysis configuration.
    pub configuration: ThresholdConfig,

    /// Distances represented by the input.
    pub distances: Vec<u64>,

    /// Pairwise crossing analysis.
    pub crossings: CrossingAnalysis,

    /// Finite-size scaling fit.
    pub scaling_fit: Option<ScalingFit>,

    /// Primary threshold estimate.
    pub estimate: Option<ThresholdEstimate>,

    /// Pseudo-thresholds for individual distances.
    pub pseudothresholds: Vec<PseudothresholdEstimate>,

    /// Classification at the requested physical error rate, if one was
    /// supplied.
    pub classification: Option<ThresholdClassification>,

    /// Physical error rate used for classification.
    pub classification_physical_error_rate: Option<f64>,

    /// Number of curves analyzed.
    pub curve_count: usize,

    /// Total number of observations analyzed.
    pub observation_count: usize,
}

impl ThresholdAnalysis {
    /// Returns whether a usable threshold was identified.
    #[must_use]
    pub fn has_threshold(&self) -> bool {
        self.estimate.is_some()
    }

    /// Returns the primary threshold, if available.
    #[must_use]
    pub fn threshold(&self) -> Option<f64> {
        self.estimate.as_ref().map(|estimate| estimate.threshold)
    }

    /// Returns whether the analysis is scientifically complete enough to
    /// report a threshold as an estimate.
    #[must_use]
    pub fn is_identifiable(&self) -> bool {
        self.estimate
            .as_ref()
            .map_or(false, ThresholdEstimate::is_in_domain)
    }
}

// ============================================================================
// Analyzer
// ============================================================================

/// Production QEC threshold analyzer.
#[derive(Debug, Clone, Copy)]
pub struct ThresholdAnalyzer {
    configuration: ThresholdConfig,
}

impl ThresholdAnalyzer {
    /// Creates a validated analyzer.
    pub fn new(
        configuration: ThresholdConfig,
    ) -> ThresholdResult<Self> {
        configuration.validate()?;

        Ok(Self { configuration })
    }

    /// Returns the analyzer configuration.
    #[must_use]
    pub const fn configuration(&self) -> ThresholdConfig {
        self.configuration
    }

    /// Runs complete threshold analysis.
    pub fn analyze(
        &self,
        context: ThresholdContext,
        curves: &[ThresholdCurve],
    ) -> ThresholdResult<ThresholdAnalysis> {
        context.validate()?;

        validate_curves(
            curves,
            &self.configuration,
        )?;

        let distances = sorted_unique_distances(curves)?;

        if distances.len() < self.configuration.minimum_distances {
            return Err(ThresholdError::InsufficientData {
                requirement: "minimum number of distinct code distances",
                observed: distances.len(),
                required: self.configuration.minimum_distances,
            });
        }

        let crossing_analysis =
            analyze_pairwise_crossings(curves)?;

        let scaling_fit = match self.configuration.estimator {
            ThresholdEstimatorMethod::PairwiseCrossing => None,

            ThresholdEstimatorMethod::FiniteSizeScaling
            | ThresholdEstimatorMethod::Combined => {
                finite_size_scaling(
                    curves,
                    &self.configuration,
                )
                .ok()
            }
        };

        let crossing_estimate =
            crossing_analysis.median_threshold;

        let primary_estimate = select_primary_estimate(
            self.configuration.estimator,
            crossing_estimate,
            scaling_fit.as_ref().map(|fit| fit.threshold),
            self.configuration.require_threshold_in_domain,
            curves,
            self.configuration.confidence_level,
            &crossing_analysis,
            scaling_fit.as_ref(),
        )?;

        let pseudothresholds =
            pseudothresholds(curves, Baseline::default())?;

        let physical_range =
            global_physical_error_range(curves)?;

        Ok(ThresholdAnalysis {
            schema_version: THRESHOLD_SCHEMA_VERSION,
            benchmark_id: THRESHOLD_BENCHMARK_ID,
            estimator_version: THRESHOLD_ESTIMATOR_VERSION,
            context,
            configuration: self.configuration,
            distances,
            crossings: crossing_analysis,
            scaling_fit,
            estimate: primary_estimate,
            pseudothresholds,
            classification: None,
            classification_physical_error_rate: None,
            curve_count: curves.len(),
            observation_count: curves
                .iter()
                .map(ThresholdCurve::len)
                .sum(),
        })
    }

    /// Runs threshold analysis and classifies a specified physical error rate.
    pub fn analyze_at_physical_error(
        &self,
        context: ThresholdContext,
        curves: &[ThresholdCurve],
        physical_error_rate: f64,
    ) -> ThresholdResult<ThresholdAnalysis> {
        validate_probability(
            physical_error_rate,
            "classification_physical_error_rate",
        )?;

        let mut analysis = self.analyze(context, curves)?;

        let classification =
            classify_at_physical_error(curves, physical_error_rate)?;

        analysis.classification = Some(classification);
        analysis.classification_physical_error_rate =
            Some(physical_error_rate);

        Ok(analysis)
    }

    /// Estimates the pseudo-threshold for one curve.
    pub fn pseudothreshold(
        &self,
        curve: &ThresholdCurve,
        baseline: Baseline,
    ) -> ThresholdResult<Option<PseudothresholdEstimate>> {
        baseline.validate()?;

        let mut estimates =
            pseudo_thresholds_for_curve(curve, baseline)?;

        Ok(estimates.drain(..).next().map(|physical_error_rate| {
            PseudothresholdEstimate {
                distance: curve.distance,
                physical_error_rate,
                baseline,
                interpolated: true,
            }
        }))
    }
}

// ============================================================================
// Validation
// ============================================================================

fn validate_curves(
    curves: &[ThresholdCurve],
    configuration: &ThresholdConfig,
) -> ThresholdResult<()> {
    if curves.len() > MAX_CURVES {
        return Err(ThresholdError::TooManyCurves {
            count: curves.len(),
            maximum: MAX_CURVES,
        });
    }

    if curves.len() < configuration.minimum_distances {
        return Err(ThresholdError::InsufficientData {
            requirement: "distinct distance curves",
            observed: curves.len(),
            required: configuration.minimum_distances,
        });
    }

    let mut total_points = 0usize;

    for curve in curves {
        if curve.is_empty() {
            return Err(ThresholdError::EmptyCurve);
        }

        if curve.len()
            < configuration.minimum_points_per_curve
        {
            return Err(ThresholdError::InsufficientData {
                requirement: "points per distance curve",
                observed: curve.len(),
                required: configuration.minimum_points_per_curve,
            });
        }

        total_points = total_points
            .checked_add(curve.len())
            .ok_or(ThresholdError::TotalPointLimitExceeded {
                count: usize::MAX,
                maximum: MAX_TOTAL_POINTS,
            })?;

        if total_points > MAX_TOTAL_POINTS {
            return Err(
                ThresholdError::TotalPointLimitExceeded {
                    count: total_points,
                    maximum: MAX_TOTAL_POINTS,
                },
            );
        }
    }

    Ok(())
}

fn sorted_unique_distances(
    curves: &[ThresholdCurve],
) -> ThresholdResult<Vec<u64>> {
    let mut distances = Vec::with_capacity(curves.len());

    for curve in curves {
        if !distances.contains(&curve.distance) {
            distances.push(curve.distance);
        }
    }

    distances.sort_unstable();

    Ok(distances)
}

fn global_physical_error_range(
    curves: &[ThresholdCurve],
) -> ThresholdResult<(f64, f64)> {
    let mut minimum = 1.0;
    let mut maximum = 0.0;
    let mut found = false;

    for curve in curves {
        if let Some((curve_min, curve_max)) =
            curve.physical_error_range()
        {
            minimum = minimum.min(curve_min);
            maximum = maximum.max(curve_max);
            found = true;
        }
    }

    if !found {
        return Err(ThresholdError::EmptyCurve);
    }

    Ok((minimum, maximum))
}

// ============================================================================
// Pairwise crossing analysis
// ============================================================================

fn analyze_pairwise_crossings(
    curves: &[ThresholdCurve],
) -> ThresholdResult<CrossingAnalysis> {
    let mut sorted_curves: Vec<&ThresholdCurve> =
        curves.iter().collect();

    sorted_curves.sort_unstable_by_key(|curve| curve.distance);

    let mut crossings = Vec::new();

    for left_index in 0..sorted_curves.len() {
        for right_index in
            (left_index + 1)..sorted_curves.len()
        {
            let lower = sorted_curves[left_index];
            let upper = sorted_curves[right_index];

            crossings.extend(
                pairwise_curve_crossings(lower, upper)?,
            );
        }
    }

    crossings.sort_unstable_by(|left, right| {
        left.physical_error_rate
            .total_cmp(&right.physical_error_rate)
    });

    let values: Vec<f64> = crossings
        .iter()
        .map(|crossing| crossing.physical_error_rate)
        .collect();

    let median_threshold = median(&values);
    let minimum_threshold = values.first().copied();
    let maximum_threshold = values.last().copied();

    let median_absolute_deviation =
        median_absolute_deviation(&values);

    Ok(CrossingAnalysis {
        crossings,
        median_threshold,
        minimum_threshold,
        maximum_threshold,
        median_absolute_deviation,
    })
}

fn pairwise_curve_crossings(
    lower: &ThresholdCurve,
    upper: &ThresholdCurve,
) -> ThresholdResult<Vec<CurveCrossing>> {
    let mut result = Vec::new();

    let mut common_x = Vec::new();

    for observation in lower.observations() {
        let x = observation.physical_error_rate;

        if upper
            .physical_error_range()
            .map_or(false, |(min, max)| {
                x >= min && x <= max
            })
        {
            common_x.push(x);
        }
    }

    for observation in upper.observations() {
        let x = observation.physical_error_rate;

        if lower
            .physical_error_range()
            .map_or(false, |(min, max)| {
                x >= min && x <= max
            })
        {
            common_x.push(x);
        }
    }

    common_x.sort_unstable_by(f64::total_cmp);
    deduplicate_sorted(&mut common_x);

    if common_x.len() < 2 {
        return Ok(result);
    }

    for pair in common_x.windows(2) {
        let x1 = pair[0];
        let x2 = pair[1];

        let lower_y1 = lower.interpolate(x1)?;
        let lower_y2 = lower.interpolate(x2)?;

        let upper_y1 = upper.interpolate(x1)?;
        let upper_y2 = upper.interpolate(x2)?;

        let d1 = lower_y1 - upper_y1;
        let d2 = lower_y2 - upper_y2;

        if approx_zero(d1) {
            result.push(CurveCrossing {
                lower_distance: lower.distance,
                upper_distance: upper.distance,
                physical_error_rate: x1,
                residual: d1.abs(),
            });

            continue;
        }

        if approx_zero(d2) {
            result.push(CurveCrossing {
                lower_distance: lower.distance,
                upper_distance: upper.distance,
                physical_error_rate: x2,
                residual: d2.abs(),
            });

            continue;
        }

        if d1.signum() != d2.signum() {
            let x = interpolate_linear(
                x1,
                d1,
                x2,
                d2,
                0.0,
            )?;

            let lower_value = lower.interpolate(x)?;
            let upper_value = upper.interpolate(x)?;

            result.push(CurveCrossing {
                lower_distance: lower.distance,
                upper_distance: upper.distance,
                physical_error_rate: x,
                residual: (lower_value - upper_value).abs(),
            });
        }
    }

    Ok(result)
}

// ============================================================================
// Primary estimator selection
// ============================================================================

fn select_primary_estimate(
    method: ThresholdEstimatorMethod,
    crossing: Option<f64>,
    scaling: Option<f64>,
    require_domain: bool,
    curves: &[ThresholdCurve],
    confidence_level: f64,
    crossing_analysis: &CrossingAnalysis,
    scaling_fit: Option<&ScalingFit>,
) -> ThresholdResult<Option<ThresholdEstimate>> {
    let (physical_min, physical_max) =
        global_physical_error_range(curves)?;

    let candidate = match method {
        ThresholdEstimatorMethod::PairwiseCrossing => crossing,

        ThresholdEstimatorMethod::FiniteSizeScaling => scaling,

        ThresholdEstimatorMethod::Combined => match (crossing, scaling) {
            (Some(crossing), Some(scaling)) => {
                // The combined estimator is intentionally conservative:
                // when both methods agree, use their midpoint. When they
                // disagree substantially, return the crossing estimate because
                // it remains directly observable and expose the discrepancy
                // through the result fields.
                let tolerance =
                    0.10 * (physical_max - physical_min).max(
                        DENOMINATOR_EPSILON,
                    );

                if (crossing - scaling).abs() <= tolerance {
                    Some((crossing + scaling) / 2.0)
                } else {
                    Some(crossing)
                }
            }

            (Some(crossing), None) => Some(crossing),

            (None, Some(scaling)) => Some(scaling),

            (None, None) => None,
        },
    };

    let Some(threshold) = candidate else {
        return Ok(None);
    };

    validate_probability(threshold, "threshold")?;

    if require_domain
        && (threshold < physical_min
            || threshold > physical_max)
    {
        return Err(ThresholdError::ThresholdNotIdentifiable {
            reason: "estimated threshold lies outside the measured \
                     physical-error domain",
        });
    }

    Ok(Some(ThresholdEstimate {
        threshold,
        method,
        crossing_estimate: crossing,
        scaling_estimate: scaling,
        crossing_spread: crossing_analysis.spread(),
        scaling_fit: scaling_fit.cloned(),
        confidence_level,
        physical_error_min: physical_min,
        physical_error_max: physical_max,
    }))
}

// ============================================================================
// Finite-size scaling
// ============================================================================

fn finite_size_scaling(
    curves: &[ThresholdCurve],
    configuration: &ThresholdConfig,
) -> ThresholdResult<ScalingFit> {
    let (physical_min, physical_max) =
        global_physical_error_range(curves)?;

    if physical_max <= physical_min {
        return Err(ThresholdError::ScalingFitUnavailable);
    }

    let (window_min, window_max) =
        scaling_window(
            physical_min,
            physical_max,
            configuration.scaling_window_fraction,
        );

    let threshold_points =
        evenly_spaced_grid(
            window_min,
            window_max,
            configuration.threshold_grid_points,
        )?;

    let nu_points =
        evenly_spaced_grid(
            configuration.nu_min,
            configuration.nu_max,
            configuration.nu_grid_points,
        )?;

    let mut best: Option<ScalingFit> = None;

    for threshold in threshold_points {
        for nu in &nu_points {
            let fit = fit_scaling_at(
                curves,
                threshold,
                *nu,
                window_min,
                window_max,
            )?;

            let replace = match &best {
                None => true,

                Some(previous) => {
                    fit.weighted_sse < previous.weighted_sse
                }
            };

            if replace {
                best = Some(fit);
            }
        }
    }

    best.ok_or(ThresholdError::ScalingFitUnavailable)
}

fn scaling_window(
    minimum: f64,
    maximum: f64,
    fraction: f64,
) -> (f64, f64) {
    if fraction >= 1.0 {
        return (minimum, maximum);
    }

    let center = (minimum + maximum) / 2.0;
    let half_width =
        (maximum - minimum) * fraction / 2.0;

    (
        (center - half_width).max(0.0),
        (center + half_width).min(1.0),
    )
}

fn fit_scaling_at(
    curves: &[ThresholdCurve],
    threshold: f64,
    nu: f64,
    window_min: f64,
    window_max: f64,
) -> ThresholdResult<ScalingFit> {
    let mut matrix = [[0.0_f64; 3]; 3];
    let mut vector = [0.0_f64; 3];

    let mut observations = Vec::new();

    for curve in curves {
        let distance = curve.distance as f64;

        if distance <= 0.0 {
            return Err(ThresholdError::InvalidDistance {
                distance: curve.distance,
            });
        }

        let distance_scale =
            distance.powf(1.0 / nu);

        for observation in curve.observations() {
            let p = observation.physical_error_rate;

            if p < window_min || p > window_max {
                continue;
            }

            let y = observation.logical_error_rate();

            let variance = binomial_variance(
                y,
                observation.point.trials,
            )?;

            let weight =
                1.0 / variance.max(MIN_VARIANCE);

            let x = (p - threshold) * distance_scale;

            let features = [1.0, x, x * x];

            for row in 0..3 {
                for column in 0..3 {
                    matrix[row][column] +=
                        weight
                            * features[row]
                            * features[column];
                }

                vector[row] += weight * features[row] * y;
            }

            observations.push((
                y,
                weight,
                features,
            ));
        }
    }

    if observations.len() < 3 {
        return Err(ThresholdError::InsufficientData {
            requirement: "at least three observations for finite-size scaling",
            observed: observations.len(),
            required: 3,
        });
    }

    let coefficients =
        solve_3x3(matrix, vector)?;

    let mut weighted_sse = 0.0;
    let mut weighted_total = 0.0;
    let weighted_mean = {
        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (y, weight, _) in &observations {
            numerator += weight * y;
            denominator += weight;
        }

        if denominator <= DENOMINATOR_EPSILON {
            return Err(ThresholdError::SingularRegression);
        }

        numerator / denominator
    };

    for (y, weight, features) in &observations {
        let predicted =
            coefficients[0] * features[0]
                + coefficients[1] * features[1]
                + coefficients[2] * features[2];

        let residual = y - predicted;

        weighted_sse += weight * residual * residual;

        let centered = y - weighted_mean;
        weighted_total += weight * centered * centered;
    }

    validate_finite(
        weighted_sse,
        "weighted_scaling_sse",
    )?;

    let weighted_rmse =
        (weighted_sse / observations.len() as f64).sqrt();

    let r_squared =
        if weighted_total > DENOMINATOR_EPSILON {
            Some(1.0 - weighted_sse / weighted_total)
        } else {
            None
        };

    Ok(ScalingFit {
        threshold,
        nu,
        a: coefficients[0],
        b: coefficients[1],
        c: coefficients[2],
        weighted_sse,
        weighted_rmse,
        r_squared,
        observations: observations.len(),
        parameters: 5,
        threshold_grid_points: 0,
        nu_grid_points: 0,
    })
}

// ============================================================================
// Pseudothreshold analysis
// ============================================================================

/// Calculates pseudo-threshold estimates for all supplied curves.
pub fn pseudothresholds(
    curves: &[ThresholdCurve],
    baseline: Baseline,
) -> ThresholdResult<Vec<PseudothresholdEstimate>> {
    baseline.validate()?;

    let mut result = Vec::new();

    for curve in curves {
        let estimates =
            pseudo_thresholds_for_curve(curve, baseline)?;

        if let Some(first) = estimates.first() {
            result.push(PseudothresholdEstimate {
                distance: curve.distance,
                physical_error_rate: *first,
                baseline,
                interpolated: true,
            });
        }
    }

    result.sort_unstable_by_key(|estimate| estimate.distance);

    Ok(result)
}

fn pseudo_thresholds_for_curve(
    curve: &ThresholdCurve,
    baseline: Baseline,
) -> ThresholdResult<Vec<f64>> {
    let mut crossings = Vec::new();

    if curve.len() < 2 {
        return Ok(crossings);
    }

    for pair in curve.observations().windows(2) {
        let left = pair[0];
        let right = pair[1];

        let left_difference =
            left.logical_error_rate()
                - baseline.evaluate(left.physical_error_rate);

        let right_difference =
            right.logical_error_rate()
                - baseline.evaluate(right.physical_error_rate);

        if approx_zero(left_difference) {
            crossings.push(left.physical_error_rate);
            continue;
        }

        if approx_zero(right_difference) {
            crossings.push(right.physical_error_rate);
            continue;
        }

        if left_difference.signum()
            != right_difference.signum()
        {
            let crossing = interpolate_linear(
                left.physical_error_rate,
                left_difference,
                right.physical_error_rate,
                right_difference,
                0.0,
            )?;

            crossings.push(crossing);
        }
    }

    deduplicate_sorted(&mut crossings);

    Ok(crossings)
}

// ============================================================================
// Classification
// ============================================================================

/// Classifies a physical error rate using distance scaling.
///
/// A rate is considered below threshold only when the ordering of logical
/// error rates is consistently improving with increasing distance.
///
/// It is considered above threshold when larger distances consistently worsen
/// logical error rates.
///
/// Mixed ordering is reported as non-monotonic/indeterminate rather than
/// silently forcing a binary decision.
pub fn classify_at_physical_error(
    curves: &[ThresholdCurve],
    physical_error_rate: f64,
) -> ThresholdResult<ThresholdClassification> {
    validate_probability(
        physical_error_rate,
        "physical_error_rate",
    )?;

    if curves.len() < 2 {
        return Err(ThresholdError::InsufficientData {
            requirement: "at least two distance curves",
            observed: curves.len(),
            required: 2,
        });
    }

    let mut ordered: Vec<&ThresholdCurve> =
        curves.iter().collect();

    ordered.sort_unstable_by_key(|curve| curve.distance);

    let mut improvements = 0usize;
    let mut degradations = 0usize;

    for pair in ordered.windows(2) {
        let left = pair[0];
        let right = pair[1];

        let Some((left_min, left_max)) =
            left.physical_error_range()
        else {
            continue;
        };

        let Some((right_min, right_max)) =
            right.physical_error_range()
        else {
            continue;
        };

        if physical_error_rate < left_min
            || physical_error_rate > left_max
            || physical_error_rate < right_min
            || physical_error_rate > right_max
        {
            continue;
        }

        let left_error =
            left.interpolate(physical_error_rate)?;

        let right_error =
            right.interpolate(physical_error_rate)?;

        if right_error + PROBABILITY_EPSILON
            < left_error
        {
            improvements += 1;
        } else if right_error
            > left_error + PROBABILITY_EPSILON
        {
            degradations += 1;
        }
    }

    if improvements == 0 && degradations == 0 {
        return Ok(ThresholdClassification::Indeterminate);
    }

    if improvements > 0 && degradations > 0 {
        return Ok(ThresholdClassification::NonMonotonic);
    }

    if improvements > 0 {
        return Ok(ThresholdClassification::BelowThreshold);
    }

    Ok(ThresholdClassification::AboveThreshold)
}

// ============================================================================
// Wilson interval
// ============================================================================

/// Computes a Wilson score interval for a binomial proportion.
pub fn wilson_interval(
    successes: u64,
    trials: u64,
    confidence_level: f64,
) -> ThresholdResult<ConfidenceInterval> {
    if trials == 0 {
        return Err(ThresholdError::InsufficientData {
            requirement: "non-zero trials",
            observed: 0,
            required: 1,
        });
    }

    if successes > trials {
        return Err(ThresholdError::ErrorsExceedTrials {
            errors: successes,
            trials,
        });
    }

    validate_confidence_level(confidence_level)?;

    let p = successes as f64 / trials as f64;

    let alpha = 1.0 - confidence_level;

    let z =
        standard_normal_quantile(1.0 - alpha / 2.0)?;

    let n = trials as f64;
    let z2 = z * z;

    let denominator = 1.0 + z2 / n;

    if denominator <= DENOMINATOR_EPSILON {
        return Err(ThresholdError::NonFiniteStatistic {
            statistic: "wilson_denominator",
        });
    }

    let center =
        (p + z2 / (2.0 * n)) / denominator;

    let margin =
        z
            * ((p * (1.0 - p) / n)
                + z2 / (4.0 * n * n))
                .sqrt()
            / denominator;

    let lower = (center - margin).max(0.0);
    let upper = (center + margin).min(1.0);

    let interval = ConfidenceInterval {
        lower,
        estimate: p,
        upper,
        confidence_level,
        method: ConfidenceIntervalMethod::Wilson,
    };

    if !interval.is_valid() {
        return Err(ThresholdError::NonFiniteStatistic {
            statistic: "wilson_interval",
        });
    }

    Ok(interval)
}

// ============================================================================
// Numerical helpers
// ============================================================================

fn validate_probability(
    value: f64,
    field: &'static str,
) -> ThresholdResult<f64> {
    if !value.is_finite() {
        return Err(ThresholdError::NonFinite {
            field,
            value,
        });
    }

    if value < -PROBABILITY_EPSILON
        || value > 1.0 + PROBABILITY_EPSILON
    {
        return Err(ThresholdError::ProbabilityOutOfRange {
            field,
            value,
        });
    }

    Ok(value.max(0.0).min(1.0))
}

fn validate_confidence_level(
    value: f64,
) -> ThresholdResult<()> {
    if !value.is_finite()
        || value < MIN_CONFIDENCE_LEVEL
        || value > MAX_CONFIDENCE_LEVEL
    {
        return Err(ThresholdError::InvalidConfiguration {
            field: "confidence_level",
            reason:
                "must be finite and within the supported confidence range",
        });
    }

    Ok(())
}

fn validate_finite(
    value: f64,
    statistic: &'static str,
) -> ThresholdResult<()> {
    if !value.is_finite() {
        return Err(ThresholdError::NonFiniteStatistic {
            statistic,
        });
    }

    Ok(())
}

fn approx_zero(value: f64) -> bool {
    value.abs() <= PROBABILITY_EPSILON
}

fn approx_equal(left: f64, right: f64) -> bool {
    (left - right).abs() <= X_ORDER_EPSILON
}

fn interpolate_linear(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    target_y: f64,
) -> ThresholdResult<f64> {
    let denominator = y2 - y1;

    if denominator.abs() <= DENOMINATOR_EPSILON {
        return Err(
            ThresholdError::DegenerateInterpolation {
                x1,
                x2,
            },
        );
    }

    let fraction =
        (target_y - y1) / denominator;

    let x = x1 + fraction * (x2 - x1);

    validate_finite(x, "interpolated_threshold")?;

    Ok(x)
}

fn binomial_variance(
    probability: f64,
    trials: u64,
) -> ThresholdResult<f64> {
    validate_probability(
        probability,
        "logical_error_probability",
    )?;

    if trials == 0 {
        return Err(ThresholdError::InsufficientData {
            requirement: "non-zero trials",
            observed: 0,
            required: 1,
        });
    }

    let n = trials as f64;

    let variance =
        probability * (1.0 - probability) / n;

    validate_finite(
        variance,
        "binomial_variance",
    )?;

    Ok(variance)
}

// ============================================================================
// 3x3 regression solver
// ============================================================================

fn solve_3x3(
    mut matrix: [[f64; 3]; 3],
    mut vector: [f64; 3],
) -> ThresholdResult<[f64; 3]> {
    for pivot in 0..3 {
        let mut pivot_row = pivot;
        let mut pivot_abs =
            matrix[pivot][pivot].abs();

        for row in (pivot + 1)..3 {
            let candidate =
                matrix[row][pivot].abs();

            if candidate > pivot_abs {
                pivot_abs = candidate;
                pivot_row = row;
            }
        }

        if pivot_abs <= DENOMINATOR_EPSILON {
            return Err(ThresholdError::SingularRegression);
        }

        if pivot_row != pivot {
            matrix.swap(pivot, pivot_row);
            vector.swap(pivot, pivot_row);
        }

        for row in (pivot + 1)..3 {
            let factor =
                matrix[row][pivot]
                    / matrix[pivot][pivot];

            for column in pivot..3 {
                matrix[row][column] -=
                    factor * matrix[pivot][column];
            }

            vector[row] -=
                factor * vector[pivot];
        }
    }

    let mut solution = [0.0; 3];

    for row in (0..3).rev() {
        let mut value = vector[row];

        for column in (row + 1)..3 {
            value -=
                matrix[row][column]
                    * solution[column];
        }

        let denominator =
            matrix[row][row];

        if denominator.abs()
            <= DENOMINATOR_EPSILON
        {
            return Err(ThresholdError::SingularRegression);
        }

        solution[row] =
            value / denominator;

        validate_finite(
            solution[row],
            "regression_coefficient",
        )?;
    }

    Ok(solution)
}

// ============================================================================
// Deterministic grid generation
// ============================================================================

fn evenly_spaced_grid(
    minimum: f64,
    maximum: f64,
    count: usize,
) -> ThresholdResult<Vec<f64>> {
    if count < 2 {
        return Err(ThresholdError::InvalidConfiguration {
            field: "grid_points",
            reason: "must be at least two",
        });
    }

    validate_finite(minimum, "grid_minimum")?;
    validate_finite(maximum, "grid_maximum")?;

    if maximum <= minimum {
        return Err(ThresholdError::InvalidConfiguration {
            field: "grid_range",
            reason: "maximum must be greater than minimum",
        });
    }

    let denominator =
        (count - 1) as f64;

    let step =
        (maximum - minimum) / denominator;

    let mut values =
        Vec::with_capacity(count);

    for index in 0..count {
        let value =
            minimum + step * index as f64;

        validate_finite(value, "grid_value")?;

        values.push(value);
    }

    Ok(values)
}

// ============================================================================
// Robust aggregation
// ============================================================================

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sorted =
        values.to_vec();

    sorted.sort_unstable_by(f64::total_cmp);

    let middle =
        sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        Some(
            (sorted[middle - 1] + sorted[middle])
                / 2.0,
        )
    } else {
        Some(sorted[middle])
    }
}

fn median_absolute_deviation(
    values: &[f64],
) -> Option<f64> {
    let center = median(values)?;

    let deviations: Vec<f64> =
        values
            .iter()
            .map(|value| (value - center).abs())
            .collect();

    median(&deviations)
}

fn deduplicate_sorted(values: &mut Vec<f64>) {
    if values.len() < 2 {
        return;
    }

    let mut write_index = 1usize;

    for read_index in 1..values.len() {
        let previous =
            values[write_index - 1];

        let current =
            values[read_index];

        if !approx_equal(previous, current) {
            values[write_index] = current;
            write_index += 1;
        }
    }

    values.truncate(write_index);
}

// ============================================================================
// Standard-normal inverse CDF
// ============================================================================

/// Inverse standard-normal cumulative distribution function.
///
/// This is the Acklam rational approximation, implemented locally so the
/// threshold module does not require a numerical dependency merely for a
/// confidence interval.
///
/// The implementation is deterministic and validated at the public boundary.
fn standard_normal_quantile(
    probability: f64,
) -> ThresholdResult<f64> {
    if !probability.is_finite()
        || probability <= 0.0
        || probability >= 1.0
    {
        return Err(ThresholdError::InvalidConfiguration {
            field: "normal_quantile_probability",
            reason: "must be finite and strictly between zero and one",
        });
    }

    // Coefficients from Peter J. Acklam's inverse-normal approximation.
    const A: [f64; 6] = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.383577518672690e2,
        -3.066479806614716e1,
        2.506628277459239,
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
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        7.784695709041462e-3,
        3.224671290700398e-1,
        2.445134137142996,
        3.754408661907416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    let result;

    if probability < LOW {
        let q =
            (-2.0 * probability.ln()).sqrt();

        result =
            (((((C[0] * q + C[1]) * q + C[2])
                * q
                + C[3])
                * q
                + C[4])
                * q
                + C[5])
                / ((((D[0] * q + D[1]) * q + D[2])
                    * q
                    + D[3])
                    * q
                    + 1.0);
    } else if probability <= HIGH {
        let q =
            probability - 0.5;

        let r = q * q;

        result =
            (((((A[0] * r + A[1]) * r + A[2])
                * r
                + A[3])
                * r
                + A[4])
                * r
                + A[5])
                * q
                / (((((B[0] * r + B[1]) * r + B[2])
                    * r
                    + B[3])
                    * r
                    + B[4])
                    * r
                    + 1.0);
    } else {
        let q =
            (-2.0 * (1.0 - probability).ln()).sqrt();

        result =
            -(((((C[0] * q + C[1]) * q + C[2])
                * q
                + C[3])
                * q
                + C[4])
                * q
                + C[5])
                / ((((D[0] * q + D[1]) * q + D[2])
                    * q
                    + D[3])
                    * q
                    + 1.0);
    }

    validate_finite(
        result,
        "standard_normal_quantile",
    )?;

    Ok(result)
}

// ============================================================================
// Convenience constructors
// ============================================================================

/// Builds a threshold curve from compact tuples.
///
/// Each tuple is:
///
/// ```text
/// (physical_error_rate, logical_errors, trials)
/// ```
pub fn curve_from_counts(
    distance: u64,
    observations: &[
        (f64, u64, u64),
    ],
) -> ThresholdResult<ThresholdCurve> {
    let mut curve =
        ThresholdCurve::new(distance)?;

    for &(physical_error_rate, logical_errors, trials)
        in observations
    {
        curve.push(ThresholdObservation::new(
            physical_error_rate,
            distance,
            logical_errors,
            trials,
        )?)?;
    }

    Ok(curve)
}

/// Performs a complete threshold analysis with default configuration.
pub fn analyze_threshold(
    context: ThresholdContext,
    curves: &[ThresholdCurve],
) -> ThresholdResult<ThresholdAnalysis> {
    ThresholdAnalyzer::new(
        ThresholdConfig::default(),
    )?
    .analyze(context, curves)
}

/// Performs a complete threshold analysis at a specified physical error rate.
pub fn analyze_threshold_at(
    context: ThresholdContext,
    curves: &[ThresholdCurve],
    physical_error_rate: f64,
) -> ThresholdResult<ThresholdAnalysis> {
    ThresholdAnalyzer::new(
        ThresholdConfig::default(),
    )?
    .analyze_at_physical_error(
        context,
        curves,
        physical_error_rate,
    )
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ThresholdContext {
        ThresholdContext::new(
            "surface_code",
            "standard_circuit",
            "depolarizing",
            "mwpm",
            "logical_z",
            "per-cycle physical Pauli error",
            "logical failure per correction cycle",
        )
        .expect("valid test context")
    }

    fn sample_curve(
        distance: u64,
        rates: &[(f64, u64, u64)],
    ) -> ThresholdCurve {
        curve_from_counts(distance, rates)
            .expect("valid test curve")
    }

    #[test]
    fn validates_threshold_point() {
        assert!(ThresholdPoint::new(3, 10, 100).is_ok());

        assert!(
            ThresholdPoint::new(0, 10, 100).is_err()
        );

        assert!(
            ThresholdPoint::new(3, 101, 100).is_err()
        );
    }

    #[test]
    fn wilson_interval_is_valid() {
        let interval =
            wilson_interval(50, 100, 0.95)
                .expect("valid Wilson interval");

        assert!(interval.is_valid());
        assert!(interval.lower < 0.5);
        assert!(interval.upper > 0.5);
    }

    #[test]
    fn wilson_interval_handles_zero_errors() {
        let interval =
            wilson_interval(0, 100, 0.95)
                .expect("valid Wilson interval");

        assert_eq!(interval.estimate, 0.0);
        assert_eq!(interval.lower, 0.0);
        assert!(interval.upper > 0.0);
    }

    #[test]
    fn wilson_interval_handles_all_errors() {
        let interval =
            wilson_interval(100, 100, 0.95)
                .expect("valid Wilson interval");

        assert_eq!(interval.estimate, 1.0);
        assert_eq!(interval.upper, 1.0);
        assert!(interval.lower < 1.0);
    }

    #[test]
    fn curve_requires_strictly_increasing_x() {
        let result = curve_from_counts(
            3,
            &[
                (0.01, 10, 100),
                (0.01, 20, 100),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn curve_interpolation_is_deterministic() {
        let curve = sample_curve(
            3,
            &[
                (0.01, 10, 100),
                (0.02, 20, 100),
            ],
        );

        let result =
            curve.interpolate(0.015)
                .expect("interpolation should succeed");

        assert!((result - 0.15).abs() < 1.0e-12);
    }

    #[test]
    fn pseudo_threshold_matches_physical_baseline() {
        let curve = sample_curve(
            3,
            &[
                (0.01, 2, 1000),
                (0.02, 25, 1000),
                (0.03, 40, 1000),
            ],
        );

        let values =
            pseudo_thresholds_for_curve(
                &curve,
                Baseline::PhysicalErrorRate,
            )
            .expect("pseudo threshold analysis");

        assert_eq!(values.len(), 2);
        assert!(values[0] >= 0.01);
        assert!(values[0] <= 0.02);
    }

    #[test]
    fn pairwise_crossing_detects_transition() {
        let lower = sample_curve(
            3,
            &[
                (0.01, 50, 1000),
                (0.02, 80, 1000),
                (0.03, 120, 1000),
            ],
        );

        let upper = sample_curve(
            5,
            &[
                (0.01, 20, 1000),
                (0.02, 60, 1000),
                (0.03, 160, 1000),
            ],
        );

        let analysis =
            analyze_pairwise_crossings(&[lower, upper])
                .expect("crossing analysis");

        assert_eq!(analysis.crossing_count(), 1);

        let threshold =
            analysis.median_threshold
                .expect("crossing exists");

        assert!(threshold > 0.02);
        assert!(threshold < 0.03);
    }

    #[test]
    fn classification_below_threshold_when_larger_distance_is_better() {
        let curves = vec![
            sample_curve(
                3,
                &[
                    (0.01, 20, 1000),
                    (0.02, 40, 1000),
                    (0.03, 70, 1000),
                ],
            ),
            sample_curve(
                5,
                &[
                    (0.01, 10, 1000),
                    (0.02, 20, 1000),
                    (0.03, 40, 1000),
                ],
            ),
            sample_curve(
                7,
                &[
                    (0.01, 5, 1000),
                    (0.02, 10, 1000),
                    (0.03, 20, 1000),
                ],
            ),
        ];

        let classification =
            classify_at_physical_error(
                &curves,
                0.02,
            )
            .expect("classification");

        assert_eq!(
            classification,
            ThresholdClassification::BelowThreshold
        );
    }

    #[test]
    fn classification_above_threshold_when_larger_distance_is_worse() {
        let curves = vec![
            sample_curve(
                3,
                &[
                    (0.01, 5, 1000),
                    (0.02, 10, 1000),
                    (0.03, 20, 1000),
                ],
            ),
            sample_curve(
                5,
                &[
                    (0.01, 10, 1000),
                    (0.02, 20, 1000),
                    (0.03, 40, 1000),
                ],
            ),
            sample_curve(
                7,
                &[
                    (0.01, 20, 1000),
                    (0.02, 40, 1000),
                    (0.03, 80, 1000),
                ],
            ),
        ];

        let classification =
            classify_at_physical_error(
                &curves,
                0.02,
            )
            .expect("classification");

        assert_eq!(
            classification,
            ThresholdClassification::AboveThreshold
        );
    }

    #[test]
    fn classification_detects_mixed_scaling() {
        let curves = vec![
            sample_curve(
                3,
                &[
                    (0.01, 10, 1000),
                    (0.02, 30, 1000),
                ],
            ),
            sample_curve(
                5,
                &[
                    (0.01, 5, 1000),
                    (0.02, 40, 1000),
                ],
            ),
            sample_curve(
                7,
                &[
                    (0.01, 20, 1000),
                    (0.02, 20, 1000),
                ],
            ),
        ];

        let classification =
            classify_at_physical_error(
                &curves,
                0.02,
            )
            .expect("classification");

        assert_eq!(
            classification,
            ThresholdClassification::NonMonotonic
        );
    }

    #[test]
    fn analyzer_requires_multiple_distances() {
        let configuration = ThresholdConfig {
            minimum_distances: 2,
            ..ThresholdConfig::default()
        };

        let analyzer =
            ThresholdAnalyzer::new(configuration)
                .expect("valid configuration");

        let curve = sample_curve(
            3,
            &[
                (0.01, 10, 1000),
                (0.02, 20, 1000),
                (0.03, 40, 1000),
            ],
        );

        assert!(
            analyzer
                .analyze(context(), &[curve])
                .is_err()
        );
    }

    #[test]
    fn finite_size_scaling_produces_finite_fit() {
        let curves = vec![
            sample_curve(
                3,
                &[
                    (0.005, 80, 10000),
                    (0.010, 120, 10000),
                    (0.015, 180, 10000),
                    (0.020, 260, 10000),
                    (0.025, 360, 10000),
                ],
            ),
            sample_curve(
                5,
                &[
                    (0.005, 50, 10000),
                    (0.010, 80, 10000),
                    (0.015, 130, 10000),
                    (0.020, 220, 10000),
                    (0.025, 350, 10000),
                ],
            ),
            sample_curve(
                7,
                &[
                    (0.005, 30, 10000),
                    (0.010, 50, 10000),
                    (0.015, 90, 10000),
                    (0.020, 170, 10000),
                    (0.025, 330, 10000),
                ],
            ),
        ];

        let configuration = ThresholdConfig {
            estimator:
                ThresholdEstimatorMethod::FiniteSizeScaling,
            threshold_grid_points: 21,
            nu_grid_points: 11,
            ..ThresholdConfig::default()
        };

        let analyzer =
            ThresholdAnalyzer::new(configuration)
                .expect("valid analyzer");

        let analysis =
            analyzer
                .analyze(context(), &curves)
                .expect("analysis should succeed");

        let fit =
            analysis.scaling_fit
                .expect("scaling fit should exist");

        assert!(fit.is_valid());
        assert!(fit.threshold >= 0.005);
        assert!(fit.threshold <= 0.025);
        assert!(fit.nu > 0.0);
    }

    #[test]
    fn combined_analysis_is_reproducible() {
        let curves = vec![
            sample_curve(
                3,
                &[
                    (0.005, 30, 10000),
                    (0.010, 50, 10000),
                    (0.015, 90, 10000),
                    (0.020, 170, 10000),
                    (0.025, 330, 10000),
                ],
            ),
            sample_curve(
                5,
                &[
                    (0.005, 20, 10000),
                    (0.010, 35, 10000),
                    (0.015, 65, 10000),
                    (0.020, 140, 10000),
                    (0.025, 310, 10000),
                ],
            ),
            sample_curve(
                7,
                &[
                    (0.005, 10, 10000),
                    (0.010, 20, 10000),
                    (0.015, 45, 10000),
                    (0.020, 110, 10000),
                    (0.025, 290, 10000),
                ],
            ),
        ];

        let analyzer =
            ThresholdAnalyzer::new(
                ThresholdConfig {
                    threshold_grid_points: 31,
                    nu_grid_points: 15,
                    ..ThresholdConfig::default()
                },
            )
            .expect("valid analyzer");

        let first =
            analyzer
                .analyze(context(), &curves)
                .expect("first analysis");

        let second =
            analyzer
                .analyze(context(), &curves)
                .expect("second analysis");

        assert_eq!(first, second);
    }

    #[test]
    fn default_baseline_is_physical_error_rate() {
        assert_eq!(
            Baseline::default(),
            Baseline::PhysicalErrorRate
        );
    }

    #[test]
    fn standard_normal_quantile_is_reasonable() {
        let q =
            standard_normal_quantile(0.975)
                .expect("quantile");

        assert!((q - 1.959963).abs() < 1.0e-4);
    }

    #[test]
    fn grid_generation_is_deterministic() {
        let grid =
            evenly_spaced_grid(0.0, 1.0, 5)
                .expect("grid");

        assert_eq!(
            grid,
            vec![0.0, 0.25, 0.5, 0.75, 1.0]
        );
    }

    #[test]
    fn context_rejects_empty_fields() {
        assert!(
            ThresholdContext::new(
                "",
                "circuit",
                "noise",
                "decoder",
                "z",
                "physical",
                "logical",
            )
            .is_err()
        );
    }
}