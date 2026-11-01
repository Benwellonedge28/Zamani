
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Tensor Shapes
//!
//! Path: `src/ai/tensor/shapes.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the AI tensor subsystem's shape-analysis and compatibility
//! boundary while reusing the canonical AI semantic shape representation.
//!
//! This module supports:
//!
//! - Arbitrary-rank tensor shapes.
//! - Known dimensions.
//! - Symbolic dimensions.
//! - Constraint-backed dimensions.
//! - Partially known shapes.
//! - Shape equality analysis.
//! - Broadcasting compatibility.
//! - Symbolic shape constraints.
//! - Explicit unresolved analysis.
//! - Deterministic shape diagnostics.
//! - Conversion-independent shape descriptions.
//!
//! ## AUTHORITATIVE TYPES
//!
//! `frontend::semantic::domains::ai::DimensionExpr` is the authoritative
//! symbolic dimension representation.
//!
//! `frontend::semantic::domains::ai::TensorShape` is the authoritative
//! AI semantic tensor shape representation.
//!
//! This module MUST NOT introduce another dimension expression language
//! or another canonical TensorShape type.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Shape compatibility analysis.
//! - AI shape constraint descriptions.
//! - Broadcasting compatibility analysis.
//! - Shape analysis outcomes.
//! - Shape-specific validation errors.
//! - Shape-analysis interfaces.
//!
//! ## DOES NOT OWN
//!
//! - Canonical semantic types.
//! - Symbol resolution.
//! - General-purpose constraint solving.
//! - Dimension expression evaluation.
//! - Tensor element storage.
//! - Numerical tensor operations.
//! - Tensor operation IR.
//! - Hardware allocation.
//! - Physical tensor layout.
//! - Device selection.
//! - Scheduling.
//! - Distributed placement.
//! - Runtime execution.
//!
//! ## INTEGRATION
//!
//! Semantic authority:
//! `frontend::semantic::domains::ai`
//!
//! Dimension inference:
//! `src/ai/tensor/dimensions.rs`
//!
//! Symbolic transformations:
//! `src/ai/tensor/symbolic.rs`
//!
//! Tensor operation contracts:
//! `src/ai/tensor/operations.rs`
//!
//! Tensor interoperability:
//! `src/ai/tensor/interoperability.rs`
//!
//! Concrete classical storage:
//! `src/classical/collections/tensor.rs`
//!
//! Universal tensor representation:
//! `src/zuir/dialects/tensor/`
//!
//! Source grammar:
//! `grammar/ai/tensors.g4`
//!
//! AI grammar composition:
//! `grammar/ai/ai.g4`
//!
//! ## SCALABILITY
//!
//! No universal maximum tensor rank, dimension count, shape size,
//! parameter count, or hardware capacity is imposed.
//!
//! Vec length represents rank. It is not a language-level rank limit.
//!
//! Concrete execution remains subject to:
//!
//! - Available memory.
//! - Host integer representation.
//! - Compiler operational budgets.
//! - Target capabilities.
//! - Runtime resource availability.
//!
//! Unresolved symbolic shapes must never be silently treated as
//! validated concrete shapes.
//!
//! ## DETERMINISM
//!
//! Shape comparisons preserve dimension order.
//!
//! Constraint collections preserve insertion order.
//!
//! No randomized collection iteration is exposed.
//!
//! ## FREEZE CRITERIA
//!
//! - Canonical semantic types are reused.
//! - Symbolic shapes are preserved.
//! - Known incompatible shapes are rejected.
//! - Unresolved compatibility is explicitly represented.
//! - Broadcasting does not invent symbolic values.
//! - Rank is not artificially restricted.
//! - Errors preserve their meaning.
//! - All tests pass.
//!
//! No hardware-specific assumptions are permitted.

use std::fmt;

pub use crate::frontend::semantic::domains::ai::{
    DimensionExpr,
    TensorShape,
};

// ============================================================================
// SHAPE CONSTRAINTS
// ============================================================================

/// A relationship between two symbolic tensor dimensions.
///
/// Constraints describe obligations that must be established by the
/// authoritative semantic constraint solver.
///
/// This type does not evaluate arbitrary expressions.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ShapeConstraint {
    /// Both dimensions must have the same mathematical value.
    Equal {
        left: DimensionExpr,
        right: DimensionExpr,
    },

    /// The dimensions must be compatible under broadcasting.
///
/// Broadcasting is valid when the dimensions are equal or when at
/// least one dimension is mathematically equal to one.
    Broadcastable {
        left: DimensionExpr,
        right: DimensionExpr,
    },
}

impl ShapeConstraint {
    /// Returns the first dimension involved in the constraint.
    #[must_use]
    pub fn left(&self) -> &DimensionExpr {
        match self {
            Self::Equal { left, .. } => left,
            Self::Broadcastable { left, .. } => left,
        }
    }

    /// Returns the second dimension involved in the constraint.
    #[must_use]
    pub fn right(&self) -> &DimensionExpr {
        match self {
            Self::Equal { right, .. } => right,
            Self::Broadcastable { right, .. } => right,
        }
    }
}

// ============================================================================
// SHAPE ANALYSIS OUTCOMES
// ============================================================================

/// Describes the result of a shape compatibility analysis.
///
/// An unresolved result is deliberately different from a successful
/// compatibility result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeCompatibility {
    /// Compatibility has been established from available information.
    Compatible,

    /// A definite incompatibility has been established.
    Incompatible {
        /// The dimension position responsible for incompatibility.
        axis: Option<usize>,

        /// Explanation of the incompatibility.
        reason: ShapeIncompatibility,
    },

    /// Compatibility depends on symbolic facts that have not yet
    /// been established.
    RequiresResolution {
        /// Obligations that the canonical constraint solver must resolve.
        constraints: Vec<ShapeConstraint>,
    },
}

impl ShapeCompatibility {
    /// Returns true only when compatibility has been established.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns true when compatibility is definitively impossible.
    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible { .. })
    }

    /// Returns true when further semantic analysis is required.
    #[must_use]
    pub fn requires_resolution(&self) -> bool {
        matches!(self, Self::RequiresResolution { .. })
    }

    /// Returns the unresolved obligations, if any.
    #[must_use]
    pub fn constraints(&self) -> &[ShapeConstraint] {
        match self {
            Self::RequiresResolution { constraints } => constraints,
            _ => &[],
        }
    }
}

/// Reasons why two shapes cannot be compatible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeIncompatibility {
    /// Their ranks differ.
    RankMismatch {
        left_rank: usize,
        right_rank: usize,
    },

    /// Two known dimensions have different values.
    DimensionMismatch {
        left: u128,
        right: u128,
    },

    /// A broadcasting relationship cannot be satisfied.
    BroadcastingMismatch {
        left: u128,
        right: u128,
    },
}

// ============================================================================
// SHAPE ANALYSIS
// ============================================================================

/// Result of a tensor shape operation.
///
/// The output shape may remain unresolved when symbolic relationships
/// cannot yet be established.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShapeAnalysis {
    /// Resulting shape, when it can be established.
    pub shape: Option<TensorShape>,

    /// Compatibility outcome.
    pub compatibility: ShapeCompatibility,
}

impl ShapeAnalysis {
    /// Creates a successfully established shape analysis.
    #[must_use]
    pub fn compatible(shape: TensorShape) -> Self {
        Self {
            shape: Some(shape),
            compatibility: ShapeCompatibility::Compatible,
        }
    }

    /// Creates an analysis requiring further constraint resolution.
    #[must_use]
    pub fn unresolved(
        constraints: Vec<ShapeConstraint>,
    ) -> Self {
        Self {
            shape: None,
            compatibility: ShapeCompatibility::RequiresResolution {
                constraints,
            },
        }
    }

    /// Creates a definitively incompatible analysis.
    #[must_use]
    pub fn incompatible(
        axis: Option<usize>,
        reason: ShapeIncompatibility,
    ) -> Self {
        Self {
            shape: None,
            compatibility: ShapeCompatibility::Incompatible {
                axis,
                reason,
            },
        }
    }

    /// Returns the shape only when analysis established a result.
    #[must_use]
    pub fn resolved_shape(&self) -> Option<&TensorShape> {
        self.shape.as_ref()
    }

    /// Returns whether the analysis established compatibility.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.compatibility.is_compatible()
    }
}

// ============================================================================
// SHAPE ERRORS
// ============================================================================

/// Errors produced by shape-specific validation and analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeError {
    /// A required dimension is missing.
    MissingDimension {
        axis: usize,
    },

    /// A shape contains an invalid constant dimension.
    InvalidDimension {
        axis: usize,
    },

    /// A concrete dimension cannot be represented by the requested
    /// host indexing representation.
    DimensionNotRepresentable {
        axis: usize,
        value: u128,
    },

    /// Shape compatibility has not been established.
    UnresolvedCompatibility,

    /// Shapes are incompatible.
    Incompatible {
        axis: Option<usize>,
        reason: ShapeIncompatibility,
    },
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDimension { axis } => {
                write!(f, "missing tensor dimension at axis {axis}")
            }

            Self::InvalidDimension { axis } => {
                write!(f, "invalid tensor dimension at axis {axis}")
            }

            Self::DimensionNotRepresentable { axis, value } => {
                write!(
                    f,
                    "dimension {value} at axis {axis} cannot be represented"
                )
            }

            Self::UnresolvedCompatibility => {
                f.write_str(
                    "tensor shape compatibility requires further resolution"
                )
            }

            Self::Incompatible { axis, reason } => {
                write!(
                    f,
                    "incompatible tensor shapes at axis {axis:?}: {reason:?}"
                )
            }
        }
    }
}

impl std::error::Error for ShapeError {}

// ============================================================================
// DIMENSION KNOWLEDGE
// ============================================================================

/// Information about a symbolic dimension.
///
/// The authoritative semantic system supplies this information.
/// This module does not resolve symbols itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DimensionKnowledge {
    /// The mathematical dimension is known.
    Known(u128),

    /// The dimension is valid but its value remains unknown.
    Unknown,

    /// The dimension has been established as invalid.
    Invalid,
}

/// Interface to the canonical dimension and constraint infrastructure.
///
/// The frontend semantic adapter and future dimensions subsystem should
/// implement this interface by delegating to the existing semantic
/// constraint and constant-evaluation services.
pub trait DimensionResolver {
    /// Resolves the available knowledge about a dimension.
    fn resolve_dimension(
        &self,
        dimension: &DimensionExpr,
    ) -> DimensionKnowledge;

    /// Establishes whether two dimensions are mathematically equal.
    ///
    /// `None` means that equality cannot currently be established.
    fn prove_equal(
        &self,
        left: &DimensionExpr,
        right: &DimensionExpr,
    ) -> Option<bool>;
}

// ============================================================================
// SHAPE CONSTRUCTION
// ============================================================================

/// Constructs AI semantic shapes using the canonical shape representation.
pub struct ShapeBuilder;

impl ShapeBuilder {
    /// Constructs a shape from canonical dimension expressions.
    ///
    /// An empty dimension list represents a scalar, consistent with
    /// the existing semantic TensorShape contract.
    ///
    /// This operation does not allocate tensor elements.
    #[must_use]
    pub fn from_dimensions(
        dimensions: Vec<DimensionExpr>,
    ) -> TensorShape {
        TensorShape::new(dimensions)
    }

    /// Constructs a shape containing known constant dimensions.
    #[must_use]
    pub fn from_constants(
        dimensions: Vec<u128>,
    ) -> TensorShape {
        TensorShape::new(
            dimensions
                .into_iter()
                .map(DimensionExpr::Constant)
                .collect(),
        )
    }

    /// Returns the rank of a shape.
    ///
    /// Rank is represented by the number of dimensions. There is
    /// no language-level maximum rank.
    #[must_use]
    pub fn rank(shape: &TensorShape) -> usize {
        shape.rank()
    }

    /// Returns true when the shape represents a scalar.
    #[must_use]
    pub fn is_scalar(shape: &TensorShape) -> bool {
        shape.is_scalar()
    }

    /// Returns whether every dimension is a known constant.
    #[must_use]
    pub fn is_fully_static(shape: &TensorShape) -> bool {
        shape
            .dimensions
            .iter()
            .all(|dimension| matches!(dimension, DimensionExpr::Constant(_)))
    }

    /// Returns whether at least one dimension is symbolic or unresolved.
    #[must_use]
    pub fn is_symbolic(shape: &TensorShape) -> bool {
        !Self::is_fully_static(shape)
    }
}

// ============================================================================
// SHAPE COMPATIBILITY
// ============================================================================

/// Shape compatibility operations.
pub struct ShapeCompatibilityAnalyzer;

impl ShapeCompatibilityAnalyzer {
    /// Determines whether two shapes are compatible for elementwise
    /// operations requiring equal dimensions.
    ///
    /// Different ranks are incompatible.
    ///
    /// Different known dimensions are incompatible.
    ///
    /// Symbolic relationships become explicit constraints.
    ///
    /// Identical symbolic expressions are compatible without requiring
    /// another solver invocation.
    #[must_use]
    pub fn equal<R: DimensionResolver>(
        left: &TensorShape,
        right: &TensorShape,
        resolver: &R,
    ) -> ShapeCompatibility {
        if left.rank() != right.rank() {
            return ShapeCompatibility::Incompatible {
                axis: None,
                reason: ShapeIncompatibility::RankMismatch {
                    left_rank: left.rank(),
                    right_rank: right.rank(),
                },
            };
        }

        let mut constraints = Vec::new();

        for (axis, (left_dim, right_dim)) in left
            .dimensions
            .iter()
            .zip(right.dimensions.iter())
            .enumerate()
        {
            if left_dim == right_dim {
                continue;
            }

            let left_value = resolver.resolve_dimension(left_dim);
            let right_value = resolver.resolve_dimension(right_dim);

            match (left_value, right_value) {
                (
                    DimensionKnowledge::Invalid,
                    _,
                )
                | (
                    _,
                    DimensionKnowledge::Invalid,
                ) => {
                    return ShapeCompatibility::Incompatible {
                        axis: Some(axis),
                        reason: ShapeIncompatibility::DimensionMismatch {
                            left: 0,
                            right: 0,
                        },
                    };
                }

                (
                    DimensionKnowledge::Known(a),
                    DimensionKnowledge::Known(b),
                ) if a != b => {
                    return ShapeCompatibility::Incompatible {
                        axis: Some(axis),
                        reason: ShapeIncompatibility::DimensionMismatch {
                            left: a,
                            right: b,
                        },
                    };
                }

                (
                    DimensionKnowledge::Known(a),
                    DimensionKnowledge::Known(b),
                ) if a == b => {}

                _ => {
                    match resolver.prove_equal(left_dim, right_dim) {
                        Some(true) => {}

                        Some(false) => {
                            return ShapeCompatibility::Incompatible {
                                axis: Some(axis),
                                reason: ShapeIncompatibility::DimensionMismatch {
                                    left: 0,
                                    right: 0,
                                },
                            };
                        }

                        None => {
                            constraints.push(ShapeConstraint::Equal {
                                left: left_dim.clone(),
                                right: right_dim.clone(),
                            });
                        }
                    }
                }
            }
        }

        if constraints.is_empty() {
            ShapeCompatibility::Compatible
        } else {
            ShapeCompatibility::RequiresResolution {
                constraints,
            }
        }
    }

    /// Determines whether two shapes can participate in broadcasting.
    ///
    /// Dimensions are compared from the trailing axis.
    ///
    /// A missing leading dimension is treated as an implicit dimension
    /// of one according to standard broadcasting semantics.
    ///
    /// Symbolic compatibility is not guessed. Unresolved relationships
    /// are returned as explicit constraints.
    #[must_use]
    pub fn broadcast<R: DimensionResolver>(
        left: &TensorShape,
        right: &TensorShape,
        resolver: &R,
    ) -> ShapeAnalysis {
        let result_rank = left.rank().max(right.rank());

        let mut result_dimensions = Vec::new();

        if result_dimensions
            .try_reserve_exact(result_rank)
            .is_err()
        {
            return ShapeAnalysis::unresolved(Vec::new());
        }

        let mut constraints = Vec::new();

        for axis in 0..result_rank {
            let left_index = left.rank().checked_sub(axis + 1);
            let right_index = right.rank().checked_sub(axis + 1);

            let left_dim = left_index.map(|index| &left.dimensions[index]);
            let right_dim = right_index.map(|index| &right.dimensions[index]);

            let output = match (left_dim, right_dim) {
                (Some(a), Some(b)) => {
                    Self::broadcast_dimension(
                        a,
                        b,
                        axis,
                        resolver,
                        &mut constraints,
                    )
                }

                (Some(a), None) | (None, Some(a)) => {
                    Some(a.clone())
                }

                (None, None) => None,
            };

            match output {
                Some(dimension) => result_dimensions.push(dimension),

                None => {
                    if constraints.is_empty() {
                        return ShapeAnalysis::incompatible(
                            Some(axis),
                            ShapeIncompatibility::BroadcastingMismatch {
                                left: 0,
                                right: 0,
                            },
                        );
                    }
                }
            }
        }

        if !constraints.is_empty() {
            return ShapeAnalysis::unresolved(constraints);
        }

        result_dimensions.reverse();

        ShapeAnalysis::compatible(
            TensorShape::new(result_dimensions)
        )
    }

    fn broadcast_dimension<R: DimensionResolver>(
        left: &DimensionExpr,
        right: &DimensionExpr,
        axis: usize,
        resolver: &R,
        constraints: &mut Vec<ShapeConstraint>,
    ) -> Option<DimensionExpr> {
        if left == right {
            return Some(left.clone());
        }

        let left_value = resolver.resolve_dimension(left);
        let right_value = resolver.resolve_dimension(right);

        match (left_value, right_value) {
            (
                DimensionKnowledge::Invalid,
                _,
            )
            | (
                _,
                DimensionKnowledge::Invalid,
            ) => None,

            (
                DimensionKnowledge::Known(1),
                _,
            ) => Some(right.clone()),

            (
                _,
                DimensionKnowledge::Known(1),
            ) => Some(left.clone()),

            (
                DimensionKnowledge::Known(a),
                DimensionKnowledge::Known(b),
            ) if a == b => Some(left.clone()),

            (
                DimensionKnowledge::Known(a),
                DimensionKnowledge::Known(b),
            ) => {
                let _ = axis;
                let _ = a;
                let _ = b;
                None
            }

            _ => {
                match resolver.prove_equal(left, right) {
                    Some(true) => Some(left.clone()),

                    Some(false) => {
                        let left_is_one = matches!(
                            left_value,
                            DimensionKnowledge::Known(1)
                        );

                        let right_is_one = matches!(
                            right_value,
                            DimensionKnowledge::Known(1)
                        );

                        if left_is_one {
                            Some(right.clone())
                        } else if right_is_one {
                            Some(left.clone())
                        } else {
                            constraints.push(
                                ShapeConstraint::Broadcastable {
                                    left: left.clone(),
                                    right: right.clone(),
                                },
                            );

                            None
                        }
                    }

                    None => {
                        constraints.push(
                            ShapeConstraint::Broadcastable {
                                left: left.clone(),
                                right: right.clone(),
                            },
                        );

                        None
                    }
                }
            }
        }
    }
}

// ============================================================================
// SHAPE VALIDATION
// ============================================================================

/// Validates a shape using the canonical dimension resolver.
pub struct ShapeValidator;

impl ShapeValidator {
    /// Validates every dimension without imposing a fixed rank limit.
    ///
    /// Unknown symbolic dimensions are not considered invalid.
    /// Their eventual validity must be established by the semantic
    /// constraint infrastructure.
    pub fn validate<R: DimensionResolver>(
        shape: &TensorShape,
        resolver: &R,
    ) -> Result<(), ShapeError> {
        for (axis, dimension) in shape.dimensions.iter().enumerate() {
            if matches!(
                resolver.resolve_dimension(dimension),
                DimensionKnowledge::Invalid
            ) {
                return Err(ShapeError::InvalidDimension { axis });
            }
        }

        Ok(())
    }

    /// Converts a fully concrete semantic shape into host-sized
    /// dimension lengths.
    ///
    /// This is a metadata conversion only. It does not allocate storage.
    ///
    /// It deliberately rejects unresolved symbolic dimensions.
    ///
    /// The classical collection layer remains responsible for its
    /// own concrete shape invariants.
    pub fn concrete_dimensions<R: DimensionResolver>(
        shape: &TensorShape,
        resolver: &R,
    ) -> Result<Vec<usize>, ShapeError> {
        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(shape.rank())
            .map_err(|_| ShapeError::UnresolvedCompatibility)?;

        for (axis, dimension) in shape.dimensions.iter().enumerate() {
            match resolver.resolve_dimension(dimension) {
                DimensionKnowledge::Known(value) => {
                    let concrete = usize::try_from(value)
                        .map_err(|_| {
                            ShapeError::DimensionNotRepresentable {
                                axis,
                                value,
                            }
                        })?;

                    dimensions.push(concrete);
                }

                DimensionKnowledge::Unknown => {
                    return Err(ShapeError::UnresolvedCompatibility);
                }

                DimensionKnowledge::Invalid => {
                    return Err(ShapeError::InvalidDimension {
                        axis,
                    });
                }
            }
        }

        Ok(dimensions)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    struct TestResolver {
        values: BTreeMap<DimensionExpr, DimensionKnowledge>,
        equalities: BTreeMap<
            (DimensionExpr, DimensionExpr),
            bool,
        >,
    }

    impl TestResolver {
        fn new() -> Self {
            Self {
                values: BTreeMap::new(),
                equalities: BTreeMap::new(),
            }
        }

        fn with_value(
            mut self,
            expression: DimensionExpr,
            value: DimensionKnowledge,
        ) -> Self {
            self.values.insert(expression, value);
            self
        }

        fn with_equality(
            mut self,
            left: DimensionExpr,
            right: DimensionExpr,
            equal: bool,
        ) -> Self {
            self.equalities.insert(
                (left.clone(), right.clone()),
                equal,
            );

            self.equalities.insert(
                (right, left),
                equal,
            );

            self
        }
    }

    impl DimensionResolver for TestResolver {
        fn resolve_dimension(
            &self,
            dimension: &DimensionExpr,
        ) -> DimensionKnowledge {
            match dimension {
                DimensionExpr::Constant(value) => {
                    DimensionKnowledge::Known(*value)
                }

                _ => self.values
                    .get(dimension)
                    .copied()
                    .unwrap_or(DimensionKnowledge::Unknown),
            }
        }

        fn prove_equal(
            &self,
            left: &DimensionExpr,
            right: &DimensionExpr,
        ) -> Option<bool> {
            self.equalities
                .get(&(left.clone(), right.clone()))
                .copied()
        }
    }

    fn shape(values: &[u128]) -> TensorShape {
        ShapeBuilder::from_constants(values.to_vec())
    }

    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    #[test]
    fn constructs_arbitrary_rank_shape() {
        let dimensions = (0..256)
            .map(|value| DimensionExpr::Constant(value))
            .collect();

        let shape = ShapeBuilder::from_dimensions(dimensions);

        assert_eq!(shape.rank(), 256);
    }

    #[test]
    fn empty_shape_represents_scalar() {
        let shape = TensorShape::new(Vec::new());

        assert!(ShapeBuilder::is_scalar(&shape));
        assert_eq!(shape.rank(), 0);
    }

    #[test]
    fn identifies_static_shape() {
        assert!(ShapeBuilder::is_fully_static(
            &shape(&[2, 3, 4])
        ));
    }

    #[test]
    fn identifies_symbolic_shape() {
        let expression = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "batch"
            ).unwrap()
        );

        let shape = ShapeBuilder::from_dimensions(vec![
            DimensionExpr::Constant(16),
            expression,
        ]);

        assert!(ShapeBuilder::is_symbolic(&shape));
    }

    // ---------------------------------------------------------------------
    // Equality
    // ---------------------------------------------------------------------

    #[test]
    fn equal_static_shapes_are_compatible() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::equal(
                &shape(&[2, 3]),
                &shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_compatible());
    }

    #[test]
    fn different_ranks_are_incompatible() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::equal(
                &shape(&[2, 3]),
                &shape(&[2, 3, 4]),
                &resolver,
            );

        assert!(result.is_incompatible());
    }

    #[test]
    fn different_known_dimensions_are_incompatible() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::equal(
                &shape(&[2, 3]),
                &shape(&[2, 4]),
                &resolver,
            );

        assert!(result.is_incompatible());
    }

    #[test]
    fn symbolic_equality_remains_unresolved() {
        let a = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "a"
            ).unwrap()
        );

        let b = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "b"
            ).unwrap()
        );

        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::equal(
                &TensorShape::new(vec![a]),
                &TensorShape::new(vec![b]),
                &resolver,
            );

        assert!(result.requires_resolution());
    }

    #[test]
    fn resolved_symbolic_equality_is_compatible() {
        let a = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "a"
            ).unwrap()
        );

        let b = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "b"
            ).unwrap()
        );

        let resolver = TestResolver::new()
            .with_equality(a.clone(), b.clone(), true);

        let result =
            ShapeCompatibilityAnalyzer::equal(
                &TensorShape::new(vec![a]),
                &TensorShape::new(vec![b]),
                &resolver,
            );

        assert!(result.is_compatible());
    }

    // ---------------------------------------------------------------------
    // Broadcasting
    // ---------------------------------------------------------------------

    #[test]
    fn broadcasts_equal_shapes() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &shape(&[2, 3]),
                &shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(result.shape, Some(shape(&[2, 3])));
    }

    #[test]
    fn broadcasts_singleton_dimension() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &shape(&[1, 3]),
                &shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(result.shape, Some(shape(&[2, 3])));
    }

    #[test]
    fn broadcasts_different_ranks() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &shape(&[3]),
                &shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(result.shape, Some(shape(&[2, 3])));
    }

    #[test]
    fn rejects_incompatible_broadcast_dimensions() {
        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &shape(&[2, 3]),
                &shape(&[4, 5]),
                &resolver,
            );

        assert!(result.compatibility.is_incompatible());
    }

    #[test]
    fn symbolic_broadcast_requires_resolution() {
        let a = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "a"
            ).unwrap()
        );

        let b = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "b"
            ).unwrap()
        );

        let resolver = TestResolver::new();

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &TensorShape::new(vec![a]),
                &TensorShape::new(vec![b]),
                &resolver,
            );

        assert!(result.compatibility.requires_resolution());
    }

    // ---------------------------------------------------------------------
    // Concrete dimensions
    // ---------------------------------------------------------------------

    #[test]
    fn converts_concrete_dimensions() {
        let resolver = TestResolver::new();

        let result =
            ShapeValidator::concrete_dimensions(
                &shape(&[2, 3, 4]),
                &resolver,
            ).unwrap();

        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn rejects_unresolved_concrete_dimensions() {
        let symbol = DimensionExpr::Symbol(
            crate::frontend::semantic::domains::ai::AiId::new(
                "batch"
            ).unwrap()
        );

        let resolver = TestResolver::new();

        let result =
            ShapeValidator::concrete_dimensions(
                &TensorShape::new(vec![symbol]),
                &resolver,
            );

        assert_eq!(
            result,
            Err(ShapeError::UnresolvedCompatibility)
        );
    }

    #[test]
    fn validates_zero_length_dimensions() {
        let resolver = TestResolver::new();

        let tensor_shape = shape(&[0, 3]);

        assert!(
            ShapeValidator::validate(
                &tensor_shape,
                &resolver,
            ).is_ok()
        );
    }
}
