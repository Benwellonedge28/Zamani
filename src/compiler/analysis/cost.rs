//! Cost estimates for compiler analysis and realization planning.
//!
//! This module provides a target-independent, extensible cost vocabulary.
//! It does not:
//! - select a target or backend;
//! - allocate physical resources;
//! - schedule runtime work;
//! - determine feasibility;
//! - own canonical IR or semantic information.
//!
//! Callers provide estimates from canonical program analysis, target
//! capability providers, or registered cost-model implementations.
//!
//! # Numeric representation
//!
//! Integer quantities use `u128` to avoid common narrow-counter limits.
//! This is a representation limit, not a hardware or program-size limit.
//! Arithmetic overflow is reported as `CostError::ArithmeticOverflow`.
//! Floating-point values are not used for exact resource quantities.
//!
//! # Extensibility
//!
//! `CostMetric` includes an extensible namespaced metric. New compiler
//! domains can add metrics without changing this module's enum variants.
//! Existing metrics retain their meaning across compiler stages.

use std::collections::BTreeMap;
use std::fmt;

/// Stable identifier for a cost metric.
///
/// Names are namespaced strings, for example:
/// - `compiler.compile_time`
— - `resource.memory_bytes`
/// - `execution.latency_ns`
/// - `communication.transfer_bytes`
/// - `quantum.estimated_error`
///
/// The identifier is deliberately not a closed enum so new domains can
/// introduce metrics without modifying this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MetricId(String);

impl MetricId {
    /// Construct a metric identifier.
    ///
    /// Empty names and names containing only whitespace are rejected.
    /// No arbitrary maximum name length is imposed here.
    pub fn new(name: impl Into<String>) -> Result<Self, CostError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(CostError::InvalidMetricId);
        }

        Ok(Self(name))
    }

    /// Return the metric's canonical name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Standard cost metrics used across compiler domains.
///
/// Domain-specific metrics should use [`CostMetric::Custom`] rather than
/// adding a new variant for every backend or computing architecture.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CostMetric {
    /// Estimated compiler work, measured in an implementation-defined unit.
    CompileWork,

    /// Estimated compilation wall-clock duration, in nanoseconds.
    CompileTimeNs,

    /// Estimated execution latency, in nanoseconds.
    ExecutionLatencyNs,

    /// Estimated throughput, in operations per second.
    ThroughputPerSecond,

    /// Estimated energy, in microjoules.
    EnergyMicrojoules,

    /// Estimated memory consumption, in bytes.
    MemoryBytes,

    /// Estimated storage consumption, in bytes.
    StorageBytes,

    /// Estimated communication volume, in bytes.
    CommunicationBytes,

    /// Estimated number of operations.
    OperationCount,

    /// Estimated number of synchronization events.
    SynchronizationCount,

    /// Domain-specific metric.
    Custom(MetricId),
}

impl CostMetric {
    /// Return the metric's stable textual name.
    pub fn name(&self) -> String {
        match self {
            Self::CompileWork => "compiler.compile_work".to_owned(),
            Self::CompileTimeNs => "compiler.compile_time_ns".to_owned(),
            Self::ExecutionLatencyNs => "execution.latency_ns".to_owned(),
            Self::ThroughputPerSecond => "execution.throughput_per_second".to_owned(),
            Self::EnergyMicrojoules => "execution.energy_microjoules".to_owned(),
            Self::MemoryBytes => "resource.memory_bytes".to_owned(),
            Self::StorageBytes => "resource.storage_bytes".to_owned(),
            Self::CommunicationBytes => "communication.bytes".to_owned(),
            Self::OperationCount => "program.operation_count".to_owned(),
            Self::SynchronizationCount => {
                "execution.synchronization_count".to_owned()
            }
            Self::Custom(id) => id.as_str().to_owned(),
        }
    }
}

/// A non-negative, exact quantity.
///
/// Units are defined by the associated [`CostMetric`]. This type does not
/// assume that every metric is additive or comparable with every other one.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CostQuantity(u128);

impl CostQuantity {
    /// Construct a quantity from its exact integer representation.
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Return the exact integer representation.
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Add two quantities with overflow checking.
    pub fn checked_add(self, rhs: Self) -> Result<Self, CostError> {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or(CostError::ArithmeticOverflow)
    }

    /// Multiply two quantities with overflow checking.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, CostError> {
        self.0
            .checked_mul(rhs.0)
            .map(Self)
            .ok_or(CostError::ArithmeticOverflow)
    }

    /// Return the maximum of two quantities.
    pub const fn max(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            self
        } else {
            rhs
        }
    }

    /// Return the minimum of two quantities.
    pub const fn min(self, rhs: Self) -> Self {
        if self.0 <= rhs.0 {
            self
        } else {
            rhs
        }
    }
}

impl From<u64> for CostQuantity {
    fn from(value: u64) -> Self {
        Self(u128::from(value))
    }
}

impl From<u128> for CostQuantity {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl fmt::Display for CostQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Whether a cost value is exact, estimated, or not currently known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EstimateQuality {
    /// Derived from exact analysis or an exact measurement.
    Exact,

    /// Derived from a model, heuristic, or measurement extrapolation.
    Estimated,

    /// A value is not currently available.
    Unknown,
}

/// Bound on a non-negative cost quantity.
///
/// `Unknown` is not zero. `Unbounded` means the model cannot provide a
/// finite upper bound. These states prevent missing data from being
/// silently treated as free work or unlimited capacity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CostBound {
    /// The exact value is known.
    Exact(CostQuantity),

    /// A lower bound is known, but the exact value is not.
    AtLeast(CostQuantity),

    /// A finite upper bound is known, but the exact value is not.
    AtMost(CostQuantity),

    /// Both lower and upper bounds are known.
    Range {
        lower: CostQuantity,
        upper: CostQuantity,
    },

    /// The cost is unknown.
    Unknown,

    /// The cost has no finite upper bound in this model.
    Unbounded,
}

impl CostBound {
    /// Construct a validated range.
    pub fn range(
        lower: CostQuantity,
        upper: CostQuantity,
    ) -> Result<Self, CostError> {
        if lower > upper {
            return Err(CostError::InvalidRange);
        }

        Ok(Self::Range { lower, upper })
    }

    /// Return the exact value, if this bound is exact.
    pub const fn exact_value(self) -> Option<CostQuantity> {
        match self {
            Self::Exact(value) => Some(value),
            _ => None,
        }
    }

    /// Return a known lower bound, if available.
    pub const fn lower_bound(self) -> Option<CostQuantity> {
        match self {
            Self::Exact(value) | Self::AtLeast(value) => Some(value),
            Self::Range { lower, .. } => Some(lower),
            Self::AtMost(_) | Self::Unknown | Self::Unbounded => None,
        }
    }

    /// Return a known finite upper bound, if available.
    pub const fn upper_bound(self) -> Option<CostQuantity> {
        match self {
            Self::Exact(value) | Self::AtMost(value) => Some(value),
            Self::Range { upper, .. } => Some(upper),
            Self::AtLeast(_) | Self::Unknown | Self::Unbounded => None,
        }
    }

    /// Add two bounds conservatively.
    ///
    /// The method never converts unknown or unbounded quantities into
    /// exact values. If finite arithmetic overflows, it returns an error.
    pub fn checked_add(self, rhs: Self) -> Result<Self, CostError> {
        use CostBound::*;

        match (self, rhs) {
            (Exact(a), Exact(b)) => Ok(Exact(a.checked_add(b)?)),

            (Unbounded, _) | (_, Unbounded) => Ok(Unbounded),

            (Unknown, _) | (_, Unknown) => Ok(Unknown),

            (AtLeast(a), AtLeast(b)) => Ok(AtLeast(a.checked_add(b)?)),

            (AtLeast(a), Exact(b)) | (Exact(b), AtLeast(a)) => {
                Ok(AtLeast(a.checked_add(b)?))
            }

            (AtMost(a), AtMost(b)) => Ok(AtMost(a.checked_add(b)?)),

            (AtMost(a), Exact(b)) | (Exact(b), AtMost(a)) => {
                Ok(AtMost(a.checked_add(b)?))
            }

            (
                Range {
                    lower: a_lower,
                    upper: a_upper,
                },
                Range {
                    lower: b_lower,
                    upper: b_upper,
                },
            ) => Self::range(
                a_lower.checked_add(b_lower)?,
                a_upper.checked_add(b_upper)?,
            ),

            (
                Range {
                    lower,
                    upper,
                },
                Exact(value),
            )
            | (
                Exact(value),
                Range {
                    lower,
                    upper,
                },
            ) => Self::range(
                lower.checked_add(value)?,
                upper.checked_add(value)?,
            ),

            (
                Range {
                    lower,
                    upper,
                },
                AtLeast(value),
            )
            | (
                AtLeast(value),
                Range {
                    lower,
                    upper,
                },
            ) => Ok(AtLeast(lower.checked_add(value)?.min(upper.checked_add(value)?))),

            (
                Range {
                    lower,
                    upper,
                },
                AtMost(value),
            )
            | (
                AtMost(value),
                Range {
                    lower,
                    upper,
                },
            ) => Ok(AtMost(upper.checked_add(value)?.max(lower.checked_add(value)?))),

            (AtLeast(a), AtMost(b)) | (AtMost(b), AtLeast(a)) => {
                // Both bounds exist, but their exact relationship may not.
                // We can retain a valid interval only when it is ordered.
                if a <= b {
                    Self::range(a, b)
                } else {
                    Ok(Unknown)
                }
            }
        }
    }
}

/// Provenance of a cost estimate.
///
/// This is descriptive metadata; it does not claim that the named source
/// has been independently verified.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CostProvenance {
    /// Stable identifier of the estimator, analysis, or measurement source.
    pub source: String,

    /// Optional version of the estimator or model.
    pub model_version: Option<String>,

    /// Optional note explaining the estimate.
    pub note: Option<String>,
}

impl CostProvenance {
    /// Construct provenance with a source identifier.
    pub fn new(source: impl Into<String>) -> Result<Self, CostError> {
        let source = source.into();

        if source.trim().is_empty() {
            return Err(CostError::InvalidProvenance);
        }

        Ok(Self {
            source,
            model_version: None,
            note: None,
        })
    }
}

/// One cost estimate for a single metric.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostEstimate {
    /// Metric being estimated.
    pub metric: CostMetric,

    /// Exact, bounded, unknown, or unbounded value.
    pub bound: CostBound,

    /// Confidence/quality classification.
    pub quality: EstimateQuality,

    /// Optional source provenance.
    pub provenance: Option<CostProvenance>,
}

impl CostEstimate {
    /// Construct an estimate.
    pub const fn new(
        metric: CostMetric,
        bound: CostBound,
        quality: EstimateQuality,
    ) -> Self {
        Self {
            metric,
            bound,
            quality,
            provenance: None,
        }
    }

    /// Attach provenance.
    pub fn with_provenance(mut self, provenance: CostProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Construct an exact estimate.
    pub const fn exact(metric: CostMetric, value: CostQuantity) -> Self {
        Self::new(
            metric,
            CostBound::Exact(value),
            EstimateQuality::Exact,
        )
    }

    /// Construct an unknown estimate.
    pub const fn unknown(metric: CostMetric) -> Self {
        Self::new(
            metric,
            CostBound::Unknown,
            EstimateQuality::Unknown,
        )
    }
}

/// A collection of cost estimates for a program, operation, plan, or stage.
///
/// Metrics are stored by identifier for deterministic iteration.
/// Each metric is kept separate: memory bytes are not added to latency,
/// and latency is not compared with energy unless a policy explicitly
/// defines a conversion or weighting.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostVector {
    estimates: BTreeMap<CostMetric, CostEstimate>,
}

impl CostVector {
    /// Create an empty cost vector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace an estimate for its metric.
    pub fn insert(&mut self, estimate: CostEstimate) {
        self.estimates.insert(estimate.metric.clone(), estimate);
    }

    /// Get an estimate by metric.
    pub fn get(&self, metric: &CostMetric) -> Option<&CostEstimate> {
        self.estimates.get(metric)
    }

    /// Iterate estimates in deterministic metric order.
    pub fn iter(&self) -> impl Iterator<Item = (&CostMetric, &CostEstimate)> {
        self.estimates.iter()
    }

    /// Return true if the vector has no estimates.
    pub fn is_empty(&self) -> bool {
        self.estimates.is_empty()
    }

    /// Return the number of metrics in this vector.
    pub fn len(&self) -> usize {
        self.estimates.len()
    }

    /// Add a cost vector metric-by-metric.
    ///
    /// Metrics missing from one side are preserved as-is. Matching metrics
    /// are combined only when their bounds can be combined conservatively.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, CostError> {
        let mut result = self.clone();

        for (metric, rhs_estimate) in rhs.iter() {
            match result.estimates.get(metric) {
                Some(lhs_estimate) => {
                    let bound = lhs_estimate.bound.checked_add(rhs_estimate.bound)?;

                    let quality = if lhs_estimate.quality == EstimateQuality::Unknown
                        || rhs_estimate.quality == EstimateQuality::Unknown
                    {
                        EstimateQuality::Unknown
                    } else if lhs_estimate.quality == EstimateQuality::Estimated
                        || rhs_estimate.quality == EstimateQuality::Estimated
                    {
                        EstimateQuality::Estimated
                    } else {
                        EstimateQuality::Exact
                    };

                    result.insert(CostEstimate {
                        metric: metric.clone(),
                        bound,
                        quality,
                        provenance: None,
                    });
                }
                None => {
                    result.insert(rhs_estimate.clone());
                }
            }
        }

        Ok(result)
    }
}

/// A cost objective.
///
/// Objective direction is explicit; lower cost is not universally better
/// for metrics such as throughput.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ObjectiveDirection {
    Minimize,
    Maximize,
}

/// One objective used to compare plans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostObjective {
    pub metric: CostMetric,
    pub direction: ObjectiveDirection,
}

/// A policy describing which metrics matter to a decision.
///
/// This policy does not itself decide feasibility. Resource constraints and
/// capability requirements must be evaluated by their respective analyses.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostPolicy {
    objectives: Vec<CostObjective>,
}

impl CostPolicy {
    /// Create an empty policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an objective.
    pub fn add_objective(&mut self, objective: CostObjective) {
        self.objectives.push(objective);
    }

    /// Return objectives in policy order.
    pub fn objectives(&self) -> &[CostObjective] {
        &self.objectives
    }

    /// Compare two cost vectors lexicographically using this policy.
    ///
    /// Returns `None` if the policy is empty, a required metric is missing,
    /// or an estimate is not exact. This avoids inventing an ordering from
    /// unknown or partial data.
    pub fn compare_exact(
        &self,
        lhs: &CostVector,
        rhs: &CostVector,
    ) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        for objective in &self.objectives {
            let lhs_value = lhs
                .get(&objective.metric)?
                .bound
                .exact_value()?;

            let rhs_value = rhs
                .get(&objective.metric)?
                .bound
                .exact_value()?;

            let ordering = lhs_value.cmp(&rhs_value);

            let ordering = match objective.direction {
                ObjectiveDirection::Minimize => ordering,
                ObjectiveDirection::Maximize => ordering.reverse(),
            };

            if ordering != Ordering::Equal {
                return Some(ordering);
            }
        }

        if self.objectives.is_empty() {
            None
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// Errors produced by cost construction or arithmetic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CostError {
    InvalidMetricId,
    InvalidRange,
    InvalidProvenance,
    ArithmeticOverflow,
}

impl fmt::Display for CostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetricId => {
                f.write_str("cost metric identifier must not be empty")
            }
            Self::InvalidRange => {
                f.write_str("cost range lower bound exceeds upper bound")
            }
            Self::InvalidProvenance => {
                f.write_str("cost provenance source must not be empty")
            }
            Self::ArithmeticOverflow => {
                f.write_str("cost arithmetic overflowed its numeric representation")
            }
        }
    }
}

impl std::error::Error for CostError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn quantity(value: u128) -> CostQuantity {
        CostQuantity::new(value)
    }

    #[test]
    fn metric_id_rejects_empty_name() {
        assert_eq!(
            MetricId::new("  "),
            Err(CostError::InvalidMetricId)
        );
    }

    #[test]
    fn metric_id_accepts_namespaced_domain_metric() {
        let id = MetricId::new("quantum.estimated_error")
            .expect("valid metric identifier");

        assert_eq!(id.as_str(), "quantum.estimated_error");
    }

    #[test]
    fn exact_bounds_add_exactly() {
        let result = CostBound::Exact(quantity(10))
            .checked_add(CostBound::Exact(quantity(12)))
            .expect("addition succeeds");

        assert_eq!(result, CostBound::Exact(quantity(22)));
    }

    #[test]
    fn unknown_is_not_treated_as_zero() {
        let result = CostBound::Unknown
            .checked_add(CostBound::Exact(quantity(12)))
            .expect("unknown addition succeeds");

        assert_eq!(result, CostBound::Unknown);
    }

    #[test]
    fn unbounded_remains_unbounded() {
        let result = CostBound::Unbounded
            .checked_add(CostBound::Exact(quantity(12)))
            .expect("unbounded addition succeeds");

        assert_eq!(result, CostBound::Unbounded);
    }

    #[test]
    fn range_rejects_reversed_bounds() {
        assert_eq!(
            CostBound::range(quantity(9), quantity(3)),
            Err(CostError::InvalidRange)
        );
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let result = quantity(u128::MAX).checked_add(quantity(1));

        assert_eq!(result, Err(CostError::ArithmeticOverflow));
    }

    #[test]
    fn cost_vector_preserves_distinct_metrics() {
        let mut costs = CostVector::new();

        costs.insert(CostEstimate::exact(
            CostMetric::MemoryBytes,
            quantity(4096),
        ));

        costs.insert(CostEstimate::exact(
            CostMetric::ExecutionLatencyNs,
            quantity(200),
        ));

        assert_eq!(
            costs.get(&CostMetric::MemoryBytes)
                .and_then(|estimate| estimate.bound.exact_value()),
            Some(quantity(4096))
        );

        assert_eq!(
            costs.get(&CostMetric::ExecutionLatencyNs)
                .and_then(|estimate| estimate.bound.exact_value()),
            Some(quantity(200))
        );
    }

    #[test]
    fn cost_vector_adds_matching_metrics() {
        let mut lhs = CostVector::new();
        lhs.insert(CostEstimate::exact(
            CostMetric::OperationCount,
            quantity(4),
        ));

        let mut rhs = CostVector::new();
        rhs.insert(CostEstimate::exact(
            CostMetric::OperationCount,
            quantity(6),
        ));

        let result = lhs.checked_add(&rhs).expect("vector addition succeeds");

        assert_eq!(
            result.get(&CostMetric::OperationCount)
                .and_then(|estimate| estimate.bound.exact_value()),
            Some(quantity(10))
        );
    }

    #[test]
    fn policy_compares_exact_values_in_declared_order() {
        let mut policy = CostPolicy::new();
        policy.add_objective(CostObjective {
            metric: CostMetric::ExecutionLatencyNs,
            direction: ObjectiveDirection::Minimize,
        });

        let mut faster = CostVector::new();
        faster.insert(CostEstimate::exact(
            CostMetric::ExecutionLatencyNs,
            quantity(20),
        ));

        let mut slower = CostVector::new();
        slower.insert(CostEstimate::exact(
            CostMetric::ExecutionLatencyNs,
            quantity(30),
        ));

        assert_eq!(
            policy.compare_exact(&faster, &slower),
            Some(std::cmp::Ordering::Less)
        );
    }

    #[test]
    fn policy_does_not_order_unknown_estimates() {
        let mut policy = CostPolicy::new();
        policy.add_objective(CostObjective {
            metric: CostMetric::ExecutionLatencyNs,
            direction: ObjectiveDirection::Minimize,
        });

        let mut unknown = CostVector::new();
        unknown.insert(CostEstimate::unknown(
            CostMetric::ExecutionLatencyNs,
        ));

        let mut exact = CostVector::new();
        exact.insert(CostEstimate::exact(
            CostMetric::ExecutionLatencyNs,
            quantity(10),
        ));

        assert_eq!(policy.compare_exact(&unknown, &exact), None);
    }
}