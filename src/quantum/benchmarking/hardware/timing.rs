//! Zamani Quantum Benchmarking — Hardware Timing Contract
//!
//! Production-grade backend timing representation and normalization for the
//! quantum benchmarking subsystem.
//!
//! # Responsibility
//!
//! This module defines the timing information that a concrete quantum backend,
//! simulator, emulator, provider, or hardware adapter may report to the
//! benchmarking subsystem.
//!
//! It deliberately does NOT:
//!
//! - measure host-side elapsed time;
//! - execute quantum circuits;
//! - submit jobs;
//! - communicate with providers;
//! - calculate throughput;
//! - calculate fidelity;
//! - calculate benchmark scores;
//! - calculate Quantum Volume;
//! - perform statistical analysis;
//! - own benchmark lifecycle orchestration.
//!
//! Host-side lifecycle timing is owned by:
//!
//! ```text
//! quantum::benchmarking::execution::timing
//! ```
//!
//! Backend-reported timing is owned by this module.
//!
//! The distinction is essential.
//!
//! ```text
//!                    EXECUTION LIFECYCLE
//!
//! host/process clock ───────► execution::timing
//!                                  │
//!                                  │
//!                                  ▼
//!                         host-observed timing
//!
//!
//!                    BACKEND / PROVIDER TIMING
//!
//! hardware/provider ───────► hardware::timing
//!                                  │
//!                                  │
//!                                  ▼
//!                         backend-reported timing
//! ```
//!
//! These measurements MUST NOT be silently mixed.
//!
//! # Why this layer exists
//!
//! A quantum backend may report timing that is fundamentally different from
//! the wall-clock time observed by the Zamani process.
//!
//! For example:
//!
//! ```text
//! submit request
//!       │
//!       ├── provider queue:       2.1 s
//!       ├── device execution:    8.4 ms
//!       ├── readout:             1.2 ms
//!       └── result transfer:     4.7 ms
//!
//! host wall-clock:
//!       2.118 s
//! ```
//!
//! The benchmark must preserve those values independently.
//!
//! A provider may also report device-native timing in:
//!
//! - nanoseconds;
//! - picoseconds;
//! - clock cycles;
//! - sample periods;
//! - hardware ticks;
//! - provider-defined units.
//!
//! This module normalizes supported physical time units while retaining the
//! original source/unit metadata.
//!
//! # Clock semantics
//!
//! Timing values have an explicit clock domain.
//!
//! This prevents an invalid operation such as subtracting timestamps from two
//! unrelated clocks.
//!
//! Supported clock domains include:
//!
//! - host monotonic clock;
//! - host wall clock;
//! - device monotonic clock;
//! - provider monotonic clock;
//! - backend simulation clock;
//! - external clock;
//! - unknown/custom clock.
//!
//! A timestamp from one clock domain MUST NOT be directly subtracted from a
//! timestamp from another clock domain.
//!
//! # Absolute versus elapsed time
//!
//! `Duration` is used for elapsed timing whenever possible.
//!
//! Absolute timestamps are retained only as provenance metadata.
//!
//! The following rule is mandatory:
//!
//! ```text
//! elapsed duration → monotonic/controlled duration source
//! wall timestamp    → provenance only
//! ```
//!
//! Wall-clock adjustments, NTP corrections, VM clock changes, leap-second
//! handling, or provider timestamp synchronization MUST NOT be allowed to
//! corrupt elapsed-time calculations.
//!
//! # Missing versus zero
//!
//! `None` means:
//!
//!     timing was not measured, not supplied, or unavailable.
//!
//! `Some(Duration::ZERO)` means:
//!
//!     timing was explicitly measured and reported as zero.
//!
//! These states are semantically different.
//!
//! # Backend neutrality
//!
//! The representation supports:
//!
//! - superconducting QPUs;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin/semiconductor systems;
//! - topological systems;
//! - analog systems;
//! - annealers;
//! - simulators;
//! - emulators;
//! - logical/fault-tolerant systems;
//! - hybrid quantum-classical systems.
//!
//! It therefore avoids assuming that every backend has a conventional
//! "circuit execution time".
//!
//! # Integration contract
//!
//! This module is intentionally independent of the rest of the benchmarking
//! subsystem.
//!
//! The dependency direction is:
//!
//! ```text
//! hardware::timing
//!        │
//!        ├──────────────► execution::timing
//!        │
//!        ├──────────────► execution::response
//!        │
//!        └──────────────► metrics::runtime / throughput
//! ```
//!
//! The reverse dependency is forbidden:
//!
//! ```text
//! hardware::timing ─X─► execution::executor
//! hardware::timing ─X─► benchmark protocols
//! ```
//!
//! In particular, this file must not import:
//!
//! ```text
//! execution::executor
//! protocols::*
//! metrics::*
//! quantum::ir
//! ```
//!
//! This keeps it independently implementable and testable.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! No external dependency is required.
//!
//! # Production invariants
//!
//! 1. No floating-point value is used as the canonical duration representation.
//! 2. Duration values use `std::time::Duration`.
//! 3. Provider units must be validated before conversion.
//! 4. Integer overflow during unit conversion must be detected.
//! 5. Negative elapsed durations are rejected.
//! 6. Unknown units are never silently interpreted.
//! 7. Clock domains must be preserved.
//! 8. Provider timing must remain distinguishable from host timing.
//! 9. Missing timing must remain distinguishable from zero timing.
//! 10. Provider-reported timing must never overwrite host-observed timing.
//! 11. Provider metadata must be retained for reproducibility.
//! 12. Timing values must remain independent of benchmark semantics.
//! 13. No I/O is performed by this module.
//! 14. No logging or printing is performed by this module.
//! 15. Malformed backend timing must produce a structured error.
//! 16. Conversions must be deterministic.
//! 17. Canonical nanosecond precision must not imply that the source actually
//!     measured nanoseconds; source resolution is retained separately.
//! 18. A backend may provide only a subset of timing fields.
//! 19. A backend must never be forced to fabricate timing information.
//! 20. Provider timestamps from unrelated clock domains must not be compared.
//!
//! # Scientific interpretation
//!
//! This module records timing. It does not decide what timing means for a
//! benchmark.
//!
//! For example:
//!
//! ```text
//! hardware execution duration
//!             │
//!             ▼
//! hardware::timing
//!             │
//!             ▼
//! metrics::runtime
//!             │
//!             ▼
//! benchmark-specific interpretation
//! ```
//!
//! This distinction is necessary because a QPU execution duration, a queue
//! duration, a circuit-layer operation rate, and an end-to-end time-to-solution
//! are different scientific quantities.

use std::fmt;
use std::time::Duration;

// =============================================================================
// Schema version
// =============================================================================

/// Stable schema version for backend timing information.
///
/// Increment this only when the semantic meaning or serialized structure of
/// the timing contract changes incompatibly.
pub const HARDWARE_TIMING_SCHEMA_VERSION: u16 = 1;

/// Stable API version for this timing module.
pub const HARDWARE_TIMING_API_VERSION: u16 = 1;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by backend timing validation or unit conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingError {
    /// A negative duration was supplied.
    NegativeDuration {
        /// Human-readable field identifying the invalid value.
        field: &'static str,
    },

    /// The supplied timing value cannot be represented using the selected
    /// unit.
    InvalidValue {
        /// Field containing the invalid value.
        field: &'static str,
        /// Explanation of the validation failure.
        reason: &'static str,
    },

    /// Integer conversion would overflow.
    ConversionOverflow {
        /// Source unit.
        source: TimeUnit,
        /// Target unit.
        target: TimeUnit,
    },

    /// A timestamp pair belongs to different clock domains.
    ClockDomainMismatch {
        /// First clock domain.
        start: ClockDomain,
        /// Second clock domain.
        end: ClockDomain,
    },

    /// An end timestamp occurs before its start timestamp.
    EndBeforeStart {
        /// Timing field being validated.
        field: &'static str,
    },

    /// A required source resolution is unavailable.
    MissingResolution,

    /// A custom unit identifier was empty.
    EmptyCustomUnit,

    /// A custom clock identifier was empty.
    EmptyCustomClock,

    /// A provider supplied a timing record that is structurally invalid.
    InvalidRecord {
        /// Explanation of the invalid record.
        reason: &'static str,
    },
}

impl fmt::Display for TimingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeDuration { field } => {
                write!(f, "negative duration for timing field '{field}'")
            }

            Self::InvalidValue { field, reason } => {
                write!(f, "invalid timing value for '{field}': {reason}")
            }

            Self::ConversionOverflow { source, target } => {
                write!(
                    f,
                    "timing conversion overflow from {source} to {target}"
                )
            }

            Self::ClockDomainMismatch { start, end } => {
                write!(
                    f,
                    "cannot compare timestamps from different clock domains: \
                     {start} versus {end}"
                )
            }

            Self::EndBeforeStart { field } => {
                write!(f, "end timestamp precedes start timestamp for '{field}'")
            }

            Self::MissingResolution => {
                f.write_str("timing source resolution is missing")
            }

            Self::EmptyCustomUnit => {
                f.write_str("custom timing unit identifier must not be empty")
            }

            Self::EmptyCustomClock => {
                f.write_str("custom clock identifier must not be empty")
            }

            Self::InvalidRecord { reason } => {
                write!(f, "invalid backend timing record: {reason}")
            }
        }
    }
}

impl std::error::Error for TimingError {}

// =============================================================================
// Time unit
// =============================================================================

/// Unit used by a backend when reporting an elapsed duration.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeUnit {
    /// One second.
    Seconds,

    /// One millisecond.
    Milliseconds,

    /// One microsecond.
    Microseconds,

    /// One nanosecond.
    Nanoseconds,

    /// One picosecond.
    Picoseconds,

    /// One femtosecond.
    Femtoseconds,

    /// Backend clock cycles.
    Cycles,

    /// Backend/provider-defined unit.
    Custom(String),
}

impl TimeUnit {
    /// Returns a stable machine-readable name.
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

    /// Returns whether this unit represents an SI physical time unit.
    pub const fn is_physical_time(self: &Self) -> bool {
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

    /// Validates the unit.
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

/// Clock domain from which a timing value or timestamp originated.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClockDomain {
    /// Host process monotonic clock.
    HostMonotonic,

    /// Host system wall clock.
    HostWallClock,

    /// Quantum-device monotonic clock.
    DeviceMonotonic,

    /// Provider-controlled monotonic clock.
    ProviderMonotonic,

    /// Backend simulator's logical clock.
    Simulation,

    /// External synchronized clock.
    External,

    /// User-defined clock domain.
    Custom(String),

    /// Clock source was not supplied.
    Unknown,
}

impl ClockDomain {
    /// Returns a stable machine-readable identifier.
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

    /// Returns whether elapsed subtraction between two timestamps in this
    /// domain is meaningful without an additional synchronization operation.
    pub const fn supports_elapsed_difference(&self) -> bool {
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

/// Origin of a timing measurement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimingSource {
    /// Measured directly by the Zamani host process.
    Host,

    /// Reported by the quantum device.
    Device,

    /// Reported by a cloud/provider service.
    Provider,

    /// Generated by a simulator.
    Simulator,

    /// Generated by an emulator/digital twin.
    Emulator,

    /// Supplied by an external measurement system.
    External,

    /// User-defined source.
    Custom(String),
}

impl TimingSource {
    /// Stable source identifier.
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
}

impl fmt::Display for TimingSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Timing resolution
// =============================================================================

/// Resolution of a backend timing source.
///
/// Resolution is metadata about what the backend can actually distinguish.
/// It must not be inferred from the normalized representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingResolution {
    /// Smallest meaningful increment.
    pub value: u64,

    /// Unit of the increment.
    pub unit: TimeUnit,
}

impl TimingResolution {
    /// Creates a timing resolution.
    pub fn new(value: u64, unit: TimeUnit) -> Result<Self, TimingError> {
        if value == 0 {
            return Err(TimingError::InvalidValue {
                field: "resolution.value",
                reason: "resolution must be greater than zero",
            });
        }

        unit.validate()?;

        Ok(Self { value, unit })
    }
}

// =============================================================================
// Raw timing value
// =============================================================================

/// Raw backend timing measurement before normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTiming {
    /// Non-negative magnitude reported by the backend.
    pub value: u64,

    /// Unit used by the backend.
    pub unit: TimeUnit,

    /// Source that produced the measurement.
    pub source: TimingSource,

    /// Clock domain associated with the measurement.
    pub clock: ClockDomain,

    /// Reported source resolution, if known.
    pub resolution: Option<TimingResolution>,
}

impl RawTiming {
    /// Creates a raw timing value.
    pub fn new(
        value: u64,
        unit: TimeUnit,
        source: TimingSource,
        clock: ClockDomain,
    ) -> Result<Self, TimingError> {
        unit.validate()?;
        clock.validate()?;

        Ok(Self {
            value,
            unit,
            source,
            clock,
            resolution: None,
        })
    }

    /// Adds source-resolution metadata.
    pub fn with_resolution(
        mut self,
        resolution: TimingResolution,
    ) -> Result<Self, TimingError> {
        resolution.unit.validate()?;

        self.resolution = Some(resolution);

        Ok(self)
    }

    /// Converts a physical time unit into canonical `Duration`.
    ///
    /// Clock-cycle/custom units cannot be converted without additional
    /// calibration information and therefore return an error.
    pub fn to_duration(&self) -> Result<Duration, TimingError> {
        self.unit.validate()?;

        match self.unit {
            TimeUnit::Seconds => Duration::from_secs(self.value),

            TimeUnit::Milliseconds => Duration::from_millis(self.value),

            TimeUnit::Microseconds => Duration::from_micros(self.value),

            TimeUnit::Nanoseconds => Duration::from_nanos(self.value),

            TimeUnit::Picoseconds => {
                // Duration has nanosecond precision. Preserve sub-nanosecond
                // information by rounding down only after a checked conversion.
                let nanos = self.value / 1_000;

                if nanos > u64::MAX as u64 {
                    return Err(TimingError::ConversionOverflow {
                        source: TimeUnit::Picoseconds,
                        target: TimeUnit::Nanoseconds,
                    });
                }

                Ok(Duration::from_nanos(nanos))
            }

            TimeUnit::Femtoseconds => {
                let nanos = self.value / 1_000_000;

                Ok(Duration::from_nanos(nanos))
            }

            TimeUnit::Cycles => Err(TimingError::InvalidValue {
                field: "value",
                reason: "clock cycles require a clock frequency before conversion",
            }),

            TimeUnit::Custom(_) => Err(TimingError::InvalidValue {
                field: "value",
                reason: "custom timing units require an explicit conversion",
            }),
        }
    }
}

// =============================================================================
// Clock frequency
// =============================================================================

/// Frequency information required to convert backend clock cycles into time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockFrequency {
    /// Frequency in hertz.
    pub hz: u64,
}

impl ClockFrequency {
    /// Creates a validated clock frequency.
    pub fn new(hz: u64) -> Result<Self, TimingError> {
        if hz == 0 {
            return Err(TimingError::InvalidValue {
                field: "frequency.hz",
                reason: "clock frequency must be greater than zero",
            });
        }

        Ok(Self { hz })
    }

    /// Converts a cycle count into a `Duration`.
    ///
    /// Conversion uses integer arithmetic and deliberately rounds down to
    /// nanosecond precision. The original cycle count remains available in
    /// the raw measurement.
    pub fn cycles_to_duration(self, cycles: u64) -> Result<Duration, TimingError> {
        if self.hz == 0 {
            return Err(TimingError::InvalidValue {
                field: "frequency.hz",
                reason: "clock frequency must be greater than zero",
            });
        }

        let seconds = cycles / self.hz;
        let remainder = cycles % self.hz;

        let nanos = ((remainder as u128) * 1_000_000_000u128) / self.hz as u128;

        if nanos > u32::MAX as u128 {
            return Err(TimingError::ConversionOverflow {
                source: TimeUnit::Cycles,
                target: TimeUnit::Nanoseconds,
            });
        }

        Ok(Duration::new(seconds, nanos as u32))
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Backend timestamp.
///
/// Timestamps are retained for provenance and same-clock interval calculation.
/// They are not interchangeable with host wall-clock timestamps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackendTimestamp {
    /// Timestamp value.
    pub value: u64,

    /// Unit of the timestamp.
    pub unit: TimeUnit,

    /// Clock domain.
    pub clock: ClockDomain,
}

impl BackendTimestamp {
    /// Creates a backend timestamp.
    pub fn new(
        value: u64,
        unit: TimeUnit,
        clock: ClockDomain,
    ) -> Result<Self, TimingError> {
        unit.validate()?;
        clock.validate()?;

        Ok(Self {
            value,
            unit,
            clock,
        })
    }
}

// =============================================================================
// Timing interval
// =============================================================================

/// A backend-reported interval between two timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingInterval {
    /// Start timestamp.
    pub start: BackendTimestamp,

    /// End timestamp.
    pub end: BackendTimestamp,
}

impl TimingInterval {
    /// Creates a timing interval after validating the clock domains.
    pub fn new(
        start: BackendTimestamp,
        end: BackendTimestamp,
    ) -> Result<Self, TimingError> {
        if start.clock != end.clock {
            return Err(TimingError::ClockDomainMismatch {
                start: start.clock,
                end: end.clock,
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the interval in canonical `Duration` where both timestamps use
    /// a physical time unit.
    ///
    /// Clock-cycle/custom timestamps must be normalized by the backend before
    /// this method can be used.
    pub fn duration(&self) -> Result<Duration, TimingError> {
        if self.start.unit != self.end.unit {
            return Err(TimingError::InvalidRecord {
                reason: "start and end timestamps use different units",
            });
        }

        if !self.start.unit.is_physical_time() {
            return Err(TimingError::InvalidValue {
                field: "interval",
                reason: "non-physical timestamp units require backend normalization",
            });
        }

        if self.end.value < self.start.value {
            return Err(TimingError::EndBeforeStart {
                field: "interval",
            });
        }

        let delta = self.end.value - self.start.value;

        RawTiming::new(
            delta,
            self.start.unit.clone(),
            TimingSource::Device,
            self.start.clock.clone(),
        )?
        .to_duration()
    }
}

// =============================================================================
// Hardware timing field
// =============================================================================

/// A named backend timing quantity.
///
/// The set of standard fields below covers common gate-model and provider
/// execution lifecycles while allowing custom backend timing fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingField {
    /// Provider queue waiting time.
    Queue,

    /// Provider request submission time.
    Submission,

    /// Backend-side compilation time.
    Compilation,

    /// Backend transpilation time.
    Transpilation,

    /// Backend routing time.
    Routing,

    /// Backend scheduling time.
    Scheduling,

    /// Device execution time.
    Execution,

    /// Device readout time.
    Readout,

    /// Result packaging time.
    ResultPackaging,

    /// Result transfer/retrieval time.
    ResultRetrieval,

    /// Pulse generation time.
    PulseGeneration,

    /// Analog evolution time.
    AnalogEvolution,

    /// Annealing evolution time.
    Annealing,

    /// Syndrome extraction time.
    SyndromeExtraction,

    /// Decoder time.
    Decoding,

    /// Logical operation time.
    LogicalOperation,

    /// Total backend-reported duration.
    BackendTotal,

    /// Backend-specific custom field.
    Custom(String),
}

impl TimingField {
    /// Stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queue => "queue",
            Self::Submission => "submission",
            Self::Compilation => "compilation",
            Self::Transpilation => "transpilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Execution => "execution",
            Self::Readout => "readout",
            Self::ResultPackaging => "result_packaging",
            Self::ResultRetrieval => "result_retrieval",
            Self::PulseGeneration => "pulse_generation",
            Self::AnalogEvolution => "analog_evolution",
            Self::Annealing => "annealing",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::Decoding => "decoding",
            Self::LogicalOperation => "logical_operation",
            Self::BackendTotal => "backend_total",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Validates a timing field.
    pub fn validate(&self) -> Result<(), TimingError> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(TimingError::InvalidValue {
                    field: "timing_field",
                    reason: "custom timing field must not be empty",
                });
            }
        }

        Ok(())
    }
}

impl fmt::Display for TimingField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Normalized timing measurement
// =============================================================================

/// Canonical backend timing measurement.
///
/// `duration` is the normalized value used by the rest of benchmarking.
/// `raw` preserves the original backend measurement and provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareTimingMeasurement {
    /// Timing field.
    pub field: TimingField,

    /// Canonical elapsed duration.
    pub duration: Duration,

    /// Original backend timing.
    pub raw: RawTiming,

    /// Optional source timestamp interval.
    pub interval: Option<TimingInterval>,
}

impl HardwareTimingMeasurement {
    /// Creates a measurement from a raw physical-time value.
    pub fn from_raw(
        field: TimingField,
        raw: RawTiming,
    ) -> Result<Self, TimingError> {
        field.validate()?;

        let duration = raw.to_duration()?;

        Ok(Self {
            field,
            duration,
            raw,
            interval: None,
        })
    }

    /// Attaches a timestamp interval.
    pub fn with_interval(
        mut self,
        interval: TimingInterval,
    ) -> Result<Self, TimingError> {
        let interval_duration = interval.duration()?;

        if interval_duration != self.duration {
            return Err(TimingError::InvalidRecord {
                reason: "reported duration does not match timestamp interval",
            });
        }

        self.interval = Some(interval);

        Ok(self)
    }
}

// =============================================================================
// Backend timing record
// =============================================================================

/// Complete backend timing record for one execution.
///
/// This is deliberately independent from host lifecycle timing.
///
/// It can be attached to an execution response without changing the meaning
/// of the host-observed `ExecutionTiming`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendTimingRecord {
    /// Stable schema version.
    pub schema_version: u16,

    /// Backend/provider identifier.
    pub backend_id: String,

    /// Optional backend version.
    pub backend_version: Option<String>,

    /// Timing measurements reported by the backend.
    pub measurements: Vec<HardwareTimingMeasurement>,

    /// Optional backend total.
    ///
    /// This is independent from the host-observed total wall time.
    pub backend_total: Option<HardwareTimingMeasurement>,

    /// Optional backend clock frequency used by cycle-based measurements.
    pub clock_frequency: Option<ClockFrequency>,

    /// Backend timing source.
    pub source: TimingSource,

    /// Primary backend clock domain.
    pub clock: ClockDomain,

    /// Whether the timing values were directly measured or provider-estimated.
    pub quality: TimingQuality,
}

impl BackendTimingRecord {
    /// Creates an empty timing record.
    pub fn new(
        backend_id: impl Into<String>,
        source: TimingSource,
        clock: ClockDomain,
    ) -> Result<Self, TimingError> {
        let backend_id = backend_id.into();

        if backend_id.trim().is_empty() {
            return Err(TimingError::InvalidRecord {
                reason: "backend_id must not be empty",
            });
        }

        source.as_str();

        clock.validate()?;

        Ok(Self {
            schema_version: HARDWARE_TIMING_SCHEMA_VERSION,
            backend_id,
            backend_version: None,
            measurements: Vec::new(),
            backend_total: None,
            clock_frequency: None,
            source,
            clock,
            quality: TimingQuality::Measured,
        })
    }

    /// Adds a backend version.
    pub fn with_backend_version(mut self, version: impl Into<String>) -> Self {
        self.backend_version = Some(version.into());
        self
    }

    /// Adds a clock frequency.
    pub fn with_clock_frequency(
        mut self,
        frequency: ClockFrequency,
    ) -> Self {
        self.clock_frequency = Some(frequency);
        self
    }

    /// Sets timing quality.
    pub fn with_quality(mut self, quality: TimingQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Adds a normalized timing measurement.
    ///
    /// Duplicate fields are rejected. This prevents silently replacing an
    /// earlier provider measurement.
    pub fn add_measurement(
        &mut self,
        measurement: HardwareTimingMeasurement,
    ) -> Result<(), TimingError> {
        if measurement.raw.source != self.source {
            return Err(TimingError::InvalidRecord {
                reason: "measurement source does not match timing-record source",
            });
        }

        if measurement.raw.clock != self.clock {
            return Err(TimingError::InvalidRecord {
                reason: "measurement clock does not match timing-record clock",
            });
        }

        if self
            .measurements
            .iter()
            .any(|existing| existing.field == measurement.field)
        {
            return Err(TimingError::InvalidRecord {
                reason: "duplicate timing field",
            });
        }

        self.measurements.push(measurement);

        Ok(())
    }

    /// Sets the backend-reported total.
    pub fn set_backend_total(
        &mut self,
        measurement: HardwareTimingMeasurement,
    ) -> Result<(), TimingError> {
        if measurement.field != TimingField::BackendTotal {
            return Err(TimingError::InvalidRecord {
                reason: "backend total measurement must use TimingField::BackendTotal",
            });
        }

        if measurement.raw.source != self.source {
            return Err(TimingError::InvalidRecord {
                reason: "backend total source does not match timing-record source",
            });
        }

        if measurement.raw.clock != self.clock {
            return Err(TimingError::InvalidRecord {
                reason: "backend total clock does not match timing-record clock",
            });
        }

        self.backend_total = Some(measurement);

        Ok(())
    }

    /// Finds a measurement by field.
    pub fn get(&self, field: &TimingField) -> Option<&HardwareTimingMeasurement> {
        self.measurements
            .iter()
            .find(|measurement| &measurement.field == field)
    }

    /// Returns the number of timing measurements.
    pub fn measurement_count(&self) -> usize {
        self.measurements.len()
    }

    /// Returns whether a particular field is available.
    pub fn contains(&self, field: &TimingField) -> bool {
        self.get(field).is_some()
    }

    /// Returns the sum of explicitly reported backend phases.
    ///
    /// This is informational. It must not be assumed to equal
    /// `backend_total` because backend operations can overlap.
    pub fn measured_phase_time(&self) -> Duration {
        self.measurements
            .iter()
            .fold(Duration::ZERO, |total, measurement| {
                total.saturating_add(measurement.duration)
            })
    }

    /// Returns whether the record has a backend total.
    pub fn has_backend_total(&self) -> bool {
        self.backend_total.is_some()
    }

    /// Validates the complete record.
    pub fn validate(&self) -> Result<(), TimingError> {
        if self.schema_version == 0 {
            return Err(TimingError::InvalidRecord {
                reason: "schema_version must be non-zero",
            });
        }

        if self.backend_id.trim().is_empty() {
            return Err(TimingError::InvalidRecord {
                reason: "backend_id must not be empty",
            });
        }

        self.clock.validate()?;

        for measurement in &self.measurements {
            measurement.field.validate()?;

            if measurement.raw.source != self.source {
                return Err(TimingError::InvalidRecord {
                    reason: "measurement source mismatch",
                });
            }

            if measurement.raw.clock != self.clock {
                return Err(TimingError::InvalidRecord {
                    reason: "measurement clock mismatch",
                });
            }
        }

        if let Some(total) = &self.backend_total {
            if total.raw.source != self.source {
                return Err(TimingError::InvalidRecord {
                    reason: "backend total source mismatch",
                });
            }

            if total.raw.clock != self.clock {
                return Err(TimingError::InvalidRecord {
                    reason: "backend total clock mismatch",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Timing quality
// =============================================================================

/// Scientific quality classification of a timing measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingQuality {
    /// Directly measured by the timing source.
    Measured,

    /// Hardware/provider supplied a calibrated estimate.
    Estimated,

    /// Timing was derived from other backend values.
    Derived,

    /// Timing is simulated.
    Simulated,

    /// Timing is emulated.
    Emulated,

    /// Timing source quality is unknown.
    Unknown,
}

impl TimingQuality {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Estimated => "estimated",
            Self::Derived => "derived",
            Self::Simulated => "simulated",
            Self::Emulated => "emulated",
            Self::Unknown => "unknown",
        }
    }
}

// =============================================================================
// Canonical phase mapping
// =============================================================================

/// Maps a backend timing field into the canonical execution timing phase when
/// such a mapping exists.
///
/// Not every hardware timing quantity maps one-to-one to the execution timing
/// lifecycle. For example, pulse generation may be part of backend execution
/// rather than a distinct host lifecycle phase.
pub const fn canonical_phase(field: &TimingField) -> Option<CanonicalTimingPhase> {
    match field {
        TimingField::Queue => Some(CanonicalTimingPhase::Queue),

        TimingField::Submission => Some(CanonicalTimingPhase::Submission),

        TimingField::Compilation => Some(CanonicalTimingPhase::Compilation),

        TimingField::Transpilation => Some(CanonicalTimingPhase::Transpilation),

        TimingField::Routing => Some(CanonicalTimingPhase::Routing),

        TimingField::Scheduling => Some(CanonicalTimingPhase::Scheduling),

        TimingField::Execution
        | TimingField::PulseGeneration
        | TimingField::AnalogEvolution
        | TimingField::Annealing
        | TimingField::LogicalOperation => Some(CanonicalTimingPhase::Execution),

        TimingField::Readout | TimingField::SyndromeExtraction => {
            Some(CanonicalTimingPhase::Readout)
        }

        TimingField::ResultRetrieval | TimingField::ResultPackaging => {
            Some(CanonicalTimingPhase::ResultRetrieval)
        }

        TimingField::Decoding => None,

        TimingField::BackendTotal => None,

        TimingField::Custom(_) => None,
    }
}

/// Canonical lifecycle phase represented by a backend timing measurement.
///
/// This intentionally mirrors the stable semantic phases from
/// `execution::timing` without importing that module, preventing a dependency
/// cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanonicalTimingPhase {
    /// Compilation.
    Compilation,

    /// Transpilation.
    Transpilation,

    /// Routing.
    Routing,

    /// Scheduling.
    Scheduling,

    /// Provider queue.
    Queue,

    /// Submission.
    Submission,

    /// Quantum/backend execution.
    Execution,

    /// Readout.
    Readout,

    /// Result retrieval.
    ResultRetrieval,
}

impl CanonicalTimingPhase {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compilation => "compilation",
            Self::Transpilation => "transpilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Queue => "queue",
            Self::Submission => "submission",
            Self::Execution => "execution",
            Self::Readout => "readout",
            Self::ResultRetrieval => "result_retrieval",
        }
    }
}

// =============================================================================
// Timing normalization
// =============================================================================

/// Normalized timing value suitable for consumption by benchmarking layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalizedTiming {
    /// Canonical duration.
    pub duration: Duration,

    /// Original source unit.
    pub source_unit: TimeUnit,

    /// Timing source.
    pub source: TimingSource,

    /// Clock domain.
    pub clock: ClockDomain,

    /// Timing quality.
    pub quality: TimingQuality,
}

impl NormalizedTiming {
    /// Normalizes a raw physical-time timing value.
    pub fn from_raw(
        raw: &RawTiming,
        quality: TimingQuality,
    ) -> Result<Self, TimingError> {
        Ok(Self {
            duration: raw.to_duration()?,
            source_unit: raw.unit.clone(),
            source: raw.source.clone(),
            clock: raw.clock.clone(),
            quality,
        })
    }
}

// =============================================================================
// Backend timing comparison
// =============================================================================

/// Safe comparison between two timing values.
///
/// This type prevents accidental comparison of timing measurements that come
/// from different clock domains or incompatible semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingComparison {
    /// First normalized timing value.
    pub left: NormalizedTiming,

    /// Second normalized timing value.
    pub right: NormalizedTiming,
}

impl TimingComparison {
    /// Creates a comparison after validating compatible clock domains.
    pub fn new(
        left: NormalizedTiming,
        right: NormalizedTiming,
    ) -> Result<Self, TimingError> {
        if left.clock != right.clock {
            return Err(TimingError::ClockDomainMismatch {
                start: left.clock,
                end: right.clock,
            });
        }

        Ok(Self { left, right })
    }

    /// Returns `left - right`, saturating at zero.
    ///
    /// This is intended for performance reporting where a negative elapsed
    /// difference is not meaningful.
    pub fn saturating_difference(&self) -> Duration {
        self.left
            .duration
            .saturating_sub(self.right.duration)
    }
}

// =============================================================================
// Timing conversion helpers
// =============================================================================

/// Converts a raw physical time value into `Duration`.
///
/// This is a small stable helper for backend adapters.
pub fn normalize_duration(
    value: u64,
    unit: TimeUnit,
    source: TimingSource,
    clock: ClockDomain,
) -> Result<NormalizedTiming, TimingError> {
    let raw = RawTiming::new(value, unit, source, clock)?;

    NormalizedTiming::from_raw(&raw, TimingQuality::Measured)
}

/// Converts clock cycles into normalized timing.
pub fn normalize_cycles(
    cycles: u64,
    frequency: ClockFrequency,
    source: TimingSource,
    clock: ClockDomain,
) -> Result<NormalizedTiming, TimingError> {
    let duration = frequency.cycles_to_duration(cycles)?;

    Ok(NormalizedTiming {
        duration,
        source_unit: TimeUnit::Cycles,
        source,
        clock,
        quality: TimingQuality::Measured,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_units_have_stable_names() {
        assert_eq!(TimeUnit::Seconds.as_str(), "s");
        assert_eq!(TimeUnit::Milliseconds.as_str(), "ms");
        assert_eq!(TimeUnit::Microseconds.as_str(), "us");
        assert_eq!(TimeUnit::Nanoseconds.as_str(), "ns");
        assert_eq!(TimeUnit::Picoseconds.as_str(), "ps");
        assert_eq!(TimeUnit::Femtoseconds.as_str(), "fs");
        assert_eq!(TimeUnit::Cycles.as_str(), "cycles");
    }

    #[test]
    fn raw_nanoseconds_are_normalized_exactly() {
        let raw = RawTiming::new(
            1_500,
            TimeUnit::Nanoseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        assert_eq!(
            raw.to_duration().expect("normalization"),
            Duration::from_nanos(1_500)
        );
    }

    #[test]
    fn raw_microseconds_are_normalized_exactly() {
        let raw = RawTiming::new(
            25,
            TimeUnit::Microseconds,
            TimingSource::Provider,
            ClockDomain::ProviderMonotonic,
        )
        .expect("valid timing");

        assert_eq!(
            raw.to_duration().expect("normalization"),
            Duration::from_micros(25)
        );
    }

    #[test]
    fn picoseconds_are_normalized_to_duration_precision() {
        let raw = RawTiming::new(
            2_500,
            TimeUnit::Picoseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        assert_eq!(
            raw.to_duration().expect("normalization"),
            Duration::from_nanos(2)
        );
    }

    #[test]
    fn femtoseconds_are_normalized_to_duration_precision() {
        let raw = RawTiming::new(
            3_000_000,
            TimeUnit::Femtoseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        assert_eq!(
            raw.to_duration().expect("normalization"),
            Duration::from_nanos(3)
        );
    }

    #[test]
    fn cycles_require_frequency() {
        let raw = RawTiming::new(
            100,
            TimeUnit::Cycles,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        assert!(matches!(
            raw.to_duration(),
            Err(TimingError::InvalidValue { .. })
        ));
    }

    #[test]
    fn cycle_conversion_is_deterministic() {
        let frequency = ClockFrequency::new(1_000_000_000)
            .expect("valid frequency");

        let duration = frequency
            .cycles_to_duration(10)
            .expect("valid conversion");

        assert_eq!(duration, Duration::from_nanos(10));
    }

    #[test]
    fn cycle_conversion_handles_fractional_nanoseconds() {
        let frequency =
            ClockFrequency::new(3).expect("valid frequency");

        let duration = frequency
            .cycles_to_duration(1)
            .expect("valid conversion");

        assert_eq!(duration, Duration::from_nanos(333_333_333));
    }

    #[test]
    fn timestamp_intervals_require_same_clock_domain() {
        let start = BackendTimestamp::new(
            100,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = BackendTimestamp::new(
            200,
            TimeUnit::Nanoseconds,
            ClockDomain::ProviderMonotonic,
        )
        .expect("valid timestamp");

        assert!(matches!(
            TimingInterval::new(start, end),
            Err(TimingError::ClockDomainMismatch { .. })
        ));
    }

    #[test]
    fn timestamp_interval_rejects_backward_time() {
        let start = BackendTimestamp::new(
            200,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = BackendTimestamp::new(
            100,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let interval =
            TimingInterval::new(start, end).expect("same clock");

        assert!(matches!(
            interval.duration(),
            Err(TimingError::EndBeforeStart { .. })
        ));
    }

    #[test]
    fn timestamp_interval_calculates_duration() {
        let start = BackendTimestamp::new(
            1_000,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let end = BackendTimestamp::new(
            2_500,
            TimeUnit::Nanoseconds,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timestamp");

        let interval =
            TimingInterval::new(start, end).expect("same clock");

        assert_eq!(
            interval.duration().expect("valid interval"),
            Duration::from_nanos(1_500)
        );
    }

    #[test]
    fn measurement_preserves_raw_provenance() {
        let raw = RawTiming::new(
            42,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let measurement = HardwareTimingMeasurement::from_raw(
            TimingField::Execution,
            raw.clone(),
        )
        .expect("valid measurement");

        assert_eq!(measurement.raw, raw);
        assert_eq!(measurement.duration, Duration::from_micros(42));
    }

    #[test]
    fn backend_record_rejects_duplicate_fields() {
        let raw_one = RawTiming::new(
            10,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let raw_two = RawTiming::new(
            20,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let measurement_one =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw_one,
            )
            .expect("valid measurement");

        let measurement_two =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw_two,
            )
            .expect("valid measurement");

        let mut record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        record
            .add_measurement(measurement_one)
            .expect("first measurement");

        assert!(record.add_measurement(measurement_two).is_err());
    }

    #[test]
    fn backend_record_rejects_mismatched_source() {
        let raw = RawTiming::new(
            10,
            TimeUnit::Microseconds,
            TimingSource::Provider,
            ClockDomain::ProviderMonotonic,
        )
        .expect("valid timing");

        let measurement =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw,
            )
            .expect("valid measurement");

        let mut record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        assert!(record.add_measurement(measurement).is_err());
    }

    #[test]
    fn backend_record_preserves_independent_total() {
        let raw_execution = RawTiming::new(
            100,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let raw_total = RawTiming::new(
            250,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let execution =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw_execution,
            )
            .expect("valid measurement");

        let total =
            HardwareTimingMeasurement::from_raw(
                TimingField::BackendTotal,
                raw_total,
            )
            .expect("valid measurement");

        let mut record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        record
            .add_measurement(execution)
            .expect("execution");

        record
            .set_backend_total(total)
            .expect("total");

        assert!(record.has_backend_total());
        assert_eq!(
            record.backend_total
                .expect("total exists")
                .duration,
            Duration::from_micros(250)
        );
    }

    #[test]
    fn canonical_mapping_preserves_execution_boundary() {
        assert_eq!(
            canonical_phase(&TimingField::Execution),
            Some(CanonicalTimingPhase::Execution)
        );

        assert_eq!(
            canonical_phase(&TimingField::Readout),
            Some(CanonicalTimingPhase::Readout)
        );

        assert_eq!(
            canonical_phase(&TimingField::Decoding),
            None
        );

        assert_eq!(
            canonical_phase(&TimingField::BackendTotal),
            None
        );
    }

    #[test]
    fn custom_units_must_not_be_empty() {
        let result = RawTiming::new(
            10,
            TimeUnit::Custom(String::new()),
            TimingSource::External,
            ClockDomain::External,
        );

        assert!(matches!(
            result,
            Err(TimingError::EmptyCustomUnit)
        ));
    }

    #[test]
    fn custom_clock_must_not_be_empty() {
        let result = RawTiming::new(
            10,
            TimeUnit::Nanoseconds,
            TimingSource::External,
            ClockDomain::Custom(String::new()),
        );

        assert!(matches!(
            result,
            Err(TimingError::EmptyCustomClock)
        ));
    }

    #[test]
    fn missing_timing_is_not_zero() {
        let record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        assert!(record.get(&TimingField::Execution).is_none());
    }

    #[test]
    fn zero_timing_is_explicit() {
        let raw = RawTiming::new(
            0,
            TimeUnit::Nanoseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("zero is a valid measured duration");

        let measurement =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw,
            )
            .expect("valid measurement");

        assert_eq!(measurement.duration, Duration::ZERO);
    }

    #[test]
    fn normalization_preserves_source_and_clock() {
        let normalized = normalize_duration(
            100,
            TimeUnit::Nanoseconds,
            TimingSource::Provider,
            ClockDomain::ProviderMonotonic,
        )
        .expect("valid timing");

        assert_eq!(normalized.duration, Duration::from_nanos(100));
        assert_eq!(normalized.source, TimingSource::Provider);
        assert_eq!(
            normalized.clock,
            ClockDomain::ProviderMonotonic
        );
        assert_eq!(normalized.source_unit, TimeUnit::Nanoseconds);
    }

    #[test]
    fn normalized_cycle_timing_preserves_cycle_source_unit() {
        let normalized = normalize_cycles(
            100,
            ClockFrequency::new(1_000_000_000)
                .expect("valid frequency"),
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        assert_eq!(normalized.duration, Duration::from_nanos(100));
        assert_eq!(normalized.source_unit, TimeUnit::Cycles);
    }

    #[test]
    fn backend_record_phase_sum_is_independent_of_total() {
        let execution_raw = RawTiming::new(
            100,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let readout_raw = RawTiming::new(
            50,
            TimeUnit::Microseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let execution =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                execution_raw,
            )
            .expect("valid measurement");

        let readout =
            HardwareTimingMeasurement::from_raw(
                TimingField::Readout,
                readout_raw,
            )
            .expect("valid measurement");

        let mut record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        record
            .add_measurement(execution)
            .expect("execution");

        record
            .add_measurement(readout)
            .expect("readout");

        assert_eq!(
            record.measured_phase_time(),
            Duration::from_micros(150)
        );
    }

    #[test]
    fn record_validation_succeeds_for_valid_record() {
        let raw = RawTiming::new(
            100,
            TimeUnit::Nanoseconds,
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid timing");

        let measurement =
            HardwareTimingMeasurement::from_raw(
                TimingField::Execution,
                raw,
            )
            .expect("valid measurement");

        let mut record = BackendTimingRecord::new(
            "test-qpu",
            TimingSource::Device,
            ClockDomain::DeviceMonotonic,
        )
        .expect("valid record");

        record
            .add_measurement(measurement)
            .expect("measurement");

        assert!(record.validate().is_ok());
    }
}