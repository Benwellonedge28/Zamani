
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Constant Folding
//!
//! Path: `src/classical/expressions/folding.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!   Contract-oriented implementation. Mark as verified only after integrating
//!   a concrete semantic adapter and running the repository's build and tests.
//!
//! PURPOSE
//!   Define a target-independent, type-independent contract for folding
//!   canonical frontend expressions into semantic constant values.
//!
//! OWNERSHIP
//!   The canonical frontend AST owns Expression, ExpressionKind, Literal,
//!   OperatorRef, and NodeId. This module defines folding outcomes and the
//!   semantic adapter interface. It does not duplicate AST or IR types.
//!
//! DOES NOT OWN
//!   Parsing, literal decoding, name resolution, type inference, operator
//!   definitions, arbitrary-precision numeric representations, runtime
//!   execution, optimization legality, or IR construction.
//!
//! SAFETY AND SCALABILITY
//!   This module uses safe Rust only. It imposes no language-level ceiling on
//!   expression count, integer width, tensor rank, hardware count, or program
//!   size. Actual memory and compilation budgets belong to the compiler's
//!   explicit resource policy.
//!
//! IMPORTANT
//!   A fold is valid only when the semantic adapter proves that the expression
//!   is eligible and computes the result according to Zamani's specified
//!   semantics. This module never guesses operator meaning from spelling.

use std::collections::BTreeMap;
use std::fmt;

use crate::frontend::ast::node::expressions::expression::{
    Expression,
    ExpressionKind,
    OperatorRef,
};
use crate::frontend::ast::node::expressions::literal::Literal;
use crate::frontend::ast::node::node_id::NodeId;

/// A value produced by the canonical semantic constant evaluator.
///
/// `V` is intentionally generic. The concrete semantic subsystem chooses the
/// value representation, including integer precision, floating-point policy,
/// decimal/rational values, strings, aggregates, and domain-specific values.
pub type FoldResult<V, E> = Result<FoldOutcome<V>, E>;

/// Outcome of attempting to fold one expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FoldOutcome<V> {
    /// The expression was evaluated successfully at compile time.
    Constant(V),

    /// The expression is valid to analyze but is not currently foldable.
    ///
    /// This is not an error. For example, an identifier may refer to a
    /// runtime value or an operator may not permit compile-time evaluation.
    NotConstant(NotConstantReason),

    /// The expression is deliberately left for a later compiler stage.
    ///
    /// This differs from `NotConstant`: the adapter has determined that
    /// another stage owns the decision or transformation.
    Deferred(DeferredReason),
}

/// Reasons an expression was not folded.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NotConstantReason {
    /// The expression depends on a runtime value.
    RuntimeDependency,

    /// The operator does not provide a permitted constant interpretation.
    OperatorNotConstEvaluable,

    /// The expression may produce effects or observable state changes.
    ObservableEffects,

    /// The expression depends on an unavailable compile-time capability.
    CapabilityUnavailable,

    /// The expression's type or overload has not yet been resolved.
    SemanticInformationUnavailable,

    /// The construct has no registered constant-folding implementation.
    UnsupportedConstruct,

    /// The operation is not permitted in a compile-time evaluation context.
    PolicyDenied,
}

/// Reasons folding was deferred.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeferredReason {
    /// Type inference or overload resolution must run first.
    RequiresTypeResolution,

    /// A dependent constant has not yet been evaluated.
    RequiresDependency,

    /// A domain extension owns the transformation.
    ExtensionOwned,

    /// Another compiler phase owns this optimization.
    LaterCompilerPhase,
}

/// A stable identity for a folding diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoldSite {
    /// Canonical frontend AST node identity.
    pub node: NodeId,
}

/// Structured failure from constant folding.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FoldError {
    /// The adapter attempted to fold a node that is not an expression.
    InvalidExpressionSite(FoldSite),

    /// The adapter supplied a result for a different node than requested.
    ResultIdentityMismatch {
        /// Node requested by the caller.
        expected: FoldSite,

        /// Node reported by the adapter.
        actual: FoldSite,
    },

    /// The adapter or caller attempted to overwrite an existing cached result.
    DuplicateResult(FoldSite),

    /// A dependency required by the folding operation was not available.
    MissingDependency {
        /// Expression whose evaluation requires the dependency.
        expression: FoldSite,

        /// Missing dependency identity.
        dependency: NodeId,
    },

    /// A dependency cycle prevents evaluation in the current evaluation model.
    DependencyCycle {
        /// Node at which the cycle was detected.
        node: NodeId,
    },

    /// The configured work budget was exhausted.
    ///
    /// Budgets are explicit operational controls, not restrictions on which
    /// programs are valid Zamani programs.
    WorkBudgetExceeded {
        /// Work performed so far.
        consumed: u64,

        /// Configured budget.
        budget: u64,
    },
}

impl fmt::Display for FoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExpressionSite(site) => {
                write!(f, "AST node {:?} is not an expression", site.node)
            }
            Self::ResultIdentityMismatch { expected, actual } => {
                write!(
                    f,
                    "fold result identity mismatch: expected {:?}, got {:?}",
                    expected.node, actual.node
                )
            }
            Self::DuplicateResult(site) => {
                write!(f, "fold result already exists for {:?}", site.node)
            }
            Self::MissingDependency {
                expression,
                dependency,
            } => {
                write!(
                    f,
                    "expression {:?} requires unavailable dependency {:?}",
                    expression.node, dependency
                )
            }
            Self::DependencyCycle { node } => {
                write!(f, "constant dependency cycle at {:?}", node)
            }
            Self::WorkBudgetExceeded { consumed, budget } => {
                write!(
                    f,
                    "constant-folding work budget exceeded: consumed {consumed}, \
                     budget {budget}"
                )
            }
        }
    }
}

impl std::error::Error for FoldError {}

/// Optional operational budget for a folding operation.
///
/// `None` means the caller has not configured a work-count budget. It does
/// not promise unlimited memory, time, or operating-system resources.
///
/// A work-count budget is deliberately not embedded into expression
/// semantics. The compiler or build policy chooses it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FoldBudget {
    /// Maximum accounted work units, if configured.
    pub max_work_units: Option<u64>,
}

impl FoldBudget {
    /// Construct an explicitly budgeted operation.
    #[must_use]
    pub const fn with_max_work_units(maximum: u64) -> Self {
        Self {
            max_work_units: Some(maximum),
        }
    }

    /// Construct an operation without a module-level work-count budget.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_work_units: None,
        }
    }

    /// Check whether another work unit may be consumed.
    pub fn consume(
        self,
        consumed: &mut u64,
    ) -> Result<(), FoldError> {
        if let Some(budget) = self.max_work_units {
            if *consumed >= budget {
                return Err(FoldError::WorkBudgetExceeded {
                    consumed: *consumed,
                    budget,
                });
            }
        }

        *consumed = consumed
            .checked_add(1)
            .ok_or(FoldError::WorkBudgetExceeded {
                consumed: u64::MAX,
                budget: self.max_work_units.unwrap_or(u64::MAX),
            })?;

        Ok(())
    }
}

/// Read-only view of already-computed fold results.
///
/// The map is keyed by canonical AST identity. It does not own AST nodes or
/// assign identities. The concrete value type is supplied by the semantic
/// layer.
pub type FoldValueTable<V> = BTreeMap<NodeId, FoldOutcome<V>>;

/// The semantic contract required by the folding engine.
///
/// Implement this trait in the semantic/value integration layer. Implementors
/// must use canonical name resolution, type resolution, operator semantics,
/// effect rules, and compile-time policy.
///
/// The adapter must not execute arbitrary host code, perform implicit I/O,
/// inspect target hardware to change source-language meaning, or silently
/// truncate a result.
///
/// `V` should have value semantics. `Clone` is required because one constant
/// may be referenced by multiple expressions in the semantic graph.
pub trait ConstantFoldSemantics {
    /// Semantic constant value type.
    type Value: Clone + PartialEq + Eq;

    /// Error returned by the concrete evaluator.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Decode and evaluate a literal using the canonical value semantics.
    ///
    /// Preserve source spelling in the AST. Decoding, radix handling,
    /// precision, escapes, and range checks happen in this adapter.
    fn fold_literal(
        &self,
        site: FoldSite,
        literal: &Literal,
    ) -> FoldResult<Self::Value, Self::Error>;

    /// Resolve an identifier expression to a compile-time value, if available.
    ///
    /// The identifier's referenced AST node is canonical frontend data.
    /// This method must use the existing symbol/name-resolution service.
    fn fold_identifier(
        &self,
        site: FoldSite,
        identifier: NodeId,
        known_values: &FoldValueTable<Self::Value>,
    ) -> FoldResult<Self::Value, Self::Error>;

    /// Fold a unary expression after its operand is known to be constant.
    fn fold_unary(
        &self,
        site: FoldSite,
        operator: &OperatorRef,
        operand: &Self::Value,
    ) -> FoldResult<Self::Value, Self::Error>;

    /// Fold a binary expression after both operands are known to be constant.
    ///
    /// This method is for eager binary operations only. Short-circuit operators
    /// and conditional evaluation must be handled by the expression scheduler
    /// so that an unselected operand is not evaluated.
    fn fold_binary(
        &self,
        site: FoldSite,
        operator: &OperatorRef,
        left: &Self::Value,
        right: &Self::Value,
    ) -> FoldResult<Self::Value, Self::Error>;

    /// Fold another expression kind when its required child values are known.
    ///
    /// `children` contains only dependencies selected by the scheduler as
    /// value dependencies. Type nodes, patterns, attributes, and other
    /// non-value children must not be treated as constant values.
    ///
    /// The default is conservative: unsupported expressions remain unfurled.
    fn fold_other(
        &self,
        _site: FoldSite,
        _expression: &Expression,
        _children: &[(NodeId, Self::Value)],
    ) -> FoldResult<Self::Value, Self::Error> {
        Ok(FoldOutcome::NotConstant(
            NotConstantReason::UnsupportedConstruct,
        ))
    }
}

/// Result of folding one canonical expression through its semantic adapter.
///
/// This entry point deliberately does not perform recursive graph traversal:
/// graph traversal, lazy dependency scheduling, cycle detection, and
/// dependency resolution belong to the compiler's existing semantic graph
/// service. That avoids a second AST store and avoids accidentally evaluating
/// both branches of conditional or short-circuit expressions.
pub fn fold_one<S>(
    semantics: &S,
    site: FoldSite,
    expression: &Expression,
    known_values: &FoldValueTable<S::Value>,
    children: &[(NodeId, S::Value)],
) -> FoldResult<S::Value, S::Error>
where
    S: ConstantFoldSemantics,
{
    if expression.id() != site.node {
        return Err(S::Error::from(FoldAdapterError::IdentityMismatch {
            expected: site.node,
            actual: expression.id(),
        }));
    }

    match expression.kind() {
        ExpressionKind::Identifier { identifier } => {
            semantics.fold_identifier(site, *identifier, known_values)
        }

        ExpressionKind::Literal { .. } => {
            // Literal nodes are resolved and passed directly to fold_literal
            // by the canonical AST graph integration. An ExpressionKind only
            // contains the literal's NodeId, not the Literal itself.
            //
            // Therefore this generic entry point cannot decode a literal
            // without the canonical node resolver. It reports deferral rather
            // than guessing a representation or looking up an alternate AST.
            Ok(FoldOutcome::Deferred(
                DeferredReason::RequiresDependency,
            ))
        }

        ExpressionKind::Unary { operator, operand } => {
            let Some(value) = known_values.get(operand) else {
                return Ok(FoldOutcome::Deferred(
                    DeferredReason::RequiresDependency,
                ));
            };

            match value {
                FoldOutcome::Constant(value) => {
                    semantics.fold_unary(site, operator, value)
                }
                FoldOutcome::NotConstant(reason) => {
                    Ok(FoldOutcome::NotConstant(reason.clone()))
                }
                FoldOutcome::Deferred(reason) => {
                    Ok(FoldOutcome::Deferred(reason.clone()))
                }
            }
        }

        ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            let Some(left_result) = known_values.get(left) else {
                return Ok(FoldOutcome::Deferred(
                    DeferredReason::RequiresDependency,
                ));
            };

            let Some(right_result) = known_values.get(right) else {
                return Ok(FoldOutcome::Deferred(
                    DeferredReason::RequiresDependency,
                ));
            };

            let left_value = match left_result {
                FoldOutcome::Constant(value) => value,
                FoldOutcome::NotConstant(reason) => {
                    return Ok(FoldOutcome::NotConstant(reason.clone()));
                }
                FoldOutcome::Deferred(reason) => {
                    return Ok(FoldOutcome::Deferred(reason.clone()));
                }
            };

            let right_value = match right_result {
                FoldOutcome::Constant(value) => value,
                FoldOutcome::NotConstant(reason) => {
                    return Ok(FoldOutcome::NotConstant(reason.clone()));
                }
                FoldOutcome::Deferred(reason) => {
                    return Ok(FoldOutcome::Deferred(reason.clone()));
                }
            };

            semantics.fold_binary(site, operator, left_value, right_value)
        }

        _ => semantics.fold_other(site, expression, children),
    }
}

/// Error used when the folding adapter is given an inconsistent AST identity.
///
/// This is an adapter-contract failure, distinct from a language expression
/// that simply cannot be folded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldAdapterError {
    /// Expected canonical identity.
    pub expected: NodeId,

    /// Actual identity reported by the expression.
    pub actual: NodeId,
}

impl fmt::Display for FoldAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fold adapter expected expression {:?}, received {:?}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for FoldAdapterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrestricted_budget_accounts_for_work() {
        let mut consumed = 0;

        assert!(FoldBudget::unrestricted().consume(&mut consumed).is_ok());
        assert_eq!(consumed, 1);
    }

    #[test]
    fn configured_budget_rejects_work_past_limit() {
        let budget = FoldBudget::with_max_work_units(1);
        let mut consumed = 0;

        assert!(budget.consume(&mut consumed).is_ok());

        assert_eq!(
            budget.consume(&mut consumed),
            Err(FoldError::WorkBudgetExceeded {
                consumed: 1,
                budget: 1,
            })
        );
    }

    #[test]
    fn not_constant_and_deferred_are_distinct_outcomes() {
        let not_constant: FoldOutcome<u64> =
            FoldOutcome::NotConstant(NotConstantReason::RuntimeDependency);

        let deferred: FoldOutcome<u64> =
            FoldOutcome::Deferred(DeferredReason::RequiresTypeResolution);

        assert_ne!(not_constant, deferred);
    }

    #[test]
    fn fold_budget_is_not_a_semantic_size_limit() {
        assert_eq!(FoldBudget::unrestricted().max_work_units, None);
        assert_eq!(
            FoldBudget::with_max_work_units(100).max_work_units,
            Some(100)
        );
    }
}
