
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration tests for Zamani's native semantic effect subsystem.
//!
//! File: `src/frontend/semantic/tests/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! PURPOSE
//!
//! Verify that the canonical effect model, inference, declared-effect
//! compatibility, and authorization APIs compose correctly.
//!
//! AUTHORITY
//!
//! - `grammar/spec/effects.md`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-composition.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/semantic/effects/inference.rs`
//! - `src/frontend/semantic/effects/checking.rs`
//! - `src/frontend/semantic/effects/authorization.rs`
//!
//! SCOPE
//!
//! These tests verify semantic effect APIs. They do not replace parser
//! conformance tests, policy-language resolution tests, full frontend
//! pipeline tests, or target-feasibility tests.
//!
//! SCALABILITY
//!
//! Tests use dynamically generated effect identities and graph nodes. Fixture
//! sizes are test workload choices, not language or hardware limits.
//! Production analysis budgets belong to the semantic session/pipeline.
//!
//! INTEGRATION
//!
//! Declare this file from `src/frontend/semantic/tests/mod.rs`, and declare
//! the test module from the semantic root. See the integration notes following
//! this file.

use std::collections::BTreeSet;

use crate::frontend::ast::node::effects::effect_kind::EffectKind;
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::effects::{
    check_effect_compatibility,
    check_effect_compatibility_with,
    infer_effects,
    AuthorizationDecision,
    AuthorizationDecisionSource,
    AuthorizationRule,
    ConstantKey,
    DeclarationKey,
    EffectArgument,
    EffectAuthorizationPolicy,
    EffectCheckIssue,
    EffectCheckingPolicy,
    EffectDependency,
    EffectInferenceError,
    EffectInferenceGraph,
    EffectInstance,
    EffectKind as _,
    EffectPropagation,
    EffectRow,
    EffectSet,
    EffectVariable,
    ExactEffectRelation,
    TypeKey,
};

// -----------------------------------------------------------------------------
// Shared test helpers
// -----------------------------------------------------------------------------

fn kind(namespace: &str, name: &str) -> EffectKind {
    EffectKind::new(namespace, name)
        .expect("test effect identity must be structurally valid")
}

fn effect(namespace: &str, name: &str) -> EffectInstance {
    EffectInstance::simple(kind(namespace, name))
}

fn row(effects: impl IntoIterator<Item = EffectInstance>) -> EffectRow {
    EffectRow::closed(effects)
        .expect("test effect row must be structurally valid")
}

fn open_row(
    effects: impl IntoIterator<Item = EffectInstance>,
    variables: impl IntoIterator<Item = EffectVariable>,
) -> EffectRow {
    EffectRow::new(effects, variables)
        .expect("test open effect row must be structurally valid")
}

fn variable(name: &str) -> EffectVariable {
    EffectVariable::new(name)
        .expect("test effect variable must be structurally valid")
}

fn node_id(value: u64) -> NodeId {
    NodeId::new(value).expect("test AST node identity must be nonzero")
}

fn graph_node(
    graph: &mut EffectInferenceGraph,
    id: u64,
    local_effects: EffectRow,
) {
    graph
        .add_node(node_id(id), local_effects)
        .expect("test graph node must be valid and unique");
}

fn allow_rule(policy_id: &str) -> AuthorizationRule {
    AuthorizationRule::allow(policy_id)
        .expect("test allow rule must be valid")
}

fn deny_rule(policy_id: &str) -> AuthorizationRule {
    AuthorizationRule::deny(policy_id)
        .expect("test deny rule must be valid")
}

// -----------------------------------------------------------------------------
// Canonical effect identity and effect model
// -----------------------------------------------------------------------------

#[test]
fn effect_identity_is_open_world_across_computational_domains() {
    let identities = [
        kind("classical", "io"),
        kind("quantum", "measurement"),
        kind("hdl", "signal_access"),
        kind("hybrid", "boundary_crossing"),
        kind("distributed", "communication"),
        kind("accelerator", "tensor_compute"),
        kind("future_domain", "new_operation"),
    ];

    let distinct: BTreeSet<_> = identities.iter().cloned().collect();

    assert_eq!(distinct.len(), identities.len());
    assert_eq!(identities[1].to_string(), "quantum:measurement");
    assert_eq!(identities[2].to_string(), "hdl:signal_access");
    assert_eq!(identities[6].to_string(), "future_domain:new_operation");
}

#[test]
fn effect_identity_preserves_case_and_namespace() {
    assert_ne!(kind("example", "Read"), kind("example", "read"));
    assert_ne!(kind("domain_a", "operation"), kind("domain_b", "operation"));
}

#[test]
fn effect_instances_distinguish_resolved_arguments() {
    let effect_kind = kind("example", "operation");

    let first = EffectInstance::new(
        effect_kind.clone(),
        vec![EffectArgument::symbolic("shape_a").unwrap()],
    )
    .unwrap();

    let second = EffectInstance::new(
        effect_kind,
        vec![EffectArgument::symbolic("shape_b").unwrap()],
    )
    .unwrap();

    assert_ne!(first, second);
}

#[test]
fn effect_arguments_keep_their_canonical_categories() {
    let type_key = TypeKey::new("type:canonical-id").unwrap();
    let declaration_key = DeclarationKey::new("declaration:canonical-id").unwrap();
    let constant_key = ConstantKey::new("constant:symbolic-size").unwrap();

    let arguments = [
        EffectArgument::Type(type_key),
        EffectArgument::Declaration(declaration_key),
        EffectArgument::Constant(constant_key),
        EffectArgument::symbolic("symbolic_dimension").unwrap(),
    ];

    for argument in &arguments {
        argument
            .validate()
            .expect("constructed effect arguments must validate");
    }

    // Distinct argument categories must not collapse to the same identity.
    for left in 0..arguments.len() {
        for right in (left + 1)..arguments.len() {
            assert_ne!(arguments[left], arguments[right]);
        }
    }
}

#[test]
fn malformed_effect_arguments_are_rejected() {
    assert!(EffectArgument::symbolic("").is_err());
    assert!(EffectArgument::symbolic("bad\0value").is_err());
    assert!(TypeKey::new("").is_err());
    assert!(DeclarationKey::new("").is_err());
    assert!(ConstantKey::new("").is_err());
}

#[test]
fn effect_sets_deduplicate_exact_instances_only() {
    let first = effect("example", "operation");
    let second = EffectInstance::new(
        kind("example", "operation"),
        vec![EffectArgument::symbolic("parameter").unwrap()],
    )
    .unwrap();

    let set = EffectSet::from_effects([
        first.clone(),
        first.clone(),
        second.clone(),
    ])
    .unwrap();

    assert_eq!(set.len(), 2);
    assert!(set.contains(&first));
    assert!(set.contains(&second));
}

#[test]
fn effect_set_operations_have_structural_set_semantics() {
    let a = effect("example", "a");
    let b = effect("example", "b");
    let c = effect("example", "c");

    let left = EffectSet::from_effects([a.clone(), b.clone()]).unwrap();
    let right = EffectSet::from_effects([b.clone(), c.clone()]).unwrap();

    let union = left.union(&right);
    let intersection = left.intersection(&right);
    let difference = left.difference(&right);

    assert_eq!(union.len(), 3);
    assert_eq!(intersection.len(), 1);
    assert!(intersection.contains(&b));
    assert_eq!(difference.len(), 1);
    assert!(difference.contains(&a));
    assert!(left.is_subset_of(&union));
}

#[test]
fn empty_effect_row_is_empty_without_claiming_source_purity() {
    let empty = EffectRow::empty();

    assert!(empty.is_empty());
    assert!(empty.is_closed());
    assert_eq!(empty.effect_count(), 0);
    assert_eq!(empty.variable_count(), 0);
    assert_eq!(empty.to_string(), "{}");

    // The row API represents effects; it does not independently establish
    // every other semantic property of a source computation.
}

#[test]
fn open_effect_rows_preserve_unresolved_variables() {
    let row = open_row(
        [effect("example", "io")],
        [variable("e"), variable("f")],
    );

    assert!(!row.is_closed());
    assert_eq!(row.effect_count(), 1);
    assert_eq!(row.variable_count(), 2);
    assert_eq!(
        row.variables().map(EffectVariable::name).collect::<Vec<_>>(),
        vec!["e", "f"]
    );
}

#[test]
fn duplicate_effect_row_variables_are_rejected() {
    let e = variable("e");

    let result = EffectRow::new([], [e.clone(), e]);

    assert!(result.is_err());
}

#[test]
fn large_effect_names_are_not_rejected_by_an_arbitrary_domain_limit() {
    // A bounded fixture exercises long identifiers without treating the
    // fixture length as a language maximum.
    let namespace = format!("domain_{}", "n".repeat(4096));
    let name = format!("operation_{}", "x".repeat(4096));

    let identity = kind(&namespace, &name);

    assert_eq!(identity.namespace(), namespace);
    assert_eq!(identity.name(), name);
}

// -----------------------------------------------------------------------------
// Effect inference
// -----------------------------------------------------------------------------

#[test]
fn inference_propagates_effects_from_dependency_to_dependent() {
    let io = effect("classical", "io");
    let mut graph = EffectInferenceGraph::new();

    graph_node(&mut graph, 1, EffectRow::empty());
    graph_node(&mut graph, 2, row([io.clone()]));

    graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap();

    let inferred = infer_effects(&graph).unwrap();

    assert_eq!(inferred.get(node_id(1)).unwrap(), &row([io.clone()]));
    assert_eq!(inferred.get(node_id(2)).unwrap(), &row([io]));
}

#[test]
fn latent_dependency_does_not_execute_the_dependency_effects() {
    let io = effect("classical", "io");
    let mut graph = EffectInferenceGraph::new();

    graph_node(&mut graph, 1, EffectRow::empty());
    graph_node(&mut graph, 2, row([io.clone()]));

    graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Latent)
        .unwrap();

    let inferred = infer_effects(&graph).unwrap();

    assert_eq!(inferred.get(node_id(1)), Some(&EffectRow::empty()));
    assert_eq!(inferred.get(node_id(2)), Some(&row([io])));
}

#[test]
fn inference_handles_recursive_dependency_cycles() {
    let io = effect("classical", "io");
    let measurement = effect("quantum", "measurement");
    let mut graph = EffectInferenceGraph::new();

    graph_node(&mut graph, 1, row([io.clone()]));
    graph_node(&mut graph, 2, row([measurement.clone()]));
    graph_node(&mut graph, 3, EffectRow::empty());

    graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap();
    graph
        .add_dependency(node_id(2), node_id(3), EffectPropagation::Propagates)
        .unwrap();
    graph
        .add_dependency(node_id(3), node_id(1), EffectPropagation::Propagates)
        .unwrap();

    let expected = row([io, measurement]);
    let inferred = infer_effects(&graph).unwrap();

    for id in [1, 2, 3] {
        assert_eq!(inferred.get(node_id(id)), Some(&expected));
    }
}

#[test]
fn inference_is_independent_of_node_insertion_order() {
    fn build(order: &[u64]) -> EffectInferenceGraph {
        let mut graph = EffectInferenceGraph::new();

        for id in order {
            let local = match id {
                1 => row([effect("example", "a")]),
                2 => row([effect("example", "b")]),
                _ => EffectRow::empty(),
            };

            graph_node(&mut graph, *id, local);
        }

        graph
            .add_dependency(node_id(3), node_id(1), EffectPropagation::Propagates)
            .unwrap();
        graph
            .add_dependency(node_id(3), node_id(2), EffectPropagation::Propagates)
            .unwrap();

        graph
    }

    let first = infer_effects(&build(&[1, 2, 3])).unwrap();
    let second = infer_effects(&build(&[3, 2, 1])).unwrap();

    assert_eq!(first, second);
}

#[test]
fn inference_rejects_duplicate_node_identity() {
    let mut graph = EffectInferenceGraph::new();

    graph.add_node(node_id(1), EffectRow::empty()).unwrap();

    assert!(matches!(
        graph.add_node(node_id(1), EffectRow::empty()),
        Err(EffectInferenceError::DuplicateNode { .. })
    ));
}

#[test]
fn inference_rejects_a_missing_dependency_target() {
    let mut graph = EffectInferenceGraph::new();

    graph.add_node(node_id(1), EffectRow::empty()).unwrap();
    graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap();

    assert!(matches!(
        infer_effects(&graph),
        Err(EffectInferenceError::MissingNode { .. })
    ));
}

#[test]
fn inference_rejects_a_missing_dependency_source() {
    let mut graph = EffectInferenceGraph::new();

    let result = graph.add_dependency(
        node_id(1),
        node_id(2),
        EffectPropagation::Propagates,
    );

    assert!(matches!(
        result,
        Err(EffectInferenceError::MissingSourceNode { .. })
    ));
}

#[test]
fn inference_is_idempotent_for_repeated_dependency_insertion() {
    let mut graph = EffectInferenceGraph::new();

    graph_node(&mut graph, 1, EffectRow::empty());
    graph_node(&mut graph, 2, row([effect("example", "io")]));

    assert!(graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap());

    assert!(!graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap());

    let result = infer_effects(&graph).unwrap();

    assert_eq!(
        result.get(node_id(1)),
        Some(&row([effect("example", "io")]))
    );
}

#[test]
fn inference_scales_with_a_generated_dependency_graph() {
    // This is a finite stress fixture, not a semantic maximum.
    let node_count: u64 = 512;
    let mut graph = EffectInferenceGraph::new();

    graph_node(
        &mut graph,
        1,
        row([effect("future_domain", "generated_effect")]),
    );

    for id in 2..=node_count {
        graph_node(&mut graph, id, EffectRow::empty());
        graph
            .add_dependency(
                node_id(id),
                node_id(id - 1),
                EffectPropagation::Propagates,
            )
            .unwrap();
    }

    let result = infer_effects(&graph).unwrap();

    assert_eq!(result.len(), node_count as usize);

    let expected = row([effect("future_domain", "generated_effect")]);

    for id in 1..=node_count {
        assert_eq!(result.get(node_id(id)), Some(&expected));
    }
}

// -----------------------------------------------------------------------------
// Declared-effect compatibility
// -----------------------------------------------------------------------------

#[test]
fn closed_declaration_accepts_a_subset_of_declared_effects() {
    let actual = row([effect("example", "read")]);
    let declared = row([
        effect("example", "read"),
        effect("example", "write"),
    ]);

    let report = check_effect_compatibility(&actual, &declared).unwrap();

    assert!(report.is_compatible());
    assert!(!report.has_issues());
}

#[test]
fn closed_declaration_rejects_an_undeclared_effect() {
    let actual = row([effect("example", "network")]);
    let declared = row([effect("example", "io")]);

    let report = check_effect_compatibility(&actual, &declared).unwrap();

    assert!(!report.is_compatible());
    assert_eq!(report.issues().len(), 1);
    assert!(matches!(
        report.issues()[0],
        EffectCheckIssue::UndeclaredEffect { .. }
    ));
}

#[test]
fn empty_declaration_does_not_allow_arbitrary_effects() {
    let actual = row([effect("future_domain", "operation")]);

    let report =
        check_effect_compatibility(&actual, &EffectRow::empty()).unwrap();

    assert!(!report.is_compatible());
}

#[test]
fn effect_variables_must_match_by_identity() {
    let actual = open_row([], [variable("actual")]);
    let declared = open_row([], [variable("declared")]);

    let report = check_effect_compatibility(&actual, &declared).unwrap();

    assert!(!report.is_compatible());
    assert!(matches!(
        report.issues()[0],
        EffectCheckIssue::UndeclaredEffectVariable { .. }
    ));
}

#[test]
fn matching_open_row_variables_are_compatible() {
    let actual = open_row(
        [effect("example", "io")],
        [variable("e")],
    );
    let declared = open_row(
        [effect("example", "io"), effect("example", "network")],
        [variable("e")],
    );

    let report = check_effect_compatibility(&actual, &declared).unwrap();

    assert!(report.is_compatible());
}

#[test]
fn concrete_effect_widening_requires_explicit_opt_in() {
    let actual = row([effect("example", "network")]);
    let declared = open_row([], [variable("e")]);

    let strict = check_effect_compatibility(&actual, &declared).unwrap();
    assert!(!strict.is_compatible());
    assert!(!strict.widened());

    let widened = check_effect_compatibility_with(
        &actual,
        &declared,
        EffectCheckingPolicy::allow_widening_for_open_rows(),
        &ExactEffectRelation,
    )
    .unwrap();

    assert!(widened.is_compatible());
    assert!(widened.widened());
    assert_eq!(widened.widening().len(), 1);
}

#[test]
fn open_row_widening_does_not_hide_unmatched_effect_variables() {
    let actual = open_row([], [variable("unresolved")]);
    let declared = open_row([], [variable("declared")]);

    let report = check_effect_compatibility_with(
        &actual,
        &declared,
        EffectCheckingPolicy::allow_widening_for_open_rows(),
        &ExactEffectRelation,
    )
    .unwrap();

    assert!(!report.is_compatible());
    assert!(report.issues().iter().any(|issue| matches!(
        issue,
        EffectCheckIssue::UndeclaredEffectVariable { .. }
    )));
}

// -----------------------------------------------------------------------------
// Authorization and policy behavior
// -----------------------------------------------------------------------------

#[test]
fn authorization_defaults_to_deny() {
    let policy = EffectAuthorizationPolicy::new();
    let report = policy
        .authorize(&row([effect("example", "io")]))
        .unwrap();

    assert!(!report.is_authorized());
    assert!(report.has_denials());
    assert_eq!(report.denied_count(), 1);
    assert_eq!(report.evaluated_count(), 1);
    assert_eq!(
        report.decisions()[0].source(),
        AuthorizationDecisionSource::DefaultDeny
    );
    assert_eq!(
        report.decisions()[0].decision(),
        AuthorizationDecision::Deny
    );
}

#[test]
fn an_explicit_kind_allow_rule_authorizes_matching_effects() {
    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(kind("classical", "io"), allow_rule("policy-io"))
        .unwrap();

    let report = policy
        .authorize(&row([
            effect("classical", "io"),
            effect("classical", "io"),
        ]))
        .unwrap();

    // The row is a set: repeated identical effects normalize to one instance.
    assert!(report.is_authorized());
    assert_eq!(report.evaluated_count(), 1);
    assert_eq!(
        report.decisions()[0].source(),
        AuthorizationDecisionSource::EffectKindRule
    );
    assert_eq!(report.decisions()[0].policy_id(), Some("policy-io"));
}

#[test]
fn deny_rule_overrides_a_kind_level_allow() {
    let allowed = effect("quantum", "measurement");
    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(
            kind("quantum", "measurement"),
            allow_rule("quantum-allow"),
        )
        .unwrap();
    policy
        .add_instance_rule(allowed.clone(), deny_rule("measurement-deny"))
        .unwrap();

    let report = policy.authorize(&row([allowed])).unwrap();

    assert!(!report.is_authorized());
    assert_eq!(report.denied_count(), 1);
    assert_eq!(
        report.decisions()[0].decision(),
        AuthorizationDecision::Deny
    );
    assert_eq!(
        report.decisions()[0].source(),
        AuthorizationDecisionSource::ExactInstanceRule
    );
    assert_eq!(
        report.decisions()[0].policy_id(),
        Some("measurement-deny")
    );
}

#[test]
fn kind_level_deny_overrides_an_exact_instance_allow() {
    let allowed_instance = EffectInstance::new(
        kind("example", "operation"),
        vec![EffectArgument::symbolic("approved_case").unwrap()],
    )
    .unwrap();

    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(
            kind("example", "operation"),
            deny_rule("broad-deny"),
        )
        .unwrap();
    policy
        .add_instance_rule(
            allowed_instance.clone(),
            allow_rule("specific-allow"),
        )
        .unwrap();

    let report = policy.authorize(&row([allowed_instance])).unwrap();

    assert!(!report.is_authorized());
    assert_eq!(
        report.decisions()[0].source(),
        AuthorizationDecisionSource::EffectKindRule
    );
    assert_eq!(
        report.decisions()[0].policy_id(),
        Some("broad-deny")
    );
}

#[test]
fn exact_instance_allow_is_more_specific_when_no_deny_applies() {
    let allowed_instance = EffectInstance::new(
        kind("example", "operation"),
        vec![EffectArgument::symbolic("approved_case").unwrap()],
    )
    .unwrap();

    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(
            kind("example", "operation"),
            allow_rule("kind-allow"),
        )
        .unwrap();
    policy
        .add_instance_rule(
            allowed_instance.clone(),
            allow_rule("instance-allow"),
        )
        .unwrap();

    let report = policy.authorize(&row([allowed_instance])).unwrap();

    assert!(report.is_authorized());
    assert_eq!(
        report.decisions()[0].source(),
        AuthorizationDecisionSource::ExactInstanceRule
    );
    assert_eq!(
        report.decisions()[0].policy_id(),
        Some("instance-allow")
    );
}

#[test]
fn duplicate_authorization_rules_are_rejected() {
    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(kind("example", "io"), allow_rule("first"))
        .unwrap();

    assert!(policy
        .add_kind_rule(kind("example", "io"), deny_rule("second"))
        .is_err());

    let instance = effect("example", "operation");

    policy
        .add_instance_rule(instance.clone(), allow_rule("instance-first"))
        .unwrap();

    assert!(policy
        .add_instance_rule(instance, deny_rule("instance-second"))
        .is_err());
}

#[test]
fn unresolved_effect_variables_are_not_authorized_as_pure() {
    let policy = EffectAuthorizationPolicy::new();
    let report = policy
        .authorize(&open_row([], [variable("unknown_effects")]))
        .unwrap();

    assert!(!report.is_authorized());
    assert!(!report.has_denials());
    assert!(report.has_unresolved_variables());
    assert_eq!(report.unresolved_variables().len(), 1);
}

#[test]
fn authorization_reports_are_deterministic() {
    let mut policy = EffectAuthorizationPolicy::new();

    policy
        .add_kind_rule(kind("example", "a"), allow_rule("allow-a"))
        .unwrap();
    policy
        .add_kind_rule(kind("example", "b"), deny_rule("deny-b"))
        .unwrap();

    let first = row([
        effect("example", "b"),
        effect("example", "a"),
    ]);
    let second = row([
        effect("example", "a"),
        effect("example", "b"),
    ]);

    assert_eq!(
        policy.authorize(&first).unwrap(),
        policy.authorize(&second).unwrap()
    );
}

// -----------------------------------------------------------------------------
// End-to-end composition of inference, checking, and authorization
// -----------------------------------------------------------------------------

#[test]
fn inferred_effects_flow_through_contract_checking_and_authorization() {
    let io = effect("classical", "io");
    let measurement = effect("quantum", "measurement");

    let mut graph = EffectInferenceGraph::new();
    graph_node(&mut graph, 1, EffectRow::empty());
    graph_node(&mut graph, 2, row([io.clone()]));
    graph_node(&mut graph, 3, row([measurement.clone()]));

    graph
        .add_dependency(node_id(1), node_id(2), EffectPropagation::Propagates)
        .unwrap();
    graph
        .add_dependency(node_id(1), node_id(3), EffectPropagation::Propagates)
        .unwrap();

    let inferred = infer_effects(&graph).unwrap();
    let actual = inferred.get(node_id(1)).unwrap();

    let declared = row([io.clone(), measurement.clone()]);
    let compatibility = check_effect_compatibility(actual, &declared).unwrap();

    assert!(compatibility.is_compatible());

    let mut policy = EffectAuthorizationPolicy::new();
    policy
        .add_kind_rule(kind("classical", "io"), allow_rule("allow-io"))
        .unwrap();
    policy
        .add_kind_rule(
            kind("quantum", "measurement"),
            allow_rule("allow-measurement"),
        )
        .unwrap();

    let authorization = policy.authorize(actual).unwrap();

    assert!(authorization.is_authorized());
    assert_eq!(authorization.evaluated_count(), 2);
}

#[test]
fn a_compatible_effect_contract_does_not_imply_authorization() {
    let actual = row([effect("distributed", "communication")]);
    let declared = row([effect("distributed", "communication")]);

    let compatibility = check_effect_compatibility(&actual, &declared).unwrap();
    assert!(compatibility.is_compatible());

    let policy = EffectAuthorizationPolicy::new();
    let authorization = policy.authorize(&actual).unwrap();

    assert!(!authorization.is_authorized());
    assert_eq!(authorization.denied_count(), 1);
}
