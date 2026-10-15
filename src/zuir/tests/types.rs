#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZUIR type-system integration and regression tests.
//!
//! Path: src/zuir/tests/types.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Verify the contracts shared by ZUIR semantic type definitions, symbolic
//! constraints, logical layouts, and extensible computational-domain types.
//!
//! This suite complements unit tests owned by the individual implementation
//! modules. It deliberately exercises public APIs and cross-module behavior.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/types/mod.rs
//! - src/zuir/types/type_def.rs
//! - src/zuir/types/layout.rs
//! - src/zuir/types/constraints.rs
//! - src/zuir/types/compatibility.rs
//! - src/zuir/types/type_id.rs
//! - src/zuir/verify/types.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//! - grammar/specification/semantic-model.md
//!
//! OWNS
//!
//! - Cross-module regression tests for semantic type contracts.
//! - Portability and scalability regression tests.
//! - Tests that symbolic and unknown information is not silently discarded.
//! - Tests that extension-defined types do not require new core variants.
//! - Tests that invalid type definitions are rejected by their owner.
//!
//! DOES_NOT_OWN
//!
//! - Production type-system implementation.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Type inference or generic constraint solving.
//! - Physical target layout, ABI, or resource allocation.
//! - Dialect registration or executable extension loading.
//! - Claims that every target supports every semantic type.
//!
//! INTEGRATION
//!
//! The tests use the existing canonical type subsystem. They must not define
//! replacement TypeDef, TypeKind, QualifiedName, TypeId, or layout models.
//!
//! The parent test module must register this file exactly once. If
//! src/zuir/tests/ is not currently the repository's test-module convention,
//! integrate the module through the existing test entry point rather than
//! creating a competing test hierarchy.
//!
//! SCALABILITY
//!
//! Tests must not encode universal hardware capacities. Large integer
//! quantities are tested using symbolic representations or arbitrary-width
//! decimal strings. Tests remain finite and must not claim literal infinite
//! memory, time, or computational capacity.
//!
//! FREEZE CRITERIA
//!
//! - Imports match the actual public APIs.
//! - All tests compile and pass on the declared Rust baseline.
//! - The test module is registered exactly once.
//! - No unsafe code or hardware-specific capacity assumptions are added.
//! - Any API mismatch is resolved against the owning implementation,
//!   not by adding duplicate types or weakening production invariants.

use std::collections::BTreeMap;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::constraints::Natural;
use crate::zuir::types::layout::{
    LayoutIntegerWidth, LayoutPolicy, LayoutShape, LogicalLayout,
};
use crate::zuir::types::type_def::{
    ConstantArgument, Dimension, FieldDef, IntegerSignedness, IntegerWidth,
    SymbolicExpression, TypeArgument, TypeAttribute, TypeConstraint, TypeDef,
    TypeDefError, TypeKind, TypeRef, TypeVisibility, VariantDef,
};

// -------------------------------------------------------------------------
// Shared test helpers
// -------------------------------------------------------------------------

fn qname(value: &str) -> QualifiedName {
    QualifiedName::parse(value).expect("test qualified name must be valid")
}

fn definition(name: &str, kind: TypeKind) -> TypeDef {
    TypeDef::new(qname(name), kind).expect("test type must be valid")
}

fn symbolic(expression: &str) -> SymbolicExpression {
    SymbolicExpression {
        expression: expression.to_owned(),
    }
}

fn named(name: &str) -> TypeRef {
    TypeRef::Named(qname(name))
}

fn layout_of(name: &str, kind: TypeKind) -> LogicalLayout {
    LogicalLayout::from_type_def(&definition(name, kind))
        .expect("valid semantic type must have a logical layout")
}

// -------------------------------------------------------------------------
// Type construction and validation
// -------------------------------------------------------------------------

#[test]
fn accepts_unbounded_integer_semantics() {
    let ty = definition(
        "core::Integer",
        TypeKind::Integer {
            signedness: IntegerSignedness::Signless,
            width: IntegerWidth::Unbounded,
        },
    );

    assert!(ty.validate().is_ok());
}

#[test]
fn rejects_zero_width_integers() {
    let result = TypeDef::new(
        qname("core::InvalidInteger"),
        TypeKind::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: IntegerWidth::Fixed(0),
        },
    );

    assert!(matches!(result, Err(TypeDefError::ZeroIntegerWidth)));
}

#[test]
fn accepts_large_fixed_integer_width_without_machine_assumptions() {
    let ty = definition(
        "core::WideInteger",
        TypeKind::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: IntegerWidth::Fixed(u128::MAX),
        },
    );

    assert!(ty.validate().is_ok());

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(
        layout.shape,
        LayoutShape::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: LayoutIntegerWidth::Fixed(width),
        } if width == u128::MAX
    ));
}

#[test]
fn accepts_symbolic_array_dimensions() {
    let ty = definition(
        "tensor::Vector",
        TypeKind::Array {
            element: named("core::Number"),
            length: Dimension::Symbolic(symbolic("vector_length")),
        },
    );

    assert!(ty.validate().is_ok());
}

#[test]
fn rejects_empty_symbolic_array_dimensions() {
    let result = TypeDef::new(
        qname("tensor::InvalidVector"),
        TypeKind::Array {
            element: named("core::Number"),
            length: Dimension::Symbolic(symbolic("  ")),
        },
    );

    assert!(matches!(
        result,
        Err(TypeDefError::EmptySymbolicExpression)
    ));
}

#[test]
fn accepts_dynamic_dimensions_without_claiming_unlimited_allocation() {
    let ty = definition(
        "tensor::DynamicArray",
        TypeKind::Array {
            element: named("core::Number"),
            length: Dimension::Dynamic,
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(
        layout.shape,
        LayoutShape::Array {
            length: Dimension::Dynamic,
            ..
        }
    ));
}

#[test]
fn rejects_duplicate_record_fields() {
    let result = TypeDef::new(
        qname("core::DuplicateRecord"),
        TypeKind::Record {
            fields: vec![
                FieldDef {
                    name: "value".to_owned(),
                    ty: named("core::Integer"),
                    attributes: BTreeMap::new(),
                },
                FieldDef {
                    name: "value".to_owned(),
                    ty: named("core::Boolean"),
                    attributes: BTreeMap::new(),
                },
            ],
        },
    );

    assert!(matches!(
        result,
        Err(TypeDefError::DuplicateName {
            category: "record field",
            ..
        })
    ));
}

#[test]
fn rejects_empty_union() {
    let result = TypeDef::new(
        qname("core::InvalidUnion"),
        TypeKind::Union {
            alternatives: Vec::new(),
        },
    );

    assert!(matches!(result, Err(TypeDefError::EmptyUnion)));
}

#[test]
fn revalidates_mutated_or_deserialized_type_definitions() {
    let mut ty = definition("core::InitiallyValid", TypeKind::Boolean);

    ty.kind = TypeKind::Union {
        alternatives: Vec::new(),
    };

    assert_eq!(ty.validate(), Err(TypeDefError::EmptyUnion));
}

#[test]
fn accepts_extension_defined_types_without_core_changes() {
    let ty = definition(
        "vendor::SpecialTensor",
        TypeKind::Dialect {
            dialect: qname("vendor::tensor"),
            type_name: qname("tensor::Special"),
            arguments: vec![TypeArgument::Symbolic(symbolic("rank"))],
        },
    );

    assert!(ty.validate().is_ok());
}

#[test]
fn preserves_type_visibility_as_declaration_metadata() {
    let mut ty = definition("core::PublicValue", TypeKind::Boolean);

    assert_eq!(ty.visibility, TypeVisibility::Private);

    ty.visibility = TypeVisibility::Public;

    assert_eq!(ty.visibility, TypeVisibility::Public);
    assert!(ty.validate().is_ok());
}

// -------------------------------------------------------------------------
// Logical layouts must preserve semantic information
// -------------------------------------------------------------------------

#[test]
fn preserves_symbolic_integer_width() {
    let expression = symbolic("required_integer_width");

    let layout = layout_of(
        "core::SymbolicInteger",
        TypeKind::Integer {
            signedness: IntegerSignedness::Signed,
            width: IntegerWidth::Symbolic(expression.clone()),
        },
    );

    assert!(matches!(
        layout.shape,
        LayoutShape::Integer {
            signedness: IntegerSignedness::Signed,
            width: LayoutIntegerWidth::Symbolic(ref actual),
        } if actual == &expression
    ));
}

#[test]
fn preserves_record_field_order_and_attributes() {
    let mut attributes = BTreeMap::new();

    attributes.insert(
        "documentation".to_owned(),
        TypeAttribute::String("payload".to_owned()),
    );

    let ty = definition(
        "core::Payload",
        TypeKind::Record {
            fields: vec![
                FieldDef {
                    name: "first".to_owned(),
                    ty: named("core::Integer"),
                    attributes: attributes.clone(),
                },
                FieldDef {
                    name: "second".to_owned(),
                    ty: named("core::Boolean"),
                    attributes: BTreeMap::new(),
                },
            ],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    match layout.shape {
        LayoutShape::Record { fields } => {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "first");
            assert_eq!(fields[1].name, "second");
            assert_eq!(fields[0].attributes, attributes);
        }
        other => panic!("expected record layout, got {other:?}"),
    }
}

#[test]
fn preserves_sum_variant_names_and_payloads() {
    let ty = definition(
        "core::Result",
        TypeKind::Sum {
            variants: vec![
                VariantDef {
                    name: "Success".to_owned(),
                    fields: vec![FieldDef {
                        name: "value".to_owned(),
                        ty: named("core::Integer"),
                        attributes: BTreeMap::new(),
                    }],
                    attributes: BTreeMap::new(),
                },
                VariantDef {
                    name: "Failure".to_owned(),
                    fields: Vec::new(),
                    attributes: BTreeMap::new(),
                },
            ],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    match layout.shape {
        LayoutShape::Sum { variants } => {
            assert_eq!(variants.len(), 2);
            assert_eq!(variants[0].name, "Success");
            assert_eq!(variants[0].fields.len(), 1);
            assert_eq!(variants[1].name, "Failure");
            assert!(variants[1].fields.is_empty());
        }
        other => panic!("expected sum layout, got {other:?}"),
    }
}

#[test]
fn preserves_named_references_without_inventing_definitions() {
    let ty = definition(
        "core::Container",
        TypeKind::Tuple {
            elements: vec![named("application::Payload")],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(
        layout.shape,
        LayoutShape::Tuple { ref elements }
            if matches!(
                elements.as_slice(),
                [LayoutShape::NamedType { name }]
                    if name == &qname("application::Payload")
            )
    ));
}

#[test]
fn preserves_quantum_types_without_physical_qubit_allocation() {
    let ty = definition(
        "quantum::State",
        TypeKind::Quantum {
            domain_type: qname("quantum::StateVector"),
            parameters: vec![TypeArgument::Symbolic(symbolic("logical_dimension"))],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(layout.shape, LayoutShape::Quantum { .. }));
}

#[test]
fn preserves_hdl_types_without_fixed_signal_width() {
    let ty = definition(
        "hdl::Signal",
        TypeKind::Hardware {
            domain_type: qname("hdl::Signal"),
            parameters: vec![TypeArgument::Symbolic(symbolic("signal_width"))],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(layout.shape, LayoutShape::Hardware { .. }));
}

#[test]
fn preserves_domain_defined_type_arguments() {
    let ty = definition(
        "tensor::Tensor",
        TypeKind::Dialect {
            dialect: qname("tensor::core"),
            type_name: qname("tensor::Tensor"),
            arguments: vec![
                TypeArgument::Type(named("core::Float")),
                TypeArgument::Symbolic(symbolic("tensor_rank")),
                TypeArgument::Constant(ConstantArgument::Integer(
                    "1234567890123456789012345678901234567890".to_owned(),
                )),
            ],
        },
    );

    let layout = LogicalLayout::from_type_def(&ty).expect("logical layout");

    assert!(matches!(layout.shape, LayoutShape::Dialect { .. }));
}

#[test]
fn supports_empty_tuple_and_empty_record() {
    let tuple = layout_of(
        "core::EmptyTuple",
        TypeKind::Tuple {
            elements: Vec::new(),
        },
    );

    let record = layout_of(
        "core::EmptyRecord",
        TypeKind::Record {
            fields: Vec::new(),
        },
    );

    assert!(matches!(
        tuple.shape,
        LayoutShape::Tuple { ref elements } if elements.is_empty()
    ));

    assert!(matches!(
        record.shape,
        LayoutShape::Record { ref fields } if fields.is_empty()
    ));
}

#[test]
fn rejects_inline_layout_nesting_beyond_configured_budget() {
    let ty = definition(
        "core::Nested",
        TypeKind::Tuple {
            elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
            }))],
        },
    );

    let policy = LayoutPolicy {
        max_inline_depth: Some(1),
    };

    let result = LogicalLayout::from_type_def_with_policy(&ty, &policy);

    assert!(matches!(
        result,
        Err(crate::zuir::types::layout::LayoutError::NestingBudgetExceeded { .. })
    ));
}

#[test]
fn accepts_inline_layout_within_configured_budget() {
    let ty = definition(
        "core::Nested",
        TypeKind::Tuple {
            elements: vec![TypeRef::Inline(Box::new(TypeKind::Tuple {
                elements: vec![TypeRef::Inline(Box::new(TypeKind::Boolean))],
            }))],
        },
    );

    let policy = LayoutPolicy {
        max_inline_depth: Some(8),
    };

    assert!(
        LogicalLayout::from_type_def_with_policy(&ty, &policy).is_ok()
    );
}

// -------------------------------------------------------------------------
// Arbitrary-width natural numbers used by symbolic constraints
// -------------------------------------------------------------------------

#[test]
fn natural_normalizes_leading_zeroes() {
    let value = Natural::parse("00000042").expect("valid natural number");

    assert_eq!(value.as_str(), "42");
}

#[test]
fn natural_preserves_values_beyond_fixed_machine_widths() {
    let decimal = "12345678901234567890123456789012345678901234567890";
    let value = Natural::parse(decimal).expect("valid arbitrary-width number");

    assert_eq!(value.as_str(), decimal);
    assert!(value.validate().is_ok());
}

#[test]
fn natural_compares_arbitrary_width_values_without_truncation() {
    let smaller = Natural::parse("999999999999999999999999999999")
        .expect("valid natural number");

    let larger = Natural::parse("1000000000000000000000000000000")
        .expect("valid natural number");

    assert!(smaller.compare(&larger).is_lt());
    assert!(larger.compare(&smaller).is_gt());
    assert!(smaller.compare(&smaller).is_eq());
}

#[test]
fn natural_rejects_signed_or_non_decimal_input() {
    for invalid in ["", "-1", "+1", "1.0", "1_000", "12x", " 1"] {
        assert!(
            Natural::parse(invalid).is_err(),
            "expected {invalid:?} to be rejected"
        );
    }
}

#[test]
fn natural_has_canonical_zero_and_one() {
    assert_eq!(Natural::zero().as_str(), "0");
    assert_eq!(Natural::one().as_str(), "1");
    assert!(Natural::zero().compare(&Natural::one()).is_lt());
}

// -------------------------------------------------------------------------
// Declarative constraints remain explicit
// -------------------------------------------------------------------------

#[test]
fn type_constraints_preserve_symbolic_expressions() {
    let constraint = TypeConstraint::IntegerWidthAtLeast(symbolic("minimum_width"));

    let ty = definition(
        "core::ConstrainedInteger",
        TypeKind::Parameter {
            name: "T".to_owned(),
            bounds: vec![constraint.clone()],
        },
    );

    assert!(ty.validate().is_ok());

    match ty.kind {
        TypeKind::Parameter { bounds, .. } => {
            assert_eq!(bounds, vec![constraint]);
        }
        other => panic!("expected parameter type, got {other:?}"),
    }
}

#[test]
fn dialect_constraints_do_not_require_new_core_type_variants() {
    let constraint = TypeConstraint::Dialect {
        namespace: qname("tensor::core"),
        constraint: qname("tensor::ShapeCompatible"),
    };

    let ty = definition(
        "tensor::Constrained",
        TypeKind::Opaque {
            constraints: vec![constraint.clone()],
        },
    );

    assert!(ty.validate().is_ok());

    match ty.kind {
        TypeKind::Opaque { constraints } => {
            assert_eq!(constraints, vec![constraint]);
        }
        other => panic!("expected opaque type, got {other:?}"),
    }
}

#[test]
fn rejects_empty_symbolic_type_arguments() {
    let result = TypeDef::new(
        qname("tensor::Invalid"),
        TypeKind::Dialect {
            dialect: qname("tensor::core"),
            type_name: qname("tensor::Tensor"),
            arguments: vec![TypeArgument::Symbolic(symbolic(" "))],
        },
    );

    assert!(matches!(
        result,
        Err(TypeDefError::EmptySymbolicExpression)
    ));
}