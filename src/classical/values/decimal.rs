//! Zamani — Exact Classical Decimal Values
//!
//! File: src/classical/values/decimal.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: unsafe Rust forbidden.
//!
//! STATUS
//!   Standalone decimal-value implementation.
//!   Repository integration and compiler verification are pending.
//!
//! PURPOSE
//!   Represent finite base-10 decimal values exactly, without converting
//!   through f32 or f64 and without imposing a fixed coefficient-width limit.
//!
//! AUTHORITY
//!   grammar/spec/classical.md
//!   grammar/lexer/literals.md
//!   Canonical semantic/IR ownership must be reconciled with
//!   src/quantum/ir/classical/ before integration.
//!
//! REPRESENTATION
//!   value = sign * coefficient * 10^(-scale)
//!
//!   coefficient is a non-negative, arbitrary-length sequence of decimal
//!   digits stored most-significant digit first.
//!
//!   scale is signed metadata. Positive scale denotes fractional digits;
//!   negative scale denotes a positive power of ten.
//!
//! OWNERSHIP
//!   This module owns the exact decimal value representation and its
//!   value-level operations. It does not own source AST nodes, literal spans,
//!   semantic type identities, canonical IR values, or target representations.
//!
//! SCALABILITY
//!   No fixed coefficient-width or decimal-precision ceiling is imposed.
//!   Actual allocation and metadata limits are reported as errors.
//!
//! NOTE
//!   This implementation intentionally does not claim infinite resources.
//!   Exponent metadata uses i64; values whose scale cannot be represented by
//!   that metadata type are rejected explicitly. Replacing that metadata with
//!   an arbitrary-width exponent requires a separate representation migration.
//!
//! INTEGRATION
//!   - values/mod.rs should declare `pub mod decimal` and export the public API.
//!   - The canonical IR must adopt or explicitly adapt this representation.
//!   - Literal parsing must preserve original spelling and source spans in the
//!     AST, independently of the normalized decimal value.
//!   - Rounding and conversion to binary floating point must be explicit
//!     operations owned by a conversion policy.
//!
//! FREEZE RULE
//!   Do not add a second canonical decimal type elsewhere. Resolve canonical
//!   ownership and compatibility before exposing this type in public IR APIs.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::str::FromStr;

/// Errors produced by exact decimal construction and arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalError {
    /// The input is not a valid decimal literal.
    InvalidLiteral,

    /// The scale or exponent cannot be represented by the metadata type.
    ScaleOverflow,

    /// The result cannot be represented using the available metadata.
    ArithmeticOverflow,

    /// An allocation request failed.
    AllocationFailure,

    /// Division by zero was requested.
    DivisionByZero,

    /// The requested conversion is not exact.
    InexactConversion,

    /// The value is outside the requested conversion's range.
    ConversionOverflow,
}

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral =>
                f.write_str("invalid decimal literal"),
            Self::ScaleOverflow =>
                f.write_str("decimal scale or exponent overflow"),
            Self::ArithmeticOverflow =>
                f.write_str("decimal arithmetic metadata overflow"),
            Self::AllocationFailure =>
                f.write_str("insufficient resources to allocate decimal storage"),
            Self::DivisionByZero =>
                f.write_str("decimal division by zero"),
            Self::InexactConversion =>
                f.write_str("decimal conversion would lose precision"),
            Self::ConversionOverflow =>
                f.write_str("decimal value is outside the conversion range"),
        }
    }
}

impl std::error::Error for DecimalError {}

/// Exact finite base-10 decimal.
///
/// The coefficient contains digits from most significant to least significant.
/// Every digit is in `0..=9`.
///
/// Zero has a canonical positive sign, coefficient `[0]`, and scale `0`.
///
/// Nonzero values are normalized by removing coefficient trailing zeroes
/// while adjusting the scale. Thus `1.20` and `1.2` have the same numeric
/// identity. Source spelling belongs in the AST, not in this value.
#[derive(Clone, Debug)]
pub struct DecimalValue {
    negative: bool,
    digits: Vec<u8>,
    scale: i64,
}

impl DecimalValue {
    /// Creates exact decimal zero.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            negative: false,
            digits: vec![0],
            scale: 0,
        }
    }

    /// Parses a decimal value without using binary floating point.
    ///
    /// Accepted forms include:
    ///
    /// - `0`
    /// - `-12`
    /// - `+12.50`
    /// - `.125`
    /// - `12.`
    /// - `1.25e3`
    /// - `1.25E-3`
    /// - `1_000.25`
    ///
    /// Underscores are permitted only between decimal digits. A leading sign
    /// is accepted by this value parser for convenience; the source grammar
    /// may instead tokenize signs as separate operators.
    ///
    /// Exponents are stored through scale metadata. Parsing does not expand
    /// exponent notation into an arbitrarily large string of zeroes.
    pub fn parse(input: &str) -> Result<Self, DecimalError> {
        if input.is_empty() {
            return Err(DecimalError::InvalidLiteral);
        }

        let bytes = input.as_bytes();
        let mut index = 0;
        let mut negative = false;

        if bytes[index] == b'+' || bytes[index] == b'-' {
            negative = bytes[index] == b'-';
            index += 1;

            if index == bytes.len() {
                return Err(DecimalError::InvalidLiteral);
            }
        }

        let mut digits = Vec::new();
        let mut fractional_digits: i64 = 0;
        let mut saw_digit = false;
        let mut saw_point = false;
        let mut previous_was_digit = false;

        while index < bytes.len() {
            match bytes[index] {
                b'0'..=b'9' => {
                    digits
                        .try_reserve(1)
                        .map_err(|_| DecimalError::AllocationFailure)?;

                    digits.push(bytes[index] - b'0');
                    saw_digit = true;
                    previous_was_digit = true;

                    if saw_point {
                        fractional_digits = fractional_digits
                            .checked_add(1)
                            .ok_or(DecimalError::ScaleOverflow)?;
                    }

                    index += 1;
                }

                b'_' => {
                    let next_is_digit = bytes
                        .get(index + 1)
                        .is_some_and(u8::is_ascii_digit);

                    if !previous_was_digit || !next_is_digit {
                        return Err(DecimalError::InvalidLiteral);
                    }

                    previous_was_digit = false;
                    index += 1;
                }

                b'.' if !saw_point => {
                    saw_point = true;
                    previous_was_digit = false;
                    index += 1;
                }

                b'e' | b'E' => break,

                _ => return Err(DecimalError::InvalidLiteral),
            }
        }

        if !saw_digit {
            return Err(DecimalError::InvalidLiteral);
        }

        let mut exponent: i64 = 0;

        if index < bytes.len() {
            if bytes[index] != b'e' && bytes[index] != b'E' {
                return Err(DecimalError::InvalidLiteral);
            }

            index += 1;

            let exponent_negative = if index < bytes.len()
                && (bytes[index] == b'+' || bytes[index] == b'-')
            {
                let is_negative = bytes[index] == b'-';
                index += 1;
                is_negative
            } else {
                false
            };

            let start = index;
            let mut previous_digit = false;

            while index < bytes.len() {
                match bytes[index] {
                    b'0'..=b'9' => {
                        exponent = exponent
                            .checked_mul(10)
                            .and_then(|v| {
                                v.checked_add((bytes[index] - b'0') as i64)
                            })
                            .ok_or(DecimalError::ScaleOverflow)?;

                        previous_digit = true;
                        index += 1;
                    }

                    b'_' => {
                        let next_is_digit = bytes
                            .get(index + 1)
                            .is_some_and(u8::is_ascii_digit);

                        if !previous_digit || !next_is_digit {
                            return Err(DecimalError::InvalidLiteral);
                        }

                        previous_digit = false;
                        index += 1;
                    }

                    _ => return Err(DecimalError::InvalidLiteral),
                }
            }

            if index == start {
                return Err(DecimalError::InvalidLiteral);
            }

            if exponent_negative {
                exponent = exponent
                    .checked_neg()
                    .ok_or(DecimalError::ScaleOverflow)?;
            }
        }

        let scale = fractional_digits
            .checked_sub(exponent)
            .ok_or(DecimalError::ScaleOverflow)?;

        let mut value = Self {
            negative,
            digits,
            scale,
        };

        value.normalize();
        Ok(value)
    }

    /// Returns whether the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }

    /// Returns whether the value is negative and nonzero.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.negative && !self.is_zero()
    }

    /// Returns the signed decimal scale.
    #[must_use]
    pub const fn scale(&self) -> i64 {
        self.scale
    }

    /// Returns the normalized coefficient digits.
    ///
    /// Digits are in most-significant-first order and each digit is in `0..=9`.
    #[must_use]
    pub fn coefficient_digits(&self) -> &[u8] {
        &self.digits
    }

    /// Returns a new value with the opposite sign.
    #[must_use]
    pub fn negated(&self) -> Self {
        let mut result = self.clone();

        if !result.is_zero() {
            result.negative = !result.negative;
        }

        result
    }

    /// Returns the absolute value.
    #[must_use]
    pub fn abs(&self) -> Self {
        let mut result = self.clone();
        result.negative = false;
        result
    }

    /// Checks internal representation invariants.
    pub fn validate(&self) -> Result<(), DecimalError> {
        if self.digits.is_empty()
            || self.digits.iter().any(|digit| *digit > 9)
        {
            return Err(DecimalError::InvalidLiteral);
        }

        if self.digits.len() > 1 && self.digits[0] == 0 {
            return Err(DecimalError::InvalidLiteral);
        }

        if self.is_zero() {
            if self.negative || self.scale != 0 {
                return Err(DecimalError::InvalidLiteral);
            }
        } else if self.digits.last() == Some(&0) {
            return Err(DecimalError::InvalidLiteral);
        }

        Ok(())
    }

    /// Adds two exact decimal values.
    ///
    /// Alignment uses the greater scale. A very large scale difference can
    /// require substantial memory; allocation failure is reported rather
    /// than imposing an arbitrary decimal-precision ceiling.
    pub fn checked_add(&self, other: &Self) -> Result<Self, DecimalError> {
        let common_scale = self.scale.max(other.scale);
        let left_shift = common_scale
            .checked_sub(self.scale)
            .ok_or(DecimalError::ArithmeticOverflow)?;
        let right_shift = common_scale
            .checked_sub(other.scale)
            .ok_or(DecimalError::ArithmeticOverflow)?;

        let left = shifted_digits(&self.digits, left_shift)?;
        let right = shifted_digits(&other.digits, right_shift)?;

        let (negative, digits) = if self.negative == other.negative {
            (self.negative, add_digits(&left, &right)?)
        } else {
            match compare_digits(&left, &right) {
                Ordering::Equal => return Ok(Self::zero()),
                Ordering::Greater => (
                    self.negative,
                    subtract_digits(&left, &right)?,
                ),
                Ordering::Less => (
                    other.negative,
                    subtract_digits(&right, &left)?,
                ),
            }
        };

        let mut result = Self {
            negative,
            digits,
            scale: common_scale,
        };

        result.normalize();
        Ok(result)
    }

    /// Subtracts two exact decimal values.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, DecimalError> {
        self.checked_add(&other.negated())
    }

    /// Multiplies two exact decimal values.
    pub fn checked_mul(&self, other: &Self) -> Result<Self, DecimalError> {
        let scale = self.scale
            .checked_add(other.scale)
            .ok_or(DecimalError::ArithmeticOverflow)?;

        if self.is_zero() || other.is_zero() {
            return Ok(Self::zero());
        }

        let digits = multiply_digits(&self.digits, &other.digits)?;

        let mut result = Self {
            negative: self.negative != other.negative,
            digits,
            scale,
        };

        result.normalize();
        Ok(result)
    }

    /// Converts to `i128` only when the conversion is exact and representable.
    ///
    /// This is an interoperability helper, not the language's integer-width
    /// limit. Arbitrary-width integer conversion belongs in the integer
    /// subsystem and should not route through this method.
    pub fn to_i128_exact(&self) -> Result<i128, DecimalError> {
        let digits = if self.scale <= 0 {
            let extra = self.scale
                .checked_neg()
                .ok_or(DecimalError::ConversionOverflow)?;

            let extra = usize::try_from(extra)
                .map_err(|_| DecimalError::ConversionOverflow)?;

            let mut expanded = self.digits.clone();
            expanded
                .try_reserve(extra)
                .map_err(|_| DecimalError::AllocationFailure)?;
            expanded.resize(
                expanded.len()
                    .checked_add(extra)
                    .ok_or(DecimalError::ConversionOverflow)?,
                0,
            );
            expanded
        } else {
            let scale = usize::try_from(self.scale)
                .map_err(|_| DecimalError::ConversionOverflow)?;

            if scale > self.digits.len() {
                return if self.is_zero() {
                    Ok(0)
                } else {
                    Err(DecimalError::InexactConversion)
                };
            }

            let split = self.digits.len() - scale;

            if self.digits[split..].iter().any(|digit| *digit != 0) {
                return Err(DecimalError::InexactConversion);
            }

            self.digits[..split].to_vec()
        };

        let mut magnitude: u128 = 0;

        for digit in digits {
            magnitude = magnitude
                .checked_mul(10)
                .and_then(|value| value.checked_add(u128::from(digit)))
                .ok_or(DecimalError::ConversionOverflow)?;
        }

        if self.negative {
            if magnitude > (i128::MAX as u128) + 1 {
                return Err(DecimalError::ConversionOverflow);
            }

            if magnitude == (i128::MAX as u128) + 1 {
                Ok(i128::MIN)
            } else {
                Ok(-(magnitude as i128))
            }
        } else {
            i128::try_from(magnitude)
                .map_err(|_| DecimalError::ConversionOverflow)
        }
    }

    fn normalize(&mut self) {
        let first_nonzero = self
            .digits
            .iter()
            .position(|digit| *digit != 0);

        let Some(first_nonzero) = first_nonzero else {
            self.digits.clear();
            self.digits.push(0);
            self.negative = false;
            self.scale = 0;
            return;
        };

        if first_nonzero > 0 {
            self.digits.drain(..first_nonzero);
        }

        while self.digits.len() > 1 && self.digits.last() == Some(&0) {
            // A value with trailing coefficient zeroes can be normalized by
            // decreasing its scale. Avoid silent metadata overflow.
            match self.scale.checked_sub(1) {
                Some(scale) => {
                    self.digits.pop();
                    self.scale = scale;
                }
                None => break,
            }
        }
    }
}

impl FromStr for DecimalValue {
    type Err = DecimalError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

impl PartialEq for DecimalValue {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative
            && self.scale == other.scale
            && self.digits == other.digits
    }
}

impl Eq for DecimalValue {}

impl Hash for DecimalValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.negative.hash(state);
        self.scale.hash(state);
        self.digits.hash(state);
    }
}

impl PartialOrd for DecimalValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DecimalValue {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.is_negative(), other.is_negative()) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }

        let magnitude = compare_magnitude(self, other);

        if self.is_negative() {
            magnitude.reverse()
        } else {
            magnitude
        }
    }
}

impl fmt::Display for DecimalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            f.write_str("-")?;
        }

        if self.scale <= 0 {
            f.write_str(&digits_to_string(&self.digits))?;

            let zeros = self.scale
                .checked_neg()
                .ok_or(fmt::Error)?;

            for _ in 0..zeros {
                f.write_str("0")?;
            }

            return Ok(());
        }

        let scale = usize::try_from(self.scale).map_err(|_| fmt::Error)?;

        if scale >= self.digits.len() {
            f.write_str("0.")?;

            for _ in 0..(scale - self.digits.len()) {
                f.write_str("0")?;
            }

            f.write_str(&digits_to_string(&self.digits))
        } else {
            let split = self.digits.len() - scale;

            f.write_str(&digits_to_string(&self.digits[..split]))?;
            f.write_str(".")?;
            f.write_str(&digits_to_string(&self.digits[split..]))
        }
    }
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect()
}

fn shifted_digits(digits: &[u8], shift: i64) -> Result<Vec<u8>, DecimalError> {
    let shift = usize::try_from(shift)
        .map_err(|_| DecimalError::ArithmeticOverflow)?;

    let capacity = digits.len()
        .checked_add(shift)
        .ok_or(DecimalError::ArithmeticOverflow)?;

    let mut result = Vec::new();
    result
        .try_reserve(capacity)
        .map_err(|_| DecimalError::AllocationFailure)?;

    result.extend_from_slice(digits);
    result.resize(capacity, 0);

    Ok(result)
}

fn compare_digits(left: &[u8], right: &[u8]) -> Ordering {
    match left.len().cmp(&right.len()) {
        Ordering::Equal => left.cmp(right),
        ordering => ordering,
    }
}

fn add_digits(left: &[u8], right: &[u8]) -> Result<Vec<u8>, DecimalError> {
    let capacity = left.len()
        .max(right.len())
        .checked_add(1)
        .ok_or(DecimalError::ArithmeticOverflow)?;

    let mut reversed = Vec::new();
    reversed
        .try_reserve(capacity)
        .map_err(|_| DecimalError::AllocationFailure)?;

    let mut carry = 0u8;
    let mut li = left.len();
    let mut ri = right.len();

    while li > 0 || ri > 0 || carry > 0 {
        let a = if li > 0 {
            li -= 1;
            left[li]
        } else {
            0
        };

        let b = if ri > 0 {
            ri -= 1;
            right[ri]
        } else {
            0
        };

        let sum = a + b + carry;
        reversed.push(sum % 10);
        carry = sum / 10;
    }

    reversed.reverse();
    Ok(reversed)
}

/// Subtracts `right` from `left`; the caller must establish left >= right.
fn subtract_digits(
    left: &[u8],
    right: &[u8],
) -> Result<Vec<u8>, DecimalError> {
    if compare_digits(left, right) == Ordering::Less {
        return Err(DecimalError::ArithmeticOverflow);
    }

    let mut reversed = Vec::new();
    reversed
        .try_reserve(left.len())
        .map_err(|_| DecimalError::AllocationFailure)?;

    let mut borrow = 0i16;
    let mut li = left.len();
    let mut ri = right.len();

    while li > 0 {
        li -= 1;

        let a = i16::from(left[li]);
        let b = if ri > 0 {
            ri -= 1;
            i16::from(right[ri])
        } else {
            0
        };

        let mut difference = a - b - borrow;

        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        reversed.push(difference as u8);
    }

    if borrow != 0 {
        return Err(DecimalError::ArithmeticOverflow);
    }

    while reversed.len() > 1 && reversed.last() == Some(&0) {
        reversed.pop();
    }

    reversed.reverse();
    Ok(reversed)
}

fn multiply_digits(
    left: &[u8],
    right: &[u8],
) -> Result<Vec<u8>, DecimalError> {
    if (left.len() == 1 && left[0] == 0)
        || (right.len() == 1 && right[0] == 0)
    {
        return Ok(vec![0]);
    }

    let length = left.len()
        .checked_add(right.len())
        .ok_or(DecimalError::ArithmeticOverflow)?;

    let mut result = Vec::new();
    result
        .try_reserve(length)
        .map_err(|_| DecimalError::AllocationFailure)?;
    result.resize(length, 0u8);

    for (li, &a) in left.iter().rev().enumerate() {
        let mut carry = 0u16;

        for (ri, &b) in right.iter().rev().enumerate() {
            let index = li + ri;
            let product = u16::from(a) * u16::from(b)
                + u16::from(result[index])
                + carry;

            result[index] = (product % 10) as u8;
            carry = product / 10;
        }

        let mut index = li + right.len();

        while carry > 0 {
            if index >= result.len() {
                return Err(DecimalError::ArithmeticOverflow);
            }

            let sum = u16::from(result[index]) + carry;
            result[index] = (sum % 10) as u8;
            carry = sum / 10;
            index += 1;
        }
    }

    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }

    result.reverse();
    Ok(result)
}

/// Compares magnitudes without expanding either coefficient.
fn compare_magnitude(left: &DecimalValue, right: &DecimalValue) -> Ordering {
    // Number of integer-position digits, represented as signed metadata.
    let left_order = (left.digits.len() as i128) - i128::from(left.scale);
    let right_order = (right.digits.len() as i128) - i128::from(right.scale);

    match left_order.cmp(&right_order) {
        Ordering::Equal => {
            // Compare significant digit streams, treating missing digits as
            // trailing zeroes. No giant temporary coefficient is allocated.
            let left_scale = i128::from(left.scale);
            let right_scale = i128::from(right.scale);
            let common_scale = left_scale.max(right_scale);

            let left_shift = (common_scale - left_scale) as usize;
            let right_shift = (common_scale - right_scale) as usize;

            let left_len = left.digits.len() + left_shift;
            let right_len = right.digits.len() + right_shift;

            for index in 0..left_len.max(right_len) {
                let a = if index < left_len {
                    if index < left.digits.len() {
                        left.digits[index]
                    } else {
                        0
                    }
                } else {
                    0
                };

                let b = if index < right_len {
                    if index < right.digits.len() {
                        right.digits[index]
                    } else {
                        0
                    }
                } else {
                    0
                };

                match a.cmp(&b) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
            }

            Ordering::Equal
        }
        ordering => ordering,
    }
}

#[cfg(test)]
mod tests {
    use super::{DecimalError, DecimalValue};
    use core::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use core::hash::{Hash, Hasher};

    fn decimal(input: &str) -> DecimalValue {
        DecimalValue::parse(input).expect("valid decimal")
    }

    fn hash(value: &DecimalValue) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn parses_exact_decimal_values() {
        assert_eq!(decimal("0.1").to_string(), "0.1");
        assert_eq!(decimal("12345678901234567890").to_string(),
                   "12345678901234567890");
        assert_eq!(decimal("1.25e3").to_string(), "1250");
        assert_eq!(decimal("1.25e-3").to_string(), "0.00125");
    }

    #[test]
    fn normalizes_equivalent_values() {
        let a = decimal("1.20");
        let b = decimal("1.2");

        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
        assert_eq!(a.to_string(), "1.2");
    }

    #[test]
    fn parses_digit_separators() {
        assert_eq!(decimal("1_000.25").to_string(), "1000.25");
        assert_eq!(decimal("1_2_3").to_string(), "123");
    }

    #[test]
    fn rejects_malformed_literals() {
        for input in [
            "", ".", "+", "-", "_1", "1_", "1__2", "1._2", "1e", "1e+",
            "1e_", "1e2x", "1..2",
        ] {
            assert_eq!(
                DecimalValue::parse(input),
                Err(DecimalError::InvalidLiteral),
                "input: {input}"
            );
        }
    }

    #[test]
    fn exact_addition_and_subtraction() {
        assert_eq!(
            decimal("0.1").checked_add(&decimal("0.2")).unwrap().to_string(),
            "0.3"
        );

        assert_eq!(
            decimal("1000").checked_sub(&decimal("0.001")).unwrap().to_string(),
            "999.999"
        );

        assert_eq!(
            decimal("2.5").checked_sub(&decimal("2.5")).unwrap(),
            DecimalValue::zero()
        );
    }

    #[test]
    fn exact_multiplication() {
        assert_eq!(
            decimal("0.1").checked_mul(&decimal("0.2")).unwrap().to_string(),
            "0.02"
        );

        assert_eq!(
            decimal("-12.5").checked_mul(&decimal("4")).unwrap().to_string(),
            "-50"
        );
    }

    #[test]
    fn numeric_ordering_handles_signs_and_scales() {
        assert_eq!(decimal("-10").cmp(&decimal("-2")), Ordering::Less);
        assert_eq!(decimal("-0.1").cmp(&decimal("0")), Ordering::Less);
        assert_eq!(decimal("1.20").cmp(&decimal("1.2")), Ordering::Equal);
        assert_eq!(decimal("1000").cmp(&decimal("999.99")), Ordering::Greater);
    }

    #[test]
    fn exact_integer_conversion() {
        assert_eq!(decimal("123").to_i128_exact(), Ok(123));
        assert_eq!(decimal("-123").to_i128_exact(), Ok(-123));
        assert_eq!(decimal("1.0").to_i128_exact(), Ok(1));
        assert_eq!(
            decimal("1.25").to_i128_exact(),
            Err(DecimalError::InexactConversion)
        );
    }

    #[test]
    fn zero_is_canonical() {
        assert_eq!(decimal("-0.000").to_string(), "0");
        assert_eq!(decimal("0e20"), DecimalValue::zero());
        assert!(!decimal("-0").is_negative());
    }

    #[test]
    fn representation_invariants_hold() {
        for input in ["0", "-0.0", "1.20", "-12.345", "1e20", "1e-20"] {
            assert!(decimal(input).validate().is_ok(), "{input}");
        }
    }
}