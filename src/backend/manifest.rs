
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Backend Artifact Manifest
//!
//! Path: `src/backend/manifest.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define a versioned, portable description of artifacts produced by a
//! backend compilation. A manifest records artifact identity, target,
//! format, encoding, size, content digest, validation evidence, provenance,
//! and artifact-to-artifact dependencies.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The manifest schema and its explicit schema version.
//! - Serializable snapshots of artifact metadata.
//! - Manifest construction and structural validation.
//! - Dependency-reference validation.
//! - Deterministic ordering of records.
//! - Manifest serialization and SHA-256 fingerprinting.
//! - Compatibility checks for manifest schema versions.
//!
//! It does not own:
//!
//! - Artifact payload storage or artifact ID allocation.
//! - Source-language semantics, grammar, semantic analysis, or IR.
//! - Target selection, hardware discovery, scheduling, or placement.
//! - Format-specific artifact verification.
//! - Filesystem persistence, deployment, linking, or execution.
//! - Trust decisions, digital signatures, or secret management.
//!
//! ## Architectural authority
//!
//! This module follows the semantic ownership boundaries described in:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! It integrates with:
//!
//! - `src/backend/artifact.rs`
//! - `src/backend/ids.rs`
//! - `src/backend/errors.rs`
//! - `src/backend/output.rs`
//! - `src/backend/reproducibility.rs` (when implemented)
//! - `src/compiler/linker.rs`
//!
//! ## Integrity and trust
//!
//! Artifact digests cover the exact payload bytes. A digest detects
//! accidental or deliberate content changes only when compared with a
//! trusted expected digest. It is not a signature and does not establish
//! artifact authenticity.
//!
//! A manifest is descriptive metadata, not proof that a compiler, verifier,
//! toolchain, or build process behaved honestly.
//!
//! ## Determinism
//!
//! Artifact records and dependency lists are sorted by canonical artifact
//! identity. Free-form metadata uses `BTreeMap`. No timestamps, host paths,
//! environment dumps, random values, or machine-local identifiers are added
//! implicitly.
//!
//! ## Scalability
//!
//! No universal artifact-count or artifact-size limit is imposed. Memory,
//! address-space, serialization, and process limits remain finite. Callers
//! should apply configurable operational budgets at the pipeline or storage
//! boundary rather than encoding hardware ceilings into this schema.
//!
//! ## Persistence boundary
//!
//! This module serializes to and from strings/byte vectors only. It does not
//! create directories, write files, access the network, or execute tools.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::artifact::{
    ArtifactEncoding, ArtifactProvenance, ArtifactValidation, OutputArtifact,
};
use super::errors::{BackendError, BackendResult};
use super::ids::ArtifactId;

/// Canonical identifier for this manifest schema.
pub const MANIFEST_SCHEMA_ID: &str = "zamani.backend.artifact-manifest";

/// Current manifest schema version.
///
/// Increment this when a change makes old serialized manifests incompatible
/// with the new schema contract. Additive changes must still be reviewed for
/// their effect on deterministic serialization and compatibility.
pub const MANIFEST_SCHEMA_VERSION: u32 = 1;

/// A portable snapshot of artifact validation evidence.
///
/// This deliberately does not serialize the artifact payload itself. The
/// payload is represented by its exact byte length and digest, allowing a
/// manifest to describe artifacts stored separately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ManifestValidation {
    /// No successful validation has been recorded.
    Unverified,

    /// A named verifier successfully validated the artifact.
    Verified {
        /// Name of the verifier or validation procedure.
        verifier: String,

        /// Optional verifier version or immutable identity.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        verifier_version: Option<String>,
    },

    /// Verification was explicitly waived by the applicable policy.
    NotRequired {
        /// Explanation of why verification was not required.
        reason: String,
    },
}

impl From<&ArtifactValidation> for ManifestValidation {
    fn from(validation: &ArtifactValidation) -> Self {
        match validation {
            ArtifactValidation::Unverified => Self::Unverified,
            ArtifactValidation::Verified {
                verifier,
                verifier_version,
            } => Self::Verified {
                verifier: verifier.clone(),
                verifier_version: verifier_version.clone(),
            },
            ArtifactValidation::NotRequired { reason } => Self::NotRequired {
                reason: reason.clone(),
            },
        }
    }
}

impl ManifestValidation {
    fn validate(&self) -> BackendResult<()> {
        match self {
            Self::Unverified => Ok(()),
            Self::Verified {
                verifier,
                verifier_version,
            } => {
                if verifier.trim().is_empty() {
                    return Err(invalid_manifest(
                        "verified artifact must name its verifier",
                    ));
                }

                if verifier_version
                    .as_ref()
                    .is_some_and(|value| value.trim().is_empty())
                {
                    return Err(invalid_manifest(
                        "verifier version must not be empty when supplied",
                    ));
                }

                Ok(())
            }
            Self::NotRequired { reason } => {
                if reason.trim().is_empty() {
                    return Err(invalid_manifest(
                        "verification waiver must include a reason",
                    ));
                }

                Ok(())
            }
        }
    }
}

/// Serializable snapshot of artifact provenance.
///
/// Provenance values are descriptive and must not be treated as authenticated
/// claims. Callers should avoid embedding secrets, access tokens, private
/// source contents, or machine-local paths in these fields.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ManifestProvenance {
    /// Producing backend identifier, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,

    /// Producing backend version, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_version: Option<String>,

    /// Stable source or IR identity, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_identity: Option<String>,

    /// Compilation or transformation stage, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,

    /// Ordered transformations known to have occurred.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transformations: Vec<String>,
}

impl From<&ArtifactProvenance> for ManifestProvenance {
    fn from(provenance: &ArtifactProvenance) -> Self {
        Self {
            backend: provenance.backend.clone(),
            backend_version: provenance.backend_version.clone(),
            input_identity: provenance.input_identity.clone(),
            stage: provenance.stage.clone(),
            transformations: provenance.transformations.clone(),
        }
    }
}

impl ManifestProvenance {
    fn validate(&self) -> BackendResult<()> {
        for (field, value) in [
            ("backend", self.backend.as_deref()),
            ("backend version", self.backend_version.as_deref()),
            ("input identity", self.input_identity.as_deref()),
            ("stage", self.stage.as_deref()),
        ] {
            if value.is_some_and(|value| value.trim().is_empty()) {
                return Err(invalid_manifest(format!(
                    "provenance {field} must not be empty when supplied"
                )));
            }
        }

        if self
            .transformations
            .iter()
            .any(|value| value.trim().is_empty())
        {
            return Err(invalid_manifest(
                "provenance transformations must not contain empty entries",
            ));
        }

        Ok(())
    }
}

/// One artifact record in a manifest.
///
/// The record stores metadata and integrity information, not the artifact's
/// actual payload. `artifact_id` uses the canonical textual representation
/// defined by `backend::ids`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestArtifact {
    /// Canonical artifact identity.
    pub artifact_id: String,

    /// Identity or triple of the target that produced the artifact.
    pub target: String,

    /// Extensible artifact-format identifier.
    pub format: String,

    /// Payload encoding, distinct from artifact format.
    pub encoding: ManifestEncoding,

    /// Exact payload length in bytes.
    pub size_bytes: u64,

    /// SHA-256 digest of the exact payload bytes.
    ///
    /// The canonical form is `sha256:` followed by 64 lowercase hexadecimal
    /// characters.
    pub content_digest: String,

    /// Optional suggested extension without a leading dot or path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,

    /// Optional media type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    /// Recorded validation evidence.
    pub validation: ManifestValidation,

    /// Optional provenance record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ManifestProvenance>,

    /// IDs of artifacts that this artifact depends on.
    ///
    /// This list is sorted and contains no duplicates. References must resolve
    /// to artifact records in the same manifest.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
}

/// Stable textual representation of an artifact's payload encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEncoding {
    /// Text-labelled payload.
    Text,

    /// Binary or opaque payload.
    Binary,
}

impl From<ArtifactEncoding> for ManifestEncoding {
    fn from(encoding: ArtifactEncoding) -> Self {
        match encoding {
            ArtifactEncoding::Text => Self::Text,
            ArtifactEncoding::Binary => Self::Binary,
        }
    }
}

impl ManifestArtifact {
    fn from_output_artifact(artifact: &OutputArtifact) -> BackendResult<Self> {
        artifact.validate_metadata()?;

        let digest = artifact.payload.sha256_digest();

        // Never copy an unchecked digest into the manifest. The artifact
        // validator checks any digest already stored on OutputArtifact.
        if let Some(recorded_digest) = &artifact.content_digest {
            if recorded_digest != &digest {
                return Err(invalid_manifest(format!(
                    "artifact {} has a content digest that does not match its payload",
                    artifact.id
                )));
            }
        }

        let size_bytes = u64::try_from(artifact.size_bytes()).map_err(|_| {
            invalid_manifest(format!(
                "artifact {} size cannot be represented as u64",
                artifact.id
            ))
        })?;

        Ok(Self {
            artifact_id: artifact.id.to_string(),
            target: artifact.target.clone(),
            format: artifact.format.clone(),
            encoding: artifact.payload.encoding().into(),
            size_bytes,
            content_digest: digest,
            extension: artifact.extension.clone(),
            media_type: artifact.media_type.clone(),
            validation: (&artifact.validation).into(),
            provenance: artifact
                .provenance
                .as_ref()
                .map(ManifestProvenance::from),
            dependencies: Vec::new(),
        })
    }

    fn validate(&self) -> BackendResult<()> {
        let parsed_id = ArtifactId::parse(&self.artifact_id).map_err(|error| {
            invalid_manifest(format!(
                "invalid artifact ID {:?}: {error}",
                self.artifact_id
            ))
        })?;

        if parsed_id.to_string() != self.artifact_id {
            return Err(invalid_manifest(
                "artifact ID is not in canonical textual form",
            ));
        }

        if self.target.trim().is_empty() {
            return Err(invalid_manifest(format!(
                "artifact {} has an empty target",
                self.artifact_id
            )));
        }

        // Reuse the artifact format identifier grammar, rather than
        // maintaining a second, potentially divergent format-name grammar.
        super::artifact::ArtifactFormat::new(self.format.clone())?;

        if let Some(extension) = &self.extension {
            if extension.trim().is_empty()
                || extension.starts_with('.')
                || extension.contains('/')
                || extension.contains('\\')
                || extension.contains('\0')
                || extension.contains("..")
            {
                return Err(invalid_manifest(format!(
                    "artifact {} has an invalid filename extension",
                    self.artifact_id
                )));
            }
        }

        if let Some(media_type) = &self.media_type {
            if media_type.trim().is_empty()
                || media_type.contains('\n')
                || media_type.contains('\r')
            {
                return Err(invalid_manifest(format!(
                    "artifact {} has an invalid media type",
                    self.artifact_id
                )));
            }
        }

        validate_sha256(&self.content_digest)?;
        self.validation.validate()?;

        if let Some(provenance) = &self.provenance {
            provenance.validate()?;
        }

        let mut unique_dependencies = BTreeSet::new();
        for dependency in &self.dependencies {
            let dependency_id = ArtifactId::parse(dependency).map_err(|error| {
                invalid_manifest(format!(
                    "artifact {} references invalid dependency ID {:?}: {error}",
                    self.artifact_id, dependency
                ))
            })?;

            if dependency_id.to_string() != *dependency {
                return Err(invalid_manifest(
                    "dependency ID is not in canonical textual form",
                ));
            }

            if dependency == &self.artifact_id {
                return Err(invalid_manifest(format!(
                    "artifact {} must not depend on itself",
                    self.artifact_id
                )));
            }

            if !unique_dependencies.insert(dependency.clone()) {
                return Err(invalid_manifest(format!(
                    "artifact {} contains a duplicate dependency",
                    self.artifact_id
                )));
            }
        }

        if !self.dependencies.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(invalid_manifest(format!(
                "dependencies for artifact {} are not in canonical sorted order",
                self.artifact_id
            )));
        }

        Ok(())
    }
}

/// A versioned, deterministic manifest describing a collection of artifacts.
///
/// `artifacts` are sorted by canonical artifact ID. Metadata keys are sorted
/// through `BTreeMap`. No timestamp or machine-specific state is inserted
/// automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    /// Stable schema identifier.
    pub schema: String,

    /// Version of this manifest schema.
    pub schema_version: u32,

    /// Optional stable compilation/build identity.
    ///
    /// This should identify a build using declared inputs, not a local path,
    /// wall-clock time, or an unrecorded environmental value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,

    /// Optional backend implementation identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,

    /// Optional backend implementation version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,

    /// Deterministically ordered artifact records.
    pub artifacts: Vec<ManifestArtifact>,

    /// Additional non-secret, portable build metadata.
    ///
    /// Callers are responsible for not putting credentials, secrets, private
    /// source contents, or unstable machine-local paths into this map.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl Default for ArtifactManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactManifest {
    /// Create an empty manifest using the current schema version.
    pub fn new() -> Self {
        Self {
            schema: MANIFEST_SCHEMA_ID.to_owned(),
            schema_version: MANIFEST_SCHEMA_VERSION,
            build_id: None,
            producer: None,
            producer_version: None,
            artifacts: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Construct a manifest from backend output artifacts.
    ///
    /// Payload digests are calculated from the exact artifact bytes.
    /// Duplicate artifact identities are rejected. The resulting records are
    /// sorted by canonical artifact ID for deterministic serialization.
    pub fn from_artifacts(artifacts: &[OutputArtifact]) -> BackendResult<Self> {
        let mut manifest = Self::new();

        for artifact in artifacts {
            manifest.add_artifact(artifact)?;
        }

        manifest.validate()?;
        Ok(manifest)
    }

    /// Add an artifact snapshot to the manifest.
    ///
    /// Dependencies can be added after all relevant artifacts have been
    /// inserted. The manifest must pass `validate` before it is serialized
    /// or published.
    pub fn add_artifact(&mut self, artifact: &OutputArtifact) -> BackendResult<()> {
        self.ensure_current_schema()?;

        let record = ManifestArtifact::from_output_artifact(artifact)?;

        if self
            .artifacts
            .iter()
            .any(|existing| existing.artifact_id == record.artifact_id)
        {
            return Err(invalid_manifest(format!(
                "duplicate artifact ID {}",
                record.artifact_id
            )));
        }

        self.artifacts.push(record);
        self.artifacts
            .sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));

        Ok(())
    }

    /// Add a dependency from one artifact to another.
    ///
    /// Both artifacts must already exist in the manifest. Duplicate
    /// dependencies are rejected rather than silently ignored.
    pub fn add_dependency(
        &mut self,
        artifact_id: &ArtifactId,
        dependency_id: &ArtifactId,
    ) -> BackendResult<()> {
        self.ensure_current_schema()?;

        if artifact_id == dependency_id {
            return Err(invalid_manifest(
                "an artifact cannot depend on itself",
            ));
        }

        let artifact_key = artifact_id.to_string();
        let dependency_key = dependency_id.to_string();

        if !self
            .artifacts
            .iter()
            .any(|entry| entry.artifact_id == artifact_key)
        {
            return Err(invalid_manifest(format!(
                "dependency owner {artifact_key} is not in the manifest"
            )));
        }

        if !self
            .artifacts
            .iter()
            .any(|entry| entry.artifact_id == dependency_key)
        {
            return Err(invalid_manifest(format!(
                "dependency target {dependency_key} is not in the manifest"
            )));
        }

        let entry = self
            .artifacts
            .iter_mut()
            .find(|entry| entry.artifact_id == artifact_key)
            .ok_or_else(|| invalid_manifest("dependency owner disappeared"))?;

        if entry.dependencies.iter().any(|value| value == &dependency_key) {
            return Err(invalid_manifest(format!(
                "artifact {artifact_key} already lists dependency {dependency_key}"
            )));
        }

        entry.dependencies.push(dependency_key);
        entry.dependencies.sort();

        Ok(())
    }

    /// Validate the schema, metadata, artifact records, dependency references,
    /// and dependency graph.
    ///
    /// This validates manifest consistency, not artifact-format correctness
    /// or source-program semantic equivalence.
    pub fn validate(&self) -> BackendResult<()> {
        self.ensure_current_schema()?;

        validate_optional_text("build ID", self.build_id.as_deref())?;
        validate_optional_text("producer", self.producer.as_deref())?;
        validate_optional_text("producer version", self.producer_version.as_deref())?;

        for (key, value) in &self.metadata {
            if key.trim().is_empty() {
                return Err(invalid_manifest("metadata keys must not be empty"));
            }

            if value.trim().is_empty() {
                return Err(invalid_manifest(format!(
                    "metadata value for key {key:?} must not be empty"
                )));
            }
        }

        let mut ids = BTreeSet::new();
        let mut previous_id: Option<&str> = None;

        for artifact in &self.artifacts {
            artifact.validate()?;

            if !ids.insert(artifact.artifact_id.as_str()) {
                return Err(invalid_manifest(format!(
                    "duplicate artifact ID {}",
                    artifact.artifact_id
                )));
            }

            if previous_id.is_some_and(|previous| previous >= artifact.artifact_id.as_str()) {
                return Err(invalid_manifest(
                    "artifact records must be strictly sorted by canonical artifact ID",
                ));
            }

            previous_id = Some(artifact.artifact_id.as_str());
        }

        // Every dependency must resolve inside this manifest.
        for artifact in &self.artifacts {
            for dependency in &artifact.dependencies {
                if !ids.contains(dependency.as_str()) {
                    return Err(invalid_manifest(format!(
                        "artifact {} references missing dependency {}",
                        artifact.artifact_id, dependency
                    )));
                }
            }
        }

        self.validate_dependency_acyclicity()?;
        Ok(())
    }

    /// Serialize the validated manifest as deterministic compact JSON.
    ///
    /// The same validated in-memory manifest produces the same JSON bytes
    /// with the same serde implementation and schema version.
    pub fn to_json(&self) -> BackendResult<String> {
        self.validate()?;

        serde_json::to_string(self).map_err(|error| {
            invalid_manifest(format!("failed to serialize manifest JSON: {error}"))
        })
    }

    /// Serialize the validated manifest as deterministic pretty-printed JSON.
    pub fn to_pretty_json(&self) -> BackendResult<String> {
        self.validate()?;

        serde_json::to_string_pretty(self).map_err(|error| {
            invalid_manifest(format!(
                "failed to serialize pretty manifest JSON: {error}"
            ))
        })
    }

    /// Parse a manifest from JSON and validate its complete structure.
    ///
    /// Unknown schema versions are rejected. The caller should not treat
    /// parse success as proof that referenced payload files exist.
    pub fn from_json(json: &str) -> BackendResult<Self> {
        let manifest: Self = serde_json::from_str(json).map_err(|error| {
            invalid_manifest(format!("failed to parse manifest JSON: {error}"))
        })?;

        manifest.validate()?;
        Ok(manifest)
    }

    /// Compute a SHA-256 digest of the canonical compact JSON representation.
    ///
    /// This is a fingerprint of the manifest bytes, not a digital signature.
    pub fn sha256_digest(&self) -> BackendResult<String> {
        let serialized = self.to_json()?;
        let digest = Sha256::digest(serialized.as_bytes());

        Ok(format!("sha256:{}", hex::encode(digest)))
    }

    /// Return the number of artifact records in this manifest.
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Return whether this manifest contains no artifact records.
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    fn ensure_current_schema(&self) -> BackendResult<()> {
        if self.schema != MANIFEST_SCHEMA_ID {
            return Err(invalid_manifest(format!(
                "unsupported manifest schema {:?}",
                self.schema
            )));
        }

        if self.schema_version != MANIFEST_SCHEMA_VERSION {
            return Err(invalid_manifest(format!(
                "unsupported manifest schema version {}; this implementation supports version {}",
                self.schema_version, MANIFEST_SCHEMA_VERSION
            )));
        }

        Ok(())
    }

    /// Validate that the artifact dependency graph is acyclic.
    ///
    /// This uses an iterative topological traversal to avoid recursion-depth
    /// dependence on the number of artifacts.
    fn validate_dependency_acyclicity(&self) -> BackendResult<()> {
        let mut indegree: BTreeMap<&str, usize> = self
            .artifacts
            .iter()
            .map(|artifact| (artifact.artifact_id.as_str(), 0))
            .collect();

        let mut dependents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

        // If A depends on B, B must be emitted/available before A.
        for artifact in &self.artifacts {
            for dependency in &artifact.dependencies {
                let degree = indegree.get_mut(artifact.artifact_id.as_str()).ok_or_else(|| {
                    invalid_manifest("artifact missing from dependency index")
                })?;

                *degree = degree.checked_add(1).ok_or_else(|| {
                    invalid_manifest("dependency count overflow")
                })?;

                dependents
                    .entry(dependency.as_str())
                    .or_default()
                    .push(artifact.artifact_id.as_str());
            }
        }

        let mut ready: VecDeque<&str> = indegree
            .iter()
            .filter_map(|(artifact_id, degree)| (*degree == 0).then_some(*artifact_id))
            .collect();

        let mut visited = 0usize;

        while let Some(artifact_id) = ready.pop_front() {
            visited = visited.checked_add(1).ok_or_else(|| {
                invalid_manifest("dependency traversal count overflow")
            })?;

            if let Some(children) = dependents.get(artifact_id) {
                for child in children {
                    let degree = indegree.get_mut(child).ok_or_else(|| {
                        invalid_manifest("dependent artifact missing from dependency index")
                    })?;

                    *degree = degree.checked_sub(1).ok_or_else(|| {
                        invalid_manifest("invalid dependency graph indegree")
                    })?;

                    if *degree == 0 {
                        ready.push_back(child);
                    }
                }
            }
        }

        if visited != self.artifacts.len() {
            return Err(invalid_manifest(
                "artifact dependency graph contains a cycle",
            ));
        }

        Ok(())
    }
}

fn validate_optional_text(field: &str, value: Option<&str>) -> BackendResult<()> {
    if value.is_some_and(|value| value.trim().is_empty()) {
        return Err(invalid_manifest(format!(
            "{field} must not be empty when supplied"
        )));
    }

    Ok(())
}

fn validate_sha256(value: &str) -> BackendResult<()> {
    let Some(hex_digest) = value.strip_prefix("sha256:") else {
        return Err(invalid_manifest(
            "artifact digest must use the sha256: prefix",
        ));
    };

    if hex_digest.len() != 64
        || !hex_digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid_manifest(
            "artifact SHA-256 digest must contain 64 lowercase hexadecimal digits",
        ));
    }

    Ok(())
}

fn invalid_manifest(message: impl Into<String>) -> BackendError {
    BackendError::invalid_artifact(format!("invalid artifact manifest: {}", message.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::artifact::{ArtifactPayload, OutputArtifact};
    use crate::backend::ids::{ArtifactId, IdNamespace};

    fn artifact_id(value: u128) -> ArtifactId {
        ArtifactId::new(
            IdNamespace::new("zamani.artifact").expect("valid namespace"),
            value,
        )
        .expect("valid artifact ID")
    }

    fn sample_artifact(value: u128, format: &str) -> OutputArtifact {
        OutputArtifact::new(
            artifact_id(value),
            "test-target",
            format,
            ArtifactPayload::text(format!("artifact-{value}")),
        )
        .expect("valid artifact")
    }

    #[test]
    fn builds_and_round_trips_a_manifest() {
        let artifacts = vec![
            sample_artifact(2, "llvm-ir"),
            sample_artifact(1, "native-assembly"),
        ];

        let manifest = ArtifactManifest::from_artifacts(&artifacts)
            .expect("manifest should build");

        assert_eq!(manifest.artifact_count(), 2);
        assert!(manifest.artifacts[0].artifact_id < manifest.artifacts[1].artifact_id);

        let json = manifest.to_json().expect("serialization should succeed");
        let parsed =
            ArtifactManifest::from_json(&json).expect("manifest should parse");

        assert_eq!(manifest, parsed);
    }

    #[test]
    fn records_exact_payload_digest() {
        let artifact = sample_artifact(1, "llvm-ir");
        let expected = artifact.payload.sha256_digest();

        let manifest =
            ArtifactManifest::from_artifacts(&[artifact]).expect("manifest should build");

        assert_eq!(manifest.artifacts[0].content_digest, expected);
        assert!(manifest.artifacts[0].content_digest.starts_with("sha256:"));
    }

    #[test]
    fn rejects_duplicate_artifact_ids() {
        let first = sample_artifact(1, "llvm-ir");
        let second = sample_artifact(1, "wasm");

        assert!(ArtifactManifest::from_artifacts(&[first, second]).is_err());
    }

    #[test]
    fn rejects_unknown_schema_version() {
        let mut manifest = ArtifactManifest::new();
        manifest.schema_version = MANIFEST_SCHEMA_VERSION + 1;

        assert!(manifest.validate().is_err());
        assert!(manifest.to_json().is_err());
    }

    #[test]
    fn rejects_unknown_schema_identifier() {
        let mut manifest = ArtifactManifest::new();
        manifest.schema = "vendor.other-manifest".to_owned();

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn dependency_references_must_resolve() {
        let mut manifest =
            ArtifactManifest::from_artifacts(&[sample_artifact(1, "llvm-ir")])
                .expect("manifest should build");

        manifest.artifacts[0].dependencies.push(artifact_id(99).to_string());

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn accepts_valid_dependency_and_rejects_cycles() {
        let first = sample_artifact(1, "llvm-ir");
        let second = sample_artifact(2, "object");

        let mut manifest =
            ArtifactManifest::from_artifacts(&[first, second])
                .expect("manifest should build");

        manifest
            .add_dependency(&artifact_id(2), &artifact_id(1))
            .expect("dependency should be accepted");
        manifest.validate().expect("acyclic dependency should validate");

        manifest
            .add_dependency(&artifact_id(1), &artifact_id(2))
            .expect("individual edge can be added before whole-graph validation");

        assert!(manifest.validate().is_err());
        assert!(manifest.to_json().is_err());
    }

    #[test]
    fn rejects_self_dependency() {
        let mut manifest =
            ArtifactManifest::from_artifacts(&[sample_artifact(1, "llvm-ir")])
                .expect("manifest should build");

        assert!(
            manifest
                .add_dependency(&artifact_id(1), &artifact_id(1))
                .is_err()
        );
    }

    #[test]
    fn serializes_deterministically_for_the_same_manifest() {
        let manifest = ArtifactManifest::from_artifacts(&[
            sample_artifact(3, "wasm"),
            sample_artifact(1, "llvm-ir"),
            sample_artifact(2, "qasm"),
        ])
        .expect("manifest should build");

        assert_eq!(
            manifest.to_json().expect("serialize"),
            manifest.to_json().expect("serialize again")
        );

        assert_eq!(
            manifest.sha256_digest().expect("digest"),
            manifest.sha256_digest().expect("same digest")
        );
    }

    #[test]
    fn rejects_noncanonical_digest_text() {
        let mut manifest =
            ArtifactManifest::from_artifacts(&[sample_artifact(1, "llvm-ir")])
                .expect("manifest should build");

        manifest.artifacts[0].content_digest = "sha256:ABC".to_owned();

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn empty_manifest_is_valid() {
        let manifest = ArtifactManifest::new();

        manifest.validate().expect("empty manifest is valid");
        assert!(manifest.is_empty());
    }
}
