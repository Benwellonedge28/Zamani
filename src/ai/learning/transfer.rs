
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Transfer Learning Contracts
//!
//! Path: src/ai/learning/transfer.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines target-independent transfer-learning descriptions.
//!
//! Transfer learning describes the reuse of knowledge, parameters,
//! representations, model components, features, or other transferable
//! artifacts from a source model or task when constructing or adapting
//! a target model or task.
//!
//! This module describes WHAT may be transferred and under WHICH
//! compatibility and lineage contracts.
//!
//! It does not execute numerical transfer.
//!
//! # Architectural ownership
//!
//! Source syntax:
//!     grammar/statements/learn.g4
//!
//! Transfer-learning grammar:
//!     grammar/ai/transfer-learning.g4
//!
//! Model identity:
//!     crate::ai::ids
//!
//! Parameter declarations:
//!     crate::ai::model::parameters
//!
//! Model definitions:
//!     crate::ai::model::definition
//!
//! Model versions:
//!     crate::ai::version
//!
//! Canonical semantic validation:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computation representation:
//!     crate::zuir
//!
//! # Does not own
//!
//! - AST construction.
//! - Source parsing.
//! - Semantic name resolution.
//! - Canonical type definitions.
//! - Tensor storage.
//! - Numerical parameter copying.
//! - Optimizer execution.
//! - Training execution.
//! - Automatic differentiation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Distributed scheduling.
//! - Runtime execution.
//! - Artifact serialization.
//!
//! # POCO-REAF
//!
//! Transfer descriptions are independent of physical execution targets.
//!
//! No language-level maximum is imposed on:
//!
//! - Source models.
//! - Target models.
//! - Transfer mappings.
//! - Parameters.
//! - Components.
//! - Transferable artifacts.
//! - Model depth.
//! - Tensor rank.
//! - Training steps.
//! - Workers.
//! - Devices.
//!
//! Actual feasibility depends on available resources, capabilities,
//! policies, implementation limits, and physical constraints.
//!
//! Literal infinite computation is not promised.

use core::fmt;

use std::collections::BTreeSet;
use std::error::Error;

use crate::ai::ids::{
    AiModelId,
    AiParameterId,
};

use crate::ai::model::parameters::{
    ModelParameters,
};

use crate::ai::version::AiModelSchemaVersion;

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), TransferError> {
    if value.trim().is_empty() {
        return Err(TransferError::EmptyText { field });
    }

    if value.chars().any(char::is_control) {
        return Err(TransferError::ControlCharacter { field });
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), TransferError> {
    validate_text(field, value)?;

    if value
        .split('.')
        .any(|segment| {
            segment.is_empty()
                || !segment.chars().enumerate().all(
                    |(index, character)| {
                        character == '_'
                            || character.is_alphanumeric()
                                && (index > 0
                                    || !character.is_numeric())
                    },
                )
        })
    {
        return Err(
            TransferError::InvalidIdentifier {
                field,
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// MODEL REFERENCES
// ============================================================================

/// A versioned model reference used in a transfer operation.
///
/// This identifies a semantic model declaration.
///
/// It does not identify a physical device, memory allocation,
/// loaded model instance, or runtime execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransferModelRef {
    /// Canonical AI model identity.
    pub model_id: AiModelId,

    /// Model schema version.
    pub schema_version: AiModelSchemaVersion,
}

impl TransferModelRef {
    /// Creates a versioned model reference.
    #[must_use]
    pub const fn new(
        model_id: AiModelId,
        schema_version: AiModelSchemaVersion,
    ) -> Self {
        Self {
            model_id,
            schema_version,
        }
    }
}

// ============================================================================
// TRANSFER STRATEGIES
// ============================================================================

/// An open-world transfer strategy identifier.
///
/// Strategy identifiers are semantic names, not executable algorithms.
///
/// Examples include:
///
/// - `ai.transfer.full`
/// - `ai.transfer.fine_tuning`
/// - `ai.transfer.feature_extraction`
/// - `ai.transfer.linear_probing`
/// - `ai.transfer.parameter_efficient`
/// - `ai.transfer.adapter`
/// - `ai.transfer.knowledge_distillation`
/// - `ai.transfer.domain_adaptation`
/// - `ai.transfer.representation`
/// - `vendor.extension.transfer.custom`
///
/// The registry and implementation infrastructure determine whether
/// a particular strategy is available.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransferStrategy(String);

impl TransferStrategy {
    /// Creates a validated strategy identity.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, TransferError> {
        let identifier = identifier.into();

        validate_identifier(
            "transfer strategy",
            &identifier,
        )?;

        Ok(Self(identifier))
    }

    /// Returns the strategy identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransferStrategy {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// TRANSFERABLE ARTIFACTS
// ============================================================================

/// Classification of transferable knowledge or model information.
///
/// This is an open-world semantic classification.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransferArtifactKind(String);

impl TransferArtifactKind {
    /// Creates a validated artifact classification.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, TransferError> {
        let identifier = identifier.into();

        validate_identifier(
            "transfer artifact kind",
            &identifier,
        )?;

        Ok(Self(identifier))
    }

    /// Returns the classification.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An opaque reference to a transferable artifact.
///
/// The artifact may represent:
///
/// - A learned representation.
/// - A feature extractor.
/// - A parameter collection.
/// - A knowledge representation.
/// - A learned policy.
/// - A model component.
/// - A probabilistic representation.
/// - A quantum-derived representation.
/// - Another extension-defined artifact.
///
/// The actual artifact bytes and storage are externally owned.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransferArtifactRef {
    /// Artifact classification.
    pub kind: TransferArtifactKind,

    /// Opaque artifact identity.
    pub identity: String,

    /// Source model identity.
    pub source_model: AiModelId,

    /// Optional source artifact version or revision.
    pub revision: Option<String>,
}

impl TransferArtifactRef {
    /// Creates a transferable artifact reference.
    pub fn new(
        kind: TransferArtifactKind,
        identity: impl Into<String>,
        source_model: AiModelId,
    ) -> Result<Self, TransferError> {
        let identity = identity.into();

        validate_text(
            "transfer artifact identity",
            &identity,
        )?;

        Ok(Self {
            kind,
            identity,
            source_model,
            revision: None,
        })
    }

    /// Attaches an artifact revision.
    pub fn with_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Result<Self, TransferError> {
        let revision = revision.into();

        validate_text(
            "artifact revision",
            &revision,
        )?;

        self.revision = Some(revision);

        Ok(self)
    }
}

// ============================================================================
// PARAMETER TRANSFER
// ============================================================================

/// Describes the intended treatment of a target parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TransferDisposition {
    /// Preserve the target's existing trainability declaration.
    PreserveTarget,

    /// Describe the target parameter as frozen for subsequent learning.
    Freeze,

    /// Permit subsequent learning to update the target parameter.
    FineTune,
}

/// Describes how a source parameter relates to a target parameter.
///
/// A transformation is an opaque semantic reference. It is not executed
/// by this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransferParameterMapping {
    /// Source parameter identity.
    pub source: AiParameterId,

    /// Target parameter identity.
    pub target: AiParameterId,

    /// Optional registered transformation identity.
    ///
    /// Required when the source and target semantic types differ.
    pub transformation: Option<String>,

    /// Intended target parameter disposition.
    pub disposition: TransferDisposition,
}

impl TransferParameterMapping {
    /// Creates an identity-preserving parameter mapping.
    #[must_use]
    pub const fn new(
        source: AiParameterId,
        target: AiParameterId,
    ) -> Self {
        Self {
            source,
            target,
            transformation: None,
            disposition: TransferDisposition::PreserveTarget,
        }
    }

    /// Declares a transformation for the parameter mapping.
    pub fn with_transformation(
        mut self,
        identifier: impl Into<String>,
    ) -> Result<Self, TransferError> {
        let identifier = identifier.into();

        validate_identifier(
            "parameter transformation",
            &identifier,
        )?;

        self.transformation = Some(identifier);

        Ok(self)
    }

    /// Sets the intended target parameter disposition.
    #[must_use]
    pub fn with_disposition(
        mut self,
        disposition: TransferDisposition,
    ) -> Self {
        self.disposition = disposition;
        self
    }
}

// ============================================================================
// UNMAPPED PARAMETER POLICIES
// ============================================================================

/// Policy for target parameters without an explicit source mapping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnmappedTargetPolicy {
    /// Reject a transfer with unmapped target parameters.
    Reject,

    /// Retain the target's existing initialization/value policy.
    PreserveTarget,
}

/// Policy for source parameters not referenced by a mapping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnmappedSourcePolicy {
    /// Unreferenced source parameters are permitted.
    Ignore,

    /// Reject a transfer with unused source parameters.
    Reject,
}

// ============================================================================
// TRANSFER LINEAGE
// ============================================================================

/// Provenance references associated with a transfer operation.
///
/// These are references into canonical provenance or artifact systems.
/// This structure does not create another provenance database.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TransferLineage {
    /// Optional source model provenance reference.
    pub source_provenance: Option<String>,

    /// Optional target model provenance reference.
    pub target_provenance: Option<String>,

    /// Optional dataset lineage reference.
    pub dataset_provenance: Option<String>,

    /// Optional source artifact provenance reference.
    pub artifact_provenance: Option<String>,

    /// Optional parent transfer operation reference.
    pub parent_transfer: Option<String>,

    /// Additional externally defined lineage references.
    pub additional_references: Vec<String>,
}

impl TransferLineage {
    /// Validates all populated lineage references.
    pub fn validate(&self) -> Result<(), TransferError> {
        let references = [
            self.source_provenance.as_deref(),
            self.target_provenance.as_deref(),
            self.dataset_provenance.as_deref(),
            self.artifact_provenance.as_deref(),
            self.parent_transfer.as_deref(),
        ];

        for reference in references.into_iter().flatten() {
            validate_text(
                "provenance reference",
                reference,
            )?;
        }

        for reference in &self.additional_references {
            validate_text(
                "additional provenance reference",
                reference,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// TRANSFER CONTRACT
// ============================================================================

/// Complete declarative transfer-learning contract.
///
/// This describes the relationship between source and target models.
///
/// It does not allocate, copy, or transform numerical parameter values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferPlan {
    /// Source model.
    pub source: TransferModelRef,

    /// Target model.
    pub target: TransferModelRef,

    /// Transfer strategy.
    pub strategy: TransferStrategy,

    /// Explicit parameter mappings.
    pub parameter_mappings: Vec<TransferParameterMapping>,

    /// Additional transferable artifacts.
    pub artifacts: Vec<TransferArtifactRef>,

    /// Policy for unmapped target parameters.
    pub unmapped_target_policy: UnmappedTargetPolicy,

    /// Policy for unused source parameters.
    pub unmapped_source_policy: UnmappedSourcePolicy,

    /// Provenance and lineage.
    pub lineage: TransferLineage,
}

impl TransferPlan {
    /// Creates a transfer plan.
    pub fn new(
        source: TransferModelRef,
        target: TransferModelRef,
        strategy: TransferStrategy,
    ) -> Self {
        Self {
            source,
            target,
            strategy,
            parameter_mappings: Vec::new(),
            artifacts: Vec::new(),
            unmapped_target_policy:
                UnmappedTargetPolicy::Reject,
            unmapped_source_policy:
                UnmappedSourcePolicy::Ignore,
            lineage: TransferLineage::default(),
        }
    }

    /// Adds a parameter mapping.
    ///
    /// Duplicate target mappings are rejected during validation.
    #[must_use]
    pub fn with_parameter_mapping(
        mut self,
        mapping: TransferParameterMapping,
    ) -> Self {
        self.parameter_mappings.push(mapping);
        self
    }

    /// Adds a transferable artifact.
    #[must_use]
    pub fn with_artifact(
        mut self,
        artifact: TransferArtifactRef,
    ) -> Self {
        self.artifacts.push(artifact);
        self
    }

    /// Sets the unmapped target parameter policy.
    #[must_use]
    pub fn with_unmapped_target_policy(
        mut self,
        policy: UnmappedTargetPolicy,
    ) -> Self {
        self.unmapped_target_policy = policy;
        self
    }

    /// Sets the unmapped source parameter policy.
    #[must_use]
    pub fn with_unmapped_source_policy(
        mut self,
        policy: UnmappedSourcePolicy,
    ) -> Self {
        self.unmapped_source_policy = policy;
        self
    }

    /// Attaches transfer lineage.
    #[must_use]
    pub fn with_lineage(
        mut self,
        lineage: TransferLineage,
    ) -> Self {
        self.lineage = lineage;
        self
    }

    /// Validates the transfer plan against canonical source and target
    /// parameter declarations.
    ///
    /// This checks local structural and parameter compatibility.
    ///
    /// It does not establish that:
    ///
    /// - Model identities resolve to registered models.
    /// - The strategy implementation exists.
    /// - Transformations are registered.
    /// - Capabilities are available.
    /// - Resources are sufficient.
    /// - Authorization has been granted.
    /// - Numerical transfer has executed.
    pub fn validate(
        &self,
        source_parameters: &ModelParameters,
        target_parameters: &ModelParameters,
    ) -> Result<TransferValidationReport, TransferError> {
        self.lineage.validate()?;

        if self.source.model_id == self.target.model_id
            && self.source.schema_version
                != self.target.schema_version
        {
            // Cross-version self-adaptation is permitted.
            // Compatibility must be established by the canonical
            // model compatibility authority.
        }

        let mut mapped_targets = BTreeSet::new();
        let mut mapped_sources = BTreeSet::new();

        for mapping in &self.parameter_mappings {
            if !mapped_targets.insert(mapping.target) {
                return Err(
                    TransferError::DuplicateTargetMapping(
                        mapping.target,
                    ),
                );
            }

            let source = source_parameters
                .as_slice()
                .iter()
                .find(|parameter| {
                    parameter.id == mapping.source
                })
                .ok_or(
                    TransferError::UnknownSourceParameter(
                        mapping.source,
                    ),
                )?;

            let target = target_parameters
                .as_slice()
                .iter()
                .find(|parameter| {
                    parameter.id == mapping.target
                })
                .ok_or(
                    TransferError::UnknownTargetParameter(
                        mapping.target,
                    ),
                )?;

            if source.ty != target.ty
                && mapping.transformation.is_none()
            {
                return Err(
                    TransferError::IncompatibleParameterTypes {
                        source: mapping.source,
                        target: mapping.target,
                    },
                );
            }

            if mapping.disposition
                == TransferDisposition::FineTune
                && !target.trainable
            {
                return Err(
                    TransferError::TargetParameterNotTrainable(
                        mapping.target,
                    ),
                );
            }

            mapped_sources.insert(mapping.source);
        }

        for artifact in &self.artifacts {
            if artifact.source_model != self.source.model_id {
                return Err(
                    TransferError::ArtifactSourceMismatch {
                        expected: self.source.model_id,
                        actual: artifact.source_model,
                    },
                );
            }
        }

        let unmapped_source: Vec<_> = source_parameters
            .as_slice()
            .iter()
            .filter(|parameter| {
                !mapped_sources.contains(&parameter.id)
            })
            .map(|parameter| parameter.id)
            .collect();

        let unmapped_target: Vec<_> = target_parameters
            .as_slice()
            .iter()
            .filter(|parameter| {
                !mapped_targets.contains(&parameter.id)
            })
            .map(|parameter| parameter.id)
            .collect();

        if self.unmapped_source_policy
            == UnmappedSourcePolicy::Reject
            && !unmapped_source.is_empty()
        {
            return Err(
                TransferError::UnmappedSourceParameters {
                    count: unmapped_source.len(),
                },
            );
        }

        if self.unmapped_target_policy
            == UnmappedTargetPolicy::Reject
            && !unmapped_target.is_empty()
        {
            return Err(
                TransferError::UnmappedTargetParameters {
                    count: unmapped_target.len(),
                },
            );
        }

        Ok(TransferValidationReport {
            mapping_count: self.parameter_mappings.len(),
            artifact_count: self.artifacts.len(),
            unmapped_source,
            unmapped_target,
        })
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Successful local transfer-plan validation report.
///
/// This report does not imply execution or target feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferValidationReport {
    /// Number of validated parameter mappings.
    pub mapping_count: usize,

    /// Number of referenced transferable artifacts.
    pub artifact_count: usize,

    /// Unmapped source parameter identities.
    pub unmapped_source: Vec<AiParameterId>,

    /// Unmapped target parameter identities.
    pub unmapped_target: Vec<AiParameterId>,
}

impl TransferValidationReport {
    /// Returns whether every target parameter has an explicit mapping.
    #[must_use]
    pub fn has_complete_target_mapping(&self) -> bool {
        self.unmapped_target.is_empty()
    }

    /// Returns whether every source parameter is referenced.
    #[must_use]
    pub fn has_complete_source_mapping(&self) -> bool {
        self.unmapped_source.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Transfer-learning contract validation errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransferError {
    /// A required text field is empty.
    EmptyText {
        /// Field name.
        field: &'static str,
    },

    /// A text field contains control characters.
    ControlCharacter {
        /// Field name.
        field: &'static str,
    },

    /// An identifier is malformed.
    InvalidIdentifier {
        /// Field name.
        field: &'static str,

        /// Invalid identifier.
        value: String,
    },

    /// A source parameter does not exist.
    UnknownSourceParameter(AiParameterId),

    /// A target parameter does not exist.
    UnknownTargetParameter(AiParameterId),

    /// Multiple source parameters map to the same target parameter.
    DuplicateTargetMapping(AiParameterId),

    /// Parameter types differ without an explicit transformation.
    IncompatibleParameterTypes {
        /// Source parameter.
        source: AiParameterId,

        /// Target parameter.
        target: AiParameterId,
    },

    /// Fine-tuning was requested for a non-trainable target.
    TargetParameterNotTrainable(AiParameterId),

    /// An artifact references a different source model.
    ArtifactSourceMismatch {
        /// Expected model.
        expected: AiModelId,

        /// Actual model.
        actual: AiModelId,
    },

    /// Some source parameters were not mapped.
    UnmappedSourceParameters {
        /// Number of unmapped parameters.
        count: usize,
    },

    /// Some target parameters were not mapped.
    UnmappedTargetParameters {
        /// Number of unmapped parameters.
        count: usize,
    },
}

impl fmt::Display for TransferError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} contains control characters"
                )
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "invalid {field} identifier: {value}"
                )
            }

            Self::UnknownSourceParameter(id) => {
                write!(
                    formatter,
                    "unknown source parameter: {id}"
                )
            }

            Self::UnknownTargetParameter(id) => {
                write!(
                    formatter,
                    "unknown target parameter: {id}"
                )
            }

            Self::DuplicateTargetMapping(id) => {
                write!(
                    formatter,
                    "duplicate target parameter mapping: {id}"
                )
            }

            Self::IncompatibleParameterTypes {
                source,
                target,
            } => {
                write!(
                    formatter,
                    "incompatible parameter types: {source} -> {target}"
                )
            }

            Self::TargetParameterNotTrainable(id) => {
                write!(
                    formatter,
                    "target parameter is not trainable: {id}"
                )
            }

            Self::ArtifactSourceMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "artifact source mismatch: expected {expected}, got {actual}"
                )
            }

            Self::UnmappedSourceParameters { count } => {
                write!(
                    formatter,
                    "{count} source parameters are unmapped"
                )
            }

            Self::UnmappedTargetParameters { count } => {
                write!(
                    formatter,
                    "{count} target parameters are unmapped"
                )
            }
        }
    }
}

impl Error for TransferError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_identifiers_are_open_world() {
        let strategy = TransferStrategy::new(
            "vendor.extension.domain_adaptation",
        )
        .expect("valid strategy");

        assert_eq!(
            strategy.as_str(),
            "vendor.extension.domain_adaptation"
        );
    }

    #[test]
    fn empty_strategy_is_rejected() {
        assert!(matches!(
            TransferStrategy::new(""),
            Err(TransferError::EmptyText { .. })
        ));
    }

    #[test]
    fn malformed_strategy_is_rejected() {
        assert!(matches!(
            TransferStrategy::new("ai..transfer"),
            Err(TransferError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn artifact_revision_is_preserved() {
        let model = AiModelId::new(1)
            .expect("valid model ID");

        let artifact = TransferArtifactRef::new(
            TransferArtifactKind::new(
                "ai.representation",
            )
            .expect("valid artifact kind"),
            "representation.encoder",
            model,
        )
        .expect("valid artifact")
        .with_revision("revision-2")
        .expect("valid revision");

        assert_eq!(
            artifact.revision.as_deref(),
            Some("revision-2")
        );
    }

    #[test]
    fn lineage_accepts_multiple_references() {
        let lineage = TransferLineage {
            source_provenance: Some("source-lineage".into()),
            target_provenance: Some("target-lineage".into()),
            dataset_provenance: Some("dataset-lineage".into()),
            artifact_provenance: None,
            parent_transfer: None,
            additional_references: vec![
                "external-lineage".into(),
            ],
        };

        assert!(lineage.validate().is_ok());
    }

    #[test]
    fn lineage_rejects_empty_references() {
        let lineage = TransferLineage {
            source_provenance: Some(String::new()),
            ..TransferLineage::default()
        };

        assert!(lineage.validate().is_err());
    }

    #[test]
    fn parameter_mapping_defaults_to_preserving_target() {
        let source = AiParameterId::new(1)
            .expect("valid source parameter");

        let target = AiParameterId::new(2)
            .expect("valid target parameter");

        let mapping = TransferParameterMapping::new(
            source,
            target,
        );

        assert_eq!(
            mapping.disposition,
            TransferDisposition::PreserveTarget
        );

        assert!(mapping.transformation.is_none());
    }

    #[test]
    fn mapping_transformation_is_extensible() {
        let source = AiParameterId::new(1)
            .expect("valid source parameter");

        let target = AiParameterId::new(2)
            .expect("valid target parameter");

        let mapping = TransferParameterMapping::new(
            source,
            target,
        )
        .with_transformation(
            "vendor.extension.parameter_projection",
        )
        .expect("valid transformation");

        assert_eq!(
            mapping.transformation.as_deref(),
            Some(
                "vendor.extension.parameter_projection"
            )
        );
    }

    #[test]
    fn validation_report_distinguishes_complete_mappings() {
        let report = TransferValidationReport {
            mapping_count: 2,
            artifact_count: 1,
            unmapped_source: Vec::new(),
            unmapped_target: Vec::new(),
        };

        assert!(report.has_complete_source_mapping());
        assert!(report.has_complete_target_mapping());
    }

    #[test]
    fn partial_transfer_is_reported_explicitly() {
        let report = TransferValidationReport {
            mapping_count: 1,
            artifact_count: 0,
            unmapped_source: vec![
                AiParameterId::new(3).unwrap(),
            ],
            unmapped_target: vec![
                AiParameterId::new(4).unwrap(),
            ],
        };

        assert!(!report.has_complete_source_mapping());
        assert!(!report.has_complete_target_mapping());
    }
}
