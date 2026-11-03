
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Pipeline Dependency Analysis
//!
//! Path: `src/ai/pipeline/dependencies.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Analyze the logical dependency relationships in an existing AI pipeline
//! graph without constructing another graph representation or performing
//! execution planning.
//!
//! The analysis provides:
//!
//! - direct predecessor and dependent relationships;
//! - dependency edge classifications;
//! - transitive dependency queries;
//! - deterministic topological ordering when the graph is acyclic;
//! - strongly connected components;
//! - explicit identification of cyclic components;
//! - explicit cycle-related errors when an acyclic order is required.
//!
//! ## Ownership
//!
//! This module owns dependency analysis derived from PipelineGraph.
//!
//! It does not own:
//!
//! - pipeline definitions;
//! - stage declarations;
//! - graph construction;
//! - graph edge storage;
//! - source AST construction;
//! - grammar interpretation;
//! - name resolution;
//! - semantic type checking;
//! - cycle legality policy;
//! - execution planning;
//! - scheduling;
//! - resource allocation;
//! - hardware selection;
//! - distributed placement;
//! - runtime execution;
//! - universal IR representations.
//!
//! ## Canonical dependencies
//!
//! `crate::ai::pipeline::graph`
//!     Owns PipelineGraph, PipelineEdge and PipelineEdgeKind.
//!
//! `crate::ai::pipeline::definition`
//!     Owns PipelineStageId.
//!
//! `crate::ai::ids`
//!     Owns AiPipelineId through the graph.
//!
//! No competing graph or identifier representation is introduced.
//!
//! ## Dependency semantics
//!
//! All three graph edge kinds impose logical ordering relationships:
//!
//! - Dependency: an explicit logical dependency.
//! - DataFlow: a produced value must be available to its consumer.
//! - Control: a control relationship must be respected.
//!
//! Their classifications remain distinguishable through this API.
//!
//! ## Cycles
//!
//! This module detects cycles but does not decide whether they are legal.
//!
//! Some computational models may permit feedback, iteration, recursion,
//! streaming feedback or explicitly bounded/unbounded iterative regions.
//!
//! The canonical semantic validation layer determines whether a particular
//! cycle is permitted by the language and its declared semantics.
//!
//! A topological order exists only when the analyzed graph is acyclic.
//!
//! ## Determinism
//!
//! Ordered maps and sets are used throughout.
//!
//! Equivalent graphs produce equivalent analysis results regardless of
//! their original edge insertion order.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on:
//!
//! - stages;
//! - edges;
//! - direct dependencies;
//! - transitive dependencies;
//! - graph depth;
//! - graph fan-in;
//! - graph fan-out;
//! - strongly connected components.
//!
//! The algorithms are iterative rather than recursive, avoiding dependence
//! on the native call-stack depth for graph traversal.
//!
//! Dynamic collection capacity, address space, available memory and
//! operational budgets remain implementation constraints.
//!
//! This module does not promise literal infinite storage or computation.
//!
//! ## POCO-REAF
//!
//! Dependency analysis operates exclusively on logical stage identities
//! and relationships.
//!
//! It does not introduce:
//!
//! - physical device identities;
//! - CPU, GPU, FPGA or QPU assignments;
//! - vendor-specific execution requirements;
//! - network addresses;
//! - worker identities;
//! - execution placement;
//! - scheduling decisions.
//!
//! The same dependency relationships can therefore be analyzed before
//! considering different compatible execution environments.
//!
//! ## Integration
//!
//! PipelineGraph
//!     |
//!     v
//! THIS MODULE
//!     |
//!     +--> direct dependency queries
//!     |
//!     +--> transitive dependency queries
//!     |
//!     +--> cycle and component analysis
//!     |
//!     +--> optional topological order
//!     |
//!     v
//! pipeline/validation.rs
//!     |
//!     v
//! semantic validation and abstract execution planning
//!     |
//!     v
//! compiler / distributed / runtime infrastructure
//!
//! This module never bypasses the canonical semantic authority.
//!
//! ## Freeze criteria
//!
//! - Consumes the existing PipelineGraph.
//! - Does not introduce another graph representation.
//! - Preserves all edge classifications.
//! - Accounts for all edge kinds in ordering analysis.
//! - Detects cycles and self-dependencies.
//! - Identifies strongly connected components.
//! - Produces deterministic topological order for acyclic graphs.
//! - Uses iterative graph traversal.
//! - Handles empty and disconnected graphs.
//! - Does not impose arbitrary graph-size ceilings.
//! - Contains no unsafe Rust.
//! - Tests cover structural and algorithmic invariants.

use crate::ai::pipeline::definition::PipelineStageId;

use crate::ai::pipeline::graph::{
    PipelineEdgeKind,
    PipelineGraph,
};

use crate::ai::ids::AiPipelineId;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

// ============================================================================
// DEPENDENCY ANALYSIS
// ============================================================================

/// Immutable dependency analysis for one AI pipeline.
///
/// The original PipelineGraph remains the canonical graph representation.
///
/// This structure contains derived indexes and analysis results only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyAnalysis {
    pipeline_id: AiPipelineId,

    /// Immediate predecessors of each stage.
    predecessors: BTreeMap<PipelineStageId, BTreeSet<PipelineStageId>>,

    /// Immediate dependents of each stage.
    dependents: BTreeMap<PipelineStageId, BTreeSet<PipelineStageId>>,

    /// Edge classifications for each directed stage relationship.
    ///
    /// Multiple edge kinds may exist between the same two stages.
    edge_kinds: BTreeMap<
        (PipelineStageId, PipelineStageId),
        BTreeSet<PipelineEdgeKind>,
    >,

    /// Deterministic topological order, present only for acyclic graphs.
    topological_order: Option<Vec<PipelineStageId>>,

    /// All strongly connected components.
    ///
    /// Components and their members are deterministically ordered.
    components: Vec<Vec<PipelineStageId>>,

    /// Components containing a cycle.
    ///
    /// A component is cyclic when it contains multiple stages or a
    /// self-dependency.
    cyclic_components: Vec<Vec<PipelineStageId>>,

    /// Whether the graph contains at least one self-dependency.
    has_self_dependency: bool,

    /// Number of distinct directed stage relationships.
    relationship_count: usize,
}

impl DependencyAnalysis {
    /// Analyze the dependencies of an existing pipeline graph.
    ///
    /// Every graph edge contributes an ordering relationship, regardless
    /// of whether it represents an explicit dependency, data flow or
    /// control dependency.
    ///
    /// This constructor does not reject cycles. Cycles are returned as
    /// analysis results so that the semantic validation layer can decide
    /// whether they are legal.
    #[must_use]
    pub fn analyze(graph: &PipelineGraph) -> Self {
        let mut predecessors = BTreeMap::new();
        let mut dependents = BTreeMap::new();

        for node in graph.nodes() {
            predecessors.insert(node.clone(), BTreeSet::new());
            dependents.insert(node.clone(), BTreeSet::new());
        }

        let mut edge_kinds: BTreeMap<
            (PipelineStageId, PipelineStageId),
            BTreeSet<PipelineEdgeKind>,
        > = BTreeMap::new();

        let mut has_self_dependency = false;

        for edge in graph.edges() {
            let source = edge.source().stage_id().clone();
            let target = edge.target().stage_id().clone();

            has_self_dependency |= source == target;

            predecessors
                .entry(target.clone())
                .or_default()
                .insert(source.clone());

            dependents
                .entry(source.clone())
                .or_default()
                .insert(target.clone());

            edge_kinds
                .entry((source, target))
                .or_default()
                .insert(edge.kind());
        }

        let relationship_count = edge_kinds.len();

        let components = strongly_connected_components(
            &predecessors,
            &dependents,
        );

        let cyclic_components: Vec<Vec<PipelineStageId>> = components
            .iter()
            .filter(|component| {
                component.len() > 1
                    || component.first().is_some_and(|stage| {
                        predecessors
                            .get(stage)
                            .is_some_and(|parents| parents.contains(stage))
                    })
            })
            .cloned()
            .collect();

        let topological_order = if cyclic_components.is_empty() {
            Some(compute_topological_order(
                &predecessors,
                &dependents,
            ))
        } else {
            None
        };

        Self {
            pipeline_id: graph.pipeline_id(),
            predecessors,
            dependents,
            edge_kinds,
            topological_order,
            components,
            cyclic_components,
            has_self_dependency,
            relationship_count,
        }
    }

    /// Return the owning pipeline identity.
    #[must_use]
    pub const fn pipeline_id(&self) -> AiPipelineId {
        self.pipeline_id
    }

    /// Return the number of analyzed stages.
    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.predecessors.len()
    }

    /// Return the number of distinct directed stage relationships.
    ///
    /// Different edge kinds or data-flow port connections between the
    /// same pair of stages count as one stage relationship here.
    ///
    /// The original graph remains authoritative for the exact edge count.
    #[must_use]
    pub const fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    /// Return whether the analysis contains no stages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.predecessors.is_empty()
    }

    /// Return whether the analyzed graph contains a cycle.
    #[must_use]
    pub fn has_cycles(&self) -> bool {
        !self.cyclic_components.is_empty()
    }

    /// Return whether the analyzed graph contains a self-dependency.
    #[must_use]
    pub const fn has_self_dependency(&self) -> bool {
        self.has_self_dependency
    }

    /// Return all strongly connected components.
    ///
    /// Every stage belongs to exactly one component.
    ///
    /// Component membership does not by itself imply that the component
    /// is semantically invalid.
    #[must_use]
    pub fn components(&self) -> &[Vec<PipelineStageId>] {
        &self.components
    }

    /// Return all cyclic strongly connected components.
    ///
    /// A component containing one stage is cyclic only when that stage
    /// has a self-dependency.
    #[must_use]
    pub fn cyclic_components(&self) -> &[Vec<PipelineStageId>] {
        &self.cyclic_components
    }

    /// Return the deterministic topological order, if one exists.
    ///
    /// `None` means that the graph contains at least one cycle.
    ///
    /// This order describes dependency precedence. It is not a runtime
    /// schedule and does not imply that stages execute sequentially.
    #[must_use]
    pub fn topological_order(&self) -> Option<&[PipelineStageId]> {
        self.topological_order.as_deref()
    }

    /// Require an acyclic dependency graph and return its topological order.
    ///
    /// This is appropriate for consumers that require a DAG.
    ///
    /// It must not be used to reject language constructs whose semantics
    /// explicitly permit cycles or feedback.
    pub fn require_acyclic(
        &self,
    ) -> Result<&[PipelineStageId], DependencyAnalysisError> {
        self.topological_order()
            .ok_or_else(|| DependencyAnalysisError::CyclicDependencies {
                components: self.cyclic_components.clone(),
            })
    }

    /// Return the immediate predecessors of a stage.
    ///
    /// Returns `None` when the stage does not belong to this pipeline.
    #[must_use]
    pub fn direct_dependencies(
        &self,
        stage: &PipelineStageId,
    ) -> Option<&BTreeSet<PipelineStageId>> {
        self.predecessors.get(stage)
    }

    /// Return the immediate dependents of a stage.
    ///
    /// Returns `None` when the stage does not belong to this pipeline.
    #[must_use]
    pub fn direct_dependents(
        &self,
        stage: &PipelineStageId,
    ) -> Option<&BTreeSet<PipelineStageId>> {
        self.dependents.get(stage)
    }

    /// Return the edge kinds connecting two stages.
    ///
    /// Multiple edge kinds can connect the same pair of stages.
    ///
    /// Returns `None` if no relationship exists between the stages.
    #[must_use]
    pub fn edge_kinds(
        &self,
        source: &PipelineStageId,
        target: &PipelineStageId,
    ) -> Option<&BTreeSet<PipelineEdgeKind>> {
        self.edge_kinds
            .get(&(source.clone(), target.clone()))
    }

    /// Return whether a stage is present in this analysis.
    #[must_use]
    pub fn contains_stage(&self, stage: &PipelineStageId) -> bool {
        self.predecessors.contains_key(stage)
    }

    /// Return every stage transitively required by the supplied stage.
    ///
    /// The result includes direct and indirect predecessors.
    ///
    /// The supplied stage itself is excluded, even when it participates
    /// in a cycle. Cyclic relationships are available separately through
    /// `cyclic_components`.
    ///
    /// Returns an error for an unknown stage.
    pub fn transitive_dependencies(
        &self,
        stage: &PipelineStageId,
    ) -> Result<BTreeSet<PipelineStageId>, DependencyAnalysisError> {
        self.reachable(
            stage,
            &self.predecessors,
        )
    }

    /// Return every stage transitively depending on the supplied stage.
    ///
    /// The result includes direct and indirect dependents.
    ///
    /// The supplied stage itself is excluded, even when it participates
    /// in a cycle.
    ///
    /// Returns an error for an unknown stage.
    pub fn transitive_dependents(
        &self,
        stage: &PipelineStageId,
    ) -> Result<BTreeSet<PipelineStageId>, DependencyAnalysisError> {
        self.reachable(
            stage,
            &self.dependents,
        )
    }

    /// Return all stages that have no direct dependencies.
    ///
    /// These are structural source stages, not necessarily valid
    /// semantic pipeline entry points.
    #[must_use]
    pub fn source_stages(&self) -> impl Iterator<Item = &PipelineStageId> {
        self.predecessors
            .iter()
            .filter(|(_, parents)| parents.is_empty())
            .map(|(stage, _)| stage)
    }

    /// Return all stages that have no direct dependents.
    ///
    /// These are structural sink stages, not necessarily valid
    /// semantic pipeline completion points.
    #[must_use]
    pub fn sink_stages(&self) -> impl Iterator<Item = &PipelineStageId> {
        self.dependents
            .iter()
            .filter(|(_, children)| children.is_empty())
            .map(|(stage, _)| stage)
    }

    /// Return the number of direct dependencies of a stage.
    ///
    /// Returns `None` for an unknown stage.
    #[must_use]
    pub fn direct_dependency_count(
        &self,
        stage: &PipelineStageId,
    ) -> Option<usize> {
        self.predecessors.get(stage).map(BTreeSet::len)
    }

    /// Return the number of direct dependents of a stage.
    ///
    /// Returns `None` for an unknown stage.
    #[must_use]
    pub fn direct_dependent_count(
        &self,
        stage: &PipelineStageId,
    ) -> Option<usize> {
        self.dependents.get(stage).map(BTreeSet::len)
    }

    /// Traverse a relationship index iteratively.
    ///
    /// Iterative traversal avoids recursion-depth dependence for large
    /// dependency graphs.
    fn reachable(
        &self,
        start: &PipelineStageId,
        relationships: &BTreeMap<
            PipelineStageId,
            BTreeSet<PipelineStageId>,
        >,
    ) -> Result<BTreeSet<PipelineStageId>, DependencyAnalysisError> {
        if !relationships.contains_key(start) {
            return Err(DependencyAnalysisError::UnknownStage {
                stage: start.clone(),
            });
        }

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();

        if let Some(initial) = relationships.get(start) {
            pending.extend(initial.iter().cloned());
        }

        while let Some(stage) = pending.pop_front() {
            if stage == *start || !visited.insert(stage.clone()) {
                continue;
            }

            if let Some(neighbors) = relationships.get(&stage) {
                pending.extend(neighbors.iter().cloned());
            }
        }

        Ok(visited)
    }
}

// ============================================================================
// TOPOLOGICAL ORDER
// ============================================================================

/// Compute a deterministic topological order.
///
/// This function is called only after strongly connected component
/// analysis establishes that the graph is acyclic.
///
/// The returned vector contains every graph stage exactly once.
fn compute_topological_order(
    predecessors: &BTreeMap<
        PipelineStageId,
        BTreeSet<PipelineStageId>,
    >,
    dependents: &BTreeMap<
        PipelineStageId,
        BTreeSet<PipelineStageId>,
    >,
) -> Vec<PipelineStageId> {
    let mut indegree: BTreeMap<PipelineStageId, usize> = predecessors
        .iter()
        .map(|(stage, parents)| (stage.clone(), parents.len()))
        .collect();

    let mut ready: BTreeSet<PipelineStageId> = indegree
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(stage, _)| stage.clone())
        .collect();

    let mut order = Vec::with_capacity(indegree.len());

    while let Some(stage) = ready.pop_first() {
        order.push(stage.clone());

        if let Some(children) = dependents.get(&stage) {
            for child in children {
                if let Some(count) = indegree.get_mut(child) {
                    *count -= 1;

                    if *count == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }
    }

    order
}

// ============================================================================
// STRONGLY CONNECTED COMPONENTS
// ============================================================================

/// Compute strongly connected components using iterative Kosaraju traversal.
///
/// The first traversal uses outgoing relationships to establish finishing
/// order. The second traversal uses incoming relationships to collect
/// components.
///
/// Both passes are iterative, avoiding recursive call-stack growth.
fn strongly_connected_components(
    predecessors: &BTreeMap<
        PipelineStageId,
        BTreeSet<PipelineStageId>,
    >,
    dependents: &BTreeMap<
        PipelineStageId,
        BTreeSet<PipelineStageId>,
    >,
) -> Vec<Vec<PipelineStageId>> {
    let mut visited = BTreeSet::new();
    let mut finish_order = Vec::with_capacity(predecessors.len());

    // First pass: compute finishing order over outgoing edges.
    for root in dependents.keys() {
        if visited.contains(root) {
            continue;
        }

        let mut stack = vec![(root.clone(), false)];

        while let Some((stage, expanded)) = stack.pop() {
            if expanded {
                finish_order.push(stage);
                continue;
            }

            if !visited.insert(stage.clone()) {
                continue;
            }

            stack.push((stage.clone(), true));

            if let Some(children) = dependents.get(&stage) {
                for child in children.iter().rev() {
                    if !visited.contains(child) {
                        stack.push((child.clone(), false));
                    }
                }
            }
        }
    }

    // Second pass: collect components over incoming edges.
    visited.clear();

    let mut components = Vec::new();

    for root in finish_order.into_iter().rev() {
        if !visited.insert(root.clone()) {
            continue;
        }

        let mut component = BTreeSet::new();
        let mut stack = vec![root];

        while let Some(stage) = stack.pop() {
            component.insert(stage.clone());

            if let Some(parents) = predecessors.get(&stage) {
                for parent in parents.iter().rev() {
                    if visited.insert(parent.clone()) {
                        stack.push(parent.clone());
                    }
                }
            }
        }

        components.push(component.into_iter().collect());
    }

    // Canonical component ordering makes results independent of traversal
    // implementation details and graph insertion order.
    components.sort();

    components
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by dependency analysis queries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyAnalysisError {
    /// A query referenced a stage outside the analyzed pipeline.
    UnknownStage {
        /// The unknown stage identity.
        stage: PipelineStageId,
    },

    /// An acyclic ordering was requested for a graph containing cycles.
    CyclicDependencies {
        /// The strongly connected components containing cycles.
        components: Vec<Vec<PipelineStageId>>,
    },
}

impl fmt::Display for DependencyAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownStage { stage } => {
                write!(
                    f,
                    "dependency analysis references unknown pipeline stage '{stage}'"
                )
            }

            Self::CyclicDependencies { components } => {
                write!(
                    f,
                    "an acyclic dependency order cannot be produced: \
                     {} cyclic component(s) detected",
                    components.len()
                )
            }
        }
    }
}

impl Error for DependencyAnalysisError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::pipeline::definition::{
        PipelineDefinitionBuilder,
        PipelineOperationRef,
        PipelinePort,
        PipelinePortDirection,
        PipelineStageDefinition,
    };

    use crate::ai::pipeline::graph::PipelineGraph;

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value).expect("valid stage identity")
    }

    fn stage(value: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(value),
            PipelineOperationRef::new("zamani.ai", "transform")
                .expect("valid operation"),
        )
    }

    fn definition(
        stages: Vec<PipelineStageDefinition>,
    ) -> crate::ai::pipeline::definition::PipelineDefinition {
        let mut builder = PipelineDefinitionBuilder::new(
            AiPipelineId::new(1).expect("nonzero pipeline ID"),
            "dependency-test",
        )
        .expect("valid pipeline");

        for stage in stages {
            builder.add_stage(stage).expect("unique stage");
        }

        builder.build()
    }

    fn graph(
        stages: &[&str],
        edges: &[(&str, &str)],
    ) -> PipelineGraph {
        let definition = definition(
            stages.iter().map(|name| stage(name)).collect(),
        );

        let mut builder = PipelineGraph::builder(&definition);

        for (source, target) in edges {
            builder
                .add_dependency(&stage_id(source), &stage_id(target))
                .expect("valid dependency");
        }

        builder.build()
    }

    fn names(stages: &[PipelineStageId]) -> Vec<&str> {
        stages.iter().map(PipelineStageId::as_str).collect()
    }

    #[test]
    fn empty_graph_has_empty_acyclic_order() {
        let graph = graph(&[], &[]);
        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(analysis.is_empty());
        assert!(!analysis.has_cycles());
        assert_eq!(analysis.stage_count(), 0);
        assert_eq!(analysis.relationship_count(), 0);
        assert_eq!(analysis.topological_order(), Some([].as_slice()));
        assert!(analysis.components().is_empty());
        assert!(analysis.cyclic_components().is_empty());
    }

    #[test]
    fn independent_stages_are_all_source_and_sink_stages() {
        let graph = graph(&["alpha", "beta", "gamma"], &[]);
        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(
            analysis.source_stages().map(PipelineStageId::as_str).collect::<Vec<_>>(),
            ["alpha", "beta", "gamma"],
        );

        assert_eq!(
            analysis.sink_stages().map(PipelineStageId::as_str).collect::<Vec<_>>(),
            ["alpha", "beta", "gamma"],
        );

        assert_eq!(
            analysis.topological_order().map(names),
            Some(vec!["alpha", "beta", "gamma"]),
        );
    }

    #[test]
    fn direct_dependencies_and_dependents_are_correct() {
        let graph = graph(
            &["source", "middle", "sink"],
            &[("source", "middle"), ("middle", "sink")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(
            names(
                &analysis
                    .direct_dependencies(&stage_id("middle"))
                    .expect("known stage")
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
            ),
            ["source"],
        );

        assert_eq!(
            names(
                &analysis
                    .direct_dependents(&stage_id("middle"))
                    .expect("known stage")
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
            ),
            ["sink"],
        );
    }

    #[test]
    fn transitive_dependencies_are_computed() {
        let graph = graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("b", "c"), ("c", "d")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        let dependencies = analysis
            .transitive_dependencies(&stage_id("d"))
            .expect("known stage");

        assert_eq!(
            dependencies.iter().map(PipelineStageId::as_str).collect::<Vec<_>>(),
            ["a", "b", "c"],
        );
    }

    #[test]
    fn transitive_dependents_are_computed() {
        let graph = graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("b", "c"), ("c", "d")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        let dependents = analysis
            .transitive_dependents(&stage_id("a"))
            .expect("known stage");

        assert_eq!(
            dependents.iter().map(PipelineStageId::as_str).collect::<Vec<_>>(),
            ["b", "c", "d"],
        );
    }

    #[test]
    fn transitive_queries_reject_unknown_stages() {
        let graph = graph(&["known"], &[]);
        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(matches!(
            analysis.transitive_dependencies(&stage_id("missing")),
            Err(DependencyAnalysisError::UnknownStage { .. })
        ));

        assert!(matches!(
            analysis.transitive_dependents(&stage_id("missing")),
            Err(DependencyAnalysisError::UnknownStage { .. })
        ));
    }

    #[test]
    fn topological_order_respects_dependency_direction() {
        let graph = graph(
            &["z", "a", "m"],
            &[("a", "m"), ("m", "z")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(
            analysis.topological_order().map(names),
            Some(vec!["a", "m", "z"]),
        );
    }

    #[test]
    fn topological_order_is_deterministic_for_independent_stages() {
        let first = graph(
            &["z", "a", "m"],
            &[],
        );

        let second = graph(
            &["m", "z", "a"],
            &[],
        );

        let left = DependencyAnalysis::analyze(&first);
        let right = DependencyAnalysis::analyze(&second);

        assert_eq!(left.topological_order(), right.topological_order());
        assert_eq!(left.components(), right.components());
    }

    #[test]
    fn detects_multi_stage_cycles() {
        let graph = graph(
            &["a", "b", "c"],
            &[("a", "b"), ("b", "c"), ("c", "a")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(analysis.has_cycles());
        assert_eq!(
            analysis.cyclic_components().len(),
            1,
        );

        assert_eq!(
            analysis.cyclic_components()[0]
                .iter()
                .map(PipelineStageId::as_str)
                .collect::<Vec<_>>(),
            ["a", "b", "c"],
        );

        assert!(analysis.topological_order().is_none());
    }

    #[test]
    fn detects_self_dependencies() {
        let graph = graph(&["loop"], &[("loop", "loop")]);
        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(analysis.has_cycles());
        assert!(analysis.has_self_dependency());

        assert_eq!(
            analysis.cyclic_components()[0]
                .iter()
                .map(PipelineStageId::as_str)
                .collect::<Vec<_>>(),
            ["loop"],
        );
    }

    #[test]
    fn identifies_multiple_independent_cyclic_components() {
        let graph = graph(
            &["a", "b", "c", "d", "e"],
            &[
                ("a", "b"),
                ("b", "a"),
                ("c", "d"),
                ("d", "c"),
            ],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(analysis.cyclic_components().len(), 2);
        assert!(analysis.has_cycles());
    }

    #[test]
    fn distinguishes_cycle_members_from_downstream_stages() {
        let graph = graph(
            &["a", "b", "c"],
            &[("a", "b"), ("b", "a"), ("b", "c")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(analysis.cyclic_components().len(), 1);

        assert_eq!(
            analysis.cyclic_components()[0]
                .iter()
                .map(PipelineStageId::as_str)
                .collect::<Vec<_>>(),
            ["a", "b"],
        );
    }

    #[test]
    fn acyclic_order_request_reports_cycle_components() {
        let graph = graph(
            &["a", "b"],
            &[("a", "b"), ("b", "a")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(matches!(
            analysis.require_acyclic(),
            Err(DependencyAnalysisError::CyclicDependencies {
                components
            }) if components.len() == 1
        ));
    }

    #[test]
    fn data_flow_edges_are_ordering_dependencies() {
        let mut producer = stage("producer");
        producer
            .add_port(
                PipelinePort::new(
                    "result",
                    PipelinePortDirection::Output,
                    None,
                )
                .expect("valid output"),
            )
            .expect("unique port");

        let mut consumer = stage("consumer");
        consumer
            .add_port(
                PipelinePort::new(
                    "input",
                    PipelinePortDirection::Input,
                    None,
                )
                .expect("valid input"),
            )
            .expect("unique port");

        let definition = definition(vec![producer, consumer]);
        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_data_flow(
                &stage_id("producer"),
                "result",
                &stage_id("consumer"),
                "input",
            )
            .expect("valid data flow");

        let graph = builder.build();
        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(
            names(
                &analysis
                    .direct_dependencies(&stage_id("consumer"))
                    .expect("consumer")
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
            ),
            ["producer"],
        );

        assert_eq!(
            analysis.edge_kinds(
                &stage_id("producer"),
                &stage_id("consumer"),
            ),
            Some(&BTreeSet::from([PipelineEdgeKind::DataFlow])),
        );
    }

    #[test]
    fn edge_kinds_are_preserved_between_the_same_stages() {
        let graph_definition = definition(vec![
            stage("first"),
            stage("second"),
        ]);

        let mut builder = PipelineGraph::builder(&graph_definition);

        builder
            .add_dependency(&stage_id("first"), &stage_id("second"))
            .expect("dependency");

        builder
            .add_control_dependency(&stage_id("first"), &stage_id("second"))
            .expect("control dependency");

        let graph = builder.build();
        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(analysis.relationship_count(), 1);

        assert_eq!(
            analysis.edge_kinds(
                &stage_id("first"),
                &stage_id("second"),
            ),
            Some(&BTreeSet::from([
                PipelineEdgeKind::Dependency,
                PipelineEdgeKind::Control,
            ])),
        );
    }

    #[test]
    fn cycles_are_not_automatically_declared_semantically_invalid() {
        let graph = graph(
            &["iteration", "feedback"],
            &[("iteration", "feedback"), ("feedback", "iteration")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        // The analysis reports the cycle. Semantic validation owns the
        // decision about whether this feedback relationship is legal.
        assert!(analysis.has_cycles());
        assert_eq!(analysis.cyclic_components().len(), 1);
    }

    #[test]
    fn every_stage_belongs_to_exactly_one_component() {
        let graph = graph(
            &["a", "b", "c", "d"],
            &[("a", "b"), ("b", "a"), ("c", "d")],
        );

        let analysis = DependencyAnalysis::analyze(&graph);

        let members: BTreeSet<_> = analysis
            .components()
            .iter()
            .flat_map(|component| component.iter().cloned())
            .collect();

        assert_eq!(members.len(), analysis.stage_count());
        assert_eq!(members.len(), 4);
    }
}
