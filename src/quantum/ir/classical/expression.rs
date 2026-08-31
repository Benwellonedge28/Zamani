//! Zamani Quantum IR — Classical Expression Model
//!
//! Canonical, hardware-independent representation of classical expressions
//! used by the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module owns the semantic representation and manipulation of classical
//! expressions.
//!
//! It describes:
//!
//! - constants;
//! - logical classical-bit reads;
//! - unary operations;
//! - binary operations;
//! - static expression typing;
//! - expression validation;
//! - expression evaluation;
//! - referenced classical-resource discovery;
//! - deterministic expression rendering;
//! - expression structural metrics.
//!
//! It does NOT own:
//!
//! - quantum qubits;
//! - quantum gates;
//! - measurement execution;
//! - hardware registers;
//! - CPU registers;
//! - memory addresses;
//! - routing;
//! - scheduling;
//! - backend execution;
//! - device calibration;
//! - frontend parsing.
//!
//! # Critical dependency rule
//!
//! Classical expressions operate on classical resources only.
//!
//! Therefore this module MUST NOT depend on:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! A quantum-to-classical relationship is represented elsewhere, for example
//! by measurement semantics. Once a measurement has produced a classical
//! resource, this module operates only on the resulting classical resource.
//!
//! The dependency boundary is therefore:
//!
//! ```text
//! qubit.rs
//!     │
//!     ▼
//! measurement.rs
//!     │
//!     ▼
//! classical.rs
//!     │
//!     ▼
//! classical/expression.rs
//! ```
//!
//! This module never points back toward `qubit.rs`.
//!
//! # Scalability
//!
//! Expression size is determined by the program and available resources.
//!
//! There is no architectural expression-size constant and no quantum-machine
//! size assumption in this module.
//!
//! In particular, this module contains no:
//!
//! - `MAX_QUBITS`;
//! - `MAX_CLASSICAL_BITS`;
//! - fixed register width;
//! - fixed expression depth;
//! - fixed number of operands;
//! - fixed machine size.
//!
//! Explicit resource limits, when required, are supplied by the caller.
//!
//! # Iterative execution
//!
//! Expression trees can be arbitrarily deep within available resources.
//!
//! Recursive traversal is therefore intentionally avoided for:
//!
//! - node counting;
//! - depth calculation;
//! - classical-bit collection;
//! - validation;
//! - evaluation;
//! - rendering.
//!
//! Explicit stacks are used instead of the Rust call stack.
//!
//! This prevents a very deeply nested valid expression from causing stack
//! exhaustion merely because its semantic depth is large.
//!
//! # Safety
//!
//! This module uses only safe Rust.
//!
//! `unsafe` code is forbidden at the crate/module level.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! Parent `classical.rs` owns:
//!
//! - `ClassicalBitId`;
//! - `ClassicalBitSet`;
//! - `ClassicalValue`;
//! - `ClassicalType`;
//! - `ClassicalError`.
//!
//! This module owns:
//!
//! - `ClassicalBinaryOp`;
//! - `ClassicalUnaryOp`;
//! - `ClassicalExpression`;
//! - expression-specific algorithms.
//!
//! `classical.rs` should expose this module with:
//!
//! ```text
//! pub mod expression;
//! ```
//!
//! and re-export the public expression types as appropriate.
//!
//! The existing expression implementation in `classical.rs` should then be
//! removed so that there is exactly one canonical definition of each
//! expression type.
//!
//! No downstream module should define a second `ClassicalExpression`.
//!
//! # Determinism
//!
//! Expression traversal order is deterministic.
//!
//! Classical-bit discovery uses the canonical `ClassicalBitSet`, whose
//! deterministic ordering is provided by its underlying ordered set.
//!
//! Expression rendering is canonical and does not depend on:
//!
//! - hash-map ordering;
//! - memory addresses;
//! - process state;
//! - hardware;
//! - thread scheduling.
//!
//! # Canonical expression model
//!
//! ```text
//! ClassicalExpression
//! ├── Value
//! ├── Bit
//! ├── Unary
//! └── Binary
//! ```
//!
//! The model is deliberately small. New classical language features should
//! be introduced through explicit IR evolution rather than by silently
//! changing the meaning of existing nodes.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::fmt::Write as FmtWrite;

use super::{
    ClassicalBitId,
    ClassicalBitSet,
    ClassicalError,
    ClassicalType,
    ClassicalValue,
};

// =============================================================================
// Unary operators
// =============================================================================

/// Unary operator for a classical expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalUnaryOp {
    /// Boolean logical negation.
    Not,

    /// Signed integer arithmetic negation.
    Negate,

    /// Integer bitwise complement.
    BitNot,
}

impl ClassicalUnaryOp {
    /// Returns the canonical source-level spelling.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Not => "!",
            Self::Negate => "-",
            Self::BitNot => "~",
        }
    }

    /// Returns whether the operand must be boolean.
    #[must_use]
    pub const fn requires_bool(self) -> bool {
        matches!(self, Self::Not)
    }

    /// Returns whether the operand must be an integer.
    #[must_use]
    pub const fn requires_integer(self) -> bool {
        matches!(self, Self::Negate | Self::BitNot)
    }

    /// Returns the operator precedence.
    ///
    /// Larger values bind more strongly.
    #[must_use]
    pub const fn precedence(self) -> u8 {
        7
    }
}

impl fmt::Display for ClassicalUnaryOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

// =============================================================================
// Binary operators
// =============================================================================

/// Binary operator for a classical expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalBinaryOp {
    /// Integer addition.
    Add,

    /// Integer subtraction.
    Subtract,

    /// Integer multiplication.
    Multiply,

    /// Integer division.
    Divide,

    /// Integer remainder.
    Remainder,

    /// Equality comparison.
    Equal,

    /// Inequality comparison.
    NotEqual,

    /// Less-than comparison.
    LessThan,

    /// Less-than-or-equal comparison.
    LessThanOrEqual,

    /// Greater-than comparison.
    GreaterThan,

    /// Greater-than-or-equal comparison.
    GreaterThanOrEqual,

    /// Boolean conjunction.
    And,

    /// Boolean disjunction.
    Or,

    /// Integer bitwise AND.
    BitAnd,

    /// Integer bitwise OR.
    BitOr,

    /// Integer bitwise XOR.
    BitXor,
}

impl ClassicalBinaryOp {
    /// Returns the canonical source-level spelling.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Remainder => "%",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
        }
    }

    /// Returns the operator precedence.
    ///
    /// Larger values bind more strongly.
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::BitOr => 3,
            Self::BitXor => 4,
            Self::BitAnd => 5,

            Self::Equal | Self::NotEqual => 6,

            Self::LessThan
            | Self::LessThanOrEqual
            | Self::GreaterThan
            | Self::GreaterThanOrEqual => 6,

            Self::Add | Self::Subtract => 7,

            Self::Multiply
            | Self::Divide
            | Self::Remainder => 8,
        }
    }

    /// Returns whether this operator is a comparison.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::LessThan
                | Self::LessThanOrEqual
                | Self::GreaterThan
                | Self::GreaterThanOrEqual
        )
    }

    /// Returns whether this operator is boolean-only.
    #[must_use]
    pub const fn is_boolean_operator(self) -> bool {
        matches!(self, Self::And | Self::Or)
    }

    /// Returns whether this operator is integer-only.
    #[must_use]
    pub const fn is_integer_operator(self) -> bool {
        matches!(
            self,
            Self::Add
                | Self::Subtract
                | Self::Multiply
                | Self::Divide
                | Self::Remainder
                | Self::BitAnd
                | Self::BitOr
                | Self::BitXor
        )
    }
}

impl fmt::Display for ClassicalBinaryOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.symbol())
    }
}

// =============================================================================
// Classical expression
// =============================================================================

/// Canonical classical expression.
///
/// Expressions are semantic descriptions. They do not execute themselves.
///
/// A classical expression can contain:
///
/// - a constant value;
/// - a classical-bit read;
/// - a unary operation;
/// - a binary operation.
///
/// The tree is finite because each recursive edge is owned through `Box`.
///
/// Algorithms over the tree are implemented iteratively so that expression
/// depth does not consume proportional Rust call-stack space.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalExpression {
    /// Constant classical value.
    Value(ClassicalValue),

    /// Read one logical classical bit.
    Bit(ClassicalBitId),

    /// Unary expression.
    Unary {
        /// Unary operator.
        op: ClassicalUnaryOp,

        /// Operand.
        operand: Box<ClassicalExpression>,
    },

    /// Binary expression.
    Binary {
        /// Binary operator.
        op: ClassicalBinaryOp,

        /// Left operand.
        left: Box<ClassicalExpression>,

        /// Right operand.
        right: Box<ClassicalExpression>,
    },
}

impl ClassicalExpression {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a constant expression.
    #[must_use]
    pub const fn value(value: ClassicalValue) -> Self {
        Self::Value(value)
    }

    /// Creates a boolean constant.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Value(ClassicalValue::Bool(value))
    }

    /// Creates a signed integer constant.
    #[must_use]
    pub const fn int(value: i128) -> Self {
        Self::Value(ClassicalValue::Int(value))
    }

    /// Creates an unsigned integer constant.
    #[must_use]
    pub const fn uint(value: u128) -> Self {
        Self::Value(ClassicalValue::UInt(value))
    }

    /// Creates a classical-bit reference.
    #[must_use]
    pub const fn bit(bit: ClassicalBitId) -> Self {
        Self::Bit(bit)
    }

    /// Creates a unary expression.
    #[must_use]
    pub fn unary(
        op: ClassicalUnaryOp,
        operand: Self,
    ) -> Self {
        Self::Unary {
            op,
            operand: Box::new(operand),
        }
    }

    /// Creates a binary expression.
    #[must_use]
    pub fn binary(
        op: ClassicalBinaryOp,
        left: Self,
        right: Self,
    ) -> Self {
        Self::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Creates `left + right`.
    #[must_use]
    pub fn add(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Add, left, right)
    }

    /// Creates `left - right`.
    #[must_use]
    pub fn subtract(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Subtract, left, right)
    }

    /// Creates `left * right`.
    #[must_use]
    pub fn multiply(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Multiply, left, right)
    }

    /// Creates `left / right`.
    #[must_use]
    pub fn divide(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Divide, left, right)
    }

    /// Creates `left % right`.
    #[must_use]
    pub fn remainder(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Remainder, left, right)
    }

    /// Creates `left == right`.
    #[must_use]
    pub fn equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Equal, left, right)
    }

    /// Creates `left != right`.
    #[must_use]
    pub fn not_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::NotEqual, left, right)
    }

    /// Creates `left < right`.
    #[must_use]
    pub fn less_than(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::LessThan, left, right)
    }

    /// Creates `left <= right`.
    #[must_use]
    pub fn less_than_or_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            ClassicalBinaryOp::LessThanOrEqual,
            left,
            right,
        )
    }

    /// Creates `left > right`.
    #[must_use]
    pub fn greater_than(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            ClassicalBinaryOp::GreaterThan,
            left,
            right,
        )
    }

    /// Creates `left >= right`.
    #[must_use]
    pub fn greater_than_or_equal(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(
            ClassicalBinaryOp::GreaterThanOrEqual,
            left,
            right,
        )
    }

    /// Creates `left && right`.
    #[must_use]
    pub fn and(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::And, left, right)
    }

    /// Creates `left || right`.
    #[must_use]
    pub fn or(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::Or, left, right)
    }

    /// Creates `left & right`.
    #[must_use]
    pub fn bit_and(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::BitAnd, left, right)
    }

    /// Creates `left | right`.
    #[must_use]
    pub fn bit_or(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::BitOr, left, right)
    }

    /// Creates `left ^ right`.
    #[must_use]
    pub fn bit_xor(
        left: Self,
        right: Self,
    ) -> Self {
        Self::binary(ClassicalBinaryOp::BitXor, left, right)
    }

    /// Creates `!operand`.
    #[must_use]
    pub fn not(operand: Self) -> Self {
        Self::unary(ClassicalUnaryOp::Not, operand)
    }

    /// Creates `-operand`.
    #[must_use]
    pub fn negate(operand: Self) -> Self {
        Self::unary(ClassicalUnaryOp::Negate, operand)
    }

    /// Creates `~operand`.
    #[must_use]
    pub fn bit_not(operand: Self) -> Self {
        Self::unary(ClassicalUnaryOp::BitNot, operand)
    }

    // =========================================================================
    // Structural queries
    // =========================================================================

    /// Returns whether this expression directly or indirectly references a
    /// classical bit.
    #[must_use]
    pub fn references_bits(&self) -> bool {
        let mut stack = Vec::new();
        stack.push(self);

        while let Some(expression) = stack.pop() {
            match expression {
                Self::Value(_) => {}

                Self::Bit(_) => return true,

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        false
    }

    /// Returns the number of nodes in this expression.
    ///
    /// Traversal is iterative.
    ///
    /// If the mathematical count cannot be represented by `usize`, the result
    /// saturates at `usize::MAX`.
    #[must_use]
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = Vec::new();

        stack.push(self);

        while let Some(expression) = stack.pop() {
            count = match count.checked_add(1) {
                Some(value) => value,
                None => return usize::MAX,
            };

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        count
    }

    /// Returns the maximum expression depth.
    ///
    /// A leaf has depth zero.
    ///
    /// Traversal is iterative.
    #[must_use]
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = Vec::new();

        stack.push((self, 0usize));

        while let Some((expression, depth)) = stack.pop() {
            if depth > maximum {
                maximum = depth;
            }

            let next_depth = depth.saturating_add(1);

            match expression {
                Self::Value(_) | Self::Bit(_) => {}

                Self::Unary { operand, .. } => {
                    stack.push((operand, next_depth));
                }

                Self::Binary { left, right, .. } => {
                    stack.push((left, next_depth));
                    stack.push((right, next_depth));
                }
            }
        }

        maximum
    }

    /// Collects all referenced classical bits.
    ///
    /// The resulting collection is deterministic.
    ///
    /// Traversal is iterative.
    #[must_use]
    pub fn collect_bits(&self) -> ClassicalBitSet {
        let mut result = ClassicalBitSet::new();
        let mut stack = Vec::new();

        stack.push(self);

        while let Some(expression) = stack.pop() {
            match expression {
                Self::Value(_) => {}

                Self::Bit(bit) => {
                    result.insert(*bit);
                }

                Self::Unary { operand, .. } => {
                    stack.push(operand);
                }

                Self::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        result
    }

    /// Returns the static type of the expression.
    ///
    /// Validation is iterative and therefore does not use recursive Rust
    /// stack frames.
    pub fn validate(&self) -> Result<ClassicalType, ClassicalError> {
        self.validate_with_depth_limit(usize::MAX)
    }

    /// Validates the expression using an explicit caller-provided maximum
    /// expression depth.
    ///
    /// The supplied limit is an execution/compiler policy, not an architectural
    /// limit on Zamani.
    pub fn validate_with_depth_limit(
        &self,
        maximum_depth: usize,
    ) -> Result<ClassicalType, ClassicalError> {
        let mut stack = Vec::new();
        let mut types = Vec::new();

        stack.push(ValidationFrame::Visit {
            expression: self,
            depth: 0,
        });

        while let Some(frame) = stack.pop() {
            match frame {
                ValidationFrame::Visit {
                    expression,
                    depth,
                } => {
                    if depth > maximum_depth {
                        return Err(
                            ClassicalError::ExpressionDepthExceeded {
                                depth,
                                maximum: maximum_depth,
                            },
                        );
                    }

                    match expression {
                        Self::Value(value) => {
                            types.push(value.classical_type());
                        }

                        Self::Bit(_) => {
                            types.push(ClassicalType::Bool);
                        }

                        Self::Unary { operand, .. } => {
                            stack.push(ValidationFrame::ReduceUnary {
                                expression,
                            });

                            stack.push(ValidationFrame::Visit {
                                expression: operand,
                                depth: depth.saturating_add(1),
                            });
                        }

                        Self::Binary { left, right, .. } => {
                            stack.push(ValidationFrame::ReduceBinary {
                                expression,
                            });

                            stack.push(ValidationFrame::Visit {
                                expression: right,
                                depth: depth.saturating_add(1),
                            });

                            stack.push(ValidationFrame::Visit {
                                expression: left,
                                depth: depth.saturating_add(1),
                            });
                        }
                    }
                }

                ValidationFrame::ReduceUnary { expression } => {
                    let operand_type = types
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let operation = match expression {
                        Self::Unary { op, .. } => *op,
                        _ => return Err(ClassicalError::InvalidExpression),
                    };

                    types.push(validate_unary(operation, operand_type)?);
                }

                ValidationFrame::ReduceBinary { expression } => {
                    let right = types
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let left = types
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let operation = match expression {
                        Self::Binary { op, .. } => *op,
                        _ => return Err(ClassicalError::InvalidExpression),
                    };

                    types.push(validate_binary(
                        operation,
                        left,
                        right,
                    )?);
                }
            }
        }

        types
            .pop()
            .ok_or(ClassicalError::InvalidExpression)
    }

    // =========================================================================
    // Evaluation
    // =========================================================================

    /// Evaluates the expression using a caller-supplied classical-bit resolver.
    ///
    /// The resolver is read-only from the expression engine's perspective.
    ///
    /// Evaluation is iterative.
    ///
    /// Boolean `&&` and `||` use short-circuit semantics.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> Result<ClassicalValue, ClassicalError>
    where
        F: Fn(ClassicalBitId) -> Option<ClassicalValue>,
    {
        let mut stack = Vec::new();
        let mut values = Vec::new();

        stack.push(EvaluationFrame::Visit(self));

        while let Some(frame) = stack.pop() {
            match frame {
                EvaluationFrame::Visit(expression) => {
                    match expression {
                        Self::Value(value) => {
                            values.push(*value);
                        }

                        Self::Bit(bit) => {
                            let value = resolver(*bit).ok_or(
                                ClassicalError::UnboundClassicalBit {
                                    bit: *bit,
                                },
                            )?;

                            values.push(value);
                        }

                        Self::Unary { operand, .. } => {
                            stack.push(
                                EvaluationFrame::ReduceUnary {
                                    expression,
                                },
                            );

                            stack.push(
                                EvaluationFrame::Visit(operand),
                            );
                        }

                        Self::Binary { op, left, right } => {
                            match op {
                                ClassicalBinaryOp::And => {
                                    stack.push(
                                        EvaluationFrame::ShortCircuitAnd {
                                            right,
                                        },
                                    );

                                    stack.push(
                                        EvaluationFrame::Visit(left),
                                    );
                                }

                                ClassicalBinaryOp::Or => {
                                    stack.push(
                                        EvaluationFrame::ShortCircuitOr {
                                            right,
                                        },
                                    );

                                    stack.push(
                                        EvaluationFrame::Visit(left),
                                    );
                                }

                                _ => {
                                    stack.push(
                                        EvaluationFrame::ReduceBinary {
                                            expression,
                                        },
                                    );

                                    stack.push(
                                        EvaluationFrame::Visit(right),
                                    );

                                    stack.push(
                                        EvaluationFrame::Visit(left),
                                    );
                                }
                            }
                        }
                    }
                }

                EvaluationFrame::ReduceUnary { expression } => {
                    let operand = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let operation = match expression {
                        Self::Unary { op, .. } => *op,
                        _ => return Err(ClassicalError::InvalidExpression),
                    };

                    values.push(evaluate_unary(operation, operand)?);
                }

                EvaluationFrame::ReduceBinary { expression } => {
                    let right = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let left = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    let operation = match expression {
                        Self::Binary { op, .. } => *op,
                        _ => return Err(ClassicalError::InvalidExpression),
                    };

                    values.push(evaluate_binary(
                        operation,
                        left,
                        right,
                    )?);
                }

                EvaluationFrame::ShortCircuitAnd { right } => {
                    let left = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    match left {
                        ClassicalValue::Bool(false) => {
                            values.push(ClassicalValue::Bool(false));
                        }

                        ClassicalValue::Bool(true) => {
                            stack.push(
                                EvaluationFrame::FinishBooleanAnd,
                            );

                            stack.push(EvaluationFrame::Visit(right));
                        }

                        other => {
                            return Err(ClassicalError::TypeMismatch {
                                expected: "bool",
                                found: other.type_name(),
                            });
                        }
                    }
                }

                EvaluationFrame::FinishBooleanAnd => {
                    let right = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    match right {
                        ClassicalValue::Bool(value) => {
                            values.push(ClassicalValue::Bool(value));
                        }

                        other => {
                            return Err(ClassicalError::TypeMismatch {
                                expected: "bool",
                                found: other.type_name(),
                            });
                        }
                    }
                }

                EvaluationFrame::ShortCircuitOr { right } => {
                    let left = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    match left {
                        ClassicalValue::Bool(true) => {
                            values.push(ClassicalValue::Bool(true));
                        }

                        ClassicalValue::Bool(false) => {
                            stack.push(
                                EvaluationFrame::FinishBooleanOr,
                            );

                            stack.push(EvaluationFrame::Visit(right));
                        }

                        other => {
                            return Err(ClassicalError::TypeMismatch {
                                expected: "bool",
                                found: other.type_name(),
                            });
                        }
                    }
                }

                EvaluationFrame::FinishBooleanOr => {
                    let right = values
                        .pop()
                        .ok_or(ClassicalError::InvalidExpression)?;

                    match right {
                        ClassicalValue::Bool(value) => {
                            values.push(ClassicalValue::Bool(value));
                        }

                        other => {
                            return Err(ClassicalError::TypeMismatch {
                                expected: "bool",
                                found: other.type_name(),
                            });
                        }
                    }
                }
            }
        }

        values
            .pop()
            .ok_or(ClassicalError::InvalidExpression)
    }

    // =========================================================================
    // Canonical rendering
    // =========================================================================

    /// Returns a deterministic canonical textual representation.
    ///
    /// Rendering is iterative.
    ///
    /// A private `String` writer and explicit rendering stack are used so that
    /// deeply nested expressions never recursively consume the Rust call
    /// stack.
    #[must_use]
    pub fn to_canonical_string(&self) -> String {
        let mut writer = StringWriter::new();
        let mut stack = Vec::new();

        stack.push(RenderFrame::Expression {
            expression: self,
            parent_precedence: 0,
            position: RenderPosition::Root,
        });

        while let Some(frame) = stack.pop() {
            match frame {
                RenderFrame::Expression {
                    expression,
                    parent_precedence,
                    position,
                } => {
                    let precedence = expression.precedence();

                    let needs_parentheses =
                        precedence < parent_precedence;

                    if needs_parentheses {
                        writer.push_char('(');
                    }

                    match expression {
                        Self::Value(value) => {
                            writer.push_value(*value);

                            if needs_parentheses {
                                writer.push_char(')');
                            }
                        }

                        Self::Bit(bit) => {
                            writer.push_bit(*bit);

                            if needs_parentheses {
                                writer.push_char(')');
                            }
                        }

                        Self::Unary { op, operand } => {
                            stack.push(RenderFrame::CloseParentheses {
                                enabled: needs_parentheses,
                            });

                            stack.push(RenderFrame::Expression {
                                expression: operand,
                                parent_precedence: op.precedence(),
                                position: RenderPosition::UnaryOperand,
                            });

                            stack.push(RenderFrame::Literal(op.symbol()));
                        }

                        Self::Binary { op, left, right } => {
                            stack.push(RenderFrame::CloseParentheses {
                                enabled: needs_parentheses,
                            });

                            stack.push(RenderFrame::Expression {
                                expression: right,
                                parent_precedence: binary_right_precedence(
                                    *op,
                                ),
                                position: RenderPosition::RightOperand,
                            });

                            stack.push(RenderFrame::Literal(op.symbol()));

                            stack.push(RenderFrame::Expression {
                                expression: left,
                                parent_precedence: op.precedence(),
                                position: RenderPosition::LeftOperand,
                            });
                        }
                    }

                    let _ = position;
                }

                RenderFrame::Literal(text) => {
                    writer.push_str(text);
                }

                RenderFrame::CloseParentheses { enabled } => {
                    if enabled {
                        writer.push_char(')');
                    }
                }
            }
        }

        writer.finish()
    }

    /// Returns the static precedence of this expression.
    #[must_use]
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Value(_) | Self::Bit(_) => u8::MAX,

            Self::Unary { op, .. } => op.precedence(),

            Self::Binary { op, .. } => op.precedence(),
        }
    }
}

impl fmt::Display for ClassicalExpression {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let rendered = self.to_canonical_string();
        formatter.write_str(&rendered)
    }
}

// =============================================================================
// Predicate support
// =============================================================================

/// A boolean classical predicate suitable for dynamic quantum control.
///
/// A predicate is an expression whose static type is `bool`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassicalPredicate {
    expression: ClassicalExpression,
}

impl ClassicalPredicate {
    /// Creates a predicate after validating that the expression is boolean.
    pub fn new(
        expression: ClassicalExpression,
    ) -> Result<Self, ClassicalError> {
        let expression_type = expression.validate()?;

        if expression_type != ClassicalType::Bool {
            return Err(ClassicalError::PredicateMustBeBoolean {
                found: expression_type,
            });
        }

        Ok(Self { expression })
    }

    /// Creates a predicate with an explicit depth policy.
    pub fn with_depth_limit(
        expression: ClassicalExpression,
        maximum_depth: usize,
    ) -> Result<Self, ClassicalError> {
        let expression_type =
            expression.validate_with_depth_limit(maximum_depth)?;

        if expression_type != ClassicalType::Bool {
            return Err(ClassicalError::PredicateMustBeBoolean {
                found: expression_type,
            });
        }

        Ok(Self { expression })
    }

    /// Returns the underlying expression.
    #[must_use]
    pub fn expression(&self) -> &ClassicalExpression {
        &self.expression
    }

    /// Returns the referenced classical bits.
    #[must_use]
    pub fn referenced_bits(&self) -> ClassicalBitSet {
        self.expression.collect_bits()
    }

    /// Returns the number of expression nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.expression.node_count()
    }

    /// Returns the expression depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.expression.depth()
    }

    /// Evaluates the predicate.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> Result<bool, ClassicalError>
    where
        F: Fn(ClassicalBitId) -> Option<ClassicalValue>,
    {
        match self.expression.evaluate(resolver)? {
            ClassicalValue::Bool(value) => Ok(value),

            other => Err(ClassicalError::PredicateMustBeBoolean {
                found: other.classical_type(),
            }),
        }
    }

    /// Creates `bit == value`.
    pub fn equals(
        bit: ClassicalBitId,
        value: ClassicalValue,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::equal(
            ClassicalExpression::bit(bit),
            ClassicalExpression::value(value),
        ))
    }

    /// Creates `bit != value`.
    pub fn not_equals(
        bit: ClassicalBitId,
        value: ClassicalValue,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::not_equal(
            ClassicalExpression::bit(bit),
            ClassicalExpression::value(value),
        ))
    }

    /// Creates a predicate from one classical bit.
    pub fn bit(
        bit: ClassicalBitId,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::bit(bit))
    }

    /// Creates logical conjunction.
    pub fn and(
        left: Self,
        right: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::and(
            left.expression,
            right.expression,
        ))
    }

    /// Creates logical disjunction.
    pub fn or(
        left: Self,
        right: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::or(
            left.expression,
            right.expression,
        ))
    }

    /// Creates logical negation.
    pub fn not(
        predicate: Self,
    ) -> Result<Self, ClassicalError> {
        Self::new(ClassicalExpression::not(
            predicate.expression,
        ))
    }
}

// =============================================================================
// Iterative traversal frames
// =============================================================================

enum ValidationFrame<'a> {
    Visit {
        expression: &'a ClassicalExpression,
        depth: usize,
    },

    ReduceUnary {
        expression: &'a ClassicalExpression,
    },

    ReduceBinary {
        expression: &'a ClassicalExpression,
    },
}

enum EvaluationFrame<'a> {
    Visit(&'a ClassicalExpression),

    ReduceUnary {
        expression: &'a ClassicalExpression,
    },

    ReduceBinary {
        expression: &'a ClassicalExpression,
    },

    ShortCircuitAnd {
        right: &'a ClassicalExpression,
    },

    FinishBooleanAnd,

    ShortCircuitOr {
        right: &'a ClassicalExpression,
    },

    FinishBooleanOr,
}

#[derive(Clone, Copy)]
enum RenderPosition {
    Root,
    UnaryOperand,
    LeftOperand,
    RightOperand,
}

enum RenderFrame<'a> {
    Expression {
        expression: &'a ClassicalExpression,
        parent_precedence: u8,
        position: RenderPosition,
    },

    Literal(&'static str),

    CloseParentheses {
        enabled: bool,
    },
}

// =============================================================================
// Internal deterministic String writer
// =============================================================================

/// Small internal string writer used by canonical expression rendering.
///
/// This wrapper intentionally owns only a `String`.
///
/// It does not use:
///
/// - raw pointers;
//! - unsafe code;
//! - global state;
//! - leaked allocations;
//! - recursive formatting.
struct StringWriter {
    output: String,
}

impl StringWriter {
    /// Creates an empty writer.
    #[must_use]
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    /// Appends a string slice.
    fn push_str(
        &mut self,
        value: &str,
    ) {
        self.output.push_str(value);
    }

    /// Appends one character.
    fn push_char(
        &mut self,
        value: char,
    ) {
        self.output.push(value);
    }

    /// Appends a classical bit.
    fn push_bit(
        &mut self,
        bit: ClassicalBitId,
    ) {
        let _ = write!(self.output, "{bit}");
    }

    /// Appends a classical value.
    fn push_value(
        &mut self,
        value: ClassicalValue,
    ) {
        match value {
            ClassicalValue::Bool(value) => {
                if value {
                    self.output.push_str("true");
                } else {
                    self.output.push_str("false");
                }
            }

            ClassicalValue::Int(value) => {
                let _ = write!(self.output, "{value}");
            }

            ClassicalValue::UInt(value) => {
                let _ = write!(self.output, "{value}");
            }
        }
    }

    /// Consumes the writer and returns its finished string.
    #[must_use]
    fn finish(self) -> String {
        self.output
    }
}

// =============================================================================
// Operator validation
// =============================================================================

fn validate_unary(
    operation: ClassicalUnaryOp,
    operand: ClassicalType,
) -> Result<ClassicalType, ClassicalError> {
    if operation.requires_bool() {
        if operand != ClassicalType::Bool {
            return Err(ClassicalError::TypeMismatch {
                expected: "bool",
                found: type_name(operand),
            });
        }

        return Ok(ClassicalType::Bool);
    }

    if operation.requires_integer() {
        return match operand {
            ClassicalType::Int | ClassicalType::UInt => Ok(operand),

            ClassicalType::Bool => Err(ClassicalError::TypeMismatch {
                expected: "integer",
                found: "bool",
            }),
        };
    }

    Err(ClassicalError::InvalidExpression)
}

fn validate_binary(
    operation: ClassicalBinaryOp,
    left: ClassicalType,
    right: ClassicalType,
) -> Result<ClassicalType, ClassicalError> {
    match operation {
        ClassicalBinaryOp::And | ClassicalBinaryOp::Or => {
            if left == ClassicalType::Bool
                && right == ClassicalType::Bool
            {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::Equal | ClassicalBinaryOp::NotEqual => {
            if left == right {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::LessThan
        | ClassicalBinaryOp::LessThanOrEqual
        | ClassicalBinaryOp::GreaterThan
        | ClassicalBinaryOp::GreaterThanOrEqual => {
            if left == right
                && matches!(
                    left,
                    ClassicalType::Int | ClassicalType::UInt
                )
            {
                Ok(ClassicalType::Bool)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }

        ClassicalBinaryOp::Add
        | ClassicalBinaryOp::Subtract
        | ClassicalBinaryOp::Multiply
        | ClassicalBinaryOp::Divide
        | ClassicalBinaryOp::Remainder
        | ClassicalBinaryOp::BitAnd
        | ClassicalBinaryOp::BitOr
        | ClassicalBinaryOp::BitXor => {
            if left == right
                && matches!(
                    left,
                    ClassicalType::Int | ClassicalType::UInt
                )
            {
                Ok(left)
            } else {
                Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left,
                    right,
                })
            }
        }
    }
}

// =============================================================================
// Operator evaluation
// =============================================================================

fn evaluate_unary(
    operation: ClassicalUnaryOp,
    operand: ClassicalValue,
) -> Result<ClassicalValue, ClassicalError> {
    match operation {
        ClassicalUnaryOp::Not => match operand {
            ClassicalValue::Bool(value) => {
                Ok(ClassicalValue::Bool(!value))
            }

            other => Err(ClassicalError::TypeMismatch {
                expected: "bool",
                found: other.type_name(),
            }),
        },

        ClassicalUnaryOp::Negate => match operand {
            ClassicalValue::Int(value) => value
                .checked_neg()
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            ClassicalValue::UInt(_) => {
                Err(ClassicalError::TypeMismatch {
                    expected: "int",
                    found: "uint",
                })
            }

            ClassicalValue::Bool(_) => {
                Err(ClassicalError::TypeMismatch {
                    expected: "integer",
                    found: "bool",
                })
            }
        },

        ClassicalUnaryOp::BitNot => match operand {
            ClassicalValue::Int(value) => {
                Ok(ClassicalValue::Int(!value))
            }

            ClassicalValue::UInt(value) => {
                Ok(ClassicalValue::UInt(!value))
            }

            ClassicalValue::Bool(_) => {
                Err(ClassicalError::TypeMismatch {
                    expected: "integer",
                    found: "bool",
                })
            }
        },
    }
}

fn evaluate_binary(
    operation: ClassicalBinaryOp,
    left: ClassicalValue,
    right: ClassicalValue,
) -> Result<ClassicalValue, ClassicalError> {
    match operation {
        ClassicalBinaryOp::And | ClassicalBinaryOp::Or => {
            match (left, right) {
                (
                    ClassicalValue::Bool(a),
                    ClassicalValue::Bool(b),
                ) => {
                    let result = match operation {
                        ClassicalBinaryOp::And => a && b,
                        ClassicalBinaryOp::Or => a || b,
                        _ => unreachable_operator(),
                    };

                    Ok(ClassicalValue::Bool(result))
                }

                (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                    operation,
                    left: a.classical_type(),
                    right: b.classical_type(),
                }),
            }
        }

        ClassicalBinaryOp::Equal => {
            Ok(ClassicalValue::Bool(left == right))
        }

        ClassicalBinaryOp::NotEqual => {
            Ok(ClassicalValue::Bool(left != right))
        }

        ClassicalBinaryOp::LessThan => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a < b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a < b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::LessThanOrEqual => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a <= b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a <= b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::GreaterThan => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Bool(a > b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::Bool(a > b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::GreaterThanOrEqual => {
            match (left, right) {
                (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                    Ok(ClassicalValue::Bool(a >= b))
                }

                (
                    ClassicalValue::UInt(a),
                    ClassicalValue::UInt(b),
                ) => Ok(ClassicalValue::Bool(a >= b)),

                (a, b) => {
                    Err(ClassicalError::BinaryTypeMismatch {
                        operation,
                        left: a.classical_type(),
                        right: b.classical_type(),
                    })
                }
            }
        }

        ClassicalBinaryOp::Add => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_add(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_add(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedOverflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Subtract => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_sub(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedUnderflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_sub(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedUnderflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Multiply => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => a
                .checked_mul(b)
                .map(ClassicalValue::Int)
                .ok_or(ClassicalError::SignedOverflow),

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => a
                .checked_mul(b)
                .map(ClassicalValue::UInt)
                .ok_or(ClassicalError::UnsignedOverflow),

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Divide => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                if b == 0 {
                    return Err(ClassicalError::DivisionByZero);
                }

                a.checked_div(b)
                    .map(ClassicalValue::Int)
                    .ok_or(ClassicalError::SignedOverflow)
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                if b == 0 {
                    return Err(ClassicalError::DivisionByZero);
                }

                a.checked_div(b)
                    .map(ClassicalValue::UInt)
                    .ok_or(ClassicalError::UnsignedOverflow)
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::Remainder => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                if b == 0 {
                    return Err(ClassicalError::RemainderByZero);
                }

                a.checked_rem(b)
                    .map(ClassicalValue::Int)
                    .ok_or(ClassicalError::SignedOverflow)
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                if b == 0 {
                    return Err(ClassicalError::RemainderByZero);
                }

                a.checked_rem(b)
                    .map(ClassicalValue::UInt)
                    .ok_or(ClassicalError::UnsignedOverflow)
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitAnd => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a & b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a & b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitOr => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a | b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a | b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },

        ClassicalBinaryOp::BitXor => match (left, right) {
            (ClassicalValue::Int(a), ClassicalValue::Int(b)) => {
                Ok(ClassicalValue::Int(a ^ b))
            }

            (ClassicalValue::UInt(a), ClassicalValue::UInt(b)) => {
                Ok(ClassicalValue::UInt(a ^ b))
            }

            (a, b) => Err(ClassicalError::BinaryTypeMismatch {
                operation,
                left: a.classical_type(),
                right: b.classical_type(),
            }),
        },
    }
}

// =============================================================================
// Formatting helpers
// =============================================================================

fn binary_right_precedence(
    operation: ClassicalBinaryOp,
) -> u8 {
    match operation {
        ClassicalBinaryOp::Subtract
        | ClassicalBinaryOp::Divide
        | ClassicalBinaryOp::Remainder
        | ClassicalBinaryOp::LessThan
        | ClassicalBinaryOp::LessThanOrEqual
        | ClassicalBinaryOp::GreaterThan
        | ClassicalBinaryOp::GreaterThanOrEqual
        | ClassicalBinaryOp::Equal
        | ClassicalBinaryOp::NotEqual => {
            operation.precedence().saturating_add(1)
        }

        _ => operation.precedence(),
    }
}

fn type_name(
    value: ClassicalType,
) -> &'static str {
    match value {
        ClassicalType::Bool => "bool",
        ClassicalType::Int => "int",
        ClassicalType::UInt => "uint",
    }
}

#[inline]
fn unreachable_operator() -> bool {
    false
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn bit(index: usize) -> ClassicalExpression {
        ClassicalExpression::bit(ClassicalBitId::new(index))
    }

    #[test]
    fn constants_have_expected_types() {
        assert_eq!(
            ClassicalExpression::bool(true)
                .validate()
                .expect("boolean must validate"),
            ClassicalType::Bool
        );

        assert_eq!(
            ClassicalExpression::int(-10)
                .validate()
                .expect("integer must validate"),
            ClassicalType::Int
        );

        assert_eq!(
            ClassicalExpression::uint(10)
                .validate()
                .expect("unsigned integer must validate"),
            ClassicalType::UInt
        );
    }

    #[test]
    fn classical_bits_are_boolean() {
        assert_eq!(
            bit(0)
                .validate()
                .expect("classical bit must be boolean"),
            ClassicalType::Bool
        );
    }

    #[test]
    fn arithmetic_type_is_preserved() {
        let expression = ClassicalExpression::add(
            ClassicalExpression::int(2),
            ClassicalExpression::int(3),
        );

        assert_eq!(
            expression
                .validate()
                .expect("addition must validate"),
            ClassicalType::Int
        );
    }

    #[test]
    fn mismatched_arithmetic_is_rejected() {
        let expression = ClassicalExpression::add(
            ClassicalExpression::int(2),
            ClassicalExpression::uint(3),
        );

        assert!(expression.validate().is_err());
    }

    #[test]
    fn comparisons_return_bool() {
        let expression = ClassicalExpression::less_than(
            ClassicalExpression::int(2),
            ClassicalExpression::int(3),
        );

        assert_eq!(
            expression
                .validate()
                .expect("comparison must validate"),
            ClassicalType::Bool
        );
    }

    #[test]
    fn logical_operations_require_bool() {
        let expression = ClassicalExpression::and(
            ClassicalExpression::bool(true),
            ClassicalExpression::bool(false),
        );

        assert_eq!(
            expression
                .validate()
                .expect("logical expression must validate"),
            ClassicalType::Bool
        );
    }

    #[test]
    fn logical_operations_reject_integer_operands() {
        let expression = ClassicalExpression::and(
            ClassicalExpression::int(1),
            ClassicalExpression::int(0),
        );

        assert!(expression.validate().is_err());
    }

    #[test]
    fn evaluation_is_checked() {
        let expression = ClassicalExpression::add(
            ClassicalExpression::int(20),
            ClassicalExpression::int(22),
        );

        let result = expression
            .evaluate(&|_| None)
            .expect("evaluation must succeed");

        assert_eq!(result, ClassicalValue::Int(42));
    }

    #[test]
    fn classical_bit_resolution_works() {
        let expression = ClassicalExpression::add(
            bit(0),
            ClassicalExpression::int(10),
        );

        let result = expression
            .evaluate(&|id| {
                if id == ClassicalBitId::new(0) {
                    Some(ClassicalValue::Int(32))
                } else {
                    None
                }
            })
            .expect("bit must resolve");

        assert_eq!(result, ClassicalValue::Int(42));
    }

    #[test]
    fn missing_classical_bit_is_an_error() {
        let expression = bit(99);

        assert!(matches!(
            expression.evaluate(&|_| None),
            Err(ClassicalError::UnboundClassicalBit {
                bit
            }) if bit == ClassicalBitId::new(99)
        ));
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let expression = ClassicalExpression::divide(
            ClassicalExpression::int(10),
            ClassicalExpression::int(0),
        );

        assert!(matches!(
            expression.evaluate(&|_| None),
            Err(ClassicalError::DivisionByZero)
        ));
    }

    #[test]
    fn integer_overflow_is_rejected() {
        let expression = ClassicalExpression::add(
            ClassicalExpression::int(i128::MAX),
            ClassicalExpression::int(1),
        );

        assert!(matches!(
            expression.evaluate(&|_| None),
            Err(ClassicalError::SignedOverflow)
        ));
    }

    #[test]
    fn references_bits_is_iterative() {
        let expression = ClassicalExpression::add(
            ClassicalExpression::int(1),
            bit(7),
        );

        assert!(expression.references_bits());
    }

    #[test]
    fn collect_bits_is_deterministic() {
        let expression = ClassicalExpression::add(
            bit(9),
            ClassicalExpression::add(
                bit(2),
                bit(9),
            ),
        );

        let bits = expression.collect_bits();

        assert!(bits.contains(&ClassicalBitId::new(2)));
        assert!(bits.contains(&ClassicalBitId::new(9)));
        assert_eq!(bits.len(), 2);
    }

    #[test]
    fn node_count_is_correct() {
        let expression = ClassicalExpression::add(
            bit(0),
            ClassicalExpression::multiply(
                bit(1),
                ClassicalExpression::int(2),
            ),
        );

        assert_eq!(expression.node_count(), 5);
    }

    #[test]
    fn depth_is_correct() {
        let expression = ClassicalExpression::add(
            bit(0),
            ClassicalExpression::multiply(
                bit(1),
                ClassicalExpression::int(2),
            ),
        );

        assert_eq!(expression.depth(), 2);
    }

    #[test]
    fn depth_limit_is_enforced() {
        let expression = ClassicalExpression::not(
            ClassicalExpression::not(
                ClassicalExpression::not(
                    ClassicalExpression::bool(true),
                ),
            ),
        );

        assert!(matches!(
            expression.validate_with_depth_limit(1),
            Err(ClassicalError::ExpressionDepthExceeded {
                depth: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn predicate_requires_boolean() {
        let result =
            ClassicalPredicate::new(ClassicalExpression::int(1));

        assert!(matches!(
            result,
            Err(ClassicalError::PredicateMustBeBoolean {
                found: ClassicalType::Int
            })
        ));
    }

    #[test]
    fn predicate_evaluates() {
        let predicate = ClassicalPredicate::equals(
            ClassicalBitId::new(0),
            ClassicalValue::UInt(1),
        )
        .expect("predicate must be valid");

        let result = predicate
            .evaluate(&|id| {
                if id == ClassicalBitId::new(0) {
                    Some(ClassicalValue::UInt(1))
                } else {
                    None
                }
            })
            .expect("predicate must evaluate");

        assert!(result);
    }

    #[test]
    fn short_circuit_and_does_not_read_unused_bit() {
        let expression = ClassicalExpression::and(
            ClassicalExpression::bool(false),
            bit(999_999),
        );

        let result = expression
            .evaluate(&|_| None)
            .expect("right side must not be evaluated");

        assert_eq!(result, ClassicalValue::Bool(false));
    }

    #[test]
    fn short_circuit_or_does_not_read_unused_bit() {
        let expression = ClassicalExpression::or(
            ClassicalExpression::bool(true),
            bit(999_999),
        );

        let result = expression
            .evaluate(&|_| None)
            .expect("right side must not be evaluated");

        assert_eq!(result, ClassicalValue::Bool(true));
    }

    #[test]
    fn canonical_rendering_is_deterministic() {
        let expression = ClassicalExpression::and(
            ClassicalExpression::equal(
                bit(0),
                ClassicalExpression::uint(1),
            ),
            ClassicalExpression::not(bit(1)),
        );

        assert_eq!(
            expression.to_canonical_string(),
            "c0 == 1 && !c1"
        );
    }

    #[test]
    fn rendering_preserves_precedence() {
        let expression = ClassicalExpression::multiply(
            ClassicalExpression::add(
                ClassicalExpression::int(1),
                ClassicalExpression::int(2),
            ),
            ClassicalExpression::int(3),
        );

        assert_eq!(
            expression.to_canonical_string(),
            "(1 + 2) * 3"
        );
    }

    #[test]
    fn rendering_is_iterative_for_deep_expression() {
        let mut expression = ClassicalExpression::bool(true);

        for _ in 0..10_000 {
            expression =
                ClassicalExpression::not(expression);
        }

        assert_eq!(expression.node_count(), 10_001);

        let rendered = expression.to_canonical_string();

        assert!(!rendered.is_empty());
    }
}