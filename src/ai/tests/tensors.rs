
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Tests
//!
//! Path: src/ai/tests/tensors.rs
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the behavioral contracts of the existing AI tensor subsystem.
//!
//! This module tests:
//!
//! - Canonical semantic tensor shapes.
//! - Arbitrary-rank and dynamic shape descriptions.
//! - Symbolic and unresolved dimensions.
//! - Shape compatibility and validation.
//! - Logical tensor layouts.
//! - Numerical data type semantics.
//! - Tensor operation contracts.
//! - Symbolic tensor graphs.
//! - Classical tensor interoperability.
//! - Conversion errors and information preservation.
//!
//! ## Architectural ownership
//!
//! These tests exercise existing authoritative implementations:
//!
//! - frontend::semantic::domains::ai:
//!   canonical DimensionExpr, TensorShape, AiId and AiOrigin.
//!
//! - ai::tensor:
//!   AI tensor descriptions and interoperability.
//!
//! - classical::collections::tensor:
//!   canonical classical tensor storage.
//!
//! - classical::collections::array:
//!   canonical classical array shape.
//!
//! - zuir::dialects::tensor:
//!   future universal tensor IR integration boundary.
//!
//! - grammar/ai/tensors.g4:
//!   canonical tensor source grammar.
//!
//! This test module does not introduce another tensor implementation,
//! semantic type system, tensor storage system, or compiler IR.
//!
//! ## Scalability
//!
//! There is no language-level maximum tensor rank, dimension count,
//! tensor count, operation count, or model size.
//!
//! Tests use representative dynamically constructed workloads.
//! They do not establish universal capacity limits.
//!
//! Actual materialization remains subject to available resources.
//!
//! ## Integration
//!
//! The canonical path is:
//!
//! grammar/ai/tensors.g4
//!     -> canonical frontend AST
//!     -> frontend semantic analysis
//!     -> ai::tensor
//!     -> zuir::dialects::tensor
//!     -> compiler lowering
//!     -> runtime / target realization
//!
//! Classical interoperability uses:
//!
//! ai::tensor::interoperability
//!     -> classical::collections::array
//!     -> classical::collections::tensor
//!
//! This file tests the currently implemented Rust boundaries.
//! It does not claim that the complete grammar-to-runtime pathway
//! has already been integrated.

use crate::ai::tensor::{
    self,
    classify_shape_analysis,
    compatibility as _,
    from_classical_shape,
    to_classical_shape,
    validate_classical_tensor,
    validate_conversion_report,
    validate_operation_structure,
    AiTensorDType,
    ByteOrder,
    ConversionFidelity,
    ConversionReport,
    DType,
    DTypeCompatibility,
    DTypeError,
    DenseOrder,
    DimensionKnowledge,
    DimensionResolver,
    DistributedLayout,
    InteropDimensionKnowledge,
    InteropDimensionResolver,
    IntegerSignedness,
    LayoutError,
    LayoutValidation,
    LayoutValidationContext,
    NumericCategory,
    PartitionStrategy,
    Representation,
    ShapeBuilder,
    ShapeCompatibility,
    ShapeCompatibilityAnalyzer,
    ShapeConstraint,
    ShapeValidator,
    SparseFormat,
    SparseLayout,
    StrideValue,
    SymbolicTensorGraph,
    SymbolicTensorOp,
    TensorAttribute,
    TensorAttributeValue,
    TensorCapabilityRequirement,
    TensorDescriptor,
    TensorLayout,
    TensorOperation,
    TensorOperationEffect,
    TensorOperationId,
    TensorOperationIssue,
    TensorShape,
    TensorValueContract,
    TensorValueId,
};

use crate::classical::collections::tensor::ClassicalTensor;
use crate::frontend::semantic::domains::ai::{
    AiId,
    AiOrigin,
    DimensionExpr,
};

// ============================================================================
// TEST SUPPORT
// ============================================================================

/// Resolves constant dimensions while preserving unresolved symbolic
/// dimensions.
///
/// Production semantic adapters must delegate to the canonical semantic
/// constraint infrastructure rather than treating this test resolver
/// as a production solver.
#[derive(Clone, Copy, Debug, Default)]
struct TestDimensionResolver;

impl DimensionResolver for TestDimensionResolver {
    fn resolve_dimension(
        &self,
        dimension: &DimensionExpr,
    ) -> DimensionKnowledge {
        match dimension {
            DimensionExpr::Constant(value) => {
                DimensionKnowledge::Known(*value)
            }

            _ => DimensionKnowledge::Unknown,
        }
    }

    fn prove_equal(
        &self,
        left: &DimensionExpr,
        right: &DimensionExpr,
    ) -> Option<bool> {
        if left == right {
            return Some(true);
        }

        match (
            self.resolve_dimension(left),
            self.resolve_dimension(right),
        ) {
            (
                DimensionKnowledge::Known(left),
                DimensionKnowledge::Known(right),
            ) => Some(left == right),

            _ => None,
        }
    }
}

impl InteropDimensionResolver for TestDimensionResolver {
    fn resolve(
        &self,
        expression: &DimensionExpr,
    ) -> InteropDimensionKnowledge {
        match expression {
            DimensionExpr::Constant(value) => {
                InteropDimensionKnowledge::Known(*value)
            }

            _ => InteropDimensionKnowledge::Unknown,
        }
    }
}

fn constant_shape(dimensions: &[u128]) -> TensorShape {
    ShapeBuilder::from_constants(dimensions.to_vec())
}

fn symbolic_shape(identity: &str, trailing: &[u128]) -> TensorShape {
    let symbol = AiId::new(identity)
        .expect("test symbol identity must be valid");

    let mut dimensions = Vec::with_capacity(trailing.len() + 1);

    dimensions.push(DimensionExpr::Symbol(symbol));

    dimensions.extend(
        trailing
            .iter()
            .copied()
            .map(DimensionExpr::Constant),
    );

    TensorShape::new(dimensions)
}

fn tensor_descriptor(shape: TensorShape) -> TensorDescriptor {
    TensorDescriptor::new(
        shape,
        DType::Bool,
        TensorLayout::row_major(),
        AiOrigin::default(),
    )
}

fn tensor_value(
    identity: &str,
    shape: TensorShape,
) -> TensorValueContract {
    TensorValueContract::new(
        TensorValueId::new(identity)
            .expect("test tensor identity must be valid"),
        shape,
        AiTensorDType::Bool,
    )
}

// ============================================================================
// CANONICAL SHAPE TESTS
// ============================================================================

#[test]
fn tensor_shapes_reuse_canonical_semantic_representation() {
    let shape = TensorShape::new(vec![
        DimensionExpr::Constant(4),
        DimensionExpr::Constant(8),
        DimensionExpr::Constant(16),
    ]);

    assert_eq!(shape.rank(), 3);

    assert_eq!(
        shape.dimensions,
        vec![
            DimensionExpr::Constant(4),
            DimensionExpr::Constant(8),
            DimensionExpr::Constant(16),
        ]
    );

    assert!(tensor::is_fully_static(&shape));
    assert!(!tensor::is_symbolic(&shape));
}

#[test]
fn scalar_shape_has_zero_rank() {
    let shape = TensorShape::new(Vec::new());

    assert_eq!(tensor::rank(&shape), 0);
    assert!(tensor::is_scalar(&shape));
    assert!(tensor::is_fully_static(&shape));
}

#[test]
fn zero_length_dimension_is_preserved_as_shape_metadata() {
    let shape = constant_shape(&[8, 0, 16]);

    assert_eq!(shape.rank(), 3);

    assert_eq!(
        shape.dimensions[1],
        DimensionExpr::Constant(0)
    );

    assert!(ShapeValidator::validate(
        &shape,
        &TestDimensionResolver,
    )
    .is_ok());
}

#[test]
fn symbolic_dimensions_are_preserved_without_materialization() {
    let shape = symbolic_shape("batch", &[128, 64]);

    assert_eq!(shape.rank(), 3);

    assert!(tensor::is_symbolic(&shape));
    assert!(!tensor::is_fully_static(&shape));

    assert!(matches!(
        &shape.dimensions[0],
        DimensionExpr::Symbol(_)
    ));
}

#[test]
fn dynamically_constructed_rank_is_not_restricted_by_ai_tensor_api() {
    let dimensions = (0_u128..257)
        .map(DimensionExpr::Constant)
        .collect::<Vec<_>>();

    let shape = TensorShape::new(dimensions);

    assert_eq!(shape.rank(), 257);
    assert_eq!(
        shape.dimensions.first(),
        Some(&DimensionExpr::Constant(0))
    );
    assert_eq!(
        shape.dimensions.last(),
        Some(&DimensionExpr::Constant(256))
    );
}

#[test]
fn shape_builder_preserves_arbitrary_dimension_values() {
    let shape = ShapeBuilder::from_constants(vec![
        u128::MAX,
        1,
        0,
    ]);

    assert_eq!(shape.rank(), 3);

    assert_eq!(
        shape.dimensions[0],
        DimensionExpr::Constant(u128::MAX)
    );
}

// ============================================================================
// SHAPE COMPATIBILITY AND VALIDATION
// ============================================================================

#[test]
fn equal_static_shapes_are_compatible() {
    let left = constant_shape(&[4, 8, 16]);
    let right = constant_shape(&[4, 8, 16]);

    let result = ShapeCompatibilityAnalyzer::equal(
        &left,
        &right,
        &TestDimensionResolver,
    );

    assert!(result.is_compatible());
    assert!(!result.is_incompatible());
}

#[test]
fn different_static_shapes_are_incompatible() {
    let left = constant_shape(&[4, 8]);
    let right = constant_shape(&[4, 16]);

    let result = ShapeCompatibilityAnalyzer::equal(
        &left,
        &right,
        &TestDimensionResolver,
    );

    assert!(result.is_incompatible());
}

#[test]
fn different_ranks_are_incompatible() {
    let left = constant_shape(&[4, 8]);
    let right = constant_shape(&[4, 8, 16]);

    let result = ShapeCompatibilityAnalyzer::equal(
        &left,
        &right,
        &TestDimensionResolver,
    );

    assert!(result.is_incompatible());
}

#[test]
fn unresolved_symbolic_shape_is_not_mistaken_for_concrete_shape() {
    let shape = symbolic_shape("dynamic_batch", &[64]);

    assert!(ShapeValidator::validate(
        &shape,
        &TestDimensionResolver,
    )
    .is_ok());

    assert!(
        ShapeValidator::concrete_dimensions(
            &shape,
            &TestDimensionResolver,
        )
        .is_err()
    );
}

#[test]
fn concrete_dimensions_are_checked_before_conversion() {
    let shape = constant_shape(&[2, 3, 5]);

    let dimensions = ShapeValidator::concrete_dimensions(
        &shape,
        &TestDimensionResolver,
    )
    .expect("constant dimensions must resolve");

    assert_eq!(dimensions, vec![2, 3, 5]);
}

#[test]
fn symbolic_shape_analysis_preserves_unresolved_constraints() {
    let left = symbolic_shape("batch_left", &[64]);
    let right = symbolic_shape("batch_right", &[64]);

    let result = ShapeCompatibilityAnalyzer::equal(
        &left,
        &right,
        &TestDimensionResolver,
    );

    assert!(result.requires_resolution());

    assert!(!result.constraints().is_empty());
}

#[test]
fn shape_validation_does_not_rewrite_symbolic_dimensions() {
    let shape = symbolic_shape("sequence_length", &[256]);
    let original = shape.clone();

    ShapeValidator::validate(
        &shape,
        &TestDimensionResolver,
    )
    .expect("unresolved dimensions are not automatically invalid");

    assert_eq!(shape, original);
}

// ============================================================================
// LOGICAL LAYOUT TESTS
// ============================================================================

#[test]
fn row_major_layout_is_device_independent() {
    let layout = TensorLayout::row_major();

    assert_eq!(layout.kind_name(), "dense");
}

#[test]
fn column_major_layout_is_distinct_from_row_major() {
    let row_major = TensorLayout::row_major();
    let column_major = TensorLayout::column_major();

    assert_ne!(row_major, column_major);

    assert_eq!(
        column_major.kind_name(),
        "dense"
    );
}

#[test]
fn strided_layout_validates_against_canonical_rank() {
    let layout = TensorLayout::strided(vec![
        StrideValue::Known(8),
        StrideValue::Known(1),
    ]);

    assert_eq!(
        layout.validate(LayoutValidationContext::known_rank(2)),
        Ok(LayoutValidation::Valid)
    );
}

#[test]
fn strided_layout_rejects_rank_mismatch() {
    let layout = TensorLayout::strided(vec![
        StrideValue::Known(8),
    ]);

    assert!(matches!(
        layout.validate(LayoutValidationContext::known_rank(2)),
        Err(LayoutError::RankMismatch { .. })
    ));
}

#[test]
fn symbolic_strides_remain_unresolved() {
    let layout = TensorLayout::strided(vec![
        StrideValue::Symbolic("stride_batch".to_owned()),
    ]);

    let result = layout
        .validate(LayoutValidationContext::known_rank(1))
        .expect("symbolic stride metadata is structurally valid");

    assert!(matches!(
        result,
        LayoutValidation::Unresolved(_)
    ));
}

#[test]
fn sparse_layout_preserves_its_declared_format() {
    let layout = TensorLayout::sparse(
        SparseLayout::new(
            SparseFormat::Coordinate,
            None,
        ),
    );

    assert_eq!(layout.kind_name(), "sparse");
}

#[test]
fn distributed_layout_describes_logical_partitioning_not_devices() {
    let layout = TensorLayout::distributed(
        DistributedLayout::new(
            PartitionStrategy::Replicated,
            true,
        ),
    );

    assert_eq!(layout.kind_name(), "distributed");
}

#[test]
fn unknown_layout_rank_is_reported_as_unresolved() {
    let layout = TensorLayout::strided(vec![
        StrideValue::Known(1),
        StrideValue::Known(4),
    ]);

    let result = layout
        .validate(LayoutValidationContext::unresolved_rank())
        .expect("unknown rank must not be treated as invalid");

    assert!(matches!(
        result,
        LayoutValidation::Unresolved(_)
    ));
}

// ============================================================================
// NUMERICAL TYPE TESTS
// ============================================================================

#[test]
fn boolean_dtype_preserves_boolean_semantics() {
    let dtype = DType::Bool;

    assert_eq!(dtype.category(), NumericCategory::Boolean);
    assert_eq!(dtype.bit_width(), Some(1));
}

#[test]
fn arbitrary_integer_width_is_represented_without_machine_assumptions() {
    let dtype = tensor::dtypes::unsigned_integer(257)
        .expect("nonzero integer width must be representable");

    assert_eq!(dtype.bit_width(), Some(257));
    assert!(dtype.is_integer());
}

#[test]
fn zero_width_integer_is_rejected() {
    assert!(matches!(
        tensor::dtypes::signed_integer(0),
        Err(DTypeError::ZeroBitWidth)
    ));
}

#[test]
fn integer_compatibility_does_not_silently_change_signedness() {
    let signed = tensor::dtypes::signed_integer(32)
        .expect("valid signed type");

    let unsigned = tensor::dtypes::unsigned_integer(32)
        .expect("valid unsigned type");

    assert_eq!(
        tensor::dtypes::compatibility(&signed, &unsigned),
        DTypeCompatibility::Incompatible
    );
}

#[test]
fn representation_width_calculation_is_checked() {
    let representation = Representation::fixed(
        257,
        None,
        ByteOrder::Native,
    )
    .expect("valid representation");

    assert_eq!(representation.bit_width(), 257);
    assert_eq!(representation.byte_width(), None);
    assert_eq!(representation.minimum_storage_bytes(), Some(33));
}

// ============================================================================
// TENSOR OPERATION CONTRACT TESTS
// ============================================================================

#[test]
fn tensor_operation_accepts_valid_input_and_output_contracts() {
    let input = tensor_value(
        "input",
        constant_shape(&[2, 4]),
    );

    let output = tensor_value(
        "output",
        constant_shape(&[2, 4]),
    );

    let operation = TensorOperation::new(
        TensorOperationId::new(
            "zamani.tensor",
            "identity",
            Some("1.0.0".to_owned()),
        )
        .expect("valid operation identity"),
        vec![input],
        vec![output],
    )
    .expect("operation must have an output");

    assert_eq!(operation.inputs().len(), 1);
    assert_eq!(operation.outputs().len(), 1);

    assert!(validate_operation_structure(&operation).is_empty());
}

#[test]
fn tensor_operation_requires_an_output() {
    let result = TensorOperation::new(
        TensorOperationId::new(
            "zamani.tensor",
            "invalid",
            None,
        )
        .expect("valid operation identity"),
        Vec::new(),
        Vec::new(),
    );

    assert!(result.is_err());
}

#[test]
fn tensor_operation_rejects_duplicate_input_identities() {
    let first = tensor_value(
        "shared_input",
        constant_shape(&[2]),
    );

    let second = tensor_value(
        "shared_input",
        constant_shape(&[2]),
    );

    let output = tensor_value(
        "output",
        constant_shape(&[2]),
    );

    let operation = TensorOperation::new(
        TensorOperationId::new(
            "zamani.tensor",
            "duplicate_input_test",
            None,
        )
        .expect("valid operation identity"),
        vec![first, second],
        vec![output],
    )
    .expect("operation construction");

    assert!(validate_operation_structure(&operation)
        .iter()
        .any(|issue| matches!(
            issue,
            TensorOperationIssue::DuplicateInputValue(_)
        )));
}

#[test]
fn tensor_operation_rejects_input_output_identity_collision() {
    let input = tensor_value(
        "same_identity",
        constant_shape(&[2]),
    );

    let output = tensor_value(
        "same_identity",
        constant_shape(&[2]),
    );

    let operation = TensorOperation::new(
        TensorOperationId::new(
            "zamani.tensor",
            "identity_collision",
            None,
        )
        .expect("valid operation identity"),
        vec![input],
        vec![output],
    )
    .expect("operation construction");

    assert!(validate_operation_structure(&operation)
        .iter()
        .any(|issue| matches!(
            issue,
            TensorOperationIssue::InputOutputIdentityCollision(_)
        )));
}

#[test]
fn tensor_operation_preserves_attributes_effects_and_requirements() {
    let operation = TensorOperation::new(
        TensorOperationId::new(
            "zamani.tensor",
            "custom_operation",
            None,
        )
        .expect("valid operation identity"),
        vec![
            tensor_value("input", constant_shape(&[2]))
        ],
        vec![
            tensor_value("output", constant_shape(&[2]))
        ],
    )
    .expect("valid operation")
    .with_attribute(
        TensorAttribute::new(
            "mode",
            TensorAttributeValue::Text("portable".to_owned()),
        )
        .expect("valid attribute"),
    )
    .expect("attribute insertion")
    .with_effect(TensorOperationEffect::Pure)
    .with_requirement(
        TensorCapabilityRequirement::new(
            "tensor",
            "elementwise",
        )
        .expect("valid capability requirement"),
    );

    assert!(operation.attributes().contains_key("mode"));
    assert!(!operation.effects().is_empty());
    assert_eq!(operation.requirements().len(), 1);
}

// ============================================================================
// SYMBOLIC TENSOR GRAPH TESTS
// ============================================================================

#[test]
fn symbolic_tensor_graph_starts_empty() {
    let graph = SymbolicTensorGraph::new();

    assert!(graph.is_empty());
    assert_eq!(graph.len(), 0);
}

#[test]
fn symbolic_tensor_graph_accepts_symbolic_inputs() {
    let mut graph = SymbolicTensorGraph::new();

    let input = graph
        .input(
            AiId::new("model_input")
                .expect("valid input identity"),
            symbolic_shape("batch", &[128]),
        )
        .expect("symbolic input construction");

    assert_eq!(graph.len(), 1);
    assert!(graph.get(input).is_some());

    assert!(graph.validate().is_ok());
}

#[test]
fn symbolic_tensor_graph_rejects_unknown_dependencies() {
    let mut graph = SymbolicTensorGraph::new();

    let result = graph.insert(
        SymbolicTensorOp::Add,
        vec![
            tensor::symbolic::SymbolicExprId::from_test_placeholder(),
        ],
        constant_shape(&[2]),
    );

    assert!(result.is_err());
}

#[test]
fn symbolic_tensor_graph_preserves_deterministic_topological_order() {
    let mut graph = SymbolicTensorGraph::new();

    let first = graph
        .input(
            AiId::new("first").expect("valid identity"),
            constant_shape(&[2]),
        )
        .expect("first input");

    let second = graph
        .insert(
            SymbolicTensorOp::Add,
            vec![first],
            constant_shape(&[2]),
        )
        .expect("second node");

    let third = graph
        .insert(
            SymbolicTensorOp::Multiply,
            vec![second],
            constant_shape(&[2]),
        )
        .expect("third node");

    assert_eq!(
        graph.topological_order(),
        vec![first, second, third]
    );

    assert!(graph.validate().is_ok());
}

// ============================================================================
// CLASSICAL TENSOR INTEROPERABILITY
// ============================================================================

#[test]
fn canonical_semantic_shape_converts_to_classical_shape() {
    let semantic_shape = constant_shape(&[2, 3, 4]);

    let classical_shape = to_classical_shape(
        &semantic_shape,
        &TestDimensionResolver,
    )
    .expect("fully resolved shape must convert");

    assert_eq!(classical_shape.rank(), 3);
    assert_eq!(classical_shape.dimensions(), &[2, 3, 4]);
}

#[test]
fn classical_shape_converts_back_to_canonical_semantic_shape() {
    let classical_shape =
        crate::classical::collections::array::ClassicalArrayShape::new(
            vec![2, 3, 4],
        )
        .expect("valid classical shape");

    let semantic_shape = from_classical_shape(&classical_shape)
        .expect("classical shape must convert");

    assert_eq!(semantic_shape.rank(), 3);

    assert_eq!(
        semantic_shape.dimensions,
        vec![
            DimensionExpr::Constant(2),
            DimensionExpr::Constant(3),
            DimensionExpr::Constant(4),
        ]
    );
}

#[test]
fn unresolved_dimensions_cannot_be_materialized_as_classical_shape() {
    let shape = symbolic_shape("unresolved_batch", &[64]);

    assert!(to_classical_shape(
        &shape,
        &TestDimensionResolver,
    )
    .is_err());
}

#[test]
fn classical_tensor_storage_is_reused_without_copying() {
    let tensor = ClassicalTensor::from_dense_with_dimensions(
        vec![2, 2],
        vec![1_u32, 2, 3, 4],
    )
    .expect("valid dense tensor");

    let descriptor = tensor_descriptor(
        constant_shape(&[2, 2]),
    );

    assert!(validate_classical_tensor(
        &tensor,
        &descriptor,
        &TestDimensionResolver,
    )
    .is_ok());

    assert_eq!(tensor.rank(), 2);
    assert_eq!(tensor.logical_size(), Ok(4));
}

#[test]
fn classical_tensor_view_preserves_canonical_storage() {
    let tensor = ClassicalTensor::from_dense_with_dimensions(
        vec![2, 2],
        vec![10_u32, 20, 30, 40],
    )
    .expect("valid dense tensor");

    let descriptor = tensor_descriptor(
        constant_shape(&[2, 2]),
    );

    let view = tensor::interoperability::ClassicalTensorView::new(
        &tensor,
        &descriptor,
        &TestDimensionResolver,
    )
    .expect("compatible tensor view");

    assert_eq!(view.tensor().rank(), 2);
    assert_eq!(view.descriptor().rank(), 2);

    assert_eq!(
        view.tensor().get_flat(0),
        Some(&10)
    );
}

#[test]
fn classical_tensor_shape_mismatch_is_rejected() {
    let tensor = ClassicalTensor::from_dense_with_dimensions(
        vec![2, 2],
        vec![1_u32, 2, 3, 4],
    )
    .expect("valid dense tensor");

    let descriptor = tensor_descriptor(
        constant_shape(&[2, 3]),
    );

    assert!(validate_classical_tensor(
        &tensor,
        &descriptor,
        &TestDimensionResolver,
    )
    .is_err());
}

#[test]
fn scalar_semantics_are_not_silently_converted_to_classical_array_shape() {
    let scalar = TensorShape::new(Vec::new());

    assert!(to_classical_shape(
        &scalar,
        &TestDimensionResolver,
    )
    .is_err());
}

// ============================================================================
// CONVERSION FIDELITY TESTS
// ============================================================================

#[test]
fn lossless_conversion_accepts_lossless_report() {
    let report = ConversionReport {
        lost_properties: Vec::new(),
        explanations: Vec::new(),
    };

    assert!(validate_conversion_report(
        &report,
        ConversionFidelity::Lossless,
    )
    .is_ok());
}

#[test]
fn conversion_policy_does_not_silently_accept_reported_information_loss() {
    let report = ConversionReport {
        lost_properties: vec![
            tensor::interoperability::LostTensorProperty::Layout,
        ],
        explanations: vec![
            "Destination cannot preserve the source layout".to_owned(),
        ],
    };

    assert!(validate_conversion_report(
        &report,
        ConversionFidelity::Lossless,
    )
    .is_err());
}

// ============================================================================
// POCO-REAF CONTRACT TESTS
// ============================================================================

#[test]
fn logical_tensor_metadata_does_not_require_element_allocation() {
    let descriptor = tensor_descriptor(
        symbolic_shape("dynamic_batch", &[1024, 512]),
    );

    assert_eq!(descriptor.rank(), 3);
    assert!(!descriptor.has_only_constant_dimensions());
}

#[test]
fn unmaterialized_classical_tensor_preserves_large_logical_shape_metadata() {
    let tensor = ClassicalTensor::<u8>::unmaterialized_with_dimensions(
        vec![1024, 1024, 1024],
    )
    .expect("shape metadata should be constructible without dense allocation");

    assert_eq!(tensor.rank(), 3);
    assert_eq!(tensor.stored_len(), 0);
}

#[test]
fn symbolic_graph_does_not_materialize_tensor_elements() {
    let mut graph = SymbolicTensorGraph::new();

    let input = graph
        .input(
            AiId::new("large_input")
                .expect("valid identity"),
            symbolic_shape("batch", &[4096, 4096]),
        )
        .expect("symbolic graph input");

    assert_eq!(graph.len(), 1);
    assert!(graph.get(input).is_some());
    assert!(graph.validate().is_ok());
}
