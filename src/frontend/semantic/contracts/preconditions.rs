
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Preconditions
//!
//! Path: `src/frontend/semantic/contracts/preconditions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Semantically assess and record preconditions already represented by the
//! canonical semantic contract model.
//!
//! This module supports standalone requirements and function preconditions
//! after parsing, AST construction, structural validation, and contract
//! extraction have completed.
//!
//! ## AUTHORITY
//!
//! - `grammar/statements/contract.g4`
//! - `grammar/functions/contracts.g4`
//! - `grammar/validation/preconditions.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/contracts/model.rs`
//! - The normative Zamani contract and semantic specifications.
//!
//! Grammar owns source syntax. The canonical AST owns source structure.
//! `contracts/model.rs` owns the semantic contract representation.
//! This module owns precondition assessment and obligation recording.
//!
//! ## OWNS
//!
//! - Preconditions-specific analysis interfaces.
//! - The result of assessing a precondition.
//! - Structured precondition assessment issues.
//! - Mapping assessments to canonical verification outcomes.
//! - Explicit handling of incomplete analysis.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing, grammar productions, or AST construction.
//! - Name resolution, type inference, or general expression checking.
//! - The canonical semantic type or effect systems.
//! - Capability discovery or target-feasibility checking.
//! - Arbitrary expression execution or runtime evaluation.
//! - Theorem proving, solver implementation, or proof authentication.
//! - Diagnostic rendering or diagnostic-code registry ownership.
//! - Physical resource allocation, scheduling, routing, or lowering.
//! - Classical IR, quantum IR, HDL IR, or runtime state.
//!
//! ## INTEGRATION CONTRACT
//!
//! - Input contracts use `contracts/model.rs`.
//! - Conditions reference expressions by canonical AST `NodeId` and `Span`.
//! - `PreconditionSemantics` consumes those references and returns static
//!   semantic facts; it must never execute source expressions.
//! - Existing type and effect analysis remain authoritative for type and
//!   effect facts.
//! - `ObligationIdAllocator` owns obligation identity allocation.
//! - `ContractObligation` and `VerificationOutcome` own recorded outcomes.
//! - The owning diagnostic layer translates `PreconditionIssue` values into
//!   the repository's canonical diagnostic representation.
//! - The semantic pipeline decides when this analysis runs.
//!
//! ## SCALABILITY
//!
//! No hardware capacities, domain counts, fixed precondition counts, or
//! language-level program-size ceilings are introduced.
//!
//! The default analysis budget is unlimited. An explicitly configured budget
//! is an operational compiler policy, not a language restriction. Exhaustion
//! is reported as incomplete analysis; it never means successful verification.
//!
//! ## DETERMINISM
//!
//! Results preserve source/extraction order. The supplied semantic assessor
//! must itself provide deterministic results for equivalent semantic inputs.
//! This module uses no global mutable state, randomness, clock, filesystem,
//! network, or hardware access.
//!
//! ## ERROR HANDLING
//!
//! Obligation-ID exhaustion is returned as an error. Non-Boolean predicates,
//! effectful predicates, unresolved facts, unsupported assessment, and
//! incomplete assessment remain explicit outcomes rather than being accepted.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze this module when:
//!
//! 1. its imports match the canonical contract model;
//! 2. `contracts/mod.rs` exports `model` and `preconditions`;
//! 3. the parent semantic module exports `contracts`;
//! 4. all precondition assessment tests pass;
//! 5. the owning diagnostic layer maps issues to canonical diagnostics;
//! 6. the pipeline propagates incomplete analysis correctly;
//! 7. workspace compilation and semantic integration tests pass.

use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::{NodeId, Span};

use super::model::{
    Contract,
    ContractExpression,
    ContractKind,
    ContractModelError,
    ContractObligation,
    ObligationIdAllocator,
    ObligationOrigin,
    VerificationOutcome,
};

/// Policy for running precondition analysis.
///
/// `max_preconditions: None` means no additional precondition-count budget
/// is imposed by this module. The compiler may still enforce independently
/// configured operational budgets elsewhere.
///
/// A configured budget limits the number of preconditions assessed during
/// this invocation. It does not change source-language validity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PreconditionOptions {
    max_preconditions: Option<usize>,
}

impl PreconditionOptions {
    /// Create options with no additional precondition-count budget.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_preconditions: None,
        }
    }

    /// Create options with an explicit assessment budget.
    ///
    /// A budget of zero assesses no conditions. If at least one precondition
    /// exists, the returned analysis is incomplete.
    #[must_use]
    pub const fn with_max_preconditions(maximum: usize) -> Self {
        Self {
            max_preconditions: Some(maximum),
        }
    }

    /// Return the configured precondition-count budget.
    ///
    /// `None` means that this module imposes no additional count budget.
    #[must_use]
    pub const fn max_preconditions(self) -> Option<usize> {
        self.max_preconditions
    }
}

/// Semantic type classification for a precondition expression.
///
/// This is an assessment supplied by the existing semantic type system. It
/// does not introduce a second semantic type representation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PredicateType {
    /// The expression has been established to have Boolean type.
    Boolean,

    /// The expression has a known non-Boolean type.
    NonBoolean {
        /// Human-readable description of the resolved type.
        actual: String,
    },

    /// The type could not be established.
    Unresolved {
        /// Explanation suitable for diagnostic adaptation.
        reason: String,
    },
}

/// Effect classification for a precondition expression.
///
/// Preconditions are not executed by this module. An expression with effects
/// cannot be statically evaluated through this interface as though it were a
/// pure predicate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PredicateEffects {
    /// The expression has been established to be pure under the language's
    /// normative effect rules.
    Pure,

    /// One or more effects are present.
    Effectful {
        /// Canonical effect names or identifiers, in deterministic order.
        effects: Vec<String>,
    },

    /// The effect information is not available or is inconclusive.
    Unknown {
        /// Explanation of the unresolved effect information.
        reason: String,
    },
}

/// Static truth assessment of a precondition expression.
///
/// `ProvenTrue` and `ProvenFalse` may be returned only when the responsible
/// semantic or verification component has sufficient grounds for that
/// conclusion under its documented soundness assumptions.
///
/// A concrete example evaluation, unsupported guess, or absence of a
/// counterexample is not sufficient evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PredicateTruth {
    /// The condition has been established to hold.
    ProvenTrue {
        /// Optional reference to the evidence supporting this conclusion.
        evidence: Option<String>,
    },

    /// The condition has been established to be false.
    ProvenFalse {
        /// Optional reference to the evidence supporting this conclusion.
        evidence: Option<String>,
    },

    /// Static analysis cannot establish truth, but a separate authorized
    /// runtime-checking mechanism may be able to evaluate the condition.
    RuntimeCheckRequired,

    /// The condition could not be decided.
    Unresolved {
        /// Explanation of the unresolved condition.
        reason: String,
    },

    /// The required assessment is not supported by the available analysis.
    Unsupported {
        /// Explanation of the unsupported operation or theory.
        reason: String,
    },

    /// Assessment did not finish within the applicable operational budget.
    Incomplete {
        /// Explanation of why assessment did not finish.
        reason: String,
    },
}

/// Complete static assessment supplied by the semantic analysis services.
///
/// The assessor must not execute the expression, invoke effectful functions,
/// access hardware, or perform runtime authorization. It reports established
/// type/effect facts and an appropriate static truth classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateAnalysis {
    predicate_type: PredicateType,
    effects: PredicateEffects,
    truth: PredicateTruth,
}

impl PredicateAnalysis {
    /// Construct a complete predicate assessment.
    #[must_use]
    pub const fn new(
        predicate_type: PredicateType,
        effects: PredicateEffects,
        truth: PredicateTruth,
    ) -> Self {
        Self {
            predicate_type,
            effects,
            truth,
        }
    }

    /// Return the predicate's type classification.
    #[must_use]
    pub const fn predicate_type(&self) -> &PredicateType {
        &self.predicate_type
    }

    /// Return the predicate's effect classification.
    #[must_use]
    pub const fn effects(&self) -> &PredicateEffects {
        &self.effects
    }

    /// Return the predicate's static truth assessment.
    #[must_use]
    pub const fn truth(&self) -> &PredicateTruth {
        &self.truth
    }
}

/// Service that assesses precondition expressions using canonical semantic
/// facts.
///
/// Implementations should adapt the existing type, effect, constant-analysis,
/// and verification services. They must not create an alternate symbol table
/// or semantic type system.
///
/// # Safety and soundness contract
///
/// This method is an analysis interface, not an expression evaluator.
/// Implementations must not execute arbitrary source expressions.
///
/// Returning `ProvenTrue` or `ProvenFalse` asserts that the responsible
/// analysis has established that result under its documented soundness
/// assumptions. Callers remain responsible for the trust policy applied to
/// verification evidence.
pub trait PreconditionSemantics {
    /// Assess a condition identified by its canonical AST reference.
    fn analyze_condition(
        &self,
        expression: ContractExpression,
    ) -> PredicateAnalysis;
}

/// Final classification of one assessed precondition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreconditionStatus {
    /// The condition was established to hold.
    Proven,

    /// The condition was established to be false.
    Disproven,

    /// A permitted runtime-checking stage must handle the condition.
    RuntimeCheckRequired,

    /// The condition does not have Boolean type.
    InvalidType,

    /// The condition has effects that this precondition analysis does not
    /// permit it to evaluate as a pure predicate.
    EffectViolation,

    /// Required semantic facts could not be established.
    Unresolved,

    /// The required analysis is unsupported.
    Unsupported,

    /// Analysis did not complete.
    Incomplete,
}

/// Structured issue discovered during precondition assessment.
///
/// These values are intended for translation into the repository's canonical
/// diagnostic system. They are not user-facing rendered diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreconditionIssue {
    /// The predicate has a known non-Boolean type.
    NonBooleanPredicate {
        /// Description of the actual type.
        actual: String,
    },

    /// The predicate's type is unresolved.
    UnresolvedPredicateType {
        /// Explanation of the unresolved type.
        reason: String,
    },

    /// The predicate contains effects disallowed by this pure-assessment
    /// boundary.
    EffectfulPredicate {
        /// Effect identifiers in deterministic order.
        effects: Vec<String>,
    },

    /// Effect information could not be established.
    UnknownPredicateEffects {
        /// Explanation of the missing or inconclusive information.
        reason: String,
    },

    /// The condition was established to be false.
    FalsePrecondition {
        /// Optional evidence reference.
        evidence: Option<String>,
    },

    /// The condition could not be decided.
    UnresolvedCondition {
        /// Explanation of the unresolved condition.
        reason: String,
    },

    /// The requested analysis is unsupported.
    UnsupportedCondition {
        /// Explanation of the unsupported analysis.
        reason: String,
    },

    /// Assessment did not complete.
    IncompleteCondition {
        /// Explanation of the incomplete analysis.
        reason: String,
    },
}

/// Assessment and obligation record for one precondition.
///
/// The source references remain those of the canonical AST and contract
/// model. This record does not duplicate expression syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreconditionReport {
    contract_node: NodeId,
    clause_node: NodeId,
    clause_span: Span,
    condition: ContractExpression,
    status: PreconditionStatus,
    issues: Vec<PreconditionIssue>,
    obligation: ContractObligation,
}

impl PreconditionReport {
    /// Return the AST node owning the contract.
    #[must_use]
    pub const fn contract_node(&self) -> NodeId {
        self.contract_node
    }

    /// Return the AST node introducing the precondition clause.
    #[must_use]
    pub const fn clause_node(&self) -> NodeId {
        self.clause_node
    }

    /// Return the source span of the precondition clause.
    #[must_use]
    pub const fn clause_span(&self) -> Span {
        self.clause_span
    }

    /// Return the canonical expression reference.
    #[must_use]
    pub const fn condition(&self) -> ContractExpression {
        self.condition
    }

    /// Return the final classification of this assessment.
    #[must_use]
    pub const fn status(&self) -> PreconditionStatus {
        self.status
    }

    /// Return structured issues for diagnostic adaptation.
    #[must_use]
    pub fn issues(&self) -> &[PreconditionIssue] {
        &self.issues
    }

    /// Return the recorded verification obligation.
    #[must_use]
    pub const fn obligation(&self) -> &ContractObligation {
        &self.obligation
    }

    /// Return whether the precondition was established to hold.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        matches!(self.status, PreconditionStatus::Proven)
    }
}

/// Completion state for a precondition-analysis invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreconditionCompletion {
    /// Every precondition in the input contract was assessed.
    Completed,

    /// Analysis stopped before all preconditions were assessed.
    ///
    /// `next_clause` identifies the first unassessed precondition clause.
    /// The caller must not treat the overall contract as fully analyzed.
    Incomplete {
        /// AST identity of the next unassessed precondition clause.
        next_clause: NodeId,
    },
}

/// Complete result of assessing the preconditions in one contract.
///
/// Reports preserve source/extraction order. An incomplete result explicitly
/// identifies where analysis stopped; omitted clauses are not considered
/// valid, false, or proven.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreconditionAnalysis {
    reports: Vec<PreconditionReport>,
    completion: PreconditionCompletion,
}

impl PreconditionAnalysis {
    /// Return the reports in source/extraction order.
    #[must_use]
    pub fn reports(&self) -> &[PreconditionReport] {
        &self.reports
    }

    /// Return the completion state.
    #[must_use]
    pub const fn completion(&self) -> PreconditionCompletion {
        self.completion
    }

    /// Return whether every precondition in the input contract was assessed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completion, PreconditionCompletion::Completed)
    }

    /// Return whether all assessed preconditions were proven and analysis
    /// completed.
    ///
    /// An empty set of preconditions is vacuously satisfied only if the
    /// analysis completed. This method does not assert that other contract
    /// kinds, function bodies, effects, or program semantics are valid.
    #[must_use]
    pub fn all_preconditions_proven(&self) -> bool {
        self.is_complete() && self.reports.iter().all(PreconditionReport::is_proven)
    }
}

/// Failure to construct a complete precondition-analysis result.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreconditionAnalysisError {
    /// The canonical obligation-ID allocator exhausted its identity space.
    ObligationIdentityExhausted,
}

impl fmt::Display for PreconditionAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObligationIdentityExhausted => {
                formatter.write_str(
                    "verification-obligation identity space exhausted",
                )
            }
        }
    }
}

impl Error for PreconditionAnalysisError {}

impl From<ContractModelError> for PreconditionAnalysisError {
    fn from(error: ContractModelError) -> Self {
        match error {
            ContractModelError::ObligationIdExhausted => {
                Self::ObligationIdentityExhausted
            }
            _ => Self::ObligationIdentityExhausted,
        }
    }
}

/// Assess all precondition clauses in a canonical semantic contract.
///
/// Only clauses whose canonical kind is `ContractKind::Precondition` are
/// processed. Postconditions, invariants, assertions, and extension-defined
/// clauses remain the responsibility of their owning analysis modules.
///
/// # Arguments
///
/// - `contract`: canonical semantic contract extracted from the validated AST.
/// - `semantics`: adapter over existing semantic type/effect/truth services.
/// - `obligation_ids`: allocator owned by the caller's obligation-ID domain.
/// - `options`: explicit operational analysis policy.
///
/// # Guarantees
///
/// - Does not parse or modify AST nodes.
/// - Does not execute condition expressions.
/// - Does not inspect or allocate hardware.
/// - Does not silently mark unresolved or incomplete work as proven.
/// - Preserves clause order.
/// - Returns an explicit incomplete state when the configured budget is
///   exhausted.
/// - Returns an error if obligation identity allocation fails.
///
/// The caller must validate AST graph membership and source references before
/// invoking this function. This function does not replace general expression
/// validation, symbol resolution, or the normative effect checker.
pub fn analyze_preconditions<S>(
    contract: &Contract,
    semantics: &S,
    obligation_ids: &mut ObligationIdAllocator,
    options: PreconditionOptions,
) -> Result<PreconditionAnalysis, PreconditionAnalysisError>
where
    S: PreconditionSemantics + ?Sized,
{
    let mut clauses = contract
        .clauses()
        .iter()
        .filter(|clause| clause.kind() == &ContractKind::Precondition)
        .peekable();

    let mut reports = Vec::new();

    while let Some(clause) = clauses.next() {
        if options
            .max_preconditions
            .is_some_and(|maximum| reports.len() >= maximum)
        {
            return Ok(PreconditionAnalysis {
                reports,
                completion: PreconditionCompletion::Incomplete {
                    next_clause: clause.source_node(),
                },
            });
        }

        let expression = clause.condition();
        let assessment = semantics.analyze_condition(expression);

        let (status, outcome, issues) = classify_assessment(&assessment);

        let id = obligation_ids
            .allocate()
            .map_err(PreconditionAnalysisError::from)?;

        let mut obligation = ContractObligation::pending(
            id,
            contract.source_node(),
            clause,
            ObligationOrigin::Precondition,
        );

        obligation.record_outcome(outcome);

        reports.push(PreconditionReport {
            contract_node: contract.source_node(),
            clause_node: clause.source_node(),
            clause_span: clause.source_span(),
            condition: expression,
            status,
            issues,
            obligation,
        });
    }

    Ok(PreconditionAnalysis {
        reports,
        completion: PreconditionCompletion::Completed,
    })
}

fn classify_assessment(
    assessment: &PredicateAnalysis,
) -> (
    PreconditionStatus,
    VerificationOutcome,
    Vec<PreconditionIssue>,
) {
    // Type correctness takes precedence over truth assessment. A known
    // non-Boolean expression is not a valid predicate, even if some other
    // component supplied a truth result for it.
    match assessment.predicate_type() {
        PredicateType::NonBoolean { actual } => {
            return (
                PreconditionStatus::InvalidType,
                VerificationOutcome::Unknown {
                    reason: Some(
                        "precondition expression is not Boolean".to_owned(),
                    ),
                },
                vec![PreconditionIssue::NonBooleanPredicate {
                    actual: actual.clone(),
                }],
            );
        }
        PredicateType::Unresolved { reason } => {
            return (
                PreconditionStatus::Unresolved,
                VerificationOutcome::Unknown {
                    reason: Some(reason.clone()),
                },
                vec![PreconditionIssue::UnresolvedPredicateType {
                    reason: reason.clone(),
                }],
            );
        }
        PredicateType::Boolean => {}
    }

    // Do not use a truth result to bypass effect restrictions. This module
    // records semantic facts; it never invokes an effectful condition.
    match assessment.effects() {
        PredicateEffects::Effectful { effects } => {
            return (
                PreconditionStatus::EffectViolation,
                VerificationOutcome::Unknown {
                    reason: Some(
                        "effectful precondition cannot be assessed as a pure predicate"
                            .to_owned(),
                    ),
                },
                vec![PreconditionIssue::EffectfulPredicate {
                    effects: effects.clone(),
                }],
            );
        }
        PredicateEffects::Unknown { reason } => {
            return (
                PreconditionStatus::Unresolved,
                VerificationOutcome::Unknown {
                    reason: Some(reason.clone()),
                },
                vec![PreconditionIssue::UnknownPredicateEffects {
                    reason: reason.clone(),
                }],
            );
        }
        PredicateEffects::Pure => {}
    }

    match assessment.truth() {
        PredicateTruth::ProvenTrue { evidence } => (
            PreconditionStatus::Proven,
            VerificationOutcome::Proven {
                evidence: evidence.clone(),
            },
            Vec::new(),
        ),

        PredicateTruth::ProvenFalse { evidence } => (
            PreconditionStatus::Disproven,
            VerificationOutcome::Disproven {
                evidence: evidence.clone(),
            },
            vec![PreconditionIssue::FalsePrecondition {
                evidence: evidence.clone(),
            }],
        ),

        PredicateTruth::RuntimeCheckRequired => (
            PreconditionStatus::RuntimeCheckRequired,
            VerificationOutcome::Pending,
            Vec::new(),
        ),

        PredicateTruth::Unresolved { reason } => (
            PreconditionStatus::Unresolved,
            VerificationOutcome::Unknown {
                reason: Some(reason.clone()),
            },
            vec![PreconditionIssue::UnresolvedCondition {
                reason: reason.clone(),
            }],
        ),

        PredicateTruth::Unsupported { reason } => (
            PreconditionStatus::Unsupported,
            VerificationOutcome::Unsupported {
                reason: Some(reason.clone()),
            },
            vec![PreconditionIssue::UnsupportedCondition {
                reason: reason.clone(),
            }],
        ),

        PredicateTruth::Incomplete { reason } => (
            PreconditionStatus::Incomplete,
            VerificationOutcome::Incomplete {
                reason: Some(reason.clone()),
            },
            vec![PreconditionIssue::IncompleteCondition {
                reason: reason.clone(),
            }],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedSemantics {
        assessment: PredicateAnalysis,
    }

    impl PreconditionSemantics for FixedSemantics {
        fn analyze_condition(
            &self,
            _expression: ContractExpression,
        ) -> PredicateAnalysis {
            self.assessment.clone()
        }
    }

    fn node_after(node: NodeId) -> NodeId {
        node.checked_next().expect("test node ID available")
    }

    fn contract_with(
        kinds: &[ContractKind],
    ) -> Contract {
        let contract_node = NodeId::first();
        let contract_span = Span::default();

        let mut next = node_after(contract_node);
        let mut clauses = Vec::new();

        for kind in kinds {
            let clause_node = next;
            next = node_after(next);

            let expression_node = next;
            next = node_after(next);

            clauses.push(super::super::model::ContractClause::new(
                clause_node,
                contract_span,
                kind.clone(),
                ContractExpression::new(expression_node, contract_span),
                None,
            ));
        }

        Contract::new(contract_node, contract_span, clauses)
            .expect("test contract is structurally valid")
    }

    fn semantics(truth: PredicateTruth) -> FixedSemantics {
        FixedSemantics {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Pure,
                truth,
            ),
        }
    }

    fn analyze(
        contract: &Contract,
        semantics: &impl PreconditionSemantics,
        options: PreconditionOptions,
    ) -> PreconditionAnalysis {
        analyze_preconditions(
            contract,
            semantics,
            &mut ObligationIdAllocator::new(),
            options,
        )
        .expect("test obligation allocation should succeed")
    }

    #[test]
    fn proven_true_precondition_records_proven_outcome() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = semantics(PredicateTruth::ProvenTrue {
            evidence: Some("test-evidence".to_owned()),
        });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert!(result.is_complete());
        assert!(result.all_preconditions_proven());
        assert_eq!(result.reports().len(), 1);
        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::Proven,
        );
        assert!(result.reports()[0].obligation().is_proven());
    }

    #[test]
    fn proven_false_precondition_is_not_accepted() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = semantics(PredicateTruth::ProvenFalse {
            evidence: None,
        });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert!(!result.all_preconditions_proven());
        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::Disproven,
        );
        assert!(matches!(
            result.reports()[0].obligation().outcome(),
            VerificationOutcome::Disproven { .. }
        ));
        assert!(matches!(
            result.reports()[0].issues(),
            [PreconditionIssue::FalsePrecondition { .. }]
        ));
    }

    #[test]
    fn runtime_check_required_remains_pending() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = semantics(PredicateTruth::RuntimeCheckRequired);

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert!(!result.all_preconditions_proven());
        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::RuntimeCheckRequired,
        );
        assert!(matches!(
            result.reports()[0].obligation().outcome(),
            VerificationOutcome::Pending
        ));
    }

    #[test]
    fn non_boolean_predicate_cannot_be_marked_proven() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = FixedSemantics {
            assessment: PredicateAnalysis::new(
                PredicateType::NonBoolean {
                    actual: "Integer".to_owned(),
                },
                PredicateEffects::Pure,
                PredicateTruth::ProvenTrue { evidence: None },
            ),
        };

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::InvalidType,
        );
        assert!(!result.reports()[0].obligation().is_proven());
        assert!(matches!(
            result.reports()[0].issues(),
            [PreconditionIssue::NonBooleanPredicate { .. }]
        ));
    }

    #[test]
    fn effectful_predicate_cannot_bypass_effect_checks() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = FixedSemantics {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Effectful {
                    effects: vec!["io".to_owned()],
                },
                PredicateTruth::ProvenTrue { evidence: None },
            ),
        };

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::EffectViolation,
        );
        assert!(!result.reports()[0].obligation().is_proven());
    }

    #[test]
    fn unresolved_condition_is_never_proven() {
        let contract = contract_with(&[ContractKind::Precondition]);
        let service = semantics(PredicateTruth::Unresolved {
            reason: "symbol is unresolved".to_owned(),
        });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::Unresolved,
        );
        assert!(!result.reports()[0].obligation().is_proven());
    }

    #[test]
    fn unsupported_and_incomplete_assessments_remain_explicit() {
        let contract = contract_with(&[ContractKind::Precondition]);

        let unsupported = analyze(
            &contract,
            &semantics(PredicateTruth::Unsupported {
                reason: "required theory unavailable".to_owned(),
            }),
            PreconditionOptions::unrestricted(),
        );

        assert_eq!(
            unsupported.reports()[0].status(),
            PreconditionStatus::Unsupported,
        );

        let incomplete = analyze(
            &contract,
            &semantics(PredicateTruth::Incomplete {
                reason: "analysis budget exhausted".to_owned(),
            }),
            PreconditionOptions::unrestricted(),
        );

        assert_eq!(
            incomplete.reports()[0].status(),
            PreconditionStatus::Incomplete,
        );
        assert!(!incomplete.all_preconditions_proven());
    }

    #[test]
    fn non_precondition_clauses_are_not_processed() {
        let contract = contract_with(&[
            ContractKind::Precondition,
            ContractKind::Postcondition,
            ContractKind::Invariant,
        ]);
        let service = semantics(PredicateTruth::ProvenTrue { evidence: None });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert!(result.is_complete());
        assert_eq!(result.reports().len(), 1);
        assert_eq!(
            result.reports()[0].status(),
            PreconditionStatus::Proven,
        );
    }

    #[test]
    fn zero_budget_returns_incomplete_with_next_clause() {
        let contract = contract_with(&[
            ContractKind::Precondition,
            ContractKind::Precondition,
        ]);
        let service = semantics(PredicateTruth::ProvenTrue { evidence: None });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::with_max_preconditions(0),
        );

        assert!(result.reports().is_empty());
        assert!(!result.is_complete());

        let expected_next = contract
            .clauses()
            .iter()
            .find(|clause| clause.kind() == &ContractKind::Precondition)
            .expect("test precondition")
            .source_node();

        assert_eq!(
            result.completion(),
            PreconditionCompletion::Incomplete {
                next_clause: expected_next,
            },
        );
        assert!(!result.all_preconditions_proven());
    }

    #[test]
    fn budget_limits_assessment_not_source_validity() {
        let contract = contract_with(&[
            ContractKind::Precondition,
            ContractKind::Precondition,
        ]);
        let service = semantics(PredicateTruth::ProvenTrue { evidence: None });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::with_max_preconditions(1),
        );

        assert_eq!(result.reports().len(), 1);
        assert!(!result.is_complete());

        let expected_next = contract
            .clauses()
            .iter()
            .filter(|clause| clause.kind() == &ContractKind::Precondition)
            .nth(1)
            .expect("second test precondition")
            .source_node();

        assert_eq!(
            result.completion(),
            PreconditionCompletion::Incomplete {
                next_clause: expected_next,
            },
        );
    }

    #[test]
    fn empty_precondition_set_completes_successfully() {
        let contract = contract_with(&[
            ContractKind::Postcondition,
            ContractKind::Invariant,
        ]);
        let service = semantics(PredicateTruth::ProvenTrue { evidence: None });

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        assert!(result.is_complete());
        assert!(result.reports().is_empty());
        assert!(result.all_preconditions_proven());
    }

    #[test]
    fn source_order_is_preserved() {
        let contract = contract_with(&[
            ContractKind::Precondition,
            ContractKind::Postcondition,
            ContractKind::Precondition,
        ]);
        let service = semantics(PredicateTruth::RuntimeCheckRequired);

        let result = analyze(
            &contract,
            &service,
            PreconditionOptions::unrestricted(),
        );

        let expected: Vec<NodeId> = contract
            .clauses()
            .iter()
            .filter(|clause| clause.kind() == &ContractKind::Precondition)
            .map(|clause| clause.source_node())
            .collect();

        let actual: Vec<NodeId> = result
            .reports()
            .iter()
            .map(PreconditionReport::clause_node)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn obligation_ids_are_allocated_by_the_caller_owned_allocator() {
        let contract = contract_with(&[
            ContractKind::Precondition,
            ContractKind::Precondition,
        ]);
        let service = semantics(PredicateTruth::RuntimeCheckRequired);
        let mut allocator = ObligationIdAllocator::new();

        let result = analyze_preconditions(
            &contract,
            &service,
            &mut allocator,
            PreconditionOptions::unrestricted(),
        )
        .expect("test obligation allocation");

        assert_eq!(result.reports().len(), 2);
        assert_ne!(
            result.reports()[0].obligation().id(),
            result.reports()[1].obligation().id(),
        );
    }
}
