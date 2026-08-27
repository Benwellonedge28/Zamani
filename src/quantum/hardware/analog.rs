//! Zamani Quantum Hardware — Analog Quantum Computing
//!
//! Production-grade, provider-neutral representation of analog quantum
//! workloads, with particular support for Analog Hamiltonian Simulation (AHS).
//!
//! # Responsibility
//!
//! This module owns the semantic representation and validation of analog
//! quantum workloads at the hardware abstraction boundary.
//!
//! It provides:
//!
//! - physical analog registers;
//! - spatial coordinates;
//! - atom/resource placement;
//! - time series;
//! - piecewise-linear and piecewise-constant controls;
//! - driving fields;
//! - global and local detuning;
//! - phase control;
//! - analog Hamiltonian terms;
//! - pair interactions;
//! - interaction models;
//! - analog observables;
//! - analog programs;
//! - execution-independent analog workload validation;
//! - deterministic canonicalization;
//! - stable serialization;
//! - provider-neutral capability requirements;
//! - resource estimates;
//! - explicit experimental-feature requirements;
//! - provenance and schema information.
//!
//! # Non-responsibilities
//!
//! This module deliberately does NOT own:
//!
//! - provider APIs;
//! - HTTP/network communication;
//! - authentication;
//! - credentials;
//! - provider-specific SDKs;
//! - job submission;
//! - job polling;
//! - queues;
//! - result retrieval;
//! - provider pricing;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - simulator implementation;
//! - emulator implementation;
//! - benchmarking statistics;
//! - source-language parsing;
//! - OpenQASM parsing;
//! - QIR generation.
//!
//! Those concerns belong to their respective hardware/quantum subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum frontend / IR
//!      |
//!      v
//! Analog workload construction
//!      |
//!      v
//! hardware::analog
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! compatibility        resource estimation
//!      |                    |
//!      +----------+---------+
//!                 |
//!                 v
//!             execution
//!                 |
//!                 v
//!          provider adapter
//!                 |
//!                 v
//!          analog-capable QPU
//! ```
//!
//! # Important semantic distinction
//!
//! Analog quantum computing is NOT represented as a sequence of ordinary
//! gates.
//!
//! A gate-model workload describes operations such as:
//!
//! ```text
//! H q0
//! CX q0,q1
//! RZ(theta) q0
//! ```
//!
//! An analog workload instead describes:
//!
//! ```text
//! physical register
//! +
//! spatial geometry
//! +
//! time-dependent Hamiltonian/control fields
//! +
//! interaction model
//! +
//! execution duration
//! ```
//!
//! This matches the model used by current Analog Hamiltonian Simulation
//! systems, where the user specifies a physical register and the temporal and
//! spatial dependence of manipulating fields rather than a gate sequence.
//!
//! # Production design goals
//!
//! The implementation is:
//!
//! - provider-neutral;
//! - deterministic;
//! - serialization-safe;
//! - validation-heavy;
//! - explicit about units;
//! - explicit about experimental features;
//! - independent of network state;
//! - independent of wall-clock time;
//! - free of unsafe Rust;
//! - suitable for local simulators and real QPUs;
//! - extensible to future analog architectures.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! The repository currently declares `rust-version = "1.97.1"`.
//!
//! # Integration contract
//!
//! Later modules should consume this file as follows:
//!
//! ```text
//! analog.rs
//!     |
//!     +--> compatibility.rs
//!     |       checks AnalogProgram requirements against backend capabilities
//!     |
//!     +--> resource_estimator.rs
//!     |       estimates duration, resources and workload size
//!     |
//!     +--> execution.rs
//!     |       wraps AnalogProgram inside QuantumWorkload
//!     |
//!     +--> backend.rs
//!     |       advertises analog capability
//!     |
//!     +--> adapters/aws_braket.rs
//!     |       maps AnalogProgram to provider-specific AHS representation
//!     |
//!     +--> adapters/quera.rs
//!     |       maps AnalogProgram to QuEra-native representation
//!     |
//!     +--> simulator.rs
//!     |       executes AnalogProgram locally
//!     |
//!     +--> emulator.rs
//!             executes AnalogProgram using a hardware model
//! ```
//!
//! This module must remain independent of those consumers.
//!
//! # Capability integration
//!
//! An analog workload generally requires the provider-neutral hardware
//! capability corresponding to analog control.
//!
//! This module intentionally returns stable capability identifiers as strings
//! rather than importing the capability module. That keeps the dependency
//! direction acyclic and allows `capabilities.rs` to remain authoritative for
//! capability semantics.
//!
//! The canonical requirement identifiers emitted here are:
//!
//! - `analog_control`;
//! - `analog_hamiltonian_simulation`;
//! - `spatial_control` when spatially varying controls are used;
//! - `time_dependent_control` when controls vary with time;
//! - `local_control` when local fields are used;
//! - `pair_interaction` when explicit pair interactions are required.
//!
//! Provider adapters are responsible for mapping these requirements to their
//! provider-specific capability representation.
//!
//! # Security
//!
//! This module stores no credentials and performs no network operations.
//!
//! User/provider metadata is intentionally bounded.
//!
//! # Determinism
//!
//! This module never reads:
//!
//! - the system clock;
//! - operating-system randomness;
//! - network state;
//! - provider state.
//!
//! Floating-point validation rejects NaN and infinite values.
//!
//! Collections which form part of the semantic model preserve insertion order
//! where order is semantically meaningful. Canonical serialization is provided
//! separately through deterministic sorting of unordered metadata.
//!
//! # Units
//!
//! SI units are used at the hardware boundary:
//!
//! - time: seconds;
//! - position: metres;
//! - angular frequency: radians per second;
//! - phase: radians;
//! - dimensionless amplitudes: dimensionless unless explicitly documented;
//! - energy/frequency-like coefficients: represented by the provider-neutral
//!   `EnergyScale` abstraction.
//!
//! Providers are responsible for converting these values to native units.
//!
//! # Important physical rule
//!
//! A time series is not automatically interpreted as a linear interpolation.
//! Its interpolation semantics are explicitly declared.
//!
//! This prevents a serious class of hardware bugs where a provider assumes
//! piecewise-linear interpolation while the program intended a piecewise-
//! constant control field.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for analog hardware workloads.
pub const ANALOG_SCHEMA_ID: &str = "zamani.quantum.hardware.analog";

/// Current semantic schema version.
pub const ANALOG_SCHEMA_VERSION: u16 = 1;

/// Maximum number of analog resources in one register.
pub const MAX_REGISTER_RESOURCES: usize = 1_000_000;

/// Maximum number of time-series samples in one control field.
pub const MAX_TIME_SERIES_POINTS: usize = 10_000_000;

/// Maximum number of Hamiltonian terms.
pub const MAX_HAMILTONIAN_TERMS: usize = 10_000_000;

/// Maximum number of observables.
pub const MAX_OBSERVABLES: usize = 100_000;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum program name length.
pub const MAX_PROGRAM_NAME_LENGTH: usize = 512;

/// Maximum resource label length.
pub const MAX_RESOURCE_LABEL_LENGTH: usize = 256;

/// Minimum physically meaningful distance used by validation.
///
/// This is intentionally very small and should not be interpreted as a
/// universal hardware limit. It exists to reject duplicate coordinates caused
/// by numerical corruption.
pub const MIN_DISTINCT_POSITION_METERS: f64 = 1.0e-15;

// =============================================================================
// Error model
// =============================================================================

/// Error returned when an analog workload violates its semantic or physical
/// invariants.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalogError {
    /// A required collection is empty.
    EmptyField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A collection exceeds its production safety limit.
    CollectionLimitExceeded {
        /// Name of the collection.
        field: &'static str,
        /// Actual number of elements.
        actual: usize,
        /// Maximum permitted number.
        maximum: usize,
    },

    /// A string exceeds a safety limit.
    StringTooLong {
        /// Name of the field.
        field: &'static str,
        /// Actual byte length.
        actual: usize,
        /// Maximum permitted byte length.
        maximum: usize,
    },

    /// A floating-point value is NaN or infinite.
    NonFiniteValue {
        /// Name of the invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// A floating-point value is outside a permitted range.
    ValueOutOfRange {
        /// Name of the invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
        /// Inclusive lower bound.
        minimum: Option<f64>,
        /// Inclusive upper bound.
        maximum: Option<f64>,
    },

    /// A time series is not strictly ordered.
    NonMonotonicTime {
        /// Previous timestamp.
        previous: f64,
        /// Current timestamp.
        current: f64,
        /// Index of the current point.
        index: usize,
    },

    /// A time series begins before zero.
    NegativeTime {
        /// Invalid timestamp.
        time: f64,
        /// Index of the invalid point.
        index: usize,
    },

    /// A program duration is invalid.
    InvalidDuration {
        /// Supplied duration.
        duration: f64,
    },

    /// A register resource refers to an invalid identifier.
    InvalidResourceId {
        /// Invalid resource identifier.
        id: u32,
    },

    /// A control field refers to a resource that does not exist.
    ResourceIndexOutOfBounds {
        /// Referenced resource.
        resource: usize,
        /// Register size.
        register_size: usize,
    },

    /// A pair interaction references the same resource twice.
    SelfInteraction {
        /// Resource identifier.
        resource: usize,
    },

    /// A pair interaction references invalid resources.
    InvalidInteractionResource {
        /// First resource.
        first: usize,
        /// Second resource.
        second: usize,
        /// Register size.
        register_size: usize,
    },

    /// Two distinct resources are physically coincident.
    CoincidentResources {
        /// First resource.
        first: usize,
        /// Second resource.
        second: usize,
    },

    /// A duplicate resource identifier exists.
    DuplicateResourceId {
        /// Duplicate identifier.
        id: u32,
    },

    /// An analog program has no physical register.
    MissingRegister,

    /// A workload has no controls or Hamiltonian terms.
    MissingHamiltonian,

    /// The final time is inconsistent with the program.
    DurationMismatch {
        /// Program duration.
        program_duration: f64,
        /// Last control timestamp.
        last_time: f64,
    },

    /// A control field is inconsistent with the declared program duration.
    ControlDurationMismatch {
        /// Field name.
        field: &'static str,
        /// Last field timestamp.
        last_time: f64,
        /// Program duration.
        program_duration: f64,
    },

    /// A local field has a resource mask with the wrong length.
    InvalidResourceMask {
        /// Field name.
        field: &'static str,
        /// Actual mask size.
        actual: usize,
        /// Required mask size.
        expected: usize,
    },

    /// A field uses an invalid interpolation mode.
    InvalidInterpolation,

    /// An observable references an invalid resource.
    InvalidObservableResource {
        /// Resource index.
        resource: usize,
        /// Register size.
        register_size: usize,
    },

    /// An observable has no terms.
    EmptyObservable {
        /// Observable identifier.
        name: String,
    },

    /// Duplicate observable identifier.
    DuplicateObservable {
        /// Observable identifier.
        name: String,
    },

    /// Duplicate metadata key.
    DuplicateMetadataKey {
        /// Duplicate key.
        key: String,
    },

    /// Invalid metadata key.
    InvalidMetadataKey {
        /// Invalid key.
        key: String,
    },

    /// Invalid metadata value.
    InvalidMetadataValue {
        /// Metadata key.
        key: String,
    },

    /// A program contains a provider-specific requirement that is not valid.
    InvalidCapabilityRequirement {
        /// Requirement identifier.
        capability: String,
    },

    /// A program cannot be canonicalized because its semantic representation is
    /// inconsistent.
    CanonicalizationFailed {
        /// Explanation.
        reason: String,
    },
}

impl fmt::Display for AnalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "analog field `{field}` cannot be empty")
            }
            Self::CollectionLimitExceeded {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "analog field `{field}` contains {actual} elements; maximum is {maximum}"
            ),
            Self::StringTooLong {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "analog field `{field}` has length {actual}; maximum is {maximum}"
            ),
            Self::NonFiniteValue { field, value } => {
                write!(formatter, "analog field `{field}` contains non-finite value {value}")
            }
            Self::ValueOutOfRange {
                field,
                value,
                minimum,
                maximum,
            } => {
                write!(formatter, "analog field `{field}` has value {value} outside allowed range")?;

                if let Some(minimum) = minimum {
                    write!(formatter, " minimum={minimum}")?;
                }

                if let Some(maximum) = maximum {
                    write!(formatter, " maximum={maximum}")?;
                }

                Ok(())
            }
            Self::NonMonotonicTime {
                previous,
                current,
                index,
            } => write!(
                formatter,
                "time series is not strictly increasing at index {index}: previous={previous}, current={current}"
            ),
            Self::NegativeTime { time, index } => {
                write!(formatter, "time series contains negative time {time} at index {index}")
            }
            Self::InvalidDuration { duration } => {
                write!(formatter, "invalid analog program duration: {duration}")
            }
            Self::InvalidResourceId { id } => {
                write!(formatter, "invalid analog resource id: {id}")
            }
            Self::ResourceIndexOutOfBounds {
                resource,
                register_size,
            } => write!(
                formatter,
                "analog resource index {resource} is outside register size {register_size}"
            ),
            Self::SelfInteraction { resource } => {
                write!(formatter, "analog resource {resource} cannot interact with itself")
            }
            Self::InvalidInteractionResource {
                first,
                second,
                register_size,
            } => write!(
                formatter,
                "interaction resources ({first}, {second}) are invalid for register size {register_size}"
            ),
            Self::CoincidentResources { first, second } => write!(
                formatter,
                "analog resources {first} and {second} have coincident physical coordinates"
            ),
            Self::DuplicateResourceId { id } => {
                write!(formatter, "duplicate analog resource id {id}")
            }
            Self::MissingRegister => formatter.write_str("analog program requires a physical register"),
            Self::MissingHamiltonian => {
                formatter.write_str("analog program requires at least one Hamiltonian/control term")
            }
            Self::DurationMismatch {
                program_duration,
                last_time,
            } => write!(
                formatter,
                "analog program duration {program_duration} does not match final control time {last_time}"
            ),
            Self::ControlDurationMismatch {
                field,
                last_time,
                program_duration,
            } => write!(
                formatter,
                "analog field `{field}` ends at {last_time}, but program duration is {program_duration}"
            ),
            Self::InvalidResourceMask {
                field,
                actual,
                expected,
            } => write!(
                formatter,
                "analog field `{field}` has resource mask length {actual}; expected {expected}"
            ),
            Self::InvalidInterpolation => {
                formatter.write_str("invalid analog interpolation mode")
            }
            Self::InvalidObservableResource {
                resource,
                register_size,
            } => write!(
                formatter,
                "observable references resource {resource}, but register contains {register_size} resources"
            ),
            Self::EmptyObservable { name } => {
                write!(formatter, "analog observable `{name}` contains no terms")
            }
            Self::DuplicateObservable { name } => {
                write!(formatter, "duplicate analog observable `{name}`")
            }
            Self::DuplicateMetadataKey { key } => {
                write!(formatter, "duplicate analog metadata key `{key}`")
            }
            Self::InvalidMetadataKey { key } => {
                write!(formatter, "invalid analog metadata key `{key}`")
            }
            Self::InvalidMetadataValue { key } => {
                write!(formatter, "invalid analog metadata value for `{key}`")
            }
            Self::InvalidCapabilityRequirement { capability } => write!(
                formatter,
                "invalid analog capability requirement `{capability}`"
            ),
            Self::CanonicalizationFailed { reason } => {
                write!(formatter, "analog canonicalization failed: {reason}")
            }
        }
    }
}

impl Error for AnalogError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_finite(field: &'static str, value: f64) -> Result<(), AnalogError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AnalogError::NonFiniteValue { field, value })
    }
}

fn validate_non_negative(field: &'static str, value: f64) -> Result<(), AnalogError> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(AnalogError::ValueOutOfRange {
            field,
            value,
            minimum: Some(0.0),
            maximum: None,
        });
    }

    Ok(())
}

fn validate_string(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), AnalogError> {
    if value.is_empty() {
        return Err(AnalogError::EmptyField { field });
    }

    if value.len() > maximum {
        return Err(AnalogError::StringTooLong {
            field,
            actual: value.len(),
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AnalogError::InvalidMetadataValue {
            key: value.to_owned(),
        });
    }

    Ok(())
}

// =============================================================================
// Interpolation
// =============================================================================

/// Semantics used between consecutive time-series samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Interpolation {
    /// Value remains constant until the next sample.
    ///
    /// This is appropriate for controls whose semantics are piecewise
    /// constant.
    Step,

    /// Value changes linearly between samples.
    Linear,

    /// Value is held at the last sample after the final timestamp.
    ///
    /// This mode is only valid when the consumer explicitly permits
    /// extrapolation/hold semantics.
    HoldLast,
}

impl Default for Interpolation {
    fn default() -> Self {
        Self::Linear
    }
}

impl fmt::Display for Interpolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Step => formatter.write_str("step"),
            Self::Linear => formatter.write_str("linear"),
            Self::HoldLast => formatter.write_str("hold_last"),
        }
    }
}

// =============================================================================
// Time series
// =============================================================================

/// One time/value sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimePoint {
    /// Time from program start in seconds.
    pub time_seconds: f64,

    /// Control value at this time.
    pub value: f64,
}

impl TimePoint {
    /// Creates a time-series point.
    pub fn new(time_seconds: f64, value: f64) -> Result<Self, AnalogError> {
        validate_non_negative("time_seconds", time_seconds)?;
        validate_finite("value", value)?;

        Ok(Self {
            time_seconds,
            value,
        })
    }
}

/// Deterministic time-dependent analog control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Interpolation semantics.
    pub interpolation: Interpolation,

    /// Strictly increasing samples.
    pub points: Vec<TimePoint>,
}

impl TimeSeries {
    /// Creates an empty time series with the requested interpolation mode.
    pub fn new(interpolation: Interpolation) -> Self {
        Self {
            interpolation,
            points: Vec::new(),
        }
    }

    /// Creates a time series from points and validates ordering.
    pub fn from_points(
        interpolation: Interpolation,
        points: Vec<TimePoint>,
    ) -> Result<Self, AnalogError> {
        let series = Self {
            interpolation,
            points,
        };

        series.validate()?;
        Ok(series)
    }

    /// Appends one point while enforcing strict temporal ordering.
    pub fn push(&mut self, point: TimePoint) -> Result<(), AnalogError> {
        if self.points.len() >= MAX_TIME_SERIES_POINTS {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "time_series.points",
                actual: self.points.len() + 1,
                maximum: MAX_TIME_SERIES_POINTS,
            });
        }

        if let Some(previous) = self.points.last() {
            if point.time_seconds <= previous.time_seconds {
                return Err(AnalogError::NonMonotonicTime {
                    previous: previous.time_seconds,
                    current: point.time_seconds,
                    index: self.points.len(),
                });
            }
        }

        self.points.push(point);
        Ok(())
    }

    /// Returns the first timestamp, if any.
    pub fn start_time(&self) -> Option<f64> {
        self.points.first().map(|point| point.time_seconds)
    }

    /// Returns the final timestamp, if any.
    pub fn end_time(&self) -> Option<f64> {
        self.points.last().map(|point| point.time_seconds)
    }

    /// Returns true if the series contains no samples.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Validates all samples and temporal invariants.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.points.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "time_series.points",
            });
        }

        if self.points.len() > MAX_TIME_SERIES_POINTS {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "time_series.points",
                actual: self.points.len(),
                maximum: MAX_TIME_SERIES_POINTS,
            });
        }

        for (index, point) in self.points.iter().enumerate() {
            validate_non_negative("time_seconds", point.time_seconds)?;
            validate_finite("value", point.value)?;

            if index > 0 {
                let previous = self.points[index - 1].time_seconds;

                if point.time_seconds <= previous {
                    return Err(AnalogError::NonMonotonicTime {
                        previous,
                        current: point.time_seconds,
                        index,
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Spatial coordinates
// =============================================================================

/// Three-dimensional physical coordinate in metres.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate in metres.
    pub x_meters: f64,

    /// Y coordinate in metres.
    pub y_meters: f64,

    /// Z coordinate in metres.
    pub z_meters: f64,
}

impl Position {
    /// Creates a 3-D position.
    pub fn new(
        x_meters: f64,
        y_meters: f64,
        z_meters: f64,
    ) -> Result<Self, AnalogError> {
        validate_finite("position.x_meters", x_meters)?;
        validate_finite("position.y_meters", y_meters)?;
        validate_finite("position.z_meters", z_meters)?;

        Ok(Self {
            x_meters,
            y_meters,
            z_meters,
        })
    }

    /// Creates a two-dimensional position.
    pub fn xy(x_meters: f64, y_meters: f64) -> Result<Self, AnalogError> {
        Self::new(x_meters, y_meters, 0.0)
    }

    /// Euclidean distance to another position in metres.
    pub fn distance_meters(self, other: Self) -> f64 {
        let dx = self.x_meters - other.x_meters;
        let dy = self.y_meters - other.y_meters;
        let dz = self.z_meters - other.z_meters;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Validates coordinate finiteness.
    pub fn validate(&self) -> Result<(), AnalogError> {
        validate_finite("position.x_meters", self.x_meters)?;
        validate_finite("position.y_meters", self.y_meters)?;
        validate_finite("position.z_meters", self.z_meters)?;
        Ok(())
    }
}

// =============================================================================
// Analog resource/register
// =============================================================================

/// A physical quantum resource used by an analog workload.
///
/// In a neutral-atom AHS implementation this can correspond to an atom.
/// Future providers may map it to another physical quantum degree of freedom.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogResource {
    /// Stable workload-local resource identifier.
    pub id: u32,

    /// Physical position.
    pub position: Position,

    /// Optional human-readable label.
    pub label: Option<String>,
}

impl AnalogResource {
    /// Creates a resource.
    pub fn new(id: u32, position: Position) -> Result<Self, AnalogError> {
        if id == u32::MAX {
            return Err(AnalogError::InvalidResourceId { id });
        }

        position.validate()?;

        Ok(Self {
            id,
            position,
            label: None,
        })
    }

    /// Assigns a human-readable label.
    pub fn with_label(mut self, label: impl Into<String>) -> Result<Self, AnalogError> {
        let label = label.into();

        if label.len() > MAX_RESOURCE_LABEL_LENGTH {
            return Err(AnalogError::StringTooLong {
                field: "resource.label",
                actual: label.len(),
                maximum: MAX_RESOURCE_LABEL_LENGTH,
            });
        }

        if label.chars().any(char::is_control) {
            return Err(AnalogError::InvalidMetadataValue { key: label });
        }

        self.label = Some(label);
        Ok(self)
    }
}

/// Physical analog register.
///
/// The order of resources is semantically significant because workload-local
/// resource indices refer to this ordering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogRegister {
    /// Physical resources.
    pub resources: Vec<AnalogResource>,
}

impl AnalogRegister {
    /// Creates an empty register.
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    /// Creates a register from resources.
    pub fn from_resources(resources: Vec<AnalogResource>) -> Result<Self, AnalogError> {
        let register = Self { resources };
        register.validate()?;
        Ok(register)
    }

    /// Adds a resource.
    pub fn push(&mut self, resource: AnalogResource) -> Result<(), AnalogError> {
        if self.resources.len() >= MAX_REGISTER_RESOURCES {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "register.resources",
                actual: self.resources.len() + 1,
                maximum: MAX_REGISTER_RESOURCES,
            });
        }

        if self.resources.iter().any(|existing| existing.id == resource.id) {
            return Err(AnalogError::DuplicateResourceId { id: resource.id });
        }

        self.resources.push(resource);
        Ok(())
    }

    /// Number of physical resources.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns true when there are no resources.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Gets a resource by workload-local index.
    pub fn get(&self, index: usize) -> Option<&AnalogResource> {
        self.resources.get(index)
    }

    /// Validates resource IDs, positions and geometric uniqueness.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.resources.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "register.resources",
            });
        }

        if self.resources.len() > MAX_REGISTER_RESOURCES {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "register.resources",
                actual: self.resources.len(),
                maximum: MAX_REGISTER_RESOURCES,
            });
        }

        let mut ids = BTreeSet::new();

        for resource in &self.resources {
            if resource.id == u32::MAX {
                return Err(AnalogError::InvalidResourceId { id: resource.id });
            }

            resource.position.validate()?;

            if resource
                .label
                .as_deref()
                .is_some_and(|label| label.len() > MAX_RESOURCE_LABEL_LENGTH)
            {
                return Err(AnalogError::StringTooLong {
                    field: "resource.label",
                    actual: resource.label.as_deref().unwrap_or("").len(),
                    maximum: MAX_RESOURCE_LABEL_LENGTH,
                });
            }

            if !ids.insert(resource.id) {
                return Err(AnalogError::DuplicateResourceId { id: resource.id });
            }
        }

        for first in 0..self.resources.len() {
            for second in (first + 1)..self.resources.len() {
                let distance = self.resources[first]
                    .position
                    .distance_meters(self.resources[second].position);

                if !distance.is_finite() {
                    return Err(AnalogError::NonFiniteValue {
                        field: "resource.distance",
                        value: distance,
                    });
                }

                if distance < MIN_DISTINCT_POSITION_METERS {
                    return Err(AnalogError::CoincidentResources { first, second });
                }
            }
        }

        Ok(())
    }
}

impl Default for AnalogRegister {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Control fields
// =============================================================================

/// Global coherent driving field.
///
/// The fields correspond to the common AHS abstraction of amplitude, phase and
/// detuning. The precise physical interpretation remains a provider/device
/// responsibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrivingField {
    /// Time-dependent amplitude/Rabi-frequency-like control.
    pub amplitude: TimeSeries,

    /// Time-dependent phase in radians.
    pub phase: TimeSeries,

    /// Time-dependent global detuning in radians per second.
    pub detuning: TimeSeries,
}

impl DrivingField {
    /// Creates a driving field.
    pub fn new(
        amplitude: TimeSeries,
        phase: TimeSeries,
        detuning: TimeSeries,
    ) -> Result<Self, AnalogError> {
        let field = Self {
            amplitude,
            phase,
            detuning,
        };

        field.validate()?;
        Ok(field)
    }

    /// Validates all component series.
    pub fn validate(&self) -> Result<(), AnalogError> {
        self.amplitude.validate()?;
        self.phase.validate()?;
        self.detuning.validate()?;

        validate_equal_end_time(
            "driving_field.amplitude",
            self.amplitude.end_time(),
            self.phase.end_time(),
        )?;

        validate_equal_end_time(
            "driving_field.amplitude",
            self.amplitude.end_time(),
            self.detuning.end_time(),
        )?;

        Ok(())
    }

    /// Returns the final control time.
    pub fn end_time(&self) -> f64 {
        self.amplitude
            .end_time()
            .expect("validated driving field has amplitude samples")
    }
}

/// Local detuning applied to selected resources.
///
/// The mask has one boolean entry per register resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalDetuningField {
    /// Resource-selection mask.
    pub resource_mask: Vec<bool>,

    /// Time-dependent detuning.
    pub detuning: TimeSeries,
}

impl LocalDetuningField {
    /// Creates a local detuning field.
    pub fn new(
        resource_mask: Vec<bool>,
        detuning: TimeSeries,
    ) -> Result<Self, AnalogError> {
        if resource_mask.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "local_detuning.resource_mask",
            });
        }

        detuning.validate()?;

        Ok(Self {
            resource_mask,
            detuning,
        })
    }

    /// Validates against a register size.
    pub fn validate_for_register(
        &self,
        register_size: usize,
    ) -> Result<(), AnalogError> {
        if self.resource_mask.len() != register_size {
            return Err(AnalogError::InvalidResourceMask {
                field: "local_detuning.resource_mask",
                actual: self.resource_mask.len(),
                expected: register_size,
            });
        }

        self.detuning.validate()
    }

    /// Returns the final control time.
    pub fn end_time(&self) -> f64 {
        self.detuning
            .end_time()
            .expect("validated local detuning has samples")
    }

    /// Returns true if at least one resource is affected.
    pub fn affects_resources(&self) -> bool {
        self.resource_mask.iter().any(|value| *value)
    }
}

/// Generic spatially varying control field.
///
/// This abstraction is deliberately provider-neutral and is useful for future
/// analog architectures beyond the current global/local detuning examples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialControlField {
    /// Stable workload-local field name.
    pub name: String,

    /// Resource-selection mask.
    pub resource_mask: Vec<bool>,

    /// Time-dependent control values.
    pub values: TimeSeries,

    /// Whether this field is experimental.
    pub experimental: bool,
}

impl SpatialControlField {
    /// Creates a spatial control field.
    pub fn new(
        name: impl Into<String>,
        resource_mask: Vec<bool>,
        values: TimeSeries,
    ) -> Result<Self, AnalogError> {
        let name = name.into();

        validate_string("spatial_control.name", &name, MAX_PROGRAM_NAME_LENGTH)?;

        if resource_mask.is_empty() {
            return Err(AnalogError::EmptyField {
                field: "spatial_control.resource_mask",
            });
        }

        values.validate()?;

        Ok(Self {
            name,
            resource_mask,
            values,
            experimental: false,
        })
    }

    /// Marks the control as experimental.
    pub fn experimental(mut self, value: bool) -> Self {
        self.experimental = value;
        self
    }

    /// Validates against a register size.
    pub fn validate_for_register(
        &self,
        register_size: usize,
    ) -> Result<(), AnalogError> {
        if self.resource_mask.len() != register_size {
            return Err(AnalogError::InvalidResourceMask {
                field: "spatial_control.resource_mask",
                actual: self.resource_mask.len(),
                expected: register_size,
            });
        }

        self.values.validate()
    }

    /// Returns the final control time.
    pub fn end_time(&self) -> f64 {
        self.values
            .end_time()
            .expect("validated spatial field has samples")
    }
}

// =============================================================================
// Interaction models
// =============================================================================

/// Provider-neutral pair-interaction model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PairInteraction {
    /// Inverse-power-law interaction:
    ///
    /// `strength / distance^power`
    InversePower {
        /// Interaction strength in the provider-neutral energy scale.
        strength: f64,

        /// Positive interaction exponent.
        power: f64,
    },

    /// Van der Waals-style interaction:
    ///
    /// `strength / distance^6`
    VanDerWaals {
        /// Interaction strength.
        strength: f64,
    },

    /// Dipole-dipole-style interaction:
    ///
    /// `strength / distance^3`
    DipoleDipole {
        /// Interaction strength.
        strength: f64,
    },

    /// Provider-defined interaction law.
    ///
    /// The identifier must be interpreted by the provider adapter.
    Custom {
        /// Stable provider-neutral extension identifier.
        model_id: String,

        /// Model parameter.
        strength: f64,
    },
}

impl PairInteraction {
    /// Validates interaction parameters.
    pub fn validate(&self) -> Result<(), AnalogError> {
        match self {
            Self::InversePower { strength, power } => {
                validate_finite("interaction.strength", *strength)?;
                validate_finite("interaction.power", *power)?;

                if *power <= 0.0 {
                    return Err(AnalogError::ValueOutOfRange {
                        field: "interaction.power",
                        value: *power,
                        minimum: Some(f64::MIN_POSITIVE),
                        maximum: None,
                    });
                }
            }
            Self::VanDerWaals { strength } => {
                validate_finite("interaction.strength", *strength)?;
            }
            Self::DipoleDipole { strength } => {
                validate_finite("interaction.strength", *strength)?;
            }
            Self::Custom { model_id, strength } => {
                validate_string("interaction.model_id", model_id, MAX_PROGRAM_NAME_LENGTH)?;
                validate_finite("interaction.strength", *strength)?;
            }
        }

        Ok(())
    }
}

/// Explicit pair interaction between two resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PairInteractionTerm {
    /// First register-local resource index.
    pub first: usize,

    /// Second register-local resource index.
    pub second: usize,

    /// Interaction law.
    pub interaction: PairInteraction,
}

impl PairInteractionTerm {
    /// Creates a pair interaction.
    pub fn new(
        first: usize,
        second: usize,
        interaction: PairInteraction,
    ) -> Result<Self, AnalogError> {
        if first == second {
            return Err(AnalogError::SelfInteraction { resource: first });
        }

        interaction.validate()?;

        Ok(Self {
            first,
            second,
            interaction,
        })
    }

    /// Validates resource indices.
    pub fn validate_for_register(
        &self,
        register_size: usize,
    ) -> Result<(), AnalogError> {
        if self.first >= register_size || self.second >= register_size {
            return Err(AnalogError::InvalidInteractionResource {
                first: self.first,
                second: self.second,
                register_size,
            });
        }

        if self.first == self.second {
            return Err(AnalogError::SelfInteraction {
                resource: self.first,
            });
        }

        self.interaction.validate()
    }
}

// =============================================================================
// Hamiltonian terms
// =============================================================================

/// Provider-neutral analog Hamiltonian/control term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HamiltonianTerm {
    /// Global coherent driving field.
    DrivingField(DrivingField),

    /// Spatially selective detuning.
    LocalDetuning(LocalDetuningField),

    /// Generic spatially varying control.
    SpatialControl(SpatialControlField),

    /// Explicit pair interaction.
    PairInteraction(PairInteractionTerm),
}

impl HamiltonianTerm {
    /// Returns a stable semantic identifier.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::DrivingField(_) => "driving_field",
            Self::LocalDetuning(_) => "local_detuning",
            Self::SpatialControl(_) => "spatial_control",
            Self::PairInteraction(_) => "pair_interaction",
        }
    }

    /// Returns the final time of the term, when applicable.
    pub fn end_time(&self) -> Option<f64> {
        match self {
            Self::DrivingField(field) => Some(field.end_time()),
            Self::LocalDetuning(field) => Some(field.end_time()),
            Self::SpatialControl(field) => Some(field.end_time()),
            Self::PairInteraction(_) => None,
        }
    }

    /// Validates this term against a register.
    pub fn validate_for_register(
        &self,
        register_size: usize,
    ) -> Result<(), AnalogError> {
        match self {
            Self::DrivingField(field) => field.validate(),
            Self::LocalDetuning(field) => {
                field.validate_for_register(register_size)
            }
            Self::SpatialControl(field) => {
                field.validate_for_register(register_size)
            }
            Self::PairInteraction(term) => {
                term.validate_for_register(register_size)
            }
        }
    }

    /// Whether this term requires spatially selective control.
    pub fn requires_spatial_control(&self) -> bool {
        matches!(
            self,
            Self::LocalDetuning(_) | Self::SpatialControl(_)
        )
    }

    /// Whether this term is experimental by declaration.
    pub fn is_experimental(&self) -> bool {
        matches!(
            self,
            Self::SpatialControl(SpatialControlField {
                experimental: true,
                ..
            })
        )
    }
}

// =============================================================================
// Observables
// =============================================================================

/// Supported analog observable basis operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservableOperator {
    /// Population/occupation operator.
    Number,

    /// Pauli-X-like observable.
    X,

    /// Pauli-Y-like observable.
    Y,

    /// Pauli-Z-like observable.
    Z,

    /// Identity operator.
    Identity,
}

impl fmt::Display for ObservableOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number => formatter.write_str("number"),
            Self::X => formatter.write_str("x"),
            Self::Y => formatter.write_str("y"),
            Self::Z => formatter.write_str("z"),
            Self::Identity => formatter.write_str("identity"),
        }
    }
}

/// One term in an observable.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ObservableTerm {
    /// Register-local resource.
    pub resource: usize,

    /// Operator.
    pub operator: ObservableOperator,

    /// Real coefficient.
    pub coefficient: f64,
}

impl ObservableTerm {
    /// Creates an observable term.
    pub fn new(
        resource: usize,
        operator: ObservableOperator,
        coefficient: f64,
    ) -> Result<Self, AnalogError> {
        validate_finite("observable.coefficient", coefficient)?;

        Ok(Self {
            resource,
            operator,
            coefficient,
        })
    }
}

/// An observable requested from an analog execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogObservable {
    /// Stable workload-local identifier.
    pub name: String,

    /// Observable terms.
    pub terms: Vec<ObservableTerm>,
}

impl AnalogObservable {
    /// Creates an observable.
    pub fn new(
        name: impl Into<String>,
        terms: Vec<ObservableTerm>,
    ) -> Result<Self, AnalogError> {
        let name = name.into();

        validate_string(
            "observable.name",
            &name,
            MAX_PROGRAM_NAME_LENGTH,
        )?;

        let observable = Self { name, terms };

        observable.validate()?;

        Ok(observable)
    }

    /// Validates observable semantics.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.terms.is_empty() {
            return Err(AnalogError::EmptyObservable {
                name: self.name.clone(),
            });
        }

        for term in &self.terms {
            validate_finite("observable.coefficient", term.coefficient)?;
        }

        Ok(())
    }

    /// Validates resource indices against a register.
    pub fn validate_for_register(
        &self,
        register_size: usize,
    ) -> Result<(), AnalogError> {
        self.validate()?;

        for term in &self.terms {
            if term.resource >= register_size {
                return Err(AnalogError::InvalidObservableResource {
                    resource: term.resource,
                    register_size,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Program metadata
// =============================================================================

/// Deterministic metadata attached to an analog workload.
///
/// Metadata is deliberately represented as strings because this file must not
/// depend on a provider-specific metadata schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalogMetadata {
    /// Key/value metadata.
    pub values: BTreeMap<String, String>,
}

impl AnalogMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts metadata after validating limits.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AnalogError> {
        if self.values.len() >= MAX_METADATA_ENTRIES {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "metadata.values",
                actual: self.values.len() + 1,
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        let key = key.into();
        let value = value.into();

        if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
            return Err(AnalogError::InvalidMetadataKey { key });
        }

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(AnalogError::StringTooLong {
                field: "metadata.value",
                actual: value.len(),
                maximum: MAX_METADATA_VALUE_LENGTH,
            });
        }

        if key.chars().any(char::is_control) {
            return Err(AnalogError::InvalidMetadataKey { key });
        }

        if value.chars().any(char::is_control) {
            return Err(AnalogError::InvalidMetadataValue { key });
        }

        if self.values.contains_key(&key) {
            return Err(AnalogError::DuplicateMetadataKey { key });
        }

        Ok(self.values.insert(key, value))
    }

    /// Validates metadata.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.values.len() > MAX_METADATA_ENTRIES {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "metadata.values",
                actual: self.values.len(),
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        for (key, value) in &self.values {
            if key.is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
                return Err(AnalogError::InvalidMetadataKey { key: key.clone() });
            }

            if value.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(AnalogError::StringTooLong {
                    field: "metadata.value",
                    actual: value.len(),
                    maximum: MAX_METADATA_VALUE_LENGTH,
                });
            }

            if key.chars().any(char::is_control) {
                return Err(AnalogError::InvalidMetadataKey { key: key.clone() });
            }

            if value.chars().any(char::is_control) {
                return Err(AnalogError::InvalidMetadataValue { key: key.clone() });
            }
        }

        Ok(())
    }
}

impl Default for AnalogMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Execution semantics
// =============================================================================

/// Measurement/sampling policy for an analog workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalogSamplingMode {
    /// Return sampled computational outcomes.
    Samples,

    /// Return probabilities where supported.
    Probabilities,

    /// Request expectation values for explicitly supplied observables.
    ExpectationValues,
}

impl Default for AnalogSamplingMode {
    fn default() -> Self {
        Self::Samples
    }
}

/// Execution policy independent of provider APIs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogExecutionOptions {
    /// Number of repetitions/shots.
    pub shots: u64,

    /// Sampling/result mode.
    pub sampling_mode: AnalogSamplingMode,

    /// Optional deterministic seed.
    pub seed: Option<u64>,

    /// Whether experimental capabilities may be used.
    pub allow_experimental: bool,
}

impl Default for AnalogExecutionOptions {
    fn default() -> Self {
        Self {
            shots: 1,
            sampling_mode: AnalogSamplingMode::Samples,
            seed: None,
            allow_experimental: false,
        }
    }
}

impl AnalogExecutionOptions {
    /// Validates execution options.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.shots == 0 {
            return Err(AnalogError::ValueOutOfRange {
                field: "execution.shots",
                value: 0.0,
                minimum: Some(1.0),
                maximum: Some(u64::MAX as f64),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Analog program
// =============================================================================

/// Complete provider-neutral analog Hamiltonian workload.
///
/// This is the primary type that later `execution.rs` should place inside its
/// generic `QuantumWorkload::AnalogProgram` variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogProgram {
    /// Stable schema identifier.
    pub schema: String,

    /// Schema version.
    pub schema_version: u16,

    /// Optional human-readable workload name.
    pub name: Option<String>,

    /// Physical register/layout.
    pub register: AnalogRegister,

    /// Hamiltonian/control terms.
    pub terms: Vec<HamiltonianTerm>,

    /// Requested observables.
    pub observables: Vec<AnalogObservable>,

    /// Total program duration in seconds.
    pub duration_seconds: f64,

    /// Execution options.
    pub execution: AnalogExecutionOptions,

    /// Provider-neutral metadata.
    pub metadata: AnalogMetadata,
}

impl AnalogProgram {
    /// Creates a new analog program.
    pub fn new(
        register: AnalogRegister,
        terms: Vec<HamiltonianTerm>,
        duration_seconds: f64,
    ) -> Result<Self, AnalogError> {
        let program = Self {
            schema: ANALOG_SCHEMA_ID.to_owned(),
            schema_version: ANALOG_SCHEMA_VERSION,
            name: None,
            register,
            terms,
            observables: Vec::new(),
            duration_seconds,
            execution: AnalogExecutionOptions::default(),
            metadata: AnalogMetadata::default(),
        };

        program.validate()?;
        Ok(program)
    }

    /// Assigns a human-readable name.
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self, AnalogError> {
        let name = name.into();

        validate_string("program.name", &name, MAX_PROGRAM_NAME_LENGTH)?;

        self.name = Some(name);
        Ok(self)
    }

    /// Adds an observable.
    pub fn add_observable(
        &mut self,
        observable: AnalogObservable,
    ) -> Result<(), AnalogError> {
        if self.observables.len() >= MAX_OBSERVABLES {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "program.observables",
                actual: self.observables.len() + 1,
                maximum: MAX_OBSERVABLES,
            });
        }

        if self
            .observables
            .iter()
            .any(|existing| existing.name == observable.name)
        {
            return Err(AnalogError::DuplicateObservable {
                name: observable.name,
            });
        }

        observable.validate_for_register(self.register.len())?;

        self.observables.push(observable);
        Ok(())
    }

    /// Adds a Hamiltonian/control term.
    pub fn add_term(
        &mut self,
        term: HamiltonianTerm,
    ) -> Result<(), AnalogError> {
        if self.terms.len() >= MAX_HAMILTONIAN_TERMS {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "program.terms",
                actual: self.terms.len() + 1,
                maximum: MAX_HAMILTONIAN_TERMS,
            });
        }

        term.validate_for_register(self.register.len())?;
        self.terms.push(term);

        Ok(())
    }

    /// Returns whether the workload contains spatially selective controls.
    pub fn requires_spatial_control(&self) -> bool {
        self.terms
            .iter()
            .any(HamiltonianTerm::requires_spatial_control)
    }

    /// Returns whether the workload contains explicitly experimental controls.
    pub fn requires_experimental_capabilities(&self) -> bool {
        self.terms
            .iter()
            .any(HamiltonianTerm::is_experimental)
    }

    /// Returns the provider-neutral capability identifiers required by this
    /// workload.
    ///
    /// These strings are intentionally stable and provider-neutral.
    pub fn required_capabilities(&self) -> BTreeSet<String> {
        let mut capabilities = BTreeSet::new();

        capabilities.insert("analog_control".to_owned());
        capabilities.insert("analog_hamiltonian_simulation".to_owned());
        capabilities.insert("time_dependent_control".to_owned());

        if self.requires_spatial_control() {
            capabilities.insert("spatial_control".to_owned());
        }

        if self
            .terms
            .iter()
            .any(|term| matches!(term, HamiltonianTerm::LocalDetuning(_)))
        {
            capabilities.insert("local_control".to_owned());
        }

        if self
            .terms
            .iter()
            .any(|term| matches!(term, HamiltonianTerm::PairInteraction(_)))
        {
            capabilities.insert("pair_interaction".to_owned());
        }

        if self.requires_experimental_capabilities() {
            capabilities.insert("experimental_analog_control".to_owned());
        }

        capabilities
    }

    /// Returns the largest number of physical resources used.
    pub fn resource_count(&self) -> usize {
        self.register.len()
    }

    /// Validates the complete workload.
    pub fn validate(&self) -> Result<(), AnalogError> {
        if self.schema != ANALOG_SCHEMA_ID {
            return Err(AnalogError::CanonicalizationFailed {
                reason: format!(
                    "unexpected schema `{}`; expected `{ANALOG_SCHEMA_ID}`",
                    self.schema
                ),
            });
        }

        if self.schema_version == 0 || self.schema_version > ANALOG_SCHEMA_VERSION {
            return Err(AnalogError::CanonicalizationFailed {
                reason: format!(
                    "unsupported analog schema version {}",
                    self.schema_version
                ),
            });
        }

        if let Some(name) = &self.name {
            validate_string("program.name", name, MAX_PROGRAM_NAME_LENGTH)?;
        }

        self.register.validate()?;

        if self.terms.is_empty() {
            return Err(AnalogError::MissingHamiltonian);
        }

        if self.terms.len() > MAX_HAMILTONIAN_TERMS {
            return Err(AnalogError::CollectionLimitExceeded {
                field: "program.terms",
                actual: self.terms.len(),
                maximum: MAX_HAMILTONIAN_TERMS,
            });
        }

        validate_non_negative("program.duration_seconds", self.duration_seconds)?;

        if self.duration_seconds <= 0.0 {
            return Err(AnalogError::InvalidDuration {
                duration: self.duration_seconds,
            });
        }

        for term in &self.terms {
            term.validate_for_register(self.register.len())?;

            if let Some(end_time) = term.end_time() {
                validate_equal_duration(
                    term.kind(),
                    end_time,
                    self.duration_seconds,
                )?;
            }
        }

        self.execution.validate()?;

        self.metadata.validate()?;

        let mut observable_names = BTreeSet::new();

        for observable in &self.observables {
            observable.validate_for_register(self.register.len())?;

            if !observable_names.insert(observable.name.clone()) {
                return Err(AnalogError::DuplicateObservable {
                    name: observable.name.clone(),
                });
            }
        }

        Ok(())
    }

    /// Produces a validated canonical copy.
    ///
    /// Canonicalization sorts semantically unordered collections while
    /// preserving resource order and time-series order.
    pub fn canonicalize(&self) -> Result<Self, AnalogError> {
        self.validate()?;

        let mut canonical = self.clone();

        canonical.terms.sort_by(|left, right| {
            left.kind()
                .cmp(right.kind())
                .then_with(|| canonical_term_key(left).cmp(&canonical_term_key(right)))
        });

        canonical
            .observables
            .sort_by(|left, right| left.name.cmp(&right.name));

        Ok(canonical)
    }

    /// Returns a deterministic semantic fingerprint input.
    ///
    /// The method deliberately does not hash itself; callers can serialize
    /// this canonical structure using their chosen repository-wide hashing
    /// mechanism.
    pub fn canonical_serializable(&self) -> Result<Self, AnalogError> {
        self.canonicalize()
    }
}

fn canonical_term_key(term: &HamiltonianTerm) -> String {
    match term {
        HamiltonianTerm::DrivingField(_) => "driving_field".to_owned(),
        HamiltonianTerm::LocalDetuning(field) => format!(
            "local_detuning:{}",
            field
                .resource_mask
                .iter()
                .map(|value| if *value { '1' } else { '0' })
                .collect::<String>()
        ),
        HamiltonianTerm::SpatialControl(field) => {
            format!("spatial_control:{}", field.name)
        }
        HamiltonianTerm::PairInteraction(term) => {
            format!("pair:{}:{}", term.first, term.second)
        }
    }
}

// =============================================================================
// Resource estimation
// =============================================================================

/// Provider-neutral resource estimate for an analog workload.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AnalogResourceEstimate {
    /// Number of physical resources.
    pub resource_count: usize,

    /// Program duration in seconds.
    pub duration_seconds: f64,

    /// Number of requested shots.
    pub shots: u64,

    /// Number of Hamiltonian/control terms.
    pub term_count: usize,

    /// Number of requested observables.
    pub observable_count: usize,

    /// Whether spatial control is required.
    pub requires_spatial_control: bool,

    /// Whether experimental capabilities are required.
    pub requires_experimental_capabilities: bool,
}

impl AnalogProgram {
    /// Computes a deterministic hardware-independent resource estimate.
    pub fn estimate_resources(&self) -> Result<AnalogResourceEstimate, AnalogError> {
        self.validate()?;

        Ok(AnalogResourceEstimate {
            resource_count: self.resource_count(),
            duration_seconds: self.duration_seconds,
            shots: self.execution.shots,
            term_count: self.terms.len(),
            observable_count: self.observables.len(),
            requires_spatial_control: self.requires_spatial_control(),
            requires_experimental_capabilities: self
                .requires_experimental_capabilities(),
        })
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for production-safe analog workloads.
///
/// The builder is intentionally thin. It does not bypass validation.
#[derive(Debug, Default)]
pub struct AnalogProgramBuilder {
    name: Option<String>,
    register: Option<AnalogRegister>,
    terms: Vec<HamiltonianTerm>,
    observables: Vec<AnalogObservable>,
    duration_seconds: Option<f64>,
    execution: AnalogExecutionOptions,
    metadata: AnalogMetadata,
}

impl AnalogProgramBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the workload name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the physical register.
    pub fn register(mut self, register: AnalogRegister) -> Self {
        self.register = Some(register);
        self
    }

    /// Adds a Hamiltonian term.
    pub fn term(mut self, term: HamiltonianTerm) -> Self {
        self.terms.push(term);
        self
    }

    /// Adds an observable.
    pub fn observable(mut self, observable: AnalogObservable) -> Self {
        self.observables.push(observable);
        self
    }

    /// Sets total execution duration in seconds.
    pub fn duration_seconds(mut self, duration_seconds: f64) -> Self {
        self.duration_seconds = Some(duration_seconds);
        self
    }

    /// Sets execution options.
    pub fn execution(mut self, execution: AnalogExecutionOptions) -> Self {
        self.execution = execution;
        self
    }

    /// Adds metadata.
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AnalogError> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Builds and fully validates the analog program.
    pub fn build(self) -> Result<AnalogProgram, AnalogError> {
        let register = self.register.ok_or(AnalogError::MissingRegister)?;

        let duration_seconds = self.duration_seconds.ok_or(
            AnalogError::EmptyField {
                field: "program.duration_seconds",
            },
        )?;

        let mut program =
            AnalogProgram::new(register, self.terms, duration_seconds)?;

        if let Some(name) = self.name {
            program = program.with_name(name)?;
        }

        program.execution = self.execution;
        program.metadata = self.metadata;

        for observable in self.observables {
            program.add_observable(observable)?;
        }

        program.validate()?;
        Ok(program)
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn validate_equal_end_time(
    field_a: &'static str,
    end_a: Option<f64>,
    end_b: Option<f64>,
) -> Result<(), AnalogError> {
    let end_a = end_a.ok_or(AnalogError::EmptyField { field: field_a })?;
    let end_b = end_b.ok_or(AnalogError::EmptyField { field: field_a })?;

    if end_a != end_b {
        return Err(AnalogError::DurationMismatch {
            program_duration: end_a,
            last_time: end_b,
        });
    }

    Ok(())
}

fn validate_equal_duration(
    field: &'static str,
    end_time: f64,
    duration: f64,
) -> Result<(), AnalogError> {
    if end_time != duration {
        return Err(AnalogError::ControlDurationMismatch {
            field,
            last_time: end_time,
            program_duration: duration,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn point(time: f64, value: f64) -> TimePoint {
        TimePoint::new(time, value).expect("test point must be valid")
    }

    fn basic_register() -> AnalogRegister {
        AnalogRegister::from_resources(vec![
            AnalogResource::new(0, Position::xy(0.0, 0.0).unwrap()).unwrap(),
            AnalogResource::new(1, Position::xy(5.0e-6, 0.0).unwrap()).unwrap(),
        ])
        .unwrap()
    }

    fn basic_driving_field() -> DrivingField {
        DrivingField::new(
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0e-6, 1.0)],
            )
            .unwrap(),
            TimeSeries::from_points(
                Interpolation::Step,
                vec![point(0.0, 0.0), point(1.0e-6, 0.0)],
            )
            .unwrap(),
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, -1.0), point(1.0e-6, 1.0)],
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn time_series_requires_strictly_increasing_time() {
        let result = TimeSeries::from_points(
            Interpolation::Linear,
            vec![point(0.0, 0.0), point(0.0, 1.0)],
        );

        assert!(matches!(
            result,
            Err(AnalogError::NonMonotonicTime { .. })
        ));
    }

    #[test]
    fn time_series_rejects_negative_time() {
        let result = TimePoint::new(-1.0, 0.0);

        assert!(matches!(result, Err(AnalogError::ValueOutOfRange { .. })));
    }

    #[test]
    fn position_distance_is_deterministic() {
        let first = Position::xy(0.0, 0.0).unwrap();
        let second = Position::xy(3.0, 4.0).unwrap();

        assert_eq!(first.distance_meters(second), 5.0);
    }

    #[test]
    fn register_rejects_duplicate_ids() {
        let result = AnalogRegister::from_resources(vec![
            AnalogResource::new(0, Position::xy(0.0, 0.0).unwrap()).unwrap(),
            AnalogResource::new(0, Position::xy(1.0, 0.0).unwrap()).unwrap(),
        ]);

        assert!(matches!(
            result,
            Err(AnalogError::DuplicateResourceId { id: 0 })
        ));
    }

    #[test]
    fn register_rejects_coincident_resources() {
        let result = AnalogRegister::from_resources(vec![
            AnalogResource::new(0, Position::xy(0.0, 0.0).unwrap()).unwrap(),
            AnalogResource::new(1, Position::xy(0.0, 0.0).unwrap()).unwrap(),
        ]);

        assert!(matches!(
            result,
            Err(AnalogError::CoincidentResources { .. })
        ));
    }

    #[test]
    fn driving_field_requires_matching_end_times() {
        let result = DrivingField::new(
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0, 1.0)],
            )
            .unwrap(),
            TimeSeries::from_points(
                Interpolation::Step,
                vec![point(0.0, 0.0), point(2.0, 0.0)],
            )
            .unwrap(),
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0, 0.0)],
            )
            .unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn local_detuning_requires_correct_mask_size() {
        let field = LocalDetuningField::new(
            vec![true],
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0e-6, 1.0)],
            )
            .unwrap(),
        )
        .unwrap();

        assert!(matches!(
            field.validate_for_register(2),
            Err(AnalogError::InvalidResourceMask { .. })
        ));
    }

    #[test]
    fn pair_interaction_rejects_self_interaction() {
        let result = PairInteractionTerm::new(
            0,
            0,
            PairInteraction::VanDerWaals { strength: 1.0 },
        );

        assert!(matches!(
            result,
            Err(AnalogError::SelfInteraction { resource: 0 })
        ));
    }

    #[test]
    fn program_validates() {
        let program = AnalogProgram::new(
            basic_register(),
            vec![HamiltonianTerm::DrivingField(
                basic_driving_field(),
            )],
            1.0e-6,
        )
        .unwrap();

        program.validate().unwrap();
    }

    #[test]
    fn program_requires_analog_capabilities() {
        let program = AnalogProgram::new(
            basic_register(),
            vec![HamiltonianTerm::DrivingField(
                basic_driving_field(),
            )],
            1.0e-6,
        )
        .unwrap();

        let capabilities = program.required_capabilities();

        assert!(capabilities.contains("analog_control"));
        assert!(capabilities.contains("analog_hamiltonian_simulation"));
        assert!(capabilities.contains("time_dependent_control"));
    }

    #[test]
    fn local_control_adds_spatial_requirements() {
        let field = LocalDetuningField::new(
            vec![true, false],
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0e-6, -1.0)],
            )
            .unwrap(),
        )
        .unwrap();

        let program = AnalogProgram::new(
            basic_register(),
            vec![
                HamiltonianTerm::DrivingField(basic_driving_field()),
                HamiltonianTerm::LocalDetuning(field),
            ],
            1.0e-6,
        )
        .unwrap();

        let capabilities = program.required_capabilities();

        assert!(capabilities.contains("spatial_control"));
        assert!(capabilities.contains("local_control"));
    }

    #[test]
    fn experimental_spatial_control_is_explicit() {
        let field = SpatialControlField::new(
            "local_detuning",
            vec![true, false],
            TimeSeries::from_points(
                Interpolation::Linear,
                vec![point(0.0, 0.0), point(1.0e-6, -1.0)],
            )
            .unwrap(),
        )
        .unwrap()
        .experimental(true);

        let program = AnalogProgram::new(
            basic_register(),
            vec![
                HamiltonianTerm::DrivingField(basic_driving_field()),
                HamiltonianTerm::SpatialControl(field),
            ],
            1.0e-6,
        )
        .unwrap();

        assert!(program.requires_experimental_capabilities());
        assert!(
            program
                .required_capabilities()
                .contains("experimental_analog_control")
        );
    }

    #[test]
    fn observable_resource_indices_are_checked() {
        let observable = AnalogObservable::new(
            "z0",
            vec![ObservableTerm::new(
                5,
                ObservableOperator::Z,
                1.0,
            )
            .unwrap()],
        )
        .unwrap();

        let result = observable.validate_for_register(2);

        assert!(matches!(
            result,
            Err(AnalogError::InvalidObservableResource {
                resource: 5,
                register_size: 2
            })
        ));
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = AnalogMetadata::new();

        metadata.insert("z", "last").unwrap();
        metadata.insert("a", "first").unwrap();

        let keys: Vec<&String> = metadata.values.keys().collect();

        assert_eq!(keys, vec![&"a".to_owned(), &"z".to_owned()]);
    }

    #[test]
    fn builder_constructs_valid_program() {
        let program = AnalogProgramBuilder::new()
            .name("test-ahs")
            .register(basic_register())
            .term(HamiltonianTerm::DrivingField(
                basic_driving_field(),
            ))
            .duration_seconds(1.0e-6)
            .build()
            .unwrap();

        assert_eq!(program.resource_count(), 2);
        assert_eq!(program.duration_seconds, 1.0e-6);
    }

    #[test]
    fn resource_estimate_is_deterministic() {
        let program = AnalogProgramBuilder::new()
            .register(basic_register())
            .term(HamiltonianTerm::DrivingField(
                basic_driving_field(),
            ))
            .duration_seconds(1.0e-6)
            .build()
            .unwrap();

        let estimate = program.estimate_resources().unwrap();

        assert_eq!(estimate.resource_count, 2);
        assert_eq!(estimate.term_count, 1);
        assert_eq!(estimate.duration_seconds, 1.0e-6);
    }

    #[test]
    fn canonicalization_preserves_resource_order() {
        let program = AnalogProgramBuilder::new()
            .register(basic_register())
            .term(HamiltonianTerm::DrivingField(
                basic_driving_field(),
            ))
            .duration_seconds(1.0e-6)
            .build()
            .unwrap();

        let canonical = program.canonicalize().unwrap();

        assert_eq!(
            canonical.register.resources[0].id,
            program.register.resources[0].id
        );
        assert_eq!(
            canonical.register.resources[1].id,
            program.register.resources[1].id
        );
    }

    #[test]
    fn serde_round_trip_preserves_program() {
        let program = AnalogProgramBuilder::new()
            .register(basic_register())
            .term(HamiltonianTerm::DrivingField(
                basic_driving_field(),
            ))
            .duration_seconds(1.0e-6)
            .build()
            .unwrap();

        let encoded = serde_json::to_string(&program).unwrap();
        let decoded: AnalogProgram =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, program);
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let result = Position::new(f64::NAN, 0.0, 0.0);

        assert!(matches!(
            result,
            Err(AnalogError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn zero_shots_are_rejected() {
        let options = AnalogExecutionOptions {
            shots: 0,
            ..Default::default()
        };

        assert!(options.validate().is_err());
    }
}