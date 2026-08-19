//! Numerically safe arithmetic for the Zamani Quantum Error Correction
//! subsystem.
//!
//! This module provides checked integer arithmetic and validated floating-
//! point operations for QEC algorithms. It is designed for untrusted or
//! externally supplied workloads and therefore:
//!
//! * rejects NaN and infinities where finite values are required;
//! * validates probabilities before logarithmic conversion;
//! * prevents integer overflow/underflow;
//! * prevents invalid logarithmic weights;
//! * provides checked distance and size calculations;
//! * avoids panic-based arithmetic;
//! * provides deterministic floating-point validation helpers;
//! * treats numerical failure as an explicit `Result`;
//! * supports bounded probability and weight calculations.
//!
//! The module intentionally does not silently clamp invalid values. Invalid
//! numerical state must remain observable to callers.
//!
//! Mathematical conventions:
//!
//! ```text
//! probability p ∈ (0, 1]
//!
//! weight(p) = -ln(p)
//!
//! p = 1       => weight = 0
//! 0 < p < 1   => weight > 0
//! p <= 0      => invalid
//! p > 1       => invalid
//! NaN/∞       => invalid
//! ```
//!
//! For probability-zero events, use `zero_probability_weight()` rather than
//! attempting to evaluate `-ln(0)`.

use core::fmt;

/// Result type used by this module.
pub type ArithmeticResult<T> = Result<T, ArithmeticError>;

/// Numerical errors that can occur during QEC arithmetic.
///
/// This error type is intentionally independent from the broader QEC error
/// hierarchy. The QEC root module can later convert it into `QecError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    /// An integer addition overflowed.
    IntegerOverflow,

    /// An integer subtraction overflowed.
    IntegerUnderflow,

    /// An integer multiplication overflowed.
    IntegerMultiplicationOverflow,

    /// An integer division was invalid.
    DivisionByZero,

    /// A signed integer absolute value cannot be represented.
    AbsoluteValueOverflow,

    /// A floating-point value was NaN.
    NaN,

    /// A floating-point value was positive or negative infinity.
    Infinite,

    /// A floating-point value was required to be finite and was not.
    NonFinite,

    /// A probability was outside the permitted range.
    InvalidProbability,

    /// A probability was negative.
    NegativeProbability,

    /// A logarithmic operation received zero.
    LogarithmOfZero,

    /// A logarithmic operation received a negative value.
    LogarithmOfNegative,

    /// A computed value became non-finite.
    NumericalOverflow,

    /// A calculated weight was negative when a non-negative weight was
    /// required.
    NegativeWeight,

    /// A distance or size was invalid.
    InvalidDistance,

    /// A requested quantity exceeded the supplied bound.
    LimitExceeded,

    /// A conversion could not be represented exactly/safely.
    ConversionOverflow,
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::IntegerOverflow => "integer addition overflow",
            Self::IntegerUnderflow => "integer subtraction underflow",
            Self::IntegerMultiplicationOverflow => "integer multiplication overflow",
            Self::DivisionByZero => "division by zero",
            Self::AbsoluteValueOverflow => "integer absolute value overflow",
            Self::NaN => "NaN is not permitted",
            Self::Infinite => "infinite value is not permitted",
            Self::NonFinite => "non-finite value is not permitted",
            Self::InvalidProbability => "invalid probability",
            Self::NegativeProbability => "negative probability",
            Self::LogarithmOfZero => "logarithm of zero is not finite",
            Self::LogarithmOfNegative => "logarithm of a negative value is undefined",
            Self::NumericalOverflow => "floating-point numerical overflow",
            Self::NegativeWeight => "negative weight is not permitted",
            Self::InvalidDistance => "invalid distance",
            Self::LimitExceeded => "arithmetic limit exceeded",
            Self::ConversionOverflow => "numeric conversion overflow",
        };

        f.write_str(message)
    }
}

impl std::error::Error for ArithmeticError {}

/// Validated finite floating-point value.
///
/// This wrapper prevents NaN and infinity from entering numerical code that
/// explicitly requires finite values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite value.
    pub fn new(value: f64) -> ArithmeticResult<Self> {
        if value.is_nan() {
            return Err(ArithmeticError::NaN);
        }

        if value.is_infinite() {
            return Err(ArithmeticError::Infinite);
        }

        Ok(Self(value))
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns true when the value is exactly zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns true when the value is positive.
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0 > 0.0
    }

    /// Returns true when the value is negative.
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0 < 0.0
    }
}

impl TryFrom<f64> for FiniteF64 {
    type Error = ArithmeticError;

    fn try_from(value: f64) -> ArithmeticResult<Self> {
        Self::new(value)
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.0
    }
}

/// A validated probability.
///
/// The representation permits `0.0` because a zero-probability event can be
/// meaningful in a noise model. Callers requiring a strictly positive
/// probability should use [`PositiveProbability`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Creates a probability in `[0, 1]`.
    pub fn new(value: f64) -> ArithmeticResult<Self> {
        validate_finite(value)?;

        if value < 0.0 {
            return Err(ArithmeticError::NegativeProbability);
        }

        if value > 1.0 {
            return Err(ArithmeticError::InvalidProbability);
        }

        Ok(Self(value))
    }

    /// Returns the probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns true when the probability is exactly zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns true when the probability is exactly one.
    #[must_use]
    pub fn is_one(self) -> bool {
        self.0 == 1.0
    }
}

impl TryFrom<f64> for Probability {
    type Error = ArithmeticError;

    fn try_from(value: f64) -> ArithmeticResult<Self> {
        Self::new(value)
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.0
    }
}

/// A strictly positive probability in `(0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PositiveProbability(f64);

impl PositiveProbability {
    /// Creates a strictly positive probability.
    pub fn new(value: f64) -> ArithmeticResult<Self> {
        validate_finite(value)?;

        if value <= 0.0 {
            return Err(ArithmeticError::InvalidProbability);
        }

        if value > 1.0 {
            return Err(ArithmeticError::InvalidProbability);
        }

        Ok(Self(value))
    }

    /// Returns the probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Converts the probability to a decoding weight.
    ///
    /// `weight = -ln(p)`
    pub fn to_weight(self) -> ArithmeticResult<NonNegativeWeight> {
        probability_to_weight(self)
    }
}

impl TryFrom<f64> for PositiveProbability {
    type Error = ArithmeticError;

    fn try_from(value: f64) -> ArithmeticResult<Self> {
        Self::new(value)
    }
}

impl From<PositiveProbability> for f64 {
    fn from(value: PositiveProbability) -> Self {
        value.0
    }
}

/// A validated non-negative finite decoding weight.
///
/// Infinite weights are deliberately not represented by this type. If a
/// caller needs to represent an impossible edge, it should represent that
/// state explicitly in its graph abstraction rather than smuggling infinity
/// into numerical arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NonNegativeWeight(f64);

impl NonNegativeWeight {
    /// Creates a finite non-negative weight.
    pub fn new(value: f64) -> ArithmeticResult<Self> {
        validate_finite(value)?;

        if value < 0.0 {
            return Err(ArithmeticError::NegativeWeight);
        }

        Ok(Self(value))
    }

    /// Returns the underlying weight.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns true when the weight is zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl TryFrom<f64> for NonNegativeWeight {
    type Error = ArithmeticError;

    fn try_from(value: f64) -> ArithmeticResult<Self> {
        Self::new(value)
    }
}

impl From<NonNegativeWeight> for f64 {
    fn from(value: NonNegativeWeight) -> Self {
        value.0
    }
}

/// Validates that a floating-point number is finite.
pub fn validate_finite(value: f64) -> ArithmeticResult<()> {
    if value.is_nan() {
        return Err(ArithmeticError::NaN);
    }

    if value.is_infinite() {
        return Err(ArithmeticError::Infinite);
    }

    Ok(())
}

/// Validates a probability in `[0, 1]`.
pub fn validate_probability(value: f64) -> ArithmeticResult<()> {
    Probability::new(value).map(|_| ())
}

/// Validates a strictly positive probability in `(0, 1]`.
pub fn validate_positive_probability(value: f64) -> ArithmeticResult<()> {
    PositiveProbability::new(value).map(|_| ())
}

/// Converts a strictly positive probability into a non-negative decoding
/// weight.
///
/// The standard mapping is:
///
/// `w = -ln(p)`
///
/// Therefore:
///
/// * `p = 1` produces `0`;
/// * `0 < p < 1` produces a positive finite weight;
/// * `p <= 0` is rejected;
/// * `p > 1` is rejected;
/// * NaN and infinity are rejected.
pub fn probability_to_weight(
    probability: PositiveProbability,
) -> ArithmeticResult<NonNegativeWeight> {
    let p = probability.get();
    let weight = -p.ln();

    validate_finite(weight)?;

    // Guard against unexpected platform/libm behaviour.
    if weight < 0.0 {
        return Err(ArithmeticError::NegativeWeight);
    }

    NonNegativeWeight::new(weight)
}

/// Converts a probability directly into a decoding weight.
pub fn probability_to_weight_f64(probability: f64) -> ArithmeticResult<NonNegativeWeight> {
    probability
        .try_into()
        .and_then(probability_to_weight)
}

/// Returns the mathematically infinite weight associated with a zero
/// probability as an explicit `Option`.
///
/// `None` means the event is impossible and therefore should not normally
/// produce a finite graph edge.
///
/// This avoids representing infinity as a valid decoder weight.
#[must_use]
pub const fn zero_probability_weight() -> Option<NonNegativeWeight> {
    None
}

/// Computes a finite natural logarithm.
///
/// Zero and negative values are rejected rather than returning `-∞` or NaN.
pub fn checked_ln(value: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(value)?;

    if value == 0.0 {
        return Err(ArithmeticError::LogarithmOfZero);
    }

    if value < 0.0 {
        return Err(ArithmeticError::LogarithmOfNegative);
    }

    let result = value.ln();
    FiniteF64::new(result)
}

/// Computes a finite base-2 logarithm.
pub fn checked_log2(value: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(value)?;

    if value == 0.0 {
        return Err(ArithmeticError::LogarithmOfZero);
    }

    if value < 0.0 {
        return Err(ArithmeticError::LogarithmOfNegative);
    }

    FiniteF64::new(value.log2())
}

/// Computes `-ln(value)` as a non-negative weight.
pub fn negative_ln(value: f64) -> ArithmeticResult<NonNegativeWeight> {
    probability_to_weight_f64(value)
}

/// Checked addition for `usize`.
pub fn checked_add_usize(lhs: usize, rhs: usize) -> ArithmeticResult<usize> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked subtraction for `usize`.
pub fn checked_sub_usize(lhs: usize, rhs: usize) -> ArithmeticResult<usize> {
    lhs.checked_sub(rhs)
        .ok_or(ArithmeticError::IntegerUnderflow)
}

/// Checked multiplication for `usize`.
pub fn checked_mul_usize(lhs: usize, rhs: usize) -> ArithmeticResult<usize> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Checked multiplication followed by addition.
///
/// Useful for flattened indexing:
///
/// `index = row * stride + column`
pub fn checked_mul_add_usize(
    lhs: usize,
    multiplier: usize,
    addend: usize,
) -> ArithmeticResult<usize> {
    let product = checked_mul_usize(lhs, multiplier)?;
    checked_add_usize(product, addend)
}

/// Checked division for `usize`.
pub fn checked_div_usize(lhs: usize, rhs: usize) -> ArithmeticResult<usize> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(lhs / rhs)
}

/// Checked addition for `u64`.
pub fn checked_add_u64(lhs: u64, rhs: u64) -> ArithmeticResult<u64> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked multiplication for `u64`.
pub fn checked_mul_u64(lhs: u64, rhs: u64) -> ArithmeticResult<u64> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Checked addition for `u128`.
pub fn checked_add_u128(lhs: u128, rhs: u128) -> ArithmeticResult<u128> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked multiplication for `u128`.
pub fn checked_mul_u128(lhs: u128, rhs: u128) -> ArithmeticResult<u128> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Computes an absolute signed distance safely.
///
/// Unlike `i64::abs()`, this function explicitly handles the minimum
/// representable integer instead of allowing an overflow.
pub fn checked_abs_i64(value: i64) -> ArithmeticResult<u64> {
    if value == i64::MIN {
        return Err(ArithmeticError::AbsoluteValueOverflow);
    }

    Ok(value.unsigned_abs())
}

/// Computes the absolute difference between two unsigned coordinates.
///
/// This is preferable to `a - b` because it cannot underflow.
#[must_use]
pub fn abs_diff_usize(lhs: usize, rhs: usize) -> usize {
    lhs.abs_diff(rhs)
}

/// Computes a Manhattan distance using checked arithmetic.
///
/// `distance = |x1-x2| + |y1-y2|`
pub fn checked_manhattan_distance(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) -> ArithmeticResult<usize> {
    let dx = abs_diff_usize(x1, x2);
    let dy = abs_diff_usize(y1, y2);

    checked_add_usize(dx, dy)
}

/// Computes a three-dimensional Manhattan distance.
///
/// `distance = |x1-x2| + |y1-y2| + |z1-z2|`
pub fn checked_manhattan_distance_3d(
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
) -> ArithmeticResult<usize> {
    let dx = abs_diff_usize(x1, x2);
    let dy = abs_diff_usize(y1, y2);
    let dz = abs_diff_usize(z1, z2);

    let xy = checked_add_usize(dx, dy)?;
    checked_add_usize(xy, dz)
}

/// Validates a code distance.
///
/// Surface-code distance must normally be at least three for a non-trivial
/// error-correcting code. The minimum is kept explicit here so callers do not
/// accidentally construct a meaningless code.
pub fn validate_code_distance(distance: usize) -> ArithmeticResult<()> {
    if distance < 3 {
        return Err(ArithmeticError::InvalidDistance);
    }

    Ok(())
}

/// Validates a code distance against an explicit upper bound.
pub fn validate_code_distance_with_limit(
    distance: usize,
    maximum: usize,
) -> ArithmeticResult<()> {
    validate_code_distance(distance)?;

    if distance > maximum {
        return Err(ArithmeticError::LimitExceeded);
    }

    Ok(())
}

/// Computes `distance²` safely.
pub fn checked_square_usize(value: usize) -> ArithmeticResult<usize> {
    checked_mul_usize(value, value)
}

/// Computes the number of cells in a square lattice safely.
pub fn checked_square_size(side: usize) -> ArithmeticResult<usize> {
    checked_square_usize(side)
}

/// Computes the number of cells in a rectangular lattice safely.
pub fn checked_rectangle_size(width: usize, height: usize) -> ArithmeticResult<usize> {
    checked_mul_usize(width, height)
}

/// Computes a bounded allocation size.
///
/// This should be used before allocating arrays whose length is derived from
/// externally supplied QEC parameters.
pub fn checked_allocation_size(
    element_count: usize,
    element_size: usize,
    maximum_bytes: usize,
) -> ArithmeticResult<usize> {
    let bytes = checked_mul_usize(element_count, element_size)?;

    if bytes > maximum_bytes {
        return Err(ArithmeticError::LimitExceeded);
    }

    Ok(bytes)
}

/// Checked conversion from `u64` to `usize`.
pub fn checked_u64_to_usize(value: u64) -> ArithmeticResult<usize> {
    usize::try_from(value).map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Checked conversion from `u128` to `usize`.
pub fn checked_u128_to_usize(value: u128) -> ArithmeticResult<usize> {
    usize::try_from(value).map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Checked conversion from `usize` to `u64`.
pub fn checked_usize_to_u64(value: usize) -> ArithmeticResult<u64> {
    u64::try_from(value).map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Performs a checked floating-point addition.
///
/// Floating-point addition is considered invalid if the result becomes
/// non-finite.
pub fn checked_add_f64(lhs: f64, rhs: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs + rhs).map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Performs a checked floating-point subtraction.
pub fn checked_sub_f64(lhs: f64, rhs: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs - rhs).map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Performs a checked floating-point multiplication.
pub fn checked_mul_f64(lhs: f64, rhs: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs * rhs).map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Performs a checked floating-point division.
pub fn checked_div_f64(lhs: f64, rhs: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    if rhs == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    FiniteF64::new(lhs / rhs).map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Computes a weighted sum without allowing NaN or infinity to enter the
/// result.
pub fn checked_weight_sum<I>(weights: I) -> ArithmeticResult<NonNegativeWeight>
where
    I: IntoIterator<Item = NonNegativeWeight>,
{
    let mut total = 0.0_f64;

    for weight in weights {
        total += weight.get();

        if !total.is_finite() {
            return Err(ArithmeticError::NumericalOverflow);
        }
    }

    NonNegativeWeight::new(total)
}

/// Computes the numerically stable log-sum-exp operation.
///
/// This is useful when decoding probabilities represented in logarithmic
/// form. The implementation avoids directly exponentiating large positive
/// values.
///
/// For an empty iterator, the operation returns an error rather than silently
/// producing `-∞`.
pub fn checked_log_sum_exp<I>(values: I) -> ArithmeticResult<FiniteF64>
where
    I: IntoIterator<Item = f64>,
{
    let values: Vec<f64> = values
        .into_iter()
        .map(|value| {
            validate_finite(value)?;
            Ok(value)
        })
        .collect::<ArithmeticResult<Vec<_>>>()?;

    let Some(&maximum) = values.iter().max_by(|a, b| a.total_cmp(b)) else {
        return Err(ArithmeticError::InvalidInput);
    };

    let mut sum = 0.0_f64;

    for value in values {
        sum += (value - maximum).exp();

        if !sum.is_finite() {
            return Err(ArithmeticError::NumericalOverflow);
        }
    }

    let result = maximum + sum.ln();

    FiniteF64::new(result).map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Returns whether a floating-point value is safely usable as a finite
/// numerical quantity.
#[must_use]
pub fn is_valid_finite(value: f64) -> bool {
    value.is_finite()
}

/// Returns whether a value is a valid probability in `[0, 1]`.
#[must_use]
pub fn is_valid_probability(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

/// Returns whether a value is a valid strictly positive probability.
#[must_use]
pub fn is_valid_positive_probability(value: f64) -> bool {
    value.is_finite() && value > 0.0 && value <= 1.0
}

/// Returns whether a value is a valid non-negative finite decoding weight.
#[must_use]
pub fn is_valid_weight(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_values_are_accepted() {
        assert!(FiniteF64::new(0.0).is_ok());
        assert!(FiniteF64::new(-1.5).is_ok());
        assert!(FiniteF64::new(f64::MAX).is_ok());
    }

    #[test]
    fn nan_is_rejected() {
        assert_eq!(
            FiniteF64::new(f64::NAN),
            Err(ArithmeticError::NaN)
        );
    }

    #[test]
    fn infinity_is_rejected() {
        assert_eq!(
            FiniteF64::new(f64::INFINITY),
            Err(ArithmeticError::Infinite)
        );
    }

    #[test]
    fn probability_range_is_enforced() {
        assert!(Probability::new(0.0).is_ok());
        assert!(Probability::new(0.5).is_ok());
        assert!(Probability::new(1.0).is_ok());

        assert!(Probability::new(-0.1).is_err());
        assert!(Probability::new(1.1).is_err());
        assert!(Probability::new(f64::NAN).is_err());
        assert!(Probability::new(f64::INFINITY).is_err());
    }

    #[test]
    fn positive_probability_rejects_zero() {
        assert!(PositiveProbability::new(0.0).is_err());
        assert!(PositiveProbability::new(-0.1).is_err());
        assert!(PositiveProbability::new(0.5).is_ok());
        assert!(PositiveProbability::new(1.0).is_ok());
    }

    #[test]
    fn probability_to_weight_is_correct() {
        let weight = probability_to_weight_f64(1.0)
            .expect("p=1 should produce zero weight");

        assert_eq!(weight.get(), 0.0);

        let weight = probability_to_weight_f64(0.5)
            .expect("p=0.5 should produce a finite weight");

        assert!(weight.get() > 0.0);
        assert!(weight.get().is_finite());
    }

    #[test]
    fn zero_probability_is_not_encoded_as_infinity() {
        assert!(probability_to_weight_f64(0.0).is_err());
        assert!(zero_probability_weight().is_none());
    }

    #[test]
    fn invalid_probability_cannot_generate_weight() {
        assert!(probability_to_weight_f64(-1.0).is_err());
        assert!(probability_to_weight_f64(1.1).is_err());
        assert!(probability_to_weight_f64(f64::NAN).is_err());
        assert!(probability_to_weight_f64(f64::INFINITY).is_err());
    }

    #[test]
    fn checked_usize_arithmetic_detects_overflow() {
        assert_eq!(
            checked_add_usize(usize::MAX, 1),
            Err(ArithmeticError::IntegerOverflow)
        );

        assert_eq!(
            checked_mul_usize(usize::MAX, 2),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );
    }

    #[test]
    fn checked_usize_arithmetic_detects_underflow() {
        assert_eq!(
            checked_sub_usize(0, 1),
            Err(ArithmeticError::IntegerUnderflow)
        );
    }

    #[test]
    fn checked_division_rejects_zero() {
        assert_eq!(
            checked_div_usize(10, 0),
            Err(ArithmeticError::DivisionByZero)
        );

        assert_eq!(
            checked_div_f64(10.0, 0.0),
            Err(ArithmeticError::DivisionByZero)
        );
    }

    #[test]
    fn checked_manhattan_distance_is_safe() {
        assert_eq!(
            checked_manhattan_distance(0, 0, 3, 4)
                .expect("distance should be valid"),
            7
        );
    }

    #[test]
    fn checked_manhattan_distance_detects_overflow() {
        let result = checked_manhattan_distance(
            0,
            0,
            usize::MAX,
            usize::MAX,
        );

        assert_eq!(
            result,
            Err(ArithmeticError::IntegerOverflow)
        );
    }

    #[test]
    fn code_distance_is_validated() {
        assert!(validate_code_distance(3).is_ok());
        assert!(validate_code_distance(5).is_ok());
        assert!(validate_code_distance(2).is_err());
        assert!(validate_code_distance(0).is_err());
    }

    #[test]
    fn code_distance_limit_is_enforced() {
        assert!(validate_code_distance_with_limit(5, 5).is_ok());
        assert!(validate_code_distance_with_limit(7, 5).is_err());
    }

    #[test]
    fn allocation_size_is_bounded() {
        assert_eq!(
            checked_allocation_size(10, 8, 80)
                .expect("allocation should fit"),
            80
        );

        assert_eq!(
            checked_allocation_size(11, 8, 80),
            Err(ArithmeticError::LimitExceeded)
        );
    }

    #[test]
    fn allocation_multiplication_overflow_is_detected() {
        assert_eq!(
            checked_allocation_size(usize::MAX, 2, usize::MAX),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );
    }

    #[test]
    fn checked_floating_point_operations_reject_non_finite_inputs() {
        assert!(checked_add_f64(f64::NAN, 1.0).is_err());
        assert!(checked_add_f64(f64::INFINITY, 1.0).is_err());
        assert!(checked_mul_f64(f64::NAN, 1.0).is_err());
        assert!(checked_div_f64(1.0, f64::NAN).is_err());
    }

    #[test]
    fn checked_logarithms_reject_invalid_inputs() {
        assert!(checked_ln(1.0).is_ok());
        assert!(checked_ln(0.0).is_err());
        assert!(checked_ln(-1.0).is_err());
        assert!(checked_ln(f64::NAN).is_err());
        assert!(checked_ln(f64::INFINITY).is_err());

        assert!(checked_log2(1.0).is_ok());
        assert!(checked_log2(0.0).is_err());
        assert!(checked_log2(-1.0).is_err());
    }

    #[test]
    fn weight_sum_is_checked() {
        let weights = [
            NonNegativeWeight::new(1.0).expect("valid"),
            NonNegativeWeight::new(2.0).expect("valid"),
            NonNegativeWeight::new(3.0).expect("valid"),
        ];

        assert_eq!(
            checked_weight_sum(weights)
                .expect("sum should be valid")
                .get(),
            6.0
        );
    }

    #[test]
    fn negative_weights_are_rejected() {
        assert_eq!(
            NonNegativeWeight::new(-1.0),
            Err(ArithmeticError::NegativeWeight)
        );
    }

    #[test]
    fn log_sum_exp_is_stable() {
        let result = checked_log_sum_exp([
            -1000.0,
            -1001.0,
            -1002.0,
        ])
        .expect("log-sum-exp should be finite");

        assert!(result.get().is_finite());
    }

    #[test]
    fn log_sum_exp_rejects_empty_input() {
        assert!(checked_log_sum_exp([]).is_err());
    }

    #[test]
    fn numeric_predicates_are_correct() {
        assert!(is_valid_finite(1.0));
        assert!(!is_valid_finite(f64::NAN));
        assert!(!is_valid_finite(f64::INFINITY));

        assert!(is_valid_probability(0.0));
        assert!(is_valid_probability(1.0));
        assert!(!is_valid_probability(-1.0));
        assert!(!is_valid_probability(2.0));

        assert!(is_valid_positive_probability(0.1));
        assert!(!is_valid_positive_probability(0.0));

        assert!(is_valid_weight(0.0));
        assert!(is_valid_weight(10.0));
        assert!(!is_valid_weight(-1.0));
        assert!(!is_valid_weight(f64::INFINITY));
    }
}