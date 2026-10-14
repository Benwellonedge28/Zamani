#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration-oriented tests for Zamani's canonical semantic generic API.
//!
//! Path: src/frontend/semantic/tests/generics.rs
//!
//! PURPOSE
//! Verify generic argument binding, substitutions, parameter bounds,
//! where-clause obligations, deferred results, error propagation, determinism,
//! and scalability using the canonical AST and SemanticType representation.
//!
//! AUTHORITY
//! - grammar/functions/generics.g4
//! - grammar/types/generic.g4
//! - grammar/declarations/types.g4
//! - grammar/specification/types.md
//! - grammar/specification/semantics.md
//! - src/frontend/ast/node/generics/parameter.rs
//! - src/frontend/ast/node/generics/where_clause.rs
//! - src/frontend/ast/node/types/type_expr.rs
//! - src/frontend/semantic/types/generics.rs
//! - src/frontend/semantic/types/ty.rs
//!
//! OWNERSHIP
//! This file tests the generic semantic API. It does not implement a second
//! generic binder, production type resolver, trait registry, or compiler pass.
//!
//! TEST ADAPTER
//! TestTypeSystem is a deliberately small, deterministic adapter implementing
//! GenericTypeSystem. It supplies controlled bound outcomes so the generic
//! orchestration can be tested independently of the still-separate production
//! type-resolution and trait-resolution integration.
//!
//! A passing test with this adapter is not evidence that production trait
//! resolution or the complete compiler pipeline is integrated.
//!
//! SCALABILITY
//! The tests impose no language-level generic arity or bound-count limit.
//! A finite stress sample checks collection behavior without claiming that a
//! finite test proves support for literally infinite input.
//!
//! INTEGRATION
//! Register this module from src/frontend/semantic/tests/mod.rs after the
//! frontend and semantic module hierarchy is exposed by the crate.
//!
//! FREEZE CRITERIA
//! - Tests compile against the repository's canonical public APIs.
//! - Generic constraints are never silently treated as proven when deferred.
//! - Substitutions and diagnostics have deterministic behavior.
//! - Invalid inputs produce structured errors rather than panics.
//! - Workspace formatting, tests, and lint gates pass.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::generics::parameter::TypeParameter;
use crate::frontend::ast::node::generics::where_clause::{
    WhereClause, WherePredicate,
};
use crate::frontend::ast::node::types::type_expr::{
    TypeExpr, TypeParameterName, TypePath,
};
use crate::frontend::semantic::types::generics::{
    bind_type_arguments, lookup_parameter, validate_parameters,
    ConstraintEvaluation, ConstraintOrigin, GenericError, GenericSubstitution,
    GenericTypeSystem,
};
use crate::frontend::semantic::types::ty::{
    QualifiedName, SemanticName, SemanticType,
};

// =============================================================================
// Test adapter
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestTypeError {
    message: String,
}

impl TestTypeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TestTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TestTypeError {}

/// A controlled adapter for exercising the generic binder's public contract.
///
/// Its bound rules are test fixtures, not Zamani's normative trait rules.
/// The production implementation must delegate these decisions to the
/// canonical type, trait, and constraint services.
#[derive(Default)]
struct TestTypeSystem;

impl TestTypeSystem {
    fn named_type(name: &str) -> SemanticType {
        SemanticType::named(QualifiedName::single(name))
    }

    fn source_name(expression: &TypeExpr) -> Option<&str> {
        match expression {
            TypeExpr::Identifier(path) => {
                path.last().map(|name| name.as_str())
            }
            _ => None,
        }
    }

    fn semantic_name(ty: &SemanticType) -> Option<&str> {
        match ty {
            SemanticType::Named { name, .. } => {
                name.last().map(SemanticName::as_str)
            }
            _ => None,
        }
    }
}

impl GenericTypeSystem for TestTypeSystem {
    type Type = SemanticType;
    type Error = TestTypeError;

    fn resolve_type_expr(
        &mut self,
        expression: &TypeExpr,
        substitutions: &GenericSubstitution<Self::Type>,
    ) -> Result<Self::Type, Self::Error> {
        match expression {
            TypeExpr::Identifier(path) => {
                if path.is_empty() {
                    return Err(TestTypeError::new(
                        "cannot resolve an empty type path",
                    ));
                }

                let name = QualifiedName::from_segments(
                    path.segments()
                        .iter()
                        .map(|segment| segment.as_str()),
                );

                Ok(SemanticType::named(name))
            }

            TypeExpr::GenericParameter(parameter) => substitutions
                .get(parameter.as_str())
                .cloned()
                .ok_or_else(|| {
                    TestTypeError::new(format!(
                        "unresolved generic parameter `{}`",
                        parameter.as_str()
                    ))
                }),

            TypeExpr::Generic { base, arguments } => {
                let resolved_base =
                    self.resolve_type_expr(base, substitutions)?;

                let resolved_arguments = arguments
                    .iter()
                    .map(|argument| {
                        self.resolve_type_expr(argument, substitutions)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                match resolved_base {
                    SemanticType::Named { name, .. } => {
                        Ok(SemanticType::applied(name, resolved_arguments))
                    }
                    _ => Err(TestTypeError::new(
                        "generic constructor must resolve to a named type",
                    )),
                }
            }

            _ => Err(TestTypeError::new(format!(
                "test adapter does not implement resolution for `{expression}`"
            ))),
        }
    }

    fn check_bound(
        &mut self,
        actual: &Self::Type,
        bound: &TypeExpr,
        _substitutions: &GenericSubstitution<Self::Type>,
    ) -> Result<ConstraintEvaluation, Self::Error> {
        let bound_name = Self::source_name(bound).ok_or_else(|| {
            TestTypeError::new("test bound must be a named type")
        })?;

        let actual_name = Self::semantic_name(actual).ok_or_else(|| {
            TestTypeError::new(
                "test adapter cannot establish a bound for this type shape",
            )
        })?;

        match bound_name {
            // Test fixture: these named types satisfy the Numeric fixture.
            "Numeric" if matches!(actual_name, "i32" | "f64") => {
                Ok(ConstraintEvaluation::Satisfied)
            }

            // Test fixture: a controlled deferred obligation.
            "DeferredBound" => Ok(ConstraintEvaluation::Deferred {
                reason: "the test fixture has no evidence yet".to_owned(),
            }),

            // The test fixture does not treat an unknown bound as satisfied.
            _ => Ok(ConstraintEvaluation::Unsatisfied),
        }
    }
}

// =============================================================================
// Shared construction helpers
// =============================================================================

fn source_named(name: &str) -> TypeExpr {
    TypeExpr::Identifier(TypePath::single(name))
}

fn source_parameter(name: &str) -> TypeExpr {
    TypeExpr::GenericParameter(TypeParameterName::new(name))
}

fn semantic_named(name: &str) -> SemanticType {
    TestTypeSystem::named_type(name)
}

fn numeric_parameter(name: &str) -> TypeParameter {
    TypeParameter::with_bounds(name, vec![source_named("Numeric")])
}

// =============================================================================
// Parameter validation
// =============================================================================

#[test]
fn accepts_empty_generic_parameter_list() {
    let parameters: Vec<TypeParameter> = Vec::new();

    assert!(validate_parameters::<TestTypeError>(&parameters).is_ok());
}

#[test]
fn accepts_distinct_parameter_names() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("U"),
        TypeParameter::new("Value"),
    ];

    assert!(validate_parameters::<TestTypeError>(&parameters).is_ok());
}

#[test]
fn rejects_duplicate_parameter_names_with_both_positions() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("U"),
        TypeParameter::new("T"),
    ];

    let result = validate_parameters::<TestTypeError>(&parameters);

    assert!(matches!(
        result,
        Err(GenericError::DuplicateParameter {
            name,
            first_index: 0,
            duplicate_index: 2,
        }) if name == "T"
    ));
}

#[test]
fn rejects_whitespace_only_parameter_names() {
    let parameters = vec![TypeParameter::new("   ")];

    let result = validate_parameters::<TestTypeError>(&parameters);

    assert!(matches!(
        result,
        Err(GenericError::EmptyParameterName { index: 0 })
    ));
}

#[test]
fn parameter_validation_does_not_conflate_case_distinct_names() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("t"),
    ];

    // Name normalization and case rules belong to the language's canonical
    // identifier/name-resolution contract, not this generic binding module.
    assert!(validate_parameters::<TestTypeError>(&parameters).is_ok());
}

// =============================================================================
// Argument binding and substitutions
// =============================================================================

#[test]
fn binds_one_argument_to_one_parameter() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("Value")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("matching generic arity should bind");

    assert!(binding.is_complete());
    assert_eq!(binding.substitutions().len(), 1);
    assert_eq!(
        binding.substitutions().get("T"),
        Some(&semantic_named("Value"))
    );
}

#[test]
fn binds_multiple_arguments_without_cross_contamination() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("U"),
    ];
    let arguments = vec![
        semantic_named("LeftValue"),
        semantic_named("RightValue"),
    ];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("both arguments should bind independently");

    assert_eq!(
        binding.substitutions().get("T"),
        Some(&semantic_named("LeftValue"))
    );
    assert_eq!(
        binding.substitutions().get("U"),
        Some(&semantic_named("RightValue"))
    );
}

#[test]
fn accepts_zero_parameters_and_zero_arguments() {
    let binding = bind_type_arguments(
        &[],
        &[],
        None,
        &mut TestTypeSystem,
    )
    .expect("a nongeneric declaration has valid zero arity");

    assert!(binding.is_complete());
    assert!(binding.substitutions().is_empty());
    assert_eq!(binding.pending_constraint_count(), 0);
}

#[test]
fn rejects_too_few_arguments() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("U"),
    ];
    let arguments = vec![semantic_named("OnlyOne")];

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::ArityMismatch {
            expected: 2,
            actual: 1,
        })
    ));
}

#[test]
fn rejects_too_many_arguments() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![
        semantic_named("First"),
        semantic_named("Unexpected"),
    ];

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::ArityMismatch {
            expected: 1,
            actual: 2,
        })
    ));
}

#[test]
fn substitution_iteration_is_deterministic() {
    let parameters = vec![
        TypeParameter::new("Z"),
        TypeParameter::new("A"),
        TypeParameter::new("M"),
    ];
    let arguments = vec![
        semantic_named("TypeZ"),
        semantic_named("TypeA"),
        semantic_named("TypeM"),
    ];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("the arguments should bind");

    let names = binding
        .substitutions()
        .iter()
        .map(|(name, _)| name.to_owned())
        .collect::<Vec<_>>();

    // The public substitution API documents BTreeMap name ordering.
    assert_eq!(names, vec!["A", "M", "Z"]);

    // Deterministic iteration does not change the declared parameter mapping.
    assert_eq!(
        binding.substitutions().get("Z"),
        Some(&semantic_named("TypeZ"))
    );
}

// =============================================================================
// Bounds and deferred obligations
// =============================================================================

#[test]
fn accepts_an_argument_that_satisfies_its_bound() {
    let parameters = vec![numeric_parameter("T")];
    let arguments = vec![semantic_named("i32")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("the Numeric fixture accepts i32");

    assert!(binding.is_complete());
    assert_eq!(binding.satisfied_constraint_count(), 1);
    assert_eq!(binding.pending_constraint_count(), 0);
}

#[test]
fn rejects_an_argument_that_violates_its_bound() {
    let parameters = vec![numeric_parameter("T")];
    let arguments = vec![semantic_named("Text")];

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::UnsatisfiedParameterBound {
            parameter,
            bound,
            bound_index: 0,
        }) if parameter == "T" && bound == "Numeric"
    ));
}

#[test]
fn preserves_deferred_parameter_bounds_as_pending_obligations() {
    let parameters = vec![TypeParameter::with_bounds(
        "T",
        vec![source_named("DeferredBound")],
    )];
    let arguments = vec![semantic_named("SomeType")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("a deferred bound is pending work, not proof of failure");

    assert!(!binding.is_complete());
    assert_eq!(binding.pending_constraint_count(), 1);
    assert_eq!(binding.satisfied_constraint_count(), 0);

    let pending = &binding.pending_constraints()[0];

    assert_eq!(pending.actual_type(), &semantic_named("SomeType"));
    assert_eq!(pending.bound(), &source_named("DeferredBound"));
    assert_eq!(
        pending.origin(),
        &ConstraintOrigin::Parameter {
            parameter: "T".to_owned(),
            bound_index: 0,
        }
    );
    assert!(!pending.reason().is_empty());
}

#[test]
fn retains_satisfied_count_when_a_later_bound_is_deferred() {
    let parameters = vec![TypeParameter::with_bounds(
        "T",
        vec![
            source_named("Numeric"),
            source_named("DeferredBound"),
        ],
    )];
    let arguments = vec![semantic_named("i32")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("the unresolved second bound should remain pending");

    assert_eq!(binding.satisfied_constraint_count(), 1);
    assert_eq!(binding.pending_constraint_count(), 1);
    assert!(!binding.is_complete());

    assert_eq!(
        binding.pending_constraints()[0].origin(),
        &ConstraintOrigin::Parameter {
            parameter: "T".to_owned(),
            bound_index: 1,
        }
    );
}

// =============================================================================
// where-clause integration
// =============================================================================

#[test]
fn checks_where_clause_using_the_complete_substitution() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("i32")];

    let where_clause = WhereClause::from_predicates(vec![
        WherePredicate::with_bounds(
            source_parameter("T"),
            vec![source_named("Numeric")],
        ),
    ]);

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        Some(&where_clause),
        &mut TestTypeSystem,
    )
    .expect("the where-clause subject T should resolve to i32");

    assert!(binding.is_complete());
    assert_eq!(binding.satisfied_constraint_count(), 1);
}

#[test]
fn reports_an_unsatisfied_where_clause_bound() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("Text")];

    let where_clause = WhereClause::from_predicates(vec![
        WherePredicate::with_bounds(
            source_parameter("T"),
            vec![source_named("Numeric")],
        ),
    ]);

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        Some(&where_clause),
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::UnsatisfiedWhereBound {
            predicate_index: 0,
            bound,
            bound_index: 0,
        }) if bound == "Numeric"
    ));
}

#[test]
fn rejects_a_where_predicate_without_bounds() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("i32")];

    let where_clause = WhereClause::from_predicates(vec![
        WherePredicate::new(source_parameter("T")),
    ]);

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        Some(&where_clause),
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::EmptyWherePredicate {
            predicate_index: 0,
        })
    ));
}

#[test]
fn propagates_unresolved_where_clause_subject_errors() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("i32")];

    let where_clause = WhereClause::from_predicates(vec![
        WherePredicate::with_bounds(
            source_parameter("Missing"),
            vec![source_named("Numeric")],
        ),
    ]);

    let result = bind_type_arguments(
        &parameters,
        &arguments,
        Some(&where_clause),
        &mut TestTypeSystem,
    );

    assert!(matches!(
        result,
        Err(GenericError::Resolution { .. })
    ));
}

#[test]
fn records_multiple_where_clause_obligations_in_source_order() {
    let parameters = vec![
        TypeParameter::new("T"),
        TypeParameter::new("U"),
    ];
    let arguments = vec![
        semantic_named("i32"),
        semantic_named("SomeType"),
    ];

    let where_clause = WhereClause::from_predicates(vec![
        WherePredicate::with_bounds(
            source_parameter("T"),
            vec![source_named("Numeric")],
        ),
        WherePredicate::with_bounds(
            source_parameter("U"),
            vec![source_named("DeferredBound")],
        ),
    ]);

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        Some(&where_clause),
        &mut TestTypeSystem,
    )
    .expect("the second predicate should remain deferred");

    assert_eq!(binding.satisfied_constraint_count(), 1);
    assert_eq!(binding.pending_constraint_count(), 1);

    assert_eq!(
        binding.pending_constraints()[0].origin(),
        &ConstraintOrigin::WhereClause {
            predicate_index: 1,
            bound_index: 0,
        }
    );
}

// =============================================================================
// Nested type expressions and substitution
// =============================================================================

#[test]
fn resolves_a_nested_generic_type_using_bound_arguments() {
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("Element")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("T should bind to Element");

    let source_type = TypeExpr::generic(
        TypeExpr::name("Container"),
        vec![TypeExpr::generic(
            TypeExpr::name("Sequence"),
            vec![source_parameter("T")],
        )],
    );

    let resolved = TestTypeSystem
        .resolve_type_expr(
            &mut TestTypeSystem,
            &source_type,
            binding.substitutions(),
        )
        .expect("nested generic arguments should resolve recursively");

    let expected = SemanticType::applied(
        QualifiedName::single("Container"),
        vec![SemanticType::applied(
            QualifiedName::single("Sequence"),
            vec![semantic_named("Element")],
        )],
    );

    assert_eq!(resolved, expected);
}

#[test]
fn lookup_parameter_uses_the_canonical_ast_parameter_name() {
    let mut substitutions = GenericSubstitution::new();

    // This test module exercises the public generic substitution contract.
    // Insertion itself remains owned by the generic binder.
    let parameters = vec![TypeParameter::new("T")];
    let arguments = vec![semantic_named("ResolvedValue")];

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("the substitution should be constructed");

    // Avoid inventing a second insertion API: use the binding's canonical map.
    substitutions = binding.substitutions().clone();

    let parameter_name = TypeParameterName::new("T");
    let result = lookup_parameter(&parameter_name, &substitutions);

    assert_eq!(result, Some(&semantic_named("ResolvedValue")));
}

// =============================================================================
// Representative scalability and domain-neutrality checks
// =============================================================================

#[test]
fn supports_a_large_parameter_collection_without_a_language_arity_ceiling() {
    // This is a regression sample, not a maximum supported arity.
    // The production language must not interpret this sample size as a limit.
    let sample_size = 256usize;

    let parameters = (0..sample_size)
        .map(|index| TypeParameter::new(format!("T{index}")))
        .collect::<Vec<_>>();

    let arguments = (0..sample_size)
        .map(|index| semantic_named(&format!("Type{index}")))
        .collect::<Vec<_>>();

    let binding = bind_type_arguments(
        &parameters,
        &arguments,
        None,
        &mut TestTypeSystem,
    )
    .expect("the test sample should bind without an arbitrary small ceiling");

    assert!(binding.is_complete());
    assert_eq!(binding.substitutions().len(), sample_size);

    for index in [0, sample_size / 2, sample_size - 1] {
        assert_eq!(
            binding.substitutions().get(&format!("T{index}")),
            Some(&semantic_named(&format!("Type{index}")))
        );
    }
}

#[test]
fn generic_binding_does_not_require_hardware_specific_type_names() {
    let parameters = vec![TypeParameter::new("T")];

    // These are semantic type names in this test adapter, not device claims.
    let domain_types = [
        "ClassicalValue",
        "QuantumState",
        "HardwareSignal",
        "TensorValue",
        "DistributedValue",
    ];

    for domain_type in domain_types {
        let arguments = vec![semantic_named(domain_type)];

        let binding = bind_type_arguments(
            &parameters,
            &arguments,
            None,
            &mut TestTypeSystem,
        )
        .expect("unbounded generic binding should remain domain-neutral");

        assert_eq!(
            binding.substitutions().get("T"),
            Some(&semantic_named(domain_type))
        );
    }
}