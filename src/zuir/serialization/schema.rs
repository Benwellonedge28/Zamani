#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Serialization Schema Contract
//!
//! Canonical path: `src/zuir/serialization/schema.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: `unsafe` Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Define the versioned, domain-neutral schema contract used to describe
//! serialized ZUIR documents, objects, dialects, and extensions.
//!
//! This module specifies how serialized data is described. It does not encode
//! or decode bytes, interpret computational semantics, or execute operations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `src/zuir/schema/extensions.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/operation.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/grammar.md`
//!
//! The ZUIR specification governs representation semantics. The grammar
//! governs syntax. The semantic model governs source meaning. This module
//! owns only the serialization schema contract.
//!
//! ## OWNS
//!
//! - Stable schema identifiers and schema versions.
//! - Field identifiers, wire kinds, and field cardinality.
//! - Schema descriptors and their structural validation.
//! - Explicit unknown-field and compatibility policies.
//! - Deterministic schema manifests.
//! - Configurable schema-validation resource budgets.
//!
//! ## DOES_NOT_OWN
//!
//! - AST nodes, lexical tokens, parser productions, or semantic analysis.
//! - ZUIR program, operation, region, block, or value semantics.
//! - Concrete quantum, classical, HDL, tensor, or distributed operations.
//! - Encoding, decoding, binary framing, checksums, or cryptographic hashes.
//! - Executable extension loading or extension authorization.
//! - Physical hardware capacities or target-specific memory layouts.
//! - Schema migration implementation or compatibility negotiation transport.
//!
//! ## INTEGRATION
//!
//! `serialization/mod.rs` exposes this module.
//! `serialization/encode.rs` consumes validated schema descriptors.
//! `serialization/decode.rs` checks versions and field policies before
//! interpreting untrusted serialized values.
//! `serialization/validation.rs` may compose these checks with IR validation.
//! `serialization/migration.rs`, if introduced, owns actual migrations.
//! `fingerprint/` owns stable content fingerprints.
//!
//! The schema layer must not depend on any of those downstream modules.
//!
//! ## SCALABILITY
//!
//! There are no fixed limits on program size, operation count, qubits,
//! devices, registers, tensor rank, or hardware resources.
//!
//! Descriptor collections are dynamically sized. A caller may supply explicit
//! validation budgets to protect a process from excessive input. Such budgets
//! are operational safeguards, not universal representation limits.
//!
//! No finite implementation can guarantee infinite memory or execution time.
//!
//! ## DETERMINISM
//!
//! Canonical field order is ascending `FieldId`. Duplicate identifiers and
//! noncanonical field ordering are rejected. A canonical manifest must be
//! produced by a deterministic renderer over the validated descriptor.
//!
//! Semantic sequences must never be reordered merely to canonicalize bytes.
//!
//! ## FREEZE CRITERIA
//!
//! Before freezing this file, the implementation must:
//!
//! - Compile with the actual serialization module and crate dependencies.
//! - Pass the tests in this file and integration tests for encoder/decoder.
//! - Establish the canonical manifest encoding and its compatibility policy.
//! - Verify that unknown-field handling cannot silently discard semantic data.
//! - Verify that resource budgets are checked before expensive processing.
//! - Confirm that the public schema APIs agree with ZUIR-SPEC.md.
//!
//! This file defines schema metadata; it is not a claim that serialization
//! or the full ZUIR pipeline is already production-ready.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

/// Stable namespace for ZUIR serialization schemas.
pub const SCHEMA_NAMESPACE: &str = "zamani.zuir.serialization";

/// Stable identifier for the complete serialized ZUIR document contract.
pub const DOCUMENT_SCHEMA_ID: &str = "zamani.zuir.serialization.document";

/// Stable identifier for the serialized ZUIR payload contract.
pub const PAYLOAD_SCHEMA_ID: &str = "zamani.zuir.serialization.payload";

/// Initial schema major version.
pub const SCHEMA_MAJOR: u32 = 1;

/// Initial schema minor version.
pub const SCHEMA_MINOR: u32 = 0;

/// Initial schema patch version.
pub const SCHEMA_PATCH: u32 = 0;

/// Current schema version.
///
/// Change these values only through a reviewed schema evolution.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion =
    SchemaVersion::new(SCHEMA_MAJOR, SCHEMA_MINOR, SCHEMA_PATCH);

/// Version of the serialization schema.
///
/// This is distinct from the semantic IR version, language version, compiler
/// version, dialect versions, and target/runtime versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SchemaVersion {
    /// Creates a version from its components.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor component.
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch component.
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns the initial/current version declared by this module.
    pub const fn current() -> Self {
        CURRENT_SCHEMA_VERSION
    }

    /// Returns whether two versions have the same major version.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether both versions are identical.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Conservative compatibility check.
    ///
    /// `reader.supports(writer)` is true only when the writer's schema is no
    /// newer than the reader's schema and both have the same major version.
    ///
    /// This is a necessary version check, not proof that individual fields
    /// or semantic extensions are compatible.
    pub const fn supports(self, writer: Self) -> bool {
        self.major == writer.major
            && (writer.minor < self.minor
                || (writer.minor == self.minor && writer.patch <= self.patch))
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Stable, qualified schema identifier.
///
/// Identifiers are textual so independently developed dialects can introduce
/// new schema identities without changing a central Rust enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaId(String);

impl SchemaId {
    /// Creates and validates a schema identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, SchemaError> {
        let value = value.into();
        validate_identifier(&value, IdentifierKind::Schema)?;
        Ok(Self(value))
    }

    /// Returns the identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned text.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Persistent identifier of a field within a schema.
///
/// Zero is reserved and invalid. IDs must not be reused for a different
/// meaning within a compatible schema lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldId(u32);

impl FieldId {
    /// Constructs a nonzero field identifier.
    pub const fn new(value: u32) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the underlying identifier.
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for FieldId {
    type Error = SchemaError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(SchemaError::ReservedFieldId(0))
    }
}

impl fmt::Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Kind of schema descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchemaKind {
    /// Complete serialized document.
    Document,
    /// Document payload.
    Payload,
    /// An individual schema-defined object.
    Object,
    /// A dialect schema.
    Dialect,
    /// An extension-owned schema.
    Extension,
}

/// Canonical wire-level value category.
///
/// Wire categories describe representation, not semantic meaning. A future
/// quantum operation, tensor type, or HDL construct does not need a new
/// `WireKind` if its value can be represented by an existing category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WireKind {
    Unit,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bytes,
    Sequence,
    Map,
    Object,
    SchemaId,
    Identity,
    Extension,
    Expression,
}

impl WireKind {
    /// Stable schema metadata discriminant.
    ///
    /// These values must not be reused for different meanings. They are
    /// metadata identifiers, not a complete binary encoding specification.
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

    /// Returns the fixed wire width for primitive fixed-width values.
    ///
    /// `None` means the encoding contract must define variable-length framing.
    pub const fn fixed_width(self) -> Option<usize> {
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
}

/// Number of values allowed for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FieldCardinality {
    /// Exactly one value is required.
    Required,
    /// At most one value may be present.
    Optional,
    /// Zero or more values may be present.
    Repeated,
}

impl FieldCardinality {
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    pub const fn is_repeated(self) -> bool {
        matches!(self, Self::Repeated)
    }
}

/// Policy for fields unknown to a reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownFieldPolicy {
    /// Reject unknown fields.
    Reject,
    /// Skip unknown fields only when the decoder can safely skip their entire
    /// framed representation.
    Skip,
    /// Preserve the complete opaque representation for lossless forwarding.
    Preserve,
}

/// Schema compatibility policy.
///
/// This policy supplements version comparison; it does not replace field
/// validation or semantic compatibility checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityPolicy {
    /// The schema version must match exactly.
    Exact,
    /// Older schemas of the same major version may be considered.
    BackwardCompatible,
    /// The caller must negotiate compatibility using additional information.
    Negotiated,
    /// An explicit migration is required.
    MigrationRequired,
}

/// Descriptor for one persistent field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    /// Stable wire field ID.
    pub id: FieldId,
    /// Human-readable field name.
    pub name: String,
    /// Wire representation category.
    pub wire_kind: WireKind,
    /// Field presence/multiplicity rule.
    pub cardinality: FieldCardinality,
    /// Whether a compatible reader can skip the complete encoded value.
    pub skippable: bool,
    /// Whether the field contributes to semantic identity.
    ///
    /// This flag is descriptive metadata. The canonical serializer and
    /// fingerprint implementation must define how they use it.
    pub semantic: bool,
}

impl FieldDescriptor {
    /// Constructs a field descriptor.
    pub fn new(
        id: FieldId,
        name: impl Into<String>,
        wire_kind: WireKind,
        cardinality: FieldCardinality,
        skippable: bool,
        semantic: bool,
    ) -> Result<Self, SchemaError> {
        let name = name.into();
        validate_identifier(&name, IdentifierKind::Field)?;

        Ok(Self {
            id,
            name,
            wire_kind,
            cardinality,
            skippable,
            semantic,
        })
    }
}

/// Immutable-by-convention schema descriptor.
///
/// The fields are private so construction and mutation cannot bypass the
/// validation contract. Descriptors can be created dynamically for
/// independently developed dialects and extensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaDescriptor {
    id: SchemaId,
    kind: SchemaKind,
    version: SchemaVersion,
    compatibility: CompatibilityPolicy,
    unknown_fields: UnknownFieldPolicy,
    canonical: bool,
    fields: Vec<FieldDescriptor>,
}

impl SchemaDescriptor {
    /// Creates a descriptor. Call `validate` before using it to interpret
    /// serialized input.
    pub fn new(
        id: SchemaId,
        kind: SchemaKind,
        version: SchemaVersion,
        compatibility: CompatibilityPolicy,
        unknown_fields: UnknownFieldPolicy,
        canonical: bool,
        fields: Vec<FieldDescriptor>,
    ) -> Self {
        Self {
            id,
            kind,
            version,
            compatibility,
            unknown_fields,
            canonical,
            fields,
        }
    }

    pub fn id(&self) -> &SchemaId {
        &self.id
    }

    pub const fn kind(&self) -> SchemaKind {
        self.kind
    }

    pub const fn version(&self) -> SchemaVersion {
        self.version
    }

    pub const fn compatibility_policy(&self) -> CompatibilityPolicy {
        self.compatibility
    }

    pub const fn unknown_field_policy(&self) -> UnknownFieldPolicy {
        self.unknown_fields
    }

    pub const fn is_canonical(&self) -> bool {
        self.canonical
    }

    pub fn fields(&self) -> &[FieldDescriptor] {
        &self.fields
    }

    /// Finds a field by its persistent ID.
    pub fn field(&self, id: FieldId) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|field| field.id == id)
    }

    /// Finds a field by its declared name.
    pub fn field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|field| field.name == name)
    }

    /// Validates the descriptor with no additional caller-specific budget.
    ///
    /// This checks descriptor structure only. It does not allocate decoded
    /// values, validate a document payload, or prove semantic compatibility.
    pub fn validate(&self) -> Result<(), SchemaError> {
        self.validate_with_limits(&SchemaValidationLimits::unbounded())
    }

    /// Validates this descriptor against an explicit operational budget.
    pub fn validate_with_limits(
        &self,
        limits: &SchemaValidationLimits,
    ) -> Result<(), SchemaError> {
        validate_identifier(self.id.as_str(), IdentifierKind::Schema)?;

        if self.fields.len() > limits.max_fields {
            return Err(SchemaError::FieldBudgetExceeded {
                actual: self.fields.len(),
                limit: limits.max_fields,
            });
        }

        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();
        let mut previous = None;

        for field in &self.fields {
            validate_identifier(&field.name, IdentifierKind::Field)?;

            if !ids.insert(field.id) {
                return Err(SchemaError::DuplicateFieldId(field.id));
            }

            if !names.insert(field.name.as_str()) {
                return Err(SchemaError::DuplicateFieldName(field.name.clone()));
            }

            if self.canonical {
                if let Some(previous_id) = previous {
                    if field.id <= previous_id {
                        return Err(SchemaError::NonCanonicalFieldOrder {
                            previous: previous_id,
                            current: field.id,
                        });
                    }
                }

                previous = Some(field.id);
            }

            if field.cardinality == FieldCardinality::Repeated
                && field.wire_kind == WireKind::Unit
            {
                return Err(SchemaError::InvalidFieldContract {
                    field: field.id,
                    reason: "repeated fields cannot use the unit wire kind",
                });
            }

            if self.unknown_fields == UnknownFieldPolicy::Skip && !field.skippable {
                // This is not an error: skippable describes how a reader may
                // handle this field when it is unknown, not how its owner
                // handles the field it knows.
            }
        }

        Ok(())
    }

    /// Checks version compatibility with a writer schema.
    ///
    /// This does not compare field-level contracts; callers must validate
    /// fields and any required semantic compatibility separately.
    pub fn check_version(
        &self,
        writer: SchemaVersion,
    ) -> Result<(), SchemaError> {
        let compatible = match self.compatibility {
            CompatibilityPolicy::Exact => self.version.is_exact(writer),
            CompatibilityPolicy::BackwardCompatible => self.version.supports(writer),
            CompatibilityPolicy::Negotiated => {
                return Err(SchemaError::NegotiationRequired {
                    reader: self.version,
                    writer,
                });
            }
            CompatibilityPolicy::MigrationRequired => {
                if self.version.is_exact(writer) {
                    true
                } else {
                    return Err(SchemaError::MigrationRequired {
                        reader: self.version,
                        writer,
                    });
                }
            }
        };

        if compatible {
            Ok(())
        } else {
            Err(SchemaError::IncompatibleVersion {
                reader: self.version,
                writer,
            })
        }
    }
}

/// Operational budget for validating schema descriptors.
///
/// `usize::MAX` means this layer imposes no additional field-count budget.
/// It does not mean a process can allocate infinite memory. Untrusted-input
/// decoding should use a separate finite budget before allocating payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaValidationLimits {
    pub max_fields: usize,
}

impl SchemaValidationLimits {
    pub const fn new(max_fields: usize) -> Self {
        Self { max_fields }
    }

    pub const fn unbounded() -> Self {
        Self {
            max_fields: usize::MAX,
        }
    }
}

impl Default for SchemaValidationLimits {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Structured errors produced by schema construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    EmptyIdentifier,
    InvalidIdentifier(String),
    ReservedFieldId(u32),
    DuplicateFieldId(FieldId),
    DuplicateFieldName(String),
    NonCanonicalFieldOrder {
        previous: FieldId,
        current: FieldId,
    },
    InvalidFieldContract {
        field: FieldId,
        reason: &'static str,
    },
    FieldBudgetExceeded {
        actual: usize,
        limit: usize,
    },
    IncompatibleVersion {
        reader: SchemaVersion,
        writer: SchemaVersion,
    },
    NegotiationRequired {
        reader: SchemaVersion,
        writer: SchemaVersion,
    },
    MigrationRequired {
        reader: SchemaVersion,
        writer: SchemaVersion,
    },
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => f.write_str("schema identifier cannot be empty"),
            Self::InvalidIdentifier(value) => {
                write!(f, "invalid schema or field identifier: {value:?}")
            }
            Self::ReservedFieldId(id) => {
                write!(f, "field identifier {id} is reserved")
            }
            Self::DuplicateFieldId(id) => {
                write!(f, "duplicate field identifier {id}")
            }
            Self::DuplicateFieldName(name) => {
                write!(f, "duplicate field name {name:?}")
            }
            Self::NonCanonicalFieldOrder { previous, current } => {
                write!(
                    f,
                    "noncanonical field order: {current} follows {previous}"
                )
            }
            Self::InvalidFieldContract { field, reason } => {
                write!(f, "invalid contract for field {field}: {reason}")
            }
            Self::FieldBudgetExceeded { actual, limit } => {
                write!(f, "schema has {actual} fields; validation budget is {limit}")
            }
            Self::IncompatibleVersion { reader, writer } => {
                write!(f, "reader schema {reader} cannot consume writer schema {writer}")
            }
            Self::NegotiationRequired { reader, writer } => {
                write!(
                    f,
                    "schema compatibility negotiation required: reader {reader}, writer {writer}"
                )
            }
            Self::MigrationRequired { reader, writer } => {
                write!(
                    f,
                    "schema migration required: reader {reader}, writer {writer}"
                )
            }
        }
    }
}

impl Error for SchemaError {}

#[derive(Debug, Clone, Copy)]
enum IdentifierKind {
    Schema,
    Field,
}

fn validate_identifier(value: &str, kind: IdentifierKind) -> Result<(), SchemaError> {
    if value.is_empty() {
        return Err(SchemaError::EmptyIdentifier);
    }

    // Reject NUL and control characters. Do not impose an ASCII-only rule:
    // schema owners may define their own portable naming conventions.
    if value.chars().any(char::is_control) {
        return Err(SchemaError::InvalidIdentifier(value.to_owned()));
    }

    // Avoid leading/trailing whitespace changing identifiers invisibly.
    if value.trim() != value {
        return Err(SchemaError::InvalidIdentifier(value.to_owned()));
    }

    match kind {
        IdentifierKind::Schema => {
            // Schema identifiers are qualified textual names.
            if !value.contains('.') {
                return Err(SchemaError::InvalidIdentifier(value.to_owned()));
            }
        }
        IdentifierKind::Field => {
            // Field names must be nonempty. More restrictive naming rules
            // belong to the owning schema/dialect contract.
        }
    }

    Ok(())
}

/// Descriptive manifest for schema discovery and compatibility tooling.
///
/// The manifest is metadata, not the canonical serialized document itself.
/// A stable textual or binary rendering must be defined by the owning
/// canonical serialization module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaManifest {
    pub namespace: String,
    pub document_schema: SchemaId,
    pub payload_schema: SchemaId,
    pub version: SchemaVersion,
}

impl SchemaManifest {
    pub fn current() -> Result<Self, SchemaError> {
        Ok(Self {
            namespace: SCHEMA_NAMESPACE.to_owned(),
            document_schema: SchemaId::new(DOCUMENT_SCHEMA_ID)?,
            payload_schema: SchemaId::new(PAYLOAD_SCHEMA_ID)?,
            version: CURRENT_SCHEMA_VERSION,
        })
    }
}

/// Returns whether the reader can consider the writer version under the
/// conservative same-major, no-future-version rule.
pub const fn is_schema_version_compatible(
    reader: SchemaVersion,
    writer: SchemaVersion,
) -> bool {
    reader.supports(writer)
}

/// Validates a textual schema identifier.
pub fn validate_schema_id(value: &str) -> Result<(), SchemaError> {
    validate_identifier(value, IdentifierKind::Schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema() -> SchemaDescriptor {
        let fields = vec![
            FieldDescriptor::new(
                FieldId::new(1).expect("nonzero ID"),
                "format_version",
                WireKind::U32,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
            FieldDescriptor::new(
                FieldId::new(2).expect("nonzero ID"),
                "payload",
                WireKind::Bytes,
                FieldCardinality::Required,
                true,
                true,
            )
            .expect("valid field"),
        ];

        SchemaDescriptor::new(
            SchemaId::new(DOCUMENT_SCHEMA_ID).expect("valid schema ID"),
            SchemaKind::Document,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::BackwardCompatible,
            UnknownFieldPolicy::Reject,
            true,
            fields,
        )
    }

    #[test]
    fn current_version_is_stable() {
        assert_eq!(CURRENT_SCHEMA_VERSION, SchemaVersion::new(1, 0, 0));
    }

    #[test]
    fn older_patch_is_accepted_by_newer_reader() {
        assert!(SchemaVersion::new(1, 2, 3).supports(SchemaVersion::new(1, 2, 2)));
    }

    #[test]
    fn future_minor_is_not_assumed_compatible() {
        assert!(!SchemaVersion::new(1, 0, 0).supports(SchemaVersion::new(1, 1, 0)));
    }

    #[test]
    fn future_major_is_not_compatible() {
        assert!(!SchemaVersion::new(2, 0, 0).supports(SchemaVersion::new(1, 9, 9)));
    }

    #[test]
    fn zero_field_id_is_rejected() {
        assert!(FieldId::new(0).is_none());
        assert!(FieldId::try_from(0).is_err());
    }

    #[test]
    fn descriptor_validates() {
        assert!(sample_schema().validate().is_ok());
    }

    #[test]
    fn duplicate_field_ids_are_rejected() {
        let id = FieldId::new(1).expect("nonzero ID");
        let fields = vec![
            FieldDescriptor::new(
                id,
                "first",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
            FieldDescriptor::new(
                id,
                "second",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
        ];

        let schema = SchemaDescriptor::new(
            SchemaId::new("zamani.test.duplicates").expect("valid ID"),
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::Exact,
            UnknownFieldPolicy::Reject,
            true,
            fields,
        );

        assert!(matches!(
            schema.validate(),
            Err(SchemaError::DuplicateFieldId(_))
        ));
    }

    #[test]
    fn noncanonical_field_order_is_rejected() {
        let fields = vec![
            FieldDescriptor::new(
                FieldId::new(2).expect("nonzero ID"),
                "second",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
            FieldDescriptor::new(
                FieldId::new(1).expect("nonzero ID"),
                "first",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
        ];

        let schema = SchemaDescriptor::new(
            SchemaId::new("zamani.test.order").expect("valid ID"),
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::Exact,
            UnknownFieldPolicy::Reject,
            true,
            fields,
        );

        assert!(matches!(
            schema.validate(),
            Err(SchemaError::NonCanonicalFieldOrder { .. })
        ));
    }

    #[test]
    fn duplicate_field_names_are_rejected() {
        let fields = vec![
            FieldDescriptor::new(
                FieldId::new(1).expect("nonzero ID"),
                "value",
                WireKind::U64,
                FieldCardinality::Required,
                false,
                true,
            )
            .expect("valid field"),
            FieldDescriptor::new(
                FieldId::new(2).expect("nonzero ID"),
                "value",
                WireKind::U64,
                FieldCardinality::Optional,
                false,
                true,
            )
            .expect("valid field"),
        ];

        let schema = SchemaDescriptor::new(
            SchemaId::new("zamani.test.names").expect("valid ID"),
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::Exact,
            UnknownFieldPolicy::Reject,
            true,
            fields,
        );

        assert!(matches!(
            schema.validate(),
            Err(SchemaError::DuplicateFieldName(_))
        ));
    }

    #[test]
    fn field_budget_is_enforced() {
        let schema = sample_schema();

        assert!(matches!(
            schema.validate_with_limits(&SchemaValidationLimits::new(1)),
            Err(SchemaError::FieldBudgetExceeded {
                actual: 2,
                limit: 1
            })
        ));
    }

    #[test]
    fn field_lookup_is_stable() {
        let schema = sample_schema();
        let id = FieldId::new(2).expect("nonzero ID");

        assert_eq!(schema.field(id).map(|field| field.name.as_str()), Some("payload"));
        assert_eq!(
            schema.field_by_name("format_version").map(|field| field.id.value()),
            Some(1)
        );
    }

    #[test]
    fn schema_ids_reject_empty_and_unqualified_values() {
        assert!(SchemaId::new("").is_err());
        assert!(SchemaId::new("unqualified").is_err());
        assert!(SchemaId::new("zamani.test.schema").is_ok());
    }

    #[test]
    fn schema_ids_reject_control_characters() {
        assert!(SchemaId::new("zamani.test.bad\nschema").is_err());
    }

    #[test]
    fn repeated_unit_field_is_rejected() {
        let fields = vec![
            FieldDescriptor::new(
                FieldId::new(1).expect("nonzero ID"),
                "empty_values",
                WireKind::Unit,
                FieldCardinality::Repeated,
                true,
                false,
            )
            .expect("field descriptor is structurally constructible"),
        ];

        let schema = SchemaDescriptor::new(
            SchemaId::new("zamani.test.unit").expect("valid ID"),
            SchemaKind::Object,
            CURRENT_SCHEMA_VERSION,
            CompatibilityPolicy::Exact,
            UnknownFieldPolicy::Reject,
            true,
            fields,
        );

        assert!(matches!(
            schema.validate(),
            Err(SchemaError::InvalidFieldContract { .. })
        ));
    }

    #[test]
    fn negotiated_compatibility_does_not_silently_succeed() {
        let mut schema = sample_schema();
        schema.compatibility = CompatibilityPolicy::Negotiated;

        assert!(matches!(
            schema.check_version(CURRENT_SCHEMA_VERSION),
            Err(SchemaError::NegotiationRequired { .. })
        ));
    }

    #[test]
    fn migration_policy_is_explicit() {
        let mut schema = sample_schema();
        schema.compatibility = CompatibilityPolicy::MigrationRequired;

        assert!(schema.check_version(CURRENT_SCHEMA_VERSION).is_ok());
        assert!(matches!(
            schema.check_version(SchemaVersion::new(0, 9, 0)),
            Err(SchemaError::MigrationRequired { .. })
        ));
    }

    #[test]
    fn wire_widths_are_consistent() {
        assert_eq!(WireKind::Bool.fixed_width(), Some(1));
        assert_eq!(WireKind::U16.fixed_width(), Some(2));
        assert_eq!(WireKind::U32.fixed_width(), Some(4));
        assert_eq!(WireKind::U64.fixed_width(), Some(8));
        assert_eq!(WireKind::Bytes.fixed_width(), None);
    }

    #[test]
    fn manifest_uses_canonical_identifiers() {
        let manifest = SchemaManifest::current().expect("valid built-in IDs");

        assert_eq!(manifest.namespace, SCHEMA_NAMESPACE);
        assert_eq!(manifest.document_schema.as_str(), DOCUMENT_SCHEMA_ID);
        assert_eq!(manifest.payload_schema.as_str(), PAYLOAD_SCHEMA_ID);
        assert_eq!(manifest.version, CURRENT_SCHEMA_VERSION);
    }
}