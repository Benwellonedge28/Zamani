
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Constants
//!
//! Path: `src/hdl/values/constant.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent known HDL constants exactly, independently of the host machine's
//! native integer width. This module provides arbitrary-length integer
//! magnitudes, signed integer literals, typed integer constants, and explicit
//! logic-vector constants.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and the applicable delegate grammars
//! - `grammar/spec/type-system.md`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/integer.rs`
//! - `src/hdl/types/logic.rs`
//!
//! The grammar owns literal syntax. The frontend owns parsed source structure
//! and semantic identity. The canonical HDL semantic module owns `HdlWidth`,
//! `HdlSignedness`, and `HdlValueType`. HDL integer and logic type modules
//! own their respective type contracts.
//!
//! This file owns constant-value representation and the local operations
//! required to construct, inspect, format, and conservatively check constants.
//!
//! ## Does not own
//!
//! - Parsing Zamani source text or defining numeric-literal syntax.
//! - Symbolic-expression evaluation or parameter resolution.
//! - A replacement AST, semantic type system, or width-expression model.
//! - General HDL expression evaluation or operation-specific overflow policy.
//! - Implicit truncation, extension, rounding, or saturation.
//! - Physical layout, device allocation, synthesis, or target realization.
//! - HDLIR structures, builders, passes, verification, or lowering.
//!
//! Symbolic values belong in `values/symbolic.rs`. Type definitions belong
//! in `types/`. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Scalability
//!
//! Integer magnitudes use dynamically sized base-2^32 limbs. The limb size is
//! an implementation detail, not a maximum HDL value width. Values are never
//! silently narrowed to `u8`, `u16`, `u32`, `u64`, or `u128`.
//!
//! The canonical HDL type system currently represents concrete widths with
//! `HdlWidth::Known(u128)`. This module preserves that contract when checking
//! a value against a declared type. A symbolic width produces an indeterminate
//! fit result rather than being guessed.
//!
//! Actual memory use and execution time depend on the supplied value and
//! available resources. Callers may apply explicit operational budgets, but
//! those budgets must not change a constant's meaning.
//!
//! ## Integration
//!
//! - `src/hdl/values/mod.rs` declares `pub mod constant;`.
//! - `src/hdl/types/integer.rs` supplies `IntegerType` and `IntegerSignedness`.
//! - The frontend or literal adapter constructs constants after parsing.
//! - `values/symbolic.rs` handles parameter-dependent values separately.
//! - Expression evaluation decides operation-specific arithmetic semantics.
//! - Simulation may consume `LogicVectorConstant` without interpreting it as
//!   a physical register or allocating hardware.
//!
//! This module deliberately does not assume an unverified AST constructor,
//! source-span constructor, diagnostic constructor, or literal grammar.
//!
//! ## Freeze criteria
//!
//! - Arbitrary-length magnitudes round-trip through supported radices.
//! - Negative zero is normalized to zero.
//! - Type-fit checks distinguish fits, does-not-fit, and indeterminate.
//! - Symbolic widths are never guessed.
//! - Four-state logic values are never silently coerced into known bits.
//! - No target-specific or HDLIR dependencies are introduced.
//! - Module registration and repository-wide compilation are verified.

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::domains::hdl::HdlWidth;
use crate::hdl::types::integer::{IntegerSignedness, IntegerType};

/// Errors produced while constructing or formatting an HDL constant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstantError {
    /// A radix was outside the supported range 2 through 36.
    InvalidRadix {
        /// The radix supplied by the caller.
        radix: u32,
    },

    /// The digit sequence was empty.
    EmptyDigits,

    /// A digit was not valid for the supplied radix.
    InvalidDigit {
        /// The invalid character.
        digit: char,
        /// The requested radix.
        radix: u32,
        /// The character's zero-based position in the supplied sequence.
        index: usize,
    },

    /// A logic vector was constructed without any bits.
    EmptyLogicVector,

    /// A requested output radix was invalid.
    InvalidOutputRadix {
        /// The requested radix.
        radix: u32,
    },
}

impl fmt::Display for ConstantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRadix { radix } => {
                write!(f, "invalid integer radix {radix}; expected 2..=36")
            }
            Self::EmptyDigits => f.write_str("integer digit sequence is empty"),
            Self::InvalidDigit { digit, radix, index } => write!(
                f,
                "invalid digit {digit:?} for radix {radix} at index {index}"
            ),
            Self::EmptyLogicVector => {
                f.write_str("HDL logic-vector constant must contain at least one bit")
            }
            Self::InvalidOutputRadix { radix } => {
                write!(f, "invalid output radix {radix}; expected 2..=36")
            }
        }
    }
}

impl Error for ConstantError {}

// ---------------------------------------------------------------------------
// Arbitrary-length unsigned magnitude
// ---------------------------------------------------------------------------

/// An arbitrary-length, non-negative integer magnitude.
///
/// The value is stored as little-endian base-2^32 limbs. Zero has no limbs.
/// This representation is internal to constant storage; it does not prescribe
/// the width, encoding, or layout of a hardware type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct IntegerMagnitude {
    limbs: Vec<u32>,
}

impl IntegerMagnitude {
    /// Construct a magnitude from a native unsigned integer.
    ///
    /// This is a convenience constructor, not a limit on the values the type
    /// can represent.
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        let mut remaining = value;
        let mut limbs = Vec::new();

        while remaining != 0 {
            limbs.push(remaining as u32);
            remaining >>= 32;
        }

        Self { limbs }
    }

    /// Return the zero magnitude.
    #[must_use]
    pub const fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    /// Return whether this magnitude is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// Parse a digit sequence into a magnitude in the supplied radix.
    ///
    /// Radices from 2 through 36 are supported. ASCII digits `0` through `9`
    /// and letters `a` through `z` or `A` through `Z` represent digit values
    /// 0 through 35. Separators, prefixes, signs, whitespace, and source-level
    /// literal syntax must be handled by the caller's grammar adapter.
    ///
    /// Leading zeroes are accepted and do not affect the resulting value.
    pub fn parse_radix(
        digits: &str,
        radix: u32,
    ) -> Result<Self, ConstantError> {
        if !(2..=36).contains(&radix) {
            return Err(ConstantError::InvalidRadix { radix });
        }
        if digits.is_empty() {
            return Err(ConstantError::EmptyDigits);
        }

        let mut result = Self::zero();

        for (index, character) in digits.char_indices() {
            let digit = character.to_digit(radix).ok_or(
                ConstantError::InvalidDigit {
                    digit: character,
                    radix,
                    index,
                },
            )?;

            result.multiply_small(radix);
            result.add_small(digit);
        }

        Ok(result)
    }

    /// Return the number of significant bits in this magnitude.
    ///
    /// Zero has a bit length of zero. The result describes the magnitude,
    /// not a declared HDL type width or a signed representation width.
    #[must_use]
    pub fn bit_length(&self) -> u128 {
        let Some(&highest) = self.limbs.last() else {
            return 0;
        };

        let high_bits = u128::from(u32::BITS - highest.leading_zeros());
        (self.limbs.len() as u128 - 1) * u128::from(u32::BITS) + high_bits
    }

    /// Compare this magnitude with another without converting either to a
    /// host-sized integer.
    #[must_use]
    pub fn compare(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;

        match self.limbs.len().cmp(&other.limbs.len()) {
            Ordering::Equal => {
                for (left, right) in self.limbs.iter().rev().zip(
                    other.limbs.iter().rev(),
                ) {
                    match left.cmp(right) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }

    /// Format this magnitude in radix 2 through 36.
    ///
    /// The returned string contains only digits and letters. It does not
    /// include a radix prefix, sign, type annotation, or source-language
    /// separator.
    pub fn to_radix_string(
        &self,
        radix: u32,
    ) -> Result<String, ConstantError> {
        if !(2..=36).contains(&radix) {
            return Err(ConstantError::InvalidOutputRadix { radix });
        }
        if self.is_zero() {
            return Ok(String::from("0"));
        }

        const DIGITS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

        let mut work = self.clone();
        let mut output = Vec::new();

        while !work.is_zero() {
            let remainder = work.divide_small(radix);
            output.push(DIGITS[remainder as usize]);
        }

        output.reverse();

        // The output bytes are exclusively ASCII digits and lowercase letters.
        Ok(String::from_utf8(output)
            .expect("radix formatting only emits valid ASCII"))
    }

    fn normalize(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    fn multiply_small(&mut self, multiplier: u32) {
        if multiplier == 0 || self.is_zero() {
            self.limbs.clear();
            return;
        }

        let mut carry = 0_u64;

        for limb in &mut self.limbs {
            let product =
                u64::from(*limb) * u64::from(multiplier) + carry;
            *limb = product as u32;
            carry = product >> 32;
        }

        if carry != 0 {
            self.limbs.push(carry as u32);
        }
    }

    fn add_small(&mut self, addend: u32) {
        if addend == 0 {
            return;
        }

        let mut carry = u64::from(addend);
        let mut index = 0;

        while carry != 0 {
            if index == self.limbs.len() {
                self.limbs.push(0);
            }

            let sum = u64::from(self.limbs[index]) + carry;
            self.limbs[index] = sum as u32;
            carry = sum >> 32;
            index += 1;
        }

        self.normalize();
    }

    /// Divide by a small nonzero divisor and return the remainder.
    fn divide_small(&mut self, divisor: u32) -> u32 {
        debug_assert_ne!(divisor, 0);

        let mut remainder = 0_u64;

        for limb in self.limbs.iter_mut().rev() {
            let dividend =
                (remainder << 32) | u64::from(*limb);
            *limb = (dividend / u64::from(divisor)) as u32;
            remainder = dividend % u64::from(divisor);
        }

        self.normalize();
        remainder as u32
    }

    fn is_power_of_two_at(&self, bit: u128) -> bool {
        if self.bit_length() != bit + 1 {
            return false;
        }

        let limb_index = bit / u128::from(u32::BITS);
        let bit_index = (bit % u128::from(u32::BITS)) as u32;

        let Ok(limb_index) = usize::try_from(limb_index) else {
            return false;
        };

        self.limbs.iter().enumerate().all(|(index, limb)| {
            if index == limb_index {
                *limb == (1_u32 << bit_index)
            } else {
                *limb == 0
            }
        })
    }
}

impl Ord for IntegerMagnitude {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.compare(other)
    }
}

impl PartialOrd for IntegerMagnitude {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ---------------------------------------------------------------------------
// Signed integer constants
// ---------------------------------------------------------------------------

/// An exact integer constant represented as sign and magnitude.
///
/// The sign is always non-negative for zero. This representation preserves
/// the mathematical value independently of a declared HDL width. A typed
/// constant is not assumed to fit its declared type until checked.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IntegerConstant {
    negative: bool,
    magnitude: IntegerMagnitude,
    integer_type: Option<IntegerType>,
}

/// Result of checking an integer constant against its declared type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstantFit {
    /// The constant is proven to fit the concrete declared type.
    Fits,

    /// The constant is proven not to fit the declared type.
    DoesNotFit,

    /// The type width is symbolic and cannot yet be resolved.
    Indeterminate,
}

impl IntegerConstant {
    /// Construct a non-negative integer constant.
    #[must_use]
    pub fn unsigned(magnitude: IntegerMagnitude) -> Self {
        Self {
            negative: false,
            magnitude,
            integer_type: None,
        }
    }

    /// Construct a signed integer constant from a sign and magnitude.
    ///
    /// A negative zero is normalized to zero.
    #[must_use]
    pub fn signed(negative: bool, magnitude: IntegerMagnitude) -> Self {
        let negative = negative && !magnitude.is_zero();

        Self {
            negative,
            magnitude,
            integer_type: None,
        }
    }

    /// Parse a signed integer constant from a sign and radix digit sequence.
    pub fn parse_radix(
        negative: bool,
        digits: &str,
        radix: u32,
    ) -> Result<Self, ConstantError> {
        Ok(Self::signed(
            negative,
            IntegerMagnitude::parse_radix(digits, radix)?,
        ))
    }

    /// Attach an explicit HDL integer type to this constant.
    ///
    /// This does not truncate, extend, or reinterpret the value. Use
    /// [`Self::check_fit`] to determine whether the value fits the type.
    #[must_use]
    pub fn with_type(mut self, integer_type: IntegerType) -> Self {
        self.integer_type = Some(integer_type);
        self
    }

    /// Return whether the mathematical value is negative.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    /// Return whether the mathematical value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.magnitude.is_zero()
    }

    /// Return the exact non-negative magnitude.
    #[must_use]
    pub const fn magnitude(&self) -> &IntegerMagnitude {
        &self.magnitude
    }

    /// Return the optional declared integer type.
    #[must_use]
    pub const fn integer_type(&self) -> Option<&IntegerType> {
        self.integer_type.as_ref()
    }

    /// Check whether this constant fits its declared integer type.
    ///
    /// The check follows ordinary mathematical signed/unsigned range rules
    /// for the supplied width. It does not select an overflow policy for
    /// arithmetic operations and does not perform any conversion.
    ///
    /// Symbolic widths produce [`ConstantFit::Indeterminate`]. This method
    /// never guesses their value or converts them to a host-sized integer.
    #[must_use]
    pub fn check_fit(&self) -> Option<ConstantFit> {
        let integer_type = self.integer_type.as_ref()?;
        Some(self.fits_type(integer_type))
    }

    /// Check whether this constant fits the supplied integer type.
    #[must_use]
    pub fn fits_type(&self, integer_type: &IntegerType) -> ConstantFit {
        let Some(width) = integer_type.width().as_known() else {
            return ConstantFit::Indeterminate;
        };

        if width == 0 {
            // IntegerType's constructor rejects zero widths. Treat an
            // externally malformed value conservatively rather than fitting it.
            return ConstantFit::DoesNotFit;
        }

        if self.magnitude.is_zero() {
            return if !self.negative {
                ConstantFit::Fits
            } else {
                // Negative zero is normalized by every public constructor.
                ConstantFit::DoesNotFit
            };
        }

        match integer_type.signedness() {
            IntegerSignedness::Unsigned => {
                if self.negative {
                    ConstantFit::DoesNotFit
                } else if self.magnitude.bit_length() <= width {
                    ConstantFit::Fits
                } else {
                    ConstantFit::DoesNotFit
                }
            }
            IntegerSignedness::Signed => {
                let magnitude_bits = self.magnitude.bit_length();

                if self.negative {
                    // Two's-complement signed range includes -2^(width - 1).
                    if magnitude_bits < width {
                        ConstantFit::Fits
                    } else if magnitude_bits == width
                        && self.magnitude.is_power_of_two_at(width - 1)
                    {
                        ConstantFit::Fits
                    } else {
                        ConstantFit::DoesNotFit
                    }
                } else if magnitude_bits < width {
                    // Positive signed values must leave the sign bit clear.
                    ConstantFit::Fits
                } else {
                    ConstantFit::DoesNotFit
                }
            }
        }
    }

    /// Format the exact integer in radix 2 through 36.
    pub fn to_radix_string(
        &self,
        radix: u32,
    ) -> Result<String, ConstantError> {
        let digits = self.magnitude.to_radix_string(radix)?;

        if self.negative {
            Ok(format!("-{digits}"))
        } else {
            Ok(digits)
        }
    }
}

// ---------------------------------------------------------------------------
// Logic-vector constants
// ---------------------------------------------------------------------------

/// One logic bit in a constant vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LogicBit {
    /// Known logical zero.
    Zero,

    /// Known logical one.
    One,

    /// Unknown logic state.
    Unknown,

    /// High-impedance logic state.
    HighImpedance,
}

/// A known-width sequence of logic states.
///
/// Bits are stored in declaration order, most-significant bit first. This is
/// a logical value representation; it does not define byte order, physical
/// layout, pin order, or target-specific encoding.
///
/// The vector must not be interpreted as a known integer if any bit is
/// unknown or high-impedance.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LogicVectorConstant {
    bits: Vec<LogicBit>,
}

impl LogicVectorConstant {
    /// Construct a logic-vector constant.
    ///
    /// An empty vector is rejected. The number of bits is determined by the
    /// supplied collection; there is no arbitrary HDL vector-size ceiling.
    pub fn new(bits: Vec<LogicBit>) -> Result<Self, ConstantError> {
        if bits.is_empty() {
            return Err(ConstantError::EmptyLogicVector);
        }

        Ok(Self { bits })
    }

    /// Return the bits in declaration order, most-significant bit first.
    #[must_use]
    pub fn bits(&self) -> &[LogicBit] {
        &self.bits
    }

    /// Return the vector width as a count of stored bits.
    ///
    /// This is the length of the in-memory representation, not a substitute
    /// for the canonical semantic `HdlWidth` when widths remain symbolic.
    #[must_use]
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Return whether every bit is known zero or one.
    #[must_use]
    pub fn is_fully_known(&self) -> bool {
        self.bits
            .iter()
            .all(|bit| matches!(bit, LogicBit::Zero | LogicBit::One))
    }

    /// Return a binary string using `0`, `1`, `x`, and `z`.
    ///
    /// Unknown and high-impedance states are preserved. The output is a
    /// diagnostic/value representation, not a promise about source grammar.
    #[must_use]
    pub fn to_binary_string(&self) -> String {
        self.bits
            .iter()
            .map(|bit| match bit {
                LogicBit::Zero => '0',
                LogicBit::One => '1',
                LogicBit::Unknown => 'x',
                LogicBit::HighImpedance => 'z',
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Common constant value
// ---------------------------------------------------------------------------

/// A known HDL constant value.
///
/// Symbolic or parameter-dependent values are intentionally not represented
/// by this enum; those belong to the symbolic-value subsystem. The variant
/// names do not prescribe source-level literal spelling.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum HdlConstant {
    /// Exact mathematical integer constant.
    Integer(IntegerConstant),

    /// Explicit sequence of digital logic states.
    LogicVector(LogicVectorConstant),

    /// A Boolean constant.
    Boolean(bool),

    /// A string constant whose source-level escape processing has already
    /// been performed by the appropriate frontend adapter.
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unsigned_type(width: u128) -> IntegerType {
        IntegerType::unsigned(width).expect("positive test width")
    }

    fn signed_type(width: u128) -> IntegerType {
        IntegerType::signed(width).expect("positive test width")
    }

    #[test]
    fn arbitrary_magnitude_round_trips_radices() {
        let value = IntegerMagnitude::parse_radix(
            "123456789abcdef0123456789abcdef",
            16,
        )
        .expect("valid hexadecimal digits");

        let hexadecimal = value
            .to_radix_string(16)
            .expect("valid output radix");

        assert_eq!(hexadecimal, "123456789abcdef0123456789abcdef");

        let reparsed =
            IntegerMagnitude::parse_radix(&hexadecimal, 16)
                .expect("formatted value must parse");

        assert_eq!(value, reparsed);
    }

    #[test]
    fn leading_zeroes_do_not_change_magnitude() {
        let plain =
            IntegerMagnitude::parse_radix("12345", 10).expect("valid digits");
        let padded =
            IntegerMagnitude::parse_radix("00012345", 10).expect("valid digits");

        assert_eq!(plain, padded);
    }

    #[test]
    fn negative_zero_is_normalized() {
        let zero = IntegerConstant::signed(true, IntegerMagnitude::zero());

        assert!(!zero.is_negative());
        assert!(zero.is_zero());
        assert_eq!(
            zero.to_radix_string(10).expect("valid radix"),
            "0"
        );
    }

    #[test]
    fn unsigned_type_rejects_negative_values() {
        let value = IntegerConstant::signed(
            true,
            IntegerMagnitude::from_u128(1),
        );

        assert_eq!(
            value.fits_type(&unsigned_type(8)),
            ConstantFit::DoesNotFit
        );
    }

    #[test]
    fn unsigned_type_accepts_maximum_value() {
        let value = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(255),
        );

        assert_eq!(
            value.fits_type(&unsigned_type(8)),
            ConstantFit::Fits
        );
    }

    #[test]
    fn unsigned_type_rejects_value_that_needs_an_extra_bit() {
        let value = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(256),
        );

        assert_eq!(
            value.fits_type(&unsigned_type(8)),
            ConstantFit::DoesNotFit
        );
    }

    #[test]
    fn signed_type_accepts_positive_and_negative_boundaries() {
        let positive_max = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(127),
        );
        let negative_min = IntegerConstant::signed(
            true,
            IntegerMagnitude::from_u128(128),
        );

        assert_eq!(
            positive_max.fits_type(&signed_type(8)),
            ConstantFit::Fits
        );
        assert_eq!(
            negative_min.fits_type(&signed_type(8)),
            ConstantFit::Fits
        );
    }

    #[test]
    fn signed_type_rejects_out_of_range_boundaries() {
        let positive_overflow = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(128),
        );
        let negative_overflow = IntegerConstant::signed(
            true,
            IntegerMagnitude::from_u128(129),
        );

        assert_eq!(
            positive_overflow.fits_type(&signed_type(8)),
            ConstantFit::DoesNotFit
        );
        assert_eq!(
            negative_overflow.fits_type(&signed_type(8)),
            ConstantFit::DoesNotFit
        );
    }

    #[test]
    fn one_bit_signed_range_is_checked_correctly() {
        let zero = IntegerConstant::unsigned(IntegerMagnitude::zero());
        let negative_one = IntegerConstant::signed(
            true,
            IntegerMagnitude::from_u128(1),
        );
        let positive_one = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(1),
        );

        assert_eq!(zero.fits_type(&signed_type(1)), ConstantFit::Fits);
        assert_eq!(
            negative_one.fits_type(&signed_type(1)),
            ConstantFit::Fits
        );
        assert_eq!(
            positive_one.fits_type(&signed_type(1)),
            ConstantFit::DoesNotFit
        );
    }

    #[test]
    fn attached_type_can_be_checked_without_mutating_value() {
        let value = IntegerConstant::unsigned(
            IntegerMagnitude::from_u128(12),
        )
        .with_type(unsigned_type(4));

        assert_eq!(value.check_fit(), Some(ConstantFit::Fits));
        assert_eq!(
            value.to_radix_string(10).expect("valid radix"),
            "12"
        );
    }

    #[test]
    fn logic_vector_preserves_unknown_and_high_impedance_states() {
        let vector = LogicVectorConstant::new(vec![
            LogicBit::One,
            LogicBit::Unknown,
            LogicBit::HighImpedance,
            LogicBit::Zero,
        ])
        .expect("non-empty vector");

        assert_eq!(vector.width(), 4);
        assert!(!vector.is_fully_known());
        assert_eq!(vector.to_binary_string(), "1xz0");
    }

    #[test]
    fn fully_known_logic_vector_is_recognized() {
        let vector = LogicVectorConstant::new(vec![
            LogicBit::One,
            LogicBit::Zero,
            LogicBit::One,
        ])
        .expect("non-empty vector");

        assert!(vector.is_fully_known());
        assert_eq!(vector.to_binary_string(), "101");
    }

    #[test]
    fn invalid_radices_and_digits_are_rejected() {
        assert!(matches!(
            IntegerMagnitude::parse_radix("101", 1),
            Err(ConstantError::InvalidRadix { .. })
        ));

        assert!(matches!(
            IntegerMagnitude::parse_radix("102", 2),
            Err(ConstantError::InvalidDigit { .. })
        ));

        assert!(matches!(
            IntegerMagnitude::parse_radix("", 10),
            Err(ConstantError::EmptyDigits)
        ));
    }

    #[test]
    fn empty_logic_vector_is_rejected() {
        assert_eq!(
            LogicVectorConstant::new(Vec::new()),
            Err(ConstantError::EmptyLogicVector)
        );
    }
}
