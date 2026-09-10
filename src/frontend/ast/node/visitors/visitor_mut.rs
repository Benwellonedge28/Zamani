//! # Zamani Frontend AST — Mutable Visitor Contract
//!
//! `src/frontend/ast/node/visitors/visitor_mut.rs`
//!
//! This module defines the canonical mutable visitor protocol for the native
//! Zamani Abstract Syntax Tree (AST).
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
//!      ├───────────────┬──────────────────┐
//!      │               │                  │
//!      ▼               ▼                  ▼
//! validation       immutable visitor   mutable visitor
//!                                         │
//!                                         ▼
//!                                  fold / transformation
//!                                         │
//!                                         ▼
//!                                  semantic analysis
//!                                         │
//!                                         ▼
//!                                        ZUIR
//! ```
//!
//! This file owns the **mutable visitor protocol** only.
//!
//! It does not own:
//!
//! - concrete AST node definitions;
//! - child relationships;
//! - traversal order;
//! - recursive/iterative traversal implementation;
//! - parser logic;
//! - semantic analysis;
//! - type checking;
//! - quantum compilation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - backend selection;
//! - hardware mapping;
//! - runtime execution;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! ## Core architectural invariant
//!
//! The native AST is source-oriented and domain-neutral.
//!
//! Mutable visitation therefore means:
//!
//! ```text
//! mutate source-level AST representation
//! ```
//!
//! and never:
//!
//! ```text
//! mutate hardware realization
//! mutate physical qubit mapping
//! mutate backend instructions
//! mutate scheduler state
//! mutate QIR
//! mutate runtime state
//! ```
//!
//! This preserves:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! The same mutable visitor protocol can operate on an AST representing:
//!
//! - tiny programs;
//! - very large programs;
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - HDL-oriented source;
//! - future computational domains.
//!
//! No machine-size assumption is encoded here.
//!
//! ## Why mutation is separated from traversal
//!
//! A mutable visitor answers:
//!
//! ```text
//! "What source-level mutation should occur when this node is encountered?"
//! ```
//!
//! The walker answers:
//!
//! ```text
//! "Which children exist and in what order should they be visited?"
//! ```
//!
//! Keeping these responsibilities separate is essential.
//!
//! `visitor_mut.rs` must never contain a giant match over every AST node type.
//!
//! Structural traversal belongs to:
//!
//! ```text
//! src/frontend/ast/node/visitors/walk.rs
//! ```
//!
//! Transformation/fold semantics belong to:
//!
//! ```text
//! src/frontend/ast/node/visitors/fold.rs
//! ```
//!
//! ## Mutation boundary
//!
//! The canonical mutable visitor receives `&mut Node`.
//!
//! This gives visitors controlled access to the common source-level fields:
//!
//! - `NodeId`;
//! - `NodeKind`;
//! - `Span`;
//! - `NodeMetadata`.
//!
//! It deliberately does not invent a second mutable node abstraction.
//!
//! Concrete AST child fields remain owned by their concrete AST node types and
//! are handled by the structural walker/fold infrastructure.
//!
//! This distinction prevents this file from becoming coupled to every current
//! and future declaration, expression, statement, type, pattern, resource,
//! domain, or extension.
//!
//! ## Important transformation rule
//!
//! A mutable visitor may change source-level representation, but changing a
//! node's identity must remain exceptional.
//!
//! `NodeId` is identity, not an ownership identifier.
//!
//! Therefore transformations should normally:
//!
//! - preserve `NodeId` when transforming the same logical node;
//! - allocate a new `NodeId` when creating a genuinely new logical node;
//! - never derive identity from memory addresses;
//! - never derive identity from timestamps;
//! - never derive identity from backend state.
//!
//! The canonical `Node` implementation follows this identity model.
//!
//! ## NodeKind extensibility
//!
//! `NodeKind` is extensible and can distinguish native core nodes from
//! namespaced extension nodes. The mutable visitor therefore does not define
//! methods such as:
//!
//! ```text
//! visit_x_gate()
//! visit_h_gate()
//! visit_cnot()
//! visit_ibm_backend()
//! visit_qpu()
//! ```
//!
//! Such APIs would make the AST visitor a closed registry of today's
//! technologies.
//!
//! Instead, visitors receive the canonical `Node` and can inspect its
//! namespaced `NodeKind` when they intentionally understand an extension.
//!
//! A new quantum technology, computational domain, or hardware backend does
//! not require modifying this file.
//!
//! ## Error model
//!
//! Every fallible callback uses an associated error type:
//!
//! ```text
//! type Error;
//! ```
//!
//! The AST therefore does not impose a single global error type.
//!
//! This permits independent users such as:
//!
//! - structural transformations;
//! - source rewriting;
//! - formatting preparation;
//! - migration tools;
//! - semantic preparation;
//! - IDE tooling;
//! - documentation tooling;
//! - AST normalization;
//! - source-to-source transforms.
//!
//! to define their own errors.
//!
//! Errors immediately terminate the current operation.
//!
//! ## Cancellation
//!
//! Cancellation is represented by `should_cancel()` rather than a concrete
//! cancellation-token type.
//!
//! This keeps the AST independent of:
//!
//! - async runtimes;
//! - executor implementations;
//! - operating-system signals;
//! - compiler session objects;
//! - downstream scheduling/runtime infrastructure.
//!
//! A caller may connect this hook to any cancellation mechanism.
//!
//! ## Scalability
//!
//! This module contains no language-level limits for:
//!
//! - AST node count;
//! - nesting depth;
//! - declarations;
//! - expressions;
//! - statements;
//! - generic parameters;
//! - resources;
//! - quantum resources;
//! - operations;
//! - modules;
//! - domains.
//!
//! Any traversal/resource budget belongs to the caller or traversal layer and
//! must be explicit and configurable.
//!
//! In particular, this file must never contain constants such as:
//!
//! ```text
//! MAX_NODES
//! MAX_QUBITS
//! MAX_DEPTH
//! MAX_REGISTER_SIZE
//! ```
//!
//! Those would incorrectly turn compiler safety policy into language semantics.
//!
//! ## Determinism
//!
//! This module owns no global mutable state.
//!
//! It uses no:
//!
//! - randomness;
//! - timestamps;
//! - memory addresses;
//! - thread identifiers;
//! - process identifiers;
//! - backend state.
//!
//! Deterministic traversal order is the responsibility of the walker.
//!
//! A mutable visitor must not use unordered global state to decide how the AST
//! is transformed.
//!
//! ## Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! Only stable standard-library facilities and canonical AST infrastructure are
//! used.
//!
//! ## Dependency contract
//!
//! Allowed dependencies are limited to:
//!
//! ```text
//! node.rs
//! node_id.rs
//! node_kind.rs
//! source/span.rs
//! metadata.rs
//! standard library
//! ```
//!
//! The module must never depend on:
//!
//! ```text
//! parser
//! lexer
//! semantic
//! compiler
//! ZUIR
//! quantum::ir
//! hardware
//! optimizer
//! scheduler
//! router
//! QEC
//! runtime
//! backend
//! ```
//!
//! ## Integration contract
//!
//! The intended flow is:
//!
//! ```text
//! concrete AST
//!      │
//!      ▼
//! visitors/walk.rs
//!      │
//!      ├── mutable visitor
//!      │       │
//!      │       └── mutate common source-level node state
//!      │
//!      └── concrete child traversal
//!
//! resulting AST
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//! ```
//!
//! A mutable visitor is therefore a frontend transformation mechanism, not a
//! backend transformation mechanism.
//!
//! ## Integration with `visitor.rs`
//!
//! `visitor.rs` and this module intentionally have parallel protocols.
//!
//! The immutable visitor receives:
//!
//! ```text
//! &Node
//! ```
//!
//! The mutable visitor receives:
//!
//! ```text
//! &mut Node
//! ```
//!
//! They must not be merged into a single trait using interior mutability.
//!
//! Interior mutability would make mutation semantics less explicit and could
//! obscure aliasing requirements.
//!
//! ## Integration with `walk.rs`
//!
//! `walk.rs` owns:
//!
//! - traversal order;
//! - child discovery;
//! - recursion/iteration strategy;
//! - traversal budgets;
//! - cancellation checkpoints;
//! - depth accounting.
//!
//! It should call the mutable visitor lifecycle in this order:
//!
//! ```text
//! start
//!   │
//!   ▼
//! observe depth
//!   │
//!   ▼
//! enter node
//!   │
//!   ▼
//! visit node
//!   │
//!   ├── Continue ───────────► visit children
//!   │
//!   ├── SkipChildren ───────► skip children
//!   │
//!   └── Stop ───────────────► stop traversal
//!   │
//!   ▼
//! leave node
//!   │
//!   ▼
//! finish
//! ```
//!
//! The exact child traversal must remain outside this file.
//!
//! ## Integration with `fold.rs`
//!
//! `fold.rs` should be preferred when a transformation needs to replace,
//! remove, or rebuild concrete child nodes.
//!
//! This visitor is appropriate for mutations of the canonical node envelope,
//! such as:
//!
//! - metadata normalization;
//! - source-span adjustment;
//! - source-level node-kind normalization;
//! - transformation bookkeeping.
//!
//! A fold may use this visitor protocol as part of a larger transformation,
//! but this visitor must not become a fold implementation.
//!
//! ## Integration with validation
//!
//! Mutation should normally occur before final structural validation.
//!
//! A transformation pipeline may therefore be:
//!
//! ```text
//! parsed AST
//!     │
//!     ▼
//! mutable visitor/fold
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//! ```
//!
//! If a transformation changes a structural invariant, validation is
//! responsible for detecting it.
//!
//! The mutable visitor itself does not perform global AST validation.
//!
//! ## Integration with semantic analysis
//!
//! Semantic analysis must never depend on mutable visitor side effects as a
//! hidden communication mechanism.
//!
//! If semantic information is required, it belongs in the semantic model or
//! explicit side tables.
//!
//! This visitor remains source-level.
//!
//! ## Integration with quantum computation
//!
//! Quantum source is visited exactly like other source.
//!
//! This module does not know what a qubit, gate, QPU, topology, pulse,
//! calibration, decoder, or physical mapping is.
//!
//! A quantum-specific extension visitor may inspect namespaced extension kinds,
//! but that logic belongs in the extension layer.
//!
//! Consequently:
//!
//! ```text
//! new gate
//! new quantum technology
//! new topology
//! new backend
//! new hardware architecture
//! ```
//!
//! must not require changes here.
//!
//! ## POCO-REAF invariant
//!
//! Mutation must preserve the distinction:
//!
//! ```text
//! source intent
//!     ≠
//! semantic meaning
//!     ≠
//! implementation
//!     ≠
//! hardware realization
//! ```
//!
//! A mutable AST visitor must never perform hardware mapping merely because it
//! happens to see a source-level resource.
//!
//! ## File completion contract
//!
//! This file is complete when it provides:
//!
//! - mutable visitor control;
//! - error propagation;
//! - lifecycle hooks;
//! - extension/core dispatch;
//! - cancellation;
//! - depth observation;
//! - canonical-node helpers;
//! - deterministic behavior;
//! - no concrete-node enumeration;
//! - no downstream dependencies;
//! - safe mutation;
//! - unit tests.
//!
//! Adding a new:
//!
//! - declaration;
//! - expression;
//! - statement;
//! - pattern;
//! - type;
//! - resource;
//! - domain;
//! - quantum technology;
//! - hardware backend;
//! - target architecture;
//!
//! must not require changing this file merely to make the new construct
//! traversable.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::NodeKind;
use super::super::source::Span;

/// Controls how a mutable AST walker proceeds after a visitor callback.
///
/// The control value is deliberately independent of the concrete AST node
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VisitMutControl {
    /// Continue with the current node's children.
    Continue,

    /// Keep the mutation performed on the current node but do not visit its
    /// children.
    SkipChildren,

    /// Stop traversal successfully.
    Stop,
}

impl VisitMutControl {
    /// Returns `true` when child traversal should continue.
    #[inline]
    #[must_use]
    pub const fn should_visit_children(self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns `true` when traversal should stop.
    #[inline]
    #[must_use]
    pub const fn should_stop(self) -> bool {
        matches!(self, Self::Stop)
    }

    /// Returns `true` when children should be skipped.
    #[inline]
    #[must_use]
    pub const fn should_skip_children(self) -> bool {
        matches!(self, Self::SkipChildren)
    }

    /// Combines two decisions conservatively.
    ///
    /// The strongest termination decision wins:
    ///
    /// ```text
    /// Stop > SkipChildren > Continue
    /// ```
    #[inline]
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Stop, _) | (_, Self::Stop) => Self::Stop,
            (Self::SkipChildren, _) | (_, Self::SkipChildren) => {
                Self::SkipChildren
            }
            (Self::Continue, Self::Continue) => Self::Continue,
        }
    }
}

/// Canonical mutable visitor contract for the Zamani AST.
///
/// The trait operates on the common [`Node`] representation. Concrete child
/// traversal remains the responsibility of `walk.rs`.
///
/// # Mutation semantics
///
/// A visitor may mutate:
///
/// - node metadata;
/// - source span;
/// - node classification;
/// - other source-level fields exposed by [`Node`].
///
/// It must not use mutation to introduce:
///
/// - semantic resolution;
/// - physical resources;
/// - backend state;
/// - hardware topology;
/// - scheduling;
/// - runtime state.
///
/// # Object safety
///
/// The trait contains no generic callback methods and can therefore be used as
/// a trait object where the associated error type is known.
pub trait AstVisitorMut {
    /// Error returned by this visitor.
    type Error;

    /// Called once before traversal starts.
    ///
    /// The default implementation continues.
    #[inline]
    fn start(&mut self) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Called when a node is entered.
    ///
    /// `depth` is observational traversal depth. It is not a language limit.
    ///
    /// The node is mutable, so this is the primary hook for source-level
    /// mutation that must happen before child traversal.
    #[inline]
    fn enter_node(
        &mut self,
        _node: &mut Node,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Called after the node's children have been processed.
    ///
    /// This is useful for post-order normalization.
    #[inline]
    fn leave_node(
        &mut self,
        _node: &mut Node,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Called once after successful traversal or an explicit `Stop`.
    ///
    /// A walker should not invoke this hook after an error unless that walker's
    /// documented contract explicitly provides transactional cleanup.
    #[inline]
    fn finish(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Handles a native core AST node.
    ///
    /// This is intentionally generic rather than exposing one method for every
    /// `CoreNodeKind`.
    #[inline]
    fn visit_core(
        &mut self,
        _node: &mut Node,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Handles a namespaced extension AST node.
    ///
    /// No quantum, HDL, AI, accelerator, vendor, or future-domain enumeration
    /// is present here.
    #[inline]
    fn visit_extension(
        &mut self,
        _node: &mut Node,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Dispatches to the core or extension hook according to the canonical
    /// `NodeKind`.
    ///
    /// This is the only classification logic owned by the visitor protocol.
    #[inline]
    fn visit_node(
        &mut self,
        node: &mut Node,
        depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        if node.kind().is_extension() {
            self.visit_extension(node, depth)
        } else {
            self.visit_core(node, depth)
        }
    }

    /// Returns whether the walker should cancel before another traversal unit.
    ///
    /// The default is never to cancel.
    #[inline]
    fn should_cancel(&self) -> bool {
        false
    }

    /// Allows the visitor to observe traversal depth independently from node
    /// processing.
    ///
    /// This can be used by resource-aware transformations without turning
    /// `depth` into a language-level restriction.
    #[inline]
    fn observe_depth(
        &mut self,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        Ok(VisitMutControl::Continue)
    }

    /// Performs the pre-child portion of the canonical mutable lifecycle.
    ///
    /// The order is:
    ///
    /// ```text
    /// observe_depth
    ///      ↓
    /// enter_node
    ///      ↓
    /// visit_node
    /// ```
    ///
    /// Child traversal is deliberately not performed here.
    #[inline]
    fn begin_node(
        &mut self,
        node: &mut Node,
        depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        let depth_control = self.observe_depth(depth)?;

        if depth_control.should_stop() {
            return Ok(VisitMutControl::Stop);
        }

        let enter_control = self.enter_node(node, depth)?;
        let combined = depth_control.combine(enter_control);

        if combined.should_stop() {
            return Ok(VisitMutControl::Stop);
        }

        let visit_control = self.visit_node(node, depth)?;

        Ok(combined.combine(visit_control))
    }

    /// Performs the post-child portion of the lifecycle.
    ///
    /// The caller must invoke this only after child traversal has completed or
    /// has intentionally been skipped.
    #[inline]
    fn end_node(
        &mut self,
        node: &mut Node,
        depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        self.leave_node(node, depth)
    }

    /// Returns the canonical node kind without exposing its storage.
    #[inline]
    fn node_kind<'a>(&self, node: &'a Node) -> &'a NodeKind {
        node.kind()
    }

    /// Returns the canonical node identity.
    #[inline]
    fn node_id(&self, node: &Node) -> NodeId {
        node.id()
    }

    /// Returns the canonical source span.
    #[inline]
    fn node_span<'a>(&self, node: &'a Node) -> &'a Span {
        node.span()
    }

    /// Returns immutable node metadata.
    #[inline]
    fn node_metadata<'a>(
        &self,
        node: &'a Node,
    ) -> &'a NodeMetadata {
        node.metadata()
    }

    /// Returns mutable node metadata.
    ///
    /// This is intentionally exposed as metadata rather than requiring callers
    /// to know the internal representation of `Node`.
    #[inline]
    fn node_metadata_mut<'a>(
        &mut self,
        node: &'a mut Node,
    ) -> &'a mut NodeMetadata {
        node.metadata_mut()
    }
}

/// A mutable visitor backed by a closure.
///
/// This adapter is useful for small source-level transformations and tests
/// where defining a named visitor type would add unnecessary ceremony.
pub struct NodeCallbackVisitorMut<F> {
    callback: F,
}

impl<F> NodeCallbackVisitorMut<F> {
    /// Creates a mutable callback visitor.
    #[inline]
    pub const fn new(callback: F) -> Self {
        Self { callback }
    }

    /// Returns a shared reference to the callback.
    #[inline]
    pub fn callback(&self) -> &F {
        &self.callback
    }

    /// Returns mutable access to the callback.
    #[inline]
    pub fn callback_mut(&mut self) -> &mut F {
        &mut self.callback
    }

    /// Consumes the visitor and returns the callback.
    #[inline]
    pub fn into_callback(self) -> F {
        self.callback
    }
}

impl<F, E> AstVisitorMut for NodeCallbackVisitorMut<F>
where
    F: FnMut(&mut Node) -> Result<VisitMutControl, E>,
{
    type Error = E;

    #[inline]
    fn visit_node(
        &mut self,
        node: &mut Node,
        _depth: usize,
    ) -> Result<VisitMutControl, Self::Error> {
        (self.callback)(node)
    }
}

/// Mutable traversal state for query/transform visitors that need an explicit
/// stop flag.
///
/// The state contains no node pointer and no memory address.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StopStateMut {
    stopped: bool,
}

impl StopStateMut {
    /// Creates a non-stopped state.
    #[inline]
    pub const fn new() -> Self {
        Self { stopped: false }
    }

    /// Returns whether a stop request has been recorded.
    #[inline]
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.stopped
    }

    /// Records a stop request.
    #[inline]
    pub const fn stop(&mut self) {
        self.stopped = true;
    }

    /// Clears the stop state.
    #[inline]
    pub const fn reset(&mut self) {
        self.stopped = false;
    }
}

/// Returns the canonical mutable node for any concrete AST node implementing
/// [`AstNode`].
///
/// This helper deliberately does not introduce another AST abstraction.
#[inline]
pub fn canonical_node_mut<'a, T>(node: &'a mut T) -> &'a mut Node
where
    T: AstNode + ?Sized,
{
    node.node_mut()
}

/// Visits exactly one canonical node without traversing children.
///
/// This helper is primarily useful for:
///
/// - unit testing mutable visitors;
/// - parser normalization;
/// - isolated AST transformations;
/// - tooling.
///
/// `depth` is fixed to zero because this function is not a tree traversal.
/// Callers performing actual traversal should use `walk.rs`.
#[inline]
pub fn visit_one_mut<T, V>(
    node: &mut T,
    visitor: &mut V,
) -> Result<VisitMutControl, V::Error>
where
    T: AstNode + ?Sized,
    V: AstVisitorMut + ?Sized,
{
    let canonical = node.node_mut();

    if visitor.should_cancel() {
        return Ok(VisitMutControl::Stop);
    }

    let begin = visitor.begin_node(canonical, 0)?;

    if begin.should_stop() {
        return Ok(VisitMutControl::Stop);
    }

    let end = visitor.end_node(canonical, 0)?;

    Ok(begin.combine(end))
}

/// Visits one already-canonical mutable node without traversing children.
///
/// This is the low-level entry point intended for `walk.rs`.
#[inline]
pub fn visit_canonical_node_mut<V>(
    node: &mut Node,
    depth: usize,
    visitor: &mut V,
) -> Result<VisitMutControl, V::Error>
where
    V: AstVisitorMut + ?Sized,
{
    if visitor.should_cancel() {
        return Ok(VisitMutControl::Stop);
    }

    let begin = visitor.begin_node(node, depth)?;

    if begin.should_stop() {
        return Ok(VisitMutControl::Stop);
    }

    let end = visitor.end_node(node, depth)?;

    Ok(begin.combine(end))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::metadata::NodeMetadata;
    use super::super::super::node::Node;
    use super::super::super::node_id::NodeId;
    use super::super::super::node_kind::{CoreNodeKind, NodeKind};
    use super::super::super::source::Span;

    fn test_node() -> Node {
        let id = NodeId::new(1).expect("one is a valid NodeId");

        Node::new(
            id,
            NodeKind::core(CoreNodeKind::Program),
            Span::default(),
            NodeMetadata::default(),
        )
    }

    #[derive(Debug, Default)]
    struct RecordingVisitor {
        events: Vec<&'static str>,
    }

    impl AstVisitorMut for RecordingVisitor {
        type Error = ();

        fn start(&mut self) -> Result<VisitMutControl, Self::Error> {
            self.events.push("start");
            Ok(VisitMutControl::Continue)
        }

        fn observe_depth(
            &mut self,
            _depth: usize,
        ) -> Result<VisitMutControl, Self::Error> {
            self.events.push("depth");
            Ok(VisitMutControl::Continue)
        }

        fn enter_node(
            &mut self,
            _node: &mut Node,
            _depth: usize,
        ) -> Result<VisitMutControl, Self::Error> {
            self.events.push("enter");
            Ok(VisitMutControl::Continue)
        }

        fn visit_core(
            &mut self,
            _node: &mut Node,
            _depth: usize,
        ) -> Result<VisitMutControl, Self::Error> {
            self.events.push("core");
            Ok(VisitMutControl::Continue)
        }

        fn leave_node(
            &mut self,
            _node: &mut Node,
            _depth: usize,
        ) -> Result<VisitMutControl, Self::Error> {
            self.events.push("leave");
            Ok(VisitMutControl::Continue)
        }

        fn finish(&mut self) -> Result<(), Self::Error> {
            self.events.push("finish");
            Ok(())
        }
    }

    #[test]
    fn continue_is_default_combination() {
        assert_eq!(
            VisitMutControl::Continue.combine(VisitMutControl::Continue),
            VisitMutControl::Continue
        );
    }

    #[test]
    fn stop_dominates_all_other_controls() {
        assert_eq!(
            VisitMutControl::Continue.combine(VisitMutControl::Stop),
            VisitMutControl::Stop
        );

        assert_eq!(
            VisitMutControl::SkipChildren.combine(VisitMutControl::Stop),
            VisitMutControl::Stop
        );

        assert_eq!(
            VisitMutControl::Stop.combine(VisitMutControl::Continue),
            VisitMutControl::Stop
        );

        assert_eq!(
            VisitMutControl::Stop.combine(VisitMutControl::SkipChildren),
            VisitMutControl::Stop
        );
    }

    #[test]
    fn skip_children_dominates_continue() {
        assert_eq!(
            VisitMutControl::Continue
                .combine(VisitMutControl::SkipChildren),
            VisitMutControl::SkipChildren
        );

        assert_eq!(
            VisitMutControl::SkipChildren
                .combine(VisitMutControl::Continue),
            VisitMutControl::SkipChildren
        );
    }

    #[test]
    fn canonical_node_mut_exposes_same_node() {
        let mut node = test_node();

        let canonical = canonical_node_mut(&mut node);

        assert_eq!(canonical.id(), node.id());
        assert_eq!(canonical.kind(), node.kind());
        assert_eq!(canonical.span(), node.span());
    }

    #[test]
    fn callback_can_mutate_metadata() {
        let mut node = test_node();

        let mut visitor =
            NodeCallbackVisitorMut::new(|node: &mut Node| {
                let _metadata = node.metadata_mut();
                Ok::<VisitMutControl, ()>(
                    VisitMutControl::Continue,
                )
            });

        let result = visit_canonical_node_mut(
            &mut node,
            0,
            &mut visitor,
        );

        assert_eq!(result, Ok(VisitMutControl::Continue));
    }

    #[test]
    fn callback_can_request_skip_children() {
        let mut node = test_node();

        let mut visitor =
            NodeCallbackVisitorMut::new(|_node: &mut Node| {
                Ok::<VisitMutControl, ()>(
                    VisitMutControl::SkipChildren,
                )
            });

        let result = visit_canonical_node_mut(
            &mut node,
            0,
            &mut visitor,
        );

        assert_eq!(result, Ok(VisitMutControl::SkipChildren));
    }

    #[test]
    fn callback_can_request_stop() {
        let mut node = test_node();

        let mut visitor =
            NodeCallbackVisitorMut::new(|_node: &mut Node| {
                Ok::<VisitMutControl, ()>(
                    VisitMutControl::Stop,
                )
            });

        let result = visit_canonical_node_mut(
            &mut node,
            0,
            &mut visitor,
        );

        assert_eq!(result, Ok(VisitMutControl::Stop));
    }

    #[test]
    fn cancellation_stops_before_mutation() {
        #[derive(Default)]
        struct Cancelled;

        impl AstVisitorMut for Cancelled {
            type Error = ();

            fn should_cancel(&self) -> bool {
                true
            }

            fn visit_node(
                &mut self,
                _node: &mut Node,
                _depth: usize,
            ) -> Result<VisitMutControl, Self::Error> {
                panic!("visit_node must not run after cancellation");
            }
        }

        let mut node = test_node();
        let original = node.clone();
        let mut visitor = Cancelled;

        let result =
            visit_canonical_node_mut(&mut node, 0, &mut visitor);

        assert_eq!(result, Ok(VisitMutControl::Stop));
        assert_eq!(node, original);
    }

    #[test]
    fn error_propagates_without_suppression() {
        #[derive(Debug, PartialEq, Eq)]
        struct TestError;

        struct Failing;

        impl AstVisitorMut for Failing {
            type Error = TestError;

            fn visit_node(
                &mut self,
                _node: &mut Node,
                _depth: usize,
            ) -> Result<VisitMutControl, Self::Error> {
                Err(TestError)
            }
        }

        let mut node = test_node();
        let mut visitor = Failing;

        let result =
            visit_canonical_node_mut(&mut node, 0, &mut visitor);

        assert_eq!(result, Err(TestError));
    }

    #[test]
    fn begin_node_preserves_mutation_before_children() {
        struct MutatingVisitor;

        impl AstVisitorMut for MutatingVisitor {
            type Error = ();

            fn visit_core(
                &mut self,
                node: &mut Node,
                _depth: usize,
            ) -> Result<VisitMutControl, Self::Error> {
                node.set_span(Span::default());
                Ok(VisitMutControl::Continue)
            }
        }

        let mut node = test_node();
        let mut visitor = MutatingVisitor;

        let result = visitor.begin_node(&mut node, 0);

        assert_eq!(result, Ok(VisitMutControl::Continue));
    }

    #[test]
    fn lifecycle_hooks_are_individually_callable() {
        let mut visitor = RecordingVisitor;
        let mut node = test_node();

        assert_eq!(
            visitor.start(),
            Ok(VisitMutControl::Continue)
        );

        assert_eq!(
            visitor.begin_node(&mut node, 0),
            Ok(VisitMutControl::Continue)
        );

        assert_eq!(
            visitor.end_node(&mut node, 0),
            Ok(VisitMutControl::Continue)
        );

        assert_eq!(visitor.finish(), Ok(()));

        assert_eq!(
            visitor.events,
            vec![
                "start",
                "depth",
                "enter",
                "core",
                "leave",
                "finish"
            ]
        );
    }

    #[test]
    fn node_helpers_use_canonical_identity_and_source_information() {
        struct HelperVisitor;

        impl AstVisitorMut for HelperVisitor {
            type Error = ();

            fn visit_core(
                &mut self,
                node: &mut Node,
                _depth: usize,
            ) -> Result<VisitMutControl, Self::Error> {
                assert_eq!(self.node_id(node), node.id());
                assert_eq!(self.node_kind(node), node.kind());
                assert_eq!(self.node_span(node), node.span());
                assert_eq!(
                    self.node_metadata(node),
                    node.metadata()
                );

                let _metadata = self.node_metadata_mut(node);

                Ok(VisitMutControl::Continue)
            }
        }

        let mut node = test_node();
        let mut visitor = HelperVisitor;

        assert_eq!(
            visitor.begin_node(&mut node, 0),
            Ok(VisitMutControl::Continue)
        );
    }
}