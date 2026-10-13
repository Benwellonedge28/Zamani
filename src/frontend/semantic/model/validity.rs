#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Model — Validity Assessment
//!
//! Path: `src/frontend/semantic/model/validity.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Proposed implementation; integration and workspace verification
//! are required before this module can be marked frozen.
//!
//! ## PURPOSE
//!
//! Assess whether the available semantic-analysis evidence supports a
//! program's validity, invalidity, or unresolved status.
//!
//! This module is the evidence-evaluation layer. It does not introduce a
//! second validity enum: `program.rs` remains the sole owner of
//! `SemanticValidity` and `AnalysisCompletion`.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/expressions/provenance.g4`
//! - `grammar/quantum/provenance.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/program.rs`
//!
//! The language specifications define semantic meaning. The canonical AST
//! defines source structure. `program.rs` owns semantic-result lifecycle
//! transitions. This module evaluates evidence supplied by the analysis
//! pipeline and does not replace those authorities.
//!
//! ## OWNS
//!
//! - The input record describing validity-relevant analysis evidence.
//! - Deterministic assessment of that evidence.
//! - Explanations for why validity could not be established.
//! - A safe predicate for deciding whether the assessment supports
//!   valid-only downstream processing.
//!
//! ## DOES_NOT_OWN
//!
//! - `SemanticValidity` or `AnalysisCompletion` declarations.
//! - Semantic diagnostics or diagnostic rendering.
//! - Name resolution, type checking, effect checking, or domain validation.
//! - Obligation definitions or the evidence needed to discharge them.
//! - AST nodes, source spans, semantic facts, or provenance records.
//! - Target discovery, resource feasibility, hardware allocation, or runtime
//!   execution.
//! - Classical IR, `quantum::ir`, HDL IR, or any other intermediate
//!   representation.
//!
//! ## INTEGRATION CONTRACT
//!
//! `ValidityEvidence` is assembled by the semantic pipeline after collecting
//! results from the required analysis passes. The caller must count only
//! confirmed semantic errors as semantic errors. Warnings and target-specific
//! feasibility failures must not be counted as source-language semantic
//! errors merely because they are diagnostics.
//!
//! `ValidityAssessment::evaluate` produces the existing
//! `program::SemanticValidity`. The caller must apply that result through
//! `SemanticProgram::finalize_validity`; it must not directly mutate private
//! model state or bypass the program lifecycle.
//!
//! `program.rs` remains responsible for enforcing completion and pending
//! obligation invariants. This module independently checks the evidence
//! supplied by the caller, providing a second, explicit guard against
//! incorrectly certifying an incomplete analysis.
//!
//! ## VALIDITY RULES
//!
//! - A confirmed semantic error is sufficient to classify a program as
//!   invalid, even if some other checks did not finish.
//! - Without a confirmed semantic error, incomplete, aborted, or failed
//!   analysis cannot establish validity.
//! - Without a confirmed semantic error, an internal analysis failure or an
//!   incomplete required-check set cannot establish validity.
//! - A completed analysis with unresolved facts or pending obligations is
//!   unresolved, not valid.
//! - Validity is established only when required checks are complete, analysis
//!   completed reliably, no semantic errors were found, no unresolved facts
//!   remain, and no obligations remain pending.
//! - Target feasibility is not source-language validity.
//!
//! ## SCALABILITY AND DETERMINISM
//!
//! This module introduces no hardware-capacity ceilings and no domain-count
//! limits. Counts describe evidence already produced by the caller; they do
//! not constrain what a Zamani program may express.
//!
//! Assessment work is constant with respect to the numeric evidence counts.
//! The reason list has a fixed number of categories and is emitted in a stable
//! order. It does not allocate one entry per error, fact, or obligation.
//!
//! No finite implementation can process literally infinite input. Actual
//! analysis is bounded by available resources and explicit operational
//! policies. A budget-exhausted analysis must report incomplete or failed
//! completion rather than claim validity.
//!
//! ## ERROR HANDLING
//!
//! Assessment is a total, deterministic operation over a `ValidityEvidence`
//! value. It does not panic, perform I/O, inspect the environment, or access
//! hardware.
//!
//! Evidence counts are caller-supplied summaries. This module cannot prove
//! that a caller counted all errors or ran all required checks. The pipeline
//! must construct the evidence from authoritative pass results.
//!
//! ## FREEZE CRITERIA
//!
//! - `model/mod.rs` declares and exports this module.
//! - The semantic pipeline builds evidence from actual pass results.
//! - Finalization goes through `SemanticProgram::finalize_validity`.
//! - Unit tests and integration tests pass.
//! - Formatting and workspace checks pass on the supported Rust toolchain.

use core::fmt;

use super::program::{AnalysisCompletion, SemanticValidity};

/// Evidence used to assess the semantic validity of one analysis result.
///
/// This is a summary of analysis evidence, not a copy of the semantic program.
/// Its fields are private so callers use the documented constructor and
/// builder methods. Counts are descriptive and impose no language-level
/// limits on the program being analyzed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidityEvidence {
    completion: AnalysisCompletion,
    required_checks_complete: bool,
    semantic_error_count: usize,
    unresolved_fact_count: usize,
    pending_obligation_count: usize,
    internal_failure_count: usize,
}

impl ValidityEvidence {
    /// Create an evidence record for the supplied analysis completion state.
    ///
    /// Required checks default to incomplete. This fail-closed default means
    /// callers must explicitly affirm that all required checks completed
    /// before an error-free result can be assessed as valid.
    #[must_use]
    pub const fn new(completion: AnalysisCompletion) -> Self {
        Self {
            completion,
            required_checks_complete: false,
            semantic_error_count: 0,
            unresolved_fact_count: 0,
            pending_obligation_count: 0,
            internal_failure_count: 0,
        }
    }

    /// Set whether every required semantic check completed.
    ///
    /// Set this to `true` only after the pipeline has verified that all
    /// mandatory passes for the applicable language version and enabled
    /// features ran to completion. A pass skipped because of a budget,
    /// unsupported construct, cancellation, or internal failure must not be
    /// counted as completed merely because the pipeline reached its end.
    #[must_use]
    pub const fn with_required_checks_complete(
        mut self,
        complete: bool,
    ) -> Self {
        self.required_checks_complete = complete;
        self
    }

    /// Set the number of confirmed source-language semantic errors.
    ///
    /// Do not include warnings, informational diagnostics, or target-only
    /// feasibility failures unless the normative language rules explicitly
    /// classify them as semantic errors.
    #[must_use]
    pub const fn with_semantic_error_count(
        mut self,
        count: usize,
    ) -> Self {
        self.semantic_error_count = count;
        self
    }

    /// Set the number of unresolved semantic facts.
    ///
    /// Examples may include unresolved names, unresolved types, or other
    /// facts that the required semantic rules say must be resolved before
    /// validity can be established. The caller owns the classification.
    #[must_use]
    pub const fn with_unresolved_fact_count(
        mut self,
        count: usize,
    ) -> Self {
        self.unresolved_fact_count = count;
        self
    }

    /// Set the number of obligations that remain pending.
    ///
    /// The value should agree with the authoritative obligation store in the
    /// semantic program. A non-zero count prevents a valid assessment.
    #[must_use]
    pub const fn with_pending_obligation_count(
        mut self,
        count: usize,
    ) -> Self {
        self.pending_obligation_count = count;
        self
    }

    /// Set the number of internal failures affecting result reliability.
    ///
    /// Examples include a failed analysis service, an unrecoverable pass
    /// failure, or a failure that makes the completeness of evidence
    /// uncertain. Ordinary source-language errors are counted separately.
    #[must_use]
    pub const fn with_internal_failure_count(
        mut self,
        count: usize,
    ) -> Self {
        self.internal_failure_count = count;
        self
    }

    /// Return the recorded analysis completion state.
    #[must_use]
    pub const fn completion(self) -> AnalysisCompletion {
        self.completion
    }

    /// Return whether all required semantic checks are reported complete.
    #[must_use]
    pub const fn required_checks_complete(self) -> bool {
        self.required_checks_complete
    }

    /// Return the number of confirmed semantic errors.
    #[must_use]
    pub const fn semantic_error_count(self) -> usize {
        self.semantic_error_count
    }

    /// Return the number of unresolved semantic facts.
    #[must_use]
    pub const fn unresolved_fact_count(self) -> usize {
        self.unresolved_fact_count
    }

    /// Return the number of pending obligations.
    #[must_use]
    pub const fn pending_obligation_count(self) -> usize {
        self.pending_obligation_count
    }

    /// Return the number of internal failures affecting reliability.
    #[must_use]
    pub const fn internal_failure_count(self) -> usize {
        self.internal_failure_count
    }
}

impl Default for ValidityEvidence {
    /// Create conservative evidence with incomplete analysis.
    ///
    /// The default cannot certify validity.
    fn default() -> Self {
        Self::new(AnalysisCompletion::Incomplete)
    }
}

/// A reason that limits or determines a validity assessment.
///
/// Reasons are summary categories, not one entry per diagnostic or obligation.
/// Their order in [`ValidityAssessment::reasons`] is deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ValidityReason {
    /// One or more confirmed semantic errors were found.
    SemanticErrors {
        /// Number of confirmed semantic errors.
        count: usize,
    },

    /// Analysis did not reach the complete state.
    AnalysisNotComplete {
        /// Actual completion state.
        completion: AnalysisCompletion,
    },

    /// At least one required semantic check did not complete.
    RequiredChecksIncomplete,

    /// One or more semantic facts remain unresolved.
    UnresolvedFacts {
        /// Number of unresolved facts.
        count: usize,
    },

    /// One or more obligations remain pending.
    PendingObligations {
        /// Number of pending obligations.
        count: usize,
    },

    /// An internal failure makes the analysis result unreliable.
    InternalFailures {
        /// Number of internal failures reported by the caller.
        count: usize,
    },
}

impl fmt::Display for ValidityReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SemanticErrors { count } => {
                write!(formatter, "{count} semantic error(s) were found")
            }
            Self::AnalysisNotComplete { completion } => {
                write!(formatter, "analysis completion is {completion}")
            }
            Self::RequiredChecksIncomplete => {
                formatter.write_str("one or more required semantic checks did not complete")
            }
            Self::UnresolvedFacts { count } => {
                write!(formatter, "{count} semantic fact(s) remain unresolved")
            }
            Self::PendingObligations { count } => {
                write!(formatter, "{count} semantic obligation(s) remain pending")
            }
            Self::InternalFailures { count } => {
                write!(formatter, "{count} internal failure(s) affect analysis reliability")
            }
        }
    }
}

/// The result of evaluating validity-relevant evidence.
///
/// The `validity` field uses the canonical enum owned by `program.rs`.
/// This assessment does not mutate a `SemanticProgram`; the caller must apply
/// the result through the program's checked finalization API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidityAssessment {
    validity: SemanticValidity,
    reasons: Vec<ValidityReason>,
}

impl ValidityAssessment {
    /// Evaluate the supplied evidence using conservative validity rules.
    ///
    /// A confirmed semantic error is enough to establish invalidity, even if
    /// another pass did not finish. In the absence of a confirmed error,
    /// incomplete or unreliable analysis remains unknown. Completed and
    /// reliable analysis with unresolved facts or obligations is unresolved.
    /// Only complete, reliable, error-free analysis with no unresolved facts
    /// or obligations is assessed as valid.
    #[must_use]
    pub fn evaluate(evidence: &ValidityEvidence) -> Self {
        let mut reasons = Vec::new();

        if evidence.semantic_error_count > 0 {
            reasons.push(ValidityReason::SemanticErrors {
                count: evidence.semantic_error_count,
            });
        }

        if !evidence.completion.is_complete() {
            reasons.push(ValidityReason::AnalysisNotComplete {
                completion: evidence.completion,
            });
        }

        if !evidence.required_checks_complete {
            reasons.push(ValidityReason::RequiredChecksIncomplete);
        }

        if evidence.unresolved_fact_count > 0 {
            reasons.push(ValidityReason::UnresolvedFacts {
                count: evidence.unresolved_fact_count,
            });
        }

        if evidence.pending_obligation_count > 0 {
            reasons.push(ValidityReason::PendingObligations {
                count: evidence.pending_obligation_count,
            });
        }

        if evidence.internal_failure_count > 0 {
            reasons.push(ValidityReason::InternalFailures {
                count: evidence.internal_failure_count,
            });
        }

        let validity = if evidence.semantic_error_count > 0 {
            SemanticValidity::Invalid
        } else if !evidence.completion.is_complete()
            || !evidence.required_checks_complete
            || evidence.internal_failure_count > 0
        {
            SemanticValidity::Unknown
        } else if evidence.unresolved_fact_count > 0
            || evidence.pending_obligation_count > 0
        {
            SemanticValidity::Unresolved
        } else {
            SemanticValidity::Valid
        };

        Self { validity, reasons }
    }

    /// Return the canonical semantic validity state.
    #[must_use]
    pub const fn validity(&self) -> SemanticValidity {
        self.validity
    }

    /// Return the stable, ordered reasons associated with this assessment.
    #[must_use]
    pub fn reasons(&self) -> &[ValidityReason] {
        &self.reasons
    }

    /// Return whether this assessment establishes semantic validity.
    ///
    /// This is an assessment of supplied evidence, not a replacement for
    /// `SemanticProgram::is_valid`, which also checks the program's actual
    /// completion and validity lifecycle state.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.validity.is_valid()
    }

    /// Return whether the assessment permits a caller to request valid
    /// finalization from the semantic program model.
    ///
    /// The program model remains the final authority and can reject the
    /// request if its actual state or obligation store disagrees with the
    /// evidence supplied here.
    #[must_use]
    pub const fn permits_valid_finalization(&self) -> bool {
        self.validity.is_valid()
    }

    /// Return whether any reason was recorded.
    #[must_use]
    pub fn has_reasons(&self) -> bool {
        !self.reasons.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_evidence() -> ValidityEvidence {
        ValidityEvidence::new(AnalysisCompletion::Complete)
            .with_required_checks_complete(true)
    }

    #[test]
    fn default_evidence_cannot_establish_validity() {
        let assessment = ValidityAssessment::evaluate(&ValidityEvidence::default());

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(!assessment.is_valid());
        assert!(!assessment.permits_valid_finalization());
        assert!(assessment.has_reasons());
    }

    #[test]
    fn complete_error_free_analysis_is_valid() {
        let assessment = ValidityAssessment::evaluate(&complete_evidence());

        assert_eq!(assessment.validity(), SemanticValidity::Valid);
        assert!(assessment.is_valid());
        assert!(assessment.permits_valid_finalization());
        assert!(!assessment.has_reasons());
    }

    #[test]
    fn confirmed_semantic_error_establishes_invalidity() {
        let evidence = complete_evidence().with_semantic_error_count(1);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Invalid);
        assert!(!assessment.permits_valid_finalization());
        assert!(assessment.reasons().iter().any(|reason| {
            matches!(reason, ValidityReason::SemanticErrors { count: 1 })
        }));
    }

    #[test]
    fn a_confirmed_error_remains_invalid_when_analysis_is_incomplete() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Incomplete)
            .with_semantic_error_count(2);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Invalid);
        assert!(assessment.reasons().iter().any(|reason| {
            matches!(
                reason,
                ValidityReason::AnalysisNotComplete {
                    completion: AnalysisCompletion::Incomplete
                }
            )
        }));
    }

    #[test]
    fn incomplete_analysis_without_known_errors_is_unknown() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Incomplete)
            .with_required_checks_complete(false);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(!assessment.is_valid());
    }

    #[test]
    fn aborted_analysis_cannot_be_assessed_as_valid() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Aborted)
            .with_required_checks_complete(true);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(!assessment.permits_valid_finalization());
    }

    #[test]
    fn failed_analysis_cannot_be_assessed_as_valid() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Failed)
            .with_required_checks_complete(true);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(!assessment.permits_valid_finalization());
    }

    #[test]
    fn missing_required_checks_prevent_validity() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Complete);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(assessment.reasons().contains(
            &ValidityReason::RequiredChecksIncomplete
        ));
    }

    #[test]
    fn unresolved_facts_prevent_validity() {
        let evidence = complete_evidence().with_unresolved_fact_count(3);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unresolved);
        assert!(!assessment.is_valid());
        assert!(assessment.reasons().iter().any(|reason| {
            matches!(reason, ValidityReason::UnresolvedFacts { count: 3 })
        }));
    }

    #[test]
    fn pending_obligations_prevent_validity() {
        let evidence = complete_evidence().with_pending_obligation_count(2);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unresolved);
        assert!(assessment.reasons().iter().any(|reason| {
            matches!(reason, ValidityReason::PendingObligations { count: 2 })
        }));
    }

    #[test]
    fn internal_failure_prevents_validity() {
        let evidence = complete_evidence().with_internal_failure_count(1);
        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Unknown);
        assert!(assessment.reasons().iter().any(|reason| {
            matches!(reason, ValidityReason::InternalFailures { count: 1 })
        }));
    }

    #[test]
    fn all_blockers_are_reported_in_a_stable_order() {
        let evidence = ValidityEvidence::new(AnalysisCompletion::Failed)
            .with_required_checks_complete(false)
            .with_semantic_error_count(4)
            .with_unresolved_fact_count(5)
            .with_pending_obligation_count(6)
            .with_internal_failure_count(7);

        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Invalid);
        assert_eq!(
            assessment.reasons(),
            &[
                ValidityReason::SemanticErrors { count: 4 },
                ValidityReason::AnalysisNotComplete {
                    completion: AnalysisCompletion::Failed
                },
                ValidityReason::RequiredChecksIncomplete,
                ValidityReason::UnresolvedFacts { count: 5 },
                ValidityReason::PendingObligations { count: 6 },
                ValidityReason::InternalFailures { count: 7 },
            ]
        );
    }

    #[test]
    fn target_feasibility_is_not_a_validity_input() {
        // Target capability and resource feasibility are intentionally absent
        // from ValidityEvidence. They belong to target admission/planning.
        let assessment = ValidityAssessment::evaluate(&complete_evidence());

        assert_eq!(assessment.validity(), SemanticValidity::Valid);
    }

    #[test]
    fn assessment_does_not_mutate_its_evidence() {
        let evidence = complete_evidence().with_pending_obligation_count(1);
        let before = evidence;
        let _assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(evidence, before);
    }

    #[test]
    fn large_evidence_counts_do_not_change_assessment_cost_by_iteration() {
        let evidence = complete_evidence()
            .with_semantic_error_count(usize::MAX)
            .with_unresolved_fact_count(usize::MAX)
            .with_pending_obligation_count(usize::MAX)
            .with_internal_failure_count(usize::MAX);

        let assessment = ValidityAssessment::evaluate(&evidence);

        assert_eq!(assessment.validity(), SemanticValidity::Invalid);
        assert_eq!(assessment.reasons().len(), 5);
    }
}