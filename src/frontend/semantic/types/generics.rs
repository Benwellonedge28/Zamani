
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani semantic generics
//!
//! File: `src/frontend/semantic/types/generics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Bind source-level generic parameters to already-resolved semantic types,
//! preserve generic substitutions, and evaluate parameter bounds and
//! `where`-clause obligations through the canonical semantic type system.
//!
//! ## Ownership
//!
//! This module owns:
//! - generic argument arity checking;
//! - duplicate generic parameter detection;
//! - declaration-local substitution environments;
//! - orchestration of generic bound checks;
//! - explicit tracking of deferred generic obligations;
//! - structured errors for invalid generic applications.
//!
//! This module does not own:
//! - the canonical AST or semantic type representation;
//! - parsing or source grammar;
//! - name and symbol resolution;
//! - type unification or inference-variable solving;
//! - trait/interface implementation databases;
//! - associated-type resolution;
//! - constant evaluation;
//! - type compatibility or coercion rules;
//! - target feasibility or hardware resource allocation;
//! - IR construction or lowering.
//!
//! ## Integration contract
//!
//! The canonical source representations are:
//! - `generics::parameter::TypeParameter`;
//! - `generics::where_clause::WhereClause`;
//! - `generics::where_clause::WherePredicate`;
//! - `types::type_expr::TypeExpr`.
//!
//! The canonical resolved type is supplied by the semantic type system
//! through `GenericTypeSystem::Type`. This module intentionally does not
//! invent a competing `SemanticType` enum.
//!
//! The type system implementation must:
//! - resolve a source `TypeExpr` in the current substitution environment;
//! - determine whether a resolved type satisfies a source-level bound;
//! - return `Deferred` when a constraint cannot yet be decided;
//! - return an error for actual resolution/checking failures.
//!
//! A deferred constraint is never reported as proven.
//!
//! ## Scalability
//!
//! There are no language-level limits on generic parameter count, argument
//! count, bound count, or computational domain.
//!
//! Storage grows with the input and available compiler resources. Any
//! operational limits belong to the semantic session/configuration layer.
//! They must be reported as incomplete analysis, not as a language-level
//! rejection.
//!
//! No finite implementation can process literally infinite input. The
//! portability guarantee is that this module introduces no arbitrary
//! machine-capacity ceiling.
//!
//! ## Determinism
//!
//! - Parameter and bound traversal follows source order.
//! - Substitution lookup uses `BTreeMap` for deterministic iteration.
//! - No hash-map iteration determines diagnostics or semantic results.
//! - No implicit type conversion is performed here.
//!
//! ## Error policy
//!
//! Invalid arity, duplicate names, empty names, empty `where` predicates,
//! resolution failures, and unsatisfied constraints are explicit.
//! Unresolved constraints remain obligations and cannot certify an instance.
//!
//! ## Freeze criteria
//!
//! This module is ready for integration when:
//! - the canonical semantic type implements `GenericTypeSystem`;
//! - `types::traits` or the approved constraint service supplies sound bound
//!   checking;
//! - callers pass arguments after ordinary type resolution;
//! - deferred obligations are preserved by the semantic model;
//! - generic tests and the workspace checks pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::generics::parameter::TypeParameter;
use crate::frontend::ast::node::generics::where_clause::{
    WhereClause, WherePredicate,
};
use crate::frontend::ast::node::types::type_expr::{
    TypeExpr, TypeParameterName,
};

/// Stable local schema version for generic semantic APIs.
///
/// This is not the Zamani language version, AST schema version, or compiler
/// version.
pub const GENERICS_SEMANTIC_SCHEMA_VERSION: u16 = 1;

/// Result of checking a generic bound.
///
/// `Deferred` means that the compiler cannot yet establish whether the
/// obligation is satisfied. It is not equivalent to `Satisfied`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintEvaluation {
    /// The constraint has been proven satisfied.
    Satisfied,

    /// The constraint has been proven unsatisfied.
    Unsatisfied,

    /// More semantic information is required to decide the constraint.
    Deferred {
        /// A stable, human-readable explanation for diagnostics or tooling.
        ///
        /// Callers should not use this string as a machine-readable error
        /// code. The owning diagnostics subsystem should assign that code.
        reason: String,
    },
}

/// Origin of a pending generic constraint.
///
/// Indices refer to the original source-order collection. They are not
/// source offsets or AST node identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintOrigin {
    /// A bound attached directly to a generic parameter.
    Parameter {
        /// Parameter spelling in its declaration-local scope.
        parameter: String,

        /// Zero-based position in the parameter's bound list.
        bound_index: usize,
    },

    /// A bound in a `where`-clause predicate.
    WhereClause {
        /// Zero-based position in the `where` clause.
        predicate_index: usize,

        /// Zero-based position in that predicate's bound list.
        bound_index: usize,
    },
}

/// A generic constraint that has not yet been proven.
///
/// Keeping the actual type, source-level bound, and origin allows a later
/// pass to retry the check and produce source-aware diagnostics. The source
/// AST remains authoritative; this value is a semantic obligation, not a
/// second AST node.
#[derive(Clone, Debug)]
pub struct PendingConstraint<T> {
    actual_type: T,
    bound: TypeExpr,
    origin: ConstraintOrigin,
    reason: String,
}

impl<T> PendingConstraint<T> {
    /// The resolved type that must satisfy the bound.
    #[must_use]
    pub fn actual_type(&self) -> &T {
        &self.actual_type
    }

    /// The original source-level bound.
    #[must_use]
    pub fn bound(&self) -> &TypeExpr {
        &self.bound
    }

    /// The source-order origin of this obligation.
    #[must_use]
    pub fn origin(&self) -> &ConstraintOrigin {
        &self.origin
    }

    /// Why the constraint could not yet be decided.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Declaration-local mapping from generic parameter names to resolved types.
///
/// The map is deliberately scoped to one generic declaration or instantiation.
/// It is not a global symbol table. Name resolution and declaration identity
/// remain the responsibility of the symbol subsystem.
///
/// Once the canonical semantic model provides stable generic-parameter IDs,
/// this API can be extended to use those IDs without changing the bound
/// checking algorithm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericSubstitution<T> {
    values: BTreeMap<String, T>,
}

impl<T> Default for GenericSubstitution<T> {
    fn default() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }
}

impl<T> GenericSubstitution<T> {
    /// Creates an empty substitution environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Looks up a parameter by its source spelling.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&T> {
        self.values.get(name)
    }

    /// Returns whether a parameter name is present.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Number of substitutions in this environment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether this environment contains no substitutions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Deterministic iteration in parameter-name order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&str, &T)> {
        self.values
            .iter()
            .map(|(name, value)| (name.as_str(), value))
    }

    /// Consumes the environment and returns its mapping.
    #[must_use]
    pub fn into_map(self) -> BTreeMap<String, T> {
        self.values
    }

    fn insert_checked(
        &mut self,
        name: String,
        value: T,
    ) -> Result<(), GenericError<core::convert::Infallible>> {
        if self.values.contains_key(&name) {
            return Err(GenericError::DuplicateSubstitution { name });
        }

        self.values.insert(name, value);
        Ok(())
    }
}

/// Output of generic argument binding and constraint checking.
///
/// An instance may have a usable substitution while still carrying deferred
/// obligations. Callers must inspect `is_complete()` before treating all
/// generic constraints as proven.
#[derive(Clone, Debug)]
pub struct GenericBinding<T> {
    substitutions: GenericSubstitution<T>,
    pending: Vec<PendingConstraint<T>>,
    satisfied_constraints: usize,
}

impl<T> GenericBinding<T> {
    /// The resolved declaration-local substitutions.
    #[must_use]
    pub fn substitutions(&self) -> &GenericSubstitution<T> {
        &self.substitutions
    }

    /// Constraints that still require semantic resolution.
    #[must_use]
    pub fn pending_constraints(&self) -> &[PendingConstraint<T>] {
        &self.pending
    }

    /// Number of constraints proven satisfied during this pass.
    #[must_use]
    pub fn satisfied_constraint_count(&self) -> usize {
        self.satisfied_constraints
    }

    /// Whether all constraints examined by this pass were decided and
    /// satisfied.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty()
    }

    /// Number of unresolved obligations.
    #[must_use]
    pub fn pending_constraint_count(&self) -> usize {
        self.pending.len()
    }

    /// Consumes the result and returns the substitution and pending work.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (GenericSubstitution<T>, Vec<PendingConstraint<T>>) {
        (self.substitutions, self.pending)
    }
}

/// Errors produced by generic binding and constraint orchestration.
///
/// `E` is the error type supplied by the canonical semantic type system.
#[derive(Debug)]
#[non_exhaustive]
pub enum GenericError<E> {
    /// A generic parameter name is empty or whitespace-only.
    EmptyParameterName {
        /// Zero-based parameter index.
        index: usize,
    },

    /// A parameter name is declared more than once in the same generic list.
    DuplicateParameter {
        /// Duplicated name.
        name: String,

        /// First declaration index.
        first_index: usize,

        /// Duplicate declaration index.
        duplicate_index: usize,
    },

    /// The number of supplied arguments does not match the declaration.
    ArityMismatch {
        /// Number of declared parameters.
        expected: usize,

        /// Number of supplied arguments.
        actual: usize,
    },

    /// A substitution key was inserted more than once.
    DuplicateSubstitution {
        /// Duplicated key.
        name: String,
    },

    /// A `where` predicate has no bounds to check.
    EmptyWherePredicate {
        /// Zero-based predicate index.
        predicate_index: usize,
    },

    /// Resolving a source type expression failed.
    Resolution {
        /// Context suitable for diagnostics.
        context: String,

        /// Underlying semantic resolver error.
        source: E,
    },

    /// The type system proved that a parameter bound was not satisfied.
    UnsatisfiedParameterBound {
        /// Generic parameter name.
        parameter: String,

        /// Original bound in source form.
        bound: String,

        /// Zero-based bound index.
        bound_index: usize,
    },

    /// The type system proved that a `where`-clause bound was not satisfied.
    UnsatisfiedWhereBound {
        /// Zero-based predicate index.
        predicate_index: usize,

        /// Original bound in source form.
        bound: String,

        /// Zero-based bound index.
        bound_index: usize,
    },
}

impl<E: fmt::Display> fmt::Display for GenericError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyParameterName { index } => {
                write!(formatter, "generic parameter {index} has an empty name")
            }
            Self::DuplicateParameter {
                name,
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "generic parameter `{name}` at index {duplicate_index} \
                 duplicates the declaration at index {first_index}"
            ),
            Self::ArityMismatch { expected, actual } => write!(
                formatter,
                "generic argument count mismatch: expected {expected}, got {actual}"
            ),
            Self::DuplicateSubstitution { name } => {
                write!(formatter, "generic substitution for `{name}` already exists")
            }
            Self::EmptyWherePredicate { predicate_index } => write!(
                formatter,
                "where-clause predicate {predicate_index} has no bounds"
            ),
            Self::Resolution { context, source } => {
                write!(formatter, "failed to resolve {context}: {source}")
            }
            Self::UnsatisfiedParameterBound {
                parameter,
                bound,
                bound_index,
            } => write!(
                formatter,
                "type argument for `{parameter}` does not satisfy bound \
                 {bound_index} (`{bound}`)"
            ),
            Self::UnsatisfiedWhereBound {
                predicate_index,
                bound,
                bound_index,
            } => write!(
                formatter,
                "where-clause predicate {predicate_index} does not satisfy \
                 bound {bound_index} (`{bound}`)"
            ),
        }
    }
}

impl<E> Error for GenericError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Resolution { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Interface to the canonical semantic type and constraint systems.
///
/// This trait is the integration seam for `types/ty.rs`, `types/traits.rs`,
/// `types/aliases.rs`, and `types/unification.rs`.
///
/// Implementations must not report an unknown or error-tainted type as
/// satisfying a constraint. They should return `Deferred` when a result
/// depends on unresolved inference variables or incomplete trait information.
pub trait GenericTypeSystem {
    /// Canonical resolved semantic type.
    ///
    /// This associated type must be the repository's authoritative semantic
    /// type, not a generic-only replacement type.
    type Type: Clone;

    /// Error returned by semantic type resolution and constraint checking.
    type Error: Error + 'static;

    /// Resolve source-level type syntax using the supplied generic
    /// substitutions.
    ///
    /// Name resolution, alias expansion, and inference-variable handling
    /// remain owned by their canonical services.
    fn resolve_type_expr(
        &mut self,
        expression: &TypeExpr,
        substitutions: &GenericSubstitution<Self::Type>,
    ) -> Result<Self::Type, Self::Error>;

    /// Check whether `actual` satisfies a source-level bound.
    ///
    /// Trait/interface lookup and bound semantics belong to the canonical
    /// trait/constraint subsystem. This method must not perform target
    /// capability negotiation or resource allocation.
    fn check_bound(
        &mut self,
        actual: &Self::Type,
        bound: &TypeExpr,
        substitutions: &GenericSubstitution<Self::Type>,
    ) -> Result<ConstraintEvaluation, Self::Error>;
}

/// Validates a generic parameter list's semantic naming requirements.
///
/// This checks duplicate names and empty names only. Structural AST validation
/// of each bound must be performed by the canonical AST validation layer.
/// The function does not resolve bounds or decide their meaning.
pub fn validate_parameters<E>(
    parameters: &[TypeParameter],
) -> Result<(), GenericError<E>> {
    let mut first_indices = BTreeMap::<String, usize>::new();

    for (index, parameter) in parameters.iter().enumerate() {
        let name = parameter.name_str();

        if name.trim().is_empty() {
            return Err(GenericError::EmptyParameterName { index });
        }

        if let Some(first_index) = first_indices.get(name) {
            return Err(GenericError::DuplicateParameter {
                name: name.to_owned(),
                first_index: *first_index,
                duplicate_index: index,
            });
        }

        first_indices.insert(name.to_owned(), index);
    }

    Ok(())
}

/// Binds already-resolved type arguments to declared generic parameters and
/// checks parameter bounds plus an optional `where` clause.
///
/// Arguments are expected to have passed through ordinary type resolution
/// before this function is called. The caller must not pass syntax-level
/// `TypeExpr` values as if they were resolved semantic types.
///
/// The function builds the complete substitution before checking constraints,
/// allowing a bound to refer to another parameter in the same declaration.
///
/// `Deferred` outcomes are retained as obligations. They do not count as
/// successful proof of the generic constraints.
pub fn bind_type_arguments<S>(
    parameters: &[TypeParameter],
    arguments: &[S::Type],
    where_clause: Option<&WhereClause>,
    system: &mut S,
) -> Result<GenericBinding<S::Type>, GenericError<S::Error>>
where
    S: GenericTypeSystem,
{
    validate_parameters::<S::Error>(parameters)?;

    if parameters.len() != arguments.len() {
        return Err(GenericError::ArityMismatch {
            expected: parameters.len(),
            actual: arguments.len(),
        });
    }

    let mut substitutions = GenericSubstitution::new();

    for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
        // Duplicate parameter names were already rejected above.
        substitutions
            .insert_checked(parameter.name_str().to_owned(), argument.clone())
            .map_err(|error| match error {
                GenericError::DuplicateSubstitution { name } => {
                    GenericError::DuplicateSubstitution { name }
                }
                _ => unreachable!("insert_checked only returns duplicate-key errors"),
            })?;
    }

    let mut pending = Vec::new();
    let mut satisfied_constraints = 0usize;

    // Parameter bounds are checked in declaration order, and each parameter's
    // bounds are checked in their original source order.
    for (parameter_index, parameter) in parameters.iter().enumerate() {
        let actual = arguments
            .get(parameter_index)
            .expect("arity was checked before binding");

        for (bound_index, bound) in parameter.bounds().iter().enumerate() {
            let outcome = system
                .check_bound(actual, bound, &substitutions)
                .map_err(|source| GenericError::Resolution {
                    context: format!(
                        "bound {bound_index} of generic parameter `{}`",
                        parameter.name_str()
                    ),
                    source,
                })?;

            match outcome {
                ConstraintEvaluation::Satisfied => {
                    satisfied_constraints = satisfied_constraints.saturating_add(1);
                }
                ConstraintEvaluation::Unsatisfied => {
                    return Err(GenericError::UnsatisfiedParameterBound {
                        parameter: parameter.name_str().to_owned(),
                        bound: bound.to_string(),
                        bound_index,
                    });
                }
                ConstraintEvaluation::Deferred { reason } => {
                    pending.push(PendingConstraint {
                        actual_type: actual.clone(),
                        bound: bound.clone(),
                        origin: ConstraintOrigin::Parameter {
                            parameter: parameter.name_str().to_owned(),
                            bound_index,
                        },
                        reason,
                    });
                }
            }
        }
    }

    // A where-clause predicate can constrain an expression more complex than
    // a bare parameter. Resolve its subject in the complete substitution.
    if let Some(where_clause) = where_clause {
        for (predicate_index, predicate) in where_clause.predicates().iter().enumerate() {
            if predicate.bounds().is_empty() {
                return Err(GenericError::EmptyWherePredicate {
                    predicate_index,
                });
            }

            check_where_predicate(
                predicate,
                predicate_index,
                &substitutions,
                system,
                &mut pending,
                &mut satisfied_constraints,
            )?;
        }
    }

    Ok(GenericBinding {
        substitutions,
        pending,
        satisfied_constraints,
    })
}

fn check_where_predicate<S>(
    predicate: &WherePredicate,
    predicate_index: usize,
    substitutions: &GenericSubstitution<S::Type>,
    system: &mut S,
    pending: &mut Vec<PendingConstraint<S::Type>>,
    satisfied_constraints: &mut usize,
) -> Result<(), GenericError<S::Error>>
where
    S: GenericTypeSystem,
{
    let actual = system
        .resolve_type_expr(predicate.subject(), substitutions)
        .map_err(|source| GenericError::Resolution {
            context: format!("subject of where-clause predicate {predicate_index}"),
            source,
        })?;

    for (bound_index, bound) in predicate.bounds().iter().enumerate() {
        let outcome = system
            .check_bound(&actual, bound, substitutions)
            .map_err(|source| GenericError::Resolution {
                context: format!(
                    "bound {bound_index} of where-clause predicate {predicate_index}"
                ),
                source,
            })?;

        match outcome {
            ConstraintEvaluation::Satisfied => {
                *satisfied_constraints = satisfied_constraints.saturating_add(1);
            }
            ConstraintEvaluation::Unsatisfied => {
                return Err(GenericError::UnsatisfiedWhereBound {
                    predicate_index,
                    bound: bound.to_string(),
                    bound_index,
                });
            }
            ConstraintEvaluation::Deferred { reason } => {
                pending.push(PendingConstraint {
                    actual_type: actual.clone(),
                    bound: bound.clone(),
                    origin: ConstraintOrigin::WhereClause {
                        predicate_index,
                        bound_index,
                    },
                    reason,
                });
            }
        }
    }

    Ok(())
}

/// Resolve a source-level generic parameter reference using the supplied
/// substitution environment.
///
/// This helper handles a direct `GenericParameter` node only. Nested type
/// substitution belongs to the canonical type substitution service and must
/// not be implemented by string replacement here.
pub fn lookup_parameter<'a, T>(
    name: &TypeParameterName,
    substitutions: &'a GenericSubstitution<T>,
) -> Option<&'a T> {
    substitutions.get(name.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::types::type_expr::TypePath;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MockType(String);

    #[derive(Debug)]
    struct MockError(String);

    impl fmt::Display for MockError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl Error for MockError {}

    struct MockTypeSystem;

    impl GenericTypeSystem for MockTypeSystem {
        type Type = MockType;
        type Error = MockError;

        fn resolve_type_expr(
            &mut self,
            expression: &TypeExpr,
            substitutions: &GenericSubstitution<Self::Type>,
        ) -> Result<Self::Type, Self::Error> {
            match expression {
                TypeExpr::GenericParameter(name) => substitutions
                    .get(name.as_str())
                    .cloned()
                    .ok_or_else(|| {
                        MockError(format!(
                            "unresolved generic parameter `{}`",
                            name.as_str()
                        ))
                    }),
                TypeExpr::Identifier(path) => Ok(MockType(path.to_string())),
                other => Ok(MockType(other.to_string())),
            }
        }

        fn check_bound(
            &mut self,
            actual: &Self::Type,
            bound: &TypeExpr,
            _substitutions: &GenericSubstitution<Self::Type>,
        ) -> Result<ConstraintEvaluation, Self::Error> {
            let bound_name = bound.to_string();

            match (actual.0.as_str(), bound_name.as_str()) {
                ("i32", "Numeric") | ("f64", "Numeric") => {
                    Ok(ConstraintEvaluation::Satisfied)
                }
                (_, "PendingBound") => Ok(ConstraintEvaluation::Deferred {
                    reason: "trait implementation is not yet known".to_owned(),
                }),
                _ => Ok(ConstraintEvaluation::Unsatisfied),
            }
        }
    }

    fn named(name: &str) -> TypeExpr {
        TypeExpr::Identifier(TypePath::single(name))
    }

    #[test]
    fn rejects_duplicate_parameter_names() {
        let parameters = vec![
            TypeParameter::new("T"),
            TypeParameter::new("T"),
        ];

        let result = validate_parameters::<MockError>(&parameters);

        assert!(matches!(
            result,
            Err(GenericError::DuplicateParameter {
                first_index: 0,
                duplicate_index: 1,
                ..
            })
        ));
    }

    #[test]
    fn rejects_wrong_argument_arity() {
        let parameters = vec![TypeParameter::new("T")];
        let arguments: Vec<MockType> = Vec::new();

        let result = bind_type_arguments(
            &parameters,
            &arguments,
            None,
            &mut MockTypeSystem,
        );

        assert!(matches!(
            result,
            Err(GenericError::ArityMismatch {
                expected: 1,
                actual: 0
            })
        ));
    }

    #[test]
    fn binds_arguments_and_checks_bounds() {
        let parameters = vec![TypeParameter::with_bounds(
            "T",
            vec![named("Numeric")],
        )];
        let arguments = vec![MockType("i32".to_owned())];

        let result = bind_type_arguments(
            &parameters,
            &arguments,
            None,
            &mut MockTypeSystem,
        )
        .expect("the mock numeric bound should be satisfied");

        assert!(result.is_complete());
        assert_eq!(
            result.substitutions().get("T"),
            Some(&MockType("i32".to_owned()))
        );
        assert_eq!(result.satisfied_constraint_count(), 1);
    }

    #[test]
    fn deferred_constraints_are_not_reported_as_complete() {
        let parameters = vec![TypeParameter::with_bounds(
            "T",
            vec![named("PendingBound")],
        )];
        let arguments = vec![MockType("SomeType".to_owned())];

        let result = bind_type_arguments(
            &parameters,
            &arguments,
            None,
            &mut MockTypeSystem,
        )
        .expect("a deferred constraint is an obligation, not a hard failure");

        assert!(!result.is_complete());
        assert_eq!(result.pending_constraint_count(), 1);
    }

    #[test]
    fn rejects_proven_unsatisfied_bounds() {
        let parameters = vec![TypeParameter::with_bounds(
            "T",
            vec![named("Numeric")],
        )];
        let arguments = vec![MockType("Text".to_owned())];

        let result = bind_type_arguments(
            &parameters,
            &arguments,
            None,
            &mut MockTypeSystem,
        );

        assert!(matches!(
            result,
            Err(GenericError::UnsatisfiedParameterBound { .. })
        ));
    }

    #[test]
    fn checks_where_clause_using_the_full_substitution() {
        let parameters = vec![TypeParameter::new("T")];
        let arguments = vec![MockType("i32".to_owned())];

        let where_clause = WhereClause::from_predicates(vec![
            WherePredicate::with_bounds(
                TypeExpr::GenericParameter(TypeParameterName::new("T")),
                vec![named("Numeric")],
            ),
        ]);

        let result = bind_type_arguments(
            &parameters,
            &arguments,
            Some(&where_clause),
            &mut MockTypeSystem,
        )
        .expect("T should satisfy Numeric");

        assert!(result.is_complete());
        assert_eq!(result.satisfied_constraint_count(), 1);
    }
}
