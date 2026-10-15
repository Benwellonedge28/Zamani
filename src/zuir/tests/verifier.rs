#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Verification Integration Tests
//!
//! Path: `src/zuir/tests/verifier.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is prohibited.
//!
//! STATUS
//!
//! Proposed integration-test module. Wire this file into the repository's
//! actual test-module tree before treating it as part of the test suite.
//!
//! PURPOSE
//!
//! Test the public contracts of the canonical ZUIR verification stages:
//! control-flow verification, dominance analysis, resource-intent
//! verification, and full-verification reporting.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/verify/mod.rs`
//! - `src/zuir/verify/full.rs`
//! - `src/zuir/verify/structural.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `src/zuir/verify/dominance.rs`
//! - `src/zuir/verify/types.rs`
//! - `src/zuir/verify/effects.rs`
//! - `src/zuir/verify/ownership.rs`
//! - `src/zuir/verify/resources.rs`
//! - `src/zuir/verify/dialects.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/`
//! - `src/zuir/schema/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! OWNS
//!
//! - Regression tests for verification-stage public contracts.
//! - Empty-input, deterministic-report, and stage-boundary tests.
//! - Checks that empty verification views are handled consistently.
//! - Checks that a clean focused-stage report is not described as proof
//!   of complete program validity.
//!
//! DOES_NOT_OWN
//!
//! - Production verification algorithms.
//! - A parallel IR representation, verifier, diagnostic system, or schema.
//! - Source parsing, AST construction, name resolution, or type inference.
//! - Quantum gate semantics, HDL semantics, or backend-specific legality.
//! - Physical resource allocation, target selection, scheduling, or runtime.
//! - A claim that every SSA, ownership, effect, or dialect invariant is
//!   already implemented by the current verifier pipeline.
//!
//! INTEGRATION CONTRACT
//!
//! These tests call the existing verifier implementations directly.
//! The individual verifiers remain authoritative for their own checks;
//! `verify_full` remains the orchestrator for the currently integrated
//! whole-program verification stages.
//!
//! The canonical source pipeline remains:
//!
//! source -> AST -> semantic model -> ZUIR -> domain IR -> target lowering
//!
//! The grammar defines source syntax and composition. It does not define
//! ZUIR's whole-program verification algorithms. These tests must not parse
//! grammar productions or infer semantic correctness from source syntax.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module introduces no hardware capacities, operation-count limits,
//! fixed graph sizes, qubit limits, tensor-rank limits, or device limits.
//! Tests use dynamically sized collections and do not depend on a specific
//! execution target.
//!
//! No finite machine can process literally infinite input. The production
//! verifier must scale with available resources, avoid artificial universal
//! limits, and report detectable failures without silently truncating input.
//!
//! ERROR AND COMPLETENESS CONTRACT
//!
//! A clean focused-stage report establishes only that the checks performed
//! by that stage found no issues in the supplied view.
//!
//! A successful dominance report is not a proof of complete SSA legality.
//! A successful resource report is not a proof of target feasibility.
//! A successful full-verification report establishes only the properties
//! covered by the stages actually invoked and their supplied context.
//!
//! FREEZE CRITERIA
//!
//! - Every imported type and function exists in the canonical implementation.
//! - The test module is declared by the actual repository test tree.
//! - Focused-stage behavior and deterministic reporting are tested.
//! - Tests do not depend on private implementation fields.
//! - Relevant unit and integration tests pass.
//! - Formatting, compilation, and Clippy pass.
//! - No unsafe code or fixed hardware-size assumptions are introduced.

// =============================================================================
// Canonical verifier APIs
// =============================================================================

use crate::zuir::verify::control_flow::{
    verify_control_flow,
    ControlFlowInput,
};

use crate::zuir::verify::dominance::{
    verify_dominance,
    DominanceInput,
};

use crate::zuir::verify::full::{
    FullVerificationReport,
    VerificationStatus,
};

use crate::zuir::verify::resources::{
    verify_resources,
};

// =============================================================================
// Empty control-flow views
// =============================================================================

/// An empty control-flow view must be accepted without inventing blocks,
/// regions, entry points, or successor edges.
#[test]
fn control_flow_verifier_accepts_an_empty_view() {
    let regions = [];
    let blocks = [];

    let report = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert!(
        report.is_valid(),
        "an empty control-flow view must not produce spurious errors: {:?}",
        report.issues()
    );

    assert_eq!(report.issue_count(), 0);
    assert_eq!(report.regions_checked(), 0);
    assert_eq!(report.blocks_checked(), 0);
    assert_eq!(report.edges_checked(), 0);
}

/// Repeated verification of an unchanged view must produce the same report.
#[test]
fn control_flow_verification_is_deterministic_for_identical_inputs() {
    let regions = [];
    let blocks = [];

    let first = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    let second = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert_eq!(first, second);
}

// =============================================================================
// Empty dominance views
// =============================================================================

/// Dominance analysis must handle an empty graph without panicking or
/// fabricating a region entry block.
#[test]
fn dominance_verifier_accepts_an_empty_graph() {
    let regions = [];
    let blocks = [];

    let report = verify_dominance(&DominanceInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert!(
        report.is_valid(),
        "an empty graph must not produce spurious dominance errors: {:?}",
        report.issues()
    );

    assert_eq!(report.issue_count(), 0);
    assert_eq!(report.regions_checked(), 0);
    assert_eq!(report.blocks_checked(), 0);
    assert_eq!(report.edges_checked(), 0);
}

/// Dominance results for the same immutable graph must be reproducible.
#[test]
fn dominance_analysis_is_deterministic_for_identical_inputs() {
    let regions = [];
    let blocks = [];

    let first = verify_dominance(&DominanceInput {
        regions: &regions,
        blocks: &blocks,
    });

    let second = verify_dominance(&DominanceInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert_eq!(first, second);
}

// =============================================================================
// Resource-intent verification
// =============================================================================

/// No resource declarations means there are no contradictory declarations
/// to report. It does not mean a physical target has been selected or that
/// the program can execute on every target.
#[test]
fn resource_verifier_accepts_an_empty_intent_set() {
    let intents = [];

    let report = verify_resources(&intents);

    assert!(
        report.is_valid(),
        "an empty resource-intent set must not produce spurious errors: {:?}",
        report.issues()
    );

    assert!(
        report.is_complete(),
        "no resource checks should be deferred when there are no intents"
    );

    assert_eq!(report.intents_checked(), 0);
    assert_eq!(report.assertions_checked(), 0);
    assert_eq!(report.capabilities_checked(), 0);
    assert_eq!(report.topology_relations_checked(), 0);
}

/// Resource verification must return the same ordered findings for an
/// unchanged input. This protects deterministic downstream diagnostics.
#[test]
fn resource_verification_is_deterministic_for_identical_inputs() {
    let intents = [];

    let first = verify_resources(&intents);
    let second = verify_resources(&intents);

    assert_eq!(first, second);
}

// =============================================================================
// Full-verification report contract
// =============================================================================

/// A newly initialized report has no findings and no completed stages.
/// This tests the report data contract, not the validity of a real program.
#[test]
fn default_full_verification_report_has_no_findings_or_completed_stages() {
    let report = FullVerificationReport::default();

    assert_eq!(report.status(), VerificationStatus::Valid);
    assert!(report.is_valid());
    assert!(!report.has_errors());
    assert!(!report.is_incomplete());
    assert!(report.issues().is_empty());
    assert!(report.stages_completed().is_empty());
}

/// Full-verification status must be consumed through its public API rather
/// than inferred by parsing human-readable diagnostic messages.
#[test]
fn full_verification_report_exposes_stable_machine_readable_findings() {
    let report = FullVerificationReport::default();

    for issue in report.issues() {
        assert!(
            !issue.code().is_empty(),
            "each finding must expose a machine-readable code"
        );
    }

    assert_eq!(report.count(
        crate::zuir::verify::full::VerificationSeverity::Error
    ), 0);

    assert_eq!(report.count(
        crate::zuir::verify::full::VerificationSeverity::Warning
    ), 0);

    assert_eq!(report.count(
        crate::zuir::verify::full::VerificationSeverity::Deferred
    ), 0);
}

// =============================================================================
// Verification boundary regressions
// =============================================================================

/// A focused verifier's clean result must not be used as a substitute for
/// running the full verification pipeline over the complete IR snapshot.
///
/// This test deliberately verifies an empty CFG only. It does not claim
/// that any real program has passed structural, type, effect, ownership,
/// resource, dialect, or SSA verification.
#[test]
fn focused_verification_is_scoped_to_the_supplied_view() {
    let regions = [];
    let blocks = [];

    let control_flow = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    let dominance = verify_dominance(&DominanceInput {
        regions: &regions,
        blocks: &blocks,
    });

    assert!(control_flow.is_valid());
    assert!(dominance.is_valid());

    assert_eq!(control_flow.regions_checked(), regions.len());
    assert_eq!(control_flow.blocks_checked(), blocks.len());
    assert_eq!(dominance.regions_checked(), regions.len());
    assert_eq!(dominance.blocks_checked(), blocks.len());
}

/// Repeating all currently covered empty-view checks must not introduce
/// global-state dependence or vary according to unrelated host hardware.
#[test]
fn empty_view_verification_has_no_hidden_target_dependency() {
    let regions = [];
    let blocks = [];
    let intents = [];

    let control_flow = verify_control_flow(&ControlFlowInput {
        regions: &regions,
        blocks: &blocks,
    });

    let dominance = verify_dominance(&DominanceInput {
        regions: &regions,
        blocks: &blocks,
    });

    let resources = verify_resources(&intents);

    assert!(control_flow.is_valid());
    assert!(dominance.is_valid());
    assert!(resources.is_valid());

    assert_eq!(control_flow.issue_count(), 0);
    assert_eq!(dominance.issue_count(), 0);
    assert_eq!(resources.error_count(), 0);
}