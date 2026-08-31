//! Zamani Quantum IR — Abstract Control Channel Semantics
//!
//! Canonical, hardware-independent representation of quantum control,
//! acquisition, measurement, optical, microwave, analog, and other abstract
//! channels used by the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `channel.rs` defines WHAT an abstract quantum-control or acquisition
//! channel means in the canonical Quantum IR.
//!
//! It does NOT define:
//!
//! - physical DACs;
//! - ADCs;
//! - microwave generators;
//! - laser hardware;
//! - RF electronics;
//! - optical hardware;
//! - physical wiring;
//! - device topology;
//! - hardware channel numbers;
//! - physical resource allocation;
//! - calibration values;
//! - calibration execution;
//! - logical-to-physical routing;
//! - scheduling;
//! - pulse synthesis;
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
//! # Architectural boundary
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
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//!   pulse.rs                       channel.rs
//!      |                               |
//!      | waveform reference             | abstract channel
//!      | frame reference                | semantics
//!      | channel reference              |
//!      |                               |
//!      +---------------+---------------+
//!                      |
//!                      v
//!                 optimization
//!                      |
//!                      v
//!                  routing
//!                      |
//!                      v
//!                 scheduling
//!                      |
//!                      v
//!                   hardware
//!                      |
//!                      v
//!                    backend
//!                      |
//!                      v
//!                     QPU
//! ```
//!
//! The IR answers:
//!
//! > What abstract channel does this operation require?
//!
//! The hardware layer answers:
//!
//! > What physical resource implements that channel on this target?
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
//! - analog processors;
//! - annealing systems;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - future quantum architectures.
//!
//! This file therefore contains NO architectural machine-size limit.
//!
//! Values such as:
//!
//! ```text
//! 63
//! 4096
//! 1_000_000
//! ```
//!
//! must never silently become a quantum-machine or channel-count limit.
//!
//! Explicit resource/security policies belong to `limits.rs` and downstream
//! compiler policies.
//!
//! # Canonical identity
//!
//! Channel identity is owned by `identity.rs`:
//!
//! ```rust
//! quantum::ir::identity::ChannelId
//! ```
//!
//! A `ChannelId` identifies an abstract IR channel.
//!
//! It does NOT identify:
//!
//! - a physical DAC;
//! - a physical ADC;
//! - a cable;
//! - an RF port;
//! - a laser;
//! - a microwave source;
//! - an optical path;
//! - a hardware channel index;
//! - a device-specific resource.
//!
//! # Canonical qubit integration
//!
//! Channel targets may refer to canonical logical/physical qubit vocabulary
//! through:
//!
//! ```rust
//! quantum::ir::qubit::QubitRef
//! ```
//!
//! The channel module does not perform routing or allocation.
//!
//! A logical target such as `QubitRef::Logical(q0)` remains logical until a
//! downstream mapping/routing stage establishes a physical placement.
//!
//! # Pulse-level example
//!
//! A Zamani source operation such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! may eventually lower to a pulse containing an abstract channel reference:
//!
//! ```text
//! Pulse
//!   target  = q
//!   channel = Drive
//!   amp     = 0.3
//!   dur     = 20ns
//! ```
//!
//! `channel.rs` describes the meaning of `Drive`.
//!
//! It does NOT decide which physical drive line, DAC, oscillator, microwave
//! source, or calibration implements it.
//!
//! # Channel categories
//!
//! The semantic model supports common quantum-control families without making
//! any particular hardware architecture mandatory:
//!
//! - drive;
//! - control;
//! - measure;
//! - acquire;
//! - readout;
//! - flux;
//! - microwave;
//! - laser;
//! - optical;
//! - analog;
//! - synchronization;
//! - custom.
//!
//! The `Custom` variant exists so a new architecture does not require a core
//! IR redesign merely because it introduces a new channel concept.
//!
//! # Separation from hardware
//!
//! The following distinction is mandatory:
//!
//! ```text
//! IR channel
//!     = semantic resource/reference
//!
//! Hardware channel
//!     = actual physical resource
//!
//! Routing
//!     = chooses physical placement
//!
//! Scheduling
//!     = chooses time
//!
//! Calibration
//!     = determines target-specific control parameters
//!
//! Backend
//!     = translates semantics to executable instructions
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `ChannelId`.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitRef`.
//!
//! `pulse.rs`
//!     Stores `ChannelId` references when a pulse requires a channel.
//!
//! `waveform.rs`
//!     Defines waveform semantics independently of channel semantics.
//!
//! `frame.rs`
//!     Defines frame semantics independently of channel semantics.
//!
//! `timing.rs`
//!     Defines program-wide temporal semantics.
//!
//! `schedule.rs`
//!     Determines when channel resources are occupied.
//!
//! `mapping.rs`
//!     Resolves logical qubit references to physical qubit resources.
//!
//! `hardware/`
//!     Determines whether a concrete target provides the requested channel
//!     category and maps it to actual hardware resources.
//!
//! `optimization/`
//!     May transform channel usage while preserving program semantics.
//!
//! `validation.rs`
//!     Performs whole-program namespace and structural validation.
//!
//! `serialization.rs`
//!     Serializes the stable channel representation.
//!
//! `hash.rs`
//!     May derive deterministic content identity from channel structure.
//!
//! `provenance.rs`
//!     Records channel-related transformations and lineage.
//!
//! `operation.rs`
//!     References channels through the strongly typed `ChannelId`.
//!
//! # File completion guarantee
//!
//! This file intentionally contains:
//!
//! - channel identity integration;
//! - channel kind vocabulary;
//! - channel scope;
//! - channel direction;
//! - channel access semantics;
//! - channel target semantics;
//! - channel requirements;
//! - channel constraints;
//! - custom channel support;
//! - deterministic metadata;
//! - resource-safe metadata validation;
//! - checked construction;
//! - no hardware assumptions;
//! - no fixed machine-size ceiling;
//! - unit tests;
//! - integration documentation.
//!
//! Later implementation of waveform, frame, timing, scheduling, hardware,
//! optimization, or backend modules should consume this contract rather than
//! requiring the channel semantic model to be redesigned.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use super::identity::ChannelId;
use super::qubit::QubitRef;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for abstract channel IR.
pub const CHANNEL_SCHEMA_ID: &str = "zamani.quantum.ir.channel";

/// Stable semantic schema version.
///
/// Breaking semantic changes require a new major IR contract.
pub const CHANNEL_SCHEMA_VERSION: u16 = 1;

/// Default maximum metadata key length in UTF-8 bytes.
///
/// This is an input/resource-safety policy and is NOT a quantum-machine
/// capacity limit.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 256;

/// Default maximum metadata value length in UTF-8 bytes.
///
/// This is an input/resource-safety policy and is NOT a quantum-machine
/// capacity limit.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Default maximum metadata fields on one channel.
///
/// This is an input/resource-safety policy and is NOT a channel-count limit.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Default maximum number of channel targets.
///
/// This is an explicit construction policy and can be replaced by a larger
/// caller-provided policy.
pub const DEFAULT_MAX_TARGETS: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Result type for channel construction and local validation.
pub type ChannelResult<T> = Result<T, ChannelError>;

// =============================================================================
// Channel kind
// =============================================================================

/// Semantic family of an abstract quantum-control or acquisition channel.
///
/// These values describe meaning, not implementation.
///
/// For example, `Drive` does not imply a particular microwave frequency,
/// physical wire, DAC, oscillator, laser, or hardware provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelKind {
    /// Primary coherent control channel for applying quantum operations.
    Drive,

    /// Auxiliary or cross-resonance/control channel.
    Control,

    /// Channel used to initiate a measurement interaction.
    Measure,

    /// Channel used to acquire/read a measurement signal.
    Acquire,

    /// Abstract readout channel.
    Readout,

    /// Flux or equivalent slow/analog control channel.
    Flux,

    /// Microwave control channel.
    Microwave,

    /// Optical/laser control channel.
    Laser,

    /// General optical channel.
    Optical,

    /// General analog-control channel.
    Analog,

    /// Synchronization/control-marker channel.
    Synchronization,

    /// User- or architecture-defined channel family.
    Custom(String),
}

impl ChannelKind {
    /// Returns the stable semantic name of this channel kind.
    ///
    /// Custom channel names are returned as-is.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Drive => "drive",
            Self::Control => "control",
            Self::Measure => "measure",
            Self::Acquire => "acquire",
            Self::Readout => "readout",
            Self::Flux => "flux",
            Self::Microwave => "microwave",
            Self::Laser => "laser",
            Self::Optical => "optical",
            Self::Analog => "analog",
            Self::Synchronization => "synchronization",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns whether this is a custom channel kind.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Creates a validated custom channel kind.
    pub fn custom(name: impl Into<String>) -> ChannelResult<Self> {
        let name = name.into();

        validate_non_empty_name(
            &name,
            "custom channel kind",
        )?;

        Ok(Self::Custom(name))
    }
}

impl fmt::Display for ChannelKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Channel scope
// =============================================================================

/// Semantic scope of a channel.
///
/// Scope describes how the channel is conceptually associated with quantum
/// resources. It does not allocate those resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelScope {
    /// Channel is independent of a specific qubit.
    Global,

    /// Channel is associated with one target resource.
    PerTarget,

    /// Channel may act on a group of targets.
    MultiTarget,

    /// Channel is associated with a pair or relation between targets.
    Pairwise,

    /// Channel scope is architecture-defined.
    Custom,
}

impl Default for ChannelScope {
    fn default() -> Self {
        Self::PerTarget
    }
}

impl fmt::Display for ChannelScope {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Global => "global",
            Self::PerTarget => "per_target",
            Self::MultiTarget => "multi_target",
            Self::Pairwise => "pairwise",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Channel direction
// =============================================================================

/// Direction of information/control flow associated with a channel.
///
/// Direction is semantic. It does not imply physical signal wiring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelDirection {
    /// Channel is used to send control into a quantum resource.
    Input,

    /// Channel is used to receive/acquire information.
    Output,

    /// Channel can both send and receive.
    Bidirectional,

    /// Direction is not applicable.
    Unspecified,
}

impl Default for ChannelDirection {
    fn default() -> Self {
        Self::Input
    }
}

impl fmt::Display for ChannelDirection {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::Bidirectional => "bidirectional",
            Self::Unspecified => "unspecified",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Channel access
// =============================================================================

/// Semantic access policy for a channel.
///
/// This describes how IR operations may conceptually use the channel.
/// Scheduling and hardware compatibility determine actual concurrent use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelAccess {
    /// Only one operation may conceptually occupy the channel at a time.
    Exclusive,

    /// Multiple compatible operations may share the channel.
    Shared,

    /// Channel is read-only.
    ReadOnly,

    /// Channel is write-only.
    WriteOnly,

    /// Access is determined by the target.
    TargetDefined,
}

impl Default for ChannelAccess {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for ChannelAccess {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::ReadOnly => "read_only",
            Self::WriteOnly => "write_only",
            Self::TargetDefined => "target_defined",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Channel target
// =============================================================================

/// Semantic target associated with an abstract channel.
///
/// A channel can be global, target a logical/physical qubit vocabulary, or
/// remain architecture-defined.
///
/// `QubitRef` deliberately preserves the distinction between logical and
/// physical identity.
///
/// This module never performs logical-to-physical routing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelTarget {
    /// Channel applies globally and has no specific qubit target.
    Global,

    /// Channel is associated with a canonical logical or physical qubit.
    Qubit(QubitRef),

    /// Channel is associated with multiple canonical qubit references.
    ///
    /// The constructor validates uniqueness and deterministic ordering.
    Qubits(Vec<QubitRef>),

    /// Architecture-defined target identity.
    ///
    /// This is a semantic extension point. It does not grant permission to
    /// bypass IR validation.
    Custom(String),
}

impl ChannelTarget {
    /// Creates a global channel target.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates a single-qubit target.
    #[must_use]
    pub const fn qubit(qubit: QubitRef) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a validated multi-qubit target.
    ///
    /// The supplied order is preserved because operand order can matter to
    /// downstream semantics.
    pub fn qubits(
        qubits: Vec<QubitRef>,
    ) -> ChannelResult<Self> {
        validate_unique_qubit_refs(&qubits)?;

        if qubits.is_empty() {
            return Err(ChannelError::EmptyTarget);
        }

        Ok(Self::Qubits(qubits))
    }

    /// Creates a custom semantic target.
    pub fn custom(
        target: impl Into<String>,
    ) -> ChannelResult<Self> {
        let target = target.into();

        validate_non_empty_name(
            &target,
            "custom channel target",
        )?;

        Ok(Self::Custom(target))
    }

    /// Returns the number of explicit qubit targets.
    ///
    /// Global and custom targets return zero because they are not represented
    /// as explicit qubit collections.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        match self {
            Self::Global | Self::Custom(_) => 0,
            Self::Qubit(_) => 1,
            Self::Qubits(qubits) => qubits.len(),
        }
    }

    /// Returns whether the target is global.
    #[must_use]
    pub fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether the target contains explicit qubit identity.
    #[must_use]
    pub fn contains_qubit_identity(&self) -> bool {
        matches!(
            self,
            Self::Qubit(_) | Self::Qubits(_)
        )
    }

    /// Returns an iterator over explicit qubit targets.
    ///
    /// Global/custom targets produce an empty iterator.
    pub fn qubits(
        &self,
    ) -> Box<dyn Iterator<Item = QubitRef> + '_> {
        match self {
            Self::Global | Self::Custom(_) => {
                Box::new(std::iter::empty())
            }

            Self::Qubit(qubit) => {
                Box::new(std::iter::once(*qubit))
            }

            Self::Qubits(qubits) => {
                Box::new(qubits.iter().copied())
            }
        }
    }
}

// =============================================================================
// Channel requirement
// =============================================================================

/// Capability/requirement description attached to an abstract channel.
///
/// These fields describe semantic requirements and are deliberately free from
/// hardware implementation details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelRequirement {
    kind: ChannelKind,
    scope: ChannelScope,
    direction: ChannelDirection,
    access: ChannelAccess,
}

impl ChannelRequirement {
    /// Creates a channel requirement.
    #[must_use]
    pub const fn new(
        kind: ChannelKind,
        scope: ChannelScope,
        direction: ChannelDirection,
        access: ChannelAccess,
    ) -> Self {
        Self {
            kind,
            scope,
            direction,
            access,
        }
    }

    /// Returns the required channel kind.
    #[must_use]
    pub fn kind(&self) -> &ChannelKind {
        &self.kind
    }

    /// Returns the required scope.
    #[must_use]
    pub const fn scope(&self) -> ChannelScope {
        self.scope
    }

    /// Returns the required direction.
    #[must_use]
    pub const fn direction(&self) -> ChannelDirection {
        self.direction
    }

    /// Returns the required access mode.
    #[must_use]
    pub const fn access(&self) -> ChannelAccess {
        self.access
    }
}

// =============================================================================
// Channel constraints
// =============================================================================

/// Target-independent semantic constraints for an abstract channel.
///
/// These constraints do not encode hardware-specific values such as a DAC
/// sample rate or physical frequency.
///
/// They describe structural requirements that can be understood at the IR
/// level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelConstraints {
    minimum_targets: usize,
    maximum_targets: Option<usize>,
    allow_logical_targets: bool,
    allow_physical_targets: bool,
    allow_global_target: bool,
}

impl Default for ChannelConstraints {
    fn default() -> Self {
        Self {
            minimum_targets: 0,
            maximum_targets: None,
            allow_logical_targets: true,
            allow_physical_targets: true,
            allow_global_target: true,
        }
    }
}

impl ChannelConstraints {
    /// Creates default unconstrained semantic channel constraints.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            minimum_targets: 0,
            maximum_targets: None,
            allow_logical_targets: true,
            allow_physical_targets: true,
            allow_global_target: true,
        }
    }

    /// Sets the minimum number of explicit targets.
    pub const fn with_minimum_targets(
        mut self,
        minimum_targets: usize,
    ) -> Self {
        self.minimum_targets = minimum_targets;
        self
    }

    /// Sets the maximum number of explicit targets.
    ///
    /// `None` means no channel-level maximum is imposed.
    pub const fn with_maximum_targets(
        mut self,
        maximum_targets: Option<usize>,
    ) -> Self {
        self.maximum_targets = maximum_targets;
        self
    }

    /// Controls whether logical qubit targets are allowed.
    pub const fn with_logical_targets(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_logical_targets = allowed;
        self
    }

    /// Controls whether physical qubit targets are allowed.
    pub const fn with_physical_targets(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_physical_targets = allowed;
        self
    }

    /// Controls whether a global target is allowed.
    pub const fn with_global_target(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_global_target = allowed;
        self
    }

    /// Returns the minimum target count.
    #[must_use]
    pub const fn minimum_targets(&self) -> usize {
        self.minimum_targets
    }

    /// Returns the optional maximum target count.
    #[must_use]
    pub const fn maximum_targets(&self) -> Option<usize> {
        self.maximum_targets
    }

    /// Returns whether logical targets are allowed.
    #[must_use]
    pub const fn allows_logical_targets(&self) -> bool {
        self.allow_logical_targets
    }

    /// Returns whether physical targets are allowed.
    #[must_use]
    pub const fn allows_physical_targets(&self) -> bool {
        self.allow_physical_targets
    }

    /// Returns whether a global target is allowed.
    #[must_use]
    pub const fn allows_global_target(&self) -> bool {
        self.allow_global_target
    }

    /// Validates a target against the semantic constraints.
    pub fn validate(
        &self,
        target: &ChannelTarget,
    ) -> ChannelResult<()> {
        if target.is_global() {
            if !self.allow_global_target {
                return Err(ChannelError::GlobalTargetNotAllowed);
            }

            if self.minimum_targets > 0 {
                return Err(
                    ChannelError::InsufficientTargets {
                        minimum: self.minimum_targets,
                        actual: 0,
                    },
                );
            }

            return Ok(());
        }

        let count = target.qubit_count();

        if count < self.minimum_targets {
            return Err(
                ChannelError::InsufficientTargets {
                    minimum: self.minimum_targets,
                    actual: count,
                },
            );
        }

        if let Some(maximum) = self.maximum_targets {
            if count > maximum {
                return Err(
                    ChannelError::TooManyTargets {
                        maximum,
                        actual: count,
                    },
                );
            }
        }

        match target {
            ChannelTarget::Qubit(qubit) => {
                self.validate_qubit_ref(*qubit)?;
            }

            ChannelTarget::Qubits(qubits) => {
                for qubit in qubits {
                    self.validate_qubit_ref(*qubit)?;
                }
            }

            ChannelTarget::Global | ChannelTarget::Custom(_) => {}
        }

        Ok(())
    }

    fn validate_qubit_ref(
        &self,
        qubit: QubitRef,
    ) -> ChannelResult<()> {
        match qubit {
            QubitRef::Logical(_) if !self.allow_logical_targets => {
                Err(ChannelError::LogicalTargetNotAllowed)
            }

            QubitRef::Physical(_) if !self.allow_physical_targets => {
                Err(ChannelError::PhysicalTargetNotAllowed)
            }

            _ => Ok(()),
        }
    }
}

// =============================================================================
// Channel metadata policy
// =============================================================================

/// Explicit limits for channel metadata.
///
/// Metadata is auxiliary information. It is not part of hardware
/// authentication, calibration, or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelMetadataLimits {
    /// Maximum number of metadata fields.
    pub max_fields: usize,

    /// Maximum UTF-8 byte length of one key.
    pub max_key_bytes: usize,

    /// Maximum UTF-8 byte length of one value.
    pub max_value_bytes: usize,
}

impl Default for ChannelMetadataLimits {
    fn default() -> Self {
        Self {
            max_fields: DEFAULT_MAX_METADATA_FIELDS,
            max_key_bytes: DEFAULT_MAX_METADATA_KEY_BYTES,
            max_value_bytes: DEFAULT_MAX_METADATA_VALUE_BYTES,
        }
    }
}

impl ChannelMetadataLimits {
    /// Creates explicit metadata limits.
    #[must_use]
    pub const fn new(
        max_fields: usize,
        max_key_bytes: usize,
        max_value_bytes: usize,
    ) -> Self {
        Self {
            max_fields,
            max_key_bytes,
            max_value_bytes,
        }
    }
}

// =============================================================================
// Channel metadata
// =============================================================================

/// Deterministically ordered channel metadata.
///
/// `BTreeMap` is used deliberately so serialization, hashing, diagnostics,
/// and reproducible compilation do not depend on insertion order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChannelMetadata {
    fields: BTreeMap<String, String>,
}

impl ChannelMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts one metadata field under explicit limits.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        limits: ChannelMetadataLimits,
    ) -> ChannelResult<()> {
        let key = key.into();
        let value = value.into();

        validate_metadata_field(
            &key,
            &value,
            limits,
        )?;

        if !self.fields.contains_key(&key)
            && self.fields.len() >= limits.max_fields
        {
            return Err(
                ChannelError::MetadataFieldLimitExceeded {
                    maximum: limits.max_fields,
                },
            );
        }

        self.fields.insert(key, value);
        Ok(())
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// Returns the number of metadata fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns an iterator over deterministic key/value pairs.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.fields
            .iter()
            .map(|(key, value)| {
                (key.as_str(), value.as_str())
            })
    }
}

// =============================================================================
// Channel
// =============================================================================

/// Canonical abstract quantum-control/acquisition channel.
///
/// A `Channel` is a semantic IR object. It is deliberately independent of
/// physical hardware.
///
/// The object can be used by pulse-level IR, measurement/acquisition IR,
/// analog IR, scheduling, and backend lowering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Channel {
    id: ChannelId,
    kind: ChannelKind,
    scope: ChannelScope,
    direction: ChannelDirection,
    access: ChannelAccess,
    target: ChannelTarget,
    constraints: ChannelConstraints,
    metadata: ChannelMetadata,
}

impl Channel {
    /// Creates a channel with default scope/direction/access and no target
    /// constraints beyond the target itself.
    pub fn new(
        id: ChannelId,
        kind: ChannelKind,
    ) -> Self {
        Self {
            id,
            kind,
            scope: ChannelScope::default(),
            direction: ChannelDirection::default(),
            access: ChannelAccess::default(),
            target: ChannelTarget::Global,
            constraints: ChannelConstraints::default(),
            metadata: ChannelMetadata::default(),
        }
    }

    /// Creates a channel with a complete semantic specification.
    pub fn try_new(
        id: ChannelId,
        requirement: ChannelRequirement,
        target: ChannelTarget,
        constraints: ChannelConstraints,
        metadata: ChannelMetadata,
    ) -> ChannelResult<Self> {
        let channel = Self {
            id,
            kind: requirement.kind,
            scope: requirement.scope,
            direction: requirement.direction,
            access: requirement.access,
            target,
            constraints,
            metadata,
        };

        channel.validate()?;
        Ok(channel)
    }

    /// Returns the channel identity.
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        self.id
    }

    /// Returns the channel semantic kind.
    #[must_use]
    pub fn kind(&self) -> &ChannelKind {
        &self.kind
    }

    /// Returns the channel scope.
    #[must_use]
    pub const fn scope(&self) -> ChannelScope {
        self.scope
    }

    /// Returns the channel direction.
    #[must_use]
    pub const fn direction(&self) -> ChannelDirection {
        self.direction
    }

    /// Returns the channel access policy.
    #[must_use]
    pub const fn access(&self) -> ChannelAccess {
        self.access
    }

    /// Returns the channel target.
    #[must_use]
    pub fn target(&self) -> &ChannelTarget {
        &self.target
    }

    /// Returns the channel constraints.
    #[must_use]
    pub const fn constraints(&self) -> &ChannelConstraints {
        &self.constraints
    }

    /// Returns channel metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ChannelMetadata {
        &self.metadata
    }

    /// Returns a builder with a different scope.
    #[must_use]
    pub const fn with_scope(
        mut self,
        scope: ChannelScope,
    ) -> Self {
        self.scope = scope;
        self
    }

    /// Returns a builder with a different direction.
    #[must_use]
    pub const fn with_direction(
        mut self,
        direction: ChannelDirection,
    ) -> Self {
        self.direction = direction;
        self
    }

    /// Returns a builder with a different access policy.
    #[must_use]
    pub const fn with_access(
        mut self,
        access: ChannelAccess,
    ) -> Self {
        self.access = access;
        self
    }

    /// Returns a builder with an explicit target.
    #[must_use]
    pub fn with_target(
        mut self,
        target: ChannelTarget,
    ) -> Self {
        self.target = target;
        self
    }

    /// Returns a builder with explicit target constraints.
    #[must_use]
    pub fn with_constraints(
        mut self,
        constraints: ChannelConstraints,
    ) -> Self {
        self.constraints = constraints;
        self
    }

    /// Returns a builder with metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: ChannelMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds one metadata field using explicit limits.
    pub fn with_metadata_field(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        limits: ChannelMetadataLimits,
    ) -> ChannelResult<Self> {
        self.metadata.insert(
            key,
            value,
            limits,
        )?;

        Ok(self)
    }

    /// Validates the channel's local semantic invariants.
    ///
    /// This does NOT verify whether hardware exists or supports the channel.
    pub fn validate(&self) -> ChannelResult<()> {
        self.constraints.validate(&self.target)?;

        match self.scope {
            ChannelScope::Global => {
                if !self.target.is_global() {
                    return Err(
                        ChannelError::ScopeTargetMismatch {
                            scope: ChannelScope::Global,
                        },
                    );
                }
            }

            ChannelScope::PerTarget => {
                if self.target.is_global() {
                    return Err(
                        ChannelError::ScopeTargetMismatch {
                            scope: ChannelScope::PerTarget,
                        },
                    );
                }

                if self.target.qubit_count() != 1 {
                    return Err(
                        ChannelError::ScopeTargetMismatch {
                            scope: ChannelScope::PerTarget,
                        },
                    );
                }
            }

            ChannelScope::MultiTarget => {
                if self.target.qubit_count() < 1 {
                    return Err(
                        ChannelError::ScopeTargetMismatch {
                            scope: ChannelScope::MultiTarget,
                        },
                    );
                }
            }

            ChannelScope::Pairwise => {
                if self.target.qubit_count() != 2 {
                    return Err(
                        ChannelError::ScopeTargetMismatch {
                            scope: ChannelScope::Pairwise,
                        },
                    );
                }
            }

            ChannelScope::Custom => {}
        }

        Ok(())
    }

    /// Returns the channel's semantic requirement.
    #[must_use]
    pub fn requirement(&self) -> ChannelRequirement {
        ChannelRequirement::new(
            self.kind.clone(),
            self.scope,
            self.direction,
            self.access,
        )
    }

    /// Returns the number of explicit qubit targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.target.qubit_count()
    }

    /// Returns whether this channel is global.
    #[must_use]
    pub fn is_global(&self) -> bool {
        self.target.is_global()
    }

    /// Returns whether this channel is a drive channel.
    #[must_use]
    pub fn is_drive(&self) -> bool {
        self.kind == ChannelKind::Drive
    }

    /// Returns whether this channel is a measurement channel.
    #[must_use]
    pub fn is_measurement(&self) -> bool {
        matches!(
            self.kind,
            ChannelKind::Measure
                | ChannelKind::Acquire
                | ChannelKind::Readout
        )
    }

    /// Returns whether this channel is a control channel.
    #[must_use]
    pub fn is_control(&self) -> bool {
        matches!(
            self.kind,
            ChannelKind::Control
                | ChannelKind::Drive
                | ChannelKind::Flux
                | ChannelKind::Microwave
                | ChannelKind::Laser
                | ChannelKind::Optical
                | ChannelKind::Analog
        )
    }
}

// =============================================================================
// Channel collection
// =============================================================================

/// Deterministic collection of channels indexed by `ChannelId`.
///
/// The collection is intentionally a `BTreeMap` so iteration order is stable
/// across runs and platforms.
///
/// It does not impose a fixed number of channels. Callers may enforce explicit
/// resource policies before insertion.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChannelSet {
    channels: BTreeMap<ChannelId, Channel>,
}

impl ChannelSet {
    /// Creates an empty channel collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a channel.
    ///
    /// Duplicate identities are rejected rather than silently replaced.
    pub fn insert(
        &mut self,
        channel: Channel,
    ) -> ChannelResult<()> {
        channel.validate()?;

        if self.channels.contains_key(&channel.id()) {
            return Err(
                ChannelError::DuplicateChannelId {
                    id: channel.id(),
                },
            );
        }

        self.channels.insert(
            channel.id(),
            channel,
        );

        Ok(())
    }

    /// Returns a channel by identity.
    #[must_use]
    pub fn get(
        &self,
        id: ChannelId,
    ) -> Option<&Channel> {
        self.channels.get(&id)
    }

    /// Returns the number of channels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.channels.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Removes a channel by identity.
    pub fn remove(
        &mut self,
        id: ChannelId,
    ) -> Option<Channel> {
        self.channels.remove(&id)
    }

    /// Returns deterministic channel iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&ChannelId, &Channel)> {
        self.channels.iter()
    }

    /// Returns whether a channel identity exists.
    #[must_use]
    pub fn contains(
        &self,
        id: ChannelId,
    ) -> bool {
        self.channels.contains_key(&id)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by channel construction and local validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
    /// A required name was empty.
    EmptyName {
        field: &'static str,
    },

    /// A custom name exceeded the allowed UTF-8 length.
    NameTooLong {
        field: &'static str,
        maximum: usize,
        actual: usize,
    },

    /// A custom channel target was empty.
    EmptyTarget,

    /// Duplicate qubit target was supplied.
    DuplicateTarget {
        target: QubitRef,
    },

    /// Logical qubit targets are forbidden by the channel constraints.
    LogicalTargetNotAllowed,

    /// Physical qubit targets are forbidden by the channel constraints.
    PhysicalTargetNotAllowed,

    /// Global target is forbidden by the channel constraints.
    GlobalTargetNotAllowed,

    /// Fewer targets than required.
    InsufficientTargets {
        minimum: usize,
        actual: usize,
    },

    /// More targets than permitted by the channel constraints.
    TooManyTargets {
        maximum: usize,
        actual: usize,
    },

    /// Channel scope and target shape are inconsistent.
    ScopeTargetMismatch {
        scope: ChannelScope,
    },

    /// Metadata key is too large.
    MetadataKeyTooLong {
        maximum: usize,
        actual: usize,
    },

    /// Metadata value is too large.
    MetadataValueTooLong {
        maximum: usize,
        actual: usize,
    },

    /// Metadata field count exceeded its explicit policy.
    MetadataFieldLimitExceeded {
        maximum: usize,
    },

    /// Two channels use the same identity in one collection.
    DuplicateChannelId {
        id: ChannelId,
    },
}

impl fmt::Display for ChannelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName { field } => {
                write!(
                    formatter,
                    "{field} cannot be empty"
                )
            }

            Self::NameTooLong {
                field,
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "{field} exceeds maximum length: {actual} > {maximum} bytes"
                )
            }

            Self::EmptyTarget => {
                write!(
                    formatter,
                    "channel target cannot be empty"
                )
            }

            Self::DuplicateTarget { target } => {
                write!(
                    formatter,
                    "channel target {target} appears more than once"
                )
            }

            Self::LogicalTargetNotAllowed => {
                write!(
                    formatter,
                    "logical qubit target is not allowed by channel constraints"
                )
            }

            Self::PhysicalTargetNotAllowed => {
                write!(
                    formatter,
                    "physical qubit target is not allowed by channel constraints"
                )
            }

            Self::GlobalTargetNotAllowed => {
                write!(
                    formatter,
                    "global channel target is not allowed by channel constraints"
                )
            }

            Self::InsufficientTargets {
                minimum,
                actual,
            } => {
                write!(
                    formatter,
                    "channel requires at least {minimum} target(s), received {actual}"
                )
            }

            Self::TooManyTargets {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "channel permits at most {maximum} target(s), received {actual}"
                )
            }

            Self::ScopeTargetMismatch { scope } => {
                write!(
                    formatter,
                    "channel scope {scope} is incompatible with its target"
                )
            }

            Self::MetadataKeyTooLong {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "metadata key exceeds maximum length: {actual} > {maximum} bytes"
                )
            }

            Self::MetadataValueTooLong {
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "metadata value exceeds maximum length: {actual} > {maximum} bytes"
                )
            }

            Self::MetadataFieldLimitExceeded {
                maximum,
            } => {
                write!(
                    formatter,
                    "metadata field limit exceeded: maximum {maximum}"
                )
            }

            Self::DuplicateChannelId { id } => {
                write!(
                    formatter,
                    "channel identity {id} already exists"
                )
            }
        }
    }
}

impl std::error::Error for ChannelError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_non_empty_name(
    value: &str,
    field: &'static str,
) -> ChannelResult<()> {
    if value.trim().is_empty() {
        return Err(
            ChannelError::EmptyName { field },
        );
    }

    Ok(())
}

fn validate_unique_qubit_refs(
    qubits: &[QubitRef],
) -> ChannelResult<()> {
    let mut seen = std::collections::BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(
                ChannelError::DuplicateTarget {
                    target: *qubit,
                },
            );
        }
    }

    Ok(())
}

fn validate_metadata_field(
    key: &str,
    value: &str,
    limits: ChannelMetadataLimits,
) -> ChannelResult<()> {
    validate_non_empty_name(
        key,
        "metadata key",
    )?;

    let key_bytes = key.len();

    if key_bytes > limits.max_key_bytes {
        return Err(
            ChannelError::MetadataKeyTooLong {
                maximum: limits.max_key_bytes,
                actual: key_bytes,
            },
        );
    }

    let value_bytes = value.len();

    if value_bytes > limits.max_value_bytes {
        return Err(
            ChannelError::MetadataValueTooLong {
                maximum: limits.max_value_bytes,
                actual: value_bytes,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a logical-qubit drive channel.
///
/// This represents the semantic concept of driving a logical qubit.
///
/// It does NOT allocate a physical drive line.
pub fn drive_channel(
    id: ChannelId,
    qubit: QubitRef,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Drive,
            ChannelScope::PerTarget,
            ChannelDirection::Input,
            ChannelAccess::Exclusive,
        ),
        ChannelTarget::qubit(qubit),
        ChannelConstraints::new()
            .with_minimum_targets(1)
            .with_maximum_targets(Some(1)),
        ChannelMetadata::new(),
    )
}

/// Creates a measurement channel for one logical/physical target.
///
/// This is semantic measurement-control vocabulary, not hardware readout
/// electronics.
pub fn measure_channel(
    id: ChannelId,
    qubit: QubitRef,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Measure,
            ChannelScope::PerTarget,
            ChannelDirection::Input,
            ChannelAccess::Exclusive,
        ),
        ChannelTarget::qubit(qubit),
        ChannelConstraints::new()
            .with_minimum_targets(1)
            .with_maximum_targets(Some(1)),
        ChannelMetadata::new(),
    )
}

/// Creates an acquisition channel for one target.
///
/// Acquisition is represented semantically. Actual ADC/readout hardware
/// remains outside the IR.
pub fn acquire_channel(
    id: ChannelId,
    qubit: QubitRef,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Acquire,
            ChannelScope::PerTarget,
            ChannelDirection::Output,
            ChannelAccess::Exclusive,
        ),
        ChannelTarget::qubit(qubit),
        ChannelConstraints::new()
            .with_minimum_targets(1)
            .with_maximum_targets(Some(1)),
        ChannelMetadata::new(),
    )
}

/// Creates a global synchronization channel.
pub fn synchronization_channel(
    id: ChannelId,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Synchronization,
            ChannelScope::Global,
            ChannelDirection::Input,
            ChannelAccess::Shared,
        ),
        ChannelTarget::global(),
        ChannelConstraints::new(),
        ChannelMetadata::new(),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    fn channel_id(value: u64) -> ChannelId {
        ChannelId::new(value)
    }

    #[test]
    fn channel_identity_is_strongly_typed() {
        let id = channel_id(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "channel42");
    }

    #[test]
    fn drive_channel_uses_canonical_qubit_module() {
        let channel = drive_channel(
            channel_id(1),
            QubitRef::Logical(
                QubitId::new(0),
            ),
        )
        .expect("drive channel should be valid");

        assert_eq!(
            channel.kind(),
            &ChannelKind::Drive
        );

        assert_eq!(
            channel.target_count(),
            1
        );

        assert!(channel.is_drive());
    }

    #[test]
    fn physical_target_is_distinct_from_logical_target() {
        let logical = ChannelTarget::qubit(
            QubitRef::Logical(
                QubitId::new(0),
            ),
        );

        let physical = ChannelTarget::qubit(
            QubitRef::Physical(
                PhysicalQubitId::new(0),
            ),
        );

        assert_ne!(
            logical,
            physical
        );
    }

    #[test]
    fn duplicate_multi_target_qubits_are_rejected() {
        let q = QubitRef::Logical(
            QubitId::new(0),
        );

        let result = ChannelTarget::qubits(
            vec![q, q],
        );

        assert!(matches!(
            result,
            Err(
                ChannelError::DuplicateTarget { .. }
            )
        ));
    }

    #[test]
    fn pairwise_channel_requires_two_targets() {
        let q0 = QubitRef::Logical(
            QubitId::new(0),
        );

        let q1 = QubitRef::Logical(
            QubitId::new(1),
        );

        let target = ChannelTarget::qubits(
            vec![q0, q1],
        )
        .expect("two unique targets are valid");

        let channel = Channel::try_new(
            channel_id(10),
            ChannelRequirement::new(
                ChannelKind::Control,
                ChannelScope::Pairwise,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            target,
            ChannelConstraints::new()
                .with_minimum_targets(2)
                .with_maximum_targets(Some(2)),
            ChannelMetadata::new(),
        )
        .expect("pairwise channel should be valid");

        assert_eq!(
            channel.target_count(),
            2
        );

        assert_eq!(
            channel.scope(),
            ChannelScope::Pairwise
        );
    }

    #[test]
    fn global_channel_requires_global_target() {
        let result = Channel::try_new(
            channel_id(20),
            ChannelRequirement::new(
                ChannelKind::Synchronization,
                ChannelScope::Global,
                ChannelDirection::Input,
                ChannelAccess::Shared,
            ),
            ChannelTarget::qubit(
                QubitRef::Logical(
                    QubitId::new(0),
                ),
            ),
            ChannelConstraints::new(),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(
                ChannelError::ScopeTargetMismatch {
                    scope: ChannelScope::Global
                }
            )
        ));
    }

    #[test]
    fn per_target_channel_rejects_multiple_targets() {
        let q0 = QubitRef::Logical(
            QubitId::new(0),
        );

        let q1 = QubitRef::Logical(
            QubitId::new(1),
        );

        let target = ChannelTarget::qubits(
            vec![q0, q1],
        )
        .expect("targets should be structurally valid");

        let result = Channel::try_new(
            channel_id(30),
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            target,
            ChannelConstraints::new(),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(
                ChannelError::ScopeTargetMismatch {
                    scope: ChannelScope::PerTarget
                }
            )
        ));
    }

    #[test]
    fn logical_targets_can_be_forbidden_explicitly() {
        let result = Channel::try_new(
            channel_id(40),
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            ChannelTarget::qubit(
                QubitRef::Logical(
                    QubitId::new(0),
                ),
            ),
            ChannelConstraints::new()
                .with_logical_targets(false)
                .with_physical_targets(true),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(
                ChannelError::LogicalTargetNotAllowed
            )
        ));
    }

    #[test]
    fn physical_targets_can_be_forbidden_explicitly() {
        let result = Channel::try_new(
            channel_id(41),
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            ChannelTarget::qubit(
                QubitRef::Physical(
                    PhysicalQubitId::new(0),
                ),
            ),
            ChannelConstraints::new()
                .with_logical_targets(true)
                .with_physical_targets(false),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(
                ChannelError::PhysicalTargetNotAllowed
            )
        ));
    }

    #[test]
    fn metadata_is_deterministic() {
        let limits = ChannelMetadataLimits::new(
            4,
            32,
            64,
        );

        let mut metadata = ChannelMetadata::new();

        metadata
            .insert(
                "z",
                "last",
                limits,
            )
            .expect("metadata should be valid");

        metadata
            .insert(
                "a",
                "first",
                limits,
            )
            .expect("metadata should be valid");

        let values: Vec<_> =
            metadata.iter().collect();

        assert_eq!(
            values,
            vec![
                ("a", "first"),
                ("z", "last"),
            ]
        );
    }

    #[test]
    fn metadata_limits_are_enforced() {
        let limits = ChannelMetadataLimits::new(
            1,
            3,
            4,
        );

        let mut metadata = ChannelMetadata::new();

        assert!(metadata
            .insert(
                "abcd",
                "ok",
                limits,
            )
            .is_err());

        assert!(metadata
            .insert(
                "ok",
                "12345",
                limits,
            )
            .is_err());

        metadata
            .insert(
                "ok",
                "1234",
                limits,
            )
            .expect("first metadata field should fit");

        assert!(metadata
            .insert(
                "two",
                "value",
                limits,
            )
            .is_err());
    }

    #[test]
    fn channel_set_rejects_duplicate_ids() {
        let id = channel_id(100);

        let first = drive_channel(
            id,
            QubitRef::Logical(
                QubitId::new(0),
            ),
        )
        .expect("first channel should be valid");

        let second = drive_channel(
            id,
            QubitRef::Logical(
                QubitId::new(1),
            ),
        )
        .expect("second channel should be locally valid");

        let mut channels = ChannelSet::new();

        channels
            .insert(first)
            .expect("first insertion should succeed");

        assert!(matches!(
            channels.insert(second),
            Err(
                ChannelError::DuplicateChannelId { .. }
            )
        ));
    }

    #[test]
    fn channel_set_iteration_is_deterministic() {
        let mut channels = ChannelSet::new();

        for value in [30_u64, 10_u64, 20_u64] {
            channels
                .insert(
                    drive_channel(
                        channel_id(value),
                        QubitRef::Logical(
                            QubitId::new(
                                value as usize,
                            ),
                        ),
                    )
                    .expect("channel should be valid"),
                )
                .expect("insertion should succeed");
        }

        let ids: Vec<u64> = channels
            .iter()
            .map(|(id, _)| id.value())
            .collect();

        assert_eq!(
            ids,
            vec![10, 20, 30]
        );
    }

    #[test]
    fn synchronization_channel_is_global() {
        let channel =
            synchronization_channel(
                channel_id(200),
            )
            .expect("synchronization channel should be valid");

        assert_eq!(
            channel.scope(),
            ChannelScope::Global
        );

        assert!(channel.is_global());

        assert_eq!(
            channel.kind(),
            &ChannelKind::Synchronization
        );
    }

    #[test]
    fn measurement_channel_has_output_only_acquisition_semantics() {
        let channel = acquire_channel(
            channel_id(300),
            QubitRef::Logical(
                QubitId::new(7),
            ),
        )
        .expect("acquisition channel should be valid");

        assert_eq!(
            channel.kind(),
            &ChannelKind::Acquire
        );

        assert_eq!(
            channel.direction(),
            ChannelDirection::Output
        );

        assert!(channel.is_measurement());
    }

    #[test]
    fn custom_channel_kind_is_supported() {
        let kind =
            ChannelKind::custom(
                "ion-trap-beam",
            )
            .expect("custom channel kind should be valid");

        assert!(kind.is_custom());
        assert_eq!(
            kind.name(),
            "ion-trap-beam"
        );
    }

    #[test]
    fn no_machine_size_limit_is_encoded() {
        let large_id =
            ChannelId::new(
                u64::MAX,
            );

        let channel =
            synchronization_channel(
                large_id,
            )
            .expect(
                "large identity values must remain representable",
            );

        assert_eq!(
            channel.id().value(),
            u64::MAX
        );
    }

    #[test]
    fn channel_requirement_round_trips_semantically() {
        let original =
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            );

        let channel =
            Channel::try_new(
                channel_id(400),
                original.clone(),
                ChannelTarget::qubit(
                    QubitRef::Logical(
                        QubitId::new(3),
                    ),
                ),
                ChannelConstraints::new()
                    .with_minimum_targets(1)
                    .with_maximum_targets(
                        Some(1),
                    ),
                ChannelMetadata::new(),
            )
            .expect(
                "channel should be valid",
            );

        assert_eq!(
            channel.requirement(),
            original
        );
    }
}