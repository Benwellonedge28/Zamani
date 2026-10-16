
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Expressions — Evaluation Order
//!
//! File: `src/classical/expressions/evaluation_order.rs`
//! Status: Proposed; compilation and repository integration are required
//! before this file can be marked verified or frozen.
//!
//! ## Purpose
//!
//! Describe the evaluation order and control-flow boundaries of expressions
//! represented by the canonical frontend AST.
//!
//! This module produces a structural evaluation plan. It does not execute
//! expressions, resolve names, infer types, invoke operators, or lower to IR.
//!
//! ## Authority and ownership
//!
//! Canonical source AST:
//! `crate::frontend::ast::node::expressions::expression`.
//!
//! The grammar owns syntax, precedence, and associativity. The frontend AST
//! owns source structure and NodeId identity. Semantic analysis owns operator
//! meaning, effects, overload resolution, and language-defined evaluation
//! rules. The canonical IR owns the representation used by later compiler
//! phases.
//!
//! This module owns only the structural evaluation-plan representation and
//! the mapping from canonical ExpressionKind variants to those plans.
//!
//! ## Important semantic boundary
//!
//! An evaluation plan is not permission to reorder operations.
//! A consumer must preserve the plan's sequencing and control-flow boundaries.
//! Reordering, duplication, elimination, or parallelization additionally
//! requires proof that effects, exceptions, ownership, aliasing, memory
//! ordering, and observable behavior remain valid.
//!
//! Short-circuit behavior is supplied by the semantic integration through an
//! operator classifier. This module does not guess that an arbitrary operator
//! is logical merely from its spelling.
//!
//! Extension expressions remain extension-defined. Their child evaluation
//! contract must be supplied by the registered extension implementation.
//!
//! ## Scalability
//!
//! This module imposes no artificial maximum expression depth, operand count,
//! collection size, number of AST nodes, or hardware scale. Plans use ordinary
//! dynamically allocated Rust collections. Available memory and explicit
//! caller resource policies remain operational constraints.
//!
//! ## Safety
//!
//! Safe Rust only. No global mutable state, hidden evaluation, hardware
//! assumptions, or runtime execution is performed here.

use crate::frontend::ast::node::expressions::expression::{
    ExpressionExtension,
    ExpressionKind,
    OperatorRef,
};
use crate::frontend::ast::node::node_id::NodeId;

/// The short-circuit rule selected by semantic analysis for an operator.
///
/// These variants describe conditional evaluation, not the truth table or
/// type semantics of the operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShortCircuitKind {
    /// Evaluate the right operand only when the left operand requires it
    /// under the language's logical-AND semantics.
    And,

    /// Evaluate the right operand only when the left operand requires it
    /// under the language's logical-OR semantics.
    Or,

    /// Evaluate the right operand only when the left operand indicates that
    /// a fallback value is required.
    Coalesce,
}

/// Evaluation category for a binary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryEvaluation {
    /// Both operands are evaluated in the prescribed left-to-right order.
    ///
    /// This is a sequencing description, not a claim that the operation is
    /// pure or that its operands can safely be reordered.
    EagerLeftToRight,

    /// The right operand is conditionally evaluated according to this rule.
    ShortCircuit(ShortCircuitKind),
}

/// Deferred execution boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeferredKind {
    /// A lambda or closure body is not evaluated merely by constructing it.
    LambdaBody,

    /// An async body is not executed merely by constructing the async value.
    AsyncBody,
}

/// A structural evaluation plan for one expression.
///
/// Node IDs identify nodes in the canonical frontend AST. They are not
/// values, machine addresses, registers, or IR identifiers.
///
/// The consumer must interpret control-flow variants rather than flattening
/// all referenced nodes into an unconditional execution sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluationPlan {
    /// The expression has no runtime-evaluated child expressions.
    ///
    /// This includes identifier/literal syntax references and type syntax.
    /// Evaluation of the expression node itself is the caller's responsibility.
    NoRuntimeChildren,

    /// Evaluate one child expression.
    Evaluate {
        expression: NodeId,
    },

    /// Evaluate all listed child expressions in the listed order.
    ///
    /// This is appropriate only where all listed children are unconditionally
    /// evaluated. It must not be used to flatten conditional branches,
    /// match arms, or deferred bodies.
    Sequence {
        expressions: Vec<NodeId>,
    },

    /// Evaluate a binary expression.
    ///
    /// Eager operations evaluate left and then right. Short-circuit operations
    /// evaluate the left first and decide whether to evaluate the right using
    /// the specified semantic rule.
    Binary {
        left: NodeId,
        operator: OperatorRef,
        right: NodeId,
        evaluation: BinaryEvaluation,
    },

    /// Evaluate a conditional expression.
    ///
    /// Evaluate `condition` first. Evaluate `then_branch` or `else_branch`
    /// according to the condition's runtime result. Never evaluate both
    /// branches merely because both are present in the AST.
    Conditional {
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    },

    /// Evaluate a match expression.
    ///
    /// Evaluate the scrutinee first. Arm selection, pattern evaluation,
    /// guards, and the selected arm's execution are controlled by the match
    /// semantics; the arms are not an eager sequence.
    Match {
        scrutinee: NodeId,
        arms: Vec<NodeId>,
    },

    /// Evaluate a loop body according to the loop's control-flow semantics.
    ///
    /// The body may execute zero, one, or many times depending on the
    /// enclosing construct and runtime conditions. This plan does not unroll it.
    Loop {
        body: NodeId,
    },

    /// Construct a lambda/closure without eagerly executing its body.
    ///
    /// Parameter nodes describe declarations; they are not runtime argument
    /// expressions. Capture analysis and capture evaluation rules belong to
    /// the relevant semantic and closure-construction machinery.
    Deferred {
        kind: DeferredKind,
        body: NodeId,
        parameters: Vec<NodeId>,
    },

    /// Evaluate a callable expression before its arguments, then evaluate
    /// arguments in their source order.
    ///
    /// The actual invocation occurs after these prerequisites and is outside
    /// this structural child plan.
    Call {
        callee: NodeId,
        arguments: Vec<NodeId>,
    },

    /// Evaluate an indexed access.
    ///
    /// The object expression is evaluated before the index expression.
    Index {
        object: NodeId,
        index: NodeId,
    },

    /// Evaluate a member access.
    ///
    /// The object expression is evaluated. The member identifier is syntax,
    /// not an independent runtime expression to eagerly evaluate.
    MemberAccess {
        object: NodeId,
    },

    /// Evaluate an assignment.
    ///
    /// First evaluate the target as a place/designator, not as an ordinary
    /// value read; then evaluate the assigned value. A consumer must ensure
    /// the target's location computation is not repeated inadvertently.
    Assignment {
        target: NodeId,
        value: NodeId,
    },

    /// Evaluate a compound assignment.
    ///
    /// The target's place/designator is computed once, then the RHS is
    /// evaluated. Read-modify-write details, aliasing, and operator semantics
    /// must be implemented by semantic lowering without duplicating target
    /// side effects.
    CompoundAssignment {
        target: NodeId,
        operator: OperatorRef,
        value: NodeId,
    },

    /// Evaluate the expression used by an error-propagating operation.
    ///
    /// Whether evaluation continues or propagates an error is determined by
    /// the language's result/exception semantics.
    Try {
        expression: NodeId,
    },

    /// Evaluate the awaited expression before applying await semantics.
    Await {
        expression: NodeId,
    },

    /// Construct an async computation without eagerly executing its body.
    Async {
        body: NodeId,
    },

    /// Evaluate the spawned computation expression before transferring
    /// control to the concurrency/runtime subsystem.
    ///
    /// Scheduling, synchronization, cancellation, and completion ordering
    /// are not decided here.
    Spawn {
        expression: NodeId,
    },

    /// Evaluate range endpoints in source order.
    ///
    /// Absent endpoints are not evaluated.
    Range {
        endpoints: Vec<NodeId>,
        inclusive: bool,
    },

    /// Evaluate values of an aggregate in source order.
    ///
    /// Type names and field names are syntactic/type information and are not
    /// independently evaluated as runtime value expressions here.
    Aggregate {
        values: Vec<NodeId>,
    },

    /// Evaluate the value operand of a cast or type ascription.
    ///
    /// Target type syntax is not a runtime value expression in this plan.
    TypeOperation {
        expression: NodeId,
    },

    /// Delegate evaluation behavior to the expression extension contract.
    ///
    /// The core must not guess the order or effects of extension operands.
    ExtensionDefined {
        extension: ExpressionExtension,
    },
}

/// Classifies whether a binary operator requires conditional right-operand
/// evaluation.
///
/// Return `Some(kind)` only when the semantic contract guarantees the
/// corresponding short-circuit behavior. Return `None` for eager operators
/// or operators whose evaluation rule is not known to this integration.
///
/// The callback receives the canonical AST operator identity. It should use
/// semantic operator identity, not source spelling alone, whenever possible.
pub type ShortCircuitClassifier<'a> =
    dyn Fn(&OperatorRef) -> Option<ShortCircuitKind> + 'a;

/// Creates a structural evaluation plan for a canonical expression kind.
///
/// `classify_short_circuit` is supplied by the semantic integration. This
/// keeps operator meaning out of the AST and prevents accidental assumptions
/// about user-defined, dialect-specific, quantum, or future-domain operators.
///
/// The returned plan does not execute any node and does not validate that
/// referenced NodeIds exist. The aggregate AST validator owns graph integrity.
///
/// # Examples
///
/// ```ignore
/// let plan = plan_expression(expression.kind(), &|operator| {
///     match (operator.namespace(), operator.name()) {
///         ("zamani", "logical-and") => Some(ShortCircuitKind::And),
///         ("zamani", "logical-or") => Some(ShortCircuitKind::Or),
///         _ => None,
///     }
/// });
/// ```
///
/// The names above are illustrative integration examples, not a claim that
/// they are the repository's finalized semantic operator identities.
#[must_use]
pub fn plan_expression(
    kind: &ExpressionKind,
    classify_short_circuit: &ShortCircuitClassifier<'_>,
) -> EvaluationPlan {
    match kind {
        ExpressionKind::Identifier { .. }
        | ExpressionKind::Literal { .. } => EvaluationPlan::NoRuntimeChildren,

        ExpressionKind::Unary { operand, .. } => EvaluationPlan::Evaluate {
            expression: *operand,
        },

        ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            let evaluation = match classify_short_circuit(operator) {
                Some(rule) => BinaryEvaluation::ShortCircuit(rule),
                None => BinaryEvaluation::EagerLeftToRight,
            };

            EvaluationPlan::Binary {
                left: *left,
                operator: operator.clone(),
                right: *right,
                evaluation,
            }
        }

        ExpressionKind::Conditional {
            condition,
            then_branch,
            else_branch,
        } => EvaluationPlan::Conditional {
            condition: *condition,
            then_branch: *then_branch,
            else_branch: *else_branch,
        },

        ExpressionKind::Block { statements } => EvaluationPlan::Sequence {
            expressions: statements.clone(),
        },

        ExpressionKind::Match { scrutinee, arms } => EvaluationPlan::Match {
            scrutinee: *scrutinee,
            arms: arms.clone(),
        },

        ExpressionKind::Loop { body } => EvaluationPlan::Loop { body: *body },

        ExpressionKind::Lambda { parameters, body } => EvaluationPlan::Deferred {
            kind: DeferredKind::LambdaBody,
            body: *body,
            parameters: parameters.clone(),
        },

        ExpressionKind::Call { callee, arguments } => EvaluationPlan::Call {
            callee: *callee,
            arguments: arguments.clone(),
        },

        ExpressionKind::MemberAccess { object, .. } => {
            EvaluationPlan::MemberAccess { object: *object }
        }

        ExpressionKind::Index { object, index } => EvaluationPlan::Index {
            object: *object,
            index: *index,
        },

        ExpressionKind::Range {
            start,
            end,
            inclusive,
        } => {
            let mut endpoints = Vec::with_capacity(
                usize::from(start.is_some()) + usize::from(end.is_some()),
            );

            if let Some(start) = start {
                endpoints.push(*start);
            }

            if let Some(end) = end {
                endpoints.push(*end);
            }

            EvaluationPlan::Range {
                endpoints,
                inclusive: *inclusive,
            }
        }

        ExpressionKind::Array { elements }
        | ExpressionKind::Tuple { elements } => EvaluationPlan::Aggregate {
            values: elements.clone(),
        },

        ExpressionKind::Struct { fields, .. } => EvaluationPlan::Aggregate {
            values: fields.clone(),
        },

        ExpressionKind::Cast { expression, .. }
        | ExpressionKind::TypeAscription { expression, .. } => {
            EvaluationPlan::TypeOperation {
                expression: *expression,
            }
        }

        ExpressionKind::Assignment { target, value } => {
            EvaluationPlan::Assignment {
                target: *target,
                value: *value,
            }
        }

        ExpressionKind::CompoundAssignment {
            target,
            operator,
            value,
        } => EvaluationPlan::CompoundAssignment {
            target: *target,
            operator: operator.clone(),
            value: *value,
        },

        ExpressionKind::Try { expression } => EvaluationPlan::Try {
            expression: *expression,
        },

        ExpressionKind::Await { expression } => EvaluationPlan::Await {
            expression: *expression,
        },

        ExpressionKind::Async { body } => EvaluationPlan::Async { body: *body },

        ExpressionKind::Spawn { expression } => EvaluationPlan::Spawn {
            expression: *expression,
        },

        ExpressionKind::Construction {
            arguments, ..
        } => EvaluationPlan::Aggregate {
            values: arguments.clone(),
        },

        ExpressionKind::Extension(extension) => {
            EvaluationPlan::ExtensionDefined {
                extension: extension.clone(),
            }
        }
    }
}

/// Returns whether the plan contains an extension-defined evaluation boundary.
///
/// This is useful for callers that need to ensure every extension has an
/// installed evaluation contract before attempting execution or lowering.
#[must_use]
pub fn requires_extension_evaluation_contract(plan: &EvaluationPlan) -> bool {
    matches!(plan, EvaluationPlan::ExtensionDefined { .. })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    fn no_short_circuit(_: &OperatorRef) -> Option<ShortCircuitKind> {
        None
    }

    fn logical_classifier(operator: &OperatorRef) -> Option<ShortCircuitKind> {
        match (operator.namespace(), operator.name()) {
            ("zamani", "logical-and") => Some(ShortCircuitKind::And),
            ("zamani", "logical-or") => Some(ShortCircuitKind::Or),
            _ => None,
        }
    }

    #[test]
    fn binary_defaults_to_left_to_right_evaluation() {
        let operator =
            OperatorRef::native("add").expect("operator identity should be valid");

        let kind = ExpressionKind::Binary {
            left: id(1),
            operator: operator.clone(),
            right: id(2),
        };

        let plan = plan_expression(&kind, &no_short_circuit);

        assert_eq!(
            plan,
            EvaluationPlan::Binary {
                left: id(1),
                operator,
                right: id(2),
                evaluation: BinaryEvaluation::EagerLeftToRight,
            }
        );
    }

    #[test]
    fn semantic_classifier_can_enable_short_circuiting() {
        let operator = OperatorRef::new("zamani", "logical-and")
            .expect("operator identity should be valid");

        let kind = ExpressionKind::Binary {
            left: id(1),
            operator: operator.clone(),
            right: id(2),
        };

        let plan = plan_expression(&kind, &logical_classifier);

        assert_eq!(
            plan,
            EvaluationPlan::Binary {
                left: id(1),
                operator,
                right: id(2),
                evaluation: BinaryEvaluation::ShortCircuit(ShortCircuitKind::And),
            }
        );
    }

    #[test]
    fn conditional_branches_are_not_flattened_into_a_sequence() {
        let kind = ExpressionKind::Conditional {
            condition: id(1),
            then_branch: id(2),
            else_branch: Some(id(3)),
        };

        let plan = plan_expression(&kind, &no_short_circuit);

        assert_eq!(
            plan,
            EvaluationPlan::Conditional {
                condition: id(1),
                then_branch: id(2),
                else_branch: Some(id(3)),
            }
        );
    }

    #[test]
    fn calls_preserve_callee_and_argument_order() {
        let kind = ExpressionKind::Call {
            callee: id(1),
            arguments: vec![id(2), id(3), id(4)],
        };

        assert_eq!(
            plan_expression(&kind, &no_short_circuit),
            EvaluationPlan::Call {
                callee: id(1),
                arguments: vec![id(2), id(3), id(4)],
            }
        );
    }

    #[test]
    fn assignment_is_distinguished_from_ordinary_value_evaluation() {
        let kind = ExpressionKind::Assignment {
            target: id(1),
            value: id(2),
        };

        assert_eq!(
            plan_expression(&kind, &no_short_circuit),
            EvaluationPlan::Assignment {
                target: id(1),
                value: id(2),
            }
        );
    }

    #[test]
    fn lambda_body_is_deferred() {
        let kind = ExpressionKind::Lambda {
            parameters: vec![id(1), id(2)],
            body: id(3),
        };

        assert_eq!(
            plan_expression(&kind, &no_short_circuit),
            EvaluationPlan::Deferred {
                kind: DeferredKind::LambdaBody,
                body: id(3),
                parameters: vec![id(1), id(2)],
            }
        );
    }

    #[test]
    fn blocks_preserve_statement_order() {
        let kind = ExpressionKind::Block {
            statements: vec![id(1), id(2), id(3)],
        };

        assert_eq!(
            plan_expression(&kind, &no_short_circuit),
            EvaluationPlan::Sequence {
                expressions: vec![id(1), id(2), id(3)],
            }
        );
    }

    #[test]
    fn identifiers_and_literals_have_no_runtime_child_plan() {
        let identifier = ExpressionKind::Identifier {
            identifier: id(1),
        };
        let literal = ExpressionKind::Literal {
            literal: id(2),
        };

        assert_eq!(
            plan_expression(&identifier, &no_short_circuit),
            EvaluationPlan::NoRuntimeChildren
        );
        assert_eq!(
            plan_expression(&literal, &no_short_circuit),
            EvaluationPlan::NoRuntimeChildren
        );
    }
}
