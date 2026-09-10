//! AST folding and transformation infrastructure.
//!
//! # Architectural role
//!
//! This module provides the transformation/folding layer for the native
//! Zamani AST.
//!
//! The native AST is source-oriented and domain-neutral. This module therefore
//! MUST NOT contain knowledge of:
//!
//! - quantum hardware;
//! - physical qubits;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC implementation;
//! - resilience implementation;
//! - backend instructions;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor-specific operations.
//!
//! Those concerns belong to later compiler/domain layers.
//!
//! # Traversal model
//!
//! Folding is implemented with an explicit heap-backed work stack rather than
//! Rust call-stack recursion. Consequently, traversal depth is constrained
//! only by available resources or an explicitly supplied compiler policy.
//!
//! There is no language-level maximum depth, node count, qubit count, register
//! size, or machine size in this module.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the generic fold protocol;
//! - iterative fold orchestration;
//! - fold configuration;
//! - fold diagnostics/errors;
//! - deterministic traversal semantics.
//!
//! This module does NOT own:
//!
//! - AST node definitions;
//! - NodeId allocation;
//! - source spans;
//! - semantic analysis;
//! - type checking;
//! - ZUIR;
//! - quantum IR;
//! - backend lowering.
//!
//! # Integration
//!
//! Conceptual pipeline:
//!
//! ```text
//! Source
//!   |
//!   v
//! Lexer
//!   |
//!   v
//! Parser
//!   |
//!   v
//! Native AST
//!   |
//!   +--> structural validation
//!   |
//!   +--> immutable visitors
//!   |
//!   +--> folds / source-level transformations
//!   |
//!   v
//! Semantic Analysis
//!   |
//!   v
//! Semantic Model
//!   |
//!   v
//! ZUIR
//! ```
//!
//! A fold is therefore a source-AST transformation facility. It must not be
//! used as a replacement for semantic lowering or domain-specific compilation.

use std::collections::HashSet;

use super::visitor::{AstVisitor, VisitControl};
use super::super::{Node, NodeId};

/// Determines whether traversal may be cancelled.
///
/// This intentionally lives behind a very small interface so AST traversal
/// does not depend on the compiler's particular cancellation implementation.
///
/// Compiler layers may adapt their own cancellation token to this trait.
pub trait FoldCancellation {
    /// Returns `true` when traversal should terminate.
    fn is_cancelled(&self) -> bool;
}

/// Configuration controlling resource-sensitive fold execution.
///
/// All limits are optional. `None` means that this layer imposes no limit.
///
/// These limits are compiler/resource-safety policies, NOT language
/// semantics. In particular, this type must never be changed to introduce a
/// fixed language-level AST depth or node count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldOptions<'a> {
    /// Optional maximum number of node occurrences visited.
    ///
    /// This counts visits, not unique NodeIds. A shared DAG node may therefore
    /// be visited more than once.
    pub max_nodes: Option<usize>,

    /// Optional maximum traversal depth.
    ///
    /// The root node has depth zero.
    pub max_depth: Option<usize>,

    /// Optional external cancellation source.
    pub cancellation: Option<&'a dyn FoldCancellation>,
}

impl<'a> Default for FoldOptions<'a> {
    fn default() -> Self {
        Self {
            max_nodes: None,
            max_depth: None,
            cancellation: None,
        }
    }
}

impl<'a> FoldOptions<'a> {
    /// Creates an unlimited fold configuration.
    ///
    /// "Unlimited" means that this layer does not impose an artificial
    /// semantic limit. Actual execution remains bounded by available
    /// resources and the host process/runtime.
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_depth: None,
            cancellation: None,
        }
    }

    /// Sets an optional node-visit budget.
    pub const fn with_max_nodes(mut self, limit: Option<usize>) -> Self {
        self.max_nodes = limit;
        self
    }

    /// Sets an optional depth budget.
    pub const fn with_max_depth(mut self, limit: Option<usize>) -> Self {
        self.max_depth = limit;
        self
    }

    /// Sets an optional cancellation source.
    pub const fn with_cancellation(
        mut self,
        cancellation: Option<&'a dyn FoldCancellation>,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }
}

/// Result information produced by a successful fold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoldReport {
    /// Number of node occurrences visited.
    pub visited_nodes: usize,

    /// Greatest depth reached.
    pub maximum_depth: usize,

    /// Whether the visitor deliberately requested early termination.
    pub stopped_early: bool,
}

/// Errors produced by the fold engine.
///
/// Visitor errors and provider errors are kept distinct so callers can
/// preserve the original error type without stringifying or losing context.
#[derive(Debug, PartialEq, Eq)]
pub enum FoldError<ProviderError, VisitorError> {
    /// The AST provider could not provide a node or its children.
    Provider(ProviderError),

    /// The visitor/folder rejected or failed while processing a node.
    Visitor(VisitorError),

    /// Traversal was cancelled by an external cancellation source.
    Cancelled,

    /// The configured node-visit budget was exceeded.
    NodeLimitExceeded {
        /// Configured maximum.
        limit: usize,
    },

    /// The configured depth budget was exceeded.
    DepthLimitExceeded {
        /// Depth at which the violation occurred.
        depth: usize,

        /// Configured maximum.
        limit: usize,
    },

    /// A cycle was found on the active traversal path.
    ///
    /// Native ASTs are expected to be trees or otherwise acyclic structures.
    /// Shared DAG nodes are allowed: only an active-path cycle is rejected.
    CycleDetected {
        /// Node participating in the cycle.
        node_id: NodeId,
    },
}

/// Provides read access to the canonical AST.
///
/// This is deliberately an abstraction over AST storage. The fold engine must
/// not assume that nodes are stored recursively, in a Vec, arena, map, or any
/// particular representation.
///
/// Implementations may use:
///
/// - an arena;
/// - indexed storage;
/// - immutable persistent storage;
/// - a program/module store;
/// - another canonical AST container.
///
/// The provider owns the actual child topology.
///
/// # Determinism
///
/// `children()` MUST yield children in the canonical AST order. The fold
/// engine never sorts children because doing so could destroy source order and
/// introduce unnecessary work.
///
/// # Extension support
///
/// The fold engine deliberately does not match on `NodeKind`. Therefore a new
/// AST extension does not require this module to be modified merely because a
/// new node kind has been introduced.
pub trait AstNodeProvider {
    /// Error produced when accessing AST storage.
    type Error;

    /// Iterator over a node's children.
    ///
    /// The iterator borrows the provider and therefore avoids allocating a
    /// new child vector for every node.
    type Children<'a>: Iterator<Item = NodeId> + 'a
    where
        Self: 'a;

    /// Returns the canonical node identified by `node_id`.
    fn node<'a>(&'a self, node_id: NodeId) -> Result<&'a Node, Self::Error>;

    /// Returns the node's children in canonical traversal order.
    fn children<'a>(
        &'a self,
        node_id: NodeId,
    ) -> Result<Self::Children<'a>, Self::Error>;
}

/// A generic fold over the canonical Zamani AST.
///
/// A folder is an [`AstVisitor`] whose visit callbacks can request:
///
/// - normal descent;
/// - skipping the current node's children;
/// - stopping the traversal.
///
/// This type intentionally performs no node-kind dispatch itself.
///
/// # Why no `NodeKind` match?
///
/// A central match such as:
///
/// ```text
/// match node.kind {
///     QuantumGate => ...
///     Function => ...
///     ...
/// }
/// ```
///
/// would make the core traversal coupled to a closed list of constructs.
/// That directly conflicts with Zamani's extensibility and POCO-REAF goals.
///
/// Node-specific transformations belong in the supplied visitor/folder.
///
/// # Stack safety
///
/// Traversal uses an explicit `Vec` of frames. It does not recursively invoke
/// Rust functions for AST depth.
pub fn fold<S, F>(
    provider: &S,
    folder: &mut F,
    root: NodeId,
    options: FoldOptions<'_>,
) -> Result<FoldReport, FoldError<S::Error, F::Error>>
where
    S: AstNodeProvider,
    F: AstVisitor,
{
    let mut stack = Vec::new();
    let mut active_path = HashSet::new();

    stack.push(Frame::Enter {
        node_id: root,
        depth: 0,
    });

    let mut report = FoldReport::default();

    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Enter { node_id, depth } => {
                if let Some(cancellation) = options.cancellation {
                    if cancellation.is_cancelled() {
                        return Err(FoldError::Cancelled);
                    }
                }

                if let Some(limit) = options.max_depth {
                    if depth > limit {
                        return Err(FoldError::DepthLimitExceeded {
                            depth,
                            limit,
                        });
                    }
                }

                if let Some(limit) = options.max_nodes {
                    if report.visited_nodes >= limit {
                        return Err(FoldError::NodeLimitExceeded { limit });
                    }
                }

                if !active_path.insert(node_id) {
                    return Err(FoldError::CycleDetected { node_id });
                }

                let node = provider
                    .node(node_id)
                    .map_err(FoldError::Provider)?;

                report.visited_nodes = report
                    .visited_nodes
                    .checked_add(1)
                    .ok_or(FoldError::NodeLimitExceeded {
                        limit: usize::MAX,
                    })?;

                report.maximum_depth = report.maximum_depth.max(depth);

                match folder
                    .enter(node)
                    .map_err(FoldError::Visitor)?
                {
                    VisitControl::Continue => {
                        // Leave is deliberately scheduled before Children so
                        // the stack executes:
                        //
                        //   enter(parent)
                        //   enter(child_1)
                        //   ...
                        //   leave(child_1)
                        //   ...
                        //   leave(parent)
                        //
                        // This provides deterministic balanced traversal for
                        // nodes whose child traversal completes normally.
                        stack.push(Frame::Leave {
                            node_id,
                            node,
                        });

                        let children = provider
                            .children(node_id)
                            .map_err(FoldError::Provider)?;

                        stack.push(Frame::Children {
                            node_id,
                            depth,
                            children,
                        });
                    }

                    VisitControl::SkipChildren => {
                        // A skipped node has still been entered, therefore its
                        // leave callback is invoked immediately to preserve
                        // enter/leave pairing.
                        folder
                            .leave(node)
                            .map_err(FoldError::Visitor)?;

                        active_path.remove(&node_id);
                    }

                    VisitControl::Stop => {
                        // Stop is treated as a completed visit of the current
                        // node. This preserves the visitor's enter/leave
                        // contract without descending further.
                        folder
                            .leave(node)
                            .map_err(FoldError::Visitor)?;

                        active_path.remove(&node_id);
                        report.stopped_early = true;

                        return Ok(report);
                    }
                }
            }

            Frame::Children {
                node_id,
                depth,
                mut children,
            } => {
                if let Some(cancellation) = options.cancellation {
                    if cancellation.is_cancelled() {
                        return Err(FoldError::Cancelled);
                    }
                }

                match children.next() {
                    Some(child_id) => {
                        // Put the iterator back first. The child is then
                        // processed before requesting the next sibling.
                        stack.push(Frame::Children {
                            node_id,
                            depth,
                            children,
                        });

                        let child_depth = depth
                            .checked_add(1)
                            .ok_or_else(|| {
                                FoldError::DepthLimitExceeded {
                                    depth: usize::MAX,
                                    limit: options
                                        .max_depth
                                        .unwrap_or(usize::MAX),
                                }
                            })?;

                        stack.push(Frame::Enter {
                            node_id: child_id,
                            depth: child_depth,
                        });
                    }

                    None => {
                        // No children remain. The corresponding Leave frame
                        // already exists underneath this frame.
                    }
                }
            }

            Frame::Leave { node_id, node } => {
                folder
                    .leave(node)
                    .map_err(FoldError::Visitor)?;

                active_path.remove(&node_id);
            }
        }
    }

    Ok(report)
}

/// Internal iterative traversal frame.
///
/// The frame owns no AST node storage. Nodes remain owned by the canonical
/// provider.
///
/// `Children` stores the provider's iterator instead of collecting every
/// child into a temporary Vec. This is important for very high fan-out AST
/// nodes.
enum Frame<'a, S>
where
    S: AstNodeProvider,
{
    Enter {
        node_id: NodeId,
        depth: usize,
    },

    Children {
        node_id: NodeId,
        depth: usize,
        children: S::Children<'a>,
    },

    Leave {
        node_id: NodeId,
        node: &'a Node,
    },
}