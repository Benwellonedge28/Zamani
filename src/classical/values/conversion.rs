
 //! Zamani — Classical Value Conversion Policies
 //!
 //! File: src/classical/values/conversion.rs
 //! Rust: 1.97+; edition 2021.
 //!
 //! PURPOSE
 //!   Provide a shared policy boundary for explicit conversions between
 //!   canonical classical values and Rust primitive interoperability types.
 //!
 //! AUTHORITY AND OWNERSHIP
 //!   This module owns conversion policy, conversion diagnostics, and the
 //!   TryConvertTo contract. It does not own numeric representations.
 //!
 //!   Canonical representations remain owned by:
 //!     crate::quantum::ir::classical::integer
 //!     crate::quantum::ir::classical::float
 //!     crate::quantum::ir::classical::value
 //!
 //! DOES NOT OWN
 //!   - source syntax, tokenization, or AST nodes;
 //!   - semantic type identities or type inference;
 //!   - ClassicalValue or a second value enum;
 //!   - decimal/rational/complex arithmetic implementations;
 //!   - canonical serialization or hashing;
 //!   - runtime execution, allocation, scheduling, or hardware selection.
 //!
 //! POCO-REAF
 //!   No universal machine-width or hardware-count limits are introduced.
 //!   Conversions to a fixed-width Rust type are explicitly bounded by that
 //!   destination type. Arbitrary-width values remain arbitrary-width when
 //!   represented by the canonical integer implementation.
 //!
 //! INTEGRATION
 //!   - Declare this module from src/classical/values/mod.rs.
 //!   - Re-export its public API there according to the finalized visibility
 //!     policy.
 //!   - Keep canonical integer and float types owned by quantum::ir::classical.
 //!   - Callers must choose a policy explicitly when loss, rounding, or
 //!     overflow behavior matters.
 //!   - Semantic analysis must reject incompatible source-level conversions
 //!     before lowering; this module is not a substitute for type checking.
 //!   - Do not add conversion arms to a competing ClassicalValue enum here.
 //!
 //! SAFETY
 //!   No unsafe Rust. No silent truncation, wrapping, or lossy conversion
 //!   under the default policy.

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

/// Controls how a conversion handles a value outside the destination range.
///
/// `Saturate` is an explicit interoperability choice. It must not be used
/// implicitly by semantic analysis or canonical IR construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OverflowPolicy {
    /// Return an error if the destination cannot represent the value.
    #[default]
    Reject,

    /// Clamp to the destination's nearest representable endpoint.
    Saturate,
}

/// Controls rounding when converting a floating-point value to an integer.
///
/// Integer-to-integer conversions are exact and do not use this setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RoundingMode {
    /// Reject fractional values.
    #[default]
    Exact,

    /// Discard the fractional component toward zero.
    TowardZero,

    /// Round to the nearest integer, with halfway cases away from zero.
    NearestTiesAway,

    /// Round toward negative infinity.
    Floor,

    /// Round toward positive infinity.
    Ceiling,
}

/// Controls whether a conversion may discard numeric information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PrecisionPolicy {
    /// Reject conversions that cannot guarantee exact representation.
    #[default]
    Exact,

    /// Permit loss where a specific conversion implementation documents it.
    AllowLoss,
}

/// Policy supplied to a conversion operation.
///
/// Defaults are conservative: reject overflow, reject rounding, and preserve
/// precision. The policy never changes the meaning of a source-language type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConversionPolicy {
    /// Overflow behavior.
    pub overflow: OverflowPolicy,

    /// Floating-point-to-integer rounding behavior.
    pub rounding: RoundingMode,

    /// Precision-loss behavior.
    pub precision: PrecisionPolicy,
}

impl ConversionPolicy {
    /// Returns the conservative policy: exact, rejecting overflow.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            overflow: OverflowPolicy::Reject,
            rounding: RoundingMode::Exact,
            precision: PrecisionPolicy::Exact,
        }
    }

    /// Returns a policy that allows range clamping but no rounding or
    /// precision loss.
    #[must_use]
    pub const fn saturating() -> Self {
        Self {
            overflow: OverflowPolicy::Saturate,
            rounding: RoundingMode::Exact,
            precision: PrecisionPolicy::Exact,
        }
    }
}

/// Failures raised by conversion adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionError {
    /// The value is outside the destination type's range.
    Overflow,

    /// The source has a fractional component and exact conversion was asked
    /// for, or a requested rounding operation cannot be applied.
    Inexact,

    /// The value cannot be represented exactly in the destination format.
    PrecisionLoss,

    /// A negative value cannot be represented by an unsigned destination.
    NegativeToUnsigned,

    /// The source value is invalid for the requested conversion.
    InvalidSource,

    /// The conversion is not implemented for this pair of representations.
    Unsupported,

    /// A canonical integer operation failed.
    Integer(IntegerError),

    /// A canonical floating-point operation failed.
    Float(ClassicalFloatError),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow => formatter.write_str("conversion overflow"),
            Self::Inexact => formatter.write_str(
                "conversion requires rounding or would discard a fractional component",
            ),
            Self::PrecisionLoss => formatter.write_str(
                "conversion cannot preserve the source value exactly",
            ),
            Self::NegativeToUnsigned => formatter.write_str(
                "negative value cannot be converted to an unsigned type",
            ),
            Self::InvalidSource => formatter.write_str(
                "source value is invalid for the requested conversion",
            ),
            Self::Unsupported => formatter.write_str(
                "conversion is not supported for this source and destination",
            ),
            Self::Integer(error) => write!(formatter, "integer conversion failed: {error}"),
            Self::Float(error) => write!(formatter, "floating-point conversion failed: {error}"),
        }
    }
}

impl std::error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            Self::Float(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IntegerError> for ConversionError {
    fn from(error: IntegerError) -> Self {
        Self::Integer(error)
    }
}

impl From<ClassicalFloatError> for ConversionError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Float(error)
    }
}

/// An explicit conversion from one representation to another.
///
/// Implementations must document whether they preserve the complete source
/// value. A conversion must never silently wrap or truncate.
///
/// This trait deliberately avoids a universal value enum. Additional
/// representations can implement it without modifying existing conversions.
pub trait TryConvertTo<T> {
    /// Converts `self` to `T` using the supplied policy.
    fn try_convert_to(
        &self,
        policy: ConversionPolicy,
    ) -> Result<T, ConversionError>;
}

/// Converts a canonical signed arbitrary-width integer to `i128`.
///
/// `i128` is an interoperability destination, not Zamani's integer ceiling.
impl TryConvertTo<i128> for SignedInteger {
    fn try_convert_to(
        &self,
        policy: ConversionPolicy,
    ) -> Result<i128, ConversionError> {
        match self.to_i128() {
            Ok(value) => Ok(value),
            Err(error) => match policy.overflow {
                OverflowPolicy::Reject => Err(ConversionError::Integer(error)),
                OverflowPolicy::Saturate => {
                    if self.is_negative() {
                        Ok(i128::MIN)
                    } else {
                        Ok(i128::MAX)
                    }
                }
            },
        }
    }
}

/// Converts a canonical unsigned arbitrary-width integer to `u128`.
impl TryConvertTo<u128> for UnsignedInteger {
    fn try_convert_to(
        &self,
        policy: ConversionPolicy,
    ) -> Result<u128, ConversionError> {
        match self.to_u128() {
            Ok(value) => Ok(value),
            Err(error) => match policy.overflow {
                OverflowPolicy::Reject => Err(ConversionError::Integer(error)),
                OverflowPolicy::Saturate => Ok(u128::MAX),
            },
        }
    }
}

/// Converts a signed canonical integer to an unsigned canonical integer.
///
/// This conversion remains arbitrary-width. A negative value is always
/// rejected; saturation does not change its sign or reinterpret its bits.
impl TryConvertTo<UnsignedInteger> for SignedInteger {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<UnsignedInteger, ConversionError> {
        self.to_unsigned().map_err(|error| match error {
            IntegerError::NegativeToUnsigned => ConversionError::NegativeToUnsigned,
            other => ConversionError::Integer(other),
        })
    }
}

/// Converts an unsigned canonical integer to a signed canonical integer.
///
/// The destination is arbitrary-width, so this does not narrow to a machine
/// integer. The canonical implementation owns the conversion details.
impl TryConvertTo<SignedInteger> for UnsignedInteger {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<SignedInteger, ConversionError> {
        self.to_signed().map_err(ConversionError::Integer)
    }
}

/// Converts a signed primitive integer to the canonical arbitrary-width form.
impl TryConvertTo<SignedInteger> for i128 {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<SignedInteger, ConversionError> {
        Ok(SignedInteger::from_i128(*self))
    }
}

/// Converts an unsigned primitive integer to the canonical arbitrary-width form.
impl TryConvertTo<UnsignedInteger> for u128 {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<UnsignedInteger, ConversionError> {
        Ok(UnsignedInteger::from_u128(*self))
    }
}

/// Converts a canonical finite float to its `f64` interoperability value.
///
/// The canonical type guarantees finiteness at construction time.
impl TryConvertTo<f64> for ClassicalFloat {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<f64, ConversionError> {
        Ok(self.get())
    }
}

/// Converts `f64` to the canonical finite float representation.
///
/// NaN and infinities are rejected by the canonical implementation.
impl TryConvertTo<ClassicalFloat> for f64 {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<ClassicalFloat, ConversionError> {
        ClassicalFloat::new(*self).map_err(ConversionError::Float)
    }
}

/// Converts `f32` to canonical binary64.
///
/// Every finite IEEE-754 binary32 value is exactly representable in binary64.
/// Non-finite inputs remain invalid.
impl TryConvertTo<ClassicalFloat> for f32 {
    fn try_convert_to(
        &self,
        _policy: ConversionPolicy,
    ) -> Result<ClassicalFloat, ConversionError> {
        ClassicalFloat::new(f64::from(*self)).map_err(ConversionError::Float)
    }
}

/// Converts a canonical float to a signed machine integer.
///
/// Rounding is explicit. Range checks are performed before the cast so Rust's
/// saturating float-to-integer cast semantics cannot silently decide Zamani
/// conversion behavior.
impl TryConvertTo<i64> for ClassicalFloat {
    fn try_convert_to(
        &self,
        policy: ConversionPolicy,
    ) -> Result<i64, ConversionError> {
        let original = self.get();
        let rounded = round_float(original, policy.rounding)?;

        // The upper bound is exclusive because `i64::MAX as f64` rounds to
        // 2^63, which is outside the signed i64 range.
        let upper_exclusive = 9_223_372_036_854_775_808.0_f64;
        let lower_inclusive = -9_223_372_036_854_775_808.0_f64;

        if rounded < lower_inclusive || rounded >= upper_exclusive {
            return match policy.overflow {
                OverflowPolicy::Reject => Err(ConversionError::Overflow),
                OverflowPolicy::Saturate if rounded.is_sign_negative() => Ok(i64::MIN),
                OverflowPolicy::Saturate => Ok(i64::MAX),
            };
        }

        if policy.rounding == RoundingMode::Exact && rounded != original {
            return Err(ConversionError::Inexact);
        }

        Ok(rounded as i64)
    }
}

/// Converts a canonical float to an unsigned machine integer.
impl TryConvertTo<u64> for ClassicalFloat {
    fn try_convert_to(
        &self,
        policy: ConversionPolicy,
    ) -> Result<u64, ConversionError> {
        let original = self.get();
        let rounded = round_float(original, policy.rounding)?;

        // 2^64 is exactly representable as f64; it is an exclusive bound.
        let upper_exclusive = 18_446_744_073_709_551_616.0_f64;

        if rounded < 0.0 || rounded >= upper_exclusive {
            return match policy.overflow {
                OverflowPolicy::Reject if rounded < 0.0 => {
                    Err(ConversionError::NegativeToUnsigned)
                }
                OverflowPolicy::Reject => Err(ConversionError::Overflow),
                OverflowPolicy::Saturate if rounded < 0.0 => Ok(0),
                OverflowPolicy::Saturate => Ok(u64::MAX),
            };
        }

        if policy.rounding == RoundingMode::Exact && rounded != original {
            return Err(ConversionError::Inexact);
        }

        Ok(rounded as u64)
    }
}

fn round_float(
    value: f64,
    mode: RoundingMode,
) -> Result<f64, ConversionError> {
    if !value.is_finite() {
        return Err(ConversionError::InvalidSource);
    }

    let result = match mode {
        RoundingMode::Exact | RoundingMode::TowardZero => value.trunc(),
        RoundingMode::NearestTiesAway => value.round(),
        RoundingMode::Floor => value.floor(),
        RoundingMode::Ceiling => value.ceil(),
    };

    if !result.is_finite() {
        return Err(ConversionError::Overflow);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{
        ConversionError,
        ConversionPolicy,
        OverflowPolicy,
        PrecisionPolicy,
        RoundingMode,
        TryConvertTo,
    };
    use crate::quantum::ir::classical::float::ClassicalFloat;
    use crate::quantum::ir::classical::integer::{
        SignedInteger,
        UnsignedInteger,
    };

    #[test]
    fn default_policy_is_conservative() {
        let policy = ConversionPolicy::default();

        assert_eq!(policy.overflow, OverflowPolicy::Reject);
        assert_eq!(policy.rounding, RoundingMode::Exact);
        assert_eq!(policy.precision, PrecisionPolicy::Exact);
    }

    #[test]
    fn primitive_integers_convert_to_arbitrary_width_values() {
        let signed = i128::MIN
            .try_convert_to(ConversionPolicy::exact())
            .expect("all i128 values fit the arbitrary-width signed form");

        let unsigned = u128::MAX
            .try_convert_to(ConversionPolicy::exact())
            .expect("all u128 values fit the arbitrary-width unsigned form");

        assert_eq!(signed.to_i128(), Ok(i128::MIN));
        assert_eq!(unsigned.to_u128(), Ok(u128::MAX));
    }

    #[test]
    fn arbitrary_width_integer_to_primitive_is_checked() {
        let value = UnsignedInteger::from_be_bytes(&[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);

        assert_eq!(
            value.try_convert_to(ConversionPolicy::exact()),
            Err(ConversionError::Integer(
                crate::quantum::ir::classical::integer::IntegerError::ConversionOverflow
            ))
        );
    }

    #[test]
    fn signed_to_unsigned_rejects_negative_values() {
        let value = SignedInteger::from_i128(-1);

        assert_eq!(
            value.try_convert_to::<UnsignedInteger>(ConversionPolicy::exact()),
            Err(ConversionError::NegativeToUnsigned)
        );
    }

    #[test]
    fn unsigned_to_signed_preserves_arbitrary_width() {
        let value = UnsignedInteger::from_be_bytes(&[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);

        let converted: SignedInteger = value
            .try_convert_to(ConversionPolicy::exact())
            .expect("canonical signed representation is arbitrary-width");

        assert_eq!(converted.is_negative(), false);
        assert!(converted.bit_width() > 128);
    }

    #[test]
    fn f32_widens_to_f64_without_precision_loss() {
        let source = 0.1_f32;
        let converted: ClassicalFloat = source
            .try_convert_to(ConversionPolicy::exact())
            .expect("finite f32 widens exactly");

        assert_eq!(converted.get(), f64::from(source));
    }

    #[test]
    fn non_finite_float_inputs_are_rejected() {
        assert_eq!(
            f64::NAN.try_convert_to::<ClassicalFloat>(ConversionPolicy::exact()),
            Err(ConversionError::Float(
                crate::quantum::ir::classical::float::ClassicalFloatError::NonFinite
            ))
        );
    }

    #[test]
    fn float_to_integer_requires_explicit_rounding() {
        let value = ClassicalFloat::new(3.5).expect("finite value");

        assert_eq!(
            value.try_convert_to::<i64>(ConversionPolicy::exact()),
            Err(ConversionError::Inexact)
        );

        let policy = ConversionPolicy {
            overflow: OverflowPolicy::Reject,
            rounding: RoundingMode::NearestTiesAway,
            precision: PrecisionPolicy::Exact,
        };

        assert_eq!(value.try_convert_to::<i64>(policy), Ok(4));
    }

    #[test]
    fn float_to_integer_checks_signed_and_unsigned_ranges() {
        let signed = ClassicalFloat::new(-1.0).expect("finite value");
        let unsigned = ClassicalFloat::new(1.0e30).expect("finite value");

        assert_eq!(
            signed.try_convert_to::<u64>(ConversionPolicy::exact()),
            Err(ConversionError::NegativeToUnsigned)
        );
        assert_eq!(
            unsigned.try_convert_to::<u64>(ConversionPolicy::exact()),
            Err(ConversionError::Overflow)
        );
    }

    #[test]
    fn saturation_is_opt_in() {
        let value = ClassicalFloat::new(1.0e30).expect("finite value");

        assert_eq!(
            value.try_convert_to::<u64>(ConversionPolicy::exact()),
            Err(ConversionError::Overflow)
        );

        let policy = ConversionPolicy {
            overflow: OverflowPolicy::Saturate,
            rounding: RoundingMode::Exact,
            precision: PrecisionPolicy::Exact,
        };

        assert_eq!(value.try_convert_to::<u64>(policy), Ok(u64::MAX));
    }
}
