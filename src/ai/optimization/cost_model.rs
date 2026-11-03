
// src/ai/optimization/cost_model.rs

//! # AI Cost Model
//!
//! Target-independent, extensible, multidimensional cost estimation
//! for Zamani AI computations.
//!
//! ## Architectural ownership
//!
//! This module describes and estimates computational costs.
//! It does not:
//!
//! - Select physical execution devices.
//! - Allocate resources.
//! - Schedule workloads.
//! - Partition computational graphs.
//! - Implement tensor storage.
//! - Execute AI operations.
//! - Define AI semantic correctness.
//!
//! Target-specific cost coefficients must be supplied by the appropriate
//! downstream target or execution analysis.
//!
//! ## POCO-REAF
//!
//! The implementation has no fixed workload, tensor-rank, model-size,
//! parameter-count, accelerator-count, or worker-count ceilings.
//!
//! Cost dimensions are extensible through MetricId.
//! Operational limits belong to separately supplied execution policies.
//!
//! ## Numerical policy
//!
//! All externally supplied floating-point values are validated.
//! NaN, infinity, negative values, and arithmetic overflow are rejected.
//! Estimates are not silently saturated or converted into successful
//! zero-cost results.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// Identifies a cost dimension.
///
/// The identifier is open-ended. New computational cost dimensions
/// can be introduced without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricId(String);

impl MetricId {
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CostModelError::InvalidMetricId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Standard metric identifiers.
///
/// These are conventions, not an exhaustive enumeration.
/// Custom metrics remain supported.
pub mod metrics {
    pub const COMPUTE_OPERATIONS: &str = "compute.operations";
    pub const MEMORY_BYTES: &str = "memory.bytes";
    pub const MEMORY_READ_BYTES: &str = "memory.read_bytes";
    pub const MEMORY_WRITE_BYTES: &str = "memory.write_bytes";
    pub const NETWORK_BYTES: &str = "network.bytes";
    pub const NETWORK_MESSAGES: &str = "network.messages";
    pub const SYNCHRONIZATION_POINTS: &str = "synchronization.points";
    pub const ENERGY_JOULES: &str = "energy.joules";
    pub const LATENCY_SECONDS: &str = "latency.seconds";
    pub const STORAGE_BYTES: &str = "storage.bytes";
    pub const FINANCIAL_COST: &str = "financial.cost";
}

/// A non-negative, finite quantity.
///
/// Quantities are kept separate from their units and cost coefficients.
/// This avoids silently combining incompatible units.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity(f64);

impl Quantity {
    pub fn new(value: f64) -> Result<Self, CostModelError> {
        if !value.is_finite() {
            return Err(CostModelError::NonFiniteValue);
        }

        if value < 0.0 {
            return Err(CostModelError::NegativeValue);
        }

        Ok(Self(value))
    }

    pub fn zero() -> Self {
        Self(0.0)
    }

    pub fn get(self) -> f64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Result<Self, CostModelError> {
        Self::new(self.0 + other.0)
    }

    fn checked_mul(self, other: Self) -> Result<Self, CostModelError> {
        Self::new(self.0 * other.0)
    }
}

/// Describes an abstract computational workload.
///
/// This is a collection of logical workload quantities.
/// It does not describe physical device allocation.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Workload {
    quantities: BTreeMap<MetricId, Quantity>,
}

impl Workload {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn set(
        &mut self,
        metric: MetricId,
        quantity: Quantity,
    ) {
        self.quantities.insert(metric, quantity);
    }

    pub fn get(&self, metric: &MetricId) -> Option<Quantity> {
        self.quantities.get(metric).copied()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &Quantity)> {
        self.quantities.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.quantities.is_empty()
    }
}

/// A cost coefficient.
///
/// Represents the estimated cost associated with one unit
/// of a particular workload metric.
///
/// The unit and interpretation are defined by the profile
/// supplying the coefficient.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostCoefficient {
    cost_per_unit: Quantity,
    standard_deviation: Quantity,
}

impl CostCoefficient {
    pub fn new(
        cost_per_unit: Quantity,
        standard_deviation: Quantity,
    ) -> Self {
        Self {
            cost_per_unit,
            standard_deviation,
        }
    }

    pub fn cost_per_unit(&self) -> Quantity {
        self.cost_per_unit
    }

    pub fn standard_deviation(&self) -> Quantity {
        self.standard_deviation
    }
}

/// An externally supplied cost profile.
///
/// A profile may represent:
///
/// - An abstract execution realization.
/// - A CPU execution cost model.
/// - A GPU execution cost model.
/// - A distributed execution cost model.
/// - A quantum execution cost model.
/// - A hybrid execution cost model.
///
/// This module does not determine which profile should be selected.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostProfile {
    coefficients: BTreeMap<MetricId, CostCoefficient>,
    version: String,
}

impl CostProfile {
    pub fn new(
        version: impl Into<String>,
    ) -> Result<Self, CostModelError> {
        let version = version.into();

        if version.trim().is_empty() {
            return Err(CostModelError::InvalidProfileVersion);
        }

        Ok(Self {
            coefficients: BTreeMap::new(),
            version,
        })
    }

    pub fn version(&self) -> &str {
        &self.version
    }

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

    pub fn coefficient(
        &self,
        metric: &MetricId,
    ) -> Option<&CostCoefficient> {
        self.coefficients.get(metric)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &CostCoefficient)> {
        self.coefficients.iter()
    }
}

/// Describes how a metric contributes to an optimization objective.
///
/// Objective weights are supplied by the caller.
/// There are no implicit universal optimization priorities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveWeight {
    weight: Quantity,
}

impl ObjectiveWeight {
    pub fn new(weight: Quantity) -> Self {
        Self { weight }
    }

    pub fn get(self) -> Quantity {
        self.weight
    }
}

/// An explicitly supplied optimization objective.
///
/// Scalar scoring is optional because different objectives may
/// have incompatible units and should not automatically be combined.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Objective {
    weights: BTreeMap<MetricId, ObjectiveWeight>,
}

impl Objective {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_weight(
        &mut self,
        metric: MetricId,
        weight: ObjectiveWeight,
    ) {
        self.weights.insert(metric, weight);
    }

    pub fn weight(
        &self,
        metric: &MetricId,
    ) -> Option<ObjectiveWeight> {
        self.weights.get(metric).copied()
    }
}

/// Individual estimated cost contribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EstimatedCost {
    pub expected: Quantity,
    pub standard_deviation: Quantity,
}

/// Multidimensional cost estimate.
///
/// The individual metrics remain available rather than being
/// prematurely reduced to one scalar.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostEstimate {
    values: BTreeMap<MetricId, EstimatedCost>,
    profile_version: String,
}

impl CostEstimate {
    pub fn values(
        &self,
    ) -> &BTreeMap<MetricId, EstimatedCost> {
        &self.values
    }

    pub fn get(
        &self,
        metric: &MetricId,
    ) -> Option<&EstimatedCost> {
        self.values.get(metric)
    }

    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }

    /// Calculates an optional weighted score.
    ///
    /// The caller is responsible for ensuring that all objective
    /// dimensions have meaningful compatible units.
    ///
    /// Missing objective dimensions are errors rather than
    /// silently being treated as zero.
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

/// Abstract cost-model interface.
///
/// Implementations can provide analytical, empirical, learned,
/// simulated, or composite cost estimation.
pub trait CostModel {
    fn estimate(
        &self,
        workload: &Workload,
        profile: &CostProfile,
    ) -> Result<CostEstimate, CostModelError>;
}

/// Analytical cost model.
///
/// Uses externally supplied coefficients and logical workload
/// quantities. It does not assume any particular hardware.
#[derive(Debug, Clone, Default)]
pub struct AnalyticalCostModel;

impl AnalyticalCostModel {
    pub fn new() -> Self {
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

            let expected = quantity.checked_mul(
                coefficient.cost_per_unit(),
            )?;

            let standard_deviation = quantity.checked_mul(
                coefficient.standard_deviation(),
            )?;

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

/// Describes why an estimate could not be produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostModelError {
    InvalidMetricId,
    InvalidProfileVersion,
    NegativeValue,
    NonFiniteValue,
    DuplicateMetric(String),
    MissingCoefficient(String),
    MissingEstimatedMetric(String),
    InvalidModelConfiguration(String),
    ArithmeticOverflow,
}

impl fmt::Display for CostModelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidMetricId => {
                write!(formatter, "Cost metric identifier is empty")
            }
            Self::InvalidProfileVersion => {
                write!(formatter, "Cost profile version is empty")
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
                    "Missing cost coefficient for metric: {metric}"
                )
            }
            Self::MissingEstimatedMetric(metric) => {
                write!(
                    formatter,
                    "Objective references an unestimated metric: {metric}"
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

/// Builder for an analytical cost profile.
#[derive(Debug, Clone)]
pub struct CostProfileBuilder {
    profile: CostProfile,
}

impl CostProfileBuilder {
    pub fn new(
        version: impl Into<String>,
    ) -> Result<Self, CostModelError> {
        Ok(Self {
            profile: CostProfile::new(version)?,
        })
    }

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

    pub fn build(self) -> CostProfile {
        self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(name: &str) -> MetricId {
        MetricId::new(name).expect("valid metric")
    }

    #[test]
    fn estimates_multiple_cost_dimensions() {
        let mut workload = Workload::new();

        workload.insert(
            metric(metrics::COMPUTE_OPERATIONS),
            Quantity::new(100.0).unwrap(),
        ).unwrap();

        workload.insert(
            metric(metrics::MEMORY_BYTES),
            Quantity::new(2048.0).unwrap(),
        ).unwrap();

        let profile = CostProfileBuilder::new("test-v1")
            .unwrap()
            .coefficient(
                metrics::COMPUTE_OPERATIONS,
                0.5,
                0.1,
            )
            .unwrap()
            .coefficient(
                metrics::MEMORY_BYTES,
                0.01,
                0.002,
            )
            .unwrap()
            .build();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        assert_eq!(
            estimate.get(&metric(metrics::COMPUTE_OPERATIONS))
                .unwrap().expected.get(),
            50.0,
        );

        assert_eq!(
            estimate.get(&metric(metrics::MEMORY_BYTES))
                .unwrap().expected.get(),
            20.48,
        );
    }

    #[test]
    fn supports_extension_metrics() {
        let mut workload = Workload::new();

        workload.insert(
            metric("custom.quantum.entanglement"),
            Quantity::new(12.0).unwrap(),
        ).unwrap();

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
            estimate.get(
                &metric("custom.quantum.entanglement")
            ).unwrap().expected.get(),
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
    fn detects_missing_coefficients() {
        let mut workload = Workload::new();

        workload.insert(
            metric("unknown.metric"),
            Quantity::new(1.0).unwrap(),
        ).unwrap();

        let profile = CostProfile::new("v1").unwrap();

        let result = AnalyticalCostModel
            .estimate(&workload, &profile);

        assert_eq!(
            result,
            Err(CostModelError::MissingCoefficient(
                "unknown.metric".to_owned()
            )),
        );
    }

    #[test]
    fn rejects_duplicate_workload_metrics() {
        let mut workload = Workload::new();

        workload.insert(
            metric("compute"),
            Quantity::new(1.0).unwrap(),
        ).unwrap();

        assert!(matches!(
            workload.insert(
                metric("compute"),
                Quantity::new(2.0).unwrap(),
            ),
            Err(CostModelError::DuplicateMetric(_))
        ));
    }

    #[test]
    fn weighted_objective_preserves_explicit_priorities() {
        let mut workload = Workload::new();

        workload.insert(
            metric(metrics::COMPUTE_OPERATIONS),
            Quantity::new(10.0).unwrap(),
        ).unwrap();

        let profile = CostProfileBuilder::new("v1")
            .unwrap()
            .coefficient(
                metrics::COMPUTE_OPERATIONS,
                2.0,
                0.0,
            )
            .unwrap()
            .build();

        let estimate = AnalyticalCostModel
            .estimate(&workload, &profile)
            .unwrap();

        let mut objective = Objective::new();

        objective.set_weight(
            metric(metrics::COMPUTE_OPERATIONS),
            ObjectiveWeight::new(
                Quantity::new(3.0).unwrap(),
            ),
        );

        assert_eq!(
            estimate.score(&objective).unwrap().get(),
            60.0,
        );
    }

    #[test]
    fn detects_arithmetic_overflow() {
        let result = Quantity::new(f64::MAX)
            .unwrap()
            .checked_mul(
                Quantity::new(2.0).unwrap(),
            );

        assert_eq!(
            result,
            Err(CostModelError::NonFiniteValue),
        );
    }

    #[test]
    fn empty_workload_has_zero_cost() {
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
}
