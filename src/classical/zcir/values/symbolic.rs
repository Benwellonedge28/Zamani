
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Symbolic Values
//!
//! Canonical path: `src/classical/zcir/values/symbolic.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the ZCIR integration API for symbolic constant values while
//! preserving ZUIR as the authoritative owner of the value representation.
//!
//! A symbolic value represents an expression or symbolic reference whose
//! meaning has not yet been evaluated. It is not, by itself, an expression
//! AST, a resolved symbol, a type, or an executable operation.
//!
//! ## Ownership
//!
//! This module owns:
//! - Convenience constructors for canonical symbolic constant values.
//! - Symbolic-value inspection and extraction helpers.
//! - Symbolic-value construction and validation integration tests.
//!
//! This module does not own:
//! - A second symbolic-value enum or constant representation.
//! - Expression ASTs or the canonical ZUIR expression model.
//! - Symbol resolution, type inference, or semantic analysis.
//! - Expression evaluation, constant folding, or algebraic simplification.
//! - Parsing or interpretation of symbolic expression strings.
//! - Symbol, value, operation, or source identifiers.
//! - Physical memory allocation, scheduling, or target lowering.
//!
//! ## Authority
//!
//! The authoritative value representation is:
//!
//! `crate::zuir::values::constants::ConstantValue::Symbolic`
//!
//! The authoritative local validation policy and error types are:
//!
//! - `ConstantValidationBudget`
//! - `ConstantValidationError`
//!
//! The canonical expression representation is owned by
//! `crate::zuir::values::expressions`.
//!
//! ## Integration contract
//!
//! Frontend:
//!   Source spelling -> lexer/parser -> AST.
//!
//! Semantic analysis:
//!   Resolve names and determine expression meaning and type.
//!
//! ZUIR expressions:
//!   Represent structured, typed expressions and their resolved references.
//!
//! ZUIR constants:
//!   Represent symbolic constant payloads without evaluating them.
//!
//! ZCIR:
//!   Reuse the canonical symbolic constant representation through this
//!   adapter; do not create a competing symbolic data model.
//!
//! Verification and evaluation:
//!   Verify types, symbol references, expression legality, and evaluation
//!   requirements in their owning subsystems.
//!
//! Lowering:
//!   Convert only verified and supported representations, preserving
//!   semantics or returning an explicit error.
//!
//! ## Representation contract
//!
//! The `expression` field is opaque text. This module does not claim that it
//! is syntactically valid Zamani, resolved, well-typed, or executable.
//! A nonempty expression string is only a locally valid symbolic payload.
//!
//! Callers needing structural expression semantics must use the canonical
//! expression model instead of encoding an AST into this string.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no fixed expression-length or symbolic-value-count
//! ceiling. It does not use host-sized integers to encode expression meaning.
//!
//! An explicit validation budget may limit validation work and text bytes.
//! Such a budget is an operational policy, not a language-level restriction.
//! Validation budgets cannot undo memory already consumed constructing a
//! caller-provided string.
//!
//! No implementation can promise literal infinity. Practical limits remain
//! bounded by available memory, address space, execution time, and other
//! finite resources.
//!
//! ## Serialization contract
//!
//! Serialization uses the canonical `ConstantValue` representation and its
//! existing Serde implementation. This module does not define a second wire
//! format. Deserialized values must be validated before being trusted.
//!
//! ## Error contract
//!
//! Construction and validation reuse `ConstantValidationError` directly.
//! This module does not wrap canonical errors in a competing error hierarchy.
//!
//! ## Freeze criteria
//!
//! - Uses the canonical ZUIR symbolic representation.
//! - Rejects empty and whitespace-only symbolic payloads.
//! - Supports explicit validation budgets.
//! - Does not parse or evaluate expression text.
//! - Does not duplicate expression or constant types.
//! - Uses safe Rust only.
//! - Passes module and repository integration tests.

use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

/// Construct a canonical symbolic constant.
///
/// The input is retained as opaque expression text. This function validates
/// the local symbolic representation but does not parse, resolve, type-check,
/// or evaluate the expression.
///
/// For resource-constrained or untrusted inputs, prefer
/// [`symbolic_value_with_budget`].
///
/// # Errors
///
/// Returns the canonical validation error if the expression is empty or
/// contains only whitespace.
pub fn symbolic_value(
    expression: impl Into<String>,
) -> Result<ConstantValue, ConstantValidationError> {
    symbolic_value_with_budget(
        expression,
        &ConstantValidationBudget::unlimited(),
    )
}

/// Construct a canonical symbolic constant under an explicit budget.
///
/// The budget is applied after the input has been converted into an owned
/// `String`. Therefore, callers processing untrusted input must also enforce
/// appropriate limits at the input-reading or decoding boundary.
///
/// The budget limits validation work; it is not a general memory allocator
/// limit and does not restrict the semantics of Zamani programs.
///
/// # Errors
///
/// Returns a canonical validation error if the symbolic payload is invalid
/// or if the configured validation budget is exceeded.
pub fn symbolic_value_with_budget(
    expression: impl Into<String>,
    budget: &ConstantValidationBudget,
) -> Result<ConstantValue, ConstantValidationError> {
    let value = ConstantValue::Symbolic {
        expression: expression.into(),
    };

    value.validate_with(budget)?;
    Ok(value)
}

/// Borrow the symbolic expression text, if `value` is symbolic.
///
/// The returned string is opaque payload text, not a parsed expression AST.
///
/// Returns `None` for every non-symbolic constant variant.
#[must_use]
pub fn as_symbolic(value: &ConstantValue) -> Option<&str> {
    match value {
        ConstantValue::Symbolic { expression } => Some(expression.as_str()),
        _ => None,
    }
}

/// Consume a constant and extract its symbolic expression text.
///
/// Returns `None` if the value is not symbolic. This operation does not
/// validate the payload; call the canonical value validator before trusting
/// values originating from untrusted sources.
#[must_use]
pub fn into_symbolic(value: ConstantValue) -> Option<String> {
    match value {
        ConstantValue::Symbolic { expression } => Some(expression),
        _ => None,
    }
}

/// Return whether a constant uses the canonical symbolic-value variant.
///
/// This is a representation check only. It does not validate the payload
/// or establish that the expression is meaningful.
#[must_use]
pub fn is_symbolic(value: &ConstantValue) -> bool {
    matches!(value, ConstantValue::Symbolic { .. })
}

/// Validate a symbolic constant with no explicit operational budget.
///
/// This delegates to the canonical constant validator, which validates
/// symbolic payloads along with other constant representations.
///
/// This function accepts a `ConstantValue` because callers may need to
/// validate a symbolic value obtained from deserialization or another
/// canonical-value API. It validates the complete value representation;
/// it does not require the root value to be symbolic.
///
/// For a constructor that guarantees a symbolic root, use
/// [`symbolic_value`] instead.
pub fn validate_symbolic_representation(
    value: &ConstantValue,
) -> Result<(), ConstantValidationError> {
    value.validate()
}

/// Validate a constant representation under an explicit operational budget.
///
/// This delegates to ZUIR's canonical validator, preserving its node-count,
/// text-byte accounting, error behavior, and traversal guarantees.
///
/// The function does not require the root value to be symbolic; nested
/// symbolic values inside aggregates are validated by the canonical
/// validator as well.
pub fn validate_symbolic_representation_with_budget(
    value: &ConstantValue,
    budget: &ConstantValidationBudget,
) -> Result<(), ConstantValidationError> {
    value.validate_with(budget)
}

#[cfg(test)]
mod tests {
    use super::{
        as_symbolic,
        into_symbolic,
        is_symbolic,
        symbolic_value,
        symbolic_value_with_budget,
        validate_symbolic_representation,
        validate_symbolic_representation_with_budget,
    };

    use crate::zuir::values::constants::{
        ConstantValidationBudget,
        ConstantValidationError,
        ConstantValue,
    };

    #[test]
    fn constructs_symbolic_value() {
        let value = symbolic_value("x + y").expect("valid symbolic value");

        assert_eq!(as_symbolic(&value), Some("x + y"));
        assert!(is_symbolic(&value));
    }

    #[test]
    fn preserves_expression_text_without_rewriting_it() {
        let expression = "  x + (y * z)  ";
        let value = symbolic_value(expression).expect("nonempty symbolic value");

        assert_eq!(as_symbolic(&value), Some(expression));
    }

    #[test]
    fn rejects_empty_expression() {
        assert_eq!(
            symbolic_value(""),
            Err(ConstantValidationError::EmptySymbolicExpression)
        );
    }

    #[test]
    fn rejects_whitespace_only_expression() {
        assert_eq!(
            symbolic_value(" \t\n "),
            Err(ConstantValidationError::EmptySymbolicExpression)
        );
    }

    #[test]
    fn accepts_unicode_expression_text() {
        let expression = "α + β";
        let value = symbolic_value(expression).expect("valid Unicode text");

        assert_eq!(as_symbolic(&value), Some(expression));
    }

    #[test]
    fn budget_rejects_zero_allowed_nodes() {
        let budget = ConstantValidationBudget::new(Some(0), None);

        assert_eq!(
            symbolic_value_with_budget("x", &budget),
            Err(ConstantValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn budget_rejects_insufficient_text_bytes() {
        let budget = ConstantValidationBudget::new(None, Some(2));

        assert_eq!(
            symbolic_value_with_budget("long", &budget),
            Err(ConstantValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn budget_accepts_payload_within_limits() {
        let budget = ConstantValidationBudget::new(Some(1), Some(3));
        let value =
            symbolic_value_with_budget("x+y", &budget).expect("within budget");

        assert_eq!(as_symbolic(&value), Some("x+y"));
    }

    #[test]
    fn accessors_return_none_for_non_symbolic_values() {
        let value = ConstantValue::Boolean(true);

        assert_eq!(as_symbolic(&value), None);
        assert_eq!(into_symbolic(value.clone()), None);
        assert!(!is_symbolic(&value));
    }

    #[test]
    fn consuming_accessor_returns_expression_text() {
        let value = symbolic_value("compute(input)").expect("valid symbolic value");

        assert_eq!(
            into_symbolic(value),
            Some("compute(input)".to_owned())
        );
    }

    #[test]
    fn validates_canonical_symbolic_value() {
        let value = symbolic_value("x").expect("valid symbolic value");

        assert_eq!(validate_symbolic_representation(&value), Ok(()));
    }

    #[test]
    fn canonical_validator_rejects_invalid_deserialized_payload() {
        // Deserialization can construct data without going through the
        // checked symbolic constructor. Validation remains mandatory.
        let value = ConstantValue::Symbolic {
            expression: "   ".to_owned(),
        };

        assert_eq!(
            validate_symbolic_representation(&value),
            Err(ConstantValidationError::EmptySymbolicExpression)
        );
    }

    #[test]
    fn budgeted_validation_delegates_to_canonical_validator() {
        let value = symbolic_value("x").expect("valid symbolic value");
        let budget = ConstantValidationBudget::new(Some(1), Some(1));

        assert_eq!(
            validate_symbolic_representation_with_budget(&value, &budget),
            Ok(())
        );
    }

    #[test]
    fn budgeted_validation_reports_text_exhaustion() {
        let value = symbolic_value("long expression").expect("valid symbolic value");
        let budget = ConstantValidationBudget::new(None, Some(2));

        assert_eq!(
            validate_symbolic_representation_with_budget(&value, &budget),
            Err(ConstantValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn canonical_serialization_round_trips_symbolic_value() {
        let original =
            symbolic_value("f(argument)").expect("valid symbolic value");

        let encoded =
            serde_json::to_string(&original).expect("serialize canonical value");

        let decoded: ConstantValue =
            serde_json::from_str(&encoded).expect("deserialize canonical value");

        assert_eq!(decoded, original);
        assert_eq!(as_symbolic(&decoded), Some("f(argument)"));
        assert_eq!(validate_symbolic_representation(&decoded), Ok(()));
    }

    #[test]
    fn nested_symbolic_values_use_canonical_validation() {
        let value = ConstantValue::Sequence(vec![
            symbolic_value("first").expect("valid first expression"),
            symbolic_value("second").expect("valid second expression"),
        ]);

        assert_eq!(validate_symbolic_representation(&value), Ok(()));
    }

    #[test]
    fn nested_invalid_symbolic_value_is_rejected() {
        let value = ConstantValue::Sequence(vec![ConstantValue::Symbolic {
            expression: String::new(),
        }]);

        assert_eq!(
            validate_symbolic_representation(&value),
            Err(ConstantValidationError::EmptySymbolicExpression)
        );
    }
}
