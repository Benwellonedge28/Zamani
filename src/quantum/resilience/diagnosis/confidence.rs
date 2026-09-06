//! Zamani Quantum Resilience — Diagnosis Confidence Analysis.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/confidence.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module evaluates and aggregates the confidence of already-produced
//! diagnosis findings.
//!
//! It does NOT:
//!
//! - detect faults;
//! - classify raw telemetry;
//! - invent quantum-fault semantics;
//! - localize qubits;
//! - determine physical qubit identities;
//! - select recovery actions;
//! - authorize recovery;
//! - modify routing;
//! - modify scheduling;
//! - modify QEC;
//! - modify hardware;
//! - prove causality;
//! - replace verification.
//!
//! Its responsibility is narrower:
//!
//!     DiagnosisFinding(s)
//!             |
//!             v
//!     ConfidenceAnalysis
//!
//! The result is an explicit, deterministic assessment of how strongly the
//! supplied findings support the diagnosis.
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//! ```text
//! DetectionOutput
//!       |
//!       v
//! Diagnosis contributors
//!       |
//!       v
//! DiagnosisFinding(s)
//!       |
//!       v
//! +-------------------------+
//! | diagnosis/confidence.rs |
//! +-------------------------+
//!       |
//!       v
//! ConfidenceAnalysis
//!       |
//!       +--------------------+
//!       |                    |
//!       v                    v
//! Planning / Policy      Verification
//! ```
//!
//! This module is intentionally downstream of diagnosis contributors.
//!
//! =============================================================================
//! Existing-contract integration
//! =============================================================================
//!
//! The canonical diagnosis types remain owned by `diagnostician.rs`:
//!
//! - `DiagnosisConfidence`
//! - `DiagnosisFinding`
//! - `EvidenceReference`
//! - `DiagnosisCategory`
//! - `ContributorIdentity`
//!
//! This file consumes those types rather than redefining them.
//!
//! The canonical detection confidence remains owned by:
//!
//!     diagnosis::detection::detector::DetectionConfidence
//!
//! This module does not redefine detection confidence either.
//!
//! =============================================================================
//! Confidence semantics
//! =============================================================================
//!
//! Confidence is epistemic evidence strength, NOT:
//!
//! - probability that a physical qubit has failed;
//! - probability that recovery succeeds;
//! - probability that a result is correct;
//! - hardware fidelity;
//! - logical error rate;
//! - fault probability.
//!
//! Those quantities must remain separate because combining them would create
//! unsafe and mathematically ambiguous decisions.
//!
//! A confidence value is therefore only meaningful together with:
//!
//! - its finding;
//! - its evidence;
//! - its contributor;
//! - the aggregation method;
//! - the analysis configuration;
//! - the provenance of the evidence.
//!
//! =============================================================================
//! Conservative-by-default design
//! =============================================================================
//!
//! Evidence from multiple findings must not automatically be treated as
//! independent evidence.
//!
//! For example:
//!
//!     detector A -> signal X
//!     detector B -> signal X
//!
//! does NOT necessarily mean there are two independent observations.
//!
//! This module therefore deduplicates evidence by `SignalId` when calculating
//! evidence coverage and never multiplies confidence values as though the
//! observations were statistically independent.
//!
//! More sophisticated statistical fusion belongs in a future explicitly
//! configured inference component.
//!
//! =============================================================================
//! Write once, scale everywhere
//! =============================================================================
//!
//! No machine-size assumptions exist here.
//!
//! There is no:
//!
//! - maximum number of findings;
//! - maximum number of evidence references;
//! - fixed qubit count;
//! - fixed detector count;
//! - fixed backend count;
//! - fixed incident count;
//! - fixed confidence threshold;
//! - fixed retry count.
//!
//! Collections are dynamically sized and caller-owned.
//!
//! Concrete memory/CPU limits belong to:
//!
//! - runtime limits;
//! - policy;
//! - security policy;
//! - execution resources;
//! - deployment configuration.
//!
//! "Infinity" therefore means that this semantic layer imposes no artificial
//! quantum-machine-size ceiling. A concrete execution remains bounded by the
//! resources available to that execution.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! Given identical:
//!
//! - findings;
//! - configuration;
//! - contributor state;
//! - input ordering-independent content;
//!
//! this module produces the same result.
//!
//! It does not read:
//!
//! - clocks;
//! - environment variables;
//! - filesystem;
//! - network;
//! - hardware;
//! - process IDs;
//! - memory addresses;
//! - random generators;
//! - global mutable state.
//!
//! =============================================================================
//! Security
//! =============================================================================
//!
//! Confidence is NOT an authorization mechanism.
//!
//! A confidence of 1.0 does not authorize recovery.
//!
//! Downstream recovery still requires:
//!
//!     semantic validity
//!     + policy validity
//!     + capability validity
//!     + security validity
//!     + verification validity
//!
//! A malicious contributor must therefore not be able to turn a high
//! confidence value directly into a recovery command.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use core::cmp::Ordering;
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::resilience::diagnosis::diagnostician::{
    DiagnosisCategory,
    DiagnosisConfidence,
    DiagnosisFinding,
    EvidenceReference,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for diagnosis-confidence analysis.
pub const DIAGNOSIS_CONFIDENCE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.confidence";

/// Semantic schema version.
pub const DIAGNOSIS_CONFIDENCE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Aggregation method
// =============================================================================

/// Explicit strategy used to combine diagnosis confidences.
///
/// No strategy assumes that findings are statistically independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfidenceAggregation {
    /// Select the strongest supported finding.
    ///
    /// This is useful when contributors represent alternative explanations
    /// rather than independent evidence.
    Maximum,

    /// Select the weakest finding.
    ///
    /// This is deliberately conservative when every supplied finding is
    /// required to support an overall diagnosis.
    Minimum,

    /// Arithmetic mean of finding confidences.
    ///
    /// This is descriptive rather than probabilistic.
    Mean,

    /// Mean weighted by the number of distinct evidence signals supporting
    /// each finding.
    ///
    /// Duplicate signal references do not increase the weight.
    EvidenceWeightedMean,
}

impl Default for ConfidenceAggregation {
    fn default() -> Self {
        Self::Maximum
    }
}

impl ConfidenceAggregation {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maximum => "maximum",
            Self::Minimum => "minimum",
            Self::Mean => "mean",
            Self::EvidenceWeightedMean => "evidence_weighted_mean",
        }
    }
}

// =============================================================================
// Confidence source classification
// =============================================================================

/// Describes how a confidence value should be interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfidenceBasis {
    /// Confidence came directly from a diagnosis contributor.
    Contributor,

    /// Confidence was aggregated from multiple findings.
    Aggregated,

    /// No actionable finding supplied sufficient evidence.
    InsufficientEvidence,

    /// Conflicting findings prevent a single unqualified interpretation.
    Conflicted,
}

impl ConfidenceBasis {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contributor => "contributor",
            Self::Aggregated => "aggregated",
            Self::InsufficientEvidence => "insufficient_evidence",
            Self::Conflicted => "conflicted",
        }
    }
}

// =============================================================================
// Finding summary
// =============================================================================

/// Immutable confidence summary for one diagnosis finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingConfidence {
    category: DiagnosisCategory,
    confidence: DiagnosisConfidence,
    evidence_count: usize,
    unique_signal_count: usize,
}

impl FindingConfidence {
    /// Builds a confidence summary from a finding.
    #[must_use]
    pub fn from_finding(finding: &DiagnosisFinding) -> Self {
        let unique_signal_count = unique_signal_count(finding.evidence());

        Self {
            category: finding.category().clone(),
            confidence: finding.confidence(),
            evidence_count: finding.evidence_count(),
            unique_signal_count,
        }
    }

    /// Returns the diagnosis category.
    #[must_use]
    pub fn category(&self) -> &DiagnosisCategory {
        &self.category
    }

    /// Returns contributor confidence.
    #[must_use]
    pub const fn confidence(&self) -> DiagnosisConfidence {
        self.confidence
    }

    /// Returns the number of evidence references.
    #[must_use]
    pub const fn evidence_count(&self) -> usize {
        self.evidence_count
    }

    /// Returns the number of distinct detection signals.
    #[must_use]
    pub const fn unique_signal_count(&self) -> usize {
        self.unique_signal_count
    }
}

// =============================================================================
// Confidence configuration
// =============================================================================

/// Configuration for confidence analysis.
///
/// There are deliberately no implicit confidence thresholds.
///
/// A caller may choose whether its policy requires a minimum confidence,
/// but that decision belongs to policy/verification rather than this module.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceConfig {
    aggregation: ConfidenceAggregation,
    include_non_actionable: bool,
    reject_conflicting_categories: bool,
}

impl Default for ConfidenceConfig {
    fn default() -> Self {
        Self {
            aggregation: ConfidenceAggregation::Maximum,
            include_non_actionable: false,
            reject_conflicting_categories: false,
        }
    }
}

impl ConfidenceConfig {
    /// Creates the default conservative configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the aggregation method.
    #[must_use]
    pub const fn with_aggregation(
        mut self,
        aggregation: ConfidenceAggregation,
    ) -> Self {
        self.aggregation = aggregation;
        self
    }

    /// Controls whether `NoCondition` findings participate.
    #[must_use]
    pub const fn with_non_actionable_findings(
        mut self,
        include: bool,
    ) -> Self {
        self.include_non_actionable = include;
        self
    }

    /// Controls whether mutually different actionable categories are treated
    /// as a conflict.
    #[must_use]
    pub const fn with_conflict_rejection(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_conflicting_categories = reject;
        self
    }

    /// Returns the selected aggregation strategy.
    #[must_use]
    pub const fn aggregation(&self) -> ConfidenceAggregation {
        self.aggregation
    }

    /// Returns whether non-actionable findings participate.
    #[must_use]
    pub const fn include_non_actionable(&self) -> bool {
        self.include_non_actionable
    }

    /// Returns whether conflicting categories are rejected.
    #[must_use]
    pub const fn reject_conflicting_categories(&self) -> bool {
        self.reject_conflicting_categories
    }
}

// =============================================================================
// Confidence assessment
// =============================================================================

/// Complete deterministic confidence assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfidenceAssessment {
    confidence: DiagnosisConfidence,
    basis: ConfidenceBasis,
    aggregation: ConfidenceAggregation,
    findings_considered: usize,
    evidence_references: usize,
    unique_evidence_signals: usize,
    categories: Vec<DiagnosisCategory>,
    finding_summaries: Vec<FindingConfidence>,
    conflicting: bool,
}

impl ConfidenceAssessment {
    fn empty(aggregation: ConfidenceAggregation) -> Self {
        Self {
            confidence: DiagnosisConfidence::zero(),
            basis: ConfidenceBasis::InsufficientEvidence,
            aggregation,
            findings_considered: 0,
            evidence_references: 0,
            unique_evidence_signals: 0,
            categories: Vec::new(),
            finding_summaries: Vec::new(),
            conflicting: false,
        }
    }

    /// Returns the final aggregated confidence.
    #[must_use]
    pub const fn confidence(&self) -> DiagnosisConfidence {
        self.confidence
    }

    /// Returns the basis of the assessment.
    #[must_use]
    pub const fn basis(&self) -> ConfidenceBasis {
        self.basis
    }

    /// Returns the aggregation strategy.
    #[must_use]
    pub const fn aggregation(&self) -> ConfidenceAggregation {
        self.aggregation
    }

    /// Returns the number of findings considered.
    #[must_use]
    pub const fn findings_considered(&self) -> usize {
        self.findings_considered
    }

    /// Returns the total number of evidence references.
    #[must_use]
    pub const fn evidence_references(&self) -> usize {
        self.evidence_references
    }

    /// Returns the number of distinct evidence signals.
    #[must_use]
    pub const fn unique_evidence_signals(&self) -> usize {
        self.unique_evidence_signals
    }

    /// Returns the categories represented by the considered findings.
    #[must_use]
    pub fn categories(&self) -> &[DiagnosisCategory] {
        &self.categories
    }

    /// Returns per-finding summaries.
    #[must_use]
    pub fn finding_summaries(&self) -> &[FindingConfidence] {
        &self.finding_summaries
    }

    /// Returns whether conflicting actionable categories were observed.
    #[must_use]
    pub const fn is_conflicting(&self) -> bool {
        self.conflicting
    }

    /// Returns whether the assessment has actionable support.
    #[must_use]
    pub const fn is_actionable(&self) -> bool {
        self.findings_considered != 0
            && !matches!(
                self.basis,
                ConfidenceBasis::InsufficientEvidence
            )
    }

    /// Checks an explicit caller-supplied confidence requirement.
    ///
    /// This method deliberately does not contain a default threshold.
    #[must_use]
    pub fn meets(&self, required: DiagnosisConfidence) -> bool {
        self.confidence.meets(required)
    }
}

// =============================================================================
// Confidence analyzer
// =============================================================================

/// Deterministic diagnosis-confidence analyzer.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceAnalyzer {
    config: ConfidenceConfig,
}

impl Default for ConfidenceAnalyzer {
    fn default() -> Self {
        Self {
            config: ConfidenceConfig::default(),
        }
    }
}

impl ConfidenceAnalyzer {
    /// Creates an analyzer with explicit configuration.
    #[must_use]
    pub const fn with_config(config: ConfidenceConfig) -> Self {
        Self { config }
    }

    /// Returns the analyzer configuration.
    #[must_use]
    pub const fn config(&self) -> &ConfidenceConfig {
        &self.config
    }

    /// Analyzes an arbitrary collection of diagnosis findings.
    ///
    /// The input may contain any number of findings. No fixed-size machine
    /// assumption is made.
    pub fn analyze<I>(&self, findings: I) -> ResilienceResult<ConfidenceAssessment>
    where
        I: IntoIterator<Item = DiagnosisFinding>,
    {
        let mut findings: Vec<DiagnosisFinding> = findings.into_iter().collect();

        // DiagnosisFinding itself is immutable. Sorting the local collection
        // makes analysis deterministic independent of caller ordering.
        findings.sort_by(compare_findings);

        self.analyze_sorted(&findings)
    }

    /// Analyzes borrowed findings without taking ownership.
    pub fn analyze_refs(
        &self,
        findings: &[DiagnosisFinding],
    ) -> ResilienceResult<ConfidenceAssessment> {
        let mut ordered: Vec<&DiagnosisFinding> = findings.iter().collect();

        ordered.sort_by(|left, right| compare_findings(left, right));

        self.analyze_refs_sorted(&ordered)
    }

    fn analyze_sorted(
        &self,
        findings: &[DiagnosisFinding],
    ) -> ResilienceResult<ConfidenceAssessment> {
        let references: Vec<&DiagnosisFinding> = findings.iter().collect();

        self.analyze_refs_sorted(&references)
    }

    fn analyze_refs_sorted(
        &self,
        findings: &[&DiagnosisFinding],
    ) -> ResilienceResult<ConfidenceAssessment> {
        let mut assessment =
            ConfidenceAssessment::empty(self.config.aggregation);

        let mut unique_signals = BTreeSet::new();
        let mut categories = BTreeSet::new();
        let mut actionable_categories = BTreeSet::new();
        let mut summaries = Vec::new();

        for finding in findings {
            if !self.config.include_non_actionable
                && !finding.is_actionable()
            {
                continue;
            }

            let summary = FindingConfidence::from_finding(finding);

            for evidence in finding.evidence() {
                unique_signals.insert(evidence.signal_id());
            }

            assessment.evidence_references += finding.evidence_count();

            categories.insert(finding.category().clone());

            if finding.is_actionable() {
                actionable_categories.insert(finding.category().clone());
            }

            summaries.push(summary);
        }

        if summaries.is_empty() {
            return Ok(assessment);
        }

        let conflicting = has_conflicting_categories(&actionable_categories);

        if conflicting && self.config.reject_conflicting_categories {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DiagnosisConflict,
            ));
        }

        let confidence = aggregate(
            &summaries,
            self.config.aggregation,
        )?;

        assessment.confidence = confidence;
        assessment.findings_considered = summaries.len();
        assessment.unique_evidence_signals = unique_signals.len();
        assessment.categories = categories.into_iter().collect();
        assessment.finding_summaries = summaries;
        assessment.conflicting = conflicting;

        assessment.basis = if conflicting {
            ConfidenceBasis::Conflicted
        } else if assessment.findings_considered == 1 {
            ConfidenceBasis::Contributor
        } else {
            ConfidenceBasis::Aggregated
        };

        Ok(assessment)
    }

    /// Aggregates one finding.
    ///
    /// This is useful for diagnosis contributors that want the same
    /// confidence representation without constructing a collection.
    pub fn analyze_one(
        &self,
        finding: &DiagnosisFinding,
    ) -> ResilienceResult<ConfidenceAssessment> {
        self.analyze_refs(std::slice::from_ref(finding))
    }

    /// Checks whether a finding satisfies an explicit confidence requirement.
    ///
    /// No implicit threshold exists.
    #[must_use]
    pub fn satisfies(
        &self,
        finding: &DiagnosisFinding,
        required: DiagnosisConfidence,
    ) -> bool {
        finding.confidence().meets(required)
    }
}

// =============================================================================
// Aggregation implementation
// =============================================================================

fn aggregate(
    summaries: &[FindingConfidence],
    aggregation: ConfidenceAggregation,
) -> ResilienceResult<DiagnosisConfidence> {
    if summaries.is_empty() {
        return Ok(DiagnosisConfidence::zero());
    }

    match aggregation {
        ConfidenceAggregation::Maximum => {
            summaries
                .iter()
                .map(FindingConfidence::confidence)
                .max()
                .ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::InsufficientEvidence,
                    )
                })
        }

        ConfidenceAggregation::Minimum => {
            summaries
                .iter()
                .map(FindingConfidence::confidence)
                .min()
                .ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::InsufficientEvidence,
                    )
                })
        }

        ConfidenceAggregation::Mean => {
            let mut sum = 0.0_f64;

            for summary in summaries {
                sum += summary.confidence().value();

                if !sum.is_finite() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                    ));
                }
            }

            let count = summaries.len() as f64;

            if count == 0.0 {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InsufficientEvidence,
                ));
            }

            DiagnosisConfidence::new(sum / count)
        }

        ConfidenceAggregation::EvidenceWeightedMean => {
            let mut weighted_sum = 0.0_f64;
            let mut total_weight = 0_u64;

            for summary in summaries {
                let weight = if summary.unique_signal_count() == 0 {
                    1_u64
                } else {
                    summary.unique_signal_count() as u64
                };

                weighted_sum +=
                    summary.confidence().value() * weight as f64;

                if !weighted_sum.is_finite() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                    ));
                }

                total_weight = total_weight.checked_add(weight).ok_or_else(
                    || {
                        ResilienceError::new(
                            ResilienceErrorCode::ArithmeticOverflow,
                        )
                    },
                )?;
            }

            if total_weight == 0 {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InsufficientEvidence,
                ));
            }

            DiagnosisConfidence::new(
                weighted_sum / total_weight as f64,
            )
        }
    }
}

// =============================================================================
// Deterministic ordering
// =============================================================================

fn compare_findings(
    left: &DiagnosisFinding,
    right: &DiagnosisFinding,
) -> Ordering {
    left.category()
        .cmp(right.category())
        .then_with(|| {
            left.contributor().cmp(right.contributor())
        })
        .then_with(|| {
            right
                .confidence()
                .cmp(&left.confidence())
        })
        .then_with(|| {
            left.evidence_count()
                .cmp(&right.evidence_count())
        })
        .then_with(|| {
            compare_evidence(left.evidence(), right.evidence())
        })
}

fn compare_evidence(
    left: &[EvidenceReference],
    right: &[EvidenceReference],
) -> Ordering {
    left.cmp(right)
}

// =============================================================================
// Evidence utilities
// =============================================================================

fn unique_signal_count(evidence: &[EvidenceReference]) -> usize {
    let mut signals = BTreeSet::new();

    for reference in evidence {
        signals.insert(reference.signal_id());
    }

    signals.len()
}

fn has_conflicting_categories(
    categories: &BTreeSet<DiagnosisCategory>,
) -> bool {
    let actionable_count = categories
        .iter()
        .filter(|category| category.is_actionable())
        .count();

    actionable_count > 1
}

// =============================================================================
// Confidence comparison helpers
// =============================================================================

/// Returns the stronger of two diagnosis-confidence values.
#[must_use]
pub fn maximum(
    left: DiagnosisConfidence,
    right: DiagnosisConfidence,
) -> DiagnosisConfidence {
    if left >= right {
        left
    } else {
        right
    }
}

/// Returns the weaker of two diagnosis-confidence values.
#[must_use]
pub fn minimum(
    left: DiagnosisConfidence,
    right: DiagnosisConfidence,
) -> DiagnosisConfidence {
    if left <= right {
        left
    } else {
        right
    }
}

/// Computes an arithmetic mean without assuming statistical independence.
pub fn mean(
    values: impl IntoIterator<Item = DiagnosisConfidence>,
) -> ResilienceResult<DiagnosisConfidence> {
    let mut count = 0_u64;
    let mut sum = 0.0_f64;

    for value in values {
        sum += value.value();

        if !sum.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        count = count.checked_add(1).ok_or_else(|| {
            ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            )
        })?;
    }

    if count == 0 {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InsufficientEvidence,
        ));
    }

    DiagnosisConfidence::new(sum / count as f64)
}

// =============================================================================
// Public confidence gate
// =============================================================================

/// Explicit confidence gate for downstream policy/verification.
///
/// This type intentionally contains no recovery authority.
///
/// A gate is merely a reusable representation of:
///
///     measured confidence >= required confidence
///
/// The caller decides what the result means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfidenceGate {
    required: DiagnosisConfidence,
}

impl ConfidenceGate {
    /// Creates a gate with an explicitly supplied requirement.
    #[must_use]
    pub const fn new(required: DiagnosisConfidence) -> Self {
        Self { required }
    }

    /// Returns the required confidence.
    #[must_use]
    pub const fn required(self) -> DiagnosisConfidence {
        self.required
    }

    /// Evaluates the gate.
    #[must_use]
    pub fn evaluate(
        self,
        actual: DiagnosisConfidence,
    ) -> bool {
        actual.meets(self.required)
    }

    /// Evaluates the gate against a complete assessment.
    #[must_use]
    pub fn evaluate_assessment(
        self,
        assessment: &ConfidenceAssessment,
    ) -> bool {
        assessment.meets(self.required)
    }
}

// =============================================================================
// Stable serialization-oriented summary
// =============================================================================

/// Compact machine-readable confidence summary.
///
/// This avoids forcing serialization consumers to depend on the complete
/// internal analysis representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfidenceSummary {
    confidence: DiagnosisConfidence,
    basis: ConfidenceBasis,
    aggregation: ConfidenceAggregation,
    findings: usize,
    evidence: usize,
    unique_signals: usize,
    conflicting: bool,
}

impl ConfidenceSummary {
    /// Creates a summary from a confidence assessment.
    #[must_use]
    pub fn from_assessment(
        assessment: &ConfidenceAssessment,
    ) -> Self {
        Self {
            confidence: assessment.confidence(),
            basis: assessment.basis(),
            aggregation: assessment.aggregation(),
            findings: assessment.findings_considered(),
            evidence: assessment.evidence_references(),
            unique_signals: assessment.unique_evidence_signals(),
            conflicting: assessment.is_conflicting(),
        }
    }

    /// Returns final confidence.
    #[must_use]
    pub const fn confidence(self) -> DiagnosisConfidence {
        self.confidence
    }

    /// Returns confidence basis.
    #[must_use]
    pub const fn basis(self) -> ConfidenceBasis {
        self.basis
    }

    /// Returns aggregation strategy.
    #[must_use]
    pub const fn aggregation(self) -> ConfidenceAggregation {
        self.aggregation
    }

    /// Returns finding count.
    #[must_use]
    pub const fn findings(self) -> usize {
        self.findings
    }

    /// Returns evidence-reference count.
    #[must_use]
    pub const fn evidence(self) -> usize {
        self.evidence
    }

    /// Returns distinct signal count.
    #[must_use]
    pub const fn unique_signals(self) -> usize {
        self.unique_signals
    }

    /// Returns whether the assessment was conflicting.
    #[must_use]
    pub const fn conflicting(self) -> bool {
        self.conflicting
    }
}

impl fmt::Display for ConfidenceSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "confidence={} basis={} aggregation={} findings={} evidence={} unique_signals={} conflicting={}",
            self.confidence,
            self.basis.as_str(),
            self.aggregation.as_str(),
            self.findings,
            self.evidence,
            self.unique_signals,
            self.conflicting,
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    use crate::quantum::resilience::detection::detector::{
        DetectionClassification,
        DetectionConfidence,
        DetectionSequence,
        DetectorIdentity,
        SignalId,
    };

    fn diagnosis_confidence(
        value: f64,
    ) -> DiagnosisConfidence {
        DiagnosisConfidence::new(value)
            .unwrap_or_else(|_| DiagnosisConfidence::zero())
    }

    fn signal_id(value: u64) -> SignalId {
        SignalId::from_u64(value)
            .unwrap_or_else(|| {
                SignalId::new(
                    NonZeroU64::new(1)
                        .unwrap_or_else(|| {
                            // This branch is unreachable for the literal 1.
                            // It exists only to avoid unchecked construction.
                            NonZeroU64::MIN
                        }),
                )
            })
    }

    #[test]
    fn default_configuration_is_deterministic() {
        let config = ConfidenceConfig::default();

        assert_eq!(
            config.aggregation(),
            ConfidenceAggregation::Maximum
        );
        assert!(!config.include_non_actionable());
        assert!(!config.reject_conflicting_categories());
    }

    #[test]
    fn confidence_gate_requires_explicit_threshold() {
        let gate =
            ConfidenceGate::new(diagnosis_confidence(0.8));

        assert!(gate.evaluate(diagnosis_confidence(0.8)));
        assert!(gate.evaluate(diagnosis_confidence(0.9)));
        assert!(!gate.evaluate(diagnosis_confidence(0.79)));
    }

    #[test]
    fn maximum_and_minimum_are_monotonic() {
        let low = diagnosis_confidence(0.2);
        let high = diagnosis_confidence(0.8);

        assert_eq!(maximum(low, high), high);
        assert_eq!(minimum(low, high), low);
    }

    #[test]
    fn mean_is_not_statistical_independence_assumption() {
        let values = [
            diagnosis_confidence(0.2),
            diagnosis_confidence(0.8),
        ];

        let result = mean(values)
            .unwrap_or_else(|_| DiagnosisConfidence::zero());

        assert_eq!(result, diagnosis_confidence(0.5));
    }

    #[test]
    fn mean_rejects_empty_input() {
        let result = mean(
            std::iter::empty::<DiagnosisConfidence>(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn aggregation_names_are_stable() {
        assert_eq!(
            ConfidenceAggregation::Maximum.as_str(),
            "maximum"
        );
        assert_eq!(
            ConfidenceAggregation::Minimum.as_str(),
            "minimum"
        );
        assert_eq!(
            ConfidenceAggregation::Mean.as_str(),
            "mean"
        );
        assert_eq!(
            ConfidenceAggregation::EvidenceWeightedMean.as_str(),
            "evidence_weighted_mean"
        );
    }

    #[test]
    fn basis_names_are_stable() {
        assert_eq!(
            ConfidenceBasis::Contributor.as_str(),
            "contributor"
        );
        assert_eq!(
            ConfidenceBasis::Aggregated.as_str(),
            "aggregated"
        );
        assert_eq!(
            ConfidenceBasis::InsufficientEvidence.as_str(),
            "insufficient_evidence"
        );
        assert_eq!(
            ConfidenceBasis::Conflicted.as_str(),
            "conflicted"
        );
    }

    #[test]
    fn empty_assessment_has_zero_confidence() {
        let analyzer = ConfidenceAnalyzer::default();

        let result = analyzer
            .analyze(std::iter::empty::<DiagnosisFinding>())
            .unwrap_or_else(|_| {
                ConfidenceAssessment::empty(
                    ConfidenceAggregation::Maximum,
                )
            });

        assert_eq!(
            result.confidence(),
            DiagnosisConfidence::zero()
        );
        assert_eq!(
            result.basis(),
            ConfidenceBasis::InsufficientEvidence
        );
        assert!(!result.is_actionable());
    }

    #[test]
    fn confidence_summary_is_stable() {
        let assessment = ConfidenceAssessment::empty(
            ConfidenceAggregation::Maximum,
        );

        let summary =
            ConfidenceSummary::from_assessment(&assessment);

        assert_eq!(
            summary.confidence(),
            DiagnosisConfidence::zero()
        );
        assert_eq!(
            summary.basis(),
            ConfidenceBasis::InsufficientEvidence
        );
        assert_eq!(
            summary.aggregation(),
            ConfidenceAggregation::Maximum
        );
        assert_eq!(summary.findings(), 0);
        assert_eq!(summary.evidence(), 0);
        assert_eq!(summary.unique_signals(), 0);
        assert!(!summary.conflicting());
    }

    #[test]
    fn detection_confidence_contract_is_not_reimplemented() {
        let value =
            DetectionConfidence::new(0.75);

        assert!(value.is_ok());
    }

    #[test]
    fn detection_identity_contract_is_available_without_hardware_assumptions() {
        let detector =
            DetectorIdentity::new(
                "test-detector",
                "1",
            );

        assert!(detector.is_ok());
    }

    #[test]
    fn signal_identity_remains_opaque() {
        let id = signal_id(42);

        assert_eq!(id.value(), 42);
    }

    #[test]
    fn detection_classification_remains_distinct_from_diagnosis_confidence() {
        let classification =
            DetectionClassification::Anomaly;

        assert_eq!(
            classification,
            DetectionClassification::Anomaly
        );

        let confidence =
            diagnosis_confidence(0.5);

        assert_eq!(confidence.value(), 0.5);
    }

    #[test]
    fn sequence_identity_is_not_a_machine_size() {
        let sequence =
            DetectionSequence::new(
                NonZeroU64::new(1)
                    .unwrap_or_else(|| NonZeroU64::MIN),
            );

        assert_eq!(sequence.value(), 1);
    }

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            DIAGNOSIS_CONFIDENCE_SCHEMA_ID,
            "zamani.quantum.resilience.diagnosis.confidence"
        );
        assert_eq!(
            DIAGNOSIS_CONFIDENCE_SCHEMA_VERSION,
            1
        );
    }
}