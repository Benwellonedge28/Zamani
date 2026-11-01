#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Parameters
//!
//! Path: `src/ai/model/parameters.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent AI model parameter declarations, constraints,
//! default-expression references, initialization metadata, provenance
//! references, and parameter collection validation.
//!
//! A parameter is a semantic declaration. It is not an allocated tensor,
//! a runtime value, a hardware resource, or an executable IR operation.
//!
//! ## Authority
//!
//! Source syntax:
//!     grammar/ai/parameters.g4
//!
//! Canonical AST:
//!     crate::frontend::ast
//!
//! Canonical semantic types:
//!     crate::frontend::semantic::types::ty
//!
//! Canonical expression references:
//!     crate::ai::model::definition::ModelExpressionRef
//!
//! AI parameter identity:
//!     crate::ai::ids::AiParameterId
//!
//! Canonical provenance identity:
//!     crate::frontend::semantic::model::ids::ProvenanceId
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! ## Owns
//!
//! - AI model parameter declaration metadata.
//! - Parameter identity and source-level name.
//! - Canonical semantic type reference.
//! - Default-expression reference.
//! - Declarative parameter constraints.
//! - Initialization metadata.
//! - Parameter trainability and mutability declarations.
//! - Parameter provenance references.
//! - Parameter schema-version metadata.
//! - Local parameter collection validation.
//!
//! ## Does not own
//!
//! - Source parsing or AST construction.
//! - Type definitions or type inference.
//! - Expression ASTs or expression evaluation.
//! - Tensor shape/type authorities.
//! - Tensor storage or materialization.
//! - Random-number generation.
//! - Weight initialization execution.
//! - Training or optimizer execution.
//! - Model graph structure.
//! - ZUIR operation implementation.
//! - Device discovery or hardware selection.
//! - Memory allocation or scheduling.
//! - Distributed execution.
//!
//! ## Scalability
//!
//! No fixed maximum parameter count, tensor rank, model size, parameter
//! shape, number of constraints, metadata entries, or model depth is imposed.
//!
//! Collections grow according to available resources. Their lengths are
//! implementation details, not language-level semantic limits.
//!
//! Allocation failure is not converted into a valid or partially validated
//! parameter set. Standard ordered collections may still abort on allocator
//! exhaustion; this module does not claim recoverable handling of every
//! process-level allocation failure.
//!
//! ## Integration
//!
//! The frontend adapter resolves source names, types, default expressions,
//! constraints, and provenance into canonical semantic references.
//!
//! ModelDefinition owns the overall model declaration and member structure.
//! This module owns parameter-specific declarations and validation.
//!
//! The AI semantic validator performs language-level type, expression,
//! constraint, and policy checks.
//!
//! ZUIR adapters lower validated parameter meaning into canonical IR
//! references and operations. This module does not create a second IR.
//!
//! ## Freeze criteria
//!
//! - Canonical AST adapter supplies resolved parameter declarations.
//! - Canonical semantic types and expression references are reused.
//! - Duplicate parameter identities and names are rejected.
//! - Invalid constraints and initialization metadata are rejected.
//! - No runtime values or tensor storage are introduced.
//! - Parameter collection validation is deterministic.
//! - Rust 1.97+, edition 2021, safe Rust only.
//! - Relevant unit and integration tests pass.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::ai::ids::AiParameterId;
use crate::ai::model::definition::ModelExpressionRef;
use crate::ai::version::AiModelSchemaVersion;
use crate::frontend::semantic::model::ids::ProvenanceId;
use crate::frontend::semantic::types::ty::{SemanticName, SemanticType};

// =============================================================================
// Parameter initialization metadata
// =============================================================================

/// Declarative initialization metadata for a model parameter.
///
/// This type describes the intended initialization policy. It does not
/// generate values, allocate tensor storage, or access random-number
/// generators.
///
/// The policy name is open-ended so extensions can add initialization
/// semantics without requiring a closed universal enumeration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParameterInitialization {
    /// Qualified initialization policy identifier.
    ///
    /// Examples:
    ///
    /// - `ai.initialization.zero`
    /// - `ai.initialization.one`
    /// - `ai.initialization.constant`
    /// - `ai.initialization.uniform`
    /// - `ai.initialization.normal`
    /// - `ai.initialization.kaiming`
    /// - `ai.initialization.xavier`
    /// - `vendor.extension.initialization.custom`
    pub policy: String,

    /// Optional canonical expression reference for policy arguments.
    ///
    /// The referenced expression is owned by the frontend/semantic system.
    /// It is not parsed or evaluated here.
    pub arguments: Option<ModelExpressionRef>,

    /// Optional externally supplied random-stream or seed reference.
    ///
    /// This is an opaque reference, not a generated random seed.
    pub random_stream_reference: Option<String>,
}

impl ParameterInitialization {
    /// Construct initialization metadata with no arguments or random stream.
    pub fn new(
        policy: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let policy = policy.into();

        validate_qualified_identifier(
            "initialization policy",
            &policy,
        )?;

        Ok(Self {
            policy,
            arguments: None,
            random_stream_reference: None,
        })
    }

    /// Attach a canonical expression reference for initialization arguments.
    #[must_use]
    pub fn with_arguments(
        mut self,
        arguments: ModelExpressionRef,
    ) -> Self {
        self.arguments = Some(arguments);
        self
    }

    /// Attach an opaque externally supplied random-stream reference.
    pub fn with_random_stream_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let reference = reference.into();

        validate_non_empty_text(
            "random stream reference",
            &reference,
        )?;

        self.random_stream_reference = Some(reference);

        Ok(self)
    }

    /// Validate the metadata, including values obtained through decoding.
    pub fn validate(&self) -> Result<(), ParameterError> {
        validate_qualified_identifier(
            "initialization policy",
            &self.policy,
        )?;

        if let Some(reference) = &self.random_stream_reference {
            validate_non_empty_text(
                "random stream reference",
                reference,
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Parameter constraints
// =============================================================================

/// A declarative constraint associated with a parameter.
///
/// The constraint expression is represented by a canonical semantic
/// expression reference. This module does not evaluate it.
///
/// Examples of possible source-level constraints include:
///
/// - a positive learning rate;
/// - a permitted range for a scalar;
/// - a relationship between symbolic dimensions;
/// - a policy requirement on a parameter value.
///
/// Whether a constraint is satisfiable is determined by the canonical
/// semantic analysis and, where necessary, the execution system.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParameterConstraint {
    /// Canonical reference to the constraint expression.
    pub expression: ModelExpressionRef,

    /// Optional qualified constraint category.
    ///
    /// This is descriptive metadata. It does not replace expression
    /// validation or semantic checking.
    pub category: Option<String>,

    /// Optional diagnostic description.
    pub description: Option<String>,
}

impl ParameterConstraint {
    /// Construct a constraint from a canonical expression reference.
    #[must_use]
    pub fn new(expression: ModelExpressionRef) -> Self {
        Self {
            expression,
            category: None,
            description: None,
        }
    }

    /// Attach a qualified constraint category.
    pub fn with_category(
        mut self,
        category: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let category = category.into();

        validate_qualified_identifier(
            "constraint category",
            &category,
        )?;

        self.category = Some(category);

        Ok(self)
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let description = description.into();

        validate_non_empty_text(
            "constraint description",
            &description,
        )?;

        self.description = Some(description);

        Ok(self)
    }

    /// Validate constraint metadata.
    pub fn validate(&self) -> Result<(), ParameterError> {
        if let Some(category) = &self.category {
            validate_qualified_identifier(
                "constraint category",
                category,
            )?;
        }

        if let Some(description) = &self.description {
            validate_non_empty_text(
                "constraint description",
                description,
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Parameter metadata
// =============================================================================

/// Extensible metadata associated with a model parameter.
///
/// Metadata is descriptive. It must not be used to bypass canonical
/// semantic validation, security checks, or execution requirements.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParameterMetadata {
    /// Qualified metadata key.
    pub key: String,

    /// Metadata value.
    pub value: String,
}

impl ParameterMetadata {
    /// Construct validated parameter metadata.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ParameterError> {
        let key = key.into();
        let value = value.into();

        validate_qualified_identifier(
            "parameter metadata key",
            &key,
        )?;

        validate_non_empty_text(
            "parameter metadata value",
            &value,
        )?;

        Ok(Self { key, value })
    }

    /// Validate metadata, including values obtained through decoding.
    pub fn validate(&self) -> Result<(), ParameterError> {
        validate_qualified_identifier(
            "parameter metadata key",
            &self.key,
        )?;

        validate_non_empty_text(
            "parameter metadata value",
            &self.value,
        )
    }
}

// =============================================================================
// Parameter declaration
// =============================================================================

/// A target-independent AI model parameter declaration.
///
/// The semantic type is supplied by the canonical semantic type system.
/// Tensor shapes, symbolic dimensions, and type constraints remain owned by
/// that system.
///
/// `default` and `constraints` reference canonical expressions. They do not
/// contain a duplicate expression AST.
///
/// The parameter's actual value, if one exists, is managed by the appropriate
/// model state, tensor, training, or runtime infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelParameter {
    /// Strongly typed AI parameter identity.
    pub id: AiParameterId,

    /// Source-level semantic name.
    pub name: SemanticName,

    /// Canonical resolved semantic type.
    pub ty: SemanticType,

    /// Optional default expression reference.
    pub default: Option<ModelExpressionRef>,

    /// Declarative constraints.
    pub constraints: Vec<ParameterConstraint>,

    /// Optional initialization policy.
    pub initialization: Option<ParameterInitialization>,

    /// Whether the parameter is declared trainable.
    ///
    /// This is a semantic declaration, not permission to mutate a runtime
    /// tensor or a guarantee that every backend can train it.
    pub trainable: bool,

    /// Whether the parameter is declared mutable.
    ///
    /// Mutability semantics are checked by the canonical semantic layer.
    pub mutable: bool,

    /// Extensible descriptive metadata.
    pub metadata: Vec<ParameterMetadata>,

    /// Optional canonical provenance identity.
    ///
    /// The provenance record itself is owned by the canonical provenance
    /// infrastructure.
    pub provenance: Option<ProvenanceId>,

    /// AI model schema version under which this declaration was authored.
    ///
    /// This is schema metadata, not a runtime compatibility guarantee.
    pub schema_version: AiModelSchemaVersion,
}

impl ModelParameter {
    /// Construct a parameter declaration.
    ///
    /// The caller supplies the canonical identity, name, type, and schema
    /// version. Name resolution and type checking remain upstream.
    #[must_use]
    pub fn new(
        id: AiParameterId,
        name: SemanticName,
        ty: SemanticType,
        schema_version: AiModelSchemaVersion,
    ) -> Self {
        Self {
            id,
            name,
            ty,
            default: None,
            constraints: Vec::new(),
            initialization: None,
            trainable: false,
            mutable: false,
            metadata: Vec::new(),
            provenance: None,
            schema_version,
        }
    }

    /// Attach a canonical default-expression reference.
    #[must_use]
    pub fn with_default(
        mut self,
        default: ModelExpressionRef,
    ) -> Self {
        self.default = Some(default);
        self
    }

    /// Append a declarative constraint.
    #[must_use]
    pub fn with_constraint(
        mut self,
        constraint: ParameterConstraint,
    ) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Set initialization metadata.
    #[must_use]
    pub fn with_initialization(
        mut self,
        initialization: ParameterInitialization,
    ) -> Self {
        self.initialization = Some(initialization);
        self
    }

    /// Declare the parameter trainable.
    #[must_use]
    pub fn with_trainable(mut self, trainable: bool) -> Self {
        self.trainable = trainable;
        self
    }

    /// Declare the parameter mutable.
    #[must_use]
    pub fn with_mutable(mut self, mutable: bool) -> Self {
        self.mutable = mutable;
        self
    }

    /// Append validated metadata.
    pub fn with_metadata(
        mut self,
        metadata: ParameterMetadata,
    ) -> Result<Self, ParameterError> {
        metadata.validate()?;
        self.metadata.push(metadata);
        Ok(self)
    }

    /// Attach a canonical provenance identity.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: ProvenanceId,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Validate parameter-local invariants.
    ///
    /// This does not evaluate defaults or constraints and does not perform
    /// canonical type checking.
    pub fn validate(&self) -> Result<(), ParameterError> {
        for constraint in &self.constraints {
            constraint.validate()?;
        }

        if let Some(initialization) = &self.initialization {
            initialization.validate()?;
        }

        for metadata in &self.metadata {
            metadata.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Parameter collection
// =============================================================================

/// A validated parameter collection.
///
/// The collection preserves declaration order. The index is derived from
/// that collection and is not a second mutable source of parameter data.
///
/// Construction validates duplicate identities and duplicate semantic
/// names. The collection does not own model graph structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelParameters {
    parameters: Vec<ModelParameter>,
    by_id: BTreeMap<AiParameterId, usize>,
    by_name: BTreeMap<SemanticName, usize>,
}

impl ModelParameters {
    /// Construct a parameter collection and validate its invariants.
    ///
    /// No fixed parameter count is imposed. Duplicate detection uses ordered
    /// indexes rather than repeated linear scans.
    pub fn new(
        parameters: Vec<ModelParameter>,
    ) -> Result<Self, ParameterError> {
        let mut by_id = BTreeMap::new();
        let mut by_name = BTreeMap::new();

        for (index, parameter) in parameters.iter().enumerate() {
            parameter.validate()?;

            if by_id.insert(parameter.id, index).is_some() {
                return Err(ParameterError::DuplicateIdentity(
                    parameter.id,
                ));
            }

            if by_name.insert(parameter.name.clone(), index).is_some() {
                return Err(ParameterError::DuplicateName);
            }
        }

        Ok(Self {
            parameters,
            by_id,
            by_name,
        })
    }

    /// Return the parameter declarations in source/declaration order.
    #[must_use]
    pub fn as_slice(&self) -> &[ModelParameter] {
        &self.parameters
    }

    /// Consume the collection and return its declarations.
    #[must_use]
    pub fn into_vec(self) -> Vec<ModelParameter> {
        self.parameters
    }

    /// Return the number of declared parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Find a parameter by its canonical identity.
    #[must_use]
    pub fn get_by_id(
        &self,
        id: AiParameterId,
    ) -> Option<&ModelParameter> {
        self.by_id
            .get(&id)
            .map(|&index| &self.parameters[index])
    }

    /// Find a parameter by its canonical semantic name.
    #[must_use]
    pub fn get_by_name(
        &self,
        name: &SemanticName,
    ) -> Option<&ModelParameter> {
        self.by_name
            .get(name)
            .map(|&index| &self.parameters[index])
    }

    /// Iterate over declarations in source/declaration order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &ModelParameter> + '_ {
        self.parameters.iter()
    }

    /// Validate the collection's indexes and parameter-local invariants.
    ///
    /// This is useful after decoding or when checking externally supplied
    /// data. The index is never trusted without checking its consistency.
    pub fn validate(&self) -> Result<(), ParameterError> {
        if self.by_id.len() != self.parameters.len()
            || self.by_name.len() != self.parameters.len()
        {
            return Err(ParameterError::IndexInconsistency);
        }

        for (index, parameter) in self.parameters.iter().enumerate() {
            parameter.validate()?;

            if self.by_id.get(&parameter.id) != Some(&index) {
                return Err(ParameterError::IndexInconsistency);
            }

            if self.by_name.get(&parameter.name) != Some(&index) {
                return Err(ParameterError::IndexInconsistency);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Parameter declaration and collection validation errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParameterError {
    /// A required textual field was empty or contained only whitespace.
    EmptyText {
        field: &'static str,
    },

    /// A qualified identifier was malformed.
    InvalidQualifiedIdentifier {
        field: &'static str,
        value: String,
    },

    /// The parameter identity was already present in the collection.
    DuplicateIdentity(AiParameterId),

    /// A parameter name was already present in the collection.
    DuplicateName,

    /// The derived collection indexes did not match the declarations.
    IndexInconsistency,
}

impl fmt::Display for ParameterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::InvalidQualifiedIdentifier { field, value } => {
                write!(
                    formatter,
                    "invalid {field} identifier: {value:?}"
                )
            }
            Self::DuplicateIdentity(id) => {
                write!(
                    formatter,
                    "duplicate AI parameter identity: {id}"
                )
            }
            Self::DuplicateName => {
                write!(formatter, "duplicate AI parameter name")
            }
            Self::IndexInconsistency => {
                write!(
                    formatter,
                    "AI parameter collection index is inconsistent"
                )
            }
        }
    }
}

impl Error for ParameterError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_non_empty_text(
    field: &'static str,
    value: &str,
) -> Result<(), ParameterError> {
    if value.trim().is_empty() {
        return Err(ParameterError::EmptyText { field });
    }

    Ok(())
}

/// Validate an extensible qualified identifier.
///
/// This intentionally does not impose a finite vocabulary of initialization
/// policies, metadata keys, or constraint categories.
fn validate_qualified_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), ParameterError> {
    validate_non_empty_text(field, value)?;

    let mut segment_has_character = false;

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
        {
            return Err(
                ParameterError::InvalidQualifiedIdentifier {
                    field,
                    value: value.to_owned(),
                },
            );
        }

        if character == '.' {
            if !segment_has_character {
                return Err(
                    ParameterError::InvalidQualifiedIdentifier {
                        field,
                        value: value.to_owned(),
                    },
                );
            }

            segment_has_character = false;
        } else {
            segment_has_character = true;
        }
    }

    if !segment_has_character {
        return Err(
            ParameterError::InvalidQualifiedIdentifier {
                field,
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_initialization_policy() {
        assert!(matches!(
            ParameterInitialization::new(""),
            Err(ParameterError::EmptyText { .. })
        ));
    }

    #[test]
    fn rejects_malformed_qualified_policy() {
        assert!(matches!(
            ParameterInitialization::new("ai..initialization"),
            Err(ParameterError::InvalidQualifiedIdentifier { .. })
        ));
    }

    #[test]
    fn accepts_extension_defined_initialization_policy() {
        let initialization =
            ParameterInitialization::new(
                "vendor.extension.initialization.custom",
            )
            .expect("qualified extension policy should be accepted");

        assert_eq!(
            initialization.policy,
            "vendor.extension.initialization.custom"
        );
    }

    #[test]
    fn metadata_requires_a_nonempty_key_and_value() {
        assert!(ParameterMetadata::new("ai.parameter.label", "").is_err());

        assert!(
            ParameterMetadata::new(
                "ai.parameter.label",
                "weight tensor",
            )
            .is_ok()
        );
    }

    #[test]
    fn qualified_identifier_rejects_empty_segments() {
        assert!(
            validate_qualified_identifier(
                "test identifier",
                "ai..parameter",
            )
            .is_err()
        );

        assert!(
            validate_qualified_identifier(
                "test identifier",
                ".ai.parameter",
            )
            .is_err()
        );

        assert!(
            validate_qualified_identifier(
                "test identifier",
                "ai.parameter.",
            )
            .is_err()
        );
    }

    #[test]
    fn qualified_identifier_accepts_future_namespaces() {
        assert!(
            validate_qualified_identifier(
                "test identifier",
                "future.domain.parameter.policy",
            )
            .is_ok()
        );
    }
}