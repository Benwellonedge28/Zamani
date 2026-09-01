//! Zamani Quantum Noise (ZQN) — Characterization Uncertainty.
//!
//! # Purpose
//!
//! This module owns the mathematical representation and calculation of
//! statistical uncertainty for characterization results.
//!
//! It answers:
//!
//! > "Given an explicitly defined estimator/sufficient-statistics contract,
//! > how uncertain is the resulting quantity?"
//!
//! This module deliberately separates:
//!
//! - point estimation;
//! - statistical uncertainty;
//! - systematic/calibration uncertainty;
//! - confidence intervals;
//! - credible intervals;
//! - deterministic mathematical bounds;
//! - approximation error.
//!
//! A characterization observation is evidence.
//! An estimator turns evidence into an estimate.
//! This module quantifies uncertainty around that estimate.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - uncertainty method identifiers;
//! - confidence-level validation;
//! - uncertainty input contracts;
//! - confidence intervals;
//! - credible intervals;
//! - deterministic bounded-error intervals;
//! - standard-error calculations;
//! - Wilson binomial intervals;
//! - Clopper-Pearson exact binomial intervals;
//! - Beta posterior credible intervals;
//! - normal-approximation intervals;
//! - Hoeffding bounded-mean intervals;
//! - Poisson-rate normal-approximation intervals;
//! - weighted-mean effective sample size;
//! - numerical primitives required by those methods;
//! - uncertainty calculation policy;
//! - uncertainty result contracts;
//! - numerical/resource validation;
//! - uncertainty schema identity.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - raw characterization observations;
//! - observation IDs;
//! - experiment generation;
//! - characterization protocols;
//! - point estimators;
//! - canonical Quantum IR;
//! - qubit identity;
//! - quantum channels;
//! - noise models;
//! - calibration storage;
//! - hardware APIs;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - random-number generation;
//! - Bayesian model selection;
//! - MCMC;
//! - tomography reconstruction;
//! - randomized-benchmarking protocol design;
//! - serialization wire formats;
//! - cryptographic hashing;
//! - vendor-specific numerical backends.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! characterization::observation
//!             |
//!             v
//! characterization::estimator
//!             |
//!             | sufficient statistics
//!             v
//! characterization::uncertainty
//!             |
//!       +-----+----------------------+
//!       |                            |
//!       v                            v
//! confidence/credible interval   deterministic bound
//!       |                            |
//!       +-------------+--------------+
//!                     |
//!                     v
//!             characterization result
//!                     |
//!          +----------+-----------+
//!          |                      |
//!          v                      v
//!       calibration           ZQN noise model
//! ```
//!
//! # Fundamental separation
//!
//! A point estimate is not an uncertainty estimate.
//!
//! For example:
//!
//! ```text
//! successes = 501
//! trials    = 1000
//!
//! point estimate = 0.501
//! ```
//!
//! does NOT imply a particular confidence interval until an uncertainty
//! method and confidence level have been explicitly selected.
//!
//! Likewise:
//!
//! ```text
//! calibration uncertainty
//! ```
//!
//! is not automatically equivalent to:
//!
//! ```text
//! sampling uncertainty
//! ```
//!
//! This module keeps those concepts separate.
//!
//! # Scientific correctness
//!
//! No uncertainty method silently:
//!
//! - changes an estimator;
//! - clamps invalid probabilities;
//! - converts NaN to zero;
//! - converts infinity to a finite value;
//! - treats a confidence interval as a probability that a fixed parameter
//!   lies inside the interval;
//! - treats a credible interval as a frequentist confidence interval;
//! - treats statistical uncertainty as systematic uncertainty.
//!
//! Every approximation is explicitly represented in the result.
//!
//! # Approximation policy
//!
//! The caller chooses whether approximate methods are permitted.
//!
//! ```text
//! ExactOnly
//!     |
//!     +--> exact-supported method required
//!
//! AllowApproximation
//!     |
//!     +--> approximation permitted
//! ```
//!
//! A normal approximation is therefore never silently substituted for an
//! exact method.
//!
//! # Scalability
//!
//! There is no semantic limit on:
//!
//! - number of shots;
//! - number of experiments;
//! - number of characterized resources;
//! - number of observations;
//! - number of qubits;
//! - number of machines;
//! - number of distributed nodes;
//! - characterization duration.
//!
//! This module operates primarily on sufficient statistics:
//!
//! ```text
//! raw shots
//!     |
//!     v
//! streaming estimator
//!     |
//!     v
//! sufficient statistics
//!     |
//!     v
//! this module
//! ```
//!
//! Therefore uncertainty calculation does not require retaining the entire
//! raw observation stream.
//!
//! Integer shot/count arithmetic is checked.
//! Floating-point values are validated for finiteness.
//!
//! Resource limits apply only to numerical algorithms such as iterative
//! inverse-beta evaluation. They are caller-selected policy, not semantic
//! machine-size limits.
//!
//! # Determinism
//!
//! This module:
//!
//! - uses no RNG;
//! - uses no global mutable state;
//! - does not read the system clock;
//! - does not depend on thread scheduling;
//! - does not use hash-map iteration;
//! - performs deterministic numerical algorithms.
//!
//! Identical validated inputs and policy produce identical results on the
//! same supported Rust/platform floating-point environment.
//!
//! Parallelism belongs outside this module. If a caller parallelizes
//! sufficient-statistics generation, it must use a deterministic reduction
//! policy when bit-for-bit reproducibility is required.
//!
//! # Numerical policy
//!
//! Invalid values are rejected.
//!
//! In particular:
//!
//! ```text
//! NaN
//! +∞
//! -∞
//! negative variance
//! invalid probabilities
//! invalid confidence levels
//! invalid weights
//! ```
//!
//! are never silently corrected.
//!
//! Numerical underflow to an exactly representable zero may occur naturally
//! in IEEE-754 arithmetic and is not by itself considered invalid.
//!
//! # Serialization
//!
//! This module defines semantic contracts only.
//!
//! It does NOT define a wire format.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! The semantic schema identity defined here allows that layer to distinguish
//! uncertainty-model versions without coupling serialization to Rust struct
//! layout.
//!
//! # Integration with estimator.rs
//!
//! `characterization::estimator` should produce one of the following
//! sufficient-statistics inputs:
//!
//! ```text
//! UncertaintyInput::Bernoulli
//! UncertaintyInput::Mean
//! UncertaintyInput::WeightedMean
//! UncertaintyInput::PoissonRate
//! UncertaintyInput::BoundedMean
//! ```
//!
//! The estimator remains responsible for determining what was observed and
//! what the point estimate means.
//!
//! This module remains responsible only for quantifying uncertainty under the
//! selected mathematical assumptions.
//!
//! # Integration with observation.rs
//!
//! `observation.rs` owns:
//!
//! - Observation;
//! - MeasurementPayload;
//! - OutcomeHistogram;
//! - ScalarSample;
//! - ComplexSample;
//! - shot streams.
//!
//! This module must not import those raw structures merely to calculate
//! uncertainty. The estimator is the adaptation boundary.
//!
//! # Integration with protocol.rs
//!
//! A protocol may specify an uncertainty requirement such as:
//!
//! ```text
//! exact binomial interval
//! confidence = 0.99
//! ```
//!
//! but protocol.rs does not perform the mathematics.
//!
//! The estimator/characterization layer constructs an `UncertaintyRequest`
//! and passes it here.
//!
//! # Integration with calibration
//!
//! Calibration uncertainty may be represented by a higher-level result that
//! combines:
//!
//! ```text
//! statistical uncertainty
//! +
//! systematic/calibration uncertainty
//! +
//! model uncertainty
//! ```
//!
//! This module does not silently combine those quantities because their
//! combination law depends on the scientific model.
//!
//! # Integration with ZQN noise
//!
//! A characterized noise parameter may be represented as:
//!
//! ```text
//! estimate = 0.0017
//! statistical uncertainty = 0.0002
//! confidence = 0.95
//! ```
//!
//! The resulting uncertainty information can be consumed by noise models,
//! propagation, routing, scheduling, QEC, simulation and benchmarking.
//!
//! Those consumers must not reinterpret the uncertainty semantics.
//!
//! # Quantum-resource identity
//!
//! This module does not require a qubit identity to perform uncertainty
//! mathematics.
//!
//! When a higher-level characterization result associates an uncertainty
//! result with a resource, the surrounding result type must use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file deliberately does not define another qubit identifier.
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
//! - no external numerical dependency;
//! - no unsafe code.
//!
//! # Security
//!
//! Uncertainty calculations may consume untrusted characterization data.
//!
//! Therefore:
//!
//! - all floating-point inputs are validated;
//! - integer overflow is checked;
//! - iterative algorithms have explicit iteration limits;
//! - tolerance is validated;
//! - pathological inputs cannot request an unbounded numerical loop;
//! - no allocation scales with shot count;
//! - no recursion is used for numerical algorithms;
//! - no external process is invoked.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. it owns uncertainty mathematics rather than estimation;
//! 2. it has no dependency on raw observation storage;
//! 3. it has no dependency on protocol implementation;
//! 4. it has no duplicate quantum-resource identity;
//! 5. invalid numerical inputs are rejected;
//! 6. exact and approximate methods are distinguishable;
//! 7. confidence and credible intervals are distinguishable;
//! 8. sufficient statistics are sufficient to calculate supported methods;
//! 9. integer overflow is checked;
//! 10. iterative algorithms have caller-selected resource limits;
//! 11. no machine-size limit is encoded;
//! 12. no random state exists;
//! 13. no unsafe Rust exists;
//! 14. the semantic schema is versioned;
//! 15. the public API can be consumed by future estimator, calibration,
//!     benchmarking, simulation and propagation modules without modifying
//!     this file merely because those modules are added.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for characterization uncertainty.
pub const UNCERTAINTY_SCHEMA_ID: &str =
    "zamani.quantum.zqn.characterization.uncertainty";

/// Semantic version of this uncertainty contract.
///
/// This is independent from:
///
/// - the Zamani language version;
/// - Quantum IR version;
/// - ZQN implementation version;
/// - serialization format version;
/// - hardware version;
/// - calibration version.
pub const UNCERTAINTY_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance for iterative mathematical calculations.
///
/// This is a numerical default, not a physical accuracy guarantee.
pub const DEFAULT_NUMERICAL_TOLERANCE: f64 = 1.0e-12;

/// Default maximum number of iterations for numerical inversion.
///
/// This is an algorithmic safety default, not a limit on observations,
/// qubits, experiments, or machines.
pub const DEFAULT_MAX_ITERATIONS: u64 = 256;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by uncertainty calculations.
#[derive(Clone, Debug, PartialEq)]
pub enum UncertaintyError {
    /// A supplied floating-point value is NaN or infinite.
    NonFiniteValue {
        /// Semantic name of the invalid field.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A probability is outside the closed interval [0, 1].
    InvalidProbability {
        /// Semantic field name.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A confidence level is not strictly between zero and one.
    InvalidConfidenceLevel {
        /// Supplied confidence level.
        value: f64,
    },

    /// A variance is negative.
    InvalidVariance {
        /// Semantic field name.
        field: &'static str,
        /// Invalid variance.
        value: f64,
    },

    /// A standard deviation is negative.
    InvalidStandardDeviation {
        /// Semantic field name.
        field: &'static str,
        /// Invalid standard deviation.
        value: f64,
    },

    /// A weight is invalid.
    InvalidWeight {
        /// Supplied weight.
        value: f64,
    },

    /// A sample count is mathematically insufficient for the requested
    /// method.
    InsufficientSamples {
        /// Number of samples supplied.
        samples: u64,
        /// Minimum mathematically required number.
        required: u64,
    },

    /// A denominator is zero.
    ZeroDenominator {
        /// Semantic denominator name.
        field: &'static str,
    },

    /// A numerator is greater than its denominator.
    InvalidCountRelation {
        /// Numerator.
        numerator: u64,
        /// Denominator.
        denominator: u64,
    },

    /// Integer arithmetic overflow occurred.
    IntegerOverflow {
        /// Operation description.
        operation: &'static str,
    },

    /// Floating-point arithmetic produced a non-finite result.
    NumericalFailure {
        /// Description of the failed operation.
        operation: &'static str,
    },

    /// The iterative numerical method did not converge.
    NonConvergence {
        /// Algorithm name.
        algorithm: &'static str,
        /// Number of iterations performed.
        iterations: u64,
        /// Requested tolerance.
        tolerance: f64,
    },

    /// Numerical tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// Maximum iteration count is invalid.
    InvalidIterationLimit {
        /// Supplied limit.
        value: u64,
    },

    /// The requested method is incompatible with the supplied statistics.
    IncompatibleInput {
        /// Method description.
        method: &'static str,
        /// Input description.
        input: &'static str,
    },

    /// Approximation was requested but policy forbids it.
    ApproximationNotAllowed {
        /// Method requiring approximation.
        method: &'static str,
    },

    /// A deterministic bound is invalid.
    InvalidBound {
        /// Lower bound.
        lower: f64,
        /// Upper bound.
        upper: f64,
    },

    /// The supported schema version does not understand the requested
    /// schema.
    UnsupportedSchemaVersion {
        /// Encountered version.
        found: u32,
        /// Highest supported version.
        supported: u32,
    },
}

impl fmt::Display for UncertaintyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "uncertainty field `{field}` contains non-finite value {value}"
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "uncertainty field `{field}` is not a probability: {value}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be strictly between 0 and 1, got {value}"
                )
            }

            Self::InvalidVariance { field, value } => {
                write!(
                    formatter,
                    "variance `{field}` must be non-negative, got {value}"
                )
            }

            Self::InvalidStandardDeviation { field, value } => {
                write!(
                    formatter,
                    "standard deviation `{field}` must be non-negative, got {value}"
                )
            }

            Self::InvalidWeight { value } => {
                write!(
                    formatter,
                    "weight must be finite and strictly positive, got {value}"
                )
            }

            Self::InsufficientSamples { samples, required } => {
                write!(
                    formatter,
                    "insufficient samples: received {samples}, require at least {required}"
                )
            }

            Self::ZeroDenominator { field } => {
                write!(formatter, "denominator `{field}` must be non-zero")
            }

            Self::InvalidCountRelation {
                numerator,
                denominator,
            } => {
                write!(
                    formatter,
                    "count relation invalid: numerator {numerator} exceeds denominator {denominator}"
                )
            }

            Self::IntegerOverflow { operation } => {
                write!(
                    formatter,
                    "integer overflow during uncertainty operation `{operation}`"
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "non-finite numerical result during `{operation}`"
                )
            }

            Self::NonConvergence {
                algorithm,
                iterations,
                tolerance,
            } => {
                write!(
                    formatter,
                    "algorithm `{algorithm}` did not converge after {iterations} iterations at tolerance {tolerance}"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "numerical tolerance must be finite and strictly positive, got {value}"
                )
            }

            Self::InvalidIterationLimit { value } => {
                write!(
                    formatter,
                    "iteration limit must be greater than zero, got {value}"
                )
            }

            Self::IncompatibleInput { method, input } => {
                write!(
                    formatter,
                    "uncertainty method `{method}` is incompatible with input `{input}`"
                )
            }

            Self::ApproximationNotAllowed { method } => {
                write!(
                    formatter,
                    "uncertainty method `{method}` requires approximation but policy forbids it"
                )
            }

            Self::InvalidBound { lower, upper } => {
                write!(
                    formatter,
                    "invalid bound [{lower}, {upper}]"
                )
            }

            Self::UnsupportedSchemaVersion { found, supported } => {
                write!(
                    formatter,
                    "unsupported uncertainty schema version {found}; supported through {supported}"
                )
            }
        }
    }
}

impl std::error::Error for UncertaintyError {}

/// Result type for uncertainty operations.
pub type UncertaintyResult<T> = Result<T, UncertaintyError>;

// =============================================================================
// Numerical validation
// =============================================================================

fn require_finite(field: &'static str, value: f64) -> UncertaintyResult<f64> {
    if !value.is_finite() {
        return Err(UncertaintyError::NonFiniteValue { field, value });
    }

    Ok(value)
}

fn require_probability(
    field: &'static str,
    value: f64,
) -> UncertaintyResult<f64> {
    require_finite(field, value)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(UncertaintyError::InvalidProbability { field, value });
    }

    Ok(value)
}

fn checked_add_u64(
    left: u64,
    right: u64,
    operation: &'static str,
) -> UncertaintyResult<u64> {
    left.checked_add(right)
        .ok_or(UncertaintyError::IntegerOverflow { operation })
}

// =============================================================================
// Confidence level
// =============================================================================

/// Valid confidence level.
///
/// The value represents the conventional frequentist confidence coefficient,
/// for example:
///
/// ```text
/// 0.90
/// 0.95
/// 0.99
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ConfidenceLevel(f64);

impl ConfidenceLevel {
    /// Creates a validated confidence level.
    ///
    /// The accepted mathematical domain is:
    ///
    /// ```text
    /// 0 < confidence < 1
    /// ```
    pub fn new(value: f64) -> UncertaintyResult<Self> {
        require_finite("confidence", value)?;

        if !(0.0 < value && value < 1.0) {
            return Err(UncertaintyError::InvalidConfidenceLevel { value });
        }

        Ok(Self(value))
    }

    /// Returns the confidence coefficient.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the two-sided tail probability.
    #[must_use]
    pub fn alpha(self) -> f64 {
        1.0 - self.0
    }
}

// =============================================================================
// Calculation policy
// =============================================================================

/// Policy controlling uncertainty calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UncertaintyPolicy {
    /// Absolute numerical tolerance used by iterative calculations.
    pub tolerance: f64,

    /// Maximum number of iterations permitted for iterative algorithms.
    pub max_iterations: u64,

    /// Whether explicitly approximate methods are allowed.
    pub approximation: ApproximationPolicy,
}

impl Default for UncertaintyPolicy {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_NUMERICAL_TOLERANCE,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            approximation: ApproximationPolicy::AllowApproximation,
        }
    }
}

impl UncertaintyPolicy {
    /// Validates the calculation policy.
    pub fn validate(&self) -> UncertaintyResult<()> {
        if !self.tolerance.is_finite() || self.tolerance <= 0.0 {
            return Err(UncertaintyError::InvalidTolerance {
                value: self.tolerance,
            });
        }

        if self.max_iterations == 0 {
            return Err(UncertaintyError::InvalidIterationLimit {
                value: self.max_iterations,
            });
        }

        Ok(())
    }
}

/// Controls whether approximate numerical/statistical methods may be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApproximationPolicy {
    /// Only methods whose declared result is exact under their mathematical
    /// model may be used.
    ExactOnly,

    /// Explicitly approximate methods may be used.
    AllowApproximation,
}

// =============================================================================
// Uncertainty method
// =============================================================================

/// Supported uncertainty methodologies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UncertaintyMethod {
    /// Standard error of an IID arithmetic mean.
    StandardErrorMean,

    /// Two-sided normal approximation for a mean.
    NormalMean,

    /// Two-sided Wilson score interval for a Bernoulli proportion.
    WilsonProportion,

    /// Two-sided exact Clopper-Pearson binomial interval.
    ClopperPearsonBinomial,

    /// Bayesian Beta posterior credible interval.
    BetaPosterior,

    /// Hoeffding distribution-free bounded-mean confidence interval.
    HoeffdingBoundedMean,

    /// Normal approximation for a Poisson rate.
    NormalPoissonRate,
}

impl UncertaintyMethod {
    /// Returns a stable method identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandardErrorMean => "standard-error-mean",
            Self::NormalMean => "normal-mean",
            Self::WilsonProportion => "wilson-proportion",
            Self::ClopperPearsonBinomial => "clopper-pearson-binomial",
            Self::BetaPosterior => "beta-posterior",
            Self::HoeffdingBoundedMean => "hoeffding-bounded-mean",
            Self::NormalPoissonRate => "normal-poisson-rate",
        }
    }

    /// Whether the method uses an approximation.
    #[must_use]
    pub const fn is_approximate(self) -> bool {
        match self {
            Self::StandardErrorMean
            | Self::ClopperPearsonBinomial
            | Self::BetaPosterior
            | Self::HoeffdingBoundedMean => false,

            Self::NormalMean
            | Self::WilsonProportion
            | Self::NormalPoissonRate => true,
        }
    }
}

// =============================================================================
// Sufficient statistics
// =============================================================================

/// Sufficient statistics consumed by the uncertainty layer.
///
/// This enum is intentionally independent of the raw observation
/// representation.
///
/// Characterization estimators convert arbitrary observation streams into
/// these contracts.
///
/// No variant requires retaining every raw shot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UncertaintyInput {
    /// Bernoulli/binomial observations.
    Bernoulli {
        /// Number of successes.
        successes: u64,

        /// Number of trials.
        trials: u64,
    },

    /// IID scalar observations summarized by their arithmetic mean and
    /// unbiased sample variance.
    Mean {
        /// Number of observations.
        samples: u64,

        /// Arithmetic sample mean.
        mean: f64,

        /// Unbiased sample variance.
        variance: f64,
    },

    /// Independently weighted scalar observations.
    ///
    /// `variance` is the weighted population variance:
    ///
    /// ```text
    /// sum(w_i (x_i - mean)^2) / sum(w_i)
    /// ```
    ///
    /// `sum_weights` and `sum_squared_weights` are retained so effective
    /// sample size can be calculated without retaining individual weights.
    WeightedMean {
        /// Weighted mean.
        mean: f64,

        /// Weighted population variance.
        variance: f64,

        /// Sum of positive weights.
        sum_weights: f64,

        /// Sum of squared weights.
        sum_squared_weights: f64,
    },

    /// Poisson event count over a finite positive exposure.
    PoissonRate {
        /// Observed event count.
        events: u64,

        /// Exposure in an arbitrary positive unit.
        exposure: f64,
    },

    /// Scalar mean where every observation is known to lie in [lower, upper].
    BoundedMean {
        /// Number of observations.
        samples: u64,

        /// Arithmetic sample mean.
        mean: f64,

        /// Known lower bound.
        lower: f64,

        /// Known upper bound.
        upper: f64,
    },
}

impl UncertaintyInput {
    /// Validates the sufficient-statistics contract.
    pub fn validate(&self) -> UncertaintyResult<()> {
        match *self {
            Self::Bernoulli {
                successes,
                trials,
            } => {
                if trials == 0 {
                    return Err(UncertaintyError::ZeroDenominator {
                        field: "trials",
                    });
                }

                if successes > trials {
                    return Err(UncertaintyError::InvalidCountRelation {
                        numerator: successes,
                        denominator: trials,
                    });
                }
            }

            Self::Mean {
                samples,
                mean,
                variance,
            } => {
                require_finite("mean", mean)?;
                require_finite("variance", variance)?;

                if samples < 2 {
                    return Err(UncertaintyError::InsufficientSamples {
                        samples,
                        required: 2,
                    });
                }

                if variance < 0.0 {
                    return Err(UncertaintyError::InvalidVariance {
                        field: "variance",
                        value: variance,
                    });
                }
            }

            Self::WeightedMean {
                mean,
                variance,
                sum_weights,
                sum_squared_weights,
            } => {
                require_finite("mean", mean)?;
                require_finite("variance", variance)?;
                require_finite("sum_weights", sum_weights)?;
                require_finite(
                    "sum_squared_weights",
                    sum_squared_weights,
                )?;

                if variance < 0.0 {
                    return Err(UncertaintyError::InvalidVariance {
                        field: "variance",
                        value: variance,
                    });
                }

                if sum_weights <= 0.0 {
                    return Err(UncertaintyError::InvalidWeight {
                        value: sum_weights,
                    });
                }

                if sum_squared_weights <= 0.0 {
                    return Err(UncertaintyError::InvalidWeight {
                        value: sum_squared_weights,
                    });
                }

                if sum_squared_weights
                    > sum_weights * sum_weights
                    && (sum_weights * sum_weights).is_finite()
                {
                    return Err(UncertaintyError::IncompatibleInput {
                        method: "weighted-statistics-validation",
                        input: "sum_squared_weights",
                    });
                }
            }

            Self::PoissonRate {
                events,
                exposure,
            } => {
                require_finite("exposure", exposure)?;

                if exposure <= 0.0 {
                    return Err(UncertaintyError::InvalidWeight {
                        value: exposure,
                    });
                }

                let _ = events;
            }

            Self::BoundedMean {
                samples,
                mean,
                lower,
                upper,
            } => {
                require_finite("mean", mean)?;
                require_finite("lower", lower)?;
                require_finite("upper", upper)?;

                if samples == 0 {
                    return Err(UncertaintyError::InsufficientSamples {
                        samples,
                        required: 1,
                    });
                }

                if lower > upper {
                    return Err(UncertaintyError::InvalidBound { lower, upper });
                }

                if mean < lower || mean > upper {
                    return Err(UncertaintyError::InvalidBound {
                        lower,
                        upper,
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Interval
// =============================================================================

/// Kind of uncertainty interval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntervalKind {
    /// Frequentist confidence interval.
    Confidence,

    /// Bayesian posterior credible interval.
    Credible,

    /// Deterministic mathematical bound.
    Deterministic,
}

/// Validated closed interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    /// Lower endpoint.
    pub lower: f64,

    /// Upper endpoint.
    pub upper: f64,

    /// Semantic interval type.
    pub kind: IntervalKind,

    /// Confidence/credible coefficient when applicable.
    ///
    /// `None` for deterministic bounds.
    pub level: Option<ConfidenceLevel>,
}

impl Interval {
    /// Constructs and validates an interval.
    pub fn new(
        lower: f64,
        upper: f64,
        kind: IntervalKind,
        level: Option<ConfidenceLevel>,
    ) -> UncertaintyResult<Self> {
        require_finite("interval.lower", lower)?;
        require_finite("interval.upper", upper)?;

        if lower > upper {
            return Err(UncertaintyError::InvalidBound { lower, upper });
        }

        match kind {
            IntervalKind::Confidence | IntervalKind::Credible => {
                if level.is_none() {
                    return Err(UncertaintyError::InvalidConfidenceLevel {
                        value: 0.0,
                    });
                }
            }

            IntervalKind::Deterministic => {
                if level.is_some() {
                    return Err(UncertaintyError::IncompatibleInput {
                        method: "deterministic-interval",
                        input: "confidence-level",
                    });
                }
            }
        }

        Ok(Self {
            lower,
            upper,
            kind,
            level,
        })
    }

    /// Returns the interval width.
    #[must_use]
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns the midpoint.
    #[must_use]
    pub fn midpoint(self) -> f64 {
        self.lower + (self.upper - self.lower) * 0.5
    }
}

// =============================================================================
// Uncertainty result
// =============================================================================

/// Complete uncertainty calculation result.
///
/// The result contains the point estimate because this makes the uncertainty
/// object self-describing, but this module does not decide the semantic
/// meaning of that point estimate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UncertaintyResultValue {
    /// Point estimate supplied by the sufficient-statistics contract.
    pub estimate: f64,

    /// Standard error, where the selected method defines one.
    pub standard_error: Option<f64>,

    /// Optional interval.
    pub interval: Option<Interval>,

    /// Effective sample size when meaningful.
    pub effective_sample_size: Option<f64>,

    /// Number of observations/trials represented by the input where a
    /// conventional integer sample size exists.
    pub sample_count: Option<u64>,

    /// Method used.
    pub method: UncertaintyMethod,

    /// Whether the mathematical method is an explicit approximation.
    pub approximate: bool,

    /// Stable semantic schema version.
    pub schema_version: u32,
}

impl UncertaintyResultValue {
    /// Validates the result itself.
    pub fn validate(&self) -> UncertaintyResult<()> {
        require_finite("estimate", self.estimate)?;

        if let Some(standard_error) = self.standard_error {
            require_finite("standard_error", standard_error)?;

            if standard_error < 0.0 {
                return Err(UncertaintyError::InvalidStandardDeviation {
                    field: "standard_error",
                    value: standard_error,
                });
            }
        }

        if let Some(effective_sample_size) = self.effective_sample_size {
            require_finite(
                "effective_sample_size",
                effective_sample_size,
            )?;

            if effective_sample_size <= 0.0 {
                return Err(UncertaintyError::InvalidWeight {
                    value: effective_sample_size,
                });
            }
        }

        if self.schema_version == 0 {
            return Err(UncertaintyError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: UNCERTAINTY_SCHEMA_VERSION,
            });
        }

        if let Some(interval) = self.interval {
            require_finite("interval.lower", interval.lower)?;
            require_finite("interval.upper", interval.upper)?;
        }

        Ok(())
    }
}

// =============================================================================
// Public engine
// =============================================================================

/// Stateless uncertainty calculation engine.
///
/// It contains only explicit numerical policy and no mutable global state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UncertaintyEngine {
    /// Numerical/resource policy.
    pub policy: UncertaintyPolicy,
}

impl Default for UncertaintyEngine {
    fn default() -> Self {
        Self {
            policy: UncertaintyPolicy::default(),
        }
    }
}

impl UncertaintyEngine {
    /// Creates an engine with explicit policy.
    pub fn new(policy: UncertaintyPolicy) -> UncertaintyResult<Self> {
        policy.validate()?;

        Ok(Self { policy })
    }

    /// Calculates uncertainty using the selected method.
    pub fn calculate(
        &self,
        method: UncertaintyMethod,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        self.policy.validate()?;
        input.validate()?;

        if method.is_approximate()
            && self.policy.approximation == ApproximationPolicy::ExactOnly
        {
            return Err(UncertaintyError::ApproximationNotAllowed {
                method: method.as_str(),
            });
        }

        match method {
            UncertaintyMethod::StandardErrorMean => {
                self.standard_error_mean(input, confidence)
            }

            UncertaintyMethod::NormalMean => {
                self.normal_mean(input, confidence)
            }

            UncertaintyMethod::WilsonProportion => {
                self.wilson_proportion(input, confidence)
            }

            UncertaintyMethod::ClopperPearsonBinomial => {
                self.clopper_pearson(input, confidence)
            }

            UncertaintyMethod::BetaPosterior => {
                self.beta_posterior(input, confidence)
            }

            UncertaintyMethod::HoeffdingBoundedMean => {
                self.hoeffding_bounded_mean(input, confidence)
            }

            UncertaintyMethod::NormalPoissonRate => {
                self.normal_poisson_rate(input, confidence)
            }
        }
    }

    /// Calculates the standard error of an arithmetic mean.
    pub fn standard_error_mean(
        &self,
        input: UncertaintyInput,
        _confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (samples, mean, variance) = match input {
            UncertaintyInput::Mean {
                samples,
                mean,
                variance,
            } => (samples, mean, variance),

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "standard-error-mean",
                    input: "non-mean",
                })
            }
        };

        let denominator = samples as f64;
        let standard_error = safe_sqrt(
            variance / denominator,
            "standard-error-mean",
        )?;

        let result = UncertaintyResultValue {
            estimate: mean,
            standard_error: Some(standard_error),
            interval: None,
            effective_sample_size: Some(denominator),
            sample_count: Some(samples),
            method: UncertaintyMethod::StandardErrorMean,
            approximate: false,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Calculates a two-sided normal-approximation interval for a scalar mean.
    ///
    /// This method is appropriate only when the normal approximation is
    /// scientifically justified by the caller's model/data.
    pub fn normal_mean(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (samples, mean, variance) = match input {
            UncertaintyInput::Mean {
                samples,
                mean,
                variance,
            } => (samples, mean, variance),

            UncertaintyInput::WeightedMean {
                mean,
                variance,
                sum_weights,
                sum_squared_weights,
            } => {
                let effective =
                    effective_sample_size(sum_weights, sum_squared_weights)?;

                let standard_error =
                    safe_sqrt(variance / effective, "weighted-normal-se")?;

                let z = standard_normal_quantile(
                    0.5 + confidence.value() * 0.5,
                )?;

                let half_width = safe_mul(
                    z,
                    standard_error,
                    "weighted-normal-half-width",
                )?;

                let interval = Interval::new(
                    mean - half_width,
                    mean + half_width,
                    IntervalKind::Confidence,
                    Some(confidence),
                )?;

                let result = UncertaintyResultValue {
                    estimate: mean,
                    standard_error: Some(standard_error),
                    interval: Some(interval),
                    effective_sample_size: Some(effective),
                    sample_count: None,
                    method: UncertaintyMethod::NormalMean,
                    approximate: true,
                    schema_version: UNCERTAINTY_SCHEMA_VERSION,
                };

                result.validate()?;
                return Ok(result);
            }

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "normal-mean",
                    input: "non-mean",
                })
            }
        };

        let standard_error = safe_sqrt(
            variance / samples as f64,
            "normal-mean-standard-error",
        )?;

        let z = standard_normal_quantile(
            0.5 + confidence.value() * 0.5,
        )?;

        let half_width =
            safe_mul(z, standard_error, "normal-mean-half-width")?;

        let interval = Interval::new(
            mean - half_width,
            mean + half_width,
            IntervalKind::Confidence,
            Some(confidence),
        )?;

        let result = UncertaintyResultValue {
            estimate: mean,
            standard_error: Some(standard_error),
            interval: Some(interval),
            effective_sample_size: Some(samples as f64),
            sample_count: Some(samples),
            method: UncertaintyMethod::NormalMean,
            approximate: true,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Wilson score interval for a binomial proportion.
    ///
    /// The Wilson interval is a normal-score approximation but has better
    /// finite-sample behavior than the simple Wald interval, particularly
    /// near 0 and 1.
    pub fn wilson_proportion(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (successes, trials) = match input {
            UncertaintyInput::Bernoulli {
                successes,
                trials,
            } => (successes, trials),

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "wilson-proportion",
                    input: "non-bernoulli",
                })
            }
        };

        let n = trials as f64;
        let p = successes as f64 / n;

        let z = standard_normal_quantile(
            0.5 + confidence.value() * 0.5,
        )?;

        let z_squared = safe_mul(z, z, "wilson-z-squared")?;
        let denominator = 1.0 + z_squared / n;

        let center = (p + z_squared / (2.0 * n)) / denominator;

        let variance_term = p * (1.0 - p) / n;
        let finite_term = z_squared / (4.0 * n * n);
        let radius_inner = variance_term + finite_term;

        let radius = safe_mul(
            z / denominator,
            safe_sqrt(radius_inner, "wilson-radius-inner")?,
            "wilson-radius",
        )?;

        let lower = (center - radius).max(0.0);
        let upper = (center + radius).min(1.0);

        let interval = Interval::new(
            lower,
            upper,
            IntervalKind::Confidence,
            Some(confidence),
        )?;

        let standard_error =
            safe_sqrt(p * (1.0 - p) / n, "wilson-standard-error")?;

        let result = UncertaintyResultValue {
            estimate: p,
            standard_error: Some(standard_error),
            interval: Some(interval),
            effective_sample_size: Some(n),
            sample_count: Some(trials),
            method: UncertaintyMethod::WilsonProportion,
            approximate: true,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Exact two-sided Clopper-Pearson binomial confidence interval.
    ///
    /// This uses inverse regularized incomplete-beta evaluation and therefore
    /// does not use a normal approximation.
    pub fn clopper_pearson(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (successes, trials) = match input {
            UncertaintyInput::Bernoulli {
                successes,
                trials,
            } => (successes, trials),

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "clopper-pearson-binomial",
                    input: "non-bernoulli",
                })
            }
        };

        let alpha = confidence.alpha();
        let lower = if successes == 0 {
            0.0
        } else {
            inverse_regularized_beta(
                alpha * 0.5,
                successes as f64,
                (trials - successes + 1) as f64,
                self.policy.tolerance,
                self.policy.max_iterations,
            )?
        };

        let upper = if successes == trials {
            1.0
        } else {
            inverse_regularized_beta(
                1.0 - alpha * 0.5,
                (successes + 1) as f64,
                (trials - successes) as f64,
                self.policy.tolerance,
                self.policy.max_iterations,
            )?
        };

        let p = successes as f64 / trials as f64;

        let interval = Interval::new(
            lower,
            upper,
            IntervalKind::Confidence,
            Some(confidence),
        )?;

        let result = UncertaintyResultValue {
            estimate: p,
            standard_error: None,
            interval: Some(interval),
            effective_sample_size: Some(trials as f64),
            sample_count: Some(trials),
            method: UncertaintyMethod::ClopperPearsonBinomial,
            approximate: false,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Bayesian Beta posterior credible interval.
    ///
    /// The caller supplies a Beta prior through `BetaPrior`.
    ///
    /// This method is Bayesian and therefore must never be labelled a
    /// frequentist confidence interval.
    pub fn beta_posterior(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (successes, trials) = match input {
            UncertaintyInput::Bernoulli {
                successes,
                trials,
            } => (successes, trials),

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "beta-posterior",
                    input: "non-bernoulli",
                })
            }
        };

        let prior = BetaPrior::jeffreys()?;

        self.beta_posterior_with_prior(
            successes,
            trials,
            prior,
            confidence,
        )
    }

    /// Bayesian Beta posterior with an explicit prior.
    pub fn beta_posterior_with_prior(
        &self,
        successes: u64,
        trials: u64,
        prior: BetaPrior,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        if trials == 0 {
            return Err(UncertaintyError::ZeroDenominator {
                field: "trials",
            });
        }

        if successes > trials {
            return Err(UncertaintyError::InvalidCountRelation {
                numerator: successes,
                denominator: trials,
            });
        }

        prior.validate()?;

        let failures = trials - successes;

        let posterior_alpha =
            prior.alpha + successes as f64;
        let posterior_beta =
            prior.beta + failures as f64;

        require_finite("posterior_alpha", posterior_alpha)?;
        require_finite("posterior_beta", posterior_beta)?;

        let lower = inverse_regularized_beta(
            confidence.alpha() * 0.5,
            posterior_alpha,
            posterior_beta,
            self.policy.tolerance,
            self.policy.max_iterations,
        )?;

        let upper = inverse_regularized_beta(
            1.0 - confidence.alpha() * 0.5,
            posterior_alpha,
            posterior_beta,
            self.policy.tolerance,
            self.policy.max_iterations,
        )?;

        let posterior_mean =
            posterior_alpha / (posterior_alpha + posterior_beta);

        let interval = Interval::new(
            lower,
            upper,
            IntervalKind::Credible,
            Some(confidence),
        )?;

        let result = UncertaintyResultValue {
            estimate: posterior_mean,
            standard_error: None,
            interval: Some(interval),
            effective_sample_size: Some(trials as f64),
            sample_count: Some(trials),
            method: UncertaintyMethod::BetaPosterior,
            approximate: false,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Hoeffding confidence interval for a bounded mean.
    ///
    /// For observations in [a,b], with n IID observations, the two-sided
    /// Hoeffding radius is:
    ///
    /// ```text
    /// (b-a) * sqrt( ln(2 / delta) / (2n) )
    /// ```
    ///
    /// This is a distribution-free concentration bound under the IID and
    /// bounded-observation assumptions.
    pub fn hoeffding_bounded_mean(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (samples, mean, lower_bound, upper_bound) = match input {
            UncertaintyInput::BoundedMean {
                samples,
                mean,
                lower,
                upper,
            } => (samples, mean, lower, upper),

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "hoeffding-bounded-mean",
                    input: "non-bounded-mean",
                })
            }
        };

        let width = upper_bound - lower_bound;

        if width == 0.0 {
            let interval = Interval::new(
                mean,
                mean,
                IntervalKind::Deterministic,
                None,
            )?;

            let result = UncertaintyResultValue {
                estimate: mean,
                standard_error: Some(0.0),
                interval: Some(interval),
                effective_sample_size: Some(samples as f64),
                sample_count: Some(samples),
                method: UncertaintyMethod::HoeffdingBoundedMean,
                approximate: false,
                schema_version: UNCERTAINTY_SCHEMA_VERSION,
            };

            result.validate()?;
            return Ok(result);
        }

        let delta = 1.0 - confidence.value();

        let logarithm =
            (2.0 / delta).ln();

        let denominator = 2.0 * samples as f64;

        let radius = safe_mul(
            width,
            safe_sqrt(
                logarithm / denominator,
                "hoeffding-radius-inner",
            )?,
            "hoeffding-radius",
        )?;

        let interval = Interval::new(
            (mean - radius).max(lower_bound),
            (mean + radius).min(upper_bound),
            IntervalKind::Confidence,
            Some(confidence),
        )?;

        let result = UncertaintyResultValue {
            estimate: mean,
            standard_error: None,
            interval: Some(interval),
            effective_sample_size: Some(samples as f64),
            sample_count: Some(samples),
            method: UncertaintyMethod::HoeffdingBoundedMean,
            approximate: false,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }

    /// Normal approximation for a Poisson event rate.
    ///
    /// This method is intentionally marked approximate.
    ///
    /// Exact Poisson intervals require a different numerical contract and
    /// should not be silently substituted here.
    pub fn normal_poisson_rate(
        &self,
        input: UncertaintyInput,
        confidence: ConfidenceLevel,
    ) -> UncertaintyResult<UncertaintyResultValue> {
        let (events, exposure) = match input {
            UncertaintyInput::PoissonRate { events, exposure } => {
                (events, exposure)
            }

            _ => {
                return Err(UncertaintyError::IncompatibleInput {
                    method: "normal-poisson-rate",
                    input: "non-poisson-rate",
                })
            }
        };

        let rate = events as f64 / exposure;

        let standard_error =
            safe_sqrt(rate / exposure, "poisson-rate-standard-error")?;

        let z = standard_normal_quantile(
            0.5 + confidence.value() * 0.5,
        )?;

        let radius = safe_mul(
            z,
            standard_error,
            "poisson-rate-radius",
        )?;

        let interval = Interval::new(
            (rate - radius).max(0.0),
            rate + radius,
            IntervalKind::Confidence,
            Some(confidence),
        )?;

        let result = UncertaintyResultValue {
            estimate: rate,
            standard_error: Some(standard_error),
            interval: Some(interval),
            effective_sample_size: Some(events.max(1) as f64),
            sample_count: Some(events),
            method: UncertaintyMethod::NormalPoissonRate,
            approximate: true,
            schema_version: UNCERTAINTY_SCHEMA_VERSION,
        };

        result.validate()?;
        Ok(result)
    }
}

// =============================================================================
// Beta prior
// =============================================================================

/// Beta distribution prior used for Bayesian binomial uncertainty.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BetaPrior {
    /// Alpha shape parameter.
    pub alpha: f64,

    /// Beta shape parameter.
    pub beta: f64,
}

impl BetaPrior {
    /// Creates a validated Beta prior.
    pub fn new(alpha: f64, beta: f64) -> UncertaintyResult<Self> {
        require_finite("prior.alpha", alpha)?;
        require_finite("prior.beta", beta)?;

        if alpha <= 0.0 {
            return Err(UncertaintyError::InvalidWeight { value: alpha });
        }

        if beta <= 0.0 {
            return Err(UncertaintyError::InvalidWeight { value: beta });
        }

        Ok(Self { alpha, beta })
    }

    /// Jeffreys prior Beta(1/2, 1/2).
    pub fn jeffreys() -> UncertaintyResult<Self> {
        Self::new(0.5, 0.5)
    }

    /// Uniform prior Beta(1, 1).
    pub fn uniform() -> UncertaintyResult<Self> {
        Self::new(1.0, 1.0)
    }
}

// =============================================================================
// Weighted statistics
// =============================================================================

/// Calculates effective sample size:
///
/// ```text
/// n_eff = (Σw)^2 / Σ(w²)
/// ```
pub fn effective_sample_size(
    sum_weights: f64,
    sum_squared_weights: f64,
) -> UncertaintyResult<f64> {
    require_finite("sum_weights", sum_weights)?;
    require_finite("sum_squared_weights", sum_squared_weights)?;

    if sum_weights <= 0.0 {
        return Err(UncertaintyError::InvalidWeight {
            value: sum_weights,
        });
    }

    if sum_squared_weights <= 0.0 {
        return Err(UncertaintyError::InvalidWeight {
            value: sum_squared_weights,
        });
    }

    let numerator = safe_mul(
        sum_weights,
        sum_weights,
        "effective-sample-size-numerator",
    )?;

    let result = numerator / sum_squared_weights;

    require_finite("effective_sample_size", result)?;

    if result <= 0.0 {
        return Err(UncertaintyError::NumericalFailure {
            operation: "effective-sample-size",
        });
    }

    Ok(result)
}

// =============================================================================
// Numerical helpers
// =============================================================================

fn safe_mul(
    left: f64,
    right: f64,
    operation: &'static str,
) -> UncertaintyResult<f64> {
    let result = left * right;

    if !result.is_finite() {
        return Err(UncertaintyError::NumericalFailure { operation });
    }

    Ok(result)
}

fn safe_sqrt(
    value: f64,
    operation: &'static str,
) -> UncertaintyResult<f64> {
    if !value.is_finite() || value < 0.0 {
        return Err(UncertaintyError::NumericalFailure { operation });
    }

    let result = value.sqrt();

    if !result.is_finite() {
        return Err(UncertaintyError::NumericalFailure { operation });
    }

    Ok(result)
}

/// Inverse standard normal CDF.
///
/// This is the Acklam rational approximation. It is deterministic and uses
/// no external numerical dependency.
///
/// The returned value is an explicit numerical approximation and is used only
/// by methods that already declare themselves approximate.
fn standard_normal_quantile(p: f64) -> UncertaintyResult<f64> {
    require_probability("normal-quantile-probability", p)?;

    if p <= 0.0 || p >= 1.0 {
        return Err(UncertaintyError::InvalidProbability {
            field: "normal-quantile-probability",
            value: p,
        });
    }

    // Peter J. Acklam inverse-normal approximation coefficients.
    const A1: f64 = -3.969_683_028_665_376e1;
    const A2: f64 = 2.209_460_984_245_205e2;
    const A3: f64 = -2.759_285_104_469_687e2;
    const A4: f64 = 1.383_577_518_672_690e2;
    const A5: f64 = -3.066_479_806_614_716e1;
    const A6: f64 = 2.506_628_277_459_239;

    const B1: f64 = -5.447_609_879_822_406e1;
    const B2: f64 = 1.615_858_368_580_409e2;
    const B3: f64 = -1.556_989_798_598_866e2;
    const B4: f64 = 6.680_131_188_771_972e1;
    const B5: f64 = -1.328_068_155_288_572e1;

    const C1: f64 = -7.784_894_002_430_293e-3;
    const C2: f64 = -3.223_964_580_411_365e-1;
    const C3: f64 = -2.400_758_277_161_838;
    const C4: f64 = -2.549_732_539_343_734;
    const C5: f64 = 4.374_664_141_464_968;
    const C6: f64 = 2.938_163_982_698_783;

    const D1: f64 = 7.784_695_709_041_462e-3;
    const D2: f64 = 3.224_671_290_700_398e-1;
    const D3: f64 = 2.445_134_137_142_996;
    const D4: f64 = 3.754_408_661_907_416;

    const LOW: f64 = 0.024_25;
    const HIGH: f64 = 1.0 - LOW;

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        let numerator =
            ((((C1 * q + C2) * q + C3) * q + C4) * q + C5)
                * q
                + C6;

        let denominator =
            (((D1 * q + D2) * q + D3) * q + D4) * q
                + 1.0;

        let result = numerator / denominator;

        require_finite("normal-quantile", result)
    } else if p <= HIGH {
        let q = p - 0.5;
        let r = q * q;

        let numerator =
            (((((A1 * r + A2) * r + A3) * r + A4) * r + A5)
                * r
                + A6)
                * q;

        let denominator =
            ((((B1 * r + B2) * r + B3) * r + B4) * r + B5)
                * r
                + 1.0;

        let result = numerator / denominator;

        require_finite("normal-quantile", result)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        let numerator =
            ((((C1 * q + C2) * q + C3) * q + C4) * q + C5)
                * q
                + C6;

        let denominator =
            (((D1 * q + D2) * q + D3) * q + D4) * q
                + 1.0;

        let result = -(numerator / denominator);

        require_finite("normal-quantile", result)
    }
}

// =============================================================================
// Gamma / beta numerical functions
// =============================================================================

/// Natural logarithm of Gamma(x) using the Lanczos approximation.
fn ln_gamma(x: f64) -> UncertaintyResult<f64> {
    require_finite("gamma.argument", x)?;

    if x <= 0.0 {
        return Err(UncertaintyError::NumericalFailure {
            operation: "ln-gamma-domain",
        });
    }

    // Lanczos coefficients for g=7.
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];

    if x < 0.5 {
        let reflected =
            core::f64::consts::PI
                / ((core::f64::consts::PI * x).sin());

        let result =
            reflected.ln() - ln_gamma(1.0 - x)?;

        return require_finite("ln-gamma", result);
    }

    let z = x - 1.0;

    let mut sum = COEFFICIENTS[0];

    for (index, coefficient) in COEFFICIENTS.iter().enumerate().skip(1) {
        sum += coefficient / (z + index as f64);
    }

    let t = z + 7.5;

    let result =
        0.5 * (2.0 * core::f64::consts::PI).ln()
            + (z + 0.5) * t.ln()
            - t
            + sum.ln();

    require_finite("ln-gamma", result)
}

/// Continued fraction for the incomplete beta function.
fn beta_continued_fraction(
    a: f64,
    b: f64,
    x: f64,
    tolerance: f64,
    max_iterations: u64,
) -> UncertaintyResult<f64> {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;

    if d.abs() < 1.0e-300 {
        d = 1.0e-300;
    }

    d = 1.0 / d;

    let mut h = d;

    let mut iteration = 1_u64;

    while iteration <= max_iterations {
        let m = iteration as f64;
        let m2 = 2.0 * m;

        let numerator_a =
            m * (b - m) * x
                / ((qam + m2) * (a + m2));

        d = 1.0 + numerator_a * d;

        if d.abs() < 1.0e-300 {
            d = 1.0e-300;
        }

        c = 1.0 + numerator_a / c;

        if c.abs() < 1.0e-300 {
            c = 1.0e-300;
        }

        d = 1.0 / d;
        h *= d * c;

        let numerator_b =
            -(a + m)
                * (qab + m)
                * x
                / ((a + m2) * (qap + m2));

        d = 1.0 + numerator_b * d;

        if d.abs() < 1.0e-300 {
            d = 1.0e-300;
        }

        c = 1.0 + numerator_b / c;

        if c.abs() < 1.0e-300 {
            c = 1.0e-300;
        }

        d = 1.0 / d;

        let delta = d * c;

        h *= delta;

        if (delta - 1.0).abs() <= tolerance {
            return require_finite(
                "beta-continued-fraction",
                h,
            );
        }

        iteration += 1;
    }

    Err(UncertaintyError::NonConvergence {
        algorithm: "incomplete-beta-continued-fraction",
        iterations: max_iterations,
        tolerance,
    })
}

/// Regularized incomplete beta I_x(a,b).
fn regularized_beta(
    x: f64,
    a: f64,
    b: f64,
    tolerance: f64,
    max_iterations: u64,
) -> UncertaintyResult<f64> {
    require_finite("beta.x", x)?;
    require_finite("beta.a", a)?;
    require_finite("beta.b", b)?;

    if !(0.0..=1.0).contains(&x) {
        return Err(UncertaintyError.InvalidProbability {
            field: "beta.x",
            value: x,
        });
    }

    if a <= 0.0 || b <= 0.0 {
        return Err(UncertaintyError.NumericalFailure {
            operation: "regularized-beta-domain",
        });
    }

    if x == 0.0 {
        return Ok(0.0);
    }

    if x == 1.0 {
        return Ok(1.0);
    }

    let log_front =
        a * x.ln()
            + b * (1.0 - x).ln()
            - ln_gamma(a)?
            - ln_gamma(b)?
            + ln_gamma(a + b)?;

    let front = log_front.exp();

    if !front.is_finite() {
        return Err(UncertaintyError.NumericalFailure {
            operation: "regularized-beta-front",
        });
    }

    let result = if x < (a + 1.0) / (a + b + 2.0) {
        front
            * beta_continued_fraction(
                a,
                b,
                x,
                tolerance,
                max_iterations,
            )?
            / a
    } else {
        let complement =
            front
                * beta_continued_fraction(
                    b,
                    a,
                    1.0 - x,
                    tolerance,
                    max_iterations,
                )?
                / b;

        1.0 - complement
    };

    if !result.is_finite() {
        return Err(UncertaintyError::NumericalFailure {
            operation: "regularized-beta",
        });
    }

    // Numerical roundoff may produce a tiny excursion outside [0,1].
    // Only correct values whose violation is within numerical tolerance.
    if result < 0.0 {
        if result >= -tolerance {
            return Ok(0.0);
        }

        return Err(UncertaintyError::NumericalFailure {
            operation: "regularized-beta-range",
        });
    }

    if result > 1.0 {
        if result <= 1.0 + tolerance {
            return Ok(1.0);
        }

        return Err(UncertaintyError::NumericalFailure {
            operation: "regularized-beta-range",
        });
    }

    Ok(result)
}

/// Inverse regularized incomplete beta.
///
/// The implementation uses deterministic bisection. This is intentionally
/// slower than specialized approximations but has a clear convergence and
/// resource contract.
fn inverse_regularized_beta(
    target: f64,
    a: f64,
    b: f64,
    tolerance: f64,
    max_iterations: u64,
) -> UncertaintyResult<f64> {
    require_probability("beta.inverse.target", target)?;
    require_finite("beta.inverse.a", a)?;
    require_finite("beta.inverse.b", b)?;

    if a <= 0.0 || b <= 0.0 {
        return Err(UncertaintyError.NumericalFailure {
            operation: "inverse-beta-domain",
        });
    }

    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(UncertaintyError::InvalidTolerance {
            value: tolerance,
        });
    }

    if max_iterations == 0 {
        return Err(UncertaintyError::InvalidIterationLimit {
            value: max_iterations,
        });
    }

    if target == 0.0 {
        return Ok(0.0);
    }

    if target == 1.0 {
        return Ok(1.0);
    }

    let mut lower = 0.0;
    let mut upper = 1.0;

    let mut iteration = 0_u64;

    while iteration < max_iterations {
        let midpoint = lower + (upper - lower) * 0.5;

        let value = regularized_beta(
            midpoint,
            a,
            b,
            tolerance * 0.25,
            max_iterations,
        )?;

        let error = value - target;

        if error.abs() <= tolerance
            || (upper - lower).abs() <= tolerance
        {
            return Ok(midpoint);
        }

        if value < target {
            lower = midpoint;
        } else {
            upper = midpoint;
        }

        iteration += 1;
    }

    Err(UncertaintyError::NonConvergence {
        algorithm: "inverse-regularized-beta",
        iterations: max_iterations,
        tolerance,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_level_rejects_zero_and_one() {
        assert!(ConfidenceLevel::new(0.0).is_err());
        assert!(ConfidenceLevel::new(1.0).is_err());
        assert!(ConfidenceLevel::new(0.95).is_ok());
    }

    #[test]
    fn bernoulli_rejects_invalid_counts() {
        let input = UncertaintyInput::Bernoulli {
            successes: 11,
            trials: 10,
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn bernoulli_rejects_zero_trials() {
        let input = UncertaintyInput::Bernoulli {
            successes: 0,
            trials: 0,
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn mean_rejects_non_finite_values() {
        let input = UncertaintyInput::Mean {
            samples: 10,
            mean: f64::NAN,
            variance: 1.0,
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn mean_rejects_negative_variance() {
        let input = UncertaintyInput::Mean {
            samples: 10,
            mean: 0.0,
            variance: -1.0,
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn effective_sample_size_uniform_weights() {
        let result =
            effective_sample_size(10.0, 10.0).expect("valid");

        assert!((result - 10.0).abs() < 1.0e-12);
    }

    #[test]
    fn standard_error_is_positive_for_positive_variance() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Mean {
            samples: 100,
            mean: 2.0,
            variance: 4.0,
        };

        let result = engine
            .standard_error_mean(input, confidence)
            .expect("valid");

        assert!((result.standard_error.expect("present") - 0.2).abs() < 1.0e-12);
    }

    #[test]
    fn wilson_interval_is_inside_probability_domain() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Bernoulli {
            successes: 5,
            trials: 10,
        };

        let result = engine
            .wilson_proportion(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower <= 0.5);
        assert!(interval.upper >= 0.5);
    }

    #[test]
    fn clopper_pearson_extreme_zero_successes() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Bernoulli {
            successes: 0,
            trials: 10,
        };

        let result = engine
            .clopper_pearson(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert_eq!(interval.lower, 0.0);
        assert!(interval.upper > 0.0);
        assert!(interval.upper <= 1.0);
    }

    #[test]
    fn clopper_pearson_extreme_all_successes() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Bernoulli {
            successes: 10,
            trials: 10,
        };

        let result = engine
            .clopper_pearson(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.lower < 1.0);
        assert_eq!(interval.upper, 1.0);
    }

    #[test]
    fn beta_prior_requires_positive_parameters() {
        assert!(BetaPrior::new(0.0, 1.0).is_err());
        assert!(BetaPrior::new(1.0, 0.0).is_err());
        assert!(BetaPrior::new(0.5, 0.5).is_ok());
    }

    #[test]
    fn beta_posterior_returns_credible_interval() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Bernoulli {
            successes: 50,
            trials: 100,
        };

        let result = engine
            .beta_posterior(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert_eq!(interval.kind, IntervalKind::Credible);
        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower < result.estimate);
        assert!(interval.upper > result.estimate);
    }

    #[test]
    fn hoeffding_interval_respects_known_bounds() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::BoundedMean {
            samples: 1000,
            mean: 0.5,
            lower: 0.0,
            upper: 1.0,
        };

        let result = engine
            .hoeffding_bounded_mean(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
    }

    #[test]
    fn zero_width_bound_is_exact() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::BoundedMean {
            samples: 100,
            mean: 2.0,
            lower: 2.0,
            upper: 2.0,
        };

        let result = engine
            .hoeffding_bounded_mean(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert_eq!(interval.lower, 2.0);
        assert_eq!(interval.upper, 2.0);
        assert_eq!(interval.kind, IntervalKind::Deterministic);
    }

    #[test]
    fn normal_poisson_rate_is_non_negative() {
        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::PoissonRate {
            events: 100,
            exposure: 1000.0,
        };

        let result = engine
            .normal_poisson_rate(input, confidence)
            .expect("valid");

        let interval = result.interval.expect("interval");

        assert!(interval.lower >= 0.0);
        assert!(result.estimate >= 0.0);
    }

    #[test]
    fn exact_only_rejects_normal_approximation() {
        let policy = UncertaintyPolicy {
            approximation: ApproximationPolicy::ExactOnly,
            ..UncertaintyPolicy::default()
        };

        let engine =
            UncertaintyEngine::new(policy).expect("valid");

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let input = UncertaintyInput::Bernoulli {
            successes: 50,
            trials: 100,
        };

        let result = engine.calculate(
            UncertaintyMethod::WilsonProportion,
            input,
            confidence,
        );

        assert!(matches!(
            result,
            Err(UncertaintyError::ApproximationNotAllowed { .. })
        ));
    }

    #[test]
    fn schema_is_nonzero() {
        assert!(UNCERTAINTY_SCHEMA_VERSION > 0);
        assert!(!UNCERTAINTY_SCHEMA_ID.is_empty());
    }

    #[test]
    fn standard_normal_quantile_is_symmetric() {
        let lower =
            standard_normal_quantile(0.025).expect("valid");

        let upper =
            standard_normal_quantile(0.975).expect("valid");

        assert!((lower + upper).abs() < 1.0e-10);
    }

    #[test]
    fn inverse_beta_round_trip_is_reasonable() {
        let x = 0.37;
        let a = 2.5;
        let b = 4.0;

        let probability =
            regularized_beta(
                x,
                a,
                b,
                DEFAULT_NUMERICAL_TOLERANCE,
                DEFAULT_MAX_ITERATIONS,
            )
            .expect("valid");

        let reconstructed =
            inverse_regularized_beta(
                probability,
                a,
                b,
                1.0e-10,
                DEFAULT_MAX_ITERATIONS,
            )
            .expect("valid");

        assert!((reconstructed - x).abs() < 1.0e-8);
    }

    #[test]
    fn interval_rejects_reversed_bounds() {
        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        assert!(
            Interval::new(
                1.0,
                0.0,
                IntervalKind::Confidence,
                Some(confidence),
            )
            .is_err()
        );
    }

    #[test]
    fn deterministic_interval_does_not_accept_confidence() {
        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        assert!(
            Interval::new(
                0.0,
                1.0,
                IntervalKind::Deterministic,
                Some(confidence),
            )
            .is_err()
        );
    }

    #[test]
    fn no_randomness_or_global_state_is_required() {
        // Compile-time/structural contract test:
        // UncertaintyEngine is Copy and contains only explicit policy.
        let first = UncertaintyEngine::default();
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn very_large_count_does_not_require_per_shot_storage() {
        let input = UncertaintyInput::Bernoulli {
            successes: u64::MAX - 1,
            trials: u64::MAX,
        };

        assert!(input.validate().is_ok());

        let engine = UncertaintyEngine::default();

        let confidence =
            ConfidenceLevel::new(0.95).expect("valid");

        let result = engine
            .wilson_proportion(input, confidence)
            .expect("valid");

        assert!(result.estimate > 0.0);
        assert!(result.estimate <= 1.0);
    }

    #[test]
    fn non_finite_bound_is_rejected() {
        let input = UncertaintyInput::BoundedMean {
            samples: 10,
            mean: 0.0,
            lower: f64::NEG_INFINITY,
            upper: 1.0,
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn invalid_policy_is_rejected() {
        let policy = UncertaintyPolicy {
            tolerance: 0.0,
            ..UncertaintyPolicy::default()
        };

        assert!(UncertaintyEngine::new(policy).is_err());
    }

    #[test]
    fn weighted_effective_sample_size_is_bounded_for_uniform_weights() {
        let result =
            effective_sample_size(4.0, 4.0).expect("valid");

        assert_eq!(result, 4.0);
    }

    #[test]
    fn beta_prior_uniform_is_valid() {
        let prior =
            BetaPrior::uniform().expect("valid");

        assert_eq!(prior.alpha, 1.0);
        assert_eq!(prior.beta, 1.0);
    }
}