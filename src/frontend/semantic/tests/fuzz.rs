#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Semantic-analysis fuzz and robustness tests for Zamani.
//!
//! File:
//!     src/frontend/semantic/tests/fuzz.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # PURPOSE
//!
//! Exercise the canonical AST program-root validation contract, semantic-result
//! validity gates, deterministic generated inputs, numeric boundary handling,
//! and operational work-budget contracts.
//!
//! These tests deliberately use existing repository interfaces. They do not
//! implement a second parser, AST, semantic model, or intermediate
//! representation.
//!
//! # AUTHORITY
//!
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/semantic-boundaries.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - src/frontend/ast/node/program/
//! - src/frontend/ast/node/node_id.rs
//! - src/frontend/ast/node/source/span.rs
//! - src/frontend/semantic/api.rs
//! - src/frontend/semantic/config.rs
//!
//! Normative language and AST contracts take precedence over assumptions
//! introduced by a test.
//!
//! # OWNS
//!
//! - Deterministic test-input generation.
//! - AST program-root robustness tests.
//! - Semantic-result lifecycle and validity-gate property tests.
//! - Operational-budget boundary tests.
//!
//! # DOES_NOT_OWN
//!
//! - Source parsing or lexical recognition.
//! - Semantic analysis implementation.
//! - Name resolution, type inference, or domain-specific validation.
//! - Production AST or semantic-model definitions.
//! - Diagnostic production or rendering.
//! - Target feasibility, allocation, scheduling, or lowering.
//!
//! # INTEGRATION CONTRACT
//!
//! This file uses the canonical `Program`, `NodeId`, `Span`, `NodeMetadata`,
//! `SemanticResult`, `SemanticStatus`, `SemanticDiagnostic`,
//! `SemanticModelState`, `WorkBudget`, and `SemanticLimits` APIs.
//!
//! Register it exactly once in the semantic test module:
//!
//!     #[path = "fuzz.rs"]
//!     mod fuzz;
//!
//! The parent module must itself be wired into the crate's test build.
//!
//! # IMPORTANT COVERAGE LIMIT
//!
//! These tests do not claim to fuzz source-to-AST parsing or a production
//! semantic-analysis pipeline. Such end-to-end fuzzing requires a concrete,
//! integrated ParserService and semantic service implementation, plus the
//! canonical AST storage/provider needed to resolve child NodeIds.
//!
//! Once those services are available, add a separate integration harness that
//! feeds generated source and AST graphs into the actual implementations.
//! Do not replace the production implementations with test doubles and then
//! claim end-to-end coverage.
//!
//! # SCALABILITY
//!
//! Test workload sizes below are finite so routine test runs are predictable.
//! They are not language-level ceilings, hardware-capacity restrictions, or
//! limits on the number of declarations, dimensions, resources, or domains a
//! Zamani program may express.
//!
//! "Unlimited" means no explicit policy budget; it does not promise infinite
//! memory, execution time, stack space, or address space.
//!
//! # SAFETY
//!
//! Safe Rust only. No unsafe blocks, external fuzzing dependency, global
//! mutable state, environmental input, filesystem access, or randomness.
//!
//! # FREEZE CRITERIA
//!
//! This file may be frozen when:
//!
//! - it is registered exactly once;
//! - it compiles against the canonical AST and semantic API;
//! - generated AST roots validate deterministically;
//! - invalid construction and boundary cases fail safely;
//! - semantic-result validity gates never certify incomplete, invalid,
//!   aborted, failed, or diagnostic-incomplete results;
//! - work-budget boundaries behave consistently;
//! - the workspace test suite passes on the supported Rust toolchain.
//!
//! Passing this file alone does not certify the semantic pipeline or the
//! compiler as a whole.

use core::num::NonZeroUsize;

use crate::frontend::ast::node::metadata::NodeMetadata;
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::program::Program;
use crate::frontend::ast::node::source::span::{SourceId, SourceOffset, Span};

use crate::frontend::semantic::api::{
    SemanticDiagnostic,
    SemanticModelState,
    SemanticResult,
    SemanticStatus,
};
use crate::frontend::semantic::config::{
    SemanticLimits,
    WorkBudget,
};

// ============================================================================
// Deterministic input generation
// ============================================================================

/// A small deterministic generator for repeatable robustness tests.
///
/// This generator is deliberately local to a test invocation. It does not
/// use process-global randomness, wall-clock time, or environment state.
///
/// Wrapping arithmetic is used only for generator state evolution. Generated
/// AST identities and source-coordinate arithmetic use checked operations.
#[derive(Debug, Clone, Copy)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    /// Create a generator with an explicit reproducible seed.
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Produce the next deterministic pseudo-random value.
    fn next_u64(&mut self) -> u64 {
        // Xorshift state evolution intentionally wraps within u64.
        let mut value = self.state;
        value ^= value.wrapping_shl(13);
        value ^= value.wrapping_shr(7);
        value ^= value.wrapping_shl(17);
        self.state = value;
        value
    }

    /// Produce a value in the inclusive interval [0, upper].
    ///
    /// Modulo bias is acceptable here because these are robustness inputs,
    /// not cryptographic values or statistical benchmarks.
    fn bounded_u64(&mut self, upper: u64) -> u64 {
        if upper == u64::MAX {
            self.next_u64()
        } else {
            self.next_u64() % upper.saturating_add(1)
        }
    }
}

// ============================================================================
// Test-only semantic API contract types
// ============================================================================

/// Test representation of the model's lifecycle assertions.
///
/// This type is not a production semantic model. It allows the generic
/// SemanticResult validity gates to be tested independently of the current
/// semantic pipeline implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProbeModel {
    complete: bool,
    valid: bool,
}

impl SemanticModelState for ProbeModel {
    fn establishes_completion(&self) -> bool {
        self.complete
    }

    fn establishes_validity(&self) -> bool {
        self.valid
    }
}

/// Test diagnostic that identifies whether a result contains an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProbeDiagnostic {
    error: bool,
}

impl SemanticDiagnostic for ProbeDiagnostic {
    fn is_error(&self) -> bool {
        self.error
    }
}

fn complete_valid_model() -> ProbeModel {
    ProbeModel {
        complete: true,
        valid: true,
    }
}

fn result_for(
    status: SemanticStatus,
    model: Option<ProbeModel>,
    diagnostics: Vec<ProbeDiagnostic>,
    diagnostics_complete: bool,
) -> SemanticResult<ProbeModel, ProbeDiagnostic> {
    SemanticResult::from_parts_with_diagnostic_completeness(
        status,
        model,
        diagnostics,
        diagnostics_complete,
    )
}

// ============================================================================
// Canonical AST root helpers
// ============================================================================

/// Construct a valid half-open source span.
///
/// Offsets are UTF-8 byte coordinates. This helper does not claim that the
/// coordinates correspond to a particular source file's contents; that
/// validation belongs to the source-map layer.
fn make_span(source: u64, start: u64, end: u64) -> Span {
    Span::new(
        SourceId::from_raw(source),
        SourceOffset::from_raw(start),
        SourceOffset::from_raw(end),
    )
    .expect("generated span must have start <= end")
}

/// Construct an empty canonical program root with an explicit identity.
fn empty_program(raw_id: u64, span: Span) -> Program {
    let id = NodeId::new(raw_id)
        .expect("generated root identity must be nonzero");

    Program::new(id, span, NodeMetadata::default())
}

/// Return a positive NodeId, rejecting zero by construction.
fn node_id(raw_id: u64) -> NodeId {
    NodeId::new(raw_id).expect("generated node identity must be nonzero")
}

// ============================================================================
// Determinism and AST construction robustness
// ============================================================================

#[test]
fn generated_empty_program_roots_validate() {
    let mut generator = DeterministicGenerator::new(0x5A4D_414E_495F_4153);

    // The count is a test workload, not a language-level limit.
    for case_index in 0_u64..512 {
        let raw_id = case_index
            .checked_add(1)
            .expect("test root ID arithmetic must not overflow");

        let first = generator.bounded_u64(u64::MAX / 2);
        let second = generator.bounded_u64(u64::MAX / 2);

        let start = first.min(second);
        let end = first.max(second);

        let source = case_index
            .checked_add(1)
            .expect("test source identity arithmetic must not overflow");

        let span = make_span(source, start, end);
        let program = empty_program(raw_id, span);

        assert!(program.is_empty());
        assert_eq!(program.child_count(), 0);
        assert_eq!(program.validate(), Ok(()));
    }
}

#[test]
fn generated_programs_preserve_child_order_and_validate() {
    let mut generator = DeterministicGenerator::new(0x504F_434F_5245_4146);

    // Each iteration builds a finite graph fragment. No value in this test
    // represents a maximum AST size or a maximum hardware capacity.
    for case_index in 0_u64..256 {
        let root_raw = case_index
            .checked_mul(128)
            .and_then(|value| value.checked_add(1))
            .expect("test root identity must not overflow");

        let root_id = node_id(root_raw);
        let span = make_span(case_index, 0, generator.bounded_u64(1_000_000));
        let mut program = Program::new(
            root_id,
            span,
            NodeMetadata::default(),
        );

        let child_count = generator.bounded_u64(48) as usize;

        for child_index in 0..child_count {
            let offset = u64::try_from(child_index)
                .expect("test child index must fit u64")
                .checked_add(1)
                .expect("test child offset must not overflow");

            let child_raw = root_raw
                .checked_add(offset)
                .expect("test child identity must not overflow");

            let child_id = node_id(child_raw);

            assert!(
                program.push_child(child_id).is_ok(),
                "unique non-root child IDs should be accepted"
            );
        }

        assert_eq!(program.child_count(), child_count);
        assert!(program.validate().is_ok());

        for (index, child) in program.iter_children().enumerate() {
            let expected_offset = u64::try_from(index)
                .expect("test child index must fit u64")
                .checked_add(1)
                .expect("test child offset must not overflow");

            assert_eq!(
                child.as_u64(),
                root_raw
                    .checked_add(expected_offset)
                    .expect("test child identity must not overflow")
            );
        }
    }
}

#[test]
fn duplicate_child_ids_are_rejected_without_corrupting_the_program() {
    let root = node_id(1);
    let child = node_id(2);

    let mut program = Program::new(
        root,
        make_span(1, 0, 10),
        NodeMetadata::default(),
    );

    assert!(program.push_child(child).is_ok());
    assert!(program.push_child(child).is_err());

    assert_eq!(program.child_count(), 1);
    assert_eq!(program.first_child(), Some(child));
    assert_eq!(program.last_child(), Some(child));
    assert!(program.validate().is_ok());
}

#[test]
fn root_self_reference_is_rejected_without_corrupting_the_program() {
    let root = node_id(42);

    let mut program = Program::new(
        root,
        make_span(2, 0, 0),
        NodeMetadata::default(),
    );

    assert!(program.push_child(root).is_err());

    assert!(program.is_empty());
    assert!(program.validate().is_ok());
}

#[test]
fn child_queries_are_safe_for_empty_and_out_of_range_indices() {
    let program = empty_program(1, make_span(1, 0, 0));

    assert_eq!(program.child(0), None);
    assert_eq!(program.first_child(), None);
    assert_eq!(program.last_child(), None);
    assert!(!program.contains_child(node_id(2)));
    assert_eq!(program.iter_children().count(), 0);
    assert_eq!(program.iter_children_reversed().count(), 0);
}

#[test]
fn program_root_identity_and_span_boundary_values_are_representable() {
    // These are representational boundary tests, not promises that every
    // downstream compiler component can allocate arbitrary-size inputs.
    let root = node_id(u64::MAX);
    let span = Span::point(
        SourceId::from_raw(u64::MAX),
        SourceOffset::from_raw(u64::MAX),
    );

    let program = Program::new(root, span, NodeMetadata::default());

    assert_eq!(program.id(), root);
    assert_eq!(program.span(), span);
    assert!(program.validate().is_ok());
}

#[test]
fn reversed_spans_are_rejected() {
    let result = Span::new(
        SourceId::from_raw(1),
        SourceOffset::from_raw(u64::MAX),
        SourceOffset::from_raw(0),
    );

    assert!(result.is_err());
}

// ============================================================================
// Semantic-result validity gates
// ============================================================================

#[test]
fn complete_valid_results_are_certified_only_when_all_gates_pass() {
    let result = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        Vec::new(),
        true,
    );

    assert!(result.is_complete());
    assert!(result.is_valid());
    assert!(!result.has_errors());
    assert!(result.diagnostics_complete());
}

#[test]
fn incomplete_analysis_is_never_certified_valid() {
    let statuses = [
        SemanticStatus::Incomplete,
        SemanticStatus::Aborted,
        SemanticStatus::Failed,
    ];

    for status in statuses {
        let result = result_for(
            status,
            Some(complete_valid_model()),
            Vec::new(),
            true,
        );

        assert!(!result.is_valid());
        assert!(!result.is_complete());
    }
}

#[test]
fn absent_models_are_never_certified_valid() {
    let result = result_for(
        SemanticStatus::Complete,
        None,
        Vec::new(),
        true,
    );

    assert!(!result.has_model());
    assert!(!result.is_valid());
}

#[test]
fn incomplete_or_invalid_models_are_never_certified_valid() {
    let model_states = [
        ProbeModel {
            complete: false,
            valid: false,
        },
        ProbeModel {
            complete: false,
            valid: true,
        },
        ProbeModel {
            complete: true,
            valid: false,
        },
    ];

    for model in model_states {
        let result = result_for(
            SemanticStatus::Complete,
            Some(model),
            Vec::new(),
            true,
        );

        assert!(!result.is_valid());
    }
}

#[test]
fn errors_prevent_validity_even_if_analysis_completed() {
    let error_cases = [
        vec![ProbeDiagnostic { error: true }],
        vec![
            ProbeDiagnostic { error: false },
            ProbeDiagnostic { error: true },
        ],
    ];

    for diagnostics in error_cases {
        let result = result_for(
            SemanticStatus::Complete,
            Some(complete_valid_model()),
            diagnostics,
            true,
        );

        assert!(result.is_complete());
        assert!(result.has_errors());
        assert!(!result.is_valid());
    }
}

#[test]
fn incomplete_diagnostic_collections_prevent_validity() {
    let result = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        Vec::new(),
        false,
    );

    assert!(result.is_complete());
    assert!(!result.diagnostics_complete());
    assert!(!result.is_valid());
}

#[test]
fn generated_result_combinations_obey_the_validity_truth_table() {
    let statuses = [
        SemanticStatus::Complete,
        SemanticStatus::Incomplete,
        SemanticStatus::Aborted,
        SemanticStatus::Failed,
    ];

    let model_states = [
        None,
        Some(ProbeModel {
            complete: false,
            valid: false,
        }),
        Some(ProbeModel {
            complete: false,
            valid: true,
        }),
        Some(ProbeModel {
            complete: true,
            valid: false,
        }),
        Some(ProbeModel {
            complete: true,
            valid: true,
        }),
    ];

    for status in statuses {
        for model in model_states {
            for has_error in [false, true] {
                for diagnostics_complete in [false, true] {
                    let diagnostics = if has_error {
                        vec![ProbeDiagnostic { error: true }]
                    } else {
                        Vec::new()
                    };

                    let expected_validity = status == SemanticStatus::Complete
                        && model.is_some_and(|value| {
                            value.complete && value.valid
                        })
                        && !has_error
                        && diagnostics_complete;

                    let result = result_for(
                        status,
                        model,
                        diagnostics,
                        diagnostics_complete,
                    );

                    assert_eq!(
                        result.is_valid(),
                        expected_validity,
                        "validity gate mismatch: status={status:?}, \
                         model={model:?}, has_error={has_error}, \
                         diagnostics_complete={diagnostics_complete}"
                    );
                }
            }
        }
    }
}

#[test]
fn repeated_result_construction_is_deterministic() {
    let first = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        vec![ProbeDiagnostic { error: false }],
        true,
    );

    let second = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        vec![ProbeDiagnostic { error: false }],
        true,
    );

    assert_eq!(first, second);
    assert_eq!(first.status(), second.status());
    assert_eq!(first.model(), second.model());
    assert_eq!(first.diagnostics(), second.diagnostics());
    assert_eq!(first.is_valid(), second.is_valid());
}

// ============================================================================
// Operational-budget boundary properties
// ============================================================================

#[test]
fn unlimited_work_budgets_do_not_impose_policy_limits() {
    let budget = WorkBudget::unlimited();

    assert!(budget.is_unlimited());
    assert!(budget.limit().is_none());

    for amount in [0, 1, 2, 17, usize::MAX] {
        assert!(budget.permits(amount));
    }
}

#[test]
fn bounded_work_budgets_accept_the_limit_and_reject_excess() {
    let limit = NonZeroUsize::new(17)
        .expect("test limit must be nonzero");
    let budget = WorkBudget::bounded(limit);

    assert!(!budget.is_unlimited());
    assert_eq!(budget.limit(), Some(limit));

    assert!(budget.permits(0));
    assert!(budget.permits(1));
    assert!(budget.permits(17));
    assert!(!budget.permits(18));
    assert!(!budget.permits(usize::MAX));
}

#[test]
fn zero_is_not_silently_converted_into_a_bounded_budget() {
    assert!(WorkBudget::from_usize(0).is_none());
    assert!(WorkBudget::from_usize(1).is_some());
}

#[test]
fn unlimited_semantic_limits_are_explicitly_unlimited() {
    let limits = SemanticLimits::unlimited();

    assert!(limits.total_work().is_unlimited());
    assert!(limits.semantic_entities().is_unlimited());
    assert!(limits.dependency_edges().is_unlimited());
    assert!(limits.recursion_depth().is_unlimited());
    assert!(limits.constraint_steps().is_unlimited());
    assert!(limits.constant_evaluation_steps().is_unlimited());
    assert!(limits.diagnostics().is_unlimited());
    assert!(limits.extension_contributions().is_unlimited());
}

#[test]
fn generated_budget_boundaries_are_consistent() {
    let mut generator = DeterministicGenerator::new(0x4255_4447_4554_5354);

    for _ in 0..512 {
        let raw_limit = generator.bounded_u64(1_000_000);
        let Some(platform_limit) = usize::try_from(raw_limit).ok() else {
            continue;
        };

        // Zero is rejected by the constructor. Positive values must satisfy
        // the exact inclusive-boundary contract.
        let Some(budget) = WorkBudget::from_usize(platform_limit) else {
            assert_eq!(platform_limit, 0);
            continue;
        };

        assert!(budget.permits(platform_limit));

        if let Some(next) = platform_limit.checked_add(1) {
            assert!(!budget.permits(next));
        }

        assert!(!budget.is_unlimited());
    }
}

// ============================================================================
// Regression guard: semantic results must not encode target feasibility
// ============================================================================

#[test]
fn semantic_result_contract_does_not_require_a_target_description() {
    // The semantic result API is parameterized by the semantic model and
    // diagnostic types, not by a target, device, topology, or runtime handle.
    //
    // This test verifies the API-level boundary only. It does not establish
    // that a concrete target can realize a program.
    let result = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        Vec::new(),
        true,
    );

    assert!(result.is_valid());
}

#[test]
fn generated_workload_sizes_are_not_semantic_validity_inputs() {
    // The number of test-generated child IDs is intentionally not fed into
    // SemanticResult validity. Resource quantities and target feasibility
    // must be handled by their owning semantic/realization contracts.
    let result = result_for(
        SemanticStatus::Complete,
        Some(complete_valid_model()),
        Vec::new(),
        true,
    );

    assert!(result.is_valid());
}