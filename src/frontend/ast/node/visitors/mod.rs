//! # Zamani Frontend AST — Visitor Infrastructure
//!
//! This module defines the canonical visitor contracts for the native Zamani
//! Abstract Syntax Tree.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer / parser
//!      │
//!      ▼
//! native Zamani AST
//!      │
//!      ├── visitors  ← this module
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
//!      │
//!      ├── classical IR
//!      ├── quantum IR
//!      ├── HDL IR
//!      └── future domain IRs
//!
//! ```
//!
//! ## Purpose
//!
//! Visitor infrastructure provides a stable way for compiler phases and tools
//! to inspect or transform the native AST without coupling those phases to
//! concrete storage layouts.
//!
//! The visitor layer is deliberately concerned with:
//!
//! - AST node observation;
//! - AST traversal control;
//! - deterministic traversal;
//! - cancellation;
//! - depth/resource accounting;
//! - read-only traversal;
//! - mutable traversal;
//! - visitor-local state;
//! - explicit traversal ownership;
//! - extension-node handling.
//!
//! It is NOT responsible for:
//!
//! - semantic analysis;
//! - type checking;
//! - name resolution;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum error correction;
//! - calibration;
//! - hardware mapping;
//! - backend execution;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR construction.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! The visitor API contains no machine-size assumptions.
//!
//! It does not assume:
//!
//! - a fixed number of AST nodes;
//! - a fixed nesting depth as language semantics;
//! - a fixed number of qubits;
//! - a fixed number of resources;
//! - a fixed number of processors;
//! - a fixed backend;
//! - a fixed quantum technology;
//! - a fixed computational domain.
//!
//! Compiler resource limits may be supplied explicitly through [`VisitContext`]
//! for operational safety. Such limits are policies, not language semantics.
//!
//! Therefore the same visitor contract can operate on a tiny AST or on an AST
//! whose size is bounded only by the resources available to the compiler.
//!
//! ## Design principle
//!
//! A visitor observes the AST; it does not redefine the AST.
//!
//! ```text
//! AST node definitions
//!       │
//!       ▼
//! AstNode
//!       │
//!       ▼
//! Visit / VisitMut
//!       │
//!       ▼
//! traversal implementation
//!       │
//!       ├── semantic analysis
//!       ├── validation
//!       ├── diagnostics
//!       ├── tooling
//!       ├── serialization inspection
//!       └── transformations
//! ```
//!
//! ## Important separation
//!
//! `Visit` and `VisitMut` are intentionally generic over AST nodes.
//! They do not enumerate quantum operations, hardware devices, gate sets,
//! backends, or future computational domains.
//!
//! This prevents the visitor infrastructure from becoming a hidden closed
//! enumeration of the Zamani language.
//!
//! ## Integration contract
//!
//! Concrete AST modules implement [`Visit`] and/or [`VisitMut`] for their
//! node/container types.
//!
//! A future `walk.rs` may provide canonical child traversal functions.
//! A future `fold.rs` may provide transformation/folding infrastructure.
//!
//! Those modules must depend on this contract, never the reverse.
//!
//! ```text
//! node_id.rs ────────┐
//! node_kind.rs ──────┤
//! node.rs ───────────┤
//!                     ▼
//!              visitors/mod.rs
//!                     │
//!              ┌──────┴──────┐
//!              ▼             ▼
//!          walk.rs        fold.rs
//!              │             │
//!              └──────┬──────┘
//!                     ▼
//!            concrete AST nodes
//!                     │
//!          ┌──────────┼──────────┐
//!          ▼          ▼          ▼
//!       semantic   validation  tooling
//!          │
//!          ▼
//!         ZUIR
//! ```
//!
//! The dependency direction must never be reversed.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - no `unsafe`;
//! - stable standard-library facilities only.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.

use core::fmt;
use core::marker::PhantomData;

use super::node::{AstNode, Node};
use super::node_id::NodeId;
use super::node_kind::NodeKind;
use super::source::Span;

/// Visitor API schema version.
///
/// This is independent from:
//!
//! - Zamani language version;
//! - AST node schema version;
//! - serialized AST format version;
//! - ZUIR version;
//! - compiler version.
pub const AST_VISITOR_SCHEMA_VERSION: u16 = 1;

/// Default maximum number of visited nodes when an explicit bounded traversal
/// policy is requested.
///
/// This value is NOT a language limit and is never automatically imposed on
/// traversal. An unbounded context uses `None`.
pub const DEFAULT_VISIT_NODE_LIMIT: Option<u64> = None;

/// Default maximum traversal depth.
///
/// `None` means that the visitor infrastructure itself imposes no semantic
/// nesting limit. Operational limits may be supplied explicitly by callers.
pub const DEFAULT_VISIT_DEPTH_LIMIT: Option<u64> = None;

/// Default cancellation polling interval.
///
/// A visitor must remain responsive to cancellation without requiring every
/// concrete node implementation to reinvent cancellation handling.
pub const DEFAULT_CANCELLATION_CHECK_INTERVAL: u64 = 1024;

/// Traversal decision returned by visitor callbacks.
///
/// The decision belongs to the traversal protocol, not to any semantic domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitDecision {
    /// Continue visiting this node and its children.
    Continue,

    /// Do not visit this node's children, but continue with subsequent nodes.
    SkipChildren,

    /// Stop traversal successfully.
    Stop,
}

impl VisitDecision {
    /// Returns `true` when traversal should continue.
    #[inline]
    pub const fn should_continue(self) -> bool {
        matches!(self, Self::Continue | Self::SkipChildren)
    }

    /// Returns `true` when children should be visited.
    #[inline]
    pub const fn should_visit_children(self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns `true` when traversal should stop.
    #[inline]
    pub const fn should_stop(self) -> bool {
        matches!(self, Self::Stop)
    }
}

/// Result of a visitor operation.
///
/// Visitor implementations use this type instead of panicking on normal
/// control-flow conditions such as cancellation or explicit traversal limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitOutcome {
    /// Traversal completed normally.
    Complete,

    /// Traversal was intentionally stopped by a visitor.
    Stopped,

    /// Traversal was cancelled.
    Cancelled,

    /// Traversal exceeded a caller-supplied resource policy.
    LimitExceeded,
}

impl VisitOutcome {
    /// Returns whether traversal completed without interruption.
    #[inline]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether traversal stopped intentionally.
    #[inline]
    pub const fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped)
    }

    /// Returns whether traversal was cancelled.
    #[inline]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    /// Returns whether a configured resource limit was reached.
    #[inline]
    pub const fn is_limit_exceeded(self) -> bool {
        matches!(self, Self::LimitExceeded)
    }
}

/// Reason a traversal operation terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitTermination {
    /// Visitor explicitly requested termination.
    Stopped,

    /// Caller cancellation was observed.
    Cancelled,

    /// Node-count limit was reached.
    NodeLimit,

    /// Depth limit was reached.
    DepthLimit,
}

impl fmt::Display for VisitTermination {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stopped => formatter.write_str("visitor requested traversal stop"),
            Self::Cancelled => formatter.write_str("AST traversal was cancelled"),
            Self::NodeLimit => formatter.write_str("AST traversal node limit exceeded"),
            Self::DepthLimit => formatter.write_str("AST traversal depth limit exceeded"),
        }
    }
}

/// Error produced by the visitor infrastructure.
///
/// This type intentionally describes traversal mechanics only. Semantic errors
/// belong to semantic analysis and AST validation errors belong to validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisitError {
    /// Traversal was cancelled.
    Cancelled {
        /// Node at which cancellation was observed, if known.
        node: Option<NodeId>,
    },

    /// Traversal exceeded its configured node budget.
    NodeLimitExceeded {
        /// Configured maximum.
        limit: u64,

        /// Number of nodes observed when the limit was reached.
        visited: u64,

        /// Node at which the limit was observed, if known.
        node: Option<NodeId>,
    },

    /// Traversal exceeded its configured nesting budget.
    DepthLimitExceeded {
        /// Configured maximum depth.
        limit: u64,

        /// Depth observed.
        depth: u64,

        /// Node at which the limit was observed, if known.
        node: Option<NodeId>,
    },

    /// A concrete traversal implementation attempted an invalid operation.
    ///
    /// This is reserved for structural traversal failures and should not be
    /// used for semantic errors.
    Structural {
        /// Human-readable description.
        message: String,

        /// Node associated with the problem, if known.
        node: Option<NodeId>,
    },
}

impl fmt::Display for VisitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled { node } => match node {
                Some(node) => write!(formatter, "AST traversal cancelled at node {node}"),
                None => formatter.write_str("AST traversal cancelled"),
            },

            Self::NodeLimitExceeded {
                limit,
                visited,
                node,
            } => match node {
                Some(node) => write!(
                    formatter,
                    "AST traversal node limit {limit} exceeded after {visited} nodes at {node}"
                ),
                None => write!(
                    formatter,
                    "AST traversal node limit {limit} exceeded after {visited} nodes"
                ),
            },

            Self::DepthLimitExceeded {
                limit,
                depth,
                node,
            } => match node {
                Some(node) => write!(
                    formatter,
                    "AST traversal depth limit {limit} exceeded at depth {depth} at {node}"
                ),
                None => write!(
                    formatter,
                    "AST traversal depth limit {limit} exceeded at depth {depth}"
                ),
            },

            Self::Structural { message, node } => match node {
                Some(node) => write!(formatter, "AST structural traversal error at {node}: {message}"),
                None => write!(formatter, "AST structural traversal error: {message}"),
            },
        }
    }
}

impl std::error::Error for VisitError {}

/// Result type used by visitor traversal APIs.
pub type VisitResult<T = VisitOutcome> = Result<T, VisitError>;

/// Indicates which phase of a node visit is occurring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitPhase {
    /// Before children are visited.
    Enter,

    /// After children have been visited.
    Leave,
}

impl VisitPhase {
    /// Returns whether this is the enter phase.
    #[inline]
    pub const fn is_enter(self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Returns whether this is the leave phase.
    #[inline]
    pub const fn is_leave(self) -> bool {
        matches!(self, Self::Leave)
    }
}

/// Lightweight immutable information about the currently visited node.
///
/// `NodeVisit<'a>` intentionally exposes the canonical [`Node`] rather than
/// concrete AST storage.
#[derive(Debug, Clone, Copy)]
pub struct NodeVisit<'a> {
    node: &'a Node,
    depth: u64,
}

impl<'a> NodeVisit<'a> {
    /// Creates a visit record.
    #[inline]
    pub(crate) const fn new(node: &'a Node, depth: u64) -> Self {
        Self { node, depth }
    }

    /// Returns the canonical node.
    #[inline]
    pub const fn node(self) -> &'a Node {
        self.node
    }

    /// Returns the stable AST node ID.
    #[inline]
    pub const fn id(self) -> NodeId {
        self.node.id()
    }

    /// Returns the node kind.
    #[inline]
    pub fn kind(self) -> &'a NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
    #[inline]
    pub fn span(self) -> &'a Span {
        self.node.span()
    }

    /// Returns the traversal depth.
    ///
    /// The root node has depth zero.
    #[inline]
    pub const fn depth(self) -> u64 {
        self.depth
    }

    /// Returns whether this is an extension node.
    #[inline]
    pub fn is_extension(self) -> bool {
        self.node.is_extension()
    }

    /// Returns whether this is a native core node.
    #[inline]
    pub fn is_core(self) -> bool {
        self.node.is_core()
    }
}

/// Mutable information about the currently visited node.
///
/// Only the common node metadata is exposed through [`AstNode::node_mut`].
/// Concrete traversal implementations decide which concrete fields may be
/// changed.
pub struct NodeVisitMut<'a> {
    node: &'a mut Node,
    depth: u64,
}

impl<'a> NodeVisitMut<'a> {
    /// Creates a mutable visit record.
    #[inline]
    pub(crate) fn new(node: &'a mut Node, depth: u64) -> Self {
        Self { node, depth }
    }

    /// Returns the canonical node.
    #[inline]
    pub fn node(&self) -> &Node {
        self.node
    }

    /// Returns mutable access to the canonical node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        self.node
    }

    /// Returns the stable AST node ID.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the node kind.
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the traversal depth.
    #[inline]
    pub const fn depth(&self) -> u64 {
        self.depth
    }

    /// Returns whether this is an extension node.
    #[inline]
    pub fn is_extension(&self) -> bool {
        self.node.is_extension()
    }

    /// Returns whether this is a native core node.
    #[inline]
    pub fn is_core(&self) -> bool {
        self.node.is_core()
    }
}

/// Explicit cancellation state for a traversal.
///
/// Cancellation is caller-owned. The visitor framework never creates global
/// cancellation state and never relies on process-global mutable data.
///
/// The boolean state can safely be shared by the owner of a traversal when
/// appropriate; this type itself deliberately does not prescribe a threading
/// mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CancellationToken {
    cancelled: bool,
}

impl CancellationToken {
    /// Creates a non-cancelled token.
    #[inline]
    pub const fn new() -> Self {
        Self { cancelled: false }
    }

    /// Creates an already-cancelled token.
    #[inline]
    pub const fn cancelled() -> Self {
        Self { cancelled: true }
    }

    /// Requests cancellation.
    #[inline]
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Clears cancellation.
    ///
    /// The caller is responsible for ensuring that reusing a token is
    /// appropriate for the surrounding compilation operation.
    #[inline]
    pub fn reset(&mut self) {
        self.cancelled = false;
    }

    /// Returns whether cancellation has been requested.
    #[inline]
    pub const fn is_cancelled(self) -> bool {
        self.cancelled
    }
}

/// Resource policy for AST traversal.
///
/// All limits are optional. `None` means no limit is imposed by the visitor
/// infrastructure.
///
/// These are compiler operational policies, never source-language limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VisitLimits {
    /// Maximum number of visited nodes.
    pub max_nodes: Option<u64>,

    /// Maximum traversal depth.
    pub max_depth: Option<u64>,
}

impl VisitLimits {
    /// Creates an unbounded traversal policy.
    #[inline]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_depth: None,
        }
    }

    /// Creates a bounded traversal policy.
    #[inline]
    pub const fn bounded(max_nodes: Option<u64>, max_depth: Option<u64>) -> Self {
        Self {
            max_nodes,
            max_depth,
        }
    }

    /// Returns the default visitor policy.
    #[inline]
    pub const fn default_policy() -> Self {
        Self {
            max_nodes: DEFAULT_VISIT_NODE_LIMIT,
            max_depth: DEFAULT_VISIT_DEPTH_LIMIT,
        }
    }

    /// Validates a proposed depth.
    #[inline]
    pub const fn permits_depth(self, depth: u64) -> bool {
        match self.max_depth {
            Some(limit) => depth <= limit,
            None => true,
        }
    }

    /// Validates a proposed visited-node count.
    #[inline]
    pub const fn permits_nodes(self, nodes: u64) -> bool {
        match self.max_nodes {
            Some(limit) => nodes <= limit,
            None => true,
        }
    }
}

impl Default for VisitLimits {
    #[inline]
    fn default() -> Self {
        Self::default_policy()
    }
}

/// Mutable state owned by one traversal operation.
///
/// This state is deliberately passed explicitly rather than hidden in global
/// state, allowing deterministic and independently reproducible traversals.
#[derive(Debug, Clone)]
pub struct VisitContext {
    limits: VisitLimits,
    visited: u64,
    depth: u64,
    cancellation: CancellationToken,
    cancellation_check_interval: u64,
}

impl VisitContext {
    /// Creates a traversal context with explicit limits.
    #[inline]
    pub fn new(limits: VisitLimits) -> Self {
        Self {
            limits,
            visited: 0,
            depth: 0,
            cancellation: CancellationToken::new(),
            cancellation_check_interval: DEFAULT_CANCELLATION_CHECK_INTERVAL,
        }
    }

    /// Creates an unlimited traversal context.
    #[inline]
    pub fn unlimited() -> Self {
        Self::new(VisitLimits::unlimited())
    }

    /// Returns the configured limits.
    #[inline]
    pub const fn limits(&self) -> VisitLimits {
        self.limits
    }

    /// Replaces traversal limits.
    ///
    /// This does not change the AST.
    #[inline]
    pub fn set_limits(&mut self, limits: VisitLimits) {
        self.limits = limits;
    }

    /// Returns the number of nodes observed by this context.
    #[inline]
    pub const fn visited(&self) -> u64 {
        self.visited
    }

    /// Returns the current traversal depth.
    #[inline]
    pub const fn depth(&self) -> u64 {
        self.depth
    }

    /// Returns the cancellation state.
    #[inline]
    pub const fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Requests cancellation.
    #[inline]
    pub fn cancel(&mut self) {
        self.cancellation.cancel();
    }

    /// Resets cancellation.
    #[inline]
    pub fn reset_cancellation(&mut self) {
        self.cancellation.reset();
    }

    /// Returns the cancellation polling interval.
    #[inline]
    pub const fn cancellation_check_interval(&self) -> u64 {
        self.cancellation_check_interval
    }

    /// Sets the cancellation polling interval.
    ///
    /// Zero is normalized to one so a traversal can never accidentally disable
    /// cancellation polling through a divide/modulo operation in a downstream
    /// implementation.
    #[inline]
    pub fn set_cancellation_check_interval(&mut self, interval: u64) {
        self.cancellation_check_interval = if interval == 0 { 1 } else { interval };
    }

    /// Records one visited node and validates the configured node limit.
    ///
    /// The increment is checked. Counter overflow is treated as exhaustion
    /// rather than wrapping around.
    pub fn record_node(&mut self, node: Option<NodeId>) -> VisitResult {
        let next = self
            .visited
            .checked_add(1)
            .ok_or_else(|| VisitError::NodeLimitExceeded {
                limit: u64::MAX,
                visited: u64::MAX,
                node,
            })?;

        if !self.limits.permits_nodes(next) {
            return Err(VisitError::NodeLimitExceeded {
                limit: self.limits.max_nodes.unwrap_or(u64::MAX),
                visited: next,
                node,
            });
        }

        self.visited = next;

        if self
            .cancellation_check_interval
            .checked_sub(1)
            .map(|_| true)
            .unwrap_or(true)
            && self.visited % self.cancellation_check_interval == 0
        {
            self.check_cancellation(node)?;
        }

        Ok(VisitOutcome::Complete)
    }

    /// Enters a child depth.
    ///
    /// The operation is checked before changing the context, so a failed
    /// operation leaves the context unchanged.
    pub fn enter_depth(&mut self, node: Option<NodeId>) -> VisitResult {
        let next = self
            .depth
            .checked_add(1)
            .ok_or_else(|| VisitError::DepthLimitExceeded {
                limit: self.limits.max_depth.unwrap_or(u64::MAX),
                depth: u64::MAX,
                node,
            })?;

        if !self.limits.permits_depth(next) {
            return Err(VisitError::DepthLimitExceeded {
                limit: self.limits.max_depth.unwrap_or(u64::MAX),
                depth: next,
                node,
            });
        }

        self.depth = next;
        Ok(VisitOutcome::Complete)
    }

    /// Leaves the current depth.
    ///
    /// Saturation is used defensively rather than allowing an invalid
    /// underflow to panic.
    #[inline]
    pub fn leave_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    /// Checks cancellation explicitly.
    #[inline]
    pub fn check_cancellation(&self, node: Option<NodeId>) -> VisitResult {
        if self.cancellation.is_cancelled() {
            Err(VisitError::Cancelled { node })
        } else {
            Ok(VisitOutcome::Complete)
        }
    }
}

impl Default for VisitContext {
    #[inline]
    fn default() -> Self {
        Self::new(VisitLimits::default())
    }
}

/// Read-only visitor over the native Zamani AST.
///
/// Concrete AST node types may implement `Visit` by delegating to a canonical
/// walker. The trait itself deliberately does not require knowledge of every
/// AST node type.
///
/// # Design
///
/// The visitor has two generic hooks:
///
/// - [`Visit::enter`] before children;
/// - [`Visit::leave`] after children.
///
/// This keeps traversal policy separate from node semantics.
///
/// # Extension safety
///
/// Extension nodes are observable through `NodeKind::Extension`. A visitor
/// that does not understand a particular extension can safely continue or skip
/// its children according to the traversal implementation.
///
/// A visitor must never assume that all node kinds are represented by a closed
/// enum.
pub trait Visit {
    /// Called when entering a node.
    ///
    /// Returning [`VisitDecision::SkipChildren`] prevents child traversal.
    #[inline]
    fn enter(&mut self, _node: NodeVisit<'_>, _context: &mut VisitContext) -> VisitResult<VisitDecision> {
        Ok(VisitDecision::Continue)
    }

    /// Called after all children have been visited.
    #[inline]
    fn leave(&mut self, _node: NodeVisit<'_>, _context: &mut VisitContext) -> VisitResult<VisitDecision> {
        Ok(VisitDecision::Continue)
    }

    /// Called for extension nodes before normal child traversal.
    ///
    /// The default implementation treats extensions exactly like ordinary
    /// nodes. This means the visitor API remains forward-compatible.
    #[inline]
    fn visit_extension(
        &mut self,
        node: NodeVisit<'_>,
        context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        self.enter(node, context)
    }

    /// Called for core/native nodes before normal child traversal.
    #[inline]
    fn visit_core(
        &mut self,
        node: NodeVisit<'_>,
        context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        self.enter(node, context)
    }
}

/// Mutable visitor over the native Zamani AST.
///
/// This trait is intentionally separate from [`Visit`] so read-only compiler
/// phases can remain statically unable to mutate AST state.
///
/// Transformations must preserve AST invariants and should normally allocate
/// new identities when creating genuinely new nodes.
pub trait VisitMut {
    /// Called when entering a mutable node.
    ///
    /// Returning [`VisitDecision::SkipChildren`] prevents child traversal.
    #[inline]
    fn enter(
        &mut self,
        _node: NodeVisitMut<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        Ok(VisitDecision::Continue)
    }

    /// Called after all children have been visited.
    #[inline]
    fn leave(
        &mut self,
        _node: NodeVisitMut<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        Ok(VisitDecision::Continue)
    }

    /// Called for extension nodes before normal child traversal.
    #[inline]
    fn visit_extension(
        &mut self,
        node: NodeVisitMut<'_>,
        context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        self.enter(node, context)
    }

    /// Called for native/core nodes before normal child traversal.
    #[inline]
    fn visit_core(
        &mut self,
        node: NodeVisitMut<'_>,
        context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        self.enter(node, context)
    }
}

/// Marker trait for values that can be visited by the native AST visitor
/// infrastructure.
///
/// Concrete AST containers should implement this trait when they own a
/// complete traversal entry point.
///
/// The associated lifetime is intentionally expressed through the methods
/// rather than stored in the trait itself, allowing one visitor instance to
/// process arbitrarily many nodes sequentially.
pub trait Visitable {
    /// Visits this value using a read-only visitor.
    fn visit<V: Visit>(
        &self,
        visitor: &mut V,
        context: &mut VisitContext,
    ) -> VisitResult;

    /// Visits this value using a mutable visitor.
    fn visit_mut<V: VisitMut>(
        &mut self,
        visitor: &mut V,
        context: &mut VisitContext,
    ) -> VisitResult;
}

/// Lightweight adapter for visiting one canonical [`Node`].
///
/// This adapter is useful for foundational infrastructure and tests. Concrete
/// AST nodes should generally implement their own `Visitable` traversal so
/// their children can be visited in source order.
pub struct NodeVisitor<'a, V> {
    visitor: &'a mut V,
    marker: PhantomData<&'a V>,
}

impl<'a, V> NodeVisitor<'a, V> {
    /// Creates a read-only visitor adapter.
    #[inline]
    pub fn new(visitor: &'a mut V) -> Self {
        Self {
            visitor,
            marker: PhantomData,
        }
    }
}

impl<'a, V: Visit> NodeVisitor<'a, V> {
    /// Visits a single canonical node without traversing children.
    ///
    /// This is intentionally limited to the node container. Child traversal
    /// belongs to concrete AST structures or the future canonical `walk`
    /// module.
    pub fn visit_node(
        &mut self,
        node: &'a Node,
        context: &mut VisitContext,
    ) -> VisitResult {
        context.record_node(Some(node.id()))?;

        let visit = NodeVisit::new(node, context.depth());

        let decision = if node.is_extension() {
            self.visitor.visit_extension(visit, context)?
        } else {
            self.visitor.visit_core(visit, context)?
        };

        match decision {
            VisitDecision::Continue | VisitDecision::SkipChildren => {
                let _ = self.visitor.leave(visit, context)?;
                Ok(VisitOutcome::Complete)
            }
            VisitDecision::Stop => Ok(VisitOutcome::Stopped),
        }
    }
}

/// Mutable adapter for visiting one canonical [`Node`].
pub struct NodeVisitorMut<'a, V> {
    visitor: &'a mut V,
    marker: PhantomData<&'a mut V>,
}

impl<'a, V> NodeVisitorMut<'a, V> {
    /// Creates a mutable visitor adapter.
    #[inline]
    pub fn new(visitor: &'a mut V) -> Self {
        Self {
            visitor,
            marker: PhantomData,
        }
    }
}

impl<'a, V: VisitMut> NodeVisitorMut<'a, V> {
    /// Visits a single canonical node without traversing children.
    pub fn visit_node(
        &mut self,
        node: &'a mut Node,
        context: &mut VisitContext,
    ) -> VisitResult {
        let id = node.id();
        context.record_node(Some(id))?;

        let depth = context.depth();
        let visit = NodeVisitMut::new(node, depth);

        let decision = if visit.is_extension() {
            self.visitor.visit_extension(visit, context)?
        } else {
            self.visitor.visit_core(visit, context)?
        };

        match decision {
            VisitDecision::Continue | VisitDecision::SkipChildren => Ok(VisitOutcome::Complete),
            VisitDecision::Stop => Ok(VisitOutcome::Stopped),
        }
    }
}

/// Visitor that counts nodes.
///
/// This utility intentionally counts only nodes actually presented to it by
/// traversal. It does not inspect or assume anything about the node's domain.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NodeCounter {
    count: u64,
}

impl NodeCounter {
    /// Creates a zero-count counter.
    #[inline]
    pub const fn new() -> Self {
        Self { count: 0 }
    }

    /// Returns the number of visited nodes.
    #[inline]
    pub const fn count(self) -> u64 {
        self.count
    }
}

impl Visit for NodeCounter {
    fn enter(
        &mut self,
        _node: NodeVisit<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        self.count = self.count.saturating_add(1);
        Ok(VisitDecision::Continue)
    }
}

/// Visitor that records the first extension node encountered.
///
/// It does not interpret the extension and therefore remains future-compatible.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FirstExtension {
    namespace: Option<String>,
    name: Option<String>,
    node: Option<NodeId>,
}

impl FirstExtension {
    /// Creates an empty extension detector.
    #[inline]
    pub const fn new() -> Self {
        Self {
            namespace: None,
            name: None,
            node: None,
        }
    }

    /// Returns the first extension node ID, if one was observed.
    #[inline]
    pub const fn node_id(&self) -> Option<NodeId> {
        self.node
    }

    /// Returns the first extension namespace, if one was observed.
    #[inline]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the first extension name, if one was observed.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Visit for FirstExtension {
    fn visit_extension(
        &mut self,
        node: NodeVisit<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        if self.node.is_none() {
            self.node = Some(node.id());

            if let Some(extension) = node.kind().as_extension() {
                self.namespace = Some(extension.namespace().to_owned());
                self.name = Some(extension.name().to_owned());
            }
        }

        Ok(VisitDecision::Continue)
    }
}

/// Visitor that stops at the first node matching a predicate.
///
/// The predicate receives the immutable node view and must not mutate AST
/// state.
pub struct FindVisitor<F> {
    predicate: F,
    found: Option<NodeId>,
}

impl<F> FindVisitor<F> {
    /// Creates a finder from a node predicate.
    #[inline]
    pub fn new(predicate: F) -> Self {
        Self {
            predicate,
            found: None,
        }
    }

    /// Returns the first matching node ID.
    #[inline]
    pub const fn found(&self) -> Option<NodeId> {
        self.found
    }
}

impl<F> Visit for FindVisitor<F>
where
    F: FnMut(NodeVisit<'_>) -> bool,
{
    fn enter(
        &mut self,
        node: NodeVisit<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        if (self.predicate)(node) {
            self.found = Some(node.id());
            Ok(VisitDecision::Stop)
        } else {
            Ok(VisitDecision::Continue)
        }
    }
}

/// Visitor that stops at the first node of a requested [`NodeKind`].
///
/// This comparison is exact and does not interpret semantic meaning.
pub struct FindKindVisitor {
    target: NodeKind,
    found: Option<NodeId>,
}

impl FindKindVisitor {
    /// Creates a kind finder.
    #[inline]
    pub fn new(target: NodeKind) -> Self {
        Self {
            target,
            found: None,
        }
    }

    /// Returns the first matching node ID.
    #[inline]
    pub const fn found(&self) -> Option<NodeId> {
        self.found
    }

    /// Returns the requested node kind.
    #[inline]
    pub fn target(&self) -> &NodeKind {
        &self.target
    }
}

impl Visit for FindKindVisitor {
    fn enter(
        &mut self,
        node: NodeVisit<'_>,
        _context: &mut VisitContext,
    ) -> VisitResult<VisitDecision> {
        if node.kind() == &self.target {
            self.found = Some(node.id());
            Ok(VisitDecision::Stop)
        } else {
            Ok(VisitDecision::Continue)
        }
    }
}

/// Generic no-op visitor.
///
/// Useful as a traversal probe and as a base for callers that only need the
/// canonical walker to execute.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopVisitor;

impl Visit for NoopVisitor {}

/// Generic no-op mutable visitor.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopVisitorMut;

impl VisitMut for NoopVisitorMut {}

/// Visit all nodes using a caller-provided read-only visitor.
///
/// This function is intentionally a thin API boundary. Concrete traversal
/// lives in [`Visitable`] implementations rather than in this module.
///
/// Keeping the dispatch here small prevents `mod.rs` from becoming a second
/// AST implementation.
#[inline]
pub fn visit<T, V>(
    root: &T,
    visitor: &mut V,
    context: &mut VisitContext,
) -> VisitResult
where
    T: Visitable + ?Sized,
    V: Visit,
{
    root.visit(visitor, context)
}

/// Visit all nodes using a caller-provided mutable visitor.
#[inline]
pub fn visit_mut<T, V>(
    root: &mut T,
    visitor: &mut V,
    context: &mut VisitContext,
) -> VisitResult
where
    T: Visitable + ?Sized,
    V: VisitMut,
{
    root.visit_mut(visitor, context)
}

/// Visit a single canonical node.
///
/// This helper does not invent child relationships. It is intended for
/// foundational nodes and tests.
#[inline]
pub fn visit_node<V>(
    node: &Node,
    visitor: &mut V,
    context: &mut VisitContext,
) -> VisitResult
where
    V: Visit,
{
    let mut adapter = NodeVisitor::new(visitor);
    adapter.visit_node(node, context)
}

/// Mutably visit a single canonical node.
#[inline]
pub fn visit_node_mut<V>(
    node: &mut Node,
    visitor: &mut V,
    context: &mut VisitContext,
) -> VisitResult
where
    V: VisitMut,
{
    let mut adapter = NodeVisitorMut::new(visitor);
    adapter.visit_node(node, context)
}

/// Returns the visitor infrastructure schema version.
#[inline]
pub const fn schema_version() -> u16 {
    AST_VISITOR_SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::node_kind::{CoreNodeKind, NodeKind};
    use crate::frontend::ast::node::source::Span;

    fn test_node() -> Node {
        Node::new(
            NodeId::first(),
            NodeKind::core(CoreNodeKind::Identifier),
            Span::default(),
            NodeMetadata::default(),
        )
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(schema_version(), AST_VISITOR_SCHEMA_VERSION);
        assert_eq!(AST_VISITOR_SCHEMA_VERSION, 1);
    }

    #[test]
    fn visit_decisions_have_expected_semantics() {
        assert!(VisitDecision::Continue.should_continue());
        assert!(VisitDecision::Continue.should_visit_children());
        assert!(!VisitDecision::Continue.should_stop());

        assert!(VisitDecision::SkipChildren.should_continue());
        assert!(!VisitDecision::SkipChildren.should_visit_children());

        assert!(!VisitDecision::Stop.should_continue());
        assert!(VisitDecision::Stop.should_stop());
    }

    #[test]
    fn unlimited_limits_have_no_semantic_bound() {
        let limits = VisitLimits::unlimited();

        assert!(limits.permits_nodes(u64::MAX));
        assert!(limits.permits_depth(u64::MAX));
    }

    #[test]
    fn bounded_limits_are_checked() {
        let limits = VisitLimits::bounded(Some(2), Some(3));

        assert!(limits.permits_nodes(1));
        assert!(limits.permits_nodes(2));
        assert!(!limits.permits_nodes(3));

        assert!(limits.permits_depth(0));
        assert!(limits.permits_depth(3));
        assert!(!limits.permits_depth(4));
    }

    #[test]
    fn context_counts_nodes_deterministically() {
        let mut context = VisitContext::unlimited();

        assert_eq!(
            context.record_node(Some(NodeId::first())),
            Ok(VisitOutcome::Complete)
        );
        assert_eq!(context.visited(), 1);
    }

    #[test]
    fn context_tracks_depth_without_underflow() {
        let mut context = VisitContext::unlimited();

        assert_eq!(
            context.enter_depth(Some(NodeId::first())),
            Ok(VisitOutcome::Complete)
        );
        assert_eq!(context.depth(), 1);

        context.leave_depth();
        assert_eq!(context.depth(), 0);

        context.leave_depth();
        assert_eq!(context.depth(), 0);
    }

    #[test]
    fn cancellation_is_explicit() {
        let mut context = VisitContext::unlimited();

        assert!(!context.is_cancelled());

        context.cancel();

        assert!(context.is_cancelled());

        let result = context.check_cancellation(Some(NodeId::first()));

        assert!(matches!(
            result,
            Err(VisitError::Cancelled { node: Some(_) })
        ));
    }

    #[test]
    fn node_counter_counts_entered_nodes() {
        let mut counter = NodeCounter::new();
        let mut context = VisitContext::unlimited();
        let node = test_node();

        assert_eq!(
            visit_node(&node, &mut counter, &mut context),
            Ok(VisitOutcome::Complete)
        );

        assert_eq!(counter.count(), 1);
    }

    #[test]
    fn noop_visitor_is_valid() {
        let mut visitor = NoopVisitor;
        let mut context = VisitContext::unlimited();
        let node = test_node();

        assert_eq!(
            visit_node(&node, &mut visitor, &mut context),
            Ok(VisitOutcome::Complete)
        );
    }

    #[test]
    fn find_kind_finds_exact_kind() {
        let node = test_node();
        let mut visitor =
            FindKindVisitor::new(NodeKind::core(CoreNodeKind::Identifier));
        let mut context = VisitContext::unlimited();

        let result = visit_node(&node, &mut visitor, &mut context);

        assert_eq!(result, Ok(VisitOutcome::Complete));
        assert_eq!(visitor.found(), Some(NodeId::first()));
    }

    #[test]
    fn find_kind_does_not_match_other_kind() {
        let node = test_node();
        let mut visitor =
            FindKindVisitor::new(NodeKind::core(CoreNodeKind::Function));
        let mut context = VisitContext::unlimited();

        let result = visit_node(&node, &mut visitor, &mut context);

        assert_eq!(result, Ok(VisitOutcome::Complete));
        assert_eq!(visitor.found(), None);
    }

    #[test]
    fn stop_decision_stops_single_node_visit() {
        struct StopVisitor;

        impl Visit for StopVisitor {
            fn enter(
                &mut self,
                _node: NodeVisit<'_>,
                _context: &mut VisitContext,
            ) -> VisitResult<VisitDecision> {
                Ok(VisitDecision::Stop)
            }
        }

        let node = test_node();
        let mut visitor = StopVisitor;
        let mut context = VisitContext::unlimited();

        assert_eq!(
            visit_node(&node, &mut visitor, &mut context),
            Ok(VisitOutcome::Stopped)
        );
    }

    #[test]
    fn extension_dispatch_is_separate_from_core_dispatch() {
        struct RecordingVisitor {
            core: u64,
            extension: u64,
        }

        impl Visit for RecordingVisitor {
            fn visit_core(
                &mut self,
                _node: NodeVisit<'_>,
                _context: &mut VisitContext,
            ) -> VisitResult<VisitDecision> {
                self.core += 1;
                Ok(VisitDecision::Continue)
            }

            fn visit_extension(
                &mut self,
                _node: NodeVisit<'_>,
                _context: &mut VisitContext,
            ) -> VisitResult<VisitDecision> {
                self.extension += 1;
                Ok(VisitDecision::Continue)
            }
        }

        let core_node = test_node();

        let extension_kind =
            NodeKind::extension("test", "future-node").expect("valid extension kind");

        let extension_node = Node::new(
            NodeId::new(2).expect("non-zero ID"),
            extension_kind,
            Span::default(),
            NodeMetadata::default(),
        );

        let mut visitor = RecordingVisitor {
            core: 0,
            extension: 0,
        };

        let mut context = VisitContext::unlimited();

        visit_node(&core_node, &mut visitor, &mut context).expect("core visit");
        visit_node(&extension_node, &mut visitor, &mut context)
            .expect("extension visit");

        assert_eq!(visitor.core, 1);
        assert_eq!(visitor.extension, 1);
    }

    #[test]
    fn mutable_visitor_can_update_metadata() {
        struct MetadataVisitor;

        impl VisitMut for MetadataVisitor {
            fn enter(
                &mut self,
                mut node: NodeVisitMut<'_>,
                _context: &mut VisitContext,
            ) -> VisitResult<VisitDecision> {
                let _metadata = node.node_mut().metadata_mut();
                Ok(VisitDecision::Continue)
            }
        }

        let mut node = test_node();
        let mut visitor = MetadataVisitor;
        let mut context = VisitContext::unlimited();

        assert_eq!(
            visit_node_mut(&mut node, &mut visitor, &mut context),
            Ok(VisitOutcome::Complete)
        );
    }

    #[test]
    fn context_node_limit_is_enforced_without_wrapping() {
        let mut context = VisitContext::new(VisitLimits::bounded(Some(1), None));

        assert_eq!(
            context.record_node(Some(NodeId::first())),
            Ok(VisitOutcome::Complete)
        );

        let second = NodeId::new(2).expect("non-zero ID");

        assert!(matches!(
            context.record_node(Some(second)),
            Err(VisitError::NodeLimitExceeded {
                limit: 1,
                visited: 2,
                node: Some(_)
            })
        ));

        assert_eq!(context.visited(), 1);
    }

    #[test]
    fn context_depth_limit_is_enforced_without_mutating_failed_state() {
        let mut context = VisitContext::new(VisitLimits::bounded(None, Some(1)));

        assert_eq!(
            context.enter_depth(Some(NodeId::first())),
            Ok(VisitOutcome::Complete)
        );
        assert_eq!(context.depth(), 1);

        let second = NodeId::new(2).expect("non-zero ID");

        assert!(matches!(
            context.enter_depth(Some(second)),
            Err(VisitError::DepthLimitExceeded {
                limit: 1,
                depth: 2,
                node: Some(_)
            })
        ));

        assert_eq!(context.depth(), 1);
    }

    #[test]
    fn zero_cancellation_interval_is_normalized() {
        let mut context = VisitContext::unlimited();

        context.set_cancellation_check_interval(0);

        assert_eq!(context.cancellation_check_interval(), 1);
    }

    #[test]
    fn visitor_context_has_no_machine_specific_state() {
        let context = VisitContext::unlimited();

        assert_eq!(context.visited(), 0);
        assert_eq!(context.depth(), 0);
        assert!(!context.is_cancelled());
        assert_eq!(context.limits(), VisitLimits::unlimited());
    }
}