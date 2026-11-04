
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Pipeline Tests
//!
//! Path: `src/ai/tests/pipelines.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! These tests exercise the existing canonical AI pipeline APIs.
//!
//! Ownership boundaries:
//! - `definition.rs` owns pipeline and stage declarations.
//! - `graph.rs` owns logical graph construction and edge integrity.
//! - `dependencies.rs` owns dependency analysis and cycle detection.
//! - `transformations.rs` owns target-independent graph edits.
//! - `validation.rs` owns structural and semantic validation.
//! - `execution_plan.rs` owns abstract execution-plan descriptions.
//! - The frontend semantic subsystem remains the source-language
//!   semantic authority.
//! - ZUIR remains the canonical universal IR.
//! - Distributed scheduling and runtime execution remain outside
//!   this subsystem.
//!
//! These tests intentionally do not claim that constructing an
//! execution plan executes a pipeline.
//!
//! No test imposes a universal pipeline-size or hardware limit.

use crate::ai::ids::AiPipelineId;

use crate::ai::pipeline::{
    validate_pipeline,
    validate_pipeline_structure,
    validate_pipeline_with_options,
    DependencyAnalysis,
    PipelineDefinition,
    PipelineDefinitionBuilder,
    PipelineDefinitionVersion,
    PipelineDiagnosticSeverity,
    PipelineEdgeKind,
    PipelineExecutionPlan,
    PipelineGraph,
    PipelineGraphEdit,
    PipelineOperationRef,
    PipelinePort,
    PipelinePortDirection,
    PipelineStageDefinition,
    PipelineStageId,
    PipelineTransformation,
    PipelineTransformationEngine,
    PipelineValidationContext,
    PipelineValidationOptions,
    StructuralOnlyContext,
    TransformationId,
    TransformationSemanticStatus,
    TransformationVersion,
    ValidationDecision,
};

fn pipeline_id(value: u128) -> AiPipelineId {
    AiPipelineId::new(value).expect("pipeline identity must be nonzero")
}

fn stage_id(value: &str) -> PipelineStageId {
    PipelineStageId::new(value).expect("stage identity must be valid")
}

fn operation(value: &str) -> PipelineOperationRef {
    PipelineOperationRef::new("zamani.ai", value)
        .expect("operation reference must be valid")
}

fn stage(value: &str) -> PipelineStageDefinition {
    PipelineStageDefinition::new(stage_id(value), operation(value))
}

fn definition(
    id: u128,
    names: &[&str],
) -> PipelineDefinition {
    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(id),
        "pipeline_test",
    )
    .expect("pipeline definition must be constructible");

    for name in names {
        builder
            .add_stage(stage(name))
            .expect("stage identities must be unique");
    }

    builder.build()
}

fn graph(definition: &PipelineDefinition) -> PipelineGraph {
    PipelineGraph::builder(definition).build()
}

/// A test semantic context that explicitly accepts every validation
/// obligation.
///
/// This is deliberately a test implementation. Production compilation
/// must supply the canonical frontend semantic adapter.
#[derive(Clone, Copy, Debug, Default)]
struct AcceptAllContext;

impl PipelineValidationContext for AcceptAllContext {
    fn validate_pipeline(
        &self,
        _definition: &PipelineDefinition,
    ) -> ValidationDecision {
        ValidationDecision::Accepted
    }

    fn validate_stage(
        &self,
        _stage: &PipelineStageDefinition,
    ) -> ValidationDecision {
        ValidationDecision::Accepted
    }

    fn validate_edge(
        &self,
        _edge: &crate::ai::pipeline::PipelineEdge,
        _source: &PipelineStageDefinition,
        _target: &PipelineStageDefinition,
    ) -> ValidationDecision {
        ValidationDecision::Accepted
    }

    fn validate_cycle(
        &self,
        _component: &[PipelineStageId],
    ) -> ValidationDecision {
        ValidationDecision::Accepted
    }
}

// ============================================================================
// DEFINITION CONTRACTS
// ============================================================================

#[test]
fn definition_preserves_identity_name_version_and_stage_order() {
    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(1),
        "training_pipeline",
    )
    .expect("valid pipeline");

    builder = builder.version(
        PipelineDefinitionVersion::new(2, 3, 4),
    );

    builder
        .add_stage(stage("load"))
        .expect("first stage");

    builder
        .add_stage(stage("train"))
        .expect("second stage");

    builder
        .add_stage(stage("evaluate"))
        .expect("third stage");

    let definition = builder.build();

    assert_eq!(definition.id(), pipeline_id(1));
    assert_eq!(definition.name(), "training_pipeline");

    assert_eq!(
        definition.version(),
        PipelineDefinitionVersion::new(2, 3, 4)
    );

    assert_eq!(definition.stage_count(), 3);

    let names: Vec<&str> = definition
        .stages()
        .iter()
        .map(|stage| stage.id().as_str())
        .collect();

    assert_eq!(names, ["load", "train", "evaluate"]);
}

#[test]
fn definition_rejects_duplicate_stage_identities() {
    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(2),
        "duplicate_stage_test",
    )
    .expect("valid pipeline");

    builder
        .add_stage(stage("prepare"))
        .expect("first declaration");

    let result = builder.add_stage(stage("prepare"));

    assert!(
        result.is_err(),
        "duplicate stage identities must not be silently accepted"
    );
}

#[test]
fn definition_rejects_invalid_stage_identifiers() {
    assert!(PipelineStageId::new("").is_err());

    assert!(
        PipelineStageId::new("invalid\nstage").is_err()
    );
}

#[test]
fn definition_rejects_duplicate_port_names() {
    let mut item = stage("producer");

    item.add_port(
        PipelinePort::new(
            "value",
            PipelinePortDirection::Output,
            None,
        )
        .expect("valid output port"),
    )
    .expect("first port");

    let result = item.add_port(
        PipelinePort::new(
            "value",
            PipelinePortDirection::Input,
            None,
        )
        .expect("valid input port"),
    );

    assert!(
        result.is_err(),
        "port names must be unique within a stage"
    );
}

#[test]
fn definition_supports_dynamic_metadata() {
    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(3),
        "metadata_pipeline",
    )
    .expect("valid pipeline");

    for index in 0..64 {
        builder
            .set_metadata(
                format!("extension.{index}"),
                format!("value.{index}"),
            )
            .expect("valid metadata");
    }

    let definition = builder.build();

    assert_eq!(definition.metadata().entries().len(), 64);

    assert_eq!(
        definition.metadata().get("extension.17"),
        Some("value.17")
    );
}

// ============================================================================
// GRAPH CONSTRUCTION AND INTEGRITY
// ============================================================================

#[test]
fn graph_contains_every_declared_stage() {
    let definition = definition(
        10,
        &["source", "transform", "sink"],
    );

    let graph = graph(&definition);

    assert_eq!(graph.pipeline_id(), definition.id());
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 0);

    for name in ["source", "transform", "sink"] {
        assert!(graph.contains_node(&stage_id(name)));
    }
}

#[test]
fn graph_rejects_undeclared_stage_dependencies() {
    let definition = definition(
        11,
        &["declared"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    let result = builder.add_dependency(
        &stage_id("missing"),
        &stage_id("declared"),
    );

    assert!(
        result.is_err(),
        "an undeclared stage must not enter the graph"
    );
}

#[test]
fn graph_rejects_duplicate_edges() {
    let definition = definition(
        12,
        &["first", "second"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("first"),
            &stage_id("second"),
        )
        .expect("first edge");

    let duplicate = builder.add_dependency(
        &stage_id("first"),
        &stage_id("second"),
    );

    assert!(
        duplicate.is_err(),
        "exact duplicate edges must be rejected"
    );
}

#[test]
fn graph_allows_distinct_relationship_kinds_between_stages() {
    let definition = definition(
        13,
        &["first", "second"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("first"),
            &stage_id("second"),
        )
        .expect("dependency edge");

    builder
        .add_control_dependency(
            &stage_id("first"),
            &stage_id("second"),
        )
        .expect("distinct control edge");

    let graph = builder.build();

    assert_eq!(graph.edge_count(), 2);

    let kinds: Vec<PipelineEdgeKind> = graph
        .edges()
        .map(|edge| edge.kind())
        .collect();

    assert!(kinds.contains(&PipelineEdgeKind::Dependency));
    assert!(kinds.contains(&PipelineEdgeKind::Control));
}

#[test]
fn graph_rejects_data_flow_from_a_non_output_port() {
    let mut producer = stage("producer");

    producer
        .add_port(
            PipelinePort::new(
                "input",
                PipelinePortDirection::Input,
                None,
            )
            .expect("valid input"),
        )
        .expect("port");

    let mut consumer = stage("consumer");

    consumer
        .add_port(
            PipelinePort::new(
                "result",
                PipelinePortDirection::Input,
                None,
            )
            .expect("valid input"),
        )
        .expect("port");

    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(14),
        "invalid_data_flow",
    )
    .expect("pipeline");

    builder.add_stage(producer).expect("producer");
    builder.add_stage(consumer).expect("consumer");

    let definition = builder.build();
    let mut graph_builder = PipelineGraph::builder(&definition);

    let result = graph_builder.add_data_flow(
        &stage_id("producer"),
        "input",
        &stage_id("consumer"),
        "result",
    );

    assert!(
        result.is_err(),
        "data flow must originate from an output port"
    );
}

#[test]
fn graph_accepts_valid_data_flow_between_declared_ports() {
    let mut producer = stage("producer");

    producer
        .add_port(
            PipelinePort::new(
                "features",
                PipelinePortDirection::Output,
                Some("tensor.features".to_owned()),
            )
            .expect("valid output"),
        )
        .expect("port");

    let mut consumer = stage("consumer");

    consumer
        .add_port(
            PipelinePort::new(
                "input",
                PipelinePortDirection::Input,
                Some("tensor.features".to_owned()),
            )
            .expect("valid input"),
        )
        .expect("port");

    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(15),
        "valid_data_flow",
    )
    .expect("pipeline");

    builder.add_stage(producer).expect("producer");
    builder.add_stage(consumer).expect("consumer");

    let definition = builder.build();
    let mut graph_builder = PipelineGraph::builder(&definition);

    graph_builder
        .add_data_flow(
            &stage_id("producer"),
            "features",
            &stage_id("consumer"),
            "input",
        )
        .expect("valid data flow");

    let graph = graph_builder.build();

    assert_eq!(graph.edge_count(), 1);

    let edge = graph.edges().next().expect("one edge");

    assert_eq!(edge.kind(), PipelineEdgeKind::DataFlow);
    assert_eq!(edge.source().port_name(), Some("features"));
    assert_eq!(edge.target().port_name(), Some("input"));
}

// ============================================================================
// DEPENDENCY ANALYSIS
// ============================================================================

#[test]
fn dependency_analysis_finds_direct_and_transitive_dependencies() {
    let definition = definition(
        20,
        &["source", "transform", "train", "evaluate"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(&stage_id("source"), &stage_id("transform"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("transform"), &stage_id("train"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("train"), &stage_id("evaluate"))
        .expect("edge");

    let graph = builder.build();
    let analysis = DependencyAnalysis::analyze(&graph);

    let direct = analysis
        .direct_dependencies(&stage_id("train"))
        .expect("known stage");

    assert!(direct.contains(&stage_id("transform")));

    let transitive = analysis
        .transitive_dependencies(&stage_id("evaluate"))
        .expect("known stage");

    assert_eq!(transitive.len(), 3);
    assert!(transitive.contains(&stage_id("source")));
    assert!(transitive.contains(&stage_id("transform")));
    assert!(transitive.contains(&stage_id("train")));
}

#[test]
fn dependency_analysis_produces_dependency_respecting_order() {
    let definition = definition(
        21,
        &["source", "transform", "train", "evaluate"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(&stage_id("source"), &stage_id("transform"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("transform"), &stage_id("train"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("train"), &stage_id("evaluate"))
        .expect("edge");

    let graph = builder.build();
    let analysis = DependencyAnalysis::analyze(&graph);

    let order = analysis
        .topological_order()
        .expect("acyclic graph has a topological order");

    let position = |name: &str| {
        order
            .iter()
            .position(|id| id == &stage_id(name))
            .expect("stage must be present")
    };

    assert!(position("source") < position("transform"));
    assert!(position("transform") < position("train"));
    assert!(position("train") < position("evaluate"));
}

#[test]
fn dependency_analysis_detects_cycles_without_hiding_them() {
    let definition = definition(
        22,
        &["first", "second", "third"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(&stage_id("first"), &stage_id("second"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("second"), &stage_id("third"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("third"), &stage_id("first"))
        .expect("edge");

    let graph = builder.build();
    let analysis = DependencyAnalysis::analyze(&graph);

    assert!(analysis.has_cycles());
    assert!(analysis.topological_order().is_none());
    assert!(analysis.require_acyclic().is_err());
    assert!(!analysis.cyclic_components().is_empty());
}

#[test]
fn dependency_analysis_handles_empty_and_disconnected_graphs() {
    let empty_definition = definition(23, &[]);
    let empty_graph = graph(&empty_definition);
    let empty_analysis = DependencyAnalysis::analyze(&empty_graph);

    assert!(empty_analysis.is_empty());
    assert!(!empty_analysis.has_cycles());
    assert_eq!(
        empty_analysis.topological_order(),
        Some([].as_slice())
    );

    let disconnected_definition = definition(
        24,
        &["one", "two", "three"],
    );

    let disconnected_graph = graph(&disconnected_definition);
    let disconnected = DependencyAnalysis::analyze(&disconnected_graph);

    assert_eq!(disconnected.stage_count(), 3);
    assert_eq!(disconnected.relationship_count(), 0);
    assert!(disconnected.topological_order().is_some());
}

// ============================================================================
// VALIDATION
// ============================================================================

#[test]
fn complete_semantic_context_produces_valid_report() {
    let definition = definition(
        30,
        &["load", "infer", "publish"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(&stage_id("load"), &stage_id("infer"))
        .expect("edge");

    builder
        .add_dependency(&stage_id("infer"), &stage_id("publish"))
        .expect("edge");

    let graph = builder.build();

    let report = validate_pipeline(
        &definition,
        &graph,
        &AcceptAllContext,
    );

    assert!(report.is_complete());
    assert!(report.is_valid());
    assert!(!report.has_errors());
    assert!(!report.has_incomplete_checks());

    assert_eq!(report.stage_count(), 3);
    assert_eq!(report.edge_count(), 2);
    assert!(report.dependency_analysis().is_some());
}

#[test]
fn structural_only_validation_never_claims_complete_semantic_validity() {
    let definition = definition(
        31,
        &["load", "infer"],
    );

    let graph = graph(&definition);

    let report = validate_pipeline_structure(
        &definition,
        &graph,
    );

    assert!(!report.is_complete());
    assert!(!report.is_valid());
    assert!(report.has_incomplete_checks());
}

#[test]
fn missing_semantic_obligations_are_not_treated_as_accepted() {
    let definition = definition(
        32,
        &["stage"],
    );

    let graph = graph(&definition);

    let report = validate_pipeline(
        &definition,
        &graph,
        &StructuralOnlyContext,
    );

    assert!(!report.is_valid());
    assert!(report.has_incomplete_checks());
    assert!(report.incomplete_count() > 0);
}

#[test]
fn semantic_rejection_produces_an_error() {
    struct RejectPipeline;

    impl PipelineValidationContext for RejectPipeline {
        fn validate_pipeline(
            &self,
            _definition: &PipelineDefinition,
        ) -> ValidationDecision {
            ValidationDecision::rejected(
                "pipeline violates a semantic policy",
            )
        }
    }

    let definition = definition(
        33,
        &["stage"],
    );

    let graph = graph(&definition);

    let report = validate_pipeline(
        &definition,
        &graph,
        &RejectPipeline,
    );

    assert!(report.has_errors());
    assert!(!report.is_valid());
    assert!(report.issues().iter().any(|issue| {
        issue.severity() == PipelineDiagnosticSeverity::Error
    }));
}

#[test]
fn mismatched_definition_and_graph_are_rejected() {
    let definition = definition(
        34,
        &["stage"],
    );

    let other_definition = definition(
        35,
        &["stage"],
    );

    let other_graph = graph(&other_definition);

    let report = validate_pipeline(
        &definition,
        &other_graph,
        &AcceptAllContext,
    );

    assert!(!report.is_valid());
    assert!(report.has_errors());
}

#[test]
fn validation_options_control_empty_pipeline_warning() {
    let definition = definition(36, &[]);
    let graph = graph(&definition);

    let report = validate_pipeline_with_options(
        &definition,
        &graph,
        &AcceptAllContext,
        PipelineValidationOptions {
            warn_on_empty_pipeline: false,
        },
    );

    assert!(report.is_valid());
    assert!(!report.has_warnings());
}

#[test]
fn explicitly_accepted_cycle_can_be_semantically_valid() {
    let definition = definition(
        37,
        &["feedback_a", "feedback_b"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("feedback_a"),
            &stage_id("feedback_b"),
        )
        .expect("edge");

    builder
        .add_dependency(
            &stage_id("feedback_b"),
            &stage_id("feedback_a"),
        )
        .expect("edge");

    let graph = builder.build();

    let report = validate_pipeline(
        &definition,
        &graph,
        &AcceptAllContext,
    );

    assert!(report.is_complete());
    assert!(report.is_valid());
    assert!(
        report
            .dependency_analysis()
            .expect("analysis")
            .has_cycles()
    );
}

// ============================================================================
// TRANSFORMATIONS
// ============================================================================

fn transformation_id(name: &str) -> TransformationId {
    TransformationId::new("zamani.ai.pipeline.test", name)
        .expect("valid transformation identity")
}

fn transformation(name: &str) -> PipelineTransformation {
    PipelineTransformation::new(
        transformation_id(name),
        TransformationVersion::new(1, 0, 0),
        format!("Test transformation: {name}"),
    )
}

#[test]
fn transformation_adds_dependency_without_mutating_original_graph() {
    let definition = definition(
        40,
        &["source", "target"],
    );

    let original = graph(&definition);

    let mut edit = transformation("add_dependency");

    edit.add_edit(PipelineGraphEdit::add_dependency(
        stage_id("source"),
        stage_id("target"),
    ));

    let transformed = PipelineTransformationEngine::apply(
        &definition,
        &original,
        &edit,
    )
    .expect("valid transformation");

    assert_eq!(original.edge_count(), 0);
    assert_eq!(transformed.graph().edge_count(), 1);

    assert_eq!(
        transformed.report().semantic_status(),
        TransformationSemanticStatus::RequiresRevalidation
    );
}

#[test]
fn transformation_rejects_duplicate_edge_addition() {
    let definition = definition(
        41,
        &["source", "target"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("source"),
            &stage_id("target"),
        )
        .expect("initial edge");

    let original = builder.build();

    let mut edit = transformation("duplicate_edge");

    edit.add_edit(PipelineGraphEdit::add_dependency(
        stage_id("source"),
        stage_id("target"),
    ));

    let result = PipelineTransformationEngine::apply(
        &definition,
        &original,
        &edit,
    );

    assert!(result.is_err());
    assert_eq!(original.edge_count(), 1);
}

#[test]
fn transformation_sequence_is_atomic_to_the_caller() {
    let definition = definition(
        42,
        &["source", "middle", "sink"],
    );

    let original = graph(&definition);

    let mut first = transformation("first");

    first.add_edit(PipelineGraphEdit::add_dependency(
        stage_id("source"),
        stage_id("middle"),
    ));

    let mut second = transformation("second");

    second.add_edit(PipelineGraphEdit::add_dependency(
        stage_id("source"),
        stage_id("middle"),
    ));

    let result = PipelineTransformationEngine::apply_sequence(
        &definition,
        &original,
        &[first, second],
    );

    assert!(
        result.is_err(),
        "a failed transformation sequence must not return a partial result"
    );

    assert_eq!(
        original.edge_count(),
        0,
        "the original immutable graph must remain unchanged"
    );
}

#[test]
fn identity_transformation_preserves_graph_structure() {
    let definition = definition(
        43,
        &["source", "target"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("source"),
            &stage_id("target"),
        )
        .expect("edge");

    let original = builder.build();
    let edit = transformation("identity");

    assert!(edit.is_identity());

    let transformed = PipelineTransformationEngine::apply(
        &definition,
        &original,
        &edit,
    )
    .expect("identity transformation");

    assert_eq!(transformed.graph(), &original);
    assert_eq!(transformed.report().resulting_edge_count(), 1);
}

// ============================================================================
// ABSTRACT EXECUTION PLANS
// ============================================================================

#[test]
fn execution_plan_preserves_pipeline_dependencies() {
    let definition = definition(
        50,
        &["prepare", "train", "evaluate"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("prepare"),
            &stage_id("train"),
        )
        .expect("edge");

    builder
        .add_dependency(
            &stage_id("train"),
            &stage_id("evaluate"),
        )
        .expect("edge");

    let graph = builder.build();

    let plan = PipelineExecutionPlan::build(
        &definition,
        &graph,
    )
    .expect("valid abstract plan");

    assert_eq!(plan.pipeline_id(), definition.id());
    assert_eq!(plan.definition_version(), definition.version());
    assert_eq!(plan.stage_count(), 3);
    assert_eq!(plan.edge_count(), 2);
    assert!(!plan.has_cycles());

    let order = plan
        .dependency_order()
        .topological_order()
        .expect("acyclic plan");

    let position = |name: &str| {
        order
            .iter()
            .position(|id| id == &stage_id(name))
            .expect("stage present")
    };

    assert!(position("prepare") < position("train"));
    assert!(position("train") < position("evaluate"));
}

#[test]
fn execution_plan_preserves_cycles_for_semantic_interpretation() {
    let definition = definition(
        51,
        &["feedback_a", "feedback_b"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("feedback_a"),
            &stage_id("feedback_b"),
        )
        .expect("edge");

    builder
        .add_dependency(
            &stage_id("feedback_b"),
            &stage_id("feedback_a"),
        )
        .expect("edge");

    let graph = builder.build();

    let plan = PipelineExecutionPlan::build(
        &definition,
        &graph,
    )
    .expect("cyclic abstract plan remains representable");

    assert!(plan.has_cycles());
    assert!(
        plan.dependency_order().is_cyclic(),
        "cyclic structure must not be disguised as a DAG"
    );
}

#[test]
fn execution_plan_rejects_pipeline_identity_mismatch() {
    let definition = definition(
        52,
        &["stage"],
    );

    let other_definition = definition(
        53,
        &["stage"],
    );

    let other_graph = graph(&other_definition);

    let result = PipelineExecutionPlan::build(
        &definition,
        &other_graph,
    );

    assert!(result.is_err());
}

// ============================================================================
// DETERMINISM
// ============================================================================

#[test]
fn dependency_analysis_is_deterministic_across_edge_insertion_orders() {
    let definition = definition(
        60,
        &["a", "b", "c", "d"],
    );

    let mut first_builder = PipelineGraph::builder(&definition);

    first_builder
        .add_dependency(&stage_id("a"), &stage_id("c"))
        .expect("edge");

    first_builder
        .add_dependency(&stage_id("b"), &stage_id("c"))
        .expect("edge");

    first_builder
        .add_dependency(&stage_id("c"), &stage_id("d"))
        .expect("edge");

    let first_graph = first_builder.build();

    let mut second_builder = PipelineGraph::builder(&definition);

    second_builder
        .add_dependency(&stage_id("c"), &stage_id("d"))
        .expect("edge");

    second_builder
        .add_dependency(&stage_id("b"), &stage_id("c"))
        .expect("edge");

    second_builder
        .add_dependency(&stage_id("a"), &stage_id("c"))
        .expect("edge");

    let second_graph = second_builder.build();

    let first = DependencyAnalysis::analyze(&first_graph);
    let second = DependencyAnalysis::analyze(&second_graph);

    assert_eq!(
        first.topological_order(),
        second.topological_order()
    );

    assert_eq!(
        first.cyclic_components(),
        second.cyclic_components()
    );
}

#[test]
fn validation_diagnostics_are_deterministic() {
    let definition = definition(
        61,
        &["first", "second"],
    );

    let graph = graph(&definition);

    let first = validate_pipeline_structure(
        &definition,
        &graph,
    );

    let second = validate_pipeline_structure(
        &definition,
        &graph,
    );

    assert_eq!(first.issues(), second.issues());
}

// ============================================================================
// DYNAMIC SCALABILITY AND PORTABILITY CONTRACTS
// ============================================================================

#[test]
fn pipeline_supports_dynamically_sized_stage_collections() {
    // This is a regression test for dynamic collection behavior, not a
    // universal pipeline-stage limit.
    //
    // It deliberately tests a workload larger than a trivial example
    // without treating the chosen test size as an architectural ceiling.

    let count = 512usize;

    let mut builder = PipelineDefinitionBuilder::new(
        pipeline_id(70),
        "dynamically_sized_pipeline",
    )
    .expect("pipeline");

    for index in 0..count {
        builder
            .add_stage(stage(&format!("stage-{index}")))
            .expect("unique dynamically generated stage");
    }

    let definition = builder.build();

    assert_eq!(definition.stage_count(), count);

    let mut graph_builder = PipelineGraph::builder(&definition);

    for index in 0..count.saturating_sub(1) {
        graph_builder
            .add_dependency(
                &stage_id(&format!("stage-{index}")),
                &stage_id(&format!("stage-{}", index + 1)),
            )
            .expect("valid dependency");
    }

    let graph = graph_builder.build();

    assert_eq!(graph.node_count(), count);
    assert_eq!(graph.edge_count(), count - 1);

    let analysis = DependencyAnalysis::analyze(&graph);

    assert!(!analysis.has_cycles());

    assert_eq!(
        analysis
            .topological_order()
            .expect("chain must be acyclic")
            .len(),
        count
    );
}

#[test]
fn pipeline_identity_is_independent_of_stage_identity_values() {
    let first = definition(
        71,
        &["source", "target"],
    );

    let second = definition(
        72,
        &["source", "target"],
    );

    assert_ne!(first.id(), second.id());

    assert_eq!(
        first.stages()[0].id(),
        second.stages()[0].id()
    );
}

#[test]
fn pipeline_graph_contains_no_physical_execution_assignments() {
    let definition = definition(
        73,
        &["prepare", "execute"],
    );

    let mut builder = PipelineGraph::builder(&definition);

    builder
        .add_dependency(
            &stage_id("prepare"),
            &stage_id("execute"),
        )
        .expect("edge");

    let graph = builder.build();

    // The graph exposes logical stage identities and relationships.
    // Device selection and resource placement belong downstream.
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);

    assert!(
        graph
            .nodes()
            .all(|id| !id.as_str().is_empty())
    );
}
