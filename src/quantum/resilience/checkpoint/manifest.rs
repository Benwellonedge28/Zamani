//! Zamani Quantum Resilience — Checkpoint Manifest
//!
//! Path:
//!     src/quantum/resilience/checkpoint/manifest.rs
//!
//! Purpose:
//!     Defines the canonical, backend-independent manifest for a quantum
//!     resilience checkpoint.
//
//! Architectural ownership:
//!
//!     manifest.rs
//!         Owns:
//!             - manifest identity;
//!             - manifest schema/version;
//!             - checkpoint-to-manifest association;
//!             - artifact enumeration;
//!             - artifact roles;
//!             - artifact ordering;
//!             - artifact cardinality;
//!             - manifest lifecycle;
//!             - manifest-level metadata;
//!             - deterministic manifest validation;
//!             - manifest construction contracts.
//!
//!     checkpoint.rs
//!         Owns:
//!             - checkpoint identity;
//!             - checkpoint semantic boundary;
//!             - checkpoint state classification;
//!             - checkpoint payload descriptors;
//!             - resource/qubit scope;
//!             - checkpoint lifecycle.
//!
//!     snapshot.rs
//!         Owns:
//!             - snapshot-specific state descriptors;
//!             - provider/runtime snapshot metadata.
//!
//!     storage.rs
//!         Owns:
//!             - physical storage;
//!             - storage reads/writes/deletes;
//!             - storage-provider adaptation.
//!
//!     integrity.rs
//!         Owns:
//!             - digest calculation;
//!             - signatures;
//!             - authenticity;
//!             - integrity verification.
//!
//!     compatibility.rs
//!         Owns:
//!             - checkpoint compatibility;
//!             - target capability compatibility;
//!             - schema compatibility.
//!
//!     recovery::*
//!         Owns:
//!             - restore;
//!             - rollback;
//!             - resume;
//!             - migration orchestration.
//!
//! Important:
//!
//!     A manifest is metadata. It does not contain checkpoint payload bytes.
//!
//!     A manifest MUST NOT imply that a quantum state is restorable merely
//!     because artifacts exist. Restoration semantics belong to checkpoint.rs,
//!     snapshot.rs, compatibility.rs, and recovery::*.
//!
//! Scalability:
//!
//!     This module deliberately contains no:
//!
//!         MAX_QUBITS
//!         MAX_ARTIFACTS
//!         MAX_CHECKPOINTS
//!         MAX_DEVICES
//!         MAX_BYTES
//!         provider-specific limits
//!
//!     Collections are dynamically sized. Actual limits are imposed by the
//!     caller, policy, resource manager, memory/storage provider, and runtime.
//!
//! Determinism:
//!
//!     Manifest validation is deterministic.
//!
//!     Artifact sequence numbers are explicit rather than derived from
//!     insertion order.
//!
//!     Artifact identities are opaque and provider-neutral.
//!
//!     Metadata uses BTreeMap so serialized key ordering is deterministic.
//!
//! Security:
//!
//!     This module does not calculate cryptographic digests or verify
//!     signatures. It only records references required by those layers.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::checkpoint::{
    ArtifactId,
    CheckpointId,
    CheckpointTimestamp,
};

// ============================================================================
// Schema identity
// ============================================================================

/// Stable manifest schema namespace.
pub const CHECKPOINT_MANIFEST_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint.manifest";

/// Current manifest schema major version.
///
/// A change that makes an existing serialized manifest impossible to interpret
/// according to its previous contract requires a major version change.
pub const CHECKPOINT_MANIFEST_SCHEMA_MAJOR: u16 = 1;

/// Current manifest schema minor version.
///
/// Backwards-compatible additions may advance this version.
pub const CHECKPOINT_MANIFEST_SCHEMA_MINOR: u16 = 0;

/// Current manifest schema patch version.
///
/// Non-semantic corrections may advance this version.
pub const CHECKPOINT_MANIFEST_SCHEMA_PATCH: u16 = 0;

// ============================================================================
// Manifest identity
// ============================================================================

/// Stable opaque manifest identifier.
///
/// The representation is intentionally opaque. A manifest ID must not encode:
///
/// - provider;
/// - hardware size;
/// - qubit count;
/// - storage implementation;
/// - memory address;
/// - retry count.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ManifestId(String);

impl ManifestId {
    /// Creates a validated manifest identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ManifestError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ManifestError::InvalidIdentifier {
                field: "manifest_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ManifestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Schema version
// ============================================================================

/// Three-component manifest schema version.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct ManifestSchemaVersion {
    /// Breaking version.
    pub major: u16,

    /// Backwards-compatible feature version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}

impl ManifestSchemaVersion {
    /// Current schema version.
    pub const CURRENT: Self = Self {
        major: CHECKPOINT_MANIFEST_SCHEMA_MAJOR,
        minor: CHECKPOINT_MANIFEST_SCHEMA_MINOR,
        patch: CHECKPOINT_MANIFEST_SCHEMA_PATCH,
    };

    /// Creates a schema version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns whether this version has the same major version as another.
    #[must_use]
    pub const fn compatible_major(self, other: Self) -> bool {
        self.major == other.major
    }
}

// ============================================================================
// Artifact role
// ============================================================================

/// Semantic role of an artifact inside a checkpoint manifest.
///
/// This describes what the artifact represents, not how it is stored.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum ManifestArtifactRole {
    /// Core checkpoint metadata.
    Metadata,

    /// Checkpoint execution state.
    ExecutionState,

    /// Classical runtime state.
    ClassicalState,

    /// Compiled/replayable program representation.
    CompiledProgram,

    /// Canonical program/IR reference.
    ProgramRepresentation,

    /// Logical quantum state representation.
    LogicalState,

    /// QEC state.
    QecState,

    /// Provider-supported runtime snapshot.
    ProviderSnapshot,

    /// Reconstructible runtime state.
    ReconstructibleState,

    /// Measurement-boundary information.
    MeasurementBoundary,

    /// Intermediate classical results required for replay.
    ClassicalResults,

    /// Execution journal/replay information.
    ExecutionJournal,

    /// Runtime-specific auxiliary state.
    RuntimeMetadata,

    /// Provenance information.
    Provenance,

    /// Compatibility requirements.
    Compatibility,

    /// Integrity metadata.
    Integrity,

    /// Signature/authentication metadata.
    Authentication,

    /// Provider-specific auxiliary artifact.
    ProviderAuxiliary,

    /// User/application-defined artifact.
    Custom,
}

// ============================================================================
// Artifact descriptor
// ============================================================================

/// A single artifact entry in a checkpoint manifest.
///
/// The descriptor intentionally does not contain the artifact bytes.
/// Storage references and cryptographic verification data remain owned by the
/// corresponding checkpoint/storage/integrity contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestArtifact {
    /// Stable artifact identity owned by checkpoint.rs.
    pub artifact_id: ArtifactId,

    /// Semantic role of the artifact.
    pub role: ManifestArtifactRole,

    /// Explicit deterministic position in the manifest.
    ///
    /// Sequence numbers are sparse-capable and do not imply a fixed maximum.
    pub sequence: u64,

    /// Whether restoration requires this artifact.
    ///
    /// Optional artifacts may be absent if the restoration policy permits it.
    pub required: bool,

    /// Optional logical grouping identifier.
    ///
    /// This allows large distributed checkpoints to group artifacts without
    /// requiring a single giant in-memory payload.
    pub group_id: Option<String>,

    /// Optional artifact media/content type.
    ///
    /// This is descriptive only. Serialization/storage implementations decide
    /// the actual encoding.
    pub media_type: Option<String>,

    /// Optional uncompressed logical size in bytes.
    ///
    /// This is metadata and does not allocate that amount of memory.
    pub logical_byte_length: Option<u64>,
}

impl ManifestArtifact {
    /// Creates a required artifact descriptor.
    pub fn required(
        artifact_id: ArtifactId,
        role: ManifestArtifactRole,
        sequence: u64,
    ) -> Self {
        Self {
            artifact_id,
            role,
            sequence,
            required: true,
            group_id: None,
            media_type: None,
            logical_byte_length: None,
        }
    }

    /// Creates an optional artifact descriptor.
    pub fn optional(
        artifact_id: ArtifactId,
        role: ManifestArtifactRole,
        sequence: u64,
    ) -> Self {
        Self {
            artifact_id,
            role,
            sequence,
            required: false,
            group_id: None,
            media_type: None,
            logical_byte_length: None,
        }
    }

    /// Adds a logical group identifier.
    #[must_use]
    pub fn with_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    /// Adds a media/content type.
    #[must_use]
    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Adds the logical byte length.
    #[must_use]
    pub fn with_logical_byte_length(
        mut self,
        logical_byte_length: u64,
    ) -> Self {
        self.logical_byte_length = Some(logical_byte_length);
        self
    }

    /// Validates the artifact descriptor.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if let Some(group_id) = &self.group_id {
            if group_id.trim().is_empty() {
                return Err(ManifestError::InvalidArtifactMetadata {
                    artifact_id: self.artifact_id.clone(),
                    field: "group_id",
                });
            }
        }

        if let Some(media_type) = &self.media_type {
            if media_type.trim().is_empty() {
                return Err(ManifestError::InvalidArtifactMetadata {
                    artifact_id: self.artifact_id.clone(),
                    field: "media_type",
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Manifest lifecycle
// ============================================================================

/// Lifecycle state of a checkpoint manifest.
///
/// Storage transactions are implemented elsewhere. This enum records the
/// logical state observed by those layers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum ManifestState {
    /// Manifest has been constructed but not committed.
    Draft,

    /// Manifest has passed local validation and is ready for persistence.
    Prepared,

    /// Manifest and its required artifact references have been committed.
    Committed,

    /// Manifest has been invalidated and must not be used for restoration.
    Invalidated,

    /// Manifest has been superseded by another manifest.
    Superseded,
}

impl ManifestState {
    /// Returns whether the manifest can be used as an authoritative manifest.
    #[must_use]
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::Committed)
    }

    /// Returns whether this state permits mutation through the builder.
    #[must_use]
    pub const fn is_mutable(self) -> bool {
        matches!(self, Self::Draft)
    }
}

// ============================================================================
// Manifest metadata
// ============================================================================

/// Deterministic, extensible manifest metadata.
///
/// `BTreeMap` is deliberately used instead of `HashMap` so serializers and
/// hashes can obtain stable key ordering.
pub type ManifestMetadata = BTreeMap<String, String>;

/// Optional relationship between manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestLineage {
    /// Parent manifest from which this manifest was derived.
    pub parent_manifest_id: Option<ManifestId>,

    /// Manifest that supersedes this one, when known.
    pub supersedes_manifest_id: Option<ManifestId>,

    /// Stable opaque lineage reason.
    pub reason: Option<String>,
}

impl ManifestLineage {
    /// Empty lineage.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            parent_manifest_id: None,
            supersedes_manifest_id: None,
            reason: None,
        }
    }

    /// Validates lineage.
    pub fn validate(
        &self,
        manifest_id: &ManifestId,
    ) -> Result<(), ManifestError> {
        if self.parent_manifest_id.as_ref() == Some(manifest_id) {
            return Err(ManifestError::SelfReferentialLineage);
        }

        if self.supersedes_manifest_id.as_ref() == Some(manifest_id) {
            return Err(ManifestError::SelfReferentialLineage);
        }

        if let Some(reason) = &self.reason {
            if reason.trim().is_empty() {
                return Err(ManifestError::InvalidLineage);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Manifest
// ============================================================================

/// Canonical checkpoint manifest.
///
/// A manifest identifies and orders all artifacts belonging to one logical
/// checkpoint. It intentionally contains metadata/references rather than
/// payload bytes.
///
/// This design allows:
///
///     tiny checkpoint
///         -> one artifact
///
///     large checkpoint
///         -> many artifacts
///
///     distributed checkpoint
///         -> artifacts distributed across many resources
///
/// without changing the manifest contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointManifest {
    /// Stable schema namespace.
    pub schema_id: String,

    /// Manifest schema version.
    pub schema_version: ManifestSchemaVersion,

    /// Stable manifest identity.
    pub manifest_id: ManifestId,

    /// Checkpoint to which this manifest belongs.
    pub checkpoint_id: CheckpointId,

    /// Creation timestamp.
    pub created_at: CheckpointTimestamp,

    /// Current manifest lifecycle state.
    pub state: ManifestState,

    /// Enumerated artifacts.
    ///
    /// `Arc<[T]>` keeps the representation immutable after construction while
    /// avoiding unnecessary copies when manifests are shared.
    pub artifacts: Arc<[ManifestArtifact]>,

    /// Optional designated root artifact.
    ///
    /// The root is a logical relationship only. Storage hierarchy is owned by
    /// storage.rs.
    pub root_artifact_id: Option<ArtifactId>,

    /// Manifest lineage.
    pub lineage: ManifestLineage,

    /// Extensible deterministic metadata.
    pub metadata: ManifestMetadata,
}

impl CheckpointManifest {
    /// Creates a new draft manifest.
    ///
    /// The artifact list is validated immediately. The returned manifest is
    /// still a draft and therefore not authoritative.
    pub fn new(
        manifest_id: ManifestId,
        checkpoint_id: CheckpointId,
        created_at: CheckpointTimestamp,
        artifacts: impl IntoIterator<Item = ManifestArtifact>,
    ) -> Result<Self, ManifestError> {
        let artifacts: Vec<ManifestArtifact> = artifacts.into_iter().collect();

        let manifest = Self {
            schema_id: CHECKPOINT_MANIFEST_SCHEMA_ID.to_owned(),
            schema_version: ManifestSchemaVersion::CURRENT,
            manifest_id,
            checkpoint_id,
            created_at,
            state: ManifestState::Draft,
            artifacts: Arc::from(artifacts),
            root_artifact_id: None,
            lineage: ManifestLineage::empty(),
            metadata: BTreeMap::new(),
        };

        manifest.validate()?;

        Ok(manifest)
    }

    /// Returns the number of artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Returns whether no artifacts are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    /// Returns all artifacts in manifest order.
    #[must_use]
    pub fn artifacts(&self) -> &[ManifestArtifact] {
        &self.artifacts
    }

    /// Finds an artifact by identity.
    #[must_use]
    pub fn artifact(
        &self,
        artifact_id: &ArtifactId,
    ) -> Option<&ManifestArtifact> {
        self.artifacts
            .iter()
            .find(|artifact| &artifact.artifact_id == artifact_id)
    }

    /// Returns all artifacts having the specified semantic role.
    ///
    /// The returned iterator is lazy and therefore does not allocate another
    /// artifact collection.
    pub fn artifacts_with_role(
        &self,
        role: ManifestArtifactRole,
    ) -> impl Iterator<Item = &ManifestArtifact> {
        self.artifacts
            .iter()
            .filter(move |artifact| artifact.role == role)
    }

    /// Returns all required artifacts lazily.
    pub fn required_artifacts(
        &self,
    ) -> impl Iterator<Item = &ManifestArtifact> {
        self.artifacts.iter().filter(|artifact| artifact.required)
    }

    /// Returns all optional artifacts lazily.
    pub fn optional_artifacts(
        &self,
    ) -> impl Iterator<Item = &ManifestArtifact> {
        self.artifacts.iter().filter(|artifact| !artifact.required)
    }

    /// Returns the total declared logical payload size.
    ///
    /// `None` means at least one artifact has no declared size.
    ///
    /// Overflow is treated as unknown rather than wrapping.
    #[must_use]
    pub fn declared_logical_byte_length(&self) -> Option<u64> {
        let mut total = 0_u64;

        for artifact in self.artifacts.iter() {
            let length = artifact.logical_byte_length?;

            total = total.checked_add(length)?;
        }

        Some(total)
    }

    /// Returns the root artifact, if configured.
    #[must_use]
    pub fn root_artifact(&self) -> Option<&ManifestArtifact> {
        self.root_artifact_id
            .as_ref()
            .and_then(|id| self.artifact(id))
    }

    /// Sets the root artifact.
    ///
    /// The manifest remains a value object; this method is intended for
    /// controlled construction before committing the manifest.
    pub fn set_root_artifact(
        &mut self,
        artifact_id: ArtifactId,
    ) -> Result<(), ManifestError> {
        if !self.state.is_mutable() {
            return Err(ManifestError::ImmutableManifest);
        }

        if self.artifact(&artifact_id).is_none() {
            return Err(ManifestError::UnknownArtifact {
                artifact_id,
            });
        }

        self.root_artifact_id = Some(artifact_id);

        Ok(())
    }

    /// Sets lineage metadata.
    pub fn set_lineage(
        &mut self,
        lineage: ManifestLineage,
    ) -> Result<(), ManifestError> {
        if !self.state.is_mutable() {
            return Err(ManifestError::ImmutableManifest);
        }

        lineage.validate(&self.manifest_id)?;
        self.lineage = lineage;

        Ok(())
    }

    /// Inserts deterministic metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ManifestError> {
        if !self.state.is_mutable() {
            return Err(ManifestError::ImmutableManifest);
        }

        let key = key.into();

        if key.trim().is_empty() {
            return Err(ManifestError::InvalidMetadataKey);
        }

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Marks the manifest as prepared.
    ///
    /// This does not perform storage I/O. The storage layer is responsible for
    /// atomic persistence.
    pub fn prepare(&mut self) -> Result<(), ManifestError> {
        self.validate()?;

        if !self.state.is_mutable() {
            return Err(ManifestError::InvalidStateTransition {
                from: self.state,
                to: ManifestState::Prepared,
            });
        }

        self.state = ManifestState::Prepared;

        Ok(())
    }

    /// Marks the manifest as committed.
    ///
    /// The storage implementation must only call this after its persistence
    /// transaction has successfully established the manifest and required
    /// artifact references.
    pub fn commit(&mut self) -> Result<(), ManifestError> {
        self.validate()?;

        if !matches!(self.state, ManifestState::Prepared) {
            return Err(ManifestError::InvalidStateTransition {
                from: self.state,
                to: ManifestState::Committed,
            });
        }

        self.state = ManifestState::Committed;

        Ok(())
    }

    /// Invalidates the manifest.
    ///
    /// An invalidated manifest must never be accepted as a restore source.
    pub fn invalidate(&mut self) -> Result<(), ManifestError> {
        if matches!(self.state, ManifestState::Invalidated) {
            return Ok(());
        }

        if matches!(self.state, ManifestState::Superseded) {
            return Err(ManifestError::InvalidStateTransition {
                from: self.state,
                to: ManifestState::Invalidated,
            });
        }

        self.state = ManifestState::Invalidated;

        Ok(())
    }

    /// Marks this manifest as superseded.
    pub fn supersede(&mut self) -> Result<(), ManifestError> {
        if !matches!(self.state, ManifestState::Committed) {
            return Err(ManifestError::InvalidStateTransition {
                from: self.state,
                to: ManifestState::Superseded,
            });
        }

        self.state = ManifestState::Superseded;

        Ok(())
    }

    /// Validates the complete manifest.
    ///
    /// This validation is deliberately pure:
    ///
    /// - no storage;
    /// - no network;
    /// - no hardware;
    /// - no provider calls;
    /// - no cryptographic hashing;
    /// - no mutation.
    ///
    /// External verification belongs to integrity.rs and compatibility.rs.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.schema_id.trim().is_empty() {
            return Err(ManifestError::InvalidSchemaId);
        }

        if self.schema_id != CHECKPOINT_MANIFEST_SCHEMA_ID {
            return Err(ManifestError::UnsupportedSchema {
                schema_id: self.schema_id.clone(),
            });
        }

        if self.schema_version.major
            != CHECKPOINT_MANIFEST_SCHEMA_MAJOR
        {
            return Err(ManifestError::IncompatibleSchemaVersion {
                expected_major: CHECKPOINT_MANIFEST_SCHEMA_MAJOR,
                actual_major: self.schema_version.major,
            });
        }

        if self.manifest_id.as_str().trim().is_empty() {
            return Err(ManifestError::InvalidIdentifier {
                field: "manifest_id",
            });
        }

        if self.checkpoint_id.as_str().trim().is_empty() {
            return Err(ManifestError::InvalidIdentifier {
                field: "checkpoint_id",
            });
        }

        self.lineage.validate(&self.manifest_id)?;

        self.validate_artifacts()?;

        if let Some(root_artifact_id) = &self.root_artifact_id {
            if self.artifact(root_artifact_id).is_none() {
                return Err(ManifestError::UnknownArtifact {
                    artifact_id: root_artifact_id.clone(),
                });
            }
        }

        for key in self.metadata.keys() {
            if key.trim().is_empty() {
                return Err(ManifestError::InvalidMetadataKey);
            }
        }

        Ok(())
    }

    fn validate_artifacts(&self) -> Result<(), ManifestError> {
        let mut previous_sequence: Option<u64> = None;

        for artifact in self.artifacts.iter() {
            artifact.validate()?;

            if let Some(previous) = previous_sequence {
                if artifact.sequence <= previous {
                    return Err(ManifestError::InvalidArtifactSequence {
                        artifact_id: artifact.artifact_id.clone(),
                        sequence: artifact.sequence,
                    });
                }
            }

            previous_sequence = Some(artifact.sequence);

            if self
                .artifacts
                .iter()
                .take_while(|candidate| {
                    candidate.sequence <= artifact.sequence
                })
                .filter(|candidate| {
                    candidate.artifact_id == artifact.artifact_id
                })
                .count()
                > 1
            {
                return Err(ManifestError::DuplicateArtifact {
                    artifact_id: artifact.artifact_id.clone(),
                });
            }
        }

        Ok(())
    }

    /// Creates an immutable prepared copy.
    pub fn into_prepared(mut self) -> Result<Self, ManifestError> {
        self.prepare()?;
        Ok(self)
    }

    /// Returns whether this manifest is committed and therefore eligible to
    /// enter external restore/compatibility verification.
    #[must_use]
    pub fn is_committed(&self) -> bool {
        self.state.is_authoritative()
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for checkpoint manifests.
///
/// The builder prevents callers from needing to construct a partially valid
/// manifest manually.
#[derive(Debug, Default)]
pub struct CheckpointManifestBuilder {
    manifest_id: Option<ManifestId>,
    checkpoint_id: Option<CheckpointId>,
    created_at: Option<CheckpointTimestamp>,
    artifacts: Vec<ManifestArtifact>,
    root_artifact_id: Option<ArtifactId>,
    lineage: ManifestLineage,
    metadata: ManifestMetadata,
}

impl CheckpointManifestBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the manifest identity.
    #[must_use]
    pub fn manifest_id(mut self, value: ManifestId) -> Self {
        self.manifest_id = Some(value);
        self
    }

    /// Sets the checkpoint identity.
    #[must_use]
    pub fn checkpoint_id(mut self, value: CheckpointId) -> Self {
        self.checkpoint_id = Some(value);
        self
    }

    /// Sets the creation timestamp.
    #[must_use]
    pub fn created_at(mut self, value: CheckpointTimestamp) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Adds one artifact.
    #[must_use]
    pub fn artifact(mut self, artifact: ManifestArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    /// Adds many artifacts.
    pub fn artifacts(
        mut self,
        artifacts: impl IntoIterator<Item = ManifestArtifact>,
    ) -> Self {
        self.artifacts.extend(artifacts);
        self
    }

    /// Sets the root artifact.
    #[must_use]
    pub fn root_artifact(mut self, artifact_id: ArtifactId) -> Self {
        self.root_artifact_id = Some(artifact_id);
        self
    }

    /// Sets lineage.
    #[must_use]
    pub fn lineage(mut self, lineage: ManifestLineage) -> Self {
        self.lineage = lineage;
        self
    }

    /// Adds metadata.
    #[must_use]
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Builds and validates a draft manifest.
    pub fn build(self) -> Result<CheckpointManifest, ManifestError> {
        let manifest_id = self
            .manifest_id
            .ok_or(ManifestError::MissingField {
                field: "manifest_id",
            })?;

        let checkpoint_id = self
            .checkpoint_id
            .ok_or(ManifestError::MissingField {
                field: "checkpoint_id",
            })?;

        let created_at = self
            .created_at
            .ok_or(ManifestError::MissingField {
                field: "created_at",
            })?;

        let manifest = CheckpointManifest {
            schema_id: CHECKPOINT_MANIFEST_SCHEMA_ID.to_owned(),
            schema_version: ManifestSchemaVersion::CURRENT,
            manifest_id,
            checkpoint_id,
            created_at,
            state: ManifestState::Draft,
            artifacts: Arc::from(self.artifacts),
            root_artifact_id: self.root_artifact_id,
            lineage: self.lineage,
            metadata: self.metadata,
        };

        manifest.validate()?;

        Ok(manifest)
    }
}

// ============================================================================
// Manifest summary
// ============================================================================

/// Lightweight manifest summary.
///
/// This is useful for discovery/listing without requiring callers to clone
/// or inspect every artifact entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestSummary {
    /// Manifest identity.
    pub manifest_id: ManifestId,

    /// Associated checkpoint.
    pub checkpoint_id: CheckpointId,

    /// Lifecycle state.
    pub state: ManifestState,

    /// Number of artifacts.
    pub artifact_count: u64,

    /// Number of required artifacts.
    pub required_artifact_count: u64,

    /// Number of optional artifacts.
    pub optional_artifact_count: u64,

    /// Declared total logical bytes, if all artifact sizes are known.
    pub declared_logical_byte_length: Option<u64>,
}

impl CheckpointManifest {
    /// Creates a lightweight summary.
    #[must_use]
    pub fn summary(&self) -> ManifestSummary {
        let mut required = 0_u64;
        let mut optional = 0_u64;

        for artifact in self.artifacts.iter() {
            if artifact.required {
                required = required.saturating_add(1);
            } else {
                optional = optional.saturating_add(1);
            }
        }

        ManifestSummary {
            manifest_id: self.manifest_id.clone(),
            checkpoint_id: self.checkpoint_id.clone(),
            state: self.state,
            artifact_count: self.artifacts.len() as u64,
            required_artifact_count: required,
            optional_artifact_count: optional,
            declared_logical_byte_length:
                self.declared_logical_byte_length(),
        }
    }
}

// ============================================================================
// Manifest validation helpers
// ============================================================================

/// Validates a manifest without consuming it.
///
/// Useful for storage, compatibility, recovery, and serialization layers.
pub fn validate_manifest(
    manifest: &CheckpointManifest,
) -> Result<(), ManifestError> {
    manifest.validate()
}

/// Returns whether two manifests belong to the same logical checkpoint.
#[must_use]
pub fn same_checkpoint(
    left: &CheckpointManifest,
    right: &CheckpointManifest,
) -> bool {
    left.checkpoint_id == right.checkpoint_id
}

/// Returns whether two manifests are different versions of the same
/// checkpoint lineage.
#[must_use]
pub fn same_lineage(
    left: &CheckpointManifest,
    right: &CheckpointManifest,
) -> bool {
    left.checkpoint_id == right.checkpoint_id
        || left.lineage.parent_manifest_id.as_ref()
            == Some(&right.manifest_id)
        || right.lineage.parent_manifest_id.as_ref()
            == Some(&left.manifest_id)
}

// ============================================================================
// Errors
// ============================================================================

/// Manifest-specific error taxonomy.
///
/// Lower-level checkpoint, storage, integrity, compatibility and recovery
/// errors remain owned by their respective modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    /// Required field was not supplied.
    MissingField {
        /// Missing field name.
        field: &'static str,
    },

    /// Identifier is empty.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// Manifest schema identifier is empty.
    InvalidSchemaId,

    /// Manifest schema is unknown.
    UnsupportedSchema {
        /// Received schema identifier.
        schema_id: String,
    },

    /// Major schema version is incompatible.
    IncompatibleSchemaVersion {
        /// Required major version.
        expected_major: u16,

        /// Received major version.
        actual_major: u16,
    },

    /// Artifact identity occurs more than once.
    DuplicateArtifact {
        /// Duplicate artifact identity.
        artifact_id: ArtifactId,
    },

    /// Artifact sequence is not strictly increasing.
    InvalidArtifactSequence {
        /// Artifact with the invalid sequence.
        artifact_id: ArtifactId,

        /// Invalid sequence.
        sequence: u64,
    },

    /// Artifact metadata is invalid.
    InvalidArtifactMetadata {
        /// Artifact identity.
        artifact_id: ArtifactId,

        /// Invalid metadata field.
        field: &'static str,
    },

    /// Requested root/relationship artifact does not exist.
    UnknownArtifact {
        /// Missing artifact identity.
        artifact_id: ArtifactId,
    },

    /// Metadata key is empty.
    InvalidMetadataKey,

    /// Lineage points back to the manifest itself.
    SelfReferentialLineage,

    /// Lineage metadata is invalid.
    InvalidLineage,

    /// Manifest was modified after becoming immutable.
    ImmutableManifest,

    /// Lifecycle transition is invalid.
    InvalidStateTransition {
        /// Existing state.
        from: ManifestState,

        /// Requested state.
        to: ManifestState,
    },
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { field } => {
                write!(formatter, "missing manifest field: {field}")
            }
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid manifest identifier: {field}")
            }
            Self::InvalidSchemaId => {
                formatter.write_str("manifest schema identifier is empty")
            }
            Self::UnsupportedSchema { schema_id } => {
                write!(formatter, "unsupported manifest schema: {schema_id}")
            }
            Self::IncompatibleSchemaVersion {
                expected_major,
                actual_major,
            } => write!(
                formatter,
                "incompatible manifest schema major version: \
                 expected {expected_major}, received {actual_major}"
            ),
            Self::DuplicateArtifact { artifact_id } => {
                write!(
                    formatter,
                    "duplicate checkpoint manifest artifact: {artifact_id}"
                )
            }
            Self::InvalidArtifactSequence {
                artifact_id,
                sequence,
            } => write!(
                formatter,
                "invalid artifact sequence {sequence} for {artifact_id}"
            ),
            Self::InvalidArtifactMetadata {
                artifact_id,
                field,
            } => write!(
                formatter,
                "invalid metadata field {field} for artifact {artifact_id}"
            ),
            Self::UnknownArtifact { artifact_id } => {
                write!(formatter, "unknown manifest artifact: {artifact_id}")
            }
            Self::InvalidMetadataKey => {
                formatter.write_str("manifest metadata key is empty")
            }
            Self::SelfReferentialLineage => {
                formatter.write_str("manifest lineage is self-referential")
            }
            Self::InvalidLineage => {
                formatter.write_str("invalid manifest lineage")
            }
            Self::ImmutableManifest => {
                formatter.write_str("manifest is immutable")
            }
            Self::InvalidStateTransition { from, to } => write!(
                formatter,
                "invalid manifest state transition: {from:?} -> {to:?}"
            ),
        }
    }
}

impl std::error::Error for ManifestError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest_id(value: &str) -> ManifestId {
        ManifestId::new(value).expect("valid manifest id")
    }

    fn checkpoint_id(value: &str) -> CheckpointId {
        CheckpointId::new(value).expect("valid checkpoint id")
    }

    fn artifact_id(value: &str) -> ArtifactId {
        ArtifactId::new(value).expect("valid artifact id")
    }

    fn timestamp() -> CheckpointTimestamp {
        CheckpointTimestamp::new(1_700_000_000, 0)
            .expect("valid timestamp")
    }

    fn artifact(
        id: &str,
        sequence: u64,
    ) -> ManifestArtifact {
        ManifestArtifact::required(
            artifact_id(id),
            ManifestArtifactRole::Metadata,
            sequence,
        )
    }

    #[test]
    fn creates_valid_manifest() {
        let manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0)],
        )
        .expect("manifest should be valid");

        assert_eq!(manifest.artifact_count(), 1);
        assert!(manifest.validate().is_ok());
        assert!(!manifest.is_committed());
    }

    #[test]
    fn builder_creates_valid_manifest() {
        let manifest = CheckpointManifestBuilder::new()
            .manifest_id(manifest_id("manifest-1"))
            .checkpoint_id(checkpoint_id("checkpoint-1"))
            .created_at(timestamp())
            .artifact(artifact("artifact-1", 0))
            .root_artifact(artifact_id("artifact-1"))
            .metadata("purpose", "recovery")
            .build()
            .expect("builder should produce valid manifest");

        assert_eq!(
            manifest.root_artifact()
                .expect("root artifact")
                .artifact_id,
            artifact_id("artifact-1")
        );
    }

    #[test]
    fn duplicate_artifacts_are_rejected() {
        let result = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0), artifact("artifact-1", 1)],
        );

        assert!(matches!(
            result,
            Err(ManifestError::DuplicateArtifact { .. })
        ));
    }

    #[test]
    fn sequence_must_be_strictly_increasing() {
        let result = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 1), artifact("artifact-2", 1)],
        );

        assert!(matches!(
            result,
            Err(ManifestError::InvalidArtifactSequence { .. })
        ));
    }

    #[test]
    fn root_must_exist() {
        let mut manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0)],
        )
        .expect("valid manifest");

        let result =
            manifest.set_root_artifact(artifact_id("does-not-exist"));

        assert!(matches!(
            result,
            Err(ManifestError::UnknownArtifact { .. })
        ));
    }

    #[test]
    fn lifecycle_is_monotonic_for_normal_commit_path() {
        let mut manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0)],
        )
        .expect("valid manifest");

        assert_eq!(manifest.state, ManifestState::Draft);

        manifest.prepare().expect("prepare");
        assert_eq!(manifest.state, ManifestState::Prepared);

        manifest.commit().expect("commit");
        assert_eq!(manifest.state, ManifestState::Committed);
        assert!(manifest.is_committed());
    }

    #[test]
    fn cannot_commit_draft_directly() {
        let mut manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0)],
        )
        .expect("valid manifest");

        let result = manifest.commit();

        assert!(matches!(
            result,
            Err(ManifestError::InvalidStateTransition {
                from: ManifestState::Draft,
                to: ManifestState::Committed,
            })
        ));
    }

    #[test]
    fn self_lineage_is_rejected() {
        let id = manifest_id("manifest-1");

        let lineage = ManifestLineage {
            parent_manifest_id: Some(id.clone()),
            supersedes_manifest_id: None,
            reason: Some("invalid".to_owned()),
        };

        assert!(matches!(
            lineage.validate(&id),
            Err(ManifestError::SelfReferentialLineage)
        ));
    }

    #[test]
    fn summary_is_small_and_deterministic() {
        let manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [
                artifact("artifact-1", 0)
                    .with_logical_byte_length(10),
                ManifestArtifact::optional(
                    artifact_id("artifact-2"),
                    ManifestArtifactRole::Provenance,
                    1,
                )
                .with_logical_byte_length(20),
            ],
        )
        .expect("valid manifest");

        let summary = manifest.summary();

        assert_eq!(summary.artifact_count, 2);
        assert_eq!(summary.required_artifact_count, 1);
        assert_eq!(summary.optional_artifact_count, 1);
        assert_eq!(
            summary.declared_logical_byte_length,
            Some(30)
        );
    }

    #[test]
    fn unknown_size_produces_unknown_total() {
        let manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [
                artifact("artifact-1", 0)
                    .with_logical_byte_length(10),
                artifact("artifact-2", 1),
            ],
        )
        .expect("valid manifest");

        assert_eq!(
            manifest.declared_logical_byte_length(),
            None
        );
    }

    #[test]
    fn zero_artifact_manifest_is_structurally_valid() {
        let manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            std::iter::empty::<ManifestArtifact>(),
        )
        .expect("empty manifest can represent a metadata-only checkpoint");

        assert!(manifest.is_empty());
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn committed_manifest_cannot_be_modified_with_metadata() {
        let mut manifest = CheckpointManifest::new(
            manifest_id("manifest-1"),
            checkpoint_id("checkpoint-1"),
            timestamp(),
            [artifact("artifact-1", 0)],
        )
        .expect("valid manifest");

        manifest.prepare().expect("prepare");
        manifest.commit().expect("commit");

        let result = manifest.insert_metadata("key", "value");

        assert!(matches!(
            result,
            Err(ManifestError::ImmutableManifest)
        ));
    }
}