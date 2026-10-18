
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Integer Values
//!
//! Path: `src/classical/zcir/values/integer.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the classical-facing API for portable integer values in ZCIR.
//!
//! This module adapts the canonical integer representation owned by
//! `crate::zuir::values::constants`. It does not create another integer
//! representation or an independent arithmetic implementation.
//!
//! ## Authority and ownership
//!
//! `crate::zuir::values::constants` owns:
//!
//! - `ArbitraryInteger`, the canonical arbitrary-width integer representation.
//! - `ConstantValue::Integer`, the canonical integer constant variant.
//! - `ConstantValidationBudget`, the operational validation policy.
//! - `ConstantValidationError`, the representation-validation errors.
//!
//! The frontend owns source-literal parsing.
//!
//! The semantic model owns the meaning of integer types, operations,
//! conversions, overflow behavior, and observable results.
//!
//! The type subsystem owns integer signedness and width.
//!
//! The evaluator owns arbitrary-precision arithmetic and its work budgets.
//!
//! The verifier owns type compatibility and IR invariants.
//!
//! Lowering owns target-specific representation and feasibility checks.
//!
//! ## This module owns
//!
//! - The stable ZCIR-facing integer-value import path.
//! - Validated construction from canonical decimal notation.
//! - Non-narrowing inspection of sign, zero, and magnitude.
//! - Conversion between the canonical integer and constant-value forms.
//! - Convenience validation entry points using canonical ZUIR validation.
//!
//! ## This module does not own
//!
//! - A second integer struct or numeric-value enum.
//! - Integer arithmetic or constant folding.
//! - Parsing binary, octal, hexadecimal, or separator-containing source
//!   literals.
//! - Signedness, integer type widths, or type inference.
//! - Overflow, underflow, wrapping, saturating, or trapping policies.
//! - Implicit conversions to machine integers or floating-point values.
//! - Memory allocation, register selection, or target-specific layout.
//!
//! ## Canonical representation
//!
//! An integer is stored as a canonical decimal string.
//!
//! Valid representations include:
//!
//! - `0`
//! - `1`
//! - `42`
//! - `-42`
//! - Decimal integers with more digits than any native machine integer.
//!
//! Invalid representations include:
//!
//! - `+42`
//! - `042`
//! - `-0`
//! - Empty strings.
//! - Whitespace or non-decimal digits.
//!
//! Source spellings such as binary or hexadecimal literals must be parsed
//! and converted by the authoritative literal/frontend subsystem before
//! being passed to this canonical decimal constructor.
//!
//! ## Scalability and POCO-REAF
//!
//! No fixed maximum integer width, digit count, register width, or collection
//! capacity is imposed by this adapter.
//!
//! A caller may supply an explicit operational validation budget. An
//! unrestricted validation request means only that this adapter adds no
//! configured budget; it does not guarantee unlimited physical resources.
//!
//! Construction and inspection avoid conversion to native numeric types.
//! All actual storage remains bounded by available memory and the
//! representational limits of the underlying implementation.
//!
//! ## Integration contract
//!
//! 1. `src/classical/zcir/values/mod.rs` declares `pub mod integer;`.
//! 2. That module may re-export the names documented below.
//! 3. `values/constant.rs` reuses the same canonical integer type.
//! 4. `values/aggregate.rs` stores integers through `ConstantValue`.
//! 5. `values/symbolic.rs` does not evaluate symbolic integer expressions.
//! 6. The frontend passes parsed, semantically interpreted integer values
//!    into this API without requiring target-specific narrowing.
//! 7. The canonical evaluator implements arithmetic and operation-specific
//!    resource accounting.
//! 8. The type verifier checks an integer value against its semantic type.
//! 9. ZUIR lowering preserves exact values or reports explicit infeasibility.
//!
//! ## Error policy
//!
//! This adapter reuses the canonical validation error type.
//! A wrong `ConstantValue` variant is represented by `None` from the
//! optional extraction API, not by misusing `InvalidInteger`.
//! Budget exhaustion remains distinguishable from malformed encodings.
//!
//! ## Freeze criteria
//!
//! - No competing integer representation is introduced.
//! - No numeric narrowing or implicit arithmetic is performed.
//! - Canonical decimal validation is delegated to ZUIR.
//! - Explicit validation budgets are passed through unchanged.
//! - The source grammar and semantic ownership boundaries are preserved.
//! - Unit tests and repository-wide integration checks pass.

use crate::zuir::values::constants::{
    ArbitraryInteger,
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
};

// =============================================================================
// Canonical public API
// =============================================================================

/// The canonical arbitrary-width integer representation exposed through ZCIR.
///
/// This is a re-export, not a new integer type. Values retain the same
/// equality, serialization, and validation behavior as their ZUIR owners.
pub use crate::zuir::values::constants::ArbitraryInteger as ZcirIntegerValue;

/// The canonical constant-value representation used for integer integration.
pub use crate::zuir::values::constants::ConstantValue as ZcirConstantValue;

/// The canonical validation budget for integer constant values.
pub use crate::zuir::values::constants::ConstantValidationBudget
    as ZcirIntegerValidationBudget;

/// The canonical validation error for integer constant values.
pub use crate::zuir::values::constants::ConstantValidationError
    as ZcirIntegerValidationError;

// =============================================================================
// Construction
// =============================================================================

/// Construct an integer from canonical decimal notation.
///
/// This function does not parse source-language literal syntax. For example,
/// a source spelling such as `0xff` must first be interpreted by the frontend
/// and converted into its canonical decimal representation.
///
/// No native integer conversion is performed.
///
/// # Errors
///
/// Returns `InvalidInteger` if the supplied string is not canonical decimal
/// notation.
pub fn from_canonical_decimal(
    decimal: impl Into<String>,
) -> Result<ArbitraryInteger, ConstantValidationError> {
    ArbitraryInteger::new(decimal)
}

/// Construct the canonical integer zero.
///
/// Zero is represented as `"0"`, never `"-0"` or another normalized spelling.
#[must_use]
pub fn zero() -> ArbitraryInteger {
    ArbitraryInteger::zero()
}

/// Construct a canonical integer constant value.
///
/// The returned value uses the canonical ZUIR `ConstantValue::Integer`
/// variant. It does not introduce a ZCIR-specific value representation.
pub fn into_constant_value(integer: ArbitraryInteger) -> ConstantValue {
    ConstantValue::Integer(integer)
}

// =============================================================================
// Inspection
// =============================================================================

/// Return a reference to the canonical integer if the value is an integer.
///
/// Returns `None` for every other constant variant. This is a representation
/// query, not a type-compatibility check.
#[must_use]
pub fn as_integer(value: &ConstantValue) -> Option<&ArbitraryInteger> {
    match value {
        ConstantValue::Integer(integer) => Some(integer),
        _ => None,
    }
}

/// Consume a constant value and extract its canonical integer, if present.
///
/// Returns `None` when the value is not an integer. No parsing, conversion,
/// normalization, or allocation of a replacement integer is performed.
#[must_use]
pub fn into_integer(value: ConstantValue) -> Option<ArbitraryInteger> {
    match value {
        ConstantValue::Integer(integer) => Some(integer),
        _ => None,
    }
}

/// Return whether an integer is zero.
///
/// This uses the canonical representation and does not parse the integer
/// into a native numeric type.
#[must_use]
pub fn is_zero(integer: &ArbitraryInteger) -> bool {
    integer.is_zero()
}

/// Return whether an integer is negative.
///
/// The caller must validate externally supplied or deserialized values before
/// relying on this query. Canonical integer strings use `-` only for negative
/// values and do not permit negative zero.
#[must_use]
pub fn is_negative(integer: &ArbitraryInteger) -> bool {
    integer.as_str().starts_with('-')
}

/// Return whether an integer is positive.
///
/// This is false for zero and negative integers.
#[must_use]
pub fn is_positive(integer: &ArbitraryInteger) -> bool {
    !is_zero(integer) && !is_negative(integer)
}

/// Return the unsigned decimal magnitude of a canonical integer.
///
/// For example, `-123` yields `123`, `123` yields `123`, and `0` yields `0`.
///
/// This method does not construct another integer and does not convert the
/// magnitude to a native numeric type.
///
/// Validate untrusted or deserialized integers before relying on this
/// operation.
#[must_use]
pub fn magnitude_decimal(integer: &ArbitraryInteger) -> &str {
    integer
        .as_str()
        .strip_prefix('-')
        .unwrap_or_else(|| integer.as_str())
}

// =============================================================================
// Validation
// =============================================================================

/// Validate a canonical integer representation.
///
/// The canonical implementation owns the accepted encoding rules. This
/// function deliberately does not reimplement them.
pub fn validate_integer(
    integer: &ArbitraryInteger,
) -> Result<(), ConstantValidationError> {
    integer.validate()
}

/// Validate an integer as a constant value.
///
/// This delegates to the canonical constant validator. It ensures that
/// validation follows the same rules as other ZUIR constant values.
pub fn validate_integer_value(
    integer: &ArbitraryInteger,
) -> Result<(), ConstantValidationError> {
    ConstantValue::Integer(integer.clone()).validate()
}

/// Validate an integer under an explicit operational budget.
///
/// The canonical constant validator accounts for the root node and the
/// encoded decimal text according to its documented budget policy.
///
/// The supplied policy is passed through without modification.
pub fn validate_integer_value_with(
    integer: &ArbitraryInteger,
    budget: &ConstantValidationBudget,
) -> Result<(), ConstantValidationError> {
    ConstantValue::Integer(integer.clone()).validate_with(budget)
}

/// Validate a constant value and extract an integer reference.
///
/// Returns `Ok(None)` for a valid non-integer constant, `Ok(Some(...))` for
/// a valid integer constant, and an error for an invalid constant encoding.
///
/// This distinction prevents a wrong value variant from being misreported
/// as a malformed integer.
pub fn validate_and_as_integer(
    value: &ConstantValue,
) -> Result<Option<&ArbitraryInteger>, ConstantValidationError> {
    value.validate()?;
    Ok(as_integer(value))
}

/// Validate a constant value under a supplied budget and extract its integer
/// reference when the value is an integer.
///
/// Budget exhaustion and malformed values remain canonical validation errors.
pub fn validate_and_as_integer_with(
    value: &ConstantValue,
    budget: &ConstantValidationBudget,
) -> Result<Option<&ArbitraryInteger>, ConstantValidationError> {
    value.validate_with(budget)?;
    Ok(as_integer(value))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_zero_canonically() {
        let value = zero();

        assert_eq!(value.as_str(), "0");
        assert!(is_zero(&value));
        assert!(!is_negative(&value));
        assert!(!is_positive(&value));
        assert_eq!(magnitude_decimal(&value), "0");
    }

    #[test]
    fn accepts_canonical_positive_integer() {
        let value = from_canonical_decimal("123456789")
            .expect("canonical decimal integer should be accepted");

        assert_eq!(value.as_str(), "123456789");
        assert!(is_positive(&value));
        assert!(!is_negative(&value));
        assert!(!is_zero(&value));
        assert_eq!(magnitude_decimal(&value), "123456789");
    }

    #[test]
    fn accepts_canonical_negative_integer() {
        let value = from_canonical_decimal("-123456789")
            .expect("canonical negative integer should be accepted");

        assert_eq!(value.as_str(), "-123456789");
        assert!(is_negative(&value));
        assert!(!is_positive(&value));
        assert!(!is_zero(&value));
        assert_eq!(magnitude_decimal(&value), "123456789");
    }

    #[test]
    fn preserves_values_beyond_native_integer_widths() {
        let decimal = "12345678901234567890123456789012345678901234567890";
        let value = from_canonical_decimal(decimal)
            .expect("arbitrary-width canonical integer should be accepted");

        assert_eq!(value.as_str(), decimal);
        assert_eq!(magnitude_decimal(&value), decimal);
        assert!(validate_integer(&value).is_ok());
    }

    #[test]
    fn does_not_accept_noncanonical_integer_encodings() {
        for invalid in ["", "+1", "01", "-0", "-01", " 1", "1 ", "1.0", "1e3"] {
            assert_eq!(
                from_canonical_decimal(invalid),
                Err(ConstantValidationError::InvalidInteger),
                "expected rejection of {invalid:?}",
            );
        }
    }

    #[test]
    fn preserves_integer_when_wrapped_as_constant_value() {
        let integer = from_canonical_decimal("-987654321")
            .expect("valid integer");

        let value = into_constant_value(integer.clone());

        assert_eq!(as_integer(&value), Some(&integer));
        assert_eq!(into_integer(value), Some(integer));
    }

    #[test]
    fn returns_none_for_non_integer_values() {
        let boolean = ConstantValue::Boolean(true);
        let string = ConstantValue::String("42".to_owned());

        assert!(as_integer(&boolean).is_none());
        assert!(as_integer(&string).is_none());

        assert!(into_integer(boolean).is_none());
        assert!(into_integer(string).is_none());
    }

    #[test]
    fn validates_integer_using_canonical_rules() {
        let integer = from_canonical_decimal("123456789")
            .expect("valid integer");

        assert!(validate_integer(&integer).is_ok());
        assert!(validate_integer_value(&integer).is_ok());
    }

    #[test]
    fn accepts_budget_sufficient_for_integer() {
        let integer = from_canonical_decimal("123")
            .expect("valid integer");

        let budget = ConstantValidationBudget::new(Some(1), Some(3));

        assert!(validate_integer_value_with(&integer, &budget).is_ok());
    }

    #[test]
    fn reports_node_budget_exhaustion() {
        let integer = from_canonical_decimal("1")
            .expect("valid integer");

        let budget = ConstantValidationBudget::new(Some(0), None);

        assert_eq!(
            validate_integer_value_with(&integer, &budget),
            Err(ConstantValidationError::NodeBudgetExceeded),
        );
    }

    #[test]
    fn reports_text_budget_exhaustion() {
        let integer = from_canonical_decimal("123")
            .expect("valid integer");

        let budget = ConstantValidationBudget::new(None, Some(2));

        assert_eq!(
            validate_integer_value_with(&integer, &budget),
            Err(ConstantValidationError::TextBudgetExceeded),
        );
    }

    #[test]
    fn extraction_validates_before_returning_integer() {
        let value = ConstantValue::Integer(
            from_canonical_decimal("456").expect("valid integer"),
        );

        let extracted = validate_and_as_integer(&value)
            .expect("valid constant")
            .expect("integer constant");

        assert_eq!(extracted.as_str(), "456");
    }

    #[test]
    fn extraction_preserves_valid_non_integer_variant() {
        let value = ConstantValue::Boolean(false);

        assert_eq!(validate_and_as_integer(&value), Ok(None));
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let integer = from_canonical_decimal(
            "1234567890123456789012345678901234567890",
        )
        .expect("valid arbitrary-width integer");

        assert_eq!(validate_integer_value(&integer), Ok(()));
        assert_eq!(validate_integer_value(&integer), Ok(()));
    }
}
