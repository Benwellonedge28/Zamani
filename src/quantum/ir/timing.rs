//! Zamani Quantum IR — Canonical Timing Model
//!
//! Hardware-independent, deterministic and resource-safe timing semantics for
//! the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::timing` defines WHAT TIME MEANS in the canonical Quantum IR.
//!
//! It owns:
//!
//! - canonical durations;
//! - absolute time points;
//! - signed temporal offsets;
//! - exact time units;
//! - exact parsing of timing literals;
//! - checked timing arithmetic;
//! - temporal intervals;
//! - timing bounds;
//! - timing constraints;
//! - temporal dependencies;
//! - synchronization semantics;
//! - symbolic/stretched timing intent;
//! - conversion boundaries for backend-dependent timing;
//! - deterministic formatting;
//! - timing-specific validation.
//!
//! It does NOT own:
//!
//! - hardware clocks;
//! - DAC/ADC sample rates;
//! - backend `dt` definitions;
//! - physical pulse calibration;
//! - waveform generation;
//! - scheduling algorithms;
//! - routing;
//! - optimization policy;
//! - QPU execution;
//! - simulator time evolution.
//!
//! Those belong to downstream layers.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and can subsequently target:
//!
//! - a one-qubit machine;
//! - a small QPU;
//! - a large QPU;
//! - a distributed quantum system;
//! - a fault-tolerant logical machine;
//! - a simulator;
//! - a pulse-controlled processor;
//! - an analog processor;
//! - an annealing system.
//!
//! Timing therefore has NO architectural machine-size limit.
//!
//! A timing value is independent of the number of qubits.
//!
//! # Canonical internal unit
//!
//! The canonical exact unit is the attosecond (`10^-18` seconds).
//!
//! This provides:
//!
//! - exact integer representation;
//! - deterministic arithmetic;
//! - no floating-point timing drift;
//! - nanosecond pulse support;
//! - femtosecond and attosecond precision;
//! - sufficiently large representable finite time ranges.
//!
//! The canonical representation is:
//!
//! ```text
//! 1 second = 1_000_000_000_000_000_000 attoseconds
//! ```
//!
//! `u128` is used intentionally.
//!
//! This is not an architectural claim that a physical machine operates at
//! attosecond resolution. Hardware-specific resolution is established later
//! by the hardware/scheduling layers.
//!
//! # Important distinction
//!
//! ```text
//! Duration
//!     = amount of elapsed time
//!
//! TimePoint
//!     = absolute semantic time coordinate
//!
//! TimeOffset
//!     = signed relative displacement
//!
//! TimeInterval
//!     = [start, end) semantic interval
//!
//! TimingConstraint
//!     = relationship that must hold
//!
//! TemporalDependency
//!     = ordering relationship between events
//!
//! SynchronizationPoint
//!     = semantic execution rendezvous
//!
//! Stretch
//!     = unresolved timing intent
//!
//! Backend timing
//!     = target-specific realization
//! ```
//!
//! # Hardware boundary
//!
//! This module intentionally does not know about:
//!
//! - physical qubit topology;
//! - hardware channels;
//! - sample clocks;
//! - calibration;
//! - device latency;
//! - gate duration calibration;
//! - waveform sample rates;
//! - QPU scheduling.
//!
//! For example, `20ns` is a valid semantic duration.
//!
//! Whether a target can realize exactly `20ns` is a downstream hardware
//! compatibility question.
//!
//! # Pulse-level example
//!
//! A Zamani source operation such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! ultimately uses:
//!
//! ```text
//! Duration::nanoseconds(20)
//! ```
//!
//! The timing module does not decide which physical channel or waveform is
//! used.
//!
//! # OpenQASM compatibility
//!
//! The model deliberately supports concepts needed by modern quantum timing
//! systems:
//!
//! - fixed durations;
//! - delays;
//! - relative timing;
//! - absolute timing;
//! - timing constraints;
//! - synchronization;
//! - unresolved/stretch timing intent;
//! - backend-dependent resolution.
//!
//! Backend-specific `dt` units are represented separately by
//! `BackendTimeUnit` and must never silently become a universal physical unit.
//!
//! # Scalability and safety
//!
//! All arithmetic is checked.
//!
//! No arithmetic operation intentionally wraps.
//!
//! No collection is allocated merely to represent a time range.
//!
//! No recursive timing traversal is required by this module.
//!
//! There is no hidden `63`, `4096`, or other machine-size boundary.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Stable Rust only.
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `parameter.rs` may provide symbolic values to higher-level timing
//! expressions, but concrete resolved timing is owned here.
//!
//! `pulse.rs` consumes `Duration`, `TimePoint`, `TimeOffset`, and
//! `TimingConstraint`.
//!
//! `schedule.rs` consumes `TimeInterval`, `ScheduledTime`, and
//! `TemporalDependency`.
//!
//! `operation.rs` consumes timing requirements without knowing hardware.
//!
//! `validation.rs` validates timing invariants.
//!
//! `analysis.rs` uses checked timing arithmetic for critical-path analysis.
//!
//! `limits.rs` may consume `Duration::attoseconds()` for schedule-time policy
//! accounting.
//!
//! `hardware/timing.rs` resolves these semantic values against actual device
//! timing.
//!
//! `hardware/pulse/*` resolves pulse durations against actual sample clocks.
//!
//! `scheduling/` computes actual placement.
//!
//! `routing/` remains completely independent of this module.
//!
//! `optimization/` may transform timing constraints but does not own the
//! canonical time representation.
//!
//! `qubit.rs` remains independent: timing does not need to import qubit
//! identities because timing semantics are reusable for qubit, classical,
//! pulse, analog and distributed operations.
//!
//! This module therefore intentionally does NOT import
//! `quantum::ir::qubit`.
//!
//! That is the correct dependency boundary: use the qubit module where a
//! timing-aware operation actually owns qubit operands, not inside the generic
//! time primitive itself.
//! ```

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

// =============================================================================
// Canonical constants
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

/// Maximum number of decimal fractional digits representable exactly by the
/// canonical attosecond representation.
pub const MAX_DECIMAL_FRACTION_DIGITS: usize = 18;

// =============================================================================
// Timing errors
// =============================================================================

/// Errors produced by the canonical timing model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingError {
    /// A numeric timing value is invalid.
    InvalidValue {
        /// Human-readable reason.
        message: String,
    },

    /// A timing literal has an unknown unit.
    UnknownUnit {
        /// Unit text supplied by the caller.
        unit: String,
    },

    /// A timing literal is malformed.
    InvalidLiteral {
        /// Original literal.
        literal: String,
    },

    /// Decimal precision exceeds exact attosecond representation.
    ExcessivePrecision {
        /// Number of supplied fractional digits.
        digits: usize,

        /// Maximum exact number of digits.
        maximum: usize,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow,

    /// A subtraction would produce a negative unsigned duration.
    NegativeDuration,

    /// A division operation has a zero divisor.
    DivisionByZero,

    /// An interval has invalid bounds.
    InvalidInterval {
        /// Start time.
        start: u128,

        /// End time.
        end: u128,
    },

    /// A constraint is internally inconsistent.
    InvalidConstraint {
        /// Human-readable reason.
        message: String,
    },

    /// A backend timing conversion cannot be represented exactly.
    InexactBackendConversion {
        /// Canonical attoseconds.
        attoseconds: u128,
    },

    /// A backend-dependent unit has an invalid scale.
    InvalidBackendScale,

    /// A stretch/timing variable has an invalid value.
    InvalidStretch {
        /// Human-readable reason.
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
                    "invalid timing interval: start {start} is greater \
                     than end {end}"
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

/// Canonical result type for timing operations.
pub type TimingResult<T> = Result<T, TimingError>;

// =============================================================================
// Time unit
// =============================================================================

/// Exact semantic SI time unit supported by the canonical IR.
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
    /// Returns the exact number of attoseconds in one unit.
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

    /// Returns the canonical short source spelling.
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

    /// Parses a timing-unit spelling.
    ///
    /// Accepted aliases:
    ///
    /// - `as`
    /// - `fs`
    /// - `ps`
    /// - `ns`
    /// - `us`
    /// - `µs`
    /// - `μs`
    /// - `ms`
    /// - `s`
    pub fn parse(text: &str) -> TimingResult<Self> {
        match text {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// =============================================================================
// Duration
// =============================================================================

/// Exact non-negative semantic duration.
///
/// Internally stored in attoseconds.
///
/// `Duration` is deliberately independent of `std::time::Duration` because
/// quantum IR timing is a semantic representation and must not inherit host
/// operating-system timing semantics.
///
/// This type can represent durations much larger than ordinary hardware
/// execution windows without changing the IR architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Duration {
    attoseconds: u128,
}

impl Duration {
    /// Zero duration.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Maximum representable finite duration.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    /// Creates a duration directly from canonical attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Returns the canonical attosecond representation.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Creates an exact duration from an integer count of a unit.
    pub fn from_units(value: u128, unit: TimeUnit) -> TimingResult<Self> {
        let scale = unit.attoseconds();

        let attoseconds = value
            .checked_mul(scale)
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

    /// Creates a duration from an exact decimal quantity and unit.
    ///
    /// Examples:
    ///
    /// ```text
    /// 20ns
    /// 20 ns
    /// 1.5ns
    /// 0.25us
    /// ```
    ///
    /// More than 18 fractional decimal digits are rejected because the
    /// canonical representation has exact attosecond precision.
    pub fn from_decimal(
        value: &str,
        unit: TimeUnit,
    ) -> TimingResult<Self> {
        parse_decimal_duration(value, unit)
    }

    /// Parses a complete timing literal.
    pub fn parse(literal: &str) -> TimingResult<Self> {
        let trimmed = literal.trim();

        if trimmed.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            });
        }

        let split_index = trimmed
            .find(|character: char| {
                character.is_ascii_alphabetic()
                    || character == 'µ'
                    || character == 'μ'
            })
            .ok_or_else(|| TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            })?;

        let number = trimmed[..split_index].trim();
        let unit = trimmed[split_index..].trim();

        if number.is_empty() || unit.is_empty() {
            return Err(TimingError::InvalidLiteral {
                literal: literal.to_owned(),
            });
        }

        let parsed_unit = TimeUnit::parse(unit)?;

        Self::from_decimal(number, parsed_unit)
    }

    /// Returns whether the duration is exactly zero.
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
    pub fn checked_div(self, divisor: u128) -> TimingResult<Self> {
        if divisor == 0 {
            return Err(TimingError::DivisionByZero);
        }

        Ok(Self::from_attoseconds(
            self.attoseconds / divisor,
        ))
    }

    /// Returns the exact integer number of units when representable.
    ///
    /// Returns an error if the duration is not an exact multiple of the unit.
    pub fn to_units_exact(self, unit: TimeUnit) -> TimingResult<u128> {
        let scale = unit.attoseconds();

        if self.attoseconds % scale != 0 {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: self.attoseconds,
            });
        }

        Ok(self.attoseconds / scale)
    }

    /// Returns the whole-number component in the requested unit.
    #[must_use]
    pub const fn whole_units(self, unit: TimeUnit) -> u128 {
        self.attoseconds / unit.attoseconds()
    }

    /// Returns the remainder after extracting whole units.
    #[must_use]
    pub const fn remainder_attoseconds(self, unit: TimeUnit) -> u128 {
        self.attoseconds % unit.attoseconds()
    }

    /// Returns the duration as a canonical decimal string.
    ///
    /// The output uses seconds only when the value is an exact number of
    /// seconds; otherwise it chooses the largest standard unit that preserves
    /// an integer representation.
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

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.attoseconds.cmp(&other.attoseconds)
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

impl FromStr for Duration {
    type Err = TimingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// =============================================================================
// Signed time offset
// =============================================================================

/// Signed relative timing displacement.
///
/// This is distinct from `Duration` because timing constraints may need to
/// express relationships such as:
///
/// ```text
/// event_b = event_a - 5ns
/// ```
///
/// A physical operation duration itself must remain non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeOffset {
    attoseconds: i128,
}

impl TimeOffset {
    /// Zero offset.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Creates an offset directly from attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: i128) -> Self {
        Self { attoseconds }
    }

    /// Returns the canonical attosecond value.
    #[must_use]
    pub const fn attoseconds(self) -> i128 {
        self.attoseconds
    }

    /// Creates a positive offset from a duration.
    pub fn positive(duration: Duration) -> TimingResult<Self> {
        let value = i128::try_from(duration.attoseconds())
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds: value })
    }

    /// Creates a negative offset from a duration.
    pub fn negative(duration: Duration) -> TimingResult<Self> {
        let value = i128::try_from(duration.attoseconds())
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let value = value
            .checked_neg()
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds: value })
    }

    /// Returns whether this offset is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.attoseconds < 0
    }

    /// Returns whether this offset is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.attoseconds == 0
    }

    /// Returns the absolute magnitude as a duration.
    pub fn magnitude(self) -> TimingResult<Duration> {
        let value = self
            .attoseconds
            .checked_abs()
            .ok_or(TimingError::ArithmeticOverflow)?;

        let value = u128::try_from(value)
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        Ok(Duration::from_attoseconds(value))
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_add(other.attoseconds)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds: value })
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> TimingResult<Self> {
        let value = self
            .attoseconds
            .checked_sub(other.attoseconds)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self { attoseconds: value })
    }
}

impl Ord for TimeOffset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.attoseconds.cmp(&other.attoseconds)
    }
}

impl PartialOrd for TimeOffset {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TimeOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.attoseconds < 0 {
            let magnitude = self
                .attoseconds
                .checked_abs()
                .unwrap_or(i128::MAX);

            write!(f, "-{}as", magnitude)
        } else {
            write!(f, "{}as", self.attoseconds)
        }
    }
}

// =============================================================================
// Time point
// =============================================================================

/// Absolute semantic time coordinate.
///
/// A `TimePoint` is always non-negative.
///
/// It is not wall-clock time and has no relationship to `SystemTime`.
/// It represents time relative to the semantic beginning of the enclosing
/// program, schedule, region, calibration or timing domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimePoint {
    attoseconds: u128,
}

impl TimePoint {
    /// Semantic time zero.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Maximum representable finite time point.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    /// Creates a time point from canonical attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Returns canonical attoseconds.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Creates a time point from a duration measured from semantic time zero.
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Self {
        Self::from_attoseconds(duration.attoseconds())
    }

    /// Returns the elapsed duration from time zero.
    #[must_use]
    pub const fn as_duration(self) -> Duration {
        Duration::from_attoseconds(self.attoseconds)
    }

    /// Advances the time point by a non-negative duration.
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

    /// Moves the time point backwards by a non-negative duration.
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

    /// Applies a signed offset.
    pub fn checked_offset(
        self,
        offset: TimeOffset,
    ) -> TimingResult<Self> {
        if offset.attoseconds() >= 0 {
            let magnitude = u128::try_from(offset.attoseconds())
                .map_err(|_| TimingError::ArithmeticOverflow)?;

            let value = self
                .attoseconds
                .checked_add(magnitude)
                .ok_or(TimingError::ArithmeticOverflow)?;

            Ok(Self::from_attoseconds(value))
        } else {
            let magnitude = offset
                .attoseconds()
                .checked_abs()
                .ok_or(TimingError::ArithmeticOverflow)?;

            let magnitude = u128::try_from(magnitude)
                .map_err(|_| TimingError::ArithmeticOverflow)?;

            let value = self
                .attoseconds
                .checked_sub(magnitude)
                .ok_or(TimingError::NegativeDuration)?;

            Ok(Self::from_attoseconds(value))
        }
    }

    /// Returns the signed offset from another time point.
    pub fn offset_from(
        self,
        other: Self,
    ) -> TimingResult<TimeOffset> {
        if self.attoseconds >= other.attoseconds {
            let difference = self.attoseconds - other.attoseconds;

            let difference = i128::try_from(difference)
                .map_err(|_| TimingError::ArithmeticOverflow)?;

            Ok(TimeOffset::from_attoseconds(difference))
        } else {
            let difference = other.attoseconds - self.attoseconds;

            let difference = i128::try_from(difference)
                .map_err(|_| TimingError::ArithmeticOverflow)?;

            let difference = difference
                .checked_neg()
                .ok_or(TimingError::ArithmeticOverflow)?;

            Ok(TimeOffset::from_attoseconds(difference))
        }
    }
}

impl Ord for TimePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.attoseconds.cmp(&other.attoseconds)
    }
}

impl PartialOrd for TimePoint {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
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

/// Half-open semantic time interval `[start, end)`.
///
/// An interval with equal start/end is valid and represents zero elapsed time.
///
/// Intervals never contain a negative duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeInterval {
    start: TimePoint,
    end: TimePoint,
}

impl TimeInterval {
    /// Creates an interval `[start, end)`.
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

    /// Creates an interval beginning at `start` with `duration`.
    pub fn from_start_duration(
        start: TimePoint,
        duration: Duration,
    ) -> TimingResult<Self> {
        let end = start.checked_add(duration)?;
        Self::new(start, end)
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the exclusive end time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns elapsed duration.
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

    /// Returns whether this interval is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.attoseconds() == self.end.attoseconds()
    }

    /// Returns whether two intervals overlap with positive duration.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns whether this interval touches another interval at one boundary.
    #[must_use]
    pub const fn touches(self, other: Self) -> bool {
        self.end == other.start || other.end == self.start
    }

    /// Returns whether this interval contains a time point.
    ///
    /// The end point is exclusive.
    #[must_use]
    pub const fn contains(self, point: TimePoint) -> bool {
        self.start <= point && point < self.end
    }
}

// =============================================================================
// Backend-dependent timing
// =============================================================================

/// Explicit backend-dependent timing unit.
///
/// This is intentionally NOT part of the universal semantic `TimeUnit`.
///
/// For example, a backend may define:
///
/// ```text
/// dt = 2.0 ns
/// ```
///
/// The actual mapping belongs to the target description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackendTimeUnit {
    /// Number of attoseconds represented by one backend tick.
    attoseconds_per_tick: u128,
}

impl BackendTimeUnit {
    /// Creates a backend unit from an exact integer attosecond scale.
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

    /// Returns the exact backend tick scale.
    #[must_use]
    pub const fn attoseconds_per_tick(self) -> u128 {
        self.attoseconds_per_tick
    }

    /// Converts a semantic duration to an exact integer backend tick count.
    pub fn duration_to_ticks(
        self,
        duration: Duration,
    ) -> TimingResult<u128> {
        duration.to_units_exact(
            TimeUnit::Attosecond,
        )?;

        if duration.attoseconds() % self.attoseconds_per_tick != 0 {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: duration.attoseconds(),
            });
        }

        Ok(duration.attoseconds() / self.attoseconds_per_tick)
    }

    /// Converts backend ticks into an exact semantic duration.
    pub fn ticks_to_duration(
        self,
        ticks: u128,
    ) -> TimingResult<Duration> {
        let attoseconds = ticks
            .checked_mul(self.attoseconds_per_tick)
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Duration::from_attoseconds(attoseconds))
    }
}

// =============================================================================
// Timing bounds
// =============================================================================

/// A lower/upper bound for a timing quantity.
///
/// `None` means unbounded.
///
/// This is used for unresolved timing intent and hardware-independent
/// constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingBounds {
    minimum: Option<Duration>,
    maximum: Option<Duration>,
}

impl TimingBounds {
    /// Creates unconstrained bounds.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            minimum: None,
            maximum: None,
        }
    }

    /// Creates exact bounds.
    #[must_use]
    pub const fn exact(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: Some(duration),
        }
    }

    /// Creates lower-only bounds.
    #[must_use]
    pub const fn at_least(duration: Duration) -> Self {
        Self {
            minimum: Some(duration),
            maximum: None,
        }
    }

    /// Creates upper-only bounds.
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
                    message: "minimum duration exceeds maximum duration"
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

    /// Returns whether a duration satisfies these bounds.
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
}

// =============================================================================
// Temporal relation
// =============================================================================

/// Semantic relationship between two time points or events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalRelation {
    /// First event must occur before the second.
    Before,

    /// First event must occur no later than the second.
    BeforeOrAt,

    /// Events must occur at the same time.
    Simultaneous,

    /// First event must occur after the second.
    After,

    /// First event must occur no earlier than the second.
    AfterOrAt,

    /// The second event must begin at least a specified duration after the
    /// first event.
    SeparatedByAtLeast(Duration),

    /// The second event must begin no more than a specified duration after the
    /// first event.
    SeparatedByAtMost(Duration),

    /// The difference between event times must equal a specified duration.
    SeparatedByExactly(Duration),
}

// =============================================================================
// Timing constraint
// =============================================================================

/// Hardware-independent timing constraint.
///
/// `TimingConstraint` contains semantic intent only. It does not identify
/// operations by raw pointers or references, allowing it to remain serializable
/// and independent of operation ownership.
///
/// Callers provide stable operation/event identifiers at higher IR layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingConstraint<Id> {
    first: Id,
    second: Id,
    relation: TemporalRelation,
}

impl<Id> TimingConstraint<Id> {
    /// Creates a timing constraint.
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

    /// Returns the first event identifier.
    #[must_use]
    pub const fn first(&self) -> &Id {
        &self.first
    }

    /// Returns the second event identifier.
    #[must_use]
    pub const fn second(&self) -> &Id {
        &self.second
    }

    /// Returns the temporal relation.
    #[must_use]
    pub const fn relation(&self) -> TemporalRelation {
        self.relation
    }
}

// =============================================================================
// Temporal dependency
// =============================================================================

/// A semantic ordering dependency between two events.
///
/// This is deliberately smaller than a scheduling edge. Scheduling algorithms
/// may add many more constraints later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemporalDependency<Id> {
    predecessor: Id,
    successor: Id,
    minimum_separation: Duration,
}

impl<Id> TemporalDependency<Id> {
    /// Creates a dependency with zero minimum separation.
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

    /// Creates a dependency with a minimum separation.
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

    /// Returns the predecessor.
    #[must_use]
    pub const fn predecessor(&self) -> &Id {
        &self.predecessor
    }

    /// Returns the successor.
    #[must_use]
    pub const fn successor(&self) -> &Id {
        &self.successor
    }

    /// Returns the minimum required separation.
    #[must_use]
    pub const fn minimum_separation(&self) -> Duration {
        self.minimum_separation
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Semantic synchronization scope.
///
/// A scheduler may later lower these scopes into concrete scheduling barriers,
/// channel barriers, qubit barriers, or distributed synchronization protocols.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynchronizationScope<Id> {
    /// Synchronize an explicitly named set of events/resources.
    Explicit(Vec<Id>),

    /// Synchronize all resources in the enclosing region.
    Region,

    /// Synchronize all resources participating in the enclosing program.
    Program,

    /// Backend-defined synchronization scope.
    Named(String),
}

impl<Id> SynchronizationScope<Id> {
    /// Creates an explicit synchronization scope.
    #[must_use]
    pub fn explicit<I>(ids: I) -> Self
    where
        I: IntoIterator<Item = Id>,
    {
        Self::Explicit(ids.into_iter().collect())
    }

    /// Returns whether this scope contains explicit identifiers.
    #[must_use]
    pub const fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
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

    /// Creates a synchronization point with an allowed time window.
    pub fn within(
        scope: SynchronizationScope<Id>,
        earliest: Option<TimePoint>,
        latest: Option<TimePoint>,
    ) -> TimingResult<Self> {
        if let (Some(earliest), Some(latest)) = (earliest, latest) {
            if earliest > latest {
                return Err(TimingError::InvalidConstraint {
                    message: "synchronization earliest time exceeds latest time"
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

    /// Returns the synchronization scope.
    #[must_use]
    pub const fn scope(&self) -> &SynchronizationScope<Id> {
        &self.scope
    }

    /// Returns earliest permitted synchronization time.
    #[must_use]
    pub const fn earliest(&self) -> Option<TimePoint> {
        self.earliest
    }

    /// Returns latest permitted synchronization time.
    #[must_use]
    pub const fn latest(&self) -> Option<TimePoint> {
        self.latest
    }
}

// =============================================================================
// Timing intent
// =============================================================================

/// High-level timing intent.
///
/// This captures design intent before a scheduler resolves concrete times.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingIntent {
    /// Place an operation as early as possible.
    AsEarlyAsPossible,

    /// Place an operation as late as possible while respecting constraints.
    AsLateAsPossible,

    /// Preserve the requested absolute timing.
    Fixed,

    /// Preserve a requested spacing between events.
    PreserveSpacing,

    /// Allow the scheduler to choose freely within constraints.
    Flexible,
}

// =============================================================================
// Stretch
// =============================================================================

/// Symbolic non-negative timing stretch.
///
/// A stretch represents unresolved timing intent. It is NOT a floating-point
/// value and cannot be directly executed until a downstream timing resolver
/// assigns a concrete duration.
///
/// This makes the type suitable for constructs inspired by high-level quantum
/// timing systems where timing relationships are resolved after compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stretch {
    name: String,
    bounds: TimingBounds,
    resolved: Option<Duration>,
}

impl Stretch {
    /// Creates an unresolved stretch.
    pub fn new<S: Into<String>>(
        name: S,
        bounds: TimingBounds,
    ) -> TimingResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(TimingError::InvalidStretch {
                message: "stretch name cannot be empty".to_owned(),
            });
        }

        Ok(Self {
            name,
            bounds,
            resolved: None,
        })
    }

    /// Returns the stretch name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the allowed bounds.
    #[must_use]
    pub const fn bounds(&self) -> TimingBounds {
        self.bounds
    }

    /// Returns whether the stretch has been resolved.
    #[must_use]
    pub const fn is_resolved(&self) -> bool {
        self.resolved.is_some()
    }

    /// Returns the resolved duration if available.
    #[must_use]
    pub const fn resolved(&self) -> Option<Duration> {
        self.resolved
    }

    /// Resolves the stretch to a concrete duration.
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

    /// Clears the resolution.
    pub fn clear_resolution(&mut self) {
        self.resolved = None;
    }
}

// =============================================================================
// Scheduled time
// =============================================================================

/// Timing representation used by a later scheduling layer.
///
/// A scheduled event may remain unresolved until scheduling is complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScheduledTime {
    /// Concrete semantic time.
    Resolved(TimePoint),

    /// Event has no concrete placement yet.
    Unresolved,
}

impl ScheduledTime {
    /// Creates a resolved scheduled time.
    #[must_use]
    pub const fn resolved(time: TimePoint) -> Self {
        Self::Resolved(time)
    }

    /// Creates an unresolved scheduled time.
    #[must_use]
    pub const fn unresolved() -> Self {
        Self::Unresolved
    }

    /// Returns the concrete time if resolved.
    #[must_use]
    pub const fn as_time_point(self) -> Option<TimePoint> {
        match self {
            Self::Resolved(time) => Some(time),
            Self::Unresolved => None,
        }
    }

    /// Returns whether the time has been resolved.
    #[must_use]
    pub const fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved(_))
    }
}

// =============================================================================
// Delay intent
// =============================================================================

/// Hardware-independent delay semantic.
///
/// The scheduler/hardware layers determine how a delay is realized.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelaySpec {
    /// Fixed concrete duration.
    Fixed(Duration),

    /// Duration constrained by bounds.
    Bounded(TimingBounds),

    /// Unresolved symbolic stretch.
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

    /// Creates a stretch-based delay.
    #[must_use]
    pub fn stretch(stretch: Stretch) -> Self {
        Self::Stretch(stretch)
    }

    /// Returns the fixed duration if already concrete.
    #[must_use]
    pub fn resolved_duration(&self) -> Option<Duration> {
        match self {
            Self::Fixed(duration) => Some(*duration),
            Self::Bounded(bounds) => match (
                bounds.minimum(),
                bounds.maximum(),
            ) {
                (Some(minimum), Some(maximum))
                    if minimum == maximum =>
                {
                    Some(minimum)
                }

                _ => None,
            },
            Self::Stretch(stretch) => stretch.resolved(),
        }
    }

    /// Returns whether this delay has been fully resolved.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.resolved_duration().is_some()
    }
}

// =============================================================================
// Timing domain
// =============================================================================

/// Semantic timing domain.
///
/// A program can contain nested domains, for example a pulse calibration block
/// or a logical subroutine. Each domain has its own semantic zero while still
/// being convertible to an enclosing domain through a scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingDomain {
    origin: TimePoint,
}

impl TimingDomain {
    /// Creates a domain whose semantic zero is at global time zero.
    #[must_use]
    pub const fn root() -> Self {
        Self {
            origin: TimePoint::ZERO,
        }
    }

    /// Creates a nested timing domain at the supplied parent time.
    #[must_use]
    pub const fn at(origin: TimePoint) -> Self {
        Self { origin }
    }

    /// Returns the domain origin.
    #[must_use]
    pub const fn origin(self) -> TimePoint {
        self.origin
    }

    /// Converts a local time into parent/global semantic time.
    pub fn to_global(
        self,
        local: TimePoint,
    ) -> TimingResult<TimePoint> {
        self.origin.checked_add(local.as_duration())
    }

    /// Converts a global time into this domain.
    pub fn to_local(
        self,
        global: TimePoint,
    ) -> TimingResult<TimePoint> {
        global.checked_sub(self.origin.as_duration())
    }
}

// =============================================================================
// Timing validation helpers
// =============================================================================

/// Validates a duration as a canonical semantic timing value.
pub fn validate_duration(
    duration: Duration,
) -> TimingResult<()> {
    let _ = duration;
    Ok(())
}

/// Validates an interval.
pub fn validate_interval(
    interval: TimeInterval,
) -> TimingResult<()> {
    if interval.start() > interval.end() {
        return Err(TimingError::InvalidInterval {
            start: interval.start().attoseconds(),
            end: interval.end().attoseconds(),
        });
    }

    Ok(())
}

/// Validates timing bounds.
pub fn validate_bounds(
    bounds: TimingBounds,
) -> TimingResult<()> {
    if let (Some(minimum), Some(maximum)) =
        (bounds.minimum(), bounds.maximum())
    {
        if minimum > maximum {
            return Err(TimingError::InvalidConstraint {
                message: "minimum exceeds maximum".to_owned(),
            });
        }
    }

    Ok(())
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

    if value.starts_with('+') {
        return parse_decimal_duration(&value[1..], unit);
    }

    if value.starts_with('-') {
        return Err(TimingError::InvalidValue {
            message: "semantic Duration cannot be negative".to_owned(),
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
        && !integer_part.chars().all(|c| c.is_ascii_digit())
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

    let fractional_attoseconds = if fractional_part.is_empty() {
        0
    } else {
        let fractional_integer = fractional_part
            .parse::<u128>()
            .map_err(|_| TimingError::ArithmeticOverflow)?;

        let power = 10u128
            .checked_pow(
                u32::try_from(fractional_part.len())
                    .map_err(|_| TimingError::ArithmeticOverflow)?,
            )
            .ok_or(TimingError::ArithmeticOverflow)?;

        let numerator = fractional_integer
            .checked_mul(scale)
            .ok_or(TimingError::ArithmeticOverflow)?;

        if numerator % power != 0 {
            return Err(TimingError::InexactBackendConversion {
                attoseconds: numerator,
            });
        }

        numerator / power
    };

    let total = integer_attoseconds
        .checked_add(fractional_attoseconds)
        .ok_or(TimingError::ArithmeticOverflow)?;

    Ok(Duration::from_attoseconds(total))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_have_exact_scales() {
        assert_eq!(TimeUnit::Attosecond.attoseconds(), 1);
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
    fn parses_nanoseconds() {
        let duration = Duration::parse("20ns").unwrap();

        assert_eq!(
            duration.attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn parses_whitespace() {
        let duration = Duration::parse("20 ns").unwrap();

        assert_eq!(
            duration,
            Duration::nanoseconds(20).unwrap()
        );
    }

    #[test]
    fn parses_fractional_nanoseconds_exactly() {
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
    fn rejects_negative_duration() {
        assert!(Duration::parse("-1ns").is_err());
    }

    #[test]
    fn rejects_unknown_unit() {
        assert!(Duration::parse("10dt").is_err());
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
    fn time_point_arithmetic() {
        let start = TimePoint::ZERO;
        let duration = Duration::nanoseconds(20).unwrap();

        let end = start.checked_add(duration).unwrap();

        assert_eq!(end.attoseconds(), 20_000_000_000);

        assert_eq!(
            end.offset_from(start).unwrap(),
            TimeOffset::positive(duration).unwrap()
        );
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
    fn adjacent_intervals_touch_but_do_not_overlap() {
        let a = TimeInterval::new(
            TimePoint::ZERO,
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            ),
        )
        .unwrap();

        let b = TimeInterval::new(
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            ),
            TimePoint::from_duration(
                Duration::nanoseconds(40).unwrap(),
            ),
        )
        .unwrap();

        assert!(!a.overlaps(b));
        assert!(a.touches(b));
    }

    #[test]
    fn backend_tick_conversion_is_exact() {
        let dt = BackendTimeUnit::from_attoseconds_per_tick(
            1_000_000_000,
        )
        .unwrap();

        let duration = Duration::nanoseconds(20).unwrap();

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
        let dt = BackendTimeUnit::from_attoseconds_per_tick(
            2_000_000_000,
        )
        .unwrap();

        let duration = Duration::nanoseconds(1).unwrap();

        assert!(dt.duration_to_ticks(duration).is_err());
    }

    #[test]
    fn timing_bounds_work() {
        let minimum = Duration::nanoseconds(10).unwrap();
        let maximum = Duration::nanoseconds(20).unwrap();

        let bounds =
            TimingBounds::new(Some(minimum), Some(maximum))
                .unwrap();

        assert!(bounds.contains(
            Duration::nanoseconds(15).unwrap()
        ));

        assert!(!bounds.contains(
            Duration::nanoseconds(25).unwrap()
        ));
    }

    #[test]
    fn exact_bounds_are_detected() {
        let duration = Duration::nanoseconds(20).unwrap();

        let bounds = TimingBounds::exact(duration);

        assert_eq!(
            bounds.minimum(),
            Some(duration)
        );

        assert_eq!(
            bounds.maximum(),
            Some(duration)
        );

        assert!(bounds.contains(duration));
    }

    #[test]
    fn stretch_resolution_respects_bounds() {
        let minimum = Duration::nanoseconds(10).unwrap();
        let maximum = Duration::nanoseconds(30).unwrap();

        let bounds =
            TimingBounds::new(Some(minimum), Some(maximum))
                .unwrap();

        let mut stretch =
            Stretch::new("echo_gap", bounds).unwrap();

        assert!(!stretch.is_resolved());

        stretch
            .resolve(Duration::nanoseconds(20).unwrap())
            .unwrap();

        assert_eq!(
            stretch.resolved(),
            Some(Duration::nanoseconds(20).unwrap())
        );
    }

    #[test]
    fn invalid_stretch_resolution_is_rejected() {
        let bounds = TimingBounds::exact(
            Duration::nanoseconds(20).unwrap(),
        );

        let mut stretch =
            Stretch::new("gap", bounds).unwrap();

        assert!(
            stretch
                .resolve(Duration::nanoseconds(10).unwrap())
                .is_err()
        );
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

        let global = domain.to_global(local).unwrap();

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
    fn canonical_string_prefers_largest_exact_unit() {
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
    }

    #[test]
    fn zero_is_canonical() {
        assert_eq!(
            Duration::ZERO.canonical_string(),
            "0as"
        );
    }

    #[test]
    fn huge_finite_duration_is_representable() {
        let duration =
            Duration::from_attoseconds(u128::MAX);

        assert_eq!(
            duration.attoseconds(),
            u128::MAX
        );
    }

    #[test]
    fn no_machine_size_limit_is_encoded() {
        let tiny =
            Duration::nanoseconds(20).unwrap();

        let enormous =
            Duration::from_attoseconds(u128::MAX);

        assert!(tiny < enormous);
    }

    #[test]
    fn synchronization_window_is_validated() {
        let early = TimePoint::ZERO;

        let late =
            TimePoint::from_duration(
                Duration::nanoseconds(20).unwrap(),
            );

        let scope =
            SynchronizationScope::<u32>::Region;

        let sync =
            SynchronizationPoint::within(
                scope,
                Some(early),
                Some(late),
            )
            .unwrap();

        assert_eq!(sync.earliest(), Some(early));
        assert_eq!(sync.latest(), Some(late));
    }

    #[test]
    fn delay_fixed_resolves() {
        let delay =
            DelaySpec::fixed(
                Duration::nanoseconds(20).unwrap(),
            );

        assert!(delay.is_resolved());

        assert_eq!(
            delay.resolved_duration(),
            Some(Duration::nanoseconds(20).unwrap())
        );
    }
}