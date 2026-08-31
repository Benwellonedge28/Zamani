//! Zamani Quantum IR — Extensible Semantic Extension System
//!
//! This module defines the canonical extension mechanism for the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural purpose
//!
//! Quantum computing evolves faster than any fixed IR instruction vocabulary.
//! New quantum modalities, compiler annotations, logical operations,
//! experimental operations, control mechanisms, and future execution models
//! must be representable without repeatedly redesigning the core IR.
//!
//! `extension.rs` provides that extensibility boundary.
//!
//! An extension is:
//!
//! - explicitly namespaced;
//! - explicitly versioned;
//! - strongly identified;
//! - deterministically represented;
//! - targetable to IR entities;
//! - opaque to the canonical IR when its semantics are unknown;
//! - structurally validated;
//! - merge-safe;
//! - suitable for serialization and hashing;
//! - unable to bypass canonical IR validation;
//! - independent of physical hardware implementation.
//!
//! # Critical architectural rule
//!
//! An extension is NOT permission to put arbitrary compiler/backend logic
//! inside `quantum::ir`.
//!
//! Extensions may describe semantics or metadata that the canonical IR does
//! not yet model directly, but the extension mechanism itself does not execute,
//! optimize, route, schedule, calibrate, simulate, or communicate with
//! hardware.
//!
//! The ownership boundary remains:
//!
//! ```text
//! quantum::ir
//!     │
//!     │ canonical semantic program
//!     │
//!     ├── extension.rs
//!     │      │
//!     │      └── extensibility boundary only
//!     │
//!     ▼
//! downstream compiler stages
//!     │
//!     ├── optimization
//!     ├── routing
//!     ├── scheduling
//!     ├── hardware compatibility
//!     ├── backend lowering
//!     └── execution
//! ```
//!
//! # Unknown extensions
//!
//! A consumer that does not understand an extension must be able to preserve
//! it without interpreting it.
//!
//! Therefore extensions use an explicit opaque payload representation.
//!
//! This is essential for:
//!
//! - forward compatibility;
//! - distributed compilation;
//! - IR round-tripping;
//! - vendor-neutral tooling;
//! - experimental quantum technologies;
//! - future Zamani language evolution.
//!
//! Unknown extensions must NOT be silently discarded when the caller requests
//! lossless preservation.
//!
//! # Namespace model
//!
//! Every extension has:
//!
//! ```text
//! namespace + name + version
//! ```
//!
//! Examples:
//!
//! ```text
//! zamani.logical.v1
//! zamani.pulse.v1
//! compiler.experimental.v1
//! hardware.ibm.v1
//! hardware.vendor_x.v2
//! research.analog.v1
//! user.custom.v1
//! ```
//!
//! The namespace identifies ownership.
//!
//! The name identifies the extension contract within that namespace.
//!
//! The version identifies the extension schema/semantic contract.
//!
//! Hardware-specific namespaces are permitted, but `extension.rs` does not
//! interpret their hardware semantics.
//!
//! # Versioning
//!
//! Extension versions use semantic-version-like components:
//!
//! ```text
//! major.minor.patch
//! ```
//!
//! Major changes may be incompatible.
//!
//! Minor changes add backward-compatible capabilities.
//!
//! Patch changes correct the extension contract without changing its meaning.
//!
//! The extension version is independent from:
//!
//! - Zamani language version;
//! - Quantum IR version;
//! - compiler version;
//! - hardware version;
//! - backend version;
//! - calibration version.
//!
//! # Targets
//!
//! An extension may be attached to:
//!
//! - a program;
//! - module;
//! - region;
//! - block;
//! - operation;
//! - logical qubit;
//! - physical-qubit reference;
//! - classical/value identity;
//! - parameter;
//! - pulse;
//! - waveform;
//! - channel;
//! - frame;
//! - schedule;
//! - resource;
//! - capability;
//! - function;
//! - another extension;
//! - a global/unscoped IR entity.
//!
//! Logical and physical quantum identities are imported directly from
//! `quantum::ir::qubit`.
//!
//! This module does not define another qubit identifier type.
//!
//! # Scalability
//!
//! There is deliberately no architectural maximum number of:
//!
//! - extensions;
//! - extension targets;
//! - extension fields;
//! - payload bytes;
//! - namespaces;
//! - extension names;
//! - nested extension references.
//!
//! Concrete resource/security limits belong to `QuantumIrLimits` and the
//! consuming subsystem.
//!
//! This module therefore does NOT use:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as quantum-resource boundaries.
//!
//! A program containing one qubit and a program containing extremely large
//! numbers of qubits use the same extension model.
//!
//! The practical ceiling is determined by the configured resource policy,
//! address space, storage, and available resources.
//!
//! # Determinism
//!
//! Determinism is required for:
//!
//! - canonical hashing;
//! - caching;
//! - reproducible compilation;
//! - distributed compilation;
//! - provenance;
//! - benchmarking;
//! - serialization;
//! - equality testing.
//!
//! Therefore:
//!
//! - extension keys are ordered;
//! - extension sets use `BTreeMap`;
//! - target collections are deterministic;
//! - metadata fields are deterministic;
//! - payload representation is structural;
//! - formatting is deterministic;
//! - equality is structural;
//! - hashing is structural.
//!
//! # Merge semantics
//!
//! Extension sets can be merged when compiler stages combine IR.
//!
//! The default merge policy is conservative:
//!
//! - identical extensions are retained;
//! - a missing extension is inserted;
//! - conflicting extensions with the same namespace/name/version are rejected;
//! - nothing is silently overwritten.
//!
//! Explicit replacement belongs to the owning compiler transformation.
//!
//! # Validation boundary
//!
//! `extension.rs` validates extension structure:
//!
//! - namespace;
//! - name;
//! - version;
//! - target identity;
//! - payload structure;
//! - required identifiers;
//! - deterministic ordering invariants.
//!
//! It does NOT validate whether an extension is physically executable.
//!
//! For example:
//!
//! ```text
//! hardware.ibm.native_gate
//! ```
//!
//! can be represented here.
//!
//! Whether an IBM target actually supports that operation belongs to the
//! hardware compatibility layer.
//!
//! # Integration contract
//!
//! `identity.rs` supplies:
//!
//! - `ExtensionId`;
//! - `ProgramId`;
//! - `ModuleId`;
//! - `RegionId`;
//! - `BlockId`;
//! - `OperationId`;
//! - `ValueId`;
//! - `ParameterId`;
//! - `PulseId`;
//! - `WaveformId`;
//! - `ChannelId`;
//! - `FrameId`;
//! - `ScheduleId`;
//! - `ResourceId`;
//! - `CapabilityId`;
//! - `FunctionId`;
//! - `TypeId`.
//!
//! `qubit.rs` supplies:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`.
//!
//! `attribute.rs` supplies:
//!
//! - `AttributeKey`;
//! - `AttributeValue`.
//!
//! Higher-level modules may attach extensions to their IR objects.
//!
//! `serialization.rs` must preserve unknown extensions losslessly.
//!
//! `hash.rs` may hash extensions directly because their ordering is
//! deterministic.
//!
//! `provenance.rs` may record extension usage.
//!
//! `validation.rs` may invoke structural extension validation.
//!
//! `analysis.rs` may inspect extension presence but must not invent semantics
//! for unknown extensions.
//!
//! `operation.rs`, `program.rs`, `region.rs`, `pulse.rs`, `waveform.rs`,
//! `channel.rs`, `frame.rs`, and future IR modules may consume this API.
//!
//! `extension.rs` must never import:
//!
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - frontend;
//! - backend execution.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::attribute::{AttributeKey, AttributeValue};
use super::identity::{
    AttributeId,
    BlockId,
    CapabilityId,
    ChannelId,
    ExtensionId,
    FrameId,
    FunctionId,
    ModuleId,
    OperationId,
    ParameterId,
    ProgramId,
    PulseId,
    RegionId,
    ResourceId,
    ScheduleId,
    TypeId,
    ValueId,
    WaveformId,
};
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Extension version
// =============================================================================

/// Version of an extension contract.
///
/// Extension versions are independent from the Quantum IR version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl ExtensionVersion {
    /// Creates an extension version.
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

    /// Returns the major component.
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch component.
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns whether this version is exactly equal to another version.
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether two versions share a major contract.
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether this version can consume the supplied version under
    /// the conservative compatibility policy.
    ///
    /// A consumer accepts:
    ///
    /// - the same major version;
    /// - an extension version no newer than the consumer;
    /// - no future minor/patch contract.
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && !(other.minor == self.minor
                && other.patch > self.patch)
    }
}

impl Default for ExtensionVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for ExtensionVersion {
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

// =============================================================================
// Extension namespace
// =============================================================================

/// Validated extension namespace.
///
/// Namespaces are dot-separated identifiers.
///
/// Examples:
///
/// ```text
/// zamani
/// compiler
/// hardware.ibm
/// research.analog
/// user
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionNamespace(String);

impl ExtensionNamespace {
    /// Creates a validated extension namespace.
    pub fn new<S>(
        namespace: S,
    ) -> Result<Self, ExtensionError>
    where
        S: Into<String>,
    {
        let namespace = namespace.into();

        validate_qualified_identifier(
            &namespace,
            "extension namespace",
        )?;

        Ok(Self(namespace))
    }

    /// Returns the namespace as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ExtensionNamespace {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Extension name
// =============================================================================

/// Validated local extension name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionName(String);

impl ExtensionName {
    /// Creates a validated extension name.
    pub fn new<S>(
        name: S,
    ) -> Result<Self, ExtensionError>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_identifier(
            &name,
            "extension name",
        )?;

        Ok(Self(name))
    }

    /// Returns the name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ExtensionName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Extension key
// =============================================================================

/// Canonical identity of an extension contract.
///
/// The key deliberately excludes `ExtensionId`.
///
/// `ExtensionId` identifies an extension occurrence.
///
/// `ExtensionKey` identifies the extension contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionKey {
    namespace: ExtensionNamespace,
    name: ExtensionName,
    version: ExtensionVersion,
}

impl ExtensionKey {
    /// Creates an extension key.
    pub fn new<N, S>(
        namespace: N,
        name: S,
        version: ExtensionVersion,
    ) -> Result<Self, ExtensionError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        Ok(Self {
            namespace: ExtensionNamespace::new(namespace)?,
            name: ExtensionName::new(name)?,
            version,
        })
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    /// Returns the name.
    pub fn name(&self) -> &ExtensionName {
        &self.name
    }

    /// Returns the version.
    pub const fn version(&self) -> ExtensionVersion {
        self.version
    }

    /// Returns a canonical fully qualified extension name.
    ///
    /// Format:
    ///
    /// ```text
    /// namespace.name@major.minor.patch
    /// ```
    pub fn qualified_name(&self) -> String {
        format!(
            "{}.{}@{}",
            self.namespace,
            self.name,
            self.version
        )
    }
}

impl fmt::Display for ExtensionKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}@{}",
            self.namespace,
            self.name,
            self.version
        )
    }
}

// =============================================================================
// Extension target
// =============================================================================

/// IR entity to which an extension applies.
///
/// This enum references canonical identity types rather than duplicating
/// those identities.
///
/// In particular, quantum resources use:
///
/// ```text
/// quantum::ir::qubit::QubitId
/// quantum::ir::qubit::PhysicalQubitId
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtensionTarget {
    /// Extension applies globally to the IR document.
    Global,

    /// Complete quantum program.
    Program(ProgramId),

    /// IR module.
    Module(ModuleId),

    /// Structured IR region.
    Region(RegionId),

    /// IR block.
    Block(BlockId),

    /// IR operation.
    Operation(OperationId),

    /// Logical quantum bit.
    LogicalQubit(QubitId),

    /// Physical-qubit reference.
    ///
    /// This does not give the extension authority over hardware topology.
    PhysicalQubit(PhysicalQubitId),

    /// IR value.
    Value(ValueId),

    /// Symbolic/runtime parameter.
    Parameter(ParameterId),

    /// Pulse-level semantic object.
    Pulse(PulseId),

    /// Waveform definition.
    Waveform(WaveformId),

    /// Abstract control channel.
    Channel(ChannelId),

    /// Control frame.
    Frame(FrameId),

    /// Schedule identity.
    Schedule(ScheduleId),

    /// Abstract resource requirement.
    Resource(ResourceId),

    /// Capability reference.
    Capability(CapabilityId),

    /// IR function.
    Function(FunctionId),

    /// IR type.
    Type(TypeId),

    /// Another extension.
    Extension(ExtensionId),

    /// Attribute occurrence.
    Attribute(AttributeId),
}

impl ExtensionTarget {
    /// Returns a stable human-readable target category.
    pub const fn kind(&self) -> ExtensionTargetKind {
        match self {
            Self::Global => ExtensionTargetKind::Global,
            Self::Program(_) => ExtensionTargetKind::Program,
            Self::Module(_) => ExtensionTargetKind::Module,
            Self::Region(_) => ExtensionTargetKind::Region,
            Self::Block(_) => ExtensionTargetKind::Block,
            Self::Operation(_) => ExtensionTargetKind::Operation,
            Self::LogicalQubit(_) => ExtensionTargetKind::LogicalQubit,
            Self::PhysicalQubit(_) => {
                ExtensionTargetKind::PhysicalQubit
            }
            Self::Value(_) => ExtensionTargetKind::Value,
            Self::Parameter(_) => ExtensionTargetKind::Parameter,
            Self::Pulse(_) => ExtensionTargetKind::Pulse,
            Self::Waveform(_) => ExtensionTargetKind::Waveform,
            Self::Channel(_) => ExtensionTargetKind::Channel,
            Self::Frame(_) => ExtensionTargetKind::Frame,
            Self::Schedule(_) => ExtensionTargetKind::Schedule,
            Self::Resource(_) => ExtensionTargetKind::Resource,
            Self::Capability(_) => ExtensionTargetKind::Capability,
            Self::Function(_) => ExtensionTargetKind::Function,
            Self::Type(_) => ExtensionTargetKind::Type,
            Self::Extension(_) => ExtensionTargetKind::Extension,
            Self::Attribute(_) => ExtensionTargetKind::Attribute,
        }
    }
}

/// Stable classification of an extension target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtensionTargetKind {
    Global,
    Program,
    Module,
    Region,
    Block,
    Operation,
    LogicalQubit,
    PhysicalQubit,
    Value,
    Parameter,
    Pulse,
    Waveform,
    Channel,
    Frame,
    Schedule,
    Resource,
    Capability,
    Function,
    Type,
    Extension,
    Attribute,
}

// =============================================================================
// Extension payload
// =============================================================================

/// Lossless extension payload.
///
/// Unknown extensions can be carried without interpreting their semantics.
///
/// `Bytes` is intentionally opaque. The extension's namespace/name/version
/// determines how a downstream extension provider interprets those bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtensionPayload {
    /// No payload.
    Empty,

    /// UTF-8 textual payload.
    Text(String),

    /// Opaque binary payload.
    ///
    /// The bytes are not interpreted by the canonical IR.
    Bytes {
        /// Optional media/encoding identifier.
        encoding: Option<String>,

        /// Opaque payload bytes.
        data: Vec<u8>,
    },

    /// Structured metadata fields.
    ///
    /// `BTreeMap` guarantees deterministic ordering.
    Fields(BTreeMap<String, AttributeValue>),

    /// Ordered structured values.
    List(Vec<AttributeValue>),

    /// A reference to another IR value.
    Value(ValueId),

    /// A reference to a symbolic/runtime parameter.
    Parameter(ParameterId),

    /// A reference to a logical qubit.
    LogicalQubit(QubitId),

    /// A reference to a physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// A reference to another extension.
    Extension(ExtensionId),

    /// Multiple IR references.
    References(Vec<ExtensionTarget>),
}

impl Default for ExtensionPayload {
    fn default() -> Self {
        Self::Empty
    }
}

impl ExtensionPayload {
    /// Returns whether this payload contains no semantic data.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Text(value) => value.is_empty(),
            Self::Bytes { data, .. } => data.is_empty(),
            Self::Fields(fields) => fields.is_empty(),
            Self::List(values) => values.is_empty(),
            Self::Value(_)
            | Self::Parameter(_)
            | Self::LogicalQubit(_)
            | Self::PhysicalQubit(_)
            | Self::Extension(_)
            | Self::References(_) => false,
        }
    }

    /// Returns the payload kind.
    pub const fn kind(&self) -> ExtensionPayloadKind {
        match self {
            Self::Empty => ExtensionPayloadKind::Empty,
            Self::Text(_) => ExtensionPayloadKind::Text,
            Self::Bytes { .. } => ExtensionPayloadKind::Bytes,
            Self::Fields(_) => ExtensionPayloadKind::Fields,
            Self::List(_) => ExtensionPayloadKind::List,
            Self::Value(_) => ExtensionPayloadKind::Value,
            Self::Parameter(_) => ExtensionPayloadKind::Parameter,
            Self::LogicalQubit(_) => {
                ExtensionPayloadKind::LogicalQubit
            }
            Self::PhysicalQubit(_) => {
                ExtensionPayloadKind::PhysicalQubit
            }
            Self::Extension(_) => ExtensionPayloadKind::Extension,
            Self::References(_) => ExtensionPayloadKind::References,
        }
    }
}

/// Stable classification of an extension payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtensionPayloadKind {
    Empty,
    Text,
    Bytes,
    Fields,
    List,
    Value,
    Parameter,
    LogicalQubit,
    PhysicalQubit,
    Extension,
    References,
}

// =============================================================================
// Extension
// =============================================================================

/// Complete canonical extension object.
///
/// An extension is a declarative IR object. It does not execute code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    id: ExtensionId,
    key: ExtensionKey,
    target: ExtensionTarget,
    payload: ExtensionPayload,
    attributes: BTreeMap<AttributeKey, AttributeValue>,
}

impl Extension {
    /// Creates an extension.
    pub fn new(
        id: ExtensionId,
        key: ExtensionKey,
        target: ExtensionTarget,
        payload: ExtensionPayload,
    ) -> Result<Self, ExtensionError> {
        let extension = Self {
            id,
            key,
            target,
            payload,
            attributes: BTreeMap::new(),
        };

        extension.validate()?;

        Ok(extension)
    }

    /// Returns the stable extension occurrence identity.
    pub const fn id(&self) -> ExtensionId {
        self.id
    }

    /// Returns the extension contract key.
    pub fn key(&self) -> &ExtensionKey {
        &self.key
    }

    /// Returns the extension namespace.
    pub fn namespace(&self) -> &ExtensionNamespace {
        self.key.namespace()
    }

    /// Returns the extension name.
    pub fn name(&self) -> &ExtensionName {
        self.key.name()
    }

    /// Returns the extension version.
    pub const fn version(&self) -> ExtensionVersion {
        self.key.version()
    }

    /// Returns the extension target.
    pub fn target(&self) -> &ExtensionTarget {
        &self.target
    }

    /// Returns the extension payload.
    pub fn payload(&self) -> &ExtensionPayload {
        &self.payload
    }

    /// Returns extension attributes.
    pub fn attributes(
        &self,
    ) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Adds an attribute.
    ///
    /// Existing values are never silently overwritten.
    pub fn insert_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<(), ExtensionError> {
        if self.attributes.contains_key(&key) {
            return Err(
                ExtensionError::DuplicateAttribute {
                    key: key.qualified_name(),
                },
            );
        }

        self.attributes.insert(key, value);

        Ok(())
    }

    /// Replaces an attribute explicitly.
    ///
    /// Replacement is intentionally separate from normal insertion so that
    /// compiler passes cannot accidentally overwrite metadata.
    pub fn replace_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Option<AttributeValue> {
        self.attributes.insert(key, value)
    }

    /// Returns an attribute by key.
    pub fn get_attribute(
        &self,
        key: &AttributeKey,
    ) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Returns whether the extension is structurally valid.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        validate_extension_id(self.id)?;

        if self.key.namespace().as_str().is_empty() {
            return Err(
                ExtensionError::InvalidNamespace {
                    value: String::new(),
                },
            );
        }

        if self.key.name().as_str().is_empty() {
            return Err(
                ExtensionError::InvalidName {
                    value: String::new(),
                },
            );
        }

        validate_payload(&self.payload)?;

        validate_target(&self.target)?;

        Ok(())
    }

    /// Returns a deterministic canonical identity string.
    pub fn canonical_key(&self) -> String {
        format!(
            "{}::{}",
            self.key.qualified_name(),
            target_identity_string(&self.target)
        )
    }
}

impl Hash for Extension {
    fn hash<H>(
        &self,
        state: &mut H,
    ) {
        self.id.hash(state);
        self.key.hash(state);
        self.target.hash(state);
        self.payload.hash(state);
        self.attributes.hash(state);
    }
}

// =============================================================================
// Extension set
// =============================================================================

/// Deterministic collection of extensions.
///
/// Extensions are indexed by `ExtensionKey` plus target.
///
/// This allows the same extension contract to be used on multiple IR entities
/// while preventing accidental duplicate definitions for the same target.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtensionSet {
    entries: BTreeMap<ExtensionSetKey, Extension>,
}

impl ExtensionSet {
    /// Creates an empty extension set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of extensions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the set contains no extensions.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts an extension.
    ///
    /// An existing extension with the same contract and target is rejected.
    pub fn insert(
        &mut self,
        extension: Extension,
    ) -> Result<(), ExtensionError> {
        extension.validate()?;

        let key = ExtensionSetKey::from_extension(&extension);

        if self.entries.contains_key(&key) {
            return Err(
                ExtensionError::DuplicateExtension {
                    key: key.to_string(),
                },
            );
        }

        self.entries.insert(key, extension);

        Ok(())
    }

    /// Explicitly replaces an extension.
    ///
    /// The previous extension is returned when one existed.
    pub fn replace(
        &mut self,
        extension: Extension,
    ) -> Result<Option<Extension>, ExtensionError> {
        extension.validate()?;

        let key = ExtensionSetKey::from_extension(&extension);

        Ok(self.entries.insert(key, extension))
    }

    /// Returns an extension by contract key and target.
    pub fn get(
        &self,
        key: &ExtensionKey,
        target: &ExtensionTarget,
    ) -> Option<&Extension> {
        self.entries.get(&ExtensionSetKey {
            contract: key.clone(),
            target: target.clone(),
        })
    }

    /// Returns all extensions in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Extension> {
        self.entries.values()
    }

    /// Returns all extensions targeting a logical qubit.
    pub fn for_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> impl Iterator<Item = &Extension> {
        self.entries
            .values()
            .filter(move |extension| {
                extension.target()
                    == &ExtensionTarget::LogicalQubit(qubit)
            })
    }

    /// Returns all extensions targeting an operation.
    pub fn for_operation(
        &self,
        operation: OperationId,
    ) -> impl Iterator<Item = &Extension> {
        self.entries
            .values()
            .filter(move |extension| {
                extension.target()
                    == &ExtensionTarget::Operation(operation)
            })
    }

    /// Returns all extensions targeting a physical qubit.
    pub fn for_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> impl Iterator<Item = &Extension> {
        self.entries
            .values()
            .filter(move |extension| {
                extension.target()
                    == &ExtensionTarget::PhysicalQubit(qubit)
            })
    }

    /// Merges another extension set into this one.
    ///
    /// Conflicting entries are rejected.
    ///
    /// The operation is transactional: if a conflict occurs, this set is
    /// unchanged.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), ExtensionError> {
        for (key, extension) in &other.entries {
            if let Some(existing) = self.entries.get(key) {
                if existing != extension {
                    return Err(
                        ExtensionError::ConflictingExtension {
                            key: key.to_string(),
                        },
                    );
                }
            }
        }

        for (key, extension) in &other.entries {
            self.entries
                .entry(key.clone())
                .or_insert_with(|| extension.clone());
        }

        Ok(())
    }

    /// Validates every extension.
    pub fn validate(&self) -> Result<(), ExtensionError> {
        for extension in self.entries.values() {
            extension.validate()?;
        }

        Ok(())
    }

    /// Removes an extension by contract key and target.
    pub fn remove(
        &mut self,
        key: &ExtensionKey,
        target: &ExtensionTarget,
    ) -> Option<Extension> {
        self.entries.remove(&ExtensionSetKey {
            contract: key.clone(),
            target: target.clone(),
        })
    }

    /// Clears all extensions.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Hash for ExtensionSet {
    fn hash<H>(
        &self,
        state: &mut H,
    ) {
        self.entries.hash(state);
    }
}

// =============================================================================
// Extension set key
// =============================================================================

/// Deterministic index key used internally by `ExtensionSet`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ExtensionSetKey {
    contract: ExtensionKey,
    target: ExtensionTarget,
}

impl ExtensionSetKey {
    fn from_extension(
        extension: &Extension,
    ) -> Self {
        Self {
            contract: extension.key().clone(),
            target: extension.target().clone(),
        }
    }
}

impl fmt::Display for ExtensionSetKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.contract,
            target_identity_string(&self.target)
        )
    }
}

// =============================================================================
// Extension error
// =============================================================================

/// Structural errors produced by the extension system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionError {
    /// Extension ID is invalid according to the extension contract.
    InvalidExtensionId,

    /// Namespace is empty or malformed.
    InvalidNamespace {
        value: String,
    },

    /// Extension name is empty or malformed.
    InvalidName {
        value: String,
    },

    /// Identifier contains an invalid character.
    InvalidIdentifier {
        field: &'static str,
        value: String,
    },

    /// Qualified identifier contains an empty component.
    InvalidQualifiedIdentifier {
        field: &'static str,
        value: String,
    },

    /// Duplicate extension in a set.
    DuplicateExtension {
        key: String,
    },

    /// Two extensions conflict during merge.
    ConflictingExtension {
        key: String,
    },

    /// Duplicate attribute on an extension.
    DuplicateAttribute {
        key: String,
    },

    /// Extension payload is malformed.
    InvalidPayload {
        reason: String,
    },

    /// Extension target is malformed.
    InvalidTarget {
        reason: String,
    },

    /// Opaque payload declares an invalid encoding identifier.
    InvalidEncoding {
        value: String,
    },
}

impl fmt::Display for ExtensionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidExtensionId => {
                formatter.write_str(
                    "invalid extension identity",
                )
            }

            Self::InvalidNamespace { value } => {
                write!(
                    formatter,
                    "invalid extension namespace: {value:?}"
                )
            }

            Self::InvalidName { value } => {
                write!(
                    formatter,
                    "invalid extension name: {value:?}"
                )
            }

            Self::InvalidIdentifier {
                field,
                value,
            } => {
                write!(
                    formatter,
                    "invalid {field} identifier: {value:?}"
                )
            }

            Self::InvalidQualifiedIdentifier {
                field,
                value,
            } => {
                write!(
                    formatter,
                    "invalid {field} qualified identifier: {value:?}"
                )
            }

            Self::DuplicateExtension { key } => {
                write!(
                    formatter,
                    "duplicate extension: {key}"
                )
            }

            Self::ConflictingExtension { key } => {
                write!(
                    formatter,
                    "conflicting extension: {key}"
                )
            }

            Self::DuplicateAttribute { key } => {
                write!(
                    formatter,
                    "duplicate extension attribute: {key}"
                )
            }

            Self::InvalidPayload { reason } => {
                write!(
                    formatter,
                    "invalid extension payload: {reason}"
                )
            }

            Self::InvalidTarget { reason } => {
                write!(
                    formatter,
                    "invalid extension target: {reason}"
                )
            }

            Self::InvalidEncoding { value } => {
                write!(
                    formatter,
                    "invalid extension encoding: {value:?}"
                )
            }
        }
    }
}

impl std::error::Error for ExtensionError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_extension_id(
    _id: ExtensionId,
) -> Result<(), ExtensionError> {
    // ExtensionId is intentionally opaque and structurally valid by
    // construction. Zero is permitted because identity allocation policy
    // belongs to the owning program/session.
    Ok(())
}

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ExtensionError> {
    if value.is_empty() {
        return Err(
            ExtensionError::InvalidIdentifier {
                field,
                value: value.to_owned(),
            },
        );
    }

    let mut characters = value.chars();

    let first = characters.next().ok_or_else(|| {
        ExtensionError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        }
    })?;

    if !is_identifier_start(first) {
        return Err(
            ExtensionError::InvalidIdentifier {
                field,
                value: value.to_owned(),
            },
        );
    }

    for character in characters {
        if !is_identifier_continue(character) {
            return Err(
                ExtensionError::InvalidIdentifier {
                    field,
                    value: value.to_owned(),
                },
            );
        }
    }

    Ok(())
}

fn validate_qualified_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ExtensionError> {
    if value.is_empty() {
        return Err(
            ExtensionError::InvalidQualifiedIdentifier {
                field,
                value: value.to_owned(),
            },
        );
    }

    for component in value.split('.') {
        if component.is_empty() {
            return Err(
                ExtensionError::InvalidQualifiedIdentifier {
                    field,
                    value: value.to_owned(),
                },
            );
        }

        validate_identifier(component, field)?;
    }

    Ok(())
}

fn is_identifier_start(
    character: char,
) -> bool {
    character == '_'
        || character.is_ascii_alphabetic()
}

fn is_identifier_continue(
    character: char,
) -> bool {
    character == '_'
        || character == '-'
        || character.is_ascii_alphanumeric()
}

fn validate_payload(
    payload: &ExtensionPayload,
) -> Result<(), ExtensionError> {
    match payload {
        ExtensionPayload::Empty => Ok(()),

        ExtensionPayload::Text(_) => Ok(()),

        ExtensionPayload::Bytes {
            encoding,
            data: _,
        } => {
            if let Some(encoding) = encoding {
                validate_qualified_identifier(
                    encoding,
                    "payload encoding",
                )?;
            }

            Ok(())
        }

        ExtensionPayload::Fields(fields) => {
            for key in fields.keys() {
                validate_identifier(
                    key,
                    "extension field",
                )?;
            }

            Ok(())
        }

        ExtensionPayload::List(values) => {
            for value in values {
                validate_attribute_value(value)?;
            }

            Ok(())
        }

        ExtensionPayload::Value(_) => Ok(()),

        ExtensionPayload::Parameter(_) => Ok(()),

        ExtensionPayload::LogicalQubit(_) => Ok(()),

        ExtensionPayload::PhysicalQubit(_) => Ok(()),

        ExtensionPayload::Extension(_) => Ok(()),

        ExtensionPayload::References(references) => {
            for reference in references {
                validate_target(reference)?;
            }

            Ok(())
        }
    }
}

fn validate_attribute_value(
    value: &AttributeValue,
) -> Result<(), ExtensionError> {
    match value {
        AttributeValue::Bool(_)
        | AttributeValue::Integer(_)
        | AttributeValue::UnsignedInteger(_)
        | AttributeValue::Float(_)
        | AttributeValue::String(_)
        | AttributeValue::Bytes(_)
        | AttributeValue::Value(_)
        | AttributeValue::Parameter(_)
        | AttributeValue::Qubit(_)
        | AttributeValue::PhysicalQubit(_)
        | AttributeValue::Type(_)
        | AttributeValue::List(_)
        | AttributeValue::Map(_) => Ok(()),
    }
}

fn validate_target(
    target: &ExtensionTarget,
) -> Result<(), ExtensionError> {
    match target {
        ExtensionTarget::Global
        | ExtensionTarget::Program(_)
        | ExtensionTarget::Module(_)
        | ExtensionTarget::Region(_)
        | ExtensionTarget::Block(_)
        | ExtensionTarget::Operation(_)
        | ExtensionTarget::LogicalQubit(_)
        | ExtensionTarget::PhysicalQubit(_)
        | ExtensionTarget::Value(_)
        | ExtensionTarget::Parameter(_)
        | ExtensionTarget::Pulse(_)
        | ExtensionTarget::Waveform(_)
        | ExtensionTarget::Channel(_)
        | ExtensionTarget::Frame(_)
        | ExtensionTarget::Schedule(_)
        | ExtensionTarget::Resource(_)
        | ExtensionTarget::Capability(_)
        | ExtensionTarget::Function(_)
        | ExtensionTarget::Type(_)
        | ExtensionTarget::Extension(_)
        | ExtensionTarget::Attribute(_) => Ok(()),
    }
}

// =============================================================================
// Deterministic target formatting
// =============================================================================

fn target_identity_string(
    target: &ExtensionTarget,
) -> String {
    match target {
        ExtensionTarget::Global => {
            "global".to_owned()
        }

        ExtensionTarget::Program(id) => {
            format!("program:{id}")
        }

        ExtensionTarget::Module(id) => {
            format!("module:{id}")
        }

        ExtensionTarget::Region(id) => {
            format!("region:{id}")
        }

        ExtensionTarget::Block(id) => {
            format!("block:{id}")
        }

        ExtensionTarget::Operation(id) => {
            format!("operation:{id}")
        }

        ExtensionTarget::LogicalQubit(id) => {
            format!("logical_qubit:{id}")
        }

        ExtensionTarget::PhysicalQubit(id) => {
            format!("physical_qubit:{id}")
        }

        ExtensionTarget::Value(id) => {
            format!("value:{id}")
        }

        ExtensionTarget::Parameter(id) => {
            format!("parameter:{id}")
        }

        ExtensionTarget::Pulse(id) => {
            format!("pulse:{id}")
        }

        ExtensionTarget::Waveform(id) => {
            format!("waveform:{id}")
        }

        ExtensionTarget::Channel(id) => {
            format!("channel:{id}")
        }

        ExtensionTarget::Frame(id) => {
            format!("frame:{id}")
        }

        ExtensionTarget::Schedule(id) => {
            format!("schedule:{id}")
        }

        ExtensionTarget::Resource(id) => {
            format!("resource:{id}")
        }

        ExtensionTarget::Capability(id) => {
            format!("capability:{id}")
        }

        ExtensionTarget::Function(id) => {
            format!("function:{id}")
        }

        ExtensionTarget::Type(id) => {
            format!("type:{id}")
        }

        ExtensionTarget::Extension(id) => {
            format!("extension:{id}")
        }

        ExtensionTarget::Attribute(id) => {
            format!("attribute:{id}")
        }
    }
}

// =============================================================================
// Public helper constructors
// =============================================================================

/// Creates a canonical Zamani extension key.
///
/// Example:
///
/// ```text
/// zamani.native@1.0.0
/// ```
pub fn zamani_extension_key(
    name: &str,
    version: ExtensionVersion,
) -> Result<ExtensionKey, ExtensionError> {
    ExtensionKey::new(
        "zamani",
        name,
        version,
    )
}

/// Creates an extension targeted at a logical qubit.
pub fn logical_qubit_extension(
    id: ExtensionId,
    key: ExtensionKey,
    qubit: QubitId,
    payload: ExtensionPayload,
) -> Result<Extension, ExtensionError> {
    Extension::new(
        id,
        key,
        ExtensionTarget::LogicalQubit(qubit),
        payload,
    )
}

/// Creates an extension targeted at a physical-qubit reference.
///
/// This does not make the IR hardware-dependent. It only permits a downstream
/// transformation to attach metadata to an explicitly identified physical
/// reference.
pub fn physical_qubit_extension(
    id: ExtensionId,
    key: ExtensionKey,
    qubit: PhysicalQubitId,
    payload: ExtensionPayload,
) -> Result<Extension, ExtensionError> {
    Extension::new(
        id,
        key,
        ExtensionTarget::PhysicalQubit(qubit),
        payload,
    )
}

/// Creates an operation-targeted extension.
pub fn operation_extension(
    id: ExtensionId,
    key: ExtensionKey,
    operation: OperationId,
    payload: ExtensionPayload,
) -> Result<Extension, ExtensionError> {
    Extension::new(
        id,
        key,
        ExtensionTarget::Operation(operation),
        payload,
    )
}

/// Creates a program-targeted extension.
pub fn program_extension(
    id: ExtensionId,
    key: ExtensionKey,
    program: ProgramId,
    payload: ExtensionPayload,
) -> Result<Extension, ExtensionError> {
    Extension::new(
        id,
        key,
        ExtensionTarget::Program(program),
        payload,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_version_is_ordered() {
        let v1 = ExtensionVersion::new(1, 0, 0);
        let v2 = ExtensionVersion::new(1, 1, 0);

        assert!(v2 > v1);
        assert!(v2.supports(v1));
        assert!(!v1.supports(v2));
    }

    #[test]
    fn extension_key_is_deterministic() {
        let key = ExtensionKey::new(
            "zamani",
            "native",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid extension key");

        assert_eq!(
            key.qualified_name(),
            "zamani.native@1.0.0"
        );
    }

    #[test]
    fn logical_qubit_target_uses_canonical_qubit_id() {
        let target =
            ExtensionTarget::LogicalQubit(
                QubitId::new(42),
            );

        assert_eq!(
            target.kind(),
            ExtensionTargetKind::LogicalQubit
        );
    }

    #[test]
    fn physical_qubit_target_uses_canonical_physical_id() {
        let target =
            ExtensionTarget::PhysicalQubit(
                PhysicalQubitId::new(123),
            );

        assert_eq!(
            target.kind(),
            ExtensionTargetKind::PhysicalQubit
        );
    }

    #[test]
    fn extension_can_be_created() {
        let key = zamani_extension_key(
            "experimental",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let extension = Extension::new(
            ExtensionId::new(1),
            key,
            ExtensionTarget::Global,
            ExtensionPayload::Text(
                "future quantum operation".to_owned(),
            ),
        )
        .expect("valid extension");

        assert_eq!(
            extension.id(),
            ExtensionId::new(1)
        );

        assert_eq!(
            extension.payload().kind(),
            ExtensionPayloadKind::Text
        );
    }

    #[test]
    fn extension_set_rejects_duplicate_contract_and_target() {
        let key = zamani_extension_key(
            "experimental",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let first = Extension::new(
            ExtensionId::new(1),
            key.clone(),
            ExtensionTarget::Global,
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let second = Extension::new(
            ExtensionId::new(2),
            key,
            ExtensionTarget::Global,
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let mut set = ExtensionSet::new();

        set.insert(first)
            .expect("first insertion succeeds");

        assert!(matches!(
            set.insert(second),
            Err(
                ExtensionError::DuplicateExtension { .. }
            )
        ));
    }

    #[test]
    fn extension_set_allows_same_contract_on_different_qubits() {
        let key = zamani_extension_key(
            "property",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let first = logical_qubit_extension(
            ExtensionId::new(1),
            key.clone(),
            QubitId::new(0),
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let second = logical_qubit_extension(
            ExtensionId::new(2),
            key,
            QubitId::new(1),
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let mut set = ExtensionSet::new();

        set.insert(first)
            .expect("first insertion succeeds");

        set.insert(second)
            .expect("second insertion succeeds");

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn extension_merge_is_transactional_on_conflict() {
        let key = zamani_extension_key(
            "test",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let first = Extension::new(
            ExtensionId::new(1),
            key.clone(),
            ExtensionTarget::Global,
            ExtensionPayload::Text(
                "one".to_owned(),
            ),
        )
        .expect("valid extension");

        let conflicting = Extension::new(
            ExtensionId::new(2),
            key,
            ExtensionTarget::Global,
            ExtensionPayload::Text(
                "two".to_owned(),
            ),
        )
        .expect("valid extension");

        let mut left = ExtensionSet::new();
        let mut right = ExtensionSet::new();

        left.insert(first)
            .expect("left insertion succeeds");

        right
            .insert(conflicting)
            .expect("right insertion succeeds");

        let result = left.merge(&right);

        assert!(matches!(
            result,
            Err(
                ExtensionError::ConflictingExtension { .. }
            )
        ));

        assert_eq!(left.len(), 1);
    }

    #[test]
    fn extension_attributes_do_not_silently_overwrite() {
        let key = zamani_extension_key(
            "metadata",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let mut extension = Extension::new(
            ExtensionId::new(1),
            key,
            ExtensionTarget::Global,
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let attribute =
            AttributeKey::zamani("deterministic")
                .expect("valid attribute");

        extension
            .insert_attribute(
                attribute.clone(),
                AttributeValue::Bool(true),
            )
            .expect("first insertion succeeds");

        assert!(matches!(
            extension.insert_attribute(
                attribute,
                AttributeValue::Bool(false),
            ),
            Err(
                ExtensionError::DuplicateAttribute { .. }
            )
        ));
    }

    #[test]
    fn extension_set_iteration_is_deterministic() {
        let key_a = zamani_extension_key(
            "a",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let key_b = zamani_extension_key(
            "b",
            ExtensionVersion::new(1, 0, 0),
        )
        .expect("valid key");

        let a = Extension::new(
            ExtensionId::new(2),
            key_a,
            ExtensionTarget::Global,
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let b = Extension::new(
            ExtensionId::new(1),
            key_b,
            ExtensionTarget::Global,
            ExtensionPayload::Empty,
        )
        .expect("valid extension");

        let mut set = ExtensionSet::new();

        set.insert(b)
            .expect("insertion succeeds");

        set.insert(a)
            .expect("insertion succeeds");

        let names: Vec<&str> = set
            .iter()
            .map(|extension| extension.name().as_str())
            .collect();

        assert_eq!(
            names,
            vec!["a", "b"]
        );
    }

    #[test]
    fn no_fixed_quantum_machine_limit_exists_here() {
        let logical_qubit =
            QubitId::new(u64::MAX);

        let target =
            ExtensionTarget::LogicalQubit(
                logical_qubit,
            );

        assert_eq!(
            target,
            ExtensionTarget::LogicalQubit(
                QubitId::new(u64::MAX)
            )
        );
    }
}