#![forbid(unsafe_code)]

//! Ownership and borrowing conformance tests for Zamani's native frontend.
//!
//! File: src/frontend/semantic/tests/ownership.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Authority
//!
//! - Canonical AST: `crate::frontend::ast::node`.
//! - Ownership engine: `crate::frontend::semantic::types::ownership`.
//! - Ownership syntax: `grammar/memory/ownership.g4`.
//! - Borrowing syntax: `grammar/memory/borrowing.g4`.
//! - Reference syntax: `grammar/memory/references.g4`.
//! - Reference type syntax: `grammar/types/reference.g4`.
//! - Semantic rules: `grammar/specification/semantic-model.md`.
//!
//! These are semantic-engine tests. They do not replace parser tests or
//! claim that the AST-to-control-flow-graph adapter has been implemented.
//!
//! # Test contract
//!
//! The suite verifies the behavior exposed by the ownership engine:
//!
//! - valid initialization and reads;
//! - moves, use after move, and reinitialization;
//! - shared and mutable borrow rules;
//! - place overlap and conservative alias handling;
//! - ownership across control-flow joins;
//! - scope-outlives relationships;
//! - linear and affine consumption behavior;
//! - malformed graph handling;
//! - deterministic diagnostic ordering;
//! - explicit incomplete results when operational budgets are exhausted.
//!
//! No test assumes that a program's meaning depends on a particular machine,
//! accelerator, quantum device, register width, or resource count.
//!
//! # Integration
//!
//! Include this file from `src/frontend/semantic/types/mod.rs` under
//! `#[cfg(test)]`, using the module declaration described after this file.
//!
//! These tests intentionally use only the ownership engine's public API.

use std::collections::BTreeMap;

use crate::frontend::semantic::types::ownership::{
    analyze_ownership,
    AnalysisStatus,
    BasicBlock,
    BlockId,
    BorrowKind,
    ControlFlowGraph,
    GraphError,
    IndexProjection,
    LoanId,
    OwnershipConfig,
    OwnershipDiagnosticCode,
    OwnershipEvent,
    OwnershipMode,
    PlaceId,
    PlacePath,
    Projection,
    ScopeId,
    ScopeTree,
    ScopeTreeError,
    SourceSite,
};

// -----------------------------------------------------------------------------
// Shared test helpers
// -----------------------------------------------------------------------------

fn site() -> SourceSite {
    SourceSite::default()
}

fn root_scope() -> ScopeTree {
    let mut scopes = ScopeTree::default();

    scopes
        .insert(ScopeId(0), None)
        .expect("the root scope should be accepted");

    scopes
}

fn root_and_child_scopes() -> ScopeTree {
    let mut scopes = root_scope();

    scopes
        .insert(ScopeId(1), Some(ScopeId(0)))
        .expect("a child of the registered root should be accepted");

    scopes
}

fn root_place(id: u64) -> PlacePath {
    PlacePath::root(PlaceId(id))
}

fn declare(
    place: PlacePath,
    mode: OwnershipMode,
    scope: ScopeId,
    initialized: bool,
) -> OwnershipEvent {
    OwnershipEvent::Declare {
        place,
        mode,
        scope,
        initialized,
        site: site(),
    }
}

fn read(place: PlacePath) -> OwnershipEvent {
    OwnershipEvent::Read {
        place,
        site: site(),
    }
}

fn move_place(place: PlacePath) -> OwnershipEvent {
    OwnershipEvent::Move {
        place,
        site: site(),
    }
}

fn write(place: PlacePath) -> OwnershipEvent {
    OwnershipEvent::Write {
        place,
        site: site(),
    }
}

fn borrow(
    loan: u64,
    place: PlacePath,
    kind: BorrowKind,
    borrower_scope: ScopeId,
) -> OwnershipEvent {
    OwnershipEvent::Borrow {
        loan: LoanId(loan),
        place,
        kind,
        borrower_scope,
        site: site(),
    }
}

fn end_borrow(loan: u64) -> OwnershipEvent {
    OwnershipEvent::EndBorrow {
        loan: LoanId(loan),
        site: site(),
    }
}

fn use_borrow(loan: u64) -> OwnershipEvent {
    OwnershipEvent::UseBorrow {
        loan: LoanId(loan),
        site: site(),
    }
}

fn drop_place(place: PlacePath) -> OwnershipEvent {
    OwnershipEvent::Drop {
        place,
        site: site(),
    }
}

fn scope_exit(scope: ScopeId) -> OwnershipEvent {
    OwnershipEvent::ScopeExit {
        scope,
        site: site(),
    }
}

fn single_block_graph(
    events: Vec<OwnershipEvent>,
    scopes: ScopeTree,
) -> ControlFlowGraph {
    let entry = BlockId(0);

    let block = BasicBlock {
        id: entry,
        events,
        successors: Vec::new(),
    };

    let mut blocks = BTreeMap::new();
    blocks.insert(entry, block);

    ControlFlowGraph {
        entry,
        blocks,
        scopes,
    }
}

fn analyze(events: Vec<OwnershipEvent>) -> crate::frontend::semantic::types::ownership::OwnershipReport {
    analyze_ownership(
        &single_block_graph(events, root_scope()),
        &OwnershipConfig::default(),
    )
}

fn contains_code(
    report: &crate::frontend::semantic::types::ownership::OwnershipReport,
    expected: OwnershipDiagnosticCode,
) -> bool {
    report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == expected)
}

fn assert_no_diagnostics(
    report: &crate::frontend::semantic::types::ownership::OwnershipReport,
) {
    assert_eq!(
        report.status,
        AnalysisStatus::Complete,
        "expected complete analysis, got {:?}",
        report.status
    );

    assert!(
        report.diagnostics.is_empty(),
        "expected no diagnostics, got: {:?}",
        report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>()
    );

    assert!(
        report.is_valid(),
        "a complete analysis without diagnostics should be valid"
    );
}

// -----------------------------------------------------------------------------
// Basic initialization and move semantics
// -----------------------------------------------------------------------------

#[test]
fn initialized_place_can_be_read() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        read(place),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn uninitialized_place_cannot_be_read() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            false,
        ),
        read(place),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::ReadBeforeInitialization
    ));
    assert_eq!(report.status, AnalysisStatus::Complete);
    assert!(!report.is_valid());
}

#[test]
fn moving_an_initialized_place_invalidates_the_original() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        move_place(place.clone()),
        read(place),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::UseAfterMove
    ));
}

#[test]
fn writing_a_moved_place_reinitializes_it() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        move_place(place.clone()),
        write(place.clone()),
        read(place),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn affine_value_may_remain_unconsumed() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place,
            OwnershipMode::Affine,
            ScopeId(0),
            true,
        ),
        scope_exit(ScopeId(0)),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn linear_value_must_be_consumed_before_scope_exit() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place,
            OwnershipMode::Linear,
            ScopeId(0),
            true,
        ),
        scope_exit(ScopeId(0)),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::LinearValueNotConsumed
    ));
}

#[test]
fn moving_a_linear_value_satisfies_consumption_requirement() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Linear,
            ScopeId(0),
            true,
        ),
        move_place(place),
        scope_exit(ScopeId(0)),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn dropping_a_linear_value_is_reported() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Linear,
            ScopeId(0),
            true,
        ),
        drop_place(place),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::LinearValueDropped
    ));
}

// -----------------------------------------------------------------------------
// Borrowing rules
// -----------------------------------------------------------------------------

#[test]
fn multiple_shared_borrows_of_the_same_place_are_allowed() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Shared, ScopeId(0)),
        borrow(2, place.clone(), BorrowKind::Shared, ScopeId(0)),
        use_borrow(1),
        use_borrow(2),
        end_borrow(1),
        end_borrow(2),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn shared_borrow_allows_reading_the_owner() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Shared, ScopeId(0)),
        read(place),
        end_borrow(1),
    ]);

    assert_no_diagnostics(&report);
}

#[test]
fn mutable_borrow_conflicts_with_an_existing_shared_borrow() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Shared, ScopeId(0)),
        borrow(2, place, BorrowKind::Mutable, ScopeId(0)),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::ConflictingBorrow
    ));
}

#[test]
fn mutable_borrow_conflicts_with_an_existing_mutable_borrow() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Mutable, ScopeId(0)),
        borrow(2, place, BorrowKind::Mutable, ScopeId(0)),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::ConflictingBorrow
    ));
}

#[test]
fn reading_owner_during_mutable_borrow_is_rejected() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Mutable, ScopeId(0)),
        read(place),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::AccessWhileMutablyBorrowed
    ));
}

#[test]
fn mutation_during_shared_borrow_is_rejected() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Shared, ScopeId(0)),
        OwnershipEvent::Mutate {
            place,
            site: site(),
        },
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::MutationWhileBorrowed
    ));
}

#[test]
fn moving_a_borrowed_place_is_rejected() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place.clone(), BorrowKind::Shared, ScopeId(0)),
        move_place(place),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::MoveWhileBorrowed
    ));
}

#[test]
fn using_an_unknown_loan_is_rejected() {
    let report = analyze(vec![use_borrow(404)]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::UnknownLoan
    ));
}

#[test]
fn ending_an_unknown_loan_is_rejected() {
    let report = analyze(vec![end_borrow(404)]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::UnknownLoan
    ));
}

// -----------------------------------------------------------------------------
// Scope and lifetime relationships
// -----------------------------------------------------------------------------

#[test]
fn parent_scope_outlives_child_scope() {
    let scopes = root_and_child_scopes();

    assert!(scopes.outlives(ScopeId(0), ScopeId(1)));
    assert!(scopes.outlives(ScopeId(0), ScopeId(0)));
    assert!(!scopes.outlives(ScopeId(1), ScopeId(0)));
}

#[test]
fn duplicate_scope_registration_is_rejected() {
    let mut scopes = root_scope();

    let result = scopes.insert(ScopeId(0), None);

    assert_eq!(result, Err(ScopeTreeError::DuplicateScope(ScopeId(0))));
}

#[test]
fn scope_cannot_reference_an_unregistered_parent() {
    let mut scopes = ScopeTree::default();

    let result = scopes.insert(ScopeId(2), Some(ScopeId(1)));

    assert_eq!(
        result,
        Err(ScopeTreeError::UnknownParent {
            scope: ScopeId(2),
            parent: ScopeId(1),
        })
    );
}

#[test]
fn borrow_cannot_outlive_its_owner_scope() {
    let place = root_place(1);

    let report = analyze_ownership(
        &single_block_graph(
            vec![
                declare(
                    place.clone(),
                    OwnershipMode::Unrestricted,
                    ScopeId(1),
                    true,
                ),
                borrow(1, place, BorrowKind::Shared, ScopeId(0)),
            ],
            root_and_child_scopes(),
        ),
        &OwnershipConfig::default(),
    );

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::BorrowOutlivesOwner
    ));
}

#[test]
fn owner_scope_exit_with_an_active_borrow_is_reported() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        borrow(1, place, BorrowKind::Shared, ScopeId(0)),
        scope_exit(ScopeId(0)),
    ]);

    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::BorrowOutlivesOwner
    ));
}

// -----------------------------------------------------------------------------
// Place paths and conservative overlap analysis
// -----------------------------------------------------------------------------

#[test]
fn different_roots_do_not_overlap() {
    assert!(!root_place(1).may_overlap(&root_place(2)));
}

#[test]
fn different_named_fields_are_disjoint() {
    let root = root_place(1);

    assert!(!root.field("left").may_overlap(&root.field("right")));
}

#[test]
fn a_parent_place_overlaps_its_field() {
    let root = root_place(1);
    let field = root.field("value");

    assert!(root.may_overlap(&field));
    assert!(field.may_overlap(&root));
}

#[test]
fn distinct_constant_indices_are_disjoint() {
    let root = root_place(1);

    let first = root.index(IndexProjection::Constant("0".into()));
    let second = root.index(IndexProjection::Constant("1".into()));

    assert!(!first.may_overlap(&second));
}

#[test]
fn dynamic_indices_are_conservatively_overlapping() {
    let root = root_place(1);

    let dynamic = root.index(IndexProjection::Dynamic);
    let constant = root.index(IndexProjection::Constant("7".into()));

    assert!(dynamic.may_overlap(&constant));
    assert!(constant.may_overlap(&dynamic));
}

#[test]
fn different_extension_projections_are_conservative() {
    let root = root_place(1);

    let first = root.field("container");
    let second = PlacePath {
        root: PlaceId(1),
        projections: vec![Projection::Extension {
            namespace: "custom".into(),
            key: "slot".into(),
        }],
    };

    assert!(first.may_overlap(&second));
}

// -----------------------------------------------------------------------------
// Control-flow graph validation and dataflow
// -----------------------------------------------------------------------------

#[test]
fn missing_entry_block_produces_incomplete_analysis() {
    let graph = ControlFlowGraph {
        entry: BlockId(10),
        blocks: BTreeMap::new(),
        scopes: root_scope(),
    };

    assert_eq!(graph.validate(), Err(GraphError::MissingEntry(BlockId(10))));

    let report = analyze_ownership(&graph, &OwnershipConfig::default());

    assert_eq!(report.status, AnalysisStatus::Incomplete);
    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::InvalidControlFlowGraph
    ));
    assert!(!report.is_valid());
}

#[test]
fn missing_successor_is_rejected_before_analysis() {
    let entry = BlockId(0);

    let block = BasicBlock {
        id: entry,
        events: Vec::new(),
        successors: vec![BlockId(99)],
    };

    let mut blocks = BTreeMap::new();
    blocks.insert(entry, block);

    let graph = ControlFlowGraph {
        entry,
        blocks,
        scopes: root_scope(),
    };

    assert_eq!(
        graph.validate(),
        Err(GraphError::MissingSuccessor {
            block: BlockId(0),
            successor: BlockId(99),
        })
    );
}

#[test]
fn inconsistent_block_map_identity_is_rejected() {
    let block = BasicBlock {
        id: BlockId(1),
        events: Vec::new(),
        successors: Vec::new(),
    };

    let mut blocks = BTreeMap::new();
    blocks.insert(BlockId(0), block);

    let graph = ControlFlowGraph {
        entry: BlockId(0),
        blocks,
        scopes: root_scope(),
    };

    assert_eq!(
        graph.validate(),
        Err(GraphError::BlockKeyMismatch {
            key: BlockId(0),
            block: BlockId(1),
        })
    );
}

#[test]
fn read_after_move_at_control_flow_join_is_not_definitely_valid() {
    let place = root_place(1);

    let entry = BlockId(0);
    let moved_path = BlockId(1);
    let unchanged_path = BlockId(2);
    let join = BlockId(3);

    let mut blocks = BTreeMap::new();

    blocks.insert(
        entry,
        BasicBlock {
            id: entry,
            events: vec![declare(
                place.clone(),
                OwnershipMode::Unrestricted,
                ScopeId(0),
                true,
            )],
            successors: vec![moved_path, unchanged_path],
        },
    );

    blocks.insert(
        moved_path,
        BasicBlock {
            id: moved_path,
            events: vec![move_place(place.clone())],
            successors: vec![join],
        },
    );

    blocks.insert(
        unchanged_path,
        BasicBlock {
            id: unchanged_path,
            events: Vec::new(),
            successors: vec![join],
        },
    );

    blocks.insert(
        join,
        BasicBlock {
            id: join,
            events: vec![read(place)],
            successors: Vec::new(),
        },
    );

    let graph = ControlFlowGraph {
        entry,
        blocks,
        scopes: root_scope(),
    };

    let report = analyze_ownership(&graph, &OwnershipConfig::default());

    assert_eq!(report.status, AnalysisStatus::Complete);
    assert!(
        contains_code(
            &report,
            OwnershipDiagnosticCode::PossiblyUnavailable
        ),
        "a value moved along only one incoming path must not be treated as definitely available"
    );
}

// -----------------------------------------------------------------------------
// Explicit operational budgets and incomplete results
// -----------------------------------------------------------------------------

#[test]
fn exhausted_dataflow_budget_never_reports_validity() {
    let place = root_place(1);

    let graph = single_block_graph(
        vec![declare(
            place,
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        )],
        root_scope(),
    );

    let config = OwnershipConfig {
        max_dataflow_steps: Some(0),
        max_diagnostics: None,
    };

    let report = analyze_ownership(&graph, &config);

    assert_eq!(report.status, AnalysisStatus::Incomplete);
    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::AnalysisBudgetExceeded
    ));
    assert!(!report.is_valid());
}

#[test]
fn exhausted_diagnostic_budget_is_explicit() {
    let place = root_place(1);

    let graph = single_block_graph(
        vec![
            declare(
                place.clone(),
                OwnershipMode::Unrestricted,
                ScopeId(0),
                false,
            ),
            read(place),
        ],
        root_scope(),
    );

    let config = OwnershipConfig {
        max_dataflow_steps: None,
        max_diagnostics: Some(0),
    };

    let report = analyze_ownership(&graph, &config);

    assert_eq!(report.status, AnalysisStatus::Incomplete);
    assert!(contains_code(
        &report,
        OwnershipDiagnosticCode::AnalysisBudgetExceeded
    ));
    assert!(!report.is_valid());
}

// -----------------------------------------------------------------------------
// Determinism and extensibility properties
// -----------------------------------------------------------------------------

#[test]
fn repeated_analysis_produces_stably_ordered_diagnostics() {
    let place = root_place(1);

    let graph = single_block_graph(
        vec![
            declare(
                place.clone(),
                OwnershipMode::Unrestricted,
                ScopeId(0),
                false,
            ),
            read(place.clone()),
            move_place(place),
        ],
        root_scope(),
    );

    let first = analyze_ownership(&graph, &OwnershipConfig::default());
    let second = analyze_ownership(&graph, &OwnershipConfig::default());

    let diagnostic_key = |report: &crate::frontend::semantic::types::ownership::OwnershipReport| {
        report
            .diagnostics
            .iter()
            .map(|diagnostic| {
                (
                    diagnostic.block,
                    diagnostic.event_index,
                    diagnostic.code,
                    diagnostic.place.clone(),
                    diagnostic.loan,
                )
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(first.status, second.status);
    assert_eq!(diagnostic_key(&first), diagnostic_key(&second));
}

#[test]
fn ownership_analysis_does_not_require_hardware_metadata() {
    let place = root_place(1);

    let report = analyze(vec![
        declare(
            place.clone(),
            OwnershipMode::Unrestricted,
            ScopeId(0),
            true,
        ),
        read(place),
    ]);

    assert_no_diagnostics(&report);
}