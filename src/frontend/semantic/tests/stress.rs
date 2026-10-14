
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Analysis — Stress Tests
//!
//! File: `src/frontend/semantic/tests/stress.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Exercise the semantic-model storage and lifecycle contracts under large,
//! progressively generated workloads. Detect regressions in deterministic
//! iteration, checked AST identities, fact/provenance retention, obligation
//! tracking, finalization, and error-state handling.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/program.rs`
//! - `src/frontend/semantic/model/facts.rs`
//! - `src/frontend/semantic/model/ids.rs`
//!
//! Normative specifications define language meaning. The AST owns source
//! structure and AST identity. The semantic model owns semantic facts and
//! model lifecycle. This test module owns only stress-test workloads.
//!
//! ## DOES NOT OWN
//!
//! This file does not implement:
//!
//! - parsing or grammar recognition;
//! - AST storage or AST graph validation;
//! - name resolution or symbol-table construction;
//! - type inference or type checking;
//! - effects, capabilities, resources, or contract semantics;
//! - quantum, classical, HDL, tensor, or distributed validation;
//! - target discovery, feasibility, scheduling, or resource allocation;
//! - IR construction, lowering, or runtime execution.
//!
//! ## SCALABILITY CONTRACT
//!
//! The workload sizes below are finite test inputs, not semantic limits.
//! There is no language-level maximum implied by any test constant.
//!
//! A finite process cannot process literally infinite input. Workloads are
//! bounded here to keep ordinary test runs useful on different hosts. Larger
//! workloads belong in separately configured stress or benchmark jobs.
//!
//! A production implementation must not interpret these test sizes as
//! maximum AST nodes, facts, declarations, obligations, domains, qubits,
//! devices, tensor dimensions, or hardware resources.
//!
//! ## INTEGRATION CONTRACT
//!
//! This file tests the existing program-level semantic-model API directly.
//! It deliberately does not assume that a complete semantic-analysis service,
//! AST storage provider, or domain-validation pipeline has been implemented.
//!
//! The parent test module must register this file with `mod stress;`.
//!
//! The test uses:
//!
//! - `Program::without_metadata` for the canonical AST root;
//! - `NodeId::first` and `NodeId::checked_next` for checked identities;
//! - `Span::default` for a source span independent of machine properties;
//! - `SemanticProgram` for facts, provenance, obligations, and lifecycle.
//!
//! The AST graph-membership of generated IDs is not established here. That
//! validation belongs to AST storage and the future integrated pipeline.
//!
//! ## FREEZE CRITERIA
//!
//! - The file compiles against the canonical AST and semantic-model APIs.
//! - All workloads retain every successfully inserted item.
//! - Iteration order is deterministic and independent of insertion order
//!   wherever the model contract declares it to be deterministic.
//! - Checked identity generation never silently wraps.
//! - Pending obligations prevent valid finalization.
//! - Incomplete or aborted analysis never certifies validity.
//! - No test imposes a language-level resource ceiling.
//! - The test is registered by the semantic test module.
//!
//! These tests are necessary but do not alone certify the full semantic
//! subsystem as production-ready.

use std::collections::BTreeSet;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::program::Program as AstProgram;
use crate::frontend::ast::node::source::Span;
use crate::frontend::semantic::model::program::{
    AnalysisCompletion,
    SemanticProgram,
    SemanticProgramError,
    SemanticValidity,
};

// These values control the cost of this test suite only.
// They are not language limits or semantic-model capacity restrictions.
const FACT_WORKLOAD: usize = 12_000;
const PROVENANCE_WORKLOAD: usize = 4_096;
const OBLIGATION_WORKLOAD: usize = 2_048;
const REPEATED_FACT_WORKLOAD: usize = 8_192;

/// Construct the smallest canonical AST root used by model stress tests.
///
/// The model's storage contract needs a stable AST root identity. These tests
/// intentionally do not manufacture a parallel AST representation.
fn ast_program() -> AstProgram {
    AstProgram::without_metadata(NodeId::first(), Span::default())
}

/// Return `count` distinct, consecutive AST node IDs.
///
/// Identity allocation uses the AST's checked successor operation. An
/// exhausted representation is a test failure rather than permission to wrap
/// around and reuse an identity.
fn consecutive_node_ids(count: usize) -> Vec<NodeId> {
    let mut ids = Vec::with_capacity(count);
    let mut current = NodeId::first();

    for index in 0..count {
        ids.push(current);

        if index + 1 < count {
            current = current
                .checked_next()
                .expect("the requested stress workload must fit NodeId");
        }
    }

    ids
}

#[test]
fn large_fact_workload_retains_every_fact() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);
    let ids = consecutive_node_ids(FACT_WORKLOAD);

    // Reverse insertion order deliberately differs from deterministic
    // BTreeMap iteration order.
    for (index, node_id) in ids.iter().copied().enumerate().rev() {
        model
            .add_fact(node_id, format!("fact-{index}"))
            .expect("inserting a fact into an unfinalized model must succeed");
    }

    assert_eq!(model.fact_node_count(), FACT_WORKLOAD);
    assert_eq!(model.fact_count(), FACT_WORKLOAD);

    let observed: Vec<NodeId> = model
        .iter_facts()
        .map(|fact| fact.node_id())
        .collect();

    let expected: Vec<NodeId> = ids.iter().copied().collect();

    assert_eq!(
        observed, expected,
        "fact iteration must follow deterministic AST-node identity order"
    );

    for (index, node_id) in ids.iter().copied().enumerate() {
        let facts = model
            .facts_for(node_id)
            .expect("every inserted node must retain its facts");

        assert_eq!(facts, &[format!("fact-{index}")]);
    }

    assert_eq!(
        model.completion(),
        AnalysisCompletion::Incomplete,
        "mutating semantic facts must not certify completed analysis"
    );

    assert_eq!(model.validity(), SemanticValidity::Unknown);
    assert!(!model.is_valid());
}

#[test]
fn many_facts_for_one_node_preserve_insertion_order() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);
    let node_id = ast.id();

    for index in 0..REPEATED_FACT_WORKLOAD {
        model
            .add_fact(node_id, format!("fact-{index}"))
            .expect("the model is not finalized");
    }

    assert_eq!(model.fact_node_count(), 1);
    assert_eq!(model.fact_count(), REPEATED_FACT_WORKLOAD);

    let facts = model
        .facts_for(node_id)
        .expect("the root must retain its associated facts");

    assert_eq!(facts.len(), REPEATED_FACT_WORKLOAD);
    assert_eq!(facts.first().map(String::as_str), Some("fact-0"));
    assert_eq!(
        facts.last().map(String::as_str),
        Some(format!("fact-{}", REPEATED_FACT_WORKLOAD - 1).as_str())
    );

    for (index, fact) in facts.iter().enumerate() {
        assert_eq!(fact, &format!("fact-{index}"));
    }
}

#[test]
fn large_provenance_workload_is_retained_and_deterministic() {
    let ast = ast_program();
    let mut model: SemanticProgram<String, (), String> =
        SemanticProgram::new(&ast);
    let ids = consecutive_node_ids(PROVENANCE_WORKLOAD);

    for (index, node_id) in ids.iter().copied().enumerate().rev() {
        model
            .add_provenance(node_id, format!("origin-{index}"))
            .expect("provenance insertion must succeed before finalization");
    }

    let observed: Vec<(NodeId, &str)> = model
        .iter_provenance()
        .map(|(node_id, provenance)| (node_id, provenance.as_str()))
        .collect();

    assert_eq!(observed.len(), PROVENANCE_WORKLOAD);

    for (index, (node_id, provenance)) in observed.iter().enumerate() {
        assert_eq!(*node_id, ids[index]);
        assert_eq!(*provenance, format!("origin-{index}"));
        assert_eq!(
            model
                .provenance_for(*node_id)
                .expect("inserted provenance must remain accessible"),
            &[format!("origin-{index}")]
        );
    }
}

#[test]
fn large_obligation_workload_blocks_validity_until_discharged() {
    let ast = ast_program();
    let mut model: SemanticProgram<String, String> = SemanticProgram::new(&ast);

    for index in 0..OBLIGATION_WORKLOAD {
        model
            .add_obligation(format!("obligation-{index}"))
            .expect("adding an obligation before finalization must succeed");
    }

    assert_eq!(model.obligation_count(), OBLIGATION_WORKLOAD);
    assert_eq!(
        model.pending_obligation_count(),
        OBLIGATION_WORKLOAD
    );

    model
        .mark_complete()
        .expect("an analysis without abort or failure may complete");

    let error = model
        .finalize_validity(SemanticValidity::Valid)
        .expect_err("pending obligations must prevent valid finalization");

    assert!(matches!(
        error,
        SemanticProgramError::UnresolvedObligations { count }
            if count == OBLIGATION_WORKLOAD
    ));

    assert!(!model.is_valid());
    assert_eq!(model.validity(), SemanticValidity::Unknown);

    for index in 0..OBLIGATION_WORKLOAD {
        model
            .obligation_mut(index)
            .expect("every obligation index must exist")
            .discharge();
    }

    assert_eq!(model.pending_obligation_count(), 0);

    // Mutating obligations invalidates the prior completion state, so
    // completion must be explicitly re-established before finalization.
    model
        .mark_complete()
        .expect("analysis may complete after all checks are performed");

    model
        .finalize_validity(SemanticValidity::Valid)
        .expect("completed analysis with no pending obligations may finalize");

    assert!(model.is_valid());
    assert_eq!(model.validity(), SemanticValidity::Valid);
}

#[test]
fn duplicate_node_fact_associations_are_retained_without_duplicate_keys() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);
    let ids = consecutive_node_ids(FACT_WORKLOAD);

    // Repeatedly associate facts with a deterministic subset of node IDs.
    // The subset is a test workload, not a model capacity.
    let mut unique_ids = BTreeSet::new();

    for (index, node_id) in ids.iter().copied().enumerate() {
        unique_ids.insert(node_id);

        model
            .add_fact(node_id, format!("first-{index}"))
            .expect("first fact must be retained");

        model
            .add_fact(node_id, format!("second-{index}"))
            .expect("second fact must be retained");
    }

    assert_eq!(unique_ids.len(), FACT_WORKLOAD);
    assert_eq!(model.fact_node_count(), FACT_WORKLOAD);
    assert_eq!(model.fact_count(), FACT_WORKLOAD * 2);

    for (index, node_id) in ids.iter().copied().enumerate() {
        assert_eq!(
            model.facts_for(node_id).expect("facts must exist"),
            &[
                format!("first-{index}"),
                format!("second-{index}")
            ]
        );
    }
}

#[test]
fn insertion_order_does_not_change_node_key_iteration_order() {
    let ast = ast_program();
    let ids = consecutive_node_ids(FACT_WORKLOAD);

    let mut forward: SemanticProgram<String> = SemanticProgram::new(&ast);
    let mut reverse: SemanticProgram<String> = SemanticProgram::new(&ast);

    for node_id in ids.iter().copied() {
        forward
            .add_fact(node_id, node_id.to_string())
            .expect("forward insertion must succeed");
    }

    for node_id in ids.iter().copied().rev() {
        reverse
            .add_fact(node_id, node_id.to_string())
            .expect("reverse insertion must succeed");
    }

    let forward_snapshot: Vec<(NodeId, Vec<String>)> = forward
        .facts()
        .iter()
        .map(|(node_id, facts)| (*node_id, facts.clone()))
        .collect();

    let reverse_snapshot: Vec<(NodeId, Vec<String>)> = reverse
        .facts()
        .iter()
        .map(|(node_id, facts)| (*node_id, facts.clone()))
        .collect();

    assert_eq!(
        forward_snapshot, reverse_snapshot,
        "the same node-keyed facts must have the same iteration order"
    );
}

#[test]
fn aborted_analysis_cannot_be_promoted_to_complete_or_valid() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

    model
        .add_fact(ast.id(), "partial-fact".to_owned())
        .expect("partial fact insertion must succeed");

    model.mark_aborted();

    assert_eq!(model.completion(), AnalysisCompletion::Aborted);
    assert!(!model.is_valid());

    assert!(
        matches!(
            model.mark_complete(),
            Err(SemanticProgramError::InvalidCompletionTransition { .. })
        ),
        "an aborted analysis must not be silently promoted to complete"
    );

    assert!(
        model
            .finalize_validity(SemanticValidity::Valid)
            .is_err(),
        "aborted analysis must not certify semantic validity"
    );

    assert!(!model.is_valid());
}

#[test]
fn failed_analysis_cannot_certify_semantic_validity() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

    model.mark_failed();

    assert_eq!(model.completion(), AnalysisCompletion::Failed);
    assert!(!model.is_valid());

    assert!(
        model
            .finalize_validity(SemanticValidity::Valid)
            .is_err(),
        "failed analysis must not certify semantic validity"
    );

    assert!(!model.is_valid());
}

#[test]
fn incomplete_analysis_cannot_certify_semantic_validity() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

    model
        .add_fact(ast.id(), "partial".to_owned())
        .expect("fact insertion must succeed");

    model.mark_incomplete();

    assert_eq!(model.completion(), AnalysisCompletion::Incomplete);
    assert_eq!(model.validity(), SemanticValidity::Unknown);
    assert!(!model.is_valid());

    assert!(
        matches!(
            model.finalize_validity(SemanticValidity::Valid),
            Err(SemanticProgramError::IncompleteCannotBeValid { .. })
        ),
        "incomplete analysis must not certify semantic validity"
    );
}

#[test]
fn checked_node_id_generation_never_wraps_or_reuses_ids() {
    let ids = consecutive_node_ids(FACT_WORKLOAD);
    let unique: BTreeSet<NodeId> = ids.iter().copied().collect();

    assert_eq!(unique.len(), ids.len());

    for pair in ids.windows(2) {
        assert_eq!(
            pair[0].checked_next(),
            Some(pair[1]),
            "consecutive identities must advance through checked arithmetic"
        );
    }

    // The numeric identity is not a hardware capacity or source-language
    // limit. This test verifies only that the chosen workload's IDs are
    // representable and distinct.
    assert!(ids.iter().all(|node_id| node_id.get() != 0));
}

#[test]
fn empty_model_remains_unvalidated_until_explicit_finalization() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

    assert_eq!(model.ast_root(), ast.id());
    assert_eq!(model.fact_node_count(), 0);
    assert_eq!(model.fact_count(), 0);
    assert_eq!(model.obligation_count(), 0);
    assert_eq!(model.pending_obligation_count(), 0);
    assert!(!model.is_valid());

    model
        .mark_complete()
        .expect("empty analysis may complete");

    model
        .finalize_validity(SemanticValidity::Valid)
        .expect("the model lifecycle permits explicit valid finalization");

    assert!(model.is_valid());
}

#[test]
fn adding_facts_after_valid_finalization_is_rejected() {
    let ast = ast_program();
    let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

    model
        .mark_complete()
        .expect("analysis must complete before finalization");

    model
        .finalize_validity(SemanticValidity::Valid)
        .expect("valid finalization must succeed");

    let error = model
        .add_fact(ast.id(), "late-fact".to_owned())
        .expect_err("finalized valid models must reject new facts");

    assert!(matches!(
        error,
        SemanticProgramError::AlreadyFinalized { .. }
    ));

    assert!(model.is_valid());
    assert_eq!(model.fact_count(), 0);
}
