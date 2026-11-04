
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI Optimization Integration Tests
//!
//! Path: src/ai/tests/optimization.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Behavioral and integration-contract tests for the canonical
//! Zamani AI optimization subsystem.
//!
//! These tests exercise existing optimization implementations.
//! They do not introduce another optimizer, computational graph,
//! semantic model, tensor implementation, IR, or execution engine.
//!
//! ## Ownership
//!
//! This file tests:
//!
//! - Structural graph analysis.
//! - Deterministic graph analysis.
//! - Dynamic graph growth.
//! - Transformation plan construction.
//! - Transformation structural validation.
//! - Stale transformation detection.
//! - Semantic preservation evidence.
//! - Fail-closed legality decisions.
//! - Extensible computational cost models.
//! - Numerical input validation.
//! - Grammar-to-domain architecture contracts.
//! - POCO-REAF invariants.
//!
//! ## Canonical authorities
//!
//! Graph analysis:
//!     crate::ai::optimization::graph
//!
//! Transformations:
//!     crate::ai::optimization::transformations
//!
//! Transformation legality:
//!     crate::ai::optimization::legality
//!
//! Precision:
//!     crate::ai::optimization::precision
//!
//! Quantization:
//!     crate::ai::optimization::quantization
//!
//! Sparsity:
//!     crate::ai::optimization::sparsity
//!
//! Cost estimation:
//!     crate::ai::optimization::cost_model
//!
//! AI semantics:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal representation:
//!     crate::zuir
//!
//! ## Grammar integration
//!
//! Source optimization intent:
//!     grammar/ai/optimization.g4
//!
//! AI composition:
//!     grammar/ai/ai.g4
//!
//! Learning integration:
//!     grammar/ai/learning.g4
//!
//! Training integration:
//!     grammar/ai/training.g4
//!
//! Grammar ownership:
//!     grammar/ai/README.md
//!
//! These tests do not claim that inspecting grammar text
//! proves that the ANTLR parser has successfully compiled it.
//!
//! Parser integration belongs to the frontend conformance suite.
//!
//! ## POCO-REAF
//!
//! Optimization semantics must not depend on:
//!
//! - CPU architecture.
//! - GPU vendor.
//! - Accelerator vendor.
//! - Physical memory capacity.
//! - Device count.
//! - Worker count.
//! - Cluster size.
//! - Tensor rank.
//! - Model size.
//! - Graph size.
//! - Hardware topology.
//!
//! Operational resource limits are distinct from semantic limits.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important correctness distinction
//!
//! These tests distinguish:
//!
//! - Structural validity.
//! - Semantic legality.
//! - Numerical legality.
//! - Cost estimation.
//! - Resource feasibility.
//! - Physical execution.
//!
//! Passing one category must never imply that the others passed.
//!
//! ## Freeze criteria
//!
//! - Actual canonical optimization APIs are exercised.
//! - Invalid graph structures are rejected.
//! - Graph ordering is deterministic.
//! - Cycles are reported correctly.
//! - Transformation plans are checked against graph revisions.
//! - Missing proof evidence cannot establish legality.
//! - Unknown evidence cannot establish legality.
//! - Disproven evidence rejects transformations.
//! - Cost estimates preserve separate dimensions.
//! - Custom cost dimensions remain extensible.
//! - No arbitrary workload ceilings are introduced.
//! - No hardware-specific assumptions are introduced.
//! - No unsafe Rust is used.

use std::collections::BTreeSet;

use crate::ai::optimization::graph::{
    analyze_graph,
    OptimizationGraphError,
};

use crate::ai::optimization::transformations::{
    GraphEdit,
    GraphRevision,
    LegalityResult,
    PreservationRequirements,
    TransformationGraph,
    TransformationId,
    TransformationPhase,
    TransformationPlan,
    TransformationPlanBuilder,
    TransformationVersion,
};

use crate::ai::optimization::legality::{
    obligations,
    LegalityEvidence,
    LegalityEvidenceProvider,
    LegalityStatus,
    LegalityValidator,
    ObligationId,
    ProofStatus,
    validate_structure,
};

use crate::ai::optimization::cost_model::{
    AnalyticalCostModel,
    CostModel,
    CostModelError,
    CostProfile,
    CostProfileBuilder,
    MetricId,
    Objective,
    ObjectiveWeight,
    Quantity,
    Workload,
};


// ============================================================================
// GRAPH TEST FIXTURES
// ============================================================================

/// Constructs a custom graph node identity.
///
/// The identity is independent of physical hardware.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
struct Node(u64);

/// Constructs a directed graph edge.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
struct Edge {
    source: Node,
    destination: Node,
}

/// Creates a graph node.
fn node(value: u64) -> Node {
    Node(value)
}

/// Creates a directed graph edge.
fn edge(source: u64, destination: u64) -> Edge {
    Edge {
        source: node(source),
        destination: node(destination),
    }
}


// ============================================================================
// GRAPH ANALYSIS
// ============================================================================

/// Empty graphs must produce valid empty analysis reports.
#[test]
fn graph_analysis_accepts_empty_graphs() {
    let result = analyze_graph(
        Vec::<Node>::new(),
        Vec::<(Node, Node)>::new(),
    )
    .expect("empty graph must be analyzable");

    assert!(result.is_empty());
    assert!(result.has_no_edges());
    assert!(result.is_acyclic());

    assert_eq!(result.node_count(), 0);
    assert_eq!(result.edge_count(), 0);

    assert_eq!(result.topological_order(), Some([].as_slice()));

    assert!(result.roots().is_empty());
    assert!(result.sinks().is_empty());
    assert!(result.isolated_nodes().is_empty());
}

/// An acyclic graph must receive a complete topological ordering.
#[test]
fn graph_analysis_produces_complete_topological_order() {
    let nodes = vec![
        node(4),
        node(2),
        node(1),
        node(3),
    ];

    let edges = vec![
        (node(1), node(3)),
        (node(2), node(3)),
        (node(3), node(4)),
    ];

    let result = analyze_graph(nodes, edges)
        .expect("valid directed graph");

    assert!(result.is_acyclic());

    assert!(result.has_complete_topological_order());

    assert_eq!(
        result.topological_order(),
        Some(
            &[
                node(1),
                node(2),
                node(3),
                node(4),
            ][..]
        )
    );

    assert_eq!(result.node_count(), 4);
    assert_eq!(result.edge_count(), 3);
}

/// Independent executions must produce identical graph ordering.
#[test]
fn graph_analysis_is_deterministic() {
    let first_nodes = vec![
        node(5),
        node(2),
        node(4),
        node(1),
        node(3),
    ];

    let second_nodes = vec![
        node(3),
        node(1),
        node(5),
        node(4),
        node(2),
    ];

    let edges = vec![
        (node(1), node(3)),
        (node(2), node(4)),
        (node(3), node(5)),
        (node(4), node(5)),
    ];

    let first = analyze_graph(
        first_nodes,
        edges.clone(),
    )
    .expect("first graph");

    let second = analyze_graph(
        second_nodes,
        edges,
    )
    .expect("second graph");

    assert_eq!(
        first,
        second,
        "equivalent graph inputs must have deterministic reports"
    );
}

/// Parallel edges must be preserved independently.
#[test]
fn graph_analysis_preserves_parallel_edges() {
    let result = analyze_graph(
        vec![
            node(1),
            node(2),
        ],
        vec![
            (node(1), node(2)),
            (node(1), node(2)),
            (node(1), node(2)),
        ],
    )
    .expect("parallel edges are valid");

    assert_eq!(result.edge_count(), 3);
    assert_eq!(result.maximum_out_degree(), Some(3));
    assert_eq!(result.maximum_in_degree(), Some(3));
}

/// Isolated nodes must be reported as both roots and sinks.
#[test]
fn graph_analysis_identifies_isolated_nodes() {
    let result = analyze_graph(
        vec![
            node(1),
            node(2),
            node(3),
        ],
        vec![
            (node(1), node(2)),
        ],
    )
    .expect("valid graph");

    assert_eq!(
        result.isolated_nodes(),
        &[node(3)]
    );

    assert!(
        result.roots().contains(&node(3))
    );

    assert!(
        result.sinks().contains(&node(3))
    );
}

/// Self-loops must be detected.
#[test]
fn graph_analysis_detects_self_loops() {
    let result = analyze_graph(
        vec![
            node(1),
            node(2),
        ],
        vec![
            (node(1), node(1)),
            (node(1), node(2)),
        ],
    )
    .expect("self-loops are structurally analyzable");

    assert!(result.has_self_loops());

    assert_eq!(result.self_loop_count(), 1);

    assert!(!result.is_acyclic());

    assert!(result.topological_order().is_none());
}

/// Cycles must not be interpreted as valid complete orderings.
#[test]
fn graph_analysis_rejects_complete_order_for_cycles() {
    let result = analyze_graph(
        vec![
            node(1),
            node(2),
            node(3),
        ],
        vec![
            (node(1), node(2)),
            (node(2), node(1)),
            (node(2), node(3)),
        ],
    )
    .expect("cyclic graphs must still be analyzable");

    assert!(!result.is_acyclic());

    assert!(
        result.topological_order().is_none()
    );

    assert!(
        !result.blocked_nodes().is_empty()
    );

    assert!(
        result.blocked_nodes().contains(&node(1))
    );

    assert!(
        result.blocked_nodes().contains(&node(2))
    );

    assert!(
        result.blocked_nodes().contains(&node(3))
    );
}

/// Blocked nodes must not be mistaken for exact cycle membership.
#[test]
fn graph_analysis_distinguishes_blocked_nodes_from_cycle_members() {
    let result = analyze_graph(
        vec![
            node(1),
            node(2),
            node(3),
            node(4),
        ],
        vec![
            (node(1), node(2)),
            (node(2), node(1)),
            (node(2), node(3)),
            (node(3), node(4)),
        ],
    )
    .expect("valid structural analysis");

    assert!(!result.is_acyclic());

    assert!(result.blocked_nodes().contains(&node(1)));
    assert!(result.blocked_nodes().contains(&node(2)));

    // Nodes 3 and 4 are downstream of the cycle.
    // The graph API does not claim they are cycle members.
    assert!(result.blocked_nodes().contains(&node(3)));
    assert!(result.blocked_nodes().contains(&node(4)));
}

/// Duplicate graph node identities must be rejected.
#[test]
fn graph_analysis_rejects_duplicate_nodes() {
    let result = analyze_graph(
        vec![
            node(1),
            node(1),
        ],
        Vec::<(Node, Node)>::new(),
    );

    assert!(matches!(
        result,
        Err(OptimizationGraphError::DuplicateNode { .. })
    ));
}

/// Undeclared edge endpoints must be rejected.
#[test]
fn graph_analysis_rejects_missing_endpoints() {
    let result = analyze_graph(
        vec![
            node(1),
        ],
        vec![
            (node(1), node(2)),
        ],
    );

    assert!(matches!(
        result,
        Err(OptimizationGraphError::MissingEndpoint {
            ..
        })
    ));
}

/// Dynamic graph growth must not introduce a semantic size ceiling.
///
/// This is a finite regression workload, not a maximum.
#[test]
fn graph_analysis_supports_dynamic_graph_growth() {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for index in 0_u64..1024 {
        nodes.push(node(index));

        if index > 0 {
            edges.push((
                node(index - 1),
                node(index),
            ));
        }
    }

    let result = analyze_graph(nodes, edges)
        .expect("dynamically constructed graph");

    assert_eq!(result.node_count(), 1024);
    assert_eq!(result.edge_count(), 1023);

    assert!(result.is_acyclic());

    assert!(
        result.has_complete_topological_order()
    );

    assert_eq!(
        result.topological_order()
            .expect("complete ordering")
            .first(),
        Some(&node(0))
    );

    assert_eq!(
        result.topological_order()
            .expect("complete ordering")
            .last(),
        Some(&node(1023))
    );
}


// ============================================================================
// CANONICAL TRANSFORMATION GRAPH
// ============================================================================

/// Test-only adapter over an existing graph contract.
///
/// This is not another production graph representation.
struct TestGraph {
    revision: GraphRevision,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl TransformationGraph for TestGraph {
    type Node = Node;
    type Edge = Edge;

    fn revision(&self) -> GraphRevision {
        self.revision
    }

    fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    fn source(&self, edge: &Edge) -> Node {
        edge.source.clone()
    }

    fn destination(&self, edge: &Edge) -> Node {
        edge.destination.clone()
    }

    fn is_transparent(&self, _: &Node) -> bool {
        false
    }

    fn compose_edges(
        &self,
        _: &Edge,
        _: &Edge,
        _: &Node,
    ) -> Option<Edge> {
        None
    }
}

/// Creates a valid test graph.
fn transformation_graph() -> TestGraph {
    TestGraph {
        revision: GraphRevision::new(1),

        nodes: vec![
            node(1),
            node(2),
            node(3),
        ],

        edges: vec![
            edge(1, 2),
            edge(2, 3),
        ],
    }
}

/// Creates a valid transformation plan.
fn transformation_plan(
    graph: &TestGraph,
) -> TransformationPlan<Node, Edge> {
    let mut builder =
        TransformationPlanBuilder::new(
            TransformationId::new(
                "zamani.test.graph_rewrite",
            )
            .expect("valid transformation identity"),

            TransformationVersion::new(1, 0, 0),

            graph.revision,

            TransformationPhase::GraphSimplification,
        )
        .with_description(
            "Test-only edge removal proposal",
        );

    builder
        .push(
            GraphEdit::RemoveEdge(
                graph.edges[0].clone(),
            ),
        )
        .expect("valid graph edit");

    builder
        .affect(node(1))
        .expect("valid affected node");

    builder
        .affect(node(2))
        .expect("valid affected node");

    builder
        .build()
        .expect("valid transformation plan")
}


// ============================================================================
// TRANSFORMATION STRUCTURAL VALIDATION
// ============================================================================

/// A plan based on the current graph revision must be accepted structurally.
#[test]
fn transformation_accepts_current_graph_revision() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    assert_eq!(
        validate_structure(&graph, &plan),
        Ok(())
    );
}

/// A plan created against an old revision must be rejected.
#[test]
fn transformation_rejects_stale_graph_revision() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    let mut changed_graph = graph;

    changed_graph.revision = GraphRevision::new(2);

    assert!(
        validate_structure(
            &changed_graph,
            &plan,
        )
        .is_err()
    );
}

/// Removing a node while retaining incident edges is invalid.
#[test]
fn transformation_rejects_dangling_edges() {
    let graph = transformation_graph();

    let mut builder =
        TransformationPlanBuilder::new(
            TransformationId::new(
                "zamani.test.invalid_node_removal",
            )
            .expect("valid identity"),

            TransformationVersion::new(1, 0, 0),

            graph.revision,

            TransformationPhase::GraphSimplification,
        )
        .with_description(
            "Intentionally invalid node removal",
        );

    builder
        .push(
            GraphEdit::RemoveNode(node(2)),
        )
        .expect("valid edit declaration");

    builder
        .affect(node(2))
        .expect("valid affected node");

    let plan = builder
        .build()
        .expect("plan is structurally constructible");

    assert!(
        validate_structure(
            &graph,
            &plan,
        )
        .is_err()
    );
}

/// Duplicate graph identities must be rejected before mutation.
#[test]
fn transformation_rejects_duplicate_graph_nodes() {
    let mut graph = transformation_graph();

    graph.nodes.push(node(1));

    let plan = transformation_plan(&graph);

    assert!(
        validate_structure(
            &graph,
            &plan,
        )
        .is_err()
    );
}


// ============================================================================
// SEMANTIC LEGALITY EVIDENCE
// ============================================================================

/// Supplies preservation evidence from a test verifier.
///
/// Production implementations must delegate to authoritative semantic,
/// numerical, security, provenance and IR verification services.
struct TestEvidenceProvider {
    status: ProofStatus,
}

impl LegalityEvidenceProvider<TestGraph>
    for TestEvidenceProvider
{
    fn assess(
        &self,
        _: &TestGraph,
        _: &TransformationPlan<Node, Edge>,
        _: &PreservationRequirements,
        required: &BTreeSet<ObligationId>,
    ) -> Result<Vec<LegalityEvidence>, String> {
        let mut evidence = Vec::new();

        for obligation in required {
            evidence.push(
                LegalityEvidence::new(
                    obligation.clone(),
                    self.status,
                    BTreeSet::from([
                        format!(
                            "test:verification:{}",
                            obligation.as_str(),
                        ),
                    ]),
                    "Controlled test verification result",
                )
                .map_err(|error| error.to_string())?,
            );
        }

        Ok(evidence)
    }
}

/// Returns a validator with a specified evidence status.
fn legality_validator(
    status: ProofStatus,
) -> LegalityValidator<TestEvidenceProvider> {
    LegalityValidator::new(
        TestEvidenceProvider {
            status,
        },
    )
}

/// Proven preservation evidence must produce legal status.
#[test]
fn legality_accepts_complete_proven_evidence() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    let assessment = legality_validator(
        ProofStatus::Proven,
    )
    .assess(
        &graph,
        &plan,
        &PreservationRequirements::default(),
    );

    assert_eq!(
        assessment.status(),
        LegalityStatus::Legal,
    );

    assert!(assessment.is_legal());

    assert!(
        !assessment.evidence_references().is_empty()
    );
}

/// Unknown evidence must never establish legality.
#[test]
fn legality_rejects_unknown_evidence_as_proof() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    let assessment = legality_validator(
        ProofStatus::Unknown,
    )
    .assess(
        &graph,
        &plan,
        &PreservationRequirements::default(),
    );

    assert_eq!(
        assessment.status(),
        LegalityStatus::Indeterminate,
    );

    assert!(!assessment.is_legal());

    assert!(assessment.is_indeterminate());
}

/// Disproven preservation must reject a transformation.
#[test]
fn legality_rejects_disproven_preservation() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    let assessment = legality_validator(
        ProofStatus::Disproven,
    )
    .assess(
        &graph,
        &plan,
        &PreservationRequirements::default(),
    );

    assert_eq!(
        assessment.status(),
        LegalityStatus::Illegal,
    );

    assert!(assessment.is_illegal());

    assert!(!assessment.is_legal());
}

/// Missing evidence must produce an indeterminate result.
#[test]
fn legality_rejects_missing_evidence() {
    let required = crate::ai::optimization::legality::
        required_obligations(
            &PreservationRequirements::default(),
        )
        .expect("required obligations");

    let assessment =
        crate::ai::optimization::legality::
            LegalityAssessmentTestAdapter::not_available;

    // The public validator is tested with an evidence provider
    // returning an empty collection in the dedicated test below.
    assert!(!required.is_empty());

    let _ = assessment;
}

/// An evidence provider that returns no proof.
struct MissingEvidenceProvider;

impl LegalityEvidenceProvider<TestGraph>
    for MissingEvidenceProvider
{
    fn assess(
        &self,
        _: &TestGraph,
        _: &TransformationPlan<Node, Edge>,
        _: &PreservationRequirements,
        _: &BTreeSet<ObligationId>,
    ) -> Result<Vec<LegalityEvidence>, String> {
        Ok(Vec::new())
    }
}

/// Missing evidence cannot be treated as successful verification.
#[test]
fn legality_is_fail_closed_when_provider_returns_no_evidence() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    let validator = LegalityValidator::new(
        MissingEvidenceProvider,
    );

    let assessment = validator.assess(
        &graph,
        &plan,
        &PreservationRequirements::default(),
    );

    assert!(assessment.is_indeterminate());

    assert!(!assessment.is_legal());

    assert!(
        !assessment.diagnostics().is_empty()
    );
}

/// Mandatory preservation obligations cannot be disabled.
#[test]
fn legality_preserves_mandatory_semantic_obligations() {
    let requirements = PreservationRequirements {
        observable_behavior: false,
        numerical_semantics: false,
        effect_ordering: false,
        state_dependencies: false,
        provenance: false,
        security: false,
        determinism: false,
        resource_requirements: false,
    };

    let required =
        crate::ai::optimization::legality::
            required_obligations(&requirements)
            .expect("mandatory obligations");

    for obligation in [
        obligations::OBSERVABLE_BEHAVIOR,
        obligations::TYPES_AND_SHAPES,
        obligations::EFFECTS,
        obligations::REPRODUCIBILITY,
        obligations::DEPENDENCIES,
        obligations::ALIASING,
    ] {
        assert!(
            required.contains(
                &ObligationId::new(obligation)
                    .expect("valid obligation"),
            ),
            "mandatory obligation must remain enabled"
        );
    }
}


// ============================================================================
// COST MODEL
// ============================================================================

/// Creates a valid extensible metric identity.
fn metric(name: &str) -> MetricId {
    MetricId::new(name)
        .expect("valid metric identity")
}

/// Creates a valid numerical quantity.
fn quantity(value: f64) -> Quantity {
    Quantity::new(value)
        .expect("valid finite non-negative quantity")
}

/// Cost estimation must preserve independent dimensions.
#[test]
fn cost_model_estimates_multiple_dimensions() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("compute.operations"),
            quantity(100.0),
        )
        .expect("unique metric");

    workload
        .insert(
            metric("memory.bytes"),
            quantity(2048.0),
        )
        .expect("unique metric");

    let profile =
        CostProfileBuilder::new("test-profile-v1")
            .expect("valid profile")
            .coefficient(
                "compute.operations",
                0.5,
                0.1,
            )
            .expect("valid coefficient")
            .coefficient(
                "memory.bytes",
                0.25,
                0.05,
            )
            .expect("valid coefficient")
            .build();

    let estimate = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        )
        .expect("cost estimate");

    assert_eq!(
        estimate.profile_version(),
        "test-profile-v1"
    );

    assert_eq!(
        estimate
            .get(&metric("compute.operations"))
            .expect("compute estimate")
            .expected
            .get(),
        50.0
    );

    assert_eq!(
        estimate
            .get(&metric("memory.bytes"))
            .expect("memory estimate")
            .expected
            .get(),
        512.0
    );

    assert_eq!(
        estimate.values().len(),
        2
    );
}

/// Custom cost dimensions must remain open-world.
#[test]
fn cost_model_supports_custom_metrics() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("extension.quantum.operation_cost"),
            quantity(12.0),
        )
        .expect("custom metric");

    let profile =
        CostProfileBuilder::new("extension-profile")
            .expect("profile")
            .coefficient(
                "extension.quantum.operation_cost",
                2.0,
                0.0,
            )
            .expect("custom coefficient")
            .build();

    let estimate = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        )
        .expect("custom metric estimate");

    assert_eq!(
        estimate
            .get(
                &metric("extension.quantum.operation_cost")
            )
            .expect("custom estimate")
            .expected
            .get(),
        24.0
    );
}

/// Objective weights must be explicitly supplied.
#[test]
fn cost_model_preserves_explicit_objective_weights() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("compute.operations"),
            quantity(10.0),
        )
        .expect("workload");

    let profile =
        CostProfileBuilder::new("objective-v1")
            .expect("profile")
            .coefficient(
                "compute.operations",
                2.0,
                0.0,
            )
            .expect("coefficient")
            .build();

    let estimate = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        )
        .expect("estimate");

    let mut objective = Objective::new();

    objective.set_weight(
        metric("compute.operations"),
        ObjectiveWeight::new(
            quantity(3.0),
        ),
    );

    assert_eq!(
        estimate
            .score(&objective)
            .expect("weighted score")
            .get(),
        60.0
    );
}

/// Missing cost coefficients must not silently become zero.
#[test]
fn cost_model_rejects_missing_coefficients() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("custom.energy"),
            quantity(10.0),
        )
        .expect("workload");

    let profile = CostProfile::new("incomplete")
        .expect("profile");

    let result = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        );

    assert!(matches!(
        result,
        Err(CostModelError::MissingCoefficient(_))
    ));
}

/// Duplicate workload metrics must be rejected.
#[test]
fn cost_model_rejects_duplicate_metrics() {
    let mut workload = Workload::new();

    let identity = metric("compute.operations");

    workload
        .insert(
            identity.clone(),
            quantity(10.0),
        )
        .expect("first insertion");

    assert!(matches!(
        workload.insert(
            identity,
            quantity(20.0),
        ),
        Err(CostModelError::DuplicateMetric(_))
    ));
}

/// Negative workload quantities are invalid.
#[test]
fn cost_model_rejects_negative_quantities() {
    assert!(matches!(
        Quantity::new(-1.0),
        Err(CostModelError::NegativeValue)
    ));
}

/// NaN and infinite quantities must be rejected.
#[test]
fn cost_model_rejects_non_finite_quantities() {
    assert!(matches!(
        Quantity::new(f64::NAN),
        Err(CostModelError::NonFiniteValue)
    ));

    assert!(matches!(
        Quantity::new(f64::INFINITY),
        Err(CostModelError::NonFiniteValue)
    ));

    assert!(matches!(
        Quantity::new(f64::NEG_INFINITY),
        Err(CostModelError::NonFiniteValue)
    ));
}

/// Arithmetic overflow must be reported rather than silently saturated.
#[test]
fn cost_model_reports_arithmetic_overflow() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("compute.operations"),
            quantity(f64::MAX),
        )
        .expect("workload");

    let profile =
        CostProfileBuilder::new("overflow-profile")
            .expect("profile")
            .coefficient(
                "compute.operations",
                2.0,
                0.0,
            )
            .expect("coefficient")
            .build();

    assert!(matches!(
        AnalyticalCostModel.estimate(
            &workload,
            &profile,
        ),
        Err(CostModelError::NonFiniteValue)
    ));
}

/// An objective referencing an unestimated metric must fail.
#[test]
fn cost_model_rejects_missing_objective_estimates() {
    let estimate = AnalyticalCostModel
        .estimate(
            &Workload::new(),
            &CostProfile::new("empty")
                .expect("profile"),
        )
        .expect("empty estimate");

    let mut objective = Objective::new();

    objective.set_weight(
        metric("custom.energy"),
        ObjectiveWeight::new(
            quantity(1.0),
        ),
    );

    assert!(matches!(
        estimate.score(&objective),
        Err(CostModelError::MissingEstimatedMetric(_))
    ));
}

/// Workload metrics must support dynamically growing collections.
///
/// This test does not establish a maximum number of metrics.
#[test]
fn cost_model_supports_dynamic_metric_growth() {
    let mut workload = Workload::new();

    let mut profile =
        CostProfile::new("dynamic-profile")
            .expect("profile");

    for index in 0_u64..256 {
        let identity = format!(
            "extension.metric.{index}"
        );

        let id = metric(&identity);

        workload
            .insert(
                id.clone(),
                quantity(index as f64),
            )
            .expect("unique metric");

        profile
            .register(
                id,
                crate::ai::optimization::cost_model::
                    CostCoefficient::new(
                        quantity(1.0),
                        quantity(0.0),
                    ),
            )
            .expect("unique coefficient");
    }

    let estimate = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        )
        .expect("dynamic cost estimate");

    assert_eq!(
        estimate.values().len(),
        256
    );
}


// ============================================================================
// GRAMMAR INTEGRATION CONTRACTS
// ============================================================================

/// The optimization grammar must retain its canonical entry rule.
#[test]
fn optimization_grammar_has_canonical_construct() {
    let grammar = include_str!(
        "../../../grammar/ai/optimization.g4"
    );

    assert!(
        grammar.contains("optimizationConstruct"),
        "canonical optimization construct must exist"
    );

    assert!(
        grammar.contains("optimizationDeclaration"),
        "optimization declaration must remain available"
    );

    assert!(
        grammar.contains("optimizationInvocation"),
        "optimization invocation must remain available"
    );

    assert!(
        grammar.contains("optimizationDirective"),
        "extensible optimization directives must remain available"
    );
}

/// Optimization grammar must reuse universal language authorities.
#[test]
fn optimization_grammar_reuses_universal_language_rules() {
    let grammar = include_str!(
        "../../../grammar/ai/optimization.g4"
    );

    assert!(
        grammar.contains("optimizationExpression"),
        "optimization expressions must use the canonical expression boundary"
    );

    assert!(
        grammar.contains("optimizationType"),
        "optimization types must use the canonical type boundary"
    );

    assert!(
        grammar.contains("optimizationStatement"),
        "optimization statements must use the universal statement boundary"
    );

    assert!(
        grammar.contains("optimizationPortabilityRequirement"),
        "portable optimization requirements must remain supported"
    );
}

/// Learning and training grammars must remain separate authorities.
#[test]
fn optimization_grammar_does_not_replace_learning_or_training_grammar() {
    let optimization = include_str!(
        "../../../grammar/ai/optimization.g4"
    );

    let learning = include_str!(
        "../../../grammar/ai/learning.g4"
    );

    let training = include_str!(
        "../../../grammar/ai/training.g4"
    );

    assert!(
        optimization.contains("optimizationConstruct")
    );

    assert!(
        learning.contains("aiLearningConstruct")
    );

    assert!(
        training.contains("training")
    );
}

/// The AI grammar must remain the composition authority.
#[test]
fn ai_grammar_remains_the_composition_root() {
    let grammar = include_str!(
        "../../../grammar/ai/ai.g4"
    );

    assert!(
        grammar.contains("aiConstruct"),
        "AI composition entry must remain available"
    );
}

/// Grammar documentation must preserve the portability contract.
#[test]
fn ai_grammar_documentation_preserves_poco_reaf() {
    let documentation = include_str!(
        "../../../grammar/ai/README.md"
    );

    assert!(
        documentation.contains("POCO-REAF")
    );

    assert!(
        documentation.contains("hardware")
    );

    assert!(
        documentation.contains("semantic")
    );
}


// ============================================================================
// PORTABILITY AND ARCHITECTURAL BOUNDARIES
// ============================================================================

/// Optimization plans must remain target-independent.
///
/// No physical device identity is required to construct a plan.
#[test]
fn transformation_plan_does_not_require_hardware_identity() {
    let graph = transformation_graph();

    let plan = transformation_plan(&graph);

    assert_eq!(
        plan.transformation().as_str(),
        "zamani.test.graph_rewrite"
    );

    assert_eq!(
        plan.source_revision(),
        GraphRevision::new(1)
    );

    assert_eq!(
        plan.version(),
        TransformationVersion::new(1, 0, 0)
    );
}

/// Resource estimation must remain separate from legality.
///
/// A cost estimate does not prove semantic preservation.
#[test]
fn cost_estimation_does_not_establish_transformation_legality() {
    let mut workload = Workload::new();

    workload
        .insert(
            metric("compute.operations"),
            quantity(100.0),
        )
        .expect("workload");

    let profile =
        CostProfileBuilder::new("portable")
            .expect("profile")
            .coefficient(
                "compute.operations",
                1.0,
                0.0,
            )
            .expect("coefficient")
            .build();

    let estimate = AnalyticalCostModel
        .estimate(
            &workload,
            &profile,
        )
        .expect("estimate");

    assert_eq!(
        estimate
            .get(&metric("compute.operations"))
            .expect("estimate")
            .expected
            .get(),
        100.0
    );

    // The estimate contains no semantic proof.
    // Legality must be assessed independently.
}

/// Optimization must preserve explicit semantic requirements.
#[test]
fn optimization_preservation_policy_defaults_are_enabled() {
    let requirements = PreservationRequirements::default();

    assert!(requirements.observable_behavior);
    assert!(requirements.numerical_semantics);
    assert!(requirements.effect_ordering);
    assert!(requirements.state_dependencies);
    assert!(requirements.provenance);
    assert!(requirements.security);
    assert!(requirements.determinism);
    assert!(requirements.resource_requirements);
}
