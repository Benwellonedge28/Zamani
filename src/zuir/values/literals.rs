#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Portable Literals
//!
//! Canonical path: `src/zuir/values/literals.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the literal-facing API for typed, portable ZUIR values while
//! reusing the canonical constant representation defined in `constants.rs`.
//!
//! A literal's represented value is independent of a target's native
//! integer width, floating-point format, register width, memory capacity,
//! quantum-resource capacity, or execution architecture.
//!
//! ## STATUS
//!
//! Proposed new module. Freeze after module integration, compilation,
//! tests, API review, and validation of serialization compatibility.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/values/constants.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/expressions/literals.g4`
//! - `grammar/lexer/numeric-literals.g4`
//! - `src/frontend/ast/node/expressions/literal.rs`
//!
//! ## OWNS
//!
//! - The public literal facade over canonical typed constants.
//! - Literal-specific construction and access methods.
//! - Delegation of representation validation to the constant subsystem.
//! - Literal validation errors and validation-budget integration.
//!
//! ## DOES_NOT_OWN
//!
//! - Source literal grammar, lexing, escape processing, or parsing.
//! - Source AST nodes, source spans, or source-preserving syntax.
//! - A second numeric, string, aggregate, or extension-value model.
//! - Name resolution, type inference, or type compatibility.
//! - Constant evaluation, arithmetic, or constant folding.
//! - Target-specific numeric conversion or physical layout.
//! - Dialect registration or extension execution.
//! - Complete-program serialization or IR verification.
//!
//! ## INTEGRATION
//!
//! - `constants.rs` owns `Constant`, `ConstantValue`, and validation budgets.
//! - `type_def.rs` owns semantic `TypeRef`.
//! - The frontend AST owns source-level literal spelling and syntax.
//! - Semantic analysis determines the literal's semantic type.
//! - ZUIR type verification checks value/type compatibility.
//! - `expressions.rs` may represent literal values using `ConstantValue`.
//! - Serialization must validate decoded literals before trusting them.
//! - Target-specific lowering owns conversions into concrete representations.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module introduces no fixed integer width, decimal precision, string
//! length, aggregate length, tensor rank, qubit count, device count, or
//! hardware-capacity limit.
//!
//! Collections and text are owned by the canonical constant model. Callers
//! may impose explicit operational validation budgets. Such budgets do not
//! redefine the language's semantic limits.
//!
//! No implementation can promise literal infinity: all operations remain
//! subject to available memory, address space, time, and finite identifier
//! or indexing representations.
//!
//! ## SEMANTIC BOUNDARY
//!
//! Local literal validation establishes representation validity only.
//! It does not prove that a literal conforms to its declared semantic type.
//! The canonical type verifier owns that decision.
//!
//! ## FREEZE CRITERIA
//!
//! - No duplicate constant-value representation.
//! - No implicit conversion through host-sized numeric types.
//! - Validation delegates to the canonical constant implementation.
//! - Deserialized values can be explicitly revalidated.
//! - Resource limits are explicit and configurable.
//! - No source parser, evaluator, or target dependency is introduced.
//! - Module integration, tests, and repository compilation succeed.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::zuir::types::type_def::TypeRef;
use crate::zuir::values::constants::{
    Constant,
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

/// The canonical validation budget used for portable literals.
///
/// The budget limits validation work, not the semantic size of a valid
/// literal. `None` means that the corresponding explicit limit is absent.
pub type LiteralValidationBudget = ConstantValidationBudget;

/// The canonical portable value representation used by literals.
///
/// This alias deliberately prevents `literals.rs` from creating a competing
/// value enum. Add new canonical value forms in `constants.rs`, not here.
pub type LiteralValue = ConstantValue;

/// A typed literal in the ZUIR value model.
///
/// This is a lightweight facade over [`Constant`]. It deliberately reuses
/// the canonical type-and-value representation rather than duplicating it.
///
/// Construction does not establish that the represented value is compatible
/// with its declared type. Call [`Literal::validate`] for local value
/// validation and the canonical type verifier for semantic validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Literal {
    constant: Constant,
}

impl Literal {
    /// Construct a typed literal.
    ///
    /// This method does not evaluate the value or establish semantic type
    /// compatibility. Validate the result before trusting external input.
    #[must_use]
    pub fn new(ty: TypeRef, value: LiteralValue) -> Self {
        Self {
            constant: Constant::new(ty, value),
        }
    }

    /// Wrap an existing canonical constant as a literal.
    #[must_use]
    pub fn from_constant(constant: Constant) -> Self {
        Self { constant }
    }

    /// Borrow the literal's declared semantic type.
    #[must_use]
    pub fn ty(&self) -> &TypeRef {
        self.constant.ty()
    }

    /// Borrow the literal's canonical value.
    #[must_use]
    pub fn value(&self) -> &LiteralValue {
        self.constant.value()
    }

    /// Borrow the canonical constant underlying this literal.
    #[must_use]
    pub fn as_constant(&self) -> &Constant {
        &self.constant
    }

    /// Consume this literal and return its canonical constant.
    #[must_use]
    pub fn into_constant(self) -> Constant {
        self.constant
    }

    /// Validate the literal's local value representation.
    ///
    /// No additional validation ceiling is imposed by this method.
    /// The constant subsystem performs iterative structural validation.
    ///
    /// This method does not validate type compatibility, resolve named
    /// types, or check whether a target supports the represented value.
    pub fn validate(&self) -> Result<(), LiteralValidationError> {
        self.validate_with(&LiteralValidationBudget::unlimited())
    }

    /// Validate the literal under an explicit operational budget.
    ///
    /// The configured limits are passed to the canonical constant
    /// implementation. This method does not silently replace an exhausted
    /// budget with an unlimited one.
    pub fn validate_with(
        &self,
        budget: &LiteralValidationBudget,
    ) -> Result<(), LiteralValidationError> {
        self.constant
            .validate_with(budget)
            .map_err(LiteralValidationError::InvalidConstant)
    }
}

impl From<Constant> for Literal {
    fn from(constant: Constant) -> Self {
        Self::from_constant(constant)
    }
}

impl From<Literal> for Constant {
    fn from(literal: Literal) -> Self {
        literal.into_constant()
    }
}

/// A literal representation validation failure.
///
/// Semantic type errors, unresolved type references, and target feasibility
/// errors are intentionally handled by their respective subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LiteralValidationError {
    /// The canonical constant representation is invalid or exceeded its
    /// configured validation budget.
    InvalidConstant(ConstantValidationError),
}

impl fmt::Display for LiteralValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConstant(error) => {
                write!(f, "invalid ZUIR literal: {error}")
            }
        }
    }
}

impl Error for LiteralValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidConstant(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::schema::types::QualifiedName;

    fn test_type() -> TypeRef {
        TypeRef::Named(
            QualifiedName::parse("core::Boolean")
                .expect("test type name must be valid"),
        )
    }

    #[test]
    fn constructs_and_validates_boolean_literal() {
        let literal = Literal::new(test_type(), LiteralValue::Boolean(true));

        assert_eq!(literal.value(), &LiteralValue::Boolean(true));
        assert!(literal.validate().is_ok());
    }

    #[test]
    fn converts_to_and_from_canonical_constant() {
        let constant = Constant::new(test_type(), LiteralValue::Boolean(false));
        let literal = Literal::from(constant.clone());

        assert_eq!(literal.as_constant(), &constant);

        let recovered: Constant = literal.into();
        assert_eq!(recovered, constant);
    }

    #[test]
    fn validation_respects_node_budget() {
        let literal = Literal::new(
            test_type(),
            LiteralValue::Sequence(vec![
                LiteralValue::Boolean(true),
                LiteralValue::Boolean(false),
            ]),
        );

        let budget = LiteralValidationBudget::new(Some(2), None);

        assert!(matches!(
            literal.validate_with(&budget),
            Err(LiteralValidationError::InvalidConstant(
                ConstantValidationError::NodeBudgetExceeded
            ))
        ));
    }

    #[test]
    fn validation_rejects_invalid_canonical_value() {
        let literal = Literal::new(
            test_type(),
            LiteralValue::BytesHex("0G".to_owned()),
        );

        assert!(matches!(
            literal.validate(),
            Err(LiteralValidationError::InvalidConstant(
                ConstantValidationError::InvalidByteEncoding
            ))
        ));
    }

    #[test]
    fn arbitrary_width_integer_remains_a_canonical_constant() {
        use crate::zuir::values::constants::ArbitraryInteger;

        let integer = ArbitraryInteger::new(
            "12345678901234567890123456789012345678901234567890",
        )
        .expect("canonical arbitrary-width integer");

        let literal = Literal::new(test_type(), LiteralValue::Integer(integer));

        assert!(literal.validate().is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_literal() {
        let literal = Literal::new(test_type(), LiteralValue::Boolean(true));

        let encoded =
            serde_json::to_string(&literal).expect("literal should serialize");

        let decoded: Literal =
            serde_json::from_str(&encoded).expect("literal should deserialize");

        assert_eq!(decoded, literal);
        assert!(decoded.validate().is_ok());
    }
}