#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration-level conformance tests for Zamani semantic resource analysis.
//!
//! Path: src/frontend/semantic/tests/resources.rs
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! PURPOSE
//!
//! Verify that the canonical semantic resource model, symbolic-expression
//! service, requirement evaluator, and constraint evaluator preserve
//! target-independent resource intent without arbitrary hardware ceilings.
//!
//! AUTHORITY
//!
//! - grammar/resources/requirements.g4
//! - grammar/resources/constraints.g4
//! - grammar/resources/preferences.g4
//! - src/frontend/ast/node/resources/resource.rs
//! - src/frontend/ast/node/resources/resource_kind.rs
//! - src/frontend/semantic/resources/model.rs
//! - src/frontend/semantic/resources/requirements.rs
//! - src/frontend/semantic/resources/symbolic.rs
//! - src/frontend/semantic/resources/constraints.rs
//!
//! OWNERSHIP
//!
//! These tests own no production semantic implementation. They exercise the
//! canonical APIs and use a deterministic test resolver to provide explicit
//! semantic evidence.
//!
//! DOES NOT TEST
//!
//! - Physical resource allocation.
//! - Hardware discovery or target selection.
//! - Parser correctness or AST construction.
//! - General expression type checking.
//! - Unit conversion without a registered conversion service.
//! - Runtime execution or backend feasibility.
//!
//! SCALABILITY CONTRACT
//!
//! No test assumes a fixed number of CPUs, GPUs, FPGAs, QPUs, nodes,
//! registers, devices, tensor dimensions, or resource kinds.
//!
//! Arbitrarily large decimal values are tested as textual quantities.
//! Operational test sizes are bounded by the test process's available
//! memory and execution resources.
//!
//! INTEGRATION
//!
//! Register this file in the semantic test module using `mod resources;`
//! once that module exists. The parent test module must itself be registered
//! by the canonical semantic module.
//!
//! These tests must compile against the actual production APIs. Do not add
//! duplicate resource types or test-only production implementations.

use core::cmp::Ordering;

use crate::frontend::ast::node::resources::resource_kind::ResourceKind;
use crate::frontend::semantic::resources::constraints::{
    AssessmentKind,
    ConstraintOutcome,
    ResourceConstraintEvaluator,
    ResourceConstraintResolver,
    ResourceValueResolution,
    ExpressionValueResolution,
    ResolvedResourceValue,
};
use crate::frontend::semantic::resources::model::{
    ConstraintOperand,
    ConstraintOperator,
    ExactQuantity,
    ResourceConstraint,
    ResourceIntent,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};
use crate::frontend::semantic::resources::symbolic::{
    normalize,
    NormalizationBudget,
    SymbolicOperator,
    SymbolicResourceExpr,
};

// ---------------------------------------------------------------------------
// Shared test helpers
// ---------------------------------------------------------------------------

fn resource_kind(namespace: &str, name: &str) -> ResourceKind {
    ResourceKind::new(namespace, name)
        .expect("test resource kind must be structurally valid")
}

fn quantity(value: &str) -> ExactQuantity {
    ExactQuantity::new(value)
        .expect("test quantity must be a valid exact non-negative decimal")
}

fn resource_unit(namespace: &str, name: &str) -> ResourceUnit {
    ResourceUnit::new(namespace, name)
        .expect("test unit identity must be valid")
}

fn requirement(
    namespace: &str,
    name: &str,
    requested: &str,
    intent: ResourceIntent,
) -> ResourceRequirement {
    ResourceRequirement::new(
        resource_kind(namespace, name),
        ResourceQuantity::known(requested)
            .expect("requested quantity must be valid"),
        None,
        intent,
    )
    .expect("test requirement must be valid")
}

fn constraint(
    namespace: &str,
    name: &str,
    operator: ConstraintOperator,
    expected: &str,
    intent: ResourceIntent,
    unit: Option<ResourceUnit>,
) -> ResourceConstraint {
    ResourceConstraint::new(
        resource_kind(namespace, name),
        operator,
        ConstraintOperand::Known(quantity(expected)),
        unit,
        intent,
        None,
    )
    .expect("test constraint must be valid")
}

/// Explicit, deterministic evidence supplied to the semantic evaluator.
///
/// This mock never discovers hardware. A `Known` value represents evidence
/// supplied by the test; `Unavailable` and `Symbolic` remain unresolved.
#[derive(Clone, Debug)]
struct TestResolver {
    value: ResourceValueResolution,
    expression: ExpressionValueResolution,
}

impl TestResolver {
    fn known(value: &str) -> Self {
        Self {
            value: ResourceValueResolution::Known(
                ResolvedResourceValue::new(quantity(value), None),
            ),
            expression: ExpressionValueResolution::Symbolic,
        }
    }

    fn known_with_unit(value: &str, unit: ResourceUnit) -> Self {
        Self {
            value: ResourceValueResolution::Known(
                ResolvedResourceValue::new(quantity(value), Some(unit)),
            ),
            expression: ExpressionValueResolution::Symbolic,
        }
    }

    fn unavailable() -> Self {
        Self {
            value: ResourceValueResolution::Unavailable,
            expression: ExpressionValueResolution::Symbolic,
        }
    }

    fn symbolic() -> Self {
        Self {
            value: ResourceValueResolution::Symbolic { expression: None },
            expression: ExpressionValueResolution::Symbolic,
        }
    }

    fn failing() -> Self {
        Self {
            value: ResourceValueResolution::Unavailable,
            expression: ExpressionValueResolution::Invalid(
                "deliberate test failure".to_owned(),
            ),
        }
    }
}

impl ResourceConstraintResolver for TestResolver {
    fn resource_value(
        &self,
        _kind: &ResourceKind,
    ) -> Result<ResourceValueResolution, String> {
        Ok(self.value.clone())
    }

    fn expression_value(
        &self,
        _expression: crate::frontend::ast::node::node_id::NodeId,
    ) -> Result<ExpressionValueResolution, String> {
        Ok(self.expression.clone())
    }
}

// ---------------------------------------------------------------------------
// Exact quantities: arbitrary magnitude, canonical form, checked semantics
// ---------------------------------------------------------------------------

#[test]
fn exact_quantities_do_not_depend_on_host_integer_width() {
    let input = concat!(
        "12345678901234567890123456789012345678901234567890",
        "12345678901234567890123456789012345678901234567890"
    );

    let parsed = quantity(input);

    assert_eq!(parsed.as_str(), input);
    assert!(!parsed.is_zero());
}

#[test]
fn exact_quantities_canonicalize_leading_and_trailing_zeroes() {
    let cases = [
        ("00042", "42"),
        ("00042.5000", "42.5"),
        ("000.125000", "0.125"),
        ("000.000", "0"),
        ("0", "0"),
        ("100000000000000000000000000000000000000000000000000", 
         "100000000000000000000000000000000000000000000000000"),
    ];

    for (input, expected) in cases {
        assert_eq!(quantity(input).as_str(), expected, "{input}");
    }
}

#[test]
fn exact_quantity_comparison_is_numeric_not_lexicographic() {
    let cases = [
        ("2", "10", Ordering::Less),
        ("10", "2", Ordering::Greater),
        ("1.2", "1.20", Ordering::Equal),
        ("0.09", "0.1", Ordering::Less),
        ("999999999999999999999999999999", 
         "1000000000000000000000000000000", Ordering::Less),
        ("12345678901234567890.1234",
         "12345678901234567890.1235", Ordering::Less),
    ];

    for (left, right, expected) in cases {
        assert_eq!(
            quantity(left).numeric_cmp(&quantity(right)),
            expected,
            "{left} compared with {right}"
        );
    }
}

#[test]
fn exact_quantity_rejects_invalid_or_ambiguous_input() {
    for invalid in [
        "",
        "-1",
        "+1",
        ".5",
        "1.",
        "1.2.3",
        "1e9",
        " 1",
        "1 ",
        "NaN",
        "Infinity",
        "１２",
    ] {
        assert!(
            ExactQuantity::new(invalid).is_err(),
            "invalid quantity unexpectedly accepted: {invalid:?}"
        );
    }
}

#[test]
fn decimal_quantities_remain_exact_without_floating_point_rounding() {
    let first = quantity("0.100000000000000000000000000000000001");
    let second = quantity("0.100000000000000000000000000000000002");

    assert_eq!(first.numeric_cmp(&second), Ordering::Less);
    assert_ne!(first, second);
}

// ---------------------------------------------------------------------------
// Extensible resource identities and units
// ---------------------------------------------------------------------------

#[test]
fn resource_kinds_support_future_namespaces_without_core_changes() {
    let kinds = [
        resource_kind("core", "memory"),
        resource_kind("quantum", "logical-resource"),
        resource_kind("hdl", "signal-storage"),
        resource_kind("future.neural", "distributed-tensor"),
        resource_kind("future.network", "communication-capacity"),
    ];

    for kind in kinds {
        let encoded = kind.to_string();
        let decoded = ResourceKind::parse(&encoded)
            .expect("canonical resource kind must round-trip");

        assert_eq!(kind, decoded);
    }
}

#[test]
fn resource_kind_identity_is_namespaced() {
    let first = resource_kind("future.alpha", "capacity");
    let second = resource_kind("future.beta", "capacity");

    assert_ne!(first, second);
    assert_ne!(first.to_string(), second.to_string());
}

#[test]
fn resource_units_are_extensible_and_stable() {
    let first = resource_unit("future.energy", "logical-unit");
    let second = resource_unit("future.energy", "logical-unit");

    assert_eq!(first, second);
    assert_eq!(first.canonical_name(), "future.energy:logical-unit");

    let other = resource_unit("future.energy", "alternate-unit");
    assert_ne!(first, other);
}

#[test]
fn invalid_unit_identities_are_rejected() {
    assert!(ResourceUnit::new("", "byte").is_err());
    assert!(ResourceUnit::new("core", "").is_err());
    assert!(ResourceUnit::new("core", " ").is_err());
    assert!(ResourceUnit::new("core\n", "byte").is_err());
}

// ---------------------------------------------------------------------------
// Requirements: preserve intent, quantity, and symbolic state
// ---------------------------------------------------------------------------

#[test]
fn known_resource_requirement_preserves_all_semantic_fields() {
    let expected_kind = resource_kind("future.compute", "capacity");
    let expected_unit = resource_unit("future.compute", "work-unit");

    let result = ResourceRequirement::new(
        expected_kind.clone(),
        ResourceQuantity::known(
            "1234567890123456789012345678901234567890"
        )
        .expect("valid exact quantity"),
        Some(expected_unit.clone()),
        ResourceIntent::Required,
    )
    .expect("valid requirement");

    assert_eq!(result.kind(), &expected_kind);
    assert_eq!(result.intent(), ResourceIntent::Required);
    assert_eq!(result.unit(), Some(&expected_unit));

    let known = result
        .quantity()
        .known_value()
        .expect("quantity must remain known");

    assert_eq!(
        known.as_str(),
        "1234567890123456789012345678901234567890"
    );
}

#[test]
fn resource_intents_remain_semantically_distinct() {
    let intents = [
        ResourceIntent::Required,
        ResourceIntent::Preferred,
        ResourceIntent::Allowed,
        ResourceIntent::Forbidden,
        ResourceIntent::Hint,
        ResourceIntent::Budget,
    ];

    for (index, left) in intents.iter().enumerate() {
        for right in intents.iter().skip(index + 1) {
            assert_ne!(
                left, right,
                "distinct resource intents must not collapse"
            );
        }
    }
}

#[test]
fn resource_requirement_sort_keys_are_repeatable() {
    let first = requirement(
        "future.distributed",
        "capacity",
        "12345678901234567890",
        ResourceIntent::Required,
    );

    let second = requirement(
        "future.distributed",
        "capacity",
        "12345678901234567890",
        ResourceIntent::Required,
    );

    assert_eq!(first.canonical_sort_key(), second.canonical_sort_key());
}

#[test]
fn unknown_resource_kinds_are_not_rejected_merely_for_being_new() {
    let new_kind = resource_kind(
        "future.domain.with.versioning",
        "resource-class",
    );

    let result = ResourceRequirement::new(
        new_kind.clone(),
        ResourceQuantity::known("7").expect("valid quantity"),
        None,
        ResourceIntent::Required,
    );

    assert!(result.is_ok());
    assert_eq!(
        result.expect("valid requirement").kind(),
        &new_kind
    );
}

// ---------------------------------------------------------------------------
// Symbolic resource expressions: preserve meaning and enforce work budgets
// ---------------------------------------------------------------------------

#[test]
fn symbolic_constants_use_the_canonical_exact_quantity_model() {
    let expression = SymbolicResourceExpr::constant(
        "000123456789012345678901234567890.5000",
    )
    .expect("valid symbolic constant");

    assert!(expression.is_constant());
    assert_eq!(
        expression
            .constant_value()
            .expect("constant value")
            .as_str(),
        "123456789012345678901234567890.5"
    );
}

#[test]
fn symbolic_ast_expressions_keep_their_canonical_identity() {
    let node = crate::frontend::ast::node::node_id::NodeId::new(41)
        .expect("positive AST node identity");

    let expression = SymbolicResourceExpr::ast_expression(node);

    assert_eq!(expression.ast_node_id(), Some(node));
    assert!(!expression.is_constant());

    let quantity = expression
        .to_resource_quantity()
        .expect("direct AST expression can be represented symbolically");

    assert_eq!(quantity.expression_id(), Some(node));
    assert!(!quantity.is_known());
}

#[test]
fn symbolic_compound_expressions_do_not_invent_ast_identities() {
    let expression = SymbolicResourceExpr::add(
        SymbolicResourceExpr::constant("100")
            .expect("valid left constant"),
        SymbolicResourceExpr::constant("200")
            .expect("valid right constant"),
    );

    assert_eq!(expression.ast_node_id(), None);
    assert_eq!(expression.node_count(), 3);
}

#[test]
fn symbolic_normalization_folds_exact_integer_constants() {
    let expression = SymbolicResourceExpr::add(
        SymbolicResourceExpr::constant("100")
            .expect("valid left constant"),
        SymbolicResourceExpr::constant("200")
            .expect("valid right constant"),
    );

    let result = normalize(expression, NormalizationBudget::unlimited())
        .expect("normalization should succeed");

    assert_eq!(result.visited_nodes(), 3);
    assert_eq!(
        result
            .expression()
            .constant_value()
            .expect("folded constant")
            .as_str(),
        "300"
    );
}

#[test]
fn symbolic_normalization_preserves_unresolved_expressions() {
    let node = crate::frontend::ast::node::node_id::NodeId::new(99)
        .expect("positive AST node identity");

    let expression = SymbolicResourceExpr::add(
        SymbolicResourceExpr::ast_expression(node),
        SymbolicResourceExpr::constant("0")
            .expect("valid constant"),
    );

    let result = normalize(expression, NormalizationBudget::unlimited())
        .expect("normalization should preserve symbolic meaning");

    assert_eq!(result.expression().ast_node_id(), Some(node));
}

#[test]
fn symbolic_normalization_rejects_exhausted_work_budgets() {
    let expression = SymbolicResourceExpr::add(
        SymbolicResourceExpr::constant("1").expect("valid constant"),
        SymbolicResourceExpr::constant("2").expect("valid constant"),
    );

    let result = normalize(expression, NormalizationBudget::new(Some(2)));

    assert!(
        result.is_err(),
        "an incomplete normalization must not be reported as success"
    );
}

#[test]
fn symbolic_normalization_accepts_a_sufficient_explicit_budget() {
    let expression = SymbolicResourceExpr::multiply(
        SymbolicResourceExpr::constant("12345678901234567890")
            .expect("valid constant"),
        SymbolicResourceExpr::constant("10")
            .expect("valid constant"),
    );

    let result = normalize(expression, NormalizationBudget::new(Some(3)))
        .expect("the budget permits all three expression nodes");

    assert_eq!(result.visited_nodes(), 3);
    assert_eq!(
        result
            .expression()
            .constant_value()
            .expect("folded multiplication")
            .as_str(),
        "123456789012345678900"
    );
}

#[test]
fn symbolic_normalization_rejects_negative_resource_arithmetic() {
    let expression = SymbolicResourceExpr::subtract(
        SymbolicResourceExpr::constant("1").expect("valid constant"),
        SymbolicResourceExpr::constant("2").expect("valid constant"),
    );

    assert!(
        normalize(expression, NormalizationBudget::unlimited()).is_err(),
        "negative resource quantities must not silently wrap"
    );
}

#[test]
fn symbolic_normalization_rejects_division_by_zero() {
    let expression = SymbolicResourceExpr::divide(
        SymbolicResourceExpr::constant("10").expect("valid constant"),
        SymbolicResourceExpr::constant("0").expect("valid constant"),
    );

    assert!(
        normalize(expression, NormalizationBudget::unlimited()).is_err(),
        "division by zero must produce an explicit error"
    );
}

#[test]
fn symbolic_commutative_expressions_have_deterministic_keys() {
    let left = SymbolicResourceExpr::add(
        SymbolicResourceExpr::constant("2").expect("valid constant"),
        SymbolicResourceExpr::constant("3").expect("valid constant"),
    );

    let right = SymbolicResourceExpr::add(
        SymbolicResourceExpr::constant("3").expect("valid constant"),
        SymbolicResourceExpr::constant("2").expect("valid constant"),
    );

    let left = normalize(left, NormalizationBudget::unlimited())
        .expect("normalization")
        .into_expression();

    let right = normalize(right, NormalizationBudget::unlimited())
        .expect("normalization")
        .into_expression();

    assert_eq!(left.canonical_key(), right.canonical_key());
}

// ---------------------------------------------------------------------------
// Constraint evaluation: evidence, units, outcomes, and enforcement
// ---------------------------------------------------------------------------

#[test]
fn satisfied_required_constraint_is_proven_only_with_evidence() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "core",
        "memory",
        ConstraintOperator::GreaterThanOrEqual,
        "100",
        ResourceIntent::Required,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::known("100"),
    );

    assert_eq!(result.kind(), AssessmentKind::Constraint);
    assert_eq!(result.outcome(), ConstraintOutcome::Satisfied);
    assert!(!result.blocks_acceptance());
    assert!(!result.needs_more_evidence());
}

#[test]
fn violated_required_constraint_blocks_acceptance() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "core",
        "memory",
        ConstraintOperator::GreaterThanOrEqual,
        "100",
        ResourceIntent::Required,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::known("99"),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Violated);
    assert!(result.blocks_acceptance());
}

#[test]
fn violated_preference_remains_advisory() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "future.compute",
        "throughput",
        ConstraintOperator::GreaterThanOrEqual,
        "1000",
        ResourceIntent::Preferred,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::known("10"),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Violated);
    assert!(!result.blocks_acceptance());
}

#[test]
fn missing_resource_evidence_is_deferred_not_satisfied() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "quantum",
        "logical-resource",
        ConstraintOperator::GreaterThanOrEqual,
        "100000000000000000000000000000000000",
        ResourceIntent::Required,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::unavailable(),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
    assert!(result.needs_more_evidence());
    assert!(!result.blocks_acceptance());
}

#[test]
fn symbolic_resource_evidence_is_not_assumed_to_be_zero() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "future.distributed",
        "node-capacity",
        ConstraintOperator::GreaterThan,
        "0",
        ResourceIntent::Required,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::symbolic(),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
    assert!(result.needs_more_evidence());
}

#[test]
fn incompatible_units_are_deferred_without_conversion_evidence() {
    let evaluator = ResourceConstraintEvaluator::new();

    let actual_unit = resource_unit("length", "meter");
    let requested_unit = resource_unit("length", "centimeter");

    let condition = constraint(
        "future.geometry",
        "distance",
        ConstraintOperator::GreaterThan,
        "100",
        ResourceIntent::Required,
        Some(requested_unit),
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::known_with_unit("2", actual_unit),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
    assert!(result.needs_more_evidence());
}

#[test]
fn identical_units_allow_exact_comparison() {
    let evaluator = ResourceConstraintEvaluator::new();
    let unit = resource_unit("data", "logical-unit");

    let condition = constraint(
        "future.data",
        "capacity",
        ConstraintOperator::GreaterThanOrEqual,
        "10",
        ResourceIntent::Required,
        Some(unit.clone()),
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &TestResolver::known_with_unit("10", unit),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Satisfied);
}

#[test]
fn resolver_failures_do_not_produce_successful_assessments() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "core",
        "compute",
        ConstraintOperator::GreaterThan,
        "0",
        ResourceIntent::Required,
        None,
    );

    let result = evaluator.evaluate_constraint(
        &condition,
        &FailingResourceResolver,
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Invalid);
    assert!(result.blocks_acceptance());
    assert!(result.needs_more_evidence());
}

struct FailingResourceResolver;

impl ResourceConstraintResolver for FailingResourceResolver {
    fn resource_value(
        &self,
        _kind: &ResourceKind,
    ) -> Result<ResourceValueResolution, String> {
        Err("deliberate resolver failure".to_owned())
    }

    fn expression_value(
        &self,
        _expression: crate::frontend::ast::node::node_id::NodeId,
    ) -> Result<ExpressionValueResolution, String> {
        Err("deliberate expression failure".to_owned())
    }
}

#[test]
fn resource_requirement_evaluation_preserves_required_semantics() {
    let evaluator = ResourceConstraintEvaluator::new();

    let required = requirement(
        "core",
        "memory",
        "100",
        ResourceIntent::Required,
    );

    let enough = evaluator.evaluate_requirement(
        &required,
        &TestResolver::known("100"),
    );

    assert_eq!(enough.kind(), AssessmentKind::Requirement);
    assert_eq!(enough.outcome(), ConstraintOutcome::Satisfied);

    let insufficient = evaluator.evaluate_requirement(
        &required,
        &TestResolver::known("99"),
    );

    assert_eq!(insufficient.outcome(), ConstraintOutcome::Violated);
    assert!(insufficient.blocks_acceptance());
}

#[test]
fn unresolved_requirement_quantity_remains_deferred() {
    let evaluator = ResourceConstraintEvaluator::new();

    let requested = crate::frontend::ast::node::node_id::NodeId::new(700)
        .expect("positive AST node identity");

    let required = ResourceRequirement::new(
        resource_kind("quantum", "logical-resource"),
        ResourceQuantity::Symbolic(requested),
        None,
        ResourceIntent::Required,
    )
    .expect("valid symbolic requirement");

    let result = evaluator.evaluate_requirement(
        &required,
        &TestResolver::known("100"),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Deferred);
    assert!(result.needs_more_evidence());
    assert!(!result.blocks_acceptance());
}

#[test]
fn allowed_and_hint_intents_do_not_become_mandatory_requirements() {
    let evaluator = ResourceConstraintEvaluator::new();

    for intent in [ResourceIntent::Allowed, ResourceIntent::Hint] {
        let item = requirement("future.compute", "capacity", "999", intent);

        let result = evaluator.evaluate_requirement(
            &item,
            &TestResolver::unavailable(),
        );

        assert_eq!(
            result.outcome(),
            ConstraintOutcome::NotApplicable,
            "intent {intent:?} must not impose an invented capacity test"
        );
        assert!(!result.blocks_acceptance());
    }
}

#[test]
fn forbidden_resource_is_satisfied_only_when_known_zero() {
    let evaluator = ResourceConstraintEvaluator::new();

    let forbidden = requirement(
        "future.domain",
        "prohibited-resource",
        "0",
        ResourceIntent::Forbidden,
    );

    let unused = evaluator.evaluate_requirement(
        &forbidden,
        &TestResolver::known("0"),
    );

    assert_eq!(unused.outcome(), ConstraintOutcome::Satisfied);

    let in_use = evaluator.evaluate_requirement(
        &forbidden,
        &TestResolver::known("1"),
    );

    assert_eq!(in_use.outcome(), ConstraintOutcome::Violated);
    assert!(in_use.blocks_acceptance());
}

#[test]
fn advisory_resource_requirements_do_not_block_on_violation() {
    let evaluator = ResourceConstraintEvaluator::new();

    let preferred = requirement(
        "future.compute",
        "throughput",
        "1000",
        ResourceIntent::Preferred,
    );

    let result = evaluator.evaluate_requirement(
        &preferred,
        &TestResolver::known("1"),
    );

    assert_eq!(result.outcome(), ConstraintOutcome::Violated);
    assert!(!result.blocks_acceptance());
}

// ---------------------------------------------------------------------------
// Determinism and domain neutrality
// ---------------------------------------------------------------------------

#[test]
fn repeated_resource_evaluation_is_deterministic() {
    let evaluator = ResourceConstraintEvaluator::new();

    let condition = constraint(
        "future.tensor",
        "element-capacity",
        ConstraintOperator::GreaterThanOrEqual,
        "123456789012345678901234567890",
        ResourceIntent::Required,
        None,
    );

    let resolver = TestResolver::known(
        "123456789012345678901234567890",
    );

    let first = evaluator.evaluate_constraint(&condition, &resolver);
    let second = evaluator.evaluate_constraint(&condition, &resolver);

    assert_eq!(first, second);
}

#[test]
fn resource_model_is_domain_neutral() {
    let examples = [
        ("core", "compute"),
        ("core", "memory"),
        ("quantum", "logical-resource"),
        ("hdl", "signal-storage"),
        ("ai.tensor", "dimension"),
        ("distributed", "communication-capacity"),
        ("future.domain", "resource-class"),
    ];

    for (namespace, name) in examples {
        let item = requirement(
            namespace,
            name,
            "123456789012345678901234567890",
            ResourceIntent::Required,
        );

        assert_eq!(item.kind().namespace(), namespace);
        assert_eq!(item.kind().name(), name);
        assert_eq!(
            item.quantity()
                .known_value()
                .expect("known quantity")
                .as_str(),
            "123456789012345678901234567890"
        );
    }
}

#[test]
fn resource_requirements_do_not_allocate_or_select_hardware() {
    let item = requirement(
        "quantum",
        "logical-resource",
        "100000000000000000000000000000000000",
        ResourceIntent::Required,
    );

    assert_eq!(item.intent(), ResourceIntent::Required);
    assert!(item.source_node().is_none());

    // The semantic model records a requirement. No physical target, device
    // handle, placement, or allocation is present in this public contract.
}

#[test]
fn resource_constraint_comparison_operators_remain_distinct() {
    let operators = [
        ConstraintOperator::Equal,
        ConstraintOperator::NotEqual,
        ConstraintOperator::LessThan,
        ConstraintOperator::LessThanOrEqual,
        ConstraintOperator::GreaterThan,
        ConstraintOperator::GreaterThanOrEqual,
    ];

    for (index, left) in operators.iter().enumerate() {
        for right in operators.iter().skip(index + 1) {
            assert_ne!(left, right);
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration and implementation limits are not language-level ceilings
// ---------------------------------------------------------------------------

#[test]
fn explicitly_unlimited_normalization_budget_is_representable() {
    let budget = NormalizationBudget::unlimited();

    assert_eq!(budget.max_nodes(), None);
}

#[test]
fn finite_normalization_budget_is_explicit_and_inspectable() {
    let budget = NormalizationBudget::new(Some(17));

    assert_eq!(budget.max_nodes(), Some(17));
}

#[test]
fn large_symbolic_quantities_are_not_converted_to_usize() {
    let expression = SymbolicResourceExpr::constant(
        "999999999999999999999999999999999999999999999999999999999999999999",
    )
    .expect("arbitrary-length decimal constant must remain representable");

    let result = expression
        .to_resource_quantity()
        .expect("constant can be represented as a known resource quantity");

    assert_eq!(
        result
            .known_value()
            .expect("known quantity")
            .as_str(),
        "999999999999999999999999999999999999999999999999999999999999999999"
    );
}