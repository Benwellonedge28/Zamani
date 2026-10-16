//! Zamani — Classical Floating-Point Adapter
//!
//! File: src/classical/values/float.rs
//!
//! STATUS
//!   Adapter implementation; repository-wide integration verification
//!   remains pending.
//!
//! PURPOSE
//!   Expose the canonical floating-point semantics through the classical
//!   computation subsystem without introducing a second float implementation.
//!
//! CANONICAL AUTHORITY
//!   crate::quantum::ir::classical::float
//!
//! OWNERSHIP
//!   The canonical IR module owns:
//!     - ClassicalFloat
//!     - ClassicalFloatError
//!     - FloatFormat
//!     - floating-point representation invariants
//!     - checked floating-point operations
//!     - canonical IEEE-754 bit representation
//!
//!   This adapter owns only the classical-facing import boundary.
//!
//! DOES NOT OWN
//!   - floating-point lexical syntax or literal parsing;
//!   - source AST nodes;
//!   - FloatType or the canonical type system;
//!   - ClassicalValue or aggregate value semantics;
//!   - symbolic parameter expressions;
//!   - target-specific numeric formats;
//!   - memory allocation or execution;
//!   - serialization formats or content-hashing algorithms;
//!   - hardware capability discovery;
//!   - compiler or runtime resource budgets.
//!
//! POCO-REAF
//!   The adapter contains no hardware-specific assumptions or artificial
//!   semantic limits. The underlying f64 representation is finite precision;
//!   it is not a claim of arbitrary precision or infinite numeric range.
//!
//! SAFETY
//!   No unsafe Rust is permitted.
//!
//! RUST
//!   Rust 2021; Rust 1.97 or later.
//!
//! INTEGRATION CONTRACT
//!   1. values/mod.rs declares `pub mod float` and re-exports this API.
//!   2. classical/mod.rs exposes the values module.
//!   3. The canonical IR implementation remains the sole owner of float
//!      semantics.
//!   4. ClassicalValue integration must use a deliberate compatibility
//!      migration; this adapter does not silently replace FiniteFloat.
//!   5. Frontend literal parsing must validate input and report diagnostics
//!      through the existing frontend/diagnostic contracts.
//!   6. Canonical serialization and hashing must use the established IR
//!      serialization and hashing contracts, not a new local format.
//!
//! FREEZE RULE
//!   Changes to the canonical float representation, numeric semantics, or
//!   error contract require review in the canonical IR owner. Changes to
//!   this adapter must not duplicate those definitions.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// Re-export the canonical implementation.
//
// Do not define a second ClassicalFloat, ClassicalFloatError, FloatFormat,
// floating-point representation, arithmetic engine, or serialization format
// here. These names must resolve to the exact same types used by the IR.
pub use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
    FloatFormat,
};

#[cfg(test)]
mod tests {
    use super::{
        ClassicalFloat,
        ClassicalFloatError,
        FloatFormat,
    };

    #[test]
    fn adapter_exposes_the_canonical_float_type() {
        let value = ClassicalFloat::new(1.25)
            .expect("a finite value must be accepted");

        assert_eq!(value.get(), 1.25);
        assert_eq!(value.format(), FloatFormat::F64);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn canonical_bit_representation_round_trips() {
        let original = ClassicalFloat::new(-12.5)
            .expect("a finite value must be accepted");

        let restored = ClassicalFloat::from_bits(original.bits())
            .expect("finite IEEE-754 bits must be accepted");

        assert_eq!(original, restored);
        assert_eq!(
            original.canonical_bytes(),
            restored.canonical_bytes()
        );
    }

    #[test]
    fn non_finite_inputs_are_rejected_by_the_canonical_owner() {
        assert_eq!(
            ClassicalFloat::new(f64::NAN),
            Err(ClassicalFloatError::NonFinite)
        );

        assert_eq!(
            ClassicalFloat::new(f64::INFINITY),
            Err(ClassicalFloatError::NonFinite)
        );

        assert_eq!(
            ClassicalFloat::new(f64::NEG_INFINITY),
            Err(ClassicalFloatError::NonFinite)
        );
    }

    #[test]
    fn arithmetic_uses_the_canonical_checked_contract() {
        let left = ClassicalFloat::new(6.0)
            .expect("finite input");
        let right = ClassicalFloat::new(2.0)
            .expect("finite input");

        assert_eq!(
            left.checked_add(right)
                .expect("finite sum")
                .get(),
            8.0
        );

        assert_eq!(
            left.checked_div(right)
                .expect("finite quotient")
                .get(),
            3.0
        );

        assert_eq!(
            left.checked_div(ClassicalFloat::zero()),
            Err(ClassicalFloatError::DivisionByZero)
        );
    }

    #[test]
    fn canonical_zero_identity_is_preserved() {
        let positive = ClassicalFloat::zero();
        let negative = ClassicalFloat::negative_zero();

        // Structural identity preserves the IEEE-754 sign bit.
        assert_ne!(positive, negative);

        // Numeric comparison still follows floating-point semantics.
        assert_eq!(positive.get(), negative.get());

        assert!(positive.is_positive_zero());
        assert!(negative.is_negative_zero());
    }

    #[test]
    fn float_format_metadata_is_hardware_independent() {
        assert_eq!(FloatFormat::F64.name(), "f64");
        assert_eq!(FloatFormat::F64.bit_width(), 64);
        assert_eq!(FloatFormat::F64.byte_width(), 8);
    }
}