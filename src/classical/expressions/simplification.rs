
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Expression Simplification
//!
//! File: src/classical/expressions/simplification.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!   Implementation candidate. Freeze only after compiling against the
//!   repository's actual AST graph store and running the integration suite.
//!
//! PURPOSE
//!   Provide a target-independent, iterative framework for simplifying
//!   expressions represented by the canonical frontend AST.
//!
//! AUTHORITY
//!   - grammar/expressions/expressions.g4
//!   - grammar/expressions/compile-time.g4
//!   - grammar/declarations/constants.g4
//!   - src/frontend/ast/node/expressions/expression.rs
//!   - src/frontend/ast/node/expressions/literal.rs
//!   - src/frontend/ast/node/node_id.rs
//!   - the canonical semantic analysis and value/operator contracts
//!
//! OWNERSHIP
//!   The frontend owns AST nodes, node identity, source spans, operators,
//!   extensions, and source structure.
//!
//!   This module owns:
//!   - simplification request and result contracts;
//!   - iterative traversal of a canonical expression graph;
//!   - cycle and missing-node detection;
//!   - optional, explicit work budgets;
//!   - deterministic reporting of proposed equivalent replacements.
//!
//! DOES NOT OWN
//!   Parsing, AST construction, literal decoding, name resolution, type
//!   inference, operator semantics, effect analysis, constant evaluation,
//!   arbitrary-precision arithmetic, canonical IR, or runtime execution.
//!
//! SEMANTIC SAFETY
//!   Every rewrite must be justified by the supplied semantic adapter.
//!   This module never assumes that an expression is pure, that an operator
//!   is built-in, or that two expressions are equivalent based on spelling.
//!
//! AST AND IR OWNERSHIP
//!   Replacements identify existing canonical AST nodes by NodeId. This
//!   module does not mutate the AST, create AST nodes, or create a second IR.
//!   Applying a rewrite to a mutable/transformed program belongs to the
//!   canonical AST transformation or IR infrastructure.
//!
//! POCO-REAF
//!   No machine, hardware, register-width, expression-count, or AST-size
//!   semantic ceiling is introduced. Operational work budgets are explicit
//!   and optional. Actual memory and time remain finite.
//!
//! DETERMINISM
//!   Children are processed in canonical source order. Ordered maps are used
//!   for published results. Hash collections are used only for membership
//!   checks; their iteration order never affects results.
//!
//! FREEZE RULE
//!   Do not duplicate AST types or add hard-coded operator rewrite tables.
//!   Changes to the AST graph contract, semantic adapter, or rewrite
//!   application mechanism require an integration review.

use std::collections::{BTreeMap, HashSet};
use std::error::Error as StdError;
use std::fmt;

use crate::frontend::ast::node::expressions::expression::{
    Expression,
    ExpressionError,
    ExpressionKind,
    ExpressionValidationPolicy,
};
use crate::frontend::ast::node::node_id::NodeId;

// =============================================================================
// Public result types
// =============================================================================

/// Result type for expression simplification.
pub type SimplificationResult<T, E> = Result<T, SimplificationError<E>>;

/// A canonical AST node resolved by the caller's AST graph.
///
/// `Other` means that the ID resolves to a valid node which is not represented
/// by the canonical `Expression` type. For example, an expression may refer to
/// a literal, identifier, type, pattern, or declaration node.
///
/// This is a borrowed lookup result, not another AST representation.
#[derive(Clone, Copy, Debug)]
pub enum ResolvedSimplificationNode<'a> {
    /// A canonical frontend expression.
    Expression(&'a Expression),

    /// Another existing canonical frontend AST node.
    Other,
}

/// A child dependency and its proposed simplified representative.
///
/// The original ID identifies the exact edge in the source expression.
/// The simplified ID is the representative returned by processing that
/// dependency. If no rewrite was approved, both IDs are equal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimplifiedChild {
    /// Original child identity from the canonical AST.
    pub original: NodeId,

    /// Proposed simplified representative of that child.
    pub simplified: NodeId,
}

/// Why a particular expression was or was not rewritten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimplificationReason {
    /// The semantic adapter approved no replacement.
    Unchanged,

    /// The semantic adapter approved an equivalent canonical AST node.
    Rewritten {
        /// Original expression identity.
        from: NodeId,

        /// Existing canonical AST node selected as its equivalent.
        to: NodeId,
    },
}

/// The result for one node encountered by the graph traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimplificationRecord {
    /// The canonical AST node whose result this record describes.
    pub original: NodeId,

    /// The approved representative for this node.
    ///
    /// This is a reference to an existing canonical AST node. It is not an
    /// allocated AST identity, a machine address, or an IR value.
    pub simplified: NodeId,

    /// The reason this result was produced.
    pub reason: SimplificationReason,
}

impl SimplificationRecord {
    /// Returns whether this record represents an approved rewrite.
    #[must_use]
    pub fn was_rewritten(&self) -> bool {
        self.original != self.simplified
    }
}

/// The completed result of one bottom-up simplification pass.
///
/// This pass does not mutate the original AST. Consumers may use the mapping
/// to build a separate transformed AST through the canonical transformation
/// infrastructure, or to inform a later IR pass.
///
/// One pass does not promise a global fixed point. A consumer that performs
/// repeated passes must impose its own explicit termination/budget policy and
/// detect rewrite oscillation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimplificationReport {
    root: NodeId,
    records: BTreeMap<NodeId, SimplificationRecord>,
    visited_nodes: usize,
    rewrite_count: usize,
}

impl SimplificationReport {
    /// Returns the requested root identity.
    #[must_use]
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// Returns the root's proposed representative.
    ///
    /// The root record is guaranteed to exist for a successful report.
    #[must_use]
    pub fn simplified_root(&self) -> NodeId {
        self.records
            .get(&self.root)
            .map(|record| record.simplified)
            .unwrap_or(self.root)
    }

    /// Returns all processed records in deterministic NodeId order.
    #[must_use]
    pub fn records(&self) -> &BTreeMap<NodeId, SimplificationRecord> {
        &self.records
    }

    /// Returns a record for a processed node, if present.
    #[must_use]
    pub fn record(&self, id: NodeId) -> Option<&SimplificationRecord> {
        self.records.get(&id)
    }

    /// Number of unique canonical AST nodes resolved by the pass.
    ///
    /// This is an observed metric, not a language-level limit.
    #[must_use]
    pub const fn visited_nodes(&self) -> usize {
        self.visited_nodes
    }

    /// Number of nodes for which the pass approved a different representative.
    ///
    /// This count is for this pass only; it does not count hypothetical
    /// downstream AST or IR rewrites.
    #[must_use]
    pub const fn rewrite_count(&self) -> usize {
        self.rewrite_count
    }

    /// Returns whether the pass approved no replacements.
    #[must_use]
    pub const fn is_unchanged(&self) -> bool {
        self.rewrite_count == 0
    }
}

// =============================================================================
// Explicit operational budget
// =============================================================================

/// Optional work-count budget for one simplification pass.
///
/// `None` means that this module does not impose a work-count budget.
/// It does not promise infinite memory, infinite execution time, or unlimited
/// host resources.
///
/// This is an operational control, not a restriction on which Zamani programs
/// are valid. Callers can use it for compilation policy or denial-of-service
/// protection without embedding fixed semantic ceilings in the language.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SimplificationBudget {
    /// Maximum number of unique nodes that may be processed, if configured.
    pub max_work_units: Option<usize>,
}

impl SimplificationBudget {
    /// Creates an operation with no module-level work-count budget.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_work_units: None,
        }
    }

    /// Creates an operation with an explicit work-count budget.
    #[must_use]
    pub const fn with_max_work_units(maximum: usize) -> Self {
        Self {
            max_work_units: Some(maximum),
        }
    }

    /// Accounts for one unit of work.
    ///
    /// # Errors
    ///
    /// Returns `WorkBudgetExceeded` when another unit would exceed the
    /// configured budget, or `WorkCounterOverflow` if the counter cannot
    /// represent the additional unit.
    pub fn consume(
        self,
        consumed: &mut usize,
    ) -> Result<(), SimplificationBudgetError> {
        if let Some(maximum) = self.max_work_units {
            if *consumed >= maximum {
                return Err(SimplificationBudgetError::WorkBudgetExceeded {
                    consumed: *consumed,
                    budget: maximum,
                });
            }
        }

        *consumed = consumed
            .checked_add(1)
            .ok_or(SimplificationBudgetError::WorkCounterOverflow)?;

        Ok(())
    }
}

/// Errors specific to operational work accounting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimplificationBudgetError {
    /// The configured work budget has been exhausted.
    WorkBudgetExceeded {
        /// Number of units already consumed.
        consumed: usize,

        /// Configured maximum.
        budget: usize,
    },

    /// The host-sized work counter cannot represent another unit.
    WorkCounterOverflow,
}

impl fmt::Display for SimplificationBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkBudgetExceeded { consumed, budget } => write!(
                f,
                "simplification work budget exceeded: consumed {consumed}, \
                 budget {budget}"
            ),
            Self::WorkCounterOverflow => {
                f.write_str("simplification work counter overflow")
            }
        }
    }
}

impl StdError for SimplificationBudgetError {}

// =============================================================================
// Semantic rewrite contract
// =============================================================================

/// Semantic authority for approving expression simplifications.
///
/// Implementations belong to the semantic/value/optimization integration.
/// This module does not supply a default list of rewrite rules because
/// equivalence depends on Zamani's actual operator, type, effect, exception,
/// overflow, memory, and evaluation-order semantics.
///
/// Implementations must:
///
/// - use canonical operator and extension identities;
/// - preserve observable behavior, including effects and exceptions;
/// - preserve type and ownership constraints;
/// - respect short-circuit and conditional evaluation;
/// - account for integer overflow, floating-point behavior, and conversions;
/// - avoid duplicating or eliminating effectful operations;
/// - return only an existing canonical AST node as a replacement;
/// - avoid host execution, I/O, hardware probing, or target-dependent
///   changes to source-language meaning.
///
/// Returning `None` means no rewrite is approved for this node during this
/// pass. It is not an error and does not imply that the expression can never
/// be simplified by a later semantic phase.
pub trait SimplificationSemantics {
    /// Error type for semantic rewrite decisions.
    type Error: StdError + Send + Sync + 'static;

    /// Proposes an equivalent existing AST node for this expression.
    ///
    /// `children` are supplied in the canonical AST's child order. Each entry
    /// preserves the original edge and gives the result of processing that
    /// dependency. Non-expression child nodes remain unchanged.
    ///
    /// Return `Some(id)` only when the semantic contract proves that the
    /// referenced existing AST node is equivalent to `expression` in the
    /// current context. Return `None` when no rewrite is approved.
    ///
    /// This method must not mutate the AST. The caller verifies that any
    /// returned replacement ID resolves in the same canonical AST graph.
    fn simplify(
        &self,
        expression: &Expression,
        children: &[SimplifiedChild],
    ) -> Result<Option<NodeId>, Self::Error>;
}

// =============================================================================
// Structured errors
// =============================================================================

/// Failures that prevent a simplification pass from completing.
#[derive(Debug)]
#[non_exhaustive]
pub enum SimplificationError<E> {
    /// The root identity did not resolve to a canonical expression.
    InvalidRoot {
        /// Requested root identity.
        id: NodeId,
    },

    /// A referenced child or replacement did not resolve in the AST graph.
    MissingNode {
        /// Expression containing the unresolved reference, if known.
        parent: Option<NodeId>,

        /// Missing canonical identity.
        id: NodeId,
    },

    /// An expression failed canonical local structural validation.
    InvalidExpression {
        /// Expression identity.
        id: NodeId,

        /// Canonical AST validation failure.
        source: ExpressionError,
    },

    /// A cycle was encountered in the expression graph.
    ///
    /// Cycles are reported instead of being treated as valid simplification
    /// dependencies. A DAG may share nodes; sharing is not itself a cycle.
    CyclicExpressionGraph {
        /// Node at which the active traversal path encountered a cycle.
        id: NodeId,
    },

    /// The semantic adapter failed while evaluating rewrite legality.
    Semantic(E),

    /// The configured operational work budget was exhausted.
    WorkBudgetExceeded {
        /// Units consumed before failure.
        consumed: usize,

        /// Configured budget.
        budget: usize,
    },

    /// The work counter cannot represent another unit.
    WorkCounterOverflow,

    /// The semantic adapter returned a replacement that is not present in
    /// the canonical AST graph.
    InvalidReplacement {
        /// Expression being simplified.
        expression: NodeId,

        /// Replacement identity returned by the semantic adapter.
        replacement: NodeId,
    },
}

impl<E: fmt::Display> fmt::Display for SimplificationError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot { id } => {
                write!(f, "simplification root {id:?} is not an expression")
            }
            Self::MissingNode { parent, id } => match parent {
                Some(parent) => write!(
                    f,
                    "expression {parent:?} references missing AST node {id:?}"
                ),
                None => write!(f, "missing AST node {id:?}"),
            },
            Self::InvalidExpression { id, source } => {
                write!(
                    f,
                    "expression {id:?} failed structural validation: {source}"
                )
            }
            Self::CyclicExpressionGraph { id } => {
                write!(f, "cyclic expression graph at AST node {id:?}")
            }
            Self::Semantic(source) => {
                write!(f, "semantic simplification failed: {source}")
            }
            Self::WorkBudgetExceeded { consumed, budget } => write!(
                f,
                "simplification work budget exceeded: consumed {consumed}, \
                 budget {budget}"
            ),
            Self::WorkCounterOverflow => {
                f.write_str("simplification work counter overflow")
            }
            Self::InvalidReplacement {
                expression,
                replacement,
            } => write!(
                f,
                "simplification of {expression:?} returned unresolved \
                 replacement {replacement:?}"
            ),
        }
    }
}

impl<E: StdError + 'static> StdError for SimplificationError<E> {}

// =============================================================================
// Iterative graph traversal
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VisitState {
    Active,
    Complete,
}

#[derive(Clone, Copy, Debug)]
enum WorkItem {
    Enter {
        id: NodeId,
        parent: Option<NodeId>,
    },
    Exit {
        id: NodeId,
    },
}

fn budget_error<E>(
    error: SimplificationBudgetError,
) -> SimplificationError<E> {
    match error {
        SimplificationBudgetError::WorkBudgetExceeded { consumed, budget } => {
            SimplificationError::WorkBudgetExceeded { consumed, budget }
        }
        SimplificationBudgetError::WorkCounterOverflow => {
            SimplificationError::WorkCounterOverflow
        }
    }
}

/// Performs one bottom-up simplification pass over a canonical expression DAG.
///
/// # Arguments
///
/// - `root`: identity of the root expression.
/// - `resolve`: lookup into the existing canonical AST graph. Return
///   `None` only when an ID does not exist. Return `Other` for a valid node
///   that is not represented by `Expression`.
/// - `semantics`: semantic authority for rewrite legality.
/// - `budget`: optional operational work-count budget.
///
/// # Behavior
///
/// 1. Resolve and validate the root expression.
/// 2. Traverse its reachable graph iteratively, without recursive Rust calls.
/// 3. Process expression children before their parent.
/// 4. Preserve canonical child order when calling the semantic adapter.
/// 5. Leave non-expression nodes unchanged.
/// 6. Reject missing nodes and cycles instead of silently skipping them.
/// 7. Verify that each proposed replacement resolves in the same AST graph.
/// 8. Return a deterministic report without mutating the AST.
///
/// The pass is structural: it does not execute the program. The resolver is
/// expected to provide a stable view of one AST graph for the duration of the
/// call.
///
/// # Errors
///
/// Returns a structured error for an invalid root, missing node, malformed
/// expression, cyclic graph, semantic failure, invalid replacement, or
/// exhausted configured budget.
///
/// # Scalability
///
/// No fixed language-level graph-size or nesting-depth limit is imposed.
pub fn simplify_expression_graph<'a, Resolve, Semantics>(
    root: NodeId,
    mut resolve: Resolve,
    semantics: &Semantics,
    budget: SimplificationBudget,
) -> SimplificationResult<SimplificationReport, Semantics::Error>
where
    Resolve: FnMut(NodeId) -> Option<ResolvedSimplificationNode<'a>>,
    Semantics: SimplificationSemantics,
{
    match resolve(root) {
        Some(ResolvedSimplificationNode::Expression(_)) => {}
        Some(ResolvedSimplificationNode::Other) | None => {
            return Err(SimplificationError::InvalidRoot { id: root });
        }
    }

    let mut pending = vec![WorkItem::Enter {
        id: root,
        parent: None,
    }];
    let mut states = BTreeMap::<NodeId, VisitState>::new();
    let mut active = HashSet::<NodeId>::new();
    let mut records = BTreeMap::<NodeId, SimplificationRecord>::new();
    let mut consumed = 0usize;

    while let Some(item) = pending.pop() {
        match item {
            WorkItem::Enter { id, parent } => {
                if states.get(&id) == Some(&VisitState::Complete) {
                    continue;
                }

                if active.contains(&id) {
                    return Err(SimplificationError::CyclicExpressionGraph { id });
                }

                budget
                    .consume(&mut consumed)
                    .map_err(budget_error::<Semantics::Error>)?;

                let resolved = resolve(id).ok_or(
                    SimplificationError::MissingNode { parent, id },
                )?;

                match resolved {
                    ResolvedSimplificationNode::Other => {
                        records.insert(
                            id,
                            SimplificationRecord {
                                original: id,
                                simplified: id,
                                reason: SimplificationReason::Unchanged,
                            },
                        );
                        states.insert(id, VisitState::Complete);
                    }

                    ResolvedSimplificationNode::Expression(expression) => {
                        expression
                            .validate_structure(
                                ExpressionValidationPolicy::unrestricted(),
                            )
                            .map_err(|source| {
                                SimplificationError::InvalidExpression {
                                    id,
                                    source,
                                }
                            })?;

                        states.insert(id, VisitState::Active);
                        active.insert(id);

                        pending.push(WorkItem::Exit { id });

                        // Push in reverse so LIFO traversal processes children
                        // in canonical source order.
                        let children = expression.child_node_ids();
                        for child in children.into_iter().rev() {
                            pending.push(WorkItem::Enter {
                                id: child,
                                parent: Some(id),
                            });
                        }
                    }
                }
            }

            WorkItem::Exit { id } => {
                let expression = match resolve(id) {
                    Some(ResolvedSimplificationNode::Expression(expression)) => {
                        expression
                    }
                    Some(ResolvedSimplificationNode::Other) | None => {
                        return Err(SimplificationError::MissingNode {
                            parent: None,
                            id,
                        });
                    }
                };

                let child_ids = expression.child_node_ids();
                let mut children = Vec::with_capacity(child_ids.len());

                for child_id in child_ids {
                    let record = records.get(&child_id).ok_or(
                        SimplificationError::MissingNode {
                            parent: Some(id),
                            id: child_id,
                        },
                    )?;

                    children.push(SimplifiedChild {
                        original: child_id,
                        simplified: record.simplified,
                    });
                }

                let proposed = semantics
                    .simplify(expression, &children)
                    .map_err(SimplificationError::Semantic)?;

                let simplified = match proposed {
                    None => id,
                    Some(replacement) => {
                        if resolve(replacement).is_none() {
                            return Err(SimplificationError::InvalidReplacement {
                                expression: id,
                                replacement,
                            });
                        }

                        // If the replacement was already processed in this
                        // pass, use its known representative. Otherwise retain
                        // the canonical replacement identity; the caller can
                        // run a subsequent pass under an explicit policy.
                        records
                            .get(&replacement)
                            .map(|record| record.simplified)
                            .unwrap_or(replacement)
                    }
                };

                let reason = if simplified == id {
                    SimplificationReason::Unchanged
                } else {
                    SimplificationReason::Rewritten {
                        from: id,
                        to: simplified,
                    }
                };

                records.insert(
                    id,
                    SimplificationRecord {
                        original: id,
                        simplified,
                        reason,
                    },
                );

                active.remove(&id);
                states.insert(id, VisitState::Complete);
            }
        }
    }

    let rewrite_count = records
        .values()
        .filter(|record| record.was_rewritten())
        .count();

    Ok(SimplificationReport {
        root,
        visited_nodes: records.len(),
        records,
        rewrite_count,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct NoRewrite;

    impl SimplificationSemantics for NoRewrite {
        type Error = std::convert::Infallible;

        fn simplify(
            &self,
            _expression: &Expression,
            _children: &[SimplifiedChild],
        ) -> Result<Option<NodeId>, Self::Error> {
            Ok(None)
        }
    }

    #[derive(Debug)]
    struct ReplaceWithChild {
        child: NodeId,
    }

    impl SimplificationSemantics for ReplaceWithChild {
        type Error = std::convert::Infallible;

        fn simplify(
            &self,
            expression: &Expression,
            _children: &[SimplifiedChild],
        ) -> Result<Option<NodeId>, Self::Error> {
            if expression.id() == node_id(1) {
                Ok(Some(self.child))
            } else {
                Ok(None)
            }
        }
    }

    fn node_id(raw: u64) -> NodeId {
        NodeId::new(raw).expect("test node IDs must be non-zero")
    }

    fn test_expression(id: u64, child: NodeId) -> Expression {
        use crate::frontend::ast::node::expressions::expression::ExpressionKind;
        use crate::frontend::ast::node::source::Span;

        Expression::without_metadata(
            node_id(id),
            ExpressionKind::Identifier { identifier: child },
            Span::default(),
        )
    }

    #[test]
    fn unrestricted_budget_accounts_for_work() {
        let mut consumed = 0;

        assert!(
            SimplificationBudget::unrestricted()
                .consume(&mut consumed)
                .is_ok()
        );
        assert_eq!(consumed, 1);
    }

    #[test]
    fn explicit_zero_budget_rejects_work() {
        let mut consumed = 0;

        let error = SimplificationBudget::with_max_work_units(0)
            .consume(&mut consumed)
            .expect_err("zero budget must reject the first unit");

        assert_eq!(
            error,
            SimplificationBudgetError::WorkBudgetExceeded {
                consumed: 0,
                budget: 0,
            }
        );
    }

    #[test]
    fn unchanged_expression_produces_a_report() {
        let root = node_id(1);
        let child = node_id(2);
        let expression = test_expression(1, child);

        let report = simplify_expression_graph(
            root,
            |id| {
                if id == root {
                    Some(ResolvedSimplificationNode::Expression(&expression))
                } else if id == child {
                    Some(ResolvedSimplificationNode::Other)
                } else {
                    None
                }
            },
            &NoRewrite,
            SimplificationBudget::unrestricted(),
        )
        .expect("valid graph should simplify successfully");

        assert_eq!(report.root(), root);
        assert_eq!(report.simplified_root(), root);
        assert_eq!(report.rewrite_count(), 0);
        assert!(report.is_unchanged());
        assert_eq!(report.visited_nodes(), 2);
    }

    #[test]
    fn semantic_adapter_can_select_an_existing_node() {
        let root = node_id(1);
        let child = node_id(2);
        let expression = test_expression(1, child);

        let report = simplify_expression_graph(
            root,
            |id| {
                if id == root {
                    Some(ResolvedSimplificationNode::Expression(&expression))
                } else if id == child {
                    Some(ResolvedSimplificationNode::Other)
                } else {
                    None
                }
            },
            &ReplaceWithChild { child },
            SimplificationBudget::unrestricted(),
        )
        .expect("existing replacement should be accepted");

        assert_eq!(report.simplified_root(), child);
        assert_eq!(report.rewrite_count(), 1);

        let record = report.record(root).expect("root record must exist");
        assert_eq!(record.original, root);
        assert_eq!(record.simplified, child);
        assert!(record.was_rewritten());
    }

    #[test]
    fn unresolved_replacement_is_rejected() {
        let root = node_id(1);
        let child = node_id(2);
        let missing = node_id(3);
        let expression = test_expression(1, child);

        let error = simplify_expression_graph(
            root,
            |id| {
                if id == root {
                    Some(ResolvedSimplificationNode::Expression(&expression))
                } else if id == child {
                    Some(ResolvedSimplificationNode::Other)
                } else {
                    None
                }
            },
            &ReplaceWithChild { child: missing },
            SimplificationBudget::unrestricted(),
        )
        .expect_err("missing replacement must be rejected");

        assert!(matches!(
            error,
            SimplificationError::InvalidReplacement {
                expression,
                replacement,
            } if expression == root && replacement == missing
        ));
    }
}
