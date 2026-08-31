//! Zamani Quantum IR — Pulse Waveform Semantics
//!
//! Canonical, hardware-independent waveform definitions for the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural contract
//!
//! This module defines WHAT a waveform means.
//!
//! It does NOT define:
//!
//! - a physical DAC;
//! - an ADC;
//! - a hardware sample clock;
//! - a physical control channel;
//! - a physical qubit;
//! - a hardware topology;
//! - routing;
//! - scheduling;
//! - calibration execution;
//! - vendor SDKs;
//! - provider authentication;
//! - QPU execution;
//! - simulator state;
//! - optimization policy;
//! - frontend syntax.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Dependency direction
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      +---------------------------+
//!      |                           |
//!      v                           v
//! pulse::waveform              pulse
//!      |                           |
//!      +-------------+-------------+
//!                    |
//!                    v
//!              optimization
//!                    |
//!                    v
//!               scheduling
//!                    |
//!                    v
//!                hardware
//!                    |
//!                    v
//!                 backend
//! ```
//!
//! `waveform.rs` therefore has no dependency on `quantum::ir::qubit`.
//! A waveform is reusable independently of which logical or physical qubit
//! eventually consumes a pulse containing it.
//!
//! # Universal-program principle
//!
//! A waveform definition must not encode a machine-size assumption.
//!
//! The following are resource instances, not architectural limits:
//!
//! ```text
//! 1 sample
//! 1,000 samples
//! 1,000,000 samples
//! N samples
//! ```
//!
//! A validator may impose an explicit resource policy for one compilation,
//! service invocation, or trust boundary. Such a policy MUST NOT become the
//! semantic maximum of Zamani.
//!
//! # Semantic separation
//!
//! ```text
//! Waveform  = signal shape
//! Pulse     = execution/control event
//! Channel   = abstract destination/source
//! Frame     = phase/frequency reference
//! Timing    = temporal placement
//! Hardware  = physical realization
//! ```
//!
//! # Important numerical rule
//!
//! Concrete floating-point values entering the canonical waveform model must
//! be finite. NaN and positive/negative infinity are rejected.
//!
//! The IR does NOT universally impose an amplitude range such as:
//!
//! ```text
//! -1 <= amplitude <= 1
//! ```
//!
//! because amplitude units and physical ranges are target dependent.
//!
//! # Parametric versus sampled waveforms
//!
//! Parametric waveforms remain symbolic until a downstream target actually
//! requires samples.
//!
//! Sampled waveforms contain explicit samples in source order.
//!
//! This module never silently:
//!
//! - resamples;
//! - interpolates;
//! - clips;
//! - normalizes;
//! - quantizes;
//! - truncates;
//! - changes sample order.
//!
//! Those are explicit transformations owned by downstream passes.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe
//! - no external dependency
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `identity.rs`
//!     Supplies `WaveformId`.
//!
//! `parameter.rs`
//!     Supplies symbolic scalar parameters.
//!
//! `pulse.rs`
//!     References waveform definitions.
//!
//! `channel.rs`
//!     Owns abstract channel semantics.
//!
//! `frame.rs`
//!     Owns phase/frequency frame semantics.
//!
//! `timing.rs`
//!     Owns program-level temporal semantics.
//!
//! `schedule.rs`
//!     Owns execution placement.
//!
//! `serialization.rs`
//!     Owns canonical persistence.
//!
//! `hash.rs`
//!     Owns canonical content hashing.
//!
//! `validation.rs`
//!     Owns whole-program validation.
//!
//! Hardware/backend layers
//!     Materialize semantic waveform definitions into target-specific
//!     representations.
//!
//! No later addition of channel, frame, timing, scheduler, backend, or hardware
//! implementation should require changing the semantic meaning of this file.
//!
//! -----------------------------------------------------------------------------
//! Safety
//! -----------------------------------------------------------------------------
//!
//! No unsafe Rust is permitted.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::identity::WaveformId;
use super::super::parameter::Parameter;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the waveform dialect.
pub const WAVEFORM_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.waveform";

/// Semantic schema major version.
///
/// Breaking changes to the meaning of existing waveform constructs require
/// an IR/schema migration.
pub const WAVEFORM_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const WAVEFORM_SCHEMA_MINOR: u16 = 0;

/// Semantic schema patch version.
pub const WAVEFORM_SCHEMA_PATCH: u16 = 0;

/// Returns the waveform schema version as `(major, minor, patch)`.
#[must_use]
pub const fn waveform_schema_version() -> (u16, u16, u16) {
    (
        WAVEFORM_SCHEMA_MAJOR,
        WAVEFORM_SCHEMA_MINOR,
        WAVEFORM_SCHEMA_PATCH,
    )
}

// =============================================================================
// Result
// =============================================================================

/// Result produced by waveform construction and validation.
pub type WaveformResult<T> = Result<T, WaveformError>;

// =============================================================================
// Sample rate
// =============================================================================

/// Semantic waveform sample rate expressed in samples per second.
///
/// This is NOT necessarily the physical device clock.
///
/// A downstream target may map this semantic rate to a target-specific clock,
/// interpolation scheme, or sampling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WaveformSampleRate {
    samples_per_second: u64,
}

impl WaveformSampleRate {
    /// Creates a non-zero sample rate.
    pub const fn new(samples_per_second: u64) -> WaveformResult<Self> {
        if samples_per_second == 0 {
            Err(WaveformError::ZeroSampleRate)
        } else {
            Ok(Self {
                samples_per_second,
            })
        }
    }

    /// Returns samples per second.
    #[must_use]
    pub const fn samples_per_second(self) -> u64 {
        self.samples_per_second
    }

    /// Returns the rate interpreted as hertz.
    #[must_use]
    pub const fn hertz(self) -> u64 {
        self.samples_per_second
    }
}

impl fmt::Display for WaveformSampleRate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}Sa/s", self.samples_per_second)
    }
}

// =============================================================================
// Complex sample
// =============================================================================

/// Finite complex IQ sample.
///
/// `i` is the in-phase component and `q` is the quadrature component.
///
/// No vendor-specific IQ convention is encoded here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexSample {
    i: f64,
    q: f64,
}

impl ComplexSample {
    /// Creates a finite IQ sample.
    pub fn new(i: f64, q: f64) -> WaveformResult<Self> {
        ensure_finite(i, "I component")?;
        ensure_finite(q, "Q component")?;

        Ok(Self { i, q })
    }

    /// Creates a real-valued sample.
    pub fn real(value: f64) -> WaveformResult<Self> {
        Self::new(value, 0.0)
    }

    /// Creates zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self { i: 0.0, q: 0.0 }
    }

    /// Returns the I component.
    #[must_use]
    pub const fn i(self) -> f64 {
        self.i
    }

    /// Returns the Q component.
    #[must_use]
    pub const fn q(self) -> f64 {
        self.q
    }

    /// Returns whether both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.i.is_finite() && self.q.is_finite()
    }

    /// Calculates squared magnitude with explicit finite-result checking.
    ///
    /// Floating-point multiplication has no `checked_mul` operation in Rust.
    /// Overflow is therefore detected by checking the result for finiteness.
    pub fn magnitude_squared(self) -> WaveformResult<f64> {
        self.validate()?;

        let ii = self.i * self.i;
        if !ii.is_finite() {
            return Err(WaveformError::NumericalOverflow);
        }

        let qq = self.q * self.q;
        if !qq.is_finite() {
            return Err(WaveformError::NumericalOverflow);
        }

        let result = ii + qq;

        if result.is_finite() {
            Ok(result)
        } else {
            Err(WaveformError::NumericalOverflow)
        }
    }

    /// Calculates magnitude with explicit finite-result checking.
    pub fn magnitude(self) -> WaveformResult<f64> {
        let squared = self.magnitude_squared()?;
        let magnitude = squared.sqrt();

        if magnitude.is_finite() {
            Ok(magnitude)
        } else {
            Err(WaveformError::NumericalOverflow)
        }
    }

    /// Validates the sample.
    pub fn validate(&self) -> WaveformResult<()> {
        ensure_finite(self.i, "I component")?;
        ensure_finite(self.q, "Q component")?;
        Ok(())
    }
}

impl Default for ComplexSample {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Sample format
// =============================================================================

/// Representation of explicitly stored samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformSampleFormat {
    /// Real-valued samples.
    Real,

    /// Complex IQ samples.
    ComplexIq,
}

impl Default for WaveformSampleFormat {
    fn default() -> Self {
        Self::Real
    }
}

// =============================================================================
// Normalization semantics
// =============================================================================

/// Declarative normalization semantics.
///
/// This enum describes meaning. It does not cause implicit numerical
/// transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformNormalization {
    /// No normalization declaration.
    None,

    /// Declared unit peak magnitude.
    UnitPeak,

    /// Declared unit energy.
    UnitEnergy,

    /// Explicitly normalized by a source/compiler transformation.
    Explicit,
}

impl Default for WaveformNormalization {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// Domain
// =============================================================================

/// Mathematical domain of a waveform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformDomain {
    /// Normalized domain `[0, 1]`.
    Normalized,

    /// Explicit normalized-domain interval.
    ///
    /// The actual bounds are stored by the waveform definition that requires
    /// them rather than being hidden in this enum.
    Explicit,
}

impl Default for WaveformDomain {
    fn default() -> Self {
        Self::Normalized
    }
}

// =============================================================================
// Constant
// =============================================================================

/// Constant-valued waveform.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantWaveform {
    /// Constant complex value.
    pub value: ComplexSample,
}

impl ConstantWaveform {
    /// Creates a constant waveform.
    pub fn new(value: ComplexSample) -> WaveformResult<Self> {
        value.validate()?;
        Ok(Self { value })
    }

    /// Creates a real constant waveform.
    pub fn real(value: f64) -> WaveformResult<Self> {
        Self::new(ComplexSample::real(value)?)
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        self.value.validate()
    }
}

// =============================================================================
// Square
// =============================================================================

/// Square waveform.
#[derive(Debug, Clone, PartialEq)]
pub struct SquareWaveform {
    /// Complex amplitude.
    pub amplitude: ComplexSample,
}

impl SquareWaveform {
    /// Creates a square waveform.
    pub fn new(amplitude: ComplexSample) -> WaveformResult<Self> {
        amplitude.validate()?;
        Ok(Self { amplitude })
    }

    /// Creates a real square waveform.
    pub fn real(amplitude: f64) -> WaveformResult<Self> {
        Self::new(ComplexSample::real(amplitude)?)
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        self.amplitude.validate()
    }
}

// =============================================================================
// Gaussian
// =============================================================================

/// Gaussian waveform envelope.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianWaveform {
    /// Peak amplitude.
    pub amplitude: Parameter,

    /// Standard deviation in normalized-domain units.
    pub sigma: Parameter,

    /// Center in normalized-domain units.
    pub center: Parameter,
}

impl GaussianWaveform {
    /// Creates a Gaussian waveform.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
    ) -> WaveformResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&sigma)?;
        validate_parameter(&center)?;

        if let Some(value) = sigma.as_constant() {
            if value <= 0.0 {
                return Err(WaveformError::InvalidParameter(
                    "Gaussian sigma must be greater than zero",
                ));
            }
        }

        Ok(Self {
            amplitude,
            sigma,
            center,
        })
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.sigma.clone(),
            self.center.clone(),
        )
        .map(|_| ())
    }
}

// =============================================================================
// DRAG
// =============================================================================

/// DRAG waveform envelope.
///
/// The exact physical implementation is target dependent.
#[derive(Debug, Clone, PartialEq)]
pub struct DragWaveform {
    /// Base Gaussian amplitude.
    pub amplitude: Parameter,

    /// Gaussian standard deviation.
    pub sigma: Parameter,

    /// Gaussian center.
    pub center: Parameter,

    /// Derivative correction coefficient.
    pub beta: Parameter,
}

impl DragWaveform {
    /// Creates a DRAG waveform.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
        beta: Parameter,
    ) -> WaveformResult<Self> {
        validate_parameter(&amplitude)?;
        validate_parameter(&sigma)?;
        validate_parameter(&center)?;
        validate_parameter(&beta)?;

        if let Some(value) = sigma.as_constant() {
            if value <= 0.0 {
                return Err(WaveformError::InvalidParameter(
                    "DRAG sigma must be greater than zero",
                ));
            }
        }

        Ok(Self {
            amplitude,
            sigma,
            center,
            beta,
        })
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        Self::new(
            self.amplitude.clone(),
            self.sigma.clone(),
            self.center.clone(),
            self.beta.clone(),
        )
        .map(|_| ())
    }
}

// =============================================================================
// Sine
// =============================================================================

/// Sinusoidal waveform.
#[derive(Debug, Clone, PartialEq)]
pub struct SinusoidalWaveform {
    /// Peak complex amplitude.
    pub amplitude: ComplexSample,

    /// Number of cycles over the normalized domain.
    pub cycles: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,
}

impl SinusoidalWaveform {
    /// Creates a sine waveform.
    pub fn new(
        amplitude: ComplexSample,
        cycles: Parameter,
        phase: Parameter,
    ) -> WaveformResult<Self> {
        amplitude.validate()?;
        validate_parameter(&cycles)?;
        validate_parameter(&phase)?;

        Ok(Self {
            amplitude,
            cycles,
            phase,
        })
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        Self::new(
            self.amplitude,
            self.cycles.clone(),
            self.phase.clone(),
        )
        .map(|_| ())
    }
}

// =============================================================================
// Cosine
// =============================================================================

/// Cosine waveform.
#[derive(Debug, Clone, PartialEq)]
pub struct CosineWaveform {
    /// Peak complex amplitude.
    pub amplitude: ComplexSample,

    /// Number of cycles over the normalized domain.
    pub cycles: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,
}

impl CosineWaveform {
    /// Creates a cosine waveform.
    pub fn new(
        amplitude: ComplexSample,
        cycles: Parameter,
        phase: Parameter,
    ) -> WaveformResult<Self> {
        amplitude.validate()?;
        validate_parameter(&cycles)?;
        validate_parameter(&phase)?;

        Ok(Self {
            amplitude,
            cycles,
            phase,
        })
    }

    /// Validates the definition.
    pub fn validate(&self) -> WaveformResult<()> {
        Self::new(
            self.amplitude,
            self.cycles.clone(),
            self.phase.clone(),
        )
        .map(|_| ())
    }
}

// =============================================================================
// Arbitrary sampled waveform
// =============================================================================

/// Explicitly sampled waveform.
///
/// Samples are stored in temporal order.
///
/// No implicit resampling, interpolation, clipping, normalization or
/// quantization is performed.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledWaveform {
    samples: Vec<ComplexSample>,
    sample_rate: Option<WaveformSampleRate>,
    format: WaveformSampleFormat,
}

impl SampledWaveform {
    /// Creates a sampled waveform.
    pub fn new(
        samples: Vec<ComplexSample>,
        sample_rate: Option<WaveformSampleRate>,
        format: WaveformSampleFormat,
    ) -> WaveformResult<Self> {
        let waveform = Self {
            samples,
            sample_rate,
            format,
        };

        waveform.validate()?;
        Ok(waveform)
    }

    /// Creates a real sampled waveform.
    pub fn from_real(
        samples: Vec<f64>,
        sample_rate: Option<WaveformSampleRate>,
    ) -> WaveformResult<Self> {
        let mut converted = Vec::with_capacity(samples.len());

        for value in samples {
            converted.push(ComplexSample::real(value)?);
        }

        Self::new(
            converted,
            sample_rate,
            WaveformSampleFormat::Real,
        )
    }

    /// Creates a complex IQ sampled waveform.
    pub fn from_iq(
        samples: Vec<ComplexSample>,
        sample_rate: Option<WaveformSampleRate>,
    ) -> WaveformResult<Self> {
        Self::new(
            samples,
            sample_rate,
            WaveformSampleFormat::ComplexIq,
        )
    }

    /// Returns sample count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether the waveform has no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns samples in exact source order.
    #[must_use]
    pub fn samples(&self) -> &[ComplexSample] {
        &self.samples
    }

    /// Returns the semantic sample rate.
    #[must_use]
    pub const fn sample_rate(&self) -> Option<WaveformSampleRate> {
        self.sample_rate
    }

    /// Returns the sample format.
    #[must_use]
    pub const fn format(&self) -> WaveformSampleFormat {
        self.format
    }

    /// Returns one sample.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<ComplexSample> {
        self.samples.get(index).copied()
    }

    /// Calculates discrete sample energy.
    ///
    /// This is a mathematical property of the stored samples. It does not
    /// claim to be physical pulse energy unless the consuming model defines
    /// the appropriate units.
    pub fn energy(&self) -> WaveformResult<f64> {
        self.validate()?;

        let mut total = 0.0_f64;

        for sample in &self.samples {
            let contribution = sample.magnitude_squared()?;
            let next = total + contribution;

            if !next.is_finite() {
                return Err(WaveformError::NumericalOverflow);
            }

            total = next;
        }

        Ok(total)
    }

    /// Calculates maximum sample magnitude.
    pub fn peak_magnitude(&self) -> WaveformResult<f64> {
        self.validate()?;

        let mut peak = 0.0_f64;

        for sample in &self.samples {
            let magnitude = sample.magnitude()?;

            if magnitude > peak {
                peak = magnitude;
            }
        }

        Ok(peak)
    }

    /// Validates all samples and format invariants.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.samples.is_empty() {
            return Err(WaveformError::EmptySampledWaveform);
        }

        if let Some(rate) = self.sample_rate {
            if rate.samples_per_second() == 0 {
                return Err(WaveformError::ZeroSampleRate);
            }
        }

        for sample in &self.samples {
            sample.validate()?;

            if matches!(self.format, WaveformSampleFormat::Real)
                && sample.q() != 0.0
            {
                return Err(WaveformError::SampleFormatMismatch);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Piecewise
// =============================================================================

/// One piecewise waveform segment.
///
/// Segment positions are represented symbolically so that a later compiler
/// may bind them without modifying the waveform definition.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseSegment {
    /// Segment start.
    pub start: Parameter,

    /// Segment end.
    pub end: Parameter,

    /// Segment waveform.
    pub waveform: Box<WaveformKind>,
}

impl PiecewiseSegment {
    /// Creates a segment.
    pub fn new(
        start: Parameter,
        end: Parameter,
        waveform: WaveformKind,
    ) -> WaveformResult<Self> {
        validate_parameter(&start)?;
        validate_parameter(&end)?;
        waveform.validate()?;

        validate_segment_bounds(&start, &end)?;

        Ok(Self {
            start,
            end,
            waveform: Box::new(waveform),
        })
    }

    /// Validates the segment.
    pub fn validate(&self) -> WaveformResult<()> {
        validate_parameter(&self.start)?;
        validate_parameter(&self.end)?;
        validate_segment_bounds(&self.start, &self.end)?;
        self.waveform.validate()
    }
}

/// Piecewise waveform.
///
/// Segment order is semantically significant and is therefore preserved.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseWaveform {
    segments: Vec<PiecewiseSegment>,
}

impl PiecewiseWaveform {
    /// Creates a piecewise waveform.
    pub fn new(
        segments: Vec<PiecewiseSegment>,
    ) -> WaveformResult<Self> {
        if segments.is_empty() {
            return Err(WaveformError::EmptyPiecewiseWaveform);
        }

        for segment in &segments {
            segment.validate()?;
        }

        Ok(Self { segments })
    }

    /// Returns segments in semantic order.
    #[must_use]
    pub fn segments(&self) -> &[PiecewiseSegment] {
        &self.segments
    }

    /// Returns number of segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether there are no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Validates the piecewise structure.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.segments.is_empty() {
            return Err(WaveformError::EmptyPiecewiseWaveform);
        }

        for segment in &self.segments {
            segment.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Composition
// =============================================================================

/// Semantic composition operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformComposition {
    /// Pointwise addition.
    Add,

    /// Pointwise multiplication.
    Multiply,

    /// Temporal concatenation.
    Concatenate,

    /// Envelope/modulation composition.
    Modulate,
}

impl Default for WaveformComposition {
    fn default() -> Self {
        Self::Concatenate
    }
}

/// Composite waveform.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeWaveform {
    /// Composition operation.
    pub operation: WaveformComposition,

    /// Ordered child definitions.
    components: Vec<Box<WaveformKind>>,
}

impl CompositeWaveform {
    /// Creates a composite waveform.
    pub fn new(
        operation: WaveformComposition,
        components: Vec<WaveformKind>,
    ) -> WaveformResult<Self> {
        if components.is_empty() {
            return Err(WaveformError::EmptyCompositeWaveform);
        }

        for component in &components {
            component.validate()?;
        }

        let components = components
            .into_iter()
            .map(Box::new)
            .collect();

        Ok(Self {
            operation,
            components,
        })
    }

    /// Returns composition operation.
    #[must_use]
    pub const fn operation(&self) -> WaveformComposition {
        self.operation
    }

    /// Returns ordered components.
    #[must_use]
    pub fn components(&self) -> &[Box<WaveformKind>] {
        &self.components
    }

    /// Returns component count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether there are no components.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Validates the composite definition.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.components.is_empty() {
            return Err(WaveformError::EmptyCompositeWaveform);
        }

        for component in &self.components {
            component.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Extension waveform
// =============================================================================

/// Namespaced semantic waveform extension.
///
/// Custom waveforms contain descriptive data only. They cannot contain
/// executable Rust, shell commands, provider credentials, or arbitrary code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomWaveform {
    namespace: String,
    kind: String,
    parameters: BTreeMap<String, String>,
}

impl CustomWaveform {
    /// Creates a custom waveform reference.
    pub fn new<N, K>(
        namespace: N,
        kind: K,
        parameters: BTreeMap<String, String>,
    ) -> WaveformResult<Self>
    where
        N: Into<String>,
        K: Into<String>,
    {
        let namespace = namespace.into();
        let kind = kind.into();

        validate_extension_name(
            &namespace,
            WaveformError::EmptyExtensionNamespace,
        )?;

        validate_extension_name(
            &kind,
            WaveformError::EmptyExtensionKind,
        )?;

        let waveform = Self {
            namespace,
            kind,
            parameters,
        };

        waveform.validate()?;
        Ok(waveform)
    }

    /// Returns extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns extension kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns deterministic parameters.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    /// Validates names and parameter structure.
    pub fn validate(&self) -> WaveformResult<()> {
        validate_extension_name(
            &self.namespace,
            WaveformError::EmptyExtensionNamespace,
        )?;

        validate_extension_name(
            &self.kind,
            WaveformError::EmptyExtensionKind,
        )?;

        for (key, _) in &self.parameters {
            if key.trim().is_empty() {
                return Err(WaveformError::EmptyExtensionParameterKey);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Waveform kind
// =============================================================================

/// Canonical semantic waveform taxonomy.
///
/// Standard forms are intentionally finite, but `Custom` prevents the
/// standard taxonomy from becoming the architectural limit of Zamani.
#[derive(Debug, Clone, PartialEq)]
pub enum WaveformKind {
    /// Constant signal.
    Constant(ConstantWaveform),

    /// Square signal.
    Square(SquareWaveform),

    /// Gaussian envelope.
    Gaussian(GaussianWaveform),

    /// DRAG envelope.
    Drag(DragWaveform),

    /// Sine signal.
    Sine(SinusoidalWaveform),

    /// Cosine signal.
    Cosine(CosineWaveform),

    /// Explicitly sampled signal.
    Sampled(SampledWaveform),

    /// Piecewise signal.
    Piecewise(PiecewiseWaveform),

    /// Composite signal.
    Composite(CompositeWaveform),

    /// Extension-defined signal.
    Custom(CustomWaveform),
}

impl WaveformKind {
    /// Validates the complete semantic structure.
    pub fn validate(&self) -> WaveformResult<()> {
        match self {
            Self::Constant(value) => value.validate(),
            Self::Square(value) => value.validate(),
            Self::Gaussian(value) => value.validate(),
            Self::Drag(value) => value.validate(),
            Self::Sine(value) => value.validate(),
            Self::Cosine(value) => value.validate(),
            Self::Sampled(value) => value.validate(),
            Self::Piecewise(value) => value.validate(),
            Self::Composite(value) => value.validate(),
            Self::Custom(value) => value.validate(),
        }
    }

    /// Returns a stable semantic name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Constant(_) => "constant",
            Self::Square(_) => "square",
            Self::Gaussian(_) => "gaussian",
            Self::Drag(_) => "drag",
            Self::Sine(_) => "sine",
            Self::Cosine(_) => "cosine",
            Self::Sampled(_) => "sampled",
            Self::Piecewise(_) => "piecewise",
            Self::Composite(_) => "composite",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns whether the definition directly contains samples.
    #[must_use]
    pub const fn is_sampled(&self) -> bool {
        matches!(self, Self::Sampled(_))
    }

    /// Returns whether the definition is parametric.
    #[must_use]
    pub const fn is_parametric(&self) -> bool {
        matches!(
            self,
            Self::Gaussian(_)
                | Self::Drag(_)
                | Self::Sine(_)
                | Self::Cosine(_)
        )
    }

    /// Returns whether the definition contains composition.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        matches!(self, Self::Piecewise(_) | Self::Composite(_))
    }

    /// Counts directly represented samples without recursive traversal.
    ///
    /// For nested definitions this intentionally counts only directly stored
    /// samples. Whole-tree resource accounting belongs to
    /// `Waveform::resource_usage`.
    #[must_use]
    pub fn direct_sample_count(&self) -> usize {
        match self {
            Self::Sampled(waveform) => waveform.len(),
            Self::Constant(_)
            | Self::Square(_)
            | Self::Gaussian(_)
            | Self::Drag(_)
            | Self::Sine(_)
            | Self::Cosine(_)
            | Self::Piecewise(_)
            | Self::Composite(_)
            | Self::Custom(_) => 0,
        }
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Deterministic descriptive waveform metadata.
///
/// `BTreeMap` is deliberately used instead of `HashMap` so serialization and
/// iteration order remain deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WaveformMetadata {
    entries: BTreeMap<String, String>,
}

impl WaveformMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts metadata.
    ///
    /// Resource limits are deliberately not embedded here. Validation policy
    /// controls resource consumption.
    pub fn insert<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> WaveformResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        Ok(self.entries.insert(key, value))
    }

    /// Returns one metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns all metadata in deterministic order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Returns metadata count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validates metadata under an explicit policy.
    pub fn validate(
        &self,
        policy: WaveformValidationPolicy,
    ) -> WaveformResult<()> {
        if self.entries.len() > policy.max_metadata_fields {
            return Err(WaveformError::MetadataFieldLimitExceeded {
                actual: self.entries.len(),
                limit: policy.max_metadata_fields,
            });
        }

        for (key, value) in &self.entries {
            validate_metadata_key_with_policy(key, policy)?;
            validate_metadata_value_with_policy(value, policy)?;
        }

        Ok(())
    }
}

// =============================================================================
// Waveform
// =============================================================================

/// Complete canonical waveform definition.
///
/// A waveform is reusable and independent of:
///
/// - logical qubits;
/// - physical qubits;
/// - channels;
/// - frames;
/// - schedules;
/// - hardware.
#[derive(Debug, Clone, PartialEq)]
pub struct Waveform {
    id: WaveformId,
    kind: WaveformKind,
    normalization: WaveformNormalization,
    domain: WaveformDomain,
    sample_rate: Option<WaveformSampleRate>,
    metadata: WaveformMetadata,
}

impl Waveform {
    /// Creates a waveform with default semantic configuration.
    pub fn new(
        id: WaveformId,
        kind: WaveformKind,
    ) -> WaveformResult<Self> {
        Self::with_configuration(
            id,
            kind,
            WaveformNormalization::None,
            WaveformDomain::Normalized,
            None,
            WaveformMetadata::default(),
        )
    }

    /// Creates a waveform with explicit configuration.
    pub fn with_configuration(
        id: WaveformId,
        kind: WaveformKind,
        normalization: WaveformNormalization,
        domain: WaveformDomain,
        sample_rate: Option<WaveformSampleRate>,
        metadata: WaveformMetadata,
    ) -> WaveformResult<Self> {
        let waveform = Self {
            id,
            kind,
            normalization,
            domain,
            sample_rate,
            metadata,
        };

        waveform.validate()?;
        Ok(waveform)
    }

    /// Returns stable waveform identity.
    #[must_use]
    pub const fn id(&self) -> WaveformId {
        self.id
    }

    /// Returns semantic waveform definition.
    #[must_use]
    pub fn kind(&self) -> &WaveformKind {
        &self.kind
    }

    /// Returns normalization semantics.
    #[must_use]
    pub const fn normalization(&self) -> WaveformNormalization {
        self.normalization
    }

    /// Returns domain semantics.
    #[must_use]
    pub const fn domain(&self) -> WaveformDomain {
        self.domain
    }

    /// Returns optional semantic sample rate.
    #[must_use]
    pub const fn sample_rate(&self) -> Option<WaveformSampleRate> {
        self.sample_rate
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub fn metadata(&self) -> &WaveformMetadata {
        &self.metadata
    }

    /// Returns stable semantic kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        self.kind.kind_name()
    }

    /// Returns whether directly sampled.
    #[must_use]
    pub const fn is_sampled(&self) -> bool {
        self.kind.is_sampled()
    }

    /// Returns whether parametric.
    #[must_use]
    pub const fn is_parametric(&self) -> bool {
        self.kind.is_parametric()
    }

    /// Returns whether composite.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        self.kind.is_composite()
    }

    /// Returns directly stored sample count.
    #[must_use]
    pub fn direct_sample_count(&self) -> usize {
        self.kind.direct_sample_count()
    }

    /// Returns resource usage using an iterative traversal.
    ///
    /// This avoids using Rust call-stack depth proportional to waveform nesting.
    pub fn resource_usage(&self) -> WaveformResult<WaveformResourceUsage> {
        let mut usage = WaveformResourceUsage::default();

        let mut stack: Vec<(&WaveformKind, usize)> = Vec::new();
        stack.push((&self.kind, 0));

        while let Some((kind, depth)) = stack.pop() {
            usage.node_count = usage
                .node_count
                .checked_add(1)
                .ok_or(WaveformError::SizeOverflow)?;

            usage.max_nesting_depth = usage.max_nesting_depth.max(depth);

            match kind {
                WaveformKind::Sampled(sampled) => {
                    usage.sample_count = usage
                        .sample_count
                        .checked_add(sampled.len())
                        .ok_or(WaveformError::SizeOverflow)?;
                }

                WaveformKind::Piecewise(piecewise) => {
                    usage.piecewise_segment_count = usage
                        .piecewise_segment_count
                        .checked_add(piecewise.len())
                        .ok_or(WaveformError::SizeOverflow)?;

                    for segment in piecewise.segments().iter().rev() {
                        stack.push((
                            segment.waveform.as_ref(),
                            depth
                                .checked_add(1)
                                .ok_or(WaveformError::SizeOverflow)?,
                        ));
                    }
                }

                WaveformKind::Composite(composite) => {
                    usage.composite_component_count = usage
                        .composite_component_count
                        .checked_add(composite.len())
                        .ok_or(WaveformError::SizeOverflow)?;

                    for component in composite.components().iter().rev() {
                        stack.push((
                            component.as_ref(),
                            depth
                                .checked_add(1)
                                .ok_or(WaveformError::SizeOverflow)?,
                        ));
                    }
                }

                WaveformKind::Constant(_)
                | WaveformKind::Square(_)
                | WaveformKind::Gaussian(_)
                | WaveformKind::Drag(_)
                | WaveformKind::Sine(_)
                | WaveformKind::Cosine(_)
                | WaveformKind::Custom(_) => {}
            }
        }

        Ok(usage)
    }

    /// Validates using the default policy.
    pub fn validate(&self) -> WaveformResult<()> {
        self.validate_with_policy(WaveformValidationPolicy::default())
    }

    /// Validates using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: WaveformValidationPolicy,
    ) -> WaveformResult<()> {
        policy.validate()?;

        self.metadata.validate(policy)?;

        if let Some(rate) = self.sample_rate {
            if rate.samples_per_second() == 0 {
                return Err(WaveformError::ZeroSampleRate);
            }
        }

        let usage = self.resource_usage()?;

        if usage.sample_count > policy.max_samples {
            return Err(WaveformError::SampleLimitExceeded {
                actual: usage.sample_count,
                limit: policy.max_samples,
            });
        }

        if usage.composite_component_count
            > policy.max_composite_components
        {
            return Err(
                WaveformError::CompositeComponentLimitExceeded {
                    actual: usage.composite_component_count,
                    limit: policy.max_composite_components,
                },
            );
        }

        if usage.piecewise_segment_count
            > policy.max_piecewise_segments
        {
            return Err(
                WaveformError::PiecewiseSegmentLimitExceeded {
                    actual: usage.piecewise_segment_count,
                    limit: policy.max_piecewise_segments,
                },
            );
        }

        if usage.node_count > policy.max_nodes {
            return Err(WaveformError::NodeLimitExceeded {
                actual: usage.node_count,
                limit: policy.max_nodes,
            });
        }

        if usage.max_nesting_depth > policy.max_nesting_depth {
            return Err(WaveformError::NestingDepthExceeded {
                actual: usage.max_nesting_depth,
                limit: policy.max_nesting_depth,
            });
        }

        /*
         * Validate semantic nodes iteratively.
         *
         * Constructors validate their immediate structure, but a waveform can
         * be assembled through public fields or future deserialization paths.
         * Whole-tree validation therefore remains mandatory.
         */
        let mut stack: Vec<&WaveformKind> = Vec::new();
        stack.push(&self.kind);

        while let Some(kind) = stack.pop() {
            match kind {
                WaveformKind::Constant(value) => value.validate()?,
                WaveformKind::Square(value) => value.validate()?,
                WaveformKind::Gaussian(value) => value.validate()?,
                WaveformKind::Drag(value) => value.validate()?,
                WaveformKind::Sine(value) => value.validate()?,
                WaveformKind::Cosine(value) => value.validate()?,
                WaveformKind::Sampled(value) => value.validate()?,

                WaveformKind::Piecewise(value) => {
                    value.validate()?;

                    for segment in value.segments().iter().rev() {
                        stack.push(segment.waveform.as_ref());
                    }
                }

                WaveformKind::Composite(value) => {
                    value.validate()?;

                    for component in value.components().iter().rev() {
                        stack.push(component.as_ref());
                    }
                }

                WaveformKind::Custom(value) => value.validate()?,
            }
        }

        Ok(())
    }

    /// Returns a compact deterministic summary.
    pub fn summary(&self) -> WaveformSummary {
        let usage = self.resource_usage().unwrap_or_default();

        WaveformSummary {
            id: self.id,
            kind: self.kind_name(),
            node_count: usage.node_count,
            sample_count: usage.sample_count,
            piecewise_segment_count: usage.piecewise_segment_count,
            composite_component_count: usage.composite_component_count,
            max_nesting_depth: usage.max_nesting_depth,
            sampled: self.is_sampled(),
            parametric: self.is_parametric(),
            composite: self.is_composite(),
            normalization: self.normalization,
            domain: self.domain,
            sample_rate: self.sample_rate,
            metadata_fields: self.metadata.len(),
        }
    }

    /// Changes normalization semantics without changing numerical data.
    #[must_use]
    pub fn with_normalization(
        mut self,
        normalization: WaveformNormalization,
    ) -> Self {
        self.normalization = normalization;
        self
    }

    /// Changes domain semantics.
    #[must_use]
    pub fn with_domain(
        mut self,
        domain: WaveformDomain,
    ) -> Self {
        self.domain = domain;
        self
    }

    /// Adds a semantic sample-rate declaration.
    #[must_use]
    pub fn with_sample_rate(
        mut self,
        sample_rate: WaveformSampleRate,
    ) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Adds deterministic metadata.
    pub fn with_metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> WaveformResult<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.metadata.insert(key, value)?;
        Ok(self)
    }
}

// =============================================================================
// Resource usage
// =============================================================================

/// Resource usage observed in one waveform definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WaveformResourceUsage {
    /// Number of waveform nodes.
    pub node_count: usize,

    /// Total explicitly stored samples across the waveform tree.
    pub sample_count: usize,

    /// Number of piecewise segments.
    pub piecewise_segment_count: usize,

    /// Number of composite children.
    pub composite_component_count: usize,

    /// Maximum nested semantic depth.
    pub max_nesting_depth: usize,
}

// =============================================================================
// Summary
// =============================================================================

/// Compact deterministic waveform summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveformSummary {
    /// Waveform identity.
    pub id: WaveformId,

    /// Stable semantic kind.
    pub kind: &'static str,

    /// Number of semantic nodes.
    pub node_count: usize,

    /// Total explicit sample count.
    pub sample_count: usize,

    /// Piecewise segment count.
    pub piecewise_segment_count: usize,

    /// Composite child count.
    pub composite_component_count: usize,

    /// Maximum nesting depth.
    pub max_nesting_depth: usize,

    /// Whether the root is sampled.
    pub sampled: bool,

    /// Whether the root is parametric.
    pub parametric: bool,

    /// Whether the root is composite.
    pub composite: bool,

    /// Normalization semantics.
    pub normalization: WaveformNormalization,

    /// Domain semantics.
    pub domain: WaveformDomain,

    /// Optional sample rate.
    pub sample_rate: Option<WaveformSampleRate>,

    /// Metadata field count.
    pub metadata_fields: usize,
}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit resource/security policy for waveform validation.
///
/// This is NOT the maximum waveform representable by Zamani.
///
/// It answers:
///
/// > How much waveform structure is this validation invocation permitted to
/// > inspect?
///
/// A service, compiler invocation, sandbox, or trusted local build can choose
/// its own policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveformValidationPolicy {
    /// Maximum explicitly stored samples.
    pub max_samples: usize,

    /// Maximum composite children.
    pub max_composite_components: usize,

    /// Maximum piecewise segments.
    pub max_piecewise_segments: usize,

    /// Maximum semantic nodes.
    pub max_nodes: usize,

    /// Maximum nesting depth.
    pub max_nesting_depth: usize,

    /// Maximum metadata fields.
    pub max_metadata_fields: usize,

    /// Maximum metadata key length in bytes.
    pub max_metadata_key_bytes: usize,

    /// Maximum metadata value length in bytes.
    pub max_metadata_value_bytes: usize,
}

impl Default for WaveformValidationPolicy {
    fn default() -> Self {
        Self {
            max_samples: 4_194_304,
            max_composite_components: 4096,
            max_piecewise_segments: 4096,
            max_nodes: 1_000_000,
            max_nesting_depth: 1024,
            max_metadata_fields: 4096,
            max_metadata_key_bytes: 256,
            max_metadata_value_bytes: 65_536,
        }
    }
}

impl WaveformValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_samples: usize,
        max_composite_components: usize,
        max_piecewise_segments: usize,
        max_nodes: usize,
        max_nesting_depth: usize,
        max_metadata_fields: usize,
        max_metadata_key_bytes: usize,
        max_metadata_value_bytes: usize,
    ) -> Self {
        Self {
            max_samples,
            max_composite_components,
            max_piecewise_segments,
            max_nodes,
            max_nesting_depth,
            max_metadata_fields,
            max_metadata_key_bytes,
            max_metadata_value_bytes,
        }
    }

    /// Creates a policy whose limits are bounded only by the host's `usize`
    /// addressable resource space.
    ///
    /// This is still subject to:
    ///
    /// - available memory;
    /// - allocator behavior;
    /// - process limits;
    /// - operating-system limits;
    /// - compiler/runtime resource limits.
    ///
    /// It does NOT claim mathematical infinity.
    #[must_use]
    pub const fn unlimited_for_explicit_resources() -> Self {
        Self {
            max_samples: usize::MAX,
            max_composite_components: usize::MAX,
            max_piecewise_segments: usize::MAX,
            max_nodes: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_metadata_fields: usize::MAX,
            max_metadata_key_bytes: usize::MAX,
            max_metadata_value_bytes: usize::MAX,
        }
    }

    /// Validates the policy itself.
    pub const fn validate(self) -> WaveformResult<()> {
        if self.max_samples == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_samples cannot be zero",
            ));
        }

        if self.max_composite_components == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_composite_components cannot be zero",
            ));
        }

        if self.max_piecewise_segments == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_piecewise_segments cannot be zero",
            ));
        }

        if self.max_nodes == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_nodes cannot be zero",
            ));
        }

        if self.max_nesting_depth == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_nesting_depth cannot be zero",
            ));
        }

        if self.max_metadata_fields == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_metadata_fields cannot be zero",
            ));
        }

        if self.max_metadata_key_bytes == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_metadata_key_bytes cannot be zero",
            ));
        }

        if self.max_metadata_value_bytes == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_metadata_value_bytes cannot be zero",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Waveform-specific structural, semantic and resource errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveformError {
    /// A concrete floating-point value is NaN or infinite.
    NonFiniteValue {
        /// Semantic field name.
        field: &'static str,
    },

    /// The canonical parameter subsystem rejected a parameter.
    Parameter(String),

    /// A concrete parameter violates a waveform invariant.
    InvalidParameter(&'static str),

    /// Floating-point arithmetic produced a non-finite result.
    NumericalOverflow,

    /// Integer resource accounting overflowed.
    SizeOverflow,

    /// Sampled waveform has no samples.
    EmptySampledWaveform,

    /// Piecewise waveform has no segments.
    EmptyPiecewiseWaveform,

    /// Composite waveform has no components.
    EmptyCompositeWaveform,

    /// Segment bounds are structurally invalid.
    InvalidSegmentBounds,

    /// Sample rate is zero.
    ZeroSampleRate,

    /// Real sample format contains a non-zero Q component.
    SampleFormatMismatch,

    /// Validation policy itself is invalid.
    InvalidValidationPolicy(&'static str),

    /// Sample resource policy exceeded.
    SampleLimitExceeded {
        /// Observed count.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Composite component resource policy exceeded.
    CompositeComponentLimitExceeded {
        /// Observed count.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Piecewise segment resource policy exceeded.
    PiecewiseSegmentLimitExceeded {
        /// Observed count.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Node resource policy exceeded.
    NodeLimitExceeded {
        /// Observed count.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Nesting resource policy exceeded.
    NestingDepthExceeded {
        /// Observed depth.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Metadata field count exceeded.
    MetadataFieldLimitExceeded {
        /// Observed count.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata key exceeds policy.
    MetadataKeyTooLong {
        /// Actual byte length.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Metadata value exceeds policy.
    MetadataValueTooLong {
        /// Actual byte length.
        actual: usize,

        /// Policy maximum.
        limit: usize,
    },

    /// Extension namespace is empty.
    EmptyExtensionNamespace,

    /// Extension kind is empty.
    EmptyExtensionKind,

    /// Extension parameter key is empty.
    EmptyExtensionParameterKey,
}

impl fmt::Display for WaveformError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field } => {
                write!(formatter, "waveform {field} must be finite")
            }

            Self::Parameter(message) => {
                write!(formatter, "invalid waveform parameter: {message}")
            }

            Self::InvalidParameter(message) => {
                formatter.write_str(message)
            }

            Self::NumericalOverflow => {
                formatter.write_str(
                    "waveform numerical operation produced a non-finite result",
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "waveform resource accounting overflowed",
                )
            }

            Self::EmptySampledWaveform => {
                formatter.write_str(
                    "sampled waveform cannot be empty",
                )
            }

            Self::EmptyPiecewiseWaveform => {
                formatter.write_str(
                    "piecewise waveform cannot be empty",
                )
            }

            Self::EmptyCompositeWaveform => {
                formatter.write_str(
                    "composite waveform cannot be empty",
                )
            }

            Self::InvalidSegmentBounds => {
                formatter.write_str(
                    "piecewise segment has invalid concrete bounds",
                )
            }

            Self::ZeroSampleRate => {
                formatter.write_str(
                    "waveform sample rate cannot be zero",
                )
            }

            Self::SampleFormatMismatch => {
                formatter.write_str(
                    "real waveform samples must have zero quadrature",
                )
            }

            Self::InvalidValidationPolicy(message) => {
                formatter.write_str(message)
            }

            Self::SampleLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform sample count {actual} exceeds validation limit {limit}"
                )
            }

            Self::CompositeComponentLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform composite component count {actual} exceeds validation limit {limit}"
                )
            }

            Self::PiecewiseSegmentLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform piecewise segment count {actual} exceeds validation limit {limit}"
                )
            }

            Self::NodeLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform node count {actual} exceeds validation limit {limit}"
                )
            }

            Self::NestingDepthExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform nesting depth {actual} exceeds validation limit {limit}"
                )
            }

            Self::MetadataFieldLimitExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform metadata field count {actual} exceeds validation limit {limit}"
                )
            }

            Self::EmptyMetadataKey => {
                formatter.write_str(
                    "waveform metadata key cannot be empty",
                )
            }

            Self::MetadataKeyTooLong { actual, limit } => {
                write!(
                    formatter,
                    "waveform metadata key length {actual} exceeds validation limit {limit}"
                )
            }

            Self::MetadataValueTooLong { actual, limit } => {
                write!(
                    formatter,
                    "waveform metadata value length {actual} exceeds validation limit {limit}"
                )
            }

            Self::EmptyExtensionNamespace => {
                formatter.write_str(
                    "custom waveform extension namespace cannot be empty",
                )
            }

            Self::EmptyExtensionKind => {
                formatter.write_str(
                    "custom waveform extension kind cannot be empty",
                )
            }

            Self::EmptyExtensionParameterKey => {
                formatter.write_str(
                    "custom waveform extension parameter key cannot be empty",
                )
            }
        }
    }
}

impl std::error::Error for WaveformError {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn ensure_finite(
    value: f64,
    field: &'static str,
) -> WaveformResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(WaveformError::NonFiniteValue { field })
    }
}

fn validate_parameter(
    parameter: &Parameter,
) -> WaveformResult<()> {
    parameter
        .validate()
        .map_err(|error| WaveformError::Parameter(error.to_string()))
}

fn validate_segment_bounds(
    start: &Parameter,
    end: &Parameter,
) -> WaveformResult<()> {
    if let (Some(start), Some(end)) =
        (start.as_constant(), end.as_constant())
    {
        ensure_finite(start, "segment start")?;
        ensure_finite(end, "segment end")?;

        if start < 0.0 || end > 1.0 || start >= end {
            return Err(WaveformError::InvalidSegmentBounds);
        }
    }

    Ok(())
}

fn validate_metadata_key(
    key: &str,
) -> WaveformResult<()> {
    if key.trim().is_empty() {
        Err(WaveformError::EmptyMetadataKey)
    } else {
        Ok(())
    }
}

fn validate_metadata_key_with_policy(
    key: &str,
    policy: WaveformValidationPolicy,
) -> WaveformResult<()> {
    validate_metadata_key(key)?;

    let actual = key.len();

    if actual > policy.max_metadata_key_bytes {
        return Err(WaveformError::MetadataKeyTooLong {
            actual,
            limit: policy.max_metadata_key_bytes,
        });
    }

    Ok(())
}

fn validate_metadata_value_with_policy(
    value: &str,
    policy: WaveformValidationPolicy,
) -> WaveformResult<()> {
    let actual = value.len();

    if actual > policy.max_metadata_value_bytes {
        return Err(WaveformError::MetadataValueTooLong {
            actual,
            limit: policy.max_metadata_value_bytes,
        });
    }

    Ok(())
}

fn validate_extension_name(
    value: &str,
    empty_error: WaveformError,
) -> WaveformResult<()> {
    if value.trim().is_empty() {
        Err(empty_error)
    } else {
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::WaveformId;

    fn constant_parameter(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("test parameter must be finite")
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            waveform_schema_version(),
            (
                WAVEFORM_SCHEMA_MAJOR,
                WAVEFORM_SCHEMA_MINOR,
                WAVEFORM_SCHEMA_PATCH
            )
        );
    }

    #[test]
    fn zero_sample_rate_is_rejected() {
        assert!(matches!(
            WaveformSampleRate::new(0),
            Err(WaveformError::ZeroSampleRate)
        ));
    }

    #[test]
    fn very_large_sample_rate_is_supported() {
        let rate = WaveformSampleRate::new(u64::MAX)
            .expect("non-zero rate must be valid");

        assert_eq!(
            rate.samples_per_second(),
            u64::MAX
        );
    }

    #[test]
    fn nan_is_rejected() {
        assert!(matches!(
            ComplexSample::new(f64::NAN, 0.0),
            Err(WaveformError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(matches!(
            ComplexSample::new(f64::INFINITY, 0.0),
            Err(WaveformError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn finite_iq_is_accepted() {
        let sample = ComplexSample::new(0.25, -0.75)
            .expect("finite IQ must be valid");

        assert_eq!(sample.i(), 0.25);
        assert_eq!(sample.q(), -0.75);
        assert!(sample.is_finite());
    }

    #[test]
    fn real_sample_has_zero_q() {
        let sample = ComplexSample::real(0.5)
            .expect("finite real value must be valid");

        assert_eq!(sample.i(), 0.5);
        assert_eq!(sample.q(), 0.0);
    }

    #[test]
    fn magnitude_is_checked_without_checked_float_operations() {
        let sample = ComplexSample::new(3.0, 4.0)
            .expect("finite sample");

        assert_eq!(
            sample.magnitude()
                .expect("magnitude must be finite"),
            5.0
        );
    }

    #[test]
    fn real_sampled_waveform_is_valid() {
        let waveform = SampledWaveform::from_real(
            vec![0.0, 0.5, 1.0, 0.5, 0.0],
            None,
        )
        .expect("valid waveform");

        assert_eq!(waveform.len(), 5);
        assert_eq!(
            waveform.format(),
            WaveformSampleFormat::Real
        );
    }

    #[test]
    fn iq_sampled_waveform_is_valid() {
        let waveform = SampledWaveform::from_iq(
            vec![
                ComplexSample::new(1.0, 0.0)
                    .expect("finite"),
                ComplexSample::new(0.0, 1.0)
                    .expect("finite"),
            ],
            Some(
                WaveformSampleRate::new(1_000_000_000)
                    .expect("non-zero"),
            ),
        )
        .expect("valid IQ waveform");

        assert_eq!(waveform.len(), 2);
        assert_eq!(
            waveform.format(),
            WaveformSampleFormat::ComplexIq
        );
    }

    #[test]
    fn real_format_rejects_nonzero_q() {
        let result = SampledWaveform::new(
            vec![
                ComplexSample::new(1.0, 0.25)
                    .expect("finite")
            ],
            None,
            WaveformSampleFormat::Real,
        );

        assert!(matches!(
            result,
            Err(WaveformError::SampleFormatMismatch)
        ));
    }

    #[test]
    fn empty_sampled_waveform_is_rejected() {
        let result = SampledWaveform::new(
            Vec::new(),
            None,
            WaveformSampleFormat::Real,
        );

        assert!(matches!(
            result,
            Err(WaveformError::EmptySampledWaveform)
        ));
    }

    #[test]
    fn gaussian_rejects_zero_sigma() {
        let result = GaussianWaveform::new(
            constant_parameter(1.0),
            constant_parameter(0.0),
            constant_parameter(0.5),
        );

        assert!(matches!(
            result,
            Err(WaveformError::InvalidParameter(_))
        ));
    }

    #[test]
    fn symbolic_gaussian_remains_symbolic() {
        let sigma = Parameter::symbol("sigma")
            .expect("valid symbol");

        let waveform = GaussianWaveform::new(
            constant_parameter(1.0),
            sigma,
            constant_parameter(0.5),
        )
        .expect("valid Gaussian");

        assert!(waveform.sigma.is_symbolic());
    }

    #[test]
    fn drag_is_valid() {
        let waveform = DragWaveform::new(
            constant_parameter(1.0),
            constant_parameter(0.2),
            constant_parameter(0.5),
            constant_parameter(0.1),
        )
        .expect("valid DRAG");

        waveform.validate()
            .expect("DRAG must remain valid");
    }

    #[test]
    fn sine_supports_symbolic_cycles() {
        let cycles = Parameter::symbol("cycles")
            .expect("valid symbol");

        let waveform = SinusoidalWaveform::new(
            ComplexSample::real(1.0)
                .expect("finite"),
            cycles,
            constant_parameter(0.0),
        )
        .expect("valid sine");

        waveform.validate()
            .expect("sine must validate");
    }

    #[test]
    fn invalid_piecewise_bounds_are_rejected() {
        let result = PiecewiseSegment::new(
            constant_parameter(0.75),
            constant_parameter(0.25),
            WaveformKind::Square(
                SquareWaveform::real(1.0)
                    .expect("valid"),
            ),
        );

        assert!(matches!(
            result,
            Err(WaveformError::InvalidSegmentBounds)
        ));
    }

    #[test]
    fn valid_piecewise_waveform_is_accepted() {
        let segment = PiecewiseSegment::new(
            constant_parameter(0.0),
            constant_parameter(0.5),
            WaveformKind::Square(
                SquareWaveform::real(1.0)
                    .expect("valid"),
            ),
        )
        .expect("valid segment");

        let waveform = PiecewiseWaveform::new(
            vec![segment],
        )
        .expect("valid piecewise");

        waveform.validate()
            .expect("piecewise must validate");
    }

    #[test]
    fn composite_order_is_preserved() {
        let first = WaveformKind::Square(
            SquareWaveform::real(1.0)
                .expect("valid"),
        );

        let second = WaveformKind::Constant(
            ConstantWaveform::real(0.5)
                .expect("valid"),
        );

        let composite = CompositeWaveform::new(
            WaveformComposition::Add,
            vec![first, second],
        )
        .expect("valid composite");

        assert_eq!(composite.len(), 2);
        assert_eq!(
            composite.operation(),
            WaveformComposition::Add
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata = WaveformMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata insertion");

        metadata
            .insert("a", "first")
            .expect("metadata insertion");

        let keys = metadata
            .entries()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec![
                String::from("a"),
                String::from("z"),
            ]
        );
    }

    #[test]
    fn empty_metadata_key_is_rejected() {
        let mut metadata = WaveformMetadata::new();

        let result = metadata.insert("", "value");

        assert!(matches!(
            result,
            Err(WaveformError::EmptyMetadataKey)
        ));
    }

    #[test]
    fn custom_waveform_is_namespaced() {
        let custom = CustomWaveform::new(
            "zamani.example",
            "custom_envelope",
            BTreeMap::new(),
        )
        .expect("custom waveform");

        assert_eq!(
            custom.namespace(),
            "zamani.example"
        );

        assert_eq!(
            custom.kind(),
            "custom_envelope"
        );
    }

    #[test]
    fn waveform_does_not_require_qubit_identity() {
        let waveform = Waveform::new(
            WaveformId::new(1),
            WaveformKind::Square(
                SquareWaveform::real(0.3)
                    .expect("valid"),
            ),
        )
        .expect("valid waveform");

        assert_eq!(
            waveform.id(),
            WaveformId::new(1)
        );
    }

    #[test]
    fn waveform_supports_large_identity_values() {
        let waveform = Waveform::new(
            WaveformId::new(u64::MAX),
            WaveformKind::Square(
                SquareWaveform::real(0.3)
                    .expect("valid"),
            ),
        )
        .expect("large identity must be valid");

        assert_eq!(
            waveform.id().value(),
            u64::MAX
        );
    }

    #[test]
    fn large_finite_amplitude_is_not_clipped() {
        let waveform = SquareWaveform::real(1_000_000.0)
            .expect("finite amplitude");

        waveform.validate()
            .expect("target-independent IR must not clip");
    }

    #[test]
    fn normalization_is_declarative() {
        let waveform = Waveform::new(
            WaveformId::new(1),
            WaveformKind::Square(
                SquareWaveform::real(0.5)
                    .expect("valid"),
            ),
        )
        .expect("valid")
        .with_normalization(
            WaveformNormalization::UnitPeak,
        );

        assert_eq!(
            waveform.normalization(),
            WaveformNormalization::UnitPeak
        );

        match waveform.kind() {
            WaveformKind::Square(square) => {
                assert_eq!(square.amplitude.i(), 0.5);
            }

            _ => panic!("expected square"),
        }
    }

    #[test]
    fn resource_usage_counts_nested_samples() {
        let inner = WaveformKind::Sampled(
            SampledWaveform::from_real(
                vec![0.0, 1.0, 0.0],
                None,
            )
            .expect("valid"),
        );

        let composite = WaveformKind::Composite(
            CompositeWaveform::new(
                WaveformComposition::Concatenate,
                vec![inner],
            )
            .expect("valid"),
        );

        let waveform = Waveform::new(
            WaveformId::new(10),
            composite,
        )
        .expect("valid");

        let usage = waveform
            .resource_usage()
            .expect("resource accounting");

        assert_eq!(usage.sample_count, 3);
        assert_eq!(usage.composite_component_count, 1);
        assert_eq!(usage.node_count, 2);
    }

    #[test]
    fn validation_policy_can_be_explicitly_large() {
        let policy =
            WaveformValidationPolicy::unlimited_for_explicit_resources();

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn validation_policy_rejects_zero_limits() {
        let policy = WaveformValidationPolicy::new(
            0,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
        );

        assert!(matches!(
            policy.validate(),
            Err(WaveformError::InvalidValidationPolicy(_))
        ));
    }

    #[test]
    fn sampled_energy_is_correct() {
        let waveform =
            SampledWaveform::from_real(
                vec![1.0, 2.0, 3.0],
                None,
            )
            .expect("valid");

        assert_eq!(
            waveform.energy()
                .expect("finite energy"),
            14.0
        );
    }

    #[test]
    fn sampled_peak_is_correct() {
        let waveform =
            SampledWaveform::from_real(
                vec![
                    -0.5,
                    0.25,
                    0.75,
                    0.1,
                ],
                None,
            )
            .expect("valid");

        assert_eq!(
            waveform.peak_magnitude()
                .expect("finite peak"),
            0.75
        );
    }

    #[test]
    fn waveform_summary_is_deterministic() {
        let waveform = Waveform::new(
            WaveformId::new(42),
            WaveformKind::Gaussian(
                GaussianWaveform::new(
                    constant_parameter(0.3),
                    constant_parameter(0.1),
                    constant_parameter(0.5),
                )
                .expect("valid"),
            ),
        )
        .expect("valid");

        let summary = waveform.summary();

        assert_eq!(
            summary.id,
            WaveformId::new(42)
        );

        assert_eq!(
            summary.kind,
            "gaussian"
        );

        assert!(summary.parametric);
        assert!(!summary.sampled);
    }
}