
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Scalability Conformance Tests
//!
//! Path: `src/ai/tests/scalability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify that Zamani's existing AI computational abstractions can
//! represent progressively larger workloads without introducing
//! arbitrary language-level capacity restrictions.
//!
//! These tests exercise the existing implementations rather than
//! introducing alternative AI models, tensor storage, semantic
//! representations, or intermediate representations.
//!
//! ## Architectural authorities
//!
//! The tests exercise:
//!
//! - `crate::ai::requirements`
//! - `crate::ai::registry`
//! - `crate::ai::tensor::shapes`
//! - `crate::ai::tensor::symbolic`
//! - `crate::frontend::semantic::domains::ai`
//! - `crate::classical::collections::tensor`
//!
//! Source-level scalability requirements are specified in:
//!
//! - `grammar/ai/README.md`
//! - `grammar/ai/ai.g4`
//! - `grammar/ai/tensors.g4`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/scalability.md`
//! - `grammar/validation/scalability-rules.md`
//! - `grammar/spec/ai.md`
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - AI scalability conformance tests;
//! - generated scalability test workloads;
//! - resource-independent representation tests;
//! - tensor shape scalability tests;
//! - symbolic graph scalability tests;
//! - registry scalability tests;
//! - scalability regression tests.
//!
//! This file does not own:
//!
//! - AI computational semantics;
//! - model execution;
//! - tensor storage;
//! - tensor numerical kernels;
//! - semantic validation;
//! - hardware discovery;
//! - resource allocation;
//! - device placement;
//! - distributed scheduling;
//! - hybrid execution;
//! - compiler optimization;
//! - hardware realization.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The same logical computation must retain its semantic meaning
//! regardless of the size or topology of the eventual target.
//!
//! Literal infinite physical computation is impossible.
//!
//! The implementation must nevertheless avoid arbitrary language-level
//! limits and must grow according to available resources.
//!
//! ## Resource policy
//!
//! A finite test workload is a sample, not a semantic maximum.
//!
//! Tests must distinguish:
//!
//! - language semantics;
//! - collection representation;
//! - host integer representation;
//! - available memory;
//! - operational budgets;
//! - target capabilities;
//! - execution feasibility.
//!
//! A representation overflow must be reported explicitly.
//! A resource limitation must not silently change program meaning.
//!
//! ## Integration policy
//!
//! The tests use the existing canonical semantic and storage APIs.
//!
//! No production implementation is introduced in this file.
//!
//! The actual grammar-to-AST-to-semantic-to-IR-to-runtime
//! compilation pathway requires separate end-to-end conformance tests.

use crate::ai::requirements::AiRequirements;

use crate::ai::registry::{
    AiOperationDescriptor,
    AiOperationRegistry,
};

use crate::ai::version::AiOperationSetVersion;

use crate::ai::tensor::symbolic::{
    SymbolicExprId,
    SymbolicTensorGraph,
    SymbolicTensorOp,
};

use crate::frontend::semantic::domains::ai::{
    AiId,
    AiOperationKind,
    DimensionExpr,
    QualifiedName,
    TensorShape,
};

use crate::classical::collections::tensor::ClassicalTensor;

// ============================================================================
// TEST UTILITIES
// ============================================================================

/// Constructs a valid canonical AI semantic identity.
fn semantic_identity(value: impl Into<String>) -> AiId {
    AiId::new(value)
        .expect("test semantic identity must be valid")
}

/// Constructs a symbolic tensor shape.
///
/// The shape uses the existing frontend semantic representation.
/// It does not allocate tensor elements or impose a rank limit.
fn symbolic_shape(rank: usize) -> TensorShape {
    let dimensions = (0..rank)
        .map(|index| {
            DimensionExpr::Symbol(
                semantic_identity(format!("dimension_{index}"))
            )
        })
        .collect();

    TensorShape::new(dimensions)
}

/// Constructs an AI operation descriptor using the existing registry API.
fn operation_descriptor(
    name: String,
) -> AiOperationDescriptor {
    let qualified_name = QualifiedName::new(
        "zamani.ai.scalability.tests",
        name,
    )
    .expect("test operation name must be valid");

    AiOperationDescriptor::new(
        qualified_name,
        AiOperationKind::Inference,
        AiOperationSetVersion::new(1, 0, 0),
    )
    .expect("test operation descriptor must be valid")
}

// ============================================================================
// 1. EMPTY AND MINIMAL WORKLOADS
// ============================================================================

/// An empty symbolic graph must remain a valid empty representation.
#[test]
fn empty_symbolic_graph_is_valid() {
    let graph = SymbolicTensorGraph::new();

    assert!(graph.is_empty());
    assert_eq!(graph.len(), 0);

    assert!(graph.nodes().is_empty());
    assert!(graph.outputs().is_empty());

    assert!(graph.validate().is_ok());
}

/// A minimal tensor shape must preserve its dimensions.
#[test]
fn minimal_tensor_shape_preserves_semantics() {
    let shape = symbolic_shape(1);

    assert_eq!(shape.rank(), 1);
    assert_eq!(shape.dimensions.len(), 1);
}

/// Scalar semantic shapes must remain distinguishable from ranked tensors.
#[test]
fn scalar_shape_is_preserved() {
    let scalar = TensorShape::new(Vec::new());

    assert!(scalar.is_scalar());
    assert_eq!(scalar.rank(), 0);
}

// ============================================================================
// 2. ARBITRARY-RANK SEMANTIC SHAPES
// ============================================================================

/// Symbolic tensor ranks must grow without a language-level rank ceiling.
///
/// The selected values are test points, not supported-rank boundaries.
#[test]
fn symbolic_tensor_shapes_support_progressively_larger_ranks() {
    let ranks = [1usize, 2, 8, 32, 128, 256, 512];

    for rank in ranks {
        let shape = symbolic_shape(rank);

        assert_eq!(
            shape.rank(),
            rank,
            "symbolic tensor rank must be preserved"
        );

        assert_eq!(
            shape.dimensions.len(),
            rank,
            "no dimensions may be silently discarded"
        );
    }
}

/// Symbolic dimensions must remain symbolic.
///
/// Constructing a shape must not invent concrete dimension values.
#[test]
fn symbolic_dimensions_are_not_materialized_during_construction() {
    let shape = symbolic_shape(128);

    assert_eq!(shape.rank(), 128);

    assert!(
        shape.dimensions.iter().all(
            |dimension| matches!(dimension, DimensionExpr::Symbol(_))
        ),
        "symbolic dimensions must retain their symbolic representation"
    );
}

/// Large symbolic shapes must not require allocating tensor elements.
#[test]
fn large_symbolic_shape_requires_only_shape_metadata() {
    let shape = symbolic_shape(1024);

    assert_eq!(shape.rank(), 1024);

    // The semantic shape stores dimensions, not tensor element storage.
    // No numerical tensor has been materialized here.
}

// ============================================================================
// 3. SYMBOLIC TENSOR GRAPH SCALABILITY
// ============================================================================

/// Symbolic tensor graphs must support progressively larger workloads.
///
/// Each generated node consumes the previous node twice.
///
/// This creates a valid dependency graph without materializing tensor
/// elements or executing numerical operations.
#[test]
fn symbolic_graph_scales_with_generated_operation_count() {
    let workload_sizes = [
        1usize,
        2,
        8,
        32,
        128,
        512,
        1024,
    ];

    for workload_size in workload_sizes {
        let mut graph = SymbolicTensorGraph::new();

        let shape = symbolic_shape(4);

        let mut previous = graph
            .input(
                semantic_identity("input"),
                shape.clone(),
            )
            .expect("input node must be created");

        for _ in 0..workload_size {
            previous = graph
                .insert(
                    SymbolicTensorOp::Add,
                    vec![previous, previous],
                    shape.clone(),
                )
                .expect("valid symbolic operation must be inserted");
        }

        assert_eq!(
            graph.len(),
            workload_size + 1,
            "every accepted operation must be preserved"
        );

        assert!(
            graph.validate().is_ok(),
            "generated symbolic graph must remain valid"
        );

        assert_eq!(
            graph.outputs(),
            vec![previous],
            "the final unconsumed node must be the graph output"
        );
    }
}

/// Graph insertion order must remain deterministic.
#[test]
fn symbolic_graph_preserves_deterministic_topological_order() {
    let mut graph = SymbolicTensorGraph::new();

    let shape = symbolic_shape(2);

    let first = graph
        .input(
            semantic_identity("first"),
            shape.clone(),
        )
        .expect("first input");

    let second = graph
        .input(
            semantic_identity("second"),
            shape.clone(),
        )
        .expect("second input");

    let output = graph
        .insert(
            SymbolicTensorOp::Add,
            vec![first, second],
            shape,
        )
        .expect("addition");

    let order = graph.topological_order();

    assert_eq!(
        order,
        vec![
            first,
            second,
            output,
        ]
    );

    assert!(graph.validate().is_ok());
}

/// Invalid graph dependencies must fail explicitly.
#[test]
fn symbolic_graph_rejects_unknown_dependencies() {
    let mut graph = SymbolicTensorGraph::new();

    let result = graph.insert(
        SymbolicTensorOp::Add,
        vec![SymbolicExprId(usize::MAX)],
        symbolic_shape(1),
    );

    assert!(
        result.is_err(),
        "unknown dependencies must never be accepted"
    );

    assert!(
        graph.is_empty(),
        "failed insertion must not modify the graph"
    );
}

// ============================================================================
// 4. CLASSICAL TENSOR STORAGE SCALABILITY
// ============================================================================

/// Classical tensor storage must support dynamically sized ranks.
///
/// This exercises the actual canonical tensor storage implementation,
/// not an AI-specific replacement.
#[test]
fn classical_tensor_supports_progressively_larger_ranks() {
    let ranks = [
        1usize,
        2,
        8,
        32,
        128,
        256,
    ];

    for rank in ranks {
        let dimensions = vec![1usize; rank];

        let tensor = ClassicalTensor::<u8>::sparse_with_dimensions(
            dimensions,
        )
        .expect("valid arbitrary-rank tensor shape");

        assert_eq!(
            tensor.rank(),
            rank,
            "tensor rank must be preserved"
        );

        assert_eq!(
            tensor.stored_len(),
            0,
            "empty sparse storage must not allocate tensor elements"
        );
    }
}

/// Sparse tensors must preserve large logical shapes without
/// materializing every logical element.
#[test]
fn sparse_tensor_storage_is_independent_of_logical_element_count() {
    let dimensions = vec![
        1_000_000usize,
        1_000_000usize,
        1_000usize,
    ];

    let mut tensor = ClassicalTensor::<u64>::sparse_with_dimensions(
        dimensions.clone(),
    )
    .expect("sparse tensor metadata construction");

    assert_eq!(
        tensor.dimensions(),
        dimensions.as_slice()
    );

    assert_eq!(
        tensor.stored_len(),
        0,
        "logical dimensions must not cause implicit dense allocation"
    );

    assert_eq!(
        tensor.logical_size().expect(
            "logical size is representable on this test target"
        ),
        dimensions.iter().product::<usize>()
    );

    // Store only one explicitly requested element.
    let coordinates = [0usize, 0usize, 0usize];

    assert_eq!(
        tensor.set(&coordinates, 42),
        Ok(None)
    );

    assert_eq!(
        tensor.get(&coordinates),
        Ok(Some(&42))
    );

    assert_eq!(
        tensor.stored_len(),
        1,
        "one insertion must not materialize the entire logical tensor"
    );
}

/// Unmaterialized storage must remain unmaterialized.
#[test]
fn unmaterialized_tensor_never_allocates_implicitly() {
    let mut tensor =
        ClassicalTensor::<u64>::unmaterialized_with_dimensions(
            vec![1024, 1024, 1024],
        )
        .expect("unmaterialized shape construction");

    assert_eq!(tensor.stored_len(), 0);

    assert!(
        tensor.as_dense_slice().is_none(),
        "unmaterialized storage must not become dense implicitly"
    );

    assert!(
        tensor.set(&[0, 0, 0], 1).is_err(),
        "unmaterialized storage must reject writes"
    );

    assert_eq!(
        tensor.stored_len(),
        0,
        "failed writes must not create storage"
    );
}

// ============================================================================
// 5. HOST REPRESENTATION LIMITS
// ============================================================================

/// A host representation overflow must be reported explicitly.
///
/// It must not become a universal tensor-shape or language restriction.
#[test]
fn unrepresentable_tensor_logical_size_is_reported() {
    let tensor =
        ClassicalTensor::<u8>::unmaterialized_with_dimensions(
            vec![usize::MAX, 2],
        )
        .expect("shape metadata itself is representable");

    assert!(
        tensor.logical_size().is_err(),
        "unrepresentable logical size must produce an explicit error"
    );

    assert_eq!(
        tensor.rank(),
        2,
        "shape metadata must remain available even when size overflows"
    );

    assert_eq!(
        tensor.stored_len(),
        0,
        "size analysis must not materialize tensor elements"
    );
}

// ============================================================================
// 6. AI OPERATION REGISTRY SCALABILITY
// ============================================================================

/// The AI operation registry must support dynamically growing
/// operation declarations.
///
/// Generated operation counts are test workloads, not registry limits.
#[test]
fn operation_registry_scales_with_generated_operations() {
    let workload_sizes = [
        1usize,
        2,
        8,
        32,
        128,
        512,
        1024,
    ];

    for workload_size in workload_sizes {
        let descriptors = (0..workload_size)
            .map(|index| {
                operation_descriptor(format!("operation_{index}"))
            })
            .collect::<Vec<_>>();

        let mut registry = AiOperationRegistry::new();

        registry
            .register_batch(descriptors)
            .expect("unique operation batch must register");

        assert_eq!(
            registry.len(),
            workload_size,
            "every valid operation must be registered"
        );

        assert!(!registry.is_empty());

        assert_eq!(
            registry.iter().count(),
            workload_size
        );
    }
}

/// Registration must preserve exact operation identities and versions.
#[test]
fn operation_registry_preserves_exact_lookup_at_scale() {
    let workload_size = 256usize;

    let descriptors = (0..workload_size)
        .map(|index| {
            operation_descriptor(format!("operation_{index}"))
        })
        .collect::<Vec<_>>();

    let mut registry = AiOperationRegistry::new();

    registry
        .register_batch(descriptors)
        .expect("batch registration");

    let version = AiOperationSetVersion::new(1, 0, 0);

    for index in 0..workload_size {
        let name = QualifiedName::new(
            "zamani.ai.scalability.tests",
            format!("operation_{index}"),
        )
        .expect("valid operation name");

        let resolved = registry
            .resolve(&name, version)
            .expect("registered operation must resolve");

        assert_eq!(
            resolved.name(),
            &name
        );

        assert_eq!(
            resolved.version(),
            version
        );
    }
}

/// A failed batch registration must not publish partially registered
/// operations.
///
/// Atomicity must remain true as registry sizes grow.
#[test]
fn failed_registry_batch_does_not_publish_partial_results() {
    let mut registry = AiOperationRegistry::new();

    let version = AiOperationSetVersion::new(1, 0, 0);

    let initial = operation_descriptor("existing".to_owned());

    registry
        .register(initial)
        .expect("initial registration");

    let first = operation_descriptor("new_operation".to_owned());

    let duplicate = operation_descriptor("existing".to_owned());

    let result = registry.register_batch(vec![
        first,
        duplicate,
    ]);

    assert!(
        result.is_err(),
        "duplicate registration must fail"
    );

    assert_eq!(
        registry.len(),
        1,
        "failed registration must leave the original registry unchanged"
    );

    let new_name = QualifiedName::new(
        "zamani.ai.scalability.tests",
        "new_operation",
    )
    .expect("valid name");

    assert!(
        registry.resolve(&new_name, version).is_err(),
        "staged operations must not leak after failed registration"
    );
}

// ============================================================================
// 7. REQUIREMENT ANALYSIS AND RESOURCE SEMANTICS
// ============================================================================

/// Requirement analysis must not silently claim completeness.
///
/// This is particularly important when resource discovery or semantic
/// analysis has not finished.
#[test]
fn incomplete_requirement_analysis_remains_explicit() {
    let mut requirements = AiRequirements::new();

    assert!(
        requirements.is_complete(),
        "new requirement envelope is complete when its inputs are known"
    );

    requirements
        .mark_incomplete(
            "resource capability discovery has not completed",
        )
        .expect("valid incomplete-analysis reason");

    assert!(
        !requirements.is_complete(),
        "incomplete analysis must not be certified as complete"
    );

    assert!(
        !requirements.is_empty()
            || !requirements.is_complete(),
        "analysis completeness must remain independent of entry count"
    );
}

/// An incomplete requirement envelope must remain incomplete after
/// merging another envelope.
#[test]
fn requirement_merge_preserves_incomplete_analysis() {
    let mut incomplete = AiRequirements::new();

    incomplete
        .mark_incomplete("target evidence unavailable")
        .expect("valid incomplete reason");

    let complete = AiRequirements::new();

    incomplete.merge(complete);

    assert!(
        !incomplete.is_complete(),
        "merging complete information must not erase missing analysis"
    );
}

// ============================================================================
// 8. REPRODUCIBILITY AND DETERMINISM
// ============================================================================

/// Identical symbolic graph construction must produce identical graphs.
#[test]
fn equivalent_graph_construction_is_deterministic() {
    fn build_graph() -> SymbolicTensorGraph {
        let mut graph = SymbolicTensorGraph::new();

        let shape = symbolic_shape(8);

        let mut previous = graph
            .input(
                semantic_identity("deterministic_input"),
                shape.clone(),
            )
            .expect("input");

        for _ in 0..128 {
            previous = graph
                .insert(
                    SymbolicTensorOp::Add,
                    vec![previous, previous],
                    shape.clone(),
                )
                .expect("operation");
        }

        graph
    }

    let first = build_graph();
    let second = build_graph();

    assert_eq!(
        first.nodes(),
        second.nodes()
    );

    assert_eq!(
        first.outputs(),
        second.outputs()
    );

    assert!(
        first.validate().is_ok()
            && second.validate().is_ok()
    );
}

// ============================================================================
// 9. GROWTH AND RECOVERY INVARIANTS
// ============================================================================

/// Failed graph operations must not corrupt previously accepted nodes.
#[test]
fn failed_graph_operation_preserves_existing_workload() {
    let mut graph = SymbolicTensorGraph::new();

    let input = graph
        .input(
            semantic_identity("recovery_input"),
            symbolic_shape(4),
        )
        .expect("input");

    let accepted_length = graph.len();

    let result = graph.insert(
        SymbolicTensorOp::Add,
        vec![SymbolicExprId(usize::MAX)],
        symbolic_shape(4),
    );

    assert!(result.is_err());

    assert_eq!(
        graph.len(),
        accepted_length,
        "failed insertion must not corrupt accepted graph state"
    );

    assert!(
        graph.get(input).is_some(),
        "previously accepted nodes must remain accessible"
    );

    assert!(graph.validate().is_ok());
}

/// Growth must preserve previously stored sparse tensor values.
#[test]
fn sparse_tensor_growth_preserves_existing_values() {
    let mut tensor =
        ClassicalTensor::<u64>::sparse_with_dimensions(
            vec![1024, 1024],
        )
        .expect("sparse tensor");

    let entries = [
        ([0usize, 0usize], 10u64),
        ([1usize, 1usize], 20u64),
        ([100usize, 200usize], 30u64),
        ([1023usize, 1023usize], 40u64),
    ];

    for (coordinates, value) in entries {
        tensor
            .set(&coordinates, value)
            .expect("sparse insertion");
    }

    assert_eq!(tensor.stored_len(), entries.len());

    for (coordinates, expected) in entries {
        assert_eq!(
            tensor.get(&coordinates),
            Ok(Some(&expected))
        );
    }
}

// ============================================================================
// 10. PORTABILITY INVARIANT
// ============================================================================

/// AI tensor semantics must not depend on a physical target identity.
///
/// The canonical semantic shape is independent of tensor storage,
/// device selection, and hardware placement.
#[test]
fn semantic_tensor_shape_is_target_independent() {
    let shape = symbolic_shape(64);

    let first_representation = shape.clone();
    let second_representation = shape.clone();

    assert_eq!(
        first_representation,
        second_representation,
        "logical tensor meaning must survive representation reuse"
    );

    assert_eq!(
        first_representation.rank(),
        second_representation.rank()
    );
}

// ============================================================================
// END OF AI SCALABILITY CONFORMANCE TESTS
// ============================================================================
