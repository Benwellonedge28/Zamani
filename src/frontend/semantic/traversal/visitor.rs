
#![forbid(unsafe_code)]

//! # Zamani Semantic Analysis — AST Visitor Contract
//!
//! Path: `src/frontend/semantic/traversal/visitor.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the semantic-layer visitor contract over the canonical Zamani AST.
//!
//! This module adapts semantic callbacks to the existing immutable AST visitor.
//! It does not implement a second AST visitor protocol or a traversal algorithm.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/ast/node/visitors/visitor.rs`
//! - `src/frontend/ast/node/traversal/walk.rs`
//! - `src/frontend/ast/node/traversal/children.rs`
//!
//! The grammar defines syntax. The semantic specifications define meaning.
//! The canonical AST owns node structure, identities, kinds, and source spans.
//!
//! ## OWNERSHIP
//!
//! This file owns:
//! - the semantic visitor callback contract;
//! - the adapter between semantic callbacks and the canonical AST visitor;
//! - access to the wrapped semantic visitor;
//! - contract tests for callback forwarding.
//!
//! This file does not own:
//! - AST node or identity definitions;
//! - child enumeration;
//! - traversal algorithms or traversal storage;
//! - semantic facts, symbol tables, or type inference;
//! - diagnostics or analysis budgets;
//! - domain-specific semantic rules;
//! - target feasibility or hardware allocation;
//! - classical, quantum, or HDL IR.
//!
//! ## INTEGRATION
//!
//! `traversal/mod.rs` declares and re-exports this module.
//! The canonical AST walker consumes its existing `AstVisitor` contract.
//! Semantic passes implement `SemanticVisitor` and use `SemanticVisitorAdapter`
//! when invoking that walker.
//!
//! The adapter does not assume a concrete AST storage implementation.
//! A caller must provide the existing walker's required node/child provider.
//!
//! ## SCALABILITY
//!
//! This module introduces no fixed AST-size, nesting-depth, resource-count,
//! device-count, or computational-domain ceiling. It does not recurse through
//! the AST. Operational budgets, cancellation checks, cycle detection, and
//! traversal limits remain the responsibility of the canonical walker.
//!
//! No finite implementation can process an actually infinite input. The
//! architecture must avoid arbitrary language-level ceilings and report
//! operational exhaustion explicitly.
//!
//! ## DETERMINISM
//!
//! This adapter preserves callback order and results. Traversal order is
//! defined by the canonical walker and child provider, not by this module.
//!
//! ## ERROR HANDLING
//!
//! Semantic visitor errors are forwarded unchanged. They are not converted to
//! success, suppressed, or replaced with target-specific diagnostics.
//!
//! ## FREEZE CRITERIA
//!
//! - The semantic visitor does not redefine AST types.
//! - The adapter implements the canonical `AstVisitor` contract.
//! - Callback results, errors, and cancellation are forwarded.
//! - Core and extension dispatch remain owned by the AST visitor.
//! - Unit tests pass with the repository's canonical AST types.

use crate::frontend::ast::node::node::Node;
use crate::frontend::ast::node::visitors::visitor::{
    AstVisitor, VisitControl,
};

/// Semantic-layer visitor over canonical AST nodes.
///
/// Implementations inspect source-level nodes and perform semantic-pass work.
/// They must not assume that a node belongs to a fixed set of computational
/// domains or that the program will run on a particular target.
///
/// This trait deliberately receives the canonical [`Node`] rather than
/// defining a semantic copy of an AST node.
///
/// # Callback contract
///
/// The walker invokes `enter_semantic` when it enters a node. Its result
/// determines whether the walk continues into children, skips the current
/// node's children, or stops successfully.
///
/// `exit_semantic` is invoked after the permitted descendants have completed,
/// subject to the canonical walker's early-stop and error behavior.
///
/// `depth` is the logical AST depth reported by the walker. It is not a
/// machine-capacity limit.
///
/// # Error contract
///
/// The associated error type is unconstrained. A semantic pass may use its
/// own structured error type without forcing all passes to share one.
///
/// # Extension contract
///
/// The adapter below forwards nodes through the canonical AST visitor's
/// default dispatch. Core nodes and namespaced extension nodes therefore
/// follow the same semantic callback contract.
pub trait SemanticVisitor {
    /// Error returned by semantic callbacks.
    type Error;

    /// Called when semantic traversal enters a node.
    ///
    /// Return [`VisitControl::Continue`] to visit children,
    /// [`VisitControl::SkipChildren`] to skip children, or
    /// [`VisitControl::Stop`] to stop successfully.
    fn enter_semantic(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error>;

    /// Called after the permitted descendants of a node have completed.
    ///
    /// The canonical walker controls whether this callback occurs after
    /// cancellation, early termination, or an error.
    fn exit_semantic(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<(), Self::Error>;

    /// Indicates whether the visitor requests traversal cancellation.
    ///
    /// The walker determines when cancellation is checked. This method must
    /// be inexpensive and must not mutate the AST.
    ///
    /// The default is `false`.
    #[inline]
    fn should_cancel(&self) -> bool {
        false
    }
}

/// Adapter from a semantic visitor to the canonical AST visitor protocol.
///
/// This type allows the existing AST walker to drive semantic callbacks
/// without duplicating its dispatch, traversal, cycle detection, or limits.
///
/// The adapter owns the visitor value, not the AST or the traversal state.
#[derive(Debug, Clone)]
pub struct SemanticVisitorAdapter<V> {
    visitor: V,
}

impl<V> SemanticVisitorAdapter<V> {
    /// Wraps a semantic visitor for use with the canonical AST walker.
    #[must_use]
    pub const fn new(visitor: V) -> Self {
        Self { visitor }
    }

    /// Borrows the wrapped semantic visitor.
    #[inline]
    #[must_use]
    pub const fn visitor(&self) -> &V {
        &self.visitor
    }

    /// Mutably borrows the wrapped semantic visitor.
    #[inline]
    pub fn visitor_mut(&mut self) -> &mut V {
        &mut self.visitor
    }

    /// Recovers the wrapped semantic visitor.
    #[must_use]
    pub fn into_inner(self) -> V {
        self.visitor
    }
}

impl<V: SemanticVisitor> AstVisitor for SemanticVisitorAdapter<V> {
    type Error = V::Error;

    #[inline]
    fn enter(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        self.visitor.enter_semantic(node, depth)
    }

    #[inline]
    fn exit(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<(), Self::Error> {
        self.visitor.exit_semantic(node, depth)
    }

    #[inline]
    fn should_cancel(&self) -> bool {
        self.visitor.should_cancel()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::Infallible;

    #[derive(Debug, Default)]
    struct RecordingVisitor {
        entered: Vec<usize>,
        exited: Vec<usize>,
        cancel: bool,
    }

    impl SemanticVisitor for RecordingVisitor {
        type Error = Infallible;

        fn enter_semantic(
            &mut self,
            _node: &Node,
            depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.entered.push(depth);
            Ok(VisitControl::Continue)
        }

        fn exit_semantic(
            &mut self,
            _node: &Node,
            depth: usize,
        ) -> Result<(), Self::Error> {
            self.exited.push(depth);
            Ok(())
        }

        fn should_cancel(&self) -> bool {
            self.cancel
        }
    }

    #[test]
    fn adapter_forwards_enter_and_exit_callbacks() {
        let node = Node::default();
        let visitor = RecordingVisitor::default();
        let mut adapter = SemanticVisitorAdapter::new(visitor);

        assert_eq!(
            adapter.enter(&node, 3),
            Ok(VisitControl::Continue)
        );
        assert_eq!(adapter.exit(&node, 3), Ok(()));

        let visitor = adapter.into_inner();
        assert_eq!(visitor.entered, vec![3]);
        assert_eq!(visitor.exited, vec![3]);
    }

    #[test]
    fn adapter_preserves_skip_children_and_stop_decisions() {
        struct DecisionVisitor(VisitControl);

        impl SemanticVisitor for DecisionVisitor {
            type Error = Infallible;

            fn enter_semantic(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<VisitControl, Self::Error> {
                Ok(self.0)
            }

            fn exit_semantic(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        let node = Node::default();

        for decision in [VisitControl::SkipChildren, VisitControl::Stop] {
            let mut adapter =
                SemanticVisitorAdapter::new(DecisionVisitor(decision));

            assert_eq!(adapter.enter(&node, 0), Ok(decision));
        }
    }

    #[test]
    fn adapter_preserves_visitor_errors() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct TestError;

        struct FailingVisitor;

        impl SemanticVisitor for FailingVisitor {
            type Error = TestError;

            fn enter_semantic(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<VisitControl, Self::Error> {
                Err(TestError)
            }

            fn exit_semantic(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<(), Self::Error> {
                Err(TestError)
            }
        }

        let node = Node::default();
        let mut adapter = SemanticVisitorAdapter::new(FailingVisitor);

        assert_eq!(adapter.enter(&node, 0), Err(TestError));
        assert_eq!(adapter.exit(&node, 0), Err(TestError));
    }

    #[test]
    fn adapter_forwards_cancellation_state() {
        let visitor = RecordingVisitor {
            cancel: true,
            ..RecordingVisitor::default()
        };
        let adapter = SemanticVisitorAdapter::new(visitor);

        assert!(adapter.should_cancel());
        assert!(adapter.visitor().should_cancel());
    }

    #[test]
    fn visitor_can_be_recovered_after_adaptation() {
        let visitor = RecordingVisitor::default();
        let adapter = SemanticVisitorAdapter::new(visitor);
        let recovered = adapter.into_inner();

        assert!(recovered.entered.is_empty());
        assert!(recovered.exited.is_empty());
    }
}
