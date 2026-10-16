
//! Zamani — Classical Complex-Number Values
//!
//! File: src/classical/values/complex.rs
//!
//! STATUS
//!   Self-contained implementation; repository-wide module registration,
//!   canonical-value integration, and workspace verification remain pending.
//!
//! PURPOSE
//!   Provide safe, deterministic, target-independent complex scalar values
//!   for classical, scientific, numerical, quantum-adjacent, and hybrid
//!   computation.
//!
//! AUTHORITY AND OWNERSHIP
//!   This file owns Complex32, Complex64, ComplexScalar, ComplexError,
//!   checked complex arithmetic, explicit precision conversion, and the
//!   canonical byte encoding of these concrete representations.
//!
//!   It does not own ClassicalValue, semantic type declarations, source
//!   syntax, parsing, expression trees, memory allocation, execution,
//!   numerical optimization policy, or hardware capabilities.
//!
//! SCALABILITY
//!   No machine-count, array-size, tensor-rank, or hardware-specific limits
//!   are introduced here. IEEE-754 f32 and f64 have finite precision and
//!   range. They are concrete formats, not the semantic ceiling of Zamani.
//!   Resource availability cannot make a finite-width format arbitrary
//!   precision; additional formats require an explicit representation.
//!
//! SAFETY
//!   Safe Rust only. No unsafe code.
//!
//! DEPENDENCIES
//!   Rust standard library only.
//!
//! INTEGRATION CONTRACT
//!   1. values/mod.rs declares `pub mod complex` and re-exports the public
//!      API if that matches the finalized module visibility policy.
//!   2. classical/mod.rs exposes values through its established module tree.
//!   3. ClassicalValue integration belongs to the existing canonical value
//!      owner and must not create a second competing value enum.
//!   4. Numeric algorithms consume ComplexScalar and its checked methods;
//!      they must not silently assume that all complex formats are f64.
//!   5. The frontend owns literal parsing and maps valid syntax to checked
//!      constructors. This module does not parse Zamani source text.
//!   6. Backend lowering maps these values to supported target formats and
//!      rejects unsupported precision or range explicitly.
//!   7. Serialization and hashing must follow the repository's canonical
//!      serialization and hashing policy. The byte encoding below is a
//!      deterministic representation for this module, not a replacement
//!      for a repository-wide serialization protocol.
//!
//! FREEZE RULE
//!   Changes to numeric semantics, canonical bytes, equality, or conversion
//!   behavior require compatibility review and regression tests.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

/// Errors produced by checked complex-number operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplexError {
    /// A component is NaN or positive/negative infinity.
    NonFiniteInput,

    /// An operation produced a non-finite component.
    NonFiniteResult,

    /// Division by a complex number whose two components are zero.
    DivisionByZero,

    /// The requested conversion cannot preserve a finite result.
    ConversionOutOfRange,

    /// The byte representation has the wrong length.
    InvalidEncodingLength {
        /// Required number of bytes.
        expected: usize,
        /// Actual number of bytes.
        actual: usize,
    },

    /// A byte representation contains a non-finite component.
    NonFiniteEncoding,
}

impl fmt::Display for ComplexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput => {
                f.write_str("complex input contains a non-finite component")
            }
            Self::NonFiniteResult => {
                f.write_str("complex operation produced a non-finite result")
            }
            Self::DivisionByZero => {
                f.write_str("division by complex zero")
            }
            Self::ConversionOutOfRange => {
                f.write_str("complex precision conversion is out of range")
            }
            Self::InvalidEncodingLength { expected, actual } => {
                write!(
                    f,
                    "invalid complex encoding length: expected {expected} bytes, got {actual}"
                )
            }
            Self::NonFiniteEncoding => {
                f.write_str("complex encoding contains a non-finite component")
            }
        }
    }
}

impl std::error::Error for ComplexError {}

/// Common checked interface for concrete complex scalar formats.
///
/// Implementations must validate inputs and outputs of checked operations.
/// This trait intentionally does not prescribe a particular hardware format.
pub trait ComplexScalar: Copy + PartialEq + fmt::Debug {
    /// The real scalar type used by this complex representation.
    type Real: Copy + PartialOrd + fmt::Debug;

    /// Constructs a value, rejecting non-finite components.
    fn try_new(real: Self::Real, imaginary: Self::Real)
        -> Result<Self, ComplexError>;

    /// Returns the real component.
    fn real(self) -> Self::Real;

    /// Returns the imaginary component.
    fn imaginary(self) -> Self::Real;

    /// Returns the additive identity.
    fn zero() -> Self;

    /// Returns the multiplicative identity.
    fn one() -> Self;

    /// Tests whether both components are exactly zero.
    fn is_zero(self) -> bool;

    /// Tests whether both components are finite.
    fn is_finite(self) -> bool;

    /// Checked addition.
    fn checked_add(self, rhs: Self) -> Result<Self, ComplexError>;

    /// Checked subtraction.
    fn checked_sub(self, rhs: Self) -> Result<Self, ComplexError>;

    /// Checked multiplication.
    fn checked_mul(self, rhs: Self) -> Result<Self, ComplexError>;

    /// Checked division.
    fn checked_div(self, rhs: Self) -> Result<Self, ComplexError>;

    /// Complex conjugate.
    fn conjugate(self) -> Result<Self, ComplexError>;

    /// Magnitude computed with the scalar format's hypot operation.
    fn magnitude(self) -> Result<Self::Real, ComplexError>;

    /// Squared magnitude. May overflow even when the components are finite.
    fn checked_norm_squared(self) -> Result<Self::Real, ComplexError>;

    /// Returns the principal argument in radians.
    fn argument(self) -> Result<Self::Real, ComplexError>;
}

macro_rules! define_complex {
    (
        $name:ident,
        $real:ty,
        $component_bytes:expr,
        $bits_to_bytes:ident,
        $bytes_to_bits:ident
    ) => {
        /// A finite IEEE-754 complex scalar using the specified component
        /// format.
        ///
        /// Components are private so safe callers cannot bypass checked
        /// construction by assigning NaN or infinity directly.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            real: $real,
            imaginary: $real,
        }

        impl $name {
            /// Number of bytes in this concrete canonical encoding.
            pub const ENCODED_LEN: usize = 2 * $component_bytes;

            /// Constructs a finite complex number.
            pub fn new(
                real: $real,
                imaginary: $real,
            ) -> Result<Self, ComplexError> {
                if !real.is_finite() || !imaginary.is_finite() {
                    return Err(ComplexError::NonFiniteInput);
                }

                Ok(Self { real, imaginary })
            }

            /// Real component.
            #[must_use]
            pub const fn real(self) -> $real {
                self.real
            }

            /// Imaginary component.
            #[must_use]
            pub const fn imaginary(self) -> $real {
                self.imaginary
            }

            /// Additive identity.
            #[must_use]
            pub const fn zero() -> Self {
                Self {
                    real: 0.0,
                    imaginary: 0.0,
                }
            }

            /// Multiplicative identity.
            #[must_use]
            pub const fn one() -> Self {
                Self {
                    real: 1.0,
                    imaginary: 0.0,
                }
            }

            /// Returns true only when both components compare equal to zero.
            #[must_use]
            pub fn is_zero(self) -> bool {
                self.real == 0.0 && self.imaginary == 0.0
            }

            /// Tests both components for finiteness.
            #[must_use]
            pub fn is_finite(self) -> bool {
                self.real.is_finite() && self.imaginary.is_finite()
            }

            /// Validates the invariant, useful at trust boundaries.
            pub fn validate(self) -> Result<(), ComplexError> {
                if self.is_finite() {
                    Ok(())
                } else {
                    Err(ComplexError::NonFiniteInput)
                }
            }

            /// Checked addition.
            pub fn checked_add(self, rhs: Self) -> Result<Self, ComplexError> {
                Self::new(
                    self.real + rhs.real,
                    self.imaginary + rhs.imaginary,
                )
                .map_err(|error| match error {
                    ComplexError::NonFiniteInput => ComplexError::NonFiniteResult,
                    other => other,
                })
            }

            /// Checked subtraction.
            pub fn checked_sub(self, rhs: Self) -> Result<Self, ComplexError> {
                Self::new(
                    self.real - rhs.real,
                    self.imaginary - rhs.imaginary,
                )
                .map_err(|error| match error {
                    ComplexError::NonFiniteInput => ComplexError::NonFiniteResult,
                    other => other,
                })
            }

            /// Checked multiplication.
            ///
            /// Intermediate arithmetic follows the component format's
            /// rounding rules. If the result is non-finite, the operation
            /// returns an error rather than silently accepting infinity/NaN.
            pub fn checked_mul(self, rhs: Self) -> Result<Self, ComplexError> {
                let real =
                    self.real * rhs.real - self.imaginary * rhs.imaginary;
                let imaginary =
                    self.real * rhs.imaginary + self.imaginary * rhs.real;

                Self::new(real, imaginary).map_err(|error| match error {
                    ComplexError::NonFiniteInput => ComplexError::NonFiniteResult,
                    other => other,
                })
            }

            /// Checked complex division using a scaled ratio algorithm.
            ///
            /// This avoids the common avoidable overflow caused by directly
            /// forming `rhs.real² + rhs.imaginary²`. It cannot prevent every
            /// possible overflow or underflow inherent in finite-width
            /// IEEE-754 arithmetic; those cases are reported or rounded
            /// according to the format.
            pub fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
                if rhs.is_zero() {
                    return Err(ComplexError::DivisionByZero);
                }

                let a = self.real;
                let b = self.imaginary;
                let c = rhs.real;
                let d = rhs.imaginary;

                let (real, imaginary) = if c.abs() >= d.abs() {
                    let ratio = d / c;
                    let denominator = c + d * ratio;
                    (
                        (a + b * ratio) / denominator,
                        (b - a * ratio) / denominator,
                    )
                } else {
                    let ratio = c / d;
                    let denominator = d + c * ratio;
                    (
                        (a * ratio + b) / denominator,
                        (b * ratio - a) / denominator,
                    )
                };

                Self::new(real, imaginary).map_err(|error| match error {
                    ComplexError::NonFiniteInput => ComplexError::NonFiniteResult,
                    other => other,
                })
            }

            /// Complex conjugate, with signed-zero behavior inherited from
            /// the component format.
            pub fn conjugate(self) -> Result<Self, ComplexError> {
                Self::new(self.real, -self.imaginary)
            }

            /// Magnitude computed using `hypot` to reduce avoidable
            /// intermediate overflow and underflow.
            pub fn magnitude(self) -> Result<$real, ComplexError> {
                let value = self.real.hypot(self.imaginary);
                if value.is_finite() {
                    Ok(value)
                } else {
                    Err(ComplexError::NonFiniteResult)
                }
            }

            /// Squared magnitude.
            ///
            /// Unlike `magnitude`, this operation can overflow for finite
            /// components because the squared result may exceed the format's
            /// representable range.
            pub fn checked_norm_squared(self) -> Result<$real, ComplexError> {
                let value =
                    self.real * self.real + self.imaginary * self.imaginary;

                if value.is_finite() {
                    Ok(value)
                } else {
                    Err(ComplexError::NonFiniteResult)
                }
            }

            /// Principal argument in radians, in the range returned by
            /// the underlying `atan2` implementation.
            pub fn argument(self) -> Result<$real, ComplexError> {
                if self.is_zero() {
                    return Err(ComplexError::DivisionByZero);
                }

                let value = self.imaginary.atan2(self.real);
                if value.is_finite() {
                    Ok(value)
                } else {
                    Err(ComplexError::NonFiniteResult)
                }
            }

            /// Explicit conversion to the other built-in precision.
            ///
            /// Conversion may lose precision. The conversion is not
            /// represented as exact or lossless.
            pub fn to_f64(self) -> Result<Complex64, ComplexError>
            where
                $name: IntoComplex64,
            {
                self.into_complex64()
            }

            /// Deterministic fixed-width byte representation:
            /// real component followed by imaginary component, each in
            /// little-endian IEEE-754 bit order.
            ///
            /// This format is local to these concrete types. A repository
            /// serialization protocol must version and wrap it as required.
            #[must_use]
            pub fn canonical_bytes(self) -> [u8; Self::ENCODED_LEN] {
                let mut bytes = [0_u8; Self::ENCODED_LEN];
                let real = $bits_to_bytes(self.real);
                let imaginary = $bits_to_bytes(self.imaginary);

                let mut i = 0;
                while i < $component_bytes {
                    bytes[i] = real[i];
                    bytes[$component_bytes + i] = imaginary[i];
                    i += 1;
                }

                bytes
            }

            /// Decodes the fixed-width little-endian representation.
            ///
            /// Non-finite values are rejected, so decoding cannot bypass
            /// the constructor invariant.
            pub fn from_canonical_bytes(
                bytes: &[u8],
            ) -> Result<Self, ComplexError> {
                if bytes.len() != Self::ENCODED_LEN {
                    return Err(ComplexError::InvalidEncodingLength {
                        expected: Self::ENCODED_LEN,
                        actual: bytes.len(),
                    });
                }

                let mut real_bytes = [0_u8; $component_bytes];
                let mut imaginary_bytes = [0_u8; $component_bytes];

                let mut i = 0;
                while i < $component_bytes {
                    real_bytes[i] = bytes[i];
                    imaginary_bytes[i] = bytes[$component_bytes + i];
                    i += 1;
                }

                let real = $bytes_to_bits(real_bytes);
                let imaginary = $bytes_to_bits(imaginary_bytes);

                Self::new(real, imaginary)
                    .map_err(|_| ComplexError::NonFiniteEncoding)
            }
        }

        impl ComplexScalar for $name {
            type Real = $real;

            fn try_new(
                real: Self::Real,
                imaginary: Self::Real,
            ) -> Result<Self, ComplexError> {
                Self::new(real, imaginary)
            }

            fn real(self) -> Self::Real {
                self.real
            }

            fn imaginary(self) -> Self::Real {
                self.imaginary
            }

            fn zero() -> Self {
                Self::zero()
            }

            fn one() -> Self {
                Self::one()
            }

            fn is_zero(self) -> bool {
                self.is_zero()
            }

            fn is_finite(self) -> bool {
                self.is_finite()
            }

            fn checked_add(self, rhs: Self) -> Result<Self, ComplexError> {
                self.checked_add(rhs)
            }

            fn checked_sub(self, rhs: Self) -> Result<Self, ComplexError> {
                self.checked_sub(rhs)
            }

            fn checked_mul(self, rhs: Self) -> Result<Self, ComplexError> {
                self.checked_mul(rhs)
            }

            fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
                self.checked_div(rhs)
            }

            fn conjugate(self) -> Result<Self, ComplexError> {
                self.conjugate()
            }

            fn magnitude(self) -> Result<Self::Real, ComplexError> {
                self.magnitude()
            }

            fn checked_norm_squared(self) -> Result<Self::Real, ComplexError> {
                self.checked_norm_squared()
            }

            fn argument(self) -> Result<Self::Real, ComplexError> {
                self.argument()
            }
        }
    };
}

/// Converts a concrete complex value into binary64 precision.
pub trait IntoComplex64 {
    /// Converts components to f64, rejecting non-finite converted values.
    fn into_complex64(self) -> Result<Complex64, ComplexError>;
}

define_complex!(
    Complex32,
    f32,
    4,
    f32_to_le_bytes,
    f32_from_le_bytes
);

define_complex!(
    Complex64,
    f64,
    8,
    f64_to_le_bytes,
    f64_from_le_bytes
);

fn f32_to_le_bytes(value: f32) -> [u8; 4] {
    value.to_bits().to_le_bytes()
}

fn f32_from_le_bytes(bytes: [u8; 4]) -> f32 {
    f32::from_bits(u32::from_le_bytes(bytes))
}

fn f64_to_le_bytes(value: f64) -> [u8; 8] {
    value.to_bits().to_le_bytes()
}

fn f64_from_le_bytes(bytes: [u8; 8]) -> f64 {
    f64::from_bits(u64::from_le_bytes(bytes))
}

impl IntoComplex64 for Complex32 {
    fn into_complex64(self) -> Result<Complex64, ComplexError> {
        Complex64::new(
            f64::from(self.real),
            f64::from(self.imaginary),
        )
        .map_err(|_| ComplexError::ConversionOutOfRange)
    }
}

impl IntoComplex64 for Complex64 {
    fn into_complex64(self) -> Result<Complex64, ComplexError> {
        Complex64::new(self.real, self.imaginary)
            .map_err(|_| ComplexError::ConversionOutOfRange)
    }
}

impl Complex64 {
    /// Explicit conversion from binary64 to binary32 precision.
    ///
    /// This conversion can lose precision or range. It rejects finite
    /// binary64 components that become non-finite in binary32.
    pub fn to_f32(self) -> Result<Complex32, ComplexError> {
        let real = self.real as f32;
        let imaginary = self.imaginary as f32;

        Complex32::new(real, imaginary)
            .map_err(|_| ComplexError::ConversionOutOfRange)
    }
}

#[cfg(test)]
mod tests {
    use super::{Complex32, Complex64, ComplexError, ComplexScalar};

    #[test]
    fn constructors_reject_non_finite_components() {
        assert_eq!(
            Complex64::new(f64::NAN, 0.0),
            Err(ComplexError::NonFiniteInput)
        );
        assert_eq!(
            Complex64::new(0.0, f64::INFINITY),
            Err(ComplexError::NonFiniteInput)
        );
        assert_eq!(
            Complex32::new(f32::NEG_INFINITY, 0.0),
            Err(ComplexError::NonFiniteInput)
        );
    }

    #[test]
    fn checked_arithmetic_returns_expected_values() {
        let a = Complex64::new(3.0, 2.0).expect("finite components");
        let b = Complex64::new(1.0, -4.0).expect("finite components");

        assert_eq!(
            a.checked_add(b).expect("finite sum"),
            Complex64::new(4.0, -2.0).expect("finite components")
        );
        assert_eq!(
            a.checked_sub(b).expect("finite difference"),
            Complex64::new(2.0, 6.0).expect("finite components")
        );
        assert_eq!(
            a.checked_mul(b).expect("finite product"),
            Complex64::new(11.0, -10.0).expect("finite components")
        );
    }

    #[test]
    fn division_by_zero_is_explicit() {
        let value = Complex64::one();
        assert_eq!(
            value.checked_div(Complex64::zero()),
            Err(ComplexError::DivisionByZero)
        );
    }

    #[test]
    fn division_avoids_naive_squared_denominator_overflow() {
        let a = Complex64::new(1.0e300, 1.0e300)
            .expect("finite components");
        let b = Complex64::new(1.0e300, -1.0e300)
            .expect("finite components");

        let quotient = a.checked_div(b).expect("finite quotient");
        assert!((quotient.real() - 0.0).abs() < 1.0e-12);
        assert!((quotient.imaginary() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn magnitude_uses_hypot() {
        let value = Complex64::new(3.0, 4.0).expect("finite components");
        assert_eq!(value.magnitude().expect("finite magnitude"), 5.0);
    }

    #[test]
    fn norm_squared_reports_overflow() {
        let value = Complex64::new(f64::MAX / 2.0, f64::MAX / 2.0)
            .expect("finite components");
        assert_eq!(
            value.checked_norm_squared(),
            Err(ComplexError::NonFiniteResult)
        );
    }

    #[test]
    fn canonical_bytes_round_trip() {
        let value = Complex64::new(-12.5, 0.25).expect("finite components");
        let restored = Complex64::from_canonical_bytes(&value.canonical_bytes())
            .expect("valid encoding");

        assert_eq!(value, restored);
    }

    #[test]
    fn invalid_encoding_length_is_rejected() {
        assert_eq!(
            Complex64::from_canonical_bytes(&[0_u8; 3]),
            Err(ComplexError::InvalidEncodingLength {
                expected: 16,
                actual: 3,
            })
        );
    }

    #[test]
    fn non_finite_encoding_is_rejected() {
        let invalid = Complex64 {
            real: f64::NAN,
            imaginary: 0.0,
        }
        .canonical_bytes();

        assert_eq!(
            Complex64::from_canonical_bytes(&invalid),
            Err(ComplexError::NonFiniteEncoding)
        );
    }

    #[test]
    fn precision_conversion_is_explicit() {
        let original = Complex32::new(1.25, -2.5).expect("finite components");
        let widened = original.to_f64().expect("widening is representable");

        assert_eq!(widened.real(), 1.25_f64);
        assert_eq!(widened.imaginary(), -2.5_f64);

        let narrowed = widened.to_f32().expect("components are representable");
        assert_eq!(narrowed, original);
    }

    #[test]
    fn conversion_rejects_overflow_to_f32() {
        let value = Complex64::new(f64::MAX, 0.0).expect("finite components");
        assert_eq!(
            value.to_f32(),
            Err(ComplexError::ConversionOutOfRange)
        );
    }

    #[test]
    fn scalar_trait_works_for_both_formats() {
        fn identity<T: ComplexScalar>() -> T {
            T::one()
        }

        assert!(identity::<Complex32>().is_finite());
        assert!(identity::<Complex64>().is_finite());
    }

    #[test]
    fn signed_zero_is_preserved_in_encoding() {
        let value = Complex64::new(-0.0, 0.0).expect("finite components");
        let bytes = value.canonical_bytes();
        let restored =
            Complex64::from_canonical_bytes(&bytes).expect("valid encoding");

        assert_eq!(restored.real().to_bits(), (-0.0_f64).to_bits());
    }
}
