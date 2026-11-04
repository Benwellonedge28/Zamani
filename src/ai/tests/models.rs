
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Integration Tests
//!
//! File: src/ai/tests/models.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Verify the observable contracts of the canonical Zamani AI model
//! subsystem and its integration with the model graph, operations,
//! signatures, and structural validation.
//!
//! These tests exercise existing production implementations.
//! They do not introduce another model representation, graph implementation,
//! AST, tensor system, semantic type system, or executable IR.
//!
//! ============================================================================
//! AUTHORITATIVE IMPLEMENTATIONS
//! ============================================================================
//!
//! Model declarations:
//!     crate::ai::model::definition
//!
//! Model graphs:
//!     crate::ai::model::graph
//!
//! Model operations:
//!     crate::ai::model::operations
//!
//! Model signatures:
//!     crate::ai::model::signatures
//!
//! Model validation:
//!     crate::ai::model::validation
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! Canonical AI grammar:
//!     grammar/ai/models.g4
//!
//! AI grammar composition:
//!     grammar/ai/ai.g4
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - Integration tests for model construction.
//! - Integration tests for model structural invariants.
//! - Integration tests for graph behavior.
//! - Integration tests for model operation contracts.
//! - Integration tests for model signatures.
//! - Integration tests for deterministic behavior.
//! - Integration tests for dynamically sized model structures.
//! - Regression tests for cross-module invariants.
//!
//! This file does not own:
//!
//! - Model implementation.
//! - Model graph algorithms.
//! - Model execution.
//! - Model training.
//! - Tensor storage.
//! - AST construction.
//! - Semantic analysis.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Backend execution.
//!
//! ============================================================================
//! POCO-REAF CONTRACT
//! ============================================================================
//!
//! Model semantics must remain independent of:
//!
//! - CPU count.
//! - GPU count.
//! - Accelerator count.
//! - Machine memory.
//! - Distributed node count.
//! - Physical topology.
//! - Hardware vendor.
//! - Runtime placement.
//!
//! Tests may use finite generated workloads to verify scalability,
//! but those workload sizes must never become implementation ceilings.
//!
//! Literal infinite execution is not promised.
//! Actual execution remains constrained by available resources.
//!
//! ============================================================================

use crate::ai::model::definition::{
    ModelConnection,
    ModelDefinition,
    ModelDefinitionError,
    ModelEndpoint,
    ModelExpressionRef,
    ModelMember,
    ModelMemberId,
    ModelMemberPayload,
    ModelMemberRole,
};

use crate::ai::model::graph::{
    ModelGraph,
    ModelGraphError,
};

use crate::ai::model::operations::{
    ModelOperation,
    ModelOperationId,
    ModelOperationPort,
    ModelOperationPortDirection,
    ModelOperationSignature,
    ModelOperationVersion,
};

use crate::ai::model::signatures::{
    ModelSignature,
};

use crate::ai::model::validation::{
    ModelValidationIssue,
    ModelValidator,
};

use crate::frontend::semantic::types::ty::{
    QualifiedName,
    SemanticName,
    SemanticType,
};

// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Creates a canonical semantic name.
fn name(value: &str) -> SemanticName {
    SemanticName::new(value)
}

/// Creates a canonical qualified name.
fn qualified(value: &str) -> QualifiedName {
    QualifiedName::single(value)
}

/// Creates a model member identity.
fn member_id(value: &str) -> ModelMemberId {
    ModelMemberId::new(value)
        .expect("test member identity must be valid")
}

/// Creates an ordinary typed model member.
fn member(
    id: &str,
    member_name: &str,
) -> ModelMember {
    ModelMember::new(
        member_id(id),
        name(member_name),
        ModelMemberRole::Component,
        ModelMemberPayload::Typed {
            ty: SemanticType::Unit,
            initializer: None,
        },
    )
}

/// Creates a region member.
fn region(
    id: &str,
    region_name: &str,
) -> ModelMember {
    ModelMember::new(
        member_id(id),
        name(region_name),
        ModelMemberRole::Region,
        ModelMemberPayload::Block,
    )
}

/// Creates a directed connection.
fn connection(
    source: &str,
    destination: &str,
) -> ModelConnection {
    ModelConnection {
        source: ModelEndpoint::new(
            member_id(source),
            name("output"),
        ),
        destination: ModelEndpoint::new(
            member_id(destination),
            name("input"),
        ),
    }
}

/// Creates a basic valid model.
fn empty_model() -> ModelDefinition {
    ModelDefinition::new(qualified("TestModel"))
}

/// Creates a model containing a simple computational chain.
fn chain_model() -> ModelDefinition {
    empty_model()
        .with_member(member("input", "input"))
        .with_member(member("encoder", "encoder"))
        .with_member(member("decoder", "decoder"))
        .with_member(member("output", "output"))
        .with_connection(connection("input", "encoder"))
        .with_connection(connection("encoder", "decoder"))
        .with_connection(connection("decoder", "output"))
}

/// Creates a simple operation contract.
fn operation(
    id: &str,
) -> ModelOperation {
    ModelOperation::new(
        ModelOperationId::new(id)
            .expect("operation identity must be valid"),
        ModelOperationVersion::new(1, 0, 0),
        qualified("zamani.ai.compute"),
        ModelOperationSignature::new(
            Vec::new(),
            Vec::new(),
        ),
    )
}

// ============================================================================
// 1. MODEL CONSTRUCTION
// ============================================================================

#[test]
fn creates_a_valid_empty_model() {
    let model = empty_model();

    assert!(model.validate().is_ok());
    assert_eq!(model.member_count(), 0);
    assert_eq!(model.connection_count(), 0);
    assert_eq!(model.generic_parameter_count(), 0);
}

#[test]
fn creates_a_model_with_multiple_members() {
    let model = empty_model()
        .with_member(member("input", "input"))
        .with_member(member("hidden", "hidden"))
        .with_member(member("output", "output"));

    assert!(model.validate().is_ok());
    assert_eq!(model.member_count(), 3);
}

#[test]
fn preserves_model_member_source_order() {
    let model = empty_model()
        .with_member(member("first", "first"))
        .with_member(member("second", "second"))
        .with_member(member("third", "third"));

    let actual: Vec<&str> = model
        .members
        .iter()
        .map(|item| item.name.as_str())
        .collect();

    assert_eq!(
        actual,
        vec!["first", "second", "third"]
    );
}

#[test]
fn retrieves_members_using_canonical_identity() {
    let model = empty_model()
        .with_member(member("encoder", "encoder"))
        .with_member(member("decoder", "decoder"));

    let encoder = model.member(&member_id("encoder"));

    assert!(encoder.is_some());

    assert_eq!(
        encoder.expect("encoder must exist").name.as_str(),
        "encoder"
    );

    assert!(model.member(&member_id("missing")).is_none());
}

#[test]
fn rejects_duplicate_member_identities() {
    let model = empty_model()
        .with_member(member("duplicate", "first"))
        .with_member(member("duplicate", "second"));

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::DuplicateMemberIdentity)
    );
}

#[test]
fn rejects_duplicate_member_names_in_the_same_scope() {
    let model = empty_model()
        .with_member(member("first", "encoder"))
        .with_member(member("second", "encoder"));

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::DuplicateMemberName)
    );
}

#[test]
fn permits_equal_member_names_in_different_regions() {
    let left = member_id("left");
    let right = member_id("right");

    let model = empty_model()
        .with_member(region("left", "left"))
        .with_member(region("right", "right"))
        .with_member(
            member("left_input", "input")
                .with_parent(left),
        )
        .with_member(
            member("right_input", "input")
                .with_parent(right),
        );

    assert!(model.validate().is_ok());
}

#[test]
fn rejects_members_with_unknown_parent_regions() {
    let model = empty_model()
        .with_member(
            member("child", "child")
                .with_parent(member_id("missing")),
        );

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::UnknownParentRegion)
    );
}

#[test]
fn rejects_non_region_members_as_parent_regions() {
    let model = empty_model()
        .with_member(member("parent", "parent"))
        .with_member(
            member("child", "child")
                .with_parent(member_id("parent")),
        );

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::ParentIsNotARegion)
    );
}

#[test]
fn accepts_nested_model_regions() {
    let outer = member_id("outer");
    let inner = member_id("inner");

    let model = empty_model()
        .with_member(region("outer", "outer"))
        .with_member(
            region("inner", "inner")
                .with_parent(outer.clone()),
        )
        .with_member(
            member("parameter", "parameter")
                .with_parent(inner),
        );

    assert!(model.validate().is_ok());
    assert_eq!(model.member_count(), 3);
}

// ============================================================================
// 2. MODEL CONNECTIONS
// ============================================================================

#[test]
fn accepts_valid_directed_connections() {
    let model = chain_model();

    assert!(model.validate().is_ok());
    assert_eq!(model.connection_count(), 3);
}

#[test]
fn rejects_unknown_connection_sources() {
    let model = empty_model()
        .with_member(member("destination", "destination"))
        .with_connection(connection("missing", "destination"));

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::UnknownConnectionSource)
    );
}

#[test]
fn rejects_unknown_connection_destinations() {
    let model = empty_model()
        .with_member(member("source", "source"))
        .with_connection(connection("source", "missing"));

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::UnknownConnectionDestination)
    );
}

#[test]
fn rejects_connections_from_a_port_to_itself() {
    let model = empty_model()
        .with_member(member("node", "node"))
        .with_connection(connection("node", "node"));

    assert_eq!(
        model.validate(),
        Err(ModelDefinitionError::SelfConnection)
    );
}

#[test]
fn preserves_parallel_connections() {
    let model = empty_model()
        .with_member(member("source", "source"))
        .with_member(member("destination", "destination"))
        .with_connection(connection("source", "destination"))
        .with_connection(connection("source", "destination"));

    assert!(model.validate().is_ok());
    assert_eq!(model.connection_count(), 2);

    let graph = ModelGraph::new(&model)
        .expect("parallel connections are valid");

    assert_eq!(graph.edge_count(), 2);
    assert_eq!(
        graph.out_degree(&member_id("source")),
        Some(2)
    );
}

// ============================================================================
// 3. GRAPH CONSTRUCTION AND INDEXING
// ============================================================================

#[test]
fn constructs_graph_from_canonical_model() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid model must produce a graph");

    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 3);
    assert!(!graph.is_empty());
    assert!(!graph.has_no_edges());
}

#[test]
fn constructs_an_empty_graph() {
    let model = empty_model();

    let graph = ModelGraph::new(&model)
        .expect("empty valid model must produce a graph");

    assert!(graph.is_empty());
    assert!(graph.has_no_edges());
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn graph_uses_canonical_member_identities() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert!(graph.node(&member_id("encoder")).is_some());
    assert!(graph.node(&member_id("missing")).is_none());
}

#[test]
fn graph_preserves_node_source_order() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let actual: Vec<&str> = graph
        .nodes()
        .map(|node| node.name.as_str())
        .collect();

    assert_eq!(
        actual,
        vec!["input", "encoder", "decoder", "output"]
    );
}

#[test]
fn graph_preserves_edge_source_order() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let actual: Vec<(&str, &str)> = graph
        .edges()
        .map(|edge| {
            (
                edge.source.member.as_str(),
                edge.destination.member.as_str(),
            )
        })
        .collect();

    assert_eq!(
        actual,
        vec![
            ("input", "encoder"),
            ("encoder", "decoder"),
            ("decoder", "output"),
        ]
    );
}

#[test]
fn graph_calculates_incoming_and_outgoing_degrees() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(
        graph.in_degree(&member_id("input")),
        Some(0)
    );

    assert_eq!(
        graph.out_degree(&member_id("input")),
        Some(1)
    );

    assert_eq!(
        graph.in_degree(&member_id("encoder")),
        Some(1)
    );

    assert_eq!(
        graph.out_degree(&member_id("encoder")),
        Some(1)
    );

    assert_eq!(
        graph.in_degree(&member_id("output")),
        Some(1)
    );

    assert_eq!(
        graph.out_degree(&member_id("output")),
        Some(0)
    );
}

#[test]
fn graph_returns_none_for_unknown_member_degrees() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(
        graph.in_degree(&member_id("unknown")),
        None
    );

    assert_eq!(
        graph.out_degree(&member_id("unknown")),
        None
    );
}

// ============================================================================
// 4. GRAPH TRAVERSAL
// ============================================================================

#[test]
fn finds_all_reachable_members() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let reachable = graph
        .reachable_from(&member_id("input"))
        .expect("input exists");

    let actual: Vec<&str> = reachable
        .iter()
        .map(|node| node.name.as_str())
        .collect();

    assert_eq!(
        actual,
        vec!["input", "encoder", "decoder", "output"]
    );
}

#[test]
fn reachability_includes_the_starting_member() {
    let model = empty_model()
        .with_member(member("isolated", "isolated"));

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let reachable = graph
        .reachable_from(&member_id("isolated"))
        .expect("member exists");

    assert_eq!(reachable.len(), 1);
    assert_eq!(reachable[0].name.as_str(), "isolated");
}

#[test]
fn rejects_unknown_graph_traversal_start() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(
        graph.reachable_from(&member_id("missing")),
        Err(ModelGraphError::UnknownMember(
            member_id("missing")
        ))
    );
}

#[test]
fn finds_ancestors_using_reverse_traversal() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let ancestors = graph
        .ancestors_of(&member_id("output"))
        .expect("output exists");

    let actual: Vec<&str> = ancestors
        .iter()
        .map(|node| node.name.as_str())
        .collect();

    assert_eq!(
        actual,
        vec!["output", "decoder", "encoder", "input"]
    );
}

#[test]
fn handles_shared_dependencies() {
    let model = empty_model()
        .with_member(member("input", "input"))
        .with_member(member("left", "left"))
        .with_member(member("right", "right"))
        .with_member(member("merge", "merge"))
        .with_connection(connection("input", "left"))
        .with_connection(connection("input", "right"))
        .with_connection(connection("left", "merge"))
        .with_connection(connection("right", "merge"));

    let graph = ModelGraph::new(&model)
        .expect("shared dependencies are valid");

    let reachable = graph
        .reachable_from(&member_id("input"))
        .expect("input exists");

    assert_eq!(reachable.len(), 4);

    assert_eq!(
        graph.in_degree(&member_id("merge")),
        Some(2)
    );
}

// ============================================================================
// 5. TOPOLOGICAL ORDERING AND CYCLES
// ============================================================================

#[test]
fn calculates_topological_order_for_acyclic_models() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let ordered = graph
        .topological_order()
        .expect("chain must be acyclic");

    let positions: Vec<&str> = ordered
        .iter()
        .map(|node| node.id.as_str())
        .collect();

    let input = positions
        .iter()
        .position(|id| *id == "input")
        .expect("input exists");

    let encoder = positions
        .iter()
        .position(|id| *id == "encoder")
        .expect("encoder exists");

    let decoder = positions
        .iter()
        .position(|id| *id == "decoder")
        .expect("decoder exists");

    let output = positions
        .iter()
        .position(|id| *id == "output")
        .expect("output exists");

    assert!(input < encoder);
    assert!(encoder < decoder);
    assert!(decoder < output);
}

#[test]
fn detects_directed_cycles_without_mutating_model() {
    let model = empty_model()
        .with_member(member("first", "first"))
        .with_member(member("second", "second"))
        .with_connection(connection("first", "second"))
        .with_connection(connection("second", "first"));

    // A cycle is a graph property. It does not invalidate the
    // structural model declaration by itself.
    assert!(model.validate().is_ok());

    let graph = ModelGraph::new(&model)
        .expect("cyclic graph representation is permitted");

    assert!(graph.has_cycle());
    assert!(graph.topological_order().is_err());
}

#[test]
fn identifies_disconnected_graph_components() {
    let model = empty_model()
        .with_member(member("first", "first"))
        .with_member(member("second", "second"))
        .with_member(member("third", "third"))
        .with_connection(connection("first", "second"));

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(graph.node_count(), 3);

    let order = graph
        .topological_order()
        .expect("disconnected acyclic graph");

    assert_eq!(order.len(), 3);
}

#[test]
fn identifies_graph_sources_and_sinks() {
    let model = chain_model();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    let sources: Vec<&str> = graph
        .sources()
        .map(|node| node.id.as_str())
        .collect();

    let sinks: Vec<&str> = graph
        .sinks()
        .map(|node| node.id.as_str())
        .collect();

    assert_eq!(sources, vec!["input"]);
    assert_eq!(sinks, vec!["output"]);
}

#[test]
fn identifies_isolated_model_members() {
    let model = empty_model()
        .with_member(member("connected", "connected"))
        .with_member(member("isolated", "isolated"))
        .with_member(member("destination", "destination"))
        .with_connection(connection("connected", "destination"));

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(
        graph.is_isolated(&member_id("isolated")),
        Some(true)
    );

    assert_eq!(
        graph.is_isolated(&member_id("connected")),
        Some(false)
    );

    assert_eq!(
        graph.is_isolated(&member_id("missing")),
        None
    );
}

// ============================================================================
// 6. MODEL OPERATION CONTRACTS
// ============================================================================

#[test]
fn validates_individual_model_operations() {
    let operation = operation("compute");

    assert!(operation.validate().is_ok());
}

#[test]
fn validates_multiple_distinct_model_operations() {
    let model = chain_model();

    let operations = vec![
        operation("encode"),
        operation("transform"),
        operation("decode"),
    ];

    let report = ModelValidator::new()
        .validate_with_operations(&model, &operations)
        .expect("validation report must be produced");

    assert!(report.is_valid());
    assert_eq!(report.issue_count(), 0);
}

#[test]
fn rejects_duplicate_operation_identities() {
    let model = chain_model();

    let operations = vec![
        operation("compute"),
        operation("compute"),
    ];

    let report = ModelValidator::new()
        .validate_with_operations(&model, &operations)
        .expect("validation report must be produced");

    assert!(!report.is_valid());

    assert!(report.issues().iter().any(|issue| {
        matches!(
            issue,
            ModelValidationIssue::DuplicateOperationIdentity {
                first_index: 0,
                duplicate_index: 1,
            }
        )
    }));
}

#[test]
fn preserves_operation_declaration_order() {
    let model = chain_model();

    let operations = vec![
        operation("first"),
        operation("second"),
        operation("third"),
    ];

    let report = ModelValidator::new()
        .validate_with_operations(&model, &operations)
        .expect("validation report must be produced");

    assert!(report.is_valid());

    // Operation order is supplied by the caller and must not
    // be silently reordered by structural validation.
    assert_eq!(
        operations[0].id.as_str(),
        "first"
    );

    assert_eq!(
        operations[1].id.as_str(),
        "second"
    );

    assert_eq!(
        operations[2].id.as_str(),
        "third"
    );
}

// ============================================================================
// 7. MODEL SIGNATURES
// ============================================================================

#[test]
fn constructs_an_empty_model_signature() {
    let signature = ModelSignature::new(
        Vec::new(),
        Vec::new(),
    );

    assert!(signature.validate().is_ok());
    assert_eq!(signature.input_count(), 0);
    assert_eq!(signature.output_count(), 0);
}

#[test]
fn preserves_model_signature_order() {
    let signature = ModelSignature::new(
        Vec::new(),
        Vec::new(),
    );

    assert!(signature.validate().is_ok());
}

#[test]
fn converts_an_existing_operation_signature() {
    let operation_signature = ModelOperationSignature::new(
        Vec::new(),
        Vec::new(),
    );

    let signature = ModelSignature::from_operation_signature(
        operation_signature,
    );

    assert!(signature.validate().is_ok());
}

// ============================================================================
// 8. IMMUTABILITY AND DETERMINISM
// ============================================================================

#[test]
fn graph_construction_does_not_mutate_model_definition() {
    let model = chain_model();

    let original_members = model.members.clone();
    let original_connections = model.connections.clone();

    let graph = ModelGraph::new(&model)
        .expect("valid graph");

    assert_eq!(model.members, original_members);
    assert_eq!(model.connections, original_connections);

    assert_eq!(graph.node_count(), original_members.len());
    assert_eq!(graph.edge_count(), original_connections.len());
}

#[test]
fn repeated_graph_construction_is_deterministic() {
    let model = chain_model();

    let first = ModelGraph::new(&model)
        .expect("first graph");

    let second = ModelGraph::new(&model)
        .expect("second graph");

    let first_order: Vec<String> = first
        .topological_order()
        .expect("first ordering")
        .iter()
        .map(|node| node.id.as_str().to_owned())
        .collect();

    let second_order: Vec<String> = second
        .topological_order()
        .expect("second ordering")
        .iter()
        .map(|node| node.id.as_str().to_owned())
        .collect();

    assert_eq!(first_order, second_order);
}

#[test]
fn repeated_validation_produces_equivalent_reports() {
    let model = chain_model();

    let validator = ModelValidator::new();

    let first = validator
        .validate(&model)
        .expect("first validation");

    let second = validator
        .validate(&model)
        .expect("second validation");

    assert_eq!(first, second);
}

// ============================================================================
// 9. SCALABILITY AND POCO-REAF
// ============================================================================

#[test]
fn supports_dynamically_sized_model_definitions() {
    // This is a finite test workload, not a universal capacity.
    //
    // The model representation must derive its capacity from available
    // resources rather than a language-level MAX_MEMBERS constant.

    let member_count = 1_024usize;

    let mut model = empty_model();

    for index in 0..member_count {
        let id = format!("member_{index}");
        let member_name = format!("component_{index}");

        model = model.with_member(
            member(&id, &member_name),
        );
    }

    assert!(model.validate().is_ok());
    assert_eq!(model.member_count(), member_count);

    let graph = ModelGraph::new(&model)
        .expect("dynamically sized graph");

    assert_eq!(graph.node_count(), member_count);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn supports_dynamically_sized_model_graphs() {
    // A generated chain verifies that graph construction and traversal
    // do not rely on recursive call-stack depth.

    let member_count = 1_024usize;

    let mut model = empty_model();

    for index in 0..member_count {
        let id = format!("node_{index}");
        let member_name = format!("component_{index}");

        model = model.with_member(
            member(&id, &member_name),
        );
    }

    for index in 0..member_count.saturating_sub(1) {
        let source = format!("node_{index}");
        let destination = format!("node_{}", index + 1);

        model = model.with_connection(
            connection(&source, &destination),
        );
    }

    assert!(model.validate().is_ok());

    let graph = ModelGraph::new(&model)
        .expect("large graph construction");

    assert_eq!(graph.node_count(), member_count);
    assert_eq!(
        graph.edge_count(),
        member_count.saturating_sub(1)
    );

    let reachable = graph
        .reachable_from(&member_id("node_0"))
        .expect("iterative traversal");

    assert_eq!(reachable.len(), member_count);

    let ordered = graph
        .topological_order()
        .expect("generated chain is acyclic");

    assert_eq!(ordered.len(), member_count);
}

#[test]
fn supports_models_without_hardware_specific_metadata() {
    let model = chain_model();

    assert!(model.validate().is_ok());

    // The canonical model declaration does not require:
    //
    // - a CPU identifier;
    // - a GPU identifier;
    // - a QPU identifier;
    // - a physical node identifier;
    // - an accelerator allocation;
    // - a memory address.
    //
    // Those concerns belong to downstream realization.

    assert_eq!(model.member_count(), 4);
    assert_eq!(model.connection_count(), 3);
}

#[test]
fn model_semantics_remain_independent_of_execution_placement() {
    let model = chain_model();

    let first = model.clone();
    let second = model.clone();

    assert_eq!(first, second);
    assert!(first.validate().is_ok());
    assert!(second.validate().is_ok());
}

// ============================================================================
// 10. ERROR AND VALIDATION BOUNDARIES
// ============================================================================

#[test]
fn invalid_model_is_reported_without_false_success() {
    let model = empty_model()
        .with_member(member("duplicate", "first"))
        .with_member(member("duplicate", "second"));

    let report = ModelValidator::new()
        .validate(&model)
        .expect("validation report allocation");

    assert!(!report.is_valid());
    assert!(report.has_errors());
    assert_eq!(report.issue_count(), 1);
}

#[test]
fn valid_structure_does_not_claim_runtime_feasibility() {
    let model = chain_model();

    let report = ModelValidator::new()
        .validate(&model)
        .expect("structural validation");

    assert!(report.is_valid());

    // Structural validity does not prove:
    //
    // - target capability availability;
    // - numerical compatibility;
    // - memory feasibility;
    // - device availability;
    // - execution success.
    //
    // These checks remain downstream.

    assert_eq!(report.issue_count(), 0);
}

#[test]
fn invalid_model_graph_cannot_be_constructed() {
    let model = empty_model()
        .with_member(member("duplicate", "first"))
        .with_member(member("duplicate", "second"));

    assert!(matches!(
        ModelGraph::new(&model),
        Err(ModelGraphError::InvalidDefinition(
            ModelDefinitionError::DuplicateMemberIdentity
        ))
    ));
}

// ============================================================================
// END OF MODEL INTEGRATION TESTS
// ============================================================================
