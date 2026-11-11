
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Mathematical Foundation
//!
//! Path: `src/stdlib/foundation/math.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide portable, checked, reusable mathematical operations for Zamani.
//!
//! This module is a mathematical API layer. It builds on the numeric
//! foundation and does not create another numeric representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Checked elementary mathematical operations.
//! - Floating-point domain validation.
//! - Finite-input and finite-result validation.
//! - Explicit angle representation.
//! - Mathematical constants for supported real representations.
//! - Checked trigonometric and hyperbolic functions.
//! - Checked roots, powers and logarithms.
//! - Common scalar mathematical utilities.
//!
//! This module does not own:
//!
//! - Numeric literal syntax or parsing.
//! - Semantic numeric types.
//! - Canonical arbitrary-width integer representations.
//! - Decimal, rational, complex or fixed-point representations.
//! - Tensor or matrix storage.
//! - Linear algebra algorithms.
//! - Statistical algorithms.
//! - Symbolic mathematics or theorem proving.
//! - Quantum numerical semantics.
//! - Hardware numeric formats.
//! - Hardware discovery or selection.
//! - Memory allocation or resource scheduling.
//!
//! # Canonical integration
//!
//! Numeric scalar operations:
//!     `crate::stdlib::foundation::numeric`
//!
//! Semantic type authority:
//!     `crate::frontend::semantic::types`
//!
//! Arbitrary-width integer authority:
//!     `crate::quantum::ir::classical::integer`
//!
//! Canonical tensor authority:
//!     `crate::classical::collections::tensor`
//!
//! Existing higher-level mathematical subsystem:
//!     `crate::stdlib::math_foundations`
//!
//! Advanced numerical algorithms:
//!     Future or existing scientific-computing modules.
//!
//! # Numerical policy
//!
//! The checked real-number API accepts only finite operands and finite
//! results. Domain errors are reported explicitly.
//!
//! This policy does not redefine IEEE-754 semantics for every Zamani
//! computation. Callers requiring NaN propagation, infinities, signed-zero
//! distinctions, alternative rounding modes, or specialized numeric formats
//! must use an API whose contract explicitly provides those semantics.
//!
//! # Scalability
//!
//! This module imposes no universal hardware limits, tensor-rank limits,
//! processor limits, device limits, or arbitrary numerical width limits.
//!
//! The primitive `f32` and `f64` implementations are interoperability
//! implementations. They are not the semantic precision ceiling of Zamani.
//!
//! Additional numeric representations can implement `RealMath` when their
//! mathematical semantics satisfy its documented contract.
//!
//! Finite physical resources remain real operational constraints. This
//! module does not promise infinite memory, execution time or precision.
//!
//! # Determinism
//!
//! Operations preserve their documented scalar semantics.
//! This module does not silently parallelize, reassociate or distribute
//! numerical operations.
//!
//! Floating-point results may differ across implementations or platforms
//! where the underlying floating-point contract permits such differences.
//!
//! # Dependencies
//!
//! Uses only the Rust standard library and the existing Zamani
//! `NumericScalar` / `NumericError` interface.

use core::fmt;

use crate::stdlib::foundation::numeric::{
    NumericError,
    NumericScalar,
};

// =============================================================================
// 1. ERRORS
// =============================================================================

/// Structured failures from mathematical operations.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum MathError {
    /// An underlying checked numeric operation failed.
    Numeric(NumericError),

    /// An operand is not finite.
    NonFiniteInput {
        /// Mathematical operation being evaluated.
        function: &'static str,
    },

    /// The operation produced a non-finite result.
    NonFiniteResult {
        /// Mathematical operation being evaluated.
        function: &'static str,
    },

    /// An operand is outside the mathematical domain.
    Domain {
        /// Mathematical operation being evaluated.
        function: &'static str,
        /// Explanation of the domain violation.
        reason: &'static str,
    },

    /// Bounds supplied to a bounded operation are invalid.
    InvalidBounds {
        /// Mathematical operation being evaluated.
        function: &'static str,
    },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(error) => {
                write!(f, "mathematical numeric operation failed: {error}")
            }

            Self::NonFiniteInput { function } => {
                write!(f, "{function} received a non-finite input")
            }

            Self::NonFiniteResult { function } => {
                write!(f, "{function} produced a non-finite result")
            }

            Self::Domain { function, reason } => {
                write!(f, "{function}: {reason}")
            }

            Self::InvalidBounds { function } => {
                write!(f, "{function}: lower bound exceeds upper bound")
            }
        }
    }
}

impl std::error::Error for MathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric(error) => Some(error),
            _ => None,
        }
    }
}

impl From<NumericError> for MathError {
    fn from(error: NumericError) -> Self {
        Self::Numeric(error)
    }
}

// =============================================================================
// 2. REAL NUMBER CONTRACT
// =============================================================================

/// Real-number operations supported by the checked mathematical API.
///
/// Implementations must preserve their representation's mathematical
/// meaning and must not silently substitute another precision.
///
/// The initial built-in implementations are `f32` and `f64`.
///
/// Other real-number representations may implement this trait when they
/// can provide the operations with the documented semantics.
pub trait RealMath: NumericScalar + Copy + PartialOrd {
    /// Returns whether this value is finite.
    fn is_finite(self) -> bool;

    /// Returns whether this value has a negative sign.
    ///
    /// This distinguishes negative zero for floating-point representations.
    fn is_sign_negative(self) -> bool;

    /// Returns the mathematical constant pi in this representation.
    fn pi() -> Self;

    /// Returns the mathematical constant e in this representation.
    fn e() -> Self;

    /// Converts an `f64` constant into this representation.
    ///
    /// Implementations must document any precision loss inherent in
    /// their representation.
    fn from_f64_constant(value: f64) -> Self;

    /// Absolute value.
    fn abs_value(self) -> Self;

    /// Square root.
    fn sqrt_value(self) -> Self;

    /// Cube root.
    fn cbrt_value(self) -> Self;

    /// Exponential function.
    fn exp_value(self) -> Self;

    /// Natural logarithm.
    fn ln_value(self) -> Self;

    /// Logarithm with an arbitrary base.
    fn log_value(self, base: Self) -> Self;

    /// Power with a real exponent.
    fn pow_value(self, exponent: Self) -> Self;

    /// Sine, with input in radians.
    fn sin_value(self) -> Self;

    /// Cosine, with input in radians.
    fn cos_value(self) -> Self;

    /// Tangent, with input in radians.
    fn tan_value(self) -> Self;

    /// Inverse sine, returning radians.
    fn asin_value(self) -> Self;

    /// Inverse cosine, returning radians.
    fn acos_value(self) -> Self;

    /// Inverse tangent, returning radians.
    fn atan_value(self) -> Self;

    /// Two-argument inverse tangent, returning radians.
    fn atan2_value(self, x: Self) -> Self;

    /// Hyperbolic sine.
    fn sinh_value(self) -> Self;

    /// Hyperbolic cosine.
    fn cosh_value(self) -> Self;

    /// Hyperbolic tangent.
    fn tanh_value(self) -> Self;

    /// Hypotenuse.
    fn hypot_value(self, other: Self) -> Self;

    /// Floor.
    fn floor_value(self) -> Self;

    /// Ceiling.
    fn ceil_value(self) -> Self;

    /// Round to nearest integer-valued floating-point number.
    fn round_value(self) -> Self;

    /// Truncate toward zero.
    fn trunc_value(self) -> Self;

    /// Fractional component.
    fn fract_value(self) -> Self;
}

// =============================================================================
// 3. BUILT-IN REAL IMPLEMENTATIONS
// =============================================================================

macro_rules! implement_real_math {
    ($type:ty) => {
        impl RealMath for $type {
            fn is_finite(self) -> bool {
                <$type>::is_finite(self)
            }

            fn is_sign_negative(self) -> bool {
                <$type>::is_sign_negative(self)
            }

            fn pi() -> Self {
                core::$type::consts::PI
            }

            fn e() -> Self {
                core::$type::consts::E
            }

            fn from_f64_constant(value: f64) -> Self {
                value as $type
            }

            fn abs_value(self) -> Self {
                <$type>::abs(self)
            }

            fn sqrt_value(self) -> Self {
                <$type>::sqrt(self)
            }

            fn cbrt_value(self) -> Self {
                <$type>::cbrt(self)
            }

            fn exp_value(self) -> Self {
                <$type>::exp(self)
            }

            fn ln_value(self) -> Self {
                <$type>::ln(self)
            }

            fn log_value(self, base: Self) -> Self {
                <$type>::log(self, base)
            }

            fn pow_value(self, exponent: Self) -> Self {
                <$type>::powf(self, exponent)
            }

            fn sin_value(self) -> Self {
                <$type>::sin(self)
            }

            fn cos_value(self) -> Self {
                <$type>::cos(self)
            }

            fn tan_value(self) -> Self {
                <$type>::tan(self)
            }

            fn asin_value(self) -> Self {
                <$type>::asin(self)
            }

            fn acos_value(self) -> Self {
                <$type>::acos(self)
            }

            fn atan_value(self) -> Self {
                <$type>::atan(self)
            }

            fn atan2_value(self, x: Self) -> Self {
                <$type>::atan2(self, x)
            }

            fn sinh_value(self) -> Self {
                <$type>::sinh(self)
            }

            fn cosh_value(self) -> Self {
                <$type>::cosh(self)
            }

            fn tanh_value(self) -> Self {
                <$type>::tanh(self)
            }

            fn hypot_value(self, other: Self) -> Self {
                <$type>::hypot(self, other)
            }

            fn floor_value(self) -> Self {
                <$type>::floor(self)
            }

            fn ceil_value(self) -> Self {
                <$type>::ceil(self)
            }

            fn round_value(self) -> Self {
                <$type>::round(self)
            }

            fn trunc_value(self) -> Self {
                <$type>::trunc(self)
            }

            fn fract_value(self) -> Self {
                <$type>::fract(self)
            }
        }
    };
}

implement_real_math!(f32);
implement_real_math!(f64);

// =============================================================================
// 4. SHARED VALIDATION
// =============================================================================

#[inline]
fn validate_input<T: RealMath>(
    value: T,
    function: &'static str,
) -> Result<(), MathError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(MathError::NonFiniteInput { function })
    }
}

#[inline]
fn validate_result<T: RealMath>(
    value: T,
    function: &'static str,
) -> Result<T, MathError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(MathError::NonFiniteResult { function })
    }
}

#[inline]
fn validate_two_inputs<T: RealMath>(
    left: T,
    right: T,
    function: &'static str,
) -> Result<(), MathError> {
    validate_input(left, function)?;
    validate_input(right, function)
}

// =============================================================================
// 5. MATHEMATICAL CONSTANTS
// =============================================================================

/// Returns pi in the selected real-number representation.
#[must_use]
pub fn pi<T: RealMath>() -> T {
    T::pi()
}

/// Returns tau (2 * pi) in the selected representation.
pub fn tau<T: RealMath>() -> Result<T, MathError> {
    validate_result(T::pi() + T::pi(), "tau")
}

/// Returns Euler's number in the selected representation.
#[must_use]
pub fn e<T: RealMath>() -> T {
    T::e()
}

// =============================================================================
// 6. EXPLICIT ANGLE REPRESENTATION
// =============================================================================

/// An angle represented in radians.
///
/// Keeping the unit in the type prevents accidental mixing of degrees
/// and radians at API boundaries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle<T> {
    radians: T,
}

impl<T: RealMath> Angle<T> {
    /// Creates an angle from radians.
    pub fn from_radians(radians: T) -> Result<Self, MathError> {
        validate_input(radians, "Angle::from_radians")?;
        Ok(Self { radians })
    }

    /// Creates an angle from degrees.
    pub fn from_degrees(degrees: T) -> Result<Self, MathError> {
        validate_input(degrees, "Angle::from_degrees")?;

        let radians =
            degrees * T::pi() / T::from_f64_constant(180.0);

        validate_result(radians, "Angle::from_degrees")?;

        Ok(Self { radians })
    }

    /// Returns the angle in radians.
    #[must_use]
    pub fn radians(self) -> T {
        self.radians
    }

    /// Returns the angle in degrees.
    pub fn degrees(self) -> Result<T, MathError> {
        let degrees =
            self.radians * T::from_f64_constant(180.0) / T::pi();

        validate_result(degrees, "Angle::degrees")
    }
}

// =============================================================================
// 7. BASIC REAL-NUMBER UTILITIES
// =============================================================================

/// Returns the absolute value of a finite real number.
pub fn abs<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "abs")?;
    validate_result(value.abs_value(), "abs")
}

/// Returns the smaller of two finite real numbers.
pub fn min<T: RealMath>(left: T, right: T) -> Result<T, MathError> {
    validate_two_inputs(left, right, "min")?;

    if left <= right {
        Ok(left)
    } else {
        Ok(right)
    }
}

/// Returns the larger of two finite real numbers.
pub fn max<T: RealMath>(left: T, right: T) -> Result<T, MathError> {
    validate_two_inputs(left, right, "max")?;

    if left >= right {
        Ok(left)
    } else {
        Ok(right)
    }
}

/// Clamps a finite value to an inclusive interval.
///
/// Returns an error if the lower bound is greater than the upper bound.
pub fn clamp<T: RealMath>(
    value: T,
    lower: T,
    upper: T,
) -> Result<T, MathError> {
    validate_input(value, "clamp")?;
    validate_two_inputs(lower, upper, "clamp")?;

    if lower > upper {
        return Err(MathError::InvalidBounds {
            function: "clamp",
        });
    }

    if value < lower {
        Ok(lower)
    } else if value > upper {
        Ok(upper)
    } else {
        Ok(value)
    }
}

/// Computes the sign as -1, 0 or 1.
///
/// Both positive and negative zero return zero.
pub fn sign<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "sign")?;

    if value == T::zero() {
        Ok(T::zero())
    } else if value.is_sign_negative() {
        Ok(T::from_f64_constant(-1.0))
    } else {
        Ok(T::one())
    }
}

// =============================================================================
// 8. ROOTS, EXPONENTIALS AND LOGARITHMS
// =============================================================================

/// Computes the square root of a non-negative finite value.
pub fn sqrt<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "sqrt")?;

    if value < T::zero() {
        return Err(MathError::Domain {
            function: "sqrt",
            reason: "the real square root requires a non-negative operand",
        });
    }

    validate_result(value.sqrt_value(), "sqrt")
}

/// Computes the real cube root of a finite value.
pub fn cbrt<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "cbrt")?;
    validate_result(value.cbrt_value(), "cbrt")
}

/// Computes e raised to a finite exponent.
pub fn exp<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "exp")?;
    validate_result(value.exp_value(), "exp")
}

/// Computes the natural logarithm of a positive finite value.
pub fn ln<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "ln")?;

    if value <= T::zero() {
        return Err(MathError::Domain {
            function: "ln",
            reason: "the real natural logarithm requires a positive operand",
        });
    }

    validate_result(value.ln_value(), "ln")
}

/// Computes the logarithm of a positive value using a valid base.
///
/// The base must be positive and must not equal one.
pub fn log<T: RealMath>(
    value: T,
    base: T,
) -> Result<T, MathError> {
    validate_two_inputs(value, base, "log")?;

    if value <= T::zero() {
        return Err(MathError::Domain {
            function: "log",
            reason: "the logarithm argument must be positive",
        });
    }

    if base <= T::zero() || base == T::one() {
        return Err(MathError::Domain {
            function: "log",
            reason: "the logarithm base must be positive and different from one",
        });
    }

    validate_result(value.log_value(base), "log")
}

/// Computes a real power.
///
/// Negative bases require integral exponents. Zero raised to a
/// non-positive exponent is rejected.
pub fn pow<T: RealMath>(
    base: T,
    exponent: T,
) -> Result<T, MathError> {
    validate_two_inputs(base, exponent, "pow")?;

    if base == T::zero() && exponent <= T::zero() {
        return Err(MathError::Domain {
            function: "pow",
            reason: "zero cannot be raised to a non-positive exponent",
        });
    }

    if base < T::zero() && exponent.fract_value() != T::zero() {
        return Err(MathError::Domain {
            function: "pow",
            reason: "a negative real base requires an integral exponent",
        });
    }

    validate_result(base.pow_value(exponent), "pow")
}

// =============================================================================
// 9. TRIGONOMETRY
// =============================================================================

/// Computes sine from an angle in radians.
pub fn sin<T: RealMath>(angle: Angle<T>) -> Result<T, MathError> {
    validate_result(angle.radians.sin_value(), "sin")
}

/// Computes cosine from an angle in radians.
pub fn cos<T: RealMath>(angle: Angle<T>) -> Result<T, MathError> {
    validate_result(angle.radians.cos_value(), "cos")
}

/// Computes tangent from an angle in radians.
pub fn tan<T: RealMath>(angle: Angle<T>) -> Result<T, MathError> {
    validate_result(angle.radians.tan_value(), "tan")
}

/// Computes inverse sine and returns an angle in radians.
pub fn asin<T: RealMath>(value: T) -> Result<Angle<T>, MathError> {
    validate_input(value, "asin")?;

    if value < T::from_f64_constant(-1.0)
        || value > T::one()
    {
        return Err(MathError::Domain {
            function: "asin",
            reason: "the real inverse sine requires an operand in [-1, 1]",
        });
    }

    let radians = validate_result(value.asin_value(), "asin")?;

    Ok(Angle { radians })
}

/// Computes inverse cosine and returns an angle in radians.
pub fn acos<T: RealMath>(value: T) -> Result<Angle<T>, MathError> {
    validate_input(value, "acos")?;

    if value < T::from_f64_constant(-1.0)
        || value > T::one()
    {
        return Err(MathError::Domain {
            function: "acos",
            reason: "the real inverse cosine requires an operand in [-1, 1]",
        });
    }

    let radians = validate_result(value.acos_value(), "acos")?;

    Ok(Angle { radians })
}

/// Computes inverse tangent and returns an angle in radians.
pub fn atan<T: RealMath>(value: T) -> Result<Angle<T>, MathError> {
    validate_input(value, "atan")?;

    let radians = validate_result(value.atan_value(), "atan")?;

    Ok(Angle { radians })
}

/// Computes the two-argument inverse tangent.
///
/// The result is an angle in radians. The origin is rejected because
/// its direction is mathematically undefined.
pub fn atan2<T: RealMath>(
    y: T,
    x: T,
) -> Result<Angle<T>, MathError> {
    validate_two_inputs(y, x, "atan2")?;

    if x == T::zero() && y == T::zero() {
        return Err(MathError::Domain {
            function: "atan2",
            reason: "direction is undefined at the origin",
        });
    }

    let radians = validate_result(y.atan2_value(x), "atan2")?;

    Ok(Angle { radians })
}

// =============================================================================
// 10. HYPERBOLIC FUNCTIONS
// =============================================================================

/// Computes hyperbolic sine.
pub fn sinh<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "sinh")?;
    validate_result(value.sinh_value(), "sinh")
}

/// Computes hyperbolic cosine.
pub fn cosh<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "cosh")?;
    validate_result(value.cosh_value(), "cosh")
}

/// Computes hyperbolic tangent.
pub fn tanh<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "tanh")?;
    validate_result(value.tanh_value(), "tanh")
}

// =============================================================================
// 11. HYPOTENUSE AND DISTANCE PRIMITIVE
// =============================================================================

/// Computes the hypotenuse using the underlying representation's
/// numerically stable hypotenuse operation.
pub fn hypot<T: RealMath>(
    x: T,
    y: T,
) -> Result<T, MathError> {
    validate_two_inputs(x, y, "hypot")?;
    validate_result(x.hypot_value(y), "hypot")
}

// =============================================================================
// 12. ROUNDING
// =============================================================================

/// Rounds toward negative infinity.
pub fn floor<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "floor")?;
    validate_result(value.floor_value(), "floor")
}

/// Rounds toward positive infinity.
pub fn ceil<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "ceil")?;
    validate_result(value.ceil_value(), "ceil")
}

/// Rounds to the nearest integer-valued floating-point number.
///
/// The built-in `f32` and `f64` implementations use Rust's standard
/// round semantics: halfway cases are rounded away from zero.
pub fn round<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "round")?;
    validate_result(value.round_value(), "round")
}

/// Truncates toward zero.
pub fn trunc<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "trunc")?;
    validate_result(value.trunc_value(), "trunc")
}

/// Returns the fractional component.
pub fn fract<T: RealMath>(value: T) -> Result<T, MathError> {
    validate_input(value, "fract")?;
    validate_result(value.fract_value(), "fract")
}

// =============================================================================
// 13. INTERPOLATION
// =============================================================================

/// Performs checked linear interpolation.
///
/// The parameter is not restricted to [0, 1], so extrapolation is supported.
///
/// This function uses the expression:
///
/// `start + (end - start) * amount`
///
/// Intermediate overflow is reported rather than silently accepted.
pub fn lerp<T: RealMath>(
    start: T,
    end: T,
    amount: T,
) -> Result<T, MathError> {
    validate_input(start, "lerp")?;
    validate_input(end, "lerp")?;
    validate_input(amount, "lerp")?;

    let difference = end - start;

    validate_result(difference, "lerp")?;

    let scaled = difference * amount;

    validate_result(scaled, "lerp")?;

    validate_result(start + scaled, "lerp")
}

// =============================================================================
// 14. GENERIC CHECKED ARITHMETIC FACADE
// =============================================================================

/// Checked addition using the canonical numeric scalar contract.
pub fn add<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, MathError> {
    Ok(left.checked_add(right)?)
}

/// Checked subtraction using the canonical numeric scalar contract.
pub fn subtract<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, MathError> {
    Ok(left.checked_sub(right)?)
}

/// Checked multiplication using the canonical numeric scalar contract.
pub fn multiply<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, MathError> {
    Ok(left.checked_mul(right)?)
}

/// Checked division using the canonical numeric scalar contract.
pub fn divide<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, MathError> {
    Ok(left.checked_div(right)?)
}

/// Checked remainder using the canonical numeric scalar contract.
pub fn remainder<T: NumericScalar>(
    left: &T,
    right: &T,
) -> Result<T, MathError> {
    Ok(left.checked_rem(right)?)
}

/// Checked negation using the canonical numeric scalar contract.
pub fn negate<T: NumericScalar>(
    value: &T,
) -> Result<T, MathError> {
    Ok(value.checked_neg()?)
}

// =============================================================================
// 15. TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON_F64: f64 = 1.0e-12;
    const EPSILON_F32: f32 = 1.0e-5;

    fn approximately_equal_f64(left: f64, right: f64) {
        assert!(
            (left - right).abs() <= EPSILON_F64,
            "expected {left} to be approximately {right}"
        );
    }

    fn approximately_equal_f32(left: f32, right: f32) {
        assert!(
            (left - right).abs() <= EPSILON_F32,
            "expected {left} to be approximately {right}"
        );
    }

    #[test]
    fn constants_are_available_for_both_builtin_real_types() {
        approximately_equal_f64(pi::<f64>(), core::f64::consts::PI);
        approximately_equal_f32(pi::<f32>(), core::f32::consts::PI);

        approximately_equal_f64(e::<f64>(), core::f64::consts::E);
        approximately_equal_f32(e::<f32>(), core::f32::consts::E);

        approximately_equal_f64(
            tau::<f64>().unwrap(),
            core::f64::consts::TAU,
        );
    }

    #[test]
    fn angle_conversions_are_explicit() {
        let angle = Angle::<f64>::from_degrees(180.0).unwrap();

        approximately_equal_f64(
            angle.radians(),
            core::f64::consts::PI,
        );

        approximately_equal_f64(
            angle.degrees().unwrap(),
            180.0,
        );
    }

    #[test]
    fn angle_rejects_non_finite_values() {
        assert!(matches!(
            Angle::<f64>::from_radians(f64::INFINITY),
            Err(MathError::NonFiniteInput { .. })
        ));

        assert!(matches!(
            Angle::<f64>::from_degrees(f64::NAN),
            Err(MathError::NonFiniteInput { .. })
        ));
    }

    #[test]
    fn square_root_checks_its_domain() {
        assert_eq!(sqrt(0.0_f64), Ok(0.0));

        approximately_equal_f64(
            sqrt(9.0_f64).unwrap(),
            3.0,
        );

        assert!(matches!(
            sqrt(-1.0_f64),
            Err(MathError::Domain { .. })
        ));
    }

    #[test]
    fn cube_root_supports_negative_real_values() {
        approximately_equal_f64(
            cbrt(-27.0_f64).unwrap(),
            -3.0,
        );
    }

    #[test]
    fn logarithms_check_their_domains() {
        approximately_equal_f64(
            ln(core::f64::consts::E).unwrap(),
            1.0,
        );

        approximately_equal_f64(
            log(100.0_f64, 10.0).unwrap(),
            2.0,
        );

        assert!(matches!(
            ln(0.0_f64),
            Err(MathError::Domain { .. })
        ));

        assert!(matches!(
            log(10.0_f64, 1.0),
            Err(MathError::Domain { .. })
        ));

        assert!(matches!(
            log(-10.0_f64, 10.0),
            Err(MathError::Domain { .. })
        ));
    }

    #[test]
    fn exponential_overflow_is_reported() {
        assert!(matches!(
            exp(f64::MAX),
            Err(MathError::NonFiniteResult { .. })
        ));
    }

    #[test]
    fn powers_check_real_number_domains() {
        approximately_equal_f64(
            pow(-2.0_f64, 3.0).unwrap(),
            -8.0,
        );

        assert!(matches!(
            pow(-2.0_f64, 0.5),
            Err(MathError::Domain { .. })
        ));

        assert!(matches!(
            pow(0.0_f64, -1.0),
            Err(MathError::Domain { .. })
        ));
    }

    #[test]
    fn trigonometric_functions_use_radians() {
        let half_pi =
            Angle::<f64>::from_radians(core::f64::consts::FRAC_PI_2)
                .unwrap();

        approximately_equal_f64(
            sin(half_pi).unwrap(),
            1.0,
        );

        approximately_equal_f64(
            cos(Angle::from_radians(0.0).unwrap()).unwrap(),
            1.0,
        );
    }

    #[test]
    fn inverse_trigonometric_functions_return_angles() {
        let angle = asin(1.0_f64).unwrap();

        approximately_equal_f64(
            angle.radians(),
            core::f64::consts::FRAC_PI_2,
        );

        assert!(matches!(
            asin(2.0_f64),
            Err(MathError::Domain { .. })
        ));

        assert!(matches!(
            acos(-2.0_f64),
            Err(MathError::Domain { .. })
        ));
    }

    #[test]
    fn atan2_rejects_the_origin() {
        assert!(matches!(
            atan2(0.0_f64, 0.0_f64),
            Err(MathError::Domain { .. })
        ));

        let angle = atan2(1.0_f64, 0.0).unwrap();

        approximately_equal_f64(
            angle.radians(),
            core::f64::consts::FRAC_PI_2,
        );
    }

    #[test]
    fn hyperbolic_functions_are_available() {
        approximately_equal_f64(
            sinh(0.0_f64).unwrap(),
            0.0,
        );

        approximately_equal_f64(
            cosh(0.0_f64).unwrap(),
            1.0,
        );

        approximately_equal_f64(
            tanh(0.0_f64).unwrap(),
            0.0,
        );
    }

    #[test]
    fn hypot_uses_stable_underlying_operation() {
        approximately_equal_f64(
            hypot(3.0_f64, 4.0).unwrap(),
            5.0,
        );
    }

    #[test]
    fn rounding_operations_have_explicit_results() {
        assert_eq!(floor(2.9_f64), Ok(2.0));
        assert_eq!(ceil(2.1_f64), Ok(3.0));
        assert_eq!(round(2.5_f64), Ok(3.0));
        assert_eq!(round(-2.5_f64), Ok(-3.0));
        assert_eq!(trunc(-2.9_f64), Ok(-2.0));
        assert_eq!(fract(2.75_f64), Ok(0.75));
    }

    #[test]
    fn basic_utilities_validate_inputs() {
        assert_eq!(abs(-5.0_f64), Ok(5.0));
        assert_eq!(min(2.0_f64, 3.0), Ok(2.0));
        assert_eq!(max(2.0_f64, 3.0), Ok(3.0));
        assert_eq!(clamp(5.0_f64, 0.0, 3.0), Ok(3.0));
        assert_eq!(sign(-5.0_f64), Ok(-1.0));
        assert_eq!(sign(0.0_f64), Ok(0.0));

        assert!(matches!(
            clamp(1.0_f64, 3.0, 0.0),
            Err(MathError::InvalidBounds { .. })
        ));

        assert!(matches!(
            abs(f64::NAN),
            Err(MathError::NonFiniteInput { .. })
        ));
    }

    #[test]
    fn interpolation_checks_intermediate_overflow() {
        assert_eq!(
            lerp(0.0_f64, 10.0, 0.5),
            Ok(5.0)
        );

        assert!(matches!(
            lerp(-f64::MAX, f64::MAX, 0.5),
            Err(MathError::NonFiniteResult { .. })
        ));
    }

    #[test]
    fn generic_arithmetic_reuses_numeric_foundation() {
        assert_eq!(add(&10_i64, &20_i64), Ok(30));
        assert_eq!(subtract(&20_i64, &10_i64), Ok(10));
        assert_eq!(multiply(&6_i64, &7_i64), Ok(42));
        assert_eq!(divide(&21_i64, &3_i64), Ok(7));
        assert_eq!(remainder(&22_i64, &5_i64), Ok(2));
        assert_eq!(negate(&42_i64), Ok(-42));
    }

    #[test]
    fn generic_arithmetic_preserves_overflow_errors() {
        assert!(matches!(
            add(&i64::MAX, &1_i64),
            Err(MathError::Numeric(
                NumericError::Overflow { .. }
            ))
        ));
    }

    #[test]
    fn f32_operations_use_f32_results() {
        approximately_equal_f32(
            sqrt(9.0_f32).unwrap(),
            3.0,
        );

        approximately_equal_f32(
            sin(
                Angle::<f32>::from_degrees(90.0).unwrap()
            ).unwrap(),
            1.0,
        );
    }

    #[test]
    fn non_finite_inputs_are_rejected() {
        assert!(matches!(
            exp(f64::INFINITY),
            Err(MathError::NonFiniteInput { .. })
        ));

        assert!(matches!(
            hypot(f64::NAN, 1.0),
            Err(MathError::NonFiniteInput { .. })
        ));
    }
}
