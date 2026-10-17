
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Structural Graph Validation
//!
//! Path: `src/classical/validation/structural.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define a target-independent structural validator for classical computation
//! graphs, including graphs exposed by a future or existing ZCIR adapter.
//!
//! This module is deliberately independent of a particular IR representation.
//! It defines the graph-provider contract, structural invariants, operational
//! budgets, deterministic traversal behavior, and structured failure model.
//!
//! # Authority and ownership
//!
//! The following components retain their existing authority:
//!
//! - `grammar/Zamani.g4`: source syntax.
//! - `grammar/specification/semantic-model.md`: language semantics.
//! - `src/frontend/ast/node/validation/structural.rs`: native AST structure.
//! - `src/classical/ids.rs`: classical-facing canonical ID re-exports.
//! - `src/classical/zcir/`: classical IR integration boundary.
//! - `src/zuir/ids.rs`: canonical general-purpose ZUIR identities.
//!
//! This module owns only validation of a graph supplied through
//! [`StructuralGraph`].
//!
//! It does not define another AST, classical IR, identity allocator, type
//! system, resource manager, scheduler, optimizer, or execution engine.
//!
//! # Structural invariants
//!
//! The validator checks:
//!
//! - the requested root exists;
//! - every requested node exists;
//! - a provider returns the identity requested by the caller;
//! - cycles do not exist in the reachable graph;
//! - shared nodes follow the configured graph policy;
//! - optional node, edge, and depth budgets are respected;
//! - counters do not silently overflow;
//! - traversal follows provider-supplied child order;
//! - failures are returned explicitly.
//!
//! # Deliberate exclusions
//!
//! This module does not validate:
//!
//! - type correctness or generic constraints;
//! - variable and function name resolution;
//! - expression or arithmetic semantics;
//! - effect or ownership correctness;
//! - memory safety of a separate runtime;
//! - control-flow terminator semantics;
//! - resource feasibility or target capabilities;
//! - optimization legality;
//! - ABI compatibility;
//! - hardware availability or placement;
//! - quantum gates, qubits, QEC, routing, or scheduling.
//!
//! These checks belong to their respective canonical owners.
//!
//! # POCO-REAF and scalability
//!
//! No default node-count, edge-count, depth, hardware-count, collection-size,
//! or machine-size ceiling is introduced here.
//!
//! Optional budgets are operational safeguards, not language semantics.
//! Callers may configure them according to their compilation or validation
//! policy. Exhausting a budget is reported explicitly and never means that
//! the source program is inherently invalid.
//!
//! No finite implementation can guarantee literal infinity: memory,
//! representable identifiers, counters, and execution time remain finite.
//! This module avoids introducing arbitrary semantic ceilings of its own.
//!
//! # Algorithm
//!
//! Traversal is iterative, not recursive. This avoids consuming the Rust call
//! stack in proportion to graph depth.
//!
//! A deterministic ordered set tracks visited and currently active node IDs.
//! The provider's child order is preserved. Ordered sets are used only for
//! membership checks; they never determine traversal order.
//!
//! # Integration sequence
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! frontend AST structural validation
//!     |
//!     v
//! semantic analysis
//!     |
//!     v
//! classical IR / ZCIR adapter
//!     |
//!     v
//! StructuralGraph
//!     |
//!     v
//! this structural validator
//!     |
//!     v
//! type / semantic / effect / resource verification
//!     |
//!     v
//! optimization and target lowering
//! ```
//!
//! The AST validator must not be replaced by this graph validator. An AST
//! adapter and a ZCIR adapter serve different validation boundaries.
//!
//! # Integration status
//!
//! This file introduces an independent graph-validation API. An IR-specific
//! adapter must implement [`StructuralGraph`] before the validator can be
//! used directly against canonical ZCIR. The adapter must use the repository's
//! canonical IDs and expose real graph relationships; it must not synthesize
//! IDs, silently omit edges, or reinterpret graph semantics.

use core::fmt;
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// Graph-provider contract
// ---------------------------------------------------------------------------

/// A node in a structural computation graph.
///
/// `id` must be the canonical identity of the represented graph node.
/// `children` contains its outgoing structural references in their
/// authoritative order.
///
/// This type intentionally contains no domain-specific payload. The provider
/// retains ownership of the actual IR node, its types, operations, source
/// provenance, and semantic attributes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralNode<I> {
    /// Canonical identity of this node.
    pub id: I,

    /// Outgoing structural references in authoritative order.
    pub children: Vec<I>,
}

impl<I> StructuralNode<I> {
    /// Constructs a structural node view.
    #[must_use]
    pub fn new(id: I, children: Vec<I>) -> Self {
        Self { id, children }
    }

    /// Constructs a structural node with no outgoing references.
    #[must_use]
    pub fn leaf(id: I) -> Self {
        Self {
            id,
            children: Vec::new(),
        }
    }
}

/// Provides read-only access to a computation graph.
///
/// Implementations must return a node whose identity matches the requested
/// identity. `Ok(None)` means that the identity does not exist.
/// `Err(error)` means that the provider could not answer the request.
///
/// The provider must not omit structural edges to make a graph pass validation.
/// The order of `children` must be stable for a stable graph snapshot.
///
/// # Snapshot contract
///
/// A provider used for validation must expose a consistent read-only snapshot
/// for the duration of the call. Concurrent mutation that changes identities
/// or edges during traversal is outside this trait's guarantees.
///
/// # Identity contract
///
/// IDs must have stable equality and ordering semantics during validation.
/// They must represent canonical node identity, not vector positions,
/// addresses, source offsets, or allocation order inferred by this module.
pub trait StructuralGraph {
    /// Canonical node identity type.
    type Id: Copy + Ord + fmt::Debug;

    /// Provider-specific access failure.
    type Error;

    /// Returns the requested node, or `None` when the ID does not exist.
    ///
    /// The returned node's `id` must equal `id`.
    fn node(
        &self,
        id: Self::Id,
    ) -> Result<Option<StructuralNode<Self::Id>>, Self::Error>;
}

// ---------------------------------------------------------------------------
// Operational policy
// ---------------------------------------------------------------------------

/// The operational budget whose limit was reached.
///
/// These budgets constrain one validation invocation. They do not define
/// source-language validity or a universal limit on program size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuralBudget {
    /// Maximum number of unique nodes validated.
    Nodes,

    /// Maximum number of outgoing references inspected.
    Edges,

    /// Maximum depth of a newly visited node, with root depth equal to zero.
    Depth,
}

impl fmt::Display for StructuralBudget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nodes => formatter.write_str("node"),
            Self::Edges => formatter.write_str("edge"),
            Self::Depth => formatter.write_str("depth"),
        }
    }
}

/// Structural validation policy.
///
/// The default policy has no artificial node, edge, or depth budgets and
/// rejects both cycles and repeated references to completed nodes.
///
/// To validate DAG-shaped graphs, callers may disable `reject_shared_nodes`.
/// Cycles remain forbidden regardless of that setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralValidationConfig {
    /// Optional maximum number of unique nodes validated.
    pub max_nodes: Option<usize>,

    /// Optional maximum number of outgoing references inspected.
    pub max_edges: Option<usize>,

    /// Optional maximum depth; the root is at depth zero.
    pub max_depth: Option<usize>,

    /// Whether a reference to an already completed node is rejected.
    ///
    /// Set to `false` when the graph contract explicitly permits shared
    /// subgraphs. This does not permit cycles.
    pub reject_shared_nodes: bool,
}

impl Default for StructuralValidationConfig {
    fn default() -> Self {
        Self::unrestricted()
    }
}

impl StructuralValidationConfig {
    /// Creates an unrestricted structural validation policy.
    ///
    /// Unrestricted means no artificial node, edge, or depth budget is
    /// imposed by this validator. Real process and platform resource limits
    /// still apply.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
            max_depth: None,
            reject_shared_nodes: true,
        }
    }

    /// Configures an optional node budget.
    #[must_use]
    pub const fn with_max_nodes(mut self, limit: Option<usize>) -> Self {
        self.max_nodes = limit;
        self
    }

    /// Configures an optional edge budget.
    #[must_use]
    pub const fn with_max_edges(mut self, limit: Option<usize>) -> Self {
        self.max_edges = limit;
        self
    }

    /// Configures an optional depth budget.
    #[must_use]
    pub const fn with_max_depth(mut self, limit: Option<usize>) -> Self {
        self.max_depth = limit;
        self
    }

    /// Configures whether completed nodes may be shared by multiple parents.
    ///
    /// Active-path cycles are always rejected.
    #[must_use]
    pub const fn with_reject_shared_nodes(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_shared_nodes = reject;
        self
    }
}

// ---------------------------------------------------------------------------
// Result statistics
// ---------------------------------------------------------------------------

/// Deterministic statistics for a successful validation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StructuralValidationStatistics {
    /// Number of unique nodes validated.
    pub nodes_validated: usize,

    /// Number of outgoing references inspected.
    pub edges_validated: usize,

    /// Maximum depth of any uniquely visited node.
    ///
    /// A graph containing only its root has a maximum depth of zero.
    pub maximum_depth: usize,
}

// ---------------------------------------------------------------------------
// Structured errors
// ---------------------------------------------------------------------------

/// Structural graph validation failure.
///
/// Provider failures are preserved as their original error value. Callers can
/// inspect the variant instead of parsing an error message.
#[derive(Debug, PartialEq, Eq)]
pub enum StructuralValidationError<I, E> {
    /// The requested root does not exist.
    MissingRoot {
        /// Requested root identity.
        id: I,
    },

    /// A referenced node does not exist.
    MissingNode {
        /// Node containing the missing reference, when known.
        referenced_by: Option<I>,

        /// Referenced identity that could not be resolved.
        id: I,
    },

    /// A provider returned a node with a different identity.
    IdentityMismatch {
        /// Requested identity.
        requested: I,

        /// Identity returned by the provider.
        returned: I,
    },

    /// A directed cycle was found.
    Cycle {
        /// Parent node whose outgoing edge closes the cycle.
        from: I,

        /// Referenced node already on the active traversal path.
        to: I,
    },

    /// A previously completed node was referenced again while shared nodes
    /// were disallowed.
    SharedNode {
        /// Node containing the repeated reference.
        referenced_by: I,

        /// Previously completed node.
        id: I,
    },

    /// A configured operational budget was exceeded.
    BudgetExceeded {
        /// Budget that was exceeded.
        budget: StructuralBudget,

        /// Configured operational limit.
        limit: usize,

        /// Observed count or depth that exceeded the limit.
        observed: usize,
    },

    /// A validation counter could not represent its next value.
    CounterOverflow {
        /// Counter that could not be incremented.
        budget: StructuralBudget,
    },

    /// The graph provider failed.
    Provider(E),
}

impl<I, E> fmt::Display for StructuralValidationError<I, E>
where
    I: fmt::Debug,
    E: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRoot { id } => {
                write!(formatter, "structural graph root {id:?} does not exist")
            }
            Self::MissingNode { referenced_by, id } => {
                if let Some(parent) = referenced_by {
                    write!(
                        formatter,
                        "structural node {parent:?} references missing node {id:?}"
                    )
                } else {
                    write!(
                        formatter,
                        "structural graph references missing node {id:?}"
                    )
                }
            }
            Self::IdentityMismatch { requested, returned } => {
                write!(
                    formatter,
                    "graph provider returned identity {returned:?} \
                     when {requested:?} was requested"
                )
            }
            Self::Cycle { from, to } => {
                write!(
                    formatter,
                    "structural graph contains a cycle from {from:?} to {to:?}"
                )
            }
            Self::SharedNode { referenced_by, id } => {
                write!(
                    formatter,
                    "node {referenced_by:?} references previously \
                     completed node {id:?}, but shared nodes are disallowed"
                )
            }
            Self::BudgetExceeded {
                budget,
                limit,
                observed,
            } => {
                write!(
                    formatter,
                    "structural validation {budget} budget exceeded: \
                     limit {limit}, observed {observed}"
                )
            }
            Self::CounterOverflow { budget } => {
                write!(
                    formatter,
                    "structural validation {budget} counter overflowed"
                )
            }
            Self::Provider(error) => {
                write!(
                    formatter,
                    "structural graph provider failed: {error}"
                )
            }
        }
    }
}

impl<I, E> std::error::Error for StructuralValidationError<I, E>
where
    I: fmt::Debug,
    E: fmt::Debug + fmt::Display + 'static,
{
}

// ---------------------------------------------------------------------------
// Iterative traversal
// ---------------------------------------------------------------------------

/// A traversal frame for iterative depth-first validation.
struct Frame<I> {
    id: I,
    children: Vec<I>,
    next_child: usize,
    depth: usize,
}

impl<I> Frame<I> {
    fn new(node: StructuralNode<I>, depth: usize) -> Self {
        Self {
            id: node.id,
            children: node.children,
            next_child: 0,
            depth,
        }
    }
}

/// Validates the structure reachable from `root`.
///
/// The validator is stateless: all graph-specific state is local to the call.
/// It does not allocate IDs, mutate the graph, inspect hardware, or invoke
/// compiler transformations.
///
/// # Errors
///
/// Returns a typed error for missing nodes, identity mismatches, cycles,
/// disallowed sharing, provider failures, counter overflow, or budget
/// exhaustion.
///
/// # Complexity
///
/// For a graph with `V` unique reachable nodes and `E` inspected edges, the
/// traversal performs `O((V + E) log V)` membership work using ordered sets,
/// plus the cost of the provider and storage for the traversal state.
///
/// Memory use is proportional to the reachable graph's node identities,
/// active traversal path, and provider-returned child lists. This module does
/// not promise constant memory for arbitrarily large graphs.
pub fn validate_structure<G>(
    graph: &G,
    root: G::Id,
) -> Result<StructuralValidationStatistics, StructuralValidationError<G::Id, G::Error>>
where
    G: StructuralGraph,
{
    validate_structure_with_config(
        graph,
        root,
        StructuralValidationConfig::default(),
    )
}

/// Validates the structure reachable from `root` under an explicit policy.
pub fn validate_structure_with_config<G>(
    graph: &G,
    root: G::Id,
    config: StructuralValidationConfig,
) -> Result<StructuralValidationStatistics, StructuralValidationError<G::Id, G::Error>>
where
    G: StructuralGraph,
{
    let root_node = graph
        .node(root)
        .map_err(StructuralValidationError::Provider)?
        .ok_or(StructuralValidationError::MissingRoot { id: root })?;

    if root_node.id != root {
        return Err(StructuralValidationError::IdentityMismatch {
            requested: root,
            returned: root_node.id,
        });
    }

    if let Some(limit) = config.max_nodes {
        if limit == 0 {
            return Err(StructuralValidationError::BudgetExceeded {
                budget: StructuralBudget::Nodes,
                limit,
                observed: 1,
            });
        }
    }

    if let Some(limit) = config.max_depth {
        if limit == 0 && !root_node.children.is_empty() {
            // A depth limit of zero permits the root itself. Whether any
            // child is actually reachable is handled during traversal.
            // Do not reject here: a root with no children is valid.
        }
    }

    let mut visited = BTreeSet::new();
    let mut active = BTreeSet::new();
    let mut stack = Vec::new();

    visited.insert(root);
    active.insert(root);

    let mut statistics = StructuralValidationStatistics {
        nodes_validated: 1,
        edges_validated: 0,
        maximum_depth: 0,
    };

    stack.push(Frame::new(root_node, 0));

    while !stack.is_empty() {
        let has_more_children = {
            let frame = stack.last().expect("stack checked as non-empty");

            frame.next_child < frame.children.len()
        };

        if !has_more_children {
            let completed = stack.pop().expect("stack checked as non-empty");

            active.remove(&completed.id);
            continue;
        }

        // Advance the parent's cursor before processing the child. The parent
        // frame remains on the stack throughout child traversal.
        let (parent_id, child_id, child_depth) = {
            let parent = stack.last_mut().expect("stack checked as non-empty");

            let child_id = parent.children[parent.next_child];
            parent.next_child += 1;

            let child_depth = parent.depth.checked_add(1).ok_or(
                StructuralValidationError::CounterOverflow {
                    budget: StructuralBudget::Depth,
                },
            )?;

            (parent.id, child_id, child_depth)
        };

        statistics.edges_validated = statistics
            .edges_validated
            .checked_add(1)
            .ok_or(StructuralValidationError::CounterOverflow {
                budget: StructuralBudget::Edges,
            })?;

        if let Some(limit) = config.max_edges {
            if statistics.edges_validated > limit {
                return Err(StructuralValidationError::BudgetExceeded {
                    budget: StructuralBudget::Edges,
                    limit,
                    observed: statistics.edges_validated,
                });
            }
        }

        // A back-edge to an active node is a cycle, even when shared completed
        // nodes are permitted.
        if active.contains(&child_id) {
            return Err(StructuralValidationError::Cycle {
                from: parent_id,
                to: child_id,
            });
        }

        // A completed node can be reused only when the graph contract allows
        // shared subgraphs. The edge itself has already been counted.
        if visited.contains(&child_id) {
            if config.reject_shared_nodes {
                return Err(StructuralValidationError::SharedNode {
                    referenced_by: parent_id,
                    id: child_id,
                });
            }

            continue;
        }

        if let Some(limit) = config.max_depth {
            if child_depth > limit {
                return Err(StructuralValidationError::BudgetExceeded {
                    budget: StructuralBudget::Depth,
                    limit,
                    observed: child_depth,
                });
            }
        }

        let next_node_count = statistics
            .nodes_validated
            .checked_add(1)
            .ok_or(StructuralValidationError::CounterOverflow {
                budget: StructuralBudget::Nodes,
            })?;

        if let Some(limit) = config.max_nodes {
            if next_node_count > limit {
                return Err(StructuralValidationError::BudgetExceeded {
                    budget: StructuralBudget::Nodes,
                    limit,
                    observed: next_node_count,
                });
            }
        }

        let child = graph
            .node(child_id)
            .map_err(StructuralValidationError::Provider)?
            .ok_or(StructuralValidationError::MissingNode {
                referenced_by: Some(parent_id),
                id: child_id,
            })?;

        if child.id != child_id {
            return Err(StructuralValidationError::IdentityMismatch {
                requested: child_id,
                returned: child.id,
            });
        }

        visited.insert(child_id);
        active.insert(child_id);

        statistics.nodes_validated = next_node_count;
        statistics.maximum_depth = statistics.maximum_depth.max(child_depth);

        stack.push(Frame::new(child, child_depth));
    }

    Ok(statistics)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Default)]
    struct TestGraph {
        nodes: BTreeMap<u64, StructuralNode<u64>>,
        failures: BTreeSet<u64>,
    }

    impl TestGraph {
        fn with_nodes(nodes: impl IntoIterator<Item = StructuralNode<u64>>) -> Self {
            Self {
                nodes: nodes.into_iter().map(|node| (node.id, node)).collect(),
                failures: BTreeSet::new(),
            }
        }

        fn insert(&mut self, node: StructuralNode<u64>) {
            self.nodes.insert(node.id, node);
        }
    }

    impl StructuralGraph for TestGraph {
        type Id = u64;
        type Error = &'static str;

        fn node(
            &self,
            id: Self::Id,
        ) -> Result<Option<StructuralNode<Self::Id>>, Self::Error> {
            if self.failures.contains(&id) {
                return Err("test provider failure");
            }

            Ok(self.nodes.get(&id).cloned())
        }
    }

    #[test]
    fn validates_a_single_leaf() {
        let graph = TestGraph::with_nodes([StructuralNode::leaf(1)]);

        let statistics = validate_structure(&graph, 1).expect("valid graph");

        assert_eq!(
            statistics,
            StructuralValidationStatistics {
                nodes_validated: 1,
                edges_validated: 0,
                maximum_depth: 0,
            }
        );
    }

    #[test]
    fn validates_a_tree_and_counts_edges() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2, 3]),
            StructuralNode::leaf(2),
            StructuralNode::new(3, vec![4]),
            StructuralNode::leaf(4),
        ]);

        let statistics = validate_structure(&graph, 1).expect("valid tree");

        assert_eq!(statistics.nodes_validated, 4);
        assert_eq!(statistics.edges_validated, 3);
        assert_eq!(statistics.maximum_depth, 2);
    }

    #[test]
    fn rejects_a_missing_root() {
        let graph = TestGraph::default();

        assert!(matches!(
            validate_structure(&graph, 7),
            Err(StructuralValidationError::MissingRoot { id: 7 })
        ));
    }

    #[test]
    fn rejects_a_missing_child_reference() {
        let graph = TestGraph::with_nodes([StructuralNode::new(1, vec![9])]);

        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::MissingNode {
                referenced_by: Some(1),
                id: 9,
            })
        ));
    }

    #[test]
    fn rejects_cycles() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2]),
            StructuralNode::new(2, vec![1]),
        ]);

        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::Cycle { from: 2, to: 1 })
        ));
    }

    #[test]
    fn rejects_self_cycles() {
        let graph = TestGraph::with_nodes([StructuralNode::new(1, vec![1])]);

        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::Cycle { from: 1, to: 1 })
        ));
    }

    #[test]
    fn rejects_shared_nodes_by_default() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2, 3]),
            StructuralNode::new(2, vec![4]),
            StructuralNode::new(3, vec![4]),
            StructuralNode::leaf(4),
        ]);

        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::SharedNode {
                referenced_by: 3,
                id: 4,
            })
        ));
    }

    #[test]
    fn permits_shared_nodes_when_configured_as_a_dag() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2, 3]),
            StructuralNode::new(2, vec![4]),
            StructuralNode::new(3, vec![4]),
            StructuralNode::leaf(4),
        ]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_reject_shared_nodes(false);

        let statistics =
            validate_structure_with_config(&graph, 1, config)
                .expect("DAG must be accepted");

        assert_eq!(statistics.nodes_validated, 4);
        assert_eq!(statistics.edges_validated, 4);
        assert_eq!(statistics.maximum_depth, 2);
    }

    #[test]
    fn node_budget_is_enforced() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2]),
            StructuralNode::leaf(2),
        ]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_max_nodes(Some(1));

        assert!(matches!(
            validate_structure_with_config(&graph, 1, config),
            Err(StructuralValidationError::BudgetExceeded {
                budget: StructuralBudget::Nodes,
                limit: 1,
                observed: 2,
            })
        ));
    }

    #[test]
    fn edge_budget_is_enforced() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2, 3]),
            StructuralNode::leaf(2),
            StructuralNode::leaf(3),
        ]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_max_edges(Some(1));

        assert!(matches!(
            validate_structure_with_config(&graph, 1, config),
            Err(StructuralValidationError::BudgetExceeded {
                budget: StructuralBudget::Edges,
                limit: 1,
                observed: 2,
            })
        ));
    }

    #[test]
    fn depth_budget_is_enforced() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2]),
            StructuralNode::new(2, vec![3]),
            StructuralNode::leaf(3),
        ]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_max_depth(Some(1));

        assert!(matches!(
            validate_structure_with_config(&graph, 1, config),
            Err(StructuralValidationError::BudgetExceeded {
                budget: StructuralBudget::Depth,
                limit: 1,
                observed: 2,
            })
        ));
    }

    #[test]
    fn zero_node_budget_rejects_the_root() {
        let graph = TestGraph::with_nodes([StructuralNode::leaf(1)]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_max_nodes(Some(0));

        assert!(matches!(
            validate_structure_with_config(&graph, 1, config),
            Err(StructuralValidationError::BudgetExceeded {
                budget: StructuralBudget::Nodes,
                limit: 0,
                observed: 1,
            })
        ));
    }

    #[test]
    fn zero_depth_budget_accepts_a_leaf() {
        let graph = TestGraph::with_nodes([StructuralNode::leaf(1)]);

        let config =
            StructuralValidationConfig::unrestricted()
                .with_max_depth(Some(0));

        let statistics =
            validate_structure_with_config(&graph, 1, config)
                .expect("root at depth zero must pass");

        assert_eq!(statistics.maximum_depth, 0);
    }

    #[test]
    fn provider_errors_are_preserved() {
        let mut graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![2]),
            StructuralNode::leaf(2),
        ]);

        graph.failures.insert(2);

        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::Provider(
                "test provider failure"
            ))
        ));
    }

    #[test]
    fn provider_identity_mismatch_is_rejected() {
        struct MismatchedGraph;

        impl StructuralGraph for MismatchedGraph {
            type Id = u64;
            type Error = &'static str;

            fn node(
                &self,
                id: u64,
            ) -> Result<Option<StructuralNode<u64>>, Self::Error> {
                if id == 1 {
                    Ok(Some(StructuralNode::leaf(2)))
                } else {
                    Ok(None)
                }
            }
        }

        assert!(matches!(
            validate_structure(&MismatchedGraph, 1),
            Err(StructuralValidationError::IdentityMismatch {
                requested: 1,
                returned: 2,
            })
        ));
    }

    #[test]
    fn default_policy_has_no_artificial_size_budgets() {
        let config = StructuralValidationConfig::default();

        assert_eq!(config.max_nodes, None);
        assert_eq!(config.max_edges, None);
        assert_eq!(config.max_depth, None);
        assert!(config.reject_shared_nodes);
    }

    #[test]
    fn child_order_is_preserved() {
        let graph = TestGraph::with_nodes([
            StructuralNode::new(1, vec![3, 2]),
            StructuralNode::leaf(2),
            StructuralNode::new(3, vec![9]),
        ]);

        // The first missing reference encountered must be node 9 because
        // node 3 is the first child in the provider-supplied order.
        assert!(matches!(
            validate_structure(&graph, 1),
            Err(StructuralValidationError::MissingNode {
                referenced_by: Some(3),
                id: 9,
            })
        ));
    }

    #[test]
    fn unrelated_unreachable_nodes_do_not_affect_validation() {
        let graph = TestGraph::with_nodes([
            StructuralNode::leaf(1),
            StructuralNode::new(100, vec![999]),
        ]);

        let statistics =
            validate_structure(&graph, 1).expect("root component is valid");

        assert_eq!(statistics.nodes_validated, 1);
        assert_eq!(statistics.edges_validated, 0);
    }
}
