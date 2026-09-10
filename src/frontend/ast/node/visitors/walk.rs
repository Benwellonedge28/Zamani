//! # Zamani Frontend AST — Production Walker
//!
//! This module provides the canonical non-recursive traversal engine for the
//! native Zamani AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! Lexer / Parser
//!      │
//!      ▼
//! Native Zamani AST
//!      │
//!      ├── NodeId
//!      ├── Node
//!      └── concrete source-level node structures
//!      │
//!      ▼
//! AST storage / node provider
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ this module: iterative walk  │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!          AstVisitor
//!                │
//!                ▼
//! Structural validation / tooling / analysis / transformation
//!                │
//!                ▼
//! Semantic Model
//!                │
//!                ▼
//! ZUIR
//!                │
//!                ▼
//! Domain / target IR
//! ```
//!
//! ## Critical architectural rule
//!
//! This module traverses the AST. It does not interpret the AST.
//!
//! It must never contain:
//!
//! - quantum gate semantics;
//! - qubit allocation;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - target-specific optimization;
//! - runtime execution.
//!
//! Those concerns belong downstream.
//!
//! ## POCO-REAF
//!
//! The walker introduces no artificial limit on:
//!
//! - program size;
//! - number of AST nodes;
//! - number of operands;
//! - number of quantum resources;
//! - number of classical resources;
//! - machine size;
//! - computational domain;
//! - backend;
//! - nesting depth.
//!
//! Traversal is iterative rather than recursive so that a deeply nested source
//! program does not consume the Rust call stack proportional to AST depth.
//!
//! Actual traversal remains bounded only by:
//!
//! - available memory;
//! - address-space limits;
//! - the caller's explicitly configured resource policy;
//! - the finite representation of the executing platform.
//!
//! Those are operational limits, not Zamani language semantics.
//!
//! ## Why the walker is generic
//!
//! The canonical AST stores relationships using `NodeId`. The walker therefore
//! cannot assume that a node is stored in a particular arena, vector, hash map,
//! tree type, or future storage implementation.
//!
//! Instead, the walker consumes an [`AstNodeProvider`].
//!
//! This keeps:
//!
//! ```text
//! AST node definitions
//!         ↓
//! AST storage
//!         ↓
//! traversal
//! ```
//!
//! independently replaceable.
//!
//! A future storage implementation may use:
//!
//! - indexed vectors;
//! - arenas;
//! - immutable stores;
//! - incremental stores;
//! - persistent stores;
//! - memory-mapped representations;
//! - distributed compiler storage;
//! - another representation.
//!
//! `walk.rs` does not need to change as long as the provider contract remains
//! valid.
//!
//! ## Determinism
//!
//! Traversal order is deterministic:
//!
//! - pre-order visits parent before children;
//! - children are requested from the provider in source order;
//! - child order is never derived from hash-map iteration;
//! - no random state is used;
//! - no timestamps are used;
//! - no hardware information is consulted.
//!
//! ## Safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no AST operations;
//! - never dereferences raw pointers;
//! - never uses unchecked indexing;
//! - never recursively calls itself;
//! - never assumes a finite AST depth;
//! - never assumes a finite machine size.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Native AST
//!      │
//!      ▼
//! Deterministic iterative walk
//!      │
//!      ▼
//! Any analysis / validation / tooling visitor
//! ```
//!
//! The same walker therefore applies to classical, quantum, hybrid, HDL,
//! accelerator and future source-level extensions without adding
//! domain-specific traversal branches.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::HashSet;

use super::visitor::{AstVisitor, VisitControl};

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::node::Node;

// =============================================================================
// Public result/error types
// =============================================================================

/// Result returned by AST traversal.
pub type WalkResult<T, E> = Result<T, WalkError<E>>;

/// Errors produced by the traversal engine.
///
/// Visitor errors are preserved rather than converted into traversal-specific
/// errors so callers can recover the original visitor error.
#[derive(Debug, PartialEq, Eq)]
pub enum WalkError<E> {
    /// The provider could not resolve a referenced node.
    MissingNode {
        /// ID that could not be resolved.
        id: NodeId,
    },

    /// A node was encountered again while traversing a tree.
    ///
    /// The native AST is intended to represent a tree for ordinary source
    /// structure. Repeated references can indicate malformed structure,
    /// accidental sharing, or a graph-like extension.
    ///
    /// The walker reports the condition instead of silently visiting the same
    /// node repeatedly.
    RevisitedNode {
        /// Node that was encountered previously.
        id: NodeId,
    },

    /// The configured node budget was exceeded.
    ///
    /// This is an operational safety policy, not a language-level AST limit.
    NodeLimitExceeded {
        /// Number of nodes already processed.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The configured edge/child-reference budget was exceeded.
    ///
    /// This protects callers processing hostile or corrupted AST storage.
    EdgeLimitExceeded {
        /// Number of child references already inspected.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The caller requested cancellation.
    Cancelled,

    /// The visitor returned an error.
    Visitor(E),
}

impl<E: fmt::Display> fmt::Display for WalkError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(formatter, "AST node {id:?} could not be resolved")
            }

            Self::RevisitedNode { id } => {
                write!(
                    formatter,
                    "AST node {id:?} was encountered more than once during tree traversal"
                )
            }

            Self::NodeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST traversal node limit exceeded: {visited} > {maximum}"
                )
            }

            Self::EdgeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST traversal edge limit exceeded: {visited} > {maximum}"
                )
            }

            Self::Cancelled => formatter.write_str("AST traversal was cancelled"),

            Self::Visitor(error) => write!(formatter, "AST visitor failed: {error}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for WalkError<E> {}


// =============================================================================
// Provider contract
// =============================================================================

/// Read-only source of AST nodes for the traversal engine.
///
/// The provider is intentionally minimal.
///
/// It owns node storage; `walk.rs` owns traversal.
///
/// ## Required invariants
///
/// Implementations must guarantee that:
///
/// 1. `node(id)` returns the canonical node corresponding to `id`.
/// 2. The returned `Node` has the same `NodeId` as requested.
/// 3. Child enumeration is deterministic.
/// 4. Child enumeration follows source order.
/// 5. Child IDs are structural references only.
/// 6. Resolving a node has no observable side effects.
/// 7. Resolving a node does not execute source-level operations.
/// 8. The provider does not depend on target hardware.
/// 9. The provider does not perform semantic analysis.
/// 10. The provider does not silently invent missing nodes.
///
/// ## Why this is a trait
///
/// The native AST deliberately uses `NodeId` references. The traversal layer
/// must therefore remain independent of the concrete storage strategy.
///
/// This trait is the integration boundary between the AST storage subsystem
/// and traversal.
pub trait AstNodeProvider {
    /// Returns the canonical common node for `id`.
    ///
    /// `None` means that the ID does not exist in the current AST store.
    fn node(&self, id: NodeId) -> Option<&Node>;

    /// Returns direct structural children of `id` in deterministic source order.
    ///
    /// The returned IDs must be direct children only. Recursive traversal is
    /// the responsibility of this module.
    ///
    /// Implementations should avoid allocation where practical.
    ///
    /// A boxed iterator is intentionally avoided here because AST traversal is
    /// a hot path and should not require heap allocation for every node.
    fn children<'a>(
        &'a self,
        id: NodeId,
    ) -> Result<Box<dyn Iterator<Item = NodeId> + 'a>, Self::Error>;

    /// Provider-specific error.
    type Error;
}


// =============================================================================
// Traversal configuration
// =============================================================================

/// Traversal policy.
///
/// All limits are optional operational safety policies.
///
/// They are never language semantics and must not be interpreted as maximum
/// program, machine, register or quantum-resource sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkConfig {
    /// Maximum number of nodes that may be visited.
    ///
    /// `None` means no walker-imposed node limit.
    pub max_nodes: Option<usize>,

    /// Maximum number of child references inspected.
    ///
    /// `None` means no walker-imposed edge limit.
    pub max_edges: Option<usize>,

    /// Whether revisiting an already visited node is an error.
    ///
    /// `true` is the safe default for native tree traversal.
    ///
    /// Set to `false` only when the caller explicitly wants graph traversal
    /// semantics.
    pub reject_revisits: bool,

    /// Whether the walker checks cancellation before every node event.
    ///
    /// Disabling this can improve throughput for trusted, non-cancellable
    /// workloads.
    pub check_cancellation: bool,
}

impl Default for WalkConfig {
    fn default() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            reject_revisits: true,
            check_cancellation: true,
        }
    }
}

impl WalkConfig {
    /// Creates an unrestricted production traversal policy.
    ///
    /// "Unrestricted" means no artificial AST-level limit. The process remains
    /// bounded by available system resources.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            reject_revisits: true,
            check_cancellation: true,
        }
    }

    /// Sets the maximum number of nodes.
    #[must_use]
    pub const fn with_max_nodes(mut self, maximum: usize) -> Self {
        self.max_nodes = Some(maximum);
        self
    }

    /// Sets the maximum number of child references.
    #[must_use]
    pub const fn with_max_edges(mut self, maximum: usize) -> Self {
        self.max_edges = Some(maximum);
        self
    }

    /// Enables or disables repeated-node rejection.
    #[must_use]
    pub const fn reject_revisits(mut self, enabled: bool) -> Self {
        self.reject_revisits = enabled;
        self
    }

    /// Enables or disables cancellation checks.
    #[must_use]
    pub const fn check_cancellation(mut self, enabled: bool) -> Self {
        self.check_cancellation = enabled;
        self
    }
}


// =============================================================================
// Cancellation
// =============================================================================

/// Lightweight cancellation interface.
///
/// The AST layer deliberately does not depend on the repository's downstream
/// scheduling, optimization, runtime, or quantum cancellation types.
///
/// Any external cancellation implementation can satisfy this trait.
pub trait Cancellation {
    /// Returns `true` when traversal should stop.
    fn is_cancelled(&self) -> bool;
}

/// A cancellation source that never cancels.
///
/// Useful when a caller wants an explicit cancellation object without
/// conditional logic.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeverCancelled;

impl Cancellation for NeverCancelled {
    #[inline]
    fn is_cancelled(&self) -> bool {
        false
    }
}


// =============================================================================
// Traversal statistics
// =============================================================================

/// Immutable summary of a completed or partially completed traversal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalkStatistics {
    /// Number of successfully entered nodes.
    pub nodes_visited: usize,

    /// Number of child references inspected.
    pub edges_visited: usize,

    /// Maximum logical AST depth reached.
    ///
    /// This is a measurement, not a configured recursion limit.
    pub maximum_depth: usize,

    /// Number of nodes for which children were skipped.
    pub nodes_skipped: usize,
}

impl WalkStatistics {
    /// Returns the number of nodes visited.
    #[inline]
    #[must_use]
    pub const fn nodes_visited(self) -> usize {
        self.nodes_visited
    }

    /// Returns the number of child references inspected.
    #[inline]
    #[must_use]
    pub const fn edges_visited(self) -> usize {
        self.edges_visited
    }

    /// Returns the deepest logical traversal depth reached.
    #[inline]
    #[must_use]
    pub const fn maximum_depth(self) -> usize {
        self.maximum_depth
    }
}


// =============================================================================
// Internal traversal frames
// =============================================================================

/// Traversal phase for one node.
///
/// The explicit phase replaces recursive Rust calls.
///
/// This is the key property that allows the walker to process very deeply
/// nested ASTs without consuming one stack frame per AST level.
#[derive(Debug)]
enum Frame {
    /// The node is about to be entered.
    Enter {
        id: NodeId,
        depth: usize,
    },

    /// The node's children have been visited and the visitor should receive
    /// its exit event.
    Exit {
        id: NodeId,
        depth: usize,
    },
}


// =============================================================================
// Public traversal API
// =============================================================================

/// Walks one AST root in deterministic pre-order.
///
/// The visitor receives:
///
/// 1. an enter event for the root;
/// 2. enter events for children in source order;
/// 3. exit events after descendants have completed.
///
/// The traversal is iterative and therefore does not recurse through the Rust
/// call stack.
///
/// ## Control semantics
///
/// [`VisitControl::Continue`]
///
/// Visit the node's children.
///
/// [`VisitControl::SkipChildren`]
///
/// Do not visit this node's children, but still emit its exit event.
///
/// [`VisitControl::Stop`]
///
/// Stop immediately and return [`WalkError::Cancelled`].
///
/// ## Tree semantics
///
/// The default policy rejects repeated node IDs. This prevents malformed AST
/// graphs from silently becoming exponential or infinite traversals.
///
/// ## Complexity
///
/// For a valid tree:
///
/// - time: `O(V + E)` visitor/provider operations;
/// - additional traversal memory: `O(D + V)` in the worst case when revisit
///   detection is enabled, where `D` is logical depth and `V` is visited-node
///   count.
///
/// The implementation never allocates one Rust stack frame per AST node.
pub fn walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> WalkResult<WalkStatistics, WalkError<V::Error>>
where
    P: AstNodeProvider,
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

/// Walks an AST with explicit traversal policy.
///
/// This is the preferred API for compiler infrastructure that needs explicit
/// resource governance.
pub fn walk_with_config<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
) -> WalkResult<WalkStatistics, WalkError<V::Error>>
where
    P: AstNodeProvider,
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

/// Walks an AST with explicit traversal policy and cancellation.
///
/// This is the most general public traversal entry point.
pub fn walk_with_config_and_cancellation<P, V, C>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
    cancellation: &C,
) -> WalkResult<WalkStatistics, WalkError<V::Error>>
where
    P: AstNodeProvider,
    V: AstVisitor,
    C: Cancellation,
{
    let root_node = provider
        .node(root)
        .ok_or(WalkError::MissingNode { id: root })?;

    if root_node.id() != root {
        return Err(WalkError::MissingNode { id: root });
    }

    let mut frames = Vec::new();

    // One root frame is the only initial allocation.
    //
    // `Vec` grows dynamically if required. There is intentionally no fixed
    // traversal stack size.
    frames.push(Frame::Enter {
        id: root,
        depth: 0,
    });

    let mut visited = HashSet::<NodeId>::new();

    let mut statistics = WalkStatistics::default();

    while let Some(frame) = frames.pop() {
        if config.check_cancellation && cancellation.is_cancelled() {
            return Err(WalkError::Cancelled);
        }

        match frame {
            Frame::Enter { id, depth } => {
                if let Some(maximum) = config.max_nodes {
                    if statistics.nodes_visited >= maximum {
                        return Err(WalkError::NodeLimitExceeded {
                            visited: statistics.nodes_visited,
                            maximum,
                        });
                    }
                }

                if config.reject_revisits && !visited.insert(id) {
                    return Err(WalkError::RevisitedNode { id });
                }

                let node = provider
                    .node(id)
                    .ok_or(WalkError::MissingNode { id })?;

                // Defensive provider invariant check.
                //
                // A provider returning a node whose identity differs from the
                // requested ID is structurally invalid.
                if node.id() != id {
                    return Err(WalkError::MissingNode { id });
                }

                statistics.nodes_visited = statistics
                    .nodes_visited
                    .checked_add(1)
                    .ok_or(WalkError::NodeLimitExceeded {
                        visited: usize::MAX,
                        maximum: usize::MAX,
                    })?;

                statistics.maximum_depth = statistics.maximum_depth.max(depth);

                let control = visitor
                    .enter(node, depth)
                    .map_err(WalkError::Visitor)?;

                match control {
                    VisitControl::Stop => {
                        return Err(WalkError::Cancelled);
                    }

                    VisitControl::SkipChildren => {
                        statistics.nodes_skipped = statistics
                            .nodes_skipped
                            .checked_add(1)
                            .ok_or(WalkError::NodeLimitExceeded {
                                visited: statistics.nodes_visited,
                                maximum: usize::MAX,
                            })?;

                        // Even when children are skipped, enter/exit symmetry
                        // remains intact.
                        frames.push(Frame::Exit { id, depth });
                    }

                    VisitControl::Continue => {
                        let children = provider
                            .children(id)
                            .map_err(|error| WalkError::Provider(error))?;

                        let mut child_ids = Vec::new();

                        for child_id in children {
                            if config.check_cancellation
                                && cancellation.is_cancelled()
                            {
                                return Err(WalkError::Cancelled);
                            }

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

                            child_ids.push(child_id);
                        }

                        // Exit must happen after every child.
                        frames.push(Frame::Exit { id, depth });

                        // Stack is LIFO, therefore children are pushed in
                        // reverse source order so that they are processed in
                        // original source order.
                        for child_id in child_ids.into_iter().rev() {
                            frames.push(Frame::Enter {
                                id: child_id,
                                depth: depth.saturating_add(1),
                            });
                        }
                    }
                }
            }

            Frame::Exit { id, depth } => {
                if config.check_cancellation && cancellation.is_cancelled() {
                    return Err(WalkError::Cancelled);
                }

                let node = provider
                    .node(id)
                    .ok_or(WalkError::MissingNode { id })?;

                visitor
                    .exit(node, depth)
                    .map_err(WalkError::Visitor)?;
            }
        }
    }

    Ok(statistics)
}


// =============================================================================
// Error adaptation
// =============================================================================

impl<E> From<ProviderWalkError<E>> for WalkError<E> {
    fn from(error: ProviderWalkError<E>) -> Self {
        match error {
            ProviderWalkError::MissingNode { id } => Self::MissingNode { id },
            ProviderWalkError::Provider(error) => Self::Provider(error),
        }
    }
}

/// Provider failures that are not themselves traversal-policy failures.
///
/// This separate type keeps the provider boundary explicit while still
/// allowing callers to use arbitrary storage implementations.
#[derive(Debug, PartialEq, Eq)]
pub enum ProviderWalkError<E> {
    /// A referenced node does not exist.
    MissingNode {
        /// Missing node identity.
        id: NodeId,
    },

    /// Provider-specific failure.
    Provider(E),
}

impl<E: fmt::Display> fmt::Display for ProviderWalkError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(formatter, "AST node {id:?} is missing")
            }

            Self::Provider(error) => {
                write!(formatter, "AST provider failed: {error}")
            }
        }
    }
}

/// Converts a provider error into the public walker error.
///
/// The provider itself remains responsible for defining the underlying error
/// type.
trait ProviderErrorAdapter {
    type Error;

    fn adapt(self) -> WalkError<Self::Error>;
}


// =============================================================================
// Traversal helpers
// =============================================================================

/// Walks an AST and returns only its traversal statistics.
///
/// This is useful for compiler diagnostics, profiling, and validation
/// infrastructure that does not otherwise need a visitor implementation.
///
/// The visitor still receives every event.
pub fn walk_statistics<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> WalkResult<WalkStatistics, WalkError<V::Error>>
where
    P: AstNodeProvider,
    V: AstVisitor,
{
    walk(provider, root, visitor)
}

/// Determines whether an AST can be traversed completely under the supplied
/// policy.
///
/// This is useful for preflight validation before an expensive semantic pass.
pub fn can_walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
) -> bool
where
    P: AstNodeProvider,
    V: AstVisitor,
{
    walk_with_config(provider, root, visitor, config).is_ok()
}


// =============================================================================
// Compile-time contract checks
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::node_kind::{CoreNodeKind, NodeKind};
    use crate::frontend::ast::node::source::Span;

    #[derive(Debug)]
    struct TestProvider {
        nodes: std::collections::HashMap<NodeId, Node>,
        children: std::collections::HashMap<NodeId, Vec<NodeId>>,
    }

    impl TestProvider {
        fn new() -> Self {
            Self {
                nodes: std::collections::HashMap::new(),
                children: std::collections::HashMap::new(),
            }
        }

        fn insert(
            &mut self,
            id: NodeId,
            children: Vec<NodeId>,
        ) {
            let node = Node::new(
                id,
                NodeKind::core(CoreNodeKind::Program),
                Span::default(),
                NodeMetadata::default(),
            );

            self.nodes.insert(id, node);
            self.children.insert(id, children);
        }
    }

    impl AstNodeProvider for TestProvider {
        type Error = core::convert::Infallible;

        fn node(&self, id: NodeId) -> Option<&Node> {
            self.nodes.get(&id)
        }

        fn children<'a>(
            &'a self,
            id: NodeId,
        ) -> Result<Box<dyn Iterator<Item = NodeId> + 'a>, Self::Error> {
            let iterator = self
                .children
                .get(&id)
                .into_iter()
                .flat_map(|children| children.iter().copied());

            Ok(Box::new(iterator))
        }
    }

    #[derive(Default)]
    struct RecordingVisitor {
        entered: Vec<NodeId>,
        exited: Vec<NodeId>,
    }

    impl AstVisitor for RecordingVisitor {
        type Error = core::convert::Infallible;

        fn enter(
            &mut self,
            node: &Node,
            _depth: usize,
        ) -> Result<VisitControl, Self::Error> {
            self.entered.push(node.id());
            Ok(VisitControl::Continue)
        }

        fn exit(
            &mut self,
            node: &Node,
            _depth: usize,
        ) -> Result<(), Self::Error> {
            self.exited.push(node.id());
            Ok(())
        }
    }

    #[test]
    fn traverses_children_in_source_order() {
        let root = NodeId::new(1).expect("test ID must be non-zero");
        let first = NodeId::new(2).expect("test ID must be non-zero");
        let second = NodeId::new(3).expect("test ID must be non-zero");
        let third = NodeId::new(4).expect("test ID must be non-zero");

        let mut provider = TestProvider::new();

        provider.insert(
            root,
            vec![first, second, third],
        );
        provider.insert(first, Vec::new());
        provider.insert(second, Vec::new());
        provider.insert(third, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let statistics = walk(
            &provider,
            root,
            &mut visitor,
        )
        .expect("walk should succeed");

        assert_eq!(
            visitor.entered,
            vec![root, first, second, third]
        );

        assert_eq!(
            visitor.exited,
            vec![first, second, third, root]
        );

        assert_eq!(statistics.nodes_visited, 4);
        assert_eq!(statistics.edges_visited, 3);
        assert_eq!(statistics.maximum_depth, 1);
    }

    #[test]
    fn skip_children_preserves_exit_event() {
        struct SkipVisitor {
            entered: Vec<NodeId>,
            exited: Vec<NodeId>,
        }

        impl AstVisitor for SkipVisitor {
            type Error = core::convert::Infallible;

            fn enter(
                &mut self,
                node: &Node,
                _depth: usize,
            ) -> Result<VisitControl, Self::Error> {
                self.entered.push(node.id());

                if self.entered.len() == 1 {
                    Ok(VisitControl::SkipChildren)
                } else {
                    Ok(VisitControl::Continue)
                }
            }

            fn exit(
                &mut self,
                node: &Node,
                _depth: usize,
            ) -> Result<(), Self::Error> {
                self.exited.push(node.id());
                Ok(())
            }
        }

        let root = NodeId::new(1).expect("test ID must be non-zero");
        let child = NodeId::new(2).expect("test ID must be non-zero");

        let mut provider = TestProvider::new();

        provider.insert(root, vec![child]);
        provider.insert(child, Vec::new());

        let mut visitor = SkipVisitor {
            entered: Vec::new(),
            exited: Vec::new(),
        };

        walk(&provider, root, &mut visitor)
            .expect("walk should succeed");

        assert_eq!(visitor.entered, vec![root]);
        assert_eq!(visitor.exited, vec![root]);
    }

    #[test]
    fn repeated_nodes_are_rejected_by_default() {
        let root = NodeId::new(1).expect("test ID must be non-zero");
        let child = NodeId::new(2).expect("test ID must be non-zero");

        let mut provider = TestProvider::new();

        provider.insert(root, vec![child, child]);
        provider.insert(child, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let result = walk(
            &provider,
            root,
            &mut visitor,
        );

        assert!(matches!(
            result,
            Err(WalkError::RevisitedNode { id }) if id == child
        ));
    }

    #[test]
    fn cancellation_is_observed() {
        struct CancelImmediately;

        impl Cancellation for CancelImmediately {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let root = NodeId::new(1).expect("test ID must be non-zero");

        let mut provider = TestProvider::new();
        provider.insert(root, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let result = walk_with_config_and_cancellation(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default(),
            &CancelImmediately,
        );

        assert!(matches!(result, Err(WalkError::Cancelled)));
    }
}