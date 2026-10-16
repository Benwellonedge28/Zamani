
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani — Classical Symbolic Values
//!
//! File: src/classical/values/symbolic.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide the classical-computing API for symbolic expressions while
//! preserving the canonical expression semantics already owned by
//! `crate::quantum::ir::classical::expression`.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - The classical-facing `SymbolicExpression` type alias.
//! - Symbol construction through the canonical expression constructor.
//! - Explicit policy-based structural validation.
//! - Deterministic discovery of referenced symbolic names.
//!
//! The canonical expression module owns:
//! - `ClassicalExpression` and its expression variants.
//! - `ExpressionError` and `ExpressionResult`.
//! - `ExpressionValidationPolicy`.
//! - Expression structure, operators, calls, references, and evaluation
//!   semantics.
//!
//! This module MUST NOT introduce a competing expression enum, AST, IR,
//! symbolic parameter representation, or evaluator.
//!
//! # Grammar integration
//!
//! `grammar/classical/symbolic.g4` defines a symbolic-domain composition
//! boundary. Canonical expression syntax, identifiers, operators, calls,
//! and precedence remain owned by the existing Zamani grammar.
//!
//! Source parsing and literal interpretation belong to the frontend.
//! Name resolution, types, effects, and mathematical meaning belong to
//! semantic analysis.
//!
//! # POCO-REAF and scalability
//!
//! This adapter introduces no hardware-specific assumptions and no
//! universal maximum for symbols, expression nodes, dimensions, or
//! mathematical operations.
//!
//! Optional validation limits are supplied by the caller through the
//! canonical `ExpressionValidationPolicy`. An absent limit means that this
//! adapter imposes no corresponding local policy limit; it does not promise
//! infinite memory, execution time, or computational resources.
//!
//! # Integration
//!
//! 1. `values/mod.rs` declares this module and exports its public API.
//! 2. `classical/mod.rs` exposes the values module through the established
//!    classical module hierarchy.
//! 3. The canonical expression implementation remains the sole owner of
//!    expression semantics.
//! 4. The frontend lowers parsed expressions to the canonical expression
//!    model and uses this adapter where the classical-facing API is needed.
//! 5. Semantic analysis resolves symbols and validates types, effects,
//!    capabilities, and resource requirements.
//! 6. Canonical IR lowering consumes the existing expression representation.
//! 7. Optimizers must preserve the canonical expression's observable
//!    semantics.
//!
//! No downstream consumer needs a second symbolic expression type.
//!
//! # Freeze criteria
//!
//! This file is complete against its adapter contract when its canonical
//! imports compile, module exports are registered, all tests pass, and
//! existing expression consumers remain compatible.
//!
//! Repository-wide integration is not considered verified until the
//! canonical expression module and parent exports compile together.

use std::collections::BTreeSet;

pub use crate::quantum::ir::classical::expression::{
    ClassicalExpression,
    ExpressionError,
    ExpressionResult,
    ExpressionValidationPolicy,
};

/// Classical-facing name for Zamani's canonical symbolic expression.
///
/// This is a type alias, not a second expression representation.
/// All expression variants and semantic behavior are owned by
/// `ClassicalExpression`.
pub type SymbolicExpression = ClassicalExpression;

/// Constructs a symbolic reference using the canonical expression rules.
///
/// The supplied name is passed to the canonical constructor unchanged.
/// Empty names and any other conditions rejected by that constructor
/// return its canonical error.
///
/// Name lookup and declaration resolution are deliberately deferred to
/// semantic analysis.
pub fn symbol<S: Into<String>>(name: S) -> ExpressionResult<SymbolicExpression> {
    ClassicalExpression::symbol(name)
}

/// Constructs a symbolic reference without imposing the default symbol
/// byte-length limit of the canonical constructor.
///
/// This does not bypass the canonical constructor's empty-name check.
/// Callers should subsequently validate the containing expression using
/// the policy appropriate to their compilation or execution context.
///
/// This function does not imply that the resulting expression is bound,
/// typed, evaluable, or supported by a particular execution target.
pub fn symbol_with_policy<S: Into<String>>(
    name: S,
) -> ExpressionResult<SymbolicExpression> {
    ClassicalExpression::symbol_unchecked_by_policy(name)
}

/// Validates an expression using the canonical expression validator.
///
/// Structural validation does not resolve names or prove that a symbolic
/// operation is mathematically valid. Those responsibilities belong to
/// semantic analysis and the relevant mathematical subsystem.
pub fn validate(
    expression: &SymbolicExpression,
    policy: ExpressionValidationPolicy,
) -> ExpressionResult<()> {
    expression.validate_with_policy(policy)
}

/// Returns the unique symbolic names referenced by an expression.
///
/// The returned names are ordered lexicographically by the canonical
/// expression implementation. They are not resolved declarations.
///
/// The caller-supplied policy controls any configured collection limit.
/// The canonical implementation owns traversal and limit enforcement.
pub fn collect_symbols(
    expression: &SymbolicExpression,
    policy: ExpressionValidationPolicy,
) -> ExpressionResult<Vec<String>> {
    expression.collect_symbols_with_policy(policy)
}

/// Collects unique symbolic names into an existing ordered set.
///
/// This helper allows callers to combine symbols from multiple expressions
/// without introducing a randomized collection or another symbol registry.
///
/// The set is caller-owned; this function does not clear existing entries.
pub fn collect_symbols_into(
    expression: &SymbolicExpression,
    symbols: &mut BTreeSet<String>,
) {
    expression.collect_symbols_into(symbols);
}

#[cfg(test)]
mod tests {
    use super::{
        collect_symbols,
        collect_symbols_into,
        symbol,
        symbol_with_policy,
        validate,
        ExpressionError,
        ExpressionValidationPolicy,
        SymbolicExpression,
    };
    use std::collections::BTreeSet;

    #[test]
    fn symbolic_alias_uses_canonical_expression_representation() {
        let expression: SymbolicExpression =
            symbol("amplitude").expect("valid symbol");

        assert_eq!(
            expression,
            SymbolicExpression::symbol("amplitude")
                .expect("same canonical symbol")
        );
    }

    #[test]
    fn valid_symbol_is_preserved() {
        let expression = symbol("state_amplitude")
            .expect("non-empty symbol");

        assert_eq!(
            collect_symbols(&expression, ExpressionValidationPolicy::default())
                .expect("symbol collection"),
            vec!["state_amplitude".to_owned()]
        );
    }

    #[test]
    fn empty_symbol_is_rejected_by_canonical_constructor() {
        assert!(matches!(
            symbol(""),
            Err(ExpressionError::EmptySymbol)
        ));
    }

    #[test]
    fn policy_aware_constructor_still_rejects_empty_symbol() {
        assert!(matches!(
            symbol_with_policy(""),
            Err(ExpressionError::EmptySymbol)
        ));
    }

    #[test]
    fn explicit_policy_validates_expression() {
        let expression = symbol("x").expect("valid symbol");

        validate(
            &expression,
            ExpressionValidationPolicy::default(),
        )
        .expect("expression satisfies default policy");
    }

    #[test]
    fn symbol_collection_is_deterministic_and_unique() {
        let expression = SymbolicExpression::binary(
            crate::quantum::ir::classical::expression::BinaryOperator::Add,
            symbol("z").expect("valid symbol"),
            symbol("a").expect("valid symbol"),
            crate::quantum::ir::classical::expression::ExpressionType::Unknown,
        );

        let names = collect_symbols(
            &expression,
            ExpressionValidationPolicy::default(),
        )
        .expect("symbol collection");

        assert_eq!(names, vec!["a".to_owned(), "z".to_owned()]);
    }

    #[test]
    fn symbols_can_be_combined_into_a_caller_owned_set() {
        let first = symbol("beta").expect("valid symbol");
        let second = symbol("alpha").expect("valid symbol");
        let repeated = symbol("beta").expect("valid symbol");

        let mut names = BTreeSet::new();

        collect_symbols_into(&first, &mut names);
        collect_symbols_into(&second, &mut names);
        collect_symbols_into(&repeated, &mut names);

        assert_eq!(
            names.into_iter().collect::<Vec<_>>(),
            vec!["alpha".to_owned(), "beta".to_owned()]
        );
    }

    #[test]
    fn caller_can_configure_symbol_collection_limits() {
        let expression = SymbolicExpression::binary(
            crate::quantum::ir::classical::expression::BinaryOperator::Add,
            symbol("left").expect("valid symbol"),
            symbol("right").expect("valid symbol"),
            crate::quantum::ir::classical::expression::ExpressionType::Unknown,
        );

        let mut policy = ExpressionValidationPolicy::default();
        policy.max_collected_symbols = Some(1);

        assert!(collect_symbols(&expression, policy).is_err());
    }
}
