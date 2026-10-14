
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Semantic type-system integration tests
//!
//! Path: `src/frontend/semantic/tests/types.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! PURPOSE
//!
//! Verify the integration contracts of the canonical semantic type system:
//! type construction and validation, symbolic cardinalities, unification,
//! transactional rollback, compatibility, coercion policy, and exact
//! compile-time integer representation.
//!
//! AUTHORITY
//!
//! - `grammar/specification/types.md`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/types/types.g4`
//! - `grammar/declarations/types.g4`
//! - `src/frontend/ast/node/types/type_expr.rs`
//! - `src/frontend/semantic/types/`
//!
//! BOUNDARIES
//!
//! These tests verify semantic type APIs. They do not treat this test module
//! as the implementation of AST-to-semantic type resolution, generic bound
//! solving, ownership analysis, or domain-specific validation.
//!
//! A type that is structurally representable is not automatically a valid
//! program. A symbolic obligation is not automatically proven. Target
//! feasibility is not part of source-level type compatibility.
//!
//! INTEGRATION
//!
//! Include this module from `src/frontend/semantic/tests/mod.rs` with:
//!
//!     mod types;
//!
//! The semantic test module must itself be included from the semantic
//! subsystem's test configuration. These tests use public semantic APIs,
//! not private implementation details.
//!
//! SCALABILITY
//!
//! Tests use symbolic values and decimal strings beyond native integer
//! widths. No test assumes a fixed hardware capacity, generic arity,
//! tensor rank, register width, or quantum-resource count.
//!
//! FREEZE CRITERIA
//!
//! - This module compiles against the public APIs it exercises.
//! - The semantic type, unification, compatibility, constant, and coercion
//!   contracts tested here pass.
//! - The complete workspace and semantic integration suite pass.
//! - AST resolver and parser-to-semantic integration tests also pass.

use crate::frontend::semantic::types::{
    aliases,
    coercions::{
        CoercionContext, CoercionDecision, CoercionProperties, CoercionRegistry,
        CoercionSite, ExactTypeRule,
    },
    compatibility::{
        check_compatibility, ArrayLength, Compatibility, CompatibilityPolicy,
        TypeShape, TypeView,
    },
    constants::{BigInt, ExactDecimal},
    constraints::{
        ConstraintSet, ConstraintStatus, TypeConstraint, TypeRelation,
        UnresolvedReason,
    },
    ty::{
        Cardinality, IntegerSignedness, IntegerWidth, PrimitiveType,
        QualifiedName, SemanticName, SemanticType, TypeLevelValue,
        TypeValidationError, TypeValidationPolicy, TypeVariableId,
    },
    unification::{
        TypeUnifier, UnificationConfig, UnificationError,
    },
};

fn boolean_type() -> SemanticType {
    SemanticType::Primitive(PrimitiveType::Boolean)
}

fn string_type() -> SemanticType {
    SemanticType::Primitive(PrimitiveType::String)
}

fn integer_type(width: &str) -> SemanticType {
    SemanticType::integer(
        IntegerSignedness::Signed,
        IntegerWidth::bits(width).expect("test width must be valid"),
    )
}

fn named_type(name: &str) -> SemanticType {
    SemanticType::named(QualifiedName::single(name))
}

fn variable(id: u64) -> SemanticType {
    SemanticType::Variable(TypeVariableId(id))
}

fn known_cardinality(decimal: &str) -> Cardinality {
    Cardinality::Known(
        TypeLevelValue::integer(decimal)
            .expect("test cardinality must be a valid non-negative integer"),
    )
}

// =============================================================================
// Canonical semantic type representation
// =============================================================================

#[test]
fn primitive_types_use_the_canonical_semantic_representation() {
    assert_eq!(
        boolean_type(),
        SemanticType::Primitive(PrimitiveType::Boolean)
    );

    assert_eq!(
        string_type(),
        SemanticType::Primitive(PrimitiveType::String)
    );
}

#[test]
fn named_types_preserve_qualified_identity() {
    let left = SemanticType::named(QualifiedName::from_segments([
        "package",
        "module",
        "Record",
    ]));

    let right = SemanticType::named(QualifiedName::from_segments([
        "package",
        "other_module",
        "Record",
    ]));

    assert_ne!(
        left, right,
        "different qualified names must not collapse into one type"
    );
}

#[test]
fn generic_arguments_are_preserved_in_order() {
    let first = SemanticType::applied(
        QualifiedName::single("Container"),
        vec![boolean_type(), string_type()],
    );

    let second = SemanticType::applied(
        QualifiedName::single("Container"),
        vec![string_type(), boolean_type()],
    );

    assert_ne!(
        first, second,
        "generic argument order is semantically significant"
    );
}

#[test]
fn symbolic_integer_width_is_not_narrowed_to_host_width() {
    let width_expression = "width_parameter * lanes + reserved_bits";
    let width = IntegerWidth::symbolic(width_expression)
        .expect("non-empty symbolic width");

    assert_eq!(width.fixed_decimal(), None);
    assert_eq!(
        width,
        IntegerWidth::Symbolic(width_expression.into())
    );
    assert!(width.validate().is_ok());
}

#[test]
fn arbitrarily_large_fixed_integer_width_is_preserved() {
    let decimal_width =
        "123456789012345678901234567890123456789012345678901234567890";

    let width =
        IntegerWidth::bits(decimal_width).expect("valid positive decimal width");

    assert_eq!(width.fixed_decimal(), Some(decimal_width));
    assert!(width.validate().is_ok());
}

#[test]
fn invalid_integer_widths_are_rejected() {
    for invalid in ["", "0", "000", "-1", "+8", "8.5", "width"] {
        assert!(
            IntegerWidth::bits(invalid).is_err(),
            "width {invalid:?} must not be accepted as a positive decimal width"
        );
    }
}

#[test]
fn symbolic_cardinality_is_retained_without_evaluation() {
    let symbolic = TypeLevelValue::Expression {
        operator: SemanticName::new("+"),
        operands: vec![
            TypeLevelValue::Parameter(SemanticName::new("N")),
            TypeLevelValue::integer("1").expect("valid integer"),
        ],
    };

    let ty = SemanticType::array(
        integer_type("64"),
        Cardinality::Symbolic(symbolic.clone()),
    );

    assert_eq!(
        ty,
        SemanticType::Array {
            element: Box::new(integer_type("64")),
            cardinality: Cardinality::Symbolic(symbolic),
        }
    );
    assert!(
        !ty.is_incomplete(),
        "a well-formed symbolic cardinality is not itself an inference error"
    );
}

#[test]
fn distinct_known_array_lengths_remain_distinct() {
    let four = SemanticType::array(
        boolean_type(),
        known_cardinality("4"),
    );
    let five = SemanticType::array(
        boolean_type(),
        known_cardinality("5"),
    );

    assert_ne!(
        four, five,
        "array cardinality must not be silently discarded"
    );
}

#[test]
fn error_and_inference_variable_types_are_incomplete() {
    assert!(SemanticType::Error.is_incomplete());
    assert!(variable(17).is_incomplete());

    assert!(
        !boolean_type().is_incomplete(),
        "a fully resolved primitive should not be marked incomplete"
    );
}

#[test]
fn type_validation_rejects_invalid_structure() {
    let invalid_name = SemanticType::named(QualifiedName::single(""));

    assert!(
        matches!(
            invalid_name.validate(&TypeValidationPolicy::default()),
            Err(TypeValidationError::EmptyName)
        ),
        "an empty semantic name must not pass structural validation"
    );
}

#[test]
fn type_validation_obeys_explicit_operational_depth_budget() {
    let nested = SemanticType::Optional(Box::new(
        SemanticType::Optional(Box::new(boolean_type())),
    ));

    let policy = TypeValidationPolicy {
        max_depth: Some(0),
    };

    assert!(matches!(
        nested.validate(&policy),
        Err(TypeValidationError::TraversalBudgetExceeded { limit: 0 })
    ));

    assert!(
        nested.validate(&TypeValidationPolicy::default()).is_ok(),
        "an absent explicit depth budget must not introduce an arbitrary fixed limit"
    );
}

// =============================================================================
// Unification
// =============================================================================

#[test]
fn unifier_binds_and_resolves_an_inference_variable() {
    let mut unifier = TypeUnifier::new();
    let inference_variable = variable(1);
    let expected = integer_type("64");

    unifier
        .unify(&inference_variable, &expected)
        .expect("variable should unify with a concrete type");

    assert_eq!(
        unifier
            .resolve(&inference_variable)
            .expect("binding should resolve"),
        expected
    );
}

#[test]
fn unifier_resolves_transitive_bindings() {
    let mut unifier = TypeUnifier::new();
    let first = variable(101);
    let second = variable(102);
    let concrete = boolean_type();

    unifier
        .unify(&first, &second)
        .expect("variables should unify");

    unifier
        .unify(&second, &concrete)
        .expect("second variable should bind to concrete type");

    assert_eq!(
        unifier.resolve(&first).expect("first binding resolves"),
        concrete
    );
}

#[test]
fn unifier_rejects_recursive_type_variable_binding() {
    let mut unifier = TypeUnifier::new();
    let inference_variable = variable(7);

    let recursive_type =
        SemanticType::Optional(Box::new(inference_variable.clone()));

    assert!(matches!(
        unifier.unify(&inference_variable, &recursive_type),
        Err(UnificationError::OccursCheck { .. })
    ));

    assert_eq!(
        unifier.binding_count(),
        0,
        "failed occurs checks must not leave a partial binding"
    );
}

#[test]
fn failed_unification_rolls_back_all_partial_bindings() {
    let mut unifier = TypeUnifier::new();

    let left = SemanticType::Tuple(vec![
        variable(1),
        named_type("module::Left"),
    ]);

    let right = SemanticType::Tuple(vec![
        integer_type("32"),
        named_type("module::Right"),
    ]);

    assert!(
        unifier.unify(&left, &right).is_err(),
        "distinct nominal types must not unify merely because they are both named"
    );

    assert_eq!(
        unifier.binding_count(),
        0,
        "a failed compound unification must be transactional"
    );
}

#[test]
fn unifier_rejects_mismatched_known_array_cardinalities() {
    let left = SemanticType::array(
        integer_type("64"),
        known_cardinality("4"),
    );

    let right = SemanticType::array(
        integer_type("64"),
        known_cardinality("5"),
    );

    assert!(
        unifier_result_is_error(&left, &right),
        "different known array lengths must not be treated as equal"
    );
}

fn unifier_result_is_error(
    left: &SemanticType,
    right: &SemanticType,
) -> bool {
    TypeUnifier::new().unify(left, right).is_err()
}

#[test]
fn unifier_does_not_certify_error_recovery_types() {
    let mut unifier = TypeUnifier::new();

    assert!(matches!(
        unifier.unify(&SemanticType::Error, &boolean_type()),
        Err(UnificationError::ErrorTypeParticipates)
    ));

    assert_eq!(unifier.binding_count(), 0);
}

#[test]
fn unifier_rejects_zero_work_budget() {
    assert!(matches!(
        TypeUnifier::with_config(UnificationConfig {
            max_work: Some(0),
        }),
        Err(UnificationError::InvalidConfiguration)
    ));
}

// =============================================================================
// Compatibility contract
// =============================================================================

// The compatibility engine is deliberately expressed over TypeShape.
// This adapter is a test fixture only; it is not another production type
// representation and does not replace SemanticType.

#[derive(Clone, Debug, PartialEq, Eq)]
enum CompatibilityFixture {
    Primitive(&'static str),
    Nominal {
        identity: &'static str,
        arguments: Vec<Self>,
    },
    Tuple(Vec<Self>),
    Array {
        element: Box<Self>,
        length: &'static str,
    },
    InferenceVariable(u64),
    Unresolved,
    Error,
}

impl TypeShape for CompatibilityFixture {
    fn type_view(&self) -> TypeView<'_, Self> {
        match self {
            Self::Primitive(name) => TypeView::Primitive(name),

            Self::Nominal {
                identity,
                arguments,
            } => TypeView::Nominal {
                identity,
                arguments,
            },

            Self::Tuple(elements) => TypeView::Tuple(elements),

            Self::Array { element, length } => TypeView::Array {
                element,
                length: ArrayLength::Known(length),
            },

            Self::InferenceVariable(id) => {
                TypeView::InferenceVariable(*id)
            }

            Self::Unresolved => TypeView::Unresolved,
            Self::Error => TypeView::Error,
        }
    }
}

#[test]
fn compatibility_accepts_equal_resolved_primitives() {
    let actual = CompatibilityFixture::Primitive("core::bool");
    let expected = CompatibilityFixture::Primitive("core::bool");

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Compatible
    );
}

#[test]
fn compatibility_rejects_different_primitive_types() {
    let actual = CompatibilityFixture::Primitive("core::bool");
    let expected = CompatibilityFixture::Primitive("core::string");

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Incompatible
    );
}

#[test]
fn compatibility_rejects_different_nominal_identities() {
    let actual = CompatibilityFixture::Nominal {
        identity: "package_a::Record",
        arguments: Vec::new(),
    };

    let expected = CompatibilityFixture::Nominal {
        identity: "package_b::Record",
        arguments: Vec::new(),
    };

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Incompatible
    );
}

#[test]
fn compatibility_checks_generic_arguments() {
    let actual = CompatibilityFixture::Nominal {
        identity: "core::Container",
        arguments: vec![CompatibilityFixture::Primitive("core::bool")],
    };

    let expected = CompatibilityFixture::Nominal {
        identity: "core::Container",
        arguments: vec![CompatibilityFixture::Primitive("core::string")],
    };

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Incompatible
    );
}

#[test]
fn compatibility_checks_tuple_arity() {
    let actual = CompatibilityFixture::Tuple(vec![
        CompatibilityFixture::Primitive("core::bool"),
    ]);

    let expected = CompatibilityFixture::Tuple(vec![
        CompatibilityFixture::Primitive("core::bool"),
        CompatibilityFixture::Primitive("core::string"),
    ]);

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Incompatible
    );
}

#[test]
fn compatibility_checks_array_cardinality() {
    let actual = CompatibilityFixture::Array {
        element: Box::new(CompatibilityFixture::Primitive("core::bool")),
        length: "4",
    };

    let expected = CompatibilityFixture::Array {
        element: Box::new(CompatibilityFixture::Primitive("core::bool")),
        length: "5",
    };

    assert_eq!(
        check_compatibility(
            &actual,
            &expected,
            CompatibilityPolicy::exact(),
        ),
        Compatibility::Incompatible
    );
}

#[test]
fn unresolved_and_error_types_are_not_compatibility_proofs() {
    let expected = CompatibilityFixture::Primitive("core::bool");

    for actual in [
        CompatibilityFixture::Unresolved,
        CompatibilityFixture::Error,
        CompatibilityFixture::InferenceVariable(91),
    ] {
        assert_eq!(
            check_compatibility(
                &actual,
                &expected,
                CompatibilityPolicy::exact(),
            ),
            Compatibility::Undetermined,
            "unresolved and tainted types must not be certified compatible"
        );
    }
}

// =============================================================================
// Explicit coercion policy
// =============================================================================

fn test_coercion_site() -> CoercionSite {
    CoercionSite::new("tests", "types")
        .expect("static test coercion site must be valid")
}

#[test]
fn coercion_registry_does_not_invent_conversions() {
    let registry = CoercionRegistry::new();

    let result = registry
        .resolve(
            &boolean_type(),
            &string_type(),
            &CoercionContext::new(test_coercion_site()),
        )
        .expect("a coercion query should return a decision");

    assert_eq!(result, CoercionDecision::NoCoercion);
}

#[test]
fn coercion_registry_accepts_identity_without_a_rule() {
    let registry = CoercionRegistry::new();
    let ty = integer_type("64");

    let result = registry
        .resolve(
            &ty,
            &ty,
            &CoercionContext::new(test_coercion_site()),
        )
        .expect("identity conversion should be decidable");

    assert_eq!(result, CoercionDecision::Identity);
}

#[test]
fn coercion_registry_rejects_duplicate_rule_ids() {
    let mut registry = CoercionRegistry::new();

    let first = ExactTypeRule::new(
        "tests.identity-like-rule",
        boolean_type(),
        string_type(),
        CoercionProperties::preserving(10),
    );

    let second = ExactTypeRule::new(
        "tests.identity-like-rule",
        boolean_type(),
        string_type(),
        CoercionProperties::preserving(20),
    );

    registry.register(first).expect("first rule registers");

    assert!(
        registry.register(second).is_err(),
        "a duplicate rule ID must not silently replace an existing rule"
    );

    assert_eq!(
        registry.rule_ids(),
        vec!["tests.identity-like-rule"]
    );
}

#[test]
fn coercion_registry_does_not_certify_incomplete_types() {
    let registry = CoercionRegistry::new();

    let result = registry
        .resolve(
            &variable(500),
            &boolean_type(),
            &CoercionContext::new(test_coercion_site()),
        )
        .expect("an incomplete type should produce a structured decision");

    assert!(
        matches!(result, CoercionDecision::Undetermined { .. }),
        "an inference variable must not be accepted as a concrete source type"
    );
}

// =============================================================================
// Exact compile-time values and scalable cardinalities
// =============================================================================

#[test]
fn arbitrary_magnitude_integer_round_trips_without_narrowing() {
    let decimal =
        "1234567890123456789012345678901234567890123456789012345678901234567890";

    let value =
        BigInt::parse_radix(decimal, 10).expect("valid decimal integer");

    assert_eq!(value.to_string(), decimal);
    assert!(
        value.to_u128().is_none(),
        "a value outside u128 must not be silently truncated"
    );
}

#[test]
fn arbitrary_magnitude_integer_arithmetic_is_exact() {
    let left = BigInt::parse_radix(
        "999999999999999999999999999999",
        10,
    )
    .expect("valid integer");

    let right = BigInt::parse_radix(
        "1",
        10,
    )
    .expect("valid integer");

    assert_eq!(
        left.add(&right).to_string(),
        "1000000000000000000000000000000"
    );
}

#[test]
fn decimal_constant_preserves_source_precision_and_spelling() {
    let spelling = "12345678901234567890.0012300e+40";

    let value = ExactDecimal::parse(spelling.to_owned())
        .expect("valid exact decimal literal");

    assert_eq!(value.spelling(), spelling);
}

#[test]
fn malformed_constant_values_return_errors_instead_of_panicking() {
    for source in ["", "_12", "12_", "1.2.3", "4e", "1e+"] {
        assert!(
            ExactDecimal::parse(source.to_owned()).is_err(),
            "invalid decimal {source:?} must be rejected"
        );
    }

    for source in ["", "12_", "-12", "+12", "xyz"] {
        assert!(
            BigInt::parse_radix(source, 10).is_err(),
            "invalid integer {source:?} must be rejected"
        );
    }
}

// =============================================================================
// Unresolved constraints are not proven constraints
// =============================================================================

#[test]
fn type_constraints_start_unresolved_and_cannot_certify_validity() {
    let mut constraints = ConstraintSet::new();

    let id = constraints
        .insert(
            TypeConstraint::TypeRelation {
                relation: TypeRelation::Equal,
                left: named_type("module::Left"),
                right: named_type("module::Right"),
            },
            Some("types.rs test"),
        )
        .expect("well-formed constraint should be recorded");

    let stored = constraints.get(id).expect("constraint must be stored");

    assert!(stored.is_unresolved());
    assert!(!stored.is_proven());
    assert!(!constraints.all_proven());
}

#[test]
fn symbolic_type_obligations_remain_unresolved_until_proven() {
    let mut constraints = ConstraintSet::new();

    let id = constraints
        .insert(
            TypeConstraint::TypeRelation {
                relation: TypeRelation::Equal,
                left: SemanticType::array(
                    boolean_type(),
                    Cardinality::Symbolic(TypeLevelValue::Parameter(
                        SemanticName::new("N"),
                    )),
                ),
                right: SemanticType::array(
                    boolean_type(),
                    Cardinality::Symbolic(TypeLevelValue::Parameter(
                        SemanticName::new("M"),
                    )),
                ),
            },
            Some("symbolic array equality"),
        )
        .expect("symbolic type obligation should be recorded");

    constraints
        .record_status(
            id,
            ConstraintStatus::Unresolved {
                reason: UnresolvedReason::SymbolicValue,
            },
        )
        .expect("unresolved status should be recorded");

    assert_eq!(constraints.unresolved_count(), 1);
    assert!(!constraints.all_proven());
}

// =============================================================================
// Determinism and isolation
// =============================================================================

#[test]
fn independent_unifiers_do_not_share_mutable_substitutions() {
    let first_variable = variable(1);
    let second_variable = variable(1);

    let mut first = TypeUnifier::new();
    let second = TypeUnifier::new();

    first
        .unify(&first_variable, &boolean_type())
        .expect("first session should bind its variable");

    assert!(first.is_bound(TypeVariableId(1)));
    assert!(
        !second.is_bound(TypeVariableId(1)),
        "one analysis session must not mutate another session's substitutions"
    );

    assert_eq!(
        first.resolve(&first_variable).expect("first resolves"),
        boolean_type()
    );

    assert_eq!(
        second.resolve(&second_variable).expect("unbound variable remains unresolved"),
        second_variable
    );
}

#[test]
fn repeated_unification_produces_the_same_resolved_type() {
    let left = SemanticType::Tuple(vec![
        variable(11),
        integer_type("128"),
    ]);

    let right = SemanticType::Tuple(vec![
        boolean_type(),
        integer_type("128"),
    ]);

    let mut first = TypeUnifier::new();
    let mut second = TypeUnifier::new();

    let first_result = first.unify(&left, &right).expect("first unification");
    let second_result = second.unify(&left, &right).expect("second unification");

    assert_eq!(first_result, second_result);
    assert_eq!(
        first.resolve(&left).expect("first resolves"),
        second.resolve(&left).expect("second resolves")
    );
}

// =============================================================================
// Domain-neutral extensibility
// =============================================================================

#[test]
fn extension_types_can_represent_new_domains_without_core_type_variants() {
    let quantum_value = SemanticType::Extension {
        namespace: QualifiedName::from_segments(["zamani", "quantum"]),
        name: SemanticName::new("LogicalValue"),
        arguments: vec![integer_type("128")],
        attributes: Vec::new(),
    };

    let tensor_value = SemanticType::Extension {
        namespace: QualifiedName::from_segments(["zamani", "tensor"]),
        name: SemanticName::new("Tensor"),
        arguments: vec![
            boolean_type(),
            SemanticType::array(
                integer_type("64"),
                Cardinality::Symbolic(TypeLevelValue::Parameter(
                    SemanticName::new("RANK"),
                )),
            ),
        ],
        attributes: Vec::new(),
    };

    assert!(!quantum_value.is_incomplete());
    assert!(!tensor_value.is_incomplete());
    assert_ne!(quantum_value, tensor_value);

    assert!(
        quantum_value.validate(&TypeValidationPolicy::default()).is_ok()
    );
    assert!(
        tensor_value.validate(&TypeValidationPolicy::default()).is_ok()
    );
}
