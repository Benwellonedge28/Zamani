//! # Zamani Frontend AST — Canonical Iterative Walker
//!
//! This module owns structural traversal of the native Zamani AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer / Parser
//!     │
//!     ▼
//! Native Zamani AST
//!     │
//!     ├── NodeId
//!     ├── Node
//!     └── source-level child relationships
//!     │
//!     ▼
//! AST storage/provider
//!     │
//!     ▼
//! ┌──────────────────────────────┐
//! │ this module: structural walk │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!          AstVisitor
//!                │
//!        ┌───────┼────────┐
//!        ▼       ▼        ▼
//!   validation  tooling  analysis
//!                │
//!                ▼
//!          Semantic Model
//!                │
//!                ▼
//!               ZUIR
//!                │
//!        ┌───────┼────────────┐
//!        ▼       ▼            ▼
//!    Classical Quantum       HDL
//!       IR       IR           IR
//! ```
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - deterministic structural traversal;
//! - iterative traversal state;
//! - pre-order enter events;
//! - post-order exit events;
//! - traversal cancellation;
//! - configurable operational traversal limits;
//! - repeated-node detection;
//! - cycle detection;
//! - traversal statistics;
//! - propagation of provider and visitor failures.
//!
//! This file does **not** own:
//!
//! - concrete AST node definitions;
//! - child relationship definitions;
//! - AST storage;
//! - parsing;
//! - semantic analysis;
//! - type checking;
//! - generic substitution;
//! - resource allocation;
//! - quantum semantics;
//! - quantum routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience;
//! - backend selection;
//! - hardware mapping;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR;
//! - runtime execution.
//!
//! ## Canonical integration boundary
//!
//! Child enumeration is owned by:
//!
//! `super::children::AstChildrenProvider`
//!
//! This file must not create another competing child-enumeration trait.
//!
//! The walker only asks the provider:
//!
//! ```text
//! NodeId → direct children in canonical source order
//! ```
//!
//! and:
//!
//! ```text
//! NodeId → canonical Node
//! ```
//!
//! The storage implementation remains replaceable.
//!
//! ## POCO-REAF
//!
//! The walker imposes no semantic limit on:
//!
//! - program size;
//! - AST node count;
//! - number of declarations;
//! - number of expressions;
//! - number of statements;
//! - number of quantum resources;
//! - number of classical resources;
//! - number of operations;
//! - machine size;
//! - qubit count;
//! - computational domain;
//! - backend;
//! - hardware topology;
//! - nesting depth.
//!
//! Traversal is iterative rather than recursively implemented in Rust.
//! Therefore logical AST depth does not consume one Rust call-stack frame per
//! AST level.
//!
//! Any configured node/edge/depth policy is an operational safety policy, not
//! a Zamani language limitation.
//!
//! ## Determinism
//!
//! Given the same:
//!
//! - AST;
//! - provider;
//! - visitor;
//! - traversal configuration;
//!
//! traversal observes the same structural ordering.
//!
//! Child ordering is supplied by `AstChildrenProvider` and must be canonical.
//! This module never obtains ordering from hash-map iteration.
//!
//! ## Tree and DAG semantics
//!
//! Native source ASTs normally behave as trees.
//!
//! The default configuration therefore rejects repeated node IDs.
//!
//! A caller may explicitly allow repeated visits when operating on a DAG-like
//! structural representation. Even in that mode, active-path cycle detection
//! remains enabled so malformed cyclic structures cannot cause unbounded
//! traversal.
//!
//! ## Safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - forbids `unsafe`;
//! - performs no I/O;
//! - executes no source-level operations;
//! - performs no hardware access;
//! - never dereferences raw pointers;
//! - never uses unchecked indexing;
//! - does not recursively traverse the AST;
//! - does not contain fixed machine-size assumptions.
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
//! - no `unsafe`.
//!
//! ## File completion contract
//!
//! Once this file is complete, adding:
//!
//! - a new declaration;
//! - a new expression;
//! - a new statement;
//! - a new type;
//! - a new resource;
//! - a new domain;
//! - a new quantum technology;
//! - a new hardware backend;
//! - a new computational target;
//!
//! must not require modifying this walker merely because that construct exists.
//!
//! Only the canonical child enumeration contract must be updated when a new
//! source-level node introduces a new structural child relationship.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::HashSet;

use super::children::AstChildrenProvider;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::visitors::visitor::{AstVisitor, VisitControl};

// =============================================================================
// Public result types
// =============================================================================

/// Result returned by an AST walk.
///
/// `PE` is the provider error type.
///
/// `VE` is the visitor error type.
pub type WalkResult<T, PE, VE> = Result<T, WalkError<PE, VE>>;

/// Errors that can terminate structural AST traversal.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WalkError<PE, VE> {
    /// The provider could not resolve a referenced node.
    MissingNode {
        /// The unresolved node identity.
        id: NodeId,
    },

    /// The provider returned a node whose identity differs from the requested
    /// identity.
    ///
    /// This indicates invalid AST storage/provider behavior.
    NodeIdentityMismatch {
        /// Identity requested from the provider.
        requested: NodeId,

        /// Identity returned by the provider.
        returned: NodeId,
    },

    /// The same node identity was encountered more than once while repeated
    /// visits were disabled.
    RevisitedNode {
        /// Revisited node.
        id: NodeId,
    },

    /// A node was encountered while it was already active on the current
    /// traversal path.
    ///
    /// This is always rejected because allowing it would permit an infinite
    /// structural walk.
    CycleDetected {
        /// Node participating in the cycle.
        id: NodeId,
    },

    /// The configured node budget was exceeded.
    ///
    /// This is an operational safety policy and is never a language-level
    /// maximum.
    NodeLimitExceeded {
        /// Number of nodes already entered.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The configured edge/child-reference budget was exceeded.
    ///
    /// This is an operational safety policy and is never a language-level
    /// maximum.
    EdgeLimitExceeded {
        /// Number of child references already inspected.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The configured logical depth budget was exceeded.
    ///
    /// This protects compiler services against hostile or accidentally
    /// pathological structures without imposing a language-level nesting
    /// limit.
    DepthLimitExceeded {
        /// Logical depth of the node that would have been entered.
        depth: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The traversal was cancelled by the caller.
    Cancelled,

    /// The AST child provider failed.
    Provider(PE),

    /// The visitor failed.
    Visitor(VE),
}

impl<PE: fmt::Display, VE: fmt::Display> fmt::Display for WalkError<PE, VE> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(formatter, "AST node {id:?} could not be resolved")
            }

            Self::NodeIdentityMismatch {
                requested,
                returned,
            } => {
                write!(
                    formatter,
                    "AST provider returned node {returned:?} for requested node {requested:?}"
                )
            }

            Self::RevisitedNode { id } => {
                write!(
                    formatter,
                    "AST node {id:?} was encountered more than once during tree traversal"
                )
            }

            Self::CycleDetected { id } => {
                write!(
                    formatter,
                    "AST traversal detected a structural cycle involving node {id:?}"
                )
            }

            Self::NodeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST traversal node limit exceeded: visited {visited}, maximum {maximum}"
                )
            }

            Self::EdgeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST traversal edge limit exceeded: visited {visited}, maximum {maximum}"
                )
            }

            Self::DepthLimitExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "AST traversal depth limit exceeded: depth {depth}, maximum {maximum}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("AST traversal was cancelled")
            }

            Self::Provider(error) => {
                write!(formatter, "AST child provider failed: {error}")
            }

            Self::Visitor(error) => {
                write!(formatter, "AST visitor failed: {error}")
            }
        }
    }
}

impl<PE, VE> std::error::Error for WalkError<PE, VE>
where
    PE: std::error::Error + 'static,
    VE: std::error::Error + 'static,
{
}

// =============================================================================
// Walk provider
// =============================================================================

/// Read-only provider required by the canonical walker.
///
/// `AstChildrenProvider` owns direct-child enumeration.
///
/// `AstWalkProvider` adds canonical node resolution.
///
/// The separation is intentional:
///
/// ```text
/// children.rs
///     │
///     └── structural child enumeration
///
/// walk.rs
///     │
///     └── traversal algorithm
///
/// AST storage
///     │
///     └── node resolution
/// ```
///
/// The provider may internally use:
///
/// - indexed storage;
/// - arenas;
/// - hash maps;
/// - persistent structures;
/// - immutable stores;
/// - incremental stores;
/// - memory-mapped structures;
/// - other future representations.
///
/// The walker does not depend on any particular storage representation.
pub trait AstWalkProvider: AstChildrenProvider {
    /// Resolves a canonical AST node.
    ///
    /// `None` means that the requested node does not exist.
    fn node(&self, id: NodeId) -> Option<&Node>;
}

// =============================================================================
// Traversal configuration
// =============================================================================

/// Operational policy for AST traversal.
///
/// All limits are optional.
///
/// `None` means that the walker does not impose that particular artificial
/// limit.
///
/// These values are never language semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkConfig {
    /// Optional maximum number of nodes entered.
    pub max_nodes: Option<usize>,

    /// Optional maximum number of direct-child references inspected.
    pub max_edges: Option<usize>,

    /// Optional maximum logical depth.
    ///
    /// Depth zero is the root.
    pub max_depth: Option<usize>,

    /// Whether repeated completed nodes are rejected.
    ///
    /// `true` is the default for native AST tree traversal.
    ///
    /// When `false`, DAG-style sharing is allowed, but cycles remain rejected.
    pub reject_revisits: bool,
}

impl Default for WalkConfig {
    fn default() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            max_depth: None,
            reject_revisits: true,
        }
    }
}

impl WalkConfig {
    /// Returns an unrestricted production traversal configuration.
    ///
    /// "Unrestricted" means that no artificial compiler-level traversal limit
    /// is imposed. The process is still naturally bounded by available
    /// resources and the executing platform.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            max_depth: None,
            reject_revisits: true,
        }
    }

    /// Sets an operational maximum number of entered nodes.
    #[must_use]
    pub const fn with_max_nodes(mut self, maximum: usize) -> Self {
        self.max_nodes = Some(maximum);
        self
    }

    /// Sets an operational maximum number of inspected child references.
    #[must_use]
    pub const fn with_max_edges(mut self, maximum: usize) -> Self {
        self.max_edges = Some(maximum);
        self
    }

    /// Sets an operational maximum logical depth.
    #[must_use]
    pub const fn with_max_depth(mut self, maximum: usize) -> Self {
        self.max_depth = Some(maximum);
        self
    }

    /// Enables or disables repeated-node rejection.
    ///
    /// Disabling this permits DAG-style node sharing but does not permit
    /// cycles.
    #[must_use]
    pub const fn reject_revisits(mut self, enabled: bool) -> Self {
        self.reject_revisits = enabled;
        self
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// External cancellation source.
///
/// The walker deliberately does not depend on any scheduler, runtime,
/// asynchronous executor, or repository-wide cancellation implementation.
pub trait Cancellation {
    /// Returns `true` when traversal should terminate.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation source that never requests cancellation.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeverCancelled;

impl Cancellation for NeverCancelled {
    #[inline]
    fn is_cancelled(&self) -> bool {
        false
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Structural statistics accumulated during traversal.
///
/// These values describe the current traversal only. They are not language
/// limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalkStatistics {
    /// Number of nodes whose `enter` event completed successfully.
    pub nodes_visited: usize,

    /// Number of child references successfully inspected.
    pub edges_visited: usize,

    /// Maximum logical depth reached.
    pub maximum_depth: usize,

    /// Number of nodes for which children were deliberately skipped.
    pub nodes_skipped: usize,
}

impl WalkStatistics {
    /// Returns the number of entered nodes.
    #[inline]
    #[must_use]
    pub const fn nodes_visited(self) -> usize {
        self.nodes_visited
    }

    /// Returns the number of inspected child references.
    #[inline]
    #[must_use]
    pub const fn edges_visited(self) -> usize {
        self.edges_visited
    }

    /// Returns the maximum logical depth reached.
    #[inline]
    #[must_use]
    pub const fn maximum_depth(self) -> usize {
        self.maximum_depth
    }

    /// Returns the number of nodes whose children were skipped.
    #[inline]
    #[must_use]
    pub const fn nodes_skipped(self) -> usize {
        self.nodes_skipped
    }
}

// =============================================================================
// Internal traversal state
// =============================================================================

/// Explicit traversal stack frame.
///
/// The walker deliberately stores the child iterator itself rather than first
/// collecting every child into a temporary vector.
///
/// This is important for scalability:
///
/// ```text
/// old approach:
/// node
///   └── collect all children → Vec<NodeId>
///                           → push children
///
/// canonical approach:
/// node
///   └── retain child iterator
///       └── consume one child at a time
/// ```
///
/// Consequently a node with a very large number of direct children does not
/// require a second temporary allocation proportional to that entire child
/// list.
enum Frame<'a, P>
where
    P: AstChildrenProvider + ?Sized,
{
    /// Enter a node.
    Enter {
        id: NodeId,
        depth: usize,
    },

    /// Continue consuming the direct children of a node.
    Children {
        id: NodeId,
        depth: usize,
        children: P::Children<'a>,
    },

    /// Exit a node after all permitted children have completed.
    Exit {
        id: NodeId,
        depth: usize,
    },
}

// =============================================================================
// Public traversal functions
// =============================================================================

/// Walks an AST using the default traversal configuration.
///
/// Traversal is:
///
/// - deterministic;
/// - iterative;
/// - source-order preserving;
/// - tree-safe;
/// - hardware-independent.
///
/// [`VisitControl::Stop`] is reported as successful completion.
pub fn walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> WalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
{
    walk_with_config_and_cancellation(
        provider,
        root,
        visitor,
        WalkConfig::default(),
        &NeverCancelled,
    )
}

/// Walks an AST using an explicit operational traversal configuration.
pub fn walk_with_config<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
) -> WalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
{
    walk_with_config_and_cancellation(
        provider,
        root,
        visitor,
        config,
        &NeverCancelled,
    )
}

/// Walks an AST using explicit configuration and an external cancellation
/// source.
///
/// This is the most general traversal API.
pub fn walk_with_config_and_cancellation<P, V, C>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
    cancellation: &C,
) -> WalkResult<WalkStatistics, P::Error, V::Error>
where
    P: AstWalkProvider,
    V: AstVisitor,
    C: Cancellation,
{
    let root_node = provider
        .node(root)
        .ok_or(WalkError::MissingNode { id: root })?;

    if root_node.id() != root {
        return Err(WalkError::NodeIdentityMismatch {
            requested: root,
            returned: root_node.id(),
        });
    }

    let mut frames = Vec::new();

    frames.push(Frame::Enter {
        id: root,
        depth: 0,
    });

    let mut visited = HashSet::<NodeId>::new();
    let mut active = HashSet::<NodeId>::new();

    let mut statistics = WalkStatistics::default();

    while let Some(frame) = frames.pop() {
        if cancellation.is_cancelled() || visitor.should_cancel() {
            return Err(WalkError::Cancelled);
        }

        match frame {
            Frame::Enter { id, depth } => {
                if let Some(maximum) = config.max_depth {
                    if depth > maximum {
                        return Err(WalkError::DepthLimitExceeded {
                            depth,
                            maximum,
                        });
                    }
                }

                if config.reject_revisits && !visited.insert(id) {
                    return Err(WalkError::RevisitedNode { id });
                }

                if !active.insert(id) {
                    return Err(WalkError::CycleDetected { id });
                }

                if let Some(maximum) = config.max_nodes {
                    if statistics.nodes_visited >= maximum {
                        active.remove(&id);

                        return Err(WalkError::NodeLimitExceeded {
                            visited: statistics.nodes_visited,
                            maximum,
                        });
                    }
                }

                let node = match provider.node(id) {
                    Some(node) => node,

                    None => {
                        active.remove(&id);

                        return Err(WalkError::MissingNode { id });
                    }
                };

                if node.id() != id {
                    active.remove(&id);

                    return Err(WalkError::NodeIdentityMismatch {
                        requested: id,
                        returned: node.id(),
                    });
                }

                statistics.nodes_visited = statistics
                    .nodes_visited
                    .checked_add(1)
                    .ok_or(WalkError::NodeLimitExceeded {
                        visited: usize::MAX,
                        maximum: usize::MAX,
                    })?;

                statistics.maximum_depth =
                    statistics.maximum_depth.max(depth);

                let control = visitor
                    .enter(node, depth)
                    .map_err(WalkError::Visitor)?;

                match control {
                    VisitControl::Stop => {
                        active.remove(&id);

                        // Stop is explicitly successful according to the
                        // canonical AstVisitor contract.
                        return Ok(statistics);
                    }

                    VisitControl::SkipChildren => {
                        statistics.nodes_skipped = statistics
                            .nodes_skipped
                            .checked_add(1)
                            .ok_or(WalkError::NodeLimitExceeded {
                                visited: statistics.nodes_visited,
                                maximum: usize::MAX,
                            })?;

                        // Exit remains scheduled so enter/exit symmetry is
                        // preserved for a skipped subtree.
                        frames.push(Frame::Exit { id, depth });
                    }

                    VisitControl::Continue => {
                        let children = provider
                            .children(id)
                            .map_err(WalkError::Provider)?;

                        // The exit event must occur after all permitted
                        // children.
                        frames.push(Frame::Exit { id, depth });

                        // Keep the provider's iterator alive on the explicit
                        // traversal stack. This avoids collecting all direct
                        // children into a temporary Vec.
                        frames.push(Frame::Children {
                            id,
                            depth,
                            children,
                        });
                    }
                }
            }

            Frame::Children {
                id,
                depth,
                mut children,
            } => {
                if cancellation.is_cancelled() || visitor.should_cancel() {
                    return Err(WalkError::Cancelled);
                }

                let child_id = match children.next() {
                    Some(child_id) => child_id,

                    None => {
                        // No more children. The corresponding Exit frame is
                        // already below this frame on the stack.
                        continue;
                    }
                };

                if let Some(maximum) = config.max_edges {
                    if statistics.edges_visited >= maximum {
                        return Err(WalkError::EdgeLimitExceeded {
                            visited: statistics.edges_visited,
                            maximum,
                        });
                    }
                }

                statistics.edges_visited = statistics
                    .edges_visited
                    .checked_add(1)
                    .ok_or(WalkError::EdgeLimitExceeded {
                        visited: usize::MAX,
                        maximum: usize::MAX,
                    })?;

                // Resume the current parent's iterator after this child has
                // completed.
                frames.push(Frame::Children {
                    id,
                    depth,
                    children,
                });

                let child_depth = depth.checked_add(1).ok_or(
                    WalkError::DepthLimitExceeded {
                        depth: usize::MAX,
                        maximum: config.max_depth.unwrap_or(usize::MAX),
                    },
                )?;

                frames.push(Frame::Enter {
                    id: child_id,
                    depth: child_depth,
                });
            }

            Frame::Exit { id, depth } => {
                if cancellation.is_cancelled() || visitor.should_cancel() {
                    return Err(WalkError::Cancelled);
                }

                let node = provider
                    .node(id)
                    .ok_or(WalkError::MissingNode { id })?;

                if node.id() != id {
                    return Err(WalkError::NodeIdentityMismatch {
                        requested: id,
                        returned: node.id(),
                    });
                }

                visitor
                    .exit(node, depth)
                    .map_err(WalkError::Visitor)?;

                active.remove(&id);
            }
        }
    }

    Ok(statistics)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::convert::Infallible;

    use crate::frontend::ast::node::node::Node;
    use crate::frontend::ast::node::node_id::NodeId;

    // -------------------------------------------------------------------------
    // Test provider
    // -------------------------------------------------------------------------

    struct TestProvider {
        nodes: HashMap<NodeId, Node>,
        children: HashMap<NodeId, Vec<NodeId>>,
        empty: Vec<NodeId>,
    }

    impl TestProvider {
        fn new() -> Self {
            Self {
                nodes: HashMap::new(),
                children: HashMap::new(),
                empty: Vec::new(),
            }
        }

        fn add_node(&mut self, id: NodeId) {
            self.nodes.insert(id, Node::default());
        }

        fn set_children(&mut self, id: NodeId, children: Vec<NodeId>) {
            self.children.insert(id, children);
        }
    }

    impl AstChildrenProvider for TestProvider {
        type Error = Infallible;

        type Children<'a>
            = std::slice::Iter<'a, NodeId>
        where
            Self: 'a;

        fn children<'a>(
            &'a self,
            node_id: NodeId,
        ) -> Result<Self::Children<'a>, Self::Error> {
            Ok(self
                .children
                .get(&node_id)
                .unwrap_or(&self.empty)
                .iter())
        }
    }

    impl AstWalkProvider for TestProvider {
        fn node(&self, id: NodeId) -> Option<&Node> {
            self.nodes.get(&id)
        }
    }

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be non-zero")
    }

    // -------------------------------------------------------------------------
    // Test visitor
    // -------------------------------------------------------------------------

    #[derive(Default)]
    struct RecordingVisitor {
        events: Vec<(NodeId, bool, usize)>,
        stop_after: Option<usize>,
        entered: usize,
    }

    impl AstVisitor for RecordingVisitor {
        type Error = Infallible;

        fn enter(
            &mut self,
            node: &Node,
            depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.entered += 1;

            self.events.push((node.id(), true, depth));

            if let Some(limit) = self.stop_after {
                if self.entered >= limit {
                    return Ok(VisitControl::Stop);
                }
            }

            Ok(VisitControl::Continue)
        }

        fn exit(
            &mut self,
            node: &Node,
            depth: usize,
        ) -> Result<(), Self::Error> {
            self.events.push((node.id(), false, depth));
            Ok(())
        }
    }

    // -------------------------------------------------------------------------
    // Basic traversal
    // -------------------------------------------------------------------------

    #[test]
    fn walks_children_in_source_order() {
        let root = node_id(1);
        let first = node_id(2);
        let second = node_id(3);
        let third = node_id(4);

        let mut provider = TestProvider::new();

        for id in [root, first, second, third] {
            provider.add_node(id);
        }

        provider.set_children(root, vec![first, second, third]);

        let mut visitor = RecordingVisitor::default();

        let result = walk(&provider, root, &mut visitor)
            .expect("walk should succeed");

        assert_eq!(result.nodes_visited, 4);
        assert_eq!(result.edges_visited, 3);

        assert_eq!(
            visitor.events,
            vec![
                (root, true, 0),
                (first, true, 1),
                (first, false, 1),
                (second, true, 1),
                (second, false, 1),
                (third, true, 1),
                (third, false, 1),
                (root, false, 0),
            ]
        );
    }

    // -------------------------------------------------------------------------
    // Empty tree
    // -------------------------------------------------------------------------

    #[test]
    fn walks_single_node() {
        let root = node_id(1);

        let mut provider = TestProvider::new();
        provider.add_node(root);

        let mut visitor = RecordingVisitor::default();

        let result = walk(&provider, root, &mut visitor)
            .expect("walk should succeed");

        assert_eq!(result.nodes_visited, 1);
        assert_eq!(result.edges_visited, 0);
        assert_eq!(result.maximum_depth, 0);
    }

    // -------------------------------------------------------------------------
    // Skip children
    // -------------------------------------------------------------------------

    struct SkipVisitor;

    impl AstVisitor for SkipVisitor {
        type Error = Infallible;

        fn enter(
            &mut self,
            node: &Node,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            if node.id() == node_id(1) {
                Ok(VisitControl::SkipChildren)
            } else {
                Ok(VisitControl::Continue)
            }
        }
    }

    #[test]
    fn skip_children_does_not_descend() {
        let root = node_id(1);
        let child = node_id(2);

        let mut provider = TestProvider::new();
        provider.add_node(root);
        provider.add_node(child);
        provider.set_children(root, vec![child]);

        let mut visitor = SkipVisitor;

        let result = walk(&provider, root, &mut visitor)
            .expect("walk should succeed");

        assert_eq!(result.nodes_visited, 1);
        assert_eq!(result.edges_visited, 0);
        assert_eq!(result.nodes_skipped, 1);
    }

    // -------------------------------------------------------------------------
    // Stop is successful
    // -------------------------------------------------------------------------

    #[test]
    fn stop_is_successful_early_termination() {
        let root = node_id(1);
        let child = node_id(2);

        let mut provider = TestProvider::new();
        provider.add_node(root);
        provider.add_node(child);
        provider.set_children(root, vec![child]);

        let mut visitor = RecordingVisitor {
            stop_after: Some(1),
            ..RecordingVisitor::default()
        };

        let result = walk(&provider, root, &mut visitor)
            .expect("Stop must be successful");

        assert_eq!(result.nodes_visited, 1);
        assert_eq!(
            visitor.events,
            vec![(root, true, 0)]
        );
    }

    // -------------------------------------------------------------------------
    // Missing root
    // -------------------------------------------------------------------------

    #[test]
    fn missing_root_is_reported() {
        let provider = TestProvider::new();

        let mut visitor = RecordingVisitor::default();

        let error = walk(
            &provider,
            node_id(1),
            &mut visitor,
        )
        .expect_err("missing root must fail");

        assert_eq!(
            error,
            WalkError::MissingNode {
                id: node_id(1),
            }
        );
    }

    // -------------------------------------------------------------------------
    // Repeated node
    // -------------------------------------------------------------------------

    #[test]
    fn repeated_node_is_rejected_by_default() {
        let root = node_id(1);
        let shared = node_id(2);
        let left = node_id(3);
        let right = node_id(4);

        let mut provider = TestProvider::new();

        for id in [root, shared, left, right] {
            provider.add_node(id);
        }

        provider.set_children(root, vec![left, right]);
        provider.set_children(left, vec![shared]);
        provider.set_children(right, vec![shared]);

        let mut visitor = RecordingVisitor::default();

        let error = walk(
            &provider,
            root,
            &mut visitor,
        )
        .expect_err("shared node must be rejected in tree mode");

        assert_eq!(
            error,
            WalkError::RevisitedNode { id: shared }
        );
    }

    // -------------------------------------------------------------------------
    // DAG sharing
    // -------------------------------------------------------------------------

    #[test]
    fn repeated_nodes_can_be_allowed_explicitly() {
        let root = node_id(1);
        let shared = node_id(2);
        let left = node_id(3);
        let right = node_id(4);

        let mut provider = TestProvider::new();

        for id in [root, shared, left, right] {
            provider.add_node(id);
        }

        provider.set_children(root, vec![left, right]);
        provider.set_children(left, vec![shared]);
        provider.set_children(right, vec![shared]);

        let mut visitor = RecordingVisitor::default();

        let result = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().reject_revisits(false),
        )
        .expect("DAG sharing should be allowed");

        assert_eq!(result.nodes_visited, 5);
    }

    // -------------------------------------------------------------------------
    // Cycle detection
    // -------------------------------------------------------------------------

    #[test]
    fn cycles_are_rejected_even_when_revisits_are_allowed() {
        let first = node_id(1);
        let second = node_id(2);

        let mut provider = TestProvider::new();

        provider.add_node(first);
        provider.add_node(second);

        provider.set_children(first, vec![second]);
        provider.set_children(second, vec![first]);

        let mut visitor = RecordingVisitor::default();

        let error = walk_with_config(
            &provider,
            first,
            &mut visitor,
            WalkConfig::default().reject_revisits(false),
        )
        .expect_err("cycles must never be traversed indefinitely");

        assert_eq!(
            error,
            WalkError::CycleDetected { id: first }
        );
    }

    // -------------------------------------------------------------------------
    // Node limit
    // -------------------------------------------------------------------------

    #[test]
    fn node_limit_is_operational_and_configurable() {
        let root = node_id(1);
        let child = node_id(2);

        let mut provider = TestProvider::new();
        provider.add_node(root);
        provider.add_node(child);
        provider.set_children(root, vec![child]);

        let mut visitor = RecordingVisitor::default();

        let error = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().with_max_nodes(1),
        )
        .expect_err("node limit must be enforced");

        assert_eq!(
            error,
            WalkError::NodeLimitExceeded {
                visited: 1,
                maximum: 1,
            }
        );
    }

    // -------------------------------------------------------------------------
    // Edge limit
    // -------------------------------------------------------------------------

    #[test]
    fn edge_limit_is_operational_and_configurable() {
        let root = node_id(1);
        let first = node_id(2);
        let second = node_id(3);

        let mut provider = TestProvider::new();

        for id in [root, first, second] {
            provider.add_node(id);
        }

        provider.set_children(root, vec![first, second]);

        let mut visitor = RecordingVisitor::default();

        let error = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().with_max_edges(1),
        )
        .expect_err("edge limit must be enforced");

        assert_eq!(
            error,
            WalkError::EdgeLimitExceeded {
                visited: 1,
                maximum: 1,
            }
        );
    }

    // -------------------------------------------------------------------------
    // Depth limit
    // -------------------------------------------------------------------------

    #[test]
    fn depth_limit_is_operational_and_configurable() {
        let first = node_id(1);
        let second = node_id(2);

        let mut provider = TestProvider::new();
        provider.add_node(first);
        provider.add_node(second);
        provider.set_children(first, vec![second]);

        let mut visitor = RecordingVisitor::default();

        let error = walk_with_config(
            &provider,
            first,
            &mut visitor,
            WalkConfig::default().with_max_depth(0),
        )
        .expect_err("depth limit must be enforced");

        assert_eq!(
            error,
            WalkError::DepthLimitExceeded {
                depth: 1,
                maximum: 0,
            }
        );
    }

    // -------------------------------------------------------------------------
    // Deep iterative traversal
    // -------------------------------------------------------------------------

    #[test]
    fn deeply_nested_tree_does_not_require_recursive_rust_calls() {
        const DEPTH: u64 = 10_000;

        let mut provider = TestProvider::new();

        for value in 1..=DEPTH {
            provider.add_node(node_id(value));
        }

        for value in 1..DEPTH {
            provider.set_children(
                node_id(value),
                vec![node_id(value + 1)],
            );
        }

        let mut visitor = RecordingVisitor::default();

        let result = walk(
            &provider,
            node_id(1),
            &mut visitor,
        )
        .expect("deep iterative traversal should succeed");

        assert_eq!(result.nodes_visited, DEPTH as usize);
        assert_eq!(result.edges_visited, (DEPTH - 1) as usize);
        assert_eq!(result.maximum_depth, (DEPTH - 1) as usize);
    }

    // -------------------------------------------------------------------------
    // Cancellation
    // -------------------------------------------------------------------------

    struct CancelImmediately;

    impl Cancellation for CancelImmediately {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    #[test]
    fn external_cancellation_is_reported() {
        let root = node_id(1);

        let mut provider = TestProvider::new();
        provider.add_node(root);

        let mut visitor = RecordingVisitor::default();

        let error = walk_with_config_and_cancellation(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default(),
            &CancelImmediately,
        )
        .expect_err("cancellation must terminate traversal");

        assert_eq!(error, WalkError::Cancelled);
    }
}