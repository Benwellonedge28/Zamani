
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Numeric Foundation
//!
//! Path: src/stdlib/foundation/numeric.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provide portable, reusable numerical operation contracts for the
//! Zamani standard library.
//!
//! This module supports:
//!
//! - Primitive signed integers.
//! - Primitive unsigned integers.
//! - Primitive floating-point values.
//! - Canonical arbitrary-width signed integers.
//! - Canonical arbitrary-width unsigned integers.
//! - Checked arithmetic.
//! - Generic numerical aggregation.
//! - Numeric interoperability.
//! - Explicit arithmetic failures.
//!
//! ARCHITECTURAL OWNERSHIP
//!
//! This module does not own:
//!
//! - Source-language numeric syntax.
//! - Semantic numeric types.
//! - Numeric literal parsing.
//! - Canonical arbitrary-width integer representation.
//! - Decimal or fixed-point representation.
//! - Tensor storage.
//! - Matrix storage.
//! - Classical IR.
//! - Quantum IR.
//! - HDL representation.
//! - Numerical algorithm execution.
//! - Hardware selection.
//! - Resource allocation.
//! - Runtime scheduling.
//!
//! CANONICAL AUTHORITIES
//!
//! Numeric source syntax:
//!     grammar/lexer/numeric-literals.g4
//!
//! Semantic types:
//!     src/frontend/semantic/types/ty.rs
//!
//! Arbitrary-width integers:
//!     src/quantum/ir/classical/integer.rs
//!
//! Classical numeric adapters:
//!     src/classical/values/
//!
//! Tensor storage:
//!     Existing canonical tensor subsystem.
//!
//! Numerical algorithms:
//!     src/stdlib/scientific/
//!
//! SCALABILITY
//!
//! This module defines no arbitrary maximum numeric width,
//! tensor rank, processor count, device count, or data size.
//!
//! Primitive Rust numeric types are interoperability types,
//! not the semantic limits of Zamani.
//!
//! Arbitrary-width integer operations delegate to the existing
//! canonical integer implementation.
//!
//! Physical resource limitations remain explicit operational
//! constraints rather than language-level restrictions.
//!
//! FLOATING-POINT POLICY
//!
//! Primitive checked floating-point operations reject non-finite
//! operands and non-finite results.
//!
//! This is an explicit finite-arithmetic API, not a redefinition
//! of all Zamani floating-point semantics.
//!
//! IEEE-754 operations requiring NaN, infinity, signed-zero,
//! or alternative rounding behavior must use the appropriate
//! explicitly specified numerical contract.
//!
//! DETERMINISM
//!
//! Generic aggregation functions preserve input iteration order.
//! They do not silently parallelize floating-point reductions.
//!
//! RESOURCE POLICY
//!
//! No global allocator, execution scheduler, hardware registry,
//! or machine discovery mechanism is introduced here.

use core::fmt;
use core::ops::Add;

use crate::quantum::ir::classical::integer::{
    IntegerError,
    SignedInteger,
    UnsignedInteger,
};

// =============================================================================
// 1. NUMERIC ERRORS
// =============================================================================

/// Operations that can produce a numerical failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumericOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    Negation,
    Summation,
    Product,
    DotProduct,
}

/// Structured errors produced by numerical operations.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum NumericError {
    /// Arithmetic cannot represent the requested result.
    Overflow {
        operation: NumericOperation,
    },

    /// Division or remainder by zero.
    DivisionByZero {
        operation: NumericOperation,
    },

    /// A negative value cannot be represented as unsigned.
    NegativeUnsigned,

    /// A floating-point operand is NaN or infinite.
    NonFiniteOperand,

    /// An operation produced NaN or infinity.
    NonFiniteResult {
        operation: NumericOperation,
    },

    /// A numerical input collection is empty.
    EmptyInput,

    /// Input lengths are incompatible.
    LengthMismatch {
        left: usize,
        right: usize,
    },

    /// An underlying canonical arbitrary-width integer operation failed.
    Integer(IntegerError),
}

impl fmt::Display for NumericOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Addition => "addition",
            Self::Subtraction => "subtraction",
            Self::Multiplication => "multiplication",
            Self::Division => "division",
            Self::Remainder => "remainder",
            Self::Negation => "negation",
            Self::Summation => "summation",
            Self::Product => "product",
            Self::DotProduct => "dot product",
        };

        f.write_str(name)
    }
}

impl fmt::Display for NumericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow { operation } => {
                write!(f, "numeric overflow during {operation}")
            }

            Self::DivisionByZero { operation } => {
                write!(f, "division by zero during {operation}")
            }

            Self::NegativeUnsigned => {
                f.write_str(
                    "negative value cannot be represented as unsigned"
                )
            }

            Self::NonFiniteOperand => {
                f.write_str("non-finite numerical operand")
            }

            Self::NonFiniteResult { operation } => {
                write!(
                    f,
                    "non-finite numerical result during {operation}"
                )
            }

            Self::EmptyInput => {
                f.write_str("numerical input is empty")
            }

            Self::LengthMismatch { left, right } => {
                write!(
                    f,
                    "numerical input lengths differ: {left} and {right}"
                )
            }

            Self::Integer(error) => {
                write!(f, "canonical integer operation failed: {error}")
            }
        }
    }
}

impl std::error::Error for NumericError {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IntegerError> for NumericError {
    fn from(error: IntegerError) -> Self {
        match error {
            IntegerError::DivisionByZero => {
                Self::DivisionByZero {
                    operation: NumericOperation::Division,
                }
            }

            other => Self::Integer(other),
        }
    }
}

// =============================================================================
// 2. NUMERIC OPERATION CONTRACT
// =============================================================================

/// Shared checked numerical operation interface.
///
/// Implementations must:
///
/// - Never silently wrap arithmetic.
/// - Never silently saturate arithmetic.
/// - Report unsupported operations explicitly.
/// - Preserve the canonical representation.
/// - Avoid narrowing conversions.
///
/// This trait is extensible to additional numeric representations.
pub trait NumericScalar: Clone + PartialEq {
    /// Returns additive identity.
    fn zero() -> Self;

    /// Returns multiplicative identity.
    fn one() -> Self;

    /// Returns whether this value represents zero.
    fn is_zero(&self) -> bool;

    /// Checked addition.
    fn checked_add(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError>;

    /// Checked subtraction.
    fn checked_sub(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError>;

    /// Checked multiplication.
    fn checked_mul(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError>;

    /// Checked division.
    fn checked_div(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError>;

    /// Checked remainder.
    fn checked_rem(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError>;

    /// Checked negation.
    ///
    /// Unsigned numeric types may reject nonzero negation.
    fn checked_neg(
        &self,
    ) -> Result<Self, NumericError>;
}

// =============================================================================
// 3. PRIMITIVE SIGNED INTEGER IMPLEMENTATIONS
// =============================================================================

macro_rules! implement_signed_numeric {
    ($($type:ty),+ $(,)?) => {
        $(
            impl NumericScalar for $type {
                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }

                fn is_zero(&self) -> bool {
                    *self == 0
                }

                fn checked_add(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_add(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Addition,
                        })
                }

                fn checked_sub(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_sub(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Subtraction,
                        })
                }

                fn checked_mul(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_mul(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Multiplication,
                        })
                }

                fn checked_div(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    if *rhs == 0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Division,
                        });
                    }

                    <$type>::checked_div(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Division,
                        })
                }

                fn checked_rem(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    if *rhs == 0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Remainder,
                        });
                    }

                    <$type>::checked_rem(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Remainder,
                        })
                }

                fn checked_neg(
                    &self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_neg(*self)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Negation,
                        })
                }
            }
        )+
    };
}

implement_signed_numeric!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
);

// =============================================================================
// 4. PRIMITIVE UNSIGNED INTEGER IMPLEMENTATIONS
// =============================================================================

macro_rules! implement_unsigned_numeric {
    ($($type:ty),+ $(,)?) => {
        $(
            impl NumericScalar for $type {
                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }

                fn is_zero(&self) -> bool {
                    *self == 0
                }

                fn checked_add(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_add(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Addition,
                        })
                }

                fn checked_sub(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_sub(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Subtraction,
                        })
                }

                fn checked_mul(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    <$type>::checked_mul(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Multiplication,
                        })
                }

                fn checked_div(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    if *rhs == 0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Division,
                        });
                    }

                    <$type>::checked_div(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Division,
                        })
                }

                fn checked_rem(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    if *rhs == 0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Remainder,
                        });
                    }

                    <$type>::checked_rem(*self, *rhs)
                        .ok_or(NumericError::Overflow {
                            operation: NumericOperation::Remainder,
                        })
                }

                fn checked_neg(
                    &self,
                ) -> Result<Self, NumericError> {
                    if self.is_zero() {
                        Ok(0)
                    } else {
                        Err(NumericError::NegativeUnsigned)
                    }
                }
            }
        )+
    };
}

implement_unsigned_numeric!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
);

// =============================================================================
// 5. FINITE FLOATING-POINT IMPLEMENTATIONS
// =============================================================================

macro_rules! implement_float_numeric {
    ($($type:ty),+ $(,)?) => {
        $(
            impl NumericScalar for $type {
                fn zero() -> Self {
                    0.0
                }

                fn one() -> Self {
                    1.0
                }

                fn is_zero(&self) -> bool {
                    *self == 0.0
                }

                fn checked_add(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;
                    ensure_finite(*rhs)?;

                    ensure_result(
                        *self + *rhs,
                        NumericOperation::Addition,
                    )
                }

                fn checked_sub(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;
                    ensure_finite(*rhs)?;

                    ensure_result(
                        *self - *rhs,
                        NumericOperation::Subtraction,
                    )
                }

                fn checked_mul(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;
                    ensure_finite(*rhs)?;

                    ensure_result(
                        *self * *rhs,
                        NumericOperation::Multiplication,
                    )
                }

                fn checked_div(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;
                    ensure_finite(*rhs)?;

                    if *rhs == 0.0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Division,
                        });
                    }

                    ensure_result(
                        *self / *rhs,
                        NumericOperation::Division,
                    )
                }

                fn checked_rem(
                    &self,
                    rhs: &Self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;
                    ensure_finite(*rhs)?;

                    if *rhs == 0.0 {
                        return Err(NumericError::DivisionByZero {
                            operation: NumericOperation::Remainder,
                        });
                    }

                    ensure_result(
                        *self % *rhs,
                        NumericOperation::Remainder,
                    )
                }

                fn checked_neg(
                    &self,
                ) -> Result<Self, NumericError> {
                    ensure_finite(*self)?;

                    ensure_result(
                        -*self,
                        NumericOperation::Negation,
                    )
                }
            }
        )+
    };
}

fn ensure_finite<T: FiniteNumber>(
    value: T,
) -> Result<(), NumericError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(NumericError::NonFiniteOperand)
    }
}

fn ensure_result<T: FiniteNumber>(
    value: T,
    operation: NumericOperation,
) -> Result<T, NumericError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(NumericError::NonFiniteResult { operation })
    }
}

/// Minimal finite-number interface for primitive floating-point values.
pub trait FiniteNumber: Copy {
    /// Whether the value is finite.
    fn is_finite(self) -> bool;
}

impl FiniteNumber for f32 {
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }
}

impl FiniteNumber for f64 {
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }
}

implement_float_numeric!(f32, f64);

// =============================================================================
// 6. CANONICAL ARBITRARY-WIDTH INTEGER ADAPTERS
// =============================================================================

/// Implements standard-library numeric operations for Zamani's
/// canonical arbitrary-width unsigned integer.
///
/// No new integer representation is introduced.
impl NumericScalar for UnsignedInteger {
    fn zero() -> Self {
        Self::from_u8(0)
    }

    fn one() -> Self {
        Self::from_u8(1)
    }

    fn is_zero(&self) -> bool {
        UnsignedInteger::is_zero(self)
    }

    fn checked_add(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        UnsignedInteger::checked_add(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_sub(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        UnsignedInteger::checked_sub(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_mul(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        UnsignedInteger::checked_mul(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_div(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        UnsignedInteger::checked_div(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_rem(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        UnsignedInteger::checked_rem(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_neg(
        &self,
    ) -> Result<Self, NumericError> {
        if self.is_zero() {
            Ok(Self::zero())
        } else {
            Err(NumericError::NegativeUnsigned)
        }
    }
}

/// Implements standard-library numeric operations for Zamani's
/// canonical arbitrary-width signed integer.
///
/// No new integer representation is introduced.
impl NumericScalar for SignedInteger {
    fn zero() -> Self {
        Self::from_i8(0)
    }

    fn one() -> Self {
        Self::from_i8(1)
    }

    fn is_zero(&self) -> bool {
        SignedInteger::is_zero(self)
    }

    fn checked_add(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        SignedInteger::checked_add(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_sub(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        SignedInteger::checked_sub(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_mul(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        SignedInteger::checked_mul(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_div(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        SignedInteger::checked_div(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_rem(
        &self,
        rhs: &Self,
    ) -> Result<Self, NumericError> {
        SignedInteger::checked_rem(self, rhs)
            .map_err(NumericError::from)
    }

    fn checked_neg(
        &self,
    ) -> Result<Self, NumericError> {
        Ok(SignedInteger::negated(self))
    }
}

// =============================================================================
// 7. GENERIC NUMERIC OPERATIONS
// =============================================================================

/// Performs checked addition.
pub fn add<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, NumericError> {
    left.checked_add(right)
}

/// Performs checked subtraction.
pub fn subtract<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, NumericError> {
    left.checked_sub(right)
}

/// Performs checked multiplication.
pub fn multiply<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, NumericError> {
    left.checked_mul(right)
}

/// Performs checked division.
pub fn divide<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, NumericError> {
    left.checked_div(right)
}

/// Performs checked remainder.
pub fn remainder<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, NumericError> {
    left.checked_rem(right)
}

/// Performs checked negation.
pub fn negate<T: NumericScalar>(
    value: &T,
) -> Result<T, NumericError> {
    value.checked_neg()
}

// =============================================================================
// 8. GENERIC NUMERIC AGGREGATION
// =============================================================================

/// Calculates the checked sum of an iterator.
///
/// The iteration order is preserved.
///
/// Floating-point reassociation or parallel reduction is not
/// performed implicitly.
pub fn checked_sum<T, I>(
    values: I,
) -> Result<T, NumericError>
where
    T: NumericScalar,
    I: IntoIterator<Item = T>,
{
    values.into_iter().try_fold(
        T::zero(),
        |accumulator, value| {
            accumulator.checked_add(&value)
        },
    )
}

/// Calculates the checked product of an iterator.
///
/// The identity for an empty product is one.
pub fn checked_product<T, I>(
    values: I,
) -> Result<T, NumericError>
where
    T: NumericScalar,
    I: IntoIterator<Item = T>,
{
    values.into_iter().try_fold(
        T::one(),
        |accumulator, value| {
            accumulator.checked_mul(&value)
        },
    )
}

/// Calculates a checked dot product.
///
/// Input iterators must contain the same number of values.
///
/// No matrix or tensor storage is introduced.
pub fn checked_dot_product<T, A, B>(
    left: A,
    right: B,
) -> Result<T, NumericError>
where
    T: NumericScalar,
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = T>,
{
    let mut left = left.into_iter();
    let mut right = right.into_iter();

    let mut accumulator = T::zero();

    loop {
        match (left.next(), right.next()) {
            (Some(a), Some(b)) => {
                let product = a.checked_mul(&b)?;

                accumulator =
                    accumulator.checked_add(&product)?;
            }

            (None, None) => {
                return Ok(accumulator);
            }

            (Some(_), None) | (None, Some(_)) => {
                return Err(NumericError::LengthMismatch {
                    left: 0,
                    right: 0,
                });
            }
        }
    }
}

// =============================================================================
// 9. EXPLICIT CONVERSION SUPPORT
// =============================================================================

/// Converts an integer into f64 using an explicitly lossy conversion.
///
/// This API intentionally advertises possible precision loss.
///
/// It must not be used as an implicit semantic conversion.
pub trait ToF64Lossy {
    /// Converts the value to f64.
    fn to_f64_lossy(&self) -> f64;
}

macro_rules! implement_to_f64 {
    ($($type:ty),+ $(,)?) => {
        $(
            impl ToF64Lossy for $type {
                fn to_f64_lossy(&self) -> f64 {
                    *self as f64
                }
            }
        )+
    };
}

implement_to_f64!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
);

impl ToF64Lossy for f32 {
    fn to_f64_lossy(&self) -> f64 {
        f64::from(*self)
    }
}

impl ToF64Lossy for f64 {
    fn to_f64_lossy(&self) -> f64 {
        *self
    }
}

// =============================================================================
// 10. TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_integer_addition_is_checked() {
        assert_eq!(add(&10_i32, &20_i32), Ok(30));

        assert!(matches!(
            add(&i32::MAX, &1),
            Err(NumericError::Overflow {
                operation: NumericOperation::Addition
            })
        ));
    }

    #[test]
    fn primitive_integer_subtraction_is_checked() {
        assert_eq!(subtract(&20_i32, &10_i32), Ok(10));

        assert!(matches!(
            subtract(&i32::MIN, &1),
            Err(NumericError::Overflow { .. })
        ));
    }

    #[test]
    fn primitive_integer_multiplication_is_checked() {
        assert_eq!(multiply(&6_i32, &7_i32), Ok(42));

        assert!(multiply(&i32::MAX, &2).is_err());
    }

    #[test]
    fn division_by_zero_is_explicit() {
        assert!(matches!(
            divide(&10_i32, &0),
            Err(NumericError::DivisionByZero { .. })
        ));
    }

    #[test]
    fn signed_division_overflow_is_detected() {
        assert!(matches!(
            divide(&i32::MIN, &-1),
            Err(NumericError::Overflow { .. })
        ));
    }

    #[test]
    fn unsigned_negation_is_explicit() {
        assert_eq!(negate(&0_u64), Ok(0));

        assert_eq!(
            negate(&5_u64),
            Err(NumericError::NegativeUnsigned)
        );
    }

    #[test]
    fn floating_point_rejects_non_finite_results() {
        assert!(matches!(
            multiply(&f64::MAX, &2.0),
            Err(NumericError::NonFiniteResult { .. })
        ));
    }

    #[test]
    fn floating_point_rejects_nan_operands() {
        assert_eq!(
            add(&f64::NAN, &1.0),
            Err(NumericError::NonFiniteOperand)
        );
    }

    #[test]
    fn floating_point_division_by_zero_is_rejected() {
        assert!(matches!(
            divide(&1.0_f64, &0.0),
            Err(NumericError::DivisionByZero { .. })
        ));
    }

    #[test]
    fn generic_sum_preserves_correct_result() {
        let values = [1_i64, 2, 3, 4, 5];

        assert_eq!(checked_sum(values), Ok(15));
    }

    #[test]
    fn generic_product_preserves_correct_result() {
        let values = [2_i64, 3, 4];

        assert_eq!(checked_product(values), Ok(24));
    }

    #[test]
    fn empty_sum_has_zero_identity() {
        let values: [i64; 0] = [];

        assert_eq!(checked_sum(values), Ok(0));
    }

    #[test]
    fn empty_product_has_one_identity() {
        let values: [i64; 0] = [];

        assert_eq!(checked_product(values), Ok(1));
    }

    #[test]
    fn dot_product_is_correct() {
        let left = [1_i64, 2, 3];
        let right = [4_i64, 5, 6];

        assert_eq!(
            checked_dot_product(left, right),
            Ok(32)
        );
    }

    #[test]
    fn dot_product_rejects_different_lengths() {
        let left = [1_i64, 2];
        let right = [3_i64];

        assert!(matches!(
            checked_dot_product(left, right),
            Err(NumericError::LengthMismatch { .. })
        ));
    }

    #[test]
    fn arbitrary_width_unsigned_integer_uses_canonical_authority() {
        let value = UnsignedInteger::from_be_bytes(&[
            0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        let one = UnsignedInteger::from_u8(1);

        let result = add(&value, &one).unwrap();

        assert!(result > value);
    }

    #[test]
    fn arbitrary_width_signed_integer_supports_checked_arithmetic() {
        let left = SignedInteger::from_i64(100);
        let right = SignedInteger::from_i64(50);

        let result = subtract(&left, &right).unwrap();

        assert_eq!(result.to_i64(), Ok(50));
    }

    #[test]
    fn canonical_integer_negation_works() {
        let value = SignedInteger::from_i64(42);

        let result = negate(&value).unwrap();

        assert_eq!(result.to_i64(), Ok(-42));
    }

    #[test]
    fn lossy_conversion_is_explicit() {
        let value = u128::MAX;

        let converted = value.to_f64_lossy();

        assert!(converted.is_finite());
    }
}
