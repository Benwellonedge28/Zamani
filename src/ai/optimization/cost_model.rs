#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Optimization — Cost Model
//!
//! Path: `src/ai/optimization/cost_model.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides target-independent, extensible, multidimensional cost
//! estimation for Zamani AI computations.
//!
//! ## Architectural ownership
//!
//! This module describes and estimates computational costs. It does not:
//!
//! - Select physical execution devices.
//! - Allocate resources.
//! - Schedule workloads.
//! - Partition computational graphs.
//! - Implement tensor storage.
//! - Execute AI operations.
//! - Define AI semantic correctness.
//!
//! Target-specific coefficients are supplied by downstream target,
//! runtime, or execution-analysis components.
//!
//! ## POCO-REAF
//!
//! There are no fixed workload, tensor-rank, model-size, parameter-count,
//! accelerator-count, or worker-count ceilings.
//!
//! Cost dimensions are extensible through `MetricId`. Operational
//! limits belong to separately supplied execution policies.
//!
//! ## Numerical contract
//!
//! Externally supplied quantities and coefficients must be finite and
//! non-negative. NaN, infinity, and negative values are rejected.
//!
//! Arithmetic overflow is reported explicitly. Estimates are never
//! silently saturated or converted into successful zero-cost results.
//!
//! ## Units contract
//!
//! A `Quantity` does not carry a physical unit. The metric identity and
//! the supplied profile define the interpretation of each quantity and
//! coefficient. Callers must not combine incompatible dimensions in an
//! objective unless their units and interpretation make that meaningful.
//!
//! ## Estimation contract
//!
//! An analytical estimate requires a coefficient for every workload
//! metric. Missing coefficients are errors, not implicit zero costs.
//!
//! A profile's standard-deviation coefficient is scaled linearly by
//! workload quantity. This is a coefficient-based estimate, not a
//! statistical independence or variance-combination model.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// ============================================================================
// METRIC IDENTITIES
// ============================================================================

/// Identifies a cost dimension.
///
/// The identifier is open-ended. New computational cost dimensions
/// can be introduced without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricId(String);

impl MetricId {
    /// Creates a metric identity.
    ///
    /// Empty, whitespace-only, surrounding-whitespace, and control
    /// character-containing identifiers are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        if value.trim().is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(CostModelError::InvalidMetricId);
        }

        Ok(Self(value))
    }

    /// Returns the canonical metric identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Standard metric identifiers.
///
/// These are conventions, not an exhaustive enumeration.
/// Custom metrics remain supported.
pub mod metrics {
    /// Computational operation count.
    pub const COMPUTE_OPERATIONS: &str = "compute.operations";

    /// Memory footprint or movement in bytes.
    pub const MEMORY_BYTES: &str = "memory.bytes";

    /// Memory-read volume in bytes.
    pub const MEMORY_READ_BYTES: &str = "memory.read_bytes";

    /// Memory-write volume in bytes.
    pub const MEMORY_WRITE_BYTES: &str = "memory.write_bytes";

    /// Network transfer volume in bytes.
    pub const NETWORK_BYTES: &str = "network.bytes";

    /// Network message count.
    pub const NETWORK_MESSAGES: &str = "network.messages";

    /// Synchronization point count.
    pub const SYNCHRONIZATION_POINTS: &str =
        "synchronization.points";

    /// Energy in joules.
    pub const ENERGY_JOULES: &str = "energy.joules";

    /// Latency in seconds.
    pub const LATENCY_SECONDS: &str = "latency.seconds";

    /// Storage volume in bytes.
    pub const STORAGE_BYTES: &str = "storage.bytes";

    /// Financial cost in a profile-defined currency or unit.
    pub const FINANCIAL_COST: &str = "financial.cost";
}

// ============================================================================
// QUANTITIES
// ============================================================================

/// A finite, non-negative quantity.
///
/// Quantities are kept separate from their units and cost coefficients.
/// This avoids silently combining incompatible units.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity(f64);

impl Quantity {
    /// Constructs a finite, non-negative quantity.
    pub fn new(value: f64) -> Result<Self, CostModelError> {
        if !value.is_finite() {
            return Err(CostModelError::NonFiniteValue);
        }

        if value < 0.0 {
            return Err(CostModelError::NegativeValue);
        }

        Ok(Self(value))
    }

    /// Returns a zero quantity.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the underlying numerical value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Result<Self, CostModelError> {
        let result = self.0 + other.0;

        if !result.is_finite() {
            return Err(CostModelError::ArithmeticOverflow);
        }

        Self::new(result)
    }

    fn checked_mul(self, other: Self) -> Result<Self, CostModelError> {
        let result = self.0 * other.0;

        if !result.is_finite() {
            return Err(CostModelError::ArithmeticOverflow);
        }

        Self::new(result)
    }
}

// ============================================================================
// WORKLOAD
// ============================================================================

/// Describes an abstract computational workload.
///
/// This is a collection of logical workload quantities. It does not
/// describe physical device allocation.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Workload {
    quantities: BTreeMap<MetricId, Quantity>,
}

impl Workload {
    /// Creates an empty workload.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a metric quantity.
    ///
    /// Returns an error if the metric already exists.
    pub fn insert(
        &mut self,
        metric: MetricId,
        quantity: Quantity,
    ) -> Result<(), CostModelError> {
        if self.quantities.contains_key(&metric) {
            return Err(CostModelError::DuplicateMetric(
                metric.as_str().to_owned(),
            ));
        }

        self.quantities.insert(metric, quantity);
        Ok(())
    }

    /// Inserts or replaces a metric quantity.
    pub fn set(
        &mut self,
        metric: MetricId,
        quantity: Quantity,
    ) {
        self.quantities.insert(metric, quantity);
    }

    /// Returns a metric quantity, if present.
    #[must_use]
    pub fn get(&self, metric: &MetricId) -> Option<Quantity> {
        self.quantities.get(metric).copied()
    }

    /// Iterates over workload metrics in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &Quantity)> {
        self.quantities.iter()
    }

    /// Returns whether the workload has no metrics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.quantities.is_empty()
    }
}

// ============================================================================
// COST COEFFICIENTS
// ============================================================================

/// A cost coefficient.
///
/// Represents the estimated cost associated with one unit of a
/// particular workload metric.
///
/// The unit and interpretation are defined by the supplying profile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostCoefficient {
    cost_per_unit: Quantity,
    standard_deviation: Quantity,
}

impl CostCoefficient {
    /// Creates a cost coefficient.
    ///
    /// Both values must already satisfy the `Quantity` contract.
    #[must_use]
    pub const fn new(
        cost_per_unit: Quantity,
        standard_deviation: Quantity,
    ) -> Self {
        Self {
            cost_per_unit,
            standard_deviation,
        }
    }

    /// Returns the estimated cost per workload unit.
    #[must_use]
    pub const fn cost_per_unit(&self) -> Quantity {
        self.cost_per_unit
    }

    /// Returns the standard-deviation coefficient per workload unit.
    #[must_use]
    pub const fn standard_deviation(&self) -> Quantity {
        self.standard_deviation
    }
}

// ============================================================================
// COST PROFILES
// ============================================================================

/// An externally supplied cost profile.
///
/// A profile may represent an abstract execution realization, CPU,
/// GPU, distributed, quantum, hybrid, or other execution cost model.
///
/// This module does not determine which profile should be selected.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostProfile {
    coefficients: BTreeMap<MetricId, CostCoefficient>,
    version: String,
}

impl CostProfile {
    /// Creates a cost profile with a non-empty version identifier.
    pub fn new(
        version: impl Into<String>,
    ) -> Result<Self, CostModelError> {
        let version = version.into();

        if version.trim().is_empty()
            || version.trim() != version
            || version.chars().any(char::is_control)
        {
            return Err(CostModelError::InvalidProfileVersion);
        }

        Ok(Self {
            coefficients: BTreeMap::new(),
            version,
        })
    }

    /// Returns the profile version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Registers a coefficient for a metric.
    ///
    /// Returns an error if the metric is already registered.
    pub fn register(
        &mut self,
        metric: MetricId,
        coefficient: CostCoefficient,
    ) -> Result<(), CostModelError> {
        if self.coefficients.contains_key(&metric) {
            return Err(CostModelError::DuplicateMetric(
                metric.as_str().to_owned(),
            ));
        }

        self.coefficients.insert(metric, coefficient);
        Ok(())
    }

    /// Returns a coefficient for a metric, if present.
    #[must_use]
    pub fn coefficient(
        &self,
        metric: &MetricId,
    ) -> Option<&CostCoefficient> {
        self.coefficients.get(metric)
    }

    /// Iterates over profile coefficients in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &CostCoefficient)> {
        self.coefficients.iter()
    }
}

// ============================================================================
// OBJECTIVES
// ============================================================================

/// Describes how a metric contributes to an optimization objective.
///
/// Objective weights are supplied by the caller. There are no implicit
/// universal optimization priorities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveWeight {
    weight: Quantity,
}

impl ObjectiveWeight {
    /// Creates an objective weight.
    #[must_use]
    pub const fn new(weight: Quantity) -> Self {
        Self { weight }
    }

    /// Returns the weight quantity.
    #[must_use]
    pub const fn get(self) -> Quantity {
        self.weight
    }
}

/// An explicitly supplied optimization objective.
///
/// Scalar scoring is optional because different objectives may have
/// incompatible units and should not automatically be combined.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Objective {
    weights: BTreeMap<MetricId, ObjectiveWeight>,
}

impl Objective {
    /// Creates an empty objective.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or replaces the weight for a metric.
    pub fn set_weight(
        &mut self,
        metric: MetricId,
        weight: ObjectiveWeight,
    ) {
        self.weights.insert(metric, weight);
    }

    /// Returns the weight for a metric, if present.
    #[must_use]
    pub fn weight(
        &self,
        metric: &MetricId,
    ) -> Option<ObjectiveWeight> {
        self.weights.get(metric).copied()
    }
}

// ============================================================================
// ESTIMATES
// ============================================================================

/// Individual estimated cost contribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EstimatedCost {
    /// Expected cost for the metric.
    pub expected: Quantity,

    /// Estimated standard deviation for the metric.
    pub standard_deviation: Quantity,
}

/// Multidimensional cost estimate.
///
/// Individual metrics remain available rather than being prematurely
/// reduced to one scalar.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostEstimate {
    values: BTreeMap<MetricId, EstimatedCost>,
    profile_version: String,
}

impl CostEstimate {
    /// Returns all estimated metric values.
    #[must_use]
    pub fn values(&self) -> &BTreeMap<MetricId, EstimatedCost> {
        &self.values
    }

    /// Returns the estimate for a metric, if present.
    #[must_use]
    pub fn get(
        &self,
        metric: &MetricId,
    ) -> Option<&EstimatedCost> {
        self.values.get(metric)
    }

    /// Returns the profile version used to produce the estimate.
    #[must_use]
    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }

    /// Calculates an optional weighted score.
    ///
    /// The caller is responsible for ensuring that objective
    /// dimensions have meaningful compatible units.
    ///
    /// Missing objective dimensions are errors rather than silently
    /// being treated as zero.
    pub fn score(
        &self,
        objective: &Objective,
    ) -> Result<Quantity, CostModelError> {
        let mut total = Quantity::zero();

        for (metric, weight) in &objective.weights {
            let estimate = self.values.get(metric).ok_or_else(|| {
                CostModelError::MissingEstimatedMetric(
                    metric.as_str().to_owned(),
                )
            })?;

            let contribution = estimate
                .expected
                .checked_mul(weight.get())?;

            total = total.checked_add(contribution)?;
        }

        Ok(total)
    }
}

// ============================================================================
// COST MODEL CONTRACT
// ============================================================================

/// Abstract cost-model interface.
///
/// Implementations can provide analytical, empirical, learned,
/// simulated, or composite cost estimation.
pub trait CostModel {
    /// Estimates the cost of a workload under a supplied profile.
    ///
    /// Implementations must not treat missing required data as zero
    /// cost or return a successful estimate after arithmetic overflow.
    fn estimate(
        &self,
        workload: &Workload,
        profile: &CostProfile,
    ) -> Result<CostEstimate, CostModelError>;
}

// ============================================================================
// ANALYTICAL COST MODEL
// ============================================================================

/// Analytical cost model.
///
/// Uses externally supplied coefficients and logical workload
/// quantities. It does not assume any particular hardware.
#[derive(Debug, Clone, Default)]
pub struct AnalyticalCostModel;

impl AnalyticalCostModel {
    /// Creates an analytical cost model.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl CostModel for AnalyticalCostModel {
    fn estimate(
        &self,
        workload: &Workload,
        profile: &CostProfile,
    ) -> Result<CostEstimate, CostModelError> {
        let mut values = BTreeMap::new();

        for (metric, quantity) in workload.iter() {
            let coefficient =
                profile.coefficient(metric).ok_or_else(|| {
                    CostModelError::MissingCoefficient(
                        metric.as_str().to_owned(),
                    )
                })?;

            let expected = quantity
                .checked_mul(coefficient.cost_per_unit())?;

            let standard_deviation = quantity
                .checked_mul(coefficient.standard_deviation())?;

            values.insert(
                metric.clone(),
                EstimatedCost {
                    expected,
                    standard_deviation,
                },
            );
        }

        Ok(CostEstimate {
            values,
            profile_version: profile.version().to_owned(),
        })
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Describes why a cost estimate could not be produced.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CostModelError {
    /// A metric identifier is invalid.
    InvalidMetricId,

    /// A profile version is invalid.
    InvalidProfileVersion,

    /// A negative quantity or coefficient was supplied.
    NegativeValue,

    /// A non-finite numerical value was supplied.
    NonFiniteValue,

    /// A metric was inserted more than once.
    DuplicateMetric(String),

    /// A workload metric has no coefficient in the profile.
    MissingCoefficient(String),

    /// An objective references a metric that was not estimated.
    MissingEstimatedMetric(String),

    /// A model configuration is invalid.
    InvalidModelConfiguration(String),

    /// An arithmetic operation overflowed the representable range.
    ArithmeticOverflow,
}

impl fmt::Display for CostModelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidMetricId => {
                write!(formatter, "Cost metric identifier is invalid")
            }
            Self::InvalidProfileVersion => {
                write!(formatter, "Cost profile version is invalid")
            }
            Self::NegativeValue => {
                write!(formatter, "Negative cost quantity is invalid")
            }
            Self::NonFiniteValue => {
                write!(formatter, "Non-finite numerical value is invalid")
            }
            Self::DuplicateMetric(metric) => {
                write!(formatter, "Duplicate cost metric: {metric}")
            }
            Self::MissingCoefficient(metric) => {
                write!(
                    formatter,
                    "Missing cost coefficient for metric: {metric}",
                )
            }
            Self::MissingEstimatedMetric(metric) => {
                write!(
                    formatter,
                    "Objective references an unestimated metric: {metric}",
                )
            }
            Self::InvalidModelConfiguration(message) => {
                write!(formatter, "Invalid cost model: {message}")
            }
            Self::ArithmeticOverflow => {
                write!(formatter, "Cost estimation arithmetic overflow")
            }
        }
    }
}

impl Error for CostModelError {}

// ============================================================================
// PROFILE BUILDER
// ============================================================================

/// Builder for an analytical cost profile.
#[derive(Debug, Clone)]
pub struct CostProfileBuilder {
    profile: CostProfile,
}

impl CostProfileBuilder {
    /// Creates a profile builder.
    pub fn new(
        version: impl Into<String>,
    ) -> Result<Self, CostModelError> {
        Ok(Self {
            profile: CostProfile::new(version)?,
        })
    }

    /// Adds a metric coefficient.
    pub fn coefficient(
        mut self,
        metric: impl Into<String>,
        cost_per_unit: f64,
        standard_deviation: f64,
    ) -> Result<Self, CostModelError> {
        let metric = MetricId::new(metric)?;

        let coefficient = CostCoefficient::new(
            Quantity::new(cost_per_unit)?,
            Quantity::new(standard_deviation)?,
        );

        self.profile.register(metric, coefficient)?;

        Ok(self)
    }

    /// Builds the profile.
    #[must_use]
    pub fn build(self) -> CostProfile {
        self.profile
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(name: &str) -> MetricId {
        MetricId::new(name).expect("valid metric")
    }

    fn quantity(value: f64) -> Quantity {
        Quantity::new(value).expect("valid quantity")
    }

    #[test]
    fn estimates_multiple_cost_dimensions() {
        let mut workload = Workload::new();

        workload
            .insert(
                metric(metrics::COMPUTE_OPERATIONS),
                quantity(100.0),
            )
            .unwrap();

        workload
            .insert(
                metric(metrics::MEMORY_BYTES),
                quantity(2048.0),
            )
            .unwrap();

        let profile = CostProfileBuilder::new("test-v1")
            .unwrap()
            .coefficient(metrics::COMPUTE_OPERATIONS, 0.5, 0.1)
            .unwrap()
            .coefficient(metrics::MEMORY_BYTES, 0.01, 0.002)
            .unwrap()
            .build();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        assert_eq!(
            estimate
                .get(&metric(metrics::COMPUTE_OPERATIONS))
                .unwrap()
                .expected
                .get(),
            50.0,
        );

        assert_eq!(
            estimate
                .get(&metric(metrics::MEMORY_BYTES))
                .unwrap()
                .expected
                .get(),
            20.48,
        );
    }

    #[test]
    fn supports_extension_metrics() {
        let mut workload = Workload::new();

        workload
            .insert(
                metric("custom.quantum.entanglement"),
                quantity(12.0),
            )
            .unwrap();

        let profile = CostProfileBuilder::new("custom-v1")
            .unwrap()
            .coefficient(
                "custom.quantum.entanglement",
                2.0,
                0.25,
            )
            .unwrap()
            .build();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        assert_eq!(
            estimate
                .get(&metric("custom.quantum.entanglement"))
                .unwrap()
                .expected
                .get(),
            24.0,
        );
    }

    #[test]
    fn rejects_negative_quantities() {
        assert_eq!(
            Quantity::new(-1.0),
            Err(CostModelError::NegativeValue),
        );
    }

    #[test]
    fn rejects_nan() {
        assert_eq!(
            Quantity::new(f64::NAN),
            Err(CostModelError::NonFiniteValue),
        );
    }

    #[test]
    fn rejects_infinity() {
        assert_eq!(
            Quantity::new(f64::INFINITY),
            Err(CostModelError::NonFiniteValue),
        );
    }

    #[test]
    fn rejects_negative_zero_only_by_value_semantics() {
        // IEEE-754 negative zero is numerically equal to zero and is
        // accepted as a non-negative quantity.
        assert_eq!(
            Quantity::new(-0.0).unwrap().get(),
            0.0,
        );
    }

    #[test]
    fn detects_missing_coefficients() {
        let mut workload = Workload::new();

        workload
            .insert(metric("unknown.metric"), quantity(1.0))
            .unwrap();

        let profile = CostProfile::new("v1").unwrap();

        let result = AnalyticalCostModel.estimate(&workload, &profile);

        assert_eq!(
            result,
            Err(CostModelError::MissingCoefficient(
                "unknown.metric".to_owned(),
            )),
        );
    }

    #[test]
    fn rejects_duplicate_workload_metrics() {
        let mut workload = Workload::new();

        workload
            .insert(metric("compute"), quantity(1.0))
            .unwrap();

        assert!(matches!(
            workload.insert(metric("compute"), quantity(2.0)),
            Err(CostModelError::DuplicateMetric(_)),
        ));
    }

    #[test]
    fn rejects_duplicate_profile_coefficients() {
        let mut profile = CostProfile::new("v1").unwrap();

        profile
            .register(
                metric("compute"),
                CostCoefficient::new(quantity(1.0), quantity(0.0)),
            )
            .unwrap();

        assert!(matches!(
            profile.register(
                metric("compute"),
                CostCoefficient::new(quantity(2.0), quantity(0.0)),
            ),
            Err(CostModelError::DuplicateMetric(_)),
        ));
    }

    #[test]
    fn weighted_objective_preserves_explicit_priorities() {
        let mut workload = Workload::new();

        workload
            .insert(
                metric(metrics::COMPUTE_OPERATIONS),
                quantity(10.0),
            )
            .unwrap();

        let profile = CostProfileBuilder::new("v1")
            .unwrap()
            .coefficient(metrics::COMPUTE_OPERATIONS, 2.0, 0.0)
            .unwrap()
            .build();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        let mut objective = Objective::new();

        objective.set_weight(
            metric(metrics::COMPUTE_OPERATIONS),
            ObjectiveWeight::new(quantity(3.0)),
        );

        assert_eq!(
            estimate.score(&objective).unwrap().get(),
            60.0,
        );
    }

    #[test]
    fn objective_missing_metric_is_an_error() {
        let estimate = CostEstimate::default();
        let mut objective = Objective::new();

        objective.set_weight(
            metric("not.estimated"),
            ObjectiveWeight::new(quantity(1.0)),
        );

        assert_eq!(
            estimate.score(&objective),
            Err(CostModelError::MissingEstimatedMetric(
                "not.estimated".to_owned(),
            )),
        );
    }

    #[test]
    fn multiplication_overflow_is_reported_explicitly() {
        let result = quantity(f64::MAX).checked_mul(quantity(2.0));

        assert_eq!(
            result,
            Err(CostModelError::ArithmeticOverflow),
        );
    }

    #[test]
    fn addition_overflow_is_reported_explicitly() {
        let result = quantity(f64::MAX).checked_add(quantity(f64::MAX));

        assert_eq!(
            result,
            Err(CostModelError::ArithmeticOverflow),
        );
    }

    #[test]
    fn estimate_rejects_overflow_in_expected_cost() {
        let mut workload = Workload::new();

        workload
            .insert(metric("compute"), quantity(f64::MAX))
            .unwrap();

        let profile = CostProfileBuilder::new("overflow-v1")
            .unwrap()
            .coefficient("compute", 2.0, 0.0)
            .unwrap()
            .build();

        assert_eq!(
            AnalyticalCostModel.estimate(&workload, &profile),
            Err(CostModelError::ArithmeticOverflow),
        );
    }

    #[test]
    fn estimate_rejects_overflow_in_standard_deviation() {
        let mut workload = Workload::new();

        workload
            .insert(metric("compute"), quantity(f64::MAX))
            .unwrap();

        let profile = CostProfileBuilder::new("overflow-v1")
            .unwrap()
            .coefficient("compute", 0.0, 2.0)
            .unwrap()
            .build();

        assert_eq!(
            AnalyticalCostModel.estimate(&workload, &profile),
            Err(CostModelError::ArithmeticOverflow),
        );
    }

    #[test]
    fn weighted_score_rejects_overflow() {
        let estimate = CostEstimate {
            values: BTreeMap::from([(
                metric("compute"),
                EstimatedCost {
                    expected: quantity(f64::MAX),
                    standard_deviation: Quantity::zero(),
                },
            )]),
            profile_version: "test-v1".to_owned(),
        };

        let mut objective = Objective::new();

        objective.set_weight(
            metric("compute"),
            ObjectiveWeight::new(quantity(2.0)),
        );

        assert_eq!(
            estimate.score(&objective),
            Err(CostModelError::ArithmeticOverflow),
        );
    }

    #[test]
    fn empty_workload_has_zero_dimensions() {
        let workload = Workload::new();
        let profile = CostProfile::new("v1").unwrap();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        assert!(estimate.values().is_empty());
    }

    #[test]
    fn preserves_profile_version() {
        let workload = Workload::new();
        let profile = CostProfile::new("2026.1").unwrap();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        assert_eq!(estimate.profile_version(), "2026.1");
    }

    #[test]
    fn rejects_invalid_metric_identity() {
        assert_eq!(
            MetricId::new("  "),
            Err(CostModelError::InvalidMetricId),
        );

        assert_eq!(
            MetricId::new(" metric"),
            Err(CostModelError::InvalidMetricId),
        );

        assert_eq!(
            MetricId::new("metric\nname"),
            Err(CostModelError::InvalidMetricId),
        );
    }

    #[test]
    fn rejects_invalid_profile_version() {
        assert_eq!(
            CostProfile::new(" "),
            Err(CostModelError::InvalidProfileVersion),
        );

        assert_eq!(
            CostProfile::new("v1\n"),
            Err(CostModelError::InvalidProfileVersion),
        );
    }
}