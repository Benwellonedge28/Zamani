//! Zamani Quantum IR — Canonical Serialization Schema
//!
//! This module defines the stable schema contract for the Zamani Quantum IR
//! serialization subsystem.
//!
//! # Architectural role
//!
//! `schema.rs` owns:
//!
//! - serialization schema identity;
//! - schema versioning;
//! - schema compatibility policy;
//! - canonical field identifiers;
//! - canonical wire-value kinds;
//! - schema descriptors;
//! - field metadata;
//! - extension/unknown-field policy;
//! - schema validation;
//! - deterministic schema manifests.
//!
//! It does NOT own:
//!
//! - binary encoding;
//! - binary decoding;
//! - quantum-program semantics;
//! - gates;
//! - qubit allocation;
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - backend execution.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural boundary
//!
//! ```text
//!                         Zamani semantic IR
//!                                │
//!                                ▼
//!                    ┌───────────────────────┐
//!                    │ serialization::schema │
//!                    │                       │
//!                    │ WHAT is the wire      │
//!                    │ contract?             │
//!                    └───────────┬───────────┘
//!                                │
//!                                ▼
//!                    serialization::serialization
//!                                │
//!                                ▼
//!                         canonical bytes
//! ```
//!
//! The distinction is important:
//!
//! ```text
//! schema.rs
//!     = contract
//!
//! serialization.rs
//!     = mechanism
//! ```
//!
//! # Universal-program principle
//!
//! This schema deliberately contains no fixed quantum-machine size.
//!
//! It does NOT define:
//!
//! - maximum qubits;
//! - maximum logical qubits;
//! - maximum physical qubits;
//! - maximum gates;
//! - maximum operations;
//! - maximum registers;
//! - maximum topology size;
//! - maximum number of quantum nodes.
//!
//! A qubit count is data in the semantic IR, not a schema constant.
//!
//! A `QubitId` remains owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! and a physical identity remains owned by:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The schema only describes how such values are represented on the wire.
//!
//! # Resource limits
//!
//! Schema limits and decoder limits are deliberately separate.
//!
//! ```text
//! schema
//!     = what can be represented
//!
//! DecodeLimits
//!     = how much a particular process is willing to consume
//!
//! hardware capability
//!     = what a particular machine provides
//! ```
//!
//! Therefore a decoder configured for a small embedded device does not make
//! that device's limits part of the Zamani IR architecture.
//!
//! # Versioning
//!
//! There are several independent version domains:
//!
//! ```text
//! Zamani language version
//!          !=
//! compiler version
//!          !=
//! Quantum IR semantic version
//!          !=
//! serialization format version
//!          !=
//! schema version
//!          !=
//! hardware version
//!          !=
//! calibration version
//! ```
//!
//! This module only owns the serialization-schema version.
//!
//! `IrVersion` remains owned by `identity.rs`.
//!
//! # Compatibility model
//!
//! Compatibility is intentionally conservative.
//!
//! A reader may consume an older schema when the schema explicitly declares
//! that the newer reader can interpret it.
//!
//! Unknown fields are not automatically discarded.
//!
//! Depending on the field policy, unknown fields are:
//!
//! - preserved;
//! - skipped explicitly;
//! - rejected.
//!
//! Unknown semantic extensions must never silently change program meaning.
//!
//! # Canonical determinism
//!
//! Schema descriptors are ordered by their stable numeric field identifier.
//!
//! No `HashMap` or randomized collection is used by the canonical schema
//! representation.
//!
//! This guarantees that two identical schema definitions produce identical
//! manifests when iterated through their canonical representation.
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
//! - no `unsafe`.
//!
//! No external dependency is required by this module.
//!
//! -----------------------------------------------------------------------------
//! Contract
//! -----------------------------------------------------------------------------
//!
//! Every schema-defined object has:
//!
//! 1. stable schema identifier;
//! 2. schema version;
//! 3. field identifiers;
//! 4. field wire kinds;
//! 5. required/optional semantics;
//! 6. unknown-field policy;
//! 7. compatibility policy.
//!
//! -----------------------------------------------------------------------------
//! Important integration rule
//! -----------------------------------------------------------------------------
//!
//! New quantum operations, gates, architectures, QEC codes, pulse forms,
//! hardware capabilities, and vendor extensions must NOT require changing this
//! file unless they introduce a new *wire-level primitive* or change an
//! existing schema contract.
//!
//! Prefer defining new schema IDs in their owning serialization module and
//! representing them through the extension mechanism.
//!
//! -----------------------------------------------------------------------------
//! No unsafe code
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// =============================================================================
// Schema identity
// =============================================================================

/// Stable namespace for Zamani Quantum IR serialization schemas.
///
/// This identifier is deliberately independent from:
///
/// - the Rust crate name;
/// - compiler version;
/// - Zamani language version;
/// - hardware vendor;
/// - hardware model.
///
/// It is part of the persistent serialization contract.
pub const SCHEMA_NAMESPACE: &str = "zamani.quantum.ir.serialization";

/// Stable identifier for the canonical Quantum IR document schema.
///
/// This identifies the complete serialized-document contract rather than an
/// individual semantic IR object.
pub const DOCUMENT_SCHEMA_ID: &str =
    "zamani.quantum.ir.serialization.document";

/// Stable identifier for the canonical payload schema.
///
/// The framing header belongs to `serialization.rs`; this identifier describes
/// the semantic payload contract carried inside that frame.
pub const PAYLOAD_SCHEMA_ID: &str =
    "zamani.quantum.ir.serialization.payload";

/// Current schema major version.
///
/// A major version change means existing readers may no longer be able to
/// interpret the schema without an explicit migration.
pub const SCHEMA_MAJOR: u16 = 1;

/// Current schema minor version.
///
/// Minor releases may add explicitly compatible fields or capabilities.
pub const SCHEMA_MINOR: u16 = 0;

/// Current schema patch version.
///
/// Patch releases must preserve the semantic and wire contract.
pub const SCHEMA_PATCH: u16 = 0;

/// Current complete schema version.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion =
    SchemaVersion::new(SCHEMA_MAJOR, SCHEMA_MINOR, SCHEMA_PATCH);

// =============================================================================
// Schema version
// =============================================================================

/// Version of the canonical Quantum IR serialization schema.
///
/// This is intentionally separate from `IrVersion`.
///
/// `IrVersion` describes semantic IR.
///
/// `SchemaVersion` describes the representation contract used to persist or
/// transport that IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SchemaVersion {
    /// Creates a schema version.
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

    /// Returns whether this is the current schema version.
    pub const fn is_current(self) -> bool {
        self.major == CURRENT_SCHEMA_VERSION.major
            && self.minor == CURRENT_SCHEMA_VERSION.minor
            && self.patch == CURRENT_SCHEMA_VERSION.patch
    }

    /// Returns whether both versions share the same major contract.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether this schema can consume `other` under the conservative
    /// compatibility policy.
    ///
    /// Older minor/patch releases of the same major are potentially readable.
    /// Future releases are not assumed readable.
    pub const fn supports(self, other: Self) -> bool {
        other.major == self.major
            && (
                other.minor < self.minor
                || (
                    other.minor == self.minor
                    && other.patch <= self.patch
                )
            )
    }

    /// Returns whether two schema versions are exactly equal.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether this version requires an explicit migration to reach
    /// `other`.
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

/// Kind of schema object described by this module.
///
/// Keeping these categories explicit prevents the document schema from
/// becoming coupled to one particular quantum-computation model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemaKind {
    /// Complete serialized Quantum IR document.
    Document,

    /// Canonical semantic payload.
    Payload,

    /// IR object schema.
    Object,

    /// Extension schema.
    Extension,

    /// Dialect schema.
    Dialect,
}

impl SchemaKind {
    /// Returns a stable wire-independent numeric discriminant.
    ///
    /// These values are part of the schema metadata contract and must not be
    /// reused for another meaning.
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

/// Canonical wire-level value category.
///
/// This describes representation, not semantic meaning.
///
/// For example:
///
/// - `QubitId` is semantically a quantum identity;
/// - its wire representation is `U64`.
///
/// This separation prevents the wire format from becoming a duplicate quantum
/// type system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WireKind {
    /// No value.
    Unit,

    /// Boolean encoded canonically as one byte.
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

    /// UTF-8 string.
    String,

    /// Raw bytes.
    Bytes,

    /// Length-prefixed sequence of values.
    Sequence,

    /// Length-prefixed map represented in canonical key order by its owner.
    Map,

    /// Nested schema-defined object.
    Object,

    /// Stable schema identifier.
    SchemaId,

    /// Stable IR object identity represented as a `u64`.
    Identity,

    /// Explicitly versioned extension payload.
    Extension,

    /// Canonical symbolic expression/value.
    Expression,
}

impl WireKind {
    /// Returns a stable numeric discriminant.
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

    /// Returns whether this kind has a fixed byte width.
    ///
    /// `None` means the value is variable length.
    pub const fn fixed_width(self) -> Option<u8> {
        match self {
            Self::Unit => Some(0),
            Self::Bool => Some(1),
            Self::U8 | Self::I8 => Some(1),
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

    /// Returns whether this wire kind can carry arbitrary extension data.
    pub const fn supports_extension_payload(self) -> bool {
        matches!(
            self,
            Self::Bytes | Self::Extension | Self::Object
        )
    }
}

// =============================================================================
// Field identity
// =============================================================================

/// Stable schema field identifier.
///
/// Field identifiers are not Rust field positions.
///
/// They are persistent wire-level identifiers.
///
/// A field's numeric identifier MUST NOT be reused for a different semantic
/// meaning within the same schema major version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldId(u32);

impl FieldId {
    /// Creates a field identifier.
    ///
    /// Zero is reserved and therefore rejected by schema validation rather than
    /// this constructor, because the constructor is intentionally infallible
    /// and useful in `const` declarations.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    pub const fn value(self) -> u32 {
        self.0
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
        write!(formatter, "field{}", self.0)
    }
}

// =============================================================================
// Field cardinality
// =============================================================================

/// Cardinality of a schema field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCardinality {
    /// Exactly one value must be present.
    Required,

    /// The field may be absent.
    Optional,

    /// The field may occur as a sequence.
    Repeated,
}

impl FieldCardinality {
    /// Returns whether the field must be present.
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns whether the field can occur multiple times.
    pub const fn is_repeated(self) -> bool {
        matches!(self, Self::Repeated)
    }
}

// =============================================================================
// Unknown-field policy
// =============================================================================

/// Policy governing fields unknown to a reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownFieldPolicy {
    /// Unknown fields are rejected.
    Reject,

    /// Unknown fields may be skipped when their complete wire representation
    /// can be safely skipped without interpreting semantic content.
    Skip,

    /// Unknown fields are retained for round-trip preservation.
    Preserve,
}

impl UnknownFieldPolicy {
    /// Returns whether unknown fields are accepted.
    pub const fn accepts_unknown(self) -> bool {
        !matches!(self, Self::Reject)
    }

    /// Returns whether unknown fields must be retained.
    pub const fn preserves_unknown(self) -> bool {
        matches!(self, Self::Preserve)
    }
}

// =============================================================================
// Compatibility policy
// =============================================================================

/// Compatibility policy for schema evolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityPolicy {
    /// Exact schema version required.
    Exact,

    /// Older versions of the same major may be consumed.
    BackwardCompatible,

    /// Older and newer minor/patch versions may interoperate when their field
    /// contracts explicitly permit it.
    Negotiated,

    /// Explicit migration is required.
    MigrationRequired,
}

impl CompatibilityPolicy {
    /// Returns whether a migration is mandatory by policy.
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::MigrationRequired)
    }
}

// =============================================================================
// Field descriptor
// =============================================================================

/// Immutable description of one schema field.
///
/// A field descriptor deliberately contains no Rust type parameters and no
/// references to semantic IR objects. This keeps the schema layer independent
/// from the implementation representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldDescriptor {
    /// Stable field identifier.
    pub id: FieldId,

    /// Human-readable stable field name.
    pub name: &'static str,

    /// Wire representation.
    pub wire_kind: WireKind,

    /// Field cardinality.
    pub cardinality: FieldCardinality,

    /// Whether unknown readers may safely skip this field.
    pub skippable: bool,

    /// Whether the field is part of canonical semantic identity.
    ///
    /// Non-semantic metadata must not affect canonical semantic hashes.
    pub semantic: bool,
}

impl FieldDescriptor {
    /// Creates a field descriptor.
    pub const fn new(
        id: FieldId,
        name: &'static str,
        wire_kind: WireKind,
        cardinality: FieldCardinality,
        skippable: bool,
        semantic: bool,
    ) -> Self {
        Self {
            id,
            name,
            wire_kind,
            cardinality,
            skippable,
            semantic,
        }
    }
}

// =============================================================================
// Schema descriptor
// =============================================================================

/// Immutable description of one complete schema.
///
/// The descriptor is deliberately represented as a slice of field descriptors
/// supplied by the owning schema. It therefore does not allocate and does not
/// require a registry or global mutable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaDescriptor {
    /// Stable schema identifier.
    pub id: &'static str,

    /// Schema category.
    pub kind: SchemaKind,

    /// Schema version.
    pub version: SchemaVersion,

    /// Compatibility policy.
    pub compatibility: CompatibilityPolicy,

    /// Unknown-field policy.
    pub unknown_fields: UnknownFieldPolicy,

    /// Whether field order is semantically meaningful.
    ///
    /// Canonical schema definitions should normally be field-ID ordered.
    pub ordered_fields: bool,

    /// Static field descriptors.
    pub fields: &'static [FieldDescriptor],
}

impl SchemaDescriptor {
    /// Creates a schema descriptor.
    pub const fn new(
        id: &'static str,
        kind: SchemaKind,
        version: SchemaVersion,
        compatibility: CompatibilityPolicy,
        unknown_fields: UnknownFieldPolicy,
        ordered_fields: bool,
        fields: &'static [FieldDescriptor],
    ) -> Self {
        Self {
            id,
            kind,
            version,
            compatibility,
            unknown_fields,
            ordered_fields,
            fields,
        }
    }

    /// Finds a field by stable identifier.
    pub fn field(&self, id: FieldId) -> Option<&FieldDescriptor> {
        let mut index = 0;

        while index < self.fields.len() {
            let field = &self.fields[index];

            if field.id == id {
                return Some(field);
            }

            index += 1;
        }

        None
    }

    /// Finds a field by its stable schema name.
    pub fn field_by_name(
        &self,
        name: &str,
    ) -> Option<&FieldDescriptor> {
        let mut index = 0;

        while index < self.fields.len() {
            let field = &self.fields[index];

            if field.name == name {
                return Some(field);
            }

            index += 1;
        }

        None
    }

    /// Returns the number of declared fields.
    pub const fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the schema declares the supplied field.
    pub fn contains_field(&self, id: FieldId) -> bool {
        self.field(id).is_some()
    }

    /// Validates the schema descriptor itself.
    ///
    /// This checks structural schema invariants without inspecting semantic IR
    /// values.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if self.id.is_empty() {
            return Err(SchemaError::EmptySchemaId);
        }

        if self.id.as_bytes().contains(&0) {
            return Err(SchemaError::InvalidSchemaId);
        }

        if self.fields.is_empty() {
            return Err(SchemaError::NoFields {
                schema: self.id,
            });
        }

        let mut index = 0;

        while index < self.fields.len() {
            let field = &self.fields[index];

            if field.id.value() == 0 {
                return Err(SchemaError::ReservedFieldId {
                    schema: self.id,
                    field: field.name,
                });
            }

            if field.name.is_empty() {
                return Err(SchemaError::EmptyFieldName {
                    schema: self.id,
                });
            }

            if field.name.as_bytes().contains(&0) {
                return Err(SchemaError::InvalidFieldName {
                    schema: self.id,
                    field: field.name,
                });
            }

            index += 1;
        }

        // Duplicate field IDs are prohibited.
        let mut left = 0;

        while left < self.fields.len() {
            let mut right = left + 1;

            while right < self.fields.len() {
                if self.fields[left].id == self.fields[right].id {
                    return Err(SchemaError::DuplicateFieldId {
                        schema: self.id,
                        field_id: self.fields[left].id,
                    });
                }

                right += 1;
            }

            left += 1;
        }

        // Canonical schema descriptors must be field-ID ordered.
        if self.ordered_fields {
            let mut index = 1;

            while index < self.fields.len() {
                if self.fields[index - 1].id > self.fields[index].id {
                    return Err(SchemaError::FieldsNotCanonical {
                        schema: self.id,
                    });
                }

                index += 1;
            }
        }

        Ok(())
    }
}

// =============================================================================
// Canonical document schema
// =============================================================================
//
// These fields describe the stable outer document contract used by
// serialization.rs.
//
// The actual byte framing remains owned by serialization.rs.
// This table gives the semantic schema identity to that framing.

/// Canonical document field: serialization format version.
pub const FIELD_FORMAT_VERSION: FieldId = FieldId::new(1);

/// Canonical document field: semantic IR major version.
pub const FIELD_IR_MAJOR: FieldId = FieldId::new(2);

/// Canonical document field: semantic IR minor version.
pub const FIELD_IR_MINOR: FieldId = FieldId::new(3);

/// Canonical document field: semantic IR patch version.
pub const FIELD_IR_PATCH: FieldId = FieldId::new(4);

/// Canonical document field: payload byte length.
pub const FIELD_PAYLOAD_LENGTH: FieldId = FieldId::new(5);

/// Canonical document field: payload integrity checksum.
pub const FIELD_PAYLOAD_CHECKSUM: FieldId = FieldId::new(6);

/// Canonical document field: schema identifier.
pub const FIELD_SCHEMA_ID: FieldId = FieldId::new(7);

/// Canonical document field: serialized semantic payload.
pub const FIELD_PAYLOAD: FieldId = FieldId::new(8);

/// Canonical fields of the document schema.
///
/// These descriptors describe the persistent contract; they do not cause the
/// binary serializer to use a different framing layout.
pub static DOCUMENT_FIELDS: [FieldDescriptor; 8] = [
    FieldDescriptor::new(
        FIELD_FORMAT_VERSION,
        "format_version",
        WireKind::U16,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_IR_MAJOR,
        "ir_major",
        WireKind::U16,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_IR_MINOR,
        "ir_minor",
        WireKind::U16,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_IR_PATCH,
        "ir_patch",
        WireKind::U16,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_PAYLOAD_LENGTH,
        "payload_length",
        WireKind::U64,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_PAYLOAD_CHECKSUM,
        "payload_checksum",
        WireKind::U32,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_SCHEMA_ID,
        "schema_id",
        WireKind::SchemaId,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_PAYLOAD,
        "payload",
        WireKind::Bytes,
        FieldCardinality::Required,
        false,
        true,
    ),
];

/// Canonical document schema descriptor.
///
/// The descriptor is immutable and deterministic.
pub static DOCUMENT_SCHEMA: SchemaDescriptor = SchemaDescriptor::new(
    DOCUMENT_SCHEMA_ID,
    SchemaKind::Document,
    CURRENT_SCHEMA_VERSION,
    CompatibilityPolicy::BackwardCompatible,
    UnknownFieldPolicy::Reject,
    true,
    &DOCUMENT_FIELDS,
);

// =============================================================================
// Canonical payload schema
// =============================================================================

/// Canonical payload field: semantic object type.
pub const FIELD_OBJECT_TYPE: FieldId = FieldId::new(1);

/// Canonical payload field: semantic object version.
pub const FIELD_OBJECT_VERSION: FieldId = FieldId::new(2);

/// Canonical payload field: object identity.
pub const FIELD_OBJECT_ID: FieldId = FieldId::new(3);

/// Canonical payload field: semantic object data.
pub const FIELD_OBJECT_DATA: FieldId = FieldId::new(4);

/// Canonical payload field: extension data.
pub const FIELD_EXTENSIONS: FieldId = FieldId::new(5);

/// Canonical payload fields.
///
/// The payload remains intentionally generic. Concrete IR modules define their
/// own object schemas and extensions rather than requiring this file to know
/// every future quantum operation.
pub static PAYLOAD_FIELDS: [FieldDescriptor; 5] = [
    FieldDescriptor::new(
        FIELD_OBJECT_TYPE,
        "object_type",
        WireKind::SchemaId,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_OBJECT_VERSION,
        "object_version",
        WireKind::Object,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_OBJECT_ID,
        "object_id",
        WireKind::Identity,
        FieldCardinality::Optional,
        true,
        true,
    ),
    FieldDescriptor::new(
        FIELD_OBJECT_DATA,
        "object_data",
        WireKind::Object,
        FieldCardinality::Required,
        false,
        true,
    ),
    FieldDescriptor::new(
        FIELD_EXTENSIONS,
        "extensions",
        WireKind::Extension,
        FieldCardinality::Repeated,
        true,
        false,
    ),
];

/// Canonical payload schema descriptor.
pub static PAYLOAD_SCHEMA: SchemaDescriptor = SchemaDescriptor::new(
    PAYLOAD_SCHEMA_ID,
    SchemaKind::Payload,
    CURRENT_SCHEMA_VERSION,
    CompatibilityPolicy::BackwardCompatible,
    UnknownFieldPolicy::Preserve,
    true,
    &PAYLOAD_FIELDS,
);

// =============================================================================
// Stable object schema IDs
// =============================================================================
//
// These are identifiers for existing conceptual IR boundaries. They do not
// force every future object into a closed enum.
//
// New domains can register additional IDs in their owning serialization
// modules.

/// Stable schema identifier for a quantum program.
pub const PROGRAM_SCHEMA_ID: &str =
    "zamani.quantum.ir.program";

/// Stable schema identifier for a quantum circuit.
pub const CIRCUIT_SCHEMA_ID: &str =
    "zamani.quantum.ir.circuit";

/// Stable schema identifier for a quantum operation.
pub const OPERATION_SCHEMA_ID: &str =
    "zamani.quantum.ir.operation";

/// Stable schema identifier for a quantum gate invocation.
pub const GATE_SCHEMA_ID: &str =
    "zamani.quantum.ir.gate";

/// Stable schema identifier for a quantum measurement.
pub const MEASUREMENT_SCHEMA_ID: &str =
    "zamani.quantum.ir.measurement";

/// Stable schema identifier for a logical/physical qubit identity object.
pub const QUBIT_SCHEMA_ID: &str =
    "zamani.quantum.ir.qubit";

/// Stable schema identifier for a classical value.
pub const CLASSICAL_VALUE_SCHEMA_ID: &str =
    "zamani.quantum.ir.classical.value";

/// Stable schema identifier for a symbolic parameter.
pub const PARAMETER_SCHEMA_ID: &str =
    "zamani.quantum.ir.parameter";

/// Stable schema identifier for a control-flow object.
pub const CONTROL_FLOW_SCHEMA_ID: &str =
    "zamani.quantum.ir.control_flow";

/// Stable schema identifier for a pulse object.
pub const PULSE_SCHEMA_ID: &str =
    "zamani.quantum.ir.pulse";

/// Stable schema identifier for a waveform.
pub const WAVEFORM_SCHEMA_ID: &str =
    "zamani.quantum.ir.waveform";

/// Stable schema identifier for a frame.
pub const FRAME_SCHEMA_ID: &str =
    "zamani.quantum.ir.frame";

/// Stable schema identifier for a channel.
pub const CHANNEL_SCHEMA_ID: &str =
    "zamani.quantum.ir.channel";

/// Stable schema identifier for a timing object.
pub const TIMING_SCHEMA_ID: &str =
    "zamani.quantum.ir.timing";

/// Stable schema identifier for an abstract resource.
pub const RESOURCE_SCHEMA_ID: &str =
    "zamani.quantum.ir.resource";

/// Stable schema identifier for a capability requirement.
pub const CAPABILITY_SCHEMA_ID: &str =
    "zamani.quantum.ir.capability";

/// Stable schema identifier for a mapping record.
pub const MAPPING_SCHEMA_ID: &str =
    "zamani.quantum.ir.mapping";

/// Stable schema identifier for a schedule.
pub const SCHEDULE_SCHEMA_ID: &str =
    "zamani.quantum.ir.schedule";

/// Stable schema identifier for provenance.
pub const PROVENANCE_SCHEMA_ID: &str =
    "zamani.quantum.ir.provenance";

/// Stable schema identifier for an extension object.
pub const EXTENSION_SCHEMA_ID: &str =
    "zamani.quantum.ir.extension";

// =============================================================================
// Schema registry view
// =============================================================================

/// Static collection of the foundational schema identifiers.
///
/// This is intentionally a slice rather than a mutable registry.
///
/// The canonical schema layer must not contain global mutable state.
///
/// Future dialects/extensions should be registered through their own
/// serialization infrastructure.
pub static CORE_SCHEMA_IDS: [&str; 18] = [
    PROGRAM_SCHEMA_ID,
    CIRCUIT_SCHEMA_ID,
    OPERATION_SCHEMA_ID,
    GATE_SCHEMA_ID,
    MEASUREMENT_SCHEMA_ID,
    QUBIT_SCHEMA_ID,
    CLASSICAL_VALUE_SCHEMA_ID,
    PARAMETER_SCHEMA_ID,
    CONTROL_FLOW_SCHEMA_ID,
    PULSE_SCHEMA_ID,
    WAVEFORM_SCHEMA_ID,
    FRAME_SCHEMA_ID,
    CHANNEL_SCHEMA_ID,
    TIMING_SCHEMA_ID,
    RESOURCE_SCHEMA_ID,
    CAPABILITY_SCHEMA_ID,
    MAPPING_SCHEMA_ID,
    SCHEDULE_SCHEMA_ID,
];

/// Returns whether a schema identifier is one of the foundational canonical
/// schemas.
///
/// This function does not reject unknown schema IDs. Unknown schemas are a
/// normal requirement for forward-compatible dialects and extensions.
pub fn is_core_schema_id(id: &str) -> bool {
    let mut index = 0;

    while index < CORE_SCHEMA_IDS.len() {
        if CORE_SCHEMA_IDS[index] == id {
            return true;
        }

        index += 1;
    }

    false
}

// =============================================================================
// Schema errors
// =============================================================================

/// Errors produced while validating schema definitions or compatibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    /// Schema identifier is empty.
    EmptySchemaId,

    /// Schema identifier contains a prohibited NUL byte.
    InvalidSchemaId,

    /// Schema contains no fields.
    NoFields {
        /// Schema identifier.
        schema: &'static str,
    },

    /// A field uses reserved identifier zero.
    ReservedFieldId {
        /// Schema identifier.
        schema: &'static str,

        /// Field name.
        field: &'static str,
    },

    /// Field name is empty.
    EmptyFieldName {
        /// Schema identifier.
        schema: &'static str,
    },

    /// Field name contains a NUL byte.
    InvalidFieldName {
        /// Schema identifier.
        schema: &'static str,

        /// Field name.
        field: &'static str,
    },

    /// Two fields use the same persistent field identifier.
    DuplicateFieldId {
        /// Schema identifier.
        schema: &'static str,

        /// Duplicate identifier.
        field_id: FieldId,
    },

    /// Fields are not in canonical identifier order.
    FieldsNotCanonical {
        /// Schema identifier.
        schema: &'static str,
    },

    /// Schema versions are incompatible.
    IncompatibleVersion {
        /// Reader version.
        reader: SchemaVersion,

        /// Writer version.
        writer: SchemaVersion,
    },

    /// A required field is missing.
    MissingRequiredField {
        /// Schema identifier.
        schema: &'static str,

        /// Missing field.
        field: &'static str,
    },

    /// A field's wire kind does not match its descriptor.
    WireKindMismatch {
        /// Schema identifier.
        schema: &'static str,

        /// Field name.
        field: &'static str,
    },

    /// An unknown field cannot be accepted under the active policy.
    UnknownField {
        /// Schema identifier.
        schema: &'static str,

        /// Unknown field identifier.
        field_id: FieldId,
    },
}

impl fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchemaId => {
                write!(formatter, "schema identifier cannot be empty")
            }

            Self::InvalidSchemaId => {
                write!(
                    formatter,
                    "schema identifier contains an invalid NUL byte"
                )
            }

            Self::NoFields { schema } => {
                write!(
                    formatter,
                    "schema `{schema}` must declare at least one field"
                )
            }

            Self::ReservedFieldId { schema, field } => {
                write!(
                    formatter,
                    "schema `{schema}` field `{field}` uses reserved field ID 0"
                )
            }

            Self::EmptyFieldName { schema } => {
                write!(
                    formatter,
                    "schema `{schema}` contains an empty field name"
                )
            }

            Self::InvalidFieldName { schema, field } => {
                write!(
                    formatter,
                    "schema `{schema}` field `{field}` contains an invalid NUL byte"
                )
            }

            Self::DuplicateFieldId {
                schema,
                field_id,
            } => {
                write!(
                    formatter,
                    "schema `{schema}` contains duplicate field ID {}",
                    field_id.value()
                )
            }

            Self::FieldsNotCanonical { schema } => {
                write!(
                    formatter,
                    "schema `{schema}` fields are not in canonical ID order"
                )
            }

            Self::IncompatibleVersion { reader, writer } => {
                write!(
                    formatter,
                    "schema version {reader} cannot consume schema version {writer}"
                )
            }

            Self::MissingRequiredField { schema, field } => {
                write!(
                    formatter,
                    "schema `{schema}` is missing required field `{field}`"
                )
            }

            Self::WireKindMismatch { schema, field } => {
                write!(
                    formatter,
                    "schema `{schema}` field `{field}` has an incompatible wire kind"
                )
            }

            Self::UnknownField {
                schema,
                field_id,
            } => {
                write!(
                    formatter,
                    "schema `{schema}` does not define field ID {}",
                    field_id.value()
                )
            }
        }
    }
}

impl std::error::Error for SchemaError {}

// =============================================================================
// Compatibility helpers
// =============================================================================

/// Checks whether a reader schema can consume a writer schema.
pub const fn is_schema_version_compatible(
    reader: SchemaVersion,
    writer: SchemaVersion,
) -> bool {
    reader.supports(writer)
}

/// Validates schema-version compatibility and returns a structured error when
/// incompatible.
pub fn require_schema_compatibility(
    reader: SchemaVersion,
    writer: SchemaVersion,
) -> Result<(), SchemaError> {
    if reader.supports(writer) {
        Ok(())
    } else {
        Err(SchemaError::IncompatibleVersion {
            reader,
            writer,
        })
    }
}

/// Returns the canonical current schema version.
///
/// This function is useful to downstream serialization modules because it
/// avoids duplicating the current-version constant.
pub const fn current_schema_version() -> SchemaVersion {
    CURRENT_SCHEMA_VERSION
}

/// Returns the stable schema namespace.
pub const fn schema_namespace() -> &'static str {
    SCHEMA_NAMESPACE
}

// =============================================================================
// Canonical schema validation
// =============================================================================

/// Validates all foundational schema descriptors.
///
/// This function is intended to be called from serialization tests and
/// integration tests.
///
/// It performs no allocation and does not mutate global state.
pub fn validate_core_schemas() -> Result<(), SchemaError> {
    DOCUMENT_SCHEMA.validate()?;
    PAYLOAD_SCHEMA.validate()?;
    Ok(())
}

// =============================================================================
// Semantic identity documentation helpers
// =============================================================================
//
// The schema deliberately does not import `QubitId` or `PhysicalQubitId`.
//
// Their semantic ownership remains:
//
//     quantum::ir::qubit::QubitId
//     quantum::ir::qubit::PhysicalQubitId
//
// Their serialization is represented generically as `WireKind::Identity`.
//
// This is intentional.
//
// If schema.rs imported and stored concrete qubit values, the schema layer
// would become coupled to individual IR objects and could no longer describe
// arbitrary serialized object schemas independently.
//
// The owning qubit serializer must therefore encode/decode the canonical
// `quantum::ir::qubit` identities while using this schema contract.
//
// =============================================================================
// Canonical schema manifest
// =============================================================================

/// Stable manifest information for the canonical schema.
///
/// This is deliberately a compact value type so callers can expose schema
/// information through diagnostics, introspection, tooling, or compatibility
/// negotiation without exposing mutable global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaManifest {
    /// Namespace.
    pub namespace: &'static str,

    /// Complete document schema identifier.
    pub document_schema: &'static str,

    /// Payload schema identifier.
    pub payload_schema: &'static str,

    /// Current schema version.
    pub version: SchemaVersion,
}

impl SchemaManifest {
    /// Returns the canonical manifest.
    pub const fn current() -> Self {
        Self {
            namespace: SCHEMA_NAMESPACE,
            document_schema: DOCUMENT_SCHEMA_ID,
            payload_schema: PAYLOAD_SCHEMA_ID,
            version: CURRENT_SCHEMA_VERSION,
        }
    }
}

impl Default for SchemaManifest {
    fn default() -> Self {
        Self::current()
    }
}

// =============================================================================
// Schema evolution rules
// =============================================================================

/// Stable schema-evolution rules.
///
/// These are documentation-backed constants that downstream tooling can use
/// when reporting compatibility decisions.
pub const EVOLUTION_RULES: &[&str] = &[
    "A field identifier must never be reused for a different semantic meaning.",
    "A schema major-version change may be breaking.",
    "A schema minor-version change may only add explicitly compatible semantics.",
    "A schema patch-version change must preserve the existing contract.",
    "Required fields cannot be removed without a major-version decision.",
    "Existing field wire kinds cannot change incompatibly.",
    "Unknown extensions must not silently change semantic meaning.",
    "Canonical semantic ordering must remain deterministic.",
    "Semantic sequences must never be reordered merely for serialization convenience.",
    "Machine capacity must never be encoded as a schema-wide quantum limit.",
    "Physical hardware identifiers must not replace logical IR identities.",
    "Serialization schema version must remain independent from semantic IR version.",
];

// =============================================================================
// Reserved field range
// =============================================================================
//
// The schema reserves only a small identifier range for structural fields.
// It does NOT reserve a maximum field count for quantum operations.
//
// All ordinary schema fields may use any non-zero u32 FieldId that their owning
// schema can safely define without collisions.

/// Lowest reserved structural field identifier.
pub const RESERVED_FIELD_ID_MIN: u32 = 1;

/// Highest reserved structural field identifier.
///
/// IDs above this value are available to owning object schemas.
pub const RESERVED_FIELD_ID_MAX: u32 = 1024;

/// Returns whether a field ID is in the structural reserved range.
pub const fn is_reserved_structural_field(id: FieldId) -> bool {
    let value = id.value();

    value >= RESERVED_FIELD_ID_MIN
        && value <= RESERVED_FIELD_ID_MAX
}

// =============================================================================
// Schema ID validation
// =============================================================================

/// Validates a schema identifier.
///
/// Schema identifiers are intentionally text-based so new dialects and
/// extensions do not require modifying a central enum.
pub fn validate_schema_id(id: &str) -> Result<(), SchemaError> {
    if id.is_empty() {
        return Err(SchemaError::EmptySchemaId);
    }

    if id.as_bytes().contains(&0) {
        return Err(SchemaError::InvalidSchemaId);
    }

    Ok(())
}

// =============================================================================
// Stable schema contract
// =============================================================================

/// Returns the canonical document schema descriptor.
///
/// This function returns the immutable descriptor by reference and therefore
/// does not allocate.
pub const fn document_schema() -> &'static SchemaDescriptor {
    &DOCUMENT_SCHEMA
}

/// Returns the canonical payload schema descriptor.
///
/// This function returns the immutable descriptor by reference and therefore
/// does not allocate.
pub const fn payload_schema() -> &'static SchemaDescriptor {
    &PAYLOAD_SCHEMA
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_schema_version_is_stable() {
        assert_eq!(
            CURRENT_SCHEMA_VERSION,
            SchemaVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn schema_version_supports_older_patch() {
        let current = SchemaVersion::new(1, 0, 2);
        let older = SchemaVersion::new(1, 0, 1);

        assert!(current.supports(older));
    }

    #[test]
    fn schema_version_rejects_future_major() {
        let current = SchemaVersion::new(1, 0, 0);
        let future = SchemaVersion::new(2, 0, 0);

        assert!(!current.supports(future));
    }

    #[test]
    fn schema_version_rejects_future_minor() {
        let current = SchemaVersion::new(1, 0, 0);
        let future = SchemaVersion::new(1, 1, 0);

        assert!(!current.supports(future));
    }

    #[test]
    fn field_ids_are_stable() {
        assert_eq!(FIELD_FORMAT_VERSION.value(), 1);
        assert_eq!(FIELD_PAYLOAD.value(), 8);
    }

    #[test]
    fn document_schema_is_valid() {
        assert!(DOCUMENT_SCHEMA.validate().is_ok());
    }

    #[test]
    fn payload_schema_is_valid() {
        assert!(PAYLOAD_SCHEMA.validate().is_ok());
    }

    #[test]
    fn all_core_schemas_are_valid() {
        assert!(validate_core_schemas().is_ok());
    }

    #[test]
    fn duplicate_field_ids_are_rejected() {
        static FIELDS: [FieldDescriptor; 2] = [
            FieldDescriptor::new(
                FieldId::new(1),
                "a",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            ),
            FieldDescriptor::new(
                FieldId::new(1),
                "b",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            ),
        ];

        static SCHEMA: SchemaDescriptor = SchemaDescriptor::new(
            "zamani.test.duplicate",
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::BackwardCompatible,
            UnknownFieldPolicy::Reject,
            true,
            &FIELDS,
        );

        assert!(matches!(
            SCHEMA.validate(),
            Err(SchemaError::DuplicateFieldId { .. })
        ));
    }

    #[test]
    fn non_canonical_field_order_is_rejected() {
        static FIELDS: [FieldDescriptor; 2] = [
            FieldDescriptor::new(
                FieldId::new(2),
                "b",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            ),
            FieldDescriptor::new(
                FieldId::new(1),
                "a",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            ),
        ];

        static SCHEMA: SchemaDescriptor = SchemaDescriptor::new(
            "zamani.test.order",
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::BackwardCompatible,
            UnknownFieldPolicy::Reject,
            true,
            &FIELDS,
        );

        assert!(matches!(
            SCHEMA.validate(),
            Err(SchemaError::FieldsNotCanonical { .. })
        ));
    }

    #[test]
    fn field_lookup_is_deterministic() {
        let field = DOCUMENT_SCHEMA
            .field(FIELD_PAYLOAD)
            .expect("payload field must exist");

        assert_eq!(field.name, "payload");
        assert_eq!(field.wire_kind, WireKind::Bytes);
    }

    #[test]
    fn field_lookup_by_name_is_deterministic() {
        let field = DOCUMENT_SCHEMA
            .field_by_name("payload_checksum")
            .expect("checksum field must exist");

        assert_eq!(
            field.id,
            FIELD_PAYLOAD_CHECKSUM
        );
    }

    #[test]
    fn unknown_schema_ids_are_allowed() {
        assert!(!is_core_schema_id(
            "zamani.quantum.future.architecture"
        ));
    }

    #[test]
    fn unknown_field_policy_is_explicit() {
        assert!(!UnknownFieldPolicy::Reject.accepts_unknown());
        assert!(UnknownFieldPolicy::Skip.accepts_unknown());
        assert!(UnknownFieldPolicy::Preserve.accepts_unknown());
        assert!(UnknownFieldPolicy::Preserve.preserves_unknown());
    }

    #[test]
    fn wire_kind_widths_are_correct() {
        assert_eq!(WireKind::Bool.fixed_width(), Some(1));
        assert_eq!(WireKind::U16.fixed_width(), Some(2));
        assert_eq!(WireKind::U32.fixed_width(), Some(4));
        assert_eq!(WireKind::U64.fixed_width(), Some(8));
        assert_eq!(WireKind::String.fixed_width(), None);
    }

    #[test]
    fn structural_field_range_is_explicit() {
        assert!(is_reserved_structural_field(
            FieldId::new(1)
        ));

        assert!(is_reserved_structural_field(
            FieldId::new(1024)
        ));

        assert!(!is_reserved_structural_field(
            FieldId::new(1025)
        ));
    }

    #[test]
    fn schema_manifest_is_stable() {
        let manifest = SchemaManifest::current();

        assert_eq!(
            manifest.namespace,
            SCHEMA_NAMESPACE
        );

        assert_eq!(
            manifest.document_schema,
            DOCUMENT_SCHEMA_ID
        );

        assert_eq!(
            manifest.payload_schema,
            PAYLOAD_SCHEMA_ID
        );

        assert_eq!(
            manifest.version,
            CURRENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn schema_ids_validate() {
        assert!(
            validate_schema_id(
                "zamani.quantum.ir.example"
            )
            .is_ok()
        );

        assert!(
            validate_schema_id("")
                .is_err()
        );

        assert!(
            validate_schema_id("bad\0schema")
                .is_err()
        );
    }
}