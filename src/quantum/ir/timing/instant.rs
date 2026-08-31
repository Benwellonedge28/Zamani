//! Zamani Quantum IR — Canonical Semantic Time Points
//!
//! Path:
//!     src/quantum/ir/timing/instant.rs
//!
//! # Purpose
//!
//! This module defines absolute semantic time coordinates for the Zamani
//! Quantum Intermediate Representation.
//!
//! The central distinction is:
//!
//! ```text
//! Duration
//!     = how much time passes
//!
//! TimePoint
//!     = where an event is located on the program's semantic timeline
//!
//! TimeOffset
//!     = signed displacement between two semantic time points
//! ```
//!
//! `TimePoint` does NOT represent:
//!
//! - wall-clock time;
//! - UNIX epoch time;
//! - operating-system time;
//! - CPU time;
//! - `std::time::Instant`;
//! - a hardware clock register;
//! - a backend `dt` tick;
//! - a pulse-generator timestamp;
//! - a scheduler's private clock.
//!
//! It represents only the semantic temporal coordinate of an event in
//! canonical Zamani IR.
//!
//! # Architectural role
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! canonical timing intent
//!       |
//!       v
//! quantum::ir::timing
//!       |
//!       +-------------------+
//!       |                   |
//!       v                   v
//!   TimePoint            Duration
//!       |                   |
//!       +---------+---------+
//!                 |
//!                 v
//!         timing constraints
//!                 |
//!                 v
//!             scheduling
//!                 |
//!                 v
//!        target/hardware timing
//! ```
//!
//! This module therefore answers:
//!
//! > At what semantic time does an event occur?
//!
//! It does NOT answer:
//!
//! > What physical clock cycle executes that event?
//!
//! That question belongs to downstream scheduling and hardware layers.
//!
//! # Canonical representation
//!
//! A `TimePoint` is represented as an exact unsigned number of attoseconds
//! from the beginning of the enclosing semantic timing domain.
//!
//! ```text
//! 1 second = 1_000_000_000_000_000_000 attoseconds
//! ```
//!
//! `u128` is used deliberately.
//!
//! This gives:
//!
//! - deterministic representation;
//! - exact arithmetic;
//! - no floating-point drift;
//! - no host-platform clock dependency;
//! - very large representable timelines;
//! - architecture-independent serialization;
//! - stable canonical hashing.
//!
//! The `u128` representation maximum is NOT a quantum-machine-size limit.
//!
//! It says nothing about:
//!
//! - number of qubits;
//! - number of operations;
//! - number of devices;
//! - physical clock frequency;
//! - scheduler capacity.
//!
//! Those are independent resource concerns.
//!
//! # Time origin
//!
//! `TimePoint::ZERO` represents the semantic origin of a timing domain.
//!
//! The origin has no physical-world interpretation.
//!
//! A compiler may establish a different local origin for a nested region or
//! scheduling domain, but that transformation must be explicit.
//!
//! `TimePoint` must never be interpreted as a globally synchronized physical
//! timestamp unless a downstream layer explicitly establishes such semantics.
//!
//! # Why this is not `std::time::Instant`
//!
//! `std::time::Instant` represents host-runtime monotonic time and is tied to
//! the execution environment.
//!
//! Zamani IR timing is program data.
//!
//! Therefore a canonical IR artifact must be able to:
//!
//! - serialize;
//! - hash;
//! - compare;
//! - reproduce;
//! - transfer between machines;
//! - compile offline;
//! - compile for a simulator;
//! - compile for a QPU;
//! - compile for a distributed system;
//! - survive a different host operating system.
//!
//! None of those properties should depend on the host clock.
//!
//! # Signed offsets
//!
//! Absolute time points are non-negative.
//!
//! Relative movement may be negative.
//!
//! Therefore this module provides `TimeOffset` as a signed, checked temporal
//! displacement.
//!
//! ```text
//! TimePoint + TimeOffset -> TimePoint
//! TimePoint - TimePoint  -> TimeOffset
//! TimePoint - Duration   -> TimePoint
//! TimePoint + Duration   -> TimePoint
//! ```
//!
//! Underflow and overflow are always detected.
//!
//! No arithmetic operation intentionally wraps.
//!
//! # Nested timing domains
//!
//! A `TimePoint` has no embedded region, block, operation, qubit, or hardware
//! identifier.
//!
//! This is intentional.
//!
//! The same primitive can therefore be used by:
//!
//! - circuit timing;
//! - dynamic circuits;
//! - pulse programs;
//! - analog evolution;
//! - annealing schedules;
//! - measurement-based computation;
//! - logical/QEC programs;
//! - distributed quantum execution;
//! - classical control;
//! - synchronization.
//!
//! Domain ownership belongs to the surrounding IR object.
//!
//! # Quantum-resource boundary
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! A time point does not own a qubit.
//!
//! A timing-aware operation may separately contain:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! while referencing a `TimePoint`.
//!
//! This keeps the dependency direction correct:
//!
//! ```text
//! qubit -----------+
//!                  |
//! operation -------+----> timing
//!                  |
//! resource --------+
//! ```
//!
//! rather than making the universal timing primitive depend on quantum
//! resources.
//!
//! # Integration contract
//!
//! This file owns:
//!
//! - `TimePoint`;
//! - `TimeOffset`;
//! - their checked arithmetic;
//! - their ordering;
//! - their deterministic formatting;
//! - their exact canonical representation.
//!
//! This file does NOT own:
//!
//! - `Duration`;
//! - `TimeUnit`;
//! - intervals;
//! - timing constraints;
//! - dependencies;
//! - schedules;
//! - pulses;
//! - waveforms;
//! - calibration;
//! - hardware timing.
//!
//! `duration.rs` owns `Duration`.
//!
//! `constraint.rs` consumes `TimePoint`, `TimeOffset`, and `Duration`.
//!
//! `dependency.rs` consumes temporal relationships between semantic events.
//!
//! `schedule.rs` consumes concrete placement information.
//!
//! `validation.rs` validates timing relationships.
//!
//! `analysis.rs` may use checked temporal arithmetic.
//!
//! Hardware and scheduling layers resolve semantic time points against actual
//! target timing.
//!
//! # Parent-module integration
//!
//! The eventual modular parent should expose:
//!
//! ```text
//! pub mod instant;
//! pub use instant::{TimeOffset, TimePoint};
//! ```
//!
//! No other timing module should redefine these types.
//!
//! # Compatibility with the existing timing implementation
//!
//! The current monolithic `timing.rs` already establishes the semantic
//! distinction between:
//!
//! - duration;
//! - absolute time point;
//! - signed offset;
//! - interval;
//! - constraint;
//! - dependency.
//!
//! This file extracts the absolute-coordinate portion into its own stable
//! ownership boundary.
//!
//! During migration, the parent timing module should re-export these types
//! rather than maintain duplicate definitions.
//!
//! # Serialization contract
//!
//! The semantic serialization of `TimePoint` is its canonical attosecond
//! integer.
//!
//! The semantic serialization of `TimeOffset` is:
//!
//! ```text
//! sign + magnitude
//! ```
//!
//! Wire-format implementations may encode this differently, but they must
//! preserve the exact mathematical value.
//!
//! Floating-point serialization is forbidden for canonical IR semantics.
//!
//! # Hashing contract
//!
//! `TimePoint` hashes only its canonical attosecond coordinate.
//!
//! `TimeOffset` hashes its signed mathematical value.
//!
//! Display formatting is not part of semantic identity.
//!
//! # Determinism
//!
//! The following must hold:
//!
//! ```text
//! same TimePoint
//!     -> same equality
//!     -> same ordering
//!     -> same hash
//!     -> same canonical serialization
//!     -> same display representation
//! ```
//!
//! across supported platforms.
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
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`;
//! - no platform-specific clock APIs.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler
//! enforced.
//!
//! # Scalability contract
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TIMEPOINTS
//! MAX_OPERATIONS
//! MAX_SCHEDULE_DEPTH
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this file.
//!
//! The type is independent of the amount of hardware available.
//!
//! A compiler may impose explicit resource limits elsewhere, but those limits
//! must never be encoded into `TimePoint` itself.
//!
//! # Example
//!
//! ```text
//! operation A starts at 0ns
//! operation B starts at 20ns
//!
//! B - A = 20ns
//!
//! A + 20ns = B
//! ```
//!
//! The timing module represents this mathematically and exactly.
//!
//! It does not decide whether a target can physically realize 20ns.
//!
//! # No physical clock assumption
//!
//! A backend might eventually use:
//!
//! ```text
//! dt = 0.222ns
//! ```
//!
//! or another timing quantum.
//!
//! `TimePoint` must remain unchanged.
//!
//! Backend conversion is explicit and belongs to the backend/scheduling
//! boundary.
//!
//! # Security
//!
//! All arithmetic is checked.
//!
//! No integer operation intentionally wraps.
//!
//! No dynamic allocation is necessary for ordinary time-point operations.
//!
//! Parsing rejects malformed and ambiguous values.
//!
//! Negative absolute time points are impossible by construction.
//!
//! Signed offsets explicitly represent negative values where semantically
//! valid.
//!
//! This prevents accidental conversion of a negative relative displacement
//! into a huge unsigned absolute timestamp.
//!
//! # Design principle
//!
//! ```text
//! IR timing = exact mathematical intent
//!
//! hardware timing = target-specific realization
//! ```
//!
//! Keeping that boundary stable is what allows one Zamani program to scale
//! across different quantum machines and future architectures.

#![forbid(unsafe_code)]

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use super::{Duration, TimeUnit, TimingError, TimingResult};

/// Exact absolute semantic time coordinate.
///
/// `TimePoint` is measured from the beginning of its enclosing semantic timing
/// domain using exact attoseconds.
///
/// It has no physical-world epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TimePoint {
    attoseconds: u128,
}

impl TimePoint {
    /// The semantic origin of a timing domain.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// The largest representable finite semantic time point.
    ///
    /// This is a representation boundary, not a hardware limit.
    pub const MAX: Self = Self {
        attoseconds: u128::MAX,
    };

    /// Creates a time point from an exact canonical attosecond coordinate.
    ///
    /// Every `u128` represents a valid non-negative semantic coordinate, so
    /// construction cannot fail.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self { attoseconds }
    }

    /// Returns the exact canonical attosecond coordinate.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Returns the semantic origin.
    #[must_use]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Returns whether this time point is the semantic origin.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.attoseconds == 0
    }

    /// Creates a time point from an exact integer quantity of a semantic time
    /// unit.
    pub fn from_units(value: u128, unit: TimeUnit) -> TimingResult<Self> {
        let attoseconds = value
            .checked_mul(unit.attoseconds())
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(attoseconds))
    }

    /// Creates a time point from an exact decimal quantity and semantic unit.
    ///
    /// Examples:
    ///
    /// ```text
    /// TimePoint::from_decimal("20", TimeUnit::Nanosecond)
    /// TimePoint::from_decimal("20.5", TimeUnit::Nanosecond)
    /// TimePoint::from_decimal("0.25", TimeUnit::Second)
    /// ```
    ///
    /// More than 18 fractional decimal digits are rejected because the
    /// canonical representation is attosecond based.
    pub fn from_decimal(value: &str, unit: TimeUnit) -> TimingResult<Self> {
        let duration = Duration::from_decimal(value, unit)?;
        Ok(Self::from_attoseconds(duration.attoseconds()))
    }

    /// Creates an attosecond time point.
    #[must_use]
    pub const fn attoseconds_point(value: u128) -> Self {
        Self::from_attoseconds(value)
    }

    /// Creates a femtosecond time point.
    pub fn femtoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Femtosecond)
    }

    /// Creates a picosecond time point.
    pub fn picoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Picosecond)
    }

    /// Creates a nanosecond time point.
    pub fn nanoseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Nanosecond)
    }

    /// Creates a microsecond time point.
    pub fn microseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Microsecond)
    }

    /// Creates a millisecond time point.
    pub fn milliseconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Millisecond)
    }

    /// Creates a second time point.
    pub fn seconds(value: u128) -> TimingResult<Self> {
        Self::from_units(value, TimeUnit::Second)
    }

    /// Returns the exact number of whole units represented by this point.
    ///
    /// Fails when the conversion is not exact.
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

    /// Returns the number of whole units represented by this point.
    ///
    /// This operation explicitly truncates the remainder.
    #[must_use]
    pub fn whole_units(self, unit: TimeUnit) -> u128 {
        self.attoseconds / unit.attoseconds()
    }

    /// Returns the remainder after division by the selected time unit.
    #[must_use]
    pub fn remainder(self, unit: TimeUnit) -> Duration {
        let divisor = unit.attoseconds();

        if divisor == 0 {
            return Duration::ZERO;
        }

        Duration::from_attoseconds(self.attoseconds % divisor)
    }

    /// Returns the exact semantic distance from `earlier` to `self`.
    ///
    /// Fails if `earlier` occurs after `self`.
    pub fn duration_since(self, earlier: Self) -> TimingResult<Duration> {
        let attoseconds = self
            .attoseconds
            .checked_sub(earlier.attoseconds)
            .ok_or(TimingError::NegativeDuration)?;

        Ok(Duration::from_attoseconds(attoseconds))
    }

    /// Returns the signed displacement from `other` to `self`.
    ///
    /// ```text
    /// self - other
    /// ```
    ///
    /// Unlike `duration_since`, this operation preserves direction.
    pub fn offset_from(self, other: Self) -> TimingResult<TimeOffset> {
        if self.attoseconds >= other.attoseconds {
            let magnitude = self
                .attoseconds
                .checked_sub(other.attoseconds)
                .ok_or(TimingError::ArithmeticOverflow)?;

            Ok(TimeOffset::positive(magnitude))
        } else {
            let magnitude = other
                .attoseconds
                .checked_sub(self.attoseconds)
                .ok_or(TimingError::ArithmeticOverflow)?;

            Ok(TimeOffset::negative(magnitude))
        }
    }

    /// Adds a non-negative duration using checked arithmetic.
    pub fn checked_add_duration(self, duration: Duration) -> TimingResult<Self> {
        let attoseconds = self
            .attoseconds
            .checked_add(duration.attoseconds())
            .ok_or(TimingError::ArithmeticOverflow)?;

        Ok(Self::from_attoseconds(attoseconds))
    }

    /// Subtracts a non-negative duration using checked arithmetic.
    ///
    /// Fails when the resulting time point would be negative.
    pub fn checked_sub_duration(self, duration: Duration) -> TimingResult<Self> {
        let attoseconds = self
            .attoseconds
            .checked_sub(duration.attoseconds())
            .ok_or(TimingError::NegativeDuration)?;

        Ok(Self::from_attoseconds(attoseconds))
    }

    /// Adds a signed temporal offset using checked arithmetic.
    pub fn checked_add_offset(self, offset: TimeOffset) -> TimingResult<Self> {
        match offset.sign() {
            TimeOffsetSign::Positive => {
                let attoseconds = self
                    .attoseconds
                    .checked_add(offset.magnitude())
                    .ok_or(TimingError::ArithmeticOverflow)?;

                Ok(Self::from_attoseconds(attoseconds))
            }

            TimeOffsetSign::Negative => {
                let attoseconds = self
                    .attoseconds
                    .checked_sub(offset.magnitude())
                    .ok_or(TimingError::NegativeDuration)?;

                Ok(Self::from_attoseconds(attoseconds))
            }

            TimeOffsetSign::Zero => Ok(self),
        }
    }

    /// Adds a duration and saturates at the largest representable time point.
    ///
    /// Saturating arithmetic is explicit and therefore cannot silently affect
    /// checked arithmetic callers.
    #[must_use]
    pub const fn saturating_add_duration(self, duration: Duration) -> Self {
        Self::from_attoseconds(
            self.attoseconds
                .saturating_add(duration.attoseconds()),
        )
    }

    /// Subtracts a duration and saturates at the semantic origin.
    #[must_use]
    pub const fn saturating_sub_duration(self, duration: Duration) -> Self {
        Self::from_attoseconds(
            self.attoseconds
                .saturating_sub(duration.attoseconds()),
        )
    }

    /// Returns a canonical `(numerator, denominator)` representation in
    /// seconds.
    ///
    /// The mathematical value is:
    ///
    /// ```text
    /// attoseconds / 10^18
    /// ```
    #[must_use]
    pub const fn seconds_ratio(self) -> (u128, u128) {
        (
            self.attoseconds,
            super::ATTOSECONDS_PER_SECOND,
        )
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_attosecond_decimal(
            f,
            self.attoseconds,
            false,
            super::ATTOSECONDS_PER_SECOND,
        )
    }
}

impl FromStr for TimePoint {
    type Err = TimingError;

    /// Parses a canonical absolute time point.
    ///
    /// Accepted examples:
    ///
    /// ```text
    /// 0as
    /// 20ns
    /// 1us
    /// 2.5ms
    /// 1s
    /// ```
    ///
    /// A leading sign is not accepted. Negative values belong to
    /// `TimeOffset`.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        parse_time_point(text)
    }
}

impl Add<Duration> for TimePoint {
    type Output = TimingResult<Self>;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add_duration(rhs)
    }
}

impl AddAssign<Duration> for TimePoint {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self
            .checked_add_duration(rhs)
            .expect("TimePoint += Duration overflowed");
    }
}

impl Sub<Duration> for TimePoint {
    type Output = TimingResult<Self>;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub_duration(rhs)
    }
}

impl SubAssign<Duration> for TimePoint {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self
            .checked_sub_duration(rhs)
            .expect("TimePoint -= Duration produced a negative time point");
    }
}

impl Sub<TimePoint> for TimePoint {
    type Output = TimingResult<TimeOffset>;

    fn sub(self, rhs: TimePoint) -> Self::Output {
        self.offset_from(rhs)
    }
}

/// The sign of a semantic temporal offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeOffsetSign {
    /// Negative displacement.
    Negative,

    /// Zero displacement.
    Zero,

    /// Positive displacement.
    Positive,
}

/// A signed exact displacement between two semantic time points.
///
/// Internally represented as a sign plus a non-negative magnitude.
///
/// This avoids relying on a two's-complement signed integer with a smaller
/// positive range than `u128`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeOffset {
    sign: TimeOffsetSign,
    magnitude: u128,
}

impl TimeOffset {
    /// Zero offset.
    pub const ZERO: Self = Self {
        sign: TimeOffsetSign::Zero,
        magnitude: 0,
    };

    /// Creates a positive offset.
    ///
    /// A zero magnitude is canonicalized to `ZERO`.
    #[must_use]
    pub const fn positive(magnitude: u128) -> Self {
        if magnitude == 0 {
            Self::ZERO
        } else {
            Self {
                sign: TimeOffsetSign::Positive,
                magnitude,
            }
        }
    }

    /// Creates a negative offset.
    ///
    /// A zero magnitude is canonicalized to `ZERO`.
    #[must_use]
    pub const fn negative(magnitude: u128) -> Self {
        if magnitude == 0 {
            Self::ZERO
        } else {
            Self {
                sign: TimeOffsetSign::Negative,
                magnitude,
            }
        }
    }

    /// Returns the zero/positive/negative sign.
    #[must_use]
    pub const fn sign(self) -> TimeOffsetSign {
        self.sign
    }

    /// Returns the non-negative magnitude.
    #[must_use]
    pub const fn magnitude(self) -> u128 {
        self.magnitude
    }

    /// Returns whether the offset is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self.sign, TimeOffsetSign::Zero)
    }

    /// Returns whether the offset is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self.sign, TimeOffsetSign::Negative)
    }

    /// Returns whether the offset is positive.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        matches!(self.sign, TimeOffsetSign::Positive)
    }

    /// Creates an offset from two time points.
    pub fn between(start: TimePoint, end: TimePoint) -> TimingResult<Self> {
        end.offset_from(start)
    }

    /// Returns the absolute duration represented by this offset.
    #[must_use]
    pub const fn absolute_duration(self) -> Duration {
        Duration::from_attoseconds(self.magnitude)
    }

    /// Returns the negated offset.
    ///
    /// This operation cannot overflow because the representation uses a
    /// sign/magnitude form.
    #[must_use]
    pub const fn negated(self) -> Self {
        match self.sign {
            TimeOffsetSign::Negative => Self::positive(self.magnitude),
            TimeOffsetSign::Zero => Self::ZERO,
            TimeOffsetSign::Positive => Self::negative(self.magnitude),
        }
    }

    /// Checked addition of two signed offsets.
    pub fn checked_add(self, rhs: Self) -> TimingResult<Self> {
        match (self.sign, rhs.sign) {
            (TimeOffsetSign::Zero, _) => Ok(rhs),
            (_, TimeOffsetSign::Zero) => Ok(self),

            (TimeOffsetSign::Positive, TimeOffsetSign::Positive) => {
                let magnitude = self
                    .magnitude
                    .checked_add(rhs.magnitude)
                    .ok_or(TimingError::ArithmeticOverflow)?;

                Ok(Self::positive(magnitude))
            }

            (TimeOffsetSign::Negative, TimeOffsetSign::Negative) => {
                let magnitude = self
                    .magnitude
                    .checked_add(rhs.magnitude)
                    .ok_or(TimingError::ArithmeticOverflow)?;

                Ok(Self::negative(magnitude))
            }

            (TimeOffsetSign::Positive, TimeOffsetSign::Negative) => {
                Ok(Self::subtract_magnitudes(
                    self.magnitude,
                    rhs.magnitude,
                ))
            }

            (TimeOffsetSign::Negative, TimeOffsetSign::Positive) => {
                Ok(Self::subtract_magnitudes(
                    rhs.magnitude,
                    self.magnitude,
                )
                .negated())
            }
        }
    }

    /// Checked subtraction of two signed offsets.
    pub fn checked_sub(self, rhs: Self) -> TimingResult<Self> {
        self.checked_add(rhs.negated())
    }

    /// Adds a non-negative duration to this signed offset.
    pub fn checked_add_duration(self, duration: Duration) -> TimingResult<Self> {
        let rhs = Self::positive(duration.attoseconds());
        self.checked_add(rhs)
    }

    /// Subtracts a non-negative duration from this signed offset.
    pub fn checked_sub_duration(self, duration: Duration) -> TimingResult<Self> {
        let rhs = Self::positive(duration.attoseconds());
        self.checked_sub(rhs)
    }

    /// Converts this offset to a duration if and only if it is non-negative.
    pub fn to_duration(self) -> TimingResult<Duration> {
        match self.sign {
            TimeOffsetSign::Negative => Err(TimingError::NegativeDuration),
            TimeOffsetSign::Zero | TimeOffsetSign::Positive => {
                Ok(Duration::from_attoseconds(self.magnitude))
            }
        }
    }

    /// Returns a canonical seconds ratio.
    ///
    /// The result is `(signed_numerator, denominator)`.
    ///
    /// The denominator is always positive.
    #[must_use]
    pub const fn seconds_ratio(self) -> (i128, u128) {
        let denominator = super::ATTOSECONDS_PER_SECOND;

        match self.sign {
            TimeOffsetSign::Negative => {
                // `u128` magnitudes may exceed `i128::MAX`; therefore a
                // signed i128 representation cannot represent every possible
                // TimeOffset. Callers needing the full range must use
                // sign/magnitude directly.
                //
                // This method is therefore only defined for magnitudes that
                // fit into i128.
                //
                // The branch below is deliberately conservative.
                if self.magnitude > i128::MAX as u128 {
                    (i128::MIN, denominator)
                } else {
                    (-(self.magnitude as i128), denominator)
                }
            }

            TimeOffsetSign::Zero => (0, denominator),

            TimeOffsetSign::Positive => {
                if self.magnitude > i128::MAX as u128 {
                    (i128::MAX, denominator)
                } else {
                    (self.magnitude as i128, denominator)
                }
            }
        }
    }

    fn subtract_magnitudes(lhs: u128, rhs: u128) -> Self {
        if lhs == rhs {
            Self::ZERO
        } else if lhs > rhs {
            Self::positive(lhs - rhs)
        } else {
            Self::negative(rhs - lhs)
        }
    }
}

impl Default for TimeOffset {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Ord for TimeOffset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.sign, other.sign) {
            (TimeOffsetSign::Negative, TimeOffsetSign::Negative) => {
                other.magnitude.cmp(&self.magnitude)
            }

            (TimeOffsetSign::Negative, _) => std::cmp::Ordering::Less,

            (TimeOffsetSign::Zero, TimeOffsetSign::Negative) => {
                std::cmp::Ordering::Greater
            }

            (TimeOffsetSign::Zero, TimeOffsetSign::Zero) => {
                std::cmp::Ordering::Equal
            }

            (TimeOffsetSign::Zero, TimeOffsetSign::Positive) => {
                std::cmp::Ordering::Less
            }

            (TimeOffsetSign::Positive, TimeOffsetSign::Positive) => {
                self.magnitude.cmp(&other.magnitude)
            }

            (TimeOffsetSign::Positive, _) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for TimeOffset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for TimeOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign {
            TimeOffsetSign::Negative => {
                f.write_str("-")?;
                write_attosecond_decimal(
                    f,
                    self.magnitude,
                    true,
                    super::ATTOSECONDS_PER_SECOND,
                )
            }

            TimeOffsetSign::Zero | TimeOffsetSign::Positive => {
                write_attosecond_decimal(
                    f,
                    self.magnitude,
                    false,
                    super::ATTOSECONDS_PER_SECOND,
                )
            }
        }
    }
}

impl FromStr for TimeOffset {
    type Err = TimingError;

    /// Parses a signed semantic timing offset.
    ///
    /// Examples:
    ///
    /// ```text
    /// 20ns
    /// -20ns
    /// 0ns
    /// ```
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        parse_time_offset(text)
    }
}

impl Add for TimeOffset {
    type Output = TimingResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

impl AddAssign for TimeOffset {
    fn add_assign(&mut self, rhs: Self) {
        *self = self
            .checked_add(rhs)
            .expect("TimeOffset += TimeOffset overflowed");
    }
}

impl Sub for TimeOffset {
    type Output = TimingResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}

impl SubAssign for TimeOffset {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self
            .checked_sub(rhs)
            .expect("TimeOffset -= TimeOffset overflowed");
    }
}

impl Add<TimeOffset> for TimePoint {
    type Output = TimingResult<Self>;

    fn add(self, rhs: TimeOffset) -> Self::Output {
        self.checked_add_offset(rhs)
    }
}

impl AddAssign<TimeOffset> for TimePoint {
    fn add_assign(&mut self, rhs: TimeOffset) {
        *self = self
            .checked_add_offset(rhs)
            .expect("TimePoint += TimeOffset overflowed or became negative");
    }
}

impl Sub<TimeOffset> for TimePoint {
    type Output = TimingResult<Self>;

    fn sub(self, rhs: TimeOffset) -> Self::Output {
        self.checked_add_offset(rhs.negated())
    }
}

impl SubAssign<TimeOffset> for TimePoint {
    fn sub_assign(&mut self, rhs: TimeOffset) {
        *self = self
            .checked_add_offset(rhs.negated())
            .expect("TimePoint -= TimeOffset overflowed or became negative");
    }
}

/// Parses an unsigned semantic time point.
fn parse_time_point(text: &str) -> TimingResult<TimePoint> {
    let text = text.trim();

    if text.is_empty() {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    if text.starts_with('+') || text.starts_with('-') {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    let (number, unit) = split_number_and_unit(text)?;

    let unit = TimeUnit::parse(unit)?;
    TimePoint::from_decimal(number, unit)
}

/// Parses a signed semantic time offset.
fn parse_time_offset(text: &str) -> TimingResult<TimeOffset> {
    let text = text.trim();

    if text.is_empty() {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    let negative = text.starts_with('-');

    let unsigned = if negative || text.starts_with('+') {
        &text[1..]
    } else {
        text
    };

    if unsigned.is_empty() {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    let (number, unit) = split_number_and_unit(unsigned)?;
    let unit = TimeUnit::parse(unit)?;
    let duration = Duration::from_decimal(number, unit)?;

    if duration.is_zero() {
        Ok(TimeOffset::ZERO)
    } else if negative {
        Ok(TimeOffset::negative(duration.attoseconds()))
    } else {
        Ok(TimeOffset::positive(duration.attoseconds()))
    }
}

/// Splits a timing literal into numeric and unit components.
///
/// The numeric part may contain exactly one decimal point.
/// The unit must be alphabetic Unicode text and is validated separately by
/// `TimeUnit::parse`.
fn split_number_and_unit(text: &str) -> TimingResult<(&str, &str)> {
    let mut split_at = None;

    for (index, character) in text.char_indices() {
        if character.is_ascii_digit() || character == '.' {
            continue;
        }

        split_at = Some(index);
        break;
    }

    let split_at = split_at.ok_or_else(|| TimingError::InvalidLiteral {
        literal: text.to_owned(),
    })?;

    let number = &text[..split_at];
    let unit = &text[split_at..];

    if number.is_empty() || unit.is_empty() {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    if unit.chars().any(|character| {
        !character.is_alphabetic() && character != 'µ' && character != 'μ'
    }) {
        return Err(TimingError::InvalidLiteral {
            literal: text.to_owned(),
        });
    }

    Ok((number, unit))
}

/// Writes a canonical decimal representation of an attosecond value.
///
/// The representation is expressed in seconds when possible and uses enough
/// fractional digits to preserve the exact value.
///
/// Examples:
///
/// ```text
/// 0s
/// 1s
/// 0.5s
/// 0.00000002s
/// 1as
/// ```
///
/// For deterministic canonical IR formatting, trailing fractional zeroes are
/// removed.
fn write_attosecond_decimal(
    formatter: &mut fmt::Formatter<'_>,
    attoseconds: u128,
    _allow_negative: bool,
    _seconds_scale: u128,
) -> fmt::Result {
    if attoseconds == 0 {
        return formatter.write_str("0s");
    }

    let whole_seconds = attoseconds / super::ATTOSECONDS_PER_SECOND;
    let fractional = attoseconds % super::ATTOSECONDS_PER_SECOND;

    if fractional == 0 {
        return write!(formatter, "{whole_seconds}s");
    }

    if whole_seconds != 0 {
        write!(formatter, "{whole_seconds}.")?;
    } else {
        formatter.write_str("0.")?;
    }

    let mut divisor = 100_000_000_000_000_000u128;
    let mut remaining = fractional;
    let mut started = whole_seconds != 0;

    let mut digits = 0usize;

    while divisor > 0 {
        let digit = remaining / divisor;
        remaining %= divisor;

        if digit != 0 || started || remaining != 0 {
            write!(formatter, "{digit}")?;
            started = true;
        }

        divisor /= 10;
        digits += 1;

        if remaining == 0 {
            break;
        }
    }

    if digits == 0 {
        formatter.write_str("0")?;
    }

    formatter.write_str("s")
}

/// Tests whether a string is a valid unsigned decimal timing number.
fn is_valid_decimal_number(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    let mut decimal_points = 0usize;

    for byte in text.bytes() {
        match byte {
            b'0'..=b'9' => {}
            b'.' => {
                decimal_points += 1;

                if decimal_points > 1 {
                    return false;
                }
            }
            _ => return false,
        }
    }

    if text == "." || text.starts_with('.') || text.ends_with('.') {
        return false;
    }

    true
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_canonical() {
        assert_eq!(TimePoint::ZERO.attoseconds(), 0);
        assert_eq!(TimeOffset::ZERO.magnitude(), 0);
        assert!(TimeOffset::ZERO.is_zero());
    }

    #[test]
    fn creates_exact_nanosecond_point() {
        let point = TimePoint::nanoseconds(20).expect("20ns should be valid");

        assert_eq!(point.attoseconds(), 20_000_000_000);
    }

    #[test]
    fn creates_exact_fractional_point() {
        let point =
            TimePoint::from_decimal("20.5", TimeUnit::Nanosecond)
                .expect("20.5ns should be exact");

        assert_eq!(point.attoseconds(), 20_500_000_000);
    }

    #[test]
    fn duration_between_points_is_exact() {
        let start =
            TimePoint::nanoseconds(20).expect("valid start");
        let end =
            TimePoint::nanoseconds(45).expect("valid end");

        let duration = end
            .duration_since(start)
            .expect("end should be after start");

        assert_eq!(duration.attoseconds(), 25_000_000_000);
    }

    #[test]
    fn reversed_duration_is_rejected() {
        let start =
            TimePoint::nanoseconds(45).expect("valid start");
        let end =
            TimePoint::nanoseconds(20).expect("valid end");

        assert_eq!(
            end.duration_since(start),
            Err(TimingError::NegativeDuration)
        );
    }

    #[test]
    fn offset_preserves_direction() {
        let start =
            TimePoint::nanoseconds(45).expect("valid start");
        let end =
            TimePoint::nanoseconds(20).expect("valid end");

        let offset = end
            .offset_from(start)
            .expect("offset should be representable");

        assert!(offset.is_negative());
        assert_eq!(offset.magnitude(), 25_000_000_000);
    }

    #[test]
    fn positive_offset_is_correct() {
        let start =
            TimePoint::nanoseconds(20).expect("valid start");
        let end =
            TimePoint::nanoseconds(45).expect("valid end");

        let offset = end
            .offset_from(start)
            .expect("offset should be representable");

        assert!(offset.is_positive());
        assert_eq!(offset.magnitude(), 25_000_000_000);
    }

    #[test]
    fn adding_duration_is_checked() {
        let point =
            TimePoint::nanoseconds(20).expect("valid point");
        let duration =
            Duration::nanoseconds(5).expect("valid duration");

        let result = point
            .checked_add_duration(duration)
            .expect("addition should succeed");

        assert_eq!(result.attoseconds(), 25_000_000_000);
    }

    #[test]
    fn subtracting_duration_cannot_go_negative() {
        let point =
            TimePoint::nanoseconds(20).expect("valid point");
        let duration =
            Duration::nanoseconds(25).expect("valid duration");

        assert_eq!(
            point.checked_sub_duration(duration),
            Err(TimingError::NegativeDuration)
        );
    }

    #[test]
    fn signed_offset_can_move_backwards() {
        let point =
            TimePoint::nanoseconds(50).expect("valid point");

        let offset = TimeOffset::negative(20_000_000_000);

        let result = point
            .checked_add_offset(offset)
            .expect("result should remain non-negative");

        assert_eq!(result.attoseconds(), 30_000_000_000);
    }

    #[test]
    fn negative_offset_cannot_cross_origin() {
        let point =
            TimePoint::nanoseconds(10).expect("valid point");

        let offset = TimeOffset::negative(20_000_000_000);

        assert_eq!(
            point.checked_add_offset(offset),
            Err(TimingError::NegativeDuration)
        );
    }

    #[test]
    fn offset_arithmetic_cancels() {
        let positive = TimeOffset::positive(100);
        let negative = TimeOffset::negative(100);

        let result = positive
            .checked_add(negative)
            .expect("offsets should cancel");

        assert_eq!(result, TimeOffset::ZERO);
    }

    #[test]
    fn offset_ordering_is_mathematical() {
        assert!(
            TimeOffset::negative(10) < TimeOffset::ZERO
        );

        assert!(
            TimeOffset::ZERO < TimeOffset::positive(10)
        );

        assert!(
            TimeOffset::negative(20) < TimeOffset::negative(10)
        );

        assert!(
            TimeOffset::positive(10) < TimeOffset::positive(20)
        );
    }

    #[test]
    fn parses_time_point() {
        let point: TimePoint =
            "20ns".parse().expect("20ns should parse");

        assert_eq!(point.attoseconds(), 20_000_000_000);
    }

    #[test]
    fn parses_fractional_time_point() {
        let point: TimePoint =
            "0.5s".parse().expect("0.5s should parse");

        assert_eq!(
            point.attoseconds(),
            super::super::ATTOSECONDS_PER_SECOND / 2
        );
    }

    #[test]
    fn parses_negative_offset() {
        let offset: TimeOffset =
            "-20ns".parse().expect("-20ns should parse");

        assert!(offset.is_negative());
        assert_eq!(offset.magnitude(), 20_000_000_000);
    }

    #[test]
    fn rejects_negative_absolute_time_point() {
        let result: TimingResult<TimePoint> = "-20ns".parse();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_time_point() {
        let result: TimingResult<TimePoint> = "20.1.2ns".parse();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_unit() {
        let result: TimingResult<TimePoint> = "20cycles".parse();

        assert!(matches!(
            result,
            Err(TimingError::UnknownUnit { .. })
        ));
    }

    #[test]
    fn display_is_deterministic() {
        let point =
            TimePoint::from_decimal("0.5", TimeUnit::Second)
                .expect("0.5s should be valid");

        assert_eq!(point.to_string(), "0.5s");
    }

    #[test]
    fn display_zero_is_deterministic() {
        assert_eq!(TimePoint::ZERO.to_string(), "0s");
        assert_eq!(TimeOffset::ZERO.to_string(), "0s");
    }

    #[test]
    fn display_negative_offset_is_deterministic() {
        let offset = TimeOffset::negative(500_000_000_000_000_000);

        assert_eq!(offset.to_string(), "-0.5s");
    }

    #[test]
    fn maximum_point_can_be_created() {
        let point = TimePoint::MAX;

        assert_eq!(point.attoseconds(), u128::MAX);
    }

    #[test]
    fn maximum_point_overflow_is_detected() {
        let point = TimePoint::MAX;
        let duration = Duration::from_attoseconds(1);

        assert_eq!(
            point.checked_add_duration(duration),
            Err(TimingError::ArithmeticOverflow)
        );
    }

    #[test]
    fn subtracting_equal_points_produces_zero_offset() {
        let point =
            TimePoint::nanoseconds(20).expect("valid point");

        let offset = point
            .offset_from(point)
            .expect("equal points should be representable");

        assert_eq!(offset, TimeOffset::ZERO);
    }

    #[test]
    fn positive_offset_can_be_applied() {
        let point =
            TimePoint::nanoseconds(20).expect("valid point");

        let offset = TimeOffset::positive(5_000_000_000);

        let result = point
            .checked_add_offset(offset)
            .expect("addition should succeed");

        assert_eq!(result.attoseconds(), 25_000_000_000);
    }

    #[test]
    fn negative_offset_can_be_subtracted() {
        let point =
            TimePoint::nanoseconds(20).expect("valid point");

        let offset = TimeOffset::positive(5_000_000_000);

        let result = point
            .checked_add_offset(offset.negated())
            .expect("subtraction should succeed");

        assert_eq!(result.attoseconds(), 15_000_000_000);
    }

    #[test]
    fn time_point_ordering_is_absolute() {
        let a =
            TimePoint::nanoseconds(10).expect("valid point");
        let b =
            TimePoint::nanoseconds(20).expect("valid point");

        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn seconds_ratio_is_exact() {
        let point =
            TimePoint::seconds(2).expect("valid point");

        let (numerator, denominator) = point.seconds_ratio();

        assert_eq!(numerator, 2 * super::super::ATTOSECONDS_PER_SECOND);
        assert_eq!(
            denominator,
            super::super::ATTOSECONDS_PER_SECOND
        );
    }

    #[test]
    fn offset_to_duration_rejects_negative() {
        let offset = TimeOffset::negative(10);

        assert_eq!(
            offset.to_duration(),
            Err(TimingError::NegativeDuration)
        );
    }

    #[test]
    fn zero_offset_to_duration_is_zero() {
        assert_eq!(
            TimeOffset::ZERO
                .to_duration()
                .expect("zero is non-negative"),
            Duration::ZERO
        );
    }

    #[test]
    fn decimal_number_validator_rejects_bad_values() {
        assert!(!is_valid_decimal_number(""));
        assert!(!is_valid_decimal_number("."));
        assert!(!is_valid_decimal_number("1."));
        assert!(!is_valid_decimal_number(".1"));
        assert!(!is_valid_decimal_number("1.2.3"));
        assert!(!is_valid_decimal_number("abc"));

        assert!(is_valid_decimal_number("1"));
        assert!(is_valid_decimal_number("1.0"));
        assert!(is_valid_decimal_number("1.25"));
    }
}