
```rust
//! Zamani — Classical Numeric Arithmetic
//!
//! File: src/classical/numeric/arithmetic.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!   Contract-first implementation.
//!   Repository-wide compilation and integration verification are pending.
//!
//! PURPOSE
//!   Provide a hardware-independent arithmetic API over Zamani's canonical
//!   arbitrary-width integer and floating-point implementations.
//!
//! AUTHORITY
//!   Integer representation and semantics:
//!     crate::quantum::ir::classical::integer
//!
//!   Floating-point representation and semantics:
//!     crate::quantum::ir::classical::float
//!
//!   Arithmetic syntax:
//!     grammar/expressions/arithmetic.g4
//!     grammar/expressions/expressions.g4
//!
//! OWNERSHIP
//!   This file owns:
//!     - the arithmetic operator vocabulary at this API boundary;
//!     - domain-specific checked arithmetic dispatch;
//!     - arithmetic error categorization and propagation;
//!     - integer exponentiation using canonical integer operations;
//!     - arithmetic conformance tests.
//!
//!   This file does not own:
//!     - numeric representations or canonical numeric types;
//!     - source syntax, tokenization, AST, or expression precedence;
//!     - semantic type inference or overload resolution;
//!     - the aggregate ClassicalValue type;
//!     - canonical IR, expression IR, or symbolic expression trees;
//!     - rounding policy, numeric promotion, or implicit conversions;
//!     - target-specific instructions or hardware selection;
//!     - runtime scheduling, memory allocation policy, or resource discovery.
//!
//! SEMANTIC CONTRACT
//!   - No implicit integer wrapping or saturation.
//!   - Unsigned subtraction rejects negative results.
//!   - Signed integer division truncates toward zero.
//!   - Signed integer remainder follows the canonical implementation.
//!   - Integer exponentiation requires a non-negative integer exponent.
//!   - Floating-point inputs and results must satisfy the canonical
//!     ClassicalFloat finite-value contract.
//!   - Remainder and division reject zero divisors.
//!   - No operation silently changes numeric domains.
//!   - Power is exponentiation, not an invented grammar operator: callers
//!     must map the canonical POWER token to ArithmeticOperator::Power.
//!
//! POCO-REAF
//!   There is no universal integer-width, hardware-count, or data-size
//!   ceiling in this module. Integer operations delegate to the canonical
//!   arbitrary-width implementation. Actual computations remain subject to
//!   available resources and explicit compiler/runtime policies.
//!
//! INTEGRATION
//!   1. Declare `pub mod arithmetic;` in src/classical/numeric/mod.rs.
//!   2. Expose `numeric` through src/classical/mod.rs.
//!   3. Map AST arithmetic operators to ArithmeticOperator during semantic
//!      analysis; do not parse or reimplement grammar here.
//!   4. Use these functions for concrete numeric evaluation only. Symbolic
//!      expressions must remain symbolic in their owning expression/IR layer.
//!   5. Keep type promotion, fixed-width overflow policy, and conversions in
//!      their designated semantic/type-system owners.
//!   6. Do not create a second ClassicalValue or canonical numeric IR.
//!
//! FREEZE RULE
//!   Changes to canonical numeric representations belong in their canonical
//!   owners. Changes to language-level overflow, conversion, or rounding
//!   semantics require a specification and compatibility review.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
};

use crate::quantum::ir::classical::integer::{
    IntegerError,
    SignedInteger,
    UnsignedInteger,
};

/// Arithmetic operators supported by this concrete numeric API.
///
/// This enum represents semantic dispatch, not source tokens or AST nodes.
/// The frontend remains responsible for mapping source operators to these
/// variants after parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperator {
    /// Addition.
    Add,

    /// Subtraction.
    Subtract,

    /// Multiplication.
    Multiply,

    /// Division.
    Divide,

    /// Remainder.
    Remainder,

    /// Exponentiation.
    Power,
}

/// Errors produced by the classical arithmetic boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticError {
    /// A canonical integer operation failed.
    Integer(IntegerError),

    /// A canonical floating-point operation failed.
    Float(ClassicalFloatError),

    /// An unsigned subtraction would produce a negative result.
    NegativeUnsignedResult,

    /// An integer exponent is negative.
    NegativeIntegerExponent,

    /// An operation is not defined by this concrete arithmetic API.
    UnsupportedOperation {
        /// The requested operator.
        operator: ArithmeticOperator,

        /// The numeric domain.
        domain: &'static str,
    },
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(error) => {
                write!(formatter, "integer arithmetic failed: {error}")
            }

            Self::Float(error) => {
                write!(formatter, "floating-point arithmetic failed: {error}")
            }

            Self::NegativeUnsignedResult => {
                formatter.write_str(
                    "unsigned subtraction would produce a negative result",
                )
            }

            Self::NegativeIntegerExponent => {
                formatter.write_str(
                    "integer exponentiation requires a non-negative exponent",
                )
            }

            Self::UnsupportedOperation { operator, domain } => {
                write!(
                    formatter,
                    "operator {operator:?} is unsupported for {domain}"
                )
            }
        }
    }
}

impl std::error::Error for ArithmeticError {}

impl From<IntegerError> for ArithmeticError {
    fn from(error: IntegerError) -> Self {
        Self::Integer(error)
    }
}

impl From<ClassicalFloatError> for ArithmeticError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Float(error)
    }
}

/// Result type for concrete classical arithmetic.
pub type ArithmeticResult<T> = Result<T, ArithmeticError>;

/// Applies an arithmetic operator to arbitrary-width unsigned integers.
///
/// No fixed-width truncation, wrapping, or saturation is performed.
///
/// Exponentiation uses exponentiation by squaring. The exponent is itself
/// arbitrary-width; it is not converted to `u32`, `u64`, or `usize`.
pub fn apply_unsigned(
    operator: ArithmeticOperator,
    lhs: &UnsignedInteger,
    rhs: &UnsignedInteger,
) -> ArithmeticResult<UnsignedInteger> {
    match operator {
        ArithmeticOperator::Add => {
            Ok(lhs.checked_add(rhs)?)
        }

        ArithmeticOperator::Subtract => {
            lhs.checked_sub(rhs)
                .map_err(|error| match error {
                    IntegerError::ConversionOverflow => {
                        ArithmeticError::NegativeUnsignedResult
                    }
                    other => ArithmeticError::Integer(other),
                })
        }

        ArithmeticOperator::Multiply => {
            Ok(lhs.checked_mul(rhs)?)
        }

        ArithmeticOperator::Divide => {
            Ok(lhs.checked_div(rhs)?)
        }

        ArithmeticOperator::Remainder => {
            Ok(lhs.checked_rem(rhs)?)
        }

        ArithmeticOperator::Power => {
            unsigned_power(lhs, rhs)
        }
    }
}

/// Applies an arithmetic operator to arbitrary-width signed integers.
///
/// Division truncates toward zero according to the canonical integer
/// implementation. Integer exponentiation rejects negative exponents because
/// negative powers generally require a rational or floating-point domain.
pub fn apply_signed(
    operator: ArithmeticOperator,
    lhs: &SignedInteger,
    rhs: &SignedInteger,
) -> ArithmeticResult<SignedInteger> {
    match operator {
        ArithmeticOperator::Add => {
            Ok(lhs.checked_add(rhs)?)
        }

        ArithmeticOperator::Subtract => {
            Ok(lhs.checked_sub(rhs)?)
        }

        ArithmeticOperator::Multiply => {
            Ok(lhs.checked_mul(rhs)?)
        }

        ArithmeticOperator::Divide => {
            Ok(lhs.checked_div(rhs)?)
        }

        ArithmeticOperator::Remainder => {
            Ok(lhs.checked_rem(rhs)?)
        }

        ArithmeticOperator::Power => {
            if rhs.is_negative() {
                return Err(ArithmeticError::NegativeIntegerExponent);
            }

            let exponent = rhs.magnitude();
            let magnitude = unsigned_power(lhs.magnitude(), exponent)?;

            SignedInteger::from_parts(lhs.is_negative(), magnitude)
                .map_err(ArithmeticError::Integer)
        }
    }
}

/// Applies an arithmetic operator to canonical finite floating-point values.
///
/// This boundary uses the canonical float constructor to reject non-finite
/// results. It does not introduce implicit conversions, approximate equality,
/// alternate rounding modes, or target-specific floating-point behavior.
pub fn apply_float(
    operator: ArithmeticOperator,
    lhs: ClassicalFloat,
    rhs: ClassicalFloat,
) -> ArithmeticResult<ClassicalFloat> {
    let left = lhs.get();
    let right = rhs.get();

    let result = match operator {
        ArithmeticOperator::Add => left + right,

        ArithmeticOperator::Subtract => left - right,

        ArithmeticOperator::Multiply => left * right,

        ArithmeticOperator::Divide => {
            if right == 0.0 {
                return Err(ArithmeticError::Float(
                    ClassicalFloatError::DivisionByZero,
                ));
            }

            left / right
        }

        ArithmeticOperator::Remainder => {
            if right == 0.0 {
                return Err(ArithmeticError::Float(
                    ClassicalFloatError::DivisionByZero,
                ));
            }

            left % right
        }

        ArithmeticOperator::Power => left.powf(right),
    };

    ClassicalFloat::new(result).map_err(ArithmeticError::Float)
}

/// Computes `base ^ exponent` for arbitrary-width unsigned integers.
///
/// This is exponentiation by squaring. It avoids a linear number of
/// multiplications for large exponents and never converts the exponent into a
/// primitive integer.
///
/// The algorithm does not impose an architectural size ceiling. The resulting
/// integer can nevertheless require substantial memory and computation.
pub fn unsigned_power(
    base: &UnsignedInteger,
    exponent: &UnsignedInteger,
) -> ArithmeticResult<UnsignedInteger> {
    let mut result = UnsignedInteger::from_u8(1);

    if exponent.is_zero() {
        // The concrete integer API defines 0^0 as 1 for this operation.
        return Ok(result);
    }

    if base.is_zero() {
        return Ok(UnsignedInteger::zero());
    }

    let mut factor = base.clone();
    let exponent_width = exponent.bit_width();

    for index in 0..exponent_width {
        if exponent.bit(index)? {
            result = result.checked_mul(&factor)?;
        }

        // Avoid an unnecessary final square.
        if index + 1 < exponent_width {
            factor = factor.checked_mul(&factor)?;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{
        apply_float,
        apply_signed,
        apply_unsigned,
        unsigned_power,
        ArithmeticError,
        ArithmeticOperator,
    };

    use crate::quantum::ir::classical::float::{
        ClassicalFloat,
        ClassicalFloatError,
    };

    use crate::quantum::ir::classical::integer::{
        IntegerError,
        SignedInteger,
        UnsignedInteger,
    };

    #[test]
    fn unsigned_addition_and_multiplication_are_exact() {
        let a = UnsignedInteger::from_u64(u64::MAX);
        let b = UnsignedInteger::from_u8(2);

        let sum = apply_unsigned(ArithmeticOperator::Add, &a, &b)
            .expect("arbitrary-width addition should succeed");

        let product =
            apply_unsigned(ArithmeticOperator::Multiply, &a, &b)
                .expect("arbitrary-width multiplication should succeed");

        assert_eq!(
            sum,
            UnsignedInteger::from_u128(u64::MAX as u128 + 2)
        );

        assert_eq!(
            product,
            UnsignedInteger::from_u128(u64::MAX as u128 * 2)
        );
    }

    #[test]
    fn unsigned_subtraction_rejects_negative_results() {
        let smaller = UnsignedInteger::from_u8(2);
        let larger = UnsignedInteger::from_u8(3);

        assert_eq!(
            apply_unsigned(
                ArithmeticOperator::Subtract,
                &smaller,
                &larger,
            ),
            Err(ArithmeticError::NegativeUnsignedResult)
        );
    }

    #[test]
    fn unsigned_division_and_remainder_reject_zero() {
        let numerator = UnsignedInteger::from_u64(10);
        let zero = UnsignedInteger::zero();

        assert_eq!(
            apply_unsigned(
                ArithmeticOperator::Divide,
                &numerator,
                &zero,
            ),
            Err(ArithmeticError::Integer(
                IntegerError::DivisionByZero
            ))
        );

        assert_eq!(
            apply_unsigned(
                ArithmeticOperator::Remainder,
                &numerator,
                &zero,
            ),
            Err(ArithmeticError::Integer(
                IntegerError::DivisionByZero
            ))
        );
    }

    #[test]
    fn signed_division_truncates_toward_zero() {
        let numerator = SignedInteger::from_i64(-7);
        let denominator = SignedInteger::from_i64(3);

        let quotient = apply_signed(
            ArithmeticOperator::Divide,
            &numerator,
            &denominator,
        )
        .expect("nonzero divisor");

        let remainder = apply_signed(
            ArithmeticOperator::Remainder,
            &numerator,
            &denominator,
        )
        .expect("nonzero divisor");

        assert_eq!(quotient, SignedInteger::from_i64(-2));
        assert_eq!(remainder, SignedInteger::from_i64(-1));
    }

    #[test]
    fn integer_power_handles_zero_and_large_exponents() {
        let zero = UnsignedInteger::zero();
        let one = UnsignedInteger::from_u8(1);
        let base = UnsignedInteger::from_u8(2);
        let exponent = UnsignedInteger::from_u8(10);

        assert_eq!(
            unsigned_power(&zero, &zero).expect("zero exponent"),
            one
        );

        assert_eq!(
            unsigned_power(&base, &exponent).expect("power"),
            UnsignedInteger::from_u16(1024)
        );
    }

    #[test]
    fn signed_power_rejects_negative_exponents() {
        let base = SignedInteger::from_i64(2);
        let exponent = SignedInteger::from_i64(-1);

        assert_eq!(
            apply_signed(ArithmeticOperator::Power, &base, &exponent),
            Err(ArithmeticError::NegativeIntegerExponent)
        );
    }

    #[test]
    fn floating_point_arithmetic_uses_canonical_values() {
        let left = ClassicalFloat::new(6.0).expect("finite input");
        let right = ClassicalFloat::new(2.0).expect("finite input");

        assert_eq!(
            apply_float(ArithmeticOperator::Add, left, right)
                .expect("addition")
                .get(),
            8.0
        );

        assert_eq!(
            apply_float(ArithmeticOperator::Divide, left, right)
                .expect("division")
                .get(),
            3.0
        );

        assert_eq!(
            apply_float(ArithmeticOperator::Remainder, left, right)
                .expect("remainder")
                .get(),
            0.0
        );
    }

    #[test]
    fn floating_point_division_by_zero_is_rejected() {
        let numerator = ClassicalFloat::new(1.0).expect("finite input");
        let zero = ClassicalFloat::zero();

        assert_eq!(
            apply_float(ArithmeticOperator::Divide, numerator, zero),
            Err(ArithmeticError::Float(
                ClassicalFloatError::DivisionByZero
            ))
        );
    }

    #[test]
    fn floating_point_overflow_does_not_return_infinity() {
        let left = ClassicalFloat::new(f64::MAX).expect("finite input");
        let right = ClassicalFloat::new(2.0).expect("finite input");

        assert_eq!(
            apply_float(ArithmeticOperator::Multiply, left, right),
            Err(ArithmeticError::Float(
                ClassicalFloatError::NonFinite
            ))
        );
    }

    #[test]
    fn float_power_uses_the_explicit_power_operator() {
        let base = ClassicalFloat::new(2.0).expect("finite input");
        let exponent = ClassicalFloat::new(10.0).expect("finite input");

        assert_eq!(
            apply_float(ArithmeticOperator::Power, base, exponent)
                .expect("finite result")
                .get(),
            1024.0
        );
    }
}
```
