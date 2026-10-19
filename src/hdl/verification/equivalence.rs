
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Equivalence Contracts
//!
//! Path: `src/hdl/verification/equivalence.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Define target-independent equivalence claims, provider assessments, and
//! deterministic assessment aggregation for HDL verification.
//!
//! This module is a verification boundary, not an equivalence solver. It
//! records which subjects are compared, the relation being claimed, the
//! reported result, and the evidence classification supplied by a provider.
//!
//! ## Authority and integration
//!
//! - `grammar/spec/hdl.md` defines portable HDL intent and ownership.
//! - `grammar/hdl/assertions.g4` and `grammar/hdl/verification.g4` define
//!   verification syntax; they do not define a proof algorithm.
//! - `src/frontend/semantic/domains/hdl.rs` owns `HdlId` and
//!   `HdlSourceLocation`.
//! - `src/hdl/contract.rs` owns behavioral contracts and contract outcomes.
//! - `src/hdl/verification/assertions.rs` owns assertion/property structure.
//! - `src/hdl/hdlir/` remains the exclusive future owner of HDLIR.
//!
//! The frontend adapter supplies stable HDL subject identities. A registered
//! equivalence provider evaluates a claim and returns an assessment. This
//! module validates claim/assessment relationships and summarizes results;
//! it does not independently authenticate provider evidence or prove claims.
//!
//! ## Does not own
//!
//! - Parsing, AST nodes, source maps, name resolution, or type checking.
//! - A second hardware design model or intermediate representation.
//! - Formal solvers, SAT/SMT integration, model checking, or simulation.
//! - Synthesis, lowering, scheduling, placement, routing, or target selection.
//! - Provider trust policy, proof-certificate validation, or proof soundness.
//!
//! ## Scalability and determinism
//!
//! No language-level limit is imposed on claims, subjects, or assessments.
//! Optional operational budgets are explicit and cause an incomplete result;
//! they do not change HDL semantics. Collections and identity maps are
//! dynamically sized. As with all software, actual execution is bounded by
//! available memory, time, and identifier representation.
//!
//! Results and diagnostics are ordered deterministically. Input ordering does
//! not determine the ordering of claim identities or diagnostics.
//!
//! ## Soundness warning
//!
//! An assessment is a provider-reported fact, not a proof checked here.
//! `Equivalent` must only be reported by a provider whose method establishes
//! the claimed relation for the declared scope and assumptions. A simulation
//! that merely finds no counterexample in tested traces is not, by itself,
//! a proof of unbounded equivalence.
//!
//! ## Freeze criteria
//!
//! Freeze this API after the frontend adapter and provider integration agree
//! on subject identity, relation scope, assumptions, provider trust, and
//! evidence validation. The file is complete only when crate-level checks
//! and the relevant integration tests pass.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::{HdlId, HdlSourceLocation};

/// The semantic relation that an equivalence provider is asked to establish.
///
/// These variants describe the claim's meaning, not the implementation
/// algorithm used to check it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EquivalenceRelation {
    /// Equal observable outputs for the same permitted inputs, under the
    /// claim's explicitly supplied environmental assumptions.
    Combinational,

    /// Equal observable behavior across the declared sequential execution
    /// scope, including the relevant initial-state and clock assumptions.
    Sequential,

    /// The candidate implementation preserves the specified reference
    /// behavior while permitting only the explicitly allowed refinements.
    Refinement,

    /// An extension-defined relation whose semantics are supplied by a
    /// registered provider.
    Extension,
}

/// The two HDL subjects participating in an equivalence claim.
///
/// Subject identities are references to canonical declarations or
/// compilation artifacts. This structure does not contain a design AST,
/// implementation graph, or IR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceSubjects {
    /// The reference subject whose behavior is being preserved.
    pub reference: HdlId,

    /// The candidate subject being compared with the reference.
    pub candidate: HdlId,
}

impl EquivalenceSubjects {
    /// Construct an ordered reference/candidate pair.
    ///
    /// A subject may be compared with itself. Such a claim still requires a
    /// provider assessment; this constructor does not infer a proof.
    #[must_use]
    pub fn new(reference: HdlId, candidate: HdlId) -> Self {
        Self {
            reference,
            candidate,
        }
    }
}

/// A source-level equivalence claim.
///
/// `id` identifies this claim within the caller-selected analysis scope.
/// The subjects' identities must be unique in their owning semantic
/// compilation context, as guaranteed by that context rather than this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceClaim {
    /// Stable identity of this claim in the supplied analysis scope.
    pub id: HdlId,

    /// Reference and candidate subjects.
    pub subjects: EquivalenceSubjects,

    /// Relation the provider is requested to establish.
    pub relation: EquivalenceRelation,

    /// Optional namespaced identifier for an extension-defined relation.
    ///
    /// Required when `relation` is `Extension`, and forbidden otherwise.
    pub extension_relation: Option<String>,

    /// Optional source location for the claim.
    pub location: Option<HdlSourceLocation>,

    /// Human-readable assumptions or scope notes retained for reporting.
    ///
    /// These notes are not parsed or proved by this module. The provider must
    /// receive the authoritative semantic assumptions through its own
    /// integration contract.
    pub scope_description: Option<String>,
}

impl EquivalenceClaim {
    /// Construct a claim after validating its local shape.
    ///
    /// This does not establish that either subject exists or that the
    /// requested relation is semantically meaningful for those subjects.
    pub fn new(
        id: HdlId,
        subjects: EquivalenceSubjects,
        relation: EquivalenceRelation,
        extension_relation: Option<String>,
        location: Option<HdlSourceLocation>,
        scope_description: Option<String>,
    ) -> Result<Self, EquivalenceInputError> {
        match (relation, extension_relation.as_deref()) {
            (EquivalenceRelation::Extension, None) => {
                return Err(EquivalenceInputError::MissingExtensionRelation);
            }
            (EquivalenceRelation::Extension, Some(value))
                if value.trim().is_empty() || value.trim() != value =>
            {
                return Err(EquivalenceInputError::InvalidExtensionRelation);
            }
            (EquivalenceRelation::Extension, Some(_)) => {}
            (_, Some(_)) => {
                return Err(EquivalenceInputError::UnexpectedExtensionRelation);
            }
            (_, None) => {}
        }

        Ok(Self {
            id,
            subjects,
            relation,
            extension_relation,
            location,
            scope_description,
        })
    }
}

/// The relation result reported by an equivalence provider.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EquivalenceOutcome {
    /// The provider reports that the requested relation was established.
    Equivalent,

    /// The provider reports a valid counterexample or other decisive evidence
    /// that the requested relation does not hold.
    NotEquivalent,

    /// The provider could not decide the claim.
    Unknown,

    /// The analysis did not complete, for example because it was cancelled
    /// or an operational budget was exhausted.
    Incomplete,

    /// No provider assessment has been performed.
    NotAssessed,
}

/// Classification of the evidence reported by a provider.
///
/// The classification is descriptive. It is not a certificate and does not
/// independently establish that the evidence is sound or trustworthy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EquivalenceEvidenceKind {
    /// A formal proof or checked proof certificate, subject to the provider's
    /// actual validation guarantees.
    FormalProof,

    /// An exhaustive analysis over a finite, explicitly defined state/input
    /// space. The provider must establish that the space is exhaustive.
    ExhaustiveAnalysis,

    /// A counterexample or execution trace that demonstrates a difference
    /// within the declared observation semantics.
    Counterexample,

    /// Testing or simulation evidence that is not necessarily exhaustive.
    BoundedTesting,

    /// Evidence from a registered external provider.
    Extension,

    /// No evidence was supplied.
    None,
}

/// Provider-reported assessment for one equivalence claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceAssessment {
    /// Claim identity to which this assessment applies.
    pub claim_id: HdlId,

    /// Provider-reported outcome.
    pub outcome: EquivalenceOutcome,

    /// Classification of the supplied evidence.
    pub evidence_kind: EquivalenceEvidenceKind,

    /// Optional stable provider identifier.
    pub provider: Option<String>,

    /// Optional provider-owned evidence reference, such as a certificate or
    /// counterexample artifact identifier. This is not fetched or validated
    /// by this module.
    pub evidence_reference: Option<String>,

    /// Optional source location associated with the assessment.
    pub location: Option<HdlSourceLocation>,
}

impl EquivalenceAssessment {
    /// Construct an assessment record.
    ///
    /// The constructor validates local metadata only. Provider registration,
    /// evidence integrity, claim-scope binding, and proof validation belong
    /// to the verification integration boundary.
    pub fn new(
        claim_id: HdlId,
        outcome: EquivalenceOutcome,
        evidence_kind: EquivalenceEvidenceKind,
        provider: Option<String>,
        evidence_reference: Option<String>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, EquivalenceInputError> {
        if provider.as_deref().is_some_and(|value| {
            value.trim().is_empty() || value.trim() != value
        }) {
            return Err(EquivalenceInputError::InvalidProvider);
        }

        if evidence_reference.as_deref().is_some_and(|value| {
            value.trim().is_empty() || value.trim() != value
        }) {
            return Err(EquivalenceInputError::InvalidEvidenceReference);
        }

        if evidence_kind == EquivalenceEvidenceKind::None
            && evidence_reference.is_some()
        {
            return Err(EquivalenceInputError::ReferenceWithoutEvidence);
        }

        Ok(Self {
            claim_id,
            outcome,
            evidence_kind,
            provider,
            evidence_reference,
            location,
        })
    }
}

/// Operational limits for equivalence assessment aggregation.
///
/// `None` means no explicit limit is applied at that boundary. These values
/// are not source-language or hardware-capacity limits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EquivalenceLimits {
    /// Maximum number of claims accepted for one analysis.
    pub max_claims: Option<usize>,

    /// Maximum number of assessments accepted for one analysis.
    pub max_assessments: Option<usize>,

    /// Maximum number of retained diagnostics. Omitted diagnostics are
    /// counted in [`EquivalenceSummary::omitted_diagnostics`].
    pub max_diagnostics: Option<usize>,
}

/// Stable machine-readable equivalence diagnostic categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum EquivalenceDiagnosticCode {
    /// The same claim identity was declared more than once.
    DuplicateClaim,

    /// An assessment refers to no claim in the current analysis.
    UnknownClaimAssessment,

    /// More than one assessment was supplied for one claim.
    DuplicateAssessment,

    /// A decisive outcome was supplied without an evidence classification.
    MissingEvidence,

    /// The outcome and evidence classification are inconsistent.
    InconsistentEvidence,

    /// No assessment was supplied for a claim.
    MissingAssessment,

    /// The configured claim or assessment budget was exceeded.
    BudgetExceeded,

    /// The input scope was empty; no non-vacuous equivalence conclusion exists.
    EmptyScope,

    /// The analysis did not complete.
    IncompleteAnalysis,
}

/// Severity of a diagnostic emitted by equivalence aggregation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EquivalenceDiagnosticSeverity {
    /// The input or assessment prevents a valid conclusion.
    Error,

    /// The result is incomplete or inconclusive.
    Warning,

    /// Informational context that does not itself invalidate a claim.
    Note,
}

/// A deterministic, machine-readable diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceDiagnostic {
    /// Stable diagnostic category.
    pub code: EquivalenceDiagnosticCode,

    /// Diagnostic severity.
    pub severity: EquivalenceDiagnosticSeverity,

    /// Related claim identity, when available.
    pub claim_id: Option<HdlId>,

    /// Human-readable explanation.
    pub message: String,
}

/// Aggregate status of the supplied equivalence analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EquivalenceStatus {
    /// Every claim in a non-empty, fully analyzed scope was accepted as
    /// equivalent under the provider assessments supplied to this module.
    ///
    /// This status does not independently authenticate those assessments.
    Equivalent,

    /// At least one claim has a decisive reported inequivalence.
    NotEquivalent,

    /// The input contains invalid or conflicting claim/assessment structure.
    Invalid,

    /// The scope is valid but at least one claim is unknown, unassessed, or
    /// otherwise not decidable from the supplied information.
    Inconclusive,

    /// The analysis could not process the entire requested scope.
    Incomplete,
}

/// Counts of claim and assessment outcomes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EquivalenceSummary {
    /// Number of unique claims accepted into the analysis.
    pub claims: usize,

    /// Claims with a decisive equivalent outcome and non-empty evidence kind.
    pub equivalent: usize,

    /// Claims with a decisive not-equivalent outcome and non-empty evidence.
    pub not_equivalent: usize,

    /// Claims assessed as unknown.
    pub unknown: usize,

    /// Claims whose provider analysis did not complete.
    pub incomplete: usize,

    /// Claims explicitly marked not assessed.
    pub not_assessed: usize,

    /// Claims with no assessment record.
    pub missing_assessment: usize,

    /// Number of assessments that could not be associated with a unique claim.
    pub rejected_assessments: usize,

    /// Number of diagnostics suppressed by the configured diagnostic budget.
    pub omitted_diagnostics: usize,
}

/// Complete result of aggregating one equivalence analysis scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceAnalysisResult {
    /// Aggregate conclusion.
    pub status: EquivalenceStatus,

    /// Whether every supplied claim and assessment was processed.
    pub complete: bool,

    /// Outcome counts.
    pub summary: EquivalenceSummary,

    /// Retained diagnostics in deterministic order.
    pub diagnostics: Vec<EquivalenceDiagnostic>,
}

/// Input construction or local metadata validation errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EquivalenceInputError {
    /// An extension relation was not named.
    MissingExtensionRelation,

    /// An extension relation identifier was empty or had surrounding
    /// whitespace.
    InvalidExtensionRelation,

    /// A built-in relation was supplied an extension identifier.
    UnexpectedExtensionRelation,

    /// A provider identifier was empty or had surrounding whitespace.
    InvalidProvider,

    /// An evidence reference was empty or had surrounding whitespace.
    InvalidEvidenceReference,

    /// An evidence reference was supplied while evidence was classified as
    /// absent.
    ReferenceWithoutEvidence,
}

impl fmt::Display for EquivalenceInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingExtensionRelation => {
                f.write_str("extension equivalence relation requires an identifier")
            }
            Self::InvalidExtensionRelation => {
                f.write_str("extension equivalence relation identifier is invalid")
            }
            Self::UnexpectedExtensionRelation => {
                f.write_str("built-in equivalence relation cannot carry an extension identifier")
            }
            Self::InvalidProvider => f.write_str("equivalence provider identifier is invalid"),
            Self::InvalidEvidenceReference => {
                f.write_str("equivalence evidence reference is invalid")
            }
            Self::ReferenceWithoutEvidence => {
                f.write_str("evidence reference cannot be supplied when evidence is absent")
            }
        }
    }
}

impl Error for EquivalenceInputError {}

/// Aggregate claim assessments deterministically.
///
/// This function does not execute a provider, fetch evidence, or validate a
/// formal proof. It checks identity relationships and aggregates the results
/// reported by the caller.
///
/// Duplicate claim identities are invalid. If an assessment identity is
/// duplicated, none of the assessments for that claim is trusted for
/// aggregation: choosing one by input order could hide a conflict.
///
/// A decisive outcome with `EquivalenceEvidenceKind::None` is rejected.
/// Other evidence classifications are recorded as provider assertions and
/// are not independently validated here.
///
/// An empty claim scope is inconclusive rather than vacuously equivalent.
#[must_use]
pub fn analyze_equivalence(
    claims: &[EquivalenceClaim],
    assessments: &[EquivalenceAssessment],
    limits: EquivalenceLimits,
) -> EquivalenceAnalysisResult {
    let mut summary = EquivalenceSummary::default();
    let mut diagnostics = Vec::new();
    let mut complete = true;
    let diagnostic_limit = limits.max_diagnostics;

    fn emit(
        diagnostics: &mut Vec<EquivalenceDiagnostic>,
        summary: &mut EquivalenceSummary,
        limit: Option<usize>,
        diagnostic: EquivalenceDiagnostic,
    ) {
        if limit.is_some_and(|maximum| diagnostics.len() >= maximum) {
            summary.omitted_diagnostics = summary.omitted_diagnostics.saturating_add(1);
        } else {
            diagnostics.push(diagnostic);
        }
    }

    if claims.is_empty() {
        emit(
            &mut diagnostics,
            &mut summary,
            diagnostic_limit,
            EquivalenceDiagnostic {
                code: EquivalenceDiagnosticCode::EmptyScope,
                severity: EquivalenceDiagnosticSeverity::Warning,
                claim_id: None,
                message: "no equivalence claims were supplied; no equivalence conclusion can be drawn".to_owned(),
            },
        );
        return EquivalenceAnalysisResult {
            status: EquivalenceStatus::Inconclusive,
            complete: true,
            summary,
            diagnostics,
        };
    }

    if limits.max_claims.is_some_and(|maximum| claims.len() > maximum)
        || limits
            .max_assessments
            .is_some_and(|maximum| assessments.len() > maximum)
    {
        complete = false;
        emit(
            &mut diagnostics,
            &mut summary,
            diagnostic_limit,
            EquivalenceDiagnostic {
                code: EquivalenceDiagnosticCode::BudgetExceeded,
                severity: EquivalenceDiagnosticSeverity::Error,
                claim_id: None,
                message: "equivalence analysis input exceeded a configured operational budget".to_owned(),
            },
        );
        emit(
            &mut diagnostics,
            &mut summary,
            diagnostic_limit,
            EquivalenceDiagnostic {
                code: EquivalenceDiagnosticCode::IncompleteAnalysis,
                severity: EquivalenceDiagnosticSeverity::Warning,
                claim_id: None,
                message: "equivalence analysis was not performed because an input budget was exceeded".to_owned(),
            },
        );
        return EquivalenceAnalysisResult {
            status: EquivalenceStatus::Incomplete,
            complete,
            summary,
            diagnostics,
        };
    }

    let mut claims_by_id: BTreeMap<HdlId, &EquivalenceClaim> = BTreeMap::new();
    let mut duplicate_claims = BTreeSet::new();

    for claim in claims {
        if claims_by_id.insert(claim.id.clone(), claim).is_some() {
            duplicate_claims.insert(claim.id.clone());
        }
    }

    for id in &duplicate_claims {
        emit(
            &mut diagnostics,
            &mut summary,
            diagnostic_limit,
            EquivalenceDiagnostic {
                code: EquivalenceDiagnosticCode::DuplicateClaim,
                severity: EquivalenceDiagnosticSeverity::Error,
                claim_id: Some(id.clone()),
                message: format!("equivalence claim identity `{id}` is duplicated"),
            },
        );
    }

    let mut assessments_by_id: BTreeMap<HdlId, Vec<&EquivalenceAssessment>> =
        BTreeMap::new();

    for assessment in assessments {
        if !claims_by_id.contains_key(&assessment.claim_id) {
            summary.rejected_assessments =
                summary.rejected_assessments.saturating_add(1);
            emit(
                &mut diagnostics,
                &mut summary,
                diagnostic_limit,
                EquivalenceDiagnostic {
                    code: EquivalenceDiagnosticCode::UnknownClaimAssessment,
                    severity: EquivalenceDiagnosticSeverity::Error,
                    claim_id: Some(assessment.claim_id.clone()),
                    message: format!(
                        "assessment refers to unknown equivalence claim `{}`",
                        assessment.claim_id
                    ),
                },
            );
            continue;
        }
        assessments_by_id
            .entry(assessment.claim_id.clone())
            .or_default()
            .push(assessment);
    }

    let mut invalid = !duplicate_claims.is_empty();
    let mut any_not_equivalent = false;
    let mut any_inconclusive = false;

    for (id, claim) in &claims_by_id {
        if duplicate_claims.contains(id) {
            invalid = true;
            continue;
        }

        let Some(records) = assessments_by_id.get(id) else {
            summary.missing_assessment = summary.missing_assessment.saturating_add(1);
            any_inconclusive = true;
            emit(
                &mut diagnostics,
                &mut summary,
                diagnostic_limit,
                EquivalenceDiagnostic {
                    code: EquivalenceDiagnosticCode::MissingAssessment,
                    severity: EquivalenceDiagnosticSeverity::Warning,
                    claim_id: Some(id.clone()),
                    message: format!("equivalence claim `{id}` has no assessment"),
                },
            );
            continue;
        };

        if records.len() != 1 {
            invalid = true;
            summary.rejected_assessments =
                summary.rejected_assessments.saturating_add(records.len());
            emit(
                &mut diagnostics,
                &mut summary,
                diagnostic_limit,
                EquivalenceDiagnostic {
                    code: EquivalenceDiagnosticCode::DuplicateAssessment,
                    severity: EquivalenceDiagnosticSeverity::Error,
                    claim_id: Some(id.clone()),
                    message: format!(
                        "equivalence claim `{id}` has {} assessments; exactly one is required",
                        records.len()
                    ),
                },
            );
            continue;
        }

        let assessment = records[0];
        let decisive = matches!(
            assessment.outcome,
            EquivalenceOutcome::Equivalent | EquivalenceOutcome::NotEquivalent
        );

        if decisive && assessment.evidence_kind == EquivalenceEvidenceKind::None {
            invalid = true;
            emit(
                &mut diagnostics,
                &mut summary,
                diagnostic_limit,
                EquivalenceDiagnostic {
                    code: EquivalenceDiagnosticCode::MissingEvidence,
                    severity: EquivalenceDiagnosticSeverity::Error,
                    claim_id: Some(id.clone()),
                    message: format!(
                        "decisive assessment for claim `{id}` has no evidence classification"
                    ),
                },
            );
            continue;
        }

        let evidence_consistent = match assessment.outcome {
            EquivalenceOutcome::Equivalent => !matches!(
                assessment.evidence_kind,
                EquivalenceEvidenceKind::Counterexample
            ),
            EquivalenceOutcome::NotEquivalent => {
                assessment.evidence_kind == EquivalenceEvidenceKind::Counterexample
                    || assessment.evidence_kind == EquivalenceEvidenceKind::FormalProof
                    || assessment.evidence_kind == EquivalenceEvidenceKind::ExhaustiveAnalysis
                    || assessment.evidence_kind == EquivalenceEvidenceKind::Extension
            }
            EquivalenceOutcome::Unknown
            | EquivalenceOutcome::Incomplete
            | EquivalenceOutcome::NotAssessed => true,
        };

        if !evidence_consistent {
            invalid = true;
            emit(
                &mut diagnostics,
                &mut summary,
                diagnostic_limit,
                EquivalenceDiagnostic {
                    code: EquivalenceDiagnosticCode::InconsistentEvidence,
                    severity: EquivalenceDiagnosticSeverity::Error,
                    claim_id: Some(id.clone()),
                    message: format!(
                        "assessment evidence classification conflicts with the reported outcome for claim `{id}`"
                    ),
                },
            );
            continue;
        }

        match assessment.outcome {
            EquivalenceOutcome::Equivalent => {
                summary.equivalent = summary.equivalent.saturating_add(1);
            }
            EquivalenceOutcome::NotEquivalent => {
                summary.not_equivalent = summary.not_equivalent.saturating_add(1);
                any_not_equivalent = true;
            }
            EquivalenceOutcome::Unknown => {
                summary.unknown = summary.unknown.saturating_add(1);
                any_inconclusive = true;
            }
            EquivalenceOutcome::Incomplete => {
                summary.incomplete = summary.incomplete.saturating_add(1);
                any_inconclusive = true;
                complete = false;
                emit(
                    &mut diagnostics,
                    &mut summary,
                    diagnostic_limit,
                    EquivalenceDiagnostic {
                        code: EquivalenceDiagnosticCode::IncompleteAnalysis,
                        severity: EquivalenceDiagnosticSeverity::Warning,
                        claim_id: Some(id.clone()),
                        message: format!("equivalence analysis for claim `{id}` did not complete"),
                    },
                );
            }
            EquivalenceOutcome::NotAssessed => {
                summary.not_assessed = summary.not_assessed.saturating_add(1);
                any_inconclusive = true;
            }
        }

        let _ = claim;
    }

    summary.claims = claims_by_id.len();

    diagnostics.sort_by(|left, right| {
        (
            left.claim_id.as_ref(),
            left.code,
            left.severity,
            left.message.as_str(),
        )
            .cmp(&(
                right.claim_id.as_ref(),
                right.code,
                right.severity,
                right.message.as_str(),
            ))
    });

    let status = if !complete {
        EquivalenceStatus::Incomplete
    } else if invalid {
        EquivalenceStatus::Invalid
    } else if any_not_equivalent {
        EquivalenceStatus::NotEquivalent
    } else if any_inconclusive {
        EquivalenceStatus::Inconclusive
    } else {
        EquivalenceStatus::Equivalent
    };

    EquivalenceAnalysisResult {
        status,
        complete,
        summary,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier must be valid")
    }

    fn claim(value: &str) -> EquivalenceClaim {
        EquivalenceClaim::new(
            id(value),
            EquivalenceSubjects::new(id("reference"), id("candidate")),
            EquivalenceRelation::Combinational,
            None,
            None,
            None,
        )
        .expect("test claim must be valid")
    }

    fn assessment(
        claim_id: &str,
        outcome: EquivalenceOutcome,
        evidence: EquivalenceEvidenceKind,
    ) -> EquivalenceAssessment {
        EquivalenceAssessment::new(
            id(claim_id),
            outcome,
            evidence,
            Some("test.provider".to_owned()),
            (evidence != EquivalenceEvidenceKind::None)
                .then(|| "test-evidence".to_owned()),
            None,
        )
        .expect("test assessment metadata must be valid")
    }

    #[test]
    fn empty_scope_is_inconclusive_not_equivalent() {
        let result = analyze_equivalence(&[], &[], EquivalenceLimits::default());
        assert_eq!(result.status, EquivalenceStatus::Inconclusive);
        assert!(result.complete);
        assert_eq!(result.diagnostics[0].code, EquivalenceDiagnosticCode::EmptyScope);
    }

    #[test]
    fn all_claims_with_decisive_equivalent_assessments_are_equivalent() {
        let claims = [claim("c1"), claim("c2")];
        let assessments = [
            assessment(
                "c1",
                EquivalenceOutcome::Equivalent,
                EquivalenceEvidenceKind::FormalProof,
            ),
            assessment(
                "c2",
                EquivalenceOutcome::Equivalent,
                EquivalenceEvidenceKind::ExhaustiveAnalysis,
            ),
        ];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Equivalent);
        assert_eq!(result.summary.equivalent, 2);
    }

    #[test]
    fn a_counterexample_produces_not_equivalent() {
        let claims = [claim("c1")];
        let assessments = [assessment(
            "c1",
            EquivalenceOutcome::NotEquivalent,
            EquivalenceEvidenceKind::Counterexample,
        )];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::NotEquivalent);
        assert_eq!(result.summary.not_equivalent, 1);
    }

    #[test]
    fn unknown_and_missing_assessments_never_pass() {
        let claims = [claim("c1"), claim("c2")];
        let assessments = [assessment(
            "c1",
            EquivalenceOutcome::Unknown,
            EquivalenceEvidenceKind::BoundedTesting,
        )];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Inconclusive);
        assert_eq!(result.summary.unknown, 1);
        assert_eq!(result.summary.missing_assessment, 1);
    }

    #[test]
    fn decisive_outcome_without_evidence_is_invalid() {
        let claims = [claim("c1")];
        let assessments = [assessment(
            "c1",
            EquivalenceOutcome::Equivalent,
            EquivalenceEvidenceKind::None,
        )];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Invalid);
    }

    #[test]
    fn evidence_that_contradicts_outcome_is_invalid() {
        let claims = [claim("c1")];
        let assessments = [assessment(
            "c1",
            EquivalenceOutcome::Equivalent,
            EquivalenceEvidenceKind::Counterexample,
        )];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Invalid);
    }

    #[test]
    fn duplicate_claims_are_invalid() {
        let claims = [claim("same"), claim("same")];
        let result = analyze_equivalence(
            &claims,
            &[],
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == EquivalenceDiagnosticCode::DuplicateClaim
        }));
    }

    #[test]
    fn duplicate_assessments_are_not_resolved_by_input_order() {
        let claims = [claim("c1")];
        let assessments = [
            assessment(
                "c1",
                EquivalenceOutcome::Equivalent,
                EquivalenceEvidenceKind::FormalProof,
            ),
            assessment(
                "c1",
                EquivalenceOutcome::NotEquivalent,
                EquivalenceEvidenceKind::Counterexample,
            ),
        ];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Invalid);
        assert_eq!(result.summary.equivalent, 0);
        assert_eq!(result.summary.not_equivalent, 0);
    }

    #[test]
    fn unknown_claim_assessment_is_reported() {
        let claims = [claim("known")];
        let assessments = [assessment(
            "orphan",
            EquivalenceOutcome::Equivalent,
            EquivalenceEvidenceKind::FormalProof,
        )];
        let result = analyze_equivalence(
            &claims,
            &assessments,
            EquivalenceLimits::default(),
        );
        assert_eq!(result.status, EquivalenceStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == EquivalenceDiagnosticCode::UnknownClaimAssessment
        }));
    }

    #[test]
    fn operational_budget_exhaustion_is_incomplete() {
        let claims = [claim("c1")];
        let result = analyze_equivalence(
            &claims,
            &[],
            EquivalenceLimits {
                max_claims: Some(0),
                ..EquivalenceLimits::default()
            },
        );
        assert_eq!(result.status, EquivalenceStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn diagnostic_budget_bounds_retained_diagnostics() {
        let claims = [claim("same"), claim("same")];
        let result = analyze_equivalence(
            &claims,
            &[],
            EquivalenceLimits {
                max_diagnostics: Some(0),
                ..EquivalenceLimits::default()
            },
        );
        assert!(result.diagnostics.is_empty());
        assert!(result.summary.omitted_diagnostics > 0);
    }

    #[test]
    fn diagnostics_are_stable_under_assessment_reordering() {
        let claims = [claim("b"), claim("a")];
        let first = [
            assessment(
                "a",
                EquivalenceOutcome::Unknown,
                EquivalenceEvidenceKind::BoundedTesting,
            ),
            assessment(
                "b",
                EquivalenceOutcome::NotAssessed,
                EquivalenceEvidenceKind::None,
            ),
        ];
        let second = [first[1].clone(), first[0].clone()];
        let left = analyze_equivalence(
            &claims,
            &first,
            EquivalenceLimits::default(),
        );
        let right = analyze_equivalence(
            &claims,
            &second,
            EquivalenceLimits::default(),
        );
        assert_eq!(left, right);
    }

    #[test]
    fn extension_relation_requires_a_namespaced_identifier() {
        assert_eq!(
            EquivalenceClaim::new(
                id("extension"),
                EquivalenceSubjects::new(id("a"), id("b")),
                EquivalenceRelation::Extension,
                None,
                None,
                None,
            ),
            Err(EquivalenceInputError::MissingExtensionRelation)
        );
    }
}
