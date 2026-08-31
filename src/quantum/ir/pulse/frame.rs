//! Zamani Quantum IR — Pulse Frame Semantics
//!
//! Canonical, hardware-independent representation of pulse/control frames.
//!
//! # Architectural role
//!
//! A frame is a semantic coordinate system for pulse-level control.
//!
//! A frame may describe:
//!
//! - an abstract frequency reference;
//! - an abstract phase reference;
//! - an optional abstract channel association;
//! - optional logical-qubit association;
//! - optional parent/composition relationship;
//! - deterministic frame-local metadata.
//!
//! A frame does NOT represent:
//!
//! - a physical oscillator;
//! - a microwave generator;
//! - a laser;
//! - a DAC;
//! - an ADC;
//! - a physical cable;
//! - a hardware port allocation;
//! - a vendor API;
//! - a calibration implementation;
//! - a scheduler;
//! - a logical-to-physical mapper;
//! - backend execution.
//!
//! Those concerns belong to downstream hardware, calibration, scheduling,
//! mapping, and backend subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! canonical Zamani Quantum IR
//!      |
//!      +-------------------------+
//!      |                         |
//!      v                         v
//!   pulse                     circuit
//!      |
//!      v
//! frame semantics
//!      |
//!      v
//! target capability resolution
//!      |
//!      v
//! calibration / lowering
//!      |
//!      v
//! physical control system
//! ```
//!
//! The frame layer therefore answers:
//!
//! > What control coordinate system does this pulse operation use?
//!
//! It does not answer:
//!
//! > Which physical oscillator implements that coordinate system?
//!
//! # OpenPulse-style semantics
//!
//! A frame conceptually carries:
//!
//! - frequency;
//! - phase;
//! - an abstract channel/port association;
//! - a time reference.
//!
//! This implementation intentionally keeps time progression itself outside
//! this file. The canonical timing/scheduling subsystem owns execution time.
//!
//! A frame may therefore be interpreted as:
//!
//! ```text
//! frame
//! ├── frequency
//! ├── phase
//! ├── channel reference
//! ├── logical targets
//! └── optional parent frame
//! ```
//!
//! # Universal-program principle
//!
//! Zamani programs are written once and compiled to available resources.
//!
//! This module therefore contains no fixed assumptions about:
//!
//! - number of qubits;
//! - number of frames;
//! - number of channels;
//! - number of devices;
//! - processor topology;
//! - vendor;
//! - architecture;
//! - pulse technology;
//! - oscillator technology.
//!
//! A frame identifier is not a machine-size limit.
//!
//! Resource limits, when required for security or compilation policy, must be
//! supplied explicitly by the caller or by `quantum::ir::limits`.
//!
//! # Logical-qubit integration
//!
//! The canonical qubit namespace is:
//!
//! ```rust
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! This module never creates another logical-qubit identifier type.
//!
//! A frame may optionally refer to logical qubits. Such a reference expresses
//! semantic association only. It does NOT perform logical-to-physical mapping.
//!
//! # Parameter integration
//!
//! Frequency and phase are represented by the canonical `Parameter` type.
//!
//! This permits:
//!
//! ```text
//! frequency = 5.0
//! frequency = drive_frequency
//! frequency = base_frequency + detuning
//!
//! phase = 0.0
//! phase = theta
//! phase = theta / 2
//! ```
//!
//! The parameter layer remains unit-neutral. The frame layer gives a parameter
//! semantic meaning as either frequency or phase.
//!
//! # Determinism
//!
//! Collections exposed by this module use deterministic ordering where
//! ordering has semantic or canonical-representation relevance.
//!
//! In particular:
//!
//! - targets are stored in deterministic order;
//! - metadata uses `BTreeMap`;
//! - validation does not depend on hash-map iteration;
//! - frame transformations preserve explicit ordering.
//!
//! # Safety
//!
//! - Rust 1.97 / 1.97.1.
//! - Rust 2021.
//! - Stable Rust.
//! - No nightly features.
//! - No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Dependency contract
//!
//! This file depends only on established foundational IR contracts:
//!
//! ```text
//! super::super::identity
//! super::super::parameter
//! super::super::qubit
//! ```
//!
//! It does not depend on:
//!
//! - pulse.rs;
//! - waveform.rs;
//! - channel.rs;
//! - timing.rs;
//! - schedule.rs;
//! - operation.rs;
//! - hardware;
//! - backend;
//! - frontend.
//!
//! Later modules may consume this module without requiring semantic changes to
//! the frame model.
//!
//! # Integration contract
//!
//! `identity.rs`
//!     Supplies `FrameId` and `ChannelId`.
//!
//! `parameter.rs`
//!     Supplies concrete and symbolic parameter semantics.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId` and `QubitRef`.
//!
//! `pulse.rs`
//!     References `FrameId` from pulse operations.
//!
//! `waveform.rs`
//!     Remains independent of frame implementation.
//!
//! `channel.rs`
//!     Defines abstract channel semantics referenced by `ChannelId`.
//!
//! `timing.rs`
//!     Owns temporal execution semantics.
//!
//! `schedule.rs`
//!     Determines when frame-affecting operations execute.
//!
//! `calibration.rs`
//!     Resolves abstract frame semantics to calibrated physical control.
//!
//! `mapping.rs`
//!     Resolves logical qubits to physical resources.
//!
//! `hardware/`
//!     Determines whether the requested frame can be realized.
//!
//! `serialization.rs`
//!     Serializes the deterministic structural representation.
//!
//! `hash.rs`
//!     May hash the structural representation.
//!
//! `provenance.rs`
//!     Tracks frame transformations and source lineage.
//!
//! # File completion guarantee
//!
//! This file owns the complete semantic frame contract:
//!
//! - frame identity reference;
//! - frequency semantics;
//! - phase semantics;
//! - channel association;
//! - logical-qubit association;
//! - parent-frame relationship;
//! - frame transformation semantics;
//! - metadata;
//! - validation policies;
//! - deterministic construction;
//! - checked parameter handling;
//! - structural equality;
//! - deterministic inspection;
//! - tests.
//!
//! Adding later pulse, timing, scheduling, hardware, or backend modules must
//! not require changing this semantic contract merely because those modules
//! are implemented.
//!
//! # Important distinction
//!
//! A frame is NOT a schedule.
//!
//! A frame is NOT a pulse.
//!
//! A frame is NOT a waveform.
//!
//! A frame is NOT a channel implementation.
//!
//! A frame is NOT a physical oscillator.
//!
//! A frame is a semantic coordinate system used by pulse/control operations.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::identity::{ChannelId, FrameId};
use super::super::parameter::Parameter;
use super::super::qubit::{QubitId, QubitRef};

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for pulse frames.
pub const FRAME_SCHEMA_ID: &str = "zamani.quantum.ir.pulse.frame";

/// Current semantic frame schema version.
pub const FRAME_SCHEMA_VERSION: u16 = 1;

/// Default maximum metadata key size in UTF-8 bytes.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default maximum metadata value size in UTF-8 bytes.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Default maximum metadata fields.
///
/// This is an input/resource-safety policy, not a quantum-machine limit.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum number of logical targets validated by one explicit policy.
///
/// This is not a semantic maximum. A caller may construct a larger frame under
/// a larger explicit policy.
pub const DEFAULT_MAX_TARGETS: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Result type for frame construction and validation.
pub type FrameResult<T> = Result<T, FrameError>;

// =============================================================================
// Frame frequency
// =============================================================================

/// Semantic frequency assigned to a frame.
///
/// The underlying [`Parameter`] remains unit-neutral. The consuming hardware
/// layer is responsible for determining the physical frequency unit and
/// realization.
///
/// A symbolic value is allowed:
///
/// ```text
/// FrameFrequency::new(Parameter::Symbol(...))
/// ```
///
/// No hardware-specific frequency range is enforced here.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameFrequency {
    parameter: Parameter,
}

impl FrameFrequency {
    /// Creates a frequency from a canonical parameter.
    pub fn new(parameter: Parameter) -> FrameResult<Self> {
        parameter
            .validate()
            .map_err(FrameError::Parameter)?;

        Ok(Self { parameter })
    }

    /// Creates a concrete finite frequency.
    pub fn constant(value: f64) -> FrameResult<Self> {
        let parameter =
            Parameter::constant(value).map_err(FrameError::Parameter)?;

        Self::new(parameter)
    }

    /// Creates a symbolic frequency.
    pub fn symbol<S: Into<String>>(name: S) -> FrameResult<Self> {
        let parameter =
            Parameter::symbol(name).map_err(FrameError::Parameter)?;

        Self::new(parameter)
    }

    /// Returns the underlying canonical parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.parameter
    }

    /// Returns whether the frequency is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.parameter.is_symbolic()
    }

    /// Returns a concrete frequency when directly represented as a constant.
    #[must_use]
    pub fn as_constant(&self) -> Option<f64> {
        self.parameter.as_constant()
    }

    /// Validates the frequency.
    pub fn validate(&self) -> FrameResult<()> {
        self.parameter
            .validate()
            .map_err(FrameError::Parameter)
    }

    /// Binds the frequency through an explicit resolver.
    pub fn bind<F>(&self, resolver: &F) -> FrameResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        self.parameter
            .bind(resolver)
            .map_err(FrameError::Parameter)
    }
}

impl From<Parameter> for FrameFrequency {
    fn from(parameter: Parameter) -> Self {
        Self { parameter }
    }
}

// =============================================================================
// Frame phase
// =============================================================================

/// Semantic phase assigned to a frame.
///
/// Phase is represented by the canonical unit-neutral [`Parameter`].
///
/// The frame layer deliberately does not impose a universal range such as:
///
/// ```text
/// 0 <= phase < 2π
/// ```
///
/// because symbolic phases, unwrapped phases, phase accumulation and backend
/// normalization are legitimate compiler representations.
///
/// Hardware-specific normalization belongs downstream.
#[derive(Debug, Clone, PartialEq)]
pub struct FramePhase {
    parameter: Parameter,
}

impl FramePhase {
    /// Creates a phase from a canonical parameter.
    pub fn new(parameter: Parameter) -> FrameResult<Self> {
        parameter
            .validate()
            .map_err(FrameError::Parameter)?;

        Ok(Self { parameter })
    }

    /// Creates a concrete finite phase.
    pub fn constant(value: f64) -> FrameResult<Self> {
        let parameter =
            Parameter::constant(value).map_err(FrameError::Parameter)?;

        Self::new(parameter)
    }

    /// Creates a symbolic phase.
    pub fn symbol<S: Into<String>>(name: S) -> FrameResult<Self> {
        let parameter =
            Parameter::symbol(name).map_err(FrameError::Parameter)?;

        Self::new(parameter)
    }

    /// Returns the underlying canonical parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.parameter
    }

    /// Returns whether the phase is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.parameter.is_symbolic()
    }

    /// Returns a concrete phase when directly represented as a constant.
    #[must_use]
    pub fn as_constant(&self) -> Option<f64> {
        self.parameter.as_constant()
    }

    /// Validates the phase.
    pub fn validate(&self) -> FrameResult<()> {
        self.parameter
            .validate()
            .map_err(FrameError::Parameter)
    }

    /// Binds the phase through an explicit resolver.
    pub fn bind<F>(&self, resolver: &F) -> FrameResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        self.parameter
            .bind(resolver)
            .map_err(FrameError::Parameter)
    }
}

impl From<Parameter> for FramePhase {
    fn from(parameter: Parameter) -> Self {
        Self { parameter }
    }
}

// =============================================================================
// Frame target
// =============================================================================

/// Semantic target associated with a frame.
///
/// The target is optional because some frames are channel-scoped rather than
/// directly associated with one logical qubit.
///
/// The canonical qubit namespace comes from
/// [`crate::quantum::ir::qubit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FrameTarget {
    /// Logical qubit association.
    Logical(QubitId),

    /// Explicit physical-qubit association at a downstream compilation
    /// boundary.
    ///
    /// This variant is intentionally not present here because canonical frame
    /// semantics should remain logical by default. Physical placement belongs
    /// to mapping.
    ///
    /// Keeping the target type logical-only prevents accidental coupling of
    /// source-level frame semantics to hardware allocation.
}

impl FrameTarget {
    /// Creates a logical frame target.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical_qubit(self) -> QubitId {
        match self {
            Self::Logical(qubit) => qubit,
        }
    }

    /// Converts a canonical [`QubitRef`] when it is logical.
    ///
    /// Physical references are rejected because physical placement is a
    /// downstream mapping concern.
    pub fn from_qubit_ref(reference: QubitRef) -> FrameResult<Self> {
        match reference {
            QubitRef::Logical(qubit) => Ok(Self::Logical(qubit)),
            QubitRef::Physical(qubit) => {
                Err(FrameError::PhysicalTargetNotAllowed { qubit })
            }
        }
    }
}

impl From<QubitId> for FrameTarget {
    fn from(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }
}

// =============================================================================
// Frame metadata
// =============================================================================

/// Deterministic frame metadata.
///
/// Metadata is descriptive/annotative and must not be used as a substitute for
/// semantic fields.
///
/// Metadata keys and values are UTF-8 strings and are bounded by an explicit
/// policy during validated construction.
pub type FrameMetadata = BTreeMap<String, String>;

/// Explicit metadata validation policy.
///
/// This is a resource/input policy, not a quantum-machine-size limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMetadataPolicy {
    /// Maximum key size in UTF-8 bytes.
    pub max_key_bytes: usize,

    /// Maximum value size in UTF-8 bytes.
    pub max_value_bytes: usize,

    /// Maximum number of metadata entries.
    pub max_fields: usize,
}

impl Default for FrameMetadataPolicy {
    fn default() -> Self {
        Self {
            max_key_bytes: DEFAULT_MAX_METADATA_KEY_BYTES,
            max_value_bytes: DEFAULT_MAX_METADATA_VALUE_BYTES,
            max_fields: DEFAULT_MAX_METADATA_FIELDS,
        }
    }
}

impl FrameMetadataPolicy {
    /// Creates an explicit metadata policy.
    #[must_use]
    pub const fn new(
        max_key_bytes: usize,
        max_value_bytes: usize,
        max_fields: usize,
    ) -> Self {
        Self {
            max_key_bytes,
            max_value_bytes,
            max_fields,
        }
    }

    /// Validates the policy itself.
    pub const fn validate(self) -> FrameResult<()> {
        if self.max_key_bytes == 0 {
            return Err(FrameError::InvalidMetadataPolicy {
                reason: "maximum metadata key size cannot be zero",
            });
        }

        if self.max_value_bytes == 0 {
            return Err(FrameError::InvalidMetadataPolicy {
                reason: "maximum metadata value size cannot be zero",
            });
        }

        if self.max_fields == 0 {
            return Err(FrameError::InvalidMetadataPolicy {
                reason: "maximum metadata field count cannot be zero",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Frame target validation policy
// =============================================================================

/// Explicit target-validation policy.
///
/// No target limit is embedded in the semantic frame itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameTargetPolicy {
    /// Maximum number of logical targets accepted by this validation call.
    pub max_targets: usize,

    /// Whether duplicate logical targets are rejected.
    pub reject_duplicates: bool,
}

impl Default for FrameTargetPolicy {
    fn default() -> Self {
        Self {
            max_targets: DEFAULT_MAX_TARGETS,
            reject_duplicates: true,
        }
    }
}

impl FrameTargetPolicy {
    /// Creates an explicit target policy.
    #[must_use]
    pub const fn new(
        max_targets: usize,
        reject_duplicates: bool,
    ) -> Self {
        Self {
            max_targets,
            reject_duplicates,
        }
    }

    /// Creates an effectively unbounded target policy for callers that already
    /// have an external resource budget.
    ///
    /// `usize::MAX` is a policy value, not a statement that memory is infinite.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_targets: usize::MAX,
            reject_duplicates: true,
        }
    }

    /// Validates the policy.
    pub const fn validate(self) -> FrameResult<()> {
        if self.max_targets == 0 {
            return Err(FrameError::InvalidTargetPolicy {
                reason: "maximum target count cannot be zero",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Frame transformation
// =============================================================================

/// Semantic frame transformation.
///
/// Transformations describe changes to the frame coordinate system.
///
/// They do not directly schedule hardware operations.
///
/// For example:
///
/// ```text
/// shift frequency by Δf
/// shift phase by Δφ
/// ```
///
/// may later be lowered to a hardware-specific instruction sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameTransform {
    /// Replace the frame frequency with a new semantic value.
    SetFrequency(FrameFrequency),

    /// Replace the frame phase with a new semantic value.
    SetPhase(FramePhase),

    /// Add a semantic frequency delta.
    ShiftFrequency(Parameter),

    /// Add a semantic phase delta.
    ShiftPhase(Parameter),

    /// Associate the frame with an abstract channel.
    SetChannel(Option<ChannelId>),
}

impl FrameTransform {
    /// Validates the transformation.
    pub fn validate(&self) -> FrameResult<()> {
        match self {
            Self::SetFrequency(frequency) => frequency.validate(),

            Self::SetPhase(phase) => phase.validate(),

            Self::ShiftFrequency(parameter)
            | Self::ShiftPhase(parameter) => parameter
                .validate()
                .map_err(FrameError::Parameter),

            Self::SetChannel(_) => Ok(()),
        }
    }

    /// Returns whether the transformation contains symbolic parameters.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::SetFrequency(frequency) => frequency.is_symbolic(),

            Self::SetPhase(phase) => phase.is_symbolic(),

            Self::ShiftFrequency(parameter)
            | Self::ShiftPhase(parameter) => parameter.is_symbolic(),

            Self::SetChannel(_) => false,
        }
    }
}

// =============================================================================
// Frame definition
// =============================================================================

/// Canonical semantic pulse/control frame.
///
/// A frame provides a coordinate system for pulse operations.
///
/// The frame is intentionally independent from:
///
/// - physical oscillators;
/// - physical ports;
/// - DACs;
/// - ADCs;
/// - hardware calibration;
/// - routing;
/// - scheduling.
///
/// # Identity
///
/// [`FrameId`] is a stable IR identity supplied by the owning program/session.
///
/// This type does not allocate global identifiers.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    id: FrameId,

    /// Optional human-readable semantic name.
    name: Option<String>,

    /// Frame frequency.
    frequency: Option<FrameFrequency>,

    /// Frame phase.
    phase: Option<FramePhase>,

    /// Optional abstract channel association.
    channel: Option<ChannelId>,

    /// Logical qubits semantically associated with this frame.
    ///
    /// This is not a physical placement.
    targets: Vec<FrameTarget>,

    /// Optional parent frame.
    ///
    /// A parent relationship expresses semantic inheritance/composition.
    /// Cycles must be rejected by the program-level validator.
    parent: Option<FrameId>,

    /// Whether this frame is declared as immutable after definition.
    ///
    /// This is a semantic declaration useful to downstream passes; it does
    /// not itself enforce scheduler behavior.
    immutable: bool,

    /// Deterministic descriptive metadata.
    metadata: FrameMetadata,
}

impl Frame {
    /// Creates an empty frame with no hardware assumptions.
    #[must_use]
    pub const fn new(id: FrameId) -> Self {
        Self {
            id,
            name: None,
            frequency: None,
            phase: None,
            channel: None,
            targets: Vec::new(),
            parent: None,
            immutable: false,
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a frame with a concrete frequency and phase.
    pub fn with_frequency_phase(
        id: FrameId,
        frequency: FrameFrequency,
        phase: FramePhase,
    ) -> FrameResult<Self> {
        frequency.validate()?;
        phase.validate()?;

        Ok(Self {
            id,
            name: None,
            frequency: Some(frequency),
            phase: Some(phase),
            channel: None,
            targets: Vec::new(),
            parent: None,
            immutable: false,
            metadata: BTreeMap::new(),
        })
    }

    /// Returns the stable frame identity.
    #[must_use]
    pub const fn id(&self) -> FrameId {
        self.id
    }

    /// Sets the optional semantic name.
    pub fn set_name<S: Into<String>>(
        &mut self,
        name: S,
    ) -> FrameResult<()> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(FrameError::EmptyName);
        }

        if name.contains('\0') {
            return Err(FrameError::InvalidName {
                reason: "frame name contains a NUL character",
            });
        }

        self.name = Some(name);
        Ok(())
    }

    /// Returns the semantic name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the frame frequency.
    pub fn set_frequency(
        &mut self,
        frequency: FrameFrequency,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;
        frequency.validate()?;
        self.frequency = Some(frequency);
        Ok(())
    }

    /// Returns the frame frequency.
    #[must_use]
    pub fn frequency(&self) -> Option<&FrameFrequency> {
        self.frequency.as_ref()
    }

    /// Sets the frame phase.
    pub fn set_phase(
        &mut self,
        phase: FramePhase,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;
        phase.validate()?;
        self.phase = Some(phase);
        Ok(())
    }

    /// Returns the frame phase.
    #[must_use]
    pub fn phase(&self) -> Option<&FramePhase> {
        self.phase.as_ref()
    }

    /// Sets the abstract channel association.
    ///
    /// This stores only `ChannelId`. It does not allocate or validate a
    /// physical channel.
    pub fn set_channel(
        &mut self,
        channel: Option<ChannelId>,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;
        self.channel = channel;
        Ok(())
    }

    /// Returns the abstract channel association.
    #[must_use]
    pub const fn channel(&self) -> Option<ChannelId> {
        self.channel
    }

    /// Sets the logical targets from a caller-provided collection.
    ///
    /// Targets are copied into deterministic sorted order.
    pub fn set_targets<I>(
        &mut self,
        targets: I,
        policy: FrameTargetPolicy,
    ) -> FrameResult<()>
    where
        I: IntoIterator<Item = FrameTarget>,
    {
        self.ensure_mutable()?;
        policy.validate()?;

        let mut collected = Vec::new();

        for target in targets {
            if collected.len() >= policy.max_targets {
                return Err(FrameError::TargetLimitExceeded {
                    limit: policy.max_targets,
                });
            }

            collected.push(target);
        }

        collected.sort();

        if policy.reject_duplicates {
            for pair in collected.windows(2) {
                if pair[0] == pair[1] {
                    return Err(FrameError::DuplicateTarget {
                        target: pair[0],
                    });
                }
            }
        }

        self.targets = collected;

        Ok(())
    }

    /// Adds one logical target.
    ///
    /// Duplicate detection is always enabled for this operation.
    pub fn add_target(
        &mut self,
        target: FrameTarget,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;

        match self.targets.binary_search(&target) {
            Ok(_) => Err(FrameError::DuplicateTarget { target }),

            Err(index) => {
                self.targets.insert(index, target);
                Ok(())
            }
        }
    }

    /// Adds one canonical logical qubit target.
    pub fn add_qubit(
        &mut self,
        qubit: QubitId,
    ) -> FrameResult<()> {
        self.add_target(FrameTarget::Logical(qubit))
    }

    /// Returns all targets in deterministic order.
    #[must_use]
    pub fn targets(&self) -> &[FrameTarget] {
        &self.targets
    }

    /// Returns the number of logical targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether the frame has no explicit logical targets.
    #[must_use]
    pub fn is_targetless(&self) -> bool {
        self.targets.is_empty()
    }

    /// Sets the optional parent frame.
    ///
    /// This method rejects direct self-parenting. Longer cycles must be
    /// rejected by program-level validation because that requires access to the
    /// complete frame graph.
    pub fn set_parent(
        &mut self,
        parent: Option<FrameId>,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;

        if parent == Some(self.id) {
            return Err(FrameError::SelfParenting);
        }

        self.parent = parent;
        Ok(())
    }

    /// Returns the optional parent frame.
    #[must_use]
    pub const fn parent(&self) -> Option<FrameId> {
        self.parent
    }

    /// Marks this frame immutable.
    ///
    /// Once set, semantic mutation through the normal setters is rejected.
    pub fn freeze(&mut self) {
        self.immutable = true;
    }

    /// Returns whether this frame is immutable.
    #[must_use]
    pub const fn is_immutable(&self) -> bool {
        self.immutable
    }

    /// Adds metadata using the default policy.
    pub fn insert_metadata<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) -> FrameResult<Option<String>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.insert_metadata_with_policy(
            key,
            value,
            FrameMetadataPolicy::default(),
        )
    }

    /// Adds metadata using an explicit policy.
    pub fn insert_metadata_with_policy<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
        policy: FrameMetadataPolicy,
    ) -> FrameResult<Option<String>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.ensure_mutable()?;
        policy.validate()?;

        let key = key.into();
        let value = value.into();

        validate_metadata_entry(&key, &value, policy)?;

        if !self.metadata.contains_key(&key)
            && self.metadata.len() >= policy.max_fields
        {
            return Err(FrameError::MetadataLimitExceeded {
                limit: policy.max_fields,
            });
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &FrameMetadata {
        &self.metadata
    }

    /// Removes metadata by key.
    pub fn remove_metadata(
        &mut self,
        key: &str,
    ) -> FrameResult<Option<String>> {
        self.ensure_mutable()?;
        Ok(self.metadata.remove(key))
    }

    /// Applies one semantic frame transformation.
    pub fn apply_transform(
        &mut self,
        transform: FrameTransform,
    ) -> FrameResult<()> {
        self.ensure_mutable()?;
        transform.validate()?;

        match transform {
            FrameTransform::SetFrequency(frequency) => {
                self.frequency = Some(frequency);
            }

            FrameTransform::SetPhase(phase) => {
                self.phase = Some(phase);
            }

            FrameTransform::ShiftFrequency(delta) => {
                self.frequency = Some(add_parameter(
                    self.frequency
                        .as_ref()
                        .map(FrameFrequency::parameter)
                        .cloned()
                        .unwrap_or_else(|| {
                            Parameter::Constant(0.0)
                        }),
                    delta,
                )?);
            }

            FrameTransform::ShiftPhase(delta) => {
                self.phase = Some(add_phase_parameter(
                    self.phase
                        .as_ref()
                        .map(FramePhase::parameter)
                        .cloned()
                        .unwrap_or_else(|| {
                            Parameter::Constant(0.0)
                        }),
                    delta,
                )?);
            }

            FrameTransform::SetChannel(channel) => {
                self.channel = channel;
            }
        }

        Ok(())
    }

    /// Returns whether any frame parameter is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.frequency
            .as_ref()
            .is_some_and(FrameFrequency::is_symbolic)
            || self
                .phase
                .as_ref()
                .is_some_and(FramePhase::is_symbolic)
    }

    /// Validates using default policies.
    pub fn validate(&self) -> FrameResult<()> {
        self.validate_with_policies(
            FrameTargetPolicy::default(),
            FrameMetadataPolicy::default(),
        )
    }

    /// Validates using explicit policies.
    ///
    /// This validates all properties that can be determined from the frame
    /// alone. Graph-wide parent-cycle validation belongs to the program-level
    /// validator.
    pub fn validate_with_policies(
        &self,
        target_policy: FrameTargetPolicy,
        metadata_policy: FrameMetadataPolicy,
    ) -> FrameResult<()> {
        target_policy.validate()?;
        metadata_policy.validate()?;

        if self.name.as_deref().is_some_and(str::is_empty) {
            return Err(FrameError::EmptyName);
        }

        if self.name.as_deref().is_some_and(|name| {
            name.contains('\0')
        }) {
            return Err(FrameError::InvalidName {
                reason: "frame name contains a NUL character",
            });
        }

        if self.targets.len() > target_policy.max_targets {
            return Err(FrameError::TargetLimitExceeded {
                limit: target_policy.max_targets,
            });
        }

        if target_policy.reject_duplicates {
            for pair in self.targets.windows(2) {
                if pair[0] >= pair[1] {
                    return Err(FrameError::TargetsNotCanonical);
                }
            }
        }

        if let Some(frequency) = &self.frequency {
            frequency.validate()?;
        }

        if let Some(phase) = &self.phase {
            phase.validate()?;
        }

        if self.parent == Some(self.id) {
            return Err(FrameError::SelfParenting);
        }

        if self.metadata.len() > metadata_policy.max_fields {
            return Err(FrameError::MetadataLimitExceeded {
                limit: metadata_policy.max_fields,
            });
        }

        for (key, value) in &self.metadata {
            validate_metadata_entry(key, value, metadata_policy)?;
        }

        Ok(())
    }

    /// Returns a deterministic structural summary.
    ///
    /// This is intended for diagnostics, hashing adapters and tests. It is not
    /// a serialization format.
    #[must_use]
    pub fn structural_summary(&self) -> FrameStructuralSummary {
        FrameStructuralSummary {
            id: self.id,
            has_name: self.name.is_some(),
            has_frequency: self.frequency.is_some(),
            has_phase: self.phase.is_some(),
            channel: self.channel,
            target_count: self.targets.len(),
            parent: self.parent,
            immutable: self.immutable,
            metadata_count: self.metadata.len(),
        }
    }

    fn ensure_mutable(&self) -> FrameResult<()> {
        if self.immutable {
            return Err(FrameError::Immutable);
        }

        Ok(())
    }
}

// =============================================================================
// Structural summary
// =============================================================================

/// Deterministic summary of frame structure.
///
/// This type intentionally excludes free-form metadata contents and parameter
/// expressions so that it remains lightweight for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameStructuralSummary {
    /// Frame identity.
    pub id: FrameId,

    /// Whether a semantic name exists.
    pub has_name: bool,

    /// Whether a frequency is present.
    pub has_frequency: bool,

    /// Whether a phase is present.
    pub has_phase: bool,

    /// Optional channel association.
    pub channel: Option<ChannelId>,

    /// Number of logical targets.
    pub target_count: usize,

    /// Optional parent frame.
    pub parent: Option<FrameId>,

    /// Whether the frame has been frozen.
    pub immutable: bool,

    /// Number of metadata fields.
    pub metadata_count: usize,
}

// =============================================================================
// Parameter helpers
// =============================================================================

fn add_parameter(
    left: Parameter,
    right: Parameter,
) -> FrameResult<FrameFrequency> {
    let expression =
        super::super::parameter::ParameterExpression::Add(
            Box::new(left),
            Box::new(right),
        );

    let parameter =
        Parameter::expression(expression)
            .map_err(FrameError::Parameter)?;

    FrameFrequency::new(parameter)
}

fn add_phase_parameter(
    left: Parameter,
    right: Parameter,
) -> FrameResult<FramePhase> {
    let expression =
        super::super::parameter::ParameterExpression::Add(
            Box::new(left),
            Box::new(right),
        );

    let parameter =
        Parameter::expression(expression)
            .map_err(FrameError::Parameter)?;

    FramePhase::new(parameter)
}

// =============================================================================
// Metadata validation
// =============================================================================

fn validate_metadata_entry(
    key: &str,
    value: &str,
    policy: FrameMetadataPolicy,
) -> FrameResult<()> {
    if key.is_empty() {
        return Err(FrameError::EmptyMetadataKey);
    }

    if key.as_bytes().len() > policy.max_key_bytes {
        return Err(FrameError::MetadataKeyTooLarge {
            size: key.as_bytes().len(),
            limit: policy.max_key_bytes,
        });
    }

    if value.as_bytes().len() > policy.max_value_bytes {
        return Err(FrameError::MetadataValueTooLarge {
            size: value.as_bytes().len(),
            limit: policy.max_value_bytes,
        });
    }

    if key.contains('\0') || value.contains('\0') {
        return Err(FrameError::MetadataContainsNul);
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by frame-local construction or validation.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameError {
    /// Underlying canonical parameter validation failed.
    Parameter(crate::quantum::ir::errors::IrError),

    /// A frame name was empty.
    EmptyName,

    /// A frame name was structurally invalid.
    InvalidName {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// A frame attempted to parent itself.
    SelfParenting,

    /// A physical qubit was supplied where canonical frame semantics require a
    /// logical qubit.
    PhysicalTargetNotAllowed {
        /// Rejected physical qubit.
        qubit: crate::quantum::ir::qubit::PhysicalQubitId,
    },

    /// A duplicate target was supplied.
    DuplicateTarget {
        /// Duplicate target.
        target: FrameTarget,
    },

    /// Targets were not in canonical deterministic order.
    TargetsNotCanonical,

    /// Explicit target policy was exceeded.
    TargetLimitExceeded {
        /// Configured policy limit.
        limit: usize,
    },

    /// Metadata policy itself is invalid.
    InvalidMetadataPolicy {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// Metadata field count exceeds policy.
    MetadataLimitExceeded {
        /// Configured policy limit.
        limit: usize,
    },

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata key is too large.
    MetadataKeyTooLarge {
        /// Actual byte size.
        size: usize,

        /// Configured limit.
        limit: usize,
    },

    /// Metadata value is too large.
    MetadataValueTooLarge {
        /// Actual byte size.
        size: usize,

        /// Configured limit.
        limit: usize,
    },

    /// Metadata contains a NUL byte.
    MetadataContainsNul,

    /// Mutation was attempted after freezing.
    Immutable,
}

impl fmt::Display for FrameError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Parameter(error) => {
                write!(formatter, "invalid frame parameter: {error}")
            }

            Self::EmptyName => {
                formatter.write_str("frame name cannot be empty")
            }

            Self::InvalidName { reason } => {
                write!(formatter, "invalid frame name: {reason}")
            }

            Self::SelfParenting => {
                formatter.write_str(
                    "a frame cannot directly parent itself",
                )
            }

            Self::PhysicalTargetNotAllowed { qubit } => {
                write!(
                    formatter,
                    "physical qubit `{qubit}` cannot be used as a \
                     canonical frame target"
                )
            }

            Self::DuplicateTarget { target } => {
                write!(
                    formatter,
                    "frame target `{target:?}` is duplicated"
                )
            }

            Self::TargetsNotCanonical => {
                formatter.write_str(
                    "frame targets are not in canonical deterministic order",
                )
            }

            Self::TargetLimitExceeded { limit } => {
                write!(
                    formatter,
                    "frame target count exceeds explicit policy limit {limit}"
                )
            }

            Self::InvalidMetadataPolicy { reason } => {
                write!(
                    formatter,
                    "invalid frame metadata policy: {reason}"
                )
            }

            Self::MetadataLimitExceeded { limit } => {
                write!(
                    formatter,
                    "frame metadata count exceeds explicit policy limit {limit}"
                )
            }

            Self::EmptyMetadataKey => {
                formatter.write_str("frame metadata key cannot be empty")
            }

            Self::MetadataKeyTooLarge { size, limit } => {
                write!(
                    formatter,
                    "frame metadata key is {size} bytes; maximum is {limit}"
                )
            }

            Self::MetadataValueTooLarge { size, limit } => {
                write!(
                    formatter,
                    "frame metadata value is {size} bytes; maximum is {limit}"
                )
            }

            Self::MetadataContainsNul => {
                formatter.write_str(
                    "frame metadata cannot contain NUL characters",
                )
            }

            Self::Immutable => {
                formatter.write_str(
                    "frame is immutable and cannot be modified",
                )
            }
        }
    }
}

impl std::error::Error for FrameError {}

// =============================================================================
// Display implementations
// =============================================================================

impl fmt::Display for FrameFrequency {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}", self.parameter)
    }
}

impl fmt::Display for FramePhase {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}", self.parameter)
    }
}

impl fmt::Display for FrameTarget {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Logical(qubit) => write!(formatter, "{qubit}"),
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "frame {}", self.id)?;

        if let Some(name) = &self.name {
            write!(formatter, " ({name})")?;
        }

        if let Some(frequency) = &self.frequency {
            write!(formatter, " frequency={frequency}")?;
        }

        if let Some(phase) = &self.phase {
            write!(formatter, " phase={phase}")?;
        }

        if let Some(channel) = self.channel {
            write!(formatter, " channel={channel}")?;
        }

        if !self.targets.is_empty() {
            write!(formatter, " targets=[")?;

            for (index, target) in self.targets.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{target}")?;
            }

            formatter.write_str("]")?;
        }

        if let Some(parent) = self.parent {
            write!(formatter, " parent={parent}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn frame_id(value: u64) -> FrameId {
        FrameId::new(value)
    }

    fn qubit(value: usize) -> QubitId {
        QubitId::new(value)
    }

    #[test]
    fn frame_can_be_created_without_hardware_information() {
        let frame = Frame::new(frame_id(1));

        assert_eq!(frame.id(), frame_id(1));
        assert!(frame.frequency().is_none());
        assert!(frame.phase().is_none());
        assert!(frame.channel().is_none());
        assert!(frame.targets().is_empty());
        assert!(frame.parent().is_none());

        assert!(frame.validate().is_ok());
    }

    #[test]
    fn frame_frequency_accepts_concrete_parameter() {
        let frequency =
            FrameFrequency::constant(5.0).expect("valid frequency");

        assert_eq!(frequency.as_constant(), Some(5.0));
        assert!(!frequency.is_symbolic());
        assert!(frequency.validate().is_ok());
    }

    #[test]
    fn frame_frequency_accepts_symbolic_parameter() {
        let frequency =
            FrameFrequency::symbol("drive_frequency")
                .expect("valid symbol");

        assert!(frequency.is_symbolic());
        assert_eq!(
            frequency.bind(&|name| {
                if name == "drive_frequency" {
                    Some(5.0)
                } else {
                    None
                }
            })
            .expect("binding should succeed"),
            5.0
        );
    }

    #[test]
    fn frame_phase_accepts_symbolic_parameter() {
        let phase =
            FramePhase::symbol("theta").expect("valid phase");

        assert!(phase.is_symbolic());
        assert_eq!(
            phase
                .bind(&|name| {
                    if name == "theta" {
                        Some(0.25)
                    } else {
                        None
                    }
                })
                .expect("binding should succeed"),
            0.25
        );
    }

    #[test]
    fn frame_uses_canonical_qubit_id() {
        let mut frame = Frame::new(frame_id(2));

        frame
            .add_qubit(qubit(17))
            .expect("first target should succeed");

        assert_eq!(
            frame.targets(),
            &[FrameTarget::Logical(qubit(17))]
        );
    }

    #[test]
    fn frame_targets_are_deterministically_sorted() {
        let mut frame = Frame::new(frame_id(3));

        frame
            .set_targets(
                [
                    FrameTarget::Logical(qubit(9)),
                    FrameTarget::Logical(qubit(2)),
                    FrameTarget::Logical(qubit(5)),
                ],
                FrameTargetPolicy::unlimited(),
            )
            .expect("targets should be accepted");

        assert_eq!(
            frame.targets(),
            &[
                FrameTarget::Logical(qubit(2)),
                FrameTarget::Logical(qubit(5)),
                FrameTarget::Logical(qubit(9)),
            ]
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let mut frame = Frame::new(frame_id(4));

        let result = frame.set_targets(
            [
                FrameTarget::Logical(qubit(1)),
                FrameTarget::Logical(qubit(1)),
            ],
            FrameTargetPolicy::unlimited(),
        );

        assert!(matches!(
            result,
            Err(FrameError::DuplicateTarget { .. })
        ));
    }

    #[test]
    fn physical_qubit_is_rejected_from_canonical_frame_target() {
        let physical =
            crate::quantum::ir::qubit::PhysicalQubitId::new(7);

        let result =
            FrameTarget::from_qubit_ref(QubitRef::Physical(physical));

        assert!(matches!(
            result,
            Err(FrameError::PhysicalTargetNotAllowed { .. })
        ));
    }

    #[test]
    fn logical_qubit_ref_is_accepted() {
        let logical = qubit(7);

        let target =
            FrameTarget::from_qubit_ref(QubitRef::Logical(logical))
                .expect("logical target should be accepted");

        assert_eq!(
            target,
            FrameTarget::Logical(logical)
        );
    }

    #[test]
    fn direct_self_parenting_is_rejected() {
        let id = frame_id(5);
        let mut frame = Frame::new(id);

        let result = frame.set_parent(Some(id));

        assert!(matches!(
            result,
            Err(FrameError::SelfParenting)
        ));
    }

    #[test]
    fn parent_frame_is_allowed() {
        let mut frame = Frame::new(frame_id(6));

        frame
            .set_parent(Some(frame_id(10)))
            .expect("different parent should be accepted");

        assert_eq!(frame.parent(), Some(frame_id(10)));
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut frame = Frame::new(frame_id(7));

        frame
            .insert_metadata("z", "last")
            .expect("metadata should succeed");

        frame
            .insert_metadata("a", "first")
            .expect("metadata should succeed");

        let keys = frame
            .metadata()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec!["a".to_owned(), "z".to_owned()]
        );
    }

    #[test]
    fn metadata_policy_is_enforced() {
        let mut frame = Frame::new(frame_id(8));

        let policy = FrameMetadataPolicy::new(2, 2, 1);

        assert!(frame
            .insert_metadata_with_policy(
                "abc",
                "x",
                policy
            )
            .is_err());

        frame
            .insert_metadata_with_policy(
                "a",
                "x",
                policy
            )
            .expect("first metadata entry should succeed");

        let second =
            frame.insert_metadata_with_policy(
                "b",
                "y",
                policy,
            );

        assert!(matches!(
            second,
            Err(FrameError::MetadataLimitExceeded { .. })
        ));
    }

    #[test]
    fn symbolic_frame_detection_works() {
        let mut frame = Frame::new(frame_id(9));

        frame
            .set_frequency(
                FrameFrequency::symbol("f")
                    .expect("valid frequency"),
            )
            .expect("frequency should be set");

        assert!(frame.is_symbolic());
    }

    #[test]
    fn concrete_frame_is_not_symbolic() {
        let mut frame = Frame::new(frame_id(10));

        frame
            .set_frequency(
                FrameFrequency::constant(5.0)
                    .expect("valid frequency"),
            )
            .expect("frequency should be set");

        frame
            .set_phase(
                FramePhase::constant(0.0)
                    .expect("valid phase"),
            )
            .expect("phase should be set");

        assert!(!frame.is_symbolic());
    }

    #[test]
    fn phase_shift_creates_symbolic_expression_when_needed() {
        let mut frame = Frame::new(frame_id(11));

        frame
            .set_phase(
                FramePhase::symbol("theta")
                    .expect("valid phase"),
            )
            .expect("phase should be set");

        frame
            .apply_transform(
                FrameTransform::ShiftPhase(
                    Parameter::constant(0.5)
                        .expect("valid constant"),
                ),
            )
            .expect("phase shift should succeed");

        let phase = frame
            .phase()
            .expect("phase should exist");

        assert!(phase.is_symbolic());
    }

    #[test]
    fn frequency_shift_creates_expression() {
        let mut frame = Frame::new(frame_id(12));

        frame
            .set_frequency(
                FrameFrequency::constant(5.0)
                    .expect("valid frequency"),
            )
            .expect("frequency should be set");

        frame
            .apply_transform(
                FrameTransform::ShiftFrequency(
                    Parameter::constant(0.1)
                        .expect("valid constant"),
                ),
            )
            .expect("frequency shift should succeed");

        assert!(frame.frequency().is_some());
        assert!(frame.frequency().unwrap().is_symbolic());
    }

    #[test]
    fn channel_is_only_an_abstract_reference() {
        let mut frame = Frame::new(frame_id(13));
        let channel = ChannelId::new(42);

        frame
            .set_channel(Some(channel))
            .expect("channel association should succeed");

        assert_eq!(frame.channel(), Some(channel));
    }

    #[test]
    fn freeze_prevents_mutation() {
        let mut frame = Frame::new(frame_id(14));

        frame.freeze();

        assert!(frame.is_immutable());

        let result = frame.set_phase(
            FramePhase::constant(0.0)
                .expect("valid phase"),
        );

        assert!(matches!(
            result,
            Err(FrameError::Immutable)
        ));
    }

    #[test]
    fn structural_summary_is_deterministic() {
        let mut frame = Frame::new(frame_id(15));

        frame
            .set_frequency(
                FrameFrequency::constant(5.0)
                    .expect("valid frequency"),
            )
            .expect("frequency should be set");

        frame
            .add_qubit(qubit(0))
            .expect("target should succeed");

        frame
            .insert_metadata("purpose", "drive")
            .expect("metadata should succeed");

        let summary = frame.structural_summary();

        assert_eq!(summary.id, frame_id(15));
        assert!(summary.has_frequency);
        assert!(!summary.has_phase);
        assert_eq!(summary.target_count, 1);
        assert_eq!(summary.metadata_count, 1);
    }

    #[test]
    fn validation_accepts_large_policy_without_architectural_limit() {
        let mut frame = Frame::new(frame_id(16));

        frame
            .set_targets(
                [FrameTarget::Logical(qubit(0))],
                FrameTargetPolicy::unlimited(),
            )
            .expect("target should succeed");

        assert!(frame
            .validate_with_policies(
                FrameTargetPolicy::unlimited(),
                FrameMetadataPolicy::default(),
            )
            .is_ok());
    }

    #[test]
    fn invalid_metadata_nul_is_rejected() {
        let mut frame = Frame::new(frame_id(17));

        let result =
            frame.insert_metadata("key", "bad\0value");

        assert!(matches!(
            result,
            Err(FrameError::MetadataContainsNul)
        ));
    }

    #[test]
    fn frame_display_is_stable_enough_for_diagnostics() {
        let mut frame = Frame::new(frame_id(18));

        frame
            .set_name("drive")
            .expect("name should succeed");

        frame
            .set_frequency(
                FrameFrequency::constant(5.0)
                    .expect("valid frequency"),
            )
            .expect("frequency should succeed");

        frame
            .set_phase(
                FramePhase::constant(0.0)
                    .expect("valid phase"),
            )
            .expect("phase should succeed");

        frame
            .add_qubit(qubit(0))
            .expect("target should succeed");

        let text = frame.to_string();

        assert!(text.contains("frame"));
        assert!(text.contains("drive"));
        assert!(text.contains("frequency"));
        assert!(text.contains("phase"));
        assert!(text.contains("q0"));
    }
}