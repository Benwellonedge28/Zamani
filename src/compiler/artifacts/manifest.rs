
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Artifact Manifest
//!
//! Canonical path: src/compiler/artifacts/manifest.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Define the canonical manifest that groups compiler artifacts and records
//! their relationships, roots, metadata, and provenance.
//!
//! An ArtifactManifest describes a collection of artifacts. It does not
//! duplicate their payloads, content digests, descriptors, or provenance.
//!
//! OWNERSHIP
//!
//! OWNS:
//! - Manifest identity and manifest schema version.
//! - The collection of canonical Artifact values.
//! - Artifact dependency relationships.
//! - Explicit root/entry artifact identities.
//! - Manifest-level metadata.
//! - Manifest-level provenance.
//! - Structural and dependency-graph validation.
//!
//! DOES NOT OWN:
//! - Individual artifact representation.
//! - Artifact payload storage or retrieval.
//! - Artifact content digest computation.
//! - Stable wire serialization or deserialization.
//! - Portability classification.
//! - Semantic or IR verification.
//! - Validation attestations or trust decisions.
//! - Backend selection or code generation.
//! - Symbol resolution, linking, or relocation.
//! - Runtime execution or deployment.
//!
//! INTEGRATION
//!
//! artifact.rs:
//!     Supplies Artifact, ArtifactId, ArtifactMetadata, and
//!     ArtifactProvenance.
//!
//! metadata.rs:
//!     May provide manifest metadata conventions, but must reuse
//!     ArtifactMetadata rather than introduce a competing metadata type.
//!
//! portability.rs:
//!     Evaluates portability of the artifacts referenced by this manifest.
//!
//! validation.rs:
//!     Validates artifact contents and records validation evidence.
//!
//! serialization.rs:
//!     Owns versioned manifest encoding and decoding.
//!
//! linker:
//!     Consumes dependency relationships and artifact references.
//!
//! backend:
//!     Produces Artifact values that can be inserted into a manifest.
//!
//! runtime:
//!     Resolves external payload references and executes valid realizations.
//!
//! GRAMMAR BOUNDARY
//!
//! The Zamani grammar defines source syntax. The semantic model defines
//! program meaning. The canonical IRs define computational representations.
//! This manifest describes compilation products after those stages.
//!
//! No grammar production or source-language semantic rule is defined here.
//!
//! SCALABILITY
//!
//! No fixed maximum is imposed on:
//! - Artifact count.
//! - Dependency count.
//! - Manifest roots.
//! - Identifier length.
//! - Computing domains.
//! - Target architectures.
//! - Hardware resources.
//!
//! Collections grow according to available memory and allocator capacity.
//! Operational limits, if required, must be supplied by an external policy.
//!
//! DEPENDENCY DIRECTION
//!
//! artifact.rs
//!     |
//!     v
//! manifest.rs
//!     |
//!     +--> validation.rs
//!     +--> portability.rs
//!     +--> serialization.rs
//!     +--> linker integration
//!
//! This module does not depend on those downstream modules.
//!
//! DETERMINISM
//!
//! Artifacts are indexed by ArtifactId in a BTreeMap.
//! Dependencies and roots use BTreeSet.
//! Iteration order is therefore stable for identical manifest contents.
//!
//! Deterministic collection order does not imply deterministic backend
//! output or deterministic serialization. Those contracts belong to
//! their respective owners.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use super::artifact::{
    Artifact,
    ArtifactError,
    ArtifactId,
    ArtifactMetadata,
    ArtifactProvenance,
};

// ============================================================================
// Manifest constants and versioning
// ============================================================================

/// Stable manifest schema identifier.
///
/// This identifies the manifest contract, not a particular manifest.
pub const ARTIFACT_MANIFEST_SCHEMA_ID: &str =
    "zamani.compiler.artifact-manifest";

/// Initial manifest schema major version.
pub const ARTIFACT_MANIFEST_SCHEMA_MAJOR: u32 = 1;

/// Initial manifest schema minor version.
pub const ARTIFACT_MANIFEST_SCHEMA_MINOR: u32 = 0;

/// Initial manifest schema patch version.
pub const ARTIFACT_MANIFEST_SCHEMA_PATCH: u32 = 0;

/// Current artifact manifest schema version.
///
/// This version is independent of:
/// - Zamani language version.
/// - Compiler release version.
/// - Artifact model API version.
/// - ZUIR format version.
/// - Individual domain IR versions.
/// - Backend and runtime versions.
pub const CURRENT_ARTIFACT_MANIFEST_VERSION: ManifestVersion =
    ManifestVersion::new(
        ARTIFACT_MANIFEST_SCHEMA_MAJOR,
        ARTIFACT_MANIFEST_SCHEMA_MINOR,
        ARTIFACT_MANIFEST_SCHEMA_PATCH,
    );

// ============================================================================
// Manifest version
// ============================================================================

/// Version of the artifact manifest schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManifestVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ManifestVersion {
    /// Constructs a manifest schema version.
    pub const fn new(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor version.
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch version.
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns the current manifest schema version.
    pub const fn current() -> Self {
        CURRENT_ARTIFACT_MANIFEST_VERSION
    }

    /// Returns whether both versions have the same major version.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether the versions are identical.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Conservative reader compatibility check.
    ///
    /// A reader supports a writer when:
    /// - Both versions have the same major version.
    /// - The writer version is not newer than the reader version.
    ///
    /// This is a version-level check only. It does not prove that every
    /// field, extension, artifact format, or semantic feature is supported.
    pub const fn supports(self, writer: Self) -> bool {
        self.major == writer.major
            && (
                writer.minor < self.minor
                    || (
                        writer.minor == self.minor
                            && writer.patch <= self.patch
                    )
            )
    }
}

impl Default for ManifestVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for ManifestVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch,
        )
    }
}

// ============================================================================
// Manifest identity
// ============================================================================

/// Stable identity of an artifact manifest.
///
/// The identifier is opaque to this module. A caller or identity provider
/// is responsible for choosing collision-resistant identities.
///
/// No arbitrary identifier-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManifestId(String);

impl ManifestId {
    /// Creates a validated manifest identity.
    pub fn new(value: impl Into<String>) -> Result<Self, ManifestError> {
        let value = value.into();

        validate_identifier(&value, "manifest identity")?;

        Ok(Self(value))
    }

    /// Returns the identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identity.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ManifestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<String> for ManifestId {
    type Error = ManifestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ManifestId {
    type Error = ManifestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// Identifier validation
// ============================================================================

/// Validates an identifier without imposing an arbitrary length limit.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ManifestError> {
    if value.is_empty() {
        return Err(ManifestError::InvalidIdentifier {
            field,
            reason: "identifier cannot be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(ManifestError::InvalidIdentifier {
            field,
            reason: "leading or trailing whitespace is not permitted"
                .to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ManifestError::InvalidIdentifier {
            field,
            reason: "control characters are not permitted".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// Dependency kind
// ============================================================================

/// Extensible relationship category between two artifacts.
///
/// This is deliberately a textual identifier rather than a closed enum.
///
/// Examples include:
/// - `compile-time`
/** - `link-time`
/// - `runtime`
/// - `interface`
/** - `generated-from`
/// - `resource`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyKind(String);

impl DependencyKind {
    /// Creates a dependency kind.
    pub fn new(value: impl Into<String>) -> Result<Self, ManifestError> {
        let value = value.into();

        validate_identifier(&value, "dependency kind")?;

        Ok(Self(value))
    }

    /// Returns the dependency kind.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for DependencyKind {
    type Error = ManifestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// Artifact dependency
// ============================================================================

/// A directed dependency relationship between two artifacts.
///
/// `dependent` is the artifact that requires another artifact.
/// `prerequisite` is the artifact it requires.
///
/// For example:
///
/// `application` depends on `library`.
///
/// The relationship is represented as:
///
/// dependent   = application
/// prerequisite = library
///
/// This direction is explicit so consumers do not have to infer it.
///
/// The manifest records relationships. It does not perform linking,
/// symbol resolution, relocation, or runtime loading.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactDependency {
    dependent: ArtifactId,
    prerequisite: ArtifactId,
    kind: DependencyKind,
}

impl ArtifactDependency {
    /// Creates a dependency relationship.
    pub fn new(
        dependent: ArtifactId,
        prerequisite: ArtifactId,
        kind: DependencyKind,
    ) -> Self {
        Self {
            dependent,
            prerequisite,
            kind,
        }
    }

    /// Returns the dependent artifact identity.
    pub fn dependent(&self) -> &ArtifactId {
        &self.dependent
    }

    /// Returns the prerequisite artifact identity.
    pub fn prerequisite(&self) -> &ArtifactId {
        &self.prerequisite
    }

    /// Returns the relationship category.
    pub fn kind(&self) -> &DependencyKind {
        &self.kind
    }
}

// ============================================================================
// Manifest errors
// ============================================================================

/// Errors produced by manifest construction and structural validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ManifestError {
    InvalidIdentifier {
        field: &'static str,
        reason: String,
    },

    DuplicateArtifact {
        artifact: ArtifactId,
    },

    MissingArtifact {
        artifact: ArtifactId,
    },

    MissingDependencyDependent {
        artifact: ArtifactId,
    },

    MissingDependencyPrerequisite {
        artifact: ArtifactId,
    },

    MissingRootArtifact {
        artifact: ArtifactId,
    },

    DependencyCycle {
        remaining_artifacts: Vec<ArtifactId>,
    },

    SelfDependency {
        artifact: ArtifactId,
    },

    Artifact {
        source: ArtifactError,
    },

    CountOverflow,
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason } => {
                write!(f, "invalid {field}: {reason}")
            }

            Self::DuplicateArtifact { artifact } => {
                write!(
                    f,
                    "manifest already contains artifact '{artifact}'"
                )
            }

            Self::MissingArtifact { artifact } => {
                write!(
                    f,
                    "manifest does not contain artifact '{artifact}'"
                )
            }

            Self::MissingDependencyDependent { artifact } => {
                write!(
                    f,
                    "dependency refers to missing dependent artifact \
                     '{artifact}'"
                )
            }

            Self::MissingDependencyPrerequisite { artifact } => {
                write!(
                    f,
                    "dependency refers to missing prerequisite artifact \
                     '{artifact}'"
                )
            }

            Self::MissingRootArtifact { artifact } => {
                write!(
                    f,
                    "manifest root refers to missing artifact '{artifact}'"
                )
            }

            Self::DependencyCycle { remaining_artifacts } => {
                write!(
                    f,
                    "artifact dependency graph contains a cycle involving \
                     {} artifact(s)",
                    remaining_artifacts.len(),
                )
            }

            Self::SelfDependency { artifact } => {
                write!(
                    f,
                    "artifact '{artifact}' cannot depend on itself"
                )
            }

            Self::Artifact { source } => {
                write!(f, "artifact error: {source}")
            }

            Self::CountOverflow => {
                write!(f, "manifest graph count overflow")
            }
        }
    }
}

impl Error for ManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Artifact { source } => Some(source),
            _ => None,
        }
    }
}

impl From<ArtifactError> for ManifestError {
    fn from(source: ArtifactError) -> Self {
        Self::Artifact { source }
    }
}

/// Result type for manifest operations.
pub type ManifestResult<T> = Result<T, ManifestError>;

// ============================================================================
// Canonical artifact manifest
// ============================================================================

/// Canonical collection of artifacts and their dependency relationships.
///
/// Artifacts are stored by identity. This prevents duplicate artifact IDs
/// and provides deterministic lookup and iteration.
///
/// The manifest owns the collection, not the individual artifact model.
/// Every artifact retains the canonical representation defined in
/// artifact.rs.
///
/// A manifest can be structurally valid while containing artifacts that
/// have not yet passed content, semantic, security, or portability checks.
/// Those checks belong to their respective subsystems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactManifest {
    id: ManifestId,
    version: ManifestVersion,

    artifacts: BTreeMap<ArtifactId, Artifact>,

    dependencies: BTreeSet<ArtifactDependency>,

    roots: BTreeSet<ArtifactId>,

    metadata: ArtifactMetadata,

    provenance: ArtifactProvenance,
}

impl ArtifactManifest {
    /// Creates an empty manifest using the current schema version.
    pub fn new(
        id: ManifestId,
        provenance: ArtifactProvenance,
    ) -> Self {
        Self::with_version(
            id,
            ManifestVersion::current(),
            provenance,
        )
    }

    /// Creates an empty manifest with an explicitly declared schema version.
    ///
    /// This does not prove compatibility with any reader or writer.
    pub fn with_version(
        id: ManifestId,
        version: ManifestVersion,
        provenance: ArtifactProvenance,
    ) -> Self {
        Self {
            id,
            version,
            artifacts: BTreeMap::new(),
            dependencies: BTreeSet::new(),
            roots: BTreeSet::new(),
            metadata: ArtifactMetadata::new(),
            provenance,
        }
    }

    // ------------------------------------------------------------------------
    // Identity and version
    // ------------------------------------------------------------------------

    /// Returns the manifest identity.
    pub fn id(&self) -> &ManifestId {
        &self.id
    }

    /// Returns the manifest schema version.
    pub const fn version(&self) -> ManifestVersion {
        self.version
    }

    /// Returns the stable schema identifier.
    pub const fn schema_id(&self) -> &'static str {
        ARTIFACT_MANIFEST_SCHEMA_ID
    }

    // ------------------------------------------------------------------------
    // Metadata and provenance
    // ------------------------------------------------------------------------

    /// Returns manifest-level metadata.
    pub fn metadata(&self) -> &ArtifactMetadata {
        &self.metadata
    }

    /// Returns mutable manifest-level metadata.
    ///
    /// Metadata validation is performed by ArtifactMetadata itself.
    pub fn metadata_mut(&mut self) -> &mut ArtifactMetadata {
        &mut self.metadata
    }

    /// Returns manifest provenance.
    pub fn provenance(&self) -> &ArtifactProvenance {
        &self.provenance
    }

    // ------------------------------------------------------------------------
    // Artifact collection
    // ------------------------------------------------------------------------

    /// Inserts an artifact.
    ///
    /// Duplicate identities are rejected. This method never silently
    /// replaces an existing artifact.
    pub fn insert_artifact(
        &mut self,
        artifact: Artifact,
    ) -> ManifestResult<()> {
        let id = artifact.id().clone();

        if self.artifacts.contains_key(&id) {
            return Err(ManifestError::DuplicateArtifact {
                artifact: id,
            });
        }

        self.artifacts.insert(id, artifact);

        Ok(())
    }

    /// Returns an artifact by identity.
    pub fn artifact(
        &self,
        id: &ArtifactId,
    ) -> Option<&Artifact> {
        self.artifacts.get(id)
    }

    /// Returns the number of artifacts.
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Returns whether the manifest contains no artifacts.
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    /// Iterates over artifacts in deterministic identity order.
    pub fn artifacts(
        &self,
    ) -> impl Iterator<Item = (&ArtifactId, &Artifact)> {
        self.artifacts.iter()
    }

    /// Returns all artifact identities in deterministic order.
    pub fn artifact_ids(
        &self,
    ) -> impl Iterator<Item = &ArtifactId> {
        self.artifacts.keys()
    }

    // ------------------------------------------------------------------------
    // Dependencies
    // ------------------------------------------------------------------------

    /// Adds a dependency relationship.
    ///
    /// References are checked by `validate()`. This permits callers to
    /// construct a manifest in any insertion order.
    ///
    /// The returned boolean is false when the exact relationship already
    /// exists. Duplicate relationships are therefore idempotent.
    pub fn add_dependency(
        &mut self,
        dependency: ArtifactDependency,
    ) -> bool {
        self.dependencies.insert(dependency)
    }

    /// Adds a dependency using explicit artifact identities and kind.
    pub fn add_dependency_between(
        &mut self,
        dependent: ArtifactId,
        prerequisite: ArtifactId,
        kind: DependencyKind,
    ) -> bool {
        self.add_dependency(
            ArtifactDependency::new(
                dependent,
                prerequisite,
                kind,
            ),
        )
    }

    /// Returns the dependency relationships in deterministic order.
    pub fn dependencies(
        &self,
    ) -> impl Iterator<Item = &ArtifactDependency> {
        self.dependencies.iter()
    }

    /// Returns the number of dependency relationships.
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns the direct prerequisites of an artifact.
    ///
    /// The returned iterator follows deterministic artifact identity and
    /// dependency-kind ordering.
    pub fn prerequisites(
        &self,
        artifact: &ArtifactId,
    ) -> impl Iterator<Item = &ArtifactDependency> {
        self.dependencies
            .iter()
            .filter(move |dependency| {
                dependency.dependent() == artifact
            })
    }

    /// Returns the artifacts that directly depend on the supplied artifact.
    pub fn dependents(
        &self,
        artifact: &ArtifactId,
    ) -> impl Iterator<Item = &ArtifactDependency> {
        self.dependencies
            .iter()
            .filter(move |dependency| {
                dependency.prerequisite() == artifact
            })
    }

    // ------------------------------------------------------------------------
    // Root artifacts
    // ------------------------------------------------------------------------

    /// Adds an explicit root or entry artifact.
    ///
    /// A root is an artifact from which a consumer may begin a particular
    /// operation, such as deployment, packaging, or realization.
    ///
    /// Root status does not mean that the artifact is executable.
    pub fn add_root(
        &mut self,
        artifact: ArtifactId,
    ) -> bool {
        self.roots.insert(artifact)
    }

    /// Removes a root declaration.
    pub fn remove_root(
        &mut self,
        artifact: &ArtifactId,
    ) -> bool {
        self.roots.remove(artifact)
    }

    /// Returns root artifact identities in deterministic order.
    pub fn roots(
        &self,
    ) -> impl Iterator<Item = &ArtifactId> {
        self.roots.iter()
    }

    /// Returns the number of declared roots.
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    // ------------------------------------------------------------------------
    // Structural validation
    // ------------------------------------------------------------------------

    /// Validates all artifact references and the dependency graph.
    ///
    /// This checks manifest structure only. It does not validate artifact
    /// payload bytes, signatures, semantics, portability, backend support,
    /// or runtime compatibility.
    pub fn validate(&self) -> ManifestResult<()> {
        self.validate_dependency_references()?;
        self.validate_roots()?;
        self.validate_acyclic_dependencies()?;

        Ok(())
    }

    /// Checks that every dependency endpoint exists.
    fn validate_dependency_references(&self) -> ManifestResult<()> {
        for dependency in &self.dependencies {
            if dependency.dependent() == dependency.prerequisite() {
                return Err(ManifestError::SelfDependency {
                    artifact: dependency.dependent().clone(),
                });
            }

            if !self.artifacts.contains_key(dependency.dependent()) {
                return Err(
                    ManifestError::MissingDependencyDependent {
                        artifact: dependency.dependent().clone(),
                    },
                );
            }

            if !self.artifacts.contains_key(dependency.prerequisite()) {
                return Err(
                    ManifestError::MissingDependencyPrerequisite {
                        artifact: dependency.prerequisite().clone(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Checks that every declared root exists.
    fn validate_roots(&self) -> ManifestResult<()> {
        for root in &self.roots {
            if !self.artifacts.contains_key(root) {
                return Err(ManifestError::MissingRootArtifact {
                    artifact: root.clone(),
                });
            }
        }

        Ok(())
    }

    /// Detects cycles using Kahn's topological algorithm.
    ///
    /// This implementation is iterative rather than recursive, avoiding
    /// dependency-graph traversal that consumes call-stack depth.
    ///
    /// Edges are interpreted as:
    ///
    /// prerequisite -> dependent
    ///
    /// The algorithm is O(V + E), where V is the artifact count and E is
    /// the dependency relationship count, excluding ordered-map/set costs.
    fn validate_acyclic_dependencies(&self) -> ManifestResult<()> {
        let mut indegree: BTreeMap<ArtifactId, usize> = self
            .artifacts
            .keys()
            .cloned()
            .map(|id| (id, 0))
            .collect();

        let mut dependents_by_prerequisite:
            BTreeMap<ArtifactId, Vec<ArtifactId>> =
                BTreeMap::new();

        for dependency in &self.dependencies {
            let dependent = dependency.dependent();
            let prerequisite = dependency.prerequisite();

            let degree = indegree
                .get_mut(dependent)
                .ok_or_else(|| {
                    ManifestError::MissingDependencyDependent {
                        artifact: dependent.clone(),
                    }
                })?;

            *degree = degree
                .checked_add(1)
                .ok_or(ManifestError::CountOverflow)?;

            dependents_by_prerequisite
                .entry(prerequisite.clone())
                .or_default()
                .push(dependent.clone());
        }

        let mut ready = VecDeque::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.push_back(id.clone());
            }
        }

        let mut processed = 0usize;

        while let Some(prerequisite) = ready.pop_front() {
            processed = processed
                .checked_add(1)
                .ok_or(ManifestError::CountOverflow)?;

            if let Some(dependents) =
                dependents_by_prerequisite.get(&prerequisite)
            {
                for dependent in dependents {
                    let degree = indegree
                        .get_mut(dependent)
                        .ok_or_else(|| {
                            ManifestError::MissingDependencyDependent {
                                artifact: dependent.clone(),
                            }
                        })?;

                    *degree = degree
                        .checked_sub(1)
                        .ok_or(ManifestError::CountOverflow)?;

                    if *degree == 0 {
                        ready.push_back(dependent.clone());
                    }
                }
            }
        }

        if processed != self.artifacts.len() {
            let remaining_artifacts = indegree
                .into_iter()
                .filter_map(|(id, degree)| {
                    (degree != 0).then_some(id)
                })
                .collect();

            return Err(ManifestError::DependencyCycle {
                remaining_artifacts,
            });
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Dependency ordering
    // ------------------------------------------------------------------------

    /// Returns a topological ordering of all artifacts.
    ///
    /// Prerequisites appear before their dependents.
    ///
    /// This is a deterministic order for identical manifest contents.
    /// Among currently independent artifacts, identity ordering is used.
    ///
    /// This method does not schedule execution or link artifacts.
    pub fn topological_order(
        &self,
    ) -> ManifestResult<Vec<ArtifactId>> {
        self.validate_dependency_references()?;
        self.validate_roots()?;

        let mut indegree: BTreeMap<ArtifactId, usize> = self
            .artifacts
            .keys()
            .cloned()
            .map(|id| (id, 0))
            .collect();

        let mut dependents_by_prerequisite:
            BTreeMap<ArtifactId, BTreeSet<ArtifactId>> =
                BTreeMap::new();

        for dependency in &self.dependencies {
            let degree = indegree
                .get_mut(dependency.dependent())
                .ok_or_else(|| {
                    ManifestError::MissingDependencyDependent {
                        artifact: dependency.dependent().clone(),
                    }
                })?;

            *degree = degree
                .checked_add(1)
                .ok_or(ManifestError::CountOverflow)?;

            dependents_by_prerequisite
                .entry(dependency.prerequisite().clone())
                .or_default()
                .insert(dependency.dependent().clone());
        }

        // BTreeSet makes selection deterministic without imposing a
        // fixed-size priority queue or a hardware-related ceiling.
        let mut ready: BTreeSet<ArtifactId> = indegree
            .iter()
            .filter_map(|(id, degree)| {
                (*degree == 0).then_some(id.clone())
            })
            .collect();

        let mut ordered = Vec::with_capacity(self.artifacts.len());

        while let Some(next) = ready.pop_first() {
            ordered.push(next.clone());

            if let Some(dependents) =
                dependents_by_prerequisite.get(&next)
            {
                for dependent in dependents {
                    let degree = indegree
                        .get_mut(dependent)
                        .ok_or_else(|| {
                            ManifestError::MissingDependencyDependent {
                                artifact: dependent.clone(),
                            }
                        })?;

                    *degree = degree
                        .checked_sub(1)
                        .ok_or(ManifestError::CountOverflow)?;

                    if *degree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if ordered.len() != self.artifacts.len() {
            let remaining_artifacts = indegree
                .into_iter()
                .filter_map(|(id, degree)| {
                    (degree != 0).then_some(id)
                })
                .collect();

            return Err(ManifestError::DependencyCycle {
                remaining_artifacts,
            });
        }

        Ok(ordered)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::artifacts::artifact::{
        ArtifactDescriptor,
        ArtifactFormatId,
        ArtifactKindId,
        ArtifactProducerId,
    };

    fn artifact_id(value: &str) -> ArtifactId {
        ArtifactId::new(value).expect("valid artifact identity")
    }

    fn manifest_id(value: &str) -> ManifestId {
        ManifestId::new(value).expect("valid manifest identity")
    }

    fn provenance() -> ArtifactProvenance {
        ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.compiler")
                .expect("valid producer identity"),
        )
    }

    fn artifact(value: &str) -> Artifact {
        let descriptor = ArtifactDescriptor::new(
            artifact_id(value),
            ArtifactKindId::new("portable-ir")
                .expect("valid artifact kind"),
            ArtifactFormatId::new("application/x-zamani-zuir")
                .expect("valid artifact format"),
            provenance(),
        );

        Artifact::new_inline(
            descriptor,
            format!("payload:{value}").into_bytes(),
        )
    }

    fn manifest() -> ArtifactManifest {
        ArtifactManifest::new(
            manifest_id("manifest.test"),
            provenance(),
        )
    }

    fn dependency(
        dependent: &str,
        prerequisite: &str,
    ) -> ArtifactDependency {
        ArtifactDependency::new(
            artifact_id(dependent),
            artifact_id(prerequisite),
            DependencyKind::new("compile-time")
                .expect("valid dependency kind"),
        )
    }

    #[test]
    fn creates_manifest_with_current_version() {
        let manifest = manifest();

        assert_eq!(
            manifest.version(),
            CURRENT_ARTIFACT_MANIFEST_VERSION,
        );

        assert_eq!(
            manifest.schema_id(),
            ARTIFACT_MANIFEST_SCHEMA_ID,
        );

        assert!(manifest.is_empty());
    }

    #[test]
    fn manifest_version_is_independent_and_ordered() {
        let older = ManifestVersion::new(1, 0, 0);
        let newer = ManifestVersion::new(1, 1, 0);

        assert!(newer.supports(older));
        assert!(!older.supports(newer));
        assert!(older.same_major(newer));
        assert!(!older.is_exact(newer));
    }

    #[test]
    fn rejects_invalid_manifest_identity() {
        assert!(ManifestId::new("").is_err());
        assert!(ManifestId::new(" leading").is_err());
        assert!(ManifestId::new("invalid\nidentity").is_err());
    }

    #[test]
    fn accepts_long_manifest_identity_without_arbitrary_limit() {
        let value = "m".repeat(16_384);

        let id = ManifestId::new(value.clone())
            .expect("long identity should be accepted");

        assert_eq!(id.as_str(), value);
    }

    #[test]
    fn inserts_and_retrieves_canonical_artifact() {
        let mut manifest = manifest();
        let value = artifact("module.main");

        manifest
            .insert_artifact(value)
            .expect("insertion should succeed");

        assert_eq!(manifest.artifact_count(), 1);

        assert!(
            manifest.artifact(&artifact_id("module.main")).is_some()
        );
    }

    #[test]
    fn rejects_duplicate_artifact_identity() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("module.main"))
            .expect("first insertion");

        let result = manifest.insert_artifact(artifact("module.main"));

        assert!(matches!(
            result,
            Err(ManifestError::DuplicateArtifact { .. })
        ));
    }

    #[test]
    fn permits_future_artifact_kinds() {
        let descriptor = ArtifactDescriptor::new(
            artifact_id("future.artifact"),
            ArtifactKindId::new("future.photonic.computation")
                .expect("future kind is valid"),
            ArtifactFormatId::new("application/x-future-photonic")
                .expect("future format is valid"),
            provenance(),
        );

        let mut manifest = manifest();

        manifest
            .insert_artifact(
                Artifact::new_inline(descriptor, b"future".to_vec()),
            )
            .expect("insertion should succeed");

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn dependency_direction_is_explicit() {
        let relationship = dependency(
            "application",
            "library",
        );

        assert_eq!(
            relationship.dependent(),
            &artifact_id("application"),
        );

        assert_eq!(
            relationship.prerequisite(),
            &artifact_id("library"),
        );

        assert_eq!(
            relationship.kind().as_str(),
            "compile-time",
        );
    }

    #[test]
    fn dependencies_can_be_declared_before_artifacts() {
        let mut manifest = manifest();

        manifest.add_dependency(dependency(
            "application",
            "library",
        ));

        manifest
            .insert_artifact(artifact("application"))
            .expect("application insertion");

        manifest
            .insert_artifact(artifact("library"))
            .expect("library insertion");

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn validates_missing_dependency_dependent() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("library"))
            .expect("library insertion");

        manifest.add_dependency(dependency(
            "missing.application",
            "library",
        ));

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::MissingDependencyDependent { .. })
        ));
    }

    #[test]
    fn validates_missing_dependency_prerequisite() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("application"))
            .expect("application insertion");

        manifest.add_dependency(dependency(
            "application",
            "missing.library",
        ));

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::MissingDependencyPrerequisite { .. })
        ));
    }

    #[test]
    fn rejects_self_dependency() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("module.self"))
            .expect("artifact insertion");

        manifest.add_dependency(dependency(
            "module.self",
            "module.self",
        ));

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::SelfDependency { .. })
        ));
    }

    #[test]
    fn rejects_dependency_cycles() {
        let mut manifest = manifest();

        for id in ["a", "b", "c"] {
            manifest
                .insert_artifact(artifact(id))
                .expect("artifact insertion");
        }

        manifest.add_dependency(dependency("a", "b"));
        manifest.add_dependency(dependency("b", "c"));
        manifest.add_dependency(dependency("c", "a"));

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn accepts_acyclic_dependency_graph() {
        let mut manifest = manifest();

        for id in ["application", "library", "core"] {
            manifest
                .insert_artifact(artifact(id))
                .expect("artifact insertion");
        }

        manifest.add_dependency(dependency(
            "application",
            "library",
        ));

        manifest.add_dependency(dependency(
            "library",
            "core",
        ));

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn topological_order_places_prerequisites_first() {
        let mut manifest = manifest();

        for id in ["application", "library", "core"] {
            manifest
                .insert_artifact(artifact(id))
                .expect("artifact insertion");
        }

        manifest.add_dependency(dependency(
            "application",
            "library",
        ));

        manifest.add_dependency(dependency(
            "library",
            "core",
        ));

        let ordered = manifest
            .topological_order()
            .expect("valid dependency order");

        let core = ordered
            .iter()
            .position(|id| id == &artifact_id("core"))
            .expect("core is present");

        let library = ordered
            .iter()
            .position(|id| id == &artifact_id("library"))
            .expect("library is present");

        let application = ordered
            .iter()
            .position(|id| id == &artifact_id("application"))
            .expect("application is present");

        assert!(core < library);
        assert!(library < application);
    }

    #[test]
    fn dependency_insertion_is_idempotent() {
        let mut manifest = manifest();
        let relationship = dependency("application", "library");

        assert!(manifest.add_dependency(relationship.clone()));
        assert!(!manifest.add_dependency(relationship));

        assert_eq!(manifest.dependency_count(), 1);
    }

    #[test]
    fn roots_must_reference_existing_artifacts() {
        let mut manifest = manifest();

        manifest.add_root(artifact_id("missing.root"));

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::MissingRootArtifact { .. })
        ));
    }

    #[test]
    fn valid_roots_are_preserved() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("application"))
            .expect("artifact insertion");

        assert!(manifest.add_root(artifact_id("application")));
        assert!(!manifest.add_root(artifact_id("application")));

        assert!(manifest.validate().is_ok());

        assert_eq!(manifest.root_count(), 1);
    }

    #[test]
    fn root_status_does_not_imply_executability() {
        let mut manifest = manifest();

        manifest
            .insert_artifact(artifact("portable.module"))
            .expect("artifact insertion");

        manifest.add_root(artifact_id("portable.module"));

        // This module deliberately has no executable or portability
        // classification API. Those decisions belong to other modules.
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn independent_artifacts_have_deterministic_order() {
        let mut first = manifest();
        let mut second = manifest();

        for id in ["z", "a", "m"] {
            first
                .insert_artifact(artifact(id))
                .expect("first insertion");
        }

        for id in ["m", "z", "a"] {
            second
                .insert_artifact(artifact(id))
                .expect("second insertion");
        }

        assert_eq!(
            first.artifact_ids().cloned().collect::<Vec<_>>(),
            second.artifact_ids().cloned().collect::<Vec<_>>(),
        );

        assert_eq!(
            first.topological_order().expect("first order"),
            second.topological_order().expect("second order"),
        );
    }

    #[test]
    fn manifest_metadata_uses_existing_metadata_authority() {
        let mut manifest = manifest();

        manifest
            .metadata_mut()
            .insert("build.profile", "portable")
            .expect("metadata insertion");

        assert_eq!(
            manifest.metadata().get("build.profile"),
            Some("portable"),
        );
    }

    #[test]
    fn manifest_preserves_existing_artifact_payload_and_digest() {
        let mut manifest = manifest();
        let original = artifact("module.main");
        let expected_digest = original.digest().clone();

        manifest
            .insert_artifact(original)
            .expect("artifact insertion");

        let stored = manifest
            .artifact(&artifact_id("module.main"))
            .expect("stored artifact");

        assert_eq!(stored.digest(), &expected_digest);
        assert!(stored.verify_inline_payload().is_ok());
    }

    #[test]
    fn empty_manifest_is_structurally_valid() {
        let manifest = manifest();

        assert!(manifest.validate().is_ok());

        assert_eq!(
            manifest.topological_order().expect("empty order"),
            Vec::<ArtifactId>::new(),
        );
    }

    #[test]
    fn cloning_manifest_preserves_contents() {
        let mut original = manifest();

        original
            .insert_artifact(artifact("module.main"))
            .expect("artifact insertion");

        original.add_root(artifact_id("module.main"));

        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
