
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Floating-Point Values
//!
//! Path: `src/classical/zcir/values/float.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the ZCIR-facing API for portable floating-point constants by
//! adapting the canonical representation in `crate::zuir::values::constants`.
//!
//! This module deliberately does not introduce a second floating-point
//! representation. The canonical `FloatBits` and `ConstantValue` types remain
//! the owners of the stored value and its structural equality and
//! serialization behavior.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The ZCIR-facing import path for canonical floating-point constants.
//! - Checked construction from a format identifier and canonical hexadecimal
//!   bit-pattern encoding.
//! - Borrowed and consuming extraction from canonical constant values.
//! - Convenience entry points for canonical validation.
//!
//! This module does not own:
//!
//! - Source literal parsing or decimal-to-binary conversion.
//! - Floating-point format definitions or format registration.
//! - The mapping from a format name to its bit width or encoding.
//! - Floating-point arithmetic, rounding, overflow, or underflow semantics.
//! - Type inference, type compatibility, or implicit conversions.
//! - Target-specific lowering or hardware capability checks.
//! - Runtime allocation or execution.
//!
//! ## Canonical representation
//!
//! `FloatBits` stores:
//!
//! - `format`: a nonempty semantic format identifier;
//! - `bits_hex`: a canonical lowercase hexadecimal representation of the
//!   bit pattern.
//!
//! The canonical validator rejects empty encodings, uppercase hexadecimal,
//! invalid hexadecimal characters, and redundant leading zeroes.
//!
//! The representation must not be converted through `f32`, `f64`, a host
//! floating-point parser, or a JSON numeric value. Such conversions could
//! lose precision, payload bits, signed zero, or distinctions between
//! floating-point formats.
//!
//! The hexadecimal encoding is an integer-like canonical representation.
//! The format registry or semantic type verifier is responsible for
//! establishing the format's bit width and interpreting its bit pattern.
//!
//! ## Floating-point semantics
//!
//! Structural equality is representation equality: the format identifier
//! and bit-pattern encoding must both match.
//!
//! This preserves distinctions such as positive and negative zero, distinct
//! NaN payloads, and different encodings that have different bit patterns.
//! This module does not define numeric comparison or arithmetic semantics.
//!
//! Whether a format admits NaNs, infinities, subnormals, signed zero, or
//! particular payload encodings is determined by that format's contract and
//! the relevant semantic specification—not by this adapter.
//!
//! ## POCO-REAF and scalability
//!
//! No fixed floating-point precision, exponent width, bit width, or target
//! capacity is imposed here.
//!
//! This module accepts format identifiers and bit encodings supported by the
//! canonical representation. It does not claim that every target supports
//! every format, or that physical resources are unlimited.
//!
//! Callers validating untrusted or very large values should use the canonical
//! constant-validation budget. `None` in that budget means that no explicit
//! limit is imposed for that dimension; it does not guarantee infinite
//! resources.
//!
//! ## Integration contract
//!
//! - `values/mod.rs` declares `pub mod float;` and selectively re-exports
//!   this module's public API.
//! - `values/constant.rs` reuses canonical `Constant` and `ConstantValue`.
//! - `values/integer.rs` continues to own only the integer adapter.
//! - `types/float.rs` owns or adapts semantic floating-point type information;
//!   it must not duplicate `FloatBits`.
//! - `types/layout.rs` and the format registry define format/layout rules.
//! - The frontend parses source spellings and produces canonical values
//!   without silently selecting a host-specific format.
//! - The semantic verifier checks that a constant's format and value conform
//!   to its declared type.
//! - Arithmetic and constant folding belong to the evaluator or appropriate
//!   optimization subsystem.
//! - ZUIR lowering preserves the representation or reports an explicit
//!   unsupported-format or conversion error.
//! - Target-specific conversion must declare its rounding and exceptional
//!   value policy; it must not silently reinterpret the bits.
//!
//! ## Freeze criteria
//!
//! - No duplicate floating-point value type is introduced.
//! - No implicit conversion through host floating-point primitives occurs.
//! - Canonical validation and errors are reused.
//! - Validation budgets are passed to the canonical validator unchanged.
//! - Tests cover malformed encodings, large encodings, bit preservation,
//!   serialization, and budget exhaustion.
//! - Repository integration and compilation pass before this file is frozen.

use crate::zuir::values::constants::{
    ConstantValidationBudget,
    ConstantValidationError,
    ConstantValue,
    FloatBits,
};

// =============================================================================
// Canonical public API
// =============================================================================

/// Canonical floating-point bit representation exposed through ZCIR.
///
/// This is a re-export of the ZUIR-owned type, not a second representation.
/// Equality and serialization preserve the canonical format identifier and
/// bit-pattern encoding.
pub use crate::zuir::values::constants::FloatBits as ZcirFloatValue;

/// Canonical constant-value representation used for ZCIR value integration.
pub use crate::zuir::values::constants::ConstantValue as ZcirConstantValue;

/// Canonical validation budget for constant values.
pub use crate::zuir::values::constants::ConstantValidationBudget
    as ZcirFloatValidationBudget;

/// Canonical error type for constant representation validation.
pub use crate::zuir::values::constants::ConstantValidationError
    as ZcirFloatValidationError;

// =============================================================================
// Construction
// =============================================================================

/// Construct a canonical floating-point representation.
///
/// `format` identifies the semantic floating-point format. The format name
/// does not imply target support, a specific physical representation, or a
/// conversion policy.
///
/// `bits_hex` is the canonical lowercase hexadecimal bit-pattern encoding
/// expected by the canonical ZUIR representation.
///
/// This function does not parse a source-language numeric literal and does
/// not convert a decimal number to a floating-point bit pattern.
///
/// The format registry or semantic type verifier must separately check
/// format-specific constraints, including the required bit width.
///
/// # Errors
///
/// Returns a canonical `ConstantValidationError` if the format identifier
/// or bit-pattern encoding fails local validation.
pub fn from_bit_pattern(
    format: impl Into<String>,
    bits_hex: impl Into<String>,
) -> Result<FloatBits, ConstantValidationError> {
    FloatBits::new(format, bits_hex)
}

/// Wrap a canonical floating-point representation in a constant value.
///
/// No conversion or normalization is performed. The contained format and
/// bit pattern are preserved.
#[must_use]
pub fn into_constant_value(value: FloatBits) -> ConstantValue {
    ConstantValue::Float(value)
}

// =============================================================================
// Inspection
// =============================================================================

/// Borrow a floating-point representation from a canonical constant.
///
/// Returns `None` if `value` is not the canonical floating-point variant.
///
/// This is a representation query, not a semantic type-compatibility check.
#[must_use]
pub fn as_float(value: &ConstantValue) -> Option<&FloatBits> {
    match value {
        ConstantValue::Float(float) => Some(float),
        _ => None,
    }
}

/// Consume a canonical constant and extract its floating-point
/// representation, if present.
///
/// Returns `None` for other constant variants. No conversion, normalization,
/// or replacement allocation is performed.
#[must_use]
pub fn into_float(value: ConstantValue) -> Option<FloatBits> {
    match value {
        ConstantValue::Float(float) => Some(float),
        _ => None,
    }
}

/// Return the format identifier of a floating-point representation.
///
/// The returned name has not necessarily been resolved by a format registry.
#[must_use]
pub fn format_name(value: &FloatBits) -> &str {
    value.format.as_str()
}

/// Return the canonical hexadecimal bit-pattern encoding.
///
/// This returns the representation exactly as stored. It does not interpret
/// the encoding as an integer or convert it to a host floating-point value.
#[must_use]
pub fn bit_pattern_hex(value: &FloatBits) -> &str {
    value.bits_hex.as_str()
}

// =============================================================================
// Validation
// =============================================================================

/// Validate a floating-point representation using the canonical ZUIR rules.
///
/// This validates the local encoding. It does not establish that the format
/// exists in the format registry, that the bit width matches the format, or
/// that the value is compatible with a declared semantic type.
pub fn validate_float(value: &FloatBits) -> Result<(), ConstantValidationError> {
    value.validate()
}

/// Validate a canonical constant value using the default canonical policy.
///
/// The validation behavior is delegated to ZUIR. For aggregates or other
/// value kinds, the canonical validator checks the complete represented
/// value according to its own rules.
pub fn validate_constant_value(
    value: &ConstantValue,
) -> Result<(), ConstantValidationError> {
    value.validate()
}

/// Validate a canonical constant value with an explicit operational budget.
///
/// Budget exhaustion is returned using the canonical error variants. This
/// function does not replace the supplied budget with a hidden default and
/// does not treat budget exhaustion as an invalid floating-point encoding.
pub fn validate_constant_value_with(
    value: &ConstantValue,
    budget: &ConstantValidationBudget,
) -> Result<(), ConstantValidationError> {
    value.validate_with(budget)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_float() -> FloatBits {
        from_bit_pattern("core::binary32", "3f800000")
            .expect("valid canonical floating-point representation")
    }

    #[test]
    fn constructs_canonical_float_representation() {
        let value = sample_float();

        assert_eq!(format_name(&value), "core::binary32");
        assert_eq!(bit_pattern_hex(&value), "3f800000");
        assert!(validate_float(&value).is_ok());
    }

    #[test]
    fn rejects_empty_format_identifier() {
        assert_eq!(
            from_bit_pattern("", "0"),
            Err(ConstantValidationError::InvalidFloatFormat)
        );

        assert_eq!(
            from_bit_pattern("   ", "0"),
            Err(ConstantValidationError::InvalidFloatFormat)
        );
    }

    #[test]
    fn rejects_empty_bit_pattern() {
        assert_eq!(
            from_bit_pattern("core::binary32", ""),
            Err(ConstantValidationError::InvalidFloatBits)
        );
    }

    #[test]
    fn rejects_uppercase_hexadecimal() {
        assert_eq!(
            from_bit_pattern("core::binary32", "3F800000"),
            Err(ConstantValidationError::InvalidFloatBits)
        );
    }

    #[test]
    fn rejects_non_hexadecimal_bit_pattern() {
        assert_eq!(
            from_bit_pattern("core::binary32", "3f80zz00"),
            Err(ConstantValidationError::InvalidFloatBits)
        );
    }

    #[test]
    fn rejects_noncanonical_leading_zeroes() {
        assert_eq!(
            from_bit_pattern("core::binary32", "03f800000"),
            Err(ConstantValidationError::NonCanonicalFloatBits)
        );

        assert_eq!(
            from_bit_pattern("core::binary32", "00"),
            Err(ConstantValidationError::NonCanonicalFloatBits)
        );
    }

    #[test]
    fn accepts_canonical_zero_bit_pattern() {
        let value = from_bit_pattern("core::binary32", "0")
            .expect("canonical zero bit pattern");

        assert_eq!(bit_pattern_hex(&value), "0");
        assert!(validate_float(&value).is_ok());
    }

    #[test]
    fn preserves_large_bit_pattern_without_host_float_conversion() {
        let bits = format!("1{}", "a".repeat(4096));
        let value = from_bit_pattern("extension::wide_float", bits.clone())
            .expect("valid arbitrary-length canonical hexadecimal encoding");

        assert_eq!(bit_pattern_hex(&value), bits);
        assert_eq!(format_name(&value), "extension::wide_float");
    }

    #[test]
    fn preserves_distinct_bit_patterns() {
        let first = from_bit_pattern("core::binary32", "0").unwrap();
        let second = from_bit_pattern("core::binary32", "80000000").unwrap();

        // The adapter compares representations; it does not collapse
        // encodings that a format may interpret as positive and negative zero.
        assert_ne!(first, second);
    }

    #[test]
    fn preserves_distinct_payload_encodings() {
        let first = from_bit_pattern("core::binary32", "7fc00001").unwrap();
        let second = from_bit_pattern("core::binary32", "7fc00002").unwrap();

        // This layer does not classify NaNs or rewrite payload bits.
        assert_ne!(first, second);
    }

    #[test]
    fn wraps_and_borrows_canonical_constant_value() {
        let float = sample_float();
        let value = into_constant_value(float.clone());

        assert_eq!(as_float(&value), Some(&float));
        assert!(validate_constant_value(&value).is_ok());
    }

    #[test]
    fn consumes_canonical_constant_value() {
        let float = sample_float();
        let value = into_constant_value(float.clone());

        assert_eq!(into_float(value), Some(float));
    }

    #[test]
    fn extraction_returns_none_for_non_float_values() {
        let value = ConstantValue::Boolean(true);

        assert!(as_float(&value).is_none());
        assert!(into_float(value).is_none());
    }

    #[test]
    fn canonical_validation_rejects_malformed_deserialized_shape() {
        // Public fields and deserialization can construct values without
        // going through FloatBits::new. Validate before trusting such values.
        let malformed = FloatBits {
            format: "core::binary32".to_owned(),
            bits_hex: "ABC".to_owned(),
        };

        assert_eq!(
            validate_float(&malformed),
            Err(ConstantValidationError::InvalidFloatBits)
        );
    }

    #[test]
    fn forwards_explicit_node_budget() {
        let value = into_constant_value(sample_float());
        let budget = ConstantValidationBudget::new(Some(0), None);

        assert_eq!(
            validate_constant_value_with(&value, &budget),
            Err(ConstantValidationError::NodeBudgetExceeded)
        );
    }

    #[test]
    fn forwards_explicit_text_budget() {
        let value = into_constant_value(sample_float());
        let budget = ConstantValidationBudget::new(None, Some(1));

        assert_eq!(
            validate_constant_value_with(&value, &budget),
            Err(ConstantValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn validates_with_sufficient_budget() {
        let value = into_constant_value(sample_float());
        let budget = ConstantValidationBudget::new(Some(1), Some(64));

        assert!(validate_constant_value_with(&value, &budget).is_ok());
    }

    #[test]
    fn canonical_serialization_round_trip_preserves_bits() {
        let original = into_constant_value(sample_float());

        let encoded =
            serde_json::to_string(&original).expect("serialize canonical constant");

        let decoded: ConstantValue =
            serde_json::from_str(&encoded).expect("deserialize canonical constant");

        assert_eq!(decoded, original);
        assert!(validate_constant_value(&decoded).is_ok());
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let value = into_constant_value(sample_float());

        let first = validate_constant_value(&value);
        let second = validate_constant_value(&value);

        assert_eq!(first, second);
    }
}
