#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Semantic Contract Integration Tests
//!
//! Path: `src/frontend/semantic/tests/contracts.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Verify the public integration contracts of Zamani's semantic contract
//! model, precondition analysis, postcondition collection, and invariant
//! collection.
//!
//! These tests exercise the existing production interfaces. They do not
//! introduce another contract model, expression representation, verifier,
//! AST, parser, or diagnostic system.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/grammar.md`
//! - `grammar/Zamani.g4`
//! - `grammar/statements/contract.g4`
//! - `grammar/functions/contracts.g4`
//! - `grammar/validation/preconditions.g4`
//! - `grammar/validation/postconditions.g4`
//! - `grammar/validation/invariants.g4`
//! - `grammar/contracts/ast-mappings.yaml`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/contracts/`
//!
//! The normative specifications define semantic meaning. The canonical
//! grammar defines source syntax. The AST owns source identity and spans.
//! The semantic contract model owns clauses, obligations, and outcomes.
//!
//! ## OWNS
//!
//! - Public contract API integration tests.
//! - Cross-module contract invariants.
//! - Regression tests for obligation lifecycle and analysis completion.
//! - Regression tests for canonical grammar ownership.
//! - Regression tests for target-independent contract behavior.
//!
//! ## DOES_NOT_OWN
//!
//! - Production semantic implementation.
//! - Grammar rules or parser behavior.
//! - Expression evaluation or theorem proving.
//! - AST construction or graph-membership validation.
//! - The semantic diagnostic registry.
//! - Target discovery, physical allocation, scheduling, or lowering.
//!
//! ## INTEGRATION CONTRACT
//!
//! The parent `src/frontend/semantic/tests/mod.rs` must declare:
//!
//!     mod contracts;
//!
//! The parent semantic module must expose `contracts`, and the contracts
//! module must expose its `model`, `preconditions`, `postconditions`, and
//! `invariants` submodules.
//!
//! This file imports only the public interfaces of those modules.
//! It deliberately does not depend on private implementation fields.
//!
//! The AST's canonical `NodeId` and `Span` types are used throughout.
//! Test-only source identities are deterministic and locally allocated.
//!
//! Grammar checks use `include_str!` to inspect the checked-out normative
//! grammar files at compile time. They verify ownership boundaries, not
//! whether an ANTLR parser has been generated or successfully executed.
//!
//! ## SCALABILITY
//!
//! These tests introduce no hardware limits or domain-count limits.
//! Test data sizes are selected per test, and all identity construction
//! uses checked, non-zero AST node identities.
//!
//! A finite test process cannot test literally infinite inputs. Scalability
//! is tested through symbolic/target-independent contracts, configurable
//! analysis budgets, explicit incomplete states, and absence of arbitrary
//! hardware ceilings.
//!
//! ## DETERMINISM
//!
//! Fixtures use explicit identities and stable source order. Tests must not
//! rely on hash-map iteration order, wall-clock time, global counters,
//! environment-specific hardware, or network access.
//!
//! ## ERROR HANDLING
//!
//! Invalid contract structure must be rejected. Unsupported, unresolved,
//! disproven, and incomplete verification must not be treated as proven.
//! Budget exhaustion must be distinguishable from successful completion.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze this file when:
//!
//! 1. the parent semantic test module includes it;
//! 2. the public contract APIs match the imported interfaces;
//! 3. grammar paths match the repository's canonical files;
//! 4. all tests pass with the supported Rust toolchain;
//! 5. the complete workspace and semantic integration tests pass;
//! 6. the test suite contains no unsafe code or machine-specific assumptions.

// =============================================================================
// Public API imports
// =============================================================================

use crate::frontend::ast::node::{NodeId, Span};
use crate::frontend::semantic::contracts::invariants::{
    collect_invariants, invariant_count, InvariantCompletion, InvariantOptions,
};
use crate::frontend::semantic::contracts::model::{
    Contract, ContractClause, ContractExpression, ContractKind, ContractMessage,
    ContractModelError, ObligationIdAllocator, ObligationOrigin,
    VerificationOutcome,
};
use crate::frontend::semantic::contracts::postconditions::{
    collect_postconditions, postcondition_count,
};
use crate::frontend::semantic::contracts::preconditions::{
    analyze_preconditions, PredicateAnalysis, PredicateEffects, PredicateTruth,
    PredicateType, PreconditionCompletion, PreconditionOptions,
    PreconditionSemantics, PreconditionStatus,
};

// =============================================================================
// Deterministic test fixtures
// =============================================================================

/// Construct a valid, non-zero AST identity for test data.
///
/// Production code must obtain identities through the AST construction
/// layer. Tests use explicit values to make fixture construction
/// deterministic and independent of global state.
fn node_id(raw: u64) -> NodeId {
    NodeId::new(raw).expect("test AST node identities must be non-zero")
}

/// Construct the canonical empty source span.
///
/// A zero-length span is valid in the AST coordinate model and is suitable
/// for these source-identity tests. Span/source-map correspondence is
/// verified by the AST/source-map subsystem, not by this contract suite.
fn span() -> Span {
    Span::default()
}

/// Construct a contract clause with explicit source and expression IDs.
fn clause(
    source_node: u64,
    expression_node: u64,
    kind: ContractKind,
) -> ContractClause {
    ContractClause::new(
        node_id(source_node),
        span(),
        kind,
        ContractExpression::new(node_id(expression_node), span()),
        None,
    )
}

/// Construct a contract with deterministic source-order clause identities.
///
/// Each clause uses an identity distinct from the contract and its
/// expression. The helper preserves the caller's kind ordering.
fn contract_with(kinds: &[ContractKind]) -> Contract {
    let clauses = kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| {
            let offset = u64::try_from(index)
                .expect("test fixture index must fit in u64")
                .checked_mul(2)
                .and_then(|value| value.checked_add(2))
                .expect("test fixture identity must not overflow");

            let expression_node = offset
                .checked_add(1)
                .expect("test expression identity must not overflow");

            clause(offset, expression_node, kind.clone())
        })
        .collect();

    Contract::new(node_id(1), span(), clauses)
        .expect("test contract must satisfy structural invariants")
}

/// A deterministic semantic assessor for testing precondition integration.
///
/// It returns established facts and never evaluates source expressions.
/// The assessment is fixed for each instance, making repeated analyses
/// reproducible.
#[derive(Clone)]
struct FixedSemantics {
    assessment: PredicateAnalysis,
}

impl FixedSemantics {
    fn proven_true() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Pure,
                PredicateTruth::ProvenTrue {
                    evidence: Some("test:proven-true".to_owned()),
                },
            ),
        }
    }

    fn proven_false() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Pure,
                PredicateTruth::ProvenFalse {
                    evidence: Some("test:proven-false".to_owned()),
                },
            ),
        }
    }

    fn runtime_check_required() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Pure,
                PredicateTruth::RuntimeCheckRequired,
            ),
        }
    }

    fn non_boolean() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::NonBoolean {
                    actual: "integer".to_owned(),
                },
                PredicateEffects::Pure,
                PredicateTruth::ProvenTrue {
                    evidence: Some("must-not-bypass-type-checking".to_owned()),
                },
            ),
        }
    }

    fn effectful() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Effectful {
                    effects: vec!["io.write".to_owned()],
                },
                PredicateTruth::ProvenTrue {
                    evidence: Some("must-not-bypass-effect-checking".to_owned()),
                },
            ),
        }
    }

    fn incomplete() -> Self {
        Self {
            assessment: PredicateAnalysis::new(
                PredicateType::Boolean,
                PredicateEffects::Pure,
                PredicateTruth::Incomplete {
                    reason: "test analysis budget exhausted".to_owned(),
                },
            ),
        }
    }
}

impl PreconditionSemantics for FixedSemantics {
    fn analyze_condition(
        &self,
        _expression: ContractExpression,
    ) -> PredicateAnalysis {
        self.assessment.clone()
    }
}

/// Run precondition analysis with a fresh, explicitly owned obligation
/// allocator.
fn analyze(
    contract: &Contract,
    semantics: &impl PreconditionSemantics,
    options: PreconditionOptions,
) -> crate::frontend::semantic::contracts::preconditions::PreconditionAnalysis {
    analyze_preconditions(
        contract,
        semantics,
        &mut ObligationIdAllocator::new(),
        options,
    )
    .expect("the small test fixture must not exhaust obligation identities")
}

// =============================================================================
// Canonical contract model
// =============================================================================

#[test]
fn contract_module_exposes_stable_schema_identity() {
    use crate::frontend::semantic::contracts::{
        module_name, schema_version, CONTRACTS_MODULE_SCHEMA_VERSION,
    };

    assert_eq!(module_name(), "zamani.frontend.semantic.contracts");
    assert_eq!(schema_version(), CONTRACTS_MODULE_SCHEMA_VERSION);
    assert_ne!(CONTRACTS_MODULE_SCHEMA_VERSION, 0);
}

#[test]
fn contract_preserves_source_identity_and_clause_order() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    assert_eq!(contract.source_node(), node_id(1));
    assert_eq!(contract.source_span(), span());
    assert_eq!(contract.clause_count(), 3);
    assert!(!contract.is_empty());

    assert_eq!(
        contract.clauses()[0].kind(),
        &ContractKind::Precondition
    );
    assert_eq!(
        contract.clauses()[1].kind(),
        &ContractKind::Postcondition
    );
    assert_eq!(
        contract.clauses()[2].kind(),
        &ContractKind::Invariant
    );

    assert_eq!(contract.clauses()[0].source_node(), node_id(2));
    assert_eq!(contract.clauses()[0].condition().node_id(), node_id(3));
    assert_eq!(contract.clauses()[1].source_node(), node_id(4));
    assert_eq!(contract.clauses()[1].condition().node_id(), node_id(5));
}

#[test]
fn contract_rejects_duplicate_clause_source_identities() {
    let duplicate_identity = node_id(2);

    let first = ContractClause::new(
        duplicate_identity,
        span(),
        ContractKind::Precondition,
        ContractExpression::new(node_id(3), span()),
        None,
    );

    let second = ContractClause::new(
        duplicate_identity,
        span(),
        ContractKind::Postcondition,
        ContractExpression::new(node_id(4), span()),
        None,
    );

    let result = Contract::new(node_id(1), span(), vec![first, second]);

    assert!(matches!(
        result,
        Err(ContractModelError::DuplicateClauseNode { node_id })
            if node_id == duplicate_identity
    ));
}

#[test]
fn empty_contract_is_structurally_representable() {
    let contract = Contract::new(node_id(1), span(), Vec::new())
        .expect("an empty contract is representable by the model");

    assert!(contract.is_empty());
    assert_eq!(contract.clause_count(), 0);
    assert!(contract.clauses().is_empty());
}

#[test]
fn optional_contract_message_is_metadata_not_proof_evidence() {
    let message = ContractMessage::new("input must satisfy the precondition");

    let clause = ContractClause::new(
        node_id(2),
        span(),
        ContractKind::Precondition,
        ContractExpression::new(node_id(3), span()),
        Some(message),
    );

    assert_eq!(
        clause.message().map(ContractMessage::as_str),
        Some("input must satisfy the precondition")
    );

    let contract = Contract::new(node_id(1), span(), vec![clause])
        .expect("the clause must form a valid contract");

    let mut ids = ObligationIdAllocator::new();
    let obligations = collect_postconditions(&contract, &mut ids)
        .expect("absence of postconditions must be handled");

    assert!(obligations.is_empty());
}

// =============================================================================
// Obligation identity and verification lifecycle
// =============================================================================

#[test]
fn obligation_ids_are_monotonic_within_one_allocator() {
    let mut ids = ObligationIdAllocator::new();

    let first = ids.allocate().expect("first obligation ID");
    let second = ids.allocate().expect("second obligation ID");
    let third = ids.allocate().expect("third obligation ID");

    assert_eq!(first.get(), 1);
    assert_eq!(second.get(), 2);
    assert_eq!(third.get(), 3);
    assert!(first < second);
    assert!(second < third);
}

#[test]
fn postcondition_collection_creates_pending_obligations() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_postconditions(&contract, &mut ids)
        .expect("postcondition collection must succeed");

    assert_eq!(postcondition_count(&contract), 2);
    assert_eq!(result.len(), 2);
    assert!(!result.is_empty());

    let obligations = result.as_slice();

    assert_eq!(obligations[0].id().get(), 1);
    assert_eq!(obligations[1].id().get(), 2);

    assert_eq!(obligations[0].contract_node(), contract.source_node());
    assert_eq!(obligations[0].clause_node(), node_id(4));
    assert_eq!(obligations[1].clause_node(), node_id(6));

    for obligation in obligations {
        assert_eq!(obligation.origin(), ObligationOrigin::Postcondition);
        assert_eq!(obligation.outcome(), &VerificationOutcome::Pending);
        assert!(!obligation.is_proven());
    }
}

#[test]
fn empty_postcondition_selection_does_not_consume_ids() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Invariant,
        ContractKind::Assertion,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_postconditions(&contract, &mut ids)
        .expect("an empty selection is successful");

    assert!(result.is_empty());

    let next = ids.allocate().expect("allocator remains usable");
    assert_eq!(next.get(), 1);
}

#[test]
fn only_explicit_proven_outcome_counts_as_proven() {
    let outcomes = [
        VerificationOutcome::Pending,
        VerificationOutcome::Disproven { evidence: None },
        VerificationOutcome::Unknown { reason: None },
        VerificationOutcome::Unsupported { reason: None },
        VerificationOutcome::Incomplete { reason: None },
    ];

    for outcome in outcomes {
        assert!(
            !outcome.is_proven(),
            "non-proven outcome was accepted: {outcome:?}"
        );
        assert!(outcome.requires_attention());
    }

    assert!(
        VerificationOutcome::Proven { evidence: None }.is_proven()
    );
}

#[test]
fn recording_an_outcome_is_explicit_and_local_to_the_obligation() {
    let contract = contract_with(&[ContractKind::Postcondition]);
    let mut ids = ObligationIdAllocator::new();

    let obligations = collect_postconditions(&contract, &mut ids)
        .expect("collection succeeds");

    let mut obligation = obligations.as_slice()[0].clone();

    assert!(!obligation.is_proven());

    obligation.record_outcome(VerificationOutcome::Proven {
        evidence: Some("trusted-test-verifier:evidence-1".to_owned()),
    });

    assert!(obligation.is_proven());
    assert!(matches!(
        obligation.outcome(),
        VerificationOutcome::Proven {
            evidence: Some(evidence)
        } if evidence == "trusted-test-verifier:evidence-1"
    ));

    // Recording a result on a cloned obligation does not mutate the
    // original collection or authenticate the supplied evidence.
    assert_eq!(
        obligations.as_slice()[0].outcome(),
        &VerificationOutcome::Pending
    );
}

// =============================================================================
// Preconditions
// =============================================================================

#[test]
fn proven_true_precondition_is_reported_as_proven() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(result.all_preconditions_proven());
    assert_eq!(result.reports().len(), 1);

    let report = &result.reports()[0];

    assert_eq!(report.status(), PreconditionStatus::Proven);
    assert!(report.is_proven());
    assert_eq!(report.contract_node(), contract.source_node());
    assert_eq!(report.clause_node(), node_id(2));
    assert_eq!(report.condition().node_id(), node_id(3));
    assert_eq!(
        report.obligation().origin(),
        ObligationOrigin::Precondition
    );
    assert!(report.obligation().is_proven());
}

#[test]
fn proven_false_precondition_is_not_accepted() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::proven_false(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(!result.all_preconditions_proven());
    assert_eq!(
        result.reports()[0].status(),
        PreconditionStatus::Disproven
    );
    assert!(!result.reports()[0].obligation().is_proven());
}

#[test]
fn runtime_check_required_is_not_static_proof() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::runtime_check_required(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(!result.all_preconditions_proven());
    assert_eq!(
        result.reports()[0].status(),
        PreconditionStatus::RuntimeCheckRequired
    );
    assert!(!result.reports()[0].obligation().is_proven());
}

#[test]
fn non_boolean_precondition_cannot_be_proven_by_truth_assessment_alone() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::non_boolean(),
        PreconditionOptions::unrestricted(),
    );

    assert_eq!(
        result.reports()[0].status(),
        PreconditionStatus::InvalidType
    );
    assert!(!result.reports()[0].obligation().is_proven());
    assert!(!result.all_preconditions_proven());
}

#[test]
fn effectful_precondition_cannot_bypass_effect_checking() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::effectful(),
        PreconditionOptions::unrestricted(),
    );

    assert_eq!(
        result.reports()[0].status(),
        PreconditionStatus::EffectViolation
    );
    assert!(!result.reports()[0].obligation().is_proven());
}

#[test]
fn incomplete_precondition_assessment_is_not_proof() {
    let contract = contract_with(&[ContractKind::Precondition]);
    let result = analyze(
        &contract,
        &FixedSemantics::incomplete(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(!result.all_preconditions_proven());
    assert_eq!(
        result.reports()[0].status(),
        PreconditionStatus::Incomplete
    );
    assert!(!result.reports()[0].obligation().is_proven());
}

#[test]
fn precondition_analysis_ignores_other_contract_kinds() {
    let contract = contract_with(&[
        ContractKind::Postcondition,
        ContractKind::Invariant,
        ContractKind::Assertion,
    ]);

    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(result.reports().is_empty());
    assert!(result.all_preconditions_proven());
}

#[test]
fn zero_precondition_budget_reports_incomplete_analysis() {
    let contract = contract_with(&[
        ContractKind::Postcondition,
        ContractKind::Precondition,
        ContractKind::Precondition,
    ]);

    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::with_max_preconditions(0),
    );

    assert!(!result.is_complete());
    assert!(!result.all_preconditions_proven());
    assert!(result.reports().is_empty());

    assert!(matches!(
        result.completion(),
        PreconditionCompletion::Incomplete { next_clause }
            if *next_clause == node_id(4)
    ));
}

#[test]
fn precondition_budget_counts_only_preconditions() {
    let contract = contract_with(&[
        ContractKind::Postcondition,
        ContractKind::Precondition,
        ContractKind::Invariant,
        ContractKind::Precondition,
    ]);

    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::with_max_preconditions(1),
    );

    assert!(!result.is_complete());
    assert_eq!(result.reports().len(), 1);
    assert_eq!(result.reports()[0].clause_node(), node_id(4));

    assert!(matches!(
        result.completion(),
        PreconditionCompletion::Incomplete { next_clause }
            if *next_clause == node_id(8)
    ));
}

#[test]
fn empty_precondition_selection_completes_successfully() {
    let contract = contract_with(&[
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::unrestricted(),
    );

    assert!(result.is_complete());
    assert!(result.reports().is_empty());
    assert!(result.all_preconditions_proven());
}

// =============================================================================
// Invariants
// =============================================================================

#[test]
fn invariant_count_selects_only_supported_invariant_kinds() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
        ContractKind::LoopInvariant,
        ContractKind::Assertion,
        ContractKind::FailureCondition,
    ]);

    assert_eq!(invariant_count(&contract), 2);
}

#[test]
fn invariant_collection_preserves_order_kind_and_source_identity() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Invariant,
        ContractKind::Postcondition,
        ContractKind::LoopInvariant,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_invariants(
        &contract,
        &mut ids,
        InvariantOptions::unrestricted(),
    )
    .expect("invariant collection succeeds");

    assert!(result.is_complete());
    assert_eq!(result.len(), 2);

    let reports: Vec<_> = result.iter().collect();

    assert_eq!(reports[0].contract_node(), contract.source_node());
    assert_eq!(reports[0].clause_node(), node_id(4));
    assert_eq!(
        reports[0].kind().as_contract_kind(),
        &ContractKind::Invariant
    );
    assert!(!reports[0].kind().is_loop_invariant());

    assert_eq!(reports[1].clause_node(), node_id(8));
    assert_eq!(
        reports[1].kind().as_contract_kind(),
        &ContractKind::LoopInvariant
    );
    assert!(reports[1].kind().is_loop_invariant());

    for report in reports {
        assert_eq!(
            report.obligation().origin(),
            ObligationOrigin::Invariant
        );
        assert_eq!(
            report.obligation().outcome(),
            &VerificationOutcome::Pending
        );
        assert!(!report.is_proven());
    }
}

#[test]
fn invariant_budget_exhaustion_is_explicit() {
    let contract = contract_with(&[
        ContractKind::Invariant,
        ContractKind::Precondition,
        ContractKind::LoopInvariant,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_invariants(
        &contract,
        &mut ids,
        InvariantOptions::with_max_invariants(1),
    )
    .expect("budget exhaustion is a result, not an allocation error");

    assert!(!result.is_complete());
    assert_eq!(result.len(), 1);

    assert!(matches!(
        result.completion(),
        InvariantCompletion::Incomplete { next_clause }
            if next_clause == node_id(6)
    ));

    assert_eq!(result.iter().next().map(|r| r.clause_node()), Some(node_id(2)));
}

#[test]
fn zero_invariant_budget_does_not_create_obligations() {
    let contract = contract_with(&[
        ContractKind::Invariant,
        ContractKind::LoopInvariant,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_invariants(
        &contract,
        &mut ids,
        InvariantOptions::with_max_invariants(0),
    )
    .expect("zero budget must be represented explicitly");

    assert!(!result.is_complete());
    assert!(result.is_empty());

    let next = ids.allocate().expect("uncollected obligations use no IDs");
    assert_eq!(next.get(), 1);
}

#[test]
fn no_invariants_is_a_complete_empty_result() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Assertion,
    ]);

    let mut ids = ObligationIdAllocator::new();
    let result = collect_invariants(
        &contract,
        &mut ids,
        InvariantOptions::unrestricted(),
    )
    .expect("an empty selection succeeds");

    assert!(result.is_complete());
    assert!(result.is_empty());

    let next = ids.allocate().expect("no identity was consumed");
    assert_eq!(next.get(), 1);
}

// =============================================================================
// Cross-pass integration
// =============================================================================

#[test]
fn contract_analysis_passes_share_one_identity_allocator() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    let mut ids = ObligationIdAllocator::new();

    let preconditions = analyze_preconditions(
        &contract,
        &FixedSemantics::proven_true(),
        &mut ids,
        PreconditionOptions::unrestricted(),
    )
    .expect("precondition analysis succeeds");

    let postconditions = collect_postconditions(&contract, &mut ids)
        .expect("postcondition collection succeeds");

    let invariants = collect_invariants(
        &contract,
        &mut ids,
        InvariantOptions::unrestricted(),
    )
    .expect("invariant collection succeeds");

    assert_eq!(preconditions.reports()[0].obligation().id().get(), 1);
    assert_eq!(postconditions.as_slice()[0].id().get(), 2);
    assert_eq!(invariants.iter().next().expect("one invariant").obligation().id().get(), 3);

    assert_eq!(
        preconditions.reports()[0].obligation().origin(),
        ObligationOrigin::Precondition
    );
    assert_eq!(
        postconditions.as_slice()[0].origin(),
        ObligationOrigin::Postcondition
    );
    assert_eq!(
        invariants.iter().next().expect("one invariant").obligation().origin(),
        ObligationOrigin::Invariant
    );
}

#[test]
fn independent_runs_with_identical_inputs_are_deterministic() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    let first_preconditions = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::unrestricted(),
    );
    let second_preconditions = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::unrestricted(),
    );

    assert_eq!(first_preconditions, second_preconditions);

    let mut first_ids = ObligationIdAllocator::new();
    let first_postconditions =
        collect_postconditions(&contract, &mut first_ids).expect("first collection");

    let mut second_ids = ObligationIdAllocator::new();
    let second_postconditions =
        collect_postconditions(&contract, &mut second_ids).expect("second collection");

    assert_eq!(first_postconditions, second_postconditions);

    let mut first_ids = ObligationIdAllocator::new();
    let first_invariants = collect_invariants(
        &contract,
        &mut first_ids,
        InvariantOptions::unrestricted(),
    )
    .expect("first invariant collection");

    let mut second_ids = ObligationIdAllocator::new();
    let second_invariants = collect_invariants(
        &contract,
        &mut second_ids,
        InvariantOptions::unrestricted(),
    )
    .expect("second invariant collection");

    assert_eq!(first_invariants, second_invariants);
}

// =============================================================================
// Canonical grammar ownership
// =============================================================================

// Paths are relative to `src/frontend/semantic/tests/`.
const ROOT_GRAMMAR: &str = include_str!("../../../../grammar/Zamani.g4");
const GRAMMAR_OVERVIEW: &str = include_str!("../../../../grammar/grammar.md");
const STATEMENT_CONTRACT_GRAMMAR: &str =
    include_str!("../../../../grammar/statements/contract.g4");
const FUNCTION_CONTRACT_GRAMMAR: &str =
    include_str!("../../../../grammar/functions/contracts.g4");
const PRECONDITION_VALIDATION_GRAMMAR: &str =
    include_str!("../../../../grammar/validation/preconditions.g4");
const POSTCONDITION_VALIDATION_GRAMMAR: &str =
    include_str!("../../../../grammar/validation/postconditions.g4");
const INVARIANT_VALIDATION_GRAMMAR: &str =
    include_str!("../../../../grammar/validation/invariants.g4");
const AST_CONTRACT_MAPPINGS: &str =
    include_str!("../../../../grammar/contracts/ast-mappings.yaml");

#[test]
fn canonical_root_grammar_is_present_and_domain_neutral() {
    assert!(!ROOT_GRAMMAR.trim().is_empty());
    assert!(!GRAMMAR_OVERVIEW.trim().is_empty());

    // The root orchestrates the grammar hierarchy; it must not become an
    // independent owner of every domain's source-level productions.
    assert!(
        ROOT_GRAMMAR.contains("canonical")
            || ROOT_GRAMMAR.contains("CANONICAL"),
        "root grammar must document its canonical authority"
    );
}

#[test]
fn standalone_contract_syntax_has_one_canonical_owner() {
    for rule in [
        "contractStatement",
        "requiresStatement",
        "ensuresStatement",
        "invariantStatement",
        "contractCondition",
    ] {
        assert!(
            STATEMENT_CONTRACT_GRAMMAR.contains(rule),
            "canonical statement contract grammar is missing rule {rule}"
        );
    }

    assert!(
        STATEMENT_CONTRACT_GRAMMAR.contains("requiresStatement")
    );
    assert!(
        STATEMENT_CONTRACT_GRAMMAR.contains("ensuresStatement")
    );
    assert!(
        STATEMENT_CONTRACT_GRAMMAR.contains("invariantStatement")
    );
}

#[test]
fn function_contract_syntax_preserves_canonical_rule_names() {
    for rule in [
        "functionRequiresContract",
        "functionEnsuresContract",
        "functionInvariantContract",
        "functionContractTerminator",
    ] {
        assert!(
            FUNCTION_CONTRACT_GRAMMAR.contains(rule),
            "function contract grammar is missing rule {rule}"
        );
    }
}

#[test]
fn validation_grammars_delegate_instead_of_redefining_contract_syntax() {
    assert!(
        PRECONDITION_VALIDATION_GRAMMAR.contains("requiresStatement"),
        "precondition validation must reuse canonical syntax"
    );

    assert!(
        POSTCONDITION_VALIDATION_GRAMMAR.contains("ensuresStatement"),
        "postcondition validation must reuse canonical syntax"
    );

    assert!(
        INVARIANT_VALIDATION_GRAMMAR.contains("invariantStatement"),
        "invariant validation must reuse canonical syntax"
    );
}

#[test]
fn contract_ast_mapping_specification_is_present() {
    assert!(
        !AST_CONTRACT_MAPPINGS.trim().is_empty(),
        "contract-to-AST mapping must be available for integration review"
    );
}

// =============================================================================
// Portability and scalability regression checks
// =============================================================================

#[test]
fn contract_kinds_are_not_limited_to_a_fixed_hardware_inventory() {
    let extension = ContractKind::Extension(
        crate::frontend::semantic::contracts::model::ContractKindId::new(
            "future.domain-condition",
        )
        .expect("valid namespaced extension contract kind"),
    );

    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Invariant,
        extension.clone(),
    ]);

    assert_eq!(contract.clause_count(), 3);
    assert_eq!(contract.clauses()[2].kind(), &extension);

    // Extension-defined kinds are not silently reclassified as invariants.
    assert_eq!(invariant_count(&contract), 1);
    assert_eq!(postcondition_count(&contract), 0);
}

#[test]
fn symbolic_source_contracts_do_not_require_target_information() {
    // The contract model is target-independent. A source-level predicate
    // can be represented without a CPU, GPU, QPU, FPGA, node count, device
    // inventory, physical layout, or target-feasibility result.
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Postcondition,
        ContractKind::Invariant,
    ]);

    assert_eq!(contract.clause_count(), 3);

    for clause in contract.clauses() {
        assert_ne!(clause.source_node(), clause.condition().node_id());
    }
}

#[test]
fn analysis_budget_is_operational_not_a_proof_of_validity() {
    let contract = contract_with(&[
        ContractKind::Precondition,
        ContractKind::Precondition,
    ]);

    let result = analyze(
        &contract,
        &FixedSemantics::proven_true(),
        PreconditionOptions::with_max_preconditions(1),
    );

    assert_eq!(result.reports().len(), 1);
    assert!(!result.is_complete());
    assert!(!result.all_preconditions_proven());
}