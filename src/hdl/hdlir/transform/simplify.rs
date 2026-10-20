
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Conservative Expression Simplification
//!
//! Path: `src/hdl/hdlir/transform/simplify.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Compact an HDLIR expression graph to the nodes reachable from explicitly
//! supplied roots. Preserve expression structure, operand order, constants,
//! symbolic names, widths, casts, and opaque extension payloads.
//!
//! This is a conservative foundational transformation. It does not perform
//! algebraic rewrites, constant folding, or extension-defined optimization.
//! Such rewrites require authoritative operator semantics and explicit
//! preservation contracts that the current expression graph API does not
//! provide on its own.
//!
//! ## Authority and ownership
//!
//! The implementation follows:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars.
//! - `grammar/specification/semantic-model.md`.
//! - `src/hdl/hdlir/values/expressions.rs`.
//! - `src/hdl/hdlir/values/constants.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/types/logic.rs`.
//! - `src/hdl/hdlir/passes/contracts.rs`, when integrated with the pass
//!   manager's actual preservation and invalidation contracts.
//!
//! The grammar owns syntax; frontend semantics own source meaning;
//! the expression module owns expression representation; this module owns
//! reachability-based expression graph compaction.
//!
//! This module does not own the complete hardware design graph, operation
//! semantics, type inference, source spans, clocks, state transitions,
//! memory behavior, physical resources, or target realization.
//!
//! ## Semantics-preservation contract
//!
//! Given a valid expression graph and valid roots, the output contains exactly
//! the expression nodes reachable from those roots through the graph's
//! declared operands. Each retained node has the same expression kind and
//! ordered operands as its source node, modulo graph-local ID remapping.
//!
//! No retained expression is evaluated, reordered, coerced, or interpreted.
//! Unknown and high-impedance logic values remain unchanged. Extension
//! operations and their ordered attributes remain opaque and unchanged.
//!
//! The caller must supply every expression root that is externally observable
//! or required by the owning HDL construct. An omitted root is treated as
//! intentionally dead; this module cannot infer observability independently.
//!
//! ## Scalability
//!
//! There is no fixed expression-count or hardware-capacity ceiling. Work is
//! proportional to the reachable graph and the traversal storage required,
//! subject to host representation, available memory, and the source graph's
//! configured construction budget.
//!
//! Traversal is iterative through `ExprGraph::postorder`; expression depth
//! does not recursively consume this module's call stack. Graph-local IDs
//! are remapped explicitly and are not treated as persistent identities.
//!
//! Allocation failures reported by the existing expression graph API are
//! propagated. As with ordinary Rust collections, this module cannot promise
//! that every allocation made by every standard-library collection is
//! recoverable on every platform.
//!
//! ## Integration
//!
//! - `values/expressions.rs` provides the authoritative expression graph.
//! - `transform/mod.rs` exposes this module.
//! - A future pass manager may invoke this function after checking its pass
//!   preconditions and must invalidate/recompute dependent analyses afterward.
//! - Callers must preserve the returned graph and returned roots together.
//! - Design-level dead-code elimination must separately account for hardware
//!   connectivity, effects, state, clocks, memory, and other observable roots.
//!
//! ## Freeze criteria
//!
//! - Graph structure is validated before transformation.
//! - Invalid roots and graph errors return errors rather than being ignored.
//! - All returned roots reference nodes in the returned graph.
//! - Retained nodes preserve their original expression kinds and operand order.
//! - Output is deterministic for the same graph and ordered root list.
//! - Tests cover empty roots, shared subexpressions, dead nodes, malformed
//!   roots, and explicitly constrained output budgets.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::hdl::hdlir::values::expressions::{
    ExprError, ExprGraph, ExprId, ExprKind,
};

/// Result type for conservative expression simplification.
pub type SimplifyResult<T> = Result<T, SimplifyError>;

/// Errors returned by conservative expression graph compaction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SimplifyError {
    /// The source expression graph is invalid or traversal failed.
    ExpressionGraph(ExprError),

    /// A required operand was not mapped during graph reconstruction.
    ///
    /// A valid topologically ordered graph should never produce this error.
    MissingOperandMapping {
        /// The original graph-local operand identity.
        operand: ExprId,
    },

    /// An original root unexpectedly had no reconstructed identity.
    ///
    /// This indicates an internal transformation invariant violation.
    MissingRootMapping {
        /// The original graph-local root identity.
        root: ExprId,
    },
}

impl fmt::Display for SimplifyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionGraph(error) => {
                write!(formatter, "expression graph error: {error}")
            }
            Self::MissingOperandMapping { operand } => write!(
                formatter,
                "expression simplification could not remap operand {}",
                operand.index()
            ),
            Self::MissingRootMapping { root } => write!(
                formatter,
                "expression simplification could not remap root {}",
                root.index()
            ),
        }
    }
}

impl Error for SimplifyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ExpressionGraph(error) => Some(error),
            Self::MissingOperandMapping { .. }
            | Self::MissingRootMapping { .. } => None,
        }
    }
}

impl From<ExprError> for SimplifyError {
    fn from(error: ExprError) -> Self {
        Self::ExpressionGraph(error)
    }
}

/// Statistics for one expression graph compaction.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SimplifyStats {
    /// Number of nodes in the input graph.
    pub original_nodes: usize,

    /// Number of nodes retained in the output graph.
    pub retained_nodes: usize,

    /// Number of unreachable nodes omitted from the output graph.
    pub removed_nodes: usize,
}

impl SimplifyStats {
    /// Return whether any unreachable nodes were removed.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.removed_nodes != 0
    }
}

/// A compacted expression graph and its remapped roots.
#[derive(Clone, Debug)]
pub struct SimplifiedExpressions {
    graph: ExprGraph,
    roots: Vec<ExprId>,
    old_to_new: BTreeMap<ExprId, ExprId>,
    stats: SimplifyStats,
}

impl SimplifiedExpressions {
    /// Return the compacted expression graph.
    ///
    /// The graph is owned by this result. Its IDs are local to this graph.
    #[must_use]
    pub const fn graph(&self) -> &ExprGraph {
        &self.graph
    }

    /// Consume this result and return the compacted expression graph.
    #[must_use]
    pub fn into_graph(self) -> ExprGraph {
        self.graph
    }

    /// Return roots in the same order as the input roots.
    ///
    /// Repeated roots remain repeated. Every returned ID belongs to
    /// `graph()`.
    #[must_use]
    pub fn roots(&self) -> &[ExprId] {
        &self.roots
    }

    /// Return the original-to-compacted identity mapping for retained nodes.
    ///
    /// Unreachable nodes are deliberately absent from this map.
    #[must_use]
    pub fn old_to_new(&self) -> &BTreeMap<ExprId, ExprId> {
        &self.old_to_new
    }

    /// Return the transformation statistics.
    #[must_use]
    pub const fn stats(&self) -> SimplifyStats {
        self.stats
    }
}

/// Compact an expression graph to the nodes reachable from `roots`.
///
/// The input graph is not modified. The returned graph owns a fresh set of
/// graph-local IDs; callers must use the returned roots with that graph and
/// must not reuse the original IDs as if they referred to the same nodes.
///
/// Each root is traversed in caller-supplied order. Each reachable node is
/// reconstructed once, and shared subexpressions remain shared. The source
/// graph's configured construction budget is retained by the output graph.
///
/// An empty root list produces an empty graph. This is intentional: callers
/// define the observable roots, and this function does not guess which
/// expressions are externally observable.
///
/// # Errors
///
/// Returns an error if the source graph is invalid, a root is invalid, graph
/// traversal fails, or reconstruction encounters an invariant violation.
pub fn simplify_expressions(
    source: &ExprGraph,
    roots: &[ExprId],
) -> SimplifyResult<SimplifiedExpressions> {
    source.validate()?;

    let original_nodes = source.len();
    let mut output = ExprGraph::with_budget(source.budget());
    let mut old_to_new = BTreeMap::<ExprId, ExprId>::new();

    // Validate roots before building output. This ensures malformed roots
    // cannot produce a result that appears successful.
    for &root in roots {
        if source.get(root).is_none() {
            // Use the expression graph's canonical error representation.
            let _ = source.postorder(root)?;
        }
    }

    // `postorder` is iterative and returns operands before their users.
    // Mapping nodes as they are encountered means each operand is already
    // available when its user is reconstructed.
    for &root in roots {
        for old_id in source.postorder(root)? {
            if old_to_new.contains_key(&old_id) {
                continue;
            }

            let node = source.get(old_id).ok_or(
                SimplifyError::MissingRootMapping { root: old_id },
            )?;

            let new_kind = remap_kind(node.kind(), &old_to_new)?;
            let new_id = output.insert(new_kind)?;
            old_to_new.insert(old_id, new_id);
        }
    }

    let mut new_roots = Vec::new();
    new_roots
        .try_reserve(roots.len())
        .map_err(|_| ExprError::AllocationFailed)?;

    for &root in roots {
        let mapped = old_to_new
            .get(&root)
            .copied()
            .ok_or(SimplifyError::MissingRootMapping { root })?;
        new_roots.push(mapped);
    }

    output.validate()?;

    let retained_nodes = output.len();
    let removed_nodes = original_nodes
        .checked_sub(retained_nodes)
        .ok_or_else(|| {
            SimplifyError::ExpressionGraph(ExprError::InvalidGraph {
                node: None,
                reason: "compacted graph contains more nodes than the source graph",
            })
        })?;

    Ok(SimplifiedExpressions {
        graph: output,
        roots: new_roots,
        old_to_new,
        stats: SimplifyStats {
            original_nodes,
            retained_nodes,
            removed_nodes,
        },
    })
}

/// Rebuild an expression kind using the output graph's operand identities.
///
/// All semantic payloads are cloned unchanged. Only graph-local operand
/// identities are replaced. Extension operations are never interpreted.
fn remap_kind(
    kind: &ExprKind,
    mapping: &BTreeMap<ExprId, ExprId>,
) -> SimplifyResult<ExprKind> {
    let remap = |old: ExprId| {
        mapping
            .get(&old)
            .copied()
            .ok_or(SimplifyError::MissingOperandMapping { operand: old })
    };

    Ok(match kind {
        ExprKind::Constant(value) => ExprKind::Constant(value.clone()),

        ExprKind::Symbol(name) => ExprKind::Symbol(name.clone()),

        ExprKind::Unary { operator, operand } => ExprKind::Unary {
            operator: *operator,
            operand: remap(*operand)?,
        },

        ExprKind::Binary { operator, lhs, rhs } => ExprKind::Binary {
            operator: *operator,
            lhs: remap(*lhs)?,
            rhs: remap(*rhs)?,
        },

        ExprKind::Conditional {
            condition,
            then_value,
            else_value,
        } => ExprKind::Conditional {
            condition: remap(*condition)?,
            then_value: remap(*then_value)?,
            else_value: remap(*else_value)?,
        },

        ExprKind::Concat { operands } => {
            let mut mapped = Vec::new();
            mapped
                .try_reserve(operands.len())
                .map_err(|_| ExprError::AllocationFailed)?;

            for &operand in operands {
                mapped.push(remap(operand)?);
            }

            ExprKind::Concat { operands: mapped }
        }

        ExprKind::Extract {
            value,
            offset,
            width,
        } => ExprKind::Extract {
            value: remap(*value)?,
            offset: remap(*offset)?,
            width: width.clone(),
        },

        ExprKind::Cast {
            value,
            width,
            signedness,
        } => ExprKind::Cast {
            value: remap(*value)?,
            width: width.clone(),
            signedness: *signedness,
        },

        ExprKind::Extension {
            dialect,
            operation,
            operands,
            attributes,
        } => {
            let mut mapped_operands = Vec::new();
            mapped_operands
                .try_reserve(operands.len())
                .map_err(|_| ExprError::AllocationFailed)?;

            for &operand in operands {
                mapped_operands.push(remap(operand)?);
            }

            ExprKind::Extension {
                dialect: dialect.clone(),
                operation: operation.clone(),
                operands: mapped_operands,
                attributes: attributes.clone(),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::values::expressions::UnaryOperator;

    #[test]
    fn removes_unreachable_nodes_and_remaps_roots() {
        let mut source = ExprGraph::new();
        let live = source.symbol("live").expect("valid symbol");
        let result = source
            .insert(ExprKind::Unary {
                operator: UnaryOperator::LogicalNot,
                operand: live,
            })
            .expect("valid unary expression");
        let _dead = source.symbol("unreachable").expect("valid symbol");

        let compacted =
            simplify_expressions(&source, &[result]).expect("simplification succeeds");

        assert_eq!(compacted.stats().original_nodes, 3);
        assert_eq!(compacted.stats().retained_nodes, 2);
        assert_eq!(compacted.stats().removed_nodes, 1);
        assert_eq!(compacted.graph().len(), 2);
        assert_eq!(compacted.roots().len(), 1);
        assert!(compacted.graph().get(compacted.roots()[0]).is_some());
        assert_eq!(source.len(), 3, "the source graph remains unchanged");
    }

    #[test]
    fn preserves_shared_subexpressions() {
        let mut source = ExprGraph::new();
        let shared = source.symbol("shared").expect("valid symbol");
        let left = source
            .insert(ExprKind::Unary {
                operator: UnaryOperator::LogicalNot,
                operand: shared,
            })
            .expect("valid left expression");
        let right = source
            .insert(ExprKind::Unary {
                operator: UnaryOperator::LogicalNot,
                operand: shared,
            })
            .expect("valid right expression");

        let compacted =
            simplify_expressions(&source, &[left, right]).expect("simplification succeeds");

        assert_eq!(compacted.graph().len(), 3);
        assert_eq!(compacted.stats().removed_nodes, 0);

        let mapped_shared = compacted.old_to_new()[&shared];
        let mapped_left = compacted.old_to_new()[&left];
        let mapped_right = compacted.old_to_new()[&right];

        match compacted.graph().get(mapped_left).expect("left exists").kind() {
            ExprKind::Unary { operand, .. } => assert_eq!(*operand, mapped_shared),
            other => panic!("expected unary expression, found {other:?}"),
        }

        match compacted.graph().get(mapped_right).expect("right exists").kind() {
            ExprKind::Unary { operand, .. } => assert_eq!(*operand, mapped_shared),
            other => panic!("expected unary expression, found {other:?}"),
        }
    }

    #[test]
    fn empty_roots_produce_an_empty_graph() {
        let mut source = ExprGraph::new();
        source.symbol("unused").expect("valid symbol");

        let compacted =
            simplify_expressions(&source, &[]).expect("empty root set is valid");

        assert!(compacted.graph().is_empty());
        assert!(compacted.roots().is_empty());
        assert_eq!(compacted.stats().removed_nodes, 1);
    }

    #[test]
    fn preserves_duplicate_root_order() {
        let mut source = ExprGraph::new();
        let root = source.symbol("root").expect("valid symbol");

        let compacted =
            simplify_expressions(&source, &[root, root]).expect("simplification succeeds");

        assert_eq!(compacted.roots().len(), 2);
        assert_eq!(compacted.roots()[0], compacted.roots()[1]);
        assert_eq!(compacted.graph().len(), 1);
    }

    #[test]
    fn rejects_an_invalid_root() {
        let source = ExprGraph::new();

        assert!(simplify_expressions(&source, &[ExprId::from_test_only()]).is_err());
    }
}
