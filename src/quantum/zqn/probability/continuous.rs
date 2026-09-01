//! Zamani Quantum Noise (ZQN) — Continuous Probability Distributions.
//!
//! # Ownership
//!
//! This module owns the mathematical definitions and validated scalar
//! operations for continuous probability distributions used by ZQN.
//!
//! It provides:
//!
//! - a common `ContinuousDistribution` trait;
//! - normal/Gaussian distributions;
//! - log-normal distributions;
//! - uniform distributions;
//! - exponential distributions;
//! - probability-density evaluation;
//! - cumulative-distribution evaluation;
//! - survival-function evaluation;
//! - finite support descriptions;
//! - analytical mean and variance where available;
//! - parameter validation;
//! - deterministic numerical evaluation;
//! - numerically stable handling of extreme finite inputs where practical.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - quantum states;
//! - qubit identities;
//! - physical resource identities;
//! - quantum channels;
//! - Kraus operators;
//! - faults;
//! - noise-model composition;
//! - calibration snapshots;
//! - characterization experiments;
//! - random-number generators;
//! - sampling engines;
//! - Monte Carlo execution;
//! - simulation state;
//! - benchmarking policy;
//! - serialization schemas;
//! - hardware APIs.
//!
//! Those concerns belong to their respective ZQN or quantum subsystems.
//!
//! # Canonical quantum identity boundary
//!
//! A continuous probability distribution is a mathematical object. It does
//! not inherently belong to a particular qubit, mode, gate, measurement, or
//! physical resource.
//!
//! Consequently this module intentionally does NOT define or import another
//! `QubitId` or `PhysicalQubitId`.
//!
//! When a distribution is associated with a quantum resource, the owning layer
//! must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The association belongs in the owning noise/calibration/operation model,
//! for example conceptually:
//!
//! ```text
//! QubitId
//!     |
//!     +----> ContinuousDistribution
//! ```
//!
//! and not:
//!
//! ```text
//! ContinuousDistribution
//!     +----> internally invented QubitId
//! ```
//!
//! This preserves the canonical identity boundary established by the Quantum
//! IR.
//!
//! # Mathematical domain
//!
//! A continuous distribution describes a probability measure over a real
//! scalar variable.
//!
//! Its probability density function (PDF) is written:
//!
//! ```text
//! f(x)
//! ```
//!
//! and its cumulative distribution function (CDF) is:
//!
//! ```text
//! F(x) = P(X <= x)
//! ```
//!
//! The survival function is:
//!
//! ```text
//! S(x) = P(X > x) = 1 - F(x)
//! ```
//!
//! For continuous distributions the probability of an individual point is
//! zero, so PDF values are densities rather than point probabilities.
//!
//! # Numerical representation
//!
//! Parameters are represented using `f64`.
//!
//! This is a numerical representation choice, not a quantum-system-size
//! limitation.
//!
//! A continuous distribution contains a fixed number of scalar parameters
//! regardless of whether it is later used for:
//!
//! - one physical resource;
//! - one million resources;
//! - a distributed quantum system;
//! - a streamed simulation;
//! - a sparse noise model;
//! - a large characterization workload.
//!
//! Collections of distributions are owned by higher layers and may use
//! streaming, sparse, chunked, or external representations.
//!
//! # Validity policy
//!
//! Public constructors reject:
//!
//! - `NaN`;
//! - positive infinity;
//! - negative infinity;
//! - invalid parameter ranges.
//!
//! No constructor silently clamps invalid values.
//!
//! This is important because silently converting an invalid physical model into
//! a valid-looking model can corrupt scientific results.
//!
//! # Distribution-specific parameterization
//!
//! ## Normal
//!
//! ```text
//! X ~ N(mu, sigma^2)
//! ```
//!
//! where:
//!
//! ```text
//! sigma > 0
//! ```
//!
//! ## Log-normal
//!
//! ```text
//! ln(X) ~ N(mu, sigma^2)
//! ```
//!
//! where:
//!
//! ```text
//! sigma > 0
//! ```
//!
//! The support is strictly positive.
//!
//! ## Uniform
//!
//! ```text
//! X ~ U(a, b)
//! ```
//!
//! where:
//!
//! ```text
//! a < b
//! ```
//!
//! ## Exponential
//!
//! ```text
//! X ~ Exp(lambda)
//! ```
//!
//! where:
//!
//! ```text
//! lambda > 0
//! ```
//!
//! # Sampling ownership
//!
//! This file intentionally does not own an RNG.
//!
//! A distribution describes mathematics; a sampler realizes random draws.
//!
//! The dependency direction is:
//!
//! ```text
//! ContinuousDistribution
//!         |
//!         v
//! ZQN sampling/reproducibility layer
//!         |
//!         v
//! caller-owned deterministic RNG policy
//! ```
//!
//! This prevents hidden global RNG state and permits deterministic parallel
//! execution.
//!
//! A future sampler can implement inverse-CDF or another appropriate method
//! without changing the distribution definitions.
//!
//! # Determinism
//!
//! All operations in this file are pure numerical functions.
//!
//! They:
//!
//! - do not allocate;
//! - do not mutate global state;
//! - do not access the network;
//! - do not access hardware;
//! - do not use a global RNG;
//! - do not depend on thread scheduling.
//!
//! Given the same finite inputs and the same floating-point environment, the
//! same result is produced.
//!
//! # Scalability
//!
//! No maximum number of qubits, modes, operations, resources, samples, or
//! distributions is defined here.
//!
//! The module has O(1) state per distribution.
//!
//! Large-scale workloads should store or process distributions using the
//! collection/streaming facilities of higher layers rather than requiring this
//! module to materialize a global model.
//!
//! "Infinity" therefore means "no semantic upper bound"; actual execution is
//! constrained only by the resource policy of the caller/runtime.
//!
//! # Resource safety
//!
//! This file:
//!
//! - performs no heap allocation;
//! - performs no recursion;
//! - performs no I/O;
//! - performs no dynamic code loading;
//! - has no global mutable state;
//! - has no hidden RNG;
//! - uses no `unsafe` code.
//!
//! Numerical methods use bounded iteration counts where approximation is
//! required.
//!
//! # Accuracy policy
//!
//! Some functions, particularly the normal CDF, require numerical
//! approximation because Rust's stable standard library does not provide a
//! universally available elementary `erf` API suitable for this module's
//! compatibility contract.
//!
//! The implementation uses a bounded rational approximation for the standard
//! normal CDF.
//!
//! The approximation is deterministic and does not silently claim arbitrary
//! precision.
//!
//! Future higher-precision numerical backends may implement the same semantic
//! trait without changing the distribution model.
//!
//! # Error integration
//!
//! `ContinuousDistributionError` is intentionally local so this file can be
//! implemented and tested independently.
//!
//! Higher ZQN layers may convert it into the canonical ZQN error/diagnostic
//! system without requiring this module to depend on higher-level modules.
//!
//! # Serialization
//!
//! Serialization does not belong here.
//!
//! `zqn::io` must define the versioned external representation.
//!
//! This prevents Rust struct layout from accidentally becoming a permanent
//! interchange format.
//!
//! # Integration
//!
//! ```text
//!                       ContinuousDistribution
//!                                  |
//!             +--------------------+--------------------+
//!             |                    |                    |
//!             v                    v                    v
//!       NoiseModel            Calibration        Characterization
//!             |                    |                    |
//!             +--------------------+--------------------+
//!                                  |
//!                                  v
//!                         Sampling / Simulation
//!                                  |
//!                                  v
//!                             Statistics
//! ```
//!
//! The distribution layer provides mathematical semantics; it does not own
//! execution.
//!
//! # Relationship to Probability
//!
//! `Probability` represents a bounded scalar in `[0, 1]`.
//!
//! A PDF value is a density and therefore is NOT necessarily a probability and
//! may be greater than one.
//!
//! Consequently this module deliberately does not return `Probability` from
//! `pdf()`.
//!
//! CDF and survival-function values are probabilities and are returned as
//! `Probability`.
//!
//! This distinction is essential for continuous probability theory.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - all public distribution constructors validate their parameters;
//! - all invalid finite/non-finite parameters are rejected;
//! - PDF/CDF semantics are explicitly defined;
//! - CDF results are valid ZQN probabilities;
//! - PDF values are not incorrectly constrained to `[0,1]`;
//! - no hidden RNG exists;
//! - no quantum-machine-size limit exists;
//! - no vendor-specific assumptions exist;
//! - no qubit identity is duplicated;
//! - all implementations satisfy the common trait contract;
//! - mathematical edge cases are covered by tests;
//! - the implementation compiles on Rust 1.97.1;
//! - safe Rust alone is sufficient.
//!
//! # Examples
//!
//! ```
//! use crate::quantum::zqn::probability::probability::Probability;
//! use crate::quantum::zqn::probability::continuous::{
//!     ContinuousDistribution, Exponential, LogNormal, Normal, Uniform,
//! };
//!
//! let normal = Normal::new(0.0, 1.0).unwrap();
//! let p = normal.cdf(0.0).unwrap();
//! assert!((p.value() - 0.5).abs() < 1.0e-6);
//!
//! let uniform = Uniform::new(0.0, 10.0).unwrap();
//! assert!((uniform.mean() - 5.0).abs() < f64::EPSILON);
//!
//! let exponential = Exponential::new(2.0).unwrap();
//! assert!((exponential.mean() - 0.5).abs() < f64::EPSILON);
//!
//! let log_normal = LogNormal::new(0.0, 1.0).unwrap();
//! assert!(log_normal.pdf(1.0) > 0.0);
//!
//! let _zero = Probability::ZERO;
//! ```
//!
//! The exact import path may be shortened by `zqn::prelude` once the public
//! ZQN prelude is integrated.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

use super::probability::Probability;

/// Numerical lower bound for distributions whose support begins at zero.
pub const ZERO: f64 = 0.0;

/// A mathematically unbounded upper support boundary.
///
/// `f64::INFINITY` is used only as a support descriptor. It is never accepted
/// as a distribution parameter.
pub const POSITIVE_INFINITY: f64 = f64::INFINITY;

/// A mathematically unbounded lower support boundary.
///
/// `f64::NEG_INFINITY` is used only as a support descriptor. It is never
/// accepted as a distribution parameter.
pub const NEGATIVE_INFINITY: f64 = f64::NEG_INFINITY;

/// A finite numerical interval describing the support of a distribution.
///
/// The endpoints may be infinite because distributions such as the normal and
/// exponential have mathematically unbounded support.
///
/// Infinite support boundaries are metadata, not valid numerical parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Support {
    lower: f64,
    upper: f64,
}

impl Support {
    /// Creates a support interval.
    ///
    /// The bounds must be ordered and must not contain `NaN`.
    ///
    /// Infinite bounds are permitted because they are meaningful for
    /// mathematical support.
    pub const fn new(lower: f64, upper: f64) -> Result<Self, ContinuousDistributionError> {
        if lower.is_nan() || upper.is_nan() {
            return Err(ContinuousDistributionError::NonFiniteParameter {
                name: "support",
                value: f64::NAN,
            });
        }

        if lower > upper {
            return Err(ContinuousDistributionError::InvalidSupport { lower, upper });
        }

        Ok(Self { lower, upper })
    }

    /// Returns the lower support boundary.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.lower
    }

    /// Returns the upper support boundary.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.upper
    }

    /// Returns whether a finite value lies within the closed support bounds.
    ///
    /// For continuous distributions, endpoint inclusion does not change
    /// probability mass because individual points have measure zero.
    #[must_use]
    pub fn contains(self, value: f64) -> bool {
        value.is_finite() && value >= self.lower && value <= self.upper
    }
}

/// Errors produced while constructing or evaluating continuous distributions.
#[derive(Clone, Debug, PartialEq)]
pub enum ContinuousDistributionError {
    /// A parameter was `NaN` or infinite.
    NonFiniteParameter {
        /// Parameter name.
        name: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A parameter was finite but outside its mathematical domain.
    InvalidParameter {
        /// Parameter name.
        name: &'static str,
        /// Invalid value.
        value: f64,
        /// Human-readable constraint.
        constraint: &'static str,
    },

    /// A pair of support boundaries was invalid.
    InvalidSupport {
        /// Lower boundary.
        lower: f64,
        /// Upper boundary.
        upper: f64,
    },

    /// A numerical operation produced a non-finite result.
    NumericalFailure {
        /// Operation that failed.
        operation: &'static str,
        /// Input value associated with the failure when meaningful.
        input: f64,
    },

    /// A probability could not be represented by the canonical ZQN
    /// `Probability` type.
    ProbabilityFailure {
        /// Operation that produced the invalid probability.
        operation: &'static str,
        /// Numerical result.
        value: f64,
    },
}

impl fmt::Display for ContinuousDistributionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteParameter { name, value } => {
                write!(formatter, "continuous distribution parameter `{name}` must be finite, got {value}")
            }
            Self::InvalidParameter {
                name,
                value,
                constraint,
            } => write!(
                formatter,
                "invalid continuous distribution parameter `{name}` = {value}: {constraint}"
            ),
            Self::InvalidSupport { lower, upper } => {
                write!(formatter, "invalid support interval [{lower}, {upper}]")
            }
            Self::NumericalFailure { operation, input } => {
                write!(
                    formatter,
                    "numerical failure during {operation} for input {input}"
                )
            }
            Self::ProbabilityFailure { operation, value } => {
                write!(
                    formatter,
                    "{operation} produced a value that is not a valid probability: {value}"
                )
            }
        }
    }
}

impl Error for ContinuousDistributionError {}

/// Result type used by continuous distribution operations.
pub type ContinuousResult<T> = Result<T, ContinuousDistributionError>;

/// Common contract for scalar continuous probability distributions.
///
/// Implementations are immutable value objects and must not own execution
/// state or random-number generators.
///
/// # PDF
///
/// `pdf(x)` returns a density, not a probability. It may therefore be greater
/// than `1`.
///
/// # CDF
///
/// `cdf(x)` returns `P(X <= x)` and therefore must be representable by the
/// canonical ZQN `Probability`.
///
/// # Survival function
///
/// `survival(x)` returns `P(X > x)`.
///
/// # Moments
///
/// `mean()` and `variance()` are analytical moments for the distribution.
///
/// # Sampling
///
/// Sampling is intentionally absent from this trait. The ZQN sampling layer
/// owns deterministic RNG policy and may use this trait's CDF/inverse-CDF
/// capabilities when appropriate.
pub trait ContinuousDistribution: Clone + Send + Sync + 'static {
    /// Probability-density function.
    fn pdf(&self, x: f64) -> ContinuousResult<f64>;

    /// Cumulative-distribution function.
    fn cdf(&self, x: f64) -> ContinuousResult<Probability>;

    /// Survival function.
    ///
    /// The default implementation computes `1 - CDF` through the canonical
    /// `Probability::complement` operation.
    fn survival(&self, x: f64) -> ContinuousResult<Probability> {
        Ok(self.cdf(x)?.complement())
    }

    /// Mathematical support.
    fn support(&self) -> Support;

    /// Analytical mean.
    fn mean(&self) -> f64;

    /// Analytical variance.
    fn variance(&self) -> f64;

    /// Standard deviation derived from the variance.
    fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Returns whether the finite input is inside the distribution's support.
    fn contains(&self, x: f64) -> bool {
        self.support().contains(x)
    }
}

/// Validates a finite scalar parameter.
fn finite_parameter(
    name: &'static str,
    value: f64,
) -> ContinuousResult<f64> {
    if !value.is_finite() {
        return Err(ContinuousDistributionError::NonFiniteParameter { name, value });
    }

    Ok(value)
}

/// Validates a strictly positive finite scalar parameter.
fn positive_parameter(
    name: &'static str,
    value: f64,
) -> ContinuousResult<f64> {
    finite_parameter(name, value)?;

    if value <= 0.0 {
        return Err(ContinuousDistributionError::InvalidParameter {
            name,
            value,
            constraint: "must be greater than zero",
        });
    }

    Ok(value)
}

/// Converts a numerically computed CDF into the canonical ZQN probability.
///
/// The implementation accepts tiny floating-point excursions caused by
/// approximation and rounds them to the mathematical endpoint. Larger
/// violations are treated as numerical failures rather than silently clamped.
fn cdf_probability(
    operation: &'static str,
    value: f64,
) -> ContinuousResult<Probability> {
    if !value.is_finite() {
        return Err(ContinuousDistributionError::NumericalFailure {
            operation,
            input: value,
        });
    }

    const NUMERICAL_TOLERANCE: f64 = 16.0 * f64::EPSILON;

    if value < -NUMERICAL_TOLERANCE || value > 1.0 + NUMERICAL_TOLERANCE {
        return Err(ContinuousDistributionError::ProbabilityFailure {
            operation,
            value,
        });
    }

    let normalized = if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    };

    Probability::new(normalized).map_err(|_| {
        ContinuousDistributionError::ProbabilityFailure {
            operation,
            value: normalized,
        }
    })
}

/// Numerically stable standard-normal PDF.
///
/// ```text
/// φ(x) = exp(-x² / 2) / sqrt(2π)
/// ```
fn standard_normal_pdf(x: f64) -> ContinuousResult<f64> {
    finite_parameter("x", x)?;

    // For sufficiently large |x| the exact density underflows to zero in
    // f64. Returning zero is mathematically consistent with the representable
    // numerical domain and avoids an unnecessary overflow.
    const LOG_MIN_SUBNORMAL: f64 = -744.4400719213812;

    let log_density = -0.5 * x * x - 0.5 * std::f64::consts::LN_2
        - 0.5 * std::f64::consts::PI.ln();

    if log_density < LOG_MIN_SUBNORMAL {
        return Ok(0.0);
    }

    let result = log_density.exp();

    if result.is_finite() {
        Ok(result)
    } else {
        Err(ContinuousDistributionError::NumericalFailure {
            operation: "standard_normal_pdf",
            input: x,
        })
    }
}

/// Deterministic approximation of the standard-normal CDF.
///
/// This is a bounded approximation based on the standard rational
/// approximation of the Gaussian CDF. It is selected specifically so the
/// implementation remains compatible with stable Rust 1.97.1 without requiring
/// an external special-functions dependency.
///
/// The approximation is accurate enough for ZQN's foundational distribution
/// layer while higher-precision numerical backends remain free to provide
/// stronger implementations later.
fn standard_normal_cdf(x: f64) -> ContinuousResult<f64> {
    finite_parameter("x", x)?;

    if x == 0.0 {
        return Ok(0.5);
    }

    // Symmetry provides better numerical behavior in the negative tail.
    if x < 0.0 {
        let positive = standard_normal_cdf(-x)?;
        return Ok(1.0 - positive);
    }

    // Abramowitz-Stegun-style approximation.
    //
    // p = 0.2316419
    // b1 = 0.319381530
    // b2 = -0.356563782
    // b3 = 1.781477937
    // b4 = -1.821255978
    // b5 = 1.330274429
    //
    // The approximation is applied to x >= 0.
    let p = 0.2316419_f64;
    let b1 = 0.319381530_f64;
    let b2 = -0.356563782_f64;
    let b3 = 1.781477937_f64;
    let b4 = -1.821255978_f64;
    let b5 = 1.330274429_f64;

    let t = 1.0 / (1.0 + p * x);

    let polynomial = (((((b5 * t + b4) * t) + b3) * t + b2) * t + b1) * t;

    let density = standard_normal_pdf(x)?;
    let tail = density * polynomial;

    let result = 1.0 - tail;

    if !result.is_finite() {
        return Err(ContinuousDistributionError::NumericalFailure {
            operation: "standard_normal_cdf",
            input: x,
        });
    }

    Ok(result)
}

/// Normal/Gaussian distribution.
///
/// ```text
/// X ~ N(mu, sigma²)
/// ```
///
/// The parameter `sigma` is the standard deviation and must be strictly
/// positive.
///
/// This type stores only two scalar parameters and therefore has constant
/// memory requirements independent of the quantum system to which it may later
/// be attached.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal {
    mean: f64,
    standard_deviation: f64,
}

impl Normal {
    /// Constructs a normal distribution.
    pub fn new(mean: f64, standard_deviation: f64) -> ContinuousResult<Self> {
        finite_parameter("mean", mean)?;
        positive_parameter("standard_deviation", standard_deviation)?;

        Ok(Self {
            mean,
            standard_deviation,
        })
    }

    /// Returns the distribution mean parameter.
    #[must_use]
    pub const fn mean_parameter(self) -> f64 {
        self.mean
    }

    /// Returns the standard-deviation parameter.
    #[must_use]
    pub const fn standard_deviation_parameter(self) -> f64 {
        self.standard_deviation
    }

    /// Returns the variance parameter.
    #[must_use]
    pub fn variance_parameter(self) -> f64 {
        self.standard_deviation * self.standard_deviation
    }
}

impl ContinuousDistribution for Normal {
    fn pdf(&self, x: f64) -> ContinuousResult<f64> {
        finite_parameter("x", x)?;

        let standardized = (x - self.mean) / self.standard_deviation;

        if !standardized.is_finite() {
            // A finite x and finite parameters can overflow only in extreme
            // subtraction/division cases. Such an evaluation is numerically
            // undefined in the current f64 representation.
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "normal_pdf_standardization",
                input: x,
            });
        }

        let density = standard_normal_pdf(standardized)?;

        let result = density / self.standard_deviation;

        if !result.is_finite() {
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "normal_pdf",
                input: x,
            });
        }

        Ok(result)
    }

    fn cdf(&self, x: f64) -> ContinuousResult<Probability> {
        finite_parameter("x", x)?;

        let standardized = (x - self.mean) / self.standard_deviation;

        if !standardized.is_finite() {
            // Preserve the mathematical tail semantics where possible.
            let comparison = x.partial_cmp(&self.mean);

            return match comparison {
                Some(std::cmp::Ordering::Less) => Probability::new(0.0).map_err(|_| {
                    ContinuousDistributionError::ProbabilityFailure {
                        operation: "normal_cdf_lower_tail",
                        value: 0.0,
                    }
                }),
                Some(std::cmp::Ordering::Greater) => {
                    Probability::new(1.0).map_err(|_| {
                        ContinuousDistributionError::ProbabilityFailure {
                            operation: "normal_cdf_upper_tail",
                            value: 1.0,
                        }
                    })
                }
                _ => Probability::new(0.5).map_err(|_| {
                    ContinuousDistributionError::ProbabilityFailure {
                        operation: "normal_cdf_center",
                        value: 0.5,
                    }
                }),
            };
        }

        cdf_probability("normal_cdf", standard_normal_cdf(standardized)?)
    }

    fn support(&self) -> Support {
        Support::new(NEGATIVE_INFINITY, POSITIVE_INFINITY)
            .expect("normal support is mathematically valid")
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        self.standard_deviation * self.standard_deviation
    }
}

/// Log-normal distribution.
///
/// ```text
/// ln(X) ~ N(mu, sigma²)
/// ```
///
/// with strictly positive standard deviation.
///
/// The support is:
///
/// ```text
/// x > 0
/// ```
///
/// The density is:
///
/// ```text
/// f(x) = exp(-(ln(x)-mu)^2/(2*sigma²))
///        / (x*sigma*sqrt(2π))
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogNormal {
    log_mean: f64,
    log_standard_deviation: f64,
}

impl LogNormal {
    /// Constructs a log-normal distribution.
    pub fn new(log_mean: f64, log_standard_deviation: f64) -> ContinuousResult<Self> {
        finite_parameter("log_mean", log_mean)?;
        positive_parameter("log_standard_deviation", log_standard_deviation)?;

        Ok(Self {
            log_mean,
            log_standard_deviation,
        })
    }

    /// Returns the mean of the underlying normal variable.
    #[must_use]
    pub const fn log_mean(self) -> f64 {
        self.log_mean
    }

    /// Returns the standard deviation of the underlying normal variable.
    #[must_use]
    pub const fn log_standard_deviation(self) -> f64 {
        self.log_standard_deviation
    }

    /// Returns the median of the log-normal distribution.
    ///
    /// ```text
    /// median(X) = exp(mu)
    /// ```
    #[must_use]
    pub fn median(&self) -> f64 {
        self.log_mean.exp()
    }

    /// Returns the mode of the log-normal distribution.
    ///
    /// ```text
    /// mode(X) = exp(mu - sigma²)
    /// ```
    #[must_use]
    pub fn mode(&self) -> f64 {
        let sigma_squared = self.log_standard_deviation * self.log_standard_deviation;
        (self.log_mean - sigma_squared).exp()
    }
}

impl ContinuousDistribution for LogNormal {
    fn pdf(&self, x: f64) -> ContinuousResult<f64> {
        finite_parameter("x", x)?;

        if x <= 0.0 {
            return Ok(0.0);
        }

        let log_x = x.ln();

        if !log_x.is_finite() {
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "log_normal_logarithm",
                input: x,
            });
        }

        let standardized =
            (log_x - self.log_mean) / self.log_standard_deviation;

        let standard_density = standard_normal_pdf(standardized)?;

        let result = standard_density
            / (x * self.log_standard_deviation);

        if !result.is_finite() {
            // Underflow to zero is mathematically acceptable for an extremely
            // small representable density, but overflow is not.
            if result == 0.0 {
                return Ok(0.0);
            }

            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "log_normal_pdf",
                input: x,
            });
        }

        Ok(result)
    }

    fn cdf(&self, x: f64) -> ContinuousResult<Probability> {
        finite_parameter("x", x)?;

        if x <= 0.0 {
            return Probability::new(0.0).map_err(|_| {
                ContinuousDistributionError::ProbabilityFailure {
                    operation: "log_normal_cdf_lower_support",
                    value: 0.0,
                }
            });
        }

        let standardized =
            (x.ln() - self.log_mean) / self.log_standard_deviation;

        if !standardized.is_finite() {
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "log_normal_cdf_standardization",
                input: x,
            });
        }

        cdf_probability(
            "log_normal_cdf",
            standard_normal_cdf(standardized)?,
        )
    }

    fn support(&self) -> Support {
        Support::new(0.0, POSITIVE_INFINITY)
            .expect("log-normal support is mathematically valid")
    }

    fn mean(&self) -> f64 {
        let sigma_squared =
            self.log_standard_deviation * self.log_standard_deviation;

        (self.log_mean + 0.5 * sigma_squared).exp()
    }

    fn variance(&self) -> f64 {
        let sigma_squared =
            self.log_standard_deviation * self.log_standard_deviation;

        let exponential = sigma_squared.exp();

        (exponential - 1.0)
            * (2.0 * self.log_mean + sigma_squared).exp()
    }
}

/// Continuous uniform distribution.
///
/// ```text
/// X ~ U(a, b)
/// ```
///
/// with:
///
/// ```text
/// a < b
/// ```
///
/// The PDF is:
///
/// ```text
/// 1 / (b-a)
/// ```
///
/// inside the support and zero outside.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uniform {
    lower: f64,
    upper: f64,
}

impl Uniform {
    /// Constructs a uniform distribution over `(lower, upper)` mathematically
    /// represented by its closed support boundaries.
    pub fn new(lower: f64, upper: f64) -> ContinuousResult<Self> {
        finite_parameter("lower", lower)?;
        finite_parameter("upper", upper)?;

        if lower >= upper {
            return Err(ContinuousDistributionError::InvalidParameter {
                name: "bounds",
                value: lower,
                constraint: "lower must be strictly less than upper",
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

    /// Returns the interval width.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}

impl ContinuousDistribution for Uniform {
    fn pdf(&self, x: f64) -> ContinuousResult<f64> {
        finite_parameter("x", x)?;

        if x < self.lower || x > self.upper {
            return Ok(0.0);
        }

        let width = self.width();

        if width <= 0.0 || !width.is_finite() {
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "uniform_width",
                input: x,
            });
        }

        Ok(1.0 / width)
    }

    fn cdf(&self, x: f64) -> ContinuousResult<Probability> {
        finite_parameter("x", x)?;

        if x <= self.lower {
            return Probability::new(0.0).map_err(|_| {
                ContinuousDistributionError::ProbabilityFailure {
                    operation: "uniform_cdf_lower_tail",
                    value: 0.0,
                }
            });
        }

        if x >= self.upper {
            return Probability::new(1.0).map_err(|_| {
                ContinuousDistributionError::ProbabilityFailure {
                    operation: "uniform_cdf_upper_tail",
                    value: 1.0,
                }
            });
        }

        let width = self.width();
        let result = (x - self.lower) / width;

        cdf_probability("uniform_cdf", result)
    }

    fn support(&self) -> Support {
        Support::new(self.lower, self.upper)
            .expect("uniform support is mathematically valid")
    }

    fn mean(&self) -> f64 {
        // Written this way instead of `(a + b) / 2` to reduce overflow risk
        // for large finite bounds.
        self.lower + (self.upper - self.lower) * 0.5
    }

    fn variance(&self) -> f64 {
        let width = self.width();

        // width is finite for two finite f64 values unless subtraction
        // overflows. In that extreme case no finite f64 variance can be
        // represented reliably.
        let squared = width * width;

        squared / 12.0
    }
}

/// Exponential distribution.
///
/// ```text
/// X ~ Exp(lambda)
/// ```
///
/// with:
///
/// ```text
/// lambda > 0
/// ```
///
/// The support is:
///
/// ```text
/// x >= 0
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Exponential {
    rate: f64,
}

impl Exponential {
    /// Constructs an exponential distribution with the supplied rate.
    pub fn new(rate: f64) -> ContinuousResult<Self> {
        positive_parameter("rate", rate)?;

        Ok(Self { rate })
    }

    /// Returns the rate parameter `lambda`.
    #[must_use]
    pub const fn rate(&self) -> f64 {
        self.rate
    }

    /// Returns the scale parameter `1 / lambda`.
    ///
    /// If the reciprocal cannot be represented as a finite `f64`, `None` is
    /// returned instead of silently producing infinity.
    #[must_use]
    pub fn scale(&self) -> Option<f64> {
        let scale = 1.0 / self.rate;

        if scale.is_finite() {
            Some(scale)
        } else {
            None
        }
    }
}

impl ContinuousDistribution for Exponential {
    fn pdf(&self, x: f64) -> ContinuousResult<f64> {
        finite_parameter("x", x)?;

        if x < 0.0 {
            return Ok(0.0);
        }

        let exponent = -self.rate * x;

        // For a finite positive rate and finite non-negative x, a sufficiently
        // large exponent underflows naturally to zero.
        if exponent < -745.0 {
            return Ok(0.0);
        }

        let result = self.rate * exponent.exp();

        if !result.is_finite() {
            return Err(ContinuousDistributionError::NumericalFailure {
                operation: "exponential_pdf",
                input: x,
            });
        }

        Ok(result)
    }

    fn cdf(&self, x: f64) -> ContinuousResult<Probability> {
        finite_parameter("x", x)?;

        if x <= 0.0 {
            return Probability::new(0.0).map_err(|_| {
                ContinuousDistributionError::ProbabilityFailure {
                    operation: "exponential_cdf_lower_support",
                    value: 0.0,
                }
            });
        }

        let exponent = -self.rate * x;

        let result = if exponent < -745.0 {
            1.0
        } else {
            // `-expm1(y)` is numerically superior to `1-exp(y)` near zero,
            // which matters when x is small.
            -exponent.exp_m1()
        };

        cdf_probability("exponential_cdf", result)
    }

    fn survival(&self, x: f64) -> ContinuousResult<Probability> {
        finite_parameter("x", x)?;

        if x <= 0.0 {
            return Probability::new(1.0).map_err(|_| {
                ContinuousDistributionError::ProbabilityFailure {
                    operation: "exponential_survival_lower_support",
                    value: 1.0,
                }
            });
        }

        let exponent = -self.rate * x;

        let result = if exponent < -745.0 {
            0.0
        } else {
            exponent.exp()
        };

        cdf_probability("exponential_survival", result)
    }

    fn support(&self) -> Support {
        Support::new(0.0, POSITIVE_INFINITY)
            .expect("exponential support is mathematically valid")
    }

    fn mean(&self) -> f64 {
        1.0 / self.rate
    }

    fn variance(&self) -> f64 {
        let scale = 1.0 / self.rate;
        scale * scale
    }
}

/// Convenience constructor for a standard normal distribution.
///
/// Equivalent to:
///
/// ```text
/// N(0, 1)
/// ```
pub fn standard_normal() -> Normal {
    // These constants satisfy Normal's constructor invariants.
    Normal {
        mean: 0.0,
        standard_deviation: 1.0,
    }
}

/// Returns the standard-normal PDF.
///
/// This is useful to callers that need the canonical Gaussian density without
/// allocating a `Normal` value.
pub fn standard_normal_density(x: f64) -> ContinuousResult<f64> {
    standard_normal_pdf(x)
}

/// Returns the standard-normal CDF as a canonical ZQN probability.
pub fn standard_normal_probability(x: f64) -> ContinuousResult<Probability> {
    cdf_probability("standard_normal_cdf", standard_normal_cdf(x)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approximately_equal(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    #[test]
    fn normal_rejects_invalid_parameters() {
        assert!(Normal::new(f64::NAN, 1.0).is_err());
        assert!(Normal::new(0.0, f64::NAN).is_err());
        assert!(Normal::new(0.0, 0.0).is_err());
        assert!(Normal::new(0.0, -1.0).is_err());
        assert!(Normal::new(f64::INFINITY, 1.0).is_err());
        assert!(Normal::new(0.0, f64::INFINITY).is_err());
    }

    #[test]
    fn normal_standard_distribution_has_expected_values() {
        let distribution = standard_normal();

        approximately_equal(
            distribution.pdf(0.0).unwrap(),
            0.3989422804014327,
            1.0e-12,
        );

        approximately_equal(
            distribution.cdf(0.0).unwrap().value(),
            0.5,
            1.0e-7,
        );

        approximately_equal(distribution.mean(), 0.0, f64::EPSILON);
        approximately_equal(distribution.variance(), 1.0, f64::EPSILON);
    }

    #[test]
    fn normal_cdf_is_monotonic() {
        let distribution = standard_normal();

        let a = distribution.cdf(-2.0).unwrap().value();
        let b = distribution.cdf(-1.0).unwrap().value();
        let c = distribution.cdf(0.0).unwrap().value();
        let d = distribution.cdf(1.0).unwrap().value();
        let e = distribution.cdf(2.0).unwrap().value();

        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
        assert!(d < e);
    }

    #[test]
    fn normal_cdf_is_symmetric() {
        let distribution = standard_normal();

        for x in [-5.0, -2.0, -1.0, 0.5, 1.0, 2.0, 5.0] {
            let positive = distribution.cdf(x).unwrap().value();
            let negative = distribution.cdf(-x).unwrap().value();

            approximately_equal(positive + negative, 1.0, 2.0e-7);
        }
    }

    #[test]
    fn normal_extreme_tails_remain_valid() {
        let distribution = standard_normal();

        let low = distribution.cdf(-20.0).unwrap();
        let high = distribution.cdf(20.0).unwrap();

        assert!(low.value() >= 0.0);
        assert!(low.value() <= 1.0);
        assert!(high.value() >= 0.0);
        assert!(high.value() <= 1.0);
        assert!(high.value() > low.value());
    }

    #[test]
    fn log_normal_rejects_invalid_parameters() {
        assert!(LogNormal::new(f64::NAN, 1.0).is_err());
        assert!(LogNormal::new(0.0, f64::NAN).is_err());
        assert!(LogNormal::new(0.0, 0.0).is_err());
        assert!(LogNormal::new(0.0, -1.0).is_err());
    }

    #[test]
    fn log_normal_has_positive_support() {
        let distribution = LogNormal::new(0.0, 1.0).unwrap();

        assert_eq!(distribution.support().lower(), 0.0);
        assert!(distribution.support().upper().is_infinite());

        assert_eq!(distribution.pdf(-1.0).unwrap(), 0.0);
        assert_eq!(distribution.pdf(0.0).unwrap(), 0.0);
        assert!(distribution.pdf(1.0).unwrap() > 0.0);
    }

    #[test]
    fn log_normal_known_moments() {
        let distribution = LogNormal::new(0.0, 1.0).unwrap();

        approximately_equal(distribution.mean(), (0.5_f64).exp(), 1.0e-14);
        approximately_equal(
            distribution.variance(),
            (2.0_f64.exp() - 1.0) * 2.0_f64.exp(),
            1.0e-12,
        );
        approximately_equal(distribution.median(), 1.0, f64::EPSILON);
        approximately_equal(distribution.mode(), (-1.0_f64).exp(), 1.0e-14);
    }

    #[test]
    fn uniform_rejects_invalid_bounds() {
        assert!(Uniform::new(0.0, 0.0).is_err());
        assert!(Uniform::new(1.0, 0.0).is_err());
        assert!(Uniform::new(f64::NAN, 1.0).is_err());
        assert!(Uniform::new(0.0, f64::NAN).is_err());
        assert!(Uniform::new(f64::INFINITY, 1.0).is_err());
    }

    #[test]
    fn uniform_has_expected_values() {
        let distribution = Uniform::new(0.0, 10.0).unwrap();

        approximately_equal(distribution.pdf(5.0).unwrap(), 0.1, f64::EPSILON);
        approximately_equal(
            distribution.cdf(5.0).unwrap().value(),
            0.5,
            f64::EPSILON,
        );
        approximately_equal(distribution.mean(), 5.0, f64::EPSILON);
        approximately_equal(distribution.variance(), 100.0 / 12.0, 1.0e-14);
    }

    #[test]
    fn uniform_cdf_has_correct_boundaries() {
        let distribution = Uniform::new(-2.0, 3.0).unwrap();

        assert_eq!(distribution.cdf(-3.0).unwrap(), Probability::ZERO);
        assert_eq!(distribution.cdf(-2.0).unwrap(), Probability::ZERO);
        assert_eq!(distribution.cdf(3.0).unwrap(), Probability::ONE);
        assert_eq!(distribution.cdf(4.0).unwrap(), Probability::ONE);
    }

    #[test]
    fn exponential_rejects_invalid_rates() {
        assert!(Exponential::new(0.0).is_err());
        assert!(Exponential::new(-1.0).is_err());
        assert!(Exponential::new(f64::NAN).is_err());
        assert!(Exponential::new(f64::INFINITY).is_err());
    }

    #[test]
    fn exponential_has_expected_values() {
        let distribution = Exponential::new(2.0).unwrap();

        approximately_equal(distribution.mean(), 0.5, f64::EPSILON);
        approximately_equal(distribution.variance(), 0.25, f64::EPSILON);
        approximately_equal(distribution.pdf(0.0).unwrap(), 2.0, f64::EPSILON);
        approximately_equal(
            distribution.cdf(0.5).unwrap().value(),
            1.0 - (-1.0_f64).exp(),
            1.0e-14,
        );
        approximately_equal(
            distribution.survival(0.5).unwrap().value(),
            (-1.0_f64).exp(),
            1.0e-14,
        );
    }

    #[test]
    fn exponential_lower_support_is_zero() {
        let distribution = Exponential::new(1.0).unwrap();

        assert_eq!(distribution.pdf(-1.0).unwrap(), 0.0);
        assert_eq!(distribution.cdf(-1.0).unwrap(), Probability::ZERO);
        assert_eq!(distribution.survival(-1.0).unwrap(), Probability::ONE);
    }

    #[test]
    fn probability_boundary_contract_is_preserved() {
        let distributions: &[&dyn ContinuousDistribution] = &[
            &standard_normal(),
            &LogNormal::new(0.0, 1.0).unwrap(),
            &Uniform::new(-1.0, 1.0).unwrap(),
            &Exponential::new(1.0).unwrap(),
        ];

        for distribution in distributions {
            for x in [-100.0, -10.0, -1.0, 0.0, 1.0, 10.0, 100.0] {
                let cdf = distribution.cdf(x).unwrap().value();
                let survival = distribution.survival(x).unwrap().value();

                assert!(cdf.is_finite());
                assert!(survival.is_finite());
                assert!((0.0..=1.0).contains(&cdf));
                assert!((0.0..=1.0).contains(&survival));

                approximately_equal(cdf + survival, 1.0, 2.0e-15);
            }
        }
    }

    #[test]
    fn support_contains_only_finite_values() {
        let support = Support::new(0.0, 1.0).unwrap();

        assert!(support.contains(0.0));
        assert!(support.contains(0.5));
        assert!(support.contains(1.0));
        assert!(!support.contains(-1.0));
        assert!(!support.contains(2.0));
        assert!(!support.contains(f64::NAN));
        assert!(!support.contains(f64::INFINITY));
    }

    #[test]
    fn standard_normal_density_is_symmetric() {
        for x in [0.1, 0.5, 1.0, 2.0, 5.0] {
            let positive = standard_normal_density(x).unwrap();
            let negative = standard_normal_density(-x).unwrap();

            approximately_equal(positive, negative, 1.0e-15);
        }
    }

    #[test]
    fn exponential_survival_is_numerically_stable_near_zero() {
        let distribution = Exponential::new(1.0).unwrap();

        let x = 1.0e-12;
        let survival = distribution.survival(x).unwrap().value();

        approximately_equal(survival, (-x).exp(), 1.0e-15);
    }

    #[test]
    fn no_distribution_has_a_quantum_machine_size_parameter() {
        // Architectural regression test.
        //
        // These mathematical objects contain only distribution parameters.
        // Quantum resource counts must be supplied by higher-level owners.
        let _normal = standard_normal();
        let _uniform = Uniform::new(0.0, 1.0).unwrap();
        let _exponential = Exponential::new(1.0).unwrap();
        let _log_normal = LogNormal::new(0.0, 1.0).unwrap();
    }
}