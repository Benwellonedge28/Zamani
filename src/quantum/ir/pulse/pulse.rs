//! Zamani Quantum IR — Canonical Pulse Semantics
//!
//! `quantum::ir::pulse::pulse` defines the target-independent semantic model
//! for pulse-level quantum control.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This module answers:
//!
//!     "What pulse/control operation does the program mean?"
//!
//! It does NOT answer:
//!
//!     "Which physical device, DAC, ADC, oscillator, laser, control line,
//!      sample clock, or provider implements it?"
//!
//! Hardware realization belongs to downstream compilation layers.
//!
//! Dependency direction:
//!
//!     Zamani source
//!          |
//!          v
//!     frontend
//!          |
//!          v
//!     canonical Zamani IR
//!          |
//!          v
//!     pulse::pulse
//!          |
//!          +----------------------+
//!          |                      |
//!          v                      v
//!      waveform                 frame/channel
//!          |                      |
//!          +----------+-----------+
//!                     |
//!                     v
//!                optimization
//!                     |
//!                     v
//!                 scheduling
//!                     |
//!                     v
//!                   mapping
//!                     |
//!                     v
//!                  hardware
//!                     |
//!                     v
//!                   backend
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - semantic pulse identity;
//! - pulse operation kind;
//! - logical pulse targets;
//! - pulse duration;
//! - pulse amplitude;
//! - pulse phase;
//! - pulse frequency;
//! - waveform references;
//! - abstract channel references;
//! - abstract frame references;
//! - calibration references;
//! - pulse metadata;
//! - pulse dependencies;
//! - pulse composition;
//! - pulse-local validation;
//! - explicit validation policy;
//! - deterministic structural accessors.
//!
//! This file does NOT own:
//!
//! - waveform definitions;
//! - physical channels;
//! - hardware topology;
//! - routing;
//! - scheduling algorithms;
//! - hardware calibration data;
//! - DAC/ADC implementation;
//! - provider SDKs;
//! - QPU communication;
//! - simulation state;
//! - optimization algorithms;
//! - frontend syntax.
//!
//! ============================================================================
//! SCALABILITY CONTRACT
//! ============================================================================
//!
//! There is deliberately NO semantic maximum for:
//!
//! - number of qubits;
//! - number of pulse operations;
//! - number of targets;
//! - number of pulse sequences;
//! - number of pulse dependencies;
//! - number of metadata fields;
//! - number of waveform references;
//! - number of channels;
//! - number of frames.
//!
//! Resource limits are represented by `PulseValidationPolicy`.
//!
//! The default policy is intentionally permissive: no artificial count limit
//! is imposed by this semantic model.
//!
//! A compiler, service, sandbox, or embedded target may provide an explicit
//! policy before accepting untrusted or enormous input.
//!
//! This distinction is essential:
//!
//!     semantic capability != resource policy
//!
//! ============================================================================
//! QUANTUM IDENTITY CONTRACT
//! ============================================================================
//!
//! Pulse targets use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Logical qubits are therefore never represented by raw integers inside a
//! pulse target.
//!
//! A physical qubit must NOT be substituted for a logical qubit here.
//!
//! Logical -> physical mapping belongs to `mapping`.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no external dependencies
//! - no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::identity::{
    CalibrationId,
    ChannelId,
    ExtensionId,
    FrameId,
    PulseId,
    ResourceId,
    WaveformId,
};

use super::super::core::parameter::Parameter;
use super::super::quantum::qubit::QubitId;

// ============================================================================
// Schema
// ============================================================================

/// Stable semantic schema identifier.
pub const PULSE_SCHEMA_ID: &str = "zamani.quantum.ir.pulse";

/// Semantic schema major version.
///
/// Breaking semantic changes require a new major version.
pub const PULSE_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const PULSE_SCHEMA_MINOR: u16 = 0;

/// Semantic schema patch version.
pub const PULSE_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// Result
// ============================================================================

/// Result type used by pulse-local operations.
pub type PulseResult<T> = Result<T, PulseError>;

// ============================================================================
// Duration
// ============================================================================

/// Exact non-negative pulse duration in femtoseconds.
///
/// The representation is integer based so pulse boundaries remain
/// deterministic and free from floating-point accumulation error.
///
/// This is a semantic time quantity, not a hardware clock period.
///
/// Conversion to target-specific `dt` belongs to the timing/hardware layer.
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
pub struct PulseDuration(u128);

impl PulseDuration {
    /// Femtoseconds in one picosecond.
    pub const FEMTOSECONDS_PER_PICOSECOND: u128 = 1_000;

    /// Femtoseconds in one nanosecond.
    pub const FEMTOSECONDS_PER_NANOSECOND: u128 = 1_000_000;

    /// Femtoseconds in one microsecond.
    pub const FEMTOSECONDS_PER_MICROSECOND: u128 = 1_000_000_000;

    /// Femtoseconds in one millisecond.
    pub const FEMTOSECONDS_PER_MILLISECOND: u128 =
        1_000_000_000_000;

    /// Femtoseconds in one second.
    pub const FEMTOSECONDS_PER_SECOND: u128 =
        1_000_000_000_000_000;

    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a duration directly from femtoseconds.
    #[must_use]
    pub const fn from_femtoseconds(value: u128) -> Self {
        Self(value)
    }

    /// Creates a duration from whole picoseconds.
    pub const fn from_picoseconds(
        value: u128,
    ) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_PICOSECOND) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole nanoseconds.
    pub const fn from_nanoseconds(
        value: u128,
    ) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_NANOSECOND) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole microseconds.
    pub const fn from_microseconds(
        value: u128,
    ) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_MICROSECOND) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole milliseconds.
    pub const fn from_milliseconds(
        value: u128,
    ) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_MILLISECOND) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Creates a duration from whole seconds.
    pub const fn from_seconds(
        value: u128,
    ) -> PulseResult<Self> {
        match value.checked_mul(Self::FEMTOSECONDS_PER_SECOND) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Returns femtoseconds.
    #[must_use]
    pub const fn femtoseconds(self) -> u128 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Checked addition.
    pub const fn checked_add(
        self,
        other: Self,
    ) -> PulseResult<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(
        self,
        other: Self,
    ) -> PulseResult<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::NegativeDuration),
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(
        self,
        multiplier: u128,
    ) -> PulseResult<Self> {
        match self.0.checked_mul(multiplier) {
            Some(value) => Ok(Self(value)),
            None => Err(PulseError::DurationOverflow),
        }
    }
}

impl fmt::Display for PulseDuration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}fs",
            self.0
        )
    }
}

// ============================================================================
// Pulse operation kind
// ============================================================================

/// Semantic kind of a pulse-level operation.
///
/// This enum describes intent rather than hardware implementation.
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
pub enum PulseKind {
    /// Play a waveform/control envelope.
    Play,

    /// Capture an analog/control signal.
    Capture,

    /// Acquire a measurement signal.
    Acquire,

    /// Wait without producing a control waveform.
    Delay,

    /// Set a frame frequency.
    SetFrequency,

    /// Set a frame phase.
    SetPhase,

    /// Add a phase shift to the current frame phase.
    ShiftPhase,

    /// Synchronization/barrier event.
    Barrier,

    /// Backend-independent calibration invocation.
    Calibration,

    /// Extension-defined pulse operation.
    Extension(ExtensionId),
}

impl PulseKind {
    /// Returns whether this operation normally carries a waveform.
    #[must_use]
    pub const fn uses_waveform(self) -> bool {
        matches!(self, Self::Play | Self::Capture | Self::Acquire)
    }

    /// Returns whether this operation modifies frame state.
    #[must_use]
    pub const fn modifies_frame(self) -> bool {
        matches!(
            self,
            Self::SetFrequency
                | Self::SetPhase
                | Self::ShiftPhase
        )
    }

    /// Returns whether this operation is timing-only.
    #[must_use]
    pub const fn is_timing_only(self) -> bool {
        matches!(self, Self::Delay | Self::Barrier)
    }
}

// ============================================================================
// Target
// ============================================================================

/// Semantic pulse target.
///
/// The target remains logical and hardware-independent.
///
/// `Global` means the operation intentionally has no finite explicit qubit
/// target set. The target resolver must interpret its scope later.
///
/// `Resource` permits semantic pulse resources that are not represented as
/// qubits, such as a global control resource or an abstract mode.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum PulseTarget {
    /// One logical qubit.
    Qubit(QubitId),

    /// An explicit ordered set of logical qubits.
    ///
    /// Construction canonicalizes duplicates away while preserving sorted
    /// deterministic ordering.
    Qubits(Vec<QubitId>),

    /// A global operation whose scope is resolved by a later compilation
    /// stage.
    Global,

    /// An abstract resource target.
    Resource(ResourceId),
}

impl PulseTarget {
    /// Creates a single-qubit target.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a global target.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates a deterministic multi-qubit target.
    ///
    /// Duplicates are removed and identifiers are sorted.
    #[must_use]
    pub fn qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut set = BTreeSet::new();

        for qubit in qubits {
            set.insert(qubit);
        }

        Self::Qubits(set.into_iter().collect())
    }

    /// Returns whether this is a single-qubit target.
    #[must_use]
    pub const fn is_single_qubit(&self) -> bool {
        matches!(self, Self::Qubit(_))
    }

    /// Returns whether this is a global target.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the number of explicitly represented logical qubits.
    ///
    /// Global and resource targets return zero because they do not encode
    /// explicit logical-qubit cardinality.
    #[must_use]
    pub fn explicit_qubit_count(&self) -> usize {
        match self {
            Self::Qubit(_) => 1,
            Self::Qubits(qubits) => qubits.len(),
            Self::Global | Self::Resource(_) => 0,
        }
    }

    /// Returns the logical qubits represented by this target.
    ///
    /// Global/resource targets return an empty slice.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        match self {
            Self::Qubit(qubit) => {
                std::slice::from_ref(qubit)
            }

            Self::Qubits(qubits) => qubits.as_slice(),

            Self::Global | Self::Resource(_) => &[],
        }
    }

    /// Validates the target structure.
    pub fn validate(&self) -> PulseResult<()> {
        match self {
            Self::Qubit(_) => Ok(()),

            Self::Qubits(qubits) => {
                if qubits.is_empty() {
                    return Err(
                        PulseError::EmptyTargetSet
                    );
                }

                if qubits.windows(2).any(|pair| pair[0] >= pair[1]) {
                    return Err(
                        PulseError::NonCanonicalTargetSet
                    );
                }

                Ok(())
            }

            Self::Global | Self::Resource(_) => Ok(()),
        }
    }
}

// ============================================================================
// Pulse references
// ============================================================================

/// References to the optional semantic resources used by a pulse.
///
/// References do not imply physical allocation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PulseResources {
    waveform: Option<WaveformId>,
    channel: Option<ChannelId>,
    frame: Option<FrameId>,
    calibration: Option<CalibrationId>,
}

impl PulseResources {
    /// Creates an empty resource reference set.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            waveform: None,
            channel: None,
            frame: None,
            calibration: None,
        }
    }

    /// Sets a waveform reference.
    #[must_use]
    pub const fn with_waveform(
        mut self,
        waveform: WaveformId,
    ) -> Self {
        self.waveform = Some(waveform);
        self
    }

    /// Sets a channel reference.
    #[must_use]
    pub const fn with_channel(
        mut self,
        channel: ChannelId,
    ) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Sets a frame reference.
    #[must_use]
    pub const fn with_frame(
        mut self,
        frame: FrameId,
    ) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Sets a calibration reference.
    #[must_use]
    pub const fn with_calibration(
        mut self,
        calibration: CalibrationId,
    ) -> Self {
        self.calibration = Some(calibration);
        self
    }

    /// Returns waveform reference.
    #[must_use]
    pub const fn waveform(&self) -> Option<WaveformId> {
        self.waveform
    }

    /// Returns channel reference.
    #[must_use]
    pub const fn channel(&self) -> Option<ChannelId> {
        self.channel
    }

    /// Returns frame reference.
    #[must_use]
    pub const fn frame(&self) -> Option<FrameId> {
        self.frame
    }

    /// Returns calibration reference.
    #[must_use]
    pub const fn calibration(&self) -> Option<CalibrationId> {
        self.calibration
    }

    /// Returns whether no references are present.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.waveform.is_none()
            && self.channel.is_none()
            && self.frame.is_none()
            && self.calibration.is_none()
    }
}

impl Default for PulseResources {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// Pulse dependency
// ============================================================================

/// Explicit semantic dependency between pulse operations.
///
/// This does not schedule operations. It records a relationship that a
/// scheduler must preserve.
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
pub enum PulseDependency {
    /// `predecessor` must complete before this pulse.
    After(PulseId),

    /// This pulse must complete before `successor`.
    Before(PulseId),

    /// This pulse and another pulse must not overlap.
    NoOverlap(PulseId),
}

// ============================================================================
// Pulse metadata
// ============================================================================

/// Deterministic pulse metadata.
///
/// Metadata is descriptive and must never be treated as an authentication,
/// authorization, calibration, or hardware capability mechanism.
pub type PulseMetadata = BTreeMap<String, String>;

// ============================================================================
// Validation policy
// ============================================================================

/// Explicit validation/resource policy for pulse construction.
///
/// `None` means that this policy does not impose a local limit.
///
/// This is deliberately separate from semantic pulse representation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct PulseValidationPolicy {
    /// Optional maximum number of explicit targets.
    pub max_targets: Option<usize>,

    /// Optional maximum metadata field count.
    pub max_metadata_fields: Option<usize>,

    /// Optional maximum metadata key size in bytes.
    pub max_metadata_key_bytes: Option<usize>,

    /// Optional maximum metadata value size in bytes.
    pub max_metadata_value_bytes: Option<usize>,
}

impl Default for PulseValidationPolicy {
    fn default() -> Self {
        Self {
            max_targets: None,
            max_metadata_fields: None,
            max_metadata_key_bytes: None,
            max_metadata_value_bytes: None,
        }
    }
}

impl PulseValidationPolicy {
    /// Creates a completely unrestricted semantic validation policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_targets: None,
            max_metadata_fields: None,
            max_metadata_key_bytes: None,
            max_metadata_value_bytes: None,
        }
    }

    /// Creates a policy with an explicit target limit.
    #[must_use]
    pub const fn with_max_targets(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_targets = Some(maximum);
        self
    }

    /// Creates a policy with an explicit metadata-field limit.
    #[must_use]
    pub const fn with_max_metadata_fields(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_metadata_fields = Some(maximum);
        self
    }

    /// Creates a policy with an explicit metadata-key limit.
    #[must_use]
    pub const fn with_max_metadata_key_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_metadata_key_bytes = Some(maximum);
        self
    }

    /// Creates a policy with an explicit metadata-value limit.
    #[must_use]
    pub const fn with_max_metadata_value_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_metadata_value_bytes = Some(maximum);
        self
    }
}

// ============================================================================
// Pulse
// ============================================================================

/// Canonical semantic pulse operation.
///
/// A `Pulse` represents one control/acquisition event.
///
/// It is intentionally immutable after construction from the public API.
/// Transformations should create a new value, preserving the original pulse
/// for provenance and reproducibility.
#[derive(
    Debug,
    Clone,
    PartialEq,
)]
pub struct Pulse {
    id: PulseId,
    kind: PulseKind,
    targets: Vec<PulseTarget>,
    duration: Option<PulseDuration>,

    amplitude: Option<Parameter>,
    phase: Option<Parameter>,
    frequency: Option<Parameter>,

    resources: PulseResources,

    dependencies: Vec<PulseDependency>,

    metadata: PulseMetadata,
}

impl Pulse {
    /// Creates a new semantic pulse.
    pub fn new(
        id: PulseId,
        kind: PulseKind,
    ) -> Self {
        Self {
            id,
            kind,
            targets: Vec::new(),
            duration: None,
            amplitude: None,
            phase: None,
            frequency: None,
            resources: PulseResources::empty(),
            dependencies: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the pulse identity.
    #[must_use]
    pub const fn id(&self) -> PulseId {
        self.id
    }

    /// Returns the semantic pulse kind.
    #[must_use]
    pub const fn kind(&self) -> PulseKind {
        self.kind
    }

    /// Returns the targets.
    #[must_use]
    pub fn targets(&self) -> &[PulseTarget] {
        &self.targets
    }

    /// Returns duration.
    #[must_use]
    pub const fn duration(&self) -> Option<PulseDuration> {
        self.duration
    }

    /// Returns amplitude.
    #[must_use]
    pub fn amplitude(&self) -> Option<&Parameter> {
        self.amplitude.as_ref()
    }

    /// Returns phase.
    #[must_use]
    pub fn phase(&self) -> Option<&Parameter> {
        self.phase.as_ref()
    }

    /// Returns frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<&Parameter> {
        self.frequency.as_ref()
    }

    /// Returns resource references.
    #[must_use]
    pub const fn resources(&self) -> &PulseResources {
        &self.resources
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[PulseDependency] {
        &self.dependencies
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &PulseMetadata {
        &self.metadata
    }

    /// Adds one pulse target.
    ///
    /// Duplicate targets are rejected instead of silently changing program
    /// semantics.
    pub fn with_target(
        mut self,
        target: PulseTarget,
    ) -> PulseResult<Self> {
        target.validate()?;

        if self.targets.contains(&target) {
            return Err(
                PulseError::DuplicateTarget
            );
        }

        self.targets.push(target);
        Ok(self)
    }

    /// Replaces the complete target list.
    ///
    /// Targets are deterministically ordered.
    pub fn with_targets<I>(
        mut self,
        targets: I,
    ) -> PulseResult<Self>
    where
        I: IntoIterator<Item = PulseTarget>,
    {
        let mut targets: Vec<PulseTarget> =
            targets.into_iter().collect();

        for target in &targets {
            target.validate()?;
        }

        targets.sort();

        for pair in targets.windows(2) {
            if pair[0] == pair[1] {
                return Err(
                    PulseError::DuplicateTarget
                );
            }
        }

        self.targets = targets;
        Ok(self)
    }

    /// Sets the duration.
    #[must_use]
    pub const fn with_duration(
        mut self,
        duration: PulseDuration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets amplitude.
    pub fn with_amplitude(
        mut self,
        amplitude: Parameter,
    ) -> PulseResult<Self> {
        amplitude
            .validate()
            .map_err(PulseError::Parameter)?;

        self.amplitude = Some(amplitude);
        Ok(self)
    }

    /// Sets phase.
    pub fn with_phase(
        mut self,
        phase: Parameter,
    ) -> PulseResult<Self> {
        phase
            .validate()
            .map_err(PulseError::Parameter)?;

        self.phase = Some(phase);
        Ok(self)
    }

    /// Sets frequency.
    pub fn with_frequency(
        mut self,
        frequency: Parameter,
    ) -> PulseResult<Self> {
        frequency
            .validate()
            .map_err(PulseError::Parameter)?;

        self.frequency = Some(frequency);
        Ok(self)
    }

    /// Sets semantic resources.
    #[must_use]
    pub const fn with_resources(
        mut self,
        resources: PulseResources,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Adds a dependency.
    pub fn with_dependency(
        mut self,
        dependency: PulseDependency,
    ) -> PulseResult<Self> {
        if self.dependencies.contains(&dependency) {
            return Err(
                PulseError::DuplicateDependency
            );
        }

        self.dependencies.push(dependency);
        self.dependencies.sort();

        Ok(self)
    }

    /// Adds deterministic metadata.
    pub fn with_metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> PulseResult<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        if key.is_empty() {
            return Err(
                PulseError::EmptyMetadataKey
            );
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Validates using the unrestricted semantic policy.
    pub fn validate(&self) -> PulseResult<()> {
        self.validate_with_policy(
            PulseValidationPolicy::default(),
        )
    }

    /// Validates using an explicit resource policy.
    pub fn validate_with_policy(
        &self,
        policy: PulseValidationPolicy,
    ) -> PulseResult<()> {
        if self.targets.is_empty()
            && !matches!(
                self.kind,
                PulseKind::Global
                    | PulseKind::Barrier
                    | PulseKind::Delay
            )
        {
            // A pulse without explicit targets is legal when the operation
            // explicitly declares global/timing semantics. Ordinary play,
            // capture and acquire operations require an explicit target.
            //
            // `Global` is represented by the target list containing
            // `PulseTarget::Global`; this branch catches accidental omission.
            return Err(
                PulseError::MissingTarget
            );
        }

        for target in &self.targets {
            target.validate()?;
        }

        if let Some(maximum) = policy.max_targets {
            let count = self
                .targets
                .iter()
                .map(PulseTarget::explicit_qubit_count)
                .try_fold(
                    0usize,
                    usize::checked_add,
                )
                .ok_or(PulseError::TargetCountOverflow)?;

            if count > maximum {
                return Err(
                    PulseError::TargetLimitExceeded {
                        maximum,
                        actual: count,
                    },
                );
            }
        }

        if let Some(amplitude) = &self.amplitude {
            amplitude
                .validate()
                .map_err(PulseError::Parameter)?;
        }

        if let Some(phase) = &self.phase {
            phase
                .validate()
                .map_err(PulseError::Parameter)?;
        }

        if let Some(frequency) = &self.frequency {
            frequency
                .validate()
                .map_err(PulseError::Parameter)?;
        }

        self.validate_kind_semantics()?;

        self.validate_metadata(&policy)?;

        Ok(())
    }

    fn validate_kind_semantics(&self) -> PulseResult<()> {
        match self.kind {
            PulseKind::Play => {
                if self.resources.waveform().is_none() {
                    return Err(
                        PulseError::MissingWaveform
                    );
                }

                if self.duration.is_none() {
                    return Err(
                        PulseError::MissingDuration
                    );
                }
            }

            PulseKind::Capture | PulseKind::Acquire => {
                if self.duration.is_none() {
                    return Err(
                        PulseError::MissingDuration
                    );
                }
            }

            PulseKind::Delay => {
                if self.duration.is_none() {
                    return Err(
                        PulseError::MissingDuration
                    );
                }
            }

            PulseKind::SetFrequency => {
                if self.frequency.is_none() {
                    return Err(
                        PulseError::MissingFrequency
                    );
                }

                if self.resources.frame().is_none() {
                    return Err(
                        PulseError::MissingFrame
                    );
                }
            }

            PulseKind::SetPhase | PulseKind::ShiftPhase => {
                if self.phase.is_none() {
                    return Err(
                        PulseError::MissingPhase
                    );
                }

                if self.resources.frame().is_none() {
                    return Err(
                        PulseError::MissingFrame
                    );
                }
            }

            PulseKind::Barrier
            | PulseKind::Calibration
            | PulseKind::Extension(_) => {}
        }

        Ok(())
    }

    fn validate_metadata(
        &self,
        policy: &PulseValidationPolicy,
    ) -> PulseResult<()> {
        if let Some(maximum) =
            policy.max_metadata_fields
        {
            if self.metadata.len() > maximum {
                return Err(
                    PulseError::MetadataFieldLimitExceeded {
                        maximum,
                        actual: self.metadata.len(),
                    },
                );
            }
        }

        for (key, value) in &self.metadata {
            if let Some(maximum) =
                policy.max_metadata_key_bytes
            {
                if key.len() > maximum {
                    return Err(
                        PulseError::MetadataKeyLimitExceeded {
                            maximum,
                            actual: key.len(),
                        },
                    );
                }
            }

            if let Some(maximum) =
                policy.max_metadata_value_bytes
            {
                if value.len() > maximum {
                    return Err(
                        PulseError::MetadataValueLimitExceeded {
                            maximum,
                            actual: value.len(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns the total explicitly represented logical-qubit count.
    pub fn explicit_qubit_count(&self) -> usize {
        self.targets
            .iter()
            .map(PulseTarget::explicit_qubit_count)
            .fold(0usize, usize::saturating_add)
    }

    /// Returns whether the pulse contains symbolic values.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.amplitude
            .as_ref()
            .map_or(false, Parameter::is_symbolic)
            || self
                .phase
                .as_ref()
                .map_or(false, Parameter::is_symbolic)
            || self
                .frequency
                .as_ref()
                .map_or(false, Parameter::is_symbolic)
    }

    /// Returns all parameters in deterministic semantic order:
    ///
    /// amplitude, phase, frequency.
    pub fn parameters(&self) -> Vec<&Parameter> {
        let mut parameters = Vec::with_capacity(3);

        if let Some(value) = &self.amplitude {
            parameters.push(value);
        }

        if let Some(value) = &self.phase {
            parameters.push(value);
        }

        if let Some(value) = &self.frequency {
            parameters.push(value);
        }

        parameters
    }
}

// ============================================================================
// Pulse sequence
// ============================================================================

/// A semantic pulse sequence.
///
/// A sequence contains pulse identities rather than embedding pulse objects.
/// This prevents accidental duplication and allows the owning program to keep
/// one canonical pulse definition while composing it in multiple places.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PulseSequence {
    id: PulseId,
    pulses: Vec<PulseId>,
    metadata: PulseMetadata,
}

impl PulseSequence {
    /// Creates an empty sequence.
    #[must_use]
    pub const fn new(id: PulseId) -> Self {
        Self {
            id,
            pulses: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Returns sequence identity.
    #[must_use]
    pub const fn id(&self) -> PulseId {
        self.id
    }

    /// Returns ordered pulse identities.
    #[must_use]
    pub fn pulses(&self) -> &[PulseId] {
        &self.pulses
    }

    /// Appends a pulse identity.
    pub fn push(
        &mut self,
        pulse: PulseId,
    ) -> PulseResult<()> {
        self.pulses.push(pulse);
        Ok(())
    }

    /// Extends the sequence.
    pub fn extend<I>(
        &mut self,
        pulses: I,
    )
    where
        I: IntoIterator<Item = PulseId>,
    {
        self.pulses.extend(pulses);
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub fn metadata(&self) -> &PulseMetadata {
        &self.metadata
    }

    /// Adds metadata.
    pub fn with_metadata<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> PulseResult<()>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();

        if key.is_empty() {
            return Err(
                PulseError::EmptyMetadataKey
            );
        }

        self.metadata.insert(
            key,
            value.into(),
        );

        Ok(())
    }

    /// Validates sequence-local structure.
    pub fn validate(&self) -> PulseResult<()> {
        if self.pulses.windows(2).any(|pair| {
            pair[0] == pair[1]
        }) {
            return Err(
                PulseError::DuplicateAdjacentPulse
            );
        }

        Ok(())
    }
}

// ============================================================================
// Pulse composition
// ============================================================================

/// Hardware-independent pulse composition.
///
/// This expresses structural relationships without performing scheduling.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum PulseComposition {
    /// Sequential composition.
    Sequential(Vec<PulseId>),

    /// Parallel composition.
    Parallel(Vec<PulseId>),

    /// Repetition of one pulse.
    Repeat {
        /// Pulse being repeated.
        pulse: PulseId,

        /// Number of repetitions.
        count: u128,
    },

    /// Repetition of a sequence.
    RepeatSequence {
        /// Sequence being repeated.
        sequence: PulseId,

        /// Number of repetitions.
        count: u128,
    },
}

impl PulseComposition {
    /// Returns the number of direct pulse references.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        match self {
            Self::Sequential(pulses)
            | Self::Parallel(pulses) => pulses.len(),

            Self::Repeat { .. }
            | Self::RepeatSequence { .. } => 1,
        }
    }

    /// Validates composition-local structure.
    pub fn validate(&self) -> PulseResult<()> {
        match self {
            Self::Sequential(pulses)
            | Self::Parallel(pulses) => {
                if pulses.is_empty() {
                    return Err(
                        PulseError::EmptyComposition
                    );
                }

                Ok(())
            }

            Self::Repeat {
                count,
                ..
            }
            | Self::RepeatSequence {
                count,
                ..
            } => {
                if *count == 0 {
                    return Err(
                        PulseError::ZeroRepeatCount
                    );
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// Pulse error
// ============================================================================

/// Pulse-local error.
///
/// The error is intentionally structured so the module can be implemented
/// independently before the whole-program diagnostic layer is finalized.
///
/// The parent IR validation layer can convert these errors into canonical
/// `core::errors::IrError` diagnostics without changing pulse semantics.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum PulseError {
    /// Duration arithmetic overflowed.
    DurationOverflow,

    /// Subtraction would produce a negative duration.
    NegativeDuration,

    /// Target set is empty where explicit targets are required.
    EmptyTargetSet,

    /// Target set is not in canonical sorted unique form.
    NonCanonicalTargetSet,

    /// Same target was supplied more than once.
    DuplicateTarget,

    /// Target count arithmetic overflowed.
    TargetCountOverflow,

    /// Explicit target policy was exceeded.
    TargetLimitExceeded {
        maximum: usize,
        actual: usize,
    },

    /// A required target was omitted.
    MissingTarget,

    /// A waveform is required but absent.
    MissingWaveform,

    /// A duration is required but absent.
    MissingDuration,

    /// A phase is required but absent.
    MissingPhase,

    /// A frequency is required but absent.
    MissingFrequency,

    /// A frame is required but absent.
    MissingFrame,

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata field-count policy was exceeded.
    MetadataFieldLimitExceeded {
        maximum: usize,
        actual: usize,
    },

    /// Metadata key-size policy was exceeded.
    MetadataKeyLimitExceeded {
        maximum: usize,
        actual: usize,
    },

    /// Metadata value-size policy was exceeded.
    MetadataValueLimitExceeded {
        maximum: usize,
        actual: usize,
    },

    /// Same dependency was inserted more than once.
    DuplicateDependency,

    /// Adjacent pulse sequence entries are identical.
    DuplicateAdjacentPulse,

    /// Composition has no members.
    EmptyComposition,

    /// Repeat count cannot be zero.
    ZeroRepeatCount,

    /// Parameter validation failed.
    Parameter(String),
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

            Self::EmptyTargetSet => {
                formatter.write_str(
                    "pulse target set cannot be empty",
                )
            }

            Self::NonCanonicalTargetSet => {
                formatter.write_str(
                    "pulse target set must be strictly sorted and unique",
                )
            }

            Self::DuplicateTarget => {
                formatter.write_str(
                    "pulse target was specified more than once",
                )
            }

            Self::TargetCountOverflow => {
                formatter.write_str(
                    "pulse target count overflowed",
                )
            }

            Self::TargetLimitExceeded {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "pulse target policy exceeded: maximum {maximum}, actual {actual}",
                )
            }

            Self::MissingTarget => {
                formatter.write_str(
                    "pulse operation requires an explicit target",
                )
            }

            Self::MissingWaveform => {
                formatter.write_str(
                    "play pulse requires a waveform reference",
                )
            }

            Self::MissingDuration => {
                formatter.write_str(
                    "pulse operation requires a duration",
                )
            }

            Self::MissingPhase => {
                formatter.write_str(
                    "pulse operation requires a phase parameter",
                )
            }

            Self::MissingFrequency => {
                formatter.write_str(
                    "pulse operation requires a frequency parameter",
                )
            }

            Self::MissingFrame => {
                formatter.write_str(
                    "frame-dependent pulse operation requires a frame",
                )
            }

            Self::EmptyMetadataKey => {
                formatter.write_str(
                    "pulse metadata key cannot be empty",
                )
            }

            Self::MetadataFieldLimitExceeded {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "pulse metadata field policy exceeded: maximum {maximum}, actual {actual}",
                )
            }

            Self::MetadataKeyLimitExceeded {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "pulse metadata key exceeds policy: maximum {maximum} bytes, actual {actual} bytes",
                )
            }

            Self::MetadataValueLimitExceeded {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "pulse metadata value exceeds policy: maximum {maximum} bytes, actual {actual} bytes",
                )
            }

            Self::DuplicateDependency => {
                formatter.write_str(
                    "pulse dependency was specified more than once",
                )
            }

            Self::DuplicateAdjacentPulse => {
                formatter.write_str(
                    "pulse sequence contains duplicate adjacent pulse identities",
                )
            }

            Self::EmptyComposition => {
                formatter.write_str(
                    "pulse composition cannot be empty",
                )
            }

            Self::ZeroRepeatCount => {
                formatter.write_str(
                    "pulse repetition count must be greater than zero",
                )
            }

            Self::Parameter(message) => {
                write!(
                    formatter,
                    "invalid pulse parameter: {message}",
                )
            }
        }
    }
}

impl std::error::Error for PulseError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::core::identity::{
        ChannelId,
        FrameId,
        PulseId,
        WaveformId,
    };

    use super::super::super::core::parameter::Parameter;

    use super::super::super::quantum::qubit::QubitId;

    #[test]
    fn duration_conversion_is_exact() {
        let duration =
            PulseDuration::from_nanoseconds(20)
                .expect("20 ns must be representable");

        assert_eq!(
            duration.femtoseconds(),
            20_000_000
        );
    }

    #[test]
    fn duration_checked_arithmetic_is_safe() {
        let duration =
            PulseDuration::from_femtoseconds(10);

        assert_eq!(
            duration
                .checked_add(
                    PulseDuration::from_femtoseconds(20)
                )
                .expect("addition must succeed")
                .femtoseconds(),
            30
        );

        assert!(
            PulseDuration::from_femtoseconds(0)
                .checked_sub(
                    PulseDuration::from_femtoseconds(1)
                )
                .is_err()
        );
    }

    #[test]
    fn target_collection_is_deterministic() {
        let target =
            PulseTarget::qubits([
                QubitId::new(9),
                QubitId::new(2),
                QubitId::new(9),
                QubitId::new(4),
            ]);

        assert_eq!(
            target.logical_qubits(),
            &[
                QubitId::new(2),
                QubitId::new(4),
                QubitId::new(9),
            ]
        );
    }

    #[test]
    fn symbolic_parameters_are_preserved() {
        let parameter =
            Parameter::symbol("drive_amplitude")
                .expect("symbol must be valid");

        let pulse =
            Pulse::new(
                PulseId::new(1),
                PulseKind::Play,
            )
            .with_target(
                PulseTarget::qubit(
                    QubitId::new(0)
                )
            )
            .expect("target must be valid")
            .with_duration(
                PulseDuration::from_nanoseconds(20)
                    .expect("duration must be valid")
            )
            .with_amplitude(parameter)
            .expect("parameter must be valid")
            .with_resources(
                PulseResources::empty()
                    .with_waveform(
                        WaveformId::new(1)
                    )
                    .with_channel(
                        ChannelId::new(1)
                    )
                    .with_frame(
                        FrameId::new(1)
                    ),
            );

        assert!(pulse.is_symbolic());
        assert!(pulse.validate().is_ok());
    }

    #[test]
    fn play_requires_waveform() {
        let pulse =
            Pulse::new(
                PulseId::new(1),
                PulseKind::Play,
            )
            .with_target(
                PulseTarget::qubit(
                    QubitId::new(0)
                )
            )
            .expect("target must be valid")
            .with_duration(
                PulseDuration::from_nanoseconds(20)
                    .expect("duration must be valid")
            );

        assert_eq!(
            pulse.validate(),
            Err(PulseError::MissingWaveform)
        );
    }

    #[test]
    fn frame_operations_require_frame() {
        let frequency =
            Parameter::constant(5.0)
                .expect("finite parameter");

        let pulse =
            Pulse::new(
                PulseId::new(2),
                PulseKind::SetFrequency,
            )
            .with_target(
                PulseTarget::qubit(
                    QubitId::new(0)
                )
            )
            .expect("target must be valid")
            .with_frequency(frequency)
            .expect("frequency must be valid");

        assert_eq!(
            pulse.validate(),
            Err(PulseError::MissingFrame)
        );
    }

    #[test]
    fn metadata_order_is_deterministic() {
        let pulse =
            Pulse::new(
                PulseId::new(1),
                PulseKind::Barrier,
            )
            .with_metadata("z", "last")
            .expect("metadata must be valid")
            .with_metadata("a", "first")
            .expect("metadata must be valid");

        let keys: Vec<&str> = pulse
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(
            keys,
            vec!["a", "z"]
        );
    }

    #[test]
    fn repeat_requires_positive_count() {
        let composition =
            PulseComposition::Repeat {
                pulse: PulseId::new(1),
                count: 0,
            };

        assert_eq!(
            composition.validate(),
            Err(PulseError::ZeroRepeatCount)
        );
    }

    #[test]
    fn explicit_policy_is_separate_from_semantics() {
        let pulse =
            Pulse::new(
                PulseId::new(1),
                PulseKind::Barrier,
            )
            .with_target(
                PulseTarget::qubit(
                    QubitId::new(0)
                )
            )
            .expect("target must be valid");

        assert!(
            pulse
                .validate_with_policy(
                    PulseValidationPolicy::unrestricted()
                )
                .is_ok()
        );

        assert!(
            pulse
                .validate_with_policy(
                    PulseValidationPolicy::default()
                        .with_max_targets(0)
                )
                .is_err()
        );
    }
}