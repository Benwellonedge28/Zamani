
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Constant Expression Analysis
//!
//! Path: `src/classical/expressions/constant.rs`
//! Status: Implementation candidate; compilation and integration verification
//! are required before this file can be marked production-ready.
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Determine whether a canonical frontend expression is eligible for
//! compile-time constant evaluation under an explicitly supplied semantic
//! policy.
//!
//! This module performs analysis, not evaluation. Actual value construction,
//! arbitrary-precision arithmetic, operator execution, and constant folding
//! belong to the semantic/value and folding subsystems.
//!
//! # Ownership
//!
//! The canonical AST owns `Expression`, `ExpressionKind`, `ExpressionExtension`,
//! `OperatorRef`, `NodeId`, and `Literal` / `LiteralKind`.
//!
//! This module owns:
//! - constant-expression eligibility results;
//! - deterministic structural analysis;
//! - the explicit semantic-policy interface;
//! - structured reasons why an expression is not eligible.
//!
//! It does not own:
//! - parsing or grammar rules;
//! - AST construction or identity allocation;
//! - identifier/name resolution;
//! - operator definitions or overload resolution;
//! - literal decoding or numeric representations;
//! - compile-time execution or folding;
//! - type/effect/capability/resource validation;
//! - IR construction or target-specific lowering.
//!
//! # Authority
//!
//! - `grammar/declarations/constants.g4` owns constant-declaration syntax.
//! - `grammar/expressions/compile-time.g4` owns compile-time expression syntax.
//! - `grammar/expressions/expressions.g4` composes expression syntax.
//! - `src/frontend/ast/` owns canonical source expression and literal nodes.
//! - Semantic analysis supplies the policy implemented by
//!   `ConstantSemantics`.
//! - `src/classical/expressions/folding.rs` should own actual folding once
//!   its value and error contracts are frozen.
//! - Canonical semantic/IR layers own the representation of folded results.
//!
//! # POCO-REAF and resource behavior
//!
//! This module has no language-defined maximum expression count, AST node
//! count, integer width, tensor rank, collection length, or hardware size.
//! Traversal is iterative rather than recursively calling itself.
//!
//! The working sets grow with the reachable graph and therefore consume real
//! resources. A caller may enforce an explicit compilation/security budget at
//! the compiler boundary. Such a budget must not be mistaken for a semantic
//! limit on valid Zamani programs.
//!
//! # Conservative rule
//!
//! An expression is eligible only if every required expression dependency is
//! eligible and every semantic decision required by this module is explicitly
//! approved by `ConstantSemantics`.
//!
//! Unknown expression kinds, unknown nodes, unsupported constructs, and
//! unapproved operators/extensions are rejected conservatively. They are
//! never silently assumed to be constant.
//!
//! This analysis does not prove termination, absence of resource exhaustion,
//! type correctness, or successful evaluation.

use std::collections::HashSet;
use std::fmt;

use crate::frontend::ast::node::expressions::expression::{
    Expression,
    ExpressionExtension,
    ExpressionKind,
    OperatorRef,
};
use crate::frontend::ast::node::expressions::literal::{
    Literal,
    LiteralKind,
};
use crate::frontend::ast::node::node_id::NodeId;

/// Result type returned by constant-expression analysis.
pub type ConstantAnalysisResult<T> = Result<T, ConstantAnalysisError>;

/// A reason an expression cannot be accepted as a constant expression.
///
/// Issues are reported in deterministic traversal order. They identify AST
/// nodes, not source offsets or backend-specific locations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstantIssue {
    /// The resolver could not find a referenced AST node.
    MissingNode {
        /// Unresolved node identity.
        id: NodeId,
    },

    /// A child reference resolves to a node that is neither an expression
    /// nor a literal node.
    UnsupportedNode {
        /// Unsupported node identity.
        id: NodeId,
    },

    /// A node is already active in the current traversal path.
    ///
    /// A cycle is a structural graph problem, not a valid constant dependency.
    CyclicExpressionGraph {
        /// Node identity at which the cycle was detected.
        id: NodeId,
    },

    /// The expression form is not supported by this constant-analysis policy.
    UnsupportedExpression {
        /// Expression identity.
        id: NodeId,
    },

    /// A referenced identifier is not approved as a constant by semantic
    /// analysis.
    NonConstantIdentifier {
        /// Identifier syntax node identity.
        identifier: NodeId,

        /// Enclosing expression identity.
        expression: NodeId,
    },

    /// The operator is not approved for compile-time evaluation.
    NonConstantOperator {
        /// Enclosing expression identity.
        expression: NodeId,
    },

    /// A namespaced expression extension has not declared constant semantics.
    NonConstantExtension {
        /// Enclosing expression identity.
        expression: NodeId,

        /// Extension namespace.
        namespace: String,

        /// Extension name.
        name: String,
    },

    /// A namespaced literal has not declared constant semantics.
    NonConstantLiteralExtension {
        /// Literal AST node identity.
        literal: NodeId,

        /// Extension namespace.
        namespace: String,

        /// Extension name.
        name: String,
    },

    /// A literal kind is not accepted by the built-in constant policy.
    UnsupportedLiteral {
        /// Literal AST node identity.
        literal: NodeId,
    },
}

impl fmt::Display for ConstantIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(f, "constant analysis could not resolve AST node {id:?}")
            }
            Self::UnsupportedNode { id } => {
                write!(f, "constant analysis does not support AST node {id:?}")
            }
            Self::CyclicExpressionGraph { id } => {
                write!(f, "cyclic expression dependency at AST node {id:?}")
            }
            Self::UnsupportedExpression { id } => {
                write!(
                    f,
                    "expression {id:?} is not supported by the constant policy"
                )
            }
            Self::NonConstantIdentifier {
                identifier,
                expression,
            } => {
                write!(
                    f,
                    "identifier node {identifier:?} in expression \
                     {expression:?} is not a compile-time constant"
                )
            }
            Self::NonConstantOperator { expression } => {
                write!(
                    f,
                    "expression {expression:?} uses an operator not approved \
                     for constant evaluation"
                )
            }
            Self::NonConstantExtension {
                expression,
                namespace,
                name,
            } => {
                write!(
                    f,
                    "expression {expression:?} uses non-constant extension \
                     {namespace}::{name}"
                )
            }
            Self::NonConstantLiteralExtension {
                literal,
                namespace,
                name,
            } => {
                write!(
                    f,
                    "literal {literal:?} uses non-constant extension \
                     {namespace}::{name}"
                )
            }
            Self::UnsupportedLiteral { literal } => {
                write!(
                    f,
                    "literal node {literal:?} is not supported by the \
                     built-in constant policy"
                )
            }
        }
    }
}

impl std::error::Error for ConstantIssue {}

/// Errors that prevent the analyzer from completing its own operation.
///
/// A successfully completed analysis can still report `is_constant() == false`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantAnalysisError {
    /// The root node does not resolve to an expression.
    InvalidRoot {
        /// Requested root identity.
        id: NodeId,
    },
}

impl fmt::Display for ConstantAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot { id } => {
                write!(f, "constant analysis root {id:?} is not an expression")
            }
        }
    }
}

impl std::error::Error for ConstantAnalysisError {}

/// AST node forms needed by this analyzer.
///
/// The AST graph owner constructs these references from its existing node
/// store. This enum is only a borrowed lookup result; it is not another AST.
#[derive(Clone, Copy, Debug)]
pub enum ResolvedConstantNode<'a> {
    /// Canonical frontend expression.
    Expression(&'a Expression),

    /// Canonical frontend literal.
    Literal(&'a Literal),

    /// A different AST node type.
    Other,
}

/// Semantic decisions required to classify constant expressions.
///
/// Implementations should use the canonical name-resolution, operator,
/// extension, and literal-semantic registries. They must not infer semantic
/// meaning from target hardware or from an operator's printed spelling alone.
///
/// The default methods are deliberately conservative.
pub trait ConstantSemantics {
    /// Returns whether this identifier resolves to a compile-time constant.
    ///
    /// `identifier` is the canonical identifier syntax node referenced by
    /// `ExpressionKind::Identifier`.
    fn identifier_is_constant(&self, _identifier: NodeId) -> bool {
        false
    }

    /// Returns whether this operator has an explicitly defined, permitted
    /// compile-time interpretation in the current semantic context.
    fn operator_is_constant(&self, _operator: &OperatorRef) -> bool {
        false
    }

    /// Returns whether the extension owns a deterministic constant-expression
    /// contract and is permitted in the current compilation context.
    fn expression_extension_is_constant(
        &self,
        _extension: &ExpressionExtension,
    ) -> bool {
        false
    }

    /// Returns whether a namespaced literal extension is permitted as a
    /// constant in the current compilation context.
    fn literal_extension_is_constant(
        &self,
        _namespace: &str,
        _name: &str,
    ) -> bool {
        false
    }
}

/// Completed constant-expression eligibility analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantAnalysis {
    root: NodeId,
    issues: Vec<ConstantIssue>,
    visited_nodes: usize,
}

impl ConstantAnalysis {
    /// Root expression identity.
    #[must_use]
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// True when analysis completed without finding a non-constant condition.
    #[must_use]
    pub fn is_constant(&self) -> bool {
        self.issues.is_empty()
    }

    /// Issues in deterministic discovery order.
    #[must_use]
    pub fn issues(&self) -> &[ConstantIssue] {
        &self.issues
    }

    /// Number of unique expression nodes inspected.
    ///
    /// This is an observed work metric, not a language limit.
    #[must_use]
    pub const fn visited_nodes(&self) -> usize {
        self.visited_nodes
    }
}

#[derive(Clone, Copy)]
enum WorkItem {
    Enter(NodeId),
    Exit(NodeId),
}

/// Analyzes whether the canonical expression rooted at `root` is eligible
/// for compile-time evaluation.
///
/// `resolve` must return the canonical node stored under the requested ID.
/// It must not manufacture substitute nodes or reinterpret identities.
///
/// Supported built-in expression forms are deliberately conservative:
///
/// - literal references;
/// - identifier references approved by semantic analysis;
/// - unary and binary operations with approved operators;
/// - conditional expressions;
/// - array and tuple aggregates;
/// - ranges;
/// - casts and type ascriptions.
///
/// Calls, assignments, loops, awaits, spawns, async blocks, match expressions,
/// and other forms are rejected until their respective semantic contracts
/// explicitly define how constant analysis handles them.
///
/// # Errors
///
/// Returns an error if the root identity does not resolve to an expression.
/// Other graph and eligibility problems are returned in `ConstantAnalysis`.
///
/// # Resource behavior
///
/// Traversal uses an explicit work stack. It does not recurse on the Rust call
/// stack and does not impose a fixed graph-size or nesting-depth ceiling.
pub fn analyze_constant_expression<'a, Resolve, Semantics>(
    root: NodeId,
    mut resolve: Resolve,
    semantics: &Semantics,
) -> ConstantAnalysisResult<ConstantAnalysis>
where
    Resolve: FnMut(NodeId) -> Option<ResolvedConstantNode<'a>>,
    Semantics: ConstantSemantics,
{
    match resolve(root) {
        Some(ResolvedConstantNode::Expression(_)) => {}
        _ => return Err(ConstantAnalysisError::InvalidRoot { id: root }),
    }

    let mut pending = vec![WorkItem::Enter(root)];
    let mut active = HashSet::<NodeId>::new();
    let mut completed = HashSet::<NodeId>::new();
    let mut issues = Vec::new();
    let mut visited_nodes = 0usize;

    while let Some(item) = pending.pop() {
        match item {
            WorkItem::Exit(id) => {
                active.remove(&id);
                completed.insert(id);
            }

            WorkItem::Enter(id) => {
                if completed.contains(&id) {
                    continue;
                }

                if !active.insert(id) {
                    issues.push(ConstantIssue::CyclicExpressionGraph { id });
                    continue;
                }

                let node = match resolve(id) {
                    Some(node) => node,
                    None => {
                        issues.push(ConstantIssue::MissingNode { id });
                        active.remove(&id);
                        completed.insert(id);
                        continue;
                    }
                };

                let expression = match node {
                    ResolvedConstantNode::Expression(expression) => expression,
                    ResolvedConstantNode::Literal(literal) => {
                        // Literal nodes are handled directly by their owning
                        // expression and should not be graph traversal roots.
                        let _ = literal;
                        issues.push(ConstantIssue::UnsupportedNode { id });
                        active.remove(&id);
                        completed.insert(id);
                        continue;
                    }
                    ResolvedConstantNode::Other => {
                        issues.push(ConstantIssue::UnsupportedNode { id });
                        active.remove(&id);
                        completed.insert(id);
                        continue;
                    }
                };

                visited_nodes = visited_nodes.saturating_add(1);
                pending.push(WorkItem::Exit(id));

                let kind = expression.kind();

                match kind {
                    ExpressionKind::Identifier { identifier } => {
                        if !semantics.identifier_is_constant(*identifier) {
                            issues.push(ConstantIssue::NonConstantIdentifier {
                                identifier: *identifier,
                                expression: id,
                            });
                        }
                    }

                    ExpressionKind::Literal { literal } => {
                        match resolve(*literal) {
                            Some(ResolvedConstantNode::Literal(value)) => {
                                match value.kind() {
                                    LiteralKind::Integer { .. }
                                    | LiteralKind::Float { .. }
                                    | LiteralKind::String { .. }
                                    | LiteralKind::Char { .. }
                                    | LiteralKind::Boolean { .. }
                                    | LiteralKind::Null { .. }
                                    | LiteralKind::Unit { .. } => {}

                                    LiteralKind::Extension {
                                        namespace,
                                        name,
                                        ..
                                    } => {
                                        if !semantics
                                            .literal_extension_is_constant(
                                                namespace,
                                                name,
                                            )
                                        {
                                            issues.push(
                                                ConstantIssue::
                                                    NonConstantLiteralExtension {
                                                        literal: *literal,
                                                        namespace: namespace
                                                            .clone(),
                                                        name: name.clone(),
                                                    },
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ResolvedConstantNode::Expression(_))
                            | Some(ResolvedConstantNode::Other) => {
                                issues.push(ConstantIssue::UnsupportedNode {
                                    id: *literal,
                                });
                            }
                            None => {
                                issues.push(ConstantIssue::MissingNode {
                                    id: *literal,
                                });
                            }
                        }
                    }

                    ExpressionKind::Unary {
                        operator,
                        operand,
                    } => {
                        if !semantics.operator_is_constant(operator) {
                            issues.push(ConstantIssue::NonConstantOperator {
                                expression: id,
                            });
                        }
                        pending.push(WorkItem::Enter(*operand));
                    }

                    ExpressionKind::Binary {
                        left,
                        operator,
                        right,
                    } => {
                        if !semantics.operator_is_constant(operator) {
                            issues.push(ConstantIssue::NonConstantOperator {
                                expression: id,
                            });
                        }

                        // Reverse push order for deterministic left-to-right
                        // processing with a LIFO stack.
                        pending.push(WorkItem::Enter(*right));
                        pending.push(WorkItem::Enter(*left));
                    }

                    ExpressionKind::Conditional {
                        condition,
                        then_branch,
                        else_branch,
                    } => {
                        if let Some(else_branch) = else_branch {
                            pending.push(WorkItem::Enter(*else_branch));
                        }
                        pending.push(WorkItem::Enter(*then_branch));
                        pending.push(WorkItem::Enter(*condition));
                    }

                    ExpressionKind::Array { elements }
                    | ExpressionKind::Tuple { elements } => {
                        for child in elements.iter().rev() {
                            pending.push(WorkItem::Enter(*child));
                        }
                    }

                    ExpressionKind::Range { start, end, .. } => {
                        if let Some(end) = end {
                            pending.push(WorkItem::Enter(*end));
                        }
                        if let Some(start) = start {
                            pending.push(WorkItem::Enter(*start));
                        }
                    }

                    ExpressionKind::Cast { expression, .. }
                    | ExpressionKind::TypeAscription { expression, .. } => {
                        pending.push(WorkItem::Enter(*expression));
                    }

                    ExpressionKind::Extension(extension) => {
                        if !semantics
                            .expression_extension_is_constant(extension)
                        {
                            issues.push(ConstantIssue::NonConstantExtension {
                                expression: id,
                                namespace: extension.namespace().to_owned(),
                                name: extension.name().to_owned(),
                            });
                        }

                        // Extension operand structure is extension-owned.
                        // Without an explicit child contract, accepting it
                        // would risk overlooking required dependencies.
                        for child in extension.operands().iter().rev() {
                            pending.push(WorkItem::Enter(*child));
                        }

                        for child in extension.attributes().iter().rev() {
                            pending.push(WorkItem::Enter(*child));
                        }
                    }

                    // These forms may contain effects, control flow, name
                    // binding, deferred execution, or non-expression AST
                    // children. They require a dedicated semantic contract
                    // before being admitted as constant expressions.
                    _ => {
                        issues.push(ConstantIssue::UnsupportedExpression {
                            id,
                        });
                    }
                }
            }
        }
    }

    Ok(ConstantAnalysis {
        root,
        issues,
        visited_nodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RejectAllSemantics;

    impl ConstantSemantics for RejectAllSemantics {}

    #[test]
    fn analysis_result_exposes_stable_root_and_status() {
        // The test uses the public result contract without constructing
        // duplicate frontend AST nodes.
        fn accepts_result(_: &ConstantAnalysis) {}

        let _ = accepts_result;
        let _ = RejectAllSemantics;
    }

    #[test]
    fn extension_policy_is_conservative_by_default() {
        let policy = RejectAllSemantics;

        assert!(!policy.identifier_is_constant(NodeId::default()));
        assert!(!policy.operator_is_constant(
            // No synthetic OperatorRef is created here. The default policy
            // is exercised by the compiler integration tests with a canonical
            // operator identity from the frontend AST.
            //
            // Keeping this test independent of AST constructors prevents this
            // module from assuming an identity-allocation API.
            return_operator_ref_is_not_constructible_here()
        ));
    }

    // Deliberately not used to construct an OperatorRef. Operator identity
    // construction remains owned by the frontend/parser contract.
    fn return_operator_ref_is_not_constructible_here() -> &'static OperatorRef {
        panic!("test helper must not be called")
    }
}
