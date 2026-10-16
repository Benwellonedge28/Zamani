#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Type Constraint Integration
//!
//! Path: `src/classical/types/constraints.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose the canonical Zamani semantic constraint system to classical
//! computation without creating a competing constraint representation,
//! registry, solver, or proof engine.
//!
//! # Architectural authority
//!
//! Normative specifications:
//! - `grammar/DESIGN.md`
//! - `grammar/specification/types.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/types/bounds.g4`
//! - `grammar/types/generic.g4`
//!
//! Canonical implementations:
//! - `crate::frontend::semantic::types::constraints`
//! - `crate::frontend::semantic::types::generics`
//! - `crate::frontend::semantic::types::unification`
//! - `crate::frontend::semantic::types::compatibility`
//! - `crate::frontend::semantic::types::traits`
//! - `crate::frontend::semantic::types::ty`
//!
//! Canonical source type syntax:
//! - `crate::frontend::ast::node::types::type_expr::TypeExpr`
//!
//! # Ownership
//!
//! This module owns:
//! - The classical-facing public integration surface for type constraints.
//! - Classical-facing aliases for canonical constraint types.
//! - A stable place for classical validation and lowering modules to import
//!   the shared semantic constraint API.
//!
//! This module does not own:
//! - Constraint identity allocation algorithms.
//! - Constraint storage or lifecycle implementation.
//! - Type equality, unification, or assignability rules.
//! - Generic parameter binding or substitution.
//! - Trait and interface implementation lookup.
//! - Constant evaluation or symbolic theorem proving.
//! - Lifetime, borrowing, ownership, or effect-checking algorithms.
//! - Capability negotiation, resource allocation, or target selection.
//! - Classical, quantum, HDL, or other domain-specific IR construction.
//!
//! # Semantic contract
//!
//! A constraint is an obligation, not proof of its own truth.
//!
//! The canonical system distinguishes:
//! - `Proven`: the authoritative semantic service established the obligation.
//! - `Disproven`: the authoritative semantic service established a conflict.
//! - `Unresolved`: the obligation has not been established either way.
//!
//! Unresolved constraints MUST NOT be treated as proven.
//! Error-recovery types MUST NOT establish successful constraint proofs.
//! Unknown extension obligations MUST remain unresolved unless a registered
//! and authorized semantic extension evaluates them.
//!
//! # Integration contract
//!
//! - Frontend AST validation checks the structure of source-level expressions.
//! - Name resolution resolves type and trait identities.
//! - Generic binding delegates parameter and `where`-clause checks to the
//!   canonical generic semantic implementation.
//! - Unification owns type-equality solving and inference substitutions.
//! - Compatibility owns language-defined compatibility and assignability.
//! - Trait resolution owns trait implementation lookup and coherence.
//! - Constant evaluation owns evaluation of type-level expressions.
//! - This module exposes the canonical obligation representation to classical
//!   validators and other semantic consumers.
//! - Classical lowering consumes validated semantic types; it must not use
//!   this module as a substitute for semantic validation.
//!
//! Domain-specific validators may add obligations using the canonical
//! extension mechanism. They must not create parallel registries for
//! classical, quantum, hybrid, distributed, or hardware constraints.
//!
//! # Resource and scalability contract
//!
//! This module introduces no fixed maximum for:
//! - Generic parameters or type arguments.
//! - Number of constraints or trait obligations.
//! - Hardware, accelerator, device, or node populations.
//! - Array lengths, tensor ranks, or symbolic dimensions.
//! - Supported computational domains.
//!
//! The canonical `ConstraintSet` supports an explicit optional operation
//! budget. Such a budget is operational policy, not a language-level
//! restriction. Exhaustion must be surfaced as an error or incomplete
//! analysis, never as successful proof.
//!
//! No finite process can handle literally infinite input. Actual work is
//! bounded by available resources, representational limits, and explicit
//! compiler policies. This module imposes no additional universal capacity
//! ceiling.
//!
//! # Determinism
//!
//! Constraint identity, ordering, storage, and lifecycle behavior are owned
//! by the canonical implementation. This façade does not reorder constraints
//! or introduce hash-map-dependent semantic outcomes.
//!
//! # Compatibility
//!
//! The aliases below are aliases of canonical types, not new representations.
//! Public APIs must not rely on the aliases having a distinct Rust type
//! identity.
//!
//! # Freeze criteria
//!
//! This file may be marked verified when:
//! - `src/classical/types/mod.rs` declares and exports this module.
//! - The canonical semantic constraint module is public and reachable.
//! - Classical validators use these canonical types.
//! - Generic, unification, trait, and compatibility integration tests pass.
//! - Unresolved obligations remain explicit through validation and lowering.
//! - No duplicate constraint solver or semantic type model is introduced.
//! - Formatting, compilation, tests, and lint checks pass.
//!
//! # Freeze rule
//!
//! A change to canonical ownership, constraint semantics, or the public
//! aliases requires an explicit compatibility and dependency review.

/// Canonical constraint identity and checked identity allocator.
pub use crate::frontend::semantic::types::constraints::{
    ConstraintId,
    ConstraintIdAllocator,
};

/// Canonical constraint record, collection, configuration, and errors.
pub use crate::frontend::semantic::types::constraints::{
    Constraint,
    ConstraintConfig,
    ConstraintError,
    ConstraintSet,
};

/// Canonical proof status and unresolved-reason classifications.
pub use crate::frontend::semantic::types::constraints::{
    ConstraintStatus,
    UnresolvedReason,
};

/// Canonical target-independent constraint representation and relations.
pub use crate::frontend::semantic::types::constraints::{
    TypeConstraint,
    TypeLevelOperand,
    TypeRelation,
    ValueRelation,
};

/// Canonical generic bound evaluation result.
///
/// `Deferred` is not equivalent to `Satisfied`.
pub use crate::frontend::semantic::types::generics::ConstraintEvaluation
    as GenericConstraintEvaluation;

/// Canonical generic constraint origin and deferred-obligation representation.
pub use crate::frontend::semantic::types::generics::{
    ConstraintOrigin,
    PendingConstraint,
};

/// Classical-facing alias for the canonical semantic constraint collection.
///
/// This is not a wrapper or a separate registry. All operations, identity
/// rules, status transitions, budgets, and errors are those of `ConstraintSet`.
pub type ClassicalConstraintSet = ConstraintSet;

/// Classical-facing alias for the canonical semantic constraint.
pub type ClassicalTypeConstraint = TypeConstraint;

/// Classical-facing alias for the canonical constraint lifecycle status.
pub type ClassicalConstraintStatus = ConstraintStatus;

/// Validates the structural integrity of a classical type constraint.
///
/// This delegates to the canonical semantic type system. Structural validity
/// does not prove that the constraint is true.
///
/// # Errors
///
/// Returns the canonical `ConstraintError` if the operands are malformed,
/// incomplete, or unsuitable for establishing a constraint.
pub fn validate_classical_constraint(
    constraint: &ClassicalTypeConstraint,
) -> Result<(), ConstraintError> {
    constraint.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_ids_are_monotonic() {
        let mut allocator = ConstraintIdAllocator::new();

        let first = allocator
            .allocate()
            .expect("the first constraint identity should be available");
        let second = allocator
            .allocate()
            .expect("the second constraint identity should be available");

        assert!(first.get() < second.get());
        assert_eq!(first.get(), 0);
        assert_eq!(second.get(), 1);
    }

    #[test]
    fn constraint_ids_are_unique_within_an_allocator() {
        let mut allocator = ConstraintIdAllocator::new();
        let first = allocator.allocate().expect("identity allocation");
        let second = allocator.allocate().expect("identity allocation");

        assert_ne!(first, second);
    }

    #[test]
    fn constraint_collection_starts_empty() {
        let constraints = ClassicalConstraintSet::new();

        assert!(constraints.is_empty());
        assert_eq!(constraints.len(), 0);
        assert!(constraints.all_proven());
        assert_eq!(constraints.unresolved_count(), 0);
        assert_eq!(constraints.disproven_count(), 0);
    }

    #[test]
    fn explicit_constraint_budget_is_retained() {
        let budget = ConstraintConfig {
            max_operations: Some(7),
        };

        let constraints = ClassicalConstraintSet::with_config(budget)
            .expect("a positive operational budget should be accepted");

        assert_eq!(constraints.config(), budget);
        assert_eq!(constraints.operations(), 0);
    }

    #[test]
    fn zero_constraint_budget_is_rejected() {
        let result = ClassicalConstraintSet::with_config(ConstraintConfig {
            max_operations: Some(0),
        });

        assert_eq!(result, Err(ConstraintError::InvalidConfiguration));
    }

    #[test]
    fn no_additional_constraint_budget_is_supported() {
        let constraints = ClassicalConstraintSet::with_config(ConstraintConfig {
            max_operations: None,
        })
        .expect("an absent additional budget is valid");

        assert_eq!(constraints.config().max_operations, None);
    }

    #[test]
    fn generic_constraint_outcomes_remain_distinct() {
        let proven = GenericConstraintEvaluation::Satisfied;
        let disproven = GenericConstraintEvaluation::Unsatisfied;
        let unresolved = GenericConstraintEvaluation::Deferred {
            reason: String::from("requires more type information"),
        };

        assert_ne!(proven, disproven);
        assert_ne!(proven, unresolved);
        assert_ne!(disproven, unresolved);
    }

    #[test]
    fn unresolved_reasons_are_explicit() {
        let inference = UnresolvedReason::TypeInference;
        let symbolic = UnresolvedReason::SymbolicValue;
        let deferred = UnresolvedReason::DeferredToLaterPass;

        assert_ne!(inference, symbolic);
        assert_ne!(inference, deferred);
        assert_ne!(symbolic, deferred);
    }

    #[test]
    fn classical_aliases_preserve_canonical_type_identity() {
        fn accepts_canonical_set(_: &ConstraintSet) {}
        fn accepts_classical_set(_: &ClassicalConstraintSet) {}

        let constraints = ConstraintSet::new();

        accepts_canonical_set(&constraints);
        accepts_classical_set(&constraints);
    }

    #[test]
    fn canonical_constraint_status_is_not_a_boolean() {
        let proven = ClassicalConstraintStatus::Proven;
        let disproven = ClassicalConstraintStatus::Disproven {
            reason: String::from("the required relation does not hold"),
        };
        let unresolved = ClassicalConstraintStatus::Unresolved {
            reason: UnresolvedReason::MissingResolver,
        };

        assert_ne!(proven, disproven);
        assert_ne!(proven, unresolved);
        assert_ne!(disproven, unresolved);
    }
}