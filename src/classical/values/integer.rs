
// src/classical/values/integer.rs
//
// Contract: Classical integer API and compatibility boundary
// Status: Proposed; integration and compiler verification pending
// Rust edition: 2021
// Minimum Rust version: 1.97
// Safety: unsafe Rust is forbidden
//
// PURPOSE
//   Expose Zamani's existing arbitrary-width integer semantics through the
//   classical subsystem without creating a second integer implementation.
//
// OWNERSHIP
//   The canonical implementation currently remains:
//       crate::quantum::ir::classical::integer
//
//   This module owns only the classical-facing public import boundary.
//   It does not redefine integer representations, arithmetic, or errors.
//
// DOES NOT OWN
//   - Source literals, parsing, AST nodes, or type checking.
//   - Semantic type identities or type-width declarations.
//   - The ClassicalValue enum.
//   - Runtime allocation, hardware representation, or execution.
//   - Compiler resource budgets or target capabilities.
//
// SCALABILITY
//   Integer width is determined by the value's canonical representation,
//   not by a universal machine-word width. Practical limits arise from
//   available memory, addressable storage, and explicit operational policy.
//
// INTEGRATION
//   - values/mod.rs declares this module and re-exports its public types.
//   - classical/mod.rs exposes the values module.
//   - quantum::ir::classical remains the existing implementation owner
//     until a separate compatibility-reviewed migration is completed.
//   - ClassicalValue integration must not introduce a second integer engine.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// Re-export the canonical arbitrary-width implementation.
// Do not duplicate these definitions in the classical subsystem.
pub use crate::quantum::ir::classical::integer::{
    IntegerError,
    IntegerValue,
    SignedInteger,
    UnsignedInteger,
};

#[cfg(test)]
mod tests {
    use super::{IntegerError, IntegerValue, SignedInteger, UnsignedInteger};

    #[test]
    fn primitive_unsigned_values_round_trip() {
        let value = UnsignedInteger::from_u64(u64::MAX);

        assert_eq!(value.to_u64(), Ok(u64::MAX));
        assert!(value.validate().is_ok());
    }

    #[test]
    fn primitive_signed_values_round_trip() {
        let positive = SignedInteger::from_i64(i64::MAX);
        let negative = SignedInteger::from_i64(i64::MIN);

        assert_eq!(positive.to_i64(), Ok(i64::MAX));
        assert_eq!(negative.to_i64(), Ok(i64::MIN));
        assert!(positive.validate().is_ok());
        assert!(negative.validate().is_ok());
    }

    #[test]
    fn arbitrary_width_unsigned_values_are_preserved() {
        // A 256-bit value, deliberately larger than primitive integer widths.
        let bytes = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
            0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x01,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xff,
        ];

        let value = UnsignedInteger::from_be_bytes(&bytes);

        assert_eq!(value.as_be_bytes(), bytes);
        assert_eq!(value.bit_width(), 256);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn canonical_zero_has_no_magnitude_bytes() {
        let value = UnsignedInteger::zero();

        assert!(value.is_zero());
        assert_eq!(value.as_be_bytes(), &[]);
        assert_eq!(value.bit_width(), 0);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn negative_unsigned_conversion_fails_explicitly() {
        let value = IntegerValue::Signed(SignedInteger::from_i64(-1));

        assert_eq!(
            value.to_unsigned(),
            Err(IntegerError::NegativeToUnsigned)
        );
    }

    #[test]
    fn width_validation_rejects_unrepresentable_values() {
        let value = UnsignedInteger::from_u64(255);

        assert!(value.validate_width(8).is_ok());
        assert!(value.validate_width(7).is_err());
    }

    #[test]
    fn checked_division_rejects_zero_divisor() {
        let numerator = UnsignedInteger::from_u64(123);
        let denominator = UnsignedInteger::zero();

        assert_eq!(
            numerator.checked_div(&denominator),
            Err(IntegerError::DivisionByZero)
        );
    }
}
