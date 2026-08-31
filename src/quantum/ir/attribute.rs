//! Zamani Quantum IR — Canonical Attribute System
//!
//! This module defines the canonical, hardware-independent attribute system
//! used by the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `attribute.rs` owns metadata attached to IR entities.
//!
//! Attributes provide structured, typed, extensible metadata without forcing
//! every new annotation or compiler concern into the core IR data structures.
//!
//! Examples include:
//!
//! ```text
//! @native
//! @experimental
//! @deprecated
//! @deterministic
//! @calibration("reference")
//! @unit("radians")
//! @semantic("logical")
//! ```
//!
//! Attributes are metadata. They do not change the fundamental ownership
//! boundaries of the IR.
//!
//! This module owns:
//!
//! - attribute identity;
//! - attribute names;
//! - attribute namespaces;
//! - attribute values;
//! - scalar metadata values;
//! - structured metadata values;
//! - references to canonical IR identities;
//! - ordered/deterministic attribute collections;
//! - duplicate detection;
//! - merge policy;
//! - structural validation;
//! - metadata size accounting;
//! - canonical textual representation;
//! - deterministic equality, ordering and hashing.
//!
//! It does NOT own:
//!
//! - gate semantics;
//! - measurement semantics;
//! - pulse generation;
//! - waveform generation;
//! - hardware capabilities;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - backend execution;
//! - frontend syntax;
//! - runtime state;
//! - calibration data itself;
//! - compiler pass implementation.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::identity
//!          │
//!          ▼
//!     attribute.rs
//!          │
//!     ┌────┼───────────────────────┐
//!     ▼    ▼                       ▼
//!   types  value                operations
//!     │    │                       │
//!     └────┴──────────┬────────────┘
//!                      ▼
//!                  program.rs
//! ```
//!
//! `attribute.rs` is intentionally independent of `operation.rs`,
//! `program.rs`, `hardware`, `routing`, `scheduling`, `optimization`, and
//! `frontend`.
//!
//! # Why attributes are separate from values
//!
//! An IR runtime value answers:
//!
//! > "What value does this program entity contain or reference?"
//!
//! An attribute answers:
//!
//! > "What metadata or declarative property is attached to this entity?"
//!
//! Keeping those concepts separate prevents metadata from becoming an
//! accidental second runtime/value system.
//!
//! The attribute value representation therefore intentionally supports the
//! metadata forms required by IR annotations while remaining independent from
//! `value.rs`.
//!
//! # Namespace model
//!
//! Attribute names are namespaced.
//!
//! Examples:
//!
//! ```text
//! zamani.native
//! zamani.experimental
//! compiler.optimization_level
//! hardware.target_hint
//! user.annotation
//! ```
//!
//! A namespace prevents unrelated producers from accidentally claiming the
//! same attribute name.
//!
//! The canonical identity of an attribute is:
//!
//! ```text
//! namespace + name
//! ```
//!
//! The `AttributeId` identifies an attribute occurrence/object; it does not
//! replace the namespace/name identity.
//!
//! # Reserved namespaces
//!
//! The IR defines the `zamani` namespace for core semantic annotations.
//!
//! Downstream systems may define their own namespaces.
//!
//! Hardware-specific namespaces are permitted as metadata, but their meaning
//! must not leak into the canonical semantic ownership of this module.
//!
//! For example:
//!
//! ```text
//! hardware.ibm.readout_mode
//! ```
//!
//! may exist as metadata, but `attribute.rs` does not interpret it.
//!
//! # Scalability
//!
//! There is no architectural maximum number of:
//!
//! - attributes;
//! - namespaces;
//! - attributes per entity;
//! - list elements;
//! - map entries;
//! - nesting depth;
//! - string length;
//! - metadata declarations.
//!
//! Concrete memory/security/resource limits are imposed by an external
//! `QuantumIrLimits` policy.
//!
//! This module therefore does NOT introduce:
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
//! A metadata collection may therefore be used with a tiny program or a
//! program containing extremely large numbers of quantum resources, subject
//! only to the explicit resource policy and available memory.
//!
//! # Quantum identity boundary
//!
//! Quantum identity remains owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Attribute values may reference those identities without defining duplicate
//! qubit identifier types.
//!
//! # Type identity boundary
//!
//! Declared IR types remain owned by:
//!
//! ```text
//! quantum::ir::identity::TypeId
//! ```
//!
//! Attribute metadata can reference a `TypeId`, but does not define types.
//!
//! # Value identity boundary
//!
//! IR values remain owned by:
//!
//! ```text
//! quantum::ir::identity::ValueId
//! ```
//!
//! Attribute metadata may reference an existing value without defining SSA,
//! dominance, use-def chains, or block semantics.
//!
//! # Parameter identity boundary
//!
//! Symbolic/runtime parameters remain owned by:
//!
//! ```text
//! quantum::ir::identity::ParameterId
//! ```
//!
//! Attribute metadata can reference parameters but does not evaluate them.
//!
//! # Determinism
//!
//! Deterministic behavior is essential for:
//!
//! - reproducible compilation;
//! - canonical hashing;
//! - caching;
//! - distributed compilation;
//! - provenance;
//! - benchmarking;
//! - IR comparison.
//!
//! Therefore:
//!
//! - attribute names are ordered;
//! - namespaces are ordered;
//! - map keys are ordered;
//! - equality is structural;
//! - hashing is structural;
//! - canonical formatting is deterministic.
//!
//! `BTreeMap` is deliberately used instead of `HashMap` for metadata maps.
//!
//! # Merge semantics
//!
//! Attributes may be combined from multiple compilation stages.
//!
//! The default merge operation is conservative:
//!
//! - identical attributes remain identical;
//! - distinct attributes are retained;
//! - conflicting values for the same namespace/name are rejected;
//! - no attribute is silently overwritten.
//!
//! This prevents an optimization pass from silently changing semantic
//! metadata emitted by an earlier stage.
//!
//! Explicit replacement can be implemented by the owning higher-level pass
//! when replacement is semantically justified.
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
//!
//! # Integration contract
//!
//! `identity.rs` supplies [`AttributeId`], [`TypeId`], [`ValueId`],
//! [`ParameterId`], and other stable identity types.
//!
//! `qubit.rs` supplies [`QubitId`] and [`PhysicalQubitId`] when attributes
//! reference logical or physical qubits.
//!
//! `types.rs` may consume attribute collections for type declarations, but
//! does not need to depend on this file.
//!
//! `value.rs` may coexist with attribute values without creating a dependency
//! cycle.
//!
//! `operation.rs` may attach attributes to operations.
//!
//! `gate.rs` may attach semantic gate metadata.
//!
//! `measurement.rs` may attach measurement metadata.
//!
//! `pulse.rs`, `waveform.rs`, `channel.rs`, and `frame.rs` may attach
//! hardware-control metadata without requiring this module to know hardware
//! implementation details.
//!
//! `region.rs` and `program.rs` may attach attributes to regions, blocks,
//! functions and programs.
//!
//! `serialization.rs` should serialize attributes structurally and preserve
//! namespace/name/value identity.
//!
//! `hash.rs` can hash attributes directly because their ordering and hashing
//! are deterministic.
//!
//! `provenance.rs` may use attributes for transformation metadata.
//!
//! `validation.rs` may validate attribute structure and policy.
//!
//! `analysis.rs` may inspect attributes but must not interpret unknown
//! namespaces as hardware semantics.
//!
//! No future quantum hardware technology should require changing this file
//! merely because a new device architecture is introduced.
//!
//! # Important compatibility rule
//!
//! This module does not depend on `types.rs` or `value.rs`.
//!
//! This keeps the foundational metadata contract independent and avoids a
//! dependency cycle:
//!
//! ```text
//! identity.rs ───────► attribute.rs
//!
//! types.rs ──────────► attribute.rs (optional consumer)
//! value.rs ──────────► attribute.rs (optional consumer)
//!
//! attribute.rs ──────X──► types.rs
//! attribute.rs ──────X──► value.rs
//! ```
//!
//! Higher-level IR modules may consume all of them independently.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::identity::{
    AttributeId,
    ParameterId,
    TypeId,
    ValueId,
};
use super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Constants
// =============================================================================

/// Canonical namespace for core Zamani IR attributes.
pub const ZAMANI_NAMESPACE: &str = "zamani";

/// Maximum byte value used by ASCII identifier validation.
///
/// This is a character classification constant, not a resource limit.
const ASCII_ZERO: u8 = b'0';
const ASCII_NINE: u8 = b'9';
const ASCII_UPPER_A: u8 = b'A';
const ASCII_UPPER_Z: u8 = b'Z';
const ASCII_LOWER_A: u8 = b'a';
const ASCII_LOWER_Z: u8 = b'z';
const ASCII_UNDERSCORE: u8 = b'_';
const ASCII_DOT: u8 = b'.';
const ASCII_DASH: u8 = b'-';

// =============================================================================
// Attribute name
// =============================================================================

/// A validated attribute namespace.
///
/// Namespaces are dot-separated identifiers.
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
        validate_qualified_name(&namespace, "namespace")?;

        Ok(Self(namespace))
    }

    /// Returns the namespace as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the namespace and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for AttributeNamespace {
    fn default() -> Self {
        Self(ZAMANI_NAMESPACE.to_owned())
    }
}

impl fmt::Display for AttributeNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for AttributeNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
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

/// A validated local attribute name.
///
/// Examples:
///
/// ```text
/// native
/// experimental
/// deterministic
/// calibration
/// optimization_level
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttributeName(String);

impl AttributeName {
    /// Creates a validated local attribute name.
    pub fn new<S>(name: S) -> Result<Self, AttributeError>
    where
        S: Into<String>,
    {
        let name = name.into();
        validate_identifier(&name, "attribute name")?;

        Ok(Self(name))
    }

    /// Returns the local attribute name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AttributeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for AttributeName {
    fn as_ref(&self) -> &str {
        self.as_str()
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

/// Canonical namespace/name identity of an attribute declaration.
///
/// This is distinct from [`AttributeId`].
///
/// `AttributeId` identifies an attribute occurrence/object.
///
/// `AttributeKey` identifies the semantic name under which the attribute is
/// stored.
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

    /// Creates a key in the canonical `zamani` namespace.
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
    ///
    /// The returned string is:
    ///
    /// ```text
    /// namespace.name
    /// ```
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(
                self.namespace.as_str().len()
                    + 1
                    + self.name.as_str().len(),
            );

        result.push_str(self.namespace.as_str());
        result.push(ASCII_DOT as char);
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
// Attribute value
// =============================================================================

/// A typed value suitable for IR metadata.
///
/// This is deliberately smaller than the canonical runtime [`Value`]
/// representation. Attributes are metadata and must not become a second
/// program-value system.
///
/// Complex program values should be referenced using [`AttributeValue::Value`]
/// instead of being copied into metadata.
///
/// Attribute values are fully structural and deterministic.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttributeValue {
    /// Boolean metadata.
    Bool(bool),

    /// Signed 128-bit metadata integer.
    ///
    /// Arbitrarily large application-level metadata can be represented by
    /// strings or external value references without imposing an IR-machine
    /// limit here.
    Integer(i128),

    /// Unsigned 128-bit metadata integer.
    UnsignedInteger(u128),

    /// Finite IEEE-754 binary64 metadata value.
    ///
    /// The constructor rejects NaN and infinity.
    Float(FiniteAttributeFloat),

    /// UTF-8 string metadata.
    String(String),

    /// A symbolic parameter reference.
    Parameter(ParameterId),

    /// A reference to an existing IR value.
    Value(ValueId),

    /// A logical qubit reference.
    Qubit(QubitId),

    /// A physical qubit reference.
    ///
    /// The attribute layer does not interpret the physical qubit or its
    /// hardware properties.
    PhysicalQubit(PhysicalQubitId),

    /// A declared IR type reference.
    Type(TypeId),

    /// An attribute identity reference.
    Attribute(AttributeId),

    /// A unit/empty metadata value.
    ///
    /// This is useful for marker attributes such as:
    ///
    /// ```text
    /// @native
    /// ```
    Unit,

    /// Ordered homogeneous metadata sequence.
    Array(Vec<Self>),

    /// Ordered heterogeneous metadata tuple.
    Tuple(Vec<Self>),

    /// Optional metadata value.
    Optional(Option<Box<Self>>),

    /// Deterministically ordered metadata object.
    ///
    /// Keys are plain strings rather than `AttributeName` because metadata
    /// object keys are values, not necessarily attribute identifiers.
    Map(BTreeMap<String, Self>),
}

impl AttributeValue {
    /// Creates a finite floating-point attribute value.
    pub fn float(value: f64) -> Result<Self, AttributeError> {
        Ok(Self::Float(
            FiniteAttributeFloat::new(value)?,
        ))
    }

    /// Creates a string attribute value.
    #[must_use]
    pub fn string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self::String(value.into())
    }

    /// Creates an array attribute value.
    #[must_use]
    pub fn array(values: Vec<Self>) -> Self {
        Self::Array(values)
    }

    /// Creates a tuple attribute value.
    #[must_use]
    pub fn tuple(values: Vec<Self>) -> Self {
        Self::Tuple(values)
    }

    /// Creates an optional attribute value.
    #[must_use]
    pub fn optional(value: Option<Self>) -> Self {
        Self::Optional(value.map(Box::new))
    }

    /// Creates an ordered map attribute value.
    #[must_use]
    pub fn map(values: BTreeMap<String, Self>) -> Self {
        Self::Map(values)
    }

    /// Returns the broad kind of this attribute value.
    #[must_use]
    pub const fn kind(&self) -> AttributeValueKind {
        match self {
            Self::Bool(_) => AttributeValueKind::Bool,
            Self::Integer(_) => AttributeValueKind::Integer,
            Self::UnsignedInteger(_) => {
                AttributeValueKind::UnsignedInteger
            }
            Self::Float(_) => AttributeValueKind::Float,
            Self::String(_) => AttributeValueKind::String,
            Self::Parameter(_) => AttributeValueKind::Parameter,
            Self::Value(_) => AttributeValueKind::Value,
            Self::Qubit(_) => AttributeValueKind::Qubit,
            Self::PhysicalQubit(_) => {
                AttributeValueKind::PhysicalQubit
            }
            Self::Type(_) => AttributeValueKind::Type,
            Self::Attribute(_) => AttributeValueKind::Attribute,
            Self::Unit => AttributeValueKind::Unit,
            Self::Array(_) => AttributeValueKind::Array,
            Self::Tuple(_) => AttributeValueKind::Tuple,
            Self::Optional(_) => AttributeValueKind::Optional,
            Self::Map(_) => AttributeValueKind::Map,
        }
    }

    /// Returns whether this value is scalar.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Bool(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::String(_)
                | Self::Parameter(_)
                | Self::Value(_)
                | Self::Qubit(_)
                | Self::PhysicalQubit(_)
                | Self::Type(_)
                | Self::Attribute(_)
                | Self::Unit
        )
    }

    /// Returns the approximate serialized memory footprint in bytes.
    ///
    /// This is a deterministic accounting helper for external resource
    /// policies. It is not a promise about the Rust allocator's exact memory
    /// usage.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        self.estimated_size_internal()
    }

    fn estimated_size_internal(&self) -> Result<u64, AttributeError> {
        fn add(
            left: u64,
            right: u64,
        ) -> Result<u64, AttributeError> {
            left.checked_add(right)
                .ok_or(AttributeError::SizeOverflow)
        }

        fn usize_to_u64(
            value: usize,
        ) -> Result<u64, AttributeError> {
            u64::try_from(value)
                .map_err(|_| AttributeError::SizeOverflow)
        }

        match self {
            Self::Bool(_) => Ok(1),

            Self::Integer(_) => Ok(16),

            Self::UnsignedInteger(_) => Ok(16),

            Self::Float(_) => Ok(8),

            Self::String(value) => {
                let length = usize_to_u64(value.len())?;
                add(8, length)
            }

            Self::Parameter(_)
            | Self::Value(_)
            | Self::Qubit(_)
            | Self::PhysicalQubit(_)
            | Self::Type(_)
            | Self::Attribute(_) => Ok(8),

            Self::Unit => Ok(0),

            Self::Array(values)
            | Self::Tuple(values) => {
                let mut total = 8_u64;

                for value in values {
                    total = add(
                        total,
                        value.estimated_size_internal()?,
                    )?;
                }

                Ok(total)
            }

            Self::Optional(value) => {
                let mut total = 1_u64;

                if let Some(value) = value {
                    total = add(
                        total,
                        value.estimated_size_internal()?,
                    )?;
                }

                Ok(total)
            }

            Self::Map(values) => {
                let mut total = 8_u64;

                for (key, value) in values {
                    total = add(
                        total,
                        usize_to_u64(key.len())?,
                    )?;

                    total = add(
                        total,
                        value.estimated_size_internal()?,
                    )?;
                }

                Ok(total)
            }
        }
    }

    /// Validates the structural invariants of this value.
    pub fn validate(&self) -> Result<(), AttributeError> {
        match self {
            Self::Float(value) => value.validate(),

            Self::String(_) => Ok(()),

            Self::Array(values)
            | Self::Tuple(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Optional(value) => {
                if let Some(value) = value {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Map(values) => {
                for (key, value) in values {
                    if key.is_empty() {
                        return Err(
                            AttributeError::EmptyMapKey
                        );
                    }

                    value.validate()?;
                }

                Ok(())
            }

            Self::Bool(_)
            | Self::Integer(_)
            | Self::UnsignedInteger(_)
            | Self::Parameter(_)
            | Self::Value(_)
            | Self::Qubit(_)
            | Self::PhysicalQubit(_)
            | Self::Type(_)
            | Self::Attribute(_)
            | Self::Unit => Ok(()),
        }
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
    /// Boolean.
    Bool,

    /// Signed integer.
    Integer,

    /// Unsigned integer.
    UnsignedInteger,

    /// Finite floating point.
    Float,

    /// String.
    String,

    /// Parameter reference.
    Parameter,

    /// Value reference.
    Value,

    /// Logical qubit reference.
    Qubit,

    /// Physical qubit reference.
    PhysicalQubit,

    /// Type reference.
    Type,

    /// Attribute reference.
    Attribute,

    /// Unit marker.
    Unit,

    /// Array.
    Array,

    /// Tuple.
    Tuple,

    /// Optional.
    Optional,

    /// Map.
    Map,
}

impl fmt::Display for AttributeValueKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Bool => "bool",
            Self::Integer => "integer",
            Self::UnsignedInteger => "unsigned_integer",
            Self::Float => "float",
            Self::String => "string",
            Self::Parameter => "parameter",
            Self::Value => "value",
            Self::Qubit => "qubit",
            Self::PhysicalQubit => "physical_qubit",
            Self::Type => "type",
            Self::Attribute => "attribute",
            Self::Unit => "unit",
            Self::Array => "array",
            Self::Tuple => "tuple",
            Self::Optional => "optional",
            Self::Map => "map",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Finite attribute float
// =============================================================================

/// A finite IEEE-754 binary64 value suitable for deterministic metadata.
///
/// NaN and infinities are rejected.
///
/// Equality and hashing use the IEEE-754 bit representation so metadata
/// equality remains deterministic.
#[derive(Clone, Copy, Debug)]
pub struct FiniteAttributeFloat(f64);

impl FiniteAttributeFloat {
    /// Creates a finite attribute float.
    pub fn new(value: f64) -> Result<Self, AttributeError> {
        if !value.is_finite() {
            return Err(
                AttributeError::NonFiniteFloat
            );
        }

        Ok(Self(value))
    }

    /// Returns the underlying `f64`.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns the IEEE-754 bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Validates this floating-point value.
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
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FiniteAttributeFloat {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect(
                "FiniteAttributeFloat contains only finite values"
            )
    }
}

// =============================================================================
// Attribute
// =============================================================================

/// One canonical IR attribute.
///
/// An attribute is identified by:
///
/// - [`AttributeId`] — occurrence/object identity;
/// - [`AttributeKey`] — semantic namespace/name;
/// - [`AttributeValue`] — optional typed metadata.
///
/// Marker attributes use [`AttributeValue::Unit`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Attribute {
    id: AttributeId,
    key: AttributeKey,
    value: AttributeValue,
}

impl Attribute {
    /// Creates a new attribute.
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

    /// Creates a marker attribute.
    pub fn marker(
        id: AttributeId,
        key: AttributeKey,
    ) -> Result<Self, AttributeError> {
        Self::new(
            id,
            key,
            AttributeValue::Unit,
        )
    }

    /// Creates a `zamani` marker attribute.
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

    /// Returns the attribute identity.
    #[must_use]
    pub const fn id(&self) -> AttributeId {
        self.id
    }

    /// Returns the attribute key.
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

    /// Returns the attribute value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Returns whether this is a marker attribute.
    #[must_use]
    pub fn is_marker(&self) -> bool {
        matches!(
            self.value,
            AttributeValue::Unit
        )
    }

    /// Validates this attribute.
    pub fn validate(&self) -> Result<(), AttributeError> {
        self.value.validate()
    }

    /// Returns a deterministic approximate size.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        let namespace =
            u64::try_from(self.namespace().as_str().len())
                .map_err(|_| AttributeError::SizeOverflow)?;

        let name =
            u64::try_from(self.name().as_str().len())
                .map_err(|_| AttributeError::SizeOverflow)?;

        let value = self.value.estimated_size()?;

        namespace
            .checked_add(name)
            .and_then(|size| size.checked_add(value))
            .and_then(|size| size.checked_add(8))
            .ok_or(AttributeError::SizeOverflow)
    }
}

impl fmt::Display for Attribute {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "@{} = {}",
            self.key,
            CanonicalAttributeValue(self.value())
        )
    }
}

// =============================================================================
// Attribute collection
// =============================================================================

/// Deterministic collection of attributes.
///
/// Attributes are indexed by semantic [`AttributeKey`].
///
/// This intentionally prevents two attributes with the same namespace/name
/// from silently coexisting with different values.
///
/// The collection is deterministic because it uses `BTreeMap`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attributes {
    values: BTreeMap<AttributeKey, Attribute>,
}

impl Attributes {
    /// Creates an empty attribute collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Returns the number of attributes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the collection contains no attributes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Inserts an attribute.
    ///
    /// If an attribute with the same semantic key already exists:
    ///
    /// - insertion succeeds as a no-op if the complete attribute is equal;
    /// - insertion fails if the values conflict.
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

    /// Inserts or explicitly replaces an attribute.
    ///
    /// This operation is intentionally separate from [`Self::insert`] so a
    /// compiler pass cannot accidentally overwrite metadata.
    pub fn insert_or_replace(
        &mut self,
        attribute: Attribute,
    ) -> Result<Option<Attribute>, AttributeError> {
        attribute.validate()?;

        let key = attribute.key().clone();

        Ok(self.values.insert(key, attribute))
    }

    /// Removes an attribute by semantic key.
    pub fn remove(
        &mut self,
        key: &AttributeKey,
    ) -> Option<Attribute> {
        self.values.remove(key)
    }

    /// Removes an attribute by namespace and name.
    pub fn remove_by_name(
        &mut self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<Attribute>, AttributeError> {
        let key = AttributeKey::new(
            namespace.to_owned(),
            name.to_owned(),
        )?;

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

    /// Returns an attribute by namespace and local name.
    pub fn get_by_name(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<&Attribute>, AttributeError> {
        let key = AttributeKey::new(
            namespace.to_owned(),
            name.to_owned(),
        )?;

        Ok(self.get(&key))
    }

    /// Returns a `zamani` namespace attribute by local name.
    pub fn get_zamani(
        &self,
        name: &str,
    ) -> Result<Option<&Attribute>, AttributeError> {
        self.get_by_name(ZAMANI_NAMESPACE, name)
    }

    /// Returns an iterator over attributes in deterministic order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Attribute> {
        self.values.values()
    }

    /// Returns an iterator over semantic keys in deterministic order.
    pub fn keys(
        &self,
    ) -> impl Iterator<Item = &AttributeKey> {
        self.values.keys()
    }

    /// Returns an iterator over `(key, attribute)` pairs.
    pub fn iter_keyed(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &Attribute)> {
        self.values.iter()
    }

    /// Clears the collection.
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
    /// Conflicting values are rejected.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), AttributeError> {
        for attribute in other.iter() {
            self.insert(attribute.clone())?;
        }

        Ok(())
    }

    /// Explicitly merges another collection with replacement semantics.
    pub fn merge_replace(
        &mut self,
        other: &Self,
    ) -> Result<(), AttributeError> {
        for attribute in other.iter() {
            self.insert_or_replace(attribute.clone())?;
        }

        Ok(())
    }

    /// Returns the approximate deterministic metadata size.
    pub fn estimated_size(&self) -> Result<u64, AttributeError> {
        let mut total = 8_u64;

        for attribute in self.values.values() {
            total = total
                .checked_add(attribute.estimated_size()?)
                .ok_or(AttributeError::SizeOverflow)?;
        }

        Ok(total)
    }

    /// Returns whether an attribute with the supplied key exists.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &AttributeKey,
    ) -> bool {
        self.values.contains_key(key)
    }

    /// Returns the underlying deterministic map.
    ///
    /// This is exposed as a read-only view so higher-level IR code can
    /// serialize or inspect attributes without taking ownership.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> &BTreeMap<AttributeKey, Attribute> {
        &self.values
    }

    /// Consumes the collection and returns the underlying map.
    #[must_use]
    pub fn into_map(
        self,
    ) -> BTreeMap<AttributeKey, Attribute> {
        self.values
    }

    /// Creates an attribute collection from an iterator.
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
// Attribute error
// =============================================================================

/// Errors produced by checked attribute construction and manipulation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeError {
    /// Namespace is empty.
    EmptyNamespace,

    /// Attribute name is empty.
    EmptyName,

    /// Namespace or name contains invalid syntax.
    InvalidName {
        /// Human-readable kind of identifier.
        kind: &'static str,

        /// Invalid name.
        value: String,
    },

    /// A namespace contains an empty segment.
    EmptyNamespaceSegment,

    /// A namespace segment is invalid.
    InvalidNamespaceSegment(String),

    /// A map metadata key is empty.
    EmptyMapKey,

    /// A floating-point metadata value is NaN or infinite.
    NonFiniteFloat,

    /// The same semantic attribute key was assigned conflicting values.
    ConflictingAttribute {
        /// Conflicting semantic key.
        key: AttributeKey,
    },

    /// An arithmetic operation overflowed while calculating metadata size.
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

            Self::InvalidName { kind, value } => {
                write!(
                    formatter,
                    "invalid {kind}: {value}"
                )
            }

            Self::EmptyNamespaceSegment => {
                formatter.write_str(
                    "attribute namespace contains an empty segment",
                )
            }

            Self::InvalidNamespaceSegment(segment) => {
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
                    "attribute floating-point value must be finite",
                )
            }

            Self::ConflictingAttribute { key } => {
                write!(
                    formatter,
                    "conflicting attribute values for {key}"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "attribute metadata size calculation overflowed",
                )
            }
        }
    }
}

impl std::error::Error for AttributeError {}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a local identifier.
fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(
            if kind == "attribute name" {
                AttributeError::EmptyName
            } else {
                AttributeError::InvalidName {
                    kind,
                    value: value.to_owned(),
                }
            }
        );
    }

    let bytes = value.as_bytes();

    for (index, byte) in bytes.iter().copied().enumerate() {
        let valid_start =
            is_ascii_letter(byte)
                || byte == ASCII_UNDERSCORE;

        let valid_continue =
            valid_start
                || is_ascii_digit(byte)
                || byte == ASCII_DASH;

        if index == 0 {
            if !valid_start {
                return Err(
                    AttributeError::InvalidName {
                        kind,
                        value: value.to_owned(),
                    }
                );
            }
        } else if !valid_continue {
            return Err(
                AttributeError::InvalidName {
                    kind,
                    value: value.to_owned(),
                }
            );
        }
    }

    Ok(())
}

/// Validates a qualified namespace.
fn validate_qualified_name(
    value: &str,
    kind: &'static str,
) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(
            AttributeError::EmptyNamespace
        );
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(
                AttributeError::EmptyNamespaceSegment
            );
        }

        validate_identifier(
            segment,
            kind,
        )
        .map_err(|_| {
            AttributeError::InvalidNamespaceSegment(
                segment.to_owned()
            )
        })?;
    }

    Ok(())
}

/// Returns whether an ASCII byte is a letter.
#[inline]
const fn is_ascii_letter(
    byte: u8,
) -> bool {
    (byte >= ASCII_LOWER_A
        && byte <= ASCII_LOWER_Z)
        || (byte >= ASCII_UPPER_A
            && byte <= ASCII_UPPER_Z)
}

/// Returns whether an ASCII byte is a decimal digit.
#[inline]
const fn is_ascii_digit(
    byte: u8,
) -> bool {
    byte >= ASCII_ZERO
        && byte <= ASCII_NINE
}

// =============================================================================
// Canonical formatting
// =============================================================================

/// Canonical deterministic attribute-value formatter.
struct CanonicalAttributeValue<'a>(
    &'a AttributeValue,
);

impl fmt::Display for CanonicalAttributeValue<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.0 {
            AttributeValue::Bool(value) => {
                write!(formatter, "{value}")
            }

            AttributeValue::Integer(value) => {
                write!(formatter, "{value}")
            }

            AttributeValue::UnsignedInteger(value) => {
                write!(formatter, "{value}u")
            }

            AttributeValue::Float(value) => {
                write!(
                    formatter,
                    "float_bits(0x{:016x})",
                    value.bits()
                )
            }

            AttributeValue::String(value) => {
                write_escaped_string(
                    formatter,
                    value,
                )
            }

            AttributeValue::Parameter(value) => {
                write!(formatter, "parameter({value})")
            }

            AttributeValue::Value(value) => {
                write!(formatter, "value({value})")
            }

            AttributeValue::Qubit(value) => {
                write!(formatter, "qubit({value})")
            }

            AttributeValue::PhysicalQubit(value) => {
                write!(
                    formatter,
                    "physical_qubit({value})"
                )
            }

            AttributeValue::Type(value) => {
                write!(formatter, "type({value})")
            }

            AttributeValue::Attribute(value) => {
                write!(
                    formatter,
                    "attribute({value})"
                )
            }

            AttributeValue::Unit => {
                formatter.write_str("unit")
            }

            AttributeValue::Array(values) => {
                formatter.write_str("[")?;

                for (index, value) in
                    values.iter().enumerate()
                {
                    if index != 0 {
                        formatter.write_str(",")?;
                    }

                    CanonicalAttributeValue(value)
                        .fmt(formatter)?;
                }

                formatter.write_str("]")
            }

            AttributeValue::Tuple(values) => {
                formatter.write_str("(")?;

                for (index, value) in
                    values.iter().enumerate()
                {
                    if index != 0 {
                        formatter.write_str(",")?;
                    }

                    CanonicalAttributeValue(value)
                        .fmt(formatter)?;
                }

                formatter.write_str(")")
            }

            AttributeValue::Optional(None) => {
                formatter.write_str("none")
            }

            AttributeValue::Optional(Some(value)) => {
                formatter.write_str("some(")?;

                CanonicalAttributeValue(value)
                    .fmt(formatter)?;

                formatter.write_str(")")
            }

            AttributeValue::Map(values) => {
                formatter.write_str("{")?;

                for (index, (key, value)) in
                    values.iter().enumerate()
                {
                    if index != 0 {
                        formatter.write_str(",")?;
                    }

                    write_escaped_string(
                        formatter,
                        key,
                    )?;

                    formatter.write_str(":")?;

                    CanonicalAttributeValue(value)
                        .fmt(formatter)?;
                }

                formatter.write_str("}")
            }
        }
    }
}

/// Writes a deterministic escaped string representation.
fn write_escaped_string(
    formatter: &mut fmt::Formatter<'_>,
    value: &str,
) -> fmt::Result {
    formatter.write_str("\"")?;

    for character in value.chars() {
        match character {
            '\\' => formatter.write_str("\\\\")?,
            '"' => formatter.write_str("\\\"")?,
            '\n' => formatter.write_str("\\n")?,
            '\r' => formatter.write_str("\\r")?,
            '\t' => formatter.write_str("\\t")?,

            character if character.is_control() => {
                write!(
                    formatter,
                    "\\u{{{:x}}}",
                    character as u32
                )?;
            }

            character => {
                formatter.write_str(
                    &character.to_string()
                )?;
            }
        }
    }

    formatter.write_str("\"")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_validation_accepts_qualified_names() {
        let namespace =
            AttributeNamespace::new(
                "compiler.optimization"
            )
            .expect("valid namespace");

        assert_eq!(
            namespace.as_str(),
            "compiler.optimization"
        );
    }

    #[test]
    fn namespace_validation_rejects_empty_segments() {
        let result =
            AttributeNamespace::new(
                "compiler..optimization"
            );

        assert_eq!(
            result,
            Err(
                AttributeError::EmptyNamespaceSegment
            )
        );
    }

    #[test]
    fn attribute_name_validation_rejects_invalid_names() {
        let result =
            AttributeName::new("123invalid");

        assert!(matches!(
            result,
            Err(AttributeError::InvalidName { .. })
        ));
    }

    #[test]
    fn attribute_key_is_deterministic() {
        let left =
            AttributeKey::new(
                "compiler",
                "optimization",
            )
            .expect("valid key");

        let right =
            AttributeKey::new(
                "compiler",
                "optimization",
            )
            .expect("valid key");

        assert_eq!(left, right);

        assert_eq!(
            left.qualified_name(),
            "compiler.optimization"
        );
    }

    #[test]
    fn marker_attribute_works() {
        let attribute =
            Attribute::zamani_marker(
                AttributeId::new(1),
                "native",
            )
            .expect("valid attribute");

        assert!(attribute.is_marker());
        assert_eq!(
            attribute.qualified_name(),
            "zamani.native"
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
            AttributeValue::float(f64::INFINITY),
            Err(
                AttributeError::NonFiniteFloat
            )
        );

        assert_eq!(
            AttributeValue::float(
                f64::NEG_INFINITY
            ),
            Err(
                AttributeError::NonFiniteFloat
            )
        );
    }

    #[test]
    fn finite_float_is_accepted() {
        let value =
            AttributeValue::float(0.3)
                .expect("finite float");

        assert_eq!(
            value.kind(),
            AttributeValueKind::Float
        );
    }

    #[test]
    fn attributes_reject_conflicts() {
        let key =
            AttributeKey::zamani("native")
                .expect("valid key");

        let first =
            Attribute::new(
                AttributeId::new(1),
                key.clone(),
                AttributeValue::Bool(true),
            )
            .expect("valid attribute");

        let second =
            Attribute::new(
                AttributeId::new(2),
                key,
                AttributeValue::Bool(false),
            )
            .expect("valid attribute");

        let mut attributes =
            Attributes::new();

        attributes
            .insert(first)
            .expect("first insertion");

        let result =
            attributes.insert(second);

        assert!(matches!(
            result,
            Err(
                AttributeError::ConflictingAttribute { .. }
            )
        ));
    }

    #[test]
    fn identical_attributes_are_idempotent() {
        let key =
            AttributeKey::zamani("native")
                .expect("valid key");

        let attribute =
            Attribute::new(
                AttributeId::new(1),
                key,
                AttributeValue::Bool(true),
            )
            .expect("valid attribute");

        let mut attributes =
            Attributes::new();

        assert!(
            attributes
                .insert(attribute.clone())
                .expect("insertion")
                .is_none()
        );

        assert!(
            attributes
                .insert(attribute)
                .expect("idempotent insertion")
                .is_none()
        );

        assert_eq!(
            attributes.len(),
            1
        );
    }

    #[test]
    fn explicit_replacement_is_supported() {
        let key =
            AttributeKey::zamani("level")
                .expect("valid key");

        let first =
            Attribute::new(
                AttributeId::new(1),
                key.clone(),
                AttributeValue::UnsignedInteger(1),
            )
            .expect("valid attribute");

        let second =
            Attribute::new(
                AttributeId::new(2),
                key,
                AttributeValue::UnsignedInteger(2),
            )
            .expect("valid attribute");

        let mut attributes =
            Attributes::new();

        attributes
            .insert(first)
            .expect("first insertion");

        let replaced =
            attributes
                .insert_or_replace(second)
                .expect("replacement");

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
    fn map_order_is_deterministic() {
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

        let formatted =
            format!(
                "{}",
                CanonicalAttributeValue(&value)
            );

        assert_eq!(
            formatted,
            "{\"a\":false,\"z\":true}"
        );
    }

    #[test]
    fn nested_values_validate() {
        let value =
            AttributeValue::array(vec![
                AttributeValue::Bool(true),
                AttributeValue::tuple(vec![
                    AttributeValue::Integer(-1),
                    AttributeValue::UnsignedInteger(2),
                ]),
                AttributeValue::optional(
                    Some(AttributeValue::String(
                        "ok".to_owned()
                    ))
                ),
            ]);

        assert!(
            value.validate().is_ok()
        );
    }

    #[test]
    fn quantum_identity_references_are_typed() {
        let logical =
            AttributeValue::Qubit(
                QubitId::new(100)
            );

        let physical =
            AttributeValue::PhysicalQubit(
                PhysicalQubitId::new(200)
            );

        assert_eq!(
            logical.kind(),
            AttributeValueKind::Qubit
        );

        assert_eq!(
            physical.kind(),
            AttributeValueKind::PhysicalQubit
        );
    }

    #[test]
    fn size_accounting_is_checked() {
        let value =
            AttributeValue::array(vec![
                AttributeValue::String(
                    "pulse".to_owned()
                ),
                AttributeValue::UnsignedInteger(20),
            ]);

        assert!(
            value.estimated_size().is_ok()
        );
    }

    #[test]
    fn deterministic_attribute_ordering() {
        let first_key =
            AttributeKey::new(
                "compiler",
                "zeta",
            )
            .expect("valid key");

        let second_key =
            AttributeKey::new(
                "compiler",
                "alpha",
            )
            .expect("valid key");

        assert!(
            second_key < first_key
        );
    }

    #[test]
    fn canonical_marker_format_is_stable() {
        let attribute =
            Attribute::zamani_marker(
                AttributeId::new(7),
                "native",
            )
            .expect("valid marker");

        assert_eq!(
            attribute.to_string(),
            "@zamani.native = unit"
        );
    }
}