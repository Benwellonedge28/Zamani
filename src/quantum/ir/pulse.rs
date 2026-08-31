//! Zamani Quantum IR — Pulse-Level Control Semantics
//!
//! Canonical, hardware-independent representation of pulse-level quantum
//! control in the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `pulse.rs` defines WHAT a pulse means in the canonical Zamani Quantum IR.
//!
//! It owns:
//!
//! - pulse identity;
//! - pulse targets;
//! - pulse duration;
//! - pulse amplitude;
//! - pulse phase;
//! - pulse frequency;
//! - waveform references;
//! - abstract channel references;
//! - frame references;
//! - pulse envelopes at the semantic level;
//! - pulse composition;
//! - pulse metadata;
//! - pulse-local validation;
//! - deterministic pulse construction;
//! - explicit pulse resource requirements;
//! - pulse-local dependency information.
//!
//! It does NOT own:
//!
//! - physical DACs;
//! - ADCs;
//! - microwave generators;
//! - lasers;
//! - physical control wiring;
//! - hardware calibration data;
//! - device topology;
//! - physical channel allocation;
//! - logical-to-physical routing;
//! - pulse scheduling algorithms;
//! - backend execution;
//! - provider SDKs;
//! - provider authentication;
//! - QPU communication;
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
//!      +----------------------+
//!      |                      |
//!      v                      v
//!   gate IR               pulse IR
//!                              |
//!                              v
//!                         optimization
//!                              |
//!                              v
//!                         scheduling
//!                              |
//!                              v
//!                           hardware
//!                              |
//!                              v
//!                            backend
//!                              |
//!                              v
//!                             QPU
//! ```
//!
//! The IR answers:
//!
//! > What pulse-level operation does the program request?
//!
//! The hardware layer answers:
//!
//! > What physical device can implement it and how?
//!
//! # Universal-program principle
//!
//! Zamani programs are written once and may target:
//!
//! - one-qubit systems;
//! - small gate QPUs;
//! - large QPUs;
//! - superconducting systems;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin systems;
//! - analog systems;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - future quantum architectures.
//!
//! This file therefore contains NO architectural quantum-machine-size limit.
//!
//! Values such as:
//!
//! ```text
//! 63
//! 4096
//! 1_000_000
//! ```
//!
//! must never be interpreted as a maximum number of qubits or pulses that
//! Zamani supports.
//!
//! Resource limits are explicit policy concerns handled by `limits.rs` and
//! higher-level compilation policies.
//!
//! # Pulse-level example
//!
//! A Zamani source-level operation such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! is represented semantically as a pulse operation containing:
//!
//! ```text
//! target     = q
//! amplitude  = 0.3
//! duration   = 20ns
//! ```
//!
//! Hardware-specific interpretation happens later.
//!
//! The IR does NOT decide which:
//!
//! - DAC;
//! - microwave source;
//! - laser;
//! - physical channel;
//! - oscillator;
//! - calibration;
//! - sample rate;
//! - device topology
//!
//! implements the pulse.
//!
//! # Dependency stability
//!
//! This file deliberately uses only currently established canonical IR
//! contracts:
//!
//! ```text
//! super::identity
//! super::parameter
//! super::qubit
//! ```
//!
//! It does not require `timing.rs`, `waveform.rs`, `channel.rs`, or `frame.rs`
//! to exist in order to compile.
//!
//! Those later modules integrate through the strongly typed identifiers already
//! defined by `identity.rs`.
//!
//! This is intentional: the file can be completed and frozen before those
//! modules are implemented.
//!
//! # Duration representation
//!
//! Pulse duration is represented internally as an unsigned number of
//! femtoseconds.
//!
//! This provides:
//!
//! - deterministic integer arithmetic;
//! - no floating-point scheduling drift;
//! - no negative duration;
//! - no NaN;
//! - no infinity;
//! - checked arithmetic;
//! - representation of sub-nanosecond control intervals;
//! - convenient conversion from source units such as `20ns`.
//!
//! `PulseDuration` is a pulse-local semantic duration. The future canonical
//! `timing.rs` layer can convert to and from this representation without
//! changing the pulse model.
//!
//! # Numerical semantics
//!
//! Pulse amplitude, phase and frequency may be concrete or symbolic.
//!
//! Symbolic values use the canonical `Parameter` type rather than strings.
//!
//! Therefore:
//!
//! ```text
//! amp=0.3
//! ```
//!
//! is represented by a validated numerical parameter, while:
//!
//! ```text
//! amp=drive_amplitude
//! ```
//!
//! may remain symbolic until a later compilation stage.
//!
//! The pulse IR never silently converts invalid floating-point values into
//! usable pulse values.
//!
//! # Amplitude semantics
//!
//! The canonical pulse model does NOT universally enforce:
//!
//! ```text
//! -1 <= amplitude <= 1
//! ```
//!
//! because amplitude units and scaling are target-dependent.
//!
//! Instead, `PulseAmplitudeLimits` can be supplied by validation or hardware
//! compatibility code.
//!
//! # Physical qubits
//!
//! Pulse semantics normally reference logical qubits through:
//!
//! ```rust
//! quantum::ir::qubit::QubitId
//! ```
//!
//! `PhysicalQubitId` is deliberately not embedded into ordinary pulse
//! semantics.
//!
//! Logical-to-physical placement belongs to routing/mapping.
//!
//! A downstream compiled representation may associate a pulse with a physical
//! target after mapping without changing the canonical source-level pulse.
//!
//! # Security
//!
//! Pulse metadata is not an authentication mechanism.
//!
//! This module never stores:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authentication headers;
//! - provider credentials.
//!
//! Metadata is explicitly bounded when using the validated metadata API.
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
//! No unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `PulseId`, `WaveformId`, `ChannelId`, `FrameId`,
//!     `OperationId`, and `CalibrationId`.
//!
//! `qubit.rs`
//!     Supplies the canonical `QubitId` and `QubitRef` types.
//!
//! `parameter.rs`
//!     Supplies concrete and symbolic parameter values.
//!
//! `timing.rs`
//!     May convert `PulseDuration` into the canonical program-wide timing
//!     representation.
//!
//! `waveform.rs`
//!     Owns full waveform definitions. This file stores only `WaveformId`
//!     references.
//!
//! `channel.rs`
//!     Owns abstract channel definitions. This file stores only `ChannelId`
//!     references.
//!
//! `frame.rs`
//!     Owns frame definitions. This file stores only `FrameId` references.
//!
//! `operation.rs`
//!     References pulses through `PulseId`.
//!
//! `schedule.rs`
//!     Determines when a pulse executes. This file does not schedule pulses.
//!
//! `mapping.rs`
//!     Resolves logical qubits to physical qubits.
//!
//! `hardware/`
//!     Determines whether a target can implement the pulse and maps abstract
//!     pulse semantics to real control resources.
//!
//! `optimization/`
//!     May transform pulses while preserving semantic equivalence.
//!
//! `validation.rs`
//!     Performs whole-program validation and target-independent structural
//!     validation.
//!
//! `serialization.rs`
//!     Serializes the canonical pulse representation.
//!
//! `hash.rs`
//!     May derive deterministic content identity from the structural fields.
//!
//! `provenance.rs`
//!     Records pulse transformations and lineage.
//!
//! # File completion guarantee
//!
//! This file intentionally contains:
//!
//! - pulse model;
//! - pulse identifiers;
//! - pulse target model;
//! - duration model;
//! - numerical parameter integration;
//! - waveform/channel/frame references;
//! - envelope model;
//! - metadata model;
//! - validation;
//! - resource-safe construction;
//! - checked arithmetic;
//! - deterministic accessors;
//! - tests;
//! - integration documentation.
//!
//! Implementing later IR modules should not require changing the semantic
//! contract established here merely because those modules are added.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::identity::{
    CalibrationId,
    ChannelId,
    FrameId,
    PulseId,
    WaveformId,
};
use super::parameter::Parameter;
use super::qubit::QubitId;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for pulse IR.
pub const PULSE_SCHEMA_ID: &str = "zamani.quantum.ir.pulse";

/// Stable semantic schema version.
///
/// Breaking semantic changes require a new major IR schema version.
pub const PULSE_SCHEMA_VERSION: u16 = 1;

/// Default maximum metadata key length in UTF-8 bytes.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default maximum metadata value length in UTF-8 bytes.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Default maximum number of metadata fields.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum number of targets in one pulse.
///
/// This is deliberately a policy value rather than an architectural quantum
/// limit. Multi-target pulses can be permitted by a larger explicit policy.
pub const DEFAULT_MAX_TARGETS: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Result type used by pulse construction and local validation.
pub type PulseResult<T> = Result<T, PulseError>;

// =============================================================================
// Pulse duration
// =============================================================================

/// Pulse duration represented in femtoseconds.
///
/// This is a non-negative integer semantic duration.
///
/// The representation is intentionally integer-based so pulse boundaries are
/// deterministic and cannot drift because of floating-point accumulation.
///
/// Examples:
///
/// ```text
/// 1 fs  = 1
/// 1 ps  = 1_000 fs
/// 1 ns  = 1_000_000 fs
/// 1 us  = 1_000_000_000 fs
/// 1 ms  = 1_000_000_000_000 fs
/// 1 s   = 1_000_000_000_000_000 fs
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct PulseDuration(u64);

impl PulseDuration {
    /// Number of femtoseconds in one picosecond.
    pub const FEMTOSECONDS_PER_PICOSECOND: u64 = 1_000;

    /// Number of femtoseconds in one nanosecond.
    pub const FEMTOSECONDS_PER_NANOSECOND: u64 = 1_000_000;

    /// Number of femtoseconds in one microsecond.
    pub const FEMTOSECONDS_PER_MICROSECOND: u64 = 1_000_000_000;

    /// Number of femtoseconds in one millisecond.
    pub const FEMTOSECONDS_PER_MILLISECOND: u64 = 1_000_000_000_000;

    /// Number of femtoseconds in one second.
    pub const FEMTOSECONDS_PER_SECOND: u64 = 1_000_000_000_000_000;

    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a duration directly from femtoseconds.
    ///
    /// All `u64` values are structurally representable.
    #[must_use]
    pub const fn from_femtoseconds(value: u64) -> Self {
        Self(value)
    }

    /// Creates a duration from whole picoseconds.
    pub const fn from_picoseconds(value: u64) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_PICOSECOND) {
            Some(fs) => Ok(Self(fs)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole nanoseconds.
    pub const fn from_nanoseconds(value: u64) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_NANOSECOND) {
            Some(fs) => Ok(Self(fs)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole microseconds.
    pub const fn from_microseconds(value: u64) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_MICROSECOND) {
            Some(fs) => Ok(Self(fs)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole milliseconds.
    pub const fn from_milliseconds(value: u64) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_MILLISECOND) {
            Some(fs) => Ok(Self(fs)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole seconds.
    pub const fn from_seconds(value: u64) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_SECOND) {
            Some(fs) => Ok(Self(fs)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Returns the duration in femtoseconds.
    #[must_use]
    pub const fn femtoseconds(self) -> u64 {
        self.0
    }

    /// Returns whether this duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Checked addition.
    pub const fn checked_add(self, other: Self) -> PulseResult<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, other: Self) -> PulseResult<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::NegativeDuration),
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, multiplier: u64) -> PulseResult<Self> {
        match self.0.checked_mul(multiplier) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Converts a duration to whole nanoseconds when exactly representable.
    #[must_use]
    pub const fn whole_nanoseconds(self) -> Option<u64> {
        if self.0 % Self::FEMTOSECONDS_PER_NANOSECOND == 0 {
            Some(self.0 / Self::FEMTOSECONDS_PER_NANOSECOND)
        } else {
            None
        }
    }
}

impl Default for PulseDuration {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for PulseDuration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ns) = self.whole_nanoseconds() {
            return write!(formatter, "{ns}ns");
        }

        write!(formatter, "{}fs", self.0)
    }
}

// =============================================================================
// Pulse phase
// =============================================================================

/// Pulse phase semantic value.
///
/// The value is represented by the canonical unit-neutral [`Parameter`].
///
/// The consuming compiler interprets the parameter as radians.
#[derive(Debug, Clone, PartialEq)]
pub struct PulsePhase {
    value: Parameter,
}

impl PulsePhase {
    /// Creates a phase from a canonical parameter.
    #[must_use]
    pub const fn new(value: Parameter) -> Self {
        Self { value }
    }

    /// Creates a concrete phase in radians.
    pub fn radians(value: f64) -> PulseResult<Self> {
        let parameter = Parameter::constant(value)
            .map_err(|error| PulseError::Parameter(error.to_string()))?;

        Ok(Self::new(parameter))
    }

    /// Returns the canonical parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.value
    }

    /// Returns whether the phase is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.value.is_symbolic()
    }

    /// Returns the concrete phase when directly available.
    #[must_use]
    pub fn as_constant(&self) -> Option<f64> {
        self.value.as_constant()
    }

    /// Validates the underlying parameter.
    pub fn validate(&self) -> PulseResult<()> {
        self.value
            .validate()
            .map_err(|error| PulseError::Parameter(error.to_string()))
    }
}

// =============================================================================
// Pulse amplitude
// =============================================================================

/// Pulse amplitude semantic value.
///
/// Amplitude is intentionally unit-neutral at the IR level because different
/// quantum technologies and control stacks use different normalization and
/// physical units.
///
/// Hardware compatibility may later impose an explicit range.
#[derive(Debug, Clone, PartialEq)]
pub struct PulseAmplitude {
    value: Parameter,
}

impl PulseAmplitude {
    /// Creates an amplitude from a canonical parameter.
    #[must_use]
    pub const fn new(value: Parameter) -> Self {
        Self { value }
    }

    /// Creates a concrete amplitude.
    pub fn constant(value: f64) -> PulseResult<Self> {
        let parameter = Parameter::constant(value)
            .map_err(|error| PulseError::Parameter(error.to_string()))?;

        Ok(Self::new(parameter))
    }

    /// Returns the canonical parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.value
    }

    /// Returns whether the amplitude is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.value.is_symbolic()
    }

    /// Returns the concrete value when directly available.
    #[must_use]
    pub fn as_constant(&self) -> Option<f64> {
        self.value.as_constant()
    }

    /// Validates the underlying parameter.
    pub fn validate(&self) -> PulseResult<()> {
        self.value
            .validate()
            .map_err(|error| PulseError::Parameter(error.to_string()))
    }
}

// =============================================================================
// Pulse frequency
// =============================================================================

/// Pulse frequency semantic value in hertz.
///
/// The frequency remains symbolic or concrete through [`Parameter`].
#[derive(Debug, Clone, PartialEq)]
pub struct PulseFrequency {
    value: Parameter,
}

impl PulseFrequency {
    /// Creates a frequency from a canonical parameter.
    #[must_use]
    pub const fn new(value: Parameter) -> Self {
        Self { value }
    }

    /// Creates a concrete frequency in hertz.
    pub fn hertz(value: f64) -> PulseResult<Self> {
        let parameter = Parameter::constant(value)
            .map_err(|error| PulseError::Parameter(error.to_string()))?;

        Ok(Self::new(parameter))
    }

    /// Returns the canonical parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.value
    }

    /// Returns whether the frequency is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.value.is_symbolic()
    }

    /// Returns the concrete frequency when directly available.
    #[must_use]
    pub fn as_constant(&self) -> Option<f64> {
        self.value.as_constant()
    }

    /// Validates the underlying parameter.
    pub fn validate(&self) -> PulseResult<()> {
        self.value
            .validate()
            .map_err(|error| PulseError::Parameter(error.to_string()))
    }
}

// =============================================================================
// Pulse target
// =============================================================================

/// Semantic target of a pulse.
///
/// The canonical source-level pulse target is a logical qubit.
///
/// Physical placement belongs to routing/mapping and hardware integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PulseTarget {
    /// Logical quantum resource.
    Qubit(QubitId),
}

impl PulseTarget {
    /// Creates a logical-qubit pulse target.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Returns the logical qubit when this target is qubit-based.
    #[must_use]
    pub const fn qubit_id(self) -> QubitId {
        match self {
            Self::Qubit(qubit) => qubit,
        }
    }
}

impl From<QubitId> for PulseTarget {
    fn from(qubit: QubitId) -> Self {
        Self::qubit(qubit)
    }
}

impl fmt::Display for PulseTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Qubit(qubit) => write!(formatter, "{qubit}"),
        }
    }
}

// =============================================================================
// Pulse targets
// =============================================================================

/// Deterministic collection of pulse targets.
///
/// The collection preserves caller order because target order may be
/// semantically relevant for future multi-resource pulse operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseTargets {
    targets: Vec<PulseTarget>,
}

impl PulseTargets {
    /// Creates a target collection.
    ///
    /// The collection must not be empty and must not contain duplicate
    /// logical qubits.
    pub fn new(targets: Vec<PulseTarget>) -> PulseResult<Self> {
        if targets.is_empty() {
            return Err(PulseError::EmptyTargets);
        }

        Self::validate_unique(&targets)?;

        Ok(Self { targets })
    }

    /// Creates a single-qubit target collection.
    #[must_use]
    pub fn single(qubit: QubitId) -> Self {
        Self {
            targets: vec![PulseTarget::Qubit(qubit)],
        }
    }

    /// Returns the number of targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether the collection is empty.
    ///
    /// This is always false for successfully constructed values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns the targets in semantic order.
    #[must_use]
    pub fn as_slice(&self) -> &[PulseTarget] {
        &self.targets
    }

    /// Returns the first target.
    #[must_use]
    pub fn first(&self) -> Option<PulseTarget> {
        self.targets.first().copied()
    }

    /// Validates uniqueness.
    fn validate_unique(targets: &[PulseTarget]) -> PulseResult<()> {
        let mut seen = std::collections::BTreeSet::new();

        for target in targets {
            let qubit = target.qubit_id();

            if !seen.insert(qubit) {
                return Err(PulseError::DuplicateTarget { qubit });
            }
        }

        Ok(())
    }

    /// Validates this collection against an explicit policy.
    pub fn validate_with_max_targets(
        &self,
        maximum: usize,
    ) -> PulseResult<()> {
        if self.targets.len() > maximum {
            return Err(PulseError::TargetLimitExceeded {
                actual: self.targets.len(),
                maximum,
            });
        }

        Self::validate_unique(&self.targets)
    }
}

// =============================================================================
// Pulse envelope
// =============================================================================

/// Semantic pulse envelope.
///
/// The actual sampled waveform belongs to `waveform.rs`.
///
/// This enum provides common high-level pulse-envelope descriptions that may
/// later be lowered into concrete waveforms.
#[derive(Debug, Clone, PartialEq)]
pub enum PulseEnvelope {
    /// Constant/square envelope.
    Constant,

    /// Gaussian envelope with standard-deviation parameter.
    Gaussian {
        /// Standard deviation in seconds or target-defined normalized units.
        sigma: Parameter,
    },

    /// DRAG envelope.
    Drag {
        /// Gaussian standard-deviation parameter.
        sigma: Parameter,

        /// DRAG correction coefficient.
        beta: Parameter,
    },

    /// Cosine envelope.
    Cosine,

    /// Sine envelope.
    Sine,

    /// User-defined named envelope.
    ///
    /// The name is semantic metadata, not a provider API identifier.
    Custom(String),
}

impl Default for PulseEnvelope {
    fn default() -> Self {
        Self::Constant
    }
}

impl PulseEnvelope {
    /// Validates the envelope.
    pub fn validate(&self) -> PulseResult<()> {
        match self {
            Self::Constant | Self::Cosine | Self::Sine => Ok(()),

            Self::Gaussian { sigma } => sigma
                .validate()
                .map_err(|error| PulseError::Parameter(error.to_string())),

            Self::Drag { sigma, beta } => {
                sigma
                    .validate()
                    .map_err(|error| {
                        PulseError::Parameter(error.to_string())
                    })?;

                beta
                    .validate()
                    .map_err(|error| {
                        PulseError::Parameter(error.to_string())
                    })
            }

            Self::Custom(name) => {
                validate_name(
                    name,
                    "pulse envelope",
                    DEFAULT_MAX_METADATA_KEY_BYTES,
                )
            }
        }
    }
}

// =============================================================================
// Pulse amplitude limits
// =============================================================================

/// Optional validation limits for pulse amplitude.
///
/// These limits are not intrinsic to the universal IR because physical
/// amplitude semantics differ between technologies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseAmplitudeLimits {
    /// Minimum allowed concrete amplitude.
    pub minimum: f64,

    /// Maximum allowed concrete amplitude.
    pub maximum: f64,
}

impl PulseAmplitudeLimits {
    /// Creates amplitude limits.
    pub fn new(minimum: f64, maximum: f64) -> PulseResult<Self> {
        if !minimum.is_finite() || !maximum.is_finite() {
            return Err(PulseError::InvalidAmplitudeLimit);
        }

        if minimum > maximum {
            return Err(PulseError::InvalidAmplitudeRange);
        }

        Ok(Self { minimum, maximum })
    }

    /// Validates one concrete amplitude.
    pub fn validate(&self, value: f64) -> PulseResult<()> {
        if !value.is_finite() {
            return Err(PulseError::NonFiniteAmplitude { value });
        }

        if value < self.minimum || value > self.maximum {
            return Err(PulseError::AmplitudeOutOfRange {
                value,
                minimum: self.minimum,
                maximum: self.maximum,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Pulse metadata
// =============================================================================

/// Metadata attached to one pulse.
///
/// Metadata is deterministic and bounded when constructed through
/// [`PulseMetadata::insert`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PulseMetadata {
    entries: BTreeMap<String, String>,
}

impl PulseMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a metadata entry using default policy limits.
    pub fn insert(
        &mut self,
        key: String,
        value: String,
    ) -> PulseResult<()> {
        self.insert_with_limits(
            key,
            value,
            DEFAULT_MAX_METADATA_KEY_BYTES,
            DEFAULT_MAX_METADATA_VALUE_BYTES,
            DEFAULT_MAX_METADATA_FIELDS,
        )
    }

    /// Inserts a metadata entry with explicit limits.
    pub fn insert_with_limits(
        &mut self,
        key: String,
        value: String,
        maximum_key_bytes: usize,
        maximum_value_bytes: usize,
        maximum_fields: usize,
    ) -> PulseResult<()> {
        if key.is_empty() {
            return Err(PulseError::EmptyMetadataKey);
        }

        if key.len() > maximum_key_bytes {
            return Err(PulseError::MetadataKeyTooLarge {
                actual: key.len(),
                maximum: maximum_key_bytes,
            });
        }

        if value.len() > maximum_value_bytes {
            return Err(PulseError::MetadataValueTooLarge {
                actual: value.len(),
                maximum: maximum_value_bytes,
            });
        }

        if !self.entries.contains_key(&key)
            && self.entries.len() >= maximum_fields
        {
            return Err(PulseError::MetadataFieldLimitExceeded {
                maximum: maximum_fields,
            });
        }

        self.entries.insert(key, value);

        Ok(())
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns metadata fields in deterministic lexical order.
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
}

// =============================================================================
// Pulse definition
// =============================================================================

/// Canonical semantic pulse definition.
///
/// A pulse describes a requested quantum-control event without describing the
/// hardware that eventually realizes it.
///
/// A pulse can contain:
///
/// - one or more logical targets;
/// - amplitude;
/// - duration;
/// - optional phase;
/// - optional frequency;
/// - optional waveform;
/// - optional abstract channel;
/// - optional frame;
/// - optional calibration reference;
/// - envelope;
/// - metadata.
///
/// The actual physical implementation remains downstream.
#[derive(Debug, Clone, PartialEq)]
pub struct Pulse {
    id: PulseId,
    targets: PulseTargets,
    amplitude: PulseAmplitude,
    duration: PulseDuration,
    phase: Option<PulsePhase>,
    frequency: Option<PulseFrequency>,
    waveform: Option<WaveformId>,
    channel: Option<ChannelId>,
    frame: Option<FrameId>,
    calibration: Option<CalibrationId>,
    envelope: PulseEnvelope,
    metadata: PulseMetadata,
}

impl Pulse {
    /// Creates a minimal pulse.
    ///
    /// A minimal pulse contains:
    ///
    /// - stable identity;
    /// - target;
    /// - amplitude;
    /// - non-zero duration.
    pub fn new(
        id: PulseId,
        target: QubitId,
        amplitude: PulseAmplitude,
        duration: PulseDuration,
    ) -> PulseResult<Self> {
        Self::builder(id)
            .target(target)
            .amplitude(amplitude)
            .duration(duration)
            .build()
    }

    /// Creates a builder for a pulse.
    #[must_use]
    pub fn builder(id: PulseId) -> PulseBuilder {
        PulseBuilder::new(id)
    }

    /// Returns the pulse identity.
    #[must_use]
    pub const fn id(&self) -> PulseId {
        self.id
    }

    /// Returns pulse targets.
    #[must_use]
    pub fn targets(&self) -> &PulseTargets {
        &self.targets
    }

    /// Returns the first target.
    #[must_use]
    pub fn target(&self) -> PulseTarget {
        self.targets
            .first()
            .expect("Pulse invariant violated: pulse must have a target")
    }

    /// Returns pulse amplitude.
    #[must_use]
    pub fn amplitude(&self) -> &PulseAmplitude {
        &self.amplitude
    }

    /// Returns pulse duration.
    #[must_use]
    pub const fn duration(&self) -> PulseDuration {
        self.duration
    }

    /// Returns optional phase.
    #[must_use]
    pub fn phase(&self) -> Option<&PulsePhase> {
        self.phase.as_ref()
    }

    /// Returns optional frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<&PulseFrequency> {
        self.frequency.as_ref()
    }

    /// Returns optional waveform reference.
    #[must_use]
    pub const fn waveform(&self) -> Option<WaveformId> {
        self.waveform
    }

    /// Returns optional abstract channel reference.
    #[must_use]
    pub const fn channel(&self) -> Option<ChannelId> {
        self.channel
    }

    /// Returns optional frame reference.
    #[must_use]
    pub const fn frame(&self) -> Option<FrameId> {
        self.frame
    }

    /// Returns optional calibration reference.
    #[must_use]
    pub const fn calibration(&self) -> Option<CalibrationId> {
        self.calibration
    }

    /// Returns the semantic envelope.
    #[must_use]
    pub fn envelope(&self) -> &PulseEnvelope {
        &self.envelope
    }

    /// Returns pulse metadata.
    #[must_use]
    pub fn metadata(&self) -> &PulseMetadata {
        &self.metadata
    }

    /// Returns whether the pulse uses a symbolic amplitude.
    #[must_use]
    pub fn has_symbolic_amplitude(&self) -> bool {
        self.amplitude.is_symbolic()
    }

    /// Returns whether the pulse uses a symbolic phase.
    #[must_use]
    pub fn has_symbolic_phase(&self) -> bool {
        self.phase
            .as_ref()
            .is_some_and(PulsePhase::is_symbolic)
    }

    /// Returns whether the pulse uses a symbolic frequency.
    #[must_use]
    pub fn has_symbolic_frequency(&self) -> bool {
        self.frequency
            .as_ref()
            .is_some_and(PulseFrequency::is_symbolic)
    }

    /// Validates using default pulse policy.
    pub fn validate(&self) -> PulseResult<()> {
        self.validate_with_policy(&PulseValidationPolicy::default())
    }

    /// Validates using an explicit pulse policy.
    pub fn validate_with_policy(
        &self,
        policy: &PulseValidationPolicy,
    ) -> PulseResult<()> {
        policy.validate()?;

        if self.duration.is_zero() {
            return Err(PulseError::ZeroDuration);
        }

        self.targets
            .validate_with_max_targets(policy.max_targets)?;

        self.amplitude.validate()?;

        if let Some(phase) = &self.phase {
            phase.validate()?;
        }

        if let Some(frequency) = &self.frequency {
            frequency.validate()?;
        }

        self.envelope.validate()?;

        if let Some(limits) = policy.amplitude_limits {
            if let Some(value) = self.amplitude.as_constant() {
                limits.validate(value)?;
            }
        }

        Ok(())
    }

    /// Returns the total number of logical qubit resources touched by the
    /// pulse.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.targets.len()
    }
}

// =============================================================================
// Pulse builder
// =============================================================================

/// Builder for canonical pulse definitions.
///
/// The builder keeps construction explicit and prevents partially initialized
/// pulse values from entering the IR.
#[derive(Debug, Clone)]
pub struct PulseBuilder {
    id: PulseId,
    targets: Vec<PulseTarget>,
    amplitude: Option<PulseAmplitude>,
    duration: Option<PulseDuration>,
    phase: Option<PulsePhase>,
    frequency: Option<PulseFrequency>,
    waveform: Option<WaveformId>,
    channel: Option<ChannelId>,
    frame: Option<FrameId>,
    calibration: Option<CalibrationId>,
    envelope: PulseEnvelope,
    metadata: PulseMetadata,
}

impl PulseBuilder {
    /// Creates a builder.
    #[must_use]
    pub fn new(id: PulseId) -> Self {
        Self {
            id,
            targets: Vec::new(),
            amplitude: None,
            duration: None,
            phase: None,
            frequency: None,
            waveform: None,
            channel: None,
            frame: None,
            calibration: None,
            envelope: PulseEnvelope::Constant,
            metadata: PulseMetadata::new(),
        }
    }

    /// Adds one logical-qubit target.
    #[must_use]
    pub fn target(mut self, target: QubitId) -> Self {
        self.targets.push(PulseTarget::Qubit(target));
        self
    }

    /// Adds multiple targets.
    pub fn targets(
        mut self,
        targets: impl IntoIterator<Item = QubitId>,
    ) -> Self {
        self.targets.extend(
            targets
                .into_iter()
                .map(PulseTarget::Qubit),
        );
        self
    }

    /// Sets amplitude.
    #[must_use]
    pub fn amplitude(
        mut self,
        amplitude: PulseAmplitude,
    ) -> Self {
        self.amplitude = Some(amplitude);
        self
    }

    /// Sets a concrete amplitude.
    pub fn amplitude_value(
        self,
        value: f64,
    ) -> PulseResult<Self> {
        Ok(self.amplitude(PulseAmplitude::constant(value)?))
    }

    /// Sets duration.
    #[must_use]
    pub fn duration(
        mut self,
        duration: PulseDuration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets a duration in nanoseconds.
    pub fn duration_nanoseconds(
        self,
        value: u64,
    ) -> PulseResult<Self> {
        Ok(self.duration(
            PulseDuration::from_nanoseconds(value)?,
        ))
    }

    /// Sets a duration in picoseconds.
    pub fn duration_picoseconds(
        self,
        value: u64,
    ) -> PulseResult<Self> {
        Ok(self.duration(
            PulseDuration::from_picoseconds(value)?,
        ))
    }

    /// Sets a duration in femtoseconds.
    #[must_use]
    pub fn duration_femtoseconds(
        self,
        value: u64,
    ) -> Self {
        self.duration(
            PulseDuration::from_femtoseconds(value),
        )
    }

    /// Sets phase.
    #[must_use]
    pub fn phase(
        mut self,
        phase: PulsePhase,
    ) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Sets a concrete phase in radians.
    pub fn phase_radians(
        self,
        value: f64,
    ) -> PulseResult<Self> {
        Ok(self.phase(PulsePhase::radians(value)?))
    }

    /// Sets frequency.
    #[must_use]
    pub fn frequency(
        mut self,
        frequency: PulseFrequency,
    ) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Sets a concrete frequency in hertz.
    pub fn frequency_hertz(
        self,
        value: f64,
    ) -> PulseResult<Self> {
        Ok(self.frequency(
            PulseFrequency::hertz(value)?,
        ))
    }

    /// References a canonical waveform.
    #[must_use]
    pub const fn waveform(
        mut self,
        waveform: WaveformId,
    ) -> Self {
        self.waveform = Some(waveform);
        self
    }

    /// References an abstract channel.
    #[must_use]
    pub const fn channel(
        mut self,
        channel: ChannelId,
    ) -> Self {
        self.channel = Some(channel);
        self
    }

    /// References a semantic control frame.
    #[must_use]
    pub const fn frame(
        mut self,
        frame: FrameId,
    ) -> Self {
        self.frame = Some(frame);
        self
    }

    /// References a calibration record.
    ///
    /// The calibration itself is not stored in the pulse.
    #[must_use]
    pub const fn calibration(
        mut self,
        calibration: CalibrationId,
    ) -> Self {
        self.calibration = Some(calibration);
        self
    }

    /// Sets a semantic envelope.
    #[must_use]
    pub fn envelope(
        mut self,
        envelope: PulseEnvelope,
    ) -> Self {
        self.envelope = envelope;
        self
    }

    /// Adds metadata.
    pub fn metadata(
        mut self,
        key: String,
        value: String,
    ) -> PulseResult<Self> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Builds and locally validates the pulse.
    pub fn build(self) -> PulseResult<Pulse> {
        let targets = PulseTargets::new(self.targets)?;

        let amplitude = self
            .amplitude
            .ok_or(PulseError::MissingAmplitude)?;

        let duration = self
            .duration
            .ok_or(PulseError::MissingDuration)?;

        let pulse = Pulse {
            id: self.id,
            targets,
            amplitude,
            duration,
            phase: self.phase,
            frequency: self.frequency,
            waveform: self.waveform,
            channel: self.channel,
            frame: self.frame,
            calibration: self.calibration,
            envelope: self.envelope,
            metadata: self.metadata,
        };

        pulse.validate()?;

        Ok(pulse)
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit validation policy for pulse objects.
///
/// Policy is separate from semantic pulse representation.
///
/// This prevents a local default such as 4096 targets from becoming a
/// universal quantum-machine limit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseValidationPolicy {
    /// Maximum number of logical targets allowed in one pulse.
    pub max_targets: usize,

    /// Optional concrete amplitude limits.
    pub amplitude_limits: Option<PulseAmplitudeLimits>,
}

impl Default for PulseValidationPolicy {
    fn default() -> Self {
        Self {
            max_targets: DEFAULT_MAX_TARGETS,
            amplitude_limits: None,
        }
    }
}

impl PulseValidationPolicy {
    /// Creates an explicit policy.
    #[must_use]
    pub const fn new(
        max_targets: usize,
        amplitude_limits: Option<PulseAmplitudeLimits>,
    ) -> Self {
        Self {
            max_targets,
            amplitude_limits,
        }
    }

    /// Validates the policy itself.
    pub const fn validate(self) -> PulseResult<()> {
        if self.max_targets == 0 {
            return Err(PulseError::ZeroTargetLimit);
        }

        Ok(())
    }
}

// =============================================================================
// Pulse sequence
// =============================================================================

/// Deterministic collection of pulse definitions.
///
/// A `PulseSequence` is an ordered semantic sequence.
///
/// It does NOT imply a physical execution schedule. Scheduling belongs to
/// `schedule.rs` / `scheduling/`.
#[derive(Debug, Clone, PartialEq)]
pub struct PulseSequence {
    pulses: Vec<Pulse>,
}

impl PulseSequence {
    /// Creates an empty sequence.
    #[must_use]
    pub fn new() -> Self {
        Self { pulses: Vec::new() }
    }

    /// Creates a sequence from pulses.
    ///
    /// Pulse identities must be unique.
    pub fn from_pulses(pulses: Vec<Pulse>) -> PulseResult<Self> {
        let sequence = Self { pulses };
        sequence.validate()?;
        Ok(sequence)
    }

    /// Appends a pulse after validating identity uniqueness.
    pub fn push(&mut self, pulse: Pulse) -> PulseResult<()> {
        if self
            .pulses
            .iter()
            .any(|existing| existing.id() == pulse.id())
        {
            return Err(PulseError::DuplicatePulseId {
                id: pulse.id(),
            });
        }

        pulse.validate()?;
        self.pulses.push(pulse);

        Ok(())
    }

    /// Returns the number of pulses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pulses.len()
    }

    /// Returns whether the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pulses.is_empty()
    }

    /// Returns pulses in explicit semantic order.
    #[must_use]
    pub fn as_slice(&self) -> &[Pulse] {
        &self.pulses
    }

    /// Returns a pulse by identity.
    #[must_use]
    pub fn get(&self, id: PulseId) -> Option<&Pulse> {
        self.pulses.iter().find(|pulse| pulse.id() == id)
    }

    /// Validates all pulses and identity uniqueness.
    pub fn validate(&self) -> PulseResult<()> {
        let mut identities = std::collections::BTreeSet::new();

        for pulse in &self.pulses {
            pulse.validate()?;

            if !identities.insert(pulse.id()) {
                return Err(PulseError::DuplicatePulseId {
                    id: pulse.id(),
                });
            }
        }

        Ok(())
    }
}

impl Default for PulseSequence {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Pulse error
// =============================================================================

/// Pulse-local validation and construction errors.
#[derive(Debug, Clone, PartialEq)]
pub enum PulseError {
    /// Pulse duration arithmetic overflowed.
    DurationOverflow,

    /// A duration subtraction would become negative.
    NegativeDuration,

    /// A pulse has zero duration.
    ZeroDuration,

    /// No target was supplied.
    EmptyTargets,

    /// A target appears more than once.
    DuplicateTarget {
        /// Duplicate logical qubit.
        qubit: QubitId,
    },

    /// Target count exceeds an explicit policy.
    TargetLimitExceeded {
        /// Actual number of targets.
        actual: usize,

        /// Policy maximum.
        maximum: usize,
    },

    /// A pulse has no amplitude.
    MissingAmplitude,

    /// A pulse has no duration.
    MissingDuration,

    /// The target limit policy is zero.
    ZeroTargetLimit,

    /// A parameter failed validation.
    Parameter(String),

    /// A concrete amplitude is not finite.
    NonFiniteAmplitude {
        /// Invalid amplitude.
        value: f64,
    },

    /// Amplitude limits themselves are invalid.
    InvalidAmplitudeLimit,

    /// Minimum amplitude is greater than maximum amplitude.
    InvalidAmplitudeRange,

    /// Concrete amplitude is outside configured limits.
    AmplitudeOutOfRange {
        /// Actual amplitude.
        value: f64,

        /// Minimum allowed amplitude.
        minimum: f64,

        /// Maximum allowed amplitude.
        maximum: f64,
    },

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata key exceeds its policy.
    MetadataKeyTooLarge {
        /// Actual UTF-8 byte length.
        actual: usize,

        /// Maximum permitted byte length.
        maximum: usize,
    },

    /// Metadata value exceeds its policy.
    MetadataValueTooLarge {
        /// Actual UTF-8 byte length.
        actual: usize,

        /// Maximum permitted byte length.
        maximum: usize,
    },

    /// Metadata field count exceeds its policy.
    MetadataFieldLimitExceeded {
        /// Maximum permitted fields.
        maximum: usize,
    },

    /// A pulse envelope/name is invalid.
    InvalidName {
        /// Semantic field name.
        field: &'static str,

        /// Reason.
        reason: &'static str,
    },

    /// Two pulses have the same identity.
    DuplicatePulseId {
        /// Duplicate identity.
        id: PulseId,
    },

    /// A generic pulse invariant failed.
    InvalidStructure {
        /// Static diagnostic.
        message: &'static str,
    },
}

impl fmt::Display for PulseError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DurationOverflow => {
                formatter.write_str(
                    "pulse duration arithmetic overflowed",
                )
            }

            Self::NegativeDuration => {
                formatter.write_str(
                    "pulse duration cannot become negative",
                )
            }

            Self::ZeroDuration => {
                formatter.write_str(
                    "pulse duration must be greater than zero",
                )
            }

            Self::EmptyTargets => {
                formatter.write_str(
                    "pulse must contain at least one target",
                )
            }

            Self::DuplicateTarget { qubit } => {
                write!(
                    formatter,
                    "pulse contains duplicate target {qubit}"
                )
            }

            Self::TargetLimitExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "pulse target count {actual} exceeds policy maximum {maximum}"
                )
            }

            Self::MissingAmplitude => {
                formatter.write_str(
                    "pulse amplitude is required",
                )
            }

            Self::MissingDuration => {
                formatter.write_str(
                    "pulse duration is required",
                )
            }

            Self::ZeroTargetLimit => {
                formatter.write_str(
                    "pulse target limit cannot be zero",
                )
            }

            Self::Parameter(message) => {
                write!(
                    formatter,
                    "invalid pulse parameter: {message}"
                )
            }

            Self::NonFiniteAmplitude { value } => {
                write!(
                    formatter,
                    "pulse amplitude must be finite, received {value}"
                )
            }

            Self::InvalidAmplitudeLimit => {
                formatter.write_str(
                    "pulse amplitude limit must contain finite values",
                )
            }

            Self::InvalidAmplitudeRange => {
                formatter.write_str(
                    "pulse amplitude minimum cannot exceed maximum",
                )
            }

            Self::AmplitudeOutOfRange {
                value,
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "pulse amplitude {value} is outside [{minimum}, {maximum}]"
                )
            }

            Self::EmptyMetadataKey => {
                formatter.write_str(
                    "pulse metadata key cannot be empty",
                )
            }

            Self::MetadataKeyTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "pulse metadata key size {actual} exceeds maximum {maximum}"
                )
            }

            Self::MetadataValueTooLarge {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "pulse metadata value size {actual} exceeds maximum {maximum}"
                )
            }

            Self::MetadataFieldLimitExceeded {
                maximum,
            } => {
                write!(
                    formatter,
                    "pulse metadata field count exceeds maximum {maximum}"
                )
            }

            Self::InvalidName { field, reason } => {
                write!(
                    formatter,
                    "invalid {field}: {reason}"
                )
            }

            Self::DuplicatePulseId { id } => {
                write!(
                    formatter,
                    "duplicate pulse identity {id}"
                )
            }

            Self::InvalidStructure { message } => {
                write!(
                    formatter,
                    "invalid pulse structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for PulseError {}

// =============================================================================
// Utility validation
// =============================================================================

/// Validates a semantic textual name.
fn validate_name(
    value: &str,
    field: &'static str,
    maximum_bytes: usize,
) -> PulseResult<()> {
    if value.is_empty() {
        return Err(PulseError::InvalidName {
            field,
            reason: "name cannot be empty",
        });
    }

    if value.len() > maximum_bytes {
        return Err(PulseError::InvalidName {
            field,
            reason: "name exceeds configured byte limit",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PulseError::InvalidName {
            field,
            reason: "name contains a control character",
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
    use crate::quantum::ir::identity::{
        ChannelId,
        FrameId,
        PulseId,
        WaveformId,
    };
    use crate::quantum::ir::qubit::QubitId;

    #[test]
    fn duration_converts_nanoseconds_exactly() {
        let duration =
            PulseDuration::from_nanoseconds(20).unwrap();

        assert_eq!(
            duration.femtoseconds(),
            20_000_000
        );

        assert_eq!(
            duration.whole_nanoseconds(),
            Some(20)
        );
    }

    #[test]
    fn duration_supports_sub_nanosecond_values() {
        let duration =
            PulseDuration::from_femtoseconds(500);

        assert_eq!(duration.femtoseconds(), 500);
        assert_eq!(duration.whole_nanoseconds(), None);
    }

    #[test]
    fn duration_overflow_is_rejected() {
        let result =
            PulseDuration::from_seconds(u64::MAX);

        assert!(matches!(
            result,
            Err(PulseError::DurationOverflow)
        ));
    }

    #[test]
    fn duration_checked_addition_is_safe() {
        let a =
            PulseDuration::from_femtoseconds(u64::MAX - 1);

        let b =
            PulseDuration::from_femtoseconds(2);

        assert!(matches!(
            a.checked_add(b),
            Err(PulseError::DurationOverflow)
        ));
    }

    #[test]
    fn duration_checked_subtraction_is_safe() {
        let a =
            PulseDuration::from_femtoseconds(1);

        let b =
            PulseDuration::from_femtoseconds(2);

        assert!(matches!(
            a.checked_sub(b),
            Err(PulseError::NegativeDuration)
        ));
    }

    #[test]
    fn pulse_amplitude_accepts_finite_values() {
        let amplitude =
            PulseAmplitude::constant(0.3).unwrap();

        assert_eq!(
            amplitude.as_constant(),
            Some(0.3)
        );
    }

    #[test]
    fn pulse_amplitude_rejects_nan() {
        assert!(
            PulseAmplitude::constant(f64::NAN).is_err()
        );
    }

    #[test]
    fn pulse_amplitude_rejects_infinity() {
        assert!(
            PulseAmplitude::constant(f64::INFINITY).is_err()
        );
    }

    #[test]
    fn single_target_pulse_can_be_constructed() {
        let pulse =
            Pulse::new(
                PulseId::new(1),
                QubitId::new(0),
                PulseAmplitude::constant(0.3).unwrap(),
                PulseDuration::from_nanoseconds(20).unwrap(),
            )
            .unwrap();

        assert_eq!(
            pulse.id(),
            PulseId::new(1)
        );

        assert_eq!(
            pulse.target().qubit_id(),
            QubitId::new(0)
        );

        assert_eq!(
            pulse.duration().whole_nanoseconds(),
            Some(20)
        );

        assert_eq!(
            pulse.amplitude().as_constant(),
            Some(0.3)
        );
    }

    #[test]
    fn pulse_builder_supports_example_shape() {
        let pulse =
            Pulse::builder(PulseId::new(7))
                .target(QubitId::new(3))
                .amplitude_value(0.3)
                .unwrap()
                .duration_nanoseconds(20)
                .unwrap()
                .build()
                .unwrap();

        assert_eq!(
            pulse.target().qubit_id(),
            QubitId::new(3)
        );

        assert_eq!(
            pulse.amplitude().as_constant(),
            Some(0.3)
        );

        assert_eq!(
            pulse.duration().whole_nanoseconds(),
            Some(20)
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let result =
            PulseTargets::new(vec![
                PulseTarget::qubit(QubitId::new(1)),
                PulseTarget::qubit(QubitId::new(1)),
            ]);

        assert!(matches!(
            result,
            Err(PulseError::DuplicateTarget { .. })
        ));
    }

    #[test]
    fn empty_targets_are_rejected() {
        let result =
            PulseTargets::new(Vec::new());

        assert!(matches!(
            result,
            Err(PulseError::EmptyTargets)
        ));
    }

    #[test]
    fn zero_duration_is_rejected() {
        let result =
            Pulse::new(
                PulseId::new(1),
                QubitId::new(0),
                PulseAmplitude::constant(0.3).unwrap(),
                PulseDuration::ZERO,
            );

        assert!(matches!(
            result,
            Err(PulseError::ZeroDuration)
        ));
    }

    #[test]
    fn symbolic_amplitude_is_preserved() {
        let amplitude =
            PulseAmplitude::new(
                Parameter::symbol("drive_amplitude")
                    .unwrap(),
            );

        assert!(amplitude.is_symbolic());
        assert_eq!(
            amplitude.as_constant(),
            None
        );
    }

    #[test]
    fn symbolic_phase_is_preserved() {
        let phase =
            PulsePhase::new(
                Parameter::symbol("phase")
                    .unwrap(),
            );

        assert!(phase.is_symbolic());
        assert_eq!(
            phase.as_constant(),
            None
        );
    }

    #[test]
    fn symbolic_frequency_is_preserved() {
        let frequency =
            PulseFrequency::new(
                Parameter::symbol("drive_frequency")
                    .unwrap(),
            );

        assert!(frequency.is_symbolic());
        assert_eq!(
            frequency.as_constant(),
            None
        );
    }

    #[test]
    fn waveform_channel_and_frame_references_are_typed() {
        let pulse =
            Pulse::builder(PulseId::new(2))
                .target(QubitId::new(0))
                .amplitude_value(0.5)
                .unwrap()
                .duration_nanoseconds(10)
                .unwrap()
                .waveform(WaveformId::new(10))
                .channel(ChannelId::new(20))
                .frame(FrameId::new(30))
                .build()
                .unwrap();

        assert_eq!(
            pulse.waveform(),
            Some(WaveformId::new(10))
        );

        assert_eq!(
            pulse.channel(),
            Some(ChannelId::new(20))
        );

        assert_eq!(
            pulse.frame(),
            Some(FrameId::new(30))
        );
    }

    #[test]
    fn amplitude_limits_are_optional() {
        let pulse =
            Pulse::builder(PulseId::new(3))
                .target(QubitId::new(0))
                .amplitude_value(2.0)
                .unwrap()
                .duration_nanoseconds(20)
                .unwrap()
                .build()
                .unwrap();

        let limits =
            PulseAmplitudeLimits::new(-1.0, 1.0)
                .unwrap();

        assert!(matches!(
            pulse.validate_with_policy(
                &PulseValidationPolicy::new(
                    DEFAULT_MAX_TARGETS,
                    Some(limits),
                )
            ),
            Err(PulseError::AmplitudeOutOfRange { .. })
        ));
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata =
            PulseMetadata::new();

        metadata
            .insert(
                "z".to_owned(),
                "last".to_owned(),
            )
            .unwrap();

        metadata
            .insert(
                "a".to_owned(),
                "first".to_owned(),
            )
            .unwrap();

        let keys: Vec<&String> =
            metadata.entries().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"a".to_owned(),
                &"z".to_owned()
            ]
        );
    }

    #[test]
    fn pulse_sequence_rejects_duplicate_ids() {
        let first =
            Pulse::new(
                PulseId::new(1),
                QubitId::new(0),
                PulseAmplitude::constant(0.3).unwrap(),
                PulseDuration::from_nanoseconds(20).unwrap(),
            )
            .unwrap();

        let second =
            Pulse::new(
                PulseId::new(1),
                QubitId::new(1),
                PulseAmplitude::constant(0.4).unwrap(),
                PulseDuration::from_nanoseconds(20).unwrap(),
            )
            .unwrap();

        let result =
            PulseSequence::from_pulses(
                vec![first, second],
            );

        assert!(matches!(
            result,
            Err(PulseError::DuplicatePulseId { .. })
        ));
    }

    #[test]
    fn pulse_sequence_preserves_order() {
        let first =
            Pulse::new(
                PulseId::new(10),
                QubitId::new(0),
                PulseAmplitude::constant(0.1).unwrap(),
                PulseDuration::from_nanoseconds(5).unwrap(),
            )
            .unwrap();

        let second =
            Pulse::new(
                PulseId::new(20),
                QubitId::new(1),
                PulseAmplitude::constant(0.2).unwrap(),
                PulseDuration::from_nanoseconds(5).unwrap(),
            )
            .unwrap();

        let sequence =
            PulseSequence::from_pulses(
                vec![first, second],
            )
            .unwrap();

        assert_eq!(
            sequence.as_slice()[0].id(),
            PulseId::new(10)
        );

        assert_eq!(
            sequence.as_slice()[1].id(),
            PulseId::new(20)
        );
    }

    #[test]
    fn large_logical_qubit_ids_are_supported() {
        let large =
            QubitId::new(usize::MAX);

        let pulse =
            Pulse::new(
                PulseId::new(u64::MAX),
                large,
                PulseAmplitude::constant(0.3).unwrap(),
                PulseDuration::from_nanoseconds(20).unwrap(),
            )
            .unwrap();

        assert_eq!(
            pulse.target().qubit_id(),
            large
        );

        assert_eq!(
            pulse.id(),
            PulseId::new(u64::MAX)
        );
    }

    #[test]
    fn no_machine_size_limit_is_encoded_in_pulse_model() {
        let ids = [
            QubitId::new(0),
            QubitId::new(63),
            QubitId::new(64),
            QubitId::new(4_096),
            QubitId::new(1_000_000),
        ];

        for id in ids {
            let pulse =
                Pulse::new(
                    PulseId::new(
                        id.index() as u64
                    ),
                    id,
                    PulseAmplitude::constant(0.3)
                        .unwrap(),
                    PulseDuration::from_nanoseconds(20)
                        .unwrap(),
                )
                .unwrap();

            assert_eq!(
                pulse.target().qubit_id(),
                id
            );
        }
    }

    #[test]
    fn envelope_parameters_are_validated() {
        let envelope =
            PulseEnvelope::Gaussian {
                sigma: Parameter::symbol(
                    "sigma"
                )
                .unwrap(),
            };

        assert!(envelope.validate().is_ok());
    }

    #[test]
    fn invalid_custom_envelope_name_is_rejected() {
        let envelope =
            PulseEnvelope::Custom(String::new());

        assert!(matches!(
            envelope.validate(),
            Err(PulseError::InvalidName { .. })
        ));
    }
}