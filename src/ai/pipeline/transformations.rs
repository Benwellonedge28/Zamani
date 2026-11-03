
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI — Pipeline Transformations
//!
//! Path: `src/ai/pipeline/transformations.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent transformations of the existing logical
//! AI pipeline graph.
//!
//! Transformations describe changes to logical pipeline relationships.
//! They do not execute pipeline operations or determine physical
//! execution arrangements.
//!
//! ## Architectural ownership
//!
//! Canonical owners:
//!
//! - `definition.rs`: pipeline and stage declarations.
//! - `graph.rs`: canonical pipeline graph and graph edges.
//! - `dependencies.rs`: dependency analysis and cycle detection.
//! - `validation.rs`: complete semantic pipeline validation.
//! - `execution_plan.rs`: abstract execution-plan construction.
//! - `frontend::semantic::domains::ai`: AI semantic authority.
//! - `zuir::dialects::ai`: universal AI operation representation.
//! - `zuir::dialects::tensor`: universal tensor operation representation.
//! - `distributed`: distributed placement and scheduling.
//! - `runtime`: execution.
//!
//! This module consumes these authorities without replacing them.
//!
//! ## Transformation model
//!
//! A transformation is a versioned, named, declarative sequence of
//! logical graph edits.
//!
//! Supported graph edits:
//!
//! - Add a logical dependency.
//! - Add a control dependency.
//! - Add a data-flow relationship.
//! - Remove an existing logical relationship.
//!
//! Graph transformations never introduce undeclared stages.
//!
//! Stage declarations and operation identities remain owned by
//! PipelineDefinition.
//!
//! ## Semantic safety
//!
//! A successfully reconstructed graph is not automatically
//! semantically valid.
//!
//! Every transformed graph is explicitly marked as requiring
//! downstream semantic revalidation.
//!
//! This is necessary because even a structurally valid graph edit
//! may change observable ordering or execution behavior.
//!
//! The canonical semantic validator must determine whether the
//! transformation preserves the source program's required meaning.
//!
//! This module never claims to prove semantic equivalence.
//!
//! ## Transactional behavior
//!
//! Transformations are applied to immutable input graphs.
//!
//! A new graph is returned only after all requested edits have
//! been applied and graph construction has succeeded.
//!
//! An error never returns a partially transformed graph.
//!
//! Applying a sequence of transformations is similarly atomic from
//! the caller's perspective: the final graph is returned only if
//! every transformation succeeds.
//!
//! ## Determinism
//!
//! Transformation identifiers are preserved exactly.
//!
//! Graph edits use ordered collections.
//!
//! Reports preserve transformation application order.
//!
//! Equivalent inputs and transformation sequences produce equivalent
//! logical graph structures.
//!
//! ## Scalability and POCO-REAF
//!
//! No semantic maximum is imposed on:
//!
//! - Pipeline stages.
//! - Graph edges.
//! - Transformation count.
//! - Transformation sequence length.
//! - Graph fan-in.
//! - Graph fan-out.
//! - Graph depth.
//! - Pipeline dependencies.
//! - Data-flow relationships.
//!
//! Dynamic collections are used.
//!
//! Actual memory, address-space, allocator, process and operational
//! budgets remain finite implementation constraints.
//!
//! Literal infinite storage or computation is not promised.
//!
//! Operational resource budgets must not redefine source semantics.
//!
//! ## Hardware independence
//!
//! This module contains no:
//!
//! - CPU identifiers.
//! - GPU identifiers.
//! - QPU identifiers.
//! - FPGA identifiers.
//! - Accelerator assignments.
//! - Vendor-specific operations.
//! - Worker assignments.
//! - Network routes.
//! - Memory addresses.
//! - Hardware-specific scheduling.
//!
//! The same logical transformations can therefore be considered
//! by different compatible compiler and execution implementations.
//!
//! ## Integration
//!
//! `PipelineDefinition`
//!     Supplies the authoritative stage declarations.
//!
//! `PipelineGraph`
//!     Supplies the authoritative logical graph.
//!
//! `PipelineGraphBuilder`
//!     Reconstructs transformed graphs.
//!
//! `DependencyAnalysis`
//!     Evaluates graph dependency preconditions.
//!
//! `validation.rs`
//!     Must revalidate every transformed graph.
//!
//! `execution_plan.rs`
//!     May consume only appropriately validated graph information.
//!
//! `zuir::dialects::ai`
//!     Receives downstream AI operation representations.
//!
//! `zuir::dialects::tensor`
//!     Receives downstream tensor operation representations.
//!
//! `distributed`
//!     Owns physical placement and scheduling.
//!
//! `runtime`
//!     Owns actual execution.
//!
//! ## Freeze criteria
//!
//! - Uses existing canonical pipeline definitions.
//! - Uses existing canonical pipeline graphs.
//! - Does not introduce another graph representation.
//! - Does not introduce another dependency analyzer.
//! - Supports all existing edge categories.
//! - Preserves declared stage identities.
//! - Rejects invalid edits.
//! - Rejects transformations against unrelated definitions.
//! - Reconstructs graphs transactionally.
//! - Reports semantic revalidation requirements.
//! - Supports deterministic transformation sequences.
//! - Does not impose arbitrary semantic capacity ceilings.
//! - Contains no unsafe Rust.

use crate::ai::pipeline::definition::{
    PipelineDefinition,
    PipelineStageId,
};

use crate::ai::pipeline::dependencies::DependencyAnalysis;

use crate::ai::pipeline::graph::{
    PipelineEdge,
    PipelineEdgeKind,
    PipelineGraph,
    PipelineGraphBuilder,
    PipelineGraphError,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

// ============================================================================
// TRANSFORMATION IDENTITY
// ============================================================================

/// A stable, namespaced identity for a pipeline transformation.
///
/// The identity is local to the transformation ecosystem.
/// It is not a pipeline ID, stage ID, ZUIR operation ID,
/// or runtime task ID.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransformationId {
    namespace: String,
    name: String,
}

impl TransformationId {
    /// Construct a transformation identity.
    ///
    /// Empty components and control characters are rejected.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, TransformationError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(
            "transformation namespace",
            &namespace,
        )?;

        validate_identity_component(
            "transformation name",
            &name,
        )?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the transformation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the transformation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for TransformationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// TRANSFORMATION VERSION
// ============================================================================

/// Version of a transformation's contract.
///
/// This is not a Zamani language version, pipeline definition
/// version, ZUIR dialect version, or model artifact version.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TransformationVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl TransformationVersion {
    /// Construct a transformation version.
    #[must_use]
    pub const fn new(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch version.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

// ============================================================================
// GRAPH EDGE SELECTOR
// ============================================================================

/// Identifies an exact logical relationship in a pipeline graph.
///
/// This is a selector for an existing graph edge.
/// It is not a second graph-edge implementation.
///
/// Data-flow selectors include both endpoint port names.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct PipelineEdgeSelector {
    kind: PipelineEdgeKind,
    source: PipelineStageId,
    source_port: Option<String>,
    target: PipelineStageId,
    target_port: Option<String>,
}

impl PipelineEdgeSelector {
    /// Construct a dependency-edge selector.
    #[must_use]
    pub fn dependency(
        source: PipelineStageId,
        target: PipelineStageId,
    ) -> Self {
        Self {
            kind: PipelineEdgeKind::Dependency,
            source,
            source_port: None,
            target,
            target_port: None,
        }
    }

    /// Construct a control-edge selector.
    #[must_use]
    pub fn control(
        source: PipelineStageId,
        target: PipelineStageId,
    ) -> Self {
        Self {
            kind: PipelineEdgeKind::Control,
            source,
            source_port: None,
            target,
            target_port: None,
        }
    }

    /// Construct a data-flow edge selector.
    ///
    /// The graph builder will verify that the source and target
    /// ports exist and have the appropriate directions.
    pub fn data_flow(
        source: PipelineStageId,
        source_port: impl Into<String>,
        target: PipelineStageId,
        target_port: impl Into<String>,
    ) -> Result<Self, TransformationError> {
        let source_port = source_port.into();
        let target_port = target_port.into();

        validate_identity_component(
            "source port",
            &source_port,
        )?;

        validate_identity_component(
            "target port",
            &target_port,
        )?;

        Ok(Self {
            kind: PipelineEdgeKind::DataFlow,
            source,
            source_port: Some(source_port),
            target,
            target_port: Some(target_port),
        })
    }

    /// Return the relationship kind.
    #[must_use]
    pub const fn kind(&self) -> PipelineEdgeKind {
        self.kind
    }

    /// Return the source stage.
    #[must_use]
    pub fn source(&self) -> &PipelineStageId {
        &self.source
    }

    /// Return the optional source port.
    #[must_use]
    pub fn source_port(&self) -> Option<&str> {
        self.source_port.as_deref()
    }

    /// Return the target stage.
    #[must_use]
    pub fn target(&self) -> &PipelineStageId {
        &self.target
    }

    /// Return the optional target port.
    #[must_use]
    pub fn target_port(&self) -> Option<&str> {
        self.target_port.as_deref()
    }

    fn from_edge(edge: &PipelineEdge) -> Self {
        Self {
            kind: edge.kind(),
            source: edge.source().stage_id().clone(),
            source_port: edge
                .source()
                .port_name()
                .map(str::to_owned),
            target: edge.target().stage_id().clone(),
            target_port: edge
                .target()
                .port_name()
                .map(str::to_owned),
        }
    }

    fn validate_shape(&self) -> Result<(), TransformationError> {
        let valid = match self.kind {
            PipelineEdgeKind::Dependency
            | PipelineEdgeKind::Control => {
                self.source_port.is_none()
                    && self.target_port.is_none()
            }

            PipelineEdgeKind::DataFlow => {
                self.source_port.is_some()
                    && self.target_port.is_some()
            }
        };

        if valid {
            Ok(())
        } else {
            Err(TransformationError::InvalidEdgeShape)
        }
    }

    fn insert_into(
        &self,
        builder: &mut PipelineGraphBuilder<'_>,
    ) -> Result<(), TransformationError> {
        self.validate_shape()?;

        match self.kind {
            PipelineEdgeKind::Dependency => {
                builder.add_dependency(
                    &self.source,
                    &self.target,
                )?;
            }

            PipelineEdgeKind::Control => {
                builder.add_control_dependency(
                    &self.source,
                    &self.target,
                )?;
            }

            PipelineEdgeKind::DataFlow => {
                builder.add_data_flow(
                    &self.source,
                    self.source_port
                        .as_deref()
                        .ok_or(TransformationError::InvalidEdgeShape)?,
                    &self.target,
                    self.target_port
                        .as_deref()
                        .ok_or(TransformationError::InvalidEdgeShape)?,
                )?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// GRAPH EDIT OPERATIONS
// ============================================================================

/// A declarative modification of pipeline graph relationships.
///
/// These operations do not modify the pipeline definition,
/// stage operation identities, or stage port declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PipelineGraphEdit {
    /// Add an explicit logical dependency.
    AddDependency {
        /// Dependency source stage.
        source: PipelineStageId,

        /// Dependency target stage.
        target: PipelineStageId,
    },

    /// Add a logical control dependency.
    AddControlDependency {
        /// Control source stage.
        source: PipelineStageId,

        /// Control target stage.
        target: PipelineStageId,
    },

    /// Add a data-flow relationship.
    AddDataFlow {
        /// Source stage.
        source: PipelineStageId,

        /// Source output port.
        source_port: String,

        /// Target stage.
        target: PipelineStageId,

        /// Target input port.
        target_port: String,
    },

    /// Remove one exact existing logical relationship.
    ///
    /// Removing an edge may change program behavior.
    /// The transformed graph therefore always requires
    /// downstream semantic revalidation.
    RemoveEdge {
        /// Exact relationship to remove.
        edge: PipelineEdgeSelector,
    },
}

impl PipelineGraphEdit {
    /// Construct an addition of a dependency.
    #[must_use]
    pub fn add_dependency(
        source: PipelineStageId,
        target: PipelineStageId,
    ) -> Self {
        Self::AddDependency {
            source,
            target,
        }
    }

    /// Construct an addition of a control dependency.
    #[must_use]
    pub fn add_control_dependency(
        source: PipelineStageId,
        target: PipelineStageId,
    ) -> Self {
        Self::AddControlDependency {
            source,
            target,
        }
    }

    /// Construct an addition of a data-flow relationship.
    #[must_use]
    pub fn add_data_flow(
        source: PipelineStageId,
        source_port: impl Into<String>,
        target: PipelineStageId,
        target_port: impl Into<String>,
    ) -> Self {
        Self::AddDataFlow {
            source,
            source_port: source_port.into(),
            target,
            target_port: target_port.into(),
        }
    }

    /// Construct removal of an exact relationship.
    #[must_use]
    pub fn remove_edge(
        edge: PipelineEdgeSelector,
    ) -> Self {
        Self::RemoveEdge { edge }
    }

    fn selector(
        &self,
    ) -> Result<Option<PipelineEdgeSelector>, TransformationError> {
        match self {
            Self::AddDependency {
                source,
                target,
            } => Ok(Some(
                PipelineEdgeSelector::dependency(
                    source.clone(),
                    target.clone(),
                ),
            )),

            Self::AddControlDependency {
                source,
                target,
            } => Ok(Some(
                PipelineEdgeSelector::control(
                    source.clone(),
                    target.clone(),
                ),
            )),

            Self::AddDataFlow {
                source,
                source_port,
                target,
                target_port,
            } => Ok(Some(
                PipelineEdgeSelector::data_flow(
                    source.clone(),
                    source_port.clone(),
                    target.clone(),
                    target_port.clone(),
                )?,
            )),

            Self::RemoveEdge { .. } => Ok(None),
        }
    }
}

// ============================================================================
// TRANSFORMATION PRECONDITIONS
// ============================================================================

/// A condition that must hold before a transformation is applied.
///
/// These are structural preconditions, not substitutes for
/// canonical semantic validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransformationPrecondition {
    /// The existing graph must be acyclic.
    RequireAcyclic,

    /// A particular stage must exist.
    RequireStage {
        /// Required stage identity.
        stage: PipelineStageId,
    },

    /// The graph must contain at least one stage.
    RequireNonEmpty,
}

// ============================================================================
// TRANSFORMATION DEFINITION
// ============================================================================

/// A complete, immutable pipeline transformation description.
///
/// A transformation is a compiler-level logical graph rewrite.
/// It contains no execution closure, hardware reference,
/// device allocation, or runtime handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineTransformation {
    id: TransformationId,
    version: TransformationVersion,
    description: String,
    preconditions: Vec<TransformationPrecondition>,
    edits: Vec<PipelineGraphEdit>,
}

impl PipelineTransformation {
    /// Construct a transformation.
    ///
    /// The transformation may contain zero or more edits.
    /// Empty transformations are permitted as explicit identity
    /// transformations.
    pub fn new(
        id: TransformationId,
        version: TransformationVersion,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            version,
            description: description.into(),
            preconditions: Vec::new(),
            edits: Vec::new(),
        }
    }

    /// Return the transformation identity.
    #[must_use]
    pub fn id(&self) -> &TransformationId {
        &self.id
    }

    /// Return the transformation version.
    #[must_use]
    pub const fn version(&self) -> TransformationVersion {
        self.version
    }

    /// Return the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return the declared preconditions.
    #[must_use]
    pub fn preconditions(
        &self,
    ) -> &[TransformationPrecondition] {
        &self.preconditions
    }

    /// Return the graph edits.
    #[must_use]
    pub fn edits(&self) -> &[PipelineGraphEdit] {
        &self.edits
    }

    /// Add a transformation precondition.
    pub fn require(
        &mut self,
        precondition: TransformationPrecondition,
    ) {
        self.preconditions.push(precondition);
    }

    /// Append a graph edit.
    pub fn add_edit(
        &mut self,
        edit: PipelineGraphEdit,
    ) {
        self.edits.push(edit);
    }

    /// Return whether this transformation has graph edits.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.edits.is_empty()
    }
}

// ============================================================================
// TRANSFORMATION REPORT
// ============================================================================

/// Semantic status of a transformed graph.
///
/// Structural reconstruction is not proof of semantic validity.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TransformationSemanticStatus {
    /// The graph requires downstream semantic validation.
    ///
    /// This is the status of every graph produced by this module.
    RequiresRevalidation,
}

/// Result information for one applied transformation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformationStepReport {
    id: TransformationId,
    version: TransformationVersion,
    previous_edge_count: usize,
    resulting_edge_count: usize,
    additions: usize,
    removals: usize,
    semantic_status: TransformationSemanticStatus,
}

impl TransformationStepReport {
    /// Return the transformation identity.
    #[must_use]
    pub fn id(&self) -> &TransformationId {
        &self.id
    }

    /// Return the transformation version.
    #[must_use]
    pub const fn version(&self) -> TransformationVersion {
        self.version
    }

    /// Return the previous graph edge count.
    #[must_use]
    pub const fn previous_edge_count(&self) -> usize {
        self.previous_edge_count
    }

    /// Return the resulting graph edge count.
    #[must_use]
    pub const fn resulting_edge_count(&self) -> usize {
        self.resulting_edge_count
    }

    /// Return the number of added relationships.
    #[must_use]
    pub const fn additions(&self) -> usize {
        self.additions
    }

    /// Return the number of removed relationships.
    #[must_use]
    pub const fn removals(&self) -> usize {
        self.removals
    }

    /// Return the semantic validation status.
    #[must_use]
    pub const fn semantic_status(
        &self,
    ) -> TransformationSemanticStatus {
        self.semantic_status
    }
}

/// Complete report for an applied transformation sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformationReport {
    pipeline_id: crate::ai::ids::AiPipelineId,
    initial_node_count: usize,
    initial_edge_count: usize,
    resulting_node_count: usize,
    resulting_edge_count: usize,
    steps: Vec<TransformationStepReport>,
    semantic_status: TransformationSemanticStatus,
}

impl TransformationReport {
    /// Return the pipeline identity.
    #[must_use]
    pub const fn pipeline_id(
        &self,
    ) -> crate::ai::ids::AiPipelineId {
        self.pipeline_id
    }

    /// Return the initial node count.
    #[must_use]
    pub const fn initial_node_count(&self) -> usize {
        self.initial_node_count
    }

    /// Return the initial edge count.
    #[must_use]
    pub const fn initial_edge_count(&self) -> usize {
        self.initial_edge_count
    }

    /// Return the resulting node count.
    #[must_use]
    pub const fn resulting_node_count(&self) -> usize {
        self.resulting_node_count
    }

    /// Return the resulting edge count.
    #[must_use]
    pub const fn resulting_edge_count(&self) -> usize {
        self.resulting_edge_count
    }

    /// Return the transformation reports in application order.
    #[must_use]
    pub fn steps(&self) -> &[TransformationStepReport] {
        &self.steps
    }

    /// Return the semantic validation status.
    #[must_use]
    pub const fn semantic_status(
        &self,
    ) -> TransformationSemanticStatus {
        self.semantic_status
    }

    /// Return the total number of added relationships.
    #[must_use]
    pub fn total_additions(&self) -> usize {
        self.steps
            .iter()
            .map(TransformationStepReport::additions)
            .sum()
    }

    /// Return the total number of removed relationships.
    #[must_use]
    pub fn total_removals(&self) -> usize {
        self.steps
            .iter()
            .map(TransformationStepReport::removals)
            .sum()
    }
}

// ============================================================================
// TRANSFORMATION RESULT
// ============================================================================

/// A successfully reconstructed transformed pipeline graph.
///
/// The original graph remains unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformedPipeline {
    graph: PipelineGraph,
    report: TransformationReport,
}

impl TransformedPipeline {
    /// Return the resulting logical graph.
    #[must_use]
    pub fn graph(&self) -> &PipelineGraph {
        &self.graph
    }

    /// Consume the result and return its graph.
    #[must_use]
    pub fn into_graph(self) -> PipelineGraph {
        self.graph
    }

    /// Return the transformation report.
    #[must_use]
    pub fn report(&self) -> &TransformationReport {
        &self.report
    }

    /// Consume the result and return the graph and report.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (PipelineGraph, TransformationReport) {
        (self.graph, self.report)
    }
}

// ============================================================================
// TRANSFORMATION ENGINE
// ============================================================================

/// Stateless pipeline graph transformation engine.
///
/// The engine operates exclusively on supplied definitions,
/// graphs, and transformation descriptions.
///
/// It owns no global registry or mutable process-wide state.
#[derive(Clone, Copy, Debug, Default)]
pub struct PipelineTransformationEngine;

impl PipelineTransformationEngine {
    /// Apply one transformation to an existing pipeline graph.
    ///
    /// The definition and graph must describe the same pipeline.
    ///
    /// The resulting graph is reconstructed from the authoritative
    /// definition and the transformed logical edge set.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The definition and graph identities differ.
    /// - The graph nodes differ from the declared stages.
    /// - A precondition is not satisfied.
    /// - An edge edit is invalid.
    /// - A duplicate edge is introduced.
    /// - An edge removal references a nonexistent relationship.
    /// - Graph reconstruction fails.
    pub fn apply(
        definition: &PipelineDefinition,
        graph: &PipelineGraph,
        transformation: &PipelineTransformation,
    ) -> Result<TransformedPipeline, TransformationError> {
        validate_graph_ownership(definition, graph)?;

        Self::validate_preconditions(
            graph,
            transformation.preconditions(),
        )?;

        let initial_node_count = graph.node_count();
        let initial_edge_count = graph.edge_count();

        let mut edges = graph
            .edges()
            .map(PipelineEdgeSelector::from_edge)
            .collect::<BTreeSet<_>>();

        let mut additions = 0usize;
        let mut removals = 0usize;

        for edit in transformation.edits() {
            match edit {
                PipelineGraphEdit::AddDependency { .. }
                | PipelineGraphEdit::AddControlDependency { .. }
                | PipelineGraphEdit::AddDataFlow { .. } => {
                    let selector = edit
                        .selector()?
                        .ok_or(
                            TransformationError::InvalidEdgeShape,
                        )?;

                    if !edges.insert(selector.clone()) {
                        return Err(
                            TransformationError::DuplicateEdit {
                                edge: selector,
                            },
                        );
                    }

                    additions += 1;
                }

                PipelineGraphEdit::RemoveEdge { edge } => {
                    edge.validate_shape()?;

                    if !edges.remove(edge) {
                        return Err(
                            TransformationError::EdgeNotFound {
                                edge: edge.clone(),
                            },
                        );
                    }

                    removals += 1;
                }
            }
        }

        let mut builder = PipelineGraph::builder(definition);

        for edge in &edges {
            edge.insert_into(&mut builder)?;
        }

        let transformed_graph = builder.build();

        // The builder reconstructs all declared nodes.
        // Confirm that the graph still belongs to the definition.
        validate_graph_ownership(
            definition,
            &transformed_graph,
        )?;

        let report = TransformationReport {
            pipeline_id: definition.id(),
            initial_node_count,
            initial_edge_count,
            resulting_node_count: transformed_graph.node_count(),
            resulting_edge_count: transformed_graph.edge_count(),
            steps: vec![
                TransformationStepReport {
                    id: transformation.id().clone(),
                    version: transformation.version(),
                    previous_edge_count: initial_edge_count,
                    resulting_edge_count: transformed_graph.edge_count(),
                    additions,
                    removals,
                    semantic_status:
                        TransformationSemanticStatus::RequiresRevalidation,
                },
            ],
            semantic_status:
                TransformationSemanticStatus::RequiresRevalidation,
        };

        Ok(TransformedPipeline {
            graph: transformed_graph,
            report,
        })
    }

    /// Apply an ordered sequence of transformations.
    ///
    /// Each transformation receives the graph produced by the
    /// preceding transformation.
    ///
    /// No intermediate graph is returned if any transformation fails.
    ///
    /// An empty sequence returns a logically identical graph.
    ///
    /// # Errors
    ///
    /// Returns the first transformation error.
    pub fn apply_sequence(
        definition: &PipelineDefinition,
        graph: &PipelineGraph,
        transformations: &[PipelineTransformation],
    ) -> Result<TransformedPipeline, TransformationError> {
        validate_graph_ownership(definition, graph)?;

        let initial_node_count = graph.node_count();
        let initial_edge_count = graph.edge_count();

        let mut current_graph = graph.clone();
        let mut reports = Vec::new();

        for transformation in transformations {
            let result = Self::apply(
                definition,
                &current_graph,
                transformation,
            )?;

            let (next_graph, report) = result.into_parts();

            reports.extend(report.steps);

            current_graph = next_graph;
        }

        let semantic_status =
            if transformations.is_empty() {
                TransformationSemanticStatus::RequiresRevalidation
            } else {
                TransformationSemanticStatus::RequiresRevalidation
            };

        let report = TransformationReport {
            pipeline_id: definition.id(),
            initial_node_count,
            initial_edge_count,
            resulting_node_count: current_graph.node_count(),
            resulting_edge_count: current_graph.edge_count(),
            steps: reports,
            semantic_status,
        };

        Ok(TransformedPipeline {
            graph: current_graph,
            report,
        })
    }

    /// Validate all structural transformation preconditions.
    ///
    /// Cycle detection is delegated to the canonical dependency
    /// analysis implementation.
    pub fn validate_preconditions(
        graph: &PipelineGraph,
        preconditions: &[TransformationPrecondition],
    ) -> Result<(), TransformationError> {
        for precondition in preconditions {
            match precondition {
                TransformationPrecondition::RequireAcyclic => {
                    let analysis = DependencyAnalysis::analyze(graph);

                    if analysis.has_cycles() {
                        return Err(
                            TransformationError::AcyclicGraphRequired,
                        );
                    }
                }

                TransformationPrecondition::RequireStage { stage } => {
                    if !graph.contains_node(stage) {
                        return Err(
                            TransformationError::RequiredStageMissing {
                                stage: stage.clone(),
                            },
                        );
                    }
                }

                TransformationPrecondition::RequireNonEmpty => {
                    if graph.is_empty() {
                        return Err(
                            TransformationError::NonEmptyGraphRequired,
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// GRAPH OWNERSHIP VALIDATION
// ============================================================================

fn validate_graph_ownership(
    definition: &PipelineDefinition,
    graph: &PipelineGraph,
) -> Result<(), TransformationError> {
    if definition.id() != graph.pipeline_id() {
        return Err(
            TransformationError::PipelineIdentityMismatch {
                definition: definition.id(),
                graph: graph.pipeline_id(),
            },
        );
    }

    let declared: BTreeSet<PipelineStageId> = definition
        .stages()
        .iter()
        .map(|stage| stage.id().clone())
        .collect();

    let graph_nodes: BTreeSet<PipelineStageId> =
        graph.nodes().cloned().collect();

    if declared != graph_nodes {
        return Err(
            TransformationError::StageSetMismatch,
        );
    }

    Ok(())
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_identity_component(
    field: &'static str,
    value: &str,
) -> Result<(), TransformationError> {
    if value.trim().is_empty() {
        return Err(
            TransformationError::EmptyIdentity {
                field,
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            TransformationError::InvalidIdentity {
                field,
            },
        );
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors raised by pipeline transformations.
#[derive(Debug)]
#[non_exhaustive]
pub enum TransformationError {
    /// An identity component was empty.
    EmptyIdentity {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// An identity component contained control characters.
    InvalidIdentity {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// The definition and graph identify different pipelines.
    PipelineIdentityMismatch {
        /// Definition identity.
        definition: crate::ai::ids::AiPipelineId,

        /// Graph identity.
        graph: crate::ai::ids::AiPipelineId,
    },

    /// Graph nodes do not match the definition's stage declarations.
    StageSetMismatch,

    /// A graph edit contains an invalid combination of ports and edge kind.
    InvalidEdgeShape,

    /// An edit attempts to add an existing edge.
    DuplicateEdit {
        /// Duplicate edge selector.
        edge: PipelineEdgeSelector,
    },

    /// An edit attempts to remove an edge that does not exist.
    EdgeNotFound {
        /// Missing edge selector.
        edge: PipelineEdgeSelector,
    },

    /// A transformation requires an acyclic graph.
    AcyclicGraphRequired,

    /// A required stage is absent.
    RequiredStageMissing {
        /// Missing stage identity.
        stage: PipelineStageId,
    },

    /// A transformation requires a nonempty graph.
    NonEmptyGraphRequired,

    /// Canonical graph construction rejected an edit.
    Graph(PipelineGraphError),
}

impl From<PipelineGraphError> for TransformationError {
    fn from(error: PipelineGraphError) -> Self {
        Self::Graph(error)
    }
}

impl fmt::Display for TransformationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentity { field } => {
                write!(f, "{field} must not be empty")
            }

            Self::InvalidIdentity { field } => {
                write!(
                    f,
                    "{field} contains a control character"
                )
            }

            Self::PipelineIdentityMismatch {
                definition,
                graph,
            } => {
                write!(
                    f,
                    "pipeline identity mismatch: definition={definition}, graph={graph}"
                )
            }

            Self::StageSetMismatch => {
                f.write_str(
                    "pipeline graph nodes do not match declared pipeline stages",
                )
            }

            Self::InvalidEdgeShape => {
                f.write_str(
                    "pipeline edge kind and endpoint ports are incompatible",
                )
            }

            Self::DuplicateEdit { edge } => {
                write!(
                    f,
                    "transformation attempts to add an existing edge: \
                     {} -> {}",
                    edge.source(),
                    edge.target(),
                )
            }

            Self::EdgeNotFound { edge } => {
                write!(
                    f,
                    "transformation attempts to remove a nonexistent edge: \
                     {} -> {}",
                    edge.source(),
                    edge.target(),
                )
            }

            Self::AcyclicGraphRequired => {
                f.write_str(
                    "transformation requires an acyclic pipeline graph",
                )
            }

            Self::RequiredStageMissing { stage } => {
                write!(
                    f,
                    "transformation requires missing stage '{stage}'"
                )
            }

            Self::NonEmptyGraphRequired => {
                f.write_str(
                    "transformation requires a nonempty pipeline graph",
                )
            }

            Self::Graph(error) => {
                write!(
                    f,
                    "pipeline graph reconstruction failed: {error}"
                )
            }
        }
    }
}

impl Error for TransformationError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Graph(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::AiPipelineId;

    use crate::ai::pipeline::definition::{
        PipelineDefinitionBuilder,
        PipelineOperationRef,
        PipelinePort,
        PipelinePortDirection,
        PipelineStageDefinition,
    };

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value)
            .expect("valid stage identity")
    }

    fn stage(value: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(value),
            PipelineOperationRef::new(
                "zamani.ai",
                "operation",
            )
            .expect("valid operation"),
        )
    }

    fn fixture() -> (
        PipelineDefinition,
        PipelineGraph,
    ) {
        let mut builder = PipelineDefinitionBuilder::new(
            AiPipelineId::new(1)
                .expect("valid pipeline identity"),
            "test_pipeline",
        )
        .expect("valid definition");

        let mut source = stage("source");

        source
            .add_port(
                PipelinePort::new(
                    "output",
                    PipelinePortDirection::Output,
                    Some("tensor".to_owned()),
                )
                .expect("valid output port"),
            )
            .expect("port insertion succeeds");

        let mut target = stage("target");

        target
            .add_port(
                PipelinePort::new(
                    "input",
                    PipelinePortDirection::Input,
                    Some("tensor".to_owned()),
                )
                .expect("valid input port"),
            )
            .expect("port insertion succeeds");

        builder
            .add_stage(source)
            .expect("source stage succeeds");

        builder
            .add_stage(target)
            .expect("target stage succeeds");

        let definition = builder.build();

        let graph = PipelineGraph::builder(&definition)
            .build();

        (definition, graph)
    }

    fn transformation(
        name: &str,
    ) -> PipelineTransformation {
        PipelineTransformation::new(
            TransformationId::new(
                "zamani.ai.pipeline",
                name,
            )
            .expect("valid transformation identity"),
            TransformationVersion::new(1, 0, 0),
            "test transformation",
        )
    }

    #[test]
    fn transformation_identity_is_validated() {
        assert!(
            TransformationId::new(
                "zamani.ai",
                "add_dependency",
            )
            .is_ok()
        );

        assert!(
            TransformationId::new(
                "",
                "add_dependency",
            )
            .is_err()
        );

        assert!(
            TransformationId::new(
                "zamani.ai",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn transformation_version_is_stable() {
        let version =
            TransformationVersion::new(2, 4, 7);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 4);
        assert_eq!(version.patch(), 7);
    }

    #[test]
    fn dependency_addition_reconstructs_graph() {
        let (definition, graph) = fixture();

        let mut pass = transformation("add_dependency");

        pass.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("transformation succeeds");

        assert_eq!(result.graph().node_count(), 2);
        assert_eq!(result.graph().edge_count(), 1);

        assert_eq!(graph.edge_count(), 0);

        assert_eq!(
            result.report().semantic_status(),
            TransformationSemanticStatus::RequiresRevalidation
        );
    }

    #[test]
    fn control_dependency_is_preserved() {
        let (definition, graph) = fixture();

        let mut pass = transformation("add_control");

        pass.add_edit(
            PipelineGraphEdit::add_control_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("control transformation succeeds");

        assert_eq!(result.graph().edge_count(), 1);

        assert_eq!(
            result.graph().edges().next().unwrap().kind(),
            PipelineEdgeKind::Control
        );
    }

    #[test]
    fn data_flow_is_validated_by_canonical_graph_builder() {
        let (definition, graph) = fixture();

        let mut pass = transformation("add_data_flow");

        pass.add_edit(
            PipelineGraphEdit::add_data_flow(
                stage_id("source"),
                "output",
                stage_id("target"),
                "input",
            ),
        );

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("data-flow transformation succeeds");

        assert_eq!(result.graph().edge_count(), 1);

        let edge = result.graph().edges().next().unwrap();

        assert_eq!(
            edge.kind(),
            PipelineEdgeKind::DataFlow
        );

        assert_eq!(
            edge.source().port_name(),
            Some("output")
        );

        assert_eq!(
            edge.target().port_name(),
            Some("input")
        );
    }

    #[test]
    fn invalid_data_flow_is_rejected() {
        let (definition, graph) = fixture();

        let mut pass = transformation("invalid_data_flow");

        pass.add_edit(
            PipelineGraphEdit::add_data_flow(
                stage_id("source"),
                "missing",
                stage_id("target"),
                "input",
            ),
        );

        assert!(
            PipelineTransformationEngine::apply(
                &definition,
                &graph,
                &pass,
            )
            .is_err()
        );

        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let (definition, graph) = fixture();

        let mut pass = transformation("duplicate");

        let source = stage_id("source");
        let target = stage_id("target");

        pass.add_edit(
            PipelineGraphEdit::add_dependency(
                source.clone(),
                target.clone(),
            ),
        );

        pass.add_edit(
            PipelineGraphEdit::add_dependency(
                source,
                target,
            ),
        );

        assert!(
            matches!(
                PipelineTransformationEngine::apply(
                    &definition,
                    &graph,
                    &pass,
                ),
                Err(TransformationError::DuplicateEdit { .. })
            )
        );
    }

    #[test]
    fn existing_edges_can_be_removed_exactly() {
        let (definition, empty_graph) = fixture();

        let mut add = transformation("add");

        add.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let populated =
            PipelineTransformationEngine::apply(
                &definition,
                &empty_graph,
                &add,
            )
            .expect("edge addition succeeds")
            .into_graph();

        let mut remove = transformation("remove");

        remove.add_edit(
            PipelineGraphEdit::remove_edge(
                PipelineEdgeSelector::dependency(
                    stage_id("source"),
                    stage_id("target"),
                ),
            ),
        );

        let result = PipelineTransformationEngine::apply(
            &definition,
            &populated,
            &remove,
        )
        .expect("edge removal succeeds");

        assert_eq!(result.graph().edge_count(), 0);
        assert_eq!(populated.edge_count(), 1);
        assert_eq!(result.report().total_removals(), 1);
    }

    #[test]
    fn removing_nonexistent_edge_fails() {
        let (definition, graph) = fixture();

        let mut pass = transformation("remove_missing");

        pass.add_edit(
            PipelineGraphEdit::remove_edge(
                PipelineEdgeSelector::dependency(
                    stage_id("source"),
                    stage_id("target"),
                ),
            ),
        );

        assert!(
            matches!(
                PipelineTransformationEngine::apply(
                    &definition,
                    &graph,
                    &pass,
                ),
                Err(TransformationError::EdgeNotFound { .. })
            )
        );
    }

    #[test]
    fn acyclic_precondition_uses_canonical_dependency_analysis() {
        let (definition, graph) = fixture();

        let mut pass = transformation("acyclic");

        pass.require(
            TransformationPrecondition::RequireAcyclic,
        );

        pass.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        assert!(
            PipelineTransformationEngine::apply(
                &definition,
                &graph,
                &pass,
            )
            .is_ok()
        );
    }

    #[test]
    fn cyclic_graph_fails_acyclic_precondition() {
        let (definition, graph) = fixture();

        let mut create_cycle = transformation("cycle");

        create_cycle.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        create_cycle.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("target"),
                stage_id("source"),
            ),
        );

        let cyclic =
            PipelineTransformationEngine::apply(
                &definition,
                &graph,
                &create_cycle,
            )
            .expect("structural graph accepts cycles")
            .into_graph();

        let mut require_acyclic =
            transformation("require_acyclic");

        require_acyclic.require(
            TransformationPrecondition::RequireAcyclic,
        );

        assert!(
            matches!(
                PipelineTransformationEngine::apply(
                    &definition,
                    &cyclic,
                    &require_acyclic,
                ),
                Err(TransformationError::AcyclicGraphRequired)
            )
        );
    }

    #[test]
    fn missing_required_stage_fails() {
        let (definition, graph) = fixture();

        let mut pass = transformation("missing_stage");

        pass.require(
            TransformationPrecondition::RequireStage {
                stage: stage_id("unknown"),
            },
        );

        assert!(
            matches!(
                PipelineTransformationEngine::apply(
                    &definition,
                    &graph,
                    &pass,
                ),
                Err(
                    TransformationError::RequiredStageMissing { .. }
                )
            )
        );
    }

    #[test]
    fn mismatched_pipeline_identity_is_rejected() {
        let (definition, graph) = fixture();

        let mut other_builder =
            PipelineDefinitionBuilder::new(
                AiPipelineId::new(2)
                    .expect("valid pipeline identity"),
                "other_pipeline",
            )
            .expect("valid definition");

        other_builder
            .add_stage(stage("source"))
            .expect("stage succeeds");

        other_builder
            .add_stage(stage("target"))
            .expect("stage succeeds");

        let other_definition = other_builder.build();

        let pass = transformation("identity");

        assert!(
            matches!(
                PipelineTransformationEngine::apply(
                    &other_definition,
                    &graph,
                    &pass,
                ),
                Err(
                    TransformationError::PipelineIdentityMismatch { .. }
                )
            )
        );
    }

    #[test]
    fn transformation_sequences_preserve_order() {
        let (definition, graph) = fixture();

        let mut first = transformation("first");

        first.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let mut second = transformation("second");

        second.add_edit(
            PipelineGraphEdit::add_control_dependency(
                stage_id("target"),
                stage_id("source"),
            ),
        );

        let transformations = vec![
            first,
            second,
        ];

        let result =
            PipelineTransformationEngine::apply_sequence(
                &definition,
                &graph,
                &transformations,
            )
            .expect("sequence succeeds");

        assert_eq!(result.graph().edge_count(), 2);

        assert_eq!(result.report().steps().len(), 2);

        assert_eq!(
            result.report().steps()[0].id().name(),
            "first"
        );

        assert_eq!(
            result.report().steps()[1].id().name(),
            "second"
        );
    }

    #[test]
    fn failed_sequence_does_not_mutate_original_graph() {
        let (definition, graph) = fixture();

        let mut first = transformation("first");

        first.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let mut second = transformation("second");

        second.add_edit(
            PipelineGraphEdit::remove_edge(
                PipelineEdgeSelector::dependency(
                    stage_id("target"),
                    stage_id("source"),
                ),
            ),
        );

        let result =
            PipelineTransformationEngine::apply_sequence(
                &definition,
                &graph,
                &[first, second],
            );

        assert!(result.is_err());
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn empty_transformation_is_a_valid_identity() {
        let (definition, graph) = fixture();

        let pass = transformation("identity");

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("identity transformation succeeds");

        assert_eq!(
            result.graph(),
            &graph
        );

        assert_eq!(
            result.report().resulting_edge_count(),
            0
        );
    }

    #[test]
    fn empty_transformation_sequence_preserves_graph() {
        let (definition, graph) = fixture();

        let result =
            PipelineTransformationEngine::apply_sequence(
                &definition,
                &graph,
                &[],
            )
            .expect("empty sequence succeeds");

        assert_eq!(result.graph(), &graph);
        assert!(result.report().steps().is_empty());
    }

    #[test]
    fn graph_nodes_are_preserved_by_transformations() {
        let (definition, graph) = fixture();

        let mut pass = transformation("add");

        pass.add_edit(
            PipelineGraphEdit::add_dependency(
                stage_id("source"),
                stage_id("target"),
            ),
        );

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("transformation succeeds");

        let original: BTreeSet<_> =
            graph.nodes().cloned().collect();

        let transformed: BTreeSet<_> =
            result.graph().nodes().cloned().collect();

        assert_eq!(original, transformed);
    }

    #[test]
    fn every_result_requires_semantic_revalidation() {
        let (definition, graph) = fixture();

        let pass = transformation("identity");

        let result = PipelineTransformationEngine::apply(
            &definition,
            &graph,
            &pass,
        )
        .expect("transformation succeeds");

        assert_eq!(
            result.report().semantic_status(),
            TransformationSemanticStatus::RequiresRevalidation
        );
    }
}
