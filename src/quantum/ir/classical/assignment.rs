//! Classical assignment operations for the Zamani Quantum IR.
//!
//! This module represents assignment of a classical expression/value to a
//! classical destination. It intentionally contains no execution logic,
//! backend assumptions, fixed-size limits, or hardware-specific behavior.
//!
//! # Architectural contract
//!
//! This module owns:
//! - [`Assignment`]
//! - [`AssignmentId`]
//! - [`AssignmentTarget`]
//!
//! This module does not own:
//! - classical expression evaluation
//! - register allocation
//! - hardware execution
//! - scheduling
//! - routing
//! - quantum operations
//! - backend-specific classical types
//!
//! # Scalability
//!
//! No fixed number of bits, registers, assignments, or expression sizes are
//! imposed here. Collection capacities are determined by the caller and by
//! the applicable [`crate::quantum::ir::core::limits`] policy.
//!
//! Semantic identifiers are never represented by `usize`. `usize` may only
//! be used by an implementation as a collection index outside this module.
//!
//! # Integration
//!
//! This module is consumed by:
//! - `classical::value`
//! - `classical::expression`
//! - `program::operation`
//! - `program::operand`
//! - `validation`
//! - `analysis`
//! - `serialization`
//! - `hashing`
//!
//! The assignment itself remains target-independent.

use core::fmt;

use super::expression::Expression;
use super::value::ClassicalValue;
use crate::quantum::ir::core::identity::ValueId;
use crate::quantum::ir::core::types::Type;

/// Stable identifier for a classical assignment.
///
/// This identifier is semantic and therefore must not depend on a vector
/// position, allocator address, thread, process, or machine word size.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssignmentId(pub u64);

impl AssignmentId {
    /// Creates an assignment identifier from its stable numeric value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable numeric value.
    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for AssignmentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "assignment{}", self.0)
    }
}

/// Destination of a classical assignment.
///
/// A destination is represented by the canonical IR [`ValueId`] rather than
/// by a container index. This permits arbitrarily large programs and allows
/// the program layer to decide how values are stored or grouped.
///
/// A future register/slice abstraction can be represented through additional
/// IR-level value definitions without changing the assignment contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssignmentTarget {
    value: ValueId,
}

impl AssignmentTarget {
    /// Creates an assignment target from a canonical IR value identifier.
    #[inline]
    pub const fn new(value: ValueId) -> Self {
        Self { value }
    }

    /// Returns the canonical value identifier represented by this target.
    #[inline]
    pub const fn value_id(self) -> ValueId {
        self.value
    }
}

/// A classical assignment in the canonical Zamani IR.
///
/// An assignment has:
///
/// `target = expression`
///
/// The expression remains symbolic until an appropriate compiler stage or
/// execution environment evaluates it. This is important for parameterized
/// quantum programs, dynamic circuits, runtime feedback, and hardware
/// independent compilation.
///
/// `value_type` records the semantic destination type. It is deliberately
/// kept alongside the assignment so validation and serialization do not need
/// to infer the destination type from an implementation-specific runtime
/// representation.
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    id: AssignmentId,
    target: AssignmentTarget,
    value_type: Type,
    expression: Expression,
}

impl Assignment {
    /// Creates a symbolic/classical assignment.
    ///
    /// No expression evaluation is performed.
    #[inline]
    pub const fn new(
        id: AssignmentId,
        target: AssignmentTarget,
        value_type: Type,
        expression: Expression,
    ) -> Self {
        Self {
            id,
            target,
            value_type,
            expression,
        }
    }

    /// Creates an assignment from an already materialized classical value.
    ///
    /// This constructor intentionally delegates conversion to
    /// [`Expression::value`] so the expression representation remains the
    /// single source of truth.
    #[inline]
    pub fn from_value(
        id: AssignmentId,
        target: AssignmentTarget,
        value_type: Type,
        value: ClassicalValue,
    ) -> Self {
        Self::new(id, target, value_type, Expression::value(value))
    }

    /// Returns the stable assignment identifier.
    #[inline]
    pub const fn id(&self) -> AssignmentId {
        self.id
    }

    /// Returns the assignment destination.
    #[inline]
    pub const fn target(&self) -> AssignmentTarget {
        self.target
    }

    /// Returns the destination's canonical value identifier.
    #[inline]
    pub const fn target_value_id(&self) -> ValueId {
        self.target.value_id()
    }

    /// Returns the semantic destination type.
    #[inline]
    pub const fn value_type(&self) -> &Type {
        &self.value_type
    }

    /// Returns the assigned expression.
    #[inline]
    pub const fn expression(&self) -> &Expression {
        &self.expression
    }

    /// Returns a mutable reference to the assigned expression.
    ///
    /// Mutation is intentionally explicit. Callers are responsible for
    /// re-running validation after modifying an assignment.
    #[inline]
    pub fn expression_mut(&mut self) -> &mut Expression {
        &mut self.expression
    }

    /// Returns whether the assignment is a compile-time literal assignment.
    ///
    /// This is a structural query only; it does not evaluate expressions.
    #[inline]
    pub fn is_constant(&self) -> bool {
        self.expression.is_constant()
    }

    /// Replaces the assigned expression.
    ///
    /// The assignment identifier, target, and destination type remain stable.
    #[inline]
    pub fn with_expression(mut self, expression: Expression) -> Self {
        self.expression = expression;
        self
    }

    /// Replaces the destination type.
    ///
    /// This does not perform type checking. Validation belongs to the
    /// canonical validation subsystem.
    #[inline]
    pub fn with_value_type(mut self, value_type: Type) -> Self {
        self.value_type = value_type;
        self
    }

    /// Replaces the assignment target.
    ///
    /// This is useful for compiler transformations such as SSA conversion,
    /// value remapping, or lowering. It does not update any external use-def
    /// chains; those belong to the program/analysis layers.
    #[inline]
    pub fn with_target(mut self, target: AssignmentTarget) -> Self {
        self.target = target;
        self
    }

    /// Returns the semantic components that participate in equality and
    /// canonical hashing.
    ///
    /// Non-semantic compiler metadata must not be added here.
    #[inline]
    pub fn semantic_parts(
        &self,
    ) -> (
        AssignmentId,
        AssignmentTarget,
        &Type,
        &Expression,
    ) {
        (
            self.id,
            self.target,
            &self.value_type,
            &self.expression,
        )
    }
}

/// Errors that can be detected locally while constructing or validating an
/// assignment.
///
/// Cross-operation, symbol-table, region, and program-wide validation belongs
/// to `crate::quantum::ir::validation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssignmentError {
    /// The assignment identifier is reserved/invalid.
    InvalidId(AssignmentId),

    /// The destination value identifier is invalid.
    InvalidTarget(ValueId),

    /// The destination type is not a valid assignment type.
    InvalidType,

    /// The assignment expression is structurally invalid.
    InvalidExpression,
}

impl fmt::Display for AssignmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(id) => {
                write!(formatter, "invalid classical assignment identifier: {id}")
            }
            Self::InvalidTarget(value) => {
                write!(formatter, "invalid classical assignment target: {value:?}")
            }
            Self::InvalidType => {
                write!(formatter, "invalid classical assignment destination type")
            }
            Self::InvalidExpression => {
                write!(formatter, "invalid classical assignment expression")
            }
        }
    }
}

impl std::error::Error for AssignmentError {}

/// Validates the local structural invariants of an assignment.
///
/// This function deliberately does not inspect the surrounding program.
/// Program-wide validation must be performed by the canonical validation
/// subsystem once symbols, values, regions, and types are available.
///
/// Keeping local validation here makes this file independently complete while
/// preventing it from acquiring dependencies on higher-level IR structures.
pub fn validate_assignment(assignment: &Assignment) -> Result<(), AssignmentError> {
    if assignment.id.get() == 0 {
        return Err(AssignmentError::InvalidId(assignment.id));
    }

    if assignment.target.value_id().get() == 0 {
        return Err(AssignmentError::InvalidTarget(
            assignment.target.value_id(),
        ));
    }

    if !assignment.expression.is_valid() {
        return Err(AssignmentError::InvalidExpression);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_id_is_stable() {
        let id = AssignmentId::new(42);

        assert_eq!(id.get(), 42);
        assert_eq!(id.to_string(), "assignment42");
    }

    #[test]
    fn target_preserves_value_id() {
        let value = ValueId::new(7);
        let target = AssignmentTarget::new(value);

        assert_eq!(target.value_id(), value);
    }

    #[test]
    fn assignment_preserves_semantic_components() {
        let id = AssignmentId::new(1);
        let value_id = ValueId::new(2);
        let target = AssignmentTarget::new(value_id);

        let expression = Expression::value(ClassicalValue::Boolean(true));

        let assignment = Assignment::new(
            id,
            target,
            Type::Boolean,
            expression.clone(),
        );

        assert_eq!(assignment.id(), id);
        assert_eq!(assignment.target(), target);
        assert_eq!(assignment.target_value_id(), value_id);
        assert_eq!(assignment.expression(), &expression);
        assert_eq!(assignment.value_type(), &Type::Boolean);
    }

    #[test]
    fn assignment_can_be_rewritten_without_changing_identity() {
        let id = AssignmentId::new(1);
        let target = AssignmentTarget::new(ValueId::new(2));

        let first = Expression::value(ClassicalValue::Boolean(false));

        let assignment = Assignment::new(
            id,
            target,
            Type::Boolean,
            first,
        );

        let second = Expression::value(ClassicalValue::Boolean(true));

        let rewritten = assignment.with_expression(second.clone());

        assert_eq!(rewritten.id(), id);
        assert_eq!(rewritten.target(), target);
        assert_eq!(rewritten.expression(), &second);
    }

    #[test]
    fn invalid_zero_assignment_id_is_rejected() {
        let assignment = Assignment::new(
            AssignmentId::new(0),
            AssignmentTarget::new(ValueId::new(1)),
            Type::Boolean,
            Expression::value(ClassicalValue::Boolean(true)),
        );

        assert_eq!(
            validate_assignment(&assignment),
            Err(AssignmentError::InvalidId(AssignmentId::new(0)))
        );
    }

    #[test]
    fn invalid_zero_target_is_rejected() {
        let assignment = Assignment::new(
            AssignmentId::new(1),
            AssignmentTarget::new(ValueId::new(0)),
            Type::Boolean,
            Expression::value(ClassicalValue::Boolean(true)),
        );

        assert_eq!(
            validate_assignment(&assignment),
            Err(AssignmentError::InvalidTarget(ValueId::new(0)))
        );
    }

    #[test]
    fn constant_assignment_is_detected_structurally() {
        let assignment = Assignment::new(
            AssignmentId::new(1),
            AssignmentTarget::new(ValueId::new(2)),
            Type::Boolean,
            Expression::value(ClassicalValue::Boolean(true)),
        );

        assert!(assignment.is_constant());
    }
}