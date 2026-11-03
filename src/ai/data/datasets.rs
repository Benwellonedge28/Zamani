
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Logical Dataset Definitions
//!
//! Path: src/ai/data/datasets.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines target-independent logical datasets consumed by AI computation.
//!
//! A dataset describes what data is available, its logical schema reference,
//! its source references, its field roles and its descriptive metadata.
//!
//! It does not implement storage, transport, materialization, preprocessing,
//! batching, streaming, physical partitioning or execution.
//!
//! ## Canonical ownership
//!
//! Dataset identity:
//!     crate::ai::ids::AiDatasetId
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI provenance:
//!     crate::ai::provenance
//!
//! Shared semantic schema:
//!     crate::frontend::semantic
//!
//! Canonical classical tensor storage:
//!     crate::classical::collections::tensor
//!
//! Dataset source syntax:
//!     grammar/data/datasets.g4
//!
//! AI dataset syntax:
//!     grammar/ai/datasets.g4
//!
//! ## POCO-REAF
//!
//! No language-level maximum is imposed on:
//!
//! - Dataset count.
//! - Dataset sources.
//! - Dataset fields.
//! - Feature declarations.
//! - Label declarations.
//! - Target declarations.
//! - Metadata entries.
//! - Dataset references.
//! - Dataset size.
//! - Record count.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Distributed partitions.
//!
//! All collections grow according to actual input and available resources.
//!
//! This module does not promise literal infinity. It avoids arbitrary
//! architectural ceilings and preserves logical meaning independently
//! of the eventual physical realization.
//!
//! ## Important distinction
//!
//! A DatasetDefinition is not a dataset execution engine.
//!
//! It describes logical dataset intent.
//!
//! Actual storage, loading, streaming, preprocessing, batching, validation,
//! partitioning and execution belong to downstream subsystems.
//!
//! ## Integration
//!
//! - data/schemas.rs resolves DatasetSchemaRef.
//! - data/batching.rs consumes AiDatasetId.
//! - data/streaming.rs consumes AiDatasetId and source references.
//! - data/preprocessing.rs consumes AiDatasetId and transformation references.
//! - data/validation.rs performs complete dataset validation.
//! - data/provenance.rs or crate::ai::provenance records dataset lineage.
//! - learning/training.rs consumes AiDatasetId.
//! - learning/federated.rs consumes AiDatasetId.
//! - data/mod.rs exports this module.
//! - ai/mod.rs exports the data subsystem.
//!
//! ## Serialization
//!
//! This file deliberately does not define a competing serialization format.
//! Dataset artifact serialization belongs to the canonical data and
//! deployment serialization infrastructure.
//!
//! Untrusted serialized data must pass through validated constructors
//! before being accepted as a DatasetDefinition.

use core::fmt;

use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::AiDatasetId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn dataset_error(
    kind: AiErrorKind,
    message: impl Into<String>,
    dataset_id: Option<AiDatasetId>,
) -> AiError {
    let error = AiError::new(kind, message)
        .with_phase(AiErrorPhase::DataProcessing);

    match dataset_id {
        Some(id) => error.with_dataset(id.to_string()),
        None => error,
    }
}

fn validate_text(
    description: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(dataset_error(
            AiErrorKind::InvalidInput,
            format!("{description} cannot be empty"),
            None,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(dataset_error(
            AiErrorKind::InvalidInput,
            format!("{description} contains control characters"),
            None,
        ));
    }

    Ok(())
}

fn validate_qualified_component(
    description: &str,
    value: &str,
) -> AiResult<()> {
    validate_text(description, value)?;

    if value.chars().any(char::is_whitespace) {
        return Err(dataset_error(
            AiErrorKind::InvalidInput,
            format!(
                "{description} must not contain whitespace"
            ),
            None,
        ));
    }

    Ok(())
}

// ============================================================================
// DATASET SCHEMA REFERENCE
// ============================================================================

/// A reference to a schema owned by the canonical data/schema subsystem.
///
/// This is a reference only. It does not define a competing schema model.
///
/// The namespace identifies the schema authority or logical namespace.
/// The identifier identifies the schema within that namespace.
///
/// Neither value is interpreted as a filesystem path, database name,
/// physical storage location or hardware identifier.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DatasetSchemaRef {
    namespace: String,
    identifier: String,
}

impl DatasetSchemaRef {
    /// Create a validated logical schema reference.
    ///
    /// # Errors
    ///
    /// Returns an AI input error when either component is invalid.
    pub fn new(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let identifier = identifier.into();

        validate_qualified_component(
            "schema namespace",
            &namespace,
        )?;

        validate_text(
            "schema identifier",
            &identifier,
        )?;

        Ok(Self {
            namespace,
            identifier,
        })
    }

    /// Return the schema namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the schema identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Return the canonical textual reference.
    ///
    /// This is a display representation, not a serialization protocol.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.identifier)
    }
}

impl fmt::Display for DatasetSchemaRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.identifier
        )
    }
}

// ============================================================================
// DATASET SOURCE KIND
// ============================================================================

/// An extensible classification of a logical dataset source.
///
/// Source kinds are open-world strings rather than a closed enumeration.
///
/// Standard classifications are provided for convenience, but extensions
/// may introduce additional classifications without modifying this module.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DatasetSourceKind(String);

impl DatasetSourceKind {
    /// Create an extensible source classification.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_qualified_component(
            "dataset source kind",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Expression-backed logical source.
    #[must_use]
    pub fn expression() -> Self {
        Self("expression".to_owned())
    }

    /// Another logical AI dataset.
    #[must_use]
    pub fn dataset() -> Self {
        Self("dataset".to_owned())
    }

    /// Logical streaming source.
    #[must_use]
    pub fn stream() -> Self {
        Self("stream".to_owned())
    }

    /// Generated data source.
    #[must_use]
    pub fn generated() -> Self {
        Self("generated".to_owned())
    }

    /// External logical source.
    #[must_use]
    pub fn external() -> Self {
        Self("external".to_owned())
    }

    /// Return the source classification.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DatasetSourceKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// DATASET SOURCE REFERENCE
// ============================================================================

/// A target-independent reference to a logical dataset source.
///
/// This reference does not open, contact, load, allocate or execute anything.
///
/// Its namespace and identifier are interpreted by the appropriate
/// semantic, interoperability or execution adapter.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DatasetSourceRef {
    kind: DatasetSourceKind,
    namespace: String,
    identifier: String,
}

impl DatasetSourceRef {
    /// Construct a source reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the kind, namespace or identifier is invalid.
    pub fn new(
        kind: DatasetSourceKind,
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let identifier = identifier.into();

        validate_qualified_component(
            "source namespace",
            &namespace,
        )?;

        validate_text(
            "source identifier",
            &identifier,
        )?;

        Ok(Self {
            kind,
            namespace,
            identifier,
        })
    }

    /// Construct a reference to a canonical logical expression.
    ///
    /// The expression is supplied by the canonical frontend adapter.
    /// This module does not parse or reinterpret it.
    pub fn expression(
        reference: impl Into<String>,
    ) -> AiResult<Self> {
        Self::new(
            DatasetSourceKind::expression(),
            "zamani.expression",
            reference,
        )
    }

    /// Construct a reference to another AI dataset.
    pub fn dataset(
        id: AiDatasetId,
    ) -> AiResult<Self> {
        Self::new(
            DatasetSourceKind::dataset(),
            "ai.dataset",
            id.to_string(),
        )
    }

    /// Construct a logical streaming-source reference.
    ///
    /// Actual stream semantics are owned by data/streaming.rs.
    pub fn stream(
        reference: impl Into<String>,
    ) -> AiResult<Self> {
        Self::new(
            DatasetSourceKind::stream(),
            "data.stream",
            reference,
        )
    }

    /// Construct a generated-data source reference.
    ///
    /// Generator execution belongs to the appropriate execution subsystem.
    pub fn generated(
        generator_namespace: impl Into<String>,
        generator_reference: impl Into<String>,
    ) -> AiResult<Self> {
        Self::new(
            DatasetSourceKind::generated(),
            generator_namespace,
            generator_reference,
        )
    }

    /// Construct an external logical source reference.
    ///
    /// The authority and reference remain opaque. No network or filesystem
    /// operation is performed.
    pub fn external(
        authority: impl Into<String>,
        reference: impl Into<String>,
    ) -> AiResult<Self> {
        Self::new(
            DatasetSourceKind::external(),
            authority,
            reference,
        )
    }

    /// Return the source classification.
    #[must_use]
    pub fn kind(&self) -> &DatasetSourceKind {
        &self.kind
    }

    /// Return the source namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the opaque source identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl fmt::Display for DatasetSourceRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}::{}",
            self.kind,
            self.namespace,
            self.identifier
        )
    }
}

// ============================================================================
// DATASET FIELD REFERENCE
// ============================================================================

/// A reference to a field declared by a canonical dataset schema.
///
/// This type identifies a logical field. It does not define its type,
/// nullability, constraints, encoding or physical representation.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DatasetFieldRef(String);

impl DatasetFieldRef {
    /// Create a logical dataset field reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "dataset field reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the original field reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DatasetFieldRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// DATASET FIELD ROLE
// ============================================================================

/// An extensible semantic role assigned to a dataset field.
///
/// Standard roles are provided, but the classification remains open-world.
///
/// A role does not replace the canonical field type or schema definition.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DatasetFieldRole(String);

impl DatasetFieldRole {
    /// Construct an extensible field role.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_qualified_component(
            "dataset field role",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Feature/input role.
    #[must_use]
    pub fn feature() -> Self {
        Self("feature".to_owned())
    }

    /// Label role.
    #[must_use]
    pub fn label() -> Self {
        Self("label".to_owned())
    }

    /// Prediction-target role.
    #[must_use]
    pub fn target() -> Self {
        Self("target".to_owned())
    }

    /// Sample-weight role.
    #[must_use]
    pub fn weight() -> Self {
        Self("weight".to_owned())
    }

    /// Grouping role.
    #[must_use]
    pub fn group() -> Self {
        Self("group".to_owned())
    }

    /// Record-identity role.
    #[must_use]
    pub fn identifier() -> Self {
        Self("identifier".to_owned())
    }

    /// Descriptive metadata role.
    #[must_use]
    pub fn metadata() -> Self {
        Self("metadata".to_owned())
    }

    /// Return the role classification.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DatasetFieldRole {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// DATASET DEFINITION
// ============================================================================

/// A target-independent logical AI dataset definition.
///
/// This structure describes dataset intent and logical relationships.
///
/// It does not contain dataset records or physical storage buffers.
///
/// Schema definitions, streaming behavior, batching, transformations,
/// validation, provenance and execution remain owned by their respective
/// subsystems.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatasetDefinition {
    id: AiDatasetId,

    name: String,

    schema: Option<DatasetSchemaRef>,

    sources: Vec<DatasetSourceRef>,

    field_roles: BTreeMap<
        DatasetFieldRef,
        BTreeSet<DatasetFieldRole>,
    >,

    metadata: BTreeMap<String, String>,
}

impl DatasetDefinition {
    /// Create a logical dataset definition.
    ///
    /// Dataset identities must come from the canonical AI identity
    /// infrastructure.
    ///
    /// # Errors
    ///
    /// Returns an error if the dataset name is invalid.
    pub fn new(
        id: AiDatasetId,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_text(
            "dataset name",
            &name,
        )?;

        Ok(Self {
            id,
            name,
            schema: None,
            sources: Vec::new(),
            field_roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        })
    }

    // ---------------------------------------------------------------------
    // Identity and descriptive information
    // ---------------------------------------------------------------------

    /// Return the canonical dataset identity.
    #[must_use]
    pub const fn id(&self) -> AiDatasetId {
        self.id
    }

    /// Return the logical dataset name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Explicitly replace the logical dataset name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> AiResult<()> {
        let name = name.into();

        validate_text(
            "dataset name",
            &name,
        )?;

        self.name = name;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Schema references
    // ---------------------------------------------------------------------

    /// Return the optional canonical schema reference.
    #[must_use]
    pub fn schema(&self) -> Option<&DatasetSchemaRef> {
        self.schema.as_ref()
    }

    /// Associate a canonical logical schema reference.
    ///
    /// This operation does not establish that the referenced schema exists
    /// or is compatible. Those checks belong to data/schemas.rs and
    /// data/validation.rs.
    pub fn set_schema(
        &mut self,
        schema: DatasetSchemaRef,
    ) {
        self.schema = Some(schema);
    }

    /// Builder-style schema association.
    #[must_use]
    pub fn with_schema(
        mut self,
        schema: DatasetSchemaRef,
    ) -> Self {
        self.set_schema(schema);
        self
    }

    /// Remove the schema reference.
    ///
    /// The returned value is the previous reference.
    pub fn clear_schema(
        &mut self,
    ) -> Option<DatasetSchemaRef> {
        self.schema.take()
    }

    // ---------------------------------------------------------------------
    // Source references
    // ---------------------------------------------------------------------

    /// Return all logical source references in declaration order.
    #[must_use]
    pub fn sources(&self) -> &[DatasetSourceRef] {
        &self.sources
    }

    /// Add a logical source reference.
    ///
    /// Duplicate references are rejected rather than silently duplicated.
    ///
    /// Source insertion order is preserved.
    ///
    /// # Errors
    ///
    /// Returns a conflict error if the same source is already present.
    pub fn add_source(
        &mut self,
        source: DatasetSourceRef,
    ) -> AiResult<()> {
        if self.sources.contains(&source) {
            return Err(dataset_error(
                AiErrorKind::Conflict,
                format!(
                    "dataset source is already registered: {source}"
                ),
                Some(self.id),
            ));
        }

        self.sources.push(source);

        Ok(())
    }

    /// Builder-style source insertion.
    pub fn with_source(
        mut self,
        source: DatasetSourceRef,
    ) -> AiResult<Self> {
        self.add_source(source)?;
        Ok(self)
    }

    /// Remove a source reference.
    ///
    /// Returns whether a matching source was removed.
    pub fn remove_source(
        &mut self,
        source: &DatasetSourceRef,
    ) -> bool {
        if let Some(position) = self.sources
            .iter()
            .position(|existing| existing == source)
        {
            self.sources.remove(position);
            true
        } else {
            false
        }
    }

    /// Remove all source references.
    ///
    /// This changes the logical definition only. It does not delete
    /// underlying data or external resources.
    pub fn clear_sources(&mut self) {
        self.sources.clear();
    }

    // ---------------------------------------------------------------------
    // Field roles
    // ---------------------------------------------------------------------

    /// Return all field-role assignments in deterministic field order.
    #[must_use]
    pub fn field_roles(
        &self,
    ) -> &BTreeMap<
        DatasetFieldRef,
        BTreeSet<DatasetFieldRole>,
    > {
        &self.field_roles
    }

    /// Return all roles assigned to a field.
    #[must_use]
    pub fn roles_for_field(
        &self,
        field: &DatasetFieldRef,
    ) -> Option<&BTreeSet<DatasetFieldRole>> {
        self.field_roles.get(field)
    }

    /// Assign a semantic role to a dataset field.
    ///
    /// A field may have multiple roles where the semantic model permits it.
    ///
    /// This method does not verify that the field exists in the referenced
    /// schema. Complete schema validation belongs downstream.
    ///
    /// # Errors
    ///
    /// Returns a conflict error if the role has already been assigned.
    pub fn assign_role(
        &mut self,
        field: DatasetFieldRef,
        role: DatasetFieldRole,
    ) -> AiResult<()> {
        let roles = self.field_roles
            .entry(field.clone())
            .or_default();

        if !roles.insert(role.clone()) {
            return Err(dataset_error(
                AiErrorKind::Conflict,
                format!(
                    "role `{role}` is already assigned to field `{field}`"
                ),
                Some(self.id),
            ));
        }

        Ok(())
    }

    /// Remove a particular field-role assignment.
    ///
    /// Empty role sets are removed from the internal map.
    pub fn remove_role(
        &mut self,
        field: &DatasetFieldRef,
        role: &DatasetFieldRole,
    ) -> bool {
        let Some(roles) = self.field_roles.get_mut(field) else {
            return false;
        };

        let removed = roles.remove(role);

        if roles.is_empty() {
            self.field_roles.remove(field);
        }

        removed
    }

    /// Return all fields assigned a particular role.
    ///
    /// The iterator follows deterministic field ordering.
    pub fn fields_with_role<'a>(
        &'a self,
        role: &'a DatasetFieldRole,
    ) -> impl Iterator<Item = &'a DatasetFieldRef> + 'a {
        self.field_roles
            .iter()
            .filter_map(move |(field, roles)| {
                roles.contains(role).then_some(field)
            })
    }

    /// Return whether the dataset contains a particular role.
    #[must_use]
    pub fn has_role(
        &self,
        role: &DatasetFieldRole,
    ) -> bool {
        self.field_roles
            .values()
            .any(|roles| roles.contains(role))
    }

    // ---------------------------------------------------------------------
    // Metadata
    // ---------------------------------------------------------------------

    /// Return metadata in deterministic key order.
    #[must_use]
    pub fn metadata(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return a metadata value.
    #[must_use]
    pub fn metadata_value(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.metadata
            .get(key)
            .map(String::as_str)
    }

    /// Insert metadata without replacing an existing value.
    ///
    /// # Errors
    ///
    /// Returns a conflict error if the key already exists.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiResult<()> {
        let key = key.into();
        let value = value.into();

        validate_qualified_component(
            "dataset metadata key",
            &key,
        )?;

        validate_text(
            "dataset metadata value",
            &value,
        )?;

        if self.metadata.contains_key(&key) {
            return Err(dataset_error(
                AiErrorKind::Conflict,
                format!(
                    "dataset metadata key already exists: {key}"
                ),
                Some(self.id),
            ));
        }

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Insert or explicitly replace metadata.
    ///
    /// Returns the previous value when one existed.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_qualified_component(
            "dataset metadata key",
            &key,
        )?;

        validate_text(
            "dataset metadata value",
            &value,
        )?;

        Ok(self.metadata.insert(key, value))
    }

    /// Remove a metadata entry.
    pub fn remove_metadata(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.metadata.remove(key)
    }

    // ---------------------------------------------------------------------
    // Structural validation
    // ---------------------------------------------------------------------

    /// Validate invariants owned by this logical dataset definition.
    ///
    /// This is not a substitute for complete semantic or schema validation.
    ///
    /// It does not establish:
    ///
    /// - Schema existence.
    /// - Source availability.
    /// - Field existence.
    /// - Field type compatibility.
    /// - Dataset integrity.
    /// - Resource feasibility.
    /// - Execution capability.
    ///
    /// Those checks belong to their authoritative subsystems.
    ///
    /// # Errors
    ///
    /// Returns an error when an internal structural invariant is violated.
    pub fn validate_structure(&self) -> AiResult<()> {
        validate_text(
            "dataset name",
            &self.name,
        )
        .map_err(|error| {
            error.with_dataset(self.id.to_string())
        })?;

        let mut sources = BTreeSet::new();

        for source in &self.sources {
            if !sources.insert(source) {
                return Err(dataset_error(
                    AiErrorKind::DataValidationFailure,
                    format!(
                        "duplicate dataset source: {source}"
                    ),
                    Some(self.id),
                ));
            }
        }

        for (field, roles) in &self.field_roles {
            if roles.is_empty() {
                return Err(dataset_error(
                    AiErrorKind::DataValidationFailure,
                    format!(
                        "dataset field `{field}` has an empty role set"
                    ),
                    Some(self.id),
                ));
            }
        }

        for (key, value) in &self.metadata {
            validate_qualified_component(
                "dataset metadata key",
                key,
            )
            .map_err(|error| {
                error.with_dataset(self.id.to_string())
            })?;

            validate_text(
                "dataset metadata value",
                value,
            )
            .map_err(|error| {
                error.with_dataset(self.id.to_string())
            })?;
        }

        Ok(())
    }

    /// Return whether the dataset has any logical source references.
    ///
    /// An empty source list is not inherently invalid: an abstract,
    /// deferred or subsequently composed dataset may have no source
    /// references at this stage.
    #[must_use]
    pub fn has_sources(&self) -> bool {
        !self.sources.is_empty()
    }

    /// Return whether the dataset has an associated schema.
    #[must_use]
    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    /// Return the number of declared field-role associations.
    ///
    /// This counts individual field-role pairs, not unique fields.
    #[must_use]
    pub fn role_assignment_count(&self) -> usize {
        self.field_roles
            .values()
            .map(BTreeSet::len)
            .sum()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dataset_id(value: u128) -> AiDatasetId {
        AiDatasetId::new(value)
            .expect("dataset identity must be nonzero")
    }

    fn dataset() -> DatasetDefinition {
        DatasetDefinition::new(
            dataset_id(1),
            "training",
        )
        .expect("valid dataset")
    }

    fn field(value: &str) -> DatasetFieldRef {
        DatasetFieldRef::new(value)
            .expect("valid field")
    }

    #[test]
    fn creates_logical_dataset() {
        let value = dataset();

        assert_eq!(value.id(), dataset_id(1));
        assert_eq!(value.name(), "training");
        assert!(!value.has_schema());
        assert!(!value.has_sources());
    }

    #[test]
    fn rejects_empty_dataset_name() {
        assert!(
            DatasetDefinition::new(
                dataset_id(1),
                "   ",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_dataset_name() {
        assert!(
            DatasetDefinition::new(
                dataset_id(1),
                "training\ninvalid",
            )
            .is_err()
        );
    }

    #[test]
    fn dataset_names_can_be_changed_explicitly() {
        let mut value = dataset();

        value.set_name("validation")
            .expect("valid replacement");

        assert_eq!(value.name(), "validation");
    }

    #[test]
    fn schema_reference_is_target_independent() {
        let schema = DatasetSchemaRef::new(
            "application.schemas",
            "training_record",
        )
        .expect("valid schema");

        assert_eq!(
            schema.namespace(),
            "application.schemas"
        );

        assert_eq!(
            schema.identifier(),
            "training_record"
        );

        assert_eq!(
            schema.qualified_name(),
            "application.schemas::training_record"
        );
    }

    #[test]
    fn rejects_invalid_schema_reference() {
        assert!(
            DatasetSchemaRef::new(
                "",
                "training",
            )
            .is_err()
        );

        assert!(
            DatasetSchemaRef::new(
                "schemas",
                "   ",
            )
            .is_err()
        );
    }

    #[test]
    fn associates_and_removes_schema() {
        let schema = DatasetSchemaRef::new(
            "schemas",
            "training",
        )
        .expect("valid schema");

        let mut value = dataset();

        value.set_schema(schema.clone());

        assert_eq!(value.schema(), Some(&schema));

        assert_eq!(
            value.clear_schema(),
            Some(schema)
        );

        assert!(!value.has_schema());
    }

    #[test]
    fn creates_expression_source() {
        let source = DatasetSourceRef::expression(
            "training_source",
        )
        .expect("valid source");

        assert_eq!(
            source.kind().as_str(),
            "expression"
        );

        assert_eq!(
            source.namespace(),
            "zamani.expression"
        );

        assert_eq!(
            source.identifier(),
            "training_source"
        );
    }

    #[test]
    fn creates_dataset_source() {
        let source = DatasetSourceRef::dataset(
            dataset_id(42),
        )
        .expect("valid dataset reference");

        assert_eq!(
            source.kind().as_str(),
            "dataset"
        );

        assert_eq!(
            source.identifier(),
            "42"
        );
    }

    #[test]
    fn creates_stream_source() {
        let source = DatasetSourceRef::stream(
            "sensor_events",
        )
        .expect("valid stream reference");

        assert_eq!(
            source.kind().as_str(),
            "stream"
        );

        assert_eq!(
            source.namespace(),
            "data.stream"
        );
    }

    #[test]
    fn creates_external_source_without_accessing_it() {
        let source = DatasetSourceRef::external(
            "external.provider",
            "logical-reference",
        )
        .expect("valid external reference");

        assert_eq!(
            source.kind().as_str(),
            "external"
        );

        assert_eq!(
            source.identifier(),
            "logical-reference"
        );
    }

    #[test]
    fn supports_future_source_kinds() {
        let source = DatasetSourceRef::new(
            DatasetSourceKind::new(
                "future.quantum_generated",
            )
            .expect("valid source kind"),
            "future.provider",
            "generated-source",
        )
        .expect("valid source");

        assert_eq!(
            source.kind().as_str(),
            "future.quantum_generated"
        );
    }

    #[test]
    fn rejects_duplicate_sources() {
        let source = DatasetSourceRef::expression(
            "training",
        )
        .expect("valid source");

        let mut value = dataset();

        value.add_source(source.clone())
            .expect("first insertion");

        let result = value.add_source(source);

        assert!(result.is_err());

        assert_eq!(value.sources().len(), 1);
    }

    #[test]
    fn preserves_source_declaration_order() {
        let first = DatasetSourceRef::expression(
            "first",
        )
        .expect("first source");

        let second = DatasetSourceRef::expression(
            "second",
        )
        .expect("second source");

        let mut value = dataset();

        value.add_source(first.clone())
            .expect("first insertion");

        value.add_source(second.clone())
            .expect("second insertion");

        assert_eq!(
            value.sources(),
            &[first, second]
        );
    }

    #[test]
    fn removes_sources_without_deleting_external_data() {
        let source = DatasetSourceRef::expression(
            "source",
        )
        .expect("valid source");

        let mut value = dataset();

        value.add_source(source.clone())
            .expect("insertion");

        assert!(value.remove_source(&source));
        assert!(!value.remove_source(&source));
        assert!(!value.has_sources());
    }

    #[test]
    fn supports_standard_field_roles() {
        assert_eq!(
            DatasetFieldRole::feature().as_str(),
            "feature"
        );

        assert_eq!(
            DatasetFieldRole::label().as_str(),
            "label"
        );

        assert_eq!(
            DatasetFieldRole::target().as_str(),
            "target"
        );
    }

    #[test]
    fn supports_open_world_field_roles() {
        let role = DatasetFieldRole::new(
            "quantum_measurement_feature",
        )
        .expect("future role");

        assert_eq!(
            role.as_str(),
            "quantum_measurement_feature"
        );
    }

    #[test]
    fn assigns_feature_and_label_roles() {
        let mut value = dataset();

        value.assign_role(
            field("input"),
            DatasetFieldRole::feature(),
        )
        .expect("feature assignment");

        value.assign_role(
            field("expected_output"),
            DatasetFieldRole::label(),
        )
        .expect("label assignment");

        assert_eq!(
            value.role_assignment_count(),
            2
        );

        assert!(
            value.has_role(
                &DatasetFieldRole::feature()
            )
        );

        assert!(
            value.has_role(
                &DatasetFieldRole::label()
            )
        );
    }

    #[test]
    fn a_field_can_have_multiple_roles() {
        let mut value = dataset();

        let input = field("input");

        value.assign_role(
            input.clone(),
            DatasetFieldRole::feature(),
        )
        .expect("feature assignment");

        value.assign_role(
            input.clone(),
            DatasetFieldRole::weight(),
        )
        .expect("weight assignment");

        assert_eq!(
            value.roles_for_field(&input)
                .expect("field roles")
                .len(),
            2
        );
    }

    #[test]
    fn duplicate_role_assignment_is_rejected() {
        let mut value = dataset();

        let input = field("input");

        value.assign_role(
            input.clone(),
            DatasetFieldRole::feature(),
        )
        .expect("first assignment");

        assert!(
            value.assign_role(
                input,
                DatasetFieldRole::feature(),
            )
            .is_err()
        );
    }

    #[test]
    fn removes_roles_and_empty_field_entries() {
        let mut value = dataset();

        let input = field("input");
        let feature = DatasetFieldRole::feature();

        value.assign_role(
            input.clone(),
            feature.clone(),
        )
        .expect("assignment");

        assert!(
            value.remove_role(
                &input,
                &feature,
            )
        );

        assert!(
            value.roles_for_field(&input).is_none()
        );
    }

    #[test]
    fn finds_fields_by_role_deterministically() {
        let mut value = dataset();

        value.assign_role(
            field("z_feature"),
            DatasetFieldRole::feature(),
        )
        .expect("assignment");

        value.assign_role(
            field("a_feature"),
            DatasetFieldRole::feature(),
        )
        .expect("assignment");

        let fields: Vec<_> = value
            .fields_with_role(
                &DatasetFieldRole::feature()
            )
            .map(DatasetFieldRef::as_str)
            .collect();

        assert_eq!(
            fields,
            vec!["a_feature", "z_feature"]
        );
    }

    #[test]
    fn metadata_insertion_is_deterministic() {
        let mut value = dataset();

        value.add_metadata(
            "source.version",
            "1",
        )
        .expect("metadata");

        value.add_metadata(
            "dataset.domain",
            "classification",
        )
        .expect("metadata");

        assert_eq!(
            value.metadata_value("source.version"),
            Some("1")
        );

        assert_eq!(
            value.metadata().len(),
            2
        );
    }

    #[test]
    fn duplicate_metadata_is_rejected() {
        let mut value = dataset();

        value.add_metadata(
            "dataset.version",
            "1",
        )
        .expect("first insertion");

        assert!(
            value.add_metadata(
                "dataset.version",
                "2",
            )
            .is_err()
        );
    }

    #[test]
    fn metadata_replacement_is_explicit() {
        let mut value = dataset();

        value.add_metadata(
            "dataset.version",
            "1",
        )
        .expect("first insertion");

        let previous = value.set_metadata(
            "dataset.version",
            "2",
        )
        .expect("replacement");

        assert_eq!(
            previous.as_deref(),
            Some("1")
        );

        assert_eq!(
            value.metadata_value("dataset.version"),
            Some("2")
        );
    }

    #[test]
    fn rejects_invalid_metadata_keys() {
        let mut value = dataset();

        assert!(
            value.add_metadata(
                "",
                "value",
            )
            .is_err()
        );
    }

    #[test]
    fn validates_a_complete_logical_dataset() {
        let mut value = dataset();

        value.set_schema(
            DatasetSchemaRef::new(
                "schemas",
                "training",
            )
            .expect("schema")
        );

        value.add_source(
            DatasetSourceRef::expression(
                "training_source",
            )
            .expect("source")
        )
        .expect("source insertion");

        value.assign_role(
            field("features"),
            DatasetFieldRole::feature(),
        )
        .expect("feature");

        value.assign_role(
            field("labels"),
            DatasetFieldRole::label(),
        )
        .expect("label");

        value.add_metadata(
            "dataset.version",
            "1",
        )
        .expect("metadata");

        assert!(
            value.validate_structure().is_ok()
        );
    }

    #[test]
    fn empty_sources_are_not_automatically_invalid() {
        let value = dataset();

        assert!(
            value.validate_structure().is_ok()
        );
    }

    #[test]
    fn dataset_identity_is_preserved() {
        let original = dataset();

        let cloned = original.clone();

        assert_eq!(
            original.id(),
            cloned.id()
        );
    }

    #[test]
    fn dataset_identity_is_the_canonical_ai_identity() {
        let value = dataset();

        let _: AiDatasetId = value.id();

        assert_eq!(
            value.id().get(),
            1
        );
    }

    #[test]
    fn dataset_metadata_has_deterministic_order() {
        let mut value = dataset();

        value.add_metadata("z", "last")
            .expect("metadata");

        value.add_metadata("a", "first")
            .expect("metadata");

        let keys: Vec<_> = value.metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(
            keys,
            vec!["a", "z"]
        );
    }
}
