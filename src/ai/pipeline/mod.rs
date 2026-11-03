
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Pipeline Subsystem
//!
//! Path: `src/ai/pipeline/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the canonical composition root for the Zamani AI
//! pipeline subsystem.
//!
//! It exposes the existing pipeline definition, graph, dependency
//! analysis, transformation, validation and abstract execution-plan
//! implementations through one stable public boundary.
//!
//! A pipeline describes computational intent and logical relationships.
//! It does not determine physical execution.
//!
//! ## Architecture
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical frontend AST
//!      |
//!      v
//! AI semantic analysis
//!      |
//!      v
//! PipelineDefinition
//!      |
//!      v
//! PipelineGraph
//!      |
//!      +---------------------+
//!      |                     |
//!      v                     v
//! DependencyAnalysis   Transformations
//!      |                     |
//!      +----------+----------+
//!                 |
//!                 v
//!          PipelineValidation
//!                 |
//!                 v
//!        PipelineExecutionPlan
//!                 |
//!                 v
//!             ZUIR AI
//!                 |
//!                 v
//!       Optimization and lowering
//!                 |
//!                 v
//!       Distributed/runtime systems
//!                 |
//!                 v
//!          Target realization
//! ```
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Pipeline subsystem module composition.
//! - Stable public pipeline API exports.
//! - Pipeline subsystem architectural documentation.
//!
//! It does not own:
//!
//! - Pipeline definitions.
//! - Pipeline graph storage.
//! - Dependency algorithms.
//! - Graph transformations.
//! - Semantic validation.
//! - Execution-plan construction.
//! - Source AST construction.
//! - AI model execution.
//! - Tensor storage.
//! - Universal intermediate representation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Physical placement.
//! - Scheduling.
//! - Distributed coordination.
//! - Runtime execution.
//!
//! Those responsibilities belong to their respective modules.
//!
//! ## Canonical module ownership
//!
//! `definition`:
//!     Owns declarative pipeline and stage definitions.
//!
//! `graph`:
//!     Owns pipeline nodes, endpoints and logical edges.
//!
//! `dependencies`:
//!     Owns dependency analysis, topological ordering and cycle detection.
//!
//! `transformations`:
//!     Owns target-independent graph transformations.
//!
//! `validation`:
//!     Owns pipeline validation coordination and completeness reporting.
//!
//! `execution_plan`:
//!     Owns target-independent abstract execution-plan descriptions.
//!
//! ## Integration boundaries
//!
//! `crate::ai::ids`:
//!     Supplies canonical AI pipeline identities.
//!
//! `crate::frontend::semantic::domains::ai`:
//!     Remains the authority for AI source-level semantics.
//!
//! `crate::zuir::dialects::ai`:
//!     Owns universal AI IR operations.
//!
//! `crate::zuir::dialects::tensor`:
//!     Owns universal tensor IR operations.
//!
//! `crate::classical`:
//!     Supplies canonical classical computation facilities.
//!
//! `crate::quantum::ir`:
//!     Supplies canonical quantum computational representations.
//!
//! `crate::hdl::hdlir`:
//!     Supplies canonical hardware description representations.
//!
//! `crate::hybrid`:
//!     Owns cross-domain computational composition.
//!
//! `crate::distributed`:
//!     Owns distributed placement, scheduling, communication and recovery.
//!
//! `crate::runtime`:
//!     Owns actual execution.
//!
//! This module does not introduce dependencies on those implementation
//! details. Integration is performed through the appropriate domain,
//! compiler and execution interfaces.
//!
//! ## Grammar integration
//!
//! The canonical grammar composition root is:
//!
//! `grammar/ai/ai.g4`
//!
//! The pipeline grammar owner is:
//!
//! `grammar/ai/pipelines.g4`
//!
//! Grammar constructs must be translated through the canonical frontend
//! AST and semantic subsystem before being represented in the AI pipeline
//! domain.
//!
//! This module does not parse grammar productions or create another AST.
//!
//! ## POCO-REAF and scalability
//!
//! Pipeline semantics must remain independent of physical execution
//! environments.
//!
//! No universal maximum is introduced for:
//!
//! - Pipeline stages.
//! - Pipeline inputs and outputs.
//! - Graph nodes and edges.
//! - Dependencies.
//! - Pipeline depth.
//! - Branches and joins.
//! - Streams and batches.
//! - Distributed participants.
//! - Model sizes.
//! - Tensor dimensions.
//!
//! Existing implementations use dynamically sized collections.
//!
//! Actual execution remains constrained by available memory, address
//! space, allocator capacity, operational budgets, implementation
//! capabilities and physical feasibility.
//!
//! Such limitations must never silently change program semantics.
//!
//! This subsystem does not promise literal infinite computation or
//! storage.
//!
//! ## Determinism
//!
//! Each subsystem retains its established deterministic ordering
//! guarantees.
//!
//! This composition root does not impose an alternative ordering policy.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe code is permitted.
//!
//! ## Freeze criteria
//!
//! - Every existing pipeline module is registered.
//! - Existing public APIs are exported without duplication.
//! - Canonical identifier ownership is preserved.
//! - No competing graph or execution-plan representation is introduced.
//! - No hardware or runtime dependencies are introduced.
//! - Public module documentation is complete.
//! - Public API integration tests pass.

// ============================================================================
// PIPELINE MODULES
// ============================================================================

/// Declarative pipeline, stage, operation and port definitions.
pub mod definition;

/// Canonical logical pipeline graph and graph construction.
pub mod graph;

/// Dependency analysis, topological ordering and cycle detection.
pub mod dependencies;

/// Target-independent pipeline graph transformations.
pub mod transformations;

/// Pipeline structural and semantic validation coordination.
pub mod validation;

/// Target-independent abstract pipeline execution plans.
pub mod execution_plan;

// ============================================================================
// PUBLIC DEFINITION API
// ============================================================================

/// Public pipeline definition interfaces.
///
/// These types describe pipeline identity, versioning, stages,
/// operation references, ports and metadata.
pub use definition::{
    PipelineDefinition,
    PipelineDefinitionBuilder,
    PipelineDefinitionError,
    PipelineDefinitionVersion,
    PipelineMetadata,
    PipelineOperationRef,
    PipelinePort,
    PipelinePortDirection,
    PipelineStageDefinition,
    PipelineStageId,
    PipelineStageMetadata,
};

// ============================================================================
// PUBLIC GRAPH API
// ============================================================================

/// Public pipeline graph interfaces.
///
/// These types describe graph nodes, endpoints, edges and graph
/// construction.
pub use graph::{
    PipelineEdge,
    PipelineEdgeKind,
    PipelineEndpoint,
    PipelineGraph,
    PipelineGraphBuilder,
    PipelineGraphError,
};

// ============================================================================
// PUBLIC DEPENDENCY API
// ============================================================================

/// Public dependency analysis interfaces.
///
/// Dependency analysis is derived from the canonical pipeline graph.
pub use dependencies::{
    DependencyAnalysis,
    DependencyAnalysisError,
};

// ============================================================================
// PUBLIC TRANSFORMATION API
// ============================================================================

/// Public graph transformation interfaces.
///
/// Transformations produce new graph representations and require
/// subsequent semantic revalidation.
pub use transformations::{
    PipelineEdgeSelector,
    PipelineGraphEdit,
    PipelineTransformation,
    PipelineTransformationEngine,
    TransformationError,
    TransformationId,
    TransformationPrecondition,
    TransformationReport,
    TransformationSemanticStatus,
    TransformationStepReport,
    TransformationVersion,
    TransformedPipeline,
};

// ============================================================================
// PUBLIC VALIDATION API
// ============================================================================

/// Public pipeline validation interfaces.
///
/// Structural validity and semantic validity remain distinct.
pub use validation::{
    validate_pipeline,
    validate_pipeline_structure,
    validate_pipeline_with_options,
    PipelineDiagnosticSeverity,
    PipelineValidationContext,
    PipelineValidationIssue,
    PipelineValidationOptions,
    PipelineValidationReport,
    PipelineValidator,
    StructuralOnlyContext,
    ValidationDecision,
};

// ============================================================================
// PUBLIC EXECUTION-PLAN API
// ============================================================================

/// Public abstract pipeline execution-plan interfaces.
///
/// These types describe logical execution dependencies without
/// selecting or allocating physical resources.
pub use execution_plan::{
    PipelineDependencyOrder,
    PipelineExecutionPlan,
    PipelineExecutionPlanError,
    PipelineStagePlan,
};

// ============================================================================
// COMPOSITION-ROOT CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::AiPipelineId;

    fn pipeline_id(value: u128) -> AiPipelineId {
        AiPipelineId::new(value)
            .expect("nonzero pipeline identity")
    }

    fn stage_id(value: &str) -> PipelineStageId {
        PipelineStageId::new(value)
            .expect("valid stage identity")
    }

    fn operation(value: &str) -> PipelineOperationRef {
        PipelineOperationRef::new("zamani.ai", value)
            .expect("valid operation reference")
    }

    fn stage(value: &str) -> PipelineStageDefinition {
        PipelineStageDefinition::new(
            stage_id(value),
            operation(value),
        )
    }

    fn definition(
        id: u128,
        names: &[&str],
    ) -> PipelineDefinition {
        let mut builder = PipelineDefinitionBuilder::new(
            pipeline_id(id),
            "pipeline_contract_test",
        )
        .expect("valid pipeline definition");

        for name in names {
            builder
                .add_stage(stage(name))
                .expect("unique stage identity");
        }

        builder.build()
    }

    /// Verifies that the public composition root exposes all the
    /// canonical pipeline lifecycle components.
    #[test]
    fn public_pipeline_api_is_available() {
        let definition = definition(
            1,
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

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(analysis.stage_count(), 3);
        assert!(!analysis.has_cycles());

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("valid structural execution plan");

        assert_eq!(plan.pipeline_id(), definition.id());
        assert_eq!(plan.stage_count(), 3);
        assert_eq!(plan.edge_count(), 2);
        assert!(plan.dependency_order().is_acyclic());

        assert!(
            plan.requires_semantic_validation(),
            "plan construction must not claim semantic validity"
        );
    }

    /// Verifies that pipeline cycles remain representable and that
    /// the public API does not fabricate a topological ordering.
    #[test]
    fn cyclic_pipeline_relationships_are_preserved() {
        let definition = definition(
            2,
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

        let analysis = DependencyAnalysis::analyze(&graph);

        assert!(analysis.has_cycles());
        assert!(analysis.topological_order().is_none());

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("cyclic structure remains representable");

        assert!(plan.has_cycles());
        assert!(
            plan.dependency_order()
                .topological_order()
                .is_none()
        );
    }

    /// Verifies that an empty pipeline can be represented structurally.
    ///
    /// Its actual source-language legality remains a semantic decision.
    #[test]
    fn empty_pipeline_is_structurally_representable() {
        let definition = definition(3, &[]);

        let graph = PipelineGraph::builder(&definition).build();

        let analysis = DependencyAnalysis::analyze(&graph);

        assert_eq!(analysis.stage_count(), 0);
        assert!(!analysis.has_cycles());

        let plan = PipelineExecutionPlan::build(
            &definition,
            &graph,
        )
        .expect("empty structural plan");

        assert!(plan.is_empty());
        assert!(plan.requires_semantic_validation());
    }

    /// Verifies that the public definition API preserves stage
    /// declaration order independently of dependency order.
    #[test]
    fn declaration_order_is_preserved() {
        let definition = definition(
            4,
            &["zeta", "alpha", "middle"],
        );

        let names: Vec<&str> = definition
            .stages()
            .iter()
            .map(|stage| stage.id().as_str())
            .collect();

        assert_eq!(
            names,
            vec!["zeta", "alpha", "middle"]
        );
    }

    /// Verifies that the public graph API preserves pipeline identity.
    #[test]
    fn graph_identity_matches_definition_identity() {
        let definition = definition(5, &["source"]);

        let graph = PipelineGraph::builder(&definition).build();

        assert_eq!(
            graph.pipeline_id(),
            definition.id()
        );
    }
}
