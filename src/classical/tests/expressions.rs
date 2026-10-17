
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Expressions — Integration Contract Tests
//!
//! File: src/classical/tests/expressions.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!   Proposed. Compile and run through the repository's actual test harness
//!   before marking this file verified.
//!
//! PURPOSE
//!   Test the public classical expression integration contract against the
//!   canonical frontend AST. Cover structural validation, extensible operator
//!   registration, evaluation-order planning, explicit validation budgets,
//!   graph traversal, and public API compatibility.
//!
//! AUTHORITY
//!   - grammar/expressions/expressions.g4
//!   - grammar/expressions/compile-time.g4
//!   - grammar/declarations/constants.g4
//!   - grammar/specification/semantic-model.md
//!   - grammar/specification/scalability-model.md
//!   - src/frontend/ast/node/expressions/expression.rs
//!   - src/frontend/ast/node/node_id.rs
//!   - src/frontend/ast/node/source/span.rs
//!   - src/classical/expressions/
//!
//! OWNERSHIP
//!   These tests do not define new expression, type, semantic, or IR models.
//!   They consume the canonical frontend AST and the public classical
//!   expression integration.
//!
//! SEMANTIC BOUNDARY
//!   Structural validation is not type checking or name resolution.
//!   Evaluation planning is not execution.
//!   Operator spelling registration is not operator semantic resolution.
//!   Optimization is not authorized merely because an expression is valid.
//!
//! SCALABILITY
//!   No universal maximum expression size, depth, operand count, hardware
//!   count, or target count is asserted. Tests for explicit budgets verify
//!   that a caller-selected policy is distinguishable from unrestricted
//!   structural validation.
//!
//! SAFETY
//!   Safe Rust only. Tests do not execute source expressions or access
//!   hardware, the filesystem, or the network.

use std::collections::BTreeMap;
use std::ops::ControlFlow;

use crate::classical::expressions::{
    plan_expression,
    validate_expression,
    walk_expression_graph,
    BinaryEvaluation,
    EvaluationPlan,
    Expression,
    ExpressionError,
    ExpressionKind,
    ExpressionValidationPolicy,
    ExpressionWalkError,
    NodeId,
    OperatorFixity,
    OperatorRef,
    OperatorRegistry,
    OperatorRegistryError,
    ShortCircuitKind,
};
use crate::frontend::ast::node::metadata::NodeMetadata;
use crate::frontend::ast::node::source::span::{SourceId, Span};
use crate::frontend::ast::node::source::span::SourceOffset;

// ---------------------------------------------------------------------------
// Shared test constructors
// ---------------------------------------------------------------------------

fn id(raw: u64) -> NodeId {
    NodeId::new(raw).expect("test AST node IDs must be non-zero")
}

fn empty_span() -> Span {
    Span::point(SourceId::from_raw(1), SourceOffset::from(0))
}

fn expression(raw_id: u64, kind: ExpressionKind) -> Expression {
    Expression::without_metadata(id(raw_id), kind, empty_span())
}

fn identifier_expression(raw_id: u64, identifier_id: u64) -> Expression {
    expression(
        raw_id,
        ExpressionKind::Identifier {
            identifier: id(identifier_id),
        },
    )
}

fn binary_expression(
    raw_id: u64,
    left: u64,
    operator: OperatorRef,
    right: u64,
) -> Expression {
    expression(
        raw_id,
        ExpressionKind::Binary {
            left: id(left),
            operator,
            right: id(right),
        },
    )
}

fn no_short_circuit(_: &OperatorRef) -> Option<ShortCircuitKind> {
    None
}

fn logical_classifier(operator: &OperatorRef) -> Option<ShortCircuitKind> {
    match (operator.namespace(), operator.name()) {
        ("zamani", "logical-and") => Some(ShortCircuitKind::And),
        ("zamani", "logical-or") => Some(ShortCircuitKind::Or),
        ("zamani", "coalesce") => Some(ShortCircuitKind::Coalesce),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Canonical AST integration and structural validation
// ---------------------------------------------------------------------------

#[test]
fn classical_expression_api_uses_canonical_frontend_types() {
    // These assignments are compile-time API compatibility checks.
    let _: Option<Expression> = None;
    let _: Option<ExpressionKind> = None;
    let _: Option<NodeId> = None;
    let _: Option<OperatorRef> = None;
    let _: Option<ExpressionError> = None;
    let _: Option<ExpressionWalkError> = None;
}

#[test]
fn valid_identifier_expression_passes_unrestricted_validation() {
    let node = identifier_expression(1, 2);

    assert!(
        validate_expression(&node, ExpressionValidationPolicy::unrestricted())
            .is_ok()
    );
}

#[test]
fn binary_expression_preserves_canonical_child_order() {
    let operator = OperatorRef::native("add")
        .expect("non-empty operator name should be valid");

    let node = binary_expression(1, 2, operator.clone(), 3);

    assert_eq!(node.id(), id(1));
    assert_eq!(
        node.child_node_ids(),
        vec![id(2), id(3)]
    );

    assert!(
        validate_expression(&node, ExpressionValidationPolicy::unrestricted())
            .is_ok()
    );
}

#[test]
fn conditional_expression_preserves_branch_presence_and_order() {
    let node = expression(
        1,
        ExpressionKind::Conditional {
            condition: id(2),
            then_branch: id(3),
            else_branch: Some(id(4)),
        },
    );

    assert_eq!(
        node.child_node_ids(),
        vec![id(2), id(3), id(4)]
    );

    assert!(
        validate_expression(&node, ExpressionValidationPolicy::unrestricted())
            .is_ok()
    );
}

#[test]
fn conditional_expression_without_else_does_not_invent_a_branch() {
    let node = expression(
        1,
        ExpressionKind::Conditional {
            condition: id(2),
            then_branch: id(3),
            else_branch: None,
        },
    );

    assert_eq!(node.child_node_ids(), vec![id(2), id(3)]);

    assert!(
        validate_expression(&node, ExpressionValidationPolicy::unrestricted())
            .is_ok()
    );
}

#[test]
fn malformed_operator_identity_is_rejected_at_construction() {
    let result = OperatorRef::native("");

    assert!(matches!(
        result,
        Err(ExpressionError::EmptyOperatorName)
    ));
}

#[test]
fn extension_identity_rejects_empty_namespace() {
    let result = crate::classical::expressions::ExpressionExtension::new(
        "",
        "operation",
        vec![id(2)],
        vec![],
    );

    assert!(matches!(
        result,
        Err(ExpressionError::EmptyExtensionNamespace)
    ));
}

#[test]
fn unrestricted_validation_policy_has_no_implicit_limits() {
    let policy = ExpressionValidationPolicy::unrestricted();

    assert_eq!(policy.max_children, None);
    assert_eq!(policy.max_extension_namespace_bytes, None);
    assert_eq!(policy.max_extension_name_bytes, None);
    assert_eq!(policy.max_operator_namespace_bytes, None);
    assert_eq!(policy.max_operator_name_bytes, None);

    assert!(policy.validate().is_ok());
}

#[test]
fn explicit_validation_budget_is_distinct_from_unrestricted_policy() {
    let unrestricted = ExpressionValidationPolicy::unrestricted();
    let bounded = ExpressionValidationPolicy::bounded(1);

    assert_eq!(unrestricted.max_children, None);
    assert_eq!(bounded.max_children, Some(1));
    assert!(bounded.validate().is_ok());
}

#[test]
fn zero_child_budget_is_rejected_as_invalid_policy() {
    assert!(
        ExpressionValidationPolicy::bounded(0)
            .validate()
            .is_err()
    );
}

#[test]
fn explicit_child_budget_reports_excess_children() {
    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    let node = binary_expression(1, 2, operator, 3);

    let result = validate_expression(
        &node,
        ExpressionValidationPolicy::bounded(1),
    );

    assert!(
        matches!(
            result,
            Err(ExpressionError::LimitExceeded {
                limit: "max_children",
                actual: 2,
                maximum: 1,
            })
        ),
        "the caller-supplied budget should be reported precisely"
    );
}

#[test]
fn an_explicit_budget_does_not_change_the_unrestricted_contract() {
    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    let node = binary_expression(1, 2, operator, 3);

    assert!(
        validate_expression(&node, ExpressionValidationPolicy::unrestricted())
            .is_ok()
    );

    assert!(
        validate_expression(
            &node,
            ExpressionValidationPolicy::bounded(1)
        )
        .is_err()
    );
}

// ---------------------------------------------------------------------------
// Operator registry: extensibility and deterministic resolution
// ---------------------------------------------------------------------------

#[test]
fn operator_registry_starts_empty() {
    let registry = OperatorRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn operator_registry_registers_and_resolves_canonical_identity() {
    let mut registry = OperatorRegistry::new();

    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    registry
        .register(OperatorFixity::Infix, "+", operator.clone())
        .expect("first registration should succeed");

    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry.resolve(OperatorFixity::Infix, "+"),
        Some(&operator)
    );
}

#[test]
fn identical_operator_registration_is_idempotent() {
    let mut registry = OperatorRegistry::new();

    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    registry
        .register(OperatorFixity::Infix, "+", operator.clone())
        .expect("initial registration should succeed");

    registry
        .register(OperatorFixity::Infix, "+", operator.clone())
        .expect("identical registration should be idempotent");

    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry.resolve(OperatorFixity::Infix, "+"),
        Some(&operator)
    );
}

#[test]
fn conflicting_operator_registration_is_not_silent() {
    let mut registry = OperatorRegistry::new();

    registry
        .register(
            OperatorFixity::Infix,
            "+",
            OperatorRef::native("add")
                .expect("operator name should be valid"),
        )
        .expect("initial registration should succeed");

    let result = registry.register(
        OperatorFixity::Infix,
        "+",
        OperatorRef::native("concatenate")
            .expect("operator name should be valid"),
    );

    assert!(
        matches!(
            result,
            Err(OperatorRegistryError::RegistrationConflict { .. })
        ),
        "conflicting meanings must not silently overwrite one another"
    );
}

#[test]
fn the_same_spelling_can_have_different_fixities() {
    let mut registry = OperatorRegistry::new();

    let prefix = OperatorRef::native("negate")
        .expect("operator name should be valid");

    let infix = OperatorRef::native("subtract")
        .expect("operator name should be valid");

    registry
        .register(OperatorFixity::Prefix, "-", prefix.clone())
        .expect("prefix registration should succeed");

    registry
        .register(OperatorFixity::Infix, "-", infix.clone())
        .expect("infix registration should succeed");

    assert_eq!(
        registry.resolve(OperatorFixity::Prefix, "-"),
        Some(&prefix)
    );

    assert_eq!(
        registry.resolve(OperatorFixity::Infix, "-"),
        Some(&infix)
    );

    assert_eq!(registry.len(), 2);
}

#[test]
fn dialect_specific_operator_takes_precedence_over_default() {
    let mut registry = OperatorRegistry::new();

    let default_operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    let domain_operator = OperatorRef::new("example.domain", "combine")
        .expect("namespaced operator should be valid");

    registry
        .register(OperatorFixity::Infix, "+", default_operator.clone())
        .expect("default registration should succeed");

    registry
        .register_in_dialect(
            "example.domain",
            OperatorFixity::Infix,
            "+",
            domain_operator.clone(),
        )
        .expect("dialect registration should succeed");

    assert_eq!(
        registry.resolve(OperatorFixity::Infix, "+"),
        Some(&default_operator)
    );

    assert_eq!(
        registry.resolve_in_dialect(
            "example.domain",
            OperatorFixity::Infix,
            "+"
        ),
        Some(&domain_operator)
    );
}

#[test]
fn unknown_operator_lookup_is_explicit() {
    let registry = OperatorRegistry::new();

    assert!(
        registry
            .resolve(OperatorFixity::Infix, "unknown")
            .is_none()
    );

    assert!(
        matches!(
            registry.require(OperatorFixity::Infix, "unknown"),
            Err(OperatorRegistryError::UnknownOperator { .. })
        )
    );
}

#[test]
fn invalid_operator_spelling_is_not_registered() {
    let mut registry = OperatorRegistry::new();

    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    let result = registry.register(
        OperatorFixity::Infix,
        "",
        operator,
    );

    assert!(
        matches!(result, Err(OperatorRegistryError::EmptySpelling))
    );

    assert!(registry.is_empty());
}

#[test]
fn operator_registry_removal_is_scoped_to_exact_fixity() {
    let mut registry = OperatorRegistry::new();

    let prefix = OperatorRef::native("negate")
        .expect("operator name should be valid");

    let infix = OperatorRef::native("subtract")
        .expect("operator name should be valid");

    registry
        .register(OperatorFixity::Prefix, "-", prefix.clone())
        .expect("prefix registration should succeed");

    registry
        .register(OperatorFixity::Infix, "-", infix.clone())
        .expect("infix registration should succeed");

    assert_eq!(
        registry.remove(OperatorFixity::Prefix, "-"),
        Some(prefix)
    );

    assert_eq!(
        registry.resolve(OperatorFixity::Prefix, "-"),
        None
    );

    assert_eq!(
        registry.resolve(OperatorFixity::Infix, "-"),
        Some(&infix)
    );
}

// ---------------------------------------------------------------------------
// Evaluation order: structural plan, not execution
// ---------------------------------------------------------------------------

#[test]
fn literal_expression_has_no_runtime_children_in_its_plan() {
    let kind = ExpressionKind::Literal { literal: id(2) };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::NoRuntimeChildren
    );
}

#[test]
fn binary_expression_defaults_to_eager_left_to_right_without_semantic_rule() {
    let operator = OperatorRef::native("add")
        .expect("operator name should be valid");

    let kind = ExpressionKind::Binary {
        left: id(2),
        operator: operator.clone(),
        right: id(3),
    };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::Binary {
            left: id(2),
            operator,
            right: id(3),
            evaluation: BinaryEvaluation::EagerLeftToRight,
        }
    );
}

#[test]
fn short_circuiting_requires_an_explicit_semantic_classifier() {
    let operator = OperatorRef::new("zamani", "logical-and")
        .expect("operator identity should be valid");

    let kind = ExpressionKind::Binary {
        left: id(2),
        operator: operator.clone(),
        right: id(3),
    };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::Binary {
            left: id(2),
            operator: operator.clone(),
            right: id(3),
            evaluation: BinaryEvaluation::EagerLeftToRight,
        }
    );

    assert_eq!(
        plan_expression(&kind, &logical_classifier),
        EvaluationPlan::Binary {
            left: id(2),
            operator,
            right: id(3),
            evaluation: BinaryEvaluation::ShortCircuit(
                ShortCircuitKind::And
            ),
        }
    );
}

#[test]
fn operator_spelling_alone_does_not_force_short_circuiting() {
    let operator = OperatorRef::new("user.extension", "logical-and")
        .expect("operator identity should be valid");

    let kind = ExpressionKind::Binary {
        left: id(2),
        operator: operator.clone(),
        right: id(3),
    };

    assert_eq!(
        plan_expression(&kind, &logical_classifier),
        EvaluationPlan::Binary {
            left: id(2),
            operator,
            right: id(3),
            evaluation: BinaryEvaluation::EagerLeftToRight,
        }
    );
}

#[test]
fn conditional_plan_keeps_branches_separate() {
    let kind = ExpressionKind::Conditional {
        condition: id(2),
        then_branch: id(3),
        else_branch: Some(id(4)),
    };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::Conditional {
            condition: id(2),
            then_branch: id(3),
            else_branch: Some(id(4)),
        }
    );
}

#[test]
fn call_plan_preserves_callee_and_argument_order() {
    let kind = ExpressionKind::Call {
        callee: id(2),
        arguments: vec![id(3), id(4), id(5)],
    };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::Call {
            callee: id(2),
            arguments: vec![id(3), id(4), id(5)],
        }
    );
}

#[test]
fn assignment_plan_does_not_rewrite_assignment_as_an_eager_binary() {
    let kind = ExpressionKind::Assignment {
        target: id(2),
        value: id(3),
    };

    assert_eq!(
        plan_expression(&kind, &no_short_circuit),
        EvaluationPlan::Assignment {
            target: id(2),
            value: id(3),
        }
    );
}

#[test]
fn extension_expression_requires_an_extension_evaluation_contract() {
    let extension =
        crate::classical::expressions::ExpressionExtension::new(
            "example.domain",
            "operation",
            vec![id(2), id(3)],
            vec![],
        )
        .expect("extension identity should be valid");

    let kind = ExpressionKind::Extension(extension.clone());

    let plan = plan_expression(&kind, &no_short_circuit);

    assert_eq!(
        plan,
        EvaluationPlan::ExtensionDefined { extension }
    );

    assert!(
        crate::classical::expressions::requires_extension_evaluation_contract(
            &plan
        )
    );
}

#[test]
fn ordinary_expression_plan_does_not_require_extension_contract() {
    let kind = ExpressionKind::Binary {
        left: id(2),
        operator: OperatorRef::native("add")
            .expect("operator name should be valid"),
        right: id(3),
    };

    let plan = plan_expression(&kind, &no_short_circuit);

    assert!(
        !crate::classical::expressions::requires_extension_evaluation_contract(
            &plan
        )
    );
}

// ---------------------------------------------------------------------------
// Graph traversal: deterministic order, missing nodes, cycle protection
// ---------------------------------------------------------------------------

#[test]
fn graph_walk_visits_root_and_its_resolvable_expression_children() {
    // A block's children are expression-node IDs. Use block expressions so
    // the test graph contains only expression nodes, matching the resolver's
    // documented return type.
    let root = expression(
        1,
        ExpressionKind::Block {
            statements: vec![id(2), id(3)],
        },
    );

    let first = expression(
        2,
        ExpressionKind::Block {
            statements: vec![],
        },
    );

    let second = expression(
        3,
        ExpressionKind::Block {
            statements: vec![],
        },
    );

    let mut graph = BTreeMap::new();
    graph.insert(root.id(), root);
    graph.insert(first.id(), first);
    graph.insert(second.id(), second);

    let mut visited = Vec::new();

    let result = walk_expression_graph(
        id(1),
        |node_id| graph.get(&node_id),
        |node| {
            visited.push(node.id());
            ControlFlow::<()>::Continue(())
        },
    );

    assert!(matches!(result, Ok(ControlFlow::Continue(()))));
    assert_eq!(visited, vec![id(1), id(2), id(3)]);
}

#[test]
fn graph_walk_reports_missing_root() {
    let result = walk_expression_graph(
        id(999),
        |_| None,
        |_| ControlFlow::<()>::Continue(()),
    );

    assert!(matches!(
        result,
        Err(ExpressionWalkError::MissingNode { id: missing })
            if missing == id(999)
    ));
}

#[test]
fn graph_walk_stops_when_visitor_requests_break() {
    let root = expression(
        1,
        ExpressionKind::Block {
            statements: vec![id(2)],
        },
    );

    let child = expression(
        2,
        ExpressionKind::Block {
            statements: vec![],
        },
    );

    let mut graph = BTreeMap::new();
    graph.insert(root.id(), root);
    graph.insert(child.id(), child);

    let mut visited = Vec::new();

    let result = walk_expression_graph(
        id(1),
        |node_id| graph.get(&node_id),
        |node| {
            visited.push(node.id());

            if node.id() == id(1) {
                ControlFlow::Break(node.id())
            } else {
                ControlFlow::Continue(())
            }
        },
    );

    assert_eq!(result, Ok(ControlFlow::Break(id(1))));
    assert_eq!(visited, vec![id(1)]);
}

#[test]
fn graph_walk_does_not_loop_forever_on_a_cycle() {
    let root = expression(
        1,
        ExpressionKind::Block {
            statements: vec![id(1)],
        },
    );

    let mut graph = BTreeMap::new();
    graph.insert(root.id(), root);

    let mut visited = Vec::new();

    let result = walk_expression_graph(
        id(1),
        |node_id| graph.get(&node_id),
        |node| {
            visited.push(node.id());
            ControlFlow::<()>::Continue(())
        },
    );

    assert!(matches!(result, Ok(ControlFlow::Continue(()))));
    assert_eq!(visited, vec![id(1)]);
}

// ---------------------------------------------------------------------------
// Determinism and portability contracts
// ---------------------------------------------------------------------------

#[test]
fn operator_registry_lookup_is_stable_across_insertion_orders() {
    let entries = [
        (
            OperatorFixity::Infix,
            "+",
            OperatorRef::native("add").expect("valid operator"),
        ),
        (
            OperatorFixity::Prefix,
            "-",
            OperatorRef::native("negate").expect("valid operator"),
        ),
        (
            OperatorFixity::Infix,
            "*",
            OperatorRef::native("multiply").expect("valid operator"),
        ),
    ];

    let mut forward = OperatorRegistry::new();
    for (fixity, spelling, operator) in entries.iter().cloned() {
        forward
            .register(fixity, spelling, operator)
            .expect("registration should succeed");
    }

    let mut reverse = OperatorRegistry::new();
    for (fixity, spelling, operator) in entries.iter().rev().cloned() {
        reverse
            .register(fixity, spelling, operator)
            .expect("registration should succeed");
    }

    for (fixity, spelling, _) in entries {
        assert_eq!(
            forward.resolve(fixity, spelling),
            reverse.resolve(fixity, spelling)
        );
    }
}

#[test]
fn expression_identity_is_not_a_hardware_or_runtime_identity() {
    let node = identifier_expression(42, 84);

    assert_eq!(node.id().get(), 42);
    assert_eq!(node.child_node_ids(), vec![id(84)]);

    // This verifies the source AST identity contract only. It does not
    // assign meaning to IDs as addresses, registers, devices, or IR values.
    assert_ne!(node.id(), id(84));
}

#[test]
fn public_expression_contract_remains_target_independent() {
    // A namespaced operator can be represented without selecting a target,
    // vendor, device, register width, or execution strategy.
    let operator = OperatorRef::new(
        "future.domain",
        "future-operation",
    )
    .expect("namespaced operator identity should be valid");

    assert_eq!(operator.namespace(), "future.domain");
    assert_eq!(operator.name(), "future-operation");
    assert_eq!(
        operator.qualified_name(),
        "future.domain:future-operation"
    );
}
