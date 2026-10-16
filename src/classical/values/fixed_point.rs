//! Zamani — Exact Fixed-Point Values
//!
//! Path: src/classical/values/fixed_point.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: no unsafe Rust.
//!
//! CONTRACT
//!
//! PURPOSE
//!   Represent fixed-point numbers exactly as an arbitrary-width signed
//!   integer coefficient and a decimal fractional scale.
//!
//! REPRESENTATION
//!   value = coefficient / 10^fractional_digits
//!
//! OWNERSHIP
//!   This file owns fixed-point representation, scale conversion, rounding,
//!   checked arithmetic, comparison, and fixed-point formatting.
//!
//!   It does not own source syntax, AST nodes, semantic type identities,
//!   canonical IR values, target-specific layouts, or runtime execution.
//!
//! AUTHORITY
//!   - grammar/spec/classical.md
//!   - grammar/lexer/literals.md
//!   - src/classical/values/integer.rs
//!   - src/classical/values/decimal.rs
//!
//! INTEGRATION
//!   - Register `pub mod fixed_point;` in src/classical/values/mod.rs.
//!   - Re-export FixedPointValue, FixedPointError, and RoundingMode.
//!   - Keep arbitrary-width integer ownership in
//!     quantum::ir::classical::integer.
//!   - Keep decimal literal spelling and source spans in the frontend.
//!   - Integrate into canonical IR only through an explicit, reviewed adapter.
//!
//! SCALABILITY
//!   No fixed coefficient bit-width or decimal precision ceiling is imposed.
//!   Fractional scale metadata uses u64; scale arithmetic is checked.
//!   Operations can require memory proportional to the resulting value.
//!   Resource exhaustion must not be confused with numeric overflow.
//!
//! SEMANTICS
//!   Scale is part of the fixed-point representation. Arithmetic preserves
//!   exactness where possible. Any operation that discards fractional digits
//!   requires an explicit rounding mode.
//!
//! FREEZE RULE
//!   Do not change coefficient ownership, scale semantics, rounding rules,
//!   or public error behavior without reviewing all downstream consumers.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use crate::classical::values::decimal::{DecimalError, DecimalValue};
use crate::quantum::ir::classical::integer::{
    IntegerError,
    SignedInteger,
    UnsignedInteger,
};

/// The rounding rule used when reducing fixed-point scale.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoundingMode {
    /// Reject a conversion if any discarded digit is non-zero.
    Exact,

    /// Discard fractional digits toward zero.
    TowardZero,

    /// Round away from zero whenever discarded digits are non-zero.
    AwayFromZero,

    /// Round toward positive infinity.
    TowardPositiveInfinity,

    /// Round toward negative infinity.
    TowardNegativeInfinity,

    /// Round to nearest; ties go away from zero.
    HalfAwayFromZero,

    /// Round to nearest; ties go to an even retained coefficient.
    HalfEven,
}

/// Errors from fixed-point parsing, conversion, or arithmetic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixedPointError {
    /// Invalid decimal input.
    InvalidLiteral,

    /// The requested scale cannot be represented by the scale metadata.
    ScaleOverflow,

    /// The operation would require an unsupported scale transformation.
    ScaleConversion,

    /// An exact conversion was requested but discarded digits are non-zero.
    InexactConversion,

    /// Division by zero.
    DivisionByZero,

    /// The underlying arbitrary-width integer operation failed.
    Integer(IntegerError),

    /// The underlying decimal parser failed.
    Decimal(DecimalError),
}

impl fmt::Display for FixedPointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral => f.write_str("invalid fixed-point value"),
            Self::ScaleOverflow => f.write_str("fixed-point scale overflow"),
            Self::ScaleConversion => {
                f.write_str("unsupported fixed-point scale conversion")
            }
            Self::InexactConversion => {
                f.write_str("fixed-point conversion would lose precision")
            }
            Self::DivisionByZero => f.write_str("fixed-point division by zero"),
            Self::Integer(error) => {
                write!(f, "fixed-point integer operation failed: {error}")
            }
            Self::Decimal(error) => {
                write!(f, "fixed-point decimal parsing failed: {error}")
            }
        }
    }
}

impl std::error::Error for FixedPointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            Self::Decimal(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IntegerError> for FixedPointError {
    fn from(value: IntegerError) -> Self {
        Self::Integer(value)
    }
}

impl From<DecimalError> for FixedPointError {
    fn from(value: DecimalError) -> Self {
        Self::Decimal(value)
    }
}

/// An exact fixed-point value.
///
/// The represented value is:
///
/// `coefficient / 10^fractional_digits`.
///
/// The coefficient is an arbitrary-width signed integer. The scale is the
/// number of decimal fractional places in this representation.
///
/// For example:
///
/// - coefficient `12345`, scale `2` represents `123.45`;
/// - coefficient `-7`, scale `3` represents `-0.007`;
/// - coefficient `120`, scale `2` represents `1.20`.
///
/// The scale is preserved by construction and by addition/subtraction.
/// Multiplication adds the operand scales. Division requires the caller to
/// specify the result scale and rounding policy.
///
/// Equality of this type is representational: equal coefficients with equal
/// scales are equal. Use `checked_cmp` for exact numeric comparison across
/// different scales.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedPointValue {
    coefficient: SignedInteger,
    fractional_digits: u64,
}

impl FixedPointValue {
    /// Constructs a fixed-point value from its coefficient and scale.
    #[must_use]
    pub fn new(coefficient: SignedInteger, fractional_digits: u64) -> Self {
        Self {
            coefficient,
            fractional_digits,
        }
    }

    /// Constructs zero with the requested scale.
    #[must_use]
    pub fn zero(fractional_digits: u64) -> Self {
        Self::new(SignedInteger::zero(), fractional_digits)
    }

    /// Constructs one with the requested scale.
    ///
    /// The coefficient is `10^fractional_digits`.
    pub fn one(fractional_digits: u64) -> Result<Self, FixedPointError> {
        let coefficient = power_of_ten(fractional_digits)?;
        let coefficient = SignedInteger::from_parts(false, coefficient)?;
        Ok(Self::new(coefficient, fractional_digits))
    }

    /// Returns the arbitrary-width coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &SignedInteger {
        &self.coefficient
    }

    /// Returns the number of decimal fractional places.
    #[must_use]
    pub const fn fractional_digits(&self) -> u64 {
        self.fractional_digits
    }

    /// Returns whether the represented value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.coefficient.is_zero()
    }

    /// Returns whether the represented value is negative.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.coefficient.is_negative()
    }

    /// Returns the additive inverse.
    #[must_use]
    pub fn negated(&self) -> Self {
        Self::new(
            self.coefficient.negated(),
            self.fractional_digits,
        )
    }

    /// Returns the absolute value.
    #[must_use]
    pub fn abs(&self) -> Self {
        let (negative, magnitude) = self.coefficient.parts();

        let coefficient = if negative {
            SignedInteger::from_parts(false, magnitude.clone())
                .unwrap_or_else(|_| SignedInteger::zero())
        } else {
            self.coefficient.clone()
        };

        Self::new(coefficient, self.fractional_digits)
    }

    /// Parses a decimal value and converts it to the requested fixed scale.
    ///
    /// The input is interpreted by the existing exact DecimalValue parser.
    /// No binary floating-point conversion is performed.
    ///
    /// When the decimal contains more fractional digits than requested,
    /// `rounding` determines whether conversion is rejected or rounded.
    pub fn parse(
        input: &str,
        fractional_digits: u64,
        rounding: RoundingMode,
    ) -> Result<Self, FixedPointError> {
        let decimal = DecimalValue::parse(input)?;
        Self::from_decimal(&decimal, fractional_digits, rounding)
    }

    /// Converts an exact decimal value to a fixed-point scale.
    pub fn from_decimal(
        decimal: &DecimalValue,
        fractional_digits: u64,
        rounding: RoundingMode,
    ) -> Result<Self, FixedPointError> {
        decimal.validate()?;

        if decimal.is_zero() {
            return Ok(Self::zero(fractional_digits));
        }

        let mut magnitude = decimal_digits_to_integer(
            decimal.coefficient_digits(),
        )?;

        let negative = decimal.is_negative();
        let decimal_scale = decimal.scale();

        if decimal_scale <= 0 {
            let integer_digits = decimal_scale
                .checked_neg()
                .ok_or(FixedPointError::ScaleOverflow)?;

            let exponent = fractional_digits
                .checked_add(
                    u64::try_from(integer_digits)
                        .map_err(|_| FixedPointError::ScaleOverflow)?,
                )
                .ok_or(FixedPointError::ScaleOverflow)?;

            magnitude = magnitude.checked_mul(&power_of_ten(exponent)?)?;
        } else {
            let decimal_scale = u64::try_from(decimal_scale)
                .map_err(|_| FixedPointError::ScaleOverflow)?;

            if decimal_scale <= fractional_digits {
                let exponent = fractional_digits - decimal_scale;
                magnitude = magnitude.checked_mul(&power_of_ten(exponent)?)?;
            } else {
                let discarded_digits = decimal_scale - fractional_digits;
                magnitude = divide_by_power_of_ten(
                    &magnitude,
                    discarded_digits,
                    negative,
                    rounding,
                )?;
            }
        }

        let coefficient = SignedInteger::from_parts(negative, magnitude)?;
        Ok(Self::new(coefficient, fractional_digits))
    }

    /// Rescales this value to another number of fractional digits.
    ///
    /// Increasing scale preserves the exact value. Reducing scale may require
    /// rounding and therefore always requires an explicit RoundingMode.
    pub fn rescale(
        &self,
        target_scale: u64,
        rounding: RoundingMode,
    ) -> Result<Self, FixedPointError> {
        if target_scale == self.fractional_digits {
            return Ok(self.clone());
        }

        let (negative, magnitude) = self.coefficient.parts();

        if magnitude.is_zero() {
            return Ok(Self::zero(target_scale));
        }

        let converted = if target_scale > self.fractional_digits {
            let delta = target_scale - self.fractional_digits;
            magnitude.checked_mul(&power_of_ten(delta)?)?
        } else {
            let delta = self.fractional_digits - target_scale;
            divide_by_power_of_ten(
                magnitude,
                delta,
                negative,
                rounding,
            )?
        };

        Ok(Self::new(
            SignedInteger::from_parts(negative, converted)?,
            target_scale,
        ))
    }

    /// Adds two values exactly, preserving the larger operand scale.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, FixedPointError> {
        let target_scale = self.fractional_digits.max(rhs.fractional_digits);
        let left = self.rescale(target_scale, RoundingMode::Exact)?;
        let right = rhs.rescale(target_scale, RoundingMode::Exact)?;

        Ok(Self::new(
            left.coefficient.checked_add(&right.coefficient)?,
            target_scale,
        ))
    }

    /// Subtracts two values exactly, preserving the larger operand scale.
    pub fn checked_sub(&self, rhs: &Self) -> Result<Self, FixedPointError> {
        self.checked_add(&rhs.negated())
    }

    /// Multiplies two values exactly.
    ///
    /// The result scale is the sum of operand scales. No rounding occurs.
    pub fn checked_mul(&self, rhs: &Self) -> Result<Self, FixedPointError> {
        let result_scale = self
            .fractional_digits
            .checked_add(rhs.fractional_digits)
            .ok_or(FixedPointError::ScaleOverflow)?;

        Ok(Self::new(
            self.coefficient.checked_mul(&rhs.coefficient)?,
            result_scale,
        ))
    }

    /// Divides by `rhs`, producing the requested fractional scale.
    ///
    /// The result is rounded according to the explicitly supplied policy.
    /// Division by zero is rejected.
    pub fn checked_div(
        &self,
        rhs: &Self,
        result_scale: u64,
        rounding: RoundingMode,
    ) -> Result<Self, FixedPointError> {
        if rhs.is_zero() {
            return Err(FixedPointError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok(Self::zero(result_scale));
        }

        let (left_negative, left_magnitude) = self.coefficient.parts();
        let (right_negative, right_magnitude) = rhs.coefficient.parts();

        // Desired coefficient:
        // (left / 10^left_scale) / (right / 10^right_scale)
        //     * 10^result_scale
        //
        // = left * 10^(right_scale + result_scale)
        //     / (right * 10^left_scale)
        //
        // Cancel common scale factors before creating large intermediates.
        let numerator_exponent = rhs
            .fractional_digits
            .checked_add(result_scale)
            .ok_or(FixedPointError::ScaleOverflow)?;

        let denominator_exponent = self.fractional_digits;

        let mut numerator = left_magnitude.clone();
        let mut denominator = right_magnitude.clone();

        if numerator_exponent >= denominator_exponent {
            numerator = numerator.checked_mul(
                &power_of_ten(numerator_exponent - denominator_exponent)?,
            )?;
        } else {
            denominator = denominator.checked_mul(
                &power_of_ten(denominator_exponent - numerator_exponent)?,
            )?;
        }

        let (mut quotient, remainder) = numerator.checked_div_rem(&denominator)?;
        let negative = left_negative ^ right_negative;

        if !remainder.is_zero() {
            match rounding {
                RoundingMode::Exact => {
                    return Err(FixedPointError::InexactConversion);
                }
                _ => {
                    let should_increment = should_round_up(
                        &quotient,
                        &remainder,
                        &denominator,
                        negative,
                        rounding,
                    )?;

                    if should_increment {
                        quotient = quotient.checked_add(&UnsignedInteger::from_u8(1))?;
                    }
                }
            }
        }

        Ok(Self::new(
            SignedInteger::from_parts(negative, quotient)?,
            result_scale,
        ))
    }

    /// Compares numeric values exactly, including values with different scales.
    pub fn checked_cmp(
        &self,
        rhs: &Self,
    ) -> Result<Ordering, FixedPointError> {
        let target_scale = self.fractional_digits.max(rhs.fractional_digits);
        let left = self.rescale(target_scale, RoundingMode::Exact)?;
        let right = rhs.rescale(target_scale, RoundingMode::Exact)?;

        Ok(left.coefficient.cmp(&right.coefficient))
    }

    /// Validates the coefficient representation.
    pub fn validate(&self) -> Result<(), FixedPointError> {
        self.coefficient.validate()?;
        Ok(())
    }
}

impl FromStr for FixedPointValue {
    type Err = FixedPointError;

    /// Parses a decimal integer/fraction at scale zero.
    ///
    /// Use `FixedPointValue::parse` when a non-zero target scale is required.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input, 0, RoundingMode::Exact)
    }
}

impl fmt::Display for FixedPointValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coefficient_text = self.coefficient.to_string();
        let negative = coefficient_text.starts_with('-');
        let digits = if negative {
            &coefficient_text[1..]
        } else {
            coefficient_text.as_str()
        };

        if negative {
            f.write_str("-")?;
        }

        if self.fractional_digits == 0 {
            return f.write_str(digits);
        }

        let digit_count = u64::try_from(digits.len())
            .map_err(|_| fmt::Error)?;

        if digit_count <= self.fractional_digits {
            f.write_str("0.")?;

            let leading_zeroes = self.fractional_digits - digit_count;

            for _ in 0..leading_zeroes {
                f.write_str("0")?;
            }

            return f.write_str(digits);
        }

        let split = usize::try_from(digit_count - self.fractional_digits)
            .map_err(|_| fmt::Error)?;

        f.write_str(&digits[..split])?;
        f.write_str(".")?;
        f.write_str(&digits[split..])
    }
}

fn decimal_digits_to_integer(
    digits: &[u8],
) -> Result<UnsignedInteger, FixedPointError> {
    if digits.is_empty() || digits.iter().any(|digit| *digit > 9) {
        return Err(FixedPointError::InvalidLiteral);
    }

    let ten = UnsignedInteger::from_u8(10);
    let mut value = UnsignedInteger::zero();

    for digit in digits {
        value = value.checked_mul(&ten)?;
        value = value.checked_add(&UnsignedInteger::from_u8(*digit))?;
    }

    Ok(value)
}

fn power_of_ten(exponent: u64) -> Result<UnsignedInteger, FixedPointError> {
    // Exponentiation by squaring avoids an O(exponent) number of
    // multiplication steps. The resulting integer still requires storage
    // proportional to the exponent.
    let mut exponent = exponent;
    let mut base = UnsignedInteger::from_u8(10);
    let mut result = UnsignedInteger::from_u8(1);

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(&base)?;
        }

        exponent >>= 1;

        if exponent > 0 {
            base = base.checked_mul(&base)?;
        }
    }

    Ok(result)
}

fn divide_by_power_of_ten(
    magnitude: &UnsignedInteger,
    exponent: u64,
    negative: bool,
    rounding: RoundingMode,
) -> Result<UnsignedInteger, FixedPointError> {
    let divisor = power_of_ten(exponent)?;
    let (mut quotient, remainder) = magnitude.checked_div_rem(&divisor)?;

    if remainder.is_zero() {
        return Ok(quotient);
    }

    if rounding == RoundingMode::Exact {
        return Err(FixedPointError::InexactConversion);
    }

    if should_round_up(
        &quotient,
        &remainder,
        &divisor,
        negative,
        rounding,
    )? {
        quotient = quotient.checked_add(&UnsignedInteger::from_u8(1))?;
    }

    Ok(quotient)
}

fn should_round_up(
    quotient: &UnsignedInteger,
    remainder: &UnsignedInteger,
    divisor: &UnsignedInteger,
    negative: bool,
    rounding: RoundingMode,
) -> Result<bool, FixedPointError> {
    if remainder.is_zero() {
        return Ok(false);
    }

    match rounding {
        RoundingMode::Exact => Err(FixedPointError::InexactConversion),

        RoundingMode::TowardZero => Ok(false),

        RoundingMode::AwayFromZero => Ok(true),

        RoundingMode::TowardPositiveInfinity => Ok(!negative),

        RoundingMode::TowardNegativeInfinity => Ok(negative),

        RoundingMode::HalfAwayFromZero | RoundingMode::HalfEven => {
            let twice_remainder =
                remainder.checked_mul(&UnsignedInteger::from_u8(2))?;

            match twice_remainder.cmp(divisor) {
                Ordering::Greater => Ok(true),
                Ordering::Less => Ok(false),
                Ordering::Equal => match rounding {
                    RoundingMode::HalfAwayFromZero => Ok(true),
                    RoundingMode::HalfEven => {
                        let (quotient, remainder) = quotient
                            .checked_div_rem(&UnsignedInteger::from_u8(2))?;

                        let _ = quotient;
                        Ok(!remainder.is_zero())
                    }
                    _ => Ok(false),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FixedPointError,
        FixedPointValue,
        RoundingMode,
    };
    use core::cmp::Ordering;

    fn fp(input: &str, scale: u64) -> FixedPointValue {
        FixedPointValue::parse(input, scale, RoundingMode::Exact)
            .expect("fixed-point test input should convert exactly")
    }

    #[test]
    fn parses_and_formats_exact_values() {
        assert_eq!(fp("123.45", 2).to_string(), "123.45");
        assert_eq!(fp("-0.007", 3).to_string(), "-0.007");
        assert_eq!(fp("12", 3).to_string(), "12.000");
        assert_eq!(fp("0.5", 4).to_string(), "0.5000");
        assert_eq!(fp("0", 3).to_string(), "0.000");
    }

    #[test]
    fn rejects_inexact_scale_reduction_in_exact_mode() {
        assert_eq!(
            FixedPointValue::parse("1.239", 2, RoundingMode::Exact),
            Err(FixedPointError::InexactConversion),
        );
    }

    #[test]
    fn supports_explicit_rounding_modes() {
        assert_eq!(
            FixedPointValue::parse("1.239", 2, RoundingMode::HalfEven)
                .unwrap()
                .to_string(),
            "1.24",
        );

        assert_eq!(
            FixedPointValue::parse("1.225", 2, RoundingMode::HalfEven)
                .unwrap()
                .to_string(),
            "1.22",
        );

        assert_eq!(
            FixedPointValue::parse("1.225", 2, RoundingMode::HalfAwayFromZero)
                .unwrap()
                .to_string(),
            "1.23",
        );

        assert_eq!(
            FixedPointValue::parse("-1.231", 2, RoundingMode::TowardPositiveInfinity)
                .unwrap()
                .to_string(),
            "-1.23",
        );

        assert_eq!(
            FixedPointValue::parse("-1.231", 2, RoundingMode::TowardNegativeInfinity)
                .unwrap()
                .to_string(),
            "-1.24",
        );
    }

    #[test]
    fn addition_and_subtraction_align_scales() {
        assert_eq!(
            fp("1.2", 1)
                .checked_add(&fp("0.03", 2))
                .unwrap()
                .to_string(),
            "1.23",
        );

        assert_eq!(
            fp("1.20", 2)
                .checked_sub(&fp("0.03", 2))
                .unwrap()
                .to_string(),
            "1.17",
        );
    }

    #[test]
    fn multiplication_preserves_exact_scale() {
        let result = fp("1.25", 2)
            .checked_mul(&fp("2.4", 1))
            .unwrap();

        assert_eq!(result.to_string(), "3.000");
        assert_eq!(result.fractional_digits(), 3);
    }

    #[test]
    fn division_uses_explicit_scale_and_rounding() {
        let result = fp("1", 0)
            .checked_div(&fp("3", 0), 4, RoundingMode::HalfEven)
            .unwrap();

        assert_eq!(result.to_string(), "0.3333");

        let rounded = fp("1", 0)
            .checked_div(&fp("8", 0), 2, RoundingMode::HalfAwayFromZero)
            .unwrap();

        assert_eq!(rounded.to_string(), "0.13");
    }

    #[test]
    fn division_by_zero_is_rejected() {
        assert_eq!(
            fp("1", 0).checked_div(
                &fp("0", 0),
                3,
                RoundingMode::HalfEven,
            ),
            Err(FixedPointError::DivisionByZero),
        );
    }

    #[test]
    fn comparison_is_exact_across_scales() {
        assert_eq!(
            fp("1.2", 1).checked_cmp(&fp("1.20", 2)).unwrap(),
            Ordering::Equal,
        );

        assert_eq!(
            fp("-1.21", 2).checked_cmp(&fp("-1.2", 1)).unwrap(),
            Ordering::Less,
        );

        assert_eq!(
            fp("0.99", 2).checked_cmp(&fp("1.00", 2)).unwrap(),
            Ordering::Less,
        );
    }

    #[test]
    fn accepts_large_coefficients() {
        let input =
            "12345678901234567890123456789012345678901234567890.12345";

        let value = fp(input, 5);

        assert_eq!(value.to_string(), input);
        assert!(value.validate().is_ok());
    }

    #[test]
    fn validates_representation() {
        assert!(fp("0", 0).validate().is_ok());
        assert!(fp("-123.45", 2).validate().is_ok());
        assert!(fp("1", 100).validate().is_ok());
    }
}