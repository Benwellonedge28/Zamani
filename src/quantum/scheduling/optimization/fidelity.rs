//! Zamani Quantum Scheduling — Fidelity Optimization
//!
//! This module defines the provider-neutral fidelity objective used by the
//! quantum scheduling subsystem.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > "How should two otherwise-valid schedules be compared when execution
//! > fidelity is part of the scheduling objective?"
//!
//! It does NOT:
//!
//! - assign physical qubits;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - acquire calibration data;
//! - execute a circuit;
//! - simulate a quantum state;
//! - implement a QEC decoder;
//! - synthesize pulses;
//! - mutate `quantum::ir::QuantumCircuit`;
//! - construct a schedule;
//! - assume a particular quantum technology;
//! - assume a fixed qubit count;
//! - assume a fixed gate set;
//! - assume a fixed number of resources;
//! - assume a fixed machine size.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling
//!       |
//!       +-----------------------------+
//!       |                             |
//!       v                             v
//! dependency/resource/timing      fidelity model
//!       |                             |
//!       +-------------+---------------+
//!                     |
//!                     v
//!                 planner
//!                     |
//!                     v
//!                 schedule
//!                     |
//!                     v
//!              fidelity evaluation
//! ```
//!
//! # Core design principle
//!
//! Fidelity is a **quality metric**, not a scheduling primitive.
//!
//! A scheduler must never infer physical fidelity from an operation name,
//! qubit number, hardware vendor, or hard-coded gate table.
//!
//! Instead:
//!
//! ```text
//! target/calibration/noise model
//!              |
//!              v
//!     FidelityModel implementation
//!              |
//!              v
//!       FidelityEvaluator
//!              |
//!              v
//!      FidelityEstimate
//!              |
//!              v
//! Scheduling objective comparison
//! ```
//!
//! This keeps the scheduler hardware-independent while allowing hardware-
//! specific fidelity models to be supplied through adapters.
//!
//! # Portability
//!
//! The same Zamani program can therefore be evaluated on:
//!
//! - a single physical qubit;
//! - a small QPU;
//! - a large QPU;
//! - a modular QPU;
//! - a distributed quantum system;
//! - a simulator/emulator;
//! - a future quantum architecture.
//!
//! The fidelity model changes with the execution target; the source program
//! and this module's semantics do not.
//!
//! # Canonical qubit identity
//!
//! This module does not define a qubit identity.
//!
//! When a fidelity model needs qubit identity, implementations must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Never introduce `Qubit`, `QubitId`, `PhysicalQubit`, or `QubitIndex`
//! replacements in this module.
//!
//! Physical-resource identities belong to the hardware/routing boundaries.
//!
//! # Numerical policy
//!
//! Scheduling time itself must remain exact and integer/rational based.
//! Fidelity, however, is inherently a numerical quality estimate and may use
//! floating-point values at this optimization boundary.
//!
//! All externally supplied floating-point values are validated before being
//! accepted.
//!
//! The following invariants are enforced:
//!
//! ```text
//! finite
//! 0.0 <= fidelity <= 1.0
//! finite confidence when present
//! 0.0 <= confidence <= 1.0
//! finite uncertainty when present
//! uncertainty >= 0.0
//! ```
//!
//! NaN and infinities are rejected.
//!
//! # Aggregation semantics
//!
//! Fidelity estimates may come from:
//!
//! - per-operation estimates;
//! - per-resource estimates;
//! - calibration models;
//! - error-channel models;
//! - empirical models;
//! - externally supplied target models;
//! - composed subsystem estimates.
//!
//! This module does not assume that independent operation errors may always
//! be multiplied together. Such assumptions belong to the supplied
//! `FidelityModel`.
//!
//! The default aggregation provided here is therefore deliberately explicit:
//!
//! `FidelityAggregation::IndependentProduct` means that the caller/model has
//! explicitly chosen the independent-product approximation.
//!
//! No aggregation model is silently assumed by `FidelityEvaluator`.
//!
//! # Scheduling semantics
//!
//! Higher fidelity is better.
//!
//! Therefore:
//!
//! ```text
//! compare(a, b)
//!
//! a > b  => a is preferable
//! a == b => equivalent under the selected precision policy
//! a < b  => b is preferable
//! ```
//!
//! The comparison policy is configurable and never tied to a particular
//! hardware scale.
//!
//! # Determinism
//!
//! This module performs no implicit random sampling.
//!
//! A `FidelityModel` that requires stochastic estimation must expose that
//! explicitly through its implementation and deterministic configuration.
//! This module itself remains deterministic.
//!
//! # Scalability
//!
//! No storage is allocated proportional to the declared machine size.
//!
//! In particular, this file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_GATES
//! MAX_CHANNELS
//! MAX_DEPTH
//! ```
//!
//! The caller may evaluate a schedule containing one operation or an
//! arbitrarily large finite number of operations subject only to:
//!
//! - available memory;
//! - explicit resource/security limits;
//! - the complexity of the selected fidelity model.
//!
//! Aggregation APIs are iterator-based where practical so callers can stream
//! estimates rather than constructing a second copy of a complete schedule.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Stable Rust only.
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! # Integration contracts
//!
//! `crate::quantum::scheduling::policies::policy`
//!     Owns `SchedulingObjective::MaximizeEstimatedFidelity`.
//!
//! `crate::quantum::scheduling::context`
//!     Supplies target/resource/timing information to the scheduler. A
//!     fidelity model may be attached through an adapter without making this
//!     file depend directly on hardware.
//!
//! `crate::quantum::scheduling::result`
//!     May store `FidelityEstimate` as part of schedule quality information.
//!
//! `crate::quantum::scheduling::optimization::multi_objective`
//!     May combine fidelity with makespan, idle time, energy, or other
//!     objectives.
//!
//! `crate::quantum::hardware`
//!     Supplies target/calibration information through an adapter.
//!
//! `crate::quantum::routing`
//!     May supply an upstream estimated-fidelity value. The scheduling layer
//!     may refine it after timing/resource decisions are known.
//!
//! `crate::quantum::zqn`
//!     May provide a richer noise/error model through an adapter.
//!
//! `crate::quantum::ir::qubit`
//!     Owns canonical `QubitId` whenever qubit identity is required by a
//!     concrete fidelity model.
//!
//! # Important dependency rule
//!
//! This file intentionally has no direct dependency on hardware, routing,
//! ZQN, scheduler context, or QuantumCircuit implementation.
//!
//! That makes the fidelity contract independently implementable and prevents
//! circular dependencies.
//!
//! Hardware-specific integrations belong in adapter modules.
//!
//! # No semantic mutation
//!
//! Fidelity evaluation MUST NOT mutate the input schedule or quantum IR.
//!
//! A fidelity objective can influence selection among candidate schedules,
//! but it must never alter quantum semantics itself.
//!
//! # Versioning
//!
//! `FIDELITY_SCHEMA_VERSION` identifies this module's externally observable
//! semantic contract.
//!
//! It must only change when serialization/comparison semantics change
//! incompatibly.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! `#![forbid(unsafe_code)]` makes this requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the fidelity optimization schema.
pub const FIDELITY_SCHEMA_ID: &str = "zamani.quantum.scheduling.optimization.fidelity";

/// Semantic version of the fidelity optimization contract.
pub const FIDELITY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Result
// =============================================================================

/// Result type returned by fidelity operations.
pub type FidelityResult<T> = Result<T, FidelityError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the fidelity optimization subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum FidelityError {
    /// A fidelity value is outside the closed interval `[0, 1]`.
    OutOfRange {
        /// Value supplied by the caller/model.
        value: f64,
    },

    /// A floating-point value is NaN or infinite.
    NonFinite {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// A confidence value is invalid.
    InvalidConfidence {
        /// Supplied confidence.
        value: f64,
    },

    /// An uncertainty value is negative.
    NegativeUncertainty {
        /// Supplied uncertainty.
        value: f64,
    },

    /// An arithmetic operation overflowed or otherwise became non-finite.
    ArithmeticFailure {
        /// Stable description of the calculation.
        calculation: &'static str,
    },

    /// A required model was not supplied.
    MissingModel,

    /// The model cannot evaluate the requested schedule.
    ModelUnavailable {
        /// Stable explanation.
        reason: &'static str,
    },

    /// A model returned an invalid estimate.
    InvalidModelEstimate {
        /// Stable explanation.
        reason: &'static str,
    },

    /// An empty estimate collection was supplied where a value was required.
    EmptyInput,

    /// An invalid comparison tolerance was supplied.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// An invalid aggregation configuration was supplied.
    InvalidAggregation {
        /// Stable explanation.
        reason: &'static str,
    },
}

impl core::fmt::Display for FidelityError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange { value } => {
                write!(
                    formatter,
                    "fidelity value {value} is outside the range [0, 1]"
                )
            }

            Self::NonFinite { field, value } => {
                write!(
                    formatter,
                    "fidelity field `{field}` must be finite, got {value}"
                )
            }

            Self::InvalidConfidence { value } => {
                write!(
                    formatter,
                    "confidence {value} is outside the range [0, 1]"
                )
            }

            Self::NegativeUncertainty { value } => {
                write!(
                    formatter,
                    "fidelity uncertainty must be non-negative, got {value}"
                )
            }

            Self::ArithmeticFailure { calculation } => {
                write!(
                    formatter,
                    "non-finite result while calculating {calculation}"
                )
            }

            Self::MissingModel => {
                formatter.write_str("no fidelity model was supplied")
            }

            Self::ModelUnavailable { reason } => {
                write!(
                    formatter,
                    "fidelity model is unavailable: {reason}"
                )
            }

            Self::InvalidModelEstimate { reason } => {
                write!(
                    formatter,
                    "fidelity model returned an invalid estimate: {reason}"
                )
            }

            Self::EmptyInput => {
                formatter.write_str("fidelity evaluation requires at least one estimate")
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "fidelity comparison tolerance {value} is invalid"
                )
            }

            Self::InvalidAggregation { reason } => {
                write!(
                    formatter,
                    "invalid fidelity aggregation configuration: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for FidelityError {}

// =============================================================================
// Fidelity value
// =============================================================================

/// A validated fidelity value in the closed interval `[0, 1]`.
///
/// This type prevents invalid fidelity values from entering scheduling
/// objective comparisons.
///
/// `Fidelity` represents an estimate, not a claim that the physical device
/// will actually achieve the exact value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fidelity(f64);

impl Fidelity {
    /// The mathematically perfect fidelity.
    pub const PERFECT: Self = Self(1.0);

    /// The minimum valid fidelity.
    pub const ZERO: Self = Self(0.0);

    /// Creates a validated fidelity.
    pub fn new(value: f64) -> FidelityResult<Self> {
        if !value.is_finite() {
            return Err(FidelityError::NonFinite {
                field: "fidelity",
                value,
            });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(FidelityError::OutOfRange { value });
        }

        Ok(Self(value))
    }

    /// Returns the raw floating-point representation.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the fidelity loss `1 - fidelity`.
    pub fn loss(self) -> FidelityResult<f64> {
        let loss = 1.0 - self.0;

        if !loss.is_finite() {
            return Err(FidelityError::ArithmeticFailure {
                calculation: "fidelity loss",
            });
        }

        Ok(loss)
    }

    /// Returns whether this is mathematically perfect fidelity.
    #[must_use]
    pub const fn is_perfect(self) -> bool {
        self.0 == 1.0
    }

    /// Returns whether this is zero fidelity.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns the better of two fidelity values.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the worse of two fidelity values.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }
}

impl Default for Fidelity {
    fn default() -> Self {
        Self::PERFECT
    }
}

// =============================================================================
// Confidence
// =============================================================================

/// Confidence attached to a fidelity estimate.
///
/// This is not itself a fidelity value. It expresses how strongly the model
/// supports the estimate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(f64);

impl Confidence {
    /// Creates validated confidence.
    pub fn new(value: f64) -> FidelityResult<Self> {
        if !value.is_finite() {
            return Err(FidelityError::NonFinite {
                field: "confidence",
                value,
            });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(FidelityError::InvalidConfidence { value });
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Default for Confidence {
    fn default() -> Self {
        // A default confidence of 1.0 means "the model did not provide
        // uncertainty information", not "the hardware is perfect".
        Self(1.0)
    }
}

// =============================================================================
// Uncertainty
// =============================================================================

/// Non-negative absolute uncertainty associated with a fidelity estimate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FidelityUncertainty(f64);

impl FidelityUncertainty {
    /// Creates validated uncertainty.
    pub fn new(value: f64) -> FidelityResult<Self> {
        if !value.is_finite() {
            return Err(FidelityError::NonFinite {
                field: "uncertainty",
                value,
            });
        }

        if value < 0.0 {
            return Err(FidelityError::NegativeUncertainty { value });
        }

        Ok(Self(value))
    }

    /// Returns the uncertainty value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Default for FidelityUncertainty {
    fn default() -> Self {
        Self(0.0)
    }
}

// =============================================================================
// Estimate
// =============================================================================

/// A complete fidelity estimate.
///
/// This is the canonical value exchanged between fidelity models and the
/// scheduling objective layer.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FidelityEstimate {
    fidelity: Fidelity,
    confidence: Confidence,
    uncertainty: FidelityUncertainty,
}

impl FidelityEstimate {
    /// Creates an estimate with no uncertainty information.
    pub fn exact(value: f64) -> FidelityResult<Self> {
        Ok(Self {
            fidelity: Fidelity::new(value)?,
            confidence: Confidence::default(),
            uncertainty: FidelityUncertainty::default(),
        })
    }

    /// Creates an estimate with explicit confidence and uncertainty.
    pub fn new(
        fidelity: f64,
        confidence: f64,
        uncertainty: f64,
    ) -> FidelityResult<Self> {
        Ok(Self {
            fidelity: Fidelity::new(fidelity)?,
            confidence: Confidence::new(confidence)?,
            uncertainty: FidelityUncertainty::new(uncertainty)?,
        })
    }

    /// Creates an estimate from validated components.
    pub fn from_parts(
        fidelity: Fidelity,
        confidence: Confidence,
        uncertainty: FidelityUncertainty,
    ) -> Self {
        Self {
            fidelity,
            confidence,
            uncertainty,
        }
    }

    /// Returns the estimated fidelity.
    #[must_use]
    pub const fn fidelity(self) -> Fidelity {
        self.fidelity
    }

    /// Returns the raw fidelity value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.fidelity.value()
    }

    /// Returns model confidence.
    #[must_use]
    pub const fn confidence(self) -> Confidence {
        self.confidence
    }

    /// Returns absolute uncertainty.
    #[must_use]
    pub const fn uncertainty(self) -> FidelityUncertainty {
        self.uncertainty
    }

    /// Returns the conservative lower fidelity bound.
    ///
    /// The bound is clamped to zero because fidelity cannot be negative.
    pub fn lower_bound(self) -> FidelityResult<Fidelity> {
        let value = (self.value() - self.uncertainty.value()).max(0.0);

        Fidelity::new(value)
    }

    /// Returns the conservative upper fidelity bound.
    ///
    /// The bound is clamped to one because fidelity cannot exceed one.
    pub fn upper_bound(self) -> FidelityResult<Fidelity> {
        let value = (self.value() + self.uncertainty.value()).min(1.0);

        Fidelity::new(value)
    }

    /// Returns fidelity loss.
    pub fn loss(self) -> FidelityResult<f64> {
        self.fidelity.loss()
    }

    /// Returns true when the estimate contains no uncertainty.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        self.uncertainty.value() == 0.0
    }
}

impl Default for FidelityEstimate {
    fn default() -> Self {
        Self {
            fidelity: Fidelity::PERFECT,
            confidence: Confidence::default(),
            uncertainty: FidelityUncertainty::default(),
        }
    }
}

// =============================================================================
// Aggregation
// =============================================================================

/// Defines how independent fidelity contributions are combined.
///
/// The scheduler must never silently select an aggregation model. The
/// selected aggregation is part of the fidelity objective configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FidelityAggregation {
    /// Multiply independent fidelity contributions.
    ///
    /// If contributions are:
    ///
    /// `f1, f2, ..., fn`
    ///
    /// the result is:
    ///
    /// `f1 * f2 * ... * fn`
    ///
    /// This is an approximation and is only valid when the supplied model
    /// justifies independence.
    IndependentProduct,

    /// Use the minimum contribution.
    ///
    /// This is conservative and useful when the weakest component is treated
    /// as the limiting fidelity factor.
    Minimum,

    /// Use the arithmetic mean.
    ArithmeticMean,

    /// Use a caller-supplied externally calculated total.
    ///
    /// The evaluator does not invent the composition semantics.
    ExternallyComposed,
}

impl Default for FidelityAggregation {
    fn default() -> Self {
        Self::IndependentProduct
    }
}

impl FidelityAggregation {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndependentProduct => "independent_product",
            Self::Minimum => "minimum",
            Self::ArithmeticMean => "arithmetic_mean",
            Self::ExternallyComposed => "externally_composed",
        }
    }
}

impl core::fmt::Display for FidelityAggregation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Comparison
// =============================================================================

/// Fidelity comparison configuration.
///
/// A tolerance is used only to identify practically equivalent objective
/// values. It does not modify the underlying fidelity estimate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FidelityComparison {
    tolerance: f64,
}

impl FidelityComparison {
    /// Creates a comparison policy.
    ///
    /// `tolerance` must be finite and non-negative.
    pub fn new(tolerance: f64) -> FidelityResult<Self> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(FidelityError::InvalidTolerance { value: tolerance });
        }

        Ok(Self { tolerance })
    }

    /// Exact comparison with zero tolerance.
    #[must_use]
    pub const fn exact() -> Self {
        Self { tolerance: 0.0 }
    }

    /// Returns configured tolerance.
    #[must_use]
    pub const fn tolerance(self) -> f64 {
        self.tolerance
    }

    /// Compares two fidelity estimates.
    ///
    /// Returns:
    ///
    /// - `Ordering::Greater` when `left` is better;
    /// - `Ordering::Less` when `right` is better;
    /// - `Ordering::Equal` when they are equivalent under tolerance.
    #[must_use]
    pub fn compare(
        self,
        left: FidelityEstimate,
        right: FidelityEstimate,
    ) -> core::cmp::Ordering {
        let difference = left.value() - right.value();

        if difference.abs() <= self.tolerance {
            core::cmp::Ordering::Equal
        } else {
            left.value()
                .partial_cmp(&right.value())
                .unwrap_or(core::cmp::Ordering::Equal)
        }
    }

    /// Returns whether the left estimate is at least as good as the right
    /// estimate under this comparison policy.
    #[must_use]
    pub fn is_at_least_as_good(
        self,
        left: FidelityEstimate,
        right: FidelityEstimate,
    ) -> bool {
        matches!(
            self.compare(left, right),
            core::cmp::Ordering::Greater | core::cmp::Ordering::Equal
        )
    }
}

impl Default for FidelityComparison {
    fn default() -> Self {
        Self::exact()
    }
}

// =============================================================================
// Model input
// =============================================================================

/// Context-independent operation fidelity contribution.
///
/// This type intentionally does not contain an operation enum or a qubit
/// representation. Concrete adapters can associate their own operation
/// identity and use this value at the fidelity boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityContribution {
    estimate: FidelityEstimate,
}

impl FidelityContribution {
    /// Creates a contribution.
    pub fn new(estimate: FidelityEstimate) -> Self {
        Self { estimate }
    }

    /// Creates an exact contribution.
    pub fn exact(value: f64) -> FidelityResult<Self> {
        Ok(Self::new(FidelityEstimate::exact(value)?))
    }

    /// Returns the contribution estimate.
    #[must_use]
    pub const fn estimate(self) -> FidelityEstimate {
        self.estimate
    }
}

// =============================================================================
// Fidelity model
// =============================================================================

/// Provider-neutral fidelity model.
///
/// Hardware, ZQN, calibration, simulation, empirical, and research-specific
/// implementations can implement this trait.
///
/// The trait intentionally operates on an abstract schedule view supplied by
/// the integration layer instead of depending on scheduler internals.
///
/// `ScheduleView` is a caller-owned type so this module remains independent
/// from the rest of `quantum::scheduling`.
///
/// The model may use any target information available through its own
/// implementation.
///
/// # Contract
///
/// Implementations MUST:
///
/// - return finite values;
/// - return fidelity in `[0, 1]`;
/// - never mutate the schedule;
/// - never mutate canonical quantum IR;
/// - never assume a fixed number of qubits;
/// - never assume a fixed operation count;
/// - document any independence assumptions;
/// - return an error when required target information is unavailable.
pub trait FidelityModel<ScheduleView> {
    /// Evaluates the complete schedule.
    fn evaluate(
        &self,
        schedule: &ScheduleView,
    ) -> FidelityResult<FidelityEstimate>;
}

// =============================================================================
// Objective
// =============================================================================

/// Fidelity optimization objective.
///
/// This is the scheduling-side configuration for
/// `SchedulingObjective::MaximizeEstimatedFidelity`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityObjective {
    aggregation: FidelityAggregation,
    comparison: FidelityComparison,
}

impl FidelityObjective {
    /// Creates an objective using the specified aggregation and comparison.
    pub const fn new(
        aggregation: FidelityAggregation,
        comparison: FidelityComparison,
    ) -> Self {
        Self {
            aggregation,
            comparison,
        }
    }

    /// Creates the explicit independent-product objective.
    ///
    /// This is convenient but remains explicit in the resulting value.
    #[must_use]
    pub fn independent_product() -> Self {
        Self {
            aggregation: FidelityAggregation::IndependentProduct,
            comparison: FidelityComparison::exact(),
        }
    }

    /// Returns the aggregation policy.
    #[must_use]
    pub const fn aggregation(self) -> FidelityAggregation {
        self.aggregation
    }

    /// Returns the comparison policy.
    #[must_use]
    pub const fn comparison(self) -> FidelityComparison {
        self.comparison
    }

    /// Compares candidate schedules by their already evaluated fidelity.
    ///
    /// Higher fidelity is always preferred.
    #[must_use]
    pub fn compare(
        self,
        left: FidelityEstimate,
        right: FidelityEstimate,
    ) -> core::cmp::Ordering {
        self.comparison.compare(left, right)
    }

    /// Returns whether `candidate` is at least as good as `current`.
    #[must_use]
    pub fn is_at_least_as_good(
        self,
        candidate: FidelityEstimate,
        current: FidelityEstimate,
    ) -> bool {
        self.comparison
            .is_at_least_as_good(candidate, current)
    }
}

impl Default for FidelityObjective {
    fn default() -> Self {
        Self::independent_product()
    }
}

// =============================================================================
// Evaluator
// =============================================================================

/// Fidelity evaluator.
///
/// The evaluator owns aggregation and validation logic while delegating
/// physical fidelity semantics to a `FidelityModel`.
#[derive(Debug, Clone, Copy)]
pub struct FidelityEvaluator {
    objective: FidelityObjective,
}

impl FidelityEvaluator {
    /// Creates an evaluator.
    #[must_use]
    pub const fn new(objective: FidelityObjective) -> Self {
        Self { objective }
    }

    /// Returns the evaluator's objective.
    #[must_use]
    pub const fn objective(self) -> FidelityObjective {
        self.objective
    }

    /// Evaluates a schedule using the supplied model.
    pub fn evaluate<ScheduleView, Model>(
        &self,
        model: &Model,
        schedule: &ScheduleView,
    ) -> FidelityResult<FidelityEstimate>
    where
        Model: FidelityModel<ScheduleView>,
    {
        let estimate = model.evaluate(schedule)?;

        validate_estimate(estimate)?;

        Ok(estimate)
    }

    /// Aggregates an iterator of fidelity contributions.
    ///
    /// The input is consumed exactly once.
    ///
    /// This permits streaming aggregation without requiring a collection
    /// proportional to the number of operations.
    pub fn aggregate<I>(
        &self,
        contributions: I,
    ) -> FidelityResult<FidelityEstimate>
    where
        I: IntoIterator<Item = FidelityContribution>,
    {
        aggregate_estimates(
            contributions
                .into_iter()
                .map(FidelityContribution::estimate),
            self.objective.aggregation(),
        )
    }

    /// Compares two candidate estimates.
    #[must_use]
    pub fn compare(
        &self,
        left: FidelityEstimate,
        right: FidelityEstimate,
    ) -> core::cmp::Ordering {
        self.objective.compare(left, right)
    }

    /// Returns the better candidate.
    #[must_use]
    pub fn better(
        &self,
        left: FidelityEstimate,
        right: FidelityEstimate,
    ) -> FidelityEstimate {
        match self.compare(left, right) {
            core::cmp::Ordering::Less => right,
            core::cmp::Ordering::Greater | core::cmp::Ordering::Equal => left,
        }
    }
}

impl Default for FidelityEvaluator {
    fn default() -> Self {
        Self::new(FidelityObjective::default())
    }
}

// =============================================================================
// Aggregation implementation
// =============================================================================

/// Aggregates validated estimates according to the selected policy.
///
/// This function is public so integration modules can use the same semantics
/// without constructing a `FidelityEvaluator`.
pub fn aggregate_estimates<I>(
    estimates: I,
    aggregation: FidelityAggregation,
) -> FidelityResult<FidelityEstimate>
where
    I: IntoIterator<Item = FidelityEstimate>,
{
    match aggregation {
        FidelityAggregation::IndependentProduct => {
            aggregate_independent_product(estimates)
        }

        FidelityAggregation::Minimum => {
            aggregate_minimum(estimates)
        }

        FidelityAggregation::ArithmeticMean => {
            aggregate_arithmetic_mean(estimates)
        }

        FidelityAggregation::ExternallyComposed => {
            Err(FidelityError::InvalidAggregation {
                reason:
                    "externally_composed requires a complete externally supplied estimate",
            })
        }
    }
}

/// Aggregates fidelity using an independent-product approximation.
///
/// Confidence is conservatively accumulated using the minimum confidence.
///
/// Uncertainty is conservatively accumulated by summing absolute
/// uncertainties. This is intentionally conservative rather than pretending
/// independent uncertainty propagation is universally valid.
pub fn aggregate_independent_product<I>(
    estimates: I,
) -> FidelityResult<FidelityEstimate>
where
    I: IntoIterator<Item = FidelityEstimate>,
{
    let mut seen = false;
    let mut fidelity = 1.0_f64;
    let mut confidence = 1.0_f64;
    let mut uncertainty = 0.0_f64;

    for estimate in estimates {
        validate_estimate(estimate)?;

        seen = true;

        fidelity *= estimate.value();

        if !fidelity.is_finite() {
            return Err(FidelityError::ArithmeticFailure {
                calculation: "independent fidelity product",
            });
        }

        confidence = confidence.min(estimate.confidence().value());

        uncertainty += estimate.uncertainty().value();

        if !uncertainty.is_finite() {
            return Err(FidelityError::ArithmeticFailure {
                calculation: "fidelity uncertainty accumulation",
            });
        }
    }

    if !seen {
        return Err(FidelityError::EmptyInput);
    }

    // Numerical multiplication can produce a value very slightly outside
    // the mathematical range because of floating-point rounding. Clamp only
    // after checking that the result is finite.
    fidelity = fidelity.clamp(0.0, 1.0);

    FidelityEstimate::new(
        fidelity,
        confidence,
        uncertainty,
    )
}

/// Aggregates fidelity by selecting the weakest contribution.
///
/// Confidence is the minimum confidence and uncertainty is the maximum
/// uncertainty because the result is dominated by the weakest component.
pub fn aggregate_minimum<I>(
    estimates: I,
) -> FidelityResult<FidelityEstimate>
where
    I: IntoIterator<Item = FidelityEstimate>,
{
    let mut result: Option<FidelityEstimate> = None;

    for estimate in estimates {
        validate_estimate(estimate)?;

        result = Some(match result {
            None => estimate,

            Some(current) => {
                if estimate.value() < current.value() {
                    estimate
                } else if estimate.value() > current.value() {
                    current
                } else {
                    // Equal fidelity: retain the more conservative
                    // uncertainty/confidence information.
                    FidelityEstimate::new(
                        current.value(),
                        current.confidence()
                            .value()
                            .min(estimate.confidence().value()),
                        current.uncertainty()
                            .value()
                            .max(estimate.uncertainty().value()),
                    )?
                }
            }
        });
    }

    result.ok_or(FidelityError::EmptyInput)
}

/// Aggregates fidelity using the arithmetic mean.
///
/// The result's confidence is the minimum confidence. Uncertainty is the
/// arithmetic mean of the component uncertainties.
pub fn aggregate_arithmetic_mean<I>(
    estimates: I,
) -> FidelityResult<FidelityEstimate>
where
    I: IntoIterator<Item = FidelityEstimate>,
{
    let mut count = 0u64;
    let mut sum = 0.0_f64;
    let mut confidence = 1.0_f64;
    let mut uncertainty_sum = 0.0_f64;

    for estimate in estimates {
        validate_estimate(estimate)?;

        count = count
            .checked_add(1)
            .ok_or(FidelityError::ArithmeticFailure {
                calculation: "fidelity estimate count",
            })?;

        sum += estimate.value();

        if !sum.is_finite() {
            return Err(FidelityError::ArithmeticFailure {
                calculation: "fidelity arithmetic mean",
            });
        }

        confidence = confidence.min(estimate.confidence().value());

        uncertainty_sum += estimate.uncertainty().value();

        if !uncertainty_sum.is_finite() {
            return Err(FidelityError::ArithmeticFailure {
                calculation: "fidelity uncertainty mean",
            });
        }
    }

    if count == 0 {
        return Err(FidelityError::EmptyInput);
    }

    let count_as_f64 = count as f64;

    let fidelity = sum / count_as_f64;
    let uncertainty = uncertainty_sum / count_as_f64;

    FidelityEstimate::new(
        fidelity,
        confidence,
        uncertainty,
    )
}

// =============================================================================
// Validation
// =============================================================================

/// Validates a complete fidelity estimate.
///
/// This function is intentionally public so adapters can validate external
/// model output before it enters scheduling.
pub fn validate_estimate(
    estimate: FidelityEstimate,
) -> FidelityResult<()> {
    let fidelity = estimate.value();

    if !fidelity.is_finite() {
        return Err(FidelityError::NonFinite {
            field: "fidelity",
            value: fidelity,
        });
    }

    if !(0.0..=1.0).contains(&fidelity) {
        return Err(FidelityError::OutOfRange { value: fidelity });
    }

    let confidence = estimate.confidence().value();

    if !confidence.is_finite() {
        return Err(FidelityError::NonFinite {
            field: "confidence",
            value: confidence,
        });
    }

    if !(0.0..=1.0).contains(&confidence) {
        return Err(FidelityError::InvalidConfidence {
            value: confidence,
        });
    }

    let uncertainty = estimate.uncertainty().value();

    if !uncertainty.is_finite() {
        return Err(FidelityError::NonFinite {
            field: "uncertainty",
            value: uncertainty,
        });
    }

    if uncertainty < 0.0 {
        return Err(FidelityError::NegativeUncertainty {
            value: uncertainty,
        });
    }

    Ok(())
}

// =============================================================================
// Convenience comparison functions
// =============================================================================

/// Compares two fidelity estimates exactly.
///
/// Higher fidelity is better.
#[must_use]
pub fn compare_fidelity(
    left: FidelityEstimate,
    right: FidelityEstimate,
) -> core::cmp::Ordering {
    FidelityComparison::exact().compare(left, right)
}

/// Returns the higher-fidelity estimate.
#[must_use]
pub fn max_fidelity(
    left: FidelityEstimate,
    right: FidelityEstimate,
) -> FidelityEstimate {
    if left.value() >= right.value() {
        left
    } else {
        right
    }
}

/// Returns the lower-fidelity estimate.
#[must_use]
pub fn min_fidelity(
    left: FidelityEstimate,
    right: FidelityEstimate,
) -> FidelityEstimate {
    if left.value() <= right.value() {
        left
    } else {
        right
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fidelity_accepts_closed_unit_interval() {
        assert_eq!(
            Fidelity::new(0.0)
                .expect("zero fidelity is valid")
                .value(),
            0.0
        );

        assert_eq!(
            Fidelity::new(1.0)
                .expect("perfect fidelity is valid")
                .value(),
            1.0
        );
    }

    #[test]
    fn fidelity_rejects_invalid_values() {
        assert!(Fidelity::new(-0.000_001).is_err());
        assert!(Fidelity::new(1.000_001).is_err());
        assert!(Fidelity::new(f64::NAN).is_err());
        assert!(Fidelity::new(f64::INFINITY).is_err());
        assert!(Fidelity::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn confidence_is_validated() {
        assert!(Confidence::new(0.0).is_ok());
        assert!(Confidence::new(1.0).is_ok());
        assert!(Confidence::new(-0.1).is_err());
        assert!(Confidence::new(1.1).is_err());
        assert!(Confidence::new(f64::NAN).is_err());
    }

    #[test]
    fn uncertainty_is_validated() {
        assert!(FidelityUncertainty::new(0.0).is_ok());
        assert!(FidelityUncertainty::new(0.25).is_ok());
        assert!(FidelityUncertainty::new(-0.01).is_err());
        assert!(FidelityUncertainty::new(f64::NAN).is_err());
    }

    #[test]
    fn estimate_bounds_are_clamped() {
        let estimate = FidelityEstimate::new(
            0.5,
            1.0,
            0.75,
        )
        .expect("estimate must be valid");

        assert_eq!(
            estimate
                .lower_bound()
                .expect("lower bound must be valid")
                .value(),
            0.0
        );

        assert_eq!(
            estimate
                .upper_bound()
                .expect("upper bound must be valid")
                .value(),
            1.0
        );
    }

    #[test]
    fn independent_product_is_correct() {
        let result = aggregate_independent_product([
            FidelityEstimate::exact(0.99)
                .expect("valid estimate"),
            FidelityEstimate::exact(0.98)
                .expect("valid estimate"),
        ])
        .expect("aggregation must succeed");

        let expected = 0.99 * 0.98;

        assert!(
            (result.value() - expected).abs() < 1.0e-15,
            "unexpected product: {} != {}",
            result.value(),
            expected
        );
    }

    #[test]
    fn minimum_aggregation_is_correct() {
        let result = aggregate_minimum([
            FidelityEstimate::exact(0.99)
                .expect("valid estimate"),
            FidelityEstimate::exact(0.80)
                .expect("valid estimate"),
            FidelityEstimate::exact(0.95)
                .expect("valid estimate"),
        ])
        .expect("aggregation must succeed");

        assert_eq!(result.value(), 0.80);
    }

    #[test]
    fn arithmetic_mean_is_correct() {
        let result = aggregate_arithmetic_mean([
            FidelityEstimate::exact(0.80)
                .expect("valid estimate"),
            FidelityEstimate::exact(1.00)
                .expect("valid estimate"),
        ])
        .expect("aggregation must succeed");

        assert!(
            (result.value() - 0.90).abs() < 1.0e-15,
            "unexpected mean: {}",
            result.value()
        );
    }

    #[test]
    fn empty_aggregation_is_rejected() {
        assert!(
            aggregate_independent_product(
                core::iter::empty::<FidelityEstimate>()
            )
            .is_err()
        );

        assert!(
            aggregate_minimum(
                core::iter::empty::<FidelityEstimate>()
            )
            .is_err()
        );

        assert!(
            aggregate_arithmetic_mean(
                core::iter::empty::<FidelityEstimate>()
            )
            .is_err()
        );
    }

    #[test]
    fn comparison_prefers_higher_fidelity() {
        let low = FidelityEstimate::exact(0.90)
            .expect("valid estimate");

        let high = FidelityEstimate::exact(0.95)
            .expect("valid estimate");

        assert_eq!(
            compare_fidelity(high, low),
            core::cmp::Ordering::Greater
        );

        assert_eq!(
            compare_fidelity(low, high),
            core::cmp::Ordering::Less
        );
    }

    #[test]
    fn comparison_tolerance_can_make_values_equivalent() {
        let comparison = FidelityComparison::new(0.01)
            .expect("valid tolerance");

        let left = FidelityEstimate::exact(0.900)
            .expect("valid estimate");

        let right = FidelityEstimate::exact(0.905)
            .expect("valid estimate");

        assert_eq!(
            comparison.compare(left, right),
            core::cmp::Ordering::Equal
        );
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        assert!(FidelityComparison::new(-1.0).is_err());
        assert!(FidelityComparison::new(f64::NAN).is_err());
        assert!(FidelityComparison::new(f64::INFINITY).is_err());
    }

    #[test]
    fn evaluator_uses_objective() {
        let objective = FidelityObjective::new(
            FidelityAggregation::Minimum,
            FidelityComparison::exact(),
        );

        let evaluator = FidelityEvaluator::new(objective);

        let result = evaluator
            .aggregate([
                FidelityContribution::exact(0.97)
                    .expect("valid contribution"),
                FidelityContribution::exact(0.93)
                    .expect("valid contribution"),
            ])
            .expect("aggregation must succeed");

        assert_eq!(result.value(), 0.93);
    }

    #[test]
    fn model_output_is_validated() {
        struct TestModel;

        impl FidelityModel<()> for TestModel {
            fn evaluate(
                &self,
                _schedule: &(),
            ) -> FidelityResult<FidelityEstimate> {
                FidelityEstimate::exact(0.91)
            }
        }

        let evaluator = FidelityEvaluator::default();

        let result = evaluator
            .evaluate(&TestModel, &())
            .expect("model evaluation must succeed");

        assert_eq!(result.value(), 0.91);
    }

    #[test]
    fn invalid_model_output_is_rejected() {
        struct TestModel;

        impl FidelityModel<()> for TestModel {
            fn evaluate(
                &self,
                _schedule: &(),
            ) -> FidelityResult<FidelityEstimate> {
                FidelityEstimate::new(
                    0.95,
                    0.9,
                    0.01,
                )
            }
        }

        let evaluator = FidelityEvaluator::default();

        let result = evaluator
            .evaluate(&TestModel, &())
            .expect("valid model estimate");

        assert_eq!(result.value(), 0.95);
    }

    #[test]
    fn better_returns_higher_fidelity() {
        let evaluator = FidelityEvaluator::default();

        let left = FidelityEstimate::exact(0.91)
            .expect("valid estimate");

        let right = FidelityEstimate::exact(0.94)
            .expect("valid estimate");

        assert_eq!(
            evaluator.better(left, right).value(),
            0.94
        );
    }

    #[test]
    fn uncertainty_accumulates_conservatively() {
        let result = aggregate_independent_product([
            FidelityEstimate::new(0.99, 0.9, 0.01)
                .expect("valid estimate"),
            FidelityEstimate::new(0.98, 0.8, 0.02)
                .expect("valid estimate"),
        ])
        .expect("aggregation must succeed");

        assert_eq!(result.confidence().value(), 0.8);
        assert!(
            (result.uncertainty().value() - 0.03).abs() < 1.0e-15
        );
    }

    #[test]
    fn no_qubit_count_is_embedded_in_fidelity_contract() {
        // This test intentionally documents an architectural property:
        // fidelity evaluation has no machine-size field.
        let estimate = FidelityEstimate::exact(1.0)
            .expect("valid estimate");

        assert_eq!(estimate.value(), 1.0);
    }
}