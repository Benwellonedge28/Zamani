
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Public API contract tests for Zamani semantic analysis.
//!
//! File:
//!     src/frontend/semantic/tests/api.rs
//!
//! Rust edition:
//!     2021
//!
//! Minimum supported Rust version:
//!     1.97
//!
//! Safety:
//!     Unsafe Rust is prohibited.
//!
//! # Purpose
//!
//! Verify the public semantic-analysis API independently of the concrete
//! type checker, symbol resolver, domain validators, or compiler backend.
//!
//! These tests establish the contract that implementation modules must
//! satisfy before they can be integrated into the native frontend pipeline.
//!
//! # Ownership
//!
//! This file owns only API-level tests and their test doubles.
//!
//! It does not own:
//! - semantic-analysis implementation;
//! - the canonical AST;
//! - semantic configuration definitions;
//! - semantic diagnostic definitions;
//! - the semantic result model;
//! - name resolution or type inference;
//! - effect, capability, resource, or contract checking;
//! - domain-specific semantic rules;
//! - target selection or hardware feasibility;
//! - IR generation or runtime execution.
//!
//! # Integration contract
//!
//! The public API under test is expected to expose:
//!
//! - `api::SemanticAnalyzer`;
//! - `api::AnalysisResult<T>`;
//! - `api::AnalysisStatus`;
//! - `config::SemanticConfig`.
//!
//! The API accepts the analyzer's canonical AST input through an associated
//! type, avoiding a second AST definition and avoiding an assumption that
//! every frontend entry point uses the same concrete AST wrapper.
//!
//! `SemanticConfig` is owned by `config.rs`.
//!
//! Structured semantic diagnostics are owned by the diagnostics subsystem.
//!
//! The semantic program/model is owned by the semantic model subsystem.
//!
//! # Architectural invariants
//!
//! - API tests must not depend on the legacy `crate::semantic` analyzer.
//! - API tests must not construct a competing AST.
//! - API tests must not require a specific processor, accelerator, device,
//!   quantum processor, topology, or execution provider.
//! - Semantic validity must remain separate from target feasibility.
//! - Incomplete or failed analysis must never be reported as valid.
//! - Identical input and configuration must produce equivalent outcomes
//!   for a deterministic analyzer implementation.
//! - Operational budgets must not become language-level hardware limits.
//!
//! # Completion criteria
//!
//! This file is complete when:
//!
//! 1. Every test passes against the canonical semantic API.
//! 2. All result-state transitions have unambiguous contracts.
//! 3. A semantic API implementation can be substituted without changing
//!    these tests merely because its internal architecture changes.
//! 4. The tests do not import implementation-private analyzer modules.
//! 5. The tests compile with the repository's supported Rust toolchain.
//!
//! NOTE:
//! The API and configuration contracts referenced below must be implemented
//! by their owning modules before this test module can compile.

use crate::frontend::semantic::api::{
    AnalysisResult,
    AnalysisStatus,
    SemanticAnalyzer,
};
use crate::frontend::semantic::config::SemanticConfig;

// =============================================================================
// Test-only input and output types
// =============================================================================

/// Small test input representing a semantic-analysis request.
///
/// This is deliberately NOT a Zamani AST. It is a test double used to verify
/// the public service boundary without inventing or duplicating AST nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestInput {
    source_identity: &'static str,
    outcome: RequestedOutcome,
}

/// Outcome requested from the mock analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestedOutcome {
    Complete,
    Invalid,
    Incomplete,
    Failed,
}

/// Test-only semantic model.
///
/// A real implementation must return the canonical semantic model, not this
/// test type.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestModel {
    semantic_identity: &'static str,
}

/// Mock semantic analyzer used to exercise the public contract.
///
/// This type performs no actual semantic analysis. Its only responsibility
/// is to prove that callers can invoke the API and inspect the result without
/// coupling themselves to a concrete analyzer implementation.
#[derive(Debug, Default, Clone, Copy)]
struct MockSemanticAnalyzer;

impl SemanticAnalyzer for MockSemanticAnalyzer {
    type Ast = TestInput;
    type Model = TestModel;

    fn analyze(
        &self,
        ast: &Self::Ast,
        _config: &SemanticConfig,
    ) -> AnalysisResult<Self::Model> {
        match ast.outcome {
            RequestedOutcome::Complete => AnalysisResult::complete(TestModel {
                semantic_identity: ast.source_identity,
            }),

            RequestedOutcome::Invalid => AnalysisResult::invalid(
                None,
                Vec::new(),
            ),

            RequestedOutcome::Incomplete => AnalysisResult::incomplete(
                None,
                Vec::new(),
            ),

            RequestedOutcome::Failed => {
                AnalysisResult::failed(Vec::new())
            }
        }
    }
}

// =============================================================================
// Test helpers
// =============================================================================

fn input(outcome: RequestedOutcome) -> TestInput {
    TestInput {
        source_identity: "test-program",
        outcome,
    }
}

fn config() -> SemanticConfig {
    SemanticConfig::default()
}

// =============================================================================
// Public API and result contracts
// =============================================================================

#[test]
fn public_analyzer_accepts_its_declared_ast_type() {
    let analyzer = MockSemanticAnalyzer;
    let ast = input(RequestedOutcome::Complete);
    let configuration = config();

    let result = analyzer.analyze(&ast, &configuration);

    assert_eq!(result.status(), AnalysisStatus::Complete);
    assert!(result.is_valid());
    assert!(result.is_complete());
    assert!(!result.is_incomplete());
    assert!(!result.is_failed());

    let model = result
        .value()
        .expect("complete analysis must contain a semantic model");

    assert_eq!(
        model.semantic_identity,
        "test-program",
    );
}

#[test]
fn complete_analysis_contains_a_model() {
    let result = AnalysisResult::complete(TestModel {
        semantic_identity: "complete-program",
    });

    assert_eq!(result.status(), AnalysisStatus::Complete);
    assert!(result.is_complete());
    assert!(result.is_valid());

    assert_eq!(
        result.value().map(|model| model.semantic_identity),
        Some("complete-program"),
    );
}

#[test]
fn invalid_analysis_is_never_reported_as_valid() {
    let result: AnalysisResult<TestModel> =
        AnalysisResult::invalid(None, Vec::new());

    assert_eq!(result.status(), AnalysisStatus::Invalid);
    assert!(!result.is_valid());
    assert!(!result.is_complete());
    assert!(!result.is_incomplete());
    assert!(!result.is_failed());

    assert!(result.value().is_none());
}

#[test]
fn incomplete_analysis_is_distinct_from_invalid_analysis() {
    let incomplete: AnalysisResult<TestModel> =
        AnalysisResult::incomplete(None, Vec::new());

    let invalid: AnalysisResult<TestModel> =
        AnalysisResult::invalid(None, Vec::new());

    assert_eq!(incomplete.status(), AnalysisStatus::Incomplete);
    assert_eq!(invalid.status(), AnalysisStatus::Invalid);

    assert!(incomplete.is_incomplete());
    assert!(!incomplete.is_valid());

    assert!(!invalid.is_incomplete());
    assert!(!invalid.is_valid());
}

#[test]
fn failed_analysis_is_distinct_from_incomplete_analysis() {
    let incomplete: AnalysisResult<TestModel> =
        AnalysisResult::incomplete(None, Vec::new());

    let failed: AnalysisResult<TestModel> =
        AnalysisResult::failed(Vec::new());

    assert_eq!(incomplete.status(), AnalysisStatus::Incomplete);
    assert_eq!(failed.status(), AnalysisStatus::Failed);

    assert!(incomplete.is_incomplete());
    assert!(failed.is_failed());

    assert!(!incomplete.is_valid());
    assert!(!failed.is_valid());
}

#[test]
fn failed_analysis_does_not_expose_a_valid_model() {
    let result: AnalysisResult<TestModel> =
        AnalysisResult::failed(Vec::new());

    assert_eq!(result.status(), AnalysisStatus::Failed);
    assert!(result.is_failed());
    assert!(!result.is_valid());
    assert!(result.value().is_none());
}

#[test]
fn analyzer_dispatches_each_outcome_without_conflating_states() {
    let analyzer = MockSemanticAnalyzer;
    let configuration = config();

    let complete = analyzer.analyze(
        &input(RequestedOutcome::Complete),
        &configuration,
    );

    let invalid = analyzer.analyze(
        &input(RequestedOutcome::Invalid),
        &configuration,
    );

    let incomplete = analyzer.analyze(
        &input(RequestedOutcome::Incomplete),
        &configuration,
    );

    let failed = analyzer.analyze(
        &input(RequestedOutcome::Failed),
        &configuration,
    );

    assert_eq!(complete.status(), AnalysisStatus::Complete);
    assert_eq!(invalid.status(), AnalysisStatus::Invalid);
    assert_eq!(incomplete.status(), AnalysisStatus::Incomplete);
    assert_eq!(failed.status(), AnalysisStatus::Failed);

    assert!(complete.is_valid());
    assert!(!invalid.is_valid());
    assert!(!incomplete.is_valid());
    assert!(!failed.is_valid());
}

// =============================================================================
// Determinism and invocation isolation
// =============================================================================

#[test]
fn identical_requests_produce_equivalent_results() {
    let analyzer = MockSemanticAnalyzer;
    let ast = input(RequestedOutcome::Complete);
    let configuration = config();

    let first = analyzer.analyze(&ast, &configuration);
    let second = analyzer.analyze(&ast, &configuration);

    assert_eq!(first.status(), second.status());
    assert_eq!(first.value(), second.value());
    assert_eq!(first.diagnostics(), second.diagnostics());
}

#[test]
fn independent_requests_do_not_share_mutable_result_state() {
    let analyzer = MockSemanticAnalyzer;
    let configuration = config();

    let first = analyzer.analyze(
        &input(RequestedOutcome::Complete),
        &configuration,
    );

    let second = analyzer.analyze(
        &input(RequestedOutcome::Invalid),
        &configuration,
    );

    assert_eq!(first.status(), AnalysisStatus::Complete);
    assert_eq!(second.status(), AnalysisStatus::Invalid);

    assert!(first.value().is_some());
    assert!(second.value().is_none());
}

#[test]
fn analyzer_can_be_reused_after_a_failed_request() {
    let analyzer = MockSemanticAnalyzer;
    let configuration = config();

    let failed = analyzer.analyze(
        &input(RequestedOutcome::Failed),
        &configuration,
    );

    let completed = analyzer.analyze(
        &input(RequestedOutcome::Complete),
        &configuration,
    );

    assert!(failed.is_failed());
    assert!(completed.is_valid());

    assert_eq!(
        completed.value().map(|model| model.semantic_identity),
        Some("test-program"),
    );
}

// =============================================================================
// Portable semantic boundary
// =============================================================================

#[test]
fn semantic_api_does_not_require_a_target_argument() {
    // The public service accepts source-level input and semantic
    // configuration only. Target selection belongs to downstream
    // feasibility analysis and realization.
    //
    // This compile-time call verifies the intended boundary: an analyzer
    // can process a semantic input without a device or target description.
    let analyzer = MockSemanticAnalyzer;
    let ast = input(RequestedOutcome::Complete);
    let configuration = config();

    let result = analyzer.analyze(&ast, &configuration);

    assert!(result.is_valid());
}

#[test]
fn semantic_validity_is_not_inferred_from_hardware_availability() {
    // A complete semantic result must be representable without attaching
    // a target, device, processor count, qubit count, topology, or runtime
    // handle to the semantic model.
    //
    // Whether a target can realize the program is a separate downstream
    // question and must not be silently folded into this API's status.
    let result = AnalysisResult::complete(TestModel {
        semantic_identity: "portable-program",
    });

    assert!(result.is_valid());
    assert_eq!(result.status(), AnalysisStatus::Complete);
}

// =============================================================================
// Scalability and architectural regression guards
// =============================================================================

#[test]
fn api_does_not_require_a_fixed_hardware_capacity() {
    // The API accepts an abstract AST input. It must not require a
    // processor count, accelerator count, qubit count, register width,
    // tensor rank, memory capacity, or node count merely to perform
    // semantic analysis.
    //
    // This is an architectural regression guard, not a claim that a finite
    // compiler process can analyze literally infinite input.
    let analyzer = MockSemanticAnalyzer;
    let ast = input(RequestedOutcome::Complete);
    let configuration = config();

    let result = analyzer.analyze(&ast, &configuration);

    assert!(result.is_valid());
}

#[test]
fn api_result_diagnostics_are_available_without_string_parsing() {
    // Diagnostics must be exposed as structured records by the public
    // result contract. Callers must not need to parse rendered messages
    // to determine whether diagnostics were retained.
    let result = AnalysisResult::complete(TestModel {
        semantic_identity: "diagnostic-contract",
    });

    let diagnostics = result.diagnostics();

    assert!(diagnostics.is_empty());
}

#[test]
fn complete_result_is_independent_of_call_order() {
    let analyzer = MockSemanticAnalyzer;
    let configuration = config();

    let first_input = input(RequestedOutcome::Complete);
    let second_input = input(RequestedOutcome::Incomplete);

    let first_before = analyzer.analyze(
        &first_input,
        &configuration,
    );

    let _intervening = analyzer.analyze(
        &second_input,
        &configuration,
    );

    let first_after = analyzer.analyze(
        &first_input,
        &configuration,
    );

    assert_eq!(first_before.status(), first_after.status());
    assert_eq!(first_before.value(), first_after.value());
    assert_eq!(
        first_before.diagnostics(),
        first_after.diagnostics(),
    );
}

// =============================================================================
// Test-contract summary
// =============================================================================

#[test]
fn api_contract_keeps_semantic_analysis_separate_from_lowering() {
    // The API's output is a semantic model, not a target-specific IR.
    //
    // A concrete implementation must not change this contract to return
    // physical allocations, scheduled instructions, backend handles, or
    // target-specific code as the semantic-analysis result.
    let result = AnalysisResult::complete(TestModel {
        semantic_identity: "semantic-model",
    });

    assert!(result.is_valid());

    assert_eq!(
        result.value().map(|model| model.semantic_identity),
        Some("semantic-model"),
    );
}
