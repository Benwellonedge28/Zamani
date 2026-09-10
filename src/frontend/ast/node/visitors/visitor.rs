//! # Zamani Frontend AST — Canonical Immutable Visitor Contract
//!
//! `src/frontend/ast/node/visitors/visitor.rs`
//!
//! This module defines the immutable visitor protocol for the native Zamani
//! source AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ├───────────────┐
//!      │               │
//!      ▼               ▼
//! validation       immutable visitor
//!                      │
//!                      ▼
//!                    walk.rs
//!                      │
//!                      ▼
//!               semantic analysis
//!                      │
//!                      ▼
//!                 semantic model
//!                      │
//!                      ▼
//!                     ZUIR
//!                      │
//!          ┌───────────┼───────────┐
//!          ▼           ▼           ▼
//!      classical     quantum       HDL
//!         IR           IR           IR
//! ```
//!
//! ## Ownership
//!
//! This file owns only the immutable visitor protocol:
//!
//! - [`VisitControl`];
//! - [`AstVisitor`];
//! - visitor entry/exit semantics;
//! - core-node dispatch hooks;
//! - extension-node dispatch hooks;
//! - visitor error propagation;
//! - traversal-depth observation.
//!
//! This file does **not** own:
//!
//! - concrete AST nodes;
//! - AST storage;
//! - child relationships;
//! - traversal order;
//! - traversal algorithms;
//! - semantic analysis;
//! - type checking;
//! - resource allocation;
//! - quantum compilation;
//! - quantum routing;
//! - quantum scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - backend selection;
//! - hardware mapping;
//! - runtime execution;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! ## Core invariant
//!
//! The visitor observes source-level AST structure only.
//!
//! It must never require knowledge of the eventual execution target.
//!
//! In particular, this module contains no assumptions about:
//!
//! - machine size;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - physical qubit mapping;
//! - scheduler;
//! - calibration;
//! - QEC implementation;
//! - computational-domain cardinality.
//!
//! This preserves Zamani's:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! invariant.
//!
//! ## Visitor versus walker
//!
//! A visitor answers:
//!
//! ```text
//! "What should happen when this node is encountered?"
//! ```
//!
//! The walker answers:
//!
//! ```text
//! "Which nodes are encountered, and in what order?"
//! ```
//!
//! Therefore:
//!
//! - this file must not enumerate child relationships;
//! - this file must not recurse through the AST;
//! - this file must not own traversal storage;
//! - `walk.rs` owns structural traversal.
//!
//! The current walker is intentionally iterative, which allows traversal of
//! deeply nested ASTs without consuming one Rust stack frame per AST level.
//!
//! ## Node integration
//!
//! [`Node`] is the canonical source-level node metadata container. It exposes:
//!
//! - stable [`NodeId`](super::super::node_id::NodeId);
//! - [`NodeKind`](super::super::node_kind::NodeKind);
//! - source [`Span`](super::super::source::Span);
//! - [`NodeMetadata`](super::super::metadata::NodeMetadata).
//!
//! This visitor therefore receives `&Node` directly instead of introducing a
//! second AST-node abstraction.
//!
//! ## Node-kind integration
//!
//! [`NodeKind`](super::super::node_kind::NodeKind) has two architectural forms:
//!
//! ```text
//! NodeKind
//! ├── Core(...)
//! └── Extension(...)
//! ```
//!
//! The visitor exposes separate hooks for these categories without making the
//! visitor a closed enumeration of quantum gates, hardware devices, AI models,
//! accelerator instructions, or future technologies.
//!
//! A new extension can therefore be observed through [`AstVisitor::enter_extension`]
//! and [`AstVisitor::exit_extension`] without changing this trait.
//!
//! ## Quantum neutrality
//!
//! Quantum computation is supported through the same source-level visitor
//! protocol as classical, hybrid, HDL, accelerator, distributed, and future
//! computational constructs.
//!
//! This file intentionally does not contain hooks such as:
//!
//! ```text
//! visit_qubit
//! visit_h_gate
//! visit_cnot
//! visit_surface_code
//! visit_qpu
//! visit_ibm_backend
//! ```
//!
//! Such hooks would make the native AST visitor a closed quantum/backend API.
//!
//! Quantum-specific visitors can instead inspect namespaced extension kinds or
//! generic source-level resource/operation nodes.
//!
//! ## Error model
//!
//! Every visitor callback returns:
//!
//! ```text
//! Result<VisitControl, V::Error>
//! ```
//!
//! The visitor owns the error type.
//!
//! The AST subsystem therefore does not force all consumers to use one global
//! diagnostic/error representation.
//!
//! The walker is responsible for adapting visitor failures into its traversal
//! error type.
//!
//! ## Control model
//!
//! [`VisitControl`] provides three structural decisions:
//!
//! - [`VisitControl::Continue`] — descend into children;
//! - [`VisitControl::SkipChildren`] — observe the node but do not descend;
//! - [`VisitControl::Stop`] — terminate traversal successfully.
//!
//! `Stop` is a successful early-termination request, not a visitor failure.
//!
//! ## Depth
//!
//! The visitor receives a logical traversal depth.
//!
//! This value is informational.
//!
//! It is **not** a language-level maximum.
//!
//! A caller may impose a configurable traversal/resource policy externally.
//! Such a policy must never be encoded as a fixed language limitation here.
//!
//! ## Determinism
//!
//! This protocol contains no:
//!
//! - global mutable state;
//! - random state;
//! - wall-clock state;
//! - process state;
//! - thread-local traversal state;
//! - memory-address identity;
//! - hardware state.
//!
//! Traversal order is exclusively owned by the walker.
//!
//! Given the same AST and deterministic walker/provider, a visitor observes the
//! same node-event ordering.
//!
//! ## Parallel use
//!
//! The visitor protocol does not require global state.
//!
//! Independent visitor instances may therefore be used by independent compiler
//! tasks. Whether a concrete visitor is `Send` or `Sync` remains determined by
//! that visitor's own state.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.
//!
//! The module explicitly forbids unsafe code.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! ## Scalability
//!
//! This protocol imposes no semantic limit on:
//!
//! - AST size;
//! - number of nodes;
//! - number of declarations;
//! - number of expressions;
//! - number of statements;
//! - number of resources;
//! - number of quantum resources;
//! - number of operations;
//! - program nesting depth;
//! - number of modules;
//! - number of computational domains.
//!
//! Operational limits belong to the caller-controlled walker/compiler policy.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   ▼
//! Native AST
//!   │
//!   ▼
//! walk.rs
//!   │
//!   ▼
//! AstVisitor
//!   │
//!   ├── validation
//!   ├── diagnostics
//!   ├── tooling
//!   ├── source analysis
//!   └── semantic preparation
//!          │
//!          ▼
//!     semantic model
//!          │
//!          ▼
//!         ZUIR
//! ```
//!
//! This module must remain below semantic analysis and ZUIR in the dependency
//! graph.
//!
//! ```text
//! visitor.rs
//!     ✗→ semantic
//!     ✗→ ZUIR
//!     ✗→ quantum::ir
//!     ✗→ hardware
//!     ✗→ optimizer
//!     ✗→ scheduler
//!     ✗→ router
//!     ✗→ runtime
//!     ✗→ backend
//! ```
//!
//! ## File completion contract
//!
//! Once this file is implemented, adding a new AST declaration, expression,
//! statement, resource, domain, quantum technology, backend, or hardware target
//! must not require modifying this visitor protocol solely because the new
//! construct exists.
//!
//! The structural walker may need a change to describe new child relationships,
//! but that is deliberately outside this file's ownership.
//!
//! This separation allows this file's public contract to remain stable.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use super::super::node::Node;
use super::super::node_kind::NodeKind;

// =============================================================================
// Visit control
// =============================================================================

/// Controls the next structural traversal action after a visitor callback.
///
/// This type is deliberately independent of concrete AST node kinds.
///
/// It therefore remains stable as Zamani grows to support new language
/// constructs and computational domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VisitControl {
    /// Continue into the current node's direct children.
    Continue,

    /// Observe the current node but do not visit its direct children.
    SkipChildren,

    /// Stop traversal successfully.
    ///
    /// This is an early-completion request, not an error.
    Stop,
}

impl VisitControl {
    /// Returns `true` when traversal should continue into children.
    #[inline]
    #[must_use]
    pub const fn should_continue(self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns `true` when the current node's children should be skipped.
    #[inline]
    #[must_use]
    pub const fn should_skip_children(self) -> bool {
        matches!(self, Self::SkipChildren)
    }

    /// Returns `true` when traversal should terminate successfully.
    #[inline]
    #[must_use]
    pub const fn should_stop(self) -> bool {
        matches!(self, Self::Stop)
    }
}

impl Default for VisitControl {
    #[inline]
    fn default() -> Self {
        Self::Continue
    }
}

// =============================================================================
// Immutable visitor
// =============================================================================

/// Canonical immutable visitor for the native Zamani AST.
///
/// The trait intentionally exposes only source-level information.
///
/// Implementations can use this protocol for:
///
/// - structural validation;
/// - source analysis;
/// - diagnostics;
/// - symbol collection;
/// - documentation tooling;
/// - dependency analysis;
/// - metrics;
/// - semantic-analysis preparation;
/// - compiler tooling.
///
/// The trait does not encode quantum, hardware, backend, or target semantics.
///
/// # Required callback contract
///
/// [`AstVisitor::enter`] is called exactly once when the walker enters a node.
///
/// If it returns [`VisitControl::Continue`], the walker may visit the node's
/// children.
///
/// If it returns [`VisitControl::SkipChildren`], the walker must not descend
/// into that node's children.
///
/// If it returns [`VisitControl::Stop`], the walker must terminate successfully.
///
/// [`AstVisitor::exit`] is called after the node's descendants have completed,
/// unless traversal terminates before that point.
///
/// A visitor error terminates traversal immediately.
///
/// # Extension contract
///
/// The default [`AstVisitor::enter`] implementation dispatches to:
///
/// - [`AstVisitor::enter_core`] for native core nodes;
/// - [`AstVisitor::enter_extension`] for extension nodes.
///
/// Likewise, [`AstVisitor::exit`] dispatches to the corresponding exit hooks.
///
/// This permits extension-aware visitors without a giant closed visitor enum.
///
/// # Important
///
/// Implementations overriding `enter` or `exit` directly take responsibility
/// for preserving any desired core/extension dispatch behavior.
pub trait AstVisitor {
    /// Error produced by this visitor.
    ///
    /// The visitor protocol deliberately does not impose a repository-wide
    /// error type.
    type Error;

    /// Called when traversal enters a node.
    ///
    /// The default implementation dispatches to the appropriate core or
    /// extension hook.
    ///
    /// `depth` is the logical structural depth supplied by the walker.
    fn enter(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        match node.kind() {
            NodeKind::Core(kind) => self.enter_core(node, *kind, depth),

            NodeKind::Extension(_) => {
                self.enter_extension(node, node.kind(), depth)
            }
        }
    }

    /// Called after all permitted descendants of a node have completed.
    ///
    /// The default implementation dispatches to the appropriate core or
    /// extension hook.
    fn exit(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<(), Self::Error> {
        match node.kind() {
            NodeKind::Core(kind) => self.exit_core(node, *kind, depth),

            NodeKind::Extension(_) => {
                self.exit_extension(node, node.kind(), depth)
            }
        }
    }

    /// Called when a native Zamani core node is entered.
    ///
    /// The default behavior is to continue traversal.
    ///
    /// `CoreNodeKind` remains intentionally coarse. Concrete computational
    /// technologies must not be added here merely because they are new.
    fn enter_core(
        &mut self,
        _node: &Node,
        _kind: super::super::node_kind::CoreNodeKind,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called when a native Zamani core node is exited.
    ///
    /// The default behavior succeeds without changing traversal state.
    fn exit_core(
        &mut self,
        _node: &Node,
        _kind: super::super::node_kind::CoreNodeKind,
        _depth: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when an extension node is entered.
    ///
    /// The entire [`NodeKind`] is supplied instead of introducing a
    /// technology-specific extension trait into the AST core.
    ///
    /// Consumers may inspect the namespaced extension identity through the
    /// existing `NodeKind` API.
    ///
    /// Unknown extensions are therefore safe to observe by default.
    fn enter_extension(
        &mut self,
        _node: &Node,
        _kind: &NodeKind,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called when an extension node is exited.
    ///
    /// The default behavior succeeds without changing traversal state.
    fn exit_extension(
        &mut self,
        _node: &Node,
        _kind: &NodeKind,
        _depth: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Returns whether this visitor requests cancellation.
    ///
    /// This is deliberately a visitor-level hook rather than a dependency on
    /// an asynchronous runtime, scheduler, or repository-wide cancellation
    /// implementation.
    ///
    /// The default implementation never requests cancellation.
    ///
    /// A walker may choose to call this at traversal boundaries.
    #[inline]
    fn should_cancel(&self) -> bool {
        false
    }
}

// =============================================================================
// Convenience visitor implementations
// =============================================================================

/// Visitor that performs no work.
///
/// This is useful for callers that need to validate traversal infrastructure
/// independently from analysis logic.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopVisitor;

impl AstVisitor for NoopVisitor {
    type Error = core::convert::Infallible;
}

// =============================================================================
// Closure adapter
// =============================================================================

/// Convenience immutable visitor backed by caller-provided closures.
///
/// This allows small tooling passes to avoid declaring a named visitor type
/// while retaining the canonical visitor protocol.
///
/// The adapter deliberately remains source-level and domain-neutral.
///
/// # Example
///
/// ```ignore
/// let mut visitor = FnVisitor::new(
///     |node, depth| {
///         // inspect node
///         let _ = (node, depth);
///         Ok::<_, MyError>(VisitControl::Continue)
///     },
///     |node, depth| {
///         let _ = (node, depth);
///         Ok::<_, MyError>(())
///     },
/// );
///
/// walk(&provider, root, &mut visitor)?;
/// ```
pub struct FnVisitor<Enter, Exit> {
    enter: Enter,
    exit: Exit,
}

impl<Enter, Exit> FnVisitor<Enter, Exit> {
    /// Creates a closure-backed visitor.
    #[must_use]
    pub const fn new(enter: Enter, exit: Exit) -> Self {
        Self { enter, exit }
    }

    /// Returns the enter callback.
    #[inline]
    pub fn enter_callback(&self) -> &Enter {
        &self.enter
    }

    /// Returns the exit callback.
    #[inline]
    pub fn exit_callback(&self) -> &Exit {
        &self.exit
    }

    /// Returns mutable access to the enter callback.
    #[inline]
    pub fn enter_callback_mut(&mut self) -> &mut Enter {
        &mut self.enter
    }

    /// Returns mutable access to the exit callback.
    #[inline]
    pub fn exit_callback_mut(&mut self) -> &mut Exit {
        &mut self.exit
    }
}

impl<Enter, Exit, E> AstVisitor for FnVisitor<Enter, Exit>
where
    Enter: FnMut(&Node, usize) -> Result<VisitControl, E>,
    Exit: FnMut(&Node, usize) -> Result<(), E>,
{
    type Error = E;

    #[inline]
    fn enter(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        (self.enter)(node, depth)
    }

    #[inline]
    fn exit(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<(), Self::Error> {
        (self.exit)(node, depth)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestError {
        Failure,
    }

    #[test]
    fn visit_control_defaults_to_continue() {
        assert_eq!(
            VisitControl::default(),
            VisitControl::Continue
        );
    }

    #[test]
    fn visit_control_predicates_are_exclusive() {
        assert!(VisitControl::Continue.should_continue());
        assert!(!VisitControl::Continue.should_skip_children());
        assert!(!VisitControl::Continue.should_stop());

        assert!(!VisitControl::SkipChildren.should_continue());
        assert!(VisitControl::SkipChildren.should_skip_children());
        assert!(!VisitControl::SkipChildren.should_stop());

        assert!(!VisitControl::Stop.should_continue());
        assert!(!VisitControl::Stop.should_skip_children());
        assert!(VisitControl::Stop.should_stop());
    }

    #[test]
    fn noop_visitor_has_infallible_error_type() {
        let mut visitor = NoopVisitor;
        let node = Node::default();

        let enter_result = visitor.enter(&node, 0);
        assert_eq!(enter_result, Ok(VisitControl::Continue));

        let exit_result = visitor.exit(&node, 0);
        assert_eq!(exit_result, Ok(()));
    }

    #[test]
    fn cancellation_defaults_to_false() {
        let visitor = NoopVisitor;
        assert!(!visitor.should_cancel());
    }

    #[test]
    fn fn_visitor_preserves_enter_and_exit_results() {
        let mut visitor = FnVisitor::new(
            |_node, _depth| Ok::<_, TestError>(VisitControl::SkipChildren),
            |_node, _depth| Ok::<_, TestError>(()),
        );

        let node = Node::default();

        assert_eq!(
            visitor.enter(&node, 4),
            Ok(VisitControl::SkipChildren)
        );

        assert_eq!(visitor.exit(&node, 4), Ok(()));
    }

    #[test]
    fn fn_visitor_preserves_errors() {
        let mut visitor = FnVisitor::new(
            |_node, _depth| Err::<VisitControl, _>(TestError::Failure),
            |_node, _depth| Err::<(), _>(TestError::Failure),
        );

        let node = Node::default();

        assert_eq!(
            visitor.enter(&node, 0),
            Err(TestError::Failure)
        );

        assert_eq!(
            visitor.exit(&node, 0),
            Err(TestError::Failure)
        );
    }

    #[test]
    fn fn_visitor_callbacks_are_mutably_accessible() {
        let mut visitor = FnVisitor::new(
            |_node, _depth| Ok::<_, TestError>(VisitControl::Continue),
            |_node, _depth| Ok::<_, TestError>(()),
        );

        let _ = visitor.enter_callback_mut();
        let _ = visitor.exit_callback_mut();
    }
}