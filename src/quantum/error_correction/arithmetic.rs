//! Numerically safe arithmetic for Zamani Quantum Error Correction.
//!
//! # Rust compatibility
//!
//! This module is intentionally compatible with the repository's pinned
//! Rust 1.97.1 toolchain.
//!
//! # Architectural role
//!
//! `arithmetic.rs` is a foundational, dependency-light numerical layer.
//!
//! ```text
//!                 arithmetic.rs
//!                      │
//!        ┌─────────────┼─────────────┐
//!        ▼             ▼             ▼
//!     limits        memory        algorithms
//!        │             │             │
//!        └─────────────┼─────────────┘
//!                      ▼
//!                 QEC execution
//! ```
//!
//! This module owns numerical safety. It does not own QEC policy.
//!
//! ## This module owns
//!
//! - checked integer arithmetic;
//! - checked integer powers;
//! - checked combinations;
//! - checked memory-size calculations;
//! - checked index calculations;
//! - finite floating-point validation;
//! - probability validation;
//! - probability-to-weight conversion;
//! - non-negative finite decoder weights;
//! - safe coordinate-distance calculations;
//! - numerical bounds;
//! - conversion helpers.
//!
//! ## This module does not own
//!
//! - resource policy;
//! - allocation;
//! - cancellation;
//! - decoder policy;
//! - QPU execution;
//! - telemetry;
//! - configuration;
//! - capability authorization.
//!
//! Higher-level modules are responsible for applying those policies.
//!
//! # Numerical safety rules
//!
//! Arithmetic must never silently:
//!
//! - wrap;
//! - saturate;
//! - truncate an unsafe conversion;
//! - turn invalid floating-point state into valid state;
//! - convert overflow into a plausible QEC result.
//!
//! All potentially unsafe arithmetic returns `Result`.
//!
//! # Probability convention
//!
//! ```text
//! p ∈ [0, 1]
//!
//! positive probability:
//! p ∈ (0, 1]
//!
//! decoder weight:
//! w = -ln(p)
//!
//! p = 1       => w = 0
//! 0 < p < 1   => w > 0
//! p = 0       => no finite weight; caller must model an impossible edge
//! p < 0       => invalid
//! p > 1       => invalid
//! NaN/∞       => invalid
//! ```
//!
//! Infinite decoder weights are deliberately not represented by
//! `NonNegativeWeight`. Impossible edges should be represented explicitly by
//! the graph layer.
//!
//! # Integration contract
//!
//! `arithmetic.rs` is intentionally independent of:
//!
//! - `limits.rs`;
//! - `memory.rs`;
//! - `resources.rs`;
//! - `errors.rs`.
//!
//! Those modules may convert `ArithmeticError` into their own canonical
//! errors at their public boundaries.
//!
//! This prevents a dependency cycle in the foundation layer.
//!
//! Every higher-level QEC module performing dangerous arithmetic should use
//! this module or equivalent checked operations. The module itself never
//! enforces workload policy; that remains the responsibility of `QecLimits`.
//!
//! # Determinism
//!
//! Integer operations are deterministic. Floating-point helpers validate
//! inputs and outputs but do not promise bit-for-bit equality across
//! different CPU/libm implementations. Deterministic execution requiring
//! cross-platform bit identity must establish an explicit numerical backend
//! policy above this layer.

use core::fmt;

/// Result type for numerically safe arithmetic.
pub type ArithmeticResult<T> = Result<T, ArithmeticError>;

/// Numerical failures produced by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticError {
    /// Addition overflowed.
    IntegerOverflow,

    /// Subtraction underflowed.
    IntegerUnderflow,

    /// Multiplication overflowed.
    IntegerMultiplicationOverflow,

    /// Division by zero was requested.
    DivisionByZero,

    /// A signed absolute value cannot be represented.
    AbsoluteValueOverflow,

    /// A floating-point value is NaN.
    NaN,

    /// A floating-point value is infinite.
    Infinite,

    /// A floating-point value is not finite.
    NonFinite,

    /// A probability is invalid.
    InvalidProbability,

    /// A probability is negative.
    NegativeProbability,

    /// A logarithm received zero.
    LogarithmOfZero,

    /// A logarithm received a negative value.
    LogarithmOfNegative,

    /// A calculated floating-point value became non-finite.
    NumericalOverflow,

    /// A decoder weight is negative.
    NegativeWeight,

    /// A distance is invalid.
    InvalidDistance,

    /// A calculated quantity exceeded an explicit arithmetic bound.
    LimitExceeded,

    /// A numeric conversion cannot be represented safely.
    ConversionOverflow,

    /// A non-zero mathematical denominator was required.
    InvalidDenominator,

    /// A requested exponentiation cannot be represented.
    ExponentiationOverflow,

    /// A combination calculation cannot be represented.
    CombinationOverflow,

    /// A requested operation is mathematically invalid.
    InvalidOperation,
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::IntegerOverflow => "integer addition overflow",
            Self::IntegerUnderflow => "integer subtraction underflow",
            Self::IntegerMultiplicationOverflow => {
                "integer multiplication overflow"
            }
            Self::DivisionByZero => "division by zero",
            Self::AbsoluteValueOverflow => "integer absolute value overflow",
            Self::NaN => "NaN is not permitted",
            Self::Infinite => "infinite value is not permitted",
            Self::NonFinite => "non-finite value is not permitted",
            Self::InvalidProbability => "invalid probability",
            Self::NegativeProbability => "negative probability",
            Self::LogarithmOfZero => "logarithm of zero is undefined",
            Self::LogarithmOfNegative => {
                "logarithm of a negative value is undefined"
            }
            Self::NumericalOverflow => "floating-point numerical overflow",
            Self::NegativeWeight => "negative weight is not permitted",
            Self::InvalidDistance => "invalid distance",
            Self::LimitExceeded => "arithmetic limit exceeded",
            Self::ConversionOverflow => "numeric conversion overflow",
            Self::InvalidDenominator => "invalid denominator",
            Self::ExponentiationOverflow => "integer exponentiation overflow",
            Self::CombinationOverflow => "combination calculation overflow",
            Self::InvalidOperation => "invalid numerical operation",
        };

        f.write_str(message)
    }
}

impl std::error::Error for ArithmeticError {}

/// A validated finite `f64`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite floating-point value.
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

    /// Returns whether the value is zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether the value is positive.
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0 > 0.0
    }

    /// Returns whether the value is negative.
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

/// A probability in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Creates a validated probability.
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

    /// Returns the underlying probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns whether the probability is zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether the probability is one.
    #[must_use]
    pub fn is_one(self) -> bool {
        self.0 == 1.0
    }

    /// Converts this probability into a positive probability when possible.
    pub fn positive(self) -> ArithmeticResult<PositiveProbability> {
        PositiveProbability::new(self.0)
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

        if value <= 0.0 || value > 1.0 {
            return Err(ArithmeticError::InvalidProbability);
        }

        Ok(Self(value))
    }

    /// Returns the underlying probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Converts the probability into a decoder weight.
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

/// A finite, non-negative decoder weight.
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

    /// Returns whether the weight is zero.
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

/// Validates that a floating-point value is finite.
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

/// Converts a strictly positive probability into a finite decoder weight.
///
/// ```text
/// w = -ln(p)
/// ```
pub fn probability_to_weight(
    probability: PositiveProbability,
) -> ArithmeticResult<NonNegativeWeight> {
    let value = probability.get();
    let weight = -value.ln();

    validate_finite(weight)?;

    if weight < 0.0 {
        return Err(ArithmeticError::NegativeWeight);
    }

    NonNegativeWeight::new(weight)
}

/// Converts an `f64` probability directly into a decoder weight.
pub fn probability_to_weight_f64(
    probability: f64,
) -> ArithmeticResult<NonNegativeWeight> {
    PositiveProbability::new(probability)
        .and_then(probability_to_weight)
}

/// Converts a probability into a decoder weight.
///
/// Zero probability is deliberately rejected because no finite decoder
/// weight can represent `-ln(0)`.
pub fn negative_ln(value: f64) -> ArithmeticResult<NonNegativeWeight> {
    probability_to_weight_f64(value)
}

/// Represents an impossible probability event.
///
/// `None` is deliberate: an impossible edge must not be represented as a
/// finite numerical weight.
#[must_use]
pub const fn zero_probability_weight() -> Option<NonNegativeWeight> {
    None
}

/// Computes a checked natural logarithm.
pub fn checked_ln(value: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(value)?;

    if value == 0.0 {
        return Err(ArithmeticError::LogarithmOfZero);
    }

    if value < 0.0 {
        return Err(ArithmeticError::LogarithmOfNegative);
    }

    FiniteF64::new(value.ln())
}

/// Computes a checked base-2 logarithm.
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

/// Checked addition for `usize`.
pub fn checked_add_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked subtraction for `usize`.
pub fn checked_sub_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    lhs.checked_sub(rhs)
        .ok_or(ArithmeticError::IntegerUnderflow)
}

/// Checked multiplication for `usize`.
pub fn checked_mul_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Checked multiplication followed by addition.
///
/// Useful for flattened indexing:
///
/// `index = base * stride + offset`
pub fn checked_mul_add_usize(
    base: usize,
    stride: usize,
    offset: usize,
) -> ArithmeticResult<usize> {
    let product = checked_mul_usize(base, stride)?;
    checked_add_usize(product, offset)
}

/// Checked division for `usize`.
pub fn checked_div_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(lhs / rhs)
}

/// Checked remainder for `usize`.
pub fn checked_rem_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(lhs % rhs)
}

/// Checked addition for `u64`.
pub fn checked_add_u64(
    lhs: u64,
    rhs: u64,
) -> ArithmeticResult<u64> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked subtraction for `u64`.
pub fn checked_sub_u64(
    lhs: u64,
    rhs: u64,
) -> ArithmeticResult<u64> {
    lhs.checked_sub(rhs)
        .ok_or(ArithmeticError::IntegerUnderflow)
}

/// Checked multiplication for `u64`.
pub fn checked_mul_u64(
    lhs: u64,
    rhs: u64,
) -> ArithmeticResult<u64> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Checked division for `u64`.
pub fn checked_div_u64(
    lhs: u64,
    rhs: u64,
) -> ArithmeticResult<u64> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(lhs / rhs)
}

/// Checked addition for `u128`.
pub fn checked_add_u128(
    lhs: u128,
    rhs: u128,
) -> ArithmeticResult<u128> {
    lhs.checked_add(rhs)
        .ok_or(ArithmeticError::IntegerOverflow)
}

/// Checked subtraction for `u128`.
pub fn checked_sub_u128(
    lhs: u128,
    rhs: u128,
) -> ArithmeticResult<u128> {
    lhs.checked_sub(rhs)
        .ok_or(ArithmeticError::IntegerUnderflow)
}

/// Checked multiplication for `u128`.
pub fn checked_mul_u128(
    lhs: u128,
    rhs: u128,
) -> ArithmeticResult<u128> {
    lhs.checked_mul(rhs)
        .ok_or(ArithmeticError::IntegerMultiplicationOverflow)
}

/// Checked division for `u128`.
pub fn checked_div_u128(
    lhs: u128,
    rhs: u128,
) -> ArithmeticResult<u128> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(lhs / rhs)
}

/// Safely computes the absolute value of an `i64`.
pub fn checked_abs_i64(value: i64) -> ArithmeticResult<u64> {
    if value == i64::MIN {
        return Err(ArithmeticError::AbsoluteValueOverflow);
    }

    Ok(value.unsigned_abs())
}

/// Safely computes the absolute difference of two `usize` coordinates.
#[must_use]
pub fn abs_diff_usize(lhs: usize, rhs: usize) -> usize {
    lhs.abs_diff(rhs)
}

/// Safely computes the absolute difference of two `u64` coordinates.
#[must_use]
pub fn abs_diff_u64(lhs: u64, rhs: u64) -> u64 {
    lhs.abs_diff(rhs)
}

/// Safely computes the Manhattan distance of two 2-D `usize` coordinates.
pub fn manhattan_distance_usize(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) -> ArithmeticResult<usize> {
    let dx = abs_diff_usize(x1, x2);
    let dy = abs_diff_usize(y1, y2);

    checked_add_usize(dx, dy)
}

/// Safely computes the Manhattan distance of two 2-D `u64` coordinates.
pub fn manhattan_distance_u64(
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
) -> ArithmeticResult<u64> {
    let dx = abs_diff_u64(x1, x2);
    let dy = abs_diff_u64(y1, y2);

    checked_add_u64(dx, dy)
}

/// Safely squares a `usize`.
pub fn checked_square_usize(value: usize) -> ArithmeticResult<usize> {
    checked_mul_usize(value, value)
}

/// Safely cubes a `usize`.
pub fn checked_cube_usize(value: usize) -> ArithmeticResult<usize> {
    let square = checked_square_usize(value)?;
    checked_mul_usize(square, value)
}

/// Computes `base^exponent` using checked integer arithmetic.
///
/// This uses exponentiation by squaring, so it does not perform `exponent`
/// multiplications unnecessarily.
pub fn checked_pow_usize(
    mut base: usize,
    mut exponent: usize,
) -> ArithmeticResult<usize> {
    let mut result = 1usize;

    while exponent != 0 {
        if exponent & 1 == 1 {
            result = checked_mul_usize(result, base)?;
        }

        exponent >>= 1;

        if exponent != 0 {
            base = checked_mul_usize(base, base)?;
        }
    }

    Ok(result)
}

/// Computes `base^exponent` for `u64`.
pub fn checked_pow_u64(
    mut base: u64,
    mut exponent: u32,
) -> ArithmeticResult<u64> {
    let mut result = 1u64;

    while exponent != 0 {
        if exponent & 1 == 1 {
            result = checked_mul_u64(result, base)?;
        }

        exponent >>= 1;

        if exponent != 0 {
            base = checked_mul_u64(base, base)?;
        }
    }

    Ok(result)
}

/// Computes `3^weight` safely.
///
/// This operation is important for exact Pauli-search distance algorithms.
pub fn checked_three_to_weight(
    weight: usize,
) -> ArithmeticResult<usize> {
    checked_pow_usize(3, weight)
}

/// Computes `2^bits` safely.
pub fn checked_two_to_bits(bits: usize) -> ArithmeticResult<usize> {
    checked_pow_usize(2, bits)
}

/// Computes a checked binomial coefficient.
///
/// Uses multiplicative reduction rather than constructing factorials, which
/// avoids unnecessary intermediate growth.
///
/// Returns an error if the exact result cannot fit in `usize`.
pub fn checked_binomial(
    n: usize,
    k: usize,
) -> ArithmeticResult<usize> {
    if k > n {
        return Ok(0);
    }

    if k == 0 || k == n {
        return Ok(1);
    }

    let reduced_k = core::cmp::min(k, n - k);

    let mut result = 1usize;

    for i in 1..=reduced_k {
        let numerator = n
            .checked_sub(reduced_k)
            .and_then(|v| v.checked_add(i))
            .ok_or(ArithmeticError::CombinationOverflow)?;

        /*
         * For exact binomial arithmetic, numerator * result is divisible by
         * i. Performing the multiplication before division can overflow even
         * when the final result fits. We therefore reduce numerator/result
         * using gcd first.
         */
        let gcd_a = gcd_usize(numerator, i);
        let numerator_reduced = numerator / gcd_a;
        let denominator_reduced = i / gcd_a;

        let gcd_b = gcd_usize(result, denominator_reduced);
        let result_reduced = result / gcd_b;
        let denominator_remaining = denominator_reduced / gcd_b;

        if denominator_remaining != 1 {
            /*
             * The exact binomial recurrence guarantees divisibility of the
             * complete product. If reduction above leaves a denominator,
             * perform the multiplication only after checking it.
             */
            let product = checked_mul_usize(
                result_reduced,
                numerator_reduced,
            )?;

            if product % denominator_remaining != 0 {
                return Err(ArithmeticError::CombinationOverflow);
            }

            result = product / denominator_remaining;
        } else {
            result = checked_mul_usize(
                result_reduced,
                numerator_reduced,
            )?;
        }
    }

    Ok(result)
}

/// Computes the number of Pauli assignments for a support of `weight`.
///
/// For each selected qubit there are three non-identity Paulis:
///
/// `X`, `Y`, `Z`
///
/// Therefore the count is `3^weight`.
pub fn checked_pauli_assignments(
    weight: usize,
) -> ArithmeticResult<usize> {
    checked_three_to_weight(weight)
}

/// Computes the number of weight-`k` non-identity Pauli operators over `n`
/// qubits.
///
/// The result is:
///
/// `C(n, k) * 3^k`
pub fn checked_weight_k_paulis(
    n: usize,
    k: usize,
) -> ArithmeticResult<usize> {
    let combinations = checked_binomial(n, k)?;
    let assignments = checked_three_to_weight(k)?;

    checked_mul_usize(combinations, assignments)
}

/// Computes the number of non-identity Pauli operators through a maximum
/// weight.
///
/// The result is:
///
/// `sum(C(n, k) * 3^k)` for `k = 1..=max_weight`.
pub fn checked_pauli_search_space(
    n: usize,
    max_weight: usize,
) -> ArithmeticResult<usize> {
    if max_weight > n {
        return Err(ArithmeticError::InvalidDistance);
    }

    let mut total = 0usize;

    for weight in 1..=max_weight {
        let count = checked_weight_k_paulis(n, weight)?;
        total = checked_add_usize(total, count)?;
    }

    Ok(total)
}

/// Computes `d^2` safely.
///
/// Surface-code construction should use this before allocation.
pub fn checked_distance_squared(
    distance: usize,
) -> ArithmeticResult<usize> {
    if distance == 0 {
        return Err(ArithmeticError::InvalidDistance);
    }

    checked_square_usize(distance)
}

/// Computes the number of cells in a rectangular grid safely.
pub fn checked_grid_size(
    width: usize,
    height: usize,
) -> ArithmeticResult<usize> {
    if width == 0 || height == 0 {
        return Err(ArithmeticError::InvalidDistance);
    }

    checked_mul_usize(width, height)
}

/// Computes a checked byte size from an element count and element size.
///
/// This is intended for memory preflight. It performs no allocation.
pub fn checked_memory_size(
    element_count: usize,
    element_size: usize,
) -> ArithmeticResult<u64> {
    let bytes = checked_mul_usize(element_count, element_size)?;

    u64::try_from(bytes)
        .map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Computes a checked byte size using `u64` operands.
pub fn checked_memory_size_u64(
    element_count: u64,
    element_size: u64,
) -> ArithmeticResult<u64> {
    checked_mul_u64(element_count, element_size)
}

/// Adds a byte count safely.
pub fn checked_add_bytes(
    current: u64,
    additional: u64,
) -> ArithmeticResult<u64> {
    checked_add_u64(current, additional)
}

/// Converts `usize` to `u64` safely.
pub fn checked_usize_to_u64(
    value: usize,
) -> ArithmeticResult<u64> {
    u64::try_from(value)
        .map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Converts `u64` to `usize` safely.
pub fn checked_u64_to_usize(
    value: u64,
) -> ArithmeticResult<usize> {
    usize::try_from(value)
        .map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Converts `u128` to `usize` safely.
pub fn checked_u128_to_usize(
    value: u128,
) -> ArithmeticResult<usize> {
    usize::try_from(value)
        .map_err(|_| ArithmeticError::ConversionOverflow)
}

/// Converts `usize` to `u128`.
///
/// This conversion is infallible and therefore does not return `Result`.
#[must_use]
pub const fn usize_to_u128(value: usize) -> u128 {
    value as u128
}

/// Computes a checked inclusive range length.
///
/// Returns zero when `start > end`.
pub fn checked_inclusive_range_len(
    start: usize,
    end: usize,
) -> ArithmeticResult<usize> {
    if start > end {
        return Ok(0);
    }

    let distance = checked_sub_usize(end, start)?;
    checked_add_usize(distance, 1)
}

/// Computes a checked exclusive range length.
///
/// Returns zero when `start >= end`.
pub fn checked_exclusive_range_len(
    start: usize,
    end: usize,
) -> ArithmeticResult<usize> {
    if start >= end {
        return Ok(0);
    }

    checked_sub_usize(end, start)
}

/// Checks that a value does not exceed an explicit arithmetic bound.
///
/// This is intentionally not connected to `QecLimits`; it is a local
/// arithmetic guard that can be used by higher-level preflight code.
pub fn ensure_within(
    value: usize,
    maximum: usize,
) -> ArithmeticResult<usize> {
    if value > maximum {
        return Err(ArithmeticError::LimitExceeded);
    }

    Ok(value)
}

/// Checks a `u64` value against an explicit bound.
pub fn ensure_u64_within(
    value: u64,
    maximum: u64,
) -> ArithmeticResult<u64> {
    if value > maximum {
        return Err(ArithmeticError::LimitExceeded);
    }

    Ok(value)
}

/// Checks a finite floating-point value against an inclusive range.
pub fn ensure_f64_in_range(
    value: f64,
    minimum: f64,
    maximum: f64,
) -> ArithmeticResult<f64> {
    validate_finite(value)?;
    validate_finite(minimum)?;
    validate_finite(maximum)?;

    if minimum > maximum {
        return Err(ArithmeticError::InvalidOperation);
    }

    if value < minimum || value > maximum {
        return Err(ArithmeticError::LimitExceeded);
    }

    Ok(value)
}

/// Adds two finite floating-point values and rejects a non-finite result.
pub fn checked_add_f64(
    lhs: f64,
    rhs: f64,
) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs + rhs)
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Subtracts two finite floating-point values and rejects a non-finite
/// result.
pub fn checked_sub_f64(
    lhs: f64,
    rhs: f64,
) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs - rhs)
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Multiplies two finite floating-point values and rejects a non-finite
/// result.
pub fn checked_mul_f64(
    lhs: f64,
    rhs: f64,
) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    FiniteF64::new(lhs * rhs)
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Divides two finite floating-point values and rejects zero denominators and
/// non-finite results.
pub fn checked_div_f64(
    lhs: f64,
    rhs: f64,
) -> ArithmeticResult<FiniteF64> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    if rhs == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    FiniteF64::new(lhs / rhs)
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Computes a finite square root.
///
/// Negative values are rejected rather than producing NaN.
pub fn checked_sqrt(value: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(value)?;

    if value < 0.0 {
        return Err(ArithmeticError::InvalidOperation);
    }

    FiniteF64::new(value.sqrt())
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Computes a finite exponential.
pub fn checked_exp(value: f64) -> ArithmeticResult<FiniteF64> {
    validate_finite(value)?;

    FiniteF64::new(value.exp())
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Computes `base.powi(exponent)` while validating the result.
pub fn checked_powi(
    base: f64,
    exponent: i32,
) -> ArithmeticResult<FiniteF64> {
    validate_finite(base)?;

    FiniteF64::new(base.powi(exponent))
        .map_err(|_| ArithmeticError::NumericalOverflow)
}

/// Computes a finite weighted sum.
///
/// This performs checked floating-point accumulation and rejects a
/// non-finite intermediate or final result.
pub fn checked_sum_f64(
    values: &[f64],
) -> ArithmeticResult<FiniteF64> {
    let mut total = 0.0;

    for &value in values {
        total = checked_add_f64(total, value)?.get();
    }

    FiniteF64::new(total)
}

/// Computes a finite weighted average.
///
/// Empty input is rejected because there is no mathematically defined mean.
pub fn checked_mean_f64(
    values: &[f64],
) -> ArithmeticResult<FiniteF64> {
    if values.is_empty() {
        return Err(ArithmeticError::InvalidOperation);
    }

    let sum = checked_sum_f64(values)?.get();

    checked_div_f64(sum, values.len() as f64)
}

/// Greatest common divisor for `usize`.
///
/// This function is total and deterministic.
#[must_use]
pub fn gcd_usize(mut lhs: usize, mut rhs: usize) -> usize {
    while rhs != 0 {
        let remainder = lhs % rhs;
        lhs = rhs;
        rhs = remainder;
    }

    lhs
}

/// Least common multiple for `usize`.
///
/// Returns an error when the result cannot be represented.
pub fn checked_lcm_usize(
    lhs: usize,
    rhs: usize,
) -> ArithmeticResult<usize> {
    if lhs == 0 || rhs == 0 {
        return Ok(0);
    }

    let divisor = gcd_usize(lhs, rhs);
    let reduced = lhs / divisor;

    checked_mul_usize(reduced, rhs)
}

/// Checked triangular number.
///
/// `n * (n + 1) / 2`
pub fn checked_triangular_number(
    n: usize,
) -> ArithmeticResult<usize> {
    let a;
    let b;

    if n % 2 == 0 {
        a = n / 2;
        b = checked_add_usize(n, 1)?;
    } else {
        a = n;
        b = checked_div_usize(
            checked_add_usize(n, 1)?,
            2,
        )?;
    }

    checked_mul_usize(a, b)
}

/// Checks that a code distance is a valid positive integer.
///
/// Odd-distance requirements are deliberately not imposed here because
/// different QEC codes have different valid distance semantics. Topology
/// validation belongs to `surface_code.rs`.
pub fn validate_distance(
    distance: usize,
) -> ArithmeticResult<usize> {
    if distance == 0 {
        return Err(ArithmeticError::InvalidDistance);
    }

    Ok(distance)
}

/// Computes the number of unordered pairs among `n` elements.
///
/// `C(n, 2)`.
pub fn checked_pair_count(
    n: usize,
) -> ArithmeticResult<usize> {
    checked_binomial(n, 2)
}

/// Computes the number of unordered triples among `n` elements.
///
/// `C(n, 3)`.
pub fn checked_triple_count(
    n: usize,
) -> ArithmeticResult<usize> {
    checked_binomial(n, 3)
}

/// Converts a non-negative `usize` into `u128`.
#[must_use]
pub const fn checked_usize_as_u128(value: usize) -> u128 {
    value as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_values_are_accepted() {
        let value = FiniteF64::new(1.25).expect("finite value");

        assert_eq!(value.get(), 1.25);
        assert!(value.is_positive());
        assert!(!value.is_zero());
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
    fn probabilities_are_bounded() {
        assert!(Probability::new(0.0).is_ok());
        assert!(Probability::new(1.0).is_ok());
        assert!(Probability::new(0.5).is_ok());

        assert_eq!(
            Probability::new(-0.1),
            Err(ArithmeticError::NegativeProbability)
        );

        assert_eq!(
            Probability::new(1.1),
            Err(ArithmeticError::InvalidProbability)
        );
    }

    #[test]
    fn positive_probability_rejects_zero() {
        assert_eq!(
            PositiveProbability::new(0.0),
            Err(ArithmeticError::InvalidProbability)
        );
    }

    #[test]
    fn probability_one_has_zero_weight() {
        let probability =
            PositiveProbability::new(1.0).expect("valid probability");

        let weight = probability
            .to_weight()
            .expect("finite zero weight");

        assert_eq!(weight.get(), 0.0);
    }

    #[test]
    fn probability_half_has_positive_weight() {
        let weight =
            probability_to_weight_f64(0.5).expect("valid probability");

        assert!(weight.get() > 0.0);
    }

    #[test]
    fn zero_probability_has_no_finite_weight() {
        assert!(zero_probability_weight().is_none());
        assert_eq!(
            probability_to_weight_f64(0.0),
            Err(ArithmeticError::InvalidProbability)
        );
    }

    #[test]
    fn checked_usize_operations_reject_overflow() {
        assert_eq!(
            checked_add_usize(usize::MAX, 1),
            Err(ArithmeticError::IntegerOverflow)
        );

        assert_eq!(
            checked_mul_usize(usize::MAX, 2),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );

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
    }

    #[test]
    fn checked_power_is_safe() {
        assert_eq!(
            checked_pow_usize(2, 10).expect("power"),
            1024
        );

        assert_eq!(
            checked_three_to_weight(4).expect("power"),
            81
        );

        assert_eq!(
            checked_pow_usize(usize::MAX, 2),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );
    }

    #[test]
    fn checked_square_is_safe() {
        assert_eq!(
            checked_square_usize(10).expect("square"),
            100
        );

        assert_eq!(
            checked_square_usize(usize::MAX),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );
    }

    #[test]
    fn coordinate_distance_is_safe() {
        assert_eq!(abs_diff_usize(10, 4), 6);

        assert_eq!(
            manhattan_distance_usize(0, 0, 3, 4)
                .expect("distance"),
            7
        );
    }

    #[test]
    fn checked_memory_size_is_safe() {
        assert_eq!(
            checked_memory_size(1024, 8).expect("memory"),
            8192
        );

        assert_eq!(
            checked_memory_size(usize::MAX, usize::MAX),
            Err(ArithmeticError::IntegerMultiplicationOverflow)
        );
    }

    #[test]
    fn binomial_values_are_correct() {
        assert_eq!(
            checked_binomial(5, 0).expect("binomial"),
            1
        );

        assert_eq!(
            checked_binomial(5, 2).expect("binomial"),
            10
        );

        assert_eq!(
            checked_binomial(10, 5).expect("binomial"),
            252
        );

        assert_eq!(
            checked_binomial(5, 8).expect("binomial"),
            0
        );
    }

    #[test]
    fn pauli_search_space_is_correct() {
        assert_eq!(
            checked_pauli_assignments(0).expect("pauli count"),
            1
        );

        assert_eq!(
            checked_pauli_assignments(3).expect("pauli count"),
            27
        );

        assert_eq!(
            checked_weight_k_paulis(2, 1).expect("pauli count"),
            6
        );
    }

    #[test]
    fn checked_distance_is_safe() {
        assert_eq!(
            checked_distance_squared(5).expect("distance"),
            25
        );

        assert_eq!(
            checked_distance_squared(0),
            Err(ArithmeticError::InvalidDistance)
        );
    }

    #[test]
    fn checked_floating_operations_reject_invalid_results() {
        assert_eq!(
            checked_div_f64(1.0, 0.0),
            Err(ArithmeticError::DivisionByZero)
        );

        assert!(checked_add_f64(1.0, 2.0).is_ok());
        assert!(checked_mul_f64(2.0, 3.0).is_ok());
    }

    #[test]
    fn gcd_and_lcm_are_correct() {
        assert_eq!(gcd_usize(48, 18), 6);

        assert_eq!(
            checked_lcm_usize(12, 18).expect("lcm"),
            36
        );
    }

    #[test]
    fn triangular_numbers_are_correct() {
        assert_eq!(
            checked_triangular_number(10).expect("triangular"),
            55
        );
    }

    #[test]
    fn explicit_bounds_are_enforced() {
        assert_eq!(
            ensure_within(5, 10).expect("within"),
            5
        );

        assert_eq!(
            ensure_within(11, 10),
            Err(ArithmeticError::LimitExceeded)
        );
    }

    #[test]
    fn range_lengths_are_safe() {
        assert_eq!(
            checked_inclusive_range_len(2, 5)
                .expect("range"),
            4
        );

        assert_eq!(
            checked_exclusive_range_len(2, 5)
                .expect("range"),
            3
        );

        assert_eq!(
            checked_inclusive_range_len(5, 2)
                .expect("empty"),
            0
        );
    }

    #[test]
    fn i64_min_absolute_value_is_rejected() {
        assert_eq!(
            checked_abs_i64(i64::MIN),
            Err(ArithmeticError::AbsoluteValueOverflow)
        );

        assert_eq!(
            checked_abs_i64(-10).expect("absolute value"),
            10
        );
    }

    #[test]
    fn zero_denominator_is_rejected() {
        assert_eq!(
            checked_div_u64(10, 0),
            Err(ArithmeticError::DivisionByZero)
        );
    }

    #[test]
    fn finite_mean_is_correct() {
        let values = [1.0, 2.0, 3.0];

        let mean =
            checked_mean_f64(&values).expect("mean");

        assert!((mean.get() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn empty_mean_is_rejected() {
        assert_eq!(
            checked_mean_f64(&[]),
            Err(ArithmeticError::InvalidOperation)
        );
    }
}