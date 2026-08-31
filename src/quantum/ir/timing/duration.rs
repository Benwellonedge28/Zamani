//! Zamani Quantum IR — Canonical Duration
//!
//! Path:
//!     src/quantum/ir/timing/duration.rs
//!
//! # Purpose
//!
//! This module defines the canonical semantic representation of a finite,
//! non-negative quantum-IR duration.
//!
//! A `Duration` represents WHAT elapsed time means in Zamani IR.
//!
//! It deliberately does not represent:
//!
//! - a wall clock;
//! - an operating-system timer;
//! - a CPU timer;
//! - a hardware sample clock;
//! - a backend `dt` unit;
//! - a pulse-channel clock;
//! - a scheduler decision;
//! - a qubit;
//! - a physical resource.
//!
//! Those concerns belong to downstream layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! canonical timing expression
//!       |
//!       v
//! quantum::ir::timing::duration::Duration
//!       |
//!       +--------------------+
//!       |                    |
//!       v                    v
//!   scheduling          pulse lowering
//!       |                    |
//!       v                    v
//! hardware timing resolution / execution
//! ```
//!
//! # Canonical representation
//!
//! A duration is stored as an exact unsigned number of attoseconds:
//!
//!     1 second = 10^18 attoseconds
//!
//! `u128` is intentionally used because the IR must not impose a small,
//! machine-specific timing range.
//!
//! The representation is:
//!
//! - exact;
//! - deterministic;
//! - allocation-free;
//! - architecture independent;
//! - safe under checked arithmetic;
//! - suitable for serialization;
//! - suitable for canonical hashing.
//!
//! `u128` is a representation boundary, not a quantum-machine-size limit.
//!
//! # Precision
//!
//! The canonical finite decimal precision is 18 fractional decimal digits.
//!
//! Examples:
//!
//!     1s
//!     20ms
//!     500us
//!     20ns
//!     5.25ns
//!     1fs
//!     1as
//!
//! A decimal value containing more than 18 fractional digits is rejected
//! rather than silently rounded.
//!
//! # Important semantic distinction
//!
//! ```text
//! Duration
//!     amount of elapsed time
//!
//! TimePoint
//!     absolute semantic position in time
//!
//! TimeOffset
//!     signed temporal displacement
//!
//! BackendTimeUnit
//!     target-specific representation
//! ```
//!
//! `Duration` owns only the first concept.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_DURATION
//!     MAX_CIRCUIT_DEPTH
//!     MAX_MACHINE_SIZE
//!     MAX_SCHEDULE_LENGTH
//!
//! in this module.
//!
//! A compiler or execution policy may impose resource limits elsewhere.
//!
//! A `Duration` is therefore valid independently of whether the eventual
//! target has one qubit, millions of qubits, or no physical QPU at all.
//!
//! # Hardware boundary
//!
//! A backend may have a clock such as:
//!
//!     dt = 0.222 ns
//!
//! The canonical IR must not replace the semantic duration with that backend
//! unit. Instead, the hardware layer converts:
//!
//!     Duration
//!         |
//!         v
//!     backend timing representation
//!
//! The conversion must be explicit and checked.
//!
//! # Dependency contract
//!
//! This module depends only on the timing parent module for:
//!
//! - `TimeUnit`;
//! - `TimingError`;
//! - `TimingResult`.
//!
//! It intentionally does NOT depend on:
//!
//! - `quantum::ir::qubit`;
//! - gates;
//! - operations;
//! - pulse;
//! - hardware;
//! - scheduling;
//! - routing;
//! - optimization;
//! - simulation.
//!
//! This keeps the duration primitive usable by all timing-aware models.
//!
//! # Integration contract
//!
//! `src/quantum/ir/timing.rs` must expose this module with:
//!
//!     pub mod duration;
//!     pub use duration::Duration;
//!
//! The parent timing module remains the owner of:
//!
//! - `TimeUnit`;
//! - `TimingError`;
//! - `TimingResult`;
//! - time points;
//! - offsets;
//! - intervals;
//! - constraints;
//! - dependencies.
//!
//! This file owns only `Duration`.
//!
//! No other timing file should redefine `Duration`.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Invariants
//!
//! Every `Duration` satisfies:
//!
//! 1. It is non-negative.
//! 2. Its canonical value is an exact number of attoseconds.
//! 3. Its internal representation is deterministic.
//! 4. Arithmetic never intentionally wraps.
//! 5. Division by zero is rejected.
//! 6. Conversion to a larger unit is never falsely reported as exact.
//! 7. Decimal parsing never silently loses precision.
//! 8. Formatting is deterministic.
//! 9. Equality is semantic equality.
//! 10. Hashing is based only on the canonical attosecond value.
//!
//! # Serialization contract
//!
//! The semantic serialization value is the canonical attosecond integer.
//!
//! Serialization layers may choose their wire representation, but they must
//! preserve the exact `u128` value and must not convert through floating point.
//!
//! # Hashing contract
//!
//! The semantic hash input is:
//!
//!     duration.attoseconds()
//!
//! No display formatting is part of the semantic identity.
//!
//! # Thread-safety
//!
//! `Duration` contains only a `u128` and is therefore `Send + Sync` through
//! Rust's normal auto-trait rules.
//!
//! # No qubit dependency
//!
//! A duration applies equally to:
//!
//! - qubit operations;
//! - classical operations;
//! - pulse operations;
//! - analog evolution;
//! - annealing schedules;
//! - measurement;
//! - distributed operations;
//! - synchronization.
//!
//! Therefore this file intentionally does not import
//! `crate::quantum::ir::qubit`.
//!
//! That is an architectural invariant, not an omission.

#![forbid(unsafe_code)]

use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::str::FromStr;

use super::{TimeUnit, TimingError, TimingResult};

// =============================================================================
// Duration
// =============================================================================

/// Exact, finite, non-negative semantic duration.
///
/// The canonical unit is the attosecond.
///
/// # Examples
///
/// ```
/// # use crate::quantum::ir::timing::duration::Duration;
///
/// let duration = Duration::nanoseconds(20);
/// assert_eq!(duration.attoseconds(), 20_000_000_000);
/// ```
///
/// A `Duration` is semantic IR data. It is not an operating-system timer and
/// does not imply that a target can physically realize the exact duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    attoseconds: u128,
}

impl Duration {
    /// Zero duration.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Maximum representable finite duration.
    ///
    /// This is the representation maximum, not a hardware limit.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    // -------------------------------------------------------------------------
    // Canonical representation
    // -------------------------------------------------------------------------

    /// Creates a duration directly from canonical attoseconds.
    ///
    /// This constructor cannot fail because every `u128` is a valid finite
    /// non-negative duration in the canonical representation.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Returns the exact canonical attosecond representation.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Returns whether the duration is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.attoseconds == 0
    }

    // -------------------------------------------------------------------------
    // Unit construction
    // -------------------------------------------------------------------------

    /// Creates a duration from an integer number of semantic time units.
    pub fn from_units(value: u128, unit: TimeUnit) -> TimingResult<Self> {
        let scale = unit.attoseconds();

        let attoseconds = value
            .checked_mul(scale)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds })
    }

    /// Creates a duration from an exact decimal quantity and a semantic unit.
    ///
    /// Examples:
    ///
    ///     Duration::from_decimal("20.5", TimeUnit::Nanosecond)
    ///     Duration::from_decimal("0.25", TimeUnit::Second)
    ///     Duration::from_decimal("1", TimeUnit::Attosecond)
    ///
    /// More than 18 fractional decimal digits are rejected.
    pub fn from_decimal(value: &str, unit: TimeUnit) -> TimingResult<Self> {
        let value = value.trim();

        if value.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        if value.starts_with('-') {
            return Err(TimingError::NegativeDuration);
        }

        if value.starts_with('+') {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        let (whole_text, fraction_text) = match value.split_once('.') {
            Some((whole, fraction)) => (whole, Some(fraction)),
            None => (value, None),
        };

        if whole_text.is_empty() && fraction_text.is_none() {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        if whole_text.is_empty() && fraction_text.is_some() {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        if whole_text.is_empty() || !whole_text.bytes().all(|b| b.is_ascii_digit()) {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        let whole = whole_text
            .parse::<u128>()
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let fractional_digits = fraction_text.unwrap_or("");

        if fractional_digits.is_empty() && value.ends_with('.') {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        if !fractional_digits
            .bytes()
            .all(|byte| byte.is_ascii_digit())
        {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        if fractional_digits.len() > super::MAX_DECIMAL_FRACTION_DIGITS {
            return Err(TimingError::ExcessivePrecision {
                digits: fractional_digits.len(),
                maximum: super::MAX_DECIMAL_FRACTION_DIGITS,
            });
        }

        let scale = unit.attoseconds();

        let whole_attoseconds = whole
            .checked_mul(scale)
            .ok_or(TimingError::ArithmeticOverflow)?;

        let fractional_attoseconds =
            decimal_fraction_to_attoseconds(fractional_digits, scale)?;

        let attoseconds = whole_attoseconds
            .checked_add(fractional_attoseconds)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds })
    }

    /// Creates zero duration.
    #[must_use]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Creates an attosecond duration.
    #[must_use]
    pub const fn attoseconds_duration(value: u128) -> Self {
        Self::from_attoseconds(value)
    }

    /// Creates a femtosecond duration.
    pub fn femtoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Femtosecond)
    }

    /// Creates a picosecond duration.
    pub fn picoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Picosecond)
    }

    /// Creates a nanosecond duration.
    pub fn nanoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Nanosecond)
    }

    /// Creates a microsecond duration.
    pub fn microseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Microsecond)
    }

    /// Creates a millisecond duration.
    pub fn milliseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Millisecond)
    }

    /// Creates a second duration.
    pub fn seconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Second)
    }

    // -------------------------------------------------------------------------
    // Exact conversions
    // -------------------------------------------------------------------------

    /// Returns the exact number of whole units contained in this duration.
    ///
    /// Returns an error when the conversion is not exact.
    pub fn to_units_exact(self, unit: TimeUnit) -> TimingResult<u128> {
        let divisor = unit.attoseconds();

        if divisor == 0 {
            return Err(TimingError::InvalidValue {
                message: "time-unit scale cannot be zero".to_owned(),
            });
        }

        if self.attoseconds % divisor != 0 {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: self.attoseconds,
            });
        }

        Ok(self.attoseconds / divisor)
    }

    /// Returns the number of whole units contained in this duration,
    /// discarding the remainder.
    ///
    /// This operation is explicit so callers cannot accidentally confuse
    /// truncation with exact conversion.
    #[must_use]
    pub fn whole_units(self, unit: TimeUnit) -> u128 {
        self.attoseconds / unit.attoseconds()
    }

    /// Returns the remainder after division into the requested unit.
    #[must_use]
    pub fn remainder(self, unit: TimeUnit) -> Self {
        let scale = unit.attoseconds();

        if scale == 0 {
            return Self::ZERO;
        }

        Self::from_attoseconds(self.attoseconds % scale)
    }

    /// Returns the exact duration in seconds as a rational pair:
    ///
    ///     numerator / denominator
    ///
    /// No floating-point conversion occurs.
    #[must_use]
    pub const fn seconds_ratio(self) -> (u128, u128) {
        (self.attoseconds, super::ATTOSECONDS_PER_SECOND)
    }

    // -------------------------------------------------------------------------
    // Checked arithmetic
    // -------------------------------------------------------------------------

    /// Checked addition.
    pub fn checked_add(self, rhs: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_add(rhs.attoseconds)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked subtraction.
    pub fn checked_sub(self, rhs: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_sub(rhs.attoseconds)
            .ok_or(TimingError::NegativeDuration)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked multiplication by a non-negative integer.
    pub fn checked_mul(self, multiplier: u128) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_mul(multiplier)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked division by a non-zero integer.
    ///
    /// Integer division truncates toward zero, which for a non-negative
    /// duration means truncation toward zero attoseconds.
    pub fn checked_div(self, divisor: u128) -> TimingResult<Self> {
        if divisor == 0 {
            return Err(TimingError::DivisionByZero);
        }

        Ok(Self::from_attoseconds(self.attoseconds / divisor))
    }

    /// Checked remainder by a non-zero integer.
    pub fn checked_rem(self, divisor: u128) -> TimingResult<Self> {
        if divisor == 0 {
            return Err(TimingError::DivisionByZero);
        }

        Ok(Self::from_attoseconds(self.attoseconds % divisor))
    }

    /// Returns the smaller of two durations.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.attoseconds <= other.attoseconds {
            self
        } else {
            other
        }
    }

    /// Returns the larger of two durations.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.attoseconds >= other.attoseconds {
            self
        } else {
            other
        }
    }

    /// Saturating addition.
    ///
    /// This is intentionally named `saturating_*` so overflow policy is
    /// explicit. Canonical compiler transformations should generally prefer
    /// `checked_add`.
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        Self::from_attoseconds(self.attoseconds.saturating_add(rhs.attoseconds))
    }

    /// Saturating subtraction.
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_attoseconds(self.attoseconds.saturating_sub(rhs.attoseconds))
    }

    /// Saturating multiplication.
    #[must_use]
    pub fn saturating_mul(self, multiplier: u128) -> Self {
        Self::from_attoseconds(self.attoseconds.saturating_mul(multiplier))
    }

    // -------------------------------------------------------------------------
    // Decimal formatting
    // -------------------------------------------------------------------------

    /// Formats the duration using the supplied semantic unit.
    ///
    /// The output is exact and never uses floating-point arithmetic.
    ///
    /// Examples:
    ///
    ///     20ns
    ///     5.25ns
    ///     1s
    #[must_use]
    pub fn format_in(self, unit: TimeUnit) -> String {
        let scale = unit.attoseconds();

        if scale == 0 {
            return format!("{}as", self.attoseconds);
        }

        let whole = self.attoseconds / scale;
        let remainder = self.attoseconds % scale;

        if remainder == 0 {
            return format!("{whole}{unit}");
        }

        let digits = decimal_digits_for_unit(scale);
        let fraction = remainder_to_decimal(remainder, scale, digits);

        format!("{whole}.{fraction}{unit}")
    }

    /// Returns a deterministic human-readable representation.
    ///
    /// The largest exact conventional unit that divides the value is chosen.
    #[must_use]
    pub fn to_string_exact(self) -> String {
        let units = [
            TimeUnit::Second,
            TimeUnit::Millisecond,
            TimeUnit::Microsecond,
            TimeUnit::Nanosecond,
            TimeUnit::Picosecond,
            TimeUnit::Femtosecond,
            TimeUnit::Attosecond,
        ];

        for unit in units {
            if self.attoseconds % unit.attoseconds() == 0 {
                return self.format_in(unit);
            }
        }

        self.format_in(TimeUnit::Attosecond)
    }
}

// =============================================================================
// Parsing
// =============================================================================

impl FromStr for Duration {
    type Err = TimingError;

    /// Parses a duration literal such as:
    ///
    ///     20ns
    ///     5.25ns
    ///     1us
    ///     2s
    ///     1fs
    ///     1as
    ///
    /// Whitespace around the complete literal is accepted.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();

        if text.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: text.to_owned(),
            });
        }

        let split_index = text
            .bytes()
            .position(|byte| byte.is_ascii_alphabetic() || byte == b'\xB5')
            .ok_or_else(|| TimingError::InvalidLiteral {
                literal: text.to_owned(),
            })?;

        let (number, unit_text) = text.split_at(split_index);

        if number.is_empty() || unit_text.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: text.to_owned(),
            });
        }

        let unit = TimeUnit::parse(unit_text)?;

        Self::from_decimal(number, unit)
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string_exact())
    }
}

// =============================================================================
// Explicit arithmetic operators
// =============================================================================

impl Add for Duration {
    type Output = TimingResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = self
            .checked_add(rhs)
            .expect("Duration addition overflow");
    }
}

impl Sub for Duration {
    type Output = TimingResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self
            .checked_sub(rhs)
            .expect("Duration subtraction underflow");
    }
}

impl Mul<u128> for Duration {
    type Output = TimingResult<Self>;

    fn mul(self, rhs: u128) -> Self::Output {
        self.checked_mul(rhs)
    }
}

impl Div<u128> for Duration {
    type Output = TimingResult<Self>;

    fn div(self, rhs: u128) -> Self::Output {
        self.checked_div(rhs)
    }
}

// =============================================================================
// Internal exact decimal helpers
// =============================================================================

/// Converts a decimal fractional component into canonical attoseconds.
///
/// The caller has already guaranteed that the number of decimal digits is at
/// most 18.
///
/// The calculation intentionally uses integer arithmetic only.
fn decimal_fraction_to_attoseconds(
    digits: &str,
    unit_scale: u128,
) -> TimingResult<u128> {
    if digits.is_empty() {
        return Ok(0);
    }

    let fraction_value = digits
        .parse::<u128>()
        .map_err(|_| TimingError::InvalidLiteral {
            literal: digits.to_owned(),
        })?;

    let denominator = pow10(digits.len()).ok_or(TimingError::ArithmeticOverflow)?;

    let numerator = fraction_value
        .checked_mul(unit_scale)
        .ok_or(TimingError::ArithmeticOverflow)?;

    if numerator % denominator != 0 {
        return Err(TimingError::ExcessivePrecision {
            digits: digits.len(),
            maximum: super::MAX_DECIMAL_FRACTION_DIGITS,
        });
    }

    Ok(numerator / denominator)
}

/// Returns 10^power exactly.
fn pow10(power: usize) -> Option<u128> {
    let mut value = 1u128;

    for _ in 0..power {
        value = value.checked_mul(10)?;
    }

    Some(value)
}

/// Returns the number of decimal places required by an exact SI unit.
///
/// Every supported SI unit is a power of ten relative to the attosecond.
fn decimal_digits_for_unit(scale: u128) -> usize {
    let mut value = scale;
    let mut digits = 0usize;

    while value > 1 {
        value /= 10;
        digits += 1;
    }

    digits
}

/// Converts an exact remainder into a fixed-width decimal fraction.
///
/// The result is trimmed on the right so:
///
///     5.250ns
///
/// becomes:
///
///     5.25ns
fn remainder_to_decimal(remainder: u128, scale: u128, width: usize) -> String {
    if remainder == 0 {
        return String::new();
    }

    let mut numerator = remainder;
    let mut output = String::with_capacity(width);

    for _ in 0..width {
        numerator *= 10;
        let digit = numerator / scale;
        numerator %= scale;

        output.push(char::from(b'0' + digit as u8));

        if numerator == 0 {
            break;
        }
    }

    while output.ends_with('0') {
        output.pop();
    }

    output
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_zero() {
        assert!(Duration::ZERO.is_zero());
        assert_eq!(Duration::ZERO.attoseconds(), 0);
    }

    #[test]
    fn integer_unit_construction_is_exact() {
        let duration = Duration::nanoseconds(20).expect("20ns should be valid");

        assert_eq!(duration.attoseconds(), 20_000_000_000);
    }

    #[test]
    fn fractional_nanoseconds_are_exact() {
        let duration =
            Duration::from_decimal("5.25", TimeUnit::Nanosecond)
                .expect("5.25ns should be exactly representable");

        assert_eq!(duration.attoseconds(), 5_250_000_000);
    }

    #[test]
    fn fractional_seconds_are_exact() {
        let duration =
            Duration::from_decimal("0.000000001", TimeUnit::Second)
                .expect("1ns expressed as seconds should be exact");

        assert_eq!(duration, Duration::nanoseconds(1).unwrap());
    }

    #[test]
    fn attosecond_is_exact() {
        let duration =
            Duration::from_decimal("1", TimeUnit::Attosecond)
                .expect("1as should be valid");

        assert_eq!(duration.attoseconds(), 1);
    }

    #[test]
    fn parses_literal() {
        let duration = "20ns"
            .parse::<Duration>()
            .expect("20ns should parse");

        assert_eq!(duration, Duration::nanoseconds(20).unwrap());
    }

    #[test]
    fn parses_fractional_literal() {
        let duration = "5.25ns"
            .parse::<Duration>()
            .expect("5.25ns should parse");

        assert_eq!(duration.attoseconds(), 5_250_000_000);
    }

    #[test]
    fn parses_microsecond_aliases() {
        let ascii = "1us"
            .parse::<Duration>()
            .expect("us should parse");

        let micro = "1µs"
            .parse::<Duration>()
            .expect("micro sign should parse");

        let greek = "1μs"
            .parse::<Duration>()
            .expect("Greek mu should parse");

        assert_eq!(ascii, micro);
        assert_eq!(micro, greek);
    }

    #[test]
    fn rejects_negative_duration() {
        let result = "-1ns".parse::<Duration>();

        assert!(matches!(result, Err(TimingError::NegativeDuration)));
    }

    #[test]
    fn rejects_positive_sign() {
        let result = "+1ns".parse::<Duration>();

        assert!(matches!(result, Err(TimingError::InvalidLiteral { .. })));
    }

    #[test]
    fn rejects_missing_unit() {
        let result = "20".parse::<Duration>();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_unit() {
        let result = "20xs".parse::<Duration>();

        assert!(matches!(result, Err(TimingError::UnknownUnit { .. })));
    }

    #[test]
    fn rejects_empty_fraction() {
        let result = "20.ns".parse::<Duration>();

        assert!(matches!(result, Err(TimingError::InvalidLiteral { .. })));
    }

    #[test]
    fn rejects_excessive_precision() {
        let result =
            Duration::from_decimal(
                "0.1234567890123456789",
                TimeUnit::Second,
            );

        assert!(matches!(
            result,
            Err(TimingError::ExcessivePrecision { .. })
        ));
    }

    #[test]
    fn checked_add_is_exact() {
        let a = Duration::nanoseconds(10).unwrap();
        let b = Duration::nanoseconds(20).unwrap();

        let result = a.checked_add(b).unwrap();

        assert_eq!(result, Duration::nanoseconds(30).unwrap());
    }

    #[test]
    fn checked_sub_is_exact() {
        let a = Duration::nanoseconds(30).unwrap();
        let b = Duration::nanoseconds(10).unwrap();

        let result = a.checked_sub(b).unwrap();

        assert_eq!(result, Duration::nanoseconds(20).unwrap());
    }

    #[test]
    fn checked_sub_rejects_negative_result() {
        let a = Duration::nanoseconds(10).unwrap();
        let b = Duration::nanoseconds(20).unwrap();

        assert!(matches!(
            a.checked_sub(b),
            Err(TimingError::NegativeDuration)
        ));
    }

    #[test]
    fn checked_multiplication_is_exact() {
        let duration = Duration::nanoseconds(20)
            .unwrap()
            .checked_mul(5)
            .unwrap();

        assert_eq!(duration, Duration::nanoseconds(100).unwrap());
    }

    #[test]
    fn checked_division_is_exact_for_whole_units() {
        let duration = Duration::nanoseconds(100)
            .unwrap()
            .checked_div(4)
            .unwrap();

        assert_eq!(duration, Duration::nanoseconds(25).unwrap());
    }

    #[test]
    fn checked_division_rejects_zero() {
        let duration = Duration::nanoseconds(100).unwrap();

        assert!(matches!(
            duration.checked_div(0),
            Err(TimingError::DivisionByZero)
        ));
    }

    #[test]
    fn exact_conversion_succeeds() {
        let duration = Duration::nanoseconds(20).unwrap();

        assert_eq!(
            duration
                .to_units_exact(TimeUnit::Nanosecond)
                .unwrap(),
            20
        );
    }

    #[test]
    fn exact_conversion_rejects_remainder() {
        let duration = Duration::nanoseconds(25).unwrap();

        assert!(matches!(
            duration.to_units_exact(TimeUnit::Nanosecond),
            Ok(25)
        ));

        assert!(matches!(
            duration.to_units_exact(TimeUnit::Microsecond),
            Err(TimingError::InexactBackendConversion { .. })
        ));
    }

    #[test]
    fn whole_units_are_explicitly_truncating() {
        let duration = Duration::nanoseconds(1_500).unwrap();

        assert_eq!(
            duration.whole_units(TimeUnit::Microsecond),
            1
        );
    }

    #[test]
    fn remainder_is_exact() {
        let duration = Duration::nanoseconds(1_500).unwrap();

        assert_eq!(
            duration.remainder(TimeUnit::Microsecond),
            Duration::nanoseconds(500).unwrap()
        );
    }

    #[test]
    fn format_integer_duration() {
        let duration = Duration::nanoseconds(20).unwrap();

        assert_eq!(duration.format_in(TimeUnit::Nanosecond), "20ns");
        assert_eq!(duration.to_string_exact(), "20ns");
    }

    #[test]
    fn format_fractional_duration() {
        let duration =
            Duration::from_decimal("5.25", TimeUnit::Nanosecond)
                .unwrap();

        assert_eq!(
            duration.format_in(TimeUnit::Nanosecond),
            "5.25ns"
        );
    }

    #[test]
    fn display_is_deterministic() {
        let duration =
            Duration::from_decimal("1.5", TimeUnit::Nanosecond)
                .unwrap();

        assert_eq!(duration.to_string(), "1.5ns");
    }

    #[test]
    fn ordering_is_semantic() {
        let small = Duration::nanoseconds(1).unwrap();
        let large = Duration::nanoseconds(2).unwrap();

        assert!(small < large);
    }

    #[test]
    fn hashing_identity_is_canonical() {
        use std::collections::HashSet;

        let one_second = Duration::seconds(1).unwrap();
        let one_billion_nanoseconds =
            Duration::nanoseconds(1_000_000_000).unwrap();

        let mut set = HashSet::new();
        set.insert(one_second);

        assert!(set.contains(&one_billion_nanoseconds));
    }

    #[test]
    fn max_duration_is_representable() {
        assert_eq!(Duration::MAX.attoseconds(), u128::MAX);
    }

    #[test]
    fn checked_overflow_is_rejected() {
        let result = Duration::MAX.checked_add(Duration::from_attoseconds(1));

        assert!(matches!(
            result,
            Err(TimingError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn checked_multiplication_overflow_is_rejected() {
        let result = Duration::MAX.checked_mul(2);

        assert!(matches!(
            result,
            Err(TimingError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn saturating_add_is_explicit() {
        let result = Duration::MAX.saturating_add(Duration::from_attoseconds(1));

        assert_eq!(result, Duration::MAX);
    }

    #[test]
    fn seconds_ratio_is_exact() {
        let duration = Duration::nanoseconds(1).unwrap();

        let (numerator, denominator) = duration.seconds_ratio();

        assert_eq!(numerator, 1_000_000_000);
        assert_eq!(denominator, 1_000_000_000_000_000_000);
    }

    #[test]
    fn arithmetic_operator_returns_checked_result() {
        let a = Duration::nanoseconds(10).unwrap();
        let b = Duration::nanoseconds(20).unwrap();

        let result = (a + b).unwrap();

        assert_eq!(result, Duration::nanoseconds(30).unwrap());
    }

    #[test]
    fn arithmetic_subtraction_returns_checked_result() {
        let a = Duration::nanoseconds(30).unwrap();
        let b = Duration::nanoseconds(10).unwrap();

        let result = (a - b).unwrap();

        assert_eq!(result, Duration::nanoseconds(20).unwrap());
    }

    #[test]
    fn decimal_round_trip_is_exact() {
        let original =
            Duration::from_decimal("123.456789", TimeUnit::Nanosecond)
                .unwrap();

        let rendered = original.format_in(TimeUnit::Nanosecond);

        let reparsed = rendered
            .parse::<Duration>()
            .unwrap();

        assert_eq!(original, reparsed);
    }

    #[test]
    fn whitespace_is_accepted_around_literal() {
        let duration = "  20ns  "
            .parse::<Duration>()
            .unwrap();

        assert_eq!(duration, Duration::nanoseconds(20).unwrap());
    }
}