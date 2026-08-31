//! Zamani Quantum IR — Pulse Envelope Semantics
//!
//! Canonical, hardware-independent representation of pulse envelopes.
//!
//! # Architectural role
//!
//! An [`Envelope`] describes the semantic shape of a control signal.
//!
//! It answers:
//!
//! > What signal envelope does the program request?
//!
//! It does NOT answer:
//!
//! - which physical qubit receives the signal;
//! - which physical channel is used;
//! - which DAC/ADC is used;
//! - which sample clock is used;
//! - which carrier frequency is selected;
//! - how the envelope is quantized;
//! - how the envelope is clipped;
//! - how it is resampled;
//! - how it is scheduled;
//! - how calibration is applied;
//! - how the device executes it.
//!
//! Those decisions belong to downstream pulse, scheduling, hardware and backend
//! layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├── quantum::ir::pulse
//!      │       │
//!      │       └── envelope
//!      │
//!      ▼
//! target-independent optimization
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! quantum::hardware
//!      │
//!      ▼
//! backend
//!      │
//!      ▼
//! physical control system
//! ```
//!
//! # Universal-program principle
//!
//! Envelope semantics contain no fixed quantum-machine size.
//!
//! They can be used by:
//!
//! - one-qubit programs;
//! - very large programs;
//! - superconducting devices;
//! - trapped-ion devices;
//! - neutral-atom devices;
//! - spin systems;
//! - photonic systems;
//! - analog systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - distributed quantum systems;
//! - future quantum architectures.
//!
//! The envelope representation is independent of the number of qubits,
//! physical channels, machines, nodes, or execution resources.
//!
//! # Important semantic rule
//!
//! This module does not silently perform physical signal processing.
//!
//! In particular, constructing an envelope MUST NOT implicitly:
//!
//! - normalize;
//! - clip;
//! - resample;
//! - interpolate sampled data;
//! - quantize;
//! - truncate;
//! - reverse sample order;
//! - select a hardware sample rate.
//!
//! Such transformations must be represented by explicit downstream
//! transformations.
//!
//! # Parameter semantics
//!
//! Analytic envelope parameters use the canonical [`super::super::parameter::Parameter`]
//! type. Therefore an envelope may remain symbolic until a later compilation
//! stage.
//!
//! For example:
//!
//! ```text
//! amplitude = drive_amplitude
//! sigma     = pulse_sigma
//! center    = pulse_center
//! ```
//!
//! may remain symbolic in the canonical IR.
//!
//! # Domain
//!
//! Analytic envelope functions use a normalized coordinate:
//!
//! ```text
//! t ∈ [0, 1]
//! ```
//!
//! The normalized coordinate is deliberately independent of physical duration.
//!
//! Physical duration belongs to pulse/timing semantics.
//!
//! Thus:
//!
//! ```text
//! Envelope
//!     = shape
//!
//! Pulse
//!     = envelope + duration + target/control semantics
//!
//! Timing
//!     = temporal placement
//!
//! Hardware
//!     = physical realization
//! ```
//!
//! # Sampled envelopes
//!
//! [`Envelope::Sampled`] preserves samples exactly as supplied.
//!
//! This module does not interpolate them during construction or ordinary
//! access. A downstream resampling/interpolation pass must explicitly perform
//! that operation.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no `unsafe`
//! - no external dependencies
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `parameter.rs`
//!     Supplies symbolic and concrete scalar parameters.
//!
//! `pulse.rs`
//!     May reference an [`EnvelopeId`] and [`Envelope`] definition.
//!
//! `waveform.rs`
//!     May use an envelope as one semantic component of a waveform.
//!
//! `timing.rs`
//!     Supplies physical duration separately from normalized envelope shape.
//!
//! `channel.rs`
//!     Associates the eventual pulse with an abstract control channel.
//!
//! `frame.rs`
//!     Supplies frequency/phase reference semantics independently of the
//!     envelope.
//!
//! `operation.rs`
//!     May reference pulse operations containing envelope references.
//!
//! `serialization.rs`
//!     Owns canonical persistence of this structure.
//!
//! `hash.rs`
//!     Owns canonical content hashing.
//!
//! `validation.rs`
//!     Performs whole-program validation.
//!
//! `optimization/`
//!     May transform envelopes only through explicit semantic-preserving
//!     transformations.
//!
//! `hardware/`
//!     Determines whether a target can realize an envelope and how it must be
//!     lowered.
//!
//! # Qubit identity
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! An envelope is independent of qubit identity.
//!
//! A pulse may eventually target a logical qubit through the canonical
//! `quantum::ir::qubit::QubitId`, but that relationship belongs to the pulse
//! operation, not to the reusable envelope definition.
//!
//! This prevents signal-shape semantics from becoming coupled to qubit
//! allocation, routing, topology or hardware mapping.
//!
//! # File completion guarantee
//!
//! This file owns:
//!
//! - envelope identity;
//! - scalar/complex envelope values;
//! - analytic envelope forms;
//! - sampled envelope representation;
//! - symbolic parameters;
//! - composition;
//! - validation;
//! - deterministic metadata;
//! - explicit validation policies;
//! - checked numerical operations;
//! - structural equality;
//! - deterministic traversal;
//! - local tests.
//!
//! Later implementation of pulse, waveform, timing, hardware or backend
//! modules must not require changing the semantic meaning of this file.
//!
//! -----------------------------------------------------------------------------
//! Safety
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::parameter::Parameter;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for Zamani pulse envelopes.
pub const ENVELOPE_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.envelope";

/// Semantic envelope schema major version.
pub const ENVELOPE_SCHEMA_MAJOR: u16 = 1;

/// Semantic envelope schema minor version.
pub const ENVELOPE_SCHEMA_MINOR: u16 = 0;

/// Semantic envelope schema patch version.
pub const ENVELOPE_SCHEMA_PATCH: u16 = 0;

/// Returns the semantic envelope schema version.
#[must_use]
pub const fn envelope_schema_version() -> (u16, u16, u16) {
    (
        ENVELOPE_SCHEMA_MAJOR,
        ENVELOPE_SCHEMA_MINOR,
        ENVELOPE_SCHEMA_PATCH,
    )
}

// =============================================================================
// Resource / validation policy
// =============================================================================

/// Default maximum number of explicit samples accepted by the convenience
/// constructor.
///
/// This is a resource/security policy, NOT a semantic limit on Zamani.
pub const DEFAULT_MAX_SAMPLES: usize = 1_048_576;

/// Default maximum number of nodes in one composition tree.
///
/// This prevents accidental pathological input while keeping the semantic
/// model independent of machine size.
pub const DEFAULT_MAX_COMPOSITION_NODES: usize = 1_048_576;

/// Default maximum metadata fields.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum metadata key length in bytes.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default maximum metadata value length in bytes.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Validation policy for envelope construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeValidationPolicy {
    /// Maximum explicit samples accepted by validation.
    pub max_samples: usize,

    /// Maximum composition nodes accepted by validation.
    pub max_composition_nodes: usize,

    /// Maximum metadata fields.
    pub max_metadata_fields: usize,

    /// Maximum metadata key size in UTF-8 bytes.
    pub max_metadata_key_bytes: usize,

    /// Maximum metadata value size in UTF-8 bytes.
    pub max_metadata_value_bytes: usize,
}

impl Default for EnvelopeValidationPolicy {
    fn default() -> Self {
        Self {
            max_samples: DEFAULT_MAX_SAMPLES,
            max_composition_nodes: DEFAULT_MAX_COMPOSITION_NODES,
            max_metadata_fields: DEFAULT_MAX_METADATA_FIELDS,
            max_metadata_key_bytes: DEFAULT_MAX_METADATA_KEY_BYTES,
            max_metadata_value_bytes: DEFAULT_MAX_METADATA_VALUE_BYTES,
        }
    }
}

impl EnvelopeValidationPolicy {
    /// Creates an explicit policy.
    #[must_use]
    pub const fn new(
        max_samples: usize,
        max_composition_nodes: usize,
        max_metadata_fields: usize,
        max_metadata_key_bytes: usize,
        max_metadata_value_bytes: usize,
    ) -> Self {
        Self {
            max_samples,
            max_composition_nodes,
            max_metadata_fields,
            max_metadata_key_bytes,
            max_metadata_value_bytes,
        }
    }

    /// Validates the policy.
    pub fn validate(&self) -> EnvelopeResult<()> {
        if self.max_samples == 0 {
            return Err(EnvelopeError::InvalidPolicy(
                "max_samples must be greater than zero",
            ));
        }

        if self.max_composition_nodes == 0 {
            return Err(EnvelopeError::InvalidPolicy(
                "max_composition_nodes must be greater than zero",
            ));
        }

        if self.max_metadata_fields == 0 {
            return Err(EnvelopeError::InvalidPolicy(
                "max_metadata_fields must be greater than zero",
            ));
        }

        if self.max_metadata_key_bytes == 0 {
            return Err(EnvelopeError::InvalidPolicy(
                "max_metadata_key_bytes must be greater than zero",
            ));
        }

        if self.max_metadata_value_bytes == 0 {
            return Err(EnvelopeError::InvalidPolicy(
                "max_metadata_value_bytes must be greater than zero",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Result / error
// =============================================================================

/// Result returned by envelope operations.
pub type EnvelopeResult<T> = Result<T, EnvelopeError>;

/// Envelope-specific errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvelopeError {
    /// A numerical value was NaN or infinite.
    NonFiniteValue {
        /// Semantic name of the invalid value.
        field: &'static str,
    },

    /// A parameter was invalid.
    InvalidParameter(&'static str),

    /// A parameter could not be evaluated.
    UnboundParameter(String),

    /// A numerical operation overflowed.
    NumericalOverflow,

    /// A numerical operation produced an invalid result.
    NumericalDomainError(&'static str),

    /// A normalized coordinate was outside `[0, 1]`.
    CoordinateOutOfRange,

    /// An empty sample sequence was supplied.
    EmptySamples,

    /// A sample index was outside the available sequence.
    SampleIndexOutOfRange,

    /// Sample count exceeded the explicit validation policy.
    SampleLimitExceeded {
        /// Number of supplied samples.
        actual: usize,

        /// Maximum allowed by the selected policy.
        maximum: usize,
    },

    /// Composition depth/node count exceeded policy.
    CompositionLimitExceeded {
        /// Number of nodes encountered.
        actual: usize,

        /// Maximum permitted nodes.
        maximum: usize,
    },

    /// Metadata validation failed.
    InvalidMetadata(&'static str),

    /// Validation policy itself was invalid.
    InvalidPolicy(&'static str),

    /// An invalid composition weight was supplied.
    InvalidWeight,

    /// A sequence contains no elements.
    EmptyComposition,

    /// An operation requires a positive parameter but received zero/negative
    /// concrete value.
    NonPositiveParameter {
        /// Semantic parameter name.
        field: &'static str,
    },
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field } => {
                write!(formatter, "{field} must be finite")
            }

            Self::InvalidParameter(message) => {
                formatter.write_str(message)
            }

            Self::UnboundParameter(name) => {
                write!(formatter, "parameter `{name}` is unbound")
            }

            Self::NumericalOverflow => {
                formatter.write_str("envelope numerical operation overflowed")
            }

            Self::NumericalDomainError(message) => {
                formatter.write_str(message)
            }

            Self::CoordinateOutOfRange => {
                formatter.write_str(
                    "envelope coordinate must be within [0, 1]",
                )
            }

            Self::EmptySamples => {
                formatter.write_str("sampled envelope cannot be empty")
            }

            Self::SampleIndexOutOfRange => {
                formatter.write_str("sample index is out of range")
            }

            Self::SampleLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "sample count {actual} exceeds validation limit {maximum}"
                )
            }

            Self::CompositionLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "composition node count {actual} exceeds validation limit {maximum}"
                )
            }

            Self::InvalidMetadata(message) => {
                formatter.write_str(message)
            }

            Self::InvalidPolicy(message) => {
                formatter.write_str(message)
            }

            Self::InvalidWeight => {
                formatter.write_str("composition weight must be finite")
            }

            Self::EmptyComposition => {
                formatter.write_str("envelope composition cannot be empty")
            }

            Self::NonPositiveParameter { field } => {
                write!(formatter, "{field} must be greater than zero")
            }
        }
    }
}

impl std::error::Error for EnvelopeError {}

// =============================================================================
// Envelope ID
// =============================================================================

/// Stable envelope identity.
///
/// The value has no relationship to a physical machine index.
///
/// It is an IR object identity only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EnvelopeId(u64);

impl EnvelopeId {
    /// Creates an envelope ID from its canonical numeric value.
    ///
    /// `0` is valid. Identity allocation policy belongs to the containing
    /// program/module and is not imposed here.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for EnvelopeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "env{}", self.0)
    }
}

// =============================================================================
// Envelope value
// =============================================================================

/// Finite complex envelope value.
///
/// The two components represent abstract in-phase and quadrature values.
///
/// No vendor-specific IQ convention is implied.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopeValue {
    in_phase: f64,
    quadrature: f64,
}

impl EnvelopeValue {
    /// Creates a finite complex envelope value.
    pub fn new(in_phase: f64, quadrature: f64) -> EnvelopeResult<Self> {
        ensure_finite(in_phase, "in_phase")?;
        ensure_finite(quadrature, "quadrature")?;

        Ok(Self {
            in_phase,
            quadrature,
        })
    }

    /// Creates a real envelope value.
    pub fn real(value: f64) -> EnvelopeResult<Self> {
        Self::new(value, 0.0)
    }

    /// Returns zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            in_phase: 0.0,
            quadrature: 0.0,
        }
    }

    /// Returns the in-phase component.
    #[must_use]
    pub const fn in_phase(self) -> f64 {
        self.in_phase
    }

    /// Returns the quadrature component.
    #[must_use]
    pub const fn quadrature(self) -> f64 {
        self.quadrature
    }

    /// Returns squared magnitude.
    pub fn magnitude_squared(self) -> EnvelopeResult<f64> {
        let i = self.in_phase * self.in_phase;

        if !i.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let q = self.quadrature * self.quadrature;

        if !q.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let value = i + q;

        if value.is_finite() {
            Ok(value)
        } else {
            Err(EnvelopeError::NumericalOverflow)
        }
    }

    /// Returns magnitude.
    pub fn magnitude(self) -> EnvelopeResult<f64> {
        let magnitude = self.magnitude_squared()?.sqrt();

        if magnitude.is_finite() {
            Ok(magnitude)
        } else {
            Err(EnvelopeError::NumericalOverflow)
        }
    }

    /// Adds two envelope values with finite-result checking.
    pub fn checked_add(self, other: Self) -> EnvelopeResult<Self> {
        let i = self.in_phase + other.in_phase;
        let q = self.quadrature + other.quadrature;

        Self::new(i, q)
    }

    /// Multiplies both components by a finite scalar.
    pub fn checked_scale(self, scalar: f64) -> EnvelopeResult<Self> {
        ensure_finite(scalar, "scale")?;

        Self::new(
            self.in_phase * scalar,
            self.quadrature * scalar,
        )
    }

    /// Validates the value.
    pub fn validate(self) -> EnvelopeResult<()> {
        ensure_finite(self.in_phase, "in_phase")?;
        ensure_finite(self.quadrature, "quadrature")?;
        Ok(())
    }
}

impl Default for EnvelopeValue {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Analytic envelope primitives
// =============================================================================

/// Constant envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantEnvelope {
    /// Constant value.
    pub value: EnvelopeValue,
}

impl ConstantEnvelope {
    /// Creates a constant envelope.
    pub fn new(value: EnvelopeValue) -> EnvelopeResult<Self> {
        value.validate()?;
        Ok(Self { value })
    }

    /// Creates a real-valued constant envelope.
    pub fn real(value: f64) -> EnvelopeResult<Self> {
        Self::new(EnvelopeValue::real(value)?)
    }

    /// Evaluates the envelope.
    pub fn evaluate(&self, _t: f64) -> EnvelopeResult<EnvelopeValue> {
        self.value.validate()?;
        Ok(self.value)
    }
}

/// Linear envelope between two values.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearEnvelope {
    /// Value at normalized coordinate `0`.
    pub start: EnvelopeValue,

    /// Value at normalized coordinate `1`.
    pub end: EnvelopeValue,
}

impl LinearEnvelope {
    /// Creates a linear envelope.
    pub fn new(
        start: EnvelopeValue,
        end: EnvelopeValue,
    ) -> EnvelopeResult<Self> {
        start.validate()?;
        end.validate()?;

        Ok(Self { start, end })
    }

    /// Evaluates the linear envelope.
    pub fn evaluate(&self, t: f64) -> EnvelopeResult<EnvelopeValue> {
        validate_coordinate(t)?;

        let i = self.start.in_phase()
            + (self.end.in_phase() - self.start.in_phase()) * t;

        let q = self.start.quadrature()
            + (self.end.quadrature() - self.start.quadrature()) * t;

        EnvelopeValue::new(i, q)
    }
}

/// Gaussian envelope.
///
/// The Gaussian is evaluated over normalized coordinate `t ∈ [0, 1]`.
///
/// The parameters are intentionally symbolic-capable.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianEnvelope {
    /// Peak amplitude.
    pub amplitude: Parameter,

    /// Standard deviation in normalized-coordinate units.
    pub sigma: Parameter,

    /// Center in normalized-coordinate units.
    pub center: Parameter,
}

impl GaussianEnvelope {
    /// Creates a Gaussian envelope.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
    ) -> EnvelopeResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&sigma)?;
        validate_parameter(&center)?;

        if let Some(value) = sigma.as_constant() {
            ensure_positive(value, "sigma")?;
        }

        Ok(Self {
            amplitude,
            sigma,
            center,
        })
    }

    /// Evaluates using an explicit parameter resolver.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        validate_coordinate(t)?;

        let amplitude = bind_parameter(&self.amplitude, resolver)?;
        let sigma = bind_parameter(&self.sigma, resolver)?;
        let center = bind_parameter(&self.center, resolver)?;

        ensure_positive(sigma, "sigma")?;

        let normalized = (t - center) / sigma;

        if !normalized.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let exponent = -0.5 * normalized * normalized;

        if !exponent.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let value = amplitude * exponent.exp();

        EnvelopeValue::real(value)
    }

    /// Validates the symbolic definition without binding it.
    pub fn validate(&self) -> EnvelopeResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.sigma.clone(),
            self.center.clone(),
        )
        .map(|_| ())
    }
}

/// DRAG envelope.
///
/// DRAG is represented semantically as an in-phase Gaussian and its derivative
/// correction in quadrature.
///
/// The exact physical calibration and interpretation are target dependent.
#[derive(Debug, Clone, PartialEq)]
pub struct DragEnvelope {
    /// Base Gaussian amplitude.
    pub amplitude: Parameter,

    /// Gaussian standard deviation.
    pub sigma: Parameter,

    /// Gaussian center.
    pub center: Parameter,

    /// Derivative correction coefficient.
    pub beta: Parameter,
}

impl DragEnvelope {
    /// Creates a DRAG envelope.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
        beta: Parameter,
    ) -> EnvelopeResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&sigma)?;
        validate_parameter(&center)?;
        validate_parameter(&beta)?;

        if let Some(value) = sigma.as_constant() {
            ensure_positive(value, "sigma")?;
        }

        Ok(Self {
            amplitude,
            sigma,
            center,
            beta,
        })
    }

    /// Evaluates the DRAG envelope.
    ///
    /// The quadrature component is the normalized derivative correction:
    ///
    /// `-beta * (t-center) / sigma² * gaussian`
    ///
    /// This defines mathematical semantics only; target-specific calibration
    /// remains outside this module.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        validate_coordinate(t)?;

        let amplitude = bind_parameter(&self.amplitude, resolver)?;
        let sigma = bind_parameter(&self.sigma, resolver)?;
        let center = bind_parameter(&self.center, resolver)?;
        let beta = bind_parameter(&self.beta, resolver)?;

        ensure_positive(sigma, "sigma")?;

        let delta = t - center;
        let normalized = delta / sigma;
        let exponent = -0.5 * normalized * normalized;

        if !normalized.is_finite() || !exponent.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let gaussian = amplitude * exponent.exp();

        if !gaussian.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let sigma_squared = sigma * sigma;

        if !sigma_squared.is_finite() || sigma_squared == 0.0 {
            return Err(EnvelopeError::NumericalOverflow);
        }

        let derivative = -beta * delta / sigma_squared * gaussian;

        EnvelopeValue::new(gaussian, derivative)
    }

    /// Validates the symbolic definition.
    pub fn validate(&self) -> EnvelopeResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.sigma.clone(),
            self.center.clone(),
            self.beta.clone(),
        )
        .map(|_| ())
    }
}

/// Cosine envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct CosineEnvelope {
    /// Amplitude.
    pub amplitude: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,

    /// Number of cycles across the normalized interval.
    pub cycles: Parameter,
}

impl CosineEnvelope {
    /// Creates a cosine envelope.
    pub fn new(
        amplitude: Parameter,
        phase: Parameter,
        cycles: Parameter,
    ) -> EnvelopeResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&phase)?;
        validate_parameter(&cycles)?;

        Ok(Self {
            amplitude,
            phase,
            cycles,
        })
    }

    /// Evaluates the cosine envelope.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        validate_coordinate(t)?;

        let amplitude = bind_parameter(&self.amplitude, resolver)?;
        let phase = bind_parameter(&self.phase, resolver)?;
        let cycles = bind_parameter(&self.cycles, resolver)?;

        let argument =
            std::f64::consts::TAU * cycles * t + phase;

        if !argument.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        EnvelopeValue::real(amplitude * argument.cos())
    }

    /// Validates the definition.
    pub fn validate(&self) -> EnvelopeResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.phase.clone(),
            self.cycles.clone(),
        )
        .map(|_| ())
    }
}

/// Sine envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct SineEnvelope {
    /// Amplitude.
    pub amplitude: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,

    /// Number of cycles across the normalized interval.
    pub cycles: Parameter,
}

impl SineEnvelope {
    /// Creates a sine envelope.
    pub fn new(
        amplitude: Parameter,
        phase: Parameter,
        cycles: Parameter,
    ) -> EnvelopeResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&phase)?;
        validate_parameter(&cycles)?;

        Ok(Self {
            amplitude,
            phase,
            cycles,
        })
    }

    /// Evaluates the sine envelope.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        validate_coordinate(t)?;

        let amplitude = bind_parameter(&self.amplitude, resolver)?;
        let phase = bind_parameter(&self.phase, resolver)?;
        let cycles = bind_parameter(&self.cycles, resolver)?;

        let argument =
            std::f64::consts::TAU * cycles * t + phase;

        if !argument.is_finite() {
            return Err(EnvelopeError::NumericalOverflow);
        }

        EnvelopeValue::real(amplitude * argument.sin())
    }

    /// Validates the definition.
    pub fn validate(&self) -> EnvelopeResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.phase.clone(),
            self.cycles.clone(),
        )
        .map(|_| ())
    }
}

// =============================================================================
// Sampled envelope
// =============================================================================

/// Explicit sampled envelope.
///
/// Samples are retained exactly in source order.
///
/// This type deliberately does not perform interpolation or resampling.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledEnvelope {
    samples: Vec<EnvelopeValue>,
}

impl SampledEnvelope {
    /// Creates a sampled envelope using the default sample-count policy.
    pub fn new(samples: Vec<EnvelopeValue>) -> EnvelopeResult<Self> {
        Self::new_with_policy(samples, EnvelopeValidationPolicy::default())
    }

    /// Creates a sampled envelope with an explicit resource policy.
    pub fn new_with_policy(
        samples: Vec<EnvelopeValue>,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<Self> {
        policy.validate()?;

        if samples.is_empty() {
            return Err(EnvelopeError::EmptySamples);
        }

        if samples.len() > policy.max_samples {
            return Err(EnvelopeError::SampleLimitExceeded {
                actual: samples.len(),
                maximum: policy.max_samples,
            });
        }

        for sample in &samples {
            sample.validate()?;
        }

        Ok(Self { samples })
    }

    /// Returns the number of samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether there are no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns a sample without interpolation.
    pub fn sample(&self, index: usize) -> EnvelopeResult<EnvelopeValue> {
        self.samples
            .get(index)
            .copied()
            .ok_or(EnvelopeError::SampleIndexOutOfRange)
    }

    /// Returns the samples in source order.
    #[must_use]
    pub fn samples(&self) -> &[EnvelopeValue] {
        &self.samples
    }

    /// Consumes the envelope and returns the samples.
    #[must_use]
    pub fn into_samples(self) -> Vec<EnvelopeValue> {
        self.samples
    }

    /// Returns the first sample.
    pub fn first(&self) -> EnvelopeResult<EnvelopeValue> {
        self.sample(0)
    }

    /// Returns the last sample.
    pub fn last(&self) -> EnvelopeResult<EnvelopeValue> {
        self.sample(self.samples.len() - 1)
    }

    /// Validates using an explicit policy.
    pub fn validate(
        &self,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<()> {
        policy.validate()?;

        if self.samples.is_empty() {
            return Err(EnvelopeError::EmptySamples);
        }

        if self.samples.len() > policy.max_samples {
            return Err(EnvelopeError::SampleLimitExceeded {
                actual: self.samples.len(),
                maximum: policy.max_samples,
            });
        }

        for sample in &self.samples {
            sample.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Composition
// =============================================================================

/// Weighted envelope component.
///
/// The weight is explicit and does not imply normalization.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeComponent {
    /// Component envelope.
    pub envelope: Box<Envelope>,

    /// Scalar multiplier.
    pub weight: f64,
}

impl EnvelopeComponent {
    /// Creates a weighted component.
    pub fn new(
        envelope: Envelope,
        weight: f64,
    ) -> EnvelopeResult<Self> {
        ensure_finite(weight, "weight")?;

        Ok(Self {
            envelope: Box::new(envelope),
            weight,
        })
    }
}

/// Composition of multiple envelopes.
///
/// Composition is additive. It does not implicitly normalize.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeEnvelope {
    components: Vec<EnvelopeComponent>,
}

impl CompositeEnvelope {
    /// Creates a composition.
    pub fn new(
        components: Vec<EnvelopeComponent>,
    ) -> EnvelopeResult<Self> {
        Self::new_with_policy(
            components,
            EnvelopeValidationPolicy::default(),
        )
    }

    /// Creates a composition with explicit resource policy.
    pub fn new_with_policy(
        components: Vec<EnvelopeComponent>,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<Self> {
        policy.validate()?;

        if components.is_empty() {
            return Err(EnvelopeError::EmptyComposition);
        }

        let mut nodes = 1usize;

        for component in &components {
            ensure_finite(component.weight, "weight")?;

            let child_nodes = component.envelope.node_count();

            nodes = nodes.checked_add(child_nodes).ok_or(
                EnvelopeError::CompositionLimitExceeded {
                    actual: usize::MAX,
                    maximum: policy.max_composition_nodes,
                },
            )?;

            if nodes > policy.max_composition_nodes {
                return Err(
                    EnvelopeError::CompositionLimitExceeded {
                        actual: nodes,
                        maximum: policy.max_composition_nodes,
                    },
                );
            }
        }

        Ok(Self { components })
    }

    /// Returns the number of components.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether the composition is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns components in source order.
    #[must_use]
    pub fn components(&self) -> &[EnvelopeComponent] {
        &self.components
    }

    /// Evaluates all components and adds their weighted values.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        validate_coordinate(t)?;

        let mut result = EnvelopeValue::zero();

        for component in &self.components {
            let value = component.envelope.evaluate(t, resolver)?;
            let weighted = value.checked_scale(component.weight)?;

            result = result.checked_add(weighted)?;
        }

        Ok(result)
    }

    /// Counts all nodes recursively without using recursion.
    pub fn node_count(&self) -> usize {
        let mut count = 1usize;

        let mut stack: Vec<&Envelope> = Vec::new();

        for component in &self.components {
            stack.push(component.envelope.as_ref());
        }

        while let Some(envelope) = stack.pop() {
            count = count.saturating_add(1);

            if let Envelope::Composite(composite) = envelope {
                for component in composite.components() {
                    stack.push(component.envelope.as_ref());
                }
            }
        }

        count
    }

    /// Validates this composition.
    pub fn validate(
        &self,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<()> {
        policy.validate()?;

        if self.components.is_empty() {
            return Err(EnvelopeError::EmptyComposition);
        }

        let nodes = self.node_count();

        if nodes > policy.max_composition_nodes {
            return Err(
                EnvelopeError::CompositionLimitExceeded {
                    actual: nodes,
                    maximum: policy.max_composition_nodes,
                },
            );
        }

        for component in &self.components {
            ensure_finite(component.weight, "weight")?;
            component.envelope.validate_with_policy(policy)?;
        }

        Ok(())
    }
}

// =============================================================================
// Envelope
// =============================================================================

/// Canonical semantic envelope.
///
/// The enum is intentionally extensible at the module/dialect boundary:
/// built-in mathematical forms provide common semantics, while future
/// envelope kinds can be represented through the IR extension/dialect system
/// instead of modifying hardware code.
#[derive(Debug, Clone, PartialEq)]
pub enum Envelope {
    /// Constant value.
    Constant(ConstantEnvelope),

    /// Linear interpolation as an explicit mathematical operation.
    Linear(LinearEnvelope),

    /// Gaussian envelope.
    Gaussian(GaussianEnvelope),

    /// DRAG envelope.
    Drag(DragEnvelope),

    /// Cosine envelope.
    Cosine(CosineEnvelope),

    /// Sine envelope.
    Sine(SineEnvelope),

    /// Explicit source-ordered samples.
    Sampled(SampledEnvelope),

    /// Additive composition.
    Composite(CompositeEnvelope),
}

impl Envelope {
    /// Creates a constant envelope.
    pub fn constant(value: EnvelopeValue) -> EnvelopeResult<Self> {
        Ok(Self::Constant(ConstantEnvelope::new(value)?))
    }

    /// Creates a real constant envelope.
    pub fn constant_real(value: f64) -> EnvelopeResult<Self> {
        Ok(Self::Constant(ConstantEnvelope::real(value)?))
    }

    /// Creates a linear envelope.
    pub fn linear(
        start: EnvelopeValue,
        end: EnvelopeValue,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Linear(LinearEnvelope::new(start, end)?))
    }

    /// Creates a Gaussian envelope.
    pub fn gaussian(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Gaussian(GaussianEnvelope::new(
            amplitude,
            sigma,
            center,
        )?))
    }

    /// Creates a DRAG envelope.
    pub fn drag(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
        beta: Parameter,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Drag(DragEnvelope::new(
            amplitude,
            sigma,
            center,
            beta,
        )?))
    }

    /// Creates a cosine envelope.
    pub fn cosine(
        amplitude: Parameter,
        phase: Parameter,
        cycles: Parameter,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Cosine(CosineEnvelope::new(
            amplitude,
            phase,
            cycles,
        )?))
    }

    /// Creates a sine envelope.
    pub fn sine(
        amplitude: Parameter,
        phase: Parameter,
        cycles: Parameter,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Sine(SineEnvelope::new(
            amplitude,
            phase,
            cycles,
        )?))
    }

    /// Creates an explicit sampled envelope.
    pub fn sampled(
        samples: Vec<EnvelopeValue>,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Sampled(SampledEnvelope::new(samples)?))
    }

    /// Creates an additive composition.
    pub fn composite(
        components: Vec<EnvelopeComponent>,
    ) -> EnvelopeResult<Self> {
        Ok(Self::Composite(CompositeEnvelope::new(components)?))
    }

    /// Evaluates an analytic envelope at normalized coordinate `t`.
    ///
    /// Sampled envelopes intentionally return an error here because this method
    /// must not silently invent an interpolation policy.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Constant(envelope) => envelope.evaluate(t),

            Self::Linear(envelope) => envelope.evaluate(t),

            Self::Gaussian(envelope) => {
                envelope.evaluate(t, resolver)
            }

            Self::Drag(envelope) => {
                envelope.evaluate(t, resolver)
            }

            Self::Cosine(envelope) => {
                envelope.evaluate(t, resolver)
            }

            Self::Sine(envelope) => {
                envelope.evaluate(t, resolver)
            }

            Self::Sampled(_) => {
                Err(EnvelopeError::NumericalDomainError(
                    "sampled envelopes require explicit sample-index access; \
                     interpolation must be an explicit downstream operation",
                ))
            }

            Self::Composite(envelope) => {
                envelope.evaluate(t, resolver)
            }
        }
    }

    /// Returns the envelope's structural node count.
    ///
    /// This operation is iterative and does not consume call-stack depth
    /// proportional to composition depth.
    pub fn node_count(&self) -> usize {
        match self {
            Self::Composite(composite) => composite.node_count(),

            _ => 1,
        }
    }

    /// Returns whether this is a sampled envelope.
    #[must_use]
    pub fn is_sampled(&self) -> bool {
        matches!(self, Self::Sampled(_))
    }

    /// Returns whether this envelope contains symbolic parameters.
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Constant(_) |
            Self::Linear(_) |
            Self::Sampled(_) => false,

            Self::Gaussian(envelope) => {
                parameter_is_symbolic(&envelope.amplitude)
                    || parameter_is_symbolic(&envelope.sigma)
                    || parameter_is_symbolic(&envelope.center)
            }

            Self::Drag(envelope) => {
                parameter_is_symbolic(&envelope.amplitude)
                    || parameter_is_symbolic(&envelope.sigma)
                    || parameter_is_symbolic(&envelope.center)
                    || parameter_is_symbolic(&envelope.beta)
            }

            Self::Cosine(envelope) => {
                parameter_is_symbolic(&envelope.amplitude)
                    || parameter_is_symbolic(&envelope.phase)
                    || parameter_is_symbolic(&envelope.cycles)
            }

            Self::Sine(envelope) => {
                parameter_is_symbolic(&envelope.amplitude)
                    || parameter_is_symbolic(&envelope.phase)
                    || parameter_is_symbolic(&envelope.cycles)
            }

            Self::Composite(composite) => composite
                .components()
                .iter()
                .any(|component| {
                    parameter_is_symbolic_envelope(
                        component.envelope.as_ref(),
                    )
                }),
        }
    }

    /// Validates with the default policy.
    pub fn validate(&self) -> EnvelopeResult<()> {
        self.validate_with_policy(EnvelopeValidationPolicy::default())
    }

    /// Validates with an explicit resource policy.
    pub fn validate_with_policy(
        &self,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<()> {
        policy.validate()?;

        match self {
            Self::Constant(envelope) => envelope.value.validate(),

            Self::Linear(envelope) => {
                envelope.start.validate()?;
                envelope.end.validate()
            }

            Self::Gaussian(envelope) => envelope.validate(),

            Self::Drag(envelope) => envelope.validate(),

            Self::Cosine(envelope) => envelope.validate(),

            Self::Sine(envelope) => envelope.validate(),

            Self::Sampled(envelope) => envelope.validate(policy),

            Self::Composite(envelope) => {
                envelope.validate(policy)
            }
        }
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Deterministic envelope metadata.
///
/// `BTreeMap` is intentional: canonical iteration order must not depend on
/// hash-map randomization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EnvelopeMetadata {
    values: BTreeMap<String, String>,
}

impl EnvelopeMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts metadata using the default policy.
    pub fn insert<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) -> EnvelopeResult<Option<String>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.insert_with_policy(
            key,
            value,
            EnvelopeValidationPolicy::default(),
        )
    }

    /// Inserts metadata using an explicit policy.
    pub fn insert_with_policy<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<Option<String>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        policy.validate()?;

        let key = key.into();
        let value = value.into();

        if key.is_empty() {
            return Err(EnvelopeError::InvalidMetadata(
                "metadata key cannot be empty",
            ));
        }

        if key.len() > policy.max_metadata_key_bytes {
            return Err(EnvelopeError::InvalidMetadata(
                "metadata key exceeds configured byte limit",
            ));
        }

        if value.len() > policy.max_metadata_value_bytes {
            return Err(EnvelopeError::InvalidMetadata(
                "metadata value exceeds configured byte limit",
            ));
        }

        if !self.values.contains_key(&key)
            && self.values.len() >= policy.max_metadata_fields
        {
            return Err(EnvelopeError::InvalidMetadata(
                "metadata field limit exceeded",
            ));
        }

        Ok(self.values.insert(key, value))
    }

    /// Gets a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// Returns metadata fields in canonical key order.
    #[must_use]
    pub fn fields(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    /// Returns the number of metadata fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Validates metadata under a policy.
    pub fn validate(
        &self,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<()> {
        policy.validate()?;

        if self.values.len() > policy.max_metadata_fields {
            return Err(EnvelopeError::InvalidMetadata(
                "metadata field limit exceeded",
            ));
        }

        for (key, value) in &self.values {
            if key.is_empty() {
                return Err(EnvelopeError::InvalidMetadata(
                    "metadata key cannot be empty",
                ));
            }

            if key.len() > policy.max_metadata_key_bytes {
                return Err(EnvelopeError::InvalidMetadata(
                    "metadata key exceeds configured byte limit",
                ));
            }

            if value.len() > policy.max_metadata_value_bytes {
                return Err(EnvelopeError::InvalidMetadata(
                    "metadata value exceeds configured byte limit",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Envelope definition
// =============================================================================

/// Complete reusable envelope definition.
///
/// This is the object that a pulse layer may reference by [`EnvelopeId`].
#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeDefinition {
    /// Stable IR identity.
    pub id: EnvelopeId,

    /// Semantic envelope body.
    pub envelope: Envelope,

    /// Optional deterministic metadata.
    pub metadata: EnvelopeMetadata,
}

impl EnvelopeDefinition {
    /// Creates an envelope definition.
    pub fn new(
        id: EnvelopeId,
        envelope: Envelope,
    ) -> EnvelopeResult<Self> {
        envelope.validate()?;

        Ok(Self {
            id,
            envelope,
            metadata: EnvelopeMetadata::new(),
        })
    }

    /// Creates an envelope definition with metadata.
    pub fn with_metadata(
        id: EnvelopeId,
        envelope: Envelope,
        metadata: EnvelopeMetadata,
    ) -> EnvelopeResult<Self> {
        envelope.validate()?;
        metadata.validate(EnvelopeValidationPolicy::default())?;

        Ok(Self {
            id,
            envelope,
            metadata,
        })
    }

    /// Validates the definition using an explicit policy.
    pub fn validate(
        &self,
        policy: EnvelopeValidationPolicy,
    ) -> EnvelopeResult<()> {
        self.envelope.validate_with_policy(policy)?;
        self.metadata.validate(policy)?;
        Ok(())
    }

    /// Evaluates the semantic envelope.
    pub fn evaluate<F>(
        &self,
        t: f64,
        resolver: &F,
    ) -> EnvelopeResult<EnvelopeValue>
    where
        F: Fn(&str) -> Option<f64>,
    {
        self.envelope.evaluate(t, resolver)
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn ensure_finite(
    value: f64,
    field: &'static str,
) -> EnvelopeResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(EnvelopeError::NonFiniteValue { field })
    }
}

fn ensure_positive(
    value: f64,
    field: &'static str,
) -> EnvelopeResult<()> {
    ensure_finite(value, field)?;

    if value > 0.0 {
        Ok(())
    } else {
        Err(EnvelopeError::NonPositiveParameter { field })
    }
}

fn validate_coordinate(t: f64) -> EnvelopeResult<()> {
    ensure_finite(t, "coordinate")?;

    if (0.0..=1.0).contains(&t) {
        Ok(())
    } else {
        Err(EnvelopeError::CoordinateOutOfRange)
    }
}

fn validate_parameter(
    parameter: &Parameter,
) -> EnvelopeResult<()> {
    parameter
        .validate()
        .map_err(|_| EnvelopeError::InvalidParameter(
            "invalid canonical quantum parameter",
        ))
}

fn bind_parameter<F>(
    parameter: &Parameter,
    resolver: &F,
) -> EnvelopeResult<f64>
where
    F: Fn(&str) -> Option<f64>,
{
    match parameter.bind(resolver) {
        Ok(value) => {
            ensure_finite(value, "bound parameter")?;
            Ok(value)
        }

        Err(error) => {
            let message = error.to_string();

            if let Some(name) = extract_parameter_name(&message) {
                Err(EnvelopeError::UnboundParameter(name))
            } else {
                Err(EnvelopeError::InvalidParameter(
                    "parameter binding failed",
                ))
            }
        }
    }
}

fn extract_parameter_name(message: &str) -> Option<String> {
    let prefix = "parameter symbol `";

    let start = message.find(prefix)? + prefix.len();
    let remainder = &message[start..];

    let end = remainder.find('`')?;

    Some(remainder[..end].to_owned())
}

fn parameter_is_symbolic(parameter: &Parameter) -> bool {
    parameter.is_symbolic()
}

fn parameter_is_symbolic_envelope(envelope: &Envelope) -> bool {
    envelope.is_symbolic()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("finite test parameter")
    }

    #[test]
    fn envelope_value_rejects_nan() {
        let result = EnvelopeValue::new(f64::NAN, 0.0);

        assert!(matches!(
            result,
            Err(EnvelopeError::NonFiniteValue {
                field: "in_phase"
            })
        ));
    }

    #[test]
    fn envelope_value_rejects_infinity() {
        let result = EnvelopeValue::new(
            f64::INFINITY,
            0.0,
        );

        assert!(matches!(
            result,
            Err(EnvelopeError::NonFiniteValue {
                field: "in_phase"
            })
        ));
    }

    #[test]
    fn constant_envelope_is_independent_of_coordinate() {
        let envelope =
            Envelope::constant_real(0.5).expect("valid envelope");

        let resolver = |_name: &str| None;

        let first = envelope
            .evaluate(0.0, &resolver)
            .expect("valid evaluation");

        let last = envelope
            .evaluate(1.0, &resolver)
            .expect("valid evaluation");

        assert_eq!(first, last);
    }

    #[test]
    fn coordinate_must_be_normalized() {
        let envelope =
            Envelope::constant_real(1.0).expect("valid envelope");

        let resolver = |_name: &str| None;

        assert!(matches!(
            envelope.evaluate(-0.1, &resolver),
            Err(EnvelopeError::CoordinateOutOfRange)
        ));

        assert!(matches!(
            envelope.evaluate(1.1, &resolver),
            Err(EnvelopeError::CoordinateOutOfRange)
        ));
    }

    #[test]
    fn linear_envelope_has_correct_endpoints() {
        let start =
            EnvelopeValue::real(0.0).expect("finite value");

        let end =
            EnvelopeValue::real(1.0).expect("finite value");

        let envelope =
            Envelope::linear(start, end)
                .expect("valid linear envelope");

        let resolver = |_name: &str| None;

        let at_zero = envelope
            .evaluate(0.0, &resolver)
            .expect("evaluation");

        let at_one = envelope
            .evaluate(1.0, &resolver)
            .expect("evaluation");

        assert_eq!(at_zero.in_phase(), 0.0);
        assert_eq!(at_one.in_phase(), 1.0);
    }

    #[test]
    fn gaussian_accepts_symbolic_parameters() {
        let amplitude =
            Parameter::symbol("amplitude")
                .expect("valid symbol");

        let sigma =
            Parameter::symbol("sigma")
                .expect("valid symbol");

        let center =
            Parameter::symbol("center")
                .expect("valid symbol");

        let envelope =
            GaussianEnvelope::new(
                amplitude,
                sigma,
                center,
            )
            .expect("valid gaussian");

        assert!(envelope.amplitude.is_symbolic());
        assert!(envelope.sigma.is_symbolic());
        assert!(envelope.center.is_symbolic());
    }

    #[test]
    fn gaussian_rejects_zero_sigma() {
        let result = GaussianEnvelope::new(
            constant(1.0),
            constant(0.0),
            constant(0.5),
        );

        assert!(matches!(
            result,
            Err(EnvelopeError::NonPositiveParameter {
                field: "sigma"
            })
        ));
    }

    #[test]
    fn gaussian_evaluates_with_explicit_binding() {
        let envelope = GaussianEnvelope::new(
            Parameter::symbol("a").expect("symbol"),
            Parameter::symbol("s").expect("symbol"),
            Parameter::symbol("c").expect("symbol"),
        )
        .expect("valid gaussian");

        let resolver = |name: &str| match name {
            "a" => Some(1.0),
            "s" => Some(0.1),
            "c" => Some(0.5),
            _ => None,
        };

        let value = envelope
            .evaluate(0.5, &resolver)
            .expect("bound gaussian");

        assert!(value.in_phase().is_finite());
        assert_eq!(value.quadrature(), 0.0);
    }

    #[test]
    fn unbound_symbol_is_explicit_error() {
        let envelope = GaussianEnvelope::new(
            Parameter::symbol("a").expect("symbol"),
            constant(0.1),
            constant(0.5),
        )
        .expect("valid gaussian");

        let resolver = |_name: &str| None;

        assert!(matches!(
            envelope.evaluate(0.5, &resolver),
            Err(EnvelopeError::UnboundParameter(_))
        ));
    }

    #[test]
    fn drag_produces_iq_value() {
        let envelope = DragEnvelope::new(
            constant(1.0),
            constant(0.1),
            constant(0.5),
            constant(0.2),
        )
        .expect("valid drag");

        let resolver = |_name: &str| None;

        let value = envelope
            .evaluate(0.5, &resolver)
            .expect("valid drag evaluation");

        assert!(value.in_phase().is_finite());
        assert!(value.quadrature().is_finite());
    }

    #[test]
    fn sampled_envelope_preserves_source_order() {
        let samples = vec![
            EnvelopeValue::real(0.1).expect("sample"),
            EnvelopeValue::real(0.2).expect("sample"),
            EnvelopeValue::real(0.3).expect("sample"),
        ];

        let envelope =
            SampledEnvelope::new(samples.clone())
                .expect("valid samples");

        assert_eq!(envelope.samples(), samples.as_slice());
        assert_eq!(
            envelope.sample(1).expect("sample").in_phase(),
            0.2
        );
    }

    #[test]
    fn sampled_envelope_does_not_implicitly_interpolate() {
        let envelope =
            Envelope::sampled(vec![
                EnvelopeValue::real(0.0).expect("sample"),
                EnvelopeValue::real(1.0).expect("sample"),
            ])
            .expect("valid sampled envelope");

        let resolver = |_name: &str| None;

        assert!(matches!(
            envelope.evaluate(0.5, &resolver),
            Err(EnvelopeError::NumericalDomainError(_))
        ));
    }

    #[test]
    fn composition_adds_components() {
        let first =
            Envelope::constant_real(0.25)
                .expect("constant");

        let second =
            Envelope::constant_real(0.75)
                .expect("constant");

        let composite =
            Envelope::composite(vec![
                EnvelopeComponent::new(first, 1.0)
                    .expect("component"),
                EnvelopeComponent::new(second, 1.0)
                    .expect("component"),
            ])
            .expect("composite");

        let resolver = |_name: &str| None;

        let result = composite
            .evaluate(0.5, &resolver)
            .expect("evaluation");

        assert!((result.in_phase() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn composition_weight_is_not_implicitly_normalized() {
        let envelope =
            Envelope::constant_real(1.0)
                .expect("constant");

        let composite =
            Envelope::composite(vec![
                EnvelopeComponent::new(envelope, 2.0)
                    .expect("component"),
            ])
            .expect("composite");

        let resolver = |_name: &str| None;

        let result = composite
            .evaluate(0.5, &resolver)
            .expect("evaluation");

        assert_eq!(result.in_phase(), 2.0);
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata = EnvelopeMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata");

        metadata
            .insert("a", "first")
            .expect("metadata");

        let keys = metadata
            .fields()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec!["a".to_owned(), "z".to_owned()]
        );
    }

    #[test]
    fn metadata_rejects_empty_key() {
        let mut metadata = EnvelopeMetadata::new();

        assert!(matches!(
            metadata.insert("", "value"),
            Err(EnvelopeError::InvalidMetadata(_))
        ));
    }

    #[test]
    fn definition_validates() {
        let envelope =
            Envelope::constant_real(0.3)
                .expect("constant");

        let definition =
            EnvelopeDefinition::new(
                EnvelopeId::new(7),
                envelope,
            )
            .expect("definition");

        definition
            .validate(EnvelopeValidationPolicy::default())
            .expect("valid definition");
    }

    #[test]
    fn symbolic_status_is_preserved() {
        let envelope =
            Envelope::gaussian(
                Parameter::symbol("a")
                    .expect("symbol"),
                constant(0.1),
                constant(0.5),
            )
            .expect("gaussian");

        assert!(envelope.is_symbolic());
    }

    #[test]
    fn node_count_is_finite_and_structural() {
        let first =
            Envelope::constant_real(1.0)
                .expect("constant");

        let second =
            Envelope::constant_real(2.0)
                .expect("constant");

        let composite =
            Envelope::composite(vec![
                EnvelopeComponent::new(first, 1.0)
                    .expect("component"),
                EnvelopeComponent::new(second, 1.0)
                    .expect("component"),
            ])
            .expect("composite");

        assert_eq!(composite.node_count(), 3);
    }

    #[test]
    fn policy_can_be_smaller_than_default() {
        let policy = EnvelopeValidationPolicy::new(
            1,
            8,
            8,
            32,
            64,
        );

        let result = SampledEnvelope::new_with_policy(
            vec![
                EnvelopeValue::real(0.0)
                    .expect("sample"),
                EnvelopeValue::real(1.0)
                    .expect("sample"),
            ],
            policy,
        );

        assert!(matches!(
            result,
            Err(EnvelopeError::SampleLimitExceeded {
                actual: 2,
                maximum: 1
            })
        ));
    }
}