
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR array values
//!
//! Path: `src/zuir/values/arrays.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//!
//! PURPOSE
//!
//! Represent target-independent array shapes and values using the canonical
//! ZUIR type and constant models.
//!
//! OWNS
//!
//! - Array shape representation.
//! - Dense, sparse, and symbolic array contents.
//! - Shape arithmetic and local representation validation.
//! - Explicit array-validation budgets and structured errors.
//!
//! DOES NOT OWN
//!
//! - Source syntax, parsing, or AST construction.
//! - Semantic type resolution or element-type compatibility.
//! - Symbolic constraint solving or expression evaluation.
//! - Physical layout, allocation, device selection, or execution.
//! - Complete-program serialization or target lowering.
//!
//! INTEGRATION
//!
//! - `types/type_def.rs` owns `TypeRef` and `Dimension`.
//! - `values/constants.rs` owns `ConstantValue` and nested value validation.
//! - Frontend grammar and AST modules own source-level array syntax.
//! - Semantic verification owns element-type compatibility.
//! - Lowering owns target-specific realization and feasibility.
//! - `values/mod.rs` declares and exports this module.
//!
//! SCALABILITY
//!
//! No universal array length, rank, hardware capacity, or element-width
//! limit is imposed. Collections are dynamically sized. Optional budgets
//! constrain validation work only; they do not redefine language semantics.
//! Concrete computation is still bounded by available resources.
//!
//! SEMANTIC BOUNDARY
//!
//! Local validation does not establish semantic type compatibility, prove
//! symbolic constraints, or guarantee that a target can execute the array.
//!
//! FREEZE CRITERIA
//!
//! Compile against the repository's actual module tree, run tests, and
//! confirm the serialized representation against the authoritative ZUIR
//! specification before marking this file frozen.

use crate::zuir::types::type_def::{Dimension, TypeRef};
use crate::zuir::values::constants::{
    ConstantValidationBudget, ConstantValidationError, ConstantValue,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// A target-independent array shape.
///
/// An empty shape represents a scalar with one element. A dynamic or
/// symbolic dimension represents a dimension that has not been resolved;
/// it does not promise unlimited runtime capacity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayShape {
    dimensions: Vec<Dimension>,
}

impl ArrayShape {
    /// Construct a shape.
    ///
    /// Call `validate` after construction and after deserialization.
    #[must_use]
    pub fn new(dimensions: Vec<Dimension>) -> Self {
        Self { dimensions }
    }

    /// Return the dimensions in logical axis order.
    #[must_use]
    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    /// Return the number of dimensions.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Return whether every dimension is statically known.
    #[must_use]
    pub fn is_fully_known(&self) -> bool {
        self.dimensions
            .iter()
            .all(|dimension| matches!(dimension, Dimension::Known(_)))
    }

    /// Compute the element count if all dimensions are known.
    ///
    /// Multiplication is checked. Overflow is an error, never a wrapped
    /// value. A zero dimension produces a zero element count.
    pub fn known_element_count(
        &self,
    ) -> Result<Option<u128>, ArrayValidationError> {
        let mut count = 1_u128;

        for dimension in &self.dimensions {
            match dimension {
                Dimension::Known(size) => {
                    count = count
                        .checked_mul(*size)
                        .ok_or(ArrayValidationError::ShapeProductOverflow)?;
                }
                Dimension::Symbolic(_) | Dimension::Dynamic => {
                    return Ok(None);
                }
            }
        }

        Ok(Some(count))
    }

    /// Validate locally represented dimensions.
    ///
    /// This does not solve symbolic expressions.
    pub fn validate(&self) -> Result<(), ArrayValidationError> {
        for dimension in &self.dimensions {
            if let Dimension::Symbolic(expression) = dimension {
                if expression.expression.trim().is_empty() {
                    return Err(ArrayValidationError::EmptySymbolicDimension);
                }
            }
        }

        let _ = self.known_element_count()?;
        Ok(())
    }
}

/// Array contents independent of any physical memory representation.
///
/// Dense elements use flat row-major order. Sparse elements contain explicit
/// coordinates only. This type does not assign a default value to omitted
/// sparse coordinates; that meaning belongs to the relevant operation or
/// dialect contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ArrayContents {
    /// Materialized elements in flat row-major order.
    Dense(Vec<ConstantValue>),

    /// Explicit coordinates mapped to values, in deterministic key order.
    Sparse(BTreeMap<Vec<u128>, ConstantValue>),

    /// An unevaluated array initializer or array-producing expression.
    Symbolic {
        expression: String,
    },
}

/// A typed array value in the canonical ZUIR value model.
///
/// This represents logical array semantics, not a physical buffer, register
/// allocation, tensor layout, or hardware address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayValue {
    /// Semantic type expected for each element.
    pub element_type: TypeRef,

    /// Logical array shape.
    pub shape: ArrayShape,

    /// Materialized or symbolic contents.
    pub contents: ArrayContents,
}

impl ArrayValue {
    /// Construct an array value without asserting semantic validity.
    #[must_use]
    pub fn new(
        element_type: TypeRef,
        shape: ArrayShape,
        contents: ArrayContents,
    ) -> Self {
        Self {
            element_type,
            shape,
            contents,
        }
    }

    /// Construct a dense array from flat row-major elements.
    #[must_use]
    pub fn dense(
        element_type: TypeRef,
        shape: ArrayShape,
        elements: Vec<ConstantValue>,
    ) -> Self {
        Self::new(
            element_type,
            shape,
            ArrayContents::Dense(elements),
        )
    }

    /// Construct a sparse array from explicit coordinates.
    #[must_use]
    pub fn sparse(
        element_type: TypeRef,
        shape: ArrayShape,
        elements: BTreeMap<Vec<u128>, ConstantValue>,
    ) -> Self {
        Self::new(
            element_type,
            shape,
            ArrayContents::Sparse(elements),
        )
    }

    /// Construct an array with a symbolic initializer.
    #[must_use]
    pub fn symbolic(
        element_type: TypeRef,
        shape: ArrayShape,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(
            element_type,
            shape,
            ArrayContents::Symbolic {
                expression: expression.into(),
            },
        )
    }

    /// Return the logical array rank.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }

    /// Return the element count if the shape is fully known.
    pub fn known_element_count(
        &self,
    ) -> Result<Option<u128>, ArrayValidationError> {
        self.shape.known_element_count()
    }

    /// Validate using a policy without explicit budgets.
    ///
    /// Use `validate_with` when handling untrusted or potentially large data.
    pub fn validate(&self) -> Result<(), ArrayValidationError> {
        self.validate_with(&ArrayValidationBudget::unlimited())
    }

    /// Validate the local array representation under an explicit budget.
    ///
    /// Rank and materialized-element limits apply to the entire array.
    /// Nested-value node and text budgets apply separately to each element;
    /// they are not aggregate whole-array budgets.
    pub fn validate_with(
        &self,
        budget: &ArrayValidationBudget,
    ) -> Result<(), ArrayValidationError> {
        self.shape.validate()?;

        if let Some(limit) = budget.max_rank {
            if self.rank() > limit {
                return Err(ArrayValidationError::RankBudgetExceeded {
                    actual: self.rank(),
                    limit,
                });
            }
        }

        match &self.contents {
            ArrayContents::Dense(elements) => {
                check_element_budget(elements.len(), budget)?;

                let expected = self
                    .shape
                    .known_element_count()?
                    .ok_or(ArrayValidationError::DenseShapeNotFullyKnown)?;

                let actual = u128::try_from(elements.len())
                    .map_err(|_| {
                        ArrayValidationError::ElementCountNotRepresentable
                    })?;

                if actual != expected {
                    return Err(
                        ArrayValidationError::DenseElementCountMismatch {
                            expected,
                            actual,
                        },
                    );
                }

                for element in elements {
                    validate_element(element, budget)?;
                }
            }

            ArrayContents::Sparse(elements) => {
                check_element_budget(elements.len(), budget)?;

                for (coordinate, value) in elements {
                    if coordinate.len() != self.rank() {
                        return Err(
                            ArrayValidationError::SparseCoordinateRankMismatch {
                                expected: self.rank(),
                                actual: coordinate.len(),
                            },
                        );
                    }

                    for (axis, index) in coordinate.iter().enumerate() {
                        match &self.shape.dimensions()[axis] {
                            Dimension::Known(size) if index >= size => {
                                return Err(
                                    ArrayValidationError::SparseIndexOutOfBounds {
                                        axis,
                                        index: *index,
                                        size: *size,
                                    },
                                );
                            }
                            Dimension::Known(_) => {}
                            Dimension::Symbolic(_) | Dimension::Dynamic => {
                                return Err(
                                    ArrayValidationError::SparseShapeNotFullyKnown,
                                );
                            }
                        }
                    }

                    validate_element(value, budget)?;
                }
            }

            ArrayContents::Symbolic { expression } => {
                if expression.trim().is_empty() {
                    return Err(
                        ArrayValidationError::EmptySymbolicInitializer,
                    );
                }

                if let Some(limit) = budget.max_symbolic_expression_bytes {
                    if expression.len() > limit {
                        return Err(
                            ArrayValidationError::SymbolicExpressionBudgetExceeded {
                                actual: expression.len(),
                                limit,
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

fn check_element_budget(
    actual: usize,
    budget: &ArrayValidationBudget,
) -> Result<(), ArrayValidationError> {
    if let Some(limit) = budget.max_materialized_elements {
        if actual > limit {
            return Err(ArrayValidationError::ElementBudgetExceeded {
                actual,
                limit,
            });
        }
    }

    Ok(())
}

fn validate_element(
    element: &ConstantValue,
    budget: &ArrayValidationBudget,
) -> Result<(), ArrayValidationError> {
    element
        .validate_with(&ConstantValidationBudget::new(
            budget.max_value_nodes_per_element,
            budget.max_text_bytes_per_element,
        ))
        .map_err(ArrayValidationError::InvalidElement)
}

/// Explicit operational safeguards for array validation.
///
/// `None` means no explicit limit for that dimension; it does not mean
/// infinite memory, time, or address space. Budgets do not alter array
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ArrayValidationBudget {
    /// Maximum rank accepted by this validation call.
    pub max_rank: Option<usize>,

    /// Maximum number of explicitly materialized dense or sparse entries.
    pub max_materialized_elements: Option<usize>,

    /// Maximum nested value nodes validated per element.
    pub max_value_nodes_per_element: Option<usize>,

    /// Maximum text bytes validated per element.
    pub max_text_bytes_per_element: Option<usize>,

    /// Maximum byte length of a symbolic initializer.
    pub max_symbolic_expression_bytes: Option<usize>,
}

impl ArrayValidationBudget {
    /// Return a policy with no explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_rank: None,
            max_materialized_elements: None,
            max_value_nodes_per_element: None,
            max_text_bytes_per_element: None,
            max_symbolic_expression_bytes: None,
        }
    }

    /// Construct a policy with explicit optional limits.
    #[must_use]
    pub const fn new(
        max_rank: Option<usize>,
        max_materialized_elements: Option<usize>,
        max_value_nodes_per_element: Option<usize>,
        max_text_bytes_per_element: Option<usize>,
        max_symbolic_expression_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_rank,
            max_materialized_elements,
            max_value_nodes_per_element,
            max_text_bytes_per_element,
            max_symbolic_expression_bytes,
        }
    }
}

/// Local array representation errors.
///
/// Type compatibility, symbolic constraint solving, and target feasibility
/// are deliberately reported by their respective subsystems.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArrayValidationError {
    EmptySymbolicDimension,
    ShapeProductOverflow,
    DenseShapeNotFullyKnown,
    ElementCountNotRepresentable,
    DenseElementCountMismatch {
        expected: u128,
        actual: u128,
    },
    RankBudgetExceeded {
        actual: usize,
        limit: usize,
    },
    ElementBudgetExceeded {
        actual: usize,
        limit: usize,
    },
    SparseShapeNotFullyKnown,
    SparseCoordinateRankMismatch {
        expected: usize,
        actual: usize,
    },
    SparseIndexOutOfBounds {
        axis: usize,
        index: u128,
        size: u128,
    },
    EmptySymbolicInitializer,
    SymbolicExpressionBudgetExceeded {
        actual: usize,
        limit: usize,
    },
    InvalidElement(ConstantValidationError),
}

impl fmt::Display for ArrayValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySymbolicDimension => {
                f.write_str("symbolic array dimension is empty")
            }
            Self::ShapeProductOverflow => {
                f.write_str("array shape product overflowed")
            }
            Self::DenseShapeNotFullyKnown => {
                f.write_str("dense array requires a fully known shape")
            }
            Self::ElementCountNotRepresentable => {
                f.write_str("materialized element count is not representable")
            }
            Self::DenseElementCountMismatch { expected, actual } => {
                write!(
                    f,
                    "dense array element count mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RankBudgetExceeded { actual, limit } => {
                write!(f, "array rank budget exceeded: {actual} > {limit}")
            }
            Self::ElementBudgetExceeded { actual, limit } => {
                write!(
                    f,
                    "array element budget exceeded: {actual} > {limit}"
                )
            }
            Self::SparseShapeNotFullyKnown => {
                f.write_str(
                    "sparse coordinates require fully known dimensions",
                )
            }
            Self::SparseCoordinateRankMismatch { expected, actual } => {
                write!(
                    f,
                    "sparse coordinate rank mismatch: expected {expected}, got {actual}"
                )
            }
            Self::SparseIndexOutOfBounds { axis, index, size } => {
                write!(
                    f,
                    "sparse index {index} is out of bounds on axis {axis} of size {size}"
                )
            }
            Self::EmptySymbolicInitializer => {
                f.write_str("symbolic array initializer is empty")
            }
            Self::SymbolicExpressionBudgetExceeded { actual, limit } => {
                write!(
                    f,
                    "symbolic expression byte budget exceeded: {actual} > {limit}"
                )
            }
            Self::InvalidElement(error) => {
                write!(f, "invalid array element: {error}")
            }
        }
    }
}

impl Error for ArrayValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidElement(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::schema::types::QualifiedName;
    use crate::zuir::values::constants::ArbitraryInteger;

    fn integer_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::Integer")
                .expect("test type name must be valid"),
        )
    }

    fn shape(dimensions: &[u128]) -> ArrayShape {
        ArrayShape::new(
            dimensions
                .iter()
                .copied()
                .map(Dimension::Known)
                .collect(),
        )
    }

    fn integer(value: &str) -> ConstantValue {
        ConstantValue::Integer(
            ArbitraryInteger::new(value).expect("test integer must be valid"),
        )
    }

    #[test]
    fn validates_dense_row_major_array() {
        let array = ArrayValue::dense(
            integer_type(),
            shape(&[2, 2]),
            vec![integer("1"), integer("2"), integer("3"), integer("4")],
        );

        assert!(array.validate().is_ok());
        assert_eq!(array.known_element_count().unwrap(), Some(4));
    }

    #[test]
    fn rejects_dense_element_count_mismatch() {
        let array = ArrayValue::dense(
            integer_type(),
            shape(&[2, 2]),
            vec![integer("1")],
        );

        assert!(matches!(
            array.validate(),
            Err(ArrayValidationError::DenseElementCountMismatch {
                expected: 4,
                actual: 1
            })
        ));
    }

    #[test]
    fn supports_symbolic_shapes_without_materialization() {
        let array = ArrayValue::symbolic(
            integer_type(),
            ArrayShape::new(vec![
                Dimension::Dynamic,
                Dimension::Known(4),
            ]),
            "compute_values()",
        );

        assert!(array.validate().is_ok());
        assert_eq!(array.known_element_count().unwrap(), None);
    }

    #[test]
    fn detects_shape_product_overflow() {
        assert!(matches!(
            shape(&[u128::MAX, 2]).known_element_count(),
            Err(ArrayValidationError::ShapeProductOverflow)
        ));
    }

    #[test]
    fn rejects_sparse_indices_out_of_bounds() {
        let mut elements = BTreeMap::new();
        elements.insert(vec![2, 0], integer("7"));

        let array = ArrayValue::sparse(
            integer_type(),
            shape(&[2, 3]),
            elements,
        );

        assert!(matches!(
            array.validate(),
            Err(ArrayValidationError::SparseIndexOutOfBounds {
                axis: 0,
                index: 2,
                size: 2
            })
        ));
    }

    #[test]
    fn enforces_explicit_rank_budget() {
        let array = ArrayValue::dense(
            integer_type(),
            shape(&[2]),
            vec![integer("1"), integer("2")],
        );

        let budget = ArrayValidationBudget::new(
            Some(0),
            Some(10),
            None,
            None,
            None,
        );

        assert!(matches!(
            array.validate_with(&budget),
            Err(ArrayValidationError::RankBudgetExceeded {
                actual: 1,
                limit: 0
            })
        ));
    }

    #[test]
    fn accepts_scalar_shape_with_one_element() {
        let array = ArrayValue::dense(
            integer_type(),
            ArrayShape::new(Vec::new()),
            vec![integer("42")],
        );

        assert!(array.validate().is_ok());
        assert_eq!(array.known_element_count().unwrap(), Some(1));
    }
}
