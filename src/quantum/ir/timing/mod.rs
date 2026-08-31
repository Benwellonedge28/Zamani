//! Zamani Quantum IR — Canonical Timing Model
//!
//! Production-grade, hardware-independent, deterministic timing semantics for
//! the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::timing` defines the semantic meaning of time.
//!
//! It owns:
//!
//! - exact semantic durations;
//! - absolute semantic time points;
//! - signed temporal offsets;
//! - semantic time units;
//! - exact timing-literal parsing;
//! - checked timing arithmetic;
//! - half-open temporal intervals;
//! - timing bounds;
//! - timing constraints;
//! - temporal dependencies;
//! - synchronization semantics;
//! - symbolic timing stretches;
//! - unresolved/resolved scheduling intent;
//! - delay semantics;
//! - nested timing domains;
//! - backend timing conversion boundaries;
//! - deterministic formatting;
//! - timing-specific validation.
//!
//! It does NOT own:
//!
//! - hardware clocks;
//! - physical sample rates;
//! - DAC/ADC configuration;
//! - physical calibration;
//! - waveform synthesis;
//! - routing;
//! - scheduling algorithms;
//! - hardware topology;
//! - optimization policy;
//! - QPU execution;
//! - simulator state evolution.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! frontend
//!       |
//!       v
//! quantum::ir
//!       |
//!       +------------------------------+
//!       |                              |
//!       v                              v
//! semantic timing                 quantum operations
//!       |                              |
//!       +---------------+--------------+
//!                       |
//!                       v
//!                  scheduling
//!                       |
//!                       v
//!                    hardware
//!                       |
//!                       v
//!                    backend
//! ```
//!
//! `timing` therefore answers:
//!
//! > What does this temporal relationship mean?
//!
//! It does not answer:
//!
//! > Which physical clock realizes it?
//!
//! # Universal-program principle
//!
//! Timing has no machine-size dependency.
//!
//! The same semantic timing model applies to:
//!
//! - one qubit;
//! - small QPUs;
//! - large QPUs;
//! - distributed quantum systems;
//! - fault-tolerant systems;
//! - pulse-controlled processors;
//! - analog systems;
//! - annealers;
//! - simulators;
//! - future quantum architectures.
//!
//! There is deliberately no:
//!
//! - maximum qubit count;
//! - maximum channel count;
//! - maximum operation count;
//! - maximum schedule size;
//! - fixed hardware topology;
//! - vendor-specific clock;
//! - vendor-specific sample rate.
//!
//! Explicit compilation/security limits belong to `quantum::ir::limits`.
//!
//! # Canonical representation
//!
//! Concrete semantic time is represented as an integer number of attoseconds:
//!
//! ```text
//! 1 second = 1_000_000_000_000_000_000 attoseconds
//! ```
//!
//! `u128` is used for unsigned semantic time values.
//!
//! This is an implementation representation, not a claim that hardware has
//! attosecond physical resolution.
//!
//! Hardware-specific timing resolution is resolved downstream.
//!
//! # Why signed offsets use sign + magnitude
//!
//! A naïve signed representation using `i128` cannot represent every possible
//! difference between two `u128` time points. The semantic model therefore uses:
//!
//! ```text
//! TimeOffset {
//!     negative: bool,
//!     magnitude: u128,
//! }
//! ```
//!
//! This permits the complete representable `u128` time domain without silently
//! introducing an `i128::MAX` architectural ceiling.
//!
//! # Timing and qubits
//!
//! This module intentionally does not import `quantum::ir::qubit::QubitId`.
//!
//! Timing is reusable for:
//!
//! - quantum operations;
//! - classical operations;
//! - pulse operations;
//! - analog evolution;
//! - synchronization;
//! - distributed execution.
//!
//! Where a timing-aware object actually owns qubit operands, that object must
//! use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Timing itself must remain independent of qubit identity.
//!
//! # Integration
//!
//! `operation.rs` may attach timing intent to operations.
//!
//! `pulse.rs` consumes `Duration`, `TimePoint`, `TimeOffset`, `DelaySpec`,
//! `TimingBounds`, and related types.
//!
//! `schedule.rs` represents scheduling results.
//!
//! `scheduling/` computes schedules.
//!
//! `hardware/` resolves semantic timing against target capabilities.
//!
//! `validation.rs` performs whole-program validation.
//!
//! `analysis.rs` uses checked timing arithmetic for duration/depth/critical-path
//! analysis.
//!
//! `serialization.rs` must serialize these values using their canonical
//! representations.
//!
//! `hash.rs` must hash canonical semantic timing values, not debug formatting.
//!
//! `qubit.rs` remains independent from this module.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! This module deliberately contains:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Filesystem migration
//!
//! This file replaces:
//!
//! ```text
//! src/quantum/ir/timing.rs
//! ```
//!
//! with:
//!
//! ```text
//! src/quantum/ir/timing/mod.rs
//! ```
//!
//! Do not keep both files because Rust treats both as competing module roots.
//!
//! The long-term structure can later split this module into:
//!
//! ```text
//! timing/
//! ├── mod.rs
//! ├── duration.rs
//! ├── instant.rs
//! ├── interval.rs
//! ├── constraint.rs
//! ├── dependency.rs
//! └── expression.rs
//! ```
//!
//! without changing the public semantic contracts defined here.
//!
//! -----------------------------------------------------------------------------
//! No hardware, routing, optimization, scheduling algorithm, or backend logic
//! belongs in this module.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::str::FromStr;

// =============================================================================
// Canonical scales
// =============================================================================

/// Number of attoseconds in one femtosecond.
pub const ATTOSECONDS_PER_FEMTOSECOND: u128 = 1_000;

/// Number of attoseconds in one picosecond.
pub const ATTOSECONDS_PER_PICOSECOND: u128 = 1_000_000;

/// Number of attoseconds in one nanosecond.
pub const ATTOSECONDS_PER_NANOSECOND: u128 = 1_000_000_000;

/// Number of attoseconds in one microsecond.
pub const ATTOSECONDS_PER_MICROSECOND: u128 = 1_000_000_000_000;

/// Number of attoseconds in one millisecond.
pub const ATTOSECONDS_PER_MILLISECOND: u128 = 1_000_000_000_000_000;

/// Number of attoseconds in one second.
pub const ATTOSECONDS_PER_SECOND: u128 = 1_000_000_000_000_000_000;

/// Maximum decimal fractional precision representable exactly by the
/// attosecond canonical representation.
pub const MAX_DECIMAL_FRACTION_DIGITS: usize = 18;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the canonical timing model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingError {
    /// A supplied timing value is invalid.
    InvalidValue {
        /// Human-readable explanation.
        message: String,
    },

    /// A timing literal contains an unsupported unit.
    UnknownUnit {
        /// Supplied unit.
        unit: String,
    },

    /// A timing literal is malformed.
    InvalidLiteral {
        /// Original literal.
        literal: String,
    },

    /// Decimal precision cannot be represented exactly.
    ExcessivePrecision {
        /// Supplied fractional digits.
        digits: usize,

        /// Maximum exact fractional digits.
        maximum: usize,
    },

    /// Checked arithmetic overflow.
    ArithmeticOverflow,

    /// A duration subtraction would become negative.
    NegativeDuration,

    /// Division by zero.
    DivisionByZero,

    /// Interval bounds are invalid.
    InvalidInterval {
        /// Start coordinate.
        start: u128,

        /// End coordinate.
        end: u128,
    },

    /// A timing constraint is internally inconsistent.
    InvalidConstraint {
        /// Human-readable explanation.
        message: String,
    },

    /// A backend conversion is not exact.
    InexactBackendConversion {
        /// Value that could not be represented.
        attoseconds: u128,
    },

    /// Backend tick scale is invalid.
    InvalidBackendScale,

    /// A stretch is invalid.
    InvalidStretch {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for TimingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { message } => {
                write!(f, "invalid timing value: {message}")
            }
            Self::UnknownUnit { unit } => {
                write!(f, "unknown timing unit `{unit}`")
            }
            Self::InvalidLiteral { literal } => {
                write!(f, "invalid timing literal `{literal}`")
            }
            Self::ExcessivePrecision { digits, maximum } => {
                write!(
                    f,
                    "timing literal contains {digits} fractional digits; \
                     maximum exact precision is {maximum}"
                )
            }
            Self::ArithmeticOverflow => {
                f.write_str("timing arithmetic overflow")
            }
            Self::NegativeDuration => {
                f.write_str("operation would produce a negative duration")
            }
            Self::DivisionByZero => {
                f.write_str("timing division by zero")
            }
            Self::InvalidInterval { start, end } => {
                write!(
                    f,
                    "invalid timing interval: start {start} exceeds end {end}"
                )
            }
            Self::InvalidConstraint { message } => {
                write!(f, "invalid timing constraint: {message}")
            }
            Self::InexactBackendConversion { attoseconds } => {
                write!(
                    f,
                    "timing value {attoseconds} attoseconds cannot be \
                     represented exactly in the selected backend unit"
                )
            }
            Self::InvalidBackendScale => {
                f.write_str("backend timing scale must be non-zero")
            }
            Self::InvalidStretch { message } => {
                write!(f, "invalid timing stretch: {message}")
            }
        }
    }
}

impl std::error::Error for TimingError {}

/// Result type used by the timing module.
pub type TimingResult<T> = Result<T, TimingError>;

// =============================================================================
// Time unit
// =============================================================================

/// Exact semantic time unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnit {
    /// Attosecond.
    Attosecond,

    /// Femtosecond.
    Femtosecond,

    /// Picosecond.
    Picosecond,

    /// Nanosecond.
    Nanosecond,

    /// Microsecond.
    Microsecond,

    /// Millisecond.
    Millisecond,

    /// Second.
    Second,
}

impl TimeUnit {
    /// Returns the number of attoseconds in one unit.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        match self {
            Self::Attosecond => 1,
            Self::Femtosecond => ATTOSECONDS_PER_FEMTOSECOND,
            Self::Picosecond => ATTOSECONDS_PER_PICOSECOND,
            Self::Nanosecond => ATTOSECONDS_PER_NANOSECOND,
            Self::Microsecond => ATTOSECONDS_PER_MICROSECOND,
            Self::Millisecond => ATTOSECONDS_PER_MILLISECOND,
            Self::Second => ATTOSECONDS_PER_SECOND,
        }
    }

    /// Returns the canonical source spelling.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Attosecond => "as",
            Self::Femtosecond => "fs",
            Self::Picosecond => "ps",
            Self::Nanosecond => "ns",
            Self::Microsecond => "us",
            Self::Millisecond => "ms",
            Self::Second => "s",
        }
    }

    /// Parses a unit.
    pub fn parse(text: &str) -> TimingResult<Self> {
        match text.trim() {
            "as" => Ok(Self::Attosecond),
            "fs" => Ok(Self::Femtosecond),
            "ps" => Ok(Self::Picosecond),
            "ns" => Ok(Self::Nanosecond),
            "us" | "µs" | "μs" => Ok(Self::Microsecond),
            "ms" => Ok(Self::Millisecond),
            "s" => Ok(Self::Second),
            other => Err(TimingError::UnknownUnit {
                unit: other.to_owned(),
            }),
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

impl FromStr for TimeUnit {
    type Err = TimingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

// =============================================================================
// Duration
// =============================================================================

/// Exact non-negative semantic duration.
///
/// The canonical representation is attoseconds stored as `u128`.
///
/// This type is deliberately separate from `std::time::Duration` because
/// semantic quantum timing is not host-clock timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    attoseconds: u128,
}

impl Duration {
    /// Zero duration.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Maximum representable semantic duration.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    /// Creates a duration directly from attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Returns canonical attoseconds.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Creates an exact integer-unit duration.
    pub fn from_units(value: u128, unit: TimeUnit) -> TimingResult<Self> {
        let attoseconds = value
            .checked_mul(unit.attoseconds())
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds })
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

    /// Parses a complete timing literal.
    ///
    /// Examples:
    ///
    /// ```text
    /// 20ns
    /// 20 ns
    /// 1.5ns
    /// 0.25us
    /// 1as
    /// ```
    pub fn parse(literal: &str) -> TimingResult<Self> {
        let trimmed = literal.trim();

        if trimmed.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            });
        }

        let split = trimmed
            .find(|character: char| {
                character.is_ascii_alphabetic()
                    || character == 'µ'
                    || character == 'μ'
            })
            .ok_or_else(|| TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            })?;

        let number = trimmed[..split].trim();
        let unit = trimmed[split..].trim();

        if number.is_empty() || unit.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            });
        }

        let parsed_unit = TimeUnit::parse(unit)?;
        Self::from_decimal(number, parsed_unit)
    }

    /// Parses an exact decimal magnitude in a supplied unit.
    pub fn from_decimal(
        value: &str,
        unit: TimeUnit,
    ) -> TimingResult<Self> {
        parse_decimal_duration(value, unit)
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.attoseconds == 0
    }

    /// Returns whether the duration is non-zero.
    #[must_use]
    pub const fn is_nonzero(self) -> bool {
        self.attoseconds != 0
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_add(other.attoseconds)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_sub(other.attoseconds)
            .ok_or(TimingError::NegativeDuration)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked multiplication by an unsigned integer.
    pub fn checked_mul(self, factor: u128) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_mul(factor)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Checked division by an unsigned integer.
    ///
    /// This performs integer division in the canonical attosecond domain.
    /// Remainders are discarded intentionally.
    pub fn checked_div(self, divisor: u128) -> TimingResult<Self> {
        if divisor == 0 {
            return Err(TimingError::DivisionByZero);
        }

        Ok(Self::from_attoseconds(
            self.attoseconds / divisor,
        ))
    }

    /// Returns the exact integer number of units.
    ///
    /// Fails if the duration is not an exact multiple of the requested unit.
    pub fn to_units_exact(self, unit: TimeUnit) -> TimingResult<u128> {
        let scale = unit.attoseconds();

        if self.attoseconds % scale != 0 {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: self.attoseconds,
            });
        }

        Ok(self.attoseconds / scale)
    }

    /// Returns whole units, discarding a smaller-unit remainder.
    #[must_use]
    pub const fn whole_units(self, unit: TimeUnit) -> u128 {
        self.attoseconds / unit.attoseconds()
    }

    /// Returns the remainder after extracting whole units.
    #[must_use]
    pub const fn remainder_attoseconds(
        self,
        unit: TimeUnit,
    ) -> u128 {
        self.attoseconds % unit.attoseconds()
    }

    /// Returns a deterministic human-readable canonical representation.
    #[must_use]
    pub fn canonical_string(self) -> String {
        if self.is_zero() {
            return "0as".to_owned();
        }

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
                return format!(
                    "{}{}",
                    self.attoseconds / unit.attoseconds(),
                    unit
                );
            }
        }

        format!("{}as", self.attoseconds)
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

impl FromStr for Duration {
    type Err = TimingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

// =============================================================================
// Signed time offset
// =============================================================================

/// Signed relative temporal displacement.
///
/// Unlike an `i128`, this representation can express the complete difference
/// domain of two `u128` time points.
///
/// A negative offset is represented by `negative = true` and a non-zero
/// magnitude.
///
/// Zero is always canonicalized to `negative = false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeOffset {
    negative: bool,
    magnitude: u128,
}

impl TimeOffset {
    /// Zero offset.
    pub const ZERO: Self = Self {
        negative: false,
        magnitude: 0,
    };

    /// Creates an offset from sign and magnitude.
    ///
    /// Zero is canonicalized to positive zero.
    #[must_use]
    pub const fn from_parts(
        negative: bool,
        magnitude: u128,
    ) -> Self {
        if magnitude == 0 {
            Self::ZERO
        } else {
            Self {
                negative,
                magnitude,
            }
        }
    }

    /// Creates a zero or positive offset.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self::from_parts(false, attoseconds)
    }

    /// Creates a positive offset from a duration.
    #[must_use]
    pub const fn positive(duration: Duration) -> Self {
        Self::from_attoseconds(duration.attoseconds())
    }

    /// Creates a negative offset from a duration.
    #[must_use]
    pub const fn negative(duration: Duration) -> Self {
        Self::from_parts(true, duration.attoseconds())
    }

    /// Returns whether the offset is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.negative
    }

    /// Returns whether the offset is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.magnitude == 0
    }

    /// Returns the unsigned magnitude.
    #[must_use]
    pub const fn magnitude(self) -> u128 {
        self.magnitude
    }

    /// Returns the signed mathematical value when it fits in `i128`.
    ///
    /// This is intentionally fallible because the semantic offset domain is
    /// wider than `i128`.
    pub fn to_i128(self) -> TimingResult<i128> {
        let magnitude = i128::try_from(self.magnitude)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        if self.negative {
            magnitude
                .checked_neg()
                .ok_or(TimingError::ArithmeticOverflow)
        } else {
            Ok(magnitude)
        }
    }

    /// Applies this offset to a time point.
    pub fn apply(self, point: TimePoint) -> TimingResult<TimePoint> {
        if self.negative {
            point.checked_sub(
                Duration::from_attoseconds(self.magnitude),
            )
        } else {
            point.checked_add(
                Duration::from_attoseconds(self.magnitude),
            )
        }
    }

    /// Returns the negated offset.
    #[must_use]
    pub const fn negated(self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else {
            Self {
                negative: !self.negative,
                magnitude: self.magnitude,
            }
        }
    }

    /// Checked addition of two offsets.
    pub fn checked_add(self, other: Self) -> TimingResult<Self> {
        match (self.negative, other.negative) {
            (false, false) => {
                let magnitude = self
                    .magnitude
                    .checked_add(other.magnitude)
                    .ok_or(TimingError::ArithmeticOverflow)?;

                Ok(Self::from_parts(false, magnitude))
            }

            (true, true) => {
                let magnitude = self
                    .magnitude
                    .checked_add(other.magnitude)
                    .ok_or(TimingError::ArithmeticOverflow)?;

                Ok(Self::from_parts(true, magnitude))
            }

            (false, true) => subtract_offsets(
                self.magnitude,
                other.magnitude,
            ),

            (true, false) => {
                let result = subtract_offsets(
                    other.magnitude,
                    self.magnitude,
                )?;

                Ok(result.negated())
            }
        }
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> TimingResult<Self> {
        self.checked_add(other.negated())
    }
}

impl Default for TimeOffset {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialOrd for TimeOffset {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeOffset {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        match (self.negative, other.negative) {
            (false, true) => std::cmp::Ordering::Greater,
            (true, false) => std::cmp::Ordering::Less,
            _ => {
                if self.negative {
                    other.magnitude.cmp(&self.magnitude)
                } else {
                    self.magnitude.cmp(&other.magnitude)
                }
            }
        }
    }
}

impl fmt::Display for TimeOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(f, "-{}as", self.magnitude)
        } else {
            write!(f, "{}as", self.magnitude)
        }
    }
}

// =============================================================================
// Time point
// =============================================================================

/// Absolute semantic time coordinate.
///
/// This is not wall-clock time and has no relationship to `SystemTime`.
///
/// It represents time relative to the beginning of its enclosing semantic
/// timing domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimePoint {
    attoseconds: u128,
}

impl TimePoint {
    /// Semantic time zero.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Maximum representable semantic time point.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    /// Creates a time point directly from attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Creates a time point from a duration measured from zero.
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Self {
        Self::from_attoseconds(duration.attoseconds())
    }

    /// Returns canonical attoseconds.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Returns the point as a duration from semantic zero.
    #[must_use]
    pub const fn as_duration(self) -> Duration {
        Duration::from_attoseconds(self.attoseconds)
    }

    /// Adds a duration.
    pub fn checked_add(
        self,
        duration: Duration,
    ) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_add(duration.attoseconds())
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Subtracts a duration.
    pub fn checked_sub(
        self,
        duration: Duration,
    ) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_sub(duration.attoseconds())
            .ok_or(TimingError::NegativeDuration)?;

        Ok(Self::from_attoseconds(value))
    }

    /// Applies a signed temporal offset.
    pub fn checked_offset(
        self,
        offset: TimeOffset,
    ) -> TimingResult<Self> {
        offset.apply(self)
    }

    /// Returns the signed offset from another point.
    ///
    /// The result is representable across the entire `u128` point domain.
    pub fn offset_from(
        self,
        other: Self,
    ) -> TimeOffset {
        if self.attoseconds >= other.attoseconds {
            TimeOffset::from_parts(
                false,
                self.attoseconds - other.attoseconds,
            )
        } else {
            TimeOffset::from_parts(
                true,
                other.attoseconds - self.attoseconds,
            )
        }
    }

    /// Returns the absolute unsigned distance between two points.
    #[must_use]
    pub fn distance(self, other: Self) -> u128 {
        if self.attoseconds >= other.attoseconds {
            self.attoseconds - other.attoseconds
        } else {
            other.attoseconds - self.attoseconds
        }
    }
}

impl Default for TimePoint {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}as", self.attoseconds)
    }
}

// =============================================================================
// Time interval
// =============================================================================

/// Half-open semantic interval `[start, end)`.
///
/// Equal start and end values are valid and represent zero duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeInterval {
    start: TimePoint,
    end: TimePoint,
}

impl TimeInterval {
    /// Creates `[start, end)`.
    pub fn new(
        start: TimePoint,
        end: TimePoint,
    ) -> TimingResult<Self> {
        if start > end {
            return Err(TimingError::InvalidInterval {
                start: start.attoseconds(),
                end: end.attoseconds(),
            });
        }

        Ok(Self { start, end })
    }

    /// Creates an interval from start plus duration.
    pub fn from_start_duration(
        start: TimePoint,
        duration: Duration,
    ) -> TimingResult<Self> {
        let end = start.checked_add(duration)?;
        Self::new(start, end)
    }

    /// Returns start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns exclusive end.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns interval duration.
    pub fn duration(self) -> TimingResult<Duration> {
        let value = self
            .end
            .attoseconds()
            .checked_sub(self.start.attoseconds())
            .ok_or(TimingError::InvalidInterval {
                start: self.start.attoseconds(),
                end: self.end.attoseconds(),
            })?;

        Ok(Duration::from_attoseconds(value))
    }

    /// Returns whether the interval is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.attoseconds() == self.end.attoseconds()
    }

    /// Returns whether the intervals overlap with positive duration.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns whether the intervals touch at a boundary.
    #[must_use]
    pub const fn touches(self, other: Self) -> bool {
        self.end == other.start || other.end == self.start
    }

    /// Returns whether the interval contains a point.
    ///
    /// The end point is exclusive.
    #[must_use]
    pub const fn contains(self, point: TimePoint) -> bool {
        self.start <= point && point < self.end
    }

    /// Returns the union of overlapping/touching intervals.
    ///
    /// Returns `None` when the intervals are disjoint.
    pub fn union(self, other: Self) -> Option<Self> {
        if !self.overlaps(other)
            && !self.touches(other)
            && !self.is_empty()
            && !other.is_empty()
        {
            return None;
        }

        let start = if self.start <= other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end >= other.end {
            self.end
        } else {
            other.end
        };

        Some(Self { start, end })
    }
}

// =============================================================================
// Backend timing
// =============================================================================

/// Explicit backend-dependent timing unit.
///
/// This is deliberately separate from `TimeUnit`.
///
/// For example, a backend may define a tick (`dt`) as an exact number of
/// attoseconds. The backend/hardware layer owns that physical interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackendTimeUnit {
    attoseconds_per_tick: u128,
}

impl BackendTimeUnit {
    /// Creates a backend tick unit.
    pub fn from_attoseconds_per_tick(
        attoseconds_per_tick: u128,
    ) -> TimingResult<Self> {
        if attoseconds_per_tick == 0 {
            return Err(TimingError::InvalidBackendScale);
        }

        Ok(Self {
            attoseconds_per_tick,
        })
    }

    /// Returns the attoseconds represented by one backend tick.
    #[must_use]
    pub const fn attoseconds_per_tick(self) -> u128 {
        self.attoseconds_per_tick
    }

    /// Converts a semantic duration to exact backend ticks.
    pub fn duration_to_ticks(
        self,
        duration: Duration,
    ) -> TimingResult<u128> {
        if duration.attoseconds()
            % self.attoseconds_per_tick
            != 0
        {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: duration.attoseconds(),
            });
        }

        Ok(duration.attoseconds() / self.attoseconds_per_tick)
    }

    /// Converts exact backend ticks to a semantic duration.
    pub fn ticks_to_duration(
        self,
        ticks: u128,
    ) -> TimingResult<Duration> {
        let attoseconds = ticks
            .checked_mul(self.attoseconds_per_tick)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Duration::from_attoseconds(attoseconds))
    }

    /// Converts a semantic time point to exact backend ticks.
    pub fn time_point_to_ticks(
        self,
        point: TimePoint,
    ) -> TimingResult<u128> {
        self.duration_to_ticks(point.as_duration())
    }

    /// Converts exact backend ticks to a semantic time point.
    pub fn ticks_to_time_point(
        self,
        ticks: u128,
    ) -> TimingResult<TimePoint> {
        Ok(TimePoint::from_duration(
            self.ticks_to_duration(ticks)?,
        ))
    }
}

// =============================================================================
// Timing bounds
// =============================================================================

/// Lower and upper bounds on a duration.
///
/// `None` means unbounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingBounds {
    minimum: Option<Duration>,
    maximum: Option<Duration>,
}

impl TimingBounds {
    /// Creates unbounded timing.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            minimum: None,
            maximum: None,
        }
    }

    /// Creates an exact duration bound.
    #[must_use]
    pub const fn exact(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: Some(duration),
        }
    }

    /// Creates a lower-only bound.
    #[must_use]
    pub const fn at_least(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: None,
        }
    }

    /// Creates an upper-only bound.
    #[must_use]
    pub const fn at_most(duration: Duration) -> Self {
        Self {
            minimum: None,
            maximum: Some(duration),
        }
    }

    /// Creates explicit bounds.
    pub fn new(
        minimum: Option<Duration>,
        maximum: Option<Duration>,
    ) -> TimingResult<Self> {
        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            if minimum > maximum {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "minimum duration exceeds maximum duration"
                            .to_owned(),
                });
            }
        }

        Ok(Self { minimum, maximum })
    }

    /// Returns the lower bound.
    #[must_use]
    pub const fn minimum(self) -> Option<Duration> {
        self.minimum
    }

    /// Returns the upper bound.
    #[must_use]
    pub const fn maximum(self) -> Option<Duration> {
        self.maximum
    }

    /// Returns whether a duration satisfies the bounds.
    #[must_use]
    pub fn contains(self, duration: Duration) -> bool {
        if let Some(minimum) = self.minimum {
            if duration < minimum {
                return false;
            }
        }

        if let Some(maximum) = self.maximum {
            if duration > maximum {
                return false;
            }
        }

        true
    }

    /// Returns whether the bounds are exact.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        match (self.minimum, self.maximum) {
            (Some(minimum), Some(maximum)) => minimum == maximum,
            _ => false,
        }
    }

    /// Returns the exact value when the bounds are exact.
    #[must_use]
    pub const fn exact_value(self) -> Option<Duration> {
        match (self.minimum, self.maximum) {
            (Some(minimum), Some(maximum))
                if minimum == maximum =>
            {
                Some(minimum)
            }
            _ => None,
        }
    }
}

// =============================================================================
// Temporal relations
// =============================================================================

/// Semantic temporal relationship between two events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalRelation {
    /// First event strictly precedes second.
    Before,

    /// First event occurs at or before second.
    BeforeOrAt,

    /// Both events occur at the same time.
    Simultaneous,

    /// First event strictly follows second.
    After,

    /// First event occurs at or after second.
    AfterOrAt,

    /// Second event starts at least this duration after first.
    SeparatedByAtLeast(Duration),

    /// Second event starts no more than this duration after first.
    SeparatedByAtMost(Duration),

    /// Second event starts exactly this duration after first.
    SeparatedByExactly(Duration),
}

impl TemporalRelation {
    /// Returns whether this relation contains a non-negative separation.
    #[must_use]
    pub const fn minimum_separation(self) -> Option<Duration> {
        match self {
            Self::SeparatedByAtLeast(duration)
            | Self::SeparatedByExactly(duration) => {
                Some(duration)
            }
            Self::Before
            | Self::BeforeOrAt
            | Self::Simultaneous
            | Self::After
            | Self::AfterOrAt
            | Self::SeparatedByAtMost(_) => None,
        }
    }
}

// =============================================================================
// Timing constraint
// =============================================================================

/// Hardware-independent timing constraint between two semantic identifiers.
///
/// `Id` is generic so this type can be used for:
///
/// - `OperationId`;
/// - `PulseId`;
/// - `ResourceId`;
/// - `BlockId`;
/// - other stable IR identities.
///
/// The timing module therefore does not need to depend on those higher-level
/// modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingConstraint<Id> {
    first: Id,
    second: Id,
    relation: TemporalRelation,
}

impl<Id> TimingConstraint<Id> {
    /// Creates a timing constraint.
    #[must_use]
    pub const fn new(
        first: Id,
        relation: TemporalRelation,
        second: Id,
    ) -> Self {
        Self {
            first,
            second,
            relation,
        }
    }

    /// Returns the first identifier.
    #[must_use]
    pub const fn first(&self) -> &Id {
        &self.first
    }

    /// Returns the second identifier.
    #[must_use]
    pub const fn second(&self) -> &Id {
        &self.second
    }

    /// Returns the relation.
    #[must_use]
    pub const fn relation(&self) -> TemporalRelation {
        self.relation
    }

    /// Returns whether the constraint is structurally self-referential.
    ///
    /// This does not reject the constraint because some IR clients may use
    /// self-references as diagnostics or intermediate construction states.
    pub fn is_self_referential(&self) -> bool
    where
        Id: PartialEq,
    {
        self.first == self.second
    }

    /// Validates the constraint's local semantics.
    pub fn validate(&self) -> TimingResult<()>
    where
        Id: PartialEq,
    {
        if self.is_self_referential() {
            match self.relation {
                TemporalRelation::Before
                | TemporalRelation::After
                | TemporalRelation::SeparatedByAtLeast(_)
                | TemporalRelation::SeparatedByAtMost(_)
                | TemporalRelation::SeparatedByExactly(_) => {
                    return Err(TimingError::InvalidConstraint {
                        message:
                            "strict or separated timing relation \
                             cannot target the same event"
                                .to_owned(),
                    });
                }
                TemporalRelation::BeforeOrAt
                | TemporalRelation::AfterOrAt
                | TemporalRelation::Simultaneous => {}
            }
        }

        Ok(())
    }
}

// =============================================================================
// Temporal dependency
// =============================================================================

/// Explicit temporal dependency between two semantic identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemporalDependency<Id> {
    predecessor: Id,
    successor: Id,
    minimum_separation: Duration,
}

impl<Id> TemporalDependency<Id> {
    /// Creates a zero-separation dependency.
    #[must_use]
    pub const fn new(
        predecessor: Id,
        successor: Id,
    ) -> Self {
        Self {
            predecessor,
            successor,
            minimum_separation: Duration::ZERO,
        }
    }

    /// Creates a dependency with minimum separation.
    #[must_use]
    pub const fn with_minimum_separation(
        predecessor: Id,
        successor: Id,
        minimum_separation: Duration,
    ) -> Self {
        Self {
            predecessor,
            successor,
            minimum_separation,
        }
    }

    /// Returns predecessor.
    #[must_use]
    pub const fn predecessor(&self) -> &Id {
        &self.predecessor
    }

    /// Returns successor.
    #[must_use]
    pub const fn successor(&self) -> &Id {
        &self.successor
    }

    /// Returns minimum separation.
    #[must_use]
    pub const fn minimum_separation(&self) -> Duration {
        self.minimum_separation
    }

    /// Validates local dependency semantics.
    pub fn validate(&self) -> TimingResult<()>
    where
        Id: PartialEq,
    {
        if self.predecessor == self.successor
            && !self.minimum_separation.is_zero()
        {
            return Err(TimingError::InvalidConstraint {
                message:
                    "a self-dependency cannot require positive separation"
                        .to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Semantic synchronization scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynchronizationScope<Id> {
    /// Explicit identifiers participate in synchronization.
    Explicit(Vec<Id>),

    /// All resources in the enclosing region.
    Region,

    /// All resources in the enclosing program.
    Program,

    /// Extensible named synchronization scope.
    Named(String),
}

impl<Id> SynchronizationScope<Id> {
    /// Creates an explicit scope.
    #[must_use]
    pub fn explicit<I>(ids: I) -> Self
    where
        I: IntoIterator<Item = Id>,
    {
        Self::Explicit(ids.into_iter().collect())
    }

    /// Returns whether the scope is explicit.
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
    }

    /// Returns the explicit identifiers when applicable.
    #[must_use]
    pub fn identifiers(&self) -> Option<&[Id]> {
        match self {
            Self::Explicit(ids) => Some(ids.as_slice()),
            _ => None,
        }
    }

    /// Removes duplicate identifiers while preserving first occurrence order.
    ///
    /// This method is intentionally provided without imposing an `Ord`
    /// requirement on the identifier type.
    pub fn deduplicate(&mut self)
    where
        Id: PartialEq,
    {
        if let Self::Explicit(ids) = self {
            let mut index = 0;

            while index < ids.len() {
                let mut later = index + 1;

                while later < ids.len() {
                    if ids[later] == ids[index] {
                        ids.remove(later);
                    } else {
                        later += 1;
                    }
                }

                index += 1;
            }
        }
    }
}

/// Semantic synchronization point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynchronizationPoint<Id> {
    scope: SynchronizationScope<Id>,
    earliest: Option<TimePoint>,
    latest: Option<TimePoint>,
}

impl<Id> SynchronizationPoint<Id> {
    /// Creates an unconstrained synchronization point.
    #[must_use]
    pub const fn new(
        scope: SynchronizationScope<Id>,
    ) -> Self {
        Self {
            scope,
            earliest: None,
            latest: None,
        }
    }

    /// Creates a synchronization point at an exact time.
    #[must_use]
    pub const fn at(
        scope: SynchronizationScope<Id>,
        time: TimePoint,
    ) -> Self {
        Self {
            scope,
            earliest: Some(time),
            latest: Some(time),
        }
    }

    /// Creates a synchronization point within a window.
    pub fn within(
        scope: SynchronizationScope<Id>,
        earliest: Option<TimePoint>,
        latest: Option<TimePoint>,
    ) -> TimingResult<Self> {
        if let (Some(earliest), Some(latest)) = (earliest, latest) {
            if earliest > latest {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "synchronization earliest time exceeds latest time"
                            .to_owned(),
                });
            }
        }

        Ok(Self {
            scope,
            earliest,
            latest,
        })
    }

    /// Returns synchronization scope.
    #[must_use]
    pub const fn scope(&self) -> &SynchronizationScope<Id> {
        &self.scope
    }

    /// Returns earliest permitted time.
    #[must_use]
    pub const fn earliest(&self) -> Option<TimePoint> {
        self.earliest
    }

    /// Returns latest permitted time.
    #[must_use]
    pub const fn latest(&self) -> Option<TimePoint> {
        self.latest
    }

    /// Validates the synchronization point.
    pub fn validate(&self) -> TimingResult<()> {
        if let (Some(earliest), Some(latest)) =
            (self.earliest, self.latest)
        {
            if earliest > latest {
                return Err(TimingError::InvalidConstraint {
                    message:
                        "synchronization earliest time exceeds latest time"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Timing intent
// =============================================================================

/// High-level scheduling intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingIntent {
    /// Place as early as possible.
    AsEarlyAsPossible,

    /// Place as late as possible.
    AsLateAsPossible,

    /// Preserve requested absolute timing.
    Fixed,

    /// Preserve requested spacing.
    PreserveSpacing,

    /// Allow flexible placement.
    Flexible,
}

// =============================================================================
// Stretch
// =============================================================================

/// Symbolic unresolved non-negative timing quantity.
///
/// A stretch is semantic intent. It must be resolved by a downstream timing
/// resolver/scheduler before physical execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stretch {
    name: String,
    bounds: TimingBounds,
    resolved: Option<Duration>,
}

impl Stretch {
    /// Creates an unresolved stretch.
    pub fn new<S>(
        name: S,
        bounds: TimingBounds,
    ) -> TimingResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(TimingError::InvalidStretch {
                message: "stretch name cannot be empty".to_owned(),
            });
        }

        validate_bounds(bounds)?;

        Ok(Self {
            name,
            bounds,
            resolved: None,
        })
    }

    /// Returns stretch name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns bounds.
    #[must_use]
    pub const fn bounds(&self) -> TimingBounds {
        self.bounds
    }

    /// Returns whether resolved.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        self.resolved.is_some()
    }

    /// Returns resolved duration.
    #[must_use]
    pub const fn resolved(&self) -> Option<Duration> {
        self.resolved
    }

    /// Resolves the stretch.
    pub fn resolve(
        &mut self,
        duration: Duration,
    ) -> TimingResult<()> {
        if !self.bounds.contains(duration) {
            return Err(TimingError::InvalidStretch {
                message: format!(
                    "duration {duration} violates stretch bounds"
                ),
            });
        }

        self.resolved = Some(duration);
        Ok(())
    }

    /// Clears the concrete resolution.
    pub fn clear_resolution(&mut self) {
        self.resolved = None;
    }
}

// =============================================================================
// Scheduled time
// =============================================================================

/// A semantic time which may remain unresolved until scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScheduledTime {
    /// Concrete semantic time.
    Resolved(TimePoint),

    /// No concrete placement has been selected.
    Unresolved,
}

impl ScheduledTime {
    /// Creates a resolved time.
    #[must_use]
    pub const fn resolved(time: TimePoint) -> Self {
        Self::Resolved(time)
    }

    /// Creates an unresolved time.
    #[must_use]
    pub const fn unresolved() -> Self {
        Self::Unresolved
    }

    /// Returns the concrete point.
    #[must_use]
    pub const fn as_time_point(self) -> Option<TimePoint> {
        match self {
            Self::Resolved(time) => Some(time),
            Self::Unresolved => None,
        }
    }

    /// Returns whether resolved.
    #[must_use]
    pub const fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved(_))
    }
}

// =============================================================================
// Delay
// =============================================================================

/// Hardware-independent delay specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelaySpec {
    /// Concrete fixed duration.
    Fixed(Duration),

    /// Duration constrained by bounds.
    Bounded(TimingBounds),

    /// Symbolically unresolved stretch.
    Stretch(Stretch),
}

impl DelaySpec {
    /// Creates a fixed delay.
    #[must_use]
    pub const fn fixed(duration: Duration) -> Self {
        Self::Fixed(duration)
    }

    /// Creates a bounded delay.
    #[must_use]
    pub const fn bounded(bounds: TimingBounds) -> Self {
        Self::Bounded(bounds)
    }

    /// Creates a stretch delay.
    #[must_use]
    pub fn stretch(stretch: Stretch) -> Self {
        Self::Stretch(stretch)
    }

    /// Returns a concrete duration if unambiguous.
    #[must_use]
    pub fn resolved_duration(&self) -> Option<Duration> {
        match self {
            Self::Fixed(duration) => Some(*duration),

            Self::Bounded(bounds) => bounds.exact_value(),

            Self::Stretch(stretch) => stretch.resolved(),
        }
    }

    /// Returns whether the delay is concrete.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.resolved_duration().is_some()
    }

    /// Validates local delay semantics.
    pub fn validate(&self) -> TimingResult<()> {
        match self {
            Self::Fixed(_) => Ok(()),

            Self::Bounded(bounds) => validate_bounds(*bounds),

            Self::Stretch(stretch) => {
                validate_bounds(stretch.bounds)
            }
        }
    }
}

// =============================================================================
// Timing domain
// =============================================================================

/// Semantic timing domain.
///
/// Nested domains allow a pulse calibration, subroutine, block, or other
/// construct to have a local temporal origin while remaining mappable to a
/// parent domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingDomain {
    origin: TimePoint,
}

impl TimingDomain {
    /// Creates the root timing domain.
    #[must_use]
    pub const fn root() -> Self {
        Self {
            origin: TimePoint::ZERO,
        }
    }

    /// Creates a domain at an enclosing-domain origin.
    #[must_use]
    pub const fn at(origin: TimePoint) -> Self {
        Self { origin }
    }

    /// Returns domain origin.
    #[must_use]
    pub const fn origin(self) -> TimePoint {
        self.origin
    }

    /// Converts local time to enclosing/global semantic time.
    pub fn to_global(
        self,
        local: TimePoint,
    ) -> TimingResult<TimePoint> {
        self.origin.checked_add(local.as_duration())
    }

    /// Converts enclosing/global time to local time.
    pub fn to_local(
        self,
        global: TimePoint,
    ) -> TimingResult<TimePoint> {
        global.checked_sub(self.origin.as_duration())
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a duration.
///
/// `Duration` itself is structurally valid for every `u128` value, so this
/// function exists as the stable validation boundary for higher-level modules.
pub const fn validate_duration(
    _duration: Duration,
) -> TimingResult<()> {
    Ok(())
}

/// Validates an interval.
pub const fn validate_interval(
    interval: TimeInterval,
) -> TimingResult<()> {
    if interval.start().attoseconds()
        > interval.end().attoseconds()
    {
        return Err(TimingError::InvalidInterval {
            start: interval.start().attoseconds(),
            end: interval.end().attoseconds(),
        });
    }

    Ok(())
}

/// Validates timing bounds.
pub const fn validate_bounds(
    bounds: TimingBounds,
) -> TimingResult<()> {
    match (bounds.minimum(), bounds.maximum()) {
        (Some(minimum), Some(maximum))
            if minimum > maximum =>
        {
            Err(TimingError::InvalidConstraint {
                message: "minimum duration exceeds maximum duration"
                    .to_owned(),
            })
        }

        _ => Ok(()),
    }
}

// =============================================================================
// Decimal parser
// =============================================================================

fn parse_decimal_duration(
    value: &str,
    unit: TimeUnit,
) -> TimingResult<Duration> {
    let value = value.trim();

    if value.is_empty() {
        return Err(TimingError::InvalidLiteral {
            literal: value.to_owned(),
        });
    }

    if let Some(stripped) = value.strip_prefix('+') {
        if stripped.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: value.to_owned(),
            });
        }

        return parse_decimal_duration(stripped, unit);
    }

    if value.starts_with('-') {
        return Err(TimingError::InvalidValue {
            message:
                "semantic Duration cannot be negative".to_owned(),
        });
    }

    let mut parts = value.split('.');

    let integer_part = parts.next().unwrap_or_default();
    let fractional_part = parts.next();

    if parts.next().is_some() {
        return Err(TimingError::InvalidLiteral {
            literal: value.to_owned(),
        });
    }

    if integer_part.is_empty() && fractional_part.is_none() {
        return Err(TimingError::InvalidLiteral {
            literal: value.to_owned(),
        });
    }

    if !integer_part.is_empty()
        && !integer_part
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return Err(TimingError::InvalidLiteral {
            literal: value.to_owned(),
        });
    }

    let fractional_part = fractional_part.unwrap_or("");

    if !fractional_part
        .chars()
        .all(|character| character.is_ascii_digit())
    {
        return Err(TimingError::InvalidLiteral {
            literal: value.to_owned(),
        });
    }

    if fractional_part.len() > MAX_DECIMAL_FRACTION_DIGITS {
        return Err(TimingError::ExcessivePrecision {
            digits: fractional_part.len(),
            maximum: MAX_DECIMAL_FRACTION_DIGITS,
        });
    }

    let integer_value = if integer_part.is_empty() {
        0
    } else {
        integer_part
            .parse::<u128>()
            .map_err(|_| TimingError::ArithmeticOverflow)?
    };

    let scale = unit.attoseconds();

    let integer_attoseconds = integer_value
        .checked_mul(scale)
        .ok_or(TimingError::ArithmeticOverflow)?;

    if fractional_part.is_empty() {
        return Ok(Duration::from_attoseconds(
            integer_attoseconds,
        ));
    }

    let fractional_integer = fractional_part
        .parse::<u128>()
        .map_err(|_| TimingError::ArithmeticOverflow)?;

    let power = pow10(fractional_part.len())?;

    let numerator = fractional_integer
        .checked_mul(scale)
        .ok_or(TimingError::ArithmeticOverflow)?;

    if numerator % power != 0 {
        return Err(TimingError::InexactBackendConversion {
            attoseconds: numerator,
        });
    }

    let fractional_attoseconds = numerator / power;

    let total = integer_attoseconds
        .checked_add(fractional_attoseconds)
        .ok_or(TimingError::ArithmeticOverflow)?;

    Ok(Duration::from_attoseconds(total))
}

fn pow10(digits: usize) -> TimingResult<u128> {
    let mut value = 1u128;
    let mut index = 0usize;

    while index < digits {
        value = value
            .checked_mul(10)
            .ok_or(TimingError::ArithmeticOverflow)?;

        index += 1;
    }

    Ok(value)
}

// =============================================================================
// Internal offset arithmetic
// =============================================================================

fn subtract_offsets(
    left: u128,
    right: u128,
) -> TimingResult<TimeOffset> {
    if left >= right {
        Ok(TimeOffset::from_parts(
            false,
            left - right,
        ))
    } else {
        Ok(TimeOffset::from_parts(
            true,
            right - left,
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_are_exact() {
        assert_eq!(
            TimeUnit::Attosecond.attoseconds(),
            1
        );

        assert_eq!(
            TimeUnit::Femtosecond.attoseconds(),
            1_000
        );

        assert_eq!(
            TimeUnit::Picosecond.attoseconds(),
            1_000_000
        );

        assert_eq!(
            TimeUnit::Nanosecond.attoseconds(),
            1_000_000_000
        );

        assert_eq!(
            TimeUnit::Microsecond.attoseconds(),
            1_000_000_000_000
        );

        assert_eq!(
            TimeUnit::Millisecond.attoseconds(),
            1_000_000_000_000_000
        );

        assert_eq!(
            TimeUnit::Second.attoseconds(),
            1_000_000_000_000_000_000
        );
    }

    #[test]
    fn parses_integer_duration() {
        let duration = Duration::parse("20ns").unwrap();

        assert_eq!(
            duration.attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn parses_whitespace_duration() {
        let duration = Duration::parse("20 ns").unwrap();

        assert_eq!(
            duration,
            Duration::nanoseconds(20).unwrap()
        );
    }

    #[test]
    fn parses_fractional_duration() {
        let duration = Duration::parse("1.5ns").unwrap();

        assert_eq!(
            duration.attoseconds(),
            1_500_000_000
        );
    }

    #[test]
    fn parses_microsecond_aliases() {
        assert_eq!(
            TimeUnit::parse("us").unwrap(),
            TimeUnit::Microsecond
        );

        assert_eq!(
            TimeUnit::parse("µs").unwrap(),
            TimeUnit::Microsecond
        );

        assert_eq!(
            TimeUnit::parse("μs").unwrap(),
            TimeUnit::Microsecond
        );
    }

    #[test]
    fn parses_positive_sign() {
        assert_eq!(
            Duration::parse("+20ns").unwrap(),
            Duration::nanoseconds(20).unwrap()
        );
    }

    #[test]
    fn rejects_negative_duration() {
        assert!(Duration::parse("-1ns").is_err());
    }

    #[test]
    fn rejects_unknown_unit() {
        assert!(Duration::parse("10dt").is_err());
    }

    #[test]
    fn rejects_malformed_literals() {
        assert!(Duration::parse("ns").is_err());
        assert!(Duration::parse("10").is_err());
        assert!(Duration::parse("1.2.3ns").is_err());
        assert!(Duration::parse("1xns").is_err());
    }

    #[test]
    fn rejects_inexact_decimal_precision() {
        // One attosecond is 1e-18 seconds. Therefore this is not exactly
        // representable in nanoseconds.
        assert!(
            Duration::parse("0.000000000000000001ns")
                .is_err()
        );
    }

    #[test]
    fn checked_duration_arithmetic() {
        let ten = Duration::nanoseconds(10).unwrap();
        let five = Duration::nanoseconds(5).unwrap();

        assert_eq!(
            ten.checked_add(five).unwrap(),
            Duration::nanoseconds(15).unwrap()
        );

        assert_eq!(
            ten.checked_sub(five).unwrap(),
            Duration::nanoseconds(5).unwrap()
        );

        assert!(five.checked_sub(ten).is_err());
    }

    #[test]
    fn duration_overflow_is_rejected() {
        assert!(
            Duration::from_units(
                u128::MAX,
                TimeUnit::Second
            )
            .is_err()
        );
    }

    #[test]
    fn time_point_arithmetic() {
        let start = TimePoint::ZERO;
        let duration = Duration::nanoseconds(20).unwrap();

        let end = start.checked_add(duration).unwrap();

        assert_eq!(
            end.attoseconds(),
            20_000_000_000
        );

        assert_eq!(
            end.offset_from(start),
            TimeOffset::positive(duration)
        );
    }

    #[test]
    fn offset_domain_exceeds_i128_without_failure() {
        let point = TimePoint::MAX;
        let offset = point.offset_from(TimePoint::ZERO);

        assert!(!offset.is_negative());
        assert_eq!(
            offset.magnitude(),
            u128::MAX
        );

        assert!(offset.to_i128().is_err());
    }

    #[test]
    fn negative_offset_applies() {
        let point = TimePoint::from_duration(
            Duration::nanoseconds(20).unwrap(),
        );

        let offset = TimeOffset::negative(
            Duration::nanoseconds(5).unwrap(),
        );

        let result = point.checked_offset(offset).unwrap();

        assert_eq!(
            result,
            TimePoint::from_duration(
                Duration::nanoseconds(15).unwrap()
            )
        );
    }

    #[test]
    fn negative_offset_underflow_is_rejected() {
        let offset = TimeOffset::negative(
            Duration::nanoseconds(1).unwrap(),
        );

        assert!(
            TimePoint::ZERO
                .checked_offset(offset)
                .is_err()
        );
    }

    #[test]
    fn offset_addition_is_checked() {
        let first = TimeOffset::from_attoseconds(u128::MAX);
        let second = TimeOffset::from_attoseconds(1);

        assert!(first.checked_add(second).is_err());
    }

    #[test]
    fn interval_is_half_open() {
        let start = TimePoint::ZERO;

        let end = TimePoint::from_duration(
            Duration::nanoseconds(20).unwrap(),
        );

        let interval =
            TimeInterval::new(start, end).unwrap();

        assert!(interval.contains(start));
        assert!(!interval.contains(end));
    }

    #[test]
    fn intervals_overlap_correctly() {
        let a = TimeInterval::new(
            TimePoint::ZERO,
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            ),
        )
        .unwrap();

        let b = TimeInterval::new(
            TimePoint::from_duration(
                Duration::nanoseconds(10).unwrap(),
            ),
            TimePoint::from_duration(
                Duration::nanoseconds(30).unwrap(),
            ),
        )
        .unwrap();

        assert!(a.overlaps(b));
    }

    #[test]
    fn adjacent_intervals_touch_without_overlap() {
        let boundary =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let a = TimeInterval::new(
            TimePoint::ZERO,
            boundary,
        )
        .unwrap();

        let b = TimeInterval::new(
            boundary,
            TimePoint::from_duration(
                Duration::nanoseconds(40).unwrap(),
            ),
        )
        .unwrap();

        assert!(!a.overlaps(b));
        assert!(a.touches(b));
    }

    #[test]
    fn interval_union_works() {
        let boundary =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let a = TimeInterval::new(
            TimePoint::ZERO,
            boundary,
        )
        .unwrap();

        let b = TimeInterval::new(
            boundary,
            TimePoint::from_duration(
                Duration::nanoseconds(40).unwrap(),
            ),
        )
        .unwrap();

        let union = a.union(b).unwrap();

        assert_eq!(
            union.start(),
            TimePoint::ZERO
        );

        assert_eq!(
            union.end(),
            TimePoint::from_duration(
                Duration::nanoseconds(40).unwrap()
            )
        );
    }

    #[test]
    fn backend_tick_conversion_is_exact() {
        let dt =
            BackendTimeUnit::from_attoseconds_per_tick(
                ATTOSECONDS_PER_NANOSECOND,
            )
            .unwrap();

        let duration =
            Duration::nanoseconds(20).unwrap();

        assert_eq!(
            dt.duration_to_ticks(duration).unwrap(),
            20
        );

        assert_eq!(
            dt.ticks_to_duration(20).unwrap(),
            duration
        );
    }

    #[test]
    fn backend_inexact_conversion_is_rejected() {
        let dt =
            BackendTimeUnit::from_attoseconds_per_tick(
                2_000_000_000,
            )
            .unwrap();

        let duration =
            Duration::nanoseconds(1).unwrap();

        assert!(
            dt.duration_to_ticks(duration).is_err()
        );
    }

    #[test]
    fn backend_overflow_is_rejected() {
        let dt =
            BackendTimeUnit::from_attoseconds_per_tick(
                u128::MAX,
            )
            .unwrap();

        assert!(
            dt.ticks_to_duration(2).is_err()
        );
    }

    #[test]
    fn bounds_work() {
        let minimum =
            Duration::nanoseconds(10).unwrap();

        let maximum =
            Duration::nanoseconds(20).unwrap();

        let bounds =
            TimingBounds::new(
                Some(minimum),
                Some(maximum),
            )
            .unwrap();

        assert!(
            bounds.contains(
                Duration::nanoseconds(15).unwrap()
            )
        );

        assert!(
            !bounds.contains(
                Duration::nanoseconds(25).unwrap()
            )
        );
    }

    #[test]
    fn exact_bounds_are_detected() {
        let duration =
            Duration::nanoseconds(20).unwrap();

        let bounds = TimingBounds::exact(duration);

        assert!(bounds.is_exact());
        assert_eq!(
            bounds.exact_value(),
            Some(duration)
        );
    }

    #[test]
    fn invalid_bounds_are_rejected() {
        let minimum =
            Duration::nanoseconds(30).unwrap();

        let maximum =
            Duration::nanoseconds(20).unwrap();

        assert!(
            TimingBounds::new(
                Some(minimum),
                Some(maximum)
            )
            .is_err()
        );
    }

    #[test]
    fn self_constraint_validation_is_conservative() {
        let constraint = TimingConstraint::new(
            1u64,
            TemporalRelation::Simultaneous,
            1u64,
        );

        assert!(constraint.validate().is_ok());

        let invalid = TimingConstraint::new(
            1u64,
            TemporalRelation::Before,
            1u64,
        );

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn temporal_dependency_validates() {
        let dependency =
            TemporalDependency::with_minimum_separation(
                1u64,
                2u64,
                Duration::nanoseconds(5).unwrap(),
            );

        assert!(dependency.validate().is_ok());
    }

    #[test]
    fn synchronization_window_validates() {
        let earliest =
            TimePoint::from_duration(
                Duration::nanoseconds(10).unwrap(),
            );

        let latest =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let point =
            SynchronizationPoint::<u64>::within(
                SynchronizationScope::explicit(
                    [1u64, 2u64],
                ),
                Some(earliest),
                Some(latest),
            )
            .unwrap();

        assert!(point.validate().is_ok());
    }

    #[test]
    fn invalid_synchronization_window_is_rejected() {
        let earliest =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let latest =
            TimePoint::from_duration(
                Duration::nanoseconds(10).unwrap(),
            );

        assert!(
            SynchronizationPoint::<u64>::within(
                SynchronizationScope::explicit(
                    [1u64, 2u64],
                ),
                Some(earliest),
                Some(latest),
            )
            .is_err()
        );
    }

    #[test]
    fn stretch_resolution_respects_bounds() {
        let bounds = TimingBounds::new(
            Some(Duration::nanoseconds(10).unwrap()),
            Some(Duration::nanoseconds(30).unwrap()),
        )
        .unwrap();

        let mut stretch =
            Stretch::new("echo_gap", bounds)
                .unwrap();

        assert!(!stretch.is_resolved());

        stretch
            .resolve(
                Duration::nanoseconds(20).unwrap()
            )
            .unwrap();

        assert_eq!(
            stretch.resolved(),
            Some(
                Duration::nanoseconds(20).unwrap()
            )
        );
    }

    #[test]
    fn stretch_rejects_out_of_bounds_resolution() {
        let bounds =
            TimingBounds::exact(
                Duration::nanoseconds(20).unwrap()
            );

        let mut stretch =
            Stretch::new("gap", bounds)
                .unwrap();

        assert!(
            stretch
                .resolve(
                    Duration::nanoseconds(10).unwrap()
                )
                .is_err()
        );
    }

    #[test]
    fn delay_resolution_works() {
        let fixed = DelaySpec::fixed(
            Duration::nanoseconds(20).unwrap(),
        );

        assert!(fixed.is_resolved());

        let bounded = DelaySpec::bounded(
            TimingBounds::exact(
                Duration::nanoseconds(20).unwrap(),
            ),
        );

        assert!(bounded.is_resolved());
    }

    #[test]
    fn timing_domain_round_trip() {
        let origin =
            TimePoint::from_duration(
                Duration::nanoseconds(100).unwrap(),
            );

        let domain = TimingDomain::at(origin);

        let local =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let global =
            domain.to_global(local).unwrap();

        assert_eq!(
            global,
            TimePoint::from_duration(
                Duration::nanoseconds(120).unwrap(),
            )
        );

        assert_eq!(
            domain.to_local(global).unwrap(),
            local
        );
    }

    #[test]
    fn canonical_string_is_deterministic() {
        assert_eq!(
            Duration::nanoseconds(20)
                .unwrap()
                .canonical_string(),
            "20ns"
        );

        assert_eq!(
            Duration::microseconds(2)
                .unwrap()
                .canonical_string(),
            "2us"
        );

        assert_eq!(
            Duration::from_attoseconds(1)
                .canonical_string(),
            "1as"
        );
    }

    #[test]
    fn scheduled_time_distinguishes_unresolved() {
        let unresolved =
            ScheduledTime::unresolved();

        let resolved =
            ScheduledTime::resolved(
                TimePoint::ZERO
            );

        assert!(!unresolved.is_resolved());
        assert!(resolved.is_resolved());
    }

    #[test]
    fn no_unsafe_contract_is_present() {
        // This test intentionally has no runtime assertion.
        // `#![forbid(unsafe_code)]` is the compile-time guarantee.
        assert_eq!(2 + 2, 4);
    }
}