
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Expression Integration
//!
//! File: src/classical/expressions/expression.rs
//!
//! STATUS
//!   Proposed implementation. Repository compilation and integration must
//!   pass before this file can be marked verified or frozen.
//!
//! PURPOSE
//!   Provide a classical-facing interface to the canonical Zamani frontend
//!   expression AST, including structural validation and stack-based graph
//!   traversal.
//!
//! AUTHORITY
//!   - grammar/specification/ and grammar/spec/
//!   - grammar/Zamani.g4
//!   - grammar/expressions/expressions.g4
//!   - grammar/expressions/conformance.md
//!   - src/frontend/ast/node/expressions/expression.rs
//!   - src/frontend/ast/node/node_id.rs
//!   - src/frontend/ast/node/source/
//!
//! OWNERSHIP
//!   The frontend AST owns Expression, ExpressionKind, NodeId, source spans,
//!   operator identities, extension identities, and local expression
//!   structural validation.
//!
//!   This module owns only classical-facing integration and graph traversal.
//!   It does not create a second AST, type system, semantic model, or IR.
//!
//! DEPENDENCIES
//!   - crate::frontend::ast::node::expressions::expression
//!   - crate::frontend::ast::node::node_id
//!   - Rust standard library
//!
//! DOES NOT OWN
//!   Parsing, grammar rules, evaluation, type inference, name resolution,
//!   overload resolution, effects, capabilities, resource discovery,
//!   optimization legality, IR construction, lowering, scheduling, hardware,
//!   runtime execution, or quantum compilation.
//!
//! TRAVERSAL CONTRACT
//!   Traversal follows child NodeIds in canonical source/structural order.
//!   Nodes are visited in preorder. Shared nodes are visited once per walk.
//!   The visited set prevents cyclic graphs from causing infinite traversal;
//!   this does not prove that the graph is structurally valid.
//!
//!   The caller supplies the authoritative AST graph resolver. A missing
//!   referenced node is reported rather than silently skipped.
//!
//! RESOURCE CONTRACT
//!   No universal expression-count, operand-count, AST-size, machine-size,
//!   hardware-count, or nesting-depth limit is introduced here.
//!   Memory consumption remains proportional to the reachable graph and
//!   traversal frontier. Callers requiring denial-of-service protection
//!   should enforce explicit compiler resource budgets at the appropriate
//!   compiler boundary.
//!
//! POCO-REAF
//!   This module is target-independent. Target capability discovery and
//!   realization remain downstream from source semantics.
//!
//! FREEZE RULE
//!   Do not duplicate or redefine frontend AST types here. Changes to the
//!   canonical AST require an explicit integration review. New classical
//!   behavior must not change the meaning of the source language.

use core::ops::ControlFlow;
use std::collections::HashSet;
use std::fmt;

pub use crate::frontend::ast::node::expressions::expression::{
    Expression,
    ExpressionError,
    ExpressionExtension,
    ExpressionKind,
    ExpressionResult,
    ExpressionValidationPolicy,
    OperatorRef,
    EXPRESSION_SCHEMA_VERSION,
};

pub use crate::frontend::ast::node::node_id::NodeId;

/// Error encountered while traversing the canonical expression graph.
///
/// This type describes traversal failures, not language-semantic failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionWalkError {
    /// The graph resolver could not find a referenced node.
    MissingNode {
        /// The unresolved AST identity.
        id: NodeId,
    },

    /// A node failed its own local expression-structure validation.
    InvalidExpression {
        /// The identity of the malformed expression.
        id: NodeId,

        /// The canonical AST validation error.
        source: ExpressionError,
    },
}

impl fmt::Display for ExpressionWalkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(
                    formatter,
                    "expression graph references missing node {id:?}"
                )
            }

            Self::InvalidExpression { id, source } => {
                write!(
                    formatter,
                    "expression {id:?} failed structural validation: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ExpressionWalkError {}

/// Result returned by expression-graph traversal.
pub type ExpressionWalkResult<T> = Result<T, ExpressionWalkError>;

/// Validates one expression using the canonical AST's structural rules.
///
/// This does not resolve child IDs against an AST graph and does not perform
/// semantic analysis.
///
/// # Errors
///
/// Returns the canonical AST's structural validation error.
pub fn validate_expression(
    expression: &Expression,
    policy: ExpressionValidationPolicy,
) -> ExpressionResult<()> {
    expression.validate_structure(policy)
}

/// Visits the reachable expression graph in preorder without recursive calls.
///
/// # Arguments
///
/// - `root`: identity of the expression where traversal starts.
/// - `resolve`: authoritative lookup of AST expressions by `NodeId`.
/// - `visitor`: callback invoked for each expression.
///
/// # Traversal behavior
///
/// - The root is visited first.
/// - Child order follows `Expression::child_node_ids()`.
/// - Shared nodes are visited once.
/// - Cyclic references cannot cause an infinite traversal.
/// - Missing child nodes return `ExpressionWalkError::MissingNode`.
/// - Malformed expressions return `ExpressionWalkError::InvalidExpression`.
/// - `ControlFlow::Break` stops traversal immediately.
///
/// This is a structural graph walk, not an evaluator. It must not be used to
/// infer execution order, short-circuit behavior, effect ordering, or whether
/// a conditional branch will execute.
///
/// # Resource behavior
///
/// The visited set and explicit stack grow with the reachable graph. No
/// artificial language-level traversal ceiling is introduced. Allocation
/// failure behavior follows the Rust standard library and process environment.
///
/// # Example
///
/// ```ignore
/// use core::ops::ControlFlow;
///
/// let result = walk_expression_graph(
///     root_id,
///     |id| ast.expression(id),
///     |expression| {
///         inspect(expression);
///         ControlFlow::<()>::Continue(())
///     },
/// )?;
/// ```
pub fn walk_expression_graph<'a, B, Resolve, Visit>(
    root: NodeId,
    mut resolve: Resolve,
    mut visitor: Visit,
) -> ExpressionWalkResult<ControlFlow<B>>
where
    Resolve: FnMut(NodeId) -> Option<&'a Expression>,
    Visit: FnMut(&'a Expression) -> ControlFlow<B>,
{
    let mut pending = vec![root];
    let mut visited = HashSet::new();

    while let Some(id) = pending.pop() {
        // Mark on discovery. This prevents cycles and duplicate work for
        // shared graph nodes without depending on hash iteration order.
        if !visited.insert(id) {
            continue;
        }

        let expression = resolve(id)
            .ok_or(ExpressionWalkError::MissingNode { id })?;

        expression
            .validate_structure(ExpressionValidationPolicy::unrestricted())
            .map_err(|source| ExpressionWalkError::InvalidExpression {
                id,
                source,
            })?;

        if let ControlFlow::Break(value) = visitor(expression) {
            return Ok(ControlFlow::Break(value));
        }

        // Canonical child enumeration preserves source/structural order.
        // Reverse the push order because the work stack is last-in, first-out.
        let children = expression.child_node_ids();

        pending.extend(children.into_iter().rev());
    }

    Ok(ControlFlow::Continue(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_expression_types_are_reexported() {
        fn accepts_canonical_expression(_: &Expression) {}

        // The test intentionally verifies the public type contract at compile
        // time without constructing a duplicate expression representation.
        let _ = accepts_canonical_expression;
    }

    #[test]
    fn unrestricted_policy_does_not_add_classical_limits() {
        let policy = ExpressionValidationPolicy::unrestricted();

        assert_eq!(policy.max_children, None);
        assert_eq!(policy.max_extension_namespace_bytes, None);
        assert_eq!(policy.max_extension_name_bytes, None);
        assert_eq!(policy.max_operator_namespace_bytes, None);
        assert_eq!(policy.max_operator_name_bytes, None);
    }

    #[test]
    fn walk_error_reports_missing_nodes() {
        let id = NodeId::default();
        let error = ExpressionWalkError::MissingNode { id };

        assert!(error.to_string().contains("missing node"));
    }
}
