//! Zamani Quantum Hardware — Timing Primitives
//!
//! Production-grade, provider-neutral timing primitives for quantum hardware.
//!
//! # Purpose
//!
//! This module defines the canonical timing vocabulary used by
//! `quantum::hardware`.
//!
//! It represents:
//!
//! - physical time units;
//! - hardware clock cycles;
//! - clock domains;
//! - timing sources;
//! - source resolution;
//! - exact elapsed intervals;
//! - absolute timestamps;
//! - instruction timing;
//! - qubit timing;
//! - measurement timing;
//! - reset timing;
//! - classical feed-forward latency;
//! - readout latency;
//! - synchronization constraints;
//! - scheduling alignment;
//! - timing relationships;
//! - timing records;
//! - timing validation.
//!
//! # Architectural boundary
//!
//! This module is intentionally independent.
//!
//! It does NOT:
//!
//! - execute quantum programs;
//! - submit jobs;
//! - communicate with providers;
//! - access hardware;
//! - perform network I/O;
//! - read calibration data;
//! - perform routing;
//! - perform scheduling;
//! - calculate benchmark metrics;
//! - calculate throughput;
//! - calculate fidelity;
//! - own benchmark lifecycle state;
//! - depend on quantum IR;
//! - depend on provider-specific adapters.
//!
//! Higher-level modules consume this module.
//!
//! ```text
//!                         quantum::hardware::timing
//!                                    │
//!              ┌─────────────────────┼─────────────────────┐
//!              │                     │                     │
//!              ▼                     ▼                     ▼
//!        instruction_set        scheduling             calibration
//!              │                     │                     │
//!              └─────────────────────┼─────────────────────┘
//!                                    ▼
//!                               backend
//!                                    │
//!                                    ▼
//!                                adapters
//! ```
//!
//! The reverse dependency is forbidden.
//!
//! ```text
//! timing ─X─► backend
//! timing ─X─► provider
//! timing ─X─► benchmarking
//! timing ─X─► IR
//! ```
//!
//! # Hardware/provider timing versus host timing
//!
//! Hardware timing and host lifecycle timing are different scientific
//! quantities and MUST NOT be silently combined.
//!
//! Example:
//!
//! ```text
//! submit request
//!     │
//!     ├── provider queue       2.1 s
//!     ├── device execution     8.4 ms
//!     ├── readout              1.2 ms
//!     └── result transfer      4.7 ms
//!
//! host wall-clock             2.118 s
//! ```
//!
//! This module can represent the provider/device values, while host process
//! lifecycle measurement belongs to the execution layer.
//!
//! # Exactness policy
//!
//! Quantum-control hardware may expose timing below one nanosecond.
//!
//! `std::time::Duration` has nanosecond resolution. Therefore this module does
//! NOT use `Duration` as the only canonical representation.
//!
//! Exact timing is represented using:
//!
//! ```text
//! TimeValue
//!     value: u64
//!     unit: TimeUnit
//! ```
//!
//! The source unit is preserved.
//!
//! Conversion to `Duration` is checked. A value such as `1 ps` cannot be
//! represented exactly as `Duration` and therefore MUST NOT silently become
//! zero.
//!
//! # Clock cycles
//!
//! Clock cycles are not physical time by themselves.
//!
//! ```text
//! duration = cycles / frequency
//! ```
//!
//! Consequently, `TimeUnit::Cycles` requires an explicit `ClockRate` when
//! converting to physical time.
//!
//! # Missing versus zero
//!
//! `None` means timing was not supplied or measured.
//!
//! `Some(TimeValue::zero(...))` means zero was explicitly measured.
//!
//! These states are intentionally distinct.
//!
//! # Clock-domain safety
//!
//! Timestamps from unrelated clock domains MUST NOT be subtracted.
//!
//! For example:
//!
//! ```text
//! device_monotonic - host_monotonic
//! ```
//!
//! is invalid unless an explicit synchronization procedure has established a
//! common time domain.
//!
//! # Supported hardware models
//!
//! The representation is deliberately technology-neutral and supports:
//!
//! - superconducting;
//! - trapped ion;
//! - neutral atom;
//! - photonic;
//! - spin/semiconductor;
//! - topological;
//! - analog;
//! - annealing;
//! - logical/fault-tolerant;
//! - distributed quantum systems;
//! - simulators;
//! - emulators;
//! - hybrid quantum-classical systems.
//!
//! # Integration contract
//!
//! Future modules may depend on this file without requiring changes here:
//!
//! ```text
//! hardware::timing
//!     ├── instruction_set.rs
//!     ├── calibration.rs
//!     ├── topology.rs
//!     ├── compatibility.rs
//!     ├── scheduling.rs
//!     ├── backend.rs
//!     ├── execution.rs
//!     └── adapters/*
//! ```
//!
//! In particular:
//!
//! - `instruction_set.rs` uses `InstructionTiming`;
//! - `calibration.rs` may use `TimingValue` and `InstructionTiming`;
//! - `scheduling.rs` consumes timing constraints;
//! - `backend.rs` exposes backend timing information;
//! - provider adapters translate provider timing into these types;
//! - benchmarking consumes timing without redefining it.
//!
//! This file MUST NOT import those modules.
//!
//! # Stability
//!
//! Public types in this file form the stable hardware timing contract.
//!
//! Provider adapters MUST adapt to this contract instead of leaking provider
//! timing types into the core hardware layer.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! No external dependency is required.
//!
//! # Production invariants
//!
//! 1. Timing values are integer-based.
//! 2. Negative elapsed timing is impossible to construct.
//! 3. Overflow is detected.
//! 4. Unknown units are never silently interpreted.
//! 5. Cycles require a clock rate for physical conversion.
//! 6. Clock domains are preserved.
//! 7. Unrelated clocks cannot be subtracted.
//! 8. Sub-nanosecond precision is never silently truncated.
//! 9. Source resolution is retained.
//! 10. Source identity is retained.
//! 11. Host and hardware timing remain distinguishable.
//! 12. Explicit zero remains distinguishable from missing timing.
//! 13. No I/O is performed.
//! 14. No provider-specific concepts are required.
//! 15. No floating-point value is required for canonical timing.
//! 16. Serialization can be added by higher-level integration without changing
//!     the semantic model.
//! 17. Timing records are deterministic.
//! 18. Invalid custom units and clocks are rejected.
//! 19. Invalid clock frequencies are rejected.
//! 20. Invalid timing relationships are rejected.
//! 21. Timing constraints never execute or schedule operations themselves.
//! 22. This module contains no benchmark-specific interpretation.

use std::fmt;
use std::time::Duration;

// =============================================================================
// Schema/API versions
// =============================================================================

/// Stable schema version of the hardware timing model.
pub const HARDWARE_TIMING_SCHEMA_VERSION: u16 = 1;

/// Stable API version of the hardware timing model.
pub const HARDWARE_TIMING_API_VERSION: u16 = 1;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by hardware timing primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingError {
    /// A timing value was invalid.
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },

    /// A conversion would overflow.
    ConversionOverflow {
        source: TimeUnit,
        target: TimeUnit,
    },

    /// A conversion requires information that was not supplied.
    MissingClockRate,

    /// A clock rate is invalid.
    InvalidClockRate {
        reason: &'static str,
    },

    /// A custom unit has an empty identifier.
    EmptyCustomUnit,

    /// A custom clock domain has an empty identifier.
    EmptyCustomClock,

    /// A custom source has an empty identifier.
    EmptyCustomSource,

    /// Timestamps belong to incompatible clock domains.
    ClockDomainMismatch {
        start: ClockDomain,
        end: ClockDomain,
    },

    /// An end timestamp precedes a start timestamp.
    EndBeforeStart {
        field: &'static str,
    },

    /// A requested `Duration` conversion would lose precision.
    LossyDurationConversion {
        value: TimeValue,
    },

    /// A timing record is structurally invalid.
    InvalidRecord {
        reason: &'static str,
    },

    /// An alignment value is invalid.
    InvalidAlignment {
        reason: &'static str,
    },

    /// A timing relationship is invalid.
    InvalidConstraint {
        reason: &'static str,
    },
}

impl fmt::Display for TimingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => {
                write!(f, "invalid timing value for '{field}': {reason}")
            }

            Self::ConversionOverflow { source, target } => {
                write!(
                    f,
                    "timing conversion overflow from {source} to {target}"
                )
            }

            Self::MissingClockRate => {
                f.write_str("clock rate is required to convert cycles to time")
            }

            Self::InvalidClockRate { reason } => {
                write!(f, "invalid clock rate: {reason}")
            }

            Self::EmptyCustomUnit => {
                f.write_str("custom timing unit identifier must not be empty")
            }

            Self::EmptyCustomClock => {
                f.write_str("custom clock identifier must not be empty")
            }

            Self::EmptyCustomSource => {
                f.write_str("custom timing source identifier must not be empty")
            }

            Self::ClockDomainMismatch { start, end } => {
                write!(
                    f,
                    "cannot subtract timestamps from different clock domains: \
                     {start} versus {end}"
                )
            }

            Self::EndBeforeStart { field } => {
                write!(
                    f,
                    "end timestamp precedes start timestamp for '{field}'"
                )
            }

            Self::LossyDurationConversion { value } => {
                write!(
                    f,
                    "timing value {value} cannot be represented exactly as \
                     std::time::Duration"
                )
            }

            Self::InvalidRecord { reason } => {
                write!(f, "invalid hardware timing record: {reason}")
            }

            Self::InvalidAlignment { reason } => {
                write!(f, "invalid timing alignment: {reason}")
            }

            Self::InvalidConstraint { reason } => {
                write!(f, "invalid timing constraint: {reason}")
            }
        }
    }
}

impl std::error::Error for TimingError {}

// =============================================================================
// Time unit
// =============================================================================

/// Unit of an elapsed timing value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnit {
    /// Seconds.
    Seconds,

    /// Milliseconds.
    Milliseconds,

    /// Microseconds.
    Microseconds,

    /// Nanoseconds.
    Nanoseconds,

    /// Picoseconds.
    Picoseconds,

    /// Femtoseconds.
    Femtoseconds,

    /// Hardware/provider clock cycles.
    Cycles,

    /// Provider-defined timing unit.
    Custom(String),
}

impl TimeUnit {
    /// Returns the canonical machine-readable symbol.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Seconds => "s",
            Self::Milliseconds => "ms",
            Self::Microseconds => "us",
            Self::Nanoseconds => "ns",
            Self::Picoseconds => "ps",
            Self::Femtoseconds => "fs",
            Self::Cycles => "cycles",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns true for physical SI time units.
    pub fn is_physical_time(&self) -> bool {
        matches!(
            self,
            Self::Seconds
                | Self::Milliseconds
                | Self::Microseconds
                | Self::Nanoseconds
                | Self::Picoseconds
                | Self::Femtoseconds
        )
    }

    /// Returns the number of femtoseconds represented by one unit.
    ///
    /// `None` means the unit cannot be represented without additional
    /// information, such as cycles or a provider-defined custom unit.
    fn femtoseconds_per_unit(&self) -> Option<u64> {
        match self {
            Self::Seconds => Some(1_000_000_000_000_000),
            Self::Milliseconds => Some(1_000_000_000_000),
            Self::Microseconds => Some(1_000_000_000),
            Self::Nanoseconds => Some(1_000_000),
            Self::Picoseconds => Some(1_000),
            Self::Femtoseconds => Some(1),
            Self::Cycles | Self::Custom(_) => None,
        }
    }

    /// Validates this unit.
    pub fn validate(&self) -> Result<(), TimingError> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(TimingError::EmptyCustomUnit);
            }
        }

        Ok(())
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Clock domain
// =============================================================================

/// Originating clock domain of a timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClockDomain {
    /// Host monotonic clock.
    HostMonotonic,

    /// Host wall clock.
    HostWallClock,

    /// Quantum-device monotonic clock.
    DeviceMonotonic,

    /// Provider monotonic clock.
    ProviderMonotonic,

    /// Simulator logical clock.
    Simulation,

    /// External synchronized clock.
    External,

    /// Provider/device/user-defined clock.
    Custom(String),

    /// Clock domain was not specified.
    Unknown,
}

impl ClockDomain {
    /// Returns the stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::HostMonotonic => "host_monotonic",
            Self::HostWallClock => "host_wall_clock",
            Self::DeviceMonotonic => "device_monotonic",
            Self::ProviderMonotonic => "provider_monotonic",
            Self::Simulation => "simulation",
            Self::External => "external",
            Self::Custom(value) => value.as_str(),
            Self::Unknown => "unknown",
        }
    }

    /// Validates the clock domain.
    pub fn validate(&self) -> Result<(), TimingError> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(TimingError::EmptyCustomClock);
            }
        }

        Ok(())
    }

    /// Returns whether elapsed subtraction is intrinsically meaningful.
    pub fn supports_elapsed_difference(&self) -> bool {
        matches!(
            self,
            Self::HostMonotonic
                | Self::DeviceMonotonic
                | Self::ProviderMonotonic
                | Self::Simulation
                | Self::External
        )
    }
}

impl fmt::Display for ClockDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Timing source
// =============================================================================

/// Source that produced a timing measurement.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingSource {
    /// Zamani host measurement.
    Host,

    /// Physical quantum device measurement.
    Device,

    /// Quantum provider measurement.
    Provider,

    /// Simulator-generated timing.
    Simulator,

    /// Hardware emulator-generated timing.
    Emulator,

    /// External instrumentation.
    External,

    /// User/provider-defined source.
    Custom(String),
}

impl TimingSource {
    /// Returns a stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Host => "host",
            Self::Device => "device",
            Self::Provider => "provider",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::External => "external",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Validates the timing source.
    pub fn validate(&self) -> Result<(), TimingError> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(TimingError::EmptyCustomSource);
            }
        }

        Ok(())
    }
}

impl fmt::Display for TimingSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Clock rate
// =============================================================================

/// Frequency used to convert hardware clock cycles into physical time.
///
/// The value is stored as integer milli-hertz to avoid floating-point
/// ambiguity in the canonical representation.
///
/// Example:
///
/// ```text
/// 5 GHz = 5_000_000_000 Hz
///      = 5_000_000_000_000 mHz
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClockRate {
    millihertz: u64,
}

impl ClockRate {
    /// Creates a clock rate from integer hertz.
    pub const fn from_hz(hz: u64) -> Result<Self, TimingError> {
        match hz.checked_mul(1_000) {
            Some(millihertz) if hz > 0 => Ok(Self { millihertz }),
            _ => Err(TimingError::InvalidClockRate {
                reason: "frequency must be positive and representable",
            }),
        }
    }

    /// Creates a clock rate from milli-hertz.
    pub const fn from_millihz(millihertz: u64) -> Result<Self, TimingError> {
        if millihertz == 0 {
            return Err(TimingError::InvalidClockRate {
                reason: "frequency must be positive",
            });
        }

        Ok(Self { millihertz })
    }

    /// Returns the frequency in integer hertz when exactly representable.
    pub const fn as_hz(self) -> Option<u64> {
        if self.millihertz % 1_000 == 0 {
            Some(self.millihertz / 1_000)
        } else {
            None
        }
    }

    /// Returns the exact stored frequency in milli-hertz.
    pub const fn as_millihz(self) -> u64 {
        self.millihertz
    }

    /// Returns the clock period as a timing value when exactly representable
    /// in femtoseconds.
    pub fn period_femtoseconds(self) -> Result<TimeValue, TimingError> {
        // period_fs = 1e15 / frequency_hz
        //
        // With frequency represented as mHz:
        //
        // period_fs = 1e18 / millihertz
        //
        // We require an exact integer result rather than silently truncating.
        let numerator: u128 = 1_000_000_000_000_000_000;

        let period = numerator / u128::from(self.millihertz);
        let remainder = numerator % u128::from(self.millihertz);

        if remainder != 0 || period > u128::from(u64::MAX) {
            return Err(TimingError::ConversionOverflow {
                source: TimeUnit::Cycles,
                target: TimeUnit::Femtoseconds,
            });
        }

        TimeValue::new(
            period as u64,
            TimeUnit::Femtoseconds,
        )
    }
}

impl fmt::Display for ClockRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(hz) = self.as_hz() {
            write!(f, "{hz} Hz")
        } else {
            write!(f, "{} mHz", self.millihertz)
        }
    }
}

// =============================================================================
// Exact time value
// =============================================================================

/// Exact non-negative timing value with an explicit unit.
///
/// The integer value is the canonical representation.
///
/// This is intentionally not a floating-point quantity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeValue {
    value: u64,
    unit: TimeUnit,
}

impl TimeValue {
    /// Creates a validated timing value.
    pub fn new(value: u64, unit: TimeUnit) -> Result<Self, TimingError> {
        unit.validate()?;

        Ok(Self { value, unit })
    }

    /// Creates a zero timing value.
    pub fn zero(unit: TimeUnit) -> Result<Self, TimingError> {
        Self::new(0, unit)
    }

    /// Returns the numeric value.
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the unit.
    pub fn unit(&self) -> &TimeUnit {
        &self.unit
    }

    /// Returns whether the value is zero.
    pub const fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// Converts physical time to femtoseconds exactly.
    ///
    /// Cycles and custom units cannot be converted without additional
    /// information.
    pub fn to_femtoseconds(&self) -> Result<u64, TimingError> {
        let multiplier = match self.unit.femtoseconds_per_unit() {
            Some(value) => value,
            None => {
                return Err(TimingError::InvalidValue {
                    field: "time.value",
                    reason: "unit requires additional conversion information",
                });
            }
        };

        self.value
            .checked_mul(multiplier)
            .ok_or(TimingError::ConversionOverflow {
                source: self.unit.clone(),
                target: TimeUnit::Femtoseconds,
            })
    }

    /// Converts cycles into an exact physical duration when the clock rate
    /// produces an integer femtosecond result.
    pub fn cycles_to_femtoseconds(
        &self,
        clock_rate: ClockRate,
    ) -> Result<u64, TimingError> {
        if !matches!(self.unit, TimeUnit::Cycles) {
            return Err(TimingError::InvalidValue {
                field: "time.unit",
                reason: "cycles_to_femtoseconds requires TimeUnit::Cycles",
            });
        }

        let numerator = u128::from(self.value)
            .checked_mul(1_000_000_000_000_000_000)
            .ok_or(TimingError::ConversionOverflow {
                source: TimeUnit::Cycles,
                target: TimeUnit::Femtoseconds,
            })?;

        let denominator = u128::from(clock_rate.as_millihz());

        let result = numerator / denominator;
        let remainder = numerator % denominator;

        if remainder != 0 || result > u128::from(u64::MAX) {
            return Err(TimingError::ConversionOverflow {
                source: TimeUnit::Cycles,
                target: TimeUnit::Femtoseconds,
            });
        }

        Ok(result as u64)
    }

    /// Converts this value to `Duration` without losing precision.
    ///
    /// Values smaller than one nanosecond are rejected rather than silently
    /// truncated.
    pub fn to_duration(&self) -> Result<Duration, TimingError> {
        let femtoseconds = self.to_femtoseconds()?;

        if femtoseconds % 1_000_000 != 0 {
            return Err(TimingError::LossyDurationConversion {
                value: self.clone(),
            });
        }

        let nanoseconds = femtoseconds / 1_000_000;

        if nanoseconds > u128::from(u64::MAX) {
            return Err(TimingError::ConversionOverflow {
                source: self.unit.clone(),
                target: TimeUnit::Nanoseconds,
            });
        }

        Ok(Duration::from_nanos(nanoseconds as u64))
    }

    /// Converts to nanoseconds exactly.
    pub fn to_nanoseconds(&self) -> Result<u64, TimingError> {
        let femtoseconds = self.to_femtoseconds()?;

        if femtoseconds % 1_000_000 != 0 {
            return Err(TimingError::ConversionOverflow {
                source: self.unit.clone(),
                target: TimeUnit::Nanoseconds,
            });
        }

        let nanoseconds = femtoseconds / 1_000_000;

        if nanoseconds > u128::from(u64::MAX) {
            return Err(TimingError::ConversionOverflow {
                source: self.unit.clone(),
                target: TimeUnit::Nanoseconds,
            });
        }

        Ok(nanoseconds as u64)
    }

    /// Returns this value normalized into the requested physical unit if the
    /// conversion is exact.
    pub fn convert_to(&self, target: TimeUnit) -> Result<Self, TimingError> {
        target.validate()?;

        if self.unit == target {
            return Ok(self.clone());
        }

        let femtoseconds = self.to_femtoseconds()?;
        let multiplier = target
            .femtoseconds_per_unit()
            .ok_or(TimingError::InvalidValue {
                field: "target",
                reason: "target unit requires additional conversion information",
            })?;

        if femtoseconds % u64::from(multiplier) != 0 {
            return Err(TimingError::ConversionOverflow {
                source: self.unit.clone(),
                target,
            });
        }

        Self::new(femtoseconds / multiplier, target)
    }
}

impl fmt::Display for TimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Absolute timestamp with explicit clock-domain provenance.
///
/// The numeric value is interpreted using `unit`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timestamp {
    value: u64,
    unit: TimeUnit,
    clock: ClockDomain,
}

impl Timestamp {
    /// Creates a timestamp.
    pub fn new(
        value: u64,
        unit: TimeUnit,
        clock: ClockDomain,
    ) -> Result<Self, TimingError> {
        unit.validate()?;
        clock.validate()?;

        if matches!(unit, TimeUnit::Cycles) {
            return Err(TimingError::InvalidValue {
                field: "timestamp.unit",
                reason: "cycle timestamps require an explicit clock-rate model",
            });
        }

        Ok(Self {
            value,
            unit,
            clock,
        })
    }

    /// Returns the raw timestamp value.
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the timestamp unit.
    pub fn unit(&self) -> &TimeUnit {
        &self.unit
    }

    /// Returns the timestamp clock domain.
    pub fn clock_domain(&self) -> &ClockDomain {
        &self.clock
    }

    /// Calculates elapsed time between timestamps from the same domain.
    pub fn elapsed_since(&self, start: &Self) -> Result<TimeValue, TimingError> {
        if self.clock != start.clock {
            return Err(TimingError::ClockDomainMismatch {
                start: start.clock.clone(),
                end: self.clock.clone(),
            });
        }

        if self.value < start.value {
            return Err(TimingError::EndBeforeStart {
                field: "timestamp",
            });
        }

        let end = TimeValue::new(self.value, self.unit.clone())?;
        let begin = TimeValue::new(start.value, start.unit.clone())?;

        let end_fs = end.to_femtoseconds()?;
        let begin_fs = begin.to_femtoseconds()?;

        let elapsed = end_fs
            .checked_sub(begin_fs)
            .ok_or(TimingError::EndBeforeStart {
                field: "timestamp",
            })?;

        TimeValue::new(elapsed, TimeUnit::Femtoseconds)
    }
}

// =============================================================================
// Resolution
// =============================================================================

/// Resolution of the underlying timing source.
///
/// Resolution describes the smallest meaningful increment reported by the
/// source. It is provenance, not a promise that every measurement has that
/// exact accuracy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimingResolution {
    value: u64,
    unit: TimeUnit,
}

impl TimingResolution {
    /// Creates a resolution.
    pub fn new(value: u64, unit: TimeUnit) -> Result<Self, TimingError> {
        if value == 0 {
            return Err(TimingError::InvalidValue {
                field: "resolution.value",
                reason: "resolution must be positive",
            });
        }

        unit.validate()?;

        if matches!(unit, TimeUnit::Cycles) {
            return Err(TimingError::InvalidValue {
                field: "resolution.unit",
                reason: "resolution must use a physical time unit",
            });
        }

        Ok(Self { value, unit })
    }

    /// Returns the numeric resolution.
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns the resolution unit.
    pub fn unit(&self) -> &TimeUnit {
        &self.unit
    }

    /// Returns the resolution as an exact timing value.
    pub fn as_time_value(&self) -> Result<TimeValue, TimingError> {
        TimeValue::new(self.value, self.unit.clone())
    }
}

// =============================================================================
// Timing interval
// =============================================================================

/// An elapsed interval with source and clock provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimingInterval {
    duration: TimeValue,
    source: TimingSource,
    clock: ClockDomain,
    resolution: Option<TimingResolution>,
}

impl TimingInterval {
    /// Creates an interval.
    pub fn new(
        duration: TimeValue,
        source: TimingSource,
        clock: ClockDomain,
        resolution: Option<TimingResolution>,
    ) -> Result<Self, TimingError> {
        source.validate()?;
        clock.validate()?;
        duration.unit().validate()?;

        if let Some(resolution) = &resolution {
            if resolution.unit().is_physical_time()
                && duration.unit().is_physical_time()
            {
                let duration_fs = duration.to_femtoseconds()?;
                let resolution_fs = resolution.as_time_value()?.to_femtoseconds()?;

                if resolution_fs == 0 || duration_fs % resolution_fs != 0 {
                    return Err(TimingError::InvalidRecord {
                        reason: "duration is inconsistent with source resolution",
                    });
                }
            }
        }

        Ok(Self {
            duration,
            source,
            clock,
            resolution,
        })
    }

    /// Returns the duration.
    pub fn duration(&self) -> &TimeValue {
        &self.duration
    }

    /// Returns the timing source.
    pub fn source(&self) -> &TimingSource {
        &self.source
    }

    /// Returns the clock domain.
    pub fn clock_domain(&self) -> &ClockDomain {
        &self.clock
    }

    /// Returns the source resolution.
    pub fn resolution(&self) -> Option<&TimingResolution> {
        self.resolution.as_ref()
    }

    /// Returns whether the interval is explicitly zero.
    pub fn is_zero(&self) -> bool {
        self.duration.is_zero()
    }
}

// =============================================================================
// Timing category
// =============================================================================

/// Semantic category of a hardware timing measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingCategory {
    /// Gate/instruction execution time.
    Instruction,

    /// Qubit reset time.
    Reset,

    /// Measurement/readout time.
    Measurement,

    /// Classical processing latency.
    ClassicalProcessing,

    /// Classical-to-quantum feed-forward latency.
    FeedForward,

    /// Device synchronization latency.
    Synchronization,

    /// Queue latency reported by a provider.
    Queue,

    /// Device execution time.
    Execution,

    /// Readout transfer latency.
    Readout,

    /// Inter-device communication latency.
    Communication,

    /// Analog-program duration.
    AnalogProgram,

    /// Annealing duration.
    Annealing,

    /// Logical/fault-tolerant operation duration.
    LogicalOperation,

    /// Provider-defined category.
    ProviderDefined,
}

// =============================================================================
// Instruction timing
// =============================================================================

/// Timing constraints/properties for a hardware instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionTiming {
    /// Canonical instruction name.
    pub instruction: String,

    /// Optional duration.
    pub duration: Option<TimeValue>,

    /// Optional source of the duration.
    pub source: Option<TimingSource>,

    /// Optional source resolution.
    pub resolution: Option<TimingResolution>,

    /// Whether the duration is guaranteed by the backend contract.
    pub deterministic: bool,

    /// Minimum legal duration, when configurable.
    pub minimum_duration: Option<TimeValue>,

    /// Maximum legal duration, when configurable.
    pub maximum_duration: Option<TimeValue>,

    /// Required scheduling alignment.
    pub alignment: Option<TimeValue>,
}

impl InstructionTiming {
    /// Creates an instruction timing record.
    pub fn new(instruction: impl Into<String>) -> Result<Self, TimingError> {
        let instruction = instruction.into();

        if instruction.trim().is_empty() {
            return Err(TimingError::InvalidValue {
                field: "instruction",
                reason: "instruction name must not be empty",
            });
        }

        Ok(Self {
            instruction,
            duration: None,
            source: None,
            resolution: None,
            deterministic: false,
            minimum_duration: None,
            maximum_duration: None,
            alignment: None,
        })
    }

    /// Sets the instruction duration.
    pub fn with_duration(mut self, duration: TimeValue) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the measurement source.
    pub fn with_source(mut self, source: TimingSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the source resolution.
    pub fn with_resolution(
        mut self,
        resolution: TimingResolution,
    ) -> Self {
        self.resolution = Some(resolution);
        self
    }

    /// Marks the duration deterministic.
    pub fn deterministic(mut self, value: bool) -> Self {
        self.deterministic = value;
        self
    }

    /// Sets a minimum duration.
    pub fn with_minimum_duration(
        mut self,
        duration: TimeValue,
    ) -> Result<Self, TimingError> {
        if let Some(maximum) = &self.maximum_duration {
            let min_fs = duration.to_femtoseconds()?;
            let max_fs = maximum.to_femtoseconds()?;

            if min_fs > max_fs {
                return Err(TimingError::InvalidConstraint {
                    reason: "minimum duration exceeds maximum duration",
                });
            }
        }

        self.minimum_duration = Some(duration);
        Ok(self)
    }

    /// Sets a maximum duration.
    pub fn with_maximum_duration(
        mut self,
        duration: TimeValue,
    ) -> Result<Self, TimingError> {
        if let Some(minimum) = &self.minimum_duration {
            let min_fs = minimum.to_femtoseconds()?;
            let max_fs = duration.to_femtoseconds()?;

            if max_fs < min_fs {
                return Err(TimingError::InvalidConstraint {
                    reason: "maximum duration is below minimum duration",
                });
            }
        }

        self.maximum_duration = Some(duration);
        Ok(self)
    }

    /// Sets scheduling alignment.
    pub fn with_alignment(
        mut self,
        alignment: TimeValue,
    ) -> Result<Self, TimingError> {
        if alignment.is_zero() {
            return Err(TimingError::InvalidAlignment {
                reason: "alignment must be positive",
            });
        }

        if !alignment.unit().is_physical_time() {
            return Err(TimingError::InvalidAlignment {
                reason: "alignment must use a physical time unit",
            });
        }

        self.alignment = Some(alignment);
        Ok(self)
    }

    /// Validates the complete record.
    pub fn validate(&self) -> Result<(), TimingError> {
        if self.instruction.trim().is_empty() {
            return Err(TimingError::InvalidValue {
                field: "instruction",
                reason: "instruction name must not be empty",
            });
        }

        if let (Some(minimum), Some(maximum)) =
            (&self.minimum_duration, &self.maximum_duration)
        {
            if minimum.to_femtoseconds()? > maximum.to_femtoseconds()? {
                return Err(TimingError::InvalidConstraint {
                    reason: "minimum duration exceeds maximum duration",
                });
            }
        }

        if let Some(alignment) = &self.alignment {
            if alignment.is_zero() {
                return Err(TimingError::InvalidAlignment {
                    reason: "alignment must be positive",
                });
            }
        }

        if let Some(source) = &self.source {
            source.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Qubit timing
// =============================================================================

/// Timing properties associated with one physical qubit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitTiming {
    /// Physical qubit identifier.
    pub qubit: usize,

    /// Reset duration.
    pub reset: Option<TimeValue>,

    /// Measurement duration.
    pub measurement: Option<TimeValue>,

    /// Readout latency.
    pub readout_latency: Option<TimeValue>,

    /// Classical feed-forward latency.
    pub feed_forward_latency: Option<TimeValue>,

    /// Minimum spacing between operations.
    pub minimum_operation_spacing: Option<TimeValue>,

    /// Required scheduling alignment.
    pub alignment: Option<TimeValue>,
}

impl QubitTiming {
    /// Creates timing information for a physical qubit.
    pub fn new(qubit: usize) -> Result<Self, TimingError> {
        if qubit == usize::MAX {
            return Err(TimingError::InvalidValue {
                field: "qubit",
                reason: "qubit identifier is reserved",
            });
        }

        Ok(Self {
            qubit,
            reset: None,
            measurement: None,
            readout_latency: None,
            feed_forward_latency: None,
            minimum_operation_spacing: None,
            alignment: None,
        })
    }

    /// Validates this qubit timing record.
    pub fn validate(&self) -> Result<(), TimingError> {
        validate_optional_physical_time(self.reset.as_ref())?;
        validate_optional_physical_time(self.measurement.as_ref())?;
        validate_optional_physical_time(self.readout_latency.as_ref())?;
        validate_optional_physical_time(
            self.feed_forward_latency.as_ref(),
        )?;
        validate_optional_physical_time(
            self.minimum_operation_spacing.as_ref(),
        )?;
        validate_optional_physical_time(self.alignment.as_ref())?;

        Ok(())
    }
}

// =============================================================================
// Synchronization
// =============================================================================

/// Synchronization requirement for hardware scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationConstraint {
    /// Minimum required separation.
    pub minimum_separation: TimeValue,

    /// Optional maximum permitted separation.
    pub maximum_separation: Option<TimeValue>,

    /// Whether the boundary must be aligned to the backend clock.
    pub clock_aligned: bool,
}

impl SynchronizationConstraint {
    /// Creates a synchronization constraint.
    pub fn new(
        minimum_separation: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&minimum_separation)?;

        Ok(Self {
            minimum_separation,
            maximum_separation: None,
            clock_aligned: false,
        })
    }

    /// Sets a maximum separation.
    pub fn with_maximum(
        mut self,
        maximum: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&maximum)?;

        let min = self.minimum_separation.to_femtoseconds()?;
        let max = maximum.to_femtoseconds()?;

        if max < min {
            return Err(TimingError::InvalidConstraint {
                reason: "maximum separation is less than minimum separation",
            });
        }

        self.maximum_separation = Some(maximum);
        Ok(self)
    }

    /// Requires clock alignment.
    pub fn clock_aligned(mut self, value: bool) -> Self {
        self.clock_aligned = value;
        self
    }

    /// Validates the constraint.
    pub fn validate(&self) -> Result<(), TimingError> {
        validate_physical_time(&self.minimum_separation)?;

        if let Some(maximum) = &self.maximum_separation {
            validate_physical_time(maximum)?;

            if maximum.to_femtoseconds()?
                < self.minimum_separation.to_femtoseconds()?
            {
                return Err(TimingError::InvalidConstraint {
                    reason: "maximum separation is less than minimum separation",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Timing record
// =============================================================================

/// Complete provider/device-reported timing record.
///
/// This is the primary aggregate for backend-reported timing information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingRecord {
    /// Schema version.
    pub schema_version: u16,

    /// Timing source.
    pub source: TimingSource,

    /// Clock domain.
    pub clock_domain: ClockDomain,

    /// Source resolution.
    pub resolution: Option<TimingResolution>,

    /// Semantic category.
    pub category: TimingCategory,

    /// Measured elapsed duration.
    pub duration: Option<TimeValue>,

    /// Start timestamp, when supplied.
    pub start: Option<Timestamp>,

    /// End timestamp, when supplied.
    pub end: Option<Timestamp>,

    /// Optional provider/device identifier.
    ///
    /// This is intentionally opaque. Hardware identity remains owned by
    /// `identity.rs`.
    pub source_id: Option<String>,
}

impl TimingRecord {
    /// Creates an empty timing record.
    pub fn new(
        source: TimingSource,
        clock_domain: ClockDomain,
        category: TimingCategory,
    ) -> Result<Self, TimingError> {
        source.validate()?;
        clock_domain.validate()?;

        Ok(Self {
            schema_version: HARDWARE_TIMING_SCHEMA_VERSION,
            source,
            clock_domain,
            resolution: None,
            category,
            duration: None,
            start: None,
            end: None,
            source_id: None,
        })
    }

    /// Sets the measured duration.
    pub fn with_duration(mut self, duration: TimeValue) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets source resolution.
    pub fn with_resolution(
        mut self,
        resolution: TimingResolution,
    ) -> Self {
        self.resolution = Some(resolution);
        self
    }

    /// Sets start timestamp.
    pub fn with_start(
        mut self,
        start: Timestamp,
    ) -> Result<Self, TimingError> {
        if start.clock_domain() != &self.clock_domain {
            return Err(TimingError::ClockDomainMismatch {
                start: self.clock_domain.clone(),
                end: start.clock_domain().clone(),
            });
        }

        self.start = Some(start);
        Ok(self)
    }

    /// Sets end timestamp.
    pub fn with_end(
        mut self,
        end: Timestamp,
    ) -> Result<Self, TimingError> {
        if end.clock_domain() != &self.clock_domain {
            return Err(TimingError::ClockDomainMismatch {
                start: self.clock_domain.clone(),
                end: end.clock_domain().clone(),
            });
        }

        self.end = Some(end);
        Ok(self)
    }

    /// Sets an opaque provider/device source identifier.
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Result<Self, TimingError> {
        let source_id = source_id.into();

        if source_id.trim().is_empty() {
            return Err(TimingError::InvalidValue {
                field: "source_id",
                reason: "source identifier must not be empty",
            });
        }

        self.source_id = Some(source_id);
        Ok(self)
    }

    /// Derives duration from start/end timestamps.
    ///
    /// The derived duration is returned in femtoseconds.
    pub fn derive_duration(&self) -> Result<Option<TimeValue>, TimingError> {
        match (&self.start, &self.end) {
            (Some(start), Some(end)) => {
                Ok(Some(end.elapsed_since(start)?))
            }

            _ => Ok(None),
        }
    }

    /// Validates the record.
    pub fn validate(&self) -> Result<(), TimingError> {
        self.source.validate()?;
        self.clock_domain.validate()?;

        if let Some(resolution) = &self.resolution {
            resolution.as_time_value()?;
        }

        if let Some(duration) = &self.duration {
            duration.unit().validate()?;
        }

        if let (Some(start), Some(end)) = (&self.start, &self.end) {
            if start.clock_domain() != end.clock_domain() {
                return Err(TimingError::ClockDomainMismatch {
                    start: start.clock_domain().clone(),
                    end: end.clock_domain().clone(),
                });
            }

            let derived = end.elapsed_since(start)?;

            if let Some(duration) = &self.duration {
                if duration.to_femtoseconds()? != derived.to_femtoseconds()? {
                    return Err(TimingError::InvalidRecord {
                        reason:
                            "explicit duration does not match start/end timestamps",
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Timing provenance
// =============================================================================

/// Reproducibility metadata for a timing measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingProvenance {
    /// Timing source.
    pub source: TimingSource,

    /// Optional opaque source identifier.
    pub source_id: Option<String>,

    /// Optional provider API/version identifier.
    pub provider_version: Option<String>,

    /// Optional firmware version.
    pub firmware_version: Option<String>,

    /// Optional timing schema version.
    pub schema_version: u16,
}

impl TimingProvenance {
    /// Creates provenance information.
    pub fn new(source: TimingSource) -> Result<Self, TimingError> {
        source.validate()?;

        Ok(Self {
            source,
            source_id: None,
            provider_version: None,
            firmware_version: None,
            schema_version: HARDWARE_TIMING_SCHEMA_VERSION,
        })
    }

    /// Sets the opaque source identifier.
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Result<Self, TimingError> {
        let source_id = source_id.into();

        if source_id.trim().is_empty() {
            return Err(TimingError::InvalidValue {
                field: "provenance.source_id",
                reason: "source identifier must not be empty",
            });
        }

        self.source_id = Some(source_id);
        Ok(self)
    }

    /// Sets provider version.
    pub fn with_provider_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.provider_version = Some(version.into());
        self
    }

    /// Sets firmware version.
    pub fn with_firmware_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.firmware_version = Some(version.into());
        self
    }
}

// =============================================================================
// Timing specification
// =============================================================================

/// Hardware-wide timing specification.
///
/// This is an immutable-style data aggregate describing timing constraints
/// without performing scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingSpecification {
    /// Backend/device clock rate, if applicable.
    pub clock_rate: Option<ClockRate>,

    /// Global scheduling alignment.
    pub alignment: Option<TimeValue>,

    /// Classical processing latency.
    pub classical_processing_latency: Option<TimeValue>,

    /// Feed-forward latency.
    pub feed_forward_latency: Option<TimeValue>,

    /// Synchronization latency.
    pub synchronization_latency: Option<TimeValue>,

    /// Per-instruction timing.
    pub instructions: Vec<InstructionTiming>,

    /// Per-qubit timing.
    pub qubits: Vec<QubitTiming>,
}

impl Default for TimingSpecification {
    fn default() -> Self {
        Self {
            clock_rate: None,
            alignment: None,
            classical_processing_latency: None,
            feed_forward_latency: None,
            synchronization_latency: None,
            instructions: Vec::new(),
            qubits: Vec::new(),
        }
    }
}

impl TimingSpecification {
    /// Creates an empty timing specification.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the backend clock rate.
    pub fn with_clock_rate(mut self, rate: ClockRate) -> Self {
        self.clock_rate = Some(rate);
        self
    }

    /// Sets global scheduling alignment.
    pub fn with_alignment(
        mut self,
        alignment: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&alignment)?;

        if alignment.is_zero() {
            return Err(TimingError::InvalidAlignment {
                reason: "alignment must be positive",
            });
        }

        self.alignment = Some(alignment);
        Ok(self)
    }

    /// Sets classical processing latency.
    pub fn with_classical_processing_latency(
        mut self,
        latency: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&latency)?;
        self.classical_processing_latency = Some(latency);
        Ok(self)
    }

    /// Sets feed-forward latency.
    pub fn with_feed_forward_latency(
        mut self,
        latency: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&latency)?;
        self.feed_forward_latency = Some(latency);
        Ok(self)
    }

    /// Sets synchronization latency.
    pub fn with_synchronization_latency(
        mut self,
        latency: TimeValue,
    ) -> Result<Self, TimingError> {
        validate_physical_time(&latency)?;
        self.synchronization_latency = Some(latency);
        Ok(self)
    }

    /// Adds instruction timing.
    pub fn add_instruction(
        &mut self,
        timing: InstructionTiming,
    ) -> Result<(), TimingError> {
        timing.validate()?;
        self.instructions.push(timing);
        Ok(())
    }

    /// Adds qubit timing.
    pub fn add_qubit(
        &mut self,
        timing: QubitTiming,
    ) -> Result<(), TimingError> {
        timing.validate()?;
        self.qubits.push(timing);
        Ok(())
    }

    /// Looks up instruction timing by exact canonical name.
    pub fn instruction(
        &self,
        name: &str,
    ) -> Option<&InstructionTiming> {
        self.instructions
            .iter()
            .find(|timing| timing.instruction == name)
    }

    /// Looks up qubit timing.
    pub fn qubit(&self, qubit: usize) -> Option<&QubitTiming> {
        self.qubits.iter().find(|timing| timing.qubit == qubit)
    }

    /// Validates the entire specification.
    pub fn validate(&self) -> Result<(), TimingError> {
        if let Some(alignment) = &self.alignment {
            validate_physical_time(alignment)?;

            if alignment.is_zero() {
                return Err(TimingError::InvalidAlignment {
                    reason: "global alignment must be positive",
                });
            }
        }

        validate_optional_physical_time(
            self.classical_processing_latency.as_ref(),
        )?;

        validate_optional_physical_time(
            self.feed_forward_latency.as_ref(),
        )?;

        validate_optional_physical_time(
            self.synchronization_latency.as_ref(),
        )?;

        for instruction in &self.instructions {
            instruction.validate()?;
        }

        for qubit in &self.qubits {
            qubit.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Helper validation functions
// =============================================================================

fn validate_physical_time(value: &TimeValue) -> Result<(), TimingError> {
    if !value.unit().is_physical_time() {
        return Err(TimingError::InvalidValue {
            field: "time.unit",
            reason: "a physical time unit is required",
        });
    }

    value.to_femtoseconds()?;
    Ok(())
}

fn validate_optional_physical_time(
    value: Option<&TimeValue>,
) -> Result<(), TimingError> {
    if let Some(value) = value {
        validate_physical_time(value)?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nanoseconds_convert_exactly() {
        let value =
            TimeValue::new(42, TimeUnit::Nanoseconds).expect("valid time");

        assert_eq!(
            value.to_nanoseconds().expect("exact conversion"),
            42
        );

        assert_eq!(
            value
                .to_duration()
                .expect("duration conversion"),
            Duration::from_nanos(42)
        );
    }

    #[test]
    fn_picoseconds_do_not_silently_truncate_to_duration() {
        let value =
            TimeValue::new(1, TimeUnit::Picoseconds).expect("valid time");

        assert!(matches!(
            value.to_duration(),
            Err(TimingError::LossyDurationConversion { .. })
        ));
    }

    #[test]
    fn femtoseconds_do_not_silently_truncate() {
        let value =
            TimeValue::new(999_999, TimeUnit::Femtoseconds)
                .expect("valid time");

        assert!(value.to_duration().is_err());
    }

    #[test]
    fn exact_nanosecond_conversion_from_femtoseconds() {
        let value =
            TimeValue::new(1_000_000, TimeUnit::Femtoseconds)
                .expect("valid time");

        assert_eq!(
            value.to_duration().expect("exact duration"),
            Duration::from_nanos(1)
        );
    }

    #[test]
    fn cycles_require_clock_rate() {
        let value =
            TimeValue::new(10, TimeUnit::Cycles).expect("valid cycles");

        assert!(value.to_femtoseconds().is_err());
    }

    #[test]
    fn cycles_can_use_clock_rate() {
        let rate =
            ClockRate::from_hz(1_000_000_000).expect("valid rate");

        let value =
            TimeValue::new(1, TimeUnit::Cycles).expect("valid cycles");

        assert_eq!(
            value
                .cycles_to_femtoseconds(rate)
                .expect("exact conversion"),
            1_000_000_000_000
        );
    }

    #[test]
    fn zero_is_explicit() {
        let value =
            TimeValue::zero(TimeUnit::Nanoseconds).expect("valid zero");

        assert!(value.is_zero());
    }

    #[test]
    fn custom_unit_must_not_be_empty() {
        let result = TimeValue::new(
            1,
            TimeUnit::Custom(String::new()),
        );

        assert!(matches!(
            result,
            Err(TimingError::EmptyCustomUnit)
        ));
    }

    #[test]
    fn custom_clock_must_not_be_empty() {
        let result = Timestamp::new(
            1,
            TimeUnit::Nanoseconds,
            ClockDomain::Custom(String::new()),
        );

        assert!(matches!(
            result,
            Err(TimingError::EmptyCustomClock)
        ));
    }

    #[test]
    fn timestamps_from_different_domains_cannot_be_subtracted() {
        let start = Timestamp::new(
            10,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = Timestamp::new(
            20,
            TimeUnit::Nanoseconds,
            ClockDomain::HostMonotonic,
        )
        .expect("valid timestamp");

        assert!(matches!(
            end.elapsed_since(&start),
            Err(TimingError::ClockDomainMismatch { .. })
        ));
    }

    #[test]
    fn timestamp_elapsed_time_is_exact() {
        let start = Timestamp::new(
            100,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = Timestamp::new(
            150,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let elapsed =
            end.elapsed_since(&start).expect("valid elapsed time");

        assert_eq!(elapsed.value(), 50_000_000);
        assert_eq!(elapsed.unit(), &TimeUnit::Femtoseconds);
    }

    #[test]
    fn backwards_timestamp_is_rejected() {
        let start = Timestamp::new(
            200,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = Timestamp::new(
            100,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        assert!(matches!(
            end.elapsed_since(&start),
            Err(TimingError::EndBeforeStart { .. })
        ));
    }

    #[test]
    fn timing_record_can_derive_duration() {
        let record = TimingRecord::new(
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
            TimingCategory::Execution,
        )
        .expect("valid record")
        .with_start(
            Timestamp::new(
                1_000,
                TimeUnit::Nanoseconds,
                ClockDomain::DeviceMonotonic,
            )
            .expect("valid start"),
        )
        .expect("compatible clock")
        .with_end(
            Timestamp::new(
                1_500,
                TimeUnit::Nanoseconds,
                ClockDomain::DeviceMonotonic,
            )
            .expect("valid end"),
        )
        .expect("compatible clock");

        let duration =
            record.derive_duration().expect("valid derivation");

        assert_eq!(
            duration.expect("duration").value(),
            500_000_000
        );
    }

    #[test]
    fn timing_record_rejects_inconsistent_explicit_duration() {
        let record = TimingRecord::new(
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
            TimingCategory::Execution,
        )
        .expect("valid record")
        .with_duration(
            TimeValue::new(20, TimeUnit::Nanoseconds)
                .expect("valid duration"),
        )
        .with_start(
            Timestamp::new(
                1_000,
                TimeUnit::Nanoseconds,
                ClockDomain::DeviceMonotonic,
            )
            .expect("valid start"),
        )
        .expect("compatible clock")
        .with_end(
            Timestamp::new(
                1_050,
                TimeUnit::Nanoseconds,
                ClockDomain::DeviceMonotonic,
            )
            .expect("valid end"),
        )
        .expect("compatible clock");

        assert!(record.validate().is_err());
    }

    #[test]
    fn instruction_timing_rejects_invalid_range() {
        let instruction = InstructionTiming::new("cx")
            .expect("valid instruction")
            .with_minimum_duration(
                TimeValue::new(20, TimeUnit::Nanoseconds)
                    .expect("valid duration"),
            )
            .expect("valid minimum");

        let result = instruction.with_maximum_duration(
            TimeValue::new(10, TimeUnit::Nanoseconds)
                .expect("valid duration"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn instruction_timing_accepts_valid_range() {
        let instruction = InstructionTiming::new("cx")
            .expect("valid instruction")
            .with_minimum_duration(
                TimeValue::new(10, TimeUnit::Nanoseconds)
                    .expect("valid duration"),
            )
            .expect("valid minimum")
            .with_maximum_duration(
                TimeValue::new(20, TimeUnit::Nanoseconds)
                    .expect("valid duration"),
            )
            .expect("valid maximum");

        assert!(instruction.validate().is_ok());
    }

    #[test]
    fn qubit_timing_validates() {
        let mut timing =
            QubitTiming::new(0).expect("valid qubit");

        timing.reset = Some(
            TimeValue::new(100, TimeUnit::Nanoseconds)
                .expect("valid duration"),
        );

        timing.measurement = Some(
            TimeValue::new(500, TimeUnit::Nanoseconds)
                .expect("valid duration"),
        );

        assert!(timing.validate().is_ok());
    }

    #[test]
    fn synchronization_constraint_rejects_reversed_range() {
        let constraint = SynchronizationConstraint::new(
            TimeValue::new(20, TimeUnit::Nanoseconds)
                .expect("valid minimum"),
        )
        .expect("valid constraint")
        .with_maximum(
            TimeValue::new(10, TimeUnit::Nanoseconds)
                .expect("valid maximum"),
        );

        assert!(constraint.is_err());
    }

    #[test]
    fn synchronization_constraint_accepts_valid_range() {
        let constraint = SynchronizationConstraint::new(
            TimeValue::new(10, TimeUnit::Nanoseconds)
                .expect("valid minimum"),
        )
        .expect("valid constraint")
        .with_maximum(
            TimeValue::new(20, TimeUnit::Nanoseconds)
                .expect("valid maximum"),
        )
        .expect("valid maximum");

        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn timing_resolution_must_be_positive() {
        let result =
            TimingResolution::new(0, TimeUnit::Nanoseconds);

        assert!(result.is_err());
    }

    #[test]
    fn timing_source_is_preserved() {
        let interval = TimingInterval::new(
            TimeValue::new(10, TimeUnit::Nanoseconds)
                .expect("valid duration"),
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
            None,
        )
        .expect("valid interval");

        assert_eq!(interval.source(), &TimingSource::Device);
        assert_eq!(
            interval.clock_domain(),
            &ClockDomain::DeviceMonotonic
        );
    }

    #[test]
    fn timing_specification_can_be_built_independently() {
        let mut specification =
            TimingSpecification::new().with_clock_rate(
                ClockRate::from_hz(5_000_000_000)
                    .expect("valid clock"),
            );

        specification
            .add_instruction(
                InstructionTiming::new("cx")
                    .expect("valid instruction")
                    .with_duration(
                        TimeValue::new(
                            300,
                            TimeUnit::Nanoseconds,
                        )
                        .expect("valid duration"),
                    )
                    .deterministic(true),
            )
            .expect("valid instruction timing");

        specification
            .add_qubit(
                QubitTiming::new(0)
                    .expect("valid qubit"),
            )
            .expect("valid qubit timing");

        assert!(specification.validate().is_ok());
    }

    #[test]
    fn clock_period_for_one_ghz_is_one_ns() {
        let rate =
            ClockRate::from_hz(1_000_000_000)
                .expect("valid clock");

        let period = rate
            .period_femtoseconds()
            .expect("exact period");

        assert_eq!(
            period.value(),
            1_000_000
        );

        assert_eq!(
            period.unit(),
            &TimeUnit::Femtoseconds
        );
    }

    #[test]
    fn source_id_cannot_be_empty() {
        let result = TimingRecord::new(
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
            TimingCategory::Execution,
        )
        .expect("valid record")
        .with_source_id("   ");

        assert!(result.is_err());
    }
}