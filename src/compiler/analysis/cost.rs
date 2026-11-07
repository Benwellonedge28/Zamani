
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Cost Analysis
//!
//! Path: src/compiler/analysis/cost.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides target-independent cost observations and estimates for
//! compiler analysis, optimization, and realization planning.
//!
//! This module establishes a common representation for:
//!
//! - extensible cost metric identities;
//! - cost units;
//! - arbitrary-length exact quantities;
//! - exact and uncertain cost bounds;
//! - estimate quality;
//! - estimator and measurement provenance;
//! - multidimensional cost vectors;
//! - explicit aggregation semantics;
//! - multi-objective comparison policies.
//!
//! # Architectural ownership
//!
//! This module owns compiler-facing cost evidence.
//!
//! It does not own:
//!
//! - source-language semantics;
//! - canonical IR;
//! - hardware capabilities;
//! - resource requirement semantics;
//! - physical resource discovery;
//! - resource allocation;
//! - backend selection;
//! - optimization algorithms;
//! - physical scheduling;
//! - quantum routing;
//! - QEC;
//! - runtime execution.
//!
//! # Canonical authorities
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Resource semantics:
//!     crate::frontend::semantic::resources
//!
//! Capability analysis:
//!     crate::compiler::analysis::capabilities
//!
//! Feasibility:
//!     crate::compiler::analysis::feasibility
//!
//! Resource planning:
//!     crate::compiler::planning::resource_planning
//!
//! Analytical optimization cost models:
//!     crate::compiler::optimization::cost_model
//!
//! Optimization profitability:
//!     crate::compiler::optimization::profitability
//!
//! Optimization planning:
//!     crate::compiler::optimization::planning
//!
//! Quantum-specific cost semantics:
//!     crate::quantum::optimization
//!
//! Quantum routing:
//!     crate::quantum::routing
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! No universal hardware population, resource capacity, operation count,
//! workload size, or computing-domain ceiling is introduced.
//!
//! Quantities use arbitrary-length decimal representations rather than
//! fixed-width machine integers.
//!
//! This does not promise infinite physical memory or infinite execution.
//! Processing remains subject to actual available resources and explicit
//! operational budgets.
//!
//! # Important invariant
//!
//! An unknown estimate is never zero.
//! An unbounded estimate is never treated as a finite estimate.
//! An estimate is not a performance guarantee.
//! A cost comparison is not a feasibility determination.
//! A cost model does not establish semantic correctness.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const COST_ANALYSIS_MODULE_NAME: &str =
    "zamani.compiler.analysis.cost";

/// Version of the public cost-analysis contract.
///
/// This is not the language version or artifact format version.
pub const COST_ANALYSIS_API_VERSION: u16 = 1;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifies an extensible cost metric.
///
/// Examples:
///
/// - compiler.compile_work
/// - compiler.compile_time_ns
/// - execution.latency_ns
/// - execution.throughput
/// - resource.memory_bytes
/// - resource.storage_bytes
/// - communication.bytes
/// - quantum.gate_operations
/// - quantum.estimated_error
/// - ai.tensor_operations
/// - hdl.synthesis_area
/// - distributed.communication_latency
/// - future.photonic.operation_cost
///
/// The namespace is open-ended.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MetricId(String);

impl MetricId {
    /// Creates a validated metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostError> {
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

/// Identifies a cost measurement unit.
///
/// Units are extensible identifiers, not a closed enumeration.
///
/// Examples:
///
/// - bytes
/// - seconds
/// - nanoseconds
/// - joules
/// - operations
/// - messages
/// - gates
/// - qubits
/// - implementation-defined units
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CostUnit(String);

impl CostUnit {
    /// Creates a validated unit identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CostError> {
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

impl fmt::Display for CostUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> Result<(), CostError> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(CostError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// EXACT QUANTITIES
// ============================================================================

/// An arbitrary-length exact non-negative integer.
///
/// The canonical representation is a decimal string.
///
/// Examples:
///
/// - 0
/// - 1024
/// - 18446744073709551616
/// - 999999999999999999999999999999999999999999
///
/// No u64, u128, usize, pointer-width or hardware-width ceiling is imposed.
///
/// Actual representation size is bounded by available memory.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CostQuantity {
    digits: String,
}

impl CostQuantity {
    /// Creates a canonical exact quantity.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CostError> {
        let value = value.as_ref();

        if value.is_empty()
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(CostError::InvalidQuantity {
                value: value.to_owned(),
            });
        }

        let normalized = value.trim_start_matches('0');

        Ok(Self {
            digits: if normalized.is_empty() {
                "0".to_owned()
            } else {
                normalized.to_owned()
            },
        })
    }

    /// Creates zero.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            digits: "0".to_owned(),
        }
    }

    /// Returns the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.digits
    }

    /// Returns whether the quantity is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.digits == "0"
    }

    /// Adds two quantities without machine-integer conversion.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, CostError> {
        let mut left = self.digits.bytes().rev();
        let mut right = rhs.digits.bytes().rev();

        let mut result = Vec::new();
        let mut carry = 0_u8;

        loop {
            let a = left.next();
            let b = right.next();

            if a.is_none() && b.is_none() && carry == 0 {
                break;
            }

            let a = a.map(|v| v - b'0').unwrap_or(0);
            let b = b.map(|v| v - b'0').unwrap_or(0);

            let sum = a + b + carry;

            result.push(b'0' + sum % 10);
            carry = sum / 10;
        }

        result.reverse();

        let digits = String::from_utf8(result)
            .map_err(|_| CostError::ArithmeticFailure)?;

        Ok(Self { digits })
    }

    /// Multiplies two quantities without machine-integer conversion.
    ///
    /// Intermediate arithmetic operates on individual decimal digits.
    pub fn checked_mul(&self, rhs: &Self) -> Result<Self, CostError> {
        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero());
        }

        let left = self.digits.as_bytes();
        let right = rhs.digits.as_bytes();

        let mut result = vec![0_u8; left.len() + right.len() + 1];

        for i in (0..left.len()).rev() {
            let mut carry = 0_u16;

            for j in (0..right.len()).rev() {
                let index = i + j + 1;

                let a = u16::from(left[i] - b'0');
                let b = u16::from(right[j] - b'0');

                let total =
                    a * b + u16::from(result[index]) + carry;

                result[index] = (total % 10) as u8;
                carry = total / 10;
            }

            let mut index = i;

            while carry > 0 {
                let total =
                    u16::from(result[index]) + carry;

                result[index] = (total % 10) as u8;
                carry = total / 10;

                if index == 0 && carry > 0 {
                    return Err(CostError::ArithmeticFailure);
                }

                index = index.saturating_sub(1);
            }
        }

        let first = result
            .iter()
            .position(|digit| *digit != 0)
            .unwrap_or(result.len() - 1);

        let digits = result[first..]
            .iter()
            .map(|digit| char::from(b'0' + *digit))
            .collect();

        Ok(Self { digits })
    }

    /// Compares two exact quantities numerically.
    #[must_use]
    pub fn compare(&self, rhs: &Self) -> Ordering {
        self.cmp(rhs)
    }
}

impl Ord for CostQuantity {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.digits
            .len()
            .cmp(&rhs.digits.len())
            .then_with(|| self.digits.cmp(&rhs.digits))
    }
}

impl PartialOrd for CostQuantity {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl From<u64> for CostQuantity {
    fn from(value: u64) -> Self {
        Self::new(value.to_string()).expect("u64 is a valid quantity")
    }
}

impl From<u128> for CostQuantity {
    fn from(value: u128) -> Self {
        Self::new(value.to_string()).expect("u128 is a valid quantity")
    }
}

impl fmt::Display for CostQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.digits)
    }
}

// ============================================================================
// ESTIMATE QUALITY
// ============================================================================

/// Describes the quality of an estimate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EstimateQuality {
    /// Established through exact analysis or exact measurement.
    Exact,

    /// Obtained through a model, heuristic or extrapolation.
    Estimated,

    /// Insufficient evidence is available.
    Unknown,
}

// ============================================================================
// COST BOUNDS
// ============================================================================

/// Describes the known bounds of a non-negative cost.
///
/// Unknown and unbounded are distinct states.
///
/// Unknown means the value cannot currently be established.
///
/// Unbounded means the model cannot establish a finite upper bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CostBound {
    /// The exact value is known.
    Exact(CostQuantity),

    /// The value is at least the specified quantity.
    AtLeast(CostQuantity),

    /// The value is at most the specified quantity.
    AtMost(CostQuantity),

    /// Both lower and upper bounds are known.
    Range {
        /// Inclusive lower bound.
        lower: CostQuantity,

        /// Inclusive upper bound.
        upper: CostQuantity,
    },

    /// The value is unknown.
    Unknown,

    /// The value has no established finite upper bound.
    Unbounded,
}

impl CostBound {
    /// Constructs a validated range.
    pub fn range(
        lower: CostQuantity,
        upper: CostQuantity,
    ) -> Result<Self, CostError> {
        if lower > upper {
            return Err(CostError::InvalidRange);
        }

        Ok(Self::Range { lower, upper })
    }

    /// Returns the exact value if available.
    #[must_use]
    pub fn exact_value(&self) -> Option<&CostQuantity> {
        match self {
            Self::Exact(value) => Some(value),
            _ => None,
        }
    }

    /// Returns an established lower bound.
    #[must_use]
    pub fn lower_bound(&self) -> Option<&CostQuantity> {
        match self {
            Self::Exact(value)
            | Self::AtLeast(value) => Some(value),

            Self::Range { lower, .. } => Some(lower),

            Self::AtMost(_)
            | Self::Unknown
            | Self::Unbounded => None,
        }
    }

    /// Returns an established finite upper bound.
    #[must_use]
    pub fn upper_bound(&self) -> Option<&CostQuantity> {
        match self {
            Self::Exact(value)
            | Self::AtMost(value) => Some(value),

            Self::Range { upper, .. } => Some(upper),

            Self::AtLeast(_)
            | Self::Unknown
            | Self::Unbounded => None,
        }
    }

    /// Adds two independent non-negative cost bounds.
    ///
    /// Bound propagation is conservative.
    ///
    /// Unknown information is never converted into zero.
    /// Missing upper bounds are never converted into finite bounds.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, CostError> {
        if matches!(self, Self::Unknown)
            || matches!(rhs, Self::Unknown)
        {
            return Ok(Self::Unknown);
        }

        let zero = CostQuantity::zero();

        let left_lower = self.lower_bound().unwrap_or(&zero);
        let right_lower = rhs.lower_bound().unwrap_or(&zero);

        let left_upper = self.upper_bound();
        let right_upper = rhs.upper_bound();

        let lower = if self.lower_bound().is_some()
            && rhs.lower_bound().is_some()
        {
            Some(left_lower.checked_add(right_lower)?)
        } else {
            None
        };

        let upper = match (left_upper, right_upper) {
            (Some(a), Some(b)) => Some(a.checked_add(b)?),
            _ => None,
        };

        match (lower, upper) {
            (Some(lower), Some(upper)) => {
                if lower == upper {
                    Ok(Self::Exact(lower))
                } else {
                    Self::range(lower, upper)
                }
            }

            (Some(lower), None) => Ok(Self::AtLeast(lower)),

            (None, Some(upper)) => Ok(Self::AtMost(upper)),

            (None, None) => Ok(Self::Unbounded),
        }
    }

    /// Returns whether the estimate has an established exact value.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

// ============================================================================
// PROVENANCE
// ============================================================================

/// Identifies the origin of a cost estimate.
///
/// Provenance is descriptive evidence.
///
/// It does not automatically authenticate the source.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CostProvenance {
    /// Estimator, analysis or measurement source.
    pub source: String,

    /// Optional estimator version.
    pub model_version: Option<String>,

    /// Optional explanatory note.
    pub note: Option<String>,
}

impl CostProvenance {
    /// Creates validated provenance.
    pub fn new(source: impl Into<String>) -> Result<Self, CostError> {
        let source = source.into();

        validate_identifier(&source, "provenance")?;

        Ok(Self {
            source,
            model_version: None,
            note: None,
        })
    }

    /// Attaches a model version.
    pub fn with_model_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, CostError> {
        let version = version.into();

        validate_identifier(&version, "model version")?;

        self.model_version = Some(version);

        Ok(self)
    }

    /// Attaches an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

// ============================================================================
// COST ESTIMATES
// ============================================================================

/// One cost estimate for one metric and one unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostEstimate {
    metric: CostMetric,
    unit: CostUnit,
    bound: CostBound,
    quality: EstimateQuality,
    provenance: Option<CostProvenance>,
}

/// A cost metric identity.
///
/// Standard metric identities are provided for common compiler needs.
/// Custom metrics remain open-ended.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CostMetric {
    /// Compiler work.
    CompileWork,

    /// Compilation duration.
    CompileTime,

    /// Execution latency.
    ExecutionLatency,

    /// Execution throughput.
    Throughput,

    /// Energy consumption.
    Energy,

    /// Peak memory consumption.
    Memory,

    /// Storage consumption.
    Storage,

    /// Communication volume.
    Communication,

    /// Operation count.
    Operations,

    /// Synchronization count.
    Synchronizations,

    /// Custom extensible metric.
    Custom(MetricId),
}

impl CostMetric {
    /// Returns the stable textual identity.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::CompileWork => "compiler.compile_work".to_owned(),
            Self::CompileTime => "compiler.compile_time".to_owned(),
            Self::ExecutionLatency => "execution.latency".to_owned(),
            Self::Throughput => "execution.throughput".to_owned(),
            Self::Energy => "execution.energy".to_owned(),
            Self::Memory => "resource.memory".to_owned(),
            Self::Storage => "resource.storage".to_owned(),
            Self::Communication => "communication.volume".to_owned(),
            Self::Operations => "program.operations".to_owned(),
            Self::Synchronizations => {
                "execution.synchronizations".to_owned()
            }
            Self::Custom(id) => id.as_str().to_owned(),
        }
    }
}

impl CostEstimate {
    /// Constructs a validated cost estimate.
    pub fn new(
        metric: CostMetric,
        unit: CostUnit,
        bound: CostBound,
        quality: EstimateQuality,
    ) -> Result<Self, CostError> {
        match (&bound, quality) {
            (CostBound::Exact(_), EstimateQuality::Exact) => {}

            (CostBound::Unknown, EstimateQuality::Unknown) => {}

            (CostBound::Unknown, _) => {
                return Err(CostError::QualityMismatch);
            }

            (_, EstimateQuality::Exact) => {
                return Err(CostError::QualityMismatch);
            }

            (_, EstimateQuality::Unknown) => {
                return Err(CostError::QualityMismatch);
            }

            (_, EstimateQuality::Estimated) => {}
        }

        Ok(Self {
            metric,
            unit,
            bound,
            quality,
            provenance: None,
        })
    }

    /// Creates an exact estimate.
    pub fn exact(
        metric: CostMetric,
        unit: CostUnit,
        value: CostQuantity,
    ) -> Self {
        Self {
            metric,
            unit,
            bound: CostBound::Exact(value),
            quality: EstimateQuality::Exact,
            provenance: None,
        }
    }

    /// Creates an unknown estimate.
    pub fn unknown(
        metric: CostMetric,
        unit: CostUnit,
    ) -> Self {
        Self {
            metric,
            unit,
            bound: CostBound::Unknown,
            quality: EstimateQuality::Unknown,
            provenance: None,
        }
    }

    /// Attaches provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: CostProvenance,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Returns the metric.
    #[must_use]
    pub fn metric(&self) -> &CostMetric {
        &self.metric
    }

    /// Returns the unit.
    #[must_use]
    pub fn unit(&self) -> &CostUnit {
        &self.unit
    }

    /// Returns the bound.
    #[must_use]
    pub fn bound(&self) -> &CostBound {
        &self.bound
    }

    /// Returns estimate quality.
    #[must_use]
    pub const fn quality(&self) -> EstimateQuality {
        self.quality
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> Option<&CostProvenance> {
        self.provenance.as_ref()
    }
}

// ============================================================================
// EXPLICIT AGGREGATION
// ============================================================================

/// Defines how two observations of the same metric may be aggregated.
///
/// Aggregation is never inferred merely from the metric name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CostAggregation {
    /// Add independent quantities.
    Sum,

    /// Retain the larger quantity.
    Maximum,

    /// Retain the smaller quantity.
    Minimum,
}

impl CostAggregation {
    /// Aggregates two bounds using the declared operation.
    pub fn aggregate(
        self,
        left: &CostBound,
        right: &CostBound,
    ) -> Result<CostBound, CostError> {
        match self {
            Self::Sum => left.checked_add(right),

            Self::Maximum => aggregate_extreme(left, right, true),

            Self::Minimum => aggregate_extreme(left, right, false),
        }
    }
}

fn aggregate_extreme(
    left: &CostBound,
    right: &CostBound,
    maximum: bool,
) -> Result<CostBound, CostError> {
    if matches!(left, CostBound::Unknown)
        || matches!(right, CostBound::Unknown)
    {
        return Ok(CostBound::Unknown);
    }

    let lower = match (left.lower_bound(), right.lower_bound()) {
        (Some(a), Some(b)) => {
            Some(if maximum {
                a.max(b).clone()
            } else {
                a.min(b).clone()
            })
        }
        _ => None,
    };

    let upper = match (left.upper_bound(), right.upper_bound()) {
        (Some(a), Some(b)) => {
            Some(if maximum {
                a.max(b).clone()
            } else {
                a.min(b).clone()
            })
        }
        _ => None,
    };

    match (lower, upper) {
        (Some(lower), Some(upper)) => {
            if lower == upper {
                Ok(CostBound::Exact(lower))
            } else {
                CostBound::range(lower, upper)
            }
        }

        (Some(lower), None) => Ok(CostBound::AtLeast(lower)),

        (None, Some(upper)) => Ok(CostBound::AtMost(upper)),

        (None, None) => Ok(CostBound::Unbounded),
    }
}

// ============================================================================
// COST VECTOR
// ============================================================================

/// A deterministic collection of independent cost estimates.
///
/// Metrics are kept separate.
///
/// Different units are never implicitly converted.
///
/// The vector does not automatically assume that latency, energy,
/// memory and operation counts can be added together.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostVector {
    estimates: BTreeMap<CostMetric, CostEstimate>,
}

impl CostVector {
    /// Creates an empty vector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an estimate.
    ///
    /// Duplicate metric identities are rejected.
    pub fn insert(
        &mut self,
        estimate: CostEstimate,
    ) -> Result<(), CostError> {
        let metric = estimate.metric.clone();

        if self.estimates.contains_key(&metric) {
            return Err(CostError::DuplicateMetric {
                metric: metric.name(),
            });
        }

        self.estimates.insert(metric, estimate);

        Ok(())
    }

    /// Explicitly replaces an existing estimate.
    ///
    /// This is intended for deliberate evidence refinement.
    pub fn replace(
        &mut self,
        estimate: CostEstimate,
    ) -> Option<CostEstimate> {
        self.estimates
            .insert(estimate.metric.clone(), estimate)
    }

    /// Retrieves an estimate.
    #[must_use]
    pub fn get(&self, metric: &CostMetric) -> Option<&CostEstimate> {
        self.estimates.get(metric)
    }

    /// Iterates estimates in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&CostMetric, &CostEstimate)> {
        self.estimates.iter()
    }

    /// Returns the number of metrics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.estimates.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.estimates.is_empty()
    }

    /// Merges two vectors without performing numerical aggregation.
    ///
    /// A duplicate metric is an error because its aggregation semantics
    /// have not been specified.
    pub fn merge(&self, rhs: &Self) -> Result<Self, CostError> {
        let mut result = self.clone();

        for estimate in rhs.estimates.values() {
            result.insert(estimate.clone())?;
        }

        Ok(result)
    }

    /// Aggregates two vectors using explicitly supplied metric operations.
    ///
    /// A missing metric on either side is preserved.
    ///
    /// Duplicate metrics require an explicit aggregation operation.
    ///
    /// Units must match exactly. Unit conversion belongs to an
    /// authoritative unit-conversion service.
    pub fn aggregate(
        &self,
        rhs: &Self,
        operations: &BTreeMap<CostMetric, CostAggregation>,
    ) -> Result<Self, CostError> {
        let mut result = self.clone();

        for (metric, incoming) in &rhs.estimates {
            match result.estimates.get(metric) {
                None => {
                    result.insert(incoming.clone())?;
                }

                Some(existing) => {
                    let operation = operations
                        .get(metric)
                        .ok_or_else(|| CostError::MissingAggregation {
                            metric: metric.name(),
                        })?;

                    if existing.unit != incoming.unit {
                        return Err(CostError::UnitMismatch {
                            metric: metric.name(),
                            left: existing.unit.as_str().to_owned(),
                            right: incoming.unit.as_str().to_owned(),
                        });
                    }

                    let bound = operation.aggregate(
                        &existing.bound,
                        &incoming.bound,
                    )?;

                    let quality = combine_quality(
                        existing.quality,
                        incoming.quality,
                    );

                    let estimate = CostEstimate::new(
                        metric.clone(),
                        existing.unit.clone(),
                        bound,
                        quality,
                    )?;

                    result.replace(estimate);
                }
            }
        }

        Ok(result)
    }
}

fn combine_quality(
    left: EstimateQuality,
    right: EstimateQuality,
) -> EstimateQuality {
    match (left, right) {
        (EstimateQuality::Unknown, _)
        | (_, EstimateQuality::Unknown) => EstimateQuality::Unknown,

        (EstimateQuality::Estimated, _)
        | (_, EstimateQuality::Estimated) => EstimateQuality::Estimated,

        _ => EstimateQuality::Exact,
    }
}

// ============================================================================
// OBJECTIVES AND COMPARISON
// ============================================================================

/// Direction of a cost objective.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ObjectiveDirection {
    /// Prefer smaller values.
    Minimize,

    /// Prefer larger values.
    Maximize,
}

/// One objective in a comparison policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostObjective {
    /// Metric to compare.
    pub metric: CostMetric,

    /// Desired direction.
    pub direction: ObjectiveDirection,
}

/// A deterministic lexicographic comparison policy.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostPolicy {
    objectives: Vec<CostObjective>,
}

impl CostPolicy {
    /// Creates an empty policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an objective.
    ///
    /// Duplicate objectives are rejected.
    pub fn add_objective(
        &mut self,
        objective: CostObjective,
    ) -> Result<(), CostError> {
        if self.objectives.iter().any(|existing| {
            existing.metric == objective.metric
        }) {
            return Err(CostError::DuplicateObjective {
                metric: objective.metric.name(),
            });
        }

        self.objectives.push(objective);

        Ok(())
    }

    /// Returns objectives in priority order.
    #[must_use]
    pub fn objectives(&self) -> &[CostObjective] {
        &self.objectives
    }

    /// Compares two cost vectors.
    ///
    /// Returns None when:
    ///
    /// - the policy is empty;
    /// - a required metric is missing;
    /// - units differ;
    /// - evidence is unknown;
    /// - uncertainty prevents a defensible ordering.
    ///
    /// Disjoint conservative ranges can be compared safely.
    pub fn compare(
        &self,
        left: &CostVector,
        right: &CostVector,
    ) -> Result<Option<Ordering>, CostError> {
        if self.objectives.is_empty() {
            return Ok(None);
        }

        for objective in &self.objectives {
            let Some(a) = left.get(&objective.metric) else {
                return Ok(None);
            };

            let Some(b) = right.get(&objective.metric) else {
                return Ok(None);
            };

            if a.unit != b.unit {
                return Err(CostError::UnitMismatch {
                    metric: objective.metric.name(),
                    left: a.unit.as_str().to_owned(),
                    right: b.unit.as_str().to_owned(),
                });
            }

            let Some(mut ordering) = compare_bounds(
                &a.bound,
                &b.bound,
            ) else {
                return Ok(None);
            };

            if objective.direction == ObjectiveDirection::Maximize {
                ordering = ordering.reverse();
            }

            if ordering != Ordering::Equal {
                return Ok(Some(ordering));
            }
        }

        Ok(Some(Ordering::Equal))
    }
}

fn compare_bounds(
    left: &CostBound,
    right: &CostBound,
) -> Option<Ordering> {
    if let (
        CostBound::Exact(a),
        CostBound::Exact(b),
    ) = (left, right) {
        return Some(a.cmp(b));
    }

    let left_lower = left.lower_bound()?;
    let right_lower = right.lower_bound()?;

    let left_upper = left.upper_bound();
    let right_upper = right.upper_bound();

    if let Some(upper) = left_upper {
        if upper < right_lower {
            return Some(Ordering::Less);
        }
    }

    if let Some(upper) = right_upper {
        if upper < left_lower {
            return Some(Ordering::Greater);
        }
    }

    None
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by cost analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CostError {
    /// An identifier was invalid.
    InvalidIdentifier {
        /// Identifier category.
        kind: &'static str,

        /// Supplied value.
        value: String,
    },

    /// A quantity was invalid.
    InvalidQuantity {
        /// Supplied value.
        value: String,
    },

    /// A cost range had reversed bounds.
    InvalidRange,

    /// Estimate quality disagreed with its bound.
    QualityMismatch,

    /// An estimate duplicated an existing metric.
    DuplicateMetric {
        /// Metric identity.
        metric: String,
    },

    /// An objective duplicated an existing objective.
    DuplicateObjective {
        /// Metric identity.
        metric: String,
    },

    /// An aggregation operation was not supplied.
    MissingAggregation {
        /// Metric identity.
        metric: String,
    },

    /// Units were incompatible.
    UnitMismatch {
        /// Metric identity.
        metric: String,

        /// Left unit.
        left: String,

        /// Right unit.
        right: String,
    },

    /// Arithmetic or internal representation failed.
    ArithmeticFailure,
}

impl fmt::Display for CostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind} identifier: {value}")
            }

            Self::InvalidQuantity { value } => {
                write!(f, "invalid non-negative cost quantity: {value}")
            }

            Self::InvalidRange => {
                f.write_str("cost range lower bound exceeds upper bound")
            }

            Self::QualityMismatch => {
                f.write_str("cost estimate quality disagrees with its bound")
            }

            Self::DuplicateMetric { metric } => {
                write!(f, "duplicate cost metric: {metric}")
            }

            Self::DuplicateObjective { metric } => {
                write!(f, "duplicate cost objective: {metric}")
            }

            Self::MissingAggregation { metric } => {
                write!(f, "aggregation semantics missing for metric: {metric}")
            }

            Self::UnitMismatch {
                metric,
                left,
                right,
            } => {
                write!(
                    f,
                    "incompatible units for {metric}: {left} and {right}"
                )
            }

            Self::ArithmeticFailure => {
                f.write_str("cost arithmetic failed")
            }
        }
    }
}

impl Error for CostError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn quantity(value: &str) -> CostQuantity {
        CostQuantity::new(value).expect("valid quantity")
    }

    fn unit(value: &str) -> CostUnit {
        CostUnit::new(value).expect("valid unit")
    }

    fn estimate(
        metric: CostMetric,
        value: &str,
        unit_name: &str,
    ) -> CostEstimate {
        CostEstimate::exact(
            metric,
            unit(unit_name),
            quantity(value),
        )
    }

    #[test]
    fn canonicalizes_arbitrary_length_quantities() {
        let value = quantity("000000123456789012345678901234567890");

        assert_eq!(
            value.as_str(),
            "123456789012345678901234567890"
        );
    }

    #[test]
    fn compares_arbitrary_length_quantities() {
        assert!(quantity("99999999999999999999")
            < quantity("100000000000000000000"));
    }

    #[test]
    fn arbitrary_length_addition_is_exact() {
        let result = quantity("999999999999999999999999")
            .checked_add(&quantity("1"))
            .expect("addition succeeds");

        assert_eq!(result.as_str(), "1000000000000000000000000");
    }

    #[test]
    fn arbitrary_length_multiplication_is_exact() {
        let result = quantity("999999999999")
            .checked_mul(&quantity("999999999999"))
            .expect("multiplication succeeds");

        assert_eq!(
            result.as_str(),
            "999999999998000000000001"
        );
    }

    #[test]
    fn multiplication_by_zero_returns_zero() {
        let result = quantity("999999999999999999")
            .checked_mul(&CostQuantity::zero())
            .expect("multiplication succeeds");

        assert!(result.is_zero());
    }

    #[test]
    fn rejects_invalid_quantities() {
        for value in ["", "-1", "+1", "1.5", "1e9", "abc"] {
            assert!(CostQuantity::new(value).is_err());
        }
    }

    #[test]
    fn exact_bounds_add_correctly() {
        let result = CostBound::Exact(quantity("10"))
            .checked_add(&CostBound::Exact(quantity("20")))
            .expect("addition succeeds");

        assert_eq!(result, CostBound::Exact(quantity("30")));
    }

    #[test]
    fn at_least_plus_at_most_preserves_only_lower_bound() {
        let result = CostBound::AtLeast(quantity("10"))
            .checked_add(&CostBound::AtMost(quantity("20")))
            .expect("addition succeeds");

        assert_eq!(result, CostBound::AtLeast(quantity("10")));
    }

    #[test]
    fn range_plus_exact_preserves_range() {
        let result = CostBound::range(
            quantity("10"),
            quantity("20"),
        )
        .expect("range")
        .checked_add(&CostBound::Exact(quantity("5")))
        .expect("addition succeeds");

        assert_eq!(
            result,
            CostBound::Range {
                lower: quantity("15"),
                upper: quantity("25"),
            }
        );
    }

    #[test]
    fn unknown_is_not_zero() {
        let result = CostBound::Unknown
            .checked_add(&CostBound::Exact(quantity("10")))
            .expect("unknown propagation");

        assert_eq!(result, CostBound::Unknown);
    }

    #[test]
    fn unbounded_remains_unbounded() {
        let result = CostBound::Unbounded
            .checked_add(&CostBound::Exact(quantity("10")))
            .expect("unbounded propagation");

        assert_eq!(result, CostBound::Unbounded);
    }

    #[test]
    fn rejects_reversed_ranges() {
        assert!(CostBound::range(
            quantity("20"),
            quantity("10"),
        ).is_err());
    }

    #[test]
    fn vectors_keep_different_metrics_separate() {
        let mut vector = CostVector::new();

        vector.insert(estimate(
            CostMetric::Memory,
            "4096",
            "bytes",
        )).expect("insert memory");

        vector.insert(estimate(
            CostMetric::ExecutionLatency,
            "200",
            "nanoseconds",
        )).expect("insert latency");

        assert_eq!(vector.len(), 2);
    }

    #[test]
    fn vectors_reject_duplicate_metrics() {
        let mut vector = CostVector::new();

        vector.insert(estimate(
            CostMetric::Memory,
            "100",
            "bytes",
        )).expect("first insert");

        assert!(vector.insert(estimate(
            CostMetric::Memory,
            "200",
            "bytes",
        )).is_err());
    }

    #[test]
    fn aggregation_requires_explicit_semantics() {
        let mut left = CostVector::new();
        left.insert(estimate(
            CostMetric::Operations,
            "10",
            "operations",
        )).unwrap();

        let mut right = CostVector::new();
        right.insert(estimate(
            CostMetric::Operations,
            "20",
            "operations",
        )).unwrap();

        assert!(left.aggregate(
            &right,
            &BTreeMap::new(),
        ).is_err());
    }

    #[test]
    fn explicit_sum_aggregates_correctly() {
        let metric = CostMetric::Operations;

        let mut left = CostVector::new();
        left.insert(estimate(
            metric.clone(),
            "10",
            "operations",
        )).unwrap();

        let mut right = CostVector::new();
        right.insert(estimate(
            metric.clone(),
            "20",
            "operations",
        )).unwrap();

        let operations = BTreeMap::from([
            (metric.clone(), CostAggregation::Sum),
        ]);

        let result = left.aggregate(&right, &operations)
            .expect("aggregation succeeds");

        assert_eq!(
            result.get(&metric)
                .unwrap()
                .bound()
                .exact_value()
                .unwrap()
                .as_str(),
            "30"
        );
    }

    #[test]
    fn aggregation_rejects_incompatible_units() {
        let metric = CostMetric::Memory;

        let mut left = CostVector::new();
        left.insert(estimate(
            metric.clone(),
            "10",
            "bytes",
        )).unwrap();

        let mut right = CostVector::new();
        right.insert(estimate(
            metric.clone(),
            "10",
            "kilobytes",
        )).unwrap();

        let operations = BTreeMap::from([
            (metric, CostAggregation::Sum),
        ]);

        assert!(left.aggregate(&right, &operations).is_err());
    }

    #[test]
    fn comparison_handles_exact_costs() {
        let metric = CostMetric::ExecutionLatency;

        let mut policy = CostPolicy::new();

        policy.add_objective(CostObjective {
            metric: metric.clone(),
            direction: ObjectiveDirection::Minimize,
        }).unwrap();

        let mut faster = CostVector::new();
        faster.insert(estimate(
            metric.clone(),
            "10",
            "nanoseconds",
        )).unwrap();

        let mut slower = CostVector::new();
        slower.insert(estimate(
            metric,
            "20",
            "nanoseconds",
        )).unwrap();

        assert_eq!(
            policy.compare(&faster, &slower).unwrap(),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn comparison_uses_disjoint_uncertainty_ranges() {
        let metric = CostMetric::ExecutionLatency;

        let mut policy = CostPolicy::new();

        policy.add_objective(CostObjective {
            metric: metric.clone(),
            direction: ObjectiveDirection::Minimize,
        }).unwrap();

        let mut first = CostVector::new();
        first.insert(CostEstimate::new(
            metric.clone(),
            unit("nanoseconds"),
            CostBound::range(
                quantity("10"),
                quantity("20"),
            ).unwrap(),
            EstimateQuality::Estimated,
        ).unwrap()).unwrap();

        let mut second = CostVector::new();
        second.insert(CostEstimate::new(
            metric,
            unit("nanoseconds"),
            CostBound::range(
                quantity("30"),
                quantity("40"),
            ).unwrap(),
            EstimateQuality::Estimated,
        ).unwrap()).unwrap();

        assert_eq!(
            policy.compare(&first, &second).unwrap(),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn overlapping_ranges_are_not_arbitrarily_ordered() {
        let metric = CostMetric::ExecutionLatency;

        let mut policy = CostPolicy::new();

        policy.add_objective(CostObjective {
            metric: metric.clone(),
            direction: ObjectiveDirection::Minimize,
        }).unwrap();

        let mut first = CostVector::new();
        first.insert(CostEstimate::new(
            metric.clone(),
            unit("nanoseconds"),
            CostBound::range(
                quantity("10"),
                quantity("30"),
            ).unwrap(),
            EstimateQuality::Estimated,
        ).unwrap()).unwrap();

        let mut second = CostVector::new();
        second.insert(CostEstimate::new(
            metric,
            unit("nanoseconds"),
            CostBound::range(
                quantity("20"),
                quantity("40"),
            ).unwrap(),
            EstimateQuality::Estimated,
        ).unwrap()).unwrap();

        assert_eq!(
            policy.compare(&first, &second).unwrap(),
            None
        );
    }

    #[test]
    fn custom_future_domain_metrics_are_supported() {
        let metric = CostMetric::Custom(
            MetricId::new("future.photonic.operation_cost")
                .unwrap()
        );

        let estimate = estimate(
            metric.clone(),
            "100000000000000000000",
            "implementation_units",
        );

        assert_eq!(estimate.metric(), &metric);
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            COST_ANALYSIS_MODULE_NAME,
            "zamani.compiler.analysis.cost"
        );

        assert_eq!(COST_ANALYSIS_API_VERSION, 1);
    }
}
