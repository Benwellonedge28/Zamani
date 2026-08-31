//! Zamani Quantum IR — Core Extension Contract
//!
//! This module defines the canonical extensibility boundary of the Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural purpose
//!
//! The canonical Zamani Quantum IR must be able to represent quantum
//! computation that the core vocabulary does not yet know about.
//!
//! Extensions therefore provide a stable mechanism for representing:
//!
//! - future quantum-computing models;
//! - new operation families;
//! - experimental semantics;
//! - vendor-specific declarations;
//! - research extensions;
//! - logical/fault-tolerant constructs;
//! - distributed-quantum constructs;
//! - new pulse/control abstractions;
//! - future IR entities;
//! - metadata required for lossless round-tripping.
//!
//! An extension is a semantic declaration. It is NOT executable compiler,
//! backend, hardware, simulator, optimizer, router, scheduler, or transport
//! logic.
//!
//! # Constitutional ownership rule
//!
//! ```text
//! canonical Zamani IR
//!        │
//!        ├── core::extension
//!        │       │
//!        │       └── extensibility contract
//!        │
//!        ▼
//! downstream compiler stages
//!        ├── optimization
//!        ├── routing
//!        ├── scheduling
//!        ├── hardware compatibility
//!        ├── lowering
//!        └── backend execution
//! ```
//!
//! `core::extension` never imports or depends upon those downstream systems.
//!
//! # Why this is necessary
//!
//! A finite enum such as:
//!
//! ```text
//! X
//! Y
//! Z
//! H
//! CX
//! CZ
//! ...
//! ```
//!
//! cannot be the complete semantic universe of Zamani Quantum IR.
//!
//! New technologies may introduce operations and resources that do not fit
//! the original gate vocabulary. The extension mechanism prevents every such
//! addition from becoming a breaking modification to the canonical IR.
//!
//! # Namespace model
//!
//! Every extension contract has:
//!
//! ```text
//! namespace + name + version
//! ```
//!
//! Example:
//!
//! ```text
//! zamani.logical@1.0.0
//! zamani.pulse@1.0.0
//! research.analog@1.2.0
//! hardware.example.native_operation@1.0.0
//! user.custom@1.0.0
//! ```
//!
//! Namespace ownership is declarative. This module does not decide whether a
//! namespace is trustworthy or executable.
//!
//! # Version model
//!
//! Extension versions use:
//!
//! ```text
//! major.minor.patch
//! ```
//!
//! Compatibility is deliberately conservative.
//!
//! A consumer may explicitly accept an older compatible extension contract,
//! but must never silently interpret a future major/minor contract.
//!
//! # Unknown extensions
//!
//! Unknown extensions MUST remain representable.
//!
//! A consumer that does not understand an extension can:
//!
//! 1. preserve it;
//! 2. round-trip it;
//! 3. hash it;
//! 4. inspect its identity;
//! 5. reject it explicitly when execution requires semantics.
//!
//! It must never silently discard it.
//!
//! # Lossless representation
//!
//! Extension payloads support:
//!
//! - null;
//! - booleans;
//! - signed integers;
//! - unsigned integers;
//! - UTF-8 strings;
//! - raw bytes;
//! - arrays;
//! - deterministic maps;
//! - references to canonical IR identities;
//! - opaque extension-defined values.
//!
//! This permits future extensions without forcing their schemas into the
//! canonical core.
//!
//! # Determinism
//!
//! Determinism is required for:
//!
//! - canonical serialization;
//! - cryptographic hashing;
//! - caching;
//! - reproducible compilation;
//! - distributed compilation;
//! - provenance;
//! - equality;
//! - testing.
//!
//! Accordingly:
//!
//! - maps use `BTreeMap`;
//! - keys are ordered;
//! - extension identities are ordered;
//! - targets are ordered;
//! - payloads have structural equality;
//! - hashing is structural;
//! - textual formatting is deterministic.
//!
//! # Scaling
//!
//! There is intentionally no architectural maximum number of:
//!
//! - extensions;
//! - extension definitions;
//! - targets;
//! - payload fields;
//! - nested payload values;
//! - namespaces;
//! - extension names;
//! - qubits;
//! - operations.
//!
//! No constants such as:
//!
//! ```text
//! 63
//! 64
//! 128
//! 4096
//! 1_000_000
//! ```
//!
//! are used as quantum-resource limits.
//!
//! Concrete resource/security limits belong to `QuantumIrLimits` and to the
//! consuming subsystem.
//!
//! Rust container capacity and available address space are implementation
//! constraints, not semantic IR limits.
//!
//! # Quantum identity boundary
//!
//! Logical and physical quantum identities are imported directly from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! This module MUST NOT define a second `QubitId` or `PhysicalQubitId`.
//!
//! # Identity boundary
//!
//! Stable IR object identities are imported from:
//!
//! ```text
//! quantum::ir::core::identity
//! ```
//!
//! `ExtensionId` identifies an extension occurrence.
//!
//! `ExtensionKey` identifies an extension contract.
//!
//! These concepts are intentionally separate.
//!
//! # Integration contract
//!
//! This module is foundational and may be consumed by:
//!
//! - program;
//! - module;
//! - region;
//! - block;
//! - operation;
//! - gate;
//! - measurement;
//! - pulse;
//! - waveform;
//! - channel;
//! - frame;
//! - timing;
//! - resources;
//! - validation;
//! - serialization;
//! - hashing;
//! - provenance;
//! - dialects;
//! - compatibility.
//!
//! This module MUST NOT depend on those higher-level modules.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! Requirements:
//!
//! - no unsafe;
//! - no nightly features;
//! - no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # File completion contract
//!
//! Once this file is implemented, later changes to:
//!
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - backend;
//! - simulation;
//! - frontend;
//! - QEC;
//! - pulse compilation
//!
//! MUST NOT require modification of this file merely because those systems
//! evolve.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

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
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Extension version
// =============================================================================

/// Version of an extension semantic contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - Quantum IR version;
/// - compiler version;
/// - hardware version;
/// - backend version;
/// - calibration version.
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
pub struct ExtensionVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl ExtensionVersion {
    /// Creates an extension version.
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

    /// Returns whether two versions have the same major contract.
    #[must_use]
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether two versions are exactly equal.
    #[must_use]
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether this version can consume the other version.
    ///
    /// Compatibility is intentionally conservative:
    ///
    /// - same major;
    /// - producer minor <= consumer minor;
    /// - if minors match, producer patch <= consumer patch.
    #[must_use]
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && (
                other.minor < self.minor
                    || other.patch <= self.patch
            )
    }

    /// Returns whether a major-version migration is required.
    #[must_use]
    pub const fn requires_major_migration(
        self,
        other: Self,
    ) -> bool {
        self.major != other.major
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

/// Validated dot-separated extension namespace.
///
/// Examples:
///
/// ```text
/// zamani
/// compiler
/// research.analog
/// hardware.vendor
/// user.custom
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
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

    /// Returns the namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace.
    #[must_use]
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

impl AsRef<str> for ExtensionNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Extension name
// =============================================================================

/// Validated local extension name.
///
/// Examples:
///
/// ```text
/// logical
/// pulse
/// native_operation
/// calibration
/// experimental
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
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
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name.
    #[must_use]
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

impl AsRef<str> for ExtensionName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Extension key
// =============================================================================

/// Canonical identity of an extension contract.
///
/// This excludes `ExtensionId`.
///
/// `ExtensionKey` identifies:
///
/// ```text
/// namespace + name + version
/// ```
///
/// `ExtensionId` identifies one occurrence of that contract.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct ExtensionKey {
    namespace: ExtensionNamespace,
    name: ExtensionName,
    version: ExtensionVersion,
}

impl ExtensionKey {
    /// Creates a validated extension contract key.
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
    #[must_use]
    pub fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    /// Returns the local name.
    #[must_use]
    pub fn name(&self) -> &ExtensionName {
        &self.name
    }

    /// Returns the extension version.
    #[must_use]
    pub const fn version(&self) -> ExtensionVersion {
        self.version
    }

    /// Returns the canonical qualified name.
    ///
    /// Format:
    ///
    /// ```text
    /// namespace.name@major.minor.patch
    /// ```
    #[must_use]
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

/// Canonical IR entity to which an extension is attached.
///
/// Quantum identity is deliberately imported from
/// `quantum::ir::qubit`.
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
pub enum ExtensionTarget {
    /// Global IR/document-level extension.
    Global,

    /// Complete program.
    Program(ProgramId),

    /// Module.
    Module(ModuleId),

    /// Region.
    Region(RegionId),

    /// Block.
    Block(BlockId),

    /// Operation.
    Operation(OperationId),

    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical-qubit reference.
    PhysicalQubit(PhysicalQubitId),

    /// SSA/runtime IR value.
    Value(ValueId),

    /// Symbolic/runtime parameter.
    Parameter(ParameterId),

    /// Pulse-level semantic object.
    Pulse(PulseId),

    /// Waveform definition.
    Waveform(WaveformId),

    /// Abstract control/acquisition channel.
    Channel(ChannelId),

    /// Abstract control frame.
    Frame(FrameId),

    /// Schedule object.
    Schedule(ScheduleId),

    /// Abstract resource.
    Resource(ResourceId),

    /// Capability declaration/reference.
    Capability(CapabilityId),

    /// Function/subroutine.
    Function(FunctionId),

    /// Declared IR type.
    Type(TypeId),

    /// Attribute occurrence.
    Attribute(AttributeId),

    /// Extension attached to another extension.
    Extension(ExtensionId),
}

impl ExtensionTarget {
    /// Returns whether this target is global.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns whether this target refers to a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this target refers to a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns the logical qubit when this target is one.
    #[must_use]
    pub const fn logical_qubit(
        self,
    ) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the physical qubit when this target is one.
    #[must_use]
    pub const fn physical_qubit(
        self,
    ) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(id),
            _ => None,
        }
    }
}

impl fmt::Display for ExtensionTarget {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Global => formatter.write_str("global"),

            Self::Program(id) => {
                write!(formatter, "program:{id}")
            }

            Self::Module(id) => {
                write!(formatter, "module:{id}")
            }

            Self::Region(id) => {
                write!(formatter, "region:{id}")
            }

            Self::Block(id) => {
                write!(formatter, "block:{id}")
            }

            Self::Operation(id) => {
                write!(formatter, "operation:{id}")
            }

            Self::LogicalQubit(id) => {
                write!(formatter, "logical-qubit:{id}")
            }

            Self::PhysicalQubit(id) => {
                write!(formatter, "physical-qubit:{id}")
            }

            Self::Value(id) => {
                write!(formatter, "value:{id}")
            }

            Self::Parameter(id) => {
                write!(formatter, "parameter:{id}")
            }

            Self::Pulse(id) => {
                write!(formatter, "pulse:{id}")
            }

            Self::Waveform(id) => {
                write!(formatter, "waveform:{id}")
            }

            Self::Channel(id) => {
                write!(formatter, "channel:{id}")
            }

            Self::Frame(id) => {
                write!(formatter, "frame:{id}")
            }

            Self::Schedule(id) => {
                write!(formatter, "schedule:{id}")
            }

            Self::Resource(id) => {
                write!(formatter, "resource:{id}")
            }

            Self::Capability(id) => {
                write!(formatter, "capability:{id}")
            }

            Self::Function(id) => {
                write!(formatter, "function:{id}")
            }

            Self::Type(id) => {
                write!(formatter, "type:{id}")
            }

            Self::Attribute(id) => {
                write!(formatter, "attribute:{id}")
            }

            Self::Extension(id) => {
                write!(formatter, "extension:{id}")
            }
        }
    }
}

// =============================================================================
// Extension value
// =============================================================================

/// Lossless, deterministic value representation for extension payloads.
///
/// This is intentionally separate from the canonical runtime/value system.
/// It exists so an unknown extension can be preserved without requiring the
/// core IR to understand its schema.
///
/// Maps are ordered by `BTreeMap`, guaranteeing deterministic traversal.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum ExtensionValue {
    /// Explicit absence/null value.
    Null,

    /// Boolean.
    Boolean(bool),

    /// Signed integer.
    Signed(i64),

    /// Unsigned integer.
    Unsigned(u64),

    /// UTF-8 string.
    String(String),

    /// Raw bytes belonging to an extension-defined encoding.
    Bytes(Vec<u8>),

    /// Ordered sequence of extension values.
    Array(Vec<ExtensionValue>),

    /// Deterministically ordered string-keyed object.
    Map(BTreeMap<String, ExtensionValue>),

    /// Reference to a canonical IR value.
    Value(ValueId),

    /// Reference to a canonical IR parameter.
    Parameter(ParameterId),

    /// Reference to a logical qubit.
    LogicalQubit(QubitId),

    /// Reference to a physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Reference to an operation.
    Operation(OperationId),

    /// Reference to another extension.
    Extension(ExtensionId),

    /// Extension-defined opaque payload.
    ///
    /// The tag identifies the extension-defined encoding/schema.
    Opaque {
        /// Extension-defined media/encoding tag.
        tag: String,

        /// Raw payload.
        data: Vec<u8>,
    },
}

impl ExtensionValue {
    /// Creates an empty deterministic map.
    #[must_use]
    pub fn empty_map() -> Self {
        Self::Map(BTreeMap::new())
    }

    /// Creates a deterministic map from an iterator.
    ///
    /// Duplicate keys are rejected instead of silently overwritten.
    pub fn map<I, K>(
        entries: I,
    ) -> Result<Self, ExtensionError>
    where
        I: IntoIterator<Item = (K, ExtensionValue)>,
        K: Into<String>,
    {
        let mut map = BTreeMap::new();

        for (key, value) in entries {
            let key = key.into();

            validate_map_key(&key)?;

            if map.insert(key.clone(), value).is_some() {
                return Err(
                    ExtensionError::DuplicatePayloadKey {
                        key,
                    },
                );
            }
        }

        Ok(Self::Map(map))
    }

    /// Returns the map when this is a map value.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> Option<&BTreeMap<String, ExtensionValue>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    /// Returns the string when this is a string value.
    #[must_use]
    pub fn as_str(
        &self,
    ) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns whether the value is opaque.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque { .. })
    }

    /// Returns whether the value is a map.
    #[must_use]
    pub const fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    /// Returns whether the value is an array.
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns the recursively calculated structural depth.
    ///
    /// This method does not impose a semantic maximum. Resource limits are
    /// enforced by the validation/limits layer.
    #[must_use]
    pub fn structural_depth(&self) -> usize {
        match self {
            Self::Array(values) => {
                values
                    .iter()
                    .map(Self::structural_depth)
                    .max()
                    .unwrap_or(0)
                    .saturating_add(1)
            }

            Self::Map(values) => {
                values
                    .values()
                    .map(Self::structural_depth)
                    .max()
                    .unwrap_or(0)
                    .saturating_add(1)
            }

            Self::Opaque { .. }
            | Self::Null
            | Self::Boolean(_)
            | Self::Signed(_)
            | Self::Unsigned(_)
            | Self::String(_)
            | Self::Bytes(_)
            | Self::Value(_)
            | Self::Parameter(_)
            | Self::LogicalQubit(_)
            | Self::PhysicalQubit(_)
            | Self::Operation(_)
            | Self::Extension(_) => 0,
        }
    }

    /// Validates this payload structurally.
    ///
    /// This does not interpret extension semantics.
    pub fn validate(
        &self,
    ) -> Result<(), ExtensionError> {
        match self {
            Self::Map(map) => {
                for key in map.keys() {
                    validate_map_key(key)?;
                }

                for value in map.values() {
                    value.validate()?;
                }
            }

            Self::Array(values) => {
                for value in values {
                    value.validate()?;
                }
            }

            Self::Opaque { tag, .. } => {
                validate_identifier(
                    tag,
                    "opaque payload tag",
                )?;
            }

            Self::Null
            | Self::Boolean(_)
            | Self::Signed(_)
            | Self::Unsigned(_)
            | Self::String(_)
            | Self::Bytes(_)
            | Self::Value(_)
            | Self::Parameter(_)
            | Self::LogicalQubit(_)
            | Self::PhysicalQubit(_)
            | Self::Operation(_)
            | Self::Extension(_) => {}
        }

        Ok(())
    }
}

impl From<bool> for ExtensionValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for ExtensionValue {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<u64> for ExtensionValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<String> for ExtensionValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ExtensionValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Vec<u8>> for ExtensionValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<ValueId> for ExtensionValue {
    fn from(value: ValueId) -> Self {
        Self::Value(value)
    }
}

impl From<ParameterId> for ExtensionValue {
    fn from(value: ParameterId) -> Self {
        Self::Parameter(value)
    }
}

impl From<QubitId> for ExtensionValue {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for ExtensionValue {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl From<OperationId> for ExtensionValue {
    fn from(value: OperationId) -> Self {
        Self::Operation(value)
    }
}

impl From<ExtensionId> for ExtensionValue {
    fn from(value: ExtensionId) -> Self {
        Self::Extension(value)
    }
}

// =============================================================================
// Extension contract
// =============================================================================

/// A complete extension occurrence.
///
/// An extension consists of:
///
/// ```text
/// identity
/// contract key
/// target
/// payload
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Extension {
    id: ExtensionId,
    key: ExtensionKey,
    target: ExtensionTarget,
    payload: ExtensionValue,
}

impl Extension {
    /// Creates an extension occurrence.
    pub fn new(
        id: ExtensionId,
        key: ExtensionKey,
        target: ExtensionTarget,
        payload: ExtensionValue,
    ) -> Result<Self, ExtensionError> {
        payload.validate()?;

        Ok(Self {
            id,
            key,
            target,
            payload,
        })
    }

    /// Returns the extension occurrence identity.
    #[must_use]
    pub const fn id(&self) -> ExtensionId {
        self.id
    }

    /// Returns the extension contract key.
    #[must_use]
    pub fn key(&self) -> &ExtensionKey {
        &self.key
    }

    /// Returns the target.
    #[must_use]
    pub const fn target(&self) -> ExtensionTarget {
        self.target
    }

    /// Returns the extension payload.
    #[must_use]
    pub fn payload(&self) -> &ExtensionValue {
        &self.payload
    }

    /// Returns a mutable payload reference.
    ///
    /// Callers must revalidate the extension after mutation.
    #[must_use]
    pub fn payload_mut(&mut self) -> &mut ExtensionValue {
        &mut self.payload
    }

    /// Replaces the payload while preserving the extension identity and
    /// contract.
    pub fn replace_payload(
        &mut self,
        payload: ExtensionValue,
    ) -> Result<(), ExtensionError> {
        payload.validate()?;
        self.payload = payload;
        Ok(())
    }

    /// Validates the complete extension.
    pub fn validate(
        &self,
    ) -> Result<(), ExtensionError> {
        validate_extension_id(self.id)?;

        validate_extension_key(&self.key)?;

        self.payload.validate()?;

        Ok(())
    }
}

// =============================================================================
// Extension contract identity
// =============================================================================

/// Collision key for extension-set storage.
///
/// The target is part of the key because the same extension contract may
/// legitimately occur on multiple IR entities.
///
/// Example:
///
/// ```text
/// zamani.logical@1.0.0 -> q0
/// zamani.logical@1.0.0 -> q1
/// ```
///
/// are distinct extension occurrences.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
struct ExtensionInstanceKey {
    target: ExtensionTarget,
    key: ExtensionKey,
}

impl ExtensionInstanceKey {
    fn new(
        target: ExtensionTarget,
        key: ExtensionKey,
    ) -> Self {
        Self {
            target,
            key,
        }
    }
}

// =============================================================================
// Extension set
// =============================================================================

/// Deterministic collection of extensions.
///
/// The collection prevents two different extension payloads from silently
/// occupying the same `(target, namespace, name, version)` slot.
///
/// The extension occurrence ID remains independently available.
///
/// This structure is intentionally based on ordered maps rather than hash
/// maps, making traversal deterministic across compiler processes.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ExtensionSet {
    by_instance: BTreeMap<ExtensionInstanceKey, Extension>,
}

impl ExtensionSet {
    /// Creates an empty extension set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            by_instance: BTreeMap::new(),
        }
    }

    /// Returns the number of extension occurrences.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_instance.len()
    }

    /// Returns whether the set contains no extensions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_instance.is_empty()
    }

    /// Inserts an extension.
    ///
    /// The insertion is rejected when another extension already occupies the
    /// same target and contract key.
    ///
    /// No existing extension is silently replaced.
    pub fn insert(
        &mut self,
        extension: Extension,
    ) -> Result<(), ExtensionError> {
        extension.validate()?;

        let instance_key =
            ExtensionInstanceKey::new(
                extension.target(),
                extension.key().clone(),
            );

        if self
            .by_instance
            .contains_key(&instance_key)
        {
            return Err(
                ExtensionError::ConflictingExtension {
                    target: extension.target(),
                    key: extension.key().clone(),
                },
            );
        }

        self.by_instance
            .insert(instance_key, extension);

        Ok(())
    }

    /// Returns an extension by target and contract key.
    #[must_use]
    pub fn get(
        &self,
        target: ExtensionTarget,
        key: &ExtensionKey,
    ) -> Option<&Extension> {
        let instance_key =
            ExtensionInstanceKey::new(
                target,
                key.clone(),
            );

        self.by_instance.get(&instance_key)
    }

    /// Returns an extension by its occurrence identity.
    #[must_use]
    pub fn get_by_id(
        &self,
        id: ExtensionId,
    ) -> Option<&Extension> {
        self.by_instance
            .values()
            .find(|extension| extension.id() == id)
    }

    /// Returns whether an extension occurrence exists.
    #[must_use]
    pub fn contains_id(
        &self,
        id: ExtensionId,
    ) -> bool {
        self.get_by_id(id).is_some()
    }

    /// Removes an extension by target and contract key.
    pub fn remove(
        &mut self,
        target: ExtensionTarget,
        key: &ExtensionKey,
    ) -> Option<Extension> {
        let instance_key =
            ExtensionInstanceKey::new(
                target,
                key.clone(),
            );

        self.by_instance.remove(&instance_key)
    }

    /// Returns deterministic extension iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Extension> {
        self.by_instance.values()
    }

    /// Returns all extensions targeting one IR entity.
    pub fn for_target(
        &self,
        target: ExtensionTarget,
    ) -> impl Iterator<Item = &Extension> {
        self.by_instance
            .iter()
            .filter(move |(instance, _)| {
                instance.target == target
            })
            .map(|(_, extension)| extension)
    }

    /// Merges another extension set.
    ///
    /// Existing identical extensions are accepted.
    ///
    /// Conflicting payloads are rejected.
    ///
    /// Nothing is silently overwritten.
    pub fn merge(
        &mut self,
        other: &ExtensionSet,
    ) -> Result<(), ExtensionError> {
        for extension in other.iter() {
            let instance_key =
                ExtensionInstanceKey::new(
                    extension.target(),
                    extension.key().clone(),
                );

            match self.by_instance.get(&instance_key) {
                None => {
                    self.by_instance.insert(
                        instance_key,
                        extension.clone(),
                    );
                }

                Some(existing)
                    if existing == extension =>
                {
                    // Already present with identical
                    // semantics. Nothing to do.
                }

                Some(_) => {
                    return Err(
                        ExtensionError::ConflictingExtension {
                            target: extension.target(),
                            key: extension.key().clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates every extension in the set.
    pub fn validate(
        &self,
    ) -> Result<(), ExtensionError> {
        for extension in self.iter() {
            extension.validate()?;
        }

        Ok(())
    }
}

impl Default for ExtensionSet {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Extension registry descriptor
// =============================================================================

/// Declarative descriptor for a known extension contract.
///
/// This does not contain executable code.
///
/// It allows a validator, serializer, dialect manager, or compatibility layer
/// to record what contract is known without making the extension mechanism
/// itself depend on those systems.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct ExtensionDescriptor {
    key: ExtensionKey,
    description: Option<String>,
}

impl ExtensionDescriptor {
    /// Creates a descriptor without a human-readable description.
    #[must_use]
    pub fn new(
        key: ExtensionKey,
    ) -> Self {
        Self {
            key,
            description: None,
        }
    }

    /// Creates a descriptor with a human-readable description.
    #[must_use]
    pub fn with_description<S>(
        key: ExtensionKey,
        description: S,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            key,
            description: Some(description.into()),
        }
    }

    /// Returns the extension contract key.
    #[must_use]
    pub fn key(&self) -> &ExtensionKey {
        &self.key
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// =============================================================================
// Extension registry
// =============================================================================

/// Deterministic registry of extension contract descriptors.
///
/// This registry describes known contracts. It does not execute them.
///
/// Unknown extensions do not need to be registered in order to be preserved.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ExtensionRegistry {
    descriptors: BTreeMap<ExtensionKey, ExtensionDescriptor>,
}

impl ExtensionRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            descriptors: BTreeMap::new(),
        }
    }

    /// Registers a descriptor.
    ///
    /// Re-registering an identical descriptor is accepted.
    ///
    /// Registering the same key with a different descriptor is rejected.
    pub fn register(
        &mut self,
        descriptor: ExtensionDescriptor,
    ) -> Result<(), ExtensionError> {
        let key = descriptor.key().clone();

        match self.descriptors.get(&key) {
            None => {
                self.descriptors.insert(
                    key,
                    descriptor,
                );
                Ok(())
            }

            Some(existing)
                if existing == &descriptor =>
            {
                Ok(())
            }

            Some(_) => Err(
                ExtensionError::ConflictingDescriptor {
                    key,
                },
            ),
        }
    }

    /// Returns a descriptor by contract key.
    #[must_use]
    pub fn get(
        &self,
        key: &ExtensionKey,
    ) -> Option<&ExtensionDescriptor> {
        self.descriptors.get(key)
    }

    /// Returns whether the contract is known.
    #[must_use]
    pub fn contains(
        &self,
        key: &ExtensionKey,
    ) -> bool {
        self.descriptors.contains_key(key)
    }

    /// Returns whether the supplied version can be consumed by the registered
    /// descriptor's contract.
    #[must_use]
    pub fn supports(
        &self,
        key: &ExtensionKey,
    ) -> bool {
        self.descriptors
            .keys()
            .any(|registered| {
                registered.namespace()
                    == key.namespace()
                    && registered.name()
                        == key.name()
                    && registered.version()
                        .supports(key.version())
            })
    }

    /// Returns the number of registered contracts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Returns whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    /// Returns deterministic registry iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ExtensionDescriptor> {
        self.descriptors.values()
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Structural validation
// =============================================================================

/// Validates an extension key.
pub fn validate_extension_key(
    key: &ExtensionKey,
) -> Result<(), ExtensionError> {
    validate_qualified_identifier(
        key.namespace().as_str(),
        "extension namespace",
    )?;

    validate_identifier(
        key.name().as_str(),
        "extension name",
    )?;

    Ok(())
}

/// Validates an extension occurrence identity.
///
/// Identity `0` is permitted because this module does not impose allocation
/// policy. The owning identity allocator decides which values are valid.
pub fn validate_extension_id(
    _id: ExtensionId,
) -> Result<(), ExtensionError> {
    Ok(())
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates a single ASCII identifier.
///
/// The first character must be:
///
/// - `A-Z`;
/// - `a-z`;
/// - `_`.
///
/// Subsequent characters may additionally contain digits.
///
/// This is intentionally an ASCII structural grammar. Unicode extension
/// namespaces can be introduced later by a versioned extension contract
/// without changing the core semantics of extension storage.
fn validate_identifier(
    value: &str,
    what: &'static str,
) -> Result<(), ExtensionError> {
    if value.is_empty() {
        return Err(
            ExtensionError::EmptyIdentifier {
                what,
            },
        );
    }

    let mut characters = value.bytes();

    let first = match characters.next() {
        Some(value) => value,
        None => {
            return Err(
                ExtensionError::EmptyIdentifier {
                    what,
                },
            );
        }
    };

    if !is_identifier_start(first) {
        return Err(
            ExtensionError::InvalidIdentifier {
                what,
                value: value.to_owned(),
            },
        );
    }

    for byte in characters {
        if !is_identifier_continue(byte) {
            return Err(
                ExtensionError::InvalidIdentifier {
                    what,
                    value: value.to_owned(),
                },
            );
        }
    }

    Ok(())
}

/// Validates a dot-separated qualified identifier.
fn validate_qualified_identifier(
    value: &str,
    what: &'static str,
) -> Result<(), ExtensionError> {
    if value.is_empty() {
        return Err(
            ExtensionError::EmptyIdentifier {
                what,
            },
        );
    }

    for segment in value.split('.') {
        validate_identifier(
            segment,
            what,
        )?;
    }

    Ok(())
}

/// Validates a deterministic extension payload map key.
///
/// Map keys intentionally use the same identifier grammar as extension names.
fn validate_map_key(
    key: &str,
) -> Result<(), ExtensionError> {
    validate_identifier(
        key,
        "extension payload map key",
    )
}

#[inline]
fn is_identifier_start(
    byte: u8,
) -> bool {
    matches!(
        byte,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'_'
    )
}

#[inline]
fn is_identifier_continue(
    byte: u8,
) -> bool {
    matches!(
        byte,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'_'
            | b'-'
    )
}

// =============================================================================
// Canonical structural fingerprint
// =============================================================================

/// Deterministic non-cryptographic structural hash for an extension.
///
/// This function is intended for:
///
/// - in-memory indexing;
/// - deterministic tests;
/// - change detection;
/// - hash-map/set support.
///
/// It is NOT the canonical cryptographic IR hash.
///
/// Canonical cryptographic hashing belongs to `quantum::ir::hash`.
#[must_use]
pub fn structural_hash(
    extension: &Extension,
) -> u64 {
    let mut hasher =
        std::collections::hash_map::DefaultHasher::new();

    extension.hash(&mut hasher);

    hasher.finish()
}

/// Deterministic structural hash for an extension set.
///
/// Iteration order is deterministic because the underlying collection is a
/// `BTreeMap`.
#[must_use]
pub fn structural_hash_set(
    extensions: &ExtensionSet,
) -> u64 {
    let mut hasher =
        std::collections::hash_map::DefaultHasher::new();

    for extension in extensions.iter() {
        extension.hash(&mut hasher);
    }

    hasher.finish()
}

// =============================================================================
// Errors
// =============================================================================

/// Structural and compatibility errors produced by the extension subsystem.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum ExtensionError {
    /// An identifier was empty.
    EmptyIdentifier {
        /// Semantic category being validated.
        what: &'static str,
    },

    /// An identifier violated the structural grammar.
    InvalidIdentifier {
        /// Semantic category being validated.
        what: &'static str,

        /// Invalid identifier.
        value: String,
    },

    /// A payload map contained the same key more than once during construction.
    DuplicatePayloadKey {
        /// Conflicting key.
        key: String,
    },

    /// Two extensions attempted to occupy the same target/contract slot with
    /// different semantics.
    ConflictingExtension {
        /// Target of the conflicting extension.
        target: ExtensionTarget,

        /// Extension contract.
        key: ExtensionKey,
    },

    /// Two descriptors attempted to define the same extension contract
    /// differently.
    ConflictingDescriptor {
        /// Extension contract.
        key: ExtensionKey,
    },
}

impl fmt::Display for ExtensionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { what } => {
                write!(
                    formatter,
                    "{what} must not be empty"
                )
            }

            Self::InvalidIdentifier {
                what,
                value,
            } => {
                write!(
                    formatter,
                    "invalid {what} `{value}`"
                )
            }

            Self::DuplicatePayloadKey { key } => {
                write!(
                    formatter,
                    "duplicate extension payload key `{key}`"
                )
            }

            Self::ConflictingExtension {
                target,
                key,
            } => {
                write!(
                    formatter,
                    "conflicting extension `{key}` on target `{target}`"
                )
            }

            Self::ConflictingDescriptor { key } => {
                write!(
                    formatter,
                    "conflicting descriptor for extension `{key}`"
                )
            }
        }
    }
}

impl std::error::Error for ExtensionError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(
        namespace: &str,
        name: &str,
    ) -> ExtensionKey {
        ExtensionKey::new(
            namespace,
            name,
            ExtensionVersion::new(
                1,
                0,
                0,
            ),
        )
        .expect("valid extension key")
    }

    #[test]
    fn version_support_is_conservative() {
        let consumer =
            ExtensionVersion::new(
                1,
                2,
                3,
            );

        assert!(
            consumer.supports(
                ExtensionVersion::new(
                    1,
                    0,
                    0,
                )
            )
        );

        assert!(
            consumer.supports(
                ExtensionVersion::new(
                    1,
                    2,
                    3,
                )
            )
        );

        assert!(
            !consumer.supports(
                ExtensionVersion::new(
                    1,
                    2,
                    4,
                )
            )
        );

        assert!(
            !consumer.supports(
                ExtensionVersion::new(
                    1,
                    3,
                    0,
                )
            )
        );

        assert!(
            !consumer.supports(
                ExtensionVersion::new(
                    2,
                    0,
                    0,
                )
            )
        );
    }

    #[test]
    fn namespace_validation_is_structural() {
        assert!(
            ExtensionNamespace::new(
                "hardware.vendor"
            )
            .is_ok()
        );

        assert!(
            ExtensionNamespace::new(
                "research.analog"
            )
            .is_ok()
        );

        assert!(
            ExtensionNamespace::new(
                ""
            )
            .is_err()
        );

        assert!(
            ExtensionNamespace::new(
                ".hardware"
            )
            .is_err()
        );

        assert!(
            ExtensionNamespace::new(
                "hardware."
            )
            .is_err()
        );
    }

    #[test]
    fn extension_name_validation_is_structural() {
        assert!(
            ExtensionName::new(
                "native_operation"
            )
            .is_ok()
        );

        assert!(
            ExtensionName::new(
                "123native"
            )
            .is_err()
        );

        assert!(
            ExtensionName::new(
                ""
            )
            .is_err()
        );
    }

    #[test]
    fn extension_key_is_deterministic() {
        let key =
            key(
                "zamani",
                "logical",
            );

        assert_eq!(
            key.to_string(),
            "zamani.logical@1.0.0"
        );

        assert_eq!(
            key.qualified_name(),
            "zamani.logical@1.0.0"
        );
    }

    #[test]
    fn payload_maps_are_deterministic() {
        let payload =
            ExtensionValue::map([
                (
                    "z",
                    ExtensionValue::Unsigned(2),
                ),
                (
                    "a",
                    ExtensionValue::Unsigned(1),
                ),
            ])
            .expect("valid map");

        let map =
            payload
                .as_map()
                .expect("map");

        let keys: Vec<&str> =
            map.keys()
                .map(String::as_str)
                .collect();

        assert_eq!(
            keys,
            vec!["a", "z"]
        );
    }

    #[test]
    fn payload_map_rejects_duplicate_keys() {
        let result =
            ExtensionValue::map(vec![
                (
                    "x",
                    ExtensionValue::Unsigned(1),
                ),
                (
                    "x",
                    ExtensionValue::Unsigned(2),
                ),
            ]);

        assert!(
            matches!(
                result,
                Err(
                    ExtensionError::DuplicatePayloadKey {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn payload_validation_recurses() {
        let payload =
            ExtensionValue::Map(
                BTreeMap::from([
                    (
                        "outer",
                        ExtensionValue::Array(
                            vec![
                                ExtensionValue::Map(
                                    BTreeMap::from([
                                        (
                                            "inner",
                                            ExtensionValue::Boolean(
                                                true,
                                            ),
                                        ),
                                    ]),
                                ),
                            ],
                        ),
                    ),
                ]),
            );

        assert!(
            payload.validate().is_ok()
        );
    }

    #[test]
    fn extension_can_target_logical_qubit() {
        let extension =
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::LogicalQubit(
                    QubitId::new(0),
                ),
                ExtensionValue::Boolean(
                    true,
                ),
            )
            .expect("valid extension");

        assert_eq!(
            extension
                .target()
                .logical_qubit(),
            Some(QubitId::new(0))
        );
    }

    #[test]
    fn extension_can_target_physical_qubit() {
        let extension =
            Extension::new(
                ExtensionId::new(2),
                key(
                    "hardware",
                    "physical",
                ),
                ExtensionTarget::PhysicalQubit(
                    PhysicalQubitId::new(7),
                ),
                ExtensionValue::String(
                    "example".to_owned(),
                ),
            )
            .expect("valid extension");

        assert_eq!(
            extension
                .target()
                .physical_qubit(),
            Some(
                PhysicalQubitId::new(7)
            )
        );
    }

    #[test]
    fn extension_set_rejects_conflicts() {
        let mut set =
            ExtensionSet::new();

        let first =
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::LogicalQubit(
                    QubitId::new(0),
                ),
                ExtensionValue::Boolean(
                    true,
                ),
            )
            .expect("valid extension");

        let second =
            Extension::new(
                ExtensionId::new(2),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::LogicalQubit(
                    QubitId::new(0),
                ),
                ExtensionValue::Boolean(
                    false,
                ),
            )
            .expect("valid extension");

        assert!(
            set.insert(first).is_ok()
        );

        assert!(
            matches!(
                set.insert(second),
                Err(
                    ExtensionError::ConflictingExtension {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn_same_contract_can_target_multiple_entities() {
        let mut set =
            ExtensionSet::new();

        let first =
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::LogicalQubit(
                    QubitId::new(0),
                ),
                ExtensionValue::Boolean(
                    true,
                ),
            )
            .expect("valid extension");

        let second =
            Extension::new(
                ExtensionId::new(2),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::LogicalQubit(
                    QubitId::new(1),
                ),
                ExtensionValue::Boolean(
                    true,
                ),
            )
            .expect("valid extension");

        assert!(
            set.insert(first).is_ok()
        );

        assert!(
            set.insert(second).is_ok()
        );

        assert_eq!(
            set.len(),
            2
        );
    }

    #[test]
    fn identical_extensions_merge_without_duplication() {
        let extension =
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::Program(
                    ProgramId::new(10),
                ),
                ExtensionValue::Null,
            )
            .expect("valid extension");

        let mut left =
            ExtensionSet::new();

        let mut right =
            ExtensionSet::new();

        left.insert(
            extension.clone()
        )
        .expect("insert");

        right.insert(
            extension
        )
        .expect("insert");

        assert!(
            left.merge(&right).is_ok()
        );

        assert_eq!(
            left.len(),
            1
        );
    }

    #[test]
    fn conflicting_merge_is_rejected() {
        let mut left =
            ExtensionSet::new();

        let mut right =
            ExtensionSet::new();

        left.insert(
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::Program(
                    ProgramId::new(1),
                ),
                ExtensionValue::Boolean(
                    true,
                ),
            )
            .expect("valid extension"),
        )
        .expect("insert");

        right.insert(
            Extension::new(
                ExtensionId::new(2),
                key(
                    "zamani",
                    "logical",
                ),
                ExtensionTarget::Program(
                    ProgramId::new(1),
                ),
                ExtensionValue::Boolean(
                    false,
                ),
            )
            .expect("valid extension"),
        )
        .expect("insert");

        assert!(
            matches!(
                left.merge(&right),
                Err(
                    ExtensionError::ConflictingExtension {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn registry_rejects_conflicting_descriptors() {
        let contract =
            key(
                "research",
                "analog",
            );

        let first =
            ExtensionDescriptor::with_description(
                contract.clone(),
                "first",
            );

        let second =
            ExtensionDescriptor::with_description(
                contract,
                "second",
            );

        let mut registry =
            ExtensionRegistry::new();

        registry
            .register(first)
            .expect("first registration");

        assert!(
            matches!(
                registry.register(second),
                Err(
                    ExtensionError::ConflictingDescriptor {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn structural_hash_is_stable_for_equal_extensions() {
        let first =
            Extension::new(
                ExtensionId::new(1),
                key(
                    "zamani",
                    "test",
                ),
                ExtensionTarget::Program(
                    ProgramId::new(1),
                ),
                ExtensionValue::Unsigned(
                    42,
                ),
            )
            .expect("valid extension");

        let second =
            first.clone();

        assert_eq!(
            structural_hash(&first),
            structural_hash(&second)
        );
    }

    #[test]
    fn unknown_opaque_extensions_are_preserved_structurally() {
        let extension =
            Extension::new(
                ExtensionId::new(99),
                key(
                    "future.architecture",
                    "operation",
                ),
                ExtensionTarget::Global,
                ExtensionValue::Opaque {
                    tag: "future.binary".to_owned(),
                    data: vec![
                        0,
                        1,
                        2,
                        3,
                        4,
                    ],
                },
            )
            .expect("valid opaque extension");

        assert!(
            extension
                .payload()
                .is_opaque()
        );

        assert!(
            extension
                .validate()
                .is_ok()
        );
    }
}