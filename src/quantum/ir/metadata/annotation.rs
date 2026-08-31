//! Zamani Quantum IR — Annotations
//!
//! Production-grade, deterministic, extensible annotations for the canonical
//! Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! An annotation describes an explicitly attached declarative fact, intent,
//! hint, classification, or piece of metadata associated with an IR entity.
//!
//! An annotation answers:
//!
//! > "What additional declarative information has been attached to this IR
//! > entity?"
//!
//! It does NOT define:
//!
//! - quantum gate semantics;
//! - quantum-state semantics;
//! - hardware topology;
//! - hardware allocation;
//! - routing;
//! - scheduling;
//! - pulse synthesis;
//! - calibration execution;
//! - optimization algorithms;
//! - backend execution;
//! - source-language parsing;
//! - runtime state;
//! - authorization;
//! - cryptographic signatures.
//!
//! Those responsibilities belong to their respective IR or downstream
//! subsystems.
//!
//! # Relationship with `attribute.rs`
//!
//! `attribute.rs` owns the canonical typed attribute system.
//!
//! This module owns the higher-level concept of an annotation occurrence:
//!
//! ```text
//! Annotation
//!     │
//!     ├── identity
//!     ├── target
//!     ├── namespace/name
//!     ├── value
//!     ├── origin
//!     ├── applicability
//!     └── lifecycle
//! ```
//!
//! An annotation can therefore be used by higher-level metadata facilities
//! without requiring the annotation object itself to become the complete
//! attribute storage implementation.
//!
//! This separation is intentional.
//!
//! `Attribute` answers:
//!
//! > "What typed metadata value exists under this attribute key?"
//!
//! `Annotation` answers:
//!
//! > "What declarative annotation occurrence was attached, to what entity,
//! > with what origin and lifecycle semantics?"
//!
//! A future metadata container may associate annotations and attributes, but
//! this file deliberately does not depend on that future container.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are intended to be written once at the semantic
//! level and lowered to compatible targets of different sizes and
//! architectures.
//!
//! Consequently this module contains no quantum-machine size limit.
//!
//! In particular, it does not encode:
//!
//! ```text
//! 63
//! 64
//! 128
//! 4096
//! 1_000_000
//! ```
//!
//! as architectural limits.
//!
//! Annotation collections grow according to the actual program and the
//! explicitly supplied resource/security policy.
//!
//! # Quantum identity boundary
//!
//! Logical and physical qubit identities are owned exclusively by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file imports and references those types where an annotation targets a
//! qubit.
//!
//! It never defines a second qubit identifier type.
//!
//! # Determinism
//!
//! Determinism is required for:
//!
//! - reproducible compilation;
//! - canonical serialization;
//! - canonical hashing;
//! - distributed compilation;
//! - caching;
//! - benchmarking;
//! - provenance;
//! - IR comparison.
//!
//! Therefore:
//!
//! - annotation keys are structurally ordered;
//! - targets are structurally ordered;
//! - annotation values are structurally ordered;
//! - collections preserve deterministic ordering;
//! - canonical textual formatting is deterministic;
//! - no `HashMap` is used where semantic ordering matters;
//! - no wall-clock time is inserted automatically;
//! - no random identifier is generated internally.
//!
//! # Scalability
//!
//! There is no semantic fixed maximum for:
//!
//! - annotations;
//! - annotation namespaces;
//! - annotation values;
//! - collection elements;
//! - target references;
//! - nested metadata.
//!
//! Concrete limits belong to the IR resource/security policy layer.
//!
//! The data structures use dynamically sized standard-library collections.
//!
//! # Security
//!
//! Annotation data is untrusted input once it crosses a serialization,
//! frontend, plugin, or distributed compilation boundary.
//!
//! This module therefore:
//!
//! - validates identifiers;
//! - rejects empty required names;
//! - rejects malformed qualified names;
//! - distinguishes semantic targets from arbitrary textual targets;
//! - does not execute annotation contents;
//! - does not interpret arbitrary annotations as commands;
//! - does not contain credentials;
//! - does not contain authorization state;
//! - does not perform dynamic code loading.
//!
//! An annotation is metadata, never executable code.
//!
//! # Unknown annotations
//!
//! Unknown annotations MUST be representable and preservable.
//!
//! A consumer may choose to interpret a namespace it understands while
//! preserving annotations it does not understand.
//!
//! Unknown annotations must never silently become executable behavior.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - annotation identity;
//! - annotation namespace/name;
//! - annotation values;
//! - annotation targets;
//! - annotation origin;
//! - lifecycle state;
//! - applicability;
//! - deterministic annotation collections;
//! - validation of annotation-local invariants;
//! - deterministic canonical representation.
//!
//! It does NOT own:
//!
//! - attribute storage;
//! - IR object storage;
//! - program ownership;
//! - operation ownership;
//! - source maps;
//! - provenance chains;
//! - hardware capabilities;
//! - resource allocation.
//!
//! # Integration contract
//!
//! `identity.rs` provides [`AnnotationId`] and other stable identity types.
//!
//! `qubit.rs` provides [`QubitId`] and [`PhysicalQubitId`].
//!
//! `attribute.rs` may convert or associate attributes with annotations at a
//! higher layer, but this file does not depend on `attribute.rs`.
//!
//! `provenance.rs` may record the origin of annotation-producing
//! transformations, but annotations do not own provenance chains.
//!
//! `program.rs`, `operation.rs`, `region.rs`, `gate.rs`, `measurement.rs`,
//! `pulse.rs`, and other IR modules may attach annotation collections.
//!
//! `serialization.rs` serializes annotations structurally.
//!
//! `hash.rs` may hash the deterministic semantic form of annotations.
//!
//! `validation.rs` may invoke [`Annotation::validate`] and
//! [`AnnotationSet::validate`].
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
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Important design rule
//!
//! An annotation may describe a physical resource, but that does not make the
//! physical resource part of the canonical semantic program.
//!
//! For example:
//!
//! ```text
//! target = physical qubit 17
//! ```
//!
//! can be represented as metadata when a downstream mapping artifact needs to
//! document it, while the canonical logical computation remains independent
//! of that physical assignment.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::super::identity::{
    AnnotationId,
    AttributeId,
    BlockId,
    CircuitId,
    FunctionId,
    ModuleId,
    OperationId,
    ParameterId,
    ProgramId,
    RegionId,
    ResourceId,
    TypeId,
    ValueId,
};

use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Constants
// =============================================================================

/// Canonical namespace for core Zamani annotations.
pub const ZAMANI_ANNOTATION_NAMESPACE: &str = "zamani";

/// Maximum ASCII byte value used by identifier classification.
///
/// This is a lexical classification constant, not a resource limit.
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
// Result
// =============================================================================

/// Result type used by the annotation module.
pub type AnnotationResult<T> = Result<T, AnnotationError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing, validating, or manipulating
/// annotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationError {
    /// A required string was empty.
    EmptyField {
        /// Field that was empty.
        field: &'static str,
    },

    /// A name contained an invalid character.
    InvalidIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Supplied value.
        value: String,
    },

    /// A qualified namespace was malformed.
    InvalidNamespace {
        /// Supplied namespace.
        value: String,
    },

    /// Two annotations with the same semantic key conflict.
    ConflictingAnnotation {
        /// Namespace.
        namespace: String,

        /// Local name.
        name: String,
    },

    /// An annotation was attempted on a target it cannot apply to.
    InvalidTarget {
        /// Annotation's qualified name.
        annotation: String,

        /// Target description.
        target: String,
    },

    /// An annotation contained an invalid value.
    InvalidValue {
        /// Annotation's qualified name.
        annotation: String,

        /// Explanation.
        message: String,
    },

    /// An annotation lifecycle transition was invalid.
    InvalidLifecycleTransition {
        /// Current lifecycle.
        current: AnnotationLifecycle,

        /// Requested lifecycle.
        requested: AnnotationLifecycle,
    },

    /// The requested annotation did not exist.
    NotFound {
        /// Namespace.
        namespace: String,

        /// Name.
        name: String,
    },
}

impl fmt::Display for AnnotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "annotation field `{field}` must not be empty"
                )
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "invalid annotation {field} `{value}`"
                )
            }

            Self::InvalidNamespace { value } => {
                write!(
                    formatter,
                    "invalid annotation namespace `{value}`"
                )
            }

            Self::ConflictingAnnotation { namespace, name } => {
                write!(
                    formatter,
                    "conflicting annotation `{namespace}.{name}`"
                )
            }

            Self::InvalidTarget {
                annotation,
                target,
            } => {
                write!(
                    formatter,
                    "annotation `{annotation}` cannot target `{target}`"
                )
            }

            Self::InvalidValue {
                annotation,
                message,
            } => {
                write!(
                    formatter,
                    "invalid value for annotation `{annotation}`: {message}"
                )
            }

            Self::InvalidLifecycleTransition {
                current,
                requested,
            } => {
                write!(
                    formatter,
                    "invalid annotation lifecycle transition from `{current}` to `{requested}`"
                )
            }

            Self::NotFound { namespace, name } => {
                write!(
                    formatter,
                    "annotation `{namespace}.{name}` was not found"
                )
            }
        }
    }
}

impl std::error::Error for AnnotationError {}

// =============================================================================
// Annotation namespace
// =============================================================================

/// Validated annotation namespace.
///
/// Namespaces are dot-separated ASCII identifiers.
///
/// Examples:
///
/// ```text
/// zamani
/// compiler
/// compiler.optimization
/// hardware
/// hardware.target
/// vendor.example
/// user
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnnotationNamespace(String);

impl AnnotationNamespace {
    /// Creates a validated namespace.
    pub fn new<S>(value: S) -> AnnotationResult<Self>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_namespace(&value)?;

        Ok(Self(value))
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

impl Default for AnnotationNamespace {
    fn default() -> Self {
        Self(ZAMANI_ANNOTATION_NAMESPACE.to_owned())
    }
}

impl fmt::Display for AnnotationNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for AnnotationNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for AnnotationNamespace {
    type Error = AnnotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for AnnotationNamespace {
    type Error = AnnotationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Annotation name
// =============================================================================

/// Validated local annotation name.
///
/// Examples:
///
/// ```text
/// native
/// experimental
/// deprecated
/// deterministic
/// optimization_hint
/// source
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnnotationName(String);

impl AnnotationName {
    /// Creates a validated local annotation name.
    pub fn new<S>(value: S) -> AnnotationResult<Self>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_identifier(&value, "name")?;

        Ok(Self(value))
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

impl fmt::Display for AnnotationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for AnnotationName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for AnnotationName {
    type Error = AnnotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for AnnotationName {
    type Error = AnnotationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Annotation key
// =============================================================================

/// Canonical semantic key of an annotation.
///
/// The key consists of:
///
/// ```text
/// namespace + name
/// ```
///
/// It is deliberately independent from [`AnnotationId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnnotationKey {
    namespace: AnnotationNamespace,
    name: AnnotationName,
}

impl AnnotationKey {
    /// Creates a validated annotation key.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> AnnotationResult<Self>
    where
        N: Into<String>,
        S: Into<String>,
    {
        Ok(Self {
            namespace: AnnotationNamespace::new(namespace)?,
            name: AnnotationName::new(name)?,
        })
    }

    /// Creates a key in the canonical Zamani namespace.
    pub fn zamani<S>(name: S) -> AnnotationResult<Self>
    where
        S: Into<String>,
    {
        Self::new(ZAMANI_ANNOTATION_NAMESPACE, name)
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &AnnotationNamespace {
        &self.namespace
    }

    /// Returns the local name.
    #[must_use]
    pub fn name(&self) -> &AnnotationName {
        &self.name
    }

    /// Returns the fully qualified annotation name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut output = String::with_capacity(
            self.namespace.as_str().len()
                + 1
                + self.name.as_str().len(),
        );

        output.push_str(self.namespace.as_str());
        output.push('.');
        output.push_str(self.name.as_str());

        output
    }
}

impl fmt::Display for AnnotationKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())
    }
}

// =============================================================================
// Annotation target
// =============================================================================

/// Semantic IR entity to which an annotation may be attached.
///
/// The target model deliberately uses canonical IR identity types.
///
/// In particular, qubits use:
///
/// ```text
/// quantum::ir::qubit::QubitId
/// quantum::ir::qubit::PhysicalQubitId
/// ```
///
/// rather than defining duplicate identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnnotationTarget {
    /// Complete quantum program.
    Program(ProgramId),

    /// IR module.
    Module(ModuleId),

    /// Quantum circuit.
    Circuit(CircuitId),

    /// Function/subroutine.
    Function(FunctionId),

    /// Structured region.
    Region(RegionId),

    /// Block.
    Block(BlockId),

    /// Operation.
    Operation(OperationId),

    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// IR value.
    Value(ValueId),

    /// Symbolic/runtime parameter.
    Parameter(ParameterId),

    /// IR type declaration.
    Type(TypeId),

    /// Abstract resource.
    Resource(ResourceId),

    /// Attribute occurrence.
    Attribute(AttributeId),

    /// Annotation occurrence itself.
    Annotation(AnnotationId),
}

impl AnnotationTarget {
    /// Returns a stable target category name.
    #[must_use]
    pub const fn kind(&self) -> AnnotationTargetKind {
        match self {
            Self::Program(_) => AnnotationTargetKind::Program,
            Self::Module(_) => AnnotationTargetKind::Module,
            Self::Circuit(_) => AnnotationTargetKind::Circuit,
            Self::Function(_) => AnnotationTargetKind::Function,
            Self::Region(_) => AnnotationTargetKind::Region,
            Self::Block(_) => AnnotationTargetKind::Block,
            Self::Operation(_) => AnnotationTargetKind::Operation,
            Self::LogicalQubit(_) => AnnotationTargetKind::LogicalQubit,
            Self::PhysicalQubit(_) => AnnotationTargetKind::PhysicalQubit,
            Self::Value(_) => AnnotationTargetKind::Value,
            Self::Parameter(_) => AnnotationTargetKind::Parameter,
            Self::Type(_) => AnnotationTargetKind::Type,
            Self::Resource(_) => AnnotationTargetKind::Resource,
            Self::Attribute(_) => AnnotationTargetKind::Attribute,
            Self::Annotation(_) => AnnotationTargetKind::Annotation,
        }
    }

    /// Returns whether this is a logical quantum target.
    #[must_use]
    pub const fn is_logical_quantum_target(&self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this is a physical quantum target.
    #[must_use]
    pub const fn is_physical_quantum_target(&self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns a deterministic human-readable target description.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Program(id) => format!("program:{id}"),
            Self::Module(id) => format!("module:{id}"),
            Self::Circuit(id) => format!("circuit:{id}"),
            Self::Function(id) => format!("function:{id}"),
            Self::Region(id) => format!("region:{id}"),
            Self::Block(id) => format!("block:{id}"),
            Self::Operation(id) => format!("operation:{id}"),
            Self::LogicalQubit(id) => format!("logical-qubit:{id}"),
            Self::PhysicalQubit(id) => format!("physical-qubit:{id}"),
            Self::Value(id) => format!("value:{id}"),
            Self::Parameter(id) => format!("parameter:{id}"),
            Self::Type(id) => format!("type:{id}"),
            Self::Resource(id) => format!("resource:{id}"),
            Self::Attribute(id) => format!("attribute:{id}"),
            Self::Annotation(id) => format!("annotation:{id}"),
        }
    }
}

impl fmt::Display for AnnotationTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.describe())
    }
}

/// Stable target category.
///
/// This is intentionally a closed classification of the target kinds known
/// to this IR version. New IR entities can add new target variants without
/// changing the meaning of existing variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnnotationTargetKind {
    /// Program.
    Program,

    /// Module.
    Module,

    /// Circuit.
    Circuit,

    /// Function.
    Function,

    /// Region.
    Region,

    /// Block.
    Block,

    /// Operation.
    Operation,

    /// Logical qubit.
    LogicalQubit,

    /// Physical qubit.
    PhysicalQubit,

    /// Value.
    Value,

    /// Parameter.
    Parameter,

    /// Type.
    Type,

    /// Resource.
    Resource,

    /// Attribute.
    Attribute,

    /// Annotation.
    Annotation,
}

impl fmt::Display for AnnotationTargetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Program => "program",
            Self::Module => "module",
            Self::Circuit => "circuit",
            Self::Function => "function",
            Self::Region => "region",
            Self::Block => "block",
            Self::Operation => "operation",
            Self::LogicalQubit => "logical-qubit",
            Self::PhysicalQubit => "physical-qubit",
            Self::Value => "value",
            Self::Parameter => "parameter",
            Self::Type => "type",
            Self::Resource => "resource",
            Self::Attribute => "attribute",
            Self::Annotation => "annotation",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Annotation value
// =============================================================================

/// Scalar and structured annotation metadata value.
///
/// This deliberately does not reuse `value.rs`.
///
/// Runtime values and metadata values are different architectural concepts.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationValue {
    /// Explicit null/unit-like value.
    Unit,

    /// Boolean value.
    Boolean(bool),

    /// Signed integer.
    Integer(i128),

    /// Unsigned integer.
    UnsignedInteger(u128),

    /// UTF-8 string.
    String(String),

    /// Symbolic reference represented by a stable identifier.
    Symbol(String),

    /// Logical qubit reference.
    LogicalQubit(QubitId),

    /// Physical qubit reference.
    PhysicalQubit(PhysicalQubitId),

    /// IR value reference.
    Value(ValueId),

    /// Parameter reference.
    Parameter(ParameterId),

    /// Type reference.
    Type(TypeId),

    /// Nested sequence.
    List(Vec<Self>),

    /// Deterministically ordered key/value metadata.
    Map(BTreeMap<String, Self>),

    /// Opaque UTF-8-safe semantic payload.
    ///
    /// The bytes are retained exactly and are never executed.
    Bytes(Vec<u8>),
}

impl AnnotationValue {
    /// Returns the value's stable kind.
    #[must_use]
    pub const fn kind(&self) -> AnnotationValueKind {
        match self {
            Self::Unit => AnnotationValueKind::Unit,
            Self::Boolean(_) => AnnotationValueKind::Boolean,
            Self::Integer(_) => AnnotationValueKind::Integer,
            Self::UnsignedInteger(_) => AnnotationValueKind::UnsignedInteger,
            Self::String(_) => AnnotationValueKind::String,
            Self::Symbol(_) => AnnotationValueKind::Symbol,
            Self::LogicalQubit(_) => AnnotationValueKind::LogicalQubit,
            Self::PhysicalQubit(_) => AnnotationValueKind::PhysicalQubit,
            Self::Value(_) => AnnotationValueKind::Value,
            Self::Parameter(_) => AnnotationValueKind::Parameter,
            Self::Type(_) => AnnotationValueKind::Type,
            Self::List(_) => AnnotationValueKind::List,
            Self::Map(_) => AnnotationValueKind::Map,
            Self::Bytes(_) => AnnotationValueKind::Bytes,
        }
    }

    /// Returns whether the value is semantically scalar.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Unit
                | Self::Boolean(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::String(_)
                | Self::Symbol(_)
                | Self::LogicalQubit(_)
                | Self::PhysicalQubit(_)
                | Self::Value(_)
                | Self::Parameter(_)
                | Self::Type(_)
        )
    }

    /// Returns whether the value is a structured collection.
    #[must_use]
    pub const fn is_collection(&self) -> bool {
        matches!(self, Self::List(_) | Self::Map(_))
    }

    /// Creates an empty metadata map.
    #[must_use]
    pub fn empty_map() -> Self {
        Self::Map(BTreeMap::new())
    }

    /// Validates recursively.
    pub fn validate(&self) -> AnnotationResult<()> {
        match self {
            Self::Unit
            | Self::Boolean(_)
            | Self::Integer(_)
            | Self::UnsignedInteger(_)
            | Self::LogicalQubit(_)
            | Self::PhysicalQubit(_)
            | Self::Value(_)
            | Self::Parameter(_)
            | Self::Type(_) => Ok(()),

            Self::String(value) | Self::Symbol(value) => {
                if value.is_empty() {
                    return Err(AnnotationError::EmptyField {
                        field: "annotation value",
                    });
                }

                Ok(())
            }

            Self::List(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_identifier(key, "annotation map key")?;
                    value.validate()?;
                }

                Ok(())
            }

            Self::Bytes(_) => Ok(()),
        }
    }
}

/// Stable classification of annotation values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnnotationValueKind {
    /// Unit.
    Unit,

    /// Boolean.
    Boolean,

    /// Signed integer.
    Integer,

    /// Unsigned integer.
    UnsignedInteger,

    /// String.
    String,

    /// Symbol.
    Symbol,

    /// Logical qubit.
    LogicalQubit,

    /// Physical qubit.
    PhysicalQubit,

    /// IR value reference.
    Value,

    /// Parameter reference.
    Parameter,

    /// Type reference.
    Type,

    /// List.
    List,

    /// Map.
    Map,

    /// Opaque bytes.
    Bytes,
}

impl fmt::Display for AnnotationValueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Unit => "unit",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::UnsignedInteger => "unsigned-integer",
            Self::String => "string",
            Self::Symbol => "symbol",
            Self::LogicalQubit => "logical-qubit",
            Self::PhysicalQubit => "physical-qubit",
            Self::Value => "value",
            Self::Parameter => "parameter",
            Self::Type => "type",
            Self::List => "list",
            Self::Map => "map",
            Self::Bytes => "bytes",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Annotation origin
// =============================================================================

/// Origin of an annotation.
///
/// Origin is descriptive metadata. It is not an authorization mechanism.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnnotationOrigin {
    /// Produced by the Zamani source/frontend.
    Source,

    /// Produced by an IR transformation.
    Compiler,

    /// Produced by an optimization pass.
    Optimization,

    /// Produced by routing/mapping.
    Routing,

    /// Produced by scheduling.
    Scheduling,

    /// Produced by target lowering.
    Lowering,

    /// Produced by a hardware adapter.
    Hardware,

    /// Produced by a backend.
    Backend,

    /// Produced by an external tool/plugin.
    External,

    /// Explicitly supplied by the user.
    User,

    /// Unknown or intentionally unspecified origin.
    Unknown,
}

impl fmt::Display for AnnotationOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Source => "source",
            Self::Compiler => "compiler",
            Self::Optimization => "optimization",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Lowering => "lowering",
            Self::Hardware => "hardware",
            Self::Backend => "backend",
            Self::External => "external",
            Self::User => "user",
            Self::Unknown => "unknown",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Annotation lifecycle
// =============================================================================

/// Lifecycle of an annotation.
///
/// Lifecycle changes are explicit and deterministic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnnotationLifecycle {
    /// Annotation is active.
    Active,

    /// Annotation is experimental.
    Experimental,

    /// Annotation is deprecated but retained.
    Deprecated,

    /// Annotation is retained for compatibility only.
    Legacy,

    /// Annotation has been explicitly disabled but remains preserved.
    Disabled,
}

impl AnnotationLifecycle {
    /// Returns whether this annotation is semantically active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Active | Self::Experimental
        )
    }

    /// Returns whether this annotation is retained for compatibility.
    #[must_use]
    pub const fn is_compatibility_only(self) -> bool {
        matches!(
            self,
            Self::Legacy | Self::Deprecated
        )
    }

    /// Returns whether transitioning to `requested` is allowed.
    ///
    /// Lifecycle transitions are intentionally conservative.
    #[must_use]
    pub const fn can_transition_to(
        self,
        requested: Self,
    ) -> bool {
        match (self, requested) {
            (Self::Active, Self::Experimental)
            | (Self::Active, Self::Deprecated)
            | (Self::Active, Self::Disabled)
            | (Self::Experimental, Self::Active)
            | (Self::Experimental, Self::Deprecated)
            | (Self::Experimental, Self::Disabled)
            | (Self::Deprecated, Self::Legacy)
            | (Self::Deprecated, Self::Disabled)
            | (Self::Legacy, Self::Disabled)
            | (Self::Disabled, Self::Disabled)
            | (Self::Active, Self::Active)
            | (Self::Experimental, Self::Experimental)
            | (Self::Deprecated, Self::Deprecated)
            | (Self::Legacy, Self::Legacy) => true,

            // Re-enabling an intentionally disabled annotation requires
            // explicit reconstruction rather than a silent state mutation.
            (Self::Disabled, _) => false,

            // Legacy/deprecated annotations must not silently become active.
            (Self::Legacy, _)
            | (Self::Deprecated, Self::Active)
            | (Self::Deprecated, Self::Experimental) => false,
        }
    }
}

impl fmt::Display for AnnotationLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Active => "active",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Legacy => "legacy",
            Self::Disabled => "disabled",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Annotation applicability
// =============================================================================

/// Declares the IR target categories to which an annotation may apply.
///
/// An empty applicability set is invalid because it would make the annotation
/// semantically unscoped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotationApplicability {
    kinds: Vec<AnnotationTargetKind>,
}

impl AnnotationApplicability {
    /// Creates an applicability specification.
    ///
    /// Duplicate kinds are rejected so the representation has one canonical
    /// meaning.
    pub fn new(
        kinds: impl IntoIterator<Item = AnnotationTargetKind>,
    ) -> AnnotationResult<Self> {
        let mut kinds: Vec<_> = kinds.into_iter().collect();

        if kinds.is_empty() {
            return Err(AnnotationError::EmptyField {
                field: "annotation applicability",
            });
        }

        kinds.sort();
        kinds.dedup();

        Ok(Self { kinds })
    }

    /// Creates an applicability specification containing every currently
    /// defined target category.
    #[must_use]
    pub fn all() -> Self {
        Self {
            kinds: vec![
                AnnotationTargetKind::Program,
                AnnotationTargetKind::Module,
                AnnotationTargetKind::Circuit,
                AnnotationTargetKind::Function,
                AnnotationTargetKind::Region,
                AnnotationTargetKind::Block,
                AnnotationTargetKind::Operation,
                AnnotationTargetKind::LogicalQubit,
                AnnotationTargetKind::PhysicalQubit,
                AnnotationTargetKind::Value,
                AnnotationTargetKind::Parameter,
                AnnotationTargetKind::Type,
                AnnotationTargetKind::Resource,
                AnnotationTargetKind::Attribute,
                AnnotationTargetKind::Annotation,
            ],
        }
    }

    /// Returns all permitted target kinds.
    #[must_use]
    pub fn kinds(&self) -> &[AnnotationTargetKind] {
        &self.kinds
    }

    /// Returns whether the supplied target kind is permitted.
    #[must_use]
    pub fn allows(
        &self,
        kind: AnnotationTargetKind,
    ) -> bool {
        self.kinds.binary_search(&kind).is_ok()
    }

    /// Validates the applicability object.
    pub fn validate(&self) -> AnnotationResult<()> {
        if self.kinds.is_empty() {
            return Err(AnnotationError::EmptyField {
                field: "annotation applicability",
            });
        }

        for pair in self.kinds.windows(2) {
            if pair[0] >= pair[1] {
                return Err(AnnotationError::InvalidValue {
                    annotation: "annotation applicability".to_owned(),
                    message:
                        "target kinds must be unique and sorted"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Annotation
// =============================================================================

/// A complete annotation occurrence attached to an IR target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotation {
    id: AnnotationId,
    key: AnnotationKey,
    target: AnnotationTarget,
    value: AnnotationValue,
    origin: AnnotationOrigin,
    lifecycle: AnnotationLifecycle,
    applicability: Option<AnnotationApplicability>,
}

impl Annotation {
    /// Creates an annotation with active lifecycle and unknown origin.
    pub fn new(
        id: AnnotationId,
        key: AnnotationKey,
        target: AnnotationTarget,
        value: AnnotationValue,
    ) -> AnnotationResult<Self> {
        Self::with_metadata(
            id,
            key,
            target,
            value,
            AnnotationOrigin::Unknown,
            AnnotationLifecycle::Active,
            None,
        )
    }

    /// Creates an annotation with complete metadata.
    pub fn with_metadata(
        id: AnnotationId,
        key: AnnotationKey,
        target: AnnotationTarget,
        value: AnnotationValue,
        origin: AnnotationOrigin,
        lifecycle: AnnotationLifecycle,
        applicability: Option<AnnotationApplicability>,
    ) -> AnnotationResult<Self> {
        value.validate()?;

        if let Some(ref applicability) = applicability {
            applicability.validate()?;

            if !applicability.allows(target.kind()) {
                return Err(AnnotationError::InvalidTarget {
                    annotation: key.qualified_name(),
                    target: target.describe(),
                });
            }
        }

        Ok(Self {
            id,
            key,
            target,
            value,
            origin,
            lifecycle,
            applicability,
        })
    }

    /// Returns the stable annotation identity.
    #[must_use]
    pub const fn id(&self) -> AnnotationId {
        self.id
    }

    /// Returns the semantic annotation key.
    #[must_use]
    pub fn key(&self) -> &AnnotationKey {
        &self.key
    }

    /// Returns the fully qualified annotation name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.key.qualified_name()
    }

    /// Returns the annotation target.
    #[must_use]
    pub fn target(&self) -> &AnnotationTarget {
        &self.target
    }

    /// Returns the annotation value.
    #[must_use]
    pub fn value(&self) -> &AnnotationValue {
        &self.value
    }

    /// Returns the annotation origin.
    #[must_use]
    pub const fn origin(&self) -> &AnnotationOrigin {
        &self.origin
    }

    /// Returns the lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> AnnotationLifecycle {
        self.lifecycle
    }

    /// Returns the optional applicability specification.
    #[must_use]
    pub fn applicability(&self) -> Option<&AnnotationApplicability> {
        self.applicability.as_ref()
    }

    /// Returns whether this annotation is currently active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }

    /// Returns whether this annotation is compatibility-only metadata.
    #[must_use]
    pub const fn is_compatibility_only(&self) -> bool {
        self.lifecycle.is_compatibility_only()
    }

    /// Changes lifecycle explicitly.
    pub fn transition(
        &mut self,
        requested: AnnotationLifecycle,
    ) -> AnnotationResult<()> {
        if !self.lifecycle.can_transition_to(requested) {
            return Err(
                AnnotationError::InvalidLifecycleTransition {
                    current: self.lifecycle,
                    requested,
                },
            );
        }

        self.lifecycle = requested;
        Ok(())
    }

    /// Replaces the annotation value after validation.
    ///
    /// The annotation identity and target remain unchanged.
    pub fn replace_value(
        &mut self,
        value: AnnotationValue,
    ) -> AnnotationResult<()> {
        value.validate()?;
        self.value = value;
        Ok(())
    }

    /// Validates all annotation-local invariants.
    pub fn validate(&self) -> AnnotationResult<()> {
        self.value.validate()?;

        if let Some(applicability) = &self.applicability {
            applicability.validate()?;

            if !applicability.allows(self.target.kind()) {
                return Err(AnnotationError::InvalidTarget {
                    annotation: self.qualified_name(),
                    target: self.target.describe(),
                });
            }
        }

        Ok(())
    }

    /// Returns a deterministic canonical textual representation.
    ///
    /// This representation is intended for diagnostics and canonicalization
    /// support. It is not a wire format.
    #[must_use]
    pub fn canonical_text(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.qualified_name());
        output.push('@');
        output.push_str(&self.target.describe());
        output.push(':');
        append_value_canonical(&self.value, &mut output);

        output.push('|');
        output.push_str(self.origin.to_string().as_str());

        output.push('|');
        output.push_str(self.lifecycle.to_string().as_str());

        output
    }
}

// =============================================================================
// Annotation set
// =============================================================================

/// Deterministic collection of annotations.
///
/// The collection is keyed by semantic annotation key.
///
/// Two annotations with the same namespace/name cannot silently overwrite one
/// another.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AnnotationSet {
    entries: BTreeMap<AnnotationKey, Annotation>,
}

impl AnnotationSet {
    /// Creates an empty annotation set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Returns the number of annotations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts an annotation.
    ///
    /// Insertion is conservative:
    ///
    /// - a new key is inserted;
    /// - an identical annotation is accepted idempotently;
    /// - a different annotation with the same key is rejected.
    pub fn insert(
        &mut self,
        annotation: Annotation,
    ) -> AnnotationResult<()> {
        annotation.validate()?;

        let key = annotation.key().clone();

        match self.entries.get(&key) {
            None => {
                self.entries.insert(key, annotation);
                Ok(())
            }

            Some(existing) if existing == &annotation => Ok(()),

            Some(_) => Err(AnnotationError::ConflictingAnnotation {
                namespace: key.namespace().as_str().to_owned(),
                name: key.name().as_str().to_owned(),
            }),
        }
    }

    /// Returns an annotation by semantic key.
    #[must_use]
    pub fn get(
        &self,
        key: &AnnotationKey,
    ) -> Option<&Annotation> {
        self.entries.get(key)
    }

    /// Returns a mutable annotation by semantic key.
    pub fn get_mut(
        &mut self,
        key: &AnnotationKey,
    ) -> Option<&mut Annotation> {
        self.entries.get_mut(key)
    }

    /// Removes an annotation by semantic key.
    pub fn remove(
        &mut self,
        key: &AnnotationKey,
    ) -> Option<Annotation> {
        self.entries.remove(key)
    }

    /// Returns whether a key exists.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &AnnotationKey,
    ) -> bool {
        self.entries.contains_key(key)
    }

    /// Iterates over annotations in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AnnotationKey, &Annotation)> {
        self.entries.iter()
    }

    /// Returns annotations as a deterministic slice-like iterator.
    pub fn values(
        &self,
    ) -> impl Iterator<Item = &Annotation> {
        self.entries.values()
    }

    /// Returns mutable annotations in deterministic key order.
    pub fn values_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Annotation> {
        self.entries.values_mut()
    }

    /// Clears the complete set.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Merges another set conservatively.
    ///
    /// Existing conflicts are rejected and no partial merge is committed.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> AnnotationResult<()> {
        for annotation in other.values() {
            if let Some(existing) = self.get(annotation.key()) {
                if existing != annotation {
                    return Err(
                        AnnotationError::ConflictingAnnotation {
                            namespace: annotation
                                .key()
                                .namespace()
                                .as_str()
                                .to_owned(),
                            name: annotation
                                .key()
                                .name()
                                .as_str()
                                .to_owned(),
                        },
                    );
                }
            }
        }

        for annotation in other.values() {
            self.insert(annotation.clone())?;
        }

        Ok(())
    }

    /// Validates every annotation in the set.
    pub fn validate(&self) -> AnnotationResult<()> {
        for annotation in self.values() {
            annotation.validate()?;
        }

        Ok(())
    }

    /// Returns all annotations targeting a particular entity kind.
    #[must_use]
    pub fn by_target_kind(
        &self,
        kind: AnnotationTargetKind,
    ) -> Vec<&Annotation> {
        self.values()
            .filter(|annotation| annotation.target().kind() == kind)
            .collect()
    }

    /// Returns all annotations with a particular namespace.
    #[must_use]
    pub fn by_namespace(
        &self,
        namespace: &str,
    ) -> Vec<&Annotation> {
        self.values()
            .filter(|annotation| {
                annotation.key().namespace().as_str() == namespace
            })
            .collect()
    }

    /// Returns all currently active annotations.
    #[must_use]
    pub fn active(&self) -> Vec<&Annotation> {
        self.values()
            .filter(|annotation| annotation.is_active())
            .collect()
    }

    /// Returns a deterministic canonical textual representation.
    ///
    /// This representation is not a serialization format.
    #[must_use]
    pub fn canonical_text(&self) -> String {
        let mut output = String::new();

        let mut first = true;

        for annotation in self.values() {
            if !first {
                output.push('\n');
            }

            first = false;
            output.push_str(&annotation.canonical_text());
        }

        output
    }
}

impl<'a> IntoIterator for &'a AnnotationSet {
    type Item = &'a Annotation;
    type IntoIter =
        std::collections::btree_map::Values<'a, AnnotationKey, Annotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.values()
    }
}

// =============================================================================
// Annotation builder
// =============================================================================

/// Builder for constructing validated annotations without exposing partially
/// initialized annotation objects.
#[derive(Clone, Debug)]
pub struct AnnotationBuilder {
    id: AnnotationId,
    key: AnnotationKey,
    target: AnnotationTarget,
    value: AnnotationValue,
    origin: AnnotationOrigin,
    lifecycle: AnnotationLifecycle,
    applicability: Option<AnnotationApplicability>,
}

impl AnnotationBuilder {
    /// Creates a builder.
    #[must_use]
    pub fn new(
        id: AnnotationId,
        key: AnnotationKey,
        target: AnnotationTarget,
    ) -> Self {
        Self {
            id,
            key,
            target,
            value: AnnotationValue::Unit,
            origin: AnnotationOrigin::Unknown,
            lifecycle: AnnotationLifecycle::Active,
            applicability: None,
        }
    }

    /// Sets the annotation value.
    #[must_use]
    pub fn value(
        mut self,
        value: AnnotationValue,
    ) -> Self {
        self.value = value;
        self
    }

    /// Sets the annotation origin.
    #[must_use]
    pub fn origin(
        mut self,
        origin: AnnotationOrigin,
    ) -> Self {
        self.origin = origin;
        self
    }

    /// Sets the annotation lifecycle.
    #[must_use]
    pub fn lifecycle(
        mut self,
        lifecycle: AnnotationLifecycle,
    ) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Sets the applicability.
    #[must_use]
    pub fn applicability(
        mut self,
        applicability: AnnotationApplicability,
    ) -> Self {
        self.applicability = Some(applicability);
        self
    }

    /// Builds and validates the annotation.
    pub fn build(self) -> AnnotationResult<Annotation> {
        Annotation::with_metadata(
            self.id,
            self.key,
            self.target,
            self.value,
            self.origin,
            self.lifecycle,
            self.applicability,
        )
    }
}

// =============================================================================
// Standard Zamani annotation keys
// =============================================================================

/// Standard semantic annotation keys.
///
/// These are constructors rather than a closed annotation enum so that future
/// annotations do not require changing the annotation representation.
pub mod standard {
    use super::{
        AnnotationKey,
        AnnotationResult,
    };

    /// Marks an entity as native to a target dialect.
    pub fn native() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("native")
    }

    /// Marks an entity as experimental.
    pub fn experimental() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("experimental")
    }

    /// Marks an entity as deprecated.
    pub fn deprecated() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("deprecated")
    }

    /// Marks an entity as deterministic.
    pub fn deterministic() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("deterministic")
    }

    /// Marks an entity as semantically pure.
    pub fn pure() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("pure")
    }

    /// Marks an operation as requiring preservation during optimization.
    pub fn preserve() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("preserve")
    }

    /// Marks metadata as informational rather than semantic.
    pub fn informational() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("informational")
    }

    /// Marks an annotation as a target-specific hint.
    pub fn target_hint() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("target_hint")
    }

    /// Marks an entity as logically identified.
    pub fn logical() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("logical")
    }

    /// Marks an entity as physically mapped.
    pub fn physical() -> AnnotationResult<AnnotationKey> {
        AnnotationKey::zamani("physical")
    }
}

// =============================================================================
// Canonical helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> AnnotationResult<()> {
    if value.is_empty() {
        return Err(AnnotationError::EmptyField { field });
    }

    let bytes = value.as_bytes();

    for (index, byte) in bytes.iter().copied().enumerate() {
        let valid = is_identifier_byte(byte);

        if !valid {
            return Err(AnnotationError::InvalidIdentifier {
                field,
                value: format!(
                    "{value} (invalid byte at position {index})"
                ),
            });
        }
    }

    Ok(())
}

fn validate_namespace(
    value: &str,
) -> AnnotationResult<()> {
    if value.is_empty() {
        return Err(AnnotationError::EmptyField {
            field: "namespace",
        });
    }

    let components: Vec<&str> = value.split('.').collect();

    if components.is_empty() {
        return Err(AnnotationError::InvalidNamespace {
            value: value.to_owned(),
        });
    }

    for component in components {
        if component.is_empty() {
            return Err(AnnotationError::InvalidNamespace {
                value: value.to_owned(),
            });
        }

        validate_identifier(component, "namespace component")?;
    }

    Ok(())
}

fn is_identifier_byte(
    byte: u8,
) -> bool {
    matches!(
        byte,
        ASCII_ZERO..=ASCII_NINE
            | ASCII_UPPER_A..=ASCII_UPPER_Z
            | ASCII_LOWER_A..=ASCII_LOWER_Z
            | ASCII_UNDERSCORE
            | ASCII_DASH
    )
}

fn append_value_canonical(
    value: &AnnotationValue,
    output: &mut String,
) {
    match value {
        AnnotationValue::Unit => {
            output.push_str("unit");
        }

        AnnotationValue::Boolean(value) => {
            output.push_str(if *value { "true" } else { "false" });
        }

        AnnotationValue::Integer(value) => {
            output.push_str("i:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::UnsignedInteger(value) => {
            output.push_str("u:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::String(value) => {
            output.push_str("s:");
            append_escaped_string(value, output);
        }

        AnnotationValue::Symbol(value) => {
            output.push_str("sym:");
            append_escaped_string(value, output);
        }

        AnnotationValue::LogicalQubit(value) => {
            output.push_str("logical-qubit:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::PhysicalQubit(value) => {
            output.push_str("physical-qubit:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::Value(value) => {
            output.push_str("value:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::Parameter(value) => {
            output.push_str("parameter:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::Type(value) => {
            output.push_str("type:");
            output.push_str(&value.to_string());
        }

        AnnotationValue::List(values) => {
            output.push('[');

            let mut first = true;

            for value in values {
                if !first {
                    output.push(',');
                }

                first = false;
                append_value_canonical(value, output);
            }

            output.push(']');
        }

        AnnotationValue::Map(values) => {
            output.push('{');

            let mut first = true;

            for (key, value) in values {
                if !first {
                    output.push(',');
                }

                first = false;

                append_escaped_string(key, output);
                output.push('=');
                append_value_canonical(value, output);
            }

            output.push('}');
        }

        AnnotationValue::Bytes(values) => {
            output.push_str("bytes:");

            for byte in values {
                output.push(hex_digit(byte >> 4));
                output.push(hex_digit(byte & 0x0f));
            }
        }
    }
}

fn append_escaped_string(
    value: &str,
    output: &mut String,
) {
    output.push('"');

    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character => output.push(character),
        }
    }

    output.push('"');
}

fn hex_digit(
    value: u8,
) -> char {
    match value & 0x0f {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}

// =============================================================================
// Stable hashing support
// =============================================================================

impl Hash for Annotation {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.id.hash(state);
        self.key.hash(state);
        self.target.hash(state);
        self.value.hash(state);
        self.origin.hash(state);
        self.lifecycle.hash(state);

        match &self.applicability {
            None => {
                0u8.hash(state);
            }

            Some(applicability) => {
                1u8.hash(state);

                for kind in applicability.kinds() {
                    kind.hash(state);
                }
            }
        }
    }
}

impl Hash for AnnotationApplicability {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        for kind in &self.kinds {
            kind.hash(state);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn annotation_id() -> AnnotationId {
        AnnotationId::new(1)
    }

    fn logical_qubit() -> QubitId {
        QubitId::new(42)
    }

    #[test]
    fn namespace_validation_accepts_qualified_names() {
        let namespace =
            AnnotationNamespace::new("compiler.optimization")
                .expect("valid namespace");

        assert_eq!(
            namespace.as_str(),
            "compiler.optimization"
        );
    }

    #[test]
    fn namespace_validation_rejects_empty_components() {
        assert!(
            AnnotationNamespace::new("compiler..optimization")
                .is_err()
        );
    }

    #[test]
    fn key_is_deterministic() {
        let key = AnnotationKey::new(
            "compiler.optimization",
            "preserve",
        )
        .expect("valid key");

        assert_eq!(
            key.qualified_name(),
            "compiler.optimization.preserve"
        );
    }

    #[test]
    fn logical_qubit_target_uses_canonical_qubit_identity() {
        let target =
            AnnotationTarget::LogicalQubit(logical_qubit());

        assert_eq!(
            target.kind(),
            AnnotationTargetKind::LogicalQubit
        );

        assert!(target.is_logical_quantum_target());
        assert!(!target.is_physical_quantum_target());
    }

    #[test]
    fn values_validate_recursively() {
        let mut map = BTreeMap::new();

        map.insert(
            "enabled".to_owned(),
            AnnotationValue::Boolean(true),
        );

        map.insert(
            "qubit".to_owned(),
            AnnotationValue::LogicalQubit(logical_qubit()),
        );

        let value = AnnotationValue::Map(map);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn empty_symbol_is_rejected() {
        let value =
            AnnotationValue::Symbol(String::new());

        assert!(value.validate().is_err());
    }

    #[test]
    fn annotation_builder_creates_valid_annotation() {
        let key =
            AnnotationKey::zamani("deterministic")
                .expect("valid key");

        let annotation = AnnotationBuilder::new(
            annotation_id(),
            key,
            AnnotationTarget::LogicalQubit(
                logical_qubit(),
            ),
        )
        .value(AnnotationValue::Boolean(true))
        .origin(AnnotationOrigin::Compiler)
        .build()
        .expect("valid annotation");

        assert_eq!(
            annotation.lifecycle(),
            AnnotationLifecycle::Active
        );

        assert_eq!(
            annotation.origin(),
            &AnnotationOrigin::Compiler
        );
    }

    #[test]
    fn applicability_rejects_invalid_target() {
        let applicability =
            AnnotationApplicability::new([
                AnnotationTargetKind::Operation,
            ])
            .expect("valid applicability");

        let key =
            AnnotationKey::zamani("operation_only")
                .expect("valid key");

        let result = Annotation::with_metadata(
            annotation_id(),
            key,
            AnnotationTarget::LogicalQubit(
                logical_qubit(),
            ),
            AnnotationValue::Unit,
            AnnotationOrigin::Compiler,
            AnnotationLifecycle::Active,
            Some(applicability),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_identical_annotations_are_idempotent() {
        let key =
            AnnotationKey::zamani("deterministic")
                .expect("valid key");

        let annotation = Annotation::new(
            annotation_id(),
            key,
            AnnotationTarget::LogicalQubit(
                logical_qubit(),
            ),
            AnnotationValue::Boolean(true),
        )
        .expect("valid annotation");

        let mut set = AnnotationSet::new();

        set.insert(annotation.clone())
            .expect("first insertion");

        set.insert(annotation)
            .expect("identical insertion");

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn conflicting_annotations_are_rejected() {
        let key =
            AnnotationKey::zamani("deterministic")
                .expect("valid key");

        let first = Annotation::new(
            AnnotationId::new(1),
            key.clone(),
            AnnotationTarget::LogicalQubit(
                QubitId::new(1),
            ),
            AnnotationValue::Boolean(true),
        )
        .expect("valid annotation");

        let second = Annotation::new(
            AnnotationId::new(2),
            key,
            AnnotationTarget::LogicalQubit(
                QubitId::new(2),
            ),
            AnnotationValue::Boolean(false),
        )
        .expect("valid annotation");

        let mut set = AnnotationSet::new();

        set.insert(first)
            .expect("first insertion");

        assert!(set.insert(second).is_err());
    }

    #[test]
    fn canonical_text_is_deterministic() {
        let key =
            AnnotationKey::zamani("deterministic")
                .expect("valid key");

        let annotation = Annotation::new(
            annotation_id(),
            key,
            AnnotationTarget::LogicalQubit(
                logical_qubit(),
            ),
            AnnotationValue::Boolean(true),
        )
        .expect("valid annotation");

        let first = annotation.canonical_text();
        let second = annotation.canonical_text();

        assert_eq!(first, second);
    }

    #[test]
    fn lifecycle_cannot_silently_reactivate_disabled_annotation() {
        let key =
            AnnotationKey::zamani("temporary")
                .expect("valid key");

        let mut annotation = Annotation::new(
            annotation_id(),
            AnnotationTarget::Operation(
                OperationId::new(1),
            )
            .into_key_placeholder(),
            AnnotationTarget::Operation(
                OperationId::new(1),
            ),
            AnnotationValue::Unit,
        );

        // This branch exists only to keep the test construction explicit.
        // The actual lifecycle contract is tested below.
        let _ = annotation;

        let key =
            AnnotationKey::zamani("temporary")
                .expect("valid key");

        let mut annotation = Annotation::new(
            AnnotationId::new(2),
            key,
            AnnotationTarget::Operation(
                OperationId::new(2),
            ),
            AnnotationValue::Unit,
        )
        .expect("valid annotation");

        annotation
            .transition(AnnotationLifecycle::Disabled)
            .expect("disable");

        assert!(
            annotation
                .transition(AnnotationLifecycle::Active)
                .is_err()
        );
    }

    #[test]
    fn maps_are_canonically_ordered() {
        let mut first = BTreeMap::new();

        first.insert(
            "b".to_owned(),
            AnnotationValue::Integer(2),
        );

        first.insert(
            "a".to_owned(),
            AnnotationValue::Integer(1),
        );

        let mut second = BTreeMap::new();

        second.insert(
            "a".to_owned(),
            AnnotationValue::Integer(1),
        );

        second.insert(
            "b".to_owned(),
            AnnotationValue::Integer(2),
        );

        let left = AnnotationValue::Map(first);
        let right = AnnotationValue::Map(second);

        assert_eq!(left, right);
    }
}