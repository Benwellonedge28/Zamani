//! Zamani — Exact Rational Values
//!
//! Path: src/classical/values/rational.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!   Represent exact rational numbers as a signed arbitrary-width numerator
//!   divided by a positive arbitrary-width denominator.
//!
//! AUTHORITY
//!   - grammar/spec/classical.md
//!   - grammar/lexer/literals.md
//!   - Canonical integer semantics:
//!     crate::quantum::ir::classical::integer
//!
//! OWNERSHIP
//!   This file owns rational-value invariants and rational arithmetic.
//!   It does not own source AST nodes, literal tokenization, type inference,
//!   canonical IR containers, target-specific numeric formats, or execution.
//!
//! REPRESENTATION
//!   value = numerator / denominator
//!
//!   - Numerator uses the existing canonical SignedInteger.
//!   - Denominator uses the existing canonical UnsignedInteger.
//!   - Denominator is always strictly positive.
//!   - Numerator and denominator are reduced by their greatest common divisor.
//!   - Zero is always 0/1.
//!
//! SCALABILITY
//!   This module introduces no fixed numerator/denominator width or precision
//!   limit. It delegates integer storage and arithmetic to Zamani's existing
//!   arbitrary-width integer subsystem. Operations remain subject to available
//!   resources and to the implementation limits of that subsystem.
//!
//! SYNTAX BOUNDARY
//!   `parse("numerator/denominator")` is a value-level helper. It does not
//!   declare slash notation to be a new source-level literal. The lexer and
//!   parser remain governed by the normative grammar and preserve source text.
//!
//! INTEGRATION
//!   - Declare this module in src/classical/values/mod.rs.
//!   - Re-export RationalValue and RationalError there.
//!   - Do not create a duplicate rational representation in the canonical IR.
//!   - Frontend lowering may adapt this value into the existing canonical
//!     classical semantic/IR representation through an explicit adapter.
//!   - Any canonical IR extension requires its own compatibility review.
//!
//! FREEZE RULE
//!   Keep canonical integer ownership in quantum::ir::classical::integer.
//!   Do not replace this representation with f64, fixed-width integers,
//!   or a second arbitrary-precision integer implementation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::hash::{Hash, Hasher};
use core::str::FromStr;

use crate::quantum::ir::classical::integer::{
    IntegerError,
    SignedInteger,
    UnsignedInteger,
};

/// Errors produced by rational construction, parsing, and arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RationalError {
    /// The supplied fraction text is malformed.
    InvalidLiteral,

    /// A denominator was zero.
    DivisionByZero,

    /// The canonical arbitrary-width integer implementation rejected
    /// an operation.
    Integer(IntegerError),
}

impl fmt::Display for RationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLiteral => {
                f.write_str("invalid rational value")
            }
            Self::DivisionByZero => {
                f.write_str("rational denominator or divisor is zero")
            }
            Self::Integer(error) => {
                write!(f, "rational integer operation failed: {error}")
            }
        }
    }
}

impl std::error::Error for RationalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            Self::InvalidLiteral | Self::DivisionByZero => None,
        }
    }
}

impl From<IntegerError> for RationalError {
    fn from(error: IntegerError) -> Self {
        Self::Integer(error)
    }
}

/// An exact rational number.
///
/// The value is `numerator / denominator`.
///
/// The denominator is strictly positive, and the fraction is always reduced.
/// Consequently numerically equal rational values have the same canonical
/// representation.
///
/// Examples:
///
/// - `0` is represented as `0/1`.
/// - `2/4` is normalized to `1/2`.
/// - `1/-2` is normalized to `-1/2`.
/// - `-3/-6` is normalized to `1/2`.
///
/// Source spelling and source spans are not stored here; those belong to the
/// frontend AST.
#[derive(Clone, Debug)]
pub struct RationalValue {
    numerator: SignedInteger,
    denominator: UnsignedInteger,
}

impl RationalValue {
    /// Constructs and normalizes a rational value.
    ///
    /// A zero denominator is rejected. A negative denominator is moved into
    /// the numerator's sign, leaving the stored denominator positive.
    pub fn new(
        numerator: SignedInteger,
        denominator: SignedInteger,
    ) -> Result<Self, RationalError> {
        if denominator.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        let (denominator_negative, denominator_magnitude) =
            denominator.parts();

        let (numerator_negative, numerator_magnitude) =
            numerator.parts();

        if numerator.is_zero() {
            return Ok(Self::zero());
        }

        let negative = numerator_negative ^ denominator_negative;
        let numerator_magnitude = numerator_magnitude.clone();
        let denominator_magnitude = denominator_magnitude.clone();

        Self::from_signed_magnitudes(
            negative,
            numerator_magnitude,
            denominator_magnitude,
        )
    }

    /// Constructs a rational from an arbitrary-width signed numerator and
    /// an arbitrary-width unsigned denominator.
    ///
    /// The denominator must be non-zero.
    pub fn from_parts(
        numerator: SignedInteger,
        denominator: UnsignedInteger,
    ) -> Result<Self, RationalError> {
        if denominator.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        let (negative, magnitude) = numerator.parts();

        if magnitude.is_zero() {
            return Ok(Self::zero());
        }

        Self::from_signed_magnitudes(
            negative,
            magnitude.clone(),
            denominator,
        )
    }

    /// Creates rational zero in canonical `0/1` form.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            numerator: SignedInteger::zero(),
            denominator: UnsignedInteger::from_u8(1),
        }
    }

    /// Creates rational one in canonical `1/1` form.
    #[must_use]
    pub fn one() -> Self {
        Self {
            numerator: SignedInteger::from_i8(1),
            denominator: UnsignedInteger::from_u8(1),
        }
    }

    /// Creates a rational value from a signed machine integer.
    ///
    /// This is a convenience constructor, not a limit on rational width.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        Self {
            numerator: SignedInteger::from_i128(value),
            denominator: UnsignedInteger::from_u8(1),
        }
    }

    /// Parses a fraction in `numerator/denominator` form.
    ///
    /// Each component is a signed or unsigned base-10 integer with optional
    /// separators between digits. Examples: `3/4`, `-7/9`, `1/-2`, `1_000/3`.
    ///
    /// Whitespace, decimal points, exponent notation, missing components,
    /// multiple slashes, and a zero denominator are rejected.
    ///
    /// This function is a rational-value parser, not a declaration of source
    /// grammar. Frontend syntax must follow the normative Zamani grammar.
    pub fn parse(input: &str) -> Result<Self, RationalError> {
        let (numerator_text, denominator_text) = input
            .split_once('/')
            .ok_or(RationalError::InvalidLiteral)?;

        if numerator_text.is_empty()
            || denominator_text.is_empty()
            || denominator_text.contains('/')
        {
            return Err(RationalError::InvalidLiteral);
        }

        let numerator = parse_signed_integer(numerator_text)?;
        let denominator = parse_signed_integer(denominator_text)?;

        Self::new(numerator, denominator)
    }

    /// Returns the canonical signed numerator.
    #[must_use]
    pub fn numerator(&self) -> &SignedInteger {
        &self.numerator
    }

    /// Returns the canonical positive denominator.
    #[must_use]
    pub fn denominator(&self) -> &UnsignedInteger {
        &self.denominator
    }

    /// Returns whether this rational value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    /// Returns whether this rational value is negative.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    /// Returns whether the rational value is an integer.
    ///
    /// Because values are normalized, this is true exactly when the
    /// denominator equals one.
    #[must_use]
    pub fn is_integer(&self) -> bool {
        self.denominator == UnsignedInteger::from_u8(1)
    }

    /// Returns the additive inverse.
    #[must_use]
    pub fn negated(&self) -> Self {
        Self {
            numerator: self.numerator.negated(),
            denominator: self.denominator.clone(),
        }
    }

    /// Returns the absolute value.
    #[must_use]
    pub fn abs(&self) -> Self {
        let (_, magnitude) = self.numerator.parts();

        let numerator = SignedInteger::from_parts(
            false,
            magnitude.clone(),
        )
        .expect("a non-negative sign is valid for every magnitude");

        Self {
            numerator,
            denominator: self.denominator.clone(),
        }
    }

    /// Returns the reciprocal.
    ///
    /// Zero has no reciprocal and produces `DivisionByZero`.
    pub fn reciprocal(&self) -> Result<Self, RationalError> {
        if self.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        let (negative, numerator_magnitude) = self.numerator.parts();

        let numerator = SignedInteger::from_parts(
            negative,
            self.denominator.clone(),
        )?;

        Self::from_signed_magnitudes(
            negative,
            self.denominator.clone(),
            numerator_magnitude.clone(),
        )
        .map(|value| {
            // `from_signed_magnitudes` receives the sign explicitly and
            // always normalizes the resulting fraction.
            let _ = numerator;
            value
        })
    }

    /// Adds two rational values exactly.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, RationalError> {
        // a/b + c/d = (a*d + c*b)/(b*d)
        let left_denominator =
            SignedInteger::from_parts(
                false,
                rhs.denominator.clone(),
            )?;

        let right_denominator =
            SignedInteger::from_parts(
                false,
                self.denominator.clone(),
            )?;

        let left = self.numerator.checked_mul(&left_denominator)?;
        let right = rhs.numerator.checked_mul(&right_denominator)?;
        let numerator = left.checked_add(&right)?;

        let denominator =
            self.denominator.checked_mul(&rhs.denominator)?;

        Self::from_parts(numerator, denominator)
    }

    /// Subtracts two rational values exactly.
    pub fn checked_sub(&self, rhs: &Self) -> Result<Self, RationalError> {
        self.checked_add(&rhs.negated())
    }

    /// Multiplies two rational values exactly.
    ///
    /// Cross-cancellation is performed before multiplication to reduce
    /// intermediate magnitude and resource use.
    pub fn checked_mul(&self, rhs: &Self) -> Result<Self, RationalError> {
        // (a/b) * (c/d)
        // Cancel gcd(|a|, d) and gcd(|c|, b) before multiplying.
        let (left_negative, left_magnitude) = self.numerator.parts();
        let (right_negative, right_magnitude) = rhs.numerator.parts();

        if left_magnitude.is_zero() || right_magnitude.is_zero() {
            return Ok(Self::zero());
        }

        let first_gcd = gcd(
            left_magnitude.clone(),
            rhs.denominator.clone(),
        )?;
        let second_gcd = gcd(
            right_magnitude.clone(),
            self.denominator.clone(),
        )?;

        let left_reduced =
            left_magnitude.checked_div(&first_gcd)?;
        let right_reduced =
            right_magnitude.checked_div(&second_gcd)?;
        let left_denominator =
            self.denominator.checked_div(&second_gcd)?;
        let right_denominator =
            rhs.denominator.checked_div(&first_gcd)?;

        let numerator_magnitude =
            left_reduced.checked_mul(&right_reduced)?;
        let denominator =
            left_denominator.checked_mul(&right_denominator)?;

        let numerator = SignedInteger::from_parts(
            left_negative ^ right_negative,
            numerator_magnitude,
        )?;

        Self::from_parts(numerator, denominator)
    }

    /// Divides two rational values exactly.
    ///
    /// Division by a zero rational value is rejected.
    pub fn checked_div(&self, rhs: &Self) -> Result<Self, RationalError> {
        if rhs.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        // (a/b) / (c/d) = (a*d) / (b*|c|), with c's sign transferred.
        let (right_negative, right_magnitude) = rhs.numerator.parts();
        let (left_negative, left_magnitude) = self.numerator.parts();

        if left_magnitude.is_zero() {
            return Ok(Self::zero());
        }

        let numerator_magnitude =
            left_magnitude.checked_mul(&rhs.denominator)?;
        let denominator =
            self.denominator.checked_mul(right_magnitude)?;

        let numerator = SignedInteger::from_parts(
            left_negative ^ right_negative,
            numerator_magnitude,
        )?;

        Self::from_parts(numerator, denominator)
    }

    /// Compares rational values exactly without converting to floating point.
    ///
    /// This comparison may allocate intermediate arbitrary-width integers.
    pub fn checked_cmp(
        &self,
        rhs: &Self,
    ) -> Result<core::cmp::Ordering, RationalError> {
        use core::cmp::Ordering;

        let (left_negative, left_magnitude) = self.numerator.parts();
        let (right_negative, right_magnitude) = rhs.numerator.parts();

        match (left_magnitude.is_zero(), right_magnitude.is_zero()) {
            (true, true) => return Ok(Ordering::Equal),
            (true, false) => {
                return Ok(if right_negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                });
            }
            (false, true) => {
                return Ok(if left_negative {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
            _ => {}
        }

        if left_negative != right_negative {
            return Ok(if left_negative {
                Ordering::Less
            } else {
                Ordering::Greater
            });
        }

        let left =
            left_magnitude.checked_mul(&rhs.denominator)?;
        let right =
            right_magnitude.checked_mul(&self.denominator)?;

        let magnitude_ordering = left.cmp(&right);

        Ok(if left_negative {
            magnitude_ordering.reverse()
        } else {
            magnitude_ordering
        })
    }

    /// Validates all representation invariants.
    pub fn validate(&self) -> Result<(), RationalError> {
        self.numerator.validate()?;
        self.denominator.validate()?;

        if self.denominator.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        let (negative, magnitude) = self.numerator.parts();

        if negative && magnitude.is_zero() {
            return Err(RationalError::Integer(IntegerError::InvalidSign));
        }

        if self.is_zero() {
            if self.denominator != UnsignedInteger::from_u8(1) {
                return Err(RationalError::InvalidLiteral);
            }

            return Ok(());
        }

        if gcd(
            magnitude.clone(),
            self.denominator.clone(),
        )? != UnsignedInteger::from_u8(1)
        {
            return Err(RationalError::InvalidLiteral);
        }

        Ok(())
    }

    fn from_signed_magnitudes(
        negative: bool,
        numerator_magnitude: UnsignedInteger,
        denominator: UnsignedInteger,
    ) -> Result<Self, RationalError> {
        if denominator.is_zero() {
            return Err(RationalError::DivisionByZero);
        }

        if numerator_magnitude.is_zero() {
            return Ok(Self::zero());
        }

        let divisor = gcd(
            numerator_magnitude.clone(),
            denominator.clone(),
        )?;

        let numerator_magnitude =
            numerator_magnitude.checked_div(&divisor)?;
        let denominator = denominator.checked_div(&divisor)?;

        let numerator = SignedInteger::from_parts(
            negative,
            numerator_magnitude,
        )?;

        Ok(Self {
            numerator,
            denominator,
        })
    }
}

impl FromStr for RationalValue {
    type Err = RationalError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

impl PartialEq for RationalValue {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator
            && self.denominator == other.denominator
    }
}

impl Eq for RationalValue {}

impl Hash for RationalValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the canonical numerator sign and magnitude, then denominator.
        // Do not depend on display formatting or source spelling.
        let (negative, magnitude) = self.numerator.parts();
        negative.hash(state);
        magnitude.as_be_bytes().hash(state);
        self.denominator.as_be_bytes().hash(state);
    }
}

impl fmt::Display for RationalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_integer() {
            return write!(f, "{}", self.numerator);
        }

        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn gcd(
    mut left: UnsignedInteger,
    mut right: UnsignedInteger,
) -> Result<UnsignedInteger, RationalError> {
    while !right.is_zero() {
        let remainder = left.checked_rem(&right)?;
        left = right;
        right = remainder;
    }

    Ok(left)
}

fn parse_signed_integer(input: &str) -> Result<SignedInteger, RationalError> {
    if input.is_empty() {
        return Err(RationalError::InvalidLiteral);
    }

    let bytes = input.as_bytes();
    let (negative, digits) = match bytes[0] {
        b'-' => (true, &bytes[1..]),
        b'+' => (false, &bytes[1..]),
        _ => (false, bytes),
    };

    if digits.is_empty() {
        return Err(RationalError::InvalidLiteral);
    }

    let magnitude = parse_unsigned_decimal(digits)?;

    SignedInteger::from_parts(negative, magnitude)
        .map_err(RationalError::from)
}

fn parse_unsigned_decimal(
    bytes: &[u8],
) -> Result<UnsignedInteger, RationalError> {
    let mut value = UnsignedInteger::zero();
    let ten = UnsignedInteger::from_u8(10);
    let mut previous_was_digit = false;
    let mut saw_digit = false;

    for (index, byte) in bytes.iter().copied().enumerate() {
        match byte {
            b'0'..=b'9' => {
                value = value.checked_mul(&ten)?;
                value = value.checked_add(
                    &UnsignedInteger::from_u8(byte - b'0'),
                )?;

                previous_was_digit = true;
                saw_digit = true;
            }

            b'_' => {
                let next_is_digit = bytes
                    .get(index + 1)
                    .is_some_and(u8::is_ascii_digit);

                if !previous_was_digit || !next_is_digit {
                    return Err(RationalError::InvalidLiteral);
                }

                previous_was_digit = false;
            }

            _ => return Err(RationalError::InvalidLiteral),
        }
    }

    if !saw_digit {
        return Err(RationalError::InvalidLiteral);
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{RationalError, RationalValue};
    use core::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn rational(text: &str) -> RationalValue {
        RationalValue::parse(text).expect("test rational must parse")
    }

    #[test]
    fn normalizes_equivalent_fractions() {
        assert_eq!(rational("2/4"), rational("1/2"));
        assert_eq!(rational("-3/-6"), rational("1/2"));
        assert_eq!(rational("1/-2"), rational("-1/2"));
        assert_eq!(rational("0/99"), RationalValue::zero());
    }

    #[test]
    fn rejects_zero_denominator() {
        assert_eq!(
            RationalValue::parse("1/0"),
            Err(RationalError::DivisionByZero),
        );
        assert_eq!(
            RationalValue::parse("0/0"),
            Err(RationalError::DivisionByZero),
        );
    }

    #[test]
    fn rejects_malformed_fraction_text() {
        for input in [
            "",
            "1",
            "/2",
            "2/",
            "1/2/3",
            "_1/2",
            "1_/2",
            "1__0/2",
            "1/2_",
            "1/ 2",
            "1.5/2",
            "1/2e3",
            "+/2",
        ] {
            assert!(
                RationalValue::parse(input).is_err(),
                "should reject {input:?}",
            );
        }
    }

    #[test]
    fn supports_separators_and_signs() {
        assert_eq!(rational("-1_000/2"), rational("-500"));
        assert_eq!(rational("+6/-8"), rational("-3/4"));
    }

    #[test]
    fn addition_and_subtraction_are_exact() {
        assert_eq!(
            rational("1/3")
                .checked_add(&rational("1/6"))
                .unwrap(),
            rational("1/2"),
        );

        assert_eq!(
            rational("1/3")
                .checked_sub(&rational("1/2"))
                .unwrap(),
            rational("-1/6"),
        );
    }

    #[test]
    fn multiplication_cross_cancels_and_normalizes() {
        assert_eq!(
            rational("14/15")
                .checked_mul(&rational("25/21"))
                .unwrap(),
            rational("10/9"),
        );
    }

    #[test]
    fn division_is_exact_and_rejects_zero() {
        assert_eq!(
            rational("2/3")
                .checked_div(&rational("4/9"))
                .unwrap(),
            rational("3/2"),
        );

        assert_eq!(
            rational("1/2").checked_div(&RationalValue::zero()),
            Err(RationalError::DivisionByZero),
        );
    }

    #[test]
    fn reciprocal_and_sign_are_correct() {
        assert_eq!(rational("-3/7").reciprocal().unwrap(), rational("-7/3"));
        assert_eq!(
            RationalValue::zero().reciprocal(),
            Err(RationalError::DivisionByZero),
        );
        assert_eq!(rational("-3/7").abs(), rational("3/7"));
        assert_eq!(rational("0/8").negated(), RationalValue::zero());
    }

    #[test]
    fn comparison_does_not_use_floating_point() {
        assert_eq!(
            rational("1/10").checked_cmp(&rational("1/9")).unwrap(),
            Ordering::Less,
        );
        assert_eq!(
            rational("-5/6").checked_cmp(&rational("-4/5")).unwrap(),
            Ordering::Less,
        );
        assert_eq!(
            rational("100/200").checked_cmp(&rational("2/4")).unwrap(),
            Ordering::Equal,
        );
    }

    #[test]
    fn canonical_display_is_deterministic() {
        assert_eq!(rational("2/4").to_string(), "1/2");
        assert_eq!(rational("4/2").to_string(), "2");
        assert_eq!(rational("-4/2").to_string(), "-2");
        assert_eq!(rational("0/3").to_string(), "0");
    }

    #[test]
    fn equal_values_have_equal_hashes() {
        let left = rational("2/4");
        let right = rational("1/2");

        let mut left_hash = DefaultHasher::new();
        let mut right_hash = DefaultHasher::new();

        left.hash(&mut left_hash);
        right.hash(&mut right_hash);

        assert_eq!(left_hash.finish(), right_hash.finish());
    }

    #[test]
    fn supports_numerators_and_denominators_beyond_primitive_widths() {
        let numerator =
            "123456789012345678901234567890123456789012345678901234567890";
        let denominator =
            "987654321098765432109876543210987654321098765432109876543211";

        let value = rational(&format!("{numerator}/{denominator}"));

        assert!(value.validate().is_ok());
        assert_eq!(value.numerator().to_string(), numerator);
        assert_eq!(value.denominator().to_string(), denominator);
    }

    #[test]
    fn validates_canonical_invariants() {
        for value in [
            RationalValue::zero(),
            RationalValue::one(),
            rational("2/4"),
            rational("-17/19"),
            rational("12345678901234567890/7"),
        ] {
            assert!(value.validate().is_ok());
        }
    }
}