
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical type contract tests.
//!
//! Path: `src/classical/tests/types.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Verify the public contract of the classical-facing type descriptors:
//! scalar types, aggregate types, symbolic/fixed widths, and target-independent
//! layout composition.
//!
//! # Authority
//!
//! - `src/classical/types/scalar.rs`: scalar descriptor semantics.
//! - `src/classical/types/aggregate.rs`: aggregate descriptor semantics.
//! - `src/classical/types/generic.rs`: generic semantic integration boundary.
//! - `src/classical/types/constraints.rs`: canonical constraint integration.
//! - `src/classical/types/layout.rs`: target-independent layout values.
//! - `src/frontend/ast/node/types/type_expr.rs`: canonical source type syntax.
//! - `src/frontend/semantic/types/ty.rs`: canonical resolved semantic types.
//! - `src/quantum/ir/classical/`: canonical classical IR boundary.
//! - `grammar/Zamani.g4`: root grammar composition contract.
//! - `grammar/specification/semantic-model.md`: normative semantic authority.
//! - `grammar/validation/semantic-boundaries.md`: ownership and validation.
//! - `grammar/specification/scalability-model.md`: scalability requirements.
//! - `grammar/specification/poco-reaf.md`: portability requirements.
//! - `grammar/types/generic.g4` and `grammar/types/bounds.g4`: generic syntax.
//!
//! # Ownership
//!
//! These tests do not define a second AST, semantic type system, constraint
//! solver, generic substitution engine, IR, or target ABI implementation.
//! They test the classical-facing descriptors and their local invariants.
//!
//! # Scalability contract
//!
//! No test here establishes a language-level maximum for scalar width,
//! aggregate field count, variant count, type nesting, layout size, hardware
//! count, or execution scale.
//!
//! Concrete Rust `usize` values are host-representable quantities only.
//! Symbolic values must remain symbolic; overflow must not wrap or silently
//! narrow. A finite test workload cannot prove literal infinite scalability.
//!
//! # Integration
//!
//! Register this test file exactly once from `src/classical/types/mod.rs`:
//
//! ```ignore
//! #[cfg(test)]
//! #[path = "../tests/types.rs"]
//! mod contract_tests;
//! ```
//!
//! The path is relative to `src/classical/types/mod.rs`.
//! Do not register this file again from `src/classical/mod.rs` or another
//! test module. `src/classical/mod.rs` must be repaired to contain valid Rust
//! module declarations before crate-wide integration can be verified.
//!
//! # Completion status
//!
//! Proposed contract tests; not marked verified until compiled and run
//! against the actual repository checkout and supported Rust toolchain.

use crate::classical::types::aggregate::{
    AggregateField,
    AggregateKind,
    AggregateMemberType,
    AggregateTypeDescriptor,
    AggregateTypeError,
    AggregateVariant,
};
use crate::classical::types::layout::{
    LayoutEngine,
    LayoutError,
    LayoutExpression,
    LayoutKind,
    LayoutValue,
    TypeLayout,
};
use crate::classical::types::scalar::{
    ScalarKind,
    ScalarTypeDescriptor,
    ScalarTypeError,
    ScalarWidth,
};

fn signed_scalar() -> ScalarTypeDescriptor {
    ScalarTypeDescriptor::unspecified(ScalarKind::SignedInteger)
        .expect("signed integer descriptor should be valid")
}

fn unsigned_scalar() -> ScalarTypeDescriptor {
    ScalarTypeDescriptor::unspecified(ScalarKind::UnsignedInteger)
        .expect("unsigned integer descriptor should be valid")
}

fn scalar_member() -> AggregateMemberType {
    AggregateMemberType::Scalar(signed_scalar())
}

fn named_field(name: &str) -> AggregateField {
    AggregateField::named(name, scalar_member())
        .expect("valid test field")
}

fn positional_field() -> AggregateField {
    AggregateField::positional(scalar_member())
}

fn scalar_layout(size: usize, alignment: usize) -> TypeLayout {
    TypeLayout::scalar(
        LayoutValue::Known(size),
        LayoutValue::Known(alignment),
    )
    .expect("valid scalar layout")
}

// -------------------------------------------------------------------------
// Scalar descriptor contracts
// -------------------------------------------------------------------------

#[test]
fn scalar_descriptor_preserves_kind_and_width() {
    let ty = ScalarTypeDescriptor::fixed_integer(true, 128)
        .expect("positive signed integer width");

    assert_eq!(ty.kind(), &ScalarKind::SignedInteger);
    assert_eq!(ty.width().as_fixed(), Some(128));
    assert!(ty.is_integer());
    assert!(ty.is_numeric());
    assert!(ty.validate().is_ok());
}

#[test]
fn signed_and_unsigned_integer_descriptors_remain_distinct() {
    let signed = ScalarTypeDescriptor::fixed_integer(true, 64)
        .expect("signed integer descriptor");
    let unsigned = ScalarTypeDescriptor::fixed_integer(false, 64)
        .expect("unsigned integer descriptor");

    assert_ne!(signed.kind(), unsigned.kind());
    assert_ne!(signed, unsigned);
    assert!(signed.is_integer());
    assert!(unsigned.is_integer());
}

#[test]
fn zero_fixed_width_is_rejected() {
    assert_eq!(
        ScalarWidth::fixed(0),
        Err(ScalarTypeError::ZeroWidth)
    );

    assert_eq!(
        ScalarTypeDescriptor::fixed_integer(true, 0),
        Err(ScalarTypeError::ZeroWidth)
    );
}

#[test]
fn fixed_width_is_not_restricted_to_a_machine_register_size() {
    // A representative width larger than common machine scalar widths.
    // This describes a type; it does not claim target support.
    let width = 4_096usize;

    let ty = ScalarTypeDescriptor::fixed_integer(true, width)
        .expect("host-representable positive width");

    assert_eq!(ty.width().as_fixed(), Some(width));
    assert!(ty.validate().is_ok());
}

#[test]
fn symbolic_width_is_preserved_without_evaluation() {
    let expression = "dimensions::required_integer_bits";

    let ty = ScalarTypeDescriptor::symbolic_width(
        ScalarKind::UnsignedInteger,
        expression,
    )
    .expect("non-empty symbolic width");

    assert!(ty.width().is_symbolic());
    assert_eq!(
        ty.width(),
        &ScalarWidth::Symbolic(expression.to_owned())
    );
    assert!(ty.validate().is_ok());
}

#[test]
fn empty_symbolic_width_is_rejected() {
    assert_eq!(
        ScalarWidth::symbolic(" \t "),
        Err(ScalarTypeError::EmptySymbolicWidth)
    );
}

#[test]
fn arbitrary_width_is_not_replaced_with_a_host_width() {
    let ty = ScalarTypeDescriptor::new(
        ScalarKind::SignedInteger,
        ScalarWidth::Arbitrary,
    )
    .expect("arbitrary-width integer descriptor");

    assert_eq!(ty.width(), &ScalarWidth::Arbitrary);
    assert_eq!(ty.width().as_fixed(), None);
    assert!(ty.validate().is_ok());
}

#[test]
fn logical_bit_has_exactly_one_bit_when_width_is_fixed() {
    let valid = ScalarTypeDescriptor::new(
        ScalarKind::Bit,
        ScalarWidth::fixed(1).expect("positive width"),
    )
    .expect("logical bit width");

    assert_eq!(valid.width().as_fixed(), Some(1));

    assert_eq!(
        ScalarTypeDescriptor::new(
            ScalarKind::Bit,
            ScalarWidth::fixed(8).expect("positive width"),
        ),
        Err(ScalarTypeError::BitWidthMustBeOne { actual: 8 })
    );
}

#[test]
fn boolean_does_not_accept_numeric_width_metadata() {
    assert!(matches!(
        ScalarTypeDescriptor::new(
            ScalarKind::Boolean,
            ScalarWidth::fixed(1).expect("positive width"),
        ),
        Err(ScalarTypeError::WidthNotApplicable {
            kind: ScalarKind::Boolean
        })
    ));
}

#[test]
fn width_not_applicable_to_decimal_rational_and_complex_descriptors() {
    for kind in [
        ScalarKind::Decimal,
        ScalarKind::Rational,
        ScalarKind::Complex,
    ] {
        let result = ScalarTypeDescriptor::new(
            kind.clone(),
            ScalarWidth::fixed(128).expect("positive width"),
        );

        assert!(matches!(
            result,
            Err(ScalarTypeError::WidthNotApplicable {
                kind: rejected
            }) if rejected == kind
        ));
    }
}

#[test]
fn extension_scalar_categories_are_open_ended() {
    let name = "vendor.example.extended_numeric";

    let ty = ScalarTypeDescriptor::unspecified(
        ScalarKind::Extension(name.to_owned()),
    )
    .expect("valid extension category");

    assert_eq!(ty.kind_name(), name);
    assert!(ty.validate().is_ok());
}

#[test]
fn empty_extension_scalar_category_is_rejected() {
    assert_eq!(
        ScalarTypeDescriptor::unspecified(
            ScalarKind::Extension(" \t ".to_owned()),
        ),
        Err(ScalarTypeError::EmptyExtensionName)
    );
}

#[test]
fn scalar_categories_expose_consistent_classification() {
    assert!(ScalarKind::SignedInteger.is_integer());
    assert!(ScalarKind::UnsignedInteger.is_integer());
    assert!(!ScalarKind::Boolean.is_integer());

    assert!(ScalarKind::SignedInteger.is_numeric());
    assert!(ScalarKind::BinaryFloat.is_numeric());
    assert!(ScalarKind::Decimal.is_numeric());
    assert!(ScalarKind::Rational.is_numeric());
    assert!(ScalarKind::Complex.is_numeric());
    assert!(!ScalarKind::Boolean.is_numeric());

    assert!(ScalarKind::Bit.supports_width());
    assert!(ScalarKind::SignedInteger.supports_width());
    assert!(ScalarKind::BinaryFloat.supports_width());
    assert!(!ScalarKind::Boolean.supports_width());
}

#[test]
fn scalar_descriptor_equality_is_structural_and_deterministic() {
    let first = ScalarTypeDescriptor::fixed_integer(true, 256)
        .expect("first descriptor");
    let second = ScalarTypeDescriptor::fixed_integer(true, 256)
        .expect("second descriptor");
    let different = ScalarTypeDescriptor::fixed_integer(true, 512)
        .expect("different width descriptor");

    assert_eq!(first, second);
    assert_ne!(first, different);
    assert_eq!(first.kind_name(), second.kind_name());
}

// -------------------------------------------------------------------------
// Aggregate descriptor contracts
// -------------------------------------------------------------------------

#[test]
fn structure_preserves_field_order_and_names() {
    let ty = AggregateTypeDescriptor::structure(vec![
        named_field("first"),
        named_field("second"),
        named_field("third"),
    ])
    .expect("valid structure");

    assert_eq!(ty.kind(), &AggregateKind::Struct);
    assert!(ty.is_product());
    assert!(!ty.is_sum());
    assert_eq!(ty.fields().len(), 3);
    assert_eq!(ty.fields()[0].name(), Some("first"));
    assert_eq!(ty.fields()[1].name(), Some("second"));
    assert_eq!(ty.fields()[2].name(), Some("third"));
    assert!(ty.validate().is_ok());
}

#[test]
fn record_and_structure_are_distinct_semantic_categories() {
    let structure = AggregateTypeDescriptor::structure(vec![named_field("value")])
        .expect("valid structure");
    let record = AggregateTypeDescriptor::record(vec![named_field("value")])
        .expect("valid record");

    assert_eq!(structure.kind(), &AggregateKind::Struct);
    assert_eq!(record.kind(), &AggregateKind::Record);
    assert_ne!(structure, record);
}

#[test]
fn duplicate_field_names_are_rejected() {
    let result = AggregateTypeDescriptor::structure(vec![
        named_field("value"),
        named_field("value"),
    ]);

    assert_eq!(
        result,
        Err(AggregateTypeError::DuplicateField {
            name: "value".to_owned(),
        })
    );
}

#[test]
fn named_aggregate_rejects_positional_fields() {
    let result = AggregateTypeDescriptor::structure(vec![
        positional_field(),
    ]);

    assert_eq!(result, Err(AggregateTypeError::NamedFieldsRequired));
}

#[test]
fn tuple_requires_positional_fields() {
    let result = AggregateTypeDescriptor::tuple(vec![
        named_field("named"),
    ]);

    assert_eq!(
        result,
        Err(AggregateTypeError::PositionalFieldsRequired)
    );
}

#[test]
fn empty_tuple_is_valid() {
    let tuple = AggregateTypeDescriptor::tuple(Vec::new())
        .expect("empty tuple is a valid product type");

    assert_eq!(tuple.kind(), &AggregateKind::Tuple);
    assert!(tuple.is_product());
    assert!(tuple.fields().is_empty());
    assert!(tuple.validate().is_ok());
}

#[test]
fn named_type_references_are_checked_for_empty_names_and_whitespace() {
    assert_eq!(
        AggregateMemberType::named(" \t "),
        Err(AggregateTypeError::EmptyTypeReference)
    );

    assert_eq!(
        AggregateMemberType::named("module::Some Type"),
        Err(AggregateTypeError::WhitespaceInTypeReference {
            name: "module::Some Type".to_owned(),
        })
    );

    assert_eq!(
        AggregateMemberType::named("module::Packet"),
        Ok(AggregateMemberType::Named("module::Packet".to_owned()))
    );
}

#[test]
fn empty_and_whitespace_field_names_are_rejected() {
    assert_eq!(
        AggregateField::named(" ", scalar_member()),
        Err(AggregateTypeError::EmptyFieldName)
    );

    assert_eq!(
        AggregateField::named("field name", scalar_member()),
        Err(AggregateTypeError::WhitespaceInFieldName {
            name: "field name".to_owned(),
        })
    );
}

#[test]
fn enum_requires_at_least_one_variant() {
    assert_eq!(
        AggregateTypeDescriptor::enumeration(Vec::new()),
        Err(AggregateTypeError::EmptyVariants)
    );
}

#[test]
fn enum_rejects_payload_variants() {
    let variant = AggregateVariant::new(
        "WithPayload",
        vec![positional_field()],
    )
    .expect("variant declaration itself is valid");

    assert!(matches!(
        AggregateTypeDescriptor::enumeration(vec![variant]),
        Err(AggregateTypeError::PayloadNotAllowed {
            kind: AggregateKind::Enum,
            variant
        }) if variant == "WithPayload"
    ));
}

#[test]
fn enum_preserves_variant_order() {
    let ty = AggregateTypeDescriptor::enumeration(vec![
        AggregateVariant::unit("First").expect("first variant"),
        AggregateVariant::unit("Second").expect("second variant"),
        AggregateVariant::unit("Third").expect("third variant"),
    ])
    .expect("valid enum");

    assert_eq!(ty.kind(), &AggregateKind::Enum);
    assert!(ty.is_sum());
    assert_eq!(ty.variants().len(), 3);
    assert_eq!(ty.variants()[0].name(), "First");
    assert_eq!(ty.variants()[1].name(), "Second");
    assert_eq!(ty.variants()[2].name(), "Third");
    assert!(ty.validate().is_ok());
}

#[test]
fn duplicate_variant_names_are_rejected() {
    let first = AggregateVariant::unit("Ready").expect("first variant");
    let second = AggregateVariant::unit("Ready").expect("second variant");

    assert_eq!(
        AggregateTypeDescriptor::enumeration(vec![first, second]),
        Err(AggregateTypeError::DuplicateVariant {
            name: "Ready".to_owned(),
        })
    );
}

#[test]
fn optional_descriptor_has_the_required_value_field() {
    let optional = AggregateTypeDescriptor::optional(scalar_member())
        .expect("valid optional descriptor");

    assert_eq!(optional.kind(), &AggregateKind::Optional);
    assert!(optional.is_sum());
    assert_eq!(optional.fields().len(), 1);
    assert!(optional.field("value").is_some());
    assert!(optional.validate().is_ok());
}

#[test]
fn result_descriptor_has_ordered_ok_and_error_fields() {
    let result = AggregateTypeDescriptor::result(
        scalar_member(),
        AggregateMemberType::named("Error").expect("named error type"),
    )
    .expect("valid result descriptor");

    assert_eq!(result.kind(), &AggregateKind::Result);
    assert_eq!(result.fields().len(), 2);
    assert_eq!(result.fields()[0].name(), Some("ok"));
    assert_eq!(result.fields()[1].name(), Some("error"));
    assert!(result.validate().is_ok());
}

#[test]
fn aggregate_extension_kinds_are_open_ended() {
    let extension = AggregateTypeDescriptor::extension(
        "vendor.example.TaggedPayload",
        vec![named_field("payload")],
        Vec::new(),
    )
    .expect("valid extension descriptor");

    assert_eq!(
        extension.kind(),
        &AggregateKind::Extension(
            "vendor.example.TaggedPayload".to_owned()
        )
    );
    assert!(extension.validate().is_ok());
}

#[test]
fn aggregate_extension_rejects_empty_extension_name() {
    assert_eq!(
        AggregateTypeDescriptor::extension(
            " ",
            Vec::new(),
            Vec::new(),
        ),
        Err(AggregateTypeError::EmptyExtensionName)
    );
}

#[test]
fn nested_aggregate_descriptors_are_structural() {
    let inner = AggregateTypeDescriptor::structure(vec![
        named_field("count"),
    ])
    .expect("inner structure");

    let outer = AggregateTypeDescriptor::structure(vec![
        AggregateField::named(
            "inner",
            AggregateMemberType::aggregate(inner.clone()),
        )
        .expect("nested field"),
    ])
    .expect("outer structure");

    assert_eq!(outer.fields().len(), 1);
    assert_eq!(outer.field("inner").expect("inner field").name(), Some("inner"));

    let AggregateMemberType::Aggregate(nested) =
        outer.field("inner").expect("inner field").ty()
    else {
        panic!("nested field must preserve aggregate member type");
    };

    assert_eq!(nested.as_ref(), &inner);
    assert!(outer.validate().is_ok());
}

#[test]
fn large_finite_aggregate_preserves_all_fields_without_a_fixed_ceiling() {
    // Representative stress input, not a language-level maximum.
    let field_count = 1_024usize;
    let fields = (0..field_count)
        .map(|index| named_field(&format!("field_{index}")))
        .collect();

    let aggregate = AggregateTypeDescriptor::structure(fields)
        .expect("all generated field names are unique");

    assert_eq!(aggregate.fields().len(), field_count);
    assert_eq!(
        aggregate.field("field_0").and_then(AggregateField::name),
        Some("field_0")
    );
    assert_eq!(
        aggregate
            .field("field_1023")
            .and_then(AggregateField::name),
        Some("field_1023")
    );
    assert!(aggregate.validate().is_ok());
}

#[test]
fn aggregate_lookup_returns_none_for_unknown_names() {
    let aggregate = AggregateTypeDescriptor::structure(vec![
        named_field("present"),
    ])
    .expect("valid structure");

    assert!(aggregate.field("missing").is_none());
    assert!(aggregate.variant("missing").is_none());
}

#[test]
fn identical_descriptors_have_deterministic_structural_equality() {
    let first = AggregateTypeDescriptor::structure(vec![
        named_field("left"),
        named_field("right"),
    ])
    .expect("first descriptor");

    let second = AggregateTypeDescriptor::structure(vec![
        named_field("left"),
        named_field("right"),
    ])
    .expect("second descriptor");

    let reordered = AggregateTypeDescriptor::structure(vec![
        named_field("right"),
        named_field("left"),
    ])
    .expect("reordered descriptor");

    assert_eq!(first, second);
    assert_ne!(first, reordered);
}

// -------------------------------------------------------------------------
// Target-independent layout contracts
// -------------------------------------------------------------------------

#[test]
fn known_layout_values_preserve_concrete_quantities() {
    let size = LayoutValue::known(24);
    let alignment = LayoutValue::known(8);

    assert_eq!(size.as_known(), Some(24));
    assert!(size.is_known());
    assert_eq!(alignment.as_known(), Some(8));
}

#[test]
fn symbolic_layout_constants_preserve_values_beyond_host_width() {
    let decimal = "12345678901234567890123456789012345678901234567890";

    let expression = LayoutExpression::constant_decimal(decimal)
        .expect("decimal constant must not be narrowed to usize");

    assert_eq!(
        expression,
        LayoutExpression::Constant(decimal.to_owned())
    );
}

#[test]
fn symbolic_layout_constants_are_canonicalized() {
    assert_eq!(
        LayoutExpression::constant_decimal("0000123")
            .expect("valid decimal"),
        LayoutExpression::Constant("123".to_owned())
    );

    assert_eq!(
        LayoutExpression::constant_decimal("0000")
            .expect("valid zero"),
        LayoutExpression::Constant("0".to_owned())
    );
}

#[test]
fn malformed_layout_constants_are_rejected() {
    for input in ["", "-1", "+1", "1.5", "12x", " 42"] {
        assert_eq!(
            LayoutExpression::constant_decimal(input),
            Err(LayoutError::InvalidDecimalConstant),
            "input {input:?} must be rejected"
        );
    }
}

#[test]
fn symbolic_layout_expression_rejects_empty_symbol() {
    assert_eq!(
        LayoutExpression::symbol("  "),
        Err(LayoutError::EmptySymbol)
    );
}

#[test]
fn concrete_layout_addition_is_checked() {
    assert_eq!(
        LayoutValue::Known(10).add(&LayoutValue::Known(20)),
        LayoutValue::Known(30)
    );

    let overflow = LayoutValue::Known(usize::MAX)
        .add(&LayoutValue::Known(1));

    assert!(
        matches!(overflow, LayoutValue::Symbolic(_)),
        "overflow must remain symbolic rather than wrap or truncate"
    );
}

#[test]
fn concrete_layout_multiplication_is_checked() {
    assert_eq!(
        LayoutValue::Known(6).multiply(&LayoutValue::Known(7)),
        LayoutValue::Known(42)
    );

    let overflow = LayoutValue::Known(usize::MAX)
        .multiply(&LayoutValue::Known(2));

    assert!(
        matches!(overflow, LayoutValue::Symbolic(_)),
        "overflow must remain symbolic rather than wrap or truncate"
    );
}

#[test]
fn unknown_unsized_and_symbolic_layout_states_remain_distinct() {
    let unknown = LayoutValue::Unknown;
    let unsized = LayoutValue::Unsized;
    let symbolic = LayoutValue::symbolic(
        LayoutExpression::symbol("N").expect("symbol"),
    );

    assert_ne!(unknown, unsized);
    assert_ne!(unknown, symbolic);
    assert_ne!(unsized, symbolic);

    assert_eq!(unknown.add(&LayoutValue::Known(1)), LayoutValue::Unknown);
    assert_eq!(unsized.add(&LayoutValue::Known(1)), LayoutValue::Unsized);
}

#[test]
fn alignment_must_not_be_zero() {
    assert_eq!(
        LayoutValue::Known(10).align_up(&LayoutValue::Known(0)),
        Err(LayoutError::ZeroAlignment)
    );

    assert_eq!(
        TypeLayout::scalar(
            LayoutValue::Known(4),
            LayoutValue::Known(0),
        ),
        Err(LayoutError::ZeroAlignment)
    );
}

#[test]
fn alignment_does_not_assume_power_of_two() {
    assert_eq!(
        LayoutValue::Known(10)
            .align_up(&LayoutValue::Known(3))
            .expect("positive alignment"),
        LayoutValue::Known(12)
    );

    assert_eq!(
        LayoutValue::Known(12)
            .align_up(&LayoutValue::Known(3))
            .expect("already aligned"),
        LayoutValue::Known(12)
    );
}

#[test]
fn scalar_layout_preserves_size_alignment_and_stride() {
    let layout = scalar_layout(3, 2);

    assert_eq!(layout.kind(), &LayoutKind::Scalar);
    assert_eq!(layout.size(), &LayoutValue::Known(3));
    assert_eq!(layout.alignment(), &LayoutValue::Known(2));
    assert_eq!(layout.stride(), &LayoutValue::Known(4));
    assert!(layout.is_fully_concrete());
}

#[test]
fn record_layout_preserves_field_order_and_computes_offsets() {
    let engine = LayoutEngine;

    let layout = engine
        .record(
            vec![
                (Some("first".to_owned()), scalar_layout(1, 1)),
                (Some("second".to_owned()), scalar_layout(4, 4)),
                (Some("third".to_owned()), scalar_layout(2, 2)),
            ],
            None,
        )
        .expect("valid record layout");

    assert_eq!(layout.kind(), &LayoutKind::Record);
    assert_eq!(layout.fields().len(), 3);
    assert_eq!(layout.fields()[0].name(), Some("first"));
    assert_eq!(layout.fields()[0].offset(), &LayoutValue::Known(0));
    assert_eq!(layout.fields()[1].name(), Some("second"));
    assert_eq!(layout.fields()[1].offset(), &LayoutValue::Known(4));
    assert_eq!(layout.fields()[2].name(), Some("third"));
    assert_eq!(layout.fields()[2].offset(), &LayoutValue::Known(8));
    assert_eq!(layout.size(), &LayoutValue::Known(12));
    assert!(layout.is_fully_concrete());
}

#[test]
fn empty_record_requires_an_explicit_layout_policy() {
    assert_eq!(
        LayoutEngine.record(Vec::new(), None),
        Err(LayoutError::EmptyRecordNeedsPolicy)
    );
}

#[test]
fn explicit_minimum_alignment_is_a_policy_input() {
    let layout = LayoutEngine
        .record(
            vec![(Some("value".to_owned()), scalar_layout(3, 1))],
            Some(LayoutValue::Known(8)),
        )
        .expect("explicit minimum alignment");

    assert_eq!(layout.alignment(), &LayoutValue::Known(8));
    assert_eq!(layout.size(), &LayoutValue::Known(8));
}

#[test]
fn symbolic_array_length_remains_symbolic() {
    let length = LayoutValue::symbolic(
        LayoutExpression::symbol("runtime_length")
            .expect("symbolic length"),
    );

    let layout = LayoutEngine
        .array(scalar_layout(4, 4), length.clone())
        .expect("symbolically sized array");

    assert_eq!(
        layout.kind(),
        &LayoutKind::Array { length }
    );
    assert!(matches!(layout.size(), LayoutValue::Symbolic(_)));
    assert_eq!(layout.alignment(), &LayoutValue::Known(4));
}

#[test]
fn array_size_overflow_is_preserved_symbolically() {
    let layout = LayoutEngine
        .array(
            scalar_layout(2, 1),
            LayoutValue::Known(usize::MAX),
        )
        .expect("array layout should preserve overflow symbolically");

    assert!(matches!(layout.size(), LayoutValue::Symbolic(_)));
}

#[test]
fn unsized_array_length_is_rejected() {
    assert_eq!(
        LayoutEngine.array(
            scalar_layout(4, 4),
            LayoutValue::Unsized,
        ),
        Err(LayoutError::InvalidArrayLength)
    );
}

#[test]
fn unsized_array_element_is_rejected() {
    let element = TypeLayout::unsized(LayoutValue::Known(1))
        .expect("valid unsized element descriptor");

    assert_eq!(
        LayoutEngine.array(element, LayoutValue::Known(10)),
        Err(LayoutError::UnsizedArrayElement)
    );
}

#[test]
fn opaque_layout_does_not_claim_concrete_target_support() {
    let layout = TypeLayout::opaque(
        LayoutValue::Unknown,
        LayoutValue::Known(8),
        LayoutValue::Unknown,
    )
    .expect("opaque layout with valid alignment");

    assert_eq!(layout.kind(), &LayoutKind::Opaque);
    assert!(!layout.is_fully_concrete());
}

#[test]
fn layout_operations_are_deterministic_for_identical_inputs() {
    let engine = LayoutEngine;

    let first = engine
        .record(
            vec![
                (Some("a".to_owned()), scalar_layout(2, 2)),
                (Some("b".to_owned()), scalar_layout(8, 8)),
            ],
            None,
        )
        .expect("first layout");

    let second = engine
        .record(
            vec![
                (Some("a".to_owned()), scalar_layout(2, 2)),
                (Some("b".to_owned()), scalar_layout(8, 8)),
            ],
            None,
        )
        .expect("second layout");

    assert_eq!(first, second);
}
