
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effect Inference
//!
//! File: `src/frontend/semantic/effects/inference.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Infer target-independent effect rows for expressions, declarations,
//! functions, calls, and compositions from an explicitly constructed semantic
//! dependency graph.
//!
//! This module implements effect propagation. It does not parse source,
//! resolve names, type-check expressions, authorize effects, discover target
//! capabilities, allocate resources, or lower programs to an IR.
//!
//! ## AUTHORITY
//!
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-composition.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `src/frontend/semantic/effects/model.rs`
//!
//! The grammar governs source syntax. The effect specification governs
//! semantic meaning. `model.rs` owns the canonical effect-row representation.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - the effect-inference dependency graph;
//! - dependency propagation modes;
//! - worklist-based effect propagation;
//! - recursive and mutually recursive effect fixed points;
//! - deterministic inference results;
//! - graph validation and inference-specific errors.
//!
//! ## DOES NOT OWN
//!
//! - AST storage or AST construction;
//! - name resolution or overload resolution;
//! - canonical effect identities or effect-row representation;
//! - type inference or type unification;
//! - effect declaration checking or subtyping;
//! - handler semantics or effect discharge;
//! - effect authorization or security policies;
//! - capabilities, resource feasibility, or allocation;
//! - hardware selection, quantum routing, scheduling, or execution;
//! - classical IR, `quantum::ir`, HDL IR, or target lowering.
//!
//! ## INTEGRATION CONTRACT
//!
//! The AST traversal and name-resolution passes construct an
//! `EffectInferenceGraph`:
//!
//! 1. Register each semantic node with its locally established effect row.
//! 2. Add propagating dependencies for evaluated children and resolved calls.
//! 3. Add latent dependencies for bodies whose effects do not occur merely
//!    because their containing value is created.
//! 4. Run `infer_effects` after the graph has been assembled.
//! 5. Attach each inferred row to its corresponding AST `NodeId` in the
//!    semantic facts/model.
//! 6. Let effect checking and authorization consume the inferred results.
//!
//! A propagating edge `caller -> callee` means that the caller inherits the
//! callee's effects. A latent edge records a structural dependency without
//! propagating the target's effects into the source node.
//!
//! The graph may contain cycles. Recursive calls are legal graph structures;
//! whether a recursive declaration is legal is decided by other semantic
//! passes.
//!
//! ## FIXED-POINT GUARANTEE
//!
//! Inference starts with each node's local effects and repeatedly propagates
//! the effects of dependencies to their dependents until no row changes.
//!
//! The transfer function is monotonic: propagation only adds effect instances
//! and effect variables. Since the graph and its initial rows are finite,
//! this algorithm terminates after finitely many successful additions, subject
//! to available memory and execution resources.
//!
//! No arbitrary iteration count, hardware limit, effect-count limit, domain
//! limit, or recursion-depth ceiling is introduced.
//!
//! ## DETERMINISM
//!
//! Nodes and dependencies use ordered collections. The worklist is FIFO, and
//! each node is queued at most once at a time. Equivalent graphs yield
//! structurally equivalent results independently of insertion order.
//!
//! ## ERROR HANDLING
//!
//! Invalid rows, duplicate nodes, missing graph nodes, and dependencies whose
//! source node is absent are reported explicitly. An error is not an empty
//! effect row and must not be interpreted as proof of purity.
//!
//! ## FREEZE CRITERIA
//!
//! - The effect-model API and AST `NodeId` API compile.
//! - Recursive and mutually recursive propagation tests pass.
//! - Latent dependencies do not leak effects into their containing values.
//! - Malformed graphs fail explicitly.
//! - Results are deterministic.
//! - The module remains independent of target-specific implementation.
//! - Workspace and integration checks pass after module integration.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

use super::model::{EffectModelError, EffectRow};

/// Describes whether a dependency contributes effects to its source node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EffectPropagation {
    /// The dependency's effects contribute to the source node's effects.
    ///
    /// Use for evaluated child expressions, executed statements, and resolved
    /// calls whose effects are part of the enclosing computation.
    Propagates,

    /// The dependency is structurally related but is not evaluated merely
    /// because the source node is evaluated.
    ///
    /// Use, for example, for a closure or lambda body when constructing the
    /// closure does not execute that body.
    ///
    /// If the closure is subsequently invoked, the resolved invocation must
    /// have a propagating dependency on the callable's effect summary.
    Latent,
}

/// A dependency on another node in the semantic inference graph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectDependency {
    target: NodeId,
    propagation: EffectPropagation,
}

impl EffectDependency {
    /// Creates a dependency on another semantic node.
    #[must_use]
    pub const fn new(
        target: NodeId,
        propagation: EffectPropagation,
    ) -> Self {
        Self {
            target,
            propagation,
        }
    }

    /// Returns the dependency target.
    #[must_use]
    pub const fn target(self) -> NodeId {
        self.target
    }

    /// Returns the propagation behavior.
    #[must_use]
    pub const fn propagation(self) -> EffectPropagation {
        self.propagation
    }
}

/// One node's local effect information and dependency edges.
///
/// `local_effects` contains effects established directly by earlier semantic
/// passes. It must not contain effects that are only inferred from the
/// dependencies stored here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectInferenceNode {
    id: NodeId,
    local_effects: EffectRow,
    dependencies: BTreeSet<EffectDependency>,
}

impl EffectInferenceNode {
    /// Creates a graph node with its already-established local effect row.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied effect row is structurally invalid.
    pub fn new(
        id: NodeId,
        local_effects: EffectRow,
    ) -> Result<Self, EffectInferenceError> {
        local_effects
            .validate()
            .map_err(|source| EffectInferenceError::InvalidEffectRow {
                node: id,
                source,
            })?;

        Ok(Self {
            id,
            local_effects,
            dependencies: BTreeSet::new(),
        })
    }

    /// Returns the AST or semantic node identity.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the local effect row.
    #[must_use]
    pub fn local_effects(&self) -> &EffectRow {
        &self.local_effects
    }

    /// Returns the dependencies in deterministic order.
    pub fn dependencies(
        &self,
    ) -> impl ExactSizeIterator<Item = &EffectDependency> {
        self.dependencies.iter()
    }
}

/// A validated container for nodes participating in effect inference.
///
/// This graph is a semantic side structure. It does not replace or duplicate
/// the canonical AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectInferenceGraph {
    nodes: BTreeMap<NodeId, EffectInferenceNode>,
}

impl EffectInferenceGraph {
    /// Creates an empty effect-inference graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered graph nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns a registered node, if present.
    #[must_use]
    pub fn get(&self, id: NodeId) -> Option<&EffectInferenceNode> {
        self.nodes.get(&id)
    }

    /// Iterates over registered nodes in stable identity order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&NodeId, &EffectInferenceNode)> {
        self.nodes.iter()
    }

    /// Registers a node with its local effects.
    ///
    /// Registration rejects duplicate identities instead of silently
    /// overwriting previously collected semantic information.
    ///
    /// Dependencies may refer to nodes registered later. All dependency
    /// targets are checked before inference starts.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateNode` when the identity is already registered, or
    /// `InvalidEffectRow` when the local row is malformed.
    pub fn add_node(
        &mut self,
        id: NodeId,
        local_effects: EffectRow,
    ) -> Result<(), EffectInferenceError> {
        if self.nodes.contains_key(&id) {
            return Err(EffectInferenceError::DuplicateNode { node: id });
        }

        let node = EffectInferenceNode::new(id, local_effects)?;
        self.nodes.insert(id, node);
        Ok(())
    }

    /// Adds a dependency from an existing source node to a target node.
    ///
    /// The target may be registered later, allowing forward references and
    /// mutually recursive call graphs to be assembled in multiple passes.
    ///
    /// Re-adding the same dependency is idempotent.
    ///
    /// # Errors
    ///
    /// Returns `MissingSourceNode` when the source node is not registered.
    pub fn add_dependency(
        &mut self,
        source: NodeId,
        target: NodeId,
        propagation: EffectPropagation,
    ) -> Result<bool, EffectInferenceError> {
        let node = self
            .nodes
            .get_mut(&source)
            .ok_or(EffectInferenceError::MissingSourceNode {
                source,
                target,
            })?;

        Ok(node.dependencies.insert(EffectDependency::new(
            target,
            propagation,
        )))
    }

    /// Adds local effects to an existing node.
    ///
    /// This is useful when separate AST traversal passes establish different
    /// direct effects for the same expression or declaration.
    ///
    /// The operation is monotonic: it unions effects and variables and never
    /// removes a previously established effect.
    ///
    /// # Errors
    ///
    /// Returns an error if the row is malformed or the node is missing.
    pub fn add_local_effects(
        &mut self,
        id: NodeId,
        effects: &EffectRow,
    ) -> Result<bool, EffectInferenceError> {
        effects
            .validate()
            .map_err(|source| EffectInferenceError::InvalidEffectRow {
                node: id,
                source,
            })?;

        let node = self
            .nodes
            .get_mut(&id)
            .ok_or(EffectInferenceError::MissingNode { node: id })?;

        let combined = node.local_effects.union(effects);
        let changed = combined != node.local_effects;

        if changed {
            node.local_effects = combined;
        }

        Ok(changed)
    }

    /// Validates all registered nodes and dependency targets.
    ///
    /// # Errors
    ///
    /// Returns the first structural error in deterministic traversal order.
    pub fn validate(&self) -> Result<(), EffectInferenceError> {
        for (id, node) in &self.nodes {
            node.local_effects.validate().map_err(|source| {
                EffectInferenceError::InvalidEffectRow {
                    node: *id,
                    source,
                }
            })?;

            for dependency in &node.dependencies {
                if !self.nodes.contains_key(&dependency.target) {
                    return Err(EffectInferenceError::MissingNode {
                        node: dependency.target,
                    });
                }
            }
        }

        Ok(())
    }
}

/// Final effect-inference results indexed by canonical node identity.
///
/// A successful result means the graph's effect rows have been propagated to
/// a fixed point. It does not mean the program is type-correct, authorized,
/// executable on a particular target, or free of unresolved obligations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectInferenceResult {
    effects: BTreeMap<NodeId, EffectRow>,
}

impl EffectInferenceResult {
    /// Returns the number of nodes with inferred rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Returns whether the result contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Returns an inferred effect row, if the node was analyzed.
    #[must_use]
    pub fn get(&self, id: NodeId) -> Option<&EffectRow> {
        self.effects.get(&id)
    }

    /// Iterates over inferred rows in stable node-identity order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&NodeId, &EffectRow)> {
        self.effects.iter()
    }

    /// Consumes the result and returns its ordered map.
    #[must_use]
    pub fn into_rows(self) -> BTreeMap<NodeId, EffectRow> {
        self.effects
    }
}

/// Errors produced while constructing or inferring an effect graph.
#[derive(Debug)]
#[non_exhaustive]
pub enum EffectInferenceError {
    /// A node identity was registered more than once.
    DuplicateNode {
        /// The duplicate identity.
        node: NodeId,
    },

    /// A dependency source was not registered when the edge was added.
    MissingSourceNode {
        /// The absent source identity.
        source: NodeId,
        /// The intended dependency target.
        target: NodeId,
    },

    /// A node referenced by the graph is absent.
    MissingNode {
        /// The missing identity.
        node: NodeId,
    },

    /// A local or dependency effect row is structurally invalid.
    InvalidEffectRow {
        /// Node whose row failed validation.
        node: NodeId,
        /// Underlying effect-model validation error.
        source: EffectModelError,
    },
}

impl fmt::Display for EffectInferenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode { node } => {
                write!(formatter, "duplicate effect-inference node {node}")
            }
            Self::MissingSourceNode { source, target } => write!(
                formatter,
                "cannot add effect dependency {source} -> {target}: \
                 source node is not registered"
            ),
            Self::MissingNode { node } => {
                write!(
                    formatter,
                    "effect-inference graph references missing node {node}"
                )
            }
            Self::InvalidEffectRow { node, source } => write!(
                formatter,
                "invalid local effect row for node {node}: {source}"
            ),
        }
    }
}

impl Error for EffectInferenceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidEffectRow { source, .. } => Some(source),
            Self::DuplicateNode { .. }
            | Self::MissingSourceNode { .. }
            | Self::MissingNode { .. } => None,
        }
    }
}

/// Infers effect rows to a fixed point.
///
/// Each node starts with its local effect row. For every propagating edge
/// `source -> target`, the target's inferred row is unioned into the source's
/// row. When a row changes, its dependents are revisited.
///
/// Cycles are supported without a recursion stack or an arbitrary iteration
/// limit. Since the transfer function only adds members from the finite set of
/// initial rows, successful inference terminates when no further additions
/// are possible.
///
/// Latent edges are validated but do not propagate effects into their source.
///
/// # Errors
///
/// Returns a structural graph error if a dependency target is missing or a
/// local effect row is malformed.
///
/// # Example
///
/// ```ignore
/// let mut graph = EffectInferenceGraph::new();
/// graph.add_node(function_id, EffectRow::empty())?;
/// graph.add_node(call_id, local_effects)?;
/// graph.add_dependency(
///     call_id,
///     function_id,
///     EffectPropagation::Propagates,
/// )?;
///
/// let result = infer_effects(&graph)?;
/// let inferred = result.get(call_id);
/// ```
pub fn infer_effects(
    graph: &EffectInferenceGraph,
) -> Result<EffectInferenceResult, EffectInferenceError> {
    graph.validate()?;

    let mut inferred: BTreeMap<NodeId, EffectRow> = graph
        .nodes
        .iter()
        .map(|(id, node)| (*id, node.local_effects.clone()))
        .collect();

    // reverse[target] lists source nodes whose rows depend on target's row.
    // Latent edges deliberately do not enter this map.
    let mut reverse: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();

    for (source, node) in &graph.nodes {
        for dependency in &node.dependencies {
            if dependency.propagation == EffectPropagation::Propagates {
                reverse
                    .entry(dependency.target)
                    .or_default()
                    .insert(*source);
            }
        }
    }

    // Initially process every node so its local effects reach all dependents.
    let mut worklist: VecDeque<NodeId> = inferred.keys().copied().collect();
    let mut queued: BTreeSet<NodeId> = worklist.iter().copied().collect();

    while let Some(changed_target) = worklist.pop_front() {
        queued.remove(&changed_target);

        let target_effects = inferred
            .get(&changed_target)
            .cloned()
            .ok_or(EffectInferenceError::MissingNode {
                node: changed_target,
            })?;

        let dependents = match reverse.get(&changed_target) {
            Some(nodes) => nodes.iter().copied().collect::<Vec<_>>(),
            None => continue,
        };

        for source in dependents {
            let source_effects = inferred
                .get(&source)
                .ok_or(EffectInferenceError::MissingNode { node: source })?;

            let combined = source_effects.union(&target_effects);

            if &combined != source_effects {
                inferred.insert(source, combined);

                if queued.insert(source) {
                    worklist.push_back(source);
                }
            }
        }
    }

    Ok(EffectInferenceResult { effects: inferred })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::effects::effect_kind::EffectKind;
    use crate::frontend::semantic::effects::model::EffectInstance;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn effect(namespace: &str, name: &str) -> EffectRow {
        let kind = EffectKind::new(namespace, name)
            .expect("test effect identity must be valid");

        EffectRow::closed([EffectInstance::simple(kind)])
            .expect("test effect row must be valid")
    }

    fn add_node(
        graph: &mut EffectInferenceGraph,
        id: u64,
        row: EffectRow,
    ) {
        graph
            .add_node(node_id(id), row)
            .expect("test graph node must be valid");
    }

    #[test]
    fn empty_graph_produces_empty_result() {
        let graph = EffectInferenceGraph::new();
        let result = infer_effects(&graph).expect("empty graph is valid");

        assert!(result.is_empty());
    }

    #[test]
    fn local_effects_are_preserved() {
        let mut graph = EffectInferenceGraph::new();
        add_node(&mut graph, 1, effect("example", "read"));

        let result = infer_effects(&graph).expect("graph is valid");

        assert_eq!(result.get(node_id(1)), Some(&effect("example", "read")));
    }

    #[test]
    fn effects_propagate_from_callee_to_caller() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, EffectRow::empty());
        add_node(&mut graph, 2, effect("example", "write"));

        graph
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        let result = infer_effects(&graph).expect("graph is valid");

        assert_eq!(
            result.get(node_id(1)),
            Some(&effect("example", "write"))
        );
    }

    #[test]
    fn multiple_effects_are_combined_without_duplicates() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, effect("example", "read"));
        add_node(&mut graph, 2, effect("example", "write"));
        add_node(&mut graph, 3, effect("example", "read"));

        graph
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        graph
            .add_dependency(
                node_id(1),
                node_id(3),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        let result = infer_effects(&graph).expect("graph is valid");
        let row = result.get(node_id(1)).expect("result contains node");

        assert_eq!(row.effect_count(), 2);
    }

    #[test]
    fn recursive_calls_reach_a_fixed_point() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, effect("example", "first"));
        add_node(&mut graph, 2, EffectRow::empty());
        add_node(&mut graph, 3, effect("example", "third"));

        // 1 calls 2, 2 calls 3, and 3 calls 1.
        graph
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        graph
            .add_dependency(
                node_id(2),
                node_id(3),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        graph
            .add_dependency(
                node_id(3),
                node_id(1),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        let result = infer_effects(&graph).expect("recursive graph is valid");

        let first = result.get(node_id(1)).expect("node exists");
        let second = result.get(node_id(2)).expect("node exists");
        let third = result.get(node_id(3)).expect("node exists");

        assert_eq!(first.effect_count(), 2);
        assert_eq!(second, first);
        assert_eq!(third, first);
    }

    #[test]
    fn latent_effects_do_not_leak_into_the_container() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, EffectRow::empty());
        add_node(&mut graph, 2, effect("example", "io"));

        graph
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Latent,
            )
            .expect("source node exists");

        let result = infer_effects(&graph).expect("graph is valid");

        assert_eq!(
            result.get(node_id(1)),
            Some(&EffectRow::empty())
        );
        assert_eq!(
            result.get(node_id(2)),
            Some(&effect("example", "io"))
        );
    }

    #[test]
    fn missing_dependency_target_is_an_error() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, EffectRow::empty());

        graph
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        assert!(matches!(
            infer_effects(&graph),
            Err(EffectInferenceError::MissingNode { node })
                if node == node_id(2)
        ));
    }

    #[test]
    fn duplicate_node_is_rejected() {
        let mut graph = EffectInferenceGraph::new();

        add_node(&mut graph, 1, EffectRow::empty());

        assert!(matches!(
            graph.add_node(node_id(1), EffectRow::empty()),
            Err(EffectInferenceError::DuplicateNode { node })
                if node == node_id(1)
        ));
    }

    #[test]
    fn inference_is_independent_of_node_insertion_order() {
        let mut first = EffectInferenceGraph::new();
        add_node(&mut first, 1, EffectRow::empty());
        add_node(&mut first, 2, effect("example", "io"));
        first
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        let mut second = EffectInferenceGraph::new();
        add_node(&mut second, 2, effect("example", "io"));
        add_node(&mut second, 1, EffectRow::empty());
        second
            .add_dependency(
                node_id(1),
                node_id(2),
                EffectPropagation::Propagates,
            )
            .expect("source node exists");

        assert_eq!(
            infer_effects(&first).expect("first graph is valid"),
            infer_effects(&second).expect("second graph is valid")
        );
    }
}
