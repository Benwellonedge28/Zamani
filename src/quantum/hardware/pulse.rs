//! Zamani Quantum — Hardware Pulse Abstraction
//!
//! Production-grade, provider-independent pulse-level quantum hardware model.
//!
//! # Responsibility
//!
//! This module is the authoritative representation of pulse-level quantum
//! control programs inside `quantum::hardware`.
//!
//! It models:
//!
//! - pulse clocks;
//! - sample rates;
//! - IQ waveform samples;
//! - waveform definitions;
//! - virtual hardware channels;
//! - drive channels;
//! - measure channels;
//! - acquire channels;
//! - control channels;
//! - classical register channels;
//! - memory channels;
//! - phase/frequency frames;
//! - pulse instructions;
//! - play instructions;
//! - delay instructions;
//! - acquire instructions;
//! - phase/frequency instructions;
//! - barrier instructions;
//! - pulse schedules;
//! - deterministic ordering;
//! - channel/resource conflicts;
//! - timing validation;
//! - waveform validation;
//! - resource bounds;
//! - serialization;
//! - stable fingerprints;
//! - provider-neutral hardware integration contracts.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP/network communication;
//! - provider authentication;
//! - credentials;
//! - provider SDKs;
//! - OpenQASM parsing;
//! - QIR generation;
//! - transpilation;
//! - routing algorithms;
//! - global quantum scheduling algorithms;
//! - calibration acquisition;
//! - benchmark mathematics;
//! - simulator implementation;
//! - hardware execution.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! hardware compatibility
//!        |
//!        v
//! pulse program
//!        |
//!        +--------------------+
//!        |                    |
//!        v                    v
//!   waveform model       timing model
//!        |                    |
//!        +---------+----------+
//!                  |
//!                  v
//!           PulseSchedule
//!                  |
//!                  v
//!          provider adapter
//!                  |
//!        +---------+---------+
//!        |         |         |
//!        v         v         v
//!      local     provider   simulator
//!        |         |         |
//!        +---------+---------+
//!                  |
//!                  v
//!             quantum hardware
//! ```
//!
//! # Integration contract
//!
//! This module is intentionally independent of `backend.rs`, `calibration.rs`
//! and `topology.rs` at compile time.
//!
//! It may therefore be completed and frozen before those modules are changed.
//!
//! Downstream modules consume this file through:
//!
//! - `BackendCapabilities::pulse_control`;
//! - `backend.rs`;
//! - future `instruction_set.rs`;
//! - future `timing.rs`;
//! - future `execution.rs`;
//! - future `compatibility.rs`;
//! - provider adapters;
//! - OpenQASM adapters;
//! - QIR/native pulse adapters;
//! - simulator/emulator backends;
//! - benchmarking.
//!
//! This module never imports benchmarking.
//!
//! # Design principle
//!
//! Pulse programs are represented using a provider-neutral vocabulary.
//!
//! A provider adapter is responsible for mapping:
//!
//! ```text
//! Zamani PulseSchedule
//!        |
//!        v
//! Provider-specific pulse representation
//! ```
//!
//! Zamani must therefore never expose IBM, IonQ, AWS, Rigetti, Quantinuum,
//! QuEra or another provider's pulse types in this module.
//!
//! # Channel model
//!
//! Modern quantum-control systems distinguish signal and acquisition resources.
//! Zamani therefore models channels explicitly rather than treating a pulse as
//! merely `(waveform, qubit)`.
//!
//! The canonical channel classes are:
//!
//! - Drive;
//! - Measure;
//! - Acquire;
//! - Control;
//! - ClassicalRegister;
//! - Memory;
//! - Marker.
//!
//! Providers may introduce additional channel kinds through `Custom(String)`.
//!
//! The channel identifier is virtual. Provider adapters map it to physical
//! control hardware.
//!
//! # Timing model
//!
//! Pulse scheduling is expressed in integer clock ticks.
//!
//! This is intentional:
//!
//! - it avoids floating-point scheduling drift;
//! - it gives deterministic compilation;
//! - it allows hardware-specific `dt`;
//! - it prevents ambiguous pulse boundaries;
//! - it lets a provider adapter convert ticks into its native units.
//!
//! A `PulseClock` supplies the physical sample rate.
//!
//! # Waveform model
//!
//! A waveform is represented as complex IQ samples:
//!
//! ```text
//! I + iQ
//! ```
//!
//! Sample amplitudes must be finite.
//!
//! The core model does NOT universally impose `|amplitude| <= 1` because some
//! hardware control systems use calibrated physical units or provider-specific
//! scaling. Instead, optional `AmplitudeLimits` allow a backend to impose its
//! actual envelope.
//!
//! # Security
//!
//! Pulse metadata must never contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authentication headers;
//! - cookies;
//! - credentials.
//!
//! Metadata is bounded to prevent accidental memory exhaustion.
//!
//! # Determinism
//!
//! All observable collections use deterministic ordering.
//!
//! Schedule instruction order is explicit and stable.
//!
//! Waveform samples are stored in their supplied order.
//!
//! Channel identifiers implement deterministic ordering.
//!
//! Fingerprints use deterministic JSON serialization.
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
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Current ecosystem note
//!
//! Zamani intentionally models provider-neutral pulse concepts rather than
//! reproducing any one provider SDK. Existing quantum-control ecosystems use
//! concepts such as drive, measure, acquire and control channels, waveforms,
//! frames and scheduled instructions. Those concepts are represented here
//! without making Zamani dependent on a provider's API lifecycle.
//!
//! # File completion guarantee
//!
//! This file is independently complete.
//!
//! It contains:
//!
//! - public model types;
//! - invariants;
//! - validation;
//! - error taxonomy;
//! - serialization;
//! - deterministic fingerprints;
//! - resource limits;
//! - construction APIs;
//! - tests;
//! - integration contracts.
//!
//! No later hardware file needs to modify this file merely because that file
//! is implemented.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const PULSE_SCHEMA_ID: &str = "zamani.quantum.hardware.pulse";

/// Serialized semantic schema version.
pub const PULSE_SCHEMA_VERSION: u16 = 1;

/// Maximum channel identifier length in UTF-8 bytes.
pub const MAX_CHANNEL_ID_LENGTH: usize = 256;

/// Maximum waveform identifier length.
pub const MAX_WAVEFORM_ID_LENGTH: usize = 256;

/// Maximum frame identifier length.
pub const MAX_FRAME_ID_LENGTH: usize = 256;

/// Maximum instruction identifier length.
pub const MAX_INSTRUCTION_ID_LENGTH: usize = 256;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum metadata fields in one pulse program.
pub const MAX_METADATA_FIELDS: usize = 4096;

/// Maximum waveform samples in one waveform.
pub const MAX_WAVEFORM_SAMPLES: usize = 10_000_000;

/// Maximum waveforms in one schedule.
pub const MAX_WAVEFORMS: usize = 1_000_000;

/// Maximum instructions in one schedule.
pub const MAX_INSTRUCTIONS: usize = 1_000_000;

/// Maximum channels in one schedule.
pub const MAX_CHANNELS: usize = 1_000_000;

/// Maximum frame count.
pub const MAX_FRAMES: usize = 1_000_000;

/// Maximum instruction operands.
pub const MAX_INSTRUCTION_OPERANDS: usize = 64;

/// Maximum control-channel source resources.
pub const MAX_CONTROL_SOURCES: usize = 64;

/// Maximum custom channel kind length.
pub const MAX_CUSTOM_CHANNEL_KIND_LENGTH: usize = 128;

// =============================================================================
// Primitive wrappers
// =============================================================================

/// Non-negative integer number of pulse-clock ticks.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PulseTicks(u64);

impl PulseTicks {
    /// Zero ticks.
    pub const ZERO: Self = Self(0);

    /// Creates a tick count.
    pub const fn new(ticks: u64) -> Self {
        Self(ticks)
    }

    /// Returns the underlying tick count.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Checked addition.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, other: u64) -> Option<Self> {
        match self.0.checked_mul(other) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns whether this duration is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for PulseTicks {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ticks", self.0)
    }
}

/// Frequency in hertz.
///
/// Frequency is represented as `f64` because physical control systems commonly
/// expose fractional frequencies. Validation rejects NaN and infinities.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct FrequencyHz(f64);

impl FrequencyHz {
    /// Creates a validated frequency.
    pub fn new(value_hz: f64) -> Result<Self, PulseError> {
        if !value_hz.is_finite() {
            return Err(PulseError::InvalidFrequency {
                value_hz,
            });
        }

        Ok(Self(value_hz))
    }

    /// Returns the frequency in hertz.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for FrequencyHz {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} Hz", self.0)
    }
}

/// Phase in radians.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PhaseRadians(f64);

impl PhaseRadians {
    /// Creates a validated phase.
    pub fn new(value: f64) -> Result<Self, PulseError> {
        if !value.is_finite() {
            return Err(PulseError::InvalidPhase { value });
        }

        Ok(Self(value))
    }

    /// Returns the phase in radians.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for PhaseRadians {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} rad", self.0)
    }
}

/// Sample rate in samples/second.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct SampleRateHz(f64);

impl SampleRateHz {
    /// Creates a positive finite sample rate.
    pub fn new(value_hz: f64) -> Result<Self, PulseError> {
        if !value_hz.is_finite() || value_hz <= 0.0 {
            return Err(PulseError::InvalidSampleRate {
                value_hz,
            });
        }

        Ok(Self(value_hz))
    }

    /// Returns samples/second.
    pub const fn get(self) -> f64 {
        self.0
    }
}

/// Pulse-clock definition.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PulseClock {
    /// Sample rate in samples per second.
    pub sample_rate: SampleRateHz,

    /// Hardware scheduling granularity.
    ///
    /// Usually one pulse tick corresponds to one sample, but some providers
    /// use a larger scheduling granularity.
    pub granularity_ticks: u64,
}

impl PulseClock {
    /// Creates a clock.
    pub fn new(
        sample_rate: SampleRateHz,
        granularity_ticks: u64,
    ) -> Result<Self, PulseError> {
        if granularity_ticks == 0 {
            return Err(PulseError::InvalidClockGranularity);
        }

        Ok(Self {
            sample_rate,
            granularity_ticks,
        })
    }

    /// Returns the duration of one tick in seconds.
    pub fn tick_seconds(self) -> f64 {
        1.0 / self.sample_rate.get()
    }

    /// Converts ticks into seconds.
    pub fn ticks_to_seconds(self, ticks: PulseTicks) -> f64 {
        ticks.get() as f64 / self.sample_rate.get()
    }
}

// =============================================================================
// IQ sample
// =============================================================================

/// Complex IQ sample.
///
/// `i` is the in-phase component and `q` is the quadrature component.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct IQSample {
    /// In-phase component.
    pub i: f64,

    /// Quadrature component.
    pub q: f64,
}

impl IQSample {
    /// Creates a validated IQ sample.
    pub fn new(i: f64, q: f64) -> Result<Self, PulseError> {
        if !i.is_finite() {
            return Err(PulseError::InvalidSampleComponent {
                component: "i",
                value: i,
            });
        }

        if !q.is_finite() {
            return Err(PulseError::InvalidSampleComponent {
                component: "q",
                value: q,
            });
        }

        Ok(Self { i, q })
    }

    /// Zero-valued IQ sample.
    pub const fn zero() -> Self {
        Self { i: 0.0, q: 0.0 }
    }

    /// Magnitude of the complex sample.
    pub fn magnitude(self) -> f64 {
        self.i.hypot(self.q)
    }

    /// Validates the sample against optional amplitude limits.
    pub fn validate(
        self,
        limits: Option<AmplitudeLimits>,
    ) -> Result<(), PulseError> {
        Self::new(self.i, self.q)?;

        if let Some(limits) = limits {
            limits.validate_magnitude(self.magnitude())?;
        }

        Ok(())
    }
}

// =============================================================================
// Amplitude limits
// =============================================================================

/// Optional hardware-specific amplitude limits.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AmplitudeLimits {
    /// Maximum allowed complex magnitude.
    pub max_magnitude: f64,
}

impl AmplitudeLimits {
    /// Creates amplitude limits.
    pub fn new(max_magnitude: f64) -> Result<Self, PulseError> {
        if !max_magnitude.is_finite() || max_magnitude <= 0.0 {
            return Err(PulseError::InvalidAmplitudeLimit {
                value: max_magnitude,
            });
        }

        Ok(Self { max_magnitude })
    }

    fn validate_magnitude(self, magnitude: f64) -> Result<(), PulseError> {
        if magnitude > self.max_magnitude {
            return Err(PulseError::AmplitudeOutOfRange {
                magnitude,
                maximum: self.max_magnitude,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Waveforms
// =============================================================================

/// Waveform representation.
///
/// `Samples` is the canonical provider-neutral representation. Named analytic
/// waveforms are represented explicitly so a provider adapter may preserve
/// symbolic information when supported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Waveform {
    /// Explicit complex IQ samples.
    Samples(Vec<IQSample>),

    /// Constant IQ value for a fixed number of samples.
    Constant {
        /// Sample value.
        value: IQSample,

        /// Number of samples.
        length: u64,
    },

    /// Gaussian envelope.
    Gaussian {
        /// Number of samples.
        length: u64,

        /// Peak amplitude.
        amplitude: IQSample,

        /// Standard deviation in samples.
        sigma: f64,
    },

    /// Drag-like Gaussian derivative envelope.
    GaussianDerivative {
        /// Number of samples.
        length: u64,

        /// Peak amplitude.
        amplitude: IQSample,

        /// Standard deviation in samples.
        sigma: f64,

        /// Derivative scale.
        beta: f64,
    },

    /// Provider-neutral named symbolic waveform.
    ///
    /// The provider adapter decides whether it can execute the symbolic form.
    Symbolic {
        /// Stable symbolic waveform identifier.
        name: String,

        /// Symbolic parameters.
        parameters: BTreeMap<String, f64>,
    },
}

impl Waveform {
    /// Creates an explicit waveform.
    pub fn samples(samples: Vec<IQSample>) -> Result<Self, PulseError> {
        if samples.is_empty() {
            return Err(PulseError::EmptyWaveform);
        }

        if samples.len() > MAX_WAVEFORM_SAMPLES {
            return Err(PulseError::WaveformTooLarge {
                requested: samples.len(),
                maximum: MAX_WAVEFORM_SAMPLES,
            });
        }

        for sample in &samples {
            sample.validate(None)?;
        }

        Ok(Self::Samples(samples))
    }

    /// Creates a constant waveform.
    pub fn constant(
        value: IQSample,
        length: u64,
    ) -> Result<Self, PulseError> {
        if length == 0 {
            return Err(PulseError::ZeroWaveformLength);
        }

        if length as usize > MAX_WAVEFORM_SAMPLES {
            return Err(PulseError::WaveformTooLarge {
                requested: length as usize,
                maximum: MAX_WAVEFORM_SAMPLES,
            });
        }

        value.validate(None)?;

        Ok(Self::Constant { value, length })
    }

    /// Creates a Gaussian waveform.
    pub fn gaussian(
        length: u64,
        amplitude: IQSample,
        sigma: f64,
    ) -> Result<Self, PulseError> {
        if length == 0 {
            return Err(PulseError::ZeroWaveformLength);
        }

        if length as usize > MAX_WAVEFORM_SAMPLES {
            return Err(PulseError::WaveformTooLarge {
                requested: length as usize,
                maximum: MAX_WAVEFORM_SAMPLES,
            });
        }

        if !sigma.is_finite() || sigma <= 0.0 {
            return Err(PulseError::InvalidWaveformParameter {
                parameter: "sigma",
                value: sigma,
            });
        }

        amplitude.validate(None)?;

        Ok(Self::Gaussian {
            length,
            amplitude,
            sigma,
        })
    }

    /// Creates a Gaussian derivative waveform.
    pub fn gaussian_derivative(
        length: u64,
        amplitude: IQSample,
        sigma: f64,
        beta: f64,
    ) -> Result<Self, PulseError> {
        if length == 0 {
            return Err(PulseError::ZeroWaveformLength);
        }

        if length as usize > MAX_WAVEFORM_SAMPLES {
            return Err(PulseError::WaveformTooLarge {
                requested: length as usize,
                maximum: MAX_WAVEFORM_SAMPLES,
            });
        }

        if !sigma.is_finite() || sigma <= 0.0 {
            return Err(PulseError::InvalidWaveformParameter {
                parameter: "sigma",
                value: sigma,
            });
        }

        if !beta.is_finite() {
            return Err(PulseError::InvalidWaveformParameter {
                parameter: "beta",
                value: beta,
            });
        }

        amplitude.validate(None)?;

        Ok(Self::GaussianDerivative {
            length,
            amplitude,
            sigma,
            beta,
        })
    }

    /// Creates a symbolic waveform.
    pub fn symbolic(
        name: impl Into<String>,
        parameters: BTreeMap<String, f64>,
    ) -> Result<Self, PulseError> {
        let name = normalize_identifier(
            "waveform symbolic name",
            &name.into(),
            MAX_WAVEFORM_ID_LENGTH,
        )?;

        for (key, value) in &parameters {
            validate_identifier(
                "waveform parameter",
                key,
                MAX_WAVEFORM_ID_LENGTH,
            )?;

            if !value.is_finite() {
                return Err(PulseError::InvalidWaveformParameter {
                    parameter: "symbolic parameter",
                    value: *value,
                });
            }
        }

        Ok(Self::Symbolic { name, parameters })
    }

    /// Returns the number of samples if statically known.
    ///
    /// Symbolic waveforms may not have a statically known sample count.
    pub fn sample_count(&self) -> Option<u64> {
        match self {
            Self::Samples(samples) => Some(samples.len() as u64),
            Self::Constant { length, .. } => Some(*length),
            Self::Gaussian { length, .. } => Some(*length),
            Self::GaussianDerivative { length, .. } => Some(*length),
            Self::Symbolic { .. } => None,
        }
    }

    /// Validates the waveform.
    pub fn validate(
        &self,
        amplitude_limits: Option<AmplitudeLimits>,
    ) -> Result<(), PulseError> {
        match self {
            Self::Samples(samples) => {
                if samples.is_empty() {
                    return Err(PulseError::EmptyWaveform);
                }

                if samples.len() > MAX_WAVEFORM_SAMPLES {
                    return Err(PulseError::WaveformTooLarge {
                        requested: samples.len(),
                        maximum: MAX_WAVEFORM_SAMPLES,
                    });
                }

                for sample in samples {
                    sample.validate(amplitude_limits)?;
                }
            }

            Self::Constant { value, length } => {
                if *length == 0 {
                    return Err(PulseError::ZeroWaveformLength);
                }

                value.validate(amplitude_limits)?;
            }

            Self::Gaussian {
                length,
                amplitude,
                sigma,
            } => {
                if *length == 0 {
                    return Err(PulseError::ZeroWaveformLength);
                }

                if *length as usize > MAX_WAVEFORM_SAMPLES {
                    return Err(PulseError::WaveformTooLarge {
                        requested: *length as usize,
                        maximum: MAX_WAVEFORM_SAMPLES,
                    });
                }

                if !sigma.is_finite() || *sigma <= 0.0 {
                    return Err(PulseError::InvalidWaveformParameter {
                        parameter: "sigma",
                        value: *sigma,
                    });
                }

                amplitude.validate(amplitude_limits)?;
            }

            Self::GaussianDerivative {
                length,
                amplitude,
                sigma,
                beta,
            } => {
                if *length == 0 {
                    return Err(PulseError::ZeroWaveformLength);
                }

                if *length as usize > MAX_WAVEFORM_SAMPLES {
                    return Err(PulseError::WaveformTooLarge {
                        requested: *length as usize,
                        maximum: MAX_WAVEFORM_SAMPLES,
                    });
                }

                if !sigma.is_finite() || *sigma <= 0.0 {
                    return Err(PulseError::InvalidWaveformParameter {
                        parameter: "sigma",
                        value: *sigma,
                    });
                }

                if !beta.is_finite() {
                    return Err(PulseError::InvalidWaveformParameter {
                        parameter: "beta",
                        value: *beta,
                    });
                }

                amplitude.validate(amplitude_limits)?;
            }

            Self::Symbolic {
                name,
                parameters,
            } => {
                validate_identifier(
                    "waveform symbolic name",
                    name,
                    MAX_WAVEFORM_ID_LENGTH,
                )?;

                for (key, value) in parameters {
                    validate_identifier(
                        "waveform parameter",
                        key,
                        MAX_WAVEFORM_ID_LENGTH,
                    )?;

                    if !value.is_finite() {
                        return Err(PulseError::InvalidWaveformParameter {
                            parameter: "symbolic parameter",
                            value: *value,
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Channel kinds
// =============================================================================

/// Canonical pulse channel kind.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChannelKind {
    /// Qubit drive/control line.
    Drive,

    /// Measurement stimulus line.
    Measure,

    /// Acquisition/ADC line.
    Acquire,

    /// Auxiliary control line.
    Control,

    /// Fast classical register destination.
    ClassicalRegister,

    /// Classical memory destination.
    Memory,

    /// Marker/control event channel.
    Marker,

    /// Provider-specific channel kind.
    Custom(String),
}

impl ChannelKind {
    /// Stable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Drive => "drive",
            Self::Measure => "measure",
            Self::Acquire => "acquire",
            Self::Control => "control",
            Self::ClassicalRegister => "classical_register",
            Self::Memory => "memory",
            Self::Marker => "marker",
            Self::Custom(value) => value.as_str(),
        }
    }

    fn validate(&self) -> Result<(), PulseError> {
        if let Self::Custom(value) = self {
            validate_identifier(
                "custom channel kind",
                value,
                MAX_CUSTOM_CHANNEL_KIND_LENGTH,
            )?;
        }

        Ok(())
    }

    /// Returns whether this channel can carry waveform output.
    pub const fn is_signal_channel(&self) -> bool {
        matches!(
            self,
            Self::Drive | Self::Measure | Self::Control | Self::Marker
        )
    }

    /// Returns whether this is an acquisition channel.
    pub const fn is_acquisition(&self) -> bool {
        matches!(self, Self::Acquire)
    }

    /// Returns whether this is a classical destination.
    pub const fn is_classical(&self) -> bool {
        matches!(
            self,
            Self::ClassicalRegister | Self::Memory
        )
    }
}

// =============================================================================
// Channel
// =============================================================================

/// Virtual hardware pulse channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PulseChannel {
    /// Channel class.
    pub kind: ChannelKind,

    /// Stable zero-based channel index.
    pub index: u32,

    /// Optional physical quantum resources controlled by the channel.
    pub resources: Vec<u32>,

    /// Optional provider-neutral descriptive label.
    pub label: Option<String>,
}

impl PulseChannel {
    /// Creates a channel.
    pub fn new(
        kind: ChannelKind,
        index: u32,
        resources: Vec<u32>,
    ) -> Result<Self, PulseError> {
        kind.validate()?;

        if resources.len() > MAX_CONTROL_SOURCES {
            return Err(PulseError::TooManyChannelResources {
                requested: resources.len(),
                maximum: MAX_CONTROL_SOURCES,
            });
        }

        let mut canonical_resources = resources;
        canonical_resources.sort_unstable();

        for pair in canonical_resources.windows(2) {
            if pair[0] == pair[1] {
                return Err(PulseError::DuplicateChannelResource {
                    resource: pair[0],
                });
            }
        }

        Ok(Self {
            kind,
            index,
            resources: canonical_resources,
            label: None,
        })
    }

    /// Creates a channel with one physical quantum resource.
    pub fn for_resource(
        kind: ChannelKind,
        index: u32,
        resource: u32,
    ) -> Result<Self, PulseError> {
        Self::new(kind, index, vec![resource])
    }

    /// Sets a descriptive label.
    pub fn with_label(
        mut self,
        label: impl Into<String>,
    ) -> Result<Self, PulseError> {
        let label = normalize_identifier(
            "channel label",
            &label.into(),
            MAX_CHANNEL_ID_LENGTH,
        )?;

        self.label = Some(label);
        Ok(self)
    }

    /// Returns whether this channel addresses a resource.
    pub fn addresses_resource(&self, resource: u32) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }

    /// Returns a stable identifier.
    pub fn stable_id(&self) -> String {
        format!("{}{}", self.kind.as_str(), self.index)
    }

    /// Validates the channel.
    pub fn validate(&self) -> Result<(), PulseError> {
        self.kind.validate()?;

        if self.resources.len() > MAX_CONTROL_SOURCES {
            return Err(PulseError::TooManyChannelResources {
                requested: self.resources.len(),
                maximum: MAX_CONTROL_SOURCES,
            });
        }

        for pair in self.resources.windows(2) {
            if pair[0] >= pair[1] {
                return Err(PulseError::NonCanonicalChannelResources);
            }
        }

        if let Some(label) = &self.label {
            validate_identifier(
                "channel label",
                label,
                MAX_CHANNEL_ID_LENGTH,
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Frame
// =============================================================================

/// Frequency/phase frame attached to a pulse channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PulseFrame {
    /// Stable frame identifier.
    pub id: String,

    /// Channel controlled by this frame.
    pub channel: PulseChannel,

    /// Current frame frequency.
    pub frequency: FrequencyHz,

    /// Current frame phase.
    pub phase: PhaseRadians,
}

impl PulseFrame {
    /// Creates a frame.
    pub fn new(
        id: impl Into<String>,
        channel: PulseChannel,
        frequency: FrequencyHz,
        phase: PhaseRadians,
    ) -> Result<Self, PulseError> {
        let id = normalize_identifier(
            "frame id",
            &id.into(),
            MAX_FRAME_ID_LENGTH,
        )?;

        channel.validate()?;

        Ok(Self {
            id,
            channel,
            frequency,
            phase,
        })
    }

    /// Validates the frame.
    pub fn validate(&self) -> Result<(), PulseError> {
        validate_identifier("frame id", &self.id, MAX_FRAME_ID_LENGTH)?;
        self.channel.validate()?;
        Ok(())
    }
}

// =============================================================================
// Acquire destination
// =============================================================================

/// Classical destination for acquisition results.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AcquireDestination {
    /// Fast low-latency classical register.
    Register(u32),

    /// General classical memory.
    Memory(u32),
}

// =============================================================================
// Pulse instructions
// =============================================================================

/// Provider-neutral pulse instruction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PulseInstruction {
    /// Play a waveform on a signal channel.
    Play {
        /// Waveform identifier.
        waveform: String,

        /// Channel receiving the waveform.
        channel: PulseChannel,

        /// Instruction start time.
        start: PulseTicks,
    },

    /// Block a channel for a duration.
    Delay {
        /// Channel being delayed.
        channel: PulseChannel,

        /// Instruction start time.
        start: PulseTicks,

        /// Delay duration.
        duration: PulseTicks,
    },

    /// Acquire data from an acquisition channel.
    Acquire {
        /// Acquisition channel.
        channel: PulseChannel,

        /// Start time.
        start: PulseTicks,

        /// Acquisition duration.
        duration: PulseTicks,

        /// Classical destination.
        destination: AcquireDestination,
    },

    /// Set an absolute frame phase.
    SetPhase {
        /// Frame identifier.
        frame_id: String,

        /// New phase.
        phase: PhaseRadians,

        /// Start time.
        start: PulseTicks,
    },

    /// Add to the current frame phase.
    ShiftPhase {
        /// Frame identifier.
        frame_id: String,

        /// Phase increment.
        phase: PhaseRadians,

        /// Start time.
        start: PulseTicks,
    },

    /// Set an absolute frame frequency.
    SetFrequency {
        /// Frame identifier.
        frame_id: String,

        /// New frequency.
        frequency: FrequencyHz,

        /// Start time.
        start: PulseTicks,
    },

    /// Shift the current frame frequency.
    ShiftFrequency {
        /// Frame identifier.
        frame_id: String,

        /// Frequency increment.
        frequency: FrequencyHz,

        /// Start time.
        start: PulseTicks,
    },

    /// Synchronization barrier.
    Barrier {
        /// Channels synchronized by the barrier.
        channels: Vec<PulseChannel>,

        /// Barrier time.
        start: PulseTicks,
    },

    /// Provider-neutral marker.
    Marker {
        /// Marker channel.
        channel: PulseChannel,

        /// Start time.
        start: PulseTicks,

        /// Marker duration.
        duration: PulseTicks,

        /// Stable marker identifier.
        marker: String,
    },
}

impl PulseInstruction {
    /// Returns the instruction start time.
    pub const fn start(&self) -> PulseTicks {
        match self {
            Self::Play { start, .. }
            | Self::Delay { start, .. }
            | Self::Acquire { start, .. }
            | Self::SetPhase { start, .. }
            | Self::ShiftPhase { start, .. }
            | Self::SetFrequency { start, .. }
            | Self::ShiftFrequency { start, .. }
            | Self::Barrier { start, .. }
            | Self::Marker { start, .. } => *start,
        }
    }

    /// Returns the instruction duration.
    ///
    /// For phase/frequency changes and barriers, the duration is zero.
    pub fn duration(
        &self,
        waveforms: &BTreeMap<String, Waveform>,
    ) -> Result<PulseTicks, PulseError> {
        match self {
            Self::Play {
                waveform,
                ..
            } => {
                let waveform = waveforms
                    .get(waveform)
                    .ok_or_else(|| PulseError::UnknownWaveform {
                        waveform: waveform.clone(),
                    })?;

                waveform
                    .sample_count()
                    .map(PulseTicks::new)
                    .ok_or_else(|| PulseError::SymbolicWaveformDurationUnknown {
                        waveform: waveform_name(waveform),
                    })
            }

            Self::Delay { duration, .. }
            | Self::Acquire { duration, .. }
            | Self::Marker { duration, .. } => Ok(*duration),

            Self::SetPhase { .. }
            | Self::ShiftPhase { .. }
            | Self::SetFrequency { .. }
            | Self::ShiftFrequency { .. }
            | Self::Barrier { .. } => Ok(PulseTicks::ZERO),
        }
    }

    /// Returns the end time.
    pub fn end(
        &self,
        waveforms: &BTreeMap<String, Waveform>,
    ) -> Result<PulseTicks, PulseError> {
        self.start()
            .checked_add(self.duration(waveforms)?)
            .ok_or(PulseError::TimingOverflow)
    }

    /// Returns all signal/classical channels used by the instruction.
    pub fn channels(&self) -> Vec<PulseChannel> {
        match self {
            Self::Play { channel, .. }
            | Self::Delay { channel, .. }
            | Self::Acquire { channel, .. }
            | Self::Marker { channel, .. } => vec![channel.clone()],

            Self::Barrier { channels, .. } => channels.clone(),

            Self::SetPhase { .. }
            | Self::ShiftPhase { .. }
            | Self::SetFrequency { .. }
            | Self::ShiftFrequency { .. } => Vec::new(),
        }
    }

    /// Validates the instruction independent of waveform lookup.
    pub fn validate(&self) -> Result<(), PulseError> {
        match self {
            Self::Play {
                waveform,
                channel,
                ..
            } => {
                validate_identifier(
                    "waveform reference",
                    waveform,
                    MAX_WAVEFORM_ID_LENGTH,
                )?;

                channel.validate()?;

                if !channel.kind.is_signal_channel() {
                    return Err(PulseError::InvalidChannelForInstruction {
                        instruction: "play",
                        channel: channel.stable_id(),
                    });
                }
            }

            Self::Delay { channel, duration, .. } => {
                channel.validate()?;

                if duration.is_zero() {
                    return Err(PulseError::ZeroDuration {
                        instruction: "delay",
                    });
                }
            }

            Self::Acquire {
                channel,
                duration,
                ..
            } => {
                channel.validate()?;

                if !channel.kind.is_acquisition() {
                    return Err(PulseError::InvalidChannelForInstruction {
                        instruction: "acquire",
                        channel: channel.stable_id(),
                    });
                }

                if duration.is_zero() {
                    return Err(PulseError::ZeroDuration {
                        instruction: "acquire",
                    });
                }
            }

            Self::SetPhase {
                frame_id,
                phase,
                ..
            }
            | Self::ShiftPhase {
                frame_id,
                phase,
                ..
            } => {
                validate_identifier(
                    "frame reference",
                    frame_id,
                    MAX_FRAME_ID_LENGTH,
                )?;

                if !phase.get().is_finite() {
                    return Err(PulseError::InvalidPhase {
                        value: phase.get(),
                    });
                }
            }

            Self::SetFrequency {
                frame_id,
                frequency,
                ..
            }
            | Self::ShiftFrequency {
                frame_id,
                frequency,
                ..
            } => {
                validate_identifier(
                    "frame reference",
                    frame_id,
                    MAX_FRAME_ID_LENGTH,
                )?;

                if !frequency.get().is_finite() {
                    return Err(PulseError::InvalidFrequency {
                        value_hz: frequency.get(),
                    });
                }
            }

            Self::Barrier { channels, .. } => {
                if channels.is_empty() {
                    return Err(PulseError::EmptyBarrier);
                }

                if channels.len() > MAX_INSTRUCTION_OPERANDS {
                    return Err(PulseError::TooManyInstructionOperands {
                        requested: channels.len(),
                        maximum: MAX_INSTRUCTION_OPERANDS,
                    });
                }

                let mut ids = BTreeSet::new();

                for channel in channels {
                    channel.validate()?;

                    if !ids.insert(channel.stable_id()) {
                        return Err(PulseError::DuplicateBarrierChannel);
                    }
                }
            }

            Self::Marker {
                channel,
                duration,
                marker,
                ..
            } => {
                channel.validate()?;

                validate_identifier(
                    "marker",
                    marker,
                    MAX_INSTRUCTION_ID_LENGTH,
                )?;

                if duration.is_zero() {
                    return Err(PulseError::ZeroDuration {
                        instruction: "marker",
                    });
                }

                if !channel.kind.is_signal_channel() {
                    return Err(PulseError::InvalidChannelForInstruction {
                        instruction: "marker",
                        channel: channel.stable_id(),
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Pulse schedule
// =============================================================================

/// Complete provider-neutral pulse program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PulseSchedule {
    /// Schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Pulse clock.
    pub clock: PulseClock,

    /// Optional backend target identifier.
    ///
    /// This is metadata only. Provider execution remains the adapter's job.
    pub target: Option<String>,

    /// Waveform definitions.
    pub waveforms: BTreeMap<String, Waveform>,

    /// Frame definitions.
    pub frames: BTreeMap<String, PulseFrame>,

    /// Explicitly declared channels.
    pub channels: BTreeMap<String, PulseChannel>,

    /// Ordered pulse instructions.
    pub instructions: Vec<PulseInstruction>,

    /// Program-level metadata.
    pub metadata: BTreeMap<String, String>,
}

impl PulseSchedule {
    /// Creates an empty pulse schedule.
    pub fn new(clock: PulseClock) -> Self {
        Self {
            schema_id: PULSE_SCHEMA_ID.to_owned(),
            schema_version: PULSE_SCHEMA_VERSION,
            clock,
            target: None,
            waveforms: BTreeMap::new(),
            frames: BTreeMap::new(),
            channels: BTreeMap::new(),
            instructions: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Sets an optional target.
    pub fn with_target(
        mut self,
        target: impl Into<String>,
    ) -> Result<Self, PulseError> {
        let target = normalize_identifier(
            "pulse target",
            &target.into(),
            MAX_CHANNEL_ID_LENGTH,
        )?;

        self.target = Some(target);
        Ok(self)
    }

    /// Adds a waveform.
    pub fn add_waveform(
        &mut self,
        id: impl Into<String>,
        waveform: Waveform,
    ) -> Result<(), PulseError> {
        if self.waveforms.len() >= MAX_WAVEFORMS {
            return Err(PulseError::WaveformCountExceeded {
                requested: self.waveforms.len() + 1,
                maximum: MAX_WAVEFORMS,
            });
        }

        let id = normalize_identifier(
            "waveform id",
            &id.into(),
            MAX_WAVEFORM_ID_LENGTH,
        )?;

        waveform.validate(None)?;

        if self.waveforms.contains_key(&id) {
            return Err(PulseError::DuplicateWaveform {
                waveform: id,
            });
        }

        self.waveforms.insert(id, waveform);

        Ok(())
    }

    /// Adds a channel.
    pub fn add_channel(
        &mut self,
        channel: PulseChannel,
    ) -> Result<(), PulseError> {
        if self.channels.len() >= MAX_CHANNELS {
            return Err(PulseError::ChannelCountExceeded {
                requested: self.channels.len() + 1,
                maximum: MAX_CHANNELS,
            });
        }

        channel.validate()?;

        let id = channel.stable_id();

        if self.channels.contains_key(&id) {
            return Err(PulseError::DuplicateChannel {
                channel: id,
            });
        }

        self.channels.insert(id, channel);

        Ok(())
    }

    /// Adds a frame.
    pub fn add_frame(
        &mut self,
        frame: PulseFrame,
    ) -> Result<(), PulseError> {
        if self.frames.len() >= MAX_FRAMES {
            return Err(PulseError::FrameCountExceeded {
                requested: self.frames.len() + 1,
                maximum: MAX_FRAMES,
            });
        }

        frame.validate()?;

        if self.frames.contains_key(&frame.id) {
            return Err(PulseError::DuplicateFrame {
                frame: frame.id,
            });
        }

        self.frames.insert(frame.id.clone(), frame);

        Ok(())
    }

    /// Adds metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), PulseError> {
        if self.metadata.len() >= MAX_METADATA_FIELDS {
            return Err(PulseError::MetadataCountExceeded {
                requested: self.metadata.len() + 1,
                maximum: MAX_METADATA_FIELDS,
            });
        }

        let key = normalize_identifier(
            "pulse metadata key",
            &key.into(),
            MAX_METADATA_KEY_LENGTH,
        )?;

        let value = value.into();

        if value.as_bytes().len() > MAX_METADATA_VALUE_LENGTH {
            return Err(PulseError::MetadataValueTooLong {
                key,
                length: value.as_bytes().len(),
                maximum: MAX_METADATA_VALUE_LENGTH,
            });
        }

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Appends an instruction.
    pub fn push(
        &mut self,
        instruction: PulseInstruction,
    ) -> Result<(), PulseError> {
        if self.instructions.len() >= MAX_INSTRUCTIONS {
            return Err(PulseError::InstructionCountExceeded {
                requested: self.instructions.len() + 1,
                maximum: MAX_INSTRUCTIONS,
            });
        }

        instruction.validate()?;

        self.instructions.push(instruction);

        Ok(())
    }

    /// Validates the entire schedule.
    pub fn validate(&self) -> Result<(), PulseError> {
        if self.schema_id != PULSE_SCHEMA_ID {
            return Err(PulseError::SchemaMismatch {
                expected: PULSE_SCHEMA_ID,
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != PULSE_SCHEMA_VERSION {
            return Err(PulseError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if let Some(target) = &self.target {
            validate_identifier(
                "pulse target",
                target,
                MAX_CHANNEL_ID_LENGTH,
            )?;
        }

        if self.waveforms.len() > MAX_WAVEFORMS {
            return Err(PulseError::WaveformCountExceeded {
                requested: self.waveforms.len(),
                maximum: MAX_WAVEFORMS,
            });
        }

        if self.frames.len() > MAX_FRAMES {
            return Err(PulseError::FrameCountExceeded {
                requested: self.frames.len(),
                maximum: MAX_FRAMES,
            });
        }

        if self.channels.len() > MAX_CHANNELS {
            return Err(PulseError::ChannelCountExceeded {
                requested: self.channels.len(),
                maximum: MAX_CHANNELS,
            });
        }

        if self.instructions.len() > MAX_INSTRUCTIONS {
            return Err(PulseError::InstructionCountExceeded {
                requested: self.instructions.len(),
                maximum: MAX_INSTRUCTIONS,
            });
        }

        if self.metadata.len() > MAX_METADATA_FIELDS {
            return Err(PulseError::MetadataCountExceeded {
                requested: self.metadata.len(),
                maximum: MAX_METADATA_FIELDS,
            });
        }

        for (id, waveform) in &self.waveforms {
            validate_identifier(
                "waveform id",
                id,
                MAX_WAVEFORM_ID_LENGTH,
            )?;

            waveform.validate(None)?;
        }

        for (id, channel) in &self.channels {
            validate_identifier(
                "channel id",
                id,
                MAX_CHANNEL_ID_LENGTH,
            )?;

            channel.validate()?;

            if id != &channel.stable_id() {
                return Err(PulseError::ChannelKeyMismatch {
                    key: id.clone(),
                    expected: channel.stable_id(),
                });
            }
        }

        for (id, frame) in &self.frames {
            if id != &frame.id {
                return Err(PulseError::FrameKeyMismatch {
                    key: id.clone(),
                    expected: frame.id.clone(),
                });
            }

            frame.validate()?;
        }

        for (key, value) in &self.metadata {
            validate_identifier(
                "pulse metadata key",
                key,
                MAX_METADATA_KEY_LENGTH,
            )?;

            if value.as_bytes().len() > MAX_METADATA_VALUE_LENGTH {
                return Err(PulseError::MetadataValueTooLong {
                    key: key.clone(),
                    length: value.as_bytes().len(),
                    maximum: MAX_METADATA_VALUE_LENGTH,
                });
            }
        }

        for instruction in &self.instructions {
            instruction.validate()?;
            self.validate_instruction_references(instruction)?;
        }

        self.validate_channel_conflicts()?;
        self.validate_frame_channels()?;

        Ok(())
    }

    fn validate_instruction_references(
        &self,
        instruction: &PulseInstruction,
    ) -> Result<(), PulseError> {
        for channel in instruction.channels() {
            let id = channel.stable_id();

            if let Some(declared) = self.channels.get(&id) {
                if declared != &channel {
                    return Err(PulseError::ChannelDefinitionMismatch {
                        channel: id,
                    });
                }
            }
        }

        match instruction {
            PulseInstruction::Play { waveform, .. } => {
                if !self.waveforms.contains_key(waveform) {
                    return Err(PulseError::UnknownWaveform {
                        waveform: waveform.clone(),
                    });
                }
            }

            PulseInstruction::SetPhase { frame_id, .. }
            | PulseInstruction::ShiftPhase { frame_id, .. }
            | PulseInstruction::SetFrequency { frame_id, .. }
            | PulseInstruction::ShiftFrequency { frame_id, .. } => {
                if !self.frames.contains_key(frame_id) {
                    return Err(PulseError::UnknownFrame {
                        frame: frame_id.clone(),
                    });
                }
            }

            PulseInstruction::Delay { .. }
            | PulseInstruction::Acquire { .. }
            | PulseInstruction::Barrier { .. }
            | PulseInstruction::Marker { .. } => {}
        }

        Ok(())
    }

    fn validate_frame_channels(&self) -> Result<(), PulseError> {
        for frame in self.frames.values() {
            let id = frame.channel.stable_id();

            if let Some(channel) = self.channels.get(&id) {
                if channel != &frame.channel {
                    return Err(PulseError::ChannelDefinitionMismatch {
                        channel: id,
                    });
                }
            }
        }

        Ok(())
    }

    /// Detects overlapping instructions on the same channel.
    ///
    /// Zero-duration instructions are permitted at the same timestamp.
    pub fn validate_channel_conflicts(&self) -> Result<(), PulseError> {
        let mut per_channel: BTreeMap<String, Vec<(PulseTicks, PulseTicks)>> =
            BTreeMap::new();

        for instruction in &self.instructions {
            let start = instruction.start();
            let end = instruction.end(&self.waveforms)?;

            for channel in instruction.channels() {
                per_channel
                    .entry(channel.stable_id())
                    .or_default()
                    .push((start, end));
            }
        }

        for (channel, mut intervals) in per_channel {
            intervals.sort_by_key(|(start, end)| (*start, *end));

            for pair in intervals.windows(2) {
                let (_, previous_end) = pair[0];
                let (next_start, _) = pair[1];

                if previous_end.get() > next_start.get() {
                    return Err(PulseError::ChannelOverlap { channel });
                }
            }
        }

        Ok(())
    }

    /// Returns the total schedule duration.
    pub fn duration(&self) -> Result<PulseTicks, PulseError> {
        let mut maximum = PulseTicks::ZERO;

        for instruction in &self.instructions {
            let end = instruction.end(&self.waveforms)?;

            if end > maximum {
                maximum = end;
            }
        }

        Ok(maximum)
    }

    /// Returns whether the schedule is empty.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Serializes the schedule deterministically.
    pub fn to_json(&self) -> Result<String, PulseError> {
        self.validate()?;

        serde_json::to_string(self).map_err(|error| {
            PulseError::Serialization {
                message: error.to_string(),
            }
        })
    }

    /// Deserializes and validates a schedule.
    pub fn from_json(json: &str) -> Result<Self, PulseError> {
        let schedule: Self = serde_json::from_str(json).map_err(|error| {
            PulseError::Serialization {
                message: error.to_string(),
            }
        })?;

        schedule.validate()?;

        Ok(schedule)
    }

    /// Returns a deterministic SHA-256 fingerprint.
    ///
    /// This fingerprint provides content identity/integrity.
    ///
    /// It is NOT a digital signature and does not authenticate the provider.
    pub fn fingerprint(&self) -> Result<String, PulseError> {
        let json = self.to_json()?;

        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());

        Ok(hex::encode(hasher.finalize()))
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Production pulse-model errors.
#[derive(Debug, Clone, PartialEq)]
pub enum PulseError {
    /// Invalid identifier.
    InvalidIdentifier {
        field: &'static str,
    },

    /// Empty identifier.
    EmptyIdentifier {
        field: &'static str,
    },

    /// Identifier exceeds its bound.
    IdentifierTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// Invalid frequency.
    InvalidFrequency {
        value_hz: f64,
    },

    /// Invalid phase.
    InvalidPhase {
        value: f64,
    },

    /// Invalid sample rate.
    InvalidSampleRate {
        value_hz: f64,
    },

    /// Invalid clock granularity.
    InvalidClockGranularity,

    /// Invalid sample component.
    InvalidSampleComponent {
        component: &'static str,
        value: f64,
    },

    /// Invalid amplitude limit.
    InvalidAmplitudeLimit {
        value: f64,
    },

    /// Sample exceeds amplitude limit.
    AmplitudeOutOfRange {
        magnitude: f64,
        maximum: f64,
    },

    /// Empty waveform.
    EmptyWaveform,

    /// Zero waveform length.
    ZeroWaveformLength,

    /// Waveform exceeds resource limit.
    WaveformTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// Invalid waveform parameter.
    InvalidWaveformParameter {
        parameter: &'static str,
        value: f64,
    },

    /// Symbolic waveform has no statically known duration.
    SymbolicWaveformDurationUnknown {
        waveform: String,
    },

    /// Too many channel resources.
    TooManyChannelResources {
        requested: usize,
        maximum: usize,
    },

    /// Duplicate channel resource.
    DuplicateChannelResource {
        resource: u32,
    },

    /// Channel resources are not canonical.
    NonCanonicalChannelResources,

    /// Duplicate waveform.
    DuplicateWaveform {
        waveform: String,
    },

    /// Duplicate channel.
    DuplicateChannel {
        channel: String,
    },

    /// Duplicate frame.
    DuplicateFrame {
        frame: String,
    },

    /// Duplicate barrier channel.
    DuplicateBarrierChannel,

    /// Empty barrier.
    EmptyBarrier,

    /// Too many instruction operands.
    TooManyInstructionOperands {
        requested: usize,
        maximum: usize,
    },

    /// Instruction cannot use a particular channel.
    InvalidChannelForInstruction {
        instruction: &'static str,
        channel: String,
    },

    /// Instruction has zero duration when positive duration is required.
    ZeroDuration {
        instruction: &'static str,
    },

    /// Unknown waveform.
    UnknownWaveform {
        waveform: String,
    },

    /// Unknown frame.
    UnknownFrame {
        frame: String,
    },

    /// Symbolic waveform duration unavailable.
    SymbolicDurationUnavailable,

    /// Timing arithmetic overflowed.
    TimingOverflow,

    /// Schedule contains overlapping instructions.
    ChannelOverlap {
        channel: String,
    },

    /// Schedule channel differs from declaration.
    ChannelDefinitionMismatch {
        channel: String,
    },

    /// Schedule map key differs from channel identifier.
    ChannelKeyMismatch {
        key: String,
        expected: String,
    },

    /// Schedule frame map key differs from frame identifier.
    FrameKeyMismatch {
        key: String,
        expected: String,
    },

    /// Waveform count exceeded.
    WaveformCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Channel count exceeded.
    ChannelCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Frame count exceeded.
    FrameCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Instruction count exceeded.
    InstructionCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Metadata count exceeded.
    MetadataCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Metadata value too long.
    MetadataValueTooLong {
        key: String,
        length: usize,
        maximum: usize,
    },

    /// Schema mismatch.
    SchemaMismatch {
        expected: &'static str,
        actual: String,
    },

    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        version: u16,
    },

    /// Serialization failure.
    Serialization {
        message: String,
    },
}

impl fmt::Display for PulseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid {field} identifier")
            }

            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} identifier cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} identifier is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidFrequency { value_hz } => {
                write!(formatter, "invalid frequency: {value_hz} Hz")
            }

            Self::InvalidPhase { value } => {
                write!(formatter, "invalid phase: {value} radians")
            }

            Self::InvalidSampleRate { value_hz } => {
                write!(formatter, "invalid sample rate: {value_hz} Hz")
            }

            Self::InvalidClockGranularity => {
                formatter.write_str("pulse clock granularity must be greater than zero")
            }

            Self::InvalidSampleComponent { component, value } => {
                write!(
                    formatter,
                    "invalid IQ {component} component: {value}"
                )
            }

            Self::InvalidAmplitudeLimit { value } => {
                write!(
                    formatter,
                    "invalid amplitude limit: {value}"
                )
            }

            Self::AmplitudeOutOfRange {
                magnitude,
                maximum,
            } => {
                write!(
                    formatter,
                    "IQ magnitude {magnitude} exceeds maximum {maximum}"
                )
            }

            Self::EmptyWaveform => {
                formatter.write_str("waveform cannot be empty")
            }

            Self::ZeroWaveformLength => {
                formatter.write_str("waveform length must be greater than zero")
            }

            Self::WaveformTooLarge {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "waveform contains {requested} samples; maximum is {maximum}"
                )
            }

            Self::InvalidWaveformParameter { parameter, value } => {
                write!(
                    formatter,
                    "invalid waveform parameter {parameter}: {value}"
                )
            }

            Self::SymbolicWaveformDurationUnknown { waveform } => {
                write!(
                    formatter,
                    "symbolic waveform `{waveform}` has no statically known duration"
                )
            }

            Self::TooManyChannelResources {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "channel contains {requested} resources; maximum is {maximum}"
                )
            }

            Self::DuplicateChannelResource { resource } => {
                write!(
                    formatter,
                    "channel contains duplicate resource {resource}"
                )
            }

            Self::NonCanonicalChannelResources => {
                formatter.write_str(
                    "channel resources must be strictly increasing",
                )
            }

            Self::DuplicateWaveform { waveform } => {
                write!(formatter, "waveform `{waveform}` already exists")
            }

            Self::DuplicateChannel { channel } => {
                write!(formatter, "channel `{channel}` already exists")
            }

            Self::DuplicateFrame { frame } => {
                write!(formatter, "frame `{frame}` already exists")
            }

            Self::DuplicateBarrierChannel => {
                formatter.write_str("barrier contains a duplicate channel")
            }

            Self::EmptyBarrier => {
                formatter.write_str("barrier must contain at least one channel")
            }

            Self::TooManyInstructionOperands {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "instruction contains {requested} operands; maximum is {maximum}"
                )
            }

            Self::InvalidChannelForInstruction {
                instruction,
                channel,
            } => {
                write!(
                    formatter,
                    "instruction `{instruction}` cannot execute on channel `{channel}`"
                )
            }

            Self::ZeroDuration { instruction } => {
                write!(
                    formatter,
                    "instruction `{instruction}` must have non-zero duration"
                )
            }

            Self::UnknownWaveform { waveform } => {
                write!(formatter, "unknown waveform `{waveform}`")
            }

            Self::UnknownFrame { frame } => {
                write!(formatter, "unknown frame `{frame}`")
            }

            Self::SymbolicDurationUnavailable => {
                formatter.write_str(
                    "symbolic waveform duration cannot be determined",
                )
            }

            Self::TimingOverflow => {
                formatter.write_str("pulse schedule timing overflow")
            }

            Self::ChannelOverlap { channel } => {
                write!(
                    formatter,
                    "pulse instructions overlap on channel `{channel}`"
                )
            }

            Self::ChannelDefinitionMismatch { channel } => {
                write!(
                    formatter,
                    "instruction channel definition conflicts with declared channel `{channel}`"
                )
            }

            Self::ChannelKeyMismatch { key, expected } => {
                write!(
                    formatter,
                    "channel map key `{key}` does not match canonical id `{expected}`"
                )
            }

            Self::FrameKeyMismatch { key, expected } => {
                write!(
                    formatter,
                    "frame map key `{key}` does not match frame id `{expected}`"
                )
            }

            Self::WaveformCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "schedule contains {requested} waveforms; maximum is {maximum}"
                )
            }

            Self::ChannelCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "schedule contains {requested} channels; maximum is {maximum}"
                )
            }

            Self::FrameCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "schedule contains {requested} frames; maximum is {maximum}"
                )
            }

            Self::InstructionCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "schedule contains {requested} instructions; maximum is {maximum}"
                )
            }

            Self::MetadataCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "schedule contains {requested} metadata fields; maximum is {maximum}"
                )
            }

            Self::MetadataValueTooLong {
                key,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "metadata `{key}` is {length} bytes; maximum is {maximum}"
                )
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    formatter,
                    "pulse schema mismatch: expected `{expected}`, got `{actual}`"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported pulse schema version {version}"
                )
            }

            Self::Serialization { message } => {
                write!(
                    formatter,
                    "pulse serialization error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for PulseError {}

// =============================================================================
// Helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), PulseError> {
    if value.trim().is_empty() {
        return Err(PulseError::EmptyIdentifier { field });
    }

    if value.as_bytes().len() > maximum {
        return Err(PulseError::IdentifierTooLong {
            field,
            length: value.as_bytes().len(),
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PulseError::InvalidIdentifier { field });
    }

    Ok(())
}

fn normalize_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<String, PulseError> {
    let value = value.trim();

    validate_identifier(field, value, maximum)?;

    Ok(value.to_owned())
}

fn waveform_name(waveform: &Waveform) -> String {
    match waveform {
        Waveform::Samples(_) => "samples".to_owned(),
        Waveform::Constant { .. } => "constant".to_owned(),
        Waveform::Gaussian { .. } => "gaussian".to_owned(),
        Waveform::GaussianDerivative { .. } => {
            "gaussian_derivative".to_owned()
        }
        Waveform::Symbolic { name, .. } => name.clone(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn clock() -> PulseClock {
        PulseClock::new(
            SampleRateHz::new(1.0e9).expect("valid sample rate"),
            1,
        )
        .expect("valid clock")
    }

    fn drive() -> PulseChannel {
        PulseChannel::for_resource(
            ChannelKind::Drive,
            0,
            0,
        )
        .expect("valid drive")
    }

    fn acquire() -> PulseChannel {
        PulseChannel::for_resource(
            ChannelKind::Acquire,
            0,
            0,
        )
        .expect("valid acquire")
    }

    #[test]
    fn pulse_ticks_are_checked() {
        assert_eq!(
            PulseTicks::new(4)
                .checked_add(PulseTicks::new(5))
                .expect("no overflow")
                .get(),
            9
        );

        assert!(
            PulseTicks::new(u64::MAX)
                .checked_add(PulseTicks::new(1))
                .is_none()
        );
    }

    #[test]
    fn iq_samples_reject_non_finite_values() {
        assert!(
            IQSample::new(f64::NAN, 0.0).is_err()
        );

        assert!(
            IQSample::new(0.0, f64::INFINITY).is_err()
        );
    }

    #[test]
    fn waveform_is_validated() {
        let sample = IQSample::new(0.5, 0.0)
            .expect("valid sample");

        let waveform = Waveform::samples(vec![sample; 4])
            .expect("valid waveform");

        assert_eq!(waveform.sample_count(), Some(4));
    }

    #[test]
    fn waveform_rejects_empty_samples() {
        assert!(
            Waveform::samples(Vec::new()).is_err()
        );
    }

    #[test]
    fn amplitude_limits_are_enforced() {
        let sample = IQSample::new(0.9, 0.0)
            .expect("valid sample");

        let limits = AmplitudeLimits::new(0.5)
            .expect("valid limits");

        assert!(sample.validate(Some(limits)).is_err());
    }

    #[test]
    fn channel_ids_are_stable() {
        let channel = PulseChannel::for_resource(
            ChannelKind::Drive,
            7,
            3,
        )
        .expect("valid channel");

        assert_eq!(channel.stable_id(), "drive7");
    }

    #[test]
    fn channel_resources_are_canonicalized() {
        let channel = PulseChannel::new(
            ChannelKind::Control,
            0,
            vec![4, 1, 3],
        )
        .expect("valid channel");

        assert_eq!(channel.resources, vec![1, 3, 4]);
    }

    #[test]
    fn duplicate_channel_resources_are_rejected() {
        assert!(
            PulseChannel::new(
                ChannelKind::Control,
                0,
                vec![1, 1],
            )
            .is_err()
        );
    }

    #[test]
    fn acquire_requires_acquire_channel() {
        let instruction = PulseInstruction::Acquire {
            channel: drive(),
            start: PulseTicks::ZERO,
            duration: PulseTicks::new(16),
            destination: AcquireDestination::Memory(0),
        };

        assert!(instruction.validate().is_err());
    }

    #[test]
    fn play_requires_signal_channel() {
        let instruction = PulseInstruction::Play {
            waveform: "w0".to_owned(),
            channel: acquire(),
            start: PulseTicks::ZERO,
        };

        assert!(instruction.validate().is_err());
    }

    #[test]
    fn schedule_can_be_built_and_validated() {
        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive())
            .expect("channel");

        let waveform = Waveform::constant(
            IQSample::new(0.2, 0.0).expect("sample"),
            16,
        )
        .expect("waveform");

        schedule
            .add_waveform("w0", waveform)
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("instruction");

        schedule.validate().expect("valid schedule");

        assert_eq!(
            schedule.duration().expect("duration").get(),
            16
        );
    }

    #[test]
    fn overlapping_instructions_are_rejected() {
        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive())
            .expect("channel");

        schedule
            .add_waveform(
                "w0",
                Waveform::constant(
                    IQSample::new(0.1, 0.0)
                        .expect("sample"),
                    10,
                )
                .expect("waveform"),
            )
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("first instruction");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::new(5),
            })
            .expect("second instruction");

        assert!(
            schedule.validate_channel_conflicts().is_err()
        );
    }

    #[test]
    fn non_overlapping_instructions_are_valid() {
        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive())
            .expect("channel");

        schedule
            .add_waveform(
                "w0",
                Waveform::constant(
                    IQSample::new(0.1, 0.0)
                        .expect("sample"),
                    10,
                )
                .expect("waveform"),
            )
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("first instruction");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::new(10),
            })
            .expect("second instruction");

        schedule
            .validate()
            .expect("schedule should be valid");
    }

    #[test]
    fn different_channels_can_overlap() {
        let drive0 = PulseChannel::for_resource(
            ChannelKind::Drive,
            0,
            0,
        )
        .expect("drive0");

        let drive1 = PulseChannel::for_resource(
            ChannelKind::Drive,
            1,
            1,
        )
        .expect("drive1");

        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive0.clone())
            .expect("drive0");

        schedule
            .add_channel(drive1.clone())
            .expect("drive1");

        schedule
            .add_waveform(
                "w0",
                Waveform::constant(
                    IQSample::new(0.1, 0.0)
                        .expect("sample"),
                    10,
                )
                .expect("waveform"),
            )
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive0,
                start: PulseTicks::ZERO,
            })
            .expect("first");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive1,
                start: PulseTicks::ZERO,
            })
            .expect("second");

        schedule
            .validate()
            .expect("parallel execution is valid");
    }

    #[test]
    fn acquire_instruction_has_duration() {
        let instruction = PulseInstruction::Acquire {
            channel: acquire(),
            start: PulseTicks::new(10),
            duration: PulseTicks::new(20),
            destination: AcquireDestination::Memory(0),
        };

        assert_eq!(
            instruction
                .duration(&BTreeMap::new())
                .expect("duration")
                .get(),
            20
        );

        assert_eq!(
            instruction
                .end(&BTreeMap::new())
                .expect("end")
                .get(),
            30
        );
    }

    #[test]
    fn phase_and_frequency_operations_are_zero_duration() {
        let frame_channel = drive();

        let frame = PulseFrame::new(
            "f0",
            frame_channel,
            FrequencyHz::new(5.0e9)
                .expect("frequency"),
            PhaseRadians::new(0.0)
                .expect("phase"),
        )
        .expect("frame");

        assert_eq!(
            frame.channel.stable_id(),
            "drive0"
        );

        let instruction = PulseInstruction::SetPhase {
            frame_id: "f0".to_owned(),
            phase: PhaseRadians::new(1.0)
                .expect("phase"),
            start: PulseTicks::ZERO,
        };

        assert_eq!(
            instruction
                .duration(&BTreeMap::new())
                .expect("duration")
                .get(),
            0
        );
    }

    #[test]
    fn schedule_round_trips_through_json() {
        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive())
            .expect("channel");

        schedule
            .add_waveform(
                "w0",
                Waveform::constant(
                    IQSample::new(0.25, 0.0)
                        .expect("sample"),
                    8,
                )
                .expect("waveform"),
            )
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("instruction");

        let json = schedule
            .to_json()
            .expect("serialize");

        let restored = PulseSchedule::from_json(&json)
            .expect("deserialize");

        assert_eq!(schedule, restored);
    }

    #[test]
    fn schedule_fingerprint_is_stable() {
        let mut first = PulseSchedule::new(clock());

        first
            .add_channel(drive())
            .expect("channel");

        first
            .add_waveform(
                "w0",
                Waveform::constant(
                    IQSample::new(0.25, 0.0)
                        .expect("sample"),
                    8,
                )
                .expect("waveform"),
            )
            .expect("waveform");

        first
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("instruction");

        let second = PulseSchedule::from_json(
            &first.to_json().expect("json"),
        )
        .expect("restored");

        assert_eq!(
            first.fingerprint().expect("fingerprint"),
            second.fingerprint().expect("fingerprint")
        );
    }

    #[test]
    fn symbolic_waveforms_require_adapter_supplied_duration() {
        let waveform = Waveform::symbolic(
            "provider_specific",
            BTreeMap::new(),
        )
        .expect("symbolic waveform");

        assert_eq!(waveform.sample_count(), None);

        let mut schedule = PulseSchedule::new(clock());

        schedule
            .add_channel(drive())
            .expect("channel");

        schedule
            .add_waveform("w0", waveform)
            .expect("waveform");

        schedule
            .push(PulseInstruction::Play {
                waveform: "w0".to_owned(),
                channel: drive(),
                start: PulseTicks::ZERO,
            })
            .expect("instruction");

        assert!(
            schedule.duration().is_err()
        );
    }
}