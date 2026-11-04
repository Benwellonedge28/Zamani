
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Deployment Manifests
//!
//! Path: src/ai/deployment/manifests.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides a portable, versioned description of AI deployment artifacts.
//!
//! The AI deployment manifest associates AI-domain artifact identities,
//! model versions, semantic versions, AI-specific metadata and dependency
//! relationships with the canonical backend artifact manifest.
//!
//! ## Architectural ownership
//!
//! This module owns:
//! - AI deployment manifest structure.
//! - AI deployment schema versioning.
//! - AI artifact-to-backend artifact associations.
//! - AI-specific artifact metadata snapshots.
//! - AI deployment dependency relationships.
//! - AI deployment manifest validation.
//! - Deterministic serialization and fingerprinting.
//!
//! It does not own:
//! - Artifact payload storage.
//! - Artifact serialization implementations.
//! - Cryptographic signing.
//! - Model execution.
//! - Model training.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Scheduling.
//! - Deployment orchestration.
//! - Filesystem persistence.
//!
//! ## Canonical integration
//!
//! AI artifacts:
//!     crate::ai::deployment::artifacts
//!
//! AI identities:
//!     crate::ai::ids
//!
//! AI versions:
//!     crate::ai::version
//!
//! AI provenance:
//!     crate::ai::provenance
//!
//! Canonical backend artifact:
//!     crate::backend::artifact
//!
//! Canonical backend manifest:
//!     crate::backend::manifest
//!
//! ## POCO-REAF
//!
//! No universal limit is imposed on:
//! - Models.
//! - Artifacts.
//! - Parameters.
//! - Tensor dimensions.
//! - Deployment targets.
//! - Accelerators.
//! - Nodes.
//! - Workers.
//! - Memory.
//! - Payload size.
//!
//! Finite representation and operational limits remain explicit.
//! This module does not promise physically infinite resources.
//!
//! ## Integrity
//!
//! SHA-256 fingerprints are integrity fingerprints, not digital signatures.
//! Manifest validity does not establish the authenticity of its producer.
//!
//! ## Determinism
//!
//! Records are ordered by AI artifact identity.
//! Dependencies are ordered by AI artifact identity.
//! Metadata uses BTreeMap.
//! No timestamps or machine-local paths are inserted implicitly.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ai::deployment::artifacts::{
    AiArtifact,
    AiArtifactError,
    AiArtifactKind,
    AiArtifactMetadata,
};

use crate::ai::ids::{
    AiArtifactId,
    AiModelId,
};

use crate::ai::version::{
    AiApiVersion,
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiSemanticVersion,
    INITIAL_AI_API_VERSION,
    INITIAL_AI_ARTIFACT_FORMAT_VERSION,
    INITIAL_AI_SEMANTIC_VERSION,
};

use crate::backend::artifact::OutputArtifact;

use crate::backend::errors::BackendError;

use crate::backend::ids::ArtifactId;

use crate::backend::manifest::ArtifactManifest;

/// Canonical AI deployment manifest schema identifier.
pub const AI_DEPLOYMENT_MANIFEST_SCHEMA: &str =
    "zamani.ai.deployment.manifest";

/// Current AI deployment manifest schema version.
///
/// Increment only when an incompatible schema evolution occurs.
pub const AI_DEPLOYMENT_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// One AI artifact's portable deployment description.
///
/// This record references the canonical backend artifact manifest.
/// It never duplicates the actual payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiDeploymentArtifactRecord {
    /// AI-domain artifact identity.
    pub artifact_id: u128,

    /// Corresponding canonical backend artifact identity.
    pub backend_artifact_id: String,

    /// Extensible AI artifact classification.
    pub kind: String,

    /// AI artifact format-contract version.
    pub format_version: AiArtifactFormatVersion,

    /// Associated AI model identity, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<u128>,

    /// Associated model schema version, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_schema_version: Option<AiModelSchemaVersion>,

    /// AI semantic contract version, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<AiSemanticVersion>,

    /// Canonical semantic provenance identity, if supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_id: Option<String>,

    /// AI-specific metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,

    /// Dependencies expressed using AI artifact identities.
    ///
    /// Dependencies are sorted and unique.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<u128>,
}

impl AiDeploymentArtifactRecord {
    /// Construct a deployment record from an existing AI artifact.
    ///
    /// The canonical backend artifact remains the owner of the payload.
    pub fn from_artifact(
        artifact: &AiArtifact,
    ) -> Result<Self, AiDeploymentManifestError> {
        artifact.validate()?;

        let record = Self {
            artifact_id: artifact.id().get(),
            backend_artifact_id: artifact
                .backend_artifact_id()
                .to_string(),
            kind: artifact.kind().as_str().to_owned(),
            format_version: artifact.format_version(),
            model_id: artifact.model_id().map(AiModelId::get),
            model_schema_version: artifact.model_schema_version(),
            semantic_version: artifact.semantic_version(),
            provenance_id: artifact
                .provenance()
                .map(|provenance| provenance.id().to_string()),
            metadata: artifact.metadata().entries().clone(),
            dependencies: Vec::new(),
        };

        record.validate()?;

        Ok(record)
    }

    /// Return the strongly typed AI artifact identity.
    pub fn ai_id(
        &self,
    ) -> Result<AiArtifactId, AiDeploymentManifestError> {
        AiArtifactId::new(self.artifact_id)
            .map_err(|_| {
                AiDeploymentManifestError::InvalidArtifactIdentity(
                    self.artifact_id,
                )
            })
    }

    /// Return the strongly typed AI model identity.
    pub fn ai_model_id(
        &self,
    ) -> Result<Option<AiModelId>, AiDeploymentManifestError> {
        self.model_id
            .map(|value| {
                AiModelId::new(value).map_err(|_| {
                    AiDeploymentManifestError::InvalidModelIdentity(
                        value,
                    )
                })
            })
            .transpose()
    }

    /// Return the canonical backend artifact identity.
    ///
    /// Parsing is delegated to the canonical backend identity type.
    pub fn backend_id(
        &self,
    ) -> Result<ArtifactId, AiDeploymentManifestError> {
        ArtifactId::parse(&self.backend_artifact_id)
            .map_err(|error| {
                AiDeploymentManifestError::InvalidBackendIdentity(
                    error.to_string(),
                )
            })
    }

    /// Validate AI-specific record invariants.
    pub fn validate(
        &self,
    ) -> Result<(), AiDeploymentManifestError> {
        self.ai_id()?;
        self.backend_id()?;
        self.ai_model_id()?;

        AiArtifactKind::new(self.kind.clone())?;

        if self.model_id.is_none()
            && self.model_schema_version.is_some()
        {
            return Err(
                AiDeploymentManifestError::ModelSchemaWithoutModel,
            );
        }

        if self.model_id.is_some()
            && self.model_schema_version.is_none()
        {
            return Err(
                AiDeploymentManifestError::ModelWithoutSchemaVersion,
            );
        }

        if self.provenance_id.as_ref().is_some_and(
            |value| value.trim().is_empty()
        ) {
            return Err(
                AiDeploymentManifestError::InvalidProvenanceIdentity,
            );
        }

        let mut metadata = AiArtifactMetadata::new();

        for (key, value) in &self.metadata {
            metadata.insert(key.clone(), value.clone())?;
        }

        metadata.validate()?;

        let mut previous = None;

        for dependency in &self.dependencies {
            if *dependency == self.artifact_id {
                return Err(
                    AiDeploymentManifestError::SelfDependency(
                        self.artifact_id,
                    ),
                );
            }

            if *dependency == 0 {
                return Err(
                    AiDeploymentManifestError::InvalidDependencyIdentity(
                        *dependency,
                    ),
                );
            }

            if previous.is_some_and(|value| value >= *dependency) {
                return Err(
                    AiDeploymentManifestError::NonCanonicalDependencies,
                );
            }

            previous = Some(*dependency);
        }

        Ok(())
    }
}

/// Portable AI deployment manifest.
///
/// The backend manifest is authoritative for payload integrity,
/// backend identities, artifact formats and backend dependencies.
///
/// This manifest adds AI-specific semantics and associations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiDeploymentManifest {
    /// Stable schema identifier.
    pub schema: String,

    /// AI deployment manifest schema version.
    pub schema_version: u32,

    /// Public AI API version.
    pub api_version: AiApiVersion,

    /// AI computational semantic contract version.
    pub semantic_version: AiSemanticVersion,

    /// Default AI artifact format-contract version.
    pub artifact_format_version: AiArtifactFormatVersion,

    /// Optional reproducible build identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,

    /// Optional producing system identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,

    /// Optional producer version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,

    /// AI-specific deployment artifact records.
    ///
    /// Records are ordered by ascending numeric AI artifact identity.
    pub artifacts: Vec<AiDeploymentArtifactRecord>,

    /// Canonical backend artifact manifest.
    ///
    /// This is the authoritative payload integrity and dependency manifest.
    pub backend_manifest: ArtifactManifest,

    /// Additional portable, non-secret deployment metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl Default for AiDeploymentManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl AiDeploymentManifest {
    /// Construct an empty AI deployment manifest.
    #[must_use]
    pub fn new() -> Self {
        Self {
            schema: AI_DEPLOYMENT_MANIFEST_SCHEMA.to_owned(),
            schema_version: AI_DEPLOYMENT_MANIFEST_SCHEMA_VERSION,
            api_version: INITIAL_AI_API_VERSION,
            semantic_version: INITIAL_AI_SEMANTIC_VERSION,
            artifact_format_version:
                INITIAL_AI_ARTIFACT_FORMAT_VERSION,
            build_id: None,
            producer: None,
            producer_version: None,
            artifacts: Vec::new(),
            backend_manifest: ArtifactManifest::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Construct a manifest from AI artifacts.
    ///
    /// All backend payload snapshots are delegated to the canonical
    /// backend manifest implementation.
    ///
    /// Duplicate AI identities and backend identities are rejected.
    pub fn from_artifacts(
        artifacts: &[AiArtifact],
    ) -> Result<Self, AiDeploymentManifestError> {
        let mut manifest = Self::new();

        for artifact in artifacts {
            manifest.add_artifact(artifact)?;
        }

        manifest.validate()?;

        Ok(manifest)
    }

    /// Add an AI artifact to the manifest.
    ///
    /// The operation is atomic: the original manifest remains unchanged
    /// if validation or backend registration fails.
    pub fn add_artifact(
        &mut self,
        artifact: &AiArtifact,
    ) -> Result<(), AiDeploymentManifestError> {
        let mut candidate = self.clone();

        candidate.ensure_current_schema()?;

        artifact.validate()?;

        let record =
            AiDeploymentArtifactRecord::from_artifact(artifact)?;

        if candidate.artifacts.iter().any(
            |existing| existing.artifact_id == record.artifact_id
        ) {
            return Err(
                AiDeploymentManifestError::DuplicateAiArtifact(
                    record.artifact_id,
                ),
            );
        }

        let backend_id = artifact.backend_artifact_id();

        if candidate.artifacts.iter().any(
            |existing| {
                existing.backend_artifact_id == backend_id.to_string()
            }
        ) {
            return Err(
                AiDeploymentManifestError::DuplicateBackendArtifact(
                    backend_id.to_string(),
                ),
            );
        }

        candidate.backend_manifest.add_artifact(
            artifact.backend_artifact(),
        )?;

        candidate.artifacts.push(record);

        candidate.artifacts.sort_by_key(
            |entry| entry.artifact_id
        );

        candidate.validate()?;

        *self = candidate;

        Ok(())
    }

    /// Add an AI artifact dependency.
    ///
    /// Both AI artifacts must already exist.
    ///
    /// The corresponding backend dependency is added in the same
    /// atomic operation.
    pub fn add_dependency(
        &mut self,
        artifact_id: AiArtifactId,
        dependency_id: AiArtifactId,
    ) -> Result<(), AiDeploymentManifestError> {
        let mut candidate = self.clone();

        candidate.ensure_current_schema()?;

        if artifact_id == dependency_id {
            return Err(
                AiDeploymentManifestError::SelfDependency(
                    artifact_id.get(),
                ),
            );
        }

        let owner_index = candidate.artifacts.iter().position(
            |entry| entry.artifact_id == artifact_id.get()
        ).ok_or(
            AiDeploymentManifestError::MissingArtifact(
                artifact_id.get(),
            )
        )?;

        let dependency = candidate.artifacts.iter().find(
            |entry| entry.artifact_id == dependency_id.get()
        ).ok_or(
            AiDeploymentManifestError::MissingArtifact(
                dependency_id.get(),
            )
        )?;

        let owner_backend = candidate.artifacts[owner_index]
            .backend_id()?;

        let dependency_backend = dependency.backend_id()?;

        let owner = &mut candidate.artifacts[owner_index];

        if owner.dependencies.contains(&dependency_id.get()) {
            return Err(
                AiDeploymentManifestError::DuplicateDependency(
                    artifact_id.get(),
                    dependency_id.get(),
                ),
            );
        }

        owner.dependencies.push(dependency_id.get());

        owner.dependencies.sort_unstable();

        candidate.backend_manifest.add_dependency(
            &owner_backend,
            &dependency_backend,
        )?;

        candidate.validate()?;

        *self = candidate;

        Ok(())
    }

    /// Set a reproducible build identity.
    pub fn set_build_id(
        &mut self,
        value: impl Into<String>,
    ) -> Result<(), AiDeploymentManifestError> {
        let value = value.into();

        validate_optional_text("build ID", Some(&value))?;

        self.build_id = Some(value);

        Ok(())
    }

    /// Set the producer identity.
    pub fn set_producer(
        &mut self,
        producer: impl Into<String>,
        version: Option<String>,
    ) -> Result<(), AiDeploymentManifestError> {
        let producer = producer.into();

        validate_optional_text("producer", Some(&producer))?;

        validate_optional_text(
            "producer version",
            version.as_deref(),
        )?;

        self.producer = Some(producer);
        self.producer_version = version;

        Ok(())
    }

    /// Insert portable, non-secret manifest metadata.
    ///
    /// Values are stored in deterministic key order.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AiDeploymentManifestError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_entry(&key, &value)?;

        Ok(self.metadata.insert(key, value))
    }

    /// Retrieve deployment metadata.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Return the AI deployment artifact records.
    #[must_use]
    pub fn artifacts(&self) -> &[AiDeploymentArtifactRecord] {
        &self.artifacts
    }

    /// Return the canonical backend artifact manifest.
    #[must_use]
    pub fn backend_manifest(&self) -> &ArtifactManifest {
        &self.backend_manifest
    }

    /// Return the number of AI deployment artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Whether this deployment manifest contains no artifacts.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    /// Validate the complete AI deployment manifest.
    ///
    /// This validates structural and referential consistency.
    ///
    /// It does not establish:
    /// - Model execution feasibility.
    /// - Runtime compatibility.
    /// - Target capability availability.
    /// - Payload authenticity.
    /// - Successful model verification.
    /// - Deployment success.
    pub fn validate(
        &self,
    ) -> Result<(), AiDeploymentManifestError> {
        self.ensure_current_schema()?;

        validate_optional_text(
            "build ID",
            self.build_id.as_deref(),
        )?;

        validate_optional_text(
            "producer",
            self.producer.as_deref(),
        )?;

        validate_optional_text(
            "producer version",
            self.producer_version.as_deref(),
        )?;

        for (key, value) in &self.metadata {
            validate_metadata_entry(key, value)?;
        }

        self.backend_manifest.validate()?;

        let mut ai_ids = BTreeSet::new();
        let mut backend_ids = BTreeSet::new();

        let mut previous_id = None;

        for record in &self.artifacts {
            record.validate()?;

            if !ai_ids.insert(record.artifact_id) {
                return Err(
                    AiDeploymentManifestError::DuplicateAiArtifact(
                        record.artifact_id,
                    ),
                );
            }

            if previous_id.is_some_and(
                |previous| previous >= record.artifact_id
            ) {
                return Err(
                    AiDeploymentManifestError::NonCanonicalArtifactOrder,
                );
            }

            previous_id = Some(record.artifact_id);

            if !backend_ids.insert(
                record.backend_artifact_id.as_str()
            ) {
                return Err(
                    AiDeploymentManifestError::DuplicateBackendArtifact(
                        record.backend_artifact_id.clone(),
                    ),
                );
            }
        }

        // Every AI artifact must reference exactly one backend artifact.
        //
        // No orphan backend artifacts are accepted in an AI deployment
        // manifest constructed through this API.
        let backend_records = &self.backend_manifest.artifacts;

        if backend_records.len() != self.artifacts.len() {
            return Err(
                AiDeploymentManifestError::ArtifactAssociationMismatch,
            );
        }

        let backend_index: BTreeMap<&str, &crate::backend::manifest::ManifestArtifact> =
            backend_records.iter().map(
                |record| (record.artifact_id.as_str(), record)
            ).collect();

        let ai_index: BTreeMap<u128, &AiDeploymentArtifactRecord> =
            self.artifacts.iter().map(
                |record| (record.artifact_id, record)
            ).collect();

        for record in &self.artifacts {
            let backend = backend_index.get(
                record.backend_artifact_id.as_str()
            ).ok_or(
                AiDeploymentManifestError::MissingBackendArtifact(
                    record.backend_artifact_id.clone(),
                )
            )?;

            let mut expected_dependencies = Vec::new();

            for dependency_id in &record.dependencies {
                if !ai_index.contains_key(dependency_id) {
                    return Err(
                        AiDeploymentManifestError::MissingArtifact(
                            *dependency_id,
                        ),
                    );
                }

                let dependency = ai_index.get(dependency_id).ok_or(
                    AiDeploymentManifestError::MissingArtifact(
                        *dependency_id,
                    )
                )?;

                expected_dependencies.push(
                    dependency.backend_artifact_id.clone()
                );
            }

            expected_dependencies.sort();

            if backend.dependencies != expected_dependencies {
                return Err(
                    AiDeploymentManifestError::ArtifactAssociationMismatch,
                );
            }
        }

        Ok(())
    }

    /// Serialize the validated manifest to deterministic compact JSON.
    pub fn to_json(
        &self,
    ) -> Result<String, AiDeploymentManifestError> {
        self.validate()?;

        Ok(serde_json::to_string(self)?)
    }

    /// Serialize the validated manifest to readable JSON.
    pub fn to_pretty_json(
        &self,
    ) -> Result<String, AiDeploymentManifestError> {
        self.validate()?;

        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Parse and validate a deployment manifest from JSON.
    ///
    /// Parsing never establishes that referenced payloads are physically
    /// present or that deployment is feasible.
    pub fn from_json(
        json: &str,
    ) -> Result<Self, AiDeploymentManifestError> {
        let manifest: Self = serde_json::from_str(json)?;

        manifest.validate()?;

        Ok(manifest)
    }

    /// Compute the SHA-256 fingerprint of the validated AI manifest.
    ///
    /// This includes both AI metadata and the backend artifact manifest.
    ///
    /// It is not a digital signature.
    pub fn sha256_digest(
        &self,
    ) -> Result<String, AiDeploymentManifestError> {
        let serialized = self.to_json()?;

        let digest = Sha256::digest(serialized.as_bytes());

        Ok(format!(
            "sha256:{}",
            hex::encode(digest)
        ))
    }

    /// Validate the manifest's schema identity and version.
    fn ensure_current_schema(
        &self,
    ) -> Result<(), AiDeploymentManifestError> {
        if self.schema != AI_DEPLOYMENT_MANIFEST_SCHEMA {
            return Err(
                AiDeploymentManifestError::UnsupportedSchema(
                    self.schema.clone(),
                ),
            );
        }

        if self.schema_version !=
            AI_DEPLOYMENT_MANIFEST_SCHEMA_VERSION
        {
            return Err(
                AiDeploymentManifestError::UnsupportedSchemaVersion {
                    supplied: self.schema_version,
                    supported: AI_DEPLOYMENT_MANIFEST_SCHEMA_VERSION,
                },
            );
        }

        Ok(())
    }
}

fn validate_optional_text(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), AiDeploymentManifestError> {
    if value.is_some_and(
        |value| value.trim().is_empty()
            || value.chars().any(char::is_control)
    ) {
        return Err(
            AiDeploymentManifestError::InvalidTextField(field),
        );
    }

    Ok(())
}

fn validate_metadata_entry(
    key: &str,
    value: &str,
) -> Result<(), AiDeploymentManifestError> {
    if key.trim().is_empty()
        || key.chars().any(char::is_control)
        || value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(
            AiDeploymentManifestError::InvalidMetadata,
        );
    }

    Ok(())
}

/// AI deployment manifest failures.
#[derive(Debug)]
#[non_exhaustive]
pub enum AiDeploymentManifestError {
    /// Invalid AI artifact identity.
    InvalidArtifactIdentity(u128),

    /// Invalid AI model identity.
    InvalidModelIdentity(u128),

    /// Invalid backend artifact identity.
    InvalidBackendIdentity(String),

    /// Invalid provenance identity.
    InvalidProvenanceIdentity,

    /// Duplicate AI artifact.
    DuplicateAiArtifact(u128),

    /// Duplicate backend artifact association.
    DuplicateBackendArtifact(String),

    /// Missing AI artifact.
    MissingArtifact(u128),

    /// Missing backend artifact.
    MissingBackendArtifact(String),

    /// Invalid artifact dependency.
    SelfDependency(u128),

    /// Invalid dependency identity.
    InvalidDependencyIdentity(u128),

    /// Duplicate dependency.
    DuplicateDependency(u128, u128),

    /// Dependencies are not in canonical order.
    NonCanonicalDependencies,

    /// Artifacts are not in canonical order.
    NonCanonicalArtifactOrder,

    /// Model schema version without model identity.
    ModelSchemaWithoutModel,

    /// Model identity without model schema version.
    ModelWithoutSchemaVersion,

    /// AI/backend artifact associations disagree.
    ArtifactAssociationMismatch,

    /// Invalid metadata.
    InvalidMetadata,

    /// Invalid text field.
    InvalidTextField(&'static str),

    /// Unknown manifest schema.
    UnsupportedSchema(String),

    /// Unknown manifest schema version.
    UnsupportedSchemaVersion {
        /// Supplied schema version.
        supplied: u32,

        /// Supported schema version.
        supported: u32,
    },

    /// AI artifact validation failure.
    Artifact(AiArtifactError),

    /// Backend manifest validation failure.
    Backend(BackendError),

    /// JSON serialization or parsing failure.
    Json(serde_json::Error),
}

impl fmt::Display for AiDeploymentManifestError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidArtifactIdentity(value) =>
                write!(formatter, "invalid AI artifact identity: {value}"),

            Self::InvalidModelIdentity(value) =>
                write!(formatter, "invalid AI model identity: {value}"),

            Self::InvalidBackendIdentity(value) =>
                write!(formatter, "invalid backend artifact identity: {value}"),

            Self::InvalidProvenanceIdentity =>
                formatter.write_str("invalid provenance identity"),

            Self::DuplicateAiArtifact(value) =>
                write!(formatter, "duplicate AI artifact: {value}"),

            Self::DuplicateBackendArtifact(value) =>
                write!(formatter, "duplicate backend artifact: {value}"),

            Self::MissingArtifact(value) =>
                write!(formatter, "missing AI artifact: {value}"),

            Self::MissingBackendArtifact(value) =>
                write!(formatter, "missing backend artifact: {value}"),

            Self::SelfDependency(value) =>
                write!(formatter, "artifact {value} depends on itself"),

            Self::InvalidDependencyIdentity(value) =>
                write!(formatter, "invalid dependency identity: {value}"),

            Self::DuplicateDependency(owner, dependency) =>
                write!(formatter, "duplicate dependency {dependency} for {owner}"),

            Self::NonCanonicalDependencies =>
                formatter.write_str("dependencies are not canonically ordered"),

            Self::NonCanonicalArtifactOrder =>
                formatter.write_str("AI artifacts are not canonically ordered"),

            Self::ModelSchemaWithoutModel =>
                formatter.write_str("model schema version requires a model identity"),

            Self::ModelWithoutSchemaVersion =>
                formatter.write_str("model identity requires a model schema version"),

            Self::ArtifactAssociationMismatch =>
                formatter.write_str("AI and backend artifact associations disagree"),

            Self::InvalidMetadata =>
                formatter.write_str("invalid or empty deployment metadata"),

            Self::InvalidTextField(field) =>
                write!(formatter, "invalid deployment text field: {field}"),

            Self::UnsupportedSchema(value) =>
                write!(formatter, "unsupported AI deployment schema: {value}"),

            Self::UnsupportedSchemaVersion { supplied, supported } =>
                write!(formatter, "unsupported AI deployment schema version {supplied}; supported version is {supported}"),

            Self::Artifact(error) =>
                write!(formatter, "AI artifact error: {error}"),

            Self::Backend(error) =>
                write!(formatter, "backend manifest error: {error}"),

            Self::Json(error) =>
                write!(formatter, "manifest JSON error: {error}"),
        }
    }
}

impl std::error::Error for AiDeploymentManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Artifact(error) => Some(error),
            Self::Backend(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<AiArtifactError> for AiDeploymentManifestError {
    fn from(error: AiArtifactError) -> Self {
        Self::Artifact(error)
    }
}

impl From<BackendError> for AiDeploymentManifestError {
    fn from(error: BackendError) -> Self {
        Self::Backend(error)
    }
}

impl From<serde_json::Error> for AiDeploymentManifestError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::deployment::artifacts::AiArtifactKind;

    use crate::backend::artifact::{
        ArtifactPayload,
    };

    use crate::backend::ids::{
        IdNamespace,
    };

    fn ai_id(value: u128) -> AiArtifactId {
        AiArtifactId::new(value).expect("valid AI ID")
    }

    fn backend_id(value: u128) -> ArtifactId {
        ArtifactId::new(
            IdNamespace::new("zamani.ai.deployment")
                .expect("valid namespace"),
            value,
        )
        .expect("valid backend ID")
    }

    fn sample_artifact(
        ai: u128,
        backend: u128,
    ) -> AiArtifact {
        let output = OutputArtifact::new(
            backend_id(backend),
            "portable",
            "ai.model+v1",
            ArtifactPayload::binary(vec![1, 2, 3, 4]),
        )
        .expect("valid backend artifact");

        AiArtifact::new(
            ai_id(ai),
            AiArtifactKind::model(),
            output,
        )
        .expect("valid AI artifact")
    }

    #[test]
    fn creates_empty_manifest() {
        let manifest = AiDeploymentManifest::new();

        assert!(manifest.is_empty());
        assert_eq!(manifest.artifact_count(), 0);

        manifest.validate().expect("valid empty manifest");
    }

    #[test]
    fn creates_manifest_from_ai_artifacts() {
        let artifacts = vec![
            sample_artifact(2, 20),
            sample_artifact(1, 10),
        ];

        let manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        assert_eq!(manifest.artifact_count(), 2);

        assert_eq!(manifest.artifacts()[0].artifact_id, 1);
        assert_eq!(manifest.artifacts()[1].artifact_id, 2);

        manifest.validate().expect("valid manifest");
    }

    #[test]
    fn rejects_duplicate_ai_identities() {
        let artifacts = vec![
            sample_artifact(1, 10),
            sample_artifact(1, 20),
        ];

        assert!(
            AiDeploymentManifest::from_artifacts(
                &artifacts
            ).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_backend_identities() {
        let artifacts = vec![
            sample_artifact(1, 10),
            sample_artifact(2, 10),
        ];

        assert!(
            AiDeploymentManifest::from_artifacts(
                &artifacts
            ).is_err()
        );
    }

    #[test]
    fn supports_atomic_dependency_registration() {
        let artifacts = vec![
            sample_artifact(1, 10),
            sample_artifact(2, 20),
        ];

        let mut manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        manifest.add_dependency(
            ai_id(2),
            ai_id(1),
        ).expect("valid dependency");

        manifest.validate().expect("valid dependency graph");

        assert_eq!(
            manifest.artifacts()[1].dependencies,
            vec![1]
        );
    }

    #[test]
    fn rejects_dependency_cycles() {
        let artifacts = vec![
            sample_artifact(1, 10),
            sample_artifact(2, 20),
        ];

        let mut manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        manifest.add_dependency(
            ai_id(1),
            ai_id(2),
        ).expect("first dependency");

        assert!(
            manifest.add_dependency(
                ai_id(2),
                ai_id(1),
            ).is_err()
        );

        // The failed second operation must leave the original
        // manifest unchanged.
        manifest.validate().expect("original graph remains valid");
    }

    #[test]
    fn rejects_missing_dependencies() {
        let artifacts = vec![
            sample_artifact(1, 10),
        ];

        let mut manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        assert!(
            manifest.add_dependency(
                ai_id(1),
                ai_id(99),
            ).is_err()
        );
    }

    #[test]
    fn supports_metadata() {
        let mut manifest = AiDeploymentManifest::new();

        manifest.insert_metadata(
            "ai.deployment.environment",
            "portable",
        ).expect("metadata insertion");

        assert_eq!(
            manifest.metadata("ai.deployment.environment"),
            Some("portable")
        );
    }

    #[test]
    fn rejects_empty_metadata() {
        let mut manifest = AiDeploymentManifest::new();

        assert!(
            manifest.insert_metadata(
                "ai.environment",
                "",
            ).is_err()
        );
    }

    #[test]
    fn serializes_and_deserializes_deterministically() {
        let artifacts = vec![
            sample_artifact(2, 20),
            sample_artifact(1, 10),
        ];

        let manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        let json = manifest.to_json().expect("JSON serialization");

        let restored = AiDeploymentManifest::from_json(
            &json,
        ).expect("JSON deserialization");

        assert_eq!(manifest, restored);

        assert_eq!(
            manifest.to_json().expect("serialization"),
            restored.to_json().expect("serialization")
        );
    }

    #[test]
    fn produces_stable_fingerprints() {
        let artifacts = vec![
            sample_artifact(1, 10),
        ];

        let manifest = AiDeploymentManifest::from_artifacts(
            &artifacts,
        ).expect("manifest construction");

        assert_eq!(
            manifest.sha256_digest().expect("digest"),
            manifest.sha256_digest().expect("same digest")
        );
    }

    #[test]
    fn rejects_unknown_schema_versions() {
        let mut manifest = AiDeploymentManifest::new();

        manifest.schema_version =
            AI_DEPLOYMENT_MANIFEST_SCHEMA_VERSION + 1;

        assert!(manifest.validate().is_err());
        assert!(manifest.to_json().is_err());
    }

    #[test]
    fn preserves_binary_payload_integrity() {
        let artifact = sample_artifact(1, 10);

        let manifest = AiDeploymentManifest::from_artifacts(
            &[artifact],
        ).expect("manifest construction");

        let backend = manifest.backend_manifest();

        assert_eq!(backend.artifact_count(), 1);

        backend.validate().expect("backend manifest validity");
    }

    #[test]
    fn supports_large_numeric_identities_without_machine_limits() {
        let large_id = u128::MAX;

        let artifact = sample_artifact(
            large_id,
            100,
        );

        let manifest = AiDeploymentManifest::from_artifacts(
            &[artifact],
        ).expect("large identity must be representable");

        assert_eq!(
            manifest.artifacts()[0].artifact_id,
            large_id
        );
    }
}
