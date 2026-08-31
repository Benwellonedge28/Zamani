//! Zamani Quantum IR — Hardware-Independent Waveform Semantics
//!
//! Canonical representation of waveform definitions used by the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `waveform.rs` defines WHAT a waveform means.
//!
//! It owns:
//!
//! - waveform identity references;
//! - waveform mathematical/semantic shape;
//! - scalar and complex IQ samples;
//! - parametric waveform definitions;
//! - sampled waveform definitions;
//! - piecewise waveform definitions;
//! - composite waveform definitions;
//! - waveform normalization semantics;
//! - sample-rate semantics;
//! - deterministic waveform metadata;
//! - resource-safe construction;
//! - waveform-local validation;
//! - checked numerical operations;
//! - canonical accessors;
//! - structural equality;
//! - deterministic ordering where applicable.
//!
//! It does NOT own:
//!
//! - physical DACs;
//! - ADCs;
//! - hardware sample clocks;
//! - physical control channels;
//! - hardware topology;
//! - qubit routing;
//! - pulse scheduling;
//! - calibration execution;
//! - provider SDKs;
//! - provider authentication;
//! - QPU execution;
//! - simulator state;
//! - optimization policy;
//! - frontend parsing.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Architectural position
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
//!      +---------------------+
//!      |                     |
//!      v                     v
//!    pulse               waveform
//!      |                     |
//!      +----------+----------+
//!                 |
//!                 v
//!          optimization
//!                 |
//!                 v
//!           scheduling
//!                 |
//!                 v
//!             hardware
//!                 |
//!                 v
//!              backend
//!                 |
//!                 v
//!                QPU
//! ```
//!
//! The distinction is deliberate:
//!
//! ```text
//! Pulse    = an event/control request.
//! Waveform = the signal shape associated with that request.
//! Channel  = where the signal is sent.
//! Frame    = the signal's phase/frequency reference.
//! Schedule = when the event occurs.
//! Hardware = how those abstractions map to physical equipment.
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once and can be compiled toward:
//!
//! - tiny quantum processors;
//! - large quantum processors;
//! - superconducting systems;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin systems;
//! - analog quantum systems;
//! - annealing systems;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - future quantum architectures.
//!
//! `waveform.rs` therefore contains NO architectural quantum-machine-size
//! ceiling.
//!
//! A finite resource limit used for validation is a policy limit, not a limit
//! on the size of quantum computers supported by Zamani.
//!
//! # Important scalability rule
//!
//! This module must never interpret:
//!
//! ```text
//! 63
//! 4096
//! 1_000_000
//! ```
//!
//! as the maximum number of qubits, waveform samples, waveform definitions,
//! or quantum machines supported by Zamani.
//!
//! Resource limits are explicit validation/compiler policy.
//!
//! # Pulse integration
//!
//! A source-level program such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! may eventually lower to a pulse whose waveform is a reusable semantic
//! waveform definition.
//!
//! For example:
//!
//! ```text
//! Pulse
//! ├── target = q
//! ├── amplitude = 0.3
//! ├── duration = 20ns
//! └── waveform = WaveformId(...)
//! ```
//!
//! The waveform itself does not contain `q`.
//!
//! This allows one waveform definition to be reused across many pulse
//! operations and many logical qubits.
//!
//! # Hardware boundary
//!
//! This module deliberately differs from:
//!
//! `quantum::hardware::pulse`
//!
//! The hardware pulse subsystem is allowed to know about:
//!
//! - actual sample rates;
//! - hardware clocks;
//! - physical waveform samples;
//! - physical channel resources;
//! - hardware-specific amplitude ranges;
//! - provider/device constraints;
//! - DAC/ADC interfaces;
//! - concrete pulse instructions.
//!
//! This module only expresses the hardware-independent semantic waveform.
//!
//! # Numerical policy
//!
//! All concrete floating-point values entering the canonical waveform model
//! must be finite.
//!
//! NaN and positive/negative infinity are rejected.
//!
//! The IR does not universally require:
//!
//! ```text
//! |amplitude| <= 1
//! ```
//!
//! because amplitude normalization is target dependent.
//!
//! Hardware compatibility validation may impose a target-specific amplitude
//! range later.
//!
//! # Complex IQ semantics
//!
//! A complex waveform sample is represented as:
//!
//! ```text
//! I + iQ
//! ```
//!
//! Both components must be finite.
//!
//! No provider-specific IQ convention is assumed.
//!
//! # Parametric waveform semantics
//!
//! Parametric waveforms describe a signal mathematically without forcing the IR
//! to materialize every sample immediately.
//!
//! This is important for scalability.
//!
//! For example, a Gaussian waveform can remain:
//!
//! ```text
//! Gaussian(amplitude, sigma, center)
//! ```
//!
//! instead of becoming millions of stored samples.
//!
//! A later compiler/backend may materialize samples according to target
//! requirements.
//!
//! # Sampled waveform semantics
//!
//! Sampled waveforms contain explicitly ordered samples.
//!
//! The IR stores samples exactly as supplied.
//!
//! It does not silently resample, interpolate, clip, normalize, quantize, or
//! truncate them.
//!
//! Such transformations belong to explicit compiler/optimization/hardware
//! passes.
//!
//! # Determinism
//!
//! Metadata uses `BTreeMap` rather than `HashMap` so iteration order is
//! deterministic.
//!
//! Composite waveform ordering is explicit and stable.
//!
//! Sample ordering is preserved exactly.
//!
//! No global mutable waveform registry exists.
//!
//! # Security
//!
//! Metadata is descriptive only.
//!
//! This module never stores:
//!
//! - API keys;
//! - passwords;
//! - access tokens;
//! - provider credentials;
//! - authentication headers;
//! - private keys;
//! - cookies.
//!
//! Metadata is bounded through explicit validation policy.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `WaveformId`.
//!
//! `parameter.rs`
//!     Supplies symbolic and deterministic scalar parameters.
//!
//! `pulse.rs`
//!     References waveform definitions using `WaveformId`.
//!
//! `channel.rs`
//!     May associate waveform-producing pulses with abstract channels. This
//!     module does not depend on channel definitions.
//!
//! `frame.rs`
//!     May provide the phase/frequency frame in which a waveform is interpreted.
//!
//! `timing.rs`
//!     May associate pulse duration with waveform evaluation time. The waveform
//!     itself remains independent of the program-wide timing implementation.
//!
//! `schedule.rs`
//!     Determines when waveform-bearing pulses execute.
//!
//! `hardware/pulse.rs`
//!     Materializes semantic waveforms into hardware-compatible samples and
//!     control instructions.
//!
//! `optimization/`
//!     May transform waveforms while preserving declared semantics.
//!
//! `validation.rs`
//!     Performs whole-program validation and target-independent validation.
//!
//! `serialization.rs`
//!     May serialize the structural waveform representation.
//!
//! `hash.rs`
//!     May derive deterministic content identity from the structural fields.
//!
//! `provenance.rs`
//!     May record waveform transformation history.
//!
//! # File completion guarantee
//!
//! This file intentionally contains:
//!
//! - complete waveform semantics;
//! - waveform shape taxonomy;
//! - scalar samples;
//! - complex IQ samples;
//! - parametric shapes;
//! - sampled shapes;
//! - piecewise shapes;
//! - composite shapes;
//! - custom extension references;
//! - sample-rate semantics;
//! - normalization semantics;
//! - deterministic metadata;
//! - validation policy;
//! - checked arithmetic;
//! - finite-number checking;
//! - deterministic constructors;
//! - structural accessors;
//! - tests;
//! - integration documentation.
//!
//! Later implementation of `channel.rs`, `frame.rs`, `timing.rs`, `schedule.rs`
//! or `operation.rs` should not require changing the semantic meaning of this
//! file merely because those modules are added.
//!
//! -----------------------------------------------------------------------------
//! Safety
//! -----------------------------------------------------------------------------
//
// No unsafe Rust is permitted in this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::identity::WaveformId;
use super::parameter::Parameter;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for semantic waveform definitions.
pub const WAVEFORM_SCHEMA_ID: &str = "zamani.quantum.ir.waveform";

/// Stable semantic schema version.
///
/// Breaking semantic changes require a new major IR schema version.
pub const WAVEFORM_SCHEMA_VERSION: u16 = 1;

/// Default metadata key size in UTF-8 bytes.
///
/// This is a resource-safety policy, not an architectural limit.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default metadata value size in UTF-8 bytes.
///
/// This is a resource-safety policy, not an architectural limit.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Default number of metadata fields.
///
/// This is a resource-safety policy, not an architectural limit.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum number of samples accepted by the default validation
/// policy.
///
/// This is NOT a maximum waveform size supported by the IR.
///
/// Larger waveforms may be accepted through an explicit validation policy.
pub const DEFAULT_MAX_SAMPLES: usize = 4_194_304;

/// Default maximum number of components in a composite waveform.
///
/// This is a validation/resource policy only.
pub const DEFAULT_MAX_COMPOSITE_COMPONENTS: usize = 4096;

/// Default maximum piecewise segments.
///
/// This is a validation/resource policy only.
pub const DEFAULT_MAX_PIECEWISE_SEGMENTS: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Result type used by waveform construction and local validation.
pub type WaveformResult<T> = Result<T, WaveformError>;

// =============================================================================
// Sample-rate semantics
// =============================================================================

/// Semantic sample rate in samples per second.
///
/// This is a waveform-definition property, not a physical device clock.
///
/// A hardware target may later quantize or translate it to its actual sample
/// clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WaveformSampleRate(u64);

impl WaveformSampleRate {
    /// Creates a sample rate from whole samples per second.
    ///
    /// Zero is rejected because a finite sampled waveform cannot have a
    /// meaningful zero sampling rate.
    pub const fn new(samples_per_second: u64) -> WaveformResult<Self> {
        if samples_per_second == 0 {
            return Err(WaveformError::ZeroSampleRate);
        }

        Ok(Self(samples_per_second))
    }

    /// Returns the sample rate in samples per second.
    #[must_use]
    pub const fn samples_per_second(self) -> u64 {
        self.0
    }

    /// Returns the sample rate in hertz.
    #[must_use]
    pub const fn hertz(self) -> u64 {
        self.0
    }
}

impl fmt::Display for WaveformSampleRate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}Sa/s", self.0)
    }
}

// =============================================================================
// Complex sample
// =============================================================================

/// A finite complex waveform sample.
///
/// The two components represent the abstract in-phase (`I`) and quadrature
/// (`Q`) components.
///
/// No provider-specific IQ convention is imposed here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexSample {
    i: f64,
    q: f64,
}

impl ComplexSample {
    /// Creates a finite complex sample.
    pub fn new(i: f64, q: f64) -> WaveformResult<Self> {
        ensure_finite(i, "I component")?;
        ensure_finite(q, "Q component")?;

        Ok(Self { i, q })
    }

    /// Creates a real-valued sample with zero quadrature component.
    pub fn real(value: f64) -> WaveformResult<Self> {
        Self::new(value, 0.0)
    }

    /// Creates a zero sample.
    #[must_use]
    pub const fn zero() -> Self {
        Self { i: 0.0, q: 0.0 }
    }

    /// Returns the in-phase component.
    #[must_use]
    pub const fn i(self) -> f64 {
        self.i
    }

    /// Returns the quadrature component.
    #[must_use]
    pub const fn q(self) -> f64 {
        self.q
    }

    /// Returns whether both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.i.is_finite() && self.q.is_finite()
    }

    /// Returns the squared magnitude.
    ///
    /// This operation does not panic and returns `None` if the floating-point
    /// multiplication/addition becomes non-finite.
    #[must_use]
    pub fn magnitude_squared(self) -> Option<f64> {
        let ii = self.i.checked_mul(self.i)?;
        let qq = self.q.checked_mul(self.q)?;
        let value = ii + qq;

        if value.is_finite() {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the magnitude.
    ///
    /// Returns `None` if the result is non-finite.
    #[must_use]
    pub fn magnitude(self) -> Option<f64> {
        let value = self.magnitude_squared()?.sqrt();

        if value.is_finite() {
            Some(value)
        } else {
            None
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

/// Semantic waveform sample representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformSampleFormat {
    /// One real-valued sample per point.
    Real,

    /// Complex IQ sample per point.
    ComplexIq,
}

impl Default for WaveformSampleFormat {
    fn default() -> Self {
        Self::Real
    }
}

// =============================================================================
// Normalization
// =============================================================================

/// Declares how a waveform's numerical amplitude should be interpreted.
///
/// Normalization is semantic metadata. It does not perform implicit scaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformNormalization {
    /// Values are interpreted exactly as represented.
    None,

    /// The waveform is declared to have unit peak magnitude.
    ///
    /// The constructor does not automatically rescale the waveform.
    UnitPeak,

    /// The waveform is declared to have unit energy.
    ///
    /// The constructor does not automatically rescale the waveform.
    UnitEnergy,

    /// The waveform has been explicitly normalized by the source/compiler.
    ///
    /// No implicit normalization is performed by this module.
    Explicit,
}

impl Default for WaveformNormalization {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// Domain semantics
// =============================================================================

/// Domain over which a parametric waveform is defined.
///
/// The canonical domain is dimensionless `[0, 1]`. A consuming pulse duration
/// maps physical time onto this domain.
///
/// This keeps waveform definitions reusable for pulses of different duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformDomain {
    /// Normalized domain `[0, 1]`.
    Normalized,

    /// Explicit normalized domain `[start, end]`.
    ///
    /// This is useful for reusable mathematical waveform fragments.
    Explicit,
}

impl Default for WaveformDomain {
    fn default() -> Self {
        Self::Normalized
    }
}

// =============================================================================
// Gaussian parameters
// =============================================================================

/// Gaussian waveform definition.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianWaveform {
    /// Peak amplitude.
    pub amplitude: Parameter,

    /// Standard deviation in normalized-domain units.
    pub sigma: Parameter,

    /// Center position in normalized-domain units.
    pub center: Parameter,
}

impl GaussianWaveform {
    /// Creates a Gaussian waveform definition.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
    ) -> WaveformResult<Self> {
        amplitude
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        sigma
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        center
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

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

    /// Validates the Gaussian definition.
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
// DRAG parameters
// =============================================================================

/// Derivative Removal by Adiabatic Gate (DRAG) waveform definition.
///
/// The exact hardware interpretation remains target-specific. The IR stores
/// the semantic parameters only.
#[derive(Debug, Clone, PartialEq)]
pub struct DragWaveform {
    /// Base Gaussian amplitude.
    pub amplitude: Parameter,

    /// Gaussian standard deviation.
    pub sigma: Parameter,

    /// Center in normalized-domain units.
    pub center: Parameter,

    /// Derivative correction coefficient.
    pub beta: Parameter,
}

impl DragWaveform {
    /// Creates a DRAG waveform definition.
    pub fn new(
        amplitude: Parameter,
        sigma: Parameter,
        center: Parameter,
        beta: Parameter,
    ) -> WaveformResult<Self> {
        amplitude
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        sigma
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        center
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        beta
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

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

    /// Validates the DRAG definition.
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
// Square / constant parameters
// =============================================================================

/// Constant waveform definition.
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

    /// Validates the waveform.
    pub fn validate(&self) -> WaveformResult<()> {
        self.value.validate()
    }
}

/// Square waveform definition.
///
/// A square waveform is constant over the normalized waveform domain.
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

    /// Validates the waveform.
    pub fn validate(&self) -> WaveformResult<()> {
        self.amplitude.validate()
    }
}

// =============================================================================
// Sinusoidal parameters
// =============================================================================

/// Sinusoidal waveform definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SinusoidalWaveform {
    /// Peak complex amplitude.
    pub amplitude: ComplexSample,

    /// Frequency multiplier over the normalized domain.
    ///
    /// For example, `1` means one complete cycle over the domain.
    pub cycles: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,
}

impl SinusoidalWaveform {
    /// Creates a sinusoidal waveform definition.
    pub fn new(
        amplitude: ComplexSample,
        cycles: Parameter,
        phase: Parameter,
    ) -> WaveformResult<Self> {
        amplitude.validate()?;

        cycles
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        phase
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        Ok(Self {
            amplitude,
            cycles,
            phase,
        })
    }

    /// Validates the waveform.
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
// Cosine parameters
// =============================================================================

/// Cosine waveform definition.
#[derive(Debug, Clone, PartialEq)]
pub struct CosineWaveform {
    /// Peak complex amplitude.
    pub amplitude: ComplexSample,

    /// Frequency multiplier over the normalized domain.
    pub cycles: Parameter,

    /// Phase offset in radians.
    pub phase: Parameter,
}

impl CosineWaveform {
    /// Creates a cosine waveform definition.
    pub fn new(
        amplitude: ComplexSample,
        cycles: Parameter,
        phase: Parameter,
    ) -> WaveformResult<Self> {
        amplitude.validate()?;

        cycles
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        phase
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        Ok(Self {
            amplitude,
            cycles,
            phase,
        })
    }

    /// Validates the waveform.
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
// Sampled waveform
// =============================================================================

/// Explicitly sampled waveform.
///
/// Samples are stored in exact source order. No implicit interpolation,
/// clipping, normalization, quantization or resampling is performed.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledWaveform {
    /// Samples in temporal order.
    samples: Vec<ComplexSample>,

    /// Optional semantic sample rate.
    ///
    /// A missing sample rate means that the consuming pulse/hardware layer
    /// supplies the temporal interpretation.
    sample_rate: Option<WaveformSampleRate>,

    /// Sample representation.
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

    /// Creates a sampled real waveform.
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

    /// Creates a sampled complex IQ waveform.
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

    /// Returns the samples.
    #[must_use]
    pub fn samples(&self) -> &[ComplexSample] {
        &self.samples
    }

    /// Returns the optional semantic sample rate.
    #[must_use]
    pub const fn sample_rate(&self) -> Option<WaveformSampleRate> {
        self.sample_rate
    }

    /// Returns the sample format.
    #[must_use]
    pub const fn format(&self) -> WaveformSampleFormat {
        self.format
    }

    /// Returns a sample by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<ComplexSample> {
        self.samples.get(index).copied()
    }

    /// Validates the sampled waveform.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.samples.is_empty() {
            return Err(WaveformError::EmptySampledWaveform);
        }

        if let Some(sample_rate) = self.sample_rate {
            if sample_rate.samples_per_second() == 0 {
                return Err(WaveformError::ZeroSampleRate);
            }
        }

        for sample in &self.samples {
            sample.validate()?;
        }

        match self.format {
            WaveformSampleFormat::Real => {
                for sample in &self.samples {
                    if sample.q() != 0.0 {
                        return Err(
                            WaveformError::SampleFormatMismatch,
                        );
                    }
                }
            }

            WaveformSampleFormat::ComplexIq => {}
        }

        Ok(())
    }

    /// Computes the sum of squared magnitudes using checked finite arithmetic.
    pub fn energy(&self) -> WaveformResult<f64> {
        self.validate()?;

        let mut total = 0.0_f64;

        for sample in &self.samples {
            let magnitude_squared = sample
                .magnitude_squared()
                .ok_or(WaveformError::NumericalOverflow)?;

            total = total
                .checked_add(magnitude_squared)
                .ok_or(WaveformError::NumericalOverflow)?;

            if !total.is_finite() {
                return Err(WaveformError::NumericalOverflow);
            }
        }

        Ok(total)
    }

    /// Computes the maximum sample magnitude.
    pub fn peak_magnitude(&self) -> WaveformResult<f64> {
        self.validate()?;

        let mut peak = 0.0_f64;

        for sample in &self.samples {
            let magnitude = sample
                .magnitude()
                .ok_or(WaveformError::NumericalOverflow)?;

            if magnitude > peak {
                peak = magnitude;
            }
        }

        Ok(peak)
    }
}

// =============================================================================
// Piecewise waveform
// =============================================================================

/// A piecewise waveform segment.
///
/// Segment boundaries are expressed in the normalized domain.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseSegment {
    /// Start position in the normalized domain.
    pub start: Parameter,

    /// End position in the normalized domain.
    pub end: Parameter,

    /// Waveform applied to this interval.
    pub waveform: Box<WaveformKind>,
}

impl PiecewiseSegment {
    /// Creates a piecewise segment.
    pub fn new(
        start: Parameter,
        end: Parameter,
        waveform: WaveformKind,
    ) -> WaveformResult<Self> {
        start
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        end
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        waveform.validate()?;

        if let (Some(start_value), Some(end_value)) =
            (start.as_constant(), end.as_constant())
        {
            if start_value < 0.0
                || end_value > 1.0
                || start_value >= end_value
            {
                return Err(WaveformError::InvalidSegmentBounds);
            }
        }

        Ok(Self {
            start,
            end,
            waveform: Box::new(waveform),
        })
    }

    /// Validates the segment.
    pub fn validate(&self) -> WaveformResult<()> {
        self.start
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        self.end
            .validate()
            .map_err(|error| WaveformError::Parameter(error.to_string()))?;

        self.waveform.validate()?;

        if let (Some(start), Some(end)) =
            (self.start.as_constant(), self.end.as_constant())
        {
            if start < 0.0 || end > 1.0 || start >= end {
                return Err(WaveformError::InvalidSegmentBounds);
            }
        }

        Ok(())
    }
}

/// Piecewise waveform definition.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseWaveform {
    /// Ordered segments.
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

    /// Returns the ordered segments.
    #[must_use]
    pub fn segments(&self) -> &[PiecewiseSegment] {
        &self.segments
    }

    /// Returns the number of segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether the waveform contains no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Validates the piecewise waveform.
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
// Composite waveform
// =============================================================================

/// Composition operator for waveform definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaveformComposition {
    /// Add waveforms pointwise.
    Add,

    /// Multiply waveforms pointwise.
    Multiply,

    /// Concatenate waveforms temporally.
    Concatenate,

    /// Apply one waveform as an envelope to another.
    Modulate,
}

impl Default for WaveformComposition {
    fn default() -> Self {
        Self::Concatenate
    }
}

/// Composite waveform definition.
///
/// Composite waveforms allow higher-level constructions without requiring the
/// compiler to materialize samples prematurely.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeWaveform {
    /// Composition operation.
    pub operation: WaveformComposition,

    /// Ordered child waveforms.
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

    /// Returns the composition operator.
    #[must_use]
    pub const fn operation(&self) -> WaveformComposition {
        self.operation
    }

    /// Returns the child waveforms.
    #[must_use]
    pub fn components(&self) -> &[Box<WaveformKind>] {
        &self.components
    }

    /// Returns the number of child waveforms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether there are no components.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Validates the composite waveform.
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
// Custom waveform reference
// =============================================================================

/// Namespaced custom waveform definition.
///
/// This allows future waveform technologies without changing the fundamental
/// waveform representation.
///
/// A custom waveform is a semantic extension reference, not executable code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomWaveform {
    /// Extension namespace.
    namespace: String,

    /// Extension-defined waveform kind.
    kind: String,

    /// Deterministic textual parameters.
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

        if namespace.trim().is_empty() {
            return Err(WaveformError::EmptyExtensionNamespace);
        }

        if kind.trim().is_empty() {
            return Err(WaveformError::EmptyExtensionKind);
        }

        Ok(Self {
            namespace,
            kind,
            parameters,
        })
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the extension-defined waveform kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns deterministic custom parameters.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    /// Validates the custom waveform reference.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.namespace.trim().is_empty() {
            return Err(WaveformError::EmptyExtensionNamespace);
        }

        if self.kind.trim().is_empty() {
            return Err(WaveformError::EmptyExtensionKind);
        }

        for (key, value) in &self.parameters {
            if key.trim().is_empty() {
                return Err(WaveformError::EmptyMetadataKey);
            }

            if key.len() > DEFAULT_MAX_METADATA_KEY_BYTES {
                return Err(WaveformError::MetadataKeyTooLong);
            }

            if value.len() > DEFAULT_MAX_METADATA_VALUE_BYTES {
                return Err(WaveformError::MetadataValueTooLong);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Waveform kind
// =============================================================================

/// Complete semantic waveform taxonomy.
///
/// This enum deliberately represents signal semantics rather than hardware
/// instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum WaveformKind {
    /// Constant-valued waveform.
    Constant(ConstantWaveform),

    /// Square waveform.
    Square(SquareWaveform),

    /// Gaussian envelope.
    Gaussian(GaussianWaveform),

    /// DRAG envelope.
    Drag(DragWaveform),

    /// Sine waveform.
    Sine(SinusoidalWaveform),

    /// Cosine waveform.
    Cosine(CosineWaveform),

    /// Explicitly sampled waveform.
    Sampled(SampledWaveform),

    /// Piecewise waveform.
    Piecewise(PiecewiseWaveform),

    /// Composite waveform.
    Composite(CompositeWaveform),

    /// Extension-defined waveform.
    Custom(CustomWaveform),
}

impl WaveformKind {
    /// Validates the waveform kind.
    pub fn validate(&self) -> WaveformResult<()> {
        match self {
            Self::Constant(waveform) => waveform.validate(),
            Self::Square(waveform) => waveform.validate(),
            Self::Gaussian(waveform) => waveform.validate(),
            Self::Drag(waveform) => waveform.validate(),
            Self::Sine(waveform) => waveform.validate(),
            Self::Cosine(waveform) => waveform.validate(),
            Self::Sampled(waveform) => waveform.validate(),
            Self::Piecewise(waveform) => waveform.validate(),
            Self::Composite(waveform) => waveform.validate(),
            Self::Custom(waveform) => waveform.validate(),
        }
    }

    /// Returns the stable semantic kind name.
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

    /// Returns the number of directly stored samples.
    ///
    /// Parametric waveforms return zero because they are not materialized.
    ///
    /// Composite and piecewise waveforms recursively count materialized
    /// samples.
    pub fn sample_count(&self) -> usize {
        match self {
            Self::Sampled(waveform) => waveform.len(),

            Self::Piecewise(waveform) => waveform
                .segments()
                .iter()
                .map(|segment| segment.waveform.sample_count())
                .fold(0usize, |total, count| {
                    total.saturating_add(count)
                }),

            Self::Composite(waveform) => waveform
                .components()
                .iter()
                .map(|component| component.sample_count())
                .fold(0usize, |total, count| {
                    total.saturating_add(count)
                }),

            Self::Constant(_)
            | Self::Square(_)
            | Self::Gaussian(_)
            | Self::Drag(_)
            | Self::Sine(_)
            | Self::Cosine(_)
            | Self::Custom(_) => 0,
        }
    }

    /// Returns whether the waveform is explicitly sampled.
    #[must_use]
    pub const fn is_sampled(&self) -> bool {
        matches!(self, Self::Sampled(_))
    }

    /// Returns whether the waveform is parametric.
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

    /// Returns whether the waveform is a composite definition.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        matches!(
            self,
            Self::Piecewise(_) | Self::Composite(_)
        )
    }
}

// =============================================================================
// Waveform metadata
// =============================================================================

/// Deterministic descriptive metadata attached to a waveform.
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

    /// Inserts metadata after basic key/value validation.
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
        validate_metadata_value(&value)?;

        if self.entries.len() >= DEFAULT_MAX_METADATA_FIELDS
            && !self.entries.contains_key(&key)
        {
            return Err(WaveformError::MetadataFieldLimitExceeded);
        }

        Ok(self.entries.insert(key, value))
    }

    /// Gets a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns all metadata in deterministic order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validates all metadata.
    pub fn validate(&self) -> WaveformResult<()> {
        if self.entries.len() > DEFAULT_MAX_METADATA_FIELDS {
            return Err(WaveformError::MetadataFieldLimitExceeded);
        }

        for (key, value) in &self.entries {
            validate_metadata_key(key)?;
            validate_metadata_value(value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Waveform
// =============================================================================

/// Complete canonical waveform definition.
///
/// A `Waveform` is a reusable semantic object. It does not identify a qubit,
/// physical channel, hardware clock, or execution time.
#[derive(Debug, Clone, PartialEq)]
pub struct Waveform {
    /// Stable IR identity.
    id: WaveformId,

    /// Semantic waveform definition.
    kind: WaveformKind,

    /// Declared normalization semantics.
    normalization: WaveformNormalization,

    /// Mathematical domain semantics.
    domain: WaveformDomain,

    /// Optional semantic sample rate.
    ///
    /// This is useful for explicitly sampled waveforms but remains optional
    /// because parametric waveforms can defer sampling to a target compiler.
    sample_rate: Option<WaveformSampleRate>,

    /// Descriptive deterministic metadata.
    metadata: WaveformMetadata,
}

impl Waveform {
    /// Creates a waveform from an explicit identity and semantic definition.
    pub fn new(
        id: WaveformId,
        kind: WaveformKind,
    ) -> WaveformResult<Self> {
        let waveform = Self {
            id,
            kind,
            normalization: WaveformNormalization::None,
            domain: WaveformDomain::Normalized,
            sample_rate: None,
            metadata: WaveformMetadata::default(),
        };

        waveform.validate()?;

        Ok(waveform)
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

    /// Returns the stable waveform identity.
    #[must_use]
    pub const fn id(&self) -> WaveformId {
        self.id
    }

    /// Returns the semantic waveform definition.
    #[must_use]
    pub fn kind(&self) -> &WaveformKind {
        &self.kind
    }

    /// Returns the waveform normalization semantics.
    #[must_use]
    pub const fn normalization(&self) -> WaveformNormalization {
        self.normalization
    }

    /// Returns the waveform domain semantics.
    #[must_use]
    pub const fn domain(&self) -> WaveformDomain {
        self.domain
    }

    /// Returns the optional semantic sample rate.
    #[must_use]
    pub const fn sample_rate(&self) -> Option<WaveformSampleRate> {
        self.sample_rate
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &WaveformMetadata {
        &self.metadata
    }

    /// Returns the semantic kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        self.kind.kind_name()
    }

    /// Returns whether the waveform is sampled.
    #[must_use]
    pub const fn is_sampled(&self) -> bool {
        self.kind.is_sampled()
    }

    /// Returns whether the waveform is parametric.
    #[must_use]
    pub const fn is_parametric(&self) -> bool {
        self.kind.is_parametric()
    }

    /// Returns whether the waveform is composite.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        self.kind.is_composite()
    }

    /// Returns the number of explicitly stored samples.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.kind.sample_count()
    }

    /// Replaces normalization semantics without modifying waveform samples.
    pub fn with_normalization(
        mut self,
        normalization: WaveformNormalization,
    ) -> Self {
        self.normalization = normalization;
        self
    }

    /// Replaces domain semantics without modifying the waveform.
    pub fn with_domain(
        mut self,
        domain: WaveformDomain,
    ) -> Self {
        self.domain = domain;
        self
    }

    /// Sets an explicit semantic sample rate.
    pub fn with_sample_rate(
        mut self,
        sample_rate: WaveformSampleRate,
    ) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Adds metadata after validating it.
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

    /// Validates the waveform using the default local policy.
    pub fn validate(&self) -> WaveformResult<()> {
        self.validate_with_policy(WaveformValidationPolicy::default())
    }

    /// Validates the waveform under an explicit resource policy.
    pub fn validate_with_policy(
        &self,
        policy: WaveformValidationPolicy,
    ) -> WaveformResult<()> {
        policy.validate()?;

        self.kind.validate()?;
        self.metadata.validate()?;

        if let Some(sample_rate) = self.sample_rate {
            if sample_rate.samples_per_second() == 0 {
                return Err(WaveformError::ZeroSampleRate);
            }
        }

        let sample_count = self.sample_count();

        if sample_count > policy.max_samples {
            return Err(WaveformError::SampleLimitExceeded {
                actual: sample_count,
                limit: policy.max_samples,
            });
        }

        validate_nested_structure(
            &self.kind,
            policy,
            0,
        )?;

        Ok(())
    }

    /// Returns a stable structural summary useful to diagnostics.
    #[must_use]
    pub fn summary(&self) -> WaveformSummary {
        WaveformSummary {
            id: self.id,
            kind: self.kind.kind_name(),
            sample_count: self.sample_count(),
            sampled: self.is_sampled(),
            parametric: self.is_parametric(),
            composite: self.is_composite(),
            normalization: self.normalization,
            domain: self.domain,
            sample_rate: self.sample_rate,
            metadata_fields: self.metadata.len(),
        }
    }
}

// =============================================================================
// Waveform summary
// =============================================================================

/// Deterministic non-owning waveform summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveformSummary {
    /// Waveform identity.
    pub id: WaveformId,

    /// Stable semantic kind name.
    pub kind: &'static str,

    /// Number of explicitly represented samples.
    pub sample_count: usize,

    /// Whether the waveform is sampled.
    pub sampled: bool,

    /// Whether the waveform is parametric.
    pub parametric: bool,

    /// Whether the waveform is composite.
    pub composite: bool,

    /// Normalization semantics.
    pub normalization: WaveformNormalization,

    /// Domain semantics.
    pub domain: WaveformDomain,

    /// Optional semantic sample rate.
    pub sample_rate: Option<WaveformSampleRate>,

    /// Number of metadata fields.
    pub metadata_fields: usize,
}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit validation/resource policy for waveform structures.
///
/// This is separate from the canonical waveform representation.
///
/// A policy controls how much work a validator is willing to perform. It does
/// not change the meaning of a waveform and does not define a maximum quantum
/// machine size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveformValidationPolicy {
    /// Maximum number of materialized samples accepted by this validation.
    pub max_samples: usize,

    /// Maximum number of composite children.
    pub max_composite_components: usize,

    /// Maximum number of piecewise segments.
    pub max_piecewise_segments: usize,

    /// Maximum nested waveform depth checked by this validation.
    pub max_nesting_depth: usize,
}

impl Default for WaveformValidationPolicy {
    fn default() -> Self {
        Self {
            max_samples: DEFAULT_MAX_SAMPLES,
            max_composite_components:
                DEFAULT_MAX_COMPOSITE_COMPONENTS,
            max_piecewise_segments:
                DEFAULT_MAX_PIECEWISE_SEGMENTS,
            max_nesting_depth: 1024,
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
        max_nesting_depth: usize,
    ) -> Self {
        Self {
            max_samples,
            max_composite_components,
            max_piecewise_segments,
            max_nesting_depth,
        }
    }

    /// Creates a policy suitable for a larger resource budget.
    ///
    /// The caller remains responsible for external memory/CPU limits.
    #[must_use]
    pub const fn unlimited_for_explicit_resources() -> Self {
        Self {
            max_samples: usize::MAX,
            max_composite_components: usize::MAX,
            max_piecewise_segments: usize::MAX,
            max_nesting_depth: usize::MAX,
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

        if self.max_nesting_depth == 0 {
            return Err(WaveformError::InvalidValidationPolicy(
                "max_nesting_depth cannot be zero",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by waveform-local construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaveformError {
    /// A concrete floating-point value was NaN or infinite.
    NonFiniteValue {
        /// Human-readable numerical field name.
        field: &'static str,
    },

    /// A parameter supplied by the canonical parameter system was invalid.
    Parameter(String),

    /// A numerical parameter has an invalid concrete value.
    InvalidParameter(&'static str),

    /// Duration/sample-related numerical computation overflowed.
    NumericalOverflow,

    /// Arithmetic involving sample counts overflowed.
    SizeOverflow,

    /// A sampled waveform contained no samples.
    EmptySampledWaveform,

    /// A piecewise waveform contained no segments.
    EmptyPiecewiseWaveform,

    /// A composite waveform contained no components.
    EmptyCompositeWaveform,

    /// A segment had invalid concrete bounds.
    InvalidSegmentBounds,

    /// A sample-rate value was zero.
    ZeroSampleRate,

    /// A real waveform contained a non-zero quadrature component.
    SampleFormatMismatch,

    /// A validation policy is invalid.
    InvalidValidationPolicy(&'static str),

    /// Too many samples for the selected validation policy.
    SampleLimitExceeded {
        /// Number of samples encountered.
        actual: usize,

        /// Policy limit.
        limit: usize,
    },

    /// Too many composite components.
    CompositeComponentLimitExceeded {
        /// Number encountered.
        actual: usize,

        /// Policy limit.
        limit: usize,
    },

    /// Too many piecewise segments.
    PiecewiseSegmentLimitExceeded {
        /// Number encountered.
        actual: usize,

        /// Policy limit.
        limit: usize,
    },

    /// Waveform nesting exceeds the selected validation budget.
    NestingDepthExceeded {
        /// Current nesting depth.
        actual: usize,

        /// Policy limit.
        limit: usize,
    },

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata key is too large.
    MetadataKeyTooLong,

    /// Metadata value is too large.
    MetadataValueTooLong,

    /// Too many metadata fields.
    MetadataFieldLimitExceeded,

    /// A custom waveform namespace is empty.
    EmptyExtensionNamespace,

    /// A custom waveform kind is empty.
    EmptyExtensionKind,
}

impl fmt::Display for WaveformError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "waveform {field} must be finite"
                )
            }

            Self::Parameter(error) => {
                write!(
                    formatter,
                    "invalid waveform parameter: {error}"
                )
            }

            Self::InvalidParameter(message) => {
                formatter.write_str(message)
            }

            Self::NumericalOverflow => {
                formatter.write_str(
                    "waveform numerical operation overflowed",
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "waveform size arithmetic overflowed",
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
                    "piecewise segment has invalid bounds",
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

            Self::CompositeComponentLimitExceeded {
                actual,
                limit,
            } => {
                write!(
                    formatter,
                    "waveform composite component count {actual} exceeds validation limit {limit}"
                )
            }

            Self::PiecewiseSegmentLimitExceeded {
                actual,
                limit,
            } => {
                write!(
                    formatter,
                    "waveform piecewise segment count {actual} exceeds validation limit {limit}"
                )
            }

            Self::NestingDepthExceeded { actual, limit } => {
                write!(
                    formatter,
                    "waveform nesting depth {actual} exceeds validation limit {limit}"
                )
            }

            Self::EmptyMetadataKey => {
                formatter.write_str(
                    "waveform metadata key cannot be empty",
                )
            }

            Self::MetadataKeyTooLong => {
                formatter.write_str(
                    "waveform metadata key is too long",
                )
            }

            Self::MetadataValueTooLong => {
                formatter.write_str(
                    "waveform metadata value is too long",
                )
            }

            Self::MetadataFieldLimitExceeded => {
                formatter.write_str(
                    "waveform metadata field limit exceeded",
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

fn validate_metadata_key(
    key: &str,
) -> WaveformResult<()> {
    if key.trim().is_empty() {
        return Err(WaveformError::EmptyMetadataKey);
    }

    if key.len() > DEFAULT_MAX_METADATA_KEY_BYTES {
        return Err(WaveformError::MetadataKeyTooLong);
    }

    Ok(())
}

fn validate_metadata_value(
    value: &str,
) -> WaveformResult<()> {
    if value.len() > DEFAULT_MAX_METADATA_VALUE_BYTES {
        return Err(WaveformError::MetadataValueTooLong);
    }

    Ok(())
}

fn validate_nested_structure(
    kind: &WaveformKind,
    policy: WaveformValidationPolicy,
    depth: usize,
) -> WaveformResult<()> {
    if depth > policy.max_nesting_depth {
        return Err(WaveformError::NestingDepthExceeded {
            actual: depth,
            limit: policy.max_nesting_depth,
        });
    }

    match kind {
        WaveformKind::Piecewise(piecewise) => {
            if piecewise.len() > policy.max_piecewise_segments {
                return Err(
                    WaveformError::PiecewiseSegmentLimitExceeded {
                        actual: piecewise.len(),
                        limit: policy.max_piecewise_segments,
                    },
                );
            }

            for segment in piecewise.segments() {
                validate_nested_structure(
                    &segment.waveform,
                    policy,
                    depth.saturating_add(1),
                )?;
            }
        }

        WaveformKind::Composite(composite) => {
            if composite.len()
                > policy.max_composite_components
            {
                return Err(
                    WaveformError::CompositeComponentLimitExceeded {
                        actual: composite.len(),
                        limit: policy.max_composite_components,
                    },
                );
            }

            for component in composite.components() {
                validate_nested_structure(
                    component,
                    policy,
                    depth.saturating_add(1),
                )?;
            }
        }

        WaveformKind::Constant(_)
        | WaveformKind::Square(_)
        | WaveformKind::Gaussian(_)
        | WaveformKind::Drag(_)
        | WaveformKind::Sine(_)
        | WaveformKind::Cosine(_)
        | WaveformKind::Sampled(_)
        | WaveformKind::Custom(_) => {}
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::WaveformId;

    fn constant_parameter(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    #[test]
    fn sample_rate_rejects_zero() {
        let result = WaveformSampleRate::new(0);

        assert!(matches!(
            result,
            Err(WaveformError::ZeroSampleRate)
        ));
    }

    #[test]
    fn sample_rate_accepts_large_explicit_value() {
        let result = WaveformSampleRate::new(u64::MAX);

        assert!(result.is_ok());
    }

    #[test]
    fn complex_sample_rejects_nan() {
        let result = ComplexSample::new(
            f64::NAN,
            0.0,
        );

        assert!(matches!(
            result,
            Err(WaveformError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn complex_sample_rejects_infinity() {
        let result = ComplexSample::new(
            f64::INFINITY,
            0.0,
        );

        assert!(matches!(
            result,
            Err(WaveformError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn complex_sample_accepts_finite_iq() {
        let sample = ComplexSample::new(
            0.25,
            -0.75,
        )
        .expect("finite IQ sample must be valid");

        assert_eq!(sample.i(), 0.25);
        assert_eq!(sample.q(), -0.75);
        assert!(sample.is_finite());
    }

    #[test]
    fn real_sample_has_zero_q() {
        let sample = ComplexSample::real(0.5)
            .expect("finite real sample must be valid");

        assert_eq!(sample.i(), 0.5);
        assert_eq!(sample.q(), 0.0);
    }

    #[test]
    fn sampled_real_waveform_is_valid() {
        let waveform = SampledWaveform::from_real(
            vec![0.0, 0.5, 1.0, 0.5, 0.0],
            None,
        )
        .expect("sampled waveform must be valid");

        assert_eq!(waveform.len(), 5);
        assert!(!waveform.is_empty());
        assert_eq!(
            waveform.format(),
            WaveformSampleFormat::Real
        );
    }

    #[test]
    fn sampled_iq_waveform_is_valid() {
        let waveform = SampledWaveform::from_iq(
            vec![
                ComplexSample::new(1.0, 0.0)
                    .expect("finite sample"),
                ComplexSample::new(0.0, 1.0)
                    .expect("finite sample"),
            ],
            Some(
                WaveformSampleRate::new(1_000_000_000)
                    .expect("non-zero sample rate"),
            ),
        )
        .expect("IQ waveform must be valid");

        assert_eq!(waveform.len(), 2);
        assert_eq!(
            waveform.format(),
            WaveformSampleFormat::ComplexIq
        );
    }

    #[test]
    fn real_waveform_rejects_nonzero_q() {
        let result = SampledWaveform::new(
            vec![
                ComplexSample::new(1.0, 0.25)
                    .expect("sample itself is finite"),
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
    fn sampled_waveform_rejects_empty_samples() {
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
    fn gaussian_rejects_non_positive_sigma() {
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
    fn gaussian_accepts_symbolic_sigma() {
        let sigma =
            Parameter::symbol("sigma")
                .expect("symbol should be valid");

        let result = GaussianWaveform::new(
            constant_parameter(1.0),
            sigma,
            constant_parameter(0.5),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn drag_accepts_finite_parameters() {
        let waveform = DragWaveform::new(
            constant_parameter(1.0),
            constant_parameter(0.2),
            constant_parameter(0.5),
            constant_parameter(0.1),
        )
        .expect("valid DRAG waveform");

        waveform
            .validate()
            .expect("valid DRAG waveform must validate");
    }

    #[test]
    fn sine_accepts_symbolic_frequency() {
        let cycles =
            Parameter::symbol("cycles")
                .expect("symbol should be valid");

        let waveform = SinusoidalWaveform::new(
            ComplexSample::real(1.0)
                .expect("finite amplitude"),
            cycles,
            constant_parameter(0.0),
        )
        .expect("valid sine waveform");

        waveform
            .validate()
            .expect("valid sine waveform");
    }

    #[test]
    fn piecewise_rejects_invalid_concrete_bounds() {
        let segment = PiecewiseSegment::new(
            constant_parameter(0.75),
            constant_parameter(0.25),
            WaveformKind::Square(
                SquareWaveform::real(1.0)
                    .expect("valid square"),
            ),
        );

        assert!(matches!(
            segment,
            Err(WaveformError::InvalidSegmentBounds)
        ));
    }

    #[test]
    fn piecewise_accepts_valid_segment() {
        let segment = PiecewiseSegment::new(
            constant_parameter(0.0),
            constant_parameter(0.5),
            WaveformKind::Square(
                SquareWaveform::real(1.0)
                    .expect("valid square"),
            ),
        )
        .expect("valid segment");

        let waveform =
            PiecewiseWaveform::new(vec![segment])
                .expect("valid piecewise waveform");

        waveform
            .validate()
            .expect("piecewise waveform must validate");
    }

    #[test]
    fn composite_waveform_is_deterministic() {
        let first = WaveformKind::Square(
            SquareWaveform::real(1.0)
                .expect("valid square"),
        );

        let second = WaveformKind::Constant(
            ConstantWaveform::real(0.5)
                .expect("valid constant"),
        );

        let waveform = CompositeWaveform::new(
            WaveformComposition::Add,
            vec![first, second],
        )
        .expect("valid composite waveform");

        assert_eq!(waveform.len(), 2);
        assert_eq!(
            waveform.operation(),
            WaveformComposition::Add
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata =
            WaveformMetadata::new();

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
                String::from("z")
            ]
        );
    }

    #[test]
    fn metadata_rejects_empty_key() {
        let mut metadata =
            WaveformMetadata::new();

        let result = metadata.insert("", "value");

        assert!(matches!(
            result,
            Err(WaveformError::EmptyMetadataKey)
        ));
    }

    #[test]
    fn custom_waveform_is_namespaced() {
        let mut parameters =
            BTreeMap::new();

        parameters.insert(
            String::from("order"),
            String::from("7"),
        );

        let custom = CustomWaveform::new(
            "zamani.example",
            "my_waveform",
            parameters,
        )
        .expect("custom waveform must be valid");

        assert_eq!(
            custom.namespace(),
            "zamani.example"
        );

        assert_eq!(
            custom.kind(),
            "my_waveform"
        );
    }

    #[test]
    fn waveform_has_no_qubit_dependency() {
        let waveform = Waveform::new(
            WaveformId::new(1),
            WaveformKind::Square(
                SquareWaveform::real(0.3)
                    .expect("valid square"),
            ),
        )
        .expect("valid waveform");

        assert_eq!(
            waveform.id(),
            WaveformId::new(1)
        );

        assert_eq!(
            waveform.kind_name(),
            "square"
        );
    }

    #[test]
    fn waveform_summary_is_stable() {
        let waveform = Waveform::new(
            WaveformId::new(42),
            WaveformKind::Gaussian(
                GaussianWaveform::new(
                    constant_parameter(0.3),
                    constant_parameter(0.1),
                    constant_parameter(0.5),
                )
                .expect("valid Gaussian"),
            ),
        )
        .expect("valid waveform");

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

    #[test]
    fn waveform_validation_policy_is_explicit() {
        let policy =
            WaveformValidationPolicy::new(
                100,
                10,
                10,
                10,
            );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn waveform_validation_policy_rejects_zero() {
        let policy =
            WaveformValidationPolicy::new(
                0,
                10,
                10,
                10,
            );

        assert!(matches!(
            policy.validate(),
            Err(
                WaveformError::InvalidValidationPolicy(_)
            )
        ));
    }

    #[test]
    fn sampled_waveform_energy_is_checked() {
        let waveform =
            SampledWaveform::from_real(
                vec![1.0, 2.0, 3.0],
                None,
            )
            .expect("valid samples");

        let energy =
            waveform
                .energy()
                .expect("finite energy");

        assert_eq!(energy, 14.0);
    }

    #[test]
    fn sampled_waveform_peak_is_checked() {
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
            .expect("valid samples");

        let peak =
            waveform
                .peak_magnitude()
                .expect("finite peak");

        assert_eq!(peak, 0.75);
    }

    #[test]
    fn waveform_supports_very_large_ids() {
        let waveform = Waveform::new(
            WaveformId::new(u64::MAX),
            WaveformKind::Square(
                SquareWaveform::real(0.3)
                    .expect("valid square"),
            ),
        )
        .expect("large identity must be valid");

        assert_eq!(
            waveform.id().value(),
            u64::MAX
        );
    }

    #[test]
    fn symbolic_parameters_remain_symbolic() {
        let amplitude =
            Parameter::symbol("amplitude")
                .expect("valid symbol");

        let gaussian =
            GaussianWaveform::new(
                amplitude.clone(),
                constant_parameter(0.1),
                constant_parameter(0.5),
            )
            .expect("valid symbolic Gaussian");

        assert!(gaussian.amplitude.is_symbolic());
    }

    #[test]
    fn no_implicit_amplitude_clipping_occurs() {
        let waveform =
            SquareWaveform::real(1000.0)
                .expect("finite amplitude is structurally valid");

        waveform
            .validate()
            .expect("IR must not impose target-specific amplitude range");
    }

    #[test]
    fn normalization_is_declarative() {
        let waveform = Waveform::new(
            WaveformId::new(1),
            WaveformKind::Square(
                SquareWaveform::real(0.5)
                    .expect("valid square"),
            ),
        )
        .expect("valid waveform")
        .with_normalization(
            WaveformNormalization::UnitPeak,
        );

        assert_eq!(
            waveform.normalization(),
            WaveformNormalization::UnitPeak
        );

        // The original numerical amplitude remains unchanged.
        match waveform.kind() {
            WaveformKind::Square(square) => {
                assert_eq!(square.amplitude.i(), 0.5);
            }

            _ => panic!("expected square waveform"),
        }
    }
}