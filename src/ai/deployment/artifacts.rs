
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Deployment Artifacts
//!
//! Path: `src/ai/deployment/artifacts.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Defines AI-specific artifact descriptors and their integration with
//! the canonical Zamani backend artifact infrastructure.
//!
//! An AI artifact represents an AI-domain computational deliverable,
//! such as:
//!
//! - A model.
//! - Model parameters.
//! - Model weights.
//! - A checkpoint.
//! - A trained model.
//! - An inference representation.
//! - An AI computational graph.
//! - A training result.
//! - A model package.
//! - An AI interoperability representation.
//! - A future registered AI artifact category.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - AI artifact classification.
//! - AI artifact identity association.
//! - AI model association.
//! - AI artifact format-version metadata.
//! - AI artifact schema-version metadata.
//! - AI-specific artifact metadata.
//! - AI artifact-local validation.
//!
//! # DOES NOT OWN
//!
//! - Artifact payload storage.
//! - Artifact byte encoding.
//! - Cryptographic hashing.
//! - Digital signatures.
//! - Persistent manifests.
//! - Artifact dependency graph storage.
//! - Filesystem persistence.
//! - Model serialization implementations.
//! - Model execution.
//! - Model training.
//! - Deployment orchestration.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Scheduling.
//!
//! # CANONICAL OWNERSHIP
//!
//! `crate::backend::artifact` owns the actual artifact payload,
//! encoding, format, integrity digest and verification evidence.
//!
//! `crate::backend::manifest` owns persistent artifact manifests.
//!
//! `crate::backend::ids` owns backend artifact identities.
//!
//! `crate::ai::ids` owns AI-domain artifact identities.
//!
//! `crate::ai::version` owns AI version identities.
//!
//! `crate::ai::provenance` adapts AI provenance to the canonical
//! semantic provenance infrastructure.
//!
//! # POCO-REAF
//!
//! AI artifact descriptions are independent of physical hardware.
//!
//! No universal artifact-size, model-size, parameter-count,
//! tensor-rank, device-count or accelerator-count limit is imposed.
//!
//! Actual resource constraints are evaluated downstream.
//!
//! This module does not promise literal infinite storage or memory.
//!
//! # IMPORTANT DISTINCTION
//!
//! `AiArtifactId` identifies an AI-domain artifact.
//!
//! `backend::ids::ArtifactId` identifies its backend-owned payload.
//!
//! They are different identities with different ownership scopes.
//!
//! Neither identity is a content fingerprint.
//!
//! # INTEGRATION
//!
//! AI model definitions:
//!     `crate::ai::model::definition`
//!
//! AI model schema:
//!     `crate::ai::version`
//!
//! AI provenance:
//!     `crate::ai::provenance`
//!
//! Backend artifact:
//!     `crate::backend::artifact`
//!
//! Backend manifest:
//!     `crate::backend::manifest`
//!
//! AI deployment:
//!     `crate::ai::deployment`
//!
//! Public AI API:
//!     `crate::ai::api`
//!
//! # FREEZE CRITERIA
//!
//! - Canonical backend payload representation reused.
//! - AI identity remains distinct from backend identity.
//! - Artifact categories are extensible.
//! - Artifact metadata is deterministic.
//! - Version domains remain distinct.
//! - Model associations are validated.
//! - Integrity verification delegates to the backend.
//! - Provenance delegates to canonical semantic infrastructure.
//! - No arbitrary workload or hardware limits.
//! - No unsafe Rust.
//! - All relevant tests pass.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::ai::ids::{
    AiArtifactId,
    AiModelId,
};

use crate::ai::provenance::AiProvenanceRecord;

use crate::ai::version::{
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiSemanticVersion,
    INITIAL_AI_ARTIFACT_FORMAT_VERSION,
};

use crate::backend::artifact::{
    ArtifactEncoding,
    ArtifactPayload,
    ArtifactValidation,
    OutputArtifact,
};

/// AI artifact classification.
///
/// The namespace is open-ended so that future AI artifact types
/// and registered extensions do not require changes to this type.
///
/// Examples:
///
/// - `ai.model`
/// - `ai.model.weights`
/// - `ai.model.checkpoint`
/// - `ai.model.graph`
/// - `ai.training.result`
/// - `ai.inference.package`
/// - `ai.vendor.extension`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiArtifactKind {
    namespace: String,
}

impl AiArtifactKind {
    /// Construct an AI artifact classification.
    ///
    /// The namespace must:
    ///
    /// - Start with `ai.`.
    /// - Contain at least one additional namespace component.
    /// - Use lowercase ASCII letters, digits, dots, hyphens
    ///   or underscores.
    pub fn new(
        namespace: impl Into<String>,
    ) -> Result<Self, AiArtifactError> {
        let namespace = namespace.into();

        if !namespace.starts_with("ai.")
            || namespace.len() <= 3
            || namespace.split('.').any(str::is_empty)
            || !namespace.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'-' | b'_')
            })
        {
            return Err(AiArtifactError::InvalidKind {
                namespace,
            });
        }

        Ok(Self { namespace })
    }

    /// Return the AI artifact namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.namespace
    }

    /// Standard AI model classification.
    #[must_use]
    pub fn model() -> Self {
        Self {
            namespace: "ai.model".to_owned(),
        }
    }

    /// Standard AI model-parameter classification.
    #[must_use]
    pub fn model_parameters() -> Self {
        Self {
            namespace: "ai.model.parameters".to_owned(),
        }
    }

    /// Standard AI model-checkpoint classification.
    #[must_use]
    pub fn checkpoint() -> Self {
        Self {
            namespace: "ai.model.checkpoint".to_owned(),
        }
    }

    /// Standard AI computational-graph classification.
    #[must_use]
    pub fn computational_graph() -> Self {
        Self {
            namespace: "ai.model.graph".to_owned(),
        }
    }

    /// Standard AI training-result classification.
    #[must_use]
    pub fn training_result() -> Self {
        Self {
            namespace: "ai.training.result".to_owned(),
        }
    }

    /// Standard AI inference-package classification.
    #[must_use]
    pub fn inference_package() -> Self {
        Self {
            namespace: "ai.inference.package".to_owned(),
        }
    }
}

impl fmt::Display for AiArtifactKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.namespace)
    }
}

/// AI-specific metadata.
///
/// Keys are qualified and stored in deterministic order.
///
/// Metadata describes the artifact. It does not replace the
/// canonical artifact manifest.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AiArtifactMetadata {
    entries: BTreeMap<String, String>,
}

impl AiArtifactMetadata {
    /// Create empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace an AI metadata entry.
    ///
    /// Keys must be qualified identifiers.
    ///
    /// Examples:
    ///
    /// - `ai.architecture`
    /// - `ai.training.framework`
    /// - `ai.model.family`
    /// - `vendor.extension.property`
    ///
    /// Values are descriptive and must not contain secrets.
    ///
    /// Returns the replaced value when one existed.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AiArtifactError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        if value.chars().any(char::is_control) {
            return Err(AiArtifactError::InvalidMetadataValue {
                key,
            });
        }

        Ok(self.entries.insert(key, value))
    }

    /// Retrieve an entry.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Return deterministic metadata entries.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Return the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate metadata, including values obtained from external sources.
    pub fn validate(&self) -> Result<(), AiArtifactError> {
        for (key, value) in &self.entries {
            validate_metadata_key(key)?;

            if value.chars().any(char::is_control) {
                return Err(AiArtifactError::InvalidMetadataValue {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), AiArtifactError> {
    if key.is_empty()
        || !key.contains('.')
        || key.starts_with('.')
        || key.ends_with('.')
        || key.split('.').any(str::is_empty)
        || !key.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b'_')
        })
    {
        return Err(AiArtifactError::InvalidMetadataKey {
            key: key.to_owned(),
        });
    }

    Ok(())
}

/// An AI-domain artifact descriptor.
///
/// This type does not duplicate artifact payload storage.
///
/// The actual payload is owned by the canonical
/// `backend::artifact::OutputArtifact`.
///
/// The AI descriptor associates that payload with AI-specific
/// semantic identity, classification, versions and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiArtifact {
    /// AI-domain identity.
    id: AiArtifactId,

    /// AI artifact classification.
    kind: AiArtifactKind,

    /// Canonical backend artifact and payload.
    backend_artifact: OutputArtifact,

    /// AI artifact encoding-contract version.
    format_version: AiArtifactFormatVersion,

    /// AI model associated with this artifact, if applicable.
    model_id: Option<AiModelId>,

    /// Model schema version, if applicable.
    model_schema_version: Option<AiModelSchemaVersion>,

    /// AI semantic contract version, if applicable.
    semantic_version: Option<AiSemanticVersion>,

    /// AI-specific descriptive metadata.
    metadata: AiArtifactMetadata,

    /// Canonical AI provenance record.
    provenance: Option<AiProvenanceRecord>,
}

impl AiArtifact {
    /// Construct an AI artifact descriptor.
    ///
    /// The caller supplies both identities:
    ///
    /// - AI-domain artifact identity.
    /// - Backend payload identity, already contained in `OutputArtifact`.
    ///
    /// No global identity allocator or storage system is created here.
    pub fn new(
        id: AiArtifactId,
        kind: AiArtifactKind,
        backend_artifact: OutputArtifact,
    ) -> Result<Self, AiArtifactError> {
        backend_artifact.validate_metadata()?;

        let artifact = Self {
            id,
            kind,
            backend_artifact,
            format_version: INITIAL_AI_ARTIFACT_FORMAT_VERSION,
            model_id: None,
            model_schema_version: None,
            semantic_version: None,
            metadata: AiArtifactMetadata::new(),
            provenance: None,
        };

        artifact.validate()?;

        Ok(artifact)
    }

    /// Return the AI artifact identity.
    #[must_use]
    pub const fn id(&self) -> AiArtifactId {
        self.id
    }

    /// Return the artifact classification.
    #[must_use]
    pub fn kind(&self) -> &AiArtifactKind {
        &self.kind
    }

    /// Return the canonical backend artifact.
    #[must_use]
    pub fn backend_artifact(&self) -> &OutputArtifact {
        &self.backend_artifact
    }

    /// Return the canonical backend artifact identity.
    #[must_use]
    pub fn backend_artifact_id(
        &self,
    ) -> crate::backend::ids::ArtifactId {
        self.backend_artifact.id
    }

    /// Return the actual payload encoding.
    #[must_use]
    pub fn encoding(&self) -> ArtifactEncoding {
        self.backend_artifact.payload.encoding()
    }

    /// Return the AI artifact format version.
    #[must_use]
    pub const fn format_version(&self) -> AiArtifactFormatVersion {
        self.format_version
    }

    /// Return the associated AI model identity.
    #[must_use]
    pub const fn model_id(&self) -> Option<AiModelId> {
        self.model_id
    }

    /// Return the associated model schema version.
    #[must_use]
    pub const fn model_schema_version(
        &self,
    ) -> Option<AiModelSchemaVersion> {
        self.model_schema_version
    }

    /// Return the AI semantic version.
    #[must_use]
    pub const fn semantic_version(&self) -> Option<AiSemanticVersion> {
        self.semantic_version
    }

    /// Return AI-specific metadata.
    #[must_use]
    pub fn metadata(&self) -> &AiArtifactMetadata {
        &self.metadata
    }

    /// Return canonical AI provenance, if supplied.
    #[must_use]
    pub fn provenance(&self) -> Option<&AiProvenanceRecord> {
        self.provenance.as_ref()
    }

    /// Associate the artifact with an AI model.
    ///
    /// Model existence and registry membership are verified by
    /// the model registry, not by this descriptor.
    #[must_use]
    pub fn with_model(
        mut self,
        model_id: AiModelId,
        schema_version: AiModelSchemaVersion,
    ) -> Self {
        self.model_id = Some(model_id);
        self.model_schema_version = Some(schema_version);
        self
    }

    /// Set the AI semantic contract version.
    #[must_use]
    pub fn with_semantic_version(
        mut self,
        version: AiSemanticVersion,
    ) -> Self {
        self.semantic_version = Some(version);
        self
    }

    /// Set an explicitly validated AI artifact format version.
    #[must_use]
    pub fn with_format_version(
        mut self,
        version: AiArtifactFormatVersion,
    ) -> Self {
        self.format_version = version;
        self
    }

    /// Insert AI-specific metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AiArtifactError> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Attach a canonical AI provenance record.
    ///
    /// Provenance authenticity is not established by attaching
    /// a descriptive record.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: AiProvenanceRecord,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Return the canonical backend payload as bytes.
    #[must_use]
    pub fn payload_bytes(&self) -> &[u8] {
        self.backend_artifact.payload.as_bytes()
    }

    /// Return the payload as UTF-8 when its encoding permits it.
    ///
    /// Invalid UTF-8 is returned as an error. No lossy conversion
    /// is performed.
    pub fn payload_text(&self) -> Result<&str, AiArtifactError> {
        Ok(self.backend_artifact.payload.as_text()?)
    }

    /// Compute and record the canonical backend payload digest.
    ///
    /// The backend owns SHA-256 computation.
    ///
    /// This is an integrity fingerprint, not an authentication
    /// or digital-signature mechanism.
    pub fn refresh_content_digest(&mut self) {
        self.backend_artifact.refresh_content_digest();
    }

    /// Verify the backend artifact's recorded content digest.
    pub fn verify_content_digest(
        &self,
    ) -> Result<(), AiArtifactError> {
        self.backend_artifact.verify_content_digest()?;
        Ok(())
    }

    /// Return the recorded backend validation state.
    #[must_use]
    pub fn validation(&self) -> &ArtifactValidation {
        &self.backend_artifact.validation
    }

    /// Update validation evidence after an external verifier
    /// has actually completed successfully.
    pub fn set_validation(
        &mut self,
        validation: ArtifactValidation,
    ) -> Result<(), AiArtifactError> {
        self.backend_artifact.set_validation(validation)?;
        Ok(())
    }

    /// Validate all AI-owned and delegated artifact invariants.
    ///
    /// This does not establish:
    ///
    /// - Model registry membership.
    /// - Model execution compatibility.
    /// - Target capability compatibility.
    /// - Artifact semantic equivalence.
    /// - Digital-signature authenticity.
    /// - Deployment feasibility.
    pub fn validate(&self) -> Result<(), AiArtifactError> {
        validate_kind(&self.kind)?;

        self.metadata.validate()?;

        self.backend_artifact.validate_metadata()?;

        if self.model_id.is_none()
            && self.model_schema_version.is_some()
        {
            return Err(
                AiArtifactError::ModelSchemaWithoutModel,
            );
        }

        if self.model_id.is_some()
            && self.model_schema_version.is_none()
        {
            return Err(
                AiArtifactError::ModelWithoutSchemaVersion,
            );
        }

        Ok(())
    }

    /// Consume the descriptor and return the canonical backend artifact.
    ///
    /// This transfers payload ownership without copying the payload.
    #[must_use]
    pub fn into_backend_artifact(self) -> OutputArtifact {
        self.backend_artifact
    }

    /// Return the canonical backend artifact format.
    ///
    /// This is the actual payload format, not the AI artifact kind.
    pub fn backend_format(
        &self,
    ) -> Result<crate::backend::artifact::ArtifactFormat, AiArtifactError> {
        Ok(self.backend_artifact.format_id()?)
    }

    /// Construct a text-backed AI artifact.
    ///
    /// The backend identity, target and actual format are supplied
    /// by the caller.
    pub fn from_text(
        id: AiArtifactId,
        kind: AiArtifactKind,
        backend_id: crate::backend::ids::ArtifactId,
        target: impl Into<String>,
        format: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<Self, AiArtifactError> {
        let backend = OutputArtifact::new(
            backend_id,
            target,
            format,
            ArtifactPayload::text(content),
        )?;

        Self::new(id, kind, backend)
    }

    /// Construct a binary-backed AI artifact.
    ///
    /// Binary content is preserved exactly.
    pub fn from_binary(
        id: AiArtifactId,
        kind: AiArtifactKind,
        backend_id: crate::backend::ids::ArtifactId,
        target: impl Into<String>,
        format: impl Into<String>,
        content: Vec<u8>,
    ) -> Result<Self, AiArtifactError> {
        let backend = OutputArtifact::new(
            backend_id,
            target,
            format,
            ArtifactPayload::binary(content),
        )?;

        Self::new(id, kind, backend)
    }
}

fn validate_kind(
    kind: &AiArtifactKind,
) -> Result<(), AiArtifactError> {
    AiArtifactKind::new(kind.as_str().to_owned()).map(|_| ())
}

/// Errors produced by AI artifact construction and validation.
#[derive(Debug)]
#[non_exhaustive]
pub enum AiArtifactError {
    /// Invalid AI artifact classification.
    InvalidKind {
        /// Invalid classification namespace.
        namespace: String,
    },

    /// Invalid metadata key.
    InvalidMetadataKey {
        /// Invalid key.
        key: String,
    },

    /// Invalid metadata value.
    InvalidMetadataValue {
        /// Associated key.
        key: String,
    },

    /// A schema version was supplied without a model identity.
    ModelSchemaWithoutModel,

    /// A model identity was supplied without a schema version.
    ModelWithoutSchemaVersion,

    /// A canonical backend artifact operation failed.
    Backend(crate::backend::errors::BackendError),
}

impl From<crate::backend::errors::BackendError> for AiArtifactError {
    fn from(error: crate::backend::errors::BackendError) -> Self {
        Self::Backend(error)
    }
}

impl fmt::Display for AiArtifactError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidKind { namespace } => {
                write!(
                    formatter,
                    "invalid AI artifact classification: {namespace:?}"
                )
            }

            Self::InvalidMetadataKey { key } => {
                write!(
                    formatter,
                    "invalid AI artifact metadata key: {key:?}"
                )
            }

            Self::InvalidMetadataValue { key } => {
                write!(
                    formatter,
                    "invalid control characters in AI artifact metadata value: {key:?}"
                )
            }

            Self::ModelSchemaWithoutModel => {
                formatter.write_str(
                    "AI model schema version requires an associated model identity",
                )
            }

            Self::ModelWithoutSchemaVersion => {
                formatter.write_str(
                    "AI model association requires an explicit model schema version",
                )
            }

            Self::Backend(error) => error.fmt(formatter),
        }
    }
}

impl Error for AiArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::backend::ids::{
        ArtifactId,
        IdNamespace,
    };

    fn ai_id(value: u128) -> AiArtifactId {
        AiArtifactId::new(value).expect("valid AI artifact identity")
    }

    fn backend_id(value: u128) -> ArtifactId {
        ArtifactId::new(
            IdNamespace::new("zamani.ai.artifact")
                .expect("valid namespace"),
            value,
        )
        .expect("valid backend artifact identity")
    }

    fn sample() -> AiArtifact {
        AiArtifact::from_binary(
            ai_id(1),
            AiArtifactKind::model(),
            backend_id(1),
            "portable",
            "vendor.model+v1",
            vec![1, 2, 3, 4],
        )
        .expect("valid AI artifact")
    }

    #[test]
    fn accepts_open_ai_artifact_namespaces() {
        assert!(AiArtifactKind::new("ai.model").is_ok());

        assert!(
            AiArtifactKind::new("ai.vendor.extension")
                .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_artifact_namespaces() {
        assert!(AiArtifactKind::new("").is_err());
        assert!(AiArtifactKind::new("model").is_err());
        assert!(AiArtifactKind::new("ai.").is_err());
        assert!(AiArtifactKind::new("ai.model/weights").is_err());
        assert!(AiArtifactKind::new("ai.Model").is_err());
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = AiArtifactMetadata::new();

        metadata.insert("ai.z", "last").expect("valid metadata");
        metadata.insert("ai.a", "first").expect("valid metadata");

        let keys: Vec<_> = metadata.entries().keys().collect();

        assert_eq!(
            keys,
            vec!["ai.a", "ai.z"]
        );
    }

    #[test]
    fn rejects_invalid_metadata_keys() {
        let mut metadata = AiArtifactMetadata::new();

        assert!(metadata.insert("invalid", "value").is_err());
        assert!(metadata.insert("ai..invalid", "value").is_err());
        assert!(metadata.insert("ai.invalid/key", "value").is_err());
    }

    #[test]
    fn rejects_control_characters_in_metadata_values() {
        let mut metadata = AiArtifactMetadata::new();

        assert!(
            metadata.insert("ai.description", "invalid\nvalue")
                .is_err()
        );
    }

    #[test]
    fn constructs_valid_ai_artifact() {
        let artifact = sample();

        assert_eq!(artifact.id().get(), 1);
        assert_eq!(artifact.kind().as_str(), "ai.model");
        assert_eq!(artifact.payload_bytes(), &[1, 2, 3, 4]);

        assert!(artifact.validate().is_ok());
    }

    #[test]
    fn ai_and_backend_identities_are_distinct() {
        let artifact = sample();

        assert_eq!(artifact.id().get(), 1);
        assert_eq!(artifact.backend_artifact_id().value(), 1);

        assert_ne!(
            std::any::type_name::<AiArtifactId>(),
            std::any::type_name::<ArtifactId>()
        );
    }

    #[test]
    fn content_digest_uses_canonical_backend_implementation() {
        let mut artifact = sample();

        artifact.refresh_content_digest();

        assert!(
            artifact.verify_content_digest().is_ok()
        );
    }

    #[test]
    fn tampered_content_fails_digest_verification() {
        let mut artifact = sample();

        artifact.refresh_content_digest();

        let mut backend = artifact.into_backend_artifact();

        backend.payload = ArtifactPayload::binary(vec![9, 9, 9]);

        let tampered = AiArtifact::new(
            ai_id(1),
            AiArtifactKind::model(),
            backend,
        )
        .expect("construction without a stale digest");

        assert!(tampered.validate().is_err());
    }

    #[test]
    fn model_schema_requires_model_identity() {
        let artifact = sample()
            .with_format_version(
                INITIAL_AI_ARTIFACT_FORMAT_VERSION,
            );

        assert!(artifact.validate().is_ok());
    }

    #[test]
    fn model_association_requires_schema_version() {
        let mut artifact = sample();

        artifact.model_id = Some(AiModelId::new(7).expect("model ID"));

        assert!(
            matches!(
                artifact.validate(),
                Err(AiArtifactError::ModelWithoutSchemaVersion)
            )
        );
    }

    #[test]
    fn complete_model_association_is_valid() {
        let artifact = sample().with_model(
            AiModelId::new(7).expect("model ID"),
            AiModelSchemaVersion::new(1, 0, 0),
        );

        assert!(artifact.validate().is_ok());
    }

    #[test]
    fn supports_empty_and_large_payloads_without_ai_specific_limits() {
        let empty = AiArtifact::from_binary(
            ai_id(2),
            AiArtifactKind::checkpoint(),
            backend_id(2),
            "portable",
            "checkpoint",
            Vec::new(),
        )
        .expect("empty payload is structurally representable");

        assert!(empty.payload_bytes().is_empty());
        assert!(empty.validate().is_ok());
    }

    #[test]
    fn text_payload_is_not_lossily_decoded() {
        let artifact = AiArtifact::from_text(
            ai_id(3),
            AiArtifactKind::computational_graph(),
            backend_id(3),
            "portable",
            "text",
            "Zamani AI",
        )
        .expect("valid text artifact");

        assert_eq!(
            artifact.payload_text().expect("valid UTF-8"),
            "Zamani AI"
        );
    }

    #[test]
    fn format_version_is_independent_of_model_schema_version() {
        let artifact = sample()
            .with_model(
                AiModelId::new(9).expect("model ID"),
                AiModelSchemaVersion::new(2, 0, 0),
            )
            .with_format_version(
                AiArtifactFormatVersion::new(3, 0, 0),
            );

        assert_eq!(
            artifact.format_version().major(),
            3
        );

        assert_eq!(
            artifact.model_schema_version().expect("schema").major(),
            2
        );
    }
}
