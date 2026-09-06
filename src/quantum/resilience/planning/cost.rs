//! Zamani Quantum Resilience — Provider-Independent Planning Cost Model
//!
//! Path:
//!     src/quantum/resilience/planning/cost.rs
//!
//! Purpose:
//!     Provide a deterministic, multidimensional, provider-neutral cost model
//!     for resilience planning.
//!
//! Architectural position:
//!
//! ```text
//!                         Diagnosis
//!                            |
//!                            v
//!                         Policy
//!                            |
//!                            v
//!                  +---------------------+
//!                  |    Cost Provider    |
//!                  |                     |
//!                  | time                |
//!                  | shots               |
//!                  | energy              |
//!                  | qubits              |
//!                  | logical error       |
//!                  | financial cost      |
//!                  | compilation effort  |
//!                  | resource pressure   |
//!                  +----------+----------+
//!                             |
//!                             v
//!                         Feasibility
//!                             |
//!                             v
//!                          Ranking
//!                             |
//!                             v
//!                            Plan
//! ```
//!
//! This module describes the estimated consequences of a resilience action.
//! It does NOT decide whether an action is safe, feasible, desirable, or
//! executable.
//!
//! Those responsibilities belong to:
//!
//!     policy/*
//!     planning/feasibility.rs
//!     planning/ranking.rs
//!     planning/plan.rs
//!     recovery/*
//!     adaptation/*
//!     verification/*
//!
//! # Core architectural rule
//!
//! A cost is not a single scalar.
//!
//! Quantum resilience decisions can trade:
//!
//! - elapsed time;
//! - additional shots;
//! - physical/logical qubits;
//! - energy;
//! - logical error probability;
//! - financial/provider cost;
//! - compilation effort;
//! - execution overhead;
//! - resource pressure;
//! - migration overhead;
//! - verification overhead.
//!
//! Therefore this module represents cost as a vector of independent
//! dimensions and provides explicit mechanisms for policy-dependent
//! aggregation.
//!
//! # Write once, scale everywhere
//!
//! This module introduces NO machine-size limit.
//!
//! It must never contain assumptions such as:
//!
//!     MAX_QUBITS = 127
//!     MAX_SHOTS = 1_000_000
//!     MAX_BACKENDS = 10
//!     RETRY_COST = 3
//!
//! Quantum-resource quantities are represented using caller-supplied values.
//!
//! A machine with one qubit and a machine with an arbitrarily large number
//! of resources use exactly the same cost representation.
//!
//! Actual execution remains limited only by:
//!
//! - available memory;
//! - available hardware;
//! - provider capabilities;
//! - policy;
//! - numerical representation;
//! - execution budgets;
//! - operating-system/process limits;
//! - caller-supplied constraints.
//!
//! Those are environmental constraints, not architectural quantum limits.
//!
//! # Determinism
//!
//! Cost calculation is deterministic for identical normalized inputs.
//!
//! Floating-point NaN and infinity are rejected.
//!
//! Arithmetic that cannot be represented safely is reported as an error
//! rather than silently wrapping.
//!
//! Cost dimensions have stable ordering and stable identifiers.
//!
//! # Important semantic distinction
//!
//! Estimated cost is NOT the same as measured outcome.
//!
//!     CostEstimate
//!         = prediction / planning information
//!
//!     Execution telemetry
//!         = observed information
//!
//!     Verification
//!         = acceptance information
//!
//! A planner must never treat an estimate as proof that an execution will
//! succeed.
//!
//! # Integration
//!
//! Upstream:
//!
//!     planning/action.rs
//!     planning/feasibility.rs
//!     policy/objectives.rs
//!     policy/budgets.rs
//!     model/capability.rs
//!     model/resource.rs
//!     diagnosis/*
//!     hardware HAL
//!     routing
//!     scheduling
//!     optimization
//!     QEC
//!     benchmarking
//!
//! Downstream:
//!
//!     planning/ranking.rs
//!     planning/plan.rs
//!     planning/planner_state.rs
//!     planning/planner.rs
//!     adaptation/*
//!     recovery/*
//!     mitigation/*
//!     verification/*
//!
//! # Canonical quantum identity
//!
//! This file does not redefine qubit identity.
//!
//! If an integration requires explicit quantum-resource identity, callers
//! must use the canonical repository types, including:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and, where available in the repository contract:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Cost itself remains resource-identity-neutral because a cost estimate
//! should not require a particular hardware representation.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for resilience planning costs.
pub const COST_SCHEMA_ID: &str = "zamani.quantum.resilience.planning.cost";

/// Semantic version of the cost-model contract.
///
/// Increment when serialized or externally observable semantics change.
pub const COST_SCHEMA_VERSION: u16 = 1;

/// Implementation version of this cost model.
///
/// This identifies implementation semantics rather than machine capability.
pub const COST_MODEL_VERSION: u32 = 1;

// =============================================================================
// Numeric representation
// =============================================================================

/// Fixed decimal precision used by [`DecimalValue`].
///
/// Six decimal places provide deterministic representation without requiring
/// floating-point values for monetary, probability, timing, or weighted
/// planning calculations.
///
/// This is a representation precision, NOT a quantum-machine limit.
pub const DECIMAL_SCALE: u32 = 6;

/// Scaling factor for [`DecimalValue`].
const DECIMAL_FACTOR: i128 = 1_000_000;

// =============================================================================
// Decimal value
// =============================================================================

/// Deterministic signed fixed-point decimal.
///
/// The value is represented as:
///
///     units / 1_000_000
///
/// This avoids NaN, infinity, and platform-dependent floating-point ordering.
///
/// The type is intentionally small and copyable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalValue {
    units: i128,
}

impl DecimalValue {
    /// Zero.
    pub const ZERO: Self = Self { units: 0 };

    /// One.
    pub const ONE: Self = Self {
        units: DECIMAL_FACTOR,
    };

    /// Creates a decimal from an integer.
    #[must_use]
    pub const fn from_integer(value: i128) -> Self {
        Self {
            units: value.saturating_mul(DECIMAL_FACTOR),
        }
    }

    /// Creates a non-negative decimal from a scaled integer.
    ///
    /// `units` is already expressed at [`DECIMAL_SCALE`] precision.
    #[must_use]
    pub const fn from_scaled_units(units: i128) -> Self {
        Self { units }
    }

    /// Returns the internal scaled representation.
    #[must_use]
    pub const fn scaled_units(self) -> i128 {
        self.units
    }

    /// Returns whether the value is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.units == 0
    }

    /// Returns whether the value is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.units < 0
    }

    /// Adds two values with overflow checking.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.units.checked_add(other.units) {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Subtracts two values with overflow checking.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.units.checked_sub(other.units) {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Multiplies two values and preserves the fixed-point scale.
    ///
    /// The calculation is:
    ///
    ///     (a * b) / DECIMAL_FACTOR
    ///
    /// with checked intermediate arithmetic.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        match self.units.checked_mul(other.units) {
            Some(product) => Some(Self {
                units: product / DECIMAL_FACTOR,
            }),
            None => None,
        }
    }

    /// Divides two values and preserves the fixed-point scale.
    pub const fn checked_div(self, other: Self) -> Option<Self> {
        if other.units == 0 {
            return None;
        }

        match self.units.checked_mul(DECIMAL_FACTOR) {
            Some(numerator) => Some(Self {
                units: numerator / other.units,
            }),
            None => None,
        }
    }

    /// Returns the absolute value with overflow checking.
    pub const fn checked_abs(self) -> Option<Self> {
        match self.units.checked_abs() {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Clamps a value to a non-negative domain.
    #[must_use]
    pub const fn max_zero(self) -> Self {
        if self.units < 0 {
            Self::ZERO
        } else {
            self
        }
    }
}

impl Default for DecimalValue {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialOrd for DecimalValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DecimalValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.units.cmp(&other.units)
    }
}

impl fmt::Display for DecimalValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative = self.units < 0;
        let magnitude = self.units.unsigned_abs();

        let whole = magnitude / DECIMAL_FACTOR as u128;
        let fraction = magnitude % DECIMAL_FACTOR as u128;

        if negative {
            write!(formatter, "-")?;
        }

        write!(formatter, "{whole}.{fraction:06}")
    }
}

// =============================================================================
// Cost dimension
// =============================================================================

/// Stable cost dimensions.
///
/// The ordering is intentionally stable because ranking, serialization and
/// deterministic diagnostics may depend on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostDimension {
    /// Estimated elapsed execution time.
    Time,

    /// Additional measurement shots.
    Shots,

    /// Additional physical/logical qubit pressure.
    Qubits,

    /// Estimated energy/resource consumption.
    Energy,

    /// Estimated probability contribution to logical failure.
    LogicalErrorProbability,

    /// Provider-independent financial/resource expenditure.
    Financial,

    /// Compilation/transformation effort.
    Compilation,

    /// Routing overhead.
    Routing,

    /// Scheduling overhead.
    Scheduling,

    /// QEC overhead.
    Qec,

    /// Error-mitigation overhead.
    Mitigation,

    /// Backend/device migration overhead.
    Migration,

    /// Verification overhead.
    Verification,

    /// Generic resource pressure.
    ResourcePressure,
}

impl CostDimension {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Shots => "shots",
            Self::Qubits => "qubits",
            Self::Energy => "energy",
            Self::LogicalErrorProbability => "logical_error_probability",
            Self::Financial => "financial",
            Self::Compilation => "compilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Qec => "qec",
            Self::Mitigation => "mitigation",
            Self::Migration => "migration",
            Self::Verification => "verification",
            Self::ResourcePressure => "resource_pressure",
        }
    }

    /// Returns all dimensions in deterministic order.
    #[must_use]
    pub const fn all() -> [Self; 14] {
        [
            Self::Time,
            Self::Shots,
            Self::Qubits,
            Self::Energy,
            Self::LogicalErrorProbability,
            Self::Financial,
            Self::Compilation,
            Self::Routing,
            Self::Scheduling,
            Self::Qec,
            Self::Mitigation,
            Self::Migration,
            Self::Verification,
            Self::ResourcePressure,
        ]
    }
}

// =============================================================================
// Cost value
// =============================================================================

/// A single cost measurement.
///
/// Values are deliberately unit-neutral at this layer.
///
/// The producer must define the unit semantics through its cost-model
/// contract. This allows different hardware and execution providers to use
/// their native measurement systems while resilience ranking remains
/// provider-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CostValue {
    value: DecimalValue,
}

impl CostValue {
    /// Creates a non-negative cost value.
    pub fn new(value: DecimalValue) -> CostResult<Self> {
        if value.is_negative() {
            return Err(CostError::NegativeCost);
        }

        Ok(Self { value })
    }

    /// Creates a zero cost.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            value: DecimalValue::ZERO,
        }
    }

    /// Creates a cost from a non-negative integer.
    pub fn from_integer(value: i128) -> CostResult<Self> {
        if value < 0 {
            return Err(CostError::NegativeCost);
        }

        Self::new(DecimalValue::from_integer(value))
    }

    /// Creates a cost from fixed-point units.
    pub fn from_scaled_units(units: i128) -> CostResult<Self> {
        Self::new(DecimalValue::from_scaled_units(units))
    }

    /// Returns the value.
    #[must_use]
    pub const fn value(self) -> DecimalValue {
        self.value
    }

    /// Returns the fixed-point representation.
    #[must_use]
    pub const fn scaled_units(self) -> i128 {
        self.value.scaled_units()
    }

    /// Adds costs.
    pub fn checked_add(self, other: Self) -> CostResult<Self> {
        let value = self
            .value
            .checked_add(other.value)
            .ok_or(CostError::ArithmeticOverflow {
                operation: "cost addition",
            })?;

        Self::new(value)
    }

    /// Multiplies a cost by a non-negative factor.
    pub fn checked_mul(self, factor: DecimalValue) -> CostResult<Self> {
        if factor.is_negative() {
            return Err(CostError::NegativeMultiplier);
        }

        let value = self
            .value
            .checked_mul(factor)
            .ok_or(CostError::ArithmeticOverflow {
                operation: "cost multiplication",
            })?;

        Self::new(value)
    }
}

// =============================================================================
// Cost vector
// =============================================================================

/// Multidimensional resilience cost.
///
/// This is the canonical cost representation passed between planner
/// components.
///
/// It deliberately avoids:
///
///     f64
///     NaN
///     infinity
///     provider-specific currencies
///     provider-specific resource identifiers
///     fixed machine dimensions
///
/// The representation contains one optional value per semantic dimension.
/// Missing dimensions mean that the estimator has no value for that
/// dimension; they do NOT mean zero.
///
/// This distinction is important.
///
/// `None` means:
///
///     unknown / not estimated
///
/// `Some(0)` means:
///
///     explicitly estimated to have no additional cost
///
/// A ranking policy must decide how unknown dimensions are treated.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CostVector {
    values: [Option<CostValue>; 14],
}

impl Default for CostVector {
    fn default() -> Self {
        Self::new()
    }
}

impl CostVector {
    /// Creates an empty cost vector.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: [None; 14],
        }
    }

    /// Associates a cost with a dimension.
    pub fn set(
        &mut self,
        dimension: CostDimension,
        value: CostValue,
    ) {
        self.values[dimension as usize] = Some(value);
    }

    /// Returns the cost for a dimension.
    #[must_use]
    pub const fn get(&self, dimension: CostDimension) -> Option<CostValue> {
        self.values[dimension as usize]
    }

    /// Removes a dimension.
    pub fn clear(&mut self, dimension: CostDimension) {
        self.values[dimension as usize] = None;
    }

    /// Returns whether the dimension has an explicit value.
    #[must_use]
    pub const fn contains(&self, dimension: CostDimension) -> bool {
        self.values[dimension as usize].is_some()
    }

    /// Returns the number of explicitly estimated dimensions.
    #[must_use]
    pub fn known_dimensions(&self) -> usize {
        self.values.iter().filter(|value| value.is_some()).count()
    }

    /// Returns whether every semantic dimension has an estimate.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.values.iter().all(|value| value.is_some())
    }

    /// Returns whether no dimensions are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|value| value.is_none())
    }

    /// Iterates over known dimensions in stable order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (CostDimension, CostValue)> + '_ {
        CostDimension::all()
            .into_iter()
            .filter_map(|dimension| {
                self.get(dimension).map(|value| (dimension, value))
            })
    }

    /// Adds two cost vectors dimension by dimension.
    ///
    /// If one side has an unknown dimension and the other side has a known
    /// value, the result remains unknown.
    ///
    /// This is intentional: unknown + known is not safely equivalent to
    /// known.
    pub fn checked_add(&self, other: &Self) -> CostResult<Self> {
        let mut result = Self::new();

        for dimension in CostDimension::all() {
            match (self.get(dimension), other.get(dimension)) {
                (Some(left), Some(right)) => {
                    result.set(dimension, left.checked_add(right)?);
                }
                _ => {}
            }
        }

        Ok(result)
    }

    /// Returns the sum of all known dimensions.
    ///
    /// This is primarily a diagnostic helper.
    ///
    /// It must NOT be used as a universal ranking function because different
    /// dimensions have different semantics.
    pub fn checked_sum_known(&self) -> CostResult<CostValue> {
        let mut result = CostValue::zero();

        for (_, value) in self.iter() {
            result = result.checked_add(value)?;
        }

        Ok(result)
    }
}

// =============================================================================
// Cost weights
// =============================================================================

/// Policy-controlled weight for one cost dimension.
///
/// A weight of zero means that the dimension does not contribute to a
/// scalarized ranking score.
///
/// This does NOT mean that the dimension is irrelevant to feasibility or
/// safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostWeight {
    /// Weight used during policy-specific scalarization.
    value: DecimalValue,
}

impl CostWeight {
    /// Creates a non-negative weight.
    pub fn new(value: DecimalValue) -> CostResult<Self> {
        if value.is_negative() {
            return Err(CostError::NegativeWeight);
        }

        Ok(Self { value })
    }

    /// Zero weight.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            value: DecimalValue::ZERO,
        }
    }

    /// Unit weight.
    #[must_use]
    pub const fn one() -> Self {
        Self {
            value: DecimalValue::ONE,
        }
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> DecimalValue {
        self.value
    }
}

impl Default for CostWeight {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Cost objective
// =============================================================================

/// Policy-controlled scalarization objective.
///
/// Scalarization is intentionally optional.
///
/// A Pareto/vector comparison is generally safer for multi-objective
/// resilience than blindly collapsing everything into one scalar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostObjective {
    weights: [CostWeight; 14],
    reject_unknown: bool,
}

impl Default for CostObjective {
    fn default() -> Self {
        Self::new()
    }
}

impl CostObjective {
    /// Creates an objective with zero weights.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            weights: [CostWeight::zero(); 14],
            reject_unknown: true,
        }
    }

    /// Sets the weight for a dimension.
    pub fn set_weight(
        &mut self,
        dimension: CostDimension,
        weight: CostWeight,
    ) {
        self.weights[dimension as usize] = weight;
    }

    /// Returns the configured weight.
    #[must_use]
    pub const fn weight(&self, dimension: CostDimension) -> CostWeight {
        self.weights[dimension as usize]
    }

    /// Configures whether missing dimensions cause scalarization to fail.
    pub fn set_reject_unknown(&mut self, reject_unknown: bool) {
        self.reject_unknown = reject_unknown;
    }

    /// Returns whether unknown dimensions are rejected.
    #[must_use]
    pub const fn reject_unknown(&self) -> bool {
        self.reject_unknown
    }

    /// Calculates a deterministic weighted score.
    ///
    /// Only dimensions with a non-zero weight participate.
    ///
    /// If a participating dimension is unknown and `reject_unknown` is true,
    /// an error is returned.
    pub fn score(&self, vector: &CostVector) -> CostResult<CostValue> {
        let mut score = CostValue::zero();

        for dimension in CostDimension::all() {
            let weight = self.weight(dimension);

            if weight.value().is_zero() {
                continue;
            }

            let value = match vector.get(dimension) {
                Some(value) => value,
                None if self.reject_unknown => {
                    return Err(CostError::UnknownDimension {
                        dimension,
                    });
                }
                None => continue,
            };

            let contribution = value.checked_mul(weight.value())?;

            score = score.checked_add(contribution)?;
        }

        Ok(score)
    }
}

// =============================================================================
// Cost confidence
// =============================================================================

/// Confidence level of an estimated cost.
///
/// Cost confidence is distinct from diagnosis confidence and verification
/// confidence. It describes confidence in the numerical estimate itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostConfidence {
    /// No meaningful estimate is available.
    Unknown,

    /// Estimate has weak evidence.
    Low,

    /// Estimate has reasonable evidence.
    Medium,

    /// Estimate has strong evidence.
    High,

    /// Estimate is directly measured or strongly established.
    Confirmed,
}

impl CostConfidence {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Confirmed => "confirmed",
        }
    }
}

impl Default for CostConfidence {
    fn default() -> Self {
        Self::Unknown
    }
}

// =============================================================================
// Cost provenance
// =============================================================================

/// Source category for a cost estimate.
///
/// This does not encode provider names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostSource {
    /// Static analytical estimation.
    Analytical,

    /// Historical observations.
    Historical,

    /// Hardware capability/resource model.
    CapabilityModel,

    /// Benchmark-derived estimate.
    Benchmark,

    /// Runtime observation.
    RuntimeObservation,

    /// Caller-supplied estimate.
    CallerProvided,

    /// Composite estimate from multiple sources.
    Composite,
}

impl CostSource {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Analytical => "analytical",
            Self::Historical => "historical",
            Self::CapabilityModel => "capability_model",
            Self::Benchmark => "benchmark",
            Self::RuntimeObservation => "runtime_observation",
            Self::CallerProvided => "caller_provided",
            Self::Composite => "composite",
        }
    }
}

// =============================================================================
// Cost estimate
// =============================================================================

/// Complete multidimensional estimate for one planned operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostEstimate {
    /// Cost vector.
    vector: CostVector,

    /// Confidence in the estimate.
    confidence: CostConfidence,

    /// Primary source of the estimate.
    source: CostSource,

    /// Whether the estimate is lower-bound, upper-bound, expected, or exact.
    bound: CostBound,

    /// Stable estimator identifier.
    estimator_id: Option<String>,
}

impl Default for CostEstimate {
    fn default() -> Self {
        Self::unknown()
    }
}

impl CostEstimate {
    /// Creates an unknown estimate.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            vector: CostVector::new(),
            confidence: CostConfidence::Unknown,
            source: CostSource::Analytical,
            bound: CostBound::Unknown,
            estimator_id: None,
        }
    }

    /// Creates an estimate from a cost vector.
    #[must_use]
    pub fn new(vector: CostVector) -> Self {
        Self {
            vector,
            confidence: CostConfidence::Unknown,
            source: CostSource::Analytical,
            bound: CostBound::Unknown,
            estimator_id: None,
        }
    }

    /// Returns the cost vector.
    #[must_use]
    pub const fn vector(&self) -> &CostVector {
        &self.vector
    }

    /// Returns confidence.
    #[must_use]
    pub const fn confidence(&self) -> CostConfidence {
        self.confidence
    }

    /// Sets confidence.
    pub fn with_confidence(mut self, confidence: CostConfidence) -> Self {
        self.confidence = confidence;
        self
    }

    /// Returns the source.
    #[must_use]
    pub const fn source(&self) -> CostSource {
        self.source
    }

    /// Sets source.
    pub fn with_source(mut self, source: CostSource) -> Self {
        self.source = source;
        self
    }

    /// Returns the estimate bound.
    #[must_use]
    pub const fn bound(&self) -> CostBound {
        self.bound
    }

    /// Sets the estimate bound.
    pub fn with_bound(mut self, bound: CostBound) -> Self {
        self.bound = bound;
        self
    }

    /// Sets a stable estimator identifier.
    ///
    /// The identifier must be non-empty.
    pub fn with_estimator_id(
        mut self,
        estimator_id: impl Into<String>,
    ) -> CostResult<Self> {
        let id = estimator_id.into();

        if id.trim().is_empty() {
            return Err(CostError::InvalidEstimatorId);
        }

        self.estimator_id = Some(id);
        Ok(self)
    }

    /// Returns the estimator identifier.
    #[must_use]
    pub fn estimator_id(&self) -> Option<&str> {
        self.estimator_id.as_deref()
    }

    /// Returns whether the estimate contains at least one known dimension.
    #[must_use]
    pub fn is_known(&self) -> bool {
        !self.vector.is_empty()
    }

    /// Adds two estimates.
    ///
    /// Metadata is retained only when it can be combined without pretending
    /// to have stronger evidence than actually exists.
    pub fn checked_add(&self, other: &Self) -> CostResult<Self> {
        let vector = self.vector.checked_add(&other.vector)?;

        let confidence = if self.confidence < other.confidence {
            self.confidence
        } else {
            other.confidence
        };

        let source = if self.source == other.source {
            self.source
        } else {
            CostSource::Composite
        };

        Ok(Self {
            vector,
            confidence,
            source,
            bound: CostBound::Expected,
            estimator_id: None,
        })
    }
}

// =============================================================================
// Cost bound
// =============================================================================

/// Semantic interpretation of an estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostBound {
    /// No bound semantics are known.
    Unknown,

    /// Estimate is a lower bound.
    Lower,

    /// Estimate is an upper bound.
    Upper,

    /// Estimate represents an expected value.
    Expected,

    /// Estimate is directly measured/exact within the declared precision.
    Exact,
}

impl CostBound {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Lower => "lower",
            Self::Upper => "upper",
            Self::Expected => "expected",
            Self::Exact => "exact",
        }
    }
}

// =============================================================================
// Action cost
// =============================================================================

/// Cost estimate associated with one resilience action.
///
/// This type intentionally does not depend on the concrete `Action` type in
/// `planning/action.rs`. That avoids a circular dependency between the
/// semantic action vocabulary and its numerical planning model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionCost {
    /// Stable action identifier.
    action_id: String,

    /// Estimated cost.
    estimate: CostEstimate,
}

impl ActionCost {
    /// Creates an action cost.
    pub fn new(
        action_id: impl Into<String>,
        estimate: CostEstimate,
    ) -> CostResult<Self> {
        let action_id = action_id.into();

        validate_identifier(&action_id)?;

        Ok(Self {
            action_id,
            estimate,
        })
    }

    /// Returns the action identifier.
    #[must_use]
    pub fn action_id(&self) -> &str {
        &self.action_id
    }

    /// Returns the estimate.
    #[must_use]
    pub const fn estimate(&self) -> &CostEstimate {
        &self.estimate
    }
}

// =============================================================================
// Plan cost
// =============================================================================

/// Aggregate cost for an entire resilience plan.
///
/// It contains both:
///
/// - aggregate multidimensional cost;
/// - per-action costs.
///
/// Keeping both allows:
///
/// - ranking;
/// - audit;
/// - explanation;
/// - provenance;
/// - debugging;
/// - post-execution comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanCost {
    /// Aggregate cost.
    total: CostEstimate,

    /// Costs for individual actions.
    actions: Vec<ActionCost>,
}

impl PlanCost {
    /// Creates an empty plan cost.
    #[must_use]
    pub fn new() -> Self {
        Self {
            total: CostEstimate::unknown(),
            actions: Vec::new(),
        }
    }

    /// Creates a plan cost from an aggregate estimate.
    #[must_use]
    pub fn from_total(total: CostEstimate) -> Self {
        Self {
            total,
            actions: Vec::new(),
        }
    }

    /// Adds an action cost and recomputes the aggregate estimate.
    pub fn push(&mut self, action_cost: ActionCost) -> CostResult<()> {
        let next_total = if self.actions.is_empty() && !self.total.is_known() {
            action_cost.estimate().clone()
        } else {
            self.total.checked_add(action_cost.estimate())?
        };

        self.actions.push(action_cost);
        self.total = next_total;

        Ok(())
    }

    /// Returns the aggregate estimate.
    #[must_use]
    pub const fn total(&self) -> &CostEstimate {
        &self.total
    }

    /// Returns individual action costs.
    #[must_use]
    pub fn actions(&self) -> &[ActionCost] {
        &self.actions
    }

    /// Returns the number of action estimates.
    #[must_use]
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Returns whether the plan has no action estimates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Calculates a policy-specific scalar score.
    pub fn score(
        &self,
        objective: &CostObjective,
    ) -> CostResult<CostValue> {
        objective.score(self.total.vector())
    }
}

impl Default for PlanCost {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Pareto comparison
// =============================================================================

/// Result of comparing two multidimensional costs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CostRelation {
    /// Left cost is strictly better in at least one dimension and no worse
    /// in any compared dimension.
    Dominates,

    /// Right cost dominates left.
    Dominated,

    /// Neither cost dominates the other.
    Tradeoff,

    /// Both costs are equal across all compared dimensions.
    Equal,

    /// Comparison cannot be established because relevant dimensions are
    /// missing.
    Incomparable,
}

/// Compares two cost vectors using Pareto dominance.
///
/// Only dimensions known in BOTH vectors are compared.
///
/// If there are no common dimensions, the result is `Incomparable`.
///
/// This function does not invent values for unknown dimensions.
#[must_use]
pub fn pareto_compare(
    left: &CostVector,
    right: &CostVector,
) -> CostRelation {
    let mut comparable = false;
    let mut left_better = false;
    let mut right_better = false;

    for dimension in CostDimension::all() {
        let (left_value, right_value) =
            match (left.get(dimension), right.get(dimension)) {
                (Some(left_value), Some(right_value)) => {
                    (left_value, right_value)
                }
                _ => continue,
            };

        comparable = true;

        match left_value.cmp(&right_value) {
            Ordering::Less => left_better = true,
            Ordering::Greater => right_better = true,
            Ordering::Equal => {}
        }
    }

    if !comparable {
        return CostRelation::Incomparable;
    }

    match (left_better, right_better) {
        (false, false) => CostRelation::Equal,
        (true, false) => CostRelation::Dominates,
        (false, true) => CostRelation::Dominated,
        (true, true) => CostRelation::Tradeoff,
    }
}

// =============================================================================
// Cost range
// =============================================================================

/// A non-negative interval for uncertain cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostRange {
    /// Minimum plausible cost.
    minimum: CostValue,

    /// Maximum plausible cost.
    maximum: CostValue,
}

impl CostRange {
    /// Creates a range.
    pub fn new(
        minimum: CostValue,
        maximum: CostValue,
    ) -> CostResult<Self> {
        if minimum > maximum {
            return Err(CostError::InvalidRange);
        }

        Ok(Self {
            minimum,
            maximum,
        })
    }

    /// Returns the minimum.
    #[must_use]
    pub const fn minimum(&self) -> CostValue {
        self.minimum
    }

    /// Returns the maximum.
    #[must_use]
    pub const fn maximum(&self) -> CostValue {
        self.maximum
    }
}

// =============================================================================
// Cost budget
// =============================================================================

/// A policy-controlled budget for one dimension.
///
/// This is a planning constraint, not a hardware limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostBudget {
    /// Dimension controlled by this budget.
    dimension: CostDimension,

    /// Maximum permitted cost.
    maximum: CostValue,
}

impl CostBudget {
    /// Creates a budget.
    #[must_use]
    pub const fn new(
        dimension: CostDimension,
        maximum: CostValue,
    ) -> Self {
        Self {
            dimension,
            maximum,
        }
    }

    /// Returns the dimension.
    #[must_use]
    pub const fn dimension(&self) -> CostDimension {
        self.dimension
    }

    /// Returns the maximum.
    #[must_use]
    pub const fn maximum(&self) -> CostValue {
        self.maximum
    }

    /// Checks an estimate against the budget.
    ///
    /// Unknown dimensions are reported as unknown rather than silently
    /// treated as zero.
    #[must_use]
    pub fn check(&self, estimate: &CostEstimate) -> BudgetResult {
        match estimate.vector().get(self.dimension) {
            None => BudgetResult::Unknown,
            Some(value) if value <= self.maximum => BudgetResult::Within,
            Some(_) => BudgetResult::Exceeded,
        }
    }
}

/// Result of a budget check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetResult {
    /// Estimate is within budget.
    Within,

    /// Estimate exceeds budget.
    Exceeded,

    /// Estimate does not contain this dimension.
    Unknown,
}

// =============================================================================
// Cost context
// =============================================================================

/// Context used by a cost estimator.
///
/// The context deliberately contains no concrete hardware implementation.
///
/// Integrations can construct it from:
///
///     quantum::hardware
///     quantum::routing
///     quantum::scheduling
///     quantum::optimization
///     quantum::qec
///     quantum::benchmarking
///     resilience::state
///
/// The actual resource quantities remain caller supplied.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CostContext {
    /// Number of operations affected by the candidate.
    affected_operations: Option<u64>,

    /// Number of additional shots.
    additional_shots: Option<u64>,

    /// Number of additional qubits/resources required.
    additional_qubits: Option<u64>,

    /// Additional execution time in implementation-defined units.
    additional_time: Option<CostValue>,

    /// Additional compilation work.
    compilation_work: Option<CostValue>,

    /// Additional routing work.
    routing_work: Option<CostValue>,

    /// Additional scheduling work.
    scheduling_work: Option<CostValue>,

    /// Additional QEC work.
    qec_work: Option<CostValue>,

    /// Additional mitigation work.
    mitigation_work: Option<CostValue>,

    /// Additional migration work.
    migration_work: Option<CostValue>,

    /// Additional verification work.
    verification_work: Option<CostValue>,

    /// Additional energy estimate.
    energy: Option<CostValue>,

    /// Additional financial/resource expenditure.
    financial: Option<CostValue>,

    /// Estimated logical error contribution.
    logical_error_probability: Option<CostValue>,

    /// Generic resource pressure.
    resource_pressure: Option<CostValue>,
}

impl CostContext {
    /// Creates an empty context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            affected_operations: None,
            additional_shots: None,
            additional_qubits: None,
            additional_time: None,
            compilation_work: None,
            routing_work: None,
            scheduling_work: None,
            qec_work: None,
            mitigation_work: None,
            migration_work: None,
            verification_work: None,
            energy: None,
            financial: None,
            logical_error_probability: None,
            resource_pressure: None,
        }
    }

    /// Sets affected operation count.
    pub fn with_affected_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.affected_operations = Some(value);
        self
    }

    /// Sets additional shots.
    pub fn with_additional_shots(
        mut self,
        value: u64,
    ) -> Self {
        self.additional_shots = Some(value);
        self
    }

    /// Sets additional qubits/resources.
    pub fn with_additional_qubits(
        mut self,
        value: u64,
    ) -> Self {
        self.additional_qubits = Some(value);
        self
    }

    /// Sets additional time.
    pub fn with_additional_time(
        mut self,
        value: CostValue,
    ) -> Self {
        self.additional_time = Some(value);
        self
    }

    /// Sets compilation work.
    pub fn with_compilation_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.compilation_work = Some(value);
        self
    }

    /// Sets routing work.
    pub fn with_routing_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.routing_work = Some(value);
        self
    }

    /// Sets scheduling work.
    pub fn with_scheduling_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.scheduling_work = Some(value);
        self
    }

    /// Sets QEC work.
    pub fn with_qec_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.qec_work = Some(value);
        self
    }

    /// Sets mitigation work.
    pub fn with_mitigation_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.mitigation_work = Some(value);
        self
    }

    /// Sets migration work.
    pub fn with_migration_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.migration_work = Some(value);
        self
    }

    /// Sets verification work.
    pub fn with_verification_work(
        mut self,
        value: CostValue,
    ) -> Self {
        self.verification_work = Some(value);
        self
    }

    /// Sets energy.
    pub fn with_energy(
        mut self,
        value: CostValue,
    ) -> Self {
        self.energy = Some(value);
        self
    }

    /// Sets financial/resource expenditure.
    pub fn with_financial(
        mut self,
        value: CostValue,
    ) -> Self {
        self.financial = Some(value);
        self
    }

    /// Sets logical-error probability contribution.
    pub fn with_logical_error_probability(
        mut self,
        value: CostValue,
    ) -> Self {
        self.logical_error_probability = Some(value);
        self
    }

    /// Sets resource pressure.
    pub fn with_resource_pressure(
        mut self,
        value: CostValue,
    ) -> Self {
        self.resource_pressure = Some(value);
        self
    }

    /// Returns affected operation count.
    #[must_use]
    pub const fn affected_operations(&self) -> Option<u64> {
        self.affected_operations
    }

    /// Returns additional shots.
    #[must_use]
    pub const fn additional_shots(&self) -> Option<u64> {
        self.additional_shots
    }

    /// Returns additional qubits.
    #[must_use]
    pub const fn additional_qubits(&self) -> Option<u64> {
        self.additional_qubits
    }

    /// Returns additional time.
    #[must_use]
    pub const fn additional_time(&self) -> Option<CostValue> {
        self.additional_time
    }

    /// Returns compilation work.
    #[must_use]
    pub const fn compilation_work(&self) -> Option<CostValue> {
        self.compilation_work
    }

    /// Returns routing work.
    #[must_use]
    pub const fn routing_work(&self) -> Option<CostValue> {
        self.routing_work
    }

    /// Returns scheduling work.
    #[must_use]
    pub const fn scheduling_work(&self) -> Option<CostValue> {
        self.scheduling_work
    }

    /// Returns QEC work.
    #[must_use]
    pub const fn qec_work(&self) -> Option<CostValue> {
        self.qec_work
    }

    /// Returns mitigation work.
    #[must_use]
    pub const fn mitigation_work(&self) -> Option<CostValue> {
        self.mitigation_work
    }

    /// Returns migration work.
    #[must_use]
    pub const fn migration_work(&self) -> Option<CostValue> {
        self.migration_work
    }

    /// Returns verification work.
    #[must_use]
    pub const fn verification_work(&self) -> Option<CostValue> {
        self.verification_work
    }

    /// Returns energy.
    #[must_use]
    pub const fn energy(&self) -> Option<CostValue> {
        self.energy
    }

    /// Returns financial cost.
    #[must_use]
    pub const fn financial(&self) -> Option<CostValue> {
        self.financial
    }

    /// Returns logical-error probability contribution.
    #[must_use]
    pub const fn logical_error_probability(&self) -> Option<CostValue> {
        self.logical_error_probability
    }

    /// Returns resource pressure.
    #[must_use]
    pub const fn resource_pressure(&self) -> Option<CostValue> {
        self.resource_pressure
    }
}

// =============================================================================
// Cost estimator trait
// =============================================================================

/// Provider-neutral cost estimator.
///
/// Implementations may use:
///
/// - analytical models;
/// - historical measurements;
/// - hardware capabilities;
/// - routing estimates;
/// - scheduling estimates;
/// - QEC estimates;
/// - benchmarking data;
/// - runtime observations.
///
/// The estimator must not execute the action.
///
/// It only estimates consequences.
pub trait CostEstimator {
    /// Stable estimator identifier.
    fn id(&self) -> &'static str;

    /// Estimate the cost of a candidate action.
    ///
    /// `action_id` is a stable semantic identifier from
    /// `planning/action.rs`.
    fn estimate(
        &self,
        action_id: &str,
        context: &CostContext,
    ) -> CostResult<CostEstimate>;
}

// =============================================================================
// Basic analytical estimator
// =============================================================================

/// A deterministic estimator that maps explicitly supplied context values
/// into the canonical cost vector.
///
/// This implementation is intentionally conservative:
///
/// - it never invents hardware characteristics;
/// - it never assumes a retry count;
/// - it never assumes a qubit count;
/// - it never assumes provider pricing;
/// - it never assumes a fidelity threshold.
#[derive(Debug, Clone, Copy, Default)]
pub struct ContextCostEstimator;

impl ContextCostEstimator {
    /// Creates the estimator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl CostEstimator for ContextCostEstimator {
    fn id(&self) -> &'static str {
        "context"
    }

    fn estimate(
        &self,
        action_id: &str,
        context: &CostContext,
    ) -> CostResult<CostEstimate> {
        validate_identifier(action_id)?;

        let mut vector = CostVector::new();

        if let Some(value) = context.additional_time() {
            vector.set(CostDimension::Time, value);
        }

        if let Some(value) = context.additional_shots() {
            vector.set(
                CostDimension::Shots,
                CostValue::from_integer(i128::from(value))?,
            );
        }

        if let Some(value) = context.additional_qubits() {
            vector.set(
                CostDimension::Qubits,
                CostValue::from_integer(i128::from(value))?,
            );
        }

        if let Some(value) = context.energy() {
            vector.set(CostDimension::Energy, value);
        }

        if let Some(value) = context.logical_error_probability() {
            vector.set(
                CostDimension::LogicalErrorProbability,
                value,
            );
        }

        if let Some(value) = context.financial() {
            vector.set(CostDimension::Financial, value);
        }

        if let Some(value) = context.compilation_work() {
            vector.set(CostDimension::Compilation, value);
        }

        if let Some(value) = context.routing_work() {
            vector.set(CostDimension::Routing, value);
        }

        if let Some(value) = context.scheduling_work() {
            vector.set(CostDimension::Scheduling, value);
        }

        if let Some(value) = context.qec_work() {
            vector.set(CostDimension::Qec, value);
        }

        if let Some(value) = context.mitigation_work() {
            vector.set(CostDimension::Mitigation, value);
        }

        if let Some(value) = context.migration_work() {
            vector.set(CostDimension::Migration, value);
        }

        if let Some(value) = context.verification_work() {
            vector.set(CostDimension::Verification, value);
        }

        if let Some(value) = context.resource_pressure() {
            vector.set(CostDimension::ResourcePressure, value);
        }

        Ok(CostEstimate::new(vector)
            .with_source(CostSource::CallerProvided)
            .with_confidence(CostConfidence::Medium)
            .with_bound(CostBound::Expected))
    }
}

// =============================================================================
// Cost model composition
// =============================================================================

/// Combines multiple independent cost estimators.
///
/// Estimators are composed without imposing a provider-specific architecture.
#[derive(Default)]
pub struct CompositeCostEstimator<E> {
    estimators: Vec<E>,
}

impl<E> CompositeCostEstimator<E> {
    /// Creates an empty composite estimator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            estimators: Vec::new(),
        }
    }

    /// Adds an estimator.
    pub fn push(&mut self, estimator: E) {
        self.estimators.push(estimator);
    }

    /// Returns the number of estimators.
    #[must_use]
    pub fn len(&self) -> usize {
        self.estimators.len()
    }

    /// Returns whether no estimators are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.estimators.is_empty()
    }

    /// Returns registered estimators.
    #[must_use]
    pub fn estimators(&self) -> &[E] {
        &self.estimators
    }
}

impl<E: CostEstimator> CostEstimator for CompositeCostEstimator<E> {
    fn id(&self) -> &'static str {
        "composite"
    }

    fn estimate(
        &self,
        action_id: &str,
        context: &CostContext,
    ) -> CostResult<CostEstimate> {
        validate_identifier(action_id)?;

        let mut aggregate: Option<CostEstimate> = None;

        for estimator in &self.estimators {
            let estimate = estimator.estimate(action_id, context)?;

            aggregate = Some(match aggregate {
                Some(current) => current.checked_add(&estimate)?,
                None => estimate,
            });
        }

        aggregate.ok_or(CostError::NoEstimator)
    }
}

// =============================================================================
// Cost errors
// =============================================================================

/// Errors produced by the resilience cost model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostError {
    /// A negative cost was supplied.
    NegativeCost,

    /// A negative multiplier was supplied.
    NegativeMultiplier,

    /// A negative weight was supplied.
    NegativeWeight,

    /// Arithmetic could not be represented safely.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// An unknown dimension was required for scalarization.
    UnknownDimension {
        /// Missing dimension.
        dimension: CostDimension,
    },

    /// A range has minimum greater than maximum.
    InvalidRange,

    /// An estimator identifier is invalid.
    InvalidEstimatorId,

    /// An action identifier is invalid.
    InvalidActionIdentifier,

    /// No estimator was registered.
    NoEstimator,
}

impl fmt::Display for CostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeCost => {
                write!(formatter, "cost cannot be negative")
            }

            Self::NegativeMultiplier => {
                write!(formatter, "cost multiplier cannot be negative")
            }

            Self::NegativeWeight => {
                write!(formatter, "cost weight cannot be negative")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "cost arithmetic overflow during {operation}"
                )
            }

            Self::UnknownDimension { dimension } => {
                write!(
                    formatter,
                    "required cost dimension `{}` is unknown",
                    dimension.as_str()
                )
            }

            Self::InvalidRange => {
                write!(formatter, "cost range minimum exceeds maximum")
            }

            Self::InvalidEstimatorId => {
                write!(formatter, "estimator identifier cannot be empty")
            }

            Self::InvalidActionIdentifier => {
                write!(formatter, "action identifier cannot be empty")
            }

            Self::NoEstimator => {
                write!(formatter, "no cost estimator is registered")
            }
        }
    }
}

impl std::error::Error for CostError {}

/// Result type for cost-model operations.
pub type CostResult<T> = Result<T, CostError>;

// =============================================================================
// Utility functions
// =============================================================================

/// Validates a stable semantic identifier.
fn validate_identifier(value: &str) -> CostResult<()> {
    if value.trim().is_empty() {
        return Err(CostError::InvalidActionIdentifier);
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_ordering_is_deterministic() {
        let low = DecimalValue::from_integer(1);
        let high = DecimalValue::from_integer(2);

        assert!(low < high);
        assert_eq!(low.cmp(&low), Ordering::Equal);
    }

    #[test]
    fn negative_cost_is_rejected() {
        let result = CostValue::from_scaled_units(-1);

        assert_eq!(result, Err(CostError::NegativeCost));
    }

    #[test]
    fn vector_distinguishes_unknown_from_zero() {
        let vector = CostVector::new();

        assert!(!vector.contains(CostDimension::Time));
        assert_eq!(
            vector.get(CostDimension::Time),
            None
        );

        let mut explicit = CostVector::new();

        explicit.set(
            CostDimension::Time,
            CostValue::zero(),
        );

        assert!(explicit.contains(CostDimension::Time));
        assert_eq!(
            explicit.get(CostDimension::Time),
            Some(CostValue::zero())
        );
    }

    #[test]
    fn vector_addition_preserves_unknown_dimensions() {
        let mut left = CostVector::new();

        left.set(
            CostDimension::Time,
            CostValue::from_integer(2).expect("valid cost"),
        );

        let mut right = CostVector::new();

        right.set(
            CostDimension::Time,
            CostValue::from_integer(3).expect("valid cost"),
        );

        right.set(
            CostDimension::Shots,
            CostValue::from_integer(10).expect("valid cost"),
        );

        let result = left
            .checked_add(&right)
            .expect("addition must succeed");

        assert_eq!(
            result.get(CostDimension::Time)
                .expect("time must be known")
                .value()
                .scaled_units(),
            5 * DECIMAL_FACTOR
        );

        assert_eq!(
            result.get(CostDimension::Shots),
            None
        );
    }

    #[test]
    fn weighted_score_is_deterministic() {
        let mut vector = CostVector::new();

        vector.set(
            CostDimension::Time,
            CostValue::from_integer(10).expect("valid cost"),
        );

        let mut objective = CostObjective::new();

        objective.set_weight(
            CostDimension::Time,
            CostWeight::one(),
        );

        let score = objective
            .score(&vector)
            .expect("score must succeed");

        assert_eq!(
            score.scaled_units(),
            10 * DECIMAL_FACTOR
        );
    }

    #[test]
    fn unknown_weighted_dimension_is_rejected_by_default() {
        let vector = CostVector::new();

        let mut objective = CostObjective::new();

        objective.set_weight(
            CostDimension::Time,
            CostWeight::one(),
        );

        assert_eq!(
            objective.score(&vector),
            Err(CostError::UnknownDimension {
                dimension: CostDimension::Time,
            })
        );
    }

    #[test]
    fn unknown_weighted_dimension_can_be_ignored_explicitly() {
        let vector = CostVector::new();

        let mut objective = CostObjective::new();

        objective.set_weight(
            CostDimension::Time,
            CostWeight::one(),
        );

        objective.set_reject_unknown(false);

        assert_eq!(
            objective
                .score(&vector)
                .expect("score must succeed"),
            CostValue::zero()
        );
    }

    #[test]
    fn pareto_equal_is_detected() {
        let mut left = CostVector::new();
        let mut right = CostVector::new();

        let value =
            CostValue::from_integer(5).expect("valid cost");

        left.set(CostDimension::Time, value);
        right.set(CostDimension::Time, value);

        assert_eq!(
            pareto_compare(&left, &right),
            CostRelation::Equal
        );
    }

    #[test]
    fn pareto_dominance_is_detected() {
        let mut left = CostVector::new();
        let mut right = CostVector::new();

        left.set(
            CostDimension::Time,
            CostValue::from_integer(2).expect("valid cost"),
        );

        right.set(
            CostDimension::Time,
            CostValue::from_integer(5).expect("valid cost"),
        );

        assert_eq!(
            pareto_compare(&left, &right),
            CostRelation::Dominates
        );
    }

    #[test]
    fn pareto_tradeoff_is_detected() {
        let mut left = CostVector::new();
        let mut right = CostVector::new();

        left.set(
            CostDimension::Time,
            CostValue::from_integer(2).expect("valid cost"),
        );

        right.set(
            CostDimension::Time,
            CostValue::from_integer(5).expect("valid cost"),
        );

        left.set(
            CostDimension::Qubits,
            CostValue::from_integer(10).expect("valid cost"),
        );

        right.set(
            CostDimension::Qubits,
            CostValue::from_integer(2).expect("valid cost"),
        );

        assert_eq!(
            pareto_compare(&left, &right),
            CostRelation::Tradeoff
        );
    }

    #[test]
    fn range_rejects_reversed_bounds() {
        let minimum =
            CostValue::from_integer(10).expect("valid cost");

        let maximum =
            CostValue::from_integer(2).expect("valid cost");

        assert_eq!(
            CostRange::new(minimum, maximum),
            Err(CostError::InvalidRange)
        );
    }

    #[test]
    fn budget_checks_known_value() {
        let maximum =
            CostValue::from_integer(10).expect("valid cost");

        let value =
            CostValue::from_integer(5).expect("valid cost");

        let mut vector = CostVector::new();

        vector.set(CostDimension::Time, value);

        let estimate = CostEstimate::new(vector);

        let budget =
            CostBudget::new(CostDimension::Time, maximum);

        assert_eq!(
            budget.check(&estimate),
            BudgetResult::Within
        );
    }

    #[test]
    fn budget_reports_unknown_dimension() {
        let maximum =
            CostValue::from_integer(10).expect("valid cost");

        let budget =
            CostBudget::new(CostDimension::Time, maximum);

        let estimate = CostEstimate::unknown();

        assert_eq!(
            budget.check(&estimate),
            BudgetResult::Unknown
        );
    }

    #[test]
    fn context_estimator_never_invents_missing_dimensions() {
        let context = CostContext::new()
            .with_additional_qubits(4)
            .with_additional_shots(100);

        let estimator = ContextCostEstimator::new();

        let estimate = estimator
            .estimate("retry", &context)
            .expect("estimate must succeed");

        assert!(
            estimate
                .vector()
                .contains(CostDimension::Qubits)
        );

        assert!(
            estimate
                .vector()
                .contains(CostDimension::Shots)
        );

        assert!(
            !estimate
                .vector()
                .contains(CostDimension::Time)
        );
    }

    #[test]
    fn action_cost_rejects_empty_identifier() {
        let result = ActionCost::new(
            "",
            CostEstimate::unknown(),
        );

        assert_eq!(
            result,
            Err(CostError::InvalidActionIdentifier)
        );
    }

    #[test]
    fn plan_cost_accumulates_action_costs() {
        let first_vector = {
            let mut vector = CostVector::new();

            vector.set(
                CostDimension::Time,
                CostValue::from_integer(2)
                    .expect("valid cost"),
            );

            vector
        };

        let second_vector = {
            let mut vector = CostVector::new();

            vector.set(
                CostDimension::Time,
                CostValue::from_integer(3)
                    .expect("valid cost"),
            );

            vector
        };

        let first = ActionCost::new(
            "retry",
            CostEstimate::new(first_vector),
        )
        .expect("valid action cost");

        let second = ActionCost::new(
            "verify",
            CostEstimate::new(second_vector),
        )
        .expect("valid action cost");

        let mut plan_cost = PlanCost::new();

        plan_cost
            .push(first)
            .expect("first action cost");

        plan_cost
            .push(second)
            .expect("second action cost");

        let total = plan_cost
            .total()
            .vector()
            .get(CostDimension::Time)
            .expect("time must be present");

        assert_eq!(
            total.scaled_units(),
            5 * DECIMAL_FACTOR
        );
    }
}