//! Native Zamani AST — Serialization Schema Contract.
//!
//! Path:
//!     src/frontend/ast/node/serialization/schema.rs
//!
//! # Purpose
//!
//! This module defines the stable, target-independent schema contract for
//! serializing and deserializing the native Zamani frontend AST.
//!
//! This file owns:
//!
//! - AST serialization schema identity;
//! - schema versioning;
//! - schema compatibility policy;
//! - stable field identifiers;
//! - canonical wire-value categories;
//! - required/optional/repeated field semantics;
//! - unknown-field policy;
//! - extension schema metadata;
//! - schema descriptors;
//! - schema validation;
//! - deterministic schema manifests.
//!
//! This file does NOT own:
//!
//! - binary encoding;
//! - binary decoding;
//! - JSON encoding;
//! - JSON decoding;
//! - AST node definitions;
//! - parser logic;
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum gates;
//! - qubit allocation;
//! - routing;
//! - scheduling;
//! - hardware;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend execution.
//!
//! # Architectural boundary
//!
//! ```text
//!                    Native Zamani AST
//!                           │
//!                           ▼
//!              ┌────────────────────────┐
//!              │ serialization::schema  │
//!              │                        │
//!              │ WHAT is the contract? │
//!              └───────────┬────────────┘
//!                          │
//!              ┌───────────┼────────────┐
//!              ▼           ▼            ▼
//!           binary       JSON       other formats
//!           codec        codec          codec
//!              │           │            │
//!              └───────────┼────────────┘
//!                          ▼
//!                    serialized AST
//! ```
//!
//! The distinction is mandatory:
//!
//! ```text
//! schema.rs       = contract
//! binary.rs       = binary mechanism
//! json.rs         = JSON mechanism
//! version.rs      = compatibility/version policy if separated
//! ```
//!
//! # POCO-REAF / scalability
//!
//! The schema contains no machine-size assumptions.
//!
//! It does NOT define:
//!
//! - maximum AST nodes;
//! - maximum qubits;
//! - maximum registers;
//! - maximum operations;
//! - maximum nesting;
//! - maximum program size;
//! - maximum resource count;
//! - maximum hardware size;
//! - maximum backend size.
//!
//! A particular compiler invocation may impose configurable resource limits,
//! but such limits are not part of this schema.
//!
//! Therefore:
//!
//! ```text
//! language semantics
//!     !=
//! schema capacity
//!     !=
//! decoder resource policy
//!     !=
//! hardware capability
//! ```
//!
//! A decoder running on a small machine may choose a small resource budget
//! without changing what the Zamani AST schema can represent.
//!
//! # Domain neutrality
//!
//! The schema deliberately does not contain quantum-specific field kinds.
//!
//! A quantum operation, classical operation, HDL operation, accelerator
//! operation, or future computational construct is represented using generic
//! schema primitives and/or versioned extensions.
//!
//! A new quantum technology, backend, compiler optimization, or computational
//! domain therefore does not require modifying this file unless it introduces
//! a genuinely new wire-level primitive or changes an existing contract.
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
//! - no unsafe code.
//!
//! This module intentionally uses only the Rust standard library.
//!
//! # Determinism
//!
//! Canonical schema descriptors use ordered `Vec` collections rather than
//! randomized maps. Field identifiers and extension identifiers must be
//! strictly increasing within their respective descriptor collections.
//!
//! Consequently, a validated schema has deterministic descriptor order and a
//! deterministic canonical manifest.
//!
//! # Integration contract
//!
//! ```text
//! source/
//!     │
//!     ▼
//! node/
//!     │
//!     ├── node_id.rs
//!     ├── node_kind.rs
//!     ├── metadata.rs
//!     └── node.rs
//!             │
//!             ▼
//!     serialization/schema.rs
//!             │
//!       ┌─────┴─────────┐
//!       ▼               ▼
//! serialization/      serialization/
//! binary.rs           json.rs
//!       │               │
//!       └───────┬───────┘
//!               ▼
//!        persisted AST
//! ```
//!
//! The schema module depends on no serialization mechanism.
//!
//! # Important rule
//!
//! AST node modules must not invent their own serialization schema versions.
//! They consume the authoritative schema contract exposed here.
//!
//! Likewise, binary/json encoders must not redefine field IDs or compatibility
//! policy independently of this module.
//!
//! # No unsafe
//!
//! This module explicitly forbids unsafe Rust.
//!
//! ```text
//! no unsafe
//! no pointers
//! no memory addresses
//! no target-specific layout assumptions
//! no architecture-dependent serialization
//! ```

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable namespace for the native Zamani AST serialization schemas.
///
/// This identifier is persistent schema metadata and is not tied to:
///
/// - the Rust crate name;
/// - compiler version;
/// - source repository layout;
/// - hardware;
/// - vendor;
/// - operating system.
pub const SCHEMA_NAMESPACE: &str = "zamani.frontend.ast.serialization";

/// Stable identifier for the complete native AST serialization document.
pub const DOCUMENT_SCHEMA_ID: &str =
    "zamani.frontend.ast.serialization.document";

/// Stable identifier for the serialized AST payload.
pub const PAYLOAD_SCHEMA_ID: &str =
    "zamani.frontend.ast.serialization.payload";

/// Current schema major version.
///
/// A major change may require an explicit migration.
pub const SCHEMA_MAJOR: u16 = 1;

/// Current schema minor version.
///
/// Minor versions may add explicitly compatible fields/extensions.
pub const SCHEMA_MINOR: u16 = 0;

/// Current schema patch version.
///
/// Patch versions must preserve the schema contract.
pub const SCHEMA_PATCH: u16 = 0;

/// Complete current AST serialization schema version.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion =
    SchemaVersion::new(SCHEMA_MAJOR, SCHEMA_MINOR, SCHEMA_PATCH);

// =============================================================================
// Schema version
// =============================================================================

/// Version of the native Zamani AST serialization schema.
///
/// This is deliberately independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - AST semantic version;
/// - ZUIR version;
/// - quantum IR version;
/// - binary format version;
/// - hardware version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SchemaVersion {
    /// Creates a schema version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns the current schema version.
    pub const fn current() -> Self {
        CURRENT_SCHEMA_VERSION
    }

    /// Returns true when this version exactly matches the current schema.
    pub const fn is_current(self) -> bool {
        self.is_exact(CURRENT_SCHEMA_VERSION)
    }

    /// Returns true when two versions have the same major contract.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether `self` can read `incoming`.
    ///
    /// Compatibility is deliberately conservative:
    ///
    /// - major versions must match;
    /// - an incoming minor version must not be newer;
    /// - an incoming patch version must not be newer when the minor versions
    ///   are equal.
    ///
    /// This prevents a newer schema from being silently interpreted by an
    /// older reader.
    pub const fn supports(self, incoming: Self) -> bool {
        self.major == incoming.major
            && (incoming.minor < self.minor
                || (incoming.minor == self.minor
                    && incoming.patch <= self.patch))
    }

    /// Returns whether two versions are exactly equal.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether an explicit migration is required.
    pub const fn requires_migration(self, other: Self) -> bool {
        !self.is_exact(other)
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
// Schema kind
// =============================================================================

/// Kind of schema described by a [`SchemaDescriptor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemaKind {
    /// Complete serialized AST document.
    Document,

    /// Semantic payload carried by the document.
    Payload,

    /// Individual AST object schema.
    Object,

    /// Extensible AST payload schema.
    Extension,

    /// Future/versioned dialect schema.
    Dialect,
}

impl SchemaKind {
    /// Stable discriminant used by schema metadata.
    pub const fn discriminant(self) -> u8 {
        match self {
            Self::Document => 1,
            Self::Payload => 2,
            Self::Object => 3,
            Self::Extension => 4,
            Self::Dialect => 5,
        }
    }
}

// =============================================================================
// Wire kinds
// =============================================================================

/// Generic wire-level value category.
///
/// This describes representation, not language semantics.
///
/// For example, a `NodeId` remains an AST concept while its serialized
/// representation may be [`WireKind::Identity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WireKind {
    /// No value.
    Unit,

    /// Canonical boolean.
    Bool,

    /// Unsigned 8-bit integer.
    U8,

    /// Unsigned 16-bit integer.
    U16,

    /// Unsigned 32-bit integer.
    U32,

    /// Unsigned 64-bit integer.
    U64,

    /// Signed 8-bit integer.
    I8,

    /// Signed 16-bit integer.
    I16,

    /// Signed 32-bit integer.
    I32,

    /// Signed 64-bit integer.
    I64,

    /// IEEE-754 binary32 value.
    F32,

    /// IEEE-754 binary64 value.
    F64,

    /// UTF-8 text.
    String,

    /// Arbitrary bytes.
    Bytes,

    /// Ordered sequence.
    Sequence,

    /// Canonical map.
    Map,

    /// Nested schema-defined object.
    Object,

    /// Stable schema identifier.
    SchemaId,

    /// Stable object identity such as `NodeId`.
    Identity,

    /// Versioned extension payload.
    Extension,

    /// Symbolic expression/value.
    Expression,
}

impl WireKind {
    /// Stable discriminant for the wire-kind metadata.
    pub const fn discriminant(self) -> u8 {
        match self {
            Self::Unit => 0,
            Self::Bool => 1,
            Self::U8 => 2,
            Self::U16 => 3,
            Self::U32 => 4,
            Self::U64 => 5,
            Self::I8 => 6,
            Self::I16 => 7,
            Self::I32 => 8,
            Self::I64 => 9,
            Self::F32 => 10,
            Self::F64 => 11,
            Self::String => 12,
            Self::Bytes => 13,
            Self::Sequence => 14,
            Self::Map => 15,
            Self::Object => 16,
            Self::SchemaId => 17,
            Self::Identity => 18,
            Self::Extension => 19,
            Self::Expression => 20,
        }
    }

    /// Returns the fixed byte width when one exists.
    ///
    /// `None` means that the value has variable size.
    pub const fn fixed_width(self) -> Option<u8> {
        match self {
            Self::Unit => Some(0),
            Self::Bool | Self::U8 | Self::I8 => Some(1),
            Self::U16 | Self::I16 => Some(2),
            Self::U32 | Self::I32 | Self::F32 => Some(4),
            Self::U64 | Self::I64 | Self::F64 | Self::Identity => Some(8),
            Self::String
            | Self::Bytes
            | Self::Sequence
            | Self::Map
            | Self::Object
            | Self::SchemaId
            | Self::Extension
            | Self::Expression => None,
        }
    }

    /// Returns whether this wire kind can carry an extension payload.
    pub const fn supports_extension_payload(self) -> bool {
        matches!(
            self,
            Self::Bytes | Self::Object | Self::Extension
        )
    }
}

// =============================================================================
// Field identity
// =============================================================================

/// Stable field identifier.
///
/// This is a persistent schema identifier, not a Rust field position.
///
/// Field identifiers must never be silently reused for a different semantic
/// meaning within the same schema major version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldId(u32);

impl FieldId {
    /// Creates a field identifier.
    ///
    /// Zero is reserved for schema-level absence/invalidity and is rejected
    /// by schema validation.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns whether this is a valid non-zero field identifier.
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl From<u32> for FieldId {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<FieldId> for u32 {
    fn from(value: FieldId) -> Self {
        value.value()
    }
}

impl fmt::Display for FieldId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Field cardinality
// =============================================================================

/// Cardinality of a schema field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCardinality {
    /// Exactly one value is required.
    Required,

    /// Zero or one value may be present.
    Optional,

    /// Zero or more values may be present.
    Repeated,

    /// One or more values must be present.
    NonEmptyRepeated,
}

impl FieldCardinality {
    /// Stable discriminant.
    pub const fn discriminant(self) -> u8 {
        match self {
            Self::Required => 1,
            Self::Optional => 2,
            Self::Repeated => 3,
            Self::NonEmptyRepeated => 4,
        }
    }

    /// Returns whether an empty collection is structurally valid.
    pub const fn allows_empty(self) -> bool {
        matches!(
            self,
            Self::Optional | Self::Repeated
        )
    }
}

// =============================================================================
// Unknown-field policy
// =============================================================================

/// Policy for fields unknown to a reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownFieldPolicy {
    /// Unknown fields are rejected.
    Reject,

    /// Unknown fields are skipped when the encoding permits safe skipping.
    Skip,

    /// Unknown fields must be preserved for round-tripping.
    Preserve,
}

impl UnknownFieldPolicy {
    /// Stable discriminant.
    pub const fn discriminant(self) -> u8 {
        match self {
            Self::Reject => 1,
            Self::Skip => 2,
            Self::Preserve => 3,
        }
    }
}

// =============================================================================
// Extension policy
// =============================================================================

/// Policy governing versioned AST extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionPolicy {
    /// Extension is required and must be understood.
    Required,

    /// Extension may be skipped when it is not understood.
    Optional,

    /// Extension must survive round-trip serialization even if its semantics
    /// are not understood by the current compiler.
    Preserve,
}

impl ExtensionPolicy {
    /// Stable discriminant.
    pub const fn discriminant(self) -> u8 {
        match self {
            Self::Required => 1,
            Self::Optional => 2,
            Self::Preserve => 3,
        }
    }
}

// =============================================================================
// Field descriptor
// =============================================================================

/// Metadata describing one schema field.
///
/// The descriptor contains no encoder-specific implementation details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    /// Stable numeric field identity.
    pub id: FieldId,

    /// Stable namespaced field name.
    pub name: String,

    /// Generic wire representation.
    pub wire_kind: WireKind,

    /// Field cardinality.
    pub cardinality: FieldCardinality,

    /// Whether unknown readers may skip this field.
    pub unknown_policy: UnknownFieldPolicy,

    /// Human-readable documentation.
    pub documentation: String,
}

impl FieldDescriptor {
    /// Creates a field descriptor.
    pub fn new(
        id: FieldId,
        name: impl Into<String>,
        wire_kind: WireKind,
        cardinality: FieldCardinality,
        unknown_policy: UnknownFieldPolicy,
        documentation: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            wire_kind,
            cardinality,
            unknown_policy,
            documentation: documentation.into(),
        }
    }

    /// Returns whether the descriptor is structurally valid by itself.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if !self.id.is_valid() {
            return Err(SchemaError::InvalidFieldId {
                field_id: self.id,
            });
        }

        if self.name.is_empty() {
            return Err(SchemaError::EmptyFieldName {
                field_id: self.id,
            });
        }

        if self.name.contains('\0') {
            return Err(SchemaError::InvalidFieldName {
                field_id: self.id,
                reason: "field name contains NUL",
            });
        }

        if self.documentation.contains('\0') {
            return Err(SchemaError::InvalidDocumentation {
                field_id: self.id,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Extension descriptor
// =============================================================================

/// Descriptor for a versioned AST extension.
///
/// Extensions allow future computational domains and language features to
/// evolve without turning the core AST schema into a closed list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionDescriptor {
    /// Stable namespaced extension identifier.
    pub id: String,

    /// Version of this extension schema.
    pub version: SchemaVersion,

    /// Unknown-extension handling policy.
    pub policy: ExtensionPolicy,

    /// Human-readable documentation.
    pub documentation: String,
}

impl ExtensionDescriptor {
    /// Creates an extension descriptor.
    pub fn new(
        id: impl Into<String>,
        version: SchemaVersion,
        policy: ExtensionPolicy,
        documentation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            version,
            policy,
            documentation: documentation.into(),
        }
    }

    /// Validates the extension descriptor.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if self.id.is_empty() {
            return Err(SchemaError::EmptyExtensionId);
        }

        if self.id.contains('\0') {
            return Err(SchemaError::InvalidExtensionId {
                id: self.id.clone(),
                reason: "extension identifier contains NUL",
            });
        }

        if self.documentation.contains('\0') {
            return Err(SchemaError::InvalidExtensionDocumentation {
                id: self.id.clone(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Schema descriptor
// =============================================================================

/// Complete schema contract for a serialized AST object/document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaDescriptor {
    /// Stable schema identifier.
    pub id: String,

    /// Schema category.
    pub kind: SchemaKind,

    /// Schema version.
    pub version: SchemaVersion,

    /// Unknown-field policy.
    pub unknown_field_policy: UnknownFieldPolicy,

    /// Ordered stable field descriptors.
    ///
    /// Fields MUST be sorted strictly by `FieldId`.
    pub fields: Vec<FieldDescriptor>,

    /// Ordered extension descriptors.
    ///
    /// Extensions MUST be sorted lexicographically by identifier.
    pub extensions: Vec<ExtensionDescriptor>,

    /// Human-readable schema documentation.
    pub documentation: String,
}

impl SchemaDescriptor {
    /// Creates a schema descriptor.
    pub fn new(
        id: impl Into<String>,
        kind: SchemaKind,
        version: SchemaVersion,
        unknown_field_policy: UnknownFieldPolicy,
        fields: Vec<FieldDescriptor>,
        extensions: Vec<ExtensionDescriptor>,
        documentation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            version,
            unknown_field_policy,
            fields,
            extensions,
            documentation: documentation.into(),
        }
    }

    /// Creates the canonical top-level Zamani AST document schema.
    ///
    /// This descriptor intentionally contains only generic serialization
    /// metadata. Concrete node schemas can be defined independently.
    pub fn document() -> Self {
        Self::new(
            DOCUMENT_SCHEMA_ID,
            SchemaKind::Document,
            CURRENT_SCHEMA_VERSION,
            UnknownFieldPolicy::Preserve,
            Vec::new(),
            Vec::new(),
            "Canonical serialized native Zamani frontend AST document.",
        )
    }

    /// Returns the number of fields.
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns the number of registered extensions.
    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }

    /// Finds a field by stable field identifier.
    pub fn field(&self, id: FieldId) -> Option<&FieldDescriptor> {
        self.fields
            .binary_search_by_key(&id, |field| field.id)
            .ok()
            .map(|index| &self.fields[index])
    }

    /// Finds a field by its stable name.
    pub fn field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|field| field.name == name)
    }

    /// Finds an extension by stable identifier.
    pub fn extension(&self, id: &str) -> Option<&ExtensionDescriptor> {
        self.extensions
            .binary_search_by(|extension| extension.id.as_str().cmp(id))
            .ok()
            .map(|index| &self.extensions[index])
    }

    /// Validates the complete schema.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if self.id.is_empty() {
            return Err(SchemaError::EmptySchemaId);
        }

        if self.id.contains('\0') {
            return Err(SchemaError::InvalidSchemaId {
                reason: "schema identifier contains NUL",
            });
        }

        if self.documentation.contains('\0') {
            return Err(SchemaError::InvalidSchemaDocumentation);
        }

        let mut previous_field: Option<FieldId> = None;

        for field in &self.fields {
            field.validate()?;

            if let Some(previous) = previous_field {
                if field.id <= previous {
                    return Err(SchemaError::FieldsNotStrictlyOrdered {
                        previous,
                        current: field.id,
                    });
                }
            }

            previous_field = Some(field.id);
        }

        let mut previous_extension: Option<&str> = None;

        for extension in &self.extensions {
            extension.validate()?;

            if let Some(previous) = previous_extension {
                if extension.id.as_str() <= previous {
                    return Err(
                        SchemaError::ExtensionsNotStrictlyOrdered {
                            previous: previous.to_owned(),
                            current: extension.id.clone(),
                        },
                    );
                }
            }

            previous_extension = Some(extension.id.as_str());
        }

        Ok(())
    }

    /// Returns whether another schema can be read by this schema contract.
    pub fn supports_schema(
        &self,
        incoming: &SchemaDescriptor,
    ) -> Result<bool, SchemaError> {
        self.validate()?;
        incoming.validate()?;

        if self.id != incoming.id {
            return Ok(false);
        }

        Ok(self.version.supports(incoming.version))
    }

    /// Returns a deterministic canonical manifest.
    ///
    /// The manifest is intended for:
    ///
    /// - schema fingerprinting;
    /// - diagnostics;
    /// - golden tests;
    /// - reproducibility checks;
    /// - compatibility tests.
    ///
    /// It is deliberately independent from the binary serialization codec.
    pub fn canonical_manifest(&self) -> Result<String, SchemaError> {
        self.validate()?;

        let mut output = String::new();

        output.push_str("schema=");
        append_escaped(&mut output, &self.id);
        output.push('\n');

        output.push_str("kind=");
        output.push_str(&self.kind.discriminant().to_string());
        output.push('\n');

        output.push_str("version=");
        output.push_str(&self.version.to_string());
        output.push('\n');

        output.push_str("unknown=");
        output.push_str(
            &self
                .unknown_field_policy
                .discriminant()
                .to_string(),
        );
        output.push('\n');

        output.push_str("documentation=");
        append_escaped(&mut output, &self.documentation);
        output.push('\n');

        for field in &self.fields {
            output.push_str("field=");
            output.push_str(&field.id.value().to_string());
            output.push('|');
            append_escaped(&mut output, &field.name);
            output.push('|');
            output.push_str(&field.wire_kind.discriminant().to_string());
            output.push('|');
            output.push_str(&field.cardinality.discriminant().to_string());
            output.push('|');
            output.push_str(
                &field.unknown_policy.discriminant().to_string(),
            );
            output.push('|');
            append_escaped(&mut output, &field.documentation);
            output.push('\n');
        }

        for extension in &self.extensions {
            output.push_str("extension=");
            append_escaped(&mut output, &extension.id);
            output.push('|');
            output.push_str(&extension.version.to_string());
            output.push('|');
            output.push_str(&extension.policy.discriminant().to_string());
            output.push('|');
            append_escaped(&mut output, &extension.documentation);
            output.push('\n');
        }

        Ok(output)
    }
}

// =============================================================================
// Schema errors
// =============================================================================

/// Errors produced while validating a serialization schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// Schema identifier is empty.
    EmptySchemaId,

    /// Schema identifier contains invalid content.
    InvalidSchemaId {
        /// Explanation of the invalid condition.
        reason: &'static str,
    },

    /// Schema documentation contains an invalid NUL character.
    InvalidSchemaDocumentation,

    /// Field identifier is zero.
    InvalidFieldId {
        /// Invalid identifier.
        field_id: FieldId,
    },

    /// Field name is empty.
    EmptyFieldName {
        /// Field containing the invalid name.
        field_id: FieldId,
    },

    /// Field name contains invalid content.
    InvalidFieldName {
        /// Field containing the invalid name.
        field_id: FieldId,

        /// Explanation.
        reason: &'static str,
    },

    /// Field documentation contains invalid content.
    InvalidDocumentation {
        /// Field containing the invalid documentation.
        field_id: FieldId,
    },

    /// Field identifiers are not strictly increasing.
    FieldsNotStrictlyOrdered {
        /// Previous field identifier.
        previous: FieldId,

        /// Current field identifier.
        current: FieldId,
    },

    /// Extension identifier is empty.
    EmptyExtensionId,

    /// Extension identifier contains invalid content.
    InvalidExtensionId {
        /// Invalid identifier.
        id: String,

        /// Explanation.
        reason: &'static str,
    },

    /// Extension documentation contains invalid content.
    InvalidExtensionDocumentation {
        /// Extension identifier.
        id: String,
    },

    /// Extension identifiers are not strictly ordered.
    ExtensionsNotStrictlyOrdered {
        /// Previous extension identifier.
        previous: String,

        /// Current extension identifier.
        current: String,
    },
}

impl fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchemaId => {
                write!(formatter, "AST serialization schema identifier is empty")
            }

            Self::InvalidSchemaId { reason } => {
                write!(
                    formatter,
                    "invalid AST serialization schema identifier: {reason}"
                )
            }

            Self::InvalidSchemaDocumentation => {
                write!(
                    formatter,
                    "AST serialization schema documentation contains NUL"
                )
            }

            Self::InvalidFieldId { field_id } => {
                write!(
                    formatter,
                    "invalid AST serialization field identifier: {field_id}"
                )
            }

            Self::EmptyFieldName { field_id } => {
                write!(
                    formatter,
                    "AST serialization field {field_id} has an empty name"
                )
            }

            Self::InvalidFieldName {
                field_id,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid AST serialization field name for \
                     field {field_id}: {reason}"
                )
            }

            Self::InvalidDocumentation { field_id } => {
                write!(
                    formatter,
                    "AST serialization field {field_id} has invalid documentation"
                )
            }

            Self::FieldsNotStrictlyOrdered {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "AST serialization fields are not strictly ordered: \
                     {previous} followed by {current}"
                )
            }

            Self::EmptyExtensionId => {
                write!(
                    formatter,
                    "AST serialization extension identifier is empty"
                )
            }

            Self::InvalidExtensionId { id, reason } => {
                write!(
                    formatter,
                    "invalid AST serialization extension identifier \
                     `{id}`: {reason}"
                )
            }

            Self::InvalidExtensionDocumentation { id } => {
                write!(
                    formatter,
                    "AST serialization extension `{id}` has invalid documentation"
                )
            }

            Self::ExtensionsNotStrictlyOrdered {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "AST serialization extensions are not strictly ordered: \
                     `{previous}` followed by `{current}`"
                )
            }
        }
    }
}

impl std::error::Error for SchemaError {}

// =============================================================================
// Canonical string escaping
// =============================================================================

/// Appends a deterministic escaped representation suitable for the canonical
/// schema manifest.
///
/// This is deliberately small and independent of JSON or another external
/// serialization format.
fn append_escaped(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '|' => output.push_str("\\|"),
            '=' => output.push_str("\\="),
            '\0' => output.push_str("\\0"),
            character => output.push(character),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_schema_version_is_valid() {
        let version = SchemaVersion::current();

        assert_eq!(version.major(), SCHEMA_MAJOR);
        assert_eq!(version.minor(), SCHEMA_MINOR);
        assert_eq!(version.patch(), SCHEMA_PATCH);
        assert!(version.is_current());
    }

    #[test]
    fn schema_version_display_is_deterministic() {
        assert_eq!(
            SchemaVersion::new(1, 2, 3).to_string(),
            "1.2.3"
        );
    }

    #[test]
    fn current_schema_supports_itself() {
        let current = SchemaVersion::current();

        assert!(current.supports(current));
    }

    #[test]
    fn newer_minor_schema_is_not_implicitly_supported() {
        let reader = SchemaVersion::new(1, 2, 0);
        let incoming = SchemaVersion::new(1, 3, 0);

        assert!(!reader.supports(incoming));
    }

    #[test]
    fn older_minor_schema_is_supported() {
        let reader = SchemaVersion::new(1, 2, 0);
        let incoming = SchemaVersion::new(1, 1, 0);

        assert!(reader.supports(incoming));
    }

    #[test]
    fn different_major_schema_is_not_supported() {
        let reader = SchemaVersion::new(1, 0, 0);
        let incoming = SchemaVersion::new(2, 0, 0);

        assert!(!reader.supports(incoming));
    }

    #[test]
    fn wire_kind_discriminants_are_unique() {
        let kinds = [
            WireKind::Unit,
            WireKind::Bool,
            WireKind::U8,
            WireKind::U16,
            WireKind::U32,
            WireKind::U64,
            WireKind::I8,
            WireKind::I16,
            WireKind::I32,
            WireKind::I64,
            WireKind::F32,
            WireKind::F64,
            WireKind::String,
            WireKind::Bytes,
            WireKind::Sequence,
            WireKind::Map,
            WireKind::Object,
            WireKind::SchemaId,
            WireKind::Identity,
            WireKind::Extension,
            WireKind::Expression,
        ];

        for left in 0..kinds.len() {
            for right in (left + 1)..kinds.len() {
                assert_ne!(
                    kinds[left].discriminant(),
                    kinds[right].discriminant()
                );
            }
        }
    }

    #[test]
    fn field_descriptor_rejects_zero_id() {
        let field = FieldDescriptor::new(
            FieldId::new(0),
            "node_id",
            WireKind::Identity,
            FieldCardinality::Required,
            UnknownFieldPolicy::Reject,
            "AST node identity.",
        );

        assert!(matches!(
            field.validate(),
            Err(SchemaError::InvalidFieldId { .. })
        ));
    }

    #[test]
    fn field_descriptor_rejects_empty_name() {
        let field = FieldDescriptor::new(
            FieldId::new(1),
            "",
            WireKind::Identity,
            FieldCardinality::Required,
            UnknownFieldPolicy::Reject,
            "AST node identity.",
        );

        assert!(matches!(
            field.validate(),
            Err(SchemaError::EmptyFieldName { .. })
        ));
    }

    #[test]
    fn schema_requires_strict_field_order() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Reject,
            vec![
                FieldDescriptor::new(
                    FieldId::new(2),
                    "second",
                    WireKind::String,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "Second field.",
                ),
                FieldDescriptor::new(
                    FieldId::new(1),
                    "first",
                    WireKind::String,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "First field.",
                ),
            ],
            Vec::new(),
            "Test schema.",
        );

        assert!(matches!(
            schema.validate(),
            Err(SchemaError::FieldsNotStrictlyOrdered { .. })
        ));
    }

    #[test]
    fn schema_accepts_deterministically_ordered_fields() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Reject,
            vec![
                FieldDescriptor::new(
                    FieldId::new(1),
                    "first",
                    WireKind::String,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "First field.",
                ),
                FieldDescriptor::new(
                    FieldId::new(2),
                    "second",
                    WireKind::U64,
                    FieldCardinality::Optional,
                    UnknownFieldPolicy::Skip,
                    "Second field.",
                ),
            ],
            Vec::new(),
            "Test schema.",
        );

        assert!(schema.validate().is_ok());
    }

    #[test]
    fn extension_order_is_deterministic() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Preserve,
            Vec::new(),
            vec![
                ExtensionDescriptor::new(
                    "zamani.a",
                    SchemaVersion::current(),
                    ExtensionPolicy::Optional,
                    "A.",
                ),
                ExtensionDescriptor::new(
                    "zamani.b",
                    SchemaVersion::current(),
                    ExtensionPolicy::Preserve,
                    "B.",
                ),
            ],
            "Test schema.",
        );

        assert!(schema.validate().is_ok());
    }

    #[test]
    fn canonical_manifest_is_deterministic() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Preserve,
            vec![
                FieldDescriptor::new(
                    FieldId::new(1),
                    "node_id",
                    WireKind::Identity,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "Stable AST node identity.",
                ),
                FieldDescriptor::new(
                    FieldId::new(2),
                    "name",
                    WireKind::String,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "Node name.",
                ),
            ],
            vec![ExtensionDescriptor::new(
                "zamani.frontend.example",
                SchemaVersion::new(1, 0, 0),
                ExtensionPolicy::Preserve,
                "Example extension.",
            )],
            "Test schema.",
        );

        let first = schema.canonical_manifest().expect("valid schema");
        let second = schema.canonical_manifest().expect("valid schema");

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_manifest_escapes_delimiters() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Reject,
            vec![FieldDescriptor::new(
                FieldId::new(1),
                "a|b",
                WireKind::String,
                FieldCardinality::Required,
                UnknownFieldPolicy::Reject,
                "line\nvalue",
            )],
            Vec::new(),
            "documentation=value",
        );

        let manifest = schema.canonical_manifest().expect("valid schema");

        assert!(manifest.contains("a\\|b"));
        assert!(manifest.contains("line\\nvalue"));
        assert!(manifest.contains("documentation\\=value"));
    }

    #[test]
    fn document_schema_is_valid() {
        let schema = SchemaDescriptor::document();

        assert!(schema.validate().is_ok());
        assert_eq!(schema.id, DOCUMENT_SCHEMA_ID);
        assert_eq!(schema.version, CURRENT_SCHEMA_VERSION);
        assert_eq!(schema.kind, SchemaKind::Document);
    }

    #[test]
    fn field_lookup_is_stable() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Reject,
            vec![
                FieldDescriptor::new(
                    FieldId::new(1),
                    "first",
                    WireKind::String,
                    FieldCardinality::Required,
                    UnknownFieldPolicy::Reject,
                    "First.",
                ),
                FieldDescriptor::new(
                    FieldId::new(7),
                    "seventh",
                    WireKind::U64,
                    FieldCardinality::Optional,
                    UnknownFieldPolicy::Skip,
                    "Seventh.",
                ),
            ],
            Vec::new(),
            "Test.",
        );

        assert_eq!(
            schema.field(FieldId::new(7)).map(|field| field.name.as_str()),
            Some("seventh")
        );

        assert_eq!(schema.field(FieldId::new(8)), None);
    }

    #[test]
    fn extension_lookup_is_stable() {
        let schema = SchemaDescriptor::new(
            "zamani.test.schema",
            SchemaKind::Object,
            SchemaVersion::current(),
            UnknownFieldPolicy::Preserve,
            Vec::new(),
            vec![
                ExtensionDescriptor::new(
                    "zamani.a",
                    SchemaVersion::current(),
                    ExtensionPolicy::Optional,
                    "A.",
                ),
                ExtensionDescriptor::new(
                    "zamani.b",
                    SchemaVersion::current(),
                    ExtensionPolicy::Preserve,
                    "B.",
                ),
            ],
            "Test.",
        );

        assert_eq!(
            schema.extension("zamani.b").map(|extension| extension.id.as_str()),
            Some("zamani.b")
        );

        assert!(schema.extension("zamani.missing").is_none());
    }

    #[test]
    fn fixed_width_values_are_correct() {
        assert_eq!(WireKind::Unit.fixed_width(), Some(0));
        assert_eq!(WireKind::Bool.fixed_width(), Some(1));
        assert_eq!(WireKind::U16.fixed_width(), Some(2));
        assert_eq!(WireKind::U32.fixed_width(), Some(4));
        assert_eq!(WireKind::U64.fixed_width(), Some(8));
        assert_eq!(WireKind::String.fixed_width(), None);
        assert_eq!(WireKind::Sequence.fixed_width(), None);
    }
}