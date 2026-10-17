
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical shape validation.
//!
//! Path: `src/classical/validation/shapes.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # File contract
//!
//! Status: Proposed implementation; repository integration and compilation
//! must be verified before this file is marked complete.
//!
//! ## Purpose
//!
//! Validate target-independent shape metadata and shape relationships used by
//! classical arrays, matrices, vectors, tensors, and other shape-parametric
//! computations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/types.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/types/array.g4`
//! - `src/frontend/ast/node/types/type_expr.rs`
//! - `src/frontend/semantic/types/ty.rs`
//!
//! ## Ownership
//!
//! This module owns shape-validation rules and their structured diagnostics.
//! It does not own a shape type system.
//!
//! Canonical semantic cardinalities and symbolic dimension expressions are
//! represented by `Cardinality` and `TypeLevelValue` in the frontend semantic
//! type system. This module consumes those types directly.
//!
//! ## Does not own
//!
//! - Grammar rules, parsing, or AST nodes.
//! - Name resolution, type inference, or constraint solving.
//! - Canonical semantic types or type-level expression representation.
//! - Concrete collection storage or runtime allocation.
//! - Target ABI, physical layout, device placement, or scheduling.
//! - Canonical classical IR or quantum IR.
//! - Runtime shape checks for expressions whose values are not statically known.
//!
//! ## Shape invariants
//!
//! - A shape is an ordered sequence of canonical semantic cardinalities.
//! - An empty shape is permitted here and represents rank zero. Concrete array
//!   APIs may impose their own array-specific requirements.
//! - Zero is a valid known dimension.
//! - Known dimensions must contain non-negative integer values.
//! - Symbolic and unspecified dimensions are never silently converted to zero.
//! - Different symbolic expressions are not assumed equal or unequal merely
//!   because their representations differ.
//! - A statically unprovable relationship requires a constraint proof or a
//!   runtime check owned by the appropriate downstream component.
//! - Broadcasting follows trailing-dimension alignment.
//! - No universal rank or dimension maximum is imposed.
//!
//! ## Integration
//!
//! 1. The frontend parses shape-related syntax into the existing `TypeExpr`.
//! 2. Semantic analysis resolves that syntax to canonical `SemanticType`,
//!    `Cardinality`, and `TypeLevelValue` values.
//! 3. This module validates shape metadata and statically decidable
//!    compatibility or broadcasting relationships.
//! 4. The caller routes unresolved relationships to canonical constraint
//!    solving or explicit runtime-check planning.
//! 5. Lowering maps validated operations into the existing canonical IR.
//! 6. Concrete collection code continues to use
//!    `crate::classical::collections::array::ClassicalArrayShape`.
//!
//! A successful validation result does not prove that a target has sufficient
//! memory, supports a particular operation, or can materialize the shape.
//!
//! ## Resource policy
//!
//! The functions in this module introduce no policy-level rank limit. They
//! perform work proportional to the supplied shape lengths. Callers processing
//! untrusted inputs may apply explicit operational budgets in shared compiler
//! configuration. Such budgets must not change language-level shape meaning.
//!
//! ## Freeze rule
//!
//! Changes to known/symbolic/unspecified dimension semantics, broadcasting,
//! or error classifications require specification and compatibility review.

use core::fmt;

use crate::frontend::semantic::types::ty::{Cardinality, TypeLevelValue};

// =============================================================================
// Structured diagnostics
// =============================================================================

/// The statically established relationship between two dimensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimensionRelation {
    /// Both dimensions are known to represent the same value.
    Equal,

    /// The available information does not prove equality or incompatibility.
    ///
    /// A caller may need canonical constraint solving or a runtime check.
    RequiresFurtherProof,
}

/// Failure produced while validating or comparing shape metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShapeValidationError {
    /// A known dimension did not contain a valid non-negative integer.
    InvalidKnownDimension {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// A dimension's canonical symbolic value is structurally invalid.
    InvalidDimensionValue {
        /// Zero-based dimension index.
        dimension: usize,
        /// Canonical semantic validation failure.
        source: crate::frontend::semantic::types::ty::TypeValidationError,
    },

    /// Two shapes have different ranks where equal ranks are required.
    RankMismatch {
        /// Rank of the first shape.
        expected: usize,
        /// Rank of the second shape.
        actual: usize,
    },

    /// Two known dimensions cannot satisfy the requested relationship.
    IncompatibleDimensions {
        /// Zero-based index in the supplied shape, when applicable.
        dimension: usize,
    },

    /// The requested relationship cannot be established statically.
    ///
    /// This is not proof of incompatibility. The caller must arrange further
    /// constraint solving or a runtime check if the operation permits it.
    RequiresFurtherProof {
        /// Zero-based dimension index, when comparing equal-rank shapes.
        dimension: Option<usize>,
    },

    /// A fallible reservation for result metadata failed.
    ///
    /// The request could not be represented using currently available
    /// allocation resources. This is an operational failure, not a language
    /// semantic maximum.
    MetadataAllocationFailed {
        /// Number of dimension entries requested for reservation.
        requested_dimensions: usize,
    },
}

impl fmt::Display for ShapeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKnownDimension { dimension } => {
                write!(
                    f,
                    "known shape dimension {dimension} is not a valid non-negative integer"
                )
            }
            Self::InvalidDimensionValue { dimension, source } => {
                write!(
                    f,
                    "shape dimension {dimension} contains an invalid semantic value: {source}"
                )
            }
            Self::RankMismatch { expected, actual } => {
                write!(
                    f,
                    "shape rank mismatch: expected {expected}, received {actual}"
                )
            }
            Self::IncompatibleDimensions { dimension } => {
                write!(
                    f,
                    "shape dimension {dimension} is statically incompatible"
                )
            }
            Self::RequiresFurtherProof { dimension } => {
                match dimension {
                    Some(dimension) => write!(
                        f,
                        "shape compatibility at dimension {dimension} requires further proof"
                    ),
                    None => f.write_str(
                        "shape compatibility requires further proof",
                    ),
                }
            }
            Self::MetadataAllocationFailed {
                requested_dimensions,
            } => write!(
                f,
                "unable to reserve metadata for {requested_dimensions} shape dimensions"
            ),
        }
    }
}

impl std::error::Error for ShapeValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidDimensionValue { source, .. } => Some(source),
            _ => None,
        }
    }
}

// =============================================================================
// Shape validation
// =============================================================================

/// Validates each dimension in a canonical semantic shape.
///
/// An empty slice is accepted: shape rank zero is meaningful at the semantic
/// level. A concrete array API may separately reject rank-zero arrays.
///
/// This function validates metadata only. It performs no element allocation,
/// dimension inference, target selection, or memory-feasibility check.
pub fn validate_shape(
    dimensions: &[Cardinality],
) -> Result<(), ShapeValidationError> {
    for (index, dimension) in dimensions.iter().enumerate() {
        validate_dimension(dimension, index)?;
    }

    Ok(())
}

/// Validates one canonical semantic cardinality.
pub fn validate_dimension(
    dimension: &Cardinality,
    index: usize,
) -> Result<(), ShapeValidationError> {
    match dimension {
        Cardinality::Known(value) => {
            validate_type_level_value(value, index)?;

            // A Known cardinality must be concrete. Symbolic names and
            // expressions belong in Cardinality::Symbolic instead.
            match value {
                TypeLevelValue::Integer(decimal) => {
                    if !is_decimal_integer(decimal) {
                        return Err(
                            ShapeValidationError::InvalidKnownDimension {
                                dimension: index,
                            },
                        );
                    }
                }
                _ => {
                    return Err(
                        ShapeValidationError::InvalidKnownDimension {
                            dimension: index,
                        },
                    );
                }
            }
        }
        Cardinality::Symbolic(value) => {
            validate_type_level_value(value, index)?;
        }
        Cardinality::Unspecified => {}
    }

    Ok(())
}

fn validate_type_level_value(
    value: &TypeLevelValue,
    index: usize,
) -> Result<(), ShapeValidationError> {
    value.validate().map_err(|source| {
        ShapeValidationError::InvalidDimensionValue {
            dimension: index,
            source,
        }
    })
}

fn is_decimal_integer(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

/// Compares two cardinalities without assuming unknown symbolic relationships.
///
/// Structurally identical symbolic expressions are equal for this comparison.
/// Distinct symbolic expressions require further proof: they are not
/// automatically incompatible.
///
/// Known integer values are compared as arbitrary-length decimal strings,
/// without narrowing them to `usize` or another machine integer.
pub fn compare_dimensions(
    left: &Cardinality,
    right: &Cardinality,
) -> Result<DimensionRelation, ShapeValidationError> {
    validate_dimension(left, 0)?;
    validate_dimension(right, 0)?;

    match (left, right) {
        (Cardinality::Unspecified, _)
        | (_, Cardinality::Unspecified) => {
            Ok(DimensionRelation::RequiresFurtherProof)
        }

        (
            Cardinality::Known(TypeLevelValue::Integer(left_value)),
            Cardinality::Known(TypeLevelValue::Integer(right_value)),
        ) => {
            if decimal_values_equal(left_value, right_value) {
                Ok(DimensionRelation::Equal)
            } else {
                Err(ShapeValidationError::IncompatibleDimensions {
                    dimension: 0,
                })
            }
        }

        (left, right) if left == right => Ok(DimensionRelation::Equal),

        // Two different symbolic expressions may still be equal after
        // substitution or constraint solving. Do not guess.
        _ => Ok(DimensionRelation::RequiresFurtherProof),
    }
}

/// Compares two shapes whose ranks must match.
///
/// This function reports the first statically incompatible dimension.
/// Symbolic or unspecified dimensions that need additional proof are reported
/// as `RequiresFurtherProof`; they are not misclassified as incompatible.
pub fn compare_shapes(
    left: &[Cardinality],
    right: &[Cardinality],
) -> Result<DimensionRelation, ShapeValidationError> {
    validate_shape(left)?;
    validate_shape(right)?;

    if left.len() != right.len() {
        return Err(ShapeValidationError::RankMismatch {
            expected: left.len(),
            actual: right.len(),
        });
    }

    let mut needs_further_proof = false;

    for (index, (left_dimension, right_dimension)) in
        left.iter().zip(right.iter()).enumerate()
    {
        match compare_dimensions(left_dimension, right_dimension) {
            Ok(DimensionRelation::Equal) => {}
            Ok(DimensionRelation::RequiresFurtherProof) => {
                needs_further_proof = true;
            }
            Err(ShapeValidationError::IncompatibleDimensions { .. }) => {
                return Err(ShapeValidationError::IncompatibleDimensions {
                    dimension: index,
                });
            }
            Err(error) => return Err(error),
        }
    }

    if needs_further_proof {
        Ok(DimensionRelation::RequiresFurtherProof)
    } else {
        Ok(DimensionRelation::Equal)
    }
}

// =============================================================================
// Broadcasting
// =============================================================================

/// Computes a statically justified broadcast shape.
///
/// Dimensions are aligned from the trailing end, as in conventional
/// multidimensional array broadcasting.
///
/// Rules:
///
/// - Equal known dimensions are preserved.
/// - A known dimension of one can broadcast to the other dimension.
/// - A known dimension of zero is valid and is not treated as one.
/// - Identical symbolic cardinalities are preserved.
/// - A symbolic or unspecified relationship that cannot be proven requires
///   further proof and returns `RequiresFurtherProof`.
/// - Different known dimensions other than one are incompatible.
///
/// The returned shape contains canonical `Cardinality` values. This function
/// does not create a new shape type or execute a runtime broadcast.
///
/// If the operation needs runtime-dependent broadcasting, the caller should
/// arrange an explicit runtime check rather than interpreting this error as
/// proof that the operation is impossible.
pub fn broadcast_shapes(
    left: &[Cardinality],
    right: &[Cardinality],
) -> Result<Vec<Cardinality>, ShapeValidationError> {
    validate_shape(left)?;
    validate_shape(right)?;

    let result_rank = left.len().max(right.len());
    let mut result = Vec::new();

    result
        .try_reserve_exact(result_rank)
        .map_err(|_| ShapeValidationError::MetadataAllocationFailed {
            requested_dimensions: result_rank,
        })?;

    // Build the result from the trailing dimensions. Reversing at the end
    // restores the canonical outermost-to-innermost order.
    for offset in 0..result_rank {
        let left_dimension = left
            .len()
            .checked_sub(offset + 1)
            .and_then(|index| left.get(index));

        let right_dimension = right
            .len()
            .checked_sub(offset + 1)
            .and_then(|index| right.get(index));

        let dimension = match (left_dimension, right_dimension) {
            // A missing leading dimension behaves as a broadcast dimension
            // of one; it is not added to the source shape itself.
            (Some(left_dimension), Some(right_dimension)) => {
                broadcast_dimension(
                    left_dimension,
                    right_dimension,
                    offset,
                )?
            }
            (Some(dimension), None) | (None, Some(dimension)) => {
                dimension.clone()
            }
            (None, None) => {
                // The loop range makes this branch unreachable, but retaining
                // an explicit branch keeps the matching exhaustive.
                continue;
            }
        };

        result.push(dimension);
    }

    result.reverse();
    Ok(result)
}

fn broadcast_dimension(
    left: &Cardinality,
    right: &Cardinality,
    axis_from_right: usize,
) -> Result<Cardinality, ShapeValidationError> {
    match (left, right) {
        (
            Cardinality::Known(TypeLevelValue::Integer(left_value)),
            Cardinality::Known(TypeLevelValue::Integer(right_value)),
        ) => {
            if decimal_values_equal(left_value, right_value) {
                return Ok(left.clone());
            }

            if decimal_is_one(left_value) {
                return Ok(right.clone());
            }

            if decimal_is_one(right_value) {
                return Ok(left.clone());
            }

            Err(ShapeValidationError::IncompatibleDimensions {
                dimension: axis_from_right,
            })
        }

        // A proven one broadcasts against a symbolic dimension without
        // requiring the symbolic value to be evaluated.
        (
            Cardinality::Known(TypeLevelValue::Integer(value)),
            other,
        ) if decimal_is_one(value) => Ok(other.clone()),

        (
            other,
            Cardinality::Known(TypeLevelValue::Integer(value)),
        ) if decimal_is_one(value) => Ok(other.clone()),

        // Structurally identical symbolic dimensions have a common result.
        (left, right) if left == right => Ok(left.clone()),

        // Unspecified and non-identical symbolic dimensions cannot be
        // resolved here. Returning one side would silently guess the result.
        _ => Err(ShapeValidationError::RequiresFurtherProof {
            dimension: Some(axis_from_right),
        }),
    }
}

// =============================================================================
// Arbitrary-length decimal comparison
// =============================================================================

/// Compares non-negative decimal strings without converting them to a machine
/// integer. Leading zeroes are ignored for comparison.
///
/// The caller validates decimal syntax before using this helper.
fn decimal_values_equal(left: &str, right: &str) -> bool {
    normalized_decimal(left) == normalized_decimal(right)
}

fn decimal_is_one(value: &str) -> bool {
    normalized_decimal(value) == "1"
}

fn normalized_decimal(value: &str) -> &str {
    let without_leading_zeroes = value.trim_start_matches('0');

    if without_leading_zeroes.is_empty() {
        "0"
    } else {
        without_leading_zeroes
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::types::ty::TypeLevelValue;

    fn known(value: &str) -> Cardinality {
        Cardinality::Known(
            TypeLevelValue::integer(value)
                .expect("test dimension should be a valid integer"),
        )
    }

    fn symbolic(name: &str) -> Cardinality {
        Cardinality::Symbolic(TypeLevelValue::Name(
            crate::frontend::semantic::types::ty::QualifiedName::single(name),
        ))
    }

    #[test]
    fn accepts_rank_zero_shape() {
        assert!(validate_shape(&[]).is_ok());
    }

    #[test]
    fn accepts_zero_length_dimension() {
        assert!(validate_shape(&[known("0")]).is_ok());
    }

    #[test]
    fn validates_arbitrarily_long_decimal_dimensions_without_narrowing() {
        let dimension =
            known("999999999999999999999999999999999999999999999999");

        assert!(validate_shape(&[dimension]).is_ok());
    }

    #[test]
    fn compares_decimal_dimensions_without_machine_integer_conversion() {
        assert_eq!(
            compare_dimensions(
                &known("00000000000000000012"),
                &known("12")
            ),
            Ok(DimensionRelation::Equal)
        );
    }

    #[test]
    fn rejects_incompatible_known_shapes() {
        assert_eq!(
            compare_shapes(&[known("2"), known("3")], &[known("2"), known("4")]),
            Err(ShapeValidationError::IncompatibleDimensions {
                dimension: 1
            })
        );
    }

    #[test]
    fn reports_rank_mismatch() {
        assert_eq!(
            compare_shapes(&[known("2")], &[known("2"), known("3")]),
            Err(ShapeValidationError::RankMismatch {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn different_symbolic_dimensions_require_further_proof() {
        assert_eq!(
            compare_dimensions(&symbolic("N"), &symbolic("M")),
            Ok(DimensionRelation::RequiresFurtherProof)
        );
    }

    #[test]
    fn identical_symbolic_dimensions_are_equal() {
        assert_eq!(
            compare_dimensions(&symbolic("N"), &symbolic("N")),
            Ok(DimensionRelation::Equal)
        );
    }

    #[test]
    fn broadcasts_trailing_dimensions() {
        let result = broadcast_shapes(
            &[known("8"), known("1"), known("4")],
            &[known("1"), known("6"), known("4")],
        )
        .expect("known compatible dimensions should broadcast");

        assert_eq!(result, vec![known("8"), known("6"), known("4")]);
    }

    #[test]
    fn broadcasts_one_against_zero_to_zero() {
        let result = broadcast_shapes(&[known("1")], &[known("0")])
            .expect("one broadcasts against zero");

        assert_eq!(result, vec![known("0")]);
    }

    #[test]
    fn rejects_incompatible_broadcast_dimensions() {
        assert_eq!(
            broadcast_shapes(&[known("2")], &[known("3")]),
            Err(ShapeValidationError::IncompatibleDimensions {
                dimension: 0
            })
        );
    }

    #[test]
    fn refuses_to_guess_symbolic_broadcast_result() {
        assert_eq!(
            broadcast_shapes(&[symbolic("N")], &[symbolic("M")]),
            Err(ShapeValidationError::RequiresFurtherProof {
                dimension: Some(0)
            })
        );
    }

    #[test]
    fn preserves_leading_dimensions_when_ranks_differ() {
        let result = broadcast_shapes(
            &[known("5"), known("1"), known("4")],
            &[known("6"), known("4")],
        )
        .expect("compatible trailing dimensions should broadcast");

        assert_eq!(result, vec![known("5"), known("6"), known("4")]);
    }
}
