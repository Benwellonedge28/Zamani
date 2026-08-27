//! Zamani Quantum Benchmarking — Statistical Validation
//!
//! Production validation boundary for statistical data used by the quantum
//! benchmarking subsystem.
//!
//! # Architectural role
//!
//! This module validates statistical INPUTS and statistical OUTPUT INVARIANTS.
//! It does not implement statistical estimators, confidence intervals,
//! regressions, bootstrap resampling, or hypothesis tests.
//!
//! The intended dependency direction is:
//!
//! ```text
//! raw observations
//!       │
//!       ▼
//! validation::statistical
//!       │
//!       ├──────────────► statistics::confidence
//!       ├──────────────► statistics::distributions
//!       ├──────────────► statistics::bootstrap
//!       ├──────────────► statistics::regression
//!       ├──────────────► statistics::hypothesis
//!       └──────────────► statistics::aggregation
//! ```
//!
//! Protocols may additionally use this module:
//!
//! ```text
//! protocol
//!    │
//!    ▼
//! statistical validation
//!    │
//!    ▼
//! statistical estimator
//!    │
//!    ▼
//! metric/result
//! ```
//!
//! # Responsibilities
//!
//! This module validates:
//!
//! - finite scalar values;
//! - probabilities;
//! - probabilities with protocol-specific tolerances;
//! - confidence levels;
//! - sample counts;
//! - success/failure counts;
//! - count/sample consistency;
//! - probability distributions;
//! - weighted observations;
//! - weights;
//! - non-empty sample sets;
//! - sample-set cardinality;
//! - paired observations;
//! - independent observation vectors;
//! - monotonic independent variables;
//! - regression inputs;
//! - regression outputs;
//! - fitted parameters;
//! - residuals;
//! - R-squared-like values;
//! - variance and standard deviation;
//! - standard errors;
//! - uncertainties;
//! - confidence bounds;
//! - effect sizes;
//! - finite differences;
//! - positive/negative quantities where required;
//! - bounded quantities;
//! - statistical convergence diagnostics;
//! - bootstrap configuration;
//! - hypothesis-test configuration;
//! - multiple-comparison parameters;
//! - numerical stability;
//! - overflow-safe sample products;
//! - representability of statistical workloads.
//!
//! # Non-responsibilities
//!
//! This module deliberately does NOT:
//!
//! - execute circuits;
//! - generate circuits;
//! - calculate confidence intervals;
//! - fit regressions;
//! - calculate p-values;
//! - perform bootstrap resampling;
//! - perform hypothesis tests;
//! - choose benchmark pass/fail criteria;
//! - access hardware;
//! - access the Quantum IR;
//! - access the runtime;
//! - print diagnostics;
//! - mutate process-global state.
//!
//! Those responsibilities belong to the statistical and protocol modules.
//!
//! # Integration
//!
//! The canonical error contract is:
//!
//! ```text
//! validation::statistical
//!          │
//!          ▼
//! core::errors::BenchmarkResult
//!          │
//!          ▼
//! statistics::*
//! ```
//!
//! `statistics::confidence` remains the canonical implementation of Wilson,
//! Clopper-Pearson, and other confidence intervals. This module only verifies
//! the values supplied to or returned by that implementation.
//!
//! `validation::input` remains the first validation boundary for complete
//! benchmark configurations. This module is the second boundary for numerical
//! and statistical data after observations have been produced.
//!
//! # Production safety
//!
//! Statistical validation is also a resource-safety boundary. Callers must not
//! be able to cause an accidental or malicious allocation by providing an
//! enormous number of bootstrap iterations, samples, dimensions, or paired
//! observations.
//!
//! The validator therefore checks:
//!
//! - non-zero workloads;
//! - configured maximums;
//! - safe integer multiplication;
//! - finite floating-point values;
//! - valid probability domains;
//! - valid uncertainty domains;
//! - valid statistical degrees of freedom;
//! - valid regression cardinality.
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
//! No additional dependencies are required.

use super::super::core::errors::{
    BenchmarkError,
    BenchmarkResult,
};

// =============================================================================
// Public component identity
// =============================================================================

/// Stable component identifier.
pub const STATISTICAL_VALIDATOR_COMPONENT_ID: &str =
    "zamani.quantum.benchmark.validation.statistical";

/// Stable validator contract version.
///
/// This version identifies the validation contract, not the Zamani compiler
/// version and not an individual statistical protocol version.
pub const STATISTICAL_VALIDATOR_VERSION: &str = "1.0.0";

// =============================================================================
// Production statistical limits
// =============================================================================

/// Maximum number of scalar observations accepted by one validation call.
///
/// This is intentionally finite even when a caller's broader benchmark limits
/// are larger. Statistical validation must never be an unbounded allocation
/// boundary.
pub const DEFAULT_MAX_OBSERVATIONS: usize = 10_000_000;

/// Maximum number of bootstrap/resampling iterations accepted by this boundary.
pub const DEFAULT_MAX_RESAMPLES: usize = 10_000_000;

/// Maximum number of independent dimensions in one statistical vector.
pub const DEFAULT_MAX_DIMENSIONS: usize = 1_000_000;

/// Maximum number of categories in a probability distribution.
pub const DEFAULT_MAX_CATEGORIES: usize = 1_000_000;

/// Maximum number of regression parameters.
pub const DEFAULT_MAX_PARAMETERS: usize = 1_000;

/// Maximum number of multiple-comparison hypotheses.
pub const DEFAULT_MAX_HYPOTHESES: usize = 1_000_000;

/// Maximum magnitude accepted for a finite floating-point value.
///
/// This prevents values which are technically finite but numerically
/// pathological from silently entering statistical calculations.
pub const DEFAULT_MAX_FINITE_MAGNITUDE: f64 = 1.0e300;

/// Minimum useful sample count for a variance estimate.
pub const MIN_VARIANCE_SAMPLES: usize = 2;

/// Minimum useful sample count for a standard-error estimate.
pub const MIN_STANDARD_ERROR_SAMPLES: usize = 2;

/// Minimum useful number of observations for an ordinary regression with an
/// intercept.
pub const MIN_REGRESSION_OBSERVATIONS: usize = 2;

/// Maximum probability-domain tolerance used only for boundary validation.
///
/// A value outside [0,1] by more than this tolerance is rejected. Values
/// inside the tolerance may be canonicalized by the consuming estimator.
pub const DEFAULT_PROBABILITY_TOLERANCE: f64 = 1.0e-12;

/// Maximum relative tolerance for sum-to-one validation.
pub const DEFAULT_DISTRIBUTION_SUM_TOLERANCE: f64 = 1.0e-10;

/// Default minimum confidence level accepted by the production validator.
pub const DEFAULT_MIN_CONFIDENCE_LEVEL: f64 = 0.5;

/// Default maximum confidence level.
///
/// Values arbitrarily close to one are numerically unstable and should not be
/// accepted without an explicit protocol-level policy.
pub const DEFAULT_MAX_CONFIDENCE_LEVEL: f64 = 0.999_999_999_999;

// =============================================================================
// Validation policy
// =============================================================================

/// Statistical validation policy.
///
/// This is explicit and copyable so a benchmark service can construct a
/// validator without global state or hidden configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalValidationPolicy {
    /// Maximum scalar observations accepted by one validation operation.
    pub max_observations: usize,

    /// Maximum bootstrap/resampling iterations.
    pub max_resamples: usize,

    /// Maximum vector dimensions.
    pub max_dimensions: usize,

    /// Maximum distribution categories.
    pub max_categories: usize,

    /// Maximum regression parameters.
    pub max_parameters: usize,

    /// Maximum number of hypotheses in a multiple-comparison operation.
    pub max_hypotheses: usize,

    /// Maximum accepted finite floating-point magnitude.
    pub max_finite_magnitude: f64,

    /// Probability-domain tolerance.
    pub probability_tolerance: f64,

    /// Distribution sum tolerance.
    pub distribution_sum_tolerance: f64,

    /// Minimum confidence level.
    pub min_confidence_level: f64,

    /// Maximum confidence level.
    pub max_confidence_level: f64,
}

impl Default for StatisticalValidationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

impl StatisticalValidationPolicy {
    /// Returns the production validation policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_observations: DEFAULT_MAX_OBSERVATIONS,
            max_resamples: DEFAULT_MAX_RESAMPLES,
            max_dimensions: DEFAULT_MAX_DIMENSIONS,
            max_categories: DEFAULT_MAX_CATEGORIES,
            max_parameters: DEFAULT_MAX_PARAMETERS,
            max_hypotheses: DEFAULT_MAX_HYPOTHESES,
            max_finite_magnitude: DEFAULT_MAX_FINITE_MAGNITUDE,
            probability_tolerance: DEFAULT_PROBABILITY_TOLERANCE,
            distribution_sum_tolerance: DEFAULT_DISTRIBUTION_SUM_TOLERANCE,
            min_confidence_level: DEFAULT_MIN_CONFIDENCE_LEVEL,
            max_confidence_level: DEFAULT_MAX_CONFIDENCE_LEVEL,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> BenchmarkResult<()> {
        if self.max_observations == 0 {
            return Err(invalid_configuration(
                "statistical.max_observations",
                "maximum observations must be greater than zero",
            ));
        }

        if self.max_resamples == 0 {
            return Err(invalid_configuration(
                "statistical.max_resamples",
                "maximum resamples must be greater than zero",
            ));
        }

        if self.max_dimensions == 0 {
            return Err(invalid_configuration(
                "statistical.max_dimensions",
                "maximum dimensions must be greater than zero",
            ));
        }

        if self.max_categories == 0 {
            return Err(invalid_configuration(
                "statistical.max_categories",
                "maximum categories must be greater than zero",
            ));
        }

        if self.max_parameters == 0 {
            return Err(invalid_configuration(
                "statistical.max_parameters",
                "maximum parameters must be greater than zero",
            ));
        }

        if self.max_hypotheses == 0 {
            return Err(invalid_configuration(
                "statistical.max_hypotheses",
                "maximum hypotheses must be greater than zero",
            ));
        }

        validate_finite_with_limit(
            "statistical.max_finite_magnitude",
            self.max_finite_magnitude,
            self.max_finite_magnitude,
        )?;

        if self.max_finite_magnitude <= 0.0 {
            return Err(invalid_configuration(
                "statistical.max_finite_magnitude",
                "maximum finite magnitude must be greater than zero",
            ));
        }

        validate_non_negative_finite(
            "statistical.probability_tolerance",
            self.probability_tolerance,
        )?;

        validate_non_negative_finite(
            "statistical.distribution_sum_tolerance",
            self.distribution_sum_tolerance,
        )?;

        validate_confidence_level_with_bounds(
            self.min_confidence_level,
            self.min_confidence_level,
            self.max_confidence_level,
        )?;

        validate_confidence_level_with_bounds(
            self.max_confidence_level,
            self.min_confidence_level,
            self.max_confidence_level,
        )?;

        if self.min_confidence_level >= self.max_confidence_level {
            return Err(invalid_configuration(
                "statistical.confidence_levels",
                "minimum confidence level must be strictly below maximum confidence level",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Validator
// =============================================================================

/// Stateless production statistical validator.
///
/// The validator contains only an explicit policy and is therefore safe to
/// construct per request or store as part of a benchmark service.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalValidator {
    policy: StatisticalValidationPolicy,
}

impl Default for StatisticalValidator {
    fn default() -> Self {
        Self::production()
    }
}

impl StatisticalValidator {
    /// Creates a production-safe validator.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            policy: StatisticalValidationPolicy::production(),
        }
    }

    /// Creates a validator with an explicit policy.
    #[must_use]
    pub const fn new(policy: StatisticalValidationPolicy) -> Self {
        Self { policy }
    }

    /// Returns the active policy.
    #[must_use]
    pub const fn policy(&self) -> &StatisticalValidationPolicy {
        &self.policy
    }

    /// Validates the policy.
    pub fn validate_policy(&self) -> BenchmarkResult<()> {
        self.policy.validate()
    }

    /// Validates a finite scalar.
    pub fn finite(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_finite_with_limit(
            field,
            value,
            self.policy.max_finite_magnitude,
        )?;

        Ok(value)
    }

    /// Validates a probability.
    pub fn probability(
        &self,
        field: &str,
        probability: f64,
    ) -> BenchmarkResult<f64> {
        validate_probability_with_tolerance(
            field,
            probability,
            self.policy.probability_tolerance,
        )
    }

    /// Validates a strict probability.
    ///
    /// Strict probabilities must be inside `(0, 1)`, rather than including
    /// the endpoints.
    pub fn strict_probability(
        &self,
        field: &str,
        probability: f64,
    ) -> BenchmarkResult<f64> {
        let value = self.probability(field, probability)?;

        if value <= 0.0 || value >= 1.0 {
            return Err(BenchmarkError::InvalidProbability {
                field: field.to_owned(),
                value: value.to_string(),
            });
        }

        Ok(value)
    }

    /// Validates a confidence level.
    pub fn confidence_level(
        &self,
        field: &str,
        level: f64,
    ) -> BenchmarkResult<f64> {
        validate_confidence_level_with_bounds(
            level,
            self.policy.min_confidence_level,
            self.policy.max_confidence_level,
        )
        .map_err(|error| {
            error.with_context(format!(
                "statistical confidence level '{}'",
                field
            ))
        })?;

        Ok(level)
    }

    /// Validates a positive integer sample count.
    pub fn samples(
        &self,
        field: &str,
        samples: usize,
    ) -> BenchmarkResult<usize> {
        validate_positive_count(field, samples)?;

        if samples > self.policy.max_observations {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: field.to_owned(),
                requested: samples as u64,
                maximum: self.policy.max_observations as u64,
            });
        }

        Ok(samples)
    }

    /// Validates a non-zero resampling count.
    pub fn resamples(
        &self,
        field: &str,
        resamples: usize,
    ) -> BenchmarkResult<usize> {
        validate_positive_count(field, resamples)?;

        if resamples > self.policy.max_resamples {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: field.to_owned(),
                requested: resamples as u64,
                maximum: self.policy.max_resamples as u64,
            });
        }

        Ok(resamples)
    }

    /// Validates a success/failure count pair.
    pub fn binomial_counts(
        &self,
        successes: usize,
        failures: usize,
    ) -> BenchmarkResult<usize> {
        let samples = successes.checked_add(failures).ok_or_else(|| {
            BenchmarkError::NumericalOverflow {
                operation: "successes + failures".to_owned(),
                value: None,
            }
        })?;

        self.samples("samples", samples)?;

        Ok(samples)
    }

    /// Validates a success count against a supplied sample count.
    pub fn success_count(
        &self,
        successes: usize,
        samples: usize,
    ) -> BenchmarkResult<()> {
        self.samples("samples", samples)?;

        if successes > samples {
            return Err(BenchmarkError::InvalidCount {
                field: "successes".to_owned(),
                value: successes as u64,
                maximum: Some(samples as u64),
            });
        }

        Ok(())
    }

    /// Validates an observation vector.
    pub fn observations(
        &self,
        field: &str,
        values: &[f64],
    ) -> BenchmarkResult<()> {
        self.validate_vector_size(field, values.len())?;

        if values.is_empty() {
            return Err(BenchmarkError::InsufficientSamples {
                required: 1,
                actual: 0,
                context: field.to_owned(),
            });
        }

        for (index, &value) in values.iter().enumerate() {
            self.finite(
                &format!("{}[{}]", field, index),
                value,
            )?;
        }

        Ok(())
    }

    /// Validates an observation vector requiring at least two observations.
    pub fn observations_for_variance(
        &self,
        field: &str,
        values: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(field, values)?;

        if values.len() < MIN_VARIANCE_SAMPLES {
            return Err(BenchmarkError::InsufficientSamples {
                required: MIN_VARIANCE_SAMPLES,
                actual: values.len(),
                context: field.to_owned(),
            });
        }

        Ok(())
    }

    /// Validates paired observations.
    pub fn paired_observations(
        &self,
        x_field: &str,
        x: &[f64],
        y_field: &str,
        y: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(x_field, x)?;
        self.observations(y_field, y)?;

        if x.len() != y.len() {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: x_field.to_owned(),
                second: y_field.to_owned(),
                reason: format!(
                    "paired observations must have equal lengths: {} != {}",
                    x.len(),
                    y.len()
                ),
            });
        }

        Ok(())
    }

    /// Validates a vector and checks its cardinality.
    pub fn validate_vector_size(
        &self,
        field: &str,
        length: usize,
    ) -> BenchmarkResult<()> {
        if length > self.policy.max_observations {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: field.to_owned(),
                requested: length as u64,
                maximum: self.policy.max_observations as u64,
            });
        }

        Ok(())
    }

    /// Validates a probability distribution.
    pub fn probability_distribution(
        &self,
        field: &str,
        probabilities: &[f64],
    ) -> BenchmarkResult<()> {
        if probabilities.is_empty() {
            return Err(BenchmarkError::InsufficientSamples {
                required: 1,
                actual: 0,
                context: format!("probability distribution '{}'", field),
            });
        }

        if probabilities.len() > self.policy.max_categories {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: format!("{}.categories", field),
                requested: probabilities.len() as u64,
                maximum: self.policy.max_categories as u64,
            });
        }

        let mut sum = 0.0f64;

        for (index, &probability) in probabilities.iter().enumerate() {
            let value = self.probability(
                &format!("{}[{}]", field, index),
                probability,
            )?;

            sum += value;

            if !sum.is_finite() {
                return Err(BenchmarkError::NonFiniteValue {
                    field: format!("{}.sum", field),
                    value: sum.to_string(),
                });
            }
        }

        if (sum - 1.0).abs() > self.policy.distribution_sum_tolerance {
            return Err(BenchmarkError::ValidationFailed {
                invariant: format!("{}.sum_to_one", field),
                reason: format!(
                    "probability distribution sums to {:.17}, expected 1.0 ± {:.3e}",
                    sum,
                    self.policy.distribution_sum_tolerance
                ),
            });
        }

        Ok(())
    }

    /// Validates non-negative weights.
    pub fn weights(
        &self,
        field: &str,
        weights: &[f64],
    ) -> BenchmarkResult<()> {
        if weights.is_empty() {
            return Err(BenchmarkError::InsufficientSamples {
                required: 1,
                actual: 0,
                context: field.to_owned(),
            });
        }

        self.validate_vector_size(field, weights.len())?;

        let mut total = 0.0f64;

        for (index, &weight) in weights.iter().enumerate() {
            validate_non_negative_finite(
                &format!("{}[{}]", field, index),
                weight,
            )?;

            total += weight;

            if !total.is_finite() {
                return Err(BenchmarkError::NonFiniteValue {
                    field: format!("{}.sum", field),
                    value: total.to_string(),
                });
            }
        }

        if total <= 0.0 {
            return Err(BenchmarkError::ValidationFailed {
                invariant: format!("{}.positive_total", field),
                reason: "at least one weight must be strictly positive".to_owned(),
            });
        }

        Ok(())
    }

    /// Validates weighted observations.
    pub fn weighted_observations(
        &self,
        values_field: &str,
        values: &[f64],
        weights_field: &str,
        weights: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(values_field, values)?;
        self.weights(weights_field, weights)?;

        if values.len() != weights.len() {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: values_field.to_owned(),
                second: weights_field.to_owned(),
                reason: format!(
                    "weighted observations require equal lengths: {} != {}",
                    values.len(),
                    weights.len()
                ),
            });
        }

        Ok(())
    }

    /// Validates a monotonically increasing independent-variable vector.
    pub fn strictly_increasing(
        &self,
        field: &str,
        values: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(field, values)?;

        for index in 1..values.len() {
            if values[index] <= values[index - 1] {
                return Err(BenchmarkError::InvalidRange {
                    field: field.to_owned(),
                    value: format!(
                        "{} <= {} at indices {} and {}",
                        values[index],
                        values[index - 1],
                        index,
                        index - 1
                    ),
                    minimum: Some(
                        "strictly increasing values".to_owned()
                    ),
                    maximum: None,
                });
            }
        }

        Ok(())
    }

    /// Validates a non-decreasing sequence.
    pub fn non_decreasing(
        &self,
        field: &str,
        values: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(field, values)?;

        for index in 1..values.len() {
            if values[index] < values[index - 1] {
                return Err(BenchmarkError::InvalidRange {
                    field: field.to_owned(),
                    value: format!(
                        "{} < {} at indices {} and {}",
                        values[index],
                        values[index - 1],
                        index,
                        index - 1
                    ),
                    minimum: Some(
                        "non-decreasing values".to_owned()
                    ),
                    maximum: None,
                });
            }
        }

        Ok(())
    }

    /// Validates an ordinary regression input set.
    ///
    /// This checks the data requirements but does not fit a model.
    pub fn regression_inputs(
        &self,
        x_field: &str,
        x: &[f64],
        y_field: &str,
        y: &[f64],
        parameter_count: usize,
    ) -> BenchmarkResult<()> {
        self.paired_observations(x_field, x, y_field, y)?;

        if x.len() < MIN_REGRESSION_OBSERVATIONS {
            return Err(BenchmarkError::InsufficientSamples {
                required: MIN_REGRESSION_OBSERVATIONS,
                actual: x.len(),
                context: "regression observations".to_owned(),
            });
        }

        if parameter_count == 0 {
            return Err(BenchmarkError::InvalidStatisticalModel {
                model: "regression".to_owned(),
                reason: "parameter count must be greater than zero".to_owned(),
            });
        }

        if parameter_count > self.policy.max_parameters {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "regression.parameters".to_owned(),
                requested: parameter_count as u64,
                maximum: self.policy.max_parameters as u64,
            });
        }

        if parameter_count >= x.len() {
            return Err(BenchmarkError::InvalidStatisticalModel {
                model: "regression".to_owned(),
                reason: format!(
                    "regression requires more observations than parameters: observations={}, parameters={}",
                    x.len(),
                    parameter_count
                ),
            });
        }

        Ok(())
    }

    /// Validates regression output parameters.
    pub fn regression_parameters(
        &self,
        field: &str,
        parameters: &[f64],
    ) -> BenchmarkResult<()> {
        if parameters.is_empty() {
            return Err(BenchmarkError::InvalidStatisticalModel {
                model: "regression".to_owned(),
                reason: "fitted parameter vector must not be empty".to_owned(),
            });
        }

        if parameters.len() > self.policy.max_parameters {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: format!("{}.parameters", field),
                requested: parameters.len() as u64,
                maximum: self.policy.max_parameters as u64,
            });
        }

        for (index, &parameter) in parameters.iter().enumerate() {
            self.finite(
                &format!("{}[{}]", field, index),
                parameter,
            )?;
        }

        Ok(())
    }

    /// Validates residuals.
    pub fn residuals(
        &self,
        field: &str,
        residuals: &[f64],
    ) -> BenchmarkResult<()> {
        self.observations(field, residuals)
    }

    /// Validates a coefficient-of-determination-like value.
    pub fn coefficient_of_determination(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        if !value.is_finite() {
            return Err(BenchmarkError::NonFiniteValue {
                field: field.to_owned(),
                value: value.to_string(),
            });
        }

        // R² can be negative for a poor model, although a fitted model
        // normally lies in [0,1]. We therefore allow a finite negative value
        // but reject values that are numerically pathological.
        if value.abs() > self.policy.max_finite_magnitude {
            return Err(BenchmarkError::NonFiniteValue {
                field: field.to_owned(),
                value: value.to_string(),
            });
        }

        Ok(value)
    }

    /// Validates variance.
    pub fn variance(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_non_negative_finite(
            field,
            value,
        )?;

        Ok(value)
    }

    /// Validates standard deviation.
    pub fn standard_deviation(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_non_negative_finite(
            field,
            value,
        )?;

        Ok(value)
    }

    /// Validates a strictly positive standard error.
    pub fn standard_error(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_positive_finite(field, value)?;

        Ok(value)
    }

    /// Validates a non-negative uncertainty.
    pub fn uncertainty(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_non_negative_finite(field, value)?;

        Ok(value)
    }

    /// Validates confidence bounds.
    pub fn confidence_bounds(
        &self,
        field: &str,
        lower: f64,
        upper: f64,
    ) -> BenchmarkResult<()> {
        self.probability(
            &format!("{}.lower", field),
            lower,
        )?;

        self.probability(
            &format!("{}.upper", field),
            upper,
        )?;

        if lower > upper {
            return Err(BenchmarkError::ValidationFailed {
                invariant: format!("{}.ordered", field),
                reason: format!(
                    "lower bound {} exceeds upper bound {}",
                    lower,
                    upper
                ),
            });
        }

        Ok(())
    }

    /// Validates a bounded interval without assuming a probability domain.
    pub fn bounded_interval(
        &self,
        field: &str,
        lower: f64,
        upper: f64,
        minimum: f64,
        maximum: f64,
    ) -> BenchmarkResult<()> {
        self.finite(
            &format!("{}.lower", field),
            lower,
        )?;

        self.finite(
            &format!("{}.upper", field),
            upper,
        )?;

        self.finite(
            &format!("{}.minimum", field),
            minimum,
        )?;

        self.finite(
            &format!("{}.maximum", field),
            maximum,
        )?;

        if minimum > maximum {
            return Err(BenchmarkError::InvalidRange {
                field: field.to_owned(),
                value: format!(
                    "minimum {} > maximum {}",
                    minimum,
                    maximum
                ),
                minimum: None,
                maximum: None,
            });
        }

        if lower < minimum || upper > maximum || lower > upper {
            return Err(BenchmarkError::ValidationFailed {
                invariant: format!("{}.bounds", field),
                reason: format!(
                    "interval [{}, {}] must lie within [{}, {}]",
                    lower,
                    upper,
                    minimum,
                    maximum
                ),
            });
        }

        Ok(())
    }

    /// Validates an effect size.
    pub fn effect_size(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        self.finite(field, value)
    }

    /// Validates a finite difference.
    pub fn finite_difference(
        &self,
        x_field: &str,
        x1: f64,
        x2: f64,
        y_field: &str,
        y1: f64,
        y2: f64,
    ) -> BenchmarkResult<f64> {
        self.finite(
            &format!("{}.1", x_field),
            x1,
        )?;

        self.finite(
            &format!("{}.2", x_field),
            x2,
        )?;

        self.finite(
            &format!("{}.1", y_field),
            y1,
        )?;

        self.finite(
            &format!("{}.2", y_field),
            y2,
        )?;

        let denominator = x2 - x1;

        if denominator == 0.0 {
            return Err(BenchmarkError::ValidationFailed {
                invariant: "finite_difference.non_zero_denominator".to_owned(),
                reason: "independent-variable values must differ".to_owned(),
            });
        }

        let numerator = y2 - y1;

        let result = numerator / denominator;

        self.finite(
            "finite_difference.result",
            result,
        )?;

        Ok(result)
    }

    /// Validates bootstrap configuration.
    pub fn bootstrap_configuration(
        &self,
        samples: usize,
        resamples: usize,
    ) -> BenchmarkResult<()> {
        self.samples("bootstrap.samples", samples)?;
        self.resamples("bootstrap.resamples", resamples)?;

        let _ = checked_product(
            samples,
            resamples,
            "bootstrap.samples * bootstrap.resamples",
        )?;

        Ok(())
    }

    /// Validates a multiple-comparison hypothesis count.
    pub fn hypothesis_count(
        &self,
        hypotheses: usize,
    ) -> BenchmarkResult<usize> {
        validate_positive_count(
            "hypotheses",
            hypotheses,
        )?;

        if hypotheses > self.policy.max_hypotheses {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "hypotheses".to_owned(),
                requested: hypotheses as u64,
                maximum: self.policy.max_hypotheses as u64,
            });
        }

        Ok(hypotheses)
    }

    /// Validates degrees of freedom.
    pub fn degrees_of_freedom(
        &self,
        field: &str,
        degrees: usize,
    ) -> BenchmarkResult<usize> {
        if degrees == 0 {
            return Err(BenchmarkError::InvalidStatisticalModel {
                model: field.to_owned(),
                reason: "degrees of freedom must be greater than zero".to_owned(),
            });
        }

        Ok(degrees)
    }

    /// Validates an effective sample size.
    pub fn effective_sample_size(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_positive_finite(
            field,
            value,
        )?;

        if value > self.policy.max_observations as f64 {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: field.to_owned(),
                requested: value as u64,
                maximum: self.policy.max_observations as u64,
            });
        }

        Ok(value)
    }

    /// Validates a statistical estimate together with its uncertainty.
    pub fn estimate_with_uncertainty(
        &self,
        estimate_field: &str,
        estimate: f64,
        uncertainty_field: &str,
        uncertainty: f64,
    ) -> BenchmarkResult<()> {
        self.finite(
            estimate_field,
            estimate,
        )?;

        self.uncertainty(
            uncertainty_field,
            uncertainty,
        )?;

        Ok(())
    }

    /// Validates a metric whose natural domain is [0,1].
    pub fn unit_interval_metric(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        self.probability(field, value)
    }

    /// Validates a non-negative rate.
    pub fn non_negative_rate(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_non_negative_finite(
            field,
            value,
        )?;

        Ok(value)
    }

    /// Validates a strictly positive rate.
    pub fn positive_rate(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        validate_positive_finite(
            field,
            value,
        )?;

        Ok(value)
    }

    /// Validates a p-value.
    pub fn p_value(
        &self,
        field: &str,
        value: f64,
    ) -> BenchmarkResult<f64> {
        self.probability(field, value)
    }

    /// Validates a significance level.
    pub fn significance_level(
        &self,
        field: &str,
        alpha: f64,
    ) -> BenchmarkResult<f64> {
        if !alpha.is_finite() {
            return Err(BenchmarkError::NonFiniteValue {
                field: field.to_owned(),
                value: alpha.to_string(),
            });
        }

        if alpha <= 0.0 || alpha >= 1.0 {
            return Err(BenchmarkError::InvalidRange {
                field: field.to_owned(),
                value: alpha.to_string(),
                minimum: Some("(0,1)".to_owned()),
                maximum: Some("(0,1)".to_owned()),
            });
        }

        Ok(alpha)
    }
}

// =============================================================================
// Standalone validation functions
// =============================================================================

/// Validates a finite value without a custom policy.
pub fn validate_finite(
    field: &str,
    value: f64,
) -> BenchmarkResult<f64> {
    validate_finite_with_limit(
        field,
        value,
        DEFAULT_MAX_FINITE_MAGNITUDE,
    )
}

/// Validates a finite value and rejects numerically pathological magnitudes.
pub fn validate_finite_with_limit(
    field: &str,
    value: f64,
    maximum_magnitude: f64,
) -> BenchmarkResult<f64> {
    if !maximum_magnitude.is_finite() || maximum_magnitude <= 0.0 {
        return Err(invalid_configuration(
            "maximum_magnitude",
            "finite maximum magnitude must be greater than zero",
        ));
    }

    if !value.is_finite() {
        return Err(BenchmarkError::NonFiniteValue {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    if value.abs() > maximum_magnitude {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.magnitude", field),
            reason: format!(
                "absolute value {} exceeds permitted magnitude {}",
                value.abs(),
                maximum_magnitude
            ),
        });
    }

    Ok(value)
}

/// Validates a probability using the production tolerance.
pub fn validate_probability(
    field: &str,
    probability: f64,
) -> BenchmarkResult<f64> {
    validate_probability_with_tolerance(
        field,
        probability,
        DEFAULT_PROBABILITY_TOLERANCE,
    )
}

/// Validates a probability with an explicit tolerance.
pub fn validate_probability_with_tolerance(
    field: &str,
    probability: f64,
    tolerance: f64,
) -> BenchmarkResult<f64> {
    validate_non_negative_finite(
        "probability.tolerance",
        tolerance,
    )?;

    if !probability.is_finite() {
        return Err(BenchmarkError::InvalidProbability {
            field: field.to_owned(),
            value: probability.to_string(),
        });
    }

    if probability < -tolerance || probability > 1.0 + tolerance {
        return Err(BenchmarkError::InvalidProbability {
            field: field.to_owned(),
            value: probability.to_string(),
        });
    }

    // Do not silently clamp the value. The validator reports the original
    // number; the consuming estimator may decide whether a tiny numerical
    // overshoot should be canonicalized.
    Ok(probability)
}

/// Validates a confidence level against explicit production bounds.
pub fn validate_confidence_level(
    field: &str,
    level: f64,
) -> BenchmarkResult<f64> {
    validate_confidence_level_with_bounds(
        level,
        DEFAULT_MIN_CONFIDENCE_LEVEL,
        DEFAULT_MAX_CONFIDENCE_LEVEL,
    )
    .map_err(|error| error.with_context(field.to_owned()))
}

/// Validates a confidence level with explicit bounds.
pub fn validate_confidence_level_with_bounds(
    level: f64,
    minimum: f64,
    maximum: f64,
) -> BenchmarkResult<f64> {
    if !minimum.is_finite() || !maximum.is_finite() {
        return Err(invalid_configuration(
            "confidence_level.bounds",
            "confidence bounds must be finite",
        ));
    }

    if minimum <= 0.0 || maximum >= 1.0 || minimum >= maximum {
        return Err(invalid_configuration(
            "confidence_level.bounds",
            "bounds must satisfy 0 < minimum < maximum < 1",
        ));
    }

    if !level.is_finite() {
        return Err(BenchmarkError::InvalidConfidenceLevel {
            value: level.to_string(),
        });
    }

    if level < minimum || level > maximum {
        return Err(BenchmarkError::InvalidConfidenceLevel {
            value: level.to_string(),
        });
    }

    Ok(level)
}

/// Validates a positive integer count.
pub fn validate_positive_count(
    field: &str,
    count: usize,
) -> BenchmarkResult<usize> {
    if count == 0 {
        return Err(BenchmarkError::InvalidCount {
            field: field.to_owned(),
            value: 0,
            maximum: None,
        });
    }

    Ok(count)
}

/// Validates a non-negative finite number.
pub fn validate_non_negative_finite(
    field: &str,
    value: f64,
) -> BenchmarkResult<f64> {
    if !value.is_finite() {
        return Err(BenchmarkError::NonFiniteValue {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    if value < 0.0 {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.non_negative", field),
            reason: format!("value {} is negative", value),
        });
    }

    Ok(value)
}

/// Validates a strictly positive finite number.
pub fn validate_positive_finite(
    field: &str,
    value: f64,
) -> BenchmarkResult<f64> {
    if !value.is_finite() {
        return Err(BenchmarkError::NonFiniteValue {
            field: field.to_owned(),
            value: value.to_string(),
        });
    }

    if value <= 0.0 {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.positive", field),
            reason: format!("value {} must be strictly positive", value),
        });
    }

    Ok(value)
}

/// Validates that a count can safely be multiplied by another count.
pub fn checked_product(
    first: usize,
    second: usize,
    operation: &str,
) -> BenchmarkResult<usize> {
    first.checked_mul(second).ok_or_else(|| {
        BenchmarkError::NumericalOverflow {
            operation: operation.to_owned(),
            value: None,
        }
    })
}

/// Validates that several dimensions can safely be multiplied.
pub fn checked_product3(
    first: usize,
    second: usize,
    third: usize,
    operation: &str,
) -> BenchmarkResult<usize> {
    let first_product = checked_product(
        first,
        second,
        operation,
    )?;

    checked_product(
        first_product,
        third,
        operation,
    )
}

/// Validates a count sum without overflow.
pub fn checked_sum(
    first: usize,
    second: usize,
    operation: &str,
) -> BenchmarkResult<usize> {
    first.checked_add(second).ok_or_else(|| {
        BenchmarkError::NumericalOverflow {
            operation: operation.to_owned(),
            value: None,
        }
    })
}

// =============================================================================
// Distribution helpers
// =============================================================================

/// Validates count probabilities against a total shot count.
///
/// This is useful when a backend returns integer counts and a downstream
/// metric converts them into probabilities.
pub fn validate_count_distribution(
    counts: &[usize],
    total_shots: usize,
) -> BenchmarkResult<()> {
    validate_positive_count(
        "total_shots",
        total_shots,
    )?;

    if counts.is_empty() {
        return Err(BenchmarkError::InsufficientSamples {
            required: 1,
            actual: 0,
            context: "count distribution".to_owned(),
        });
    }

    let mut sum = 0usize;

    for (index, &count) in counts.iter().enumerate() {
        sum = sum.checked_add(count).ok_or_else(|| {
            BenchmarkError::NumericalOverflow {
                operation: "count distribution sum".to_owned(),
                value: None,
            }
        })?;

        if count > total_shots {
            return Err(BenchmarkError::InvalidCount {
                field: format!("counts[{}]", index),
                value: count as u64,
                maximum: Some(total_shots as u64),
            });
        }
    }

    if sum != total_shots {
        return Err(BenchmarkError::ValidationFailed {
            invariant: "count_distribution.total".to_owned(),
            reason: format!(
                "count sum {} does not equal total shots {}",
                sum,
                total_shots
            ),
        });
    }

    Ok(())
}

/// Validates a collection of categorical probabilities.
///
/// Unlike `validate_probability_distribution`, this helper is public and
/// policy-free for use by small statistical utilities.
pub fn validate_probability_distribution(
    probabilities: &[f64],
) -> BenchmarkResult<()> {
    if probabilities.is_empty() {
        return Err(BenchmarkError::InsufficientSamples {
            required: 1,
            actual: 0,
            context: "probability distribution".to_owned(),
        });
    }

    let mut sum = 0.0f64;

    for (index, &probability) in probabilities.iter().enumerate() {
        let probability = validate_probability(
            &format!("probabilities[{}]", index),
            probability,
        )?;

        sum += probability;

        if !sum.is_finite() {
            return Err(BenchmarkError::NonFiniteValue {
                field: "probabilities.sum".to_owned(),
                value: sum.to_string(),
            });
        }
    }

    if (sum - 1.0).abs() > DEFAULT_DISTRIBUTION_SUM_TOLERANCE {
        return Err(BenchmarkError::ValidationFailed {
            invariant: "probability_distribution.sum_to_one".to_owned(),
            reason: format!(
                "sum {} differs from 1.0 by more than {}",
                sum,
                DEFAULT_DISTRIBUTION_SUM_TOLERANCE
            ),
        });
    }

    Ok(())
}

// =============================================================================
// Statistical-output validation
// =============================================================================

/// Validates a mean-like estimate.
pub fn validate_mean(
    field: &str,
    mean: f64,
) -> BenchmarkResult<f64> {
    validate_finite(field, mean)
}

/// Validates a variance estimate.
pub fn validate_variance(
    field: &str,
    variance: f64,
) -> BenchmarkResult<f64> {
    validate_non_negative_finite(
        field,
        variance,
    )
}

/// Validates a standard deviation estimate.
pub fn validate_standard_deviation(
    field: &str,
    standard_deviation: f64,
) -> BenchmarkResult<f64> {
    validate_non_negative_finite(
        field,
        standard_deviation,
    )
}

/// Validates a standard error estimate.
pub fn validate_standard_error(
    field: &str,
    standard_error: f64,
) -> BenchmarkResult<f64> {
    validate_positive_finite(
        field,
        standard_error,
    )
}

/// Validates an uncertainty.
pub fn validate_uncertainty(
    field: &str,
    uncertainty: f64,
) -> BenchmarkResult<f64> {
    validate_non_negative_finite(
        field,
        uncertainty,
    )
}

/// Validates ordered confidence bounds.
pub fn validate_confidence_bounds(
    field: &str,
    lower: f64,
    upper: f64,
) -> BenchmarkResult<()> {
    validate_probability(
        &format!("{}.lower", field),
        lower,
    )?;

    validate_probability(
        &format!("{}.upper", field),
        upper,
    )?;

    if lower > upper {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.ordered", field),
            reason: format!(
                "lower {} is greater than upper {}",
                lower,
                upper
            ),
        });
    }

    Ok(())
}

/// Validates an estimated value together with its uncertainty.
pub fn validate_estimate(
    estimate_field: &str,
    estimate: f64,
    uncertainty_field: &str,
    uncertainty: f64,
) -> BenchmarkResult<()> {
    validate_finite(
        estimate_field,
        estimate,
    )?;

    validate_uncertainty(
        uncertainty_field,
        uncertainty,
    )?;

    Ok(())
}

/// Validates a regression fit's essential output invariants.
pub fn validate_regression_output(
    model: &str,
    parameters: &[f64],
    residuals: &[f64],
    r_squared: Option<f64>,
) -> BenchmarkResult<()> {
    if parameters.is_empty() {
        return Err(BenchmarkError::InvalidStatisticalModel {
            model: model.to_owned(),
            reason: "fit contains no parameters".to_owned(),
        });
    }

    for (index, &parameter) in parameters.iter().enumerate() {
        validate_finite(
            &format!("{}.parameters[{}]", model, index),
            parameter,
        )?;
    }

    if residuals.is_empty() {
        return Err(BenchmarkError::InvalidStatisticalModel {
            model: model.to_owned(),
            reason: "fit contains no residuals".to_owned(),
        });
    }

    for (index, &residual) in residuals.iter().enumerate() {
        validate_finite(
            &format!("{}.residuals[{}]", model, index),
            residual,
        )?;
    }

    if let Some(r_squared) = r_squared {
        validate_finite(
            &format!("{}.r_squared", model),
            r_squared,
        )?;
    }

    Ok(())
}

/// Validates a binomial observation represented by successes and samples.
pub fn validate_binomial_observation(
    successes: usize,
    samples: usize,
) -> BenchmarkResult<f64> {
    validate_positive_count(
        "samples",
        samples,
    )?;

    if successes > samples {
        return Err(BenchmarkError::InvalidCount {
            field: "successes".to_owned(),
            value: successes as u64,
            maximum: Some(samples as u64),
        });
    }

    Ok(successes as f64 / samples as f64)
}

/// Validates that all values in a sample vector are finite.
pub fn validate_samples(
    field: &str,
    samples: &[f64],
) -> BenchmarkResult<()> {
    if samples.is_empty() {
        return Err(BenchmarkError::InsufficientSamples {
            required: 1,
            actual: 0,
            context: field.to_owned(),
        });
    }

    for (index, &sample) in samples.iter().enumerate() {
        validate_finite(
            &format!("{}[{}]", field, index),
            sample,
        )?;
    }

    Ok(())
}

/// Validates that a sample vector is large enough for variance.
pub fn validate_variance_samples(
    field: &str,
    samples: &[f64],
) -> BenchmarkResult<()> {
    validate_samples(
        field,
        samples,
    )?;

    if samples.len() < MIN_VARIANCE_SAMPLES {
        return Err(BenchmarkError::InsufficientSamples {
            required: MIN_VARIANCE_SAMPLES,
            actual: samples.len(),
            context: field.to_owned(),
        });
    }

    Ok(())
}

/// Validates that an independent variable contains no duplicate values.
pub fn validate_unique_values(
    field: &str,
    values: &[f64],
) -> BenchmarkResult<()> {
    validate_samples(
        field,
        values,
    )?;

    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] == values[j] {
                return Err(BenchmarkError::ValidationFailed {
                    invariant: format!("{}.unique", field),
                    reason: format!(
                        "duplicate value {} at indices {} and {}",
                        values[i],
                        i,
                        j
                    ),
                });
            }
        }
    }

    Ok(())
}

/// Validates a positive finite denominator.
pub fn validate_denominator(
    field: &str,
    denominator: f64,
) -> BenchmarkResult<f64> {
    validate_positive_finite(
        field,
        denominator,
    )
}

/// Validates a denominator that may be negative but must not be zero.
pub fn validate_non_zero_denominator(
    field: &str,
    denominator: f64,
) -> BenchmarkResult<f64> {
    validate_finite(
        field,
        denominator,
    )?;

    if denominator == 0.0 {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.non_zero", field),
            reason: "denominator must not be zero".to_owned(),
        });
    }

    Ok(denominator)
}

/// Validates an effect-size denominator.
pub fn validate_effect_size_denominator(
    field: &str,
    denominator: f64,
) -> BenchmarkResult<f64> {
    validate_positive_finite(
        field,
        denominator,
    )
}

/// Validates a correlation coefficient.
pub fn validate_correlation(
    field: &str,
    correlation: f64,
) -> BenchmarkResult<f64> {
    validate_finite(
        field,
        correlation,
    )?;

    if correlation < -1.0 || correlation > 1.0 {
        return Err(BenchmarkError::ValidationFailed {
            invariant: format!("{}.unit_interval", field),
            reason: format!(
                "correlation {} is outside [-1,1]",
                correlation
            ),
        });
    }

    Ok(correlation)
}

/// Validates a covariance value.
pub fn validate_covariance(
    field: &str,
    covariance: f64,
) -> BenchmarkResult<f64> {
    validate_finite(
        field,
        covariance,
    )
}

/// Validates a chi-square statistic.
pub fn validate_chi_square(
    field: &str,
    statistic: f64,
) -> BenchmarkResult<f64> {
    validate_non_negative_finite(
        field,
        statistic,
    )
}

/// Validates a z/t-like statistic.
///
/// Such statistics may be negative, so only finiteness is required.
pub fn validate_test_statistic(
    field: &str,
    statistic: f64,
) -> BenchmarkResult<f64> {
    validate_finite(
        field,
        statistic,
    )
}

/// Validates an information criterion value such as AIC/BIC.
///
/// These values are not required to be positive.
pub fn validate_information_criterion(
    field: &str,
    value: f64,
) -> BenchmarkResult<f64> {
    validate_finite(
        field,
        value,
    )
}

// =============================================================================
// Internal helpers
// =============================================================================

fn invalid_configuration(
    field: &str,
    reason: &str,
) -> BenchmarkError {
    BenchmarkError::InvalidConfiguration {
        field: field.to_owned(),
        reason: reason.to_owned(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_valid() {
        let validator = StatisticalValidator::production();

        assert!(validator.validate_policy().is_ok());
    }

    #[test]
    fn rejects_nan() {
        let result = validate_finite(
            "value",
            f64::NAN,
        );

        assert!(result.is_err());

        match result {
            Err(BenchmarkError::NonFiniteValue { field, .. }) => {
                assert_eq!(field, "value");
            }
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn rejects_infinity() {
        assert!(
            validate_finite(
                "value",
                f64::INFINITY,
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_probability_boundaries() {
        assert!(
            validate_probability(
                "p",
                0.0,
            )
            .is_ok()
        );

        assert!(
            validate_probability(
                "p",
                1.0,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_probability() {
        assert!(
            validate_probability(
                "p",
                -0.1,
            )
            .is_err()
        );

        assert!(
            validate_probability(
                "p",
                1.1,
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_confidence_level() {
        assert!(
            validate_confidence_level(
                "confidence",
                0.95,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_zero_confidence() {
        assert!(
            validate_confidence_level(
                "confidence",
                0.0,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_one_confidence() {
        assert!(
            validate_confidence_level(
                "confidence",
                1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_binomial_observation() {
        let probability =
            validate_binomial_observation(
                25,
                100,
            )
            .expect("valid binomial observation");

        assert!(
            (probability - 0.25).abs() < 1.0e-12
        );
    }

    #[test]
    fn rejects_successes_above_samples() {
        assert!(
            validate_binomial_observation(
                101,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_count_distribution() {
        assert!(
            validate_count_distribution(
                &[25, 25, 50],
                100,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_inconsistent_count_distribution() {
        assert!(
            validate_count_distribution(
                &[25, 25, 49],
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_probability_distribution() {
        assert!(
            validate_probability_distribution(
                &[0.25, 0.25, 0.50],
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_probability_distribution_not_summing_to_one() {
        assert!(
            validate_probability_distribution(
                &[0.25, 0.25, 0.25],
            )
            .is_err()
        );
    }

    #[test]
    fn validates_paired_observations() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .paired_observations(
                    "x",
                    &[1.0, 2.0, 3.0],
                    "y",
                    &[2.0, 4.0, 6.0],
                )
                .is_ok()
        );
    }

    #[test]
    fn rejects_mismatched_pairs() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .paired_observations(
                    "x",
                    &[1.0, 2.0],
                    "y",
                    &[2.0],
                )
                .is_err()
        );
    }

    #[test]
    fn validates_strictly_increasing_values() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .strictly_increasing(
                    "depth",
                    &[1.0, 2.0, 4.0, 8.0],
                )
                .is_ok()
        );
    }

    #[test]
    fn rejects_non_increasing_values() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .strictly_increasing(
                    "depth",
                    &[1.0, 2.0, 2.0],
                )
                .is_err()
        );
    }

    #[test]
    fn validates_regression_inputs() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .regression_inputs(
                    "x",
                    &[1.0, 2.0, 3.0, 4.0],
                    "y",
                    &[2.0, 4.0, 6.0, 8.0],
                    2,
                )
                .is_ok()
        );
    }

    #[test]
    fn rejects_regression_with_too_many_parameters() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .regression_inputs(
                    "x",
                    &[1.0, 2.0],
                    "y",
                    &[2.0, 4.0],
                    2,
                )
                .is_err()
        );
    }

    #[test]
    fn validates_bootstrap_configuration() {
        let validator =
            StatisticalValidator::production();

        assert!(
            validator
                .bootstrap_configuration(
                    100,
                    1_000,
                )
                .is_ok()
        );
    }

    #[test]
    fn detects_integer_product_overflow() {
        let result =
            checked_product(
                usize::MAX,
                2,
                "overflow",
            );

        assert!(result.is_err());

        match result {
            Err(BenchmarkError::NumericalOverflow { .. }) => {}
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn validates_correlation() {
        assert!(
            validate_correlation(
                "correlation",
                -1.0,
            )
            .is_ok()
        );

        assert!(
            validate_correlation(
                "correlation",
                1.0,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_correlation() {
        assert!(
            validate_correlation(
                "correlation",
                1.01,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_confidence_bounds() {
        assert!(
            validate_confidence_bounds(
                "interval",
                0.2,
                0.8,
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_reversed_confidence_bounds() {
        assert!(
            validate_confidence_bounds(
                "interval",
                0.8,
                0.2,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_positive_standard_error() {
        assert!(
            validate_standard_error(
                "stderr",
                0.01,
            )
            .is_ok()
        );

        assert!(
            validate_standard_error(
                "stderr",
                0.0,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_non_negative_variance() {
        assert!(
            validate_variance(
                "variance",
                0.0,
            )
            .is_ok()
        );

        assert!(
            validate_variance(
                "variance",
                -0.01,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_significance_level() {
        assert!(
            StatisticalValidator::production()
                .significance_level(
                    "alpha",
                    0.05,
                )
                .is_ok()
        );

        assert!(
            StatisticalValidator::production()
                .significance_level(
                    "alpha",
                    0.0,
                )
                .is_err()
        );

        assert!(
            StatisticalValidator::production()
                .significance_level(
                    "alpha",
                    1.0,
                )
                .is_err()
        );
    }

    #[test]
    fn detects_zero_denominator() {
        assert!(
            validate_non_zero_denominator(
                "denominator",
                0.0,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_effective_sample_size() {
        assert!(
            StatisticalValidator::production()
                .effective_sample_size(
                    "ess",
                    100.0,
                )
                .is_ok()
        );
    }
}