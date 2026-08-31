//! Zamani Quantum IR — Universal Abstract Channel Semantics
//!
//! Canonical, hardware-independent representation of quantum-control,
//! acquisition, measurement, optical, microwave, analog, synchronization,
//! communication, and future architecture-defined channels.
//!
//! # Architectural role
//!
//! `quantum::ir::quantum::channel` defines the semantic meaning of an
//! abstract channel used by the canonical Zamani Quantum IR.
//!
//! It answers:
//!
//! > What kind of abstract resource does this operation require?
//!
//! It does NOT answer:
//!
//! > Which physical device implements that resource?
//!
//! Physical realization belongs to downstream target/hardware layers.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - stable channel references;
//! - channel semantic kinds;
//! - channel scope;
//! - channel direction;
//! - channel access semantics;
//! - abstract channel targets;
//! - channel requirements;
//! - target constraints;
//! - deterministic channel metadata;
//! - channel collections;
//! - local channel validation;
//! - channel semantic compatibility;
//! - canonical constructors for common channel families.
//!
//! # This module does NOT own
//!
//! This module must never own:
//!
//! - DACs;
//! - ADCs;
//! - physical ports;
//! - physical wiring;
//! - microwave generators;
//! - laser hardware;
//! - oscillator hardware;
//! - physical channel numbers;
//! - hardware topology;
//! - physical allocation;
//! - routing;
//! - scheduling;
//! - calibration execution;
//! - waveform synthesis;
//! - pulse optimization;
//! - backend execution;
//! - provider SDKs;
//! - provider credentials;
//! - QPU communication;
//! - simulator state;
//! - quantum amplitudes;
//! - QEC decoding;
//! - frontend parsing.
//!
//! # Universal-program principle
//!
//! A Zamani program is written at the semantic level and may be lowered to
//! different compatible machines.
//!
//! A channel therefore represents an abstract semantic resource, not a
//! particular physical resource.
//!
//! The same semantic channel model must work for:
//!
//! - one qubit;
//! - many qubits;
//! - very large finite systems;
//! - superconducting systems;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - spin systems;
//! - analog systems;
//! - annealing systems;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - future architectures.
//!
//! There is deliberately NO architectural channel-count limit in this file.
//!
//! # Scalability rule
//!
//! This module must never encode:
//!
//! ```text
//! MAX_CHANNELS = ...
//! MAX_QUBITS = ...
//! MAX_TARGETS = ...
//! MAX_MACHINE_SIZE = ...
//! ```
//!
//! Semantic structures are bounded only by the Rust platform's representable
//! types and the memory/resources available to the compilation process.
//!
//! Explicit compiler/service resource limits belong in `limits.rs`.
//!
//! # Identity boundary
//!
//! Channel identity is owned by:
//!
//! ```text
//! quantum::ir::identity::ChannelId
//! ```
//!
//! `ChannelId` identifies an abstract IR channel.
//!
//! It does not identify:
//!
//! - a DAC;
//! - an ADC;
//! - a physical wire;
//! - a physical port;
//! - a laser;
//! - a microwave generator;
//! - a hardware channel number.
//!
//! # Qubit integration
//!
//! Qubit targets use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitRef
//! ```
//!
//! This module intentionally does not define another qubit identifier.
//!
//! Logical and physical qubit identity therefore remain distinct:
//!
//! ```text
//! QubitRef::Logical(QubitId)
//! QubitRef::Physical(PhysicalQubitId)
//! ```
//!
//! This module never performs logical-to-physical mapping.
//!
//! # Pulse integration
//!
//! A pulse may refer to a channel semantically:
//!
//! ```text
//! pulse
//!     │
//!     ├── waveform
//!     ├── frame
//!     ├── duration
//!     └── channel
//!              │
//!              ▼
//!       abstract channel
//!              │
//!              ▼
//!       target lowering
//!              │
//!              ▼
//!       physical resource
//! ```
//!
//! For example:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! can reference a semantic `Drive` channel without deciding which physical
//! drive line implements it.
//!
//! # Hardware separation
//!
//! ```text
//! Canonical IR
//!     │
//!     │ Channel
//!     ▼
//! semantic requirement
//!     │
//!     ▼
//! target capability
//!     │
//!     ▼
//! mapping/allocation
//!     │
//!     ▼
//! scheduling
//!     │
//!     ▼
//! calibration
//!     │
//!     ▼
//! backend
//!     │
//!     ▼
//! physical hardware
//! ```
//!
//! No reverse dependency is allowed.
//!
//! # Determinism
//!
//! Channel collections use `BTreeMap` rather than `HashMap` so iteration is
//! deterministic and independent of hash randomization.
//!
//! Channel metadata is also deterministically ordered.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! `identity.rs`
//!     Provides `ChannelId`.
//!
//! `qubit.rs`
//!     Provides canonical `QubitRef`.
//!
//! `operation.rs`
//!     May reference `ChannelId`.
//!
//! `pulse.rs`
//!     May reference `ChannelId`.
//!
//! `waveform.rs`
//!     Defines waveform semantics independently.
//!
//! `frame.rs`
//!     Defines frame semantics independently.
//!
//! `timing.rs`
//!     Defines temporal semantics independently.
//!
//! `schedule.rs`
//!     Determines temporal occupation of channel resources.
//!
//! `mapping.rs`
//!     Resolves logical/physical resource relationships.
//!
//! `capability.rs`
//!     Describes whether a target can satisfy channel requirements.
//!
//! `resource.rs`
//!     Represents abstract resource requirements.
//!
//! `validation.rs`
//!     Performs whole-program validation.
//!
//! `serialization.rs`
//!     Owns persistence/canonical encoding.
//!
//! `hash.rs`
//!     Owns canonical content hashing.
//!
//! `provenance.rs`
//!     Records transformations involving channels.
//!
//! `hardware`
//!     Owns physical implementation.
//!
//! `routing`
//!     Owns placement.
//!
//! `scheduling`
//!     Owns temporal scheduling.
//!
//! `backend`
//!     Owns executable lowering.
//!
//! # Important ownership rule
//!
//! `Channel` owns channel semantics.
//!
//! `ChannelRequirement` owns what an operation requires.
//!
//! `ChannelTarget` owns semantic target information.
//!
//! `ChannelConstraints` owns target-shape constraints.
//!
//! `ChannelMetadata` owns deterministic auxiliary metadata.
//!
//! `ChannelSet` owns deterministic channel collection semantics.
//!
//! None of these types own hardware implementation details.
//!
//! # File-completion guarantee
//!
//! This file intentionally contains the complete semantic contract needed by
//! downstream channel consumers. Adding a new backend, topology, calibration
//! system, pulse compiler, simulator, or hardware architecture must not require
//! modifying this file merely because the physical implementation changed.
//!
//! New channel families should normally be represented using `ChannelKind`
//! or its `Custom` extension rather than adding vendor-specific fields.
//!
//! New hardware must not require changes to `QubitRef`, `ChannelId`, or
//! `Channel` merely because its physical implementation differs.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::identity::{ChannelId, ResourceId};
use super::super::qubit::QubitRef;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier.
pub const CHANNEL_SCHEMA_ID: &str = "zamani.quantum.ir.quantum.channel";

/// Semantic schema major version.
///
/// This is local to the channel semantic contract. Global IR versioning
/// remains owned by `identity::IrVersion`.
pub const CHANNEL_SCHEMA_MAJOR: u16 = 1;

/// Semantic schema minor version.
pub const CHANNEL_SCHEMA_MINOR: u16 = 0;

// =============================================================================
// Result
// =============================================================================

/// Result type used by channel construction and local validation.
pub type ChannelResult<T> = Result<T, ChannelError>;

// =============================================================================
// Channel kind
// =============================================================================

/// Semantic family of an abstract quantum-control or acquisition channel.
///
/// These variants describe *meaning*, not physical implementation.
///
/// A `Drive` channel does not imply a microwave DAC.
/// A `Laser` channel does not imply a particular optical device.
/// An `Acquire` channel does not imply a particular ADC.
///
/// `Custom` provides an explicit extension point for architectures that need
/// semantic channel families not yet standardized by the core IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelKind {
    /// Coherent quantum-control channel.
    Drive,

    /// Auxiliary/control interaction channel.
    Control,

    /// Measurement-interaction channel.
    Measure,

    /// Acquisition channel.
    Acquire,

    /// Semantic readout channel.
    Readout,

    /// Flux-like or slow analog-control channel.
    Flux,

    /// Microwave semantic channel.
    Microwave,

    /// Laser/atomic-control semantic channel.
    Laser,

    /// Optical semantic channel.
    Optical,

    /// General analog-control channel.
    Analog,

    /// Synchronization/marker channel.
    Synchronization,

    /// Quantum communication/link-control channel.
    Communication,

    /// Channel associated with a logical/fault-tolerant control process.
    LogicalControl,

    /// Channel associated with syndrome/error-information acquisition.
    Syndrome,

    /// User/architecture-defined semantic channel family.
    Custom(String),
}

impl ChannelKind {
    /// Creates a validated custom channel kind.
    pub fn custom(name: impl Into<String>) -> ChannelResult<Self> {
        let name = name.into();
        validate_non_empty_name(&name, "custom channel kind")?;
        Ok(Self::Custom(name))
    }

    /// Returns the stable semantic name.
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
            Self::Communication => "communication",
            Self::LogicalControl => "logical_control",
            Self::Syndrome => "syndrome",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns whether this is an extension-defined kind.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Returns whether this kind is normally used for control.
    #[must_use]
    pub fn is_control_kind(&self) -> bool {
        matches!(
            self,
            Self::Drive
                | Self::Control
                | Self::Flux
                | Self::Microwave
                | Self::Laser
                | Self::Optical
                | Self::Analog
                | Self::LogicalControl
        )
    }

    /// Returns whether this kind is normally used for acquisition or
    /// measurement.
    #[must_use]
    pub fn is_measurement_kind(&self) -> bool {
        matches!(
            self,
            Self::Measure
                | Self::Acquire
                | Self::Readout
                | Self::Syndrome
        )
    }
}

impl fmt::Display for ChannelKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Channel scope
// =============================================================================

/// Semantic scope of a channel.
///
/// Scope describes how a channel relates to its targets. It does not perform
/// allocation or routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelScope {
    /// Channel is independent of an explicit quantum target.
    Global,

    /// Channel is associated with exactly one explicit target.
    PerTarget,

    /// Channel may semantically address multiple targets.
    MultiTarget,

    /// Channel represents a relationship between exactly two targets.
    Pairwise,

    /// Scope is defined by an extension or downstream dialect.
    Custom,
}

impl Default for ChannelScope {
    fn default() -> Self {
        Self::Global
    }
}

impl fmt::Display for ChannelScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Global => "global",
            Self::PerTarget => "per_target",
            Self::MultiTarget => "multi_target",
            Self::Pairwise => "pairwise",
            Self::Custom => "custom",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Channel direction
// =============================================================================

/// Semantic information/control direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelDirection {
    /// Control/information flows into the target.
    Input,

    /// Information flows from the target to the acquisition consumer.
    Output,

    /// Channel may both send and receive.
    Bidirectional,

    /// Direction has no applicable semantic interpretation.
    Unspecified,
}

impl Default for ChannelDirection {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl fmt::Display for ChannelDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::Bidirectional => "bidirectional",
            Self::Unspecified => "unspecified",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Channel access
// =============================================================================

/// Semantic access policy for a channel.
///
/// This does not schedule operations. It tells the scheduling layer what
/// semantic resource conflict model must be respected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelAccess {
    /// One active use at a time.
    Exclusive,

    /// Multiple compatible users may share the channel.
    Shared,

    /// Channel may only be read.
    ReadOnly,

    /// Channel may only be written.
    WriteOnly,

    /// Access policy is supplied by the target/dialect.
    TargetDefined,
}

impl Default for ChannelAccess {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for ChannelAccess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::ReadOnly => "read_only",
            Self::WriteOnly => "write_only",
            Self::TargetDefined => "target_defined",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Channel target
// =============================================================================

/// Semantic target of an abstract channel.
///
/// The target is intentionally not limited to physical hardware.
///
/// `Qubit` and `Qubits` use the canonical `quantum::ir::qubit::QubitRef`.
///
/// `Resource` allows channels to reference another abstract IR resource
/// without introducing a duplicate resource identity system.
///
/// `Custom` is reserved for dialect-defined target semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelTarget {
    /// Channel has no explicit quantum/resource target.
    Global,

    /// Channel targets one canonical logical or physical qubit.
    Qubit(QubitRef),

    /// Channel targets multiple canonical qubits.
    ///
    /// Ordering is preserved because operand order can be semantically
    /// significant.
    Qubits(Vec<QubitRef>),

    /// Channel targets an abstract IR resource.
    Resource(ResourceId),

    /// Channel uses a dialect-defined semantic target.
    Custom(String),
}

impl ChannelTarget {
    /// Creates a global target.
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
    /// Duplicate qubit identities are rejected.
    pub fn qubits(qubits: Vec<QubitRef>) -> ChannelResult<Self> {
        if qubits.is_empty() {
            return Err(ChannelError::EmptyTarget);
        }

        validate_unique_qubits(&qubits)?;

        Ok(Self::Qubits(qubits))
    }

    /// Creates an abstract resource target.
    #[must_use]
    pub const fn resource(resource: ResourceId) -> Self {
        Self::Resource(resource)
    }

    /// Creates a custom semantic target.
    pub fn custom(target: impl Into<String>) -> ChannelResult<Self> {
        let target = target.into();
        validate_non_empty_name(&target, "custom channel target")?;
        Ok(Self::Custom(target))
    }

    /// Returns whether the target is global.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether the target contains explicit qubit identity.
    #[must_use]
    pub const fn contains_qubit_identity(&self) -> bool {
        matches!(self, Self::Qubit(_) | Self::Qubits(_))
    }

    /// Returns the number of explicitly represented qubit targets.
    ///
    /// Global/resource/custom targets return zero because they do not encode
    /// an explicit qubit collection.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        match self {
            Self::Global | Self::Resource(_) | Self::Custom(_) => 0,
            Self::Qubit(_) => 1,
            Self::Qubits(qubits) => qubits.len(),
        }
    }

    /// Returns an iterator over explicitly represented qubits.
    pub fn qubits(&self) -> impl Iterator<Item = QubitRef> + '_ {
        match self {
            Self::Global | Self::Resource(_) | Self::Custom(_) => {
                None.into_iter().flatten()
            }
            Self::Qubit(qubit) => Some(std::iter::once(*qubit))
                .into_iter()
                .flatten(),
            Self::Qubits(qubits) => Some(qubits.iter().copied())
                .into_iter()
                .flatten(),
        }
    }
}

// =============================================================================
// Channel requirement
// =============================================================================

/// Semantic channel requirement.
///
/// A requirement describes what an operation needs. A requirement is not itself
/// a channel allocation.
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

    /// Returns the required access policy.
    #[must_use]
    pub const fn access(&self) -> ChannelAccess {
        self.access
    }
}

// =============================================================================
// Target constraints
// =============================================================================

/// Target-shape constraints for a channel.
///
/// These are semantic constraints rather than hardware capacity limits.
///
/// `maximum_targets = None` explicitly means that the channel imposes no
/// semantic maximum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelConstraints {
    minimum_targets: usize,
    maximum_targets: Option<usize>,
    allow_global_target: bool,
    allow_logical_targets: bool,
    allow_physical_targets: bool,
    allow_resource_targets: bool,
    allow_custom_targets: bool,
}

impl Default for ChannelConstraints {
    fn default() -> Self {
        Self {
            minimum_targets: 0,
            maximum_targets: None,
            allow_global_target: true,
            allow_logical_targets: true,
            allow_physical_targets: true,
            allow_resource_targets: true,
            allow_custom_targets: true,
        }
    }
}

impl ChannelConstraints {
    /// Creates unconstrained semantic target rules.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            minimum_targets: 0,
            maximum_targets: None,
            allow_global_target: true,
            allow_logical_targets: true,
            allow_physical_targets: true,
            allow_resource_targets: true,
            allow_custom_targets: true,
        }
    }

    /// Sets the minimum explicit qubit target count.
    #[must_use]
    pub const fn with_minimum_targets(mut self, value: usize) -> Self {
        self.minimum_targets = value;
        self
    }

    /// Sets the optional maximum explicit qubit target count.
    #[must_use]
    pub const fn with_maximum_targets(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.maximum_targets = value;
        self
    }

    /// Sets whether global targets are permitted.
    #[must_use]
    pub const fn with_global_targets(mut self, allowed: bool) -> Self {
        self.allow_global_target = allowed;
        self
    }

    /// Sets whether logical qubit targets are permitted.
    #[must_use]
    pub const fn with_logical_targets(mut self, allowed: bool) -> Self {
        self.allow_logical_targets = allowed;
        self
    }

    /// Sets whether physical qubit targets are permitted.
    #[must_use]
    pub const fn with_physical_targets(mut self, allowed: bool) -> Self {
        self.allow_physical_targets = allowed;
        self
    }

    /// Sets whether abstract resource targets are permitted.
    #[must_use]
    pub const fn with_resource_targets(mut self, allowed: bool) -> Self {
        self.allow_resource_targets = allowed;
        self
    }

    /// Sets whether custom targets are permitted.
    #[must_use]
    pub const fn with_custom_targets(mut self, allowed: bool) -> Self {
        self.allow_custom_targets = allowed;
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

    /// Returns whether global targets are allowed.
    #[must_use]
    pub const fn allows_global_targets(&self) -> bool {
        self.allow_global_target
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

    /// Returns whether resource targets are allowed.
    #[must_use]
    pub const fn allows_resource_targets(&self) -> bool {
        self.allow_resource_targets
    }

    /// Returns whether custom targets are allowed.
    #[must_use]
    pub const fn allows_custom_targets(&self) -> bool {
        self.allow_custom_targets
    }

    /// Validates one target against these constraints.
    pub fn validate(&self, target: &ChannelTarget) -> ChannelResult<()> {
        if target.is_global() {
            if !self.allow_global_target {
                return Err(ChannelError::GlobalTargetNotAllowed);
            }

            if self.minimum_targets != 0 {
                return Err(ChannelError::InsufficientTargets {
                    minimum: self.minimum_targets,
                    actual: 0,
                });
            }

            return Ok(());
        }

        let count = target.qubit_count();

        if count < self.minimum_targets {
            return Err(ChannelError::InsufficientTargets {
                minimum: self.minimum_targets,
                actual: count,
            });
        }

        if let Some(maximum) = self.maximum_targets {
            if count > maximum {
                return Err(ChannelError::TooManyTargets {
                    maximum,
                    actual: count,
                });
            }
        }

        match target {
            ChannelTarget::Global => {}

            ChannelTarget::Qubit(qubit) => {
                self.validate_qubit(*qubit)?;
            }

            ChannelTarget::Qubits(qubits) => {
                for qubit in qubits {
                    self.validate_qubit(*qubit)?;
                }
            }

            ChannelTarget::Resource(_) if !self.allow_resource_targets => {
                return Err(ChannelError::ResourceTargetNotAllowed);
            }

            ChannelTarget::Resource(_) => {}

            ChannelTarget::Custom(_) if !self.allow_custom_targets => {
                return Err(ChannelError::CustomTargetNotAllowed);
            }

            ChannelTarget::Custom(_) => {}
        }

        Ok(())
    }

    fn validate_qubit(&self, qubit: QubitRef) -> ChannelResult<()> {
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
// Channel metadata
// =============================================================================

/// Deterministically ordered auxiliary metadata.
///
/// Metadata is intentionally represented as strings rather than a hardware
/// configuration object. This keeps channel semantics independent from any
/// particular backend.
///
/// Resource limits for metadata belong to compiler/service policy rather than
/// to the semantic channel type.
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

    /// Inserts or replaces a metadata field.
    ///
    /// Empty keys are rejected.
    ///
    /// Existing keys are replaced deterministically.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ChannelResult<()> {
        let key = key.into();
        let value = value.into();

        validate_non_empty_name(&key, "metadata key")?;

        self.fields.insert(key, value);
        Ok(())
    }

    /// Removes a metadata field.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.fields.remove(key)
    }

    /// Gets a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// Returns whether a key exists.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    /// Returns the number of metadata fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether there are no metadata fields.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns deterministic key/value iteration.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.fields
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

// =============================================================================
// Channel
// =============================================================================

/// Canonical abstract quantum channel.
///
/// `Channel` represents semantic channel identity and requirements. It does
/// not represent a physical hardware channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Channel {
    id: ChannelId,
    requirement: ChannelRequirement,
    target: ChannelTarget,
    constraints: ChannelConstraints,
    metadata: ChannelMetadata,
}

impl Channel {
    /// Creates and validates a complete channel.
    ///
    /// This is the primary constructor.
    pub fn try_new(
        id: ChannelId,
        requirement: ChannelRequirement,
        target: ChannelTarget,
        constraints: ChannelConstraints,
        metadata: ChannelMetadata,
    ) -> ChannelResult<Self> {
        let channel = Self {
            id,
            requirement,
            target,
            constraints,
            metadata,
        };

        channel.validate()?;

        Ok(channel)
    }

    /// Creates a global channel with the supplied semantic requirement.
    pub fn global(
        id: ChannelId,
        requirement: ChannelRequirement,
    ) -> ChannelResult<Self> {
        Self::try_new(
            id,
            requirement,
            ChannelTarget::Global,
            ChannelConstraints::new(),
            ChannelMetadata::new(),
        )
    }

    /// Returns the channel identity.
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        self.id
    }

    /// Returns the complete semantic requirement.
    #[must_use]
    pub fn requirement(&self) -> &ChannelRequirement {
        &self.requirement
    }

    /// Returns the channel kind.
    #[must_use]
    pub fn kind(&self) -> &ChannelKind {
        self.requirement.kind()
    }

    /// Returns the channel scope.
    #[must_use]
    pub const fn scope(&self) -> ChannelScope {
        self.requirement.scope()
    }

    /// Returns the channel direction.
    #[must_use]
    pub const fn direction(&self) -> ChannelDirection {
        self.requirement.direction()
    }

    /// Returns the channel access mode.
    #[must_use]
    pub const fn access(&self) -> ChannelAccess {
        self.requirement.access()
    }

    /// Returns the semantic target.
    #[must_use]
    pub fn target(&self) -> &ChannelTarget {
        &self.target
    }

    /// Returns target constraints.
    #[must_use]
    pub const fn constraints(&self) -> &ChannelConstraints {
        &self.constraints
    }

    /// Returns metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ChannelMetadata {
        &self.metadata
    }

    /// Returns the number of explicitly targeted qubits.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.target.qubit_count()
    }

    /// Returns whether this is a global channel.
    #[must_use]
    pub fn is_global(&self) -> bool {
        self.target.is_global()
    }

    /// Returns whether this is a drive channel.
    #[must_use]
    pub fn is_drive(&self) -> bool {
        matches!(self.kind(), ChannelKind::Drive)
    }

    /// Returns whether this is a measurement/acquisition channel.
    #[must_use]
    pub fn is_measurement(&self) -> bool {
        self.kind().is_measurement_kind()
    }

    /// Returns whether this is a control channel.
    #[must_use]
    pub fn is_control(&self) -> bool {
        self.kind().is_control_kind()
    }

    /// Validates local semantic invariants.
    ///
    /// This deliberately does not validate target hardware.
    pub fn validate(&self) -> ChannelResult<()> {
        self.constraints.validate(&self.target)?;
        validate_scope_target(self.scope(), &self.target)?;

        if matches!(self.requirement.direction(), ChannelDirection::Output)
            && matches!(self.access(), ChannelAccess::WriteOnly)
        {
            return Err(ChannelError::DirectionAccessMismatch);
        }

        if matches!(self.requirement.direction(), ChannelDirection::Input)
            && matches!(self.access(), ChannelAccess::ReadOnly)
        {
            return Err(ChannelError::DirectionAccessMismatch);
        }

        Ok(())
    }
}

// =============================================================================
// Channel collection
// =============================================================================

/// Deterministic collection of abstract channels.
///
/// `ChannelSet` does not impose a fixed number of channels.
///
/// Resource/security limits belong to explicit compilation policy.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChannelSet {
    channels: BTreeMap<ChannelId, Channel>,
}

impl ChannelSet {
    /// Creates an empty channel set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a channel.
    ///
    /// Duplicate identities are rejected rather than silently overwritten.
    pub fn insert(&mut self, channel: Channel) -> ChannelResult<()> {
        channel.validate()?;

        let id = channel.id();

        if self.channels.contains_key(&id) {
            return Err(ChannelError::DuplicateChannelId { id });
        }

        self.channels.insert(id, channel);

        Ok(())
    }

    /// Gets a channel by identity.
    #[must_use]
    pub fn get(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.get(&id)
    }

    /// Gets a mutable channel by identity.
    pub fn get_mut(&mut self, id: ChannelId) -> Option<&mut Channel> {
        self.channels.get_mut(&id)
    }

    /// Removes a channel.
    pub fn remove(&mut self, id: ChannelId) -> Option<Channel> {
        self.channels.remove(&id)
    }

    /// Returns whether the collection contains an identity.
    #[must_use]
    pub fn contains(&self, id: ChannelId) -> bool {
        self.channels.contains_key(&id)
    }

    /// Returns the number of channels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.channels.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Returns deterministic iteration.
    pub fn iter(&self) -> impl Iterator<Item = (&ChannelId, &Channel)> {
        self.channels.iter()
    }

    /// Returns deterministic channel iteration without exposing the map.
    pub fn values(&self) -> impl Iterator<Item = &Channel> {
        self.channels.values()
    }

    /// Validates every channel in the collection.
    pub fn validate(&self) -> ChannelResult<()> {
        for channel in self.channels.values() {
            channel.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Standard constructors
// =============================================================================

/// Creates a semantic single-target drive channel.
///
/// The target may be logical or physical because channel semantics are
/// independent from routing.
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

/// Creates a semantic single-target control channel.
pub fn control_channel(
    id: ChannelId,
    qubit: QubitRef,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Control,
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

/// Creates a semantic measurement-interaction channel.
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

/// Creates a semantic acquisition channel.
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

/// Creates a semantic readout channel.
pub fn readout_channel(
    id: ChannelId,
    qubit: QubitRef,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Readout,
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
        ChannelTarget::Global,
        ChannelConstraints::new(),
        ChannelMetadata::new(),
    )
}

/// Creates a global communication channel.
pub fn communication_channel(
    id: ChannelId,
) -> ChannelResult<Channel> {
    Channel::try_new(
        id,
        ChannelRequirement::new(
            ChannelKind::Communication,
            ChannelScope::Global,
            ChannelDirection::Bidirectional,
            ChannelAccess::Shared,
        ),
        ChannelTarget::Global,
        ChannelConstraints::new(),
        ChannelMetadata::new(),
    )
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_non_empty_name(
    value: &str,
    field: &'static str,
) -> ChannelResult<()> {
    if value.trim().is_empty() {
        return Err(ChannelError::EmptyName { field });
    }

    Ok(())
}

fn validate_unique_qubits(
    qubits: &[QubitRef],
) -> ChannelResult<()> {
    let mut seen = BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(ChannelError::DuplicateTarget { target: *qubit });
        }
    }

    Ok(())
}

fn validate_scope_target(
    scope: ChannelScope,
    target: &ChannelTarget,
) -> ChannelResult<()> {
    match scope {
        ChannelScope::Global => {
            if !target.is_global() {
                return Err(ChannelError::ScopeTargetMismatch { scope });
            }
        }

        ChannelScope::PerTarget => {
            if target.qubit_count() != 1 {
                return Err(ChannelError::ScopeTargetMismatch { scope });
            }
        }

        ChannelScope::MultiTarget => {
            if target.qubit_count() == 0 {
                return Err(ChannelError::ScopeTargetMismatch { scope });
            }
        }

        ChannelScope::Pairwise => {
            if target.qubit_count() != 2 {
                return Err(ChannelError::ScopeTargetMismatch { scope });
            }
        }

        ChannelScope::Custom => {}
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors generated by local channel construction and validation.
///
/// These errors describe semantic IR problems. They do not describe hardware
/// failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelError {
    /// A required semantic name was empty.
    EmptyName {
        field: &'static str,
    },

    /// A multi-target channel was created with no targets.
    EmptyTarget,

    /// A qubit appears more than once in one channel target.
    DuplicateTarget {
        target: QubitRef,
    },

    /// Logical target use is forbidden by the channel constraints.
    LogicalTargetNotAllowed,

    /// Physical target use is forbidden by the channel constraints.
    PhysicalTargetNotAllowed,

    /// Global target use is forbidden by the channel constraints.
    GlobalTargetNotAllowed,

    /// Abstract resource target use is forbidden.
    ResourceTargetNotAllowed,

    /// Custom target use is forbidden.
    CustomTargetNotAllowed,

    /// Target count is below the semantic minimum.
    InsufficientTargets {
        minimum: usize,
        actual: usize,
    },

    /// Target count exceeds the explicitly declared semantic maximum.
    TooManyTargets {
        maximum: usize,
        actual: usize,
    },

    /// Channel scope does not match target structure.
    ScopeTargetMismatch {
        scope: ChannelScope,
    },

    /// Direction and access semantics are contradictory.
    DirectionAccessMismatch,

    /// A channel identity already exists in a channel set.
    DuplicateChannelId {
        id: ChannelId,
    },
}

impl fmt::Display for ChannelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::EmptyTarget => {
                write!(formatter, "channel target cannot be empty")
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

            Self::ResourceTargetNotAllowed => {
                write!(
                    formatter,
                    "abstract resource target is not allowed by channel constraints"
                )
            }

            Self::CustomTargetNotAllowed => {
                write!(
                    formatter,
                    "custom channel target is not allowed by channel constraints"
                )
            }

            Self::InsufficientTargets { minimum, actual } => {
                write!(
                    formatter,
                    "channel requires at least {minimum} target(s), received {actual}"
                )
            }

            Self::TooManyTargets { maximum, actual } => {
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

            Self::DirectionAccessMismatch => {
                write!(
                    formatter,
                    "channel direction and access policy are incompatible"
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
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    fn channel_id(value: u64) -> ChannelId {
        ChannelId::new(value)
    }

    fn logical(index: usize) -> QubitRef {
        QubitRef::Logical(QubitId::new(index))
    }

    fn physical(index: usize) -> QubitRef {
        QubitRef::Physical(PhysicalQubitId::new(index))
    }

    #[test]
    fn channel_id_is_stable_and_strongly_typed() {
        let id = channel_id(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "channel42");
    }

    #[test]
    fn drive_channel_uses_canonical_qubit_reference() {
        let channel = drive_channel(channel_id(1), logical(0))
            .expect("drive channel must be valid");

        assert!(channel.is_drive());
        assert_eq!(channel.target_count(), 1);
        assert_eq!(channel.scope(), ChannelScope::PerTarget);
        assert_eq!(channel.direction(), ChannelDirection::Input);
    }

    #[test]
    fn physical_and_logical_targets_are_distinct() {
        let logical_target = ChannelTarget::qubit(logical(0));
        let physical_target = ChannelTarget::qubit(physical(0));

        assert_ne!(logical_target, physical_target);
    }

    #[test]
    fn duplicate_qubit_targets_are_rejected() {
        let q = logical(0);

        let result = ChannelTarget::qubits(vec![q, q]);

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateTarget { .. })
        ));
    }

    #[test]
    fn empty_multi_target_is_rejected() {
        let result = ChannelTarget::qubits(Vec::new());

        assert!(matches!(
            result,
            Err(ChannelError::EmptyTarget)
        ));
    }

    #[test]
    fn global_channel_is_structurally_valid() {
        let channel = synchronization_channel(channel_id(2))
            .expect("global synchronization channel must be valid");

        assert!(channel.is_global());
        assert_eq!(channel.scope(), ChannelScope::Global);
        assert_eq!(channel.target_count(), 0);
    }

    #[test]
    fn per_target_channel_rejects_multiple_qubits() {
        let target = ChannelTarget::qubits(vec![logical(0), logical(1)])
            .expect("two distinct qubits should be structurally valid");

        let result = Channel::try_new(
            channel_id(3),
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
            Err(ChannelError::ScopeTargetMismatch {
                scope: ChannelScope::PerTarget
            })
        ));
    }

    #[test]
    fn pairwise_channel_requires_exactly_two_qubits() {
        let target = ChannelTarget::qubits(vec![logical(0), logical(1)])
            .expect("pair target should be valid");

        let channel = Channel::try_new(
            channel_id(4),
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
        .expect("pairwise channel must be valid");

        assert_eq!(channel.target_count(), 2);
        assert_eq!(channel.scope(), ChannelScope::Pairwise);
    }

    #[test]
    fn logical_targets_can_be_forbidden() {
        let result = Channel::try_new(
            channel_id(5),
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            ChannelTarget::qubit(logical(0)),
            ChannelConstraints::new()
                .with_logical_targets(false),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(ChannelError::LogicalTargetNotAllowed)
        ));
    }

    #[test]
    fn physical_targets_can_be_forbidden() {
        let result = Channel::try_new(
            channel_id(6),
            ChannelRequirement::new(
                ChannelKind::Drive,
                ChannelScope::PerTarget,
                ChannelDirection::Input,
                ChannelAccess::Exclusive,
            ),
            ChannelTarget::qubit(physical(0)),
            ChannelConstraints::new()
                .with_physical_targets(false),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(ChannelError::PhysicalTargetNotAllowed)
        ));
    }

    #[test]
    fn resource_targets_are_supported() {
        let target = ChannelTarget::resource(ResourceId::new(99));

        let channel = Channel::try_new(
            channel_id(7),
            ChannelRequirement::new(
                ChannelKind::Analog,
                ChannelScope::Custom,
                ChannelDirection::Input,
                ChannelAccess::TargetDefined,
            ),
            target,
            ChannelConstraints::new(),
            ChannelMetadata::new(),
        )
        .expect("resource target must be supported");

        assert_eq!(
            channel.target(),
            &ChannelTarget::Resource(ResourceId::new(99))
        );
    }

    #[test]
    fn custom_channel_kind_is_supported() {
        let kind = ChannelKind::custom("neutral_atom.tweezer")
            .expect("custom channel kind should be valid");

        assert!(kind.is_custom());
        assert_eq!(kind.name(), "neutral_atom.tweezer");
    }

    #[test]
    fn custom_channel_target_is_supported() {
        let target = ChannelTarget::custom("photonic.mode.group")
            .expect("custom target should be valid");

        assert_eq!(
            target,
            ChannelTarget::Custom(
                "photonic.mode.group".to_owned()
            )
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = ChannelMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata insertion must succeed");

        metadata
            .insert("a", "first")
            .expect("metadata insertion must succeed");

        let values: Vec<_> = metadata.iter().collect();

        assert_eq!(
            values,
            vec![
                ("a", "first"),
                ("z", "last"),
            ]
        );
    }

    #[test]
    fn metadata_replacement_is_deterministic() {
        let mut metadata = ChannelMetadata::new();

        metadata
            .insert("mode", "initial")
            .expect("metadata insertion must succeed");

        metadata
            .insert("mode", "updated")
            .expect("metadata replacement must succeed");

        assert_eq!(metadata.get("mode"), Some("updated"));
        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn channel_set_rejects_duplicate_identity() {
        let first = drive_channel(channel_id(10), logical(0))
            .expect("first channel must be valid");

        let second = drive_channel(channel_id(10), logical(1))
            .expect("second channel must be locally valid");

        let mut set = ChannelSet::new();

        set.insert(first)
            .expect("first insertion must succeed");

        assert!(matches!(
            set.insert(second),
            Err(ChannelError::DuplicateChannelId { .. })
        ));
    }

    #[test]
    fn channel_set_is_deterministic() {
        let mut set = ChannelSet::new();

        for id in [30_u64, 10_u64, 20_u64] {
            set.insert(
                drive_channel(channel_id(id), logical(id as usize))
                    .expect("channel must be valid"),
            )
            .expect("insertion must succeed");
        }

        let ids: Vec<u64> = set
            .iter()
            .map(|(id, _)| id.value())
            .collect();

        assert_eq!(ids, vec![10, 20, 30]);
    }

    #[test]
    fn very_large_identity_remains_representable() {
        let channel = synchronization_channel(
            ChannelId::new(u64::MAX),
        )
        .expect("large channel identity must be representable");

        assert_eq!(channel.id().value(), u64::MAX);
    }

    #[test]
    fn no_implicit_channel_count_limit_exists() {
        let mut set = ChannelSet::new();

        for value in 0_u64..1024 {
            set.insert(
                synchronization_channel(ChannelId::new(value))
                    .expect("channel must be valid"),
            )
            .expect("channel insertion must succeed");
        }

        assert_eq!(set.len(), 1024);
    }

    #[test]
    fn measurement_channel_has_input_semantics() {
        let channel = measure_channel(channel_id(20), logical(7))
            .expect("measurement channel must be valid");

        assert!(channel.is_measurement());
        assert_eq!(
            channel.direction(),
            ChannelDirection::Input
        );
    }

    #[test]
    fn acquisition_channel_has_output_semantics() {
        let channel = acquire_channel(channel_id(21), logical(7))
            .expect("acquisition channel must be valid");

        assert!(channel.is_measurement());
        assert_eq!(
            channel.direction(),
            ChannelDirection::Output
        );
    }

    #[test]
    fn contradictory_direction_and_access_are_rejected() {
        let result = Channel::try_new(
            channel_id(22),
            ChannelRequirement::new(
                ChannelKind::Acquire,
                ChannelScope::PerTarget,
                ChannelDirection::Output,
                ChannelAccess::WriteOnly,
            ),
            ChannelTarget::qubit(logical(0)),
            ChannelConstraints::new(),
            ChannelMetadata::new(),
        );

        assert!(matches!(
            result,
            Err(ChannelError::DirectionAccessMismatch)
        ));
    }

    #[test]
    fn target_constraints_are_explicit_not_global_limits() {
        let constraints = ChannelConstraints::new()
            .with_minimum_targets(1)
            .with_maximum_targets(None);

        let target = ChannelTarget::qubits(vec![
            logical(0),
            logical(1),
            logical(2),
            logical(3),
        ])
        .expect("targets must be valid");

        constraints
            .validate(&target)
            .expect("unbounded semantic target constraint must accept target");
    }

    #[test]
    fn target_order_is_preserved() {
        let target = ChannelTarget::qubits(vec![
            logical(3),
            logical(1),
            logical(2),
        ])
        .expect("targets must be valid");

        let values: Vec<_> = target.qubits().collect();

        assert_eq!(
            values,
            vec![
                logical(3),
                logical(1),
                logical(2),
            ]
        );
    }

    #[test]
    fn channel_validation_is_idempotent() {
        let channel = drive_channel(channel_id(30), logical(4))
            .expect("channel must be valid");

        channel
            .validate()
            .expect("first validation must succeed");

        channel
            .validate()
            .expect("second validation must succeed");
    }
}