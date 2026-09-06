//! Zamani Quantum Resilience — Confidence Model
//!
//! This module defines the foundational confidence value used by the quantum
//! resilience subsystem.
//!
//! # Architectural role
//!
//! Confidence answers:
//!
//! > "How strongly does the available evidence support this observation,
//! > diagnosis, prediction, or verification claim?"
//!
//! Confidence is epistemic evidence strength. It is NOT:
//!
//! - severity;
//! - probability of a physical fault;
//! - logical error rate;
//! - hardware fidelity;
//! - execution priority;
//! - recovery authorization;
//! - recovery cost;
//! - resource availability;
//! - resource quantity;
//! - a retry count;
//! - a policy threshold.
//!
//! Those concepts belong to other resilience and quantum subsystems.
//!
//! # Core semantic contract
//!
//! A valid [`Confidence`] is a finite normalized scalar in the closed interval:
//!
//! ```text
//! 0.0 <= confidence <= 1.0
//! ```
//!
//! where:
//!
//! ```text
//! 0.0 = no evidentiary support
//! 1.0 = maximal evidentiary support represented by this model
//! ```
//!
//! The numeric value is deliberately normalized so that the same confidence
//! representation works for:
//!
//! - one qubit;
//! - a complete QPU;
//! - a logical quantum computer;
//! - a simulator;
//! - a heterogeneous quantum system;
//! - a distributed quantum execution fabric;
//! - arbitrarily large workloads subject only to available resources.
//!
//! No machine size is encoded in this type.
//!
//! # Important distinction: confidence is not probability
//!
//! A confidence score may be derived from statistical evidence, but this type
//! does not claim that the value is a Bayesian posterior, frequentist
//! probability, likelihood, p-value, or physical fault probability.
//!
//! Those interpretations require additional mathematical contracts owned by
//! their respective subsystems.
//!
//! Therefore this module intentionally exposes a normalized confidence value,
//! not a generic `Probability` type.
//!
//! # Unknown confidence
//!
//! Unknown confidence is represented by:
//!
//! ```text
//! Option<Confidence>
//! ```
//!
//! rather than:
//!
//! ```text
//! Confidence::Unknown
//! ```
//!
//! This is intentional.
//!
//! `Confidence` itself always represents a valid confidence value.
//! Absence of confidence is represented explicitly by `None`.
//!
//! This prevents an unknown observation from accidentally being interpreted
//! as zero confidence.
//!
//! The distinction is critical:
//!
//! ```text
//! None        = confidence was not established
//! Some(0.0)   = confidence was established and is zero
//! Some(1.0)   = confidence was established at the maximum
//! ```
//!
//! # NaN and infinity
//!
//! NaN and infinite floating-point values are rejected.
//!
//! They cannot represent meaningful normalized confidence and would make
//! ordering, equality, serialization, deterministic replay, and policy
//! evaluation unsafe or ambiguous.
//!
//! # Thresholds
//!
//! This module does NOT define operational thresholds such as:
//!
//! ```text
//! 0.80
//! 0.90
//! 0.95
//! 0.99
//! ```
//!
//! Such values are policy decisions and must be supplied by:
//!
//! ```text
//! quantum::resilience::policy
//! ```
//!
//! Consumers should compare confidence against an explicitly supplied
//! `Confidence` value:
//!
//! ```text
//! confidence.meets(required_confidence)
//! ```
//!
//! This prevents machine- or application-specific thresholds from becoming
//! hidden constants in the foundational model.
//!
//! # Aggregation
//!
//! Confidence aggregation is deliberately conservative.
//!
//! This module provides `minimum`/`maximum` operations because they have
//! unambiguous ordering semantics.
//!
//! It does NOT provide a universal averaging formula.
//!
//! In particular, the following are intentionally NOT assumed:
//!
//! ```text
//! arithmetic mean
//! geometric mean
//! Bayesian multiplication
//! weighted mean
//! minimum
//! maximum
//! ```
//!
//! The mathematically correct combination rule depends on the meaning,
//! dependence structure, calibration, and provenance of the evidence.
//!
//! Diagnosis, statistical analysis, learning, or verification layers may
//! define domain-specific aggregation rules without changing this type.
//!
//! # Determinism
//!
//! The type:
//!
//! - accesses no global state;
//! - accesses no clock;
//! - accesses no hardware;
//! - accesses no filesystem;
//! - accesses no network;
//! - generates no randomness;
//! - performs no I/O.
//!
//! Given the same valid input, it produces the same result.
//!
//! No collection ordering is involved.
//!
//! # Precision
//!
//! The internal representation uses `f64` because the existing Zamani quantum
//! stack already uses floating-point confidence values in statistical and
//! quantum-noise contexts. The value is validated before construction so
//! non-finite floating-point states cannot enter the resilience model.
//!
//! This module does not pretend that `f64` is arbitrary precision.
//!
//! It is a representation of normalized confidence, not a statement that
//! confidence has infinite numerical precision.
//!
//! # No hard-coded scalability limits
//!
//! This module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_BACKENDS
//! MAX_INCIDENTS
//! MAX_FAULTS
//! MAX_OPERATIONS
//! MAX_CONFIDENCE_RECORDS
//! ```
//!
//! Confidence is O(1) state per confidence value.
//!
//! Large-scale collections, streaming observations, retention policies, and
//! memory limits belong to telemetry, history, state, or policy layers.
//!
//! # Canonical identity separation
//!
//! This module intentionally does not depend on:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Confidence is not a resource identity.
//!
//! A confidence value can later be attached to a fault, incident, diagnosis,
//! resource, QEC signal, execution result, or verification result by those
//! higher-level models.
//!
//! This keeps the dependency graph acyclic and prevents confidence from
//! becoming coupled to physical quantum topology.
//!
//! # Serialization boundary
//!
//! This file does not define a wire format and intentionally does not derive
//! `Serialize` or `Deserialize`.
//!
//! Serialization belongs to:
//!
//! ```text
//! quantum::resilience::serialization
//! ```
//!
//! The serialization layer must preserve the normalized numeric value without
//! converting invalid values into valid ones.
//!
//! # Error boundary
//!
//! Construction uses `Option` rather than introducing another resilience error
//! hierarchy:
//!
//! ```text
//! Confidence::new(value) -> Option<Confidence>
//! ```
//!
//! This keeps this foundational model independent of the higher-level
//! `ResilienceError` implementation.
//!
//! Higher-level constructors that require detailed diagnostics can translate
//! a failed confidence construction into their own existing
//! `ResilienceError` classification without changing this file.
//!
//! # Integration contract
//!
//! This file is intentionally a foundational leaf in the resilience model.
//!
//! ```text
//!                         confidence.rs
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!        model/fault.rs  model/incident.rs  diagnosis/
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                           policy/
//!                              │
//!                              ▼
//!                          planning/
//!                              │
//!                              ▼
//!                         verification/
//! ```
//!
//! It must not depend on diagnosis, policy, planning, recovery, telemetry,
//! hardware, QEC, routing, scheduling, or backend implementations.
//!
//! # Relationship to existing Zamani confidence values
//!
//! Other quantum subsystems may contain domain-specific confidence values,
//! such as measurement confidence or statistical confidence.
//!
//! This type does not replace those domain-specific types.
//!
//! Instead:
//!
//! ```text
//! domain-specific evidence
//!          │
//!          ▼
//! normalization / interpretation
//!          │
//!          ▼
//! resilience::model::Confidence
//! ```
//!
//! Conversion remains the responsibility of the subsystem that understands
//! the original confidence semantics.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::str::FromStr;

// =============================================================================
// Confidence
// =============================================================================

/// A validated normalized confidence value.
///
/// `Confidence` is an epistemic strength-of-evidence value, not a generic
/// probability type.
///
/// The invariant is always:
///
/// ```text
/// 0.0 <= value <= 1.0
/// ```
///
/// and `value` is finite.
///
/// Construction must go through [`Confidence::new`] or another constructor
/// provided by this module so that invalid floating-point values cannot enter
/// the model.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Confidence(f64);

impl Confidence {
    /// The mathematically lowest representable confidence.
    pub const MIN: Self = Self(0.0);

    /// The mathematically highest representable confidence.
    pub const MAX: Self = Self(1.0);

    /// Creates a confidence value from a normalized scalar.
    ///
    /// Returns `None` when `value` is:
    ///
    /// - NaN;
    /// - infinite;
    /// - less than zero;
    /// - greater than one.
    ///
    /// This method never clamps invalid input. Silently clamping malformed
    /// evidence would hide upstream defects and could alter safety decisions.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return None;
        }

        Some(Self(value))
    }

    /// Creates a confidence value from a normalized scalar.
    ///
    /// This is an explicit alias for [`Confidence::new`] intended for callers
    /// whose surrounding domain terminology uses "normalized confidence".
    #[must_use]
    pub fn from_normalized(value: f64) -> Option<Self> {
        Self::new(value)
    }

    /// Returns the normalized scalar representation.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the normalized scalar representation.
    ///
    /// This alias exists for APIs that use "score" terminology.
    #[must_use]
    pub const fn score(self) -> f64 {
        self.0
    }

    /// Returns the minimum confidence.
    #[must_use]
    pub const fn minimum() -> Self {
        Self::MIN
    }

    /// Returns the maximum confidence.
    #[must_use]
    pub const fn maximum() -> Self {
        Self::MAX
    }

    /// Returns whether this confidence is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether this confidence is exactly one.
    #[must_use]
    pub const fn is_certain(self) -> bool {
        self.0 == 1.0
    }

    /// Returns whether this confidence is strictly between zero and one.
    #[must_use]
    pub const fn is_strictly_between_bounds(self) -> bool {
        self.0 > 0.0 && self.0 < 1.0
    }

    /// Returns whether this confidence satisfies an explicitly supplied
    /// minimum confidence requirement.
    ///
    /// No policy threshold is embedded here.
    #[must_use]
    pub fn meets(self, required: Self) -> bool {
        self >= required
    }

    /// Returns whether this confidence is below an explicitly supplied
    /// minimum confidence requirement.
    #[must_use]
    pub fn is_below(self, required: Self) -> bool {
        self < required
    }

    /// Returns the greater of two confidence values.
    ///
    /// This operation is purely ordinal. It does not mean that the stronger
    /// evidence should automatically replace the weaker evidence in a
    /// diagnosis or incident.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the lesser of two confidence values.
    ///
    /// This is useful for conservative decision boundaries where the combined
    /// confidence cannot safely exceed the weakest required evidence.
    ///
    /// It is not a universal statistical evidence-combination rule.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the absolute difference between two confidence values.
    ///
    /// Because both operands are validated and lie in `[0, 1]`, the result is
    /// also finite and lies in `[0, 1]`.
    #[must_use]
    pub fn absolute_difference(self, other: Self) -> Self {
        let difference = (self.0 - other.0).abs();

        // The operands are both finite values in [0, 1], therefore this
        // construction cannot fail. Keeping construction through `new`
        // centralizes the invariant and protects future changes.
        Self::new(difference).unwrap_or(Self::MAX)
    }

    /// Returns the confidence as a percentage for presentation purposes.
    ///
    /// This method is intended only for presentation. The resilience model
    /// itself must continue to operate on normalized values.
    ///
    /// The returned value is in the mathematical interval `[0, 100]`.
    #[must_use]
    pub fn percentage(self) -> f64 {
        self.0 * 100.0
    }

    /// Returns the stable machine-readable textual representation.
    ///
    /// This representation intentionally does not include a `%` suffix.
    #[must_use]
    pub fn as_str(self) -> String {
        self.0.to_string()
    }

    /// Returns a confidence value representing no evidentiary support.
    #[must_use]
    pub const fn none() -> Self {
        Self::MIN
    }

    /// Returns a confidence value representing maximal support.
    ///
    /// This does not assert that the underlying physical system is perfect.
    /// It only represents the maximum confidence expressible by this model.
    #[must_use]
    pub const fn certain() -> Self {
        Self::MAX
    }
}

impl Default for Confidence {
    /// The default confidence is the absence of positive evidentiary support.
    ///
    /// This is intentionally `0.0`, not "unknown".
    ///
    /// Callers that need to distinguish "not established" from "established at
    /// zero" must use `Option<Confidence>`.
    fn default() -> Self {
        Self::MIN
    }
}

impl Eq for Confidence {}

impl Ord for Confidence {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Both values are guaranteed finite by construction.
        //
        // `partial_cmp` therefore cannot return `None`. The explicit fallback
        // makes the invariant local and keeps this implementation robust
        // against future representation changes.
        self.partial_cmp(other)
            .unwrap_or(core::cmp::Ordering::Equal)
    }
}

impl From<Confidence> for f64 {
    fn from(confidence: Confidence) -> Self {
        confidence.0
    }
}

impl fmt::Display for Confidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.to_string())
    }
}

// =============================================================================
// Parsing
// =============================================================================

/// Error-free semantic parsing interface for normalized confidence.
///
/// Invalid values are rejected rather than clamped.
impl FromStr for Confidence {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .parse::<f64>()
            .map_err(|_| "confidence must be a valid finite number")?;

        Self::new(parsed).ok_or(
            "confidence must be finite and within the inclusive range 0.0..=1.0",
        )
    }
}

// =============================================================================
// Confidence classification
// =============================================================================

/// A qualitative interpretation of a confidence value.
///
/// This enum is intentionally NOT used to establish policy thresholds.
///
/// It exists only for code that explicitly needs a semantic description after
/// supplying its own threshold configuration.
///
/// The default model does not map arbitrary confidence values into these
/// categories because doing so would introduce hidden thresholds.
///
/// Consumers should normally compare [`Confidence`] values directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfidenceRelation {
    /// Confidence is below a caller-supplied requirement.
    BelowRequirement,

    /// Confidence satisfies a caller-supplied requirement.
    MeetsRequirement,
}

impl ConfidenceRelation {
    /// Evaluates a confidence value against an explicit requirement.
    #[must_use]
    pub fn evaluate(confidence: Confidence, required: Confidence) -> Self {
        if confidence.meets(required) {
            Self::MeetsRequirement
        } else {
            Self::BelowRequirement
        }
    }

    /// Returns whether the relation means the requirement is satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::MeetsRequirement)
    }
}

// =============================================================================
// Confidence pair
// =============================================================================

/// A pair of confidence values representing an observation and a required
/// decision threshold.
///
/// This is a small foundational helper for policy/verification code.
///
/// It deliberately does not own the policy itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfidenceRequirement {
    observed: Confidence,
    required: Confidence,
}

impl ConfidenceRequirement {
    /// Creates an explicit observation-versus-requirement pair.
    #[must_use]
    pub const fn new(observed: Confidence, required: Confidence) -> Self {
        Self {
            observed,
            required,
        }
    }

    /// Returns the observed confidence.
    #[must_use]
    pub const fn observed(self) -> Confidence {
        self.observed
    }

    /// Returns the required confidence.
    #[must_use]
    pub const fn required(self) -> Confidence {
        self.required
    }

    /// Returns the comparison result.
    #[must_use]
    pub fn relation(self) -> ConfidenceRelation {
        ConfidenceRelation::evaluate(self.observed, self.required)
    }

    /// Returns whether the observation satisfies the requirement.
    #[must_use]
    pub fn is_satisfied(self) -> bool {
        self.observed.meets(self.required)
    }

    /// Returns the remaining confidence gap when the requirement is not met.
    ///
    /// `None` means there is no gap because the requirement is already met.
    #[must_use]
    pub fn deficit(self) -> Option<Confidence> {
        if self.is_satisfied() {
            None
        } else {
            Some(self.required.absolute_difference(self.observed))
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_closed_unit_interval() {
        assert_eq!(Confidence::new(0.0), Some(Confidence::MIN));
        assert_eq!(Confidence::new(1.0), Some(Confidence::MAX));
    }

    #[test]
    fn accepts_finite_interior_value() {
        let confidence = Confidence::new(0.5);

        assert_eq!(confidence.map(Confidence::value), Some(0.5));
    }

    #[test]
    fn rejects_negative_values() {
        assert!(Confidence::new(-0.000_001).is_none());
    }

    #[test]
    fn rejects_values_above_one() {
        assert!(Confidence::new(1.000_001).is_none());
    }

    #[test]
    fn rejects_nan() {
        assert!(Confidence::new(f64::NAN).is_none());
    }

    #[test]
    fn rejects_positive_infinity() {
        assert!(Confidence::new(f64::INFINITY).is_none());
    }

    #[test]
    fn rejects_negative_infinity() {
        assert!(Confidence::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn distinguishes_zero_from_unknown() {
        let zero = Some(Confidence::zero());
        let unknown: Option<Confidence> = None;

        assert_eq!(zero, Some(Confidence::MIN));
        assert!(unknown.is_none());
    }

    #[test]
    fn minimum_and_maximum_are_ordered() {
        assert!(Confidence::MIN < Confidence::MAX);
    }

    #[test]
    fn meets_requirement_is_explicit() {
        let observed = Confidence::new(0.8).expect("valid confidence");
        let required = Confidence::new(0.8).expect("valid confidence");

        assert!(observed.meets(required));
    }

    #[test]
    fn below_requirement_is_explicit() {
        let observed = Confidence::new(0.7).expect("valid confidence");
        let required = Confidence::new(0.8).expect("valid confidence");

        assert!(observed.is_below(required));
    }

    #[test]
    fn minimum_is_conservative() {
        let first = Confidence::new(0.8).expect("valid confidence");
        let second = Confidence::new(0.6).expect("valid confidence");

        assert_eq!(first.min(second).value(), 0.6);
    }

    #[test]
    fn maximum_is_ordinal() {
        let first = Confidence::new(0.8).expect("valid confidence");
        let second = Confidence::new(0.6).expect("valid confidence");

        assert_eq!(first.max(second).value(), 0.8);
    }

    #[test]
    fn absolute_difference_is_normalized() {
        let first = Confidence::new(0.8).expect("valid confidence");
        let second = Confidence::new(0.3).expect("valid confidence");

        assert_eq!(first.absolute_difference(second).value(), 0.5);
    }

    #[test]
    fn absolute_difference_is_symmetric() {
        let first = Confidence::new(0.8).expect("valid confidence");
        let second = Confidence::new(0.3).expect("valid confidence");

        assert_eq!(
            first.absolute_difference(second),
            second.absolute_difference(first)
        );
    }

    #[test]
    fn percentage_is_presentation_only() {
        let confidence = Confidence::new(0.75).expect("valid confidence");

        assert_eq!(confidence.percentage(), 75.0);
    }

    #[test]
    fn display_is_normalized() {
        let confidence = Confidence::new(0.75).expect("valid confidence");

        assert_eq!(confidence.to_string(), "0.75");
    }

    #[test]
    fn parses_valid_confidence() {
        let confidence = "0.625"
            .parse::<Confidence>()
            .expect("valid confidence");

        assert_eq!(confidence.value(), 0.625);
    }

    #[test]
    fn rejects_invalid_parsed_confidence() {
        assert!("2.0".parse::<Confidence>().is_err());
        assert!("-1.0".parse::<Confidence>().is_err());
        assert!("NaN".parse::<Confidence>().is_err());
        assert!("inf".parse::<Confidence>().is_err());
    }

    #[test]
    fn requirement_reports_satisfaction() {
        let observed = Confidence::new(0.9).expect("valid confidence");
        let required = Confidence::new(0.8).expect("valid confidence");

        let requirement = ConfidenceRequirement::new(observed, required);

        assert!(requirement.is_satisfied());
        assert_eq!(requirement.deficit(), None);
    }

    #[test]
    fn requirement_reports_deficit() {
        let observed = Confidence::new(0.6).expect("valid confidence");
        let required = Confidence::new(0.8).expect("valid confidence");

        let requirement = ConfidenceRequirement::new(observed, required);

        assert!(!requirement.is_satisfied());
        assert_eq!(
            requirement
                .deficit()
                .expect("deficit should exist")
                .value(),
            0.2
        );
    }

    #[test]
    fn relation_is_deterministic() {
        let observed = Confidence::new(0.7).expect("valid confidence");
        let required = Confidence::new(0.8).expect("valid confidence");

        assert_eq!(
            ConfidenceRelation::evaluate(observed, required),
            ConfidenceRelation::BelowRequirement
        );
    }

    #[test]
    fn default_is_zero_confidence_not_unknown() {
        assert_eq!(Confidence::default(), Confidence::MIN);
    }

    #[test]
    fn certain_is_maximum_confidence() {
        assert_eq!(Confidence::certain(), Confidence::MAX);
        assert!(Confidence::certain().is_certain());
    }

    #[test]
    fn no_confidence_is_zero() {
        assert_eq!(Confidence::none(), Confidence::MIN);
        assert!(Confidence::none().is_zero());
    }
}