//! Zamani Quantum IR — Pulse Dialect
//!
//! Path:
//!     src/quantum/ir/dialect/pulse.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines the canonical Zamani Pulse Dialect contract.
//!
//! It does NOT define the underlying pulse semantic objects themselves.
//! Those belong to:
//!
//!     quantum::ir::pulse
//!
//! In particular, this module MUST NOT redefine:
//!
//!     Pulse
//!     PulseDuration
//!     PulseTarget
//!     PulseResources
//!     PulseKind
//!     PulseId
//!     QubitId
//!     WaveformId
//!     ChannelId
//!     FrameId
//!     CalibrationId
//!     Parameter
//!
//! The dialect answers:
//!
//!     "Which semantic operations, attributes, requirements and extension
//!      contracts constitute the Zamani Pulse Dialect?"
//!
//! The pulse subsystem answers:
//!
//!     "What does a particular pulse operation mean?"
//!
//! Hardware answers:
//!
//!     "How is that semantic operation realized on this target?"
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! canonical Zamani IR
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! standard dialect                pulse dialect
//!                                      |
//!                                      v
//!                              target-independent
//!                              pulse semantics
//!                                      |
//!                                      v
//!                                  optimization
//!                                      |
//!                                      v
//!                                  scheduling
//!                                      |
//!                                      v
//!                                   mapping
//!                                      |
//!                                      v
//!                                   hardware
//!                                      |
//!                                      v
//!                                   backend
//!                                      |
//!                                      v
//!                                    QPU
//! ```
//!
//! The dependency direction MUST never be reversed.
//!
//! This file may depend on canonical IR and pulse semantics.
//!
//! It must never depend on:
//!
//!     hardware
//!     backend
//!     frontend
//!     simulator
//!     optimizer
//!     router
//!     scheduler
//!     vendor SDK
//!     provider credentials
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program is written once at the semantic level.
//!
//! The same semantic program may eventually be lowered to:
//!
//!     superconducting
//!     trapped-ion
//!     neutral-atom
//!     photonic
//!     spin
//!     topological
//!     analog
//!     distributed
//!     logical/fault-tolerant
//!     simulator
//!     future architectures
//!
//! The Pulse Dialect therefore contains NO machine-size limit.
//!
//! It does not contain:
//!
//!     MAX_QUBITS
//!     MAX_PULSES
//!     MAX_CHANNELS
//!     MAX_FRAMES
//!     MAX_WAVEFORMS
//!     MAX_TARGETS
//!
//! Any resource limit is an explicit compilation/security policy owned by the
//! appropriate resource or validation layer.
//!
//! ============================================================================
//! HARDWARE INDEPENDENCE
//! ============================================================================
//!
//! The dialect may express:
//!
//!     play
//!     capture
//!     acquire
//!     delay
//!     barrier
//!     set_frequency
//!     set_phase
//!     shift_phase
//!     calibration
//!
//! without deciding:
//!
//!     DAC
//!     ADC
//!     microwave source
//!     laser
//!     oscillator
//!     control line
//!     sample clock
//!     physical channel
//!     physical qubit
//!     topology
//!     vendor
//!
//! Those are downstream concerns.
//!
//! ============================================================================
//! OPENQASM ALIGNMENT
//! ============================================================================
//!
//! The dialect is deliberately capable of representing the semantic concepts
//! required by modern pulse-oriented quantum IRs, including:
//!
//!     waveform definitions/references
//!     frame references
//!     ports/channels
//!     play
//!     capture
//!     acquisition
//!     delays
//!     barriers
//!     calibration references
//!     phase/frequency manipulation
//!     timing constraints
//!     symbolic parameters
//!
//! OpenQASM 3.1 explicitly separates pulse-level descriptions from higher-level
//! quantum operations and includes ports, frames, waveforms, play/capture,
//! calibration and timing semantics.
//!
//! Zamani is NOT required to copy OpenQASM's AST or grammar.
//!
//! OpenQASM is an input/output dialect.
//!
//! Zamani Pulse Dialect is part of the canonical Zamani IR.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Supported:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021
//!     stable Rust
//!
//! Forbidden:
//!
//!     unsafe
//!     nightly features
//!     external dependencies
//!     vendor SDKs
//!
//! Safety is compiler-enforced with `forbid(unsafe_code)`.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Canonical pulse semantics:
//!
//!     quantum::ir::pulse
//!
//! Canonical logical qubit identity:
//!
//!     quantum::ir::qubit::QubitId
//!
//! Canonical semantic identities:
//!
//!     quantum::ir::identity
//!
//! Canonical parameters:
//!
//!     quantum::ir::parameter
//!
//! Canonical operation representation:
//!
//!     quantum::ir::operation
//!
//! Timing:
//!
//!     quantum::ir::timing
//!
//! Waveforms:
//!
//!     quantum::ir::waveform
//!
//! Channels:
//!
//!     quantum::ir::channel
//!
//! Frames:
//!
//!     quantum::ir::frame
//!
//! Calibration:
//!
//!     quantum::ir::calibration
//!
//! Validation:
//!
//!     quantum::ir::validation
//!
//! Serialization:
//!
//!     quantum::ir::serialization
//!
//! Hashing:
//!
//!     quantum::ir::hash
//!
//! Provenance:
//!
//!     quantum::ir::provenance
//!
//! Capability/resource resolution:
//!
//!     quantum::ir::resources
//!
//! Hardware lowering:
//!
//!     downstream hardware/backend layers
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//!     PulseDialect
//!     PulseDialectVersion
//!     PulseOperation
//!     PulseOperationName
//!     PulseOperationSchema
//!     PulseOperandRole
//!     PulseResultRole
//!     PulseRequirement
//!     PulseAttributeKey
//!     PulseExtension
//!     PulseExtensionRegistry
//!     PulseDialectError
//!     PulseDialectValidation
//!
//! This file does NOT own:
//!
//!     Pulse
//!     waveform definitions
//!     channel definitions
//!     frame definitions
//!     calibration definitions
//!     timing schedules
//!     physical mapping
//!     hardware capabilities
//!     execution
//!
//! ============================================================================
//! DESIGN RULE
//! ============================================================================
//!
//! Standard operations are represented as stable dialect names rather than as
//! a closed universe that prevents future extensions.
//!
//! Therefore:
//!
//!     PulseOperationName::Standard(...)
//!
//! represents the stable Zamani-defined operation namespace, while:
//!
//!     PulseOperationName::Extension(...)
//!
//! represents extension-defined operations.
//!
//! A future pulse operation must not require modification of the fundamental
//! Pulse object merely because the dialect grows.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::identity::ExtensionId;
use super::super::pulse::PulseKind;

// ============================================================================
// Dialect identity
// ============================================================================

/// Canonical fully qualified Pulse Dialect name.
pub const PULSE_DIALECT_NAME: &str = "zamani.pulse";

/// Stable dialect schema identifier.
pub const PULSE_DIALECT_SCHEMA_ID: &str =
    "zamani.quantum.ir.dialect.pulse";

/// Current Pulse Dialect major version.
pub const PULSE_DIALECT_MAJOR: u16 = 1;

/// Current Pulse Dialect minor version.
pub const PULSE_DIALECT_MINOR: u16 = 0;

/// Current Pulse Dialect patch version.
pub const PULSE_DIALECT_PATCH: u16 = 0;

/// Current Pulse Dialect version.
pub const CURRENT_PULSE_DIALECT_VERSION: PulseDialectVersion =
    PulseDialectVersion::new(
        PULSE_DIALECT_MAJOR,
        PULSE_DIALECT_MINOR,
        PULSE_DIALECT_PATCH,
    );

// ============================================================================
// Version
// ============================================================================

/// Semantic version of the Zamani Pulse Dialect.
///
/// Dialect versioning is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - hardware version;
/// - provider version;
/// - calibration version;
/// - Pulse semantic object version.
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
pub struct PulseDialectVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl PulseDialectVersion {
    /// Creates a dialect version.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns the current version.
    #[must_use]
    pub const fn current() -> Self {
        CURRENT_PULSE_DIALECT_VERSION
    }

    /// Returns whether this version belongs to the same major contract.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether `self` can consume `other` under the conservative
    /// backward-compatible dialect policy.
    #[must_use]
    pub const fn supports(self, other: Self) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && (
                other.minor < self.minor
                    || other.patch <= self.patch
            )
    }

    /// Returns whether a major migration is required.
    #[must_use]
    pub const fn requires_major_migration(
        self,
        other: Self,
    ) -> bool {
        self.major != other.major
    }
}

impl Default for PulseDialectVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for PulseDialectVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// ============================================================================
// Operation names
// ============================================================================

/// Stable standard operation names of the Zamani Pulse Dialect.
///
/// These names identify semantic intent.
///
/// They do not identify vendor instructions.
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
pub enum PulseStandardOperation {
    /// Emit/play a waveform or envelope.
    Play,

    /// Capture an analog signal.
    Capture,

    /// Acquire measurement information.
    Acquire,

    /// Delay execution without an active pulse.
    Delay,

    /// Synchronization boundary.
    Barrier,

    /// Set a frame's frequency.
    SetFrequency,

    /// Set a frame's absolute phase.
    SetPhase,

    /// Shift a frame's phase relative to its current phase.
    ShiftPhase,

    /// Invoke a semantic calibration.
    Calibration,
}

impl PulseStandardOperation {
    /// Returns the canonical operation spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Play => "play",
            Self::Capture => "capture",
            Self::Acquire => "acquire",
            Self::Delay => "delay",
            Self::Barrier => "barrier",
            Self::SetFrequency => "set_frequency",
            Self::SetPhase => "set_phase",
            Self::ShiftPhase => "shift_phase",
            Self::Calibration => "calibration",
        }
    }

    /// Converts this dialect operation to the canonical pulse semantic kind.
    #[must_use]
    pub const fn pulse_kind(self) -> PulseKind {
        match self {
            Self::Play => PulseKind::Play,
            Self::Capture => PulseKind::Capture,
            Self::Acquire => PulseKind::Acquire,
            Self::Delay => PulseKind::Delay,
            Self::Barrier => PulseKind::Barrier,
            Self::SetFrequency => PulseKind::SetFrequency,
            Self::SetPhase => PulseKind::SetPhase,
            Self::ShiftPhase => PulseKind::ShiftPhase,
            Self::Calibration => PulseKind::Calibration,
        }
    }

    /// Returns the operation from its canonical spelling.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "play" => Some(Self::Play),
            "capture" => Some(Self::Capture),
            "acquire" => Some(Self::Acquire),
            "delay" => Some(Self::Delay),
            "barrier" => Some(Self::Barrier),
            "set_frequency" => Some(Self::SetFrequency),
            "set_phase" => Some(Self::SetPhase),
            "shift_phase" => Some(Self::ShiftPhase),
            "calibration" => Some(Self::Calibration),
            _ => None,
        }
    }
}

/// Fully qualified Pulse Dialect operation name.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum PulseOperationName {
    /// Standard Zamani Pulse Dialect operation.
    Standard(PulseStandardOperation),

    /// Extension-defined pulse operation.
    Extension(ExtensionId),
}

impl PulseOperationName {
    /// Creates a standard operation name.
    #[must_use]
    pub const fn standard(
        operation: PulseStandardOperation,
    ) -> Self {
        Self::Standard(operation)
    }

    /// Creates an extension operation name.
    #[must_use]
    pub const fn extension(
        extension: ExtensionId,
    ) -> Self {
        Self::Extension(extension)
    }

    /// Returns the canonical textual name.
    ///
    /// Extension names are represented by their stable identity rather than
    /// inventing a textual provider name.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Standard(operation) => format!(
                "{}.{}",
                PULSE_DIALECT_NAME,
                operation.as_str()
            ),

            Self::Extension(extension) => format!(
                "{}.extension.{}",
                PULSE_DIALECT_NAME,
                extension.value()
            ),
        }
    }

    /// Returns whether the operation belongs to the standard namespace.
    #[must_use]
    pub const fn is_standard(&self) -> bool {
        matches!(self, Self::Standard(_))
    }

    /// Returns whether the operation is extension-defined.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_))
    }
}

impl fmt::Display for PulseOperationName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}

// ============================================================================
// Operand roles
// ============================================================================

/// Semantic role of an operand consumed by a pulse operation.
///
/// These roles describe meaning, not physical placement.
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
pub enum PulseOperandRole {
    /// Logical quantum target.
    Target,

    /// Waveform definition/reference.
    Waveform,

    /// Abstract channel reference.
    Channel,

    /// Abstract frame reference.
    Frame,

    /// Calibration reference.
    Calibration,

    /// Symbolic duration.
    Duration,

    /// Symbolic amplitude.
    Amplitude,

    /// Symbolic frequency.
    Frequency,

    /// Symbolic phase.
    Phase,

    /// Classical/measurement destination.
    Result,

    /// Extension-defined operand role.
    Extension,
}

impl PulseOperandRole {
    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Target => "target",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Calibration => "calibration",
            Self::Duration => "duration",
            Self::Amplitude => "amplitude",
            Self::Frequency => "frequency",
            Self::Phase => "phase",
            Self::Result => "result",
            Self::Extension => "extension",
        }
    }
}

// ============================================================================
// Result roles
// ============================================================================

/// Semantic role of a pulse operation result.
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
pub enum PulseResultRole {
    /// Captured analog/sample data.
    CaptureData,

    /// Measurement/acquisition result.
    Measurement,

    /// Classical discriminator result.
    DiscriminatedValue,

    /// Calibration output.
    CalibrationData,

    /// Extension-defined result.
    Extension,
}

impl PulseResultRole {
    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureData => "capture_data",
            Self::Measurement => "measurement",
            Self::DiscriminatedValue => "discriminated_value",
            Self::CalibrationData => "calibration_data",
            Self::Extension => "extension",
        }
    }
}

// ============================================================================
// Requirements
// ============================================================================

/// Capability/resource requirement expressed by a pulse operation.
///
/// This is deliberately abstract.
///
/// It does not identify a provider or machine.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum PulseRequirement {
    /// Operation requires waveform playback.
    WaveformPlayback,

    /// Operation requires analog capture.
    AnalogCapture,

    /// Operation requires measurement acquisition.
    MeasurementAcquisition,

    /// Operation requires frame frequency control.
    FrameFrequencyControl,

    /// Operation requires frame phase control.
    FramePhaseControl,

    /// Operation requires calibration support.
    Calibration,

    /// Operation requires explicit timing support.
    Timing,

    /// Operation requires dynamic/conditional execution.
    DynamicControl,

    /// Operation requires an extension capability.
    Extension(ExtensionId),
}

impl PulseRequirement {
    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::WaveformPlayback => {
                "pulse.waveform_playback"
            }

            Self::AnalogCapture => {
                "pulse.analog_capture"
            }

            Self::MeasurementAcquisition => {
                "pulse.measurement_acquisition"
            }

            Self::FrameFrequencyControl => {
                "pulse.frame_frequency_control"
            }

            Self::FramePhaseControl => {
                "pulse.frame_phase_control"
            }

            Self::Calibration => {
                "pulse.calibration"
            }

            Self::Timing => {
                "pulse.timing"
            }

            Self::DynamicControl => {
                "pulse.dynamic_control"
            }

            Self::Extension(_) => {
                "pulse.extension"
            }
        }
    }
}

// ============================================================================
// Attribute keys
// ============================================================================

/// Standard semantic attributes understood by the Pulse Dialect.
///
/// Attribute values themselves remain owned by the canonical IR attribute
/// system; this enum only defines stable names.
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
pub enum PulseAttributeKey {
    /// Pulse duration.
    Duration,

    /// Pulse amplitude.
    Amplitude,

    /// Pulse phase.
    Phase,

    /// Pulse frequency.
    Frequency,

    /// Waveform reference.
    Waveform,

    /// Channel reference.
    Channel,

    /// Frame reference.
    Frame,

    /// Calibration reference.
    Calibration,

    /// Pulse envelope description.
    Envelope,

    /// Timing alignment intent.
    Timing,

    /// Capture mode.
    CaptureMode,

    /// Acquisition mode.
    AcquisitionMode,

    /// Extension-specific attribute.
    Extension,
}

impl PulseAttributeKey {
    /// Returns the stable attribute spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Duration => "duration",
            Self::Amplitude => "amplitude",
            Self::Phase => "phase",
            Self::Frequency => "frequency",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Calibration => "calibration",
            Self::Envelope => "envelope",
            Self::Timing => "timing",
            Self::CaptureMode => "capture_mode",
            Self::AcquisitionMode => "acquisition_mode",
            Self::Extension => "extension",
        }
    }

    /// Returns the fully qualified attribute namespace.
    #[must_use]
    pub fn qualified_name(self) -> String {
        format!(
            "{}.{}",
            PULSE_DIALECT_NAME,
            self.as_str()
        )
    }
}

// ============================================================================
// Operation schema
// ============================================================================

/// Static schema describing one Pulse Dialect operation.
///
/// This is metadata about an operation, not an operation instance.
///
/// It intentionally avoids encoding a fixed number of qubits.
///
/// A target may be:
///
/// - one logical qubit;
/// - multiple logical qubits;
/// - a global semantic target;
/// - an abstract resource.
///
/// Cardinality is therefore described by [`PulseCardinality`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PulseOperationSchema {
    name: PulseOperationName,
    cardinality: PulseCardinality,
    operands: Vec<PulseOperandRole>,
    results: Vec<PulseResultRole>,
    requirements: Vec<PulseRequirement>,
    timing_required: bool,
    waveform_required: bool,
    frame_supported: bool,
    calibration_supported: bool,
}

impl PulseOperationSchema {
    /// Creates an operation schema.
    pub fn new(
        name: PulseOperationName,
        cardinality: PulseCardinality,
    ) -> PulseDialectResult<Self> {
        if let PulseCardinality::Fixed(0) = cardinality {
            return Err(
                PulseDialectError::InvalidCardinality
            );
        }

        Ok(Self {
            name,
            cardinality,
            operands: Vec::new(),
            results: Vec::new(),
            requirements: Vec::new(),
            timing_required: false,
            waveform_required: false,
            frame_supported: false,
            calibration_supported: false,
        })
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &PulseOperationName {
        &self.name
    }

    /// Returns operand roles.
    #[must_use]
    pub fn operands(&self) -> &[PulseOperandRole] {
        &self.operands
    }

    /// Returns result roles.
    #[must_use]
    pub fn results(&self) -> &[PulseResultRole] {
        &self.results
    }

    /// Returns requirements.
    #[must_use]
    pub fn requirements(&self) -> &[PulseRequirement] {
        &self.requirements
    }

    /// Returns target cardinality.
    #[must_use]
    pub const fn cardinality(&self) -> PulseCardinality {
        self.cardinality
    }

    /// Returns whether timing information is required.
    #[must_use]
    pub const fn timing_required(&self) -> bool {
        self.timing_required
    }

    /// Returns whether a waveform is required.
    #[must_use]
    pub const fn waveform_required(&self) -> bool {
        self.waveform_required
    }

    /// Returns whether frames are supported.
    #[must_use]
    pub const fn frame_supported(&self) -> bool {
        self.frame_supported
    }

    /// Returns whether calibration is supported.
    #[must_use]
    pub const fn calibration_supported(&self) -> bool {
        self.calibration_supported
    }

    /// Adds an operand role.
    #[must_use]
    pub fn with_operand(
        mut self,
        role: PulseOperandRole,
    ) -> Self {
        self.operands.push(role);
        self
    }

    /// Adds a result role.
    #[must_use]
    pub fn with_result(
        mut self,
        role: PulseResultRole,
    ) -> Self {
        self.results.push(role);
        self
    }

    /// Adds a requirement.
    #[must_use]
    pub fn with_requirement(
        mut self,
        requirement: PulseRequirement,
    ) -> Self {
        if !self.requirements.contains(&requirement) {
            self.requirements.push(requirement);
        }

        self
    }

    /// Marks timing as required.
    #[must_use]
    pub const fn requiring_timing(
        mut self,
    ) -> Self {
        self.timing_required = true;
        self
    }

    /// Marks waveform as required.
    #[must_use]
    pub const fn requiring_waveform(
        mut self,
    ) -> Self {
        self.waveform_required = true;
        self
    }

    /// Marks frame support.
    #[must_use]
    pub const fn supporting_frame(
        mut self,
    ) -> Self {
        self.frame_supported = true;
        self
    }

    /// Marks calibration support.
    #[must_use]
    pub const fn supporting_calibration(
        mut self,
    ) -> Self {
        self.calibration_supported = true;
        self
    }

    /// Validates the schema.
    pub fn validate(&self) -> PulseDialectResult<()> {
        if self.operands.contains(&PulseOperandRole::Waveform)
            && !self.waveform_required
            && !self
                .requirements
                .contains(&PulseRequirement::WaveformPlayback)
            && matches!(
                self.name,
                PulseOperationName::Standard(
                    PulseStandardOperation::Play
                )
            )
        {
            return Err(
                PulseDialectError::InvalidSchema {
                    operation: self.name.as_str(),
                    reason:
                        "play schema declares a waveform operand without waveform semantics",
                },
            );
        }

        if self
            .results
            .contains(&PulseResultRole::Measurement)
            && !matches!(
                self.name,
                PulseOperationName::Standard(
                    PulseStandardOperation::Acquire
                )
            )
            && !matches!(
                self.name,
                PulseOperationName::Standard(
                    PulseStandardOperation::Capture
                )
            )
        {
            return Err(
                PulseDialectError::InvalidSchema {
                    operation: self.name.as_str(),
                    reason:
                        "measurement result is only valid for capture/acquire schemas",
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Cardinality
// ============================================================================

/// Target cardinality for a pulse operation.
///
/// This is intentionally semantic rather than hardware-specific.
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
pub enum PulseCardinality {
    /// Exactly one target.
    Fixed(u64),

    /// One or more targets.
    AtLeastOne,

    /// Any number of explicit targets, including zero when the operation
    /// semantics permit global execution.
    Variadic,

    /// Global semantic operation with no explicit finite target requirement.
    Global,
}

impl PulseCardinality {
    /// Checks whether a concrete explicit target count is structurally
    /// compatible with this cardinality.
    ///
    /// Global operations are checked separately because zero explicit targets
    /// may be semantically intentional.
    #[must_use]
    pub const fn accepts(self, count: u64) -> bool {
        match self {
            Self::Fixed(expected) => count == expected,
            Self::AtLeastOne => count >= 1,
            Self::Variadic => true,
            Self::Global => count == 0,
        }
    }
}

// ============================================================================
// Extension descriptor
// ============================================================================

/// Description of a Pulse Dialect extension.
///
/// An extension is identified by a stable IR [`ExtensionId`].
///
/// The descriptor does not contain executable code and does not authorize
/// hardware access.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PulseExtension {
    id: ExtensionId,
    name: String,
    version: PulseDialectVersion,
    operations: Vec<PulseOperationSchema>,
}

impl PulseExtension {
    /// Creates an extension descriptor.
    pub fn new(
        id: ExtensionId,
        name: String,
        version: PulseDialectVersion,
    ) -> PulseDialectResult<Self> {
        validate_extension_name(&name)?;

        Ok(Self {
            id,
            name,
            version,
            operations: Vec::new(),
        })
    }

    /// Returns the extension identity.
    #[must_use]
    pub const fn id(&self) -> ExtensionId {
        self.id
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the extension version.
    #[must_use]
    pub const fn version(&self) -> PulseDialectVersion {
        self.version
    }

    /// Returns registered operation schemas.
    #[must_use]
    pub fn operations(&self) -> &[PulseOperationSchema] {
        &self.operations
    }

    /// Adds an operation schema.
    pub fn add_operation(
        &mut self,
        operation: PulseOperationSchema,
    ) -> PulseDialectResult<()> {
        operation.validate()?;

        if self
            .operations
            .iter()
            .any(|existing| {
                existing.name() == operation.name()
            })
        {
            return Err(
                PulseDialectError::DuplicateOperation(
                    operation.name().as_str()
                ),
            );
        }

        self.operations.push(operation);
        Ok(())
    }
}

// ============================================================================
// Extension registry
// ============================================================================

/// Deterministic registry of Pulse Dialect extensions.
///
/// `BTreeMap` is intentionally used so registry traversal does not depend on
/// randomized hash-map iteration.
///
/// This registry is local to a compilation context. It is not global mutable
/// process state.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
)]
pub struct PulseExtensionRegistry {
    extensions: BTreeMap<ExtensionId, PulseExtension>,
}

impl PulseExtensionRegistry {
    /// Creates an empty extension registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an extension.
    pub fn register(
        &mut self,
        extension: PulseExtension,
    ) -> PulseDialectResult<()> {
        if self.extensions.contains_key(&extension.id()) {
            return Err(
                PulseDialectError::DuplicateExtension(
                    extension.id()
                ),
            );
        }

        self.extensions
            .insert(extension.id(), extension);

        Ok(())
    }

    /// Gets an extension by identity.
    #[must_use]
    pub fn get(
        &self,
        id: ExtensionId,
    ) -> Option<&PulseExtension> {
        self.extensions.get(&id)
    }

    /// Returns deterministic extension entries.
    #[must_use]
    pub fn entries(
        &self,
    ) -> &BTreeMap<ExtensionId, PulseExtension> {
        &self.extensions
    }

    /// Returns whether an extension exists.
    #[must_use]
    pub fn contains(
        &self,
        id: ExtensionId,
    ) -> bool {
        self.extensions.contains_key(&id)
    }

    /// Returns the number of registered extensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.extensions.len()
    }

    /// Returns whether no extensions are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.extensions.is_empty()
    }
}

// ============================================================================
// Dialect
// ============================================================================

/// Canonical Pulse Dialect descriptor.
///
/// The descriptor is immutable after construction.
///
/// Extension registration is explicit and scoped to the descriptor instance.
///
/// No global registry is used.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PulseDialect {
    version: PulseDialectVersion,
    standard_operations:
        BTreeMap<PulseStandardOperation, PulseOperationSchema>,
    extensions: PulseExtensionRegistry,
}

impl PulseDialect {
    /// Creates the current standard Pulse Dialect.
    pub fn new() -> PulseDialectResult<Self> {
        let mut dialect = Self {
            version: PulseDialectVersion::current(),
            standard_operations: BTreeMap::new(),
            extensions: PulseExtensionRegistry::new(),
        };

        dialect.register_standard_operations()?;
        dialect.validate()?;

        Ok(dialect)
    }

    /// Creates an empty dialect for explicit schema construction.
    ///
    /// This is useful for compatibility tooling and future dialect versions.
    #[must_use]
    pub fn empty(version: PulseDialectVersion) -> Self {
        Self {
            version,
            standard_operations: BTreeMap::new(),
            extensions: PulseExtensionRegistry::new(),
        }
    }

    /// Returns the dialect name.
    #[must_use]
    pub const fn name() -> &'static str {
        PULSE_DIALECT_NAME
    }

    /// Returns the schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        PULSE_DIALECT_SCHEMA_ID
    }

    /// Returns the dialect version.
    #[must_use]
    pub const fn version(&self) -> PulseDialectVersion {
        self.version
    }

    /// Returns the standard operation schema.
    #[must_use]
    pub fn operation(
        &self,
        operation: PulseStandardOperation,
    ) -> Option<&PulseOperationSchema> {
        self.standard_operations.get(&operation)
    }

    /// Returns an operation schema by fully qualified name.
    #[must_use]
    pub fn operation_by_name(
        &self,
        name: &str,
    ) -> Option<&PulseOperationSchema> {
        self.standard_operations
            .values()
            .find(|schema| {
                schema.name().as_str() == name
            })
    }

    /// Returns the extension registry.
    #[must_use]
    pub fn extensions(
        &self,
    ) -> &PulseExtensionRegistry {
        &self.extensions
    }

    /// Registers an extension.
    pub fn register_extension(
        &mut self,
        extension: PulseExtension,
    ) -> PulseDialectResult<()> {
        self.extensions.register(extension)?;
        self.validate()
    }

    /// Validates the complete dialect.
    pub fn validate(&self) -> PulseDialectResult<()> {
        if self.version.major == 0 {
            return Err(
                PulseDialectError::InvalidVersion
            );
        }

        for schema in self.standard_operations.values() {
            schema.validate()?;
        }

        for extension in self.extensions.entries().values() {
            for operation in extension.operations() {
                operation.validate()?;
            }
        }

        Ok(())
    }

    /// Returns all standard operation schemas in deterministic order.
    #[must_use]
    pub fn standard_operations(
        &self,
    ) -> &BTreeMap<
        PulseStandardOperation,
        PulseOperationSchema,
    > {
        &self.standard_operations
    }

    fn register_standard_operations(
        &mut self,
    ) -> PulseDialectResult<()> {
        let schemas = [
            (
                PulseStandardOperation::Play,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Play
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Waveform
                )
                .with_operand(
                    PulseOperandRole::Channel
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_operand(
                    PulseOperandRole::Amplitude
                )
                .with_operand(
                    PulseOperandRole::Phase
                )
                .requiring_waveform()
                .with_requirement(
                    PulseRequirement::WaveformPlayback
                )
                .with_requirement(
                    PulseRequirement::Timing
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::Capture,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Capture
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Waveform
                )
                .with_operand(
                    PulseOperandRole::Channel
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_result(
                    PulseResultRole::CaptureData
                )
                .requiring_waveform()
                .with_requirement(
                    PulseRequirement::AnalogCapture
                )
                .with_requirement(
                    PulseRequirement::Timing
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::Acquire,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Acquire
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Channel
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_result(
                    PulseResultRole::Measurement
                )
                .with_result(
                    PulseResultRole::DiscriminatedValue
                )
                .with_requirement(
                    PulseRequirement::MeasurementAcquisition
                )
                .with_requirement(
                    PulseRequirement::Timing
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::Delay,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Delay
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Duration
                )
                .with_requirement(
                    PulseRequirement::Timing
                )
                .requiring_timing(),
            ),
            (
                PulseStandardOperation::Barrier,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Barrier
                    ),
                    PulseCardinality::Variadic,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_requirement(
                    PulseRequirement::Timing
                )
                .requiring_timing(),
            ),
            (
                PulseStandardOperation::SetFrequency,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::SetFrequency
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_operand(
                    PulseOperandRole::Frequency
                )
                .with_requirement(
                    PulseRequirement::FrameFrequencyControl
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::SetPhase,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::SetPhase
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_operand(
                    PulseOperandRole::Phase
                )
                .with_requirement(
                    PulseRequirement::FramePhaseControl
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::ShiftPhase,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::ShiftPhase
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Frame
                )
                .with_operand(
                    PulseOperandRole::Phase
                )
                .with_requirement(
                    PulseRequirement::FramePhaseControl
                )
                .supporting_frame(),
            ),
            (
                PulseStandardOperation::Calibration,
                PulseOperationSchema::new(
                    PulseOperationName::standard(
                        PulseStandardOperation::Calibration
                    ),
                    PulseCardinality::AtLeastOne,
                )?
                .with_operand(
                    PulseOperandRole::Target
                )
                .with_operand(
                    PulseOperandRole::Calibration
                )
                .with_requirement(
                    PulseRequirement::Calibration
                )
                .supporting_calibration(),
            ),
        ];

        for (operation, schema) in schemas {
            self.standard_operations
                .insert(operation, schema);
        }

        Ok(())
    }
}

impl Default for PulseDialect {
    fn default() -> Self {
        Self::new()
            .expect(
                "the built-in Zamani Pulse Dialect must be valid",
            )
    }
}

// ============================================================================
// Dialect operation validation
// ============================================================================

/// Validation information supplied by an IR operation to the Pulse Dialect.
///
/// This structure deliberately contains only semantic information required by
/// the dialect contract.
///
/// It does not contain physical hardware information.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PulseOperationValidation {
    name: PulseOperationName,
    explicit_target_count: u64,
    has_waveform: bool,
    has_channel: bool,
    has_frame: bool,
    has_calibration: bool,
    has_duration: bool,
}

impl PulseOperationValidation {
    /// Creates validation information for an operation.
    #[must_use]
    pub const fn new(
        name: PulseOperationName,
        explicit_target_count: u64,
    ) -> Self {
        Self {
            name,
            explicit_target_count,
            has_waveform: false,
            has_channel: false,
            has_frame: false,
            has_calibration: false,
            has_duration: false,
        }
    }

    /// Marks waveform presence.
    #[must_use]
    pub const fn with_waveform(
        mut self,
    ) -> Self {
        self.has_waveform = true;
        self
    }

    /// Marks channel presence.
    #[must_use]
    pub const fn with_channel(
        mut self,
    ) -> Self {
        self.has_channel = true;
        self
    }

    /// Marks frame presence.
    #[must_use]
    pub const fn with_frame(
        mut self,
    ) -> Self {
        self.has_frame = true;
        self
    }

    /// Marks calibration presence.
    #[must_use]
    pub const fn with_calibration(
        mut self,
    ) -> Self {
        self.has_calibration = true;
        self
    }

    /// Marks duration presence.
    #[must_use]
    pub const fn with_duration(
        mut self,
    ) -> Self {
        self.has_duration = true;
        self
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &PulseOperationName {
        &self.name
    }

    /// Validates the operation against the supplied dialect.
    pub fn validate(
        &self,
        dialect: &PulseDialect,
    ) -> PulseDialectResult<()> {
        let schema = match &self.name {
            PulseOperationName::Standard(operation) => {
                dialect.operation(*operation)
            }

            PulseOperationName::Extension(extension) => {
                dialect
                    .extensions()
                    .get(*extension)
                    .and_then(|extension| {
                        extension.operations().iter().find(
                            |schema| {
                                schema.name()
                                    == &self.name
                            },
                        )
                    })
            }
        };

        let schema = schema.ok_or_else(|| {
            PulseDialectError::UnknownOperation(
                self.name.as_str(),
            )
        })?;

        if !schema
            .cardinality()
            .accepts(self.explicit_target_count)
        {
            return Err(
                PulseDialectError::TargetCardinalityMismatch {
                    operation: self.name.as_str(),
                    actual: self.explicit_target_count,
                },
            );
        }

        if schema.waveform_required
            && !self.has_waveform
        {
            return Err(
                PulseDialectError::MissingRequiredOperand {
                    operation: self.name.as_str(),
                    role: PulseOperandRole::Waveform,
                },
            );
        }

        if schema.timing_required
            && !self.has_duration
        {
            return Err(
                PulseDialectError::MissingRequiredOperand {
                    operation: self.name.as_str(),
                    role: PulseOperandRole::Duration,
                },
            );
        }

        if schema.frame_supported
            && matches!(
                schema.name(),
                PulseOperationName::Standard(
                    PulseStandardOperation::SetFrequency
                        | PulseStandardOperation::SetPhase
                        | PulseStandardOperation::ShiftPhase
                )
            )
            && !self.has_frame
        {
            return Err(
                PulseDialectError::MissingRequiredOperand {
                    operation: self.name.as_str(),
                    role: PulseOperandRole::Frame,
                },
            );
        }

        if schema.calibration_supported
            && matches!(
                schema.name(),
                PulseOperationName::Standard(
                    PulseStandardOperation::Calibration
                )
            )
            && !self.has_calibration
        {
            return Err(
                PulseDialectError::MissingRequiredOperand {
                    operation: self.name.as_str(),
                    role: PulseOperandRole::Calibration,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the Pulse Dialect contract.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum PulseDialectError {
    /// A dialect version is structurally invalid.
    InvalidVersion,

    /// An operation schema has invalid semantics.
    InvalidSchema {
        /// Operation name.
        operation: String,

        /// Reason.
        reason: &'static str,
    },

    /// Operation has an invalid target cardinality.
    InvalidCardinality,

    /// Operation is not registered.
    UnknownOperation(String),

    /// Extension is already registered.
    DuplicateExtension(ExtensionId),

    /// Operation is already registered.
    DuplicateOperation(String),

    /// Required operand is missing.
    MissingRequiredOperand {
        /// Operation name.
        operation: String,

        /// Missing semantic role.
        role: PulseOperandRole,
    },

    /// Explicit target count violates the operation schema.
    TargetCardinalityMismatch {
        /// Operation name.
        operation: String,

        /// Actual explicit target count.
        actual: u64,
    },

    /// Extension name is invalid.
    InvalidExtensionName {
        /// Reason.
        reason: &'static str,
    },
}

impl fmt::Display for PulseDialectError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidVersion => {
                formatter.write_str(
                    "invalid Pulse Dialect version",
                )
            }

            Self::InvalidSchema {
                operation,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid Pulse Dialect schema {operation}: {reason}"
                )
            }

            Self::InvalidCardinality => {
                formatter.write_str(
                    "pulse operation cardinality is invalid",
                )
            }

            Self::UnknownOperation(operation) => {
                write!(
                    formatter,
                    "unknown Pulse Dialect operation {operation}"
                )
            }

            Self::DuplicateExtension(id) => {
                write!(
                    formatter,
                    "duplicate Pulse Dialect extension {id}"
                )
            }

            Self::DuplicateOperation(operation) => {
                write!(
                    formatter,
                    "duplicate Pulse Dialect operation {operation}"
                )
            }

            Self::MissingRequiredOperand {
                operation,
                role,
            } => {
                write!(
                    formatter,
                    "Pulse Dialect operation {operation} is missing required operand {}",
                    role.as_str()
                )
            }

            Self::TargetCardinalityMismatch {
                operation,
                actual,
            } => {
                write!(
                    formatter,
                    "Pulse Dialect operation {operation} does not accept {actual} explicit targets"
                )
            }

            Self::InvalidExtensionName { reason } => {
                write!(
                    formatter,
                    "invalid Pulse Dialect extension name: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for PulseDialectError {}

/// Result type for Pulse Dialect operations.
pub type PulseDialectResult<T> =
    Result<T, PulseDialectError>;

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_extension_name(
    value: &str,
) -> PulseDialectResult<()> {
    if value.is_empty() {
        return Err(
            PulseDialectError::InvalidExtensionName {
                reason: "name cannot be empty",
            },
        );
    }

    if value
        .chars()
        .any(char::is_control)
    {
        return Err(
            PulseDialectError::InvalidExtensionName {
                reason:
                    "name cannot contain control characters",
            },
        );
    }

    if value.len() > 1024 {
        return Err(
            PulseDialectError::InvalidExtensionName {
                reason:
                    "name exceeds the explicit dialect metadata policy",
            },
        );
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_is_valid() {
        let version =
            PulseDialectVersion::current();

        assert_eq!(
            version.major(),
            PULSE_DIALECT_MAJOR
        );

        assert_eq!(
            version.minor(),
            PULSE_DIALECT_MINOR
        );

        assert_eq!(
            version.patch(),
            PULSE_DIALECT_PATCH
        );
    }

    #[test]
    fn current_version_supports_itself() {
        let version =
            PulseDialectVersion::current();

        assert!(
            version.supports(version)
        );
    }

    #[test]
    fn_future_major_requires_migration() {
        let current =
            PulseDialectVersion::current();

        let future =
            PulseDialectVersion::new(
                current.major() + 1,
                0,
                0,
            );

        assert!(
            current.requires_major_migration(
                future
            )
        );

        assert!(
            !current.supports(future)
        );
    }

    #[test]
    fn standard_operation_names_are_stable() {
        assert_eq!(
            PulseStandardOperation::Play.as_str(),
            "play"
        );

        assert_eq!(
            PulseStandardOperation::Capture.as_str(),
            "capture"
        );

        assert_eq!(
            PulseStandardOperation::Acquire.as_str(),
            "acquire"
        );

        assert_eq!(
            PulseStandardOperation::Delay.as_str(),
            "delay"
        );

        assert_eq!(
            PulseStandardOperation::Barrier.as_str(),
            "barrier"
        );
    }

    #[test]
    fn standard_operation_names_round_trip() {
        let operations = [
            PulseStandardOperation::Play,
            PulseStandardOperation::Capture,
            PulseStandardOperation::Acquire,
            PulseStandardOperation::Delay,
            PulseStandardOperation::Barrier,
            PulseStandardOperation::SetFrequency,
            PulseStandardOperation::SetPhase,
            PulseStandardOperation::ShiftPhase,
            PulseStandardOperation::Calibration,
        ];

        for operation in operations {
            assert_eq!(
                PulseStandardOperation::parse(
                    operation.as_str()
                ),
                Some(operation)
            );
        }
    }

    #[test]
    fn standard_operations_map_to_canonical_pulse_kinds() {
        assert_eq!(
            PulseStandardOperation::Play.pulse_kind(),
            PulseKind::Play
        );

        assert_eq!(
            PulseStandardOperation::Capture.pulse_kind(),
            PulseKind::Capture
        );

        assert_eq!(
            PulseStandardOperation::Acquire.pulse_kind(),
            PulseKind::Acquire
        );

        assert_eq!(
            PulseStandardOperation::Delay.pulse_kind(),
            PulseKind::Delay
        );
    }

    #[test]
    fn dialect_contains_all_standard_operations() {
        let dialect =
            PulseDialect::new().unwrap();

        assert_eq!(
            dialect.standard_operations().len(),
            9
        );

        for operation in [
            PulseStandardOperation::Play,
            PulseStandardOperation::Capture,
            PulseStandardOperation::Acquire,
            PulseStandardOperation::Delay,
            PulseStandardOperation::Barrier,
            PulseStandardOperation::SetFrequency,
            PulseStandardOperation::SetPhase,
            PulseStandardOperation::ShiftPhase,
            PulseStandardOperation::Calibration,
        ] {
            assert!(
                dialect.operation(operation).is_some(),
                "missing operation {:?}",
                operation
            );
        }
    }

    #[test]
    fn play_requires_waveform() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::Play
                ),
                1,
            )
            .with_duration();

        assert!(matches!(
            validation.validate(&dialect),
            Err(
                PulseDialectError::MissingRequiredOperand {
                    role: PulseOperandRole::Waveform,
                    ..
                }
            )
        ));
    }

    #[test]
    fn play_with_waveform_is_valid() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::Play
                ),
                1,
            )
            .with_waveform()
            .with_duration();

        assert!(
            validation.validate(&dialect).is_ok()
        );
    }

    #[test]
    fn delay_requires_duration() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::Delay
                ),
                1,
            );

        assert!(matches!(
            validation.validate(&dialect),
            Err(
                PulseDialectError::MissingRequiredOperand {
                    role: PulseOperandRole::Duration,
                    ..
                }
            )
        ));
    }

    #[test]
    fn delay_with_duration_is_valid() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::Delay
                ),
                1,
            )
            .with_duration();

        assert!(
            validation.validate(&dialect).is_ok()
        );
    }

    #[test]
    fn frame_operations_require_frame() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::SetPhase
                ),
                1,
            );

        assert!(matches!(
            validation.validate(&dialect),
            Err(
                PulseDialectError::MissingRequiredOperand {
                    role: PulseOperandRole::Frame,
                    ..
                }
            )
        ));
    }

    #[test]
    fn frame_operation_with_frame_is_valid() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::SetPhase
                ),
                1,
            )
            .with_frame();

        assert!(
            validation.validate(&dialect).is_ok()
        );
    }

    #[test]
    fn calibration_requires_calibration_reference() {
        let dialect =
            PulseDialect::new().unwrap();

        let validation =
            PulseOperationValidation::new(
                PulseOperationName::standard(
                    PulseStandardOperation::Calibration
                ),
                1,
            );

        assert!(matches!(
            validation.validate(&dialect),
            Err(
                PulseDialectError::MissingRequiredOperand {
                    role: PulseOperandRole::Calibration,
                    ..
                }
            )
        ));
    }

    #[test]
    fn extension_registry_is_deterministic() {
        let mut registry =
            PulseExtensionRegistry::new();

        let extension =
            PulseExtension::new(
                ExtensionId::new(10),
                "example".to_owned(),
                PulseDialectVersion::current(),
            )
            .unwrap();

        registry
            .register(extension)
            .unwrap();

        assert_eq!(
            registry.len(),
            1
        );

        assert!(
            registry.contains(
                ExtensionId::new(10)
            )
        );
    }

    #[test]
    fn extension_names_reject_control_characters() {
        let result =
            PulseExtension::new(
                ExtensionId::new(1),
                "bad\nname".to_owned(),
                PulseDialectVersion::current(),
            );

        assert!(matches!(
            result,
            Err(
                PulseDialectError::InvalidExtensionName { .. }
            )
        ));
    }

    #[test]
    fn operation_cardinality_is_not_hardware_sized() {
        let cardinality =
            PulseCardinality::AtLeastOne;

        assert!(cardinality.accepts(1));
        assert!(cardinality.accepts(2));
        assert!(cardinality.accepts(1_000_000));
    }

    #[test]
    fn global_cardinality_is_distinct() {
        assert!(
            PulseCardinality::Global.accepts(0)
        );

        assert!(
            !PulseCardinality::Global.accepts(1)
        );
    }

    #[test]
    fn operation_name_is_fully_qualified() {
        let name =
            PulseOperationName::standard(
                PulseStandardOperation::Play
            );

        assert_eq!(
            name.as_str(),
            "zamani.pulse.play"
        );
    }

    #[test]
    fn attribute_names_are_namespaced() {
        assert_eq!(
            PulseAttributeKey::Waveform
                .qualified_name(),
            "zamani.pulse.waveform"
        );

        assert_eq!(
            PulseAttributeKey::Duration
                .qualified_name(),
            "zamani.pulse.duration"
        );
    }

    #[test]
    fn dialect_validation_is_deterministic() {
        let first =
            PulseDialect::new().unwrap();

        let second =
            PulseDialect::new().unwrap();

        assert_eq!(
            first,
            second
        );
    }
}