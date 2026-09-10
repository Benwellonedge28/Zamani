//! # Zamani Frontend AST — Canonical Visitor Contract
//!
//! `src/frontend/ast/node/visitors/visitor.rs`
//!
//! This module defines the canonical, source-level visitor contract for the
//! native Zamani Abstract Syntax Tree (AST).
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
//! validation       visitors
//!                      │
//!                      ▼
//!                   traversal
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
//! This file owns the **visitor protocol** only.
//!
//! It does not own:
//!
//! - AST node definitions;
//! - parser logic;
//! - AST storage;
//! - AST traversal algorithms;
//! - semantic analysis;
//! - type checking;
//! - resource analysis;
//! - quantum compilation;
//! - quantum routing;
//! - quantum scheduling;
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
//! The visitor observes source-level AST structure.
//!
//! It must never require knowledge of the machine on which the program will
//! eventually execute.
//!
//! Therefore this module contains no:
//!
//! - maximum qubit count;
//! - maximum register size;
//! - processor-specific logic;
//! - hardware topology;
//! - vendor names;
//! - backend identifiers;
//! - physical qubit mappings;
//! - instruction-set assumptions;
//! - fixed computational-domain list.
//!
//! This preserves:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! The same visitor protocol can therefore be used for programs ranging from
//! tiny ASTs to ASTs whose size is limited only by available compiler
//! resources and explicitly configured safety policies.
//!
//! ## Why this file must not contain concrete traversal
//!
//! A visitor and a walker have different responsibilities.
//!
//! The visitor defines:
//!
//! ```text
//! "What should happen when a node is encountered?"
//! ```
//!
//! The walker defines:
//!
//! ```text
//! "Which children are encountered, and in what order?"
//! ```
//!
//! Keeping those responsibilities separate prevents this trait from becoming
//! coupled to every concrete AST node introduced by the language.
//!
//! The intended architecture is:
//!
//! ```text
//! visitor.rs
//!     │
//!     │ defines protocol
//!     ▼
//! walk.rs
//!     │
//!     │ performs structural traversal
//!     ▼
//! concrete AST nodes
//! ```
//!
//! A future AST node can therefore be integrated into `walk.rs` without
//! requiring semantic, quantum, hardware, or backend knowledge in this file.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST node infrastructure.
//!
//! The canonical dependency is:
//!
//! ```text
//! visitor.rs
//!      │
//!      ▼
//! node.rs
//!      │
//!      ├── Node
//!      ├── AstNode
//!      └── NodeKind
//! ```
//!
//! It must never depend on:
//!
//! ```text
//! visitor.rs
//!     ✗→ parser
//!     ✗→ lexer
//!     ✗→ semantic
//!     ✗→ ZUIR
//!     ✗→ quantum::ir
//!     ✗→ hardware
//!     ✗→ optimizer
//!     ✗→ scheduler
//!     ✗→ router
//!     ✗→ QEC
//!     ✗→ runtime
//!     ✗→ backend
//! ```
//!
//! ## Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! It uses only stable standard-library facilities and the canonical AST
//! `Node`/`NodeKind` types.
//!
//! ## Safety
//!
//! There is deliberately no `unsafe` code in this module.
//!
//! Visitor implementations are ordinary safe Rust values.
//!
//! ## Scalability
//!
//! This visitor protocol does not impose a semantic limit on:
//!
//! - AST nodes;
//! - declarations;
//! - expressions;
//! - statements;
//! - patterns;
//! - generic parameters;
//! - resources;
//! - operations;
//! - quantum resources;
//! - source modules.
//!
//! Compiler resource limits belong to the compiler's configurable resource
//! policy, not to the language AST visitor contract.
//!
//! A walker may additionally enforce a caller-supplied traversal budget to
//! protect compiler processes from hostile or pathological input. Such a
//! budget must be explicit and external to the visitor semantics.
//!
//! ## Determinism
//!
//! This protocol introduces no:
//!
//! - global mutable state;
//! - random state;
//! - timestamps;
//! - process identifiers;
//! - thread identifiers;
//! - memory addresses;
//! - backend state.
//!
//! Traversal order is owned by the walker.
//!
//! When a deterministic walker is used, visitors observe that deterministic
//! order.
//!
//! ## Parallelism
//!
//! A visitor is an ordinary value and contains no hidden global state.
//!
//! This makes it possible to use separate visitor instances in independent
//! compilation tasks.
//!
//! Read-only AST access is represented through shared references.
//!
//! The visitor itself is mutable because visitors commonly accumulate analysis
//! state, diagnostics, metrics, or transformations' bookkeeping.
//!
//! ## Error handling
//!
//! Visitor operations return `Result` through an associated error type.
//!
//! This avoids forcing every AST consumer into a single global diagnostic or
//! error type.
//!
//! Examples include:
//!
//! - diagnostics visitors;
//! - symbol collectors;
//! - dependency analyzers;
//! - source metrics;
//! - AST validators;
//! - documentation extractors;
//! - semantic-analysis preparation;
//! - tooling;
//! - transformation pre-analysis.
//!
//! The visitor must never panic merely because an AST node is an extension or
//! an unfamiliar node kind.
//!
//! ## Extension handling
//!
//! `NodeKind` is intentionally extensible.
//!
//! The repository's `NodeKind` supports both native core kinds and namespaced
//! extension kinds.
//!
//! Consequently, this visitor protocol provides explicit hooks for extension
//! nodes without enumerating quantum, HDL, AI, accelerator, vendor, or future
//! technologies.
//!
//! A visitor that does not understand an extension may safely ignore it,
//! reject it, or record it according to its own policy.
//!
//! The default behavior is non-failing observation.
//!
//! ## No giant visitor enum
//!
//! This file intentionally does NOT define something like:
//!
//! ```text
//! enum VisitKind {
//!     Program,
//!     Function,
//!     XGate,
//!     HGate,
//!     CNOT,
//!     IBMBackend,
//!     ...
//! }
//! ```
//!
//! Such an enumeration would make the AST visitor a closed registry of every
//! future computational technology.
//!
//! Instead, native `NodeKind` and its extensible classification remain the
//! source of truth.
//!
//! ## Visitor lifecycle
//!
//! The canonical lifecycle is:
//!
//! ```text
//! start
//!   │
//!   ▼
//! enter node
//!   │
//!   ├── descend into children
//!   │
//!   ▼
//! leave node
//!   │
//!   ▼
//! finish
//! ```
//!
//! The walker owns actual child traversal.
//!
//! The visitor controls whether traversal should:
//!
//! - continue;
//! - skip this node's children;
//! - stop successfully.
//!
//! Errors abort traversal immediately.
//!
//! ## Integration with `Node`
//!
//! The repository's canonical `Node` provides:
//!
//! - `id()`;
//! - `kind()`;
//! - `span()`;
//! - `metadata()`;
//! - extension/core classification;
//! - diagnostic summaries.
//!
//! The visitor operates directly on that stable common representation.
//!
//! This avoids forcing the visitor contract to know the concrete layout of
//! every declaration, expression, statement, type, pattern, resource, domain,
//! or future extension.
//!
//! ## Integration with `AstNode`
//!
//! Concrete AST nodes implement the repository's `AstNode` trait.
//!
//! A future concrete walker may therefore expose an API conceptually like:
//!
//! ```text
//! walk_function(function, visitor)
//! walk_expression(expression, visitor)
//! walk_statement(statement, visitor)
//! walk_program(program, visitor)
//! ```
//!
//! Those functions can obtain the common `Node` through `AstNode::node()` and
//! invoke this visitor contract without creating a second node abstraction.
//!
//! ## Integration with `walk.rs`
//!
//! `walk.rs` should be the structural traversal owner.
//!
//! Its responsibility is:
//!
//! 1. call `visitor.start()`;
//! 2. call `visitor.enter_node(...)`;
//! 3. inspect the concrete AST node;
//! 4. visit children in canonical source order;
//! 5. honor `VisitControl`;
//! 6. call `visitor.leave_node(...)`;
//! 7. call `visitor.finish()`.
//!
//! `walk.rs` must not duplicate visitor semantics.
//!
//! `visitor.rs` must not duplicate child relationships.
//!
//! This division is deliberate.
//!
//! ## Integration with validation
//!
//! Structural validation may be implemented as a visitor.
//!
//! However, the visitor protocol itself must not assume validation.
//!
//! Validation can use:
//!
//! ```text
//! AST
//!  │
//!  ▼
//! walk
//!  │
//!  ▼
//! validation visitor
//! ```
//!
//! Semantic validation remains downstream.
//!
//! ## Integration with semantic analysis
//!
//! Semantic analysis may use visitors for source collection and preparation:
//!
//! ```text
//! AST
//!  │
//!  ▼
//! visitor
//!  │
//!  ▼
//! symbol/type/resource collection
//!  │
//!  ▼
//! semantic model
//! ```
//!
//! The visitor must not resolve symbols itself merely because a semantic
//! consumer happens to use it.
//!
//! ## Integration with ZUIR
//!
//! ZUIR lowering is not performed by this trait.
//!
//! A ZUIR lowering pass may use the visitor/traversal infrastructure, but the
//! visitor contract must remain unaware of ZUIR.
//!
//! ## Quantum integration
//!
//! Quantum programs are ordinary AST inputs to this visitor system.
//!
//! A quantum-specific visitor can inspect namespaced extension nodes or native
//! resource/operation nodes through the same protocol.
//!
//! This file does not define:
//!
//! - qubit visitors;
//! - gate visitors;
//! - QPU visitors;
//! - QEC visitors;
//! - routing visitors;
//! - scheduling visitors.
//!
//! Those are downstream or extension-level concerns.
//!
//! A new quantum technology therefore does not require modifying this file.
//!
//! ## POCO-REAF invariant
//!
//! A visitor must observe the program's source representation, not its eventual
//! hardware realization.
//!
//! Therefore a visitor cannot assume:
//!
//! ```text
//! one program = one machine
//! one resource = one physical qubit
//! one operation = one hardware instruction
//! one register = fixed hardware width
//! ```
//!
//! These are compilation decisions made later.
//!
//! ## Visitor result semantics
//!
//! `VisitControl` distinguishes three valid traversal decisions:
//!
//! - `Continue`: process this node's children;
//! - `SkipChildren`: observe this node but do not descend into its children;
//! - `Stop`: terminate traversal successfully.
//!
//! `Stop` is useful for queries such as:
//!
//! - find first matching node;
//! - locate a declaration;
//! - detect the first occurrence of a construct;
//! - tooling queries.
//!
//! `SkipChildren` is useful for:
//!
//! - opaque/generated regions;
//! - intentionally ignored metadata;
//! - analysis passes that only care about parent-level constructs;
//! - performance-sensitive queries.
//!
//! ## Cancellation
//!
//! Cancellation is represented by a visitor hook rather than a global token.
//!
//! This permits callers to use any cancellation mechanism without coupling the
//! AST subsystem to a runtime or asynchronous framework.
//!
//! The default implementation never requests cancellation.
//!
//! A walker should call `should_cancel()` at safe traversal boundaries.
//!
//! ## Depth
//!
//! The visitor receives a traversal depth.
//!
//! Depth is informational and must not be interpreted as a language-level
//! maximum.
//!
//! A walker may enforce its own configurable depth/resource policy.
//!
//! This distinction is essential for unbounded/scalable source programs:
//!
//! ```text
//! language semantics
//!       ≠
//! compiler safety policy
//! ```
//!
//! ## File-level ownership
//!
//! This file owns:
//!
//! - `VisitControl`;
//! - visitor error protocol;
//! - visitor lifecycle hooks;
//! - node-entry/exit hooks;
//! - extension hook;
//! - cancellation hook;
//! - traversal-depth observation;
//! - visitor convenience methods.
//!
//! This file does not own:
//!
//! - concrete AST nodes;
//! - child relationships;
//! - traversal order;
//! - recursion implementation;
//! - semantic state;
//! - transformation state.
//!
//! ## File-level completion contract
//!
//! Once this file is complete, changes to concrete AST node implementations
//! should not require changing this file unless the **canonical visitor
//! protocol itself** changes.
//!
//! Adding:
//!
//! - a new declaration;
//! - a new expression;
//! - a new statement;
//! - a new pattern;
//! - a new type;
//! - a new resource;
//! - a new domain;
//! - a new quantum technology;
//! - a new hardware backend;
//! - a new machine architecture;
//! - a larger machine;
//! - a new ZUIR lowering;
//!
//! must not require modifying this visitor protocol merely to make that
//! construct exist.
//!
//! ## Testing contract
//!
//! The visitor implementation must be tested for:
//!
//! - default continuation;
//! - child skipping;
//! - successful stopping;
//! - error propagation;
//! - lifecycle ordering;
//! - extension dispatch;
//! - cancellation;
//! - depth reporting;
//! - zero-sized/leaf nodes;
//! - very deep traversal when used with an iterative walker;
//! - deterministic visitor behavior;
//! - no hidden global state.
//!
//! These tests belong in the visitor subsystem's tests and do not require
//! hardware or quantum backends.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

use super::super::node::{AstNode, Node};
use super::super::node_kind::NodeKind;

/// Controls how a structural AST walker proceeds after a visitor callback.
///
/// `VisitControl` is deliberately independent of the concrete AST node set.
/// It therefore remains stable as Zamani grows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VisitControl {
    /// Continue visiting the current node's children.
    Continue,

    /// Visit the current node but do not descend into its children.
    SkipChildren,

    /// Stop traversal successfully.
    ///
    /// This is not an error. It is an explicit early-completion request.
    Stop,
}

impl VisitControl {
    /// Returns `true` when traversal should continue into children.
    #[inline]
    #[must_use]
    pub const fn should_visit_children(self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns `true` when traversal should terminate successfully.
    #[inline]
    #[must_use]
    pub const fn should_stop(self) -> bool {
        matches!(self, Self::Stop)
    }

    /// Returns `true` when this control requests child suppression.
    #[inline]
    #[must_use]
    pub const fn should_skip_children(self) -> bool {
        matches!(self, Self::SkipChildren)
    }

    /// Combines two traversal decisions conservatively.
    ///
    /// The strongest termination decision wins:
    ///
    /// ```text
    /// Stop > SkipChildren > Continue
    /// ```
    ///
    /// This is useful when a walker combines decisions from multiple
    /// observation layers.
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

/// Canonical source-level visitor contract for the Zamani AST.
///
/// The trait intentionally operates on the common [`Node`] representation
/// instead of enumerating every concrete AST type.
///
/// Concrete walkers remain responsible for obtaining a node's children.
///
/// # Object safety
///
/// The trait contains no generic callback methods and is therefore suitable
/// for use as a trait object when a concrete error type is specified.
///
/// # Error handling
///
/// Every callback returns `Result<VisitControl, Self::Error>`.
///
/// Returning an error terminates the traversal immediately.
///
/// # Example
///
/// ```ignore
/// struct Counter {
///     count: usize,
/// }
///
/// impl AstVisitor for Counter {
///     type Error = ();
///
///     fn enter_node(
///         &mut self,
///         node: &Node,
///         _depth: usize,
///     ) -> Result<VisitControl, Self::Error> {
///         self.count = self.count.saturating_add(1);
///         Ok(VisitControl::Continue)
///     }
/// }
/// ```
///
/// The example intentionally does not assume any quantum, hardware, or
/// backend representation.
pub trait AstVisitor {
    /// Error type returned by this visitor.
    ///
    /// This is deliberately associated with the visitor rather than the AST,
    /// allowing different compiler passes to use their own error/diagnostic
    /// types without coupling the AST to one global error model.
    type Error;

    /// Called once before traversal begins.
    ///
    /// The default implementation continues.
    ///
    /// This hook is suitable for resetting per-traversal state or initializing
    /// a pass-local accumulator.
    #[inline]
    fn start(&mut self) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called when a node is entered.
    ///
    /// `depth` is the structural traversal depth supplied by the walker.
    ///
    /// Depth is observational metadata, not a language limit.
    ///
    /// The default behavior observes nothing and continues.
    #[inline]
    fn enter_node(
        &mut self,
        _node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called after the visitor has had the opportunity to process the
    /// current node's children.
    ///
    /// `depth` is the same structural depth supplied to `enter_node`.
    ///
    /// The default behavior continues.
    #[inline]
    fn leave_node(
        &mut self,
        _node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called once after traversal completes normally or through an explicit
    /// `Stop` decision.
    ///
    /// The walker should not call this hook after an error unless its explicit
    /// contract says otherwise.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn finish(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called for an extension node.
    ///
    /// This hook exists because [`NodeKind`] is open-ended.
    ///
    /// The visitor receives the canonical node rather than a technology-specific
    /// object. The extension namespace/name remain owned by `NodeKind`.
    ///
    /// The default implementation continues.
    ///
    /// A visitor that understands a particular extension may inspect:
    ///
    /// ```text
    /// node.kind().as_extension()
    /// ```
    ///
    /// without requiring the core AST visitor protocol to know what the
    /// extension means.
    #[inline]
    fn visit_extension(
        &mut self,
        _node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called for a native core node.
    ///
    /// This is intentionally a generic core hook rather than a method for every
    /// current `CoreNodeKind` variant.
    ///
    /// This prevents the visitor trait from becoming an unstable list that
    /// requires modification whenever the AST evolves.
    ///
    /// The default implementation continues.
    #[inline]
    fn visit_core(
        &mut self,
        _node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Called before processing a node after its core/extension classification
    /// has been determined.
    ///
    /// This is the general classification-independent observation hook.
    ///
    /// The default implementation delegates to `visit_core` or
    /// `visit_extension`.
    ///
    /// A walker may call this method directly rather than reproducing the
    /// classification logic.
    #[inline]
    fn visit_node(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        if node.kind().is_extension() {
            self.visit_extension(node, depth)
        } else {
            self.visit_core(node, depth)
        }
    }

    /// Returns whether traversal should be cancelled before processing the next
    /// traversal unit.
    ///
    /// The default implementation never requests cancellation.
    ///
    /// This hook deliberately avoids depending on:
    ///
    /// - `CancellationToken`;
    /// - async runtimes;
    /// - threads;
    /// - operating-system signals;
    /// - compiler session types.
    ///
    /// Any caller can implement cancellation using this simple protocol.
    #[inline]
    fn should_cancel(&self) -> bool {
        false
    }

    /// Gives the visitor a chance to observe the current traversal depth.
    ///
    /// The default implementation does nothing.
    ///
    /// This method exists separately from `enter_node` so a visitor can observe
    /// depth policy without requiring node-specific processing.
    #[inline]
    fn observe_depth(
        &mut self,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        Ok(VisitControl::Continue)
    }

    /// Handles one node using the canonical visitor lifecycle.
    ///
    /// This helper does not traverse children. Child traversal remains owned by
    /// `walk.rs`.
    ///
    /// The resulting control value tells the walker whether it should descend,
    /// skip children, or stop.
    ///
    /// The order is:
    ///
    /// ```text
    /// observe depth
    ///      │
    ///      ▼
    /// enter node
    ///      │
    ///      ▼
    /// visit node
    /// ```
    ///
    /// `leave_node` is intentionally not called here because it belongs after
    /// child traversal.
    #[inline]
    fn begin_node(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        let depth_control = self.observe_depth(depth)?;

        if depth_control.should_stop() {
            return Ok(VisitControl::Stop);
        }

        let enter_control = self.enter_node(node, depth)?;
        let combined = depth_control.combine(enter_control);

        if combined.should_stop() {
            return Ok(VisitControl::Stop);
        }

        let visit_control = self.visit_node(node, depth)?;
        Ok(combined.combine(visit_control))
    }

    /// Completes the post-order portion of visiting a node.
    ///
    /// This is intended for `walk.rs`.
    ///
    /// It does not itself inspect or traverse children.
    #[inline]
    fn end_node(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        self.leave_node(node, depth)
    }

    /// Returns a stable, source-level classification for the supplied node.
    ///
    /// This convenience method deliberately returns the canonical `NodeKind`
    /// reference rather than converting it into a backend/domain enum.
    #[inline]
    fn node_kind<'a>(&self, node: &'a Node) -> &'a NodeKind {
        node.kind()
    }

    /// Returns the stable AST node identity.
    #[inline]
    fn node_id(&self, node: &Node) -> super::super::node_id::NodeId {
        node.id()
    }

    /// Returns the node's source span.
    #[inline]
    fn node_span<'a>(
        &self,
        node: &'a Node,
    ) -> &'a super::super::source::Span {
        node.span()
    }

    /// Returns immutable metadata for the node.
    #[inline]
    fn node_metadata<'a>(
        &self,
        node: &'a Node,
    ) -> &'a super::super::metadata::NodeMetadata {
        node.metadata()
    }
}

/// A visitor that cannot fail.
///
/// This is useful for pure inspection passes, metrics, formatting preparation,
/// AST indexing, and tooling that does not need a fallible operation.
///
/// It is intentionally implemented as a marker error type rather than using a
/// global AST error enum.
pub type InfallibleAstVisitor<E = core::convert::Infallible> = E;

/// Convenience result type for visitors that use `core::convert::Infallible`.
///
/// This alias is intentionally generic and does not impose a global AST error
/// model.
pub type VisitResult<T> = Result<T, core::convert::Infallible>;

/// A minimal adapter for visitors that only need to inspect nodes.
///
/// This is useful when callers want a closure-like node callback while still
/// using the canonical visitor lifecycle.
///
/// The callback is stored as ordinary safe Rust state.
///
/// No global callback registry is created.
pub struct NodeCallbackVisitor<F> {
    callback: F,
}

impl<F> NodeCallbackVisitor<F> {
    /// Creates a visitor backed by `callback`.
    ///
    /// The callback receives each encountered node and may request:
    ///
    /// - `Continue`;
    /// - `SkipChildren`;
    /// - `Stop`.
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

impl<F, E> AstVisitor for NodeCallbackVisitor<F>
where
    F: FnMut(&Node) -> Result<VisitControl, E>,
{
    type Error = E;

    #[inline]
    fn visit_node(
        &mut self,
        node: &Node,
        _depth: usize,
    ) -> Result<VisitControl, Self::Error> {
        (self.callback)(node)
    }
}

/// A visitor that records whether traversal has been requested to stop.
///
/// This adapter is useful for query-oriented passes.
///
/// It deliberately does not store a node pointer or memory address.
/// `NodeId` is the only stable node identity exposed to the caller.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StopState {
    stopped: bool,
}

impl StopState {
    /// Creates an active, non-stopped state.
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

/// Compile-time assertion helper for concrete AST visitors.
///
/// This function accepts any `AstNode` and exposes its canonical [`Node`]
/// without introducing another abstraction.
///
/// It is intentionally tiny because the real traversal belongs to `walk.rs`.
#[inline]
pub fn canonical_node<'a, T>(node: &'a T) -> &'a Node
where
    T: AstNode + ?Sized,
{
    node.node()
}

/// Visits exactly one canonical node without traversing children.
///
/// This is useful for testing visitors independently from the walker.
///
/// The function performs the canonical begin/end lifecycle:
///
/// ```text
/// begin_node
///     │
///     ▼
/// end_node
/// ```
///
/// Because there are no children in this operation, `SkipChildren` and
/// `Continue` both result in the same final lifecycle behavior.
#[inline]
pub fn visit_one<T, V>(
    node: &T,
    visitor: &mut V,
) -> Result<VisitControl, V::Error>
where
    T: AstNode + ?Sized,
    V: AstVisitor + ?Sized,
{
    let canonical = node.node();

    if visitor.should_cancel() {
        return Ok(VisitControl::Stop);
    }

    let begin = visitor.begin_node(canonical, 0)?;

    if begin.should_stop() {
        return Ok(VisitControl::Stop);
    }

    let end = visitor.end_node(canonical, 0)?;

    Ok(begin.combine(end))
}

/// Runs a visitor lifecycle for an already-canonical node.
///
/// This is the lowest-level helper intended for walkers that already possess a
/// `Node`.
///
/// It does not descend into children.
#[inline]
pub fn visit_canonical_node<V>(
    node: &Node,
    depth: usize,
    visitor: &mut V,
) -> Result<VisitControl, V::Error>
where
    V: AstVisitor + ?Sized,
{
    if visitor.should_cancel() {
        return Ok(VisitControl::Stop);
    }

    let begin = visitor.begin_node(node, depth)?;

    if begin.should_stop() {
        return Ok(VisitControl::Stop);
    }

    let end = visitor.end_node(node, depth)?;

    Ok(begin.combine(end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node::Node;
    use super::super::super::node_id::NodeId;
    use super::super::super::node_kind::{CoreNodeKind, NodeKind};
    use super::super::super::source::Span;
    use super::super::super::metadata::NodeMetadata;

    #[derive(Debug, Default)]
    struct RecordingVisitor {
        events: Vec<&'static str>,
    }

    impl AstVisitor for RecordingVisitor {
        type Error = ();

        fn start(&mut self) -> Result<VisitControl, Self::Error> {
            self.events.push("start");
            Ok(VisitControl::Continue)
        }

        fn observe_depth(
            &mut self,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.events.push("depth");
            Ok(VisitControl::Continue)
        }

        fn enter_node(
            &mut self,
            _node: &Node,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.events.push("enter");
            Ok(VisitControl::Continue)
        }

        fn visit_core(
            &mut self,
            _node: &Node,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.events.push("core");
            Ok(VisitControl::Continue)
        }

        fn leave_node(
            &mut self,
            _node: &Node,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.events.push("leave");
            Ok(VisitControl::Continue)
        }

        fn finish(&mut self) -> Result<(), Self::Error> {
            self.events.push("finish");
            Ok(())
        }
    }

    fn test_node() -> Node {
        let id = NodeId::new(1).expect("one is a valid NodeId");

        Node::new(
            id,
            NodeKind::core(CoreNodeKind::Program),
            Span::default(),
            NodeMetadata::default(),
        )
    }

    #[test]
    fn visit_control_defaults_to_continue() {
        assert_eq!(
            VisitControl::Continue.combine(VisitControl::Continue),
            VisitControl::Continue
        );
    }

    #[test]
    fn stop_dominates_other_controls() {
        assert_eq!(
            VisitControl::Continue.combine(VisitControl::Stop),
            VisitControl::Stop
        );

        assert_eq!(
            VisitControl::SkipChildren.combine(VisitControl::Stop),
            VisitControl::Stop
        );

        assert_eq!(
            VisitControl::Stop.combine(VisitControl::Continue),
            VisitControl::Stop
        );
    }

    #[test]
    fn skip_children_dominates_continue() {
        assert_eq!(
            VisitControl::Continue.combine(VisitControl::SkipChildren),
            VisitControl::SkipChildren
        );
    }

    #[test]
    fn visit_one_uses_canonical_node() {
        let node = test_node();

        let mut visitor = RecordingVisitor::default();

        let result = visit_one(&node, &mut visitor);

        assert_eq!(result, Ok(VisitControl::Continue));

        assert_eq!(
            visitor.events,
            vec![
                "depth",
                "enter",
                "core",
                "leave",
            ]
        );
    }

    #[test]
    fn callback_visitor_invokes_callback() {
        let node = test_node();
        let mut count = 0usize;

        let mut visitor = NodeCallbackVisitor::new(
            |_node: &Node| {
                count = count.saturating_add(1);
                Ok::<VisitControl, ()>(VisitControl::Continue)
            },
        );

        let result = visit_one(&node, &mut visitor);

        assert_eq!(result, Ok(VisitControl::Continue));
        assert_eq!(count, 1);
    }

    #[test]
    fn callback_visitor_can_stop() {
        let node = test_node();

        let mut visitor = NodeCallbackVisitor::new(
            |_node: &Node| Ok::<VisitControl, ()>(VisitControl::Stop),
        );

        let result = visit_canonical_node(&node, 0, &mut visitor);

        assert_eq!(result, Ok(VisitControl::Stop));
    }

    #[test]
    fn default_visitor_is_non_failing() {
        struct EmptyVisitor;

        impl AstVisitor for EmptyVisitor {
            type Error = ();
        }

        let node = test_node();
        let mut visitor = EmptyVisitor;

        assert_eq!(
            visit_canonical_node(&node, 0, &mut visitor),
            Ok(VisitControl::Continue)
        );
    }

    #[test]
    fn extension_nodes_use_extension_hook() {
        struct ExtensionVisitor {
            extension_seen: bool,
        }

        impl AstVisitor for ExtensionVisitor {
            type Error = ();

            fn visit_extension(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<VisitControl, Self::Error> {
                self.extension_seen = true;
                Ok(VisitControl::Continue)
            }
        }

        let kind = NodeKind::extension(
            "test",
            "future-operation",
        )
        .expect("valid extension kind");

        let node = Node::new(
            NodeId::new(1).expect("valid node ID"),
            kind,
            Span::default(),
            NodeMetadata::default(),
        );

        let mut visitor = ExtensionVisitor {
            extension_seen: false,
        };

        let result = visit_canonical_node(&node, 0, &mut visitor);

        assert_eq!(result, Ok(VisitControl::Continue));
        assert!(visitor.extension_seen);
    }

    #[test]
    fn cancellation_stops_before_node_processing() {
        struct CancellingVisitor;

        impl AstVisitor for CancellingVisitor {
            type Error = ();

            fn should_cancel(&self) -> bool {
                true
            }
        }

        let node = test_node();
        let mut visitor = CancellingVisitor;

        assert_eq!(
            visit_canonical_node(&node, 0, &mut visitor),
            Ok(VisitControl::Stop)
        );
    }

    #[test]
    fn node_helpers_expose_canonical_identity() {
        struct EmptyVisitor;

        impl AstVisitor for EmptyVisitor {
            type Error = ();
        }

        let node = test_node();
        let mut visitor = EmptyVisitor;

        assert_eq!(
            visitor.node_id(&node),
            NodeId::new(1).expect("valid node ID")
        );

        assert_eq!(
            visitor.node_kind(&node),
            node.kind()
        );

        assert_eq!(
            visitor.node_span(&node),
            node.span()
        );

        assert_eq!(
            visitor.node_metadata(&node),
            node.metadata()
        );
    }

    #[test]
    fn stop_state_is_deterministic() {
        let mut state = StopState::new();

        assert!(!state.is_stopped());

        state.stop();

        assert!(state.is_stopped());

        state.reset();

        assert!(!state.is_stopped());
    }
}