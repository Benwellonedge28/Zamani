//! Zamani Quantum Noise (ZQN) — Temporal Noise Semantics
//!
//! # Purpose
//!
//! This module defines backend-independent temporal noise semantics for ZQN.
//!
//! Temporal noise answers:
//!
//! > "How does a noise process depend on physical/semantic time, temporal
//! > history, or explicitly supplied temporal state?"
//!
//! This module provides the semantic vocabulary required to represent:
//!
//! - time-dependent noise;
//! - stationary temporal noise;
//! - non-stationary temporal noise;
//! - temporal drift;
//! - piecewise temporal behavior;
//! - periodic behavior;
//! - exponential relaxation/correlation;
//! - power-law correlation;
//! - arbitrary sampled temporal profiles;
//! - deterministic temporal evaluation;
//! - temporal correlation kernels;
//! - temporal validity windows;
//! - temporal composition;
//! - resource-scoped temporal behavior.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             │ canonical operation/resource identity
//!                             ▼
//!                    ┌──────────────────┐
//!                    │      ZQN         │
//!                    │                  │
//!                    │ temporal.rs      │
//!                    └────────┬─────────┘
//!                             │
//!            ┌────────────────┼─────────────────┐
//!            │                │                 │
//!            ▼                ▼                 ▼
//!        noise/model      calibration       simulation
//!            │                │                 │
//!            ▼                ▼                 ▼
//!        application       drift data       realization
//!            │
//!      ┌─────┼─────┐
//!      ▼     ▼     ▼
//!   routing scheduling QEC
//! ```
//!
//! `temporal.rs` is a semantic subsystem. It does not execute quantum state
//! evolution and does not own stochastic sampling.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - temporal coordinates;
//! - temporal intervals;
//! - temporal domains;
//! - temporal scopes;
//! - temporal profiles;
//! - temporal interpolation;
//! - temporal correlation kernels;
//! - drift functions;
//! - temporal composition;
//! - temporal validation;
//! - deterministic evaluation of temporal functions;
//! - resource-scoped temporal metadata.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum channels;
//! - Kraus/Choi/superoperator mathematics;
//! - probability distributions;
//! - random-number generation;
//! - stochastic sampling;
//! - calibration storage;
//! - hardware APIs;
//! - routing;
//! - scheduling;
//! - QEC;
//! - simulation engines;
//! - serialization formats;
//! - global registries;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective ZQN/quantum subsystems.
//!
//! # Canonical quantum identities
//!
//! When temporal noise is associated with a canonical quantum resource, this
//! module uses the identities owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define another qubit identity type.
//!
//! A temporal profile may therefore be scoped to:
//!
//! - a logical qubit;
//! - a physical qubit;
//! - a resource-independent temporal domain;
//! - an arbitrary ZQN-owned scope supplied by a higher layer.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TIME
//! MAX_SAMPLES
//! MAX_SEGMENTS
//! MAX_CORRELATIONS
//! MAX_RESOURCES
//! ```
//!
//! The semantic model has no finite machine-size ceiling.
//!
//! Large temporal datasets should be represented by callers using streaming,
//! bounded evaluation, external storage, or other resource-aware mechanisms.
//!
//! A temporal profile is not required to materialize every point in time.
//!
//! # Time representation
//!
//! Temporal coordinates use seconds represented as `f64`.
//!
//! This is a semantic coordinate representation, not a claim that every
//! hardware clock has floating-point precision.
//!
//! Higher precision or hardware-native time representations may be converted
//! into this interface by an integration layer.
//!
//! All public constructors validate:
//!
//! - finiteness;
//! - non-negative durations where required;
//! - valid ordering;
//! - valid interpolation domains;
//! - valid correlation parameters.
//!
//! NaN and infinity are rejected rather than silently normalized.
//!
//! # Determinism
//!
//! Temporal evaluation is deterministic.
//!
//! It does not:
//!
//! - read wall-clock time;
//! - access a global clock;
//! - access an RNG;
//! - use thread identity;
//! - depend on hash-map ordering;
//! - use memory addresses;
//! - mutate global state.
//!
//! The caller supplies the temporal coordinate explicitly.
//!
//! This is required so that:
//!
//! ```text
//! sequential execution
//!
//! and
//!
//! parallel execution
//! ```
//!
//! can evaluate the same temporal semantics identically.
//!
//! # Physical time versus wall-clock time
//!
//! This module never obtains system time implicitly.
//!
//! A runtime/hardware/calibration subsystem must explicitly provide the physical
//! time coordinate when physical time matters.
//!
//! This prevents otherwise identical executions from becoming nondeterministic
//! merely because they were started at different wall-clock times.
//!
//! # Approximation
//!
//! Interpolation and sampled profiles are explicitly approximate unless the
//! caller declares that the sampled representation itself is the authoritative
//! semantic representation.
//!
//! This module never silently presents interpolation as exact physical truth.
//!
//! # Resource scope
//!
//! Temporal behavior may be associated with canonical logical or physical
//! qubits. It may also be resource-independent.
//!
//! The number of resources is data, not a type-level architectural limit.
//!
//! # Integration with `noise/model.rs`
//!
//! `noise/model.rs` owns the general `NoiseModel` contract.
//!
//! Temporal noise implementations may use this module to evaluate the
//! time-dependent parameters required by a model.
//!
//! `temporal.rs` does not modify the `NoiseModel` trait.
//!
//! This preserves API stability.
//!
//! # Integration with calibration
//!
//! Calibration may provide temporal samples or drift parameters.
//!
//! Calibration remains the owner of:
//!
//! - calibration snapshots;
//! - provenance;
//! - device-specific parameter identity;
//! - validity policy.
//!
//! Temporal semantics consume those values after explicit conversion.
//!
//! # Integration with scheduling
//!
//! Scheduling may query a temporal profile at an explicitly supplied time:
//!
//! ```text
//! scheduled start time
//!        │
//!        ▼
//! TemporalProfile::evaluate
//!        │
//!        ▼
//! time-dependent noise parameter
//! ```
//!
//! The scheduler remains responsible for determining when operations execute.
//!
//! # Integration with simulation
//!
//! Simulation may evaluate temporal functions while applying channels/faults.
//!
//! This module does not know how those values are converted into quantum-state
//! evolution.
//!
//! # Integration with routing
//!
//! Routing may use temporal noise values when constructing cost functions.
//!
//! This module does not know the routing algorithm.
//!
//! # Integration with QEC
//!
//! QEC may evaluate temporal physical-error parameters when generating faults
//! or estimating logical error behavior.
//!
//! QEC remains responsible for fault-tolerant semantics.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may evaluate temporal profiles repeatedly to detect:
//!
//! - drift;
//! - aging;
//! - periodic effects;
//! - temporal correlations;
//! - calibration degradation.
//!
//! Benchmarking remains responsible for experiment orchestration and metrics.
//!
//! # Integration with hardware
//!
//! Hardware adapters may translate hardware-native clocks/calibration data into
//! the explicit `Time` representation used here.
//!
//! This module never calls hardware APIs.
//!
//! # Serialization
//!
//! This module does not implement a wire format.
//!
//! `zqn::io` owns serialization.
//!
//! All public semantic values are ordinary value types whose fields can be
//! represented explicitly by the serialization layer.
//!
//! Rust memory layout is not a serialization contract.
//!
//! # Security
//!
//! Temporal specifications can originate from untrusted configuration.
//!
//! Constructors therefore reject invalid numerical values.
//!
//! Evaluation is bounded by the amount of data supplied by the caller.
//!
//! This module does not perform uncontrolled allocation, recursion, I/O, or
//! process execution.
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
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Error type
// ============================================================================

/// Errors produced by temporal-noise semantic operations.
///
/// This local error type intentionally remains independent from the evolving
/// ZQN-wide error taxonomy so this file can be completed independently.
///
/// Integration layers may convert these errors into `ZqnError` without
/// requiring changes to the temporal semantics themselves.
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalNoiseError {
    /// A supplied floating-point value was NaN or infinite.
    NonFinite {
        /// Name of the invalid value.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A duration was negative.
    NegativeDuration {
        /// Name of the offending field.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// An interval's end precedes its start.
    InvalidInterval {
        /// Interval start.
        start: f64,
        /// Interval end.
        end: f64,
    },

    /// A correlation time must be strictly positive.
    InvalidCorrelationTime {
        /// Invalid correlation time.
        value: f64,
    },

    /// A decay/rate parameter was invalid.
    InvalidRate {
        /// Parameter name.
        field: &'static str,
        /// Invalid value.
        value: f64,
    },

    /// A period must be strictly positive.
    InvalidPeriod {
        /// Invalid period.
        value: f64,
    },

    /// A sample sequence does not satisfy the temporal-profile invariants.
    InvalidSamples {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A requested interpolation coordinate lies outside the profile domain.
    OutOfDomain {
        /// Requested coordinate.
        time: f64,
        /// Lower domain boundary.
        start: f64,
        /// Upper domain boundary.
        end: f64,
    },

    /// A numerical operation would overflow.
    NumericalOverflow {
        /// Human-readable operation name.
        operation: &'static str,
    },

    /// The caller supplied a zero-length collection where at least one item is
    /// required.
    EmptyInput {
        /// Name of the input.
        field: &'static str,
    },
}

impl fmt::Display for TemporalNoiseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(formatter, "{field} must be finite, got {value}")
            }
            Self::NegativeDuration { field, value } => {
                write!(formatter, "{field} must be non-negative, got {value}")
            }
            Self::InvalidInterval { start, end } => {
                write!(
                    formatter,
                    "invalid temporal interval: start {start} is after end {end}"
                )
            }
            Self::InvalidCorrelationTime { value } => {
                write!(
                    formatter,
                    "correlation time must be finite and strictly positive, got {value}"
                )
            }
            Self::InvalidRate { field, value } => {
                write!(
                    formatter,
                    "{field} must be finite and non-negative, got {value}"
                )
            }
            Self::InvalidPeriod { value } => {
                write!(
                    formatter,
                    "period must be finite and strictly positive, got {value}"
                )
            }
            Self::InvalidSamples { reason } => {
                write!(formatter, "invalid temporal samples: {reason}")
            }
            Self::OutOfDomain {
                time,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "time {time} is outside temporal domain [{start}, {end}]"
                )
            }
            Self::NumericalOverflow { operation } => {
                write!(formatter, "numerical overflow while performing {operation}")
            }
            Self::EmptyInput { field } => {
                write!(formatter, "{field} must not be empty")
            }
        }
    }
}

/// Result type for temporal-noise operations.
pub type TemporalNoiseResult<T> = Result<T, TemporalNoiseError>;

// ============================================================================
// Time
// ============================================================================

/// Explicit physical/semantic time coordinate in seconds.
///
/// `Time` is intentionally not tied to a hardware clock.
///
/// A runtime or hardware integration layer supplies the coordinate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time(f64);

impl Time {
    /// Creates a time coordinate.
    pub fn new(seconds: f64) -> TemporalNoiseResult<Self> {
        validate_finite("seconds", seconds)?;
        Ok(Self(seconds))
    }

    /// Creates the zero-time coordinate.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the coordinate in seconds.
    #[must_use]
    pub const fn seconds(self) -> f64 {
        self.0
    }

    /// Returns the time difference `self - earlier`.
    ///
    /// Fails if the result would be negative.
    pub fn duration_since(self, earlier: Self) -> TemporalNoiseResult<Duration> {
        let seconds = self.0 - earlier.0;

        if !seconds.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "time subtraction",
            });
        }

        if seconds < 0.0 {
            return Err(TemporalNoiseError::InvalidInterval {
                start: self.0,
                end: earlier.0,
            });
        }

        Duration::new(seconds)
    }

    /// Adds a non-negative duration.
    pub fn checked_add(self, duration: Duration) -> TemporalNoiseResult<Self> {
        let value = self.0 + duration.seconds();

        if !value.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "time addition",
            });
        }

        Self::new(value)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}s", self.0)
    }
}

// ============================================================================
// Duration
// ============================================================================

/// Non-negative temporal duration in seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration(f64);

impl Duration {
    /// Creates a duration.
    pub fn new(seconds: f64) -> TemporalNoiseResult<Self> {
        validate_finite("seconds", seconds)?;

        if seconds < 0.0 {
            return Err(TemporalNoiseError::NegativeDuration {
                field: "seconds",
                value: seconds,
            });
        }

        Ok(Self(seconds))
    }

    /// Zero duration.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns seconds.
    #[must_use]
    pub const fn seconds(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}s", self.0)
    }
}

// ============================================================================
// Temporal interval
// ============================================================================

/// A closed temporal interval.
///
/// Both endpoints are inclusive for semantic membership.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalInterval {
    start: Time,
    end: Time,
}

impl TemporalInterval {
    /// Creates an interval.
    pub fn new(start: Time, end: Time) -> TemporalNoiseResult<Self> {
        if end.seconds() < start.seconds() {
            return Err(TemporalNoiseError::InvalidInterval {
                start: start.seconds(),
                end: end.seconds(),
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the start.
    #[must_use]
    pub const fn start(self) -> Time {
        self.start
    }

    /// Returns the end.
    #[must_use]
    pub const fn end(self) -> Time {
        self.end
    }

    /// Returns the interval duration.
    pub fn duration(self) -> TemporalNoiseResult<Duration> {
        self.end.duration_since(self.start)
    }

    /// Returns whether a time belongs to this closed interval.
    #[must_use]
    pub fn contains(self, time: Time) -> bool {
        time.seconds() >= self.start.seconds() && time.seconds() <= self.end.seconds()
    }
}

// ============================================================================
// Temporal domain
// ============================================================================

/// Defines whether evaluation outside a temporal profile's domain is allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainPolicy {
    /// Evaluation outside the domain is an error.
    Reject,

    /// Values outside the domain use the nearest endpoint.
    Clamp,

    /// The profile repeats periodically.
    Repeat,
}

// ============================================================================
// Temporal scope
// ============================================================================

/// Resource scope for temporal noise.
///
/// Canonical IR qubit IDs are used where applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalScope {
    /// Applies independently of a particular canonical resource.
    Global,

    /// Applies to one logical qubit.
    LogicalQubit(QubitId),

    /// Applies to one physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl TemporalScope {
    /// Returns true if this scope is global.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns true if this scope identifies a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true if this scope identifies a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

// ============================================================================
// Temporal evaluation
// ============================================================================

/// Result of evaluating a temporal noise function.
///
/// The value is intentionally a scalar parameter rather than a probability or
/// channel. Interpretation belongs to the consumer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalValue {
    value: f64,
}

impl TemporalValue {
    /// Creates a finite temporal value.
    pub fn new(value: f64) -> TemporalNoiseResult<Self> {
        validate_finite("value", value)?;
        Ok(Self { value })
    }

    /// Returns the value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

// ============================================================================
// Temporal function trait
// ============================================================================

/// Deterministic temporal function.
///
/// Implementations map an explicit time coordinate to a scalar semantic value.
///
/// The trait intentionally has no RNG, clock, hardware, or runtime dependency.
pub trait TemporalFunction: Send + Sync {
    /// Evaluates the function at the supplied time.
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue>;

    /// Returns the semantic domain of the function.
    fn domain(&self) -> Option<TemporalInterval>;

    /// Returns the declared function kind.
    fn kind(&self) -> TemporalFunctionKind;
}

/// Describes the mathematical family of a temporal function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalFunctionKind {
    /// Constant/stationary function.
    Constant,

    /// Linear drift.
    LinearDrift,

    /// Exponential drift/relaxation.
    Exponential,

    /// Power-law behavior.
    PowerLaw,

    /// Periodic behavior.
    Periodic,

    /// Piecewise function.
    Piecewise,

    /// Interpolated sampled profile.
    Sampled,

    /// Composition of temporal functions.
    Composite,
}

// ============================================================================
// Constant temporal function
// ============================================================================

/// Stationary temporal function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantTemporal {
    value: TemporalValue,
}

impl ConstantTemporal {
    /// Creates a stationary temporal function.
    pub fn new(value: f64) -> TemporalNoiseResult<Self> {
        Ok(Self {
            value: TemporalValue::new(value)?,
        })
    }

    /// Returns the constant value.
    #[must_use]
    pub const fn value(&self) -> TemporalValue {
        self.value
    }
}

impl TemporalFunction for ConstantTemporal {
    fn evaluate(&self, _time: Time) -> TemporalNoiseResult<TemporalValue> {
        Ok(self.value)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        None
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::Constant
    }
}

// ============================================================================
// Linear drift
// ============================================================================

/// Linear temporal drift.
///
/// ```text
/// f(t) = initial + rate * (t - reference)
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearDrift {
    initial: TemporalValue,
    rate_per_second: f64,
    reference: Time,
    domain: Option<TemporalInterval>,
}

impl LinearDrift {
    /// Creates an unrestricted linear drift.
    pub fn new(
        initial: f64,
        rate_per_second: f64,
        reference: Time,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("rate_per_second", rate_per_second)?;

        Ok(Self {
            initial: TemporalValue::new(initial)?,
            rate_per_second,
            reference,
            domain: None,
        })
    }

    /// Restricts the drift to an explicit temporal domain.
    pub fn with_domain(
        mut self,
        domain: TemporalInterval,
    ) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Returns the initial value.
    #[must_use]
    pub const fn initial(&self) -> TemporalValue {
        self.initial
    }

    /// Returns the drift rate.
    #[must_use]
    pub const fn rate_per_second(&self) -> f64 {
        self.rate_per_second
    }

    /// Returns the reference time.
    #[must_use]
    pub const fn reference(&self) -> Time {
        self.reference
    }
}

impl TemporalFunction for LinearDrift {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        if let Some(domain) = self.domain {
            if !domain.contains(time) {
                return Err(TemporalNoiseError::OutOfDomain {
                    time: time.seconds(),
                    start: domain.start().seconds(),
                    end: domain.end().seconds(),
                });
            }
        }

        let elapsed = time.seconds() - self.reference.seconds();
        let change = self.rate_per_second * elapsed;

        if !change.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "linear drift evaluation",
            });
        }

        let value = self.initial.value() + change;

        TemporalValue::new(value)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        self.domain
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::LinearDrift
    }
}

// ============================================================================
// Exponential temporal function
// ============================================================================

/// Exponential temporal behavior.
///
/// ```text
/// f(t) = baseline + amplitude * exp(-rate * elapsed)
/// ```
///
/// `elapsed` is measured from `reference`.
///
/// A negative elapsed value is valid; this permits explicit backward
/// evaluation when the caller supplies a temporal domain that allows it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExponentialTemporal {
    baseline: TemporalValue,
    amplitude: f64,
    rate_per_second: f64,
    reference: Time,
    domain: Option<TemporalInterval>,
}

impl ExponentialTemporal {
    /// Creates exponential temporal behavior.
    ///
    /// `rate_per_second` must be non-negative.
    pub fn new(
        baseline: f64,
        amplitude: f64,
        rate_per_second: f64,
        reference: Time,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;
        validate_non_negative("rate_per_second", rate_per_second)?;

        Ok(Self {
            baseline: TemporalValue::new(baseline)?,
            amplitude,
            rate_per_second,
            reference,
            domain: None,
        })
    }

    /// Restricts evaluation to an explicit domain.
    pub fn with_domain(mut self, domain: TemporalInterval) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Returns the baseline.
    #[must_use]
    pub const fn baseline(&self) -> TemporalValue {
        self.baseline
    }

    /// Returns the amplitude.
    #[must_use]
    pub const fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// Returns the decay/growth rate.
    #[must_use]
    pub const fn rate_per_second(&self) -> f64 {
        self.rate_per_second
    }
}

impl TemporalFunction for ExponentialTemporal {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        if let Some(domain) = self.domain {
            if !domain.contains(time) {
                return Err(TemporalNoiseError::OutOfDomain {
                    time: time.seconds(),
                    start: domain.start().seconds(),
                    end: domain.end().seconds(),
                });
            }
        }

        let elapsed = time.seconds() - self.reference.seconds();
        let exponent = -self.rate_per_second * elapsed;

        if !exponent.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "exponential exponent evaluation",
            });
        }

        let factor = exponent.exp();

        if !factor.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "exponential evaluation",
            });
        }

        let contribution = self.amplitude * factor;

        if !contribution.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "exponential contribution",
            });
        }

        TemporalValue::new(self.baseline.value() + contribution)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        self.domain
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::Exponential
    }
}

// ============================================================================
// Power-law temporal function
// ============================================================================

/// Power-law temporal behavior.
///
/// ```text
/// f(t) = baseline + amplitude * (1 + elapsed / scale)^(-exponent)
/// ```
///
/// The domain is constrained so the base remains strictly positive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerLawTemporal {
    baseline: TemporalValue,
    amplitude: f64,
    scale_seconds: f64,
    exponent: f64,
    reference: Time,
    domain: Option<TemporalInterval>,
}

impl PowerLawTemporal {
    /// Creates power-law temporal behavior.
    pub fn new(
        baseline: f64,
        amplitude: f64,
        scale_seconds: f64,
        exponent: f64,
        reference: Time,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;
        validate_finite("exponent", exponent)?;

        if !scale_seconds.is_finite() || scale_seconds <= 0.0 {
            return Err(TemporalNoiseError::InvalidRate {
                field: "scale_seconds",
                value: scale_seconds,
            });
        }

        Ok(Self {
            baseline: TemporalValue::new(baseline)?,
            amplitude,
            scale_seconds,
            exponent,
            reference,
            domain: None,
        })
    }

    /// Restricts the model to a domain.
    pub fn with_domain(mut self, domain: TemporalInterval) -> Self {
        self.domain = Some(domain);
        self
    }
}

impl TemporalFunction for PowerLawTemporal {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        if let Some(domain) = self.domain {
            if !domain.contains(time) {
                return Err(TemporalNoiseError::OutOfDomain {
                    time: time.seconds(),
                    start: domain.start().seconds(),
                    end: domain.end().seconds(),
                });
            }
        }

        let elapsed = time.seconds() - self.reference.seconds();
        let base = 1.0 + elapsed / self.scale_seconds;

        if !base.is_finite() || base <= 0.0 {
            return Err(TemporalNoiseError::InvalidRate {
                field: "power-law base",
                value: base,
            });
        }

        let factor = base.powf(-self.exponent);

        if !factor.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "power-law evaluation",
            });
        }

        let contribution = self.amplitude * factor;

        if !contribution.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "power-law contribution",
            });
        }

        TemporalValue::new(self.baseline.value() + contribution)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        self.domain
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::PowerLaw
    }
}

// ============================================================================
// Periodic temporal function
// ============================================================================

/// Periodic temporal behavior.
///
/// ```text
/// f(t) = offset + amplitude * sin(2π * phase)
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PeriodicTemporal {
    offset: TemporalValue,
    amplitude: f64,
    period_seconds: f64,
    phase_seconds: f64,
    domain: Option<TemporalInterval>,
}

impl PeriodicTemporal {
    /// Creates periodic behavior.
    ///
    /// `period_seconds` must be strictly positive.
    pub fn new(
        offset: f64,
        amplitude: f64,
        period_seconds: f64,
        phase_seconds: f64,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;
        validate_finite("phase_seconds", phase_seconds)?;

        if !period_seconds.is_finite() || period_seconds <= 0.0 {
            return Err(TemporalNoiseError::InvalidPeriod {
                value: period_seconds,
            });
        }

        Ok(Self {
            offset: TemporalValue::new(offset)?,
            amplitude,
            period_seconds,
            phase_seconds,
            domain: None,
        })
    }

    /// Restricts evaluation to a domain.
    pub fn with_domain(mut self, domain: TemporalInterval) -> Self {
        self.domain = Some(domain);
        self
    }
}

impl TemporalFunction for PeriodicTemporal {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        if let Some(domain) = self.domain {
            if !domain.contains(time) {
                return Err(TemporalNoiseError::OutOfDomain {
                    time: time.seconds(),
                    start: domain.start().seconds(),
                    end: domain.end().seconds(),
                });
            }
        }

        let phase = (time.seconds() - self.phase_seconds) / self.period_seconds;

        if !phase.is_finite() {
            return Err(TemporalNoiseError::NumericalOverflow {
                operation: "periodic phase evaluation",
            });
        }

        let angle = core::f64::consts::TAU * phase;
        let value = self.offset.value() + self.amplitude * angle.sin();

        TemporalValue::new(value)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        self.domain
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::Periodic
    }
}

// ============================================================================
// Temporal samples
// ============================================================================

/// One authoritative temporal sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalSample {
    time: Time,
    value: TemporalValue,
}

impl TemporalSample {
    /// Creates a temporal sample.
    pub fn new(time: Time, value: f64) -> TemporalNoiseResult<Self> {
        Ok(Self {
            time,
            value: TemporalValue::new(value)?,
        })
    }

    /// Returns the time.
    #[must_use]
    pub const fn time(self) -> Time {
        self.time
    }

    /// Returns the value.
    #[must_use]
    pub const fn value(self) -> TemporalValue {
        self.value
    }
}

// ============================================================================
// Interpolation
// ============================================================================

/// Interpolation strategy for sampled temporal data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interpolation {
    /// Nearest sample.
    Nearest,

    /// Linear interpolation.
    Linear,
}

// ============================================================================
// Sampled temporal profile
// ============================================================================

/// Explicit finite sampled temporal profile.
///
/// Samples are required to be strictly increasing in time.
///
/// The implementation stores only the samples supplied by the caller. It does
/// not generate an artificial time grid.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledTemporal {
    samples: Vec<TemporalSample>,
    interpolation: Interpolation,
    domain_policy: DomainPolicy,
}

impl SampledTemporal {
    /// Creates a sampled temporal profile.
    ///
    /// At least one sample is required.
    pub fn new(
        samples: Vec<TemporalSample>,
        interpolation: Interpolation,
        domain_policy: DomainPolicy,
    ) -> TemporalNoiseResult<Self> {
        if samples.is_empty() {
            return Err(TemporalNoiseError::EmptyInput {
                field: "samples",
            });
        }

        validate_samples(&samples)?;

        Ok(Self {
            samples,
            interpolation,
            domain_policy,
        })
    }

    /// Returns the number of supplied samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether the profile contains no samples.
    ///
    /// This is always false for successfully constructed values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns the interpolation strategy.
    #[must_use]
    pub const fn interpolation(&self) -> Interpolation {
        self.interpolation
    }

    /// Returns the domain policy.
    #[must_use]
    pub const fn domain_policy(&self) -> DomainPolicy {
        self.domain_policy
    }

    /// Returns the first sample.
    #[must_use]
    pub fn first(&self) -> TemporalSample {
        self.samples[0]
    }

    /// Returns the last sample.
    #[must_use]
    pub fn last(&self) -> TemporalSample {
        self.samples[self.samples.len() - 1]
    }

    /// Returns a read-only view of the samples.
    #[must_use]
    pub fn samples(&self) -> &[TemporalSample] {
        &self.samples
    }

    fn normalized_time(&self, time: Time) -> TemporalNoiseResult<Time> {
        let first = self.first().time();
        let last = self.last().time();

        if time.seconds() >= first.seconds() && time.seconds() <= last.seconds() {
            return Ok(time);
        }

        match self.domain_policy {
            DomainPolicy::Reject => Err(TemporalNoiseError::OutOfDomain {
                time: time.seconds(),
                start: first.seconds(),
                end: last.seconds(),
            }),

            DomainPolicy::Clamp => {
                if time.seconds() < first.seconds() {
                    Ok(first)
                } else {
                    Ok(last)
                }
            }

            DomainPolicy::Repeat => {
                let period = last.seconds() - first.seconds();

                if period <= 0.0 {
                    // A single-point profile has no meaningful period. Repeating
                    // it is therefore equivalent to returning its only sample.
                    return Ok(first);
                }

                let relative = time.seconds() - first.seconds();

                if !relative.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "periodic sample normalization",
                    });
                }

                let wrapped = relative.rem_euclid(period);
                let normalized = first.seconds() + wrapped;

                Time::new(normalized)
            }
        }
    }
}

impl TemporalFunction for SampledTemporal {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        let time = self.normalized_time(time)?;
        let target = time.seconds();

        if self.samples.len() == 1 {
            return Ok(self.samples[0].value());
        }

        if target <= self.first().time().seconds() {
            return Ok(self.first().value());
        }

        if target >= self.last().time().seconds() {
            return Ok(self.last().value());
        }

        match self
            .samples
            .binary_search_by(|sample| sample.time().seconds().total_cmp(&target))
        {
            Ok(index) => Ok(self.samples[index].value()),

            Err(index) => {
                let right = self.samples[index];
                let left = self.samples[index - 1];

                match self.interpolation {
                    Interpolation::Nearest => {
                        let left_distance = target - left.time().seconds();
                        let right_distance = right.time().seconds() - target;

                        if left_distance <= right_distance {
                            Ok(left.value())
                        } else {
                            Ok(right.value())
                        }
                    }

                    Interpolation::Linear => {
                        let span = right.time().seconds() - left.time().seconds();

                        if span <= 0.0 || !span.is_finite() {
                            return Err(TemporalNoiseError::InvalidSamples {
                                reason: "sample times must be strictly increasing",
                            });
                        }

                        let fraction =
                            (target - left.time().seconds()) / span;

                        if !fraction.is_finite() {
                            return Err(TemporalNoiseError::NumericalOverflow {
                                operation: "linear interpolation",
                            });
                        }

                        let value = left.value().value()
                            + (right.value().value() - left.value().value()) * fraction;

                        TemporalValue::new(value)
                    }
                }
            }
        }
    }

    fn domain(&self) -> Option<TemporalInterval> {
        Some(
            TemporalInterval::new(
                self.first().time(),
                self.last().time(),
            )
            // The constructor cannot fail because samples were validated.
            .expect("validated temporal samples must define a valid interval"),
        )
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::Sampled
    }
}

// ============================================================================
// Temporal correlation kernel
// ============================================================================

/// Temporal correlation kernel.
///
/// A correlation kernel describes how correlation strength changes with a time
/// separation. It is not itself a probability distribution or quantum channel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemporalCorrelationKernel {
    /// Exponential correlation:
    ///
    /// ```text
    /// C(Δt) = amplitude * exp(-|Δt| / τ)
    /// ```
    Exponential {
        /// Correlation amplitude.
        amplitude: f64,

        /// Positive correlation time.
        correlation_time: Duration,
    },

    /// Gaussian correlation:
    ///
    /// ```text
    /// C(Δt) = amplitude * exp(-(Δt²)/(2τ²))
    /// ```
    Gaussian {
        /// Correlation amplitude.
        amplitude: f64,

        /// Positive correlation scale.
        correlation_time: Duration,
    },

    /// Power-law correlation:
    ///
    /// ```text
    /// C(Δt) = amplitude * (1 + |Δt|/τ)^(-exponent)
    /// ```
    PowerLaw {
        /// Correlation amplitude.
        amplitude: f64,

        /// Positive scale.
        scale: Duration,

        /// Positive exponent.
        exponent: f64,
    },
}

impl TemporalCorrelationKernel {
    /// Creates an exponential kernel.
    pub fn exponential(
        amplitude: f64,
        correlation_time: Duration,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;

        if correlation_time.seconds() <= 0.0 {
            return Err(TemporalNoiseError::InvalidCorrelationTime {
                value: correlation_time.seconds(),
            });
        }

        Ok(Self::Exponential {
            amplitude,
            correlation_time,
        })
    }

    /// Creates a Gaussian kernel.
    pub fn gaussian(
        amplitude: f64,
        correlation_time: Duration,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;

        if correlation_time.seconds() <= 0.0 {
            return Err(TemporalNoiseError::InvalidCorrelationTime {
                value: correlation_time.seconds(),
            });
        }

        Ok(Self::Gaussian {
            amplitude,
            correlation_time,
        })
    }

    /// Creates a power-law kernel.
    pub fn power_law(
        amplitude: f64,
        scale: Duration,
        exponent: f64,
    ) -> TemporalNoiseResult<Self> {
        validate_finite("amplitude", amplitude)?;
        validate_finite("exponent", exponent)?;

        if scale.seconds() <= 0.0 {
            return Err(TemporalNoiseError::InvalidCorrelationTime {
                value: scale.seconds(),
            });
        }

        if exponent <= 0.0 {
            return Err(TemporalNoiseError::InvalidRate {
                field: "exponent",
                value: exponent,
            });
        }

        Ok(Self::PowerLaw {
            amplitude,
            scale,
            exponent,
        })
    }

    /// Evaluates the correlation for a time separation.
    pub fn evaluate(&self, separation: Duration) -> TemporalNoiseResult<TemporalValue> {
        let dt = separation.seconds().abs();

        match *self {
            Self::Exponential {
                amplitude,
                correlation_time,
            } => {
                let exponent = -(dt / correlation_time.seconds());

                if !exponent.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "exponential correlation",
                    });
                }

                TemporalValue::new(amplitude * exponent.exp())
            }

            Self::Gaussian {
                amplitude,
                correlation_time,
            } => {
                let normalized = dt / correlation_time.seconds();
                let exponent = -0.5 * normalized * normalized;

                if !exponent.is_finite() {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "gaussian correlation",
                    });
                }

                TemporalValue::new(amplitude * exponent.exp())
            }

            Self::PowerLaw {
                amplitude,
                scale,
                exponent,
            } => {
                let base = 1.0 + dt / scale.seconds();

                if !base.is_finite() || base <= 0.0 {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "power-law correlation base",
                    });
                }

                let value = amplitude * base.powf(-exponent);

                TemporalValue::new(value)
            }
        }
    }
}

// ============================================================================
// Temporal composition
// ============================================================================

/// Binary composition operation for temporal functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalComposition {
    /// Add two temporal functions.
    Add,

    /// Multiply two temporal functions.
    Multiply,

    /// Subtract the second function from the first.
    Subtract,

    /// Divide the first function by the second.
    Divide,
}

/// Composition of two scalar temporal functions.
///
/// The functions are evaluated independently at the requested time and then
/// combined using the selected operation.
pub struct CompositeTemporal {
    left: Box<dyn TemporalFunction>,
    right: Box<dyn TemporalFunction>,
    operation: TemporalComposition,
}

impl CompositeTemporal {
    /// Creates a composite temporal function.
    pub fn new(
        left: Box<dyn TemporalFunction>,
        right: Box<dyn TemporalFunction>,
        operation: TemporalComposition,
    ) -> Self {
        Self {
            left,
            right,
            operation,
        }
    }

    /// Returns the composition operation.
    #[must_use]
    pub const fn operation(&self) -> TemporalComposition {
        self.operation
    }
}

impl TemporalFunction for CompositeTemporal {
    fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        let left = self.left.evaluate(time)?.value();
        let right = self.right.evaluate(time)?.value();

        let value = match self.operation {
            TemporalComposition::Add => left + right,
            TemporalComposition::Multiply => left * right,
            TemporalComposition::Subtract => left - right,

            TemporalComposition::Divide => {
                if right == 0.0 {
                    return Err(TemporalNoiseError::NumericalOverflow {
                        operation: "temporal division by zero",
                    });
                }

                left / right
            }
        };

        TemporalValue::new(value)
    }

    fn domain(&self) -> Option<TemporalInterval> {
        intersect_domains(self.left.domain(), self.right.domain())
    }

    fn kind(&self) -> TemporalFunctionKind {
        TemporalFunctionKind::Composite
    }
}

// ============================================================================
// Temporal profile
// ============================================================================

/// Production temporal-noise profile.
///
/// This is the primary object intended for integration with ZQN noise models.
///
/// It combines:
///
/// - resource scope;
/// - temporal function;
/// - optional correlation kernel;
/// - explicit semantic description.
pub struct TemporalProfile {
    scope: TemporalScope,
    function: Box<dyn TemporalFunction>,
    correlation: Option<TemporalCorrelationKernel>,
}

impl TemporalProfile {
    /// Creates a temporal profile.
    pub fn new(
        scope: TemporalScope,
        function: Box<dyn TemporalFunction>,
    ) -> Self {
        Self {
            scope,
            function,
            correlation: None,
        }
    }

    /// Adds a temporal correlation kernel.
    #[must_use]
    pub fn with_correlation(
        mut self,
        correlation: TemporalCorrelationKernel,
    ) -> Self {
        self.correlation = Some(correlation);
        self
    }

    /// Returns the resource scope.
    #[must_use]
    pub const fn scope(&self) -> TemporalScope {
        self.scope
    }

    /// Evaluates the profile.
    pub fn evaluate(&self, time: Time) -> TemporalNoiseResult<TemporalValue> {
        self.function.evaluate(time)
    }

    /// Evaluates temporal correlation at a supplied separation.
    pub fn correlation(
        &self,
        separation: Duration,
    ) -> TemporalNoiseResult<Option<TemporalValue>> {
        match self.correlation {
            Some(kernel) => Ok(Some(kernel.evaluate(separation)?)),
            None => Ok(None),
        }
    }

    /// Returns the underlying temporal-function kind.
    #[must_use]
    pub fn function_kind(&self) -> TemporalFunctionKind {
        self.function.kind()
    }

    /// Returns the function domain.
    #[must_use]
    pub fn domain(&self) -> Option<TemporalInterval> {
        self.function.domain()
    }

    /// Returns whether this profile has explicit temporal correlation.
    #[must_use]
    pub const fn has_correlation(&self) -> bool {
        self.correlation.is_some()
    }
}

// ============================================================================
// Temporal snapshot
// ============================================================================

/// Immutable temporal evaluation snapshot.
///
/// This is useful when multiple downstream consumers must use exactly the same
/// explicitly supplied time coordinate without depending on a wall clock.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalSnapshot {
    time: Time,
}

impl TemporalSnapshot {
    /// Creates a snapshot from explicit time.
    #[must_use]
    pub const fn new(time: Time) -> Self {
        Self { time }
    }

    /// Returns the snapshot time.
    #[must_use]
    pub const fn time(self) -> Time {
        self.time
    }

    /// Evaluates a profile at the snapshot's time.
    pub fn evaluate(
        self,
        profile: &TemporalProfile,
    ) -> TemporalNoiseResult<TemporalValue> {
        profile.evaluate(self.time)
    }
}

// ============================================================================
// Temporal sequence
// ============================================================================

/// A lazily consumable temporal evaluation contract.
///
/// This trait is intentionally independent from a concrete collection type.
///
/// It allows simulation, benchmarking, characterization, and runtime layers to
/// provide their own streaming execution without forcing `temporal.rs` to
/// allocate an entire temporal sequence.
pub trait TemporalEvaluator {
    /// Evaluates the temporal profile at an explicit coordinate.
    fn evaluate_at(
        &self,
        profile: &TemporalProfile,
        time: Time,
    ) -> TemporalNoiseResult<TemporalValue>;
}

/// Default deterministic evaluator.
#[derive(Debug, Clone, Copy, Default)]
pub struct DeterministicTemporalEvaluator;

impl TemporalEvaluator for DeterministicTemporalEvaluator {
    fn evaluate_at(
        &self,
        profile: &TemporalProfile,
        time: Time,
    ) -> TemporalNoiseResult<TemporalValue> {
        profile.evaluate(time)
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_finite(field: &'static str, value: f64) -> TemporalNoiseResult<()> {
    if !value.is_finite() {
        return Err(TemporalNoiseError::NonFinite { field, value });
    }

    Ok(())
}

fn validate_non_negative(
    field: &'static str,
    value: f64,
) -> TemporalNoiseResult<()> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(TemporalNoiseError::InvalidRate { field, value });
    }

    Ok(())
}

fn validate_samples(samples: &[TemporalSample]) -> TemporalNoiseResult<()> {
    if samples.is_empty() {
        return Err(TemporalNoiseError::EmptyInput {
            field: "samples",
        });
    }

    for pair in samples.windows(2) {
        let previous = pair[0].time().seconds();
        let current = pair[1].time().seconds();

        if current <= previous {
            return Err(TemporalNoiseError::InvalidSamples {
                reason: "sample times must be strictly increasing",
            });
        }
    }

    Ok(())
}

fn intersect_domains(
    left: Option<TemporalInterval>,
    right: Option<TemporalInterval>,
) -> Option<TemporalInterval> {
    match (left, right) {
        (None, None) => None,
        (Some(domain), None) | (None, Some(domain)) => Some(domain),

        (Some(left), Some(right)) => {
            let start = if left.start().seconds() >= right.start().seconds() {
                left.start()
            } else {
                right.start()
            };

            let end = if left.end().seconds() <= right.end().seconds() {
                left.end()
            } else {
                right.end()
            };

            TemporalInterval::new(start, end).ok()
        }
    }
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_rejects_non_finite_values() {
        assert!(Time::new(f64::NAN).is_err());
        assert!(Time::new(f64::INFINITY).is_err());
        assert!(Time::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn duration_rejects_negative_values() {
        assert!(Duration::new(-1.0).is_err());
        assert!(Duration::new(f64::NAN).is_err());
    }

    #[test]
    fn interval_requires_ordered_endpoints() {
        let start = Time::new(2.0).expect("valid time");
        let end = Time::new(1.0).expect("valid time");

        assert!(TemporalInterval::new(start, end).is_err());
    }

    #[test]
    fn constant_is_stationary() {
        let function =
            ConstantTemporal::new(0.25).expect("valid constant");

        let first = Time::new(0.0).expect("valid time");
        let second = Time::new(1000.0).expect("valid time");

        assert_eq!(
            function.evaluate(first).expect("evaluation").value(),
            function.evaluate(second).expect("evaluation").value()
        );
    }

    #[test]
    fn linear_drift_is_deterministic() {
        let reference = Time::zero();

        let function =
            LinearDrift::new(1.0, 2.0, reference)
                .expect("valid drift");

        let time = Time::new(3.0).expect("valid time");

        let first = function.evaluate(time).expect("evaluation");
        let second = function.evaluate(time).expect("evaluation");

        assert_eq!(first, second);
        assert_eq!(first.value(), 7.0);
    }

    #[test]
    fn exponential_decays() {
        let function =
            ExponentialTemporal::new(0.0, 1.0, 1.0, Time::zero())
                .expect("valid exponential");

        let value =
            function
                .evaluate(Time::new(1.0).expect("valid time"))
                .expect("evaluation")
                .value();

        assert!((value - (-1.0_f64).exp()).abs() < 1.0e-12);
    }

    #[test]
    fn power_law_is_finite() {
        let function =
            PowerLawTemporal::new(0.0, 1.0, 1.0, 2.0, Time::zero())
                .expect("valid power law");

        let value =
            function
                .evaluate(Time::new(1.0).expect("valid time"))
                .expect("evaluation")
                .value();

        assert!((value - 0.25).abs() < 1.0e-12);
    }

    #[test]
    fn periodic_function_is_repeatable() {
        let function =
            PeriodicTemporal::new(1.0, 2.0, 4.0, 0.0)
                .expect("valid periodic function");

        let a = function
            .evaluate(Time::new(0.5).expect("valid time"))
            .expect("evaluation");

        let b = function
            .evaluate(Time::new(4.5).expect("valid time"))
            .expect("evaluation");

        assert!((a.value() - b.value()).abs() < 1.0e-12);
    }

    #[test]
    fn sampled_profile_interpolates() {
        let samples = vec![
            TemporalSample::new(
                Time::new(0.0).expect("valid time"),
                0.0,
            )
            .expect("valid sample"),
            TemporalSample::new(
                Time::new(2.0).expect("valid time"),
                2.0,
            )
            .expect("valid sample"),
        ];

        let profile =
            SampledTemporal::new(
                samples,
                Interpolation::Linear,
                DomainPolicy::Reject,
            )
            .expect("valid profile");

        let value =
            profile
                .evaluate(Time::new(1.0).expect("valid time"))
                .expect("evaluation")
                .value();

        assert!((value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn sampled_profile_rejects_duplicate_times() {
        let samples = vec![
            TemporalSample::new(
                Time::new(0.0).expect("valid time"),
                1.0,
            )
            .expect("valid sample"),
            TemporalSample::new(
                Time::new(0.0).expect("valid time"),
                2.0,
            )
            .expect("valid sample"),
        ];

        assert!(
            SampledTemporal::new(
                samples,
                Interpolation::Linear,
                DomainPolicy::Reject,
            )
            .is_err()
        );
    }

    #[test]
    fn sampled_profile_clamps() {
        let samples = vec![
            TemporalSample::new(
                Time::new(1.0).expect("valid time"),
                10.0,
            )
            .expect("valid sample"),
            TemporalSample::new(
                Time::new(2.0).expect("valid time"),
                20.0,
            )
            .expect("valid sample"),
        ];

        let profile =
            SampledTemporal::new(
                samples,
                Interpolation::Linear,
                DomainPolicy::Clamp,
            )
            .expect("valid profile");

        let value =
            profile
                .evaluate(Time::new(-100.0).expect("valid time"))
                .expect("evaluation")
                .value();

        assert_eq!(value, 10.0);
    }

    #[test]
    fn exponential_correlation_is_symmetric() {
        let kernel =
            TemporalCorrelationKernel::exponential(
                1.0,
                Duration::new(2.0).expect("valid duration"),
            )
            .expect("valid kernel");

        let value =
            kernel
                .evaluate(Duration::new(1.0).expect("valid duration"))
                .expect("evaluation")
                .value();

        assert!((value - (-0.5_f64).exp()).abs() < 1.0e-12);
    }

    #[test]
    fn gaussian_correlation_at_zero_equals_amplitude() {
        let kernel =
            TemporalCorrelationKernel::gaussian(
                3.0,
                Duration::new(2.0).expect("valid duration"),
            )
            .expect("valid kernel");

        let value =
            kernel
                .evaluate(Duration::zero())
                .expect("evaluation")
                .value();

        assert!((value - 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn power_law_correlation_is_valid() {
        let kernel =
            TemporalCorrelationKernel::power_law(
                2.0,
                Duration::new(2.0).expect("valid duration"),
                1.0,
            )
            .expect("valid kernel");

        let value =
            kernel
                .evaluate(Duration::new(2.0).expect("valid duration"))
                .expect("evaluation")
                .value();

        assert!((value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn composite_addition_is_deterministic() {
        let left = Box::new(
            ConstantTemporal::new(2.0).expect("valid constant"),
        );

        let right = Box::new(
            ConstantTemporal::new(3.0).expect("valid constant"),
        );

        let composite =
            CompositeTemporal::new(
                left,
                right,
                TemporalComposition::Add,
            );

        let value =
            composite
                .evaluate(Time::zero())
                .expect("evaluation")
                .value();

        assert_eq!(value, 5.0);
    }

    #[test]
    fn temporal_profile_preserves_scope() {
        let profile =
            TemporalProfile::new(
                TemporalScope::Global,
                Box::new(
                    ConstantTemporal::new(1.0)
                        .expect("valid constant"),
                ),
            );

        assert!(profile.scope().is_global());
    }

    #[test]
    fn temporal_snapshot_reuses_explicit_time() {
        let profile =
            TemporalProfile::new(
                TemporalScope::Global,
                Box::new(
                    LinearDrift::new(
                        1.0,
                        2.0,
                        Time::zero(),
                    )
                    .expect("valid drift"),
                ),
            );

        let snapshot =
            TemporalSnapshot::new(
                Time::new(2.0).expect("valid time"),
            );

        let first =
            snapshot
                .evaluate(&profile)
                .expect("evaluation");

        let second =
            snapshot
                .evaluate(&profile)
                .expect("evaluation");

        assert_eq!(first, second);
        assert_eq!(first.value(), 5.0);
    }

    #[test]
    fn canonical_resource_scope_supports_logical_qubit() {
        // This test deliberately uses the canonical IR type. ZQN does not
        // manufacture another qubit identity.
        let qubit = QubitId::new(7);
        let scope = TemporalScope::LogicalQubit(qubit);

        assert!(scope.is_logical_qubit());
    }

    #[test]
    fn canonical_resource_scope_supports_physical_qubit() {
        let qubit = PhysicalQubitId::new(7);
        let scope = TemporalScope::PhysicalQubit(qubit);

        assert!(scope.is_physical_qubit());
    }
}