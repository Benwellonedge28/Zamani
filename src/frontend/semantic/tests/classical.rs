#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Classical Semantic Analysis — Integration Tests
//!
//! Path: `src/frontend/semantic/tests/classical.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Verify the public classical semantic-validation contract against the
//! canonical frontend AST and authoritative grammar composition.
//!
//! This file tests the existing implementation in
//! `semantic/domains/classical.rs`. It does not implement a second type
//! checker, symbol table, AST, expression grammar, or semantic pipeline.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `grammar/classical/classical.g4`
//! - `grammar/expressions/expressions.g4`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/classical.rs`
//!
//! The grammar owns syntax. The canonical AST owns source structure and node
//! identity. Shared semantic services own name resolution, type checking,
//! effects, capabilities, and other shared rules.
//!
//! ## OWNS
//!
//! - Tests of classical-pass result and completion contracts.
//! - Tests of configurable operational budgets.
//! - Tests of classical expression-rule dispatch.
//! - Tests of deterministic issue records.
//! - Tests that the classical grammar delegates to canonical expressions.
//!
//! ## DOES_NOT_OWN
//!
//! - Production semantic algorithms or production AST adapters.
//! - Canonical AST or grammar definitions.
//! - Whole-program validity certification.
//! - Physical hardware discovery or target feasibility.
//! - Classical IR generation, optimization, or runtime execution.
//!
//! ## INTEGRATION
//!
//! The tests consume `domains::classical` and the canonical expression AST.
//! They deliberately avoid the legacy `crate::semantic` implementation.
//!
//! Include this module from the semantic test module using `mod classical;`.
//! The semantic test module itself must be connected to the crate's test build.
//!
//! ## SCALABILITY
//!
//! No machine-capacity or language-level program-size ceiling is introduced.
//! Tests use dynamically sized collections and exercise operational limits
//! only where the API explicitly defines them.
//!
//! Literal infinity is not a finite execution guarantee: actual work remains
//! bounded by available resources, representational limits, and explicitly
//! configured operational policies.
//!
//! ## FREEZE CRITERIA
//!
//! - All tests compile against the current public APIs.
//! - Classical dispatch, completion, and budget tests pass.
//! - Grammar composition tests pass.
//! - Workspace tests and lint checks pass.
//! - The canonical AST adapter and complete semantic pipeline are tested
//!   separately before the subsystem is declared production-ready.

use crate::frontend::ast::node::expressions::expression::{
    Expression, ExpressionExtension, ExpressionKind, OperatorRef,
};
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;
use crate::frontend::semantic::domains::classical::{
    validate_classical, ClassicalAnalysisFailure, ClassicalCompletion,
    ClassicalIssue, ClassicalIssueKind, ClassicalSemanticContext,
    ClassicalValidation, ClassicalValidationPolicy,
};

use std::cell::Cell;

// Keep the paths relative to this test file. These compile-time inclusions
// verify that the tests are exercising the repository's checked-in grammar,
// rather than a copied or independently maintained grammar definition.
const CLASSICAL_GRAMMAR: &str =
    include_str!("../../../../grammar/classical/classical.g4");
const EXPRESSION_GRAMMAR: &str =
    include_str!("../../../../grammar/expressions/expressions.g4");

// =============================================================================
// Canonical AST test helpers
// =============================================================================

fn node_id(raw: u64) -> NodeId {
    NodeId::new(raw).expect("test AST node IDs must be nonzero")
}

fn identifier_expression(expression_id: u64, identifier_id: u64) -> Expression {
    Expression::without_metadata(
        node_id(expression_id),
        ExpressionKind::Identifier {
            identifier: node_id(identifier_id),
        },
        Span::default(),
    )
}

fn binary_expression(
    expression_id: u64,
    left_id: u64,
    operator_name: &str,
    right_id: u64,
) -> Expression {
    Expression::without_metadata(
        node_id(expression_id),
        ExpressionKind::Binary {
            left: node_id(left_id),
            operator: OperatorRef::native(operator_name)
                .expect("test operator identity must be valid"),
            right: node_id(right_id),
        },
        Span::default(),
    )
}

// =============================================================================
// Test-only shared semantic context
// =============================================================================

/// Small test double for shared semantic services.
///
/// It records calls made by the classical pass and can deliberately return
/// a structured issue for a name reference. It is not a production resolver
/// or type checker.
#[derive(Default)]
struct TestContext {
    name_checks: Cell<usize>,
    operator_checks: Cell<usize>,
    effect_checks: Cell<usize>,
    child_reference_checks: Cell<usize>,
    extension_checks: Cell<usize>,
    reject_names: bool,
}

impl TestContext {
    fn rejecting_names() -> Self {
        Self {
            reject_names: true,
            ..Self::default()
        }
    }

    fn name_check_count(&self) -> usize {
        self.name_checks.get()
    }

    fn operator_check_count(&self) -> usize {
        self.operator_checks.get()
    }

    fn effect_check_count(&self) -> usize {
        self.effect_checks.get()
    }

    fn child_reference_check_count(&self) -> usize {
        self.child_reference_checks.get()
    }

    fn extension_check_count(&self) -> usize {
        self.extension_checks.get()
    }
}

impl ClassicalSemanticContext for TestContext {
    fn check_name(
        &mut self,
        expression: &Expression,
        _identifier: NodeId,
    ) -> Result<Vec<ClassicalIssue>, ClassicalAnalysisFailure> {
        self.name_checks
            .set(self.name_checks.get().saturating_add(1));

        if self.reject_names {
            return Ok(vec![ClassicalIssue::new(
                ClassicalIssueKind::NameResolution,
                expression.id(),
                "test.name.unresolved",
                "The test context deliberately rejects this name.",
            )]);
        }

        Ok(Vec::new())
    }

    fn check_operator(
        &mut self,
        _expression: &Expression,
        _operator: &OperatorRef,
        _operands: &[NodeId],
    ) -> Result<Vec<ClassicalIssue>, ClassicalAnalysisFailure> {
        self.operator_checks
            .set(self.operator_checks.get().saturating_add(1));
        Ok(Vec::new())
    }

    fn check_effects(
        &mut self,
        _expression: &Expression,
    ) -> Result<Vec<ClassicalIssue>, ClassicalAnalysisFailure> {
        self.effect_checks
            .set(self.effect_checks.get().saturating_add(1));
        Ok(Vec::new())
    }

    fn check_child_references(
        &mut self,
        _expression: &Expression,
        _children: &[NodeId],
    ) -> Result<Vec<ClassicalIssue>, ClassicalAnalysisFailure> {
        self.child_reference_checks
            .set(self.child_reference_checks.get().saturating_add(1));
        Ok(Vec::new())
    }

    fn check_extension(
        &mut self,
        expression: &Expression,
        _extension: &ExpressionExtension,
    ) -> Result<Vec<ClassicalIssue>, ClassicalAnalysisFailure> {
        self.extension_checks
            .set(self.extension_checks.get().saturating_add(1));

        Ok(vec![ClassicalIssue::new(
            ClassicalIssueKind::UnsupportedExtension,
            expression.id(),
            "test.extension.unhandled",
            "The test context has no registered handler for this extension.",
        )])
    }
}

fn unrestricted_policy() -> ClassicalValidationPolicy {
    ClassicalValidationPolicy::unrestricted()
}

// =============================================================================
// Result lifecycle and validity boundaries
// =============================================================================

#[test]
fn zero_expression_program_completes_without_inventing_work() {
    let mut context = TestContext::default();

    let result = validate_classical(
        std::iter::empty::<Expression>(),
        &mut context,
        unrestricted_policy(),
    )
    .expect("an empty expression sequence should be valid input to this pass");

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 0);
    assert!(result.issues.is_empty());
    assert!(result.is_clean());

    assert_eq!(context.name_check_count(), 0);
    assert_eq!(context.operator_check_count(), 0);
    assert_eq!(context.effect_check_count(), 0);
    assert_eq!(context.child_reference_check_count(), 0);
}

#[test]
fn success_constructor_represents_a_clean_completed_pass() {
    let result = ClassicalValidation::success(0);

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 0);
    assert!(result.issues.is_empty());
    assert!(result.is_clean());
}

#[test]
fn with_issues_cannot_be_mistaken_for_a_clean_pass() {
    let issue = ClassicalIssue::new(
        ClassicalIssueKind::TypeSemantics,
        node_id(10),
        "test.type.mismatch",
        "Deliberately invalid test expression.",
    );

    let result = ClassicalValidation::with_issues(1, vec![issue]);

    assert_eq!(
        result.completion,
        ClassicalCompletion::CompleteWithIssues
    );
    assert_eq!(result.expressions_checked, 1);
    assert_eq!(result.issues.len(), 1);
    assert!(!result.is_clean());
}

#[test]
fn with_issues_with_no_issues_is_clean() {
    let result = ClassicalValidation::with_issues(3, Vec::new());

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 3);
    assert!(result.is_clean());
}

#[test]
fn incomplete_result_always_contains_an_incomplete_analysis_issue() {
    let result = ClassicalValidation::incomplete(
        2,
        Vec::new(),
        node_id(20),
        "The test stopped before completing all required checks.",
    );

    assert_eq!(result.completion, ClassicalCompletion::Incomplete);
    assert_eq!(result.expressions_checked, 2);
    assert!(!result.is_clean());

    assert_eq!(result.issues.len(), 1);
    assert_eq!(
        result.issues[0].kind,
        ClassicalIssueKind::IncompleteAnalysis
    );
    assert_eq!(result.issues[0].node, node_id(20));
    assert_eq!(
        result.issues[0].rule,
        "classical.analysis.incomplete"
    );
}

#[test]
fn incomplete_result_preserves_preexisting_issues() {
    let original = ClassicalIssue::new(
        ClassicalIssueKind::NameResolution,
        node_id(31),
        "test.name.unresolved",
        "An unresolved name was reported before the budget expired.",
    );

    let result = ClassicalValidation::incomplete(
        1,
        vec![original.clone()],
        node_id(32),
        "Operational budget exhausted.",
    );

    assert_eq!(result.completion, ClassicalCompletion::Incomplete);
    assert_eq!(result.issues.len(), 2);
    assert_eq!(result.issues[0], original);
    assert_eq!(
        result.issues[1].kind,
        ClassicalIssueKind::IncompleteAnalysis
    );
    assert!(!result.is_clean());
}

// =============================================================================
// Operational policy contracts
// =============================================================================

#[test]
fn unrestricted_policy_has_no_local_expression_or_issue_budget() {
    let policy = ClassicalValidationPolicy::unrestricted();

    assert_eq!(policy.max_expressions, None);
    assert_eq!(policy.max_issues, None);
    assert!(policy.validate().is_ok());
}

#[test]
fn default_policy_has_no_local_expression_or_issue_budget() {
    let policy = ClassicalValidationPolicy::default();

    assert_eq!(policy.max_expressions, None);
    assert_eq!(policy.max_issues, None);
    assert!(policy.validate().is_ok());
}

#[test]
fn positive_expression_budget_is_accepted() {
    let policy = ClassicalValidationPolicy {
        max_expressions: Some(1),
        max_issues: None,
    };

    assert!(policy.validate().is_ok());
}

#[test]
fn positive_issue_budget_is_accepted() {
    let policy = ClassicalValidationPolicy {
        max_expressions: None,
        max_issues: Some(1),
    };

    assert!(policy.validate().is_ok());
}

#[test]
fn zero_expression_budget_is_rejected_explicitly() {
    let policy = ClassicalValidationPolicy {
        max_expressions: Some(0),
        max_issues: None,
    };

    assert!(matches!(
        policy.validate(),
        Err(ClassicalAnalysisFailure::InvalidPolicy { .. })
    ));
}

#[test]
fn zero_issue_budget_is_rejected_explicitly() {
    let policy = ClassicalValidationPolicy {
        max_expressions: None,
        max_issues: Some(0),
    };

    assert!(matches!(
        policy.validate(),
        Err(ClassicalAnalysisFailure::InvalidPolicy { .. })
    ));
}

#[test]
fn invalid_policy_is_rejected_before_an_empty_pass_can_succeed() {
    let mut context = TestContext::default();
    let policy = ClassicalValidationPolicy {
        max_expressions: Some(0),
        max_issues: None,
    };

    let result = validate_classical(
        std::iter::empty::<Expression>(),
        &mut context,
        policy,
    );

    assert!(matches!(
        result,
        Err(ClassicalAnalysisFailure::InvalidPolicy { .. })
    ));
    assert_eq!(context.name_check_count(), 0);
    assert_eq!(context.operator_check_count(), 0);
}

// =============================================================================
// Expression dispatch and shared-service integration
// =============================================================================

#[test]
fn identifier_expression_invokes_name_and_effect_checks() {
    let expression = identifier_expression(100, 101);
    let mut context = TestContext::default();

    let result = validate_classical(
        vec![expression],
        &mut context,
        unrestricted_policy(),
    )
    .expect("test semantic services should not fail");

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 1);
    assert!(result.issues.is_empty());

    assert_eq!(context.name_check_count(), 1);
    assert_eq!(context.effect_check_count(), 1);
    assert_eq!(context.child_reference_check_count(), 1);
}

#[test]
fn unresolved_identifier_issue_is_preserved_by_the_classical_pass() {
    let expression = identifier_expression(110, 111);
    let mut context = TestContext::rejecting_names();

    let result = validate_classical(
        vec![expression],
        &mut context,
        unrestricted_policy(),
    )
    .expect("a semantic issue is not a service failure");

    assert_eq!(
        result.completion,
        ClassicalCompletion::CompleteWithIssues
    );
    assert_eq!(result.expressions_checked, 1);
    assert!(!result.is_clean());

    assert!(result.issues.iter().any(|issue| {
        issue.kind == ClassicalIssueKind::NameResolution
            && issue.rule == "test.name.unresolved"
            && issue.node == node_id(110)
    }));

    // Shared effect checking still runs when a name issue is present.
    assert_eq!(context.effect_check_count(), 1);
}

#[test]
fn binary_expression_delegates_operator_rules_to_shared_semantics() {
    let expression = binary_expression(120, 121, "add", 122);
    let mut context = TestContext::default();

    let result = validate_classical(
        vec![expression],
        &mut context,
        unrestricted_policy(),
    )
    .expect("test semantic services should not fail");

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 1);
    assert!(result.issues.is_empty());

    assert_eq!(context.operator_check_count(), 1);
    assert_eq!(context.effect_check_count(), 1);
    assert_eq!(context.child_reference_check_count(), 1);
}

#[test]
fn each_submitted_expression_is_counted_once_by_the_pass() {
    let expressions = vec![
        identifier_expression(130, 131),
        binary_expression(132, 133, "add", 134),
        identifier_expression(135, 136),
    ];

    let mut context = TestContext::default();

    let result = validate_classical(
        expressions,
        &mut context,
        unrestricted_policy(),
    )
    .expect("test semantic services should not fail");

    assert_eq!(result.completion, ClassicalCompletion::Complete);
    assert_eq!(result.expressions_checked, 3);
    assert_eq!(context.name_check_count(), 2);
    assert_eq!(context.operator_check_count(), 1);
    assert_eq!(context.effect_check_count(), 3);
    assert_eq!(context.child_reference_check_count(), 3);
}

// =============================================================================
// Budget exhaustion must not certify complete analysis
// =============================================================================

#[test]
fn expression_budget_exhaustion_returns_incomplete_analysis() {
    let expressions = vec![
        identifier_expression(200, 201),
        identifier_expression(202, 203),
    ];

    let mut context = TestContext::default();
    let policy = ClassicalValidationPolicy {
        max_expressions: Some(1),
        max_issues: None,
    };

    let result = validate_classical(expressions, &mut context, policy)
        .expect("budget exhaustion is an incomplete result, not a service error");

    assert_eq!(result.completion, ClassicalCompletion::Incomplete);
    assert_eq!(result.expressions_checked, 1);
    assert!(!result.is_clean());

    assert!(result.issues.iter().any(|issue| {
        issue.kind == ClassicalIssueKind::IncompleteAnalysis
            && issue.rule == "classical.analysis.incomplete"
    }));

    // The second expression triggers the budget check and is not analyzed.
    assert_eq!(context.name_check_count(), 1);
    assert_eq!(context.effect_check_count(), 1);
}

#[test]
fn issue_budget_exhaustion_returns_incomplete_analysis() {
    let expressions = vec![
        identifier_expression(210, 211),
        identifier_expression(212, 213),
    ];

    let mut context = TestContext::rejecting_names();
    let policy = ClassicalValidationPolicy {
        max_expressions: None,
        max_issues: Some(1),
    };

    let result = validate_classical(expressions, &mut context, policy)
        .expect("issue-budget exhaustion is an incomplete result");

    assert_eq!(result.completion, ClassicalCompletion::Incomplete);
    assert_eq!(result.expressions_checked, 1);
    assert!(!result.is_clean());

    assert!(result.issues.iter().any(|issue| {
        issue.kind == ClassicalIssueKind::NameResolution
    }));
    assert!(result.issues.iter().any(|issue| {
        issue.kind == ClassicalIssueKind::IncompleteAnalysis
    }));

    // No claim is made that the second expression was checked.
    assert_eq!(context.name_check_count(), 1);
}

#[test]
fn reaching_a_budget_does_not_report_a_clean_result() {
    let expressions = vec![
        identifier_expression(220, 221),
        identifier_expression(222, 223),
    ];

    let mut context = TestContext::default();
    let policy = ClassicalValidationPolicy {
        max_expressions: Some(1),
        max_issues: None,
    };

    let result = validate_classical(expressions, &mut context, policy)
        .expect("budget exhaustion should be explicit");

    assert_ne!(result.completion, ClassicalCompletion::Complete);
    assert!(!result.is_clean());
}

// =============================================================================
// Deterministic issue contracts
// =============================================================================

#[test]
fn issue_identity_and_rule_are_stable_values() {
    let issue = ClassicalIssue::new(
        ClassicalIssueKind::TypeSemantics,
        node_id(300),
        "classical.type.test",
        "Test type issue.",
    );

    assert_eq!(issue.kind, ClassicalIssueKind::TypeSemantics);
    assert_eq!(issue.node, node_id(300));
    assert_eq!(issue.related_node, None);
    assert_eq!(issue.rule, "classical.type.test");
    assert_eq!(issue.message, "Test type issue.");
}

#[test]
fn related_node_is_preserved() {
    let issue = ClassicalIssue::new(
        ClassicalIssueKind::AssignmentSemantics,
        node_id(310),
        "classical.assignment.test",
        "Test assignment issue.",
    )
    .with_related_node(node_id(311));

    assert_eq!(issue.node, node_id(310));
    assert_eq!(issue.related_node, Some(node_id(311)));
}

#[test]
fn repeated_analysis_with_identical_input_is_deterministic() {
    fn run_once() -> ClassicalValidation {
        let expressions = vec![
            identifier_expression(320, 321),
            binary_expression(322, 323, "add", 324),
        ];

        let mut context = TestContext::rejecting_names();

        validate_classical(
            expressions,
            &mut context,
            unrestricted_policy(),
        )
        .expect("test semantic services should not fail")
    }

    assert_eq!(run_once(), run_once());
}

// =============================================================================
// Grammar authority and composition contracts
// =============================================================================

#[test]
fn classical_expression_delegates_to_the_canonical_expression_rule() {
    assert!(
        CLASSICAL_GRAMMAR.contains("classicalExpression"),
        "the canonical classical grammar should expose its expression boundary"
    );

    assert!(
        CLASSICAL_GRAMMAR.contains("classicalExpression\n    : expression"),
        "classical expressions must delegate to the canonical expression rule"
    );

    assert!(
        EXPRESSION_GRAMMAR.contains("expression\n    : assignmentExpression"),
        "the canonical expression composition root must own the expression entry point"
    );
}

#[test]
fn classical_grammar_reuses_canonical_statement_syntax() {
    assert!(
        CLASSICAL_GRAMMAR.contains("classicalStatementSequence"),
        "the classical grammar should expose its statement-sequence boundary"
    );

    assert!(
        CLASSICAL_GRAMMAR.contains("classicalStatementSequence\n    : statement*"),
        "classical statement sequences must reuse canonical statement syntax"
    );
}

#[test]
fn classical_grammar_does_not_claim_target_feasibility_as_syntax() {
    assert!(
        CLASSICAL_GRAMMAR.contains("MUST NOT report hardware availability as a syntax error"),
        "hardware feasibility must remain outside the grammar's syntax boundary"
    );
}

#[test]
fn grammar_files_are_nonempty_and_have_the_expected_authority_markers() {
    assert!(!CLASSICAL_GRAMMAR.trim().is_empty());
    assert!(!EXPRESSION_GRAMMAR.trim().is_empty());

    assert!(
        CLASSICAL_GRAMMAR.contains("CANONICAL CLASSICAL-DOMAIN COMPOSITION GRAMMAR")
    );
    assert!(
        EXPRESSION_GRAMMAR.contains("CANONICAL EXPRESSION ORCHESTRATOR")
    );
}

// =============================================================================
// Resource-parametric behavior
// =============================================================================

#[test]
fn large_operational_budget_is_not_a_hardware_capacity_limit() {
    // This tests policy representation only. It does not allocate a matching
    // number of expressions or claim that processing arbitrary input is free.
    let budget = usize::MAX;

    let policy = ClassicalValidationPolicy {
        max_expressions: Some(budget),
        max_issues: None,
    };

    assert!(policy.validate().is_ok());
    assert_eq!(policy.max_expressions, Some(usize::MAX));
}

#[test]
fn unrestricted_policy_remains_distinct_from_a_finite_budget() {
    let unrestricted = ClassicalValidationPolicy::unrestricted();
    let finite = ClassicalValidationPolicy {
        max_expressions: Some(1),
        max_issues: Some(1),
    };

    assert_eq!(unrestricted.max_expressions, None);
    assert_eq!(unrestricted.max_issues, None);

    assert_eq!(finite.max_expressions, Some(1));
    assert_eq!(finite.max_issues, Some(1));
}