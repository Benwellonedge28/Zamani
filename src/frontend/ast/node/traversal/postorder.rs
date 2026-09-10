//! # Zamani Frontend AST — Iterative Postorder Traversal
//!
//! `src/frontend/ast/node/traversal/postorder.rs`
//!
//! ## Purpose
//!
//! This module provides the canonical iterative **postorder traversal**
//! primitive for the native Zamani frontend AST.
//!
//! Postorder means:
//!
//! ```text
//! descendants
//!     ↓
//! child
//!     ↓
//! sibling
//!     ↓
//! parent
//! ```
//!
//! For a tree:
//
//! ```text
//!        A
//!      /   \
//!     B     C
//!    / \
//!   D   E
//! ```
//!
//! the postorder sequence is:
//
//! ```text
//! D, E, B, C, A
//! ```
//!
//! ## Architectural boundary
//!
//! This module traverses AST structure only.
//!
//! It does not interpret AST meaning and must not depend on:
//!
//! - semantic analysis;
//! - type checking;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend execution;
//! - vendor APIs.
//!
//! A quantum operation, classical operation, HDL construct, accelerator
//! construct, or future extension is simply an AST node to this walker.
//!
//! This preserves:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Integration
//!
//! The traversal stack is independent from AST storage:
//
//! ```text
//! concrete AST nodes
//!       │
//!       ▼
//! NodeId relationships
//!       │
//!       ▼
//! AstNodeProvider
//!       │
//!       ▼
//! this module
//!       │
//!       ▼
//! AstVisitor
//!       │
//!       ├── structural validation
//!       ├── diagnostics
//!       ├── tooling
//!       ├── source analysis
//!       └── semantic preparation
//! ```
//!
//! The canonical provider and visitor contracts are reused from
//! `node::visitors`, rather than creating duplicate traversal abstractions.
//!
//! ## Strict postorder semantics
//!
//! `AstVisitor::enter` is still called when a node is first encountered.
//! It is used only to determine whether traversal should:
//!
//! - continue into children;
//! - skip children;
//! - stop early.
//!
//! The actual **postorder event** is `AstVisitor::exit`.
//!
//! Therefore, for a normal tree:
//!
//! ```text
//! enter(D)
//! exit(D)
//! enter(E)
//! exit(E)
//! enter(B)
//! exit(B)
//! enter(C)
//! exit(C)
//! enter(A)
//! exit(A)
//! ```
//!
//! Consumers interested specifically in postorder semantics should perform
//! their completed-node operation in `exit`.
//!
//! ## Scalability
//!
//! This implementation contains no:
//!
//! - `MAX_DEPTH`;
//! - `MAX_NODES`;
//! - `MAX_QUBITS`;
//! - `MAX_REGISTERS`;
//! - `MAX_OPERATIONS`;
//! - fixed machine size;
//! - fixed hardware topology.
//!
//! Optional `WalkConfig` limits are operational safety policies supplied by
//! the caller. They are not language semantics.
//!
//! The traversal itself is iterative and therefore does not consume one Rust
//! call-stack frame per AST level.
//!
//! More importantly, child iterators are retained on traversal frames instead
//! of materializing every sibling list into temporary vectors.
//!
//! For a tree with:
//!
//! - `V` visited nodes;
//! - `E` inspected child references;
//! - `D` maximum depth;
//!
//! traversal work is:
//!
//! ```text
//! Time  = O(V + E)
//! Stack = O(D)
//! ```
//!
//! when revisit detection is disabled.
//!
//! With the native-tree safety policy enabled, the visited-node set additionally
//! requires O(V) heap storage.
//!
//! ## Tree versus graph
//!
//! Native source AST structure is expected to be tree-shaped.
//!
//! By default, repeated `NodeId` references are rejected. This prevents a
//! malformed graph from silently causing repeated or potentially unbounded
//! traversal.
//!
//! Callers that intentionally require DAG/shared-node semantics may disable
//! revisit rejection through `WalkConfig`.
//!
//! ## Determinism
//!
//! Determinism is obtained from the provider contract:
//!
//! - the provider returns direct children in canonical source order;
//! - this module never sorts children;
//! - this module never depends on hash-map iteration order;
//! - no random state is used;
//! - no timestamps are used;
//! - no hardware information is consulted.
//!
//! ## Safety
//!
//! This module uses only safe Rust.
//!
//! It explicitly forbids unsafe code.
//!
//! Malformed provider behavior is detected rather than silently accepted.
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
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::HashSet;

use crate::frontend::ast::node::node::Node;
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::visitors::visitor::{AstVisitor, VisitControl};
use crate::frontend::ast::node::visitors::walk::{
    AstNodeProvider,
    Cancellation,
    NeverCancelled,
    WalkConfig,
};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by postorder traversal.
///
/// Provider and visitor errors remain distinct so callers can recover the
/// original error source.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PostorderError<ProviderError, VisitorError> {
    /// The provider could not resolve the requested node.
    MissingNode {
        /// Requested node identity.
        id: NodeId,
    },

    /// The provider returned a node whose identity does not match the request.
    ///
    /// This indicates a provider contract violation.
    InvalidProviderNode {
        /// Requested identity.
        requested: NodeId,

        /// Identity returned by the provider.
        actual: NodeId,
    },

    /// A node was encountered more than once while revisit rejection was
    /// enabled.
    RevisitedNode {
        /// Repeated identity.
        id: NodeId,
    },

    /// The provider returned an error while enumerating children.
    Provider(ProviderError),

    /// The visitor returned an error.
    Visitor(VisitorError),

    /// A caller-configured node budget was exceeded.
    NodeLimitExceeded {
        /// Number of nodes already entered.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A caller-configured child-reference budget was exceeded.
    EdgeLimitExceeded {
        /// Number of child references already inspected.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The logical depth cannot be represented by the host `usize`.
    ///
    /// This is a representational boundary, not a Zamani language limit.
    DepthOverflow {
        /// Parent depth whose child could not be represented.
        parent_depth: usize,
    },

    /// Traversal was cancelled by the caller or visitor.
    Cancelled,
}

impl<ProviderError: fmt::Display, VisitorError: fmt::Display> fmt::Display
    for PostorderError<ProviderError, VisitorError>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode { id } => {
                write!(formatter, "AST node {id:?} could not be resolved")
            }

            Self::InvalidProviderNode { requested, actual } => {
                write!(
                    formatter,
                    "AST provider returned node {actual:?} while resolving {requested:?}"
                )
            }

            Self::RevisitedNode { id } => {
                write!(
                    formatter,
                    "AST node {id:?} was encountered more than once during postorder traversal"
                )
            }

            Self::Provider(error) => {
                write!(formatter, "AST provider failed: {error}")
            }

            Self::Visitor(error) => {
                write!(formatter, "AST visitor failed: {error}")
            }

            Self::NodeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST postorder node limit exceeded: {visited} >= {maximum}"
                )
            }

            Self::EdgeLimitExceeded { visited, maximum } => {
                write!(
                    formatter,
                    "AST postorder edge limit exceeded: {visited} >= {maximum}"
                )
            }

            Self::DepthOverflow { parent_depth } => {
                write!(
                    formatter,
                    "AST traversal depth overflow after depth {parent_depth}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("AST postorder traversal was cancelled")
            }
        }
    }
}

impl<ProviderError, VisitorError> std::error::Error
    for PostorderError<ProviderError, VisitorError>
where
    ProviderError: std::error::Error + 'static,
    VisitorError: std::error::Error + 'static,
{
}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics collected during postorder traversal.
///
/// These values are measurements of the current traversal, never language
/// limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PostorderStatistics {
    /// Number of nodes successfully entered.
    pub nodes_visited: usize,

    /// Number of child references inspected.
    pub edges_visited: usize,

    /// Maximum logical depth reached.
    pub maximum_depth: usize,

    /// Number of nodes whose children were deliberately skipped.
    pub nodes_skipped: usize,
}

impl PostorderStatistics {
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

    /// Returns the maximum logical depth observed.
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
// Outcome
// =============================================================================

/// Result information for a completed or deliberately stopped traversal.
///
/// `completed == false` means the visitor returned `VisitControl::Stop`.
///
/// That is a successful early termination, not an error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PostorderOutcome {
    statistics: PostorderStatistics,
    completed: bool,
}

impl PostorderOutcome {
    /// Returns traversal statistics.
    #[inline]
    #[must_use]
    pub const fn statistics(self) -> PostorderStatistics {
        self.statistics
    }

    /// Returns `true` if the complete reachable traversal finished.
    #[inline]
    #[must_use]
    pub const fn completed(self) -> bool {
        self.completed
    }

    /// Returns `true` if traversal stopped before completion.
    #[inline]
    #[must_use]
    pub const fn stopped_early(self) -> bool {
        !self.completed
    }
}

// =============================================================================
// Result alias
// =============================================================================

/// Canonical postorder result type.
pub type PostorderResult<P, V> = Result<
    PostorderOutcome,
    PostorderError<
        <P as AstNodeProvider>::Error,
        <V as AstVisitor>::Error,
    >,
>;

// =============================================================================
// Public API
// =============================================================================

/// Performs an iterative postorder traversal using default policy.
///
/// The default policy:
///
/// - imposes no artificial node limit;
/// - imposes no artificial edge limit;
/// - rejects repeated nodes;
/// - checks cancellation.
pub fn walk<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
) -> PostorderResult<P, V>
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

/// Performs iterative postorder traversal using explicit policy.
pub fn walk_with_config<P, V>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
) -> PostorderResult<P, V>
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

/// Performs iterative postorder traversal using explicit policy and an
/// external cancellation source.
///
/// The cancellation abstraction is supplied by the existing AST traversal
/// infrastructure and is deliberately independent of quantum/runtime
/// cancellation implementations.
pub fn walk_with_config_and_cancellation<P, V, C>(
    provider: &P,
    root: NodeId,
    visitor: &mut V,
    config: WalkConfig,
    cancellation: &C,
) -> PostorderResult<P, V>
where
    P: AstNodeProvider,
    V: AstVisitor,
    C: Cancellation,
{
    // -------------------------------------------------------------------------
    // Explicit traversal frames
    // -------------------------------------------------------------------------
    //
    // A recursive implementation would conceptually do:
    //
    //     visit(child_1)
    //     visit(child_2)
    //     ...
    //     visit(parent)
    //
    // That would consume one Rust stack frame per AST depth.
    //
    // Instead, the parent's child iterator itself becomes a heap-backed frame.
    //
    // This is particularly important for generated, adversarial, or extremely
    // deeply nested programs.
    enum Frame<'a> {
        /// Node has not yet received its enter event.
        Enter {
            id: NodeId,
            depth: usize,
        },

        /// Node is currently enumerating children.
        ///
        /// The iterator remains alive while the next child subtree is visited.
        Children {
            id: NodeId,
            depth: usize,
            iter: Box<dyn Iterator<Item = NodeId> + 'a>,
        },

        /// All permitted descendants have completed.
        ///
        /// This is the actual postorder event point.
        Exit {
            id: NodeId,
            depth: usize,
        },
    }

    // -------------------------------------------------------------------------
    // Root validation
    // -------------------------------------------------------------------------

    let root_node = provider
        .node(root)
        .ok_or(PostorderError::MissingNode { id: root })?;

    validate_provider_node(root, root_node)?;

    // -------------------------------------------------------------------------
    // Traversal state
    // -------------------------------------------------------------------------

    let mut frames = Vec::new();

    frames.push(Frame::Enter {
        id: root,
        depth: 0,
    });

    // Revisit detection is optional because DAG/shared-node users may
    // intentionally disable it.
    //
    // No AST semantic limit is associated with this set.
    let mut visited = HashSet::<NodeId>::new();

    let mut statistics = PostorderStatistics::default();

    // -------------------------------------------------------------------------
    // Iterative traversal
    // -------------------------------------------------------------------------

    while let Some(frame) = frames.pop() {
        if config.check_cancellation && cancellation.is_cancelled() {
            return Err(PostorderError::Cancelled);
        }

        if config.check_cancellation && visitor.should_cancel() {
            return Err(PostorderError::Cancelled);
        }

        match frame {
            // -----------------------------------------------------------------
            // Enter node
            // -----------------------------------------------------------------

            Frame::Enter { id, depth } => {
                if let Some(maximum) = config.max_nodes {
                    if statistics.nodes_visited >= maximum {
                        return Err(PostorderError::NodeLimitExceeded {
                            visited: statistics.nodes_visited,
                            maximum,
                        });
                    }
                }

                if config.reject_revisits && !visited.insert(id) {
                    return Err(PostorderError::RevisitedNode { id });
                }

                let node = provider
                    .node(id)
                    .ok_or(PostorderError::MissingNode { id })?;

                validate_provider_node(id, node)?;

                statistics.nodes_visited = statistics
                    .nodes_visited
                    .checked_add(1)
                    .ok_or(PostorderError::NodeLimitExceeded {
                        visited: usize::MAX,
                        maximum: usize::MAX,
                    })?;

                statistics.maximum_depth =
                    statistics.maximum_depth.max(depth);

                let control = visitor
                    .enter(node, depth)
                    .map_err(PostorderError::Visitor)?;

                match control {
                    // ---------------------------------------------------------
                    // Early successful termination
                    // ---------------------------------------------------------

                    VisitControl::Stop => {
                        return Ok(PostorderOutcome {
                            statistics,
                            completed: false,
                        });
                    }

                    // ---------------------------------------------------------
                    // Visit node but do not descend
                    // ---------------------------------------------------------

                    VisitControl::SkipChildren => {
                        statistics.nodes_skipped = statistics
                            .nodes_skipped
                            .checked_add(1)
                            .ok_or(PostorderError::NodeLimitExceeded {
                                visited: statistics.nodes_visited,
                                maximum: usize::MAX,
                            })?;

                        // Even when children are skipped, the node itself must
                        // receive its postorder exit event.
                        frames.push(Frame::Exit { id, depth });
                    }

                    // ---------------------------------------------------------
                    // Descend into children
                    // ---------------------------------------------------------

                    VisitControl::Continue => {
                        let iter = provider
                            .children(id)
                            .map_err(PostorderError::Provider)?;

                        frames.push(Frame::Children {
                            id,
                            depth,
                            iter,
                        });
                    }
                }
            }

            // -----------------------------------------------------------------
            // Continue enumerating children
            // -----------------------------------------------------------------

            Frame::Children {
                id,
                depth,
                mut iter,
            } => {
                match iter.next() {
                    Some(child_id) => {
                        if let Some(maximum) = config.max_edges {
                            if statistics.edges_visited >= maximum {
                                return Err(
                                    PostorderError::EdgeLimitExceeded {
                                        visited: statistics.edges_visited,
                                        maximum,
                                    },
                                );
                            }
                        }

                        statistics.edges_visited = statistics
                            .edges_visited
                            .checked_add(1)
                            .ok_or(PostorderError::EdgeLimitExceeded {
                                visited: usize::MAX,
                                maximum: usize::MAX,
                            })?;

                        let child_depth = depth
                            .checked_add(1)
                            .ok_or(PostorderError::DepthOverflow {
                                parent_depth: depth,
                            })?;

                        // Put the parent frame back first.
                        //
                        // The child frame is then placed on top of it, causing
                        // the complete child subtree to execute before the
                        // parent continues with its next child.
                        //
                        // This gives:
                        //
                        //     child_1 subtree
                        //     child_2 subtree
                        //     ...
                        //     parent exit
                        //
                        // without collecting all siblings into a temporary Vec.
                        frames.push(Frame::Children {
                            id,
                            depth,
                            iter,
                        });

                        frames.push(Frame::Enter {
                            id: child_id,
                            depth: child_depth,
                        });
                    }

                    None => {
                        // Every child has completed.
                        //
                        // This frame is therefore exactly the point at which
                        // postorder requires the parent to become observable.
                        frames.push(Frame::Exit { id, depth });
                    }
                }
            }

            // -----------------------------------------------------------------
            // Postorder event
            // -----------------------------------------------------------------

            Frame::Exit { id, depth } => {
                let node = provider
                    .node(id)
                    .ok_or(PostorderError::MissingNode { id })?;

                validate_provider_node(id, node)?;

                visitor
                    .exit(node, depth)
                    .map_err(PostorderError::Visitor)?;
            }
        }
    }

    Ok(PostorderOutcome {
        statistics,
        completed: true,
    })
}

// =============================================================================
// Provider validation
// =============================================================================

/// Validates the provider's most important identity invariant.
///
/// A provider must never return a different node from the ID requested by the
/// traversal engine.
#[inline]
fn validate_provider_node<ProviderError, VisitorError>(
    requested: NodeId,
    node: &Node,
) -> Result<(), PostorderError<ProviderError, VisitorError>> {
    if node.id() == requested {
        Ok(())
    } else {
        Err(PostorderError::InvalidProviderNode {
            requested,
            actual: node.id(),
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::node_kind::{
        CoreNodeKind,
        NodeKind,
    };
    use crate::frontend::ast::node::source::Span;

    use std::collections::HashMap;

    // -------------------------------------------------------------------------
    // Test provider
    // -------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestProvider {
        nodes: HashMap<NodeId, Node>,
        children: HashMap<NodeId, Vec<NodeId>>,
    }

    impl TestProvider {
        fn new() -> Self {
            Self {
                nodes: HashMap::new(),
                children: HashMap::new(),
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

    // -------------------------------------------------------------------------
    // Test visitor
    // -------------------------------------------------------------------------

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

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    // -------------------------------------------------------------------------
    // Ordering
    // -------------------------------------------------------------------------

    #[test]
    fn visits_children_before_parent_in_source_order() {
        let root = id(1);
        let first = id(2);
        let second = id(3);
        let third = id(4);

        let mut provider = TestProvider::new();

        provider.insert(root, vec![first, second, third]);
        provider.insert(first, Vec::new());
        provider.insert(second, Vec::new());
        provider.insert(third, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let outcome = walk(
            &provider,
            root,
            &mut visitor,
        )
        .expect("postorder traversal should succeed");

        assert!(outcome.completed());

        assert_eq!(
            visitor.entered,
            vec![root, first, second, third]
        );

        assert_eq!(
            visitor.exited,
            vec![first, second, third, root]
        );

        assert_eq!(
            outcome.statistics().nodes_visited,
            4
        );

        assert_eq!(
            outcome.statistics().edges_visited,
            3
        );

        assert_eq!(
            outcome.statistics().maximum_depth,
            1
        );
    }

    // -------------------------------------------------------------------------
    // Deep nesting
    // -------------------------------------------------------------------------

    #[test]
    fn deep_tree_does_not_use_recursive_rust_stack() {
        const DEPTH: usize = 10_000;

        let mut provider = TestProvider::new();

        for index in 1..=DEPTH {
            let current = id(index as u64);

            if index == DEPTH {
                provider.insert(current, Vec::new());
            } else {
                let child = id((index + 1) as u64);
                provider.insert(current, vec![child]);
            }
        }

        let mut visitor = RecordingVisitor::default();

        let outcome = walk(
            &provider,
            id(1),
            &mut visitor,
        )
        .expect("deep postorder traversal should succeed");

        assert!(outcome.completed());
        assert_eq!(
            outcome.statistics().nodes_visited,
            DEPTH
        );
        assert_eq!(
            outcome.statistics().edges_visited,
            DEPTH - 1
        );
        assert_eq!(
            outcome.statistics().maximum_depth,
            DEPTH - 1
        );

        assert_eq!(
            visitor.exited.first().copied(),
            Some(id(DEPTH as u64))
        );

        assert_eq!(
            visitor.exited.last().copied(),
            Some(id(1))
        );
    }

    // -------------------------------------------------------------------------
    // Skip children
    // -------------------------------------------------------------------------

    #[test]
    fn skip_children_still_emits_postorder_exit() {
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

                if node.id() == id(1) {
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

        let root = id(1);
        let child = id(2);

        let mut provider = TestProvider::new();

        provider.insert(root, vec![child]);
        provider.insert(child, Vec::new());

        let mut visitor = SkipVisitor {
            entered: Vec::new(),
            exited: Vec::new(),
        };

        let outcome = walk(
            &provider,
            root,
            &mut visitor,
        )
        .expect("postorder traversal should succeed");

        assert!(outcome.completed());
        assert_eq!(visitor.entered, vec![root]);
        assert_eq!(visitor.exited, vec![root]);
        assert_eq!(
            outcome.statistics().nodes_skipped,
            1
        );
    }

    // -------------------------------------------------------------------------
    // Revisit detection
    // -------------------------------------------------------------------------

    #[test]
    fn repeated_nodes_are_rejected_by_default() {
        let root = id(1);
        let child = id(2);

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
            Err(PostorderError::RevisitedNode { id })
                if id == child
        ));
    }

    // -------------------------------------------------------------------------
    // Configured DAG/shared-node traversal
    // -------------------------------------------------------------------------

    #[test]
    fn revisit_rejection_can_be_disabled_explicitly() {
        let root = id(1);
        let child = id(2);

        let mut provider = TestProvider::new();

        provider.insert(root, vec![child, child]);
        provider.insert(child, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let outcome = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().reject_revisits(false),
        )
        .expect("DAG traversal should succeed when explicitly enabled");

        assert!(outcome.completed());

        assert_eq!(
            visitor.exited,
            vec![child, child, root]
        );
    }

    // -------------------------------------------------------------------------
    // Early successful stop
    // -------------------------------------------------------------------------

    #[test]
    fn stop_is_successful_early_termination_not_an_error() {
        struct StopVisitor;

        impl AstVisitor for StopVisitor {
            type Error = core::convert::Infallible;

            fn enter(
                &mut self,
                _node: &Node,
                _depth: usize,
            ) -> Result<VisitControl, Self::Error> {
                Ok(VisitControl::Stop)
            }
        }

        let root = id(1);

        let mut provider = TestProvider::new();
        provider.insert(root, Vec::new());

        let mut visitor = StopVisitor;

        let outcome = walk(
            &provider,
            root,
            &mut visitor,
        )
        .expect("Stop must be a successful traversal result");

        assert!(!outcome.completed());
        assert!(outcome.stopped_early());
        assert_eq!(
            outcome.statistics().nodes_visited,
            1
        );
    }

    // -------------------------------------------------------------------------
    // Node limit
    // -------------------------------------------------------------------------

    #[test]
    fn node_limit_is_an_operational_policy() {
        let root = id(1);
        let child = id(2);

        let mut provider = TestProvider::new();

        provider.insert(root, vec![child]);
        provider.insert(child, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let result = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().with_max_nodes(1),
        );

        assert!(matches!(
            result,
            Err(PostorderError::NodeLimitExceeded {
                visited: 1,
                maximum: 1
            })
        ));
    }

    // -------------------------------------------------------------------------
    // Edge limit
    // -------------------------------------------------------------------------

    #[test]
    fn edge_limit_is_enforced_without_becoming_language_semantics() {
        let root = id(1);
        let first = id(2);
        let second = id(3);

        let mut provider = TestProvider::new();

        provider.insert(root, vec![first, second]);
        provider.insert(first, Vec::new());
        provider.insert(second, Vec::new());

        let mut visitor = RecordingVisitor::default();

        let result = walk_with_config(
            &provider,
            root,
            &mut visitor,
            WalkConfig::default().with_max_edges(1),
        );

        assert!(matches!(
            result,
            Err(PostorderError::EdgeLimitExceeded {
                visited: 1,
                maximum: 1
            })
        ));
    }

    // -------------------------------------------------------------------------
    // Missing node
    // -------------------------------------------------------------------------

    #[test]
    fn missing_child_is_reported_without_panicking() {
        let root = id(1);
        let missing = id(999);

        let mut provider = TestProvider::new();
        provider.insert(root, vec![missing]);

        let mut visitor = RecordingVisitor::default();

        let result = walk(
            &provider,
            root,
            &mut visitor,
        );

        assert!(matches!(
            result,
            Err(PostorderError::MissingNode { id })
                if id == missing
        ));
    }
}