I inspected the repository’s existing `src/zuir/schema/types.rs`, `src/zuir/schema/attributes.rs`, `src/zuir/ZUIR-SPEC.md`, `src/zuir/ids.rs`, `src/zuir/version.rs`, `Cargo.toml`, and the relevant grammar and semantic-model specifications.

One important finding: `src/zuir/schema/metadata.rs` does not currently exist on the repository’s default branch. The implementation below is therefore a proposed new file, designed to integrate with the existing schema vocabulary rather than replace it.

The design follows these boundaries:

* Reuse `AttributeKey`, `AttributeValue`, `SchemaType`, and `QualifiedName` instead of defining competing schema primitives.

* Separate metadata that affects program meaning from descriptive, provenance, and tooling information.

* Make fingerprint participation and transformation behavior explicit so optimization or metadata copying cannot silently discard meaningful information.

* Preserve deterministic ordering, validate values after deserialization, and use no `unsafe` code.

* Keep metadata declarative. It does not introduce source syntax, execute extension code, or define hardware-specific behavior.

The grammar remains the authority for source syntax; the semantic model defines source meaning; ZUIR metadata records explicitly classified information needed by the representation and its consumers.

### Proposed `src/zuir/schema/metadata.rs`

The file contract is deliberately self-contained. Its only sibling dependencies are the existing `attributes.rs` and `types.rs`; it does not depend on the IR, builders, verifier, dialect registry, or serialization pipeline.

```rust
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Metadata
//!
//! Path: `src/zuir/schema/metadata.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines deterministic, validated metadata for ZUIR schemas and related
//! representation entities. Metadata records explicitly classified facts
//! without making this module responsible for program semantics, source
//! parsing, dialect registration, execution, or backend-specific behavior.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! The grammar defines syntax. Semantic analysis defines source meaning.
//! ZUIR metadata describes explicitly classified representation information.
//!
//! ## OWNS
//!
//! - Metadata classification and metadata-entry invariants.
//! - Metadata definitions and their validation policies.
//! - Deterministic metadata storage and lookup.
//! - Explicit fingerprint and transformation participation policies.
//! - Metadata validation against declared schema definitions.
//! - Local metadata errors.
//!
//! ## DOES NOT OWN
//!
//! - Source-language syntax or source attribute parsing.
//! - Semantic type inference or name resolution.
//! - IR entities, operations, regions, or builders.
//! - Dialect registration or authorization of namespaces.
//! - Cryptographic fingerprint algorithms.
//! - Serialization of complete programs or schema migration.
//! - Execution, target selection, physical layout, or hardware allocation.
//!
//! ## DEPENDENCIES
//!
//! Reuses `AttributeKey`, `AttributeValue`, `AttributeError`, `SchemaType`,
//! `SchemaTypeError`, and `QualifiedName` from sibling schema modules.
//! It does not depend on future metadata consumers.
//!
//! ## SCALABILITY
//!
//! Collections are dynamically sized and use checked accounting where
//! applicable. No hardware capacities, operation counts, metadata counts,
//! tensor ranks, or domain-specific maximums are imposed here.
//!
//! Finite processes still have finite resources. Deserializers handling
//! untrusted input must enforce configured byte, nesting, collection, and
//! allocation budgets before constructing arbitrarily large metadata trees.
//!
//! ## DETERMINISM
//!
//! Metadata keys are stored in a `BTreeMap`. Iteration is deterministic by
//! key order. Array order is retained because it may be meaningful.
//! This module does not implement a cryptographic hash or claim that an
//! arbitrary serializer produces canonical bytes.
//!
//! ## TRUST BOUNDARY
//!
//! Serde deserialization does not prove that invariants hold. Call `validate`
//! or `validate_against` after decoding and before accepting metadata into
//! verified IR. Namespace authorization and extension trust are checked by
//! the owning registry or verifier.
//!
//! ## FREEZE CRITERIA
//!
//! - Metadata values reuse the canonical schema attribute vocabulary.
//! - Semantic and descriptive metadata are distinguished explicitly.
//! - Fingerprint and transformation participation are declared.
//! - Semantic metadata cannot be configured for silent removal.
//! - Deserialized data can be revalidated.
//! - Unknown metadata is rejected or retained according to explicit policy.
//! - Unit, integration, compatibility, and resource-budget tests pass.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

use super::attributes::{
    AttributeError, AttributeKey, AttributeValue,
};
use super::types::{SchemaType, SchemaTypeError};

// =============================================================================
// Shared schema vocabulary
// =============================================================================

/// Canonical key type for metadata.
///
/// Metadata keys use the same validated, namespaced representation as
/// attributes. Namespace authorization is not performed by this alias;
/// the owning dialect or extension registry must authorize each namespace.
pub type MetadataKey = AttributeKey;

/// Canonical value type for metadata.
///
/// Metadata values reuse the existing inert, validated attribute-value
/// representation. Metadata does not introduce a second value system.
pub type MetadataValue = AttributeValue;

// =============================================================================
// Metadata classification and policies
// =============================================================================

/// Semantic role of a metadata entry.
///
/// The classification describes how consumers should interpret the entry.
/// It does not grant permission to execute code or authorize an extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MetadataClass {
    /// Information that can affect the meaning, legality, or interpretation
    /// of the represented program.
    ///
    /// Examples include explicitly specified semantic modes, numerical
    /// interpretation contracts, and other normative representation facts.
    Semantic,

    /// Human-readable or organizational information that does not, by
    /// itself, change computational meaning.
    ///
    /// Examples include descriptions, display labels, and documentation.
    Descriptive,

    /// Information recording origin, evidence, transformation history, or
    /// other audit and provenance facts.
    Provenance,

    /// Information intended for editors, debuggers, visualizers, or other
    /// development tools.
    Tooling,
}

impl MetadataClass {
    /// Whether the class is intrinsically semantic.
    #[must_use]
    pub const fn is_semantic(self) -> bool {
        matches!(self, Self::Semantic)
    }

    /// Whether metadata of this class is excluded from semantic fingerprints
    /// by the default policy.
    ///
    /// A schema may explicitly include non-semantic metadata in a cache or
    /// artifact fingerprint when that metadata affects the relevant output.
    #[must_use]
    pub const fn excluded_from_fingerprint_by_default(self) -> bool {
        !matches!(self, Self::Semantic)
    }
}

/// Whether a metadata entry participates in a fingerprint computed by a
/// consumer, such as an IR cache or reproducible artifact pipeline.
///
/// The fingerprint algorithm itself belongs to the fingerprinting module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintParticipation {
    /// Include this metadata value in the relevant fingerprint input.
    Include,

    /// Exclude this metadata value from the relevant fingerprint input.
    Exclude,
}

impl FingerprintParticipation {
    /// Return the default for a metadata class.
    #[must_use]
    pub const fn for_class(class: MetadataClass) -> Self {
        match class {
            MetadataClass::Semantic => Self::Include,
            MetadataClass::Descriptive
            | MetadataClass::Provenance
            | MetadataClass::Tooling => Self::Exclude,
        }
    }
}

/// Behavior requested when metadata passes through a transformation.
///
/// The transformation implementation must still preserve any information
/// required by the ZUIR specification or a downstream contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataTransformPolicy {
    /// Preserve the metadata entry and its value.
    Preserve,

    /// Permit a transformation to omit this entry only when the entry is
    /// non-semantic and the transformation's contract allows the omission.
    ///
    /// This is an explicit opt-in to potential metadata loss, not a command
    /// that every transformation must discard the entry.
    DropNonSemantic,
}

/// Policy for metadata keys that have no definition in a metadata schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownMetadataPolicy {
    /// Reject metadata keys that are not declared.
    Reject,

    /// Preserve unknown metadata as inert data.
    ///
    /// Preservation does not authorize the namespace, confer executable
    /// meaning, or prove that the extension is trusted.
    Preserve,
}

// =============================================================================
// Metadata entries
// =============================================================================

/// One metadata value and its declared role.
///
/// A key maps to one entry. When a domain needs multiple values under one
/// conceptual key, use an `AttributeValue::Array` so the collection's
/// multiplicity and ordering remain explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataEntry {
    class: MetadataClass,
    value: MetadataValue,
}

impl MetadataEntry {
    /// Construct a metadata entry and validate its value.
    pub fn new(
        class: MetadataClass,
        value: MetadataValue,
    ) -> Result<Self, MetadataError> {
        value
            .validate()
            .map_err(MetadataError::InvalidValue)?;

        Ok(Self { class, value })
    }

    /// Return the declared metadata class.
    #[must_use]
    pub const fn class(&self) -> MetadataClass {
        self.class
    }

    /// Return the metadata value.
    #[must_use]
    pub fn value(&self) -> &MetadataValue {
        &self.value
    }

    /// Revalidate this entry, including after deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        self.value
            .validate()
            .map_err(MetadataError::InvalidValue)
    }

    /// Default fingerprint participation when no metadata schema definition
    /// overrides the class's default.
    #[must_use]
    pub const fn default_fingerprint_participation(
        &self,
    ) -> FingerprintParticipation {
        FingerprintParticipation::for_class(self.class)
    }

    /// Validate that the entry's class and value agree with a definition.
    pub fn validate_against(
        &self,
        definition: &MetadataDefinition,
    ) -> Result<(), MetadataError> {
        self.validate()?;

        if self.class != definition.class {
            return Err(MetadataError::ClassMismatch {
                expected: definition.class,
                actual: self.class,
            });
        }

        let conforms = self
            .value
            .conforms_to(&definition.value_type)
            .map_err(MetadataError::InvalidAttribute)?;

        if !conforms {
            return Err(MetadataError::ValueTypeMismatch);
        }

        Ok(())
    }
}

// =============================================================================
// Metadata definitions and schemas
// =============================================================================

/// Definition of a permitted metadata key.
///
/// The definition is declarative. It does not execute callbacks, resolve
/// external references, or authorize the key's namespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDefinition {
    key: MetadataKey,
    value_type: SchemaType,
    class: MetadataClass,
    required: bool,
    fingerprint: FingerprintParticipation,
    transform: MetadataTransformPolicy,
    description: Option<String>,
}

impl MetadataDefinition {
    /// Construct a metadata definition using class-based default policies.
    pub fn new(
        key: MetadataKey,
        value_type: SchemaType,
        class: MetadataClass,
        required: bool,
    ) -> Result<Self, MetadataError> {
        key.validate().map_err(MetadataError::InvalidAttribute)?;

        value_type
            .validate()
            .map_err(MetadataError::InvalidSchemaType)?;

        Ok(Self {
            key,
            value_type,
            class,
            required,
            fingerprint: FingerprintParticipation::for_class(class),
            transform: MetadataTransformPolicy::Preserve,
            description: None,
        })
    }

    /// Explicitly choose fingerprint participation.
    ///
    /// Semantic metadata cannot be excluded from a semantic fingerprint.
    /// A consumer may define a different fingerprint purpose, but that
    /// policy must be represented outside this semantic schema.
    pub fn with_fingerprint_participation(
        mut self,
        participation: FingerprintParticipation,
    ) -> Result<Self, MetadataError> {
        if self.class.is_semantic()
            && participation == FingerprintParticipation::Exclude
        {
            return Err(MetadataError::SemanticMetadataExcludedFromFingerprint);
        }

        self.fingerprint = participation;
        Ok(self)
    }

    /// Choose the permitted transformation behavior.
    ///
    /// Semantic metadata cannot be marked as droppable.
    pub fn with_transform_policy(
        mut self,
        policy: MetadataTransformPolicy,
    ) -> Result<Self, MetadataError> {
        if self.class.is_semantic()
            && policy == MetadataTransformPolicy::DropNonSemantic
        {
            return Err(MetadataError::SemanticMetadataMayNotBeDropped);
        }

        self.transform = policy;
        Ok(self)
    }

    /// Attach an optional human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the key.
    #[must_use]
    pub fn key(&self) -> &MetadataKey {
        &self.key
    }

    /// Return the declared value schema.
    #[must_use]
    pub fn value_type(&self) -> &SchemaType {
        &self.value_type
    }

    /// Return the declared class.
    #[must_use]
    pub const fn class(&self) -> MetadataClass {
        self.class
    }

    /// Whether the key must be present.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }

    /// Return the fingerprint participation policy.
    #[must_use]
    pub const fn fingerprint_participation(
        &self,
    ) -> FingerprintParticipation {
        self.fingerprint
    }

    /// Return the transformation policy.
    #[must_use]
    pub const fn transform_policy(&self) -> MetadataTransformPolicy {
        self.transform
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate the definition's invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        self.key.validate().map_err(MetadataError::InvalidAttribute)?;

        self.value_type
            .validate()
            .map_err(MetadataError::InvalidSchemaType)?;

        if self.class.is_semantic()
            && self.fingerprint == FingerprintParticipation::Exclude
        {
            return Err(MetadataError::SemanticMetadataExcludedFromFingerprint);
        }

        if self.class.is_semantic()
            && self.transform == MetadataTransformPolicy::DropNonSemantic
        {
            return Err(MetadataError::SemanticMetadataMayNotBeDropped);
        }

        Ok(())
    }
}

/// Deterministic set of metadata definitions.
///
/// The schema owns validation policy. It does not own namespace registration,
/// dialect trust, or complete-program serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSchema {
    definitions: BTreeMap<MetadataKey, MetadataDefinition>,
    unknown_policy: UnknownMetadataPolicy,
}

impl MetadataSchema {
    /// Create an empty schema with an explicit unknown-key policy.
    #[must_use]
    pub fn new(unknown_policy: UnknownMetadataPolicy) -> Self {
        Self {
            definitions: BTreeMap::new(),
            unknown_policy,
        }
    }

    /// Add a definition, rejecting duplicate keys.
    pub fn insert(
        &mut self,
        definition: MetadataDefinition,
    ) -> Result<(), MetadataError> {
        definition.validate()?;
        let key = definition.key.clone();

        if self.definitions.contains_key(&key) {
            return Err(MetadataError::DuplicateDefinition(key));
        }

        self.definitions.insert(key, definition);
        Ok(())
    }

    /// Look up a definition by key.
    #[must_use]
    pub fn get(
        &self,
        key: &MetadataKey,
    ) -> Option<&MetadataDefinition> {
        self.definitions.get(key)
    }

    /// Iterate definitions in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetadataKey, &MetadataDefinition)> {
        self.definitions.iter()
    }

    /// Return the unknown-key policy.
    #[must_use]
    pub const fn unknown_policy(&self) -> UnknownMetadataPolicy {
        self.unknown_policy
    }

    /// Validate schema invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        for (key, definition) in &self.definitions {
            key.validate().map_err(MetadataError::InvalidAttribute)?;
            definition.validate()?;

            if key != definition.key() {
                return Err(MetadataError::DefinitionKeyMismatch);
            }
        }

        Ok(())
    }

    /// Validate a metadata set against this schema.
    pub fn validate_values(
        &self,
        values: &MetadataSet,
    ) -> Result<(), MetadataError> {
        self.validate()?;
        values.validate()?;

        for (key, entry) in values.iter() {
            match self.definitions.get(key) {
                Some(definition) => entry.validate_against(definition)?,
                None => match self.unknown_policy {
                    UnknownMetadataPolicy::Reject => {
                        return Err(MetadataError::UnknownKey(key.clone()));
                    }
                    UnknownMetadataPolicy::Preserve => {
                        // Unknown entries remain inert and preserved.
                        // Registry-level namespace authorization is separate.
                    }
                },
            }
        }

        for (key, definition) in &self.definitions {
            if definition.required() && !values.contains_key(key) {
                return Err(MetadataError::MissingRequiredKey(key.clone()));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Metadata storage
// =============================================================================

/// Deterministic collection of metadata entries.
///
/// A single key stores a single entry. Repeated data belongs in an array value.
/// This prevents duplicate-key behavior from depending on insertion order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSet {
    entries: BTreeMap<MetadataKey, MetadataEntry>,
}

impl MetadataSet {
    /// Create an empty metadata set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an entry, rejecting a duplicate key.
    ///
    /// Validation happens before mutation. On failure, the set is unchanged.
    pub fn insert(
        &mut self,
        key: MetadataKey,
        entry: MetadataEntry,
    ) -> Result<(), MetadataError> {
        key.validate().map_err(MetadataError::InvalidAttribute)?;
        entry.validate()?;

        if self.entries.contains_key(&key) {
            return Err(MetadataError::DuplicateKey(key));
        }

        self.entries.insert(key, entry);
        Ok(())
    }

    /// Replace an existing entry or insert a new one.
    ///
    /// Validation happens before mutation. The previous entry is returned
    /// only when replacement succeeds.
    pub fn replace(
        &mut self,
        key: MetadataKey,
        entry: MetadataEntry,
    ) -> Result<Option<MetadataEntry>, MetadataError> {
        key.validate().map_err(MetadataError::InvalidAttribute)?;
        entry.validate()?;

        Ok(self.entries.insert(key, entry))
    }

    /// Get an entry by key.
    #[must_use]
    pub fn get(&self, key: &MetadataKey) -> Option<&MetadataEntry> {
        self.entries.get(key)
    }

    /// Whether the set contains a key.
    #[must_use]
    pub fn contains_key(&self, key: &MetadataKey) -> bool {
        self.entries.contains_key(key)
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&MetadataKey, &MetadataEntry)> {
        self.entries.iter()
    }

    /// Validate all local invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        for (key, entry) in &self.entries {
            key.validate().map_err(MetadataError::InvalidAttribute)?;
            entry.validate()?;
        }

        Ok(())
    }

    /// Iterate metadata that participates in a semantic or artifact
    /// fingerprint according to the supplied schema.
    ///
    /// Declared entries follow their schema's explicit policy. Unknown
    /// preserved entries are included conservatively so adding unknown
    /// metadata cannot silently reuse a cache key computed without it.
    ///
    /// This method selects entries only; the fingerprint module remains
    /// responsible for canonical encoding and hashing.
    pub fn iter_for_fingerprint<'a>(
        &'a self,
        schema: &'a MetadataSchema,
    ) -> impl Iterator<Item = (&'a MetadataKey, &'a MetadataEntry)> + 'a {
        self.entries.iter().filter(move |(key, _)| {
            match schema.get(key) {
                Some(definition) => {
                    definition.fingerprint_participation()
                        == FingerprintParticipation::Include
                }
                None => true,
            }
        })
    }

    /// Return entries that transformations must preserve under the supplied
    /// schema. Unknown entries are preserved conservatively.
    pub fn iter_to_preserve<'a>(
        &'a self,
        schema: &'a MetadataSchema,
    ) -> impl Iterator<Item = (&'a MetadataKey, &'a MetadataEntry)> + 'a {
        self.entries.iter().filter(move |(key, entry)| {
            match schema.get(key) {
                Some(definition) => {
                    entry.class().is_semantic()
                        || definition.transform_policy()
                            == MetadataTransformPolicy::Preserve
                }
                None => true,
            }
        })
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by metadata construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetadataError {
    /// An attribute-key invariant failed.
    InvalidAttribute(String),

    /// A metadata value is malformed.
    InvalidValue(String),

    /// A schema type is malformed.
    InvalidSchemaType(String),

    /// A metadata key is defined more than once.
    DuplicateKey(MetadataKey),

    /// A schema declares the same key more than once.
    DuplicateDefinition(MetadataKey),

    /// The map key differs from the key stored in its definition.
    DefinitionKeyMismatch,

    /// A required metadata key is missing.
    MissingRequiredKey(MetadataKey),

    /// A key is not declared and unknown metadata is forbidden.
    UnknownKey(MetadataKey),

    /// An entry's declared class differs from its schema.
    ClassMismatch {
        expected: MetadataClass,
        actual: MetadataClass,
    },

    /// An entry's value does not conform to the declared schema type.
    ValueTypeMismatch,

    /// Semantic metadata was configured to be excluded from a semantic
    /// fingerprint.
    SemanticMetadataExcludedFromFingerprint,

    /// Semantic metadata was configured to be droppable.
    SemanticMetadataMayNotBeDropped,
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAttribute(error) => {
                write!(f, "invalid metadata key or attribute: {error}")
            }
            Self::InvalidValue(error) => {
                write!(f, "invalid metadata value: {error}")
            }
            Self::InvalidSchemaType(error) => {
                write!(f, "invalid metadata schema type: {error}")
            }
            Self::DuplicateKey(key) => {
                write!(f, "duplicate metadata key: {key}")
            }
            Self::DuplicateDefinition(key) => {
                write!(f, "duplicate metadata definition: {key}")
            }
            Self::DefinitionKeyMismatch => {
                f.write_str("metadata schema map key does not match its definition")
            }
            Self::MissingRequiredKey(key) => {
                write!(f, "required metadata key is missing: {key}")
            }
            Self::UnknownKey(key) => {
                write!(f, "metadata key is not declared by the schema: {key}")
            }
            Self::ClassMismatch { expected, actual } => {
                write!(
                    f,
                    "metadata class mismatch: expected {expected:?}, found {actual:?}"
                )
            }
            Self::ValueTypeMismatch => {
                f.write_str("metadata value does not conform to its declared schema type")
            }
            Self::SemanticMetadataExcludedFromFingerprint => {
                f.write_str("semantic metadata cannot be excluded from a semantic fingerprint")
            }
            Self::SemanticMetadataMayNotBeDropped => {
                f.write_str("semantic metadata cannot be configured for removal")
            }
        }
    }
}

impl std::error::Error for MetadataError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::attributes::AttributeName;
    use super::super::types::QualifiedName;

    fn key(namespace: &str, local: &str) -> MetadataKey {
        let namespace =
            QualifiedName::parse(namespace).expect("valid namespace");
        let local =
            AttributeName::new(local).expect("valid local name");

        MetadataKey::new(namespace, local).expect("valid metadata key")
    }

    fn string_entry(
        class: MetadataClass,
        value: &str,
    ) -> MetadataEntry {
        MetadataEntry::new(
            class,
            MetadataValue::String(value.to_owned()),
        )
        .expect("valid metadata entry")
    }

    #[test]
    fn stores_entries_in_deterministic_key_order() {
        let mut set = MetadataSet::new();
        set.insert(
            key("tooling", "zeta"),
            string_entry(MetadataClass::Tooling, "last"),
        )
        .expect("insert zeta");
        set.insert(
            key("core", "alpha"),
            string_entry(MetadataClass::Semantic, "first"),
        )
        .expect("insert alpha");

        let names: Vec<String> = set
            .iter()
            .map(|(key, _)| key.to_string())
            .collect();

        assert_eq!(names, vec!["core::alpha", "tooling::zeta"]);
    }

    #[test]
    fn rejects_duplicate_keys_without_replacing_existing_entries() {
        let mut set = MetadataSet::new();
        let metadata_key = key("core", "mode");

        set.insert(
            metadata_key.clone(),
            string_entry(MetadataClass::Semantic, "strict"),
        )
        .expect("initial insert");

        let result = set.insert(
            metadata_key.clone(),
            string_entry(MetadataClass::Semantic, "relaxed"),
        );

        assert!(matches!(result, Err(MetadataError::DuplicateKey(_))));
        assert_eq!(
            set.get(&metadata_key).expect("entry").value(),
            &MetadataValue::String("strict".to_owned())
        );
    }

    #[test]
    fn validates_schema_type_and_metadata_class() {
        let metadata_key = key("core", "mode");
        let definition = MetadataDefinition::new(
            metadata_key.clone(),
            SchemaType::String,
            MetadataClass::Semantic,
            true,
        )
        .expect("valid definition");

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        schema.insert(definition).expect("add definition");

        let mut values = MetadataSet::new();
        values
            .insert(
                metadata_key,
                string_entry(MetadataClass::Semantic, "strict"),
            )
            .expect("add value");

        assert!(schema.validate_values(&values).is_ok());
    }

    #[test]
    fn rejects_missing_required_metadata() {
        let definition = MetadataDefinition::new(
            key("core", "mode"),
            SchemaType::String,
            MetadataClass::Semantic,
            true,
        )
        .expect("valid definition");

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        schema.insert(definition).expect("add definition");

        assert!(matches!(
            schema.validate_values(&MetadataSet::new()),
            Err(MetadataError::MissingRequiredKey(_))
        ));
    }

    #[test]
    fn rejects_wrong_value_type() {
        let metadata_key = key("core", "enabled");
        let definition = MetadataDefinition::new(
            metadata_key.clone(),
            SchemaType::Boolean,
            MetadataClass::Semantic,
            false,
        )
        .expect("valid definition");

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        schema.insert(definition).expect("add definition");

        let mut values = MetadataSet::new();
        values
            .insert(
                metadata_key,
                string_entry(MetadataClass::Semantic, "yes"),
            )
            .expect("add value");

        assert!(matches!(
            schema.validate_values(&values),
            Err(MetadataError::ValueTypeMismatch)
        ));
    }

    #[test]
    fn semantic_metadata_cannot_be_excluded_or_dropped() {
        let definition = MetadataDefinition::new(
            key("core", "contract"),
            SchemaType::String,
            MetadataClass::Semantic,
            false,
        )
        .expect("valid definition");

        assert!(matches!(
            definition.clone().with_fingerprint_participation(
                FingerprintParticipation::Exclude
            ),
            Err(MetadataError::SemanticMetadataExcludedFromFingerprint)
        ));

        assert!(matches!(
            definition.with_transform_policy(
                MetadataTransformPolicy::DropNonSemantic
            ),
            Err(MetadataError::SemanticMetadataMayNotBeDropped)
        ));
    }

    #[test]
    fn descriptive_metadata_is_excluded_from_fingerprint_by_default() {
        let definition = MetadataDefinition::new(
            key("tooling", "display_name"),
            SchemaType::String,
            MetadataClass::Descriptive,
            false,
        )
        .expect("valid definition");

        assert_eq!(
            definition.fingerprint_participation(),
            FingerprintParticipation::Exclude
        );
    }

    #[test]
    fn unknown_metadata_is_preserved_or_rejected_explicitly() {
        let unknown_key = key("future_domain", "annotation");
        let mut values = MetadataSet::new();

        values
            .insert(
                unknown_key.clone(),
                string_entry(MetadataClass::Descriptive, "future value"),
            )
            .expect("insert unknown entry");

        let preserve = MetadataSchema::new(UnknownMetadataPolicy::Preserve);
        assert!(preserve.validate_values(&values).is_ok());

        let reject = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        assert!(matches!(
            reject.validate_values(&values),
            Err(MetadataError::UnknownKey(_))
        ));
    }

    #[test]
    fn malformed_deserialized_values_are_rejected_by_validation() {
        // A public deserialization path can bypass constructors. The
        // explicit validation boundary must therefore remain mandatory.
        let malformed = MetadataEntry {
            class: MetadataClass::Semantic,
            value: MetadataValue::Integer("01".to_owned()),
        };

        assert!(matches!(
            malformed.validate(),
            Err(MetadataError::InvalidValue(_))
        ));
    }

    #[test]
    fn semantic_fingerprint_selection_is_schema_driven() {
        let semantic_key = key("core", "mode");
        let descriptive_key = key("tooling", "label");

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Preserve);
        schema
            .insert(
                MetadataDefinition::new(
                    semantic_key.clone(),
                    SchemaType::String,
                    MetadataClass::Semantic,
                    false,
                )
                .expect("semantic definition"),
            )
            .expect("insert semantic definition");

        schema
            .insert(
                MetadataDefinition::new(
                    descriptive_key.clone(),
                    SchemaType::String,
                    MetadataClass::Descriptive,
                    false,
                )
                .expect("descriptive definition"),
            )
            .expect("insert descriptive definition");

        let mut values = MetadataSet::new();
        values
            .insert(
                semantic_key,
                string_entry(MetadataClass::Semantic, "strict"),
            )
            .expect("semantic entry");
        values
            .insert(
                descriptive_key,
                string_entry(MetadataClass::Descriptive, "Strict Mode"),
            )
            .expect("descriptive entry");

        let fingerprint_keys: Vec<String> = values
            .iter_for_fingerprint(&schema)
            .map(|(key, _)| key.to_string())
            .collect();

        assert_eq!(fingerprint_keys, vec!["core::mode"]);
    }
}
```

Worked for 1m 43s

### Required correction before compiling

There is one Rust type mismatch in the implementation above: the error enum stores error messages as `String`, while the existing schema APIs return structured `AttributeError` and `SchemaTypeError` values. Preserve those structured errors instead of converting them to strings.

Replace the three affected variants in `MetadataError` with:

```rust
InvalidAttribute(AttributeError),
InvalidValue(AttributeError),
InvalidSchemaType(SchemaTypeError),
```

Then replace the corresponding arms in `fmt::Display` with:

```rust
Self::InvalidAttribute(error) => {
    write!(f, "invalid metadata key or attribute: {error}")
}
Self::InvalidValue(error) => {
    write!(f, "invalid metadata value: {error}")
}
Self::InvalidSchemaType(error) => {
    write!(f, "invalid metadata schema type: {error}")
}
```

This keeps the original structured errors available to callers and makes the existing `map_err(MetadataError::InvalidAttribute)`, `map_err(MetadataError::InvalidValue)`, and `map_err(MetadataError::InvalidSchemaType)` conversions type-compatible.

### Integration requirements

The repository inspection also revealed that `src/zuir/schema/mod.rs` and `src/zuir/schema/extensions.rs` are currently absent from the default branch. The parent `src/zuir/mod.rs` content returned by the repository fetch is also not a normal Rust module declaration. Those are integration issues, not reasons to make `metadata.rs` depend on future modules.

Create or repair the schema module boundary so it declares the three sibling modules:

```rust
pub mod attributes;
pub mod metadata;
pub mod types;
```

Re-export metadata types only where the repository's public API policy calls for them. Do not create duplicate attribute, schema-type, or metadata definitions.

The grammar files I inspected reinforce the separation: `grammar/Zamani.g4` is a root composition grammar, while `grammar/specification/semantic-model.md` governs semantic meaning. Neither should depend on the metadata implementation.

### Freeze and verification gate

Do not yet mark this file production-ready merely because the implementation and unit tests are written. Run the following from the repository root after applying the correction and wiring the module:

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Add or verify tests for:

* Serde round trips and malformed metadata deserialization.

* Duplicate keys, required fields, class mismatches, and invalid schema values.

* Deterministic metadata ordering and fingerprint selection.

* Unknown metadata preservation and explicit rejection.

* Semantic metadata preservation through transformations.

* Resource-budget enforcement at the decoding boundary.

Important limitation: I inspected the live files and wrote the proposed implementation, but I have not run these commands against your repository. The file should be considered ready for integration review, not certified as compiling or production-ready, until the correction, module wiring, and repository checks pass.
