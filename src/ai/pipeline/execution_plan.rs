
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI — Abstract Pipeline Execution Plans
//!
//! Path: `src/ai/pipeline/execution_plan.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Construct a deterministic, target-independent execution-plan
//! representation from an existing pipeline definition and its canonical
//! pipeline graph.
//!
//! The plan preserves:
//!
//! - the owning pipeline identity;
//! - the pipeline definition version;
//! - stage operation references;
//! - declared stage ports;
//! - direct dependencies;
//! - direct dependents;
//! - all original logical graph edges;
//! - strongly connected components;
//! - deterministic dependency precedence when the graph is acyclic;
//! - the distinction between acyclic dependency order and cyclic graphs.
//!
//! This module describes how a pipeline's logical computation is related.
//! It does not execute stages or assign physical resources.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - abstract pipeline execution-plan data;
//! - stage-level plan descriptions;
//! - deterministic plan construction;
//! - dependency-order representation;
//! - cyclic-component preservation;
//! - plan construction errors;
//! - plan-to-definition and plan-to-graph consistency checks.
//!
//! It does not own:
//!
//! - pipeline definitions;
//! - pipeline graph storage;
//! - graph transformations;
//! - dependency analysis algorithms;
//! - source AST construction;
//! - grammar interpretation;
//! - name resolution;
//! - semantic type checking;
//! - AI operation registration;
//! - canonical semantic validation;
//! - ZUIR operation definitions;
//! - tensor storage;
//! - optimization legality;
//! - resource feasibility;
//! - physical memory allocation;
//! - hardware discovery;
//! - device selection;
//! - placement;
//! - scheduling;
//! - distributed coordination;
//! - runtime execution.
//!
//! ## Canonical authorities
//!
//! `definition.rs`
//!     Owns pipeline definitions, stages, operations, and ports.
//!
//! `graph.rs`
//!     Owns the canonical logical graph and its edges.
//!
//! `dependencies.rs`
//!     Owns dependency analysis, topological ordering, and cycle detection.
//!
//! `transformations.rs`
//!     Owns logical graph transformations. Transformed graphs require
//!     semantic revalidation before being treated as valid programs.
//!
//! `validation.rs`
//!     Owns pipeline validation coordination and semantic validation
//!     integration. This plan does not replace that authority.
//!
//! `frontend::semantic::domains::ai`
//!     Owns AI semantic validation.
//!
//! `zuir::dialects::ai`
//!     Owns universal AI operation representations.
//!
//! `distributed`
//!     Owns distributed placement, scheduling, and coordination.
//!
//! `runtime`
//!     Owns actual execution.
//!
//! ## Plan semantics
//!
//! A dependency order is not a runtime schedule.
//!
//! If the dependency graph is acyclic, the plan preserves the deterministic
//! topological order produced by `DependencyAnalysis`.
//!
//! That order means only that dependency predecessors must be respected.
//! Independent stages may be realized concurrently or in another order
//! permitted by their complete semantic contracts.
//!
//! If the graph contains cycles, the plan preserves its strongly connected
//! components and does not fabricate a topological order.
//!
//! Cycle legality belongs to semantic validation. A cyclic component may
//! represent a legal feedback or iterative construct, or it may be invalid.
//! This module does not decide which.
//!
//! ## Validation boundary
//!
//! Constructing a plan does not prove that the pipeline is semantically
//! valid, type-correct, capability-compatible, secure, or executable.
//!
//! A plan constructed here always requires the canonical semantic
//! validation and downstream feasibility checks appropriate to its use.
//!
//! Consumers must not interpret successful plan construction as permission
//! to execute the pipeline.
//!
//! ## POCO-REAF and scalability
//!
//! The plan contains no:
//!
//! - CPU identifiers;
//! - GPU identifiers;
//! - QPU identifiers;
//! - FPGA identifiers;
//! - accelerator assignments;
//! - vendor-specific execution handles;
//! - physical memory addresses;
//! - worker assignments;
//! - network routes;
//! - fixed machine sizes;
//! - fixed pipeline-stage limits;
//! - fixed graph-depth limits;
//! - fixed dependency-count limits.
//!
//! Dynamic collections are used throughout.
//!
//! No artificial language-level maximum is imposed on pipeline stages,
//! graph edges, dependencies, ports, or strongly connected components.
//!
//! Actual capacity remains bounded by the process address space, allocator,
//! available memory, implementation limits, and explicitly configured
//! operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! Operational resource limits must not silently redefine program meaning.
//!
//! ## Determinism
//!
//! - The canonical graph provides deterministic edge ordering.
//! - Dependency analysis provides deterministic topological ordering.
//! - Cyclic components retain the canonical analysis ordering.
//! - Stage plan ordering follows dependency order when available.
//! - Stage plan ordering falls back to stable stage identity ordering when
//!   no topological order exists.
//! - No hash-map iteration order affects plan construction.
//!
//! ## Freeze criteria
//!
//! - Uses canonical pipeline definitions.
//! - Uses canonical pipeline graphs.
//! - Uses canonical dependency analysis.
//! - Detects mismatched pipeline identities.
//! - Detects missing and unexpected graph nodes.
//! - Preserves every graph edge.
//! - Preserves cyclic components.
//! - Never fabricates a topological order for a cyclic graph.
//! - Does not claim semantic validity.
//! - Does not perform physical scheduling or placement.
//! - Does not impose arbitrary semantic ceilings.
//! - Contains no unsafe Rust.
//! - Construction and error paths have tests.

use crate::ai::pipeline::definition::{
    PipelineDefinition,
    PipelineDefinitionVersion,
    PipelineOperationRef,
    PipelinePort,
    PipelineStageId,
};

use crate::ai::pipeline::dependencies::DependencyAnalysis;

use crate::ai::pipeline::graph::{
    PipelineEdge,
    PipelineGraph,
};

use crate::ai::ids::AiPipelineId;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

// ============================================================================
// DEPENDENCY ORDER
// ============================================================================

/// The dependency-order information preserved by an abstract plan.
///
/// This describes logical precedence only. It is not a physical execution
/// schedule and does not prescribe a particular degree of concurrency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipelineDependencyOrder {
    /// The dependency graph is acyclic.
    ///
    /// `stages` contains the deterministic topological order supplied by
    /// the canonical dependency analyzer.
    ///
    /// Independent stages may still be executed concurrently where their
    /// complete semantic contracts permit it.
    Acyclic {
        /// Deterministic dependency precedence.
        stages: Vec<PipelineStageId>,
    },

    /// The dependency graph contains one or more cyclic components.
    ///
    /// No topological order is claimed.
    ///
    /// The semantic validator must decide whether the cycles are legal.
    Cyclic {
        /// Deterministically ordered strongly connected components.
        components: Vec<Vec<PipelineStageId>>,
    },
}

impl PipelineDependencyOrder {
    /// Return whether a topological order exists.
    #[must_use]
    pub const fn is_acyclic(&self) -> bool {
        matches!(self, Self::Acyclic { .. })
    }

    /// Return whether the dependency graph contains cycles.
    #[must_use]
    pub const fn is_cyclic(&self) -> bool {
        matches!(self, Self::Cyclic { .. })
    }

    /// Return the topological order when the graph is acyclic.
    ///
    /// Returns `None` when the graph contains cycles.
    #[must_use]
    pub fn topological_order(&self) -> Option<&[PipelineStageId]> {
        match self {
            Self::Acyclic { stages } => Some(stages),
            Self::Cyclic { .. } => None,
        }
    }

    /// Return cyclic components when the graph contains cycles.
    ///
    /// Returns `None` when the graph is acyclic.
    #[must_use]
    pub fn cyclic_components(
        &self,
    ) -> Option<&[Vec<PipelineStageId>]> {
        match self {
            Self::Acyclic { .. } => None,
            Self::Cyclic { components } => Some(components),
        }
    }
}

// ============================================================================
// STAGE PLAN
// ============================================================================

/// An immutable abstract description of one pipeline stage.
///
/// This structure references the canonical stage identity and operation
/// declaration. It does not contain an execution closure, device handle,
/// physical allocation, or runtime task.
///
/// The copied port declarations preserve the stage's declared interface.
/// Their semantic type and contract interpretation remain owned by the
/// canonical semantic system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineStagePlan {
    id: PipelineStageId,
    operation: PipelineOperationRef,
    ports: Vec<PipelinePort>,
    dependencies: Vec<PipelineStageId>,
    dependents: Vec<PipelineStageId>,
}

impl PipelineStagePlan {
    /// Return the canonical stage identity.
    #[must_use]
    pub fn id(&self) -> &PipelineStageId {
        &self.id
    }

    /// Return the canonical operation reference.
    #[must_use]
    pub fn operation(&self) -> &PipelineOperationRef {
        &self.operation
    }

    /// Return the declared ports in their original declaration order.
    #[must_use]
    pub fn ports(&self) -> &[PipelinePort] {
        &self.ports
    }

    /// Return the stage's direct logical predecessors.
    ///
    /// These are dependencies, not necessarily data-flow producers.
    #[must_use]
    pub fn dependencies(&self) -> &[PipelineStageId] {
        &self.dependencies
    }

    /// Return the stage's direct logical dependents.
    #[must_use]
    pub fn dependents(&self) -> &[PipelineStageId] {
        &self.dependents
    }

    /// Return the number of direct dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Return the number of direct dependents.
    #[must_use]
    pub fn dependent_count(&self) -> usize {
        self.dependents.len()
    }
}

// ============================================================================
// EXECUTION PLAN
// ============================================================================

/// An immutable, target-independent abstract AI pipeline execution plan.
///
/// This is a derived representation of an existing pipeline definition
/// and graph. It does not replace either canonical structure.
///
/// A plan is not a semantic validation certificate, a physical schedule,
/// or an authorization to execute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineExecutionPlan {
    pipeline_id: AiPipelineId,
    definition_version: PipelineDefinitionVersion,
    stages: Vec<PipelineStagePlan>,
    edges: Vec<PipelineEdge>,
    dependency_order: PipelineDependencyOrder,
    dependency_analysis: DependencyAnalysis,
}

impl PipelineExecutionPlan {
    /// Construct an abstract plan from a canonical pipeline definition
    /// and its canonical graph.
    ///
    /// The definition and graph must describe the same pipeline and contain
    /// exactly the same stage identities.
    ///
    /// The canonical dependency analyzer determines dependency order and
    /// cyclic components.
    ///
    /// This function does not perform complete semantic validation.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the graph belongs to another pipeline;
    /// - the graph omits a declared stage;
    /// - the graph contains an undeclared stage;
    /// - the derived dependency analysis belongs to another pipeline.
    ///
    /// A cyclic graph is not itself a construction error. Its components
    /// are preserved for the canonical semantic validator.
    pub fn build(
        definition: &PipelineDefinition,
        graph: &PipelineGraph,
    ) -> Result<Self, PipelineExecutionPlanError> {
        if definition.id() != graph.pipeline_id() {
            return Err(PipelineExecutionPlanError::PipelineIdentityMismatch {
                definition: definition.id(),
                graph: graph.pipeline_id(),
            });
        }

        let definition_stages: BTreeSet<PipelineStageId> = definition
            .stages()
            .iter()
            .map(|stage| stage.id().clone())
            .collect();

        let graph_stages: BTreeSet<PipelineStageId> = graph
            .nodes()
            .cloned()
            .collect();

        let missing_stages: Vec<PipelineStageId> = definition_stages
            .difference(&graph_stages)
            .cloned()
            .collect();

        let unexpected_stages: Vec<PipelineStageId> = graph_stages
            .difference(&definition_stages)
            .cloned()
            .collect();

        if !missing_stages.is_empty() || !unexpected_stages.is_empty() {
            return Err(
                PipelineExecutionPlanError::StageSetMismatch {
                    missing: missing_stages,
                    unexpected: unexpected_stages,
                },
            );
        }

        let dependency_analysis = DependencyAnalysis::analyze(graph);

        if dependency_analysis.pipeline_id() != definition.id() {
            return Err(
                PipelineExecutionPlanError::DependencyAnalysisIdentityMismatch {
                    definition: definition.id(),
                    analysis: dependency_analysis.pipeline_id(),
                },
            );
        }

        let dependency_order = match dependency_analysis.topological_order() {
            Some(order) => PipelineDependencyOrder::Acyclic {
                stages: order.to_vec(),
            },

            None => PipelineDependencyOrder::Cyclic {
                components: dependency_analysis
                    .cyclic_components()
                    .to_vec(),
            },
        };

        let stage_order: Vec<PipelineStageId> =
            match &dependency_order {
                PipelineDependencyOrder::Acyclic { stages } => {
                    stages.clone()
                }

                PipelineDependencyOrder::Cyclic { .. } => {
                    definition_stages.iter().cloned().collect()
                }
            };

        let mut stages = Vec::with_capacity(stage_order.len());

        for stage_id in stage_order {
            let definition_stage = definition
                .stage(&stage_id)
                .ok_or_else(|| {
                    PipelineExecutionPlanError::MissingDefinitionStage {
                        stage: stage_id.clone(),
                    }
                })?;

            let dependencies = dependency_analysis
                .direct_dependencies(&stage_id)
                .ok_or_else(|| {
                    PipelineExecutionPlanError::MissingDependencyEntry {
                        stage: stage_id.clone(),
                    }
                })?
                .iter()
                .cloned()
                .collect();

            let dependents = dependency_analysis
                .direct_dependents(&stage_id)
                .ok_or_else(|| {
                    PipelineExecutionPlanError::MissingDependentEntry {
                        stage: stage_id.clone(),
                    }
                })?
                .iter()
                .cloned()
                .collect();

            stages.push(PipelineStagePlan {
                id: stage_id,
                operation: definition_stage.operation().clone(),
                ports: definition_stage.ports().to_vec(),
                dependencies,
                dependents,
            });
        }

        // Preserve the exact canonical graph edges. Do not reconstruct
        // or normalize them: edge kinds and port connections are semantic
        // information owned by PipelineGraph.
        let edges = graph.edges().cloned().collect();

        Ok(Self {
            pipeline_id: definition.id(),
            definition_version: definition.version(),
            stages,
            edges,
            dependency_order,
            dependency_analysis,
        })
    }

    /// Return the canonical pipeline identity.
    #[must_use]
    pub const fn pipeline_id(&self) -> AiPipelineId {
        self.pipeline_id
    }

    /// Return the pipeline definition version.
    #[must_use]
    pub const fn definition_version(&self) -> PipelineDefinitionVersion {
        self.definition_version
    }

    /// Return stage plans in the plan's deterministic representation order.
    ///
    /// For acyclic graphs this is topological dependency order.
    ///
    /// For cyclic graphs this is stable stage-identity order. It must not
    /// be interpreted as execution precedence.
    #[must_use]
    pub fn stages(&self) -> &[PipelineStagePlan] {
        &self.stages
    }

    /// Return all original graph edges in canonical deterministic order.
    #[must_use]
    pub fn edges(&self) -> &[PipelineEdge] {
        &self.edges
    }

    /// Return the dependency-order description.
    #[must_use]
    pub fn dependency_order(&self) -> &PipelineDependencyOrder {
        &self.dependency_order
    }

    /// Return the canonical dependency analysis used to construct the plan.
    ///
    /// This is derived analysis, not a second graph representation.
    #[must_use]
    pub const fn dependency_analysis(&self) -> &DependencyAnalysis {
        &self.dependency_analysis
    }

    /// Return the number of planned stages.
    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Return the number of preserved graph edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return whether the plan contains no stages.
    ///
    /// Whether an empty pipeline is legal is determined by the language
    /// specification and canonical semantic validation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }

    /// Return whether this plan requires semantic validation.
    ///
    /// This method deliberately always returns `true`: successful plan
    /// construction is not proof of semantic validity.
    #[must_use]
    pub const fn requires_semantic_validation(&self) -> bool {
        true
    }

    /// Return whether the plan contains cyclic dependencies.
    ///
    /// This is a structural observation, not a legality decision.
    #[must_use]
    pub fn has_cycles(&self) -> bool {
        self.dependency_order.is_cyclic()
    }

    /// Find a stage plan by its canonical local identity.
    #[must_use]
    pub fn stage(
        &self,
        id: &PipelineStageId,
    ) -> Option<&PipelineStagePlan> {
        self.stages.iter().find(|stage| stage.id() == id)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors raised while constructing an abstract pipeline execution plan.
///
/// These errors describe inconsistencies between canonical pipeline
/// structures. They do not replace semantic, feasibility, or runtime errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PipelineExecutionPlanError {
    /// The graph and definition identify different pipelines.
    PipelineIdentityMismatch {
        /// Identity declared by the pipeline definition.
        definition: AiPipelineId,

        /// Identity declared by the graph.
        graph: AiPipelineId,
    },

    /// The graph does not contain exactly the definition's stage set.
    StageSetMismatch {
        /// Stages declared in the definition but absent from the graph.
        missing: Vec<PipelineStageId>,

        /// Stages present in the graph but absent from the definition.
        unexpected: Vec<PipelineStageId>,
    },

    /// A stage referenced by the derived order was not in the definition.
    MissingDefinitionStage {
        /// The missing stage identity.
        stage: PipelineStageId,
    },

    /// Dependency analysis did not contain a declared stage.
    MissingDependencyEntry {
        /// The stage whose dependency entry was missing.
        stage: PipelineStageId,
    },

    /// Dependency analysis did not contain a declared dependent entry.
    MissingDependentEntry {
        /// The stage whose dependent entry was missing.
        stage: PipelineStageId,
    },

    /// Dependency analysis unexpectedly belonged to another pipeline.
    DependencyAnalysisIdentityMismatch {
        /// Identity declared by the definition.
        definition: AiPipelineId,

        /// Identity returned by dependency analysis.
        analysis: AiPipelineId,
    },
}

impl fmt::Display for PipelineExecutionPlanError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::PipelineIdentityMismatch {
                definition,
                graph,
            } => write!(
                formatter,
                "pipeline identity mismatch: definition is {definition}, graph is {graph}"
            ),

            Self::StageSetMismatch {
                missing,
                unexpected,
            } => write!(
                formatter,
                "pipeline graph stage set does not match its definition: \
                 {} missing stage(s), {} unexpected stage(s)",
                missing.len(),
                unexpected.len()
            ),

            Self::MissingDefinitionStage { stage } => write!(
                formatter,
                "pipeline plan references undeclared stage '{stage}'"
            ),

            Self::MissingDependencyEntry { stage } => write!(
                formatter,
                "dependency analysis has no entry for stage '{stage}'"
            ),

            Self::MissingDependentEntry { stage } => write!(
                formatter,
                "dependency analysis has no dependent entry for stage '{stage}'"
            ),

            Self::DependencyAnalysisIdentityMismatch {
                definition,
                analysis,
            } => write!(
                formatter,
                "dependency analysis identity mismatch: \
                 definition is {definition}, analysis is {analysis}"
            ),
        }
    }
}

impl Error for PipelineExecutionPlanError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::pipeline::definition::{
        PipelineDefinitionBuilder,
        PipelineOperationRef,
        PipelineStageDefinition,
        PipelineStageId,
    };

    use crate::ai::pipeline::graph::PipelineGraph;

    fn pipeline_id(value: u128) -> AiPipelineId {
        AiPipelineId::new(value).expect("nonzero pipeline identity")
    }

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value).expect("valid stage identity")
    }

    fn operation(name: &str) -> PipelineOperationRef {
        PipelineOperationRef::new("zamani.ai", name)
            .expect("valid operation reference")
    }

    fn stage(name: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(name),
            operation(name),
        )
    }

    fn definition(
        id: u128,
        stage_names: &[&str],
    ) -> PipelineDefinition {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(id),
            "test_pipeline",
        )
        .expect("valid pipeline definition");

        for name in stage_names {
            builder
                .add_stage(stage(name))
                .expect("unique stage identity");
        }

        builder.build()
    }

    #[test]
    fn builds_plan_for_empty_pipeline() {
        let definition = definition(1, &[]);
        let graph = PipelineGraph::builder(&definition).build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("empty plan is structurally representable");

        assert!(plan.is_empty());
        assert_eq!(plan.stage_count(), 0);
        assert_eq!(plan.edge_count(), 0);
        assert!(plan.dependency_order().is_acyclic());
        assert!(plan.requires_semantic_validation());
    }

    #[test]
    fn builds_plan_for_single_stage() {
        let definition = definition(2, &["source"]);
        let graph = PipelineGraph::builder(&definition).build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("single-stage plan");

        assert_eq!(plan.stage_count(), 1);
        assert_eq!(plan.stages()[0].id(), &stage_id("source"));
        assert_eq!(
            plan.stages()[0].operation().name(),
            "source"
        );
        assert!(plan.dependency_order().is_acyclic());
    }

    #[test]
    fn preserves_topological_dependency_order() {
        let definition = definition(
            3,
            &["source", "transform", "sink"],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(
                &stage_id("source"),
                &stage_id("transform"),
            )
            .expect("valid dependency");

        builder
            .add_dependency(
                &stage_id("transform"),
                &stage_id("sink"),
            )
            .expect("valid dependency");

        let graph = builder.build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid abstract plan");

        let order = plan
            .dependency_order()
            .topological_order()
            .expect("acyclic graph");

        assert_eq!(
            order,
            &[
                stage_id("source"),
                stage_id("transform"),
                stage_id("sink"),
            ]
        );
    }

    #[test]
    fn preserves_direct_dependencies_and_dependents() {
        let definition = definition(
            4,
            &["source", "transform", "sink"],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(
                &stage_id("source"),
                &stage_id("transform"),
            )
            .expect("valid dependency");

        builder
            .add_dependency(
                &stage_id("transform"),
                &stage_id("sink"),
            )
            .expect("valid dependency");

        let graph = builder.build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid plan");

        let transform = plan
            .stage(&stage_id("transform"))
            .expect("transform stage exists");

        assert_eq!(
            transform.dependencies(),
            &[stage_id("source")]
        );

        assert_eq!(
            transform.dependents(),
            &[stage_id("sink")]
        );
    }

    #[test]
    fn preserves_cycles_without_fabricating_topological_order() {
        let definition = definition(
            5,
            &["first", "second"],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(
                &stage_id("first"),
                &stage_id("second"),
            )
            .expect("valid dependency");

        builder
            .add_dependency(
                &stage_id("second"),
                &stage_id("first"),
            )
            .expect("valid dependency");

        let graph = builder.build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("cycles are representable");

        assert!(plan.has_cycles());
        assert!(plan.dependency_order().is_cyclic());
        assert!(
            plan.dependency_order()
                .topological_order()
                .is_none()
        );

        assert_eq!(
            plan.dependency_order()
                .cyclic_components()
                .expect("cyclic components")
                .len(),
            1
        );
    }

    #[test]
    fn rejects_a_graph_from_another_pipeline() {
        let definition = definition(6, &["source"]);
        let other_definition = definition(7, &["source"]);
        let graph = PipelineGraph::builder(&other_definition).build();

        let result = PipelineExecutionPlan::build(
            &definition,
            &graph,
        );

        assert!(matches!(
            result,
            Err(
                PipelineExecutionPlanError::PipelineIdentityMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn preserves_original_graph_edges() {
        let definition = definition(
            8,
            &["source", "sink"],
        );

        let mut builder = PipelineGraph::builder(&definition);

        builder
            .add_dependency(
                &stage_id("source"),
                &stage_id("sink"),
            )
            .expect("valid dependency");

        let graph = builder.build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid plan");

        assert_eq!(
            plan.edge_count(),
            graph.edge_count()
        );

        assert_eq!(
            plan.edges(),
            &graph.edges().cloned().collect::<Vec<_>>()
        );
    }

    #[test]
    fn disconnected_stages_are_preserved() {
        let definition = definition(
            9,
            &["alpha", "beta", "gamma"],
        );

        let graph = PipelineGraph::builder(&definition).build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("disconnected graph is representable");

        assert_eq!(plan.stage_count(), 3);
        assert_eq!(plan.edge_count(), 0);
    }

    #[test]
    fn independent_stages_have_no_invented_dependencies() {
        let definition = definition(
            10,
            &["alpha", "beta"],
        );

        let graph = PipelineGraph::builder(&definition).build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid plan");

        for stage in plan.stages() {
            assert!(stage.dependencies().is_empty());
            assert!(stage.dependents().is_empty());
        }
    }

    #[test]
    fn plan_does_not_claim_semantic_validation() {
        let definition = definition(11, &["source"]);
        let graph = PipelineGraph::builder(&definition).build();

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid structural plan");

        assert!(plan.requires_semantic_validation());
    }
}
