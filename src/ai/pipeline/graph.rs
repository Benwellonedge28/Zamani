
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Pipeline Graph
//!
//! Path: `src/ai/pipeline/graph.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the target-independent graph representation for AI pipeline
//! stages and their logical relationships.
//!
//! A graph contains:
//!
//! - stage identities declared by a PipelineDefinition;
//! - directed logical dependency edges;
//! - directed data-flow edges connecting stage output and input ports;
//! - directed control-dependency edges.
//!
//! The graph describes computational relationships. It does not execute
//! stages or determine where they execute.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - pipeline graph nodes;
//! - pipeline graph edges;
//! - graph construction from a PipelineDefinition;
//! - endpoint and port existence validation;
//! - edge uniqueness;
//! - deterministic graph storage;
//! - graph relationship queries;
//! - immutable graph access after construction.
//!
//! It does not own:
//!
//! - pipeline stage definitions;
//! - source AST construction;
//! - grammar interpretation;
//! - name resolution;
//! - semantic type checking;
//! - cycle legality;
//! - pipeline dependency policy;
//! - pipeline transformations;
//! - execution plans;
//! - scheduling;
//! - hardware placement;
//! - distributed coordination;
//! - runtime execution;
//! - universal IR representations.
//!
//! ## Canonical dependencies
//!
//! `crate::ai::pipeline::definition` owns:
//!
//! - PipelineDefinition;
//! - PipelineStageDefinition;
//! - PipelineStageId;
//! - PipelinePort;
//! - PipelinePortDirection;
//! - PipelineDefinitionVersion;
//! - PipelineOperationRef.
//!
//! `crate::ai::ids` owns AiPipelineId.
//!
//! No competing pipeline or stage identifier is introduced.
//!
//! ## Graph semantics
//!
//! The graph is directed.
//!
//! An edge always describes a relationship from a source stage to a target
//! stage.
//!
//! Edge categories:
//!
//! - Dependency: a logical dependency between stages.
//! - DataFlow: a value flows from a declared output port to a declared
//!   input port.
//! - Control: one stage has a logical control dependency on another.
//!
//! Multiple edges between the same two stages are permitted when their
//! edge identities differ. Exact duplicate edges are rejected.
//!
//! Self-edges and cycles are structurally representable. Their legality is
//! decided by pipeline semantic validation, not by this graph container.
//!
//! ## Determinism
//!
//! Nodes and edges are stored in ordered collections.
//!
//! Node iteration follows PipelineStageId ordering.
//! Edge iteration follows PipelineEdge ordering.
//!
//! This ordering is a deterministic representation order. It does not
//! establish execution order.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - stages;
//! - edges;
//! - dependencies;
//! - inputs;
//! - outputs;
//! - graph fan-in;
//! - graph fan-out;
//! - graph depth;
//! - pipeline participants.
//!
//! Storage uses dynamically sized collections. Actual capacity is bounded
//! by the process address space, allocator, available memory, and any
//! explicitly configured operational budget.
//!
//! Allocation failure must not be interpreted as a valid partial graph.
//!
//! ## POCO-REAF
//!
//! Graph nodes and edges contain no:
//!
//! - physical device identifiers;
//! - processor identifiers;
//! - GPU identifiers;
//! - QPU identifiers;
//! - hardware addresses;
//! - worker assignments;
//! - physical network routes;
//! - vendor-specific execution handles.
//!
//! The same logical graph can be considered by different compiler,
//! optimization, placement, and execution implementations.
//!
//! ## Integration
//!
//! `definition.rs`
//!     Supplies the canonical pipeline and stage declarations.
//!
//! `dependencies.rs`
//!     Performs dependency-specific analysis over this graph.
//!
//! `validation.rs`
//!     Determines graph and pipeline semantic legality, including whether
//!     particular cycles are permitted.
//!
//! `transformations.rs`
//!     Produces transformed logical graphs while preserving the applicable
//!     semantic contracts.
//!
//! `execution_plan.rs`
//!     Consumes validated graph information to create abstract plans.
//!
//! `grammar/ai/pipelines.g4`
//!     Defines source-level pipeline declarations and logical edge syntax.
//!     The AST/semantic adapter resolves source references to stage IDs.
//!
//! `frontend/semantic/domains/ai.rs`
//!     Remains the AI semantic authority.
//!
//! `zuir/dialects/ai` and the canonical tensor dialect
//!     Represent validated computation in universal IR.
//!
//! `distributed`
//!     Owns placement, distributed scheduling, and coordination.
//!
//! `runtime`
//!     Owns execution.
//!
//! ## Freeze criteria
//!
//! - Uses canonical pipeline and stage identities.
//! - Graph construction rejects undeclared endpoints.
//! - Data-flow edges validate port existence and direction.
//! - Exact duplicate edges are rejected.
//! - Multiple distinct edges between stages are supported.
//! - Cycles remain available for semantic validation.
//! - Ordered storage gives deterministic representation.
//! - No execution or hardware policy is introduced.
//! - No arbitrary graph-size ceiling is introduced.
//! - All graph invariants have tests.
//! - Unsafe Rust is forbidden.

use crate::ai::pipeline::definition::{
    PipelineDefinition,
    PipelinePortDirection,
    PipelineStageDefinition,
    PipelineStageId,
};

use crate::ai::ids::AiPipelineId;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// ============================================================================
// EDGE ENDPOINTS
// ============================================================================

/// A named endpoint on a pipeline stage.
///
/// An endpoint identifies a stage and, optionally, one of its declared
/// ports. Stage-only endpoints are used for logical dependencies and
/// control relationships.
///
/// This is not a source AST reference or a runtime value identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineEndpoint {
    stage: PipelineStageId,
    port: Option<String>,
}

impl PipelineEndpoint {
    /// Construct a stage-only endpoint.
    pub fn stage(stage: PipelineStageId) -> Self {
        Self { stage, port: None }
    }

    /// Construct an endpoint referring to a named stage port.
    pub fn port(
        stage: PipelineStageId,
        port: impl Into<String>,
    ) -> Result<Self, PipelineGraphError> {
        let port = port.into();

        if port.is_empty() || port.chars().any(char::is_control) {
            return Err(PipelineGraphError::InvalidPortName { port });
        }

        Ok(Self {
            stage,
            port: Some(port),
        })
    }

    /// Return the stage identity.
    pub fn stage_id(&self) -> &PipelineStageId {
        &self.stage
    }

    /// Return the optional port name.
    pub fn port_name(&self) -> Option<&str> {
        self.port.as_deref()
    }
}

// ============================================================================
// EDGE KIND
// ============================================================================

/// The logical meaning of a pipeline graph edge.
///
/// These variants describe relationships, not physical execution mechanisms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipelineEdgeKind {
    /// The target stage logically depends on the source stage.
    Dependency,

    /// A value flows from a source output port to a target input port.
    DataFlow,

    /// The target stage has a logical control dependency on the source.
    Control,
}

// ============================================================================
// EDGE
// ============================================================================

/// A directed relationship between two pipeline stages.
///
/// `source` always precedes `target` in the logical relationship.
///
/// Data-flow edges contain both source and target port names.
/// Dependency and control edges contain stage-only endpoints.
///
/// The type is immutable after construction.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineEdge {
    kind: PipelineEdgeKind,
    source: PipelineEndpoint,
    target: PipelineEndpoint,
}

impl PipelineEdge {
    /// Return the edge kind.
    pub const fn kind(&self) -> PipelineEdgeKind {
        self.kind
    }

    /// Return the source endpoint.
    pub fn source(&self) -> &PipelineEndpoint {
        &self.source
    }

    /// Return the target endpoint.
    pub fn target(&self) -> &PipelineEndpoint {
        &self.target
    }
}

// ============================================================================
// GRAPH
// ============================================================================

/// An immutable logical graph for one AI pipeline.
///
/// The graph contains only declared stage identities and logical edges.
/// Stage definitions remain owned by PipelineDefinition.
///
/// The graph is not an execution plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineGraph {
    pipeline_id: AiPipelineId,
    nodes: BTreeSet<PipelineStageId>,
    edges: BTreeSet<PipelineEdge>,

    // Derived adjacency indexes.
    //
    // These indexes accelerate relationship queries. They are derived
    // exclusively from `edges` during graph construction and are not
    // independent sources of truth.
    outgoing: BTreeMap<PipelineStageId, BTreeSet<PipelineStageId>>,
    incoming: BTreeMap<PipelineStageId, BTreeSet<PipelineStageId>>,
}

impl PipelineGraph {
    /// Return the identity of the owning pipeline.
    pub const fn pipeline_id(&self) -> AiPipelineId {
        self.pipeline_id
    }

    /// Return all graph nodes in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = &PipelineStageId> {
        self.nodes.iter()
    }

    /// Return all graph edges in deterministic order.
    pub fn edges(&self) -> impl Iterator<Item = &PipelineEdge> {
        self.edges.iter()
    }

    /// Return the number of graph nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of graph edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return whether the graph has no nodes.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return whether the graph has no edges.
    pub fn has_no_edges(&self) -> bool {
        self.edges.is_empty()
    }

    /// Return whether a stage is represented in this graph.
    pub fn contains_node(&self, stage: &PipelineStageId) -> bool {
        self.nodes.contains(stage)
    }

    /// Return whether an exact edge exists.
    pub fn contains_edge(&self, edge: &PipelineEdge) -> bool {
        self.edges.contains(edge)
    }

    /// Return the distinct successor stages of a stage.
    ///
    /// The result is deterministic and contains no duplicate stage IDs,
    /// even when multiple distinct edges connect the same pair of stages.
    pub fn successors(
        &self,
        stage: &PipelineStageId,
    ) -> impl Iterator<Item = &PipelineStageId> {
        self.outgoing
            .get(stage)
            .into_iter()
            .flat_map(|successors| successors.iter())
    }

    /// Return the distinct predecessor stages of a stage.
    ///
    /// The result is deterministic and contains no duplicate stage IDs,
    /// even when multiple distinct edges connect the same pair of stages.
    pub fn predecessors(
        &self,
        stage: &PipelineStageId,
    ) -> impl Iterator<Item = &PipelineStageId> {
        self.incoming
            .get(stage)
            .into_iter()
            .flat_map(|predecessors| predecessors.iter())
    }

    /// Return all edges leaving a stage.
    ///
    /// This scans the ordered edge collection. It preserves all distinct
    /// edge kinds and port relationships.
    pub fn outgoing_edges(
        &self,
        stage: &PipelineStageId,
    ) -> impl Iterator<Item = &PipelineEdge> {
        self.edges
            .iter()
            .filter(move |edge| edge.source.stage_id() == stage)
    }

    /// Return all edges entering a stage.
    pub fn incoming_edges(
        &self,
        stage: &PipelineStageId,
    ) -> impl Iterator<Item = &PipelineEdge> {
        self.edges
            .iter()
            .filter(move |edge| edge.target.stage_id() == stage)
    }

    /// Return all edges connecting a source stage to a target stage.
    ///
    /// Multiple edges may be returned when they represent different
    /// relationships or different port connections.
    pub fn edges_between<'a>(
        &'a self,
        source: &'a PipelineStageId,
        target: &'a PipelineStageId,
    ) -> impl Iterator<Item = &'a PipelineEdge> + 'a {
        self.edges.iter().filter(move |edge| {
            edge.source.stage_id() == source
                && edge.target.stage_id() == target
        })
    }

    /// Return the number of distinct successors of a stage.
    pub fn successor_count(&self, stage: &PipelineStageId) -> usize {
        self.outgoing.get(stage).map_or(0, BTreeSet::len)
    }

    /// Return the number of distinct predecessors of a stage.
    pub fn predecessor_count(&self, stage: &PipelineStageId) -> usize {
        self.incoming.get(stage).map_or(0, BTreeSet::len)
    }

    /// Return all graph nodes that have no incoming edges.
    ///
    /// These nodes have no declared predecessor in this graph. This does not
    /// prove that they are valid pipeline entry points.
    pub fn source_nodes(&self) -> impl Iterator<Item = &PipelineStageId> {
        self.nodes.iter().filter(|node| {
            self.incoming
                .get(*node)
                .is_none_or(BTreeSet::is_empty)
        })
    }

    /// Return all graph nodes that have no outgoing edges.
    ///
    /// These nodes have no declared successor in this graph. This does not
    /// prove that they are valid pipeline completion points.
    pub fn sink_nodes(&self) -> impl Iterator<Item = &PipelineStageId> {
        self.nodes.iter().filter(|node| {
            self.outgoing
                .get(*node)
                .is_none_or(BTreeSet::is_empty)
        })
    }

    /// Return whether an exact self-edge exists.
    ///
    /// This is a structural observation, not a semantic legality decision.
    pub fn has_self_edge(&self) -> bool {
        self.edges.iter().any(|edge| {
            edge.source.stage_id() == edge.target.stage_id()
        })
    }

    /// Return a graph builder initialized from a pipeline definition.
    ///
    /// Every stage in the definition becomes a graph node. No edges are
    /// inferred from declaration order.
    pub fn builder(
        definition: &PipelineDefinition,
    ) -> PipelineGraphBuilder<'_> {
        PipelineGraphBuilder::new(definition)
    }
}

// ============================================================================
// GRAPH BUILDER
// ============================================================================

/// Builder for an immutable PipelineGraph.
///
/// The builder borrows the canonical definition while constructing edges.
/// This ensures that stage and port references are checked against the
/// exact definition from which the graph is being built.
#[derive(Debug)]
pub struct PipelineGraphBuilder<'a> {
    definition: &'a PipelineDefinition,
    nodes: BTreeSet<PipelineStageId>,
    edges: BTreeSet<PipelineEdge>,
}

impl<'a> PipelineGraphBuilder<'a> {
    /// Construct a graph builder from a pipeline definition.
    ///
    /// All declared stages become graph nodes.
    pub fn new(definition: &'a PipelineDefinition) -> Self {
        let nodes = definition
            .stages()
            .iter()
            .map(|stage| stage.id().clone())
            .collect();

        Self {
            definition,
            nodes,
            edges: BTreeSet::new(),
        }
    }

    /// Add a logical dependency from one stage to another.
    ///
    /// The source and target must both be declared stages.
    ///
    /// Self-dependencies are structurally accepted. The semantic validation
    /// subsystem determines whether they are permitted.
    pub fn add_dependency(
        &mut self,
        source: &PipelineStageId,
        target: &PipelineStageId,
    ) -> Result<(), PipelineGraphError> {
        self.add_edge(PipelineEdge {
            kind: PipelineEdgeKind::Dependency,
            source: PipelineEndpoint::stage(source.clone()),
            target: PipelineEndpoint::stage(target.clone()),
        })
    }

    /// Add a control dependency from one stage to another.
    ///
    /// A control dependency does not describe a tensor or data value.
    pub fn add_control_dependency(
        &mut self,
        source: &PipelineStageId,
        target: &PipelineStageId,
    ) -> Result<(), PipelineGraphError> {
        self.add_edge(PipelineEdge {
            kind: PipelineEdgeKind::Control,
            source: PipelineEndpoint::stage(source.clone()),
            target: PipelineEndpoint::stage(target.clone()),
        })
    }

    /// Add a data-flow relationship between declared ports.
    ///
    /// The source port must be an Output port.
    /// The target port must be an Input port.
    ///
    /// Type and contract compatibility are deliberately not decided here.
    /// Those checks belong to canonical semantic validation.
    pub fn add_data_flow(
        &mut self,
        source_stage: &PipelineStageId,
        source_port: impl Into<String>,
        target_stage: &PipelineStageId,
        target_port: impl Into<String>,
    ) -> Result<(), PipelineGraphError> {
        let source_port = source_port.into();
        let target_port = target_port.into();

        self.validate_port(
            source_stage,
            &source_port,
            PipelinePortDirection::Output,
        )?;

        self.validate_port(
            target_stage,
            &target_port,
            PipelinePortDirection::Input,
        )?;

        let source = PipelineEndpoint::port(
            source_stage.clone(),
            source_port,
        )?;

        let target = PipelineEndpoint::port(
            target_stage.clone(),
            target_port,
        )?;

        self.add_edge(PipelineEdge {
            kind: PipelineEdgeKind::DataFlow,
            source,
            target,
        })
    }

    /// Add an already constructed edge.
    ///
    /// This method is intentionally private. Public callers must use the
    /// methods that preserve endpoint and port invariants.
    fn add_edge(
        &mut self,
        edge: PipelineEdge,
    ) -> Result<(), PipelineGraphError> {
        self.validate_stage(edge.source.stage_id())?;
        self.validate_stage(edge.target.stage_id())?;

        match edge.kind {
            PipelineEdgeKind::DataFlow => {
                if edge.source.port_name().is_none()
                    || edge.target.port_name().is_none()
                {
                    return Err(PipelineGraphError::DataFlowRequiresPorts);
                }
            }

            PipelineEdgeKind::Dependency | PipelineEdgeKind::Control => {
                if edge.source.port_name().is_some()
                    || edge.target.port_name().is_some()
                {
                    return Err(PipelineGraphError::StageOnlyEdgeHasPort);
                }
            }
        }

        if self.edges.contains(&edge) {
            return Err(PipelineGraphError::DuplicateEdge {
                source: edge.source.stage.clone(),
                target: edge.target.stage.clone(),
                kind: edge.kind,
            });
        }

        self.edges.insert(edge);

        Ok(())
    }

    /// Verify that a stage is declared in the source definition.
    fn validate_stage(
        &self,
        stage: &PipelineStageId,
    ) -> Result<(), PipelineGraphError> {
        if self.nodes.contains(stage) {
            Ok(())
        } else {
            Err(PipelineGraphError::UnknownStage {
                stage: stage.clone(),
            })
        }
    }

    /// Verify that a named port exists and has the required direction.
    fn validate_port(
        &self,
        stage_id: &PipelineStageId,
        port_name: &str,
        required_direction: PipelinePortDirection,
    ) -> Result<(), PipelineGraphError> {
        self.validate_stage(stage_id)?;

        let stage = self
            .definition
            .stage(stage_id)
            .ok_or_else(|| PipelineGraphError::UnknownStage {
                stage: stage_id.clone(),
            })?;

        let port = stage
            .ports()
            .iter()
            .find(|port| port.name() == port_name)
            .ok_or_else(|| PipelineGraphError::UnknownPort {
                stage: stage_id.clone(),
                port: port_name.to_owned(),
            })?;

        if port.direction() != required_direction {
            return Err(PipelineGraphError::WrongPortDirection {
                stage: stage_id.clone(),
                port: port_name.to_owned(),
                expected: required_direction,
                actual: port.direction(),
            });
        }

        Ok(())
    }

    /// Build the immutable graph and its derived adjacency indexes.
    ///
    /// No partial graph is returned if construction fails.
    pub fn build(self) -> PipelineGraph {
        let mut outgoing: BTreeMap<
            PipelineStageId,
            BTreeSet<PipelineStageId>,
        > = self
            .nodes
            .iter()
            .cloned()
            .map(|node| (node, BTreeSet::new()))
            .collect();

        let mut incoming: BTreeMap<
            PipelineStageId,
            BTreeSet<PipelineStageId>,
        > = self
            .nodes
            .iter()
            .cloned()
            .map(|node| (node, BTreeSet::new()))
            .collect();

        for edge in &self.edges {
            outgoing
                .entry(edge.source.stage.clone())
                .or_default()
                .insert(edge.target.stage.clone());

            incoming
                .entry(edge.target.stage.clone())
                .or_default()
                .insert(edge.source.stage.clone());
        }

        PipelineGraph {
            pipeline_id: self.definition.id(),
            nodes: self.nodes,
            edges: self.edges,
            outgoing,
            incoming,
        }
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors raised while constructing or querying pipeline graph structure.
///
/// These errors describe structural graph failures. They do not replace
/// semantic diagnostics, type errors, or execution errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipelineGraphError {
    /// A referenced stage was not declared in the pipeline definition.
    UnknownStage {
        stage: PipelineStageId,
    },

    /// A referenced port was not declared on the stage.
    UnknownPort {
        stage: PipelineStageId,
        port: String,
    },

    /// A port has a direction incompatible with the requested edge.
    WrongPortDirection {
        stage: PipelineStageId,
        port: String,
        expected: PipelinePortDirection,
        actual: PipelinePortDirection,
    },

    /// A port name was empty or contained a control character.
    InvalidPortName {
        port: String,
    },

    /// A data-flow edge must identify both endpoint ports.
    DataFlowRequiresPorts,

    /// A dependency or control edge must identify stages, not ports.
    StageOnlyEdgeHasPort,

    /// An exact edge already exists.
    DuplicateEdge {
        source: PipelineStageId,
        target: PipelineStageId,
        kind: PipelineEdgeKind,
    },
}

impl fmt::Display for PipelineGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownStage { stage } => {
                write!(f, "pipeline graph references undeclared stage '{stage}'")
            }

            Self::UnknownPort { stage, port } => {
                write!(
                    f,
                    "pipeline graph references undeclared port '{port}' on stage '{stage}'"
                )
            }

            Self::WrongPortDirection {
                stage,
                port,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "port '{port}' on stage '{stage}' has direction \
                     {actual:?}; expected {expected:?}"
                )
            }

            Self::InvalidPortName { port } => {
                write!(
                    f,
                    "pipeline graph port name '{port}' is empty or contains control characters"
                )
            }

            Self::DataFlowRequiresPorts => {
                f.write_str(
                    "a data-flow edge requires both a source port and a target port",
                )
            }

            Self::StageOnlyEdgeHasPort => {
                f.write_str(
                    "dependency and control edges must use stage-only endpoints",
                )
            }

            Self::DuplicateEdge {
                source,
                target,
                kind,
            } => {
                write!(
                    f,
                    "duplicate {kind:?} edge from stage '{source}' to stage '{target}'"
                )
            }
        }
    }
}

impl Error for PipelineGraphError {}

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
        PipelineStageDefinition,
    };

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value).expect("valid stage ID")
    }

    fn stage(value: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(value),
            PipelineOperationRef::new("zamani.ai", "operation")
                .expect("valid operation reference"),
        )
    }

    fn definition(
        stages: Vec<PipelineStageDefinition>,
    ) -> PipelineDefinition {
        let id = AiPipelineId::new(1).expect("valid pipeline ID");

        let mut builder =
            PipelineDefinitionBuilder::new(id, "test_pipeline")
                .expect("valid pipeline definition");

        for stage in stages {
            builder.add_stage(stage).expect("unique stage");
        }

        builder.build()
    }

    fn stage_with_ports(
        name: &str,
        ports: Vec<PipelinePort>,
    ) -> PipelineStageDefinition {
        let mut result = stage(name);

        for port in ports {
            result.add_port(port).expect("unique port");
        }

        result
    }

    fn input(name: &str) -> PipelinePort {
        PipelinePort::new(
            name,
            PipelinePortDirection::Input,
            None,
        )
        .expect("valid input")
    }

    fn output(name: &str) -> PipelinePort {
        PipelinePort::new(
            name,
            PipelinePortDirection::Output,
            None,
        )
        .expect("valid output")
    }

    #[test]
    fn graph_contains_every_declared_stage() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
            stage("third"),
        ]);

        let graph = PipelineGraph::builder(&definition).build();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.contains_node(&stage_id("first")));
        assert!(graph.contains_node(&stage_id("second")));
        assert!(graph.contains_node(&stage_id("third")));
    }

    #[test]
    fn graph_preserves_pipeline_identity() {
        let definition = definition(vec![stage("first")]);
        let expected = definition.id();

        let graph = PipelineGraph::builder(&definition).build();

        assert_eq!(graph.pipeline_id(), expected);
    }

    #[test]
    fn dependency_connects_two_declared_stages() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
        ]);

        let first = stage_id("first");
        let second = stage_id("second");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&first, &second)
            .expect("valid dependency");

        let graph = builder.build();

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(
            graph.successors(&first).cloned().collect::<Vec<_>>(),
            vec![second.clone()],
        );
        assert_eq!(
            graph.predecessors(&second).cloned().collect::<Vec<_>>(),
            vec![first.clone()],
        );
    }

    #[test]
    fn unknown_source_stage_is_rejected() {
        let definition = definition(vec![stage("known")]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_dependency(
            &stage_id("missing"),
            &stage_id("known"),
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::UnknownStage { .. })
        ));
    }

    #[test]
    fn unknown_target_stage_is_rejected() {
        let definition = definition(vec![stage("known")]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_dependency(
            &stage_id("known"),
            &stage_id("missing"),
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::UnknownStage { .. })
        ));
    }

    #[test]
    fn exact_duplicate_dependency_is_rejected() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
        ]);

        let first = stage_id("first");
        let second = stage_id("second");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&first, &second)
            .expect("first edge");

        assert!(matches!(
            builder.add_dependency(&first, &second),
            Err(PipelineGraphError::DuplicateEdge { .. })
        ));
    }

    #[test]
    fn different_edge_kinds_between_stages_are_supported() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
        ]);

        let first = stage_id("first");
        let second = stage_id("second");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&first, &second)
            .expect("dependency");

        builder
            .add_control_dependency(&first, &second)
            .expect("control dependency");

        let graph = builder.build();

        assert_eq!(graph.edge_count(), 2);

        // Adjacency queries return distinct neighboring stages, not
        // duplicate entries for each edge.
        assert_eq!(graph.successor_count(&first), 1);
        assert_eq!(graph.predecessor_count(&second), 1);
    }

    #[test]
    fn data_flow_requires_output_to_input_ports() {
        let definition = definition(vec![
            stage_with_ports("producer", vec![output("result")]),
            stage_with_ports("consumer", vec![input("data")]),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_data_flow(
                &stage_id("producer"),
                "result",
                &stage_id("consumer"),
                "data",
            )
            .expect("valid data flow");

        let graph = builder.build();

        assert_eq!(graph.edge_count(), 1);

        let edge = graph.edges().next().expect("edge");

        assert_eq!(edge.kind(), PipelineEdgeKind::DataFlow);
        assert_eq!(edge.source().port_name(), Some("result"));
        assert_eq!(edge.target().port_name(), Some("data"));
    }

    #[test]
    fn missing_source_port_is_rejected() {
        let definition = definition(vec![
            stage("producer"),
            stage_with_ports("consumer", vec![input("data")]),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_data_flow(
            &stage_id("producer"),
            "missing",
            &stage_id("consumer"),
            "data",
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::UnknownPort { .. })
        ));
    }

    #[test]
    fn missing_target_port_is_rejected() {
        let definition = definition(vec![
            stage_with_ports("producer", vec![output("result")]),
            stage("consumer"),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_data_flow(
            &stage_id("producer"),
            "result",
            &stage_id("consumer"),
            "missing",
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::UnknownPort { .. })
        ));
    }

    #[test]
    fn input_port_cannot_be_used_as_data_flow_source() {
        let definition = definition(vec![
            stage_with_ports("producer", vec![input("data")]),
            stage_with_ports("consumer", vec![input("data")]),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_data_flow(
            &stage_id("producer"),
            "data",
            &stage_id("consumer"),
            "data",
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::WrongPortDirection {
                expected: PipelinePortDirection::Output,
                ..
            })
        ));
    }

    #[test]
    fn output_port_cannot_be_used_as_data_flow_target() {
        let definition = definition(vec![
            stage_with_ports("producer", vec![output("result")]),
            stage_with_ports("consumer", vec![output("result")]),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        let result = builder.add_data_flow(
            &stage_id("producer"),
            "result",
            &stage_id("consumer"),
            "result",
        );

        assert!(matches!(
            result,
            Err(PipelineGraphError::WrongPortDirection {
                expected: PipelinePortDirection::Input,
                ..
            })
        ));
    }

    #[test]
    fn distinct_port_connections_between_same_stages_are_supported() {
        let definition = definition(vec![
            stage_with_ports(
                "producer",
                vec![output("left"), output("right")],
            ),
            stage_with_ports(
                "consumer",
                vec![input("left"), input("right")],
            ),
        ]);

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_data_flow(
                &stage_id("producer"),
                "left",
                &stage_id("consumer"),
                "left",
            )
            .expect("left flow");

        builder
            .add_data_flow(
                &stage_id("producer"),
                "right",
                &stage_id("consumer"),
                "right",
            )
            .expect("right flow");

        let graph = builder.build();

        assert_eq!(graph.edge_count(), 2);
        assert_eq!(
            graph.successor_count(&stage_id("producer")),
            1,
        );
    }

    #[test]
    fn self_edges_are_structurally_representable() {
        let definition = definition(vec![stage("iterative")]);

        let iterative = stage_id("iterative");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&iterative, &iterative)
            .expect("structurally valid self-edge");

        let graph = builder.build();

        assert!(graph.has_self_edge());
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn cycles_are_preserved_for_semantic_validation() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
            stage("third"),
        ]);

        let first = stage_id("first");
        let second = stage_id("second");
        let third = stage_id("third");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&first, &second)
            .expect("first edge");

        builder
            .add_dependency(&second, &third)
            .expect("second edge");

        builder
            .add_dependency(&third, &first)
            .expect("third edge");

        let graph = builder.build();

        assert_eq!(graph.edge_count(), 3);
        assert!(graph.contains_node(&first));
    }

    #[test]
    fn source_and_sink_nodes_are_identified() {
        let definition = definition(vec![
            stage("source"),
            stage("middle"),
            stage("sink"),
        ]);

        let source = stage_id("source");
        let middle = stage_id("middle");
        let sink = stage_id("sink");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&source, &middle)
            .expect("first edge");

        builder
            .add_dependency(&middle, &sink)
            .expect("second edge");

        let graph = builder.build();

        assert_eq!(
            graph.source_nodes().cloned().collect::<Vec<_>>(),
            vec![source],
        );

        assert_eq!(
            graph.sink_nodes().cloned().collect::<Vec<_>>(),
            vec![sink],
        );
    }

    #[test]
    fn empty_definition_produces_empty_graph() {
        let definition = definition(Vec::new());

        let graph = PipelineGraph::builder(&definition).build();

        assert!(graph.is_empty());
        assert!(graph.has_no_edges());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn graph_iteration_is_deterministic() {
        let definition = definition(vec![
            stage("zeta"),
            stage("alpha"),
            stage("middle"),
        ]);

        let graph = PipelineGraph::builder(&definition).build();

        let nodes: Vec<_> = graph
            .nodes()
            .map(PipelineStageId::as_str)
            .collect();

        assert_eq!(nodes, ["alpha", "middle", "zeta"]);
    }

    #[test]
    fn outgoing_and_incoming_edges_preserve_edge_details() {
        let definition = definition(vec![
            stage("first"),
            stage("second"),
        ]);

        let first = stage_id("first");
        let second = stage_id("second");

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(&first, &second)
            .expect("dependency");

        builder
            .add_control_dependency(&first, &second)
            .expect("control");

        let graph = builder.build();

        assert_eq!(graph.outgoing_edges(&first).count(), 2);
        assert_eq!(graph.incoming_edges(&second).count(), 2);
        assert_eq!(graph.edges_between(&first, &second).count(), 2);
    }
}
