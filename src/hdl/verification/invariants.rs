
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Invariant Verification
//!
//! Path: `src/hdl/verification/invariants.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: HDL invariant-validation and assessment-aggregation contract.
//!
//! ## Purpose
//!
//! Validate the structure and identity relationships of HDL invariants,
//! associate verification assessments with their canonical contracts, and
//! summarize the evidence available for a collection of invariants.
//!
//! This module is deliberately an integration layer over `crate::hdl::contract`
//! rather than a competing contract model. It does not parse source, construct
//! expressions, execute assertions, or prove properties.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/assertions.g4`
//! - `grammar/hdl/verification.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/contract.rs`
//! - `src/hdl/verification/assertions.rs`
//!
//! The grammar owns syntax. The canonical AST owns expressions and source
//! structure. The semantic model owns semantic identities. `contract.rs`
//! owns HDL contract declarations and individual assessment representations.
//! This module owns invariant-specific structural checks and aggregation.
//!
//! ## Owns
//!
//! - Invariant-specific validation diagnostics and result statuses.
//! - Configurable operational budgets for invariant analysis.
//! - Duplicate, missing, unexpected, and inconsistent assessment detection.
//! - Deterministic aggregation of canonical contract assessments.
//!
//! ## Does not own
//!
//! - A second AST, expression representation, contract ID, or source-span type.
//! - General name resolution, type checking, or expression evaluation.
//! - A theorem prover, SAT/SMT solver, model checker, or simulation engine.
//! - Verification-provider authentication or proof-certificate checking.
//! - Target selection, hardware allocation, synthesis, or physical realization.
//! - HDLIR, IR construction, IR verification, or IR lowering.
//!
//! ## Soundness and result interpretation
//!
//! A structurally valid invariant is not necessarily true. A `Satisfied`
//! assessment is accepted as a provider-reported result, not independently
//! proved by this module. Provider trust, evidence soundness, assumptions,
//! scope, and proof validation remain the responsibility of the verification
//! integration and its configured trust policy.
//!
//! Missing, unknown, unresolved, unassessed, or incomplete evidence is never
//! promoted to satisfaction. An incomplete analysis cannot return `Verified`.
//!
//! ## Scalability
//!
//! There are no language-level maximums for invariant declarations, assessment
//! records, signals, modules, hardware resources, or design size. All
//! collections are dynamically sized. Callers may impose configurable
//! operational budgets for declarations, assessments, and diagnostics.
//!
//! These budgets are compiler operational policies, not source-language
//! restrictions. Exhaustion is reported explicitly. Actual processing remains
//! subject to available memory, execution time, and the host representation.
//!
//! ## Integration
//!
//! 1. The AST/semantic pipeline constructs canonical `HdlContract` values.
//! 2. Invariant declarations use `HdlContractKind::Invariant`.
//! 3. A trusted verification provider returns `HdlContractAssessment` values.
//! 4. The caller supplies those declarations and assessments to
//!    `analyze_invariants`.
//! 5. The result distinguishes structural invalidity, reported violations,
//!    inconclusive evidence, completed verification, and incomplete analysis.
//!
//! No assessment is synthesized from an invariant declaration.
//!
//! ## Freeze criteria
//!
//! - All public contract types reuse `crate::hdl::contract`.
//! - Assessment identities are matched without lossy conversions.
//! - Duplicate and orphaned assessments cannot be silently accepted.
//! - Unknown and incomplete evidence cannot produce a verified result.
//! - Budget exhaustion is distinguishable from a completed analysis.
//! - Diagnostics and aggregation are deterministic.
//! - Tests and repository integration checks pass on Rust 1.97+.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::model::ContractId;
use crate::hdl::contract::{
    HdlContract, HdlContractAssessment, HdlContractEvidenceKind, HdlContractKind,
    HdlContractOutcome,
};

// -----------------------------------------------------------------------------
// Operational configuration
// -----------------------------------------------------------------------------

/// Configurable operational budgets for invariant analysis.
///
/// These limits are not language-level or hardware-capacity limits. `None`
/// means that the corresponding budget is not imposed by this configuration.
/// Host resource exhaustion can still prevent processing.
///
/// A budget of `Some(0)` is valid configuration, but any non-empty input for
/// the corresponding stage will produce an incomplete result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InvariantLimits {
    /// Maximum number of contract declarations admitted to one analysis.
    pub max_invariants: Option<usize>,

    /// Maximum number of assessment records admitted to one analysis.
    pub max_assessments: Option<usize>,

    /// Maximum number of retained ordinary diagnostics.
    ///
    /// If exceeded, diagnostics are truncated and an additional
    /// `BudgetExceeded` diagnostic is appended. Consequently, the final
    /// result can contain one diagnostic beyond this retention budget.
    pub max_diagnostics: Option<usize>,
}

// -----------------------------------------------------------------------------
// Diagnostics
// -----------------------------------------------------------------------------

/// Stable machine-readable code for an invariant-analysis diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum InvariantDiagnosticCode {
    /// An invariant has an empty or whitespace-only name.
    InvalidInvariantName,

    /// Two invariant declarations share a canonical identity.
    DuplicateInvariantIdentity,

    /// An assessment references a contract not supplied to this analysis.
    UnknownContractAssessment,

    /// An assessment refers to a contract that is not an invariant.
    AssessmentForNonInvariant,

    /// Multiple assessments were supplied for the same invariant.
    DuplicateAssessment,

    /// A satisfied or violated outcome lacks an evidence classification.
    MissingEvidence,

    /// An invariant has no assessment record.
    MissingAssessment,

    /// A configured operational budget prevented complete analysis.
    BudgetExceeded,

    /// The analysis could not complete all required checks.
    IncompleteAnalysis,
}

/// Severity of an invariant-analysis diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InvariantDiagnosticSeverity {
    /// A confirmed structural or input-contract violation.
    Error,

    /// Missing evidence, incomplete work, or an inconclusive condition.
    Warning,

    /// Informational diagnostic.
    Note,
}

/// Structured diagnostic produced by invariant analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantDiagnostic {
    /// Stable machine-readable diagnostic code.
    pub code: InvariantDiagnosticCode,

    /// Diagnostic severity.
    pub severity: InvariantDiagnosticSeverity,

    /// Canonical contract identity, when the diagnostic concerns one contract.
    pub contract_id: Option<ContractId>,

    /// Human-readable diagnostic message.
    pub message: String,
}

impl InvariantDiagnostic {
    fn error(
        code: InvariantDiagnosticCode,
        contract_id: Option<ContractId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity: InvariantDiagnosticSeverity::Error,
            contract_id,
            message: message.into(),
        }
    }

    fn warning(
        code: InvariantDiagnosticCode,
        contract_id: Option<ContractId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity: InvariantDiagnosticSeverity::Warning,
            contract_id,
            message: message.into(),
        }
    }
}

// -----------------------------------------------------------------------------
// Results
// -----------------------------------------------------------------------------

/// Overall result of invariant validation and assessment aggregation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvariantAnalysisStatus {
    /// All supplied invariant declarations are structurally valid and every
    /// invariant has a provider-reported `Satisfied` assessment with a
    /// non-empty evidence classification.
    ///
    /// This status does not independently validate a proof or authenticate
    /// the provider.
    Verified,

    /// At least one provider-reported invariant violation was found, and the
    /// analysis completed without a higher-priority incomplete condition.
    Violated,

    /// Confirmed structural or assessment-input errors were found.
    Invalid,

    /// The analysis completed, but one or more invariants are missing
    /// assessment evidence or have inconclusive outcomes.
    Inconclusive,

    /// A required analysis stage could not finish because of an operational
    /// budget or another incomplete condition.
    Incomplete,
}

/// Counts of assessment outcomes.
///
/// Each count describes supplied assessment records associated with unique
/// invariant declarations. Missing assessments are counted separately.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InvariantAssessmentSummary {
    /// Number of invariant assessments reporting satisfaction.
    pub satisfied: usize,

    /// Number of invariant assessments reporting a violation.
    pub violated: usize,

    /// Number of assessments reporting an unknown outcome.
    pub unknown: usize,

    /// Number of assessments reporting unresolved prerequisites.
    pub unresolved: usize,

    /// Number of assessments reporting incomplete verification.
    pub incomplete: usize,

    /// Number of invariants explicitly marked not assessed.
    pub not_assessed: usize,

    /// Number of invariant declarations without an assessment record.
    pub missing_assessment: usize,
}

/// Complete output of invariant analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantAnalysisResult {
    /// Overall result.
    pub status: InvariantAnalysisStatus,

    /// Whether all requested structural and aggregation checks completed.
    pub complete: bool,

    /// Counts of canonical assessment outcomes.
    pub summary: InvariantAssessmentSummary,

    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<InvariantDiagnostic>,
}

impl InvariantAnalysisResult {
    fn from_parts(
        mut diagnostics: Vec<InvariantDiagnostic>,
        summary: InvariantAssessmentSummary,
        complete: bool,
    ) -> Self {
        diagnostics.sort_by(|left, right| {
            (
                left.contract_id,
                left.code,
                left.severity,
                &left.message,
            )
                .cmp(&(
                    right.contract_id,
                    right.code,
                    right.severity,
                    &right.message,
                ))
        });

        let incomplete = !complete
            || diagnostics.iter().any(|diagnostic| {
                matches!(
                    diagnostic.code,
                    InvariantDiagnosticCode::BudgetExceeded
                        | InvariantDiagnosticCode::IncompleteAnalysis
                )
            });

        let invalid = diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == InvariantDiagnosticSeverity::Error
        });

        let status = if incomplete {
            InvariantAnalysisStatus::Incomplete
        } else if invalid {
            InvariantAnalysisStatus::Invalid
        } else if summary.violated > 0 {
            InvariantAnalysisStatus::Violated
        } else if summary.unknown > 0
            || summary.unresolved > 0
            || summary.incomplete > 0
            || summary.not_assessed > 0
            || summary.missing_assessment > 0
        {
            InvariantAnalysisStatus::Inconclusive
        } else {
            InvariantAnalysisStatus::Verified
        };

        Self {
            status,
            complete: !incomplete,
            summary,
            diagnostics,
        }
    }
}

// -----------------------------------------------------------------------------
// Public analysis entry point
// -----------------------------------------------------------------------------

/// Analyze HDL invariant declarations and their provider-reported assessments.
///
/// This function checks declaration kinds, non-empty names, canonical identity
/// uniqueness, assessment identity matching, duplicate assessment records,
/// and the presence of evidence classifications for decisive outcomes.
///
/// It does not execute an invariant or independently validate a proof.
///
/// # Determinism
///
/// Diagnostics are sorted by contract identity, diagnostic code, severity,
/// and message. Assessment aggregation is independent of input assessment
/// order.
///
/// # Budget behavior
///
/// When either the declaration or assessment budget is exceeded, analysis
/// returns `Incomplete` without claiming that the omitted inputs were checked.
/// Diagnostic retention exhaustion also returns `Incomplete`.
///
/// # Examples
///
/// ```ignore
/// let result = analyze_invariants(&contracts, &assessments, &InvariantLimits::default());
/// assert_ne!(result.status, InvariantAnalysisStatus::Verified);
/// // A verified result requires valid declarations and sufficient evidence
/// // for every invariant in the supplied scope.
/// ```
#[must_use]
pub fn analyze_invariants(
    contracts: &[HdlContract],
    assessments: &[HdlContractAssessment],
    limits: &InvariantLimits,
) -> InvariantAnalysisResult {
    let mut diagnostics = Vec::new();
    let mut summary = InvariantAssessmentSummary::default();
    let mut complete = true;

    if limits
        .max_invariants
        .is_some_and(|maximum| contracts.len() > maximum)
    {
        diagnostics.push(InvariantDiagnostic::warning(
            InvariantDiagnosticCode::BudgetExceeded,
            None,
            "invariant declaration budget exceeded; analysis is incomplete",
        ));

        return InvariantAnalysisResult::from_parts(diagnostics, summary, false);
    }

    if limits
        .max_assessments
        .is_some_and(|maximum| assessments.len() > maximum)
    {
        diagnostics.push(InvariantDiagnostic::warning(
            InvariantDiagnosticCode::BudgetExceeded,
            None,
            "invariant assessment budget exceeded; analysis is incomplete",
        ));

        return InvariantAnalysisResult::from_parts(diagnostics, summary, false);
    }

    // Keep the first declaration for each identity as the deterministic
    // canonical representative. Duplicate declarations remain errors and
    // must never silently overwrite the original declaration.
    let mut contracts_by_id = BTreeMap::<ContractId, &HdlContract>::new();
    let mut duplicate_contract_ids = BTreeSet::<ContractId>::new();

    for contract in contracts {
        if contract.kind != HdlContractKind::Invariant {
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::AssessmentForNonInvariant,
                Some(contract.id),
                format!(
                    "contract `{}` is not an invariant declaration",
                    contract.name
                ),
            ));
            continue;
        }

        if contract.name.trim().is_empty() {
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::InvalidInvariantName,
                Some(contract.id),
                "invariant name cannot be empty or whitespace only",
            ));
        }

        if contracts_by_id.contains_key(&contract.id) {
            duplicate_contract_ids.insert(contract.id);
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::DuplicateInvariantIdentity,
                Some(contract.id),
                format!("duplicate invariant identity `{}`", contract.id),
            ));
        } else {
            contracts_by_id.insert(contract.id, contract);
        }
    }

    // Index assessments without allowing a later duplicate to overwrite an
    // earlier one. An identity with multiple assessments is ambiguous and
    // cannot be considered verified.
    let mut assessments_by_id =
        BTreeMap::<ContractId, &HdlContractAssessment>::new();
    let mut duplicate_assessment_ids = BTreeSet::<ContractId>::new();

    for assessment in assessments {
        let id = assessment.contract_id;

        if !contracts_by_id.contains_key(&id) {
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::UnknownContractAssessment,
                Some(id),
                format!(
                    "assessment references unknown or non-invariant contract `{id}`"
                ),
            ));
            continue;
        }

        if assessments_by_id.contains_key(&id) {
            duplicate_assessment_ids.insert(id);
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::DuplicateAssessment,
                Some(id),
                format!("multiple assessments were supplied for invariant `{id}`"),
            ));
        } else {
            assessments_by_id.insert(id, assessment);
        }
    }

    // Iterate in canonical identity order, independent of caller collection
    // order, so summaries and diagnostics are reproducible.
    for (id, _contract) in &contracts_by_id {
        if duplicate_contract_ids.contains(id) || duplicate_assessment_ids.contains(id) {
            // The record is ambiguous. Do not choose an assessment and call
            // it verified merely because one candidate reports satisfaction.
            continue;
        }

        let Some(assessment) = assessments_by_id.get(id) else {
            summary.missing_assessment = summary.missing_assessment.saturating_add(1);

            diagnostics.push(InvariantDiagnostic::warning(
                InvariantDiagnosticCode::MissingAssessment,
                Some(*id),
                format!("invariant `{id}` has no assessment record"),
            ));

            continue;
        };

        let decisive = matches!(
            assessment.outcome,
            HdlContractOutcome::Satisfied | HdlContractOutcome::Violated
        );

        if decisive && assessment.evidence_kind == HdlContractEvidenceKind::None {
            diagnostics.push(InvariantDiagnostic::error(
                InvariantDiagnosticCode::MissingEvidence,
                Some(*id),
                format!(
                    "invariant `{id}` has a decisive outcome without an evidence classification"
                ),
            ));
            continue;
        }

        match assessment.outcome {
            HdlContractOutcome::Satisfied => {
                summary.satisfied = summary.satisfied.saturating_add(1);
            }
            HdlContractOutcome::Violated => {
                summary.violated = summary.violated.saturating_add(1);
            }
            HdlContractOutcome::Unknown => {
                summary.unknown = summary.unknown.saturating_add(1);
            }
            HdlContractOutcome::Unresolved => {
                summary.unresolved = summary.unresolved.saturating_add(1);
            }
            HdlContractOutcome::Incomplete => {
                summary.incomplete = summary.incomplete.saturating_add(1);
                complete = false;

                diagnostics.push(InvariantDiagnostic::warning(
                    InvariantDiagnosticCode::IncompleteAnalysis,
                    Some(*id),
                    format!("verification of invariant `{id}` did not complete"),
                ));
            }
            HdlContractOutcome::NotAssessed => {
                summary.not_assessed = summary.not_assessed.saturating_add(1);
            }
        }
    }

    if let Some(maximum) = limits.max_diagnostics {
        if diagnostics.len() > maximum {
            diagnostics.truncate(maximum);

            diagnostics.push(InvariantDiagnostic::warning(
                InvariantDiagnosticCode::BudgetExceeded,
                None,
                "diagnostic retention budget exceeded; diagnostics may be omitted",
            ));

            complete = false;
        }
    }

    InvariantAnalysisResult::from_parts(diagnostics, summary, complete)
}

// -----------------------------------------------------------------------------
// Error type for adapters that need a fallible boundary
// -----------------------------------------------------------------------------

/// Input/configuration error for invariant-verification adapters.
///
/// `analyze_invariants` reports malformed declaration and assessment records
/// as structured diagnostics. This error type is intended for adapters that
/// must validate their own configuration before calling the analyzer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvariantInputError {
    /// An adapter supplied a configuration that it cannot honor.
    InvalidConfiguration,
}

impl fmt::Display for InvariantInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration => {
                formatter.write_str("invalid invariant analysis configuration")
            }
        }
    }
}

impl std::error::Error for InvariantInputError {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::NodeId;
    use crate::frontend::ast::node::source::Span;
    use crate::frontend::semantic::model::ContractId;
    use crate::hdl::contract::{
        HdlContract, HdlContractCheckability, HdlContractExpression,
    };

    fn contract(id: u64, name: &str) -> HdlContract {
        HdlContract::new(
            ContractId::new(id).expect("non-zero test identity"),
            name,
            HdlContractKind::Invariant,
            None,
            HdlContractExpression::new(
                NodeId::new(id).expect("non-zero test node identity"),
                None,
            ),
            HdlContractCheckability::Unproven,
            None,
        )
        .expect("valid invariant contract")
    }

    fn assessment(
        id: u64,
        outcome: HdlContractOutcome,
        evidence_kind: HdlContractEvidenceKind,
    ) -> HdlContractAssessment {
        HdlContractAssessment::new(
            ContractId::new(id).expect("non-zero test identity"),
            outcome,
            evidence_kind,
            None::<Span>,
        )
    }

    fn unlimited() -> InvariantLimits {
        InvariantLimits::default()
    }

    #[test]
    fn empty_scope_is_verified_vacuously() {
        let result = analyze_invariants(&[], &[], &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Verified);
        assert!(result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn satisfied_invariant_requires_evidence_classification() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::FormalProof,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Verified);
        assert_eq!(result.summary.satisfied, 1);
    }

    #[test]
    fn unknown_outcome_never_becomes_verified() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Unknown,
            HdlContractEvidenceKind::None,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Inconclusive);
        assert_eq!(result.summary.unknown, 1);
    }

    #[test]
    fn missing_assessment_is_inconclusive() {
        let contracts = [contract(1, "state_is_valid")];

        let result = analyze_invariants(&contracts, &[], &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Inconclusive);
        assert_eq!(result.summary.missing_assessment, 1);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == InvariantDiagnosticCode::MissingAssessment
        }));
    }

    #[test]
    fn violated_invariant_is_not_reported_as_verified() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Violated,
            HdlContractEvidenceKind::FormalProof,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Violated);
        assert_eq!(result.summary.violated, 1);
    }

    #[test]
    fn decisive_outcome_without_evidence_is_invalid() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::None,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == InvariantDiagnosticCode::MissingEvidence
        }));
    }

    #[test]
    fn duplicate_contract_identity_is_invalid() {
        let contracts = [
            contract(1, "first_invariant"),
            contract(1, "second_invariant"),
        ];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::FormalProof,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == InvariantDiagnosticCode::DuplicateInvariantIdentity
        }));
    }

    #[test]
    fn duplicate_assessments_are_invalid() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [
            assessment(
                1,
                HdlContractOutcome::Satisfied,
                HdlContractEvidenceKind::FormalProof,
            ),
            assessment(
                1,
                HdlContractOutcome::Satisfied,
                HdlContractEvidenceKind::StaticAnalysis,
            ),
        ];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == InvariantDiagnosticCode::DuplicateAssessment
        }));
    }

    #[test]
    fn orphaned_assessment_is_invalid() {
        let assessments = [assessment(
            99,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::FormalProof,
        )];

        let result = analyze_invariants(&[], &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == InvariantDiagnosticCode::UnknownContractAssessment
        }));
    }

    #[test]
    fn incomplete_outcome_makes_the_whole_result_incomplete() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Incomplete,
            HdlContractEvidenceKind::Extension,
        )];

        let result = analyze_invariants(&contracts, &assessments, &unlimited());

        assert_eq!(result.status, InvariantAnalysisStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn declaration_budget_exhaustion_is_incomplete() {
        let contracts = [contract(1, "state_is_valid")];
        let limits = InvariantLimits {
            max_invariants: Some(0),
            ..InvariantLimits::default()
        };

        let result = analyze_invariants(&contracts, &[], &limits);

        assert_eq!(result.status, InvariantAnalysisStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn assessment_budget_exhaustion_is_incomplete() {
        let contracts = [contract(1, "state_is_valid")];
        let assessments = [assessment(
            1,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::FormalProof,
        )];
        let limits = InvariantLimits {
            max_assessments: Some(0),
            ..InvariantLimits::default()
        };

        let result = analyze_invariants(&contracts, &assessments, &limits);

        assert_eq!(result.status, InvariantAnalysisStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn assessment_input_order_does_not_change_the_result() {
        let contracts = [contract(1, "first"), contract(2, "second")];
        let first = assessment(
            1,
            HdlContractOutcome::Satisfied,
            HdlContractEvidenceKind::FormalProof,
        );
        let second = assessment(
            2,
            HdlContractOutcome::Unknown,
            HdlContractEvidenceKind::None,
        );

        let forward =
            analyze_invariants(&contracts, &[first.clone(), second.clone()], &unlimited());
        let reverse = analyze_invariants(&contracts, &[second, first], &unlimited());

        assert_eq!(forward, reverse);
    }
}
