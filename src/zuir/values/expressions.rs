
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR expression model
//!
//! Canonical path: `src/zuir/values/expressions.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Represent typed, target-independent expressions in the Zamani Universal
//! Intermediate Representation (ZUIR).
//!
//! OWNS
//!
//! - The expression data model and its extensible operation references.
//! - References to resolved symbols and canonical IR values.
//! - Expression structure and local representation validation.
//! - Explicit validation budgets and deterministic validation errors.
//!
//! DOES_NOT_OWN
//!
//! - Source syntax, parsing, AST construction, or grammar definitions.
//! - Name lookup, scope traversal, overload resolution, or name resolution.
//! - Semantic type inference, type compatibility, or type constraint solving.
//! - Constant evaluation, arithmetic, simplification, or interpretation.
//! - Operation registration or extension-specific operation semantics.
//! - SSA definition/use validation, dominance, or control-flow verification.
//! - Target-specific lowering, scheduling, hardware allocation, or execution.
//! - Source diagnostics rendering or complete-program serialization.
//!
//! AUTHORITY
//!
//! This module must conform to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/values/constants.rs`
//! - `src/zuir/values/symbols.rs`
//! - `grammar/specification/semantics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! INTEGRATION
//!
//! - `values/mod.rs` declares this module.
//! - `types/type_def.rs` owns `TypeRef`.
//! - `values/constants.rs` owns `ConstantValue`.
//! - `values/symbols.rs` and `ids.rs` own canonical symbol/value identities.
//! - Semantic analysis resolves names and establishes expression types.
//! - The ZUIR verifier checks type compatibility and reference validity.
//! - A future evaluator owns expression execution and evaluation budgets.
//! - Registered dialects define the semantics of extensible operations.
//! - Serialization validates decoded expressions before they are trusted.
//!
//! SCALABILITY
//!
//! The model imposes no universal maximum expression count, operand count,
//! tensor rank, integer width, quantum resource count, or hardware capacity.
//! Collections are dynamically sized. Optional validation budgets are
//! operational safeguards, not language-level semantic restrictions.
//!
//! Validation uses an explicit work stack to avoid recursive traversal of
//! expression children. Allocation, address space, execution time, and the
//! finite identity and indexing spaces of the implementation remain real
//! resource constraints.
//!
//! SEMANTIC BOUNDARY
//!
//! Local validation establishes only that the expression's representation
//! is structurally acceptable. It does not prove that referenced symbols or
//! values exist, that types are compatible, or that an operation is legal.
//! Those checks belong to semantic analysis and the canonical verifier.
//!
//! Operators use qualified names rather than a closed universal operator
//! enum. Future domains can introduce operations through registered,
//! versioned dialect contracts without adding variants to this file.
//!
//! FREEZE CRITERIA
//!
//! - Uses canonical IDs, type references, constants, and qualified names.
//! - Does not implement a second resolver, type system, or evaluator.
//! - Supports extensible operations and dynamically sized operands.
//! - Validates structure iteratively with explicit optional budgets.
//! - Reports invalid representation and budget exhaustion explicitly.
//! - Contains no target-specific assumptions or unsafe Rust.
//! - Compiles with the repository's actual module tree and passes tests.

use crate::zuir::ids::{SymbolId, ValueId};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::TypeRef;
use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValue,
};

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

// =============================================================================
// Expression
// =============================================================================

/// A typed expression in the canonical ZUIR value model.
///
/// `ty` is the semantic type established by the frontend or semantic layer.
/// This structure does not resolve or prove that type reference.
///
/// Constructed or deserialized expressions must be validated locally and
/// subsequently checked by the canonical semantic verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// Semantic type of the expression.
    pub ty: TypeRef,

    /// Expression structure.
    pub kind: ExpressionKind,
}

impl Expression {
    /// Construct an expression without asserting semantic validity.
    ///
    /// Call [`Expression::validate`] for local structural validation and
    /// the canonical verifier for cross-reference and type checking.
    #[must_use]
    pub fn new(ty: TypeRef, kind: ExpressionKind) -> Self {
        Self { ty, kind }
    }

    /// Construct an expression containing a represented constant value.
    #[must_use]
    pub fn constant(ty: TypeRef, value: ConstantValue) -> Self {
        Self::new(ty, ExpressionKind::Constant(value))
    }

    /// Construct a reference to a previously resolved symbol.
    #[must_use]
    pub fn symbol(ty: TypeRef, symbol: SymbolId) -> Self {
        Self::new(ty, ExpressionKind::SymbolReference(symbol))
    }

    /// Construct a reference to a canonical IR value.
    #[must_use]
    pub fn value(ty: TypeRef, value: ValueId) -> Self {
        Self::new(ty, ExpressionKind::ValueReference(value))
    }

    /// Validate local representation using an unlimited explicit budget.
    ///
    /// "Unlimited" means that this method adds no configured traversal
    /// ceilings. It does not promise infinite memory or execution time.
    pub fn validate(&self) -> Result<(), ExpressionValidationError> {
        self.validate_with(&ExpressionValidationBudget::unlimited())
    }

    /// Validate expression structure under an explicit operational budget.
    ///
    /// Traversal of expression children is iterative. A nested constant is
    /// validated using a constant-local budget derived from this policy.
    ///
    /// Type resolution, type compatibility, symbol existence, value
    /// dominance, and dialect operation legality remain verifier duties.
    pub fn validate_with(
        &self,
        budget: &ExpressionValidationBudget,
    ) -> Result<(), ExpressionValidationError> {
        let mut pending: Vec<(&Expression, usize)> = vec![(self, 1)];
        let mut visited_nodes: usize = 0;
        let mut visited_text_bytes: usize = 0;

        while let Some((expression, depth)) = pending.pop() {
            visited_nodes = visited_nodes
                .checked_add(1)
                .ok_or(ExpressionValidationError::CounterOverflow)?;

            if let Some(limit) = budget.max_nodes {
                if visited_nodes > limit {
                    return Err(ExpressionValidationError::NodeBudgetExceeded);
                }
            }

            if let Some(limit) = budget.max_depth {
                if depth > limit {
                    return Err(ExpressionValidationError::DepthBudgetExceeded);
                }
            }

            match &expression.kind {
                ExpressionKind::Constant(value) => {
                    let constant_budget = ConstantValidationBudget::new(
                        budget.max_constant_nodes,
                        budget.max_constant_text_bytes,
                    );

                    value
                        .validate_with(&constant_budget)
                        .map_err(ExpressionValidationError::InvalidConstant)?;
                }

                ExpressionKind::SymbolReference(_) => {}

                ExpressionKind::ValueReference(_) => {}

                ExpressionKind::Unary {
                    operator,
                    operand,
                } => {
                    operator
                        .validate()
                        .map_err(|_| ExpressionValidationError::InvalidQualifiedName)?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        operator.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;

                    push_child(&mut pending, operand, depth, budget)?;
                }

                ExpressionKind::Binary {
                    operator,
                    left,
                    right,
                } => {
                    operator
                        .validate()
                        .map_err(|_| ExpressionValidationError::InvalidQualifiedName)?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        operator.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;

                    push_child(&mut pending, right, depth, budget)?;
                    push_child(&mut pending, left, depth, budget)?;
                }

                ExpressionKind::Call { callee, arguments } => {
                    push_child(&mut pending, callee, depth, budget)?;

                    for argument in arguments.iter().rev() {
                        push_child(&mut pending, argument, depth, budget)?;
                    }
                }

                ExpressionKind::Aggregate { elements } => {
                    for element in elements.iter().rev() {
                        push_child(&mut pending, element, depth, budget)?;
                    }
                }

                ExpressionKind::FieldAccess { aggregate, field } => {
                    if field.is_empty() {
                        return Err(ExpressionValidationError::EmptyFieldName);
                    }

                    add_text_bytes(
                        &mut visited_text_bytes,
                        field.len(),
                        budget.max_text_bytes,
                    )?;

                    push_child(&mut pending, aggregate, depth, budget)?;
                }

                ExpressionKind::Index { aggregate, index } => {
                    push_child(&mut pending, index, depth, budget)?;
                    push_child(&mut pending, aggregate, depth, budget)?;
                }

                ExpressionKind::Conditional {
                    condition,
                    then_value,
                    else_value,
                } => {
                    push_child(&mut pending, else_value, depth, budget)?;
                    push_child(&mut pending, then_value, depth, budget)?;
                    push_child(&mut pending, condition, depth, budget)?;
                }

                ExpressionKind::Cast {
                    value,
                    target_type: _,
                    conversion,
                } => {
                    if let Some(conversion) = conversion {
                        conversion
                            .validate()
                            .map_err(|_| {
                                ExpressionValidationError::InvalidQualifiedName
                            })?;

                        add_text_bytes(
                            &mut visited_text_bytes,
                            conversion.canonical_string().len(),
                            budget.max_text_bytes,
                        )?;
                    }

                    push_child(&mut pending, value, depth, budget)?;
                }

                ExpressionKind::Extension {
                    dialect,
                    operation,
                    operands,
                } => {
                    dialect
                        .validate()
                        .map_err(|_| ExpressionValidationError::InvalidQualifiedName)?;

                    operation
                        .validate()
                        .map_err(|_| ExpressionValidationError::InvalidQualifiedName)?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        dialect.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;

                    add_text_bytes(
                        &mut visited_text_bytes,
                        operation.canonical_string().len(),
                        budget.max_text_bytes,
                    )?;

                    for operand in operands.iter().rev() {
                        push_child(&mut pending, operand, depth, budget)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Borrow this expression's semantic type reference.
    #[must_use]
    pub fn ty(&self) -> &TypeRef {
        &self.ty
    }

    /// Borrow this expression's kind.
    #[must_use]
    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }

    /// Consume the expression and return its kind.
    #[must_use]
    pub fn into_kind(self) -> ExpressionKind {
        self.kind
    }
}

// =============================================================================
// Expression kinds
// =============================================================================

/// Structural forms supported by the canonical expression model.
///
/// This enum describes common expression shapes, not every possible
/// computational operation. Operation identity and meaning are extensible
/// through qualified names and registered dialect contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExpressionKind {
    /// A portable, represented constant.
    Constant(ConstantValue),

    /// A reference to a symbol resolved by semantic analysis.
    SymbolReference(SymbolId),

    /// A reference to a canonical IR value.
    ///
    /// The verifier must check definition existence and any SSA dominance
    /// requirements selected by the ZUIR specification.
    ValueReference(ValueId),

    /// A unary operation identified by a registered qualified name.
    Unary {
        operator: QualifiedName,
        operand: Box<Expression>,
    },

    /// A binary operation identified by a registered qualified name.
    Binary {
        operator: QualifiedName,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A call through an expression that denotes a callable value.
    ///
    /// The verifier determines callability, argument compatibility, effects,
    /// result types, and applicable calling contracts.
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    /// An ordered aggregate expression, such as a tuple or vector literal.
    ///
    /// The declared semantic type determines the aggregate's interpretation.
    Aggregate {
        elements: Vec<Expression>,
    },

    /// Access to a named field of an aggregate expression.
    FieldAccess {
        aggregate: Box<Expression>,
        field: String,
    },

    /// Indexing into an aggregate, tensor, sequence, or dialect-defined value.
    ///
    /// Index legality and bounds are established by semantic analysis or
    /// the relevant dialect verifier, not by structural validation.
    Index {
        aggregate: Box<Expression>,
        index: Box<Expression>,
    },

    /// Select a value based on a condition.
    ///
    /// The semantic layer determines condition validity and the compatible
    /// result type of the two alternatives.
    Conditional {
        condition: Box<Expression>,
        then_value: Box<Expression>,
        else_value: Box<Expression>,
    },

    /// An explicit conversion with a target semantic type.
    ///
    /// `conversion` optionally identifies a registered conversion contract.
    /// `None` does not imply that every conversion is permitted.
    Cast {
        value: Box<Expression>,
        target_type: TypeRef,
        conversion: Option<QualifiedName>,
    },

    /// An operation defined by a registered extension or dialect.
    ///
    /// The universal core preserves its qualified identity and operands.
    /// The registered dialect owns the operation's semantics and validation.
    Extension {
        dialect: QualifiedName,
        operation: QualifiedName,
        operands: Vec<Expression>,
    },
}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit resource budget for expression representation validation.
///
/// Every `None` means no configured ceiling for that dimension. A budget
/// is a compilation-safety policy, not a source-language or hardware limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExpressionValidationBudget {
    /// Maximum expression nodes visited, including the root.
    pub max_nodes: Option<usize>,

    /// Maximum nesting depth; the root has depth one.
    pub max_depth: Option<usize>,

    /// Maximum UTF-8 bytes of text counted by expression validation.
    ///
    /// This includes qualified operation names and field names.
    pub max_text_bytes: Option<usize>,

    /// Per-constant maximum number of nested constant-value nodes.
    pub max_constant_nodes: Option<usize>,

    /// Per-constant maximum number of text bytes.
    pub max_constant_text_bytes: Option<usize>,
}

impl ExpressionValidationBudget {
    /// Create a policy with no configured limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_depth: None,
            max_text_bytes: None,
            max_constant_nodes: None,
            max_constant_text_bytes: None,
        }
    }

    /// Create a policy with explicit expression-node and depth limits.
    #[must_use]
    pub const fn new(
        max_nodes: Option<usize>,
        max_depth: Option<usize>,
    ) -> Self {
        Self {
            max_nodes,
            max_depth,
            max_text_bytes: None,
            max_constant_nodes: None,
            max_constant_text_bytes: None,
        }
    }

    /// Set the maximum expression text bytes.
    #[must_use]
    pub const fn with_text_budget(mut self, max_text_bytes: Option<usize>) -> Self {
        self.max_text_bytes = max_text_bytes;
        self
    }

    /// Set the per-constant node budget.
    #[must_use]
    pub const fn with_constant_node_budget(
        mut self,
        max_constant_nodes: Option<usize>,
    ) -> Self {
        self.max_constant_nodes = max_constant_nodes;
        self
    }

    /// Set the per-constant text-byte budget.
    #[must_use]
    pub const fn with_constant_text_budget(
        mut self,
        max_constant_text_bytes: Option<usize>,
    ) -> Self {
        self.max_constant_text_bytes = max_constant_text_bytes;
        self
    }
}

// =============================================================================
// Validation errors
// =============================================================================

/// Failure during local expression representation validation.
///
/// These errors do not replace semantic diagnostics from the type checker,
/// symbol resolver, verifier, or dialect-specific validators.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpressionValidationError {
    /// A qualified name failed its canonical structural validation.
    InvalidQualifiedName,

    /// A field-access expression has an empty field name.
    EmptyFieldName,

    /// The node counter overflowed its representational range.
    CounterOverflow,

    /// The configured expression-node budget was exceeded.
    NodeBudgetExceeded,

    /// The configured nesting-depth budget was exceeded.
    DepthBudgetExceeded,

    /// The configured text-byte budget was exceeded.
    TextBudgetExceeded,

    /// The constant value has an invalid local representation.
    InvalidConstant(
        crate::zuir::values::constants::ConstantValidationError,
    ),
}

impl fmt::Display for ExpressionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQualifiedName => {
                f.write_str("expression contains an invalid qualified name")
            }
            Self::EmptyFieldName => {
                f.write_str("field-access expression has an empty field name")
            }
            Self::CounterOverflow => {
                f.write_str("expression validation counter overflowed")
            }
            Self::NodeBudgetExceeded => {
                f.write_str("expression validation node budget exceeded")
            }
            Self::DepthBudgetExceeded => {
                f.write_str("expression validation depth budget exceeded")
            }
            Self::TextBudgetExceeded => {
                f.write_str("expression validation text-byte budget exceeded")
            }
            Self::InvalidConstant(error) => {
                write!(f, "invalid constant expression value: {error}")
            }
        }
    }
}

impl Error for ExpressionValidationError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn push_child<'a>(
    pending: &mut Vec<(&'a Expression, usize)>,
    child: &'a Expression,
    parent_depth: usize,
    budget: &ExpressionValidationBudget,
) -> Result<(), ExpressionValidationError> {
    let child_depth = parent_depth
        .checked_add(1)
        .ok_or(ExpressionValidationError::CounterOverflow)?;

    if let Some(limit) = budget.max_depth {
        if child_depth > limit {
            return Err(ExpressionValidationError::DepthBudgetExceeded);
        }
    }

    pending.push((child, child_depth));
    Ok(())
}

fn add_text_bytes(
    total: &mut usize,
    bytes: usize,
    limit: Option<usize>,
) -> Result<(), ExpressionValidationError> {
    *total = total
        .checked_add(bytes)
        .ok_or(ExpressionValidationError::CounterOverflow)?;

    if let Some(limit) = limit {
        if *total > limit {
            return Err(ExpressionValidationError::TextBudgetExceeded);
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::values::constants::ConstantValue;

    fn integer_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::Integer")
                .expect("valid qualified name"),
        )
    }

    fn operation(name: &str) -> QualifiedName {
        QualifiedName::parse(name).expect("valid operation name")
    }

    #[test]
    fn validates_a_constant_expression() {
        let expression =
            Expression::constant(integer_type(), ConstantValue::Boolean(true));

        assert!(expression.validate().is_ok());
    }

    #[test]
    fn validates_a_binary_expression() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::Binary {
                operator: operation("core::add"),
                left: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )),
                right: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(false),
                )),
            },
        );

        // Structural validation deliberately does not assert that Boolean
        // operands are semantically compatible with the declared integer type.
        assert!(expression.validate().is_ok());
    }

    #[test]
    fn rejects_an_empty_field_name() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::FieldAccess {
                aggregate: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )),
                field: String::new(),
            },
        );

        assert_eq!(
            expression.validate(),
            Err(ExpressionValidationError::EmptyFieldName)
        );
    }

    #[test]
    fn enforces_node_budget() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::Binary {
                operator: operation("core::add"),
                left: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )),
                right: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(false),
                )),
            },
        );

        let budget = ExpressionValidationBudget::new(Some(2), None);

        assert_eq!(
            expression.validate_with(&budget),
            Err(ExpressionValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn enforces_depth_budget() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::Unary {
                operator: operation("core::negate"),
                operand: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )),
            },
        );

        let budget = ExpressionValidationBudget::new(None, Some(1));

        assert_eq!(
            expression.validate_with(&budget),
            Err(ExpressionValidationError::DepthBudgetExceeded)
        );
    }

    #[test]
    fn accepts_an_extension_defined_operation() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::Extension {
                dialect: operation("future_domain"),
                operation: operation("future_domain::transform"),
                operands: vec![Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )],
            },
        );

        // Registration and operation legality are verified elsewhere.
        assert!(expression.validate().is_ok());
    }

    #[test]
    fn references_use_canonical_typed_ids() {
        let symbol_id = SymbolId::new(1).expect("nonzero symbol ID");
        let value_id = ValueId::new(2).expect("nonzero value ID");

        let symbol_expression = Expression::symbol(integer_type(), symbol_id);
        let value_expression = Expression::value(integer_type(), value_id);

        assert!(symbol_expression.validate().is_ok());
        assert!(value_expression.validate().is_ok());
    }

    #[test]
    fn enforces_text_budget() {
        let expression = Expression::new(
            integer_type(),
            ExpressionKind::Unary {
                operator: operation("core::negate"),
                operand: Box::new(Expression::constant(
                    integer_type(),
                    ConstantValue::Boolean(true),
                )),
            },
        );

        let budget =
            ExpressionValidationBudget::unlimited().with_text_budget(Some(1));

        assert_eq!(
            expression.validate_with(&budget),
            Err(ExpressionValidationError::TextBudgetExceeded)
        );
    }
}
