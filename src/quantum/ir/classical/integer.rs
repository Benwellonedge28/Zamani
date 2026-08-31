//! Zamani Quantum IR — Classical Integer Semantics
//!
//! Path:
//!     src/quantum/ir/classical/integer.rs
//!
//! # Purpose
//!
//! This module owns the semantic representation and checked arithmetic
//! semantics of classical signed and unsigned integers used by the
//! Zamani Quantum IR.
//!
//! It is intentionally independent of:
//!
//! - quantum hardware;
//! - qubit identity;
//! - physical qubit mapping;
//! - topology;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - simulation state;
//! - backend execution;
//! - frontend ASTs;
//! - optimization policy.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani IR
//!                             │
//!                 ┌───────────┴───────────┐
//!                 │                       │
//!              classical               quantum
//!                 │                       │
//!          classical/integer.rs       qubit.rs
//!                 │
//!          classical/value.rs
//!                 │
//!          program/operation.rs
//!                 │
//!          control-flow.rs
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - arbitrary-width unsigned integer semantics;
//! - arbitrary-width signed integer semantics;
//! - canonical integer representation;
//! - checked integer arithmetic;
//! - checked bit operations;
//! - checked shifts;
//! - checked division/remainder;
//! - comparison;
//! - conversion between signed and unsigned representations;
//! - conversion to/from primitive Rust integers;
//! - semantic-width validation;
//! - deterministic formatting;
//! - deterministic equality and ordering.
//!
//! It does NOT own:
//!
//! - `ValueId`;
//! - classical-bit identity;
//! - register declarations;
//! - expressions;
//! - symbolic parameters;
//! - runtime memory;
//! - qubit identifiers;
//! - hardware integers;
//! - compiler resource limits.
//!
//! # Important scalability rule
//!
//! Rust primitive integers such as `u64` and `i128` are useful for
//! interoperability, but they are NOT the semantic ceiling of Zamani.
//!
//! Arbitrary-width integers use canonical big-endian magnitude bytes.
//!
//! Therefore:
//!
//! ```text
//! u8
//! u64
//! u128
//! 1024-bit
//! 1-million-bit
//! N-bit
//! ```
//!
//! are all represented by the same semantic abstraction.
//!
//! The practical limit is determined by available memory and an external
//! compilation/runtime resource policy.
//!
//! This file intentionally contains no:
//!
//! ```text
//! MAX_INTEGER_BITS
//! MAX_CLASSICAL_BITS
//! MAX_REGISTER_SIZE
//! ```
//!
//! # Canonical representation
//!
//! Unsigned integers use:
//!
//! ```text
//! zero      = []
//! non-zero  = [most-significant-byte, ..., least-significant-byte]
//! ```
//!
//! Leading zero bytes are never retained.
//!
//! Signed integers use sign + magnitude:
//!
//! ```text
//! negative = false, magnitude
//! negative = true,  magnitude
//! ```
//!
//! Zero is always represented as non-negative.
//!
//! Therefore negative zero is impossible.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external crates;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `classical/value.rs`
//!     owns the general classical-value container. It may store or expose
//!     these integer types.
//!
//! `core/types.rs`
//!     owns `SignedIntegerType` and `UnsignedIntegerType`, which describe
//!     semantic integer type widths.
//!
//! `classical/expression.rs`
//!     may use these types as operands/results of arithmetic expressions.
//!
//! `classical/predicate.rs`
//!     may use comparison operations from this module.
//!
//! `classical/assignment.rs`
//!     may use checked conversions and width validation.
//!
//! `control/*`
//!     may consume integer predicates and branch conditions.
//!
//! `program/operand.rs` / `program/result.rs`
//!     may carry integer values as classical operands/results.
//!
//! `validation/*`
//!     may call `validate_width()` and representation validation.
//!
//! `serialization/*`
//!     should serialize the canonical magnitude and sign directly.
//!
//! `hashing/*`
//!     should hash the canonical representation rather than implementation
//!     details.
//!
//! `quantum::ir::qubit`
//!     is deliberately NOT imported. Integer semantics are classical and do
//!     not own or manipulate qubit identity.
//!
//! # Error philosophy
//!
//! No arithmetic operation silently wraps.
//!
//! Checked operations return `Result`.
//!
//! This is important for dynamic quantum-classical programs because values
//! originating from measurement may not be known at compile time. Silent
//! overflow or division-by-zero behavior could otherwise make simulation,
//! compilation and hardware execution semantically diverge.
//!
//! Wrapping arithmetic, saturating arithmetic and target-specific overflow
//! behavior belong in explicit language/runtime semantics above this layer.
//!
//! # Security
//!
//! This module does not impose an arbitrary semantic integer-size limit.
//!
//! Resource exhaustion protection belongs to the caller through a compilation
//! or runtime resource policy.
//!
//! Nevertheless, all internal length arithmetic uses checked/saturating
//! operations where necessary so malformed values cannot cause accidental
//! integer wraparound in indexing calculations.
//!
//! # Determinism
//!
//! The following are deterministic:
//!
//! - equality;
//! - ordering;
//! - serialization input;
//! - display;
//! - bit width;
//! - byte width;
//! - arithmetic results.
//!
//! No `HashMap` or randomized data structure is used.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by checked classical integer operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerError {
    /// The requested operation would overflow the selected fixed semantic
    /// width.
    Overflow {
        /// Number of bits required by the result.
        required_bits: u64,

        /// Maximum permitted semantic width.
        width: u64,
    },

    /// A signed operation would overflow.
    SignedOverflow,

    /// Division by zero.
    DivisionByZero,

    /// Invalid bit index.
    BitIndexOutOfBounds {
        /// Requested index.
        index: u64,

        /// Number of available bits.
        width: u64,
    },

    /// Invalid shift amount.
    ShiftTooLarge {
        /// Requested shift.
        shift: u64,

        /// Number of available bits.
        width: u64,
    },

    /// Conversion cannot be represented.
    ConversionOverflow,

    /// Negative integer cannot be converted to unsigned.
    NegativeToUnsigned,

    /// The representation is not canonical.
    NonCanonical,

    /// A requested width is invalid.
    InvalidWidth,

    /// A signed integer has an invalid sign/magnitude combination.
    InvalidSign,

    /// A requested operation requires a positive integer.
    NotPositive,

    /// A requested operation requires a non-zero integer.
    ZeroValue,
}

impl fmt::Display for IntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow {
                required_bits,
                width,
            } => write!(
                f,
                "integer requires {required_bits} bits but semantic width is {width}"
            ),

            Self::SignedOverflow => {
                f.write_str("signed integer operation overflowed")
            }

            Self::DivisionByZero => {
                f.write_str("integer division by zero")
            }

            Self::BitIndexOutOfBounds { index, width } => {
                write!(
                    f,
                    "integer bit index {index} is outside width {width}"
                )
            }

            Self::ShiftTooLarge { shift, width } => {
                write!(
                    f,
                    "integer shift {shift} exceeds available width {width}"
                )
            }

            Self::ConversionOverflow => {
                f.write_str("integer conversion overflowed")
            }

            Self::NegativeToUnsigned => {
                f.write_str("negative integer cannot be converted to unsigned")
            }

            Self::NonCanonical => {
                f.write_str("integer representation is not canonical")
            }

            Self::InvalidWidth => {
                f.write_str("integer width must be greater than zero")
            }

            Self::InvalidSign => {
                f.write_str("integer sign/magnitude representation is invalid")
            }

            Self::NotPositive => {
                f.write_str("operation requires a positive integer")
            }

            Self::ZeroValue => {
                f.write_str("operation requires a non-zero integer")
            }
        }
    }
}

impl std::error::Error for IntegerError {}

// =============================================================================
// Unsigned integer
// =============================================================================

/// Arbitrary-width unsigned classical integer.
///
/// The magnitude is stored in canonical big-endian form.
///
/// Zero:
///
/// ```text
/// []
/// ```
///
/// Non-zero:
///
/// ```text
/// [MSB, ..., LSB]
/// ```
///
/// There is no fixed semantic maximum.
#[derive(Clone, Debug, Default)]
pub struct UnsignedInteger {
    magnitude: Vec<u8>,
}

impl UnsignedInteger {
    /// Creates zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            magnitude: Vec::new(),
        }
    }

    /// Creates an unsigned integer from `u8`.
    #[must_use]
    pub fn from_u8(value: u8) -> Self {
        Self::from_u128(value as u128)
    }

    /// Creates an unsigned integer from `u16`.
    #[must_use]
    pub fn from_u16(value: u16) -> Self {
        Self::from_u128(value as u128)
    }

    /// Creates an unsigned integer from `u32`.
    #[must_use]
    pub fn from_u32(value: u32) -> Self {
        Self::from_u128(value as u128)
    }

    /// Creates an unsigned integer from `u64`.
    #[must_use]
    pub fn from_u64(value: u64) -> Self {
        Self::from_u128(value as u128)
    }

    /// Creates an unsigned integer from `u128`.
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        Self::from_be_bytes(&value.to_be_bytes())
    }

    /// Creates an unsigned integer from arbitrary big-endian bytes.
    ///
    /// Leading zero bytes are removed.
    #[must_use]
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let first = bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len());

        Self {
            magnitude: bytes[first..].to_vec(),
        }
    }

    /// Creates an unsigned integer from owned big-endian bytes.
    #[must_use]
    pub fn from_be_bytes_owned(bytes: Vec<u8>) -> Self {
        let first = bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len());

        Self {
            magnitude: bytes[first..].to_vec(),
        }
    }

    /// Returns canonical big-endian bytes.
    #[must_use]
    pub fn as_be_bytes(&self) -> &[u8] {
        &self.magnitude
    }

    /// Consumes the integer and returns canonical big-endian bytes.
    #[must_use]
    pub fn into_be_bytes(self) -> Vec<u8> {
        self.magnitude
    }

    /// Returns whether this integer is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }

    /// Returns whether this integer is positive.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        !self.is_zero()
    }

    /// Returns the number of stored bytes.
    #[must_use]
    pub fn byte_width(&self) -> u64 {
        self.magnitude.len() as u64
    }

    /// Returns the number of significant bits.
    #[must_use]
    pub fn bit_width(&self) -> u64 {
        match self.magnitude.first() {
            None => 0,

            Some(first) => {
                let leading_zeroes = first.leading_zeros() as u64;

                self.byte_width()
                    .saturating_mul(8)
                    .saturating_sub(leading_zeroes)
            }
        }
    }

    /// Validates the canonical representation.
    pub fn validate(&self) -> Result<(), IntegerError> {
        if self.magnitude.len() > 1
            && self.magnitude.first() == Some(&0)
        {
            return Err(IntegerError::NonCanonical);
        }

        if self.magnitude.len() == 1
            && self.magnitude.first() == Some(&0)
        {
            return Err(IntegerError::NonCanonical);
        }

        Ok(())
    }

    /// Returns the bit at `index`.
    ///
    /// Bit zero is the least-significant bit.
    pub fn bit(&self, index: u64) -> Result<bool, IntegerError> {
        if index >= self.bit_width() {
            return Err(IntegerError::BitIndexOutOfBounds {
                index,
                width: self.bit_width(),
            });
        }

        let byte_from_right = (index / 8) as usize;
        let bit_in_byte = (index % 8) as u8;

        let byte_index = self
            .magnitude
            .len()
            .checked_sub(1 + byte_from_right)
            .ok_or(IntegerError::BitIndexOutOfBounds {
                index,
                width: self.bit_width(),
            })?;

        Ok((self.magnitude[byte_index] & (1u8 << bit_in_byte)) != 0)
    }

    /// Returns a bit, treating out-of-range bits as zero.
    #[must_use]
    pub fn bit_or_zero(&self, index: u64) -> bool {
        if index >= self.bit_width() {
            return false;
        }

        self.bit(index).unwrap_or(false)
    }

    /// Sets a bit and returns a new integer.
    ///
    /// This method does not mutate the original value.
    pub fn with_bit(
        &self,
        index: u64,
        value: bool,
    ) -> Result<Self, IntegerError> {
        let target_width = index
            .checked_add(1)
            .ok_or(IntegerError::ConversionOverflow)?;

        let bytes_needed_u64 = target_width
            .saturating_add(7)
            / 8;

        let bytes_needed = usize::try_from(bytes_needed_u64)
            .map_err(|_| IntegerError::ConversionOverflow)?;

        let mut bytes = if self.magnitude.len() < bytes_needed {
            let mut result =
                vec![0u8; bytes_needed - self.magnitude.len()];
            result.extend_from_slice(&self.magnitude);
            result
        } else {
            self.magnitude.clone()
        };

        let byte_from_right = (index / 8) as usize;
        let bit_in_byte = (index % 8) as u8;

        let byte_index = bytes
            .len()
            .checked_sub(1 + byte_from_right)
            .ok_or(IntegerError::ConversionOverflow)?;

        if value {
            bytes[byte_index] |= 1u8 << bit_in_byte;
        } else {
            bytes[byte_index] &= !(1u8 << bit_in_byte);
        }

        Ok(Self::from_be_bytes_owned(bytes))
    }

    /// Returns the bitwise NOT using the specified semantic width.
    pub fn not_with_width(
        &self,
        width: u64,
    ) -> Result<Self, IntegerError> {
        if width == 0 {
            return Err(IntegerError::InvalidWidth);
        }

        if self.bit_width() > width {
            return Err(IntegerError::Overflow {
                required_bits: self.bit_width(),
                width,
            });
        }

        let byte_count_u64 = width
            .checked_add(7)
            .ok_or(IntegerError::ConversionOverflow)?
            / 8;

        let byte_count = usize::try_from(byte_count_u64)
            .map_err(|_| IntegerError::ConversionOverflow)?;

        let mut bytes = vec![0u8; byte_count];

        let offset = byte_count.saturating_sub(self.magnitude.len());

        bytes[offset..].copy_from_slice(&self.magnitude);

        for byte in &mut bytes {
            *byte = !*byte;
        }

        let unused_bits = byte_count
            .checked_mul(8)
            .and_then(|bits| {
                usize::try_from(width)
                    .ok()
                    .map(|width_usize| bits.saturating_sub(width_usize))
            })
            .unwrap_or(0);

        if unused_bits > 0 {
            let mask = 0xffu8 >> unused_bits;
            bytes[0] &= mask;
        }

        Ok(Self::from_be_bytes_owned(bytes))
    }

    /// Checked addition.
    pub fn checked_add(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        Ok(Self {
            magnitude: add_magnitudes(
                &self.magnitude,
                &rhs.magnitude,
            ),
        })
    }

    /// Checked subtraction.
    ///
    /// Unsigned subtraction requires `self >= rhs`.
    pub fn checked_sub(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        if self < rhs {
            return Err(IntegerError::ConversionOverflow);
        }

        Ok(Self {
            magnitude: subtract_magnitudes(
                &self.magnitude,
                &rhs.magnitude,
            ),
        })
    }

    /// Multiplication.
    pub fn checked_mul(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        Ok(Self {
            magnitude: multiply_magnitudes(
                &self.magnitude,
                &rhs.magnitude,
            ),
        })
    }

    /// Division with remainder.
    pub fn checked_div_rem(
        &self,
        rhs: &Self,
    ) -> Result<(Self, Self), IntegerError> {
        if rhs.is_zero() {
            return Err(IntegerError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok((Self::zero(), Self::zero()));
        }

        if self < rhs {
            return Ok((Self::zero(), self.clone()));
        }

        let mut quotient = Self::zero();
        let mut remainder = Self::zero();

        let width = self.bit_width();

        let mut index = width;

        while index > 0 {
            index -= 1;

            remainder = remainder
                .checked_mul(&Self::from_u8(2))?;

            if self.bit_or_zero(index) {
                remainder =
                    remainder.checked_add(&Self::from_u8(1))?;
            }

            if remainder >= *rhs {
                remainder =
                    remainder.checked_sub(rhs)?;

                quotient =
                    quotient.with_bit(index, true)?;
            }
        }

        Ok((quotient, remainder))
    }

    /// Checked integer division.
    pub fn checked_div(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        self.checked_div_rem(rhs).map(|pair| pair.0)
    }

    /// Checked remainder.
    pub fn checked_rem(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        self.checked_div_rem(rhs).map(|pair| pair.1)
    }

    /// Checked left shift.
    pub fn checked_shl(
        &self,
        shift: u64,
    ) -> Result<Self, IntegerError> {
        if self.is_zero() {
            return Ok(Self::zero());
        }

        let new_width = self
            .bit_width()
            .checked_add(shift)
            .ok_or(IntegerError::ConversionOverflow)?;

        let byte_shift = shift / 8;
        let bit_shift = (shift % 8) as u8;

        let byte_shift_usize =
            usize::try_from(byte_shift)
                .map_err(|_| IntegerError::ConversionOverflow)?;

        let total_len = self
            .magnitude
            .len()
            .checked_add(byte_shift_usize)
            .and_then(|len| len.checked_add(1))
            .ok_or(IntegerError::ConversionOverflow)?;

        let mut result = vec![0u8; total_len];

        let start = total_len
            .checked_sub(self.magnitude.len())
            .ok_or(IntegerError::ConversionOverflow)?;

        result[start..].copy_from_slice(&self.magnitude);

        if bit_shift != 0 {
            let mut carry = 0u16;

            for index in (0..result.len()).rev() {
                let current =
                    (u16::from(result[index]) << bit_shift) | carry;

                result[index] = current as u8;
                carry = current >> 8;
            }

            if carry != 0 {
                result[0] = carry as u8;
            }
        }

        let result_value = Self::from_be_bytes_owned(result);

        if result_value.bit_width() < new_width {
            return Err(IntegerError::ConversionOverflow);
        }

        Ok(result_value)
    }

    /// Checked right shift.
    ///
    /// Bits shifted out of the least-significant side are discarded.
    pub fn checked_shr(
        &self,
        shift: u64,
    ) -> Result<Self, IntegerError> {
        if shift >= self.bit_width() {
            return Ok(Self::zero());
        }

        let byte_shift = shift / 8;
        let bit_shift = (shift % 8) as u8;

        let byte_shift_usize =
            usize::try_from(byte_shift)
                .map_err(|_| IntegerError::ConversionOverflow)?;

        if byte_shift_usize >= self.magnitude.len() {
            return Ok(Self::zero());
        }

        let end = self.magnitude.len() - byte_shift_usize;

        let mut result =
            self.magnitude[..end].to_vec();

        if bit_shift != 0 {
            let mut carry = 0u8;

            for byte in &mut result {
                let next_carry =
                    *byte << (8 - bit_shift);

                *byte =
                    (*byte >> bit_shift) | carry;

                carry = next_carry;
            }
        }

        Ok(Self::from_be_bytes_owned(result))
    }

    /// Bitwise AND.
    #[must_use]
    pub fn bitand(&self, rhs: &Self) -> Self {
        bitwise_binary(
            &self.magnitude,
            &rhs.magnitude,
            |a, b| a & b,
        )
    }

    /// Bitwise OR.
    #[must_use]
    pub fn bitor(&self, rhs: &Self) -> Self {
        bitwise_binary(
            &self.magnitude,
            &rhs.magnitude,
            |a, b| a | b,
        )
    }

    /// Bitwise XOR.
    #[must_use]
    pub fn bitxor(&self, rhs: &Self) -> Self {
        bitwise_binary(
            &self.magnitude,
            &rhs.magnitude,
            |a, b| a ^ b,
        )
    }

    /// Converts to `u8`.
    pub fn to_u8(&self) -> Result<u8, IntegerError> {
        let value = self.to_u128()?;

        u8::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to `u16`.
    pub fn to_u16(&self) -> Result<u16, IntegerError> {
        let value = self.to_u128()?;

        u16::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to `u32`.
    pub fn to_u32(&self) -> Result<u32, IntegerError> {
        let value = self.to_u128()?;

        u32::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to `u64`.
    pub fn to_u64(&self) -> Result<u64, IntegerError> {
        let value = self.to_u128()?;

        u64::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to `u128`.
    pub fn to_u128(&self) -> Result<u128, IntegerError> {
        if self.magnitude.len() > 16 {
            return Err(IntegerError::ConversionOverflow);
        }

        let mut bytes = [0u8; 16];

        let offset = 16 - self.magnitude.len();

        bytes[offset..].copy_from_slice(&self.magnitude);

        Ok(u128::from_be_bytes(bytes))
    }

    /// Restricts the integer to a semantic unsigned width.
    pub fn validate_width(
        &self,
        width: u64,
    ) -> Result<(), IntegerError> {
        if width == 0 {
            return Err(IntegerError::InvalidWidth);
        }

        let required = self.bit_width();

        if required > width {
            return Err(IntegerError::Overflow {
                required_bits: required,
                width,
            });
        }

        Ok(())
    }

    /// Truncates the value to the requested width.
    ///
    /// This operation is explicit and therefore does not silently occur during
    /// ordinary arithmetic.
    pub fn truncate_to_width(
        &self,
        width: u64,
    ) -> Result<Self, IntegerError> {
        if width == 0 {
            return Err(IntegerError::InvalidWidth);
        }

        if self.bit_width() <= width {
            return Ok(self.clone());
        }

        let byte_count_u64 =
            width
                .checked_add(7)
                .ok_or(IntegerError::ConversionOverflow)?
                / 8;

        let byte_count =
            usize::try_from(byte_count_u64)
                .map_err(|_| IntegerError::ConversionOverflow)?;

        let start = self
            .magnitude
            .len()
            .checked_sub(byte_count)
            .ok_or(IntegerError::ConversionOverflow)?;

        let mut bytes =
            self.magnitude[start..].to_vec();

        let unused_bits =
            byte_count
                .checked_mul(8)
                .and_then(|bits| {
                    usize::try_from(width)
                        .ok()
                        .map(|w| bits.saturating_sub(w))
                })
                .unwrap_or(0);

        if unused_bits > 0 {
            bytes[0] &=
                0xffu8 >> unused_bits;
        }

        Ok(Self::from_be_bytes_owned(bytes))
    }
}

impl PartialEq for UnsignedInteger {
    fn eq(&self, other: &Self) -> bool {
        self.magnitude == other.magnitude
    }
}

impl Eq for UnsignedInteger {}

impl PartialOrd for UnsignedInteger {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnsignedInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.magnitude.len().cmp(&other.magnitude.len()) {
            Ordering::Equal => {
                self.magnitude.cmp(&other.magnitude)
            }

            ordering => ordering,
        }
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        let ten = Self::from_u8(10);
        let mut value = self.clone();
        let mut digits = Vec::new();

        while !value.is_zero() {
            let (quotient, remainder) =
                value
                    .checked_div_rem(&ten)
                    .map_err(|_| fmt::Error)?;

            let digit =
                remainder
                    .to_u8()
                    .map_err(|_| fmt::Error)?;

            digits.push(
                char::from(b'0' + digit)
            );

            value = quotient;
        }

        for digit in digits.iter().rev() {
            f.write_str(
                &digit.to_string()
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Signed integer
// =============================================================================

/// Arbitrary-width signed classical integer.
///
/// Zero is always represented as positive.
///
/// Negative zero is impossible.
#[derive(Clone, Debug, Default)]
pub struct SignedInteger {
    negative: bool,
    magnitude: UnsignedInteger,
}

impl SignedInteger {
    /// Creates zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            negative: false,
            magnitude: UnsignedInteger::zero(),
        }
    }

    /// Creates a signed integer from `i8`.
    #[must_use]
    pub fn from_i8(value: i8) -> Self {
        Self::from_i128(value as i128)
    }

    /// Creates a signed integer from `i16`.
    #[must_use]
    pub fn from_i16(value: i16) -> Self {
        Self::from_i128(value as i128)
    }

    /// Creates a signed integer from `i32`.
    #[must_use]
    pub fn from_i32(value: i32) -> Self {
        Self::from_i128(value as i128)
    }

    /// Creates a signed integer from `i64`.
    #[must_use]
    pub fn from_i64(value: i64) -> Self {
        Self::from_i128(value as i128)
    }

    /// Creates a signed integer from `i128`.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        if value < 0 {
            let magnitude =
                UnsignedInteger::from_u128(
                    value.unsigned_abs(),
                );

            Self {
                negative: true,
                magnitude,
            }
        } else {
            Self {
                negative: false,
                magnitude:
                    UnsignedInteger::from_u128(
                        value as u128
                    ),
            }
        }
    }

    /// Creates a signed integer from sign and magnitude.
    pub fn from_parts(
        negative: bool,
        magnitude: UnsignedInteger,
    ) -> Result<Self, IntegerError> {
        if magnitude.is_zero() && negative {
            return Err(IntegerError::InvalidSign);
        }

        Ok(Self {
            negative: negative && !magnitude.is_zero(),
            magnitude,
        })
    }

    /// Returns zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.magnitude.magnitude.is_empty()
    }

    /// Returns whether the value is negative.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    /// Returns whether the value is positive.
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        !self.negative && !self.is_zero()
    }

    /// Returns the absolute magnitude.
    #[must_use]
    pub fn magnitude(&self) -> &UnsignedInteger {
        &self.magnitude
    }

    /// Consumes the signed integer and returns its magnitude.
    #[must_use]
    pub fn into_magnitude(self) -> UnsignedInteger {
        self.magnitude
    }

    /// Returns the signed representation's significant magnitude width.
    #[must_use]
    pub fn magnitude_bit_width(&self) -> u64 {
        self.magnitude.bit_width()
    }

    /// Returns the minimum signed width required to represent this value.
    ///
    /// A signed integer needs one sign bit.
    #[must_use]
    pub fn required_signed_width(&self) -> u64 {
        if self.is_zero() {
            1
        } else if self.negative {
            self.magnitude.bit_width().saturating_add(1)
        } else {
            self.magnitude.bit_width().saturating_add(1)
        }
    }

    /// Validates the representation.
    pub fn validate(&self) -> Result<(), IntegerError> {
        self.magnitude.validate()?;

        if self.negative && self.magnitude.is_zero() {
            return Err(IntegerError::InvalidSign);
        }

        Ok(())
    }

    /// Returns the canonical sign/magnitude representation.
    #[must_use]
    pub fn parts(&self) -> (bool, &UnsignedInteger) {
        (self.negative, &self.magnitude)
    }

    /// Returns the additive inverse.
    #[must_use]
    pub fn negated(&self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        Self {
            negative: !self.negative,
            magnitude: self.magnitude.clone(),
        }
    }

    /// Checked addition.
    pub fn checked_add(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        match (self.negative, rhs.negative) {
            (false, false) => {
                Ok(Self {
                    negative: false,
                    magnitude:
                        self.magnitude
                            .checked_add(
                                &rhs.magnitude
                            )?,
                })
            }

            (true, true) => {
                Ok(Self {
                    negative: true,
                    magnitude:
                        self.magnitude
                            .checked_add(
                                &rhs.magnitude
                            )?,
                })
            }

            (false, true) => {
                self.add_opposite_sign(
                    &rhs.magnitude
                )
            }

            (true, false) => {
                rhs.add_opposite_sign(
                    &self.magnitude
                )
                .map(|value| value.negated())
            }
        }
    }

    fn add_opposite_sign(
        &self,
        rhs_magnitude: &UnsignedInteger,
    ) -> Result<Self, IntegerError> {
        match self.magnitude.cmp(rhs_magnitude) {
            Ordering::Greater => Ok(Self {
                negative: self.negative,
                magnitude:
                    self.magnitude
                        .checked_sub(
                            rhs_magnitude
                        )?,
            }),

            Ordering::Less => Ok(Self {
                negative: !self.negative,
                magnitude:
                    rhs_magnitude
                        .checked_sub(
                            &self.magnitude
                        )?,
            }),

            Ordering::Equal => Ok(Self::zero()),
        }
    }

    /// Checked subtraction.
    pub fn checked_sub(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        self.checked_add(&rhs.negated())
    }

    /// Checked multiplication.
    pub fn checked_mul(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::zero());
        }

        Ok(Self {
            negative:
                self.negative ^ rhs.negative,
            magnitude:
                self.magnitude
                    .checked_mul(
                        &rhs.magnitude
                    )?,
        })
    }

    /// Checked division.
    ///
    /// Division truncates toward zero.
    pub fn checked_div(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        if rhs.is_zero() {
            return Err(IntegerError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok(Self::zero());
        }

        let magnitude =
            self.magnitude
                .checked_div(
                    &rhs.magnitude
                )?;

        if magnitude.is_zero() {
            return Ok(Self::zero());
        }

        Ok(Self {
            negative:
                self.negative ^ rhs.negative,
            magnitude,
        })
    }

    /// Checked remainder.
    ///
    /// The remainder has the same sign as the dividend.
    pub fn checked_rem(
        &self,
        rhs: &Self,
    ) -> Result<Self, IntegerError> {
        if rhs.is_zero() {
            return Err(IntegerError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok(Self::zero());
        }

        let magnitude =
            self.magnitude
                .checked_rem(
                    &rhs.magnitude
                )?;

        if magnitude.is_zero() {
            return Ok(Self::zero());
        }

        Ok(Self {
            negative: self.negative,
            magnitude,
        })
    }

    /// Checked division with remainder.
    pub fn checked_div_rem(
        &self,
        rhs: &Self,
    ) -> Result<(Self, Self), IntegerError> {
        if rhs.is_zero() {
            return Err(IntegerError::DivisionByZero);
        }

        let quotient =
            self.checked_div(rhs)?;

        let remainder =
            self.checked_rem(rhs)?;

        Ok((quotient, remainder))
    }

    /// Converts to `i128` when representable.
    pub fn to_i128(&self) -> Result<i128, IntegerError> {
        let magnitude =
            self.magnitude.to_u128()?;

        if self.negative {
            if magnitude >
                (i128::MAX as u128) + 1
            {
                return Err(
                    IntegerError::ConversionOverflow
                );
            }

            if magnitude ==
                (i128::MAX as u128) + 1
            {
                Ok(i128::MIN)
            } else {
                Ok(-(magnitude as i128))
            }
        } else {
            i128::try_from(magnitude)
                .map_err(|_| {
                    IntegerError::ConversionOverflow
                })
        }
    }

    /// Converts to `i64`.
    pub fn to_i64(&self) -> Result<i64, IntegerError> {
        let value = self.to_i128()?;

        i64::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to `i32`.
    pub fn to_i32(&self) -> Result<i32, IntegerError> {
        let value = self.to_i128()?;

        i32::try_from(value)
            .map_err(|_| IntegerError::ConversionOverflow)
    }

    /// Converts to an unsigned integer.
    pub fn to_unsigned(
        &self,
    ) -> Result<UnsignedInteger, IntegerError> {
        if self.negative {
            return Err(
                IntegerError::NegativeToUnsigned
            );
        }

        Ok(self.magnitude.clone())
    }

    /// Validates that this value fits in a signed semantic width.
    pub fn validate_width(
        &self,
        width: u64,
    ) -> Result<(), IntegerError> {
        if width == 0 {
            return Err(IntegerError::InvalidWidth);
        }

        if self.is_zero() {
            return Ok(());
        }

        let required =
            self.magnitude
                .bit_width()
                .saturating_add(1);

        if required > width {
            return Err(IntegerError::Overflow {
                required_bits: required,
                width,
            });
        }

        Ok(())
    }
}

impl PartialEq for SignedInteger {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative
            && self.magnitude == other.magnitude
    }
}

impl Eq for SignedInteger {}

impl PartialOrd for SignedInteger {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SignedInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (true, false) => {
                if self.is_zero() && other.is_zero() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }

            (false, true) => {
                if self.is_zero() && other.is_zero() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }

            (false, false) => {
                self.magnitude.cmp(
                    &other.magnitude
                )
            }

            (true, true) => {
                other.magnitude.cmp(
                    &self.magnitude
                )
            }
        }
    }
}

impl fmt::Display for SignedInteger {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.negative && !self.is_zero() {
            f.write_str("-")?;
        }

        write!(f, "{}", self.magnitude)
    }
}

// =============================================================================
// Integer union
// =============================================================================

/// A semantic classical integer that preserves signedness.
///
/// This is useful at IR boundaries where the signedness is part of the value
/// rather than inferred from a target machine type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntegerValue {
    /// Signed arbitrary-width integer.
    Signed(SignedInteger),

    /// Unsigned arbitrary-width integer.
    Unsigned(UnsignedInteger),
}

impl IntegerValue {
    /// Returns whether this is signed.
    #[must_use]
    pub const fn is_signed(&self) -> bool {
        matches!(self, Self::Signed(_))
    }

    /// Returns whether this is unsigned.
    #[must_use]
    pub const fn is_unsigned(&self) -> bool {
        matches!(self, Self::Unsigned(_))
    }

    /// Returns whether the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Signed(value) => value.is_zero(),
            Self::Unsigned(value) => value.is_zero(),
        }
    }

    /// Returns the semantic bit width required by the value.
    #[must_use]
    pub fn bit_width(&self) -> u64 {
        match self {
            Self::Signed(value) => {
                value.required_signed_width()
            }

            Self::Unsigned(value) => {
                value.bit_width()
            }
        }
    }

    /// Converts an unsigned value to a signed value.
    pub fn to_signed(
        &self,
    ) -> Result<SignedInteger, IntegerError> {
        match self {
            Self::Signed(value) => Ok(value.clone()),

            Self::Unsigned(value) => {
                Ok(SignedInteger {
                    negative: false,
                    magnitude: value.clone(),
                })
            }
        }
    }

    /// Converts a signed value to an unsigned value.
    pub fn to_unsigned(
        &self,
    ) -> Result<UnsignedInteger, IntegerError> {
        match self {
            Self::Unsigned(value) => Ok(value.clone()),

            Self::Signed(value) => value.to_unsigned(),
        }
    }

    /// Validates the value against an explicit semantic width.
    pub fn validate_width(
        &self,
        width: u64,
    ) -> Result<(), IntegerError> {
        match self {
            Self::Signed(value) => {
                value.validate_width(width)
            }

            Self::Unsigned(value) => {
                value.validate_width(width)
            }
        }
    }
}

// =============================================================================
// Internal arithmetic helpers
// =============================================================================

fn add_magnitudes(
    lhs: &[u8],
    rhs: &[u8],
) -> Vec<u8> {
    let mut result =
        Vec::with_capacity(
            lhs.len()
                .max(rhs.len())
                .saturating_add(1)
        );

    let mut lhs_index = lhs.len();
    let mut rhs_index = rhs.len();
    let mut carry = 0u16;

    while lhs_index > 0 || rhs_index > 0 || carry != 0 {
        let lhs_byte =
            if lhs_index > 0 {
                lhs_index -= 1;
                u16::from(lhs[lhs_index])
            } else {
                0
            };

        let rhs_byte =
            if rhs_index > 0 {
                rhs_index -= 1;
                u16::from(rhs[rhs_index])
            } else {
                0
            };

        let sum =
            lhs_byte
                .saturating_add(rhs_byte)
                .saturating_add(carry);

        result.push((sum & 0xff) as u8);
        carry = sum >> 8;
    }

    result.reverse();
    trim_leading_zeroes(result)
}

fn subtract_magnitudes(
    lhs: &[u8],
    rhs: &[u8],
) -> Vec<u8> {
    debug_assert!(
        compare_magnitudes(lhs, rhs)
            != Ordering::Less
    );

    let mut result =
        Vec::with_capacity(lhs.len());

    let mut lhs_index = lhs.len();
    let mut rhs_index = rhs.len();
    let mut borrow = 0i16;

    while lhs_index > 0 {
        lhs_index -= 1;

        let lhs_byte =
            i16::from(lhs[lhs_index]);

        let rhs_byte =
            if rhs_index > 0 {
                rhs_index -= 1;
                i16::from(rhs[rhs_index])
            } else {
                0
            };

        let mut difference =
            lhs_byte
                .saturating_sub(rhs_byte)
                .saturating_sub(borrow);

        if difference < 0 {
            difference += 256;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result.push(difference as u8);
    }

    result.reverse();

    trim_leading_zeroes(result)
}

fn multiply_magnitudes(
    lhs: &[u8],
    rhs: &[u8],
) -> Vec<u8> {
    if lhs.is_empty() || rhs.is_empty() {
        return Vec::new();
    }

    let length =
        lhs.len()
            .saturating_add(rhs.len());

    let mut result =
        vec![0u8; length];

    for lhs_index in (0..lhs.len()).rev() {
        let mut carry = 0u16;

        for rhs_index in (0..rhs.len()).rev() {
            let result_index =
                lhs_index
                    .saturating_add(
                        rhs_index
                    )
                    .saturating_add(1);

            let current =
                u16::from(result[result_index])
                    .saturating_add(
                        u16::from(lhs[lhs_index])
                            .saturating_mul(
                                u16::from(
                                    rhs[rhs_index]
                                )
                            )
                    )
                    .saturating_add(carry);

            result[result_index] =
                (current & 0xff) as u8;

            carry = current >> 8;
        }

        let mut index =
            lhs_index;

        while carry != 0 {
            if index == 0 {
                break;
            }

            index -= 1;

            let current =
                u16::from(result[index])
                    .saturating_add(carry);

            result[index] =
                (current & 0xff) as u8;

            carry = current >> 8;
        }
    }

    trim_leading_zeroes(result)
}

fn compare_magnitudes(
    lhs: &[u8],
    rhs: &[u8],
) -> Ordering {
    match lhs.len().cmp(&rhs.len()) {
        Ordering::Equal => lhs.cmp(rhs),
        ordering => ordering,
    }
}

fn trim_leading_zeroes(
    bytes: Vec<u8>,
) -> Vec<u8> {
    let first =
        bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len());

    bytes[first..].to_vec()
}

fn bitwise_binary<F>(
    lhs: &[u8],
    rhs: &[u8],
    operation: F,
) -> UnsignedInteger
where
    F: Fn(u8, u8) -> u8,
{
    let length =
        lhs.len().max(rhs.len());

    let lhs_offset =
        length.saturating_sub(lhs.len());

    let rhs_offset =
        length.saturating_sub(rhs.len());

    let mut result =
        vec![0u8; length];

    for index in 0..length {
        let lhs_byte =
            if index >= lhs_offset {
                lhs[index - lhs_offset]
            } else {
                0
            };

        let rhs_byte =
            if index >= rhs_offset {
                rhs[index - rhs_offset]
            } else {
                0
            };

        result[index] =
            operation(lhs_byte, rhs_byte);
    }

    UnsignedInteger::from_be_bytes_owned(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_canonical() {
        let zero =
            UnsignedInteger::zero();

        assert!(zero.is_zero());
        assert_eq!(
            zero.as_be_bytes(),
            &[]
        );
        assert_eq!(
            zero.bit_width(),
            0
        );
    }

    #[test]
    fn leading_zeroes_are_removed() {
        let value =
            UnsignedInteger::from_be_bytes(
                &[0, 0, 1, 2, 3],
            );

        assert_eq!(
            value.as_be_bytes(),
            &[1, 2, 3]
        );
    }

    #[test]
    fn primitive_round_trip() {
        let values = [
            0u128,
            1,
            255,
            256,
            u64::MAX as u128,
            u128::MAX,
        ];

        for value in values {
            let integer =
                UnsignedInteger::from_u128(
                    value
                );

            assert_eq!(
                integer.to_u128().unwrap(),
                value
            );
        }
    }

    #[test]
    fn addition_works() {
        let lhs =
            UnsignedInteger::from_u64(255);

        let rhs =
            UnsignedInteger::from_u64(1);

        let result =
            lhs.checked_add(&rhs).unwrap();

        assert_eq!(
            result.to_u64().unwrap(),
            256
        );
    }

    #[test]
    fn subtraction_works() {
        let lhs =
            UnsignedInteger::from_u64(100);

        let rhs =
            UnsignedInteger::from_u64(37);

        let result =
            lhs.checked_sub(&rhs).unwrap();

        assert_eq!(
            result.to_u64().unwrap(),
            63
        );
    }

    #[test]
    fn subtraction_underflow_is_rejected() {
        let lhs =
            UnsignedInteger::from_u64(1);

        let rhs =
            UnsignedInteger::from_u64(2);

        assert!(
            lhs.checked_sub(&rhs).is_err()
        );
    }

    #[test]
    fn multiplication_works() {
        let lhs =
            UnsignedInteger::from_u64(1234);

        let rhs =
            UnsignedInteger::from_u64(5678);

        let result =
            lhs.checked_mul(&rhs).unwrap();

        assert_eq!(
            result.to_u64().unwrap(),
            1234 * 5678
        );
    }

    #[test]
    fn division_and_remainder_work() {
        let lhs =
            UnsignedInteger::from_u64(100);

        let rhs =
            UnsignedInteger::from_u64(7);

        let (quotient, remainder) =
            lhs.checked_div_rem(&rhs)
                .unwrap();

        assert_eq!(
            quotient.to_u64().unwrap(),
            14
        );

        assert_eq!(
            remainder.to_u64().unwrap(),
            2
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let lhs =
            UnsignedInteger::from_u64(100);

        assert_eq!(
            lhs.checked_div(
                &UnsignedInteger::zero()
            ),
            Err(
                IntegerError::DivisionByZero
            )
        );
    }

    #[test]
    fn left_shift_works() {
        let value =
            UnsignedInteger::from_u64(3);

        let shifted =
            value.checked_shl(4).unwrap();

        assert_eq!(
            shifted.to_u64().unwrap(),
            48
        );
    }

    #[test]
    fn right_shift_works() {
        let value =
            UnsignedInteger::from_u64(48);

        let shifted =
            value.checked_shr(4).unwrap();

        assert_eq!(
            shifted.to_u64().unwrap(),
            3
        );
    }

    #[test]
    fn bit_operations_work() {
        let lhs =
            UnsignedInteger::from_u8(0b1100);

        let rhs =
            UnsignedInteger::from_u8(0b1010);

        assert_eq!(
            lhs.bitand(&rhs)
                .to_u8()
                .unwrap(),
            0b1000
        );

        assert_eq!(
            lhs.bitor(&rhs)
                .to_u8()
                .unwrap(),
            0b1110
        );

        assert_eq!(
            lhs.bitxor(&rhs)
                .to_u8()
                .unwrap(),
            0b0110
        );
    }

    #[test]
    fn bit_access_is_lsb_indexed() {
        let value =
            UnsignedInteger::from_u8(0b1010);

        assert!(!value.bit(0).unwrap());
        assert!(value.bit(1).unwrap());
        assert!(!value.bit(2).unwrap());
        assert!(value.bit(3).unwrap());
    }

    #[test]
    fn signed_positive_and_negative_values_work() {
        let positive =
            SignedInteger::from_i64(42);

        let negative =
            SignedInteger::from_i64(-42);

        assert_eq!(
            positive.to_i128().unwrap(),
            42
        );

        assert_eq!(
            negative.to_i128().unwrap(),
            -42
        );
    }

    #[test]
    fn signed_addition_works() {
        let lhs =
            SignedInteger::from_i64(-10);

        let rhs =
            SignedInteger::from_i64(4);

        let result =
            lhs.checked_add(&rhs)
                .unwrap();

        assert_eq!(
            result.to_i128().unwrap(),
            -6
        );
    }

    #[test]
    fn signed_subtraction_works() {
        let lhs =
            SignedInteger::from_i64(10);

        let rhs =
            SignedInteger::from_i64(14);

        let result =
            lhs.checked_sub(&rhs)
                .unwrap();

        assert_eq!(
            result.to_i128().unwrap(),
            -4
        );
    }

    #[test]
    fn signed_multiplication_works() {
        let lhs =
            SignedInteger::from_i64(-12);

        let rhs =
            SignedInteger::from_i64(5);

        let result =
            lhs.checked_mul(&rhs)
                .unwrap();

        assert_eq!(
            result.to_i128().unwrap(),
            -60
        );
    }

    #[test]
    fn signed_division_truncates_toward_zero() {
        let lhs =
            SignedInteger::from_i64(-17);

        let rhs =
            SignedInteger::from_i64(5);

        let quotient =
            lhs.checked_div(&rhs)
                .unwrap();

        let remainder =
            lhs.checked_rem(&rhs)
                .unwrap();

        assert_eq!(
            quotient.to_i128().unwrap(),
            -3
        );

        assert_eq!(
            remainder.to_i128().unwrap(),
            -2
        );
    }

    #[test]
    fn negative_zero_is_rejected() {
        assert_eq!(
            SignedInteger::from_parts(
                true,
                UnsignedInteger::zero()
            ),
            Err(IntegerError::InvalidSign)
        );
    }

    #[test]
    fn signed_ordering_works() {
        let negative =
            SignedInteger::from_i64(-10);

        let zero =
            SignedInteger::zero();

        let positive =
            SignedInteger::from_i64(10);

        assert!(negative < zero);
        assert!(zero < positive);
        assert!(negative < positive);
    }

    #[test]
    fn signed_unsigned_conversion_works() {
        let value =
            IntegerValue::Unsigned(
                UnsignedInteger::from_u64(123)
            );

        let signed =
            value.to_signed().unwrap();

        assert_eq!(
            signed.to_i128().unwrap(),
            123
        );
    }

    #[test]
    fn negative_unsigned_conversion_is_rejected() {
        let value =
            IntegerValue::Signed(
                SignedInteger::from_i64(-1)
            );

        assert_eq!(
            value.to_unsigned(),
            Err(
                IntegerError::NegativeToUnsigned
            )
        );
    }

    #[test]
    fn width_validation_works() {
        let value =
            UnsignedInteger::from_u64(255);

        assert!(
            value.validate_width(8).is_ok()
        );

        assert!(
            value.validate_width(7).is_err()
        );
    }

    #[test]
    fn arbitrary_width_values_work() {
        let bytes = vec![
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xab, 0xcd, 0xef,
            0x10, 0x20, 0x30, 0x40,
            0x50, 0x60, 0x70, 0x80,
            0x90, 0xa0, 0xb0, 0xc0,
        ];

        let value =
            UnsignedInteger::from_be_bytes(
                &bytes
            );

        assert_eq!(
            value.as_be_bytes(),
            bytes.as_slice()
        );

        assert_eq!(
            value.bit_width(),
            160
        );
    }

    #[test]
    fn decimal_display_works() {
        let value =
            UnsignedInteger::from_u64(
                1_000_000
            );

        assert_eq!(
            value.to_string(),
            "1000000"
        );

        let negative =
            SignedInteger::from_i64(-12345);

        assert_eq!(
            negative.to_string(),
            "-12345"
        );
    }
}