
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Cost Model
//!
//! Path: src/compiler/optimization/cost_model.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide target-independent, extensible, multidimensional analytical
//! cost estimation for compiler optimization.
//!
//! ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//! - Extensible cost metric identities.
//! - Workload descriptions.
//! - Cost units and quantities.
//! - Cost coefficients.
//! - Cost profiles.
//! - Optimization objective weights.
//! - Multidimensional cost estimates.
//! - Analytical estimation.
//! - Numerical validation.
//! - Cost-estimation errors.
//!
//! This module does not own:
//! - Program semantics.
//! - IR definitions.
//! - Optimization transformations.
//! - Pass registration.
//! - Pass execution.
//! - Hardware discovery.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling.
//! - Physical routing.
//! - Runtime execution.
//!
//! INTEGRATION
//!
//! compiler::optimization::planning
//!     consumes estimates through an explicit adapter.
//!
//! ai::optimization::cost_model
//!     retains AI-specific estimation functionality.
//!
//! Hardware and runtime subsystems
//!     supply externally measured or estimated coefficients.
//!
//! SCALABILITY
//!
//! No hardware population, workload size, tensor rank,
//! operation count, accelerator count, or domain count is
//! imposed by this module.
//!
//! Real memory, arithmetic representation, configured budgets,
//! and available processing resources remain applicable.
//!
//! NUMERICAL CONTRACT
//!
//! All numerical inputs must be finite and non-negative.
//! Overflow must be reported explicitly.
//! Missing coefficients must never silently become zero.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifies an extensible cost metric.
///
/// Examples:
/// - compute.operations
/// - memory.read_bytes
/// - memory.write_bytes
/// - network.bytes
/// - energy.joules
/// - execution.latency
/// - quantum.gate_operations
/// - tensor.operations
///
/// Custom future metrics are supported without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricId(String);

impl MetricId {
    /// Creates a validated metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        validate_identifier(&value, "metric")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifies a cost unit.
///
/// Examples include seconds, joules, bytes, operations,
/// messages, and profile-defined financial units.
///
/// Units are identifiers, not an exhaustive enumeration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId(String);

impl UnitId {
    /// Creates a validated unit identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        validate_identifier(&value, "unit")?;

        Ok(Self(value))
    }

    /// Returns the unit identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identifies a cost model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModelId(String);

impl ModelId {
    /// Creates a model identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        validate_identifier(&value, "model")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identifies a cost profile.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProfileId(String);

impl ProfileId {
    /// Creates a profile identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostModelError> {
        let value = value.into();

        validate_identifier(&value, "profile")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_identifier(
    value: &str,
    kind: &str,
) -> Result<(), CostModelError> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(CostModelError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// STANDARD METRIC CONVENTIONS
// ============================================================================

/// Common cost metric identifiers.
///
/// These are conventions rather than a closed registry.
/// Additional metrics can be created through MetricId.
pub mod metrics {
    /// Abstract computation operations.
    pub const COMPUTE_OPERATIONS: &str = "compute.operations";

    /// Memory footprint.
    pub const MEMORY_BYTES: &str = "memory.bytes";

    /// Memory reads.
    pub const MEMORY_READ_BYTES: &str = "memory.read_bytes";

    /// Memory writes.
    pub const MEMORY_WRITE_BYTES: &str = "memory.write_bytes";

    /// Network transfer volume.
    pub const NETWORK_BYTES: &str = "network.bytes";

    /// Network message count.
    pub const NETWORK_MESSAGES: &str = "network.messages";

    /// Synchronization operations.
    pub const SYNCHRONIZATION_POINTS: &str =
        "synchronization.points";

    /// Energy consumption.
    pub const ENERGY_JOULES: &str = "energy.joules";

    /// Execution latency.
    pub const LATENCY_SECONDS: &str = "latency.seconds";

    /// Storage consumption.
    pub const STORAGE_BYTES: &str = "storage.bytes";

    /// Financial expenditure.
    pub const FINANCIAL_COST: &str = "financial.cost";

    /// Quantum operation count.
    pub const QUANTUM_OPERATIONS: &str = "quantum.operations";

    /// Tensor operation count.
    pub const TENSOR_OPERATIONS: &str = "tensor.operations";

    /// Communication volume.
    pub const COMMUNICATION_BYTES: &str = "communication.bytes";
}

// ============================================================================
// NUMERICAL QUANTITIES
// ============================================================================

/// A finite, non-negative numerical quantity.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity(f64);

impl Quantity {
    /// Constructs a validated quantity.
    pub fn new(value: f64) -> Result<Self, CostModelError> {
        if !value.is_finite() {
            return Err(CostModelError::NonFiniteValue);
        }

        if value < 0.0 {
            return Err(CostModelError::NegativeValue);
        }

        Ok(Self(value))
    }

    /// Returns zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Performs checked addition.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, CostModelError> {
        Self::new(
            self.0
                .checked_add(other.0)
                .ok_or(CostModelError::ArithmeticOverflow)?,
        )
    }

    /// Performs checked multiplication.
    pub fn checked_mul(
        self,
        other: Self,
    ) -> Result<Self, CostModelError> {
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
/// Workloads contain logical quantities, not physical allocations.
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

    /// Inserts a new metric.
    ///
    /// Duplicate insertion is rejected.
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

    /// Inserts or replaces a metric.
    pub fn set(
        &mut self,
        metric: MetricId,
        quantity: Quantity,
    ) {
        self.quantities.insert(metric, quantity);
    }

    /// Retrieves a metric.
    #[must_use]
    pub fn get(&self, metric: &MetricId) -> Option<Quantity> {
        self.quantities.get(metric).copied()
    }

    /// Iterates in deterministic metric order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &Quantity)> {
        self.quantities.iter()
    }

    /// Returns whether the workload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.quantities.is_empty()
    }

    /// Returns the number of supplied metrics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.quantities.len()
    }
}

// ============================================================================
// COST COEFFICIENTS
// ============================================================================

/// Cost coefficient for one workload metric.
///
/// The coefficient's unit describes the cost produced per workload unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostCoefficient {
    cost_per_unit: Quantity,
    uncertainty_per_unit: Quantity,
}

impl CostCoefficient {
    /// Creates a coefficient.
    #[must_use]
    pub const fn new(
        cost_per_unit: Quantity,
        uncertainty_per_unit: Quantity,
    ) -> Self {
        Self {
            cost_per_unit,
            uncertainty_per_unit,
        }
    }

    /// Returns cost per workload unit.
    #[must_use]
    pub const fn cost_per_unit(self) -> Quantity {
        self.cost_per_unit
    }

    /// Returns uncertainty per workload unit.
    #[must_use]
    pub const fn uncertainty_per_unit(self) -> Quantity {
        self.uncertainty_per_unit
    }
}

// ============================================================================
// COST PROFILES
// ============================================================================

/// A cost profile supplied by an external subsystem.
///
/// Profiles may describe abstract, classical, quantum, AI, tensor,
/// hardware, distributed, or future computational environments.
///
/// The profile is evidence for estimation, not a hardware selector.
#[derive(Debug, Clone, PartialEq)]
pub struct CostProfile {
    id: ProfileId,
    version: String,
    coefficients: BTreeMap<MetricId, CostCoefficient>,
    units: BTreeMap<MetricId, UnitId>,
}

impl CostProfile {
    /// Creates a profile.
    pub fn new(
        id: ProfileId,
        version: impl Into<String>,
    ) -> Result<Self, CostModelError> {
        let version = version.into();

        validate_identifier(&version, "profile version")?;

        Ok(Self {
            id,
            version,
            coefficients: BTreeMap::new(),
            units: BTreeMap::new(),
        })
    }

    /// Returns the profile identity.
    #[must_use]
    pub fn id(&self) -> &ProfileId {
        &self.id
    }

    /// Returns the profile version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Registers a metric coefficient and its cost unit.
    pub fn register(
        &mut self,
        metric: MetricId,
        unit: UnitId,
        coefficient: CostCoefficient,
    ) -> Result<(), CostModelError> {
        if self.coefficients.contains_key(&metric) {
            return Err(CostModelError::DuplicateMetric(
                metric.as_str().to_owned(),
            ));
        }

        self.coefficients.insert(
            metric.clone(),
            coefficient,
        );

        self.units.insert(metric, unit);

        Ok(())
    }

    /// Retrieves a coefficient.
    #[must_use]
    pub fn coefficient(
        &self,
        metric: &MetricId,
    ) -> Option<&CostCoefficient> {
        self.coefficients.get(metric)
    }

    /// Retrieves a metric's cost unit.
    #[must_use]
    pub fn unit(
        &self,
        metric: &MetricId,
    ) -> Option<&UnitId> {
        self.units.get(metric)
    }

    /// Iterates over registered coefficients.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &CostCoefficient)> {
        self.coefficients.iter()
    }
}

// ============================================================================
// OBJECTIVES
// ============================================================================

/// Explicit weight assigned to one cost metric.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveWeight(Quantity);

impl ObjectiveWeight {
    /// Creates an objective weight.
    #[must_use]
    pub const fn new(weight: Quantity) -> Self {
        Self(weight)
    }

    /// Returns the weight.
    #[must_use]
    pub const fn get(self) -> Quantity {
        self.0
    }
}

/// A caller-defined optimization objective.
///
/// The caller is responsible for ensuring that weighted units
/// are compatible and meaningful.
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

    /// Sets a metric weight.
    pub fn set_weight(
        &mut self,
        metric: MetricId,
        weight: ObjectiveWeight,
    ) {
        self.weights.insert(metric, weight);
    }

    /// Retrieves a metric weight.
    #[must_use]
    pub fn weight(
        &self,
        metric: &MetricId,
    ) -> Option<ObjectiveWeight> {
        self.weights.get(metric).copied()
    }

    /// Iterates over objective weights.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetricId, &ObjectiveWeight)> {
        self.weights.iter()
    }

    /// Returns whether the objective has no weights.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.weights.is_empty()
    }
}

// ============================================================================
// ESTIMATES
// ============================================================================

/// Estimate for one cost dimension.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EstimatedCost {
    /// Expected cost.
    pub expected: Quantity,

    /// Estimated uncertainty.
    pub uncertainty: Quantity,
}

/// Multidimensional estimate.
///
/// Metrics remain independent until an explicitly supplied objective
/// requests a weighted score.
#[derive(Debug, Clone, PartialEq)]
pub struct CostEstimate {
    profile_id: ProfileId,
    profile_version: String,
    values: BTreeMap<MetricId, EstimatedCost>,
    units: BTreeMap<MetricId, UnitId>,
}

impl CostEstimate {
    /// Returns the profile identity.
    #[must_use]
    pub fn profile_id(&self) -> &ProfileId {
        &self.profile_id
    }

    /// Returns the profile version.
    #[must_use]
    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }

    /// Returns all estimated dimensions.
    #[must_use]
    pub fn values(&self) -> &BTreeMap<MetricId, EstimatedCost> {
        &self.values
    }

    /// Retrieves an estimate.
    #[must_use]
    pub fn get(
        &self,
        metric: &MetricId,
    ) -> Option<&EstimatedCost> {
        self.values.get(metric)
    }

    /// Retrieves a cost unit.
    #[must_use]
    pub fn unit(
        &self,
        metric: &MetricId,
    ) -> Option<&UnitId> {
        self.units.get(metric)
    }

    /// Calculates an explicitly weighted score.
    ///
    /// Missing metrics are errors.
    /// An empty objective is rejected rather than interpreted as
    /// a meaningful zero-cost optimization.
    pub fn score(
        &self,
        objective: &Objective,
    ) -> Result<Quantity, CostModelError> {
        if objective.is_empty() {
            return Err(CostModelError::EmptyObjective);
        }

        let mut total = Quantity::zero();

        for (metric, weight) in objective.iter() {
            let estimate = self.values.get(metric).ok_or_else(|| {
                CostModelError::MissingEstimatedMetric(
                    metric.as_str().to_owned(),
                )
            })?;

            let contribution = estimate.expected.checked_mul(
                weight.get(),
            )?;

            total = total.checked_add(contribution)?;
        }

        Ok(total)
    }
}

// ============================================================================
// COST MODEL INTERFACE
// ============================================================================

/// General cost-model interface.
///
/// Analytical, empirical, simulated, learned and composite
/// implementations can implement this contract.
pub trait CostModel {
    /// Returns the model identity.
    fn model_id(&self) -> &ModelId;

    /// Estimates a workload using an externally supplied profile.
    fn estimate(
        &self,
        workload: &Workload,
        profile: &CostProfile,
    ) -> Result<CostEstimate, CostModelError>;
}

// ============================================================================
// ANALYTICAL MODEL
// ============================================================================

/// Target-independent analytical cost model.
///
/// Uses workload quantities and externally supplied coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyticalCostModel {
    id: ModelId,
}

impl AnalyticalCostModel {
    /// Creates the analytical model.
    pub fn new(
        id: ModelId,
    ) -> Self {
        Self { id }
    }
}

impl CostModel for AnalyticalCostModel {
    fn model_id(&self) -> &ModelId {
        &self.id
    }

    fn estimate(
        &self,
        workload: &Workload,
        profile: &CostProfile,
    ) -> Result<CostEstimate, CostModelError> {
        let mut values = BTreeMap::new();
        let mut units = BTreeMap::new();

        for (metric, quantity) in workload.iter() {
            let coefficient = profile.coefficient(metric).ok_or_else(
                || {
                    CostModelError::MissingCoefficient(
                        metric.as_str().to_owned(),
                    )
                },
            )?;

            let unit = profile.unit(metric).ok_or_else(|| {
                CostModelError::MissingUnit(
                    metric.as_str().to_owned(),
                )
            })?;

            let expected = quantity.checked_mul(
                coefficient.cost_per_unit(),
            )?;

            let uncertainty = quantity.checked_mul(
                coefficient.uncertainty_per_unit(),
            )?;

            values.insert(
                metric.clone(),
                EstimatedCost {
                    expected,
                    uncertainty,
                },
            );

            units.insert(metric.clone(), unit.clone());
        }

        Ok(CostEstimate {
            profile_id: profile.id().clone(),
            profile_version: profile.version().to_owned(),
            values,
            units,
        })
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Cost estimation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CostModelError {
    /// An identifier is invalid.
    InvalidIdentifier {
        /// Identifier category.
        kind: String,
    },

    /// A negative numerical value was supplied.
    NegativeValue,

    /// A non-finite numerical value was supplied.
    NonFiniteValue,

    /// A metric was registered more than once.
    DuplicateMetric(String),

    /// A workload metric has no cost coefficient.
    MissingCoefficient(String),

    /// A workload metric has no unit.
    MissingUnit(String),

    /// An objective references an absent estimate.
    MissingEstimatedMetric(String),

    /// An objective has no dimensions.
    EmptyObjective,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow,
}

impl fmt::Display for CostModelError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind } => {
                write!(f, "Invalid {kind} identifier")
            }

            Self::NegativeValue => {
                write!(f, "Negative numerical value")
            }

            Self::NonFiniteValue => {
                write!(f, "Non-finite numerical value")
            }

            Self::DuplicateMetric(metric) => {
                write!(f, "Duplicate metric: {metric}")
            }

            Self::MissingCoefficient(metric) => {
                write!(f, "Missing cost coefficient: {metric}")
            }

            Self::MissingUnit(metric) => {
                write!(f, "Missing cost unit: {metric}")
            }

            Self::MissingEstimatedMetric(metric) => {
                write!(f, "Missing estimated metric: {metric}")
            }

            Self::EmptyObjective => {
                write!(f, "Optimization objective cannot be empty")
            }

            Self::ArithmeticOverflow => {
                write!(f, "Cost calculation overflow")
            }
        }
    }
}

impl Error for CostModelError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(name: &str) -> MetricId {
        MetricId::new(name).unwrap()
    }

    fn unit(name: &str) -> UnitId {
        UnitId::new(name).unwrap()
    }

    fn quantity(value: f64) -> Quantity {
        Quantity::new(value).unwrap()
    }

    fn profile() -> CostProfile {
        let mut profile = CostProfile::new(
            ProfileId::new("test.profile").unwrap(),
            "1",
        )
        .unwrap();

        profile.register(
            metric(metrics::COMPUTE_OPERATIONS),
            unit("operations"),
            CostCoefficient::new(
                quantity(2.0),
                quantity(0.1),
            ),
        ).unwrap();

        profile
    }

    #[test]
    fn accepts_zero_quantity() {
        assert_eq!(quantity(0.0).get(), 0.0);
    }

    #[test]
    fn rejects_negative_quantity() {
        assert!(matches!(
            Quantity::new(-1.0),
            Err(CostModelError::NegativeValue)
        ));
    }

    #[test]
    fn rejects_nan() {
        assert!(matches!(
            Quantity::new(f64::NAN),
            Err(CostModelError::NonFiniteValue)
        ));
    }

    #[test]
    fn rejects_infinity() {
        assert!(Quantity::new(f64::INFINITY).is_err());
    }

    #[test]
    fn rejects_invalid_identifier() {
        assert!(MetricId::new(" ").is_err());
        assert!(MetricId::new("bad\nmetric").is_err());
        assert!(MetricId::new(" metric").is_err());
    }

    #[test]
    fn workload_rejects_duplicates() {
        let mut workload = Workload::new();

        workload.insert(
            metric("compute.operations"),
            quantity(10.0),
        ).unwrap();

        assert!(workload.insert(
            metric("compute.operations"),
            quantity(20.0),
        ).is_err());
    }

    #[test]
    fn analytical_estimation_works() {
        let mut workload = Workload::new();

        workload.insert(
            metric(metrics::COMPUTE_OPERATIONS),
            quantity(100.0),
        ).unwrap();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        let estimate = model.estimate(
            &workload,
            &profile(),
        ).unwrap();

        let result = estimate.get(
            &metric(metrics::COMPUTE_OPERATIONS),
        ).unwrap();

        assert_eq!(result.expected.get(), 200.0);
        assert_eq!(result.uncertainty.get(), 10.0);
    }

    #[test]
    fn missing_coefficient_is_not_zero() {
        let mut workload = Workload::new();

        workload.insert(
            metric("future.custom.metric"),
            quantity(100.0),
        ).unwrap();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        assert!(matches!(
            model.estimate(&workload, &profile()),
            Err(CostModelError::MissingCoefficient(_))
        ));
    }

    #[test]
    fn empty_workload_is_supported() {
        let workload = Workload::new();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        let estimate = model.estimate(
            &workload,
            &profile(),
        ).unwrap();

        assert!(estimate.values().is_empty());
    }

    #[test]
    fn custom_future_metrics_are_supported() {
        let mut profile = profile();

        profile.register(
            metric("future.photonic.operations"),
            unit("operations"),
            CostCoefficient::new(
                quantity(0.5),
                quantity(0.02),
            ),
        ).unwrap();

        let mut workload = Workload::new();

        workload.insert(
            metric("future.photonic.operations"),
            quantity(1000.0),
        ).unwrap();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        let estimate = model.estimate(
            &workload,
            &profile,
        ).unwrap();

        assert_eq!(
            estimate.get(
                &metric("future.photonic.operations")
            ).unwrap().expected.get(),
            500.0
        );
    }

    #[test]
    fn objective_scoring_is_explicit() {
        let mut workload = Workload::new();

        workload.insert(
            metric(metrics::COMPUTE_OPERATIONS),
            quantity(100.0),
        ).unwrap();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        let estimate = model.estimate(
            &workload,
            &profile(),
        ).unwrap();

        let mut objective = Objective::new();

        objective.set_weight(
            metric(metrics::COMPUTE_OPERATIONS),
            ObjectiveWeight::new(quantity(1.0)),
        );

        assert_eq!(
            estimate.score(&objective).unwrap().get(),
            200.0
        );
    }

    #[test]
    fn empty_objective_is_rejected() {
        let mut workload = Workload::new();

        workload.insert(
            metric(metrics::COMPUTE_OPERATIONS),
            quantity(1.0),
        ).unwrap();

        let model = AnalyticalCostModel::new(
            ModelId::new("analytical").unwrap(),
        );

        let estimate = model.estimate(
            &workload,
            &profile(),
        ).unwrap();

        assert!(matches!(
            estimate.score(&Objective::new()),
            Err(CostModelError::EmptyObjective)
        ));
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let result = quantity(f64::MAX).checked_mul(
            quantity(2.0),
        );

        assert!(matches!(
            result,
            Err(CostModelError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn deterministic_iteration_is_preserved() {
        let mut first = Workload::new();

        first.set(metric("z.metric"), quantity(1.0));
        first.set(metric("a.metric"), quantity(2.0));

        let mut second = Workload::new();

        second.set(metric("a.metric"), quantity(2.0));
        second.set(metric("z.metric"), quantity(1.0));

        assert_eq!(first, second);

        assert_eq!(
            first.iter().next().unwrap().0.as_str(),
            "a.metric"
        );
    }
}
