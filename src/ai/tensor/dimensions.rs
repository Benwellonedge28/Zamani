
#![forbid(unsafe_code)]

//! AI tensor dimension analysis for Zamani.
//!
//! This module operates on the canonical semantic dimension and shape types:
//!
//! - `frontend::semantic::domains::ai::DimensionExpr`
//! - `frontend::semantic::domains::ai::TensorShape`
//!
//! It does not introduce another dimension or shape representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - dimension resolution interfaces;
//! - symbolic dimension substitution;
//! - dimension equality analysis;
//! - shape compatibility analysis;
//! - broadcasting analysis;
//! - shape transformations;
//! - checked conversion of concrete dimensions.
//!
//! It does not own:
//!
//! - parsing or AST construction;
//! - the canonical semantic constraint solver;
//! - tensor element types;
//! - tensor storage or allocation;
//! - physical layouts or placement;
//! - hardware selection;
//! - execution scheduling;
//! - a competing tensor IR.
//!
//! # Scalability
//!
//! There are no language-level limits on tensor rank or dimension size.
//! The canonical semantic representation uses `Vec` and `u128`.
//!
//! Operations are bounded by the input representation and available resources.
//! Allocation failures are reported where the standard library exposes
//! fallible reservation. This module does not promise execution on a target
//! that cannot satisfy the program's requirements.
//!
//! # Integration
//!
//! The frontend semantic adapter supplies a `DimensionResolver` backed by
//! the canonical constraint system.
//!
//! `tensor/shapes.rs` may re-export or compose these operations.
//!
//! `tensor/symbolic.rs` may build on substitution and transformation APIs.
//!
//! `tensor/interoperability.rs` converts validated concrete dimensions to
//! the canonical classical tensor representation or ZUIR tensor dialect.
//!
//! Neither adapter should duplicate the algorithms in this module.

use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::frontend::semantic::domains::ai::{
    AiId,
    DimensionExpr,
    TensorShape,
};

/// Knowledge established about a dimension by semantic analysis.
///
/// `Unknown` is not an error: a symbolic dimension may be resolved later.
///
/// `Invalid` means the canonical semantic services have established that
/// the expression is malformed or violates a dimension constraint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DimensionKnowledge {
    Known(u128),
    Unknown,
    Invalid,
}

/// Canonical semantic resolver interface.
///
/// Production implementations must delegate to the shared semantic
/// constraint and constant-evaluation infrastructure.
///
/// The default implementation deliberately makes no assumptions about
/// symbolic values.
pub trait DimensionResolver {
    /// Resolve an expression to a concrete dimension when possible.
    fn resolve_dimension(
        &self,
        expression: &DimensionExpr,
    ) -> DimensionKnowledge;

    /// Prove whether two expressions are equal.
    ///
    /// - `Some(true)`: equality is established.
    /// - `Some(false)`: inequality is established.
    /// - `None`: insufficient information.
    fn prove_equal(
        &self,
        left: &DimensionExpr,
        right: &DimensionExpr,
    ) -> Option<bool>;
}

/// A resolver that only recognizes canonical constant expressions.
///
/// Symbolic expressions remain unresolved.
#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantResolver;

impl DimensionResolver for ConstantResolver {
    fn resolve_dimension(
        &self,
        expression: &DimensionExpr,
    ) -> DimensionKnowledge {
        match evaluate_constant(expression) {
            Ok(Some(value)) => DimensionKnowledge::Known(value),
            Ok(None) => DimensionKnowledge::Unknown,
            Err(_) => DimensionKnowledge::Invalid,
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
                DimensionKnowledge::Known(a),
                DimensionKnowledge::Known(b),
            ) => Some(a == b),

            _ => None,
        }
    }
}

/// Errors produced while evaluating a dimension expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DimensionError {
    /// Arithmetic overflowed the canonical `u128` representation.
    ArithmeticOverflow,

    /// A subtraction would produce a negative dimension.
    NegativeDimension,

    /// A division has a zero divisor.
    DivisionByZero,

    /// The expression cannot be evaluated without symbolic information.
    Unresolved,

    /// The expression is invalid according to the supplied resolver.
    InvalidExpression,

    /// A required symbolic substitution is missing.
    MissingSubstitution(AiId),

    /// A substitution would create a recursive expression.
    CyclicSubstitution,

    /// A fallible allocation could not be completed.
    AllocationFailure,

    /// A concrete dimension cannot be represented by the host.
    HostDimensionOverflow {
        value: u128,
    },
}

impl std::fmt::Display for DimensionError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::ArithmeticOverflow => {
                formatter.write_str("dimension arithmetic overflow")
            }

            Self::NegativeDimension => {
                formatter.write_str("dimension subtraction is negative")
            }

            Self::DivisionByZero => {
                formatter.write_str("dimension division by zero")
            }

            Self::Unresolved => {
                formatter.write_str("dimension remains unresolved")
            }

            Self::InvalidExpression => {
                formatter.write_str("invalid dimension expression")
            }

            Self::MissingSubstitution(id) => {
                write!(
                    formatter,
                    "missing substitution for dimension symbol {}",
                    id.as_str()
                )
            }

            Self::CyclicSubstitution => {
                formatter.write_str("cyclic dimension substitution")
            }

            Self::AllocationFailure => {
                formatter.write_str("dimension allocation failed")
            }

            Self::HostDimensionOverflow { value } => {
                write!(
                    formatter,
                    "dimension {} cannot be represented by the host",
                    value
                )
            }
        }
    }
}

impl std::error::Error for DimensionError {}

/// A dimension equality or broadcasting constraint.
///
/// These are analysis results, not executable IR operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DimensionConstraint {
    Equal {
        left: DimensionExpr,
        right: DimensionExpr,
    },

    Broadcastable {
        left: DimensionExpr,
        right: DimensionExpr,
    },
}

/// Why two shapes are incompatible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeIncompatibility {
    RankMismatch {
        left: usize,
        right: usize,
    },

    DimensionMismatch {
        left: u128,
        right: u128,
    },

    BroadcastingMismatch {
        left: u128,
        right: u128,
    },

    InvalidDimension,
}

/// Result of checking shape compatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShapeCompatibility {
    Compatible,

    Incompatible {
        axis: Option<usize>,
        reason: ShapeIncompatibility,
    },

    RequiresResolution {
        constraints: Vec<DimensionConstraint>,
    },

    /// Analysis could not complete due to a resource failure.
    ///
    /// This must not be interpreted as compatible.
    Incomplete {
        reason: DimensionError,
    },
}

impl ShapeCompatibility {
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }

    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible { .. })
    }

    #[must_use]
    pub fn requires_resolution(&self) -> bool {
        matches!(self, Self::RequiresResolution { .. })
    }

    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete { .. })
    }
}

/// Result of a shape transformation or broadcasting operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShapeAnalysis {
    /// Result shape when its dimensions are fully determined.
    pub shape: Option<TensorShape>,

    /// Compatibility and resolution status.
    pub compatibility: ShapeCompatibility,
}

impl ShapeAnalysis {
    fn compatible(shape: TensorShape) -> Self {
        Self {
            shape: Some(shape),
            compatibility: ShapeCompatibility::Compatible,
        }
    }

    fn unresolved(
        constraints: Vec<DimensionConstraint>,
    ) -> Self {
        Self {
            shape: None,
            compatibility: ShapeCompatibility::RequiresResolution {
                constraints,
            },
        }
    }

    fn incompatible(
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

    fn incomplete(reason: DimensionError) -> Self {
        Self {
            shape: None,
            compatibility: ShapeCompatibility::Incomplete {
                reason,
            },
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.compatibility.is_compatible()
            && self.shape.is_some()
    }
}

/// Evaluate a fully constant dimension expression.
///
/// Symbolic and constraint expressions return `Ok(None)`.
///
/// Arithmetic is checked. Overflow is never silently wrapped.
pub fn evaluate_constant(
    expression: &DimensionExpr,
) -> Result<Option<u128>, DimensionError> {
    match expression {
        DimensionExpr::Constant(value) => {
            Ok(Some(*value))
        }

        DimensionExpr::Symbol(_) | DimensionExpr::Constraint(_) => {
            Ok(None)
        }

        DimensionExpr::Add(left, right) => {
            evaluate_binary(left, right, u128::checked_add)
        }

        DimensionExpr::Multiply(left, right) => {
            evaluate_binary(left, right, u128::checked_mul)
        }

        DimensionExpr::Subtract(left, right) => {
            let (Some(left), Some(right)) = (
                evaluate_constant(left)?,
                evaluate_constant(right)?,
            ) else {
                return Ok(None);
            };

            left.checked_sub(right)
                .map(Some)
                .ok_or(DimensionError::NegativeDimension)
        }

        DimensionExpr::Divide(left, right) => {
            let (Some(left), Some(right)) = (
                evaluate_constant(left)?,
                evaluate_constant(right)?,
            ) else {
                return Ok(None);
            };

            if right == 0 {
                return Err(DimensionError::DivisionByZero);
            }

            Ok(Some(left / right))
        }
    }
}

fn evaluate_binary(
    left: &DimensionExpr,
    right: &DimensionExpr,
    operation: fn(u128, u128) -> Option<u128>,
) -> Result<Option<u128>, DimensionError> {
    let (Some(left), Some(right)) = (
        evaluate_constant(left)?,
        evaluate_constant(right)?,
    ) else {
        return Ok(None);
    };

    operation(left, right)
        .map(Some)
        .ok_or(DimensionError::ArithmeticOverflow)
}

/// Build a shape from canonical dimension expressions.
pub struct ShapeBuilder;

impl ShapeBuilder {
    #[must_use]
    pub fn from_dimensions(
        dimensions: Vec<DimensionExpr>,
    ) -> TensorShape {
        TensorShape::new(dimensions)
    }

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

    #[must_use]
    pub fn is_scalar(shape: &TensorShape) -> bool {
        shape.is_scalar()
    }

    #[must_use]
    pub fn is_fully_static(shape: &TensorShape) -> bool {
        shape
            .dimensions
            .iter()
            .all(|dimension| {
                matches!(
                    evaluate_constant(dimension),
                    Ok(Some(_))
                )
            })
    }

    #[must_use]
    pub fn is_symbolic(shape: &TensorShape) -> bool {
        !Self::is_fully_static(shape)
    }
}

/// Symbolic substitution map.
///
/// The map is supplied by semantic analysis; this module does not create
/// or own a second symbol table.
pub type DimensionSubstitutions = BTreeMap<AiId, DimensionExpr>;

/// Substitute symbolic dimension identities in an expression.
///
/// The function recursively expands replacements and detects cycles.
pub fn substitute_dimension(
    expression: &DimensionExpr,
    substitutions: &DimensionSubstitutions,
) -> Result<DimensionExpr, DimensionError> {
    let mut active = Vec::new();

    substitute_inner(
        expression,
        substitutions,
        &mut active,
    )
}

fn substitute_inner(
    expression: &DimensionExpr,
    substitutions: &DimensionSubstitutions,
    active: &mut Vec<AiId>,
) -> Result<DimensionExpr, DimensionError> {
    match expression {
        DimensionExpr::Constant(value) => {
            Ok(DimensionExpr::Constant(*value))
        }

        DimensionExpr::Constraint(id) => {
            Ok(DimensionExpr::Constraint(id.clone()))
        }

        DimensionExpr::Symbol(id) => {
            let Some(replacement) = substitutions.get(id) else {
                return Ok(DimensionExpr::Symbol(id.clone()));
            };

            if active.contains(id) {
                return Err(DimensionError::CyclicSubstitution);
            }

            active
                .try_reserve(1)
                .map_err(|_| DimensionError::AllocationFailure)?;

            active.push(id.clone());

            let result = substitute_inner(
                replacement,
                substitutions,
                active,
            );

            active.pop();

            result
        }

        DimensionExpr::Add(left, right) => {
            Ok(DimensionExpr::Add(
                Box::new(substitute_inner(
                    left,
                    substitutions,
                    active,
                )?),
                Box::new(substitute_inner(
                    right,
                    substitutions,
                    active,
                )?),
            ))
        }

        DimensionExpr::Multiply(left, right) => {
            Ok(DimensionExpr::Multiply(
                Box::new(substitute_inner(
                    left,
                    substitutions,
                    active,
                )?),
                Box::new(substitute_inner(
                    right,
                    substitutions,
                    active,
                )?),
            ))
        }

        DimensionExpr::Subtract(left, right) => {
            Ok(DimensionExpr::Subtract(
                Box::new(substitute_inner(
                    left,
                    substitutions,
                    active,
                )?),
                Box::new(substitute_inner(
                    right,
                    substitutions,
                    active,
                )?),
            ))
        }

        DimensionExpr::Divide(left, right) => {
            Ok(DimensionExpr::Divide(
                Box::new(substitute_inner(
                    left,
                    substitutions,
                    active,
                )?),
                Box::new(substitute_inner(
                    right,
                    substitutions,
                    active,
                )?),
            ))
        }
    }
}

/// Substitute dimensions throughout a shape.
pub fn substitute_shape(
    shape: &TensorShape,
    substitutions: &DimensionSubstitutions,
) -> Result<TensorShape, DimensionError> {
    let mut dimensions = Vec::new();

    dimensions
        .try_reserve_exact(shape.rank())
        .map_err(|_| DimensionError::AllocationFailure)?;

    for dimension in &shape.dimensions {
        dimensions.push(substitute_dimension(
            dimension,
            substitutions,
        )?);
    }

    Ok(TensorShape::new(dimensions))
}

/// Shape compatibility analysis.
pub struct ShapeCompatibilityAnalyzer;

impl ShapeCompatibilityAnalyzer {
    /// Check whether shapes have equal rank and equal dimensions.
    ///
    /// Symbolic equalities remain explicit constraints.
    pub fn equal<R: DimensionResolver>(
        left: &TensorShape,
        right: &TensorShape,
        resolver: &R,
    ) -> ShapeCompatibility {
        if left.rank() != right.rank() {
            return ShapeCompatibility::Incompatible {
                axis: None,
                reason: ShapeIncompatibility::RankMismatch {
                    left: left.rank(),
                    right: right.rank(),
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

            match (
                resolver.resolve_dimension(left_dim),
                resolver.resolve_dimension(right_dim),
            ) {
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
                        reason: ShapeIncompatibility::InvalidDimension,
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

                _ => match resolver.prove_equal(
                    left_dim,
                    right_dim,
                ) {
                    Some(true) => {}

                    Some(false) => {
                        return ShapeCompatibility::Incompatible {
                            axis: Some(axis),
                            reason: ShapeIncompatibility::InvalidDimension,
                        };
                    }

                    None => {
                        constraints.push(
                            DimensionConstraint::Equal {
                                left: left_dim.clone(),
                                right: right_dim.clone(),
                            },
                        );
                    }
                },
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

    /// Analyze broadcasting using trailing-axis alignment.
    ///
    /// Standard broadcasting permits equal dimensions or a dimension of one.
    /// Missing leading dimensions are treated as implicit ones.
    ///
    /// If symbolic dimensions cannot be resolved, the result retains the
    /// partial output shape and reports all unresolved constraints.
    pub fn broadcast<R: DimensionResolver>(
        left: &TensorShape,
        right: &TensorShape,
        resolver: &R,
    ) -> ShapeAnalysis {
        let result_rank = left.rank().max(right.rank());

        let mut output = Vec::new();

        if output
            .try_reserve_exact(result_rank)
            .is_err()
        {
            return ShapeAnalysis::incomplete(
                DimensionError::AllocationFailure,
            );
        }

        let mut constraints = Vec::new();

        for trailing_axis in 0..result_rank {
            let left_index = left
                .rank()
                .checked_sub(trailing_axis + 1);

            let right_index = right
                .rank()
                .checked_sub(trailing_axis + 1);

            let left_dim = left_index
                .map(|index| &left.dimensions[index]);

            let right_dim = right_index
                .map(|index| &right.dimensions[index]);

            let dimension = match (left_dim, right_dim) {
                (Some(a), Some(b)) => {
                    match broadcast_dimension(
                        a,
                        b,
                        resolver,
                    ) {
                        Ok(Some(value)) => Some(value),

                        Ok(None) => {
                            constraints.push(
                                DimensionConstraint::Broadcastable {
                                    left: a.clone(),
                                    right: b.clone(),
                                },
                            );

                            None
                        }

                        Err(reason) => {
                            return ShapeAnalysis::incompatible(
                                Some(trailing_axis),
                                reason,
                            );
                        }
                    }
                }

                (Some(value), None)
                | (None, Some(value)) => {
                    Some(value.clone())
                }

                (None, None) => None,
            };

            if let Some(dimension) = dimension {
                output.push(dimension);
            } else if constraints.is_empty() {
                return ShapeAnalysis::incompatible(
                    Some(trailing_axis),
                    ShapeIncompatibility::InvalidDimension,
                );
            }
        }

        output.reverse();

        let partial_shape = TensorShape::new(output);

        if constraints.is_empty() {
            ShapeAnalysis::compatible(partial_shape)
        } else {
            ShapeAnalysis {
                shape: Some(partial_shape),
                compatibility: ShapeCompatibility::RequiresResolution {
                    constraints,
                },
            }
        }
    }
}

fn broadcast_dimension<R: DimensionResolver>(
    left: &DimensionExpr,
    right: &DimensionExpr,
    resolver: &R,
) -> Result<Option<DimensionExpr>, ShapeIncompatibility> {
    if left == right {
        return Ok(Some(left.clone()));
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
        ) => Err(ShapeIncompatibility::InvalidDimension),

        (
            DimensionKnowledge::Known(1),
            _,
        ) => Ok(Some(right.clone())),

        (
            _,
            DimensionKnowledge::Known(1),
        ) => Ok(Some(left.clone())),

        (
            DimensionKnowledge::Known(a),
            DimensionKnowledge::Known(b),
        ) if a == b => Ok(Some(left.clone())),

        (
            DimensionKnowledge::Known(a),
            DimensionKnowledge::Known(b),
        ) => Err(
            ShapeIncompatibility::BroadcastingMismatch {
                left: a,
                right: b,
            },
        ),

        _ => match resolver.prove_equal(left, right) {
            Some(true) => Ok(Some(left.clone())),

            Some(false) => {
                // Inequality alone is insufficient to reject broadcasting:
                // one side may still be one. The resolver must establish
                // concrete values or a more specific constraint.
                Ok(None)
            }

            None => Ok(None),
        },
    }
}

/// Shape transformations that preserve canonical dimension expressions.
pub struct ShapeTransformations;

impl ShapeTransformations {
    /// Insert a dimension at an axis.
    ///
    /// The axis is valid in `0..=rank`.
    pub fn insert_axis(
        shape: &TensorShape,
        axis: usize,
        dimension: DimensionExpr,
    ) -> Result<TensorShape, DimensionError> {
        if axis > shape.rank() {
            return Err(DimensionError::InvalidExpression);
        }

        let new_rank = shape
            .rank()
            .checked_add(1)
            .ok_or(DimensionError::ArithmeticOverflow)?;

        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(new_rank)
            .map_err(|_| DimensionError::AllocationFailure)?;

        dimensions.extend_from_slice(
            &shape.dimensions[..axis],
        );

        dimensions.push(dimension);

        dimensions.extend_from_slice(
            &shape.dimensions[axis..],
        );

        Ok(TensorShape::new(dimensions))
    }

    /// Remove one axis.
    pub fn remove_axis(
        shape: &TensorShape,
        axis: usize,
    ) -> Result<TensorShape, DimensionError> {
        if axis >= shape.rank() {
            return Err(DimensionError::InvalidExpression);
        }

        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(shape.rank() - 1)
            .map_err(|_| DimensionError::AllocationFailure)?;

        dimensions.extend_from_slice(
            &shape.dimensions[..axis],
        );

        dimensions.extend_from_slice(
            &shape.dimensions[axis + 1..],
        );

        Ok(TensorShape::new(dimensions))
    }

    /// Permute dimensions according to a caller-supplied axis mapping.
    ///
    /// Every axis must occur exactly once.
    pub fn permute_axes(
        shape: &TensorShape,
        axes: &[usize],
    ) -> Result<TensorShape, DimensionError> {
        if axes.len() != shape.rank() {
            return Err(DimensionError::InvalidExpression);
        }

        let mut seen = Vec::new();

        seen.try_reserve_exact(shape.rank())
            .map_err(|_| DimensionError::AllocationFailure)?;

        seen.resize(shape.rank(), false);

        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(shape.rank())
            .map_err(|_| DimensionError::AllocationFailure)?;

        for &axis in axes {
            let Some(is_seen) = seen.get_mut(axis) else {
                return Err(DimensionError::InvalidExpression);
            };

            if *is_seen {
                return Err(DimensionError::InvalidExpression);
            }

            *is_seen = true;

            dimensions.push(
                shape.dimensions[axis].clone(),
            );
        }

        Ok(TensorShape::new(dimensions))
    }

    /// Reverse the order of dimensions.
    pub fn reverse_axes(
        shape: &TensorShape,
    ) -> Result<TensorShape, DimensionError> {
        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(shape.rank())
            .map_err(|_| DimensionError::AllocationFailure)?;

        dimensions.extend(
            shape.dimensions.iter().rev().cloned(),
        );

        Ok(TensorShape::new(dimensions))
    }
}

/// Shape validation using the canonical resolver.
pub struct ShapeValidator;

impl ShapeValidator {
    /// Validate each dimension without imposing a rank limit.
    pub fn validate<R: DimensionResolver>(
        shape: &TensorShape,
        resolver: &R,
    ) -> Result<(), DimensionError> {
        for dimension in &shape.dimensions {
            match resolver.resolve_dimension(dimension) {
                DimensionKnowledge::Known(_) => {}

                DimensionKnowledge::Unknown => {}

                DimensionKnowledge::Invalid => {
                    return Err(
                        DimensionError::InvalidExpression,
                    );
                }
            }
        }

        Ok(())
    }

    /// Convert a fully concrete shape to host-sized dimensions.
    ///
    /// This is a metadata conversion only. It does not allocate tensor
    /// storage and does not imply that the resulting tensor is feasible.
    pub fn concrete_dimensions<R: DimensionResolver>(
        shape: &TensorShape,
        resolver: &R,
    ) -> Result<Vec<usize>, DimensionError> {
        let mut dimensions = Vec::new();

        dimensions
            .try_reserve_exact(shape.rank())
            .map_err(|_| DimensionError::AllocationFailure)?;

        for dimension in &shape.dimensions {
            match resolver.resolve_dimension(dimension) {
                DimensionKnowledge::Known(value) => {
                    let host_dimension =
                        usize::try_from(value).map_err(|_| {
                            DimensionError::HostDimensionOverflow {
                                value,
                            }
                        })?;

                    dimensions.push(host_dimension);
                }

                DimensionKnowledge::Unknown => {
                    return Err(DimensionError::Unresolved);
                }

                DimensionKnowledge::Invalid => {
                    return Err(
                        DimensionError::InvalidExpression,
                    );
                }
            }
        }

        Ok(dimensions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constant_shape(values: &[u128]) -> TensorShape {
        ShapeBuilder::from_constants(values.to_vec())
    }

    #[test]
    fn evaluates_checked_constant_arithmetic() {
        let expression = DimensionExpr::Multiply(
            Box::new(DimensionExpr::Constant(4)),
            Box::new(DimensionExpr::Constant(8)),
        );

        assert_eq!(
            evaluate_constant(&expression),
            Ok(Some(32)),
        );
    }

    #[test]
    fn detects_arithmetic_overflow() {
        let expression = DimensionExpr::Add(
            Box::new(DimensionExpr::Constant(u128::MAX)),
            Box::new(DimensionExpr::Constant(1)),
        );

        assert_eq!(
            evaluate_constant(&expression),
            Err(DimensionError::ArithmeticOverflow),
        );
    }

    #[test]
    fn rejects_negative_dimension_subtraction() {
        let expression = DimensionExpr::Subtract(
            Box::new(DimensionExpr::Constant(2)),
            Box::new(DimensionExpr::Constant(3)),
        );

        assert_eq!(
            evaluate_constant(&expression),
            Err(DimensionError::NegativeDimension),
        );
    }

    #[test]
    fn detects_division_by_zero() {
        let expression = DimensionExpr::Divide(
            Box::new(DimensionExpr::Constant(10)),
            Box::new(DimensionExpr::Constant(0)),
        );

        assert_eq!(
            evaluate_constant(&expression),
            Err(DimensionError::DivisionByZero),
        );
    }

    #[test]
    fn scalar_shape_has_zero_rank() {
        let shape = TensorShape::new(Vec::new());

        assert!(shape.is_scalar());
        assert_eq!(shape.rank(), 0);
    }

    #[test]
    fn supports_arbitrary_rank_representation() {
        let dimensions = (0..256)
            .map(|value| {
                DimensionExpr::Constant(value)
            })
            .collect();

        let shape = ShapeBuilder::from_dimensions(dimensions);

        assert_eq!(shape.rank(), 256);
    }

    #[test]
    fn broadcasts_equal_shapes() {
        let resolver = ConstantResolver;

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &constant_shape(&[2, 3]),
                &constant_shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(
            result.shape,
            Some(constant_shape(&[2, 3])),
        );
    }

    #[test]
    fn broadcasts_singleton_dimension() {
        let resolver = ConstantResolver;

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &constant_shape(&[1, 3]),
                &constant_shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(
            result.shape,
            Some(constant_shape(&[2, 3])),
        );
    }

    #[test]
    fn broadcasts_different_ranks() {
        let resolver = ConstantResolver;

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &constant_shape(&[3]),
                &constant_shape(&[2, 3]),
                &resolver,
            );

        assert!(result.is_valid());
        assert_eq!(
            result.shape,
            Some(constant_shape(&[2, 3])),
        );
    }

    #[test]
    fn rejects_incompatible_broadcasting() {
        let resolver = ConstantResolver;

        let result =
            ShapeCompatibilityAnalyzer::broadcast(
                &constant_shape(&[2, 3]),
                &constant_shape(&[4, 5]),
                &resolver,
            );

        assert!(
            result.compatibility.is_incompatible(),
        );
    }

    #[test]
    fn substitutes_symbolic_dimensions() {
        let symbol = AiId::new("batch").unwrap();

        let expression =
            DimensionExpr::Symbol(symbol.clone());

        let mut substitutions =
            DimensionSubstitutions::new();

        substitutions.insert(
            symbol,
            DimensionExpr::Constant(16),
        );

        let result = substitute_dimension(
            &expression,
            &substitutions,
        );

        assert_eq!(
            result,
            Ok(DimensionExpr::Constant(16)),
        );
    }

    #[test]
    fn rejects_cyclic_substitution() {
        let first = AiId::new("first").unwrap();
        let second = AiId::new("second").unwrap();

        let mut substitutions =
            DimensionSubstitutions::new();

        substitutions.insert(
            first.clone(),
            DimensionExpr::Symbol(second.clone()),
        );

        substitutions.insert(
            second,
            DimensionExpr::Symbol(first.clone()),
        );

        assert_eq!(
            substitute_dimension(
                &DimensionExpr::Symbol(first),
                &substitutions,
            ),
            Err(DimensionError::CyclicSubstitution),
        );
    }

    #[test]
    fn inserts_and_removes_axis() {
        let shape = constant_shape(&[2, 3]);

        let inserted = ShapeTransformations::insert_axis(
            &shape,
            1,
            DimensionExpr::Constant(7),
        )
        .unwrap();

        assert_eq!(
            inserted,
            constant_shape(&[2, 7, 3]),
        );

        let removed = ShapeTransformations::remove_axis(
            &inserted,
            1,
        )
        .unwrap();

        assert_eq!(removed, shape);
    }

    #[test]
    fn permutes_axes() {
        let shape = constant_shape(&[2, 3, 4]);

        let result = ShapeTransformations::permute_axes(
            &shape,
            &[2, 0, 1],
        )
        .unwrap();

        assert_eq!(
            result,
            constant_shape(&[4, 2, 3]),
        );
    }

    #[test]
    fn rejects_invalid_axis_permutation() {
        let shape = constant_shape(&[2, 3]);

        assert!(
            ShapeTransformations::permute_axes(
                &shape,
                &[0, 0],
            )
            .is_err(),
        );
    }

    #[test]
    fn converts_concrete_dimensions() {
        let dimensions =
            ShapeValidator::concrete_dimensions(
                &constant_shape(&[2, 3, 4]),
                &ConstantResolver,
            )
            .unwrap();

        assert_eq!(dimensions, vec![2, 3, 4]);
    }

    #[test]
    fn zero_length_dimension_is_valid() {
        let shape = constant_shape(&[0, 3]);

        assert!(
            ShapeValidator::validate(
                &shape,
                &ConstantResolver,
            )
            .is_ok(),
        );
    }
}
