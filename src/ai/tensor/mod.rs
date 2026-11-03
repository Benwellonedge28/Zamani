
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Subsystem
//!
//! Path: `src/ai/tensor/mod.rs`
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for target-independent AI tensor
//! descriptions, shape analysis, symbolic tensor computation, logical
//! layouts, numerical type descriptions, operation contracts, and tensor
//! interoperability.
//!
//! It provides a coherent public module boundary over the existing
//! implementations in this directory.
//!
//! It does not implement a second tensor system.
//!
//! ## Architectural ownership
//!
//! The following repository modules remain authoritative:
//!
//! - `frontend::semantic::domains::ai`:
//!   canonical semantic `DimensionExpr` and `TensorShape`.
//!
//! - `classical::collections::tensor`:
//!   concrete classical tensor storage.
//!
//! - `zuir::dialects::tensor`:
//!   universal tensor operation representation.
//!
//! - `frontend::semantic`:
//!   canonical source-language types, effects, constraints, and validation.
//!
//! - `distributed`:
//!   distributed execution and physical placement.
//!
//! - `compiler` and `runtime`:
//!   lowering, optimization, scheduling, allocation, and execution.
//!
//! This module does not supersede any of those authorities.
//!
//! ## Module responsibilities
//!
//! - `shapes`: shape compatibility and shape validation interfaces.
//! - `dimensions`: symbolic dimension evaluation, substitution, and
//!   dimension transformations.
//! - `layouts`: target-independent logical tensor layout descriptions.
//! - `dtypes`: target-independent numerical type descriptions.
//! - `operations`: tensor operation contracts and structural validation.
//! - `symbolic`: symbolic tensor computation graphs.
//! - `interoperability`: checked conversions and adapters to canonical
//!   tensor representations.
//!
//! ## Dependency direction
//!
//! ```text
//! frontend semantic authority
//!          |
//!          v
//! canonical DimensionExpr / TensorShape
//!          |
//!          +---------------------------+
//!          |                           |
//!          v                           v
//!       shapes                    dimensions
//!          |                           |
//!          +-------------+-------------+
//!                        |
//!            +-----------+-----------+
//!            |           |           |
//!            v           v           v
//!         layouts      dtypes     symbolic
//!            |           |           |
//!            +-----------+-----------+
//!                        |
//!                        v
//!                    operations
//!                        |
//!                        v
//!                 interoperability
//!                        |
//!             +----------+----------+
//!             |                     |
//!             v                     v
//!       classical tensor       ZUIR tensor dialect
//!             |                     |
//!             +----------+----------+
//!                        |
//!                        v
//!              compiler / runtime
//! ```
//!
//! This diagram describes conceptual dependencies, not permission for
//! lower-level modules to depend on higher-level modules.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no universal maximum for:
//!
//! - tensor rank;
//! - tensor dimensions;
//! - tensor operation count;
//! - symbolic graph nodes;
//! - tensor layout metadata;
//! - model parameter count;
//! - number of tensors;
//! - accelerator count;
//! - memory capacity;
//! - distributed workers;
//! - machine count.
//!
//! Dynamic collections grow subject to host representation and available
//! resources. Concrete execution may fail when a particular target cannot
//! satisfy the requested computation.
//!
//! Such failures must be explicit. They must not silently change tensor
//! semantics or impose a language-level capacity ceiling.
//!
//! "Infinity" is therefore not a claim that finite hardware can materialize
//! an infinite tensor. It means that the language and this module do not
//! impose an arbitrary universal capacity limit.
//!
//! ## Important distinctions
//!
//! A symbolic tensor is not necessarily materialized.
//!
//! A logical layout is not a physical memory placement.
//!
//! A dtype descriptor is not proof that a target supports that dtype.
//!
//! An operation contract is not a numerical kernel.
//!
//! A symbolic graph is not a second compiler IR.
//!
//! A classical tensor is not the universal tensor representation.
//!
//! ## Grammar integration
//!
//! Tensor source syntax is owned by:
//!
//! - `grammar/ai/tensors.g4`
//! - `grammar/ai/ai.g4`
//!
//! The grammar feeds the canonical frontend AST and semantic pipeline.
//! This module must not parse source text or construct a competing AST.
//!
//! ## Safety
//!
//! All implementations exposed by this module must use safe Rust.
//!
//! ## Compatibility
//!
//! Existing module paths remain public. New functionality should normally
//! be added to its owning submodule rather than to this composition root.

/// AI tensor shape descriptions, compatibility analysis, and validation.
///
/// Canonical `DimensionExpr` and `TensorShape` are re-exported from the
/// existing semantic authority by this module.
pub mod shapes;

/// Symbolic dimension evaluation, substitution, compatibility analysis,
/// and shape transformations.
///
/// This module intentionally remains separate from `shapes` because the
/// repository currently contains distinct implementations and interfaces
/// in both files. Their names must not be flattened into one namespace.
pub mod dimensions;

/// Target-independent logical tensor layout descriptions.
///
/// Physical placement and device-specific memory layouts remain downstream.
pub mod layouts;

/// Target-independent numerical tensor element type descriptions.
///
/// This module does not select a target representation or implement
/// numerical kernels.
pub mod dtypes;

/// Target-independent tensor operation contracts and structural validation.
///
/// Numerical execution and universal IR lowering remain outside this module.
pub mod operations;

/// Symbolic tensor expression graphs and symbolic shape transformations.
///
/// Symbolic graph construction does not allocate tensor element storage.
pub mod symbolic;

/// Checked interoperability between AI tensor descriptions, classical
/// tensor storage, and canonical tensor representations.
pub mod interoperability;

// ============================================================================
// CANONICAL SEMANTIC SHAPE TYPES
// ============================================================================

/// Canonical semantic dimension expression.
///
/// This is the existing frontend semantic type, not a new AI tensor type.
pub use shapes::DimensionExpr;

/// Canonical semantic tensor shape.
///
/// This is the existing frontend semantic type, not a new AI tensor type.
pub use shapes::TensorShape;

// ============================================================================
// SHAPE ANALYSIS PUBLIC API
// ============================================================================

pub use shapes::{
    ShapeAnalysis,
    ShapeBuilder,
    ShapeCompatibility,
    ShapeCompatibilityAnalyzer,
    ShapeConstraint,
    ShapeError,
    ShapeIncompatibility,
    ShapeValidator,
};

// ============================================================================
// DIMENSION ANALYSIS PUBLIC API
// ============================================================================

/// Dimension-analysis APIs are available under `dimensions`.
///
/// In particular, `dimensions::DimensionResolver` and
/// `shapes::DimensionResolver` are intentionally not merged. They belong
/// to different existing interfaces and must not be confused.
///
/// The module remains publicly accessible as `tensor::dimensions`.

// ============================================================================
// LOGICAL LAYOUT PUBLIC API
// ============================================================================

pub use layouts::{
    DenseLayout,
    DenseOrder,
    DistributedLayout,
    ExtensionLayout,
    LayoutCompatibility,
    LayoutError,
    LayoutId,
    LayoutValidation,
    LayoutValidationContext,
    LazyLayout,
    SparseLayout,
    StrideValue,
    StridedLayout,
    TensorLayout,
    TensorLayoutKind,
    TiledLayout,
};

// ============================================================================
// NUMERICAL TYPE PUBLIC API
// ============================================================================

pub use dtypes::{
    ByteOrder,
    ComplexSemantics,
    CustomDType,
    DType,
    DTypeCompatibility,
    DTypeError,
    FloatEncoding,
    FloatSemantics,
    IntegerSemantics,
    IntegerSignedness,
    NumericCategory,
    QuantizationScale,
    QuantizationZeroPoint,
    QuantizedSemantics,
    Representation,
};

// ============================================================================
// TENSOR OPERATION PUBLIC API
// ============================================================================

pub use operations::{
    TensorAttribute,
    TensorAttributeValue,
    TensorCapabilityRequirement,
    TensorOperation,
    TensorOperationEffect,
    TensorOperationError,
    TensorOperationId,
    TensorOperationIssue,
    TensorOperationKind,
    TensorOperationSchema,
    TensorShapeValidation,
    TensorValueContract,
    TensorValueId,
};

// ============================================================================
// SYMBOLIC TENSOR PUBLIC API
// ============================================================================

pub use symbolic::{
    DimensionSubstitution,
    SymbolicExprId,
    SymbolicTensorError,
    SymbolicTensorGraph,
    SymbolicTensorNode,
    SymbolicTensorOp,
};

// ============================================================================
// INTEROPERABILITY PUBLIC API
// ============================================================================

pub use interoperability::{
    ClassicalTensorView,
    ConversionFidelity,
    ConversionReport,
    InteroperabilityError,
    InteropDimensionKnowledge,
    InteropDimensionResolver,
    TensorConversionRequest,
    TensorDescriptor,
    TensorInteropAdapter,
    TensorRepresentationId,
};

// ============================================================================
// PUBLIC API INVARIANTS
// ============================================================================

/// Returns the canonical semantic tensor shape rank.
///
/// This is a convenience function. It does not impose a maximum rank.
#[must_use]
pub fn rank(shape: &TensorShape) -> usize {
    shape.rank()
}

/// Returns whether a canonical semantic tensor shape represents a scalar.
#[must_use]
pub fn is_scalar(shape: &TensorShape) -> bool {
    shape.is_scalar()
}

/// Returns whether all dimensions in a shape are statically known constants.
///
/// This is a structural query. It does not claim that the shape has been
/// validated by the canonical semantic constraint system.
#[must_use]
pub fn is_fully_static(shape: &TensorShape) -> bool {
    shapes::ShapeBuilder::is_fully_static(shape)
}

/// Returns whether a shape contains at least one symbolic or unresolved
/// dimension.
#[must_use]
pub fn is_symbolic(shape: &TensorShape) -> bool {
    shapes::ShapeBuilder::is_symbolic(shape)
}

// ============================================================================
// MODULE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::domains::ai::{
        AiId,
        DimensionExpr,
        TensorShape,
    };

    #[test]
    fn canonical_shape_types_are_reused() {
        let shape = TensorShape::new(vec![
            DimensionExpr::Constant(8),
            DimensionExpr::Constant(16),
        ]);

        assert_eq!(rank(&shape), 2);
        assert!(!is_scalar(&shape));
        assert!(is_fully_static(&shape));
        assert!(!is_symbolic(&shape));
    }

    #[test]
    fn scalar_shape_is_preserved() {
        let shape = TensorShape::new(Vec::new());

        assert!(is_scalar(&shape));
        assert_eq!(rank(&shape), 0);
    }

    #[test]
    fn symbolic_dimensions_are_not_materialized() {
        let symbol = AiId::new("batch")
            .expect("test symbol must be valid");

        let shape = TensorShape::new(vec![
            DimensionExpr::Symbol(symbol),
            DimensionExpr::Constant(128),
        ]);

        assert_eq!(rank(&shape), 2);
        assert!(!is_fully_static(&shape));
        assert!(is_symbolic(&shape));
    }

    #[test]
    fn module_namespaces_remain_distinct() {
        // The two modules intentionally expose separate analysis APIs.
        // Keeping these paths distinct prevents accidental name collisions.
        let _shape_builder = shapes::ShapeBuilder;
        let _dimension_resolver = dimensions::ConstantResolver;
    }

    #[test]
    fn dtype_descriptors_are_target_independent() {
        let dtype = DType::Bool;

        assert_eq!(dtype.category(), NumericCategory::Boolean);
        assert_eq!(dtype.bit_width(), Some(1));
    }

    #[test]
    fn logical_layout_does_not_require_a_device() {
        let layout = TensorLayout::row_major();

        assert_eq!(layout.kind_name(), "dense");
    }

    #[test]
    fn symbolic_graph_is_available_without_materialization() {
        let graph = SymbolicTensorGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.len(), 0);
    }
}
