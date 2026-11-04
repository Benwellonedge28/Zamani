
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Logical Dataset Schemas
//!
//! Path: src/ai/data/schemas.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Ownership
//!
//! This module owns AI dataset schema definitions, schema fields,
//! schema evolution, compatibility assessment, and structural validation.
//!
//! It does not own:
//!
//! - Dataset identities or DatasetSchemaRef.
//! - The canonical semantic type system.
//! - Tensor storage.
//! - Dataset storage or materialization.
//! - Parsing or AST construction.
//! - Hardware selection or execution.
//!
//! DatasetSchemaRef is owned by data/datasets.rs.
//!
//! ## POCO-REAF
//!
//! No arbitrary schema, field, nesting, tensor-rank, or version-count
//! ceiling is imposed by this module.
//!
//! Operational resource limits remain the responsibility of the caller.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Canonical type integration
//!
//! CanonicalTypeRef identifies a type in the authoritative Zamani
//! semantic type system. It does not implement another type system.
//!
//! The canonical semantic adapter resolves these references and performs
//! type-specific checks that this module cannot establish independently.
//!
//! ## Integration
//!
//! - data/datasets.rs owns DatasetSchemaRef.
//! - data/validation.rs validates dataset values.
//! - data/preprocessing.rs validates transformations.
//! - data/provenance.rs records schema lineage.
//! - frontend/semantic owns canonical type semantics.
//! - grammar/data/datasets.g4 and grammar/ai/datasets.g4 own syntax.
//!
//! This module does not parse source text or construct an AST.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::data::datasets::DatasetSchemaRef;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn schema_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::DataProcessing)
}

fn validate_text(
    description: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(schema_error(
            AiErrorKind::InvalidInput,
            format!("{description} cannot be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(schema_error(
            AiErrorKind::InvalidInput,
            format!("{description} contains control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// CANONICAL TYPE REFERENCE
// ============================================================================

/// An opaque reference to a type owned by the canonical semantic type system.
///
/// This type identifies a canonical type. It does not define its semantics.
///
/// The namespace and identifier are resolved by the frontend semantic
/// adapter or another authoritative type-system integration.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct CanonicalTypeRef {
    namespace: String,
    identifier: String,
}

impl CanonicalTypeRef {
    /// Create a canonical type reference.
    ///
    /// # Errors
    ///
    /// Returns an input error if either component is invalid.
    pub fn new(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let identifier = identifier.into();

        validate_text("type namespace", &namespace)?;
        validate_text("type identifier", &identifier)?;

        if namespace.chars().any(char::is_whitespace)
            || identifier.chars().any(char::is_whitespace)
        {
            return Err(schema_error(
                AiErrorKind::InvalidInput,
                "canonical type references cannot contain whitespace",
            ));
        }

        Ok(Self {
            namespace,
            identifier,
        })
    }

    /// Return the canonical type namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the canonical type identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Return the canonical display representation.
    ///
    /// This representation is not a serialization protocol.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.identifier)
    }
}

impl fmt::Display for CanonicalTypeRef {
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
// FIELD CONSTRAINTS
// ============================================================================

/// An extensible, declarative field constraint.
///
/// Constraint semantics are interpreted by the canonical semantic
/// validator or the relevant data validation adapter.
///
/// This type stores a constraint reference and does not execute it.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub struct FieldConstraint {
    namespace: String,
    identifier: String,
    expression: Option<String>,
}

impl FieldConstraint {
    /// Create a field constraint.
    ///
    /// The optional expression is an opaque canonical expression reference.
    ///
    /// # Errors
    ///
    /// Returns an input error if the namespace or identifier is invalid.
    pub fn new(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
        expression: Option<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let identifier = identifier.into();

        validate_text("constraint namespace", &namespace)?;
        validate_text("constraint identifier", &identifier)?;

        if let Some(expression) = &expression {
            validate_text("constraint expression", expression)?;
        }

        Ok(Self {
            namespace,
            identifier,
            expression,
        })
    }

    /// Return the constraint namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the constraint identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Return the optional canonical expression reference.
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }
}

// ============================================================================
// FIELD DEFINITION
// ============================================================================

/// A logical field in a dataset schema.
///
/// The canonical type reference is authoritative for type semantics.
///
/// Nullability, constraints, and metadata are schema-level declarations.
/// Physical encoding and storage layout belong downstream.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct SchemaField {
    name: String,
    canonical_type: CanonicalTypeRef,
    nullable: bool,
    constraints: BTreeSet<FieldConstraint>,
    metadata: BTreeMap<String, String>,
}

impl SchemaField {
    /// Create a schema field.
    ///
    /// # Errors
    ///
    /// Returns an input error if the name is invalid.
    pub fn new(
        name: impl Into<String>,
        canonical_type: CanonicalTypeRef,
        nullable: bool,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_text("schema field name", &name)?;

        Ok(Self {
            name,
            canonical_type,
            nullable,
            constraints: BTreeSet::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Return the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical type reference.
    #[must_use]
    pub fn canonical_type(&self) -> &CanonicalTypeRef {
        &self.canonical_type
    }

    /// Return whether the field permits null values.
    #[must_use]
    pub const fn nullable(&self) -> bool {
        self.nullable
    }

    /// Return the field constraints.
    #[must_use]
    pub fn constraints(&self) -> &BTreeSet<FieldConstraint> {
        &self.constraints
    }

    /// Return field metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Add a constraint.
    pub fn add_constraint(
        &mut self,
        constraint: FieldConstraint,
    ) {
        self.constraints.insert(constraint);
    }

    /// Set a metadata entry.
    ///
    /// Replaces an existing value for the same key.
    ///
    /// # Errors
    ///
    /// Returns an input error if the key is invalid.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiResult<()> {
        let key = key.into();
        let value = value.into();

        validate_text("field metadata key", &key)?;

        self.metadata.insert(key, value);

        Ok(())
    }
}

// ============================================================================
// SCHEMA VERSION
// ============================================================================

/// A schema version.
///
/// Versions are explicit logical identities. They are not assumed to be
/// semantic-versioning numbers or chronological timestamps.
///
/// The schema authority determines the versioning policy.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct SchemaVersion(String);

impl SchemaVersion {
    /// Create a schema version.
    ///
    /// # Errors
    ///
    /// Returns an input error if the version is empty or malformed.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text("schema version", &value)?;

        if value.chars().any(char::is_whitespace) {
            return Err(schema_error(
                AiErrorKind::InvalidInput,
                "schema version cannot contain whitespace",
            ));
        }

        Ok(Self(value))
    }

    /// Return the version string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// SCHEMA DEFINITION
// ============================================================================

/// A logical dataset schema.
///
/// A schema defines fields and their canonical type references.
///
/// Field order is preserved because declaration order may be meaningful
/// to downstream formats and source-level tooling.
///
/// Field names must be unique within one schema.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct DatasetSchema {
    reference: DatasetSchemaRef,
    version: SchemaVersion,
    fields: Vec<SchemaField>,
    field_index: BTreeMap<String, usize>,
    metadata: BTreeMap<String, String>,
}

impl DatasetSchema {
    /// Create an empty schema with a canonical reference and version.
    #[must_use]
    pub fn new(
        reference: DatasetSchemaRef,
        version: SchemaVersion,
    ) -> Self {
        Self {
            reference,
            version,
            fields: Vec::new(),
            field_index: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Return the canonical schema reference.
    #[must_use]
    pub fn reference(&self) -> &DatasetSchemaRef {
        &self.reference
    }

    /// Return the schema version.
    #[must_use]
    pub fn version(&self) -> &SchemaVersion {
        &self.version
    }

    /// Return fields in declaration order.
    #[must_use]
    pub fn fields(&self) -> &[SchemaField] {
        &self.fields
    }

    /// Return schema metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return a field by name.
    #[must_use]
    pub fn field(
        &self,
        name: &str,
    ) -> Option<&SchemaField> {
        self.field_index
            .get(name)
            .and_then(|index| self.fields.get(*index))
    }

    /// Add a field.
    ///
    /// Duplicate field names are rejected.
    ///
    /// # Errors
    ///
    /// Returns a conflict error if the field already exists.
    pub fn add_field(
        &mut self,
        field: SchemaField,
    ) -> AiResult<()> {
        if self.field_index.contains_key(field.name()) {
            return Err(schema_error(
                AiErrorKind::InvalidInput,
                format!(
                    "duplicate schema field '{}'",
                    field.name()
                ),
            ));
        }

        let index = self.fields.len();

        self.field_index
            .insert(field.name().to_owned(), index);

        self.fields.push(field);

        Ok(())
    }

    /// Set schema metadata.
    ///
    /// # Errors
    ///
    /// Returns an input error if the key is invalid.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AiResult<()> {
        let key = key.into();
        let value = value.into();

        validate_text("schema metadata key", &key)?;

        self.metadata.insert(key, value);

        Ok(())
    }

    /// Validate structural schema invariants.
    ///
    /// This does not resolve canonical types or prove field constraints.
    pub fn validate_structure(&self) -> AiResult<()> {
        if self.fields.len() != self.field_index.len() {
            return Err(schema_error(
                AiErrorKind::InvalidModel,
                "schema field index is inconsistent",
            ));
        }

        for (index, field) in self.fields.iter().enumerate() {
            match self.field_index.get(field.name()) {
                Some(mapped_index) if *mapped_index == index => {}
                _ => {
                    return Err(schema_error(
                        AiErrorKind::InvalidModel,
                        format!(
                            "schema field index mismatch for '{}'",
                            field.name()
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// COMPATIBILITY
// ============================================================================

/// Direction of a schema compatibility check.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
pub enum CompatibilityDirection {
    /// Existing data can be consumed using the new schema.
    ///
    /// The new schema may add nullable fields or relax constraints.
    Backward,

    /// Data produced under the new schema can be consumed by the old schema.
    ///
    /// The new schema may remove fields or tighten constraints only when
    /// the canonical type adapter establishes that the change is safe.
    Forward,

    /// Both backward and forward compatibility are required.
    Full,
}

/// Compatibility result.
///
/// `Incompatible` means a known incompatibility was found.
///
/// `Undetermined` means canonical type or constraint analysis is required
/// before a definitive compatibility answer can be established.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub enum CompatibilityResult {
    /// Compatibility was established.
    Compatible,

    /// A known incompatibility was found.
    Incompatible {
        /// Human-readable explanation.
        reason: String,
    },

    /// Compatibility requires canonical semantic analysis.
    Undetermined {
        /// Explanation of the unresolved obligation.
        reason: String,
    },
}

/// Compare two schema definitions structurally.
///
/// This function deliberately does not claim type compatibility when
/// canonical type references differ. That determination belongs to the
/// authoritative semantic type system.
pub fn compare_schema_structure(
    old: &DatasetSchema,
    new: &DatasetSchema,
    direction: CompatibilityDirection,
) -> CompatibilityResult {
    let old_names: BTreeSet<_> = old
        .fields()
        .iter()
        .map(SchemaField::name)
        .collect();

    let new_names: BTreeSet<_> = new
        .fields()
        .iter()
        .map(SchemaField::name)
        .collect();

    match direction {
        CompatibilityDirection::Backward => {
            for old_field in old.fields() {
                match new.field(old_field.name()) {
                    None => {
                        return CompatibilityResult::Incompatible {
                            reason: format!(
                                "field '{}' was removed",
                                old_field.name()
                            ),
                        };
                    }

                    Some(new_field)
                        if old_field.canonical_type()
                            != new_field.canonical_type() =>
                    {
                        return CompatibilityResult::Undetermined {
                            reason: format!(
                                "canonical type compatibility for '{}' \
                                 requires semantic analysis",
                                old_field.name()
                            ),
                        };
                    }

                    Some(new_field)
                        if old_field.nullable()
                            && !new_field.nullable() =>
                    {
                        return CompatibilityResult::Incompatible {
                            reason: format!(
                                "field '{}' became non-nullable",
                                old_field.name()
                            ),
                        };
                    }

                    _ => {}
                }
            }

            for new_field in new.fields() {
                if !old_names.contains(&new_field.name())
                    && !new_field.nullable()
                {
                    return CompatibilityResult::Incompatible {
                        reason: format!(
                            "new field '{}' is non-nullable",
                            new_field.name()
                        ),
                    };
                }
            }
        }

        CompatibilityDirection::Forward => {
            for new_field in new.fields() {
                match old.field(new_field.name()) {
                    None => {
                        return CompatibilityResult::Incompatible {
                            reason: format!(
                                "new field '{}' is absent from old schema",
                                new_field.name()
                            ),
                        };
                    }

                    Some(old_field)
                        if old_field.canonical_type()
                            != new_field.canonical_type() =>
                    {
                        return CompatibilityResult::Undetermined {
                            reason: format!(
                                "canonical type compatibility for '{}' \
                                 requires semantic analysis",
                                new_field.name()
                            ),
                        };
                    }

                    Some(old_field)
                        if !old_field.nullable()
                            && new_field.nullable() =>
                    {
                        return CompatibilityResult::Incompatible {
                            reason: format!(
                                "field '{}' became nullable",
                                new_field.name()
                            ),
                        };
                    }

                    _ => {}
                }
            }

            for old_field in old.fields() {
                if !new_names.contains(&old_field.name())
                    && !old_field.nullable()
                {
                    return CompatibilityResult::Incompatible {
                        reason: format!(
                            "removed field '{}' was non-nullable",
                            old_field.name()
                        ),
                    };
                }
            }
        }

        CompatibilityDirection::Full => {
            let backward = compare_schema_structure(
                old,
                new,
                CompatibilityDirection::Backward,
            );

            let forward = compare_schema_structure(
                old,
                new,
                CompatibilityDirection::Forward,
            );

            if backward != CompatibilityResult::Compatible {
                return backward;
            }

            if forward != CompatibilityResult::Compatible {
                return forward;
            }
        }
    }

    CompatibilityResult::Compatible
}

// ============================================================================
// SCHEMA EVOLUTION
// ============================================================================

/// A declarative schema evolution operation.
///
/// Operations describe changes; they do not mutate storage or dataset values.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub enum SchemaEvolution {
    /// Add a field.
    AddField(SchemaField),

    /// Remove a field by name.
    RemoveField(String),

    /// Replace a field definition.
    ReplaceField(SchemaField),

    /// Rename a field.
    RenameField {
        /// Existing field name.
        from: String,

        /// New field name.
        to: String,
    },
}

/// Apply a schema evolution sequence to a logical schema.
///
/// The result is a new schema. The input schema remains unchanged.
///
/// This operation validates structural consistency but does not establish
/// data-level compatibility or canonical type equivalence.
pub fn evolve_schema(
    schema: &DatasetSchema,
    version: SchemaVersion,
    changes: &[SchemaEvolution],
) -> AiResult<DatasetSchema> {
    let mut evolved = DatasetSchema::new(
        schema.reference().clone(),
        version,
    );

    for field in schema.fields() {
        evolved.add_field(field.clone())?;
    }

    for change in changes {
        match change {
            SchemaEvolution::AddField(field) => {
                evolved.add_field(field.clone())?;
            }

            SchemaEvolution::RemoveField(name) => {
                if !evolved.field_index.contains_key(name) {
                    return Err(schema_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "cannot remove unknown field '{name}'"
                        ),
                    ));
                }

                evolved.fields.retain(
                    |field| field.name() != name,
                );

                evolved.field_index.clear();

                for (index, field) in
                    evolved.fields.iter().enumerate()
                {
                    evolved.field_index.insert(
                        field.name().to_owned(),
                        index,
                    );
                }
            }

            SchemaEvolution::ReplaceField(field) => {
                let Some(index) = evolved
                    .field_index
                    .get(field.name())
                    .copied()
                else {
                    return Err(schema_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "cannot replace unknown field '{}'",
                            field.name()
                        ),
                    ));
                };

                evolved.fields[index] = field.clone();
            }

            SchemaEvolution::RenameField { from, to } => {
                validate_text("new field name", to)?;

                if evolved.field_index.contains_key(to) {
                    return Err(schema_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "field '{to}' already exists"
                        ),
                    ));
                }

                let Some(index) = evolved
                    .field_index
                    .get(from)
                    .copied()
                else {
                    return Err(schema_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "cannot rename unknown field '{from}'"
                        ),
                    ));
                };

                let previous = &evolved.fields[index];

                let mut replacement = SchemaField::new(
                    to.clone(),
                    previous.canonical_type().clone(),
                    previous.nullable(),
                )?;

                for constraint in previous.constraints() {
                    replacement.add_constraint(
                        constraint.clone(),
                    );
                }

                for (key, value) in previous.metadata() {
                    replacement.set_metadata(
                        key.clone(),
                        value.clone(),
                    )?;
                }

                evolved.fields[index] = replacement;

                evolved.field_index.remove(from);
                evolved.field_index.insert(
                    to.clone(),
                    index,
                );
            }
        }
    }

    evolved.validate_structure()?;

    Ok(evolved)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn schema_ref() -> DatasetSchemaRef {
        DatasetSchemaRef::new(
            "ai.dataset",
            "training",
        )
        .expect("valid schema reference")
    }

    fn canonical_type(
        name: &str,
    ) -> CanonicalTypeRef {
        CanonicalTypeRef::new(
            "zamani.types",
            name,
        )
        .expect("valid canonical type")
    }

    fn field(
        name: &str,
        nullable: bool,
    ) -> SchemaField {
        SchemaField::new(
            name,
            canonical_type("f32"),
            nullable,
        )
        .expect("valid field")
    }

    fn schema(
        version: &str,
        fields: Vec<SchemaField>,
    ) -> DatasetSchema {
        let mut schema = DatasetSchema::new(
            schema_ref(),
            SchemaVersion::new(version)
                .expect("valid version"),
        );

        for field in fields {
            schema.add_field(field)
                .expect("unique field");
        }

        schema
    }

    #[test]
    fn schema_preserves_declaration_order() {
        let schema = schema(
            "1",
            vec![
                field("first", false),
                field("second", true),
            ],
        );

        assert_eq!(
            schema.fields()[0].name(),
            "first"
        );

        assert_eq!(
            schema.fields()[1].name(),
            "second"
        );
    }

    #[test]
    fn duplicate_fields_are_rejected() {
        let mut schema = schema("1", vec![]);

        schema.add_field(field("value", false))
            .expect("first field");

        assert!(
            schema.add_field(field("value", false))
                .is_err()
        );
    }

    #[test]
    fn schema_field_lookup_is_deterministic() {
        let schema = schema(
            "1",
            vec![
                field("a", false),
                field("b", true),
            ],
        );

        assert_eq!(
            schema.field("b").map(SchemaField::name),
            Some("b")
        );

        assert!(schema.field("missing").is_none());
    }

    #[test]
    fn adding_nullable_field_is_backward_compatible() {
        let old = schema(
            "1",
            vec![field("value", false)],
        );

        let new = schema(
            "2",
            vec![
                field("value", false),
                field("optional", true),
            ],
        );

        assert_eq!(
            compare_schema_structure(
                &old,
                &new,
                CompatibilityDirection::Backward,
            ),
            CompatibilityResult::Compatible,
        );
    }

    #[test]
    fn adding_non_nullable_field_is_not_backward_compatible() {
        let old = schema(
            "1",
            vec![field("value", false)],
        );

        let new = schema(
            "2",
            vec![
                field("value", false),
                field("required", false),
            ],
        );

        assert!(matches!(
            compare_schema_structure(
                &old,
                &new,
                CompatibilityDirection::Backward,
            ),
            CompatibilityResult::Incompatible { .. }
        ));
    }

    #[test]
    fn changed_canonical_type_requires_semantic_analysis() {
        let old = schema(
            "1",
            vec![field("value", false)],
        );

        let mut new_field = SchemaField::new(
            "value",
            canonical_type("f64"),
            false,
        )
        .expect("valid field");

        new_field.add_constraint(
            FieldConstraint::new(
                "zamani.constraints",
                "finite",
                None,
            )
            .expect("valid constraint"),
        );

        let new = schema(
            "2",
            vec![new_field],
        );

        assert!(matches!(
            compare_schema_structure(
                &old,
                &new,
                CompatibilityDirection::Backward,
            ),
            CompatibilityResult::Undetermined { .. }
        ));
    }

    #[test]
    fn schema_evolution_preserves_original() {
        let original = schema(
            "1",
            vec![field("value", false)],
        );

        let evolved = evolve_schema(
            &original,
            SchemaVersion::new("2")
                .expect("version"),
            &[
                SchemaEvolution::AddField(
                    field("optional", true),
                ),
            ],
        )
        .expect("evolution");

        assert_eq!(original.fields().len(), 1);
        assert_eq!(evolved.fields().len(), 2);
    }

    #[test]
    fn schema_evolution_rejects_unknown_removal() {
        let original = schema(
            "1",
            vec![field("value", false)],
        );

        assert!(
            evolve_schema(
                &original,
                SchemaVersion::new("2")
                    .expect("version"),
                &[
                    SchemaEvolution::RemoveField(
                        "missing".to_owned(),
                    ),
                ],
            )
            .is_err()
        );
    }

    #[test]
    fn rename_preserves_field_constraints() {
        let mut original_field = field("before", false);

        original_field.add_constraint(
            FieldConstraint::new(
                "zamani.constraints",
                "finite",
                None,
            )
            .expect("constraint"),
        );

        let original = schema(
            "1",
            vec![original_field],
        );

        let evolved = evolve_schema(
            &original,
            SchemaVersion::new("2")
                .expect("version"),
            &[
                SchemaEvolution::RenameField {
                    from: "before".to_owned(),
                    to: "after".to_owned(),
                },
            ],
        )
        .expect("evolution");

        assert!(evolved.field("before").is_none());

        assert_eq!(
            evolved.field("after")
                .expect("renamed field")
                .constraints()
                .len(),
            1,
        );
    }
}
