//! Robust outlier detection for Zamani quantum benchmarking.
//!
//! # Purpose
//!
//! This module provides deterministic, auditable, non-destructive outlier
//! detection for benchmark observations.
//!
//! It intentionally does **not** delete observations. Instead, it classifies
//! observations as:
//!
//! - accepted
//! - flagged
//! - invalid
//!
//! A caller may subsequently decide whether flagged observations should be
//! excluded from a particular statistical analysis.
//!
//! # Architectural role
//!
//! ```text
//! raw benchmark observations
//!          |
//!          v
//! statistics::outliers
//!          |
//!    +-----+------+
//!    |            |
//!    v            v
//! classification  diagnostics
//!    |
//!    v
//! aggregation / regression / analysis
//! ```
//!
//! The module is deliberately independent of individual quantum protocols.
//! It can therefore be used by:
//!
//! - Quantum Volume
//! - randomized benchmarking
//! - interleaved RB
//! - simultaneous RB
//! - purity RB
//! - leakage benchmarking
//! - cycle benchmarking
//! - XEB
//! - random circuit sampling
//! - coherence measurements
//! - crosstalk experiments
//! - drift measurements
//! - application benchmarks
//! - QEC benchmarks
//! - timing/throughput measurements
//! - hardware calibration analysis
//!
//! # Important statistical policy
//!
//! Outlier detection is a diagnostic operation, not proof that a physical
//! observation is wrong.
//!
//! Quantum hardware can legitimately produce extreme observations because of:
//!
//! - calibration drift
//! - transient environmental effects
//! - thermal fluctuations
//! - correlated noise
//! - cosmic/background events
//! - control instability
//! - queue effects
//! - temporary backend degradation
//! - rare but physically meaningful error events
//!
//! Consequently this module never silently removes data.
//!
//! # Supported methods
//!
//! [`OutlierMethod::Iqr`] uses the interquartile range.
//!
//! [`OutlierMethod::Mad`] uses the median absolute deviation.
//!
//! [`OutlierMethod::ModifiedZScore`] uses the robust modified Z-score.
//!
//! [`OutlierMethod::IqrAndMad`] requires both methods to agree before an
//! observation is flagged. This is useful when conservative filtering is
//! required.
//!
//! [`OutlierMethod::EitherIqrOrMad`] flags an observation when either robust
//! detector identifies it.
//!
//! # Numerical policy
//!
//! Non-finite values are never treated as ordinary observations.
//!
//! `NaN`, positive infinity, and negative infinity are classified as invalid.
//!
//! The implementation also avoids divisions by zero when the data have zero
//! dispersion.
//!
//! # Reproducibility
//!
//! The algorithms are deterministic. No random number generator is used.
//!
//! For a fixed input sequence and configuration, the classification and
//! diagnostics are deterministic.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//! Edition: Rust 2021.
//!
//! No nightly features are required.

use std::fmt;

/// Default lower quartile multiplier used by the IQR detector.
pub const DEFAULT_IQR_MULTIPLIER: f64 = 1.5;

/// Default modified-Z threshold.
///
/// The conventional robust threshold is approximately 3.5.
pub const DEFAULT_MODIFIED_Z_THRESHOLD: f64 = 3.5;

/// Default MAD scaling constant.
///
/// `0.6744897501960817` converts MAD into the scale used by the normal
/// distribution under the standard robust modified-Z-score definition.
pub const MAD_NORMALIZATION: f64 = 0.674_489_750_196_081_7;

/// Minimum number of finite observations required for meaningful quartile/MAD
/// analysis.
pub const MIN_ROBUST_SAMPLE_SIZE: usize = 3;

/// Supported robust outlier-detection methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlierMethod {
    /// Tukey's interquartile-range rule.
    Iqr,

    /// Median absolute deviation.
    Mad,

    /// Robust modified Z-score derived from MAD.
    ModifiedZScore,

    /// An observation is flagged only when both IQR and MAD identify it.
    IqrAndMad,

    /// An observation is flagged when either IQR or MAD identifies it.
    EitherIqrOrMad,
}

impl Default for OutlierMethod {
    fn default() -> Self {
        Self::IqrAndMad
    }
}

impl fmt::Display for OutlierMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iqr => formatter.write_str("iqr"),
            Self::Mad => formatter.write_str("mad"),
            Self::ModifiedZScore => formatter.write_str("modified_z_score"),
            Self::IqrAndMad => formatter.write_str("iqr_and_mad"),
            Self::EitherIqrOrMad => formatter.write_str("either_iqr_or_mad"),
        }
    }
}

/// How non-finite observations are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonFinitePolicy {
    /// Mark non-finite observations invalid.
    Reject,

    /// Ignore non-finite observations when computing robust statistics, while
    /// retaining them in the result as invalid observations.
    IgnoreForStatistics,
}

impl Default for NonFinitePolicy {
    fn default() -> Self {
        Self::Reject
    }
}

/// Configuration for robust outlier detection.
///
/// The configuration contains no protocol-specific assumptions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlierConfig {
    /// Detection method.
    pub method: OutlierMethod,

    /// IQR multiplier.
    ///
    /// `1.5` is Tukey's conventional rule.
    pub iqr_multiplier: f64,

    /// Modified-Z threshold.
    ///
    /// `3.5` is the conventional robust threshold.
    pub modified_z_threshold: f64,

    /// Policy for non-finite input.
    pub non_finite_policy: NonFinitePolicy,

    /// Minimum number of finite observations required before robust
    /// statistics are considered meaningful.
    pub minimum_sample_size: usize,
}

impl Default for OutlierConfig {
    fn default() -> Self {
        Self {
            method: OutlierMethod::default(),
            iqr_multiplier: DEFAULT_IQR_MULTIPLIER,
            modified_z_threshold: DEFAULT_MODIFIED_Z_THRESHOLD,
            non_finite_policy: NonFinitePolicy::default(),
            minimum_sample_size: MIN_ROBUST_SAMPLE_SIZE,
        }
    }
}

impl OutlierConfig {
    /// Creates the standard production configuration.
    pub const fn production() -> Self {
        Self {
            method: OutlierMethod::IqrAndMad,
            iqr_multiplier: DEFAULT_IQR_MULTIPLIER,
            modified_z_threshold: DEFAULT_MODIFIED_Z_THRESHOLD,
            non_finite_policy: NonFinitePolicy::Reject,
            minimum_sample_size: MIN_ROBUST_SAMPLE_SIZE,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), OutlierError> {
        if !self.iqr_multiplier.is_finite() || self.iqr_multiplier < 0.0 {
            return Err(OutlierError::InvalidConfiguration {
                field: "iqr_multiplier",
                reason: "must be finite and non-negative",
            });
        }

        if !self.modified_z_threshold.is_finite()
            || self.modified_z_threshold <= 0.0
        {
            return Err(OutlierError::InvalidConfiguration {
                field: "modified_z_threshold",
                reason: "must be finite and greater than zero",
            });
        }

        if self.minimum_sample_size < 2 {
            return Err(OutlierError::InvalidConfiguration {
                field: "minimum_sample_size",
                reason: "must be at least 2",
            });
        }

        Ok(())
    }
}

/// Error returned by outlier analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum OutlierError {
    /// Configuration is invalid.
    InvalidConfiguration {
        /// Configuration field.
        field: &'static str,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// The supplied input contains no observations.
    EmptyInput,

    /// The input is too small for the requested robust analysis.
    InsufficientSamples {
        /// Number of finite observations.
        available: usize,

        /// Required number.
        required: usize,
    },

    /// A calculated statistic became non-finite.
    NumericalFailure {
        /// Statistic that failed.
        statistic: &'static str,
    },

    /// A requested operation would exceed a safety limit.
    ResourceLimitExceeded {
        /// Resource being limited.
        resource: &'static str,

        /// Requested amount.
        requested: usize,

        /// Maximum permitted amount.
        maximum: usize,
    },
}

impl fmt::Display for OutlierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, reason } => {
                write!(formatter, "invalid outlier configuration '{field}': {reason}")
            }

            Self::EmptyInput => {
                formatter.write_str("outlier analysis requires at least one observation")
            }

            Self::InsufficientSamples {
                available,
                required,
            } => {
                write!(
                    formatter,
                    "insufficient finite observations for outlier analysis: \
                     available {available}, required {required}"
                )
            }

            Self::NumericalFailure { statistic } => {
                write!(
                    formatter,
                    "non-finite numerical result while calculating '{statistic}'"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "outlier analysis resource '{resource}' exceeds limit: \
                     requested {requested}, maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for OutlierError {}

/// Classification assigned to one observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationClassification {
    /// Observation is finite and was not identified as an outlier.
    Accepted,

    /// Observation is finite but was identified as an outlier.
    Flagged,

    /// Observation is non-finite and therefore cannot participate in robust
    /// numerical analysis.
    Invalid,
}

impl ObservationClassification {
    /// Returns `true` when this observation should be considered valid
    /// numerical data.
    pub const fn is_valid(self) -> bool {
        match self {
            Self::Accepted | Self::Flagged => true,
            Self::Invalid => false,
        }
    }

    /// Returns `true` when the observation was flagged as an outlier.
    pub const fn is_flagged(self) -> bool {
        matches!(self, Self::Flagged)
    }
}

/// Diagnostic information produced for an individual observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlierObservation {
    /// Original zero-based input index.
    pub index: usize,

    /// Original observation value.
    pub value: f64,

    /// Classification.
    pub classification: ObservationClassification,

    /// Whether the IQR rule flagged this observation.
    pub iqr_flagged: bool,

    /// Whether the MAD rule flagged this observation.
    pub mad_flagged: bool,

    /// Robust modified Z-score, when computable.
    pub modified_z_score: Option<f64>,
}

impl OutlierObservation {
    /// Returns `true` if the observation is valid and not flagged.
    pub const fn is_accepted(self) -> bool {
        matches!(self.classification, ObservationClassification::Accepted)
    }

    /// Returns `true` if the observation is flagged.
    pub const fn is_flagged(self) -> bool {
        matches!(self.classification, ObservationClassification::Flagged)
    }

    /// Returns `true` if the observation is invalid.
    pub const fn is_invalid(self) -> bool {
        matches!(self.classification, ObservationClassification::Invalid)
    }
}

/// Robust statistics calculated from the finite observations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RobustStatistics {
    /// Number of original observations.
    pub total_count: usize,

    /// Number of finite observations.
    pub finite_count: usize,

    /// Number of non-finite observations.
    pub invalid_count: usize,

    /// Minimum finite value.
    pub minimum: f64,

    /// First quartile.
    pub q1: f64,

    /// Median.
    pub median: f64,

    /// Third quartile.
    pub q3: f64,

    /// Interquartile range.
    pub iqr: f64,

    /// Median absolute deviation.
    pub mad: f64,

    /// Lower IQR fence.
    pub lower_iqr_fence: f64,

    /// Upper IQR fence.
    pub upper_iqr_fence: f64,

    /// Number of finite observations flagged by IQR.
    pub iqr_outlier_count: usize,

    /// Number of finite observations flagged by MAD.
    pub mad_outlier_count: usize,

    /// Number of observations flagged by the selected method.
    pub selected_outlier_count: usize,
}

/// Complete result of an outlier analysis.
///
/// The original observations are never modified.
#[derive(Debug, Clone, PartialEq)]
pub struct OutlierAnalysis {
    /// Method used.
    pub method: OutlierMethod,

    /// Configuration used.
    pub config: OutlierConfig,

    /// Robust statistics.
    pub statistics: RobustStatistics,

    /// Per-observation classifications.
    pub observations: Vec<OutlierObservation>,
}

impl OutlierAnalysis {
    /// Number of observations classified as accepted.
    pub fn accepted_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| observation.is_accepted())
            .count()
    }

    /// Number of observations classified as flagged.
    pub fn flagged_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| observation.is_flagged())
            .count()
    }

    /// Number of observations classified as invalid.
    pub fn invalid_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| observation.is_invalid())
            .count()
    }

    /// Returns the original indices of flagged observations.
    pub fn flagged_indices(&self) -> Vec<usize> {
        self.observations
            .iter()
            .filter(|observation| observation.is_flagged())
            .map(|observation| observation.index)
            .collect()
    }

    /// Returns the original indices of invalid observations.
    pub fn invalid_indices(&self) -> Vec<usize> {
        self.observations
            .iter()
            .filter(|observation| observation.is_invalid())
            .map(|observation| observation.index)
            .collect()
    }

    /// Returns the original input values which were flagged.
    pub fn flagged_values(&self) -> Vec<f64> {
        self.observations
            .iter()
            .filter(|observation| observation.is_flagged())
            .map(|observation| observation.value)
            .collect()
    }

    /// Returns finite observations that were not flagged.
    ///
    /// This method creates a new vector and therefore should only be used when
    /// the caller explicitly wants a filtered data set. The canonical result
    /// remains non-destructive.
    pub fn accepted_values(&self) -> Vec<f64> {
        self.observations
            .iter()
            .filter(|observation| observation.is_accepted())
            .map(|observation| observation.value)
            .collect()
    }

    /// Returns the fraction of finite observations flagged as outliers.
    pub fn flagged_fraction(&self) -> Option<f64> {
        if self.statistics.finite_count == 0 {
            return None;
        }

        Some(
            self.statistics.selected_outlier_count as f64
                / self.statistics.finite_count as f64,
        )
    }
}

/// Performs production-default robust outlier analysis.
///
/// This is equivalent to:
///
/// [`OutlierDetector::with_config(OutlierConfig::production())`].
pub fn detect_outliers(values: &[f64]) -> Result<OutlierAnalysis, OutlierError> {
    OutlierDetector::with_config(OutlierConfig::production()).analyze(values)
}

/// Performs outlier analysis using an explicit configuration.
pub fn detect_outliers_with_config(
    values: &[f64],
    config: OutlierConfig,
) -> Result<OutlierAnalysis, OutlierError> {
    OutlierDetector::with_config(config).analyze(values)
}

/// Stateless production outlier detector.
///
/// The detector contains no mutable global state and is safe to reuse across
/// independent benchmark experiments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlierDetector {
    config: OutlierConfig,
}

impl Default for OutlierDetector {
    fn default() -> Self {
        Self::with_config(OutlierConfig::production())
    }
}

impl OutlierDetector {
    /// Creates a detector from explicit configuration.
    pub fn with_config(config: OutlierConfig) -> Self {
        Self { config }
    }

    /// Returns the detector configuration.
    pub const fn config(&self) -> OutlierConfig {
        self.config
    }

    /// Validates the detector configuration.
    pub fn validate(&self) -> Result<(), OutlierError> {
        self.config.validate()
    }

    /// Analyzes observations without modifying the input.
    pub fn analyze(
        &self,
        values: &[f64],
    ) -> Result<OutlierAnalysis, OutlierError> {
        self.validate()?;

        if values.is_empty() {
            return Err(OutlierError::EmptyInput);
        }

        let finite_values = collect_finite(values);

        if finite_values.len() < self.config.minimum_sample_size {
            return Err(OutlierError::InsufficientSamples {
                available: finite_values.len(),
                required: self.config.minimum_sample_size,
            });
        }

        let sorted = sorted_copy(&finite_values);

        let q1 = percentile_linear(&sorted, 0.25)?;
        let median = percentile_linear(&sorted, 0.50)?;
        let q3 = percentile_linear(&sorted, 0.75)?;

        let iqr = q3 - q1;

        if !iqr.is_finite() {
            return Err(OutlierError::NumericalFailure {
                statistic: "iqr",
            });
        }

        let deviations = absolute_deviations(&finite_values, median);
        let sorted_deviations = sorted_copy(&deviations);
        let mad = percentile_linear(&sorted_deviations, 0.50)?;

        if !mad.is_finite() {
            return Err(OutlierError::NumericalFailure {
                statistic: "mad",
            });
        }

        let lower_iqr_fence =
            q1 - self.config.iqr_multiplier * iqr;

        let upper_iqr_fence =
            q3 + self.config.iqr_multiplier * iqr;

        if !lower_iqr_fence.is_finite()
            || !upper_iqr_fence.is_finite()
        {
            return Err(OutlierError::NumericalFailure {
                statistic: "iqr_fence",
            });
        }

        let mut observations = Vec::with_capacity(values.len());

        let mut iqr_outlier_count = 0usize;
        let mut mad_outlier_count = 0usize;
        let mut selected_outlier_count = 0usize;

        for (index, &value) in values.iter().enumerate() {
            if !value.is_finite() {
                observations.push(OutlierObservation {
                    index,
                    value,
                    classification: ObservationClassification::Invalid,
                    iqr_flagged: false,
                    mad_flagged: false,
                    modified_z_score: None,
                });

                continue;
            }

            let iqr_flagged =
                is_iqr_outlier(value, lower_iqr_fence, upper_iqr_fence);

            let modified_z_score =
                modified_z_score(value, median, mad);

            let mad_flagged = match modified_z_score {
                Some(score) => score.abs()
                    > self.config.modified_z_threshold,
                None => false,
            };

            if iqr_flagged {
                iqr_outlier_count =
                    iqr_outlier_count.saturating_add(1);
            }

            if mad_flagged {
                mad_outlier_count =
                    mad_outlier_count.saturating_add(1);
            }

            let selected = match self.config.method {
                OutlierMethod::Iqr => iqr_flagged,

                OutlierMethod::Mad
                | OutlierMethod::ModifiedZScore => mad_flagged,

                OutlierMethod::IqrAndMad => {
                    iqr_flagged && mad_flagged
                }

                OutlierMethod::EitherIqrOrMad => {
                    iqr_flagged || mad_flagged
                }
            };

            if selected {
                selected_outlier_count =
                    selected_outlier_count.saturating_add(1);
            }

            observations.push(OutlierObservation {
                index,
                value,
                classification: if selected {
                    ObservationClassification::Flagged
                } else {
                    ObservationClassification::Accepted
                },
                iqr_flagged,
                mad_flagged,
                modified_z_score,
            });
        }

        let minimum = sorted
            .first()
            .copied()
            .ok_or(OutlierError::NumericalFailure {
                statistic: "minimum",
            })?;

        Ok(OutlierAnalysis {
            method: self.config.method,
            config: self.config,
            statistics: RobustStatistics {
                total_count: values.len(),
                finite_count: finite_values.len(),
                invalid_count: values.len() - finite_values.len(),
                minimum,
                q1,
                median,
                q3,
                iqr,
                mad,
                lower_iqr_fence,
                upper_iqr_fence,
                iqr_outlier_count,
                mad_outlier_count,
                selected_outlier_count,
            },
            observations,
        })
    }
}

/// Calculates the median of finite observations.
///
/// This is exposed because downstream benchmark diagnostics frequently need a
/// robust center without running the complete classification pipeline.
pub fn median(values: &[f64]) -> Result<f64, OutlierError> {
    let finite = collect_finite(values);

    if finite.is_empty() {
        return Err(OutlierError::EmptyInput);
    }

    let sorted = sorted_copy(&finite);

    percentile_linear(&sorted, 0.5)
}

/// Calculates the first quartile.
pub fn first_quartile(values: &[f64]) -> Result<f64, OutlierError> {
    let finite = collect_finite(values);

    if finite.is_empty() {
        return Err(OutlierError::EmptyInput);
    }

    percentile_linear(&sorted_copy(&finite), 0.25)
}

/// Calculates the third quartile.
pub fn third_quartile(values: &[f64]) -> Result<f64, OutlierError> {
    let finite = collect_finite(values);

    if finite.is_empty() {
        return Err(OutlierError::EmptyInput);
    }

    percentile_linear(&sorted_copy(&finite), 0.75)
}

/// Calculates the interquartile range.
pub fn interquartile_range(
    values: &[f64],
) -> Result<f64, OutlierError> {
    let q1 = first_quartile(values)?;
    let q3 = third_quartile(values)?;
    let result = q3 - q1;

    if !result.is_finite() {
        return Err(OutlierError::NumericalFailure {
            statistic: "iqr",
        });
    }

    Ok(result)
}

/// Calculates the median absolute deviation.
///
/// Non-finite observations are ignored for this calculation.
pub fn median_absolute_deviation(
    values: &[f64],
) -> Result<f64, OutlierError> {
    let finite = collect_finite(values);

    if finite.is_empty() {
        return Err(OutlierError::EmptyInput);
    }

    let center = median(&finite)?;
    let deviations = absolute_deviations(&finite, center);

    median(&deviations)
}

/// Calculates the robust modified Z-score.
///
/// Returns `None` when MAD is zero because a conventional modified Z-score
/// cannot distinguish deviations when all observations have zero robust
/// dispersion.
///
/// When MAD is zero, the caller should generally use the IQR rule or an exact
/// equality check around the median rather than treating the result as an
/// infinite Z-score.
pub fn modified_z_score(
    value: f64,
    median_value: f64,
    mad: f64,
) -> Option<f64> {
    if !value.is_finite()
        || !median_value.is_finite()
        || !mad.is_finite()
        || mad <= 0.0
    {
        return None;
    }

    let score =
        MAD_NORMALIZATION * (value - median_value) / mad;

    if score.is_finite() {
        Some(score)
    } else {
        None
    }
}

/// Determines whether an observation violates the IQR fences.
pub fn is_iqr_outlier(
    value: f64,
    lower_fence: f64,
    upper_fence: f64,
) -> bool {
    value.is_finite()
        && lower_fence.is_finite()
        && upper_fence.is_finite()
        && (value < lower_fence || value > upper_fence)
}

/// Returns finite values from the input.
///
/// This function preserves input order.
fn collect_finite(values: &[f64]) -> Vec<f64> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect()
}

/// Creates a sorted copy without modifying the caller's data.
///
/// `f64::total_cmp` provides deterministic ordering, including a defined
/// ordering for values that are otherwise problematic under partial
/// comparison. Non-finite values are already removed before this function is
/// normally called.
fn sorted_copy(values: &[f64]) -> Vec<f64> {
    let mut result = values.to_vec();

    result.sort_by(|left, right| left.total_cmp(right));

    result
}

/// Calculates a linearly interpolated percentile.
///
/// The input must be sorted and non-empty.
///
/// This implementation uses the commonly used linear interpolation rule:
///
/// `position = p * (n - 1)`
///
/// and interpolates between the surrounding observations.
fn percentile_linear(
    sorted: &[f64],
    probability: f64,
) -> Result<f64, OutlierError> {
    if sorted.is_empty() {
        return Err(OutlierError::EmptyInput);
    }

    if !probability.is_finite()
        || !(0.0..=1.0).contains(&probability)
    {
        return Err(OutlierError::InvalidConfiguration {
            field: "probability",
            reason: "must be finite and between 0 and 1",
        });
    }

    if sorted.len() == 1 {
        return Ok(sorted[0]);
    }

    let position =
        probability * (sorted.len() - 1) as f64;

    let lower_index = position.floor() as usize;
    let upper_index = position.ceil() as usize;

    let lower = sorted[lower_index];
    let upper = sorted[upper_index];

    if lower_index == upper_index {
        return Ok(lower);
    }

    let fraction = position - lower_index as f64;

    let result =
        lower + (upper - lower) * fraction;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(OutlierError::NumericalFailure {
            statistic: "percentile",
        })
    }
}

/// Calculates absolute deviations from a center.
fn absolute_deviations(
    values: &[f64],
    center: f64,
) -> Vec<f64> {
    values
        .iter()
        .map(|value| (value - center).abs())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_configuration_is_valid() {
        assert!(OutlierConfig::production().validate().is_ok());
    }

    #[test]
    fn empty_input_is_rejected() {
        let result = detect_outliers(&[]);

        assert_eq!(result, Err(OutlierError::EmptyInput));
    }

    #[test]
    fn insufficient_sample_is_rejected() {
        let result = detect_outliers(&[1.0, 2.0]);

        assert_eq!(
            result,
            Err(OutlierError::InsufficientSamples {
                available: 2,
                required: MIN_ROBUST_SAMPLE_SIZE,
            })
        );
    }

    #[test]
    fn median_for_odd_sample_is_correct() {
        let values = [5.0, 1.0, 3.0];

        let result = median(&values).unwrap();

        assert_eq!(result, 3.0);
    }

    #[test]
    fn median_for_even_sample_is_correct() {
        let values = [4.0, 1.0, 3.0, 2.0];

        let result = median(&values).unwrap();

        assert_eq!(result, 2.5);
    }

    #[test]
    fn quartiles_are_correct() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(first_quartile(&values).unwrap(), 2.0);
        assert_eq!(third_quartile(&values).unwrap(), 4.0);
    }

    #[test]
    fn iqr_is_correct() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(
            interquartile_range(&values).unwrap(),
            2.0
        );
    }

    #[test]
    fn mad_is_correct_for_symmetric_data() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];

        let result =
            median_absolute_deviation(&values).unwrap();

        assert_eq!(result, 1.0);
    }

    #[test]
    fn obvious_iqr_outlier_is_flagged() {
        let values = [
            10.0,
            11.0,
            10.5,
            9.8,
            10.2,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::Iqr,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_count(), 1);
        assert_eq!(result.flagged_indices(), vec![5]);
    }

    #[test]
    fn either_method_flags_if_one_detector_agrees() {
        let values = [
            10.0,
            11.0,
            10.5,
            9.8,
            10.2,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::EitherIqrOrMad,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_count(), 1);
        assert_eq!(result.flagged_indices(), vec![5]);
    }

    #[test]
    fn non_finite_values_are_invalid() {
        let values = [
            1.0,
            2.0,
            3.0,
            4.0,
            f64::NAN,
            f64::INFINITY,
        ];

        let result = detect_outliers(&values).unwrap();

        assert_eq!(result.invalid_count(), 2);
        assert_eq!(result.invalid_indices(), vec![4, 5]);
    }

    #[test]
    fn non_finite_values_do_not_enter_statistics() {
        let values = [
            1.0,
            2.0,
            3.0,
            f64::NAN,
            f64::INFINITY,
        ];

        let result = detect_outliers(&values).unwrap();

        assert_eq!(result.statistics.finite_count, 3);
        assert_eq!(result.statistics.invalid_count, 2);
        assert_eq!(result.statistics.median, 2.0);
    }

    #[test]
    fn input_is_not_modified() {
        let values = [
            100.0,
            1.0,
            2.0,
            3.0,
            4.0,
        ];

        let original = values;

        let _ = detect_outliers(&values).unwrap();

        assert_eq!(values, original);
    }

    #[test]
    fn modified_z_score_is_deterministic() {
        let score =
            modified_z_score(10.0, 5.0, 1.0).unwrap();

        let expected =
            MAD_NORMALIZATION * 5.0;

        assert!((score - expected).abs() < 1e-12);
    }

    #[test]
    fn modified_z_score_is_none_for_zero_mad() {
        assert_eq!(
            modified_z_score(10.0, 5.0, 0.0),
            None
        );
    }

    #[test]
    fn iqr_and_mad_require_both_detectors() {
        let values = [
            10.0,
            10.1,
            9.9,
            10.2,
            9.8,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::IqrAndMad,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_count(), 1);
    }

    #[test]
    fn modified_z_method_works() {
        let values = [
            10.0,
            10.1,
            9.9,
            10.2,
            9.8,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::ModifiedZScore,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_indices(), vec![5]);
    }

    #[test]
    fn invalid_iqr_multiplier_is_rejected() {
        let config = OutlierConfig {
            iqr_multiplier: -1.0,
            ..OutlierConfig::production()
        };

        assert!(matches!(
            config.validate(),
            Err(OutlierError::InvalidConfiguration {
                field: "iqr_multiplier",
                ..
            })
        ));
    }

    #[test]
    fn invalid_z_threshold_is_rejected() {
        let config = OutlierConfig {
            modified_z_threshold: 0.0,
            ..OutlierConfig::production()
        };

        assert!(matches!(
            config.validate(),
            Err(OutlierError::InvalidConfiguration {
                field: "modified_z_threshold",
                ..
            })
        ));
    }

    #[test]
    fn invalid_minimum_sample_size_is_rejected() {
        let config = OutlierConfig {
            minimum_sample_size: 1,
            ..OutlierConfig::production()
        };

        assert!(matches!(
            config.validate(),
            Err(OutlierError::InvalidConfiguration {
                field: "minimum_sample_size",
                ..
            })
        ));
    }

    #[test]
    fn all_identical_values_are_not_outliers() {
        let values = [
            5.0,
            5.0,
            5.0,
            5.0,
            5.0,
        ];

        let result = detect_outliers(&values).unwrap();

        assert_eq!(result.flagged_count(), 0);
        assert_eq!(result.statistics.iqr, 0.0);
        assert_eq!(result.statistics.mad, 0.0);
    }

    #[test]
    fn flagged_values_preserve_original_order() {
        let values = [
            100.0,
            10.0,
            11.0,
            10.5,
            9.8,
            10.2,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::Iqr,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_values(), vec![100.0]);
    }

    #[test]
    fn analysis_is_reproducible() {
        let values = [
            10.0,
            11.0,
            10.5,
            9.8,
            10.2,
            100.0,
        ];

        let first = detect_outliers(&values).unwrap();
        let second = detect_outliers(&values).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn flagged_fraction_is_correct() {
        let values = [
            10.0,
            11.0,
            10.5,
            9.8,
            10.2,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::Iqr,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        assert_eq!(result.flagged_fraction(), Some(1.0 / 6.0));
    }

    #[test]
    fn classifications_match_detector_flags() {
        let values = [
            10.0,
            10.1,
            9.9,
            10.2,
            9.8,
            100.0,
        ];

        let config = OutlierConfig {
            method: OutlierMethod::Iqr,
            ..OutlierConfig::production()
        };

        let result =
            detect_outliers_with_config(&values, config)
                .unwrap();

        for observation in &result.observations {
            if observation.iqr_flagged {
                assert_eq!(
                    observation.classification,
                    ObservationClassification::Flagged
                );
            }
        }
    }

    #[test]
    fn display_method_names_are_stable() {
        assert_eq!(
            OutlierMethod::Iqr.to_string(),
            "iqr"
        );

        assert_eq!(
            OutlierMethod::Mad.to_string(),
            "mad"
        );

        assert_eq!(
            OutlierMethod::ModifiedZScore.to_string(),
            "modified_z_score"
        );

        assert_eq!(
            OutlierMethod::IqrAndMad.to_string(),
            "iqr_and_mad"
        );

        assert_eq!(
            OutlierMethod::EitherIqrOrMad.to_string(),
            "either_iqr_or_mad"
        );
    }

    #[test]
    fn lower_and_upper_fences_are_reported() {
        let values = [
            1.0,
            2.0,
            3.0,
            4.0,
            5.0,
        ];

        let result = detect_outliers(&values).unwrap();

        assert_eq!(
            result.statistics.lower_iqr_fence,
            -1.0
        );

        assert_eq!(
            result.statistics.upper_iqr_fence,
            7.0
        );
    }
}