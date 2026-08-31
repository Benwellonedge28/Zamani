//! Zamani Quantum IR — Core Attribute System
//!
//! Canonical, deterministic, hardware-independent metadata for the Zamani
//! Quantum Intermediate Representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - attribute namespaces;
//! - attribute names;
//! - attribute semantic keys;
//! - attribute occurrences/identities;
//! - typed metadata values;
//! - structured metadata values;
//! - references to canonical IR identities;
//! - deterministic attribute collections;
//! - conservative merge semantics;
//! - structural validation;
//! - deterministic canonical formatting;
//! - deterministic metadata size accounting.
//!
//! This module does NOT own:
//!
//! - quantum gate semantics;
//! - quantum state;
//! - classical runtime state;
//! - operation semantics;
//! - scheduling;
//! - routing;
//! - optimization;
//! - hardware capabilities;
//! - hardware topology;
//! - calibration implementation;
//! - pulse generation;
//! - backend execution;
//! - frontend syntax;
//! - canonical cryptographic hashing.
//!
//! Those responsibilities belong to their respective IR/compiler layers.
//!
//! # Architectural principle
//!
//! An attribute describes metadata about an IR entity. It does not redefine
//! the entity.
//!
//! For example:
//!
//! ```text
//! @zamani.native
//! @zamani.experimental
//! @zamani.unit = "radians"
//! @compiler.optimization_level = 3
//! @hardware.target_hint = "example"
//! ```
//!
//! Hardware-specific attributes are permitted, but this module does not
//! interpret their semantics.
//!
//! # Namespace and name
//!
//! The semantic identity of an attribute is:
//!
//! ```text
//! AttributeKey = namespace + local name
//! ```
//!
//! `AttributeId` is an object/occurrence identity. It is intentionally
//! different from `AttributeKey`.
//!
//! This distinction is critical:
//!
//! ```text
//! AttributeId(1), zamani.native
//! AttributeId(2), zamani.native
//! ```
//!
//! describe the same semantic attribute key but different attribute objects.
//!
//! The attribute collection therefore indexes by `AttributeKey`, not by
//! `AttributeId`.
//!
//! # Quantum identity boundary
//!
//! Logical and physical qubit identities are NOT redefined here.
//!
//! They come exclusively from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This prevents multiple incompatible `QubitId` types from appearing in
//! the IR.
//!
//! # Identity boundary
//!
//! Stable IR identities come from:
//!
//! ```text
//! quantum::ir::core::identity
//! ```
//!
//! This module consumes those identities; it does not allocate them.
//!
//! # Scalability
//!
//! This module contains no quantum-machine-size constant.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_ATTRIBUTES
//! MAX_MAP_ENTRIES
//! MAX_NESTING
//! MAX_METADATA_SIZE
//! ```
//!
//! Attribute collections therefore scale according to available resources
//! and explicit external compiler/resource policies.
//!
//! No `usize` is used as a semantic IR identity.
//!
//! # Resource exhaustion safety
//!
//! Deeply nested metadata must not cause validation to recurse indefinitely.
//!
//! Validation, size accounting, and canonical rendering therefore use
//! iterative traversal where practical rather than recursive descent.
//!
//! Allocation can still fail because the host does not have sufficient
//! resources. No Rust program can promise literal mathematical infinity.
//! The semantic IR has no artificial quantum-resource ceiling.
//!
//! # Determinism
//!
//! Deterministic representation is required for:
//!
//! - reproducible compilation;
//! - distributed compilation;
//! - caching;
//! - canonical serialization;
//! - canonical hashing;
//! - provenance;
//! - benchmarking;
//! - structural comparison.
//!
//! `BTreeMap` is therefore used for all keyed attribute collections.
//!
//! # Floating point
//!
//! Attribute floating-point values are restricted to finite IEEE-754 binary64
//! values.
//!
//! NaN and infinities are rejected.
//!
//! Equality and hashing use the IEEE-754 bit representation, making
//! `+0.0` and `-0.0` distinct metadata values.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Dependency direction
//!
//! ```text
//! core::identity ───────────────► core::attribute
//!                                      │
//! quantum::ir::qubit ───────────►      │
//!                                      ▼
//!                              higher-level IR
//!                                      │
//!                      ┌───────────────┼───────────────┐
//!                      ▼               ▼               ▼
//!                  operation        program        serialization
//! ```
//!
//! This module must never depend on those higher-level modules.
//!
//! In particular, it must never import:
//!
//! ```text
//! operation
//! program
//! gate
//! hardware
//! routing
//! optimization
//! frontend
//! backend
//! ```
//!
//! # Integration contract
//!
//! `core::identity` supplies stable identity types.
//!
//! `quantum::ir::qubit` supplies logical/physical qubit identities.
//!
//! `core::types` may attach attributes to type declarations.
//!
//! `program::*` may attach attributes to programs, modules, regions, blocks,
//! operations and symbols.
//!
//! `quantum::*` may attach attributes to gates, measurements, initialization,
//! reset and quantum operations.
//!
//! `pulse::*` may attach attributes to pulse/frame/waveform objects.
//!
//! `resources::*` may attach attributes to resource/capability declarations.
//!
//! `validation::*` may validate attributes and impose external resource
//! policies.
//!
//! `serialization::*` may serialize this module structurally.
//!
//! `hashing::*` may hash this module deterministically.
//!
//! No future quantum hardware architecture should require changing this file
//! merely because the architecture introduces new metadata.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::identity::{
    AttributeId,
    BlockId,
    CalibrationId,
    CapabilityId,
    ChannelId,
    CircuitId,
    ExtensionId,
    FrameId,
    FunctionId,
    ModuleId,
    NamespaceId,
    OperationId,
    ParameterId,
    ProgramId,
    ProvenanceId,
    PulseId,
    RegionId,
    ResourceId,
    ScheduleId,
    TypeId,
    ValueId,
    WaveformId,
};

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Namespace constants
// =============================================================================

/// Canonical namespace owned by Zamani's semantic IR.
pub const ZAMANI_NAMESPACE: &str = "zamani";

/// Namespace convention for compiler-owned metadata.
pub const COMPILER_NAMESPACE: &str = "compiler";

/// Namespace convention for hardware metadata.
///
/// This namespace is intentionally not interpreted by the IR core.
pub const HARDWARE_NAMESPACE: &str = "hardware";

/// Namespace convention for user-defined metadata.
pub const USER_NAMESPACE: &str = "user";

// =============================================================================
// Attribute namespace
// =============================================================================

/// Validated attribute namespace.
///
/// A namespace consists of one or more dot-separated identifier segments.
///
/// Examples:
///
/// ```text
/// zamani
/// compiler
/// compiler.optimization
/// hardware
/// hardware.target
/// user
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttributeNamespace(String);

impl AttributeNamespace {
    /// Creates a validated namespace.
    pub fn new<S>(namespace: S) -> Result<Self, AttributeError>
    where
        S: Into<String>,
    {
        let namespace = namespace.into();

        validate_namespace(&namespace)?;

        Ok(Self(namespace))
    }

    /// Creates the canonical Zamani namespace.
    #[must_use]
    pub fn zamani() -> Self {
        Self(ZAMANI_NAMESPACE.to_owned())
    }

    /// Returns the namespace as `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns whether this namespace is the canonical Zamani namespace.
    #[must_use]
    pub fn is_zamani(&self) -> bool {
        self.0 == ZAMANI_NAMESPACE
    }
}

impl Default for AttributeNamespace {
    fn default() -> Self {
        Self::zamani()
    }
}

impl AsRef<str> for AttributeNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AttributeNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for AttributeNamespace {
    type Error = AttributeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for AttributeNamespace {
    type Error = AttributeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Attribute name
// =============================================================================

/// Validated local attribute name.
///
/// Examples:
///
/// ```text
/// native
/// experimental
/// deprecated
/// deterministic
/// optimization_level
/// target-hint
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttributeName(String);

impl AttributeName {
    /// Creates a validated local name.
    pub fn new<S>(name: S) -> Result<Self, AttributeError>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_local_name(&name)?;

        Ok(Self(name))
    }

    /// Returns the name as `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AttributeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AttributeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for AttributeName {
    type Error = AttributeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for AttributeName {
    type Error = AttributeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Attribute key
// =============================================================================

/// Semantic identity of an attribute declaration.
///
/// This is the key used by [`Attributes`].
///
/// `AttributeId` is deliberately separate from this type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttributeKey {
    namespace: AttributeNamespace,
    name: AttributeName,
}

impl AttributeKey {
    /// Creates a validated attribute key.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> Result<Self, AttributeError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        Ok(Self {
            namespace: AttributeNamespace::new(namespace)?,
            name: AttributeName::new(name)?,
        })
    }

    /// Creates a key in the canonical Zamani namespace.
    pub fn zamani<S>(name: S) -> Result<Self, AttributeError>
    where
        S: Into<String>,
    {
        Self::new(ZAMANI_NAMESPACE, name)
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &AttributeNamespace {
        &self.namespace
    }

    /// Returns the local name.
    #[must_use]
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// Returns the fully qualified name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result = String::with_capacity(
            self.namespace.as_str().len()
                + 1
                + self.name.as_str().len(),
        );

        result.push_str(self.namespace.as_str());
        result.push('.');
        result.push_str(self.name.as_str());

        result
    }
}

impl fmt::Display for AttributeKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Finite floating-point value
// =============================================================================

/// Deterministic finite IEEE-754 binary64 metadata value.
///
/// NaN and infinities are rejected.
///
/// Equality and hashing use the exact IEEE-754 bit representation.
#[derive(Clone, Copy, Debug)]
pub struct FiniteAttributeFloat(f64);

impl FiniteAttributeFloat {
    /// Creates a finite floating-point metadata value.
    pub fn new(value: f64) -> Result<Self, AttributeError> {
        if !value.is_finite() {
            return Err(AttributeError::NonFiniteFloat);
        }

        Ok(Self(value))
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns the exact IEEE-754 representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Validates this value.
    pub fn validate(self) -> Result<(), AttributeError> {
        if self.0.is_finite() {
            Ok(())
        } else {
            Err(AttributeError::NonFiniteFloat)
        }
    }
}

impl PartialEq for FiniteAttributeFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for FiniteAttributeFloat {}

impl Hash for FiniteAttributeFloat {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for FiniteAttributeFloat {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiniteAttributeFloat {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}

// =============================================================================
// Attribute value
// =============================================================================

/// Typed metadata value.
///
/// This is intentionally independent of the canonical runtime/value system.
///
/// Program values should normally be referenced using the corresponding
/// identity variant rather than duplicated into attributes.
///
/// Composite traversal is iterative in validation, size accounting and
/// canonical rendering to avoid making nesting depth a stack-based limit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttributeValue {
    /// Boolean metadata.
    Bool(bool),

    /// Signed metadata integer.
    Integer(i128),

    /// Unsigned metadata integer.
    UnsignedInteger(u128),

    /// Finite IEEE-754 binary64 metadata.
    Float(FiniteAttributeFloat),

    /// UTF-8 metadata string.
    String(String),

    /// Opaque binary metadata.
    ///
    /// This is intended for extensions and serialization-preserved metadata,
    /// not for executable code or arbitrary runtime state.
    Bytes(Vec<u8>),

    /// Symbolic/runtime parameter reference.
    Parameter(ParameterId),

    /// IR value reference.
    Value(ValueId),

    /// Logical qubit reference.
    Qubit(QubitId),

    /// Physical qubit reference.
    PhysicalQubit(PhysicalQubitId),

    /// IR type reference.
    Type(TypeId),

    /// Attribute identity reference.
    Attribute(AttributeId),

    /// Program identity reference.
    Program(ProgramId),

    /// Circuit identity reference.
    Circuit(CircuitId),

    /// Module identity reference.
    Module(ModuleId),

    /// Namespace identity reference.
    Namespace(NamespaceId),

    /// Region identity reference.
    Region(RegionId),

    /// Block identity reference.
    Block(BlockId),

    /// Operation identity reference.
    Operation(OperationId),

    /// Pulse identity reference.
    Pulse(PulseId),

    /// Waveform identity reference.
    Waveform(WaveformId),

    /// Channel identity reference.
    Channel(ChannelId),

    /// Frame identity reference.
    Frame(FrameId),

    /// Schedule identity reference.
    Schedule(ScheduleId),

    /// Resource identity reference.
    Resource(ResourceId),

    /// Capability identity reference.
    Capability(CapabilityId),

    /// Calibration identity reference.
    Calibration(CalibrationId),

    /// Function identity reference.
    Function(FunctionId),

    /// Extension identity reference.
    Extension(ExtensionId),

    /// Provenance identity reference.
    Provenance(ProvenanceId),

    /// Unit/marker metadata.
    Unit,

    /// Ordered metadata sequence.
    Array(Vec<Self>),

    /// Ordered heterogeneous metadata tuple.
    Tuple(Vec<Self>),

    /// Optional metadata.
    Optional(Option<Box<Self>>),

    /// Deterministically ordered metadata object.
    Map(BTreeMap<String, Self>),
}

impl AttributeValue {
    /// Creates a finite floating-point value.
    pub fn float(value: f64) -> Result<Self, AttributeError> {
        Ok(Self::Float(FiniteAttributeFloat::new(value)?))
    }

    /// Creates a string value.
    #[must_use]
    pub fn string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::String(value.into())
    }

    /// Creates opaque binary metadata.
    #[must_use]
    pub fn bytes(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }

    /// Creates an ordered sequence.
    #[must_use]
    pub fn array(values: Vec<Self>) -> Self {
        Self::Array(values)
    }

    /// Creates an ordered tuple.
    #[must_use]
    pub fn tuple(values: Vec<Self>) -> Self {
        Self::Tuple(values)
    }

    /// Creates an optional value.
    #[must_use]
    pub fn optional(value: Option<Self>) -> Self {
        Self::Optional(value.map(Box::new))
    }

    /// Creates a deterministic map.
    #[must_use]
    pub fn map(values: BTreeMap<String, Self>) -> Self {
        Self::Map(values)
    }

    /// Returns the broad value kind.
    #[must_use]
    pub const fn kind(&self) -> AttributeValueKind {
        match self {
            Self::Bool(_) => AttributeValueKind::Bool,
            Self::Integer(_) => AttributeValueKind::Integer,
            Self::UnsignedInteger(_) => AttributeValueKind::UnsignedInteger,
            Self::Float(_) => AttributeValueKind::Float,
            Self::String(_) => AttributeValueKind::String,
            Self::Bytes(_) => AttributeValueKind::Bytes,
            Self::Parameter(_) => AttributeValueKind::Parameter,
            Self::Value(_) => AttributeValueKind::Value,
            Self::Qubit(_) => AttributeValueKind::Qubit,
            Self::PhysicalQubit(_) => AttributeValueKind::PhysicalQubit,
            Self::Type(_) => AttributeValueKind::Type,
            Self::Attribute(_) => AttributeValueKind::Attribute,
            Self::Program(_) => AttributeValueKind::Program,
            Self::Circuit(_) => AttributeValueKind::Circuit,
            Self::Module(_) => AttributeValueKind::Module,
            Self::Namespace(_) => AttributeValueKind::Namespace,
            Self::Region(_) => AttributeValueKind::Region,
            Self::Block(_) => AttributeValueKind::Block,
            Self::Operation(_) => AttributeValueKind::Operation,
            Self::Pulse(_) => AttributeValueKind::Pulse,
            Self::Waveform(_) => AttributeValueKind::Waveform,
            Self::Channel(_) => AttributeValueKind::Channel,
            Self::Frame(_) => AttributeValueKind::Frame,
            Self::Schedule(_) => AttributeValueKind::Schedule,
            Self::Resource(_) => AttributeValueKind::Resource,
            Self::Capability(_) => AttributeValueKind::Capability,
            Self::Calibration(_) => AttributeValueKind::Calibration,
            Self::Function(_) => AttributeValueKind::Function,
            Self::Extension(_) => AttributeValueKind::Extension,
            Self::Provenance(_) => AttributeValueKind::Provenance,
            Self::Unit => AttributeValueKind::Unit,
            Self::Array(_) => AttributeValueKind::Array,
            Self::Tuple(_) => AttributeValueKind::Tuple,
            Self::Optional(_) => AttributeValueKind::Optional,
            Self::Map(_) => AttributeValueKind::Map,
        }
    }

    /// Returns whether the value is non-composite.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        !matches!(
            self,
            Self::Array(_)
                | Self::Tuple(_)
                | Self::Optional(_)
                | Self::Map(_)
        )
    }

    /// Returns whether this value contains child values.
    #[must_use]
    pub const fn is_composite(&self) -> bool {
        !self.is_scalar()
    }

    /// Validates this value without recursive stack growth.
    pub fn validate(&self) -> Result<(), AttributeError> {
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Float(float) => {
                    float.validate()?;
                }

                Self::String(_) => {}

                Self::Bytes(_) => {}

                Self::Map(values) => {
                    for (key, child) in values {
                        if key.is_empty() {
                            return Err(AttributeError::EmptyMapKey);
                        }

                        stack.push(child);
                    }
                }

                Self::Array(values) | Self::Tuple(values) => {
                    for child in values {
                        stack.push(child);
                    }
                }

                Self::Optional(Some(child)) => {
                    stack.push(child);
                }

                Self::Optional(None) => {}

                Self::Bool(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Parameter(_)
                | Self::Value(_)
                | Self::Qubit(_)
                | Self::PhysicalQubit(_)
                | Self::Type(_)
                | Self::Attribute(_)
                | Self::Program(_)
                | Self::Circuit(_)
                | Self::Module(_)
                | Self::Namespace(_)
                | Self::Region(_)
                | Self::Block(_)
                | Self::Operation(_)
                | Self::Pulse(_)
                | Self::Waveform(_)
                | Self::Channel(_)
                | Self::Frame(_)
                | Self::Schedule(_)
                | Self::Resource(_)
                | Self::Capability(_)
                | Self::Calibration(_)
                | Self::Function(_)
                | Self::Extension(_)
                | Self::Provenance(_)
                | Self::Unit => {}
            }
        }

        Ok(())
    }

    /// Calculates deterministic metadata size accounting.
    ///
    /// This is an accounting function, not an allocator measurement.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        let mut total = 0_u64;
        let mut stack = vec![(self, 1_u64)];

        while let Some((value, multiplier)) = stack.pop() {
            let own = match value {
                Self::Bool(_) => 1_u64,
                Self::Integer(_) => 16_u64,
                Self::UnsignedInteger(_) => 16_u64,
                Self::Float(_) => 8_u64,

                Self::String(value) => {
                    checked_usize_to_u64(value.len())?
                        .checked_add(8)
                        .ok_or(AttributeError::SizeOverflow)?
                }

                Self::Bytes(value) => {
                    checked_usize_to_u64(value.len())?
                        .checked_add(8)
                        .ok_or(AttributeError::SizeOverflow)?
                }

                Self::Parameter(_)
                | Self::Value(_)
                | Self::Qubit(_)
                | Self::PhysicalQubit(_)
                | Self::Type(_)
                | Self::Attribute(_)
                | Self::Program(_)
                | Self::Circuit(_)
                | Self::Module(_)
                | Self::Namespace(_)
                | Self::Region(_)
                | Self::Block(_)
                | Self::Operation(_)
                | Self::Pulse(_)
                | Self::Waveform(_)
                | Self::Channel(_)
                | Self::Frame(_)
                | Self::Schedule(_)
                | Self::Resource(_)
                | Self::Capability(_)
                | Self::Calibration(_)
                | Self::Function(_)
                | Self::Extension(_)
                | Self::Provenance(_) => 8_u64,

                Self::Unit => 0_u64,

                Self::Array(values) | Self::Tuple(values) => {
                    let length = checked_usize_to_u64(values.len())?;

                    for child in values {
                        stack.push((child, 1));
                    }

                    length
                        .checked_mul(8)
                        .and_then(|v| v.checked_add(8))
                        .ok_or(AttributeError::SizeOverflow)?
                }

                Self::Optional(Some(child)) => {
                    stack.push((child, 1));
                    1
                }

                Self::Optional(None) => 1,

                Self::Map(values) => {
                    let length = checked_usize_to_u64(values.len())?;

                    for (key, child) in values {
                        let key_size =
                            checked_usize_to_u64(key.len())?;

                        total = total
                            .checked_add(key_size)
                            .and_then(|v| v.checked_add(8))
                            .ok_or(AttributeError::SizeOverflow)?;

                        stack.push((child, 1));
                    }

                    length
                        .checked_mul(16)
                        .and_then(|v| v.checked_add(8))
                        .ok_or(AttributeError::SizeOverflow)?
                }
            };

            let contribution = own
                .checked_mul(multiplier)
                .ok_or(AttributeError::SizeOverflow)?;

            total = total
                .checked_add(contribution)
                .ok_or(AttributeError::SizeOverflow)?;
        }

        Ok(total)
    }

    /// Returns a deterministic canonical textual representation.
    ///
    /// This method uses an explicit traversal stack instead of recursive
    /// formatting.
    pub fn canonical_string(&self) -> Result<String, AttributeError> {
        let mut output = String::new();
        let mut stack = vec![CanonicalTask::Value(self)];

        while let Some(task) = stack.pop() {
            match task {
                CanonicalTask::Text(text) => {
                    output.push_str(text);
                }

                CanonicalTask::Value(value) => {
                    push_canonical_value_tasks(
                        value,
                        &mut stack,
                    );
                }

                CanonicalTask::MapEntrySeparator => {
                    output.push(',');
                }

                CanonicalTask::SequenceSeparator => {
                    output.push(',');
                }

                CanonicalTask::MapKey(key) => {
                    write_escaped_string_to(
                        &mut output,
                        key,
                    );
                }
            }
        }

        Ok(output)
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for AttributeValue {
    fn from(value: i8) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i16> for AttributeValue {
    fn from(value: i16) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        Self::Integer(i128::from(value))
    }
}

impl From<i128> for AttributeValue {
    fn from(value: i128) -> Self {
        Self::Integer(value)
    }
}

impl From<isize> for AttributeValue {
    fn from(value: isize) -> Self {
        Self::Integer(value as i128)
    }
}

impl From<u8> for AttributeValue {
    fn from(value: u8) -> Self {
        Self::UnsignedInteger(u128::from(value))
    }
}

impl From<u16> for AttributeValue {
    fn from(value: u16) -> Self {
        Self::UnsignedInteger(u128::from(value))
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        Self::UnsignedInteger(u128::from(value))
    }
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(u128::from(value))
    }
}

impl From<u128> for AttributeValue {
    fn from(value: u128) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl From<usize> for AttributeValue {
    fn from(value: usize) -> Self {
        Self::UnsignedInteger(value as u128)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Vec<u8>> for AttributeValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<ParameterId> for AttributeValue {
    fn from(value: ParameterId) -> Self {
        Self::Parameter(value)
    }
}

impl From<ValueId> for AttributeValue {
    fn from(value: ValueId) -> Self {
        Self::Value(value)
    }
}

impl From<QubitId> for AttributeValue {
    fn from(value: QubitId) -> Self {
        Self::Qubit(value)
    }
}

impl From<PhysicalQubitId> for AttributeValue {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl From<TypeId> for AttributeValue {
    fn from(value: TypeId) -> Self {
        Self::Type(value)
    }
}

impl From<AttributeId> for AttributeValue {
    fn from(value: AttributeId) -> Self {
        Self::Attribute(value)
    }
}

impl From<ProgramId> for AttributeValue {
    fn from(value: ProgramId) -> Self {
        Self::Program(value)
    }
}

impl From<CircuitId> for AttributeValue {
    fn from(value: CircuitId) -> Self {
        Self::Circuit(value)
    }
}

impl From<ModuleId> for AttributeValue {
    fn from(value: ModuleId) -> Self {
        Self::Module(value)
    }
}

impl From<NamespaceId> for AttributeValue {
    fn from(value: NamespaceId) -> Self {
        Self::Namespace(value)
    }
}

impl From<RegionId> for AttributeValue {
    fn from(value: RegionId) -> Self {
        Self::Region(value)
    }
}

impl From<BlockId> for AttributeValue {
    fn from(value: BlockId) -> Self {
        Self::Block(value)
    }
}

impl From<OperationId> for AttributeValue {
    fn from(value: OperationId) -> Self {
        Self::Operation(value)
    }
}

impl From<PulseId> for AttributeValue {
    fn from(value: PulseId) -> Self {
        Self::Pulse(value)
    }
}

impl From<WaveformId> for AttributeValue {
    fn from(value: WaveformId) -> Self {
        Self::Waveform(value)
    }
}

impl From<ChannelId> for AttributeValue {
    fn from(value: ChannelId) -> Self {
        Self::Channel(value)
    }
}

impl From<FrameId> for AttributeValue {
    fn from(value: FrameId) -> Self {
        Self::Frame(value)
    }
}

impl From<ScheduleId> for AttributeValue {
    fn from(value: ScheduleId) -> Self {
        Self::Schedule(value)
    }
}

impl From<ResourceId> for AttributeValue {
    fn from(value: ResourceId) -> Self {
        Self::Resource(value)
    }
}

impl From<CapabilityId> for AttributeValue {
    fn from(value: CapabilityId) -> Self {
        Self::Capability(value)
    }
}

impl From<CalibrationId> for AttributeValue {
    fn from(value: CalibrationId) -> Self {
        Self::Calibration(value)
    }
}

impl From<FunctionId> for AttributeValue {
    fn from(value: FunctionId) -> Self {
        Self::Function(value)
    }
}

impl From<ExtensionId> for AttributeValue {
    fn from(value: ExtensionId) -> Self {
        Self::Extension(value)
    }
}

impl From<ProvenanceId> for AttributeValue {
    fn from(value: ProvenanceId) -> Self {
        Self::Provenance(value)
    }
}

// =============================================================================
// Attribute value kind
// =============================================================================

/// Broad category of an [`AttributeValue`].
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum AttributeValueKind {
    Bool,
    Integer,
    UnsignedInteger,
    Float,
    String,
    Bytes,
    Parameter,
    Value,
    Qubit,
    PhysicalQubit,
    Type,
    Attribute,
    Program,
    Circuit,
    Module,
    Namespace,
    Region,
    Block,
    Operation,
    Pulse,
    Waveform,
    Channel,
    Frame,
    Schedule,
    Resource,
    Capability,
    Calibration,
    Function,
    Extension,
    Provenance,
    Unit,
    Array,
    Tuple,
    Optional,
    Map,
}

impl fmt::Display for AttributeValueKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let text = match self {
            Self::Bool => "bool",
            Self::Integer => "integer",
            Self::UnsignedInteger => "unsigned_integer",
            Self::Float => "float",
            Self::String => "string",
            Self::Bytes => "bytes",
            Self::Parameter => "parameter",
            Self::Value => "value",
            Self::Qubit => "qubit",
            Self::PhysicalQubit => "physical_qubit",
            Self::Type => "type",
            Self::Attribute => "attribute",
            Self::Program => "program",
            Self::Circuit => "circuit",
            Self::Module => "module",
            Self::Namespace => "namespace",
            Self::Region => "region",
            Self::Block => "block",
            Self::Operation => "operation",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Schedule => "schedule",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Calibration => "calibration",
            Self::Function => "function",
            Self::Extension => "extension",
            Self::Provenance => "provenance",
            Self::Unit => "unit",
            Self::Array => "array",
            Self::Tuple => "tuple",
            Self::Optional => "optional",
            Self::Map => "map",
        };

        formatter.write_str(text)
    }
}

// =============================================================================
// Attribute
// =============================================================================

/// One canonical IR attribute occurrence.
///
/// The `AttributeId` is occurrence identity.
///
/// Semantic lookup is always performed through [`AttributeKey`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Attribute {
    id: AttributeId,
    key: AttributeKey,
    value: AttributeValue,
}

impl Attribute {
    /// Creates an attribute.
    pub fn new(
        id: AttributeId,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<Self, AttributeError> {
        value.validate()?;

        Ok(Self {
            id,
            key,
            value,
        })
    }

    /// Creates a marker attribute whose value is [`AttributeValue::Unit`].
    pub fn marker(
        id: AttributeId,
        key: AttributeKey,
    ) -> Result<Self, AttributeError> {
        Self::new(id, key, AttributeValue::Unit)
    }

    /// Creates a marker in the Zamani namespace.
    pub fn zamani_marker<S>(
        id: AttributeId,
        name: S,
    ) -> Result<Self, AttributeError>
    where
        S: Into<String>,
    {
        Self::marker(
            id,
            AttributeKey::zamani(name)?,
        )
    }

    /// Creates a valued Zamani attribute.
    pub fn zamani<S, V>(
        id: AttributeId,
        name: S,
        value: V,
    ) -> Result<Self, AttributeError>
    where
        S: Into<String>,
        V: Into<AttributeValue>,
    {
        Self::new(
            id,
            AttributeKey::zamani(name)?,
            value.into(),
        )
    }

    /// Returns the occurrence identity.
    #[must_use]
    pub const fn id(&self) -> AttributeId {
        self.id
    }

    /// Returns the semantic key.
    #[must_use]
    pub fn key(&self) -> &AttributeKey {
        &self.key
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &AttributeNamespace {
        self.key.namespace()
    }

    /// Returns the local name.
    #[must_use]
    pub fn name(&self) -> &AttributeName {
        self.key.name()
    }

    /// Returns the fully qualified name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.key.qualified_name()
    }

    /// Returns the value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Returns the value kind.
    #[must_use]
    pub fn value_kind(&self) -> AttributeValueKind {
        self.value.kind()
    }

    /// Returns whether this is a marker attribute.
    #[must_use]
    pub fn is_marker(&self) -> bool {
        matches!(self.value, AttributeValue::Unit)
    }

    /// Validates the attribute.
    pub fn validate(&self) -> Result<(), AttributeError> {
        self.value.validate()
    }

    /// Returns deterministic metadata-size accounting.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        let namespace =
            checked_usize_to_u64(self.namespace().as_str().len())?;

        let name =
            checked_usize_to_u64(self.name().as_str().len())?;

        let value = self.value.estimated_size()?;

        8_u64
            .checked_add(namespace)
            .and_then(|v| v.checked_add(name))
            .and_then(|v| v.checked_add(value))
            .ok_or(AttributeError::SizeOverflow)
    }

    /// Returns canonical deterministic text.
    pub fn canonical_string(&self) -> Result<String, AttributeError> {
        let value = self.value.canonical_string()?;

        Ok(format!(
            "@{} = {}",
            self.key,
            value
        ))
    }
}

impl fmt::Display for Attribute {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = self
            .value
            .canonical_string()
            .map_err(|_| fmt::Error)?;

        write!(
            formatter,
            "@{} = {}",
            self.key,
            value
        )
    }
}

// =============================================================================
// Attribute collection
// =============================================================================

/// Deterministic semantic attribute collection.
///
/// The collection is keyed by [`AttributeKey`].
///
/// Consequently, one semantic attribute key can have at most one value in a
/// collection.
///
/// Attribute IDs are not the lookup key.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attributes {
    values: BTreeMap<AttributeKey, Attribute>,
}

impl Hash for Attributes {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.values.len().hash(state);

        for (key, attribute) in &self.values {
            key.hash(state);
            attribute.hash(state);
        }
    }
}

impl Attributes {
    /// Creates an empty deterministic collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Returns the number of semantic attributes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Inserts an attribute conservatively.
    ///
    /// Behavior:
    ///
    /// - same key + same value + same identity => no-op;
    /// - same key + same value + different identity => conflict;
    /// - same key + different value => conflict;
    /// - new key => inserted.
    ///
    /// No implicit overwrite is permitted.
    pub fn insert(
        &mut self,
        attribute: Attribute,
    ) -> Result<Option<Attribute>, AttributeError> {
        attribute.validate()?;

        let key = attribute.key().clone();

        if let Some(existing) = self.values.get(&key) {
            if existing == &attribute {
                return Ok(None);
            }

            return Err(
                AttributeError::ConflictingAttribute {
                    key,
                },
            );
        }

        Ok(self.values.insert(key, attribute))
    }

    /// Inserts an attribute and explicitly replaces an existing value.
    ///
    /// This API is intentionally separate from [`Self::insert`].
    pub fn insert_or_replace(
        &mut self,
        attribute: Attribute,
    ) -> Result<Option<Attribute>, AttributeError> {
        attribute.validate()?;

        let key = attribute.key().clone();

        Ok(self.values.insert(key, attribute))
    }

    /// Inserts an attribute while preserving the existing identity if the
    /// semantic key and value are identical.
    ///
    /// This is useful for idempotent compiler passes that may independently
    /// construct equivalent metadata objects.
    pub fn insert_semantic(
        &mut self,
        attribute: Attribute,
    ) -> Result<SemanticInsertResult, AttributeError> {
        attribute.validate()?;

        let key = attribute.key().clone();

        match self.values.get(&key) {
            None => {
                self.values.insert(key, attribute);
                Ok(SemanticInsertResult::Inserted)
            }

            Some(existing) if existing.value() == attribute.value() => {
                Ok(SemanticInsertResult::AlreadyPresent)
            }

            Some(_) => {
                Err(AttributeError::ConflictingAttribute { key })
            }
        }
    }

    /// Removes an attribute by semantic key.
    pub fn remove(
        &mut self,
        key: &AttributeKey,
    ) -> Option<Attribute> {
        self.values.remove(key)
    }

    /// Removes an attribute by namespace/name.
    pub fn remove_by_name(
        &mut self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<Attribute>, AttributeError> {
        let key = AttributeKey::new(namespace, name)?;
        Ok(self.remove(&key))
    }

    /// Returns an attribute by semantic key.
    #[must_use]
    pub fn get(
        &self,
        key: &AttributeKey,
    ) -> Option<&Attribute> {
        self.values.get(key)
    }

    /// Returns an attribute by namespace/name.
    pub fn get_by_name(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<&Attribute>, AttributeError> {
        let key = AttributeKey::new(namespace, name)?;
        Ok(self.get(&key))
    }

    /// Returns a Zamani attribute.
    pub fn get_zamani(
        &self,
        name: &str,
    ) -> Result<Option<&Attribute>, AttributeError> {
        self.get_by_name(ZAMANI_NAMESPACE, name)
    }

    /// Returns whether a semantic key exists.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &AttributeKey,
    ) -> bool {
        self.values.contains_key(key)
    }

    /// Returns whether a namespace/name exists.
    pub fn contains_name(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<bool, AttributeError> {
        Ok(self.get_by_name(namespace, name)?.is_some())
    }

    /// Returns attributes in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Attribute> {
        self.values.values()
    }

    /// Returns semantic keys in deterministic order.
    pub fn keys(
        &self,
    ) -> impl Iterator<Item = &AttributeKey> {
        self.values.keys()
    }

    /// Returns keyed entries in deterministic order.
    pub fn iter_keyed(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &Attribute)> {
        self.values.iter()
    }

    /// Returns the deterministic underlying map.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> &BTreeMap<AttributeKey, Attribute> {
        &self.values
    }

    /// Consumes this collection into its deterministic map.
    #[must_use]
    pub fn into_map(
        self,
    ) -> BTreeMap<AttributeKey, Attribute> {
        self.values
    }

    /// Removes all attributes.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Validates every attribute.
    pub fn validate(&self) -> Result<(), AttributeError> {
        for attribute in self.values.values() {
            attribute.validate()?;
        }

        Ok(())
    }

    /// Merges another collection conservatively.
    ///
    /// Existing semantic conflicts are rejected.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), AttributeError> {
        for attribute in other.iter() {
            self.insert_semantic(attribute.clone())?;
        }

        Ok(())
    }

    /// Merges another collection with explicit replacement semantics.
    pub fn merge_replace(
        &mut self,
        other: &Self,
    ) -> Result<(), AttributeError> {
        for attribute in other.iter() {
            self.insert_or_replace(attribute.clone())?;
        }

        Ok(())
    }

    /// Calculates deterministic metadata-size accounting.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        let mut total = 8_u64;

        for attribute in self.values.values() {
            total = total
                .checked_add(attribute.estimated_size()?)
                .ok_or(AttributeError::SizeOverflow)?;
        }

        Ok(total)
    }

    /// Creates a collection from an iterator.
    pub fn try_from_iter<I>(
        attributes: I,
    ) -> Result<Self, AttributeError>
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut result = Self::new();

        for attribute in attributes {
            result.insert(attribute)?;
        }

        Ok(result)
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;

    type IntoIter =
        std::collections::btree_map::Values<
            'a,
            AttributeKey,
            Attribute,
        >;

    fn into_iter(self) -> Self::IntoIter {
        self.values.values()
    }
}

impl IntoIterator for Attributes {
    type Item = Attribute;

    type IntoIter =
        std::collections::btree_map::IntoValues<
            AttributeKey,
            Attribute,
        >;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_values()
    }
}

// =============================================================================
// Semantic insertion result
// =============================================================================

/// Result of inserting an attribute by semantic identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SemanticInsertResult {
    /// A new semantic key was inserted.
    Inserted,

    /// An equivalent semantic key/value already existed.
    AlreadyPresent,
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the attribute system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeError {
    /// Namespace is empty.
    EmptyNamespace,

    /// Attribute name is empty.
    EmptyName,

    /// Namespace contains an empty dot-separated segment.
    EmptyNamespaceSegment,

    /// A local name contains invalid syntax.
    InvalidName {
        /// Kind of invalid identifier.
        kind: &'static str,

        /// Invalid input.
        value: String,
    },

    /// Namespace segment contains invalid syntax.
    InvalidNamespaceSegment {
        /// Invalid segment.
        segment: String,
    },

    /// Metadata object key is empty.
    EmptyMapKey,

    /// Metadata float is NaN or infinite.
    NonFiniteFloat,

    /// Two attributes use the same semantic key with incompatible metadata.
    ConflictingAttribute {
        /// Conflicting semantic key.
        key: AttributeKey,
    },

    /// Checked metadata-size accounting overflowed.
    SizeOverflow,
}

impl fmt::Display for AttributeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str(
                    "attribute namespace must not be empty",
                )
            }

            Self::EmptyName => {
                formatter.write_str(
                    "attribute name must not be empty",
                )
            }

            Self::EmptyNamespaceSegment => {
                formatter.write_str(
                    "attribute namespace contains an empty segment",
                )
            }

            Self::InvalidName { kind, value } => {
                write!(
                    formatter,
                    "invalid {kind}: {value}"
                )
            }

            Self::InvalidNamespaceSegment { segment } => {
                write!(
                    formatter,
                    "invalid attribute namespace segment: {segment}"
                )
            }

            Self::EmptyMapKey => {
                formatter.write_str(
                    "attribute metadata map key must not be empty",
                )
            }

            Self::NonFiniteFloat => {
                formatter.write_str(
                    "attribute floating-point metadata must be finite",
                )
            }

            Self::ConflictingAttribute { key } => {
                write!(
                    formatter,
                    "conflicting attributes for semantic key {key}"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "attribute metadata size accounting overflowed",
                )
            }
        }
    }
}

impl std::error::Error for AttributeError {}

// =============================================================================
// Canonical rendering
// =============================================================================

/// Explicit canonical-rendering task.
///
/// Keeping traversal state outside the call stack prevents metadata nesting
/// depth from becoming a Rust call-stack limitation.
enum CanonicalTask<'a> {
    Value(&'a AttributeValue),
    Text(&'static str),
    MapKey(&'a str),
    MapEntrySeparator,
    SequenceSeparator,
}

/// Pushes rendering tasks in reverse order so that the resulting output is
/// deterministic and correctly ordered.
fn push_canonical_value_tasks<'a>(
    value: &'a AttributeValue,
    stack: &mut Vec<CanonicalTask<'a>>,
) {
    match value {
        AttributeValue::Bool(value) => {
            stack.push(if *value {
                CanonicalTask::Text("true")
            } else {
                CanonicalTask::Text("false")
            });
        }

        AttributeValue::Integer(value) => {
            stack.push(CanonicalTask::Text(
                Box::leak(value.to_string().into_boxed_str()),
            ));
        }

        AttributeValue::UnsignedInteger(value) => {
            let text = format!("{value}u");
            stack.push(CanonicalTask::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        AttributeValue::Float(value) => {
            let text =
                format!("float_bits(0x{:016x})", value.bits());

            stack.push(CanonicalTask::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        AttributeValue::String(value) => {
            let text = canonical_escaped_string(value);
            stack.push(CanonicalTask::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        AttributeValue::Bytes(value) => {
            let mut text = String::from("bytes(");

            for byte in value {
                use std::fmt::Write;

                let _ = write!(
                    &mut text,
                    "{byte:02x}"
                );
            }

            text.push(')');

            stack.push(CanonicalTask::Text(
                Box::leak(text.into_boxed_str()),
            ));
        }

        AttributeValue::Parameter(value) => {
            push_identity_text(
                stack,
                "parameter(",
                value,
                ")",
            );
        }

        AttributeValue::Value(value) => {
            push_identity_text(
                stack,
                "value(",
                value,
                ")",
            );
        }

        AttributeValue::Qubit(value) => {
            push_identity_text(
                stack,
                "qubit(",
                value,
                ")",
            );
        }

        AttributeValue::PhysicalQubit(value) => {
            push_identity_text(
                stack,
                "physical_qubit(",
                value,
                ")",
            );
        }

        AttributeValue::Type(value) => {
            push_identity_text(
                stack,
                "type(",
                value,
                ")",
            );
        }

        AttributeValue::Attribute(value) => {
            push_identity_text(
                stack,
                "attribute(",
                value,
                ")",
            );
        }

        AttributeValue::Program(value) => {
            push_identity_text(
                stack,
                "program(",
                value,
                ")",
            );
        }

        AttributeValue::Circuit(value) => {
            push_identity_text(
                stack,
                "circuit(",
                value,
                ")",
            );
        }

        AttributeValue::Module(value) => {
            push_identity_text(
                stack,
                "module(",
                value,
                ")",
            );
        }

        AttributeValue::Namespace(value) => {
            push_identity_text(
                stack,
                "namespace(",
                value,
                ")",
            );
        }

        AttributeValue::Region(value) => {
            push_identity_text(
                stack,
                "region(",
                value,
                ")",
            );
        }

        AttributeValue::Block(value) => {
            push_identity_text(
                stack,
                "block(",
                value,
                ")",
            );
        }

        AttributeValue::Operation(value) => {
            push_identity_text(
                stack,
                "operation(",
                value,
                ")",
            );
        }

        AttributeValue::Pulse(value) => {
            push_identity_text(
                stack,
                "pulse(",
                value,
                ")",
            );
        }

        AttributeValue::Waveform(value) => {
            push_identity_text(
                stack,
                "waveform(",
                value,
                ")",
            );
        }

        AttributeValue::Channel(value) => {
            push_identity_text(
                stack,
                "channel(",
                value,
                ")",
            );
        }

        AttributeValue::Frame(value) => {
            push_identity_text(
                stack,
                "frame(",
                value,
                ")",
            );
        }

        AttributeValue::Schedule(value) => {
            push_identity_text(
                stack,
                "schedule(",
                value,
                ")",
            );
        }

        AttributeValue::Resource(value) => {
            push_identity_text(
                stack,
                "resource(",
                value,
                ")",
            );
        }

        AttributeValue::Capability(value) => {
            push_identity_text(
                stack,
                "capability(",
                value,
                ")",
            );
        }

        AttributeValue::Calibration(value) => {
            push_identity_text(
                stack,
                "calibration(",
                value,
                ")",
            );
        }

        AttributeValue::Function(value) => {
            push_identity_text(
                stack,
                "function(",
                value,
                ")",
            );
        }

        AttributeValue::Extension(value) => {
            push_identity_text(
                stack,
                "extension(",
                value,
                ")",
            );
        }

        AttributeValue::Provenance(value) => {
            push_identity_text(
                stack,
                "provenance(",
                value,
                ")",
            );
        }

        AttributeValue::Unit => {
            stack.push(CanonicalTask::Text("unit"));
        }

        AttributeValue::Array(values) => {
            push_sequence_tasks(
                values,
                '[',
                ']',
                stack,
            );
        }

        AttributeValue::Tuple(values) => {
            push_sequence_tasks(
                values,
                '(',
                ')',
                stack,
            );
        }

        AttributeValue::Optional(None) => {
            stack.push(CanonicalTask::Text("none"));
        }

        AttributeValue::Optional(Some(value)) => {
            stack.push(CanonicalTask::Text(")"));
            stack.push(CanonicalTask::Value(value));
            stack.push(CanonicalTask::Text("some("));
        }

        AttributeValue::Map(values) => {
            stack.push(CanonicalTask::Text("}"));

            let mut entries =
                values.iter().collect::<Vec<_>>();

            entries.reverse();

            for (index, (key, value)) in
                entries.into_iter().enumerate()
            {
                if index != 0 {
                    stack.push(
                        CanonicalTask::MapEntrySeparator,
                    );
                }

                stack.push(CanonicalTask::Value(value));
                stack.push(CanonicalTask::Text(":"));
                stack.push(CanonicalTask::MapKey(key));
            }

            stack.push(CanonicalTask::Text("{"));
        }
    }
}

/// Pushes sequence rendering tasks.
fn push_sequence_tasks<'a>(
    values: &'a [AttributeValue],
    open: char,
    close: char,
    stack: &mut Vec<CanonicalTask<'a>>,
) {
    let close_text = match close {
        ']' => "]",
        ')' => ")",
        _ => "",
    };

    stack.push(CanonicalTask::Text(close_text));

    for index in (0..values.len()).rev() {
        if index + 1 < values.len() {
            stack.push(CanonicalTask::SequenceSeparator);
        }

        stack.push(CanonicalTask::Value(&values[index]));
    }

    let open_text = match open {
        '[' => "[",
        '(' => "(",
        _ => "",
    };

    stack.push(CanonicalTask::Text(open_text));
}

/// Adds a deterministic identity rendering task.
///
/// The identity itself implements `Display`; this helper materializes the
/// small textual representation while preserving iterative traversal.
fn push_identity_text<T>(
    stack: &mut Vec<CanonicalTask<'static>>,
    prefix: &'static str,
    value: &T,
    suffix: &'static str,
) where
    T: fmt::Display,
{
    let text = format!(
        "{prefix}{value}{suffix}"
    );

    stack.push(CanonicalTask::Text(
        Box::leak(text.into_boxed_str()),
    ));
}

/// Produces a canonical escaped string.
///
/// Escaping is deterministic and UTF-8 preserving.
fn canonical_escaped_string(value: &str) -> String {
    let mut result = String::with_capacity(
        value.len() + 2,
    );

    write_escaped_string_to(
        &mut result,
        value,
    );

    result
}

/// Appends an escaped string to a destination.
fn write_escaped_string_to(
    destination: &mut String,
    value: &str,
) {
    use std::fmt::Write;

    destination.push('"');

    for character in value.chars() {
        match character {
            '\\' => destination.push_str("\\\\"),
            '"' => destination.push_str("\\\""),
            '\n' => destination.push_str("\\n"),
            '\r' => destination.push_str("\\r"),
            '\t' => destination.push_str("\\t"),

            character if character.is_control() => {
                let _ = write!(
                    destination,
                    "\\u{{{:x}}}",
                    character as u32
                );
            }

            character => destination.push(character),
        }
    }

    destination.push('"');
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a local attribute identifier.
fn validate_local_name(
    value: &str,
) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(AttributeError::EmptyName);
    }

    let mut characters =
        value.as_bytes().iter().copied();

    let first =
        characters.next().ok_or(
            AttributeError::EmptyName,
        )?;

    if !is_identifier_start(first) {
        return Err(
            AttributeError::InvalidName {
                kind: "attribute name",
                value: value.to_owned(),
            },
        );
    }

    for byte in characters {
        if !is_identifier_continue(byte) {
            return Err(
                AttributeError::InvalidName {
                    kind: "attribute name",
                    value: value.to_owned(),
                },
            );
        }
    }

    Ok(())
}

/// Validates a dot-qualified namespace.
fn validate_namespace(
    value: &str,
) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(AttributeError::EmptyNamespace);
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(
                AttributeError::EmptyNamespaceSegment,
            );
        }

        let bytes =
            segment.as_bytes();

        let first =
            bytes[0];

        if !is_identifier_start(first) {
            return Err(
                AttributeError::InvalidNamespaceSegment {
                    segment: segment.to_owned(),
                },
            );
        }

        for byte in bytes.iter().copied().skip(1) {
            if !is_identifier_continue(byte) {
                return Err(
                    AttributeError::InvalidNamespaceSegment {
                        segment: segment.to_owned(),
                    },
                );
            }
        }
    }

    Ok(())
}

/// ASCII identifier start.
#[inline]
const fn is_identifier_start(
    byte: u8,
) -> bool {
    matches!(
        byte,
        b'a'..=b'z'
            | b'A'..=b'Z'
            | b'_'
    )
}

/// ASCII identifier continuation.
#[inline]
const fn is_identifier_continue(
    byte: u8,
) -> bool {
    is_identifier_start(byte)
        || matches!(
            byte,
            b'0'..=b'9'
                | b'-'
        )
}

/// Checked conversion from `usize` to `u64`.
#[inline]
fn checked_usize_to_u64(
    value: usize,
) -> Result<u64, AttributeError> {
    u64::try_from(value)
        .map_err(|_| AttributeError::SizeOverflow)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zamani_namespace_is_canonical() {
        let namespace =
            AttributeNamespace::zamani();

        assert_eq!(
            namespace.as_str(),
            "zamani"
        );

        assert!(namespace.is_zamani());
    }

    #[test]
    fn qualified_namespace_is_valid() {
        let namespace =
            AttributeNamespace::new(
                "compiler.optimization",
            )
            .expect("valid namespace");

        assert_eq!(
            namespace.as_str(),
            "compiler.optimization"
        );
    }

    #[test]
    fn namespace_rejects_empty_segment() {
        assert_eq!(
            AttributeNamespace::new(
                "compiler..optimization",
            ),
            Err(
                AttributeError::EmptyNamespaceSegment
            )
        );
    }

    #[test]
    fn namespace_rejects_invalid_segment() {
        assert!(matches!(
            AttributeNamespace::new(
                "compiler.123optimization",
            ),
            Err(
                AttributeError::InvalidNamespaceSegment { .. }
            )
        ));
    }

    #[test]
    fn attribute_name_accepts_hyphen_and_underscore() {
        assert!(
            AttributeName::new(
                "target-hint",
            )
            .is_ok()
        );

        assert!(
            AttributeName::new(
                "optimization_level",
            )
            .is_ok()
        );
    }

    #[test]
    fn attribute_name_rejects_leading_digit() {
        assert!(matches!(
            AttributeName::new(
                "123invalid",
            ),
            Err(
                AttributeError::InvalidName { .. }
            )
        ));
    }

    #[test]
    fn attribute_key_is_deterministic() {
        let key =
            AttributeKey::new(
                "compiler.optimization",
                "level",
            )
            .expect("valid key");

        assert_eq!(
            key.qualified_name(),
            "compiler.optimization.level"
        );
    }

    #[test]
    fn marker_attribute_is_supported() {
        let attribute =
            Attribute::zamani_marker(
                AttributeId::new(1),
                "native",
            )
            .expect("valid attribute");

        assert!(attribute.is_marker());
        assert_eq!(
            attribute.value_kind(),
            AttributeValueKind::Unit
        );

        assert_eq!(
            attribute.to_string(),
            "@zamani.native = unit"
        );
    }

    #[test]
    fn finite_float_is_supported() {
        let value =
            AttributeValue::float(0.3)
                .expect("finite");

        assert_eq!(
            value.kind(),
            AttributeValueKind::Float
        );
    }

    #[test]
    fn non_finite_float_is_rejected() {
        assert_eq!(
            AttributeValue::float(f64::NAN),
            Err(
                AttributeError::NonFiniteFloat
            )
        );

        assert_eq!(
            AttributeValue::float(
                f64::INFINITY,
            ),
            Err(
                AttributeError::NonFiniteFloat
            )
        );

        assert_eq!(
            AttributeValue::float(
                f64::NEG_INFINITY,
            ),
            Err(
                AttributeError::NonFiniteFloat
            )
        );
    }

    #[test]
    fn float_zero_sign_is_deterministic() {
        let positive =
            FiniteAttributeFloat::new(
                0.0,
            )
            .expect("finite");

        let negative =
            FiniteAttributeFloat::new(
                -0.0,
            )
            .expect("finite");

        assert_ne!(
            positive,
            negative
        );
    }

    #[test]
    fn quantum_references_use_canonical_qubit_types() {
        let logical =
            AttributeValue::Qubit(
                QubitId::new(17),
            );

        let physical =
            AttributeValue::PhysicalQubit(
                PhysicalQubitId::new(42),
            );

        assert_eq!(
            logical.kind(),
            AttributeValueKind::Qubit
        );

        assert_eq!(
            physical.kind(),
            AttributeValueKind::PhysicalQubit
        );

        assert_eq!(
            logical.canonical_string()
                .expect("canonical"),
            "qubit(q17)"
        );

        assert_eq!(
            physical.canonical_string()
                .expect("canonical"),
            "physical_qubit(p42)"
        );
    }

    #[test]
    fn attributes_are_keyed_by_semantic_identity() {
        let key =
            AttributeKey::zamani(
                "native",
            )
            .expect("valid key");

        let first =
            Attribute::new(
                AttributeId::new(1),
                key.clone(),
                AttributeValue::Unit,
            )
            .expect("valid");

        let second =
            Attribute::new(
                AttributeId::new(2),
                key,
                AttributeValue::Unit,
            )
            .expect("valid");

        let mut attributes =
            Attributes::new();

        assert_eq!(
            attributes
                .insert_semantic(first)
                .expect("insert"),
            SemanticInsertResult::Inserted
        );

        assert_eq!(
            attributes
                .insert_semantic(second)
                .expect("semantic duplicate"),
            SemanticInsertResult::AlreadyPresent
        );

        assert_eq!(
            attributes.len(),
            1
        );
    }

    #[test]
    fn conflicting_values_are_rejected() {
        let key =
            AttributeKey::zamani(
                "native",
            )
            .expect("valid key");

        let first =
            Attribute::new(
                AttributeId::new(1),
                key.clone(),
                AttributeValue::Bool(true),
            )
            .expect("valid");

        let second =
            Attribute::new(
                AttributeId::new(2),
                key,
                AttributeValue::Bool(false),
            )
            .expect("valid");

        let mut attributes =
            Attributes::new();

        attributes
            .insert_semantic(first)
            .expect("insert");

        assert!(matches!(
            attributes.insert_semantic(second),
            Err(
                AttributeError::ConflictingAttribute { .. }
            )
        ));
    }

    #[test]
    fn explicit_replacement_is_available() {
        let key =
            AttributeKey::zamani(
                "optimization_level",
            )
            .expect("valid key");

        let first =
            Attribute::new(
                AttributeId::new(1),
                key.clone(),
                AttributeValue::UnsignedInteger(1),
            )
            .expect("valid");

        let second =
            Attribute::new(
                AttributeId::new(2),
                key,
                AttributeValue::UnsignedInteger(2),
            )
            .expect("valid");

        let mut attributes =
            Attributes::new();

        attributes
            .insert(first)
            .expect("insert");

        let replaced =
            attributes
                .insert_or_replace(second)
                .expect("replace");

        assert!(replaced.is_some());

        assert_eq!(
            attributes
                .iter()
                .next()
                .expect("attribute")
                .value(),
            &AttributeValue::UnsignedInteger(2)
        );
    }

    #[test]
    fn nested_metadata_is_validated() {
        let value =
            AttributeValue::array(vec![
                AttributeValue::Bool(true),
                AttributeValue::tuple(vec![
                    AttributeValue::Integer(-1),
                    AttributeValue::UnsignedInteger(2),
                ]),
                AttributeValue::optional(
                    Some(
                        AttributeValue::string(
                            "ok",
                        ),
                    ),
                ),
            ]);

        assert!(
            value.validate().is_ok()
        );
    }

    #[test]
    fn map_is_deterministically_ordered() {
        let mut map =
            BTreeMap::new();

        map.insert(
            "z".to_owned(),
            AttributeValue::Bool(true),
        );

        map.insert(
            "a".to_owned(),
            AttributeValue::Bool(false),
        );

        let value =
            AttributeValue::Map(map);

        assert_eq!(
            value.canonical_string()
                .expect("canonical"),
            "{\"a\":false,\"z\":true}"
        );
    }

    #[test]
    fn canonical_nested_rendering_is_stable() {
        let value =
            AttributeValue::array(vec![
                AttributeValue::Integer(1),
                AttributeValue::optional(
                    Some(
                        AttributeValue::string(
                            "x",
                        ),
                    ),
                ),
                AttributeValue::Map({
                    let mut map =
                        BTreeMap::new();

                    map.insert(
                        "b".to_owned(),
                        AttributeValue::Bool(true),
                    );

                    map.insert(
                        "a".to_owned(),
                        AttributeValue::Bool(false),
                    );

                    map
                }),
            ]);

        assert_eq!(
            value.canonical_string()
                .expect("canonical"),
            "[1,some(\"x\"),{\"a\":false,\"b\":true}]"
        );
    }

    #[test]
    fn opaque_bytes_are_deterministic() {
        let value =
            AttributeValue::bytes(vec![
                0x00,
                0x01,
                0xab,
                0xff,
            ]);

        assert_eq!(
            value.canonical_string()
                .expect("canonical"),
            "bytes(0001abff)"
        );
    }

    #[test]
    fn estimated_size_is_checked() {
        let value =
            AttributeValue::array(vec![
                AttributeValue::string(
                    "pulse",
                ),
                AttributeValue::UnsignedInteger(
                    20,
                ),
            ]);

        assert!(
            value.estimated_size().is_ok()
        );
    }

    #[test]
    fn attribute_display_is_canonical() {
        let attribute =
            Attribute::zamani(
                AttributeId::new(7),
                "unit",
                "radians",
            )
            .expect("valid");

        assert_eq!(
            attribute.to_string(),
            "@zamani.unit = \"radians\""
        );
    }

    #[test]
    fn attribute_collection_hash_is_deterministic() {
        use std::collections::hash_map::DefaultHasher;

        let attribute =
            Attribute::zamani(
                AttributeId::new(1),
                "native",
                true,
            )
            .expect("valid");

        let mut attributes =
            Attributes::new();

        attributes
            .insert(attribute)
            .expect("insert");

        let mut first =
            DefaultHasher::new();

        attributes.hash(&mut first);

        let mut second =
            DefaultHasher::new();

        attributes.hash(&mut second);

        assert_eq!(
            first.finish(),
            second.finish()
        );
    }
}