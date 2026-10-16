#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Generic Type Integration
//!
//! Path: `src/classical/types/generic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the classical-computing integration boundary for Zamani's
//! canonical generic type system.
//!
//! This module deliberately delegates generic parameter validation,
//! argument binding, substitution storage, constraint evaluation,
//! and deferred-obligation tracking to:
//!
//! `crate::frontend::semantic::types::generics`
//!
//! The classical layer must not define a second generic parameter model,
//! semantic type representation, substitution engine, or constraint solver.
//!
//! # Authority
//!
//! Normative specifications:
//! - `grammar/DESIGN.md`
//! - `grammar/specification/types.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Canonical source-level type representation:
//! - `crate::frontend::ast::node::types::type_expr::TypeExpr`
//!
//! Canonical generic declaration representation:
//! - `crate::frontend::ast::node::generics::parameter::TypeParameter`
//! - `crate::frontend::ast::node::generics::where_clause::WhereClause`
//!
//! Canonical generic semantic implementation:
//! - `crate::frontend::semantic::types::generics`
//!
//! Canonical semantic type representation:
//! - `crate::frontend::semantic::types::ty::SemanticType`
//!
//! Canonical classical IR:
//! - `crate::quantum::ir::classical`
//!
//! # Owns
//!
//! This module owns:
//! - The classical-facing generic integration boundary.
//! - Explicitly named classical entry points for generic validation
//!   and type-argument binding.
//! - Documentation of the integration guarantees and responsibilities.
//!
//! # Does not own
//!
//! This module does not own:
//! - Generic parameter or bound syntax.
//! - Source AST nodes or AST transformations.
//! - A new `GenericParameter`, `GenericArgument`, or `SemanticType` model.
//! - Type-variable allocation, unification, or occurs checking.
//! - Type inference or inference-variable substitution.
//! - Trait/interface implementation lookup.
//! - Associated-type resolution.
//! - Constraint proof algorithms.
//! - Scalar or aggregate type descriptors.
//! - Collection shapes or dependent-type arithmetic.
//! - Ownership, borrowing, or lifetime analysis.
//! - Physical memory layout or ABI representation.
//! - Canonical IR construction, optimization, or execution.
//! - Hardware discovery, resource allocation, or target selection.
//!
//! # Integration contract
//!
//! 1. The parser constructs the canonical `TypeExpr` representation.
//! 2. Structural validation checks source-level type expressions.
//! 3. Name resolution resolves generic constructors and declarations.
//! 4. The canonical semantic type system resolves each argument to its
//!    authoritative semantic type.
//! 5. The `GenericTypeSystem` implementation resolves source expressions
//!    and checks bounds against that semantic type.
//! 6. This module delegates binding and validation to the canonical generic
//!    implementation.
//! 7. Unresolved obligations remain explicit and cannot be treated as proof.
//! 8. Validated semantic types are passed to the appropriate canonical IR
//!    lowering adapter.
//!
//! `types::scalar` and `types::aggregate` provide classical-facing
//! descriptors. They must not be substituted for the canonical semantic
//! type merely because their Rust types are convenient. The semantic
//! resolver is responsible for mapping descriptors and source expressions
//! into the authoritative semantic type model.
//!
//! # Constraint semantics
//!
//! A generic bound may be satisfied, unsatisfied, or deferred.
//!
//! Deferred means that the current semantic information is insufficient.
//! It does not mean that the bound has been proven.
//!
//! The canonical generic implementation owns this distinction. This
//! module must preserve it and must not silently discard pending constraints.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no universal maximum for:
//! - Generic parameters.
//! - Generic arguments.
//! - Type bounds.
//! - `where`-clause predicates.
//! - Types, devices, workers, or computational domains.
//!
//! It does not use fixed arrays or fixed-capacity generic registries.
//! It does not assume a particular CPU, accelerator, register width,
//! memory capacity, or execution topology.
//!
//! The canonical implementation stores substitutions in deterministic
//! ordered storage and reports invalid generic applications explicitly.
//!
//! No finite process can process literally infinite input. Actual execution
//! remains subject to available memory, time, representational limits,
//! and explicit compiler resource policies. Those limits must not become
//! arbitrary language-level restrictions.
//!
//! # Determinism
//!
//! Parameter and bound traversal order is defined by the canonical generic
//! implementation. This façade must not reorder arguments, bounds, or
//! deferred constraints.
//!
//! # Error handling
//!
//! Invalid parameter declarations, argument-arity mismatches, unsatisfied
//! bounds, resolution failures, and deferred constraints are handled by
//! the canonical semantic generic API.
//!
//! This module must not panic, convert unresolved constraints into success,
//! or silently replace invalid types with another type.
//!
//! # Security
//!
//! No unsafe Rust, I/O, global mutable state, or target-specific execution
//! is permitted in this module.
//!
//! Operational resource budgets belong to the compiler/semantic session.
//! They must be explicit and must not be confused with language semantics.
//!
//! # Freeze criteria
//!
//! This module may be marked verified when:
//! - Its imports match the actual public module declarations.
//! - The canonical semantic generic implementation is used directly.
//! - Generic binding preserves all pending obligations.
//! - Classical semantic resolution implements `GenericTypeSystem`.
//! - The frontend-to-classical integration tests pass.
//! - No competing AST, type system, substitution engine, or IR is introduced.
//! - Formatting, compilation, tests, and lint checks pass on the supported
//!   toolchain.
//!
//! Changes to these ownership or delegation rules require an explicit
//! compatibility and dependency review.

use crate::frontend::ast::node::generics::parameter::TypeParameter;
use crate::frontend::ast::node::generics::where_clause::WhereClause;

use crate::frontend::semantic::types::generics::{
    bind_type_arguments, validate_parameters, ConstraintEvaluation,
    ConstraintOrigin, GenericBinding, GenericError, GenericSubstitution,
    GenericTypeSystem, PendingConstraint, GENERICS_SEMANTIC_SCHEMA_VERSION,
};

/// Validates generic parameter declarations used by classical computation.
///
/// This delegates to the canonical semantic generic implementation.
/// It checks declaration-local naming invariants; it does not resolve
/// bounds, infer types, or establish that a bound is satisfied.
///
/// # Errors
///
/// Returns the canonical `GenericError<E>` when a parameter name is empty
/// or duplicated.
///
/// Structural validation of each bound remains the responsibility of
/// the frontend's canonical AST validation layer.
pub fn validate_classical_parameters<E>(
    parameters: &[TypeParameter],
) -> Result<(), GenericError<E>> {
    validate_parameters::<E>(parameters)
}

/// Binds resolved generic arguments for a classical semantic operation.
///
/// `S::Type` must be the canonical semantic type used by the compiler.
/// The supplied arguments must already have been resolved; source-level
/// `TypeExpr` values must not be passed as if they were resolved types.
///
/// Parameter bounds and an optional `where` clause are checked by the
/// canonical semantic generic implementation. Deferred constraints are
/// preserved in the returned binding.
///
/// # Arguments
///
/// - `parameters`: canonical generic parameter declarations in source order.
/// - `arguments`: resolved semantic type arguments in source order.
/// - `where_clause`: optional canonical source-level `where` clause.
/// - `system`: the semantic adapter responsible for resolving types and
///   checking bounds against the canonical type and constraint systems.
///
/// # Errors
///
/// Returns a canonical generic error for invalid declarations, arity
/// mismatches, unsatisfied constraints, or semantic resolution failures.
///
/// A deferred constraint is retained in the successful binding; callers
/// must not certify the instantiation until required obligations are proven.
pub fn bind_classical_type_arguments<S>(
    parameters: &[TypeParameter],
    arguments: &[S::Type],
    where_clause: Option<&WhereClause>,
    system: &mut S,
) -> Result<GenericBinding<S::Type>, GenericError<S::Error>>
where
    S: GenericTypeSystem,
{
    bind_type_arguments(parameters, arguments, where_clause, system)
}

/// Canonical generic constraint outcome.
///
/// Re-exported here for classical clients so they can inspect bound
/// evaluation without defining a competing constraint-state enum.
pub use ConstraintEvaluation as ClassicalConstraintEvaluation;

/// Canonical generic binding result.
///
/// This is an alias, not a new type or representation.
pub type ClassicalGenericBinding<T> = GenericBinding<T>;

/// Canonical generic substitution environment.
///
/// This is an alias, not a second substitution map.
pub type ClassicalGenericSubstitution<T> = GenericSubstitution<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_substitution_environment_is_available() {
        let substitutions = ClassicalGenericSubstitution::<u8>::new();

        assert!(substitutions.is_empty());
        assert_eq!(substitutions.len(), 0);
        assert!(!substitutions.contains("T"));
        assert!(substitutions.get("T").is_none());
    }

    #[test]
    fn canonical_substitution_iteration_is_empty_and_deterministic() {
        let substitutions = ClassicalGenericSubstitution::<u8>::new();

        assert_eq!(substitutions.iter().count(), 0);
        assert!(substitutions.into_map().is_empty());
    }

    #[test]
    fn canonical_schema_version_is_exposed() {
        // This is the semantic generic API schema version, not the
        // Zamani language version or the version of the classical IR.
        assert_eq!(GENERICS_SEMANTIC_SCHEMA_VERSION, 1);
    }

    #[test]
    fn canonical_constraint_outcomes_remain_distinct() {
        let satisfied = ClassicalConstraintEvaluation::Satisfied;
        let unsatisfied = ClassicalConstraintEvaluation::Unsatisfied;
        let deferred = ClassicalConstraintEvaluation::Deferred {
            reason: String::from("type information is incomplete"),
        };

        assert_ne!(satisfied, unsatisfied);
        assert_ne!(satisfied, deferred);
        assert_ne!(unsatisfied, deferred);
    }

    #[test]
    fn canonical_pending_constraint_origin_is_preserved() {
        let origin = ConstraintOrigin::Parameter {
            parameter: String::from("T"),
            bound_index: 0,
        };

        assert_eq!(
            origin,
            ConstraintOrigin::Parameter {
                parameter: String::from("T"),
                bound_index: 0,
            }
        );
    }

    #[test]
    fn canonical_pending_constraint_type_is_publicly_available() {
        // The pending obligation itself is constructed by the canonical
        // generic binder, not by this façade. This compile-time use verifies
        // that the canonical obligation type remains part of the API.
        fn accepts_pending<T>(_constraint: &PendingConstraint<T>) {}

        let _ = accepts_pending::<u8>;
    }
}