//! Zamani Quantum Benchmarking — Leakage Metrics
//!
//! Production implementation of leakage-related quantum benchmarking metrics.
//!
//! # Responsibility
//!
//! This module owns the mathematical representation and calculation of
//! leakage metrics from already-observed or already-derived experimental data.
//!
//! It does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - control hardware;
//! - perform calibration discovery;
//! - inject physical noise;
//! - implement a noise model;
//! - implement leakage-reduction/control pulses;
//! - perform randomized-benchmarking sequence generation;
//! - perform exponential regression/fitting;
//! - own benchmark result envelopes;
//! - perform backend capability negotiation;
//! - persist telemetry.
//!
//! Those responsibilities belong to the corresponding benchmarking, quantum
//! hardware, runtime, error-correction, execution, and statistics modules.
//!
//! # Architectural position
//!
//! ```text
//! quantum backend / simulator / QEC execution
//!                  │
//!                  ▼
//!          raw observations
//!                  │
//!                  ▼
//!        protocol-specific analysis
//!                  │
//!                  ▼
//!              leakage.rs
//!                  │
//!        ┌─────────┼─────────┐
//!        ▼         ▼         ▼
//!   leakage     survival   recovery
//!      rate      rate       rate
//!        │         │         │
//!        └─────────┼─────────┘
//!                  ▼
//!          core::metric::Metric
//!                  │
//!                  ▼
//!       core::result / reporting
//!                  │
//!                  ▼
//!          Zamani language API
//! ```
//!
//! # Leakage semantics
//!
//! Leakage is population leaving the computational subspace into states that
//! are not represented by the intended computational basis.
//!
//! For a direct binary leakage observation:
//!
//! ```text
//! leakage_rate = leaked_events / total_events
//! ```
//!
//! For a measured computational-subspace population:
//!
//! ```text
//! leakage_rate = 1 - computational_subspace_probability
//! ```
//!
//! These two forms are mathematically related but have different provenance.
//! The resulting metric records whether it was directly observed or derived.
//!
//! # Per-cycle leakage
//!
//! If a protocol measures a computational-subspace survival probability `S`
//! after `n` nominally identical cycles, the commonly used constant-independent
//! leakage model gives:
//!
//! ```text
//! S = (1 - l)^n
//!
//! l = 1 - S^(1/n)
//! ```
//!
//! This is a MODEL-BASED DERIVATION. It must never be presented as a directly
//! observed per-cycle leakage probability. The returned metric therefore
//! records the model assumption in metadata.
//!
//! # Recovery
//!
//! Given an initial leaked population and a later recovered population:
//!
//! ```text
//! recovery_rate = recovered / initially_leaked
//! ```
//!
//! The denominator is explicitly the initially leaked population. A recovery
//! measurement with zero initially leaked population is invalid rather than
//! silently producing zero or one.
//!
//! # Confidence intervals
//!
//! Leakage observed from binary events is a binomial proportion. This module
//! uses the Wilson score interval because it behaves better than the naive
//! normal approximation near zero and one and for finite sample sizes.
//!
//! The statistical engine remains responsible for more advanced procedures
//! such as bootstrap resampling, hierarchical models, or longitudinal fitting.
//!
//! # Important distinction
//!
//! A leakage rate of zero does NOT prove that a device has no leakage.
//!
//! It means that no leakage was observed in the supplied observation set.
//! The confidence interval can still provide a non-zero upper bound.
//!
//! # Production invariants
//!
//! 1. No NaN or infinity may enter this module.
//! 2. Probabilities must lie in [0, 1].
//! 3. Counts must be internally consistent.
//! 4. Zero-denominator recovery calculations are rejected.
//! 5. Zero-cycle derived leakage calculations are rejected.
//! 6. Invalid confidence levels are rejected.
//! 7. Confidence intervals must remain inside [0, 1].
//! 8. No scientific input is silently clamped.
//! 9. No diagnostic printing occurs.
//! 10. Public constructors are fallible.
//! 11. Derived quantities explicitly identify their mathematical model.
//! 12. The module does not claim that a measurement is leakage unless the
//!     caller identifies the computational-subspace boundary.
//! 13. Shot/sample counts are retained in the canonical Metric.
//! 14. The implementation is deterministic.
//! 15. The implementation uses no unsafe code.
//!
//! # Integration
//!
//! The module depends only on:
//!
//! - `core::metric`;
//! - Rust standard library.
//!
//! This intentionally makes it usable by:
//!
//! - randomized benchmarking;
//! - leakage randomized benchmarking;
//! - cycle benchmarking;
//! - coherence characterization;
//! - QEC physical-error analysis;
//! - hardware characterization;
//! - simulator validation;
//! - application benchmarking;
//! - reporting;
//! - regression analysis.
//!
//! The intended module declaration is:
//!
//! ```ignore
//! // src/quantum/benchmarking/metrics/mod.rs
//! pub mod leakage;
//! ```
//!
//! No changes to `core::metric` are required by this file.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are used.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::benchmarking::core::metric::{
    ConfidenceMethod,
    FiniteF64,
    Metric,
    MetricConfidence,
    MetricKind,
    MetricMetadata,
    MetricQuality,
    MetricResult,
    MetricUnit,
};

/// Zero as an explicit constant for readability in mathematical expressions.
const ZERO: f64 = 0.0;

/// One as an explicit constant for readability in mathematical expressions.
const ONE: f64 = 1.0;

/// Minimum valid sample/event count.
const MIN_COUNT: u64 = 1;

/// Minimum number of cycles for a per-cycle derivation.
const MIN_CYCLES: u64 = 1;

/// Default confidence level used by convenience constructors.
///
/// The caller should use an explicit level for publication-grade experiments.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Metadata key describing how leakage was obtained.
const METHOD_KEY: &str = "leakage_method";

/// Metadata key describing the computational-subspace definition.
const SUBSPACE_KEY: &str = "computational_subspace";

/// Metadata key describing the derivation model.
const MODEL_KEY: &str = "derivation_model";

/// Metadata key describing the model version.
const MODEL_VERSION_KEY: &str = "model_version";

/// Metadata key identifying the source observation.
const SOURCE_KEY: &str = "source";

/// Metadata key identifying whether the metric is observed or derived.
const OBSERVATION_TYPE_KEY: &str = "observation_type";

/// Metadata key identifying the number of cycles.
const CYCLES_KEY: &str = "cycles";

/// Metadata key identifying the initial population.
const INITIAL_POPULATION_KEY: &str = "initial_population";

/// Metadata key identifying the recovered population.
const RECOVERED_POPULATION_KEY: &str = "recovered_population";

/// Current version of the leakage mathematical conventions.
const MODEL_VERSION: &str = "1";

/// Errors produced by leakage metric calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum LeakageError {
    /// A supplied probability was NaN, infinity, or outside [0, 1].
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// A sample/event count was zero where a denominator is required.
    InvalidCount {
        field: &'static str,
        value: u64,
    },

    /// A numerator exceeds its denominator.
    NumeratorExceedsDenominator {
        numerator: u64,
        denominator: u64,
        field: &'static str,
    },

    /// A confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// A confidence interval is invalid.
    InvalidConfidenceInterval {
        lower: f64,
        upper: f64,
    },

    /// The cycle count is invalid.
    InvalidCycleCount {
        value: u64,
    },

    /// A population needed for recovery was zero.
    ZeroInitialLeakagePopulation,

    /// A derived numerical operation produced a non-finite value.
    NonFiniteResult {
        operation: &'static str,
        value: f64,
    },

    /// A textual identifier is empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// A textual identifier is too long.
    IdentifierTooLong {
        field: &'static str,
        maximum: usize,
    },

    /// The supplied computational-subspace description is invalid.
    InvalidSubspaceDescription,

    /// The metric representation rejected the constructed metric.
    MetricConstruction(String),
}

impl std::fmt::Display for LeakageError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "{field} must be finite and within [0, 1], got {value}"
                )
            }

            Self::InvalidCount { field, value } => {
                write!(
                    formatter,
                    "{field} must be greater than zero, got {value}"
                )
            }

            Self::NumeratorExceedsDenominator {
                numerator,
                denominator,
                field,
            } => {
                write!(
                    formatter,
                    "{field} numerator {numerator} exceeds denominator {denominator}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite and strictly between 0 and 1, got {value}"
                )
            }

            Self::InvalidConfidenceInterval { lower, upper } => {
                write!(
                    formatter,
                    "invalid confidence interval [{lower}, {upper}]"
                )
            }

            Self::InvalidCycleCount { value } => {
                write!(
                    formatter,
                    "cycle count must be greater than zero, got {value}"
                )
            }

            Self::ZeroInitialLeakagePopulation => {
                write!(
                    formatter,
                    "initial leaked population must be greater than zero"
                )
            }

            Self::NonFiniteResult { operation, value } => {
                write!(
                    formatter,
                    "{operation} produced a non-finite result: {value}"
                )
            }

            Self::EmptyIdentifier { field } => {
                write!(
                    formatter,
                    "{field} must not be empty"
                )
            }

            Self::IdentifierTooLong {
                field,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} exceeds maximum length of {maximum} bytes"
                )
            }

            Self::InvalidSubspaceDescription => {
                write!(
                    formatter,
                    "computational-subspace description must not be empty"
                )
            }

            Self::MetricConstruction(message) => {
                write!(
                    formatter,
                    "leakage metric construction failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for LeakageError {}

/// Result type for leakage operations.
pub type LeakageResult<T> = Result<T, LeakageError>;

/// Validated leakage observation.
///
/// This structure represents a binary classification:
///
/// - leaked;
/// - not leaked.
///
/// The caller is responsible for ensuring that the classification criterion
/// genuinely corresponds to population outside the intended computational
/// subspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeakageCounts {
    /// Number of observations classified as leaked.
    pub leaked: u64,

    /// Total number of observations.
    pub total: u64,
}

impl LeakageCounts {
    /// Creates a validated leakage-count observation.
    pub fn new(
        leaked: u64,
        total: u64,
    ) -> LeakageResult<Self> {
        if total < MIN_COUNT {
            return Err(LeakageError::InvalidCount {
                field: "total",
                value: total,
            });
        }

        if leaked > total {
            return Err(
                LeakageError::NumeratorExceedsDenominator {
                    numerator: leaked,
                    denominator: total,
                    field: "leaked",
                },
            );
        }

        Ok(Self { leaked, total })
    }

    /// Returns the empirical leakage probability.
    #[must_use]
    pub fn probability(self) -> f64 {
        self.leaked as f64 / self.total as f64
    }

    /// Returns the number of non-leaked observations.
    #[must_use]
    pub fn retained(self) -> u64 {
        self.total - self.leaked
    }

    /// Returns the empirical computational-subspace survival probability.
    #[must_use]
    pub fn survival_probability(self) -> f64 {
        self.retained() as f64 / self.total as f64
    }
}

/// Validated recovery observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeakageRecoveryCounts {
    /// Population known or estimated to be leaked at the beginning.
    pub initially_leaked: u64,

    /// Population that returned to the computational subspace.
    pub recovered: u64,
}

impl LeakageRecoveryCounts {
    /// Creates validated recovery counts.
    pub fn new(
        initially_leaked: u64,
        recovered: u64,
    ) -> LeakageResult<Self> {
        if initially_leaked < MIN_COUNT {
            return Err(
                LeakageError::ZeroInitialLeakagePopulation,
            );
        }

        if recovered > initially_leaked {
            return Err(
                LeakageError::NumeratorExceedsDenominator {
                    numerator: recovered,
                    denominator: initially_leaked,
                    field: "recovered",
                },
            );
        }

        Ok(Self {
            initially_leaked,
            recovered,
        })
    }

    /// Returns the recovery probability.
    #[must_use]
    pub fn probability(self) -> f64 {
        self.recovered as f64
            / self.initially_leaked as f64
    }
}

/// High-level classification of how a leakage metric was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeakageMethod {
    /// Leakage was counted directly from binary observations.
    DirectObservation,

    /// Leakage was calculated as one minus measured computational-subspace
    /// population.
    ComplementOfSubspacePopulation,

    /// Leakage was inferred from survival after repeated cycles.
    PerCycleFromSurvival,

    /// Recovery was measured relative to an initially leaked population.
    RecoveryFromLeakedPopulation,
}

impl LeakageMethod {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::DirectObservation => "direct_observation",
            Self::ComplementOfSubspacePopulation => {
                "complement_of_computational_subspace_population"
            }
            Self::PerCycleFromSurvival => {
                "per_cycle_from_survival"
            }
            Self::RecoveryFromLeakedPopulation => {
                "recovery_from_leaked_population"
            }
        }
    }
}

/// Standard bundle of leakage metrics.
///
/// This avoids forcing callers to reconstruct related quantities separately.
///
/// The individual `Metric` values remain authoritative.
#[derive(Debug, Clone, PartialEq)]
pub struct LeakageMetrics {
    /// Leakage probability/rate.
    pub leakage_rate: Metric,

    /// Computational-subspace survival probability.
    pub survival_probability: Metric,
}

impl LeakageMetrics {
    /// Creates direct leakage and survival metrics from counts.
    pub fn from_counts(
        counts: LeakageCounts,
    ) -> LeakageResult<Self> {
        Self::from_counts_with_confidence(
            counts,
            DEFAULT_CONFIDENCE_LEVEL,
        )
    }

    /// Creates direct leakage and survival metrics with an explicit
    /// confidence level.
    pub fn from_counts_with_confidence(
        counts: LeakageCounts,
        confidence_level: f64,
    ) -> LeakageResult<Self> {
        validate_confidence_level(confidence_level)?;

        let leakage = counts.probability();
        let survival = counts.survival_probability();

        let confidence = wilson_confidence(
            counts.leaked,
            counts.total,
            confidence_level,
        )?;

        let survival_confidence =
            complement_confidence(&confidence)?;

        let leakage_rate = build_metric(
            MetricKind::LeakageRate,
            leakage,
            MetricUnit::Probability,
            MetricQuality::Observed,
            Some(confidence),
            Some(counts.total),
            Some(counts.total),
            vec![
                metadata(
                    METHOD_KEY,
                    LeakageMethod::DirectObservation.id(),
                )?,
                metadata(
                    OBSERVATION_TYPE_KEY,
                    "binary_observation",
                )?,
                metadata(
                    MODEL_VERSION_KEY,
                    MODEL_VERSION,
                )?,
            ],
        )?;

        let survival_probability = build_metric(
            MetricKind::Probability,
            survival,
            MetricUnit::Probability,
            MetricQuality::Observed,
            Some(survival_confidence),
            Some(counts.total),
            Some(counts.total),
            vec![
                metadata(
                    METHOD_KEY,
                    "complement_of_direct_leakage",
                )?,
                metadata(
                    DERIVED_KEY,
                    "leakage_rate",
                )?,
                metadata(
                    MODEL_VERSION_KEY,
                    MODEL_VERSION,
                )?,
            ],
        )?;

        Ok(Self {
            leakage_rate,
            survival_probability,
        })
    }
}

/// Metadata key used for complement-derived quantities.
const DERIVED_KEY: &str = "derived_from";

/// Calculates a leakage metric directly from leaked and total events.
///
/// ```text
/// leakage_rate = leaked / total
/// ```
pub fn leakage_rate_from_counts(
    leaked: u64,
    total: u64,
) -> LeakageResult<Metric> {
    let counts = LeakageCounts::new(leaked, total)?;

    leakage_rate_from_counts_with_confidence(
        counts,
        DEFAULT_CONFIDENCE_LEVEL,
    )
}

/// Calculates a leakage metric directly from leaked and total events with an
/// explicit confidence level.
pub fn leakage_rate_from_counts_with_confidence(
    counts: LeakageCounts,
    confidence_level: f64,
) -> LeakageResult<Metric> {
    validate_confidence_level(confidence_level)?;

    let probability = counts.probability();

    let confidence = wilson_confidence(
        counts.leaked,
        counts.total,
        confidence_level,
    )?;

    build_metric(
        MetricKind::LeakageRate,
        probability,
        MetricUnit::Probability,
        MetricQuality::Observed,
        Some(confidence),
        Some(counts.total),
        Some(counts.total),
        vec![
            metadata(
                METHOD_KEY,
                LeakageMethod::DirectObservation.id(),
            )?,
            metadata(
                OBSERVATION_TYPE_KEY,
                "binary_observation",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates leakage as the complement of measured computational-subspace
/// population.
///
/// ```text
/// leakage_rate = 1 - computational_subspace_probability
/// ```
///
/// This is a derived metric, not a direct leakage count.
pub fn leakage_rate_from_computational_subspace_probability(
    computational_subspace_probability: f64,
) -> LeakageResult<Metric> {
    leakage_rate_from_computational_subspace_probability_with_metadata(
        computational_subspace_probability,
        None,
        None,
        None,
    )
}

/// Calculates leakage from computational-subspace population while retaining
/// measurement context.
pub fn leakage_rate_from_computational_subspace_probability_with_metadata(
    computational_subspace_probability: f64,
    uncertainty: Option<f64>,
    sample_count: Option<u64>,
    computational_subspace: Option<&str>,
) -> LeakageResult<Metric> {
    validate_probability(
        "computational_subspace_probability",
        computational_subspace_probability,
    )?;

    let leakage =
        ONE - computational_subspace_probability;

    validate_probability("leakage_rate", leakage)?;

    let uncertainty =
        validate_optional_uncertainty(uncertainty)?;

    let mut metadata_entries = vec![
        metadata(
            METHOD_KEY,
            LeakageMethod::ComplementOfSubspacePopulation.id(),
        )?,
        metadata(
            SOURCE_KEY,
            "computational_subspace_population",
        )?,
        metadata(
            MODEL_VERSION_KEY,
            MODEL_VERSION,
        )?,
    ];

    if let Some(subspace) = computational_subspace {
        metadata_entries.push(
            metadata(SUBSPACE_KEY, subspace)?,
        );
    }

    build_metric(
        MetricKind::LeakageRate,
        leakage,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        sample_count,
        sample_count,
        metadata_entries,
    )
    .map(|mut metric| {
        metric.uncertainty = uncertainty;
        metric
    })
}

/// Calculates the computational-subspace survival probability as the
/// complement of leakage.
///
/// ```text
/// survival = 1 - leakage
/// ```
pub fn survival_probability_from_leakage(
    leakage_rate: f64,
) -> LeakageResult<Metric> {
    validate_probability("leakage_rate", leakage_rate)?;

    let survival = ONE - leakage_rate;

    build_metric(
        MetricKind::Probability,
        survival,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        vec![
            metadata(
                DERIVED_KEY,
                "leakage_rate",
            )?,
            metadata(
                METHOD_KEY,
                "complement",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates leakage per cycle from an observed survival probability after
/// repeated identical cycles.
///
/// The mathematical model is:
///
/// ```text
/// survival = (1 - leakage_per_cycle)^cycles
///
/// leakage_per_cycle = 1 - survival^(1 / cycles)
/// ```
///
/// This is a MODEL-BASED quantity. It must not be interpreted as direct
/// per-cycle observation.
///
/// The model assumes a constant independent leakage probability per cycle.
pub fn leakage_rate_per_cycle_from_survival(
    survival_probability: f64,
    cycles: u64,
) -> LeakageResult<Metric> {
    leakage_rate_per_cycle_from_survival_with_sample_count(
        survival_probability,
        cycles,
        None,
    )
}

/// Calculates model-derived per-cycle leakage while preserving sample count.
pub fn leakage_rate_per_cycle_from_survival_with_sample_count(
    survival_probability: f64,
    cycles: u64,
    sample_count: Option<u64>,
) -> LeakageResult<Metric> {
    validate_probability(
        "survival_probability",
        survival_probability,
    )?;

    validate_cycles(cycles)?;

    let exponent = ONE / cycles as f64;

    let retained_per_cycle =
        survival_probability.powf(exponent);

    let leakage_per_cycle =
        ONE - retained_per_cycle;

    validate_probability(
        "leakage_per_cycle",
        leakage_per_cycle,
    )?;

    build_metric(
        MetricKind::LeakageRate,
        leakage_per_cycle,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        None,
        sample_count,
        sample_count,
        vec![
            metadata(
                METHOD_KEY,
                LeakageMethod::PerCycleFromSurvival.id(),
            )?,
            metadata(
                SOURCE_KEY,
                "survival_probability",
            )?,
            metadata(
                MODEL_KEY,
                "constant_independent_per_cycle_leakage",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
            metadata(
                CYCLES_KEY,
                cycles.to_string(),
            )?,
        ],
    )
}

/// Calculates the total leakage after a number of cycles from a per-cycle
/// leakage probability.
///
/// ```text
/// total_survival = (1 - l)^cycles
/// total_leakage  = 1 - total_survival
/// ```
pub fn total_leakage_from_per_cycle_rate(
    leakage_per_cycle: f64,
    cycles: u64,
) -> LeakageResult<Metric> {
    validate_probability(
        "leakage_per_cycle",
        leakage_per_cycle,
    )?;

    validate_cycles(cycles)?;

    let survival_per_cycle =
        ONE - leakage_per_cycle;

    let total_survival =
        survival_per_cycle.powf(cycles as f64);

    let total_leakage =
        ONE - total_survival;

    validate_probability(
        "total_leakage",
        total_leakage,
    )?;

    build_metric(
        MetricKind::LeakageRate,
        total_leakage,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        vec![
            metadata(
                METHOD_KEY,
                "accumulated_from_per_cycle_rate",
            )?,
            metadata(
                SOURCE_KEY,
                "per_cycle_leakage_rate",
            )?,
            metadata(
                MODEL_KEY,
                "constant_independent_per_cycle_leakage",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
            metadata(
                CYCLES_KEY,
                cycles.to_string(),
            )?,
        ],
    )
}

/// Calculates recovery probability from an initially leaked population.
///
/// ```text
/// recovery_rate = recovered / initially_leaked
/// ```
pub fn recovery_rate_from_counts(
    initially_leaked: u64,
    recovered: u64,
) -> LeakageResult<Metric> {
    let counts =
        LeakageRecoveryCounts::new(
            initially_leaked,
            recovered,
        )?;

    recovery_rate_from_counts_with_confidence(
        counts,
        DEFAULT_CONFIDENCE_LEVEL,
    )
}

/// Calculates recovery probability with an explicit confidence level.
pub fn recovery_rate_from_counts_with_confidence(
    counts: LeakageRecoveryCounts,
    confidence_level: f64,
) -> LeakageResult<Metric> {
    validate_confidence_level(confidence_level)?;

    let probability = counts.probability();

    let confidence = wilson_confidence(
        counts.recovered,
        counts.initially_leaked,
        confidence_level,
    )?;

    build_metric(
        MetricKind::Probability,
        probability,
        MetricUnit::Probability,
        MetricQuality::Observed,
        Some(confidence),
        Some(counts.initially_leaked),
        Some(counts.initially_leaked),
        vec![
            metadata(
                METHOD_KEY,
                LeakageMethod::RecoveryFromLeakedPopulation.id(),
            )?,
            metadata(
                INITIAL_POPULATION_KEY,
                counts.initially_leaked.to_string(),
            )?,
            metadata(
                RECOVERED_POPULATION_KEY,
                counts.recovered.to_string(),
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates the residual leakage after recovery.
///
/// ```text
/// residual_leakage = 1 - recovery_rate
/// ```
pub fn residual_leakage_from_recovery_rate(
    recovery_rate: f64,
) -> LeakageResult<Metric> {
    validate_probability(
        "recovery_rate",
        recovery_rate,
    )?;

    let residual =
        ONE - recovery_rate;

    build_metric(
        MetricKind::LeakageRate,
        residual,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        vec![
            metadata(
                DERIVED_KEY,
                "recovery_rate",
            )?,
            metadata(
                METHOD_KEY,
                "complement",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates leakage from retained computational-subspace counts.
///
/// `retained` is the number of observations remaining in the computational
/// subspace and `total` is the total number of observations.
///
/// ```text
/// leakage = (total - retained) / total
/// ```
pub fn leakage_rate_from_retained_counts(
    retained: u64,
    total: u64,
) -> LeakageResult<Metric> {
    if total < MIN_COUNT {
        return Err(LeakageError::InvalidCount {
            field: "total",
            value: total,
        });
    }

    if retained > total {
        return Err(
            LeakageError::NumeratorExceedsDenominator {
                numerator: retained,
                denominator: total,
                field: "retained",
            },
        );
    }

    let leaked = total - retained;

    leakage_rate_from_counts(
        leaked,
        total,
    )
}

/// Calculates survival probability from retained and total observations.
pub fn survival_probability_from_retained_counts(
    retained: u64,
    total: u64,
) -> LeakageResult<Metric> {
    if total < MIN_COUNT {
        return Err(LeakageError::InvalidCount {
            field: "total",
            value: total,
        });
    }

    if retained > total {
        return Err(
            LeakageError::NumeratorExceedsDenominator {
                numerator: retained,
                denominator: total,
                field: "retained",
            },
        );
    }

    let probability =
        retained as f64 / total as f64;

    let confidence = wilson_confidence(
        retained,
        total,
        DEFAULT_CONFIDENCE_LEVEL,
    )?;

    build_metric(
        MetricKind::Probability,
        probability,
        MetricUnit::Probability,
        MetricQuality::Observed,
        Some(confidence),
        Some(total),
        Some(total),
        vec![
            metadata(
                METHOD_KEY,
                "direct_computational_subspace_survival",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates the leakage complement of an observed survival metric.
///
/// This helper is useful when a protocol already has a canonical survival
/// probability but needs the corresponding leakage metric.
pub fn leakage_rate_from_survival_probability(
    survival_probability: f64,
) -> LeakageResult<Metric> {
    validate_probability(
        "survival_probability",
        survival_probability,
    )?;

    let leakage =
        ONE - survival_probability;

    build_metric(
        MetricKind::LeakageRate,
        leakage,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        vec![
            metadata(
                DERIVED_KEY,
                "survival_probability",
            )?,
            metadata(
                METHOD_KEY,
                "complement",
            )?,
            metadata(
                MODEL_VERSION_KEY,
                MODEL_VERSION,
            )?,
        ],
    )
}

/// Calculates the survival complement of an observed leakage metric.
///
/// This helper is useful for leakage-RB and repeated-cycle protocols.
pub fn survival_probability_from_leakage_rate(
    leakage_rate: f64,
) -> LeakageResult<Metric> {
    survival_probability_from_leakage(
        leakage_rate,
    )
}

/// Returns the upper Wilson confidence bound for zero observed leakage.
///
/// This is useful when an experiment observes zero leakage. Zero observations
/// do not establish zero underlying leakage; the confidence upper bound
/// remains informative.
pub fn upper_confidence_bound_for_zero_leakage(
    samples: u64,
    confidence_level: f64,
) -> LeakageResult<f64> {
    if samples < MIN_COUNT {
        return Err(LeakageError::InvalidCount {
            field: "samples",
            value: samples,
        });
    }

    validate_confidence_level(
        confidence_level,
    )?;

    let confidence = wilson_confidence(
        0,
        samples,
        confidence_level,
    )?;

    Ok(confidence.upper.get())
}

/// Creates a confidence interval for an observed leakage count.
pub fn leakage_confidence_interval(
    leaked: u64,
    total: u64,
    confidence_level: f64,
) -> LeakageResult<MetricConfidence> {
    let counts =
        LeakageCounts::new(leaked, total)?;

    wilson_confidence(
        counts.leaked,
        counts.total,
        confidence_level,
    )
}

/// Creates a confidence interval for an observed recovery count.
pub fn recovery_confidence_interval(
    recovered: u64,
    initially_leaked: u64,
    confidence_level: f64,
) -> LeakageResult<MetricConfidence> {
    let counts =
        LeakageRecoveryCounts::new(
            initially_leaked,
            recovered,
        )?;

    wilson_confidence(
        counts.recovered,
        counts.initially_leaked,
        confidence_level,
    )
}

/// Builds a canonical Zamani metric.
///
/// This local builder deliberately uses the already-established universal
/// `Metric` representation. It does not introduce a leakage-specific result
/// type that would have to be reconciled later.
fn build_metric(
    kind: MetricKind,
    value: f64,
    unit: MetricUnit,
    quality: MetricQuality,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    shot_count: Option<u64>,
    metadata: Vec<MetricMetadata>,
) -> LeakageResult<Metric> {
    validate_metric_value(
        kind.requires_unit_interval(),
        value,
    )?;

    let mut metric =
        Metric::new(kind, unit, value)
            .map_err(|error| {
                LeakageError::MetricConstruction(
                    error.to_string(),
                )
            })?;

    metric.quality = quality;
    metric.confidence = confidence;
    metric.sample_count = sample_count;
    metric.shot_count = shot_count;
    metric.metadata = metadata;

    metric
        .validate()
        .map_err(|error| {
            LeakageError::MetricConstruction(
                error.to_string(),
            )
        })?;

    Ok(metric)
}

/// Creates one validated metric metadata entry.
fn metadata(
    key: &str,
    value: impl Into<String>,
) -> LeakageResult<MetricMetadata> {
    let value = value.into();

    if key.trim().is_empty() {
        return Err(
            LeakageError::EmptyIdentifier {
                field: "metadata.key",
            },
        );
    }

    if value.trim().is_empty() {
        return Err(
            LeakageError::EmptyIdentifier {
                field: "metadata.value",
            },
        );
    }

    const MAX_METADATA_VALUE: usize = 16 * 1024;

    if value.len() > MAX_METADATA_VALUE {
        return Err(
            LeakageError::IdentifierTooLong {
                field: "metadata.value",
                maximum: MAX_METADATA_VALUE,
            },
        );
    }

    MetricMetadata::new(
        key.to_owned(),
        value,
    )
    .map_err(|error| {
        LeakageError::MetricConstruction(
            error.to_string(),
        )
    })
}

/// Validates a probability.
fn validate_probability(
    field: &'static str,
    value: f64,
) -> LeakageResult<()> {
    if !value.is_finite()
        || !(ZERO..=ONE).contains(&value)
    {
        return Err(
            LeakageError::InvalidProbability {
                field,
                value,
            },
        );
    }

    Ok(())
}

/// Validates a metric value.
///
/// Leakage metrics are probabilities and therefore require [0, 1].
fn validate_metric_value(
    requires_unit_interval: bool,
    value: f64,
) -> LeakageResult<()> {
    if !value.is_finite() {
        return Err(
            LeakageError::NonFiniteResult {
                operation: "metric construction",
                value,
            },
        );
    }

    if requires_unit_interval {
        validate_probability(
            "metric_value",
            value,
        )?;
    }

    Ok(())
}

/// Validates an optional standard uncertainty.
fn validate_optional_uncertainty(
    uncertainty: Option<f64>,
) -> LeakageResult<Option<FiniteF64>> {
    match uncertainty {
        Some(value) => {
            if !value.is_finite()
                || value < ZERO
            {
                return Err(
                    LeakageError::InvalidProbability {
                        field: "uncertainty",
                        value,
                    },
                );
            }

            FiniteF64::new(value)
                .map(Some)
                .map_err(|error| {
                    LeakageError::MetricConstruction(
                        error.to_string(),
                    )
                })
        }

        None => Ok(None),
    }
}

/// Validates a confidence level.
fn validate_confidence_level(
    confidence_level: f64,
) -> LeakageResult<()> {
    if !confidence_level.is_finite()
        || !(ZERO < confidence_level
            && confidence_level < ONE)
    {
        return Err(
            LeakageError::InvalidConfidenceLevel {
                value: confidence_level,
            },
        );
    }

    Ok(())
}

/// Validates cycle count.
fn validate_cycles(
    cycles: u64,
) -> LeakageResult<()> {
    if cycles < MIN_CYCLES {
        return Err(
            LeakageError::InvalidCycleCount {
                value: cycles,
            },
        );
    }

    Ok(())
}

/// Calculates a Wilson confidence interval.
///
/// The calculation uses an internally implemented normal quantile so this
/// metric module does not introduce a numerical dependency or duplicate the
/// statistical-engine layer's public API.
///
/// The canonical `statistics::confidence` module can later become the common
/// implementation for all benchmark metrics without changing this file's
/// public semantics.
fn wilson_confidence(
    successes: u64,
    trials: u64,
    confidence_level: f64,
) -> LeakageResult<MetricConfidence> {
    if trials < MIN_COUNT {
        return Err(LeakageError::InvalidCount {
            field: "trials",
            value: trials,
        });
    }

    if successes > trials {
        return Err(
            LeakageError::NumeratorExceedsDenominator {
                numerator: successes,
                denominator: trials,
                field: "successes",
            },
        );
    }

    validate_confidence_level(
        confidence_level,
    )?;

    let p =
        successes as f64 / trials as f64;

    validate_probability(
        "empirical_probability",
        p,
    )?;

    let z =
        inverse_normal_cdf(
            0.5 + confidence_level / 2.0,
        );

    if !z.is_finite() {
        return Err(
            LeakageError::NonFiniteResult {
                operation: "normal_quantile",
                value: z,
            },
        );
    }

    let n = trials as f64;
    let z_squared = z * z;

    let denominator =
        1.0 + z_squared / n;

    let center =
        (p + z_squared / (2.0 * n))
            / denominator;

    let variance_term =
        (p * (ONE - p) / n)
            + (z_squared
                / (4.0 * n * n));

    if !variance_term.is_finite()
        || variance_term < ZERO
    {
        return Err(
            LeakageError::NonFiniteResult {
                operation: "wilson_variance",
                value: variance_term,
            },
        );
    }

    let margin =
        z * variance_term.sqrt()
            / denominator;

    let lower =
        (center - margin).max(ZERO);

    let upper =
        (center + margin).min(ONE);

    if !lower.is_finite()
        || !upper.is_finite()
        || lower > upper
    {
        return Err(
            LeakageError::InvalidConfidenceInterval {
                lower,
                upper,
            },
        );
    }

    MetricConfidence::new(
        confidence_level,
        lower,
        upper,
        ConfidenceMethod::Wilson,
    )
    .map_err(|error| {
        LeakageError::MetricConstruction(
            error.to_string(),
        )
    })
}

/// Transforms a confidence interval for p into the corresponding confidence
/// interval for 1-p.
///
/// The bounds reverse:
///
/// ```text
/// [p_low, p_high]
///       ↓
/// [1-p_high, 1-p_low]
/// ```
fn complement_confidence(
    confidence: &MetricConfidence,
) -> LeakageResult<MetricConfidence> {
    let lower =
        ONE - confidence.upper.get();

    let upper =
        ONE - confidence.lower.get();

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
    .map_err(|error| {
        LeakageError::MetricConstruction(
            error.to_string(),
        )
    })
}

/// Approximation of the inverse standard-normal CDF.
///
/// This is the Acklam rational approximation.
///
/// It is intentionally local to the metric implementation so the module
/// remains independently testable. The broader benchmarking statistics layer
/// may eventually centralize this implementation.
fn inverse_normal_cdf(
    p: f64,
) -> f64 {
    const A: [f64; 6] = [
        -39.69683028665376,
        220.9460984245205,
        -275.9285104469687,
        138.3577518672690,
        -30.66479806614716,
        2.506628277459239,
    ];

    const B: [f64; 5] = [
        -54.47609879822406,
        161.5858368580409,
        -155.6989798598866,
        66.80131188771972,
        -13.28068155288572,
    ];

    const C: [f64; 6] = [
        -0.007784894002430293,
        -0.3223964580411365,
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        0.007784695709041462,
        0.3224671290700398,
        2.445134137142996,
        3.754408661907416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    if p <= ZERO {
        return f64::NEG_INFINITY;
    }

    if p >= ONE {
        return f64::INFINITY;
    }

    if p < LOW {
        let q =
            (-2.0 * p.ln()).sqrt();

        return (((((C[0] * q + C[1]) * q
            + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q
                + D[2])
                * q
                + D[3])
                * q
                + ONE);
    }

    if p > HIGH {
        let q =
            (-2.0 * (ONE - p).ln()).sqrt();

        return -(((((C[0] * q + C[1]) * q
            + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q
                + D[2])
                * q
                + D[3])
                * q
                + ONE);
    }

    let q =
        p - 0.5;

    let r = q * q;

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
            * r)
            + ONE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_leakage_from_counts_is_correct() {
        let metric =
            leakage_rate_from_counts(
                10,
                1_000,
            )
            .expect("valid leakage counts");

        assert!(
            (metric.value.get() - 0.01).abs()
                < 1.0e-12
        );

        assert_eq!(
            metric.kind,
            MetricKind::LeakageRate
        );

        assert_eq!(
            metric.unit,
            MetricUnit::Probability
        );

        assert_eq!(
            metric.quality,
            MetricQuality::Observed
        );
    }

    #[test]
    fn zero_leakage_is_valid() {
        let metric =
            leakage_rate_from_counts(
                0,
                1_000,
            )
            .expect("zero observed leakage is valid");

        assert_eq!(
            metric.value.get(),
            0.0
        );

        let confidence =
            metric
                .confidence
                .expect("confidence interval");

        assert_eq!(
            confidence.lower.get(),
            0.0
        );

        assert!(
            confidence.upper.get() > 0.0
        );
    }

    #[test]
    fn complete_leakage_is_valid() {
        let metric =
            leakage_rate_from_counts(
                1_000,
                1_000,
            )
            .expect("complete leakage is valid");

        assert_eq!(
            metric.value.get(),
            1.0
        );

        let confidence =
            metric
                .confidence
                .expect("confidence interval");

        assert!(
            confidence.upper.get() <= 1.0
        );
    }

    #[test]
    fn invalid_counts_are_rejected() {
        let result =
            LeakageCounts::new(
                101,
                100,
            );

        assert!(
            matches!(
                result,
                Err(
                    LeakageError::NumeratorExceedsDenominator {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn zero_total_is_rejected() {
        let result =
            LeakageCounts::new(
                0,
                0,
            );

        assert!(
            matches!(
                result,
                Err(
                    LeakageError::InvalidCount {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn complement_of_subspace_population_is_correct() {
        let metric =
            leakage_rate_from_computational_subspace_probability(
                0.97,
            )
            .expect("valid probability");

        assert!(
            (metric.value.get() - 0.03).abs()
                < 1.0e-12
        );

        assert_eq!(
            metric.quality,
            MetricQuality::Derived
        );
    }

    #[test]
    fn invalid_subspace_probability_is_rejected() {
        assert!(
            leakage_rate_from_computational_subspace_probability(
                1.1
            )
            .is_err()
        );

        assert!(
            leakage_rate_from_computational_subspace_probability(
                -0.1
            )
            .is_err()
        );
    }

    #[test]
    fn survival_and_leakage_are_complements() {
        let survival =
            survival_probability_from_leakage(
                0.25,
            )
            .expect("valid leakage");

        assert!(
            (survival.value.get() - 0.75).abs()
                < 1.0e-12
        );

        let leakage =
            leakage_rate_from_survival_probability(
                0.75,
            )
            .expect("valid survival");

        assert!(
            (leakage.value.get() - 0.25).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn per_cycle_leakage_is_correct() {
        let metric =
            leakage_rate_per_cycle_from_survival(
                0.81,
                2,
            )
            .expect("valid survival and cycles");

        assert!(
            (metric.value.get() - 0.1).abs()
                < 1.0e-12
        );

        assert_eq!(
            metric.quality,
            MetricQuality::Estimated
        );
    }

    #[test]
    fn zero_cycles_are_rejected() {
        assert!(
            leakage_rate_per_cycle_from_survival(
                0.9,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn per_cycle_round_trip_is_consistent() {
        let per_cycle =
            0.02;

        let total =
            total_leakage_from_per_cycle_rate(
                per_cycle,
                10,
            )
            .expect("valid total leakage");

        let recovered =
            leakage_rate_per_cycle_from_survival(
                1.0 - total.value.get(),
                10,
            )
            .expect("valid inverse");

        assert!(
            (recovered.value.get()
                - per_cycle)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn recovery_rate_is_correct() {
        let metric =
            recovery_rate_from_counts(
                1_000,
                800,
            )
            .expect("valid recovery counts");

        assert!(
            (metric.value.get() - 0.8).abs()
                < 1.0e-12
        );

        assert_eq!(
            metric.quality,
            MetricQuality::Observed
        );
    }

    #[test]
    fn recovery_cannot_exceed_initial_leakage() {
        assert!(
            recovery_rate_from_counts(
                100,
                101,
            )
            .is_err()
        );
    }

    #[test]
    fn zero_initial_leakage_is_rejected() {
        assert!(
            recovery_rate_from_counts(
                0,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn residual_leakage_is_recovery_complement() {
        let metric =
            residual_leakage_from_recovery_rate(
                0.8,
            )
            .expect("valid recovery rate");

        assert!(
            (metric.value.get() - 0.2).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn retained_counts_produce_correct_leakage() {
        let metric =
            leakage_rate_from_retained_counts(
                950,
                1_000,
            )
            .expect("valid retained counts");

        assert!(
            (metric.value.get() - 0.05).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn retained_counts_produce_correct_survival() {
        let metric =
            survival_probability_from_retained_counts(
                950,
                1_000,
            )
            .expect("valid retained counts");

        assert!(
            (metric.value.get() - 0.95).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn confidence_interval_is_valid() {
        let confidence =
            leakage_confidence_interval(
                50,
                1_000,
                0.95,
            )
            .expect("valid interval");

        assert!(
            confidence.lower.get()
                <= 0.05
        );

        assert!(
            confidence.upper.get()
                >= 0.05
        );

        assert!(
            confidence.lower.get()
                >= 0.0
        );

        assert!(
            confidence.upper.get()
                <= 1.0
        );
    }

    #[test]
    fn confidence_level_must_be_valid() {
        assert!(
            leakage_confidence_interval(
                10,
                100,
                0.0,
            )
            .is_err()
        );

        assert!(
            leakage_confidence_interval(
                10,
                100,
                1.0,
            )
            .is_err()
        );

        assert!(
            leakage_confidence_interval(
                10,
                100,
                f64::NAN,
            )
            .is_err()
        );
    }

    #[test]
    fn zero_leakage_upper_bound_is_nonzero() {
        let upper =
            upper_confidence_bound_for_zero_leakage(
                1_000,
                0.95,
            )
            .expect("valid confidence bound");

        assert!(
            upper > 0.0
        );

        assert!(
            upper < 0.01
        );
    }

    #[test]
    fn metadata_identifies_derived_per_cycle_model() {
        let metric =
            leakage_rate_per_cycle_from_survival(
                0.9,
                5,
            )
            .expect("valid");

        assert!(
            metric.metadata.iter().any(
                |entry| {
                    entry.key == MODEL_KEY
                        && entry.value
                            == "constant_independent_per_cycle_leakage"
                }
            )
        );
    }

    #[test]
    fn direct_metric_contains_confidence_information() {
        let metric =
            leakage_rate_from_counts(
                25,
                1_000,
            )
            .expect("valid");

        assert!(
            metric.confidence.is_some()
        );

        assert_eq!(
            metric.sample_count,
            Some(1_000)
        );

        assert_eq!(
            metric.shot_count,
            Some(1_000)
        );
    }

    #[test]
    fn non_finite_probability_is_rejected() {
        assert!(
            leakage_rate_from_survival_probability(
                f64::NAN
            )
            .is_err()
        );

        assert!(
            leakage_rate_from_survival_probability(
                f64::INFINITY
            )
            .is_err()
        );
    }
}