
#![forbid(unsafe_code)]

//! Zamani Standard Library — Scientific Probability
//!
//! # Purpose
//!
//! Provides portable probability mathematics and extensible distribution
//! and sampling contracts for classical, scientific, AI, hybrid, quantum,
//! distributed, HDL-related analysis, and future computational domains.
//!
//! # Ownership
//!
//! This module owns:
//! - validated scalar probability values;
//! - probability arithmetic;
//! - conditional and independent-event calculations;
//! - probability validation;
//! - generic distribution and random-variable contracts;
//! - caller-controlled sampling contexts;
//! - configurable sampling budgets;
//! - a Bernoulli distribution reference implementation.
//!
//! This module does not own:
//! - AI inference or probabilistic reasoning engines;
//! - quantum noise, measurement, or quantum simulation;
//! - compiler optimization or IR transformations;
//! - tensor or matrix storage;
//! - hardware discovery or allocation;
//! - runtime scheduling;
//! - cryptographic entropy-source selection.
//!
//! # Scalability
//!
//! No arbitrary maximum number of samples, random variables, distributions,
//! outcomes, nodes, qubits, devices, or computational resources is imposed.
//!
//! Optional sampling budgets are caller-selected execution policies.
//! They do not restrict the language's mathematical expressiveness.
//!
//! Collection sizes remain subject to actual available memory and Rust's
//! collection representation.
//!
//! # Randomness
//!
//! The caller supplies the random source. This module does not create global
//! random state, seed generators implicitly, or select hardware.
//!
//! `rand::RngCore` is used as a statistical random-source interface. It does
//! not by itself guarantee cryptographic security.
//!
//! Cryptographic key generation, nonces, and security-sensitive randomness
//! must use the repository's security/cryptographic provider contracts.
//!
//! # Integration
//!
//! - `numeric.rs`: retains compatibility with the existing `Prob = f64`.
//! - `ai/reasoning/probabilistic.rs`: consumes probability and distribution
//!   contracts through explicit adapters.
//! - `quantum/zqn/probability/`: retains quantum-specific probability,
//!   distribution, and sampling ownership.
//! - grammar: probability expressions are parsed by the existing grammar;
//!   this file implements mathematical behavior, not syntax.
//! - compiler/runtime: choose valid execution providers and resource plans.
//!
//! Rust 2021, Rust 1.97+, safe Rust only.

use rand::RngCore;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU64;

/// The existing standard-library probability representation.
///
/// `numeric::Prob` is currently an alias for `f64`. Reusing it avoids
/// introducing another raw floating-point alias with a competing name.
///
/// Use [`Probability`] when a validated probability invariant is required.
pub type ProbabilityValue = crate::stdlib::numeric::Prob;

/// Mathematical lower bound for a probability.
pub const MIN_PROBABILITY: f64 = 0.0;

/// Mathematical upper bound for a probability.
pub const MAX_PROBABILITY: f64 = 1.0;

/// Validated probability scalar.
///
/// Every safely constructed instance is finite and satisfies:
///
/// `0.0 <= value <= 1.0`.
///
/// The private representation prevents callers from bypassing validation.
#[derive(Clone, Copy, Debug)]
pub struct Probability(f64);

impl Probability {
    /// Exact zero probability.
    pub const ZERO: Self = Self(0.0);

    /// Exact unit probability.
    pub const ONE: Self = Self(1.0);

    /// Creates a validated probability.
    ///
    /// No clamping, rounding, or silent normalization is performed.
    pub fn new(value: f64) -> Result<Self, ProbabilityError> {
        if !value.is_finite() {
            return Err(ProbabilityError::NonFiniteProbability { value });
        }

        if !(MIN_PROBABILITY..=MAX_PROBABILITY).contains(&value) {
            return Err(ProbabilityError::ProbabilityOutOfRange { value });
        }

        // Canonicalize negative zero.
        let value = if value == 0.0 { 0.0 } else { value };

        Ok(Self(value))
    }

    /// Returns the underlying validated floating-point value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the underlying value as an explicit numerical conversion.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Returns whether the probability is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether the probability is exactly one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1.0
    }

    /// Returns the complementary probability: `1 - p`.
    #[must_use]
    pub fn complement(self) -> Self {
        Self(1.0 - self.0)
    }

    /// Returns the minimum of two probabilities.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two probabilities.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the absolute difference between two probabilities.
    #[must_use]
    pub fn abs_difference(self, other: Self) -> Self {
        Self((self.0 - other.0).abs())
    }

    /// Multiplies two probabilities.
    ///
    /// The result remains in `[0, 1]`.
    #[must_use]
    pub fn multiply(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    /// Adds probabilities when the result remains valid.
    ///
    /// This operation does not assume that the events are disjoint.
    pub fn checked_add(self, other: Self) -> Result<Self, ProbabilityError> {
        Self::new(self.0 + other.0)
    }

    /// Subtracts probabilities when the result remains valid.
    pub fn checked_sub(self, other: Self) -> Result<Self, ProbabilityError> {
        Self::new(self.0 - other.0)
    }

    /// Divides probabilities when the divisor is nonzero and the result
    /// remains a valid probability.
    pub fn checked_div(self, other: Self) -> Result<Self, ProbabilityError> {
        if other.is_zero() {
            return Err(ProbabilityError::DivisionByZero);
        }

        Self::new(self.0 / other.0)
    }

    /// Computes the probability of the intersection of independent events.
    ///
    /// `P(A ∩ B) = P(A) * P(B)`.
    #[must_use]
    pub fn independent_intersection(self, other: Self) -> Self {
        self.multiply(other)
    }

    /// Computes the probability of the union of independent events.
    ///
    /// `P(A ∪ B) = P(A) + P(B) - P(A)P(B)`.
    #[must_use]
    pub fn independent_union(self, other: Self) -> Self {
        let result = self.0 + other.0 - self.0 * other.0;

        // The expression is mathematically guaranteed to be in [0, 1].
        // The final constructor is intentionally not needed because the
        // validated inputs and formula preserve the invariant.
        Self(result.clamp(0.0, 1.0))
    }

    /// Computes conditional probability from a joint probability:
    ///
    /// `P(A | B) = P(A ∩ B) / P(B)`.
    ///
    /// The caller must supply a valid joint probability and conditioning
    /// probability. The result is validated rather than silently clamped.
    pub fn conditional(
        joint: Self,
        condition: Self,
    ) -> Result<Self, ProbabilityError> {
        if condition.is_zero() {
            return Err(ProbabilityError::ConditionHasZeroProbability);
        }

        joint.checked_div(condition)
    }

    /// Computes Bayes' theorem:
    ///
    /// `P(A | B) = P(B | A) P(A) / P(B)`.
    pub fn bayes(
        likelihood: Self,
        prior: Self,
        evidence: Self,
    ) -> Result<Self, ProbabilityError> {
        let joint = likelihood.multiply(prior);
        Self::conditional(joint, evidence)
    }
}

impl PartialEq for Probability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Probability {}

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl std::hash::Hash for Probability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // The constructor canonicalizes -0.0 to +0.0.
        self.0.to_bits().hash(state);
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Errors produced by probability operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ProbabilityError {
    /// A probability contained NaN or an infinity.
    NonFiniteProbability {
        value: f64,
    },

    /// A probability was outside the mathematical interval `[0, 1]`.
    ProbabilityOutOfRange {
        value: f64,
    },

    /// A division operation had a zero denominator.
    DivisionByZero,

    /// Conditional probability was requested for a zero-probability event.
    ConditionHasZeroProbability,

    /// A conditional probability was mathematically inconsistent.
    InvalidConditionalProbability {
        joint: f64,
        condition: f64,
    },

    /// A distribution or random variable name was empty.
    EmptyName,

    /// A distribution parameter was invalid.
    InvalidDistributionParameter {
        name: String,
        value: f64,
    },

    /// A sampling budget was exhausted.
    SamplingBudgetExceeded {
        limit: u64,
    },

    /// A sampling counter could not be incremented.
    SamplingCounterOverflow,

    /// The random source failed to provide bytes.
    RandomSourceFailure {
        message: String,
    },

    /// A requested operation is not provided by a distribution.
    UnsupportedDistributionOperation {
        operation: String,
    },

    /// A density value was invalid.
    InvalidDensity {
        value: f64,
    },
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteProbability { value } => {
                write!(formatter, "probability must be finite, received {value}")
            }
            Self::ProbabilityOutOfRange { value } => {
                write!(
                    formatter,
                    "probability must be in [0, 1], received {value}"
                )
            }
            Self::DivisionByZero => {
                write!(formatter, "probability division by zero")
            }
            Self::ConditionHasZeroProbability => {
                write!(
                    formatter,
                    "conditional probability is undefined for a zero-probability condition"
                )
            }
            Self::InvalidConditionalProbability { joint, condition } => {
                write!(
                    formatter,
                    "joint probability {joint} is inconsistent with condition probability {condition}"
                )
            }
            Self::EmptyName => {
                write!(formatter, "name must not be empty")
            }
            Self::InvalidDistributionParameter { name, value } => {
                write!(
                    formatter,
                    "invalid distribution parameter {name}: {value}"
                )
            }
            Self::SamplingBudgetExceeded { limit } => {
                write!(formatter, "sampling budget of {limit} draws was exhausted")
            }
            Self::SamplingCounterOverflow => {
                write!(formatter, "sampling counter overflow")
            }
            Self::RandomSourceFailure { message } => {
                write!(formatter, "random source failed: {message}")
            }
            Self::UnsupportedDistributionOperation { operation } => {
                write!(
                    formatter,
                    "distribution does not support operation: {operation}"
                )
            }
            Self::InvalidDensity { value } => {
                write!(
                    formatter,
                    "probability density must be finite and non-negative, received {value}"
                )
            }
        }
    }
}

impl Error for ProbabilityError {}

/// Validates a raw probability value.
///
/// This function is useful at FFI, semantic-adapter, deserialization,
/// and legacy `Prob = f64` boundaries.
pub fn validate_probability(value: f64) -> Result<Probability, ProbabilityError> {
    Probability::new(value)
}

/// Validates a probability density.
///
/// A probability density is not itself restricted to `[0, 1]`.
/// It must instead be finite and non-negative.
pub fn validate_density(value: f64) -> Result<f64, ProbabilityError> {
    if !value.is_finite() || value < 0.0 {
        return Err(ProbabilityError::InvalidDensity { value });
    }

    Ok(value)
}

/// Configurable sampling policy.
///
/// `max_draws = None` means no application-level draw budget is imposed.
///
/// A configured budget is an execution policy, not a mathematical limit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SamplingOptions {
    max_draws: Option<NonZeroU64>,
}

impl SamplingOptions {
    /// Creates an unlimited sampling policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_draws: None }
    }

    /// Creates a sampling policy with a configurable draw budget.
    #[must_use]
    pub const fn with_max_draws(max_draws: NonZeroU64) -> Self {
        Self {
            max_draws: Some(max_draws),
        }
    }

    /// Returns the configured maximum draw count.
    #[must_use]
    pub const fn max_draws(self) -> Option<NonZeroU64> {
        self.max_draws
    }
}

/// Caller-controlled sampling context.
///
/// The context borrows the caller's RNG. It does not own, initialize,
/// seed, replace, or select the random source.
///
/// A context can be reused across multiple distribution samples when the
/// caller wants one continuous random stream.
pub struct SamplingContext<'a, R: RngCore> {
    rng: &'a mut R,
    options: SamplingOptions,
    draws: u64,
}

impl<'a, R: RngCore> SamplingContext<'a, R> {
    /// Creates a sampling context using the supplied RNG and policy.
    #[must_use]
    pub fn new(rng: &'a mut R, options: SamplingOptions) -> Self {
        Self {
            rng,
            options,
            draws: 0,
        }
    }

    /// Creates a context without an application-level draw budget.
    #[must_use]
    pub fn unlimited(rng: &'a mut R) -> Self {
        Self::new(rng, SamplingOptions::unlimited())
    }

    /// Returns the number of random draws attempted.
    #[must_use]
    pub const fn draws(&self) -> u64 {
        self.draws
    }

    /// Returns the sampling policy.
    #[must_use]
    pub const fn options(&self) -> SamplingOptions {
        self.options
    }

    fn reserve_draw(&mut self) -> Result<(), ProbabilityError> {
        if let Some(limit) = self.options.max_draws {
            if self.draws >= limit.get() {
                return Err(ProbabilityError::SamplingBudgetExceeded {
                    limit: limit.get(),
                });
            }
        }

        self.draws = self
            .draws
            .checked_add(1)
            .ok_or(ProbabilityError::SamplingCounterOverflow)?;

        Ok(())
    }

    /// Returns a uniformly distributed floating-point value in `[0, 1)`.
    ///
    /// Exactly 53 random bits are used, matching the significand precision
    /// of IEEE-754 binary64 values.
    ///
    /// The caller controls the RNG and therefore controls reproducibility.
    pub fn uniform_unit_interval(&mut self) -> Result<f64, ProbabilityError> {
        self.reserve_draw()?;

        let mut bytes = [0_u8; 8];

        self.rng
            .try_fill_bytes(&mut bytes)
            .map_err(|error| ProbabilityError::RandomSourceFailure {
                message: error.to_string(),
            })?;

        let random = u64::from_le_bytes(bytes);
        let mantissa = random >> 11;

        Ok((mantissa as f64) * (1.0 / 9_007_199_254_740_992.0))
    }

    /// Samples an integer uniformly from `0..upper_exclusive`.
    ///
    /// Rejection sampling avoids modulo bias.
    pub fn uniform_index(
        &mut self,
        upper_exclusive: usize,
    ) -> Result<usize, ProbabilityError> {
        if upper_exclusive == 0 {
            return Err(ProbabilityError::InvalidDistributionParameter {
                name: "upper_exclusive".to_owned(),
                value: 0.0,
            });
        }

        let bound = u64::try_from(upper_exclusive).map_err(|_| {
            ProbabilityError::InvalidDistributionParameter {
                name: "upper_exclusive".to_owned(),
                value: upper_exclusive as f64,
            }
        })?;

        // The accepted range has a size divisible by `bound`.
        let threshold = bound.wrapping_neg() % bound;

        loop {
            self.reserve_draw()?;

            let mut bytes = [0_u8; 8];

            self.rng
                .try_fill_bytes(&mut bytes)
                .map_err(|error| ProbabilityError::RandomSourceFailure {
                    message: error.to_string(),
                })?;

            let value = u64::from_le_bytes(bytes);

            if value >= threshold {
                let index = value % bound;

                return usize::try_from(index).map_err(|_| {
                    ProbabilityError::InvalidDistributionParameter {
                        name: "sampled_index".to_owned(),
                        value: index as f64,
                    }
                });
            }
        }
    }
}

/// Extensible mathematical distribution contract.
///
/// A distribution owns its mathematical behavior, not its random source.
///
/// Implementations may represent:
/// - discrete distributions;
/// - continuous distributions;
/// - symbolic distributions;
/// - empirical distributions;
/// - conditional distributions;
/// - domain-specific distributions;
/// - future distribution families.
///
/// No closed distribution enumeration is imposed.
pub trait Distribution {
    /// The outcome type generated by this distribution.
    type Outcome;

    /// Draws one outcome using a caller-owned sampling context.
    fn sample<R: RngCore>(
        &self,
        context: &mut SamplingContext<'_, R>,
    ) -> Result<Self::Outcome, ProbabilityError>;

    /// Returns the probability mass of an outcome, if supported.
    ///
    /// Continuous distributions normally return `None` here and implement
    /// `density` instead.
    fn probability_mass(
        &self,
        _outcome: &Self::Outcome,
    ) -> Result<Option<Probability>, ProbabilityError> {
        Ok(None)
    }

    /// Returns the probability density of an outcome, if supported.
    ///
    /// A density may exceed one. It is not a probability mass.
    fn density(
        &self,
        _outcome: &Self::Outcome,
    ) -> Result<Option<f64>, ProbabilityError> {
        Ok(None)
    }

    /// Returns the cumulative probability at an outcome, if supported.
    fn cumulative_probability(
        &self,
        _outcome: &Self::Outcome,
    ) -> Result<Option<Probability>, ProbabilityError> {
        Ok(None)
    }
}

/// A named random variable associated with a distribution.
///
/// This is a mathematical composition. It does not create hidden random
/// state or perform sampling until explicitly requested.
#[derive(Clone, Debug)]
pub struct RandomVariable<D: Distribution> {
    name: String,
    distribution: D,
}

impl<D: Distribution> RandomVariable<D> {
    /// Creates a named random variable.
    pub fn new(
        name: impl Into<String>,
        distribution: D,
    ) -> Result<Self, ProbabilityError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ProbabilityError::EmptyName);
        }

        Ok(Self { name, distribution })
    }

    /// Returns the random-variable name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the associated distribution.
    #[must_use]
    pub fn distribution(&self) -> &D {
        &self.distribution
    }

    /// Samples one outcome using the supplied context.
    pub fn sample<R: RngCore>(
        &self,
        context: &mut SamplingContext<'_, R>,
    ) -> Result<D::Outcome, ProbabilityError> {
        self.distribution.sample(context)
    }
}

/// Bernoulli distribution.
///
/// Models a binary random variable:
///
/// - `true` with probability `p`;
/// - `false` with probability `1 - p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BernoulliDistribution {
    success_probability: Probability,
}

impl BernoulliDistribution {
    /// Creates a Bernoulli distribution.
    #[must_use]
    pub const fn new(success_probability: Probability) -> Self {
        Self {
            success_probability,
        }
    }

    /// Creates a Bernoulli distribution from a raw floating-point value.
    pub fn from_value(value: f64) -> Result<Self, ProbabilityError> {
        Ok(Self::new(Probability::new(value)?))
    }

    /// Returns the probability of success.
    #[must_use]
    pub const fn success_probability(self) -> Probability {
        self.success_probability
    }

    /// Returns the probability of failure.
    #[must_use]
    pub fn failure_probability(self) -> Probability {
        self.success_probability.complement()
    }
}

impl Distribution for BernoulliDistribution {
    type Outcome = bool;

    fn sample<R: RngCore>(
        &self,
        context: &mut SamplingContext<'_, R>,
    ) -> Result<Self::Outcome, ProbabilityError> {
        if self.success_probability.is_zero() {
            return Ok(false);
        }

        if self.success_probability.is_one() {
            return Ok(true);
        }

        let draw = context.uniform_unit_interval()?;

        Ok(draw < self.success_probability.value())
    }

    fn probability_mass(
        &self,
        outcome: &Self::Outcome,
    ) -> Result<Option<Probability>, ProbabilityError> {
        let probability = if *outcome {
            self.success_probability
        } else {
            self.failure_probability()
        };

        Ok(Some(probability))
    }

    fn cumulative_probability(
        &self,
        outcome: &Self::Outcome,
    ) -> Result<Option<Probability>, ProbabilityError> {
        let probability = if *outcome {
            Probability::ONE
        } else {
            self.failure_probability()
        };

        Ok(Some(probability))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn accepts_probability_endpoints() {
        assert_eq!(Probability::new(0.0).unwrap(), Probability::ZERO);
        assert_eq!(Probability::new(1.0).unwrap(), Probability::ONE);
    }

    #[test]
    fn rejects_non_finite_probabilities() {
        assert!(Probability::new(f64::NAN).is_err());
        assert!(Probability::new(f64::INFINITY).is_err());
        assert!(Probability::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_values_outside_probability_interval() {
        assert!(Probability::new(-0.01).is_err());
        assert!(Probability::new(1.01).is_err());
    }

    #[test]
    fn canonicalizes_negative_zero() {
        let probability = Probability::new(-0.0).unwrap();

        assert_eq!(probability.value().to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn complement_is_correct() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.complement().value(), 0.75);
    }

    #[test]
    fn multiplication_preserves_probability_invariant() {
        let first = Probability::new(0.5).unwrap();
        let second = Probability::new(0.4).unwrap();

        assert_eq!(first.multiply(second).value(), 0.2);
    }

    #[test]
    fn checked_add_rejects_invalid_probability_sum() {
        let first = Probability::new(0.7).unwrap();
        let second = Probability::new(0.6).unwrap();

        assert!(first.checked_add(second).is_err());
    }

    #[test]
    fn conditional_probability_is_correct() {
        let joint = Probability::new(0.2).unwrap();
        let condition = Probability::new(0.5).unwrap();

        let result = Probability::conditional(joint, condition).unwrap();

        assert_eq!(result.value(), 0.4);
    }

    #[test]
    fn conditional_probability_rejects_zero_condition() {
        assert_eq!(
            Probability::conditional(Probability::ZERO, Probability::ZERO),
            Err(ProbabilityError::ConditionHasZeroProbability)
        );
    }

    #[test]
    fn bayes_rule_is_correct() {
        let likelihood = Probability::new(0.8).unwrap();
        let prior = Probability::new(0.5).unwrap();
        let evidence = Probability::new(0.4).unwrap();

        let posterior =
            Probability::bayes(likelihood, prior, evidence).unwrap();

        assert_eq!(posterior.value(), 1.0);
    }

    #[test]
    fn independent_union_is_correct() {
        let first = Probability::new(0.5).unwrap();
        let second = Probability::new(0.5).unwrap();

        assert_eq!(first.independent_union(second).value(), 0.75);
    }

    #[test]
    fn density_is_not_restricted_to_one() {
        assert_eq!(validate_density(2.5).unwrap(), 2.5);
    }

    #[test]
    fn rejects_negative_or_non_finite_density() {
        assert!(validate_density(-1.0).is_err());
        assert!(validate_density(f64::NAN).is_err());
        assert!(validate_density(f64::INFINITY).is_err());
    }

    #[test]
    fn random_variable_requires_a_name() {
        let distribution =
            BernoulliDistribution::new(Probability::new(0.5).unwrap());

        assert!(RandomVariable::new(" ", distribution).is_err());
    }

    #[test]
    fn bernoulli_endpoint_sampling_needs_no_random_draw() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut context = SamplingContext::unlimited(&mut rng);

        let always_false =
            BernoulliDistribution::new(Probability::ZERO);

        let always_true =
            BernoulliDistribution::new(Probability::ONE);

        assert!(!always_false.sample(&mut context).unwrap());
        assert!(always_true.sample(&mut context).unwrap());
        assert_eq!(context.draws(), 0);
    }

    #[test]
    fn bernoulli_sampling_uses_caller_rng() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut context = SamplingContext::unlimited(&mut rng);

        let distribution =
            BernoulliDistribution::new(Probability::new(0.5).unwrap());

        let _ = distribution.sample(&mut context).unwrap();

        assert_eq!(context.draws(), 1);
    }

    #[test]
    fn sampling_budget_is_enforced() {
        let mut rng = StdRng::seed_from_u64(42);

        let options = SamplingOptions::with_max_draws(
            NonZeroU64::new(1).unwrap(),
        );

        let mut context = SamplingContext::new(&mut rng, options);

        context.uniform_unit_interval().unwrap();

        assert!(matches!(
            context.uniform_unit_interval(),
            Err(ProbabilityError::SamplingBudgetExceeded { limit: 1 })
        ));
    }

    #[test]
    fn seeded_sampling_is_reproducible() {
        let distribution =
            BernoulliDistribution::new(Probability::new(0.5).unwrap());

        let mut first_rng = StdRng::seed_from_u64(1234);
        let mut second_rng = StdRng::seed_from_u64(1234);

        let mut first_context = SamplingContext::unlimited(&mut first_rng);
        let mut second_context = SamplingContext::unlimited(&mut second_rng);

        let first = distribution.sample(&mut first_context).unwrap();
        let second = distribution.sample(&mut second_context).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn uniform_index_rejects_empty_range() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut context = SamplingContext::unlimited(&mut rng);

        assert!(context.uniform_index(0).is_err());
    }

    #[test]
    fn uniform_index_returns_value_inside_range() {
        let mut rng = StdRng::seed_from_u64(1);
        let mut context = SamplingContext::unlimited(&mut rng);

        let index = context.uniform_index(17).unwrap();

        assert!(index < 17);
    }
}
