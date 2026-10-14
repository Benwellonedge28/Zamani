
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Schema Metadata
//!
//! Path: `src/zuir/schema/metadata.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Defines typed, extensible, deterministic metadata for ZUIR schema
//! entities. Metadata may describe semantic requirements, verification
//! requirements, provenance, documentation, or tooling information.
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
//! The grammar owns source syntax. Semantic analysis owns source meaning.
//! ZUIR owns the representation of established computational meaning.
//! This module defines metadata policy, not source-language semantics.
//!
//! ## OWNS
//!
//! - Metadata classification and policy declarations.
//! - Metadata definitions and schema validation.
//! - Deterministic metadata storage and lookup.
//! - Explicit transformation and fingerprint policies.
//! - Validation after deserialization.
//!
//! ## DOES NOT OWN
//!
//! - AST nodes, semantic analysis, IR nodes, or builders.
//! - Dialect registration, namespace authorization, or extension execution.
//! - Cryptographic hashing or canonical serialization.
//! - Source-map construction, diagnostics rendering, or runtime behavior.
//! - Physical resource allocation or target-specific layout.
//!
//! ## SCALABILITY
//!
//! No universal hardware or metadata-count ceiling is imposed. Collections
//! are dynamically sized. Processing untrusted input requires resource
//! budgets at the decoding and integration boundaries. Finite hardware
//! cannot provide infinite memory, address space, or execution time.
//!
//! ## DETERMINISM
//!
//! Metadata is stored in `BTreeMap`, giving deterministic key iteration.
//! Array ordering in attribute values is preserved. This module does not
//! claim that arbitrary serializer output is canonical.
//!
//! ## TRUST BOUNDARY
//!
//! Serde deserialization can construct values without calling validating
//! constructors. Call `validate` or `validate_against` before accepting
//! decoded metadata into a verified representation. Extension authorization
//! remains the responsibility of the dialect or extension registry.
//!
//! ## DEPENDENCY DIRECTION
//!
//! This module depends only on `schema::attributes` and `schema::types`.
//! It does not depend on IR nodes, builders, verification implementations,
//! serialization pipelines, or backend code.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

use super::attributes::{
    AttributeError, AttributeKey, AttributeValue,
};
use super::types::SchemaType;

// =============================================================================
// Policies
// =============================================================================

/// The role of a metadata entry.
///
/// Classification alone does not determine trust. In particular, metadata
/// that claims to be descriptive must not be allowed to override semantic
/// rules owned by another subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataClass {
    /// Changes or constrains the meaning of the represented entity.
    Semantic,

    /// Participates in validity checks but is not itself program behavior.
    Verification,

    /// Records origins, transformations, or other audit information.
    Provenance,

    /// Human-readable descriptions and documentation.
    Descriptive,

    /// Editor, debugger, visualization, or other tooling information.
    Tooling,
}

/// Whether an entry contributes to a semantic fingerprint.
///
/// The actual fingerprint algorithm and canonical byte encoding belong to
/// the fingerprinting subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintPolicy {
    /// Include the entry in the semantic fingerprint input.
    Include,

    /// Exclude the entry from the semantic fingerprint input.
    Exclude,
}

/// How a semantics-preserving transformation must treat an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformPolicy {
    /// The transformation must retain the value unchanged.
    Preserve,

    /// The transformation may update the value under its own documented
    /// correctness contract. The consumer must not assume it is unchanged.
    MayUpdate,

    /// The transformation must recalculate the value before the output is
    /// accepted. A stale value is not valid output.
    Recompute,

    /// The transformation may discard the value because it is non-semantic
    /// and the transformation contract permits its removal.
    Discard,
}

/// Whether a metadata entry is required in persisted representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistencePolicy {
    /// Persist this entry when serializing the containing metadata set.
    Persist,

    /// The entry is transient and may be omitted from persisted output.
    Ephemeral,
}

/// How validation treats keys that have no local definition.
///
/// Preserving an unknown entry does not authorize interpreting or executing
/// it. Extension registration and trust checks remain external.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownMetadataPolicy {
    /// Reject keys that are not declared by the schema.
    Reject,

    /// Preserve structurally valid unknown keys for forward compatibility.
    Preserve,
}

// =============================================================================
// Metadata definitions
// =============================================================================

/// A declaration describing one metadata key.
///
/// Definitions are schema contracts, not metadata values. They can be
/// assembled by core schema code or by a registered extension. A definition
/// must be validated before it is trusted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataDefinition {
    key: AttributeKey,
    value_type: SchemaType,
    class: MetadataClass,
    fingerprint: FingerprintPolicy,
    transform: TransformPolicy,
    persistence: PersistencePolicy,
    required: bool,
}

impl MetadataDefinition {
    /// Create a metadata definition.
    ///
    /// Inconsistent policies are rejected. Semantic metadata cannot be
    /// excluded from fingerprints, made ephemeral, or silently discarded.
    pub fn new(
        key: AttributeKey,
        value_type: SchemaType,
        class: MetadataClass,
        fingerprint: FingerprintPolicy,
        transform: TransformPolicy,
        persistence: PersistencePolicy,
        required: bool,
    ) -> Result<Self, MetadataError> {
        let definition = Self {
            key,
            value_type,
            class,
            fingerprint,
            transform,
            persistence,
            required,
        };

        definition.validate()?;
        Ok(definition)
    }

    /// The qualified metadata key.
    #[must_use]
    pub fn key(&self) -> &AttributeKey {
        &self.key
    }

    /// The required value schema.
    #[must_use]
    pub fn value_type(&self) -> &SchemaType {
        &self.value_type
    }

    /// The metadata classification.
    #[must_use]
    pub fn class(&self) -> MetadataClass {
        self.class
    }

    /// The fingerprint participation policy.
    #[must_use]
    pub fn fingerprint_policy(&self) -> FingerprintPolicy {
        self.fingerprint
    }

    /// The transformation policy.
    #[must_use]
    pub fn transform_policy(&self) -> TransformPolicy {
        self.transform
    }

    /// The persistence policy.
    #[must_use]
    pub fn persistence_policy(&self) -> PersistencePolicy {
        self.persistence
    }

    /// Whether this key must be present.
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Revalidate all invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        self.key.validate()?;
        self.value_type
            .validate()
            .map_err(MetadataError::InvalidSchemaType)?;

        if self.class == MetadataClass::Semantic {
            if self.fingerprint != FingerprintPolicy::Include {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata must participate in semantic fingerprints",
                });
            }

            if self.persistence != PersistencePolicy::Persist {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata must be persisted",
                });
            }

            if self.transform == TransformPolicy::Discard {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata cannot use the discard policy",
                });
            }
        }

        if self.class == MetadataClass::Verification
            && self.transform == TransformPolicy::Discard
        {
            return Err(MetadataError::InvalidPolicy {
                key: self.key.clone(),
                reason: "verification metadata cannot be silently discarded",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Metadata entries
// =============================================================================

/// One validated metadata value and its declared policies.
///
/// Policy fields are carried with the value so that transformation consumers
/// do not have to guess how an entry should be treated. A schema should still
/// validate entries against its registered definitions at trust boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataEntry {
    key: AttributeKey,
    value: AttributeValue,
    class: MetadataClass,
    fingerprint: FingerprintPolicy,
    transform: TransformPolicy,
    persistence: PersistencePolicy,
}

impl MetadataEntry {
    /// Construct an entry using the policies of a validated definition.
    pub fn from_definition(
        definition: &MetadataDefinition,
        value: AttributeValue,
    ) -> Result<Self, MetadataError> {
        definition.validate()?;
        value.validate()?;

        let conforms = value
            .conforms_to(definition.value_type())
            .map_err(MetadataError::InvalidAttribute)?;

        if !conforms {
            return Err(MetadataError::ValueTypeMismatch {
                key: definition.key().clone(),
            });
        }

        Ok(Self {
            key: definition.key.clone(),
            value,
            class: definition.class,
            fingerprint: definition.fingerprint,
            transform: definition.transform,
            persistence: definition.persistence,
        })
    }

    /// Return the qualified key.
    #[must_use]
    pub fn key(&self) -> &AttributeKey {
        &self.key
    }

    /// Return the metadata value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Return the classification.
    #[must_use]
    pub fn class(&self) -> MetadataClass {
        self.class
    }

    /// Return the fingerprint policy.
    #[must_use]
    pub fn fingerprint_policy(&self) -> FingerprintPolicy {
        self.fingerprint
    }

    /// Return the transformation policy.
    #[must_use]
    pub fn transform_policy(&self) -> TransformPolicy {
        self.transform
    }

    /// Return the persistence policy.
    #[must_use]
    pub fn persistence_policy(&self) -> PersistencePolicy {
        self.persistence
    }

    /// Validate entry-local invariants.
    ///
    /// The definition-backed schema check is performed by
    /// `MetadataSet::validate_against`.
    pub fn validate(&self) -> Result<(), MetadataError> {
        self.key.validate()?;
        self.value.validate()?;

        if self.class == MetadataClass::Semantic {
            if self.fingerprint != FingerprintPolicy::Include {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata must participate in semantic fingerprints",
                });
            }
            if self.persistence != PersistencePolicy::Persist {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata must be persisted",
                });
            }
            if self.transform == TransformPolicy::Discard {
                return Err(MetadataError::InvalidPolicy {
                    key: self.key.clone(),
                    reason: "semantic metadata cannot use the discard policy",
                });
            }
        }

        if self.class == MetadataClass::Verification
            && self.transform == TransformPolicy::Discard
        {
            return Err(MetadataError::InvalidPolicy {
                key: self.key.clone(),
                reason: "verification metadata cannot be silently discarded",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Schema
// =============================================================================

/// The metadata schema for one class of representation entity.
///
/// This schema is intentionally independent of IR nodes. The owning IR or
/// dialect layer decides which schema applies to a program, module, function,
/// operation, type, region, or other entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSchema {
    definitions: BTreeMap<AttributeKey, MetadataDefinition>,
    unknown: UnknownMetadataPolicy,
}

impl MetadataSchema {
    /// Create an empty schema with an explicit unknown-key policy.
    #[must_use]
    pub fn new(unknown: UnknownMetadataPolicy) -> Self {
        Self {
            definitions: BTreeMap::new(),
            unknown,
        }
    }

    /// The schema's unknown-key policy.
    #[must_use]
    pub fn unknown_policy(&self) -> UnknownMetadataPolicy {
        self.unknown
    }

    /// Insert a definition.
    ///
    /// Duplicate definitions are rejected rather than silently overwritten.
    pub fn insert(
        &mut self,
        definition: MetadataDefinition,
    ) -> Result<(), MetadataError> {
        definition.validate()?;

        if self.definitions.contains_key(definition.key()) {
            return Err(MetadataError::DuplicateDefinition(
                definition.key().clone(),
            ));
        }

        self.definitions
            .insert(definition.key().clone(), definition);
        Ok(())
    }

    /// Look up a definition.
    #[must_use]
    pub fn get(&self, key: &AttributeKey) -> Option<&MetadataDefinition> {
        self.definitions.get(key)
    }

    /// Iterate definitions in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &MetadataDefinition)> {
        self.definitions.iter()
    }

    /// Number of definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Whether this schema has no definitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Validate the schema after construction or deserialization.
    pub fn validate(&self) -> Result<(), MetadataError> {
        for (key, definition) in &self.definitions {
            definition.validate()?;
            if key != definition.key() {
                return Err(MetadataError::DefinitionKeyMismatch);
            }
        }
        Ok(())
    }
}

// =============================================================================
// Metadata set
// =============================================================================

/// A deterministic collection of metadata entries.
///
/// One entry is permitted per qualified key. If a metadata concept needs
/// multiple values, encode its value as an array or define a richer schema
/// contract instead of relying on accidental duplicate-key behavior.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MetadataSet {
    entries: BTreeMap<AttributeKey, MetadataEntry>,
}

impl MetadataSet {
    /// Create an empty metadata set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an entry, rejecting duplicate keys.
    pub fn insert(&mut self, entry: MetadataEntry) -> Result<(), MetadataError> {
        entry.validate()?;

        if self.entries.contains_key(entry.key()) {
            return Err(MetadataError::DuplicateEntry(entry.key().clone()));
        }

        self.entries.insert(entry.key().clone(), entry);
        Ok(())
    }

    /// Replace an existing entry explicitly.
    ///
    /// This operation is intentionally distinct from `insert` so that
    /// overwrites cannot happen accidentally.
    pub fn replace(
        &mut self,
        entry: MetadataEntry,
    ) -> Result<Option<MetadataEntry>, MetadataError> {
        entry.validate()?;
        Ok(self.entries.insert(entry.key().clone(), entry))
    }

    /// Get an entry by qualified key.
    #[must_use]
    pub fn get(&self, key: &AttributeKey) -> Option<&MetadataEntry> {
        self.entries.get(key)
    }

    /// Remove an entry explicitly.
    ///
    /// Callers that perform transformations must first consult its
    /// transformation policy. This low-level collection operation does not
    /// authorize semantic changes.
    pub fn remove(&mut self, key: &AttributeKey) -> Option<MetadataEntry> {
        self.entries.remove(key)
    }

    /// Iterate entries in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&AttributeKey, &MetadataEntry)> {
        self.entries.iter()
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate each entry's local invariants.
    pub fn validate(&self) -> Result<(), MetadataError> {
        for (key, entry) in &self.entries {
            entry.validate()?;
            if key != entry.key() {
                return Err(MetadataError::EntryKeyMismatch);
            }
        }
        Ok(())
    }

    /// Validate all entries against a metadata schema.
    ///
    /// Required definitions must be present. Unknown entries follow the
    /// schema's explicit policy. Unknown entries are never executed here.
    pub fn validate_against(
        &self,
        schema: &MetadataSchema,
    ) -> Result<(), MetadataError> {
        self.validate()?;
        schema.validate()?;

        for (key, entry) in &self.entries {
            match schema.get(key) {
                Some(definition) => {
                    if entry.class != definition.class
                        || entry.fingerprint != definition.fingerprint
                        || entry.transform != definition.transform
                        || entry.persistence != definition.persistence
                    {
                        return Err(MetadataError::PolicyMismatch {
                            key: key.clone(),
                        });
                    }

                    let conforms = entry
                        .value
                        .conforms_to(definition.value_type())
                        .map_err(MetadataError::InvalidAttribute)?;

                    if !conforms {
                        return Err(MetadataError::ValueTypeMismatch {
                            key: key.clone(),
                        });
                    }
                }
                None => {
                    if schema.unknown == UnknownMetadataPolicy::Reject {
                        return Err(MetadataError::UnknownMetadata(key.clone()));
                    }
                }
            }
        }

        for (key, definition) in schema.iter() {
            if definition.is_required() && !self.entries.contains_key(key) {
                return Err(MetadataError::MissingRequiredMetadata(key.clone()));
            }
        }

        Ok(())
    }

    /// Check whether a semantics-preserving transformation may remove an
    /// entry.
    ///
    /// `Discard` is allowed only for entries whose declared policy permits
    /// it. Semantic and verification entries cannot be discarded because
    /// their local validation rejects that policy.
    pub fn ensure_removal_allowed(
        &self,
        key: &AttributeKey,
    ) -> Result<(), MetadataError> {
        let Some(entry) = self.entries.get(key) else {
            return Ok(());
        };

        match entry.transform {
            TransformPolicy::Discard => Ok(()),
            TransformPolicy::Preserve => Err(MetadataError::RemovalForbidden {
                key: key.clone(),
                reason: "the entry must be preserved",
            }),
            TransformPolicy::MayUpdate => Err(MetadataError::RemovalForbidden {
                key: key.clone(),
                reason: "the entry may be updated but not silently removed",
            }),
            TransformPolicy::Recompute => Err(MetadataError::RemovalForbidden {
                key: key.clone(),
                reason: "the entry must be recomputed before output is accepted",
            }),
        }
    }

    /// Return entries that participate in the semantic fingerprint.
    ///
    /// The caller must feed these entries into the canonical fingerprint
    /// implementation. This method does not hash or serialize values.
    pub fn fingerprint_entries(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &MetadataEntry)> {
        self.entries.iter().filter(|(_, entry)| {
            entry.fingerprint_policy() == FingerprintPolicy::Include
        })
    }

    /// Return entries that must be persisted.
    pub fn persisted_entries(
        &self,
    ) -> impl Iterator<Item = (&AttributeKey, &MetadataEntry)> {
        self.entries.iter().filter(|(_, entry)| {
            entry.persistence_policy() == PersistencePolicy::Persist
        })
    }

    /// Validate transformation output against the declared metadata schema.
    ///
    /// Entries marked `Recompute` must be supplied in the output; this
    /// function does not itself calculate their values. The owning
    /// transformation pipeline must perform the recalculation.
    pub fn validate_transformation_output(
        &self,
        output: &Self,
        schema: &MetadataSchema,
    ) -> Result<(), MetadataError> {
        self.validate_against(schema)?;
        output.validate_against(schema)?;

        for (key, source) in &self.entries {
            let target = output.entries.get(key);

            match source.transform {
                TransformPolicy::Preserve => match target {
                    Some(target) if target == source => {}
                    _ => {
                        return Err(MetadataError::TransformationViolation {
                            key: key.clone(),
                            reason: "entry declared Preserve but output differs or omits it",
                        });
                    }
                },
                TransformPolicy::Recompute => {
                    if target.is_none() {
                        return Err(MetadataError::TransformationViolation {
                            key: key.clone(),
                            reason: "entry declared Recompute but output omits it",
                        });
                    }
                }
                TransformPolicy::MayUpdate | TransformPolicy::Discard => {}
            }
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors raised by metadata construction, validation, or policy checking.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetadataError {
    /// Invalid qualified metadata key or metadata value.
    InvalidAttribute(AttributeError),

    /// A schema type failed its own validation.
    InvalidSchemaType(super::types::SchemaTypeError),

    /// A key was defined more than once.
    DuplicateDefinition(AttributeKey),

    /// An entry was inserted more than once.
    DuplicateEntry(AttributeKey),

    /// A deserialized schema's map key disagrees with its definition.
    DefinitionKeyMismatch,

    /// A deserialized set's map key disagrees with its entry.
    EntryKeyMismatch,

    /// The value does not match the declared schema type.
    ValueTypeMismatch {
        key: AttributeKey,
    },

    /// A key is not known to the selected schema.
    UnknownMetadata(AttributeKey),

    /// A required metadata key is absent.
    MissingRequiredMetadata(AttributeKey),

    /// Entry policy differs from its authoritative definition.
    PolicyMismatch {
        key: AttributeKey,
    },

    /// A combination of policies is not valid.
    InvalidPolicy {
        key: AttributeKey,
        reason: &'static str,
    },

    /// An operation would violate a transformation policy.
    RemovalForbidden {
        key: AttributeKey,
        reason: &'static str,
    },

    /// Transformation output violates a metadata contract.
    TransformationViolation {
        key: AttributeKey,
        reason: &'static str,
    },
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAttribute(error) => {
                write!(f, "invalid metadata attribute: {error}")
            }
            Self::InvalidSchemaType(error) => {
                write!(f, "invalid metadata schema type: {error}")
            }
            Self::DuplicateDefinition(key) => {
                write!(f, "duplicate metadata definition: {key}")
            }
            Self::DuplicateEntry(key) => {
                write!(f, "duplicate metadata entry: {key}")
            }
            Self::DefinitionKeyMismatch => {
                f.write_str("metadata schema map key differs from its definition key")
            }
            Self::EntryKeyMismatch => {
                f.write_str("metadata set map key differs from its entry key")
            }
            Self::ValueTypeMismatch { key } => {
                write!(f, "metadata value does not match its declared type: {key}")
            }
            Self::UnknownMetadata(key) => {
                write!(f, "unknown metadata key: {key}")
            }
            Self::MissingRequiredMetadata(key) => {
                write!(f, "required metadata key is missing: {key}")
            }
            Self::PolicyMismatch { key } => {
                write!(f, "metadata policies disagree with the schema: {key}")
            }
            Self::InvalidPolicy { key, reason } => {
                write!(f, "invalid policy for metadata {key}: {reason}")
            }
            Self::RemovalForbidden { key, reason } => {
                write!(f, "cannot remove metadata {key}: {reason}")
            }
            Self::TransformationViolation { key, reason } => {
                write!(f, "metadata transformation violation for {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for MetadataError {}

impl From<AttributeError> for MetadataError {
    fn from(error: AttributeError) -> Self {
        Self::InvalidAttribute(error)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> AttributeKey {
        AttributeKey::parse(name).expect("test key must be valid")
    }

    fn definition(
        name: &str,
        class: MetadataClass,
        fingerprint: FingerprintPolicy,
        transform: TransformPolicy,
        persistence: PersistencePolicy,
        required: bool,
    ) -> MetadataDefinition {
        MetadataDefinition::new(
            key(name),
            SchemaType::String,
            class,
            fingerprint,
            transform,
            persistence,
            required,
        )
        .expect("test definition must be valid")
    }

    fn entry(definition: &MetadataDefinition, value: &str) -> MetadataEntry {
        MetadataEntry::from_definition(
            definition,
            AttributeValue::String(value.to_owned()),
        )
        .expect("test entry must be valid")
    }

    #[test]
    fn semantic_metadata_requires_fingerprint_and_persistence() {
        assert!(MetadataDefinition::new(
            key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Exclude,
            TransformPolicy::Preserve,
            PersistencePolicy::Persist,
            true,
        )
        .is_err());

        assert!(MetadataDefinition::new(
            key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Preserve,
            PersistencePolicy::Ephemeral,
            true,
        )
        .is_err());
    }

    #[test]
    fn semantic_metadata_cannot_be_discarded() {
        assert!(MetadataDefinition::new(
            key("core::meaning"),
            SchemaType::String,
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Discard,
            PersistencePolicy::Persist,
            true,
        )
        .is_err());
    }

    #[test]
    fn duplicate_definitions_are_rejected() {
        let definition = definition(
            "core::label",
            MetadataClass::Descriptive,
            FingerprintPolicy::Exclude,
            TransformPolicy::Discard,
            PersistencePolicy::Persist,
            false,
        );

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        schema.insert(definition.clone()).expect("first definition");

        assert!(matches!(
            schema.insert(definition),
            Err(MetadataError::DuplicateDefinition(_))
        ));
    }

    #[test]
    fn schema_checks_required_keys_and_types() {
        let required = definition(
            "core::label",
            MetadataClass::Descriptive,
            FingerprintPolicy::Exclude,
            TransformPolicy::Discard,
            PersistencePolicy::Persist,
            true,
        );

        let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        schema.insert(required).expect("definition");

        assert!(matches!(
            MetadataSet::new().validate_against(&schema),
            Err(MetadataError::MissingRequiredMetadata(_))
        ));

        let wrong_type = AttributeValue::Boolean(true);
        let required = schema.get(&key("core::label")).expect("definition");
        assert!(matches!(
            MetadataEntry::from_definition(required, wrong_type),
            Err(MetadataError::ValueTypeMismatch { .. })
        ));
    }

    #[test]
    fn unknown_metadata_policy_is_explicit() {
        let extension = MetadataDefinition::new(
            key("example::flag"),
            SchemaType::Boolean,
            MetadataClass::Tooling,
            FingerprintPolicy::Exclude,
            TransformPolicy::Discard,
            PersistencePolicy::Ephemeral,
            false,
        )
        .expect("definition");

        let value = entry(
            &MetadataDefinition::new(
                key("example::flag"),
                SchemaType::String,
                MetadataClass::Tooling,
                FingerprintPolicy::Exclude,
                TransformPolicy::Discard,
                PersistencePolicy::Ephemeral,
                false,
            )
            .expect("string definition"),
            "retained",
        );

        let mut set = MetadataSet::new();
        set.insert(value).expect("entry");

        let rejecting = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        assert!(matches!(
            set.validate_against(&rejecting),
            Err(MetadataError::UnknownMetadata(_))
        ));

        let preserving = MetadataSchema::new(UnknownMetadataPolicy::Preserve);
        assert!(set.validate_against(&preserving).is_ok());

        // Keep the variable used to make it explicit that a future extension
        // may define the same namespaced key under its own schema.
        assert_eq!(extension.key(), &key("example::flag"));
    }

    #[test]
    fn fingerprint_iteration_is_deterministic_and_policy_aware() {
        let included = definition(
            "core::semantic_id",
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Preserve,
            PersistencePolicy::Persist,
            false,
        );
        let excluded = definition(
            "core::description",
            MetadataClass::Descriptive,
            FingerprintPolicy::Exclude,
            TransformPolicy::Discard,
            PersistencePolicy::Persist,
            false,
        );

        let mut set = MetadataSet::new();
        set.insert(entry(&excluded, "documentation"))
            .expect("descriptive entry");
        set.insert(entry(&included, "meaning"))
            .expect("semantic entry");

        let keys: Vec<String> = set
            .fingerprint_entries()
            .map(|(key, _)| key.to_string())
            .collect();

        assert_eq!(keys, vec!["core::semantic_id"]);
    }

    #[test]
    fn preserved_metadata_must_not_change_during_transformation() {
        let preserved = definition(
            "core::semantic_id",
            MetadataClass::Semantic,
            FingerprintPolicy::Include,
            TransformPolicy::Preserve,
            PersistencePolicy::Persist,
            true,
        );

        let schema = {
            let mut schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
            schema.insert(preserved.clone()).expect("definition");
            schema
        };

        let mut input = MetadataSet::new();
        input
            .insert(entry(&preserved, "before"))
            .expect("input entry");

        let mut output = MetadataSet::new();
        output
            .insert(entry(&preserved, "after"))
            .expect("output entry");

        assert!(matches!(
            input.validate_transformation_output(&output, &schema),
            Err(MetadataError::TransformationViolation { .. })
        ));
    }

    #[test]
    fn malformed_deserialized_policy_combinations_are_rejected() {
        // Public constructors prevent this combination, but deserialization
        // can create values without invoking those constructors.
        let invalid = MetadataDefinition {
            key: key("core::meaning"),
            value_type: SchemaType::String,
            class: MetadataClass::Semantic,
            fingerprint: FingerprintPolicy::Exclude,
            transform: TransformPolicy::Preserve,
            persistence: PersistencePolicy::Persist,
            required: false,
        };

        assert!(matches!(
            invalid.validate(),
            Err(MetadataError::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn empty_schema_and_set_are_valid() {
        let schema = MetadataSchema::new(UnknownMetadataPolicy::Reject);
        let set = MetadataSet::new();

        assert!(schema.validate().is_ok());
        assert!(set.validate().is_ok());
        assert!(set.validate_against(&schema).is_ok());
        assert!(set.is_empty());
        assert_eq!(schema.len(), 0);
    }
}
