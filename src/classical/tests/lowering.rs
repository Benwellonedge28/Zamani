
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Production contract tests for classical lowering.
//!
//! Path: src/classical/tests/lowering.rs
//! Rust: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! # Authority
//!
//! Source syntax is governed by `grammar/Zamani.g4` and its composed
//! lexer/parser grammars. Semantic meaning is governed by
//! `grammar/specification/semantic-model.md`.
//!
//! # Scope
//!
//! These tests cover the existing target-independent scalar and aggregate
//! lowering planners and their shared context. They do not define an AST,
//! semantic type system, canonical IR, target layout, or runtime.
//!
//! # Integration boundary
//!
//! The canonical semantic type system remains under
//! `crate::frontend::semantic`. Classical-facing type descriptors live under
//! `crate::classical::types`. Canonical IR ownership remains with the
//! applicable existing IR subsystem, including `crate::quantum::ir::classical`
//! and `crate::zuir`.
//!
//! A successful lowering plan is not proof of target feasibility or canonical
//! IR validity. A downstream adapter must construct canonical IR and invoke
//! its verifier. Unsupported or unresolved semantics must remain explicit.

use crate::classical::lowering::aggregate::{
    AggregateLoweringError, AggregateLoweringOptions, AggregateLoweringPlan,
    AggregateMemberPlan, AggregateLoweringObligation,
};
use crate::classical::lowering::context::{
    EntityKey, EntityKind, LoweringContext, LoweringContextError, LoweringPolicy,
};
use crate::classical::lowering::scalar::{
    LoweringObligation, ScalarLoweringPlan, ScalarRepresentation,
};
use crate::classical::lowering::target_independent::{
    TargetIndependentLowerer, TargetIndependentLoweringError,
};
use crate::classical::types::aggregate::{
    AggregateField, AggregateKind, AggregateMemberType, AggregateTypeDescriptor,
};
use crate::classical::types::scalar::{
    ScalarKind, ScalarTypeDescriptor, ScalarWidth,
};

fn type_key(namespace: &str, identity: &str) -> EntityKey {
    EntityKey::new(
        EntityKind::Type,
        namespace,
        identity,
    )
    .expect("test type identity should be valid")
}

fn source_key(namespace: &str, identity: &str) -> EntityKey {
    EntityKey::new(
        EntityKind::Source,
        namespace,
        identity,
    )
    .expect("test source identity should be valid")
}

fn scalar_descriptor(kind: ScalarKind, width: ScalarWidth) -> ScalarTypeDescriptor {
    ScalarTypeDescriptor::new(kind, width)
        .expect("test scalar descriptor should be valid")
}

fn boolean_member() -> AggregateMemberType {
    AggregateMemberType::Scalar(scalar_descriptor(
        ScalarKind::Boolean,
        ScalarWidth::Unspecified,
    ))
}

fn named_field(name: &str, ty: AggregateMemberType) -> AggregateField {
    AggregateField::named(name, ty).expect("test field should be valid")
}

#[test]
fn scalar_lowering_preserves_the_original_descriptor() {
    let descriptor = scalar_descriptor(
        ScalarKind::SignedInteger,
        ScalarWidth::fixed(127).expect("positive width"),
    );

    let plan = ScalarLoweringPlan::from_descriptor(&descriptor)
        .expect("valid scalar should produce a plan");

    assert_eq!(plan.descriptor(), &descriptor);
    assert_eq!(
        plan.representation(),
        &ScalarRepresentation::SignedInteger
    );
    assert!(plan.has_obligations());
    plan.validate().expect("plan metadata should be consistent");
}

#[test]
fn symbolic_width_is_preserved_without_host_width_substitution() {
    let width = ScalarWidth::symbolic("N * element_bits")
        .expect("non-empty symbolic expression");
    let descriptor = scalar_descriptor(ScalarKind::UnsignedInteger, width);

    let plan = ScalarLoweringPlan::from_descriptor(&descriptor)
        .expect("symbolic width should be representable in a plan");

    assert!(plan.has_symbolic_width());
    assert!(plan.obligations().contains(
        &LoweringObligation::ResolveSymbolicWidth {
            expression: "N * element_bits".to_owned(),
        }
    ));
    assert_eq!(plan.descriptor(), &descriptor);
}

#[test]
fn arbitrary_width_is_not_confused_with_unspecified_width() {
    let arbitrary = ScalarLoweringPlan::from_descriptor(
        &scalar_descriptor(ScalarKind::SignedInteger, ScalarWidth::Arbitrary),
    )
    .expect("arbitrary-width descriptor should be accepted");

    let unspecified = ScalarLoweringPlan::from_descriptor(
        &scalar_descriptor(ScalarKind::SignedInteger, ScalarWidth::Unspecified),
    )
    .expect("unspecified-width descriptor should be accepted");

    assert!(arbitrary.requires_arbitrary_width());
    assert!(!arbitrary.has_symbolic_width());
    assert!(arbitrary
        .obligations()
        .contains(&LoweringObligation::PreserveArbitraryWidth));

    assert!(!unspecified.requires_arbitrary_width());
    assert!(unspecified
        .obligations()
        .contains(&LoweringObligation::ResolveUnspecifiedWidth));

    assert_ne!(arbitrary, unspecified);
}

#[test]
fn extension_scalar_category_is_preserved_as_an_obligation() {
    let descriptor = scalar_descriptor(
        ScalarKind::Extension("vendor.neural_scalar".to_owned()),
        ScalarWidth::Arbitrary,
    );

    let plan = ScalarLoweringPlan::from_descriptor(&descriptor)
        .expect("extension category should remain representable");

    assert!(plan.is_extension_category());
    assert_eq!(
        plan.representation(),
        &ScalarRepresentation::Extension("vendor.neural_scalar".to_owned())
    );
    assert!(plan.obligations().contains(
        &LoweringObligation::ExtensionAdapterRequired {
            name: "vendor.neural_scalar".to_owned(),
        }
    ));
}

#[test]
fn decimal_and_rational_semantics_are_not_silently_erased() {
    let decimal = ScalarLoweringPlan::from_descriptor(
        &scalar_descriptor(ScalarKind::Decimal, ScalarWidth::Unspecified),
    )
    .expect("decimal descriptor should be accepted");

    let rational = ScalarLoweringPlan::from_descriptor(
        &scalar_descriptor(ScalarKind::Rational, ScalarWidth::Unspecified),
    )
    .expect("rational descriptor should be accepted");

    assert!(decimal
        .obligations()
        .contains(&LoweringObligation::DecimalSemanticsRequired));
    assert!(rational
        .obligations()
        .contains(&LoweringObligation::RationalSemanticsRequired));
}

#[test]
fn aggregate_lowering_preserves_field_order_and_names() {
    let descriptor = AggregateTypeDescriptor::structure(vec![
        named_field("first", boolean_member()),
        named_field(
            "second",
            AggregateMemberType::named("math::ExactNumber")
                .expect("named type reference"),
        ),
        named_field("third", boolean_member()),
    ])
    .expect("valid structure descriptor");

    let plan = AggregateLoweringPlan::build(
        &descriptor,
        &AggregateLoweringOptions::default(),
    )
    .expect("aggregate should produce a plan");

    let root = plan.node(plan.root()).expect("root node");

    assert_eq!(root.kind(), &AggregateKind::Struct);
    assert_eq!(root.fields().len(), 3);
    assert_eq!(root.fields()[0].name(), Some("first"));
    assert_eq!(root.fields()[1].name(), Some("second"));
    assert_eq!(root.fields()[2].name(), Some("third"));

    assert!(matches!(
        root.fields()[1].member(),
        AggregateMemberPlan::Named { name } if name == "math::ExactNumber"
    ));

    plan.validate().expect("aggregate plan should be valid");
}

#[test]
fn nested_aggregate_is_preserved_as_a_plan_node() {
    let inner = AggregateTypeDescriptor::tuple(vec![
        AggregateField::positional(boolean_member()),
        AggregateField::positional(boolean_member()),
    ])
    .expect("valid inner tuple");

    let outer = AggregateTypeDescriptor::structure(vec![
        named_field("payload", AggregateMemberType::aggregate(inner)),
    ])
    .expect("valid outer structure");

    let plan = AggregateLoweringPlan::build(
        &outer,
        &AggregateLoweringOptions::default(),
    )
    .expect("nested aggregate should produce a plan");

    assert_eq!(plan.nodes().len(), 2);

    let root = plan.node(plan.root()).expect("root node");
    let nested_index = match root.fields()[0].member() {
        AggregateMemberPlan::Nested { node } => *node,
        other => panic!("expected nested aggregate, got {other:?}"),
    };

    let nested = plan.node(nested_index).expect("nested node");
    assert_eq!(nested.kind(), &AggregateKind::Tuple);
    assert_eq!(nested.fields().len(), 2);
    plan.validate().expect("nested plan should be valid");
}

#[test]
fn unresolved_named_types_remain_explicit_obligations() {
    let descriptor = AggregateTypeDescriptor::structure(vec![
        named_field(
            "future_value",
            AggregateMemberType::named("future::Value")
                .expect("valid named type reference"),
        ),
    ])
    .expect("valid structure descriptor");

    let plan = AggregateLoweringPlan::build(
        &descriptor,
        &AggregateLoweringOptions::default(),
    )
    .expect("unresolved named type can be represented in a plan");

    assert!(plan.obligations().contains(
        &AggregateLoweringObligation::ResolveNamedType {
            name: "future::Value".to_owned(),
        }
    ));

    // Planning must preserve the unresolved reference rather than inventing
    // a concrete type or claiming that the destination can realize it.
    assert!(plan.has_obligations());
}

#[test]
fn aggregate_work_budget_exhaustion_returns_no_plan() {
    let descriptor = AggregateTypeDescriptor::structure(vec![
        named_field("first", boolean_member()),
        named_field("second", boolean_member()),
    ])
    .expect("valid structure descriptor");

    let result = AggregateLoweringPlan::build(
        &descriptor,
        &AggregateLoweringOptions {
            work_budget: Some(0),
        },
    );

    assert!(matches!(
        result,
        Err(AggregateLoweringError::WorkBudgetExceeded { .. })
    ));
}

#[test]
fn target_independent_lowerer_records_scalar_plan_and_context_work() {
    let mut lowerer = TargetIndependentLowerer::with_defaults();
    let source = type_key("semantic", "type::Signed");

    let descriptor = scalar_descriptor(
        ScalarKind::SignedInteger,
        ScalarWidth::symbolic("IntegerWidth")
            .expect("symbolic width"),
    );

    let plan = lowerer
        .plan_scalar(source.clone(), &descriptor)
        .expect("valid semantic type should be planned");

    assert_eq!(plan.source(), &source);
    assert!(plan.as_scalar().is_some());
    assert!(plan.as_aggregate().is_none());
    assert_eq!(lowerer.plans().len(), 1);
    assert_eq!(lowerer.context().consumed_work(), 1);
    assert!(lowerer.context().disposition(&source).is_some());
}

#[test]
fn target_independent_lowerer_rejects_non_type_source_entities() {
    let mut lowerer = TargetIndependentLowerer::with_defaults();
    let source = source_key("frontend", "source::declaration");
    let descriptor = scalar_descriptor(
        ScalarKind::Boolean,
        ScalarWidth::Unspecified,
    );

    let result = lowerer.plan_scalar(source, &descriptor);

    assert!(matches!(
        result,
        Err(TargetIndependentLoweringError::InvalidSourceEntityKind { .. })
    ));
    assert!(lowerer.plans().is_empty());
    assert_eq!(lowerer.context().consumed_work(), 0);
}

#[test]
fn failed_scalar_planning_does_not_consume_a_rejected_work_unit() {
    let context = LoweringContext::new(LoweringPolicy {
        work_budget: Some(0),
        ..LoweringPolicy::default()
    });
    let mut lowerer = TargetIndependentLowerer::new(context);

    let result = lowerer.plan_scalar(
        type_key("semantic", "type::Boolean"),
        &scalar_descriptor(ScalarKind::Boolean, ScalarWidth::Unspecified),
    );

    assert!(matches!(
        result,
        Err(TargetIndependentLoweringError::Context(
            LoweringContextError::WorkBudgetExceeded { .. }
        ))
    ));
    assert!(lowerer.plans().is_empty());
    assert_eq!(lowerer.context().consumed_work(), 0);
}

#[test]
fn failed_aggregate_planning_does_not_append_a_partial_plan() {
    let context = LoweringContext::new(LoweringPolicy {
        work_budget: Some(0),
        ..LoweringPolicy::default()
    });
    let mut lowerer = TargetIndependentLowerer::new(context);

    let descriptor = AggregateTypeDescriptor::tuple(vec![
        AggregateField::positional(boolean_member()),
    ])
    .expect("valid tuple descriptor");

    let result = lowerer.plan_aggregate(
        type_key("semantic", "type::Tuple"),
        &descriptor,
    );

    assert!(matches!(
        result,
        Err(TargetIndependentLoweringError::Aggregate(
            AggregateLoweringError::WorkBudgetExceeded { .. }
        )) | Err(TargetIndependentLoweringError::Context(
            LoweringContextError::WorkBudgetExceeded { .. }
        ))
    ));

    assert!(lowerer.plans().is_empty());
    assert_eq!(lowerer.context().consumed_work(), 0);
}

#[test]
fn repeated_planning_is_deterministic_for_identical_inputs() {
    let descriptor = scalar_descriptor(
        ScalarKind::UnsignedInteger,
        ScalarWidth::symbolic("N"),
    );
    let source = type_key("semantic", "type::Index");

    let mut first = TargetIndependentLowerer::with_defaults();
    let mut second = TargetIndependentLowerer::with_defaults();

    first
        .plan_scalar(source.clone(), &descriptor)
        .expect("first plan");
    second
        .plan_scalar(source, &descriptor)
        .expect("second plan");

    assert_eq!(first.plans(), second.plans());
    assert_eq!(first.context(), second.context());
}

#[test]
fn planner_handles_many_independent_types_without_a_semantic_count_limit() {
    // This is a bounded regression test, not a universal maximum. The
    // implementation must not interpret this test's iteration count as a
    // language or compiler ceiling.
    let mut lowerer = TargetIndependentLowerer::with_defaults();

    for index in 0_u64..1_024 {
        let identity = format!("type::{index}");
        let descriptor = scalar_descriptor(
            ScalarKind::UnsignedInteger,
            ScalarWidth::Arbitrary,
        );

        lowerer
            .plan_scalar(type_key("semantic", &identity), &descriptor)
            .expect("each independent type should be planned");
    }

    assert_eq!(lowerer.plans().len(), 1_024);
    assert_eq!(lowerer.context().consumed_work(), 1_024);
    assert_eq!(lowerer.plans().first().expect("first plan").source().value(), "type::0");
    assert_eq!(lowerer.plans().last().expect("last plan").source().value(), "type::1023");
}
