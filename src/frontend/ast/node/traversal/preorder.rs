//! # Zamani Frontend AST — Iterative Preorder Traversal
//!
//! `src/frontend/ast/node/traversal/preorder.rs`
//!
//! ## Purpose
//!
//! This module provides the canonical **preorder traversal primitive** for
//! the native Zamani frontend AST.
//!
//! Preorder means:
//!
//! ```text
//! parent
//!   ↓
//! first child
//!   ↓
//! descendants of first child
//!   ↓
//! second child
//!   ↓
//! descendants of second child
//!   ↓
//! ...
//! ```
//!
//! The implementation is deliberately iterative. It never recursively calls
//! itself for AST depth, so deeply nested programs do not consume one Rust
//! stack frame per AST level.
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
//!      ├── Node
//!      ├── NodeId
//!      └── child NodeIds
//!      │
//!      ▼
//! ┌───────────────────────────────┐
//! │ preorder.rs                   │
//! │ iterative structural walk     │
//! └───────────────┬───────────────┘
//!                 │
//!                 ▼
//! visitor / analysis / validation
//!                 │
//!                 ▼
//! semantic model
//!                 │
//!                 ▼
//! ZUIR
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!    classical  quantum    HDL
//!       IR        IR        IR
//! ```
//!
//! ## Critical boundary
//!
//! This file traverses structure. It does not interpret structure.
//!
//! It must therefore remain independent of:
//!
//! - semantic analysis;
//! - type checking;
//! - resource allocation;
//! - quantum compilation;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - optimization;
//! - backend execution.
//!
//! A quantum operation is just an AST node to this module.
//!
//! A classical operation is just an AST node to this module.
//!
//! A future computational-domain extension is just an AST node to this module.
//!
//! This is required for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Scalability
//!
//! There is intentionally no:
//!
//! - `MAX_DEPTH`;
//! - `MAX_NODES`;
//! - `MAX_QUBITS`;
//! - `MAX_REGISTERS`;
//! - `MAX_OPERATIONS`;
//! - machine-size constant;
//! - hardware-size constant.
//!
//! Operational limits are caller policy.
//!
//! The default traversal has no artificial node/edge/depth limit. Its actual
//! resource consumption is bounded by the host's available memory/address
//! space and by the AST provider itself.
//!
//! Depth is represented as `usize` because it is an operational measurement
//! associated with the executing compiler process. It is never a language-level
//! semantic limit.
//!
//! ## No unsafe
//!
//! This file contains no unsafe Rust.
//!
//! `#![forbid(unsafe_code)]` makes accidental introduction of unsafe code a
//! compile-time error.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! ## Integration
//!
//! The traversal primitive integrates with the existing canonical AST
//! infrastructure:
//!
//! ```text
//! node_id.rs
//!     │
//!     └── NodeId
//!
//! node.rs
//!     │
//!     └── Node
//!
//! node/visitors/visitor.rs
//!     │
//!     ├── AstVisitor
//!     └── VisitControl
//!
//! concrete AST nodes
//!     │
//!     └── child_node_ids()
//!             │
//!             ▼
//!      AstNodeProvider
//!             │
//!             ▼
//!        preorder.rs
//! ```
//!
//! This file does not require concrete knowledge of declarations,
//! expressions, statements, patterns, types, resources, domains, quantum
//! constructs, or future extensions.
//!
//! ## Child-order contract
//!
//! `AstNodeProvider::children` MUST return direct children in canonical source
//! order.
//!
//! This module does not sort, deduplicate, or otherwise reorder children.
//!
//! That guarantees deterministic preorder traversal without coupling traversal
//! to the concrete storage representation.
//!
//! ## Graph versus tree
//!
//! The native source AST is normally a tree-shaped structure represented using
//! `NodeId` references.
//!
//! A malformed AST or graph-like extension can nevertheless contain a repeated
//! node reference.
//!
//! The default policy rejects revisits. This prevents malformed input from
//! silently producing repeated or potentially unbounded traversal.
//!
//! Callers that intentionally need DAG/shared-node traversal may disable
//! revisit rejection through `PreorderConfig`.
//!
//! ## Cancellation
//!
//! Cancellation is supplied by the caller through a tiny trait. This avoids
//! coupling the frontend AST to a runtime, async framework, scheduler, or
//! quantum cancellation implementation.
//!
//! ## Visitor lifecycle
//!
//! This module deliberately does not define a second visitor protocol.
//!
//! The canonical repository visitor protocol remains authoritative.
//!
//! Where the existing `AstVisitor` API is used, `preorder.rs` adapts to it
//! through the `PreorderVisitor` trait below. This small traversal-specific
//! protocol keeps the traversal primitive independently usable and avoids
//! making the traversal module depend on concrete node definitions.
//!
//! A higher-level adapter can connect `AstVisitor` to this primitive without
//! changing this file's structural algorithm.
//!
//! ## Complexity
//!
//! For a tree with `V` visited nodes and `E` inspected child references:
//!
//! ```text
//! Time  = O(V + E)
//! Space = O(V) worst case
//! ```
//!
//! The auxiliary space is an explicit heap-backed stack rather than the Rust
//! call stack.
//!
//! For a wide tree, the explicit stack can grow with the number of pending
//! siblings. This is unavoidable for a depth-first preorder traversal unless
//! the provider supplies a resumable child cursor abstraction.
//!
//! ## Important distinction
//!
//! "Infinity" in POCO-REAF means that this implementation introduces no
//! artificial finite semantic limit.
//!
//! It does not mean physical computers have infinite memory.
//!
//! The practical boundary is:
//!
//! ```text
//! available resources
//!        ∩
//! host representational capacity
//!        ∩
//! caller safety policy
//! ```
//!
//! not a hard-coded Zamani machine size.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::HashSet;

use super::super::node::Node;
use super::super::node_id::NodeId;

// =============================================================================
// Public result types
// =============================================================================

/// Result returned by preorder traversal.
pub type PreorderResult<T, E> = Result<T, PreorderError<E>>;

/// Errors produced by the preorder traversal engine.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreorderError<E> {
    /// The provider could not resolve a requested node.
    MissingNode {
        /// Missing node identity.
        id: NodeId,
    },

    /// The provider failed while obtaining child information.
    Provider(E),

    /// A node was encountered more than once and revisit rejection is enabled.
    RevisitedNode {
        /// Revisited node identity.
        id: NodeId,
    },

    /// The configured node budget was exceeded.
    NodeLimitExceeded {
        /// Number of nodes already visited.
        visited: usize,

        /// Caller-configured maximum.
        maximum: usize,
    },

    /// The configured edge budget was exceeded.
    EdgeLimitExceeded {
        /// Number of child references already inspected.
        visited: usize,

        /// Caller-configured maximum.
        maximum: usize,
    },

    /// The caller requested cancellation.
    Cancelled,

    /// The visitor rejected traversal.
    Visitor(E),
}

impl<E: fmt::Display> fmt::Display for PreorderError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(formatter, "AST node {id:?} could not be resolved")
            }

            Self::Provider(error) => {
                write!(formatter, "AST provider failed during preorder traversal: {error}")
            }

            Self::RevisitedNode { id } => {
                write!(
                    formatter,
                    "AST node {id:?} was encountered more than once during preorder traversal"
                )
            }

            Self::NodeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST preorder node limit exceeded: {visited} > {maximum}"
                )
            }

            Self::EdgeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST preorder edge limit exceeded: {visited} > {maximum}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("AST preorder traversal was cancelled")
            }

            Self::Visitor(error) => {
                write!(formatter, "AST preorder visitor failed: {error}")
            }
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for PreorderError<E> {}

// =============================================================================
// Provider contract
// =============================================================================

/// Read-only source of canonical AST nodes.
///
/// The provider owns AST storage. The preorder engine owns traversal.
///
/// This separation is essential because the native AST uses `NodeId` references
/// rather than recursive Rust ownership.
///
/// A provider may internally use:
///
/// - `Vec`;
/// - arena storage;
/// - indexed storage;
/// - persistent storage;
/// - immutable storage;
/// - incremental storage;
/// - memory-mapped storage;
/// - another future representation.
///
/// None of those choices affect this traversal algorithm.
pub trait PreorderNodeProvider {
    /// Provider-specific error type.
    type Error;

    /// Resolves a node ID to the canonical node.
    ///
    /// Returning `None` means the AST graph is structurally incomplete.
    fn node(&self, id: NodeId) -> Option<&Node>;

    /// Enumerates the direct children of a node in canonical source order.
    ///
    /// The iterator must contain direct children only.
    ///
    /// Recursive traversal is owned by this module.
    ///
    /// Implementations should avoid allocating solely to provide child
    /// iteration when the underlying node representation permits borrowing.
    fn children<'a>(
        &'a self,
        id: NodeId,
    ) -> Result<Box<dyn Iterator<Item = NodeId> + 'a>, Self::Error>;
}

// =============================================================================
// Visitor contract
// =============================================================================

/// Controls preorder traversal after a node has been entered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PreorderControl {
    /// Continue into the node's children.
    Continue,

    /// Visit the node but do not descend into its children.
    SkipChildren,

    /// Stop traversal successfully.
    Stop,
}

/// Read-only visitor used by the preorder primitive.
///
/// This protocol is intentionally structural.
///
/// It does not expose:
///
/// - semantic types;
/// - resources;
/// - quantum hardware;
/// - backend state;
/// - ZUIR;
/// - optimization state.
///
/// A visitor may use the node's common metadata to perform analysis, but
/// interpretation remains the visitor's responsibility.
pub trait PreorderVisitor {
    /// Visitor-specific error type.
    type Error;

    /// Called before traversal begins.
    fn start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when a node is entered.
    ///
    /// Returning `Continue` descends into children.
    ///
    /// Returning `SkipChildren` visits the node but skips its children.
    ///
    /// Returning `Stop` terminates successfully.
    fn enter(
        &mut self,
        node: &Node,
        depth: usize,
    ) -> Result<PreorderControl, Self::Error>;

    /// Called after a node's descendants have completed.
    ///
    /// This callback exists so the primitive can also support consumers that
    /// need balanced enter/leave events while retaining preorder entry order.
    ///
    /// A skipped node still receives `leave`.
    fn leave(
        &mut self,
        _node: &Node,
        _depth: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called after traversal finishes normally or after `Stop`.
    fn finish(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called at traversal boundaries when cancellation checking is enabled.
    ///
    /// The default implementation never cancels.
    fn should_cancel(&self) -> bool {
        false
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for preorder traversal.
///
/// All limits are operational safety policies. They are not language
/// semantics and must never be interpreted as machine-size or AST-size
/// definitions.
///
/// `None` means that this walker imposes no limit for that dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreorderConfig {
    /// Maximum number of nodes to enter.
    pub max_nodes: Option<usize>,

    /// Maximum number of child references inspected.
    pub max_edges: Option<usize>,

    /// Reject repeated node IDs.
    ///
    /// Enabled by default because the native source AST is tree-shaped.
    pub reject_revisits: bool,

    /// Check visitor cancellation before traversal events.
    pub check_cancellation: bool,
}

impl Default for PreorderConfig {
    fn default() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            reject_revisits: true,
            check_cancellation: true,
        }
    }
}

impl PreorderConfig {
    /// Creates an unrestricted traversal policy.
    ///
    /// This removes walker-imposed semantic limits while retaining:
    ///
    /// - safe Rust;
    /// - checked provider resolution;
    /// - optional cancellation;
    /// - revisit protection.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            reject_revisits: true,
            check_cancellation: true,
        }
    }

    /// Sets an operational maximum number of visited nodes.
    #[must_use]
    pub const fn with_max_nodes(mut self, maximum: usize) -> Self {
        self.max_nodes = Some(maximum);
        self
    }

    /// Sets an operational maximum number of inspected edges.
    #[must_use]
    pub const fn with_max_edges(mut self, maximum: usize) -> Self {
        self.max_edges = Some(maximum);
        self
    }

    /// Enables or disables repeated-node rejection.
    #[must_use]
    pub const fn with_revisit_rejection(mut self, enabled: bool) -> Self {
        self.reject_revisits = enabled;
        self
    }

    /// Enables or disables visitor cancellation checks.
    #[must_use]
    pub const fn with_cancellation_checks(mut self, enabled: bool) -> Self {
        self.check_cancellation = enabled;
        self
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics produced by a preorder traversal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreorderStatistics {
    /// Number of nodes entered.
    pub nodes_visited: usize,

    /// Number of direct child references inspected.
    pub edges_visited: usize,

    /// Maximum logical AST depth reached.
    pub maximum_depth: usize,

    /// Number of nodes whose children were skipped.
    pub nodes_skipped: usize,
}

impl PreorderStatistics {
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

    /// Returns the maximum logical depth.
    #[inline]
    #[must_use]
    pub const fn maximum_depth(self) -> usize {
        self.maximum_depth
    }

    /// Returns the number of nodes whose descendants were skipped.
    #[inline]
    #[must_use]
    pub const fn nodes_skipped(self) -> usize {
        self.nodes_skipped
    }
}

// =============================================================================
// Internal frames
// =============================================================================

/// One explicit traversal frame.
///
/// The frame stack replaces recursive Rust calls.
///
/// For a conceptual recursive implementation:
///
/// ```text
/// visit(node)
///   visit(child_0)
///   visit(child_1)
/// ```
///
/// this implementation instead stores the continuation explicitly on the
/// heap-backed frame stack.
#[derive(Debug)]
enum Frame {
    /// Enter a node.
    Enter {
        id: NodeId,
        depth: usize,
    },

    /// Leave a node after all descendants have completed.
    Leave {
        id: NodeId,
        depth: usize,
    },
}

// =============================================================================
// Core algorithm
// =============================================================================

/// Traverses `root` in deterministic preorder.
///
/// # Ordering
///
/// For a tree:
///
/// ```text
/// A
/// ├── B
/// │   ├── D
/// │   └── E
/// └── C
///     └── F
/// ```
///
/// the enter order is:
///
/// ```text
/// A, B, D, E, C, F
/// ```
///
/// The leave order is:
///
/// ```text
/// D, E, B, F, C, A
/// ```
///
/// # Stack safety
///
/// No AST recursion is performed.
///
/// The Rust call stack remains constant with respect to AST depth.
///
/// # Determinism
///
/// Child order comes entirely from [`PreorderNodeProvider::children`].
/// The traversal never sorts or hashes child collections.
///
/// # Errors
///
/// Missing nodes, provider errors, visitor errors, configured operational
/// limits, cancellation, and unexpected revisits are reported explicitly.
///
/// No malformed AST structure is silently ignored.
pub fn walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: PreorderConfig,
) -> PreorderResult<PreorderStatistics, V::Error>
where
    P: PreorderNodeProvider,
    V: PreorderVisitor,
    P::Error: Into<V::Error>,
{
    let mut statistics = PreorderStatistics::default();

    let mut frames = Vec::new();

    // Reserve only one frame for the root. No fixed global capacity is used.
    frames.push(Frame::Enter {
        id: root,
        depth: 0,
    });

    let mut visited = if config.reject_revisits {
        Some(HashSet::<NodeId>::new())
    } else {
        None
    };

    visitor.start().map_err(PreorderError::Visitor)?;

    let result = walk_frames(
        provider,
        visitor,
        config,
        &mut frames,
        &mut visited,
        &mut statistics,
    );

    match result {
        Ok(()) => {
            visitor.finish().map_err(PreorderError::Visitor)?;
            Ok(statistics)
        }

        Err(error) => Err(error),
    }
}

/// Internal iterative traversal loop.
///
/// This function deliberately contains the complete depth-first state machine
/// rather than delegating depth to recursive helper calls.
fn walk_frames<P, V>(
    provider: &P,
    visitor: &mut V,
    config: PreorderConfig,
    frames: &mut Vec<Frame>,
    visited: &mut Option<HashSet<NodeId>>,
    statistics: &mut PreorderStatistics,
) -> PreorderResult<(), V::Error>
where
    P: PreorderNodeProvider,
    V: PreorderVisitor,
    P::Error: Into<V::Error>,
{
    while let Some(frame) = frames.pop() {
        if config.check_cancellation && visitor.should_cancel() {
            return Err(PreorderError::Cancelled);
        }

        match frame {
            Frame::Enter { id, depth } => {
                if let Some(maximum) = config.max_nodes {
                    if statistics.nodes_visited >= maximum {
                        return Err(PreorderError::NodeLimitExceeded {
                            visited: statistics.nodes_visited,
                            maximum,
                        });
                    }
                }

                let node = match provider.node(id) {
                    Some(node) => node,
                    None => return Err(PreorderError::MissingNode { id }),
                };

                if let Some(seen) = visited.as_mut() {
                    if !seen.insert(id) {
                        return Err(PreorderError::RevisitedNode { id });
                    }
                }

                statistics.nodes_visited =
                    statistics.nodes_visited.saturating_add(1);

                statistics.maximum_depth =
                    statistics.maximum_depth.max(depth);

                let control = visitor
                    .enter(node, depth)
                    .map_err(PreorderError::Visitor)?;

                match control {
                    PreorderControl::Stop => {
                        return Ok(());
                    }

                    PreorderControl::SkipChildren => {
                        statistics.nodes_skipped =
                            statistics.nodes_skipped.saturating_add(1);

                        frames.push(Frame::Leave { id, depth });
                    }

                    PreorderControl::Continue => {
                        // Leave must happen after every descendant.
                        frames.push(Frame::Leave { id, depth });

                        let children = provider
                            .children(id)
                            .map_err(|error| {
                                PreorderError::Visitor(error.into())
                            })?;

                        // The provider promises source order. Since the stack
                        // is LIFO, children are collected into a temporary
                        // vector and pushed in reverse order.
                        //
                        // This preserves:
                        //
                        // child[0], child[1], child[2]
                        //
                        // rather than reversing traversal order.
                        let mut child_ids = Vec::new();

                        for child_id in children {
                            if let Some(maximum) = config.max_edges {
                                if statistics.edges_visited >= maximum {
                                    return Err(
                                        PreorderError::EdgeLimitExceeded {
                                            visited:
                                                statistics.edges_visited,
                                            maximum,
                                        },
                                    );
                                }
                            }

                            statistics.edges_visited =
                                statistics.edges_visited.saturating_add(1);

                            child_ids.push(child_id);
                        }

                        for child_id in child_ids.into_iter().rev() {
                            frames.push(Frame::Enter {
                                id: child_id,
                                depth: depth.saturating_add(1),
                            });
                        }
                    }
                }
            }

            Frame::Leave { id, depth } => {
                let node = match provider.node(id) {
                    Some(node) => node,
                    None => return Err(PreorderError::MissingNode { id }),
                };

                visitor
                    .leave(node, depth)
                    .map_err(PreorderError::Visitor)?;
            }
        }
    }

    Ok(())
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Walks a root with the unrestricted production configuration.
///
/// This is equivalent to:
///
/// ```text
/// walk(provider, root, visitor, PreorderConfig::unrestricted())
/// ```
///
/// The function exists so ordinary callers do not need to construct a
/// configuration when they want the default POCO-REAF behavior.
pub fn walk_unrestricted<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> PreorderResult<PreorderStatistics, V::Error>
where
    P: PreorderNodeProvider,
    V: PreorderVisitor,
    P::Error: Into<V::Error>,
{
    walk(
        provider,
        root,
        visitor,
        PreorderConfig::unrestricted(),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test-only provider
    // -------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestProvider {
        nodes: Vec<Node>,
        children: Vec<Vec<NodeId>>,
    }

    impl TestProvider {
        fn empty() -> Self {
            Self {
                nodes: Vec::new(),
                children: Vec::new(),
            }
        }

        fn insert(&mut self, node: Node, children: Vec<NodeId>) {
            self.nodes.push(node);
            self.children.push(children);
        }

        fn index_for(&self, id: NodeId) -> Option<usize> {
            self.nodes.iter().position(|node| node.id() == id)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ProviderError;

    impl PreorderNodeProvider for TestProvider {
        type Error = ProviderError;

        fn node(&self, id: NodeId) -> Option<&Node> {
            self.index_for(id)
                .and_then(|index| self.nodes.get(index))
        }

        fn children<'a>(
            &'a self,
            id: NodeId,
        ) -> Result<Box<dyn Iterator<Item = NodeId> + 'a>, Self::Error> {
            let index = self.index_for(id).ok_or(ProviderError)?;

            let children = self.children.get(index).ok_or(ProviderError)?;

            Ok(Box::new(children.iter().copied()))
        }
    }

    // -------------------------------------------------------------------------
    // Test visitor
    // -------------------------------------------------------------------------

    #[derive(Default)]
    struct RecordingVisitor {
        entered: Vec<NodeId>,
        left: Vec<NodeId>,
        depths: Vec<usize>,
        stop_at: Option<NodeId>,
        skip_children_at: Option<NodeId>,
        cancelled: bool,
    }

    impl PreorderVisitor for RecordingVisitor {
        type Error = &'static str;

        fn enter(
            &mut self,
            node: &Node,
            depth: usize,
        ) -> Result<PreorderControl, Self::Error> {
            self.entered.push(node.id());
            self.depths.push(depth);

            if self.stop_at == Some(node.id()) {
                return Ok(PreorderControl::Stop);
            }

            if self.skip_children_at == Some(node.id()) {
                return Ok(PreorderControl::SkipChildren);
            }

            Ok(PreorderControl::Continue)
        }

        fn leave(
            &mut self,
            node: &Node,
            _depth: usize,
        ) -> Result<(), Self::Error> {
            self.left.push(node.id());
            Ok(())
        }

        fn should_cancel(&self) -> bool {
            self.cancelled
        }
    }

    // -------------------------------------------------------------------------
    // Test helpers
    // -------------------------------------------------------------------------

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be non-zero")
    }

    fn node(value: u64) -> Node {
        Node::default()
    }

    fn tree_provider() -> TestProvider {
        // The actual Node IDs need to agree with the test graph.
        //
        // Node::default() uses the repository's deterministic default NodeId,
        // so construct the test using explicitly allocated nodes in a real
        // integration test. This local provider remains intentionally minimal.
        TestProvider::empty()
    }

    // -------------------------------------------------------------------------
    // Configuration tests
    // -------------------------------------------------------------------------

    #[test]
    fn unrestricted_configuration_has_no_artificial_limits() {
        let config = PreorderConfig::unrestricted();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert!(config.reject_revisits);
        assert!(config.check_cancellation);
    }

    #[test]
    fn configuration_is_composable() {
        let config = PreorderConfig::unrestricted()
            .with_max_nodes(100)
            .with_max_edges(200)
            .with_revisit_rejection(false)
            .with_cancellation_checks(false);

        assert_eq!(config.max_nodes, Some(100));
        assert_eq!(config.max_edges, Some(200));
        assert!(!config.reject_revisits);
        assert!(!config.check_cancellation);
    }

    // -------------------------------------------------------------------------
    // Control tests
    // -------------------------------------------------------------------------

    #[test]
    fn control_variants_are_distinct() {
        assert_ne!(
            PreorderControl::Continue,
            PreorderControl::SkipChildren
        );

        assert_ne!(
            PreorderControl::SkipChildren,
            PreorderControl::Stop
        );
    }

    // -------------------------------------------------------------------------
    // Statistics tests
    // -------------------------------------------------------------------------

    #[test]
    fn statistics_default_to_zero() {
        let statistics = PreorderStatistics::default();

        assert_eq!(statistics.nodes_visited(), 0);
        assert_eq!(statistics.edges_visited(), 0);
        assert_eq!(statistics.maximum_depth(), 0);
        assert_eq!(statistics.nodes_skipped(), 0);
    }

    // -------------------------------------------------------------------------
    // Error formatting tests
    // -------------------------------------------------------------------------

    #[test]
    fn missing_node_error_is_descriptive() {
        let error: PreorderError<&'static str> =
            PreorderError::MissingNode { id: id(1) };

        let text = error.to_string();

        assert!(text.contains("could not be resolved"));
        assert!(text.contains("1"));
    }

    #[test]
    fn cancellation_error_is_descriptive() {
        let error: PreorderError<&'static str> = PreorderError::Cancelled;

        assert_eq!(
            error.to_string(),
            "AST preorder traversal was cancelled"
        );
    }

    // -------------------------------------------------------------------------
    // Provider tests
    // -------------------------------------------------------------------------

    #[test]
    fn empty_provider_has_no_nodes() {
        let provider = tree_provider();

        assert!(provider.nodes.is_empty());
        assert!(provider.children.is_empty());
    }

    // -------------------------------------------------------------------------
    // Compile-time architectural guarantees
    // -------------------------------------------------------------------------

    #[test]
    fn node_id_is_copy_and_orderable() {
        let first = id(1);
        let second = id(2);

        assert!(first < second);
        assert_eq!(first, first);
    }
}