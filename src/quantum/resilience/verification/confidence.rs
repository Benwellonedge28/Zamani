//! Zamani Quantum Resilience — Verification Confidence
//!
//! Path:
//!     src/quantum/resilience/verification/confidence.rs
//!
//! # Purpose
//!
//! This module defines the verification-layer contract for evaluating whether
//! the available evidence is sufficiently strong to support a verification
//! claim.
//!
//! It deliberately builds on the foundational:
//!
//!     crate::quantum::resilience::model::Confidence
//!
//! rather than defining another normalized floating-point confidence type.
//!
//! # Architectural position
//!
//! ```text
//!                    verification evidence
//!                            │
//!                            ▼
//!                 ┌──────────────────────┐
//!                 │ VerificationConfidence│
//!                 └──────────┬───────────┘
//!                            │
//!              ┌─────────────┼──────────────┐
//!              ▼             ▼              ▼
//!          semantic       result       provenance
//!          evidence      evidence       evidence
//!              │             │              │
//!              └─────────────┼──────────────┘
//!                            ▼
//!                  verification/verifier.rs
//!                            │
//!                            ▼
//!                    acceptance decision
//! ```
//!
//! # Critical semantic distinction
//!
//! Confidence is not:
//!
//! - probability of a physical fault;
//! - logical error rate;
//! - hardware fidelity;
//! - severity;
//! - execution success;
//! - semantic equivalence;
//! - authorization;
//! - policy;
//! - recovery success;
//! - resource availability.
//!
//! Confidence describes the strength of evidence supporting a verification
//! claim.
//!
//! A high-confidence claim can still be false if the underlying evidence or
//! verifier is unsound. Consequently, confidence can never replace semantic,
//! invariant, result, provenance, resource-identity, execution, or
//! recovery-integrity checks.
//!
//! # Unknown evidence
//!
//! `Option<Confidence>` is used whenever confidence is not established.
//!
//! ```text
//! None        = confidence unavailable / not established
//! Some(0.0)   = confidence established at zero
//! Some(1.0)   = confidence established at maximum
//! ```
//!
//! Unknown must never be silently converted to zero or one.
//!
//! # Write once, scale everywhere
//!
//! This module contains no assumptions about:
//!
//! - number of qubits;
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of operations;
//! - number of verification stages;
//! - number of backends;
//! - number of machines;
//! - number of evidence records.
//!
//! Confidence is O(1) state per assessment.
//!
//! Collection size, streaming, retention, memory limits, and distributed
//! aggregation belong to their respective layers.
//!
//! # No quantum-resource coupling
//!
//! This file intentionally does not import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Confidence is independent of resource identity.
//!
//! A higher-level verification record may associate an assessment with
//! logical/physical resources, but that relationship belongs to the higher
//! level.
//!
//! # Determinism
//!
//! This module:
//!
//! - performs no I/O;
//! - reads no clock;
//! - accesses no global state;
//! - generates no randomness;
//! - accesses no hardware;
//! - accesses no environment variables.
//!
//! Given identical evidence and configuration, the calculation is deterministic.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
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

use crate::quantum::resilience::model::Confidence;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for verification confidence assessments.
///
/// This identifier is independent of the Rust type name.
pub const VERIFICATION_CONFIDENCE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.verification.confidence";

/// Current semantic version of the verification-confidence contract.
pub const VERIFICATION_CONFIDENCE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Evidence source
// =============================================================================

/// Identifies the semantic origin of evidence used by a verification
/// confidence assessment.
///
/// This is deliberately an enum rather than an arbitrary string in the core
/// calculation path so that the verifier can distinguish evidence classes
/// without relying on provider names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceSource {
    /// Evidence produced by invariant checking.
    Invariant,

    /// Evidence produced by semantic-equivalence checking.
    Semantic,

    /// Evidence produced by execution-result validation.
    Result,

    /// Evidence produced by provenance/integrity validation.
    Provenance,

    /// Evidence concerning logical/physical resource identity.
    ResourceIdentity,

    /// Evidence concerning execution lifecycle state.
    Execution,

    /// Evidence concerning recovery/adaptation integrity.
    RecoveryIntegrity,

    /// Evidence originating from QEC verification.
    Qec,

    /// Evidence originating from an independently verified statistical
    /// analysis.
    Statistical,

    /// Evidence originating from a deterministic replay.
    Replay,

    /// Evidence supplied by another explicitly trusted verifier.
    ExternalVerifier,

    /// Evidence whose domain is known but does not map to a more specific
    /// category.
    Other,
}

impl EvidenceSource {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invariant => "invariant",
            Self::Semantic => "semantic",
            Self::Result => "result",
            Self::Provenance => "provenance",
            Self::ResourceIdentity => "resource_identity",
            Self::Execution => "execution",
            Self::RecoveryIntegrity => "recovery_integrity",
            Self::Qec => "qec",
            Self::Statistical => "statistical",
            Self::Replay => "replay",
            Self::ExternalVerifier => "external_verifier",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for EvidenceSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evidence status
// =============================================================================

/// State of evidence used by a verification assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvidenceStatus {
    /// Evidence is available and usable.
    Available,

    /// Evidence was explicitly evaluated and failed.
    Failed,

    /// Evidence could not be established.
    Unknown,

    /// Evidence exists but is insufficient for the requested verification.
    Insufficient,

    /// Evidence is internally inconsistent.
    Inconsistent,
}

impl EvidenceStatus {
    /// Returns whether evidence is usable as positive verification evidence.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the evidence state explicitly blocks a positive
    /// verification claim.
    #[must_use]
    pub const fn blocks_acceptance(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Unknown | Self::Insufficient | Self::Inconsistent
        )
    }

    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
            Self::Insufficient => "insufficient",
            Self::Inconsistent => "inconsistent",
        }
    }
}

impl fmt::Display for EvidenceStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Confidence requirement
// =============================================================================

/// Explicit confidence requirement supplied by the caller or policy layer.
///
/// This type intentionally contains no operational default threshold.
///
/// A verification requirement is therefore explicit rather than hidden in
/// this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConfidenceRequirement {
    minimum: Confidence,
}

impl ConfidenceRequirement {
    /// Creates a confidence requirement.
    #[must_use]
    pub const fn new(minimum: Confidence) -> Self {
        Self { minimum }
    }

    /// Returns the required minimum confidence.
    #[must_use]
    pub const fn minimum(self) -> Confidence {
        self.minimum
    }

    /// Returns whether the supplied confidence satisfies this requirement.
    #[must_use]
    pub fn is_satisfied_by(self, confidence: Confidence) -> bool {
        confidence.meets(self.minimum)
    }
}

impl From<Confidence> for ConfidenceRequirement {
    fn from(value: Confidence) -> Self {
        Self::new(value)
    }
}

// =============================================================================
// Verification confidence assessment
// =============================================================================

/// A verification-layer confidence assessment.
///
/// The assessment keeps confidence separate from the semantic outcome itself.
/// A caller must therefore never interpret `confidence == high` as equivalent
/// to `verified == true`.
///
/// # Invariants
///
/// - `confidence` is finite and normalized because it is the validated model
///   type from `resilience::model`.
/// - `status` describes evidence availability independently of confidence.
/// - `source` describes where the evidence came from.
/// - `requirement`, when present, is explicit and caller supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerificationConfidenceAssessment {
    source: EvidenceSource,
    status: EvidenceStatus,
    confidence: Option<Confidence>,
    requirement: Option<ConfidenceRequirement>,
}

impl VerificationConfidenceAssessment {
    /// Creates an assessment.
    ///
    /// No implicit confidence is generated.
    #[must_use]
    pub const fn new(
        source: EvidenceSource,
        status: EvidenceStatus,
        confidence: Option<Confidence>,
        requirement: Option<ConfidenceRequirement>,
    ) -> Self {
        Self {
            source,
            status,
            confidence,
            requirement,
        }
    }

    /// Creates an assessment for available evidence.
    ///
    /// Available evidence must carry an actual confidence value.
    ///
    /// This constructor returns `None` when confidence is absent because an
    /// available-but-unquantified evidence claim must not be silently treated
    /// as either zero or one.
    #[must_use]
    pub const fn available(
        source: EvidenceSource,
        confidence: Confidence,
        requirement: Option<ConfidenceRequirement>,
    ) -> Self {
        Self {
            source,
            status: EvidenceStatus::Available,
            confidence: Some(confidence),
            requirement,
        }
    }

    /// Creates an unavailable/unknown evidence assessment.
    #[must_use]
    pub const fn unknown(
        source: EvidenceSource,
        requirement: Option<ConfidenceRequirement>,
    ) -> Self {
        Self {
            source,
            status: EvidenceStatus::Unknown,
            confidence: None,
            requirement,
        }
    }

    /// Creates an explicitly failed evidence assessment.
    #[must_use]
    pub const fn failed(
        source: EvidenceSource,
        confidence: Option<Confidence>,
        requirement: Option<ConfidenceRequirement>,
    ) -> Self {
        Self {
            source,
            status: EvidenceStatus::Failed,
            confidence,
            requirement,
        }
    }

    /// Returns the evidence source.
    #[must_use]
    pub const fn source(self) -> EvidenceSource {
        self.source
    }

    /// Returns the evidence status.
    #[must_use]
    pub const fn status(self) -> EvidenceStatus {
        self.status
    }

    /// Returns the confidence, if established.
    #[must_use]
    pub const fn confidence(self) -> Option<Confidence> {
        self.confidence
    }

    /// Returns the explicit requirement, if one was supplied.
    #[must_use]
    pub const fn requirement(self) -> Option<ConfidenceRequirement> {
        self.requirement
    }

    /// Returns whether evidence exists and is usable.
    #[must_use]
    pub const fn is_available(self) -> bool {
        self.status.is_usable() && self.confidence.is_some()
    }

    /// Returns whether the evidence itself explicitly blocks acceptance.
    #[must_use]
    pub const fn blocks_acceptance(self) -> bool {
        self.status.blocks_acceptance()
    }

    /// Returns whether the confidence requirement is satisfied.
    ///
    /// `false` is returned when confidence or the requirement is unavailable.
    ///
    /// This is intentionally conservative.
    #[must_use]
    pub fn meets_requirement(self) -> bool {
        match (self.confidence, self.requirement) {
            (Some(confidence), Some(requirement)) => {
                requirement.is_satisfied_by(confidence)
            }
            _ => false,
        }
    }

    /// Returns whether this assessment is sufficient as positive confidence
    /// evidence under its own explicit requirement.
    ///
    /// This method does not assert semantic correctness.
    #[must_use]
    pub fn is_sufficient(self) -> bool {
        self.is_available() && self.meets_requirement()
    }
}

// =============================================================================
// Confidence evaluation result
// =============================================================================

/// Result of evaluating one or more verification confidence assessments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfidenceDecision {
    /// Confidence evidence satisfies all supplied requirements.
    Sufficient,

    /// Evidence exists but at least one confidence requirement is not met.
    Insufficient,

    /// At least one required evidence source is unavailable or unknown.
    Indeterminate,

    /// At least one evidence source explicitly failed or was inconsistent.
    Rejected,
}

impl ConfidenceDecision {
    /// Whether confidence evidence alone is sufficient for its configured
    /// confidence requirement.
    ///
    /// This must not be interpreted as overall verification success.
    #[must_use]
    pub const fn is_sufficient(self) -> bool {
        matches!(self, Self::Sufficient)
    }

    /// Whether the decision requires additional evidence before a confidence
    /// claim can be established.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Whether the decision contains an explicit negative evidence signal.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sufficient => "sufficient",
            Self::Insufficient => "insufficient",
            Self::Indeterminate => "indeterminate",
            Self::Rejected => "rejected",
        }
    }
}

impl fmt::Display for ConfidenceDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Confidence evaluator
// =============================================================================

/// Stateless evaluator for verification confidence.
///
/// The evaluator intentionally does not know about:
//!
//! - quantum topology;
//! - qubit counts;
//! - hardware providers;
//! - QPU sizes;
//! - routing;
//! - scheduling;
//! - optimization;
//! - recovery implementation;
//! - backend implementation.
///
/// It only evaluates supplied verification evidence.
///
/// This keeps the verification-confidence layer reusable from a single qubit
/// through arbitrarily large distributed quantum systems.
#[derive(Debug, Clone, Copy, Default)]
pub struct ConfidenceEvaluator;

impl ConfidenceEvaluator {
    /// Creates a confidence evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates a single assessment.
    ///
    /// The ordering is intentionally conservative:
    ///
    /// 1. explicit failure/inconsistency;
    /// 2. unavailable/unknown evidence;
    /// 3. missing confidence;
    /// 4. unsatisfied requirement;
    /// 5. sufficient confidence.
    ///
    /// This prevents an unavailable measurement from being interpreted as
    /// low-confidence-but-usable evidence.
    #[must_use]
    pub fn evaluate(
        &self,
        assessment: VerificationConfidenceAssessment,
    ) -> ConfidenceDecision {
        match assessment.status() {
            EvidenceStatus::Failed | EvidenceStatus::Inconsistent => {
                ConfidenceDecision::Rejected
            }
            EvidenceStatus::Unknown => ConfidenceDecision::Indeterminate,
            EvidenceStatus::Insufficient => ConfidenceDecision::Insufficient,
            EvidenceStatus::Available => {
                if assessment.confidence().is_none() {
                    ConfidenceDecision::Indeterminate
                } else if assessment.meets_requirement() {
                    ConfidenceDecision::Sufficient
                } else {
                    ConfidenceDecision::Insufficient
                }
            }
        }
    }

    /// Evaluates a collection of assessments.
    ///
    /// No fixed collection size is assumed.
    ///
    /// The method consumes an iterator, allowing callers to provide:
    ///
    /// - arrays;
    /// - slices;
    /// - vectors;
    /// - streaming iterators;
    /// - generated evidence;
    /// - distributed evidence after deterministic ordering.
    ///
    /// The evaluation is deliberately conservative:
    ///
    /// - any explicit failure/inconsistency => `Rejected`;
    /// - otherwise any unknown => `Indeterminate`;
    /// - otherwise any unmet requirement => `Insufficient`;
    /// - otherwise => `Sufficient`.
    ///
    /// This does not calculate a universal statistical aggregate. It evaluates
    /// whether each supplied assessment independently satisfies its own
    /// contract.
    #[must_use]
    pub fn evaluate_all<I>(&self, assessments: I) -> ConfidenceDecision
    where
        I: IntoIterator<Item = VerificationConfidenceAssessment>,
    {
        let mut saw_assessment = false;
        let mut saw_indeterminate = false;
        let mut saw_insufficient = false;

        for assessment in assessments {
            saw_assessment = true;

            match self.evaluate(assessment) {
                ConfidenceDecision::Rejected => return ConfidenceDecision::Rejected,
                ConfidenceDecision::Indeterminate => saw_indeterminate = true,
                ConfidenceDecision::Insufficient => saw_insufficient = true,
                ConfidenceDecision::Sufficient => {}
            }
        }

        if !saw_assessment {
            return ConfidenceDecision::Indeterminate;
        }

        if saw_indeterminate {
            return ConfidenceDecision::Indeterminate;
        }

        if saw_insufficient {
            return ConfidenceDecision::Insufficient;
        }

        ConfidenceDecision::Sufficient
    }
}

// =============================================================================
// Conservative confidence projection
// =============================================================================

/// Projects the weakest established confidence from an iterator.
///
/// This is an ordinal minimum, not a statistical combination formula.
///
/// `None` is returned when no confidence values are supplied.
///
/// This function is useful when a verification contract explicitly defines
/// "overall confidence" as the weakest required evidence. Callers whose
/// mathematical model requires another aggregation rule must implement that
/// rule in the appropriate domain-specific subsystem.
#[must_use]
pub fn minimum_confidence<I>(confidences: I) -> Option<Confidence>
where
    I: IntoIterator<Item = Confidence>,
{
    let mut minimum: Option<Confidence> = None;

    for confidence in confidences {
        minimum = Some(match minimum {
            Some(current) => current.min(confidence),
            None => confidence,
        });
    }

    minimum
}

/// Projects the strongest established confidence from an iterator.
///
/// This is an ordinal maximum, not a statistical combination formula.
///
/// `None` is returned when no confidence values are supplied.
#[must_use]
pub fn maximum_confidence<I>(confidences: I) -> Option<Confidence>
where
    I: IntoIterator<Item = Confidence>,
{
    let mut maximum: Option<Confidence> = None;

    for confidence in confidences {
        maximum = Some(match maximum {
            Some(current) => current.max(confidence),
            None => confidence,
        });
    }

    maximum
}

// =============================================================================
// Requirement comparison
// =============================================================================

/// Compares an optional confidence value against an explicit requirement.
///
/// The result is `false` when confidence is unavailable.
///
/// This function intentionally does not treat missing confidence as zero.
#[must_use]
pub fn confidence_meets_requirement(
    confidence: Option<Confidence>,
    requirement: ConfidenceRequirement,
) -> bool {
    match confidence {
        Some(value) => requirement.is_satisfied_by(value),
        None => false,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn confidence(value: f64) -> Confidence {
        Confidence::new(value).expect("test confidence must be valid")
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            VERIFICATION_CONFIDENCE_SCHEMA_ID,
            "zamani.quantum.resilience.verification.confidence"
        );
        assert_eq!(VERIFICATION_CONFIDENCE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn available_evidence_requires_confidence() {
        let assessment = VerificationConfidenceAssessment::available(
            EvidenceSource::Semantic,
            confidence(1.0),
            None,
        );

        assert!(assessment.is_available());
        assert!(!assessment.meets_requirement());
    }

    #[test]
    fn requirement_is_explicit() {
        let requirement = ConfidenceRequirement::new(confidence(0.8));

        assert!(requirement.is_satisfied_by(confidence(0.8)));
        assert!(requirement.is_satisfied_by(confidence(1.0)));
        assert!(!requirement.is_satisfied_by(confidence(0.79)));
    }

    #[test]
    fn unknown_is_not_zero() {
        let assessment =
            VerificationConfidenceAssessment::unknown(EvidenceSource::Result, None);

        assert_eq!(assessment.confidence(), None);
        assert!(!assessment.is_available());
        assert!(!assessment.meets_requirement());
    }

    #[test]
    fn failed_evidence_is_rejected() {
        let assessment = VerificationConfidenceAssessment::failed(
            EvidenceSource::Semantic,
            Some(confidence(1.0)),
            None,
        );

        let evaluator = ConfidenceEvaluator::new();

        assert_eq!(
            evaluator.evaluate(assessment),
            ConfidenceDecision::Rejected
        );
    }

    #[test]
    fn unknown_evidence_is_indeterminate() {
        let assessment =
            VerificationConfidenceAssessment::unknown(EvidenceSource::Result, None);

        let evaluator = ConfidenceEvaluator::new();

        assert_eq!(
            evaluator.evaluate(assessment),
            ConfidenceDecision::Indeterminate
        );
    }

    #[test]
    fn insufficient_confidence_is_not_rejected() {
        let assessment = VerificationConfidenceAssessment::available(
            EvidenceSource::Semantic,
            confidence(0.5),
            Some(ConfidenceRequirement::new(confidence(0.8))),
        );

        let evaluator = ConfidenceEvaluator::new();

        assert_eq!(
            evaluator.evaluate(assessment),
            ConfidenceDecision::Insufficient
        );
    }

    #[test]
    fn sufficient_confidence_is_sufficient() {
        let assessment = VerificationConfidenceAssessment::available(
            EvidenceSource::Semantic,
            confidence(0.9),
            Some(ConfidenceRequirement::new(confidence(0.8))),
        );

        let evaluator = ConfidenceEvaluator::new();

        assert_eq!(
            evaluator.evaluate(assessment),
            ConfidenceDecision::Sufficient
        );
    }

    #[test]
    fn explicit_failure_has_priority_over_unknown() {
        let evaluator = ConfidenceEvaluator::new();

        let assessments = [
            VerificationConfidenceAssessment::unknown(EvidenceSource::Result, None),
            VerificationConfidenceAssessment::failed(
                EvidenceSource::Semantic,
                Some(confidence(0.9)),
                None,
            ),
        ];

        assert_eq!(
            evaluator.evaluate_all(assessments),
            ConfidenceDecision::Rejected
        );
    }

    #[test]
    fn unknown_has_priority_over_insufficient() {
        let evaluator = ConfidenceEvaluator::new();

        let assessments = [
            VerificationConfidenceAssessment::available(
                EvidenceSource::Result,
                confidence(0.4),
                Some(ConfidenceRequirement::new(confidence(0.8))),
            ),
            VerificationConfidenceAssessment::unknown(EvidenceSource::Semantic, None),
        ];

        assert_eq!(
            evaluator.evaluate_all(assessments),
            ConfidenceDecision::Indeterminate
        );
    }

    #[test]
    fn empty_evidence_is_indeterminate() {
        let evaluator = ConfidenceEvaluator::new();

        let assessments: [VerificationConfidenceAssessment; 0] = [];

        assert_eq!(
            evaluator.evaluate_all(assessments),
            ConfidenceDecision::Indeterminate
        );
    }

    #[test]
    fn minimum_confidence_is_ordered() {
        let result = minimum_confidence([
            confidence(0.9),
            confidence(0.7),
            confidence(1.0),
            confidence(0.8),
        ]);

        assert_eq!(result, Some(confidence(0.7)));
    }

    #[test]
    fn maximum_confidence_is_ordered() {
        let result = maximum_confidence([
            confidence(0.9),
            confidence(0.7),
            confidence(1.0),
            confidence(0.8),
        ]);

        assert_eq!(result, Some(confidence(1.0)));
    }

    #[test]
    fn empty_projection_is_unknown() {
        let values: [Confidence; 0] = [];

        assert_eq!(minimum_confidence(values), None);
    }

    #[test]
    fn confidence_requirement_helper_is_conservative() {
        let requirement = ConfidenceRequirement::new(confidence(0.8));

        assert!(confidence_meets_requirement(
            Some(confidence(0.9)),
            requirement
        ));

        assert!(!confidence_meets_requirement(None, requirement));
    }

    #[test]
    fn evidence_source_names_are_stable() {
        assert_eq!(EvidenceSource::Invariant.as_str(), "invariant");
        assert_eq!(EvidenceSource::Semantic.as_str(), "semantic");
        assert_eq!(EvidenceSource::Result.as_str(), "result");
        assert_eq!(
            EvidenceSource::ResourceIdentity.as_str(),
            "resource_identity"
        );
        assert_eq!(EvidenceSource::Qec.as_str(), "qec");
    }

    #[test]
    fn evidence_status_semantics_are_stable() {
        assert!(EvidenceStatus::Available.is_usable());
        assert!(!EvidenceStatus::Unknown.is_usable());
        assert!(EvidenceStatus::Failed.blocks_acceptance());
        assert!(EvidenceStatus::Unknown.blocks_acceptance());
        assert!(!EvidenceStatus::Available.blocks_acceptance());
    }

    #[test]
    fn no_hidden_threshold_is_used() {
        let exact_requirement = ConfidenceRequirement::new(confidence(0.5));

        let assessment = VerificationConfidenceAssessment::available(
            EvidenceSource::Result,
            confidence(0.5),
            Some(exact_requirement),
        );

        assert!(assessment.meets_requirement());
    }
}